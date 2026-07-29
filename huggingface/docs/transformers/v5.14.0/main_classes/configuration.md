# Configuration

The base class [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) implements the common methods for loading/saving a configuration
either from a local file or directory, or from a pretrained model configuration provided by the library (downloaded
from HuggingFace's AWS S3 repository).

Each derived config class implements model specific attributes. Common attributes present in all config classes are:
`hidden_size`, `num_attention_heads`, and `num_hidden_layers`. Text models further implement:
`vocab_size`.

## PreTrainedConfig[[transformers.PreTrainedConfig]]

- **name_or_path** (`str`, *optional*, defaults to `""`) --
  Store the string that was passed to [PreTrainedModel.from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.PreTrainedModel.from_pretrained) as `pretrained_model_name_or_path`
  if the configuration was created with such a method.
- **output_hidden_states** (`bool`, *optional*, defaults to `False`) --
  Whether or not the model should return all hidden-states.
- **output_attentions** (`bool`, *optional*, defaults to `False`) --
  Whether or not the model should returns all attentions.
- **return_dict** (`bool`, *optional*, defaults to `True`) --
  Whether or not the model should return a [ModelOutput](/docs/transformers/v5.14.0/en/main_classes/output#transformers.utils.ModelOutput) instead of a plain tuple.
- **is_encoder_decoder** (`bool`, *optional*, defaults to `False`) --
  Whether the model is used as an encoder/decoder or not.
- **chunk_size_feed_forward** (`int`, *optional*, defaults to `0`) --
  The chunk size of all feed forward layers in the residual attention blocks. A chunk size of `0` means that
  the feed forward layer is not chunked. A chunk size of n means that the feed forward layer processes `n` <
  sequence_length embeddings at a time. For more information on feed forward chunking, see [How does Feed
  Forward Chunking work?](../glossary.html#feed-forward-chunking).
- **per_layer_config** (`dict[int | str, dict[str, Any]]`, *optional*) --
  A sparse mapping from layer indices to configuration attribute overrides. Each key is a layer index, and each value contains the attributes that differ from the global config for that layer.

Parameters for fine-tuning tasks

- **architectures** (`list[str]`, *optional*) --
  Model architectures that can be used with the model pretrained weights.
- **id2label** (`dict[int, str]`, *optional*) --
  A map from index (for instance prediction index, or target index) to label.
- **label2id** (`dict[str, int]`, *optional*) --
  A map from label to index for the model.
- **num_labels** (`int`, *optional*) --
  Number of labels to use in the last layer added to the model, typically for a classification task.
- **problem_type** (`str`, *optional*) --
  Problem type for `XxxForSequenceClassification` models. Can be one of `"regression"`,
  `"single_label_classification"` or `"multi_label_classification"`.

PyTorch specific parameters

- **dtype** (`str`, *optional*) --
  The `dtype` of the weights. This attribute can be used to initialize the model to a non-default `dtype`
  (which is normally `float32`) and thus allow for optimal storage allocation. For example, if the saved
  model is `float16`, ideally we want to load it back using the minimal amount of memory needed to load
  `float16` weights.

Base class for all configuration classes. Handles a few parameters common to all models' configurations as well as
methods for loading/downloading/saving configurations.

A configuration file can be loaded and saved to disk. Loading the configuration file and using this file to
initialize a model does **not** load the model weights. It only affects the model's configuration.

Class attributes (overridden by derived classes):

- **model_type** (`str`) -- An identifier for the model type, serialized into the JSON file, and used to recreate
  the correct object in [AutoConfig](/docs/transformers/v5.14.0/en/model_doc/auto#transformers.AutoConfig).
- **has_no_defaults_at_init** (`bool`) -- Whether the config class can be initialized without providing input arguments.
  Some configurations requires inputs to be defined at init and have no default values, usually these are composite configs,
  (but not necessarily) such as [EncoderDecoderConfig](/docs/transformers/v5.14.0/en/model_doc/encoder-decoder#transformers.EncoderDecoderConfig) or [~RagConfig](/docs/transformers/v5.14.0/en/model_doc/rag#transformers.RagConfig). They have to be initialized from
  two or more configs of type [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig).
- **keys_to_ignore_at_inference** (`list[str]`) -- A list of keys to ignore by default when looking at dictionary
  outputs of the model during inference.
- **attribute_map** (`dict[str, str]`) -- A dict that maps model specific attribute names to the standardized
  naming of attributes.
- **base_model_tp_plan** (`dict[str, Any]`) -- A dict that maps sub-modules FQNs of a base model to a tensor
  parallel plan applied to the sub-module when `model.tensor_parallel` is called.
- **base_model_fsdp_plan** (`dict[Any, str]`) -- A dict that maps sub-modules of a base model to an FSDP2
  sharding strategy (e.g. `"free_full_weight"` / `"keep_full_weight"`). Keys can be wildcard module paths
  (e.g. `"layers.*"`) or tuples of paths (grouped into a single `fully_shard` call).
- **base_model_pp_plan** (`dict[str, tuple[list[str]]]`) -- A dict that maps child-modules of a base model to a
  pipeline parallel plan that enables users to place the child-module on the appropriate device.

Common attributes (present in all subclasses):

- **vocab_size** (`int`) -- The number of tokens in the vocabulary, which is also the first dimension of the
  embeddings matrix (this attribute may be missing for models that don't have a text modality like ViT).
- **hidden_size** (`int`) -- The hidden size of the model.
- **num_attention_heads** (`int`) -- The number of attention heads used in the multi-head attention layers of the
  model.
- **num_hidden_layers** (`int`) -- The number of blocks in the model.

Setting parameters for sequence generation in the model config is deprecated. For backward compatibility, loading
some of them will still be possible, but attempting to overwrite them will throw an exception -- you should set
them in a [~transformers.GenerationConfig]. Check the documentation of [~transformers.GenerationConfig] for more
information about the individual parameters.

- **repo_id** (`str`) --
  The name of the repository you want to push your config to. It should contain your organization name
  when pushing to a given organization.
- **commit_message** (`str`, *optional*) --
  Message to commit while pushing. Will default to `"Upload config"`.
- **commit_description** (`str`, *optional*) --
  The description of the commit that will be created
- **private** (`bool`, *optional*) --
  Whether to make the repo private. If `None` (default), the repo will be public unless the organization's default is private. This value is ignored if the repo already exists.
- **token** (`bool` or `str`, *optional*) --
  The token to use as HTTP bearer authorization for remote files. If `True` (default), will use the token generated
  when running `hf auth login` (stored in `~/.huggingface`).
- **revision** (`str`, *optional*) --
  Branch to push the uploaded files to.
- **create_pr** (`bool`, *optional*, defaults to `False`) --
  Whether or not to create a PR with the uploaded files or directly commit.
- **max_shard_size** (`int` or `str`, *optional*, defaults to `"50GB"`) --
  Only applicable for models. The maximum size for a checkpoint before being sharded. Checkpoints shard
  will then be each of size lower than this size. If expressed as a string, needs to be digits followed
  by a unit (like `"5MB"`).
- **tags** (`list[str]`, *optional*) --
  List of tags to push on the Hub.

Upload the configuration file to the 🤗 Model Hub.

Examples:

```python
from transformers import AutoConfig

config = AutoConfig.from_pretrained("google-bert/bert-base-cased")

# Push the config to your namespace with the name "my-finetuned-bert".
config.push_to_hub("my-finetuned-bert")

# Push the config to an organization with the name "my-finetuned-bert".
config.push_to_hub("huggingface/my-finetuned-bert")
```

Checks whether the passed dictionary and its nested dicts have a *dtype* key and if it's not None,
converts torch.dtype to a string of just the type. For example, `torch.float32` get converted into *"float32"*
string, which can then be stored in the json format.

- **config_dict** (`dict[str, Any]`) --
  Dictionary that will be used to instantiate the configuration object. Such a dictionary can be
  retrieved from a pretrained checkpoint by leveraging the [get_config_dict()](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig.get_config_dict) method.
- **kwargs** (`dict[str, Any]`) --
  Additional parameters from which to initialize the configuration object.[PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)The configuration object instantiated from those parameters.

Instantiates a [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) from a Python dictionary of parameters.

- **json_file** (`str` or `os.PathLike`) --
  Path to the JSON file containing the parameters.[PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)The configuration object instantiated from that JSON file.

Instantiates a [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) from the path to a JSON file of parameters.

- **pretrained_model_name_or_path** (`str` or `os.PathLike`) --
  This can be either:

  - a string, the *model id* of a pretrained model configuration hosted inside a model repo on
    huggingface.co.
  - a path to a *directory* containing a configuration file saved using the
    [save_pretrained()](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig.save_pretrained) method, e.g., `./my_model_directory/`.
  - a path to a saved configuration JSON *file*, e.g., `./my_model_directory/configuration.json`.
- **cache_dir** (`str` or `os.PathLike`, *optional*) --
  Path to a directory in which a downloaded pretrained model configuration should be cached if the
  standard cache should not be used.
- **force_download** (`bool`, *optional*, defaults to `False`) --
  Whether or not to force to (re-)download the configuration files and override the cached versions if
  they exist.
- **proxies** (`dict[str, str]`, *optional*) --
  A dictionary of proxy servers to use by protocol or endpoint, e.g., `{'http': 'foo.bar:3128',
  'http://hostname': 'foo.bar:4012'}.` The proxies are used on each request.
- **token** (`str` or `bool`, *optional*) --
  The token to use as HTTP bearer authorization for remote files. If `True`, or not specified, will use
  the token generated when running `hf auth login` (stored in `~/.huggingface`).
- **revision** (`str`, *optional*, defaults to `"main"`) --
  The specific model version to use. It can be a branch name, a tag name, or a commit id, since we use a
  git-based system for storing models and other artifacts on huggingface.co, so `revision` can be any
  identifier allowed by git.

  

  To test a pull request you made on the Hub, you can pass `revision="refs/pr/<pr_number>"`.

  

- **return_unused_kwargs** (`bool`, *optional*, defaults to `False`) --
  If `False`, then this function returns just the final configuration object.

  If `True`, then this functions returns a `Tuple(config, unused_kwargs)` where *unused_kwargs* is a
  dictionary consisting of the key/value pairs whose keys are not configuration attributes: i.e., the
  part of `kwargs` which has not been used to update `config` and is otherwise ignored.
- **subfolder** (`str`, *optional*, defaults to `""`) --
  In case the relevant files are located inside a subfolder of the model repo on huggingface.co, you can
  specify the folder name here.
- **kwargs** (`dict[str, Any]`, *optional*) --
  The values in kwargs of any keys which are configuration attributes will be used to override the loaded
  values. Behavior concerning key/value pairs whose keys are *not* configuration attributes is controlled
  by the `return_unused_kwargs` keyword parameter.[PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig)The configuration object instantiated from this pretrained model.

Instantiate a [PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) (or a derived class) from a pretrained model configuration.

Examples:

```python
# We can't instantiate directly the base class *PreTrainedConfig* so let's show the examples on a
# derived class: BertConfig
config = BertConfig.from_pretrained(
    "google-bert/bert-base-uncased"
)  # Download configuration from huggingface.co and cache.
config = BertConfig.from_pretrained(
    "./test/saved_model/"
)  # E.g. config (or model) was saved using *save_pretrained('./test/saved_model/')*
config = BertConfig.from_pretrained("./test/saved_model/my_configuration.json")
config = BertConfig.from_pretrained("google-bert/bert-base-uncased", output_attentions=True, foo=False)
assert config.output_attentions == True
config, unused_kwargs = BertConfig.from_pretrained(
    "google-bert/bert-base-uncased", output_attentions=True, foo=False, return_unused_kwargs=True
)
assert config.output_attentions == True
assert unused_kwargs == {"foo": False}
```

- **pretrained_model_name_or_path** (`str` or `os.PathLike`) --
  The identifier of the pre-trained checkpoint from which we want the dictionary of parameters.`tuple[Dict, Dict]`The dictionary(ies) that will be used to instantiate the configuration object.

From a `pretrained_model_name_or_path`, resolve to a dictionary of parameters, to be used for instantiating a
[PreTrainedConfig](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig) using `from_dict`.

Returns the mtp text config to be used to create the MTP model. Since the MTP layers are created by instantiating
the same classes as the main model, we need to overwrite index-specific properties of the config such as `layer_types`
or `mtp_layer_types` to create the correct mtp layers and avoid indexing issues in the layers (because MTP layers restart
the indexing of layers at 0).

- **decoder** (`Optional[bool]`, *optional*) --
  If set to `True`, then only search for decoder config names.
- **encoder** (`Optional[bool]`, *optional*) --
  If set to `True`, then only search for encoder config names.

Returns the text config related to the text input (encoder) or text output (decoder) of the model. The
`decoder` and `encoder` input arguments can be used to specify which end of the model we are interested in,
which is useful on models that have both text input and output modalities.

There are three possible outcomes of using this method:
1. On most models, it returns the original config instance itself.
2. On newer (2024+) composite models, it returns the text section of the config, which is nested under a set
   of valid names.
3. On older (2023-) composite models, it discards decoder-only parameters when `encoder=True` and vice-versa.

Return whether the current config is custom code, i.e. either code loaded from the hub, or defined in any
user-specific module/session.

Return whether the current config is custom code, i.e. code loaded from the hub, or class that we just
registered via `register_for_auto_class`.

- **auto_class** (`str` or `type`, *optional*, defaults to `"AutoConfig"`) --
  The auto class to register this new configuration with.

Register this class with a given auto class. This should only be used for custom configurations as the ones in
the library are already mapped with `AutoConfig`.

- **save_directory** (`str` or `os.PathLike`) --
  Directory where the configuration JSON file will be saved (will be created if it does not exist).
- **push_to_hub** (`bool`, *optional*, defaults to `False`) --
  Whether or not to push your model to the Hugging Face model hub after saving it. You can specify the
  repository you want to push to with `repo_id` (will default to the name of `save_directory` in your
  namespace).
- **kwargs** (`dict[str, Any]`, *optional*) --
  Additional key word arguments passed along to the [push_to_hub()](/docs/transformers/v5.14.0/en/main_classes/model#transformers.utils.PushToHubMixin.push_to_hub) method.

Save a configuration object to the directory `save_directory`, so that it can be re-loaded using the
[from_pretrained()](/docs/transformers/v5.14.0/en/main_classes/configuration#transformers.PreTrainedConfig.from_pretrained) class method.

`dict[str, Any]`Dictionary of all the attributes that make up this configuration instance.

Serializes this instance to a Python dictionary.

dict[str, Any]Dictionary of all the attributes that make up this configuration instance.

Removes all attributes from the configuration that correspond to the default config attributes for
better readability, while always retaining the `config` attribute from the class. Serializes to a
Python dictionary.

- **json_file_path** (`str` or `os.PathLike`) --
  Path to the JSON file in which this configuration instance's parameters will be saved.
- **use_diff** (`bool`, *optional*, defaults to `True`) --
  If set to `True`, only the difference between the config instance and the default `PreTrainedConfig()`
  is serialized to JSON file.

Save this instance to a JSON file.

- **use_diff** (`bool`, *optional*, defaults to `True`) --
  If set to `True`, only the difference between the config instance and the default `PreTrainedConfig()`
  is serialized to JSON string.`str`String containing all the attributes that make up this configuration instance in JSON format.

Serializes this instance to a JSON string.

- **config_dict** (`dict[str, Any]`) -- Dictionary of attributes that should be updated for this class.

Updates attributes of this class with attributes from `config_dict`.

- **update_str** (`str`) -- String with attributes that should be updated for this class.

Updates attributes of this class with attributes from `update_str`.

The expected format is ints, floats and strings as is, and for booleans use `true` or `false`. For example:
"n_embd=10,resid_pdrop=0.2,scale_attn_weights=false,summary_type=cls_index"

The keys to change have to already exist in the config object.

Run class validators on the instance.

Part of `@strict`-powered validation. Validates the architecture of the config.

Check that `layer_types` is correctly defined.

Part of `@strict`-powered validation. Validates the contents of the special tokens.
