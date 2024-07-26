pub(super) mod access_token;
pub(super) mod arc_client;
pub(super) mod circuit_breaker;

#[macro_export]
macro_rules! create_arc_wrapper {
    ($name:ident, $inner:ty) => {
        #[derive(Debug)]
        pub(crate) struct $name(Arc<$inner>);

        impl Clone for $name {
            fn clone(&self) -> Self {
                $name(Arc::clone(&self.0))
            }
        }

        impl $name {
            /// Creates a new instance of the wrapper.
            pub fn new(inner: $inner) -> Self {
                $name(Arc::new(inner))
            }

            /// Returns a clone of the inner `Arc<$inner>`.
            pub fn inner(&self) -> Arc<$inner> {
                Arc::clone(&self.0)
            }
        }

        impl Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                Arc::get_mut(&mut self.0).expect("Arc reference count is not 1")
            }
        }
    };
}
