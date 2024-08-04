# PullRequestReviewComment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | URL for the pull request review comment | 
**pull_request_review_id** | Option<**i64**> | The ID of the pull request review to which the comment belongs. | 
**id** | **i64** | The ID of the pull request review comment. | 
**node_id** | **String** | The node ID of the pull request review comment. | 
**diff_hunk** | **String** | The diff of the line that the comment refers to. | 
**path** | **String** | The relative path of the file to which the comment applies. | 
**position** | Option<**i32**> | The line index in the diff to which the comment applies. This field is deprecated; use `line` instead. | [optional]
**original_position** | Option<**i32**> | The index of the original line in the diff to which the comment applies. This field is deprecated; use `original_line` instead. | [optional]
**commit_id** | **String** | The SHA of the commit to which the comment applies. | 
**original_commit_id** | **String** | The SHA of the original commit to which the comment applies. | 
**in_reply_to_id** | Option<**i32**> | The comment ID to reply to. | [optional]
**user** | [**models::SimpleUser**](simple-user.md) |  | 
**body** | **String** | The text of the comment. | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**html_url** | **String** | HTML URL for the pull request review comment. | 
**pull_request_url** | **String** | URL for the pull request that the review comment belongs to. | 
**author_association** | [**models::AuthorAssociation**](author-association.md) |  | 
**_links** | [**models::PullRequestReviewCommentLinks**](pull_request_review_comment__links.md) |  | 
**start_line** | Option<**i32**> | The first line of the range for a multi-line comment. | [optional]
**original_start_line** | Option<**i32**> | The first line of the range for a multi-line comment. | [optional]
**start_side** | Option<**String**> | The side of the first line of the range for a multi-line comment. | [optional][default to Right]
**line** | Option<**i32**> | The line of the blob to which the comment applies. The last line of the range for a multi-line comment | [optional]
**original_line** | Option<**i32**> | The line of the blob to which the comment applies. The last line of the range for a multi-line comment | [optional]
**side** | Option<**String**> | The side of the diff to which the comment applies. The side of the last line of the range for a multi-line comment | [optional][default to Right]
**subject_type** | Option<**String**> | The level at which the comment is targeted, can be a diff line or a file. | [optional]
**reactions** | Option<[**models::ReactionRollup**](reaction-rollup.md)> |  | [optional]
**body_html** | Option<**String**> |  | [optional]
**body_text** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


