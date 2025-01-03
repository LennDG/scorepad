# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen brotli just

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown


WORKDIR /work
COPY . .

RUN just release

# Run the app!
FROM rustlang/rust:nightly-alpine AS runner

RUN apk update && \
    apk add --no-cache curl

WORKDIR /app

COPY --from=builder /work/target/release/scorepad /app/
COPY --from=builder /work/target/site /app/site

ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"

EXPOSE 3000

CMD ["/app/scorepad"]
