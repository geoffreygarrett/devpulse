# \CommitsApi

All URIs are relative to *https://dev.azure.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**commits_get**](CommitsApi.md#commits_get) | **GET** /{organization}/{project}/_apis/git/repositories/{repositoryId}/commits/{commitId} | 
[**commits_get_changes**](CommitsApi.md#commits_get_changes) | **GET** /{organization}/{project}/_apis/git/repositories/{repositoryId}/commits/{commitId}/changes | 



## commits_get

> models::GitCommit commits_get(organization, commit_id, repository_id, project, api_version, change_count)


Retrieve a particular commit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The name of the Azure DevOps organization. | [required] |
**commit_id** | **String** | The id of the commit. | [required] |
**repository_id** | **String** | The id or friendly name of the repository. To use the friendly name, projectId must also be specified. | [required] |
**project** | **String** | Project ID or project name | [required] |
**api_version** | **String** | Version of the API to use.  This should be set to '7.1-preview.1' to use this version of the api. | [required] |
**change_count** | Option<**i32**> | The number of changes to include in the result. |  |

### Return type

[**models::GitCommit**](GitCommit.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commits_get_changes

> models::GitCommitChanges commits_get_changes(organization, commit_id, repository_id, project, api_version, top, skip)


Retrieve changes for a particular commit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The name of the Azure DevOps organization. | [required] |
**commit_id** | **String** | The id of the commit. | [required] |
**repository_id** | **String** | The id or friendly name of the repository. To use the friendly name, projectId must also be specified. | [required] |
**project** | **String** | Project ID or project name | [required] |
**api_version** | **String** | Version of the API to use.  This should be set to '7.1-preview.1' to use this version of the api. | [required] |
**top** | Option<**i32**> | The maximum number of changes to return. |  |
**skip** | Option<**i32**> | The number of changes to skip. |  |

### Return type

[**models::GitCommitChanges**](GitCommitChanges.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

