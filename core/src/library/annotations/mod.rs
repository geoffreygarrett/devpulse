mod annotation;
mod annotator;
mod azure;
mod bitbucket;
mod github;
mod gitlab;
mod service;

pub mod prelude {
    pub use crate::library::annotations::annotation::{Annotation, AnnotationType};
    pub use crate::library::annotations::annotator::Annotator;
    pub use crate::library::annotations::service::{AnnotationService, Platform};
}
