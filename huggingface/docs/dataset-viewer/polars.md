# Polars 

[Polars](https://pola-rs.github.io/polars-book/user-guide/) is a fast DataFrame library written in Rust with Arrow as its foundation.

💡 Learn more about how to get the dataset URLs in the [List Parquet files](parquet) guide.

Let's start by grabbing the URLs to the `train` split of the [`tasksource/blog_authorship_corpus`](https://huggingface.co/datasets/tasksource/blog_authorship_corpus) dataset from the dataset viewer API:

```py
import requests 

r = requests.get("https://datasets-server.huggingface.co/parquet?dataset=tasksource/blog_authorship_corpus")
j = r.json()
urls = [f['url'] for f in j['parquet_files'] if f['split'] == 'train']
urls
['https://huggingface.co/datasets/tasksource/blog_authorship_corpus/resolve/refs%2Fconvert%2Fparquet/default/train/0000.parquet', 'https://huggingface.co/datasets/tasksource/blog_authorship_corpus/resolve/refs%2Fconvert%2Fparquet/default/train/0001.parquet']
```

To read from a single Parquet file, use the [`read_parquet`](https://pola-rs.github.io/polars/py-polars/html/reference/api/polars.read_parquet.html) function to read it into a DataFrame and then execute your query:

```py
import polars as pl

df = (
    pl.read_parquet("https://huggingface.co/datasets/tasksource/blog_authorship_corpus/resolve/refs%2Fconvert%2Fparquet/default/train/0000.parquet")
    .group_by("sign")
    .agg(
        [
            pl.count(),
            pl.col("text").str.len_chars().mean().alias("avg_blog_length")
        ]
    )
    .sort("avg_blog_length", descending=True)
    .limit(5)
)
print(df)
shape: (5, 3)
┌───────────┬───────┬─────────────────┐
│ sign      ┆ count ┆ avg_blog_length │
│ ---       ┆ ---   ┆ ---             │
│ str       ┆ u32   ┆ f64             │
╞═══════════╪═══════╪═════════════════╡
│ Cancer    ┆ 38956 ┆ 1206.521203     │
│ Leo       ┆ 35487 ┆ 1180.067377     │
│ Aquarius  ┆ 32723 ┆ 1152.113682     │
│ Virgo     ┆ 36189 ┆ 1117.198209     │
│ Capricorn ┆ 31825 ┆ 1102.397361     │
└───────────┴───────┴─────────────────┘
```

To read multiple Parquet files - for example, if the dataset is sharded - you'll need to use the [`concat`](https://pola-rs.github.io/polars/py-polars/html/reference/api/polars.concat.html) function to concatenate the files into a single DataFrame: 

```py
import polars as pl

df = (
    pl.concat([pl.read_parquet(url) for url in urls])
    .group_by("sign")
    .agg(
        [
            pl.count(),
            pl.col("text").str.len_chars().mean().alias("avg_blog_length")
        ]
    )
    .sort("avg_blog_length", descending=True)
    .limit(5)
)
print(df)
shape: (5, 3)
┌──────────┬───────┬─────────────────┐
│ sign     ┆ count ┆ avg_blog_length │
│ ---      ┆ ---   ┆ ---             │
│ str      ┆ u32   ┆ f64             │
╞══════════╪═══════╪═════════════════╡
│ Aquarius ┆ 49687 ┆ 1191.417212     │
│ Leo      ┆ 53811 ┆ 1183.878222     │
│ Cancer   ┆ 65048 ┆ 1158.969161     │
│ Gemini   ┆ 51985 ┆ 1156.069308     │
│ Virgo    ┆ 60399 ┆ 1140.958443     │
└──────────┴───────┴─────────────────┘
```

## Lazy API

Polars offers a [lazy API](https://pola-rs.github.io/polars-book/user-guide/lazy/using/) that is more performant and memory-efficient for large Parquet files. The LazyFrame API keeps track of what you want to do, and it'll only execute the entire query when you're ready. This way, the lazy API doesn't load everything into RAM beforehand, and it allows you to work with datasets larger than your available RAM.

To lazily read a Parquet file, use the [`scan_parquet`](https://pola-rs.github.io/polars/py-polars/html/reference/api/polars.scan_parquet.html) function instead. Then, execute the entire query with the [`collect`](https://pola-rs.github.io/polars/py-polars/html/reference/lazyframe/api/polars.LazyFrame.collect.html) function:

```py
import polars as pl

q = (
    pl.scan_parquet("https://huggingface.co/datasets/tasksource/blog_authorship_corpus/resolve/refs%2Fconvert%2Fparquet/default/train/0000.parquet")
    .group_by("sign")
    .agg(
        [
            pl.count(),
            pl.col("text").str.len_chars().mean().alias("avg_blog_length")
        ]
    )
    .sort("avg_blog_length", descending=True)
    .limit(5)
)
df = q.collect()
```
