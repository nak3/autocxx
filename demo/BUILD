load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_cc//cc:defs.bzl", "cc_library")
load("//tools/bazel:rust_autocxx_bridge.bzl", "rust_autocxx_bridge")

rust_binary(
    name = "autocxx-demo",
    rustc_env = {"AUTOCXX_RS" : ".."},
    srcs = [
        "src/main.rs",
        ":bridge/generated",
    ],
    edition = "2018",
    deps = [
        ":bridge",
        "//:autocxx",
        "//cargo:cxx",
    ],
)

rust_autocxx_bridge(
    name = "bridge",
    src = "src/main.rs",
    inc = "include",
)

cc_library(
    name = "blobstore-sys",
    srcs = ["src/blobstore.cc"],
    copts = ["-std=c++14"],
    deps = [
        ":blobstore-include",
    ],
)

cc_library(
    name = "blobstore-include",
    hdrs = ["include/blobstore.h"],
)
