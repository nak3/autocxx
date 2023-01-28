load("@rules_rust//rust:defs.bzl", "rust_library", "rust_proc_macro", "rust_binary")

#package(default_visibility = ["//visibility:public"])

rust_library(
    name = "autocxx",
    edition = "2021",
    srcs = glob(["src/**/*.rs"]),
    data = glob(["README.md"]),
    deps = [
        "//cargo:cxx",
        "//cargo:moveit",
    ],
    visibility = ["//visibility:public"],
    proc_macro_deps = [
        #"//cargo:autocxx",
	":autocxx-macro",
        "//cargo:aquamarine",
#        "//vendored/hello_cargo_library/cargo:log",
    ],
)

rust_proc_macro(
    name = "autocxx-macro",
    srcs = glob(["macro/src/**/*.rs"]),
    edition = "2021",
    visibility = ["//visibility:public"],
    deps = [
	":autocxx-parser",
        "//cargo:proc_macro_error",
        "//cargo:proc_macro2",
        "//cargo:quote",
        "//cargo:syn",
    ],
)

rust_library(
    name = "autocxx-parser",
    srcs = glob(["parser/src/**/*.rs"]),
    edition = "2021",
#    visibility = ["//visibility:public"],
    deps = [
        "//cargo:log",
        "//cargo:proc_macro2",
        "//cargo:quote",
        "//cargo:serde",
        "//cargo:thiserror",
        "//cargo:once_cell",
        "//cargo:itertools",
        "//cargo:indexmap",
        "//cargo:serde_json",
        "//cargo:syn",
    ],
#    crate_features = [
#        "reproduction_case",
#    ],

)

# Rename this for the features = build
rust_library(
    name = "autocxx-engine",
#    srcs = glob(["engine/src/*.rs"]),
    srcs = glob(["engine/src/**/**/**/*.rs"]),
    compile_data = glob(["**/*.md", "include/cxx.h"]),
    edition = "2021",
    visibility = ["//visibility:public"],
    crate_features = ["build"],
    deps = [
        "//cargo:log",
        "//cargo:proc_macro2",
        "//cargo:quote",
	"//cargo:autocxx_bindgen",
        "//cargo:itertools",
        "//cargo:cc",
        "//cargo:cxx_gen",
	":autocxx-parser",
        "//cargo:version_check",
        "//cargo:tempfile",
        "//cargo:once_cell",
        "//cargo:serde_json",
        "//cargo:miette",
        "//cargo:thiserror",
        "//cargo:regex",
        "//cargo:indexmap",
        "//cargo:prettyplease",
        "//cargo:syn",
    ],
    proc_macro_deps = [
        #"//cargo:autocxx",
        "//cargo:indoc",
        "//cargo:aquamarine",
        "//cargo:strum_macros",
        "//cargo:rustversion",
#        "//vendored/hello_cargo_library/cargo:log",
    ],
)

rust_library(
    name = "autocxx-build",
    srcs = glob(["gen/build/src/**/*.rs"]),
    edition = "2021",
    visibility = ["//visibility:public"],
    deps = [
        ":autocxx-engine",
        "//cargo:env_logger",
        "//cargo:indexmap",
    ],
)


rust_binary(
    name = "codegen",
    srcs = glob(["gen/cmd/src/**/*.rs"]),
#    data = ["gen/cmd/src/gen/include/cxx.h"],
    edition = "2021",
    visibility = ["//visibility:public"],
    deps = [
        ":autocxx-engine",
        "//cargo:clap",
        "//cargo:proc_macro2",
        "//cargo:env_logger",
        "//cargo:pathdiff",
        "//cargo:indexmap",
        "//cargo:miette",
    ],
)
