FROM rust:1.82 AS builder
RUN rustup target add wasm32-unknown-unknown
RUN wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz \
    | tar xzC /usr/local/bin trunk
WORKDIR /app
COPY . .
RUN trunk build --release

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
