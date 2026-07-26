## Delete a token

**delete** `/accounts/{account_id}/ai-search/tokens/{id}`

Permanently delete a stored AI Search credential. Credentials in use by an instance cannot be deleted.

### Path Parameters

- `account_id: string`

- `id: string`

### Returns

- `result: unknown`

- `success: true`

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/ai-search/tokens/$ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {},
  "success": true
}
```
