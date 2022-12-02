#[tokio::main]
async fn main() {
    let api = wandb::Api::default();
    let viewer = api.viewer().await.unwrap().viewer;

    match viewer {
        Some(viewer) => {
            println!("Hello, {}!", viewer.name);
        }
        None => {
            println!("You aren't logged in!");
        }
    }
}
