load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")

rust_binary(
    name = "hello-world",
    srcs = ["src/main.rs"],
    edition = "2021"
)

rust_test(
    name = "hello-world-tests",
    crate = ":hello-world"
)