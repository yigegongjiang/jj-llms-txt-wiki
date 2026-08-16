---
description: REST API and TypeScript SDK examples for deploying Workers programmatically.
title: API examples
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/cloudflare-for-platforms/llms.txt  
> Use this file to discover all available pages before exploring further.

# API examples

Last updated May 5, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/reference/platform-examples/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The following examples show how to use Cloudflare's REST API and TypeScript SDK to deploy and manage Workers programmatically.

### Prerequisites

Before using these examples, you need:

* Your **Account ID** \- Found in the Cloudflare dashboard URL or API settings
* A **dispatch namespace** \- Created via the [dashboard](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/get-started/)
* An **API token** with Workers permissions - Create one at [API Tokens ↗](https://dash.cloudflare.com/profile/api-tokens)

For SDK examples, install the Cloudflare SDK:

```sh
npm install cloudflare
```

### Deploy a user Worker

Upload a Worker script to your dispatch namespace. This is the primary operation your platform performs when customers deploy code.

```bash
# First, create the worker script file
cat > worker.mjs << 'EOF'
export default {
  async fetch(request, env, ctx) {
    return new Response("Hello from user Worker!");
  },
};
EOF

# Deploy using multipart form (required for ES modules)
curl -X PUT "https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/workers/dispatch/namespaces/$NAMESPACE_NAME/scripts/$SCRIPT_NAME" \
  -H "Authorization: Bearer $API_TOKEN" \
  -F 'metadata={"main_module": "worker.mjs"};type=application/json' \
  -F 'worker.mjs=@worker.mjs;type=application/javascript+module'
```

```typescript
import Cloudflare from "cloudflare";

const client = new Cloudflare({
	apiToken: process.env.API_TOKEN,
});

async function deployUserWorker(
	accountId: string,
	namespace: string,
	scriptName: string,
	scriptContent: string,
) {
	const scriptFile = new File([scriptContent], `${scriptName}.mjs`, {
		type: "application/javascript+module",
	});

	const result =
		await client.workersForPlatforms.dispatch.namespaces.scripts.update(
			namespace,
			scriptName,
			{
				account_id: accountId,
				metadata: {
					main_module: `${scriptName}.mjs`,
				},
				files: [scriptFile],
			},
		);

	return result;
}

// Usage
await deployUserWorker(
	"your-account-id",
	"production",
	"customer-123",
	`export default {
  async fetch(request, env, ctx) {
    return new Response("Hello from customer 123!");
  },
};`,
);
```

### Deploy with bindings and tags

Use [bindings](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/configuration/bindings/) to give each user Worker its own resources like a KV store or database. Use [tags](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/configuration/tags/) to organize Workers by customer ID, project ID, or plan type for bulk operations.

The following example shows how to deploy a Worker with its own KV namespace and tags attached:

```bash
curl -X PUT "https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/workers/dispatch/namespaces/$NAMESPACE_NAME/scripts/$SCRIPT_NAME" \
  -H "Authorization: Bearer $API_TOKEN" \
  -F 'metadata={"main_module": "worker.mjs", "bindings": [{"type": "kv_namespace", "name": "MY_KV", "namespace_id": "your-kv-namespace-id"}], "tags": ["customer-123", "production", "pro-plan"], "compatibility_date": "2024-01-01"};type=application/json' \
  -F 'worker.mjs=@worker.mjs;type=application/javascript+module'
```

```typescript
import Cloudflare from "cloudflare";

const client = new Cloudflare({
	apiToken: process.env.API_TOKEN,
});

async function deployWorkerWithBindingsAndTags(
	accountId: string,
	namespace: string,
	scriptName: string,
	scriptContent: string,
	kvNamespaceId: string,
	tags: string[],
) {
	const scriptFile = new File([scriptContent], `${scriptName}.mjs`, {
		type: "application/javascript+module",
	});

	const result =
		await client.workersForPlatforms.dispatch.namespaces.scripts.update(
			namespace,
			scriptName,
			{
				account_id: accountId,
				metadata: {
					main_module: `${scriptName}.mjs`,
					compatibility_date: "2024-01-01",
					bindings: [
						{
							type: "kv_namespace",
							name: "MY_KV",
							namespace_id: kvNamespaceId,
						},
					],
					tags: tags, // e.g., ["customer-123", "production", "pro-plan"]
				},
				files: [scriptFile],
			},
		);

	return result;
}

// Usage
const scriptContent = `export default {
  async fetch(request, env, ctx) {
    const value = await env.MY_KV.get("key") || "default";
    return new Response(value);
  },
};`;

await deployWorkerWithBindingsAndTags(
	"your-account-id",
	"production",
	"customer-123-app",
	scriptContent,
	"kv-namespace-id",
	["customer-123", "production", "pro-plan"],
);
```

For more information, refer to [Bindings](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/configuration/bindings/) and [Tags](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/configuration/tags/).

### Deploy a Worker with static assets

Deploy a Worker that serves static files (HTML, CSS, JavaScript, images). This is a three-step process:

1. Create an upload session with a manifest of files
2. Upload the asset files
3. Deploy the Worker with the assets binding

For more details on static assets configuration and options, refer to [Static assets](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/configuration/static-assets/).

**Step 1: Create upload session**

```bash
curl -X POST "https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/workers/dispatch/namespaces/$NAMESPACE_NAME/scripts/$SCRIPT_NAME/assets-upload-session" \
  -H "Authorization: Bearer $API_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "manifest": {
      "/index.html": {
        "hash": "<sha256-hash-first-16-bytes-hex>",
        "size": 1234
      },
      "/styles.css": {
        "hash": "<sha256-hash-first-16-bytes-hex>",
        "size": 567
      }
    }
  }'
```

The response includes a `jwt` token and `buckets` array indicating which files need uploading.

**Step 2: Upload assets**

```bash
curl -X POST "https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/workers/assets/upload?base64=true" \
  -H "Authorization: Bearer $JWT_FROM_STEP_1" \
  -F '<hash1>=<base64-encoded-content>' \
  -F '<hash2>=<base64-encoded-content>'
```

**Step 3: Deploy Worker with assets**

```bash
curl -X PUT "https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/workers/dispatch/namespaces/$NAMESPACE_NAME/scripts/$SCRIPT_NAME" \
  -H "Authorization: Bearer $API_TOKEN" \
  -F 'metadata={"main_module": "worker.mjs", "assets": {"jwt": "<completion-token>"}, "bindings": [{"type": "assets", "name": "ASSETS"}]};type=application/json' \
  -F 'worker.mjs=export default { async fetch(request, env) { return env.ASSETS.fetch(request); } };type=application/javascript+module'
```

```typescript
interface AssetFile {
	path: string; // e.g., "/index.html"
	content: string; // base64 encoded content
	size: number; // file size in bytes
}

async function hashContent(base64Content: string): Promise<string> {
	const binaryString = atob(base64Content);
	const bytes = new Uint8Array(binaryString.length);
	for (let i = 0; i < binaryString.length; i++) {
		bytes[i] = binaryString.charCodeAt(i);
	}
	const hashBuffer = await crypto.subtle.digest("SHA-256", bytes);
	const hashArray = Array.from(new Uint8Array(hashBuffer));
	// Use first 16 bytes (32 hex chars) per API requirement
	return hashArray
		.slice(0, 16)
		.map((b) => b.toString(16).padStart(2, "0"))
		.join("");
}

async function deployWorkerWithAssets(
	accountId: string,
	namespace: string,
	scriptName: string,
	assets: AssetFile[],
) {
	const apiToken = process.env.API_TOKEN;
	const baseUrl = `https://api.cloudflare.com/client/v4/accounts/${accountId}/workers`;

	// Step 1: Build manifest
	const manifest: Record<string, { hash: string; size: number }> = {};
	const hashToAsset = new Map<string, AssetFile>();

	for (const asset of assets) {
		const hash = await hashContent(asset.content);
		const path = asset.path.startsWith("/") ? asset.path : "/" + asset.path;
		manifest[path] = { hash, size: asset.size };
		hashToAsset.set(hash, asset);
	}

	// Step 2: Create upload session
	const sessionResponse = await fetch(
		`${baseUrl}/dispatch/namespaces/${namespace}/scripts/${scriptName}/assets-upload-session`,
		{
			method: "POST",
			headers: {
				Authorization: `Bearer ${apiToken}`,
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ manifest }),
		},
	);

	const sessionData = (await sessionResponse.json()) as {
		success: boolean;
		result?: { jwt: string; buckets?: string[][] };
	};

	if (!sessionData.success || !sessionData.result) {
		throw new Error("Failed to create upload session");
	}

	let completionToken = sessionData.result.jwt;
	const buckets = sessionData.result.buckets;

	// Step 3: Upload assets in buckets
	if (buckets && buckets.length > 0) {
		for (const bucket of buckets) {
			const formData = new FormData();
			for (const hash of bucket) {
				const asset = hashToAsset.get(hash);
				if (asset) {
					formData.append(hash, asset.content);
				}
			}

			const uploadResponse = await fetch(
				`${baseUrl}/assets/upload?base64=true`,
				{
					method: "POST",
					headers: { Authorization: `Bearer ${completionToken}` },
					body: formData,
				},
			);

			const uploadData = (await uploadResponse.json()) as {
				success: boolean;
				result?: { jwt?: string };
			};

			if (uploadData.result?.jwt) {
				completionToken = uploadData.result.jwt;
			}
		}
	}

	// Step 4: Deploy worker with assets binding
	const workerCode = `
export default {
  async fetch(request, env) {
    return env.ASSETS.fetch(request);
  }
};`;

	const deployFormData = new FormData();
	const metadata = {
		main_module: `${scriptName}.mjs`,
		assets: { jwt: completionToken },
		bindings: [{ type: "assets", name: "ASSETS" }],
	};

	deployFormData.append(
		"metadata",
		new Blob([JSON.stringify(metadata)], { type: "application/json" }),
	);
	deployFormData.append(
		`${scriptName}.mjs`,
		new Blob([workerCode], { type: "application/javascript+module" }),
	);

	const deployResponse = await fetch(
		`${baseUrl}/dispatch/namespaces/${namespace}/scripts/${scriptName}`,
		{
			method: "PUT",
			headers: { Authorization: `Bearer ${apiToken}` },
			body: deployFormData,
		},
	);

	return deployResponse.json();
}

// Usage
await deployWorkerWithAssets("your-account-id", "production", "customer-site", [
	{
		path: "/index.html",
		content: btoa("<html><body>Hello World</body></html>"),
		size: 37,
	},
	{
		path: "/styles.css",
		content: btoa("body { font-family: sans-serif; }"),
		size: 33,
	},
]);
```

### List Workers in a namespace

Retrieve all user Workers deployed to a namespace.

```bash
curl "https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/workers/dispatch/namespaces/$NAMESPACE_NAME/scripts" \
  -H "Authorization: Bearer $API_TOKEN"
```

```typescript
async function listWorkers(accountId: string, namespace: string) {
	const response = await fetch(
		`https://api.cloudflare.com/client/v4/accounts/${accountId}/workers/dispatch/namespaces/${namespace}/scripts`,
		{
			headers: {
				Authorization: `Bearer ${process.env.API_TOKEN}`,
			},
		},
	);

	const data = (await response.json()) as {
		success: boolean;
		result: Array<{ id: string; tags?: string[] }>;
	};

	return data.result;
}

// Usage
const workers = await listWorkers("your-account-id", "production");
console.log(workers);
```

### Delete Workers by tag

Delete all Workers matching a tag filter. This is useful when a customer deletes their account and you need to remove all their Workers at once.

Delete all Workers tagged with `customer-123`:

```bash
curl -X DELETE "https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/workers/dispatch/namespaces/$NAMESPACE_NAME/scripts?tags=customer-123:yes" \
  -H "Authorization: Bearer $API_TOKEN"
```

```typescript
async function deleteWorkersByTag(
	accountId: string,
	namespace: string,
	tag: string,
) {
	const response = await fetch(
		`https://api.cloudflare.com/client/v4/accounts/${accountId}/workers/dispatch/namespaces/${namespace}/scripts?tags=${tag}:yes`,
		{
			method: "DELETE",
			headers: {
				Authorization: `Bearer ${process.env.API_TOKEN}`,
			},
		},
	);

	return response.json();
}

// Usage: Delete all Workers for a customer
await deleteWorkersByTag("your-account-id", "production", "customer-123");
```

### Delete a single Worker

Delete a specific Worker by name.

```bash
curl -X DELETE "https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/workers/dispatch/namespaces/$NAMESPACE_NAME/scripts/$SCRIPT_NAME" \
  -H "Authorization: Bearer $API_TOKEN"
```

```typescript
import Cloudflare from "cloudflare";

const client = new Cloudflare({
	apiToken: process.env.API_TOKEN,
});

async function deleteWorker(
	accountId: string,
	namespace: string,
	scriptName: string,
) {
	const result =
		await client.workersForPlatforms.dispatch.namespaces.scripts.delete(
			namespace,
			scriptName,
			{ account_id: accountId },
		);

	return result;
}

// Usage
await deleteWorker("your-account-id", "production", "customer-123");
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/reference/platform-examples/#page","headline":"API examples · Cloudflare for Platforms docs","description":"REST API and TypeScript SDK examples for deploying Workers programmatically.","url":"https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/reference/platform-examples/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-05-05","publisher":{"@type":"Organization","name":"Cloudflare","url":"https://www.cloudflare.com/"},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"},"keywords":["REST API","TypeScript"]}
```
