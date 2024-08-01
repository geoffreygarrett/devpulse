# IdentityRef

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**directory_alias** | Option<**String**> | Deprecated - Can be retrieved by querying the Graph user referenced in the \"self\" entry of the IdentityRef \"_links\" dictionary | [optional]
**id** | Option<**String**> |  | [optional]
**image_url** | Option<**String**> | Deprecated - Available in the \"avatar\" entry of the IdentityRef \"_links\" dictionary | [optional]
**inactive** | Option<**bool**> | Deprecated - Can be retrieved by querying the Graph membership state referenced in the \"membershipState\" entry of the GraphUser \"_links\" dictionary | [optional]
**is_aad_identity** | Option<**bool**> | Deprecated - Can be inferred from the subject type of the descriptor (Descriptor.IsAadUserType/Descriptor.IsAadGroupType) | [optional]
**is_container** | Option<**bool**> | Deprecated - Can be inferred from the subject type of the descriptor (Descriptor.IsGroupType) | [optional]
**is_deleted_in_origin** | Option<**bool**> |  | [optional]
**profile_url** | Option<**String**> | Deprecated - not in use in most preexisting implementations of ToIdentityRef | [optional]
**unique_name** | Option<**String**> | Deprecated - use Domain+PrincipalName instead | [optional]
**_links** | Option<[**models::ReferenceLinks**](ReferenceLinks.md)> |  | [optional]
**descriptor** | Option<**String**> | The descriptor is the primary way to reference the graph subject while the system is running. This field will uniquely identify the same graph subject across both Accounts and Organizations. | [optional]
**display_name** | Option<**String**> | This is the non-unique display name of the graph subject. To change this field, you must alter its value in the source provider. | [optional]
**url** | Option<**String**> | This url is the full route to the source resource of this graph subject. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


