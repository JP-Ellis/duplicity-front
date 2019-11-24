#!/bin/bash

# Echo all commands before executing them
set -o xtrace
# Forbid any unset variables
set -o nounset
# Exit on any error
set -o errexit

run_code_coverage() {
    cargo tarpaulin --ciserver travis-ci --coveralls $TRAVIS_JOB_ID

    bash <(curl -s https://codecov.io/bash)
    echo "Uploaded code coverage to codecov.io"
}

main() {
    if [[ "$TRAVIS_RUST_VERSION" == "stable" ]]; then
        run_code_coverage
    fi
}

main
