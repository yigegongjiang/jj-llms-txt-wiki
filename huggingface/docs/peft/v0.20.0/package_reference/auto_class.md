# AutoPeftModels

The `AutoPeftModel` classes loads the appropriate PEFT model for the task type by automatically inferring it from the configuration file. They are designed to quickly and easily load a PEFT model in a single line of code without having to worry about which exact model class you need or manually loading a [PeftConfig](/docs/peft/v0.20.0/en/package_reference/config#peft.PeftConfig).

## AutoPeftModel[[peft.AutoPeftModel]]

#### peft.AutoPeftModel[[peft.AutoPeftModel]]

```python
peft.AutoPeftModel(*args, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/auto.py#L197)

#### from_pretrained[[peft.AutoPeftModel.from_pretrained]]

```python
from_pretrained(pretrained_model_name_or_path, adapter_name: str = 'default', is_trainable: bool = False, config: Optional[PeftConfig] = None, revision: Optional[str] = None, import_allowlist: Optional[list[str]] = None, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/auto.py#L88)

**Parameters:**

import_allowlist (`list[str]`, *optional*, defaults to `{get_default_import_allowlist()}`) : AutoPeftModel will attempt to instantiate the base model that is configured in the adapter config. Since this operation needs to potentially import other packages, this allowlist is a safe-guard to prevent importing malicious packages. You may need to specify your package's import name here if it is not in the defaults.

A wrapper around all the preprocessing steps a user needs to perform in order to load a PEFT model. The kwargs
are passed along to `PeftConfig` that automatically takes care of filtering the kwargs of the Hub methods and
the config object init.

The parameters are equivalent to the ones of [PeftModel.from_pretrained()](/docs/peft/v0.20.0/en/package_reference/peft_model#peft.PeftModel.from_pretrained). Differences are documented below.

## AutoPeftModelForCausalLM[[peft.AutoPeftModelForCausalLM]]

#### peft.AutoPeftModelForCausalLM[[peft.AutoPeftModelForCausalLM]]

```python
peft.AutoPeftModelForCausalLM(*args, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/auto.py#L202)

## AutoPeftModelForSeq2SeqLM[[peft.AutoPeftModelForSeq2SeqLM]]

#### peft.AutoPeftModelForSeq2SeqLM[[peft.AutoPeftModelForSeq2SeqLM]]

```python
peft.AutoPeftModelForSeq2SeqLM(*args, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/auto.py#L207)

## AutoPeftModelForSequenceClassification[[peft.AutoPeftModelForSequenceClassification]]

#### peft.AutoPeftModelForSequenceClassification[[peft.AutoPeftModelForSequenceClassification]]

```python
peft.AutoPeftModelForSequenceClassification(*args, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/auto.py#L212)

## AutoPeftModelForTokenClassification[[peft.AutoPeftModelForTokenClassification]]

#### peft.AutoPeftModelForTokenClassification[[peft.AutoPeftModelForTokenClassification]]

```python
peft.AutoPeftModelForTokenClassification(*args, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/auto.py#L217)

## AutoPeftModelForQuestionAnswering[[peft.AutoPeftModelForQuestionAnswering]]

#### peft.AutoPeftModelForQuestionAnswering[[peft.AutoPeftModelForQuestionAnswering]]

```python
peft.AutoPeftModelForQuestionAnswering(*args, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/auto.py#L222)

## AutoPeftModelForFeatureExtraction[[peft.AutoPeftModelForFeatureExtraction]]

#### peft.AutoPeftModelForFeatureExtraction[[peft.AutoPeftModelForFeatureExtraction]]

```python
peft.AutoPeftModelForFeatureExtraction(*args, **kwargs)
```

[Source](https://github.com/huggingface/peft/blob/v0.20.0/src/peft/auto.py#L227)
