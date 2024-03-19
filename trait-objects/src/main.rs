use std::fmt::Debug;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

///--------------- TRAITS THAT DEFINE BEHAVIOUR ---------------------
///---------------  COMMON TO ALL I/O BACKENDS  ---------------------

trait OperationMarker: Debug {}

trait GetRanges<M> {
    /// `byte_range`:
    /// The byte range for the file. Negative numbers are relative to the filesize.
    /// (Like indexing lists in Python.) For example:
    ///        0..-1   The entire file.
    ///        0..100  The first 100 bytes.
    ///     -100..-1   The last 100 bytes.
    ///
    /// `metadata`: Use to identify each byte_range.
    /// For example, in Zarr, this would be used to identify the
    /// location at which this chunk appears in the merged array.
    ///
    /// # Errors:
    /// If the user submits a GetRanges operation with an invalid filename then
    /// the user will receive a single Err(std::io::ErrorKind::NotFound) with context
    /// that describes the filename that failed.
    /// If a subset of the `byte_ranges` results in an error (e.g. reading beyond
    /// end of the file) then the user will receive a mixture of `Ok(Output::Buffer)`
    /// and `Err`, where the `Err` will include context such as the filename
    /// and byte_range.
    ///
    /// Returns a `Vec` because we may want to split a single large read into multiple
    /// concurrent reads.
    fn get_ranges(
        filename: PathBuf,
        byte_ranges: Vec<Range<isize>>,
        metadata: Option<Vec<M>>,
    ) -> Self;
}

///------------ CODE THAT'S SPECIFIC TO A SINGLE I/O BACKEND --------
///------------------------------------------------------------------
#[derive(Debug)]
struct GetRangesOp<M> {
    filename: PathBuf, // This will be a CString in the actual io_uring implementation.

    // Information submitted by the user:
    user_byte_ranges: Vec<Range<isize>>,
    user_metadata: Option<Vec<M>>,
    user_buffers: Vec<Option<u8>>,

    // The actual, optimised operations that get submit to io_uring:
    opt_byte_ranges: Vec<Range<isize>>,
    opt_buffer_offsets: Vec<Option<isize>>,
    opt_to_user: Vec<usize>, // Map from the optimised op to the user op.
    next_to_submit: usize,   // Next optimised operation to submit to uring.
    n_opt_completed: usize,  // Number of optimised operations that have completed.
}

impl<M> OperationMarker for GetRangesOp<M> where M: Debug {}

// Let's say the user asks for one 4 GByte file.
// Linux cannot load anything larger than 2 GB in one go.
// But we don't know the size immediately because the user used byte_range=0..-1.
// The steps will be:
// 1. Get the filesize from io_uring and, concurrently, open the file.
// 2. As soon as the filesize is returned, create one 4 GB buffer.
// 3. Set opt_byte_ranges to [0..2GB, 2GB..4GB].
// 4. Set opt_buffer_offsets to [0, 2GB].
// 5. Set opt_to_user to [0, 0].
// 6. Once the `open` operatoin has completed, submit both `read` operations.
// 7. When both optimised `read` operations have completed, submit the user_buffer and metadata to the
//    completion queue.
//
// Now let's say that the user asks for 1 million chunks from a single file.
// Some of these chunks are close, so we merge them.
// The user has asked for only 1,000 buffers to be allocated at any given time.
//
impl<M> GetRanges<M> for GetRangesOp<M> {
    fn get_ranges(
        filename: PathBuf,
        byte_ranges: Vec<Range<isize>>,
        metadata: Option<Vec<M>>,
    ) -> Self {
        let len = byte_ranges.len();
        if let Some(metadata_vec) = &metadata {
            assert_eq!(len, metadata_vec.len());
        }
        Self {
            filename,
            user_byte_ranges: byte_ranges.clone(),
            user_metadata: metadata,
            user_buffers: (0..len).map(|_| None).collect(),
            opt_buffer_offsets: (0..len).map(|_| None).collect(),
            opt_byte_ranges: byte_ranges,
            opt_to_user: (0..len).collect(),
            next_to_submit: 0,
            n_opt_completed: 0,
        }
    }
}

fn main() {
    let (tx, rx): (
        Sender<Box<dyn OperationMarker>>,
        Receiver<Box<dyn OperationMarker>>,
    ) = channel();

    let get_ranges_op = GetRangesOp::get_ranges(
        PathBuf::from("foo/bar"),
        vec![0..100, 500..-1],
        Some(vec![0, 1]),
    );

    tx.send(Box::new(get_ranges_op)).unwrap();

    let recv = rx.recv().unwrap();
    println!("{recv:?}");
}
