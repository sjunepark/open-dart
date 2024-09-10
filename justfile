set dotenv-required
set dotenv-filename := ".env.dev"

watch:
     cargo watch -q -c --no-vcs-ignores -s "just test" --env-file .env.dev

test name="":
    cargo test --all-features {{name}} -- --nocapture

watch-test name="":
    cargo watch -q -c --no-vcs-ignores -s "just test {{name}}" --env-file .env.dev

watch-test-derive-common:
    cargo watch -q -c -x "test -p derive-common"

example name="":
    cargo run --example {{name}}

watch-example name="":
    cargo watch -q -c -x "run --example {{name}}" --env-file .env.dev



