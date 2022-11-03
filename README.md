
cargo watch -x check -x test -x run

docker build --tag zero2prod --file Dockerfile .

cargo sqlx prepare -- --lib

docker run -p 8000:8000 zero2prod
