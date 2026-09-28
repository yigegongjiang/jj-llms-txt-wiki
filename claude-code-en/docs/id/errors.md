> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referensi kesalahan

> Cari pesan kesalahan runtime Claude Code dengan arti masing-masing dan cara memperbaikinya.

Halaman ini mencantumkan kesalahan runtime yang ditampilkan Claude Code dan cara memulihkan dari masing-masing, ditambah apa yang harus diperiksa ketika respons tampak tidak normal tanpa kesalahan. Untuk kesalahan instalasi seperti `command not found` atau kegagalan TLS selama penyiapan, lihat [Troubleshoot installation and login](/docs/id/troubleshoot-install).

Kecuali untuk [Wrapper and IDE errors](#wrapper-and-ide-errors), yang dicetak oleh program peluncur daripada Claude Code itu sendiri, kesalahan dan perintah pemulihan ini berlaku di seluruh CLI, [Desktop app](/docs/id/desktop), dan [cloud sessions](/docs/id/claude-code-on-the-web), karena ketiganya membungkus CLI Claude Code yang sama. Untuk masalah spesifik permukaan lainnya, lihat bagian pemecahan masalah di halaman permukaan tersebut.

<Note>
  Claude Code memanggil Claude API untuk respons model, jadi sebagian besar kesalahan runtime memetakan ke kode kesalahan API yang mendasar. Halaman ini mencakup apa arti setiap kesalahan di dalam Claude Code dan cara memulihkan. Untuk definisi kode status HTTP mentah, lihat [Claude Platform error reference](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Temukan kesalahan Anda
</h2>

Cocokkan pesan yang Anda lihat dengan bagian di bawah ini.

| Pesan                                                                                                                                                                                                                                                                | Bagian                                                                                                                      |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| `API Error: 500 Internal server error`                                                                                                                                                                                                                               | [Server errors](#api-error-500-internal-server-error)                                                                       |
| `API Error: Repeated 529 Overloaded errors`                                                                                                                                                                                                                          | [Server errors](#api-error-repeated-529-overloaded-errors)                                                                  |
| `Request timed out`                                                                                                                                                                                                                                                  | [Server errors](#request-timed-out), atau [Network](#unable-to-connect-to-api) jika pesan menyebutkan koneksi internet Anda |
| `API Error: No response from API`                                                                                                                                                                                                                                    | [Server errors](#no-response-from-api)                                                                                      |
| `Server error mid-response. The response above may be incomplete.`                                                                                                                                                                                                   | [Server errors](#the-response-above-may-be-incomplete)                                                                      |
| `Connection lost mid-response` / `Your computer went to sleep mid-response` / `The response stopped arriving`                                                                                                                                                        | [Server errors](#the-response-above-may-be-incomplete)                                                                      |
| `Connection closed mid-response` / `Response stalled mid-stream`                                                                                                                                                                                                     | [Server errors](#the-response-above-may-be-incomplete)                                                                      |
| `Connection lost before a response was produced` / `Your computer went to sleep before a response was produced` / `The response stalled before a response was produced`                                                                                              | [Automatic retries](#automatic-retries)                                                                                     |
| `Connection closed while thinking` / `Response stalled while thinking`                                                                                                                                                                                               | [Automatic retries](#automatic-retries)                                                                                     |
| `Connection lost while your computer was asleep`                                                                                                                                                                                                                     | [Automatic retries](#automatic-retries)                                                                                     |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`                                                                                                                                                                                 | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                        |
| `Auto mode could not evaluate this action and is blocking it for safety`                                                                                                                                                                                             | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                        |
| `Auto mode classifier transcript exceeded context window`                                                                                                                                                                                                            | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                        |
| `Agent aborted: auto mode classifier request refused by the safety safeguard`                                                                                                                                                                                        | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                        |
| `The server-side auto mode classifier gave no verdict`                                                                                                                                                                                                               | [Server errors](#the-server-returned-no-safety-verdict)                                                                     |
| `Auto mode is unavailable — the server returned no safety verdict for the last 10 responses`                                                                                                                                                                         | [Server errors](#the-server-returned-no-safety-verdict)                                                                     |
| `Agent terminated early due to an API error`                                                                                                                                                                                                                         | [Server errors](#agent-terminated-early-due-to-an-api-error)                                                                |
| `You've hit your session limit` / `You've hit your weekly limit` / `You've hit your Opus limit` / `You've hit your Sonnet limit`                                                                                                                                     | [Usage limits](#youve-hit-your-session-limit)                                                                               |
| `Usage credits required for 1M context`                                                                                                                                                                                                                              | [Usage limits](#usage-credits-required-for-1m-context)                                                                      |
| `the prompt to confirm went unanswered — nothing was sent`                                                                                                                                                                                                           | [Usage limits](#the-prompt-to-confirm-went-unanswered)                                                                      |
| `Server is temporarily limiting requests`                                                                                                                                                                                                                            | [Usage limits](#server-is-temporarily-limiting-requests)                                                                    |
| `Request rejected (429)`                                                                                                                                                                                                                                             | [Usage limits](#request-rejected-429)                                                                                       |
| `Credit balance is too low`                                                                                                                                                                                                                                          | [Usage limits](#credit-balance-is-too-low)                                                                                  |
| `You've hit your monthly spend limit` / `You've hit your individual spend limit` / `You've hit your org's monthly spend limit` / `You've hit your channel's monthly spend limit` / `You've hit your team's shared budget` / `You've hit your individual usage limit` | [Usage limits](#youve-hit-your-monthly-spend-limit)                                                                         |
| `Could not update your spend limit`                                                                                                                                                                                                                                  | [Usage limits](#could-not-update-your-spend-limit)                                                                          |
| `spend limit reached` / `spend limit unavailable`                                                                                                                                                                                                                    | [Usage limits](#spend-limit-reached)                                                                                        |
| `Not logged in · Please run /login`                                                                                                                                                                                                                                  | [Authentication](#not-logged-in)                                                                                            |
| `Could not resolve authentication method`                                                                                                                                                                                                                            | [Authentication](#could-not-resolve-authentication-method)                                                                  |
| `Invalid API key`                                                                                                                                                                                                                                                    | [Authentication](#invalid-api-key)                                                                                          |
| `Your apiKeyHelper script is failing`                                                                                                                                                                                                                                | [Authentication](#your-apikeyhelper-script-is-failing)                                                                      |
| `Invalid auth token · Fix external auth token`                                                                                                                                                                                                                       | [Authentication](#invalid-request-header-value)                                                                             |
| `Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable`                                                                                                                                                                                                    | [Authentication](#invalid-request-header-value)                                                                             |
| `Invalid request header from the environment · Fix the environment variable`                                                                                                                                                                                         | [Authentication](#invalid-request-header-value)                                                                             |
| `This organization has been disabled`                                                                                                                                                                                                                                | [Authentication](#this-organization-has-been-disabled)                                                                      |
| `Your organization has disabled API key authentication`                                                                                                                                                                                                              | [Authentication](#your-organization-has-disabled-api-key-authentication)                                                    |
| `Your organization has disabled Claude subscription access`                                                                                                                                                                                                          | [Authentication](#your-organization-has-disabled-claude-subscription-access)                                                |
| `Routines are disabled by your organization's policy`                                                                                                                                                                                                                | [Authentication](#routines-are-disabled-by-your-organizations-policy)                                                       |
| `Remote Control is only available when using Claude via api.anthropic.com`                                                                                                                                                                                           | [Authentication](#remote-control-requires-the-anthropic-api)                                                                |
| `OAuth token refresh failed — run /login to re-authenticate`                                                                                                                                                                                                         | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                |
| `JWT refresh failed: no OAuth token — run /login`                                                                                                                                                                                                                    | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                |
| `Claude.ai login expired`                                                                                                                                                                                                                                            | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                |
| `Claude.ai login was rejected — run /login, then /remote-control`                                                                                                                                                                                                    | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                |
| `OAuth token unavailable — run /login to restore Remote Control`                                                                                                                                                                                                     | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                |
| `Signed out of Claude — run /login, then /remote-control`                                                                                                                                                                                                            | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                |
| `signed-in claude.ai account or organization changed on this machine`                                                                                                                                                                                                | [Authentication](#remote-control-stopped-because-the-signed-in-account-changed)                                             |
| `Remote Control stopped — the app running this session is now signed in to a different Claude account`                                                                                                                                                               | [Authentication](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)               |
| `Remote Control stopped — the app running this session is signed out of Claude`                                                                                                                                                                                      | [Authentication](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)               |
| `OAuth token revoked` / `OAuth token has expired`                                                                                                                                                                                                                    | [Authentication](#oauth-token-revoked-or-expired)                                                                           |
| `API Error: 401 Invalid authentication credentials`                                                                                                                                                                                                                  | [Authentication](#api-error-401-invalid-authentication-credentials)                                                         |
| `Login expired · Please run /login`                                                                                                                                                                                                                                  | [Authentication](#login-expired)                                                                                            |
| `Claude login not accepted · Run /login, then try again`                                                                                                                                                                                                             | [Authentication](#claude-login-not-accepted)                                                                                |
| `Artifacts need a claude.ai login`                                                                                                                                                                                                                                   | [Authentication](#artifacts-need-a-claude-ai-login)                                                                         |
| `Not signed in to the Cloud gateway — run /login.`                                                                                                                                                                                                                   | [Authentication](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                    |
| `Administrator policy requires a Cloud gateway sign-in on this machine`                                                                                                                                                                                              | [Authentication](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                    |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                                                                                                                                                                                           | [Authentication](#login-expired)                                                                                            |
| `Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                            | [Authentication](#your-account-is-on-hold)                                                                                  |
| `Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                     | [Authentication](#your-account-is-on-hold)                                                                                  |
| `Anthropic profile login expired · Re-authenticate your Anthropic profile`                                                                                                                                                                                           | [Authentication](#anthropic-profile-login-expired)                                                                          |
| `Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile`                                                                                                                                                 | [Authentication](#anthropic-profile-login-expired)                                                                          |
| `does not meet scope requirement user:profile`                                                                                                                                                                                                                       | [Authentication](#oauth-scope-requirement)                                                                                  |
| `claude.ai rejected the session token` / `session token rejected`                                                                                                                                                                                                    | [Authentication](#claude-ai-rejected-the-session-token)                                                                     |
| `MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)`                                                                                                                                                                                       | [Authentication](#mcp-server-needs-you-to-sign-in-again)                                                                    |
| `rejected the credential from its headersHelper` / `rejected the Authorization header in its config`                                                                                                                                                                 | [Authentication](#mcp-server-needs-you-to-sign-in-again)                                                                    |
| `MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate`                                                                                                                                                                  | [Authentication](#mcp-server-needs-you-to-sign-in-again)                                                                    |
| `MCP server "<name>" requires re-authorization (token expired)`                                                                                                                                                                                                      | [Authentication](#mcp-server-needs-you-to-sign-in-again)                                                                    |
| `Issuer mismatch in authorization response (RFC 9207)`                                                                                                                                                                                                               | [Authentication](#issuer-mismatch-in-authorization-response)                                                                |
| `Cloud gateway session expired — run /login to reconnect.`                                                                                                                                                                                                           | [Authentication](#cloud-gateway-session-expired)                                                                            |
| `Cloud gateway <url> no longer accepts this session`                                                                                                                                                                                                                 | [Authentication](#cloud-gateway-session-expired)                                                                            |
| `Sign-in timed out while waiting for you to continue. Try again.`                                                                                                                                                                                                    | [Authentication](#sign-in-timed-out-while-waiting-for-you-to-continue)                                                      |
| `AWS credentials expired or invalid`                                                                                                                                                                                                                                 | [Authentication](#aws-credentials-expired-or-invalid)                                                                       |
| `AWS authentication failed`                                                                                                                                                                                                                                          | [Authentication](#aws-authentication-failed)                                                                                |
| `Google Cloud credentials expired or invalid`                                                                                                                                                                                                                        | [Authentication](#google-cloud-credentials-expired-or-invalid)                                                              |
| `Google Cloud authentication failed`                                                                                                                                                                                                                                 | [Authentication](#google-cloud-authentication-failed)                                                                       |
| `Microsoft Foundry authentication failed`                                                                                                                                                                                                                            | [Authentication](#microsoft-foundry-authentication-failed)                                                                  |
| `Gateway refused the request`                                                                                                                                                                                                                                        | [Authentication](#gateway-refused-the-request)                                                                              |
| `Could not load AWS credentials` / `Could not load Google Cloud credentials`                                                                                                                                                                                         | [Authentication](#could-not-load-aws-or-google-cloud-credentials)                                                           |
| `AWS default-chain credential resolve timed out`                                                                                                                                                                                                                     | [Authentication](#aws-default-chain-credential-resolve-timed-out)                                                           |
| `Timed out after 60s waiting for AWS`                                                                                                                                                                                                                                | [Authentication](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                     |
| `A request to AWS timed out. Check your network and proxy settings, then try again.`                                                                                                                                                                                 | [Authentication](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                     |
| `Could not load the default credentials` pada Google Cloud's Agent Platform                                                                                                                                                                                          | [Authentication](#could-not-load-aws-or-google-cloud-credentials)                                                           |
| `Unable to connect to API`                                                                                                                                                                                                                                           | [Network](#unable-to-connect-to-api)                                                                                        |
| `Connection refused —` / `Can't reach the API server —` / `No internet route —` / `Couldn't connect through your proxy` / `Connection dropped`, masing-masing dengan kode kesalahan dalam tanda kurung                                                               | [Network](#unable-to-connect-to-api)                                                                                        |
| `Unable to connect to Anthropic services` selama penyiapan                                                                                                                                                                                                           | [Network](#unable-to-connect-to-anthropic-services)                                                                         |
| `Socket is closed`                                                                                                                                                                                                                                                   | [Network](#socket-is-closed)                                                                                                |
| `Waiting for API response · will retry in`                                                                                                                                                                                                                           | [Automatic retries](#automatic-retries), atau [Network](#unable-to-connect-to-api) jika terus berlanjut                     |
| `API returned an empty or malformed response`                                                                                                                                                                                                                        | [Network](#api-returned-an-empty-or-malformed-response)                                                                     |
| `Streaming response ended before any complete data was received`                                                                                                                                                                                                     | [Network](#streaming-response-ended-before-any-complete-data-was-received)                                                  |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"`                                                                                                                                                                   | [Network](#bedrock-streaming-response-has-an-unexpected-content-type)                                                       |
| `SSL certificate verification failed`                                                                                                                                                                                                                                | [Network](#ssl-certificate-errors)                                                                                          |
| `SSL certificate error (...)` selama login atau startup                                                                                                                                                                                                              | [Network](#ssl-certificate-errors)                                                                                          |
| `unable to get local issuer certificate`                                                                                                                                                                                                                             | [Network](#ssl-certificate-errors)                                                                                          |
| `403` dengan `x-deny-reason: host_not_allowed` dalam sesi cloud atau routine                                                                                                                                                                                         | [Network](#host-not-allowed-in-a-cloud-session)                                                                             |
| `proxy refused the connection`                                                                                                                                                                                                                                       | [Network](#the-proxy-refused-the-connection)                                                                                |
| `403` dengan `This GraphQL query is not enabled for this session` dalam sesi cloud                                                                                                                                                                                   | [GitHub proxy](/docs/id/cloud-environments#github-proxy)                                                                         |
| `The cloud environments service returned an empty response` / `The cloud environments service returned a response in an unexpected format`                                                                                                                           | [Network](#the-cloud-environments-service-returned-an-empty-or-unexpected-response)                                         |
| `Couldn't reconnect to your Remote Control session`                                                                                                                                                                                                                  | [Network](#couldnt-reconnect-to-your-remote-control-session)                                                                |
| `N sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.`                                                                                                                                               | [Network](#sessions-ended-while-this-machine-was-offline)                                                                   |
| `Couldn't share the transcript.`                                                                                                                                                                                                                                     | [Network](#couldnt-share-the-transcript)                                                                                    |
| `Prompt is too long` / `Input is too long for requested model`                                                                                                                                                                                                       | [Request errors](#prompt-is-too-long)                                                                                       |
| `Prompt is too long · automatic compaction failed:`                                                                                                                                                                                                                  | [Request errors](#prompt-is-too-long)                                                                                       |
| `Prompt is too long · this conversation is a single exchange` / `A single-exchange conversation cannot be compacted`                                                                                                                                                 | [Request errors](#prompt-is-too-long)                                                                                       |
| `Context limit reached · /compact or /clear to continue`                                                                                                                                                                                                             | [Request errors](#prompt-is-too-long)                                                                                       |
| `Context limit reached · /clear to continue`                                                                                                                                                                                                                         | [Request errors](#prompt-is-too-long)                                                                                       |
| `capability_rejected: prompt_too_long` pada sesi gateway aplikasi Claude                                                                                                                                                                                             | [Request errors](#prompt-is-too-long)                                                                                       |
| `upstream rejected the request` / `request too large for this upstream` pada sesi gateway aplikasi Claude                                                                                                                                                            | [Upstream error messages](/docs/id/claude-apps-gateway-config#upstream-error-messages)                                           |
| `upstream rate limit exceeded` pada sesi gateway aplikasi Claude                                                                                                                                                                                                     | [Upstream error messages](/docs/id/claude-apps-gateway-config#upstream-error-messages)                                           |
| `all upstreams failed (N attempted)` pada sesi gateway aplikasi Claude                                                                                                                                                                                               | [Upstream error messages](/docs/id/claude-apps-gateway-config#upstream-error-messages)                                           |
| `Claude Code may not be enabled for your organization` setelah login gateway aplikasi Claude                                                                                                                                                                         | [Claude apps gateway troubleshooting](/docs/id/claude-apps-gateway-deploy#troubleshooting)                                       |
| `Context exceeds the ...-token limit by ... tokens` dalam output `/context`                                                                                                                                                                                          | [Request errors](#context-exceeds-the-token-limit)                                                                          |
| `Error during compaction: Conversation too long`                                                                                                                                                                                                                     | [Request errors](#error-during-compaction-conversation-too-long)                                                            |
| `Request too large`                                                                                                                                                                                                                                                  | [Request errors](#request-too-large)                                                                                        |
| `Request too large for the API's 32MB request limit`                                                                                                                                                                                                                 | [Request errors](#request-too-large)                                                                                        |
| `Image was too large`                                                                                                                                                                                                                                                | [Request errors](#image-was-too-large)                                                                                      |
| `Unable to resize image`                                                                                                                                                                                                                                             | [Request errors](#unable-to-resize-image)                                                                                   |
| `PDF too large` / `PDF is password protected`                                                                                                                                                                                                                        | [Request errors](#pdf-errors)                                                                                               |
| `Extra inputs are not permitted`                                                                                                                                                                                                                                     | [Request errors](#extra-inputs-are-not-permitted)                                                                           |
| `API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid` / `Property keys should match pattern`                                                                                                                                                      | [Request errors](#tool-input-schema-is-invalid)                                                                             |
| `There's an issue with the selected model`                                                                                                                                                                                                                           | [Request errors](#theres-an-issue-with-the-selected-model)                                                                  |
| `Model ... is not a recognized model id`                                                                                                                                                                                                                             | [Request errors](#model-is-not-a-recognized-model-id)                                                                       |
| `Model ... not found`                                                                                                                                                                                                                                                | [Request errors](#model-not-found)                                                                                          |
| `Claude Opus is not available with the Claude Pro plan`                                                                                                                                                                                                              | [Request errors](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                    |
| `Claude Code ... does not support this model; version ... or newer is required`                                                                                                                                                                                      | [Request errors](#claude-code-does-not-support-this-model)                                                                  |
| `Claude Code ... is older than the minimum version required by your organization's policy`                                                                                                                                                                           | [Request errors](#claude-code-does-not-support-this-model)                                                                  |
| `Model ... is restricted by your organization's settings`                                                                                                                                                                                                            | [Request errors](#model-is-restricted-by-your-organizations-settings)                                                       |
| `Model switch ... blocked by a PreModelSwitch hook`                                                                                                                                                                                                                  | [Request errors](#model-switch-was-blocked-by-a-premodelswitch-hook)                                                        |
| `couldn't save it as your default` / `couldn't confirm it was saved as your default`                                                                                                                                                                                 | [Request errors](#couldnt-save-it-as-your-default)                                                                          |
| `thinking.type.enabled is not supported for this model`                                                                                                                                                                                                              | [Request errors](#thinking-type-enabled-is-not-supported-for-this-model)                                                    |
| `Effort '<level>' isn't available with thinking turned off on this model`                                                                                                                                                                                            | [Request errors](#effort-isnt-available-with-thinking-turned-off)                                                           |
| `effort '<level>' is not supported when thinking is disabled`                                                                                                                                                                                                        | [Request errors](#effort-isnt-available-with-thinking-turned-off)                                                           |
| `max_tokens must be greater than thinking.budget_tokens`                                                                                                                                                                                                             | [Request errors](#thinking-budget-exceeds-output-limit)                                                                     |
| `API Error: 400 due to tool use concurrency issues`                                                                                                                                                                                                                  | [Request errors](#tool-use-or-thinking-block-mismatch)                                                                      |
| `API Error: 400 orphaned tool_result in conversation history`                                                                                                                                                                                                        | [Request errors](#tool-use-or-thinking-block-mismatch)                                                                      |
| `API Error: 400 duplicate tool_use ID in conversation history`                                                                                                                                                                                                       | [Request errors](#tool-use-or-thinking-block-mismatch)                                                                      |
| `[Unsupported tool content removed]`                                                                                                                                                                                                                                 | [Request errors](#unsupported-tool-content-removed)                                                                         |
| `role 'system' must precede an 'assistant' message`                                                                                                                                                                                                                  | [Request errors](#role-system-must-precede-an-assistant-message)                                                            |
| `Invalid encrypted_content in search_result block` / `Invalid encrypted_index in text block` / `Failed to decrypt web search result content`                                                                                                                         | [Request errors](#invalid-encrypted-content-in-search-result-block)                                                         |
| `server_tool_use.name: Input should be` pada setiap putaran sesi yang dilanjutkan                                                                                                                                                                                    | [Request errors](#unsupported-tool-content-removed)                                                                         |
| `<model> can't help with this. Start a new session to continue`                                                                                                                                                                                                      | [Request errors](#usage-policy-refusal)                                                                                     |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`                                                                                                                                                                        | [Request errors](#usage-policy-refusal)                                                                                     |
| `<model>'s safeguards flagged this message`                                                                                                                                                                                                                          | [Request errors](#safety-measures-flagged-a-cybersecurity-topic)                                                            |
| `Opus 5.5's safeguards flagged this session`                                                                                                                                                                                                                         | [Request errors](#safety-measures-flagged-a-cybersecurity-topic)                                                            |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                                                                                                                                                                                    | [Request errors](#safety-measures-flagged-a-cybersecurity-topic)                                                            |
| `Installation was killed before it could finish (exit code 137)`                                                                                                                                                                                                     | [Installation errors](#installation-was-killed-before-it-could-finish)                                                      |
| `The connection dropped while downloading the update`                                                                                                                                                                                                                | [Installation errors](#the-connection-dropped-while-downloading-the-update)                                                 |
| `Download timed out: exceeded the total deadline`                                                                                                                                                                                                                    | [Installation errors](#the-connection-dropped-while-downloading-the-update)                                                 |
| `--bg and --print conflict`                                                                                                                                                                                                                                          | [Command-line errors](#command-line-errors)                                                                                 |
| `Cloud sessions cannot be created from a --restricted session`                                                                                                                                                                                                       | [Command-line errors](#cloud-sessions-cannot-be-created-from-a-restricted-session)                                          |
| `Cloud sessions are disabled by your organization's policy`                                                                                                                                                                                                          | [Command-line errors](#cloud-sessions-are-disabled-by-your-organizations-policy)                                            |
| `Couldn't verify your organization's policy for cloud sessions`                                                                                                                                                                                                      | [Command-line errors](#cloud-sessions-are-disabled-by-your-organizations-policy)                                            |
| `Error: --json-schema is not a valid JSON Schema`                                                                                                                                                                                                                    | [Command-line errors](#command-line-errors)                                                                                 |
| `Error: Invalid --agents configuration:`                                                                                                                                                                                                                             | [Command-line errors](#invalid-agents-configuration)                                                                        |
| `Error: Settings file exceeds the 2MiB limit`                                                                                                                                                                                                                        | [Command-line errors](#settings-file-exceeds-the-2mib-limit)                                                                |
| `The current directory no longer exists (it was deleted or moved)` / `Can't read the current directory`                                                                                                                                                              | [Command-line errors](#the-current-directory-no-longer-exists)                                                              |
| `Temp directory <dir> ... Refusing to use it` / `ENOSPC: no space left on device, mkdir '<dir>'`                                                                                                                                                                     | [Command-line errors](#temp-directory-refused-or-cannot-be-created)                                                         |
| `couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded`                                                                                                                                                                        | [Command-line errors](#directory-couldnt-be-resolved-to-a-real-location)                                                    |
| `Error: Workspace not trusted` saat memulai Remote Control                                                                                                                                                                                                           | [Command-line errors](#workspace-not-trusted-when-starting-remote-control)                                                  |
| `` `<flag>` before `remote-control` is not carried over to the sessions Remote Control starts ``                                                                                                                                                                     | [Command-line errors](#not-carried-over-to-the-sessions-remote-control-starts)                                              |
| `` `claude import` is not yet available in this build ``                                                                                                                                                                                                             | [Command-line errors](#claude-import-is-not-yet-available-in-this-build)                                                    |
| `Could not read Claude Code config`                                                                                                                                                                                                                                  | [Command-line errors](#could-not-read-claude-code-config)                                                                   |
| `Could not import <server>: <reason>`                                                                                                                                                                                                                                | [Command-line errors](#could-not-import-a-server-from-claude-desktop)                                                       |
| `Cannot add MCP server to scope: managed`                                                                                                                                                                                                                            | [Command-line errors](#cannot-add-mcp-server-to-the-managed-scope)                                                          |
| `is Anthropic-hosted and doesn't support local OAuth`                                                                                                                                                                                                                | [Command-line errors](#anthropic-hosted-and-doesnt-support-local-oauth)                                                     |
| `Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes`                                                                                                                                                                                      | [Command-line errors](#cant-read-mcp-json)                                                                                  |
| `Server rejected the Authorization header minted by the configured headersHelper`                                                                                                                                                                                    | [Command-line errors](#server-rejected-the-authorization-header-minted-by-the-configured-headershelper)                     |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                                                                                                                                                                                             | [Command-line errors](#mcp-permission-prompt-tool-not-found)                                                                |
| `OAuth callback port <port> is already in use — another process may be holding it`                                                                                                                                                                                   | [Command-line errors](#oauth-callback-port-is-already-in-use)                                                               |
| `No available ports for OAuth redirect`                                                                                                                                                                                                                              | [Command-line errors](#no-available-ports-for-oauth-redirect)                                                               |
| `Shell command failed for pattern "..."`, dari `/security-review` atau skill apa pun yang menyuntikkan konteks dinamis                                                                                                                                               | [Command-line errors](#security-review-fails-without-origin-head)                                                           |
| `Shell command permission check failed for pattern "..."`, dari skill yang menyuntikkan konteks dinamis                                                                                                                                                              | [Command-line errors](#security-review-fails-without-origin-head)                                                           |
| ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``                                                                                                                                                                             | [Command-line errors](#security-review-fails-without-origin-head)                                                           |
| `Input must be provided either through stdin or as a prompt argument when using --print`                                                                                                                                                                             | [Command-line errors](#input-must-be-provided-when-using-print)                                                             |
| `Error: Input contained only whitespace`                                                                                                                                                                                                                             | [Command-line errors](#input-contained-only-whitespace)                                                                     |
| `Blank prompt — the message was only whitespace, so nothing was sent to the model.`                                                                                                                                                                                  | [Command-line errors](#input-contained-only-whitespace)                                                                     |
| `Error: stream-json input carried over 256M characters with no newline`                                                                                                                                                                                              | [Command-line errors](#stream-json-input-carried-over-256m-characters-with-no-newline)                                      |
| `Unknown command: /<name>`, dengan atau tanpa saran `Did you mean`                                                                                                                                                                                                   | [Command-line errors](#unknown-command)                                                                                     |
| `Diff is too large for ultrareview` / `PR #<N> is too large for ultrareview`                                                                                                                                                                                         | [Command-line errors](#diff-is-too-large-for-ultrareview)                                                                   |
| `Could not find merge-base with <branch>`                                                                                                                                                                                                                            | [Command-line errors](#could-not-find-merge-base-with-the-base-branch)                                                      |
| `Your checkout has no branches (detached HEAD only)`                                                                                                                                                                                                                 | [Command-line errors](#your-checkout-has-no-branches)                                                                       |
| `Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected`                                                                                                                                     | [Command-line errors](#no-github-account-is-connected-to-your-claude-account)                                               |
| `Your connected GitHub account can't see <owner>/<repo>`                                                                                                                                                                                                             | [Command-line errors](#your-connected-github-account-cant-see-the-repository)                                               |
| `The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead`                                                                                                                                           | [Command-line errors](#the-github-app-preflight-failed-transiently)                                                         |
| `GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud`                                                                                                                                                                     | [Command-line errors](#github-isnt-connected-to-your-claude-account)                                                        |
| `Single sign-on authorization needed`                                                                                                                                                                                                                                | [Command-line errors](#single-sign-on-authorization-needed)                                                                 |
| `Failed to resume the conversation`                                                                                                                                                                                                                                  | [Command-line errors](#failed-to-resume-the-conversation)                                                                   |
| `No conversation found with session ID: <session-id>`                                                                                                                                                                                                                | [Command-line errors](#no-conversation-found-with-the-session-id)                                                           |
| `Cannot switch renderers in this session`                                                                                                                                                                                                                            | [Command-line errors](#cannot-switch-renderers-in-this-session)                                                             |
| `Cannot switch renderers while work is running in the background`                                                                                                                                                                                                    | [Command-line errors](#cannot-switch-renderers-in-this-session)                                                             |
| `Couldn't open Claude Desktop`                                                                                                                                                                                                                                       | [Command-line errors](#couldnt-open-claude-desktop)                                                                         |
| `Failed to open Claude Desktop. Please try opening it manually.`                                                                                                                                                                                                     | [Command-line errors](#couldnt-open-claude-desktop)                                                                         |
| `Couldn't read your Zed keymap` / `Couldn't back up your Zed keymap` / `Couldn't update your Zed keymap`                                                                                                                                                             | [Command-line errors](#terminal-setup-left-your-zed-keymap-unchanged)                                                       |
| `Your Zed keymap isn't a readable list of keybindings`                                                                                                                                                                                                               | [Command-line errors](#terminal-setup-left-your-zed-keymap-unchanged)                                                       |
| `Skill usage reports are not available on this connection.`                                                                                                                                                                                                          | [Command-line errors](#skill-usage-reports-are-not-available-on-this-connection)                                            |
| `Custom output styles can't be selected over Remote Control or from a relayed message`                                                                                                                                                                               | [Command-line errors](#custom-output-styles-cant-be-selected-over-remote-control)                                           |
| `Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load`                                                                                                                                                           | [Command-line errors](#output-styles-are-saved-to-local-settings-which-this-session-doesnt-load)                            |
| `` `plugin eval` is currently in early access `` / `` `plugin eval` is currently unavailable ``                                                                                                                                                                      | [Plugin errors](#plugin-eval-is-currently-in-early-access)                                                                  |
| `Marketplace "<name>" is registered from an untrusted source`                                                                                                                                                                                                        | [Plugin errors](#marketplace-is-registered-from-an-untrusted-source)                                                        |
| `Marketplace "<name>" is already added from a different source`                                                                                                                                                                                                      | [Plugin errors](#marketplace-is-already-added-from-a-different-source)                                                      |
| `"<name>" is another spelling of "<reserved>", a reserved marketplace name`                                                                                                                                                                                          | [Plugin errors](#marketplace-name-is-another-spelling-of-a-reserved-name)                                                   |
| `references ${user_config.*} in a shell-form command`                                                                                                                                                                                                                | [Plugin errors](#plugin-command-references-user-config)                                                                     |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                                                                                                                                                                                   | [Plugin errors](#plugin-command-references-user-config)                                                                     |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                                                                                                                                                                                  | [Plugin errors](#plugin-command-references-user-config)                                                                     |
| `Plugin archive integrity check failed`                                                                                                                                                                                                                              | [Plugin errors](#plugin-archive-integrity-check-failed)                                                                     |
| `path escapes plugin directory`                                                                                                                                                                                                                                      | [Plugin errors](#path-escapes-plugin-directory)                                                                             |
| `path could not be checked`                                                                                                                                                                                                                                          | [Plugin errors](#path-could-not-be-checked)                                                                                 |
| `its marketplace entry path does not stay inside the marketplace directory`                                                                                                                                                                                          | [Plugin errors](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                     |
| `Plugin source path refused`                                                                                                                                                                                                                                         | [Plugin errors](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                     |
| `Failed to load marketplace configuration`                                                                                                                                                                                                                           | [Plugin errors](#failed-to-load-marketplace-configuration)                                                                  |
| `Marketplace configuration file is corrupted`                                                                                                                                                                                                                        | [Plugin errors](#failed-to-load-marketplace-configuration)                                                                  |
| `Plugin "<name>@synced" is required by your organization and can't be disabled here`                                                                                                                                                                                 | [Plugin errors](#plugin-is-required-by-your-organization)                                                                   |
| `would be spawned with zero tools — refusing`                                                                                                                                                                                                                        | [Tool errors](#agent-would-be-spawned-with-zero-tools)                                                                      |
| `File is covered by a Read deny rule in your permission settings`                                                                                                                                                                                                    | [Tool errors](#file-is-covered-by-a-read-deny-rule)                                                                         |
| `subagent_type is required: the general-purpose agent is not available in this session`                                                                                                                                                                              | [Tool errors](#subagent-type-is-required)                                                                                   |
| `Error: this write left the memory index at MEMORY.md at ..., over its ... read limit`                                                                                                                                                                               | [Tool errors](#memory-index-is-over-its-read-limit)                                                                         |
| `pkill: refusing to run`                                                                                                                                                                                                                                             | [Tool errors](#pkill-pattern-matches-the-claude-code-process)                                                               |
| `Failed to write to <name>'s inbox — nothing was sent`                                                                                                                                                                                                               | [Tool errors](#failed-to-write-to-a-teammate-inbox)                                                                         |
| `Failed to write the plan approval request to the lead's inbox — plan not submitted`                                                                                                                                                                                 | [Tool errors](#failed-to-write-to-a-teammate-inbox)                                                                         |
| `Its agent definition was not restored: the folder its definition file came from is not trusted`                                                                                                                                                                     | [Tool errors](#teammate-agent-definition-not-restored)                                                                      |
| `Message too large for cross-session delivery`                                                                                                                                                                                                                       | [Tool errors](#message-too-large-for-cross-session-delivery)                                                                |
| `Too many messages to this session just now`                                                                                                                                                                                                                         | [Tool errors](#too-many-messages-to-this-session-just-now)                                                                  |
| `Refusing to send: reply target is a symlink` / `Refusing to send: cannot vet reply target`                                                                                                                                                                          | [Tool errors](#refusing-to-send-a-cross-session-message)                                                                    |
| `Refusing to send: connected endpoint is not the expected process` / `Refusing to send: connected endpoint identity could not be read`                                                                                                                               | [Tool errors](#refusing-to-send-a-cross-session-message)                                                                    |
| `Refusing to send: connected endpoint is not owned by this user` / `Refusing to send: connected endpoint owner could not be read`                                                                                                                                    | [Tool errors](#refusing-to-send-a-cross-session-message)                                                                    |
| `Refusing to send: connected endpoint is a different process with the expected pid`                                                                                                                                                                                  | [Tool errors](#refusing-to-send-a-cross-session-message)                                                                    |
| `Refusing to read <path>: its symlink resolution changed after permission was checked (<reason>)` / `Refusing to search <path>: its symlink resolution changed after permission was checked`                                                                         | [Tool errors](#refusing-after-a-symlink-changed)                                                                            |
| `Refusing to write <path>: its parent-directory symlink resolution changed after permission was checked` / `Refusing to write <path>: it is a symbolic link. Write to the link's target path instead`                                                                | [Tool errors](#refusing-after-a-symlink-changed)                                                                            |
| `Refusing to write through symlink: <path>` / `Refusing to write into symlinked directory: <path>`                                                                                                                                                                   | [Tool errors](#refusing-after-a-symlink-changed)                                                                            |
| `Refusing to search <path>: a path one of its Read deny rules is written through changed while the search was being prepared` / `Refusing to search <path>: it could not be opened`                                                                                  | [Tool errors](#refusing-after-a-symlink-changed)                                                                            |
| `its permission check expired before it ran (too many concurrent file operations)` / `ripgrep was found only by name on PATH`                                                                                                                                        | [Tool errors](#refusing-after-a-symlink-changed)                                                                            |
| `task output swap refused (tasks dir moved or linked)`                                                                                                                                                                                                               | [Tool errors](#task-output-swap-refused)                                                                                    |
| `Command killed: its output file was replaced or could no longer be verified`                                                                                                                                                                                        | [Tool errors](#task-output-swap-refused)                                                                                    |
| `the source file is not valid UTF-8 text` / `the source file is not valid UTF-16 text`                                                                                                                                                                               | [Tool errors](#the-source-file-is-not-valid-utf-8-text)                                                                     |
| `the source file has the replacement character U+FFFD`                                                                                                                                                                                                               | [Tool errors](#the-source-file-is-not-valid-utf-8-text)                                                                     |
| `Reading a local file from outside this session's connected folders, or through a link, needs the approval card`                                                                                                                                                     | [Tool errors](#reading-a-local-file-from-outside-the-connected-folders)                                                     |
| `cannot read file_path (...) — the file could not be examined, and no one can answer the approval card`                                                                                                                                                              | [Tool errors](#reading-a-local-file-from-outside-the-connected-folders)                                                     |
| `WebFetch cannot fetch localhost or other hostnames without a dot`                                                                                                                                                                                                   | [Tool errors](#webfetch-cannot-fetch-localhost)                                                                             |
| `Can't open MCP settings while no terminal is attached to this background session`                                                                                                                                                                                   | [Background session errors](#commands-refused-in-a-background-session)                                                      |
| `Can't open MCP settings in a background session`                                                                                                                                                                                                                    | [Background session errors](#commands-refused-in-a-background-session)                                                      |
| `blocked because the path is spelled in a form that cannot be safely resolved`                                                                                                                                                                                       | [Background session errors](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved)                           |
| `blocked because the path is network-shaped`                                                                                                                                                                                                                         | [Background session errors](#write-or-command-blocked-because-the-path-names-a-network-location)                            |
| `is isolated in the worktree <path>, but this command <reason>. Refusing to run it`                                                                                                                                                                                  | [Background session errors](#command-blocked-by-the-worktree-isolation-checks)                                              |
| `too complex to verify that it stays inside the worktree`                                                                                                                                                                                                            | [Background session errors](#command-blocked-by-the-worktree-isolation-checks)                                              |
| `This session has no saved transcript`                                                                                                                                                                                                                               | [Background session errors](#this-session-has-no-saved-transcript)                                                          |
| `Can't open — this session is running in another terminal`                                                                                                                                                                                                           | [Background session errors](#this-session-is-running-in-another-terminal)                                                   |
| `This conversation is already open in another running Claude session`                                                                                                                                                                                                | [Background session errors](#this-session-is-running-in-another-terminal)                                                   |
| `This session's saved conversation is no longer on disk`                                                                                                                                                                                                             | [Background session errors](#this-sessions-saved-conversation-is-no-longer-on-disk)                                         |
| `kept <id> — its worktree is still at <path>`                                                                                                                                                                                                                        | [Background session errors](#worktree-has-commits-that-are-not-pushed-anywhere)                                             |
| `kept <id> — <n> unpushed commits on <branch>`                                                                                                                                                                                                                       | [Background session errors](#worktree-has-commits-that-are-not-pushed-anywhere)                                             |
| `kept <id> — worktree has commits that are not pushed anywhere`                                                                                                                                                                                                      | [Background session errors](#worktree-has-commits-that-are-not-pushed-anywhere)                                             |
| `terminal host process died — press Enter to restart` / `This session's terminal host process died`                                                                                                                                                                  | [Background session errors](#terminal-host-process-died)                                                                    |
| `Session isn't responding` / `Press enter again to restart this session — it isn't responding`                                                                                                                                                                       | [Background session errors](#session-isnt-responding)                                                                       |
| `Session <id> was stopped while the respawn was in flight`                                                                                                                                                                                                           | [Background session errors](#session-was-stopped-while-the-respawn-was-in-flight)                                           |
| `This session was running agent '<name>', which is no longer available`                                                                                                                                                                                              | [Background session errors](#session-agent-no-longer-available)                                                             |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                                                                                                                                                                                          | [Background session errors](#claude_code_process_wrapper-launcher-errors)                                                   |
| `EUNKNOWN: unknown error, uv_spawn`                                                                                                                                                                                                                                  | [Background session errors](#eunknown-when-starting-a-background-session)                                                   |
| `EACCES: permission denied, posix_spawn`                                                                                                                                                                                                                             | [Background session errors](#eacces-when-starting-a-background-session)                                                     |
| `exited before it became reachable`                                                                                                                                                                                                                                  | [Background session errors](#background-service-exited-before-it-became-reachable)                                          |
| `Couldn't start a background session (working directory no longer exists or is not accessible: ...)`                                                                                                                                                                 | [Background session errors](#working-directory-no-longer-exists-when-starting-a-background-session)                         |
| `Claude Code is being updated by npm on this machine (still not runnable after 2 min, ...)`                                                                                                                                                                          | [Background session errors](#eacces-when-starting-a-background-session)                                                     |
| `Claude Code process exited with code N`                                                                                                                                                                                                                             | [Wrapper and IDE errors](#claude-code-process-exited-with-code-n)                                                           |
| `The connection to Claude Code ended before this message completed`                                                                                                                                                                                                  | [Wrapper and IDE errors](#the-connection-to-claude-code-ended-before-this-message-completed)                                |
| `Could not locate the Claude CLI on PATH`                                                                                                                                                                                                                            | [Wrapper and IDE errors](#could-not-locate-the-claude-cli-on-path)                                                          |
| `Restored the code, but skipped N files`                                                                                                                                                                                                                             | [Rewind warnings and errors](#restored-the-code-but-skipped-files)                                                          |
| `No files were restored: N files failed (backup missing, or the file could not be updated)`                                                                                                                                                                          | [Rewind warnings and errors](#no-files-were-restored)                                                                       |
| `Transcript writes are failing (...)`                                                                                                                                                                                                                                | [Session saving warnings](#transcript-writes-are-failing)                                                                   |
| `Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set`                                                                                                                                                                                                  | [Session saving warnings](#transcript-saving-is-off-skip-prompt-history)                                                    |
| `Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker`                                                                                                                                                                                              | [Session saving warnings](#transcript-saving-is-off-child-session-marker)                                                   |
| `Claude Code's fullscreen renderer didn't finish starting last time on this machine` / `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`                                                                                            | [Configuration warnings](#fullscreen-failed-start-notice)                                                                   |
| `Claude Code exited after an unrecoverable interface error (...)`                                                                                                                                                                                                    | [Configuration warnings](#exited-after-an-unrecoverable-interface-error)                                                    |
| `Agent descriptions are over the 15.0k-token limit`                                                                                                                                                                                                                  | [Configuration warnings](#agent-descriptions-are-over-the-15000-token-limit)                                                |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                                                                                                                                                                                  | [Configuration warnings](#workspace-has-not-been-trusted)                                                                   |
| `is a network path, which cannot be added as a working directory`                                                                                                                                                                                                    | [Configuration warnings](#working-directory-is-a-network-path)                                                              |
| `Remote managed settings failed to load (<cause>)`                                                                                                                                                                                                                   | [Configuration warnings](#remote-managed-settings-failed-to-load)                                                           |
| `Managed settings were not approved; exiting without applying them.`                                                                                                                                                                                                 | [Configuration warnings](#managed-settings-were-not-approved)                                                               |
| `MCP server <name> is blocked by enterprise managed policy`                                                                                                                                                                                                          | [Configuration warnings](#mcp-server-is-blocked-by-enterprise-managed-policy)                                               |
| `Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.`                                                                                                                                              | [Configuration warnings](#managed-settings-document-could-not-be-parsed)                                                    |
| `Managed settings drop-in directory could not be read`                                                                                                                                                                                                               | [Configuration warnings](#managed-settings-document-could-not-be-parsed)                                                    |
| `otelHeadersHelper failed; telemetry is not being exported. See /status: ...`                                                                                                                                                                                        | [Configuration warnings](#otelheadershelper-failed)                                                                         |
| `"crossSessionInbound" must be one of "accept", "hold", "refuse"`                                                                                                                                                                                                    | [Configuration warnings](#crosssessioninbound-must-be-one-of-accept-hold-refuse)                                            |
| `headersHelper not run — this workspace has no persisted trust`                                                                                                                                                                                                      | [Configuration warnings](#headershelper-not-run)                                                                            |
| `Invalid permission rule "..." was skipped: Malformed Tool(content) rule`                                                                                                                                                                                            | [Configuration warnings](#malformed-tool-content-rule)                                                                      |
| `... is not matched by file permission checks`                                                                                                                                                                                                                       | [Configuration warnings](#is-not-matched-by-file-permission-checks)                                                         |
| `... has a wildcard before the rest of the command`                                                                                                                                                                                                                  | [Configuration warnings](#has-a-wildcard-before-the-rest-of-the-command)                                                    |
| `CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced`                                                                                                                                                                                           | [Configuration warnings](#the-200k-limit-isnt-enforced)                                                                     |
| `[claude-code:unrecognized_model]`                                                                                                                                                                                                                                   | [Configuration warnings](#unrecognized-model-id-on-a-request)                                                               |
| `Stale sandbox mask files left by a killed session`                                                                                                                                                                                                                  | [Configuration warnings](#stale-sandbox-mask-files-left-by-a-killed-session)                                                |
| Respons tampak berkualitas lebih rendah dari biasanya                                                                                                                                                                                                                | [Response quality](#responses-seem-lower-quality-than-usual)                                                                |

<h2 id="automatic-retries">
  Pengulangan otomatis
</h2>

Claude Code mengulangi kegagalan transien hingga 10 kali dengan exponential backoff sebelum menampilkan kesalahan kepada Anda. Tidak selalu mengulangi kegagalan yang tiba di tengah respons Claude. Ketika Anda melihat salah satu kesalahan di halaman ini, Claude Code telah membuat pengulangan apa pun yang berlaku untuk kegagalan tersebut; daftar di bawah mengatakan kegagalan mana yang mendapat anggaran penuh, mana yang mendapat anggaran lebih kecil, dan mana yang tidak mendapat apa pun.

Claude Code mengulangi kegagalan ini:

* Server errors, overloaded responses, dan request timeouts yang tiba sebelum respons Claude mulai streaming.
* Dropped connections. Ketika koneksi terputus di tengah permintaan sebelum Claude menyelesaikan bagian mana pun dari responsnya, termasuk pemikirannya, Claude Code mengeluarkan kembali permintaan dengan backoff yang sama dan giliran berlanjut, bahkan jika beberapa teks telah mulai streaming. Ketika terputus setelah Claude selesai berpikir tetapi sebelum memulai teks atau panggilan alat apa pun, Claude Code malah mengeluarkan kembali permintaan hingga dua kali dengan cepat, dan mengakhiri giliran dengan `Connection lost before a response was produced` jika koneksi terus terputus pada titik itu.
* Koneksi yang Claude Code deteksi rusak karena komputer Anda tidur di tengah permintaan. Claude Code menghitungnya sebagai dropped connection sesuai aturan di atas; setelah label pengulangan menyebutkan alasan spesifik, dibaca `Connection lost while your computer was asleep`, dan jika giliran berakhir setelah Claude selesai berpikir tetapi sebelum teks atau panggilan alat apa pun, pesan berbunyi `Your computer went to sleep before a response was produced`.
* Stalled response stream, ketika response headers telah tiba tetapi tidak ada respons Claude yang tiba, atau ketika Claude selesai berpikir tetapi belum memulai teks atau panggilan alat apa pun: Claude Code membatalkan koneksi yang macet dan mengeluarkan kembali permintaan paling banyak sekali, di luar anggaran 10-percobaan di atas. Jika respons macet untuk kedua kalinya setelah Claude selesai berpikir tetapi sebelum teks atau panggilan alat apa pun, Claude Code mengakhiri giliran dengan `The response stalled before a response was produced`.
* Streaming request yang API tidak pernah jawab dengan response headers, pada koneksi di mana [first-byte deadline berjalan](/docs/id/network-config#streaming-idle-watchdogs): Claude Code membatalkannya pada deadline dan mengirimnya kembali paling banyak sekali per model request, dalam anggaran pengulangan, kemudian mengakhiri giliran dengan [No response from API](#no-response-from-api) jika percobaan itu juga tidak dijawab. Pada koneksi lain, permintaan menunggu `API_TIMEOUT_MS`. Ketika Anda menetapkan `CLAUDE_CODE_RETRY_WATCHDOG`, batas satu pengulangan tidak berlaku.
* Temporary 429 throttles, tetapi bukan `429` spend-limit gateway, yang bukan throttle; lihat [Spend limit reached](#spend-limit-reached).
  * Ketika Anda masuk dengan langganan claude.ai, ini mencakup 429 throttles yang tidak membawa header kuota rencana Anda. Sebelum v2.1.199, Claude Code hanya mengulangi throttles tersebut untuk API key dan Enterprise sign-ins.
* Permintaan ditolak karena input plus `max_tokens` melebihi context limit. Mengirimnya kembali tanpa perubahan akan gagal dengan cara yang sama, jadi Claude Code mengulangi dengan `max_tokens` yang dikurangi, dan berhenti mengulangi dan melakukan compact sebagai gantinya dalam dua kasus:
  * Ketika tidak ada pengurangan yang dapat muat, misalnya ketika percakapan itu sendiri hampir mengisi context window.
  * Ketika pengulangan tidak dapat menyusutkan `max_tokens` lebih jauh. Sebelum v2.1.218, Claude Code dapat mengirim kembali permintaan yang dikurangi yang masih tidak muat, seperti ketika extended thinking budget melebihi context yang tersisa, sampai anggaran pengulangan habis.
* Expired atau missing Google Cloud credential pada [Google Cloud's Agent Platform](/docs/id/google-vertex-ai), atau AWS credentials yang gagal dimuat di mesin Anda. Claude Code membuang credentials yang di-cache dan mengulangi hingga dua kali, kemudian melaporkan kesalahan sehingga Anda dapat re-authenticate segera, seperti dijelaskan di bawah [Could not load AWS or Google Cloud credentials](#could-not-load-aws-or-google-cloud-credentials). Sebelum v2.1.228, Claude Code mengulangi failing Google Cloud credential melalui anggaran pengulangan penuh sebelum menampilkan kesalahan.
* `401` atau `403` dari Anthropic API, secara langsung atau melalui [LLM gateway](/docs/id/llm-gateway), sementara script [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) menyediakan credential. Claude Code menjalankan kembali script dan mengulangi dengan output segar-nya, dalam anggaran pengulangan penuh. Ketika script itu sendiri gagal pada re-run, Claude Code menampilkan [Your apiKeyHelper script is failing](#your-apikeyhelper-script-is-failing) sebagai gantinya.

Sebelum v2.1.227, `Connection lost before a response was produced` berbunyi `Connection closed while thinking, before producing a response` dan `The response stalled before a response was produced` berbunyi `Response stalled while thinking, before producing a response`.

Claude Code tidak mengulangi kegagalan ini:

* TLS certificate validation failure, seperti TLS-inspecting proxy, missing `NODE_EXTRA_CA_CERTS` bundle, atau expired certificate. Claude Code melaporkan kesalahan pada percobaan pertama, sehingga Anda dapat memperbaiki setup certificate dengan segera; lihat [SSL certificate errors](#ssl-certificate-errors). Claude Code masih mengulangi transient TLS conditions seperti handshake timeout. Sebelum v2.1.199, Claude Code mengulangi certificate failures melalui anggaran pengulangan penuh sebelum menampilkan kesalahan.
* Server error, dropped connection, atau stalled stream yang tiba setelah Claude menyelesaikan blok teks atau panggilan alat, atau telah memulai satu setelah menyelesaikan pemikirannya, tetapi sebelum menyelesaikan respons. Claude Code tidak menjalankan kembali permintaan, karena itu dapat mengeksekusi panggilan alat yang sama dua kali. Ini menyimpan apa yang Claude selesaikan, menjalankan panggilan alat apa pun yang Claude selesaikan, dan melanjutkan giliran dari hasil mereka. Untuk apa yang Anda lihat dalam sesi interaktif dan non-interaktif, baca [The response above may be incomplete](#the-response-above-may-be-incomplete). Sebelum v2.1.199, Claude Code membuang output parsial dan melaporkan seluruh giliran sebagai kesalahan ketika server error tiba di tengah-tengah stream.
* Kegagalan yang tiba setelah Claude menyelesaikan respons: tidak ada yang perlu diulangi, jadi Claude Code menyimpan respons lengkap dan mengakhiri giliran secara normal.
* [Amazon Bedrock streaming response dengan unexpected content-type](#bedrock-streaming-response-has-an-unexpected-content-type), karena gateway atau proxy yang menulis ulang respons akan menulis ulang pengulangan dengan cara yang sama. Memerlukan Claude Code v2.1.208 atau lebih baru.
* Non-streaming retry dari failed streaming request yang mendapat success status tetapi [no Claude API message in the body](#api-returned-an-empty-or-malformed-response). Claude Code mengakhiri giliran dengan kesalahan itu.
* Permintaan yang policy check organisasi Anda tolak, yang muncul sebagai baris `API Error:` yang membawa pesan penolakan. Administrator organisasi Anda menyiapkan check dengan [Inference hooks](https://platform.claude.com/docs/en/manage-claude/inference-hooks), fitur Claude Enterprise, dan pesan berakhir dengan instruksi yang mereka konfigurasi, atau secara default memberi tahu Anda untuk menghubungi mereka. Claude Code tidak mengirim kembali permintaan yang ditolak ke model yang sama atau ke [fallback model](/docs/id/model-config#fallback-model-chains), karena penolakan adalah tentang konten permintaan daripada model. Sebelum v2.1.239, Claude Code dapat mengirim kembali permintaan yang ditolak, tanpa streaming atau pada fallback model yang dikonfigurasi, sebelum menampilkan penolakan kepada Anda.

<h3 id="what-you-see-while-claude-code-retries-or-waits">
  Apa yang Anda lihat saat Claude Code mengulangi atau menunggu
</h3>

Saat mengulangi, spinner menampilkan countdown `Retrying in Ns · attempt x/y` setelah label kesalahan. Label menyebutkan alasan spesifik dari percobaan pertama untuk kegagalan yang dapat Anda tindaklanjuti segera: jaringan tidak aktif, TLS handshake gagal, atau Anda mencapai rate limit. Untuk kesalahan lain dibaca `API error` pada awalnya. Mulai v2.1.198 beralih ke alasan spesifik dari percobaan ketiga, atau pada percobaan terakhir ketika `CLAUDE_CODE_MAX_RETRIES` memungkinkan lebih sedikit dari tiga; versi sebelumnya beralih hanya pada percobaan terakhir.

Mulai v2.1.198, spinner tip biasa ditekan selama pengulangan. Setelah alasan kesalahan terungkap, jika kegagalan adalah 529 overload baris di bawah countdown juga menyebutkan di mana memeriksa status layanan: `status.claude.com` pada Anthropic API, atau penyedia atau host gateway yang dinamai dalam pesan pada konfigurasi lain.

Jika tidak ada data yang tiba pada response stream selama 20 detik sementara permintaan masih tertunda, spinner menampilkan `Waiting for API response · will retry in … · check your network` sebelum pengulangan apa pun telah dimulai. Permintaan belum gagal: countdown berjalan ke titik di mana Claude Code membatalkan koneksi yang macet. Setelah pembatalan, apa yang Anda lihat tergantung pada seberapa jauh respons telah sampai:

* Sebelum Claude menyelesaikan blok teks atau panggilan alat, atau memulai satu setelah menyelesaikan pemikirannya, Claude Code mengulangi permintaan atau mengakhiri giliran dengan kesalahan. [Automatic retries](#automatic-retries) mengatakan stalls mana yang diulanginya dan berapa kali.
* Setelah Claude menyelesaikan blok teks atau panggilan alat, atau memulai satu setelah menyelesaikan pemikirannya, tetapi sebelum Claude menyelesaikan respons, Claude Code menyimpan apa yang Claude selesaikan, melanjutkan giliran dari panggilan alat apa pun yang Claude selesaikan, dan menampilkan [The response above may be incomplete](#the-response-above-may-be-incomplete). Dalam sesi non-interaktif, dan untuk respons subagent dalam sesi apa pun, Claude Code mungkin pertama-tama meminta Claude untuk melanjutkan respons; entry itu mengatakan kapan itu terjadi dan kapan Anda masih melihat pemberitahuan di sana.
* Setelah Claude menyelesaikan respons, Claude Code mengakhiri giliran secara normal.

Banner menghapus dirinya sendiri setelah data dilanjutkan atau pengulangan berhasil. Jika muncul kembali pada setiap percobaan, perlakukan sebagai [network issue](#unable-to-connect-to-api). Sebelum v2.1.185, banner muncul setelah 10 detik dengan wording berbeda.

Saat Claude berkonsultasi dengan [advisor](/docs/id/advisor), banner muncul setelah 90 detik tanpa data bukan 20, karena review advisor yang panjang dapat mengirim tidak ada selama lebih dari 20 detik. Sebelum v2.1.214, threshold 20-detik diterapkan selama advisor calls juga, jadi banner muncul selama advisor reviews bahkan ketika tidak ada yang salah.

<h3 id="tune-retry-behavior">
  Sesuaikan perilaku pengulangan
</h3>

Anda dapat menyesuaikan perilaku pengulangan dengan variabel lingkungan ini:

| Variable                                              | Default | Effect                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :---------------------------------------------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/id/env-vars)             | 10      | Jumlah percobaan pengulangan. Dibatasi pada 15 mulai v2.1.186; mulai v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` menaikkan default dan menghapus batas. Turunkan untuk menampilkan kegagalan lebih cepat dalam script.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/id/env-vars)          | unset   | Atur ke `1` dalam sesi tanpa pengawasan seperti CI jobs untuk mengulangi `429` dan `529` capacity errors tanpa batas bukan gagal setelah `CLAUDE_CODE_MAX_RETRIES` percobaan. Claude Code gagal sekaligus ketika standard-speed request mendapat `429` yang melaporkan spend limit atau exhausted usage credits, bahkan satu dari [gateway spend cap](#spend-limit-reached) yang reset pada jadwal. Sebelum v2.1.239, watchdog mengulangi ini tanpa batas. Untuk fast mode requests, lihat [Handle rate limits](/docs/id/fast-mode#handle-rate-limits). Pada v2.1.199 atau lebih baru itu juga menaikkan default retry count untuk transient errors lainnya, seperti server errors, timeouts, dan dropped connections, menjadi 300, kira-kira tiga jam backoff, dan menghapus batas 15 pada `CLAUDE_CODE_MAX_RETRIES` jika Anda menetapkan variabel itu secara eksplisit. |
| [`API_TIMEOUT_MS`](/docs/id/env-vars)                      | 600000  | Per-request timeout dalam milliseconds. Naikkan untuk jaringan lambat atau proxy. Ini juga membatasi berapa lama Claude Code menunggu response headers, dijelaskan dalam [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/id/env-vars) | unset   | Deadline dalam milliseconds untuk first response byte dari streaming request. Memerlukan Claude Code v2.1.242 atau lebih baru. Untuk bagaimana Claude Code memilih deadline ketika ini unset, lihat [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

<h2 id="server-errors">
  Kesalahan server
</h2>

Sebagian besar kesalahan ini berasal dari penyedia inferensi: layanan Anthropic di Anthropic API, dan layanan di balik titik akhir penyedia tersebut di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau gateway khusus. [Auto mode tidak dapat menentukan keamanan suatu tindakan](#auto-mode-cannot-determine-the-safety-of-an-action) dan [Agent dihentikan lebih awal karena kesalahan API](#agent-terminated-early-due-to-an-api-error) juga mencakup penyebab di sisi Anda, seperti akun Amazon Bedrock yang tidak dapat memanggil model pengklasifikasi atau subagent yang mencapai batas penggunaan.

<h3 id="api-error-500-internal-server-error">
  API Error: 500 Internal server error
</h3>

Claude Code menampilkan kode status dan pesan kesalahan API untuk respons 5xx apa pun. Contoh di bawah menunjukkan respons 500 di Anthropic API:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

Kalimat terakhir menyebutkan tempat untuk memeriksa kesehatan layanan dan bervariasi menurut penyedia. Konfigurasi Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry menyebutkan status layanan penyedia tersebut. `ANTHROPIC_BASE_URL` khusus menyebutkan host gateway.

Ini menunjukkan kegagalan yang tidak terduga di dalam API. Ini tidak disebabkan oleh prompt, pengaturan, atau akun Anda.

**Yang harus dilakukan:**

* Periksa [status.claude.com](https://status.claude.com), atau halaman status penyedia yang disebutkan dalam pesan, untuk insiden aktif
* Tunggu satu menit, kemudian kirim pesan Anda lagi. Pesan asli Anda masih ada dalam percakapan, jadi untuk prompt yang panjang Anda dapat mengetik `try again` alih-alih menempel seluruh hal.
* Jika kesalahan berlanjut tanpa insiden yang diposting, jalankan `/feedback` sehingga Anthropic dapat menyelidiki dengan detail permintaan Anda. Lihat [Report an error](#report-an-error) jika `/feedback` tidak tersedia di lingkungan Anda.

<h3 id="api-error-repeated-529-overloaded-errors">
  API Error: Repeated 529 Overloaded errors
</h3>

API sementara pada kapasitas di semua pengguna. Claude Code telah mencoba kembali beberapa kali sebelum menampilkan pesan ini:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

Kalimat terakhir bervariasi menurut penyedia dengan cara yang sama seperti kesalahan 500 di atas.

529 bukan batas penggunaan Anda dan tidak dihitung terhadap kuota Anda.

**Yang harus dilakukan:**

* Periksa [status.claude.com](https://status.claude.com), atau halaman status penyedia yang disebutkan dalam pesan, untuk pemberitahuan kapasitas
* Coba lagi dalam beberapa menit
* Jalankan `/model` dan beralih ke model yang berbeda untuk terus bekerja, karena kapasitas dilacak per model. Claude Code meminta Anda untuk melakukan ini ketika satu model mengalami beban yang sangat tinggi, misalnya `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Request timed out
</h3>

API tidak merespons sebelum batas waktu koneksi.

```text theme={null}
Request timed out
```

Ini dapat terjadi selama periode beban tinggi atau ketika model menghasilkan respons yang sangat besar. Waktu tunggu permintaan default adalah 10 menit.

**Yang harus dilakukan:**

* Coba lagi permintaan
* Untuk tugas yang berjalan lama, pecah pekerjaan menjadi prompt yang lebih kecil
* Jika penyebabnya adalah jaringan lambat atau proxy, naikkan `API_TIMEOUT_MS` seperti yang dijelaskan dalam [Automatic retries](#automatic-retries)
* Jika waktu tunggu sering terjadi dan jaringan Anda sehat, lihat [Network and connection errors](#network-and-connection-errors) di bawah

<h3 id="no-response-from-api">
  No response from API
</h3>

Claude Code mengirim permintaan streaming dan API tidak mengembalikan header respons dalam batas waktu untuk byte pertama, jadi Claude Code membatalkan permintaan alih-alih menunggu waktu tunggu permintaan `API_TIMEOUT_MS` penuh, 10 menit secara default. Claude Code mengirim permintaan lagi paling banyak sekali, jika [retry budget](#tune-retry-behavior) memungkinkan. Ketika percobaan ulang juga tidak dijawab, giliran berakhir dengan pesan ini, yang menunjukkan berapa lama setiap upaya menunggu. Ketika Anda menetapkan [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/id/env-vars), batas satu percobaan ulang tidak berlaku dan Claude Code mencoba ulang di bawah anggaran yang dijelaskan dalam [Tune retry behavior](#tune-retry-behavior).

```text theme={null}
API Error: No response from API (waited 3m, then 10m on the retry). If a proxy or gateway on your network holds responses until they complete, raise API_TIMEOUT_MS or CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS to wait longer.
```

Claude Code menetapkan tunggu header respons upaya pertama dan tunggu percobaan ulang secara terpisah:

* **Upaya pertama**: [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/id/env-vars) ketika Anda menetapkannya ke 1 atau lebih, diklem antara 10 detik dan 30 menit. Jika tidak, Claude Code menggunakan waktu tunggu watchdog tingkat byte yang tercantum dalam [Streaming idle watchdogs](/docs/id/network-config#streaming-idle-watchdogs), jadi variabel yang mengubah waktu tunggu itu mengubah tunggu ini juga. Bagaimanapun, Claude Code menambahkan satu detik untuk setiap 32KB badan permintaan.
* **Percobaan ulang**: satu detik kurang dari `API_TIMEOUT_MS`, hanya di bawah 10 menit secara default, sehingga percobaan ulang dapat melampaui proxy atau gateway yang menahan respons hingga generasi selesai. Di Amazon Bedrock, percobaan ulang menggunakan batas waktu yang sama dengan upaya pertama, dan pesan menunjukkan satu durasi alih-alih dua.

Tidak ada tunggu yang melebihi satu detik kurang dari `API_TIMEOUT_MS` positif, dan `API_TIMEOUT_MS` positif di bawah 11 detik mematikan batas waktu. Watchdog tingkat byte dimulai hanya setelah header respons tiba, jadi respons yang berhenti mengirim byte setelah itu mengikuti [stalled-stream rules](#automatic-retries) alih-alih batas waktu ini.

**Yang harus dilakukan:**

* Kirim pesan Anda lagi. Pesan asli Anda masih ada dalam percakapan, jadi untuk prompt yang panjang Anda dapat mengetik `try again` alih-alih menempel seluruh hal.
* Jika terulang, perlakukan sebagai [network or proxy problem](#unable-to-connect-to-api). Proxy yang menerima koneksi dan tidak pernah meneruskan permintaan menghasilkan kesalahan ini pada setiap upaya.
* Jika proxy atau gateway di jaringan Anda menahan respons hingga selesai, naikkan `API_TIMEOUT_MS` sehingga percobaan ulang menunggu lebih lama. Di Amazon Bedrock, naikkan `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` juga.
* Jika upaya pertama terus habis waktu dan percobaan ulang kemudian berhasil, naikkan `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` sehingga upaya pertama juga menunggu cukup lama.

Sebelum v2.1.242, Claude Code menunggu waktu tunggu permintaan `API_TIMEOUT_MS` penuh, 10 menit secara default, sebelum gagal permintaan streaming yang tidak dijawab. Sebelum v2.1.261, percobaan ulang menunggu batas waktu yang sama dengan upaya pertama dan pesan tidak menunjukkan durasi.

<h3 id="the-response-above-may-be-incomplete">
  The response above may be incomplete
</h3>

Permintaan streaming gagal sementara respons masih berlangsung, setelah Claude menyelesaikan blok teks atau panggilan alat, atau telah memulai satu setelah menyelesaikan pemikirannya. Mengirim ulang permintaan dapat menjalankan panggilan alat yang sama dua kali, jadi Claude Code menyimpan output yang Claude selesaikan dan menambahkan pemberitahuan ini alih-alih membuang giliran. Varian mana yang Anda lihat menyebutkan penyebabnya:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection lost mid-response. The response above may be incomplete.
API Error: Your computer went to sleep mid-response. The response above may be incomplete.
API Error: The response stopped arriving. The response above may be incomplete.
```

* `Server error mid-response`: kesalahan server overloaded atau 5xx mid-stream. Varian ini memerlukan Claude Code v2.1.199 atau lebih baru; sebelumnya kasus itu membuang output parsial dan melaporkan seluruh giliran sebagai kesalahan.
* `Connection lost mid-response`: koneksi putus.
* `Your computer went to sleep mid-response`: Claude Code mendeteksi bahwa komputer Anda tertidur saat respons streaming. Setelah komputer Anda bangun, Claude Code memperlakukan koneksi sebagai rusak dan berhenti membacanya.
* `The response stopped arriving`: koneksi tetap terbuka tetapi berhenti mengirimkan data, jadi watchdog streaming idle membatalkannya. Sebelum v2.1.222, Claude Code juga dapat melaporkan kegagalan ini pada koneksi [gateway](/docs/id/gateways) yang dicapai melalui `ANTHROPIC_BASE_URL` atau `ANTHROPIC_AWS_BASE_URL` sementara ping keep-alive server masih tiba, karena hanya menghitung peristiwa respons yang diurai di sana; upgrade menghentikan waktu tunggu palsu itu di rute tersebut. Gateway yang dicapai melalui URL dasar penyedia seperti `ANTHROPIC_BEDROCK_BASE_URL` tidak dibungkus oleh watchdog byte; lihat [Streaming idle watchdogs](/docs/id/network-config#streaming-idle-watchdogs).

Sebelum v2.1.227, `Connection lost mid-response` membaca `Connection closed mid-response` dan `The response stopped arriving` membaca `Response stalled mid-stream`.

Dalam empat kasus, Claude Code menangani kegagalan tanpa menampilkan pemberitahuan ini segera:

* Sebelumnya dalam respons, Claude Code baik mencoba ulang kegagalan atau mengakhiri giliran dengan kesalahan yang berbeda. Lihat [Automatic retries](#automatic-retries).
* Ketika salah satu kegagalan ini tiba setelah Claude menyelesaikan respons, Claude Code menyimpan respons lengkap dan mengakhiri giliran secara normal, tanpa pemberitahuan ini. Sebelum v2.1.222, Claude Code menampilkan pemberitahuan ini ketika koneksi putus atau macet setelah respons selesai, dan melaporkan giliran sebagai kesalahan meskipun respons lengkap.
* Dalam [non-interactive session](/docs/id/headless), seperti run `-p`, run [Agent SDK](/docs/id/agent-sdk/overview), atau [cloud session](/docs/id/claude-code-on-the-web), Anda tidak harus mengirim `continue` sendiri ketika respons cut-off berada dalam percakapan utama dan berisi teks tetapi tidak ada panggilan alat: Claude Code menyimpan output parsial dan meminta Claude untuk melanjutkan dari tempat ia berhenti, hingga tiga kali berturut-turut. Anda melihat pemberitahuan ini untuk respons seperti itu hanya setelah Claude Code menggunakan kelanjutan itu. Sebelum v2.1.246, Claude Code mengakhiri giliran non-interaktif dengan pemberitahuan ini pada cut-off pertama.
* Dalam [subagent](/docs/id/sub-agents#api-errors-in-subagents), apakah sesi interaktif atau tidak: ketika respons cut-off-nya berisi teks tetapi tidak ada panggilan alat, Claude Code meminta subagent untuk melanjutkan. Pemberitahuan menjadi pesan terakhir subagent hanya setelah kelanjutan itu digunakan. Sebelum v2.1.257, subagent menampilkan pemberitahuan ini pada cut-off pertama.

**Yang harus dilakukan:**

* Dalam sesi interaktif, baca respons yang tetap di layar: Claude Code menyimpan setiap blok yang Claude selesaikan sebelum kesalahan, tetapi membuang blok akhir yang terputus ketika giliran berakhir, jadi kalimat atau panggilan alat terakhir mungkin hilang. Balas dengan `continue` untuk membuat Claude melanjutkan dari blok terakhir yang diselesaikannya.
* Dalam [non-interactive mode](/docs/id/headless) (`-p`):
  * Dengan output teks default, Claude Code mencetak blok teks terakhir yang diselesaikan yang masih dipegang dari sebelumnya dalam giliran, diikuti oleh pesan ini. Ketika tidak memegang apa pun, Claude Code mencetak pesan ini saja, misalnya karena Claude Code memadatkan percakapan di tengah-giliran dan menghapus teks itu. Sebelum v2.1.219, Claude Code hanya mencetak pesan ini dalam output teks `-p` dan menjatuhkan respons yang sudah dihasilkan.
  * Dengan `--output-format json` atau `stream-json`, Claude Code melaporkan pesan ini di bidang `result`.
  * Untuk melanjutkan giliran setelah koneksi stabil, lanjutkan sesi dan kirim `continue` seperti yang dijelaskan dalam [Continue conversations](/docs/id/headless#continue-conversations).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto mode cannot determine the safety of an action
</h3>

Model yang [auto mode](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) gunakan untuk mengklasifikasikan tindakan tidak dapat menghasilkan keputusan, jadi auto mode tidak menyetujui tindakan secara otomatis. Pesan yang Anda lihat tergantung pada bagaimana pengklasifikasi gagal.

Pembacaan, pencarian, dan pengeditan di dalam direktori kerja Anda melewati pengklasifikasi, jadi mereka terus bekerja dalam semua kasus ini.

Ketika model pengklasifikasi tidak tersedia:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait a moment and then try this action again.
```

Ketika Claude Code dapat menentukan kategori kegagalan, ia menyebutkan kategori dalam tanda kurung setelah `temporarily unavailable`, misalnya `<model> is temporarily unavailable (rate-limited), so auto mode cannot determine the safety of <tool> right now`. Kategorinya adalah `(rate-limited)`, `(overloaded)`, `(server error)`, `(timed out)`, dan `(connection failed)`. Rate-limited, overloaded, dan server errors bersifat sementara, dan mencoba ulang berfungsi. Jika `(timed out)` atau `(connection failed)` terulang, periksa koneksi Anda; lihat [Unable to connect to API](#unable-to-connect-to-api). Sebelum v2.1.229, pesan tidak pernah menyebutkan kategori dan membaca `Wait briefly and then try this action again`.

Ketika tidak ada kategori yang cocok, pesan muncul tanpa kategori dalam tanda kurung; lebih dari satu kegagalan menghasilkan bentuk itu. Di [Amazon Bedrock](/docs/id/amazon-bedrock), termasuk [Mantle endpoint](/docs/id/amazon-bedrock#use-the-mantle-endpoint), itu juga muncul ketika akun AWS Anda tidak dapat memanggil model yang disebutkan dalam pesan, dan kegagalan itu terulang pada setiap percobaan ulang sampai akun Anda diberi akses ke model.

**Yang harus dilakukan:**

* Coba lagi setelah beberapa detik; Claude melihat pesan yang sama dan biasanya mencoba ulang sendiri. Kegagalan sementara tidak terkait dengan [auto mode eligibility](/docs/id/permission-modes#eliminate-prompts-with-auto-mode); Anda tidak perlu mengubah pengaturan
* Jika percobaan ulang terus gagal, lanjutkan dengan tugas read-only dan kembali ke tindakan yang diblokir nanti
* Di Amazon Bedrock, jika pesan kembali pada setiap percobaan ulang, periksa bahwa akun Anda dapat memanggil model yang disebutkan: untuk model Amazon Bedrock standar, konfirmasi [IAM policy](/docs/id/amazon-bedrock#iam-configuration) Anda memungkinkan memanggilnya; untuk ID model Mantle, [hubungi tim akun AWS Anda](/docs/id/amazon-bedrock#mantle-endpoint-errors)

Ketika permintaan pengklasifikasi gagal karena token OAuth Anda kedaluwarsa atau diputar oleh sesi lain, Claude Code menyegarkan token dan mencoba ulang permintaan sekali, jadi kedaluwarsa token rutin tidak muncul sebagai pesan ini. Sebelum v2.1.216, token yang kedaluwarsa atau diputar gagal setiap permintaan pengklasifikasi, dan auto mode menolak setiap tindakan yang diperiksa dengan pesan ini sampai token disegarkan.

Ketika pengklasifikasi mengembalikan respons yang tidak dapat diurai:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Yang harus dilakukan:**

* Coba lagi tindakan; ini biasanya berhasil pada upaya berikutnya
* Jalankan `claude --debug` dan ulangi tindakan untuk melihat respons pengklasifikasi yang mendasar dalam log debug

Ketika pemeriksaan keamanan API terpisah memblokir permintaan pengklasifikasi karena konten percakapan sebelumnya:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

Claude Code menolak tindakan tetapi memberi tahu Claude bahwa ini bukan penilaian bahwa tindakan tidak aman, dan untuk melanjutkan dengan tugas lain daripada mencoba ulang. Penolakan ini tidak dihitung terhadap [auto mode's pause thresholds](/docs/id/permission-modes#when-auto-mode-falls-back). Dalam run `-p` [non-interactive](/docs/id/headless), Claude Code tidak menghentikan run. Apa yang Claude terima tergantung pada tempat ia meminta tindakan:

* Ke [background subagent](/docs/id/sub-agents#run-subagents-in-foreground-or-background) dalam run `-p` tanpa `--input-format stream-json`, Claude Code mengembalikan hasil kesalahan yang berisi `Agent aborted: auto mode classifier request refused by the safety safeguard in headless mode`
* Di tempat lain, termasuk sesi interaktif dan percakapan utama run `-p`, Claude Code mengembalikan penolakan itu ke Claude

Sebelum v2.1.225, Claude Code menghitung penolakan ini terhadap ambang batas jeda dan mengembalikan pesan penolakan yang sama seperti blok pengklasifikasi asli.

**Yang harus dilakukan:**

* Ini bukan keputusan tentang tindakan Anda. Konten yang sudah ada dalam percakapan Anda memicu filter keamanan di API ketika auto mode mengirim percakapan ke pengklasifikasi
* Mencoba ulang tidak akan membantu; konten percakapan yang sama akan memicu filter lagi
* Dalam sesi interaktif, beralih ke [permission mode](/docs/id/permission-modes) yang berbeda sehingga Anda dapat menyetujui tindakan ketika diminta
* Mulai percakapan segar tanpa konten pemicu

Ketika percakapan telah tumbuh lebih besar dari jendela konteks pengklasifikasi:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

Apa yang terjadi pada tindakan tergantung pada tempat Claude memintanya:

* Dalam sesi interaktif, auto mode kembali ke prompt izin normal untuk tindakan itu sehingga Anda dapat menyetujui atau menolaknya secara manual
* Ke [background subagent](/docs/id/sub-agents#run-subagents-in-foreground-or-background) dalam run `-p` [non-interactive](/docs/id/headless) tanpa `--input-format stream-json`, Claude Code mengembalikan hasil kesalahan yang berisi `Agent aborted: auto mode classifier transcript exceeded context window in headless mode`, dan run berlanjut
* Di tempat lain dalam run `-p` tanpa [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags), tidak ada prompt untuk kembali, jadi tindakan tidak berjalan dan run berlanjut

**Yang harus dilakukan:**

* Dalam sesi interaktif, setujui atau tolak tindakan dalam prompt yang muncul
* Dalam sesi interaktif, jalankan `/compact` untuk mengurangi ukuran percakapan sehingga tindakan berikutnya cocok dalam jendela pengklasifikasi lagi

<h3 id="the-server-returned-no-safety-verdict">
  The server returned no safety verdict
</h3>

Di bawah [server-side classifier review](/docs/id/permission-modes#server-side-classifier-review), auto mode menolak tindakan ketika server tidak memberikan putusan untuk itu. Penolakan menyebutkan kategori dalam tanda kurung ketika Claude Code dapat menentukan satu, seperti `(timed out)`:

```text theme={null}
The server-side auto mode classifier gave no verdict (timed out), so auto mode cannot determine the safety of <tool>.
```

Sisa pesan memberi tahu Claude apakah satu percobaan ulang dapat membantu. Sebelum beberapa penolakan ini, Claude Code menunggu sehingga upaya Claude berikutnya tidak mengikuti sekaligus. Selama tunggu dalam sesi interaktif, spinner menunjukkan `Auto mode check unavailable` dengan hitungan mundur, dan menekan `Esc` mengganggu giliran.

Setelah sepuluh respons berturut-turut tanpa putusan, auto mode menghentikan giliran:

```text theme={null}
Auto mode is unavailable — the server returned no safety verdict for the last 10 responses, so Claude stopped. Send a message to try again, or switch out of auto mode.
```

Pesan berhenti muncul di tempat yang berbeda dalam setiap jenis sesi:

* Dalam sesi interaktif, pesan muncul sebagai peringatan dalam transkrip dan giliran berakhir
* Dalam run `-p` [non-interactive](/docs/id/headless), run berakhir dan melaporkan kesalahan eksekusi. Dengan output teks default, pesan mencetak di stderr.
* Ketika [subagent](/docs/id/sub-agents) mencapai batas, subagent berhenti sebelum selesai, dan Claude menerima apa pun yang dihasilkan dengan catatan bahwa auto mode menghentikannya

**Yang harus dilakukan:**

* Kirim pesan lain untuk membuat Claude mencoba lagi. Hitungan respons dimulai dari awal.
* Jika berhenti terulang dan permintaan Anda melewati [LLM gateway or proxy](/docs/id/llm-gateway), periksa apakah itu memotong respons streaming pendek atau menulis ulangnya. [Server-side classifier review](/docs/id/permission-modes#server-side-classifier-review) mengatakan perilaku gateway mana yang menyebabkan penolakan, dan [gateway compatibility guide](/docs/id/llm-gateway-protocol#feature-pass-through) mencantumkan apa yang harus dilewatkan tanpa perubahan.
* Atur `CLAUDE_CODE_AUTO_MODE_SERVER=0` sebelum Anda memulai Claude Code untuk menggunakan permintaan pengklasifikasi miliknya sendiri. Sebelum v2.1.281, Claude Code tidak membaca variabel pada koneksi langsung ke Anthropic API.
* Untuk menyetujui tindakan sendiri, [switch out of auto mode](/docs/id/permission-modes#switch-permission-modes)

Sebelum v2.1.280, Claude Code menolak setiap tindakan dari respons tanpa putusan segera dan tidak pernah menghentikan giliran.

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent terminated early due to an API error
</h3>

Permintaan API [subagent](/docs/id/sub-agents) gagal secara terminal, misalnya karena batas penggunaan tercapai atau percobaan ulang untuk kesalahan server habis, jadi subagent berhenti sebelum menyelesaikan tugasnya. Pesan ini memerlukan Claude Code v2.1.199 atau lebih baru; sebelumnya teks kesalahan API dikembalikan ke Claude seolah-olah itu adalah hasil subagent.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Yang harus dilakukan:**

* Cocokkan detail kesalahan setelah titik dua dengan bagiannya sendiri di halaman ini, seperti [Usage limits](#usage-limits) atau [Server errors](#server-errors), dan ikuti langkah bagian itu
* Setelah kesalahan yang mendasar hilang, minta Claude untuk mencoba ulang tugas atau [resume the subagent](/docs/id/sub-agents#resume-subagents)

Ketika batas laju, overload, atau kesalahan server mengganggu subagent foreground yang sudah menghasilkan output teks, Claude menerima output parsial itu ditandai sebagai tidak lengkap alih-alih kesalahan ini. Subagent yang satu-satunya output adalah panggilan alat mendapat kesalahan ini juga; dalam v2.1.199 bentuk itu mengembalikan hasil parsial kosong. Lihat [API errors in subagents](/docs/id/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Batas penggunaan
</h2>

Sebagian besar kesalahan di bagian ini berarti kuota yang terikat pada akun atau paket Anda telah tercapai. Tiga kesalahan bekerja berbeda: [`Server is temporarily limiting requests`](#server-is-temporarily-limiting-requests) adalah throttle sisi server yang tidak terkait dengan kuota paket Anda, [`Usage credits required for 1M context`](#usage-credits-required-for-1m-context) adalah pemeriksaan hak akses daripada kuota yang habis, dan [`The prompt to confirm went unanswered`](#the-prompt-to-confirm-went-unanswered) berarti prompt persetujuan usage credits ditutup tanpa jawaban, terlepas dari apakah kuota tercapai atau tidak.

<h3 id="youve-hit-your-session-limit">
  You've hit your session limit
</h3>

Paket langganan mencakup tunjangan penggunaan bergulir. Ketika habis, Anda akan melihat salah satu pesan ini:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
You've hit your Sonnet limit · resets 3:45pm
```

Blok Claude Code memblokir permintaan lebih lanjut hingga waktu reset yang ditunjukkan dalam pesan. Batas sesi dan mingguan dibagikan di semua model, jadi beralih model tidak mengembalikan akses. Batas Opus dan Sonnet masing-masing hanya berlaku untuk permintaan ke keluarga model tersebut, jadi beralih ke model di luar keluarga dengan `/model` membuat Anda tetap bekerja.

Dalam sesi interaktif yang masuk dengan langganan claude.ai, Claude Code juga dapat menunggu di sesi terbuka dan melanjutkan tugas yang terputus segera setelah reset. Saat menunggu, baris di bagian bawah sesi berbunyi `Usage limit reached · continuing automatically at 3:45pm · esc to cancel`. Tekan `Esc` pada prompt kosong untuk membatalkan penantian. Lihat [Wait for a usage limit to reset](/docs/id/interactive-mode#wait-for-a-usage-limit-to-reset) untuk melihat apa yang Anda lihat, cara memulai atau membatalkan penantian, dan cara mematikan lanjutan otomatis. Sebelum v2.1.234, Claude Code tidak menawarkan penantian ini.

Penggunaan dihitung terhadap tunjangan sesi dan mingguan pada saat yang sama. Satu ledakan aktivitas berat, seperti fanout alur kerja besar, dapat menghabiskan tunjangan mingguan sebelum jendela sesi direset.

**Yang harus dilakukan:**

* Tunggu waktu reset yang ditunjukkan dalam kesalahan
* Di tab Code dari [Desktop app](/docs/id/desktop), kartu batas sesi menawarkan kotak centang **Auto-continue when limits reset**. Kartu batas mingguan tidak. Ketika dicentang, aplikasi Desktop mencoba kembali giliran yang terputus setelah reset dan menampilkan waktu percobaan kembali pada kartu. Kotak centang Desktop dan pengaturan **Continue automatically at usage limit** CLI di `/config` terpisah, jadi matikan masing-masing secara terpisah.
* Untuk batas Opus atau Sonnet, jalankan `/model` dan beralih ke model di luar keluarga tersebut untuk terus bekerja. Setiap model memiliki cache prompt-nya sendiri, jadi permintaan berikutnya membaca ulang seluruh percakapan tanpa cache hits; lihat [Switching models](/docs/id/prompt-caching#switching-models)
* Jalankan `/usage` untuk melihat batas paket Anda dan kapan mereka direset
* Jalankan `/usage-credits` untuk membeli penggunaan tambahan di Pro dan Max, atau memintanya dari admin Anda di Team dan Enterprise. Lihat [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) untuk cara ini ditagih.
* Untuk meningkatkan paket Anda untuk batas dasar yang lebih tinggi, lihat [claude.com/pricing](https://claude.com/pricing)

Sebelum jendela habis, Claude Code dapat memperingatkan Anda bahwa Anda telah menggunakan sebagian besar, dengan pesan seperti `You've used 85% of your session limit · resets 3:45pm`. Untuk memantau tunjangan sisa Anda secara terus-menerus, tambahkan bidang `rate_limits` ke [custom status line](/docs/id/statusline#rate-limit-usage), atau di aplikasi Desktop klik [usage ring](/docs/id/desktop#check-usage) di sebelah pemilih model.

<h3 id="usage-credits-required-for-1m-context">
  Usage credits required for 1M context
</h3>

Model yang dipilih menggunakan jendela konteks diperluas 1M-token, dan paket Anda hanya mencakupnya melalui usage credits.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on (they take effect after you restart Claude Code), or /model to switch to standard context
```

Ini adalah pemeriksaan hak akses, bukan kehabisan kuota. Ini muncul bahkan ketika tunjangan sesi dan mingguan Anda memiliki kapasitas yang tersisa. Lihat [Extended context](/docs/id/model-config#extended-context) untuk paket mana yang mencakup konteks 1M secara langsung dan mana yang memerlukan usage credits. Claude Code menjalankan pemeriksaan ini ketika Anda memilih model dengan `/model`, dan hanya pada koneksi langsung ke API Anthropic; jika Anda menunjuk `ANTHROPIC_BASE_URL` ke [LLM gateway](/docs/id/llm-gateway), `/model` memungkinkan pemilihan `[1m]` dan gateway memutuskan apakah permintaan berhasil.

Ketika kesalahan ini muncul di tengah percakapan karena konteks tumbuh melampaui 200K token, Claude Code secara otomatis mengompres percakapan kembali di bawah batas konteks standar dan menjaga sesi pada batas itu sesudahnya, jadi tidak ada tindakan yang diperlukan. Pada versi sebelum v2.1.172, kesalahan berulang pada setiap permintaan berikutnya termasuk `/compact`; jalankan `/clear` pada versi tersebut untuk pulih. Langkah-langkah di bawah berlaku ketika Anda secara eksplisit memilih model `[1m]`.

**Yang harus dilakukan:**

* Jalankan `/model` dan pilih varian tanpa akhiran `[1m]` untuk kembali ke jendela konteks standar
* Di mana pesan menyebutkan `/usage-credits`, jalankan untuk mengaktifkan penagihan terukur untuk varian 1M di Pro dan Max, atau untuk meminta usage credits dari admin Anda di Team dan Enterprise. Setelah usage credits aktif, mulai ulang Claude Code atau mulai sesi baru, mana pun yang pesan katakan. Sampai saat itu, sesi tetap pada batas konteks standar.
* Jika kesalahan berlanjut setelah `/model`, ID model 1M mungkin diatur di tempat lain. Lihat [Setting your model](/docs/id/model-config#setting-your-model) untuk lokasi konfigurasi yang harus diperiksa dalam urutan prioritas.
* Untuk menghapus varian 1M dari pemilih model sepenuhnya, atur [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/id/env-vars)

Sebelum v2.1.268, pesan berakhir dengan `run /usage-credits to turn them on, or /model to switch to standard context` dan tidak menyebutkan restart.

<h3 id="the-prompt-to-confirm-went-unanswered">
  The prompt to confirm went unanswered
</h3>

Jika akun Anda memerlukan [Fable usage-credits consent](/docs/id/model-config#fable-and-usage-credits), Claude Code meminta Anda untuk mengonfirmasi sebelum permintaan Fable menagih usage credits. Ketika tidak ada yang menjawab prompt persetujuan itu dalam sesi yang mungkin tidak memiliki siapa pun di terminalnya, Claude Code menutup prompt dan mengakhiri giliran dengan salah satu pesan ini:

```text theme={null}
Fable limit reached · continuing on Fable 5.1 uses usage credits, and the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
Fable 5.1 now uses usage credits · the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
```

Pesan menyebutkan model Fable sesi, jadi di Fable 5 mereka membaca `continuing on Fable 5` dan `Fable 5 now uses usage credits`. Sebelum v2.1.257, pesan pertama dimulai dengan `Fable 5 limit reached`.

Ini terjadi dalam sesi [Remote Control](/docs/id/remote-control), [background sessions](/docs/id/agent-view), dan sesi rekan tim [agent team](/docs/id/agent-teams). Claude Code menampilkan prompt persetujuan hanya dalam tampilan interaktif sesi sendiri: terminal tempat ia berjalan, atau, untuk sesi latar belakang, [agents view](/docs/id/agent-view) setelah Anda melampirkan. Klien Remote Control tidak dapat menampilkannya. Claude Code menutup prompt pada batas waktu [`dialogExpiry`](/docs/id/settings-reference#dialogexpiry), lima menit secara default, atau segera setelah prompt baru tiba sementara tidak ada yang mengetik di terminal itu, seperti prompt yang dikirim dari klien Remote Control. Mengetik di terminal tempat sesi berjalan membatalkan batas waktu, dan Claude Code menunggu jawaban Anda. Dalam tampilan terlampir sesi latar belakang, mengetik tidak membatalkan batas waktu, dan prompt baru masih menutup prompt persetujuan, jadi jawab sebelum salah satu terjadi. Claude Code tidak mengirim apa pun dan menjaga model Anda, jadi ketika Anda mengirim prompt berikutnya, Claude Code menampilkan prompt persetujuan lagi.

**Yang harus dilakukan:**

* Di terminal tempat sesi berjalan, kirim prompt lain dan jawab prompt persetujuan ketika muncul kembali. Untuk sesi latar belakang, lampirkan terlebih dahulu dari [agents view](/docs/id/agent-view). Mengirim ulang dari klien Remote Control menampilkan pesan ini lagi, karena klien tidak dapat menampilkan prompt.
* Jalankan `/model` untuk beralih ke model yang tidak menagih usage credits
* Untuk memberi diri Anda lebih banyak waktu untuk mencapai terminal itu, atur [`dialogExpiry`](/docs/id/settings-reference#dialogexpiry) ke nilai yang lebih lama atau `"never"`

Sebelum v2.1.236, pesan ini tidak muncul: sementara klien Remote Control terhubung, Claude Code menunggu 60 detik untuk jawaban dan kemudian melanjutkan giliran pada model default Anda.

<h3 id="server-is-temporarily-limiting-requests">
  Server is temporarily limiting requests
</h3>

API menerapkan throttle jangka pendek yang tidak terkait dengan kuota paket Anda.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code membedakan ini dari batas paket Anda dengan tidak adanya header kuota terpadu yang dibawa respons batas nyata. Mulai dari v2.1.199 ini [dicoba ulang secara otomatis](#automatic-retries) dengan backoff sebelum ditampilkan, terlepas dari cara Anda mengautentikasi. Pada versi sebelumnya, sesi yang masuk dengan langganan claude.ai gagal pada giliran pertama; hanya API key dan Enterprise sign-ins yang mencoba ulangnya.

**Yang harus dilakukan:**

* Tunggu sebentar dan coba lagi
* Periksa [status.claude.com](https://status.claude.com) jika terus berlanjut

<h3 id="request-rejected-429">
  Request rejected (429)
</h3>

Anda telah mencapai batas laju yang dikonfigurasi untuk kunci API, proyek Amazon Bedrock, atau proyek Google Cloud Anda.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

Kalimat di akhir menyebutkan di mana memeriksa kesehatan layanan dan bervariasi menurut penyedia. Konfigurasi Amazon Bedrock, Agent Platform Google Cloud, dan Microsoft Foundry menyebutkan status layanan penyedia itu sebagai gantinya dari halaman status Anthropic. `ANTHROPIC_BASE_URL` kustom menyebutkan host gateway.

**Yang harus dilakukan:**

* Jalankan `/status` dan konfirmasi kredensial aktif adalah yang Anda harapkan. `ANTHROPIC_API_KEY` yang tersesat di lingkungan Anda dapat merutekan permintaan melalui kunci tingkat rendah daripada langganan Anda.
* Periksa konsol penyedia Anda untuk batas aktif dan minta tingkat yang lebih tinggi jika diperlukan
* Untuk kunci API Anthropic, lihat [rate limits reference](https://platform.claude.com/docs/en/api/rate-limits) untuk cara kerja tingkat dan cara menetapkan batas per-workspace
* Kurangi concurrency: turunkan [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/id/env-vars), hindari menjalankan banyak subagen paralel, atau beralih ke model yang lebih kecil dengan `/model` untuk run skrip volume tinggi

<h3 id="youve-hit-your-monthly-spend-limit">
  You've hit your monthly spend limit
</h3>

Penggunaan yang disertakan paket Anda tidak dapat menutupi permintaan ini, dan [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) yang sebaliknya akan membayarnya telah mencapai batas pengeluaran. Itu terjadi ketika salah satu jendela penggunaan paket Anda telah habis, atau ketika permintaan adalah permintaan yang hanya dibayar oleh usage credits, seperti permintaan ke model yang [bills to usage credits](/docs/id/model-config#fable-and-usage-credits). Pesan menyebutkan batas siapa yang memblokir Anda. Teks setelah `·` mengatakan cara meningkatkan batas itu, dan bervariasi dengan paket Anda dan apakah Anda mengelola penagihan:

```text theme={null}
You've hit your monthly spend limit · raise it at claude.ai/settings/usage
You've hit your individual spend limit · ask your admin for a higher limit
You've hit your org's monthly spend limit · visit claude.ai/admin-settings/usage to raise it
You've hit your team's shared budget · ask your admin to raise it at claude.ai/admin-settings/usage
You've hit your channel's monthly spend limit · an org owner or channel manager can raise it in the channel's Claude settings
```

`team's shared budget` adalah anggaran terpusat yang admin tetapkan untuk grup yang Anda termasuk; pesan tidak menyebutkan grup. `channel's monthly spend limit` adalah anggaran dari satu saluran Slack tempat sesi berjalan, jadi organisasi Anda mungkin masih memiliki anggaran di luar itu.

Ketika salah satu jendela paket Anda adalah yang habis, pesan juga mengatakan kapan jendela itu direset, misalnya `· your session limit resets 3:45pm`, dan akses kembali kemudian tanpa siapa pun menaikkan batas. Pada organisasi dengan penagihan berbasis penggunaan, pesan mengatakan `usage limit` sebagai gantinya dari `spend limit`, seperti dalam `You've hit your individual usage limit`.

Sebelum v2.1.239, pesan tidak menyebutkan waktu reset jendela paket. Sebelum v2.1.268, anggaran terpusat grup menghasilkan pesan `individual spend limit` daripada `team's shared budget`.

Jika Anda terhubung melalui gateway aplikasi Claude dan melihat `spend limit reached` huruf kecil, itu adalah batas operator gateway Anda sebagai gantinya; lihat [Spend limit reached](#spend-limit-reached).

**Yang harus dilakukan:**

* Di Pro dan Max, tingkatkan batas pengeluaran bulanan Anda di [**Settings > Usage**](https://claude.ai/settings/usage) di claude.ai, atau jalankan `/usage-credits`
* Di Team dan Enterprise, tingkatkan batas di [**Admin settings > Usage**](https://claude.ai/admin-settings/usage) jika Anda mengelola penagihan, atau minta admin untuk melakukannya. `/usage-credits` mengirim permintaan itu ke admin Anda untuk Anda
* Untuk batas saluran, minta pemilik org atau manajer saluran untuk menaikkannya di claude.ai. Lihat [Per-channel limits](https://claude.com/docs/claude-tag/admins/set-spend-limit#per-channel-limits) dalam dokumentasi Claude Tag
* Jika pesan menyebutkan waktu reset untuk jendela paket Anda, Anda dapat menunggunya sebagai gantinya
* Jalankan `/usage` untuk melihat jendela paket Anda dan kapan masing-masing direset

<h3 id="spend-limit-reached">
  Spend limit reached
</h3>

Anda terhubung melalui [Claude apps gateway](/docs/id/claude-apps-gateway) dan telah melampaui [spend cap](/docs/id/claude-apps-gateway-spend-limits) yang operator gateway Anda tetapkan. Gateway memblokir permintaan Anda sampai periode yang dinamai direset atau operator menaikkan batas. Ini menandai setiap respons `429` yang diblokir `x-should-retry: false`, jadi Claude Code menampilkan pesan ini tanpa mencoba ulang.

```text theme={null}
spend limit reached (daily; resets 2026-08-09 00:00 UTC)
```

Pesan menyebutkan periode batas dan waktu reset, dan ketika operator mengonfigurasi `blocked_message`, instruksi mereka mengikutinya. Sebelum v2.1.225, pesan hanya berbunyi `spend limit reached`; gateway pada versi yang lebih lama masih mengirim bentuk yang lebih pendek itu.

**Yang harus dilakukan:**

* Tunggu waktu reset yang pesan namakan, atau ikuti instruksi operator jika pesan membawanya
* Minta operator gateway Anda untuk menaikkan batas jika Anda mencapainya secara rutin

Pesan terkait, `spend limit unavailable`, berarti gateway tidak dapat membaca catatan pengeluarannya dan memblokir permintaan sebagai tindakan pencegahan daripada atas batas Anda. Biasanya ini hilang dengan sendirinya; jika terus berlanjut, beri tahu operator gateway Anda.

<h3 id="credit-balance-is-too-low">
  Credit balance is too low
</h3>

Organisasi Console Anda telah kehabisan kredit prabayar, atau Claude Code mengirim permintaan Anda dengan kunci API Console ketika Anda bermaksud menggunakan langganan Anda.

```text theme={null}
Credit balance is too low
```

**Yang harus dilakukan:**

* Jika Anda memiliki paket Pro, Max, Team, atau Enterprise dan melihat ini, jalankan `/status` dan periksa baris `API key`. `ANTHROPIC_API_KEY` yang disetujui di lingkungan Anda merutekan permintaan melalui kunci itu daripada langganan Anda. Batalkan pengaturannya di shell saat ini dan hapus dari profil shell Anda, kemudian luncurkan ulang `claude`. Jalankan `/login` jika Anda belum masuk dengan langganan Anda.
* Tambahkan kredit di [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), dan pertimbangkan untuk mengaktifkan auto-reload di sana sehingga saldo diisi ulang sebelum mencapai nol
* Atur batas pengeluaran per-workspace di Console untuk mencegah satu proyek menguras saldo org. Lihat [Manage costs effectively](/docs/id/costs).

<h3 id="could-not-update-your-spend-limit">
  Could not update your spend limit
</h3>

Server menolak perubahan batas pengeluaran yang Anda buat dari prompt yang muncul ketika Anda mencapai batas pengeluaran Anda.

```text theme={null}
Could not update your spend limit: <reason from the server>
```

Ketika server menjelaskan penolakan, pesan berakhir dengan alasan itu, dan mencoba ulang nilai yang sama gagal lagi. Ketika kegagalan tidak memiliki alasan yang disediakan server, seperti koneksi yang terputus, pesan berbunyi `Could not update your spend limit. Press Enter to retry.` dan mencoba ulang dapat berhasil. Sebelum v2.1.216, Claude Code menampilkan bentuk generik untuk setiap kegagalan.

**Yang harus dilakukan:**

* Jika pesan menyertakan alasan, pilih batas yang memuaskannya, seperti jumlah yang lebih rendah
* Jika pesan hanya menampilkan bentuk generik, coba ulang; kegagalan mungkin bersifat sementara
* Jika perubahan terus gagal, buatlah dari [claude.ai billing settings](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) Anda di browser sebagai gantinya

<h2 id="authentication-errors">
  Kesalahan autentikasi
</h2>

Kesalahan ini berarti Claude Code tidak dapat membuktikan identitas Anda kepada API. Jalankan `/status` kapan saja untuk melihat kredensial mana yang saat ini aktif.

<h3 id="not-logged-in">
  Belum masuk
</h3>

Tidak ada kredensial yang valid tersedia untuk sesi ini.

```text theme={null}
Not logged in · Please run /login
```

**Yang harus dilakukan:**

* Jalankan `/login` untuk autentikasi dengan langganan Claude Anda atau akun Console
* Jika Anda mengharapkan variabel lingkungan untuk mengautentikasi Anda, konfirmasi bahwa `ANTHROPIC_API_KEY` diatur dan diekspor di shell tempat Anda meluncurkan `claude`
* Untuk CI atau otomasi di mana login interaktif tidak mungkin, konfigurasikan skrip [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) yang mengambil kunci saat startup
* Lihat [Prioritas autentikasi](/docs/id/authentication#authentication-precedence) untuk memahami kredensial mana yang digunakan Claude Code ketika beberapa ada

Jika Anda diminta untuk masuk berulang kali, lihat [Belum masuk atau token kedaluwarsa](/docs/id/troubleshoot-install#not-logged-in-or-token-expired) untuk pemeriksaan jam sistem dan langkah pemulihan penyimpanan kredensial macOS.

<h3 id="could-not-resolve-authentication-method">
  Tidak dapat menyelesaikan metode autentikasi
</h3>

Sesi mencapai klien API tanpa kredensial apa pun. [Sesi latar belakang](/docs/id/agent-view) dan sesi cloud menampilkan pesan ini ketika pekerja dimulai tanpa kredensial. Jalankan interaktif, `-p`, dan Agent SDK melaporkan kondisi yang sama seperti [Belum masuk](#not-logged-in) dan menulis string ini hanya ke log debug mereka, jadi jika Anda menemukannya di sana, ikuti entri itu sebagai gantinya.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

Pada versi saat ini, kesalahan berarti tidak ada kredensial yang tersedia untuk proses pekerja. Sebelum v2.1.174, sesi latar belakang yang ditugaskan ke pekerja pra-inisialisasi yang menganggur dapat gagal dengan cara ini bahkan ketika kredensial yang valid dikonfigurasi. Sebelum v2.1.176, sesi cloud yang menganggur sebelum diklaim juga dapat. Tingkatkan untuk memulihkan.

**Yang harus dilakukan:**

* Tingkatkan ke v2.1.176 atau lebih baru jika ini muncul dalam sesi latar belakang atau cloud dan kredensial Anda sudah dikonfigurasi
* Konfirmasi bahwa `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN`, atau kredensial penyedia cloud Anda diatur di lingkungan yang meluncurkan pekerja, bukan hanya di shell interaktif Anda
* Untuk Agent SDK, lihat [pengaturan autentikasi dalam panduan cepat](/docs/id/agent-sdk/quickstart#setup)
* Jalankan `/status` dalam sesi interaktif di lingkungan yang sama untuk mengonfirmasi sumber kredensial mana yang diselesaikan

<h3 id="invalid-api-key">
  Kunci API tidak valid
</h3>

Variabel lingkungan `ANTHROPIC_API_KEY` atau skrip `apiKeyHelper` mengembalikan kunci yang ditolak API, atau Claude Code memblokir kunci dari `ANTHROPIC_API_KEY` sebelum mengirimnya.

```text theme={null}
Invalid API key · Fix external API key
```

Ketika pesan berlanjut melampaui `Fix external API key` dengan deskripsi seperti `Invalid X-Api-Key header value from ANTHROPIC_API_KEY: it contains a line break at character 41 (120 characters on 2 lines).`, API tidak pernah melihat kunci. Claude Code menemukan karakter yang tidak dapat dibawa oleh header HTTP dan menghentikan permintaan sebelum mengirimnya. Lihat [Nilai header permintaan tidak valid](#invalid-request-header-value) untuk cara membaca deskripsi dan memperbaiki nilai.

**Yang harus dilakukan:**

* Periksa kesalahan ketik dan konfirmasi bahwa kunci belum dicabut di [Console](https://platform.claude.com/settings/keys)
* Di shell yang sama, jalankan `env | grep ANTHROPIC`, atau di PowerShell `Get-ChildItem Env:ANTHROPIC*`. Alat seperti direnv, plugin shell dotenv, dan terminal IDE dapat memuat kunci lama dari file `.env` di proyek Anda tanpa Anda mengaturnya secara eksplisit.
* Batalkan pengaturan `ANTHROPIC_API_KEY` dan jalankan `/login` untuk menggunakan autentikasi langganan sebagai gantinya
* Jika kunci berasal dari skrip [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper), jalankan skrip secara langsung untuk mengonfirmasi bahwa skrip mencetak kunci yang valid ke stdout
* Jalankan `/status` untuk mengonfirmasi sumber kredensial mana yang sebenarnya digunakan Claude Code

<h3 id="your-apikeyhelper-script-is-failing">
  Skrip apiKeyHelper Anda gagal
</h3>

Claude Code menjalankan perintah dalam pengaturan [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) Anda dan tidak mendapatkan kunci kembali. Tanpa satu, permintaan mencapai API dengan kredensial placeholder, dan API menolaknya dengan `401`. Panel `Authentication` di terminal menunjukkan mana dari ini yang terjadi:

* Perintah keluar dengan kesalahan atau waktu habis
* Perintah tidak mencetak apa pun ke stdout
* Perintah mencetak sesuatu selain kunci, seperti spanduk login atau baris log. Panel menunjukkan `returned output that cannot be used as an API key` dan mengatakan apa yang salah, tanpa mengulangi output. Sebelum v2.1.227, Claude Code mengirim apa pun yang dicetak perintah, setelah memangkas spasi putih di sekitarnya.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Dalam [mode non-interaktif](/docs/id/headless), stderr juga membawa alasan spesifik, diawali dengan `apiKeyHelper failed:`.

Claude Code menjalankan kembali skrip dan mencoba kembali permintaan hingga dua kali lagi sebelum menampilkan pesan ini, jadi kegagalan muncul dalam tiga upaya. Sebelum v2.1.208, Claude Code menghabiskan [anggaran percobaan ulang](#automatic-retries) penuh mengirim ulang permintaan dengan kredensial placeholder dan kemudian melaporkan kesalahan autentikasi `401` generik sebagai gantinya dari kegagalan skrip.

Menjalankan `/login` tidak membantu di sini: output pembantu [mengambil prioritas](/docs/id/authentication#authentication-precedence) atas login yang disimpan selama pengaturan ada.

**Yang harus dilakukan:**

* Jalankan perintah yang dikonfigurasi dalam `apiKeyHelper` secara langsung di shell Anda untuk mereproduksi kegagalan
* Jika perintah melaporkan sesi yang kedaluwarsa, autentikasi ulang dengan penyedia kredensial Anda, misalnya dengan masuk kembali ke SSO atau brankas rahasia Anda
* Perbaiki perintah sehingga hanya mencetak kunci ke stdout, sebagai token ASCII yang dapat dicetak tunggal hingga 16.384 karakter, dan keluar dengan kode 0. Lihat [putar kredensial dengan apiKeyHelper](/docs/id/llm-gateway-connect#rotate-credentials-with-apikeyhelper) untuk pengaturan yang berfungsi.
* Jalankan `/status` untuk melihat kegagalan dan konfirmkan bahwa `apiKeyHelper` adalah sumber kredensial aktif. Baris `apiKeyHelper` menunjukkan `Failing` dengan detail kegagalan terakhir, seperti kode keluar dan output kesalahan perintah, dan hilang setelah jalankan yang berhasil berikutnya. Sebelum v2.1.274, `/status` hanya menunjukkan sumber kredensial, bukan kegagalan.
* Setiap kali perintah gagal, kode keluar dan output kesalahannya juga muncul dalam panel `Authentication` di terminal. Sebelum v2.1.212, panel berjudul `Cloud authentication`.

<h3 id="invalid-request-header-value">
  Nilai header permintaan tidak valid
</h3>

Nilai yang Claude Code akan kirim sebagai header permintaan berisi karakter yang tidak dapat dibawa oleh header HTTP: jeda baris, byte NUL, atau karakter di atas `U+00FF`, seperti tanda kutip melengkung atau spasi lebar nol. Claude Code menghentikan permintaan sebelum apa pun dikirim dan menamai variabel atau pengaturan untuk diperbaiki. Penyebab umum adalah kredensial yang ditempel dari dokumen atau obrolan yang membawa karakter tak terlihat atau jeda baris yang tersesat.

Claude Code menjalankan pemeriksaan ini ketika mengirim permintaan ke Claude API secara langsung atau melalui [gateway LLM](/docs/id/llm-gateway). Pada penyedia cloud pihak ketiga seperti [Amazon Bedrock](/docs/id/amazon-bedrock), Claude Code tidak menjalankannya sebelum mengirim.

```text theme={null}
Invalid auth token · Fix external auth token
Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable
Invalid request header from the environment · Fix the environment variable
```

Bagian pertama dari pesan tergantung pada tempat nilai buruk berasal:

* `Invalid auth token`: token pembawa dari [`ANTHROPIC_AUTH_TOKEN`](/docs/id/env-vars) atau [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/id/env-vars)
* `Invalid ANTHROPIC_CUSTOM_HEADERS`: nama header atau nilai yang Anda atur dalam [`ANTHROPIC_CUSTOM_HEADERS`](/docs/id/env-vars). Deskripsi menghitung pasangan `Name: Value` mana yang bersalah, seperti `distinct header 2 of 3 parsed from ANTHROPIC_CUSTOM_HEADERS`, tanpa mengulangi nama atau nilai, karena Anda memilih keduanya.
* `Invalid request header from the environment`: nilai yang Claude Code salin ke header permintaan dari variabel lingkungan lain, seperti `CLAUDE_AGENT_SDK_CLIENT_APP`. Deskripsi menamai variabel untuk diperbaiki.

Claude Code melaporkan `ANTHROPIC_API_KEY` buruk yang ditangkap oleh pemeriksaan ini sebagai [Kunci API tidak valid](#invalid-api-key), dengan deskripsi trailing yang sama. Ini melaporkan kredensial `/login` yang disimpan buruk sebagai [Belum masuk](#not-logged-in) sebagai gantinya; jalankan `/login` untuk menyimpan yang baru. Output skrip [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) tidak pernah mencapai pemeriksaan ini: Claude Code memvalidasinya ketika skrip berjalan, dan output yang tidak dapat dibawa header HTTP gagal dengan [Skrip apiKeyHelper Anda gagal](#your-apikeyhelper-script-is-failing).

Setelah `·` kedua, pesan menjelaskan masalahnya, seperti dalam contoh lengkap ini:

```text theme={null}
Invalid auth token · Fix external auth token · Invalid Authorization header value from ANTHROPIC_AUTH_TOKEN: it contains a line break at character 41 (120 characters on 2 lines).
```

Posisi menghitung karakter mulai dari satu. Deskripsi dibangun dari frasa tetap dan hitungan karakter, jadi tidak pernah menyertakan nilai itu sendiri. Ini menamai karakter yang menyinggung hanya ketika itu adalah karakter tak terlihat atau tipografi yang terkenal, seperti tanda urutan byte, spasi lebar nol, atau tanda kutip melengkung, dan melaporkan apa pun yang lain sebagai `a non-ASCII character`.

**Yang harus dilakukan:**

* Atur ulang variabel atau pengaturan yang dinamai pesan, mengetik ulang karakter di sekitar posisi yang dilaporkan daripada menempel dari sumber yang sama lagi
* Untuk `ANTHROPIC_CUSTOM_HEADERS`, simpan satu pasangan `Name: Value` per baris dan tulis ulang pasangan yang dihitung pesan
* Jalankan `/status` untuk mengonfirmasi sumber kredensial mana yang aktif

<h3 id="this-organization-has-been-disabled">
  Organisasi ini telah dinonaktifkan
</h3>

Claude Code menggunakan `ANTHROPIC_API_KEY` lama dari organisasi Console yang dinonaktifkan. Ketika Anda memiliki login langganan yang disimpan, kunci menimpanya.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your subscription instead
Your ANTHROPIC_API_KEY belongs to a disabled organization · Update or unset the environment variable
API Error: 400 ... This organization has been disabled.
```

Petunjuk setelah `·` tergantung pada kredensial yang disimpan Anda: bentuk pertama muncul ketika `/login` yang disimpan dapat mengambil alih setelah Anda membatalkan pengaturan kunci, dan yang kedua ketika kunci adalah satu-satunya kredensial Anda.

Variabel lingkungan mengambil prioritas atas `/login`, jadi kunci yang diekspor dalam profil shell Anda atau dimuat dari file `.env` digunakan bahkan ketika Anda memiliki langganan Pro atau Max yang berfungsi. Dalam mode non-interaktif (`-p`), kunci selalu digunakan saat ada.

**Yang harus dilakukan:**

* Batalkan pengaturan `ANTHROPIC_API_KEY` di shell saat ini dan hapus dari profil shell Anda, kemudian luncurkan ulang `claude`
* Jika pesan mengatakan `Update or unset`, Anda tidak memiliki login yang disimpan untuk kembali. Batalkan pengaturan kunci dan jalankan `/login`, atau ganti kunci dengan yang dari organisasi Console aktif.
* Jalankan `/status` setelahnya untuk mengonfirmasi kredensial aktif adalah langganan Anda
* Jika tidak ada variabel lingkungan yang diatur dan kesalahan berlanjut, organisasi yang dinonaktifkan adalah yang terikat pada `/login` Anda. Hubungi dukungan atau masuk dengan akun berbeda.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Organisasi Anda telah menonaktifkan autentikasi kunci API
</h3>

Pesan ini memerlukan Claude Code v2.1.169 atau lebih baru. Admin organisasi Console Anda telah mematikan autentikasi kunci API, jadi API menolak kunci yang dikirim Claude Code. Petunjuk pemulihan setelah `·` bervariasi tergantung pada tempat kunci berasal:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Variabel lingkungan dan `apiKeyHelper` mengambil prioritas atas `/login`, jadi menjalankan `/login` saja tidak membantu sementara salah satu masih memasok kunci. Lihat [Prioritas autentikasi](/docs/id/authentication#authentication-precedence).

**Yang harus dilakukan:**

* Jika pesan menamai `ANTHROPIC_API_KEY`, batalkan pengaturannya di shell saat ini dan hapus dari profil shell atau file `.env` Anda, kemudian luncurkan ulang `claude`
* Jika pesan menamai `apiKeyHelper`, hapus pengaturan [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper) dari `settings.json` Anda
* Jalankan `/login` untuk masuk dengan akun claude.ai Anda
* Jalankan `/status` setelahnya untuk mengonfirmasi kredensial aktif adalah langganan Anda daripada kunci API
* Jika Anda memerlukan autentikasi kunci API untuk otomasi, minta admin organisasi Anda untuk mengaktifkannya kembali di Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Organisasi Anda telah menonaktifkan akses langganan Claude
</h3>

Organisasi Claude Anda tidak memungkinkan masuk ke Claude Code dengan login langganan. Menjalankan `/login` lagi dengan akun yang sama mengembalikan kesalahan yang sama.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Ini adalah pengaturan organisasi sisi server, jadi tidak dapat ditimpa dari pengaturan lokal, variabel lingkungan, atau bendera CLI.

Agent SDK dan mode non-interaktif `-p` menampilkan ini sebagai kode kesalahan `oauth_org_not_allowed`.

**Yang harus dilakukan:**

* Minta admin Anda untuk mengaktifkan akses Claude Code untuk organisasi Anda
* Autentikasi dengan kunci API Console sebagai gantinya dari langganan Anda. Lihat [Autentikasi Claude Console](/docs/id/authentication#claude-console-authentication) untuk pengaturan.
* Jika Anda adalah admin dan tidak melihat opsi untuk mengaktifkan akses, hubungi [dukungan Anthropic](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Rutinitas dinonaktifkan oleh kebijakan organisasi Anda
</h3>

Pemilik dalam organisasi Tim atau Enterprise Anda telah mematikan rutinitas di tingkat organisasi. Kesalahan muncul ketika Anda mencoba membuat atau menjalankan rutinitas, misalnya dari UI [Rutinitas](/docs/id/routines) di claude.ai/code. Pada Claude Code v2.1.227 atau lebih baru, pengaturan yang sama juga [menyembunyikan `/schedule`](/docs/id/routines#troubleshooting) di CLI.

```text theme={null}
Routines are disabled by your organization's policy.
```

Ini adalah pengaturan sisi server, jadi tidak dapat ditimpa dari pengaturan lokal, variabel lingkungan, atau bendera CLI.

**Yang harus dilakukan:**

* Minta Pemilik dalam organisasi Anda untuk mengaktifkan toggle **Routines** di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Untuk pekerjaan terjadwal satu kali yang tidak memerlukan rutinitas tingkat organisasi, lihat [tugas terjadwal](/docs/id/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control memerlukan Anthropic API
</h3>

Sesi tidak berbicara langsung dengan Anthropic API, jadi tidak ada backend claude.ai untuk [Remote Control](/docs/id/remote-control) untuk dipasangkan.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com. CLAUDE_CODE_USE_BEDROCK is set, so this session is using Amazon Bedrock — unset it (or run in a shell without it) to use Remote Control.
```

Kalimat kedua menjelaskan apa yang mengarahkan sesi menjauh dari Anthropic API; sebelum v2.1.219, pesan hanya kalimat pertama. Tergantung pada penyebabnya, pesan menamai:

* Variabel penyedia `CLAUDE_CODE_USE_*`, seperti `CLAUDE_CODE_USE_BEDROCK` untuk [Amazon Bedrock](/docs/id/amazon-bedrock) atau `CLAUDE_CODE_USE_VERTEX` untuk [Platform Agen Google Cloud](/docs/id/google-vertex-ai)
* [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) menunjuk ke host selain `api.anthropic.com`, seperti [gateway LLM](/docs/id/llm-gateway) atau proxy, bahkan ketika Anda masuk dengan claude.ai; sebelum v2.1.196, URL dasar khusus tidak memblokir Remote Control
* `ANTHROPIC_UNIX_SOCKET` diatur, jadi sesi mengirim permintaannya melalui soket lokal daripada ke `api.anthropic.com`
* Masuk [gateway cloud](/docs/id/claude-apps-gateway) perusahaan melalui `/login`, yang tidak mendukung Remote Control dan tidak memiliki variabel untuk dibatalkan pengaturannya

**Yang harus dilakukan:**

* Batalkan pengaturan variabel yang dinamai pesan, seperti `CLAUDE_CODE_USE_BEDROCK` atau `ANTHROPIC_BASE_URL`, dan mulai ulang sesi, atau mulai Remote Control dari sesi yang berbicara langsung dengan Anthropic API
* Jika variabel tidak diatur di shell Anda, periksa kunci `env` dalam [file pengaturan](/docs/id/settings#where-settings-live) Anda, yang menerapkan variabel lingkungan ke setiap sesi
* Untuk pesan startup Remote Control ini dan lainnya, lihat [Troubleshoot Remote Control](/docs/id/remote-control#troubleshooting)

<h3 id="remote-control-couldnt-refresh-your-login">
  Remote Control tidak dapat menyegarkan login Anda
</h3>

Claude Code menjalankan koneksi [Remote Control](/docs/id/remote-control) langsung pada kredensial berumur pendek yang diperoleh dan diperbarui menggunakan login claude.ai yang disimpan. Ketika claude.ai berhenti menerima login itu, atau Claude Code tidak memiliki login yang disimpan lagi, Claude Code menghentikan Remote Control dan memerlukan Anda untuk masuk lagi. Kedua kegagalan dapat terjadi saat Claude Code masih terhubung atau nanti, ketika memperbaharui kredensial.

Ketika Claude Code meminta layanan login untuk menyegarkan login yang disimpan dan tidak mendapat jawaban, itu membuat Remote Control tetap berjalan dan mencoba penyegaran lagi saat kredensial koneksi masih berlaku. Penyegaran tidak mendapat jawaban ketika Claude Code tidak dapat menjangkau layanan login, permintaan habis waktu, atau layanan gagal tanpa menolak login Anda. Jika layanan login masih tidak menjawab ketika kredensial itu kedaluwarsa, Claude Code menghentikan Remote Control dan melaporkan `OAuth token refresh failed`.

Ketika Claude Code menghentikan Remote Control, itu menunjukkan alasannya dalam peringatan dan dalam baris transkrip yang dimulai dengan `Remote Control disconnected`. Sesi lokal Anda terus berjalan tanpa Remote Control. Bagian ini mencakup baris-baris ini:

```text theme={null}
Remote Control disconnected — Claude.ai login expired — run /login to restore Remote Control
Remote Control disconnected — Claude.ai login expired — run /login, then /remote-control
Remote Control disconnected — Claude.ai login was rejected — run /login, then /remote-control
Remote Control disconnected — OAuth token unavailable — run /login to restore Remote Control
Remote Control disconnected — OAuth token refresh failed — run /login to re-authenticate
Remote Control disconnected — JWT refresh failed: no OAuth token — run /login
Remote Control disconnected — Signed out of Claude — run /login, then /remote-control
```

Claude Code menamai penyebabnya di tengah pesan:

* ` Claude.ai login expired` dan `Claude.ai login was rejected`: claude.ai tidak lagi menerima token login yang disimpan Anda, karena kedaluwarsa atau dicabut
* ` OAuth token unavailable`: Claude Code tidak memiliki token login yang disimpan ketika kredensial koneksi jatuh tempo untuk pembaruan
* `OAuth token refresh failed`: claude.ai menolak token login yang disimpan Anda saat Claude Code terhubung kembali, dan menyegarkan token tidak menghasilkan yang baru
* `JWT refresh failed: no OAuth token`: Claude Code tidak menemukan token login yang disimpan untuk diperbarui
* ` Signed out of Claude`: Anda keluar di mesin ini, misalnya dengan menjalankan `/logout` di terminal lain, jadi Claude Code tidak memiliki login yang disimpan untuk memperbarui koneksi

**Yang harus dilakukan:**

* Jalankan `/login` untuk masuk lagi
* Jalankan `/remote-control` untuk menghubungkan kembali sesi. Pesan yang berakhir `run /login to restore Remote Control` tidak memerlukan langkah ini: Claude Code terhubung kembali dengan sendirinya setelah Anda masuk.

Sebelum v2.1.224, `OAuth token refresh failed — run /login to re-authenticate` membaca `OAuth token refresh failed — re-authenticate, then re-enable Remote Control`, dan `JWT refresh failed: no OAuth token — run /login` membaca `no OAuth token available for recovery (code <N>)`. Pesan `Claude.ai login expired`, `Claude.ai login was rejected`, dan `OAuth token unavailable` ditambahkan dalam v2.1.225.

Sebelum v2.1.238, Claude Code melaporkan kasus yang sekarang mengatakan `Signed out of Claude` sebagai `JWT refresh failed: no OAuth token — run /login`, dan menghentikan Remote Control dengan `Claude.ai login expired — run /login to restore Remote Control` segera setelah satu penyegaran login tidak mendapat jawaban.

<h3 id="remote-control-stopped-because-the-signed-in-account-changed">
  Remote Control berhenti karena akun yang masuk berubah
</h3>

Claude Code menampilkan baris ini selama sesi [Remote Control](/docs/id/remote-control) ketika Anda masuk ke akun claude.ai atau organisasi berbeda di mesin ini. Anda membuat sakelar di luar sesi Claude Code, misalnya dengan menjalankan `/login` di terminal lain.

Sesi Remote Control yang Anda mulai saat masuk melalui `/login` milik akun claude.ai dan organisasi yang masuk pada saat itu.

```text theme={null}
Remote Control disconnected — signed-in claude.ai account or organization changed on this machine — run /remote-control to start a session for the current account, or /login to switch back, then /remote-control
```

Claude Code menghentikan sesi Remote Control segera setelah claude.ai mengonfirmasi bahwa akun atau organisasi berubah. Sesi lokal Anda terus berjalan tanpa Remote Control.

**Yang harus dilakukan:**

* Jalankan `/remote-control` untuk memulai sesi Remote Control baru di bawah akun atau organisasi saat ini
* Untuk beralih kembali, jalankan `/login` dan masuk ke akun atau organisasi sebelumnya lagi. Kemudian jalankan `/remote-control`.

Sebelum v2.1.234, Claude Code tidak memperhatikan ketika Anda beralih ke akun atau organisasi berbeda di luar sesi Claude Code. Claude Code membuat sesi Remote Control tetap terhubung sampai permintaan nanti ke server Remote Control gagal dengan `Remote Control server rejected the request (HTTP 404)`. Kegagalan itu bisa datang berjam-jam setelah sakelar.

<h3 id="remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts">
  Remote Control berhenti karena aplikasi yang menjalankan sesi keluar atau beralih akun
</h3>

Ketika aplikasi desktop Claude atau IDE menghosting sesi Anda, Claude Code mendapatkan token login dari aplikasi itu daripada dari `/login`. Ketika claude.ai menolak token itu, Claude Code meminta aplikasi untuk yang baru. Jika aplikasi menjawab bahwa itu keluar, atau bahwa itu sekarang masuk ke akun Claude berbeda, Claude Code mengakhiri sesi [Remote Control](/docs/id/remote-control) dan mengirim aplikasi salah satu baris ini:

```text theme={null}
Remote Control stopped — the app running this session is now signed in to a different Claude account
Remote Control stopped — the app running this session is signed out of Claude. Sign in there, then turn Remote Control back on
```

Sesi lokal Anda terus berjalan tanpa Remote Control.

**Yang harus dilakukan:**

* Jika aplikasi keluar, masuk lagi, kemudian aktifkan Remote Control kembali di aplikasi
* Jika aplikasi beralih akun, Claude Code tidak dapat melanjutkan sesi yang berakhir di bawah akun baru. Mulai sesi Remote Control baru di bawah akun itu.

Sebelum v2.1.238, Claude Code mengirim aplikasi pesan `run /login` yang tercantum di bawah [Remote Control tidak dapat menyegarkan login Anda](#remote-control-couldnt-refresh-your-login) dalam kedua kasus.

<h3 id="oauth-token-revoked-or-expired">
  Token OAuth dicabut atau kedaluwarsa
</h3>

Login yang disimpan tidak lagi valid. Token yang dicabut berarti Anda keluar di mana-mana atau admin menghapus akses; token yang kedaluwarsa berarti penyegaran otomatis gagal di tengah sesi.

Kedua pesan melaporkan penolakan yang dikembalikan API untuk permintaan yang dikirim Claude Code. Ketika login yang disimpan sudah dihapus setelah penyegaran yang gagal, Anda melihat [Login kedaluwarsa](#login-expired) sebagai gantinya. Jika Anda autentikasi dengan token berumur panjang dalam [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/id/env-vars), Anda melihat pesan yang sama ketika token itu kedaluwarsa atau dicabut.

```text theme={null}
OAuth token revoked · Please run /login
Please run /login · API Error: 401 OAuth token has expired ...
```

**Yang harus dilakukan:**

* Jalankan `/login` untuk masuk lagi
* Jika kesalahan kembali dalam sesi yang sama setelah autentikasi ulang, jalankan `/logout` terlebih dahulu untuk sepenuhnya menghapus token yang disimpan, kemudian `/login`
* Jika Anda autentikasi dengan variabel lingkungan `CLAUDE_CODE_OAUTH_TOKEN`, Claude Code terus mengirim nilai yang Anda atur setelah permintaan gagal dengan 401, daripada beralih ke token login yang disimpan. [`/status`](/docs/id/commands) menunjukkan kredensial ini sebagai baris `Auth token` membaca `CLAUDE_CODE_OAUTH_TOKEN`. Hasilkan token segar dengan [`claude setup-token`](/docs/id/authentication#generate-a-long-lived-token) dan mulai ulang dengannya, atau batalkan pengaturan variabel dan jalankan `/login`. Sebelum v2.1.225, Claude Code dapat mengganti nilai variabel di tengah sesi dengan token akses berumur pendek dari login yang disimpan, dan sesi gagal dengan kesalahan 401 lagi setelah token itu kedaluwarsa.
* Untuk prompt berulang untuk masuk di seluruh peluncuran, lihat pemeriksaan jam sistem dan langkah pemulihan penyimpanan kredensial macOS dalam [Troubleshooting](/docs/id/troubleshoot-install#not-logged-in-or-token-expired)
* Untuk kegagalan lainnya termasuk `403 Forbidden` dan masalah browser OAuth, lihat [Login dan autentikasi](/docs/id/troubleshoot-install#login-and-authentication)

<h3 id="api-error-401-invalid-authentication-credentials">
  API Error: 401 Kredensial autentikasi tidak valid
</h3>

API mengenali format kredensial Anda tetapi menolak akun atau organisasi di baliknya. Anthropic mengembalikan pesan ini ketika kredensial baru saja dicabut, ketika organisasi dinonaktifkan atau menghapus akses Anda, atau ketika akun itu sendiri dinonaktifkan, jadi token yang kedaluwarsa bukan penyebabnya. Kredensial dapat berupa login yang disimpan atau `ANTHROPIC_API_KEY` yang disetujui, dan perbaikannya berbeda, jadi mulai dengan menjalankan `/status` untuk melihat mana yang aktif.

```text theme={null}
Please run /login · API Error: 401 Invalid authentication credentials
```

**Yang harus dilakukan:**

* Jika `/status` menunjukkan baris `API key` yang tidak ditandai sebagai tidak digunakan, `ANTHROPIC_API_KEY` yang disetujui [`ANTHROPIC_API_KEY`](/docs/id/authentication#authentication-precedence) adalah kredensial aktif dan mengambil prioritas atas login Anda, jadi `/login` tidak menggantinya. Putar kunci di Claude Console, atau kembali ke langganan Anda dengan menjalankan `unset ANTHROPIC_API_KEY`, atau di PowerShell `Remove-Item Env:ANTHROPIC_API_KEY`.
* Jika `/status` hanya menunjukkan login Anda, jalankan `/login` sekali. Jika kredensial dicabut, login segar menggantinya.
* Jika pesan yang sama kembali untuk akun login yang sama, akun atau organisasi tidak lagi aktif. Periksa akun dan organisasi yang dilaporkan `/status`, dan minta admin organisasi Anda untuk memulihkan akses.
* Jika [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) menunjuk ke [gateway LLM](/docs/id/llm-gateway), teks setelah `401` adalah pesan gateway Anda daripada Anthropic, dan `/login` tidak mengubahnya. Perbaiki kredensial yang diharapkan gateway Anda sebagai gantinya.

<h3 id="login-expired">
  Login kedaluwarsa
</h3>

Claude Code mencoba memperbarui login claude.ai atau Claude Console yang disimpan dan layanan OAuth menolak token penyegaran yang disimpan, jadi Claude Code menghapus kredensial yang disimpan. Setelah itu, setiap permintaan model berhenti secara lokal dengan pesan ini sebelum mencapai API, karena hanya `/login` yang dapat membuat kredensial baru.

Sebelum v2.1.206, Claude Code mengirim permintaan model bagaimanapun dengan kredensial apa pun yang tersisa di lingkungan, dan setiap model kemudian gagal dengan [Ada masalah dengan model yang dipilih](#theres-an-issue-with-the-selected-model) atau 401 sebagai gantinya dari prompt untuk masuk.

```text theme={null}
Login expired · Please run /login
```

Dalam [mode non-interaktif](/docs/id/headless) (`-p`) dan [Agent SDK](/docs/id/agent-sdk/overview), pesan berbunyi sebagai berikut, dan kode kesalahan terstruktur adalah `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Ini bukan keadaan yang sama seperti [Token OAuth dicabut atau kedaluwarsa](#oauth-token-revoked-or-expired). Pesan-pesan itu melaporkan penolakan yang dikembalikan API. Claude Code sendiri menghasilkan `Login expired` untuk login yang sudah gagal diperbarui, jadi tidak mengirim permintaan. Ketika pembaruan gagal karena akun itu sendiri ditangguhkan daripada login yang sudah usang, Claude Code menunjukkan [Akun Anda ditahan](#your-account-is-on-hold) sebagai gantinya.

Sesi yang diautentikasi dengan kunci API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/id/env-vars), atau penyedia pihak ketiga tidak menggunakan login yang disimpan dan tidak pernah melihat pesan ini.

Anda dapat memeriksa keadaan ini sebelum permintaan gagal: [`/status`](/docs/id/commands) menunjukkan baris `Login` membaca `Expired — log in again`, ditambah organisasi dan email yang disimpannya untuk login yang kedaluwarsa. Baris muncul hanya ketika login yang disimpan adalah kredensial aktif Anda dan tidak dapat lagi disegarkan. Sesi yang diautentikasi dengan cara lain tidak menunjukkan baris, bahkan jika login yang kedaluwarsa tetap disimpan. Sebelum v2.1.210, `/status` tidak memberikan indikasi dalam keadaan ini bahwa login pernah ada, karena kredensial yang dihapus meninggalkannya tidak ada untuk dilaporkan.

**Yang harus dilakukan:**

* Jalankan `/login` untuk masuk lagi. Mencoba lagi tanpa masuk menunjukkan pesan yang sama pada setiap permintaan.
* Dalam mode non-interaktif, jalankan `claude` di lingkungan yang sama, selesaikan `/login`, kemudian jalankan kembali perintah Anda. Untuk otomasi yang tidak dapat masuk secara interaktif, autentikasi dengan `ANTHROPIC_API_KEY` atau [hasilkan token berumur panjang dengan `claude setup-token`](/docs/id/authentication#generate-a-long-lived-token).
* Jika masuk terus gagal, lihat [Login dan autentikasi](/docs/id/troubleshoot-install#login-and-authentication)

<h3 id="claude-login-not-accepted">
  Login Claude tidak diterima
</h3>

Anda mencoba memulai [sesi cloud](/docs/id/claude-code-on-the-web), dan server menolak untuk membuatnya dengan 401: itu tidak menerima login Claude yang dikirim mesin ini, biasanya karena login kedaluwarsa atau dicabut.

Bagian pertama dari baris adalah alasan server sendiri ketika memberikan satu. Jika tidak, baris berbunyi:

```text theme={null}
Claude login not accepted · Run /login, then try again
```

**Yang harus dilakukan:**

* Jalankan `/login`, selesaikan masuk, kemudian mulai sesi lagi

<h3 id="artifacts-need-a-claude-ai-login">
  Artefak memerlukan login claude.ai
</h3>

Claude Code menolak [artefak](/docs/id/artifacts) publikasi atau baca karena sesi tidak memiliki login claude.ai yang dapat digunakan untuk artefak.

Setiap bentuk pesan dimulai dengan kata-kata yang sama, diikuti dengan solusi yang tergantung pada cara sesi Anda autentikasi. Tanpa kredensial yang bersaing, itu berbunyi:

```text theme={null}
Artifacts need a claude.ai login. Run /login and select "Claude account with subscription", then retry — the "Anthropic Console account" option does not provide claude.ai credentials.
```

**Yang harus dilakukan:**

* Jalankan `/login` dan pilih **Claude account with subscription**. Opsi **Anthropic Console account** tidak menyediakan kredensial claude.ai.
* Ketika pesan menamai kredensial yang mengambil prioritas, seperti `ANTHROPIC_API_KEY`, pengaturan `apiKeyHelper`, atau kunci Console yang disimpan oleh `/login` sebelumnya, hapus dengan cara yang dikatakan pesan, kemudian jalankan `/login`
* Ketika pesan mengatakan sesi jarak jauh ini autentikasi melalui mesin yang meluncurkannya, masuk ke claude.ai di mesin itu, kemudian hubungkan kembali sesi
* Ketika pesan mengatakan kredensial disuntikkan oleh lingkungan host sesi, Anda tidak dapat mengubahnya dalam sesi itu; mulai sesi yang masuk ke claude.ai
* Lihat [Ketersediaan](/docs/id/artifacts#availability) untuk persyaratan lain yang dimiliki artefak, seperti paket, penyedia model, dan kebijakan organisasi

<h3 id="administrator-policy-requires-a-cloud-gateway-sign-in">
  Kebijakan administrator memerlukan masuk Cloud gateway
</h3>

Pengaturan [terkelola](/docs/id/managed-settings) administrator di mesin ini menetapkan [`forceLoginMethod`](/docs/id/settings-reference#forceloginmethod) ke `"gateway"` atau menetapkan [`forceLoginGatewayUrl`](/docs/id/settings-reference#forcelogingatewayurl). Kecuali Anda memilih penyedia cloud melalui variabel seperti `CLAUDE_CODE_USE_BEDROCK`, Claude Code kemudian hanya menerima masuk [gateway aplikasi Claude](/docs/id/claude-apps-gateway). Anda melihat salah satu dari dua pesan:

```text theme={null}
Not signed in to the Cloud gateway — run /login.
```

Permintaan model gagal dengan pesan ini ketika sesi tidak memiliki masuk gateway, misalnya karena Anda belum menjalankan `/login` sejak kebijakan mencapai mesin.

Jika Anda juga memiliki kredensial `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper` yang dikonfigurasi dan pengaturan terkelola menetapkan `forceLoginMethod`, Claude Code keluar saat startup sebagai gantinya dengan pesan yang dimulai:

```text theme={null}
Administrator policy requires a Cloud gateway sign-in on this machine; the
Anthropic-issued credential configured here (ANTHROPIC_API_KEY,
ANTHROPIC_AUTH_TOKEN, or apiKeyHelper) is not used.
```

**Yang harus dilakukan:**

* Jalankan `/login` dan selesaikan masuk di layar **Cloud gateway**
* Untuk pesan startup, hapus pengaturan `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, atau `apiKeyHelper` yang Anda konfigurasi, kemudian mulai `claude` dan jalankan `/login`
* Jika Anda percaya mesin tidak boleh memerlukan gateway, minta administrator yang mengelolanya untuk menghapus `forceLoginMethod` dan `forceLoginGatewayUrl` dari pengaturan terkelolanya

Pada v2.1.265, regresi juga menunjukkan pesan pertama dalam beberapa konfigurasi gateway LLM dan proxy yang autentikasi dengan kunci API, `apiKeyHelper`, atau header khusus, bahkan tanpa persyaratan administrator di mesin. Perbarui ke v2.1.266 atau lebih baru. Anda tidak perlu mengubah konfigurasi Anda.

Sebelum v2.1.261, pada mesin yang menetapkan `forceLoginMethod` ke `"gateway"`, Claude Code menggunakan login yang disimpan sisa daripada gagal permintaan model, dan melaporkan kredensial lingkungan yang dikonfigurasi dengan `This machine's managed settings require a first-party login` daripada pesan startup. Sebelum v2.1.265, mesin yang pengaturan terkelolanya hanya menetapkan `forceLoginGatewayUrl` tidak memerlukan masuk gateway, dan Claude Code menggunakan kredensial sisa di sana.

<h3 id="your-account-is-on-hold">
  Akun Anda ditahan
</h3>

Akun Claude di balik login Anda telah ditangguhkan. Claude Code menunjukkan pesan pertama ketika mencoba memperbarui login yang disimpan dan mempelajari tentang penahan, dan yang kedua ketika masuk yang Anda selesaikan di browser melaporkannya:

```text theme={null}
Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted
Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted
```

Masuk lagi dengan akun yang sama tidak menghapus pesan, karena penahan ada di akun daripada login. Dalam [mode non-interaktif](/docs/id/headless) (`-p`) dan [Agent SDK](/docs/id/agent-sdk/overview), kode kesalahan terstruktur adalah `account_on_hold`. Sebelum v2.1.235, Claude Code melaporkan akun yang ditahan sebagai [Login kedaluwarsa · Silakan jalankan /login](#login-expired), yang langkah pemulihan tidak dapat menghapus penahan.

**Yang harus dilakukan:**

* Buka tautan dalam pesan untuk melihat detail penahan atau mengajukan banding
* Jika Anda memiliki akun Claude lain atau kunci API yang tidak terpengaruh oleh penahan, Anda dapat terus bekerja saat penahan diselesaikan: jalankan `/login` dengan akun itu, atau atur kunci dengan `ANTHROPIC_API_KEY`

<h3 id="anthropic-profile-login-expired">
  Login profil Anthropic kedaluwarsa
</h3>

Claude Code mengautentikasi melalui profil kredensial Anthropic yang kredensial login yang disimpan telah kedaluwarsa, dan profil tidak menyimpan kredensial penyegaran yang dapat digunakan Claude Code untuk memperbarui. Claude Code menghentikan setiap permintaan secara lokal tanpa mencoba lagi, karena percobaan lagi akan membaca kredensial yang sama yang kedaluwarsa.

```text theme={null}
Anthropic profile login expired · Re-authenticate your Anthropic profile
Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile
```

Ini muncul hanya ketika kredensial aktif berasal dari profil kredensial Anthropic, yang Anda pilih dengan variabel lingkungan `ANTHROPIC_PROFILE`, yang Claude Code temukan sebagai profil aktif di direktori konfigurasi Anthropic Anda, atau yang Claude Code tulis ketika Anda [masuk tanpa kunci API](/docs/id/authentication#sign-in-without-an-api-key). Sesi yang autentikasi dengan opsi claude.ai `/login`, kunci API, token pembawa seperti `ANTHROPIC_AUTH_TOKEN`, atau penyedia pihak ketiga tidak pernah melihat pesan ini.

Pada mesin yang [menawarkan masuk tanpa kunci](/docs/id/authentication#sign-in-without-an-api-key), jalankan `/login`, pilih akun Anthropic Console, dan masuk lagi untuk memperbarui profil yang ditulis oleh masuk Console tanpa kunci atau CLI Platform Claude `ant auth login`. Claude Code mengganti kredensial yang kedaluwarsa dalam profil itu. Untuk profil federasi atau yang dibuat alat lain, `/login` tidak memperbarui kredensial. Bentuk mana yang Anda lihat tergantung pada apakah Anda memilih profil atau Claude Code menemukannya:

* Ketika Anda menetapkan `ANTHROPIC_PROFILE` secara eksplisit, pesan berakhir dengan `Re-authenticate your Anthropic profile`.
* Ketika Claude Code menemukan profil dari direktori konfigurasi Anda, pesan menawarkan `/login`, karena Claude Code memberikan prioritas `/login` yang berfungsi atas profil yang ditemukan dan kemudian autentikasi dengan akun claude.ai atau Console Anda sebagai gantinya. Sebelum v2.1.234, Claude Code menunjukkan bentuk `Re-authenticate your Anthropic profile` dalam kasus ini juga.

**Yang harus dilakukan:**

* Masuk ke profil lagi, kemudian coba lagi: pada mesin yang [menawarkan masuk tanpa kunci](/docs/id/authentication#sign-in-without-an-api-key), jalankan `/login` dan pilih akun Anthropic Console untuk profil yang ditulis oleh masuk Console tanpa kunci atau CLI Platform Claude `ant auth login`; untuk profil lain, gunakan alat yang membuatnya
* Jika administrator menyediakan kredensial profil, minta mereka untuk mengeluarkan yang baru
* Jalankan `/status` untuk mengonfirmasi sumber kredensial aktif dan nama profil
* Untuk berhenti menggunakan profil, batalkan pengaturan `ANTHROPIC_PROFILE` jika Anda menetapkannya, kemudian autentikasi dengan cara lain, seperti `/login` atau `ANTHROPIC_API_KEY`

<h3 id="oauth-scope-requirement">
  Persyaratan cakupan OAuth
</h3>

Token yang disimpan mendahului cakupan izin yang dibutuhkan fitur yang lebih baru. Anda melihat ini paling sering dari `/usage` dan indikator penggunaan baris status:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**Yang harus dilakukan:**

* Jalankan `/login` untuk mendapatkan token baru dengan cakupan saat ini. Anda tidak perlu keluar terlebih dahulu.

<h3 id="claude-ai-rejected-the-session-token">
  claude.ai menolak token sesi
</h3>

Permintaan [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) gagal karena claude.ai menolak token dari login Claude Code Anda, biasanya login yang kedaluwarsa dan tidak dapat disegarkan. Token yang ditolak adalah login Anda, bukan otorisasi konektor sendiri di claude.ai, jadi mengotorisasi konektor lagi tidak menyelesaikannya. Dalam `/mcp`, konektor menunjukkan sebagai `connected · session token rejected` dan tampilan detailnya berbunyi:

```text theme={null}
claude.ai rejected the session token. Run /login, then reconnect.
```

**Yang harus dilakukan:**

* Jalankan `/login` untuk masuk lagi
* Hubungkan kembali konektor dari `/mcp`, atau jalankan `/mcp reconnect <server>`. Menghubungkan kembali sebelum Anda masuk lagi meninggalkan konektor dalam keadaan yang sama. Opsi **Reconnect** panel `/mcp` melaporkan `your claude.ai session token was rejected`; bentuk `/mcp reconnect <server>` yang diketik melaporkan reconnect yang berhasil meskipun token masih ditolak.

Sebelum v2.1.222, Claude Code menandai konektor sebagai memerlukan autentikasi sebagai gantinya, yang menunjukkan Anda pada alur otorisasi konektor meskipun menyelesaikannya tidak menyelesaikan keadaan.

<h3 id="mcp-server-needs-you-to-sign-in-again">
  Server MCP memerlukan Anda untuk masuk lagi
</h3>

Server [MCP](/docs/id/mcp) jarak jauh menolak kredensial pada panggilan alat di tengah sesi, biasanya karena masuk atau token kedaluwarsa atau karena token kekurangan izin yang dibutuhkan alat. Panggilan alat gagal, dan `/mcp` menandai server sebagai [memerlukan autentikasi](/docs/id/mcp#authenticate-with-remote-mcp-servers).

Untuk server yang Anda masuki dari Claude Code, termasuk konektor claude.ai, masuk kedaluwarsa atau dicabut:

```text theme={null}
MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)
```

Jalankan `/mcp`, pilih server, dan masuk lagi dari menunya.

Untuk server yang dikonfigurasi dengan skrip [`headersHelper`](/docs/id/mcp#use-dynamic-headers-for-custom-authentication), Claude Code sudah menjalankan kembali pembantu dan mencoba lagi panggilan sekali sebelum menunjukkan ini:

```text theme={null}
MCP server "<name>" rejected the credential from its headersHelper (check the helper and run /mcp to reconnect, or to authenticate if the server also uses OAuth)
```

Periksa bahwa pembantu mengembalikan kredensial yang diterima server, kemudian hubungkan kembali dari `/mcp`, yang menjalankan pembantu lagi.

Untuk server dengan header `Authorization` statis dalam konfigurasinya:

```text theme={null}
MCP server "<name>" rejected the Authorization header in its config (update it, then run /mcp to reconnect)
```

Perbarui nilai header tempat server dikonfigurasi, kemudian hubungkan kembali dari `/mcp`.

Sebelum v2.1.273, kasus masuk yang kedaluwarsa, `headersHelper`, dan header `Authorization` semuanya menunjukkan `MCP server "<name>" requires re-authorization (token expired)`.

Server juga dapat menolak panggilan alat dengan HTTP 403 `insufficient_scope` untuk meminta Anda mengotorisasi cakupan, kadang-kadang yang sudah tercantum dalam token Anda. Pesan menamai cakupan itu:

```text theme={null}
MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate
```

Jalankan `/mcp`, pilih server, dan autentikasi lagi dari menunya.

Ketika konfigurasi server tidak menetapkan [`oauth.scopes`](/docs/id/mcp#restrict-oauth-scopes) atau [`authServerMetadataUrl`](/docs/id/mcp#override-oauth-metadata-discovery), Claude Code meminta cakupan yang dinamai server. Dengan pengaturan apa pun, Claude Code meminta cakupan pengaturan itu sebagai gantinya. Jika Anda menyematkan `oauth.scopes`, tambahkan cakupan yang hilang ke daftar itu sebelum Anda autentikasi lagi.

Sebelum v2.1.274, kasus ini menunjukkan pesan `needs you to sign in again`, dan sebelum v2.1.273 itu menunjukkan `requires re-authorization (token expired)` seperti kasus lainnya.

<h3 id="issuer-mismatch-in-authorization-response">
  Ketidakcocokan penerbit dalam respons otorisasi
</h3>

Selama [masuk OAuth MCP](/docs/id/mcp#authenticate-with-remote-mcp-servers), server otorisasi dialihkan kembali ke Claude Code dengan parameter `iss` yang tidak menamai penerbit yang diharapkan Claude Code dari metadata OAuth server. Penerbit yang salah pada langkah ini adalah bagaimana serangan pencampuran server otorisasi terlihat, jadi Claude Code gagal masuk daripada menukar kode otorisasi. Claude Code menunjukkan kesalahan dalam menu server `/mcp` setelah masuk browser:

```text theme={null}
Issuer mismatch in authorization response (RFC 9207): expected "https://auth.example.com", received "https://other.example.com"
```

`expected` adalah penerbit dari metadata OAuth server, dan `received` adalah nilai `iss` yang dibawa pengalihan. Masuk yang pengalihannya tidak membawa parameter `iss` melewati pemeriksaan, kecuali metadata server menetapkan `authorization_response_iss_parameter_supported`, dalam hal ini Claude Code gagal masuk.

**Yang harus dilakukan:**

* Coba masuk lagi dari `/mcp`
* Jika kesalahan berulang, laporkan ke operator server. Perbaikannya adalah sisi server: server otorisasi harus mengembalikan penerbit yang sama dalam parameter `iss` yang diiklankan dalam metadata-nya
* Untuk terhubung saat server sedang diperbaiki, mulai Claude Code dengan [`MCP_SDK_GENERATION=v1`](/docs/id/env-vars), yang [runtime](/docs/id/mcp#mcp-client-runtimes)-nya tidak menjalankan pemeriksaan ini. Ini menghapus perlindungan terhadap serangan pencampuran, jadi lebih suka perbaikan sisi server

Sebelum v2.1.232, Claude Code menggunakan runtime v2 hanya dalam peluncuran bertahap atau ketika Anda menetapkan `MCP_SDK_GENERATION=v2`.

<h3 id="aws-credentials-expired-or-invalid">
  Kredensial AWS kedaluwarsa atau tidak valid
</h3>

Token sesi AWS Anda kedaluwarsa atau ditolak. Pesan ini muncul pada 401 dari [Claude Platform di AWS](/docs/id/claude-platform-on-aws) atau [titik akhir Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint), yang merupakan cara penyedia itu melaporkan token keamanan yang kedaluwarsa.

Petunjuk tindakan di tengah bervariasi dengan pengaturan Anda. Bagian yang stabil adalah `AWS credentials expired or invalid` terkemuka:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Sebelum v2.1.273, pesan ini muncul hanya ketika `awsAuthRefresh` dikonfigurasi.

**Yang harus dilakukan:**

* Jika petunjuk mengatakan kredensial dikelola oleh lingkungan ini, aplikasi yang meluncurkan Claude Code memiliki kredensial dan langkah lain di sini tidak berlaku: coba lagi, atau hubungi administrator Anda
* Jika [`awsAuthRefresh`](/docs/id/amazon-bedrock#advanced-credential-configuration) diatur, jalankan perintah yang dinamai dalam pesan, seperti `aws sso login --profile myprofile`, di terminal lain dan selesaikan masuk browser, kemudian coba lagi. Jika tidak, segarkan kredensial AWS yang Anda gunakan sendiri: masuk SSO, kunci akses, kunci API, atau token proxy Anda
* Dengan `awsAuthRefresh` diatur dalam sesi interaktif, Anda dapat menjalankan `/login`, pilih **3rd-party platform**, kemudian pilih **Claude Platform on AWS · refresh credentials** di bawah **Using 3rd-party platforms** untuk menjalankan perintah yang sama tanpa memulai ulang Claude Code. Lihat [Konfigurasi kredensial AWS](/docs/id/claude-platform-on-aws#1-configure-aws-credentials)
* Jika kesalahan berulang setelah perintah penyegaran berhasil, konfirmasi identitas valid di luar Claude Code dengan `aws sts get-caller-identity` di shell dan profil yang sama

<h3 id="aws-authentication-failed">
  Autentikasi AWS gagal
</h3>

Penyedia AWS Anda mengembalikan 403, atau [Amazon Bedrock](/docs/id/amazon-bedrock) mengembalikan 401.

Amazon Bedrock melaporkan token keamanan yang kedaluwarsa sebagai 403, tetapi 403 juga merupakan cara itu melaporkan penolakan otorisasi, seperti `AccessDeniedException` dari izin IAM yang hilang. Claude Code tidak dapat membedakan kedua penyebab itu.

401 dari Amazon Bedrock juga mendarat di sini daripada di bawah [Kredensial AWS kedaluwarsa atau tidak valid](#aws-credentials-expired-or-invalid), karena Amazon Bedrock tidak melaporkan token yang kedaluwarsa sebagai 401. 401 dari titik akhir itu biasanya berasal dari sesuatu yang lain dalam jalur permintaan, seperti proxy perusahaan.

Penyegaran kredensial memperbaiki token yang kedaluwarsa dan tidak dapat memperbaiki penyebab lain, jadi pesan menawarkan keduanya:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

Petunjuk tindakan di tengah bervariasi dengan pengaturan Anda. Bagian yang stabil adalah `AWS authentication failed` terkemuka.

Ketika 403 adalah jawaban Amazon Bedrock bahwa Anda tidak memiliki akses ke model dengan ID model yang ditentukan, petunjuk sebagai gantinya memberi tahu Anda untuk mengaktifkan model untuk akun dan wilayah Anda di konsol Amazon Bedrock.

Sebelum v2.1.273, pesan ini muncul hanya ketika `awsAuthRefresh` dikonfigurasi.

**Yang harus dilakukan:**

* Jika petunjuk mengatakan kredensial dikelola oleh lingkungan ini, aplikasi yang meluncurkan Claude Code memiliki kredensial dan langkah lain di sini tidak berlaku: coba lagi, atau hubungi administrator Anda
* Segarkan kredensial AWS Anda jika kredensial yang kedaluwarsa adalah penyebabnya: jalankan perintah [`awsAuthRefresh`](/docs/id/amazon-bedrock#advanced-credential-configuration) yang dinamai dalam pesan ketika satu diatur, atau segarkan masuk SSO, kunci akses, kunci API, atau token proxy Anda sendiri
* Jika kredensial Anda saat ini, konfirmasi izin IAM dalam [Konfigurasi IAM](/docs/id/amazon-bedrock#iam-configuration) dilampirkan ke identitas yang Anda gunakan dan model yang dipilih diaktifkan untuk akun dan wilayah Anda
* Jalankan `aws sts get-caller-identity` untuk mengonfirmasi identitas mana yang digunakan permintaan Anda; profil `AWS_PROFILE` yang sudah usang atau profil default adalah penyebab umum ketidakcocokan izin

<h3 id="google-cloud-credentials-expired-or-invalid">
  Kredensial Google Cloud kedaluwarsa atau tidak valid
</h3>

Kredensial Google Cloud Anda untuk [Platform Agen Google Cloud](/docs/id/google-vertex-ai) kedaluwarsa atau ditolak: permintaan mengembalikan 401, yang merupakan cara Platform Agen melaporkan kedaluwarsa kredensial.

Petunjuk tindakan di tengah bervariasi dengan pengaturan Anda. Bagian yang stabil adalah `Google Cloud credentials expired or invalid` terkemuka:

```text theme={null}
Google Cloud credentials expired or invalid · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · API Error: 401 ...
```

**Yang harus dilakukan:**

* Jika petunjuk mengatakan kredensial dikelola oleh lingkungan ini, aplikasi yang meluncurkan Claude Code memiliki kredensial dan langkah lain di sini tidak berlaku: coba lagi, atau hubungi administrator Anda
* Jika Anda autentikasi dengan kredensial default aplikasi, jalankan perintah [`gcpAuthRefresh`](/docs/id/google-vertex-ai#advanced-credential-configuration) yang dinamai dalam pesan, atau `gcloud auth application-default login`, dan selesaikan masuk, kemudian coba lagi
* Jika Anda merutekan melalui [gateway LLM](/docs/id/llm-gateway) dengan `CLAUDE_CODE_SKIP_VERTEX_AUTH` diatur, segarkan token gateway dalam `ANTHROPIC_AUTH_TOKEN` atau `ANTHROPIC_CUSTOM_HEADERS`, kemudian coba lagi
* Jika Anda autentikasi dengan file kunci akun layanan, konfirmasi `GOOGLE_APPLICATION_CREDENTIALS` menunjuk ke kunci yang valid. Lihat [Konfigurasi kredensial GCP](/docs/id/google-vertex-ai#3-configure-gcp-credentials)
* Jika kesalahan berulang setelah penyegaran, konfirmasi identitas bekerja di luar Claude Code dengan `gcloud auth application-default print-access-token` di shell yang sama

Sebelum v2.1.273, 401 dari Platform Agen menunjukkan pesan `Please run /login` atau `Failed to authenticate` generik sebagai gantinya, yang tidak dapat menyegarkan kredensial Google Cloud.

<h3 id="google-cloud-authentication-failed">
  Autentikasi Google Cloud gagal
</h3>

[Platform Agen Google Cloud](/docs/id/google-vertex-ai) mengembalikan 403, yang digunakan untuk penolakan otorisasi daripada kredensial yang kedaluwarsa. Biasanya identitas yang Anda autentikasi dengan hilang izin IAM, atau model tidak diaktifkan untuk proyek Anda.

Petunjuk tindakan di tengah bervariasi dengan pengaturan Anda. Bagian yang stabil adalah `Google Cloud authentication failed` terkemuka:

```text theme={null}
Google Cloud authentication failed · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · if credentials are current, check GCP IAM permissions and Vertex AI model access · API Error: 403 ...
```

**Yang harus dilakukan:**

* Jika petunjuk mengatakan kredensial dikelola oleh lingkungan ini, aplikasi yang meluncurkan Claude Code memiliki kredensial dan langkah lain di sini tidak berlaku: coba lagi, atau hubungi administrator Anda
* Konfirmkan peran dalam [Konfigurasi IAM](/docs/id/google-vertex-ai#iam-configuration) diberikan kepada identitas yang Anda autentikasi dengan
* Konfirmkan model diaktifkan untuk proyek Anda. Lihat [Minta akses model](/docs/id/google-vertex-ai#2-request-model-access)

Sebelum v2.1.273, 403 dari Platform Agen menunjukkan pesan `Please run /login` atau `Failed to authenticate` generik sebagai gantinya, yang tidak dapat menyegarkan kredensial Google Cloud.

<h3 id="microsoft-foundry-authentication-failed">
  Autentikasi Microsoft Foundry gagal
</h3>

[Microsoft Foundry](/docs/id/microsoft-foundry) mengembalikan 401 atau 403: kredensial Azure pada permintaan ditolak, atau identitas di baliknya tidak memiliki akses ke sumber daya Foundry. `/login` tidak dapat membuat kredensial Azure. Petunjuk tindakan di tengah bervariasi dengan pengaturan Anda. Bagian yang stabil adalah `Microsoft Foundry authentication failed` terkemuka:

```text theme={null}
Microsoft Foundry authentication failed · refresh your Foundry credential (ANTHROPIC_FOUNDRY_AUTH_TOKEN, ANTHROPIC_FOUNDRY_API_KEY, Azure sign-in for Entra, or your proxy token) and retry · if credentials are current, check access to the Foundry resource · API Error: 401 ...
```

**Yang harus dilakukan:**

* Jika petunjuk mengatakan kredensial dikelola oleh lingkungan ini, aplikasi yang meluncurkan Claude Code memiliki kredensial dan langkah lain di sini tidak berlaku: coba lagi, atau hubungi administrator Anda
* Segarkan kredensial yang Anda konfigurasi dalam [Konfigurasi kredensial Azure](/docs/id/microsoft-foundry#2-configure-azure-credentials): putar `ANTHROPIC_FOUNDRY_API_KEY`, buat `ANTHROPIC_FOUNDRY_AUTH_TOKEN` segar, atau jalankan `az login` sehingga rantai kredensial Microsoft Entra default dapat masuk lagi
* Jika kredensial saat ini, konfirmkan identitas memiliki akses ke sumber daya Foundry. Lihat [Konfigurasi RBAC Azure](/docs/id/microsoft-foundry#azure-rbac-configuration)

Sebelum v2.1.273, 401 atau 403 dari Microsoft Foundry menunjukkan pesan `Please run /login` atau `Failed to authenticate` generik sebagai gantinya, yang tidak dapat menyegarkan kredensial Azure.

<h3 id="could-not-load-aws-or-google-cloud-credentials">
  Tidak dapat memuat kredensial AWS atau Google Cloud
</h3>

Claude Code tidak dapat memperoleh kredensial yang dapat digunakan dari rantai penyedia kredensial AWS atau dari kredensial default aplikasi Google Anda di mesin tempat itu berjalan, jadi tidak ada permintaan yang mencapai penyedia cloud Anda. Claude Code menghapus kredensial yang disimpan dalam cache dan mencoba lagi dua kali sebelum menunjukkan pesan ini. Detail setelah `·` menamai penyebab spesifik, seperti sesi SSO yang kedaluwarsa, kredensial default aplikasi yang hilang dilaporkan sebagai `Could not load the default credentials`, atau masuk yang dicabut dilaporkan sebagai `invalid_grant`:

```text theme={null}
API Error: Could not load AWS credentials · Could not load credentials from any providers. Check or refresh your AWS credentials and try again.
API Error: Could not load Google Cloud credentials · invalid_grant. Check or refresh your Google Cloud credentials and try again.
```

Dalam [mode non-interaktif](/docs/id/headless) dengan `-p` dan dalam [Agent SDK](/docs/id/agent-sdk/overview), kode kesalahan terstruktur adalah `cloud_credential_error`. Sebelum v2.1.267, pesan hanya menunjukkan teks detail setelah `API Error:`, dan kode terstruktur adalah `server_error` atau `unknown`.

**Yang harus dilakukan:**

* Jalankan perintah masuk penyedia Anda, seperti `aws sso login --profile myprofile` atau `gcloud auth application-default login`, kemudian coba lagi. [Kredensial Bedrock, Platform Agen, atau Foundry tidak memuat](/docs/id/troubleshoot-install#bedrock-agent-platform-or-foundry-credentials-not-loading) menunjukkan cara mengonfirmasi kredensial di luar Claude Code
* Jika detail membaca `AWS default-chain credential resolve timed out`, rantai tergantung daripada gagal, jadi ikuti [Resolusi kredensial rantai default AWS habis waktu](#aws-default-chain-credential-resolve-timed-out) sebagai gantinya

<h3 id="aws-default-chain-credential-resolve-timed-out">
  Resolusi kredensial rantai default AWS habis waktu
</h3>

Rantai penyedia kredensial default AWS tidak menghasilkan kredensial dalam 60 detik, jadi Claude Code menghentikan resolusi dan gagal permintaan. Waktu habis ini adalah satu penyebab [Tidak dapat memuat kredensial AWS atau Google Cloud](#could-not-load-aws-or-google-cloud-credentials). Kegagalan adalah resolusi kredensial lokal: permintaan tidak pernah mencapai [Amazon Bedrock](/docs/id/amazon-bedrock), [Claude Platform di AWS](/docs/id/claude-platform-on-aws), atau [titik akhir Mantle](/docs/id/amazon-bedrock#use-the-mantle-endpoint). Claude Code menghapus [cache kredensial](/docs/id/amazon-bedrock#credential-caching-and-resolution-timeout) dan mencoba lagi sebelum kesalahan ini muncul, jadi pada saat Anda melihatnya rantai telah macet pada upaya berulang.

```text theme={null}
API Error: Could not load AWS credentials · AWS default-chain credential resolve timed out. Check or refresh your AWS credentials and try again.
```

Penyebab umum adalah perintah `credential_process` dalam profil AWS Anda yang menunggu input yang tidak dapat diterima, dan kontainer atau VM yang layanan metadata instans (IMDS) tidak pernah menjawab probe rantai.

Sebelum v2.1.267, pesan membaca `API Error: AWS default-chain credential resolve timed out`.
Sebelum v2.1.207, rantai yang macet meninggalkan permintaan menunggu tanpa batas daripada gagal.

**Yang harus dilakukan:**

* Jalankan `aws sts get-caller-identity` di shell yang sama dengan `AWS_PROFILE` yang sama. Jika juga tergantung, perbaiki profil; perintah `credential_process` yang meminta secara interaktif adalah penyebab umum.
* Selesaikan langkah masuk sebelum memulai Claude Code, misalnya `aws sso login --profile myprofile`, sehingga rantai diselesaikan dari cache SSO lokal daripada menunggu alur browser
* Jika rantai Anda menjalankan masuk interaktif yang secara sah memerlukan lebih dari 60 detik, seperti SSO dengan MFA melalui pembungkus seperti `aws-vault`, naikkan batas dalam milidetik dengan [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/id/env-vars)

<h3 id="bedrock-setup-verification-timed-out-waiting-for-aws">
  Verifikasi pengaturan Bedrock habis waktu menunggu AWS
</h3>

Panggilan ke AWS selama [wizard pengaturan Bedrock](/docs/id/amazon-bedrock#sign-in-with-bedrock), seperti pencarian kredensial atau pemeriksaan identitas, tidak selesai dalam batas 60 detik. Wizard berhenti menunggu dan gagal langkah verifikasi:

```text theme={null}
Timed out after 60s waiting for AWS. Check your network and proxy settings; if a credential helper needs longer to prompt you, raise CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS.
```

Angka mencerminkan batas Anda: 60 detik secara default, atau nilai yang Anda atur dalam [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/id/env-vars).

Penyebab umum adalah jaringan atau proxy yang macet permintaan ke AWS, termasuk penyegaran token SSO, dan pembantu kredensial masih menunggu input yang tidak dapat Anda lihat. Naikkan batas hanya ketika pembantu secara sah memerlukan lebih banyak waktu.

Permintaan tunggal yang macet ke AWS juga dapat gagal pada waktu habis per-permintaan sendiri, yang menunjukkan pesan yang lebih pendek pada langkah yang sama:

```text theme={null}
A request to AWS timed out. Check your network and proxy settings, then try again.
```

Ketika waktu habis yang sama terjadi pada langkah pin model, wizard menandai model sebagai `unreachable` daripada menunjukkan pesan apa pun.

**Yang harus dilakukan:**

* Jalankan `aws sts get-caller-identity` di shell yang sama. Jika juga tergantung, macet ada di luar Claude Code, di jaringan Anda, proxy Anda, atau pembantu kredensial dalam profil AWS Anda; perbaiki itu terlebih dahulu.
* Selesaikan masuk interaktif apa pun sebelum membuka wizard, misalnya `aws sso login --profile myprofile`
* Jika pembantu kredensial dalam profil AWS Anda secara sah memerlukan lebih lama dari 60 detik untuk meminta Anda, naikkan batas dalam milidetik dengan [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/id/env-vars)

<h3 id="cloud-gateway-session-expired">
  Sesi gateway cloud kedaluwarsa
</h3>

Anda masuk melalui [gateway aplikasi Claude](/docs/id/claude-apps-gateway), dan sesi gateway yang disimpan di mesin ini telah kedaluwarsa dan tidak dapat diperbarui, atau gateway tidak lagi menerimanya, misalnya setelah [rahasia JWT gateway diganti](/docs/id/claude-apps-gateway-deploy#jwt-secret-rotation). Jika Anda melihat baris ini ketika memulai `claude` secara interaktif, sesi telah dibuka keluar dari gateway:

```text theme={null}
Cloud gateway session expired — run /login to reconnect.
```

Baris yang sama dapat muncul di tengah sesi ketika kredensial gateway kedaluwarsa dan Claude Code tidak dapat memperbarui.

Dalam jalankan [non-interaktif](/docs/id/headless), sesi latar belakang atau sesi yang tidak diawasi lainnya, atau subperintah `claude` selain `claude auth`, Claude Code keluar dengan pesan ini sebagai gantinya ketika gateway tidak lagi menerima sesi:

```text theme={null}
Cloud gateway <url> no longer accepts this session. Start `claude` and sign in again with /login.
```

**Yang harus dilakukan:**

* Jalankan `/login` dalam sesi dan selesaikan masuk browser
* Untuk peluncuran non-interaktif, mulai `claude` di lingkungan yang sama, jalankan `/login`, kemudian jalankan kembali perintah Anda

<h3 id="sign-in-timed-out-while-waiting-for-you-to-continue">
  Masuk tidak diterima ke Cloud gateway
</h3>

Anda mencoba memulai [sesi cloud](/docs/id/claude-code-on-the-web), dan server menolak untuk membuatnya dengan 401: itu tidak menerima login Claude yang dikirim mesin ini, biasanya karena login kedaluwarsa atau dicabut.

Bagian pertama dari baris adalah alasan server sendiri ketika memberikan satu. Jika tidak, baris berbunyi:

```text theme={null}
Claude login not accepted · Run /login, then try again
```

**Yang harus dilakukan:**

* Jalankan `/login`, selesaikan masuk, kemudian mulai sesi lagi

<h3 id="gateway-refused-the-request">
  Gateway menolak permintaan
</h3>

Anda masuk melalui [gateway aplikasi Claude](/docs/id/claude-apps-gateway), dan permintaan mengembalikan 403: gateway, atau upstream di baliknya, menolaknya. Masuk lagi tidak mengubah penolakan, jadi pesan menunjuk ke administrator gateway Anda:

```text theme={null}
Gateway refused the request · signing in again won't change this — check with your gateway administrator · API Error: 403 ...
```

**Yang harus dilakukan:**

* Minta administrator gateway Anda untuk mencari permintaan. Ekor `API Error:` membawa penolakan yang dikembalikan gateway
* Untuk administrator: aturan [kontrol akses](/docs/id/claude-apps-gateway-config#http-tuning) pada gateway mengembalikan 403 yang dicatat [log audit](/docs/id/claude-apps-gateway-deploy#logs) dengan alasannya, dan penolakan otorisasi upstream melewati per [Pesan kesalahan Upstream](/docs/id/claude-apps-gateway-config#upstream-error-messages)

Sebelum v2.1.273, 403 pada sesi gateway menunjukkan pesan `Please run /login` atau `Failed to authenticate` generik sebagai gantinya, dan masuk lagi tidak menghapus penolakan.

<h2 id="network-and-connection-errors">
  Kesalahan jaringan dan koneksi
</h2>

Sebagian besar kesalahan ini berarti permintaan jaringan dari Claude Code gagal mencapai tujuannya, atau sesuatu antara Claude Code dan API mengubah respons dalam perjalanannya kembali; jika entri juga memiliki penyebab lokal, seperti penulisan arsip yang gagal, isinya akan mengatakan demikian. Mereka biasanya berasal dari jaringan lokal Anda, proxy, atau firewall, atau dari kebijakan jaringan lingkungan cloud.

<h3 id="unable-to-connect-to-api">
  Unable to connect to API
</h3>

Koneksi TCP ke API gagal atau tidak pernah selesai. Untuk kode kesalahan koneksi umum, nama pesan menunjukkan jenis kegagalan dan menyimpan kode dalam tanda kurung:

```text theme={null}
Unable to connect to API. Check your internet connection
Connection refused — a firewall or proxy may be blocking it (ConnectionRefused)
Can't reach the API server — check your internet or DNS (ENOTFOUND)
No internet route — check your connection or VPN (EHOSTUNREACH)
Couldn't connect through your proxy (ERR_PROXY_TUNNEL) — the proxy refused the tunnel: check its credentials and that it allows this host
Connection dropped (ECONNRESET)
fetch failed
Request timed out. Check your internet connection and proxy settings
```

Kode yang Claude Code tidak kenali muncul sebagai `Unable to connect to API` diikuti oleh kode dalam tanda kurung. Beberapa pesan ini dapat menampilkan lebih dari satu kode: `Connection refused` dapat menampilkan `ConnectionRefused` atau `ECONNREFUSED`, misalnya, dan `Can't reach the API server` dapat menampilkan `ENOTFOUND` atau `FailedToOpenSocket`.

Sebelum v2.1.227, setiap pesan berkode ini berbunyi `Unable to connect to API` diikuti oleh kode, misalnya `Unable to connect to API (ECONNREFUSED)`.

Penyebab umum termasuk tidak ada akses internet, VPN yang memblokir `api.anthropic.com`, atau proxy perusahaan yang diperlukan yang tidak dikonfigurasi.

**Yang harus dilakukan:**

* Konfirmasi Anda dapat menjangkau host API dari shell yang sama dengan menjalankan `curl -I https://api.anthropic.com`. Di Windows PowerShell gunakan `curl.exe -I https://api.anthropic.com` sehingga alias `Invoke-WebRequest` bawaan tidak digunakan.
* Jika Anda berada di belakang proxy perusahaan, atur `HTTPS_PROXY` sebelum meluncurkan Claude Code dan lihat [Konfigurasi jaringan](/docs/id/network-config)
* Jika Anda merutekan melalui gateway LLM atau relay, atur [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) ke alamatnya. Lihat [Hubungkan Claude Code ke gateway LLM](/docs/id/llm-gateway-connect) untuk pengaturan.
* Pastikan firewall Anda memungkinkan host yang tercantum dalam [Persyaratan akses jaringan](/docs/id/network-config#network-access-requirements)
* Kegagalan intermiten [diulang secara otomatis](#automatic-retries); kegagalan persisten menunjukkan masalah jaringan lokal

Jika `curl` berhasil tetapi Claude Code masih gagal, penyebabnya biasanya sesuatu antara runtime dan jaringan daripada jaringan itu sendiri:

* Di Linux dan WSL, periksa `/etc/resolv.conf` untuk nameserver yang tidak dapat dijangkau. WSL khususnya dapat mewarisi resolver yang rusak dari host.
* Di macOS, klien VPN yang terputus atau dihapus dapat meninggalkan antarmuka terowongan atau aturan perutean. Periksa `ifconfig` untuk antarmuka `utun` yang basi dan hapus ekstensi jaringan VPN di Pengaturan Sistem.
* Docker Desktop dan runtime kontainer serupa dapat mencegat lalu lintas keluar. Keluar dari mereka dan coba lagi untuk mengesampingkan ini.

<h3 id="unable-to-connect-to-anthropic-services">
  Unable to connect to Anthropic services
</h3>

Selama pengaturan pertama kali, Claude Code memeriksa bahwa dapat menjangkau `api.anthropic.com` dan `platform.claude.com` sebelum menampilkan langkah masuk. Ketika salah satu pemeriksaan gagal, Claude Code mencetak alasannya dan keluar.

```text theme={null}
Unable to connect to Anthropic services
Failed to connect to api.anthropic.com: ECONNREFUSED
Connection to api.anthropic.com timed out after 10 seconds
A proxy is configured via HTTPS_PROXY. Check that it allows connections to the host above.
```

Claude Code mengirim pemeriksaan melalui [konfigurasi proxy](/docs/id/network-config) yang sama dengan permintaan API dan memberikan setiap probe 10 detik. Ketika probe yang gagal melewati proxy, pesan menamai variabel lingkungan yang mengonfigurasinya, seperti `HTTPS_PROXY`. Sebelum v2.1.222, pemeriksaan menggunakan transport proxy yang berbeda tanpa batas waktu: di belakang URL proxy dengan skema `https://`, dapat macet pada `Checking connectivity...` tanpa batas dan kemudian gagal meskipun permintaan API melalui proxy yang sama berhasil.

Claude Code melewati pemeriksaan ini ketika [file pengaturan terkelola, kebijakan MDM, atau pembantu kebijakan](/docs/id/managed-settings) menetapkan [`forceLoginMethod`](/docs/id/settings-reference#forceloginmethod) ke `"gateway"`, atau menetapkan [`forceLoginGatewayUrl`](/docs/id/settings-reference#forcelogingatewayurl) tanpa `forceLoginMethod`. Dengan salah satu konfigurasi, Claude Code membuka langkah masuk di layar **Cloud gateway** daripada metode masuk Anthropic. Claude Code juga melewati pemeriksaan ketika sumber pengaturan terkelola di mesin ada tetapi tidak dapat dibaca, karena sumber itu mungkin menyimpan konfigurasi gateway. Sebelum v2.1.247, Claude Code menjalankan pemeriksaan di bawah konfigurasi ini juga, dan keluar dengan kesalahan ini ketika titik akhir Anthropic tidak dapat dijangkau.

**Yang harus dilakukan:**

* Jika pesan menamai variabel proxy, periksa bahwa nilainya menunjuk ke proxy yang tepat dan minta tim jaringan Anda untuk memungkinkan koneksi HTTPS melaluinya ke host dalam pesan. Lihat [Konfigurasi jaringan](/docs/id/network-config).
* Kerjakan pemeriksaan dalam [Unable to connect to API](#unable-to-connect-to-api). Tes `curl` dan panduan firewall di sana berlaku untuk pemeriksaan ini juga.
* Jika organisasi Anda masuk melalui [cloud gateway](/docs/id/claude-apps-gateway) dan kesalahan ini muncul pada run pertama, perbarui ke Claude Code v2.1.247 atau lebih baru.
* Jika jaringan Anda terbuka dan kegagalan berlanjut, Claude Code mungkin tidak [tersedia di negara Anda](https://www.anthropic.com/supported-countries)

<h3 id="socket-is-closed">
  Socket is closed
</h3>

`Socket is closed` berarti koneksi yang membawa respons streaming ditutup sementara respons masih tiba. Penyebab paling umum adalah proxy perusahaan di Windows yang menjatuhkan terowongan yang sudah terbentuk di tengah respons.

Tergantung seberapa jauh respons telah berkembang, Claude Code mengulang permintaan, menyimpan apa yang Claude hasilkan, atau mengakhiri giliran. Lihat [Pengulangan otomatis](#automatic-retries).

Sebelum v2.1.214, Claude Code tidak mengulang kegagalan ini, dan giliran berhenti dengan kesalahan yang berisi `Socket is closed`.

**Yang harus dilakukan:**

* Jika Anda melihat kesalahan ini, perbarui ke v2.1.214 atau lebih baru dengan `claude update`, kemudian kirim pesan Anda lagi
* Jika giliran terus gagal di belakang proxy yang sama setelah memperbarui, kerjakan [Unable to connect to API](#unable-to-connect-to-api) dan periksa pengaturan proxy dalam [Konfigurasi jaringan](/docs/id/network-config)

<h3 id="api-returned-an-empty-or-malformed-response">
  API returned an empty or malformed response
</h3>

Claude Code menampilkan kesalahan ini ketika pengulangan non-streaming dari permintaan streaming yang gagal mendapatkan status kesuksesan HTTP tetapi isi bukan pesan API Claude: biasanya halaman kesalahan HTML atau masuk, isi kosong, atau JSON dalam format lain. Proxy, gateway, atau halaman masuk jaringan yang menjawab di tempat API adalah sumber biasa. Claude Code tidak mengulang permintaan, dan giliran berakhir dengan kesalahan ini.

```text theme={null}
API returned an empty or malformed response (HTTP 200) — check for a proxy or gateway intercepting the request.
```

Setelah pembukaan itu, pesan melaporkan apa yang kembali dan permintaan mana yang gagal:

* Klausa `Response:` dengan tipe konten, jenis isi, seperti `body is an HTML page` atau `empty body`, ukurannya dalam byte, dan apakah respons membawa id permintaan Anthropic. Ketika respons menamai server yang dapat dikenali, seperti `nginx` atau `cloudflare`, atau membawa header perantara, seperti `cf-ray` atau `via`, klausa mencantumkan yang itu juga.
* Kalimat yang menamai id permintaan streaming yang gagal dan kegagalan yang memicu pengulangan. Ketika aliran telah dibuka sebelum kegagalan, ia juga melaporkan berapa banyak peristiwa aliran yang tiba dan, jika ada, berapa lama aliran telah diam ketika upaya gagal.

Sebelum v2.1.234, pesan berakhir setelah `intercepting the request`.

Sebelum v2.1.271, balasan yang membawa pesan API yang valid di bawah tipe konten non-JSON seperti `text/plain` juga mengakhiri giliran dengan kesalahan ini. Beberapa gateway LLM menggunakan tipe konten itu untuk balasan non-streaming.

**Yang harus dilakukan:**

* Baca klausa `Response:` untuk melihat sistem mana yang menjawab. Isi HTML, tidak ada id permintaan Anthropic, atau server bernama seperti `nginx` atau `cloudflare` berarti sesuatu antara Claude Code dan API menjawab di tempatnya
* Jika Anda merutekan melalui [gateway LLM](/docs/id/llm-gateway-connect#troubleshoot-gateway-errors), uji rute dengan permintaan langsung dan perbaiki hop yang mengembalikan respons non-API
* Di jaringan dengan halaman masuk, seperti Wi-Fi tamu, selesaikan masuk di browser, kemudian coba lagi
* Jika hanya rute non-streaming melalui gateway Anda yang rusak, atur [`CLAUDE_CODE_DISABLE_NONSTREAMING_FALLBACK=1`](/docs/id/env-vars#variables) sehingga permintaan yang gagal di tengah aliran masuk ke jalur pengulangan normal daripada fallback ini, kecuali ketika titik akhir streaming itu sendiri mengembalikan `404`, di mana Claude Code masih jatuh kembali

<h3 id="streaming-response-ended-before-any-complete-data-was-received">
  Streaming response ended before any complete data was received
</h3>

Respons streaming dari penyedia model Anda selesai tanpa memberikan data yang dapat digunakan, jadi Claude Code mengirim ulang permintaan tanpa streaming untuk menyelesaikan giliran. Claude Code menampilkan peringatan sekali per sesi, hanya dalam sesi interaktif. Sebelum v2.1.239, Claude Code diam-diam mengulang tanpa streaming.

```text theme={null}
Streaming response ended before any complete data was received. Retrying without streaming. If this keeps happening, check any proxy or gateway between Claude Code and your model provider.
```

Claude Code mengirim setiap permintaan yang terpengaruh dua kali: upaya streaming kosong dan pengulangan. Penyebab biasa adalah proxy atau gateway yang mengonsumsi atau mengubah isi respons streaming dalam perjalanannya kembali.

**Yang harus dilakukan:**

* Konfigurasikan proxy atau gateway apa pun antara Claude Code dan penyedia model Anda untuk melewatkan isi respons streaming dan headernya tanpa dimodifikasi
* Di [Amazon Bedrock](/docs/id/amazon-bedrock), lihat [Kesalahan streaming di belakang gateway atau proxy](/docs/id/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy) untuk persyaratan header dan isi

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock streaming response has an unexpected content-type
</h3>

Gateway atau proxy antara Claude Code dan [Amazon Bedrock](/docs/id/amazon-bedrock) mengubah isi respons streaming atau header `Content-Type` nya. Amazon Bedrock melakukan streaming respons sebagai `application/vnd.amazon.eventstream`. Daripada mendekode isi yang tidak dapat dibaca, Claude Code menolak respons streaming yang berhasil yang melaporkan tipe konten yang berbeda. Claude Code tidak mengulang permintaan.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Sebelum v2.1.208, kesalahan konfigurasi yang sama muncul sebagai `API Error: Truncated event message received` setelah seluruh respons telah di-buffer.

**Yang harus dilakukan:**

* Konfigurasikan gateway untuk melewatkan isi respons `InvokeModelWithResponseStream` dan header `Content-Type` nya tanpa dimodifikasi. Perantara yang memancarkan ulang aliran sebagai peristiwa yang dikirim server adalah penyebab umum.
* Menetapkan [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/id/env-vars) menyembunyikan kesalahan ini, tetapi Claude Code tidak mendekode isi biner di bawah header yang ditulis ulang, jadi permintaan itu jatuh kembali ke jalur non-streaming yang lebih lambat. Lihat [Kesalahan streaming di belakang gateway atau proxy](/docs/id/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  SSL certificate errors
</h3>

Proxy atau perangkat keamanan di jaringan Anda mencegat lalu lintas TLS dengan sertifikat mereka sendiri, dan Claude Code tidak mempercayainya.

```text theme={null}
Unable to connect to API: SSL certificate verification failed (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
Unable to connect to API: Self-signed certificate detected (SELF_SIGNED_CERT_IN_CHAIN). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
```

Sebelum v2.1.273, kedua pesan berakhir di `Check your proxy or corporate SSL certificates`, tanpa kode OpenSSL atau petunjuk `NODE_EXTRA_CA_CERTS`.

Mulai dari v2.1.199, kegagalan validasi sertifikat tidak diulang, jadi kesalahan ini muncul pada upaya pertama daripada setelah [anggaran pengulangan](#automatic-retries) penuh. Versi sebelumnya menghabiskan beberapa menit mengulang sebelum menampilkannya. Kondisi TLS sementara, seperti batas waktu jabat tangan, masih mengulang.

Selama `/login` dan pemeriksaan konektivitas startup, kegagalan yang sama menghasilkan pesan yang berbeda:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

Di [Amazon Bedrock](/docs/id/amazon-bedrock), permintaan yang Claude Code sendiri kirim ke AWS, seperti panggilan kredensial peran STS dan SSO, penemuan model, dan pemeriksaan wizard pengaturan, bergantung pada konfigurasi sertifikat yang sama. Lihat [Kesalahan sertifikat di belakang proxy yang memeriksa TLS](/docs/id/amazon-bedrock#certificate-errors-behind-a-tls-inspecting-proxy).

**Yang harus dilakukan:**

* Ekspor bundel CA organisasi Anda dan arahkan Claude Code ke sana dengan `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Lihat [Konfigurasi jaringan](/docs/id/network-config#custom-ca-certificates) untuk instruksi pengaturan lengkap
* Jangan atur `NODE_TLS_REJECT_UNAUTHORIZED=0`, yang menonaktifkan validasi sertifikat sepenuhnya

<h3 id="host-not-allowed-in-a-cloud-session">
  Host not allowed in a cloud session
</h3>

Permintaan HTTP keluar dari sesi cloud atau rutinitas diblokir oleh kebijakan jaringan lingkungan.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Anda juga dapat melihat sertifikat TLS yang tidak cocok dengan sertifikat asli tujuan. Sesi cloud merutekan lalu lintas keluar melalui proxy yang memberlakukan kebijakan jaringan, jadi sertifikat yang tidak cocok berarti proxy mengakhiri koneksi, bukan tujuan.

Ini bukan masalah jaringan sisi klien. Sesi cloud dan [rutinitas](/docs/id/routines) berjalan di dalam VM yang di-sandbox yang lalu lintas keluarnya melalui jaringan sesi disaring ke [daftar izin lingkungan cloud](/docs/id/cloud-environments); [operasi GitHub](/docs/id/cloud-environments#github-proxy) dan lalu lintas konektor MCP menggunakan saluran terpisah, itulah mengapa mereka dapat terus bekerja sementara host lain diblokir. Lingkungan **Default** menggunakan akses **Trusted**, yang memungkinkan [daftar izin default](/docs/id/cloud-environments#default-allowed-domains) dari registri paket, API penyedia cloud, registri kontainer, dan domain pengembangan umum dan memblokir domain lain di jalur itu.

**Yang harus dilakukan:**

Langkah-langkah ini mengubah salah satu lingkungan Anda sendiri. [Lingkungan bersama organisasi](/docs/id/cloud-environments#organization-shared-environments) terbuka hanya-baca di pemilih, jadi minta Pemilik untuk mengubah akses jaringannya dari halaman **Cloud environments** dalam [pengaturan admin](https://claude.ai/admin-settings).

* Buka rutinitas untuk pengeditan, atau mulai sesi cloud. Pilih ikon cloud yang menampilkan nama lingkungan Anda, seperti **Default**, untuk membuka pemilih. Arahkan ke lingkungan Anda dan klik ikon pengaturan.
* Dalam dialog **Update cloud environment**, ubah **Network access** dari **Trusted** ke **Custom**, kemudian tambahkan domain yang diblokir ke **Allowed domains**. Masukkan satu domain per baris. Periksa **Also include default list of common package managers** untuk menyimpan [daftar izin default](/docs/id/cloud-environments#default-allowed-domains) bersama domain kustom Anda. Pilih **Full** sebagai gantinya jika Anda menginginkan akses tanpa batas.
* Klik **Save changes**. Jalankan berikutnya menggunakan daftar izin yang diperbarui.

Lihat [Akses jaringan](/docs/id/cloud-environments#network-access) untuk tingkat akses dan daftar izin default. Sesi CLI lokal tidak terpengaruh oleh kebijakan ini.

<h3 id="the-proxy-refused-the-connection">
  The proxy refused the connection
</h3>

Anda melihat pesan ini ketika Claude membaca [artefak](/docs/id/artifacts) melalui proxy yang Anda atur dalam `HTTPS_PROXY` atau [variabel proxy](/docs/id/network-config#environment-variables) terkait. Konten artefak berasal dari `*.frame.claudeusercontent.com`, jadi Claude Code terlebih dahulu mengirim proxy permintaan `CONNECT` yang memintanya membuka terowongan ke host itu. Ketika proxy menolak, tidak ada yang mencapai host, dan pesan membawa status HTTP proxy:

```text theme={null}
artifact content fetch failed (proxy refused the connection: HTTP 407)
artifact content fetch failed (proxy refused the connection: HTTP 403)
the proxy refused the connection to the artifact's content host (HTTP 502)
```

Status adalah jawaban proxy terhadap `CONNECT`. Host tidak pernah menjawab, jadi setiap status menunjuk ke perbaikan yang berbeda:

* `HTTP 407`: proxy memerlukan kredensial yang tidak didapatkan. Masukkan mereka dalam URL proxy, seperti yang ditunjukkan [Autentikasi Dasar](/docs/id/network-config#basic-authentication).
* `HTTP 403`: proxy menolak untuk menggali terowongan ke `*.frame.claudeusercontent.com`. Minta siapa pun yang menjalankan proxy untuk memungkinkan host itu, yang [Persyaratan akses jaringan](/docs/id/network-config#network-access-requirements) cantumkan.
* Status apa pun yang lain, seperti `HTTP 502`: proxy tidak membuka terowongan karena alasannya sendiri, seperti gagal menjangkau host. Cari status dalam log proxy.
* `unreadable reply` sebagai pengganti status: apa pun yang ada di alamat proxy tidak menjawab dengan baris status HTTP. Periksa bahwa alamatnya adalah proxy HTTP.

**Yang harus dilakukan:**

* Periksa alamat dan kredensial dalam variabel proxy, seperti yang dijelaskan [Konfigurasi Proxy](/docs/id/network-config#proxy-configuration), kemudian jalankan `curl -x http://proxy.example.com:8080 -I https://api.anthropic.com` dari shell tempat Anda memulai Claude Code, menggunakan URL proxy Anda sendiri. Di Windows PowerShell, jalankan `curl.exe`. Jika probe ini gagal dengan cara yang sama, perbaiki pengaturan proxy terlebih dahulu. Jika berhasil, penolakan khusus untuk host artefak.
* Jika jaringan Anda membiarkan Claude Code menjangkau host artefak secara langsung, tambahkan `.frame.claudeusercontent.com` ke [`NO_PROXY`](/docs/id/network-config#environment-variables). Simpan entri itu sempit: entri `.claudeusercontent.com` yang lebih luas juga melewati proxy untuk `bridge.claudeusercontent.com`, yang organisasi dengan [IP allowlisting](/docs/id/network-config#organization-ip-allowlists-and-proxy-egress) perlu menyimpan di proxy.

Sebelum v2.1.238, Claude Code melaporkan terowongan yang ditolak sebagai kesalahan jaringan generik.

<h3 id="the-cloud-environments-service-returned-an-empty-or-unexpected-response">
  The cloud environments service returned an empty or unexpected response
</h3>

Claude Code meminta daftar [cloud environments](/docs/id/cloud-environments) Anda di beberapa titik, seperti ketika Anda membuat sesi cloud dari CLI atau menjalankan [`/remote-env`](/docs/id/cloud-environments#select-an-environment-from-the-cli). Ketika tidak dapat membaca jawaban server, menampilkan salah satu pesan ini:

```text theme={null}
The cloud environments service returned an empty response (HTTP 200 with no body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 with a non-JSON body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 without a usable environments list). This is usually temporary — try again in a moment.
```

Server menerima permintaan tetapi menjawab dengan isi yang bukan daftar lingkungan: kosong, bukan JSON, atau JSON tanpa daftar. Ini biasanya menyertai gangguan sisi layanan dan menghapus sendiri. Tergantung pada permukaan yang meminta daftar, Claude Code dapat menambahkan awalan, seperti `couldn't list environments:` dalam dialog `/remote-env`.

**Yang harus dilakukan:**

* Coba lagi tindakan. Claude Code meminta daftar lagi setiap kali
* Jika pesan terus muncul, periksa [status.claude.com](https://status.claude.com) untuk insiden aktif

Sebelum v2.1.236, Claude Code menampilkan TypeError JavaScript mentah daripada pesan-pesan ini.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Couldn't reconnect to your Remote Control session
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

Melanjutkan dengan `claude --resume` atau `claude --continue` menghubungkan kembali ke sesi [Remote Control](/docs/id/remote-control) yang dicatat dalam percakapan itu. Pesan ini berarti koneksi kembali gagal karena alasan yang mungkin sementara, seperti gangguan jaringan atau kesalahan server, jadi Claude Code tidak dapat mengkonfirmasi apakah sesi jarak jauh masih ada. Sesi lokal Anda terus berjalan tanpa Remote Control.

**Yang harus dilakukan:**

* Jalankan `/remote-control` untuk mencoba lagi koneksi
* Mulai sesi baru dengan `claude --remote-control` untuk membuat sesi Remote Control baru
* Untuk pesan startup Remote Control lainnya, lihat [Troubleshoot Remote Control](/docs/id/remote-control#troubleshooting)

Jika server melaporkan sebaliknya bahwa sesi sebelumnya hilang, Anda tidak melihat pesan ini. Claude Code memulai sesi baru di tempatnya atau menampilkan [`Previous session is unavailable — run /remote-control to start a new one`](/docs/id/remote-control#previous-session-is-unavailable), tergantung pada [catatan koneksi kembali percakapan](/docs/id/remote-control#resume-outcomes). Dari v2.1.227 hingga v2.1.231, Claude Code menampilkan pesan yang dimulai dengan `Remote Control could not resume the previous session under the current login` sebagai gantinya, dan [versi sebelumnya berperilaku berbeda lagi](/docs/id/remote-control#reconnect-history).

<h3 id="sessions-ended-while-this-machine-was-offline">
  Sessions ended while this machine was offline
</h3>

Claude Code menampilkan pesan ini di terminal yang menjalankan [`claude remote-control`](/docs/id/remote-control#start-a-remote-control-session) setelah mesin Anda offline cukup lama sehingga server membersihkan lingkungan Remote Control yang mesin Anda layani. Sesi di lingkungan itu berakhir, dan Anda tidak dapat melanjutkannya. Hitungannya adalah jumlah sesi yang berakhir.

```text theme={null}
2 sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.
```

**Yang harus dilakukan:**

* Ketika Claude Code mencantumkan worktrees yang disimpan di bawah pesan ini, ambil pekerjaan yang tidak dikomit dari mereka
* Jalankan `claude remote-control` untuk memulai lingkungan segar

<h3 id="couldnt-share-the-transcript">
  Couldn't share the transcript
</h3>

Setelah Anda setuju untuk berbagi transkrip sesi Anda dari prompt survei, seperti [survei kualitas sesi](/docs/id/data-usage#session-quality-surveys), Claude Code mengunggahnya ke Anthropic, atau menyimpan arsip lokal sebagai gantinya di penyedia pihak ketiga, di sesi [Claude apps gateway](/docs/id/claude-apps-gateway), dan ketika tidak ada kredensial Anthropic yang tersedia. Pesan ini berarti berbagi tidak selesai.

```text theme={null}
Couldn't share the transcript.
```

Unggahan harus sesuai dengan batas 8 MiB. Dalam sesi yang panjang, Claude Code secara progresif menjatuhkan bagian dari berbagi, pengaturan model permintaan terakhir terlebih dahulu, kemudian percakapan terstruktur dan transkrip subagen, dan menampilkan pesan ini hanya ketika tidak ada versi yang dikurangi yang dapat dikirim atau kesalahan jaringan atau server menghentikan unggahan. Ketika Claude Code menyimpan arsip lokal sebagai gantinya, pesan berarti tidak dapat menulis arsip.

**Yang harus dilakukan:**

* Jalankan `/feedback` untuk mengirim transkrip dengan deskripsi apa yang terjadi. Lihat [Report an error](#report-an-error) jika `/feedback` tidak tersedia di lingkungan Anda
* Jika permintaan lain juga gagal, periksa koneksi jaringan Anda dan lihat [Unable to connect to API](#unable-to-connect-to-api)

<h2 id="request-errors">
  Kesalahan permintaan
</h2>

Kesalahan ini berkaitan dengan konten permintaan Anda. Sebagian besar kembali dari API setelah menolak permintaan; beberapa diproduksi secara lokal oleh Claude Code sebelum permintaan dikirim.

<h3 id="prompt-is-too-long">
  Prompt terlalu panjang
</h3>

Percakapan ditambah file yang dilampirkan melebihi jendela konteks model.

```text theme={null}
Prompt is too long
```

Dalam sesi interaktif, Claude Code menampilkan kesalahan ini sebagai:

```text theme={null}
Context limit reached · /compact or /clear to continue
```

Baris hanya menyebutkan `/clear` ketika [`DISABLE_COMPACT`](/docs/id/env-vars) diatur. Bentuk kesalahan yang lebih panjang, seperti bentuk kegagalan pemadatan di bawah, tetap mempertahankan pesan `Prompt is too long ·`. Dalam output `-p` dan transkrip, teksnya tetap `Prompt is too long`.

Ketika Anda mematikan auto-compact di [pengaturan pengguna](/docs/id/settings-reference#autocompactenabled) Anda, baris juga mengatakan demikian:

```text theme={null}
Context limit reached · /compact or /clear to continue · auto-compact is off · /config to turn it on
```

Tombol **Auto-compact** di `/config` menulis `autoCompactEnabled` ke pengaturan pengguna. Petunjuk muncul hanya ketika perubahan `/config` akan berlaku. Misalnya, tidak muncul ketika [`DISABLE_AUTO_COMPACT`](/docs/id/env-vars) atau [`DISABLE_COMPACT`](/docs/id/env-vars) mematikan auto-compact. Juga tidak muncul ketika cakupan prioritas lebih tinggi, seperti pengaturan proyek atau terkelola, menetapkan `autoCompactEnabled` ke `false`. Sebelum v2.1.235, baris tidak membawa petunjuk auto-compact.

Amazon Bedrock melaporkan kondisi ini sebagai `Input is too long for requested model.`, yang Claude Code tangani dengan cara yang sama. Sebelum v2.1.217, Claude Code tidak mengenali pesan Bedrock, jadi auto-compact tidak pernah dipicu dan `/compact` gagal dengan kesalahan yang sama.

Gateway [aplikasi Claude](/docs/id/claude-apps-gateway-config#upstream-error-messages) melaporkan kondisi ini sebagai `capability_rejected: prompt_too_long` ketika upstream cloud menolak permintaan dalam bentuk kesalahan penyedia sendiri. Claude Code memperlakukan token sama dengan `Prompt is too long`. Sebelum v2.1.228, Claude Code tidak mengenali token, jadi auto-compact tidak dipicu.

Ketika pemadatan otomatis berjalan pada giliran ini dan gagal pada kesalahan yang mendasar, seperti model yang tidak tersedia atau kegagalan autentikasi, pesan menyebutkan kesalahan itu setelah pemisah:

```text theme={null}
Prompt is too long · automatic compaction failed: <the underlying error>
```

Selesaikan kesalahan yang dinamai terlebih dahulu; `/compact` gagal pada kesalahan yang sama sampai Anda melakukannya. Sebelum v2.1.229, pemadatan otomatis yang gagal menampilkan `Prompt is too long` tanpa penyebabnya.

Ketika pemadatan otomatis berjalan pada kesalahan ini, biasanya merangkum pertukaran tertua Anda dan menyimpan yang terbaru. Sebagai upaya terakhir, Claude Code merangkum secara berbeda:

* Ketika tidak dapat merangkum seluruh pertukaran, Claude Code menyimpan prompt terbaru Anda kata demi kata dan merangkum semuanya sebelumnya.
* Dalam hal itu, ketika percakapan tidak berakhir dengan prompt Anda, Claude Code merangkum seluruh percakapan sebagai gantinya.

Claude Code melewati pemulihan ini ketika konten yang akan dibawanya tidak menyimpan balasan model dan kurang dari sekitar 1.000 token teks Anda sendiri, seperti pengiriman ulang singkat setelah tempel berukuran besar. Jalankan `/clear` untuk memulai segar. Sebelum v2.1.269, pemadatan gagal setiap kali tidak dapat merangkum seluruh pertukaran, jadi sesi dalam keadaan itu mencapai kesalahan ini lagi pada setiap giliran.

Percakapan pertukaran tunggal tidak memiliki giliran sebelumnya untuk dirangkum. Ketika pemadatan otomatis akan berjalan pada satu, Claude Code melewati upaya dan menjelaskan apa yang mengisi permintaan sebagai gantinya. Ketika API tidak melaporkan jumlah token dalam kesalahannya, pesan berbunyi:

```text theme={null}
Prompt is too long · this conversation is a single exchange and cannot be compacted — the request size comes mostly from system prompt, tool definitions, or attachments.
```

Ketika API melaporkan jumlah token dalam kesalahannya, Claude Code membandingkannya dengan estimasi sendiri tentang ukuran percakapan untuk mengetahui mana yang paling banyak dari permintaan: konten percakapan sendiri, atau prompt sistem, definisi alat, dan konten lampiran yang Claude Code kirim dengannya. Ketika konten percakapan sendiri paling banyak dari permintaan, pesan berbunyi:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) and this conversation's own content is most of it. A single-exchange conversation cannot be compacted; start with less content (smaller files or pasted text).
```

Ketika sebagian besar permintaan berada di luar percakapan, pesan berbunyi:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) but this conversation is only ~<conversation tokens> tokens — the rest is system prompt, tool definitions, and attachment content. A single-exchange conversation cannot be compacted; reduce attached files/tools or start with less context.
```

Sebelum v2.1.162, Claude Code mencoba pemadatan bagaimanapun dan menampilkan `Prompt is too long` telanjang ketika gagal.

**Yang harus dilakukan:**

* Jalankan `/compact` untuk merangkum giliran sebelumnya dan membebaskan ruang, atau `/clear` untuk memulai segar. Jika `/compact` menjawab `Not enough messages to compact.`, percakapan adalah pertukaran tunggal tanpa apa pun sebelumnya untuk dirangkum, jadi ruang diambil oleh prompt itu dan apa yang Claude Code kirim dengan setiap permintaan: jalankan `/clear` dan kirim ulang dengan teks tempel lebih sedikit atau lampiran lebih kecil, atau kurangi definisi alat dan file memori menggunakan langkah-langkah di bawah
* Jalankan `/context` untuk melihat rincian apa yang mengonsumsi jendela: prompt sistem, alat, file memori, dan pesan
* Nonaktifkan server MCP yang tidak Anda gunakan dengan `/mcp disable <name>` untuk menghapus definisi alat mereka dari konteks
* Pangkas file memori `CLAUDE.md` besar, atau pindahkan instruksi ke [aturan bersifat jalur](/docs/id/memory#path-specific-rules) yang dimuat hanya ketika relevan
* Subagen mewarisi setiap definisi alat MCP dari sesi induk, yang dapat mengisi jendela konteks mereka sebelum giliran pertama. Nonaktifkan server MCP yang tidak Anda gunakan sebelum menelurkan subagen.
* Auto-compact aktif secara default dan biasanya mencegah kesalahan ini. Jika Anda mematikannya di `/config` atau dengan [`DISABLE_AUTO_COMPACT`](/docs/id/env-vars), aktifkan kembali. Jika Anda tetap mematikannya, jalankan `/compact` sendiri sebelum jendela penuh.

Lihat [Jelajahi jendela konteks](/docs/id/context-window) untuk tampilan interaktif tentang bagaimana konteks terisi.

<h3 id="context-exceeds-the-token-limit">
  Konteks melebihi batas token
</h3>

`/context` menampilkan peringatan ini di bagian atas outputnya ketika percakapan telah melampaui jendela konteks model. Permintaan gagal dengan [`Prompt is too long`](#prompt-is-too-long) sampai Anda membebaskan ruang. Sesi interaktif menampilkan kesalahan itu sebagai baris `Context limit reached`.

```text theme={null}
Context exceeds the 200k-token limit by 94k tokens — run /compact or /clear to continue.
```

Ketika batas yang Anda lampaui adalah jendela pemadatan, seperti batas 200K pada model konteks 1M, peringatan berbunyi berbeda. Jendela pemadatan dapat duduk di bawah jendela konteks model, jadi permintaan melewatinya masih dapat berhasil.

```text theme={null}
Context is 94k tokens past the 200k-token compaction window — run /compact to reduce usage.
```

Kedua bentuk menyebutkan `/clear` bukan `/compact` ketika Anda telah menetapkan [`DISABLE_COMPACT`](/docs/id/env-vars).

**Yang harus dilakukan:**

* Dalam percakapan multi-giliran, jalankan `/compact` untuk merangkum giliran sebelumnya dan membebaskan ruang. Untuk memulai segar sebagai gantinya, jalankan `/clear`
* Untuk lebih banyak cara mengurangi penggunaan, lihat [Prompt is too long](#prompt-is-too-long)

Sebelum v2.1.216, `/context` menampilkan penggunaan di atas 100% tanpa baris peringatan yang menjelaskan apa artinya atau cara pulih.

<h3 id="error-during-compaction-conversation-too-long">
  Kesalahan selama pemadatan: Percakapan terlalu panjang
</h3>

`/compact` sendiri gagal karena tidak ada cukup konteks bebas untuk menampung ringkasan yang dihasilkannya.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Ini dapat terjadi ketika jendela sudah penuh pada saat auto-compact dipicu, atau ketika Anda menjalankan `/compact` setelah melihat [`Prompt is too long`](#prompt-is-too-long). Dalam sesi interaktif, kesalahan itu adalah baris `Context limit reached`.

**Yang harus dilakukan:**

* Tekan Esc dua kali untuk membuka daftar pesan dan mundur beberapa giliran. Ini menghilangkan pesan terbaru dari konteks. Kemudian jalankan `/compact` lagi.
* Jika mundur tidak membebaskan cukup ruang, jalankan `/clear` untuk memulai sesi segar. Percakapan sebelumnya Anda disimpan dan dapat dibuka kembali dengan `/resume`.

Pesan ini dan kegagalan `/compact` lainnya ditampilkan dalam gaya kesalahan. Sebelum v2.1.216, mereka dirender dalam gaya redup yang sama dengan output perintah yang berhasil, jadi Anda dapat membaca pemadatan yang gagal sebagai kesuksesan.

<h3 id="request-too-large">
  Permintaan terlalu besar
</h3>

Badan permintaan mentah melebihi batas 32MB API sebelum tokenisasi, biasanya karena konten tempel besar, hasil alat, atau lampiran. Batas ini terpisah dari [jendela konteks](#prompt-is-too-long).

```text theme={null}
Request too large (max 32MB). Accumulated images and attachments in the conversation pushed the request over the limit. Run /compact, or double press esc to go back and remove attachments.
```

Ketika permintaan langsung ke API Claude dan API sendiri menolaknya, Claude Code mengukur percakapan dan merumuskan pesan berdasarkan apakah pemulihan dapat bekerja. Melalui proxy, gateway, atau penyedia cloud Anda mendapatkan pesan umum. Bentuk yang diukur:

* `Request too large (max 32MB; 20.1MB of about 33.4MB is images or documents).`: gambar atau dokumen mendorong permintaan melampaui batas. Claude Code mencoba ulang dengan mereka dihilangkan.
* `Request too large for the API's 32MB request limit`: pesan saja sudah melampaui batas, jadi pesan mengatakan `compacting cannot make it fit` dan Claude Code tidak mencoba ulang. Dalam [mode non-interaktif](/docs/id/headless), pesan memberi tahu Anda untuk mengurangi input atau memulai sesi baru sebagai gantinya.

Sebelum v2.1.212, percakapan dengan cukup gambar terakumulasi gagal pada setiap giliran dengan `Request too large (max 32MB). Double press esc to go back and try with a smaller file.` Sebelum v2.1.229, Claude Code menampilkan saran lampiran untuk setiap penolakan, bahkan ketika pemadatan tidak dapat membantu.

**Yang harus dilakukan:**

* Jika pesan mengatakan `compacting cannot make it fit`, tekan Esc dua kali untuk mundur melampaui giliran yang menambahkan konten besar, atau jalankan `/clear` untuk memulai segar
* Jika tidak, jalankan `/compact`, yang menghilangkan gambar dan lampiran terakumulasi
* Referensikan file besar berdasarkan jalur bukan menempel konten mereka, jadi Claude dapat membacanya dalam potongan
* Untuk gambar, lihat [Image was too large](#image-was-too-large) di bawah

<h3 id="image-was-too-large">
  Gambar terlalu besar
</h3>

Gambar yang ditempel atau dilampirkan melebihi batas ukuran atau dimensi API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code mengganti gambar yang tidak dapat diproses dengan placeholder teks dan mencoba ulang, jadi pesan berikutnya berhasil. Pada versi sebelum 2.1.142, gambar yang ditempel dapat tetap berada dalam percakapan dan mengulangi kesalahan yang sama pada setiap pesan berikutnya. Untuk pulih pada versi tersebut, tekan Esc dua kali dan mundur melampaui giliran tempat gambar ditambahkan.

**Yang harus dilakukan:**

* Ubah ukuran gambar sebelum menempel. API menerima gambar hingga 8000 piksel di tepi terpanjang untuk satu gambar, atau 2000 piksel ketika banyak gambar berada dalam konteks.
* Ambil tangkapan layar yang lebih ketat dari wilayah yang relevan bukan layar penuh

<h3 id="unable-to-resize-image">
  Tidak dapat mengubah ukuran gambar
</h3>

Claude Code tidak dapat mengurangi skala gambar yang dilampirkan sebelum mengirimnya ke API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
Unable to resize image — it is a CMYK JPEG, which Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Re-save it as an RGB PNG or JPEG and try again.
Unable to resize image — it is an animated WebP whose first frame Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Save its first frame as a PNG or JPEG and try again.
Unable to resize image — its pixels could not be decoded (the file may be damaged, or use an encoding Claude Code cannot read), and it is over the … API limit (… raw, … base64), so it cannot be sent. Re-save it as a PNG or JPEG and try again.
```

Claude Code biasanya mengubah ukuran gambar besar secara otomatis. Kesalahan ini berarti gambar tidak dapat didekode atau diubah ukurannya agar sesuai dengan batas API.

**Yang harus dilakukan:**

* Jika pesan meminta Anda mengonversi gambar, konversikan ke PNG, JPEG, GIF, atau WebP dan lampirkan lagi. Claude Code dapat memverifikasi dimensi untuk format ini dari header file, tanpa mendekode gambar.
* Jika pesan melaporkan batas dimensi atau ukuran, ubah ukuran atau kompres ulang gambar di bawah batas itu sebelum melampirkan.
* Jika pesan menyebutkan penyebab, seperti JPEG CMYK, WebP animasi, atau file yang mungkin rusak, simpan ulang gambar dalam format yang disarankan pesan dan lampirkan lagi.

<h3 id="pdf-errors">
  Kesalahan PDF
</h3>

PDF yang Anda lampirkan tidak dapat diproses. Pesan ditampilkan di sini dalam bentuk non-interaktif mereka; dalam sesi interaktif mereka malah meminta Anda untuk tekan esc dua kali dan coba lagi.

```text theme={null}
PDF too large (max 100 pages, 20MB). Try reading the file a different way (e.g., extract text with pdftotext).
PDF is password protected. Try using a CLI tool to extract or convert the PDF.
The PDF file was not valid. Try converting it to text first (e.g., pdftotext).
```

**Yang harus dilakukan:**

* Untuk PDF berukuran besar, minta Claude membaca rentang halaman dengan alat Read bukan melampirkan seluruh file, atau ekstrak teks dengan alat seperti `pdftotext` dan referensikan file output berdasarkan jalur
* Untuk PDF yang dilindungi atau tidak valid, hapus kata sandi atau ekspor ulang file dari aplikasi sumbernya, kemudian coba lagi

<h3 id="extra-inputs-are-not-permitted">
  Input tambahan tidak diizinkan
</h3>

Proxy atau gateway LLM antara Claude Code dan API menghilangkan header permintaan `anthropic-beta`, jadi API menolak bidang yang bergantung padanya.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code mengirim bidang khusus beta seperti `context_management` dan `effort` bersama header `anthropic-beta` yang mengaktifkannya. Ketika gateway meneruskan badan tetapi menghilangkan header, API melihat bidang yang tidak dikenalinya.

**Yang harus dilakukan:**

* Konfigurasikan gateway Anda untuk meneruskan header `anthropic-beta`. Lihat [feature pass-through](/docs/id/llm-gateway-protocol#feature-pass-through) untuk apa yang harus diteruskan gateway.
* Sebagai fallback, atur [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/id/env-vars) sebelum meluncurkan. [Disable pre-release capabilities](/docs/id/llm-gateway-protocol#disable-pre-release-capabilities) mencakup cakupan yang tepat.

<h3 id="tool-input-schema-is-invalid">
  Skema input alat tidak valid
</h3>

Alat dalam permintaan mendeklarasikan `input_schema` yang gagal validasi JSON Schema API, jadi API menolak seluruh permintaan. Angka setelah `tools.` adalah posisi alat yang gagal dalam daftar alat permintaan, bukan nama yang dapat Anda cari.

```text theme={null}
API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid
API Error: 400 ... tools.N.custom.input_schema.properties: Property keys should match pattern '^[a-zA-Z0-9_.-]{1,64}$'
```

Bentuk pertama berarti skema bukan JSON Schema draft 2020-12 yang valid. Bentuk kedua berarti nama properti tingkat atas tidak cocok dengan pola yang dikutip pesan.

Claude Code [mengecualikan alat MCP yang skema input-nya akan gagal validasi ini](/docs/id/mcp#tools-with-invalid-input-schemas) ketika memuat alat server, jadi permintaan biasanya tidak pernah menyertakan satu.

Pada [deployment di mana pengambilan flag dimatikan](/docs/id/env-vars#features-that-need-feature-flag-fetching), atau pada mesin yang flagnya tidak pernah tiba, Claude Code mencatat dalam log server alat mana yang akan ditolak tetapi mengirimnya bagaimanapun, jadi kesalahan ini masih dapat terjadi.

Kesalahan juga dapat terjadi untuk alat yang skemanya mendeklarasikan dialek JSON Schema selain draft 2020-12 di `$schema`. Claude Code tidak memeriksa skema tersebut terhadap meta-skema JSON Schema, meskipun pemeriksaan nama properti tingkat atas masih berlaku.

Sebelum v2.1.216, tidak ada deployment yang menjalankan pemeriksaan pengecualian.

**Yang harus dilakukan:**

* Jika versi Claude Code Anda lebih awal dari v2.1.216, jalankan `claude update`.
* Hapus atau [nonaktifkan](/docs/id/mcp#disable-a-server-without-removing-it) server MCP yang mendeklarasikan skema tidak valid. Kesalahan hanya menyebutkan alat berdasarkan posisi. Pada v2.1.216 atau lebih baru, periksa log setiap server untuk baris yang menyebutkan alat yang skema input-nya akan ditolak. Jika tidak ada log yang menyebutkan satu, nonaktifkan server satu per satu.
* Jika Anda memelihara server, perbaiki `input_schema` alat. Skema harus berupa JSON Schema yang valid, dan nama properti tingkat atas harus 1 hingga 64 karakter panjang dan hanya menggunakan huruf ASCII dan digit, `_`, `.`, dan `-`. Lihat [Tools with invalid input schemas](/docs/id/mcp#tools-with-invalid-input-schemas).

<h3 id="theres-an-issue-with-the-selected-model">
  Ada masalah dengan model yang dipilih
</h3>

Nama model yang dikonfigurasi tidak dikenali atau akun Anda tidak memiliki akses ke model tersebut. Mulai dari v2.1.160 petunjuk trailing, ditampilkan di sini dalam bentuk interaktifnya, bervariasi menurut permukaan.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Yang harus dilakukan:**

* **CLI Interaktif**: jalankan `/model` untuk memilih dari model yang tersedia untuk akun Anda.
* **Mode non-interaktif (`-p`)**: teruskan `--model` dengan alias atau ID yang valid, atau atur [`ANTHROPIC_MODEL`](/docs/id/env-vars). Teks kesalahan menampilkan `Run --model` di permukaan ini.
* **Agent SDK**: teks kesalahan menghilangkan petunjuk karena model diatur secara terprogram. Atur [`model` pada `Options`](/docs/id/agent-sdk/typescript#options) di TypeScript atau [`ClaudeAgentOptions(model=...)`](/docs/id/agent-sdk/python#claudeagentoptions) di Python, dan tangani kesalahan terstruktur `model_not_found` untuk menampilkan pemilih ulang atau pemilih model Anda sendiri.
* Gunakan alias seperti `sonnet` atau `opus` bukan ID versi lengkap. Alias menyelesaikan ke default yang dipertahankan sehingga tidak menjadi usang. Lihat [Model configuration](/docs/id/model-config).
* Jika model yang salah terus kembali di CLI, ID usang diatur di suatu tempat. Periksa tempat Anda dapat mengatur model dalam [urutan prioritas](/docs/id/model-config#setting-your-model) dan hapus nilai usang.
* Model yang baru diluncurkan dapat tersedia di API Anthropic sebelum Amazon Bedrock, Platform Agent Google Cloud, atau Foundry Microsoft menawarkannya. Jika Anda menyematkan ID model baru pada salah satu penyedia tersebut dan melihat kesalahan ini, periksa katalog model penyedia Anda untuk ketersediaan di wilayah Anda, dan pertahankan versi sebelumnya yang disematkan sampai yang baru muncul di sana.
* Claude Code melaporkan login claude.ai yang kedaluwarsa sebagai [Login expired](#login-expired), bukan sebagai kesalahan ini. Sebelum v2.1.206, login yang kedaluwarsa yang tidak lagi dapat disegarkan gagal di setiap model dengan kesalahan ini; jalankan `/login` jika Anda melihat itu pada versi yang lebih lama.
* Untuk deployment Platform Agent Google Cloud, lihat [Troubleshooting Platform Agent Google Cloud](/docs/id/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Model bukan ID model yang dikenali
</h3>

String model yang Anda teruskan ke sakelar model bukan alias model, ID model yang diketahui versi Claude Code ini, atau ID yang dimulai dengan `claude-`. Penyebab biasanya adalah kesalahan ketik di ID, nama tampilan seperti `Sonnet 5` di mana ID `claude-sonnet-5` diharapkan, atau alias yang hanya dikenali versi Claude Code yang lebih baru. Claude Code menolak sakelar segera. Sebelum v2.1.200, Claude Code menyimpan string dan gagal pada permintaan berikutnya dengan [Ada masalah dengan model yang dipilih](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

Petunjuk trailing menyebutkan alias atau ID model yang paling cocok. Ketika tidak ada yang cukup dekat, itu berbunyi `Run /model to see available models.` sebagai gantinya.

Claude Code menghasilkan kesalahan ini secara lokal pada saat sakelar diminta, sebelum permintaan API apa pun dibuat. Ini berlaku ketika model diatur melalui metode [Agent SDK](/docs/id/agent-sdk/typescript) `setModel()`, oleh aplikasi seperti [Desktop app](/docs/id/desktop) yang menjalankan CLI Claude Code untuk Anda, atau ketika Anda memilih model dari perangkat yang terhubung melalui [Remote Control](/docs/id/remote-control). Sebelum v2.1.260, pemeriksaan tidak mencakup pilihan Remote Control, jadi Claude Code menerapkan pilihan dan permintaan berikutnya gagal dengan [Ada masalah dengan model yang dipilih](#theres-an-issue-with-the-selected-model).

**Yang harus dilakukan:**

* Jalankan `/model` tanpa argumen untuk membuka pemilih dan pilih dari model yang tersedia untuk akun Anda, kemudian teruskan alias atau ID yang ditampilkan di sana
* Jika Anda menggunakan alias yang didukung versi Claude Code yang lebih baru, jalankan `claude update`. ID lengkap yang dimulai dengan `claude-` melewati pemeriksaan lokal ini bahkan ketika model lebih baru dari versi Claude Code Anda. Server masih dapat memerlukan versi minimum untuk model itu; lihat [Claude Code does not support this model](#claude-code-does-not-support-this-model).
* Model yang disimpan sebelum v2.1.200 tidak diperbaiki oleh pemeriksaan ini. Jika nilai usang terus kembali, hapus dari lokasi yang tercantum di bawah [Setting your model](/docs/id/model-config#setting-your-model).
* Pemeriksaan hanya berjalan di API Anthropic. Pada penyedia atau gateway lain apa pun, termasuk `ANTHROPIC_BASE_URL` kustom, penyedia menentukan nama model, jadi Claude Code menerima string apa pun dan meneruskannya. Claude Code masih dapat menulis [baris diagnostik model yang tidak dikenali](#unrecognized-model-id-on-a-request) pada waktu permintaan, di setiap penyedia.

<h3 id="model-not-found">
  Model tidak ditemukan
</h3>

Anda memilih model dengan `/model <name>` dan Claude Code tidak dapat mengkonfirmasi bahwa model dengan nama itu ada. Ketika nama bukan [alias model](/docs/id/model-config#model-aliases) atau ejaan lain yang Claude Code terima secara lokal, `/model` memverifikasinya dengan permintaan API minimal, dan kesalahan ini biasanya adalah jawaban endpoint API Anda. Nama yang tidak dapat menjadi ID model sama sekali, seperti yang berisi spasi, mendapatkan pesan yang sama.

```text theme={null}
Model 'claude-opus-9' not found
```

Pada penyedia dengan ID model khusus penyedia, pesan dapat menambahkan saran `Try '...' instead` yang menyebutkan ID penyedia Anda untuk model fallback.

**Yang harus dilakukan:**

* Jalankan `/model` tanpa argumen dan pilih dari model yang tersedia untuk akun Anda, atau gunakan [alias model](/docs/id/model-config#model-aliases) seperti `sonnet`, yang menyelesaikan ke default yang dipertahankan
* Jika Anda mengetik ID lengkap, periksa terhadap katalog model penyedia Anda. Model yang baru diluncurkan dapat tersedia di API Anthropic sebelum penyedia atau wilayah Anda menawarkannya.
* Sebelum v2.1.265, `/model` juga menolak ejaan alias `opusplan[1m]` dengan kesalahan ini. Pada versi tersebut, perbarui Claude Code, atau atur model di [settings](/docs/id/model-config#setting-your-model) atau dengan `--model` sebagai gantinya.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus tidak tersedia dengan paket Claude Pro
</h3>

Paket langganan aktif Anda tidak menyertakan model yang Anda pilih.

```text theme={null}
Claude Opus is not available with the Claude Pro plan. If you have updated your subscription plan recently, run /logout and /login for the plan to take effect.
```

**Yang harus dilakukan:**

* Jalankan `/model` dan pilih model yang disertakan paket Anda
* Jika Anda baru-baru ini meningkatkan paket dan masih melihat ini, jalankan `/logout` kemudian `/login`. Token yang disimpan mencerminkan paket Anda pada saat Anda masuk, jadi meningkatkan di claude.ai tidak berlaku dalam sesi yang ada sampai Anda autentikasi ulang.
* Lihat [claude.com/pricing](https://claude.com/pricing) untuk model mana yang disertakan setiap paket

<h3 id="claude-code-does-not-support-this-model">
  Claude Code tidak mendukung model ini
</h3>

API menolak permintaan dengan 400 karena versi Claude Code Anda di bawah minimum yang diperlukan. Baik model yang Anda pilih memerlukan versi yang lebih baru, yang diperiksa server per model, atau kebijakan organisasi Anda memerlukan satu. 400 membawa kode kesalahan `claude_code_version_too_old`, dan pesan mengatakan minimum mana yang berlaku.

```text theme={null}
API Error: 400 Claude Code 2.1.219 does not support this model; version 2.1.255 or newer is required. Run 'claude update', or update the Claude desktop app, then try again.
```

Pesan kebijakan organisasi berbunyi:

```text theme={null}
API Error: 400 Claude Code 2.1.240 is older than the minimum version required by your organization's policy. Run 'claude update', or update the Claude desktop app, to continue.
```

**Yang harus dilakukan:**

* Jalankan `claude update`, atau perbarui aplikasi desktop Claude, kemudian mulai sesi baru
* Untuk pesan per-model, Anda dapat terus bekerja dalam sesi saat ini dengan beralih ke model lain dengan `/model`
* Untuk pesan kebijakan organisasi, perbarui sebelum Anda melanjutkan

<h3 id="model-is-restricted-by-your-organizations-settings">
  Model dibatasi oleh pengaturan organisasi Anda
</h3>

Admin organisasi Anda telah menonaktifkan model ini di konsol admin claude.ai, atau dikecualikan oleh daftar izin [`availableModels`](/docs/id/model-config#restrict-model-selection) dalam pengaturan terkelola. Ketika model yang dibatasi diatur dengan `--model`, `ANTHROPIC_MODEL`, atau pengaturan `model`, Claude Code mengganti model yang diizinkan dan melanjutkan. Mengetik `/model <name>` untuk model yang dibatasi ditolak dengan `Run /model to choose a different model.` dan sesi menyimpan model saat ini. Pemberitahuan substitusi juga dapat muncul di tengah sesi setelah admin menonaktifkan model yang sedang dijalankan sesi di konsol admin claude.ai.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Pemberitahuan dengan awalan nama agen, skill, atau perintah berarti pembatasan diterapkan pada [model yang diminta subagen](/docs/id/sub-agents#choose-a-model): subagen berjalan pada model yang diganti dan model sesi Anda tidak berubah. Sebelum v2.1.223, Claude Code menampilkan pemberitahuan hanya untuk subagen yang diluncurkan dengan alat Agent.

Claude Code memperlakukan alias keluarga model, salah satu dari `opus`, `sonnet`, `haiku`, atau `fable`, sebagai permintaan untuk keluarga itu bukan untuk versi terbarunya. Di API Anthropic dan di [Claude Platform on AWS](/docs/id/claude-platform-on-aws), alias keluarga yang dibatasi menyelesaikan ke versi terbaru keluarga yang diizinkan organisasi Anda dan daftar izin `availableModels`, dan pemberitahuan substitusi menyebutkan versi itu. Claude Code menolak `/model <alias>` hanya ketika setiap versi keluarga dibatasi. Sebelum v2.1.205, alias keluarga diganti atau ditolak berdasarkan versi terbarunya saja, bahkan ketika versi yang lebih lama dari keluarga yang sama diizinkan.

**Yang harus dilakukan:**

* Jalankan `/model` untuk memilih dari model yang diizinkan organisasi Anda. Model yang dibatasi disembunyikan dari pemilih.
* Jika model yang dibatasi diatur di `--model`, `ANTHROPIC_MODEL`, bidang `model` file pengaturan, atau frontmatter `model` dari [subagen](/docs/id/sub-agents#choose-a-model), skill, atau perintah, hapus atau perbarui nilai itu sehingga pemberitahuan tidak berulang
* Jika Anda memerlukan akses ke model yang dibatasi, minta admin organisasi Anda untuk mengaktifkannya. Lihat [Organization model restrictions](/docs/id/model-config#organization-model-restrictions).

<h3 id="model-switch-was-blocked-by-a-premodelswitch-hook">
  Sakelar model diblokir oleh hook PreModelSwitch
</h3>

Hook [PreModelSwitch](/docs/id/hooks#premodelswitch) tidak menyetujui sakelar model yang Anda atau klien minta, jadi sesi menyimpan model saat ini. Ketika sakelar berasal dari host [Agent SDK](/docs/id/agent-sdk/overview) atau [Remote Control](/docs/id/remote-control) bukan perintah yang Anda ketik, pesan berbunyi `Model switch blocked by a PreModelSwitch hook` tanpa menyebutkan model target.

```text theme={null}
Model switch to Opus 4.6 was blocked by a PreModelSwitch hook: Opus 4.6 is retired for this project. Use a newer model.
```

Alasan setelah titik dua mengatakan apa yang menolak sakelar:

* **Alasan yang ditulis hook**: hook PreModelSwitch menyediakan alasan itu ketika [menolak sakelar atau meminta konfirmasi](/docs/id/hooks#premodelswitch-decision-control). Tangani apa yang dimintanya, atau pilih model yang diizinkan hook Anda.
* **`PreModelSwitch hook <name> did not respond before its timeout`**: hook yang tidak menjawab sebelum [timeout](/docs/id/hooks#timeouts)-nya memblokir sakelar. Perbaiki perintah yang menggantung atau naikkan `timeout` hook itu, kemudian sakelar lagi.
* **`confirmation required, and this session cannot ask`**: hook menjawab `ask` tanpa alasan, dan permintaan kontrol tidak memiliki cara untuk menampilkan prompt konfirmasi. Perintah `/model` dalam run [`-p`](/docs/id/headless) melaporkan kondisi yang sama dengan `(run /model interactively to confirm)` setelah alasan. Buat sakelar dari sesi interaktif, atau ubah keputusan hook untuk model ini.
* **`so organization-managed PreModelSwitch hooks could not be checked`**: Claude Code tidak dapat mengetahui hook PreModelSwitch mana yang [plugin terkelola](/docs/id/settings-reference#enabledplugins) organisasi Anda berikan, misalnya karena plugin terkelola gagal dimuat. Salah satu hook itu mungkin memblokir sakelar, jadi Claude Code menolak bukan menerapkan sakelar yang tidak diperiksa. Awal alasan menyebutkan apa yang gagal. Claude Code memeriksa ulang pada setiap upaya sakelar, jadi kegagalan yang sejak itu telah dihapus berhenti memblokir; jika terus gagal, jalankan `claude --debug` dan sakelar lagi untuk menangkap detail, kemudian perbaiki plugin atau minta admin Anda memperbaikinya.
* **`a PreModelSwitch hook failed before answering`** atau **`PreModelSwitch hooks were cancelled (the control stream closed) before answering`**: jalankan hook berakhir tanpa putusan, dan Claude Code tidak memperlakukan itu sebagai persetujuan. Jalankan `claude --debug` untuk melihat apa yang gagal, kemudian sakelar lagi.

Sebelum v2.1.260, penolakan plugin terkelola berbunyi `plugin hooks could not be loaded, so PreModelSwitch hooks could not be checked; see the debug log`. Claude Code mencoba ulang pemuatan plugin sekali kemudian menolak sakelar kemudian dalam sesi, bahkan ketika organisasi Anda tidak mengelola plugin apa pun. Mulai ulang sesi untuk menjalankan pemuatan plugin lagi pada versi tersebut.

<h3 id="couldnt-save-it-as-your-default">
  Tidak dapat menyimpannya sebagai default Anda
</h3>

Anda memilih model untuk disimpan sebagai default Anda, misalnya dengan `/model <name>` atau `Enter` di pemilih `/model`, dan Claude Code tidak dapat menulis pilihan ke file pengaturan pengguna Anda, `~/.claude/settings.json`. Sakelar itu sendiri diterapkan, jadi sesi saat ini berjalan pada model yang Anda pilih, tetapi default Anda tidak berubah dan sesi berikutnya dimulai pada nilai lama.

```text theme={null}
Set model to Fable 5.1 for this session only · couldn't save it as your default: ~/.claude/settings.json can't be written (EROFS)
```

Alasan setelah jalur file mengatakan apa yang gagal:

* **`can't be written (<code>)`**: penulisan gagal dengan kode kesalahan sistem operasi dalam tanda kurung, seperti `EROFS` ketika file, atau file yang ditautkannya, duduk di sistem file yang menolak penulisan. Buat file dapat ditulis dan sakelar lagi. Jika alat lain menghasilkan file, atur kunci `model` di alat itu sebagai gantinya; lihat [A change you made in Claude Code is lost in new sessions](/docs/id/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **`isn't valid JSON`**: file di disk tidak diurai, dan Claude Code meninggalkannya tidak tersentuh bukan menimpa konten yang tidak dapat dibaca kembali. Perbaiki kesalahan sintaks, kemudian sakelar lagi; lihat [Fix a broken settings file](/docs/id/settings#fix-a-broken-settings-file).

Pemberitahuan yang berakhir `couldn't confirm it was saved as your default (~/.claude/settings.json is still being written)` berarti penulisan belum selesai setelah tiga detik. Ini berlanjut di latar belakang, jadi default mungkin masih disimpan; periksa model mana yang dimulai sesi berikutnya Anda, atau jalankan `/model <name>` lagi.

Sebelum v2.1.265, pemberitahuan mengatakan model itu `saved as your default for new sessions` bahkan ketika penulisan gagal.

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled tidak didukung untuk model ini
</h3>

Versi Claude Code Anda lebih lama dari minimum untuk model yang dipilih. CLI mengirim konfigurasi pemikiran yang model tidak lagi terima.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Yang harus dilakukan:**

* Jalankan `claude update` dan mulai ulang Claude Code. Opus 4.7 memerlukan v2.1.111 atau lebih baru. Opus 4.8 memerlukan v2.1.154 atau lebih baru. Sonnet 5 memerlukan v2.1.197 atau lebih baru. Opus 5 memerlukan v2.1.219 atau lebih baru. Opus 5.5 memerlukan v2.1.280 atau lebih baru
* Jika Anda tidak dapat meningkatkan, jalankan `/model` dan pilih Opus 4.6 atau Sonnet 4.6 sebagai gantinya
* Jika Anda mencapai ini di [Agent SDK](/docs/id/agent-sdk/overview), tingkatkan paket SDK sebagai gantinya. Opus 4.8 memerlukan TypeScript SDK v0.3.154 atau lebih baru dan Python SDK v0.2.88 atau lebih baru. Sonnet 5 memerlukan TypeScript SDK v0.3.197 atau lebih baru. Opus 5 memerlukan TypeScript SDK v0.3.219 atau lebih baru. Opus 5.5 memerlukan TypeScript SDK v0.3.280 atau lebih baru

<h3 id="effort-isnt-available-with-thinking-turned-off">
  Effort tidak tersedia dengan pemikiran dimatikan
</h3>

Anda mematikan [extended thinking](/docs/id/model-config#extended-thinking) dan menjalankan pada [effort level](/docs/id/model-config#adjust-effort-level) di atas `high`. Model tidak menerima kombinasi itu, jadi API menolak permintaan.

```text theme={null}
API Error: Effort 'xhigh' isn't available with thinking turned off on this model · run /effort high to continue, or turn thinking back on (unset MAX_THINKING_TOKENS=0)
```

**Yang harus dilakukan:**

* [Turunkan effort level](/docs/id/model-config#set-the-effort-level) ke `high` atau di bawahnya.
* Nyalakan pemikiran kembali, misalnya dengan membatalkan [`MAX_THINKING_TOKENS`](/docs/id/env-vars) atau menghapus [`"alwaysThinkingEnabled": false`](/docs/id/settings-reference#alwaysthinkingenabled) dari pengaturan Anda.

Sebelum v2.1.242, Claude Code menampilkan pesan API sendiri: `API Error: 400 output_config.effort 'xhigh' is not supported when thinking is disabled on this model. Use effort 'high' or below, or enable thinking.` Sebelum v2.1.251, Claude Code mengirim permintaan pada effort level yang Anda atur, jadi Opus 5 menolak setiap permintaan di atas `high` dengan pemikiran dimatikan. Claude Code sekarang mengirim effort `high` sebagai gantinya ke model yang diketahuinya menolak kombinasi, seperti Opus 5, jadi pada v2.1.251 atau lebih baru kesalahan ini mencapai Anda hanya dari model yang Claude Code tidak tahu menolaknya.

<h3 id="thinking-budget-exceeds-output-limit">
  Anggaran pemikiran melebihi batas output
</h3>

Anggaran extended thinking yang dikonfigurasi melebihi panjang respons maksimum, jadi tidak ada ruang yang tersisa untuk jawaban sebenarnya.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code menyesuaikan nilai-nilai ini secara otomatis di API Anthropic. Anda biasanya melihat kesalahan ini di Amazon Bedrock atau Platform Agent Google Cloud ketika [`MAX_THINKING_TOKENS`](/docs/id/env-vars) diatur lebih tinggi dari batas output penyedia, atau ketika mode rencana menaikkan anggaran pemikiran.

**Yang harus dilakukan:**

* Turunkan `MAX_THINKING_TOKENS`, atau naikkan [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/id/env-vars) di atas anggaran pemikiran
* Lihat [Extended thinking](/docs/id/model-config#extended-thinking) untuk bagaimana anggaran berinteraksi dengan panjang output

<h3 id="tool-use-or-thinking-block-mismatch">
  Ketidakcocokan blok penggunaan alat atau pemikiran
</h3>

Riwayat percakapan mencapai API dalam keadaan tidak konsisten, biasanya setelah panggilan alat dihentikan atau giliran diedit di tengah aliran.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 orphaned tool_result in conversation history. Run /rewind to recover the conversation.
API Error: 400 duplicate tool_use ID in conversation history. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Semua varian berarti hal yang sama: urutan blok `tool_use`, `tool_result`, dan `thinking` dalam riwayat tidak lagi cocok dengan apa yang diharapkan API.

**Yang harus dilakukan:**

* Jika Anda menggunakan Opus 4.7 atau Opus 4.8, jalankan `claude update` terlebih dahulu. Versi sebelum v2.1.156 dapat memicu kesalahan ini selama penggunaan alat normal, dan `/rewind` tidak menghapusnya.
* Jalankan `/rewind`, atau tekan Esc dua kali, untuk mundur ke checkpoint sebelum giliran yang rusak dan lanjutkan dari sana. Lihat [Checkpointing](/docs/id/checkpointing) untuk bagaimana checkpoint dibuat dan dipulihkan.

<h3 id="unsupported-tool-content-removed">
  Konten alat yang tidak didukung dihapus
</h3>

Ketika Claude Code terhubung langsung ke API Anthropic dan memuat atau melihat pratinjau sesi yang disimpan, ia menghapus konten alat yang API Anthropic tidak terima dan meninggalkan baris ini di mana konten yang dihapus duduk di antara dua blok pemikiran:

```text theme={null}
[Unsupported tool content removed]
```

Konten seperti itu mencapai file sesi ketika sesuatu selain API Anthropic menjawab dalam format API, biasanya proxy pihak ketiga yang diatur melalui [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) yang menerjemahkan panggilan alat penyedia lain. Claude Code menghapusnya hanya ketika sesi terhubung langsung ke API Anthropic, dan memuat riwayat yang disimpan seperti adanya ketika sesi berjalan melalui proxy atau di penyedia lain. Sebelum v2.1.246, Claude Code mengirim penggunaan alat dan hasilnya kembali ke API, dan setiap giliran sesi yang dilanjutkan gagal dengan kesalahan 400 seperti `messages.1.content.0.server_tool_use.name: Input should be 'web_search', 'web_fetch', ...`.

**Yang harus dilakukan:**

* Tidak ada yang diperlukan ketika Anda melihat baris placeholder. Sesi berlanjut tanpa konten yang dihapus.
* Jika setiap giliran sesi yang dilanjutkan gagal dengan kesalahan 400 sebagai gantinya, jalankan `claude update` dan lanjutkan sesi lagi. Versi sebelum v2.1.246 tidak menghapus konten.

<h3 id="role-system-must-precede-an-assistant-message">
  role 'system' harus mendahului pesan 'assistant'
</h3>

API menolak permintaan dengan 400 karena pesan sistem duduk pada posisi dalam percakapan yang tidak diterimanya:

```text theme={null}
API Error: 400 messages.6: role 'system' must precede an 'assistant' message or end the array; ...
```

Claude Code mengirim beberapa teks pengingat dan lampiran sebagai pesan sistem di dalam percakapan. Ketika API menolak posisi satu, Claude Code mencoba ulang permintaan sekali dengan teks itu dikirim sebagai pesan pengguna biasa sebagai gantinya. Pesan penempatan saudara API, seperti `use the top-level 'system' parameter for the initial system prompt`, mendapatkan pemulihan yang sama.

Ketika kesalahan muncul, pesan sistem yang ditolak bukan yang dapat dihapus Claude Code. Itu biasanya berarti proxy atau [gateway LLM](/docs/id/llm-gateway) antara Claude Code dan API menambahkan pesan sistem mereka sendiri atau mengurutkan ulang percakapan.

**Yang harus dilakukan:**

* Jalankan `/clear` untuk memulai percakapan segar. Jika kesalahan kembali di sana juga, penyebabnya ada di jalur permintaan, bukan di percakapan yang disimpan.
* Jika kesalahan berulang pada setiap giliran di belakang proxy atau gateway yang dikonfigurasi melalui [`ANTHROPIC_BASE_URL`](/docs/id/env-vars), terhubung tanpa proxy untuk mengkonfirmasi sumber, dan laporkan kesalahan ke siapa pun yang mengoperasikannya

Sebelum v2.1.280, Claude Code tidak mengenali pesan ini, jadi kesalahan juga muncul ketika pesan sistem yang ditolak adalah yang dikirim Claude Code sendiri, dan setiap giliran percakapan kemudian gagal dengan cara yang sama.

<h3 id="invalid-encrypted-content-in-search-result-block">
  Encrypted\_content tidak valid dalam blok search\_result
</h3>

API menolak permintaan dengan 400 karena riwayat percakapan menyimpan konten pencarian web yang dihosting yang tidak dapat didekripsi. Pesan menyebutkan bidang yang tidak dapat dibacanya:

```text theme={null}
API Error: 400 messages.21.content.0: Invalid `encrypted_content` in `search_result` block
API Error: 400 messages.21.content.3.citations.0: Invalid `encrypted_index` in `text` block
API Error: 400 Failed to decrypt web search result content
```

Hasil dari [alat pencarian web](/docs/id/tools-reference#websearch-tool-behavior) yang dihosting API membawa bidang terenkripsi yang hanya dapat dibaca API. API menolak permintaan yang memutar ulang konten yang tidak dapat didekripsi, seperti konten yang diproduksi untuk organisasi berbeda.

Alat [WebSearch](/docs/id/tools-reference#websearch-tool-behavior) Claude Code sendiri mencatat hasil pencarian sebagai teks biasa, jadi blok ini biasanya mencapai percakapan melalui proxy atau [gateway LLM](/docs/id/llm-gateway) yang menjalankan pencarian web yang dihosting itu sendiri.

Blok yang ditolak tetap berada dalam riwayat percakapan, jadi setiap giliran kemudian dan `/compact` gagal dengan cara yang sama.

**Yang harus dilakukan:**

* Jalankan `/clear` atau mulai sesi baru; percakapan baru tidak membawa blok yang ditolak
* Jika Anda menjalankan Claude Code di belakang proxy atau gateway, laporkan kesalahan ke siapa pun yang mengoperasikannya

<h3 id="usage-policy-refusal">
  Penolakan Kebijakan Penggunaan
</h3>

API menolak untuk merespons karena konten dalam percakapan memicu pemeriksaan [Kebijakan Penggunaan](https://www.anthropic.com/legal/aup). Pesan menyertakan ID Permintaan yang dapat Anda kutip untuk mendukung jika Anda percaya penolakan tidak benar.

```text theme={null}
API Error: Opus 4.6 can't help with this. Start a new session to continue.

Send feedback with /feedback or learn more: https://www.anthropic.com/legal/aup
```

Pesan menyebutkan model yang menolak, atau `Claude` ketika tidak ada model yang dicatat.

Pemeriksaan mengevaluasi seluruh percakapan, bukan hanya prompt terbaru Anda, jadi mengirim pesan baru dalam sesi yang sama biasanya memicu ulang penolakan yang sama. Hal yang sama berlaku setelah keluar dan membuka kembali sesi dengan `--continue` atau `--resume`, karena transkrip di disk masih berisi konten pemicu. Di [Amazon Bedrock](/docs/id/amazon-bedrock), [Platform Agent Google Cloud](/docs/id/google-vertex-ai), dan [Microsoft Foundry](/docs/id/microsoft-foundry), pesan ini juga mencakup permintaan yang langkah-langkah keselamatan model tandai sebagai topik keamanan siber. Lihat [Safety measures flagged a cybersecurity topic](#safety-measures-flagged-a-cybersecurity-topic).

Sebelum v2.1.219, pesan berbunyi `Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.`

**Yang harus dilakukan:**

* Tekan Esc dua kali atau jalankan `/rewind` untuk mundur ke checkpoint sebelum giliran yang memicu penolakan, kemudian rephrase atau ambil pendekatan berbeda. Lihat [Checkpointing](/docs/id/checkpointing).
* Jika Anda tidak dapat mengidentifikasi giliran mana yang menyebabkannya, jalankan `/clear` untuk memulai percakapan segar dalam proyek yang sama. Percakapan sebelumnya Anda disimpan di disk dan tetap tersedia di `/resume`.
* Dalam [mode non-interaktif](/docs/id/headless) (`-p`), di mana rewind tidak tersedia, coba ulang dengan prompt yang diucapkan ulang dalam sesi baru tanpa `--continue`. Pemeriksaan kebijakan bervariasi menurut model, jadi beralih ke model berbeda dengan `--model` juga dapat menyelesaikan penolakan dalam beberapa kasus.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Langkah-langkah keselamatan menandai topik keamanan siber
</h3>

Langkah-langkah keselamatan model menandai konten dalam percakapan sebagai topik keamanan siber. Pesan menyebutkan model yang menandai permintaan:

```text theme={null}
API Error: Opus 4.8's safeguards flagged this message. Our intentionally broad safeguards allow us to deliver more capabilities faster, but can sometimes flag legitimate cybersecurity work. Apply to the Cyber Verification Program to reduce these interruptions. Send feedback with /feedback or learn more: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude
```

Pesan menautkan ke [Program Verifikasi Siber](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), yang memberikan akses untuk pekerjaan keamanan siber yang sah. Di Opus 5.5, yang memerlukan v2.1.280 atau lebih baru, pesan dibuka dengan `Opus 5.5's safeguards flagged this session` sebagai gantinya. Ketika kategori yang ditandai memiliki model fallback yang tersedia, Claude Code [beralih model](/docs/id/model-config#automatic-model-fallback) bukan menampilkan kesalahan ini.

Di [Amazon Bedrock](/docs/id/amazon-bedrock), [Platform Agent Google Cloud](/docs/id/google-vertex-ai), dan [Microsoft Foundry](/docs/id/microsoft-foundry), bendera keamanan siber menghasilkan pesan [Penolakan Kebijakan Penggunaan](#usage-policy-refusal) sebagai gantinya.

Penjaga itu sendiri adalah server-side dan mendahului v2.1.203; rilis klien sejak itu hanya mengubah pesan wording.
Dari v2.1.203 melalui v2.1.218, pesan berbunyi `<model> has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center:` diikuti oleh tautan pusat bantuan yang sama, dan sesi interaktif ditambahkan `If you were not engaging in a cybersecurity topic, please send feedback via /feedback.`
Sebelum v2.1.203, itu berbunyi `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` diikuti oleh tautan formulir pengecualian.

**Yang harus dilakukan:**

* Jika pekerjaan Anda memerlukan konten ini, ajukan permohonan akses melalui [Program Verifikasi Siber](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Jika permintaan Anda bukan tentang topik keamanan siber, jalankan `/feedback` untuk melaporkan positif palsu
* Untuk terus bekerja dalam sesi yang sama, tekan Esc dua kali atau jalankan `/rewind` untuk mundur ke checkpoint sebelum giliran yang memicu bendera, kemudian ambil pendekatan berbeda. Lihat [Checkpointing](/docs/id/checkpointing).

<h2 id="installation-errors">
  Kesalahan instalasi
</h2>

Kesalahan ini muncul saat menginstal atau memperbarui Claude Code, dari [skrip instalasi](/docs/id/setup#install-claude-code), `claude install`, atau `claude update`. Untuk masalah `command not found`, PATH, izin, dan TLS selama pengaturan, lihat [Troubleshoot installation and login](/docs/id/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  Installation was killed before it could finish
</h3>

Skrip instalasi melaporkan ketika langkah `claude install` dihentikan oleh sinyal. Di Linux, kode keluar 137 berarti proses menerima SIGKILL, dan pada host dengan memori rendah itu biasanya pembunuh out-of-memory (OOM) kernel. Skrip mencetak penjelasan ini dan keluar dengan kode 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Untuk sinyal fatal lainnya, dan untuk kode keluar 137 di macOS, skrip mencetak `Installation was killed before it could finish (exit code <N>)` dengan kode keluar aktual dan menghilangkan penjelasan out-of-memory. Pesan berasal dari skrip instalasi yang digunakan macOS dan Linux, yang juga mencakup instalasi di dalam WSL; skrip instalasi Windows asli tidak pernah mencetaknya. Sebelum v2.1.200, skrip keluar hanya dengan baris `Killed` shell yang kosong.

**Yang harus dilakukan:**

* Hentikan proses lain untuk membebaskan memori, kemudian jalankan kembali penginstal
* Tambahkan ruang swap atau pindah ke instans yang lebih besar. Lihat [Install killed on low-memory Linux servers](/docs/id/troubleshoot-install#install-killed-on-low-memory-linux-servers) untuk perintah file swap.

<h3 id="the-connection-dropped-while-downloading-the-update">
  The connection dropped while downloading the update
</h3>

Koneksi ke server unduhan ditutup saat `claude install`, `claude update`, atau [automatic updater](/docs/id/setup#auto-updates) mengambil biner Claude Code, dan pengulangan tidak berhasil. Claude Code mencoba ulang unduhan ketika koneksi putus, transfer macet, atau file yang diunduh gagal checksumnya, hingga tiga percobaan total. Kesalahan HTTP yang selesai, seperti 404, tidak dicoba ulang karena server sudah menjawab. Sebelum v2.1.202, koneksi yang putus tunggal gagal mengunduh segera dengan kesalahan kosong `aborted` alih-alih mencoba ulang.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

Teks dalam tanda kurung menyebutkan percobaan mana yang gagal dan kesalahan jaringan yang mendasarinya. `claude update` mendahului pesan dengan `Error: Failed to install native update` di stderr.

Unduhan yang tetap terhubung tetapi tidak selesai dalam 10 menit gagal dengan `Download timed out: exceeded the total deadline` sebagai gantinya. Claude Code tidak mencoba ulang unduhan yang habis waktu, karena koneksi yang terlalu lambat untuk selesai dalam batas waktu tidak akan selesai pada pengulangan segera. Langkah-langkah di bawah berlaku untuk kedua pesan.

Penyebab umum adalah proxy atau gateway yang menutup transfer panjang sebelum selesai. Biner Claude Code adalah unduhan besar, jadi batas koneksi proxy yang tidak pernah mempengaruhi lalu lintas API normal masih dapat mengganggu.

**Yang harus dilakukan:**

* Jalankan `claude update` lagi. Pada jaringan yang sehat, unduhan biasanya berhasil pada run berikutnya. Untuk pesan yang habis waktu, jalankan lagi dari jaringan yang lebih cepat atau kurang dibatasi.
* Jika jaringan Anda memerlukan proxy, atur `HTTPS_PROXY` sebelum menjalankan penginstal atau `claude update`. Lihat [Check network connectivity](/docs/id/troubleshoot-install#check-network-connectivity).
* Jika proxy korporat terus menutup transfer, minta tim jaringan Anda untuk mengizinkan unduhan lengkap dari `downloads.claude.ai`. Lihat [Network access requirements](/docs/id/network-config#network-access-requirements).
* Jalankan `claude doctor` dari shell Anda untuk diagnostik instalasi

<h2 id="command-line-errors">
  Kesalahan baris perintah
</h2>

Kesalahan ini berasal dari perintah `claude` dan subperintahnya, dari nama perintah yang Anda kirimkan di prompt, dan dari perintah seperti `/security-review` yang mengumpulkan konteks dengan menjalankan perintah shell sebelum prompt mereka berjalan. Mereka juga berasal dari `/tui`, yang meluncurkan kembali CLI.

<h3 id="conflict-between-bg-and-print">
  Konflik antara --bg dan --print
</h3>

Pesan ini memerlukan Claude Code v2.1.198 atau lebih baru. Anda menggabungkan `--bg` dengan `-p` atau `--print` dalam invokasi `claude` yang sama. `--bg` memulai [sesi latar belakang](/docs/id/agent-view#from-your-shell) yang kemudian Anda lampirkan dengan `claude agents`, sementara `--print` berjalan [non-interaktif](/docs/id/headless) dan tidak pernah memulai sesi interaktif yang `claude agents` lampirkan. Sebelum v2.1.198, kombinasi ini secara diam-diam membuat pekerjaan latar belakang yang tidak pernah bisa dilampirkan.

```text theme={null}
--bg dan --print berkonflik: --print tidak pernah memulai sesi interaktif yang `claude agents` lampirkan, jadi pekerjaan tidak akan dapat dilampirkan. Prompt adalah posisional — lepaskan --print: `claude --bg '<task>'`.
```

**Yang harus dilakukan:**

* Lepaskan `-p` atau `--print`. `--bg` mengambil prompt sebagai argumen posisionalnya, jadi `claude --bg "<task>"` adalah perintah lengkapnya. Lihat [Dispatch new agents from your shell](/docs/id/agent-view#from-your-shell).
* Untuk menjalankan prompt secara non-interaktif dan mencetak hasilnya alih-alih membuat sesi latar belakang, lepaskan `--bg` dan jalankan `claude -p "<task>"`

<h3 id="invalid-agents-configuration">
  Konfigurasi --agents tidak valid
</h3>

Nilai yang Anda berikan ke `--agents` tidak valid, jadi `claude` keluar dengan kode 1 alih-alih memulai sesi. Ketika Anda melewatkan `--safe-mode`, `--resume`, atau `--continue`, atau mengatur [`CLAUDE_CODE_SAFE_MODE`](/docs/id/env-vars#variables), Claude Code tidak memeriksa nilainya dan memulai sesi. Sebelum v2.1.242, Claude Code memulai sesi bagaimanapun dan menghilangkan definisi yang tidak bisa dimuat.

```text theme={null}
Error: Invalid --agents configuration:
<what failed>
```

Apa yang mengikuti baris pertama tergantung pada bagaimana nilainya gagal. Claude Code menjalankan pemeriksaan ini secara berurutan dan berhenti di yang pertama gagal. Jika nilai Anda memiliki dua jenis masalah, Anda hanya melihat yang kedua setelah Anda memperbaiki yang pertama:

1. Ketika nilai tidak diuraikan sebagai JSON, Claude Code mencetak satu baris `invalid JSON:` yang membawa pesan parser JSON sendiri
2. Ketika diuraikan tetapi definisi agen tidak cocok dengan skema untuk [subagen yang ditentukan CLI](/docs/id/sub-agents#choose-the-subagent-scope), Claude Code mencetak satu baris per masalah
3. Ketika nama agen dimulai dengan `-`, Claude Code mencetak `<name>: agent names must not start with '-'`

Ketika ada lebih dari 20 baris masalah, Claude Code mencetak 20 yang pertama dan mengganti sisanya dengan `…and N more`.

**Yang harus dilakukan:**

* Perbaiki setiap masalah yang daftar pesan, kemudian jalankan perintah lagi. Lihat [bidang yang diambil subagen yang ditentukan CLI](/docs/id/sub-agents#choose-the-subagent-scope).

<h3 id="cloud-sessions-cannot-be-created-from-a-restricted-session">
  Sesi cloud tidak dapat dibuat dari sesi --restricted
</h3>

Ketika Anda memulai sesi dengan [`--restricted`](/docs/id/cli-reference#cli-flags), Claude Code menolak untuk membuat [sesi cloud](/docs/id/claude-code-on-the-web#from-terminal-to-cloud) darinya, karena sesi baru akan berjalan di luar proses terbatas dan tidak akan memberlakukan mode terbatas. Claude Code menolak di klien, sebelum menghubungi server, jadi tidak ada sesi cloud yang dibuat:

```text theme={null}
Cloud sessions cannot be created from a --restricted session: they would not enforce it.
```

**Yang harus dilakukan:**

* Jalankan tugas secara lokal dalam sesi terbatas
* Jika Anda mengontrol bagaimana sesi diluncurkan, mulai sesi `claude` baru tanpa `--restricted` dan buat sesi cloud dari sana

Sebelum v2.1.248, Claude Code tidak memiliki flag `--restricted`; versi sebelumnya menolak flag itu sendiri dengan kesalahan opsi yang tidak dikenal.

<h3 id="cloud-sessions-are-disabled-by-your-organizations-policy">
  Sesi cloud dinonaktifkan oleh kebijakan organisasi Anda
</h3>

Kebijakan `allow_remote_sessions` organisasi Anda mati, jadi [sesi cloud](/docs/id/claude-code-on-the-web) dan perintah yang menggunakannya tidak tersedia:

```text theme={null}
Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.
```

Pesan muncul ketika Anda [membuat sesi cloud dari terminal](/docs/id/claude-code-on-the-web#from-terminal-to-cloud) dan ketika Anda mengirimkan perintah yang memerlukan sesi cloud, seperti `/teleport`, `/remote-env`, atau `/web-setup`. Sebelum v2.1.268, mengirimkan salah satu perintah itu mengembalikan [`Unknown command`](#unknown-command) sebagai gantinya.

Ini adalah kebijakan organisasi sisi server, jadi tidak dapat ditimpa dari pengaturan lokal, variabel lingkungan, atau flag CLI.

Jika Claude Code belum memuat kebijakan organisasi Anda atau tidak dapat mengambilnya, perintah itu menjawab `Couldn't verify your organization's policy for cloud sessions. Check your network connection, then restart Claude Code and try again.` sebagai gantinya.

**Yang harus dilakukan:**

* Minta [Owner](/docs/id/server-managed-settings#access-control) di organisasi Anda untuk mengaktifkan sesi cloud di pengaturan admin Claude Code di [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Jika pesan mengatakan tidak dapat memverifikasi kebijakan, periksa koneksi jaringan Anda, kemudian mulai ulang Claude Code dan coba lagi

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  Nilai --json-schema bukan JSON Schema yang valid
</h3>

Skema yang Anda berikan ke [`--json-schema`](/docs/id/cli-reference#cli-flags) dalam [mode non-interaktif](/docs/id/headless#get-structured-output) gagal kompilasi JSON Schema, jadi `claude` keluar dengan kode 1 alih-alih menjalankan prompt. Sebelum v2.1.205, skema yang tidak valid menghasilkan output tidak terstruktur tanpa kesalahan, dan skema apa pun yang menggunakan kata kunci `format` diperlakukan sebagai tidak valid.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

Teks setelah titik dua kedua adalah diagnostik validator dan menyebutkan kata kunci atau lokasi yang gagal. Skema yang menggunakan kata kunci `format`, seperti `"format": "email"`, valid: Claude Code menerima `format` sebagai anotasi dan tidak memberlakukannya.

Claude Code menjalankan dua pemeriksaan sebelum kompilasi skema: menolak nilai yang tidak dapat diuraikan JSON dengan `Error: --json-schema is not valid JSON`, dan JSON yang valid tetapi bukan objek dengan `Error: --json-schema must be a JSON object`.

**Yang harus dilakukan:**

* Perbaiki bagian skema yang diagnostik sebutkan, kemudian jalankan kembali perintah
* Jika diagnostik adalah `schema too large`, kurangi nesting skema dan penggunaan kembali `$ref`
* Lihat [Get structured output](/docs/id/headless#get-structured-output) untuk skema kerja dan perintah

<h3 id="settings-file-exceeds-the-2mib-limit">
  File pengaturan melebihi batas 2MiB
</h3>

File yang Anda berikan ke [`--settings`](/docs/id/cli-reference#cli-flags) lebih besar dari 2 MiB, jadi `claude` keluar dengan kode 1 saat startup alih-alih memuatnya. File pengaturan adalah dokumen JSON kecil, jadi file sebesar ini biasanya berarti jalur menunjuk ke file yang salah. Sebelum v2.1.214, Claude Code membaca file tanpa pemeriksaan ukuran, dan file multi-gigabyte atau file perangkat seperti `/dev/zero` menumbuhkan memori tanpa batas.

```text theme={null}
Error: Settings file exceeds the 2MiB limit: /path/to/settings.json
```

Claude Code menolak jalur `--settings` yang bukan file biasa dengan cara yang sama: perangkat, FIFO, atau soket melaporkan `Error: Cannot use settings file (Not a regular file (device, FIFO, or socket))` diikuti oleh jalur, dan direktori melaporkan alasan `EISDIR`.

**Yang harus dilakukan:**

* Arahkan `--settings` ke file pengaturan JSON biasa di bawah 2 MiB. Lihat [Settings](/docs/id/settings) untuk formatnya.

<h3 id="the-current-directory-no-longer-exists">
  Direktori saat ini tidak lagi ada
</h3>

Anda memulai `claude` dari direktori yang dihapus atau dipindahkan setelah shell Anda memasukkannya, misalnya worktree atau direktori temp yang shell lain hapus. Claude Code tidak dapat membaca direktori kerjanya, jadi keluar dengan kode 1 sebelum memulai sesi, dalam mode interaktif dan [non-interaktif](/docs/id/headless) sama-sama. Sebelum v2.1.239, Claude Code mogok dengan sumber bundle minified dan stack `ENOENT ... uv_cwd` mentah di stderr alih-alih pesan ini.

```text theme={null}
The current directory no longer exists (it was deleted or moved). Start Claude Code from an existing directory.
error: The current working directory was deleted, so that command didn't work. Please cd into a different directory and try again.
```

Penyebab dan perbaikannya sama untuk kedua bentuk.

Ketika Claude Code tidak dapat membaca direktori kerja karena alasan lain, seperti perubahan izin, pesan menyebutkan kode kesalahan sebagai gantinya: `Can't read the current directory (EACCES). Start Claude Code from a different directory.`

Di macOS, `EPERM` untuk direktori di `~/Desktop`, `~/Documents`, `~/Downloads`, atau iCloud Drive biasanya berarti macOS memblokir aplikasi terminal Anda dari folder itu. Perintah lain yang membaca folder itu gagal dengan cara yang sama: `ls` di sana melaporkan `Operation not permitted`, bahkan dengan `sudo`.

**Yang harus dilakukan:**

* Ubah ke direktori yang ada, seperti direktori home atau proyek Anda, kemudian jalankan `claude` lagi
* Jika direktori dibuat kembali di jalur yang sama, shell Anda masih memegang yang dihapus. Jalankan `cd "$PWD"` atau tinggalkan dan masuki kembali direktori, kemudian jalankan `claude` lagi
* Untuk `EPERM` di macOS, keluar dari aplikasi terminal Anda dengan Cmd+Q, buka lagi, kembali ke folder itu, dan jalankan `claude`. Jika `ls` di folder itu masih gagal, buka **System Settings > Privacy & Security > Files and Folders**, aktifkan folder untuk aplikasi terminal Anda, kemudian buka kembali terminal

<h3 id="temp-directory-refused-or-cannot-be-created">
  Direktori temp ditolak atau tidak dapat dibuat
</h3>

Di macOS dan Linux, Claude Code membuat direktori temp pribadi saat startup, `claude-<uid>` di bawah direktori temp sistem atau override [`CLAUDE_CODE_TMPDIR`](/docs/id/env-vars). Ketika direktori tidak dapat dibuat, atau entri yang sudah ada di jalur itu gagal pemeriksaan keamanan, Claude Code mencetak kegagalan ke stderr dan keluar dengan kode 1 daripada memulai sesi:

```text wrap theme={null}
ENOSPC: no space left on device, mkdir '/tmp/claude-501'

Temp directory /tmp/claude-501 is not a directory (may be an attacker-planted symlink). Refusing to use it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is owned by uid 502, expected 501. Refusing to use it — another user may have pre-created it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is not readable (its mode may have been altered, or a path component denies search). Refusing to use it — restore its permissions (chmod 0700) or remove it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.
```

**Yang harus dilakukan:**

* Untuk `ENOSPC`, bebaskan ruang disk di volume yang menyimpan direktori temp
* Untuk bentuk `Refusing to use it`, hapus entri yang disebutkan itu sendiri, bukan apa yang ditunjuk link, dan mulai Claude Code lagi; untuk bentuk `owned by uid`, hanya administrator atau pengguna itu yang dapat menghapusnya
* Untuk `is not readable`, jalankan `chmod 0700` di direktori yang disebutkan, atau hapus dan mulai lagi
* Dalam kasus apa pun, atur [`CLAUDE_CODE_TMPDIR`](/docs/id/env-vars) ke direktori yang Anda kontrol dan mulai Claude Code lagi, meninggalkan jalur yang ditolak sendirian

<h3 id="directory-couldnt-be-resolved-to-a-real-location">
  Direktori tidak dapat diselesaikan ke lokasi nyata
</h3>

Anda menjalankan `/add-dir` untuk subdirektori direktori kerja Anda, dan Claude Code tidak dapat menyelesaikan direktori ke lokasi nyatanya.

Anda sudah memiliki akses file ke subdirektori direktori kerja, jadi `/add-dir` hanya memuat skills, commands, dan agents-nya. Sebelum memuatnya, Claude Code memeriksa bahwa lokasi nyata direktori, dengan symlink apa pun yang diselesaikan, berada di dalam direktori kerja. Ketika Claude Code tidak dapat menyelesaikan lokasi itu, tidak memuat apa pun dan menampilkan pesan ini:

```text theme={null}
packages/app couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded. Check that it is a directory inside the working directory and try again.
```

**Yang harus dilakukan:**

* Periksa bahwa jalur menyebutkan direktori nyata di dalam direktori kerja, kemudian jalankan `/add-dir` lagi
* Pesan tidak mengubah akses file Anda; hanya melaporkan bahwa konten `.claude/` direktori tidak dimuat

Sebelum v2.1.261, pesan ini juga muncul untuk setiap `/add-dir <subdirectory>` ketika direktori kerja berada di automount `/net/<host>`, di mana Claude Code menolak untuk menyelesaikan jalur dengan desain; direktori baik-baik saja dan mencoba kembali tidak bisa membantu.

<h3 id="workspace-not-trusted-when-starting-remote-control">
  Workspace tidak dipercaya saat memulai Remote Control
</h3>

Anda memulai mode server [Remote Control](/docs/id/remote-control) dengan `claude remote-control` atau alias `claude rc`-nya di direktori yang belum Anda percayai. Perintah tidak menampilkan dialog kepercayaan workspace itu sendiri, jadi keluar dengan kode 1 dan menyebutkan perbaikannya:

```text theme={null}
Error: Workspace not trusted. Please run `claude` in /Users/you/project first to review and accept the workspace trust dialog.
```

Di direktori home Anda pesan berbeda, karena dialog kepercayaan workspace tidak pernah menyimpan kepercayaan untuk direktori home, jadi menerimanya di sana tidak dapat memenuhi pemeriksaan ini. Sebelum v2.1.214, direktori home menampilkan pesan di atas, yang sarannya tidak dapat berhasil di sana.

```text theme={null}
Error: Workspace not trusted. /Users/you is your home directory, and for security home-directory trust is never saved, so running `claude` here first won't help. Run `claude rc` from a project directory instead (run `claude` there once to accept the trust dialog).
```

**Yang harus dilakukan:**

* Jalankan `claude` di direktori, terima [dialog kepercayaan workspace](/docs/id/permissions#project-allow-rules-and-workspace-trust), kemudian jalankan `claude remote-control` lagi
* Di direktori home Anda, ubah ke direktori proyek dan mulai Remote Control di sana

<h3 id="not-carried-over-to-the-sessions-remote-control-starts">
  Tidak dibawa ke sesi yang Remote Control mulai
</h3>

Anda memulai [Remote Control](/docs/id/remote-control) dengan flag `claude` global sebelum verba `remote-control`, yang akan membatasi atau mengonfigurasi sesi yang Remote Control mulai, seperti `--settings`, `--setting-sources`, `--permission-mode`, `--disallowed-tools`, atau `--mcp-config`. Flag yang ditempatkan sebelum verba tidak pernah mencapai sesi itu. Claude Code menolak untuk memulai sebagai gantinya, menyebutkan flag:

```text theme={null}
Error: `--settings` before `remote-control` is not carried over to the sessions Remote Control starts, so Remote Control refuses to start rather than drop it — remove it, and give Remote Control's own options after the verb (see `claude remote-control --help`).
```

Claude Code tidak menolak flag global yang aman untuk dijatuhkan, seperti `--verbose`, `--model`, atau `--session-id` yang disuntikkan wrapper atau `--plugin-dir`: mengabaikannya dan Remote Control dimulai.

Claude Code juga menolak untuk memulai untuk flag global yang belum dikenalinya sebagai aman, jadi flag yang ditambahkan dalam rilis yang lebih baru dapat muncul dalam pesan ini sampai rilis yang lebih baru menandainya aman.

**Yang harus dilakukan:**

* Hapus flag dari sebelum verba dan berikan [opsi Remote Control sendiri](/docs/id/remote-control#start-a-remote-control-session) setelahnya; `claude remote-control --help` mencantumnya
* Ketika flag yang ditolak adalah `--permission-mode`, jalankan `claude remote-control --permission-mode <mode>` untuk mengatur mode izin untuk sesi yang Remote Control mulai

Sebelum v2.1.248, `claude remote-control` tidak menerima flag-nya sendiri ketika flag global datang terlebih dahulu, dan perintah gagal dengan kesalahan `unknown option`.

<h3 id="claude-import-is-not-yet-available-in-this-build">
  claude import belum tersedia dalam build ini
</h3>

Anda menjalankan [`claude import`](/docs/id/cli-reference#cli-commands), dan Claude Code menemukan alur impor dimatikan, jadi perintah keluar dengan kode 1 alih-alih memulai impor. Sebelum v2.1.222, build dengan alur impor mati memperlakukan `import` sebagai prompt dan memulai sesi interaktif alih-alih mencetak pesan ini.

```text theme={null}
`claude import` is not yet available in this build. Run `claude` and use /mcp or edit ~/.claude/settings.json directly.
```

Claude Code mengaktifkan `claude import` melalui flag fitur yang diambilnya dari Anthropic dan disimpan di disk. Pesan ini berarti nilai yang disimpan mati. Penyebabnya biasanya salah satu dari berikut:

* Anda belum memulai sesi sejak instalasi, jadi Claude Code belum mengambil flag. `claude import` pertama dapat mencetak ini bahkan ketika fitur tersedia untuk Anda.
* Anda menggunakan Claude Code melalui Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, atau Claude Platform di AWS, atau melalui [gateway aplikasi Claude](/docs/id/claude-apps-gateway#availability-and-limitations). Claude Code tidak mengambil flag fitur dalam sesi ini, jadi `claude import` tetap tidak tersedia.
* Anda mengatur `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `DISABLE_GROWTHBOOK`, atau [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/id/env-vars), yang mematikan pengambilan flag fitur, jadi `claude import` tetap tidak tersedia.

**Yang harus dilakukan:**

* Pada instalasi segar, mulai `claude`, tunggu sesi dimuat, keluar, dan jalankan `claude import` lagi
* Di mana pengambilan flag fitur tetap mati, atur konfigurasi sendiri: tambahkan server MCP dengan [`claude mcp add`](/docs/id/mcp#installing-mcp-servers), dan buat [`CLAUDE.md` files](/docs/id/memory#how-claude-md-files-load), [skills dan commands](/docs/id/skills#where-skills-live), dan [subagents](/docs/id/sub-agents#choose-the-subagent-scope) yang ingin Anda bawa. Pesan juga menyebutkan `~/.claude/settings.json`. Dari konfigurasi yang `claude import` bawa, file itu hanya menyimpan [mode izin](/docs/id/settings-reference#permission-settings); Claude Code tidak membaca server MCP darinya.

<h3 id="could-not-read-claude-code-config">
  Tidak dapat membaca konfigurasi Claude Code
</h3>

Anda menjalankan [`claude import`](/docs/id/cli-reference#cli-commands) sementara Claude Code tidak dapat menguraikan `~/.claude.json`, file tempat menyimpan login dan status per-proyek Anda. Subperintah membaca file itu untuk memeriksa ketersediaan tetapi tidak menampilkan dialog pemulihan yang ditampilkan sesi interaktif, jadi keluar dengan kode 1. Sebelum v2.1.222, `claude import` dengan file konfigurasi yang tidak dapat dibaca memulai sesi interaktif, yang dialog pemulihannya menangani file.

```text theme={null}
Could not read Claude Code config — run `claude` with no arguments to recover it.
```

**Yang harus dilakukan:**

* Jalankan `claude` tanpa argumen. Claude Code mendeteksi file yang tidak valid dan menawarkan untuk mengatur ulangnya. Kemudian jalankan `claude import` lagi.
* Untuk menyimpan edit manual yang Anda buat, perbaiki sintaks JSON di `~/.claude.json` di editor sebagai gantinya, kemudian jalankan kembali `claude import`

<h3 id="could-not-import-a-server-from-claude-desktop">
  Tidak dapat mengimpor server dari Claude Desktop
</h3>

Claude Code tidak dapat menambahkan salah satu server yang Anda pilih di `claude mcp add-from-claude-desktop`. Perintah masih mengimpor server yang dipilih lainnya dan mencetak satu baris per server yang tidak dapat ditambahkan. Sebelum v2.1.205, server pertama yang gagal menghentikan impor dan tidak ada server yang dipilih ditambahkan.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

Teks setelah nama server adalah alasannya. Yang paling umum adalah pemeriksaan nama: Claude Desktop memungkinkan karakter dalam nama server, seperti spasi dan titik, yang `claude mcp` batasi ke huruf, angka, tanda hubung, dan garis bawah. Alasan lain termasuk konfigurasi server yang gagal validasi dan server yang diblokir oleh [kebijakan MCP](/docs/id/managed-mcp) organisasi Anda.

**Yang harus dilakukan:**

* Ubah nama server di `claude_desktop_config.json` untuk hanya menggunakan huruf, angka, tanda hubung, dan garis bawah, kemudian jalankan `claude mcp add-from-claude-desktop` lagi
* Tambahkan server itu langsung dengan `claude mcp add` atau `claude mcp add-json` di bawah nama yang valid. Lihat [Import MCP servers from Claude Desktop](/docs/id/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="cannot-add-mcp-server-to-the-managed-scope">
  Tidak dapat menambahkan server MCP ke cakupan yang dikelola
</h3>

Anda menjalankan `claude mcp add` atau `claude mcp add-json` dengan `--scope managed`. Cakupan itu menyimpan server yang organisasi Anda sediakan melalui pengaturan yang dikelola [`managedMcpServers`](/docs/id/settings-reference#managedmcpservers). Claude Code membacanya dari pengaturan yang dikelola saja, jadi perintah tidak dapat menulis server ke cakupan itu.

```text theme={null}
Cannot add MCP server to scope: managed
```

**Yang harus dilakukan:**

* Tambahkan server ke cakupan yang dapat Anda tulis: `local`, `user`, atau `project`. Tanpa `--scope`, perintah menggunakan `local`. Lihat [MCP installation scopes](/docs/id/mcp#mcp-installation-scopes)
* Untuk menyediakan server kepada setiap pengguna di organisasi Anda, tambahkan ke [`managedMcpServers`](/docs/id/settings-reference#managedmcpservers) dalam pengaturan yang dikelola yang Anda sebarkan

<h3 id="cant-read-mcp-json">
  Tidak dapat membaca .mcp.json
</h3>

Perintah yang membaca [`.mcp.json`](/docs/id/mcp#project-scope) proyek, seperti `claude mcp add` atau `claude mcp add-json` dengan `--scope project`, atau `claude mcp remove`, menemukan bahwa file di direktori saat ini bukan file biasa atau lebih besar dari 2 MiB, jadi keluar dengan kesalahan ini alih-alih membaca file.

```text theme={null}
Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes. Fix or remove it, then run the command again.
```

Sebelum v2.1.257, FIFO di `.mcp.json` membiarkan perintah menunggu selamanya tanpa output, dan symlink ke file perangkat seperti `/dev/zero` menumbuhkan memori sampai proses dibunuh.

**Yang harus dilakukan:**

* Periksa apa yang duduk di `.mcp.json` di direktori saat ini. Ganti dengan file JSON biasa dalam [format cakupan proyek](/docs/id/mcp#project-scope), atau hapus, kemudian jalankan perintah lagi.

<h3 id="anthropic-hosted-and-doesnt-support-local-oauth">
  Server adalah Anthropic-hosted dan tidak mendukung OAuth lokal
</h3>

Anda memulai sign-in untuk server MCP yang URL-nya menunjuk ke host konektor yang dihosting Anthropic yang mengautentikasi melalui penyedia identitas pihak ketiga. Host ini termasuk `microsoft365.mcp.claude.com`, `gmail.mcp.claude.com`, dan `gcal.mcp.claude.com`. Claude Code menolak untuk memulai alur OAuth lokal untuk host ini dari panel `/mcp` dan `claude mcp login`, karena [sign-in mereka hanya berfungsi melalui claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai).

```text theme={null}
"gmail" is Anthropic-hosted and doesn't support local OAuth. Connect it via Settings → Connectors on claude.ai (requires `claude login`), then it'll be available here automatically.
```

Claude Code mencocokkan host ini berdasarkan URL, jadi pesan muncul ketika server yang Anda tambahkan dengan `claude mcp add` atau di `.mcp.json` menunjuk ke salah satunya.

**Yang harus dilakukan:**

* Hapus entri Anda dengan `claude mcp remove <name>`, sehingga tidak dapat menyembunyikan konektor claude.ai di URL yang sama
* Setelah menghapusnya, hubungkan layanan di [claude.ai/customize/connectors](https://claude.ai/customize/connectors), sambil masuk ke akun yang Anda gunakan di Claude Code. Setelah terhubung, [konektor muncul di Claude Code secara otomatis](/docs/id/mcp#use-mcp-servers-from-claude-ai) jika metode autentikasi aktif Anda adalah login langganan claude.ai

<h3 id="server-rejected-the-authorization-header-minted-by-the-configured-headershelper">
  Server menolak header Authorization yang dicetak oleh headersHelper yang dikonfigurasi
</h3>

Server MCP yang [`headersHelper`](/docs/id/mcp#use-dynamic-headers-for-custom-authentication)-nya memasok header `Authorization` menjawab koneksi dengan HTTP 401 atau 403, jadi Claude Code melaporkan koneksi sebagai gagal. Karena helper memasok header `Authorization`, Claude Code [tidak kembali ke OAuth](/docs/id/mcp#authenticate-with-remote-mcp-servers) untuk server:

```text theme={null}
Server rejected the Authorization header minted by the configured headersHelper (HTTP 401). Check that the helper command returns a valid credential for this MCP endpoint — OAuth fallback is disabled when the helper supplies Authorization.
```

Claude Code menjalankan kembali helper pada setiap upaya koneksi, jadi retry setelah penolakan sementara, seperti perlombaan rotasi token, dapat berhasil dengan kredensial segar.

**Yang harus dilakukan:**

* Jalankan perintah `headersHelper` sendiri dengan cara Claude Code menjalankannya: dari [direktori Claude Code menjalankannya](/docs/id/mcp#where-the-helper-runs), dengan [variabel lingkungan Claude Code mengaturnya](/docs/id/mcp#use-dynamic-headers-for-custom-authentication), dan tanpa [variabel kredensial Claude Code menghapusnya](/docs/id/mcp#which-variables-a-helper-can-read) untuk server dari `.mcp.json` proyek, plugin, atau file agen proyek. Periksa bahwa itu mencetak nilai `Authorization` yang titik akhir server terima
* Setelah memperbaiki helper atau sumber kredensialnya, pilih server di `/mcp` dan pilih **Reconnect**

Sebelum v2.1.248, Claude Code menjalankan penemuan OAuth untuk server yang helper-nya memasok header `Authorization`. Penemuan itu dapat gagal dengan `Incompatible auth server: does not support dynamic client registration` alih-alih melaporkan kredensial yang ditolak.

<h3 id="mcp-permission-prompt-tool-not-found">
  Alat prompt izin MCP tidak ditemukan
</h3>

Alat yang Anda berikan ke [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags) tidak ada di antara alat MCP yang terhubung ketika run pertama kali memerlukan keputusan izin, baik karena server-nya tidak pernah terhubung atau karena tidak ada server yang terhubung yang mengekspos alat dengan nama itu. Claude Code masih mengirimkan prompt Anda: run [non-interaktif](/docs/id/headless) keluar dengan kesalahan ini, dan kode keluar 1, pada panggilan alat pertama yang memerlukan persetujuan, jadi tidak menghasilkan jawaban meskipun permintaan dibuat. Sebelum prompt pertama, Claude Code menunggu hingga timeout koneksi per-server 30 detik yang ditetapkan oleh [`MCP_TIMEOUT`](/docs/id/env-vars) untuk server itu terhubung. Sebelum v2.1.206, startup tidak menunggu server selesai terhubung, jadi server yang lambat dimulai tetapi sehat menghasilkan kesalahan ini juga.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

Daftar setelah `Available MCP tools:` menyebutkan alat MCP yang terhubung ketika tunggu berakhir.

**Yang harus dilakukan:**

* Periksa bahwa server dimulai dan tetap terhubung: jalankan `claude mcp list` di direktori yang sama dan konfirmkan server terdaftar sebagai terhubung
* Konfirmkan nama alat cocok dengan nama `mcp__<server>__<tool>` yang server ekspos
* Jika server memerlukan lebih lama dari 30 detik untuk dimulai, naikkan [`MCP_TIMEOUT`](/docs/id/env-vars)

<h3 id="oauth-callback-port-is-already-in-use">
  Port callback OAuth sudah digunakan
</h3>

Ketika Anda masuk ke server MCP jarak jauh dengan OAuth, Claude Code memulai pendengar lokal untuk menerima callback sign-in. Jika port yang pendengar itu butuhkan dipegang oleh proses lain, sign-in gagal dengan pesan ini. Ini sebagian besar terjadi dengan [port callback tetap](/docs/id/mcp#use-a-fixed-oauth-callback-port) yang ditetapkan melalui variabel [`MCP_OAUTH_CALLBACK_PORT`](/docs/id/env-vars) atau `--callback-port`, karena tanpa itu Claude Code memilih port yang tersedia.

```text theme={null}
OAuth callback port <port> is already in use — another process may be holding it. Run `lsof -ti:<port> -sTCP:LISTEN` to find it.
```

Di Windows, perintah yang disarankan adalah `netstat -ano | findstr :<port>` sebagai gantinya.

**Yang harus dilakukan:**

* Jalankan perintah dari pesan untuk menemukan proses yang memegang port, dan hentikan atau tunggu selesai
* Jika program lain memerlukan port itu secara permanen, daftarkan URI pengalihan berbeda dengan server dan atur port-nya dengan `MCP_OAUTH_CALLBACK_PORT` atau `--callback-port`, mana pun yang Anda gunakan
* Kemudian mulai sign-in lagi, misalnya dengan memilih server di `/mcp`

<h3 id="no-available-ports-for-oauth-redirect">
  Tidak ada port yang tersedia untuk pengalihan OAuth
</h3>

Ketika Anda masuk ke server MCP jarak jauh dengan [OAuth](/docs/id/mcp#authenticate-with-remote-mcp-servers), Claude Code memulai pendengar lokal untuk menerima callback sign-in. Sign-in gagal dengan pesan ini ketika Claude Code tidak dapat mengikat port lokal untuk itu. Sesuatu di mesin mencegah mendengarkan di `127.0.0.1`, misalnya perangkat lunak keamanan atau kebijakan sandbox yang menolak pendengar lokal.

```text theme={null}
No available ports for OAuth redirect
```

Sebelum v2.1.268, Claude Code tidak kembali ke port yang ditugaskan sistem operasi, jadi pesan juga muncul ketika hanya port yang dipilih sendiri tidak dapat diikat. Itu dapat terjadi pada host Windows di mana Hyper-V memesan rentang port yang mencakup port yang Claude Code pilih dari.

**Yang harus dilakukan:**

* Periksa apakah perangkat lunak keamanan atau kebijakan sandbox memblokir proses dari mendengarkan di `127.0.0.1`, dan izinkan Claude Code mengikat port lokal
* Kemudian mulai sign-in lagi, misalnya dengan memilih server di `/mcp`

<h3 id="security-review-fails-without-origin-head">
  /security-review gagal tanpa origin/HEAD
</h3>

[`/security-review`](/docs/id/commands#all-commands) membangun konteks review dengan membedakan cabang Anda terhadap `origin/HEAD`, ref lokal yang mencatat cabang mana yang default di remote `origin` Anda. Ketika ref itu tidak ada, perintah git yang mengumpulkan diff gagal dan review berhenti sebelum dimulai.

```text theme={null}
Error: Shell command failed for pattern "!`git diff --name-only origin/HEAD...`": [stderr]
fatal: ambiguous argument 'origin/HEAD...': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
```

Pesan dapat mengutip `git log` atau `git diff` berbeda sebagai gantinya. Git membuat `origin/HEAD` hanya ketika remote mengiklankan cabang default dan refspec fetch Anda mencakupnya, yang `git clone` penuh dari remote dengan commit lakukan. Ref hilang dalam setup ini:

* Checkout single-branch atau CI, yang mengambil refspec terlalu sempit
* Remote yang server-side HEAD menunjuk ke cabang yang tidak ada yang dorong
* Repository tanpa remote `origin`, atau yang tidak pernah Anda ambil

Claude Code menampilkan kesalahan yang sama untuk skill apa pun yang [menyuntikkan konteks dinamis](/docs/id/skills#when-an-injected-command-fails), dan perintah yang disuntikkan gagal membatalkan invokasi skill itu. Dua string saudara api sebelum perintah berjalan sama sekali:

* `Shell command permission check failed for pattern "..."`: pemeriksaan izin perintah tidak mengizinkannya. [Permission checks on injected commands](/docs/id/skills#permission-checks-on-injected-commands) mencakup hasil mana yang membatalkan dalam setiap mode izin dan cara pra-menyetujui perintah dengan `allowed-tools`
* ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``: frontmatter skill menuntut bash di mesin tanpanya. Instal Git untuk Windows atau ubah frontmatter ke `shell: powershell`. Lihat [How injected commands run](/docs/id/skills#how-injected-commands-run)

**Yang harus dilakukan:**

* Buat ref dengan menyebutkan cabang default remote Anda: `git remote set-head origin <default-branch>`. Ini berfungsi kapan pun ref pelacakan lokal `origin/<default-branch>` ada. Jika tidak, seperti dalam klon single-branch, ambil cabang terlebih dahulu: jalankan `git remote set-branches --add origin <branch>`, kemudian `git fetch origin`, kemudian jalankan kembali perintah set-head. Jalankan kembali `/security-review`.
* Jika Anda lebih suka tidak menyebutkan cabang, jalankan `git fetch origin` dan kemudian `git remote set-head origin --auto`, yang menanyakan remote cabang mana yang default-nya. Gagal dengan `error: Cannot determine remote HEAD` ketika remote tidak mengiklankan cabang default, karena kosong atau HEAD-nya menunjuk ke cabang yang tidak ada yang dorong; sebutkan cabang secara eksplisit sebagai gantinya. Gagal dengan `error: Not a valid ref` ketika klon Anda tidak mengambil cabang itu; perluas refspec seperti di atas terlebih dahulu.
* Jika repository tidak memiliki remote, tambahkan dengan `git remote add origin <url>` dan ambil sebelum membuat ref. Jika remote kosong, dorong cabang Anda terlebih dahulu dengan `git push -u origin HEAD` dan sebutkan cabang itu dalam perintah set-head; `origin/HEAD` kemudian menunjuk ke cabang yang baru saja Anda dorong, jadi `/security-review` melihat diff kosong sampai cabang menyimpang darinya.

<h3 id="input-must-be-provided-when-using-print">
  Input harus disediakan saat menggunakan --print
</h3>

`claude` telanjang memerlukan stdout menjadi terminal untuk memulai UI interaktif. Ketika stdout dialihkan, atau konsol bukan terminal nyata, seperti PowerShell ISE dan beberapa panel output IDE, `claude` berjalan [non-interaktif](/docs/id/headless) sebagai gantinya. Itu adalah mode yang sama dengan `claude -p`, yang memerlukan prompt, jadi pesan menyebutkan `--print` bahkan ketika Anda tidak melewatkan flag. Melewatkan `-p`/`--print` tanpa prompt dan tidak ada yang disalurkan di stdin menghasilkan kesalahan yang sama di mana pun.

```text theme={null}
Error: Input must be provided either through stdin or as a prompt argument when using --print
```

**Yang harus dilakukan:**

* Untuk penggunaan interaktif, jalankan `claude` di terminal nyata: Windows Terminal atau konsol PowerShell daripada ISE, dan terminal terintegrasi IDE Anda daripada panel output
* Untuk penggunaan satu kali, berikan prompt: `claude -p "your question"`, atau salurkan dengan `echo "your question" | claude -p`

<h3 id="input-contained-only-whitespace">
  Input hanya berisi whitespace
</h3>

Dalam [mode non-interaktif](/docs/id/headless), Claude Code menolak prompt yang terdiri sepenuhnya dari spasi, tab, atau baris baru alih-alih mengirimnya, karena API menolak pesan tanpa teks yang terlihat. Pesan mana yang Anda lihat tergantung di mana prompt kosong berasal:

* **Argumen prompt atau stdin yang disalurkan untuk `claude -p`**: `claude` keluar dengan `Error: Input contained only whitespace. Provide a prompt with text through stdin or as a prompt argument when using --print`
* **Pesan yang dikirimkan ke sesi `--input-format stream-json` atau [Agent SDK](/docs/id/agent-sdk/overview) yang berjalan**: Claude Code mengakhiri giliran tanpa memanggil model dan sesi tetap dapat digunakan. Penolakan tiba sebagai pesan informatif dan sebagai teks hasil giliran: `Blank prompt — the message was only whitespace, so nothing was sent to the model.`

Sebelum v2.1.229, Claude Code mengirimkan pesan whitespace-only ke API, yang menolak permintaan dengan kesalahan 400.

**Yang harus dilakukan:**

* Sertakan teks yang terlihat dalam prompt. Jika skrip membangun prompt dari variabel atau file, periksa bahwa sumber tidak kosong sebelum memanggil Claude Code.

<h3 id="stream-json-input-carried-over-256m-characters-with-no-newline">
  input stream-json membawa lebih dari 256M karakter tanpa baris baru
</h3>

Program Anda mengirim lebih dari 268.435.456 karakter di stdin tanpa baris baru ke run `claude -p --input-format stream-json`, jadi Claude Code mencetak kesalahan ini ke stderr dan keluar dengan kode 1 alih-alih membuffer input lebih lanjut. Pesan menyatakan anggaran itu sebagai `256M`. Sebelum v2.1.257, Claude Code membuffer input seperti itu tanpa batas, menumbuhkan memori sampai proses mogok atau dibunuh.

```text theme={null}
Error: stream-json input carried over 256M characters with no newline. Each stream-json message must be a single newline-terminated JSON line: either the producer is not newline-terminating its messages, or one message exceeded this budget.
```

Input sepanjang ini tanpa baris baru biasanya berarti produsen bukan produsen stream-json sama sekali, seperti file biner atau output log biasa yang disalurkan secara tidak sengaja. Pesan tunggal di atas anggaran gagal pemeriksaan yang sama.

**Yang harus dilakukan:**

* Periksa apa yang disalurkan ke stdin. Dengan [`--input-format stream-json`](/docs/id/cli-reference#cli-flags), setiap pesan harus satu baris JSON yang diakhiri baris baru
* Untuk mengirim teks biasa sebagai gantinya, lepaskan `--input-format stream-json`; `claude -p` membaca prompt teks biasa dari stdin secara default

<h3 id="unknown-command">
  Perintah tidak dikenal
</h3>

Dalam sesi terminal interaktif, Anda mengirimkan nama `/` yang tidak cocok dengan perintah apa pun dalam sesi ini, jadi Claude Code melaporkan nama alih-alih menjalankan apa pun:

```text theme={null}
Unknown command: /hepl. Did you mean /help?
```

Claude Code menyarankan nama perintah atau alias terdekat yang menu daftar dalam sesi ini. Ketika tidak ada yang dekat, pesan berakhir setelah nama. Penyebabnya biasanya salah satu dari berikut:

* Typo, seperti `/hepl` untuk `/help`. [How the command menu matches what you type](/docs/id/commands#how-the-command-menu-matches-what-you-type) mencakup memilih kecocokan dekat sebelum Anda kirimkan
* Perintah yang ada tetapi tidak tersedia dalam sesi ini karena persyaratan tidak terpenuhi, seperti platform, rencana, atau metode autentikasi Anda. Entri pemecahan masalah untuk [`/web-setup`](/docs/id/web-quickstart#web-setup-shows-no-commands-match-or-unknown-command) dan [`/schedule`](/docs/id/routines#schedule-returns-unknown-command) memandu dua kasus umum. Beberapa perintah menjawab dengan pesan mereka sendiri ketika kebijakan organisasi Anda menonaktifkannya, seperti [`Cloud sessions are disabled by your organization's policy`](#cloud-sessions-are-disabled-by-your-organizations-policy)
* Perintah dari [plugin](/docs/id/plugins/overview) atau [server MCP](/docs/id/mcp#use-mcp-prompts-as-commands) yang tidak diinstal atau terhubung dalam sesi ini

Claude Code menjawab nama `/` yang tidak cocok dengan cara ini hanya dalam sesi terminal interaktif. Dalam setiap sesi lain, mengirimkan prompt ke Claude sebagai pesan normal sebagai gantinya, dengan catatan bahwa perintah tidak berjalan dan daftar perintah Claude dapat jalankan dalam sesi. Sesi itu termasuk:

* run `-p`
* Aplikasi [Agent SDK](/docs/id/agent-sdk/overview)
* Tab Kode dari [Desktop app](/docs/id/desktop)
* Panel chat dari [VS Code extension](/docs/id/vs-code)
* [Sesi cloud](/docs/id/claude-code-on-the-web) dan [routines](/docs/id/routines)

Untuk perintah bawaan yang tidak dapat berjalan dalam salah satu sesi itu, Claude Code masih menjawab bahwa perintah tidak tersedia alih-alih mengirimnya ke Claude. Sebelum v2.1.274, hanya sesi cloud dan routines yang mengirimkan nama yang tidak cocok ke Claude. Sebelum v2.1.273, mereka menjawab `Unknown command` juga.

Claude Code tidak memperlakukan setiap prompt yang dimulai dengan `/` sebagai perintah. Mengirimkan prompt ke Claude sebagai pesan normal ketika kata pertama setelah `/` dimulai dengan tanda baca, seperti `/--` yang membuka komentar doc Lean, atau adalah jalur seperti `/var/log/syslog`.

Sebelum v2.1.236, jika Anda menekan `Enter` sementara menu perintah mencantumkan kecocokan dekat untuk nama yang Anda ketik, Claude Code menjalankan kecocokan itu, jadi typo seperti `/hepl` menjalankan `/help` alih-alih menghasilkan pesan ini.

**Yang harus dilakukan:**

* Jalankan nama yang disarankan, atau ketik `/` diikuti bagian dari nama untuk melihat apa yang tersedia dalam sesi ini
* Jika Claude Code melaporkan perintah yang didokumentasikan sebagai tidak dikenal, periksa barisnya dalam [referensi perintah](/docs/id/commands) untuk persyaratan yang disebutkan

<h3 id="diff-is-too-large-for-ultrareview">
  Diff terlalu besar untuk ultrareview
</h3>

Diff antara cabang Anda dan cabang dasar, termasuk perubahan yang tidak berkomitmen dan staged, melebihi batas ukuran untuk [ultrareview](/docs/id/ultrareview), jadi `/code-review ultra` dan subperintah `claude ultrareview` menolak review sebelum sesi cloud dimulai. Review yang ditolak tidak menggunakan run gratis dan tidak menagih kredit penggunaan. Pesan menyebutkan batas yang berlaku, ukuran diff Anda, dan file yang berkontribusi paling banyak baris yang berubah. Sebelum v2.1.216, pesan menampilkan hanya statistik diff mentah.

```text theme={null}
Diff is too large for ultrareview: 812 files, 96,410 lines changed (limits: 500 files, 8,000 lines). Largest files: package-lock.json (41,904 lines), dist/bundle.js (18,210 lines), src/generated/api.ts (9,876 lines). Pass a closer base branch (`/code-review ultra <branch>`) to narrow the scope, or split the change.
```

Meninjau pull request menerapkan batas yang sama; bentuk pesan itu dimulai `PR #<N> is too large for ultrareview` dan menyebutkan file dan jumlah baris PR.

**Yang harus dilakukan:**

* Berikan cabang dasar yang lebih dekat ke pekerjaan Anda, seperti `/code-review ultra develop`, jadi review mencakup hanya diff terhadap cabang itu
* Pisahkan perubahan menjadi cabang yang lebih kecil dan tinjau masing-masing. File yang pesan sebutkan berkontribusi paling banyak baris yang berubah, jadi mulai dengan memindahkan itu ke cabang mereka sendiri.

<h3 id="could-not-find-merge-base-with-the-base-branch">
  Tidak dapat menemukan merge-base dengan cabang dasar
</h3>

`/code-review ultra` dan subperintah `claude ultrareview` meninjau diff antara cabang Anda dan cabang dasar, yang memerlukan komitmen yang keduanya bagikan. Ketika `git merge-base` tidak menemukan apa pun, Claude Code menolak review sebelum sesi cloud dimulai. Pada klon Claude Code dapat verifikasi lengkap, dengan setidaknya satu cabang, itu kembali ke [meninjau setiap file yang dilacak](/docs/id/ultrareview#diff-limits-and-fallbacks) alih-alih menolak. Anda melihat penolakan ini ketika cabang dasar tidak dapat ditemukan sama sekali, ketika Claude Code tidak dapat memverifikasi bahwa klon Anda lengkap, atau di repository langka di mana diff seluruh pohon tidak mungkin, seperti format objek SHA-256.

```text theme={null}
Could not find merge-base with main. Pass the base branch explicitly (e.g. `/code-review ultra develop`) or make sure you're in a git repo with a main branch.
```

Petunjuk setelah kalimat pertama tergantung pada apa yang Claude Code amati:

* **Anda tidak melewatkan cabang dasar**: Claude Code dibandingkan terhadap cabang default repository dan menyarankan melewatkan dasar Anda secara eksplisit, seperti dalam contoh di atas
* **Anda melewatkan cabang dasar yang sudah ada di klon Anda**: petunjuk membaca ``Make sure <branch> exists locally or on origin (try `git fetch origin <branch>`)``
* **Anda melewatkan cabang dasar yang tidak ada di klon Anda**: Claude Code mengambilnya dari origin sebelum membandingkan. Petunjuk membaca ``<branch> was fetched from origin but shares no history with HEAD. If another branch is your real base, pass it explicitly (`/code-review ultra <branch>`)``; ketika Claude Code tidak dapat mengatakan apakah klon Anda dangkal, itu menyarankan `git fetch --unshallow origin` sebagai gantinya. Sebelum v2.1.221, petunjuk menyarankan `git fetch --unshallow origin` untuk setiap cabang dasar yang diambil, dan pada klon lengkap perintah itu gagal dengan `fatal: --unshallow on a complete repository does not make sense`.

**Yang harus dilakukan:**

* Jika cabang lain adalah dasar nyata Anda, berikan secara eksplisit: `/code-review ultra <branch>`
* Jika klon Anda mungkin tidak memiliki riwayat penuh, jalankan `git fetch --unshallow origin` dan jalankan kembali review

<h3 id="your-checkout-has-no-branches">
  Checkout Anda tidak memiliki cabang
</h3>

Checkout dapat memiliki komitmen tetapi tidak ada cabang: jika Anda menjalankan `git init` diikuti `git fetch <url>` dan `git checkout FETCH_HEAD`, Anda mendapatkan HEAD terlepas dengan tidak ada refs. Claude Code mengemas repository Anda sebagai bundel git untuk mengunggahnya untuk [ultrareview](/docs/id/ultrareview), dan tidak dapat membundel repository yang tidak memiliki cabang atau refs lainnya, jadi `/code-review ultra` dan subperintah `claude ultrareview` menolak review sebelum sesi cloud dimulai.

```text theme={null}
Your checkout has no branches (detached HEAD only), which cloud review can't bundle. Create one first — `git checkout -b <name>` — then rerun /code-review ultra.
```

Sebelum v2.1.221, Claude Code mencoba meninjau setiap file yang dilacak dalam checkout ini, dan unggahan gagal.

**Yang harus dilakukan:**

* Buat cabang di komitmen saat ini Anda dengan `git checkout -b <name>`, kemudian jalankan kembali review

<h3 id="no-github-account-is-connected-to-your-claude-account">
  Tidak ada akun GitHub yang terhubung ke akun Claude Anda
</h3>

Anda menjalankan `/code-review ultra <PR#>` atau `claude ultrareview <PR#>`, dan sebelum membuat sesi cloud Claude Code menanyakan server apakah [akun GitHub yang terhubung ke akun Claude Anda](/docs/id/ultrareview#review-a-pull-request) dapat menjangkau repository PR. Tidak ada akun yang terhubung, atau koneksi kedaluwarsa, jadi klon cloud akan gagal dan Claude Code menolak peluncuran. Claude Code tidak menghabiskan run gratis atau menagih kredit penggunaan untuk peluncuran yang ditolak.

```text theme={null}
Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected (or the connection expired). To fix: run /web-setup to reuse your GitHub CLI login, or connect an account at https://claude.ai/connect-github — then re-run /code-review ultra 1234 (allow a minute after connecting).
```

Ketika [`/web-setup`](/docs/id/web-quickstart#connect-from-your-terminal) tidak tersedia dalam sesi Anda, pesan hanya menyebutkan tautan claude.ai.

**Yang harus dilakukan:**

* Jalankan `/web-setup` untuk menghubungkan login GitHub CLI Anda ke akun Claude Anda, atau hubungkan akun di [claude.ai/connect-github](https://claude.ai/connect-github)
* Jalankan kembali review satu menit setelah terhubung

Sebelum v2.1.248, Claude Code tidak memeriksa ini sebelum peluncuran.

<h3 id="your-connected-github-account-cant-see-the-repository">
  Akun GitHub yang terhubung Anda tidak dapat melihat repository
</h3>

Anda menjalankan `/code-review ultra <PR#>` atau `claude ultrareview <PR#>`, dan [akun GitHub yang terhubung ke akun Claude Anda](/docs/id/ultrareview#review-a-pull-request) tidak dapat membaca repository PR, jadi klon cloud akan gagal dan Claude Code menolak peluncuran. Claude Code tidak menghabiskan run gratis atau menagih kredit penggunaan untuk peluncuran yang ditolak.

```text theme={null}
Your connected GitHub account can't see <owner>/<repo> — usually the Claude GitHub app isn't installed on <owner> or wasn't granted this repo (web-connected accounts need it for private repos), or a different GitHub account is connected. To fix: run /web-setup to reuse your GitHub CLI login, or install the app at https://github.com/apps/claude/installations/new — then re-run /code-review ultra 1234.
```

Ketika [`/web-setup`](/docs/id/web-quickstart#connect-from-your-terminal) tidak tersedia dalam sesi Anda, pesan hanya menyebutkan instalasi aplikasi.

**Yang harus dilakukan:**

* Jika CLI `gh` lokal Anda dapat membaca repository, jalankan `/web-setup` untuk menghubungkan login itu ke akun Claude Anda
* Jalankan kembali review setelah perubahan

Sebelum v2.1.248, Claude Code tidak memeriksa ini sebelum peluncuran.

<h3 id="the-github-app-preflight-failed-transiently">
  Preflight GitHub App gagal secara sementara
</h3>

Anda memulai [sesi cloud](/docs/id/claude-code-on-the-web) dari repository lokal, dan dua langkah gagal bersama-sama. Claude Code tidak dapat membangun atau mengunggah bundel repository Anda. Sebelum unggahan, itu memeriksa apakah layanan cloud dapat mengklon repository dari GitHub, dan daripada jawaban pasti, pemeriksaan itu berakhir dalam kesalahan yang retry dapat bersihkan, seperti kesalahan jaringan, timeout, atau kesalahan server sementara. Pesan lengkap dimulai dengan apa yang menghentikan bundel, misalnya `Could not upload repo bundle (<error>)`, dan berakhir dengan kalimat preflight:

```text theme={null}
Could not upload repo bundle (<error>). The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead
```

**Yang harus dilakukan:**

* Jalankan kembali perintah setelah beberapa saat. Ketika pemeriksaan GitHub lulus, Claude Code dapat memulai sesi dari klon GitHub, jadi unggahan yang gagal tidak lagi memblokir peluncuran
* Jika retry terus gagal, awal pesan menyebutkan apa yang menghentikan unggahan. Ketika penyebab itu adalah sesuatu yang dapat Anda perbaiki, perbaiki sehingga sesi dapat dimulai dari repository lokal Anda sebagai gantinya

Sebelum v2.1.251, Claude Code mengakhiri pesan dengan `Please set up GitHub on https://claude.ai/code` bahkan ketika pemeriksaan GitHub gagal hanya secara sementara, dan saran setup tidak dapat membersihkan kegagalan sementara.

<h3 id="github-isnt-connected-to-your-claude-account">
  GitHub tidak terhubung ke akun Claude Anda
</h3>

Anda memulai [sesi cloud](/docs/id/claude-code-on-the-web) dari repository lokal Anda, misalnya dengan `/autofix-pr`. Tidak ada akun GitHub yang terhubung ke akun Claude Anda, atau koneksi kedaluwarsa, jadi Claude Code menolak peluncuran:

```text theme={null}
GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud. Run /web-setup to connect with your GitHub CLI login, or connect on the web at https://claude.ai/connect-github
```

Ketika Anda membuat routine dengan [`/schedule`](/docs/id/routines), pesan yang sama muncul sebagai catatan setup yang menyebutkan repository; catatan tidak memblokir pembuatan routine.

**Yang harus dilakukan:**

* Jalankan `/web-setup` untuk menghubungkan login GitHub CLI Anda ke akun Claude Anda, atau hubungkan akun di [claude.ai/connect-github](https://claude.ai/connect-github). Lihat [GitHub authentication options](/docs/id/claude-code-on-the-web#github-authentication-options) untuk bagaimana keduanya berbeda.
* Jalankan kembali perintah satu menit setelah terhubung

Sebelum v2.1.268, Claude Code melaporkan ini sebagai kegagalan sementara pemeriksaan Claude GitHub App dan menyarankan retry atau menginstal aplikasi; tidak satupun menghubungkan akun GitHub.

<h3 id="single-sign-on-authorization-needed">
  Otorisasi single sign-on diperlukan
</h3>

Anda menjalankan [`/install-github-app`](/docs/id/github-actions#quick-setup) dan memilih repository yang organisasinya memberlakukan SAML single sign-on. Sebelum setup, Claude Code memeriksa akses Anda ke repository dengan CLI GitHub, dan GitHub menolak pemeriksaan itu karena token `gh` Anda belum diotorisasi untuk organisasi itu. Wizard menampilkan peringatan dengan langkah-langkah untuk mengotorisasi:

```text theme={null}
Single sign-on authorization needed
<owner>/<repo> belongs to an organization that enforces SAML single sign-on, and your GitHub CLI token isn't authorized for it yet.
```

**Yang harus dilakukan:**

* Otorisasi kembali login GitHub CLI Anda dengan cakupan `repo` dan `workflow` dengan menjalankan `gh auth refresh -h github.com -s repo,workflow`, dan otorisasi organisasi ketika GitHub meminta single sign-on
* Jika Anda mengautentikasi dengan token akses pribadi di `GH_TOKEN`, buka [github.com/settings/tokens](https://github.com/settings/tokens), pilih **Configure SSO** pada token, dan otorisasi organisasi
* Jalankan `/install-github-app` lagi

Sebelum v2.1.273, Claude Code menampilkan peringatan `Admin permissions required` untuk kondisi ini sebagai gantinya.

<h3 id="failed-to-resume-the-conversation">
  Gagal melanjutkan percakapan
</h3>

Claude Code tidak dapat membaca atau memproses transkrip yang disimpan untuk sesi yang Anda pilih dari [pemilih `claude --resume`](/docs/id/sessions#use-the-session-picker), jadi mengakhiri proses daripada melanjutkan dalam keadaan sebagian dimuat. Pesan mencakup perintah untuk retry:

```text theme={null}
Failed to resume the conversation.
Run claude --resume <session-id> to retry, or claude to start a new session.
```

Claude Code keluar dengan kode 1 setelah menampilkan pesan. Pemilih `/resume` di dalam sesi yang berjalan melaporkan `Failed to resume conversation` dalam percakapan sebagai gantinya, dan sesi saat ini Anda terus berjalan. Sebelum v2.1.216, resume yang gagal dari pemilih `claude --resume` tetap di spinner `Resuming conversation…` tanpa batas waktu alih-alih menampilkan pesan ini.

**Yang harus dilakukan:**

* Jalankan `claude --resume <session-id>` dengan ID sesi dari pesan untuk retry
* Jika setiap retry gagal dengan cara yang sama, jalankan `claude update` dan resume lagi. Versi sebelum v2.1.275 gagal resume ketika transkrip yang disimpan berisi entri yang tidak dapat dibaca.
* Jika retry gagal lagi, jalankan `claude` untuk memulai sesi baru

<h3 id="no-conversation-found-with-the-session-id">
  Tidak ada percakapan yang ditemukan dengan ID sesi
</h3>

Anda melewatkan ID sesi ke `claude --resume <session-id>` dan tidak ada transkrip yang disimpan cocok:

```text theme={null}
No conversation found with session ID: <session-id>
```

Claude Code keluar dengan kode 1 setelah menampilkan pesan. Claude Code [mencari proyek saat ini terlebih dahulu, kemudian setiap proyek lain di mesin ini](/docs/id/sessions#resume-a-session) untuk ID. Sebelum v2.1.223, pencarian berhenti di direktori proyek saat ini dan worktree git-nya, jadi resume dari direktori tempat sesi terakhir bekerja.

Penyebab umum:

* **ID yang salah ketik**: untuk run non-interaktif, ID adalah bidang `session_id` dari output [`--output-format json`](/docs/id/headless#get-structured-output)
* **Transkrip yang dihapus**: Claude Code menghapus transkrip setelah [periode retensi](/docs/id/sessions#where-transcripts-are-stored), 30 hari secara default, mengikuti [aturan pembersihan retensi](/docs/id/claude-directory#cleaned-up-automatically)
* **Mesin berbeda**: Claude Code menyimpan transkrip secara lokal, jadi resume sesi di mesin tempat itu berjalan
* **Salinan duplikat**: jika Anda menyalin direktori proyek di bawah `~/.claude/projects` sehingga dua transkrip membawa ID yang sama, Claude Code melaporkan pesan ini daripada resume satu salinan secara sewenang-wenang

**Yang harus dilakukan:**

* Untuk sesi interaktif, buka [pemilih sesi](/docs/id/sessions#use-the-session-picker) dengan `claude --resume` dan tekan `Ctrl+A` untuk memperluas ke setiap proyek di mesin ini, kemudian pilih sesi
* Sesi yang dibuat dengan `claude -p` atau [Agent SDK](/docs/id/agent-sdk/overview) tidak muncul dalam pemilih, jadi periksa kembali ID terhadap `session_id` yang run asli Anda cetak

<h3 id="cannot-switch-renderers-in-this-session">
  Tidak dapat mengganti renderer dalam sesi ini
</h3>

Ketika Anda mengganti renderer, Claude Code memulai ulang prosesnya. Anda menjalankan [`/tui`](/docs/id/fullscreen#enable-fullscreen-rendering) dalam sesi Claude Code menolak untuk memulai ulang, jadi tidak mengganti dan menyimpan apa pun. Pesan mana yang Anda lihat memberi tahu Anda penyebabnya:

* `Cannot switch renderers while work is running in the background`: Anda memiliki pekerjaan latar belakang yang berjalan yang restart akan meninggalkan, seperti shell latar belakang atau subagen. Tunggu pekerjaan selesai atau hentikan dengan [`/tasks`](/docs/id/commands), kemudian jalankan `/tui fullscreen` atau `/tui default` lagi
* `Cannot switch renderers in this session`: sesi memiliki pembatasan Claude Code tidak dapat lulus ke proses yang dimulai ulang. Sebelum v2.1.234, Claude Code memulai ulang bagaimanapun dan sesi yang diluncurkan kembali berjalan tanpanya

Dalam pesan pembatasan, bagian dalam tanda kurung menyebutkan pembatasan Claude Code temukan:

```text theme={null}
Cannot switch renderers in this session — it has restrictions a restart can't carry over (permission rules set for this session only). Nothing was changed. Running /tui fullscreen in a session started without them switches every later session too.
```

Setiap alasan pesan dapat tunjukkan dalam tanda kurung:

* `launch flags: a custom system prompt, a tool allowlist, or restricted settings`: Anda memulai sesi dengan flag Claude Code tidak lulus kembali ke proses yang dimulai ulang. Flag ini termasuk [`--system-prompt`](/docs/id/cli-reference#cli-flags), `--system-prompt-file`, `--append-system-prompt-file`, daftar allowlist [`--tools`](/docs/id/cli-reference#cli-flags), [`--setting-sources`](/docs/id/cli-reference#cli-flags), dan [`--permission-prompt-tool`](/docs/id/cli-reference#cli-flags)
* `permission rules set for this session only`: [pembaruan izin](/docs/id/hooks#permission-update-entries) dari hook atau pemanggil SDK menambahkan aturan deny atau ask dengan tujuan `session`. Aturan allow yang scoped sesi tidak memicu penolakan. Restart menjatuhkan mereka, dan Claude Code meminta lagi sebagai gantinya
* `ask-before-running rules with no command-line form`: pembaruan izin dari hook atau pemanggil SDK menambahkan aturan ask bersama aturan Claude Code lulus kembali sebagai `--allowed-tools` dan `--disallowed-tools`. Tidak ada flag untuk aturan ask
* `permission rules a command line cannot carry intact` dan `added directories a command line cannot carry intact`: pembaruan izin menambahkan aturan atau jalur direktori mid-session. Baris perintah proses yang dimulai ulang tidak dapat membawa teksnya sebagai nilai yang sama

**Yang harus dilakukan:**

* Dalam sesi yang dimulai tanpa pembatasan itu, jalankan `/tui fullscreen`, atau `/tui default` untuk beralih kembali. Claude Code menyimpan pengaturan [`tui`](/docs/id/settings-reference#tui) di sana

<h3 id="couldnt-open-claude-desktop">
  Couldn't open Claude Desktop
</h3>

Anda menjalankan [`/desktop`](/docs/id/desktop#coming-from-the-cli), atau alias `/app`-nya, dan perintah sistem yang Claude Code gunakan untuk membuka Claude Desktop gagal. Sesi tetap di terminal.

```text theme={null}
Error: Couldn't open Claude Desktop (`open` exited 1: LSOpenURLsWithRole() failed for the URL claude://resume?session=<session-id> with error -10814). Open Claude Desktop and run /desktop again.
```

**Yang harus dilakukan:**

* Buka Claude Desktop sendiri, kemudian jalankan `/desktop` lagi
* Untuk membaca output kesalahan lengkap perintah itu, aktifkan debug logging dengan `/debug`, jalankan `/desktop` lagi, dan periksa log debug

Sebelum v2.1.275, pesan adalah `Failed to open Claude Desktop. Please try opening it manually.` dan tidak mengatakan apa yang gagal.

<h3 id="terminal-setup-left-your-zed-keymap-unchanged">
  /terminal-setup meninggalkan keymap Zed Anda tidak berubah
</h3>

Anda menjalankan [`/terminal-setup`](/docs/id/terminal-config#enter-multiline-prompts) di Zed, dan Claude Code tidak dapat menyelesaikan pembaruan ke `keymap.json` Zed Anda, jadi meninggalkannya seperti apa adanya.

Setiap pesan menyebutkan jalur ke keymap Anda dan berakhir dengan blok keybinding untuk ditambahkan sendiri:

```text theme={null}
Couldn't update your Zed keymap, so it was left unchanged.
To add the binding yourself, add this block to the keymap array in <path to keymap.json>:
{ "context": "Terminal", "bindings": { "shift-enter": ["terminal::SendText", "\u001b\r"] } }
```

Baris pertama pesan menyebutkan penyebabnya:

* `Couldn't read your Zed keymap, so it was left unchanged.`: Claude Code tidak dapat membaca file, misalnya karena izin file
* `Your Zed keymap isn't a readable list of keybindings, so it was left unchanged.`: file dibaca dengan baik tetapi tidak diuraikan sebagai array blok keybinding, bahkan dengan komentar `//` dan trailing comma yang diizinkan
* `Couldn't back up your Zed keymap; not modifying it.`: Claude Code tidak dapat menyalin file ke backup `.bak` di sebelahnya, jadi tidak mengubah apa pun
* `Couldn't update your Zed keymap, so it was left unchanged.`: hasil yang digabung tidak memverifikasi sebagai keymap yang valid membawa keybinding, jadi Claude Code membuangnya alih-alih menulis. Blok keybinding dengan kunci yang diduplikasi dapat menyebabkan ini

**Yang harus dilakukan:**

* Salin blok dari pesan ke array tingkat atas di `keymap.json` Anda di jalur yang pesan sebutkan
* Untuk `isn't a readable list of keybindings`, perbaiki kesalahan sintaks, atau buat nilai tingkat atas file menjadi array, kemudian jalankan `/terminal-setup` lagi

Sebelum v2.1.247, `/terminal-setup` tidak dapat menguraikan keymap Zed yang menggunakan komentar `//` atau trailing comma, dan mengganti seluruh file dengan hanya keybinding-nya sendiri sambil melaporkan keybinding sebagai diinstal. Untuk mengembalikan keymap yang versi sebelumnya ganti, gunakan file backup `.bak` yang dijelaskan di bawah [Enter multiline prompts](/docs/id/terminal-config#enter-multiline-prompts).

<h3 id="skill-usage-reports-are-not-available-on-this-connection">
  Laporan penggunaan skill tidak tersedia pada koneksi ini
</h3>

Anda menjalankan [`/skill-doctor`](/docs/id/skills#find-unused-skills) melalui [Remote Control](/docs/id/remote-control), dari ponsel atau browser Anda. Claude Code tidak mengirimkan laporan penggunaan skill melalui Remote Control dan menjawab dengan pesan ini sebagai gantinya:

```text theme={null}
Skill usage reports are not available on this connection.
```

**Yang harus dilakukan:**

* Jalankan `/skill-doctor` di terminal di mesin tempat sesi berjalan, atau jalankan `claude -p "/skill-doctor"` di sana

<h3 id="custom-output-styles-cant-be-selected-over-remote-control">
  Gaya output kustom tidak dapat dipilih melalui Remote Control
</h3>

Anda menjalankan [`/output-style`](/docs/id/output-styles#change-your-output-style) dari aplikasi mobile atau web melalui [Remote Control](/docs/id/remote-control), atau perintah tiba dalam pesan yang disalurkan ke sesi. Karena giliran seperti itu mungkin tidak berasal dari pemilik akun, Claude Code mencantumkan dan memilih hanya [gaya bawaan](/docs/id/output-styles#built-in-output-styles) di atasnya, dan menambahkan pemberitahuan ini kapan pun perintah mencantumkan gaya atau tidak mengenali nama yang Anda berikan. Nama [gaya kustom](/docs/id/output-styles#create-a-custom-output-style) mendapat jawaban yang sama dengan nama yang tidak ada:

```text theme={null}
Custom output styles can't be selected over Remote Control or from a relayed message. Select one in the session itself, or pick a built-in style here.
```

**Yang harus dilakukan:**

* Pilih gaya bawaan, misalnya `/output-style concise`
* Untuk menggunakan gaya kustom, atur [`outputStyle`](/docs/id/settings-reference#outputstyle) di `.claude/settings.local.json` proyek, atau jalankan `/output-style <style>` di terminal sesi sendiri jika memilikinya

<h3 id="output-styles-are-saved-to-local-settings-which-this-session-doesnt-load">
  Gaya output disimpan ke pengaturan lokal yang sesi ini tidak muat
</h3>

Anda mencoba mengganti [gaya output](/docs/id/output-styles) dengan `/output-style <style>` atau `/config outputStyle=<style>` dalam sesi yang sumber pengaturannya mengecualikan `local`. Contohnya adalah sesi [Agent SDK](/docs/id/agent-sdk/typescript) yang [`settingSources`](/docs/id/agent-sdk/typescript#options)-nya meninggalkan `"local"` dan sesi CLI yang dimulai dengan nilai [`--setting-sources`](/docs/id/cli-reference#cli-flags) yang meninggalkan local. Kedua perintah menyimpan gaya ke `.claude/settings.local.json`, file yang sesi seperti itu tidak pernah baca kembali, jadi Claude Code menolak alih-alih menulis pengaturan yang tidak akan berpengaruh:

```text theme={null}
Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load, so the style can't be changed here.
```

**Yang harus dilakukan:**

* Tambahkan `local` ke sumber pengaturan sesi dan beralih lagi
* Atur kunci [`outputStyle`](/docs/id/settings-reference#outputstyle) dalam file pengaturan yang sesi muat, seperti `.claude/settings.json` di proyek atau `~/.claude/settings.json`. Dalam SDK TypeScript, atur `outputStyle` di dalam objek `settings` inline sebagai gantinya; lihat [Activate an output style](/docs/id/agent-sdk/modifying-system-prompts#activate-an-output-style)

<h2 id="plugin-errors">
  Kesalahan plugin
</h2>

Kesalahan ini berasal dari konfigurasi [plugin](/docs/id/plugins/overview) dan [marketplace](/docs/id/plugins/overview). Untuk masalah plugin yang tidak menghasilkan salah satu pesan di halaman ini, seperti URL marketplace yang tidak memuat atau plugin yang terpasang tetapi tidak muncul, lihat [Pemecahan masalah plugin](/docs/id/plugins/troubleshooting).

<h3 id="plugin-eval-is-currently-in-early-access">
  plugin eval is currently in early access
</h3>

Anda menjalankan [`claude plugin eval`](/docs/id/plugin-evals) atau `claude plugin eval init` dan keluar dengan kode 1 dengan salah satu pesan ini sebelum melakukan apa pun:

```text theme={null}
`plugin eval` is currently in early access
```

```text theme={null}
`plugin eval` is currently unavailable
```

Pesan pertama berarti build Anda lebih lama dari v2.1.269, versi pertama di mana perintah tersedia secara umum. Pesan kedua berarti Anthropic telah mematikan perintah di sisi server; tidak ada yang di mesin Anda yang menghidupkannya kembali.

**Yang harus dilakukan:**

* Jalankan `claude --version`, kemudian `claude update`, dan jalankan perintah lagi dalam sesi baru. Lihat [persyaratan untuk plugin evals](/docs/id/plugin-evals#requirements)
* Jika Anda melihat pesan kedua pada build saat ini, coba lagi nanti setelah `claude update` lain

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace is registered from an untrusted source
</h3>

Marketplace terdaftar dengan nama yang [dicadangkan untuk marketplace Anthropic resmi](/docs/id/plugins/marketplace-reference#marketplace-file), tetapi sumber terdaftarnya bukan repositori GitHub `anthropics`. Claude Code memeriksa ulang nama yang dicadangkan setiap kali memuat atau menyegarkan marketplace, sehingga marketplace dan plugin yang dipasang darinya berhenti memuat. Sebelum v2.1.205, nama hanya diperiksa ketika marketplace ditambahkan, jadi entri yang terdaftar sebelum namanya dicadangkan terus memuat.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

Untuk marketplace yang sumbernya bukan repositori GitHub atau URL Git, seperti direktori lokal, kalimat tengah berbunyi `can only be used with GitHub sources from the 'anthropics' organization` sebagai gantinya. `claude plugin marketplace add` menjalankan pemeriksaan yang sama, dan menolak nama yang dicadangkan dengan `Failed to add marketplace:` diikuti oleh kalimat nama yang dicadangkan yang sama.

**Yang harus dilakukan:**

* Jika marketplace sudah terdaftar, jalankan `claude plugin marketplace remove <name>`, kemudian tambahkan lagi dari repositori `github.com/anthropics` resmi
* Jika Anda menerbitkan marketplace pihak ketiga yang menggunakan nama sebelum dicadangkan, ubah namanya dan minta pengguna untuk menambahkannya kembali dari sumber Anda
* Lihat daftar nama yang dicadangkan di bawah [Marketplace schema](/docs/id/plugins/marketplace-reference#marketplace-file)

<h3 id="marketplace-name-is-another-spelling-of-a-reserved-name">
  Marketplace name is another spelling of a reserved name
</h3>

Nama marketplace bukan sendiri nama yang dicadangkan, tetapi Claude Code memperlakukannya sebagai ejaan lain dari satu. [Reserved names](/docs/id/plugins/marketplace-reference#reserved-name-spellings) mencantumkan ejaan mana yang dihitung sebagai nama yang dicadangkan. Claude Code menolak nama seperti itu ketika Anda menambahkan marketplace:

```text theme={null}
Failed to add marketplace: "claude.code.plugins" is another spelling of "claude-code-plugins", a reserved marketplace name.
```

Ketika marketplace sudah terdaftar dengan nama seperti itu, entrinya berhenti memuat, dan `/plugin`, `claude plugin install`, dan `claude plugin update` memperingatkan:

```text wrap theme={null}
known_marketplaces.json has an entry named "claude.code.plugins", another spelling of the reserved marketplace name "claude-code-plugins", so it is ignored. Remove it with: claude plugin marketplace remove claude.code.plugins
```

Ketika nama memerlukan quoting shell, penolakan saat penambahan berbunyi `This marketplace's name is another spelling of "<reserved>", a reserved marketplace name. It is not exactly the reserved name it appears to be.`

**Yang harus dilakukan:**

* Ubah nama marketplace menjadi nama yang tidak mengeja nama yang dicadangkan dan tambahkan lagi
* Untuk peringatan entri yang diabaikan, jalankan perintah `claude plugin marketplace remove` yang diberikannya, atau hapus entri dari `~/.claude/plugins/known_marketplaces.json`

<h3 id="marketplace-is-already-added-from-a-different-source">
  Marketplace is already added from a different source
</h3>

Anda mengonfirmasi penambahan marketplace melalui [`/plugin install <plugin> --marketplace <source>`](/docs/id/plugins/install#add-a-marketplace-and-install-in-one-command), dan katalog yang Claude Code ambil dari sumber itu menamai dirinya sama dengan marketplace yang sudah Anda tambahkan dari sumber berbeda. Claude Code menyimpan marketplace yang ada daripada menggantinya, dan plugin tidak terpasang.

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

**Yang harus dilakukan:**

* Jika marketplace yang sudah Anda tambahkan adalah yang Anda inginkan, pasang dari sana berdasarkan nama: `/plugin install <plugin>@<name>`
* Untuk beralih ke sumber baru, jalankan `/plugin marketplace remove <name>`, kemudian coba instalasi lagi

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

Hook plugin, [monitor](/docs/id/plugins/components#monitors), atau perintah MCP [`headersHelper`](/docs/id/mcp#use-dynamic-headers-for-custom-authentication) mereferensikan opsi `${user_config.KEY}` [plugin](/docs/id/plugins/manifest-reference#user-configuration), dan string yang disubstitusi akan diteruskan ke shell. Nilai yang dikonfigurasi berisi `$(...)`, backtick, atau `;` akan berjalan sebagai kode di sana, jadi Claude Code menolak untuk memulai komponen daripada mensubstitusi nilai. Pemeriksaan berjalan pada template perintah, jadi kesalahan muncul bahkan ketika tidak ada nilai yang dikonfigurasi. Sebelum v2.1.207, nilai disubstitusi ke dalam perintah shell.

Kata-katanya tergantung pada permukaan mana yang mereferensikan opsi. Hook bentuk shell melaporkan:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Monitor melaporkan:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

MCP `headersHelper` melaporkan:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Yang harus dilakukan:**

* Untuk hook, tambahkan array `args` sehingga berjalan dalam [exec form](/docs/id/hooks#exec-form-and-shell-form), di mana setiap `${user_config.KEY}` menjadi satu argumen tanpa shell di antaranya. Atau lepaskan referensi dan baca variabel lingkungan `$CLAUDE_PLUGIN_OPTION_<KEY>` di dalam skrip
* Untuk monitor, lepaskan referensi dan buat skrip monitor membaca nilai dari file konfigurasi
* Untuk `headersHelper`, pindahkan `${user_config.KEY}` ke bidang `headers` server, yang tidak diurai shell, atau baca nilai di dalam skrip helper

<h3 id="plugin-archive-integrity-check-failed">
  Plugin archive integrity check failed
</h3>

Entri marketplace plugin menggunakan sumber [`archive`](/docs/id/plugins/marketplace-reference#archive-plugin-source) dengan pin `sha256`, dan digest file yang diunduh tidak cocok dengan pin. Claude Code menolak instalasi, jadi tidak ada yang berubah dalam cache plugin. Ketidakcocokan memiliki tiga kemungkinan penyebab:

* File di URL berubah setelah penulis menghitung pin
* Penulis memasukkan digest yang salah di entri marketplace
* URL melayani file yang berbeda dari yang penulis pin

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

**Yang harus dilakukan:**

* Jika Anda menerbitkan plugin, hitung ulang digest file yang tepat yang dilayani URL, misalnya dengan `shasum -a 256 my-plugin.zip`, atau `Get-FileHash -Algorithm SHA256 my-plugin.zip` di PowerShell, dan perbarui `sha256` di entri marketplace
* Jika Anda memasang plugin, jalankan `/plugin marketplace update <name>` untuk menyegarkan katalog jika entri telah diperbaiki, kemudian coba instalasi lagi
* Jika digest masih tidak setuju setelah penyegaran, tanyakan kepada pemilik marketplace file mana yang mereka pin sebelum memasang

<h3 id="path-escapes-plugin-directory">
  Path escapes plugin directory
</h3>

Jalur komponen plugin, yang dideklarasikan dalam `plugin.json` plugin atau dalam [entri marketplace](/docs/id/plugins/marketplace-reference#plugin-entries), diselesaikan di luar direktori plugin itu sendiri. Claude Code menghapus jalur itu dan memuat sisa plugin. Nama komponen dalam pesan, seperti `commands` atau `hooks`, menamai bidang yang mendeklarasikan jalur.

```text theme={null}
commands path escapes plugin directory: ./../shared.md
```

Dalam output perintah `claude plugin`, kesalahan yang sama berbunyi `Path escapes plugin directory: ./../shared.md (commands)`.

Claude Code menolak baik jalur yang menunjuk di luar plugin seperti yang ditulis, seperti `../shared-utils`, dan symlink yang mengarah di luar plugin dan bukan salah satu yang [aturan symlink marketplace](/docs/id/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks) izinkan. Untuk symlink, pesan juga mengatakan di mana jalur diselesaikan:

```text theme={null}
commands path escapes plugin directory: ./commands/deploy.md — it resolves to /home/user/shared/deploy.md, outside the plugin directory
```

Di macOS dan Linux, Claude Code juga menolak jalur komponen yang berisi backslash di mana pun di dalamnya, bahkan ketika jalur tetap berada di dalam plugin. Plugin yang jalur komponen menggunakan pemisah gaya Windows memuat di Windows dan memicu penolakan ini di platform lain:

```text theme={null}
commands path escapes plugin directory: ./commands\deploy.md — its path contains a backslash, which is not resolved reliably on this platform
```

Sebelum v2.1.251, Claude Code memuat jalur `commands` yang dideklarasikan dalam entri marketplace bahkan ketika menunjuk di luar direktori plugin. Claude Code sudah menolak jalur yang dideklarasikan dalam `plugin.json` dan jalur komponen lainnya dalam entri marketplace.

Sebelum v2.1.257, pemeriksaan hanya melihat ejaan jalur, bukan di mana symlink mengarah.

**Yang harus dilakukan:**

* Pindahkan file yang direferensikan ke dalam direktori plugin dan arahkan jalur ke sana dengan jalur relatif `./`
* Jika jalur adalah symlink ke file di luar plugin, ganti symlink dengan salinan file
* Jika pesan mengatakan jalur berisi backslash, tulis jalur dengan garis miring maju, misalnya `./commands/deploy.md`
* Untuk berbagi file dengan plugin lain di marketplace yang sama, tautkan dengan symlink di dalam direktori plugin, mengikuti [aturan symlink](/docs/id/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)

<h3 id="path-could-not-be-checked">
  Path could not be checked
</h3>

Claude Code menanyakan sistem operasi apakah jalur plugin ada dan mendapat kesalahan selain "tidak ditemukan", jadi tidak memuat apa yang dinamai jalur. Berapa banyak plugin yang memuat tergantung pada jalur mana yang gagal:

* Salah satu dari [lokasi komponen default](/docs/id/plugins/manifest-reference#standard-layout) plugin, seperti folder `skills/`, file `monitors/monitors.json`, atau [`SKILL.md` di root plugin](/docs/id/plugins/components#skills): komponen lain plugin masih memuat
* Direktori plugin itu sendiri: tidak ada yang memuat dari plugin itu

Anda tidak melihat kesalahan ini untuk jalur yang tidak ada sama sekali. Di `/plugin`, kesalahan muncul di bawah plugin dan menamai jalur dan kode yang dikembalikan sistem operasi:

```text theme={null}
skills path could not be checked: /home/user/my-plugin/skills (ELOOP)
```

Di `claude plugin list`, kesalahan yang sama berbunyi `Path not found: /home/user/my-plugin/skills (skills, ELOOP)`.

Penyebab yang menghasilkan kesalahan ini termasuk:

* `ELOOP`: symlink dalam jalur menunjuk pada dirinya sendiri atau membentuk loop
* `EIO` atau `ESTALE`: jalur berada di mount jaringan yang rusak atau basi
* `EACCES`: salah satu direktori di atas jalur menolak Anda izin untuk melintasinya

**Yang harus dilakukan:**

* Ganti symlink yang menunjuk pada dirinya sendiri dengan folder nyata, atau hapus
* Jika jalur berada di mount jaringan, pasang kembali bagian
* Jika kodenya adalah `EACCES`, pulihkan izin eksekusi Anda di direktori di atas jalur
* Jalankan `/reload-plugins` setelah memperbaiki jalur, atau mulai ulang Claude Code, untuk memuat plugin atau komponen

Sebelum v2.1.265, Claude Code memperlakukan folder komponen default yang tidak dapat diperiksa sebagai tidak ada dan memuat plugin tanpa komponen itu, tanpa kesalahan.

<h3 id="marketplace-entry-path-does-not-stay-inside-the-marketplace-directory">
  Marketplace entry path does not stay inside the marketplace directory
</h3>

[Entri marketplace](/docs/id/plugins/marketplace-reference#plugin-entries) plugin mendeklarasikan jalur sumber yang Claude Code tidak dapat diselesaikan ke lokasi di dalam direktori marketplace itu sendiri, jadi plugin tidak memasang atau memuat. Penolakan mencakup:

* Jalur entri yang absolut, memanjat keluar dari marketplace dengan `..`, atau dieja seperti jalur jaringan
* Di macOS dan Linux, jalur entri yang berisi backslash di mana pun setelah `./` terkemuka
* Entri di marketplace yang diambil dari sumber jarak jauh, seperti git atau URL, yang mencapai targetnya melalui symlink yang diselesaikan di luar direktori marketplace
* Entri relatif di marketplace yang ditambahkan dari URL langsung ke `marketplace.json`: Claude Code hanya mengunduh file itu, jadi tidak ada file plugin lokal untuk jalur yang dinamai. Lihat [Plugins with relative paths fail in URL-based marketplaces](/docs/id/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)

`claude plugin install` melaporkan penolakan seperti ini:

```text theme={null}
Cannot install my-plugin@my-marketplace: its marketplace entry path does not stay inside the marketplace directory (an absolute, climbing, network-shaped, backslash-containing or link-traversing entry, an entry of a fetched marketplace that resolves or opens outside its tree — or a relative entry in a url-catalog marketplace, which has no local directory)
```

Ketika entri plugin yang sudah terpasang gagal pemeriksaan yang sama, `claude plugin list` menampilkan plugin sebagai `failed to load` dengan:

```text theme={null}
Plugin source path refused: ./my-plugin does not stay inside its marketplace directory. Check that the marketplace entry has a plain relative path.
```

**Yang harus dilakukan:**

* Jika Anda memelihara marketplace, tulis `source` entri sebagai jalur relatif biasa dengan garis miring maju, seperti `./plugins/my-plugin`, dan jaga agar symlink apa pun yang dilintasinya menunjuk ke dalam direktori marketplace
* Jika Anda menambahkan marketplace dari URL langsung, entri relatif tidak dapat diselesaikan. Minta penulis marketplace untuk menggunakan [sumber plugin lain](/docs/id/plugins/marketplace-reference#plugin-sources), atau tambahkan marketplace dari repositori git-nya sebagai gantinya

<h3 id="failed-to-load-marketplace-configuration">
  Failed to load marketplace configuration
</h3>

Claude Code menyimpan marketplace plugin yang telah Anda tambahkan dalam file registri di `~/.claude/plugins/known_marketplaces.json`. Perintah plugin yang memerlukan registri, seperti `claude plugin install`, gagal dengan salah satu dari dua pesan ketika Claude Code tidak dapat menggunakan file:

* `Failed to load marketplace configuration`: file bukan JSON yang valid, atau tidak dapat dibaca. File kosong gagal dengan cara ini juga.
* `Marketplace configuration file is corrupted`: file adalah JSON yang valid tetapi isinya tidak cocok dengan skema registri.

File yang hilang bukan kegagalan: Claude Code memperlakukannya sebagai registri tanpa marketplace.

Dengan file kosong, `claude plugin install` melaporkan:

```text theme={null}
✘ Failed to install plugin "my-plugin": Failed to load marketplace configuration: JSON Parse error: Unexpected EOF
```

Sebelum v2.1.246, `claude plugin install` tidak melaporkan kegagalan ini.

**Yang harus dilakukan:**

* Buka `~/.claude/plugins/known_marketplaces.json` dan perbaiki JSON, atau perbaiki entri yang pesan namakan sebagai tidak cocok dengan skema registri
* Jika Anda tidak dapat memperbaikinya, hapus file atau ganti isinya dengan `{}`, kemudian tambahkan kembali setiap marketplace dengan `claude plugin marketplace add <source>`. Claude Code mendaftarkan kembali marketplace yang dideklarasikan pengaturan pengguna atau terkelola Anda dalam [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces) lain kali Anda memulainya di folder yang telah Anda percayai.

<h3 id="plugin-is-required-by-your-organization">
  Plugin is required by your organization
</h3>

Anda menjalankan `claude plugin disable`, atau menggunakan tab **Installed** `/plugin`, untuk mematikan [plugin yang disinkronkan dari claude.ai](/docs/id/plugins/loading#synced-plugins) yang organisasi Anda tandai sebagai diperlukan:

```text theme={null}
Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.
```

Claude Code tidak menyimpan apa pun dan plugin tetap diaktifkan.

Ketika Anda mencoba menonaktifkan plugin yang diperlukan oleh plugin yang diperlukan, Claude Code menolak dengan cara yang sama, dengan pesan yang menamai plugin yang diperlukan yang membutuhkannya.

**Yang harus dilakukan:**

* Tanyakan admin organisasi claude.ai Anda untuk mengubah status yang diperlukan plugin di claude.ai

<h2 id="tool-errors">
  Kesalahan alat
</h2>

Kesalahan ini berasal dari alat bawaan Claude. Claude memperbaiki sebagian besar kesalahan alat secara otomatis. Ketika salah satu memerlukan perubahan dari Anda, daftar **What to do** kesalahan tersebut mengatakan apa yang harus diubah.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent would be spawned with zero tools
</h3>

Setiap entri dalam [`tools` list](/docs/id/sub-agents#supported-frontmatter-fields) subagent gagal cocok dengan alat yang dapat digunakan, jadi Claude Code menolak untuk meluncurkan subagent: tanpa alat, itu tidak bisa bertindak. Pesan mengelompokkan entri Anda berdasarkan apa yang salah:

* **Unrecognized**: entri tidak cocok dengan nama alat apa pun, biasanya kesalahan ketik seperti `Grpe` untuk `Grep`.
* **Not available to subagents**: entri menamai alat nyata yang [subagent tidak bisa gunakan](/docs/id/sub-agents#available-tools). Subagent latar belakang menyimpan set alat bawaan yang lebih kecil, jadi entri yang hanya dapat digunakan subagent foreground mendarat di sini ketika subagent akan berjalan di latar belakang, yang merupakan default. Jika Anda mencantumkan `Agent`, pesan melaporkannya di bawah grup berikutnya.
* **Matched no tools in this session**: entri valid tetapi tidak ada alat dalam sesi saat ini yang cocok sekarang, seperti `mcp__github__*` tanpa server MCP GitHub yang terhubung, atau `Agent` untuk subagent pada [batas kedalaman](/docs/id/sub-agents#let-subagents-spawn-their-own-subagents).

Menghilangkan bidang `tools` tidak pernah memicu penolakan ini. Jika Anda membiarkan daftar `tools` kosong, atau `disallowedTools` menghapus setiap entri di dalamnya, Claude Code juga melewati penolakan dan meluncurkan subagent tanpa alat.

Sebelum v2.1.208, subagent diluncurkan tanpa alat dan dapat mengembalikan hasil yang kosong atau membingungkan.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**What to do:**

* Perbaiki setiap entri yang dinamai kesalahan terhadap [alat yang tersedia untuk subagent](/docs/id/sub-agents#available-tools)
* Hapus entri untuk alat yang tidak dimiliki sesi, seperti alat MCP dari server yang tidak terhubung
* Untuk alat yang [subagent latar belakang lepaskan](/docs/id/sub-agents#available-tools), seperti `CronCreate`, hapus entri. Untuk menyimpan alat, [matikan mode fork](/docs/id/sub-agents#turn-fork-mode-on-or-off) dan minta Claude menjalankan subagent di foreground
* Hapus bidang `tools` alih-alih mencantumkan alat untuk memberikan subagent setiap [alat yang tersedia untuk subagent](/docs/id/sub-agents#available-tools)
* Untuk daftar `tools` yang hanya berisi `Agent`, naikkan [batas kedalaman](/docs/id/sub-agents#let-subagents-spawn-their-own-subagents) atau berikan agen setidaknya satu alat lain: Claude Code menahan `Agent` pada batas itu, jadi daftar tanpa yang lain di dalamnya diselesaikan ke tidak ada alat

<h3 id="file-is-covered-by-a-read-deny-rule">
  File is covered by a Read deny rule
</h3>

Alat Edit atau Write dipanggil pada jalur yang cocok dengan [aturan penolakan `Read`](/docs/id/permissions#read-and-edit), termasuk membuat file baru di jalur itu. Kedua alat mengubah konten yang harus dapat dibaca Claude, jadi Claude Code menolak panggilan sebelum akses file apa pun. NotebookEdit tidak tercakup oleh aturan penolakan `Read`. Sebelum v2.1.228, aturan memblokir alat Edit saja, dan sebelum v2.1.208, hanya aturan penolakan `Edit` yang memblokir edit.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

Ketika Claude Code menolak alat Write, pesan berakhir `and cannot be written` sebagai gantinya.

**What to do:**

* Jika Claude harus dapat mengubah file, hapus atau persempit aturan penolakan `Read` di `/permissions` atau di [settings](/docs/id/settings-reference#permission-settings)
* Jika file harus tetap tidak tersentuh, simpan aturan dan tambahkan aturan penolakan `Edit` untuk jalur yang sama untuk memblokir alat NotebookEdit juga

<h3 id="subagent-type-is-required">
  subagent\_type is required
</h3>

```text theme={null}
subagent_type is required: the general-purpose agent is not available in this session. Available agents: ...
```

Claude memanggil [Agent tool](/docs/id/tools-reference#agent-tool-behavior) tanpa `subagent_type`, dan sesi ini tidak memiliki [subagent tujuan umum](/docs/id/sub-agents#built-in-subagents) untuk kembali. Itu adalah kasus dalam dua pengaturan:

* [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/id/env-vars) diatur dalam mode non-interaktif, yang menghapus setiap subagent bawaan
* Agen thread utama sesi memiliki [`tools: Agent(...)` allowlist](/docs/id/sub-agents#restrict-which-subagents-can-be-spawned) yang meninggalkan `general-purpose`

**What to do:**

* Biasanya tidak ada: pesan mencantumkan subagent yang dimiliki sesi, jadi Claude dapat mencoba lagi dengan salah satunya
* Jika Claude terus gagal, tambahkan `general-purpose` ke allowlist `tools: Agent(...)`, atau batalkan `CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS`

Sebelum v2.1.235, panggilan yang sama gagal dengan `Agent type 'general-purpose' not found`.

<h3 id="memory-index-is-over-its-read-limit">
  Memory index is over its read limit
</h3>

Claude menulis ke indeks [memori otomatis](/docs/id/memory#auto-memory) `MEMORY.md` dan meninggalkannya di atas salah satu batas bacanya: 200 baris atau 25KB. Penulisan berhasil, tetapi hanya 200 baris pertama atau 25KB, mana pun yang lebih dulu, dimuat di awal sesi, jadi semuanya melampaui batas dijatuhkan setiap kali indeks dibaca. Sebelum v2.1.210, indeks yang melampaui batas secara diam-diam dipotong pada beban berikutnya tanpa sinyal waktu penulisan.

```text theme={null}
Error: this write left the memory index at MEMORY.md at 214 lines, over its 200-line read limit. The write succeeded, but everything past the limit is silently dropped each time the index is loaded — entries at the end are already invisible to readers. Rewrite it to under 140 lines now: keep one line per entry, move detail into topic files, and merge or drop stale entries.
```

Hanya konten yang dimuat yang dihitung terhadap batas. Frontmatter YAML dan komentar HTML tingkat blok dilepas sebelum indeks dimuat, jadi mereka dikecualikan dari pengukuran. Sebelum v2.1.211, Claude Code mengukur file mentah, dan frontmatter atau komentar dapat memicu kesalahan ini bahkan ketika konten yang dimuat cocok.

Claude Code memberikan kesalahan kepada Claude setelah penulisan daripada mencetaknya sebagai spanduk di terminal Anda, jadi Anda mungkin hanya memperhatikannya dalam transkrip.

Ketika penulisan Claude membawa file dekat dengan batas tanpa melewatinya, Claude Code mengembalikan pengingat yang lebih ringan untuk mengompak indeks alih-alih kesalahan ini.

**What to do:**

* Biarkan Claude menulis ulang `MEMORY.md`, atau minta: simpan satu baris per entri, pindahkan detail ke file topik, dan gabungkan atau lepaskan entri basi
* Untuk memangkas indeks sendiri, lihat [Audit and edit your memory](/docs/id/memory#audit-and-edit-your-memory)

<h3 id="pkill-pattern-matches-the-claude-code-process">
  pkill pattern matches the Claude Code process
</h3>

Perintah `pkill` dalam panggilan alat Bash menggunakan pola, biasanya dengan `-f`, yang cocok dengan proses Claude Code itu sendiri, jadi Claude Code menolak perintah alih-alih membiarkannya mengakhiri sesi. Claude Code menguji pola dengan `pgrep` sebelum menjalankan `pkill` dan menolak ketika ID proses miliknya ada dalam hasil. Pemeriksaan berjalan di Linux saja; di macOS, `pkill` berjalan tanpa modifikasi. Sebelum v2.1.214, perintah berjalan, dan pola yang cocok membunuh sesi Claude Code di tengah-putaran.

```text theme={null}
pkill: refusing to run — this pattern matches the Claude CLI process (PID 12345). Narrow the pattern, or target your own children with `pkill -P $$ ...`.
```

Penolakan muncul dalam hasil alat Bash daripada sebagai spanduk di terminal Anda, dan Claude biasanya menyesuaikan perintah dengan sendirinya.

**What to do:**

* Persempit pola sehingga hanya cocok dengan proses yang dimaksudkan, misalnya jalur lengkap biner target daripada substring pendek
* Untuk menghentikan proses yang dimulai oleh shell saat ini, gunakan `pkill -P $$` dengan pola, yang membatasi kecocokan ke proses anak shell itu sendiri

<h3 id="failed-to-write-to-a-teammate-inbox">
  Failed to write to a teammate's inbox
</h3>

Claude Code tidak dapat menulis pesan ke file kotak surat rekan kerja di bawah `~/.claude/teams/{team-name}/inboxes/`, jadi penerima tidak menerima apa pun. Penulisan gagal ketika Claude Code tidak dapat membuat atau memperbarui file, misalnya karena disk penuh, direktori tidak dapat ditulis, atau agen lain menahan kunci kotak surat terlalu lama. Sebelum v2.1.224, Claude Code melaporkan pesan sebagai terkirim bahkan ketika penulisan gagal.

Kesalahan muncul dalam hasil alat agen pengirim daripada sebagai spanduk di terminal Anda, dan teksnya memberitahu Claude untuk mencoba lagi:

```text theme={null}
Failed to write to researcher's inbox — nothing was sent. Try again, or message the lead.
```

Pesan protokol [tim agen](/docs/id/agent-teams) terstruktur gagal dengan cara yang sama, dan kesalahan menamai pesan yang tidak terkirim: ketika Claude Code tidak dapat menulis persetujuan rencana, penolakan rencana, permintaan shutdown, atau penolakan shutdown, kesalahan berbunyi `Failed to write the <message> to <name>'s inbox — nothing was sent`. `plan approval` dalam daftar itu adalah keputusan pemimpin yang menyetujui rencana rekan kerja; pengajuan rencana rekan kerja adalah pesan `plan approval request` yang terpisah. Pesan itu dan dua pesan protokol lainnya membawa teks pesan mereka sendiri dan konsekuensi:

* `Failed to write the plan approval request to the lead's inbox — plan not submitted; try again`: rencana rekan kerja tidak pernah mencapai pemimpin, dan rekan kerja tetap dalam mode rencana sampai pengajuan ulang berhasil
* `The permission request could not be delivered to the team lead (mailbox write failed)`: permintaan izin rekan kerja tidak pernah mencapai pemimpin, jadi tidak ada yang menyetujui panggilan alat
* `The confirmation could not be written to team-lead's inbox.`: persetujuan shutdown itu sendiri berlaku dan rekan kerja keluar; hanya konfirmasi kepada pemimpin yang hilang

Ketika Anda mengirim pesan ke rekan kerja sendiri, mengetik `@name` diikuti oleh pesan dalam sesi pemimpin, kegagalan yang sama muncul sebagai notifikasi, `Couldn't write to @name's inbox — message not sent. Try again.`, dan Claude Code menyimpan teks Anda di kotak prompt sehingga Anda dapat mengirimnya lagi.

**What to do:**

* Minta pengirim untuk mengirim ulang pesan; pertentangan untuk kunci kotak surat bersifat sementara dan jelas saat mencoba lagi
* Periksa ruang disk gratis, dan periksa bahwa `~/.claude/teams` dan file di bawahnya dapat ditulis oleh pengguna Anda

<h3 id="teammate-agent-definition-not-restored">
  Teammate's agent definition was not restored
</h3>

Claude mengirim pesan ke rekan kerja [tim agen](/docs/id/agent-teams) yang dihentikan, dan Claude Code membawanya kembali tanpa menerapkan kembali [definisi subagent](/docs/id/agent-teams#use-subagent-definitions-for-teammates) yang dihasilkannya, karena file definisinya berasal dari folder tanpa kepercayaan yang disimpan. Pemberitahuan mengikuti laporan resume dalam hasil alat agen pengirim:

```text wrap theme={null}
Its agent definition was not restored: the folder its definition file came from is not trusted (source: projectSettings), so the teammate is running with the team-essential tools and no custom instructions. To restore it, the user needs to run Claude Code in that folder once and accept the trust dialog (the --debug log names the folder); do not change trust settings on the user's behalf.
```

Pemeriksaan berlaku untuk definisi di direktori `.claude/agents/` dari proyek atau direktori `--add-dir`, dan menerima dialog kepercayaan untuk folder induk tidak memuaskannya.

**What to do:**

* Jalankan `claude` di folder yang dinamai [debug log](/docs/id/debug-your-config) dan terima dialog kepercayaan. Definisi diterapkan kembali saat Claude Code membawa rekan kerja kembali berikutnya; Anda tidak perlu memulai ulang sesi pemimpin
* Atau atur entri `hasTrustDialogAccepted` ke `true` di `~/.claude.json`, menggunakan kunci `projects["<path>"]` yang tepat yang dicetak log debug

<h3 id="message-too-large-for-cross-session-delivery">
  Message too large for cross-session delivery
</h3>

Pesan [lintas sesi](/docs/id/cross-session-messaging) Claude ke sesi lain Anda di mesin ini terlalu panjang untuk dikirim. Claude Code menolaknya, dan sesi penerima tidak mendapat apa pun. Penolakan muncul dalam hasil alat sesi pengirim, bukan sebagai spanduk di terminal Anda. Ini menamai kedua ukuran dan cara membuat pesan cocok:

```text wrap theme={null}
Failed to send to api-worker: Message too large for cross-session delivery: the serialized message is 1,203,844 characters and the limit is 1,048,576. Shorten the message text — put bulk content in a file the recipient can read rather than in the message — or split it into smaller messages.
```

Mengirim ulang teks yang sama gagal dengan cara yang sama.

**What to do:**

* Minta Claude untuk merangkum pesan, atau untuk menempatkan konten massal dalam file dan mengirim jalur file
* Minta Claude untuk membagi konten di beberapa pesan yang lebih pendek

Sebelum v2.1.235, Claude Code melaporkan pesan yang terlalu besar sebagai terkirim. Sesi penerima menjatuhkannya tanpa dibaca.

<h3 id="too-many-messages-to-this-session-just-now">
  Too many messages to this session just now
</h3>

Claude mengirim ledakan cepat [pesan lintas sesi](/docs/id/cross-session-messaging) ke salah satu sesi Anda di mesin ini, dan ledakan mencapai apa yang diterima kotak surat sesi itu. Claude Code menolak pengiriman berikutnya, dan sesi penerima tidak mendapat apa pun darinya. Penolakan muncul dalam hasil alat sesi pengirim, bukan sebagai spanduk di terminal Anda:

```text wrap theme={null}
Failed to send to api-worker: Too many messages to this session just now: 30 were sent recently and more would be dropped by its rate limit, so this one was not sent. Batch what remains into one message, or wait a little before sending more.
```

**What to do:**

* Biasanya tidak ada: Claude mengelompokkan konten yang tersisa menjadi satu pesan, atau menunggu sebelum mengirim lebih banyak
* Jika Anda sendiri yang memicu ledakan, minta Claude untuk menggabungkan apa yang tersisa menjadi satu pesan

Sebelum v2.1.236, Claude Code melaporkan pengiriman ini sebagai terkirim. Sesi penerima menjatuhkannya tanpa dibaca.

<h3 id="refusing-to-send-a-cross-session-message">
  Refusing to send a cross-session message
</h3>

Sebelum Claude Code menulis pesan [lintas sesi](/docs/id/cross-session-messaging) ke sesi lain Anda di mesin ini, itu memeriksa bahwa soket kotak surat sesi target adalah titik akhir yang ditujukan pesan. Ketika pemeriksaan gagal, Claude Code menolak pengiriman dalam sesi pengirim, dan sesi target tidak menerima apa pun. Untuk pesan yang dikirim Claude, penolakan muncul dalam hasil alat sesi pengirim:

```text theme={null}
Failed to send to api-worker: Refusing to send: reply target is a symlink
```

Teks setelah `Refusing to send:` menamai pemeriksaan yang gagal:

* `reply target is a symlink`: tautan simbolis duduk di jalur soket sesi target. Claude Code tidak mengirimkan melaluinya, karena tautan di sana dapat mengarahkan pesan ke titik akhir yang tidak dibuat sesi target.
* `cannot vet reply target`: Claude Code tidak dapat memeriksa jalur target sama sekali, misalnya karena membacanya gagal dengan kesalahan izin.
* `connected endpoint is not the expected process`: proses yang menahan soket bukan sesi yang ditujukan pesan, jadi alamat sudah usang atau proses lain mengganti soket.
* `connected endpoint identity could not be read`: Claude Code terhubung tetapi tidak dapat membaca proses mana yang menahan ujung lain, jadi tidak dapat mengkonfirmasi target. Ini bisa bersifat sementara.
* `connected endpoint is not owned by this user`: proses yang menahan soket berjalan sebagai akun pengguna yang berbeda, jadi bukan salah satu sesi Anda.
* `connected endpoint owner could not be read`: Claude Code terhubung tetapi tidak dapat membaca akun pengguna mana yang memiliki ujung lain, jadi tidak dapat mengkonfirmasi titik akhir adalah milik Anda.
* `connected endpoint is a different process with the expected pid`: ID proses cocok dengan yang ditujukan pesan, tetapi Claude Code tidak dapat mengkonfirmasi itu adalah proses yang sama. Biasanya sesi itu keluar dan sistem operasi menggunakan kembali ID prosesnya, jadi alamat sudah usang.

**What to do:**

* Biasanya tidak ada: pemeriksaan menjaga pesan agar tidak mencapai titik akhir selain sesi yang ditujukan, dan tidak ada yang dikirim
* Minta Claude untuk mencantumkan sesi Anda lagi dan mengirim ulang; penolakan yang disebabkan oleh alamat yang sudah usang jelas setelah Claude mengirim ke yang saat ini
* Jika `reply target is a symlink` berulang untuk satu sesi, periksa apa yang membuat tautan di jalur soket sesi itu, ditampilkan di `/status` di bawah `Peer address`
* Untuk `connected endpoint identity could not be read`, kirim ulang; kondisi dapat bersifat sementara
* Jika `connected endpoint is not owned by this user` muncul di mesin bersama, sesi di alamat itu berjalan di bawah akun pengguna lain, jadi Claude tidak dapat mengirim pesannya dari milik Anda

Sebelum v2.1.248, Claude Code tidak memeriksa pengguna pemilik titik akhir atau waktu mulai proses, jadi penolakan yang menamai pemeriksaan itu tidak muncul di versi sebelumnya.

<h3 id="refusing-after-a-symlink-changed">
  Refusing to read, write, or search a path
</h3>

Claude Code memeriksa [aturan izin](/docs/id/permissions#read-and-edit) jalur file, kemudian mengkonfirmasi resolusi itu lagi ketika alat membuka file atau memulai pencarian. Ketika tidak dapat mengkonfirmasi bahwa jalur masih mengarah ke lokasi yang disetujui pemeriksaan, Claude Code menolak operasi alih-alih mengikutinya. Penolakan muncul dalam hasil alat:

```text wrap theme={null}
Refusing to read /path/to/file: its symlink resolution changed after permission was checked (a link on the way now leads somewhere the check did not see). If a link in the working directory is being rewritten concurrently, stop that and retry.
```

Setiap penolakan menamai alasannya:

* `its symlink resolution changed after permission was checked`: tautan simbolis di sepanjang jalur, atau di akar pencarian Grep atau Glob, diganti antara pemeriksaan izin dan operasi. Dalam penolakan baca, frasa dalam tanda kurung menamai perbandingan mana yang gagal.
* `its parent-directory symlink resolution changed after permission was checked`: direktori yang dilalui jalur penulisan tidak lagi diselesaikan ke lokasi yang disetujui
* `it is a symbolic link. Write to the link's target path instead`: tautan simbolis duduk di lokasi penulisan yang disetujui itu sendiri, misalnya `CLAUDE.md` yang merupakan tautan simbolis ke `AGENTS.md`; pesan mengarahkan Claude ke target tautan
* `Refusing to write through symlink: <path>. Resolve the symlink and pass the real target path explicitly.`: kondisi yang sama ditangkap ketika penulis lain membuka file, seperti penulisan ke `.mcp.json` yang ditautkan
* `Refusing to write into symlinked directory: <path>`: direktori yang menyimpan file itu sendiri adalah tautan simbolis, misalnya direktori `.claude/` proyek yang ditautkan ke lokasi lain
* `a path one of its Read deny rules is written through changed while the search was being prepared. Retry.`: aturan penolakan `Read` untuk pencarian menamai jalur yang melewati tautan simbolis, dan tautan itu berubah saat Claude Code menyiapkan pencarian
* `it could not be opened (EACCES) — it is unreadable, or is being replaced concurrently.`: akar pencarian ada tetapi tidak dapat dibuka; kode dalam tanda kurung adalah kesalahan sistem operasi
* `its permission check expired before it ran (too many concurrent file operations). Retry.`: Claude Code mengeluarkan catatan persetujuan di bawah banyak operasi file simultan sebelum alat menggunakannya; mencoba lagi menjalankan pemeriksaan izin segar
* `ripgrep was found only by name on PATH, and a search outside the working directory cannot apply your Read deny rules in that configuration`: Claude Code tidak dapat menyelesaikan biner `rg` ke jalur absolut, jadi menolak pencarian di luar direktori kerja daripada menjalankan pencarian yang tidak dicakup aturan penolakan Anda

**What to do:**

* Biasanya tidak ada: penolakan mencapai Claude sebagai hasil alat, dan operasi yang ditolak tidak berjalan
* Jika penolakan tautan simbolis berulang pada satu jalur, temukan apa yang terus menulis ulang tautan di sana, seperti alat build atau file watcher, atau minta Claude menggunakan jalur yang diselesaikan file alih-alih yang ditautkan
* Jika penolakan ini muncul untuk setiap file saat Claude Code berjalan di Windows di dalam AppContainer atau sandbox token terbatas, tingkatkan ke v2.1.265 atau lebih baru
* Jika penolakan baca muncul di macOS untuk file yang tidak ada yang menulis ulang, seperti tangkapan layar yang diseret ke prompt, tingkatkan ke v2.1.273 atau lebih baru
* Untuk penolakan ripgrep, instal ripgrep dengan manajer paket Anda sehingga `rg` diselesaikan ke jalur absolut di `PATH`, atau simpan pencarian di bawah direktori kerja

Sebelum v2.1.251, Claude Code memeriksa kembali resolusi jalur hanya untuk penulisan file, jadi tautan yang diganti setelah pemeriksaan izin dapat mengarahkan baca atau pencarian ke lokasi berbeda tanpa pesan. Dari ini, hanya penolakan penulisan direktori induk, melalui-tautan, dan direktori-tertaut muncul di versi sebelumnya.

<h3 id="task-output-swap-refused">
  Task output swap refused
</h3>

Claude Code menyimpan output setiap perintah Bash ke file di bawah direktori tempnya. Setiap kali membuka salah satu file ini, itu memeriksa bahwa jalur masih mengarah ke file yang dibuat, tanpa tautan simbolis, tautan keras ekstra, atau direktori yang dipindahkan mengarahkannya. Pesan ini berarti pemeriksaan itu gagal, jadi Claude Code menolak operasi daripada menulis atau membaca output melalui jalur itu. Pesan muncul dalam hasil alat Bash:

```text wrap theme={null}
task output swap refused (tasks dir moved or linked): /private/tmp/claude-501/-Users-you-my-project/1f0e62dc-4b0a-4f5e-9c2d-8a7b6c5d4e3f/tasks/b7k2f9m3q.output. To recover: restart Claude Code with CLAUDE_CODE_TMPDIR set to a fresh directory; or, if /private/tmp/claude-501/-Users-you-my-project is a stray directory or a symbolic link that should not be there, remove that entry itself (not what it points to) and restart.
```

Teks dalam tanda kurung menamai pemeriksaan yang gagal. Alasan seperti `output symlink was re-pointed`, `output file identity changed`, dan `not a regular file` semuanya melaporkan kondisi yang sama: sesuatu di atau sepanjang jalur output tidak lagi file yang dibuat Claude Code. Hanya beberapa alasan yang membawa kalimat `To recover:`.

Jika pemeriksaan gagal saat perintah masih berjalan, Claude Code menghentikan perintah, dan hasilnya melaporkan:

```text theme={null}
Command killed: its output file was replaced or could no longer be verified
```

**What to do:**

* Tingkatkan ke v2.1.260 atau lebih baru. Versi sebelumnya kadang-kadang menunjukkan pesan ini ketika tidak ada tautan atau direktori yang dipindahkan
* Mulai ulang Claude Code dengan [`CLAUDE_CODE_TMPDIR`](/docs/id/env-vars) diatur ke direktori segar
* Atau periksa direktori proyek Anda di bawah direktori temp Claude Code, `/private/tmp/claude-501/-Users-you-my-project` dalam pesan contoh. Jika jalur itu adalah tautan simbolis, atau direktori yang tidak seharusnya ada, hapus tautan atau direktori itu sendiri daripada target tautan, dan mulai ulang Claude Code
* Jika penolakan berulang, proses mengganti, menautkan, atau menghapus entri di bawah direktori temp Claude Code saat sesi berjalan. Atur [`CLAUDE_CODE_TMPDIR`](/docs/id/env-vars) ke direktori yang tidak dikelola apa pun dan mulai ulang

<h3 id="the-source-file-is-not-valid-utf-8-text">
  The source file is not valid UTF-8 text
</h3>

Claude mencoba menerbitkan [artefak](/docs/id/artifacts) dari file yang byte-nya tidak didekodekan sebagai teks, atau yang teksnya sudah berisi karakter pengganti `U+FFFD`, jadi Claude Code menolak publikasi sebelum mengunggah apa pun. Pesan muncul dalam hasil alat Artefak dan menamai posisi pertama untuk diperbaiki:

```text wrap theme={null}
file_path: the source file is not valid UTF-8 text (first invalid byte at line 12, column 40). It may be saved in another encoding or contain binary data. Rewrite it as UTF-8, then publish again. Nothing was published.

file_path: the source file has the replacement character U+FFFD at line 12, column 40, usually left where an earlier edit or paste lost a character. Replace it with the intended text (in HTML, write an intended U+FFFD as &#xFFFD;), then publish again. Nothing was published.
```

Claude Code mendekodekan file sebagai UTF-8, atau sebagai UTF-16 ketika dimulai dengan tanda urutan byte UTF-16 little-endian. Ketika file UTF-16 seperti itu tidak didekodekan, pesan pertama menamai `UTF-16` dan masih memberitahu Anda untuk menulis ulang file sebagai UTF-8. Ketika lebih banyak posisi mengikuti yang dinamai, pesan menambahkan hitungan seperti `(+2 more)` setelah posisi.

**What to do:**

* Biasanya tidak ada: Claude menulis ulang file dan menerbitkan lagi
* Jika file adalah yang Anda tulis atau ekspor, simpan lagi sebagai UTF-8, dan ganti setiap `U+FFFD` dengan karakter yang hilang oleh edit, paste, atau konversi sebelumnya
* Untuk menampilkan `U+FFFD` yang disengaja di halaman, tuliskan sebagai `&#xFFFD;` dalam HTML alih-alih karakter literal

Sebelum v2.1.267, Claude Code mengunggah file seperti itu tanpa memeriksanya, dan server menolak publikasi sebagai gantinya.

<h3 id="reading-a-local-file-from-outside-the-connected-folders">
  Reading a local file from outside the connected folders in a Cowork session
</h3>

Dalam sesi [Cowork](https://claude.com/docs/cowork/overview) yang berjalan di mesin Anda di aplikasi Claude Desktop, Claude menamai file lokal untuk [artefak](/docs/id/artifacts). Claude Code tidak dapat mengkonfirmasi file adalah file biasa di dalam folder yang terhubung sesi: jalur duduk di luar folder itu, melewati tautan simbolis, atau dieja dengan cara yang dapat menamai file berbeda dari yang terlihat. Membaca file seperti itu memerlukan persetujuan Anda, dan dalam sesi yang tidak dapat menampilkan kartu persetujuan, seperti yang ditetapkan untuk melewati semua persetujuan, Claude Code menolak pembacaan.

Penolakan muncul dalam hasil alat Artefak; ketika file tidak dapat diperiksa sama sekali, itu menamai kegagalan itu sebagai gantinya:

```text wrap theme={null}
Reading a local file from outside this session's connected folders, or through a link, needs the approval card, and no one can answer it in this Cowork session. Use a plain file inside the connected folders; do not retry this file in this session.

cannot read file_path (ENOENT) — the file could not be examined, and no one can answer the approval card in this Cowork session. Check that the file exists as a plain file inside the connected folders, then retry with that path.
```

**What to do:**

* Biasanya tidak ada: pesan memberitahu Claude untuk menggunakan file biasa di dalam folder yang terhubung sebagai gantinya
* Untuk menempatkan file yang tepat itu dalam artefak, salin ke salah satu folder yang terhubung sesi sebagai file biasa, bukan tautan simbolis, dan tanyakan lagi

<h3 id="webfetch-cannot-fetch-localhost">
  WebFetch cannot fetch localhost
</h3>

Claude memanggil [WebFetch](/docs/id/tools-reference#webfetch-tool-behavior) dengan URL yang nama hostnya tidak memiliki titik, seperti `http://localhost:3000` atau nama intranet telanjang seperti `http://wiki/`. WebFetch menolak URL ini sebelum membuat permintaan apa pun:

```text wrap theme={null}
WebFetch cannot fetch localhost or other hostnames without a dot. To reach a local server, use Bash with curl instead.
```

**What to do:**

* Biasanya tidak ada: pesan menunjukkan Claude ke `curl` melalui alat Bash, yang dapat menjangkau server lokal dan intranet

Sebelum v2.1.268, WebFetch melaporkan URL ini dengan kesalahan `Invalid URL` generik.

<h2 id="background-session-errors">
  Kesalahan sesi latar belakang
</h2>

[Sesi latar belakang](/docs/id/agent-view) berjalan tanpa terminal interaktif mereka sendiri, jadi perintah yang membutuhkan satu berperilaku berbeda di sana. Pesan-pesan ini muncul dalam transkrip sesi latar belakang, di terminal yang terhubung ke satu, di sesi atau shell tempat Anda mengirim, atau, untuk [entri worktree-guard](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) di bawah, di sesi apa pun yang terisolasi dalam worktree atau menjalankan subagent yang terisolasi worktree; di mana pesan spesifik untuk satu permukaan, entrinya mengatakan demikian.

<h3 id="commands-refused-in-a-background-session">
  Perintah ditolak dalam sesi latar belakang
</h3>

Perintah yang membuka dialog interaktif tidak dapat melakukannya saat tidak ada terminal yang terhubung ke sesi latar belakang. `/install-github-app`, daftar pengaturan `/mcp`, dan tindakan autentikasi dalam menu server MCP merespons dengan pesan, dan sesi muncul di bawah **Needs input** dalam [tampilan agen](/docs/id/agent-view) sehingga Anda dapat menemukannya, melampirkan, dan menjalankan perintah lagi. Saat terminal terhubung, perintah-perintah ini berfungsi secara normal.

Sebelum v2.1.216, sesi tidak muncul di bawah **Needs input** setelah salah satu penolakan ini. Di v2.1.213 hingga v2.1.215, perintah masih berfungsi saat terminal terhubung, dan pesan penolakan memberi tahu Anda untuk melampirkan dan menjalankan perintah lagi. Dari v2.1.208 hingga v2.1.212, Claude Code menolaknya bahkan saat terminal terhubung, dengan pesan seperti `Can't open MCP settings in a background session`; pada versi tersebut, jalankan perintah dari sesi `claude` biasa sebagai gantinya, atau tingkatkan. Sebelum v2.1.208, mereka membuka dialog mereka di dalam sesi latar belakang. Di v2.1.208 saja, Claude Code juga menolak pemilih `/model` dalam sesi latar belakang, dan `/upgrade` mencetak URL upgrade alih-alih membuka browser.

Kata-kata tersebut menamai perintah. Daftar pengaturan `/mcp` melaporkan:

```text theme={null}
Can't open MCP settings while no terminal is attached to this background session. This session now shows "needs input" in agent view — open it and run /mcp to manage servers, or use `/mcp enable|disable|reconnect <server>` to steer without the panel.
```

**Yang harus dilakukan:**

* Lampirkan ke sesi dari tampilan agen, di mana sesi terdaftar di bawah **Needs input**, dan jalankan perintah lagi
* Atau gunakan formulir yang dinamai pesan, seperti `/mcp reconnect <server>`, `/mcp enable`, atau `/mcp disable`, yang berfungsi tanpa melampirkan

<h3 id="write-or-command-blocked-because-the-path-cannot-be-safely-resolved">
  Penulisan atau perintah diblokir karena jalur tidak dapat diselesaikan dengan aman
</h3>

Claude mengatasi file atau direktori kerja melalui ejaan yang [penjaga isolasi worktree](/docs/id/agent-view#how-file-edits-are-isolated) tidak dapat menyelesaikan ke satu lokasi yang dapat diverifikasi. Penjaga memeriksa penulisan dan direktori kerja perintah di [sesi apa pun yang terisolasi dalam worktree](/docs/id/worktrees#how-claude-code-enforces-isolation), interaktif atau latar belakang, dan di [subagent yang terisolasi worktree](/docs/id/worktrees#isolate-subagents-with-worktrees). Ini menyelesaikan symlink sebelum memeriksa bahwa operasi tidak mencapai checkout bersama, dan ketika resolusi gagal, ia memblokir operasi daripada membiarkannya mendarat di sana. Pesan menamai bentuk jalur yang ditolaknya dan cara mencoba lagi:

```text theme={null}
This write was blocked because the path is spelled in a form that cannot be safely resolved (for example through a symlink storing a raw dot segment, a network-share or device-namespace shape, or an unreadable ancestor directory). If the file is inside the worktree /path/to/worktree, address it by its direct symlink-free path instead.
```

Perintah yang diblokir melaporkan penyebab yang sama untuk direktori kerjanya dan berakhir dengan `re-run the command from its direct symlink-free path`. Sebelum v2.1.217, penjaga membandingkan ejaan jalur tanpa menyelesaikan symlink, jadi ejaan ini tidak diblokir dan penulisan yang dirutekan melalui symlink dapat mendarat di checkout bersama.

**Yang harus dilakukan:**

* Biasanya tidak ada: pesan lengkap masuk ke Claude sebagai kesalahan alat, dan Claude mencoba lagi dengan jalur langsung yang dinamainya. Untuk pengeditan file yang diblokir, tampilan percakapan menampilkan hanya baris `Error editing file` pendek; pesan lengkap muncul dalam tampilan transkrip, yang Anda buka dengan `Ctrl+O`. Perintah yang diblokir mencetaknya dalam output perintahnya.
* Jika blokir berulang pada file yang sama, jalur kemungkinan besar berjalan melalui symlink yang berkomitmen yang targetnya berisi `..`, seperti `docs/current -> ../README.md`; minta Claude untuk mengedit file target dengan jalur sebenarnya alih-alih melalui tautan

<h3 id="write-or-command-blocked-because-the-path-names-a-network-location">
  Penulisan atau perintah diblokir karena jalur menamai lokasi jaringan
</h3>

Claude mengatasi file atau direktori kerja melalui jalur yang menamai drive yang bukan di mesin Anda, bagian UNC seperti `\\server\share\file` atau jalur automount `/net`, sementara checkout sesi berada di disk lokal. [Penjaga isolasi worktree](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) yang sama tidak dapat memverifikasi bahwa jalur seperti itu tetap keluar dari checkout bersama, jadi ia memblokir operasi. Mengisolasi sesi dalam worktree tidak menghilangkan blokir. Pesan menamai bentuk jalur yang digunakan sebagai gantinya:

```text theme={null}
This write was blocked because the path is network-shaped (a UNC share or /net automount spelling) while this session's checkout is local. Isolating cannot unblock it. If the file is genuinely inside the worktree /path/to/worktree, address it by its local, plainly-spelled path instead.
```

Perintah yang diblokir melaporkan penyebab yang sama untuk direktori kerjanya dan berakhir dengan `re-run the command from its local, plainly-spelled path`. Sebelum v2.1.217, penjaga hanya membandingkan teks jalur, jadi mengatasi file di dalam checkout melalui jalur UNC atau `/net` tidak diblokir.

**Yang harus dilakukan:**

* Biasanya tidak ada: Claude mencoba lagi dengan ejaan lokal yang diminta pesan
* Jika file berada di bagian jaringan daripada file lokal yang dieja dengan jalur jaringan, file tersebut berada di luar ruang kerja lokal sesi Anda; editnya dari sesi interaktif biasa sebagai gantinya

<h3 id="command-blocked-by-the-worktree-isolation-checks">
  Perintah diblokir oleh pemeriksaan isolasi worktree
</h3>

Claude menjalankan perintah Bash atau Monitor dalam [sesi yang terisolasi dalam worktree](/docs/id/worktrees#how-claude-code-enforces-isolation), dan Claude Code menolaknya karena salah satu dari dua alasan:

* Perintah menunjukkan git ke checkout utama.
* Claude Code tidak dapat memverifikasi dari teks perintah bahwa git apa pun yang dijalankan perintah tetap berada di dalam worktree. Perintah yang tidak pernah menamai git masih dapat ditolak karena alasan ini, karena memperluas indirection variabel seperti `${!name}` atau menjalankan substitusi fungsi Bash seperti `${ command; }` menghasilkan nilai pada runtime yang dapat menjadi perintah itu sendiri.

Bagian tengah pesan menamai apa yang tidak dapat diverifikasi:

```text wrap theme={null}
This session is isolated in the worktree /path/to/worktree, but this command evaluates ${!x@P} arithmetically inside a construct too complex to verify, which can run a command hidden in a variable's value. Refusing to run it — a worktree-isolated session's git operations must target its own worktree. Split it into plain, separate commands and run them from /path/to/worktree.
```

**Yang harus dilakukan:**

* Biasanya tidak ada: Claude membaca pesan dan menulis ulang perintah dengan cara yang diminta kalimat terakhirnya
* Jika perintah yang Anda minta terus ditolak, ejakan nilai yang ditandai secara harfiah: ganti indirection atau substitusi dengan nilainya, dan jalankan git sebagai perintah biasa terpisahnya dari dalam worktree
* Untuk bertindak pada checkout utama dengan sengaja, jalankan perintah sendiri di terminal di luar sesi

<h3 id="this-session-has-no-saved-transcript">
  Sesi ini tidak memiliki transkrip yang disimpan
</h3>

Anda melampirkan ke sesi [latar belakang](/docs/id/agent-view) yang dihentikan yang dilatarbelakangkan dari percakapan lain dengan `←` atau `/background` dan dihentikan sebelum respons pertamanya selesai. Sampai respons pertama itu selesai, percakapan masih hidup hanya dalam sesi tempat ia dilatarbelakangkan, jadi `claude attach` menolak untuk memulai sesi yang dihentikan daripada memulai percakapan kosong di bawah ID sesi yang sama. Pesan berakhir dengan perintah `claude respawn` untuk sesi ini:

```text theme={null}
This session has no saved transcript — it was stopped before its first response finished. If it was backgrounded from another conversation, that one is still intact; `claude respawn <id>` starts this one fresh.
```

Membuka baris sesi yang sama dalam [tampilan agen](/docs/id/agent-view) menampilkan `Press enter again to restart this session fresh` di bawah daftar sebagai gantinya, dan `Enter` kedua pada baris memulai ulang sesi dengan percakapan kosong. Sebelum v2.1.212, membuka baris yang dihentikan menampilkan pesan penolakan tanpa cara untuk memulai ulang dari tampilan agen. Sebelum v2.1.211, membuka sesi yang dihentikan secara diam-diam memulai percakapan kosong itu dan dapat menjalankan kembali prompt asli sesi.

**Yang harus dilakukan:**

* Percakapan yang Anda latarbelakangkan masih utuh: lanjutkan dengan [`claude --resume`](/docs/id/sessions) atau terus bekerja di dalamnya
* Untuk memulai sesi yang dihentikan segar bagaimanapun, jalankan `claude respawn <id>` dengan ID dari pesan, atau tekan `Enter` dua kali pada barisnya dalam tampilan agen
* Jika sesi memang menyelesaikan respons dan Anda masih melihat penolakan ini pada versi sebelum v2.1.214, folder yang tidak dapat dibaca di `~/.claude/projects` dapat membuat pemindaian transkrip melewatkan percakapan yang disimpan; perbarui ke v2.1.214 atau lebih baru, yang mentoleransi folder yang tidak dapat dibaca selama pemindaian

<h3 id="this-session-is-running-in-another-terminal">
  Sesi ini berjalan di terminal lain
</h3>

Anda membuka baris sesi yang dihentikan dalam [tampilan agen](/docs/id/agent-view), dan percakapan yang disimpannya sudah terbuka dalam proses Claude Code langsung lain di mesin ini, jadi Claude Code menolak untuk memulai proses kedua yang akan menulis ke transkrip yang sama. Pesan mana yang Anda lihat tergantung pada [apa yang memegang percakapan](/docs/id/agent-view#opening-a-session-says-the-conversation-is-already-open):

```text theme={null}
Can't open — this session is running in another terminal
This conversation is already open in another running Claude session — use that one, or close it and try again
```

* **`running in another terminal`**: terminal memegang percakapan, misalnya yang tempat Anda melanjutkannya dengan `claude --resume` atau `/resume`. Baris juga menampilkan `Open in a terminal`.
* **`already open in another running Claude session`**: proses Claude Code non-interaktif lain menahannya, misalnya proses [sesi latar belakang](/docs/id/agent-view#the-supervisor-process) untuk percakapan yang sama yang belum keluar.

Claude Code menyimpan balasan yang Anda ketik saat membuka baris dan mengirimkannya sebagai prompt berikutnya sesi ketika sesi berikutnya dimulai.

**Yang harus dilakukan:**

* Lanjutkan percakapan dalam proses yang memilikinya, atau keluar dari proses itu dan buka baris lagi

Sebelum v2.1.248, hanya penolakan `already open in another running Claude session` yang ada: percakapan yang dilanjutkan dalam terminal tidak dihitung sebagai terbuka, dan membuka baris memulai proses Claude Code kedua yang menulis ke percakapan yang sama.

<h3 id="this-sessions-saved-conversation-is-no-longer-on-disk">
  Percakapan yang disimpan sesi ini tidak lagi ada di disk
</h3>

Anda membuka [sesi latar belakang](/docs/id/agent-view) yang berakhir saat layanan latar belakang mati, dan [pembersihan transkrip](/docs/id/settings-reference#cleanupperioddays) telah menghapus percakapan yang disimpannya, misalnya setelah mesin mati selama berminggu-minggu. Membuka baris seperti itu biasanya [melanjutkan percakapan yang disimpannya](/docs/id/agent-view#sessions-show-as-failed-after-shutdown). Tanpa apa pun yang tersisa untuk dilanjutkan, Claude Code menolak daripada menjalankan kembali prompt asli sesi tanpa bertanya:

```text theme={null}
This session's saved conversation is no longer on disk (it ended while the background service was off, and old transcripts are cleaned up), so there is nothing to resume. `claude rm 7c5dcf5d` deletes the row; `claude respawn 7c5dcf5d` runs its original prompt again instead.
```

`claude attach <id>` mencetak teks ini. Dalam tampilan agen, footer lebih pendek dan berakhir dengan `ctrl+x deletes the row`.

**Yang harus dilakukan:**

* Jalankan `claude rm <id>` untuk menghapus baris. Ketika salah satu dari [kasus yang disimpan](/docs/id/agent-view#what-deleting-a-session-removes) berlaku, `claude rm` menyimpan baris dan worktree sebagai gantinya dan menamai alasannya
* Untuk menjalankan prompt asli sesi lagi sebagai percakapan segar, jalankan `claude respawn <id>`

Sebelum v2.1.248, membuka baris seperti itu menjalankan kembali prompt asli sesi alih-alih menolak, menarik tugas yang berusia berminggu-minggu kembali ke latar depan.

<h3 id="worktree-has-commits-that-are-not-pushed-anywhere">
  Worktree memiliki komitmen yang tidak didorong ke mana pun
</h3>

Anda mencoba menghapus [sesi latar belakang](/docs/id/agent-view#what-deleting-a-session-removes) yang worktree-nya menyimpan komitmen yang Claude Code tidak dapat memastikan disimpan di tempat lain. Claude Code menyimpan worktree dan baris sesi daripada menghancurkan komitmen yang tidak terlihat. `claude rm` menamai cabang dan komitmen yang tidak didorong, dan mengatakan cara melanjutkan:

```text theme={null}
kept 7c5dcf5d — its worktree is still at "/home/you/project/.claude/worktrees/fix-login"
  2 unpushed commits on "claude/fix-login": a1b2c3d "Fix login flow" and 1 more. They exist on no remote, so deleting the worktree would lose them.
  push them and run 'claude rm 7c5dcf5d' again, or discard the worktree and its commits: claude rm 7c5dcf5d --discard-unpushed a1b2c3d000000000000000000000000000000000@0123456789abcdef0123456789abcdef
```

Ketika Claude Code tidak dapat merangkum komitmen, baris detail berbunyi `The worktree has unpushed commits` sebagai gantinya. Dalam [tampilan agen](/docs/id/agent-view), baris sesi menampilkan `not deleted` dengan alasan yang sama.

Komitmen pada remote tidak memblokir penghapusan. Begitu juga komitmen pada salinan lokal cabang default remote `origin` Anda, selama cabang itu diperiksa dalam checkout utama Anda, direktori repositori itu sendiri daripada worktree.

**Yang harus dilakukan:**

* Untuk menyimpan komitmen, dorong cabang worktree, atau gabungkan ke dalam cabang default yang diperiksa dalam checkout utama Anda, kemudian hapus sesi lagi
* Untuk membuang komitmen, jalankan perintah `claude rm <id> --discard-unpushed` yang dicetak pesan, atau tekan `Ctrl+X` dua kali pada baris sesi dalam tampilan agen lagi. Ini menghapus sesi dan worktree bersama dengan cabangnya, komitmen yang tidak didorong, dan perubahan yang tidak berkomitmen. Jika worktree telah mendapatkan komitmen sejak penolakan, Claude Code menyimpannya lagi dan menampilkan status yang diperbarui
* Ketika pesan mengatakan worktree juga dicatat oleh sesi selesai lain, menghapus lagi tidak membuangnya: dorong komitmen, kemudian hapus sesi lagi

Sebelum v2.1.268, `claude rm` menempatkan ringkasan komitmen pada baris `kept` itu sendiri. Ketika `claude rm` tidak dapat merangkum komitmen, baris `kept` berbunyi `worktree has commits that are not pushed anywhere` sebagai pengganti ringkasan.

Sebelum v2.1.260, pesan tidak menamai cabang atau komitmen, dan menghapus lagi ditolak dengan cara yang sama: menghapus sesi tanpa mendorong berarti menghapus worktree sendiri dengan `git worktree remove --force <path>`, kemudian menjalankan `claude rm <id>` lagi.

Sebelum v2.1.248, cabang default yang diperiksa dalam checkout utama Anda tidak dihitung: cabang yang sudah Anda gabungkan di sana masih memicu penolakan ini sampai komitmennya mencapai remote.

<h3 id="terminal-host-process-died">
  Proses host terminal mati
</h3>

Setiap terminal [sesi latar belakang](/docs/id/agent-view) berjalan dalam proses host di bawah layanan latar belakang, dan proses itu mati saat layanan masih menyimpan koneksinya, jadi sesi tidak dapat dijangkau.

Di Linux dan WSL, layanan latar belakang memeriksa setiap proses host setiap beberapa detik, menandai sesi gagal ketika proses telah keluar tetapi koneksinya ke layanan tidak pernah ditutup, dan menampilkan alasannya pada barisnya dalam [tampilan agen](/docs/id/agent-view#read-session-state):

```text theme={null}
terminal host process died — press Enter to restart
```

Jika Anda membuka baris sebelum pemeriksaan berjalan, footer menampilkan `This session's terminal host process died (the conversation is saved) — press Enter to restart it` dan baris berubah menjadi gagal.

Dari shell, `claude attach <id>` memulai ulang sesi yang sudah ditandai gagal untuk host yang mati, dan sebaliknya mencetak penyebab dan keluar:

```text theme={null}
Couldn't attach to <id> — This session's terminal host process died (the conversation is saved) — run `claude attach <id>` again to restart it on a fresh host.
```

Percakapan disimpan bagaimanapun.

Baris yang menjalankan [perintah shell](/docs/id/agent-view#run-a-shell-command) sebagai gantinya menampilkan `terminal host process died — its output is gone; the command was not run again`, dan `claude attach` mencetak `This command's terminal host process died — its output is gone and the command was not run again`. Claude Code tidak pernah menjalankan kembali perintah untuk Anda.

**Yang harus dilakukan:**

* Dalam tampilan agen, tekan `Enter` pada baris yang gagal; sesi dimulai ulang pada proses host segar dan percakapan dilanjutkan
* Dari shell, jalankan `claude attach <id>` lagi. Claude Code mencetak `Session <id>'s terminal host died — restarting it on a fresh one…` dan membuka kembali sesi
* Anda tidak dapat memulai ulang baris perintah shell dengan cara ini; kirim perintah lagi untuk menjalankannya kembali

Sebelum v2.1.247, proses host yang mati dapat melewati setiap pemeriksaan kelangsungan hidup yang dijalankan layanan latar belakang, jadi membuka sesi menampilkan `opening… · esc to cancel` tanpa batas dan `claude attach <id>` menunggu tanpa melaporkan kesalahan.

<h3 id="session-isnt-responding">
  Sesi tidak merespons
</h3>

Anda membuka [sesi latar belakang](/docs/id/agent-view) dan layanan latar belakang menerima pembukaan, tetapi tidak ada output yang tiba selama sekitar sepuluh detik, jadi Claude Code menyimpulkan bahwa proses yang menyampaikan terminal sesi tidak dapat memberikan output, dan mengakhiri upaya alih-alih menunggu.

Dalam tampilan agen, Claude Code menawarkan restart dalam footer:

```text theme={null}
Press enter again to restart this session — it isn't responding (its conversation is saved and resumes).
```

Dari shell, `claude attach <id>` mencetak penyebab dan keluar:

```text theme={null}
Couldn't attach to <id> — Session isn't responding — `claude stop <id>`, then `claude attach <id>` restarts it (the conversation is saved).
```

Claude Code tidak pernah memulai ulang baris yang menjalankan [perintah shell](/docs/id/agent-view#run-a-shell-command) untuk Anda, karena restart akan menjalankan perintah lagi.

**Yang harus dilakukan:**

* Dalam tampilan agen, tekan `Enter` pada baris yang sama lagi. Claude Code menghentikan proses yang tidak responsif dan memulai ulang sesi, dan percakapan dilanjutkan. Tidak ada yang dihentikan tanpa tekan kedua itu
* Dari shell, jalankan `claude stop <id>`, kemudian `claude attach <id>`
* Untuk baris perintah shell, tekan `Ctrl+X` dalam tampilan agen atau jalankan `claude stop <id>` untuk menghentikannya; kirim perintah lagi untuk menjalankannya kembali

<h3 id="session-was-stopped-while-the-respawn-was-in-flight">
  Sesi dihentikan saat respawn sedang dalam perjalanan
</h3>

Anda membuka [sesi latar belakang](/docs/id/agent-view) yang prosesnya tidak berjalan, dan saat Claude Code memulainya kembali, proses Claude Code lain menghentikannya, misalnya `claude stop` di terminal lain. Claude Code menyimpan sesi tetap dihentikan:

```text theme={null}
Session <id> was stopped while the respawn was in flight
```

Membuka sesi yang baru saja Anda kirim, saat prosesnya masih dimulai, menunggu proses sebagai gantinya. Sebelum v2.1.246, membukanya pada saat itu dapat menghentikannya dan menampilkan pesan ini.

**Yang harus dilakukan:**

* Jika Anda tidak menghentikan sesi, buka barisnya lagi dalam tampilan agen atau jalankan `claude respawn <id>` untuk memulainya kembali
* Jika Anda menghentikannya sendiri, tidak ada yang tersisa untuk dilakukan: sesi tetap dihentikan

<h3 id="session-agent-no-longer-available">
  Agen sesi tidak lagi tersedia
</h3>

Anda melanjutkan sesi yang menjalankan [agen khusus](/docs/id/sub-agents#invoke-subagents-explicitly), dimulai dengan `--agent` atau pengaturan `agent`, dan Claude Code tidak menemukan agen dengan nama itu. Ini mencari direktori asli sesi terlebih dahulu, ketika Anda telah [mempercayai ruang kerja itu](/docs/id/permissions#project-allow-rules-and-workspace-trust), kemudian direktori tempat Anda melanjutkan. Sesi masih dilanjutkan, tetapi dengan alat default, jadi pembatasan alat agen tidak lagi berlaku:

```text theme={null}
This session was running agent 'code-reviewer', which is no longer available (no agent by that name in /home/you/project). Continuing with the default tools and system prompt — the agent's tool restrictions no longer apply. To restore it, re-create the agent, or resume with an explicit --agent <name>.
```

Peringatan hanya menamai direktori yang Claude Code cari, dan muncul dalam percakapan yang dilanjutkan apakah Anda membangunkan [sesi latar belakang](/docs/id/agent-view), menjalankan `/resume` atau `claude --resume`, atau melanjutkan dalam [mode non-interaktif](/docs/id/headless), di mana ia juga masuk ke stderr. Sesi menggunakan `--input-format stream-json` tidak menampilkannya, karena Agent SDK menyediakan agen setelah startup.

Claude Code tidak menyimpan fallback ke sesi, jadi peringatan berulang pada setiap resume sampai Anda bertindak. Agen `claude` bawaan tidak memicu peringatan, karena jatuh kembali ke set alat default tidak mengubah apa pun untuk itu. Sebelum v2.1.216, Claude Code secara diam-diam melanjutkan sebagai agen default, dan pencarian mencakup hanya direktori tempat Anda melanjutkan, jadi agen yang bersifat proyek hilang pada resume apa pun dari direktori lain.

**Yang harus dilakukan:**

* Buat ulang file agen di `.claude/agents/<name>.md` dalam proyek sesi, atau di `~/.claude/agents/<name>.md` untuk agen pribadi, kemudian lanjutkan lagi
* Atau lanjutkan dengan `--agent <name>` yang menamai agen yang memang ada, untuk menjalankan sesi sebagai agen itu sebagai gantinya
* Jika agen bersifat proyek dan Anda belum mempercayai direktori asli sesi, jalankan Claude Code di sana sekali, terima dialog kepercayaan, kemudian lanjutkan lagi

<h3 id="claude_code_process_wrapper-launcher-errors">
  Kesalahan peluncur CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/id/corporate-launcher) diatur, dan nilainya tidak dapat digunakan, jadi Claude Code menolak untuk memulai proses yang terpengaruh daripada menjalankannya tanpa peluncur. Masalah konfigurasi dilaporkan dengan pesan yang dimulai dengan nama variabel dan menyatakan alasannya, misalnya:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Peluncur yang dimulai tetapi keluar tanpa mengganti dirinya dengan Claude Code gagal sesi yang dimulainya, dan baris sesi dalam tampilan agen melaporkan bahwa peluncur `must exec, not daemonize`, diikuti oleh apa pun yang dicetak peluncur. Sesi yang tidak dapat dimulai atau menjangkau layanan latar belakang karena peluncur melaporkan masalah peluncur sebagai alasan di dalam `Couldn't reach the background service (...)`.

**Yang harus dilakukan:**

* Atur variabel ke jalur absolut dari executable yang berakhir dengan memanggil `exec "$@"`. Lihat [kontrak peluncur](/docs/id/corporate-launcher#the-launcher-contract) untuk kontrak lengkap
* Periksa `/status`, yang menampilkan perintah peluncuran yang diselesaikan dalam entri Self-exec-nya dan memperingatkan ketika layanan latar belakang yang berjalan tidak cocok, atau jalankan `claude daemon status` dari shell
* Setelah memperbaiki nilai dalam blok `env` dari [pengaturan](/docs/id/corporate-launcher#set-up-the-launcher), mulai ulang layanan latar belakang dengan `claude daemon stop --any` sehingga pengiriman berikutnya memulai yang dibungkus

<h3 id="eunknown-when-starting-a-background-session">
  EUNKNOWN saat memulai sesi latar belakang
</h3>

Windows menolak untuk memulai program dengan kode kesalahan yang tidak memiliki nama standar, jadi kegagalan muncul sebagai `EUNKNOWN`. Pemicu biasanya adalah kebijakan pembatasan perangkat lunak, seperti Group Policy atau AppLocker, memblokir program yang dimulai. Kesalahan muncul ketika Anda memulai [sesi latar belakang](/docs/id/agent-view) dengan `/background` atau `claude --bg`:

```text theme={null}
Couldn't reach the background service (spawn background service: EUNKNOWN: unknown error, uv_spawn) — run 'claude daemon status'
```

Pada beberapa akun pesan mengatakan `daemon` sebagai pengganti `background service`.

Pada instalasi npm, `EUNKNOWN` yang muncul saat `npm install -g @anthropic-ai/claude-code` mengganti biner memiliki penyebab yang sama dengan [`EACCES` selama reinstal](#eacces-when-starting-a-background-session) dan hilang ketika Anda mencoba lagi setelah instalasi selesai.

Claude Code memulai layanan latar belakang melalui PowerShell sehingga layanan bertahan menutup terminal, menggunakan PowerShell 7 ketika diinstal dan Windows PowerShell 5.1 sebaliknya. Ketika tidak ada PowerShell yang dapat berjalan, Claude Code memulai layanan secara langsung sebagai gantinya, jadi kebijakan yang memblokir hanya PowerShell tidak menyebabkan kesalahan ini. Jika Anda melihatnya saat tidak ada instalasi npm yang berjalan, kebijakan memblokir executable Claude Code itu sendiri.

Sebelum v2.1.212, Claude Code hanya menggunakan Windows PowerShell 5.1 untuk memulai layanan, jadi mesin apa pun di mana Group Policy memblokir PowerShell 5.1 gagal dengan `Couldn't start the session — EUNKNOWN: unknown error, uv_spawn`, bahkan dengan PowerShell 7 diinstal.

**Yang harus dilakukan:**

* Jika pesan berbunyi `Couldn't start the session`, tingkatkan ke v2.1.212 atau lebih baru. Pada versi sebelumnya Anda juga dapat menjalankan `claude daemon run` di terminal terpisah terlebih dahulu, kemudian mulai sesi latar belakang lagi. Perintah itu menjalankan layanan latar belakang di latar depan terminal, jadi layanan berlangsung hanya selama terminal itu tetap terbuka.
* Jika instalasi npm mengganti biner, tunggu hingga selesai, kemudian mulai sesi latar belakang lagi
* Jika kesalahan muncul pada v2.1.212 atau lebih baru saat tidak ada instalasi npm yang berjalan, minta administrator Windows Anda untuk mengizinkan executable Claude Code dalam kebijakan pembatasan
* Jika layanan latar belakang berhenti ketika Anda menutup terminal, Claude Code memulainya tanpa PowerShell. Instal PowerShell 7, atau minta administrator Anda untuk membuka blokir PowerShell, sehingga layanan dapat bertahan lebih lama dari terminal.

<h3 id="eacces-when-starting-a-background-session">
  EACCES saat memulai sesi latar belakang
</h3>

Claude Code tidak dapat menjalankan binernya sendiri untuk memulai [layanan latar belakang](/docs/id/agent-view#the-supervisor-process) yang menyelenggarakan sesi latar belakang. Pada instalasi npm, ini biasanya berarti `npm install -g @anthropic-ai/claude-code` mengganti biner pada saat itu, apakah Anda menjalankannya atau [auto-updater](/docs/id/setup#auto-updates) melakukannya. Kesalahan muncul ketika Anda membuka sesi dari [tampilan agen](/docs/id/agent-view):

```text theme={null}
Couldn't start the background service — spawn background service: EACCES: permission denied, posix_spawn '/usr/local/lib/node_modules/@anthropic-ai/claude-code/bin/claude'
```

Ketika Anda memulai sesi dengan `/background` atau `claude --bg`, alasan yang sama muncul di dalam `Couldn't reach the background service (...)`. Selama jendela reinstal yang sama kesalahan dapat menamai kode lain sebagai gantinya, seperti `ENOENT` atau `ENOEXEC`, atau `EUNKNOWN` atau `EPERM` di Windows; `EUNKNOWN` yang bertahan di seluruh percobaan ulang memiliki [penyebab berbeda](#eunknown-when-starting-a-background-session).

Pada instalasi npm, Claude Code menunggu reinstal selesai dan mencoba lagi sendiri: hingga sepuluh detik, dan hingga dua menit saat instalasi npm Claude Code masih terlihat berjalan di mesin, yang mencakup proses Claude Code lain mengunduh pembaruan. Ketika instalasi melampaui waktu tunggu itu, kegagalan menamai pembaruan alih-alih kode kesalahan telanjang:

```text theme={null}
Claude Code is being updated by npm on this machine (still not runnable after 2 min, EACCES) — try again when the update finishes
```

Sebelum v2.1.257, waktu tunggu berhenti pada sepuluh detik dalam setiap kasus, jadi kesalahan ini muncul saat proses Claude Code lain masih mengunduh pembaruan. Sebelum v2.1.246, Claude Code gagal sekaligus, tanpa menunggu.

**Yang harus dilakukan:**

* Tunggu beberapa detik, kemudian buka sesi atau kirim lagi. Ketika pesan mengatakan Claude Code sedang diperbarui, coba lagi setelah pembaruan selesai.
* Jika kesalahan bertahan saat tidak ada instalasi npm yang berjalan, pengguna Anda tidak dapat menjalankan biner yang diinstal. Periksa izinnya dan direktorinya, atau instal ulang Claude Code.

<h3 id="background-service-exited-before-it-became-reachable">
  Layanan latar belakang keluar sebelum dapat dijangkau
</h3>

Proses yang Claude Code mulai sebagai [layanan latar belakang](/docs/id/agent-view#the-supervisor-process) keluar sebelum menerima koneksi, jadi Claude Code tidak dapat membuka sesi Anda. Ketika layanan mencetak kesalahan sebelum keluar, alasan dalam tanda kurung memberikan kode keluar atau sinyal dan baris pertama yang dicetak layanan, yang menamai apa yang menghentikannya:

```text theme={null}
Couldn't reach the background service (background service exited before it became reachable (exit code N): <the service's first error line>) — run 'claude daemon status'
```

Ketika Anda membuka sesi dari [tampilan agen](/docs/id/agent-view), alasan yang sama mengikuti `Couldn't start the background service —`. Ketika layanan tidak mencetak apa pun sebelum keluar, pesan mengatakan `nothing on stderr` sebagai gantinya.

Claude Code melaporkan kegagalan dengan baris kesalahan layanan. Sebelum v2.1.246, kegagalan muncul hanya setelah menunggu 45 detik, sebagai `background service did not become reachable within 45s`, tanpa baris kesalahan layanan.

Dua alasan yang dikutip memiliki penyebab yang diketahui:

* `Error: claude native binary not installed.`: instalasi npm mengganti biner Claude Code pada saat itu, jadi layanan menjalankan placeholder npm sebagai gantinya. Coba lagi setelah instalasi selesai; jika baris bertahan tanpa instalasi yang berjalan, [selesaikan instalasi npm](/docs/id/troubleshoot-install#native-binary-not-found-after-npm-install). Sebelum v2.1.257, pembaruan diri npm macOS menghasilkan kegagalan ini pada setiap awal selama jendela instalasi.
* `nothing on stderr` dengan kode keluar 1, pada setiap awal, di Windows: `daemon.lock` menamai proses yang Claude Code tidak dapat menandai atau membuktikan hilang, jadi setiap layanan baru menyimpulkan yang lain menyimpan kunci dan keluar. Kunci yang penulis Claude Code dapat membuktikan hilang diganti sendiri dan tidak menghasilkan kegagalan ini. Ketika kegagalan berulang pada setiap awal, hapus `~/.claude/daemon.lock`, kemudian buka sesi atau kirim lagi. Sebelum v2.1.257, kunci seperti itu memblokir setiap awal sampai Anda menghapus file.

**Yang harus dilakukan:**

* Jika pesan mengutip baris, perbaiki apa yang dinamainya, kemudian buka sesi atau kirim lagi. Upaya berikutnya memulai layanan lagi
* Jalankan `claude daemon status` untuk memeriksa apakah layanan berjalan sekarang

<h3 id="working-directory-no-longer-exists-when-starting-a-background-session">
  Direktori kerja tidak lagi ada saat memulai sesi latar belakang
</h3>

Anda mencoba memulai [sesi latar belakang](/docs/id/agent-view) dalam direktori yang tidak lagi ada. Ini terjadi ketika Anda mengirim dari tampilan agen atau menjalankan `/background` setelah direktori tempat Anda bekerja dihapus atau dipindahkan. Ini juga terjadi ketika Anda melampirkan ke atau memulai ulang sesi yang prosesnya telah keluar dan direktorinya hilang, karena proses baru akan dimulai di direktori yang sama. Claude Code tidak memulai sesi, dan pesan menamai direktori yang hilang:

```text theme={null}
Couldn't start a background session (working directory no longer exists or is not accessible: /tmp/demo)
```

Sebelum v2.1.257, sesi tampak dimulai dan kemudian ditampilkan dalam tampilan agen sebagai baris yang gagal dengan alasan yang sama.

**Yang harus dilakukan:**

* Buat ulang direktori yang dinamai pesan, atau kirim dari direktori yang ada, kemudian coba lagi

<h2 id="wrapper-and-ide-errors">
  Kesalahan Wrapper dan IDE
</h2>

Kesalahan ini berasal dari program yang meluncurkan Claude Code untuk Anda, seperti ekstensi IDE atau aplikasi [Agent SDK](/docs/id/agent-sdk/overview), bukan dari Claude Code itu sendiri.

<h3 id="claude-code-process-exited-with-code-n">
  Proses Claude Code keluar dengan kode N
</h3>

Proses `claude` yang mendasar keluar dengan kode bukan nol. Kode keluar saja tidak mengatakan apa yang gagal: kesalahan sebenarnya ada di output proses itu sendiri, yang wrapper tambahkan ketika menangkap apa pun dan sebaliknya menyimpannya di log-nya.

```text theme={null}
Error: Claude Code process exited with code 1
```

Di Windows, build native dapat keluar dengan kode `4294967295` tepat setelah giliran selesai. Ketika keluar itu mendarat di batas giliran, tanpa pesan menunggu dan tidak ada tugas latar belakang yang berjalan, [ekstensi VS Code](/docs/id/vs-code) menutup sesi dengan tenang alih-alih menampilkan kesalahan ini. Pesan Anda berikutnya melanjutkan percakapan.

Sebelum v2.1.273, ekstensi menampilkan kesalahan untuk keluar itu di setiap batas giliran, meskipun tidak ada yang hilang.

**Yang harus dilakukan:**

* Di VS Code, ikuti tautan **View output logs** yang ditampilkan dengan kesalahan untuk melihat kegagalan yang mendasar
* Dalam aplikasi Agent SDK, tangkap kesalahan di sekitar loop pesan Anda. Entri di bawah [CLI process exit](/docs/id/agent-sdk/troubleshooting#cli-process-exit) mencakup apa yang diterima kode Anda di setiap bahasa SDK.
* Jalankan `claude` di terminal dalam proyek yang sama. Kegagalan biasanya direproduksi di sana dengan pesan kesalahan sebenarnya, yang kemudian dapat Anda cari di halaman ini.
* Jalankan `claude doctor` di terminal untuk memeriksa instalasi dan konfigurasi

<h3 id="could-not-locate-the-claude-cli-on-path">
  Tidak dapat menemukan Claude CLI di PATH
</h3>

[Ekstensi VS Code](/docs/id/vs-code) menampilkan kesalahan ini di Windows ketika Anda membuka Claude Code di terminal terintegrasi, shell terminal adalah PowerShell, dan ekstensi tidak dapat menemukan executable `claude` yang terinstal di PATH. Ekstensi menolak untuk meluncurkan Claude Code sampai menemukan `claude` yang terinstal di PATH.

```text theme={null}
Failed to run Claude Code: Error: Could not locate the Claude CLI on PATH. Launching by name in a PowerShell terminal would run a 'claude' from the open folder instead of the installed CLI, so the launch was blocked. Make sure the Claude CLI's install directory is on your system PATH (not only your PowerShell profile), then restart VS Code and try again. VS Code reads PATH when it starts, so PATH changes take effect only after a restart.
```

**Yang harus dilakukan:**

* Buka jendela PowerShell baru di luar VS Code dan jalankan `where.exe claude`. Jika tidak mencetak jalur, CLI tidak ada di PATH Anda: tambahkan direktori instalasinya dengan mengikuti [Verify your PATH](/docs/id/troubleshoot-install#verify-your-path). Jika mencetak jalur, entri berasal dari profil PowerShell Anda atau dari perubahan PATH yang belum diambil VS Code; dua langkah berikutnya mencakup kasus-kasus tersebut.
* Atur entri PATH sebagai variabel lingkungan pengguna atau sistem, bukan di profil PowerShell Anda. Ekstensi tidak menjalankan profil Anda, jadi edit PATH yang hanya ada di sana tidak pernah mencapainya.
* Mulai ulang VS Code setelah mengubah PATH. Ekstensi memeriksa PATH yang ditangkap VS Code saat startup, jadi perubahan PATH hanya berlaku setelah restart.

<h3 id="the-connection-to-claude-code-ended-before-this-message-completed">
  Koneksi ke Claude Code berakhir sebelum pesan ini selesai
</h3>

[Ekstensi VS Code](/docs/id/vs-code) mengirim pesan Anda ke proses `claude`, dan koneksi berakhir tanpa kesalahan sebelum proses mengakui atau menyelesaikannya. Ekstensi tidak dapat mengatakan apakah pesan diproses, jadi meminta Anda untuk mengirimnya lagi:

```text theme={null}
The connection to Claude Code ended before this message completed — it may not have been processed, so please send it again.
```

**Yang harus dilakukan:**

* Kirim pesan lagi. Pesan berikutnya memulai proses `claude` segar yang melanjutkan percakapan.
* Jika terulang, jalankan `claude` di terminal dalam proyek yang sama. Kegagalan yang terus mengakhiri proses biasanya direproduksi di sana dengan pesan kesalahan sebenarnya.

<h2 id="rewind-warnings-and-errors">
  Peringatan dan kesalahan Rewind
</h2>

Pesan-pesan ini berasal dari pemulihan kode [`/rewind`](/docs/id/checkpointing). `Restored the code, but skipped N files` adalah peringatan yang menunjukkan bahwa Claude Code melewati beberapa jalur. `No files were restored` adalah kesalahan yang berarti tidak ada yang dipulihkan.

<h3 id="restored-the-code-but-skipped-files">
  Restored the code, but skipped files
</h3>

Pemulihan kode `/rewind` melewati satu atau lebih jalur terlacak alih-alih menulis atau menghapus melaluinya. Claude Code melewati jalur ketika:

* itu adalah, atau menjadi, symlink, hard link, atau file non-reguler lainnya
* direktorinya berubah sejak checkpoint
* backup-nya tidak dapat dibaca dengan aman

Jalur yang dilewati mempertahankan konten saat ini mereka. Sebelum v2.1.216, `/rewind` menulis dan menghapus melalui link di jalur terlacak, dan tidak melaporkan pemulihan sebagian.

```text theme={null}
Restored the code, but skipped 2 files: the tracked path is (or became) a link or other non-regular file, its directory changed since the checkpoint, or its backup could not be safely read. Skipped files were left untouched — run with --debug for the paths.
```

**Yang harus dilakukan:**

* Identifikasi file mana yang dilewati sehingga Anda dapat menangani masing-masing dengan langkah-langkah di bawah ini. Pesan hanya memberikan hitungan; log debug di `~/.claude/debug/<session-id>.txt` menamai setiap jalur yang dilewati saat pemulihan berjalan, jadi aktifkan logging debug dengan `/debug` sebelum pemulihan berikutnya Anda. Di macOS atau Linux, Anda dapat menemukan link secara langsung: `find . -type l` untuk symlink dan `find . -type f -links +1` untuk file hard-linked.
* Jika file yang dilewati adalah link yang Anda buat dengan sengaja, seperti file konfigurasi yang dikelola oleh manajer dotfile atau file yang hard-linked oleh alat seperti pnpm, rewind membiarkan kontennya saja. Untuk membatalkan perubahan sesi terhadapnya, minta Claude untuk membalikkan edit atau edit file sendiri
* Jika Anda tidak membuat link, periksa jalur sebelum mempercayai kontennya: sesuatu mengganti file setelah checkpoint

<h3 id="no-files-were-restored">
  No files were restored
</h3>

Claude Code menampilkan pesan ini ketika Anda memulihkan kode dengan [`/rewind`](/docs/id/checkpointing) dan tidak dapat memulihkan file apa pun dalam checkpoint tersebut. Untuk setiap file, baik backup yang disimpan Claude Code sebelum mengeditnya hilang, atau Claude Code tidak dapat menulis atau menghapus file.

```text theme={null}
Failed to restore the code:
No files were restored: 1 file failed (backup missing, or the file could not be updated)
```

Claude Code menghapus backup sesi dalam [retention sweep](/docs/id/claude-directory#cleaned-up-automatically), secara default sekitar 30 hari setelah sesi terakhir menyimpannya. Jika Anda melanjutkan sesi setelah itu, `/rewind` masih mencantumkan checkpoint-nya, tetapi rewind ke salah satunya dapat gagal dengan kesalahan ini. Jika pesan juga mengatakan `N paths were skipped for link safety`, lihat [Restored the code, but skipped files](#restored-the-code-but-skipped-files) untuk jalur-jalur tersebut.

Ketika Anda fork sesi, misalnya dengan [`--fork-session`](/docs/id/cli-reference#cli-flags) atau [`/branch`](/docs/id/sessions#branch-a-session), Claude Code menyalin backup sesi asli ke fork. Ketika Claude Code tidak dapat menyalin backup, misalnya karena disk penuh, backup tersebut hilang di fork. Rewind ke checkpoint yang membutuhkannya dapat gagal dengan kesalahan ini.

**Yang harus dilakukan:**

* Batalkan perubahan dengan cara lain: minta Claude untuk membalikkan edit-nya, atau pulihkan file dari version control. Ketika backup hilang, menjalankan `/rewind` lagi gagal dengan cara yang sama.
* Jika Claude Code tidak dapat menulis atau menghapus file, perbaiki apa yang memblokir penulisan, seperti izin file, kemudian jalankan `/rewind` lagi.
* Untuk menyimpan backup lebih lama di sesi mendatang, naikkan [`cleanupPeriodDays`](/docs/id/settings-reference#cleanupperioddays).

Sebelum v2.1.260, Claude Code secara diam-diam melewati file yang backup-nya hilang, dan rewind tampak berhasil.

<h2 id="session-saving-warnings">
  Peringatan penyimpanan sesi
</h2>

Claude Code menampilkan peringatan ini pada baris persisten di bawah kotak input ketika tidak menyimpan transkrip sesi Anda. Sesi tetap berfungsi baik cara; peringatan memberi tahu Anda bahwa sesi mungkin hilang dari [`--resume`](/docs/id/sessions) nanti.

<h3 id="transcript-writes-are-failing">
  Penulisan transkrip gagal
</h3>

Claude Code menyimpan transkrip ke disk saat Anda bekerja, dan penulisannya ke [file transkrip](/docs/id/sessions#where-transcripts-are-stored) gagal. Pesan ini menyebutkan penyebabnya dengan kode kesalahan yang mendasar, misalnya disk penuh:

```text theme={null}
Transcript writes are failing (disk full — ENOSPC) · recent messages may not be saved for resume
```

Peringatan muncul pada titik berbeda tergantung pada kesalahannya:

* Pada kegagalan pertama untuk kondisi yang tidak hilang dengan sendirinya: disk penuh, kuota disk terlampaui, sistem file hanya-baca, jalur melebihi batas panjang sistem file, atau, di macOS dan Linux, kesalahan izin
* Setelah kegagalan berulang yang berlangsung setidaknya satu menit untuk semuanya, termasuk kesalahan izin di Windows, di mana pemindaian antivirus dapat gagal pada satu penulisan yang kemudian berhasil saat dicoba ulang

Sebelum v2.1.217, Claude Code menghapus penulisan yang gagal tanpa peringatan, dan `--resume` yang hilang pesan terbaru kemudian adalah tanda pertama.

**Yang harus dilakukan:**

* Perbaiki kondisi yang disebutkan kode kesalahan: bebaskan ruang disk untuk `ENOSPC`; naikkan atau hapus kuota untuk `EDQUOT`; pulihkan akses tulis ke lokasi transkrip untuk `EACCES`, `EPERM`, atau `EROFS`
* Peringatan hilang dengan sendirinya pada penulisan berikutnya yang berhasil; tidak perlu restart
* Pesan yang dikirim saat peringatan ditampilkan mungkin masih hilang ketika Anda melanjutkan sesi nanti

<h3 id="transcript-saving-is-off-skip-prompt-history">
  Penyimpanan transkrip dimatikan karena CLAUDE\_CODE\_SKIP\_PROMPT\_HISTORY diatur
</h3>

Sesi ini dimulai dengan [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/id/env-vars) diatur, jadi Claude Code tidak menulis transkrip atau riwayat prompt untuk sesi ini:

```text theme={null}
Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set · --resume will not find this session; if unintended, unset it and restart
```

Variabel ini adalah opt-out yang disengaja untuk sesi skrip sementara, tetapi juga dapat mencapai sesi melalui profil shell, skrip pembungkus, atau proses induk yang mengekspornya.

**Yang harus dilakukan:**

* Jika Anda mengatur variabel dengan sengaja, tidak ada tindakan yang diperlukan; pemberitahuan ini mengonfirmasi bahwa sesi tidak akan muncul di `--resume`, `--continue`, atau riwayat panah-atas
* Jika tidak, hapus variabel dari shell atau skrip yang meluncurkan `claude`, kemudian mulai sesi baru. Pesan dari sesi saat ini tidak disimpan secara retroaktif.

<h3 id="transcript-saving-is-off-child-session-marker">
  Penyimpanan transkrip dimatikan karena penanda CLAUDE\_CODE\_CHILD\_SESSION yang diwariskan
</h3>

Claude Code mengatur [`CLAUDE_CODE_CHILD_SESSION`](/docs/id/env-vars) dalam subproses yang dihasilkannya, dan memperlakukan sesi interaktif yang mewarisinya sebagai bersarang: Claude Code tidak menyimpan transkrip untuk sesi tersebut, jadi sesi yang Claude sendiri mulai tidak mengisi daftar `--resume` Anda. Pemberitahuan ini berarti sesi saat ini Anda mewarisi penanda:

```text theme={null}
Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker · restart with CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1 to keep future transcripts
```

Pemberitahuan ini diharapkan ketika Anda menjalankan `claude` dari dalam sesi Claude Code lain; pemberitahuan ini menandakan salah klasifikasi ketika penanda bocor melalui perantara yang tahan lama, misalnya terminal, sesi `screen`, atau peluncur yang awalnya dimulai oleh sesi Claude Code.

Di dalam tmux, Claude Code mendeteksi penanda yang tiba melalui lingkungan global server tmux dan terus menyimpan, jadi pemberitahuan ini tidak muncul untuk kasus itu.

**Yang harus dilakukan:**

* Jika Anda memulai sesi ini dari dalam sesi Claude Code lain dengan sengaja, tidak ada tindakan yang diperlukan
* Jika ini adalah sesi tingkat atas, keluar dan restart dengan [`CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1`](/docs/id/env-vars) diatur. Penyimpanan berlaku dari restart, jadi pesan yang dikirim sebelumnya tidak disimpan.
* Untuk memperbaiki peluncuran di masa depan dari terminal atau peluncur yang sama, hapus `CLAUDE_CODE_CHILD_SESSION` dari lingkungannya

<h2 id="configuration-warnings">
  Peringatan Konfigurasi
</h2>

Claude Code menulis sebagian besar pesan ini ke stderr, bukan ke dalam percakapan, dan menulis sebagian besar di saat startup. Sebuah entri mengatakan demikian ketika pesannya muncul di tempat lain, seperti dalam log debug atau sebagai pemberitahuan startup dalam tampilan percakapan, atau pada waktu lain, seperti [baris diagnostik model yang tidak dikenali](#unrecognized-model-id-on-a-request) pada saat permintaan.

<h3 id="fullscreen-failed-start-notice">
  Fullscreen renderer tidak selesai memulai
</h3>

Sesi [fullscreen](/docs/id/fullscreen) sebelumnya di mesin ini keluar sebelum selesai memulai, jadi Claude Code memulai sesi ini pada renderer klasik dan mencetak salah satu pemberitahuan ini:

```text theme={null}
Claude Code's fullscreen renderer didn't finish starting last time on this machine, so this launch is using the classic renderer. It will try fullscreen again next launch; /tui default keeps the classic renderer.

Claude Code's fullscreen renderer has repeatedly failed to start on this machine, so it has been turned off here. Run /tui fullscreen to try it again (this also resets after an update).
```

**Yang harus dilakukan:**

* Ikuti [Fullscreen rendering](/docs/id/fullscreen#fullscreen-renderer-didnt-finish-starting). Ini mengatakan pemberitahuan mana yang Anda dapatkan, apa yang Claude Code lakukan di sesi-sesi berikutnya, dan cara mencoba fullscreen lagi atau tetap menggunakan renderer klasik.
* Jika sesi yang mati mencetak pesan keluar, lihat [Claude Code keluar setelah kesalahan antarmuka yang tidak dapat dipulihkan](#exited-after-an-unrecoverable-interface-error) untuk mengetahui apa yang dinamainya.

Sebelum v2.1.236, Claude Code tidak mencetak pemberitahuan dan terus memulai sesi dalam fullscreen rendering setelah kegagalan startup.

<h3 id="exited-after-an-unrecoverable-interface-error">
  Claude Code keluar setelah kesalahan antarmuka yang tidak dapat dipulihkan
</h3>

Claude Code mencetak pesan ini ketika keluar karena antarmuka terminalnya mengalami kesalahan yang tidak dapat dipulihkan, di salah satu renderer. Kalimat kedua muncul hanya ketika kesalahan terjadi saat renderer [fullscreen](/docs/id/fullscreen) sedang memulai:

```text theme={null}
Claude Code exited after an unrecoverable interface error (<error>). It happened while the fullscreen renderer was starting, so the next launch will use the classic renderer (CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1 forces that any time).
```

**Yang harus dilakukan:**

* Mulai Claude Code lagi. Untuk melanjutkan percakapan, jalankan `claude --resume` di direktori yang sama.
* Jika pesan menyebutkan renderer fullscreen, [Fullscreen rendering](/docs/id/fullscreen#fullscreen-renderer-didnt-finish-starting) mengatakan apa yang dilakukan peluncuran berikutnya, yang tergantung pada cara Anda mengaktifkan fullscreen, dan cara mencoba fullscreen lagi atau tetap menggunakan renderer klasik.

Sebelum v2.1.236, Claude Code keluar tanpa mencetak pesan setelah kesalahan jenis ini.

<h3 id="agent-descriptions-are-over-the-15000-token-limit">
  Deskripsi agent melebihi batas token 15.0k
</h3>

Claude Code menampilkan peringatan ini sebagai pemberitahuan startup dalam tampilan percakapan daripada di stderr. Deskripsi gabungan dari [subagents](/docs/id/sub-agents) Anda, kecuali yang bawaan, melebihi 15.000 token seperti yang diperkirakan Claude Code. Setiap agent menghitung namanya ditambah frontmatter `description` nya. Claude Code memuat setiap agent terlepas dari apakah totalnya melebihi batas, jadi peringatan tidak mengubah apa yang dimuat.

```text theme={null}
Agent descriptions are over the 15.0k-token limit (~16.2k tokens) · ask Claude to trim agent descriptions in .claude/agents/
```

**Yang harus dilakukan:**

* Perpendek frontmatter `description` dari file agent Anda, atau minta Claude untuk memangkasnya untuk Anda.
* Hapus file agent yang tidak lagi Anda gunakan.

<h3 id="workspace-has-not-been-trusted">
  Workspace belum dipercaya
</h3>

Claude Code menemukan aturan `permissions.allow` atau entri `permissions.additionalDirectories` dalam `.claude/settings.json` atau `.claude/settings.local.json` proyek dan tidak menerapkannya, karena [aturan allow dari pengaturan proyek memerlukan kepercayaan workspace](/docs/id/permissions#project-allow-rules-and-workspace-trust). Jumlah, nama pengaturan, dan file yang dinamai dalam pesan bervariasi dengan konfigurasi Anda. Aturan `deny` dan `ask` tidak terpengaruh.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Yang harus dilakukan:**

* Jalankan `claude` di direktori dan terima dialog kepercayaan. [Project allow rules and workspace trust](/docs/id/permissions#project-allow-rules-and-workspace-trust) mengatakan folder mana yang dicakup oleh penerimaan tersebut.
* Dalam [mode non-interaktif](/docs/id/headless) dengan `-p` tidak ada dialog yang ditampilkan. Atur entri `hasTrustDialogAccepted` dalam `~/.claude.json` menggunakan kunci `projects` yang tepat yang dicetak pesan.
* Jika pesan menyebutkan `.claude/settings.local.json` dan Anda memulai Claude Code di luar repositori git atau di direktori home Anda, perbarui ke v2.1.200 atau lebih baru. Versi 2.1.196 hingga 2.1.199 memperlakukan `.claude/settings.local.json` Anda sendiri sebagai yang disediakan repositori di workspace tersebut. Pada v2.1.207 dan lebih baru, pembaruan tidak cukup di luar repositori git jika Anda belum mempercayai folder: menentukan bahwa folder tidak berada di dalam repositori menjalankan git, dan Claude Code menjalankan pemeriksaan itu hanya setelah Anda menerima dialog kepercayaan, jadi gunakan langkah pertama. Direktori home Anda dan [configuration home](/docs/id/permissions#project-allow-rules-and-workspace-trust) lainnya dikecualikan dan tidak menunggu dialog. Lihat [Project allow rules and workspace trust](/docs/id/permissions#project-allow-rules-and-workspace-trust).

<h3 id="working-directory-is-a-network-path">
  Direktori kerja adalah jalur jaringan
</h3>

Claude Code tidak menambahkan jalur jaringan sebagai direktori kerja. Mencari jalur jaringan dapat menghubungi host yang dinamainya, dan di Windows kontak itu dapat mengirim kredensial Anda ke host, jadi Claude Code menolak jalur tanpa mencarinya. Anda melihat pesan ini ketika Anda menjalankan `/add-dir` dengan jalur seperti itu, atau sebagai peringatan pada startup. Ketika muncul pada startup, Claude Code memulai tanpa direktori itu.

```text theme={null}
\\server\share is a network path, which cannot be added as a working directory. On Windows, map the share to a drive letter and pass it at launch with --add-dir (a drive letter added mid-session does not yet carry remote-read trust).
```

Jalur yang Claude Code tolak dengan cara ini termasuk:

* Berbagi UNC seperti `\\server\share`
* Jalur automount seperti `/net/<host>`, kecuali Anda meluncurkan Claude Code dari direktori di bawah automount host itu
* Jalur lokal yang mencapai lokasi jaringan melalui symbolic link atau junction

Huruf drive yang dipetakan dan jalur `\\wsl$` tidak dihitung sebagai jalur jaringan.

**Yang harus dilakukan:**

* Di Windows, petakan berbagi ke huruf drive, misalnya dengan `net use Z: \\server\share`, dan teruskan drive pada peluncuran dengan `claude --add-dir Z:\`.
* Di macOS atau Linux, pasang berbagi di jalur lokal dan tambahkan jalur itu sebagai gantinya.
* Jika jalur ada di `permissions.additionalDirectories`, hapus dari file pengaturan yang mencantumnya.

Sebelum v2.1.257, Claude Code menerima jalur jaringan yang dapat dijangkau sebagai direktori kerja.

<h3 id="remote-managed-settings-failed-to-load">
  Pengaturan yang dikelola jarak jauh gagal dimuat
</h3>

Sesi Anda memenuhi syarat untuk [server-managed settings](/docs/id/server-managed-settings), tetapi Claude Code tidak dapat mengambilnya, jadi menampilkan peringatan ini dalam sesi interaktif. Penyebab dalam tanda kurung menyebutkan apa yang gagal, seperti `network error`, `request timed out`, atau `authentication rejected (401)`, dan sisa baris mengatakan kebijakan mana yang dijalankan sesi:

* **Pengaturan di-cache dari pengambilan sebelumnya yang berhasil**: Claude Code menjalankan sesi pada kebijakan yang di-cache itu, kecuali [variabel lingkungan yang ditahan](/docs/id/server-managed-settings#fetch-and-caching-behavior), dan baris berbunyi `using cached policy`.
* **Tidak ada cache**: Claude Code menjalankan sesi tanpa server-managed settings, dan baris berbunyi `no remote policy applied`.

**Yang harus dilakukan:**

* Bertindak atas penyebab yang dinamai pesan: untuk penyebab jaringan, periksa bahwa mesin ini dapat menjangkau `api.anthropic.com`; untuk penyebab autentikasi, periksa sign-in Anda dengan `/status`
* Jalankan `/status` atau `claude doctor` untuk diagnostik lengkap

Sebelum v2.1.248, Claude Code melaporkan pengambilan pengaturan yang gagal hanya dalam log debug.

<h3 id="managed-settings-were-not-approved">
  Pengaturan yang dikelola tidak disetujui
</h3>

[Server-managed settings](/docs/id/server-managed-settings) organisasi Anda mencakup pengaturan yang memerlukan persetujuan Anda, dan Anda menolak [dialog persetujuan keamanan](/docs/id/server-managed-settings#security-approval-dialogs), jadi Claude Code keluar tanpa menerapkannya:

```text theme={null}
Managed settings were not approved; exiting without applying them.
```

**Yang harus dilakukan:**

* Mulai Claude Code lagi dan setujui dialog untuk melanjutkan di bawah pengaturan organisasi Anda. Dialog yang ditolak tidak diingat, jadi muncul lagi pada startup berikutnya.
* Jika Anda tidak yakin tentang pengaturan yang dialog cantumkan, tanyakan kepada siapa pun yang memelihara pengaturan yang dikelola organisasi Anda sebelum menyetujui

<h3 id="mcp-server-is-blocked-by-enterprise-managed-policy">
  Server MCP diblokir oleh kebijakan yang dikelola perusahaan
</h3>

Anda memilih **Reconnect** pada server di `/mcp`, atau mengaktifkan kembali server yang dinonaktifkan di sana, dan pengaturan yang [membatasi server MCP](/docs/id/managed-mcp) memblokir server itu. Claude Code menolak untuk menghubungkannya dan menampilkan:

```text theme={null}
MCP server <name> is blocked by enterprise managed policy
```

Salah satu dari pengaturan ini dapat menghasilkan pesan:

* Entri [`deniedMcpServers`](/docs/id/managed-mcp#policy-based-control-with-allowlists-and-denylists) yang cocok dengan server, termasuk yang ada di `~/.claude/settings.json` Anda sendiri atau `.claude/settings.json` proyek
* Daftar [`allowedMcpServers`](/docs/id/managed-mcp#policy-based-control-with-allowlists-and-denylists) yang tidak cocok dengan server
* [`strictPluginOnlyCustomization`](/docs/id/settings-reference#strictpluginonlycustomization) dengan `mcp` terkunci, yang memblokir server yang dikonfigurasi dalam `~/.claude.json` dan `.mcp.json`
* [`disableClaudeAiConnectors`](/docs/id/mcp#disable-claude-ai-connectors), ketika server adalah konektor claude.ai

**Yang harus dilakukan:**

* Periksa file pengaturan pengguna dan proyek Anda sendiri untuk salah satu pengaturan ini dan ubah atau hapus
* Jika tidak ada pengaturan Anda sendiri yang menjelaskan blokir, tanyakan administrator Anda pengaturan yang dikelola mana yang memblokir server

Sebelum v2.1.257, **Reconnect** dan re-enable di `/mcp` dapat menghubungkan server yang pembaruan kebijakan mid-session memblokir.

<h3 id="managed-settings-document-could-not-be-parsed">
  Dokumen pengaturan yang dikelola tidak dapat diuraikan
</h3>

Organisasi Anda menerapkan [managed settings](/docs/id/managed-settings), dan salah satu dokumen yang diterapkan ada tetapi tidak dapat diuraikan sebagai objek JSON, jadi Claude Code keluar dengan kode 1 pada startup daripada berjalan tanpa kebijakan yang dibawa dokumen. Baris menyebutkan sumber yang gagal sebelum pesan:

```text theme={null}
/Library/Application Support/ClaudeCode/managed-settings.json: Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.
```

Sumbernya adalah salah satu dari:

* Jalur file `managed-settings.json` atau file drop-in di bawah `managed-settings.d`
* Profil preferensi yang dikelola macOS, `per-user managed preferences` atau `device-level managed preferences`
* Nilai registry Windows, `Registry: HKLM\SOFTWARE\Policies\ClaudeCode\Settings`

[Find entries Claude Code dropped](/docs/id/managed-settings#find-entries-claude-code-dropped) mencantumkan apa yang membuat setiap sumber tidak dapat diuraikan.

Claude Code menolak untuk memulai bahkan ketika sumber admin lain memberikan kebijakan yang valid. Anda melihat kesalahan ini dalam sesi interaktif, `claude -p`, sesi Agent SDK, [background sessions](/docs/id/agent-view), dan sebagian besar subperintah, `claude doctor` termasuk. Penolakan gagal tertutup dengan sengaja: pengaturan dalam dokumen yang tidak dapat diuraikan Claude Code tidak dapat ditegakkan, dan memulai bagaimanapun akan menjalankan sesi tanpa kontrol organisasi Anda.

Masalah skema dalam dokumen yang dapat diuraikan tidak menghasilkan kesalahan ini. [Find entries Claude Code dropped](/docs/id/managed-settings#find-entries-claude-code-dropped) mencakup apa yang Claude Code lakukan dengan satu.

Ketika direktori `managed-settings.d/` ada tetapi tidak dapat didaftar, Claude Code melaporkan `Managed settings drop-in directory could not be read:` diikuti oleh kesalahan yang mendasar sebagai gantinya. [Find entries Claude Code dropped](/docs/id/managed-settings#find-entries-claude-code-dropped) mencakup kapan kegagalan baca keluar pada startup.

**Yang harus dilakukan:**

* Jika Anda mengelola mesin, perbaiki dokumen yang dinamai sehingga diuraikan sebagai objek JSON, atau hapus file, profil, atau nilai registry. `managed-settings.json` kosong dihitung sebagai `{}` dan tidak memblokir peluncuran.
* Jika tidak, minta administrator Anda untuk memperbaiki dokumen yang diterapkan. Tidak ada dalam file pengaturan Anda sendiri yang menyebabkan atau menghapus kesalahan ini.

<h3 id="otelheadershelper-failed">
  otelHeadersHelper gagal
</h3>

Claude Code menampilkan peringatan ini sebagai notifikasi dalam antarmuka terminal, sekali per sesi interaktif, ketika skrip [`otelHeadersHelper`](/docs/id/settings-reference#otelheadershelper) gagal atau mencetak output yang tidak memenuhi [persyaratan skrip](/docs/id/monitoring-usage#script-requirements).

Sementara skrip terus gagal, ekspor gagal dan backend telemetri Anda tidak menerima apa pun dari sesi.

Teks setelah `See /status:` mengatakan apa yang gagal, seperti kode keluar skrip diikuti oleh output kesalahannya:

```text theme={null}
otelHeadersHelper failed; telemetry is not being exported. See /status: exited 1: token service unreachable
```

**Yang harus dilakukan:**

* Jalankan `/status` untuk membaca detail kegagalan.
* Perbaiki skrip sehingga keluar 0 dalam 30 detik dan mencetak objek JSON dari nilai header string di stdout. Lihat [script requirements](/docs/id/monitoring-usage#script-requirements).
* Jika organisasi Anda menerapkan skrip melalui [managed settings](/docs/id/managed-settings), minta siapa pun yang memeliharanya untuk memperbaikinya.

Dalam [mode non-interaktif](/docs/id/headless) dengan `-p`, kegagalan yang sama muncul di stderr sebagai `otelHeadersHelper failed (OpenTelemetry export headers unavailable): <error>` sebagai gantinya.

<h3 id="headershelper-not-run">
  headersHelper tidak dijalankan
</h3>

Claude Code menghubungkan server MCP dengan `headers` statis saja dan melewati [`headersHelper`](/docs/id/mcp#use-dynamic-headers-for-custom-authentication) server, karena helper adalah perintah shell dan folder tidak memiliki kepercayaan yang disimpan. Folder mendapatkan kepercayaan yang disimpan ketika Anda mengatur entrinya di `~/.claude.json` dengan tangan atau, di luar direktori home Anda, ketika Anda menerima dialog kepercayaan untuk itu dalam sesi interaktif. Lihat [Trust a folder before its headersHelper runs](/docs/id/mcp#trust-a-folder-before-its-headershelper-runs) untuk server mana pemeriksaan ini berlaku.

Claude Code menulis baris ini dalam [mode non-interaktif](/docs/id/headless) saja, sekali per server. Dalam sesi interaktif, itu menulis penolakan yang sama ke log debug sebagai gantinya.

```text theme={null}
MCP server 'internal-api': headersHelper not run — this workspace has no persisted trust; accept the trust dialog here once interactively, or set projects["/Users/you/project"].hasTrustDialogAccepted in /Users/you/.claude.json.
```

Kunci `projects` yang dicetak pesan adalah folder yang [Project allow rules and workspace trust](/docs/id/permissions#project-allow-rules-and-workspace-trust) katakan Claude Code kunci kepercayaan. Menerima dialog kepercayaan untuk folder induk tidak memenuhi pemeriksaan, dan sesi `-p` atau SDK tidak memenuhinya juga.

**Yang harus dilakukan:**

* Jalankan `claude` di folder yang dinamai pesan, terima dialog kepercayaan, kemudian jalankan perintah `-p` atau SDK Anda lagi
* Atur entri `hasTrustDialogAccepted` dalam `~/.claude.json` sendiri, menggunakan kunci `projects` yang tepat yang dicetak pesan
* Jika Anda memulai sesi di direktori home Anda, bekerja dari direktori proyek yang telah Anda percayai. Ketika Anda menerima dialog kepercayaan di direktori home Anda, Claude Code menyimpan kepercayaan itu untuk sesi saat ini saja.

<h3 id="malformed-tool-content-rule">
  Aturan Tool(content) yang salah format
</h3>

[Aturan izin](/docs/id/permissions#permission-rule-syntax) dalam salah satu file pengaturan Anda tidak memiliki bentuk `Tool` atau `Tool(content)`, misalnya karena teks mengikuti tanda kurung penutup atau salah satu tanda kurung hilang. Claude Code melewati aturan dan mencantumnya dalam dialog pengaturan yang tidak valid ketika sesi interaktif dimulai, dan dalam output [`claude doctor`](/docs/id/debug-your-config#check-resolved-settings):

```text theme={null}
Invalid permission rule "Bash(ls) x" was skipped: Malformed Tool(content) rule. Rules take the form Tool or Tool(content) and must end at the closing ")"; parentheses inside the content are literal
```

**Yang harus dilakukan:**

* Dalam file pengaturan yang tercantum dengan pesan, tulis ulang aturan sehingga berakhir pada tanda kurung penutupnya, misalnya `Bash(ls *)` sebagai pengganti `Bash(ls) x`
* Biarkan tanda kurung di dalam konten apa adanya. Mereka literal, jadi aturan seperti `Edit(./Finance (2024)/**)` valid tanpa escaping

Sebelum v2.1.260, Claude Code melaporkan aturan dengan tanda kurung yang tidak cocok sebagai `Mismatched parentheses`.

<h3 id="is-not-matched-by-file-permission-checks">
  Tidak cocok dengan pemeriksaan izin file
</h3>

Claude Code menemukan aturan [izin](/docs/id/permissions#read-and-edit) `Write`, `NotebookEdit`, `MultiEdit`, atau `Glob` dengan jalur dalam salah satu [file pengaturan](/docs/id/settings#where-settings-live) Anda, dalam [managed settings](/docs/id/managed-settings), atau dalam nilai flag `--allowedTools`, `--disallowedTools`, atau `--settings`. Itu memeriksa izin file terhadap aturan `Edit` dan `Read` saja, jadi tidak pernah berkonsultasi dengan aturan jalur yang menyebutkan salah satu alat file lainnya. Itu menyimpan aturan dan tidak mengubah apa pun yang lain; peringatan menyebutkan aturan, sumbernya dalam tanda kurung, dan penggantian untuk ditulis:

```text theme={null}
Permission deny rule (.claude/settings.json): Write(docs/**) is not matched by file permission checks — only Edit(path) rules are. Use Edit(docs/**) instead (Edit rules cover all file-editing tools).
```

**Yang harus dilakukan:**

* Ganti aturan `Write(path)`, `NotebookEdit(path)`, dan `MultiEdit(path)` warisan dengan `Edit(path)`. Aturan `Edit` mencakup semua alat pengeditan file.
* Kecuali dalam `--allowedTools`, di mana Claude Code menerima aturan `Glob` tanpa peringatan, ganti aturan `Glob(path)` dengan `Read(path)`.
* Perbaiki aturan di sumber yang dinamai peringatan dalam tanda kurung: jalur file pengaturan, atau flag itu sendiri untuk `--allowed-tools` dan `--disallowed-tools`. Jalur `claude-settings-<hash>.json` yang tidak ada di disk mewakili nilai `--settings` inline. Perbaiki JSON yang Anda teruskan ke flag itu.
* Biarkan aturan nama alat bare seperti `Write` atau `Glob` saja. Claude Code mencocokkannya di [tingkat alat](/docs/id/permissions#match-all-uses-of-a-tool) dan tidak memperingatkan tentang mereka.
* Jika sumber berbunyi `managed policy settings`, teruskan peringatan kepada siapa pun yang memelihara pengaturan yang dikelola Anda, karena Anda tidak dapat menghapusnya sendiri.

Dalam [background session](/docs/id/agent-view) atau dengan `--output-format json` atau `stream-json`, Claude Code menulis peringatan ke log debug daripada stderr, jadi output yang dibaca mesin tetap bersih. Jalankan dengan `--debug` untuk menangkapnya di `~/.claude/debug/<session-id>.txt`. Sebelum v2.1.210, Claude Code menerima aturan ini tanpa peringatan.

<h3 id="has-a-wildcard-before-the-rest-of-the-command">
  Memiliki wildcard sebelum sisa perintah
</h3>

Claude Code menemukan aturan allow `Bash` yang `*` nya datang sebelum kata belakangan yang menentukan perintah mana itu, seperti `Bash(git * main)` atau `Bash(git -C * status *)`, dalam salah satu [file pengaturan](/docs/id/settings#where-settings-live) Anda, dalam [managed settings](/docs/id/managed-settings), atau dalam nilai flag `--allowedTools` atau `--settings`. `*` cocok dengan teks apa pun, termasuk opsi yang disisipkan pada posisi itu: `Bash(git * main)` juga menyetujui `git -c core.fsmonitor=<script> diff main`, di mana `-c` membuat git menjalankan program yang dinamai perintah. [Wildcard patterns](/docs/id/permissions#wildcard-patterns) menunjukkan aturan pencocokan.

Peringatan ada sehingga Anda dapat mempersempit aturan yang wildcard-nya lebih luas dari yang Anda maksudkan. Claude Code menyimpan aturan dan tidak mengubah apa pun tentang cara pencocokannya; peringatan menyebutkan aturan dan sumbernya dalam tanda kurung:

```text theme={null}
Permission allow rule (.claude/settings.json): Bash(git -C * status *) has a wildcard before the rest of the command, so it also matches any options inserted at that position and approves them without a prompt. For git, options such as -c and --exec-path can run arbitrary commands. Replace that * with the exact value you mean, or only use * after the subcommand (for example Bash(git status *)).
```

**Yang harus dilakukan:**

* Ganti `*` sebelum subperintah dengan nilai yang tepat yang Anda maksudkan: `Bash(git checkout main)` sebagai pengganti `Bash(git * main)`.
* Pindahkan setiap `*` setelah subperintah: `Bash(git status *)` sebagai pengganti `Bash(git -C * status *)`. Tulis satu aturan per subperintah yang ingin Anda izinkan.
* Perbaiki aturan di sumber yang dinamai peringatan dalam tanda kurung: jalur file pengaturan, atau flag `--allowed-tools` itu sendiri. Jalur `claude-settings-<hash>.json` yang tidak ada di disk mewakili nilai `--settings` inline. Perbaiki JSON yang Anda teruskan ke flag itu.
* Jika sumber berbunyi `managed policy settings`, teruskan peringatan kepada siapa pun yang memelihara pengaturan yang dikelola Anda, karena Anda tidak dapat menghapusnya sendiri.

Claude Code tidak memperingatkan tentang aturan deny dan ask dengan bentuk yang sama: itu menolak atau meminta perintah tambahan yang mereka cocokkan daripada menyetujuinya. Itu juga tidak memperingatkan tentang aturan yang subperintahnya datang sebelum `*` pertama, seperti `Bash(git commit *)`, atau aturan di mana tidak ada kata selain opsi yang mengikuti `*`, seperti `Bash(git *)`, atau tentang aturan awalan `:*` seperti `Bash(git:*)`.

Dalam [background session](/docs/id/agent-view) atau dengan `--output-format json` atau `stream-json`, Claude Code menulis peringatan ke log debug daripada stderr, jadi output yang dibaca mesin tetap bersih. Jalankan dengan `--debug` untuk menangkapnya di `~/.claude/debug/<session-id>.txt`. Sebelum v2.1.246, Claude Code menerima aturan ini tanpa peringatan.

<h3 id="crosssessioninbound-must-be-one-of-accept-hold-refuse">
  crossSessionInbound harus salah satu dari accept, hold, refuse
</h3>

File pengaturan menetapkan [`crossSessionInbound`](/docs/id/settings-reference#crosssessioninbound) ke nilai yang tidak dikenali Claude Code, seperti typo `"reject"`. Kalimat kedua peringatan tergantung pada file mana yang menyimpan nilai; dalam file pengguna, proyek, lokal, atau `--settings` berbunyi:

```text theme={null}
"crossSessionInbound" must be one of "accept", "hold", "refuse"; received "reject". This value was ignored; while it is present, cross-session messages are held for your approval instead of being delivered. Set it to one of the values above.
```

Dalam [managed settings](/docs/id/managed-settings), Claude Code memperlakukan nilai yang tidak dikenali sebagai `refuse`, nilai paling ketat, dan peringatan mengatakan pesan lintas sesi ditolak sampai administrator memperbaikinya. Untuk cara hold menggabungkan dengan nilai dalam file pengaturan lainnya, lihat [`crossSessionInbound`](/docs/id/settings-reference#crosssessioninbound).

**Yang harus dilakukan:**

* Atur kunci ke `"accept"`, `"hold"`, atau `"refuse"`, atau hapus
* Ketika peringatan menyebutkan managed settings, minta administrator untuk memperbaiki nilai

Sebelum v2.1.248, Claude Code mengabaikan nilai yang tidak dikenali tanpa peringatan.

<h3 id="the-200k-limit-isnt-enforced">
  Batas 200K tidak ditegakkan
</h3>

Anda menetapkan [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/id/env-vars), yang biasanya membuat [auto-compaction](/docs/id/model-config#default-auto-compact-thresholds) menahan sesi pada model konteks 1M ke jendela 200K, tetapi tidak ada ambang batas compaction yang membatasi sesi ini pada atau di bawah 200K, jadi percakapan dapat tumbuh melampaui itu.

```text theme={null}
CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced for <model>, so this session can grow past it. To enforce it, set CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000 (or the autoCompactWindow setting).
```

Claude Code menegakkan batas 200K sendiri untuk setiap model yang dikenalinya sebagai memiliki jendela 1M asli, dan untuk ID model yang tidak dikenalinya, itu compacts pada jendela yang diasumsikan. Peringatan muncul ketika konfigurasi lain mengalahkan penegakan itu:

* ID model bukan yang dikenali Claude Code, seperti alias [LLM gateway](/docs/id/llm-gateway), dan Anda menetapkan [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/id/env-vars) atau menaikkan jendela yang diasumsikan melampaui 200K dengan [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/id/env-vars). Dalam hal ini pesan juga menawarkan `or update to a Claude Code version that recognizes <model>` sebagai obat.
* Beta `context-1m` yang diminta melalui [`ANTHROPIC_BETAS`](/docs/id/env-vars) atau flag [`--betas`](/docs/id/cli-reference#cli-flags) masih meminta API untuk jendela 1M pada model yang menerima beta itu, sementara tidak ada yang compacts sesi pada 200K

**Yang harus dilakukan:**

* Atur [`CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000`](/docs/id/env-vars), atau pengaturan [`autoCompactWindow`](/docs/id/settings-reference#autocompactwindow) ke `200000`, sehingga auto-compaction compacts pada batas 200K
* Jika pesan menyebutkan ID model yang versi ini tidak kenali, jalankan `claude update`. Versi yang mengenali ID sebagai model konteks 1M menegakkan batas tanpa konfigurasi lebih lanjut.
* Jika Anda ingin sesi menggunakan jendela penuh model sebagai gantinya, batalkan `CLAUDE_CODE_DISABLE_1M_CONTEXT`; peringatan melaporkan hanya bahwa batas 200K tidak ditegakkan

Dalam [background session](/docs/id/agent-view) atau dengan `--output-format json` atau `stream-json`, Claude Code menulis peringatan ke log debug daripada stderr.

<h3 id="unrecognized-model-id-on-a-request">
  ID model yang tidak dikenali pada permintaan
</h3>

Claude Code mengirim permintaan untuk ID model yang versi Claude Code Anda tidak kenali, dan tidak menemukan entri [`modelOverrides`](/docs/id/model-config#override-model-ids-per-version) yang memetakan ID itu ke model yang dikenalinya. Claude Code masih mengirim permintaan dengan ID seperti yang Anda konfigurasi, dan tidak keluar atau beralih model.

```text theme={null}
[claude-code:unrecognized_model] {"model":"my-proxy-model","query_source":"sdk"}
```

Dalam skrip atau harness yang membaca stderr, cocokkan pada awalan `[claude-code:unrecognized_model]`. Setelah awalan dan satu spasi, Claude Code menulis objek JSON satu baris. Claude Code dapat menambahkan bidang ke dalamnya dalam versi yang lebih baru, jadi abaikan bidang apa pun yang tidak Anda harapkan. Itu menulis setidaknya dua ini:

* `model`: string model seperti yang Anda konfigurasi
* `query_source`: jalur permintaan yang menggunakan model. Claude Code melaporkan `sdk` untuk jalankan `-p` dan nilai yang dimulai dengan `agent:` untuk subagent.

Claude Code menulis baris ke salah satu dari dua tempat, tergantung pada cara Anda menjalankannya:

* Dalam [mode non-interaktif](/docs/id/headless) dengan `-p`, Claude Code menulis ke stderr di bawah setiap `--output-format`, jadi Anda dapat menguraikan stdout tanpa memfilter baris keluar
* Dalam sesi interaktif atau [background session](/docs/id/agent-view), Claude Code menulis ke log debug sebagai gantinya; jalankan dengan `--debug` untuk menangkapnya di `~/.claude/debug/<session-id>.txt`

Claude Code menulis baris sekali per string model per proses. Itu menulis baris terpisah untuk setiap ID yang tidak dikenali lebih lanjut, seperti yang digunakan [subagent](/docs/id/sub-agents#choose-a-model) atau [background functionality](/docs/id/costs#background-token-usage).

Claude Code tidak menulis baris untuk ID penyedia yang diselesaikannya ke model yang dikenalinya, seperti Amazon Bedrock `us.anthropic.claude-...` ID, ID Agent Platform Google Cloud dengan akhiran versi `@`, dan nama deployment Microsoft Foundry yang berisi ID model Claude. Claude Code memeriksa model di balik [application inference profile ARN](/docs/id/amazon-bedrock#map-each-model-version-to-an-inference-profile) Amazon Bedrock daripada ARN itu sendiri. Itu tidak menulis baris untuk ARN yang tidak dapat diselesaikannya, seperti yang salah ketik.

**Yang harus dilakukan:**

* Jika Anda menetapkan ID dengan sengaja, seperti alias [LLM gateway](/docs/id/llm-gateway), tambahkan entri [`modelOverrides`](/docs/id/model-config#override-model-ids-per-version) ke [file pengaturan](/docs/id/settings#where-settings-live) Anda dengan ID sebagai nilainya. Gunakan ID model Anthropic sebagai kunci, bukan alias keluarga seperti `opus`. Untuk `my-proxy-model` dari baris contoh, tambahkan entri ini:

  ```json theme={null}
  {
    "modelOverrides": {
      "claude-opus-4-6": "my-proxy-model"
    }
  }
  ```

  Claude Code kemudian memperlakukan `my-proxy-model` sebagai `claude-opus-4-6` dan berhenti menulis baris.

* Jika ID menyebutkan model yang lebih baru dari versi Claude Code Anda, jalankan `claude update`

* Jika ID adalah typo, perbaiki di mana pun dari [tempat Anda dapat menetapkan model](/docs/id/model-config#setting-your-model) atau [variabel alias](/docs/id/model-config#environment-variables) yang menyimpannya. Jika `query_source` dimulai dengan `agent:`, perbaiki di mana Anda menetapkan [model subagent](/docs/id/sub-agents#choose-a-model) sebagai gantinya.

Sebelum v2.1.233, Claude Code tidak menulis baris ketika mengirim permintaan untuk ID model yang tidak dikenalinya.

<h3 id="stale-sandbox-mask-files-left-by-a-killed-session">
  File mask sandbox basi ditinggalkan oleh sesi yang dibunuh
</h3>

`claude doctor` mencetak peringatan ini dalam diagnostiknya, dan `/status` mencantumkan baris yang sama. Itu muncul di Linux dan WSL2 ketika [sandboxing](/docs/id/sandboxing) diaktifkan dengan isolasi filesystem aktif.

Sementara perintah sandboxed berjalan, sandbox menyimpan penolakan tulis pada file yang belum ada dengan membuat placeholder read-only 0-byte di sana, dan menghapusnya setelahnya. Sesi yang dibunuh sebelum pembersihan itu berjalan, misalnya oleh SIGKILL, meninggalkan placeholder di belakang. Sesi-sesi kemudian mengikatnya read-only lagi pada setiap start, jadi penulisan pengaturan seperti menyimpan "Yes, and don't ask again" gagal di mana satu duduk.

```text theme={null}
- Stale sandbox mask files left by a killed session: /home/you/project/.claude/settings.local.json
  Fix: Remove each with `rm <path>` while no other Claude Code session is running in that project — a 0-byte read-only file where a settings file belongs makes "Yes, and don't ask again" fail to save, and the sandbox binds it read-only again on every start
```

**Yang harus dilakukan:**

* Keluar dari sesi Claude Code lain yang berjalan di proyek itu, kemudian hapus setiap file yang tercantum dengan `rm`. Peringatan menyebutkan hingga tiga file dan menghitung sisanya, jadi jalankan kembali `claude doctor` setelah menghapus sampai peringatan tidak lagi muncul. Placeholder yang sandbox sesi lain masih gunakan adalah bagian hidup dari perlindungan tulis sesi itu
* Jika pilihan izin yang Anda simpan dengan "Yes, and don't ask again" tidak tetap, simpan lagi setelah menghapus placeholder

Sebelum v2.1.257, `claude doctor` tidak menandai file-file ini; versi-versi sebelumnya meninggalkan placeholder yang sama di belakang ketika sesi dibunuh.

<h2 id="responses-seem-lower-quality-than-usual">
  Respons tampak berkualitas lebih rendah dari biasanya
</h2>

Jika jawaban Claude tampak kurang mampu dari yang Anda harapkan tetapi tidak ada kesalahan yang ditampilkan, penyebabnya biasanya adalah status percakapan daripada model itu sendiri. Claude Code tidak secara diam-diam mengubah versi model. Ini dapat beralih ke model fallback dalam tiga kasus spesifik:

* [`--fallback-model`](/docs/id/cli-reference#cli-flags) yang dikonfigurasi mengambil alih setelah kesalahan ketersediaan, hanya untuk giliran itu, dengan pemberitahuan dalam transkrip
* Pemeriksaan startup Amazon Bedrock atau Google Cloud's Agent Platform menemukan model default Anda tidak tersedia
* [Fallback model otomatis](/docs/id/model-config#automatic-model-fallback) pada Fable 5.1, Fable 5, Opus 5.5, dan Opus 5 memindahkan sesi ke model fallback kategori yang ditandai, ketika kategori itu memiliki satu, dan menampilkan pemberitahuan dalam transkrip

Pemeriksaan Model selection di bawah menangkap kasus kedua dan ketiga; yang pertama muncul sebagai pemberitahuan transkrip daripada perubahan `/model`. [Model configuration](/docs/id/model-config) menjelaskan kapan setiap fallback berlaku.

Periksa ini terlebih dahulu:

* **Model selection**: jalankan `/model` untuk mengonfirmasi Anda berada di model yang Anda harapkan. Pilihan `/model` sebelumnya atau variabel lingkungan `ANTHROPIC_MODEL` mungkin membuat Anda berada di model yang lebih kecil dari yang Anda maksudkan.
* **Effort level**: jalankan `/effort` untuk memeriksa tingkat penalaran saat ini dan naikkan untuk debugging atau pekerjaan desain yang sulit. Default bervariasi menurut model, jadi periksa sebelum menganggap Anda di bawah maksimum. Lihat [Adjust effort level](/docs/id/model-config#adjust-effort-level) untuk default per-model dan pintasan `ultrathink`.
* **Context pressure**: jalankan `/context` untuk melihat seberapa penuh jendela itu. Jika mendekati kapasitas, jalankan `/compact` pada titik alami atau `/clear` untuk memulai segar. Lihat [Explore the context window](/docs/id/context-window) untuk bagaimana auto-compact mempengaruhi giliran sebelumnya.
* **Stale instructions**: file `CLAUDE.md` yang besar atau ketinggalan zaman dan definisi alat MCP mengonsumsi konteks dan dapat mengarahkan respons. Pemeriksaan `/doctor` menandai file memori yang berukuran besar dan ekstensi yang tidak digunakan, dan `/context` menampilkan penggunaan token alat MCP. Sebelum v2.1.205, `/doctor` membuka layar diagnostik yang menandai file memori yang berukuran besar dan definisi subagent.

Ketika respons salah, rewinding biasanya bekerja lebih baik daripada membalas dengan koreksi. Tekan Esc dua kali atau jalankan `/rewind` untuk mundur ke sebelum giliran buruk, kemudian rephrase prompt dengan lebih spesifik. Mengoreksi dalam-thread menjaga upaya yang salah dalam konteks, yang dapat menambatkan jawaban nanti ke sana. Lihat [Checkpointing](/docs/id/checkpointing).

Jika kualitas masih tampak tidak benar setelah memeriksa di atas, jalankan `/feedback` dan jelaskan apa yang Anda harapkan versus apa yang Anda dapatkan. Umpan balik yang dikirimkan dengan cara ini mencakup transkrip percakapan, yang merupakan cara tercepat bagi Anthropic untuk mendiagnosis regresi nyata. Lihat [Report an error](#report-an-error) jika `/feedback` tidak tersedia di lingkungan Anda.

Jika Claude memperingatkan tentang injeksi prompt yang dicurigai, atau menolak permintaan karena injeksi yang dicurigai, dan teks yang dinamai peringatan adalah konteks yang Claude Code tambahkan ke percakapan secara otomatis daripada konten file atau web, jalankan `claude update` dan coba lagi. Jika peringatan berulang setelah memperbarui, [laporkan](#report-an-error) daripada menempel konten yang ditandai kembali ke prompt. Sebelum v2.1.201, Sonnet 5 menolak beberapa permintaan dengan cara yang sama.

<h2 id="report-an-error">
  Laporkan kesalahan
</h2>

Untuk kesalahan dari komponen yang tidak tercakup di halaman ini, lihat panduan yang relevan:

* Server MCP gagal terhubung atau autentikasi: [MCP](/docs/id/mcp)
* Skrip hook gagal atau memblokir alat: [Debug hooks](/docs/id/hooks#debug-hooks)
* Izin ditolak atau kesalahan sistem file selama instalasi: [Troubleshoot installation and login](/docs/id/troubleshoot-install)

Jika kesalahan tidak tercantum di sini atau perbaikan yang disarankan tidak membantu:

* Jalankan `/feedback` di dalam Claude Code untuk mengirimkan transkrip dan deskripsi ke Anthropic. Perintah ini juga menawarkan untuk membuka masalah GitHub yang sudah diisi sebelumnya. Pengiriman ke Anthropic memerlukan [authentication](/docs/id/authentication). Di Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, dan penyedia pihak ketiga lainnya, atau ketika tidak ada kredensial Anthropic yang dikonfigurasi, `/feedback` menyimpan arsip lokal yang dapat Anda kirimkan ke perwakilan akun Anthropic Anda.
* Jalankan `claude doctor` dari shell Anda untuk diagnostik sistem file read-only dari instalasi Anda, atau jalankan pemeriksaan `/doctor` di dalam Claude Code untuk menemukan dan memperbaiki masalah pengaturan
* Periksa [status.claude.com](https://status.claude.com) untuk insiden aktif
* Cari [existing issues](https://github.com/anthropics/claude-code/issues) di GitHub
