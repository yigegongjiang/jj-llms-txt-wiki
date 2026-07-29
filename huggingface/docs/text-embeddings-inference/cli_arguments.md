# CLI arguments

To see all options to serve your models, run the following:

```console
$ text-embeddings-router --help
Text Embedding Webserver

Usage: text-embeddings-router [OPTIONS] --model-id 

Options:
      --model-id 
          The Hugging Face model ID, can be any model listed on  with the `text-embeddings-inference` tag (meaning it's compatible with Text Embeddings Inference).

          Alternatively, the specified ID can also be a path to a local directory containing the necessary model files saved by the `save_pretrained(...)` methods of either Transformers or Sentence Transformers.

          [env: MODEL_ID=]

      --revision 
          The actual revision of the model if you're referring to a model on the hub. You can use a specific commit id or a branch like `refs/pr/2`

          [env: REVISION=]

      --tokenization-workers 
          Optionally control the number of tokenizer workers used for payload tokenization, validation and truncation. Default to the number of CPU cores on the machine

          [env: TOKENIZATION_WORKERS=]

      --dtype 
          The dtype to be forced upon the model

          [env: DTYPE=]
          [possible values: float16, float32]

      --served-model-name 
          The name of the model that is being served. If not specified, defaults to `--model-id`. It is only used for the OpenAI-compatible endpoints via HTTP

          [env: SERVED_MODEL_NAME=]

      --pooling 
          Optionally control the pooling method for embedding models.

          If `pooling` is not set, the pooling configuration will be parsed from the model `1_Pooling/config.json` configuration.

          If `pooling` is set, it will override the model pooling configuration

          [env: POOLING=]

          Possible values:
          - cls:        Select the CLS token as embedding
          - mean:       Apply Mean pooling to the model embeddings
          - splade:     Apply SPLADE (Sparse Lexical and Expansion) to the model embeddings. This option is only available if the loaded model is a `ForMaskedLM` Transformer model
          - last-token: Select the last token as embedding

      --max-concurrent-requests 
          The maximum amount of concurrent requests for this particular deployment. Having a low limit will refuse clients requests instead of having them wait for too long and is usually good to handle backpressure correctly

          [env: MAX_CONCURRENT_REQUESTS=]
          [default: 512]

      --max-batch-tokens 
          **IMPORTANT** This is one critical control to allow maximum usage of the available hardware.

          This represents the total amount of potential tokens within a batch.

          For `max_batch_tokens=1000`, you could fit `10` queries of `total_tokens=100` or a single query of `1000` tokens.

          Overall this number should be the largest possible until the model is compute bound. Since the actual memory overhead depends on the model implementation, text-embeddings-inference cannot infer this number automatically.

          [env: MAX_BATCH_TOKENS=]
          [default: 16384]

      --max-batch-requests 
          Optionally control the maximum number of individual requests in a batch

          [env: MAX_BATCH_REQUESTS=]

      --max-client-batch-size 
          Control the maximum number of inputs that a client can send in a single request

          [env: MAX_CLIENT_BATCH_SIZE=]
          [default: 32]

      --auto-truncate
          Control automatic truncation of inputs that exceed the model's maximum supported size. Defaults to `true` (truncation enabled). Set to `false` to disable truncation; when disabled and the model's maximum input length exceeds `--max-batch-tokens`, the server will refuse to start with an error instead of silently truncating sequences.

          Unused for gRPC servers

          [env: AUTO_TRUNCATE=]

      --default-prompt-name 
          The name of the prompt that should be used by default for encoding. If not set, no prompt will be applied.

          Must be a key in the `sentence-transformers` configuration `prompts` dictionary.

          For example if ``default_prompt_name`` is "query" and the ``prompts`` is {"query": "query: ", ...}, then the sentence "What is the capital of France?" will be encoded as "query: What is the capital of France?" because the prompt text will be prepended before any text to encode.

          The argument '--default-prompt-name ' cannot be used with '--default-prompt `

          [env: DEFAULT_PROMPT_NAME=]

      --default-prompt 
          The prompt that should be used by default for encoding. If not set, no prompt will be applied.

          For example if ``default_prompt`` is "query: " then the sentence "What is the capital of France?" will be encoded as "query: What is the capital of France?" because the prompt text will be prepended before any text to encode.

          The argument '--default-prompt ' cannot be used with '--default-prompt-name `

          [env: DEFAULT_PROMPT=]

      --dense-path 
          Optionally, define the path to the Dense module required for some embedding models.

          Some embedding models require an extra `Dense` module which contains a single Linear layer and an activation function. By default, those `Dense` modules are stored under the `2_Dense` directory, but there might be cases where different `Dense` modules are provided, to convert the pooled embeddings into different dimensions, available as `2_Dense_` e.g. https://huggingface.co/NovaSearch/stella_en_400M_v5.

          Note that this argument is optional, only required to be set if there is no `modules.json` file or when you want to override a single Dense module path, only when running with the `candle` backend.

          [env: DENSE_PATH=]

      --hf-token 
          Your Hugging Face Hub token. If neither `--hf-token` nor `HF_TOKEN` are set, the token will be read from the `$HF_HOME/token` path, if it exists. This ensures access to private or gated models, and allows for a more permissive rate limiting

          [env: HF_TOKEN=]

      --hostname 
          The IP address to listen on

          [env: HOSTNAME=]
          [default: 0.0.0.0]

      -p, --port 
          The port to listen on

          [env: PORT=]
          [default: 3000]

      --uds-path 
          The name of the unix socket some text-embeddings-inference backends will use as they communicate internally with gRPC

          [env: UDS_PATH=]
          [default: /tmp/text-embeddings-inference-server]

      --huggingface-hub-cache 
          The location of the huggingface hub cache. Used to override the location if you want to provide a mounted disk for instance

          [env: HUGGINGFACE_HUB_CACHE=]

      --payload-limit 
          Payload size limit in bytes

          Default is 2MB

          [env: PAYLOAD_LIMIT=]
          [default: 2000000]

      --api-key 
          Set an api key for request authorization.

          By default the server responds to every request. With an api key set, the requests must have the Authorization header set with the api key as Bearer token.

          [env: API_KEY=]

      --json-output
          Outputs the logs in JSON format (useful for telemetry)

          [env: JSON_OUTPUT=]

      --disable-spans
          Whether or not to include the log trace through spans

          [env: DISABLE_SPANS=]

      --otlp-endpoint 
          The grpc endpoint for opentelemetry. Telemetry is sent to this endpoint as OTLP over gRPC. e.g. `http://localhost:4317`

          [env: OTLP_ENDPOINT=]

      --otlp-service-name 
          The service name for opentelemetry. e.g. `text-embeddings-inference.server`

          [env: OTLP_SERVICE_NAME=]
          [default: text-embeddings-inference.server]

      --prometheus-port 
          The Prometheus port to listen on

          [env: PROMETHEUS_PORT=]
          [default: 9000]

      --cors-allow-origin 
          Unused for gRPC servers

          [env: CORS_ALLOW_ORIGIN=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
