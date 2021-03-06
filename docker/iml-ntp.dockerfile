FROM rust-iml-base as builder
FROM imlteam/rust-service-base:6.3.0

COPY --from=builder /build/target/release/iml-ntp /usr/local/bin
COPY docker/wait-for-dependencies.sh /usr/local/bin/

ENTRYPOINT [ "wait-for-dependencies.sh" ]
CMD ["iml-ntp"]
