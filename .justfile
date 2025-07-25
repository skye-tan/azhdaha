[private]
default:
    @ just --choose --unsorted

# build release version
build-release:
    cargo build --release

# build debug version
build-debug:
    cargo build --debug

# convert each created dot-graph file into its associated svg image
convert-dot-graphs:
    @ for file in *.dot; do dot -Tsvg "$file" > "${file%.dot}.svg"; done

# custom test used for debugging the tool
custom-test:
    RUST_LOG=trace cargo run --release -- ./temp/compile_commands.json --dot-graph
    @ just convert-dot-graphs

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