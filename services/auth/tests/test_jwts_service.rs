#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use jsonwebtoken::Algorithm;




    use super::*;

    #[test]
    fn test_create_and_validate_token_symmetric() {
        let validation = Validation::new(Algorithm::HS256);
        let config =
            Arc::new(JwtConfig::new_symmetric("test_secret".to_string(), 3600, validation));
        let service = JwtService::new(config);

        service.rotate_keys("test_kid", "new_secret".as_bytes(), true);

        let token = service
            .generate_token("user123", "test_kid", vec!["role1".to_string()])
            .unwrap();
        let claims = service.validate_token(&token).unwrap();

        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.roles, vec!["role1"]);
    }

    // #[test]
    // fn test_create_and_validate_token_asymmetric() {
    //     let private_key = include_bytes!("private.pem");
    //     let public_key = include_bytes!("public.pem");
    //
    //     let config = Arc::new(JwtConfig::new_asymmetric(
    //         String::from_utf8(private_key.to_vec()).unwrap(),
    //         String::from_utf8(public_key.to_vec()).unwrap(),
    //         3600,
    //         Algorithm::RS256,
    //     ));
    //     let service = JwtService::new(config);
    //
    //     service.rotate_keys("test_kid", private_key, false);
    //
    //     let token = service
    //         .generate_token("user123", "test_kid", vec!["role1".to_string()])
    //         .unwrap();
    //     let claims = service.validate_token(&token).unwrap();
    //
    //     assert_eq!(claims.sub, "user123");
    //     assert_eq!(claims.roles, vec!["role1"]);
    // }

    #[test]
    fn test_invalid_token() {
        let validation = Validation::new(Algorithm::HS256);

        let config =
            Arc::new(JwtConfig::new_symmetric("test_secret".to_string(), 3600, validation));
        let service = JwtService::new(config);

        let result = service.validate_token("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_expired_token() {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 0;
        let config = Arc::new(JwtConfig::new_symmetric("test_secret".to_string(), 1, validation));
        let service = JwtService::new(config);

        service.rotate_keys("test_kid", "new_secret".as_bytes(), true);

        let token = service
            .generate_token("user123", "test_kid", vec!["role1".to_string()])
            .unwrap();

        // Wait for the token to expire
        std::thread::sleep(std::time::Duration::from_secs(2));

        let result = service.validate_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_rotate_keys() {
        let validation = Validation::new(Algorithm::HS256);
        let config =
            Arc::new(JwtConfig::new_symmetric("test_secret".to_string(), 3600, validation));
        let service = JwtService::new(config);

        service.rotate_keys("old_kid", "old_secret".as_bytes(), true);
        service.rotate_keys("new_kid", "new_secret".as_bytes(), true);

        let token = service
            .generate_token("user123", "new_kid", vec!["role1".to_string()])
            .unwrap();
        let claims = service.validate_token(&token).unwrap();

        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.roles, vec!["role1"]);
    }
}
