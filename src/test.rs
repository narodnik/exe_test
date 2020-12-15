use async_channel::unbounded;
use async_executor::Executor;
use easy_parallel::Parallel;
use smol::{future, io, Timer};
use std::time::Duration;

async fn sleep(dur: Duration) {
    Timer::after(dur).await;
}

async fn foo() {
    loop {
        println!("Hello fren");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn bar() {
    loop {
        println!("fren");
        sleep(Duration::from_secs(1)).await;
    }
}

async fn pingpong(ex: async_dup::Arc<Executor<'_>>) -> io::Result<()> {
    // spawn hello loop in parallel
    let task1 = ex.spawn(async {
        foo().await;
    });
    let task2 = ex.spawn(async {
        bar().await;
    });

    let ex2 = ex.clone();

    ex.spawn(async move {
        println!("Debug 1");
        // Using this sleep will block everything since we are using a single thread
        // for running the executor
        //thread::sleep(Duration::from_secs(5));
        ex2.spawn(async {
            println!("hello1234");
        }).await;
        sleep(Duration::from_secs(5)).await;
        println!("Debug 2");
    })
    .await;

    // This cancels the running tasks
    task1.cancel().await;
    task2.cancel().await;

    // This will wait for the tasks to finish
    //task1.await;
    //task2.await;

    Ok(())
}

fn main() -> io::Result<()> {
    let ex = async_dup::Arc::new(Executor::new());
    let (signal, shutdown) = unbounded::<()>();

    let ex2 = ex.clone();

    Parallel::new()
        // Run four executor threads.
        .each(0..1, |_| future::block_on(ex.run(shutdown.recv())))
        // Run the main future on the current thread.
        .finish(|| {
            future::block_on(async move {
                pingpong(ex2).await;
                drop(signal);
            })
        });

    Ok(())
}
