set dotenv-required
set dotenv-filename := ".env.dev"


# Watch

watch:
     cargo watch -q -c -x "c" --env-file .env.dev

watch-test name="":
    cargo watch -q -c -s "just test {{name}}" --env-file .env.dev

watch-example name="":
    cargo watch -q -c -x "run --example {{name}}" --env-file .env.dev

watch-test-variants:
    cargo watch -w test-variants -q -c -x "test -p test-variants -- --nocapture" --env-file .env.dev


# Individual commands

test name="":
    cargo test --all-features {{name}} -- --nocapture

example name="":
    cargo run --example {{name}}