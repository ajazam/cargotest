FROM scratch
COPY ./target/debug/server .
ENTRYPOINT ["server"]