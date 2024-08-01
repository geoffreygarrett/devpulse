# Commit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | 
**sha** | **String** |  | 
**node_id** | **String** |  | 
**html_url** | **String** |  | 
**comments_url** | **String** |  | 
**commit** | [**models::CommitCommit**](commit_commit.md) |  | 
**author** | Option<[**models::CommitAuthor**](commit_author.md)> |  | 
**committer** | Option<[**models::CommitAuthor**](commit_author.md)> |  | 
**parents** | [**Vec<models::CommitParentsInner>**](commit_parents_inner.md) |  | 
**stats** | Option<[**models::CommitStats**](commit_stats.md)> |  | [optional]
**files** | Option<[**Vec<models::DiffEntry>**](diff-entry.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


