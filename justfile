set dotenv-required
set dotenv-filename := ".env.dev"


# Watch

watch:
     cargo watch -i "tests/resources/**/*" -q -c -x "c" --env-file .env.dev

watch-test name="":
    cargo watch -i "tests/resources/**/*" -q -c -s "just test {{name}}" --env-file .env.dev

watch-example name="":
    cargo watch -i "tests/resources/**/*" -q -c -x "run --example {{name}}" --env-file .env.dev

watch-generate_consts:
    cargo watch -i "tests/resources/**/*" -w generate-consts -q -c -x "test -p generate_consts -- --nocapture" --env-file .env.dev

watch-test-integration:
    cargo watch -i "tests/resources/**/*" -w tests -q -c -x 'test --test "*" -- --nocapture' --env-file .env.dev


# Individual commands

test name="":
    cargo test --all-features {{name}} -- --nocapture

example name="":
    cargo run --example {{name}}