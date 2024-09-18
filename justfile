set dotenv-required
set dotenv-filename := ".env.dev"


# Watch

watch:
     cargo watch -i "tests/golden/**/*" -q -c -x "c"

watch-test name="":
    cargo watch -i "tests/golden/**/*" -q -c -s "just test {{name}}"

watch-example name="":
    cargo watch -i "tests/golden/**/*" -q -c -x "run --example {{name}}"

watch-generate_consts:
    cargo watch -i "tests/golden/**/*" -w generate-consts -q -c -x "test -p generate_consts -- --nocapture"

watch-test-integration:
    cargo watch -i "tests/golden/**/*" -w tests -q -c -x 'test --test "*" -- --nocapture'


# Individual commands

test name="":
    cargo test --all-targets {{name}} -- --nocapture

example name="":
    cargo run --example {{name}}