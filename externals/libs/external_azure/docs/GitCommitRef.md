# GitCommitRef

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_links** | Option<[**models::ReferenceLinks**](ReferenceLinks.md)> |  | [optional]
**author** | Option<[**models::GitUserDate**](GitUserDate.md)> |  | [optional]
**change_counts** | Option<[**models::ChangeCountDictionary**](ChangeCountDictionary.md)> |  | [optional]
**changes** | Option<[**Vec<models::GitChange>**](GitChange.md)> | An enumeration of the changes included with the commit. | [optional]
**comment** | Option<**String**> | Comment or message of the commit. | [optional]
**comment_truncated** | Option<**bool**> | Indicates if the comment is truncated from the full Git commit comment message. | [optional]
**commit_id** | Option<**String**> | ID (SHA-1) of the commit. | [optional]
**committer** | Option<[**models::GitUserDate**](GitUserDate.md)> |  | [optional]
**commit_too_many_changes** | Option<**bool**> | Indicates that commit contains too many changes to be displayed | [optional]
**parents** | Option<**Vec<String>**> | An enumeration of the parent commit IDs for this commit. | [optional]
**push** | Option<[**models::GitPushRef**](GitPushRef.md)> |  | [optional]
**remote_url** | Option<**String**> | Remote URL path to the commit. | [optional]
**statuses** | Option<[**Vec<models::GitStatus>**](GitStatus.md)> | A list of status metadata from services and extensions that may associate additional information to the commit. | [optional]
**url** | Option<**String**> | REST URL for this resource. | [optional]
**work_items** | Option<[**Vec<models::ResourceRef>**](ResourceRef.md)> | A list of workitems associated with this commit. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


