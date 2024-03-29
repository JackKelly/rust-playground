struct Operation<S: IoState> {
    marker: std::marker::PhantomData<S>,
}

enum IoState {
    GetRangesRead,
    GetRangesClose,
}

impl Operation<IoState::GetRangesRead> {
    fn foo() {}
}

impl Operation<IoState::GetRangesClose> {
    fn boo() {}
}

fn main() {
    let read_op: Operation<IoState::GetRangesRead> = Operation {
        marker: std::marker::PhantomData,
    };
    let close_op: Operation<IoState::GetRangesClose> = Operation {
        marker: std::marker::PhantomData,
    };

    let v = vec![read_op, close_op];
}
