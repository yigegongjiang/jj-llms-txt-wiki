# Metrics

## Metrics
[//]: # (TODO: aenum.Enum raises error when generating docs: not supported by inspect.signature. See: https://github.com/ethanfurman/aenum/issues/44)
[//]: # (### Metrics)
[//]: # ([[autodoc]] metrics.metrics.Metrics)
### Metric[[lighteval.metrics.Metric]]
#### lighteval.metrics.Metric[[lighteval.metrics.Metric]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/utils/metric_utils.py#L33)

### CorpusLevelMetric[[lighteval.metrics.utils.metric_utils.CorpusLevelMetric]]
#### lighteval.metrics.utils.metric_utils.CorpusLevelMetric[[lighteval.metrics.utils.metric_utils.CorpusLevelMetric]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/utils/metric_utils.py#L117)

Metric computed over the whole corpora, with computations happening at the aggregation phase

### SampleLevelMetric[[lighteval.metrics.utils.metric_utils.SampleLevelMetric]]
#### lighteval.metrics.utils.metric_utils.SampleLevelMetric[[lighteval.metrics.utils.metric_utils.SampleLevelMetric]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/utils/metric_utils.py#L124)

Metric computed per sample, then aggregated over the corpus

### MetricGrouping[[lighteval.metrics.utils.metric_utils.MetricGrouping]]
#### lighteval.metrics.utils.metric_utils.MetricGrouping[[lighteval.metrics.utils.metric_utils.MetricGrouping]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/utils/metric_utils.py#L106)

Some metrics are more advantageous to compute together at once.
For example, if a costly preprocessing is the same for all metrics, it makes more sense to compute it once.

### CorpusLevelMetricGrouping[[lighteval.metrics.utils.metric_utils.CorpusLevelMetricGrouping]]
#### lighteval.metrics.utils.metric_utils.CorpusLevelMetricGrouping[[lighteval.metrics.utils.metric_utils.CorpusLevelMetricGrouping]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/utils/metric_utils.py#L131)

MetricGrouping computed over the whole corpora, with computations happening at the aggregation phase

### SampleLevelMetricGrouping[[lighteval.metrics.utils.metric_utils.SampleLevelMetricGrouping]]
#### lighteval.metrics.utils.metric_utils.SampleLevelMetricGrouping[[lighteval.metrics.utils.metric_utils.SampleLevelMetricGrouping]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/utils/metric_utils.py#L138)

MetricGrouping are computed per sample, then aggregated over the corpus

## Corpus Metrics
### CorpusLevelF1Score[[lighteval.metrics.metrics_corpus.CorpusLevelF1Score]]
#### lighteval.metrics.metrics_corpus.CorpusLevelF1Score[[lighteval.metrics.metrics_corpus.CorpusLevelF1Score]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_corpus.py#L81)

compute_corpuslighteval.metrics.metrics_corpus.CorpusLevelF1Score.compute_corpushttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_corpus.py#L96[{"name": "items", "val": ": list"}]
Computes the metric score over all the corpus generated items, by using the scikit learn implementation.

### CorpusLevelPerplexityMetric[[lighteval.metrics.metrics_corpus.CorpusLevelPerplexityMetric]]
#### lighteval.metrics.metrics_corpus.CorpusLevelPerplexityMetric[[lighteval.metrics.metrics_corpus.CorpusLevelPerplexityMetric]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_corpus.py#L164)

compute_corpuslighteval.metrics.metrics_corpus.CorpusLevelPerplexityMetric.compute_corpushttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_corpus.py#L182[{"name": "items", "val": ": list"}]
Computes the metric score over all the corpus generated items.

### CorpusLevelTranslationMetric[[lighteval.metrics.metrics_corpus.CorpusLevelTranslationMetric]]
#### lighteval.metrics.metrics_corpus.CorpusLevelTranslationMetric[[lighteval.metrics.metrics_corpus.CorpusLevelTranslationMetric]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_corpus.py#L116)

compute_corpuslighteval.metrics.metrics_corpus.CorpusLevelTranslationMetric.compute_corpushttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_corpus.py#L142[{"name": "items", "val": ": list"}]
Computes the metric score over all the corpus generated items, by using the sacrebleu implementation.

### MatthewsCorrCoef[[lighteval.metrics.metrics_corpus.MatthewsCorrCoef]]
#### lighteval.metrics.metrics_corpus.MatthewsCorrCoef[[lighteval.metrics.metrics_corpus.MatthewsCorrCoef]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_corpus.py#L66)

compute_corpuslighteval.metrics.metrics_corpus.MatthewsCorrCoef.compute_corpushttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_corpus.py#L67[{"name": "items", "val": ": list"}]- **items** (list[dict]) -- List of GenerativeCorpusMetricInput0floatScore
Computes the Matthews Correlation Coefficient, using scikit learn ([doc](https://scikit-learn.org/stable/modules/generated/sklearn.metrics.matthews_corrcoef.html)).

**Parameters:**

items (list[dict]) : List of GenerativeCorpusMetricInput

**Returns:**

`float`

Score

## Sample Metrics
### ExactMatches[[lighteval.metrics.metrics_sample.ExactMatches]]
#### lighteval.metrics.metrics_sample.ExactMatches[[lighteval.metrics.metrics_sample.ExactMatches]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L81)

computelighteval.metrics.metrics_sample.ExactMatches.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L118[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing gold references.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0floatAggregated score over the current sample's items.
Computes the metric over a list of golds and predictions for one single sample.

**Parameters:**

doc (Doc) : The document containing gold references.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float`

Aggregated score over the current sample's items.
#### compute_one_item[[lighteval.metrics.metrics_sample.ExactMatches.compute_one_item]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L137)

Compares two strings only.

**Parameters:**

gold (str) : One of the possible references

pred (str) : One of the possible predictions

**Returns:**

`float`

The exact match score. Will be 1 for a match, 0 otherwise.

### F1_score[[lighteval.metrics.metrics_sample.F1_score]]
#### lighteval.metrics.metrics_sample.F1_score[[lighteval.metrics.metrics_sample.F1_score]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L170)

computelighteval.metrics.metrics_sample.F1_score.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L197[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing gold references.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0floatAggregated score over the current sample's items.
Computes the metric over a list of golds and predictions for one single sample.

**Parameters:**

doc (Doc) : The document containing gold references.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float`

Aggregated score over the current sample's items.
#### compute_one_item[[lighteval.metrics.metrics_sample.F1_score.compute_one_item]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L217)

Compares two strings only.

**Parameters:**

gold (str) : One of the possible references

pred (str) : One of the possible predictions

**Returns:**

`float`

The f1 score over the bag of words, computed using nltk.

### LoglikelihoodAcc[[lighteval.metrics.metrics_sample.LoglikelihoodAcc]]
#### lighteval.metrics.metrics_sample.LoglikelihoodAcc[[lighteval.metrics.metrics_sample.LoglikelihoodAcc]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L243)

computelighteval.metrics.metrics_sample.LoglikelihoodAcc.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L254[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing choices and gold indices.
- **model_response** (ModelResponse) -- The model's response containing logprobs.
- ****kwargs** -- Additional keyword arguments.0intThe eval score: 1 if the best log-prob choice is in gold, 0 otherwise.
Computes the log likelihood accuracy: is the choice with the highest logprob in `choices_logprob` present
in the `gold_ixs`?

**Parameters:**

doc (Doc) : The document containing choices and gold indices.

model_response (ModelResponse) : The model's response containing logprobs.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`int`

The eval score: 1 if the best log-prob choice is in gold, 0 otherwise.

### NormalizedMultiChoiceProbability[[lighteval.metrics.metrics_sample.NormalizedMultiChoiceProbability]]
#### lighteval.metrics.metrics_sample.NormalizedMultiChoiceProbability[[lighteval.metrics.metrics_sample.NormalizedMultiChoiceProbability]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L297)

computelighteval.metrics.metrics_sample.NormalizedMultiChoiceProbability.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L313[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing choices and gold indices.
- **model_response** (ModelResponse) -- The model's response containing logprobs.
- ****kwargs** -- Additional keyword arguments.0floatThe probability of the best log-prob choice being a gold choice.
Computes the log likelihood probability: chance of choosing the best choice.

**Parameters:**

doc (Doc) : The document containing choices and gold indices.

model_response (ModelResponse) : The model's response containing logprobs.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float`

The probability of the best log-prob choice being a gold choice.

### Probability[[lighteval.metrics.metrics_sample.Probability]]
#### lighteval.metrics.metrics_sample.Probability[[lighteval.metrics.metrics_sample.Probability]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L357)

computelighteval.metrics.metrics_sample.Probability.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L373[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing choices and gold indices.
- **model_response** (ModelResponse) -- The model's response containing logprobs.
- ****kwargs** -- Additional keyword arguments.0floatThe probability of the best log-prob choice being a gold choice.
Computes the log likelihood probability: chance of choosing the best choice.

**Parameters:**

doc (Doc) : The document containing choices and gold indices.

model_response (ModelResponse) : The model's response containing logprobs.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float`

The probability of the best log-prob choice being a gold choice.

### Recall[[lighteval.metrics.metrics_sample.Recall]]
#### lighteval.metrics.metrics_sample.Recall[[lighteval.metrics.metrics_sample.Recall]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L408)

computelighteval.metrics.metrics_sample.Recall.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L418[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing choices and gold indices.
- **model_response** (ModelResponse) -- The model's response containing logprobs.
- ****kwargs** -- Additional keyword arguments.0intScore: 1 if one of the top level predicted choices was correct, 0 otherwise.
Computes the recall at the requested depth level: looks at the `n` best predicted choices (with the
highest log probabilities) and see if there is an actual gold among them.

**Parameters:**

doc (Doc) : The document containing choices and gold indices.

model_response (ModelResponse) : The model's response containing logprobs.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`int`

Score: 1 if one of the top level predicted choices was correct, 0 otherwise.

### MRR[[lighteval.metrics.metrics_sample.MRR]]
#### lighteval.metrics.metrics_sample.MRR[[lighteval.metrics.metrics_sample.MRR]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L438)

computelighteval.metrics.metrics_sample.MRR.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L447[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **model_response** (ModelResponse) -- The model's response containing logprobs.
- **doc** (Doc) -- The document containing choices and gold indices.
- ****kwargs** -- Additional keyword arguments.0floatMRR score.
Mean reciprocal rank. Measures the quality of a ranking of choices (ordered by correctness).

**Parameters:**

model_response (ModelResponse) : The model's response containing logprobs.

doc (Doc) : The document containing choices and gold indices.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float`

MRR score.

### ROUGE[[lighteval.metrics.metrics_sample.ROUGE]]
#### lighteval.metrics.metrics_sample.ROUGE[[lighteval.metrics.metrics_sample.ROUGE]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L486)

computelighteval.metrics.metrics_sample.ROUGE.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L533[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing gold references.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0float or dictAggregated score over the current sample's items.
If several rouge functions have been selected, returns a dict which maps name and scores.
Computes the metric(s) over a list of golds and predictions for one single sample.

**Parameters:**

doc (Doc) : The document containing gold references.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float or dict`

Aggregated score over the current sample's items.
If several rouge functions have been selected, returns a dict which maps name and scores.

### BertScore[[lighteval.metrics.metrics_sample.BertScore]]
#### lighteval.metrics.metrics_sample.BertScore[[lighteval.metrics.metrics_sample.BertScore]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L598)

computelighteval.metrics.metrics_sample.BertScore.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L628[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing gold references.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0dictScores over the current sample's items.
Computes the prediction, recall and f1 score using the bert scorer.

**Parameters:**

doc (Doc) : The document containing gold references.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`dict`

Scores over the current sample's items.

### Extractiveness[[lighteval.metrics.metrics_sample.Extractiveness]]
#### lighteval.metrics.metrics_sample.Extractiveness[[lighteval.metrics.metrics_sample.Extractiveness]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L661)

computelighteval.metrics.metrics_sample.Extractiveness.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L685[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing input text.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0dict[str, float]The extractiveness scores.
Compute the extractiveness of the predictions.

This method calculates coverage, density, and compression scores for a single
prediction against the input text.

**Parameters:**

doc (Doc) : The document containing input text.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`dict[str, float]`

The extractiveness scores.

### Faithfulness[[lighteval.metrics.metrics_sample.Faithfulness]]
#### lighteval.metrics.metrics_sample.Faithfulness[[lighteval.metrics.metrics_sample.Faithfulness]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L717)

computelighteval.metrics.metrics_sample.Faithfulness.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L738[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing input text.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0dict[str, float]The faithfulness scores.
Compute the faithfulness of the predictions.

The SummaCZS (Summary Content Zero-Shot) model is used with configurable granularity and model variation.

**Parameters:**

doc (Doc) : The document containing input text.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`dict[str, float]`

The faithfulness scores.

### BLEURT[[lighteval.metrics.metrics_sample.BLEURT]]
#### lighteval.metrics.metrics_sample.BLEURT[[lighteval.metrics.metrics_sample.BLEURT]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L765)

computelighteval.metrics.metrics_sample.BLEURT.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L786[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing gold references.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0floatScore over the current sample's items.
Uses the stored BLEURT scorer to compute the score on the current sample.

**Parameters:**

doc (Doc) : The document containing gold references.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float`

Score over the current sample's items.

### BLEU[[lighteval.metrics.metrics_sample.BLEU]]
#### lighteval.metrics.metrics_sample.BLEU[[lighteval.metrics.metrics_sample.BLEU]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L805)

computelighteval.metrics.metrics_sample.BLEU.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L815[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing gold references.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0floatScore over the current sample's items.
Computes the sentence level BLEU between the golds and each prediction, then takes the average.

**Parameters:**

doc (Doc) : The document containing gold references.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float`

Score over the current sample's items.

### StringDistance[[lighteval.metrics.metrics_sample.StringDistance]]
#### lighteval.metrics.metrics_sample.StringDistance[[lighteval.metrics.metrics_sample.StringDistance]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L847)

computelighteval.metrics.metrics_sample.StringDistance.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L869[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing gold references.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0dictThe different scores computed
Computes all the requested metrics on the golds and prediction.

**Parameters:**

doc (Doc) : The document containing gold references.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`dict`

The different scores computed
#### edit_similarity[[lighteval.metrics.metrics_sample.StringDistance.edit_similarity]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L927)

Compute the edit similarity between two lists of strings.

Edit similarity is also used in the paper
Lee, Katherine, et al.
"Deduplicating training data makes language models better."
arXiv preprint arXiv:2107.06499 (2021).

**Returns:**

`float`

Edit similarity score between 0 and 1
#### longest_common_prefix_length[[lighteval.metrics.metrics_sample.StringDistance.longest_common_prefix_length]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L920)

Compute the length of the longest common prefix.

### Metrics allowing sampling
#### PassAtK[[lighteval.metrics.metrics_sample.PassAtK]]
#### lighteval.metrics.metrics_sample.PassAtK[[lighteval.metrics.metrics_sample.PassAtK]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1264)

computelighteval.metrics.metrics_sample.PassAtK.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1278[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing gold references.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0floatAggregated score over the current sample's items.
Computes the metric over a list of golds and predictions for one single item with possibly many samples.
It applies normalisation (if needed) to model prediction and gold, computes their per prediction score,
then aggregates the scores over the samples using a pass@k.

**Parameters:**

doc (Doc) : The document containing gold references.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float`

Aggregated score over the current sample's items.
#### pass_at_k[[lighteval.metrics.metrics_sample.PassAtK.pass_at_k]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1319)

Algo from https://arxiv.org/pdf/2107.03374

#### MajAtN[[lighteval.metrics.metrics_sample.MajAtN]]
#### lighteval.metrics.metrics_sample.MajAtN[[lighteval.metrics.metrics_sample.MajAtN]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1212)

computelighteval.metrics.metrics_sample.MajAtN.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1225[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **doc** (Doc) -- The document containing gold references.
- **model_response** (ModelResponse) -- The model's response containing predictions.
- ****kwargs** -- Additional keyword arguments.0floatAggregated score over the current sample's items.
Computes the metric over a list of golds and predictions for one single sample.
It applies normalisation (if needed) to model prediction and gold, and takes the most frequent answer of all the available ones,
then compares it to the gold.

**Parameters:**

doc (Doc) : The document containing gold references.

model_response (ModelResponse) : The model's response containing predictions.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float`

Aggregated score over the current sample's items.

#### AvgAtN[[lighteval.metrics.metrics_sample.AvgAtN]]
#### lighteval.metrics.metrics_sample.AvgAtN[[lighteval.metrics.metrics_sample.AvgAtN]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1176)

computelighteval.metrics.metrics_sample.AvgAtN.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1188[{"name": "doc", "val": ": Doc"}, {"name": "model_response", "val": ": ModelResponse"}, {"name": "**kwargs", "val": ""}]- **model_response** (ModelResponse) -- The model's response containing predictions.
- **doc** (Doc) -- The document containing gold references.
- ****kwargs** -- Additional keyword arguments.0floatAggregated score over the current sample's items.
Computes the metric over a list of golds and predictions for one single sample.
It applies normalisation (if needed) to model prediction and gold, and takes the most frequent answer of all the available ones,
then compares it to the gold.

**Parameters:**

model_response (ModelResponse) : The model's response containing predictions.

doc (Doc) : The document containing gold references.

- ****kwargs** : Additional keyword arguments.

**Returns:**

`float`

Aggregated score over the current sample's items.

## LLM-as-a-Judge
### JudgeLM[[lighteval.metrics.utils.llm_as_judge.JudgeLM]]
#### lighteval.metrics.utils.llm_as_judge.JudgeLM[[lighteval.metrics.utils.llm_as_judge.JudgeLM]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/utils/llm_as_judge.py#L67)

A class representing a judge for evaluating answers using either the chosen backend.

Methods:
evaluate_answer: Evaluates an answer using the OpenAI API or Transformers library.
__lazy_load_client: Lazy loads the OpenAI client or Transformers pipeline.
__call_api: Calls the API to get the judge's response.
__call_transformers: Calls the Transformers pipeline to get the judge's response.
__call_vllm: Calls the VLLM pipeline to get the judge's response.

dict_of_lists_to_list_of_dictslighteval.metrics.utils.llm_as_judge.JudgeLM.dict_of_lists_to_list_of_dictshttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/utils/llm_as_judge.py#L204[{"name": "dict_of_lists", "val": ""}]- **dict_of_lists** -- A dictionary where each value is a list.
  All lists are expected to have the same length.0A list of dictionaries.
Transform a dictionary of lists into a list of dictionaries.

Each dictionary in the output list will contain one element from each list in the input dictionary,
with the same keys as the input dictionary.

Example:
>>> dict_of_lists_to_list_of_dicts({'k': [1, 2, 3], 'k2': ['a', 'b', 'c']})
[{'k': 1, 'k2': 'a'}, {'k': 2, 'k2': 'b'}, {'k': 3, 'k2': 'c'}]

**Parameters:**

model (str) : The name of the model.

templates (Callable) : A function taking into account the question, options, answer, and gold and returning the judge prompt.

process_judge_response (Callable) : A function for processing the judge's response.

judge_backend (Literal["litellm", "openai", "transformers", "tgi", "vllm", "inference-providers"]) : The backend for the judge.

url (str | None) : The URL for the OpenAI API.

api_key (str | None) : The API key for the OpenAI API (either OpenAI or HF key).

max_tokens (int) : The maximum number of tokens to generate. Defaults to 512.

response_format (BaseModel | None) : The format of the response from the API, used for the OpenAI and TGI backend.

hf_provider (Literal["black-forest-labs", "cerebras", "cohere", "fal-ai", "fireworks-ai", : "inference-providers", "hyperbolic", "nebius", "novita", "openai", "replicate", "sambanova", "together"] | None): The HuggingFace provider when using the inference-providers backend.

backend_options (dict | None) : Options for the backend. Currently only supported for litellm.

**Returns:**

A list of dictionaries.
#### evaluate_answer[[lighteval.metrics.utils.llm_as_judge.JudgeLM.evaluate_answer]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/utils/llm_as_judge.py#L272)

Evaluates an answer using either Transformers or OpenAI API.

**Parameters:**

question (str) : The prompt asked to the evaluated model.

answer (str) : Answer given by the evaluated model.

options (list[str] | None) : Optional list of answer options.

gold (str | None) : Optional reference answer.

**Returns:**

A tuple containing the score, prompts, and judgment.

### JudgeLLM[[lighteval.metrics.metrics_sample.JudgeLLM]]
#### lighteval.metrics.metrics_sample.JudgeLLM[[lighteval.metrics.metrics_sample.JudgeLLM]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L942)

### JudgeLLMMTBench[[lighteval.metrics.metrics_sample.JudgeLLMMTBench]]
#### lighteval.metrics.metrics_sample.JudgeLLMMTBench[[lighteval.metrics.metrics_sample.JudgeLLMMTBench]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1047)

computelighteval.metrics.metrics_sample.JudgeLLMMTBench.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1048[{"name": "model_response", "val": ": list"}, {"name": "doc", "val": ": list"}, {"name": "**kwargs", "val": ""}]
Compute the score of a generative task using a llm as a judge.
The generative task can be multiturn with 2 turns max, in that case, we
return scores for turn 1 and 2. Also returns user_prompt and judgement
which are ignored later by the aggregator.

### JudgeLLMMixEval[[lighteval.metrics.metrics_sample.JudgeLLMMixEval]]
#### lighteval.metrics.metrics_sample.JudgeLLMMixEval[[lighteval.metrics.metrics_sample.JudgeLLMMixEval]]

[Source](https://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1082)

computelighteval.metrics.metrics_sample.JudgeLLMMixEval.computehttps://github.com/huggingface/lighteval/blob/v0.13.0/src/lighteval/metrics/metrics_sample.py#L1083[{"name": "responses", "val": ": list"}, {"name": "docs", "val": ": list"}, {"name": "**kwargs", "val": ""}]
Compute the score of a generative task using a llm as a judge.
The generative task can be multiturn with 2 turns max, in that case, we
return scores for turn 1 and 2. Also returns user_prompt and judgement
which are ignored later by the aggregator.
