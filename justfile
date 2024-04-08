dev:
    cargo watch -w src -x run | bunyan

dev_raw:
    cargo watch -w src -x run

build:
    cargo build --release

build_img:
    docker build --platform linux/arm64 -t storiny_og .

fmt:
    cargo +nightly fmt

udeps:
    cargo +nightly udeps --all-targets