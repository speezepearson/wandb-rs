#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <entity> <project>", args[0]);
        eprintln!(" e.g.: {} wandb huggingtweets", args[0]);
        std::process::exit(1);
    }
    let entity_name = &args[1];
    let project_name = &args[2];

    let api = wandb::Api::default();
    let project = api.project(&wandb::ProjectKey::Name(entity_name.into(), project_name.into())).await.unwrap().project;

    match project {
        Some(project) => {
            println!("Project id: {}", project.id);
        }
        None => {
            println!("No such project!");
        }
    }
}
