use tokio::time::{sleep, Duration};
use tokio::runtime;

async fn hello(a: u32) -> u32 {
    println!("hello world: {}", a);
    a
}

fn main() {
    //let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();
    let mut rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .build()
        .unwrap();
    rt.block_on(async { 
        let task = runtime::Handle::current().spawn(async {
            sleep(Duration::from_millis(100)).await; 
            println!("spawn");
        });
        hello(11111).await;
        task.await;
    });
}
