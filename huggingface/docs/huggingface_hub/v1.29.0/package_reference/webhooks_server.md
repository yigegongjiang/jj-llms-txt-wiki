# Webhooks Server

Webhooks are a foundation for MLOps-related features. They allow you to listen for new changes on specific repos or to
all repos belonging to particular users/organizations you're interested in following. To learn
more about webhooks on the Huggingface Hub, you can read the Webhooks [guide](https://huggingface.co/docs/hub/webhooks).

> [!TIP]
> Check out this [guide](../guides/webhooks_server) for a step-by-step tutorial on how to set up your webhooks server and
> deploy it as a Space.

> [!WARNING]
> This is an experimental feature. This means that we are still working on improving the API. Breaking changes might be
> introduced in the future without prior notice. Make sure to pin the version of `huggingface_hub` in your requirements.
> A warning is triggered when you use an experimental feature. You can disable it by setting `HF_HUB_DISABLE_EXPERIMENTAL_WARNING=1` as an environment variable.

## Server

The server is a [Gradio](https://gradio.app/) app. It has a UI to display instructions for you or your users and an API
to listen to webhooks. Implementing a webhook endpoint is as simple as decorating a function. You can then debug it
by redirecting the Webhooks to your machine (using a Gradio tunnel) before deploying it to a Space.

### WebhooksServer[[huggingface_hub.WebhooksServer]]

#### huggingface_hub.WebhooksServer[[huggingface_hub.WebhooksServer]]

```python
huggingface_hub.WebhooksServer(*args, **kwargs)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_server.py#L43)

**Parameters:**

ui (`gradio.Blocks`, optional) : A Gradio UI instance to be used as the Space landing page. If `None`, a UI displaying instructions about the configured webhooks is created.

webhook_secret (`str`, optional) : A secret key to verify incoming webhook requests. You can set this value to any secret you want as long as you also configure it in your [webhooks settings panel](https://huggingface.co/settings/webhooks). You can also set this value as the `WEBHOOK_SECRET` environment variable. If no secret is provided, the webhook endpoints are opened without any security.

The [WebhooksServer()](/docs/huggingface_hub/v1.29.0/en/package_reference/webhooks_server#huggingface_hub.WebhooksServer) class lets you create an instance of a Gradio app that can receive Huggingface webhooks.
These webhooks can be registered using the `add_webhook()` decorator. Webhook endpoints are added to
the app as a POST endpoint to the FastAPI router. Once all the webhooks are registered, the `launch` method has to be
called to start the app.

It is recommended to accept [WebhookPayload](/docs/huggingface_hub/v1.29.0/en/package_reference/webhooks_server#huggingface_hub.WebhookPayload) as the first argument of the webhook function. It is a Pydantic
model that contains all the information about the webhook event. The data will be parsed automatically for you.

Check out the [webhooks guide](../guides/webhooks_server) for a step-by-step tutorial on how to set up your
WebhooksServer and deploy it on a Space.

> [!WARNING]
> `WebhooksServer` is experimental. Its API is subject to change in the future.

> [!WARNING]
> You must have `gradio` installed to use `WebhooksServer` (`pip install --upgrade gradio`).

Example:

```python
import gradio as gr
from huggingface_hub import WebhooksServer, WebhookPayload

with gr.Blocks() as ui:
    ...

app = WebhooksServer(ui=ui, webhook_secret="my_secret_key")

@app.add_webhook("/say_hello")
async def hello(payload: WebhookPayload):
    return {"message": "hello"}

app.launch()
```

### @webhook_endpoint[[huggingface_hub.webhook_endpoint]]

#### huggingface_hub.webhook_endpoint[[huggingface_hub.webhook_endpoint]]

```python
huggingface_hub.webhook_endpoint(path: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_server.py#L226)

**Parameters:**

path (`str`, optional) : The URL path to register the webhook function. If not provided, the function name will be used as the path. In any case, all webhooks are registered under `/webhooks`.

Decorator to start a [WebhooksServer()](/docs/huggingface_hub/v1.29.0/en/package_reference/webhooks_server#huggingface_hub.WebhooksServer) and register the decorated function as a webhook endpoint.

This is a helper to get started quickly. If you need more flexibility (custom landing page or webhook secret),
you can use [WebhooksServer()](/docs/huggingface_hub/v1.29.0/en/package_reference/webhooks_server#huggingface_hub.WebhooksServer) directly. You can register multiple webhook endpoints (to the same server) by using
this decorator multiple times.

Check out the [webhooks guide](../guides/webhooks_server) for a step-by-step tutorial on how to set up your
server and deploy it on a Space.

> [!WARNING]
> `webhook_endpoint` is experimental. Its API is subject to change in the future.

> [!WARNING]
> You must have `gradio` installed to use `webhook_endpoint` (`pip install --upgrade gradio`).

Examples:
The default usage is to register a function as a webhook endpoint. The function name will be used as the path.
The server will be started automatically at exit (i.e. at the end of the script).

```python
from huggingface_hub import webhook_endpoint, WebhookPayload

@webhook_endpoint
async def trigger_training(payload: WebhookPayload):
    if payload.repo.type == "dataset" and payload.event.action == "update":
        # Trigger a training job if a dataset is updated
        ...

# Server is automatically started at the end of the script.
```

Advanced usage: register a function as a webhook endpoint and start the server manually. This is useful if you
are running it in a notebook.

```python
from huggingface_hub import webhook_endpoint, WebhookPayload

@webhook_endpoint
async def trigger_training(payload: WebhookPayload):
    if payload.repo.type == "dataset" and payload.event.action == "update":
        # Trigger a training job if a dataset is updated
        ...

# Start the server manually
trigger_training.launch()
```

## Payload[[huggingface_hub.WebhookPayload]]

[WebhookPayload](/docs/huggingface_hub/v1.29.0/en/package_reference/webhooks_server#huggingface_hub.WebhookPayload) is the main data structure that contains the payload from Webhooks. This is
a `pydantic` class which makes it very easy to use with FastAPI. If you pass it as a parameter to a webhook endpoint, it
will be automatically validated and parsed as a Python object.

For more information about webhooks payload, you can refer to the Webhooks Payload [guide](https://huggingface.co/docs/hub/webhooks#webhook-payloads).

#### huggingface_hub.WebhookPayload[[huggingface_hub.WebhookPayload]]

```python
huggingface_hub.WebhookPayload(event: WebhookPayloadEvent, repo: WebhookPayloadRepo, discussion: huggingface_hub._webhooks_payload.WebhookPayloadDiscussion | None = None, comment: huggingface_hub._webhooks_payload.WebhookPayloadComment | None = None, webhook: WebhookPayloadWebhook, movedTo: huggingface_hub._webhooks_payload.WebhookPayloadMovedTo | None = None, updatedRefs: list[huggingface_hub._webhooks_payload.WebhookPayloadUpdatedRef] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_payload.py#L129)

### WebhookPayload[[huggingface_hub.WebhookPayload]]

#### huggingface_hub.WebhookPayload[[huggingface_hub.WebhookPayload]]

```python
huggingface_hub.WebhookPayload(event: WebhookPayloadEvent, repo: WebhookPayloadRepo, discussion: huggingface_hub._webhooks_payload.WebhookPayloadDiscussion | None = None, comment: huggingface_hub._webhooks_payload.WebhookPayloadComment | None = None, webhook: WebhookPayloadWebhook, movedTo: huggingface_hub._webhooks_payload.WebhookPayloadMovedTo | None = None, updatedRefs: list[huggingface_hub._webhooks_payload.WebhookPayloadUpdatedRef] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_payload.py#L129)

### WebhookPayloadComment[[huggingface_hub.WebhookPayloadComment]]

#### huggingface_hub.WebhookPayloadComment[[huggingface_hub.WebhookPayloadComment]]

```python
huggingface_hub.WebhookPayloadComment(id: str, author: ObjectId, hidden: bool, content: str | None = None, url: WebhookPayloadUrl)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_payload.py#L94)

### WebhookPayloadDiscussion[[huggingface_hub.WebhookPayloadDiscussion]]

#### huggingface_hub.WebhookPayloadDiscussion[[huggingface_hub.WebhookPayloadDiscussion]]

```python
huggingface_hub.WebhookPayloadDiscussion(id: str, num: int, author: ObjectId, url: WebhookPayloadUrl, title: str, isPullRequest: bool, status: typing.Literal['closed', 'draft', 'open', 'merged'], changes: huggingface_hub._webhooks_payload.WebhookPayloadDiscussionChanges | None = None, pinned: bool | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_payload.py#L101)

### WebhookPayloadDiscussionChanges[[huggingface_hub.WebhookPayloadDiscussionChanges]]

#### huggingface_hub.WebhookPayloadDiscussionChanges[[huggingface_hub.WebhookPayloadDiscussionChanges]]

```python
huggingface_hub.WebhookPayloadDiscussionChanges(base: str, mergeCommitId: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_payload.py#L89)

### WebhookPayloadEvent[[huggingface_hub.WebhookPayloadEvent]]

#### huggingface_hub.WebhookPayloadEvent[[huggingface_hub.WebhookPayloadEvent]]

```python
huggingface_hub.WebhookPayloadEvent(action: typing.Literal['create', 'delete', 'move', 'update'], scope: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_payload.py#L84)

### WebhookPayloadMovedTo[[huggingface_hub.WebhookPayloadMovedTo]]

#### huggingface_hub.WebhookPayloadMovedTo[[huggingface_hub.WebhookPayloadMovedTo]]

```python
huggingface_hub.WebhookPayloadMovedTo(name: str, owner: ObjectId)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_payload.py#L75)

### WebhookPayloadRepo[[huggingface_hub.WebhookPayloadRepo]]

#### huggingface_hub.WebhookPayloadRepo[[huggingface_hub.WebhookPayloadRepo]]

```python
huggingface_hub.WebhookPayloadRepo(id: str, owner: ObjectId, head_sha: str | None = None, name: str, private: bool, subdomain: str | None = None, tags: list[str] | None = None, type: typing.Literal['dataset', 'model', 'space'], url: WebhookPayloadUrl)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_payload.py#L112)

### WebhookPayloadUrl[[huggingface_hub.WebhookPayloadUrl]]

#### huggingface_hub.WebhookPayloadUrl[[huggingface_hub.WebhookPayloadUrl]]

```python
huggingface_hub.WebhookPayloadUrl(web: str, api: str | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_payload.py#L70)

### WebhookPayloadWebhook[[huggingface_hub.WebhookPayloadWebhook]]

#### huggingface_hub.WebhookPayloadWebhook[[huggingface_hub.WebhookPayloadWebhook]]

```python
huggingface_hub.WebhookPayloadWebhook(id: str, version: typing.Literal[3])
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_webhooks_payload.py#L80)
