# PEFT types

[PeftType](/docs/peft/v0.20.0/en/package_reference/peft_types#peft.PeftType) includes the supported adapters in PEFT, and [TaskType](/docs/peft/v0.20.0/en/package_reference/peft_types#peft.TaskType) includes PEFT-supported tasks.

## PeftType[[peft.PeftType]]

Enum class for the different types of adapters in PEFT.

Supported PEFT types:
- PROMPT_TUNING
- MULTITASK_PROMPT_TUNING
- P_TUNING
- PREFIX_TUNING
- LORA
- ADALORA
- BOFT
- ADAPTION_PROMPT
- IA3
- BEFT
- LOHA
- LOKR
- OFT
- XLORA
- POLY
- LN_TUNING
- VERA
- FROD
- FOURIERFT
- HRA
- BONE
- MISS
- RANDLORA
- SHIRA
- C3A
- ROAD
- WAVEFT
- OSF
- DELORA
- GRALORA
- ADAMSS
- DEFT

## TaskType[[peft.TaskType]]

Enum class for the different types of tasks supported by PEFT.

Overview of the supported task types:
- SEQ_CLS: Text classification.
- SEQ_2_SEQ_LM: Sequence-to-sequence language modeling.
- CAUSAL_LM: Causal language modeling.
- TOKEN_CLS: Token classification.
- QUESTION_ANS: Question answering.
- FEATURE_EXTRACTION: Feature extraction. Provides the hidden states which can be used as embeddings or features
  for downstream tasks.
