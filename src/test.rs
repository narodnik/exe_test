use async_executor::Executor;
use easy_parallel::Parallel;
use futures_lite::future;

async fn sleep(dur: Duration) {
    Timer::after(dur).await;
}

async fn hello_loop() {
    loop {
        println!("Hello fren");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn pingpong(ex: Executor<'_>)  -> io::Result<()> {
    // spawn hello loop in parallel
    ex.spawn(async {
        hello_loop().await;
    })
    .detach();

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
        pingpong(ex).await;
        drop(signal);
    }));

    thread::sleep(Duration::from_secs(5));
    Ok(())
}     
