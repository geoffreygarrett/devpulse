# PullsCreateReviewCommentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | **String** | The text of the review comment. | 
**commit_id** | **String** | The SHA of the commit needing a comment. Not using the latest commit SHA may render your comment outdated if a subsequent commit modifies the line you specify as the `position`. | 
**path** | **String** | The relative path to the file that necessitates a comment. | 
**position** | Option<**i32**> | **This parameter is deprecated. Use `line` instead**. The position in the diff where you want to add a review comment. Note this value is not the same as the line number in the file. The position value equals the number of lines down from the first \"@@\" hunk header in the file you want to add a comm | [optional]
**side** | Option<**String**> | In a split diff view, the side of the diff that the pull request's changes appear on. Can be `LEFT` or `RIGHT`. Use `LEFT` for deletions that appear in red. Use `RIGHT` for additions that appear in green or unchanged lines that appear in white and are shown for context. For a multi-line comment, sid | [optional]
**line** | Option<**i32**> | **Required unless using `subject_type:file`**. The line of the blob in the pull request diff that the comment applies to. For a multi-line comment, the last line of the range that your comment applies to. | [optional]
**start_line** | Option<**i32**> | **Required when using multi-line comments unless using `in_reply_to`**. The `start_line` is the first line in the pull request diff that your multi-line comment applies to. To learn more about multi-line comments, see \"[Commenting on a pull request](https://docs.github.com/articles/commenting-on-a-p | [optional]
**start_side** | Option<**String**> | **Required when using multi-line comments unless using `in_reply_to`**. The `start_side` is the starting side of the diff that the comment applies to. Can be `LEFT` or `RIGHT`. To learn more about multi-line comments, see \"[Commenting on a pull request](https://docs.github.com/articles/commenting-on | [optional]
**in_reply_to** | Option<**i32**> | The ID of the review comment to reply to. To find the ID of a review comment with [\"List review comments on a pull request\"](#list-review-comments-on-a-pull-request). When specified, all parameters other than `body` in the request body are ignored. | [optional]
**subject_type** | Option<**String**> | The level at which the comment is targeted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


