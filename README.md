
cargo watch -x check -x test -x run

docker build --tag zero2prod --file Dockerfile .

cargo sqlx prepare -- --lib
