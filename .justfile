[private]
default:
    @just --choose --unsorted

# build release version.
build-release:
    cargo build --release

# build debug version.
build-debug:
    cargo build --debug

# run using the temp code as input and transform the generated dot file into svg. 
run-temp: build-release
    RUST_LOG=trace cargo run --release -- ./temp/compile_commands.json; dot -Tsvg test > test.svg

# check clippy lints.
clippy:
    cargo clippy -- --deny warnings

# print help.
help:
    @just --list --unsorted