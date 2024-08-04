use snafu::prelude::*;


// Define Result
pub type Result<T, E = AuthError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub(crate) enum InternalAuthError {
    JwtEncodeError { source: jsonwebtoken::errors::Error },
    JwtDecodeError { source: jsonwebtoken::errors::Error },
}

/// Represents errors that can occur in the authentication service.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub(crate) enum AuthError {
    /// Internal errors that shouldn't be exposed to the user, and shouldn't happen in normal operation.
    #[snafu(display("Internal error. Code: {}", code))]
    InternalError {
        source: InternalAuthError,
        code: u16,
    },

    /// Returned from the PKCE flow where the provided code verifier does not match the expected one.
    /// Indicates a bug in the implementation of the client library.
    #[snafu(display("The provided code verifier does not match the expected one."))]
    BadCodeVerifier,

    /// Indicates that the HTTP body of the request is not valid JSON.
    #[snafu(display("Invalid JSON in the request body."))]
    BadJson,

    /// JWT sent in the Authorization header is not valid.
    #[snafu(display("Invalid JWT token provided."))]
    BadJwt,
    /// OAuth callback from provider to Auth does not have all the required attributes (state).
    /// Indicates an issue with the OAuth provider or client library implementation.
    #[snafu(display("OAuth callback is missing required attributes."))]
    BadOauthCallback,

    /// OAuth state (data echoed back by the OAuth provider to Supabase Auth) is not in the correct format.
    /// Indicates an issue with the OAuth provider integration.
    #[snafu(display("OAuth state is not in the correct format."))]
    BadOauthState,

    /// Captcha challenge could not be verified with the captcha provider. Check your captcha integration.
    #[snafu(display("Captcha verification failed."))]
    CaptchaFailed,

    /// General database conflict, such as concurrent requests on resources that should not be modified concurrently.
    /// Check your app for concurrency issues, and if detected, back off exponentially.
    #[snafu(display("Database conflict detected."))]
    Conflict,

    /// Unlinking this identity causes the user's account to change to an email address which is already used by another user account.
    /// Indicates an issue where the user has two different accounts using different primary email addresses.
    #[snafu(display("Email conflict: identity not deletable."))]
    EmailConflictIdentityNotDeletable,

    /// Email address already exists in the system.
    #[snafu(display("Email address already exists."))]
    EmailExists,

    /// Signing in is not allowed for this user as the email address is not confirmed.
    #[snafu(display("Email address not confirmed."))]
    EmailNotConfirmed,

    /// Signups are disabled for email and password.
    #[snafu(display("Email provider is disabled."))]
    EmailProviderDisabled,

    /// PKCE flow state to which the API request relates has expired. Ask the user to sign in again.
    #[snafu(display("PKCE flow state has expired."))]
    FlowStateExpired,

    /// PKCE flow state to which the API request relates no longer exists. Ask the user to sign in again.
    #[snafu(display("PKCE flow state not found."))]
    FlowStateNotFound,

    /// The identity to which the API relates is already linked to a user.
    #[snafu(display("Identity already exists."))]
    IdentityAlreadyExists,

    /// Identity to which the API call relates does not exist, such as when an identity is unlinked or deleted.
    #[snafu(display("Identity not found."))]
    IdentityNotFound,

    /// To call this API, the user must have a higher Authenticator Assurance Level. Ask the user to solve an MFA challenge.
    #[snafu(display("Insufficient Authenticator Assurance Level (AAL)."))]
    InsufficientAal,

    /// Invite is expired or already used.
    #[snafu(display("Invite not found."))]
    InviteNotFound,

    /// Calling the supabase.auth.linkUser() and related APIs is not enabled on the Auth server.
    #[snafu(display("Manual linking is disabled."))]
    ManualLinkingDisabled,

    /// Responding to an MFA challenge should happen within a fixed time period. Request a new challenge when encountering this error.
    #[snafu(display("MFA challenge has expired."))]
    MfaChallengeExpired,

    /// MFA factors for a single user should not have the same friendly name.
    #[snafu(display("MFA factor name conflict."))]
    MfaFactorNameConflict,

    /// MFA factor no longer exists.
    #[snafu(display("MFA factor not found."))]
    MfaFactorNotFound,

    /// The enrollment process for MFA factors must begin and end with the same IP address.
    #[snafu(display("IP address mismatch during MFA enrollment."))]
    MfaIpAddressMismatch,

    /// MFA challenge could not be verified -- wrong TOTP code.
    #[snafu(display("MFA verification failed: wrong TOTP code."))]
    MfaVerificationFailed,

    /// Further MFA verification is rejected. Only returned if the MFA verification attempt hook returns a reject decision.
    #[snafu(display("MFA verification rejected."))]
    MfaVerificationRejected,

    /// This HTTP request requires an Authorization header, which is not provided.
    #[snafu(display("No Authorization header provided."))]
    NoAuthorization,

    /// User accessing the API is not admin, i.e. the JWT does not contain a role claim that identifies them as an admin of the Auth server.
    #[snafu(display("User is not an admin."))]
    NotAdmin,

    /// Using an OAuth provider which is disabled on the Auth server.
    #[snafu(display("OAuth provider is not supported."))]
    OauthProviderNotSupported,

    /// Sign in with OTPs (magic link, email OTP) is disabled. Check your server's configuration.
    #[snafu(display("Sign in with OTPs is disabled."))]
    OtpDisabled,

    /// OTP code for this sign-in has expired. Ask the user to sign in again.
    #[snafu(display("OTP code has expired."))]
    OtpExpired,

    /// Too many emails have been sent to this email address. Ask the user to wait a while before trying again.
    #[snafu(display("Too many emails sent to this address."))]
    OverEmailSendRateLimit,

    /// Too many requests have been sent by this client (IP address). Ask the user to try again in a few minutes.
    #[snafu(display("Too many requests sent by this client."))]
    OverRequestRateLimit,

    /// Too many SMS messages have been sent to this phone number. Ask the user to wait a while before trying again.
    #[snafu(display("Too many SMS messages sent to this number."))]
    OverSmsSendRateLimit,

    /// Phone number already exists in the system.
    #[snafu(display("Phone number already exists."))]
    PhoneExists,

    /// Signing in is not allowed for this user as the phone number is not confirmed.
    #[snafu(display("Phone number not confirmed."))]
    PhoneNotConfirmed,

    /// Signups are disabled for phone and password.
    #[snafu(display("Phone provider is disabled."))]
    PhoneProviderDisabled,

    /// OAuth provider is disabled for use. Check your server's configuration.
    #[snafu(display("OAuth provider is disabled."))]
    ProviderDisabled,

    /// Not all OAuth providers verify their user's email address. Supabase Auth requires emails to be verified, so this error is sent out when a verification email is sent after completing the OAuth flow.
    #[snafu(display("OAuth provider email needs verification."))]
    ProviderEmailNeedsVerification,

    /// A user needs to reauthenticate to change their password. Ask the user to reauthenticate by calling the supabase.auth.reauthenticate() API.
    #[snafu(display("Reauthentication is needed."))]
    ReauthenticationNeeded,

    /// Verifying a reauthentication failed, the code is incorrect. Ask the user to enter a new code.
    #[snafu(display("Reauthentication code is not valid."))]
    ReauthenticationNotValid,

    /// A user that is updating their password must use a different password than the one currently used.
    #[snafu(display("New password must be different from the current password."))]
    SamePassword,

    /// SAML assertion (user information) was received after sign in, but no email address was found in it which is required. Check the provider's attribute mapping and/or configuration.
    #[snafu(display("No email found in SAML assertion."))]
    SamlAssertionNoEmail,

    /// SAML assertion (user information) was received after sign in, but a user ID (called NameID) was not found in it which is required. Check the SAML identity provider's configuration.
    #[snafu(display("No user ID found in SAML assertion."))]
    SamlAssertionNoUserId,

    /// Updating the SAML metadata for a SAML identity provider is not possible, as the entity ID in the update does not match the entity ID in the database. This is equivalent to creating a new identity provider.
    #[snafu(display("SAML entity ID mismatch."))]
    SamlEntityIdMismatch,

    /// Adding a SAML identity provider that is already added.
    #[snafu(display("SAML identity provider already exists."))]
    SamlIdpAlreadyExists,

    /// SAML identity provider not found. Most often returned after IdP-initiated sign-in with an unregistered SAML identity provider.
    #[snafu(display("SAML identity provider not found."))]
    SamlIdpNotFound,

    /// Adding or updating a SAML provider failed as its metadata could not be fetched from the provided URL.
    #[snafu(display("Failed to fetch SAML metadata."))]
    SamlMetadataFetchFailed,

    /// Using Enterprise SSO with SAML 2.0 is not enabled on the Auth server.
    #[snafu(display("SAML provider is disabled."))]
    SamlProviderDisabled,

    /// SAML relay state is an object that tracks the progress of a supabase.auth.signInWithSSO() request. The SAML identity provider should respond after a fixed amount of time, after which this error is shown.
    #[snafu(display("SAML relay state has expired."))]
    SamlRelayStateExpired,

    /// SAML relay states are progressively cleaned up after they expire, which can cause this error. Ask the user to sign in again.
    #[snafu(display("SAML relay state not found."))]
    SamlRelayStateNotFound,

    /// Session to which the API request relates no longer exists. This can occur if the user has signed out, or the session entry in the database was deleted in some other way.
    #[snafu(display("Session not found."))]
    SessionNotFound,

    /// Sign ups (new account creation) is disabled on the server.
    #[snafu(display("Signups are disabled."))]
    SignupDisabled,

    /// Every user must have at least one identity attached to it, so deleting (unlinking) an identity is not allowed if it's the only one for the user.
    #[snafu(display("Cannot delete the only identity attached to the user."))]
    SingleIdentityNotDeletable,

    /// Sending an SMS message failed. Check your SMS provider configuration.
    #[snafu(display("Failed to send SMS."))]
    SmsSendFailed,

    /// Only one SSO domain can be registered per SSO identity provider.
    #[snafu(display("SSO domain already exists."))]
    SsoDomainAlreadyExists,

    /// SSO provider not found. Check the arguments in supabase.auth.signInWithSSO().
    #[snafu(display("SSO provider not found."))]
    SsoProviderNotFound,

    /// A user can only have a fixed number of enrolled MFA factors.
    #[snafu(display("Too many enrolled MFA factors."))]
    TooManyEnrolledMfaFactors,

    /// The request's X-JWT-AUD claim does not match the JWT's audience.
    #[snafu(display("Unexpected audience in JWT."))]
    UnexpectedAudience,

    /// Auth service is degraded or a bug is present, without a specific reason.
    #[snafu(display("Unexpected failure in Auth service."))]
    UnexpectedFailure { source: Box<dyn std::error::Error> },

    /// User with this information (email address, phone number) cannot be created again as it already exists.
    #[snafu(display("User already exists."))]
    UserAlreadyExists,

    /// User to which the API request relates has a banned_until property which is still active. No further API requests should be attempted until this field is cleared.
    #[snafu(display("User is banned until a specified time."))]
    UserBanned,

    /// User to which the API request relates no longer exists.
    #[snafu(display("User not found."))]
    UserNotFound,

    /// When a user comes from SSO, certain fields of the user cannot be updated (like email).
    #[snafu(display("User is SSO managed and certain fields cannot be updated."))]
    UserSsoManaged,

    /// Provided parameters are not in the expected format.
    #[snafu(display("Validation of provided parameters failed."))]
    ValidationFailed,

    /// User is signing up or changing their password without meeting the password strength criteria.
    #[snafu(display("Password does not meet strength criteria."))]
    WeakPassword,
}

impl AuthError {
    /// Converts a generic error into an AuthError.
    pub fn from_generic_error<E: std::error::Error + 'static>(error: E) -> Self {
        AuthError::UnexpectedFailure {
            source: Box::new(error),
        }
    }
}
