FROM rust:1.58.1 as build-stage
WORKDIR /api
COPY . .
RUN cargo build --release

FROM rust:1.58.1-slim
COPY --from=build-stage /api/target/release/api .
EXPOSE 80
CMD ["./api"]
