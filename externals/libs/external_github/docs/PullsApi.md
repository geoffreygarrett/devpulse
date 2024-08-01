# \PullsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pulls_slash_create**](PullsApi.md#pulls_slash_create) | **POST** /repos/{owner}/{repo}/pulls | Create a pull request
[**pulls_slash_get**](PullsApi.md#pulls_slash_get) | **GET** /repos/{owner}/{repo}/pulls/{pull_number} | Get a pull request
[**pulls_slash_list**](PullsApi.md#pulls_slash_list) | **GET** /repos/{owner}/{repo}/pulls | List pull requests



## pulls_slash_create

> models::PullRequest pulls_slash_create(owner, repo, pulls_create_request)
Create a pull request

Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https:/

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**pulls_create_request** | [**PullsCreateRequest**](PullsCreateRequest.md) |  | [required] |

### Return type

[**models::PullRequest**](pull-request.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pulls_slash_get

> models::PullRequest pulls_slash_get(owner, repo, pull_number)
Get a pull request

Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https:/

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**pull_number** | **i32** | The number that identifies the pull request. | [required] |

### Return type

[**models::PullRequest**](pull-request.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pulls_slash_list

> Vec<models::PullRequestSimple> pulls_slash_list(owner, repo, state, head, base, sort, direction, per_page, page)
List pull requests

Lists pull requests in a specified repository.  Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For m

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**state** | Option<**String**> | Either `open`, `closed`, or `all` to filter by state. |  |[default to open]
**head** | Option<**String**> | Filter pulls by head user or head organization and branch name in the format of `user:ref-name` or `organization:ref-name`. For example: `github:new-script-format` or `octocat:test-branch`. |  |
**base** | Option<**String**> | Filter pulls by base branch name. Example: `gh-pages`. |  |
**sort** | Option<**String**> | What to sort results by. `popularity` will sort by the number of comments. `long-running` will sort by date created and will limit the results to pull requests that have been open for more than a month and have had activity within the past month. |  |[default to created]
**direction** | Option<**String**> | The direction of the sort. Default: `desc` when sort is `created` or sort is not specified, otherwise `asc`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::PullRequestSimple>**](pull-request-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

