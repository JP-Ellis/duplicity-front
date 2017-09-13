# Exit on any error
set -eux

# Run clippy and see if it has anything to say
clippy() {
    if [[ "$TRAVIS_RUST_VERSION" == "nightly" && $CLIPPY ]]; then
        cargo clippy
    fi
}

# Run the standard build and test suite.
build_and_test() {
    cargo build
    cargo test
}

# Test the command line and make sure it works.
command_line() {
    # TODO
    true
}

main() {
    build_and_test
    command_line
}

main
