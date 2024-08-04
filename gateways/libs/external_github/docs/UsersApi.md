# \UsersApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_slash_check_person_is_followed_by_authenticated**](UsersApi.md#users_slash_check_person_is_followed_by_authenticated) | **GET** /user/following/{username} | Check if a person is followed by the authenticated user
[**users_slash_get_authenticated**](UsersApi.md#users_slash_get_authenticated) | **GET** /user | Get the authenticated user
[**users_slash_get_by_username**](UsersApi.md#users_slash_get_by_username) | **GET** /users/{username} | Get a user
[**users_slash_list_followers_for_authenticated_user**](UsersApi.md#users_slash_list_followers_for_authenticated_user) | **GET** /user/followers | List followers of the authenticated user
[**users_slash_list_followers_for_user**](UsersApi.md#users_slash_list_followers_for_user) | **GET** /users/{username}/followers | List followers of a user
[**users_slash_list_following_for_user**](UsersApi.md#users_slash_list_following_for_user) | **GET** /users/{username}/following | List the people a user follows
[**users_slash_list_gpg_keys_for_user**](UsersApi.md#users_slash_list_gpg_keys_for_user) | **GET** /users/{username}/gpg_keys | List GPG keys for a user
[**users_slash_list_public_keys_for_user**](UsersApi.md#users_slash_list_public_keys_for_user) | **GET** /users/{username}/keys | List public keys for a user



## users_slash_check_person_is_followed_by_authenticated

> users_slash_check_person_is_followed_by_authenticated(username)
Check if a person is followed by the authenticated user



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_get_authenticated

> models::UsersGetAuthenticated200Response users_slash_get_authenticated()
Get the authenticated user

OAuth app tokens and personal access tokens (classic) need the `user` scope in order for the response to include private profile information.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UsersGetAuthenticated200Response**](users_get_authenticated_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_get_by_username

> models::UsersGetAuthenticated200Response users_slash_get_by_username(username)
Get a user

Provides publicly available information about someone with a GitHub account.  The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::UsersGetAuthenticated200Response**](users_get_authenticated_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_followers_for_authenticated_user

> Vec<models::SimpleUser> users_slash_list_followers_for_authenticated_user(per_page, page)
List followers of the authenticated user

Lists the people following the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_followers_for_user

> Vec<models::SimpleUser> users_slash_list_followers_for_user(username, per_page, page)
List followers of a user

Lists the people following the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_following_for_user

> Vec<models::SimpleUser> users_slash_list_following_for_user(username, per_page, page)
List the people a user follows

Lists the people who the specified user follows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_gpg_keys_for_user

> Vec<models::GpgKey> users_slash_list_gpg_keys_for_user(username, per_page, page)
List GPG keys for a user

Lists the GPG keys for a user. This information is accessible by anyone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::GpgKey>**](gpg-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_public_keys_for_user

> Vec<models::KeySimple> users_slash_list_public_keys_for_user(username, per_page, page)
List public keys for a user

Lists the _verified_ public SSH keys for a user. This is accessible by anyone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::KeySimple>**](key-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

