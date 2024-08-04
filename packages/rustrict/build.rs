fn main() {
    const MESSAGE_ATTRIBUTES: &[(&str, &str)] = &[("Entity", "#[derive(Eq, Hash)]")];

    let mut builder = tonic_build::configure()
        .build_server(std::env::var("CARGO_FEATURE_TONIC_SERVER").is_ok())
        .build_client(std::env::var("CARGO_FEATURE_TONIC_CLIENT").is_ok())
        .use_arc_self(std::env::var("CARGO_FEATURE_TONIC_ARC_SELF").is_ok());

    for (message, attribute) in MESSAGE_ATTRIBUTES {
        builder = builder.message_attribute(message, attribute);
    }

    builder
        .compile(
            &[
                "proto/acl.proto",
                "proto/config.proto",
                "proto/zookie.proto",
                "proto/api.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
