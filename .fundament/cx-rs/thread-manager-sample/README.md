# ThreadManager Sample

Small one-shot binary that starts a CX thread with `ThreadManager` from
`cx-core-api`, submits a single user turn, and prints the final assistant
message.

```sh
cargo run -p cx-thread-manager-sample -- "Say hello"
```

Use `--model` to override the configured default model:

```sh
cargo run -p cx-thread-manager-sample -- --model cy/i1a "Say hello"
```

The prompt can also be piped through stdin:

```sh
printf 'Say hello\n' | cargo run -p cx-thread-manager-sample
```
