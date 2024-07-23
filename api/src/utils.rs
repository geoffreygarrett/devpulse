pub fn convert_openapi_to_axum_path(openapi_path: &str) -> String {
    let mut axum_path = String::from(openapi_path);
    while let Some(start) = axum_path.find('{') {
        if let Some(end) = axum_path.find('}') {
            let param = &axum_path[start + 1..end];
            axum_path.replace_range(start..=end, &format!(":{}", param));
        } else {
            break;
        }
    }
    axum_path
}
