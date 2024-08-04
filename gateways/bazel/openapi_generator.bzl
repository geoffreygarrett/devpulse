# Define the rule in openapi_generator.bzl
def _openapi_generator_impl(ctx):
    spec_file = ctx.file.spec
    output_directory = ctx.actions.declare_directory(ctx.attr.name + "_output")
    ctx.actions.run_shell(
        inputs = [spec_file],
        outputs = [output_directory],
        command = """
        openapi-generator generate -g rust \
          --additional-properties=packageName=external_github,useSingleRequestParameter=true,deriveBuilder=true,enableCache=true,tracing=true \
          -i {input} -o {output}
        """.format(input = spec_file.path, output = output_directory.path),
        progress_message = "Generating Rust client from OpenAPI spec",
    )
    return [DefaultInfo(files = depset([output_directory]))]

openapi_generator = rule(
    implementation = _openapi_generator_impl,
    attrs = {
        "spec": attr.label(allow_single_file = True),
    },
)

# Usage in BUILD file
openapi_generator(
    name = "generate_github_client",
    spec = "@target//file:your_openapi_file.json",
)
