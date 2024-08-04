# \OrgsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**orgs_slash_get**](OrgsApi.md#orgs_slash_get) | **GET** /orgs/{org} | Get an organization
[**orgs_slash_list_for_authenticated_user**](OrgsApi.md#orgs_slash_list_for_authenticated_user) | **GET** /user/orgs | List organizations for the authenticated user
[**orgs_slash_list_members**](OrgsApi.md#orgs_slash_list_members) | **GET** /orgs/{org}/members | List organization members



## orgs_slash_get

> models::OrganizationFull orgs_slash_get(org)
Get an organization

Gets information about an organization.  When the value of `two_factor_requirement_enabled` is `true`, the organization requires all members, billing managers, and outside collaborators to enable [two-factor authentication](https://docs.github.com/articles/securing-your-account-with-two-factor-authe

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::OrganizationFull**](organization-full.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_for_authenticated_user

> Vec<models::OrganizationSimple> orgs_slash_list_for_authenticated_user(per_page, page)
List organizations for the authenticated user

List organizations for the authenticated user.  For OAuth app tokens and personal access tokens (classic), this endpoint only lists organizations that your authorization allows you to operate on in some way (e.g., you can list teams with `read:org` scope, you can publicize your organization membersh

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::OrganizationSimple>**](organization-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_members

> Vec<models::SimpleUser> orgs_slash_list_members(org, filter, role, per_page, page)
List organization members

List all users who are members of an organization. If the authenticated user is also a member of this organization then both concealed and public members will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**filter** | Option<**String**> | Filter members returned in the list. `2fa_disabled` means that only members without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled will be returned. This options is only available for organization owners. |  |[default to all]
**role** | Option<**String**> | Filter members returned by their role. |  |[default to all]
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

