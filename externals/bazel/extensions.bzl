## extensions.bzl
load(":bazel/repositories.bzl", "internal_dependencies")

def _internal_dependencies_impl(_ctx):
    internal_dependencies()

internal_deps = module_extension(
    implementation = _internal_dependencies_impl,
)
