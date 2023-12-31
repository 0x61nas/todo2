#!/usr/bin/env just --justfile

_CARGO_TARGET_DIR := env_var_or_default("CARGO_TARGET_DIR", "target")

alias t := test
alias l := lint
alias c := check
alias cov := coverage-report
alias r := generate-readme
alias s := setup

default: check lint test

setup:
    cargo install cargo-readme
    cargo install grcov
    cargo install committed
    cargo install cargo-deny
    cargo install cargo-audit --features=fix
#   pip install codespell

# Run all the tests
test:
    # Test the default features
    cargo test
    # Test the log feature
    cargo test --no-default-features --features "log"
    # Test the `with-chrono` feature
    cargo test --no-default-features --features "with-chrono"
    # Test the `with-chrono` and `log` features
    cargo test --no-default-features --features "with-chrono,log"
    # Test the `with-time` feature
    cargo test --no-default-features --features "with-time"
    # Test the `with-time` and `log` features
    cargo test --no-default-features --features "with-time,log"


# Check the program with all features enabled.
check:
    cargo check
    cargo deny check
    cargo deny check licenses
    committed aurora..HEAD --no-merge-commit
    cargo audit
    codespell --skip="target,git" --ignore-words="{{justfile_directory()}}/.codespellignore"

@lint:
    cargo fmt --all -- --check --verbose
    cargo clippy --verbose --all-targets -- -D warnings

# Run the tests, and generate a coverage report
coverage:
    CARGO_INCREMENTAL=0 RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="{{_CARGO_TARGET_DIR}}/coverage/data/cargo-test-%p-%m.profraw" cargo test

# Generate the coverage report
coverage-report: coverage
    # Generate the report in html format using grcov
    grcov . --binary-path {{_CARGO_TARGET_DIR}}/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore "../*" -o {{_CARGO_TARGET_DIR}}/coverage/report/ --llvm --ignore "/*"

    # Open the report in the browser
    xdg-open {{_CARGO_TARGET_DIR}}/coverage/report/index.html

# Generate the readme file
@generate-readme:
    cargo readme > README.md
