FROM scratch
COPY ./cargotest/target/debug/server .
ENTRYPOINT ["server"]