# Interacting with Discussions and Pull Requests

Check the [HfApi](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi) documentation page for the reference of methods enabling
interaction with Pull Requests and Discussions on the Hub.

- [get_repo_discussions()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.get_repo_discussions)
- [get_discussion_details()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.get_discussion_details)
- [create_discussion()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.create_discussion)
- [create_pull_request()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.create_pull_request)
- [rename_discussion()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.rename_discussion)
- [comment_discussion()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.comment_discussion)
- [edit_discussion_comment()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.edit_discussion_comment)
- [change_discussion_status()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.change_discussion_status)
- [merge_pull_request()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.merge_pull_request)

## Data structures[[huggingface_hub.Discussion]]

- **title** (`str`) --
  The title of the Discussion / Pull Request
- **status** (`str`) --
  The status of the Discussion / Pull Request.
  It must be one of:
  * `"open"`
  * `"closed"`
  * `"merged"` (only for Pull Requests )
  * `"draft"` (only for Pull Requests )
- **num** (`int`) --
  The number of the Discussion / Pull Request.
- **repo_id** (`str`) --
  The id (`"{namespace}/{repo_name}"`) of the repo on which
  the Discussion / Pull Request was open.
- **repo_type** (`str`) --
  The type of the repo on which the Discussion / Pull Request was open.
  Possible values are: `"model"`, `"dataset"`, `"space"`.
- **author** (`str`) --
  The username of the Discussion / Pull Request author.
  Can be `"deleted"` if the user has been deleted since.
- **is_pull_request** (`bool`) --
  Whether or not this is a Pull Request.
- **created_at** (`datetime`) --
  The `datetime` of creation of the Discussion / Pull Request.
- **endpoint** (`str`) --
  Endpoint of the Hub. Default is https://huggingface.co.
- **git_reference** (`str`, *optional*) --
  (property) Git reference to which changes can be pushed if this is a Pull Request, `None` otherwise.
- **url** (`str`) --
  (property) URL of the discussion on the Hub.

A Discussion or Pull Request on the Hub.

This dataclass is not intended to be instantiated directly.

- **title** (`str`) --
  The title of the Discussion / Pull Request
- **status** (`str`) --
  The status of the Discussion / Pull Request.
  It can be one of:
  * `"open"`
  * `"closed"`
  * `"merged"` (only for Pull Requests )
  * `"draft"` (only for Pull Requests )
- **num** (`int`) --
  The number of the Discussion / Pull Request.
- **repo_id** (`str`) --
  The id (`"{namespace}/{repo_name}"`) of the repo on which
  the Discussion / Pull Request was open.
- **repo_type** (`str`) --
  The type of the repo on which the Discussion / Pull Request was open.
  Possible values are: `"model"`, `"dataset"`, `"space"`.
- **author** (`str`) --
  The username of the Discussion / Pull Request author.
  Can be `"deleted"` if the user has been deleted since.
- **is_pull_request** (`bool`) --
  Whether or not this is a Pull Request.
- **created_at** (`datetime`) --
  The `datetime` of creation of the Discussion / Pull Request.
- **events** (`list` of [DiscussionEvent](/docs/huggingface_hub/v1.25.1/en/package_reference/community#huggingface_hub.DiscussionEvent)) --
  The list of `DiscussionEvents` in this Discussion or Pull Request.
- **conflicting_files** (`Union[list[str], bool, None]`, *optional*) --
  A list of conflicting files if this is a Pull Request.
  `None` if `self.is_pull_request` is `False`.
  `True` if there are conflicting files but the list can't be retrieved.
- **target_branch** (`str`, *optional*) --
  The branch into which changes are to be merged if this is a
  Pull Request . `None`  if `self.is_pull_request` is `False`.
- **merge_commit_oid** (`str`, *optional*) --
  If this is a merged Pull Request , this is set to the OID / SHA of
  the merge commit, `None` otherwise.
- **diff** (`str`, *optional*) --
  The git diff if this is a Pull Request , `None` otherwise.
- **endpoint** (`str`) --
  Endpoint of the Hub. Default is https://huggingface.co.
- **git_reference** (`str`, *optional*) --
  (property) Git reference to which changes can be pushed if this is a Pull Request, `None` otherwise.
- **url** (`str`) --
  (property) URL of the discussion on the Hub.

Subclass of [Discussion](/docs/huggingface_hub/v1.25.1/en/package_reference/community#huggingface_hub.Discussion).

- **id** (`str`) --
  The ID of the event. An hexadecimal string.
- **type** (`str`) --
  The type of the event.
- **created_at** (`datetime`) --
  A [`datetime`](https://docs.python.org/3/library/datetime.html?highlight=datetime#datetime.datetime)
  object holding the creation timestamp for the event.
- **author** (`str`) --
  The username of the Discussion / Pull Request author.
  Can be `"deleted"` if the user has been deleted since.

An event in a Discussion or Pull Request.

Use concrete classes:
* [DiscussionComment](/docs/huggingface_hub/v1.25.1/en/package_reference/community#huggingface_hub.DiscussionComment)
* [DiscussionStatusChange](/docs/huggingface_hub/v1.25.1/en/package_reference/community#huggingface_hub.DiscussionStatusChange)
* [DiscussionCommit](/docs/huggingface_hub/v1.25.1/en/package_reference/community#huggingface_hub.DiscussionCommit)
* [DiscussionTitleChange](/docs/huggingface_hub/v1.25.1/en/package_reference/community#huggingface_hub.DiscussionTitleChange)

- **id** (`str`) --
  The ID of the event. An hexadecimal string.
- **type** (`str`) --
  The type of the event.
- **created_at** (`datetime`) --
  A [`datetime`](https://docs.python.org/3/library/datetime.html?highlight=datetime#datetime.datetime)
  object holding the creation timestamp for the event.
- **author** (`str`) --
  The username of the Discussion / Pull Request author.
  Can be `"deleted"` if the user has been deleted since.
- **content** (`str`) --
  The raw markdown content of the comment. Mentions, links and images are not rendered.
- **edited** (`bool`) --
  Whether or not this comment has been edited.
- **hidden** (`bool`) --
  Whether or not this comment has been hidden.
A comment in a Discussion / Pull Request.

Subclass of [DiscussionEvent](/docs/huggingface_hub/v1.25.1/en/package_reference/community#huggingface_hub.DiscussionEvent).

- **id** (`str`) --
  The ID of the event. An hexadecimal string.
- **type** (`str`) --
  The type of the event.
- **created_at** (`datetime`) --
  A [`datetime`](https://docs.python.org/3/library/datetime.html?highlight=datetime#datetime.datetime)
  object holding the creation timestamp for the event.
- **author** (`str`) --
  The username of the Discussion / Pull Request author.
  Can be `"deleted"` if the user has been deleted since.
- **new_status** (`str`) --
  The status of the Discussion / Pull Request after the change.
  It can be one of:
  * `"open"`
  * `"closed"`
  * `"merged"` (only for Pull Requests )
A change of status in a Discussion / Pull Request.

Subclass of [DiscussionEvent](/docs/huggingface_hub/v1.25.1/en/package_reference/community#huggingface_hub.DiscussionEvent).

- **id** (`str`) --
  The ID of the event. An hexadecimal string.
- **type** (`str`) --
  The type of the event.
- **created_at** (`datetime`) --
  A [`datetime`](https://docs.python.org/3/library/datetime.html?highlight=datetime#datetime.datetime)
  object holding the creation timestamp for the event.
- **author** (`str`) --
  The username of the Discussion / Pull Request author.
  Can be `"deleted"` if the user has been deleted since.
- **summary** (`str`) --
  The summary of the commit.
- **oid** (`str`) --
  The OID / SHA of the commit, as a hexadecimal string.
A commit in a Pull Request.

Subclass of [DiscussionEvent](/docs/huggingface_hub/v1.25.1/en/package_reference/community#huggingface_hub.DiscussionEvent).

- **id** (`str`) --
  The ID of the event. An hexadecimal string.
- **type** (`str`) --
  The type of the event.
- **created_at** (`datetime`) --
  A [`datetime`](https://docs.python.org/3/library/datetime.html?highlight=datetime#datetime.datetime)
  object holding the creation timestamp for the event.
- **author** (`str`) --
  The username of the Discussion / Pull Request author.
  Can be `"deleted"` if the user has been deleted since.
- **old_title** (`str`) --
  The previous title for the Discussion / Pull Request.
- **new_title** (`str`) --
  The new title.
A rename event in a Discussion / Pull Request.

Subclass of [DiscussionEvent](/docs/huggingface_hub/v1.25.1/en/package_reference/community#huggingface_hub.DiscussionEvent).
