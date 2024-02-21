use std::{thread, time::Duration};

async fn foo(task_id: usize) {
    println!("START: {task_id}");
    thread::sleep(Duration::from_millis(100));
    println!("END  : {task_id}");
}

#[tokio::main]
async fn main() {
    let mut v = Vec::new();
    for i in 0..10 {
        v.push(tokio::spawn(async move {
            foo(i).await;
        }));
    }

    for h in v {
        h.await.unwrap();
    }
}
