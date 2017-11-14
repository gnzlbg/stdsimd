set -ex

function ci_install() {
    echo "TARGET=${TARGET}"
    echo "RUSTFMT=${RUSTFMT}"
    echo "CLIPPY=${CLIPPY}"

    if [[ "${RUSTFMT}" == "On" ]]; then
        cargo install rustfmt-nightly;
    fi

    if [[ "${CLIPPY}" == "On" ]]; then
        cargo install clippy;
    fi

    if [[ "${TARGET}" == x86_64-unknown-linux-gnu-emulated ]]; then
        # Install Intel's Software Development Emulator
        INTEL_SDE_URL=https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/sde-external-8.9.0-2017-08-06-lin.tar.bz2
        curl $INTEL_SDE_URL | tar xjf -
        export TARGET=$(echo $TARGET | sed 's/-emulated//')
        export FEATURES="intel_sde"
    fi

    if [[ "${TARGET}" == arm-unknown-linux-gnueabihf ]] \
           || [[ "${TARGET}" ==  armv7-unknown-linux-gnueabihf ]] \
           || [[ "${TARGET}" ==  aarch64-unknown-linux-gnu ]]; then
        docker build -t stdsimd_${TARGET} ci/docker/${TARGET}
    fi

    if [[ "${TARGET}" == x86_64-unknown-linux-gnu ]] \
           || [[ "${TARGET}" == x86_64-apple-darwin ]]; then
        export CARGO_DRIVER=cargo
    else
        export CARGO_DRIVER=cross
    fi

    if [[ "${CARGO_DRIVER}" == "cross" ]]; then
        rustup target add $TARGET
        cargo install cross
    fi

    export FEATURES="strict,${FEATURES}"
    export RUST_BACKTRACE=1
}

cargo_test() {
    cmd="$CARGO_DRIVER test --all --target=$TARGET --features $FEATURES --verbose $1 -- --nocapture $2"
    $cmd
}

function ci_run() {
    echo "TARGET=${TARGET}"
    echo "RUSTFMT=${RUSTFMT}"
    echo "CLIPPY=${CLIPPY}"
    echo "CARGO_DRIVER=${CARGO_DRIVER}"
    echo "RUSTFLAGS=${RUSTFLAGS}"
    echo "RUST_BACKTRACE=${RUST_BACKTRACE}"
    echo "OBJDUMP=${OBJDUMP}"
    echo "FEATURES=${FEATURES}"

    cargo generate-lockfile

    # Tests are all super fast anyway, and they fault often enough on travis that
    # having only one thread increases debuggability to be worth it.
    export RUST_TEST_THREADS=1

    if [[ "${RUSTFMT}" == "On" ]]; then
        cargo fmt --all -- --write-mode=diff
    elif [[ "${CLIPPY}" == "On" ]]; then
        cargo clippy --all -- -D clippy-pedantic
    else
        cargo_test
        cargo_test "--release"
    fi
}
