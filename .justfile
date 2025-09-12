[private]
default:
    @ just --choose --unsorted

# build release version
build-release:
    cargo build --release

# build debug version
build-debug:
    cargo build

# run azhdaha with the given command
run *CMD:
    cargo run --release -- {{CMD}}

# prepare the tool and deploy the assets
deploy:
    sudo cp annotations/azhdaha.h /usr/include/
    
    mkdir -p ~/.local/include/azhdaha/
    cp -r annotations/include/* ~/.local/include/azhdaha/

# run with examples as input
run-examples:
    cargo run --release -- ./examples/compile_commands.json

# custom test used for debugging
custom-test:
    RUST_LOG=trace cargo run -- ./temp/compile_commands.json --dot-graph
    @ just convert-dot-graphs

# convert each created dot-graph file into its associated svg image
convert-dot-graphs PATH=".":
    @ for file in {{PATH}}/*.dot; do dot -Tsvg $file > {{PATH}}/${file%.dot}.svg; done

# check code style
check-lints:
    cargo fmt --check
    cargo clippy -- --deny warnings

# clean target directory and other unwanted files
clean:
    rm -f *.dot *.svg
    cargo clean

# print help
help:
    @ just --list --unsorted