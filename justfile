# Tests are run by nextest
# --all-features is not passed since `minify-html` is a optional dependency which takes long to compile,
# and is not going to be included in local development.
# CI will run with the `--all-features` flag.

set dotenv-required := true
set dotenv-filename := ".env"
set windows-shell := ["pwsh", "-NoLogo", "-NoProfile", "-Command"]

watch_base := "cargo watch -q -c -i 'tests/resources/**/*'"
no_capture := if env_var("TEST_LOG") == "true" { "--no-capture" } else { "" }

run bin="":
    clear
    cargo run --bin {{ bin }} -r

# Watch

watch:
    {{ watch_base }} -x "c --all-targets --all-features"

watch-test name="":
    {{ watch_base }} -s "just test {{ name }}"

watch-example name:
    {{ watch_base }} -s "just example {{ name }}"

watch-test-integration:
    {{ watch_base }} -x "nextest run -E 'kind(test)'"

watch-bench name="":
    {{ watch_base }} -s "just bench {{ name }}"

# Test commands

test name="":
    clear
    cargo nextest run {{ no_capture }} --all-targets --all-features {{ name }}

test-doc:
    clear
    cargo test --doc

check-lib-bins:
    clear
    cargo check --lib --bins

example name:
    clear
    cargo run --example {{ name }}

bench name="":
    clear
    cargo bench --all-features --all-targets --all-features {{ name }}

cov:
    clear
    rustup run nightly cargo llvm-cov nextest --open --lib --locked

lint:
    clear
    cargo clippy --all-targets  --all-features --locked

tree crate:
    clear
    cargo tree --all-features --all-targets -i {{ crate }}

# Others
doc:
    clear
    cargo doc --no-deps --open

git-gc:
    git gc --prune=now --aggressive
