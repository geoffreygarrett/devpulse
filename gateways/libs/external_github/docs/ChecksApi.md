# \ChecksApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**checks_slash_list_for_ref**](ChecksApi.md#checks_slash_list_for_ref) | **GET** /repos/{owner}/{repo}/commits/{ref}/check-runs | List check runs for a Git reference



## checks_slash_list_for_ref

> models::ChecksListForRef200Response checks_slash_list_for_ref(owner, repo, r#ref, check_name, status, filter, per_page, page, app_id)
List check runs for a Git reference

Lists check runs for a commit ref. The `ref` can be a SHA, branch name, or a tag name.  > [!NOTE] > The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `p

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | **String** | The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see \"[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)\" in the Git documentation. | [required] |
**check_name** | Option<**String**> | Returns check runs with the specified `name`. |  |
**status** | Option<**String**> | Returns check runs with the specified `status`. |  |
**filter** | Option<**String**> | Filters check runs by their `completed_at` timestamp. `latest` returns the most recent check runs. |  |[default to latest]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**app_id** | Option<**i32**> |  |  |

### Return type

[**models::ChecksListForRef200Response**](checks_list_for_ref_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

