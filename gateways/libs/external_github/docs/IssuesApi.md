# \IssuesApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**issues_slash_create_comment**](IssuesApi.md#issues_slash_create_comment) | **POST** /repos/{owner}/{repo}/issues/{issue_number}/comments | Create an issue comment
[**issues_slash_delete_comment**](IssuesApi.md#issues_slash_delete_comment) | **DELETE** /repos/{owner}/{repo}/issues/comments/{comment_id} | Delete an issue comment
[**issues_slash_list_comments**](IssuesApi.md#issues_slash_list_comments) | **GET** /repos/{owner}/{repo}/issues/{issue_number}/comments | List issue comments
[**issues_slash_update_comment**](IssuesApi.md#issues_slash_update_comment) | **PATCH** /repos/{owner}/{repo}/issues/comments/{comment_id} | Update an issue comment



## issues_slash_create_comment

> models::IssueComment issues_slash_create_comment(owner, repo, issue_number, issues_create_comment_request)
Create an issue comment

You can use the REST API to create comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.  This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating con

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**issue_number** | **i32** | The number that identifies the issue. | [required] |
**issues_create_comment_request** | [**IssuesCreateCommentRequest**](IssuesCreateCommentRequest.md) |  | [required] |

### Return type

[**models::IssueComment**](issue-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issues_slash_delete_comment

> issues_slash_delete_comment(owner, repo, comment_id)
Delete an issue comment

You can use the REST API to delete comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i32** | The unique identifier of the comment. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issues_slash_list_comments

> Vec<models::IssueComment> issues_slash_list_comments(owner, repo, issue_number, since, per_page, page)
List issue comments

You can use the REST API to list comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.  Issue comments are ordered by ascending ID.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.g

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**issue_number** | **i32** | The number that identifies the issue. | [required] |
**since** | Option<**String**> | Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::IssueComment>**](issue-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issues_slash_update_comment

> models::IssueComment issues_slash_update_comment(owner, repo, comment_id, issues_create_comment_request)
Update an issue comment

You can use the REST API to update comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-s

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i32** | The unique identifier of the comment. | [required] |
**issues_create_comment_request** | [**IssuesCreateCommentRequest**](IssuesCreateCommentRequest.md) |  | [required] |

### Return type

[**models::IssueComment**](issue-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

