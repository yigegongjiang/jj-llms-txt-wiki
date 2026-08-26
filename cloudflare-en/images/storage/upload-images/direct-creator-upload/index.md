---
description: Use Cloudflare Images Direct Creator Upload to let users upload images with a one-time URL without exposing your API credentials.
title: Accept user-uploaded images
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/images/llms.txt  
> Use this file to discover all available pages before exploring further.

# Accept user-uploaded images

Last updated Apr 21, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/images/storage/upload-images/direct-creator-upload/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

The Direct Creator Upload feature in Cloudflare Images lets your users upload images with a one-time upload URL without exposing your API key or token to the client. Using a direct creator upload also eliminates the need for an intermediary storage bucket and the storage/egress costs associated with it.

You can set up [webhooks](https://developers.cloudflare.com/images/storage/upload-images/configure-webhooks/) to receive notifications on your direct creator upload workflow.

## Request a one-time upload URL

Make a `POST` request to the `direct_upload` endpoint using the example below as reference.

Note

The `metadata` included in the request is never shared with end-users.

```bash
curl --request POST \
https://api.cloudflare.com/client/v4/accounts/{account_id}/images/v2/direct_upload \
--header "Authorization: Bearer <API_TOKEN>" \
--form 'requireSignedURLs=true' \
--form 'metadata={"key":"value"}'
```

After a successful request, you will receive a response similar to the example below. The `id` field is a future image identifier that will be uploaded by a creator.

```json
{
	"result": {
		"id": "2cdc28f0-017a-49c4-9ed7-87056c83901",
		"uploadURL": "https://upload.imagedelivery.net/Vi7wi5KSItxGFsWRG2Us6Q/2cdc28f0-017a-49c4-9ed7-87056c83901"
	},
	"result_info": null,
	"success": true,
	"errors": [],
	"messages": []
}
```

After calling the endpoint, a new draft image record is created, but the image will not appear in the list of images. If you want to check the status of the image record, you can make a request to the one-time upload URL using the `direct_upload` endpoint.

## Check the image record status

To check the status of a new draft image record, use the one-time upload URL as shown in the example below.

```bash
curl https://api.cloudflare.com/client/v4/accounts/{account_id}/images/v1/{image_id} \
--header "Authorization: Bearer <API_TOKEN>"
```

After a successful request, you should receive a response similar to the example below. The `draft` field is set to `true` until a creator uploads an image. After an image is uploaded, the draft field is removed.

```json
{
	"result": {
		"id": "2cdc28f0-017a-49c4-9ed7-87056c83901",
		"metadata": {
			"key": "value"
		},
		"uploaded": "2022-01-31T16:39:28.458Z",
		"requireSignedURLs": true,
		"variants": [
			"https://imagedelivery.net/Vi7wi5KSItxGFsWRG2Us6Q/2cdc28f0-017a-49c4-9ed7-87056c83901/public",
			"https://imagedelivery.net/Vi7wi5KSItxGFsWRG2Us6Q/2cdc28f0-017a-49c4-9ed7-87056c83901/thumbnail"
		],
		"draft": true
	},
	"success": true,
	"errors": [],
	"messages": []
}
```

The backend endpoint should return the `uploadURL` property to the client, which uploads the image without needing to pass any authentication information with it.

Below is an example of an HTML page that takes a one-time upload URL and uploads any image the user selects.

```html
<!DOCTYPE html>
<html>
	<body>
		<form
			action="INSERT_UPLOAD_URL_HERE"
			method="post"
			enctype="multipart/form-data"
		>
			<input type="file" id="myFile" name="file" />
			<input type="submit" />
		</form>
	</body>
</html>
```

By default, the `uploadURL` expires after 30 minutes if unused. To override this option, add the following argument to the cURL command:

```txt
--data '{"expiry":"2021-09-14T16:00:00Z"}'
```

The expiry value must be a minimum of two minutes and maximum of six hours in the future.

## Direct Creator Upload with custom ID

You can specify a [custom ID](https://developers.cloudflare.com/images/storage/upload-images/upload-custom-path/) when you first request a one-time upload URL, instead of using the automatically generated ID for your image. Note that images with a custom ID cannot be made private with the [signed URL tokens](https://developers.cloudflare.com/images/optimization/hosted-images/serve-private-images/) feature (`--requireSignedURLs=true`).

To specify a custom ID, pass a form field with the name ID and corresponding custom ID value as shown in the example below.

```txt
--form 'id=this/is/my-customid'
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"TechArticle","@id":"https://developers.cloudflare.com/images/storage/upload-images/direct-creator-upload/#page","headline":"Accept user-uploaded images · Cloudflare Images docs","description":"Use Cloudflare Images Direct Creator Upload to let users upload images with a one-time URL without exposing your API credentials.","url":"https://developers.cloudflare.com/images/storage/upload-images/direct-creator-upload/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-21","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
