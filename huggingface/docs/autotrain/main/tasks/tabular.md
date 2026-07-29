# Tabular Classification / Regression

Using AutoTrain, you can train a model to classify or regress tabular data easily.
All you need to do is select from a list of models and upload your dataset.
Parameter tuning is done automatically.

## Models

The following models are available for tabular classification / regression.

- xgboost
- random_forest
- ridge
- logistic_regression
- svm
- extra_trees
- gradient_boosting
- adaboost
- decision_tree
- knn

## Data Format

```csv
id,category1,category2,feature1,target
1,A,X,0.3373961604172684,1
2,B,Z,0.6481718720511972,0
3,A,Y,0.36824153984054797,1
4,B,Z,0.9571551589530464,1
5,B,Z,0.14035078041264515,1
6,C,X,0.8700872583584364,1
7,A,Y,0.4736080452737105,0
8,C,Y,0.8009107519796442,1
9,A,Y,0.5204774795512048,0
10,A,Y,0.6788795301189603,0
.
.
.
```

## Columns

Your CSV dataset must have two columns: `id` and `target`.

## Parameters[[autotrain.trainers.tabular.params.TabularParams]]

#### autotrain.trainers.tabular.params.TabularParams[[autotrain.trainers.tabular.params.TabularParams]]

[Source](https://github.com/huggingface/autotrain-advanced/blob/main/src/autotrain/trainers/tabular/params.py#L8)

TabularParams is a configuration class for tabular data training parameters.

**Parameters:**

data_path (str) : Path to the dataset.

model (str) : Name of the model to use. Default is "xgboost".

username (Optional[str]) : Hugging Face Username.

seed (int) : Random seed for reproducibility. Default is 42.

train_split (str) : Name of the training data split. Default is "train".

valid_split (Optional[str]) : Name of the validation data split.

project_name (str) : Name of the output directory. Default is "project-name".

token (Optional[str]) : Hub Token for authentication.

push_to_hub (bool) : Whether to push the model to the hub. Default is False.

id_column (str) : Name of the ID column. Default is "id".

target_columns (Union[List[str], str]) : Target column(s) in the dataset. Default is ["target"].

categorical_columns (Optional[List[str]]) : List of categorical columns.

numerical_columns (Optional[List[str]]) : List of numerical columns.

task (str) : Type of task (e.g., "classification"). Default is "classification".

num_trials (int) : Number of trials for hyperparameter optimization. Default is 10.

time_limit (int) : Time limit for training in seconds. Default is 600.

categorical_imputer (Optional[str]) : Imputer strategy for categorical columns.

numerical_imputer (Optional[str]) : Imputer strategy for numerical columns.

numeric_scaler (Optional[str]) : Scaler strategy for numerical columns.
