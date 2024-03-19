use std::fmt::Debug;
use std::iter::zip;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

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
    fn get_ranges(filename: PathBuf, byte_ranges: Vec<Range<isize>>, metadata: Vec<M>) -> Self;
}

///------------ CODE THAT'S SPECIFIC TO A SINGLE I/O BACKEND --------
///------------------------------------------------------------------
#[derive(Debug)]
struct GetRangesOp<M> {
    filename: PathBuf, // This will be a CString in the actual io_uring implementation.
    byte_ranges: Vec<Range<isize>>,
    metadata: Vec<M>,
    /// We use an `Option` because each `buffer` starts as `None`.
    buffers: Vec<Option<u8>>,
    next_to_submit: usize,
}

impl<M> OperationMarker for GetRangesOp<M> where M: Debug {}

impl<M> GetRanges<M> for GetRangesOp<M> {
    fn get_ranges(filename: PathBuf, byte_ranges: Vec<Range<isize>>, metadata: Vec<M>) -> Self {
        assert_eq!(byte_ranges.len(), metadata.len());
        let len = byte_ranges.len();
        Self {
            filename,
            byte_ranges,
            metadata,
            buffers: (0..len).map(|_| None).collect(),
            next_to_submit: 0,
        }
    }
}

fn main() {
    let (tx, rx): (
        Sender<Box<dyn OperationMarker>>,
        Receiver<Box<dyn OperationMarker>>,
    ) = channel();

    let get_ranges_op =
        GetRangesOp::get_ranges(PathBuf::from("foo/bar"), vec![0..100, 500..-1], vec![0, 1]);

    tx.send(Box::new(get_ranges_op)).unwrap();

    let recv = rx.recv().unwrap();
    println!("{recv:?}");
}
