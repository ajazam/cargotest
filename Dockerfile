FROM scratch
COPY ./target/debug/server.exe .
ENTRYPOINT ["server"]