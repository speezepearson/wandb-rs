use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Start a run:
    let run = wandb::init(Default::default()).await.unwrap();

    // Log some metrics:
    run.log(HashMap::from([("loss", 0.5), ("accuracy", 0.9)]));

    let run = run.finish().await;

    // We can still get the URL of the run:
    println!("Run URL: {}", run.url());

    // ...but we can't log any more metrics:
    // run.log(&[("loss", 0.5), ("accuracy", 0.9)]); // ERROR
}
