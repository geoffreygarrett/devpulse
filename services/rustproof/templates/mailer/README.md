# Email Templates Configuration README

This document provides comprehensive instructions on configuring and using email templates within your application. The
templates support multiple languages, formats (text and HTML), and can be specified via environment variables. The
system allows both file paths and URLs for flexible deployment options.

## Overview

You can customize the email templates by specifying paths or URLs using environment variables. The system dynamically
selects the appropriate template based on the provided `locale` and `format`. If the specified template is not found,
fallback mechanisms ensure that a default version is used.

## Template Arguments

Each template supports specific arguments, which are dynamically populated during rendering. These arguments are used to
personalize the emails.

### Common Arguments Across All Templates

- **`site_url`**: The base URL of your site.
- **`confirmation_url`**: The URL the user should follow to confirm the action.

### Template-Specific Arguments

1. **Invite Email Template** (`MAILER_TEMPLATES_INVITE`)
    - **`inviter_name`**: The name of the person or entity inviting the user (optional).

2. **Confirmation Email Template** (`MAILER_TEMPLATES_CONFIRMATION`)
    - **`user_email`**: The email address of the user confirming the signup.

3. **Password Recovery Email Template** (`MAILER_TEMPLATES_RECOVERY`)
    - **`user_email`**: The email address of the user resetting their password.

4. **Magic Link Email Template** (`MAILER_TEMPLATES_MAGIC_LINK`)
    - **`user_email`**: The email address of the user logging in via magic link.

5. **Email Change Confirmation Template** (`MAILER_TEMPLATES_EMAIL_CHANGE`)
    - **`old_email`**: The current email address of the user.
    - **`new_email`**: The new email address the user is changing to.

## Template Paths

### Path Structure

Specify the template location in the environment variables. Paths can be structured to include placeholders for `locale`
and `format`, allowing for dynamic resolution.

#### Example File Path Structure

```
/path/to/templates/{{locale}}/{{format}}/template_name.hbs
```

#### Example URL Structure

```
https://example.com/templates/{{locale}}/{{format}}/template_name.hbs
```

#### Query Parameters (Alternative)

```
https://example.com/templates/template_name.hbs?locale=en&format=html
```

### Supporting Both File Paths and URLs

The system can differentiate between file paths and URLs:

- **File Path**: Starts with `/`.
- **URL**: Starts with `http://` or `https://`.

### Example Configuration

```bash
export MAILER_TEMPLATES_INVITE="/path/to/templates/{{locale}}/{{format}}/mailer_invite.hbs"
export MAILER_TEMPLATES_CONFIRMATION="https://example.com/templates/{{locale}}/{{format}}/mailer_confirmation.hbs"
export MAILER_TEMPLATES_RECOVERY="/path/to/templates/{{locale}}/{{format}}/mailer_recovery.hbs"
export MAILER_TEMPLATES_MAGIC_LINK="https://example.com/templates/{{locale}}/{{format}}/mailer_magic_link.hbs"
export MAILER_TEMPLATES_EMAIL_CHANGE="/path/to/templates/{{locale}}/{{format}}/mailer_email_change.hbs"
```

## Default Templates

Below are the default text and HTML templates for each email type. These templates use Handlebars (`.hbs`) syntax and
are designed to be customizable via the arguments listed above.

### Invite Email Template

**HTML Version:**

```hbs
{{!--
  Arguments:
  - site_url: The URL of the site inviting the user.
  - confirmation_url: The URL for the user to accept the invite.
  - inviter_name: The name of the person or entity inviting the user (optional).
--}}
<h2>You have been invited</h2>

<p>You have been invited to create a user on {{site_url}}.</p>

{{#if inviter_name}}
    <p>This invitation was sent by {{inviter_name}}.</p>
{{/if}}

<p>Follow this link to accept the invite:</p>

<p><a href="{{confirmation_url}}">Accept the invite</a></p>
```

**Text Version:**

```hbs
{{!--
  Arguments:
  - site_url: The URL of the site inviting the user.
  - confirmation_url: The URL for the user to accept the invite.
  - inviter_name: The name of the person or entity inviting the user (optional).
--}}
You have been invited to create a user on {{site_url}}.

{{#if inviter_name}}
    This invitation was sent by {{inviter_name}}.
{{/if}}

Follow this link to accept the invite:

{{confirmation_url}}
```

### Confirmation Email Template

**HTML Version:**

```hbs
{{!--
  Arguments:
  - site_url: The URL of the site.
  - confirmation_url: The URL for the user to confirm their signup.
  - user_email: The email address of the user.
--}}
<h2>Confirm your signup</h2>

<p>Confirm your signup on {{site_url}}.</p>

<p>User: {{user_email}}</p>

<p>Follow this link to confirm your email:</p>

<p><a href="{{confirmation_url}}">Confirm your email</a></p>
```

**Text Version:**

```hbs
{{!--
  Arguments:
  - site_url: The URL of the site.
  - confirmation_url: The URL for the user to confirm their signup.
  - user_email: The email address of the user.
--}}
Confirm your signup on {{site_url}}.

User: {{user_email}}

Follow this link to confirm your email:

{{confirmation_url}}
```

### Password Recovery Email Template

**HTML Version:**

```hbs
{{!--
  Arguments:
  - site_url: The URL of the site.
  - confirmation_url: The URL for the user to reset their password.
  - user_email: The email address of the user.
--}}
<h2>Reset Password</h2>

<p>Reset your password for {{site_url}}.</p>

<p>User: {{user_email}}</p>

<p>Follow this link to reset your password:</p>

<p><a href="{{confirmation_url}}">Reset Password</a></p>
```

**Text Version:**

```hbs
{{!--
  Arguments:
  - site_url: The URL of the site.
  - confirmation_url: The URL for the user to reset their password.
  - user_email: The email address of the user.
--}}
Reset your password for {{site_url}}.

User: {{user_email}}

Follow this link to reset your password:

{{confirmation_url}}
```

### Magic Link Email Template

**HTML Version:**

```hbs
{{!--
  Arguments:
  - site_url: The URL of the site.
  - confirmation_url: The URL for the user to log in via magic link.
  - user_email: The email address of the user.
--}}
<h2>Magic Link</h2>

<p>Use the following link to log in to {{site_url}}:</p>

<p>User: {{user_email}}</p>

<p><a href="{{confirmation_url}}">Log In</a></p>
```

**Text Version:**

```hbs
{{!--
  Arguments:
  - site_url: The URL of the site.
  - confirmation_url: The URL for the user to log in via magic link.
  - user_email: The email address of the user.
--}}
Use the following link to log in to {{site_url}}:

User: {{user_email}}

{{confirmation_url}}
```

### Email Change Confirmation Template

**HTML Version:**

```hbs
{{!--
  Arguments:
  - site_url: The URL of the site.
  - confirmation_url: The URL for the user to confirm their email change.
  - old_email: The current email address of the user.
  - new_email: The new email address the user is changing to.
--}}
<h2>Confirm Change of Email</h2>

<p>Confirm the change of your email address for {{site_url}}.</p>

<p>Current email: {{old_email}}</p>
<p>New email: {{new_email}}</p>

<p>Follow this link to confirm the change:</p>

<p><a href="{{confirmation_url}}">Change Email</a></p>
```

**Text Version:**

```hbs
{{!--
  Arguments:
  - site_url: The URL of the site.
  - confirmation_url: The URL for the user to confirm their email change.
  - old_email: The current email address of the user.
  - new_email: The new email address the user is changing to.
--}}
Confirm the change of your email address for {{site_url}}.

Current email: {{old_email}}
New email: {{new_email}}

Follow this link to confirm the change:

{{confirmation_url}}
```
