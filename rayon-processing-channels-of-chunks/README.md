```rust
struct ByteRange<M> {
    byte_range: Range<isize>,

    // This buffer is supplied by the user for `PutRanges`.
    // For `GetRanges`, the user cannot provide a buffer. Instead, 
    // LSIO creates a buffer for each byte range in `GetRanges`.
    buffer: Option<AlignedBuffer>,

    // metadata is used to identify this byte range.
    // For example, in Zarr, this would be used to identify the
    // location at which this chunk appears in the merged array.
    metadata: M,
}

enum OperationKind {
    GetRanges,
    PutRanges,
}

struct Operation<M> {
    operation_kind: OperationKind,
    buffers: Option<Vec<AlignedBuffer>>, // For PutRanges
    byte_ranges: Option<Vec<ByteRange<M>>>,  // For GetRanges and PutRanges
    filename: CString,  // For GetRanges and PutRanges
}

impl<M> Operation<M> {
    /// If the user submits a GetRanges operation with an invalid filename then
    /// the user will receive a single Err(std::io::ErrorKind::NotFound) with context
    /// that describes the filename that failed.
    /// If a subset of the `byte_ranges` results in an error (e.g. reading beyond
    /// end of the file) then the user will receive a mixture of `Ok(Output::Buffer)`
    /// and `Err`, where the `Err` will include context such as the filename and byte_range.
    fn get_ranges(filename, byte_ranges) -> Self<M> {
    }

    fn put_ranges(filename, byte_ranges, buffers) -> Self<M> {
        // TODO: Maybe we also need a `slices: &[u8]` field, which gives one slice
        // per `byte_range`, whilst also having a `buffers` field to own the `AlignedBuffer`.
    }
}

struct OpGroup<M> {
    operations: Receiver<Operation<M>>,

    // Metadata for the whole group. Such as the filename of the merged output.
    metadata: M,
}

struct Output<M> {
    // Each `byte_range` within an `Operation::GetRanges` returns an `Output`.
    operation_kind: OperationKind,
    buffer: Option<AlignedBuffer>,
    byte_range: Option<ByteRange<M>>,
    // TODO: How to handle outputs from `ls`, `rm`, etc.?
}

struct OutputGroup<GROUPMETA, OUTPUTMETA> {
    // We use a `Receiver` so we can process the next `Buffer` as soon as the producing
    // thread finishes each `Buffer`:
    outputs: Receiver<Result<Output<OUTPUTMETA>>>,

    // Metadata for the group (e.g. the output filename).
    metadata: <GROUPMETA>,
}
```

## User code

```rust
const MAX_N_BUFFERS: usize = 1024;
let mut uring_local = IoUringLocal::new(MAX_N_BUFFERS);

let mut submission_queue: Sender<OpGroup> = uring_local.submission();

// Define operations to get a bunch of files:
let get_group_0 = OpGroup::new()
    .extend(!vec[
        Operation::get_ranges("foo.0.0", 0..-1),
        Operation::get_ranges("foo.0.1", 0..-1),
    ])
    .metadata(OutputFilename("foo_0"));

// Define operations to get a bunch of files:
let get_group_1 = OpGroup::new()
    .extend(!vec[
        Operation::get_ranges("foo.1.0", 0..-1),
        Operation::get_ranges("foo.1.1", 0..-1),
    ])
    .metadata(OutputFilename("foo_1"));

// Start loading the files in a separate threadpool:
submission_queue.send(get_group_0).unwrap();
submission_queue.send(get_group_1).unwrap();

// uring_local will load all operations from `get_group_0`. And then from `get_group_1`.
// Now we can wait on the completed items.

let completion_queue: Receiver<OutputGroup> = uring_local.completion();

let mut buffer_recycling_queue = uring_local.buffer_recycling_queue();

completion_queue.into_iter().par_bridge().for_each(|output_group: OutputGroup| {
    let out = output_group.outputs.into_iter().par_bridge()
        .map(|output| {
            assert_eq!(output.operation_kind, GetRanges);
            let decompressed = decompress(&output.buffer.unwrap());
            buffer_recycling_queue.send(output.buffer.take()).unwrap();
            decompressed
        })
        .reduce(reduce_func);
    let out = compress(out);

    // Write `out` to disk:
    let put_op = Operation::put_ranges(output_group.metadata.output_filename, 0..-1, out);
    let op_group = OpGroup::new().append(put_op);
    submission_queue.send(op_group);  // Does not block.
});
```

