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

test:
    cargo nextest run --workspace

test_ci:
    cargo nextest run --no-fail-fast --workspace

test_verbose:
    cargo nextest run --no-capture --no-fail-fast --workspace

udeps:
    cargo +nightly udeps --all-targets