FROM rust as builder
WORKDIR /usr/src/rtrs
COPY . .
RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/rtrs /usr/local/bin/rtrs
CMD ["rtrs", "--host", "0.0.0.0","--port", "80"]
