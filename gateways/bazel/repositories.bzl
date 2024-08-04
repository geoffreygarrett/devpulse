# internal/bazel/deps.bzl

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive", "http_file")

# Function to conditionally include dependencies based on flags
def internal_dependencies(openai = True, github = True, azure = True):
    def _maybe(fn, **kwargs):
        fn(**kwargs)

    # OpenAI OpenAPI Specification
    if openai:
        _maybe(
            http_file,
            name = "openai_spec",
            sha256 = "d89fbc5273e3647d3af6e7cb12c6290d9b97ccae17e2d9539e0a7152d6d5ffcf",
            urls = ["https://raw.githubusercontent.com/openai/openai-openapi/cd3c3feb77931b5fd1e8b9c1eb5fb1697821a0d0/openapi.yaml"],
        )

    # GitHub REST API Specification
    if github:
        _maybe(
            http_file,
            name = "github_spec",
            sha256 = "2184c69a0138b643eb8196070eecd2408af5d732dc39816d8ea3821ddeedd01d",
            urls = ["https://github.com/github/rest-api-description/raw/main/descriptions/api.github.com/api.github.com.2022-11-28.json"],
        )

    # Azure REST API Specifications
    if azure:
        _maybe(
            http_archive,
            name = "azure_spec",
            integrity = "sha256-rffzuAPS8W3LnirQ1CrzXieXAj4F/QahxQYV4E3XM1k=",
            urls = ["https://github.com/MicrosoftDocs/vsts-rest-api-specs/archive/613959448e7351ebd229b0bda422d70b7c7b83ab.zip"],
            strip_prefix = "vsts-rest-api-specs-613959448e7351ebd229b0bda422d70b7c7b83ab",
            build_file_content = """

filegroup(
    name = "azure_spec_git_7_1_git_json",
    srcs = glob(["specification/git/7.2/**"]),
    visibility = ["//visibility:public"]
)
            """,

            #            build_file = "//:internal/bazel/azure_spec.BUILD.bazel",

            #load("//:internal/bazel/azure_specs.bzl", "create_azure_filegroups")
            #create_azure_filegroups()
            #            """,
        )
