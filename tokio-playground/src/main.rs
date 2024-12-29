use tokio::time::{sleep, Duration};
use tokio::spawn;

#[tokio::main]
async fn main() {
    let wait1 = spawn(wait_some_seconds_my_brother(Duration::from_secs(2)));
    let wait2 = spawn(wait_some_seconds_my_brother(Duration::from_secs(4)));

    if let Err(error) = wait1.await {
        eprintln!("That's a shame my brother, wait1 failed {error}");
    }

    if let Err(error) = wait2.await {
        eprintln!("That's a shame my brother, wait2 failed {error}");
    }

    println!("Ended my brother");
}

async fn wait_some_seconds_my_brother(duration: Duration) {
    sleep(duration).await;

    println!("waited {duration:?} ma brother");
}