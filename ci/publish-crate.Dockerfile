FROM rust:1.88.0-alpine3.21@sha256:63f574f761cd3a5dca53c709915cbfe0386db868e0d8d1aaa5da32ba4e3c7ac6
RUN apk add --no-cache \
	musl-dev=1.2.5-r9

ENTRYPOINT ["cargo", "publish", "--verbose", "--target=x86_64-unknown-linux-musl", "--locked"]
