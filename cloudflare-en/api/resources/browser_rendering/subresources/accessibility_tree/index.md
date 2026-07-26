# Accessibility Tree

## Get accessibility tree page

**post** `/accounts/{account_id}/browser-rendering/accessibilityTree`

Returns the page's accessibility tree. Use `interestingOnly` to only return semantically meaningful nodes; use `root` to scope the tree to a CSS-selector-anchored subtree. Control page loading with `gotoOptions` and `waitFor*` options.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `cacheTTL: optional number`

  Cache TTL default is 5s. Set to 0 to disable.

### Body Parameters

- `actionTimeout: optional number`

  The maximum duration allowed for the browser action to complete after the page has loaded (such as taking screenshots, extracting content, or generating PDFs). If this time limit is exceeded, the action stops and returns a timeout error.

- `addScriptTag: optional array of object { id, content, type, url }`

  Adds a `<script>` tag into the page with the desired URL or content.

  - `id: optional string`

  - `content: optional string`

  - `type: optional string`

  - `url: optional string`

- `addStyleTag: optional array of object { content, url }`

  Adds a `<link rel="stylesheet">` tag into the page with the desired URL or a `<style type="text/css">` tag with the content.

  - `content: optional string`

  - `url: optional string`

- `allowRequestPattern: optional array of string`

  Only allow requests that match the provided regex patterns, eg. '/^.*.(css)'.

- `allowResourceTypes: optional array of "document" or "stylesheet" or "image" or 15 more`

  Only allow requests that match the provided resource types, eg. 'image' or 'script'.

  - `"document"`

  - `"stylesheet"`

  - `"image"`

  - `"media"`

  - `"font"`

  - `"script"`

  - `"texttrack"`

  - `"xhr"`

  - `"fetch"`

  - `"prefetch"`

  - `"eventsource"`

  - `"websocket"`

  - `"manifest"`

  - `"signedexchange"`

  - `"ping"`

  - `"cspviolationreport"`

  - `"preflight"`

  - `"other"`

- `authenticate: optional object { password, username }`

  Provide credentials for HTTP authentication.

  - `password: string`

  - `username: string`

- `bestAttempt: optional boolean`

  Attempt to proceed when 'awaited' events fail or timeout.

- `cookies: optional array of object { name, value, domain, 11 more }`

  Check [options](https://pptr.dev/api/puppeteer.page.setcookie).

  - `name: string`

    Cookie name.

  - `value: string`

  - `domain: optional string`

  - `expires: optional number`

  - `httpOnly: optional boolean`

  - `partitionKey: optional string`

  - `path: optional string`

  - `priority: optional "Low" or "Medium" or "High"`

    - `"Low"`

    - `"Medium"`

    - `"High"`

  - `sameParty: optional boolean`

  - `sameSite: optional "Strict" or "Lax" or "None"`

    - `"Strict"`

    - `"Lax"`

    - `"None"`

  - `secure: optional boolean`

  - `sourcePort: optional number`

  - `sourceScheme: optional "Unset" or "NonSecure" or "Secure"`

    - `"Unset"`

    - `"NonSecure"`

    - `"Secure"`

  - `url: optional string`

- `emulateMediaType: optional string`

- `gotoOptions: optional object { referer, referrerPolicy, timeout, waitUntil }`

  Check [options](https://pptr.dev/api/puppeteer.gotooptions).

  - `referer: optional string`

  - `referrerPolicy: optional string`

  - `timeout: optional number`

  - `waitUntil: optional "load" or "domcontentloaded" or "networkidle0" or "networkidle2" or array of "load" or "domcontentloaded" or "networkidle0" or "networkidle2"`

    - `"load" or "domcontentloaded" or "networkidle0" or "networkidle2"`

      - `"load"`

      - `"domcontentloaded"`

      - `"networkidle0"`

      - `"networkidle2"`

    - `array of "load" or "domcontentloaded" or "networkidle0" or "networkidle2"`

      - `"load"`

      - `"domcontentloaded"`

      - `"networkidle0"`

      - `"networkidle2"`

- `html: optional string`

  Set the content of the page, eg: `<h1>Hello World!!</h1>`. Either `html` or `url` must be set.

- `interestingOnly: optional boolean`

- `rejectRequestPattern: optional array of string`

  Block undesired requests that match the provided regex patterns, eg. '/^.*.(css)'.

- `rejectResourceTypes: optional array of "document" or "stylesheet" or "image" or 15 more`

  Block undesired requests that match the provided resource types, eg. 'image' or 'script'.

  - `"document"`

  - `"stylesheet"`

  - `"image"`

  - `"media"`

  - `"font"`

  - `"script"`

  - `"texttrack"`

  - `"xhr"`

  - `"fetch"`

  - `"prefetch"`

  - `"eventsource"`

  - `"websocket"`

  - `"manifest"`

  - `"signedexchange"`

  - `"ping"`

  - `"cspviolationreport"`

  - `"preflight"`

  - `"other"`

- `root: optional string`

- `setExtraHTTPHeaders: optional map[string]`

- `setJavaScriptEnabled: optional boolean`

- `url: optional string`

  URL to navigate to, eg. `https://example.com`.

- `userAgent: optional string`

- `viewport: optional object { height, width, deviceScaleFactor, 3 more }`

  Check [options](https://pptr.dev/api/puppeteer.page.setviewport).

  - `height: number`

  - `width: number`

  - `deviceScaleFactor: optional number`

  - `hasTouch: optional boolean`

  - `isLandscape: optional boolean`

  - `isMobile: optional boolean`

- `waitForSelector: optional object { selector, hidden, timeout, visible }`

  Wait for the selector to appear in page. Check [options](https://pptr.dev/api/puppeteer.page.waitforselector).

  - `selector: string`

  - `hidden: optional true`

    - `true`

  - `timeout: optional number`

  - `visible: optional true`

    - `true`

- `waitForTimeout: optional number`

  Waits for a specified timeout before continuing.

### Returns

- `meta: object { status, title }`

  - `status: optional number`

  - `title: optional string`

- `result: object { accessibilityTree }`

  - `accessibilityTree: object { role, autocomplete, checked, 23 more }`

    Accessibility tree node

    - `role: string`

    - `autocomplete: optional string`

    - `checked: optional boolean or "mixed"`

      - `boolean`

      - `"mixed"`

        - `"mixed"`

    - `children: optional array of unknown`

    - `description: optional string`

    - `disabled: optional boolean`

    - `expanded: optional boolean`

    - `focused: optional boolean`

    - `haspopup: optional string`

    - `invalid: optional string`

    - `keyshortcuts: optional string`

    - `level: optional number`

    - `modal: optional boolean`

    - `multiline: optional boolean`

    - `multiselectable: optional boolean`

    - `name: optional string`

    - `orientation: optional string`

    - `pressed: optional boolean or "mixed"`

      - `boolean`

      - `"mixed"`

        - `"mixed"`

    - `readonly: optional boolean`

    - `required: optional boolean`

    - `roledescription: optional string`

    - `selected: optional boolean`

    - `value: optional string or number`

      - `string`

      - `number`

    - `valuemax: optional number`

    - `valuemin: optional number`

    - `valuetext: optional string`

- `success: boolean`

  Response status.

- `errors: optional array of object { code, message }`

  - `code: number`

    Error code.

  - `message: string`

    Error message.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/browser-rendering/accessibilityTree \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "html": "<h1>Hello World!</h1>",
          "url": "https://www.example.com/"
        }'
```

#### Response

```json
{
  "meta": {
    "status": 0,
    "title": "title"
  },
  "result": {
    "accessibilityTree": {
      "role": "role",
      "autocomplete": "autocomplete",
      "checked": "mixed",
      "children": [
        {}
      ],
      "description": "description",
      "disabled": true,
      "expanded": true,
      "focused": true,
      "haspopup": "haspopup",
      "invalid": "invalid",
      "keyshortcuts": "keyshortcuts",
      "level": 0,
      "modal": true,
      "multiline": true,
      "multiselectable": true,
      "name": "name",
      "orientation": "orientation",
      "pressed": "mixed",
      "readonly": true,
      "required": true,
      "roledescription": "roledescription",
      "selected": true,
      "value": "string",
      "valuemax": 0,
      "valuemin": 0,
      "valuetext": "valuetext"
    }
  },
  "success": true,
  "errors": [
    {
      "code": 0,
      "message": "message"
    }
  ]
}
```

## Domain Types

### Accessibility Tree Create Response

- `AccessibilityTreeCreateResponse object { accessibilityTree }`

  - `accessibilityTree: object { role, autocomplete, checked, 23 more }`

    Accessibility tree node

    - `role: string`

    - `autocomplete: optional string`

    - `checked: optional boolean or "mixed"`

      - `boolean`

      - `"mixed"`

        - `"mixed"`

    - `children: optional array of unknown`

    - `description: optional string`

    - `disabled: optional boolean`

    - `expanded: optional boolean`

    - `focused: optional boolean`

    - `haspopup: optional string`

    - `invalid: optional string`

    - `keyshortcuts: optional string`

    - `level: optional number`

    - `modal: optional boolean`

    - `multiline: optional boolean`

    - `multiselectable: optional boolean`

    - `name: optional string`

    - `orientation: optional string`

    - `pressed: optional boolean or "mixed"`

      - `boolean`

      - `"mixed"`

        - `"mixed"`

    - `readonly: optional boolean`

    - `required: optional boolean`

    - `roledescription: optional string`

    - `selected: optional boolean`

    - `value: optional string or number`

      - `string`

      - `number`

    - `valuemax: optional number`

    - `valuemin: optional number`

    - `valuetext: optional string`
