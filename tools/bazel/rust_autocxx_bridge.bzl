# buildifier: disable=module-docstring
load("@bazel_skylib//rules:run_binary.bzl", "run_binary")
load("@rules_cc//cc:defs.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")

def rust_autocxx_bridge(name, src, inc, deps = []):
    """A macro defining a cxx bridge library

    Args:
        name (string): The name of the new target
        src (string): The rust source file to generate a bridge for
        inc (string): TODO
        deps (list, optional): A list of dependencies for the underlying cc_library. Defaults to [].
    """

    native.genrule(
        name = "%s/generated" % name,
        srcs = [src, inc],
        outs = [
	    # "autocxx-ffi-default-gen.rs",
	    "gen0.include.rs",
	#    "autocxxgen_ffi.h",
	    "autocxxgen0.h",
	    #"cxxgen.h",
	    "gen0.h",
	    "gen0.cc",
#	    "gen1.cc",
        ],

	cmd = "$(location @autocxx.rs//:codegen) $(location %s) --inc $(location %s) --outdir $(RULEDIR) --gen-rs-include --gen-cpp --fix-rs-include-name --generate-exact 2" % (src, inc),
        tools = ["@autocxx.rs//:codegen"],
    )

    cc_library(
        name = name,
        copts = ["-std=c++14"],
	#srcs = ["gen0.cc", "gen1.cc"],
	srcs = ["gen0.cc"],
        #hdrs = ["include/blobstore.h", "autocxxgen_ffi.h", "cxxgen.h"],
        hdrs = ["include/blobstore.h", "include/cxx.h", "autocxxgen0.h", "gen0.h"],
	deps = deps,
    )

#    cc_library(
#        name = "%s/include" % name,
#	hdrs = native.glob(["$(RULEDIR)/*.h"]),
#	#hdrs = [":exported_testdata"],
#    )
#
#    run_binary(
#        name = "%s/generated" % name,
#        srcs = [src, inc],
#        outs = [
#	    "autocxx-ffi-default-gen.rs",
#	    "autocxxgen_ffi.h",
#	    "cxxgen.h",
#	    "gen0.cc",
#	    "gen1.cc",
#        ],
#
#        args = [
#            "$(location %s)" % src,
#            "--inc",
#            "$(location %s)" % inc,
#            "--outdir",
#            "$(RULEDIR)",
#	    "--gen-rs-include",
#	    "--gen-cpp",
#        ],
#        tool = "@autocxx.rs//:codegen",
#    )
