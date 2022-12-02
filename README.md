# Examples

Print a project's GraphQL id:
```
$ cd examples/get-project
$ cargo run -- wandb/huggingtweets
<...snip...>
Project id: UHJvamVjdDp2MTpodWdnaW5ndHdlZXRzOndhbmRi
```

And if you've run `wandb login`, this should print your name:
```
$ cd examples/whoami
$ cargo run
<...snip...>
Hello, Example McName!
```


# Contributing

## Testing
```bash
cargo test
```

## Linting
```bash
rustup component add clippy  # if you haven't
cargo clippy
```
