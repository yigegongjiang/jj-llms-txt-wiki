# Managing collections

Check out the [HfApi](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi) documentation page for the reference of methods to manage your Space on the Hub.

- Get collection content: [get_collection()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.get_collection)
- Create new collection: [create_collection()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.create_collection)
- Update a collection: [update_collection_metadata()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.update_collection_metadata)
- Update a collection's resource group: [update_collection_resource_group()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.update_collection_resource_group)
- Delete a collection: [delete_collection()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.delete_collection)
- Add an item to a collection: [add_collection_item()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.add_collection_item)
- Update an item in a collection: [update_collection_item()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.update_collection_item)
- Remove an item from a collection: [delete_collection_item()](/docs/huggingface_hub/v1.27.0/en/package_reference/hf_api#huggingface_hub.HfApi.delete_collection_item)

### Collection[[huggingface_hub.Collection]]

#### huggingface_hub.Collection[[huggingface_hub.Collection]]

```python
huggingface_hub.Collection(**kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/hf_api.py#L1491)

**Parameters:**

slug (`str`) : Slug of the collection. E.g. `"TheBloke/recent-models-64f9a55bb3115b4f513ec026"`.

title (`str`) : Title of the collection. E.g. `"Recent models"`.

owner (`str`) : Owner of the collection. E.g. `"TheBloke"`.

items (`list[CollectionItem]`) : List of items in the collection.

last_updated (`datetime`) : Date of the last update of the collection.

position (`int`) : Position of the collection in the list of collections of the owner.

private (`bool`) : Whether the collection is private or not.

theme (`str`) : Theme of the collection. E.g. `"green"`.

upvotes (`int`) : Number of upvotes of the collection.

description (`str`, *optional*) : Description of the collection, as plain text.

url (`str`) : (property) URL of the collection on the Hub.

Contains information about a Collection on the Hub.

### CollectionItem[[huggingface_hub.CollectionItem]]

#### huggingface_hub.CollectionItem[[huggingface_hub.CollectionItem]]

```python
huggingface_hub.CollectionItem(_id: str, id: str, type: CollectionItemType_T, position: int, note: dict | None = None, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.27.0/src/huggingface_hub/hf_api.py#L1443)

**Parameters:**

item_object_id (`str`) : Unique ID of the item in the collection.

item_id (`str`) : ID of the underlying object on the Hub. Can be either a repo_id, a paper id, a collection slug or a bucket id. e.g. `"jbilcke-hf/ai-comic-factory"`, `"2307.09288"`, `"celinah/cerebras-function-calling-682607169c35fbfa98b30b9a"`.

item_type (`str`) : Type of the underlying object. Can be one of `"model"`, `"dataset"`, `"space"`, `"paper"`, `"collection"` or `"bucket"`.

position (`int`) : Position of the item in the collection.

note (`str`, *optional*) : Note associated with the item, as plain text.

Contains information about an item of a Collection (model, dataset, Space, paper, collection or bucket).
