// mod external;
//
// use crate::core::models as core_models;
// use crate::external_azure::models as azure_models;
// use crate::external_github::models as github_models;
//
// // Conversion for GitHub CommitAuthor: BLL -> API
// impl From<github_models::CommitAuthor> for core_models::CommitAuthor {
//     fn from(author: github_models::CommitAuthor) -> Self {
//         match author {
//             github_models::CommitAuthor::SimpleUser(user) => core_models::CommitAuthor {
//                 author: Some(core_models::commit_author::Author::UserInfo(user.into())),
//             },
//             github_models::CommitAuthor::EmptyObject(_) => core_models::CommitAuthor {
//                 author: Some(core_models::commit_author::Author::EmptyObject(
//                     prost_types::Empty::default(),
//                 )),
//             },
//         }
//     }
// }
//
// // Conversion for GitHub CommitAuthor: API -> BLL
// impl From<core_models::CommitAuthor> for github_models::CommitAuthor {
//     fn from(author: core_models::CommitAuthor) -> Self {
//         match author.author {
//             Some(core_models::commit_author::Author::UserInfo(user)) => {
//                 github_models::CommitAuthor::SimpleUser(Box::new(user.into()))
//             }
//             Some(core_models::commit_author::Author::EmptyObject(_)) => {
//                 github_models::CommitAuthor::EmptyObject(serde_json::Value::default())
//             }
//             None => github_models::CommitAuthor::EmptyObject(serde_json::Value::default()),
//         }
//     }
// }
//
// // Conversion for Azure GitUserDate: BLL -> API
// impl From<azure_models::GitUserDate> for core_models::GitUserDate {
//     fn from(user_date: azure_models::GitUserDate) -> Self {
//         core_models::GitUserDate {
//             user_info: Some(user_date.into()),
//         }
//     }
// }
//
// // Conversion for Azure GitUserDate: API -> BLL
// impl From<core_models::GitUserDate> for azure_models::GitUserDate {
//     fn from(user_date: core_models::GitUserDate) -> Self {
//         azure_models::GitUserDate {
//             date: user_date.user_info.and_then(|info| info.date),
//             email: user_date.user_info.and_then(|info| info.email),
//             image_url: user_date.user_info.and_then(|info| info.image_url),
//             name: user_date.user_info.and_then(|info| info.name),
//         }
//     }
// }
//
// // Conversion for common UserInfo: GitHub -> Core
// impl From<github_models::SimpleUser> for core_models::UserInfo {
//     fn from(user: github_models::SimpleUser) -> Self {
//         core_models::UserInfo {
//             date: None,
//             email: Some(user.email),
//             image_url: Some(user.avatar_url),
//             name: Some(user.login),
//         }
//     }
// }
//
// // Conversion for common UserInfo: Core -> GitHub
// impl From<core_models::UserInfo> for github_models::SimpleUser {
//     fn from(user: core_models::UserInfo) -> Self {
//         github_models::SimpleUser {
//             email: user.email.unwrap_or_default(),
//             avatar_url: user.image_url.unwrap_or_default(),
//             login: user.name.unwrap_or_default(),
//         }
//     }
// }
//
// // Conversion for common UserInfo: Azure -> Core
// impl From<azure_models::GitUserDate> for core_models::UserInfo {
//     fn from(user: azure_models::GitUserDate) -> Self {
//         core_models::UserInfo {
//             date: user.date,
//             email: user.email,
//             image_url: user.image_url,
//             name: user.name,
//         }
//     }
// }
//
// // Conversion for common UserInfo: Core -> Azure
// impl From<core_models::UserInfo> for azure_models::GitUserDate {
//     fn from(user: core_models::UserInfo) -> Self {
//         azure_models::GitUserDate {
//             date: user.date,
//             email: user.email,
//             image_url: user.image_url,
//             name: user.name,
//         }
//     }
// }
