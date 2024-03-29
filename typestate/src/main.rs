struct Operation<S: IoState> {
    marker: std::marker::PhantomData<S>,
}

trait IoState: Sized {}
enum GetRangesRead {}
enum GetRangesClose {}
impl IoState for GetRangesRead {}
impl IoState for GetRangesClose {}

impl Operation<GetRangesRead> {
    fn foo() {}
}

impl Operation<GetRangesClose> {
    fn boo() {}
}

fn main() {
    let read_op: Operation<GetRangesRead> = Operation {
        marker: std::marker::PhantomData,
    };
    let close_op: Operation<GetRangesClose> = Operation {
        marker: std::marker::PhantomData,
    };

    let v: Vec<Box<Operation<dyn IoState>>> = vec![Box::new(read_op), Box::new(close_op)];
}
