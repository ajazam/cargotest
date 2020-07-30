FROM scratch
COPY ./target/debug/server .
CMD ["/server"]
