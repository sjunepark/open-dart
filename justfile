set dotenv-required
set dotenv-filename := ".env.dev"

watch name="":
    cargo watch -q --no-vcs-ignores -s "just test {{name}}" --env-file .env.dev

test name="":
    cargo test --all-features {{name}} -- --nocapture



