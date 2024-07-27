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

pub(crate) mod auto_route {
    pub(crate) use auto_route::route;
    pub(crate) use axum_typed_routing::{TypedRouter};
}

pub(crate) use auto_route::*;
