FROM ubuntu:latest
COPY ./target/debug/server /
CMD ["/server"]