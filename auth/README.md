# Auth Container Service

This service provides JWT authentication functionality. The configuration for the JWT service is managed through
environment variables, which are used to initialize the `AuthConfig` struct.

## Environment Variables

### JWT_SECRET

- **Description**: The secret key used for signing and verifying JWT tokens.
- **Required**: Yes
- **Example**: `supersecretkey`

### JWT_EXPIRATION

- **Description**: The expiration time for JWT tokens, in seconds.
- **Required**: No (default: `3600`)
- **Example**: `3600`

### JWT_PRIVATE_KEY

- **Description**: The private key used for signing JWT tokens, particularly for asymmetric algorithms.
- **Required**: No
- **Example**: `-----BEGIN PRIVATE KEY-----...-----END PRIVATE KEY-----`

### JWT_PUBLIC_KEY

- **Description**: The public key used for verifying JWT tokens, particularly for asymmetric algorithms.
- **Required**: No
- **Example**: `-----BEGIN PUBLIC KEY-----...-----END PUBLIC KEY-----`

### JWT_LEEWAY

- **Description**: The amount of leeway (in seconds) to add to the `exp` and `nbf` validations to account for clock
  skew.
- **Required**: No (default: `60`)
- **Example**: `60`

### JWT_REJECT_TOKENS_EXPIRING_IN_LESS_THAN

- **Description**: The time (in seconds) before the `exp` at which tokens will be rejected to prevent expiration during
  transit.
- **Required**: No (default: `0`)
- **Example**: `30`

### JWT_VALIDATE_EXP

- **Description**: Whether to validate the `exp` (expiration) claim.
- **Required**: No (default: `true`)
- **Example**: `true`

### JWT_VALIDATE_NBF

- **Description**: Whether to validate the `nbf` (not before) claim.
- **Required**: No (default: `false`)
- **Example**: `false`

### JWT_VALIDATE_AUD

- **Description**: Whether to validate the `aud` (audience) claim.
- **Required**: No (default: `true`)
- **Example**: `true`

### JWT_AUD

- **Description**: The expected audience(s) for the JWT tokens. Multiple audiences can be specified, separated by
  commas.
- **Required**: No
- **Example**: `myaudience1,myaudience2`

### JWT_ISS

- **Description**: The expected issuer(s) for the JWT tokens. Multiple issuers can be specified, separated by commas.
- **Required**: No
- **Example**: `myissuer1,myissuer2`

### JWT_SUB

- **Description**: The expected subject for the JWT tokens.
- **Required**: No
- **Example**: `mysubject`

### Algorithms

By default, the service uses `HS256` algorithm for signing and verifying JWT tokens. This can be adjusted if needed by
modifying the configuration in the source code.

## Example Configuration

Here's an example of how you might set these environment variables in a `.env` file:

```dotenv
JWT_SECRET=supersecretkey
JWT_EXPIRATION=3600
JWT_PRIVATE_KEY=-----BEGIN PRIVATE KEY-----...-----END PRIVATE KEY-----
JWT_PUBLIC_KEY=-----BEGIN PUBLIC KEY-----...-----END PUBLIC KEY-----
JWT_LEEWAY=60
JWT_REJECT_TOKENS_EXPIRING_IN_LESS_THAN=0
JWT_VALIDATE_EXP=true
JWT_VALIDATE_NBF=false
JWT_VALIDATE_AUD=true
JWT_AUD=myaudience1,myaudience2
JWT_ISS=myissuer1,myissuer2
JWT_SUB=mysubject
