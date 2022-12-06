fn main() {
    let run = wandb::init(Default::default());

    let artifact = wandb::Artifact::new("my-artifact", "dataset");
    artifact.entry("script.rs").set(file!());

    run.log_artifact(&artifact);

    run.finish();
}
