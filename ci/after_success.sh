# Exit on any error
set -ux

install_kcov() {
    set -e
    # Download and install kcov
    wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz -O - | tar -xz
    cd kcov-master
    mkdir build
    cd build
    cmake ..
    make -j$(nproc)
    make install DESTDIR=../../kcov-build
    cd ../..
    rm -rf kcov-master
    set +e
}

run_kcov() {
    # Run kcov on all the test suites
    for file in target/debug/duplicity_front-*[^\.d]; do
        mkdir -p "target/cov/$(basename $file)";
        echo "Testing $(basename $file)"
        ./kcov-build/usr/local/bin/kcov \
            --exclude-pattern=/.cargo,/usr/lib\
            --verify "target/cov/$(basename $file)" \
            "$file";
    done

    # Run kcov with the binary and test various sets of arguments.
    executable="target/debug/duplicity-front"
    # TODO

    bash <(curl -s https://codecov.io/bash)
    echo "Uploaded code coverage"
}

kcov_suite() {
    if [[ "$TRAVIS_RUST_VERSION" == "stable" ]]; then
        install_kcov
        run_kcov
    fi
}

main() {
    kcov_suite
}

main
