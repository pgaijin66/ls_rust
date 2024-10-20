FROM scratch

COPY target/x86_64-unknown-linux-musl/release/ls_rust /ls_rust

ENTRYPOINT ["/ls_rust"]
