use smol::{future, io, Timer};
use std::thread;
use std::time::Duration;
use async_channel::unbounded;
use easy_parallel::Parallel;
use async_executor::Executor;

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
    
async fn pingpong(ex: &Executor<'_>)  -> io::Result<()> {
    // spawn hello loop in parallel
    ex.spawn(async {
        foo().await;
    })
    .detach();
    ex.spawn(async {
        bar().await;
    })
    .detach();
    println!("Debug 1");
    thread::sleep(Duration::from_secs(5));
    println!("Debug 2");
    Ok(())
}


fn main() -> io::Result<()> {
    let ex = Executor::new();
    let (signal, shutdown) = unbounded::<()>();

Parallel::new()
    // Run four executor threads.
    .each(0..4, |_| future::block_on(ex.run(shutdown.recv())))
    // Run the main future on the current thread.
    .finish(|| future::block_on(async {
        pingpong(&ex).await;
        drop(signal);
    }));

    Ok(())
}     
