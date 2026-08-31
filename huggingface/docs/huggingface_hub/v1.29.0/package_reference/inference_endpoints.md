# Inference Endpoints

Inference Endpoints provides a secure production solution to easily deploy models on a dedicated and autoscaling infrastructure managed by Hugging Face. An Inference Endpoint is built from a model from the [Hub](https://huggingface.co/models). This page is a reference for `huggingface_hub`'s integration with Inference Endpoints. For more information about the Inference Endpoints product, check out its [official documentation](https://huggingface.co/docs/inference-endpoints/index).

> [!TIP]
> Check out the [related guide](../guides/inference_endpoints) to learn how to use `huggingface_hub` to manage your Inference Endpoints programmatically.

Inference Endpoints can be fully managed via API. The endpoints are documented with [Swagger](https://api.endpoints.huggingface.cloud/). The [InferenceEndpoint](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint) class is a simple wrapper built on top on this API.

## Methods

A subset of the Inference Endpoint features are implemented in [HfApi](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi):

- [get_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.get_inference_endpoint) and [list_inference_endpoints()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.list_inference_endpoints) to get information about your Inference Endpoints
- [list_inference_endpoints_hardware()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.list_inference_endpoints_hardware) to list the hardware you can deploy an Inference Endpoint on
- [create_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.create_inference_endpoint), [update_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.update_inference_endpoint) and [delete_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.delete_inference_endpoint) to deploy and manage Inference Endpoints
- [pause_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.pause_inference_endpoint) and [resume_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.resume_inference_endpoint) to pause and resume an Inference Endpoint
- [scale_to_zero_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.scale_to_zero_inference_endpoint) to manually scale an Endpoint to 0 replicas

## InferenceEndpoint[[huggingface_hub.InferenceEndpoint]]

The main dataclass is [InferenceEndpoint](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint). It contains information about a deployed `InferenceEndpoint`, including its configuration and current state. Once deployed, you can run inference on the Endpoint using the  [InferenceEndpoint.client](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.client) and [InferenceEndpoint.async_client](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.async_client) properties that respectively return an [InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient) and an [AsyncInferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.AsyncInferenceClient) object.

#### huggingface_hub.InferenceEndpoint[[huggingface_hub.InferenceEndpoint]]

```python
huggingface_hub.InferenceEndpoint(namespace: str, raw: dict, _token: str | bool | None, _api: HfApi)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L99)

**Parameters:**

name (`str`) : The unique name of the Inference Endpoint.

namespace (`str`) : The namespace where the Inference Endpoint is located.

repository (`str`) : The name of the model repository deployed on this Inference Endpoint.

status ([InferenceEndpointStatus](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointStatus)) : The current status of the Inference Endpoint.

url (`str`, *optional*) : The URL of the Inference Endpoint, if available. Only a deployed Inference Endpoint will have a URL.

framework (`str`) : The machine learning framework used for the model.

revision (`str`) : The specific model revision deployed on the Inference Endpoint.

task (`str`) : The task associated with the deployed model.

created_at (`datetime.datetime`) : The timestamp when the Inference Endpoint was created.

updated_at (`datetime.datetime`) : The timestamp of the last update of the Inference Endpoint.

type ([InferenceEndpointType](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointType)) : The type of the Inference Endpoint (public, authenticated, private).

raw (`dict`) : The raw dictionary data returned from the API.

token (`str` or `bool`, *optional*) : Authentication token for the Inference Endpoint, if set when requesting the API. Will default to the locally saved token if not provided. Pass `token=False` if you don't want to send your token to the server.

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

#### from_raw[[huggingface_hub.InferenceEndpoint.from_raw]]

```python
from_raw(raw: dict, namespace: str, token: str | bool | None = None, api: typing.Optional[ForwardRef('HfApi')] = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L182)

Initialize object from raw dictionary.

#### client[[huggingface_hub.InferenceEndpoint.client]]

```python
client()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L201)

**Returns:** [InferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.InferenceClient)

an inference client pointing to the deployed endpoint.

**Raises:** [InferenceEndpointError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError)

- [InferenceEndpointError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError) -- If the Inference Endpoint is not yet deployed.

Returns a client to make predictions on this Inference Endpoint.

#### async_client[[huggingface_hub.InferenceEndpoint.async_client]]

```python
async_client()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L223)

**Returns:** [AsyncInferenceClient](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_client#huggingface_hub.AsyncInferenceClient)

an asyncio-compatible inference client pointing to the deployed endpoint.

**Raises:** [InferenceEndpointError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError)

- [InferenceEndpointError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError) -- If the Inference Endpoint is not yet deployed.

Returns a client to make predictions on this Inference Endpoint.

#### delete[[huggingface_hub.InferenceEndpoint.delete]]

```python
delete()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L468)

Delete the Inference Endpoint.

This operation is not reversible. If you don't want to be charged for an Inference Endpoint, it is preferable
to pause it with [InferenceEndpoint.pause()](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.pause) or scale it to zero with [InferenceEndpoint.scale_to_zero()](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.scale_to_zero).

This is an alias for [HfApi.delete_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.delete_inference_endpoint).

#### fetch[[huggingface_hub.InferenceEndpoint.fetch]]

```python
fetch()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L298)

**Returns:** [InferenceEndpoint](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)

the same Inference Endpoint, mutated in place with the latest data.

Fetch latest information about the Inference Endpoint.

#### pause[[huggingface_hub.InferenceEndpoint.pause]]

```python
pause()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L411)

**Returns:** [InferenceEndpoint](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)

the same Inference Endpoint, mutated in place with the latest data.

Pause the Inference Endpoint.

A paused Inference Endpoint will not be charged. It can be resumed at any time using [InferenceEndpoint.resume()](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.resume).
This is different from scaling the Inference Endpoint to zero with [InferenceEndpoint.scale_to_zero()](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.scale_to_zero), which
would be automatically restarted when a request is made to it.

This is an alias for [HfApi.pause_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.pause_inference_endpoint). The current object is mutated in place with the
latest data from the server.

#### resume[[huggingface_hub.InferenceEndpoint.resume]]

```python
resume(running_ok: bool = True)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L429)

**Parameters:**

running_ok (`bool`, *optional*) : If `True`, the method will not raise an error if the Inference Endpoint is already running. Defaults to `True`.

**Returns:** [InferenceEndpoint](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)

the same Inference Endpoint, mutated in place with the latest data.

Resume the Inference Endpoint.

This is an alias for [HfApi.resume_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.resume_inference_endpoint). The current object is mutated in place with the
latest data from the server.

#### scale_to_zero[[huggingface_hub.InferenceEndpoint.scale_to_zero]]

```python
scale_to_zero()
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L450)

**Returns:** [InferenceEndpoint](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)

the same Inference Endpoint, mutated in place with the latest data.

Scale Inference Endpoint to zero.

An Inference Endpoint scaled to zero will not be charged. It will be resumed on the next request to it, with a
cold start delay. This is different from pausing the Inference Endpoint with [InferenceEndpoint.pause()](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.pause), which
would require a manual resume with [InferenceEndpoint.resume()](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint.resume).

This is an alias for [HfApi.scale_to_zero_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.scale_to_zero_inference_endpoint). The current object is mutated in place with the
latest data from the server.

#### update[[huggingface_hub.InferenceEndpoint.update]]

```python
update(accelerator: str | None = None, instance_size: str | None = None, instance_type: str | None = None, min_replica: int | None = None, max_replica: int | None = None, scale_to_zero_timeout: int | None = None, repository: str | None = None, framework: str | None = None, revision: str | None = None, task: str | None = None, custom_image: dict | None = None, container_command: list[str] | None = None, container_args: list[str] | None = None, tensor_parallel_size: int | None = None, data_parallel_size: int | None = None, secrets: dict[str, str] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L309)

**Parameters:**

accelerator (`str`, *optional*) : The hardware accelerator to be used for inference (e.g. `"cpu"`).

instance_size (`str`, *optional*) : The size or type of the instance to be used for hosting the model (e.g. `"x4"`).

instance_type (`str`, *optional*) : The cloud instance type where the Inference Endpoint will be deployed (e.g. `"intel-icl"`).

min_replica (`int`, *optional*) : The minimum number of replicas (instances) to keep running for the Inference Endpoint.

max_replica (`int`, *optional*) : The maximum number of replicas (instances) to scale to for the Inference Endpoint.

scale_to_zero_timeout (`int`, *optional*) : The duration in minutes before an inactive endpoint is scaled to zero. 

repository (`str`, *optional*) : The name of the model repository associated with the Inference Endpoint (e.g. `"gpt2"`).

framework (`str`, *optional*) : The machine learning framework used for the model (e.g. `"custom"`).

revision (`str`, *optional*) : The specific model revision to deploy on the Inference Endpoint (e.g. `"6c0e6080953db56375760c0471a8c5f2929baf11"`).

task (`str`, *optional*) : The task on which to deploy the model (e.g. `"text-classification"`).

custom_image (`dict`, *optional*) : The container image to run. Either a dict keyed by image variant (e.g. `{"vLLM": {"url": "vllm/vllm-openai:v0.23.0", "port": 8000}}`, also `sGLang`, `tgi`, `tei`, `llamacpp`, `hfServe`, ...), which is forwarded as-is, or a flat dict describing a custom container (e.g. `{"url": ..., "port": ...}`), which is sent as `{"custom": ...}`.

container_command (`list[str]`, *optional*) : Override the container entrypoint command (maps to `model.command` in the API payload). Works with both managed engine images (e.g. vLLM, SGLang) and custom images.

container_args (`list[str]`, *optional*) : Arguments appended to the container entrypoint (maps to `model.args` in the API payload). Works with both managed engine images (e.g. vLLM, SGLang) and custom images.

tensor_parallel_size (`int`, *optional*) : Number of accelerators to shard a single model copy across (vLLM and SGLang images). When `custom_image` is not given, the image currently configured on the endpoint is fetched and updated in place, as the API requires `model.image` as a whole.

data_parallel_size (`int`, *optional*) : Number of model copies to run, one per accelerator (vLLM images).

secrets (`dict[str, str]`, *optional*) : Secret values to inject in the container environment.

**Returns:** [InferenceEndpoint](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)

the same Inference Endpoint, mutated in place with the latest data.

Update the Inference Endpoint.

This method allows the update of either the compute configuration, the deployed model, or both. All arguments are
optional but at least one must be provided.

This is an alias for [HfApi.update_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.update_inference_endpoint). The current object is mutated in place with the
latest data from the server.

#### wait[[huggingface_hub.InferenceEndpoint.wait]]

```python
wait(timeout: int | None = None, refresh_every: int = 5)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L245)

**Parameters:**

timeout (`int`, *optional*) : The maximum time to wait for the Inference Endpoint to be deployed, in seconds. If `None`, will wait indefinitely.

refresh_every (`int`, *optional*) : The time to wait between each fetch of the Inference Endpoint status, in seconds. Defaults to 5s.

**Returns:** [InferenceEndpoint](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint)

the same Inference Endpoint, mutated in place with the latest data.

**Raises:** [InferenceEndpointError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError) or `InferenceEndpointTimeoutError`

- [InferenceEndpointError](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpointError) -- 
  If the Inference Endpoint ended up in a failed state.
- `InferenceEndpointTimeoutError` -- 
  If the Inference Endpoint is not deployed after `timeout` seconds.

Wait for the Inference Endpoint to be deployed.

Information from the server will be fetched every 1s. If the Inference Endpoint is not deployed after `timeout`
seconds, a `InferenceEndpointTimeoutError` will be raised. The [InferenceEndpoint](/docs/huggingface_hub/v1.29.0/en/package_reference/inference_endpoints#huggingface_hub.InferenceEndpoint) will be mutated in place with the latest
data.

## InferenceEndpointHardware[[huggingface_hub.InferenceEndpointHardware]]

#### huggingface_hub.InferenceEndpointHardware[[huggingface_hub.InferenceEndpointHardware]]

```python
huggingface_hub.InferenceEndpointHardware(id: str, vendor: str, region: str, accelerator: str, instance_type: str, instance_size: str, architecture: str, num_accelerators: int, num_cpus: int | None, memory_gb: float, gpu_memory_gb: int | None, price_per_hour: float, status: str, max_accelerators: int, used_accelerators: int)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L500)

**Parameters:**

id (`str`) : Unique hardware identifier, e.g. `"aws-us-east-1-nvidia-l4-x1"`.

vendor (`str`) : The cloud provider hosting the hardware, e.g. `"aws"`.

region (`str`) : The cloud region the hardware is available in, e.g. `"us-east-1"`.

accelerator (`str`) : The type of hardware accelerator, e.g. `"cpu"`, `"gpu"` or `"neuron"`.

instance_type (`str`) : The cloud instance type, e.g. `"nvidia-l4"`.

instance_size (`str`) : The instance size multiplier, e.g. `"x1"`.

architecture (`str`) : Human-readable hardware description, e.g. `"Nvidia L4"`.

num_accelerators (`int`) : Number of accelerator units per replica.

num_cpus (`int`, *optional*) : Number of vCPUs per replica.

memory_gb (`float`) : RAM per replica, in GB.

gpu_memory_gb (`int`, *optional*) : Total GPU memory per replica, in GB (i.e. summed over `num_accelerators`). `None` for non-GPU hardware.

price_per_hour (`float`) : Cost per replica per hour, in USD.

status (`str`) : Availability of the hardware: `"available"`, `"low_availability"`, `"not_available"`, `"reserved"` or `"deprecated"`.

max_accelerators (`int`) : Maximum number of accelerators of this type the namespace is allowed to run.

used_accelerators (`int`) : Number of accelerators of this type currently used by the namespace.

Contains information about a hardware configuration available for Inference Endpoints.

The `vendor`, `region`, `accelerator`, `instance_type` and `instance_size` fields are exactly the values to pass to
[create_inference_endpoint()](/docs/huggingface_hub/v1.29.0/en/package_reference/hf_api#huggingface_hub.HfApi.create_inference_endpoint) (or `hf endpoints deploy`) to deploy on this hardware.

Example:
```python
>>> from huggingface_hub import list_inference_endpoints_hardware
>>> hardware = list_inference_endpoints_hardware()
>>> hardware[0]
InferenceEndpointHardware(id='aws-us-east-1-nvidia-l4-x1', vendor='aws', region='us-east-1', ...)
```

#### from_raw[[huggingface_hub.InferenceEndpointHardware.from_raw]]

```python
from_raw(raw: dict, vendor: str, region: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L565)

Initialize object from a raw compute dictionary, nested under a vendor and a region in the API response.

## InferenceEndpointStatus[[huggingface_hub.InferenceEndpointStatus]]

#### huggingface_hub.InferenceEndpointStatus[[huggingface_hub.InferenceEndpointStatus]]

```python
huggingface_hub.InferenceEndpointStatus(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L75)

An enumeration.

## InferenceEndpointType[[huggingface_hub.InferenceEndpointType]]

#### huggingface_hub.InferenceEndpointType[[huggingface_hub.InferenceEndpointType]]

```python
huggingface_hub.InferenceEndpointType(value, names = None, module = None, qualname = None, type = None, start = 1)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_inference_endpoints.py#L86)

An enumeration.

## InferenceEndpointError[[huggingface_hub.InferenceEndpointError]]

#### huggingface_hub.InferenceEndpointError[[huggingface_hub.InferenceEndpointError]]

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/errors.py#L162)

Generic exception when dealing with Inference Endpoints.
