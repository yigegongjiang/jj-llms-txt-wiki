---
title: Create Queue
---

[Skip to content](#%5Ftop) 

[API Reference](https://developers.cloudflare.com/api)

[Queues](https://developers.cloudflare.com/api/resources/queues)

Copy Markdown

Open in **Claude**

Open in **ChatGPT**

Open in **Cursor**

---

**Copy Markdown**

**View as Markdown**

# Create Queue

POST/accounts/{account\_id}/queues

Create a new queue

##### Security

API Token

The preferred authorization scheme for interacting with the Cloudflare API. [Create a token](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/).

**Example:**`Authorization: Bearer Sn3lZJTBX6kkg7OdcBUAxOO963GEIyGQqnFTOFYY`

API Email + API Key

The previous authorization scheme for interacting with the Cloudflare API, used in conjunction with a Global API key.

**Example:**`X-Auth-Email: user@example.com`

The previous authorization scheme for interacting with the Cloudflare API. When possible, use API tokens instead of Global API keys.

**Example:**`X-Auth-Key: 144c9defac04969c7bfad8efaa8ea194`

##### Accepted Permissions (at least one required)

`Queues Write` `Workers Scripts Write`

##### Path ParametersExpand Collapse 

account\_id: string

A Resource identifier.

maxLength32

##### Body ParametersJSONExpand Collapse 

queue\_name: string

##### ReturnsExpand Collapse 

errors: optional array of [ResponseInfo](https://developers.cloudflare.com/api/resources/$shared#%28resource%29%20%24shared%20%3E%20%28model%29%20response%5Finfo%20%3E%20%28schema%29) { code, message, documentation\_url, source } 

minLength1

code: number

minimum1000

message: string

documentation\_url: optional string

source: optional object { pointer } 

pointer: optional string

messages: optional array of string

result: optional [Queue](https://developers.cloudflare.com/api/resources/queues#%28resource%29%20queues%20%3E%20%28model%29%20queue%20%3E%20%28schema%29) { consumers, consumers\_total\_count, created\_on, 6 more } 

consumers: optional array of [Consumer](https://developers.cloudflare.com/api/resources/queues#%28resource%29%20queues.consumers%20%3E%20%28model%29%20consumer%20%3E%20%28schema%29)

One of the following:

Worker object { consumer\_id, created\_on, dead\_letter\_queue, 4 more } 

consumer\_id: optional string

A Resource identifier.

maxLength32

created\_on: optional string

formatdate-time

dead\_letter\_queue: optional string

Name of the dead letter queue, or empty string if not configured

queue\_name: optional string

script\_name: optional string

Name of a Worker

settings: optional object { batch\_size, max\_concurrency, max\_retries, 2 more } 

batch\_size: optional number

The maximum number of messages to include in a batch.

max\_concurrency: optional number

Maximum number of concurrent consumers that may consume from this Queue. Set to `null` to automatically opt in to the platform’s maximum (recommended).

max\_retries: optional number

The maximum number of retries

max\_wait\_time\_ms: optional number

The number of milliseconds to wait for a batch to fill up before attempting to deliver it

retry\_delay: optional number

The number of seconds to delay before making the message available for another attempt.

type: optional "worker"

HTTPPull object { consumer\_id, created\_on, dead\_letter\_queue, 3 more } 

consumer\_id: optional string

A Resource identifier.

maxLength32

created\_on: optional string

formatdate-time

dead\_letter\_queue: optional string

Name of the dead letter queue, or empty string if not configured

queue\_name: optional string

settings: optional object { batch\_size, max\_retries, retry\_delay, visibility\_timeout\_ms } 

batch\_size: optional number

The maximum number of messages to include in a batch.

max\_retries: optional number

The maximum number of retries

retry\_delay: optional number

The number of seconds to delay before making the message available for another attempt.

visibility\_timeout\_ms: optional number

The number of milliseconds that a message is exclusively leased. After the timeout, the message becomes available for another attempt.

type: optional "http\_pull"

consumers\_total\_count: optional number

created\_on: optional string

modified\_on: optional string

producers: optional array of object { script, type } or object { bucket\_name, type } 

One of the following:

MqWorkerProducer object { script, type } 

script: optional string

type: optional "worker"

MqR2Producer object { bucket\_name, type } 

bucket\_name: optional string

type: optional "r2\_bucket"

producers\_total\_count: optional number

queue\_id: optional string

queue\_name: optional string

settings: optional object { delivery\_delay, delivery\_paused, message\_retention\_period } 

delivery\_delay: optional number

Number of seconds to delay delivery of all messages to consumers.

delivery\_paused: optional boolean

Indicates if message delivery to consumers is currently paused.

message\_retention\_period: optional number

Number of seconds after which an unconsumed message will be delayed.

success: optional true

Indicates if the API call was successful or not.

### Create Queue

HTTP

HTTPHTTP

TypeScriptTypeScript

PythonPython

GoGo

TerraformTerraform

```
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/queues \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "queue_name": "example-queue"
        }'
```

200 example

```
{
  "errors": [
    {
      "code": 7003,
      "message": "No route for the URI",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    "string"
  ],
  "result": {
    "consumers": [
      {
        "consumer_id": "023e105f4ecef8ad9ca31a8372d0c353",
        "created_on": "2019-12-27T18:11:19.117Z",
        "dead_letter_queue": "dead_letter_queue",
        "queue_name": "example-queue",
        "script_name": "my-consumer-worker",
        "settings": {
          "batch_size": 50,
          "max_concurrency": 10,
          "max_retries": 3,
          "max_wait_time_ms": 5000,
          "retry_delay": 10
        },
        "type": "worker"
      }
    ],
    "consumers_total_count": 0,
    "created_on": "created_on",
    "modified_on": "modified_on",
    "producers": [
      {
        "script": "script",
        "type": "worker"
      }
    ],
    "producers_total_count": 0,
    "queue_id": "queue_id",
    "queue_name": "example-queue",
    "settings": {
      "delivery_delay": 5,
      "delivery_paused": true,
      "message_retention_period": 345600
    }
  },
  "success": true
}
```

##### Returns Examples

200 example

```
{
  "errors": [
    {
      "code": 7003,
      "message": "No route for the URI",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    "string"
  ],
  "result": {
    "consumers": [
      {
        "consumer_id": "023e105f4ecef8ad9ca31a8372d0c353",
        "created_on": "2019-12-27T18:11:19.117Z",
        "dead_letter_queue": "dead_letter_queue",
        "queue_name": "example-queue",
        "script_name": "my-consumer-worker",
        "settings": {
          "batch_size": 50,
          "max_concurrency": 10,
          "max_retries": 3,
          "max_wait_time_ms": 5000,
          "retry_delay": 10
        },
        "type": "worker"
      }
    ],
    "consumers_total_count": 0,
    "created_on": "created_on",
    "modified_on": "modified_on",
    "producers": [
      {
        "script": "script",
        "type": "worker"
      }
    ],
    "producers_total_count": 0,
    "queue_id": "queue_id",
    "queue_name": "example-queue",
    "settings": {
      "delivery_delay": 5,
      "delivery_paused": true,
      "message_retention_period": 345600
    }
  },
  "success": true
}
```