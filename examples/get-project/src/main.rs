#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <entity>/<project>", args[0]);
        eprintln!(" e.g.: {} wandb/huggingtweets", args[0]);
        std::process::exit(1);
    }
    let key = args[1].parse::<wandb::ProjectKey>().unwrap();

    let api = wandb::Api::default();
    let project = api.project(&key).await.unwrap().project;

    match project {
        Some(project) => {
            println!("Project id: {}", project.id);
        }
        None => {
            println!("No such project!");
        }
    }
}
