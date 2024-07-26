use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use reqwest::Client;

use crate::create_arc_wrapper;

create_arc_wrapper!(ArcClient, Client);

impl Default for ArcClient {
    fn default() -> Self {
        ArcClient::new(Client::new())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_arc_client_new() {
        let arc_client = ArcClient::new(Client::new());
        // Ensure that the inner Arc<Client> is valid
        assert_eq!(Arc::strong_count(&arc_client.0), 1);
    }

    #[test]
    fn test_arc_client_clone() {
        let arc_client1 = ArcClient::new(Client::new());
        let arc_client2 = arc_client1.clone();
        assert!(Arc::ptr_eq(&arc_client1.0, &arc_client2.0));
    }

    #[test]
    fn test_arc_client_deref() {
        let arc_client = ArcClient::new(Client::new());
        let client_ref: &Client = &*arc_client;
        // Ensure that dereferencing gives a valid Client reference
        assert!(std::ptr::eq(client_ref, &*arc_client.0));
    }

    #[test]
    fn test_arc_client_deref_mut() {
        let mut arc_client = ArcClient::new(Client::new());
        {
            let client_mut_ref: &mut Client = &mut *arc_client;
            // Ensure that dereferencing mut gives a valid Client mutable reference
            assert!(std::ptr::eq(client_mut_ref, Arc::get_mut(&mut arc_client.0).unwrap()));
        }
    }

    #[test]
    fn test_arc_client_inner() {
        let arc_client = ArcClient::new(Client::new());
        let inner_arc = arc_client.inner();
        // Ensure that inner() returns a valid Arc<Client> clone
        assert!(Arc::ptr_eq(&inner_arc, &arc_client.0));
    }
}
