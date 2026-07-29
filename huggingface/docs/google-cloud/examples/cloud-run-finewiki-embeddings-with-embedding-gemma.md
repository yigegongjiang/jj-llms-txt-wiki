# 100,000 FineWiki embeddings with EmbeddingGemma on Cloud Run

Written by Alvaro BartolomeLast updated 2026-04-07

This example shows how to generate 100,000 embeddings for [FineWiki](https://huggingface.co/datasets/HuggingFaceFW/finewiki) (an updated and better extracted version of the [`wikimedia/Wikipedia`](https://huggingface.co/datasets/wikimedia/wikipedia) up to August of 2025) with [Google EmbeddingGemma 300m](https://huggingface.co/google/embeddinggemma-300m) running with [Text Embeddings Inference](https://github.com/huggingface/text-embeddings-inference) on Cloud Run.

![100,000 FineWiki embeddings with EmbeddingGemma on Cloud Run](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/google-cloud/examples/cloud-run/finewiki-embeddings-with-embedding-gemma/finewiki-embeddings-cloud-run.png)

## Setup and Requirements

- You have a Google Cloud account and you are logged into the Cloud Console.
- You have the necessary permissions on Google Cloud to create a service account, assign permissions to it, create secrets, and create / delete services on Cloud Run.
- You have a Hugging Face Hub account with read-access to [`google/embeddinggemma-300m`](https://huggingface.co/google/embeddinggemma-300m).
- You have Python 3.10+ with `pip` installed locally (recommended via `uv`).

![Google EmbeddingGemma on the Hugging Face Hub](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/google-cloud/examples/cloud-run/finewiki-embeddings-with-embedding-gemma/embedding-gemma-hugging-face.png)

As [`google/embeddinggemma-300m`](https://huggingface.co/google/embeddinggemma-300m) is a gated model, you will need to [accept the gating (i.e., Google’s usage license) on the Hugging Face Hub](https://huggingface.co/google/embeddinggemma-300m), and [generate a `HF_TOKEN` with read access to the model](https://huggingface.co/settings/tokens/new?tokenType=read), so that the container can download the model weights. If you were to use the ONNX export at [`onnx-community/embeddinggemma-300m-ONNX`](https://huggingface.co/onnx-community/embeddinggemma-300m-ONNX), supported with Text Embeddings Inference on CPU, you wouldn't need to provide the token given that this variant is not under any form of gating.

## Enable APIs and Set environment variables

### Enable APIs

Before you start running the `gcloud` commands, you need to make sure that both Cloud Run and Secret Manager APIs are enabled on your Google Cloud account.

```python
!gcloud services enable run.googleapis.com secretmanager.googleapis.com
```

If you want to deploy the model on [NVIDIA RTX PRO 6000](https://cloud.google.com/blog/products/serverless/cloud-run-supports-nvidia-rtx-6000-pro-gpus-for-ai-workloads), you might need to upgrade your `gcloud` version. More information on [GPU Support and Availability on Cloud Run](https://docs.cloud.google.com/run/docs/configuring/services/gpu).

### Set environment variables

Then, you can set the following environment variables to be used through the example:

```python
%env PROJECT_ID 
%env LOCATION us-central1
%env CONTAINER_URI ghcr.io/huggingface/text-embeddings-inference:89-1.9
%env SERVICE_NAME google--embeddinggemma-300m
%env SERVICE_ACCOUNT cloud-run-sa
```

## Create `HF_TOKEN` secret

Then you need to create a new secret with the value of the `HF_TOKEN` with read access for the gated model, to prevent from exposing it as an environment variable.

Make sure to replace the `` in the command below with your Hugging Face Hub token.

```python
!printf  | gcloud secrets create HF_TOKEN --data-file=-
```

If the `HF_TOKEN` already exists, then you can just create a new version with its new value.

```python
!printf  | gcloud secrets versions add HF_TOKEN --data-file=-
```

## Create a service account

Before deploying the Cloud Run service, it's convenient to create a Service Account that will be used to create the service with access to the secret manager, as well as to invoke the service afterwards with a dedicated identity token.

```python
!gcloud iam service-accounts create $SERVICE_ACCOUNT --display-name="Cloud Run Service Account for EmbeddingGemma" --quiet
```

And grant it the `secretmanager.secretAccessor` role to be able to read the `HF_TOKEN` secret.

```python
!gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member serviceAccount:$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com \
    --role=roles/secretmanager.secretAccessor
```

## Deploy the service

Before deploying the service, we'll adjust a few Text Embeddings Inference arguments to improve performance during embedding generation. Specifically, we'll reduce concurrency and batch sizes to avoid timeouts with large FineWiki entries.

- `--max-client-batch-size 16`: process up to 16 sequences within a single batch i.e., per forward pass.
- `--max-batch-tokens 32768`: limit each batch to 16 sequences of up to 2048 tokens (the model’s maximum length defined in `config.json` of `google/embeddinggemma-300m`). Longer sequences are truncated to 2048.
- `--max-concurrent-requests 16`: lower from the default 512 to prevent excessive queuing and timeouts.

For additional configuration details, see the [Text Embeddings Router CLI documentation](https://huggingface.co/docs/text-embeddings-inference/en/cli_arguments).

Finally, mention that we will be deploying on a single NVIDIA L4 GPU with 24GB VRAM, 8 vCPUs and 32GB of RAM; but soon you will also be able to [deploy on larger GPU instances on Cloud Run e.g. NVIDIA RTX PRO 6000 Blackwell GPU with 96 GB VRAM, still on preview](https://docs.cloud.google.com/run/docs/configuring/services/gpu#gpu-type) and only supported with Text Embeddings Inference v1.9.1 or later, which as of Feb 2025 is still unreleased on Google Cloud as Hugging Face DLC.

```python
!gcloud run deploy $SERVICE_NAME \
    --image=$CONTAINER_URI \
    --args="--model-id=google/embeddinggemma-300m,--max-client-batch-size=16,--max-batch-tokens=32768,--max-concurrent-requests=16" \
    --set-secrets="HF_TOKEN=HF_TOKEN:latest" \
    --port=8080 \
    --service-account $SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com \
    --cpu=8 \
    --memory=32Gi \
    --no-cpu-throttling \
    --no-gpu-zonal-redundancy \
    --startup-probe tcpSocket.port=8080,initialDelaySeconds=240,failureThreshold=1,timeoutSeconds=240,periodSeconds=240 \
    --gpu=1 \
    --gpu-type=nvidia-l4 \
    --max-instances=3 \
    --concurrency=16 \
    --region=$LOCATION \
    --timeout=500 \
    --no-allow-unauthenticated
```

If you were to deploy on NVIDIA RTX PRO 6000, then you'd need to run the following command instead (and you could as well update the `--max-client-batch-size`, `--max-batch-tokens`, and `--max-concurrent-requests` as there will be 96GB VRAM instead of the 24GB on the NVIDA L4):

```python
!gcloud beta run deploy $SERVICE_NAME \
    --image=ghcr.io/huggingface/text-embeddings-inference:120-1.9 \
    --args="--model-id=google/embeddinggemma-300m,--max-client-batch-size=16,--max-batch-tokens=32768,--max-concurrent-requests=16" \
    --set-secrets="HF_TOKEN=HF_TOKEN:latest" \
    --port=8080 \
    --service-account $SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com \
    --cpu 20 \
    --memory 80Gi \
    --no-cpu-throttling \
    --no-gpu-zonal-redundancy \
    --startup-probe tcpSocket.port=8080,initialDelaySeconds=240,failureThreshold=1,timeoutSeconds=240,periodSeconds=240 \
    --gpu=1 \
    --gpu-type nvidia-rtx-pro-6000 \
    --max-instances=3 \
    --concurrency=16 \
    --region=$LOCATION \
    --timeout=500 \
    --no-allow-unauthenticated
```

## Send requests to the service

Once deployed, you can send requests to it via the service URL and the identity token impersonating the service account; but also [via a proxy to localhost already authenticated](https://docs.cloud.google.com/sdk/gcloud/reference/run/services/proxy).

To send requests via the URL with the service account token, you need to add the `run.invoker` role to the service account as:

```python
!gcloud run services add-iam-policy-binding $SERVICE_NAME \
    --member="serviceAccount:$SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com" \
    --role="roles/run.invoker" \
    --region=$LOCATION
```

Then you can send HTTP requests via cURL (or your preferred alternative) to the [OpenAI-compatible Embeddings API endpoint](https://developers.openai.com/api/reference/resources/embeddings/methods/create) on Text Embeddings Inference as:

```python
!gcloud run services describe $SERVICE_NAME --region $LOCATION --format 'value(status.url)'
```

```python
!gcloud auth print-identity-token --impersonate-service-account $SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com
```

```python
!curl $(gcloud run services describe $SERVICE_NAME --region $LOCATION --format 'value(status.url)')/v1/embeddings \
    -H "Authorization: Bearer $(gcloud auth print-identity-token --impersonate-service-account $SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com)" \
    -H "Content-Type: application/json" \
    -d '{"model":"google/embeddinggemma-300m","input":"task: search result | query: Which planet is known as the Red Planet?"}'
```

Or rather to the [Text Embeddings Inference API](https://huggingface.github.io/text-embeddings-inference/) directly as:

```python
!curl $(gcloud run services describe $SERVICE_NAME --region $LOCATION --format 'value(status.url)')/embed \
    -H "Authorization: Bearer $(gcloud auth print-identity-token --impersonate-service-account $SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com)" \
    -H "Content-Type: application/json" \
    -d '{"inputs":"Which planet is known as the Red Planet?","prompt_name":"query"}' \
    -sS | jq
```

## Compute 100,000 FineWiki embeddings

[`HuggingFaceFW/finewiki`](https://huggingface.co/datasets/HuggingFaceFW/finewiki) won't be used since it contains 325 language subsets, and in this example we'll be focused on the English (`en`) subset only, which contains 6.61 million rows that we'll filter down to 100,000 for the example, but you could use either another subset or another portion out of the English subset.

![FineWiki: The finest knowledge from the Free Encyclopedia](https://cdn-uploads.huggingface.co/production/uploads/62596f9e1c0a084224b93e00/nmYXgDCjHwhQpq5NaB4Uv.png)

To filter down the rows, we didn't kept 100,000 random rows, but rather tokenized all of those with the EmbeddingGemma tokenizer and given that the maximum input length is 2048, we only considered rows where the length of the `input_ids` was lower than 2048, to prevent from truncating larger sequences if those were exceeding 2048. The outcome of this filtering has been uploaded to the Hugging Face Hub at [`google-cloud-partnership/finewiki-en-100k`](https://huggingface.co/google-cloud-partnership/finewiki-en-100k) containing only the columns `text` and `id` to also make the dataset a bit lighter since we only care about those columns in this example.

```python
 %pip install datasets --upgrade --quiet
```

```python
from datasets import load_dataset

dataset = load_dataset("google-cloud-partnership/finewiki-en-100k", split="train")
dataset
```

Once the dataset is downloaded, then you can already install the required Python dependencies:

- `openai` for the OpenAI Python SDK with the Embeddings API client to send the requests.
- `pyarrow` to iteratively add new rows to a Parquet file (to be later loaded as a Hugging Face dataset).

```python
%pip install pyarrow openai --upgrade --quiet
```

And then run the following Python snippet which sends requests to the Cloud Run service URL to the OpenAI-compatible endpoint `/v1/embeddings` on Text Embeddings Inference, with the identity token for the service account (which needs to be rotated every hour as it expires).

```python
import asyncio
import os
import subprocess

import pyarrow as pa
import pyarrow.parquet as pq
from openai import AsyncOpenAI

BATCH_SIZE = 16
MAX_CONCURRENT_REQUESTS = 16
PARQUET_FILE = "embeddings.parquet"

BASE_URL = (
    subprocess.check_output(
        [
            "gcloud",
            "run",
            "services",
            "describe",
            os.getenv("SERVICE_NAME"),
            "--region",
            os.getenv("LOCATION"),
            "--format",
            "value(status.url)",
        ]
    )
    .decode("utf-8")
    .strip()
)

API_KEY = (
    subprocess.check_output(
        [
            "gcloud",
            "auth",
            "print-identity-token",
            "--impersonate-service-account",
            f"{os.getenv('SERVICE_ACCOUNT')}@{os.getenv('PROJECT_ID')}.iam.gserviceaccount.com",
        ]
    )
    .decode("utf-8")
    .strip()
)

client = AsyncOpenAI(base_url=f"{BASE_URL}/v1", api_key=API_KEY)

async def fetch_embeddings(batch: list, batch_id: int) -> tuple[dict | None, int]:
    try:
        return await client.embeddings.create(input=batch, model="google/embeddinggemma-300m"), batch_id
    except Exception as e:
        print(f"Error for batch {batch_id}: {e}")
        return None, batch_id

async def generate_embeddings() -> None:
    semaphore = asyncio.Semaphore(MAX_CONCURRENT_REQUESTS)
    writer = None
    schema = pa.schema([("id", pa.string()), ("text", pa.string()), ("embedding", pa.list_(pa.float32()))])

    async def bounded_fetch(batch, batch_id):
        async with semaphore:
            return await fetch_embeddings(batch, batch_id)

    tasks = []
    batch_metadata = {}
    total_processed = 0

    for idx, batch in enumerate(dataset.batch(batch_size=BATCH_SIZE)):
        batch_metadata[idx] = {"ids": batch["id"], "texts": batch["text"]}
        tasks.append(bounded_fetch(batch["text"], idx))

        if len(tasks) >= MAX_CONCURRENT_REQUESTS:
            writer = await process_and_write(tasks, batch_metadata, writer, schema)
            total_processed += BATCH_SIZE * len(tasks)
            tasks.clear()
            batch_metadata.clear()
            print(f"Processed {total_processed} rows")

            # NOTE: To prevent the identity token from expiring, we generate a new one after 16 * 16 rows
            # are processed i.e., after 16 requests with a batch size of 16 are processed.
            client.api_key = (
                subprocess.check_output(
                    [
                        "gcloud",
                        "auth",
                        "print-identity-token",
                        "--impersonate-service-account",
                        f"{os.getenv('SERVICE_ACCOUNT')}@{os.getenv('PROJECT_ID')}.iam.gserviceaccount.com",
                    ]
                )
                .decode("utf-8")
                .strip()
            )

    if tasks:
        writer = await process_and_write(tasks, batch_metadata, writer, schema)
        total_processed += BATCH_SIZE * len(tasks)
        print(f"Processed {total_processed} rows")

    if writer:
        writer.close()
        print(f"Embeddings saved to {PARQUET_FILE}")

async def process_and_write(tasks, batch_metadata, writer, schema) -> None:
    responses = await asyncio.gather(*tasks, return_exceptions=True)

    rows = []
    for result in responses:
        if isinstance(result, Exception) or result[0] is None:
            continue

        response, batch_idx = result
        metadata = batch_metadata.get(batch_idx)
        if not metadata:
            continue

        for id_, text, data in zip(metadata["ids"], metadata["texts"], response.data):
            rows.append({"id": id_, "text": text, "embedding": data.embedding})

    print(f"Writing {len(rows)} rows into {PARQUET_FILE}")
    if rows:
        table = pa.Table.from_pylist(rows, schema=schema)
        if writer is None:
            writer = pq.ParquetWriter(PARQUET_FILE, schema)
        writer.write_table(table)

    return writer

await generate_embeddings()
```

The snippet above will take ~5-6 hours to complete when running on a single NVIDIA L4 with 24GB VRAM + 8 vCPUs with 32GB RAM, which costs 0.70–0.75 USD/hour, meaning that generating 100,000 embeddings costs ~3.75 USD; but it could be reduced further if you were to use CPU-only instances running inference with ONNX weights instead.

Then you can read the `embeddings.parquet` file into a Hugging Face Dataset and upload the generated embeddings to the Hugging Face Hub as:

```python
output = Dataset.from_parquet(PARQUET_FILE)
output.push_to_hub("finewiki-en-100k-embeddings", split="train")
output
```

You can find the generated dataset with the embeddings at [`google-cloud-partnership/finewiki-en-100k-embeddings`](https://huggingface.co/datasets/google-cloud-partnership/finewiki-en-100k-embeddings).

![FineWiki English 100K entries with embeddings](https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/google-cloud/examples/cloud-run/finewiki-embeddings-with-embedding-gemma/embeddings-dataset.png)

Alternatively, you can also keep it locally or insert it into a vector database. As next steps on how to use the pre-computed embeddings, see [Vector Embedding Ingestion with Apache Beam and AlloyDB](https://colab.research.google.com/github/apache/beam/blob/master/examples/notebooks/beam-ml/alloydb_product_catalog_embeddings.ipynb), or also [Search with vector embeddings | Firestore Documentation](https://firebase.google.com/docs/firestore/vector-search).

## Clean up

To avoid inadvertent charges, e.g., if the Cloud Run services are inadvertently invoked more times than your monthly Cloud Run invokement allocation, you should delete the Cloud Run Service, either go to the Cloud Run Console at https://console.cloud.google.com/run and delete the `google--embeddinggemma-300` service; or run the following `gcloud` command:

```python
!gcloud run services delete $SERVICE_NAME --region $LOCATION --quiet
```

Additionally, as you created the service account for this specific service, it might make sense to remove it too:

```python
!gcloud iam service-accounts delete $SERVICE_ACCOUNT@$PROJECT_ID.iam.gserviceaccount.com --quiet
```

## References

- [gcloud run deploy | Google Cloud SDK](https://docs.cloud.google.com/sdk/gcloud/reference/run/deploy)
- [FineWiki: The finest knowledge from the Free Encyclopedia](https://huggingface.co/datasets/HuggingFaceFW/finewiki)
- [Introducing EmbeddingGemma: The Best-in-Class Open Model for On-Device Embeddings](https://developers.googleblog.com/en/introducing-embeddinggemma/)
- [Vector embeddings | OpenAI Developers](https://platform.openai.com/docs/guides/embeddings?lang=python)
