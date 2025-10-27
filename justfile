ex bin:
    cargo run --example {{bin}}

test:
    cargo test

lint:
    cargo clippy --all-targets -- -Dwarnings
