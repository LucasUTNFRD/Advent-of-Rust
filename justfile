lint:
    cargo fmt -- `find . -name "*.rs"`
    cargo clippy --all-targets --all-features

flamegraph:
    cargo flamegraph --release --bin aoc && brave "flamegraph.svg"

run:
    cargo run --bin aoc

run-release:
    cargo run --release --bin aoc

build-release:
    cargo build --release --bin aoc

scaffold year day:
    cargo run --bin scaffold -- {{year}} {{day}}
