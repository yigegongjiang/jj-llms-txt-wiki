# Inference Endpoints

Inference Endpoints provides a secure production solution to easily deploy models on a dedicated and autoscaling infrastructure managed by Hugging Face. An Inference Endpoint is built from a model from the [Hub](https://huggingface.co/models). This page is a reference for `huggingface_hub`'s integration with Inference Endpoints. For more information about the Inference Endpoints product, check out its [official documentation](https://huggingface.co/docs/inference-endpoints/index).

> [!TIP]
> Check out the [related guide](../guides/inference_endpoints) to learn how to use `huggingface_hub` to manage your Inference Endpoints programmatically.

Inference Endpoints can be fully managed via API. The endpoints are documented with [Swagger](https://api.endpoints.huggingface.cloud/). The [InferenceEndpoint](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint) class is a simple wrapper built on top on this API.

## Methods

A subset of the Inference Endpoint features are implemented in [HfApi](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi):

- [get_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.get_inference_endpoint) and [list_inference_endpoints()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.list_inference_endpoints) to get information about your Inference Endpoints
- [create_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.create_inference_endpoint), [update_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.update_inference_endpoint) and [delete_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.delete_inference_endpoint) to deploy and manage Inference Endpoints
- [pause_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.pause_inference_endpoint) and [resume_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.resume_inference_endpoint) to pause and resume an Inference Endpoint
- [scale_to_zero_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.scale_to_zero_inference_endpoint) to manually scale an Endpoint to 0 replicas

## InferenceEndpoint[[huggingface_hub.InferenceEndpoint]]

The main dataclass is [InferenceEndpoint](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint). It contains information about a deployed `InferenceEndpoint`, including its configuration and current state. Once deployed, you can run inference on the Endpoint using the  [InferenceEndpoint.client](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.client) and [InferenceEndpoint.async_client](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.async_client) properties that respectively return an [InferenceClient](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_client#huggingface_hub.InferenceClient) and an [AsyncInferenceClient](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_client#huggingface_hub.AsyncInferenceClient) object.

- **name** (`str`) --
  The unique name of the Inference Endpoint.
- **namespace** (`str`) --
  The namespace where the Inference Endpoint is located.
- **repository** (`str`) --
  The name of the model repository deployed on this Inference Endpoint.
- **status** ([InferenceEndpointStatus](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointStatus)) --
  The current status of the Inference Endpoint.
- **url** (`str`, *optional*) --
  The URL of the Inference Endpoint, if available. Only a deployed Inference Endpoint will have a URL.
- **framework** (`str`) --
  The machine learning framework used for the model.
- **revision** (`str`) --
  The specific model revision deployed on the Inference Endpoint.
- **task** (`str`) --
  The task associated with the deployed model.
- **created_at** (`datetime.datetime`) --
  The timestamp when the Inference Endpoint was created.
- **updated_at** (`datetime.datetime`) --
  The timestamp of the last update of the Inference Endpoint.
- **type** ([InferenceEndpointType](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointType)) --
  The type of the Inference Endpoint (public, authenticated, private).
- **raw** (`dict`) --
  The raw dictionary data returned from the API.
- **token** (`str` or `bool`, *optional*) --
  Authentication token for the Inference Endpoint, if set when requesting the API. Will default to the
  locally saved token if not provided. Pass `token=False` if you don't want to send your token to the server.

Contains information about a deployed Inference Endpoint.

Example:
```python
>>> from huggingface_hub import get_inference_endpoint
>>> endpoint = get_inference_endpoint("my-text-to-image")
>>> endpoint
InferenceEndpoint(name='my-text-to-image', ...)

# Get status
>>> endpoint.status
'running'
>>> endpoint.url
'https://my-text-to-image.region.vendor.endpoints.huggingface.cloud'

# Run inference
>>> endpoint.client.text_to_image(...)

# Pause endpoint to save $$$
>>> endpoint.pause()

# ...
# Resume and wait for deployment
>>> endpoint.resume()
>>> endpoint.wait()
>>> endpoint.client.text_to_image(...)
```

Initialize object from raw dictionary.

[InferenceClient](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_client#huggingface_hub.InferenceClient)an inference client pointing to the deployed endpoint.- [InferenceEndpointError](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError) -- If the Inference Endpoint is not yet deployed.[InferenceEndpointError](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError)
Returns a client to make predictions on this Inference Endpoint.

[AsyncInferenceClient](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_client#huggingface_hub.AsyncInferenceClient)an asyncio-compatible inference client pointing to the deployed endpoint.- [InferenceEndpointError](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError) -- If the Inference Endpoint is not yet deployed.[InferenceEndpointError](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError)
Returns a client to make predictions on this Inference Endpoint.

Delete the Inference Endpoint.

This operation is not reversible. If you don't want to be charged for an Inference Endpoint, it is preferable
to pause it with [InferenceEndpoint.pause()](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.pause) or scale it to zero with [InferenceEndpoint.scale_to_zero()](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.scale_to_zero).

This is an alias for [HfApi.delete_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.delete_inference_endpoint).

[InferenceEndpoint](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)the same Inference Endpoint, mutated in place with the latest data.
Fetch latest information about the Inference Endpoint.

[InferenceEndpoint](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)the same Inference Endpoint, mutated in place with the latest data.
Pause the Inference Endpoint.

A paused Inference Endpoint will not be charged. It can be resumed at any time using [InferenceEndpoint.resume()](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.resume).
This is different from scaling the Inference Endpoint to zero with [InferenceEndpoint.scale_to_zero()](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.scale_to_zero), which
would be automatically restarted when a request is made to it.

This is an alias for [HfApi.pause_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.pause_inference_endpoint). The current object is mutated in place with the
latest data from the server.

- **running_ok** (`bool`, *optional*) --
  If `True`, the method will not raise an error if the Inference Endpoint is already running. Defaults to
  `True`.[InferenceEndpoint](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)the same Inference Endpoint, mutated in place with the latest data.
Resume the Inference Endpoint.

This is an alias for [HfApi.resume_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.resume_inference_endpoint). The current object is mutated in place with the
latest data from the server.

[InferenceEndpoint](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)the same Inference Endpoint, mutated in place with the latest data.
Scale Inference Endpoint to zero.

An Inference Endpoint scaled to zero will not be charged. It will be resumed on the next request to it, with a
cold start delay. This is different from pausing the Inference Endpoint with [InferenceEndpoint.pause()](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.pause), which
would require a manual resume with [InferenceEndpoint.resume()](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.resume).

This is an alias for [HfApi.scale_to_zero_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.scale_to_zero_inference_endpoint). The current object is mutated in place with the
latest data from the server.

- **accelerator** (`str`, *optional*) --
  The hardware accelerator to be used for inference (e.g. `"cpu"`).
- **instance_size** (`str`, *optional*) --
  The size or type of the instance to be used for hosting the model (e.g. `"x4"`).
- **instance_type** (`str`, *optional*) --
  The cloud instance type where the Inference Endpoint will be deployed (e.g. `"intel-icl"`).
- **min_replica** (`int`, *optional*) --
  The minimum number of replicas (instances) to keep running for the Inference Endpoint.
- **max_replica** (`int`, *optional*) --
  The maximum number of replicas (instances) to scale to for the Inference Endpoint.
- **scale_to_zero_timeout** (`int`, *optional*) --
  The duration in minutes before an inactive endpoint is scaled to zero.

- **repository** (`str`, *optional*) --
  The name of the model repository associated with the Inference Endpoint (e.g. `"gpt2"`).
- **framework** (`str`, *optional*) --
  The machine learning framework used for the model (e.g. `"custom"`).
- **revision** (`str`, *optional*) --
  The specific model revision to deploy on the Inference Endpoint (e.g. `"6c0e6080953db56375760c0471a8c5f2929baf11"`).
- **task** (`str`, *optional*) --
  The task on which to deploy the model (e.g. `"text-classification"`).
- **custom_image** (`dict`, *optional*) --
  A custom Docker image to use for the Inference Endpoint. This is useful if you want to deploy an
  Inference Endpoint running on the `text-generation-inference` (TGI) framework (see examples).
- **secrets** (`dict[str, str]`, *optional*) --
  Secret values to inject in the container environment.[InferenceEndpoint](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)the same Inference Endpoint, mutated in place with the latest data.
Update the Inference Endpoint.

This method allows the update of either the compute configuration, the deployed model, or both. All arguments are
optional but at least one must be provided.

This is an alias for [HfApi.update_inference_endpoint()](/docs/huggingface_hub/v1.25.1/en/package_reference/hf_api#huggingface_hub.HfApi.update_inference_endpoint). The current object is mutated in place with the
latest data from the server.

- **timeout** (`int`, *optional*) --
  The maximum time to wait for the Inference Endpoint to be deployed, in seconds. If `None`, will wait
  indefinitely.
- **refresh_every** (`int`, *optional*) --
  The time to wait between each fetch of the Inference Endpoint status, in seconds. Defaults to 5s.[InferenceEndpoint](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)the same Inference Endpoint, mutated in place with the latest data.- [InferenceEndpointError](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError) -- 
  If the Inference Endpoint ended up in a failed state.
- `InferenceEndpointTimeoutError` -- 
  If the Inference Endpoint is not deployed after `timeout` seconds.[InferenceEndpointError](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError) or `InferenceEndpointTimeoutError`
Wait for the Inference Endpoint to be deployed.

Information from the server will be fetched every 1s. If the Inference Endpoint is not deployed after `timeout`
seconds, a `InferenceEndpointTimeoutError` will be raised. The [InferenceEndpoint](/docs/huggingface_hub/v1.25.1/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint) will be mutated in place with the latest
data.

## InferenceEndpointStatus[[huggingface_hub.InferenceEndpointStatus]]

An enumeration.

## InferenceEndpointType[[huggingface_hub.InferenceEndpointType]]

An enumeration.

## InferenceEndpointError[[huggingface_hub.InferenceEndpointError]]

Generic exception when dealing with Inference Endpoints.
