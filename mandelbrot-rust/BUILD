load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
rust_library(
    name = "mandelbrot_calculator",
    srcs = ["src/lib.rs"],
    edition = "2021",
    deps = all_crate_deps(
        normal = True,
    )
)

rust_test(
    name = "mandelbrot_calculator_tests",
    crate = ":mandelbrot_calculator",
    deps = all_crate_deps(
        normal = True,
    )
)