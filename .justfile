[private]
default:
    @ just --choose --unsorted

# build release version
build-release:
    cargo build --release

# build debug version
build-debug:
    cargo build --debug

# run with examples as input
run-examples:
    cargo run --release -- ./examples/compile_commands.json

# custom test used for debugging
custom-test:
    RUST_LOG=trace cargo run --release -- ./temp/compile_commands.json --dot-graph
    @ just convert-dot-graphs

# convert each created dot-graph file into its associated svg image
convert-dot-graphs PATH=".":
    @ for file in {{PATH}}/*.dot; do dot -Tsvg $file > {{PATH}}/${file%.dot}.svg; done

# check clippy lints
clippy:
    cargo clippy -- --deny warnings

# clean target directory and other unwanted files
clean:
    rm -f *.dot *.svg
    cargo clean

# check code format
check-format:
    cargo fmt --check

# print help
help:
    @ just --list --unsorted