# GitChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**change_id** | Option<**i32**> | ID of the change within the group of changes. | [optional]
**new_content_template** | Option<[**models::GitTemplate**](GitTemplate.md)> |  | [optional]
**original_path** | Option<**String**> | Original path of item if different from current path. | [optional]
**change_type** | Option<**String**> | The type of change that was made to the item. | [optional]
**item** | Option<**String**> | Current version. | [optional]
**new_content** | Option<[**models::ItemContent**](ItemContent.md)> |  | [optional]
**source_server_item** | Option<**String**> | Path of the item on the server. | [optional]
**url** | Option<**String**> | URL to retrieve the item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

