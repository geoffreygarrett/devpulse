use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// NullableSimpleUser : A GitHub user.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct SimpleUser {
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    #[serde(
        rename = "email",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub email: Option<Option<String>>,
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id", deserialize_with = "Option::deserialize")]
    pub gravatar_id: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
    #[serde(rename = "starred_at", skip_serializing_if = "Option::is_none")]
    pub starred_at: Option<String>,
}

impl SimpleUser {
    /// A GitHub user.
    pub fn new(
        login: String, id: i64, node_id: String, avatar_url: String, gravatar_id: Option<String>,
        url: String, html_url: String, followers_url: String, following_url: String,
        gists_url: String, starred_url: String, subscriptions_url: String,
        organizations_url: String, repos_url: String, events_url: String,
        received_events_url: String, r#type: String, site_admin: bool,
    ) -> SimpleUser {
        SimpleUser {
            name: None,
            email: None,
            login,
            id,
            node_id,
            avatar_url,
            gravatar_id,
            url,
            html_url,
            followers_url,
            following_url,
            gists_url,
            starred_url,
            subscriptions_url,
            organizations_url,
            repos_url,
            events_url,
            received_events_url,
            r#type,
            site_admin,
            starred_at: None,
        }
    }

    pub fn builder() -> SimpleUserBuilder {
        SimpleUserBuilder::default()
    }
}

mod _impl_github {
    use external_github;

    impl From<external_github::models::SimpleUser> for super::SimpleUser {
        fn from(internal: external_github::models::SimpleUser) -> Self {
            super::SimpleUser::new(
                internal.login,
                internal.id,
                internal.node_id,
                internal.avatar_url,
                internal.gravatar_id,
                internal.url,
                internal.html_url,
                internal.followers_url,
                internal.following_url,
                internal.gists_url,
                internal.starred_url,
                internal.subscriptions_url,
                internal.organizations_url,
                internal.repos_url,
                internal.events_url,
                internal.received_events_url,
                internal.r#type,
                internal.site_admin,
            )
        }
    }

    impl From<external_github::models::NullableSimpleUser> for super::SimpleUser {
        fn from(internal: external_github::models::NullableSimpleUser) -> Self {
            super::SimpleUser::new(
                internal.login,
                internal.id,
                internal.node_id,
                internal.avatar_url,
                internal.gravatar_id,
                internal.url,
                internal.html_url,
                internal.followers_url,
                internal.following_url,
                internal.gists_url,
                internal.starred_url,
                internal.subscriptions_url,
                internal.organizations_url,
                internal.repos_url,
                internal.events_url,
                internal.received_events_url,
                internal.r#type,
                internal.site_admin,
            )
        }
    }

    impl From<super::SimpleUser> for external_github::models::SimpleUser {
        fn from(external: super::SimpleUser) -> Self {
            external_github::models::SimpleUser {
                name: external.name,
                email: external.email,
                login: external.login,
                id: external.id,
                node_id: external.node_id,
                avatar_url: external.avatar_url,
                gravatar_id: external.gravatar_id,
                url: external.url,
                html_url: external.html_url,
                followers_url: external.followers_url,
                following_url: external.following_url,
                gists_url: external.gists_url,
                starred_url: external.starred_url,
                subscriptions_url: external.subscriptions_url,
                organizations_url: external.organizations_url,
                repos_url: external.repos_url,
                events_url: external.events_url,
                received_events_url: external.received_events_url,
                r#type: external.r#type,
                site_admin: external.site_admin,
                starred_at: external.starred_at,
            }
        }
    }

    impl From<super::SimpleUser> for external_github::models::NullableSimpleUser {
        fn from(external: super::SimpleUser) -> Self {
            external_github::models::NullableSimpleUser {
                name: external.name,
                email: external.email,
                login: external.login,
                id: external.id,
                node_id: external.node_id,
                avatar_url: external.avatar_url,
                gravatar_id: external.gravatar_id,
                url: external.url,
                html_url: external.html_url,
                followers_url: external.followers_url,
                following_url: external.following_url,
                gists_url: external.gists_url,
                starred_url: external.starred_url,
                subscriptions_url: external.subscriptions_url,
                organizations_url: external.organizations_url,
                repos_url: external.repos_url,
                events_url: external.events_url,
                received_events_url: external.received_events_url,
                r#type: external.r#type,
                site_admin: external.site_admin,
                starred_at: external.starred_at,
            }
        }
    }

    /// Converts an Option<SimpleUser> to NullableSimpleUser
    pub fn to_nullable_simple_user(
        user: Option<super::SimpleUser>,
    ) -> external_github::models::NullableSimpleUser {
        match user {
            Some(user) => external_github::models::NullableSimpleUser {
                name: user.name,
                email: user.email,
                login: user.login,
                id: user.id,
                node_id: user.node_id,
                avatar_url: user.avatar_url,
                gravatar_id: user.gravatar_id,
                url: user.url,
                html_url: user.html_url,
                followers_url: user.followers_url,
                following_url: user.following_url,
                gists_url: user.gists_url,
                starred_url: user.starred_url,
                subscriptions_url: user.subscriptions_url,
                organizations_url: user.organizations_url,
                repos_url: user.repos_url,
                events_url: user.events_url,
                received_events_url: user.received_events_url,
                r#type: user.r#type,
                site_admin: user.site_admin,
                starred_at: user.starred_at,
            },
            None => external_github::models::NullableSimpleUser::default(),
        }
    }

    /// Converts a NullableSimpleUser to Option<SimpleUser>
    pub fn from_nullable_simple_user(
        internal: external_github::models::NullableSimpleUser,
    ) -> Option<super::SimpleUser> {
        Some(super::SimpleUser {
            name: Option::from(internal.name.flatten()),
            email: Option::from(internal.email.flatten()),
            login: internal.login,
            id: internal.id,
            node_id: internal.node_id,
            avatar_url: internal.avatar_url,
            gravatar_id: internal.gravatar_id,
            url: internal.url,
            html_url: internal.html_url,
            followers_url: internal.followers_url,
            following_url: internal.following_url,
            gists_url: internal.gists_url,
            starred_url: internal.starred_url,
            subscriptions_url: internal.subscriptions_url,
            organizations_url: internal.organizations_url,
            repos_url: internal.repos_url,
            events_url: internal.events_url,
            received_events_url: internal.received_events_url,
            r#type: internal.r#type,
            site_admin: internal.site_admin,
            starred_at: internal.starred_at,
        })
    }
}
