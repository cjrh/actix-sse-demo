# actix-sse-demo

To investigate proposed SSE support for actix-web.

## Running

To get reload support, `cargo-watch` works well. Install
with `$ cargo install cargo-watch`.

```bash
$ RUST_LOG=actix_web=debug cargo watch -x run
```

Then visit `http://localhost:8080/` in your browser.
