def snake_case(name):
    """
    Converts a string to snake_case by replacing hyphens, dots, and camelCase.
    """
    name = name.replace("-", "_").replace(".", "_")
    result = []
    for i in range(len(name)):
        if name[i].isupper():
            result.append("_")
            result.append(name[i].lower())
        else:
            result.append(name[i])
    return "".join(result)

#basename
#dirname
#exists
#get_child
#is_dir
#readdir
#realpath
#
#
#decode
#encode
#encode_indent
#indent
#decode
#
#unknown json.decode(x, default=unbound)

def expand_openapi_specs(directory):
    # Recursively search for JSON files in the directory
    json_files = native.glob(["{}/**/*.json".format(directory)], allow_empty = True)

    for file_path in json_files:
        # Load and decode each JSON file
        content = native.read(file_path)
        decoded = native.json.decode(content)

        # Check if the file is an OpenAPI or Swagger spec
        if "openapi" in decoded or "swagger" in decoded:
            # Get the directory where the spec was found
            spec_directory = "/".join(file_path.split("/")[:-1])

            # Recursively create filegroups for this directory
            create_filegroups(spec_directory)
            break  # Break after handling the first found spec to prevent multiple entries for same directory

    # Recursive call for subdirectories if spec not found in the current search
    subdirectories = native.glob(["{}/*".format(directory)], allow_empty = True)
    for subdirectory in subdirectories:
        expand_openapi_specs(subdirectory)

def create_filegroups_old(base_dir, directories):
    for directory in directories:
        dir_name = snake_case(directory.replace("/", "_").replace(".", "_"))
        native.filegroup(
            name = dir_name,
            srcs = native.glob(["{}/{}/**".format(base_dir, directory)], allow_empty = True),
            visibility = ["//visibility:public"],
        )

def create_filegroups(directory):
    dir_name = snake_case(directory.replace("/", "_").replace(".", "_"))
    native.filegroup(
        name = dir_name,
        srcs = native.glob(["{}/**".format(directory)], allow_empty = True),
        visibility = ["//visibility:public"],
    )
    print("Created filegroup for directory:", directory)

def create_azure_filegroups():
    base_dir = "azure_spec"  # Base directory to start the search
    expand_openapi_specs(base_dir)

#    create_filegroups("azure_spec", [
#        "account",
#        "advancedSecurity",
#        "approvalsAndChecks",
#        "artifacts",
#        "artifactsPackageTypes",
#        "audit",
#        "build",
#        "core",
#        "dashboard",
#        "distributedTask",
#        "environments",
#        "extensionManagement",
#        "favorite",
#        "git",
#        "governance",
#        "graph",
#        "hooks",
#        "ims",
#        "memberEntitlementManagement",
#        "notification",
#        "operations",
#        "permissionsReport",
#        "pipelines",
#        "policy",
#        "processDefinitions",
#        "processadmin",
#        "processes",
#        "profile",
#        "release",
#        "reporting",
#        "search",
#        "security",
#        "securityRoles",
#        "serviceEndpoint",
#        "status",
#        "symbol",
#        "test",
#        "testPlan",
#        "testResults",
#        "tfvc",
#        "tokenAdmin",
#        "tokenAdministration",
#        "tokens",
#        "wiki",
#        "wit",
#        "work",
#    ])
