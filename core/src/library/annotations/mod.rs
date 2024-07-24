mod annotation;
mod annotator;
mod azure;
mod bitbucket;
mod github;
mod gitlab;
mod service;

pub mod prelude {
    pub use crate::lib::annotations::annotation::Annotation;
    pub use crate::lib::annotations::annotator::Annotator;
}
