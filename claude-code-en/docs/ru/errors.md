> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Справочник по ошибкам

> Найдите сообщения об ошибках Claude Code с объяснением их значения и способов исправления.

На этой странице перечислены ошибки времени выполнения, которые отображает Claude Code, и способы восстановления после каждой из них, а также что проверить, когда ответы кажутся неправильными без ошибки. Для ошибок установки, таких как `command not found` или сбои TLS во время установки, см. [Устранение неполадок при установке и входе](/docs/ru/troubleshoot-install).

За исключением [ошибок Wrapper и IDE](#wrapper-and-ide-errors), которые выводит запускающая программа, а не сам Claude Code, эти ошибки и команды восстановления применяются во всех интерфейсах: CLI, [приложении Desktop](/docs/ru/desktop) и [Claude Code в веб-версии](/docs/ru/claude-code-on-the-web), поскольку все три используют один и тот же Claude Code CLI. Для других проблем, специфичных для конкретного интерфейса, см. раздел устранения неполадок на странице этого интерфейса.

<Note>
  Claude Code вызывает Claude API для получения ответов модели, поэтому большинство ошибок времени выполнения соответствуют базовому коду ошибки API. На этой странице описано, что означает каждая ошибка в Claude Code и как восстановиться. Для определений кодов состояния HTTP в исходном виде см. [справочник по ошибкам платформы Claude](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Найдите вашу ошибку
</h2>

Сопоставьте сообщение, которое вы видите, с разделом ниже.

| Сообщение                                                                                                                                                                                                                                                            | Раздел                                                                                                                          |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `API Error: 500 Internal server error`                                                                                                                                                                                                                               | [Server errors](#api-error-500-internal-server-error)                                                                           |
| `API Error: Repeated 529 Overloaded errors`                                                                                                                                                                                                                          | [Server errors](#api-error-repeated-529-overloaded-errors)                                                                      |
| `Request timed out`                                                                                                                                                                                                                                                  | [Server errors](#request-timed-out), или [Network](#unable-to-connect-to-api) если сообщение упоминает вашу интернет-соединение |
| `API Error: No response from API`                                                                                                                                                                                                                                    | [Server errors](#no-response-from-api)                                                                                          |
| `Server error mid-response. The response above may be incomplete.`                                                                                                                                                                                                   | [Server errors](#the-response-above-may-be-incomplete)                                                                          |
| `Connection lost mid-response` / `Your computer went to sleep mid-response` / `The response stopped arriving`                                                                                                                                                        | [Server errors](#the-response-above-may-be-incomplete)                                                                          |
| `Connection closed mid-response` / `Response stalled mid-stream`                                                                                                                                                                                                     | [Server errors](#the-response-above-may-be-incomplete)                                                                          |
| `Connection lost before a response was produced` / `Your computer went to sleep before a response was produced` / `The response stalled before a response was produced`                                                                                              | [Automatic retries](#automatic-retries)                                                                                         |
| `Connection closed while thinking` / `Response stalled while thinking`                                                                                                                                                                                               | [Automatic retries](#automatic-retries)                                                                                         |
| `Connection lost while your computer was asleep`                                                                                                                                                                                                                     | [Automatic retries](#automatic-retries)                                                                                         |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`                                                                                                                                                                                 | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                            |
| `Auto mode could not evaluate this action and is blocking it for safety`                                                                                                                                                                                             | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                            |
| `Auto mode classifier transcript exceeded context window`                                                                                                                                                                                                            | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                            |
| `Agent aborted: auto mode classifier request refused by the safety safeguard`                                                                                                                                                                                        | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                                                            |
| `The server-side auto mode classifier gave no verdict`                                                                                                                                                                                                               | [Server errors](#the-server-returned-no-safety-verdict)                                                                         |
| `Auto mode is unavailable — the server returned no safety verdict for the last 10 responses`                                                                                                                                                                         | [Server errors](#the-server-returned-no-safety-verdict)                                                                         |
| `Agent terminated early due to an API error`                                                                                                                                                                                                                         | [Server errors](#agent-terminated-early-due-to-an-api-error)                                                                    |
| `You've hit your session limit` / `You've hit your weekly limit` / `You've hit your Opus limit` / `You've hit your Sonnet limit`                                                                                                                                     | [Usage limits](#youve-hit-your-session-limit)                                                                                   |
| `Usage credits required for 1M context`                                                                                                                                                                                                                              | [Usage limits](#usage-credits-required-for-1m-context)                                                                          |
| `the prompt to confirm went unanswered — nothing was sent`                                                                                                                                                                                                           | [Usage limits](#the-prompt-to-confirm-went-unanswered)                                                                          |
| `Server is temporarily limiting requests`                                                                                                                                                                                                                            | [Usage limits](#server-is-temporarily-limiting-requests)                                                                        |
| `Request rejected (429)`                                                                                                                                                                                                                                             | [Usage limits](#request-rejected-429)                                                                                           |
| `Credit balance is too low`                                                                                                                                                                                                                                          | [Usage limits](#credit-balance-is-too-low)                                                                                      |
| `You've hit your monthly spend limit` / `You've hit your individual spend limit` / `You've hit your org's monthly spend limit` / `You've hit your channel's monthly spend limit` / `You've hit your team's shared budget` / `You've hit your individual usage limit` | [Usage limits](#youve-hit-your-monthly-spend-limit)                                                                             |
| `Could not update your spend limit`                                                                                                                                                                                                                                  | [Usage limits](#could-not-update-your-spend-limit)                                                                              |
| `spend limit reached` / `spend limit unavailable`                                                                                                                                                                                                                    | [Usage limits](#spend-limit-reached)                                                                                            |
| `Not logged in · Please run /login`                                                                                                                                                                                                                                  | [Authentication](#not-logged-in)                                                                                                |
| `Could not resolve authentication method`                                                                                                                                                                                                                            | [Authentication](#could-not-resolve-authentication-method)                                                                      |
| `Invalid API key`                                                                                                                                                                                                                                                    | [Authentication](#invalid-api-key)                                                                                              |
| `Your apiKeyHelper script is failing`                                                                                                                                                                                                                                | [Authentication](#your-apikeyhelper-script-is-failing)                                                                          |
| `Invalid auth token · Fix external auth token`                                                                                                                                                                                                                       | [Authentication](#invalid-request-header-value)                                                                                 |
| `Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable`                                                                                                                                                                                                    | [Authentication](#invalid-request-header-value)                                                                                 |
| `Invalid request header from the environment · Fix the environment variable`                                                                                                                                                                                         | [Authentication](#invalid-request-header-value)                                                                                 |
| `This organization has been disabled`                                                                                                                                                                                                                                | [Authentication](#this-organization-has-been-disabled)                                                                          |
| `Your organization has disabled API key authentication`                                                                                                                                                                                                              | [Authentication](#your-organization-has-disabled-api-key-authentication)                                                        |
| `Your organization has disabled Claude subscription access`                                                                                                                                                                                                          | [Authentication](#your-organization-has-disabled-claude-subscription-access)                                                    |
| `Routines are disabled by your organization's policy`                                                                                                                                                                                                                | [Authentication](#routines-are-disabled-by-your-organizations-policy)                                                           |
| `Remote Control is only available when using Claude via api.anthropic.com`                                                                                                                                                                                           | [Authentication](#remote-control-requires-the-anthropic-api)                                                                    |
| `OAuth token refresh failed — run /login to re-authenticate`                                                                                                                                                                                                         | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                    |
| `JWT refresh failed: no OAuth token — run /login`                                                                                                                                                                                                                    | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                    |
| `Claude.ai login expired`                                                                                                                                                                                                                                            | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                    |
| `Claude.ai login was rejected — run /login, then /remote-control`                                                                                                                                                                                                    | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                    |
| `OAuth token unavailable — run /login to restore Remote Control`                                                                                                                                                                                                     | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                    |
| `Signed out of Claude — run /login, then /remote-control`                                                                                                                                                                                                            | [Authentication](#remote-control-couldnt-refresh-your-login)                                                                    |
| `signed-in claude.ai account or organization changed on this machine`                                                                                                                                                                                                | [Authentication](#remote-control-stopped-because-the-signed-in-account-changed)                                                 |
| `Remote Control stopped — the app running this session is now signed in to a different Claude account`                                                                                                                                                               | [Authentication](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)                   |
| `Remote Control stopped — the app running this session is signed out of Claude`                                                                                                                                                                                      | [Authentication](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)                   |
| `OAuth token revoked` / `OAuth token has expired`                                                                                                                                                                                                                    | [Authentication](#oauth-token-revoked-or-expired)                                                                               |
| `API Error: 401 Invalid authentication credentials`                                                                                                                                                                                                                  | [Authentication](#api-error-401-invalid-authentication-credentials)                                                             |
| `Login expired · Please run /login`                                                                                                                                                                                                                                  | [Authentication](#login-expired)                                                                                                |
| `Claude login not accepted · Run /login, then try again`                                                                                                                                                                                                             | [Authentication](#claude-login-not-accepted)                                                                                    |
| `Artifacts need a claude.ai login`                                                                                                                                                                                                                                   | [Authentication](#artifacts-need-a-claude-ai-login)                                                                             |
| `Not signed in to the Cloud gateway — run /login.`                                                                                                                                                                                                                   | [Authentication](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                        |
| `Administrator policy requires a Cloud gateway sign-in on this machine`                                                                                                                                                                                              | [Authentication](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                        |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                                                                                                                                                                                           | [Authentication](#login-expired)                                                                                                |
| `Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                            | [Authentication](#your-account-is-on-hold)                                                                                      |
| `Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                     | [Authentication](#your-account-is-on-hold)                                                                                      |
| `Anthropic profile login expired · Re-authenticate your Anthropic profile`                                                                                                                                                                                           | [Authentication](#anthropic-profile-login-expired)                                                                              |
| `Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile`                                                                                                                                                 | [Authentication](#anthropic-profile-login-expired)                                                                              |
| `does not meet scope requirement user:profile`                                                                                                                                                                                                                       | [Authentication](#oauth-scope-requirement)                                                                                      |
| `claude.ai rejected the session token` / `session token rejected`                                                                                                                                                                                                    | [Authentication](#claude-ai-rejected-the-session-token)                                                                         |
| `MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)`                                                                                                                                                                                       | [Authentication](#mcp-server-needs-you-to-sign-in-again)                                                                        |
| `rejected the credential from its headersHelper` / `rejected the Authorization header in its config`                                                                                                                                                                 | [Authentication](#mcp-server-needs-you-to-sign-in-again)                                                                        |
| `MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate`                                                                                                                                                                  | [Authentication](#mcp-server-needs-you-to-sign-in-again)                                                                        |
| `MCP server "<name>" requires re-authorization (token expired)`                                                                                                                                                                                                      | [Authentication](#mcp-server-needs-you-to-sign-in-again)                                                                        |
| `Issuer mismatch in authorization response (RFC 9207)`                                                                                                                                                                                                               | [Authentication](#issuer-mismatch-in-authorization-response)                                                                    |
| `Cloud gateway session expired — run /login to reconnect.`                                                                                                                                                                                                           | [Authentication](#cloud-gateway-session-expired)                                                                                |
| `Cloud gateway <url> no longer accepts this session`                                                                                                                                                                                                                 | [Authentication](#cloud-gateway-session-expired)                                                                                |
| `Sign-in timed out while waiting for you to continue. Try again.`                                                                                                                                                                                                    | [Authentication](#sign-in-timed-out-while-waiting-for-you-to-continue)                                                          |
| `AWS credentials expired or invalid`                                                                                                                                                                                                                                 | [Authentication](#aws-credentials-expired-or-invalid)                                                                           |
| `AWS authentication failed`                                                                                                                                                                                                                                          | [Authentication](#aws-authentication-failed)                                                                                    |
| `Google Cloud credentials expired or invalid`                                                                                                                                                                                                                        | [Authentication](#google-cloud-credentials-expired-or-invalid)                                                                  |
| `Google Cloud authentication failed`                                                                                                                                                                                                                                 | [Authentication](#google-cloud-authentication-failed)                                                                           |
| `Microsoft Foundry authentication failed`                                                                                                                                                                                                                            | [Authentication](#microsoft-foundry-authentication-failed)                                                                      |
| `Gateway refused the request`                                                                                                                                                                                                                                        | [Authentication](#gateway-refused-the-request)                                                                                  |
| `Could not load AWS credentials` / `Could not load Google Cloud credentials`                                                                                                                                                                                         | [Authentication](#could-not-load-aws-or-google-cloud-credentials)                                                               |
| `AWS default-chain credential resolve timed out`                                                                                                                                                                                                                     | [Authentication](#aws-default-chain-credential-resolve-timed-out)                                                               |
| `Timed out after 60s waiting for AWS`                                                                                                                                                                                                                                | [Authentication](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                         |
| `A request to AWS timed out. Check your network and proxy settings, then try again.`                                                                                                                                                                                 | [Authentication](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                         |
| `Could not load the default credentials` on Google Cloud's Agent Platform                                                                                                                                                                                            | [Authentication](#could-not-load-aws-or-google-cloud-credentials)                                                               |
| `Unable to connect to API`                                                                                                                                                                                                                                           | [Network](#unable-to-connect-to-api)                                                                                            |
| `Connection refused —` / `Can't reach the API server —` / `No internet route —` / `Couldn't connect through your proxy` / `Connection dropped`, каждое с кодом ошибки в скобках                                                                                      | [Network](#unable-to-connect-to-api)                                                                                            |
| `Unable to connect to Anthropic services` during setup                                                                                                                                                                                                               | [Network](#unable-to-connect-to-anthropic-services)                                                                             |
| `Socket is closed`                                                                                                                                                                                                                                                   | [Network](#socket-is-closed)                                                                                                    |
| `Waiting for API response · will retry in`                                                                                                                                                                                                                           | [Automatic retries](#automatic-retries), или [Network](#unable-to-connect-to-api) если это продолжается                         |
| `API returned an empty or malformed response`                                                                                                                                                                                                                        | [Network](#api-returned-an-empty-or-malformed-response)                                                                         |
| `Streaming response ended before any complete data was received`                                                                                                                                                                                                     | [Network](#streaming-response-ended-before-any-complete-data-was-received)                                                      |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"`                                                                                                                                                                   | [Network](#bedrock-streaming-response-has-an-unexpected-content-type)                                                           |
| `SSL certificate verification failed`                                                                                                                                                                                                                                | [Network](#ssl-certificate-errors)                                                                                              |
| `SSL certificate error (...)` during login or startup                                                                                                                                                                                                                | [Network](#ssl-certificate-errors)                                                                                              |
| `unable to get local issuer certificate`                                                                                                                                                                                                                             | [Network](#ssl-certificate-errors)                                                                                              |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                                                                                                                                                                                           | [Network](#host-not-allowed-in-a-cloud-session)                                                                                 |
| `proxy refused the connection`                                                                                                                                                                                                                                       | [Network](#the-proxy-refused-the-connection)                                                                                    |
| `403` with `This GraphQL query is not enabled for this session` in a cloud session                                                                                                                                                                                   | [GitHub proxy](/docs/ru/cloud-environments#github-proxy)                                                                             |
| `The cloud environments service returned an empty response` / `The cloud environments service returned a response in an unexpected format`                                                                                                                           | [Network](#the-cloud-environments-service-returned-an-empty-or-unexpected-response)                                             |
| `Couldn't reconnect to your Remote Control session`                                                                                                                                                                                                                  | [Network](#couldnt-reconnect-to-your-remote-control-session)                                                                    |
| `N sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.`                                                                                                                                               | [Network](#sessions-ended-while-this-machine-was-offline)                                                                       |
| `Couldn't share the transcript.`                                                                                                                                                                                                                                     | [Network](#couldnt-share-the-transcript)                                                                                        |
| `Prompt is too long` / `Input is too long for requested model`                                                                                                                                                                                                       | [Request errors](#prompt-is-too-long)                                                                                           |
| `Prompt is too long · automatic compaction failed:`                                                                                                                                                                                                                  | [Request errors](#prompt-is-too-long)                                                                                           |
| `Prompt is too long · this conversation is a single exchange` / `A single-exchange conversation cannot be compacted`                                                                                                                                                 | [Request errors](#prompt-is-too-long)                                                                                           |
| `Context limit reached · /compact or /clear to continue`                                                                                                                                                                                                             | [Request errors](#prompt-is-too-long)                                                                                           |
| `Context limit reached · /clear to continue`                                                                                                                                                                                                                         | [Request errors](#prompt-is-too-long)                                                                                           |
| `capability_rejected: prompt_too_long` on a Claude apps gateway session                                                                                                                                                                                              | [Request errors](#prompt-is-too-long)                                                                                           |
| `upstream rejected the request` / `request too large for this upstream` on a Claude apps gateway session                                                                                                                                                             | [Upstream error messages](/docs/ru/claude-apps-gateway-config#upstream-error-messages)                                               |
| `upstream rate limit exceeded` on a Claude apps gateway session                                                                                                                                                                                                      | [Upstream error messages](/docs/ru/claude-apps-gateway-config#upstream-error-messages)                                               |
| `all upstreams failed (N attempted)` on a Claude apps gateway session                                                                                                                                                                                                | [Upstream error messages](/docs/ru/claude-apps-gateway-config#upstream-error-messages)                                               |
| `Claude Code may not be enabled for your organization` after a Claude apps gateway sign-in                                                                                                                                                                           | [Claude apps gateway troubleshooting](/docs/ru/claude-apps-gateway-deploy#troubleshooting)                                           |
| `Context exceeds the ...-token limit by ... tokens` in `/context` output                                                                                                                                                                                             | [Request errors](#context-exceeds-the-token-limit)                                                                              |
| `Error during compaction: Conversation too long`                                                                                                                                                                                                                     | [Request errors](#error-during-compaction-conversation-too-long)                                                                |
| `Request too large`                                                                                                                                                                                                                                                  | [Request errors](#request-too-large)                                                                                            |
| `Request too large for the API's 32MB request limit`                                                                                                                                                                                                                 | [Request errors](#request-too-large)                                                                                            |
| `Image was too large`                                                                                                                                                                                                                                                | [Request errors](#image-was-too-large)                                                                                          |
| `Unable to resize image`                                                                                                                                                                                                                                             | [Request errors](#unable-to-resize-image)                                                                                       |
| `PDF too large` / `PDF is password protected`                                                                                                                                                                                                                        | [Request errors](#pdf-errors)                                                                                                   |
| `Extra inputs are not permitted`                                                                                                                                                                                                                                     | [Request errors](#extra-inputs-are-not-permitted)                                                                               |
| `API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid` / `Property keys should match pattern`                                                                                                                                                      | [Request errors](#tool-input-schema-is-invalid)                                                                                 |
| `There's an issue with the selected model`                                                                                                                                                                                                                           | [Request errors](#theres-an-issue-with-the-selected-model)                                                                      |
| `Model ... is not a recognized model id`                                                                                                                                                                                                                             | [Request errors](#model-is-not-a-recognized-model-id)                                                                           |
| `Model ... not found`                                                                                                                                                                                                                                                | [Request errors](#model-not-found)                                                                                              |
| `Claude Opus is not available with the Claude Pro plan`                                                                                                                                                                                                              | [Request errors](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                        |
| `Claude Code ... does not support this model; version ... or newer is required`                                                                                                                                                                                      | [Request errors](#claude-code-does-not-support-this-model)                                                                      |
| `Claude Code ... is older than the minimum version required by your organization's policy`                                                                                                                                                                           | [Request errors](#claude-code-does-not-support-this-model)                                                                      |
| `Model ... is restricted by your organization's settings`                                                                                                                                                                                                            | [Request errors](#model-is-restricted-by-your-organizations-settings)                                                           |
| `Model switch ... blocked by a PreModelSwitch hook`                                                                                                                                                                                                                  | [Request errors](#model-switch-was-blocked-by-a-premodelswitch-hook)                                                            |
| `couldn't save it as your default` / `couldn't confirm it was saved as your default`                                                                                                                                                                                 | [Request errors](#couldnt-save-it-as-your-default)                                                                              |
| `thinking.type.enabled is not supported for this model`                                                                                                                                                                                                              | [Request errors](#thinking-type-enabled-is-not-supported-for-this-model)                                                        |
| `Effort '<level>' isn't available with thinking turned off on this model`                                                                                                                                                                                            | [Request errors](#effort-isnt-available-with-thinking-turned-off)                                                               |
| `effort '<level>' is not supported when thinking is disabled`                                                                                                                                                                                                        | [Request errors](#effort-isnt-available-with-thinking-turned-off)                                                               |
| `max_tokens must be greater than thinking.budget_tokens`                                                                                                                                                                                                             | [Request errors](#thinking-budget-exceeds-output-limit)                                                                         |
| `API Error: 400 due to tool use concurrency issues`                                                                                                                                                                                                                  | [Request errors](#tool-use-or-thinking-block-mismatch)                                                                          |
| `API Error: 400 orphaned tool_result in conversation history`                                                                                                                                                                                                        | [Request errors](#tool-use-or-thinking-block-mismatch)                                                                          |
| `API Error: 400 duplicate tool_use ID in conversation history`                                                                                                                                                                                                       | [Request errors](#tool-use-or-thinking-block-mismatch)                                                                          |
| `[Unsupported tool content removed]`                                                                                                                                                                                                                                 | [Request errors](#unsupported-tool-content-removed)                                                                             |
| `role 'system' must precede an 'assistant' message`                                                                                                                                                                                                                  | [Request errors](#role-system-must-precede-an-assistant-message)                                                                |
| `Invalid encrypted_content in search_result block` / `Invalid encrypted_index in text block` / `Failed to decrypt web search result content`                                                                                                                         | [Request errors](#invalid-encrypted-content-in-search-result-block)                                                             |
| `server_tool_use.name: Input should be` on every turn of a resumed session                                                                                                                                                                                           | [Request errors](#unsupported-tool-content-removed)                                                                             |
| `<model> can't help with this. Start a new session to continue`                                                                                                                                                                                                      | [Request errors](#usage-policy-refusal)                                                                                         |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`                                                                                                                                                                        | [Request errors](#usage-policy-refusal)                                                                                         |
| `<model>'s safeguards flagged this message`                                                                                                                                                                                                                          | [Request errors](#safety-measures-flagged-a-cybersecurity-topic)                                                                |
| `Opus 5.5's safeguards flagged this session`                                                                                                                                                                                                                         | [Request errors](#safety-measures-flagged-a-cybersecurity-topic)                                                                |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                                                                                                                                                                                    | [Request errors](#safety-measures-flagged-a-cybersecurity-topic)                                                                |
| `Installation was killed before it could finish (exit code 137)`                                                                                                                                                                                                     | [Installation errors](#installation-was-killed-before-it-could-finish)                                                          |
| `The connection dropped while downloading the update`                                                                                                                                                                                                                | [Installation errors](#the-connection-dropped-while-downloading-the-update)                                                     |
| `Download timed out: exceeded the total deadline`                                                                                                                                                                                                                    | [Installation errors](#the-connection-dropped-while-downloading-the-update)                                                     |
| `--bg and --print conflict`                                                                                                                                                                                                                                          | [Command-line errors](#command-line-errors)                                                                                     |
| `Cloud sessions cannot be created from a --restricted session`                                                                                                                                                                                                       | [Command-line errors](#cloud-sessions-cannot-be-created-from-a-restricted-session)                                              |
| `Cloud sessions are disabled by your organization's policy`                                                                                                                                                                                                          | [Command-line errors](#cloud-sessions-are-disabled-by-your-organizations-policy)                                                |
| `Couldn't verify your organization's policy for cloud sessions`                                                                                                                                                                                                      | [Command-line errors](#cloud-sessions-are-disabled-by-your-organizations-policy)                                                |
| `Error: --json-schema is not a valid JSON Schema`                                                                                                                                                                                                                    | [Command-line errors](#command-line-errors)                                                                                     |
| `Error: Invalid --agents configuration:`                                                                                                                                                                                                                             | [Command-line errors](#invalid-agents-configuration)                                                                            |
| `Error: Settings file exceeds the 2MiB limit`                                                                                                                                                                                                                        | [Command-line errors](#settings-file-exceeds-the-2mib-limit)                                                                    |
| `The current directory no longer exists (it was deleted or moved)` / `Can't read the current directory`                                                                                                                                                              | [Command-line errors](#the-current-directory-no-longer-exists)                                                                  |
| `Temp directory <dir> ... Refusing to use it` / `ENOSPC: no space left on device, mkdir '<dir>'`                                                                                                                                                                     | [Command-line errors](#temp-directory-refused-or-cannot-be-created)                                                             |
| `couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded`                                                                                                                                                                        | [Command-line errors](#directory-couldnt-be-resolved-to-a-real-location)                                                        |
| `Error: Workspace not trusted` when starting Remote Control                                                                                                                                                                                                          | [Command-line errors](#workspace-not-trusted-when-starting-remote-control)                                                      |
| `` `<flag>` before `remote-control` is not carried over to the sessions Remote Control starts ``                                                                                                                                                                     | [Command-line errors](#not-carried-over-to-the-sessions-remote-control-starts)                                                  |
| `` `claude import` is not yet available in this build ``                                                                                                                                                                                                             | [Command-line errors](#claude-import-is-not-yet-available-in-this-build)                                                        |
| `Could not read Claude Code config`                                                                                                                                                                                                                                  | [Command-line errors](#could-not-read-claude-code-config)                                                                       |
| `Could not import <server>: <reason>`                                                                                                                                                                                                                                | [Command-line errors](#could-not-import-a-server-from-claude-desktop)                                                           |
| `Cannot add MCP server to scope: managed`                                                                                                                                                                                                                            | [Command-line errors](#cannot-add-mcp-server-to-the-managed-scope)                                                              |
| `is Anthropic-hosted and doesn't support local OAuth`                                                                                                                                                                                                                | [Command-line errors](#anthropic-hosted-and-doesnt-support-local-oauth)                                                         |
| `Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes`                                                                                                                                                                                      | [Command-line errors](#cant-read-mcp-json)                                                                                      |
| `Server rejected the Authorization header minted by the configured headersHelper`                                                                                                                                                                                    | [Command-line errors](#server-rejected-the-authorization-header-minted-by-the-configured-headershelper)                         |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                                                                                                                                                                                             | [Command-line errors](#mcp-permission-prompt-tool-not-found)                                                                    |
| `OAuth callback port <port> is already in use — another process may be holding it`                                                                                                                                                                                   | [Command-line errors](#oauth-callback-port-is-already-in-use)                                                                   |
| `No available ports for OAuth redirect`                                                                                                                                                                                                                              | [Command-line errors](#no-available-ports-for-oauth-redirect)                                                                   |
| `Shell command failed for pattern "..."`, from `/security-review` or any skill that injects dynamic context                                                                                                                                                          | [Command-line errors](#security-review-fails-without-origin-head)                                                               |
| `Shell command permission check failed for pattern "..."`, from a skill that injects dynamic context                                                                                                                                                                 | [Command-line errors](#security-review-fails-without-origin-head)                                                               |
| ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``                                                                                                                                                                             | [Command-line errors](#security-review-fails-without-origin-head)                                                               |
| `Input must be provided either through stdin or as a prompt argument when using --print`                                                                                                                                                                             | [Command-line errors](#input-must-be-provided-when-using-print)                                                                 |
| `Error: Input contained only whitespace`                                                                                                                                                                                                                             | [Command-line errors](#input-contained-only-whitespace)                                                                         |
| `Blank prompt — the message was only whitespace, so nothing was sent to the model.`                                                                                                                                                                                  | [Command-line errors](#input-contained-only-whitespace)                                                                         |
| `Error: stream-json input carried over 256M characters with no newline`                                                                                                                                                                                              | [Command-line errors](#stream-json-input-carried-over-256m-characters-with-no-newline)                                          |
| `Unknown command: /<name>`, with or without a `Did you mean` suggestion                                                                                                                                                                                              | [Command-line errors](#unknown-command)                                                                                         |
| `Diff is too large for ultrareview` / `PR #<N> is too large for ultrareview`                                                                                                                                                                                         | [Command-line errors](#diff-is-too-large-for-ultrareview)                                                                       |
| `Could not find merge-base with <branch>`                                                                                                                                                                                                                            | [Command-line errors](#could-not-find-merge-base-with-the-base-branch)                                                          |
| `Your checkout has no branches (detached HEAD only)`                                                                                                                                                                                                                 | [Command-line errors](#your-checkout-has-no-branches)                                                                           |
| `Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected`                                                                                                                                     | [Command-line errors](#no-github-account-is-connected-to-your-claude-account)                                                   |
| `Your connected GitHub account can't see <owner>/<repo>`                                                                                                                                                                                                             | [Command-line errors](#your-connected-github-account-cant-see-the-repository)                                                   |
| `The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead`                                                                                                                                           | [Command-line errors](#the-github-app-preflight-failed-transiently)                                                             |
| `GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud`                                                                                                                                                                     | [Command-line errors](#github-isnt-connected-to-your-claude-account)                                                            |
| `Single sign-on authorization needed`                                                                                                                                                                                                                                | [Command-line errors](#single-sign-on-authorization-needed)                                                                     |
| `Failed to resume the conversation`                                                                                                                                                                                                                                  | [Command-line errors](#failed-to-resume-the-conversation)                                                                       |
| `No conversation found with session ID: <session-id>`                                                                                                                                                                                                                | [Command-line errors](#no-conversation-found-with-the-session-id)                                                               |
| `Cannot switch renderers in this session`                                                                                                                                                                                                                            | [Command-line errors](#cannot-switch-renderers-in-this-session)                                                                 |
| `Cannot switch renderers while work is running in the background`                                                                                                                                                                                                    | [Command-line errors](#cannot-switch-renderers-in-this-session)                                                                 |
| `Couldn't open Claude Desktop`                                                                                                                                                                                                                                       | [Command-line errors](#couldnt-open-claude-desktop)                                                                             |
| `Failed to open Claude Desktop. Please try opening it manually.`                                                                                                                                                                                                     | [Command-line errors](#couldnt-open-claude-desktop)                                                                             |
| `Couldn't read your Zed keymap` / `Couldn't back up your Zed keymap` / `Couldn't update your Zed keymap`                                                                                                                                                             | [Command-line errors](#terminal-setup-left-your-zed-keymap-unchanged)                                                           |
| `Your Zed keymap isn't a readable list of keybindings`                                                                                                                                                                                                               | [Command-line errors](#terminal-setup-left-your-zed-keymap-unchanged)                                                           |
| `Skill usage reports are not available on this connection.`                                                                                                                                                                                                          | [Command-line errors](#skill-usage-reports-are-not-available-on-this-connection)                                                |
| `Custom output styles can't be selected over Remote Control or from a relayed message`                                                                                                                                                                               | [Command-line errors](#custom-output-styles-cant-be-selected-over-remote-control)                                               |
| `Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load`                                                                                                                                                           | [Command-line errors](#output-styles-are-saved-to-local-settings-which-this-session-doesnt-load)                                |
| `` `plugin eval` is currently in early access `` / `` `plugin eval` is currently unavailable ``                                                                                                                                                                      | [Plugin errors](#plugin-eval-is-currently-in-early-access)                                                                      |
| `Marketplace "<name>" is registered from an untrusted source`                                                                                                                                                                                                        | [Plugin errors](#marketplace-is-registered-from-an-untrusted-source)                                                            |
| `Marketplace "<name>" is already added from a different source`                                                                                                                                                                                                      | [Plugin errors](#marketplace-is-already-added-from-a-different-source)                                                          |
| `"<name>" is another spelling of "<reserved>", a reserved marketplace name`                                                                                                                                                                                          | [Plugin errors](#marketplace-name-is-another-spelling-of-a-reserved-name)                                                       |
| `references ${user_config.*} in a shell-form command`                                                                                                                                                                                                                | [Plugin errors](#plugin-command-references-user-config)                                                                         |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                                                                                                                                                                                   | [Plugin errors](#plugin-command-references-user-config)                                                                         |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                                                                                                                                                                                  | [Plugin errors](#plugin-command-references-user-config)                                                                         |
| `Plugin archive integrity check failed`                                                                                                                                                                                                                              | [Plugin errors](#plugin-archive-integrity-check-failed)                                                                         |
| `path escapes plugin directory`                                                                                                                                                                                                                                      | [Plugin errors](#path-escapes-plugin-directory)                                                                                 |
| `path could not be checked`                                                                                                                                                                                                                                          | [Plugin errors](#path-could-not-be-checked)                                                                                     |
| `its marketplace entry path does not stay inside the marketplace directory`                                                                                                                                                                                          | [Plugin errors](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                         |
| `Plugin source path refused`                                                                                                                                                                                                                                         | [Plugin errors](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                         |
| `Failed to load marketplace configuration`                                                                                                                                                                                                                           | [Plugin errors](#failed-to-load-marketplace-configuration)                                                                      |
| `Marketplace configuration file is corrupted`                                                                                                                                                                                                                        | [Plugin errors](#failed-to-load-marketplace-configuration)                                                                      |
| `Plugin "<name>@synced" is required by your organization and can't be disabled here`                                                                                                                                                                                 | [Plugin errors](#plugin-is-required-by-your-organization)                                                                       |
| `would be spawned with zero tools — refusing`                                                                                                                                                                                                                        | [Tool errors](#agent-would-be-spawned-with-zero-tools)                                                                          |
| `File is covered by a Read deny rule in your permission settings`                                                                                                                                                                                                    | [Tool errors](#file-is-covered-by-a-read-deny-rule)                                                                             |
| `subagent_type is required: the general-purpose agent is not available in this session`                                                                                                                                                                              | [Tool errors](#subagent-type-is-required)                                                                                       |
| `Error: this write left the memory index at MEMORY.md at ..., over its ... read limit`                                                                                                                                                                               | [Tool errors](#memory-index-is-over-its-read-limit)                                                                             |
| `pkill: refusing to run`                                                                                                                                                                                                                                             | [Tool errors](#pkill-pattern-matches-the-claude-code-process)                                                                   |
| `Failed to write to <name>'s inbox — nothing was sent`                                                                                                                                                                                                               | [Tool errors](#failed-to-write-to-a-teammate-inbox)                                                                             |
| `Failed to write the plan approval request to the lead's inbox — plan not submitted`                                                                                                                                                                                 | [Tool errors](#failed-to-write-to-a-teammate-inbox)                                                                             |
| `Its agent definition was not restored: the folder its definition file came from is not trusted`                                                                                                                                                                     | [Tool errors](#teammate-agent-definition-not-restored)                                                                          |
| `Message too large for cross-session delivery`                                                                                                                                                                                                                       | [Tool errors](#message-too-large-for-cross-session-delivery)                                                                    |
| `Too many messages to this session just now`                                                                                                                                                                                                                         | [Tool errors](#too-many-messages-to-this-session-just-now)                                                                      |
| `Refusing to send: reply target is a symlink` / `Refusing to send: cannot vet reply target`                                                                                                                                                                          | [Tool errors](#refusing-to-send-a-cross-session-message)                                                                        |
| `Refusing to send: connected endpoint is not the expected process` / `Refusing to send: connected endpoint identity could not be read`                                                                                                                               | [Tool errors](#refusing-to-send-a-cross-session-message)                                                                        |
| `Refusing to send: connected endpoint is not owned by this user` / `Refusing to send: connected endpoint owner could not be read`                                                                                                                                    | [Tool errors](#refusing-to-send-a-cross-session-message)                                                                        |
| `Refusing to send: connected endpoint is a different process with the expected pid`                                                                                                                                                                                  | [Tool errors](#refusing-to-send-a-cross-session-message)                                                                        |
| `Refusing to read <path>: its symlink resolution changed after permission was checked (<reason>)` / `Refusing to search <path>: its symlink resolution changed after permission was checked`                                                                         | [Tool errors](#refusing-after-a-symlink-changed)                                                                                |
| `Refusing to write <path>: its parent-directory symlink resolution changed after permission was checked` / `Refusing to write <path>: it is a symbolic link. Write to the link's target path instead`                                                                | [Tool errors](#refusing-after-a-symlink-changed)                                                                                |
| `Refusing to write through symlink: <path>` / `Refusing to write into symlinked directory: <path>`                                                                                                                                                                   | [Tool errors](#refusing-after-a-symlink-changed)                                                                                |
| `Refusing to search <path>: a path one of its Read deny rules is written through changed while the search was being prepared` / `Refusing to search <path>: it could not be opened`                                                                                  | [Tool errors](#refusing-after-a-symlink-changed)                                                                                |
| `its permission check expired before it ran (too many concurrent file operations)` / `ripgrep was found only by name on PATH`                                                                                                                                        | [Tool errors](#refusing-after-a-symlink-changed)                                                                                |
| `task output swap refused (tasks dir moved or linked)`                                                                                                                                                                                                               | [Tool errors](#task-output-swap-refused)                                                                                        |
| `Command killed: its output file was replaced or could no longer be verified`                                                                                                                                                                                        | [Tool errors](#task-output-swap-refused)                                                                                        |
| `the source file is not valid UTF-8 text` / `the source file is not valid UTF-16 text`                                                                                                                                                                               | [Tool errors](#the-source-file-is-not-valid-utf-8-text)                                                                         |
| `the source file has the replacement character U+FFFD`                                                                                                                                                                                                               | [Tool errors](#the-source-file-is-not-valid-utf-8-text)                                                                         |
| `Reading a local file from outside this session's connected folders, or through a link, needs the approval card`                                                                                                                                                     | [Tool errors](#reading-a-local-file-from-outside-the-connected-folders)                                                         |
| `cannot read file_path (...) — the file could not be examined, and no one can answer the approval card`                                                                                                                                                              | [Tool errors](#reading-a-local-file-from-outside-the-connected-folders)                                                         |
| `WebFetch cannot fetch localhost or other hostnames without a dot`                                                                                                                                                                                                   | [Tool errors](#webfetch-cannot-fetch-localhost)                                                                                 |
| `Can't open MCP settings while no terminal is attached to this background session`                                                                                                                                                                                   | [Background session errors](#commands-refused-in-a-background-session)                                                          |
| `Can't open MCP settings in a background session`                                                                                                                                                                                                                    | [Background session errors](#commands-refused-in-a-background-session)                                                          |
| `blocked because the path is spelled in a form that cannot be safely resolved`                                                                                                                                                                                       | [Background session errors](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved)                               |
| `blocked because the path is network-shaped`                                                                                                                                                                                                                         | [Background session errors](#write-or-command-blocked-because-the-path-names-a-network-location)                                |
| `is isolated in the worktree <path>, but this command <reason>. Refusing to run it`                                                                                                                                                                                  | [Background session errors](#command-blocked-by-the-worktree-isolation-checks)                                                  |
| `too complex to verify that it stays inside the worktree`                                                                                                                                                                                                            | [Background session errors](#command-blocked-by-the-worktree-isolation-checks)                                                  |
| `This session has no saved transcript`                                                                                                                                                                                                                               | [Background session errors](#this-session-has-no-saved-transcript)                                                              |
| `Can't open — this session is running in another terminal`                                                                                                                                                                                                           | [Background session errors](#this-session-is-running-in-another-terminal)                                                       |
| `This conversation is already open in another running Claude session`                                                                                                                                                                                                | [Background session errors](#this-session-is-running-in-another-terminal)                                                       |
| `This session's saved conversation is no longer on disk`                                                                                                                                                                                                             | [Background session errors](#this-sessions-saved-conversation-is-no-longer-on-disk)                                             |
| `kept <id> — its worktree is still at <path>`                                                                                                                                                                                                                        | [Background session errors](#worktree-has-commits-that-are-not-pushed-anywhere)                                                 |
| `kept <id> — <n> unpushed commits on <branch>`                                                                                                                                                                                                                       | [Background session errors](#worktree-has-commits-that-are-not-pushed-anywhere)                                                 |
| `kept <id> — worktree has commits that are not pushed anywhere`                                                                                                                                                                                                      | [Background session errors](#worktree-has-commits-that-are-not-pushed-anywhere)                                                 |
| `terminal host process died — press Enter to restart` / `This session's terminal host process died`                                                                                                                                                                  | [Background session errors](#terminal-host-process-died)                                                                        |
| `Session isn't responding` / `Press enter again to restart this session — it isn't responding`                                                                                                                                                                       | [Background session errors](#session-isnt-responding)                                                                           |
| `Session <id> was stopped while the respawn was in flight`                                                                                                                                                                                                           | [Background session errors](#session-was-stopped-while-the-respawn-was-in-flight)                                               |
| `This session was running agent '<name>', which is no longer available`                                                                                                                                                                                              | [Background session errors](#session-agent-no-longer-available)                                                                 |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                                                                                                                                                                                          | [Background session errors](#claude_code_process_wrapper-launcher-errors)                                                       |
| `EUNKNOWN: unknown error, uv_spawn`                                                                                                                                                                                                                                  | [Background session errors](#eunknown-when-starting-a-background-session)                                                       |
| `EACCES: permission denied, posix_spawn`                                                                                                                                                                                                                             | [Background session errors](#eacces-when-starting-a-background-session)                                                         |
| `exited before it became reachable`                                                                                                                                                                                                                                  | [Background session errors](#background-service-exited-before-it-became-reachable)                                              |
| `Couldn't start a background session (working directory no longer exists or is not accessible: ...)`                                                                                                                                                                 | [Background session errors](#working-directory-no-longer-exists-when-starting-a-background-session)                             |
| `Claude Code is being updated by npm on this machine (still not runnable after 2 min, ...)`                                                                                                                                                                          | [Background session errors](#eacces-when-starting-a-background-session)                                                         |
| `Claude Code process exited with code N`                                                                                                                                                                                                                             | [Wrapper and IDE errors](#claude-code-process-exited-with-code-n)                                                               |
| `The connection to Claude Code ended before this message completed`                                                                                                                                                                                                  | [Wrapper and IDE errors](#the-connection-to-claude-code-ended-before-this-message-completed)                                    |
| `Could not locate the Claude CLI on PATH`                                                                                                                                                                                                                            | [Wrapper and IDE errors](#could-not-locate-the-claude-cli-on-path)                                                              |
| `Restored the code, but skipped N files`                                                                                                                                                                                                                             | [Rewind warnings and errors](#restored-the-code-but-skipped-files)                                                              |
| `No files were restored: N files failed (backup missing, or the file could not be updated)`                                                                                                                                                                          | [Rewind warnings and errors](#no-files-were-restored)                                                                           |
| `Transcript writes are failing (...)`                                                                                                                                                                                                                                | [Session saving warnings](#transcript-writes-are-failing)                                                                       |
| `Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set`                                                                                                                                                                                                  | [Session saving warnings](#transcript-saving-is-off-skip-prompt-history)                                                        |
| `Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker`                                                                                                                                                                                              | [Session saving warnings](#transcript-saving-is-off-child-session-marker)                                                       |
| `Claude Code's fullscreen renderer didn't finish starting last time on this machine` / `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`                                                                                            | [Configuration warnings](#fullscreen-failed-start-notice)                                                                       |
| `Claude Code exited after an unrecoverable interface error (...)`                                                                                                                                                                                                    | [Configuration warnings](#exited-after-an-unrecoverable-interface-error)                                                        |
| `Agent descriptions are over the 15.0k-token limit`                                                                                                                                                                                                                  | [Configuration warnings](#agent-descriptions-are-over-the-15000-token-limit)                                                    |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                                                                                                                                                                                  | [Configuration warnings](#workspace-has-not-been-trusted)                                                                       |
| `is a network path, which cannot be added as a working directory`                                                                                                                                                                                                    | [Configuration warnings](#working-directory-is-a-network-path)                                                                  |
| `Remote managed settings failed to load (<cause>)`                                                                                                                                                                                                                   | [Configuration warnings](#remote-managed-settings-failed-to-load)                                                               |
| `Managed settings were not approved; exiting without applying them.`                                                                                                                                                                                                 | [Configuration warnings](#managed-settings-were-not-approved)                                                                   |
| `MCP server <name> is blocked by enterprise managed policy`                                                                                                                                                                                                          | [Configuration warnings](#mcp-server-is-blocked-by-enterprise-managed-policy)                                                   |
| `Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.`                                                                                                                                              | [Configuration warnings](#managed-settings-document-could-not-be-parsed)                                                        |
| `Managed settings drop-in directory could not be read`                                                                                                                                                                                                               | [Configuration warnings](#managed-settings-document-could-not-be-parsed)                                                        |
| `otelHeadersHelper failed; telemetry is not being exported. See /status: ...`                                                                                                                                                                                        | [Configuration warnings](#otelheadershelper-failed)                                                                             |
| `"crossSessionInbound" must be one of "accept", "hold", "refuse"`                                                                                                                                                                                                    | [Configuration warnings](#crosssessioninbound-must-be-one-of-accept-hold-refuse)                                                |
| `headersHelper not run — this workspace has no persisted trust`                                                                                                                                                                                                      | [Configuration warnings](#headershelper-not-run)                                                                                |
| `Invalid permission rule "..." was skipped: Malformed Tool(content) rule`                                                                                                                                                                                            | [Configuration warnings](#malformed-tool-content-rule)                                                                          |
| `... is not matched by file permission checks`                                                                                                                                                                                                                       | [Configuration warnings](#is-not-matched-by-file-permission-checks)                                                             |
| `... has a wildcard before the rest of the command`                                                                                                                                                                                                                  | [Configuration warnings](#has-a-wildcard-before-the-rest-of-the-command)                                                        |
| `CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced`                                                                                                                                                                                           | [Configuration warnings](#the-200k-limit-isnt-enforced)                                                                         |
| `[claude-code:unrecognized_model]`                                                                                                                                                                                                                                   | [Configuration warnings](#unrecognized-model-id-on-a-request)                                                                   |
| `Stale sandbox mask files left by a killed session`                                                                                                                                                                                                                  | [Configuration warnings](#stale-sandbox-mask-files-left-by-a-killed-session)                                                    |
| Responses seem lower quality than usual                                                                                                                                                                                                                              | [Response quality](#responses-seem-lower-quality-than-usual)                                                                    |

<h2 id="automatic-retries">
  Автоматические повторные попытки
</h2>

Claude Code повторяет временные сбои до 10 раз с экспоненциальной задержкой перед отображением ошибки. Он не всегда повторяет сбой, который происходит в середине ответа Claude. Когда вы видите одну из ошибок на этой странице, Claude Code уже выполнил все применимые повторные попытки для этого сбоя; списки ниже указывают, какие сбои получают полный бюджет, какие получают меньший, а какие не получают никакого.

Claude Code повторяет эти сбои:

* Ошибки сервера, перегруженные ответы и тайм-ауты запросов, которые приходят до того, как какая-либо часть ответа Claude начнёт передаваться потоком.
* Разорванные соединения. Когда соединение разрывается в середине запроса до того, как Claude завершит какую-либо часть своего ответа, включая его размышление, Claude Code повторно отправляет запрос с той же задержкой и ход продолжается, даже если некоторый текст уже начал передаваться потоком. Когда оно разрывается после того, как Claude завершил размышление, но до того, как он начал какой-либо текст или вызов инструмента, Claude Code вместо этого повторно отправляет запрос до двух раз в быстрой последовательности и завершает ход с `Connection lost before a response was produced`, если соединение продолжает разрываться в этот момент.
* Соединение, которое Claude Code обнаружил, было разорвано тем, что ваш компьютер перешёл в режим сна в середине запроса. Claude Code считает это разорванным соединением в соответствии с приведёнными выше правилами; как только метка повторной попытки назовёт конкретную причину, она будет читаться как `Connection lost while your computer was asleep`, и если ход завершается после того, как Claude завершил размышление, но до любого текста или вызова инструмента, сообщение читается как `Your computer went to sleep before a response was produced`.
* Застопорившийся поток ответа, когда заголовки ответа прибыли, но ни одна часть ответа Claude не прибыла, или когда Claude завершил размышление, но не начал какой-либо текст или вызов инструмента: Claude Code прерывает застопорившееся соединение и повторно отправляет запрос максимум один раз, вне бюджета из 10 попыток выше. Если ответ застопорится во второй раз после того, как Claude завершил размышление, но до любого текста или вызова инструмента, Claude Code завершает ход с `The response stalled before a response was produced`.
* Потоковый запрос, на который API никогда не отвечает заголовками ответа, на соединении, где [первый байт дедлайна работает](/docs/ru/network-config#streaming-idle-watchdogs): Claude Code прерывает его в дедлайне и повторно отправляет его максимум один раз за запрос модели, в пределах бюджета повторных попыток, затем завершает ход с [No response from API](#no-response-from-api), если эта попытка также остаётся без ответа. На других соединениях запрос ждёт `API_TIMEOUT_MS`. Когда вы устанавливаете `CLAUDE_CODE_RETRY_WATCHDOG`, ограничение на одну повторную попытку не применяется.
* Временные дроссели 429, но не лимит расходов шлюза `429`, который не является дросселем; см. [Spend limit reached](#spend-limit-reached).
  * Когда вы вошли с подпиской claude.ai, это включает дроссели 429, которые не содержат заголовков квоты вашего плана. До v2.1.199 Claude Code повторял эти дроссели только для API ключа и входов Enterprise.
* Запрос отклонен, потому что входные данные плюс `max_tokens` превышают лимит контекста. Повторная отправка его без изменений приведёт к тому же результату, поэтому Claude Code повторяет с уменьшенным `max_tokens` и прекращает повторные попытки и вместо этого выполняет компактирование в двух случаях:
  * Когда никакое сокращение не может поместиться, например когда сам разговор почти заполняет окно контекста.
  * Когда повторная попытка не может больше сокращать `max_tokens`. До v2.1.218 Claude Code мог повторно отправить уменьшенный запрос, который всё ещё не подходил, например когда бюджет расширенного размышления превышал оставшийся контекст, пока не закончился бюджет повторных попыток.
* Истёкшие или отсутствующие учётные данные Google Cloud на [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai), или учётные данные AWS, которые не загружаются на вашей машине. Claude Code отбрасывает свои кэшированные учётные данные и повторяет попытку до двух раз, затем сообщает об ошибке, чтобы вы могли повторно пройти аутентификацию сразу же, как описано в разделе [Could not load AWS or Google Cloud credentials](#could-not-load-aws-or-google-cloud-credentials). До v2.1.228 Claude Code повторял неудачные учётные данные Google Cloud через полный бюджет повторных попыток перед отображением ошибки.
* `401` или `403` из Anthropic API, напрямую или через [LLM gateway](/docs/ru/llm-gateway), пока скрипт [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper) предоставляет учётные данные. Claude Code повторно запускает скрипт и повторяет попытку с его свежим выводом, в пределах полного бюджета повторных попыток. Когда сам скрипт не работает при повторном запуске, Claude Code вместо этого показывает [Your apiKeyHelper script is failing](#your-apikeyhelper-script-is-failing).

До v2.1.227 `Connection lost before a response was produced` читалось как `Connection closed while thinking, before producing a response` и `The response stalled before a response was produced` читалось как `Response stalled while thinking, before producing a response`.

Claude Code не повторяет эти сбои:

* Сбой проверки сертификата TLS, такой как TLS-инспектирующий прокси, отсутствующий пакет `NODE_EXTRA_CA_CERTS` или истёкший сертификат. Claude Code сообщает об ошибке при первой попытке, чтобы вы могли сразу же исправить настройку сертификата; см. [SSL certificate errors](#ssl-certificate-errors). Claude Code по-прежнему повторяет временные условия TLS, такие как тайм-аут рукопожатия. До v2.1.199 Claude Code повторял сбои сертификатов через полный бюджет повторных попыток перед отображением ошибки.
* Ошибка сервера, разорванное соединение или застопорившийся поток, который приходит после того, как Claude завершил блок текста или вызов инструмента, или начал один после завершения своего размышления, но до завершения ответа. Claude Code не повторно запускает запрос, потому что это может выполнить одни и те же вызовы инструментов дважды. Он сохраняет то, что Claude завершил, запускает любые вызовы инструментов, которые Claude завершил, и продолжает ход из их результатов. Для того, что вы видите в интерактивном сеансе и в неинтерактивном, прочитайте [The response above may be incomplete](#the-response-above-may-be-incomplete). До v2.1.199 Claude Code отбрасывал частичный вывод и сообщал об всём ходе как об ошибке, когда ошибка сервера приходила в середине потока.
* Сбой, который приходит после того, как Claude завершил ответ: повторная попытка не требуется, поэтому Claude Code сохраняет полный ответ и завершает ход нормально.
* [Amazon Bedrock streaming response with an unexpected content-type](#bedrock-streaming-response-has-an-unexpected-content-type), потому что шлюз или прокси, переписывающие ответ, переписали бы повторную попытку таким же образом. Требуется Claude Code v2.1.208 или позже.
* Неполная повторная попытка потокового запроса, который получает статус успеха, но [no Claude API message in the body](#api-returned-an-empty-or-malformed-response). Claude Code завершает ход с этой ошибкой.
* Запрос, который проверка политики вашей организации отклонила, который отображается как строка `API Error:`, содержащая сообщение об отказе. Администраторы вашей организации настроили проверку с помощью [Inference hooks](https://platform.claude.com/docs/en/manage-claude/inference-hooks), функции Claude Enterprise, и сообщение заканчивается инструкциями, которые они настроили, или по умолчанию говорит вам связаться с ними. Claude Code не повторно отправляет отклонённый запрос на ту же модель или на [fallback model](/docs/ru/model-config#fallback-model-chains), потому что отказ касается содержимого запроса, а не модели. До v2.1.239 Claude Code мог повторно отправить отклонённый запрос без потока или на настроенную резервную модель перед отображением отказа.

<h3 id="what-you-see-while-claude-code-retries-or-waits">
  Что вы видите, пока Claude Code повторяет или ждёт
</h3>

Во время повторной попытки спиннер показывает обратный отсчёт `Retrying in Ns · attempt x/y` после метки ошибки. Метка называет конкретную причину с первой попытки для сбоев, на которые вы можете действовать сразу же: сеть отключена, рукопожатие TLS не удалось, или вы достигли лимита скорости. Для других ошибок сначала читается `API error`. Начиная с v2.1.198 он переключается на конкретную причину с третьей попытки, или при последней попытке, когда `CLAUDE_CODE_MAX_RETRIES` позволяет менее трёх; более ранние версии переключаются только при последней попытке.

Начиная с v2.1.198 обычный совет спиннера подавляется во время повторных попыток. После того как причина ошибки раскрыта, если сбой является перегрузкой 529, строка ниже обратного отсчёта также называет, где проверить статус услуги: `status.claude.com` на Anthropic API, или хост поставщика или шлюза, названный в сообщении на других конфигурациях.

Если никакие данные не поступают в поток ответа в течение 20 секунд, пока запрос всё ещё ожидается, спиннер показывает `Waiting for API response · will retry in … · check your network` перед началом любой повторной попытки. Запрос ещё не завершился: обратный отсчёт работает до точки, где Claude Code прерывает застопорившееся соединение. После прерывания то, что вы видите, зависит от того, как далеко продвинулся ответ:

* До того, как Claude завершил блок текста или вызов инструмента, или начал один после завершения своего размышления, Claude Code повторяет запрос или завершает ход с ошибкой. [Automatic retries](#automatic-retries) говорит, какие застои он повторяет и сколько раз.
* После того, как Claude завершил блок текста или вызов инструмента, или начал один после завершения своего размышления, но до того, как Claude завершил ответ, Claude Code сохраняет то, что Claude завершил, продолжает ход из любых вызовов инструментов, которые Claude завершил, и показывает [The response above may be incomplete](#the-response-above-may-be-incomplete). В неинтерактивном сеансе и для ответа подагента в любом сеансе Claude Code может сначала предложить Claude продолжить ответ; эта запись говорит, когда это происходит и когда вы всё ещё видите уведомление там.
* После того, как Claude завершил ответ, Claude Code завершает ход нормально.

Баннер очищается сам по себе, как только данные возобновляются или повторная попытка успешна. Если он появляется при каждой попытке, рассматривайте это как [network issue](#unable-to-connect-to-api). До v2.1.185 баннер появлялся через 10 секунд с другой формулировкой.

Пока Claude консультируется с [advisor](/docs/ru/advisor), баннер появляется через 90 секунд без данных вместо 20, потому что длительный обзор советника может не отправлять ничего более 20 секунд. До v2.1.214 порог в 20 секунд применялся и во время вызовов советника, поэтому баннер появлялся во время обзоров советника даже когда ничего не было неправильно.

<h3 id="tune-retry-behavior">
  Настройка поведения повторных попыток
</h3>

Вы можете настроить поведение повторных попыток с помощью этих переменных окружения:

| Переменная                                            | По умолчанию   | Эффект                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :---------------------------------------------------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/ru/env-vars)             | 10             | Количество попыток повторной попытки. Ограничено 15 начиная с v2.1.186; начиная с v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` повышает значение по умолчанию и удаляет ограничение. Снизьте его, чтобы быстрее выявлять сбои в скриптах.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/ru/env-vars)          | не установлено | Установите на `1` в автоматических сеансах, таких как задания CI, чтобы повторять ошибки пропускной способности `429` и `529` бесконечно вместо отказа после `CLAUDE_CODE_MAX_RETRIES` попыток. Claude Code отказывает сразу же при `429`, который сообщает о лимите расходов или исчерпанных кредитах использования, даже при одном из [gateway spend cap](#spend-limit-reached), который сбрасывается по расписанию. До v2.1.239 сторож повторял эти бесконечно. На v2.1.199 или позже он также повышает количество повторных попыток по умолчанию для других временных ошибок, таких как ошибки сервера, тайм-ауты и разорванные соединения, до 300, примерно три часа задержки, и удаляет ограничение 15 на `CLAUDE_CODE_MAX_RETRIES`, если вы явно установите эту переменную. Для запросов в режиме быстрого выполнения см. [Handle rate limits](/docs/ru/fast-mode#handle-rate-limits). |
| [`API_TIMEOUT_MS`](/docs/ru/env-vars)                      | 600000         | Тайм-аут для каждого запроса в миллисекундах. Повысьте его для медленных сетей или прокси. Он также ограничивает, как долго Claude Code ждёт заголовков ответа, описано в [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/ru/env-vars) | не установлено | Дедлайн в миллисекундах для первого байта ответа потокового запроса. Требуется Claude Code v2.1.242 или позже. Для того, как Claude Code выбирает дедлайн, когда это не установлено, см. [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h2 id="server-errors">
  Ошибки сервера
</h2>

Большинство этих ошибок поступают от поставщика вывода: сервиса Anthropic на Anthropic API и сервиса за конечной точкой этого поставщика на Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или пользовательском шлюзе. [Auto mode cannot determine the safety of an action](#auto-mode-cannot-determine-the-safety-of-an-action) и [Agent terminated early due to an API error](#agent-terminated-early-due-to-an-api-error) также охватывают причины на вашей стороне, такие как учетная запись Amazon Bedrock, которая не может вызвать модель классификатора, или подагент, который достиг лимита использования.

<h3 id="api-error-500-internal-server-error">
  API Error: 500 Internal server error
</h3>

Claude Code отображает код состояния и сообщение об ошибке API для любого ответа 5xx. Пример ниже показывает ответ 500 на Anthropic API:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

Завершающее предложение указывает, где проверить здоровье сервиса и варьируется в зависимости от поставщика. Конфигурации Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry указывают на страницу статуса этого поставщика. Пользовательский `ANTHROPIC_BASE_URL` указывает на хост шлюза.

Это указывает на неожиданный сбой внутри API. Это не вызвано вашим приглашением, настройками или учетной записью.

**Что делать:**

* Проверьте [status.claude.com](https://status.claude.com) или страницу статуса поставщика, указанную в сообщении, на предмет активных инцидентов
* Подождите минуту, затем отправьте сообщение еще раз. Ваше исходное сообщение все еще находится в разговоре, поэтому для длинного приглашения вы можете ввести `try again` вместо вставки всего текста.
* Если ошибка сохраняется без опубликованного инцидента, запустите `/feedback`, чтобы Anthropic могла расследовать детали вашего запроса. См. [Report an error](#report-an-error), если `/feedback` недоступен в вашей среде.

<h3 id="api-error-repeated-529-overloaded-errors">
  API Error: Repeated 529 Overloaded errors
</h3>

API временно работает на полную мощность для всех пользователей. Claude Code уже несколько раз повторил попытку перед отображением этого сообщения:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

Завершающее предложение варьируется в зависимости от поставщика так же, как ошибка 500 выше.

529 — это не ваш лимит использования и не учитывается в вашей квоте.

**Что делать:**

* Проверьте [status.claude.com](https://status.claude.com) или страницу статуса поставщика, указанную в сообщении, на предмет уведомлений о емкости
* Повторите попытку через несколько минут
* Запустите `/model` и переключитесь на другую модель, чтобы продолжить работу, так как емкость отслеживается для каждой модели. Claude Code предлагает вам это сделать, когда одна модель испытывает особенно высокую нагрузку, например `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Request timed out
</h3>

API не ответил до истечения срока подключения.

```text theme={null}
Request timed out
```

Это может произойти в периоды высокой нагрузки или когда модель генерирует очень большой ответ. Время ожидания запроса по умолчанию составляет 10 минут.

**Что делать:**

* Повторите запрос
* Для долгосрочных задач разбейте работу на более мелкие приглашения
* Если причина в медленной сети или прокси, увеличьте `API_TIMEOUT_MS`, как описано в [Automatic retries](#automatic-retries)
* Если тайм-ауты частые и ваша сеть в остальном здорова, см. [Network and connection errors](#network-and-connection-errors) ниже

<h3 id="no-response-from-api">
  No response from API
</h3>

Claude Code отправил потоковый запрос, и API не вернул заголовки ответа в срок для первого байта, поэтому Claude Code прервал запрос вместо ожидания полного тайм-аута запроса `API_TIMEOUT_MS`, 10 минут по умолчанию. Claude Code отправляет запрос снова максимум один раз, если [retry budget](#tune-retry-behavior) позволяет. Когда повторная попытка также остается без ответа, ход завершается этим сообщением, которое показывает, как долго ждала каждая попытка. Когда вы устанавливаете [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/ru/env-vars), ограничение на одну повторную попытку не применяется и Claude Code повторяет попытки в соответствии с бюджетом, описанным в [Tune retry behavior](#tune-retry-behavior).

```text theme={null}
API Error: No response from API (waited 3m, then 10m on the retry). If a proxy or gateway on your network holds responses until they complete, raise API_TIMEOUT_MS or CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS to wait longer.
```

Claude Code устанавливает ожидание заголовков ответа первой попытки и повторной попытки отдельно:

* **Первая попытка**: [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/ru/env-vars) когда вы устанавливаете его на 1 или более, ограничено между 10 секундами и 30 минутами. В противном случае Claude Code использует тайм-аут байт-уровня watchdog, указанный в [Streaming idle watchdogs](/docs/ru/network-config#streaming-idle-watchdogs), поэтому переменные, которые изменяют этот тайм-аут, изменяют это ожидание тоже. В любом случае Claude Code добавляет одну секунду на каждые 32 КБ тела запроса.
* **Повторная попытка**: на одну секунду меньше `API_TIMEOUT_MS`, чуть менее 10 минут по умолчанию, чтобы повторная попытка могла пережить прокси или шлюз, который держит ответ до завершения генерации. На Amazon Bedrock повторная попытка использует тот же срок, что и первая попытка, и сообщение показывает одну длительность вместо двух.

Ни одно ожидание не превышает одну секунду меньше положительного `API_TIMEOUT_MS`, и положительный `API_TIMEOUT_MS` менее 11 секунд отключает срок. Байт-уровневый watchdog начинается только после получения заголовков ответа, поэтому ответ, который перестает отправлять байты после этого, следует [stalled-stream rules](#automatic-retries) вместо этого срока.

**Что делать:**

* Отправьте сообщение еще раз. Ваше исходное сообщение все еще находится в разговоре, поэтому для длинного приглашения вы можете ввести `try again` вместо вставки всего текста.
* Если это повторяется, рассматривайте это как [network or proxy problem](#unable-to-connect-to-api). Прокси, который принимает соединение и никогда не пересылает запрос, производит эту ошибку при каждой попытке.
* Если прокси или шлюз в вашей сети держит ответы до их завершения, увеличьте `API_TIMEOUT_MS`, чтобы повторная попытка ждала дольше. На Amazon Bedrock также увеличьте `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`.
* Если первая попытка продолжает истекать по времени, а повторная попытка затем успешна, увеличьте `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`, чтобы первая попытка также ждала достаточно долго.

До v2.1.242 Claude Code ждал полного тайм-аута запроса `API_TIMEOUT_MS`, 10 минут по умолчанию, перед отказом от потокового запроса без ответа. До v2.1.261 повторная попытка ждала тот же срок, что и первая попытка, и сообщение не показывало длительности.

<h3 id="the-response-above-may-be-incomplete">
  The response above may be incomplete
</h3>

Потоковый запрос не удался, пока ответ был еще в процессе, после того как Claude завершил блок текста или вызов инструмента, или начал один после завершения своего мышления. Повторная отправка запроса может запустить одни и те же вызовы инструментов дважды, поэтому Claude Code сохраняет выходные данные, которые Claude завершил, и добавляет это уведомление вместо отказа от хода. Какой вариант вы видите, указывает на причину:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection lost mid-response. The response above may be incomplete.
API Error: Your computer went to sleep mid-response. The response above may be incomplete.
API Error: The response stopped arriving. The response above may be incomplete.
```

* `Server error mid-response`: ошибка сервера перегрузки или 5xx в середине потока. Этот вариант требует Claude Code v2.1.199 или позже; до этого этот случай отбросил частичный выход и сообщил весь ход как ошибку.
* `Connection lost mid-response`: соединение разорвалось.
* `Your computer went to sleep mid-response`: Claude Code обнаружил, что ваш компьютер перешел в режим сна во время потоковой передачи ответа. После пробуждения компьютера Claude Code рассматривает соединение как разорванное и прекращает чтение из него.
* `The response stopped arriving`: соединение оставалось открытым, но перестало доставлять данные, поэтому потоковый idle watchdog прервал его. До v2.1.222 Claude Code также мог сообщить об этом сбое на [gateway](/docs/ru/gateways) соединениях, достигнутых через `ANTHROPIC_BASE_URL` или `ANTHROPIC_AWS_BASE_URL`, пока пинги keep-alive сервера все еще поступали, потому что он считал только проанализированные события ответа там; обновление останавливает эти ложные тайм-ауты на этих маршрутах. Шлюзы, достигнутые через URL базы поставщика, такой как `ANTHROPIC_BEDROCK_BASE_URL`, не обернуты байт-уровневым watchdog; см. [Streaming idle watchdogs](/docs/ru/network-config#streaming-idle-watchdogs).

До v2.1.227 `Connection lost mid-response` читалось как `Connection closed mid-response` и `The response stopped arriving` читалось как `Response stalled mid-stream`.

В четырех случаях Claude Code обрабатывает сбой без отображения этого уведомления сразу:

* Ранее в ответе Claude Code либо повторяет попытку сбоя, либо завершает ход с другой ошибкой. См. [Automatic retries](#automatic-retries).
* Когда один из этих сбоев поступает после того, как Claude завершил ответ, Claude Code сохраняет полный ответ и завершает ход нормально, без этого уведомления. До v2.1.222 Claude Code показывал это уведомление, когда соединение разорвалось или застопорилось после завершения ответа, и сообщал ход как ошибку, даже если ответ был полным.
* В [non-interactive session](/docs/ru/headless), такой как запуск `-p`, запуск [Agent SDK](/docs/ru/agent-sdk/overview) или [cloud session](/docs/ru/claude-code-on-the-web), вам не нужно отправлять `continue` самостоятельно, когда обрезанный ответ находится в основном разговоре и содержит текст, но не вызовы инструментов: Claude Code сохраняет частичный выход и предлагает Claude продолжить с того места, где он остановился, до трех раз подряд. Вы видите это уведомление для такого ответа только после того, как Claude Code исчерпал эти продолжения. До v2.1.246 Claude Code завершал неинтерактивный ход с этим уведомлением при первом обрезании.
* В [subagent](/docs/ru/sub-agents#api-errors-in-subagents), независимо от того, интерактивна ли сессия или нет: когда его обрезанный ответ содержит текст, но не вызовы инструментов, Claude Code предлагает подагенту продолжить. Уведомление становится последним сообщением подагента только после того, как эти продолжения исчерпаны. До v2.1.257 подагент показывал это уведомление при первом обрезании.

**Что делать:**

* В интерактивной сессии прочитайте ответ, который остается на экране: Claude Code сохраняет каждый блок, который Claude завершил перед ошибкой, но отбрасывает прерванный финальный блок, когда ход заканчивается, поэтому финальные предложения или вызовы инструментов могут отсутствовать. Ответьте с `continue`, чтобы Claude продолжил с его последнего завершенного блока.
* В [non-interactive mode](/docs/ru/headless) (`-p`):
  * С выходом текста по умолчанию Claude Code печатает последний завершенный блок текста, который он все еще держит с более ранней части хода, за которым следует это сообщение. Когда он не держит ничего, Claude Code печатает только это сообщение, например, потому что Claude Code сжал разговор в середине хода и очистил этот текст. До v2.1.219 Claude Code печатал только это сообщение в выходе текста `-p` и отбрасывал ответ, который он уже произвел.
  * С `--output-format json` или `stream-json` Claude Code сообщает об этом сообщении в поле `result`.
  * Чтобы продолжить ход после стабилизации соединения, возобновите сессию и отправьте `continue`, как описано в [Continue conversations](/docs/ru/headless#continue-conversations).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto mode cannot determine the safety of an action
</h3>

Модель, которую [auto mode](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) использует для классификации действий, не смогла принять решение, поэтому auto mode не одобрил действие автоматически. Сообщение, которое вы видите, зависит от того, как классификатор не удался.

Чтения, поиски и редактирования внутри вашего рабочего каталога пропускают классификатор, поэтому они продолжают работать во всех этих случаях.

Когда модель классификатора недоступна:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait a moment and then try this action again.
```

Когда Claude Code может определить категорию сбоя, он называет категорию в скобках после `temporarily unavailable`, например `<model> is temporarily unavailable (rate-limited), so auto mode cannot determine the safety of <tool> right now`. Категории: `(rate-limited)`, `(overloaded)`, `(server error)`, `(timed out)` и `(connection failed)`. Ограничение скорости, перегрузка и ошибки сервера являются временными, и повторная попытка работает. Если `(timed out)` или `(connection failed)` повторяется, проверьте ваше соединение; см. [Unable to connect to API](#unable-to-connect-to-api). До v2.1.229 сообщение никогда не называло категорию и читалось как `Wait briefly and then try this action again`.

Когда ни одна категория не подходит, сообщение появляется без категории в скобках; более одного сбоя производит эту форму. На [Amazon Bedrock](/docs/ru/amazon-bedrock), включая [Mantle endpoint](/docs/ru/amazon-bedrock#use-the-mantle-endpoint), это также появляется, когда ваша учетная запись AWS не может вызвать модель, указанную в сообщении, и этот сбой повторяется при каждой повторной попытке, пока вашей учетной записи не будет предоставлен доступ к модели.

**Что делать:**

* Повторите попытку через несколько секунд; Claude видит то же сообщение и обычно повторяет попытку самостоятельно. Временный сбой не связан с [auto mode eligibility](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode); вам не нужно менять настройки
* Если повторные попытки продолжают не удаваться, продолжайте с задачами только для чтения и вернитесь к заблокированному действию позже
* На Amazon Bedrock, если сообщение возвращается при каждой повторной попытке, проверьте, что ваша учетная запись может вызвать модель, которую оно называет: для стандартных моделей Amazon Bedrock подтвердите, что ваша [IAM policy](/docs/ru/amazon-bedrock#iam-configuration) позволяет вызывать ее; для ID моделей Mantle [свяжитесь с вашей командой учетной записи AWS](/docs/ru/amazon-bedrock#mantle-endpoint-errors)

Когда запрос классификатора не удается, потому что ваш OAuth токен истек или был повернут другой сессией, Claude Code обновляет токен и повторяет запрос один раз, поэтому обычное истечение токена не появляется как это сообщение. До v2.1.216 истекший или повернутый токен не удавался при каждом запросе классификатора, и auto mode отрицал каждое проверенное действие с этим сообщением, пока токен не был обновлен.

Когда классификатор вернул непарсируемый ответ:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Что делать:**

* Повторите действие; это обычно успешно при следующей попытке
* Запустите `claude --debug` и повторите действие, чтобы увидеть основной ответ классификатора в журнале отладки

Когда отдельная проверка безопасности API заблокировала запрос классификатора из-за более раннего содержимого разговора:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

Claude Code отрицает действие, но говорит Claude, что это не суждение о том, что действие небезопасно, и продолжить с другими задачами, а не повторять попытку. Эти отказы не учитываются в [auto mode's pause thresholds](/docs/ru/permission-modes#when-auto-mode-falls-back). В [non-interactive](/docs/ru/headless) запуске `-p` Claude Code не останавливает запуск. То, что получает Claude, зависит от того, где оно запросило действие:

* К [background subagent](/docs/ru/sub-agents#run-subagents-in-foreground-or-background) в запуске `-p` без `--input-format stream-json` Claude Code возвращает результат ошибки, содержащий `Agent aborted: auto mode classifier request refused by the safety safeguard in headless mode`
* Везде, включая интерактивные сессии и основной разговор запуска `-p`, Claude Code возвращает это отрицание Claude

До v2.1.225 Claude Code считал эти отказы в направлении пороговых значений паузы и возвращал то же самое отклонение, что и подлинный блок классификатора.

**Что делать:**

* Это не решение о вашем действии. Содержимое, уже находящееся в вашем разговоре, вызвало фильтр безопасности на API, когда auto mode отправил разговор классификатору
* Повторная попытка не поможет; то же содержимое разговора снова вызовет фильтр
* В интерактивной сессии переключитесь на другой [permission mode](/docs/ru/permission-modes), чтобы вы могли одобрить действие при появлении запроса
* Начните свежий разговор без содержимого, вызывающего срабатывание

Когда разговор вырос больше, чем контекстное окно классификатора:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

То, что происходит с действием, зависит от того, где Claude его запросил:

* В интерактивной сессии auto mode возвращается к обычному приглашению разрешения для этого действия, чтобы вы могли одобрить или отрицать его вручную
* К [background subagent](/docs/ru/sub-agents#run-subagents-in-foreground-or-background) в [non-interactive](/docs/ru/headless) запуске `-p` без `--input-format stream-json` Claude Code возвращает результат ошибки, содержащий `Agent aborted: auto mode classifier transcript exceeded context window in headless mode`, и запуск продолжается
* В другом месте в запуске `-p` без [`--permission-prompt-tool`](/docs/ru/cli-reference#cli-flags) нет приглашения для возврата, поэтому действие не выполняется и запуск продолжается

**Что делать:**

* В интерактивной сессии одобрите или отрицайте действие в появившемся приглашении
* В интерактивной сессии запустите `/compact`, чтобы уменьшить размер разговора, чтобы последующие действия снова подходили в окно классификатора

<h3 id="the-server-returned-no-safety-verdict">
  The server returned no safety verdict
</h3>

В [server-side classifier review](/docs/ru/permission-modes#server-side-classifier-review) auto mode отрицает действие, когда сервер не дает вердикт для него. Отрицание называет категорию в скобках, когда Claude Code может определить одну, такую как `(timed out)`:

```text theme={null}
The server-side auto mode classifier gave no verdict (timed out), so auto mode cannot determine the safety of <tool>.
```

Остальная часть сообщения говорит Claude, может ли одна повторная попытка помочь. До некоторых из этих отказов Claude Code ждет, чтобы следующая попытка Claude не следовала сразу. Во время ожидания в интерактивной сессии спиннер показывает `Auto mode check unavailable` с обратным отсчетом, и нажатие `Esc` прерывает ход.

После десяти ответов подряд без вердикта auto mode останавливает ход:

```text theme={null}
Auto mode is unavailable — the server returned no safety verdict for the last 10 responses, so Claude stopped. Send a message to try again, or switch out of auto mode.
```

Сообщение об остановке появляется в другом месте в каждом типе сессии:

* В интерактивной сессии сообщение появляется как предупреждение в транскрипте и ход заканчивается
* В [non-interactive](/docs/ru/headless) запуске `-p` запуск заканчивается и сообщает об ошибке выполнения. С выходом текста по умолчанию сообщение печатается на stderr.
* Когда [subagent](/docs/ru/sub-agents) достиг лимита, подагент останавливается перед завершением, и Claude получает то, что он произвел, с примечанием, что auto mode остановил его

**Что делать:**

* Отправьте другое сообщение, чтобы Claude попробовал снова. Счет ответов начинается заново.
* Если остановка повторяется и ваши запросы проходят через [LLM gateway or proxy](/docs/ru/llm-gateway), проверьте, сокращает ли он потоковые ответы или переписывает их. [Server-side classifier review](/docs/ru/permission-modes#server-side-classifier-review) говорит, какое поведение шлюза вызывает отказы, и [gateway compatibility guide](/docs/ru/llm-gateway-protocol#feature-pass-through) перечисляет, что пропустить без изменений.
* Установите `CLAUDE_CODE_AUTO_MODE_SERVER=0` перед запуском Claude Code, чтобы использовать его собственные запросы классификатора вместо этого. До v2.1.281 Claude Code не читал переменную на прямом соединении с Anthropic API.
* Чтобы одобрить действия самостоятельно, [switch out of auto mode](/docs/ru/permission-modes#switch-permission-modes)

До v2.1.280 Claude Code отрицал каждое действие из ответа без вердикта немедленно и никогда не останавливал ход.

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent terminated early due to an API error
</h3>

Запрос API [subagent](/docs/ru/sub-agents) не удался окончательно, например, потому что был достигнут лимит использования или повторные попытки для ошибки сервера исчерпаны, поэтому подагент остановился перед завершением своей задачи. Это сообщение требует Claude Code v2.1.199 или позже; до этого текст ошибки API был возвращен Claude, как если бы это был результат подагента.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Что делать:**

* Сопоставьте деталь ошибки после двоеточия с его собственным разделом на этой странице, такой как [Usage limits](#usage-limits) или [Server errors](#server-errors), и следуйте шагам этого раздела
* После того как основная ошибка исчезнет, попросите Claude повторить задачу или [resume the subagent](/docs/ru/sub-agents#resume-subagents)

Когда ограничение скорости, перегрузка или ошибка сервера прерывает подагент переднего плана, который уже произвел текстовый выход, Claude получает этот частичный выход, отмеченный как неполный, вместо этой ошибки. Подагент, единственным выходом которого были вызовы инструментов, также получает эту ошибку; в v2.1.199 эта форма возвращала пустой частичный результат вместо этого. См. [API errors in subagents](/docs/ru/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Ограничения использования
</h2>

Большинство ошибок в этом разделе означают, что достигнута квота, привязанная к вашей учётной записи или плану. Три работают по-другому: [`Server is temporarily limiting requests`](#server-is-temporarily-limiting-requests) — это серверное ограничение скорости, не связанное с квотой вашего плана, [`Usage credits required for 1M context`](#usage-credits-required-for-1m-context) — это проверка прав доступа, а не исчерпанная квота, и [`The prompt to confirm went unanswered`](#the-prompt-to-confirm-went-unanswered) означает, что подтверждающий запрос на использование кредитов закрыт без ответа, независимо от того, была ли достигнута квота.

<h3 id="youve-hit-your-session-limit">
  You've hit your session limit
</h3>

Планы подписки включают скользящий лимит использования. Когда он заканчивается, вы видите одно из этих сообщений:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
You've hit your Sonnet limit · resets 3:45pm
```

Claude Code блокирует дальнейшие запросы до времени сброса, указанного в сообщении. Лимиты сеанса и недели являются общими для всех моделей, поэтому переключение моделей не восстанавливает доступ. Лимиты Opus и Sonnet применяются только к запросам к этому семейству моделей, поэтому переключение на модель вне семейства с помощью `/model` позволяет вам продолжить работу.

В интерактивном сеансе, вошедшем с подпиской claude.ai, Claude Code также может ждать в открытом сеансе и продолжить прерванную задачу вскоре после сброса. Пока он ждёт, строка в нижней части сеанса читает `Usage limit reached · continuing automatically at 3:45pm · esc to cancel`. Нажмите `Esc` при пустом приглашении, чтобы отменить ожидание. См. [Wait for a usage limit to reset](/docs/ru/interactive-mode#wait-for-a-usage-limit-to-reset) для информации о том, что вы видите, как начать или отменить ожидание и как отключить автоматическое продолжение. До версии v2.1.234 Claude Code не предлагал это ожидание.

Использование учитывается одновременно в лимитах сеанса и недели. Один всплеск интенсивной деятельности, такой как большой fanout рабочего процесса, может исчерпать недельный лимит до сброса окна сеанса.

**Что делать:**

* Дождитесь времени сброса, указанного в ошибке
* На вкладке Code в [Desktop app](/docs/ru/desktop) карточка session-limit предлагает флажок **Auto-continue when limits reset**. Карточка weekly-limit этого не предлагает. Когда он отмечен, Desktop app повторяет прерванный ход после сброса и показывает время повтора на карточке. Флажок Desktop и параметр **Continue automatically at usage limit** в CLI в `/config` являются отдельными, поэтому отключайте каждый отдельно.
* Для лимита Opus или Sonnet запустите `/model` и переключитесь на модель вне этого семейства, чтобы продолжить работу. Каждая модель имеет свой собственный кэш подсказок, поэтому следующий запрос повторно читает весь разговор без попаданий в кэш; см. [Switching models](/docs/ru/prompt-caching#switching-models)
* Запустите `/usage`, чтобы увидеть лимиты вашего плана и когда они сбрасываются
* Запустите `/usage-credits`, чтобы купить дополнительное использование на Pro и Max, или запросить его у администратора на Team и Enterprise. См. [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) для информации о том, как это выставляется счётом.
* Чтобы обновить ваш план для более высоких базовых лимитов, см. [claude.com/pricing](https://claude.com/pricing)

Перед тем как окно закончится, Claude Code может предупредить вас, что вы использовали большую часть его, с сообщением, таким как `You've used 85% of your session limit · resets 3:45pm`. Чтобы непрерывно отслеживать оставшийся лимит, добавьте поля `rate_limits` в [custom status line](/docs/ru/statusline#rate-limit-usage), или в Desktop app нажмите [usage ring](/docs/ru/desktop#check-usage) рядом с выбором модели.

<h3 id="usage-credits-required-for-1m-context">
  Usage credits required for 1M context
</h3>

Выбранная модель использует расширенное окно контекста с расширением 1M-токена, и ваш план включает его только через кредиты использования.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on (they take effect after you restart Claude Code), or /model to switch to standard context
```

Это проверка прав доступа, а не исчерпание квоты. Она срабатывает даже когда ваши лимиты сеанса и недели имеют оставшуюся ёмкость. См. [Extended context](/docs/ru/model-config#extended-context) для информации о том, какие планы включают контекст 1M напрямую и какие требуют кредитов использования. Claude Code выполняет эту проверку, когда вы выбираете модель с помощью `/model`, и только при прямом подключении к API Anthropic; если вы указываете `ANTHROPIC_BASE_URL` на [LLM gateway](/docs/ru/llm-gateway), `/model` позволяет выбрать `[1m]` и шлюз решает, будет ли запрос успешным.

Когда эта ошибка появляется в середине разговора, потому что контекст вырос более чем на 200K токенов, Claude Code автоматически сжимает разговор обратно под стандартный лимит контекста и сохраняет сеанс на этом лимите впоследствии, поэтому никаких действий не требуется. В версиях до v2.1.172 ошибка повторялась при каждом последующем запросе, включая `/compact`; запустите `/clear` на этих версиях для восстановления. Приведённые ниже шаги применяются, когда вы явно выбрали модель `[1m]`.

**Что делать:**

* Запустите `/model` и выберите вариант без суффикса `[1m]`, чтобы вернуться к стандартному окну контекста
* Где сообщение называет `/usage-credits`, запустите его, чтобы включить поэтапное выставление счётов для варианта 1M на Pro и Max, или запросить кредиты использования у администратора на Team и Enterprise. После включения кредитов использования перезагрузите Claude Code или начните новый сеанс, в зависимости от того, что говорит сообщение. До этого сеанс остаётся на стандартном лимите контекста.
* Если ошибка сохраняется после `/model`, ID модели 1M может быть установлен в другом месте. См. [Setting your model](/docs/ru/model-config#setting-your-model) для проверки мест конфигурации в порядке приоритета.
* Чтобы полностью удалить варианты 1M из выбора модели, установите [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/ru/env-vars)

До версии v2.1.268 сообщение заканчивалось на `run /usage-credits to turn them on, or /model to switch to standard context` и не упоминало перезагрузку.

<h3 id="the-prompt-to-confirm-went-unanswered">
  The prompt to confirm went unanswered
</h3>

Если ваша учётная запись требует [Fable usage-credits consent](/docs/ru/model-config#fable-and-usage-credits), Claude Code просит вас подтвердить перед запросом Fable, который будет выставлять счёт за кредиты использования. Когда никто не отвечает на этот запрос согласия в сеансе, который может не иметь никого у его терминала, Claude Code закрывает запрос и завершает ход одним из этих сообщений:

```text theme={null}
Fable limit reached · continuing on Fable 5.1 uses usage credits, and the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
Fable 5.1 now uses usage credits · the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
```

Сообщения называют модель Fable сеанса, поэтому на Fable 5 они читают `continuing on Fable 5` и `Fable 5 now uses usage credits`. До версии v2.1.257 первое сообщение начиналось с `Fable 5 limit reached`.

Это происходит в сеансах [Remote Control](/docs/ru/remote-control), [background sessions](/docs/ru/agent-view) и сеансах товарищей [agent team](/docs/ru/agent-teams). Claude Code показывает запрос согласия только в собственном интерактивном представлении сеанса: терминале, где он работает, или для фонового сеанса, в [agents view](/docs/ru/agent-view) после подключения. Клиент Remote Control не может его отобразить. Claude Code закрывает запрос в крайний срок [`dialogExpiry`](/docs/ru/settings-reference#dialogexpiry), пять минут по умолчанию, или как только появляется новый запрос, пока никто не печатает на этом терминале, например запрос, отправленный из клиента Remote Control. Печать на терминале, где работает сеанс, отменяет крайний срок, и Claude Code ждёт вашего ответа. В присоединённом представлении фонового сеанса печать не отменяет крайний срок, и новый запрос всё ещё закрывает запрос согласия, поэтому ответьте перед тем, как это произойдёт. Claude Code ничего не отправляет и сохраняет вашу модель, поэтому когда вы отправляете следующий запрос, Claude Code снова показывает запрос согласия.

**Что делать:**

* На терминале, где работает сеанс, отправьте другой запрос и ответьте на запрос согласия, когда он появится снова. Для фонового сеанса сначала подключитесь к нему из [agents view](/docs/ru/agent-view). Повторная отправка из клиента Remote Control показывает это сообщение снова, потому что клиент не может отобразить запрос.
* Запустите `/model`, чтобы переключиться на модель, которая не выставляет счёт за кредиты использования
* Чтобы дать себе больше времени для доступа к этому терминалу, установите [`dialogExpiry`](/docs/ru/settings-reference#dialogexpiry) на более длительное значение или `"never"`

До версии v2.1.236 это сообщение не появлялось: пока был подключен клиент Remote Control, Claude Code ждал 60 секунд ответа, а затем продолжал ход на вашей модели по умолчанию.

<h3 id="server-is-temporarily-limiting-requests">
  Server is temporarily limiting requests
</h3>

API применил кратковременное ограничение скорости, которое не связано с квотой вашего плана.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code различает их от лимита вашего плана по отсутствию унифицированных заголовков квоты, которые несёт реальный ответ лимита. Начиная с версии v2.1.199 это [retried automatically](#automatic-retries) с backoff перед отображением, независимо от того, как вы аутентифицируетесь. В более ранних версиях сеанс, вошедший с подпиской claude.ai, не прошёл ход при первом возникновении; только API key и Enterprise sign-ins повторили попытку.

**Что делать:**

* Подождите немного и попробуйте снова
* Проверьте [status.claude.com](https://status.claude.com), если это сохраняется

<h3 id="request-rejected-429">
  Request rejected (429)
</h3>

Вы достигли лимита скорости, настроенного для вашего API key, проекта Amazon Bedrock или проекта Google Cloud.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

Завершающее предложение называет, где проверить здоровье сервиса, и варьируется в зависимости от поставщика. Конфигурации Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry называют страницу статуса этого поставщика вместо страницы статуса Anthropic. Пользовательский `ANTHROPIC_BASE_URL` называет хост шлюза.

**Что делать:**

* Запустите `/status` и подтвердите, что активные учётные данные — это те, которые вы ожидаете. Случайный `ANTHROPIC_API_KEY` в вашей среде может маршрутизировать запросы через низкоуровневый ключ вместо вашей подписки.
* Проверьте консоль вашего поставщика для активных лимитов и запросите более высокий уровень, если необходимо
* Для API keys Anthropic см. [rate limits reference](https://platform.claude.com/docs/en/api/rate-limits) для информации о том, как работают уровни и как установить ограничения на рабочее пространство
* Снизьте параллелизм: понизьте [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/ru/env-vars), избегайте запуска множества параллельных подагентов, или переключитесь на меньшую модель с помощью `/model` для высокообъёмных скриптовых запусков

<h3 id="youve-hit-your-monthly-spend-limit">
  You've hit your monthly spend limit
</h3>

Включённое использование вашего плана не может покрыть этот запрос, и [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), которые в противном случае оплатили бы его, достигли лимита расходов. Это происходит, когда одно из окон использования вашего плана закончилось, или когда запрос — это запрос, который оплачивают только кредиты использования, такой как запрос к модели, которая [bills to usage credits](/docs/ru/model-config#fable-and-usage-credits). Сообщение называет, чей лимит вас заблокировал. Текст после `·` говорит, как увеличить этот лимит, и варьируется в зависимости от вашего плана и того, управляете ли вы выставлением счётов:

```text theme={null}
You've hit your monthly spend limit · raise it at claude.ai/settings/usage
You've hit your individual spend limit · ask your admin for a higher limit
You've hit your org's monthly spend limit · visit claude.ai/admin-settings/usage to raise it
You've hit your team's shared budget · ask your admin to raise it at claude.ai/admin-settings/usage
You've hit your channel's monthly spend limit · an org owner or channel manager can raise it in the channel's Claude settings
```

`team's shared budget` — это объединённый бюджет, который администратор назначил группе, к которой вы принадлежите; сообщение не называет группу. `channel's monthly spend limit` — это бюджет одного канала Slack, в котором работает сеанс, поэтому ваша организация может иметь бюджет вне его.

Когда одно из окон вашего плана — это то, что закончилось, сообщение также говорит, когда это окно сбрасывается, например `· your session limit resets 3:45pm`, и доступ возвращается тогда без того, чтобы кто-либо повышал лимит. В организациях с выставлением счётов на основе использования сообщение говорит `usage limit` вместо `spend limit`, как в `You've hit your individual usage limit`.

До версии v2.1.239 сообщение не называло время сброса окна плана. До версии v2.1.268 объединённый бюджет группы производил сообщение `individual spend limit` вместо `team's shared budget`.

Если вы подключаетесь через шлюз приложений Claude и видите строчное `spend limit reached`, это ограничение вашего оператора шлюза; см. [Spend limit reached](#spend-limit-reached).

**Что делать:**

* На Pro и Max увеличьте ваш ежемесячный лимит расходов в [**Settings > Usage**](https://claude.ai/settings/usage) на claude.ai, или запустите `/usage-credits`
* На Team и Enterprise увеличьте лимит в [**Admin settings > Usage**](https://claude.ai/admin-settings/usage), если вы управляете выставлением счётов, или попросите администратора. `/usage-credits` отправляет этот запрос вашему администратору для вас
* Для лимита канала попросите владельца организации или менеджера канала повысить его на claude.ai. См. [Per-channel limits](https://claude.com/docs/claude-tag/admins/set-spend-limit#per-channel-limits) в документации Claude Tag
* Если сообщение называет время сброса для окна вашего плана, вы можете вместо этого дождаться его
* Запустите `/usage`, чтобы увидеть окна вашего плана и когда каждое сбрасывается

<h3 id="spend-limit-reached">
  Spend limit reached
</h3>

Вы подключаетесь через [Claude apps gateway](/docs/ru/claude-apps-gateway) и прошли [spend cap](/docs/ru/claude-apps-gateway-spend-limits), который установил ваш оператор шлюза. Шлюз блокирует ваши запросы до сброса названного периода или пока оператор не повысит ограничение. Он отмечает каждый заблокированный ответ `429` как `x-should-retry: false`, поэтому Claude Code показывает это сообщение без повторных попыток.

```text theme={null}
spend limit reached (daily; resets 2026-08-09 00:00 UTC)
```

Сообщение называет период ограничения и время сброса, и когда оператор настроил `blocked_message`, его инструкции следуют за ним. До версии v2.1.225 сообщение читалось только `spend limit reached`; шлюз на более старой версии всё ещё отправляет эту более короткую форму.

**Что делать:**

* Дождитесь времени сброса, которое называет сообщение, или следуйте инструкциям оператора, если сообщение их содержит
* Попросите вашего оператора шлюза повысить ограничение, если вы его часто достигаете

Связанное сообщение, `spend limit unavailable`, означает, что шлюз не смог прочитать свои записи расходов и заблокировал запрос в качестве меры предосторожности, а не из-за вашего ограничения. Обычно это очищается само по себе; если это сохраняется, сообщите вашему оператору шлюза.

<h3 id="credit-balance-is-too-low">
  Credit balance is too low
</h3>

Ваша организация Console исчерпала предоплаченные кредиты, или Claude Code отправляет ваши запросы с помощью Console API key, когда вы имели в виду использовать вашу подписку.

```text theme={null}
Credit balance is too low
```

**Что делать:**

* Если у вас есть план Pro, Max, Team или Enterprise и вы видите это, запустите `/status` и проверьте строку `API key`. Одобренный `ANTHROPIC_API_KEY` в вашей среде маршрутизирует запросы через этот ключ вместо вашей подписки. Отмените его в текущей оболочке и удалите из профиля оболочки, затем перезапустите `claude`. Запустите `/login`, если вы ещё не вошли с вашей подпиской.
* Добавьте кредиты на [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing) и рассмотрите возможность включения автоматической перезагрузки там, чтобы баланс пополнялся перед тем, как он упадёт до нуля
* Установите ограничения расходов на рабочее пространство в Console, чтобы предотвратить истощение баланса организации одним проектом. См. [Manage costs effectively](/docs/ru/costs).

<h3 id="could-not-update-your-spend-limit">
  Could not update your spend limit
</h3>

Сервер отклонил изменение лимита расходов, которое вы сделали из подсказки, которая появляется, когда вы достигаете лимита расходов.

```text theme={null}
Could not update your spend limit: <reason from the server>
```

Когда сервер объясняет отклонение, сообщение заканчивается этой причиной, и повторная попытка того же значения снова не удаётся. Когда отказ не имеет предоставленной сервером причины, такой как разорванное соединение, сообщение читает `Could not update your spend limit. Press Enter to retry.` и повторная попытка может быть успешной. До версии v2.1.216 Claude Code показывал общую форму для каждого отказа.

**Что делать:**

* Если сообщение включает причину, выберите лимит, который её удовлетворяет, например меньшую сумму
* Если сообщение показывает только общую форму, повторите попытку; отказ может быть временным
* Если изменение продолжает не удаваться, сделайте его из ваших [claude.ai billing settings](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) в браузере вместо этого

<h2 id="authentication-errors">
  Ошибки аутентификации
</h2>

Эти ошибки означают, что Claude Code не может подтвердить вашу личность перед API. Запустите `/status` в любой момент, чтобы увидеть, какие учетные данные в настоящее время активны.

<h3 id="not-logged-in">
  Not logged in
</h3>

Для этого сеанса нет доступных действительных учетных данных.

```text theme={null}
Not logged in · Please run /login
```

**Что делать:**

* Запустите `/login` для аутентификации с помощью вашей подписки Claude или учетной записи Console
* Если вы ожидали, что переменная окружения будет аутентифицировать вас, убедитесь, что `ANTHROPIC_API_KEY` установлена и экспортирована в оболочке, где вы запустили `claude`
* Для CI или автоматизации, где интерактивный вход невозможен, настройте скрипт [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper), который получает ключ при запуске
* См. [Authentication precedence](/docs/ru/authentication#authentication-precedence), чтобы понять, какие учетные данные Claude Code использует, когда присутствует несколько

Если вам предлагается войти повторно, см. [Not logged in or token expired](/docs/ru/troubleshoot-install#not-logged-in-or-token-expired) для проверки системных часов и шагов восстановления хранилища учетных данных macOS.

<h3 id="could-not-resolve-authentication-method">
  Could not resolve authentication method
</h3>

Сеанс достиг клиента API без каких-либо учетных данных. [Background sessions](/docs/ru/agent-view) и облачные сеансы показывают это сообщение, когда рабочий процесс запускается без учетных данных. Интерактивные, `-p` и запуски Agent SDK сообщают о том же состоянии, что и [Not logged in](#not-logged-in) и записывают эту строку только в журнал отладки, поэтому если вы нашли ее там, следуйте этой записи вместо этого.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

В текущих версиях ошибка означает, что рабочему процессу не были доступны учетные данные. До v2.1.174 фоновый сеанс, назначенный неактивному предварительно инициализированному рабочему процессу, мог завершиться ошибкой таким образом, даже если были настроены действительные учетные данные. До v2.1.176 облачный сеанс, который был неактивен перед тем, как быть заявленным, тоже мог. Обновитесь для восстановления.

**Что делать:**

* Обновитесь до v2.1.176 или более поздней версии, если это появляется в фоновом или облачном сеансе и ваши учетные данные уже настроены
* Убедитесь, что `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` или ваши учетные данные облачного провайдера установлены в окружении, которое запускает рабочий процесс, а не только в вашей интерактивной оболочке
* Для Agent SDK см. [authentication setup in the quickstart](/docs/ru/agent-sdk/quickstart#setup)
* Запустите `/status` в интерактивном сеансе в том же окружении, чтобы подтвердить, какой источник учетных данных разрешается

<h3 id="invalid-api-key">
  Invalid API key
</h3>

Переменная окружения `ANTHROPIC_API_KEY` или скрипт `apiKeyHelper` вернули ключ, который API отклонил, или Claude Code заблокировал ключ из `ANTHROPIC_API_KEY` перед его отправкой.

```text theme={null}
Invalid API key · Fix external API key
```

Когда сообщение продолжается после `Fix external API key` с описанием, таким как `Invalid X-Api-Key header value from ANTHROPIC_API_KEY: it contains a line break at character 41 (120 characters on 2 lines).`, API никогда не видел ключ. Claude Code обнаружил символ, который HTTP-заголовки не могут передавать, и остановил запрос перед его отправкой. См. [Invalid request header value](#invalid-request-header-value) для того, как прочитать описание и исправить значение.

**Что делать:**

* Проверьте опечатки и убедитесь, что ключ не был отозван в [Console](https://platform.claude.com/settings/keys)
* В той же оболочке запустите `env | grep ANTHROPIC`, или в PowerShell `Get-ChildItem Env:ANTHROPIC*`. Такие инструменты, как direnv, плагины dotenv shell и терминалы IDE, могут загружать устаревший ключ из файла `.env` в вашем проекте без явной установки.
* Отмените установку `ANTHROPIC_API_KEY` и запустите `/login` для использования аутентификации подписки вместо этого
* Если ключ поступает из скрипта [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper), запустите скрипт напрямую, чтобы подтвердить, что он выводит действительный ключ на stdout
* Запустите `/status`, чтобы подтвердить, какой источник учетных данных Claude Code фактически использует

<h3 id="your-apikeyhelper-script-is-failing">
  Your apiKeyHelper script is failing
</h3>

Claude Code запустил команду в вашей настройке [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper) и не получил ключ обратно. Без него запрос достигает API с заполнительными учетными данными, и API отклоняет его с `401`. Панель `Authentication` в терминале показывает, что произошло одно из следующих:

* Команда завершилась с ошибкой или истекло время ожидания
* Команда ничего не вывела на stdout
* Команда вывела что-то, кроме ключа, например баннер входа или строку журнала. Панель показывает `returned output that cannot be used as an API key` и говорит, что не так, без повторения вывода. До v2.1.227 Claude Code отправлял все, что выводила команда, после обрезки окружающих пробелов.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

В [non-interactive mode](/docs/ru/headless) stderr также содержит конкретную причину с префиксом `apiKeyHelper failed:`.

Claude Code повторно запускает скрипт и повторяет попытку запроса еще два раза перед отображением этого сообщения, поэтому сбой проявляется в течение трех попыток. До v2.1.208 Claude Code потратил полный [retry budget](#automatic-retries) на повторную отправку запроса с заполнительными учетными данными, а затем сообщил об общей ошибке аутентификации `401` вместо сбоя скрипта.

Запуск `/login` не помогает здесь: вывод помощника [takes precedence](/docs/ru/authentication#authentication-precedence) над сохраненным входом, пока параметр присутствует.

**Что делать:**

* Запустите команду, настроенную в `apiKeyHelper`, напрямую в вашей оболочке, чтобы воспроизвести сбой
* Если команда сообщает об истекшем сеансе, повторно аутентифицируйтесь у вашего поставщика учетных данных, например, снова войдя в свой SSO или хранилище секретов
* Исправьте команду так, чтобы она выводила только ключ на stdout как один токен печатаемого ASCII до 16 384 символов и выходила с кодом 0. См. [rotate credentials with apiKeyHelper](/docs/ru/llm-gateway-connect#rotate-credentials-with-apikeyhelper) для рабочей установки.
* Запустите `/status`, чтобы подтвердить, что `apiKeyHelper` является активным источником учетных данных. Строка `apiKeyHelper` показывает `Failing` с деталью последнего сбоя, такой как код выхода и вывод ошибки команды, и исчезает после следующего успешного запуска. До v2.1.274 `/status` показал только источник учетных данных, а не сбой.
* Каждый раз, когда команда не выполняется, ее код выхода и вывод ошибки также появляются в панели `Authentication` в терминале. До v2.1.212 панель была озаглавлена `Cloud authentication`.

<h3 id="invalid-request-header-value">
  Invalid request header value
</h3>

Значение, которое Claude Code собирался отправить как заголовок запроса, содержит символ, который HTTP-заголовки не могут передавать: разрыв строки, байт NUL или символ выше `U+00FF`, такой как фигурная кавычка или нулевой пробел. Claude Code останавливает запрос перед отправкой чего-либо и называет переменную или параметр для исправления. Обычная причина — учетные данные, вставленные из документа или чата, которые содержали невидимый символ или случайный разрыв строки.

Claude Code запускает эту проверку при отправке запросов к Claude API напрямую или через [LLM gateway](/docs/ru/llm-gateway). На поставщике облачных услуг третьей стороны, таком как [Amazon Bedrock](/docs/ru/amazon-bedrock), Claude Code не запускает ее перед отправкой.

```text theme={null}
Invalid auth token · Fix external auth token
Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable
Invalid request header from the environment · Fix the environment variable
```

Первая часть сообщения зависит от того, откуда поступило плохое значение:

* `Invalid auth token`: токен-носитель из [`ANTHROPIC_AUTH_TOKEN`](/docs/ru/env-vars) или [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/ru/env-vars)
* `Invalid ANTHROPIC_CUSTOM_HEADERS`: имя или значение заголовка, которое вы установили в [`ANTHROPIC_CUSTOM_HEADERS`](/docs/ru/env-vars). Описание подсчитывает, какая пара `Name: Value` виновата, например `distinct header 2 of 3 parsed from ANTHROPIC_CUSTOM_HEADERS`, без повторения имени или значения, так как вы выбрали оба.
* `Invalid request header from the environment`: значение, которое Claude Code копирует в заголовок запроса из другой переменной окружения, такой как `CLAUDE_AGENT_SDK_CLIENT_APP`. Описание называет переменную для исправления.

Claude Code сообщает о плохом `ANTHROPIC_API_KEY`, перехваченном этой проверкой, как [Invalid API key](#invalid-api-key), с тем же завершающим описанием. Он сообщает о плохих сохраненных учетных данных `/login` как [Not logged in](#not-logged-in) вместо этого; запустите `/login`, чтобы сохранить свежие. Вывод скрипта [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper) никогда не достигает этой проверки: Claude Code проверяет его при запуске скрипта, и вывод, который HTTP-заголовок не может передавать, завершается ошибкой [Your apiKeyHelper script is failing](#your-apikeyhelper-script-is-failing).

После второго `·` сообщение описывает проблему, как в этом полном примере:

```text theme={null}
Invalid auth token · Fix external auth token · Invalid Authorization header value from ANTHROPIC_AUTH_TOKEN: it contains a line break at character 41 (120 characters on 2 lines).
```

Позиции считают символы, начиная с одного. Описание построено из фиксированных фраз и подсчетов символов, поэтому оно никогда не включает само значение. Оно называет оскорбительный символ только тогда, когда это хорошо известный невидимый или типографский символ, такой как метка порядка байтов, нулевой пробел или фигурная кавычка, и сообщает обо всем остальном как `a non-ASCII character`.

**Что делать:**

* Переустановите переменную или параметр, который называет сообщение, переписав символы вокруг указанной позиции, а не вставляя из того же источника снова
* Для `ANTHROPIC_CUSTOM_HEADERS` держите одну пару `Name: Value` на строку и переписывайте пару, которую считает сообщение
* Запустите `/status`, чтобы подтвердить, какой источник учетных данных активен

<h3 id="this-organization-has-been-disabled">
  This organization has been disabled
</h3>

Claude Code использует устаревший `ANTHROPIC_API_KEY` из отключенной организации Console. Когда у вас есть сохраненный вход подписки, ключ переопределяет его.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your subscription instead
Your ANTHROPIC_API_KEY belongs to a disabled organization · Update or unset the environment variable
API Error: 400 ... This organization has been disabled.
```

Подсказка после `·` зависит от ваших сохраненных учетных данных: первая форма появляется, когда сохраненный `/login` может взять верх после отмены установки ключа, а вторая — когда ключ является вашим единственным учетным данным.

Переменные окружения имеют приоритет над `/login`, поэтому ключ, экспортированный в профиль вашей оболочки или загруженный из файла `.env`, используется даже при наличии рабочей подписки Pro или Max. В неинтерактивном режиме (`-p`) ключ всегда используется при наличии.

**Что делать:**

* Отмените установку `ANTHROPIC_API_KEY` в текущей оболочке и удалите его из профиля вашей оболочки, затем перезапустите `claude`
* Если сообщение говорит `Update or unset`, у вас нет сохраненного входа для отката. Отмените установку ключа и запустите `/login`, или замените ключ на ключ из активной организации Console.
* Запустите `/status` после этого, чтобы подтвердить, что активные учетные данные — это ваша подписка
* Если переменная окружения не установлена и ошибка сохраняется, отключенная организация — это та, которая связана с вашим `/login`. Свяжитесь с поддержкой или войдите с другой учетной записью.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Your organization has disabled API key authentication
</h3>

Это сообщение требует Claude Code v2.1.169 или более поздней версии. Администратор организации Console отключил аутентификацию по ключу API, поэтому API отклоняет ключ, который отправляет Claude Code. Подсказка восстановления после `·` варьируется в зависимости от того, откуда поступил ключ:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Переменные окружения и `apiKeyHelper` имеют приоритет над `/login`, поэтому запуск только `/login` не помогает, пока один из них все еще предоставляет ключ. См. [Authentication precedence](/docs/ru/authentication#authentication-precedence).

**Что делать:**

* Если сообщение называет `ANTHROPIC_API_KEY`, отмените его установку в текущей оболочке и удалите из профиля вашей оболочки или файла `.env`, затем перезапустите `claude`
* Если сообщение называет `apiKeyHelper`, удалите параметр [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper) из вашего `settings.json`
* Запустите `/login` для входа с вашей учетной записью claude.ai
* Запустите `/status` после этого, чтобы подтвердить, что активные учетные данные — это ваша подписка, а не ключ API
* Если вам нужна аутентификация по ключу API для автоматизации, попросите администратора вашей организации повторно включить ее в Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Your organization has disabled Claude subscription access
</h3>

Ваша организация Claude не позволяет входить в Claude Code с входом подписки. Повторный запуск `/login` с той же учетной записью возвращает ту же ошибку.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Это параметр организации на стороне сервера, поэтому его нельзя переопределить из локальных параметров, переменных окружения или флагов CLI.

Agent SDK и режим неинтерактивности `-p` представляют это как код ошибки `oauth_org_not_allowed`.

**Что делать:**

* Попросите администратора включить доступ Claude Code для вашей организации
* Аутентифицируйтесь с помощью ключа API Console вместо вашей подписки. См. [Claude Console authentication](/docs/ru/authentication#claude-console-authentication) для установки.
* Если вы администратор и не видите опцию для включения доступа, свяжитесь с [Anthropic support](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Routines are disabled by your organization's policy
</h3>

An Owner в вашей организации Team или Enterprise отключил routines на уровне организации. Ошибка появляется при попытке создать или запустить routine, например из пользовательского интерфейса [Routines](/docs/ru/routines) на claude.ai/code. На Claude Code v2.1.227 или более поздней версии тот же параметр также [hides `/schedule`](/docs/ru/routines#troubleshooting) в CLI.

```text theme={null}
Routines are disabled by your organization's policy.
```

Это параметр на стороне сервера, поэтому его нельзя переопределить из локальных параметров, переменных окружения или флагов CLI.

**Что делать:**

* Попросите владельца в вашей организации включить переключатель **Routines** на [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Для одноразовой запланированной работы, которая не требует routines на уровне организации, см. [scheduled tasks](/docs/ru/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control requires the Anthropic API
</h3>

Сеанс не разговаривает с Anthropic API напрямую, поэтому нет бэкенда claude.ai для [Remote Control](/docs/ru/remote-control) для сопряжения.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com. CLAUDE_CODE_USE_BEDROCK is set, so this session is using Amazon Bedrock — unset it (or run in a shell without it) to use Remote Control.
```

Второе предложение объясняет, что направило сеанс от Anthropic API; до v2.1.219 сообщение было только первым предложением. В зависимости от причины сообщение называет:

* Переменную поставщика `CLAUDE_CODE_USE_*`, такую как `CLAUDE_CODE_USE_BEDROCK` для [Amazon Bedrock](/docs/ru/amazon-bedrock) или `CLAUDE_CODE_USE_VERTEX` для [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai)
* [`ANTHROPIC_BASE_URL`](/docs/ru/env-vars), указывающий на хост, отличный от `api.anthropic.com`, такой как [LLM gateway](/docs/ru/llm-gateway) или прокси, даже при входе с claude.ai; до v2.1.196 пользовательский базовый URL не блокировал Remote Control
* `ANTHROPIC_UNIX_SOCKET` установлен, поэтому сеанс отправляет свои запросы через локальный сокет, а не на `api.anthropic.com`
* Вход в корпоративный [cloud gateway](/docs/ru/claude-apps-gateway) через `/login`, который не поддерживает Remote Control и не имеет переменной для отмены установки

**Что делать:**

* Отмените установку переменной, которую называет сообщение, такой как `CLAUDE_CODE_USE_BEDROCK` или `ANTHROPIC_BASE_URL`, и перезапустите сеанс, или запустите Remote Control из сеанса, который разговаривает с Anthropic API напрямую
* Если переменная не установлена в вашей оболочке, проверьте ключ `env` в ваших [settings files](/docs/ru/settings#where-settings-live), который применяет переменные окружения к каждому сеансу
* Для этого и других сообщений запуска Remote Control см. [Troubleshoot Remote Control](/docs/ru/remote-control#troubleshooting)

<h3 id="remote-control-couldnt-refresh-your-login">
  Remote Control couldn't refresh your login
</h3>

Claude Code запускает живое соединение [Remote Control](/docs/ru/remote-control) на краткосрочных учетных данных, которые он получает и обновляет, используя ваш сохраненный вход claude.ai. Когда claude.ai перестает принимать этот вход, или Claude Code больше не имеет сохраненного входа, Claude Code останавливает Remote Control и просит вас войти снова. Любой сбой может произойти, пока Claude Code все еще подключается или позже, когда он обновляет учетные данные.

Когда Claude Code просит службу входа обновить ваш сохраненный вход и не получает ответ, он продолжает запускать Remote Control и пытается обновление снова, пока учетные данные текущего соединения все еще действительны. Обновление не получает ответ, когда Claude Code не может достичь службы входа, запрос истекает, или служба завершается ошибкой без отклонения вашего входа. Если служба входа все еще не отвечает, когда эти учетные данные истекают, Claude Code останавливает Remote Control и сообщает `OAuth token refresh failed`.

Когда Claude Code останавливает Remote Control, он показывает причину в предупреждении и в строке стенограммы, которая начинается с `Remote Control disconnected`. Ваш локальный сеанс продолжает работать без Remote Control. Этот раздел охватывает эти строки:

```text theme={null}
Remote Control disconnected — Claude.ai login expired — run /login to restore Remote Control
Remote Control disconnected — Claude.ai login expired — run /login, then /remote-control
Remote Control disconnected — Claude.ai login was rejected — run /login, then /remote-control
Remote Control disconnected — OAuth token unavailable — run /login to restore Remote Control
Remote Control disconnected — OAuth token refresh failed — run /login to re-authenticate
Remote Control disconnected — JWT refresh failed: no OAuth token — run /login
Remote Control disconnected — Signed out of Claude — run /login, then /remote-control
```

Claude Code называет причину в середине сообщения:

* `Claude.ai login expired` и `Claude.ai login was rejected`: claude.ai больше не принимает ваш сохраненный токен входа, потому что он истек или был отозван
* `OAuth token unavailable`: Claude Code не имел сохраненного токена входа, когда учетные данные соединения пришли на обновление
* `OAuth token refresh failed`: claude.ai отклонил ваш сохраненный токен входа, пока Claude Code переподключался, и обновление токена не произвело новый
* `JWT refresh failed: no OAuth token`: Claude Code не нашел сохраненный токен входа для обновления
* `Signed out of Claude`: вы вышли на этой машине, например, запустив `/logout` в другом терминале, поэтому Claude Code больше не имеет сохраненного входа для обновления соединения

**Что делать:**

* Запустите `/login` для входа снова
* Запустите `/remote-control` для переподключения сеанса. Сообщения, заканчивающиеся `run /login to restore Remote Control`, не нуждаются в этом шаге: Claude Code переподключается автоматически после входа.

До v2.1.224 `OAuth token refresh failed — run /login to re-authenticate` читалось как `OAuth token refresh failed — re-authenticate, then re-enable Remote Control`, и `JWT refresh failed: no OAuth token — run /login` читалось как `no OAuth token available for recovery (code <N>)`. Сообщения `Claude.ai login expired`, `Claude.ai login was rejected` и `OAuth token unavailable` были добавлены в v2.1.225.

До v2.1.238 Claude Code сообщал о случаях, которые теперь говорят `Signed out of Claude`, как `JWT refresh failed: no OAuth token — run /login`, и останавливал Remote Control с `Claude.ai login expired — run /login to restore Remote Control` как только одно обновление входа не получило ответ.

<h3 id="remote-control-stopped-because-the-signed-in-account-changed">
  Remote Control stopped because the signed-in account changed
</h3>

Claude Code показывает эту строку во время сеанса [Remote Control](/docs/ru/remote-control), когда вы входите в другую учетную запись claude.ai или организацию на этой машине. Вы сделали переключение вне сеанса Claude Code, например, запустив `/login` в другом терминале.

Сеанс Remote Control, который вы запустили, пока были подписаны через `/login`, принадлежит учетной записи claude.ai и организации, которые были подписаны в то время.

```text theme={null}
Remote Control disconnected — signed-in claude.ai account or organization changed on this machine — run /remote-control to start a session for the current account, or /login to switch back, then /remote-control
```

Claude Code останавливает сеанс Remote Control как только claude.ai подтверждает, что учетная запись или организация изменилась. Ваш локальный сеанс продолжает работать без Remote Control.

**Что делать:**

* Запустите `/remote-control` для запуска нового сеанса Remote Control под текущей учетной записью или организацией
* Для переключения назад запустите `/login` и войдите в предыдущую учетную запись или организацию снова. Затем запустите `/remote-control`.

До v2.1.234 Claude Code не замечал, когда вы переключались на другую учетную запись или организацию вне сеанса Claude Code. Claude Code держал сеанс Remote Control подключенным до более позднего запроса к серверу Remote Control, который завершился ошибкой `Remote Control server rejected the request (HTTP 404)`. Этот сбой мог произойти часами после переключения.

<h3 id="remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts">
  Remote Control stopped because the app running the session signed out or switched accounts
</h3>

Когда приложение Claude desktop или IDE размещает ваш сеанс, Claude Code получает свой токен входа из этого приложения, а не из `/login`. Когда claude.ai отклоняет этот токен, Claude Code просит приложение о новом. Если приложение отвечает, что оно вышло, или что оно теперь подписано на другую учетную запись Claude, Claude Code заканчивает сеанс [Remote Control](/docs/ru/remote-control) и отправляет приложению одну из этих строк:

```text theme={null}
Remote Control stopped — the app running this session is now signed in to a different Claude account
Remote Control stopped — the app running this session is signed out of Claude. Sign in there, then turn Remote Control back on
```

Ваш локальный сеанс продолжает работать без Remote Control.

**Что делать:**

* Если приложение вышло, войдите в него снова, затем включите Remote Control снова в приложении
* Если приложение переключило учетные записи, Claude Code не может продолжить завершенный сеанс под новой учетной записью. Запустите новый сеанс Remote Control под этой учетной записью.

До v2.1.238 Claude Code отправлял приложению сообщения `run /login`, перечисленные в [Remote Control couldn't refresh your login](#remote-control-couldnt-refresh-your-login) в обоих случаях.

<h3 id="oauth-token-revoked-or-expired">
  OAuth token revoked or expired
</h3>

Ваш сохраненный вход больше не действителен. Отозванный токен означает, что вы вышли везде или администратор удалил доступ; истекший токен означает, что автоматическое обновление не удалось в середине сеанса.

Оба сообщения сообщают об отклонении, которое API вернул для запроса, который отправил Claude Code. Когда сохраненный вход уже был очищен после неудачного обновления, вы видите [Login expired](#login-expired) вместо этого. Если вы аутентифицируетесь с долгоживущим токеном в [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/ru/env-vars), вы видите те же сообщения, когда этот токен истекает или отзывается.

```text theme={null}
OAuth token revoked · Please run /login
Please run /login · API Error: 401 OAuth token has expired ...
```

**Что делать:**

* Запустите `/login` для входа снова
* Если ошибка возвращается в том же сеансе после повторной аутентификации, сначала запустите `/logout` для полной очистки сохраненного токена, затем `/login`
* Если вы аутентифицируетесь с переменной окружения `CLAUDE_CODE_OAUTH_TOKEN`, Claude Code продолжает отправлять установленное вами значение после того, как запрос завершится ошибкой 401, а не переключаться на токен сохраненного входа. [`/status`](/docs/ru/commands) показывает это учетное данное как строку `Auth token`, читающую `CLAUDE_CODE_OAUTH_TOKEN`. Создайте свежий токен с помощью [`claude setup-token`](/docs/ru/authentication#generate-a-long-lived-token) и перезапустите с ним, или отмените установку переменной и запустите `/login`. До v2.1.225 Claude Code мог заменить значение переменной в середине сеанса краткосрочным токеном доступа из сохраненного входа, и сеанс снова завершился ошибками 401 после истечения этого токена.
* Для повторных запросов на вход при запусках см. проверки системных часов и шаги восстановления хранилища учетных данных macOS в [Troubleshooting](/docs/ru/troubleshoot-install#not-logged-in-or-token-expired)
* Для других сбоев, включая `403 Forbidden` и проблемы браузера OAuth, см. [Login and authentication](/docs/ru/troubleshoot-install#login-and-authentication)

<h3 id="api-error-401-invalid-authentication-credentials">
  API Error: 401 Invalid authentication credentials
</h3>

API распознал формат вашего учетного данного, но отклонил учетную запись или организацию позади него. Anthropic возвращает это сообщение, когда учетное данное было недавно отозвано, когда организация была отключена или удалила ваш доступ, или когда сама учетная запись была деактивирована, поэтому истекший токен не является причиной. Учетное данное может быть вашим сохраненным входом или одобренным `ANTHROPIC_API_KEY`, и исправление отличается, поэтому начните с запуска `/status`, чтобы увидеть, какой активен.

```text theme={null}
Please run /login · API Error: 401 Invalid authentication credentials
```

**Что делать:**

* Если `/status` показывает строку `API key`, которая не отмечена как не используется, одобренный [`ANTHROPIC_API_KEY`](/docs/ru/authentication#authentication-precedence) является активным учетным данным и имеет приоритет над вашим входом, поэтому `/login` не заменяет его. Поверните ключ в Claude Console, или вернитесь к вашей подписке, запустив `unset ANTHROPIC_API_KEY`, или в PowerShell `Remove-Item Env:ANTHROPIC_API_KEY`.
* Если `/status` показывает только ваш вход, запустите `/login` один раз. Если учетное данное было отозвано, свежий вход заменяет его.
* Если то же сообщение возвращается для той же учетной записи входа, учетная запись или организация больше не активны. Проверьте учетную запись и организацию, которые сообщает `/status`, и попросите администратора вашей организации восстановить доступ.
* Если [`ANTHROPIC_BASE_URL`](/docs/ru/env-vars) указывает на [LLM gateway](/docs/ru/llm-gateway), текст после `401` — это сообщение вашего шлюза, а не Anthropic, и `/login` не изменяет его. Исправьте учетное данное, которое ожидает ваш шлюз.

<h3 id="login-expired">
  Login expired
</h3>

Claude Code попытался обновить ваш сохраненный вход claude.ai или Claude Console, и служба OAuth отклонила сохраненный токен обновления, поэтому Claude Code очистил сохраненные учетные данные. После этого каждый запрос модели останавливается локально с этим сообщением перед тем, как достичь API, потому что только `/login` может создать новые учетные данные.

До v2.1.206 Claude Code отправлял запрос модели в любом случае с любым оставшимся учетным данным в окружении, и каждая модель затем завершалась ошибкой [There's an issue with the selected model](#theres-an-issue-with-the-selected-model) или 401 вместо запроса на вход.

```text theme={null}
Login expired · Please run /login
```

В [non-interactive mode](/docs/ru/headless) (`-p`) и [Agent SDK](/docs/ru/agent-sdk/overview) сообщение читается следующим образом, и код структурированной ошибки — `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Это не то же состояние, что [OAuth token revoked or expired](#oauth-token-revoked-or-expired). Эти сообщения сообщают об отклонении, которое API вернул. Claude Code сам производит `Login expired` для входа, который уже не удалось обновить, поэтому он не отправляет запрос. Когда обновление не удается, потому что сама учетная запись приостановлена, а не потому, что вход устарел, Claude Code показывает [Your account is on hold](#your-account-is-on-hold) вместо этого.

Сеансы, аутентифицированные с помощью ключа API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/ru/env-vars) или поставщика третьей стороны, не используют сохраненный вход и никогда не видят это сообщение.

Вы можете проверить это состояние перед тем, как запрос завершится ошибкой: [`/status`](/docs/ru/commands) показывает строку `Login`, читающую `Expired — log in again`, плюс организацию и электронную почту, которые она сохранила для истекшего входа. Строка появляется только, когда сохраненный вход является вашим активным учетным данным и больше не может быть обновлен. Сеансы, аутентифицированные другим способом, не показывают строку, даже если истекший вход остается сохраненным. До v2.1.210 `/status` не давал никакого указания в этом состоянии, что вход когда-либо существовал, потому что очищенное учетное данные оставило ему нечего сообщать.

**Что делать:**

* Запустите `/login` для входа снова. Повторная попытка без входа показывает то же сообщение на каждом запросе.
* В неинтерактивном режиме запустите `claude` в том же окружении, завершите `/login`, затем перезапустите вашу команду. Для автоматизации, которая не может войти интерактивно, аутентифицируйтесь с помощью `ANTHROPIC_API_KEY` или [generate a long-lived token with `claude setup-token`](/docs/ru/authentication#generate-a-long-lived-token).
* Если вход продолжает не удаваться, см. [Login and authentication](/docs/ru/troubleshoot-install#login-and-authentication)

<h3 id="claude-login-not-accepted">
  Claude login not accepted
</h3>

Вы попытались запустить [cloud session](/docs/ru/claude-code-on-the-web), и сервер отказался создать его с 401: он не принял вход Claude, который отправила эта машина, обычно потому, что вход истек или был отозван.

Первая часть строки — это собственная причина сервера, когда он дает одну. В противном случае строка читается:

```text theme={null}
Claude login not accepted · Run /login, then try again
```

**Что делать:**

* Запустите `/login`, завершите вход, затем запустите сеанс снова

<h3 id="artifacts-need-a-claude-ai-login">
  Artifacts need a claude.ai login
</h3>

Claude Code отказал [artifact](/docs/ru/artifacts) публикации или чтению, потому что сеанс не имеет входа claude.ai, который он может использовать для артефактов.

Каждая форма сообщения начинается с одних и тех же слов, за которыми следует средство, которое зависит от того, как ваш сеанс аутентифицируется. Без конкурирующего учетного данного он читается:

```text theme={null}
Artifacts need a claude.ai login. Run /login and select "Claude account with subscription", then retry — the "Anthropic Console account" option does not provide claude.ai credentials.
```

**Что делать:**

* Запустите `/login` и выберите **Claude account with subscription**. Опция **Anthropic Console account** не предоставляет учетные данные claude.ai.
* Когда сообщение называет учетное данное, которое имеет приоритет, такое как `ANTHROPIC_API_KEY`, параметр `apiKeyHelper` или ключ Console, сохраненный предыдущим `/login`, удалите его так, как говорит сообщение, затем запустите `/login`
* Когда сообщение говорит, что этот удаленный сеанс аутентифицируется через машину, которая его запустила, войдите в claude.ai на этой машине, затем переподключите сеанс
* Когда сообщение говорит, что учетные данные вводятся окружением хоста сеанса, вы не можете изменить их в этом сеансе; запустите сеанс, который подписан на claude.ai
* См. [Availability](/docs/ru/artifacts#availability) для других требований, которые имеют артефакты, такие как план, поставщик модели и политика организации

<h3 id="administrator-policy-requires-a-cloud-gateway-sign-in">
  Administrator policy requires a Cloud gateway sign-in
</h3>

Параметр [managed settings](/docs/ru/managed-settings) администратора на этой машине установил [`forceLoginMethod`](/docs/ru/settings-reference#forceloginmethod) на `"gateway"` или установил [`forceLoginGatewayUrl`](/docs/ru/settings-reference#forcelogingatewayurl). Если вы не выбираете поставщика облачных услуг через переменную, такую как `CLAUDE_CODE_USE_BEDROCK`, Claude Code затем принимает только вход [Claude apps gateway](/docs/ru/claude-apps-gateway). Вы видите одно из двух сообщений:

```text theme={null}
Not signed in to the Cloud gateway — run /login.
```

Запросы модели завершаются ошибкой с этим сообщением, когда сеанс не имеет входа шлюза, например, потому что вы не запустили `/login` с момента достижения политики машины.

Если у вас также есть учетное данное `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` или `apiKeyHelper`, настроенное, и параметры управления установили `forceLoginMethod`, Claude Code выходит при запуске вместо этого с сообщением, которое начинается:

```text theme={null}
Administrator policy requires a Cloud gateway sign-in on this machine; the
Anthropic-issued credential configured here (ANTHROPIC_API_KEY,
ANTHROPIC_AUTH_TOKEN, or apiKeyHelper) is not used.
```

**Что делать:**

* Запустите `/login` и завершите вход на экране **Cloud gateway**
* Для сообщения при запуске удалите параметр `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` или `apiKeyHelper`, который вы настроили, затем запустите `claude` и запустите `/login`
* Если вы считаете, что машина не должна требовать шлюз, попросите администратора, который управляет ею, удалить `forceLoginMethod` и `forceLoginGatewayUrl` из его параметров управления

На v2.1.265 регрессия также показала первое сообщение в некоторых конфигурациях LLM-gateway и прокси, которые аутентифицируются с помощью ключа API, `apiKeyHelper` или пользовательских заголовков, даже без требования администратора на машине. Обновитесь до v2.1.266 или более поздней версии. Вам не нужно менять вашу конфигурацию.

До v2.1.261 на машинах, которые установили `forceLoginMethod` на `"gateway"`, Claude Code использовал оставшийся сохраненный вход вместо того, чтобы завершить запросы модели ошибкой, и сообщал о настроенном учетном данном окружения с `This machine's managed settings require a first-party login` вместо сообщения при запуске. До v2.1.265 машина, чьи параметры управления установили только `forceLoginGatewayUrl`, не требовала входа шлюза, и Claude Code использовал оставшееся учетное данные там.

<h3 id="your-account-is-on-hold">
  Your account is on hold
</h3>

Учетная запись Claude позади вашего входа была приостановлена. Claude Code показывает первое сообщение, когда он пытается обновить ваш сохраненный вход и узнает о задержке, и второе, когда завершенный вами вход в браузер сообщает об этом:

```text theme={null}
Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted
Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted
```

Повторный вход с той же учетной записью не очищает сообщение, потому что задержка находится на учетной записи, а не на входе. В [non-interactive mode](/docs/ru/headless) (`-p`) и [Agent SDK](/docs/ru/agent-sdk/overview) код структурированной ошибки — `account_on_hold`. До v2.1.235 Claude Code сообщал о задержанной учетной записи как [Login expired · Please run /login](#login-expired), чьи шаги восстановления не могут очистить задержку.

**Что делать:**

* Откройте ссылку в сообщении, чтобы просмотреть детали задержки или обжаловать ее
* Если у вас есть другая учетная запись Claude или ключ API, на который не влияет задержка, вы можете продолжить работу, пока задержка разрешается: запустите `/login` с этой учетной записью, или установите ключ с помощью `ANTHROPIC_API_KEY`

<h3 id="anthropic-profile-login-expired">
  Anthropic profile login expired
</h3>

Claude Code аутентифицируется через профиль учетных данных Anthropic, чей сохраненный вход истек, и профиль не содержит учетные данные обновления, которые Claude Code может использовать для его обновления. Claude Code останавливает каждый запрос локально без повторных попыток, потому что повторная попытка прочитает то же истекшее учетное данные.

```text theme={null}
Anthropic profile login expired · Re-authenticate your Anthropic profile
Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile
```

Это появляется только, когда активное учетное данные поступает из профиля учетных данных Anthropic, который вы выбираете с переменной окружения `ANTHROPIC_PROFILE`, который Claude Code обнаруживает как активный профиль в вашем каталоге конфигурации Anthropic, или который Claude Code написал, когда вы [signed in without an API key](/docs/ru/authentication#sign-in-without-an-api-key). Сеансы, которые аутентифицируются с помощью опции claude.ai `/login`, ключа API, токена-носителя, такого как `ANTHROPIC_AUTH_TOKEN`, или поставщика третьей стороны, никогда не видят это сообщение.

На машине, которая [offers the keyless sign-in](/docs/ru/authentication#sign-in-without-an-api-key), запустите `/login`, выберите учетную запись Anthropic Console и войдите снова, чтобы обновить профиль, который написал вход без ключа Console или CLI Claude Platform `ant auth login`. Claude Code заменяет истекшее учетные данные в этом профиле. Для профиля федерации или профиля, созданного другим инструментом, `/login` не обновляет учетные данные. Какую форму вы видите, зависит от того, выбрали ли вы профиль или Claude Code его обнаружил:

* Когда вы явно установили `ANTHROPIC_PROFILE`, сообщение заканчивается с `Re-authenticate your Anthropic profile`.
* Когда Claude Code обнаружил профиль из вашего каталога конфигурации, сообщение предлагает `/login`, потому что Claude Code дает рабочему `/login` приоритет над обнаруженным профилем и затем аутентифицируется с вашей учетной записью claude.ai или Console вместо этого. До v2.1.234 Claude Code показал форму `Re-authenticate your Anthropic profile` в этом случае тоже.

**Что делать:**

* Войдите в профиль снова, затем повторите попытку: на машине, которая [offers the keyless sign-in](/docs/ru/authentication#sign-in-without-an-api-key), запустите `/login` и выберите учетную запись Anthropic Console для профиля, который написал вход без ключа Console или CLI Claude Platform `ant auth login`; для других профилей используйте инструмент, который их создал
* Если администратор подготовил учетные данные профиля, попросите его выдать новое
* Запустите `/status`, чтобы подтвердить активный источник учетных данных и имя профиля
* Чтобы перестать использовать профиль, отмените установку `ANTHROPIC_PROFILE`, если вы его установили, затем аутентифицируйтесь другим способом, например `/login` или `ANTHROPIC_API_KEY`

<h3 id="oauth-scope-requirement">
  OAuth scope requirement
</h3>

Сохраненный токен предшествует области разрешений, которая требуется более новой функции. Вы видите это чаще всего из `/usage` и индикатора использования строки состояния:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**Что делать:**

* Запустите `/login`, чтобы получить новый токен с текущими областями. Вам не нужно сначала выходить.

<h3 id="claude-ai-rejected-the-session-token">
  claude.ai rejected the session token
</h3>

Запрос [claude.ai connector](/docs/ru/mcp#use-mcp-servers-from-claude-ai) завершился ошибкой, потому что claude.ai отклонил токен из вашего входа Claude Code, обычно вход, который истек и не мог быть обновлен. Отклоненный токен — это ваш вход, а не собственная авторизация соединителя в claude.ai, поэтому повторная авторизация соединителя не разрешает это. В `/mcp` соединитель показывается как `connected · session token rejected` и его представление деталей читает:

```text theme={null}
claude.ai rejected the session token. Run /login, then reconnect.
```

**Что делать:**

* Запустите `/login` для входа снова
* Переподключите соединитель из `/mcp`, или запустите `/mcp reconnect <server>`. Переподключение перед повторным входом оставляет соединитель в том же состоянии. Опция **Reconnect** панели `/mcp` сообщает `your claude.ai session token was rejected`; введенная форма `/mcp reconnect <server>` сообщает об успешном переподключении, даже если токен все еще отклонен.

До v2.1.222 Claude Code отметил соединитель как нуждающийся в аутентификации вместо этого, что указывал вас на поток авторизации соединителя, даже если завершение его не разрешало состояние.

<h3 id="mcp-server-needs-you-to-sign-in-again">
  MCP server needs you to sign in again
</h3>

Удаленный [MCP server](/docs/ru/mcp) отклонил учетные данные при вызове инструмента в середине сеанса, обычно потому, что вход или токен истек или потому, что токен не имеет разрешения, которое требует инструмент. Вызов инструмента завершается ошибкой, и `/mcp` отмечает сервер как [needing authentication](/docs/ru/mcp#authenticate-with-remote-mcp-servers).

Для сервера, на который вы входите из Claude Code, включая соединитель claude.ai, вход истек или был отозван:

```text theme={null}
MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)
```

Запустите `/mcp`, выберите сервер и войдите снова из его меню.

Для сервера, настроенного с помощью скрипта [`headersHelper`](/docs/ru/mcp#use-dynamic-headers-for-custom-authentication), Claude Code уже повторно запустил помощника и повторил вызов один раз перед отображением этого:

```text theme={null}
MCP server "<name>" rejected the credential from its headersHelper (check the helper and run /mcp to reconnect, or to authenticate if the server also uses OAuth)
```

Проверьте, что помощник возвращает учетные данные, которые сервер принимает, затем переподключитесь из `/mcp`, который запускает помощника снова.

Для сервера со статическим заголовком `Authorization` в его конфигурации:

```text theme={null}
MCP server "<name>" rejected the Authorization header in its config (update it, then run /mcp to reconnect)
```

Обновите значение заголовка там, где сервер настроен, затем переподключитесь из `/mcp`.

До v2.1.273 все три случая показали `MCP server "<name>" requires re-authorization (token expired)`.

Сервер также может отказать вызову инструмента с HTTP 403 `insufficient_scope`, чтобы попросить вас авторизовать область, иногда ту, которую ваш токен уже указывает. Сообщение называет эту область:

```text theme={null}
MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate
```

Запустите `/mcp`, выберите сервер и аутентифицируйтесь снова из его меню.

Когда конфигурация сервера не устанавливает ни [`oauth.scopes`](/docs/ru/mcp#restrict-oauth-scopes), ни [`authServerMetadataUrl`](/docs/ru/mcp#override-oauth-metadata-discovery), Claude Code запрашивает область, которую назвал сервер. С любым параметром Claude Code запрашивает области этого параметра вместо этого. Если вы закрепили `oauth.scopes`, добавьте отсутствующую область в этот список перед повторной аутентификацией.

До v2.1.274 этот случай показал сообщение `needs you to sign in again`, и до v2.1.273 он показал `requires re-authorization (token expired)` как другие случаи.

<h3 id="issuer-mismatch-in-authorization-response">
  Issuer mismatch in authorization response
</h3>

Во время [MCP OAuth sign-in](/docs/ru/mcp#authenticate-with-remote-mcp-servers) сервер авторизации перенаправил обратно в Claude Code с параметром `iss`, который не называет издателя, которого Claude Code ожидал от метаданных OAuth сервера. Неправильный издатель на этом шаге — это то, как выглядит атака смешивания сервера авторизации, поэтому Claude Code завершает вход ошибкой вместо обмена кодом авторизации. Claude Code показывает ошибку в меню сервера `/mcp` после входа в браузер:

```text theme={null}
Issuer mismatch in authorization response (RFC 9207): expected "https://auth.example.com", received "https://other.example.com"
```

`expected` — это издатель из метаданных OAuth сервера, и `received` — это значение `iss`, которое перенаправление несло. Вход, чье перенаправление не содержит параметр `iss`, проходит проверку, если только метаданные сервера не установили `authorization_response_iss_parameter_supported`, в этом случае Claude Code завершает вход ошибкой.

**Что делать:**

* Попробуйте вход снова из `/mcp`
* Если ошибка повторяется, сообщите об этом оператору сервера. Исправление находится на стороне сервера: сервер авторизации должен вернуть того же издателя в параметре `iss`, который он объявляет в своих метаданных
* Для подключения, пока сервер исправляется, запустите Claude Code с [`MCP_SDK_GENERATION=v1`](/docs/ru/env-vars), чей [runtime](/docs/ru/mcp#mcp-client-runtimes) не запускает эту проверку. Это удаляет защиту от атак смешивания, поэтому предпочитайте исправление на стороне сервера

До v2.1.232 Claude Code использовал только v2 runtime в постепенном развертывании или когда вы установили `MCP_SDK_GENERATION=v2`.

<h3 id="aws-credentials-expired-or-invalid">
  AWS credentials expired or invalid
</h3>

Ваш токен сеанса AWS истек или был отклонен. Это сообщение появляется на 401 из [Claude Platform on AWS](/docs/ru/claude-platform-on-aws) или [Mantle endpoint](/docs/ru/amazon-bedrock#use-the-mantle-endpoint), что является тем, как эти поставщики сообщают об истекшем токене безопасности.

Подсказка действия в середине варьируется в зависимости от вашей установки. Стабильная часть — это ведущая `AWS credentials expired or invalid`:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

До v2.1.273 это сообщение появлялось только, когда `awsAuthRefresh` был настроен.

**Что делать:**

* Если подсказка говорит, что учетные данные управляются этим окружением, приложение, которое запустило Claude Code, владеет учетным данным и другие шаги здесь не применяются: повторите попытку, или свяжитесь с администратором
* Если [`awsAuthRefresh`](/docs/ru/amazon-bedrock#advanced-credential-configuration) установлен, запустите команду, названную в сообщении, такую как `aws sso login --profile myprofile`, в другом терминале и завершите вход в браузер, затем повторите попытку. В противном случае обновите учетные данные AWS, которые вы используете сами: ваш вход SSO, ключи доступа, ключ API или токен прокси
* С `awsAuthRefresh`, установленным в интерактивном сеансе, вы можете вместо этого запустить `/login`, выбрать **3rd-party platform**, затем выбрать **Claude Platform on AWS · refresh credentials** под **Using 3rd-party platforms**, чтобы запустить ту же команду без перезапуска Claude Code. См. [Configure AWS credentials](/docs/ru/claude-platform-on-aws#1-configure-aws-credentials)
* Если ошибка повторяется после успешного выполнения команды обновления, подтвердите, что идентификатор действителен вне Claude Code с помощью `aws sts get-caller-identity` в той же оболочке и профиле

<h3 id="aws-authentication-failed">
  AWS authentication failed
</h3>

Ваш поставщик AWS вернул 403, или [Amazon Bedrock](/docs/ru/amazon-bedrock) вернул 401.

Amazon Bedrock сообщает об истекшем токене безопасности как 403, но 403 также является тем, как он сообщает об отказе в авторизации, такой как `AccessDeniedException` из отсутствующего разрешения IAM. Claude Code не может различить эти две причины.

401 из Amazon Bedrock также приземляется здесь, а не под [AWS credentials expired or invalid](#aws-credentials-expired-or-invalid), потому что Amazon Bedrock не сообщает об истекшем токене как 401. 401 из этой конечной точки обычно поступает из чего-то еще в пути запроса, такого как корпоративный прокси.

Обновление учетных данных исправляет истекший токен и не может исправить другие причины, поэтому сообщение предлагает оба:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

Подсказка действия в середине варьируется в зависимости от вашей установки. Стабильная часть — это ведущая `AWS authentication failed`.

Когда 403 — это ответ Amazon Bedrock, что у вас нет доступа к модели с указанным ID модели, подсказка вместо этого говорит вам включить модель для вашей учетной записи и региона в консоли Amazon Bedrock.

До v2.1.273 это сообщение появлялось только, когда `awsAuthRefresh` был настроен.

**Что делать:**

* Если подсказка говорит, что учетные данные управляются этим окружением, приложение, которое запустило Claude Code, владеет учетным данным и другие шаги здесь не применяются: повторите попытку, или свяжитесь с администратором
* Обновите ваши учетные данные AWS на случай, если истекшее учетное данные является причиной: запустите команду [`awsAuthRefresh`](/docs/ru/amazon-bedrock#advanced-credential-configuration), названную в сообщении, когда она установлена, или обновите ваш вход SSO, ключи доступа, ключ API или токен прокси сами
* Если ваши учетные данные текущие, подтвердите разрешения IAM в [IAM configuration](/docs/ru/amazon-bedrock#iam-configuration), прикреплены к идентификатору, который вы используете, и что выбранная модель включена для вашей учетной записи и региона
* Запустите `aws sts get-caller-identity`, чтобы подтвердить, какой идентификатор используют ваши запросы; устаревший `AWS_PROFILE` или профиль по умолчанию — это частая причина несоответствия разрешений

<h3 id="google-cloud-credentials-expired-or-invalid">
  Google Cloud credentials expired or invalid
</h3>

Ваши учетные данные Google Cloud для [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) истекли или были отклонены: запрос вернул 401, что является тем, как Agent Platform сообщает об истечении учетных данных.

Подсказка действия в середине варьируется в зависимости от вашей установки. Стабильная часть — это ведущая `Google Cloud credentials expired or invalid`:

```text theme={null}
Google Cloud credentials expired or invalid · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · API Error: 401 ...
```

**Что делать:**

* Если подсказка говорит, что учетные данные управляются этим окружением, приложение, которое запустило Claude Code, владеет учетным данным и другие шаги здесь не применяются: повторите попытку, или свяжитесь с администратором
* Если вы аутентифицируетесь с учетными данными приложения по умолчанию, запустите команду [`gcpAuthRefresh`](/docs/ru/google-vertex-ai#advanced-credential-configuration), названную в сообщении, или `gcloud auth application-default login`, и завершите вход, затем повторите попытку
* Если вы маршрутизируете через [LLM gateway](/docs/ru/llm-gateway) с установленным `CLAUDE_CODE_SKIP_VERTEX_AUTH`, обновите токен шлюза в `ANTHROPIC_AUTH_TOKEN` или `ANTHROPIC_CUSTOM_HEADERS`, затем повторите попытку
* Если вы аутентифицируетесь с файлом ключа учетной записи обслуживания, подтвердите, что `GOOGLE_APPLICATION_CREDENTIALS` указывает на действительный ключ. См. [Configure GCP credentials](/docs/ru/google-vertex-ai#3-configure-gcp-credentials)
* Если ошибка повторяется после обновления, подтвердите, что идентификатор работает вне Claude Code с помощью `gcloud auth application-default print-access-token` в той же оболочке

До v2.1.273 401 из Agent Platform показал общее сообщение `Please run /login` или `Failed to authenticate` вместо этого, которое не может обновить учетные данные Google Cloud.

<h3 id="google-cloud-authentication-failed">
  Google Cloud authentication failed
</h3>

[Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) вернул 403, который он использует для отказов в авторизации, а не для истекших учетных данных. Обычно идентификатор, с которым вы аутентифицируетесь, отсутствует разрешение IAM, или модель не включена для вашего проекта.

Подсказка действия в середине варьируется в зависимости от вашей установки. Стабильная часть — это ведущая `Google Cloud authentication failed`:

```text theme={null}
Google Cloud authentication failed · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · if credentials are current, check GCP IAM permissions and Vertex AI model access · API Error: 403 ...
```

**Что делать:**

* Если подсказка говорит, что учетные данные управляются этим окружением, приложение, которое запустило Claude Code, владеет учетным данным и другие шаги здесь не применяются: повторите попытку, или свяжитесь с администратором
* Подтвердите роли в [IAM configuration](/docs/ru/google-vertex-ai#iam-configuration), предоставлены идентификатору, с которым вы аутентифицируетесь
* Подтвердите, что модель включена для вашего проекта. См. [Request model access](/docs/ru/google-vertex-ai#2-request-model-access)

До v2.1.273 403 из Agent Platform показал общее сообщение `Please run /login` или `Failed to authenticate` вместо этого, которое не может обновить учетные данные Google Cloud.

<h3 id="microsoft-foundry-authentication-failed">
  Microsoft Foundry authentication failed
</h3>

[Microsoft Foundry](/docs/ru/microsoft-foundry) вернул 401 или 403: учетные данные Azure в запросе были отклонены, или идентификатор позади них не имеет доступа к ресурсу Foundry. `/login` не может создавать учетные данные Azure. Подсказка действия в середине варьируется в зависимости от вашей установки. Стабильная часть — это ведущая `Microsoft Foundry authentication failed`:

```text theme={null}
Microsoft Foundry authentication failed · refresh your Foundry credential (ANTHROPIC_FOUNDRY_AUTH_TOKEN, ANTHROPIC_FOUNDRY_API_KEY, Azure sign-in for Entra, or your proxy token) and retry · if credentials are current, check access to the Foundry resource · API Error: 401 ...
```

**Что делать:**

* Если подсказка говорит, что учетные данные управляются этим окружением, приложение, которое запустило Claude Code, владеет учетным данным и другие шаги здесь не применяются: повторите попытку, или свяжитесь с администратором
* Обновите учетные данные, которые вы настроили в [Configure Azure credentials](/docs/ru/microsoft-foundry#2-configure-azure-credentials): поверните `ANTHROPIC_FOUNDRY_API_KEY`, создайте свежий `ANTHROPIC_FOUNDRY_AUTH_TOKEN`, или запустите `az login`, чтобы цепь учетных данных Microsoft Entra по умолчанию могла войти снова
* Если учетные данные текущие, подтвердите, что идентификатор имеет доступ к ресурсу Foundry. См. [Azure RBAC configuration](/docs/ru/microsoft-foundry#azure-rbac-configuration)

До v2.1.273 401 или 403 из Microsoft Foundry показал общее сообщение `Please run /login` или `Failed to authenticate` вместо этого, которое не может обновить учетные данные Azure.

<h3 id="could-not-load-aws-or-google-cloud-credentials">
  Could not load AWS or Google Cloud credentials
</h3>

Claude Code не мог получить используемые учетные данные из цепи поставщика учетных данных AWS или из ваших учетных данных приложения Google по умолчанию на машине, на которой он работает, поэтому ни один запрос не достиг вашего облачного поставщика. Claude Code очищает свои кэшированные учетные данные и повторяет попытку дважды перед отображением этого сообщения. Деталь после `·` называет конкретную причину, такую как истекший сеанс SSO, отсутствующие учетные данные приложения по умолчанию, сообщаемые как `Could not load the default credentials`, или отозванный вход, сообщаемый как `invalid_grant`:

```text theme={null}
API Error: Could not load AWS credentials · Could not load credentials from any providers. Check or refresh your AWS credentials and try again.
API Error: Could not load Google Cloud credentials · invalid_grant. Check or refresh your Google Cloud credentials and try again.
```

В [non-interactive mode](/docs/ru/headless) с `-p` и в [Agent SDK](/docs/ru/agent-sdk/overview) код структурированной ошибки — `cloud_credential_error`. До v2.1.267 сообщение показало только текст деталей после `API Error:`, и структурированный код был `server_error` или `unknown`.

**Что делать:**

* Запустите команду входа вашего поставщика, такую как `aws sso login --profile myprofile` или `gcloud auth application-default login`, затем повторите попытку. [Bedrock, Agent Platform, or Foundry credentials not loading](/docs/ru/troubleshoot-install#bedrock-agent-platform-or-foundry-credentials-not-loading) показывает, как подтвердить учетные данные вне Claude Code
* Если деталь читает `AWS default-chain credential resolve timed out`, цепь зависла, а не завершилась ошибкой, поэтому следуйте [AWS default-chain credential resolve timed out](#aws-default-chain-credential-resolve-timed-out) вместо этого

<h3 id="aws-default-chain-credential-resolve-timed-out">
  AWS default-chain credential resolve timed out
</h3>

Цепь поставщика учетных данных AWS по умолчанию не произвела учетные данные в течение 60 секунд, поэтому Claude Code остановил разрешение и завершил запрос ошибкой. Это время ожидания — одна из причин [Could not load AWS or Google Cloud credentials](#could-not-load-aws-or-google-cloud-credentials). Сбой — это локальное разрешение учетных данных: запрос никогда не достиг [Amazon Bedrock](/docs/ru/amazon-bedrock), [Claude Platform on AWS](/docs/ru/claude-platform-on-aws) или [Mantle endpoint](/docs/ru/amazon-bedrock#use-the-mantle-endpoint). Claude Code очищает свой [credential cache](/docs/ru/amazon-bedrock#credential-caching-and-resolution-timeout) и повторяет попытку перед тем, как эта ошибка проявляется, поэтому к тому времени, когда вы ее видите, цепь зависла при повторных попытках.

```text theme={null}
API Error: Could not load AWS credentials · AWS default-chain credential resolve timed out. Check or refresh your AWS credentials and try again.
```

Частые причины — команда `credential_process` в вашем профиле AWS, которая ждет входа, который она не может получить, и контейнер или VM, чей сервис метаданных экземпляра (IMDS) никогда не отвечает на зонд цепи.

До v2.1.267 сообщение читалось `API Error: AWS default-chain credential resolve timed out`.
До v2.1.207 зависшая цепь оставляла запрос ожидающим неопределенно долго вместо того, чтобы завершиться ошибкой.

**Что делать:**

* Запустите `aws sts get-caller-identity` в той же оболочке с тем же `AWS_PROFILE`. Если он также зависает, исправьте профиль; команда `credential_process`, которая запрашивает интерактивно, — это частая причина.
* Завершите шаг входа перед запуском Claude Code, например `aws sso login --profile myprofile`, поэтому цепь разрешается из локального кэша SSO вместо ожидания потока браузера
* Если ваша цепь запускает интерактивный вход, который законно нуждается в более чем 60 секундах, такой как SSO с MFA через оболочку, такую как `aws-vault`, поднимите лимит в миллисекундах с помощью [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/ru/env-vars)

<h3 id="bedrock-setup-verification-timed-out-waiting-for-aws">
  Bedrock setup verification timed out waiting for AWS
</h3>

Вызов AWS во время [Bedrock setup wizard](/docs/ru/amazon-bedrock#sign-in-with-bedrock) проверки учетных данных, такой как поиск учетных данных или проверка идентификатора, не завершился в течение лимита 60 секунд. Мастер перестает ждать и завершает шаг проверки ошибкой:

```text theme={null}
Timed out after 60s waiting for AWS. Check your network and proxy settings; if a credential helper needs longer to prompt you, raise CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS.
```

Число отражает ваш лимит: 60 секунд по умолчанию, или значение, которое вы установили в [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/ru/env-vars).

Частые причины — сеть или прокси, которые зависают запросы к AWS, включая обновление токена SSO, и помощник учетных данных, все еще ожидающий входа, который вы не можете видеть. Поднимите лимит только, когда помощник законно нуждается в большем времени.

Один зависший запрос к AWS также может завершиться ошибкой на его собственном времени ожидания для каждого запроса, которое показывает более короткое сообщение на том же шаге:

```text theme={null}
A request to AWS timed out. Check your network and proxy settings, then try again.
```

Когда те же времена ожидания происходят на шаге закрепления модели, мастер отмечает модель как `unreachable` вместо отображения любого сообщения.

**Что делать:**

* Запустите `aws sts get-caller-identity` в той же оболочке. Если он также зависает, зависание находится вне Claude Code, в вашей сети, вашем прокси или помощнике учетных данных в вашем профиле AWS; исправьте это сначала.
* Завершите любой интерактивный вход перед открытием мастера, например `aws sso login --profile myprofile`
* Если помощник учетных данных в вашем профиле AWS законно нуждается в более чем 60 секундах для запроса, поднимите лимит в миллисекундах с помощью [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/ru/env-vars)

<h3 id="cloud-gateway-session-expired">
  Cloud gateway session expired
</h3>

Вы вошли через [Claude apps gateway](/docs/ru/claude-apps-gateway), и сеанс шлюза, сохраненный на этой машине, истек и не мог быть обновлен, или шлюз больше его не принимает, например, после [JWT secret is replaced](/docs/ru/claude-apps-gateway-deploy#jwt-secret-rotation) шлюза. Если вы видите эту строку при запуске `claude` интерактивно, сеанс открылся без входа в шлюз:

```text theme={null}
Cloud gateway session expired — run /login to reconnect.
```

Та же строка может появиться в середине сеанса, когда учетные данные шлюза истекают и Claude Code не может их обновить.

В [non-interactive](/docs/ru/headless) запуске, фоновом или другом автоматическом сеансе, или подкоманде `claude`, отличной от `claude auth`, Claude Code выходит с этим сообщением вместо этого, когда шлюз больше не принимает сеанс:

```text theme={null}
Cloud gateway <url> no longer accepts this session. Start `claude` and sign in again with /login.
```

**Что делать:**

* Запустите `/login` в сеансе и завершите вход в браузер
* Для неинтерактивного запуска запустите `claude` в том же окружении, запустите `/login`, затем перезапустите вашу команду

<h3 id="sign-in-timed-out-while-waiting-for-you-to-continue">
  Sign-in timed out while waiting for you to continue
</h3>

Во время [Claude apps gateway](/docs/ru/claude-apps-gateway) входа шлюз назвал учетную запись, которая вошла, и Claude Code попросил вас подтвердить ее перед сохранением учетных данных. Вы оставили подтверждение открытым после истечения входа, и шлюз не выдал токен обновления, который мог бы его обновить, поэтому Claude Code ничего не сохранил, когда вы продолжили:

```text theme={null}
Sign-in timed out while waiting for you to continue. Try again.
```

**Что делать:**

* Запустите `/login` снова и подтвердите учетную запись перед истечением входа

<h3 id="gateway-refused-the-request">
  Gateway refused the request
</h3>

Вы подписаны через [Claude apps gateway](/docs/ru/claude-apps-gateway), и запрос вернул 403: шлюз, или вышестоящий позади него, отказал в нем. Повторный вход не изменяет отказ, поэтому сообщение указывает на администратора вашего шлюза:

```text theme={null}
Gateway refused the request · signing in again won't change this — check with your gateway administrator · API Error: 403 ...
```

**Что делать:**

* Попросите администратора вашего шлюза посмотреть запрос. Хвост `API Error:` содержит отказ, который вернул шлюз
* Для администраторов: [access control rule](/docs/ru/claude-apps-gateway-config#http-tuning) на шлюзе возвращает 403, который [audit log](/docs/ru/claude-apps-gateway-deploy#logs) записывает с его причиной, и отказ авторизации вышестоящего проходит через [Upstream error messages](/docs/ru/claude-apps-gateway-config#upstream-error-messages)

До v2.1.273 403 на сеансе шлюза показал общее сообщение `Please run /login` или `Failed to authenticate` вместо этого, и повторный вход не очистил отказ.

<h2 id="network-and-connection-errors">
  Ошибки сети и подключения
</h2>

Большинство этих ошибок означают, что сетевой запрос из Claude Code не достиг пункта назначения, или что-то между Claude Code и API изменило ответ на обратном пути; если запись также содержит локальную причину, такую как ошибка записи архива, это указано в её описании. Обычно они возникают в вашей локальной сети, прокси или брандмауэре, либо в политике сети облачной среды.

<h3 id="unable-to-connect-to-api">
  Unable to connect to API
</h3>

TCP-соединение с API не удалось или никогда не завершилось. Для распространённых кодов ошибок подключения сообщение указывает тип сбоя и сохраняет код в скобках:

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

Код, который Claude Code не распознаёт, отображается как `Unable to connect to API` с кодом в скобках. Некоторые из этих сообщений могут показывать более одного кода: `Connection refused` может показать `ConnectionRefused` или `ECONNREFUSED`, например, а `Can't reach the API server` может показать `ENOTFOUND` или `FailedToOpenSocket`.

До версии v2.1.227 каждое из этих кодированных сообщений читалось как `Unable to connect to API` с кодом, например `Unable to connect to API (ECONNREFUSED)`.

Распространённые причины включают отсутствие доступа в интернет, VPN, который блокирует `api.anthropic.com`, или требуемый корпоративный прокси, который не настроен.

**Что делать:**

* Подтвердите, что вы можете достичь хоста API из той же оболочки, запустив `curl -I https://api.anthropic.com`. В Windows PowerShell используйте `curl.exe -I https://api.anthropic.com`, чтобы встроенный псевдоним `Invoke-WebRequest` не использовался.
* Если вы находитесь за корпоративным прокси, установите `HTTPS_PROXY` перед запуском Claude Code и см. [Network configuration](/docs/ru/network-config)
* Если вы маршрутизируете через шлюз LLM или ретранслятор, установите [`ANTHROPIC_BASE_URL`](/docs/ru/env-vars) на его адрес. См. [Connect Claude Code to an LLM gateway](/docs/ru/llm-gateway-connect) для настройки.
* Убедитесь, что ваш брандмауэр разрешает хосты, указанные в [Network access requirements](/docs/ru/network-config#network-access-requirements)
* Прерывистые сбои [повторяются автоматически](#automatic-retries); постоянные сбои указывают на локальную проблему сети

Если `curl` успешен, но Claude Code всё ещё не работает, причина обычно находится между средой выполнения и сетью, а не в самой сети:

* На Linux и WSL проверьте `/etc/resolv.conf` на наличие недостижимого сервера имён. WSL в частности может унаследовать неработающий распознаватель от хоста.
* На macOS клиент VPN, который был отключен или удален, может оставить интерфейс туннеля или правило маршрутизации. Проверьте `ifconfig` на наличие устаревших интерфейсов `utun` и удалите сетевое расширение VPN в System Settings.
* Docker Desktop и аналогичные среды выполнения контейнеров могут перехватывать исходящий трафик. Закройте их и повторите попытку, чтобы исключить это.

<h3 id="unable-to-connect-to-anthropic-services">
  Unable to connect to Anthropic services
</h3>

Во время первоначальной настройки Claude Code проверяет, что он может достичь `api.anthropic.com` и `platform.claude.com` перед отображением шага входа. Когда одна из проверок не удаётся, Claude Code выводит причину и выходит.

```text theme={null}
Unable to connect to Anthropic services
Failed to connect to api.anthropic.com: ECONNREFUSED
Connection to api.anthropic.com timed out after 10 seconds
A proxy is configured via HTTPS_PROXY. Check that it allows connections to the host above.
```

Claude Code отправляет проверку через ту же [proxy configuration](/docs/ru/network-config), что и запросы API, и даёт каждому зонду 10 секунд. Когда неудачный зонд прошёл через прокси, сообщение указывает переменную окружения, которая его настроила, такую как `HTTPS_PROXY`. До версии v2.1.222 проверка использовала другой транспорт прокси без тайм-аута: за прокси URL со схемой `https://` она могла зависнуть на `Checking connectivity...` неопределённо долго, а затем не удаться, даже если запросы API через тот же прокси успешны.

Claude Code пропускает эту проверку, когда [managed settings file, MDM policy, или policy helper](/docs/ru/managed-settings) устанавливает [`forceLoginMethod`](/docs/ru/settings-reference#forceloginmethod) на `"gateway"`, или устанавливает [`forceLoginGatewayUrl`](/docs/ru/settings-reference#forcelogingatewayurl) без `forceLoginMethod`. С любой из этих конфигураций Claude Code открывает шаг входа на экране **Cloud gateway** вместо метода входа Anthropic. Claude Code также пропускает проверку, когда источник управляемых параметров на машине существует, но не может быть прочитан, поскольку этот источник может содержать конфигурацию шлюза. До версии v2.1.247 Claude Code запускал проверку и при этой конфигурации и выходил с этой ошибкой, когда конечные точки Anthropic были недостижимы.

**Что делать:**

* Если сообщение указывает переменную прокси, проверьте, что её значение указывает на правильный прокси, и попросите вашу команду сети разрешить HTTPS-соединения через него к хосту в сообщении. См. [Network configuration](/docs/ru/network-config).
* Пройдите проверки в [Unable to connect to API](#unable-to-connect-to-api). Тест `curl` и рекомендации по брандмауэру там применяются и к этой проверке.
* Если ваша организация входит через [cloud gateway](/docs/ru/claude-apps-gateway) и эта ошибка появляется при первом запуске, обновитесь до Claude Code v2.1.247 или позже.
* Если ваша сеть открыта и сбой сохраняется, Claude Code может быть [недоступен в вашей стране](https://www.anthropic.com/supported-countries)

<h3 id="socket-is-closed">
  Socket is closed
</h3>

`Socket is closed` означает, что соединение, несущее потоковый ответ, было закрыто, пока ответ всё ещё поступал. Наиболее распространённая причина — корпоративный прокси на Windows, отбрасывающий установленный туннель в середине ответа.

В зависимости от того, насколько далеко продвинулся ответ, Claude Code повторяет запрос, сохраняет то, что произвёл Claude, или завершает ход. См. [Automatic retries](#automatic-retries).

До версии v2.1.214 Claude Code не повторял этот сбой, и ход останавливался с ошибкой, содержащей `Socket is closed`.

**Что делать:**

* Если вы видите эту ошибку, обновитесь до v2.1.214 или позже с помощью `claude update`, затем отправьте ваше сообщение снова
* Если ходы продолжают не удаваться за тем же прокси после обновления, пройдите [Unable to connect to API](#unable-to-connect-to-api) и проверьте настройку прокси в [Network configuration](/docs/ru/network-config)

<h3 id="api-returned-an-empty-or-malformed-response">
  API returned an empty or malformed response
</h3>

Claude Code показывает эту ошибку, когда его повторная попытка без потока неудачного потокового запроса получает статус HTTP успеха, но тело не является сообщением Claude API: обычно это HTML-ошибка или страница входа, пустое тело или JSON в другом формате. Прокси, шлюз или страница входа в сеть, отвечающие вместо API, — обычный источник. Claude Code не повторяет запрос, и ход заканчивается этой ошибкой.

```text theme={null}
API returned an empty or malformed response (HTTP 200) — check for a proxy or gateway intercepting the request.
```

После этого открытия сообщение сообщает, что вернулось и какой запрос не удался:

* Предложение `Response:` с типом содержимого, типом тела, таким как `body is an HTML page` или `empty body`, его размером в байтах и наличием ли в ответе идентификатора запроса Anthropic. Когда ответ указывает на узнаваемый сервер, такой как `nginx` или `cloudflare`, или содержит заголовки промежуточного звена, такие как `cf-ray` или `via`, предложение также их перечисляет.
* Предложение, указывающее идентификатор неудачного потокового запроса и сбой, который вызвал повторную попытку. Когда поток открылся до сбоя, оно также сообщает, сколько событий потока прибыло и, если какие-то прибыли, как долго поток молчал, когда попытка не удалась.

До версии v2.1.234 сообщение заканчивалось после `intercepting the request`.

До версии v2.1.271 ответ, который нёс действительное сообщение API под типом содержимого, отличным от JSON, таким как `text/plain`, также заканчивал ход этой ошибкой. Некоторые шлюзы LLM используют этот тип содержимого для ответа без потока.

**Что делать:**

* Прочитайте предложение `Response:`, чтобы увидеть, какая система ответила. HTML-тело, отсутствие идентификатора запроса Anthropic или названный сервер, такой как `nginx` или `cloudflare`, означает, что что-то между Claude Code и API ответило вместо него
* Если вы маршрутизируете через [LLM gateway](/docs/ru/llm-gateway-connect#troubleshoot-gateway-errors), протестируйте маршрут прямым запросом и исправьте переход, который возвращает ответ, отличный от API
* В сети со страницей входа, такой как гостевой Wi-Fi, завершите вход в браузере, затем повторите попытку
* Если только маршрут без потока через ваш шлюз сломан, установите [`CLAUDE_CODE_DISABLE_NONSTREAMING_FALLBACK=1`](/docs/ru/env-vars#variables), чтобы запрос, который не удаётся в середине потока, перешёл на обычный путь повторной попытки вместо этого резервного варианта, кроме случаев, когда сама конечная точка потока возвращает `404`, где Claude Code всё ещё переходит на резервный вариант

<h3 id="streaming-response-ended-before-any-complete-data-was-received">
  Streaming response ended before any complete data was received
</h3>

Потоковый ответ от вашего поставщика модели завершился без доставки каких-либо полезных данных, поэтому Claude Code повторно отправил запрос без потока, чтобы завершить ход. Claude Code показывает предупреждение один раз за сеанс, только в интерактивных сеансах. До версии v2.1.239 Claude Code молча повторял попытку без потока.

```text theme={null}
Streaming response ended before any complete data was received. Retrying without streaming. If this keeps happening, check any proxy or gateway between Claude Code and your model provider.
```

Claude Code отправляет каждый затронутый запрос дважды: пустую попытку потока и повторную попытку. Обычная причина — прокси или шлюз, который потребляет или преобразует тело потокового ответа на обратном пути.

**Что делать:**

* Настройте любой прокси или шлюз между Claude Code и вашим поставщиком модели, чтобы пропускать тела потоковых ответов и их заголовки без изменений
* На [Amazon Bedrock](/docs/ru/amazon-bedrock) см. [Streaming errors behind a gateway or proxy](/docs/ru/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy) для требований к заголовкам и телу

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock streaming response has an unexpected content-type
</h3>

Шлюз или прокси между Claude Code и [Amazon Bedrock](/docs/ru/amazon-bedrock) преобразует тело потокового ответа или его заголовок `Content-Type`. Amazon Bedrock потоком передаёт ответы как `application/vnd.amazon.eventstream`. Вместо декодирования тела, которое он не может прочитать, Claude Code отклоняет успешный потоковый ответ, который сообщает другой тип содержимого. Claude Code не повторяет запрос.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

До версии v2.1.208 та же неправильная конфигурация проявлялась как `API Error: Truncated event message received` после того, как весь ответ был буферизирован.

**Что делать:**

* Настройте шлюз на пропуск тела ответа `InvokeModelWithResponseStream` и его заголовка `Content-Type` без изменений. Промежуточное звено, которое повторно излучает поток как события, отправляемые сервером, — распространённая причина.
* Установка [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/ru/env-vars) скрывает эту ошибку, но Claude Code не декодирует двоичное тело под переписанным заголовком, поэтому эти запросы переходят на более медленный путь без потока. См. [Streaming errors behind a gateway or proxy](/docs/ru/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  SSL certificate errors
</h3>

Прокси или устройство безопасности в вашей сети перехватывает трафик TLS со своим собственным сертификатом, и Claude Code ему не доверяет.

```text theme={null}
Unable to connect to API: SSL certificate verification failed (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
Unable to connect to API: Self-signed certificate detected (SELF_SIGNED_CERT_IN_CHAIN). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
```

До версии v2.1.273 оба сообщения заканчивались на `Check your proxy or corporate SSL certificates`, без кода OpenSSL или подсказки `NODE_EXTRA_CA_CERTS`.

Начиная с версии v2.1.199, сбой проверки сертификата не повторяется, поэтому эта ошибка появляется при первой попытке вместо того, чтобы появиться после полного [retry budget](#automatic-retries). Более ранние версии потратили несколько минут на повторные попытки перед её отображением. Переходящие условия TLS, такие как тайм-аут рукопожатия, всё ещё повторяются.

Во время `/login` и проверки подключения при запуске тот же сбой производит другое сообщение:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

На [Amazon Bedrock](/docs/ru/amazon-bedrock) запросы, которые Claude Code сам отправляет в AWS, такие как вызовы учётных данных роли STS и SSO, обнаружение модели и проверки мастера настройки, зависят от той же конфигурации сертификата. См. [Certificate errors behind a TLS-inspecting proxy](/docs/ru/amazon-bedrock#certificate-errors-behind-a-tls-inspecting-proxy).

**Что делать:**

* Экспортируйте пакет CA вашей организации и укажите Claude Code на него с помощью `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* См. [Network configuration](/docs/ru/network-config#custom-ca-certificates) для полных инструкций по настройке
* Не устанавливайте `NODE_TLS_REJECT_UNAUTHORIZED=0`, что полностью отключает проверку сертификата

<h3 id="host-not-allowed-in-a-cloud-session">
  Host not allowed in a cloud session
</h3>

Исходящий HTTP-запрос из облачного сеанса или подпрограммы был заблокирован политикой сети среды.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Вы также можете увидеть сертификат TLS, который не соответствует реальному сертификату пункта назначения. Облачные сеансы маршрутизируют исходящий трафик через прокси, который применяет политику сети, поэтому несоответствующий сертификат означает, что прокси завершил соединение, а не пункт назначения.

Это не проблема сети на стороне клиента. Облачные сеансы и [routines](/docs/ru/routines) работают внутри изолированной виртуальной машины, исходящий трафик которой через сеанс отфильтрован в [cloud environment's](/docs/ru/cloud-environments) список разрешений; [GitHub operations](/docs/ru/cloud-environments#github-proxy) и трафик соединителя MCP используют отдельные каналы, поэтому они могут продолжать работать, пока другие хосты заблокированы. Среда **Default** использует доступ **Trusted**, который разрешает [default allowlist](/docs/ru/cloud-environments#default-allowed-domains) реестров пакетов, API поставщиков облака, реестров контейнеров и распространённых доменов разработки и блокирует другие домены на этом пути.

**Что делать:**

Эти шаги изменяют одну из ваших собственных сред. [Организационная общая среда](/docs/ru/cloud-environments#organization-shared-environments) открывается только для чтения в селекторе, поэтому попросите владельца изменить её сетевой доступ со страницы **Cloud environments** в [admin settings](https://claude.ai/admin-settings).

* Откройте подпрограмму для редактирования или запустите облачный сеанс. Выберите значок облака, показывающий имя вашей среды, такое как **Default**, чтобы открыть селектор. Наведите указатель на вашу среду и нажмите значок параметров.
* В диалоговом окне **Update cloud environment** измените **Network access** с **Trusted** на **Custom**, затем добавьте заблокированный домен в **Allowed domains**. Введите один домен в строку. Установите флажок **Also include default list of common package managers**, чтобы сохранить [default allowlist](/docs/ru/cloud-environments#default-allowed-domains) вместе с вашими пользовательскими доменами. Выберите **Full** вместо этого, если вы хотите неограниченный доступ.
* Нажмите **Save changes**. Следующий запуск использует обновленный список разрешений.

См. [Network access](/docs/ru/cloud-environments#network-access) для уровней доступа и списка разрешений по умолчанию. Локальные сеансы CLI не затронуты этой политикой.

<h3 id="the-proxy-refused-the-connection">
  The proxy refused the connection
</h3>

Вы видите это сообщение, когда Claude читает [artifact](/docs/ru/artifacts) через прокси, который вы установили в `HTTPS_PROXY` или связанную [proxy variable](/docs/ru/network-config#environment-variables). Содержимое артефакта поступает из `*.frame.claudeusercontent.com`, поэтому Claude Code сначала отправляет прокси запрос `CONNECT`, прося его открыть туннель к этому хосту. Когда прокси отказывает, ничего не достигает хоста, и сообщение содержит статус HTTP прокси:

```text theme={null}
artifact content fetch failed (proxy refused the connection: HTTP 407)
artifact content fetch failed (proxy refused the connection: HTTP 403)
the proxy refused the connection to the artifact's content host (HTTP 502)
```

Статус — это ответ прокси на `CONNECT`. Хост никогда не ответил, поэтому каждый статус указывает на другое исправление:

* `HTTP 407`: прокси требует учётные данные, которые он не получил. Поместите их в URL прокси, как показано в [Basic authentication](/docs/ru/network-config#basic-authentication).
* `HTTP 403`: прокси отказывается туннелировать к `*.frame.claudeusercontent.com`. Попросите того, кто управляет прокси, разрешить этот хост, который [Network access requirements](/docs/ru/network-config#network-access-requirements) перечисляет.
* Любой другой статус, такой как `HTTP 502`: прокси не открыл туннель по своей причине, такой как неудача достижения хоста. Посмотрите статус в журналах прокси.
* `unreadable reply` вместо статуса: то, что находится по адресу прокси, не ответило строкой статуса HTTP. Проверьте, что адрес — это HTTP-прокси.

**Что делать:**

* Проверьте адрес и учётные данные в переменной прокси, как описано в [Proxy configuration](/docs/ru/network-config#proxy-configuration), затем запустите `curl -x http://proxy.example.com:8080 -I https://api.anthropic.com` из оболочки, в которой вы запускаете Claude Code, используя ваш собственный URL прокси. В Windows PowerShell запустите `curl.exe`. Если этот зонд не удаётся так же, сначала исправьте настройку прокси. Если он успешен, отказ специфичен для хоста артефакта.
* Если ваша сеть позволяет Claude Code достичь хоста артефакта напрямую, добавьте `.frame.claudeusercontent.com` в [`NO_PROXY`](/docs/ru/network-config#environment-variables). Сохраняйте запись узкой: более широкая запись `.claudeusercontent.com` также обходит прокси для `bridge.claudeusercontent.com`, который организациям с [IP allowlisting](/docs/ru/network-config#organization-ip-allowlists-and-proxy-egress) нужно сохранить на прокси.

До версии v2.1.238 Claude Code сообщал об отказанном туннеле как об общей ошибке сети.

<h3 id="the-cloud-environments-service-returned-an-empty-or-unexpected-response">
  The cloud environments service returned an empty or unexpected response
</h3>

Claude Code запрашивает ваш список [cloud environments](/docs/ru/cloud-environments) в нескольких местах, например, когда вы создаёте облачный сеанс из CLI или запускаете [`/remote-env`](/docs/ru/cloud-environments#select-an-environment-from-the-cli). Когда он не может прочитать ответ сервера, он показывает одно из этих сообщений:

```text theme={null}
The cloud environments service returned an empty response (HTTP 200 with no body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 with a non-JSON body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 without a usable environments list). This is usually temporary — try again in a moment.
```

Сервер принял запрос, но ответил телом, которое не является списком сред: пусто, не JSON или JSON без списка. Это обычно сопровождает сбой на стороне сервиса и очищается самостоятельно. В зависимости от поверхности, которая запросила список, Claude Code может добавить префикс, такой как `couldn't list environments:` в диалоговом окне `/remote-env`.

**Что делать:**

* Повторите действие. Claude Code запрашивает список снова каждый раз
* Если сообщение продолжает появляться, проверьте [status.claude.com](https://status.claude.com) на наличие активных инцидентов

До версии v2.1.236 Claude Code показывал необработанную ошибку JavaScript TypeError вместо этих сообщений.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Couldn't reconnect to your Remote Control session
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

Возобновление с помощью `claude --resume` или `claude --continue` переподключается к сеансу [Remote Control](/docs/ru/remote-control), записанному в этом разговоре. Это сообщение означает, что переподключение не удалось по причине, которая может быть временной, такой как сетевой сбой или ошибка сервера, поэтому Claude Code не может подтвердить, существует ли удалённый сеанс. Ваш локальный сеанс продолжает работать без Remote Control.

**Что делать:**

* Запустите `/remote-control` для повторной попытки подключения
* Запустите новый сеанс с помощью `claude --remote-control`, чтобы создать новый сеанс Remote Control
* Для других сообщений запуска Remote Control см. [Troubleshoot Remote Control](/docs/ru/remote-control#troubleshooting)

Если сервер вместо этого сообщает, что предыдущий сеанс исчез, вы не видите это сообщение. Claude Code запускает новый сеанс на его месте или показывает [`Previous session is unavailable — run /remote-control to start a new one`](/docs/ru/remote-control#previous-session-is-unavailable), в зависимости от [the conversation's reconnection record](/docs/ru/remote-control#resume-outcomes). Из версии v2.1.227 по v2.1.231 Claude Code показывал сообщение, которое начинается с `Remote Control could not resume the previous session under the current login` вместо этого, и [более ранние версии вели себя иначе](/docs/ru/remote-control#reconnect-history).

<h3 id="sessions-ended-while-this-machine-was-offline">
  Sessions ended while this machine was offline
</h3>

Claude Code показывает это сообщение в терминале, запускающем [`claude remote-control`](/docs/ru/remote-control#start-a-remote-control-session), после того как ваша машина была в автономном режиме достаточно долго, чтобы сервер очистил среду Remote Control, которую ваша машина обслуживала. Сеансы в этой среде закончились, и вы не можете их возобновить. Количество — это количество сеансов, которые закончились.

```text theme={null}
2 sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.
```

**Что делать:**

* Когда Claude Code перечисляет сохранённые worktrees под этим сообщением, подберите любую незафиксированную работу из них
* Запустите `claude remote-control` для запуска свежей среды

<h3 id="couldnt-share-the-transcript">
  Couldn't share the transcript
</h3>

После того как вы согласитесь поделиться стенограммой вашего сеанса из приглашения опроса, такого как [session quality survey](/docs/ru/data-usage#session-quality-surveys), Claude Code загружает её в Anthropic или сохраняет локальный архив вместо этого на сторонних поставщиков, на сеансах [Claude apps gateway](/docs/ru/claude-apps-gateway) и когда нет доступных учётных данных Anthropic. Это сообщение означает, что общий доступ не завершился.

```text theme={null}
Couldn't share the transcript.
```

Загрузка должна соответствовать лимиту 8 MiB. На длинном сеансе Claude Code прогрессивно отбрасывает части общего доступа, параметры модели последнего запроса в первую очередь, затем структурированный разговор и стенограммы подагента, и показывает это сообщение только когда никакая сокращённая версия не может быть отправлена или сетевая или ошибка сервера останавливает загрузку. Когда Claude Code сохраняет локальный архив вместо этого, сообщение означает, что он не мог записать архив.

**Что делать:**

* Запустите `/feedback` для отправки стенограммы с описанием того, что произошло. См. [Report an error](#report-an-error), если `/feedback` недоступен в вашей среде
* Если другие запросы также не удаются, проверьте ваше сетевое соединение и см. [Unable to connect to API](#unable-to-connect-to-api)

<h2 id="request-errors">
  Ошибки запроса
</h2>

Эти ошибки связаны с содержимым вашего запроса. Большинство из них возвращаются API после отклонения запроса; несколько производятся локально Claude Code перед отправкой запроса.

<h3 id="prompt-is-too-long">
  Prompt is too long
</h3>

Разговор плюс прикреплённые файлы превышают контекстное окно модели.

```text theme={null}
Prompt is too long
```

В интерактивном сеансе Claude Code показывает эту ошибку как:

```text theme={null}
Context limit reached · /compact или /clear для продолжения
```

Строка указывает только `/clear`, когда установлена [`DISABLE_COMPACT`](/docs/ru/env-vars). Более длинные формы ошибки, такие как форма сбоя компактирования ниже, сохраняют формулировку `Prompt is too long ·`. В выводе `-p` и в расшифровке текст остаётся `Prompt is too long`.

Когда вы отключили автоматическое компактирование в [пользовательских настройках](/docs/ru/settings-reference#autocompactenabled), строка также это указывает:

```text theme={null}
Context limit reached · /compact или /clear для продолжения · auto-compact отключён · /config для его включения
```

Переключатель **Auto-compact** в `/config` записывает `autoCompactEnabled` в пользовательские настройки. Подсказка появляется только когда изменение `/config` вступит в силу. Например, она не появляется, когда [`DISABLE_AUTO_COMPACT`](/docs/ru/env-vars) или [`DISABLE_COMPACT`](/docs/ru/env-vars) отключили автоматическое компактирование. Она также не появляется, когда область с более высоким приоритетом, такая как проект или управляемые настройки, установила `autoCompactEnabled` на `false`. До версии 2.1.235 строка не содержала подсказку об автоматическом компактировании.

Amazon Bedrock сообщает об этом состоянии как `Input is too long for requested model.`, что Claude Code обрабатывает так же. До версии 2.1.217 Claude Code не распознавал формулировку Bedrock, поэтому автоматическое компактирование никогда не срабатывало на ней и `/compact` завершался с той же ошибкой.

Шлюз [Claude apps gateway](/docs/ru/claude-apps-gateway-config#upstream-error-messages) сообщает об этом состоянии как `capability_rejected: prompt_too_long`, когда облачный вышестоящий сервер отклоняет запрос в собственной форме ошибки поставщика. Claude Code обрабатывает токен так же, как `Prompt is too long`. До версии 2.1.228 Claude Code не распознавал токен, поэтому автоматическое компактирование не срабатывало на нём.

Когда автоматическое компактирование выполнялось на этом ходу и завершилось с ошибкой на основной ошибке, такой как недоступная модель или ошибка аутентификации, сообщение называет эту ошибку после разделителя:

```text theme={null}
Prompt is too long · automatic compaction failed: <основная ошибка>
```

Сначала разрешите названную ошибку; `/compact` завершается с той же ошибкой, пока вы этого не сделаете. До версии 2.1.229 неудачное автоматическое компактирование выводило `Prompt is too long` без указания причины.

Когда автоматическое компактирование выполняется на этой ошибке, оно обычно суммирует ваши самые старые обмены и сохраняет самые новые. В качестве последней меры Claude Code суммирует иначе:

* Когда он не может суммировать какой-либо целый обмен, Claude Code сохраняет ваш самый новый prompt слово в слово и суммирует всё перед ним.
* В этом случае, когда разговор не заканчивается вашим prompt, Claude Code суммирует весь разговор вместо этого.

Claude Code пропускает это восстановление, когда содержимое, которое оно должно было бы перенести, не содержит ответа модели и менее примерно 1000 токенов вашего собственного текста, такого как короткий повтор, отправленный после чрезмерно большой вставки. Запустите `/clear` для начала заново. До версии 2.1.269 компактирование завершалось с ошибкой всякий раз, когда оно не могло суммировать целый обмен, поэтому сеанс в этом состоянии получал эту ошибку снова на каждом ходу.

Однооборотный разговор не имеет более ранних ходов для суммирования. Когда автоматическое компактирование должно было выполниться на одном, Claude Code пропускает попытку и объясняет, что заполняет запрос. Когда API не сообщает количество токенов в своей ошибке, сообщение читается:

```text theme={null}
Prompt is too long · this conversation is a single exchange and cannot be compacted — the request size comes mostly from system prompt, tool definitions, or attachments.
```

Когда API сообщает количество токенов в своей ошибке, Claude Code сравнивает их со своей оценкой размера разговора, чтобы определить, что занимает большую часть запроса: содержимое самого разговора или системный prompt, определения инструментов и содержимое вложений, которые Claude Code отправляет с ним. Когда содержимое самого разговора занимает большую часть запроса, сообщение читается:

```text theme={null}
Prompt is too long · the request is ~<токены запроса> tokens (limit <лимит>) and this conversation's own content is most of it. A single-exchange conversation cannot be compacted; start with less content (smaller files or pasted text).
```

Когда большая часть запроса находится вне разговора, сообщение читается:

```text theme={null}
Prompt is too long · the request is ~<токены запроса> tokens (limit <лимит>) but this conversation is only ~<токены разговора> tokens — the rest is system prompt, tool definitions, and attachment content. A single-exchange conversation cannot be compacted; reduce attached files/tools or start with less context.
```

До версии 2.1.162 Claude Code всё равно пытался выполнить компактирование и выводил простой `Prompt is too long` при его неудаче.

**Что делать:**

* Запустите `/compact` для суммирования более ранних ходов и освобождения места, или `/clear` для начала заново. Если `/compact` ответит `Not enough messages to compact.`, разговор — это однооборотный обмен без более ранних ходов для суммирования, поэтому место занято этим одним prompt и тем, что Claude Code отправляет с каждым запросом: запустите `/clear` и отправьте заново с меньшим вставленным текстом или меньшими вложениями, или уменьшите определения инструментов и файлы памяти, используя шаги ниже
* Запустите `/context` для просмотра разбивки того, что потребляет окно: системный prompt, инструменты, файлы памяти и сообщения
* Отключите MCP серверы, которые вы не используете, с помощью `/mcp disable <имя>` для удаления их определений инструментов из контекста
* Обрежьте большие файлы памяти `CLAUDE.md` или переместите инструкции в [правила с областью действия пути](/docs/ru/memory#path-specific-rules), которые загружаются только при необходимости
* Подагенты наследуют каждое определение инструмента MCP от родительского сеанса, что может заполнить их контекстное окно до первого хода. Отключите MCP серверы, которые вы не используете, перед созданием подагентов.
* Автоматическое компактирование включено по умолчанию и обычно предотвращает эту ошибку. Если вы отключили его в `/config` или с помощью [`DISABLE_AUTO_COMPACT`](/docs/ru/env-vars), включите его обратно. Если вы держите его отключённым, запустите `/compact` самостоятельно перед заполнением окна.

Смотрите [Explore the context window](/docs/ru/context-window) для интерактивного просмотра того, как заполняется контекст.

<h3 id="context-exceeds-the-token-limit">
  Context exceeds the token limit
</h3>

`/context` показывает это предупреждение в верхней части своего вывода, когда разговор превысил контекстное окно модели. Запросы завершаются с [`Prompt is too long`](#prompt-is-too-long) до тех пор, пока вы не освободите место. Интерактивный сеанс показывает эту ошибку как строку `Context limit reached`.

```text theme={null}
Context exceeds the 200k-token limit by 94k tokens — run /compact or /clear to continue.
```

Когда превышенный лимит — это окно компактирования, такое как граница 200K на моделях с контекстом 1M, предупреждение читается иначе. Окно компактирования может находиться ниже контекстного окна модели, поэтому запросы за его пределами могут всё ещё успешно выполняться.

```text theme={null}
Context is 94k tokens past the 200k-token compaction window — run /compact to reduce usage.
```

Обе формы называют `/clear` вместо `/compact`, когда вы установили [`DISABLE_COMPACT`](/docs/ru/env-vars).

**Что делать:**

* В многооборотном разговоре запустите `/compact` для суммирования более ранних ходов и освобождения места. Для начала заново запустите `/clear`
* Для дополнительных способов снижения использования смотрите [Prompt is too long](#prompt-is-too-long)

До версии 2.1.216 `/context` показывал использование выше 100% без строки предупреждения, объясняющей, что это означает или как восстановиться.

<h3 id="error-during-compaction-conversation-too-long">
  Error during compaction: Conversation too long
</h3>

`/compact` сам завершился с ошибкой, потому что недостаточно свободного контекста для хранения создаваемого им резюме.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Это может произойти, когда окно уже заполнено в момент срабатывания автоматического компактирования, или когда вы запускаете `/compact` после просмотра [`Prompt is too long`](#prompt-is-too-long). В интерактивном сеансе эта ошибка — это строка `Context limit reached`.

**Что делать:**

* Нажмите Esc дважды, чтобы открыть список сообщений и вернуться на несколько ходов назад. Это удаляет самые последние сообщения из контекста. Затем запустите `/compact` снова.
* Если возврат на несколько ходов не освобождает достаточно места, запустите `/clear` для начала свежего сеанса. Ваш предыдущий разговор сохраняется и может быть переоткрыт с помощью `/resume`.

Это сообщение и другие сбои `/compact` отображаются в стиле ошибки. До версии 2.1.216 они отображались в том же тусклом стиле, что и успешный вывод команды, поэтому вы могли прочитать неудачное компактирование как успех.

<h3 id="request-too-large">
  Request too large
</h3>

Тело необработанного запроса превысило лимит API в 32MB перед токенизацией, обычно из-за большого вставленного содержимого, результатов инструментов или вложений. Этот лимит отделён от [контекстного окна](#prompt-is-too-long).

```text theme={null}
Request too large (max 32MB). Accumulated images and attachments in the conversation pushed the request over the limit. Run /compact, or double press esc to go back and remove attachments.
```

Когда запрос прошёл прямо в Claude API и сам API отклонил его, Claude Code измеряет разговор и формулирует сообщение в зависимости от того, может ли работать восстановление. Через прокси, шлюз или поставщика облачных услуг вы получаете общее сообщение. Измеренные формы:

* `Request too large (max 32MB; 20.1MB of about 33.4MB is images or documents).`: изображения или документы превысили лимит запроса. Claude Code повторяет попытку с их удалением.
* `Request too large for the API's 32MB request limit`: сами сообщения превышают лимит, поэтому сообщение говорит `compacting cannot make it fit` и Claude Code не повторяет попытку. В [неинтерактивном режиме](/docs/ru/headless) сообщение говорит вам уменьшить входные данные или начать новый сеанс.

До версии 2.1.212 разговоры с достаточным накопленным количеством изображений завершались с ошибкой на каждом ходу с `Request too large (max 32MB). Double press esc to go back and try with a smaller file.` До версии 2.1.229 Claude Code показывал совет о вложениях для каждого отклонения, даже когда компактирование не могло помочь.

**Что делать:**

* Если сообщение говорит `compacting cannot make it fit`, нажмите Esc дважды, чтобы вернуться на ход, который добавил большое содержимое, или запустите `/clear` для начала заново
* В противном случае запустите `/compact`, который удаляет накопленные изображения и вложения
* Ссылайтесь на большие файлы по пути вместо вставки их содержимого, чтобы Claude мог читать их по частям
* Для изображений смотрите [Image was too large](#image-was-too-large) ниже

<h3 id="image-was-too-large">
  Image was too large
</h3>

Вставленное или прикреплённое изображение превышает лимиты размера или размеров API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code заменяет необработанное изображение текстовым заполнителем и повторяет попытку, поэтому последующие сообщения успешны. В версиях до 2.1.142 вставленное изображение могло остаться в разговоре и повторить ту же ошибку на каждом последующем сообщении. Для восстановления в этих версиях нажмите Esc дважды и вернитесь на ход, где было добавлено изображение.

**Что делать:**

* Измените размер изображения перед вставкой. API принимает изображения до 8000 пикселей на самой длинной стороне для одного изображения или 2000 пикселей, когда в контексте много изображений.
* Сделайте более плотный снимок экрана соответствующей области вместо полного экрана

<h3 id="unable-to-resize-image">
  Unable to resize image
</h3>

Claude Code не смог уменьшить масштаб прикреплённого изображения перед отправкой его в API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
Unable to resize image — it is a CMYK JPEG, which Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Re-save it as an RGB PNG or JPEG and try again.
Unable to resize image — it is an animated WebP whose first frame Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Save its first frame as a PNG or JPEG and try again.
Unable to resize image — its pixels could not be decoded (the file may be damaged, or use an encoding Claude Code cannot read), and it is over the … API limit (… raw, … base64), so it cannot be sent. Re-save it as a PNG or JPEG and try again.
```

Claude Code обычно автоматически изменяет размер больших изображений. Эти ошибки означают, что изображение не могло быть декодировано или изменено в размере, чтобы соответствовать лимитам API.

**Что делать:**

* Если сообщение просит вас преобразовать изображение, преобразуйте его в PNG, JPEG, GIF или WebP и прикрепите снова. Claude Code может проверить размеры для этих форматов из заголовка файла без декодирования изображения.
* Если сообщение сообщает о лимите размера или размеров, измените размер или перекомпрессируйте изображение ниже этого лимита перед прикреплением.
* Если сообщение называет причину, такую как CMYK JPEG, анимированный WebP или возможно повреждённый файл, пересохраните изображение в формате, который предлагает сообщение, и прикрепите его снова.

<h3 id="pdf-errors">
  PDF errors
</h3>

Прикреплённый PDF не смог быть обработан. Сообщения показаны здесь в их неинтерактивной форме; в интерактивном сеансе они вместо этого предлагают вам дважды нажать esc и попробовать снова.

```text theme={null}
PDF too large (max 100 pages, 20MB). Try reading the file a different way (e.g., extract text with pdftotext).
PDF is password protected. Try using a CLI tool to extract or convert the PDF.
The PDF file was not valid. Try converting it to text first (e.g., pdftotext).
```

**Что делать:**

* Для больших PDF попросите Claude прочитать диапазон страниц с помощью инструмента Read вместо прикрепления всего файла, или извлеките текст с помощью инструмента, такого как `pdftotext`, и ссылайтесь на выходной файл по пути
* Для защищённых или недействительных PDF удалите пароль или повторно экспортируйте файл из исходного приложения, затем попробуйте снова

<h3 id="extra-inputs-are-not-permitted">
  Extra inputs are not permitted
</h3>

Прокси или шлюз LLM между Claude Code и API удалил заголовок запроса `anthropic-beta`, поэтому API отклонил поля, которые от него зависят.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code отправляет поля, доступные только в бета-версии, такие как `context_management` и `effort`, вместе с заголовком `anthropic-beta`, который их включает. Когда шлюз пересылает тело, но удаляет заголовок, API видит поля, которые не распознаёт.

**Что делать:**

* Настройте ваш шлюз для пересылки заголовка `anthropic-beta`. Смотрите [feature pass-through](/docs/ru/llm-gateway-protocol#feature-pass-through) для того, что шлюзы должны пересылать.
* В качестве резервного варианта установите [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/ru/env-vars) перед запуском. [Disable pre-release capabilities](/docs/ru/llm-gateway-protocol#disable-pre-release-capabilities) охватывает точный объём.

<h3 id="tool-input-schema-is-invalid">
  Tool input schema is invalid
</h3>

Инструмент в запросе объявил `input_schema`, который не проходит валидацию JSON Schema API, поэтому API отклонил весь запрос. Число после `tools.` — это позиция неудачного инструмента в списке инструментов запроса, а не имя, которое вы можете найти.

```text theme={null}
API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid
API Error: 400 ... tools.N.custom.input_schema.properties: Property keys should match pattern '^[a-zA-Z0-9_.-]{1,64}$'
```

Первая форма означает, что схема не является действительной JSON Schema draft 2020-12. Вторая означает, что имя свойства верхнего уровня не соответствует шаблону, который называет сообщение.

Claude Code [исключает MCP инструменты, чья схема входных данных не прошла бы эту валидацию](/docs/ru/mcp#tools-with-invalid-input-schemas) при загрузке инструментов сервера, поэтому запросы обычно никогда не включают один.

На [развёртывании, где получение флагов отключено](/docs/ru/env-vars#features-that-need-feature-flag-fetching), или на машине, флаги которой никогда не прибывали, Claude Code записывает в журнал сервера, какой инструмент будет отклонён, но отправляет его всё равно, поэтому эта ошибка всё ещё может произойти.

Ошибка также может произойти для инструмента, чья схема объявляет диалект JSON Schema, отличный от draft 2020-12, в `$schema`. Claude Code не проверяет эти схемы против мета-схемы JSON Schema, хотя проверка имён свойств верхнего уровня всё ещё применяется.

До версии 2.1.216 ни одно развёртывание не выполняло проверки исключения.

**Что делать:**

* Если ваша версия Claude Code раньше v2.1.216, запустите `claude update`.
* Удалите или [отключите](/docs/ru/mcp#disable-a-server-without-removing-it) MCP сервер, который объявляет недействительную схему. Ошибка называет инструмент только по позиции. На v2.1.216 или позже проверьте журнал каждого сервера для строки, называющей инструмент, чья схема входных данных будет отклонена. Если ни один журнал не называет один, отключайте серверы по одному.
* Если вы поддерживаете сервер, исправьте `input_schema` инструмента. Схема должна быть действительной JSON Schema, и имена свойств верхнего уровня должны быть от 1 до 64 символов в длину и использовать только буквы ASCII и цифры, `_`, `.` и `-`. Смотрите [Tools with invalid input schemas](/docs/ru/mcp#tools-with-invalid-input-schemas).

<h3 id="theres-an-issue-with-the-selected-model">
  There's an issue with the selected model
</h3>

Имя настроенной модели не было распознано или ваша учётная запись не имеет доступа к ней. Начиная с v2.1.160 конечная подсказка, показанная здесь в её интерактивной форме, варьируется в зависимости от поверхности.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Что делать:**

* **Интерактивный CLI**: запустите `/model` для выбора из моделей, доступных вашей учётной записи.
* **Неинтерактивный режим (`-p`)**: передайте `--model` с действительным псевдонимом или ID, или установите [`ANTHROPIC_MODEL`](/docs/ru/env-vars). Текст ошибки показывает `Run --model` на этой поверхности.
* **Agent SDK**: текст ошибки опускает подсказку, потому что модель установлена программно. Установите [`model` на `Options`](/docs/ru/agent-sdk/typescript#options) в TypeScript или [`ClaudeAgentOptions(model=...)`](/docs/ru/agent-sdk/python#claudeagentoptions) в Python, и обработайте структурированную ошибку `model_not_found` для вывода вашего собственного повтора или выбора модели.
* Используйте псевдоним, такой как `sonnet` или `opus`, вместо полного версионного ID. Псевдонимы разрешаются в поддерживаемое значение по умолчанию, поэтому они не устаревают. Смотрите [Model configuration](/docs/ru/model-config).
* Если неправильная модель продолжает возвращаться в CLI, где-то установлен устаревший ID. Проверьте места, где вы можете установить модель в [порядке приоритета](/docs/ru/model-config#setting-your-model), и удалите устаревшее значение.
* Недавно запущенная модель может быть доступна на Anthropic API перед Amazon Bedrock, Google Cloud's Agent Platform или Microsoft Foundry. Если вы закрепили новый ID модели на одном из этих поставщиков и видите эту ошибку, проверьте каталог моделей вашего поставщика на доступность в вашем регионе и держите предыдущую версию закреплённой до появления новой там.
* Claude Code сообщает об истёкшем входе claude.ai как [Login expired](#login-expired), а не как эта ошибка. До версии 2.1.206 истёкший вход, который больше не мог быть обновлён, завершался с ошибкой на каждой модели с этой ошибкой; запустите `/login`, если вы видите это в более старой версии.
* Для развёртываний Google Cloud's Agent Platform смотрите [Google Cloud's Agent Platform troubleshooting](/docs/ru/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Model is not a recognized model id
</h3>

Строка модели, которую вы передали переключателю модели, не является псевдонимом модели, ID модели, который эта версия Claude Code знает, или ID, который начинается с `claude-`. Обычные причины — опечатка в ID, отображаемое имя, такое как `Sonnet 5`, где ожидается ID `claude-sonnet-5`, или псевдоним, который распознают только более новые версии Claude Code. Claude Code отклоняет переключение немедленно. До версии 2.1.200 Claude Code сохранял строку и завершался с ошибкой на следующем запросе с [There's an issue with the selected model](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

Конечная подсказка называет ближайший соответствующий псевдоним или ID модели. Когда ничего не достаточно близко, она читается `Run /model to see available models.` вместо этого.

Claude Code производит эту ошибку локально в момент запроса переключения, перед любым запросом API. Она применяется, когда модель установлена через метод [Agent SDK](/docs/ru/agent-sdk/typescript) `setModel()`, приложением, таким как [Desktop app](/docs/ru/desktop), которое запускает Claude Code CLI для вас, или когда вы выбираете модель с устройства, подключённого через [Remote Control](/docs/ru/remote-control). До версии 2.1.260 проверка не охватывала выборы Remote Control, поэтому Claude Code применял выбор и следующий запрос завершался с [There's an issue with the selected model](#theres-an-issue-with-the-selected-model).

**Что делать:**

* Запустите `/model` без аргумента, чтобы открыть выбор и выбрать из моделей, доступных вашей учётной записи, затем передайте показанный там псевдоним или ID
* Если вы использовали псевдоним, который поддерживает более новая версия Claude Code, запустите `claude update`. Полный ID, который начинается с `claude-`, проходит эту локальную проверку даже когда модель новее вашей версии Claude Code. Сервер всё ещё может требовать минимальную версию для этой модели; смотрите [Claude Code does not support this model](#claude-code-does-not-support-this-model).
* Модель, сохранённая до версии 2.1.200, не восстанавливается этой проверкой. Если устаревшее значение продолжает возвращаться, удалите его из мест, перечисленных в [Setting your model](/docs/ru/model-config#setting-your-model).
* Проверка выполняется только на Anthropic API. На любом другом поставщике или шлюзе, включая пользовательский `ANTHROPIC_BASE_URL`, поставщик определяет имена моделей, поэтому Claude Code принимает любую строку и передаёт её. Claude Code всё ещё может написать [неузнанную диагностическую строку ID модели](#unrecognized-model-id-on-a-request) во время запроса на каждом поставщике.

<h3 id="model-not-found">
  Model not found
</h3>

Вы выбрали модель с `/model <имя>` и Claude Code не смог подтвердить, что модель с этим именем существует. Когда имя не является [псевдонимом модели](/docs/ru/model-config#model-aliases) или другим написанием, которое Claude Code принимает локально, `/model` проверяет его с минимальным запросом API, и эта ошибка обычно является ответом вашей конечной точки API. Имя, которое не может быть ID модели вообще, такое как содержащее пробелы, получает то же сообщение.

```text theme={null}
Model 'claude-opus-9' not found
```

На поставщиках с ID моделей, специфичными для поставщика, сообщение может добавить предложение `Try '...' instead`, которое называет ID вашего поставщика для резервной модели.

**Что делать:**

* Запустите `/model` без аргумента и выберите из моделей, доступных вашей учётной записи, или используйте [псевдоним модели](/docs/ru/model-config#model-aliases), такой как `sonnet`, который разрешается в поддерживаемое значение по умолчанию
* Если вы ввели полный ID, проверьте его в каталоге моделей вашего поставщика. Недавно запущенная модель может быть доступна на Anthropic API перед тем, как ваш поставщик или регион её предложит.
* До версии 2.1.265 `/model` также отклонял написание псевдонима `opusplan[1m]` с этой ошибкой. В этих версиях обновите Claude Code или установите модель в [settings](/docs/ru/model-config#setting-your-model) или с помощью `--model` вместо этого.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus is not available with the Claude Pro plan
</h3>

Ваш активный план подписки не включает выбранную вами модель.

```text theme={null}
Claude Opus is not available with the Claude Pro plan. If you have updated your subscription plan recently, run /logout and /login for the plan to take effect.
```

**Что делать:**

* Запустите `/model` и выберите модель, которую включает ваш план
* Если вы недавно обновили свой план и всё ещё видите это, запустите `/logout`, затем `/login`. Сохранённый токен отражает ваш план на момент входа, поэтому обновление в интернете не вступает в силу в существующем сеансе до повторной аутентификации.
* Смотрите [claude.com/pricing](https://claude.com/pricing) для того, какие модели включает каждый план

<h3 id="claude-code-does-not-support-this-model">
  Claude Code does not support this model
</h3>

API отклонил запрос с 400, потому что ваша версия Claude Code ниже требуемого минимума. Либо выбранная вами модель требует более новую версию, которую сервер проверяет для каждой модели, либо политика вашей организации требует одну. 400 несёт код ошибки `claude_code_version_too_old`, и сообщение говорит, какой минимум применяется.

```text theme={null}
API Error: 400 Claude Code 2.1.219 does not support this model; version 2.1.255 or newer is required. Run 'claude update', or update the Claude desktop app, then try again.
```

Формулировка политики организации читается:

```text theme={null}
API Error: 400 Claude Code 2.1.240 is older than the minimum version required by your organization's policy. Run 'claude update', or update the Claude desktop app, to continue.
```

**Что делать:**

* Запустите `claude update` или обновите Claude desktop app, затем начните новый сеанс
* Для формулировки для каждой модели вы можете продолжить работу в текущем сеансе, переключившись на другую модель с помощью `/model`
* Для формулировки политики организации обновитесь перед продолжением

<h3 id="model-is-restricted-by-your-organizations-settings">
  Model is restricted by your organization's settings
</h3>

Администратор вашей организации отключил эту модель в консоли администратора claude.ai, или она исключена списком разрешений [`availableModels`](/docs/ru/model-config#restrict-model-selection) в управляемых настройках. Когда ограниченная модель была установлена с `--model`, `ANTHROPIC_MODEL` или настройкой `model`, Claude Code подставляет разрешённую модель и продолжает. Ввод `/model <имя>` для ограниченной модели отклоняется с `Run /model to choose a different model.` и сеанс сохраняет свою текущую модель. Уведомление о подстановке также может появиться в середине сеанса после того, как администратор отключит модель, на которой работает сеанс, в консоли администратора claude.ai.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Уведомление с префиксом имени агента, навыка или команды означает, что ограничение применилось к [запрошенной модели подагента](/docs/ru/sub-agents#choose-a-model): подагент работает на подставленной модели и модель вашего сеанса не изменяется. До версии 2.1.223 Claude Code показывал уведомление только для подагентов, запущенных с помощью инструмента Agent.

Claude Code обрабатывает псевдоним семейства моделей, один из `opus`, `sonnet`, `haiku` или `fable`, как запрос для этого семейства, а не для его новейшей версии. На Anthropic API и на [Claude Platform on AWS](/docs/ru/claude-platform-on-aws) ограниченный псевдоним семейства разрешается в новейшую версию семейства, которую разрешают ваша организация и список разрешений `availableModels`, и уведомление о подстановке называет эту версию. Claude Code отклоняет `/model <псевдоним>` только когда каждая версия семейства ограничена. До версии 2.1.205 псевдоним семейства был подставлен или отклонен на основе только его новейшей версии, даже когда была разрешена более старая версия того же семейства.

**Что делать:**

* Запустите `/model` для выбора из моделей, которые разрешает ваша организация. Ограниченные модели скрыты от выбора.
* Если ограниченная модель была установлена в `--model`, `ANTHROPIC_MODEL`, поле `model` файла настроек или frontmatter [подагента](/docs/ru/sub-agents#choose-a-model), навыка или команды, удалите или обновите это значение, чтобы уведомление не повторялось
* Если вам нужен доступ к ограниченной модели, попросите администратора вашей организации её включить. Смотрите [Organization model restrictions](/docs/ru/model-config#organization-model-restrictions).

<h3 id="model-switch-was-blocked-by-a-premodelswitch-hook">
  Model switch was blocked by a PreModelSwitch hook
</h3>

Хук [PreModelSwitch hook](/docs/ru/hooks#premodelswitch) не одобрил переключение модели, которое вы или клиент запросили, поэтому сеанс сохраняет свою текущую модель. Когда переключение пришло от хоста [Agent SDK](/docs/ru/agent-sdk/overview) или [Remote Control](/docs/ru/remote-control), а не от команды, которую вы ввели, сообщение читается `Model switch blocked by a PreModelSwitch hook` без названия целевой модели.

```text theme={null}
Model switch to Opus 4.6 was blocked by a PreModelSwitch hook: Opus 4.6 is retired for this project. Use a newer model.
```

Причина после двоеточия говорит, что отклонило переключение:

* **Причина, которую написал хук**: хук PreModelSwitch предоставил эту причину, когда он [отклонил переключение или попросил подтверждение](/docs/ru/hooks#premodelswitch-decision-control). Обратитесь к тому, что он просит, или выберите модель, которую разрешают ваши хуки.
* **`PreModelSwitch hook <имя> did not respond before its timeout`**: хук, который не отвечает перед его [timeout](/docs/ru/hooks#timeouts), блокирует переключение. Исправьте зависающую команду или повысьте `timeout` этого хука, затем переключитесь снова.
* **`confirmation required, and this session cannot ask`**: хук ответил `ask` без причины, и запрос управления не имеет способа показать подсказку подтверждения. Запрос `/model` в запуске [`-p`](/docs/ru/headless) сообщает то же состояние с `(run /model interactively to confirm)` после причины. Сделайте переключение из интерактивного сеанса или измените решение хука для этой модели.
* **`so organization-managed PreModelSwitch hooks could not be checked`**: Claude Code не смог определить, какие хуки PreModelSwitch предоставляют [управляемые плагины](/docs/ru/settings-reference#enabledplugins) вашей организации, например, потому что управляемый плагин не загрузился. Один из этих хуков может заблокировать переключение, поэтому Claude Code отказывает, а не применяет переключение без проверки. Начало причины называет, что не удалось. Claude Code повторно проверяет при каждой попытке переключения, поэтому сбой, который с тех пор очистился, перестаёт блокировать; если он продолжает не удаваться, запустите `claude --debug` и переключитесь снова, чтобы захватить детали, затем исправьте плагин или попросите администратора его исправить.
* **`a PreModelSwitch hook failed before answering`** или **`PreModelSwitch hooks were cancelled (the control stream closed) before answering`**: запуск хука завершился без вердикта, и Claude Code не рассматривает это как одобрение. Запустите `claude --debug`, чтобы увидеть, что не удалось, затем переключитесь снова.

До версии 2.1.260 отказ управляемого плагина читался `plugin hooks could not be loaded, so PreModelSwitch hooks could not be checked; see the debug log`. Claude Code повторил загрузку плагина один раз, а затем отклонил более поздние переключения в сеансе, даже когда ваша организация не управляла никакими плагинами. Перезагрузите сеанс, чтобы запустить загрузку плагина снова в этих версиях.

<h3 id="couldnt-save-it-as-your-default">
  Couldn't save it as your default
</h3>

Вы выбрали модель для сохранения в качестве вашего значения по умолчанию, например, с `/model <имя>` или `Enter` в выборе `/model`, и Claude Code не смог написать выбор в файл пользовательских настроек, `~/.claude/settings.json`. Само переключение применилось, поэтому текущий сеанс работает на выбранной вами модели, но ваше значение по умолчанию не изменяется и следующий сеанс начинается со старого значения.

```text theme={null}
Set model to Fable 5.1 for this session only · couldn't save it as your default: ~/.claude/settings.json can't be written (EROFS)
```

Причина после пути файла говорит, что не удалось:

* **`can't be written (<код>)`**: запись не удалась с кодом ошибки операционной системы в скобках, такой как `EROFS`, когда файл или файл, на который он ссылается, находится на файловой системе, которая отказывает в записи. Сделайте файл доступным для записи и переключитесь снова. Если другой инструмент генерирует файл, установите ключ `model` в этом инструменте вместо этого; смотрите [A change you made in Claude Code is lost in new sessions](/docs/ru/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **`isn't valid JSON`**: файл на диске не анализируется, и Claude Code оставляет его нетронутым, а не перезаписывает содержимое, которое не может прочитать обратно. Исправьте синтаксическую ошибку, затем переключитесь снова; смотрите [Fix a broken settings file](/docs/ru/settings#fix-a-broken-settings-file).

Уведомление, заканчивающееся `couldn't confirm it was saved as your default (~/.claude/settings.json is still being written)`, означает, что запись не завершилась после трёх секунд. Она продолжается в фоне, поэтому значение по умолчанию может всё ещё быть сохранено; проверьте, на какой модели начинается ваш следующий сеанс, или запустите `/model <имя>` снова.

До версии 2.1.265 уведомление говорило, что модель была `saved as your default for new sessions`, даже когда запись не удалась.

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled is not supported for this model
</h3>

Ваша версия Claude Code старше минимума для выбранной модели. CLI отправил конфигурацию мышления, которую модель больше не принимает.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Что делать:**

* Запустите `claude update` и перезагрузите Claude Code. Opus 4.7 требует v2.1.111 или позже. Opus 4.8 требует v2.1.154 или позже. Sonnet 5 требует v2.1.197 или позже. Opus 5 требует v2.1.219 или позже. Opus 5.5 требует v2.1.280 или позже
* Если вы не можете обновиться, запустите `/model` и выберите Opus 4.6 или Sonnet 4.6 вместо этого
* Если вы столкнулись с этим в [Agent SDK](/docs/ru/agent-sdk/overview), обновите пакет SDK вместо этого. Opus 4.8 требует TypeScript SDK v0.3.154 или позже и Python SDK v0.2.88 или позже. Sonnet 5 требует TypeScript SDK v0.3.197 или позже. Opus 5 требует TypeScript SDK v0.3.219 или позже. Opus 5.5 требует TypeScript SDK v0.3.280 или позже

<h3 id="effort-isnt-available-with-thinking-turned-off">
  Effort isn't available with thinking turned off
</h3>

Вы отключили [extended thinking](/docs/ru/model-config#extended-thinking) и работали на [уровне усилий](/docs/ru/model-config#adjust-effort-level) выше `high`. Модель не принимает эту комбинацию, поэтому API отклонил запрос.

```text theme={null}
API Error: Effort 'xhigh' isn't available with thinking turned off on this model · run /effort high to continue, or turn thinking back on (unset MAX_THINKING_TOKENS=0)
```

**Что делать:**

* [Понизьте уровень усилий](/docs/ru/model-config#set-the-effort-level) до `high` или ниже.
* Включите мышление обратно, например, отменив [`MAX_THINKING_TOKENS`](/docs/ru/env-vars) или удалив [`"alwaysThinkingEnabled": false`](/docs/ru/settings-reference#alwaysthinkingenabled) из ваших настроек.

До версии 2.1.242 Claude Code показывал собственное сообщение API: `API Error: 400 output_config.effort 'xhigh' is not supported when thinking is disabled on this model. Use effort 'high' or below, or enable thinking.` До версии 2.1.251 Claude Code отправлял запрос на уровне усилий, который вы установили, поэтому Opus 5 отклонял каждый запрос выше `high` с отключённым мышлением. Claude Code теперь отправляет усилие `high` вместо этого моделям, которые, как он знает, отклоняют комбинацию, такие как Opus 5, поэтому на v2.1.251 или позже эта ошибка достигает вас только от модели, которую Claude Code не знает отклоняет.

<h3 id="thinking-budget-exceeds-output-limit">
  Thinking budget exceeds output limit
</h3>

Настроенный бюджет расширенного мышления превышает максимальную длину ответа, поэтому для фактического ответа не остаётся места.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code автоматически регулирует эти значения на Anthropic API. Вы обычно видите эту ошибку на Amazon Bedrock или Google Cloud's Agent Platform, когда [`MAX_THINKING_TOKENS`](/docs/ru/env-vars) установлен выше лимита вывода поставщика, или когда режим плана повышает бюджет мышления.

**Что делать:**

* Понизьте `MAX_THINKING_TOKENS` или повысьте [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/ru/env-vars) выше бюджета мышления
* Смотрите [Extended thinking](/docs/ru/model-config#extended-thinking) для того, как бюджет взаимодействует с длиной вывода

<h3 id="tool-use-or-thinking-block-mismatch">
  Tool use or thinking block mismatch
</h3>

История разговора достигла API в несогласованном состоянии, обычно после того, как вызов инструмента был прерван или ход был отредактирован в середине потока.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 orphaned tool_result in conversation history. Run /rewind to recover the conversation.
API Error: 400 duplicate tool_use ID in conversation history. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Все варианты означают одно и то же: последовательность блоков `tool_use`, `tool_result` и `thinking` в истории больше не соответствует тому, что ожидает API.

**Что делать:**

* Если вы используете Opus 4.7 или Opus 4.8, сначала запустите `claude update`. Версии до v2.1.156 могут вызвать эту ошибку во время нормального использования инструмента, и `/rewind` её не очищает.
* Запустите `/rewind` или нажмите Esc дважды, чтобы вернуться к контрольной точке перед повреждённым ходом и продолжить оттуда. Смотрите [Checkpointing](/docs/ru/checkpointing) для того, как создаются и восстанавливаются контрольные точки.

<h3 id="unsupported-tool-content-removed">
  Unsupported tool content removed
</h3>

Когда Claude Code подключается непосредственно к Anthropic API и загружает или предварительно просматривает сохранённый сеанс, он удаляет содержимое инструмента, которое Anthropic API не принимает, и оставляет эту строку там, где удалённое содержимое находилось между двумя блоками мышления:

```text theme={null}
[Unsupported tool content removed]
```

Такое содержимое достигает файла сеанса, когда что-то другое, чем Anthropic API, ответило в формате API, обычно прокси третьей стороны, установленный через [`ANTHROPIC_BASE_URL`](/docs/ru/env-vars), который переводит вызовы инструментов другого поставщика. Claude Code удаляет его только когда сеанс подключается непосредственно к Anthropic API и загружает сохранённую историю, как она есть, когда сеанс работает через прокси или на другом поставщике. До версии 2.1.246 Claude Code отправлял использование инструмента и его результат обратно в API, и каждый ход возобновленного сеанса завершался с ошибкой 400, такой как `messages.1.content.0.server_tool_use.name: Input should be 'web_search', 'web_fetch', ...`.

**Что делать:**

* Ничего не требуется, когда вы видите строку заполнителя. Сеанс продолжается без удалённого содержимого.
* Если каждый ход возобновленного сеанса завершается с ошибкой 400 вместо этого, запустите `claude update` и возобновите сеанс снова. Версии до v2.1.246 не удаляют содержимое.

<h3 id="role-system-must-precede-an-assistant-message">
  role 'system' must precede an 'assistant' message
</h3>

API отклонил запрос с 400, потому что системное сообщение находится в позиции в разговоре, которую он не принимает:

```text theme={null}
API Error: 400 messages.6: role 'system' must precede an 'assistant' message or end the array; ...
```

Claude Code отправляет часть своего напоминания и текста вложений как системные сообщения внутри разговора. Когда API отклоняет позицию одного, Claude Code повторяет запрос один раз с этим текстом, отправленным как обычные пользовательские сообщения вместо этого. Формулировки соседства API, такие как `use the top-level 'system' parameter for the initial system prompt`, получают то же восстановление.

Когда ошибка действительно появляется, отклонённое системное сообщение не является тем, которое Claude Code может удалить. Это обычно означает, что прокси или [LLM gateway](/docs/ru/llm-gateway) между Claude Code и API добавил системное сообщение своё или переупорядочил разговор.

**Что делать:**

* Запустите `/clear` для начала свежего разговора. Если ошибка возвращается там тоже, причина находится на пути запроса, а не в сохранённом разговоре.
* Если ошибка повторяется на каждом ходу позади прокси или шлюза, настроенного через [`ANTHROPIC_BASE_URL`](/docs/ru/env-vars), подключитесь без прокси, чтобы подтвердить источник, и сообщите об ошибке тому, кто его управляет

До версии 2.1.280 Claude Code не распознавал эту формулировку, поэтому ошибка также появлялась, когда отклонённое системное сообщение было тем, которое сам Claude Code отправил, и каждый более поздний ход разговора завершался с той же ошибкой.

<h3 id="invalid-encrypted-content-in-search-result-block">
  Invalid encrypted\_content in search\_result block
</h3>

API отклонил запрос с 400, потому что история разговора содержит размещённое содержимое веб-поиска, которое он не может расшифровать. Формулировка называет поле, которое он не может прочитать:

```text theme={null}
API Error: 400 messages.21.content.0: Invalid `encrypted_content` in `search_result` block
API Error: 400 messages.21.content.3.citations.0: Invalid `encrypted_index` in `text` block
API Error: 400 Failed to decrypt web search result content
```

Результаты из размещённого [инструмента веб-поиска](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool) API содержат зашифрованные поля, которые только API может прочитать. API отклоняет запрос, который воспроизводит содержимое, которое он не может расшифровать, такое как содержимое, созданное для другой организации.

Собственный [инструмент WebSearch](/docs/ru/tools-reference#websearch-tool-behavior) Claude Code записывает результаты поиска как простой текст, поэтому эти блоки обычно достигают разговора через прокси или [LLM gateway](/docs/ru/llm-gateway), который сам запустил размещённый веб-поиск.

Отклонённые блоки остаются в истории разговора, поэтому каждый более поздний ход и `/compact` завершаются с той же ошибкой.

**Что делать:**

* Запустите `/clear` или начните новый сеанс; новый разговор не содержит отклонённые блоки
* Если вы запускаете Claude Code позади прокси или шлюза, сообщите об ошибке тому, кто его управляет

<h3 id="usage-policy-refusal">
  Usage Policy refusal
</h3>

API отклонил ответ, потому что содержимое в разговоре вызвало проверку [Usage Policy](https://www.anthropic.com/legal/aup). Сообщение включает ID запроса, который вы можете привести в поддержку, если вы считаете, что отказ неправильный.

```text theme={null}
API Error: Opus 4.6 can't help with this. Start a new session to continue.

Send feedback with /feedback or learn more: https://www.anthropic.com/legal/aup
```

Сообщение называет модель, которая отклонила, или `Claude`, когда модель не записана.

Проверка оценивает весь разговор, а не только ваш последний prompt, поэтому отправка нового сообщения в том же сеансе обычно повторно вызывает тот же отказ. То же самое применяется после выхода и повторного открытия сеанса с `--continue` или `--resume`, так как расшифровка на диске всё ещё содержит вызывающее содержимое. На [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) и [Microsoft Foundry](/docs/ru/microsoft-foundry) это сообщение также охватывает запросы, которые меры безопасности модели отметили как тему кибербезопасности. Смотрите [Safety measures flagged a cybersecurity topic](#safety-measures-flagged-a-cybersecurity-topic).

До версии 2.1.219 сообщение читалось `Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.`

**Что делать:**

* Нажмите Esc дважды или запустите `/rewind`, чтобы вернуться к контрольной точке перед ходом, который вызвал отказ, затем переформулируйте или возьмите другой подход. Смотрите [Checkpointing](/docs/ru/checkpointing).
* Если вы не можете определить, какой ход вызвал это, запустите `/clear`, чтобы начать свежий разговор в том же проекте. Ваш предыдущий разговор сохраняется на диске и остаётся доступным в `/resume`.
* В [неинтерактивном режиме](/docs/ru/headless) (`-p`), где перемотка недоступна, повторите попытку с переформулированным prompt в новом сеансе без `--continue`. Проверки политики варьируются в зависимости от модели, поэтому переключение на другую модель с помощью `--model` также может разрешить отказ в некоторых случаях.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Safety measures flagged a cybersecurity topic
</h3>

Меры безопасности модели отметили содержимое в разговоре как тему кибербезопасности. Сообщение называет модель, которая отметила запрос:

```text theme={null}
API Error: Opus 4.8's safeguards flagged this message. Our intentionally broad safeguards allow us to deliver more capabilities faster, but can sometimes flag legitimate cybersecurity work. Apply to the Cyber Verification Program to reduce these interruptions. Send feedback with /feedback or learn more: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude
```

Сообщение ссылается на [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), который предоставляет доступ для законной работы в области кибербезопасности. На Opus 5.5, который требует v2.1.280 или позже, сообщение открывается с `Opus 5.5's safeguards flagged this session` вместо этого. Когда отмеченная категория имеет доступную резервную модель, Claude Code [переключает модели](/docs/ru/model-config#automatic-model-fallback) вместо показа этой ошибки.

На [Amazon Bedrock](/docs/ru/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) и [Microsoft Foundry](/docs/ru/microsoft-foundry) флаг кибербезопасности вместо этого производит сообщение [Usage Policy refusal](#usage-policy-refusal).

Сама защита находится на стороне сервера и предшествует v2.1.203; выпуски клиентов с тех пор изменили только формулировку сообщения.
От v2.1.203 до v2.1.218 сообщение читалось `<модель> has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center:` с последующей ссылкой на центр справки, и интерактивные сеансы добавляли `If you were not engaging in a cybersecurity topic, please send feedback via /feedback.`
До версии 2.1.203 оно читалось `<модель>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` с последующей ссылкой на форму исключения.

**Что делать:**

* Если ваша работа требует этого содержимого, подайте заявку на доступ через [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Если ваш запрос не был о теме кибербезопасности, запустите `/feedback`, чтобы сообщить о ложном срабатывании
* Чтобы продолжить работу в том же сеансе, нажмите Esc дважды или запустите `/rewind`, чтобы вернуться к контрольной точке перед ходом, который вызвал флаг, затем возьмите другой подход. Смотрите [Checkpointing](/docs/ru/checkpointing).

<h2 id="installation-errors">
  Ошибки установки
</h2>

Эти ошибки появляются при установке или обновлении Claude Code из [скрипта установки](/docs/ru/setup#install-claude-code), `claude install` или `claude update`. Для проблем с `command not found`, PATH, разрешениями и TLS во время установки см. [Устранение неполадок установки и входа](/docs/ru/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  Установка была прервана до завершения
</h3>

Скрипт установки сообщает, когда этап `claude install` завершается сигналом. На Linux код выхода 137 означает, что процесс получил SIGKILL, а на хосте с низким объёмом памяти это обычно означает, что ядро активировало средство защиты от нехватки памяти (OOM killer). Скрипт выводит это объяснение и завершается с кодом 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Для любого другого фатального сигнала и для кода выхода 137 на macOS скрипт выводит `Installation was killed before it could finish (exit code <N>)` с фактическим кодом выхода и опускает объяснение о нехватке памяти. Сообщение поступает из скрипта установки, который используют macOS и Linux, и он также охватывает установки внутри WSL; встроенные скрипты установки Windows никогда его не выводят. До версии 2.1.200 скрипт завершался только с простой строкой `Killed` оболочки.

**Что делать:**

* Остановите другие процессы, чтобы освободить память, затем повторно запустите установщик
* Добавьте пространство подкачки или перейдите на экземпляр большего размера. См. [Установка прервана на серверах Linux с низким объёмом памяти](/docs/ru/troubleshoot-install#install-killed-on-low-memory-linux-servers) для команд файла подкачки.

<h3 id="the-connection-dropped-while-downloading-the-update">
  Соединение разорвалось при загрузке обновления
</h3>

Соединение с сервером загрузки закрылось, пока `claude install`, `claude update` или [автоматический обновляющий модуль](/docs/ru/setup#auto-updates) загружал двоичный файл Claude Code, и повторные попытки не восстановили соединение. Claude Code повторяет загрузку, когда соединение разрывается, передача зависает или загруженный файл не проходит проверку контрольной суммы, всего до трёх попыток. Завершённая ошибка HTTP, такая как 404, не повторяется, потому что сервер уже ответил. До версии 2.1.202 одно разорванное соединение немедленно приводило к сбою загрузки с простой ошибкой `aborted` вместо повторной попытки.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

Текст в скобках указывает, какая попытка не удалась и какая была базовая ошибка сети. `claude update` предваряет сообщение с `Error: Failed to install native update` на stderr.

Загрузка, которая остаётся подключённой, но не завершается в течение 10 минут, завершается с ошибкой `Download timed out: exceeded the total deadline`. Claude Code не повторяет загрузку с истекшим временем ожидания, потому что соединение, которое слишком медленно для завершения в течение установленного срока, не завершится при немедленной повторной попытке. Приведённые ниже шаги применяются к обоим сообщениям.

Обычная причина — прокси или шлюз, который закрывает длительную передачу до её завершения. Двоичный файл Claude Code — это большая загрузка, поэтому ограничение соединения прокси, которое никогда не влияет на обычный трафик API, всё равно может его прервать.

**Что делать:**

* Запустите `claude update` снова. На в остальном здоровой сети загрузка обычно успешна при следующем запуске. Для сообщения об истечении времени ожидания запустите его снова из более быстрой или менее ограниченной сети.
* Если ваша сеть требует прокси, установите `HTTPS_PROXY` перед запуском установщика или `claude update`. См. [Проверка подключения к сети](/docs/ru/troubleshoot-install#check-network-connectivity).
* Если корпоративный прокси продолжает закрывать передачу, попросите вашу команду сети разрешить полную загрузку с `downloads.claude.ai`. См. [Требования к доступу в сеть](/docs/ru/network-config#network-access-requirements).
* Запустите `claude doctor` из вашей оболочки для диагностики установки

***

title: "Ошибки командной строки"
description: "Справочник по ошибкам командной строки Claude Code, включая конфликты флагов, проблемы с конфигурацией и неполадки с облачными сеансами."
-------------------------------------------------------------------------------------------------------------------------------------------------------

<h2 id="command-line-errors">
  Ошибки командной строки
</h2>

Эти ошибки поступают из командной строки `claude` и её подкоманд, из имени команды, которое вы отправляете в приглашение, и из команд, таких как `/security-review`, которые собирают контекст, запуская команды оболочки перед выполнением своего приглашения. Они также поступают из `/tui`, который перезапускает CLI.

<h3 id="conflict-between-bg-and-print">
  Конфликт между --bg и --print
</h3>

Это сообщение требует Claude Code v2.1.198 или позже. Вы объединили `--bg` с `-p` или `--print` в одном вызове `claude`. `--bg` запускает [фоновый сеанс](/docs/ru/agent-view#from-your-shell), к которому вы позже подключаетесь с помощью `claude agents`, в то время как `--print` работает [неинтерактивно](/docs/ru/headless) и никогда не запускает интерактивный сеанс, к которому подключается `claude agents`. До версии 2.1.198 эта комбинация молча создавала фоновое задание, к которому невозможно было подключиться.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**Что делать:**

* Удалите `-p` или `--print`. `--bg` принимает приглашение в качестве позиционального аргумента, поэтому `claude --bg "<task>"` — это полная команда. См. [Dispatch new agents from your shell](/docs/ru/agent-view#from-your-shell).
* Чтобы запустить приглашение неинтерактивно и вывести результат вместо создания фонового сеанса, удалите `--bg` и запустите `claude -p "<task>"`

<h3 id="invalid-agents-configuration">
  Неверная конфигурация --agents
</h3>

Значение, которое вы передали в `--agents`, недействительно, поэтому `claude` выходит с кодом 1 вместо запуска сеанса. Когда вы передаёте `--safe-mode`, `--resume` или `--continue`, или устанавливаете [`CLAUDE_CODE_SAFE_MODE`](/docs/ru/env-vars#variables), Claude Code не проверяет значение и запускает сеанс. До версии 2.1.242 Claude Code запускал сеанс в любом случае и пропускал определения, которые не мог загрузить.

```text theme={null}
Error: Invalid --agents configuration:
<what failed>
```

То, что следует после первой строки, зависит от того, как значение не прошло проверку. Claude Code выполняет эти проверки по порядку и останавливается на первой, которая не пройдена. Если ваше значение имеет два вида проблем, вы видите второе только после исправления первого:

1. Когда значение не анализируется как JSON, Claude Code выводит одну строку `invalid JSON:` с собственным сообщением парсера JSON
2. Когда оно анализируется, но определение агента не соответствует схеме для [подагентов, определённых в CLI](/docs/ru/sub-agents#choose-the-subagent-scope), Claude Code выводит одну строку на проблему
3. Когда имя агента начинается с `-`, Claude Code выводит `<name>: agent names must not start with '-'`

Когда строк проблем больше 20, Claude Code выводит первые 20 и заменяет остальные на `…and N more`.

**Что делать:**

* Исправьте каждую проблему, которую указывает сообщение, затем запустите команду снова. См. [поля, которые принимает подагент, определённый в CLI](/docs/ru/sub-agents#choose-the-subagent-scope).

<h3 id="cloud-sessions-cannot-be-created-from-a-restricted-session">
  Облачные сеансы не могут быть созданы из сеанса --restricted
</h3>

Когда вы запускаете сеанс с [`--restricted`](/docs/ru/cli-reference#cli-flags), Claude Code отказывается создавать [облачные сеансы](/docs/ru/claude-code-on-the-web#from-terminal-to-cloud) из него, потому что новый сеанс будет работать вне ограниченного процесса и не будет применять ограниченный режим. Claude Code отказывает на клиенте, перед контактом с сервером, поэтому облачный сеанс не создаётся:

```text theme={null}
Cloud sessions cannot be created from a --restricted session: they would not enforce it.
```

**Что делать:**

* Запустите задачу локально в ограниченном сеансе
* Если вы контролируете способ запуска сеанса, запустите новый сеанс `claude` без `--restricted` и создайте облачный сеанс оттуда

До версии 2.1.248 Claude Code не имел флага `--restricted`; более ранние версии отклоняют сам флаг с ошибкой неизвестного параметра.

<h3 id="cloud-sessions-are-disabled-by-your-organizations-policy">
  Облачные сеансы отключены политикой вашей организации
</h3>

Политика `allow_remote_sessions` вашей организации отключена, поэтому [облачные сеансы](/docs/ru/claude-code-on-the-web) и команды, которые их используют, недоступны:

```text theme={null}
Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.
```

Сообщение появляется, когда вы [создаёте облачный сеанс из терминала](/docs/ru/claude-code-on-the-web#from-terminal-to-cloud) и когда вы отправляете команду, которая требует облачные сеансы, такие как `/teleport`, `/remote-env` или `/web-setup`. До версии 2.1.268 отправка одной из этих команд возвращала [`Unknown command`](#unknown-command) вместо этого.

Это политика организации на стороне сервера, поэтому её нельзя переопределить из локальных параметров, переменных окружения или флагов CLI.

Если Claude Code ещё не загрузил политику вашей организации или не может её получить, эти команды отвечают `Couldn't verify your organization's policy for cloud sessions. Check your network connection, then restart Claude Code and try again.` вместо этого.

**Что делать:**

* Попросите [Owner](/docs/ru/server-managed-settings#access-control) в вашей организации включить облачные сеансы в параметрах администратора Claude Code на [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Если сообщение говорит, что оно не смогло проверить политику, проверьте подключение к сети, затем перезагрузите Claude Code и попробуйте снова

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  Значение --json-schema не является действительной JSON Schema
</h3>

Схема, которую вы передали в [`--json-schema`](/docs/ru/cli-reference#cli-flags) в [неинтерактивном режиме](/docs/ru/headless#get-structured-output), не прошла компиляцию JSON Schema, поэтому `claude` выходит с кодом 1 вместо запуска приглашения. До версии 2.1.205 недействительная схема выдавала неструктурированный вывод без ошибки, и любая схема, использующая ключевое слово `format`, рассматривалась как недействительная.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

Текст после второго двоеточия — это диагностика валидатора и указывает на ключевое слово или местоположение, которое не прошло проверку. Схемы, использующие ключевое слово `format`, такие как `"format": "email"`, действительны: Claude Code принимает `format` как аннотацию и не применяет её.

Claude Code выполняет две проверки перед компиляцией схемы: он отклоняет значение, которое не анализируется как JSON, с `Error: --json-schema is not valid JSON`, и действительный JSON, который не является объектом, с `Error: --json-schema must be a JSON object`.

**Что делать:**

* Исправьте часть схемы, которую указывает диагностика, затем повторно запустите команду
* Если диагностика — `schema too large`, уменьшите вложенность схемы и повторное использование `$ref`
* См. [Get structured output](/docs/ru/headless#get-structured-output) для рабочей схемы и команды

<h3 id="settings-file-exceeds-the-2mib-limit">
  Файл параметров превышает лимит 2MiB
</h3>

Файл, который вы передали в [`--settings`](/docs/ru/cli-reference#cli-flags), больше 2 MiB, поэтому `claude` выходит с кодом 1 при запуске вместо загрузки. Файл параметров — это небольшой документ JSON, поэтому файл такого размера обычно означает, что путь указывает на неправильный файл. До версии 2.1.214 Claude Code читал файл без проверки размера, и файл размером в несколько гигабайт или файл устройства, такой как `/dev/zero`, увеличивал память без ограничений.

```text theme={null}
Error: Settings file exceeds the 2MiB limit: /path/to/settings.json
```

Claude Code отклоняет путь `--settings`, который не является обычным файлом, таким же образом: устройство, FIFO или сокет выводит `Error: Cannot use settings file (Not a regular file (device, FIFO, or socket))` с последующим путём, а каталог выводит причину `EISDIR`.

**Что делать:**

* Укажите `--settings` на обычный файл JSON параметров размером менее 2 MiB. См. [Settings](/docs/ru/settings) для формата.

<h3 id="the-current-directory-no-longer-exists">
  Текущий каталог больше не существует
</h3>

Вы запустили `claude` из каталога, который был удалён или перемещён после того, как ваша оболочка вошла в него, например рабочее дерево или временный каталог, удалённый другой оболочкой. Claude Code не может прочитать свой рабочий каталог, поэтому он выходит с кодом 1 перед запуском сеанса, как в интерактивном, так и в [неинтерактивном](/docs/ru/headless) режиме. До версии 2.1.239 Claude Code падал с минифицированным исходным кодом пакета и необработанным стеком `ENOENT ... uv_cwd` на stderr вместо этого сообщения.

```text theme={null}
The current directory no longer exists (it was deleted or moved). Start Claude Code from an existing directory.
error: The current working directory was deleted, so that command didn't work. Please cd into a different directory and try again.
```

Причина и исправление одинаковы для обеих форм.

Когда Claude Code не может прочитать рабочий каталог по другой причине, такой как изменение разрешений, сообщение указывает код ошибки вместо этого: `Can't read the current directory (EACCES). Start Claude Code from a different directory.`

На macOS `EPERM` для каталога в `~/Desktop`, `~/Documents`, `~/Downloads` или iCloud Drive обычно означает, что macOS блокирует доступ вашего приложения терминала к этой папке. Другие команды, которые читают эту папку, не работают так же: `ls` там выводит `Operation not permitted`, даже с `sudo`.

**Что делать:**

* Перейдите в каталог, который существует, такой как ваш домашний или проектный каталог, затем запустите `claude` снова
* Если каталог был пересоздан по тому же пути, ваша оболочка всё ещё содержит удалённый. Запустите `cd "$PWD"` или покиньте и повторно войдите в каталог, затем запустите `claude` снова
* Для `EPERM` на macOS выйдите из приложения терминала с помощью Cmd+Q, откройте его снова, вернитесь в эту папку и запустите `claude`. Если `ls` в этой папке всё ещё не работает, откройте **System Settings > Privacy & Security > Files and Folders**, включите папку для вашего приложения терминала, затем повторно откройте терминал

<h3 id="temp-directory-refused-or-cannot-be-created">
  Временный каталог отклонён или не может быть создан
</h3>

На macOS и Linux Claude Code создаёт приватный временный каталог при запуске, `claude-<uid>` под системным временным каталогом или переопределением [`CLAUDE_CODE_TMPDIR`](/docs/ru/env-vars). Когда каталог не может быть создан или запись, уже находящаяся по этому пути, не проходит проверки безопасности, Claude Code выводит ошибку на stderr и выходит с кодом 1 вместо запуска сеанса:

```text wrap theme={null}
ENOSPC: no space left on device, mkdir '/tmp/claude-501'

Temp directory /tmp/claude-501 is not a directory (may be an attacker-planted symlink). Refusing to use it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is owned by uid 502, expected 501. Refusing to use it — another user may have pre-created it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is not readable (its mode may have been altered, or a path component denies search). Refusing to use it — restore its permissions (chmod 0700) or remove it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.
```

**Что делать:**

* Для `ENOSPC`, освободите место на диске на томе, который содержит временный каталог
* Для форм `Refusing to use it`, удалите саму названную запись, не то, на что указывает ссылка, и запустите Claude Code снова; для формы `owned by uid`, только администратор или этот пользователь могут удалить её
* Для `is not readable`, запустите `chmod 0700` на названном каталоге или удалите его и запустите снова
* В любом из этих случаев установите [`CLAUDE_CODE_TMPDIR`](/docs/ru/env-vars) на каталог, который вы контролируете, и запустите Claude Code снова, оставляя отклонённый путь в покое

<h3 id="directory-couldnt-be-resolved-to-a-real-location">
  Каталог не удалось разрешить в реальное местоположение
</h3>

Вы запустили `/add-dir` для подкаталога вашего рабочего каталога, и Claude Code не смог разрешить каталог в его реальное местоположение.

У вас уже есть доступ к файлам подкаталога рабочего каталога, поэтому `/add-dir` только загружает его skills, команды и агентов. Перед загрузкой Claude Code проверяет, что реальное местоположение каталога, с разрешёнными всеми символическими ссылками, находится внутри рабочего каталога. Когда Claude Code не может разрешить это местоположение, он ничего не загружает и показывает это сообщение:

```text theme={null}
packages/app couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded. Check that it is a directory inside the working directory and try again.
```

**Что делать:**

* Проверьте, что путь указывает на реальный каталог внутри рабочего каталога, затем запустите `/add-dir` снова
* Сообщение не изменяет ваш доступ к файлам; оно только сообщает, что содержимое `.claude/` каталога не было загружено

До версии 2.1.261 это сообщение также появлялось для каждого `/add-dir <subdirectory>`, когда рабочий каталог находился на автомонтировании `/net/<host>`, где Claude Code по дизайну отказывается разрешать пути; каталог был в порядке и повторная попытка не могла помочь.

<h3 id="workspace-not-trusted-when-starting-remote-control">
  Рабочее пространство не доверено при запуске Remote Control
</h3>

Вы запустили сервер [Remote Control](/docs/ru/remote-control) в режиме с помощью `claude remote-control` или его псевдонима `claude rc` в каталоге, которому вы не доверяете. Команда не показывает диалог доверия рабочего пространства сама по себе, поэтому она выходит с кодом 1 и указывает исправление:

```text theme={null}
Error: Workspace not trusted. Please run `claude` in /Users/you/project first to review and accept the workspace trust dialog.
```

В вашем домашнем каталоге сообщение отличается, потому что диалог доверия рабочего пространства никогда не сохраняет доверие для домашнего каталога, поэтому принятие его там не может удовлетворить эту проверку. До версии 2.1.214 домашний каталог показывал сообщение выше, совет которого не может быть успешным там.

```text theme={null}
Error: Workspace not trusted. /Users/you is your home directory, and for security home-directory trust is never saved, so running `claude` here first won't help. Run `claude rc` from a project directory instead (run `claude` there once to accept the trust dialog).
```

**Что делать:**

* Запустите `claude` в каталоге, примите [диалог доверия рабочего пространства](/docs/ru/permissions#project-allow-rules-and-workspace-trust), затем запустите `claude remote-control` снова
* В вашем домашнем каталоге перейдите в проектный каталог и запустите Remote Control там

<h3 id="not-carried-over-to-the-sessions-remote-control-starts">
  Не перенесено в сеансы, которые запускает Remote Control
</h3>

Вы запустили [Remote Control](/docs/ru/remote-control) с глобальным флагом `claude` перед глаголом `remote-control`, флагом, который ограничивал бы или конфигурировал сеансы, которые запускает Remote Control, такие как `--settings`, `--setting-sources`, `--permission-mode`, `--disallowed-tools` или `--mcp-config`. Флаг, размещённый перед глаголом, никогда не достигает этих сеансов. Claude Code отказывает запускаться вместо этого, указывая флаг:

```text theme={null}
Error: `--settings` before `remote-control` is not carried over to the sessions Remote Control starts, so Remote Control refuses to start rather than drop it — remove it, and give Remote Control's own options after the verb (see `claude remote-control --help`).
```

Claude Code не отказывает глобальные флаги, которые безопасно удалить, такие как `--verbose`, `--model` или внедрённый оболочкой `--session-id` или `--plugin-dir`: он их игнорирует и Remote Control запускается.

Claude Code также отказывает запускаться для глобального флага, который он ещё не признал безопасным, поэтому флаг, добавленный в более новом выпуске, может появиться в этом сообщении до более позднего выпуска, который его отметит как безопасный.

**Что делать:**

* Удалите флаг перед глаголом и передайте [собственные параметры Remote Control](/docs/ru/remote-control#start-a-remote-control-session) после него; `claude remote-control --help` их перечисляет
* Когда отклонённый флаг — `--permission-mode`, запустите `claude remote-control --permission-mode <mode>` для установки режима разрешений для сеансов, которые запускает Remote Control

До версии 2.1.248 `claude remote-control` не принимал свои флаги, когда глобальный флаг шёл первым, и команда не работала с ошибкой `unknown option`.

<h3 id="claude-import-is-not-yet-available-in-this-build">
  claude import ещё не доступен в этой сборке
</h3>

Вы запустили [`claude import`](/docs/ru/cli-reference#cli-commands), и Claude Code обнаружил, что поток импорта отключен, поэтому команда выходит с кодом 1 вместо запуска импорта. До версии 2.1.222 сборка с отключённым потоком импорта рассматривала `import` как приглашение и запускала интерактивный сеанс вместо вывода этого сообщения.

```text theme={null}
`claude import` is not yet available in this build. Run `claude` and use /mcp or edit ~/.claude/settings.json directly.
```

Claude Code включает `claude import` через флаг функции, который он получает от Anthropic и кэширует на диске. Это сообщение означает, что кэшированное значение отключено. Причина обычно одна из следующих:

* Вы не запустили сеанс с момента установки, поэтому Claude Code ещё не получил флаг. Первый `claude import` может вывести это даже когда функция доступна вам.
* Вы используете Claude Code через Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или Claude Platform на AWS, или через [шлюз приложений Claude](/docs/ru/claude-apps-gateway#availability-and-limitations). Claude Code не получает флаги функций в этих сеансах, поэтому `claude import` остаётся недоступным.
* Вы установили `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `DISABLE_GROWTHBOOK` или [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ru/env-vars), которые отключают получение флагов функций, поэтому `claude import` остаётся недоступным.

**Что делать:**

* На свежей установке запустите `claude`, дождитесь загрузки сеанса, выйдите и запустите `claude import` снова
* Где получение флагов функций остаётся отключённым, установите конфигурацию самостоятельно: добавьте серверы MCP с помощью [`claude mcp add`](/docs/ru/mcp#installing-mcp-servers) и создайте [`CLAUDE.md` файлы](/docs/ru/memory#how-claude-md-files-load), [skills и команды](/docs/ru/skills#where-skills-live) и [подагентов](/docs/ru/sub-agents#choose-the-subagent-scope), которые вы хотите перенести. Сообщение также указывает `~/.claude/settings.json`. Из конфигурации, которую переносит `claude import`, этот файл содержит только [режим разрешений](/docs/ru/settings-reference#permission-settings); Claude Code не читает серверы MCP из него.

<h3 id="could-not-read-claude-code-config">
  Не удалось прочитать конфигурацию Claude Code
</h3>

Вы запустили [`claude import`](/docs/ru/cli-reference#cli-commands), пока Claude Code не мог анализировать `~/.claude.json`, файл, где он хранит вашу учётную запись и состояние для каждого проекта. Подкоманда читает этот файл для проверки доступности, но не показывает диалог восстановления, который показывает интерактивный сеанс, поэтому она выходит с кодом 1. До версии 2.1.222 `claude import` с нечитаемым файлом конфигурации запускал интерактивный сеанс, диалог восстановления которого обрабатывал файл.

```text theme={null}
Could not read Claude Code config — run `claude` with no arguments to recover it.
```

**Что делать:**

* Запустите `claude` без аргументов. Claude Code обнаруживает недействительный файл и предлагает его сбросить. Затем запустите `claude import` снова.
* Чтобы сохранить ручные правки, которые вы сделали, исправьте синтаксис JSON в `~/.claude.json` в редакторе вместо этого, затем повторно запустите `claude import`

<h3 id="could-not-import-a-server-from-claude-desktop">
  Не удалось импортировать сервер из Claude Desktop
</h3>

Claude Code не смог добавить один из серверов, которые вы выбрали в `claude mcp add-from-claude-desktop`. Команда всё ещё импортирует другие выбранные серверы и выводит одну строку на сервер, который она не смогла добавить. До версии 2.1.205 первый сервер, который не прошёл, останавливал импорт и ни один из выбранных серверов не был добавлен.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

Текст после имени сервера — это причина. Наиболее распространённая — проверка имени: Claude Desktop позволяет символы в именах серверов, такие как пробелы и точки, которые `claude mcp` ограничивает буквами, цифрами, дефисами и подчёркиваниями. Другие причины включают конфигурацию сервера, которая не проходит валидацию, и сервер, заблокированный [политикой MCP](/docs/ru/managed-mcp) вашей организации.

**Что делать:**

* Переименуйте сервер в `claude_desktop_config.json`, чтобы использовать только буквы, цифры, дефисы и подчёркивания, затем запустите `claude mcp add-from-claude-desktop` снова
* Добавьте этот сервер напрямую с помощью `claude mcp add` или `claude mcp add-json` под действительным именем. См. [Import MCP servers from Claude Desktop](/docs/ru/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="cannot-add-mcp-server-to-the-managed-scope">
  Не удалось добавить сервер MCP в управляемую область
</h3>

Вы запустили `claude mcp add` или `claude mcp add-json` с `--scope managed`. Эта область содержит серверы, которые ваша организация предоставляет через управляемый параметр [`managedMcpServers`](/docs/ru/settings-reference#managedmcpservers). Claude Code читает их только из управляемых параметров, поэтому команда не может записать сервер в эту область.

```text theme={null}
Cannot add MCP server to scope: managed
```

**Что делать:**

* Добавьте сервер в область, в которую вы можете писать: `local`, `user` или `project`. Без `--scope` команда использует `local`. См. [MCP installation scopes](/docs/ru/mcp#mcp-installation-scopes)
* Чтобы предоставить сервер каждому пользователю в вашей организации, добавьте его в [`managedMcpServers`](/docs/ru/settings-reference#managedmcpservers) в управляемых параметрах, которые вы развёртываете

<h3 id="cant-read-mcp-json">
  Не удалось прочитать .mcp.json
</h3>

Команда, которая читает проект [`.mcp.json`](/docs/ru/mcp#project-scope), такая как `claude mcp add` или `claude mcp add-json` с `--scope project` или `claude mcp remove`, обнаружила, что файл в вашем текущем каталоге не является обычным файлом или больше 2 MiB, поэтому она выходит с этой ошибкой вместо чтения файла.

```text theme={null}
Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes. Fix or remove it, then run the command again.
```

До версии 2.1.257 FIFO в `.mcp.json` оставлял команду ждущей вечно без вывода, и символическая ссылка на файл устройства, такой как `/dev/zero`, увеличивала память до тех пор, пока процесс не был убит.

**Что делать:**

* Проверьте, что находится в `.mcp.json` в вашем текущем каталоге. Замените его обычным файлом JSON в [формате проектной области](/docs/ru/mcp#project-scope) или удалите его, затем запустите команду снова.

<h3 id="anthropic-hosted-and-doesnt-support-local-oauth">
  Сервер размещён Anthropic и не поддерживает локальный OAuth
</h3>

Вы запустили вход для сервера MCP, чей URL указывает на размещённый Anthropic хост соединителя, который аутентифицируется через поставщика удостоверений третьей стороны. Эти хосты включают `microsoft365.mcp.claude.com`, `gmail.mcp.claude.com` и `gcal.mcp.claude.com`. Claude Code отказывает запускать свой локальный поток OAuth для этих хостов как из панели `/mcp`, так и из `claude mcp login`, потому что [их вход работает только через claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai).

```text theme={null}
"gmail" is Anthropic-hosted and doesn't support local OAuth. Connect it via Settings → Connectors on claude.ai (requires `claude login`), then it'll be available here automatically.
```

Claude Code сопоставляет эти хосты по URL, поэтому сообщение появляется, когда сервер, который вы добавили с помощью `claude mcp add` или в `.mcp.json`, указывает на один из них.

**Что делать:**

* Удалите вашу запись с помощью `claude mcp remove <name>`, чтобы она не могла скрыть соединитель claude.ai по тому же URL
* После удаления подключите сервис на [claude.ai/customize/connectors](https://claude.ai/customize/connectors), пока вы вошли в учётную запись, которую вы используете в Claude Code. После подключения [соединитель появляется в Claude Code автоматически](/docs/ru/mcp#use-mcp-servers-from-claude-ai), если ваш активный метод аутентификации — это вход по подписке claude.ai

<h3 id="server-rejected-the-authorization-header-minted-by-the-configured-headershelper">
  Сервер отклонил заголовок Authorization, созданный настроенным headersHelper
</h3>

Сервер MCP, чей [`headersHelper`](/docs/ru/mcp#use-dynamic-headers-for-custom-authentication) предоставляет заголовок `Authorization`, ответил на соединение с HTTP 401 или 403, поэтому Claude Code сообщает о соединении как о неудачном. Потому что помощник предоставляет заголовок `Authorization`, Claude Code [не переходит на OAuth](/docs/ru/mcp#authenticate-with-remote-mcp-servers) для сервера:

```text theme={null}
Server rejected the Authorization header minted by the configured headersHelper (HTTP 401). Check that the helper command returns a valid credential for this MCP endpoint — OAuth fallback is disabled when the helper supplies Authorization.
```

Claude Code повторно запускает помощника при каждой попытке соединения, поэтому повторная попытка после временного отклонения, такого как гонка ротации токена, может быть успешной с новым учётным данным.

**Что делать:**

* Запустите команду `headersHelper` самостоятельно так, как её запускает Claude Code: из [каталога, в котором Claude Code её запускает](/docs/ru/mcp#where-the-helper-runs), с [переменными окружения, которые Claude Code для неё устанавливает](/docs/ru/mcp#use-dynamic-headers-for-custom-authentication), и без [переменных учётных данных, которые Claude Code удаляет](/docs/ru/mcp#which-variables-a-helper-can-read) для сервера из проектного `.mcp.json`, плагина или файла проектного агента. Проверьте, что она выводит значение `Authorization`, которое принимает конечная точка сервера
* После исправления помощника или его источника учётных данных выберите сервер в `/mcp` и выберите **Reconnect**

До версии 2.1.248 Claude Code запускал обнаружение OAuth для сервера, чей помощник предоставлял заголовок `Authorization`. Это обнаружение могло не пройти с `Incompatible auth server: does not support dynamic client registration` вместо сообщения об отклонённом учётном данном.

<h3 id="mcp-permission-prompt-tool-not-found">
  Инструмент запроса разрешения MCP не найден
</h3>

Инструмент, который вы передали в [`--permission-prompt-tool`](/docs/ru/cli-reference#cli-flags), не был среди подключённых инструментов MCP, когда запуск впервые нуждался в решении разрешения, либо потому, что его сервер никогда не подключался, либо потому, что ни один подключённый сервер не предоставляет инструмент с этим именем. Claude Code всё ещё отправляет ваше приглашение: [неинтерактивный](/docs/ru/headless) запуск выходит с этой ошибкой и кодом выхода 1 при первом вызове инструмента, который требует одобрения, поэтому он не выдаёт ответ, даже хотя запрос был сделан. Перед первым приглашением Claude Code ждёт до установленного для каждого сервера времени ожидания соединения в 30 секунд, установленного [`MCP_TIMEOUT`](/docs/ru/env-vars), чтобы этот сервер подключился. До версии 2.1.206 запуск не ждал завершения соединения сервера, поэтому медленно запускающийся, но здоровый сервер также выдавал эту ошибку.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

Список после `Available MCP tools:` указывает инструменты MCP, которые были подключены, когда ожидание закончилось.

**Что делать:**

* Проверьте, что сервер запускается и остаётся подключённым: запустите `claude mcp list` в том же каталоге и подтвердите, что сервер указан как подключённый
* Подтвердите, что имя инструмента соответствует имени `mcp__<server>__<tool>`, которое предоставляет сервер
* Если серверу требуется больше 30 секунд для запуска, увеличьте [`MCP_TIMEOUT`](/docs/ru/env-vars)

<h3 id="oauth-callback-port-is-already-in-use">
  Порт обратного вызова OAuth уже используется
</h3>

Когда вы входите на удалённый сервер MCP с помощью OAuth, Claude Code запускает локальный слушатель для получения обратного вызова входа. Если порт, который нужен этому слушателю, занят другим процессом, вход не удаётся с этим сообщением. Это в основном происходит с [фиксированным портом обратного вызова](/docs/ru/mcp#use-a-fixed-oauth-callback-port), установленным через переменную [`MCP_OAUTH_CALLBACK_PORT`](/docs/ru/env-vars) или `--callback-port`, так как без неё Claude Code выбирает доступный порт.

```text theme={null}
OAuth callback port <port> is already in use — another process may be holding it. Run `lsof -ti:<port> -sTCP:LISTEN` to find it.
```

На Windows предлагаемая команда — `netstat -ano | findstr :<port>` вместо этого.

**Что делать:**

* Запустите команду из сообщения, чтобы найти процесс, занимающий порт, и остановите его или дождитесь его завершения
* Если другой программе постоянно нужен этот порт, зарегистрируйте другой URI перенаправления на сервере и установите его порт с помощью `MCP_OAUTH_CALLBACK_PORT` или `--callback-port`, в зависимости от того, что вы используете
* Затем запустите вход снова, например выбрав сервер в `/mcp`

<h3 id="no-available-ports-for-oauth-redirect">
  Нет доступных портов для перенаправления OAuth
</h3>

Когда вы входите на удалённый сервер MCP с помощью [OAuth](/docs/ru/mcp#authenticate-with-remote-mcp-servers), Claude Code запускает локальный слушатель для получения обратного вызова входа. Вход не удаётся с этим сообщением, когда Claude Code не может привязать локальный порт для него. Что-то на машине препятствует прослушиванию на `127.0.0.1`, например программное обеспечение безопасности или политика песочницы, которая запрещает локальные слушатели.

```text theme={null}
No available ports for OAuth redirect
```

До версии 2.1.268 Claude Code не переходил на порт, назначенный операционной системой, поэтому сообщение также появлялось, когда только его самостоятельно выбранные порты не могли быть привязаны. Это может произойти на хостах Windows, где Hyper-V резервирует диапазоны портов, которые охватывают порты, которые Claude Code выбирает.

**Что делать:**

* Проверьте, блокирует ли программное обеспечение безопасности или политика песочницы процессы от прослушивания на `127.0.0.1`, и разрешите Claude Code привязать локальный порт
* Затем запустите вход снова, например выбрав сервер в `/mcp`

<h3 id="security-review-fails-without-origin-head">
  /security-review не работает без origin/HEAD
</h3>

[`/security-review`](/docs/ru/commands#all-commands) создаёт контекст своего обзора, сравнивая вашу ветку с `origin/HEAD`, локальной ссылкой, которая записывает, какая ветка является стандартной на вашем удалённом `origin`. Когда эта ссылка не существует, команды git, которые собирают diff, не работают и обзор останавливается перед началом.

```text theme={null}
Error: Shell command failed for pattern "!`git diff --name-only origin/HEAD...`": [stderr]
fatal: ambiguous argument 'origin/HEAD...': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
```

Сообщение может цитировать `git log` или другой `git diff` вместо этого. Git создаёт `origin/HEAD` только когда удалённый объявляет стандартную ветку и ваша спецификация получения её охватывает, что полный `git clone` удалённого с коммитами делает. Ссылка отсутствует в этих установках:

* Получение одной ветки или получение CI, которое получает слишком узкую спецификацию
* Удалённый, чей HEAD на стороне сервера указывает на ветку, которую никто не отправил
* Репозиторий без удалённого `origin` или того, из которого вы никогда не получали

Claude Code показывает ту же ошибку для любого skill, который [внедряет динамический контекст](/docs/ru/skills#when-an-injected-command-fails), и неудачная внедрённая команда прерывает вызов этого skill. Две соседние строки срабатывают перед запуском команды вообще:

* `Shell command permission check failed for pattern "..."`: проверка разрешения команды не разрешила её. [Permission checks on injected commands](/docs/ru/skills#permission-checks-on-injected-commands) охватывает, какие результаты прерывают в каждом режиме разрешений и как предварительно одобрить команду с помощью `allowed-tools`
* ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``: frontmatter skill требует bash на машине без него. Установите Git для Windows или измените frontmatter на `shell: powershell`. См. [How injected commands run](/docs/ru/skills#how-injected-commands-run)

**Что делать:**

* Создайте ссылку, назвав стандартную ветку вашего удалённого: `git remote set-head origin <default-branch>`. Это работает всякий раз, когда существует локальная отслеживаемая ссылка `origin/<default-branch>`. Если её нет, как в клонах одной ветки, сначала получите ветку: запустите `git remote set-branches --add origin <branch>`, затем `git fetch origin`, затем повторно запустите команду set-head. Повторно запустите `/security-review`.
* Если вы предпочитаете не называть ветку, запустите `git fetch origin` и затем `git remote set-head origin --auto`, который спрашивает удалённый, какая ветка является его стандартной. Это не работает с `error: Cannot determine remote HEAD`, когда удалённый не объявляет стандартную ветку, потому что он пуст или его HEAD указывает на ветку, которую никто не отправил; вместо этого назовите ветку явно. Это не работает с `error: Not a valid ref`, когда ваш клон не получает эту ветку; сначала расширьте спецификацию, как выше.
* Если репозиторий не имеет удалённого, добавьте один с помощью `git remote add origin <url>` и получите перед созданием ссылки. Если удалённый пуст, сначала отправьте вашу ветку с помощью `git push -u origin HEAD` и назовите эту ветку в команде set-head; `origin/HEAD` затем указывает на ветку, которую вы только что отправили, поэтому `/security-review` видит пустой diff, пока ветка не отличается от неё.

<h3 id="input-must-be-provided-when-using-print">
  Входные данные должны быть предоставлены при использовании --print
</h3>

Bare `claude` нужен stdout, чтобы быть терминалом для запуска интерактивного UI. Когда stdout перенаправлен или консоль не является реальным терминалом, такой как PowerShell ISE и некоторые панели вывода IDE, `claude` вместо этого работает [неинтерактивно](/docs/ru/headless). Это тот же режим, что и `claude -p`, который требует приглашение, поэтому сообщение указывает `--print` даже когда вы не передали флаг. Передача `-p`/`--print` без приглашения и ничего не передано на stdin выдаёт ту же ошибку везде.

```text theme={null}
Error: Input must be provided either through stdin or as a prompt argument when using --print
```

**Что делать:**

* Для интерактивного использования запустите `claude` в реальном терминале: Windows Terminal или консоль PowerShell вместо ISE и интегрированный терминал вашего IDE вместо панели вывода
* Для одноразового использования передайте приглашение: `claude -p "your question"` или передайте его с помощью `echo "your question" | claude -p`

<h3 id="input-contained-only-whitespace">
  Входные данные содержали только пробелы
</h3>

В [неинтерактивном режиме](/docs/ru/headless) Claude Code отказывает приглашение, состоящее полностью из пробелов, табуляций или новых строк вместо его отправки, потому что API отклоняет сообщения без видимого текста. Какое сообщение вы видите, зависит от того, откуда пришло пустое приглашение:

* **Аргумент приглашения или передача stdin для `claude -p`**: `claude` выходит с `Error: Input contained only whitespace. Provide a prompt with text through stdin or as a prompt argument when using --print`
* **Сообщение, отправленное работающему сеансу `--input-format stream-json` или [Agent SDK](/docs/ru/agent-sdk/overview)**: Claude Code заканчивает ход без вызова модели и сеанс остаётся пригодным. Отказ приходит как информационное сообщение и как текст результата хода: `Blank prompt — the message was only whitespace, so nothing was sent to the model.`

До версии 2.1.229 Claude Code отправлял сообщение только с пробелами в API, который отклонял запрос с ошибкой 400.

**Что делать:**

* Включите видимый текст в приглашение. Если скрипт создаёт приглашение из переменной или файла, проверьте, что источник не пуст перед вызовом Claude Code.

<h3 id="stream-json-input-carried-over-256m-characters-with-no-newline">
  stream-json входные данные содержали более 256M символов без новой строки
</h3>

Ваша программа отправила более 268,435,456 символов на stdin без новой строки в запуск `claude -p --input-format stream-json`, поэтому Claude Code выводит эту ошибку на stderr и выходит с кодом 1 вместо буферизации дополнительного входа. Сообщение указывает этот бюджет как `256M`. До версии 2.1.257 Claude Code буферизировал такой вход без ограничений, увеличивая память до тех пор, пока процесс не падал или не был убит.

```text theme={null}
Error: stream-json input carried over 256M characters with no newline. Each stream-json message must be a single newline-terminated JSON line: either the producer is not newline-terminating its messages, or one message exceeded this budget.
```

Входные данные такой длины без новой строки обычно означают, что производитель вообще не является производителем stream-json, такой как двоичный файл или простой вывод журнала, случайно передаваемый. Одно сообщение сверх бюджета не проходит ту же проверку.

**Что делать:**

* Проверьте, что передаётся на stdin. С [`--input-format stream-json`](/docs/ru/cli-reference#cli-flags) каждое сообщение должно быть одной строкой JSON, завершённой новой строкой
* Чтобы вместо этого отправить простой текст, удалите `--input-format stream-json`; `claude -p` по умолчанию читает простой текстовый запрос из stdin

<h3 id="unknown-command">
  Неизвестная команда
</h3>

В интерактивном сеансе терминала вы отправили имя `/`, которое не соответствует никакой команде в этом сеансе, поэтому Claude Code сообщает имя вместо запуска чего-либо:

```text theme={null}
Unknown command: /hepl. Did you mean /help?
```

Claude Code предлагает ближайшее имя команды или псевдоним, который меню перечисляет в этом сеансе. Когда ничего не близко, сообщение заканчивается после имени. Причина обычно одна из следующих:

* Опечатка, такая как `/hepl` для `/help`. [How the command menu matches what you type](/docs/ru/commands#how-the-command-menu-matches-what-you-type) охватывает выбор близкого совпадения перед отправкой
* Команда, которая существует, но недоступна в этом сеансе, потому что требование не выполнено, такое как ваша платформа, план или метод аутентификации. Записи устранения неполадок для [`/web-setup`](/docs/ru/web-quickstart#web-setup-shows-no-commands-match-or-unknown-command) и [`/schedule`](/docs/ru/routines#schedule-returns-unknown-command) проходят через два распространённых случая. Некоторые команды отвечают своим собственным сообщением, когда политика вашей организации их отключает, такие как [`Cloud sessions are disabled by your organization's policy`](#cloud-sessions-are-disabled-by-your-organizations-policy)
* Команда из [плагина](/docs/ru/plugins/overview) или [сервера MCP](/docs/ru/mcp#use-mcp-prompts-as-commands), который не установлен или не подключен в этом сеансе

Claude Code отвечает на несовпадающее имя `/` этим способом только в интерактивном сеансе терминала. Во всех остальных сеансах он отправляет приглашение Claude как обычное сообщение вместо этого, с примечанием, что команда не запустилась и список команд, которые Claude может запустить в сеансе. Эти сеансы включают:

* Запуски `-p`
* Приложения [Agent SDK](/docs/ru/agent-sdk/overview)
* Вкладка Code [Desktop app](/docs/ru/desktop)
* Панель чата [VS Code extension](/docs/ru/vs-code)
* [Облачные сеансы](/docs/ru/claude-code-on-the-web) и [рутины](/docs/ru/routines)

Для встроенной команды, которая не может работать в одном из этих сеансов, Claude Code всё ещё отвечает, что команда недоступна вместо отправки её Claude. До версии 2.1.274 только облачные сеансы и рутины отправляли несовпадающее имя Claude. До версии 2.1.273 они также отвечали `Unknown command`.

Claude Code не рассматривает каждое приглашение, которое начинается с `/`, как команду. Он отправляет приглашение Claude как обычное сообщение, когда первое слово после `/` начинается с пунктуации, такой как `/--`, который открывает комментарий документа Lean, или является путём, такой как `/var/log/syslog`.

До версии 2.1.236, если вы нажали `Enter`, пока меню команд перечисляло близкое совпадение для имени, которое вы ввели, Claude Code запускал это совпадение, поэтому опечатка, такая как `/hepl`, запускала `/help` вместо выдачи этого сообщения.

**Что делать:**

* Запустите предложенное имя или введите `/` с последующей частью имени, чтобы увидеть, что доступно в этом сеансе
* Если Claude Code сообщает задокументированную команду как неизвестную, проверьте её строку в [справочнике команд](/docs/ru/commands) для требования, которое она указывает

<h3 id="diff-is-too-large-for-ultrareview">
  Diff слишком большой для ultrareview
</h3>

Diff между вашей веткой и базовой веткой, включая незафиксированные и поставленные в очередь изменения, превышает ограничения размера для [ultrareview](/docs/ru/ultrareview), поэтому `/code-review ultra` и подкоманда `claude ultrareview` отказывают обзор перед запуском облачного сеанса. Отклонённый обзор не использует бесплатный запуск и не выставляет счёт за использование кредитов. Сообщение указывает действующие ограничения, размер вашего diff и файлы, которые вносят наибольшее количество изменённых строк. До версии 2.1.216 сообщение показывало только необработанную статистику diff.

```text theme={null}
Diff is too large for ultrareview: 812 files, 96,410 lines changed (limits: 500 files, 8,000 lines). Largest files: package-lock.json (41,904 lines), dist/bundle.js (18,210 lines), src/generated/api.ts (9,876 lines). Pass a closer base branch (`/code-review ultra <branch>`) to narrow the scope, or split the change.
```

Обзор запроса на слияние применяет те же ограничения; эта форма сообщения начинается с `PR #<N> is too large for ultrareview` и указывает количество файлов и строк PR.

**Что делать:**

* Передайте базовую ветку ближе к вашей работе, такую как `/code-review ultra develop`, чтобы обзор охватывал только diff против этой ветки
* Разделите изменение на меньшие ветки и обзорьте каждую. Файлы, которые сообщение указывает, вносят наибольшее количество изменённых строк, поэтому начните с перемещения их в свою собственную ветку.

<h3 id="could-not-find-merge-base-with-the-base-branch">
  Не удалось найти merge-base с базовой веткой
</h3>

`/code-review ultra` и подкоманда `claude ultrareview` обзорят diff между вашей веткой и базовой веткой, что требует коммита, который они оба разделяют. Когда `git merge-base` не находит ни одного, Claude Code отказывает обзор перед запуском облачного сеанса. На клоне, который Claude Code может проверить как полный, с по крайней мере одной веткой, он переходит на [обзор каждого отслеживаемого файла](/docs/ru/ultrareview#diff-limits-and-fallbacks) вместо отказа. Вы видите этот отказ, когда базовую ветку вообще не удалось найти, когда Claude Code не может проверить, что ваш клон полный, или в редком репозитории, где diff всего дерева невозможен, такой как формат объекта SHA-256.

```text theme={null}
Could not find merge-base with main. Pass the base branch explicitly (e.g. `/code-review ultra develop`) or make sure you're in a git repo with a main branch.
```

Подсказка после первого предложения зависит от того, что Claude Code наблюдал:

* **Вы не передали базовую ветку**: Claude Code сравнил с стандартной веткой репозитория и предлагает передать вашу базу явно, как в примере выше
* **Вы передали базовую ветку, которая уже была в вашем клоне**: подсказка читает ``Make sure <branch> exists locally or on origin (try `git fetch origin <branch>`)``
* **Вы передали базовую ветку, которой не было в вашем клоне**: Claude Code получил её из origin перед сравнением. Подсказка читает ``<branch> was fetched from origin but shares no history with HEAD. If another branch is your real base, pass it explicitly (`/code-review ultra <branch>`)``; когда Claude Code не может сказать, полный ли ваш клон, он предлагает `git fetch --unshallow origin` вместо этого. До версии 2.1.221 подсказка предлагала `git fetch --unshallow origin` для каждой полученной базовой ветки, и на полном клоне эта команда не работает с `fatal: --unshallow on a complete repository does not make sense`.

**Что делать:**

* Если другая ветка является вашей реальной базой, передайте её явно: `/code-review ultra <branch>`
* Если ваш клон может не иметь полной истории, запустите `git fetch --unshallow origin` и повторно запустите обзор

<h3 id="your-checkout-has-no-branches">
  Ваш checkout не имеет веток
</h3>

Checkout может иметь коммиты, но не иметь веток: если вы запустите `git init`, затем `git fetch <url>` и `git checkout FETCH_HEAD`, вы получите отсоединённый HEAD без ссылок. Claude Code упаковывает ваш репозиторий как git bundle для загрузки его для [ultrareview](/docs/ru/ultrareview), и он не может упаковать репозиторий, который не имеет веток или других ссылок, поэтому `/code-review ultra` и подкоманда `claude ultrareview` отказывают обзор перед запуском облачного сеанса.

```text theme={null}
Your checkout has no branches (detached HEAD only), which cloud review can't bundle. Create one first — `git checkout -b <name>` — then rerun /code-review ultra.
```

До версии 2.1.221 Claude Code пытался обзорить каждый отслеживаемый файл в этом checkout, и загрузка не работала.

**Что делать:**

* Создайте ветку в вашем текущем коммите с помощью `git checkout -b <name>`, затем повторно запустите обзор

<h3 id="no-github-account-is-connected-to-your-claude-account">
  Ни одна учётная запись GitHub не подключена к вашей учётной записи Claude
</h3>

Вы запустили `/code-review ultra <PR#>` или `claude ultrareview <PR#>`, и перед созданием облачного сеанса Claude Code спрашивает сервер, может ли [учётная запись GitHub, подключённая к вашей учётной записи Claude](/docs/ru/ultrareview#review-a-pull-request), достичь репозитория PR. Ни одна учётная запись не подключена или соединение истекло, поэтому облачный клон не удастся и Claude Code отказывает запуск. Claude Code не тратит бесплатный запуск и не выставляет счёт за использование кредитов для отклонённого запуска.

```text theme={null}
Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected (or the connection expired). To fix: run /web-setup to reuse your GitHub CLI login, or connect an account at https://claude.ai/connect-github — then re-run /code-review ultra 1234 (allow a minute after connecting).
```

Когда [`/web-setup`](/docs/ru/web-quickstart#connect-from-your-terminal) недоступна в вашем сеансе, сообщение указывает только ссылку claude.ai.

**Что делать:**

* Запустите `/web-setup` для подключения вашего входа GitHub CLI к вашей учётной записи Claude или подключите учётную запись на [claude.ai/connect-github](https://claude.ai/connect-github)
* Повторно запустите обзор через минуту после подключения

До версии 2.1.248 Claude Code не проверял это перед запуском.

<h3 id="your-connected-github-account-cant-see-the-repository">
  Ваша подключённая учётная запись GitHub не может видеть репозиторий
</h3>

Вы запустили `/code-review ultra <PR#>` или `claude ultrareview <PR#>`, и [учётная запись GitHub, подключённая к вашей учётной записи Claude](/docs/ru/ultrareview#review-a-pull-request), не может прочитать репозиторий PR, поэтому облачный клон не удастся и Claude Code отказывает запуск. Claude Code не тратит бесплатный запуск и не выставляет счёт за использование кредитов для отклонённого запуска.

```text theme={null}
Your connected GitHub account can't see <owner>/<repo> — usually the Claude GitHub app isn't installed on <owner> or wasn't granted this repo (web-connected accounts need it for private repos), or a different GitHub account is connected. To fix: run /web-setup to reuse your GitHub CLI login, or install the app at https://github.com/apps/claude/installations/new — then re-run /code-review ultra 1234.
```

Когда [`/web-setup`](/docs/ru/web-quickstart#connect-from-your-terminal) недоступна в вашем сеансе, сообщение указывает только установку приложения.

**Что делать:**

* Если ваш локальный CLI `gh` может прочитать репозиторий, запустите `/web-setup` для подключения этого входа к вашей учётной записи Claude
* Повторно запустите обзор после изменения

До версии 2.1.248 Claude Code не проверял это перед запуском.

<h3 id="the-github-app-preflight-failed-transiently">
  Проверка GitHub App не прошла временно
</h3>

Вы запустили [облачный сеанс](/docs/ru/claude-code-on-the-web) из локального репозитория, и два шага не прошли вместе. Claude Code не смог создать или загрузить bundle вашего репозитория. Перед загрузкой он проверил, может ли облачный сервис клонировать репозиторий из GitHub, и вместо определённого ответа эта проверка закончилась ошибкой, которую повторная попытка могла бы очистить, такой как сетевая ошибка, тайм-аут или временная ошибка сервера. Полное сообщение начинается с того, что остановило bundle, например `Could not upload repo bundle (<error>)`, и заканчивается предложением проверки:

```text theme={null}
Could not upload repo bundle (<error>). The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead
```

**Что делать:**

* Повторно запустите команду через момент. Когда проверка GitHub пройдёт, Claude Code может запустить сеанс из клона GitHub, поэтому неудачная загрузка больше не блокирует запуск
* Если повторные попытки продолжают не работать, начало сообщения указывает, что остановило загрузку. Когда эта причина — что-то, что вы можете исправить, исправьте это, чтобы сеанс мог запуститься из вашего локального репозитория вместо этого

До версии 2.1.251 Claude Code заканчивал сообщение с `Please set up GitHub on https://claude.ai/code` даже когда проверка GitHub не прошла только временно, и совет по настройке не может очистить временный отказ.

<h3 id="github-isnt-connected-to-your-claude-account">
  GitHub не подключён к вашей учётной записи Claude
</h3>

Вы запустили [облачный сеанс](/docs/ru/claude-code-on-the-web) из вашего локального репозитория, например с помощью `/autofix-pr`. Ни одна учётная запись GitHub не подключена к вашей учётной записи Claude, или соединение истекло, поэтому Claude Code отказывает запуск:

```text theme={null}
GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud. Run /web-setup to connect with your GitHub CLI login, or connect on the web at https://claude.ai/connect-github
```

Когда вы создаёте рутину с помощью [`/schedule`](/docs/ru/routines), то же сообщение появляется как примечание настройки, которое указывает репозиторий; примечание не блокирует создание рутины.

**Что делать:**

* Запустите `/web-setup` для подключения вашего входа GitHub CLI к вашей учётной записи Claude или подключите учётную запись на [claude.ai/connect-github](https://claude.ai/connect-github). См. [GitHub authentication options](/docs/ru/claude-code-on-the-web#github-authentication-options) для того, как они отличаются.
* Повторно запустите команду через минуту после подключения

До версии 2.1.268 Claude Code сообщал об этом как о временном отказе проверки Claude GitHub App и предлагал повторить попытку или установить приложение; ни то, ни другое не подключает учётную запись GitHub.

<h3 id="single-sign-on-authorization-needed">
  Требуется авторизация единого входа
</h3>

Вы запустили [`/install-github-app`](/docs/ru/github-actions#quick-setup) и выбрали репозиторий, организация которого применяет единый вход SAML. Перед настройкой Claude Code проверяет ваш доступ к репозиторию с помощью GitHub CLI, и GitHub отказал эту проверку, потому что ваш токен `gh` ещё не авторизован для организации. Мастер показывает предупреждение с шагами для авторизации:

```text theme={null}
Single sign-on authorization needed
<owner>/<repo> belongs to an organization that enforces SAML single sign-on, and your GitHub CLI token isn't authorized for it yet.
```

**Что делать:**

* Повторно авторизуйте ваш вход GitHub CLI с областями `repo` и `workflow`, запустив `gh auth refresh -h github.com -s repo,workflow`, и авторизуйте организацию, когда GitHub запросит единый вход
* Если вы аутентифицируетесь с личным токеном доступа в `GH_TOKEN`, откройте [github.com/settings/tokens](https://github.com/settings/tokens), выберите **Configure SSO** на токене и авторизуйте организацию
* Запустите `/install-github-app` снова

До версии 2.1.273 Claude Code показывал предупреждение `Admin permissions required` для этого условия вместо этого.

<h3 id="failed-to-resume-the-conversation">
  Не удалось возобновить разговор
</h3>

Claude Code не смог прочитать или обработать сохранённую стенограмму для сеанса, который вы выбрали из [средства выбора `claude --resume`](/docs/ru/sessions#use-the-session-picker), поэтому он заканчивает процесс вместо продолжения в частично загруженном состоянии. Сообщение включает команду для повторной попытки:

```text theme={null}
Failed to resume the conversation.
Run claude --resume <session-id> to retry, or claude to start a new session.
```

Claude Code выходит с кодом 1 после показа сообщения. Средство выбора `/resume` внутри работающего сеанса сообщает `Failed to resume conversation` в разговоре вместо этого, и ваш текущий сеанс продолжает работать. До версии 2.1.216 неудачное возобновление из средства выбора `claude --resume` оставалось на спиннере `Resuming conversation…` бесконечно вместо показа этого сообщения.

**Что делать:**

* Запустите `claude --resume <session-id>` с ID сеанса из сообщения для повторной попытки
* Если каждая повторная попытка не работает так же, запустите `claude update` и возобновите снова. Версии до v2.1.275 не работают при возобновлении, когда сохранённая стенограмма содержит запись, которую они не могут прочитать.
* Если повторная попытка снова не работает, запустите `claude` для запуска нового сеанса

<h3 id="no-conversation-found-with-the-session-id">
  Разговор не найден с ID сеанса
</h3>

Вы передали ID сеанса в `claude --resume <session-id>` и ни одна сохранённая стенограмма не совпала с ним:

```text theme={null}
No conversation found with session ID: <session-id>
```

Claude Code выходит с кодом 1 после показа сообщения. Claude Code [ищет текущий проект первым, затем каждый другой проект на этой машине](/docs/ru/sessions#resume-a-session) для ID. До версии 2.1.223 поиск останавливался в текущем каталоге проекта и его git worktrees, поэтому возобновляется из каталога, в котором сеанс в последний раз работал.

Распространённые причины:

* **Неправильно введённый ID**: для неинтерактивного запуска ID — это поле `session_id` вывода [`--output-format json`](/docs/ru/headless#get-structured-output)
* **Удалённая стенограмма**: Claude Code удаляет стенограммы после [периода хранения](/docs/ru/sessions#where-transcripts-are-stored), 30 дней по умолчанию, следуя [правилам очистки хранения](/docs/ru/claude-directory#cleaned-up-automatically)
* **Другая машина**: Claude Code хранит стенограммы локально, поэтому возобновляет сеанс на машине, где он работал
* **Дублирующиеся копии**: если вы скопировали каталог проекта под `~/.claude/projects`, чтобы две стенограммы имели один и тот же ID, Claude Code сообщает это сообщение вместо возобновления одной копии произвольно

**Что делать:**

* Для интерактивного сеанса откройте [средство выбора сеанса](/docs/ru/sessions#use-the-session-picker) с помощью `claude --resume` и нажмите `Ctrl+A` для расширения на каждый проект на этой машине, затем выберите сеанс
* Сеансы, созданные с помощью `claude -p` или [Agent SDK](/docs/ru/agent-sdk/overview), не появляются в средстве выбора, поэтому повторно проверьте ID против `session_id`, который выводит ваш исходный запуск

<h3 id="cannot-switch-renderers-in-this-session">
  Не удалось переключить рендеры в этом сеансе
</h3>

Когда вы переключаете рендеры, Claude Code перезапускает свой процесс. Вы запустили [`/tui`](/docs/ru/fullscreen#enable-fullscreen-rendering) в сеансе, который Claude Code отказывает перезапускать, поэтому он не переключается и ничего не сохраняет. Какое сообщение вы видите, говорит вам причину:

* `Cannot switch renderers while work is running in the background`: у вас есть фоновая работа, которая перезапуск бросит, такая как фоновая оболочка или подагент. Дождитесь завершения работы или остановите её с помощью [`/tasks`](/docs/ru/commands), затем запустите `/tui fullscreen` или `/tui default` снова
* `Cannot switch renderers in this session`: сеанс имеет ограничения, которые Claude Code не может передать перезапущенному процессу. До версии 2.1.234 Claude Code перезапускал в любом случае и повторно запущенный сеанс работал без них

В сообщении об ограничениях часть в скобках указывает найденные ограничения Claude Code:

```text theme={null}
Cannot switch renderers in this session — it has restrictions a restart can't carry over (permission rules set for this session only). Nothing was changed. Running /tui fullscreen in a session started without them switches every later session too.
```

Каждая причина, которую сообщение может показать в скобках:

* `launch flags: a custom system prompt, a tool allowlist, or restricted settings`: вы запустили сеанс с флагом, который Claude Code не передаёт обратно перезапущенному процессу. Эти флаги включают [`--system-prompt`](/docs/ru/cli-reference#cli-flags), `--system-prompt-file`, `--append-system-prompt-file`, список [`--tools`](/docs/ru/cli-reference#cli-flags), [`--setting-sources`](/docs/ru/cli-reference#cli-flags) и [`--permission-prompt-tool`](/docs/ru/cli-reference#cli-flags)
* `permission rules set for this session only`: [обновление разрешений](/docs/ru/hooks#permission-update-entries) из hook или вызывающего SDK добавило правила deny или ask с назначением `session`. Правила allow с областью сеанса не вызывают отказ. Перезапуск их удаляет, и Claude Code вместо этого запрашивает снова
* `ask-before-running rules with no command-line form`: обновление разрешений из hook или вызывающего SDK добавило правила ask наряду с правилами, которые Claude Code передаёт обратно как `--allowed-tools` и `--disallowed-tools`. Флаг для правил ask не существует
* `permission rules a command line cannot carry intact` и `added directories a command line cannot carry intact`: обновление разрешений добавило правило или путь каталога в середине сеанса. Командная строка перезапущенного процесса не может нести его текст как то же значение

**Что делать:**

* В сеансе, запущенном без этих ограничений, запустите `/tui fullscreen` или `/tui default` для переключения обратно. Claude Code сохраняет параметр [`tui`](/docs/ru/settings-reference#tui) там

<h3 id="couldnt-open-claude-desktop">
  Не удалось открыть Claude Desktop
</h3>

Вы запустили [`/desktop`](/docs/ru/desktop#coming-from-the-cli), или его псевдоним `/app`, и системная команда, которую Claude Code использует для открытия Claude Desktop, не работала. Сеанс остаётся в терминале.

```text theme={null}
Error: Couldn't open Claude Desktop (`open` exited 1: LSOpenURLsWithRole() failed for the URL claude://resume?session=<session-id> with error -10814). Open Claude Desktop and run /desktop again.
```

**Что делать:**

* Откройте Claude Desktop самостоятельно, затем запустите `/desktop` снова
* Чтобы прочитать полный вывод ошибки этой команды, включите отладочное логирование с помощью `/debug`, запустите `/desktop` снова и проверьте журнал отладки

До версии 2.1.275 сообщение было `Failed to open Claude Desktop. Please try opening it manually.` и не говорило, что не прошло.

<h3 id="terminal-setup-left-your-zed-keymap-unchanged">
  /terminal-setup оставил вашу раскладку клавиш Zed без изменений
</h3>

Вы запустили [`/terminal-setup`](/docs/ru/terminal-config#enter-multiline-prompts) в Zed, и Claude Code не смог завершить обновление вашего `keymap.json` Zed, поэтому он оставил файл как он был.

Каждое сообщение указывает путь к вашей раскладке и заканчивается блоком сочетания клавиш для добавления самостоятельно:

```text theme={null}
Couldn't update your Zed keymap, so it was left unchanged.
To add the binding yourself, add this block to the keymap array in <path to keymap.json>:
{ "context": "Terminal", "bindings": { "shift-enter": ["terminal::SendText", "\u001b\r"] } }
```

Первая строка сообщения указывает причину:

* `Couldn't read your Zed keymap, so it was left unchanged.`: Claude Code не смог прочитать файл, например из-за разрешений файла
* `Your Zed keymap isn't a readable list of keybindings, so it was left unchanged.`: файл прочитался хорошо, но не анализируется как массив блоков сочетаний клавиш, даже с разрешёнными комментариями `//` и запятыми в конце
* `Couldn't back up your Zed keymap; not modifying it.`: Claude Code не смог скопировать файл в резервную копию `.bak` рядом с ним, поэтому он ничего не изменил
* `Couldn't update your Zed keymap, so it was left unchanged.`: объединённый результат не проверился как действительная раскладка, несущая сочетание, поэтому Claude Code отбросил его вместо записи. Блок сочетания с дублированным ключом может вызвать это

**Что делать:**

* Скопируйте блок из сообщения в массив верхнего уровня в вашем `keymap.json` по пути, который указывает сообщение
* Для `isn't a readable list of keybindings`, исправьте ошибку синтаксиса или сделайте значение верхнего уровня файла массивом, затем запустите `/terminal-setup` снова

До версии 2.1.247 `/terminal-setup` не мог анализировать раскладку Zed, которая использовала комментарии `//` или запятые в конце, и она заменила весь файл только своим собственным сочетанием, сообщая о сочетании как установленном. Чтобы восстановить раскладку, которую более ранняя версия заменила, используйте файл резервной копии `.bak`, описанный в разделе [Enter multiline prompts](/docs/ru/terminal-config#enter-multiline-prompts).

<h3 id="skill-usage-reports-are-not-available-on-this-connection">
  Отчёты об использовании Skill недоступны на этом соединении
</h3>

Вы запустили [`/skill-doctor`](/docs/ru/skills#find-unused-skills) через [Remote Control](/docs/ru/remote-control), с вашего телефона или браузера. Claude Code не отправляет отчёт об использовании skill через Remote Control и вместо этого отвечает этим сообщением:

```text theme={null}
Skill usage reports are not available on this connection.
```

**Что делать:**

* Запустите `/skill-doctor` в терминале на машине, где работает сеанс, или запустите `claude -p "/skill-doctor"` там

<h3 id="custom-output-styles-cant-be-selected-over-remote-control">
  Пользовательские стили вывода не могут быть выбраны через Remote Control
</h3>

Вы запустили [`/output-style`](/docs/ru/output-styles#change-your-output-style) из мобильного приложения или веб-версии через [Remote Control](/docs/ru/remote-control), или команда пришла в сообщении, переданном в сеанс. Потому что такой ход может не поступить от владельца учётной записи, Claude Code перечисляет и выбирает только [встроенные стили](/docs/ru/output-styles#built-in-output-styles) на нём и добавляет это уведомление всякий раз, когда команда перечисляет стили или не распознаёт имя, которое вы дали. Имя [пользовательского стиля](/docs/ru/output-styles#create-a-custom-output-style) получает тот же ответ, что и имя, которое не существует:

```text theme={null}
Custom output styles can't be selected over Remote Control or from a relayed message. Select one in the session itself, or pick a built-in style here.
```

**Что делать:**

* Выберите встроенный стиль, например `/output-style concise`
* Чтобы использовать пользовательский стиль, установите [`outputStyle`](/docs/ru/settings-reference#outputstyle) в `.claude/settings.local.json` проекта или запустите `/output-style <style>` в собственном терминале сеанса, если он у него есть

<h3 id="output-styles-are-saved-to-local-settings-which-this-session-doesnt-load">
  Стили вывода сохраняются в локальные параметры, которые этот сеанс не загружает
</h3>

Вы попытались переключить [стили вывода](/docs/ru/output-styles) с помощью `/output-style <style>` или `/config outputStyle=<style>` в сеансе, чьи источники параметров исключают `local`. Примеры — сеанс [Agent SDK](/docs/ru/agent-sdk/typescript), чьи [`settingSources`](/docs/ru/agent-sdk/typescript#options) оставляют `"local"` и сеанс CLI, запущенный с [`--setting-sources`](/docs/ru/cli-reference#cli-flags) значением, которое оставляет `local`. Обе команды сохраняют стиль в `.claude/settings.local.json`, файл, который такой сеанс никогда не читает обратно, поэтому Claude Code отказывает вместо записи параметра, который не будет иметь никакого эффекта:

```text theme={null}
Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load, so the style can't be changed here.
```

**Что делать:**

* Добавьте `local` к источникам параметров сеанса и переключитесь снова
* Установите ключ [`outputStyle`](/docs/ru/settings-reference#outputstyle) в файле параметров, который сеанс загружает, такой как `.claude/settings.json` в проекте или `~/.claude/settings.json`. В TypeScript SDK установите `outputStyle` внутри встроенного объекта `settings` вместо этого; см. [Activate an output style](/docs/ru/agent-sdk/modifying-system-prompts#activate-an-output-style)

<h2 id="plugin-errors">
  Ошибки плагинов
</h2>

Эти ошибки возникают из [конфигурации плагинов](/docs/ru/plugins/overview) и [конфигурации маркетплейса](/docs/ru/plugins/overview). Для проблем с плагинами, которые не выдают одно из сообщений на этой странице, например маркетплейс, который не загружается, или плагин, который устанавливается, но не отображается, см. [Устранение неполадок плагинов](/docs/ru/plugins/troubleshooting).

<h3 id="plugin-eval-is-currently-in-early-access">
  plugin eval is currently in early access
</h3>

Вы запустили [`claude plugin eval`](/docs/ru/plugin-evals) или `claude plugin eval init`, и команда завершилась с кодом 1 с одним из этих сообщений перед выполнением каких-либо действий:

```text theme={null}
`plugin eval` is currently in early access
```

```text theme={null}
`plugin eval` is currently unavailable
```

Первое сообщение означает, что ваша сборка старше v2.1.269, первой версии, где команда общедоступна. Второе означает, что Anthropic отключила команду на стороне сервера; ничто на вашем компьютере не включит её обратно.

**Что делать:**

* Запустите `claude --version`, затем `claude update`, и запустите команду снова в новом сеансе. См. [требования для plugin evals](/docs/ru/plugin-evals#requirements)
* Если вы видите второе сообщение в текущей сборке, попробуйте снова позже после ещё одного `claude update`

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace is registered from an untrusted source
</h3>

Маркетплейс зарегистрирован под именем, которое [зарезервировано для официальных маркетплейсов Anthropic](/docs/ru/plugins/marketplace-reference#marketplace-file), но его зарегистрированный источник не является репозиторием GitHub `anthropics`. Claude Code повторно проверяет зарезервированные имена каждый раз при загрузке или обновлении маркетплейса, поэтому маркетплейс и плагины, установленные из него, перестают загружаться. До версии v2.1.205 имя проверялось только при добавлении маркетплейса, поэтому запись, зарегистрированная до того, как её имя было зарезервировано, продолжала загружаться.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

Для маркетплейса, источник которого не является репозиторием GitHub или URL-адресом Git, например локальной директорией, среднее предложение читается как `can only be used with GitHub sources from the 'anthropics' organization` вместо этого. `claude plugin marketplace add` выполняет ту же проверку и отказывает зарезервированному имени с `Failed to add marketplace:`, за которым следует то же предложение о зарезервированном имени.

**Что делать:**

* Если маркетплейс уже зарегистрирован, запустите `claude plugin marketplace remove <name>`, затем добавьте его снова из официального репозитория `github.com/anthropics`
* Если вы публикуете сторонний маркетплейс, который использовал это имя до того, как оно было зарезервировано, переименуйте его и попросите пользователей добавить его снова из вашего источника
* См. список зарезервированных имён в разделе [Marketplace schema](/docs/ru/plugins/marketplace-reference#marketplace-file)

<h3 id="marketplace-name-is-another-spelling-of-a-reserved-name">
  Marketplace name is another spelling of a reserved name
</h3>

Имя маркетплейса само по себе не является зарезервированным именем, но Claude Code рассматривает его как другое написание одного из них. [Зарезервированные имена](/docs/ru/plugins/marketplace-reference#reserved-name-spellings) перечисляют, какие написания считаются зарезервированным именем. Claude Code отказывает такому имени при добавлении маркетплейса:

```text theme={null}
Failed to add marketplace: "claude.code.plugins" is another spelling of "claude-code-plugins", a reserved marketplace name.
```

Когда маркетплейс уже зарегистрирован под таким именем, его запись перестаёт загружаться, и `/plugin`, `claude plugin install` и `claude plugin update` предупреждают:

```text wrap theme={null}
known_marketplaces.json has an entry named "claude.code.plugins", another spelling of the reserved marketplace name "claude-code-plugins", so it is ignored. Remove it with: claude plugin marketplace remove claude.code.plugins
```

Когда имя требует кавычек оболочки, отказ во время добавления читается как `This marketplace's name is another spelling of "<reserved>", a reserved marketplace name. It is not exactly the reserved name it appears to be.`

**Что делать:**

* Переименуйте маркетплейс на имя, которое не соответствует зарезервированному имени, и добавьте его снова
* Для предупреждения об игнорируемой записи запустите команду `claude plugin marketplace remove`, которую оно даёт, или удалите запись из `~/.claude/plugins/known_marketplaces.json`

<h3 id="marketplace-is-already-added-from-a-different-source">
  Marketplace is already added from a different source
</h3>

Вы подтвердили добавление маркетплейса через [`/plugin install <plugin> --marketplace <source>`](/docs/ru/plugins/install#add-a-marketplace-and-install-in-one-command), и каталог, который Claude Code получил из этого источника, называет себя так же, как маркетплейс, который вы уже добавили из другого источника. Claude Code сохраняет существующий маркетплейс вместо его замены, и плагин не устанавливается.

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

**Что делать:**

* Если маркетплейс, который вы уже добавили, это тот, который вам нужен, установите из него по имени: `/plugin install <plugin>@<name>`
* Чтобы переключиться на новый источник, запустите `/plugin marketplace remove <name>`, затем повторите попытку установки

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

Хук плагина, [monitor](/docs/ru/plugins/components#monitors), или команда MCP [`headersHelper`](/docs/ru/mcp#use-dynamic-headers-for-custom-authentication) ссылается на опцию `${user_config.KEY}` [плагина](/docs/ru/plugins/manifest-reference#user-configuration), и подставленная строка будет передана в оболочку. Настроенное значение, содержащее `$(...)`, обратные кавычки или `;`, будет выполнено как код там, поэтому Claude Code отказывается запускать компонент вместо подстановки значения. Проверка выполняется на шаблоне команды, поэтому ошибка появляется даже когда значение ещё не настроено. До версии v2.1.207 значение подставлялось в команду оболочки.

Формулировка зависит от того, какая поверхность ссылалась на опцию. Хук в форме оболочки сообщает:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Monitor сообщает:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

MCP `headersHelper` сообщает:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Что делать:**

* Для хука добавьте массив `args`, чтобы он выполнялся в [exec форме](/docs/ru/hooks#exec-form-and-shell-form), где каждый `${user_config.KEY}` становится одним аргументом без оболочки между ними. Или удалите ссылку и прочитайте переменную окружения `$CLAUDE_PLUGIN_OPTION_<KEY>` внутри скрипта
* Для monitor удалите ссылку и пусть скрипт monitor прочитает значение из файла конфигурации
* Для `headersHelper` переместите `${user_config.KEY}` в поле `headers` сервера, которое не анализируется оболочкой, или прочитайте значение внутри скрипта помощника

<h3 id="plugin-archive-integrity-check-failed">
  Plugin archive integrity check failed
</h3>

Запись маркетплейса плагина использует [источник `archive`](/docs/ru/plugins/marketplace-reference#archive-plugin-source) с закреплением `sha256`, и дайджест загруженного файла не совпадает с закреплением. Claude Code отказывает в установке, поэтому ничего не меняется в кэше плагинов. Несовпадение имеет три возможные причины:

* Файл по URL-адресу изменился после того, как автор вычислил закрепление
* Автор ввёл неправильный дайджест в запись маркетплейса
* URL-адрес служит другим файлом, чем тот, который автор закрепил

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

**Что делать:**

* Если вы публикуете плагин, пересчитайте дайджест точного файла, который служит URL-адрес, например с помощью `shasum -a 256 my-plugin.zip`, или `Get-FileHash -Algorithm SHA256 my-plugin.zip` в PowerShell, и обновите `sha256` в записи маркетплейса
* Если вы устанавливаете плагин, запустите `/plugin marketplace update <name>` для обновления каталога на случай, если запись была исправлена, затем повторите попытку установки
* Если дайджесты всё ещё не совпадают после обновления, спросите владельца маркетплейса, какой файл они закрепили перед установкой

<h3 id="path-escapes-plugin-directory">
  Path escapes plugin directory
</h3>

Путь компонента плагина, объявленный в `plugin.json` плагина или в его [записи маркетплейса](/docs/ru/plugins/marketplace-reference#plugin-entries), разрешается вне собственной директории плагина. Claude Code отбрасывает этот путь и загружает остальную часть плагина. Имя компонента в сообщении, такое как `commands` или `hooks`, называет поле, которое объявило путь.

```text theme={null}
commands path escapes plugin directory: ./../shared.md
```

В выводе команды `claude plugin` та же ошибка читается как `Path escapes plugin directory: ./../shared.md (commands)`.

Claude Code отклоняет как путь, который указывает вне плагина в том виде, в котором он написан, например `../shared-utils`, так и символическую ссылку, которая ведёт вне плагина и не является одной из [правил символических ссылок маркетплейса](/docs/ru/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks), которые разрешены. Для символической ссылки сообщение также указывает, где разрешается путь:

```text theme={null}
commands path escapes plugin directory: ./commands/deploy.md — it resolves to /home/user/shared/deploy.md, outside the plugin directory
```

На macOS и Linux Claude Code также отклоняет путь компонента, который содержит обратную косую черту где-либо в нём, даже когда путь остаётся внутри плагина. Плагин, чьи пути компонентов используют разделители в стиле Windows, загружается на Windows и вызывает это отклонение на других платформах:

```text theme={null}
commands path escapes plugin directory: ./commands\deploy.md — its path contains a backslash, which is not resolved reliably on this platform
```

До версии v2.1.251 Claude Code загружала путь `commands`, объявленный в записи маркетплейса, даже когда он указывал вне директории плагина. Claude Code уже отклоняла пути, объявленные в `plugin.json`, и другие пути компонентов в записи маркетплейса.

До версии v2.1.257 проверка смотрела только на написание пути, а не на то, где ведёт символическая ссылка.

**Что делать:**

* Переместите упомянутый файл внутри директории плагина и укажите на него с помощью относительного пути `./`
* Если путь является символической ссылкой на файл вне плагина, замените символическую ссылку копией файла
* Если сообщение говорит, что путь содержит обратную косую черту, напишите путь с прямыми косыми чертами, например `./commands/deploy.md`
* Для совместного использования файлов с другими плагинами в одном маркетплейсе свяжите их с помощью символической ссылки внутри директории плагина, следуя [правилам символических ссылок](/docs/ru/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)

<h3 id="path-could-not-be-checked">
  Path could not be checked
</h3>

Claude Code попросила операционную систему проверить, существует ли путь плагина, и получила ошибку, отличную от «не найдено», поэтому она не загружает то, что называет путь. Сколько плагина загружается, зависит от того, какой путь не удался:

* Одно из [расположений компонентов по умолчанию](/docs/ru/plugins/manifest-reference#standard-layout) плагина, такое как папка `skills/`, файл `monitors/monitors.json` или [`SKILL.md` в корне плагина](/docs/ru/plugins/components#skills): остальные компоненты плагина всё ещё загружаются
* Собственная директория плагина: ничего из этого плагина не загружается

Вы не видите эту ошибку для пути, который вообще не существует. В `/plugin` ошибка появляется под плагином и называет путь и код, который вернула операционная система:

```text theme={null}
skills path could not be checked: /home/user/my-plugin/skills (ELOOP)
```

В `claude plugin list` та же ошибка читается как `Path not found: /home/user/my-plugin/skills (skills, ELOOP)`.

Причины, которые вызывают эту ошибку, включают:

* `ELOOP`: символическая ссылка в пути указывает на себя или образует цикл
* `EIO` или `ESTALE`: путь находится на сетевом монтировании, которое нарушено или устарело
* `EACCES`: один из директорий выше пути отказывает вам в разрешении на его обход

**Что делать:**

* Замените символическую ссылку, которая указывает на себя, на реальную папку, или удалите её
* Если путь находится на сетевом монтировании, переподключите общий ресурс
* Если код `EACCES`, восстановите ваше разрешение на выполнение в директориях выше пути
* Запустите `/reload-plugins` после исправления пути, или перезагрузите Claude Code, чтобы загрузить плагин или компонент

До версии v2.1.265 Claude Code рассматривала папку компонента по умолчанию, которую она не могла проверить, как отсутствующую и загружала плагин без этого компонента, без ошибки.

<h3 id="marketplace-entry-path-does-not-stay-inside-the-marketplace-directory">
  Marketplace entry path does not stay inside the marketplace directory
</h3>

[Запись маркетплейса](/docs/ru/plugins/marketplace-reference#plugin-entries) плагина объявляет путь источника, который Claude Code не может разрешить в расположение внутри собственной директории маркетплейса, поэтому плагин не устанавливается и не загружается. Отказ охватывает:

* Путь записи, который является абсолютным, поднимается из маркетплейса с помощью `..`, или написан как сетевой путь
* На macOS и Linux путь записи, который содержит обратную косую черту где-либо после начального `./`
* Запись в маркетплейсе, полученном из удалённого источника, такого как git или URL, которая достигает своей цели через символическую ссылку, разрешающуюся вне директории маркетплейса
* Относительная запись в маркетплейсе, добавленном из прямого URL-адреса его `marketplace.json`: Claude Code загружает только этот файл, поэтому локальные файлы плагинов не существуют для пути, чтобы назвать. См. [Plugins with relative paths fail in URL-based marketplaces](/docs/ru/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)

`claude plugin install` сообщает об отказе следующим образом:

```text theme={null}
Cannot install my-plugin@my-marketplace: its marketplace entry path does not stay inside the marketplace directory (an absolute, climbing, network-shaped, backslash-containing or link-traversing entry, an entry of a fetched marketplace that resolves or opens outside its tree — or a relative entry in a url-catalog marketplace, which has no local directory)
```

Когда запись уже установленного плагина не проходит ту же проверку, `claude plugin list` показывает плагин как `failed to load` с:

```text theme={null}
Plugin source path refused: ./my-plugin does not stay inside its marketplace directory. Check that the marketplace entry has a plain relative path.
```

**Что делать:**

* Если вы поддерживаете маркетплейс, напишите `source` записи как простой относительный путь с прямыми косыми чертами, например `./plugins/my-plugin`, и держите любую символическую ссылку, которую он пересекает, указывающей внутри директории маркетплейса
* Если вы добавили маркетплейс из прямого URL-адреса, относительные записи не могут разрешиться. Попросите автора маркетплейса использовать [другой источник плагина](/docs/ru/plugins/marketplace-reference#plugin-sources), или добавьте маркетплейс из его репозитория git вместо этого

<h3 id="failed-to-load-marketplace-configuration">
  Failed to load marketplace configuration
</h3>

Claude Code хранит маркетплейсы плагинов, которые вы добавили, в файле реестра в `~/.claude/plugins/known_marketplaces.json`. Команда плагина, которой нужен реестр, такая как `claude plugin install`, не выполняется с одним из двух сообщений, когда Claude Code не может использовать файл:

* `Failed to load marketplace configuration`: файл не является действительным JSON, или не может быть прочитан. Пустой файл также не работает таким образом.
* `Marketplace configuration file is corrupted`: файл является действительным JSON, но его содержимое не соответствует схеме реестра.

Отсутствующий файл не является ошибкой: Claude Code рассматривает его как реестр без маркетплейсов.

С пустым файлом `claude plugin install` сообщает:

```text theme={null}
✘ Failed to install plugin "my-plugin": Failed to load marketplace configuration: JSON Parse error: Unexpected EOF
```

До версии v2.1.246 `claude plugin install` не сообщала об этой ошибке.

**Что делать:**

* Откройте `~/.claude/plugins/known_marketplaces.json` и исправьте JSON, или исправьте записи, которые сообщение называет как не соответствующие схеме реестра
* Если вы не можете исправить это, удалите файл или замените его содержимое на `{}`, затем добавьте каждый маркетплейс снова с помощью `claude plugin marketplace add <source>`. Claude Code повторно регистрирует маркетплейсы, которые ваш пользователь или управляемые параметры объявляют в [`extraKnownMarketplaces`](/docs/ru/settings-reference#extraknownmarketplaces), в следующий раз, когда вы запустите его в папке, которую вы доверили.

<h3 id="plugin-is-required-by-your-organization">
  Plugin is required by your organization
</h3>

Вы запустили `claude plugin disable`, или использовали вкладку `/plugin` **Installed**, чтобы отключить [плагин, синхронизированный с claude.ai](/docs/ru/plugins/loading#synced-plugins), который ваша организация отмечает как обязательный:

```text theme={null}
Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.
```

Claude Code ничего не сохраняет и плагин остаётся включённым.

Когда вы пытаетесь отключить плагин, от которого зависит обязательный плагин, Claude Code отказывает таким же образом, с сообщением, называющим обязательный плагин, который его требует.

**Что делать:**

* Попросите администратора вашей организации claude.ai изменить статус обязательности плагина на claude.ai

<h2 id="tool-errors">
  Ошибки инструментов
</h2>

Эти ошибки исходят от встроенных инструментов Claude. Claude самостоятельно исправляет большинство ошибок инструментов. Когда требуется изменение с вашей стороны, список **Что делать** для этой ошибки указывает, что нужно изменить.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent would be spawned with zero tools
</h3>

Каждая запись в списке [`tools` подагента](/docs/ru/sub-agents#supported-frontmatter-fields) не соответствовала ни одному используемому инструменту, поэтому Claude Code отказался запускать подагента: без инструментов он не мог действовать. Сообщение группирует ваши записи по причине ошибки:

* **Unrecognized**: запись не соответствует ни одному имени инструмента, обычно это опечатка, например `Grpe` вместо `Grep`.
* **Not available to subagents**: запись называет реальный инструмент, который [подагенты не могут использовать](/docs/ru/sub-agents#available-tools). Фоновые подагенты имеют меньший встроенный набор инструментов, поэтому запись, которую может использовать только передний подагент, попадает сюда, когда подагент будет работать в фоне, что является значением по умолчанию. Если вы указываете `Agent`, сообщение сообщает об этом в следующей группе.
* **Matched no tools in this session**: запись действительна, но ни один инструмент в текущем сеансе не соответствует ей прямо сейчас, например `mcp__github__*` без подключённого сервера GitHub MCP или `Agent` для подагента на [пределе глубины](/docs/ru/sub-agents#let-subagents-spawn-their-own-subagents).

Пропуск поля `tools` никогда не вызывает этот отказ. Если вы оставляете список `tools` пустым или `disallowedTools` удаляет каждую запись в нём, Claude Code также пропускает отказ и запускает подагента без инструментов.

До версии 2.1.208 подагент запускался без инструментов и мог вернуть пустой или запутанный результат.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**Что делать:**

* Исправьте каждую запись, которую называет ошибка, в соответствии с [инструментами, доступными подагентам](/docs/ru/sub-agents#available-tools)
* Удалите записи для инструментов, которые сеанс не имеет, например инструменты MCP с сервера, который не подключён
* Для инструмента, который [фоновые подагенты отбрасывают](/docs/ru/sub-agents#available-tools), например `CronCreate`, удалите запись. Чтобы сохранить инструмент, [отключите режим fork](/docs/ru/sub-agents#turn-fork-mode-on-or-off) и попросите Claude запустить подагента на переднем плане
* Удалите поле `tools` вместо перечисления инструментов, чтобы дать подагенту каждый [инструмент, доступный подагентам](/docs/ru/sub-agents#available-tools)
* Для списка `tools`, содержащего только `Agent`, повысьте [предел глубины](/docs/ru/sub-agents#let-subagents-spawn-their-own-subagents) или дайте агенту по крайней мере один другой инструмент: Claude Code скрывает `Agent` на этом пределе, поэтому список с ничем другим в нём разрешается в отсутствие инструментов

<h3 id="file-is-covered-by-a-read-deny-rule">
  File is covered by a Read deny rule
</h3>

Инструмент Edit или Write был вызван на пути, соответствующем [правилу отказа `Read`](/docs/ru/permissions#read-and-edit), включая создание нового файла по этому пути. Оба инструмента изменяют содержимое, которое Claude должен иметь возможность прочитать обратно, поэтому Claude Code отказывает в вызове перед любым доступом к файлу. NotebookEdit не охватывается правилами отказа `Read`. До версии 2.1.228 правило блокировало только инструмент Edit, а до версии 2.1.208 только правило отказа `Edit` блокировало редактирование.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

Когда Claude Code отказывает инструменту Write, сообщение заканчивается на `and cannot be written` вместо этого.

**Что делать:**

* Если Claude должен иметь возможность изменять файл, удалите или сузьте правило отказа `Read` в `/permissions` или в [параметрах](/docs/ru/settings-reference#permission-settings)
* Если файл должен остаться нетронутым, сохраните правило и добавьте правило отказа `Edit` для того же пути, чтобы также заблокировать инструмент NotebookEdit

<h3 id="subagent-type-is-required">
  subagent\_type is required
</h3>

```text theme={null}
subagent_type is required: the general-purpose agent is not available in this session. Available agents: ...
```

Claude вызвал [инструмент Agent](/docs/ru/tools-reference#agent-tool-behavior) без `subagent_type`, и этот сеанс не имеет [универсального подагента](/docs/ru/sub-agents#built-in-subagents) для отката. Это происходит в двух конфигурациях:

* [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/ru/env-vars) установлен в неинтерактивном режиме, что удаляет каждого встроенного подагента
* Основной поток агента сеанса имеет [`tools: Agent(...)` список разрешений](/docs/ru/sub-agents#restrict-which-subagents-can-be-spawned), который исключает `general-purpose`

**Что делать:**

* Обычно ничего: сообщение перечисляет подагентов, которые есть в сеансе, поэтому Claude может повторить попытку с одним из них
* Если Claude продолжает терпеть неудачу, добавьте `general-purpose` в список разрешений `tools: Agent(...)` или отмените установку `CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS`

До версии 2.1.235 тот же вызов завершался с ошибкой `Agent type 'general-purpose' not found`.

<h3 id="memory-index-is-over-its-read-limit">
  Memory index is over its read limit
</h3>

Claude написал в индекс [автоматической памяти](/docs/ru/memory#auto-memory) `MEMORY.md` и оставил его выше одного из его пределов чтения: 200 строк или 25 КБ. Запись прошла успешно, но загружаются только первые 200 строк или 25 КБ, в зависимости от того, что меньше, в начале сеанса, поэтому всё, что находится за пределом, отбрасывается каждый раз при чтении индекса. До версии 2.1.210 индекс, превышающий лимит, молча усекался при следующей загрузке без сигнала во время записи.

```text theme={null}
Error: this write left the memory index at MEMORY.md at 214 lines, over its 200-line read limit. The write succeeded, but everything past the limit is silently dropped each time the index is loaded — entries at the end are already invisible to readers. Rewrite it to under 140 lines now: keep one line per entry, move detail into topic files, and merge or drop stale entries.
```

Только содержимое, которое загружается, учитывается в пределах. Frontmatter YAML и блочные комментарии HTML удаляются перед загрузкой индекса, поэтому они исключаются из измерения. До версии 2.1.211 Claude Code измерял исходный файл, и frontmatter или комментарии могли вызвать эту ошибку даже когда загруженное содержимое подходило.

Claude Code доставляет ошибку Claude после записи, а не выводит её как баннер в вашем терминале, поэтому вы можете заметить её только в стенограмме.

Когда запись Claude приносит файл близко к пределу без его пересечения, Claude Code возвращает более мягкое напоминание о сжатии индекса вместо этой ошибки.

**Что делать:**

* Позвольте Claude переписать `MEMORY.md` или попросите это: сохраняйте одну строку на запись, перемещайте детали в файлы тем и объединяйте или отбрасывайте устаревшие записи
* Чтобы обрезать индекс самостоятельно, см. [Аудит и редактирование вашей памяти](/docs/ru/memory#audit-and-edit-your-memory)

<h3 id="pkill-pattern-matches-the-claude-code-process">
  pkill pattern matches the Claude Code process
</h3>

Команда `pkill` в вызове инструмента Bash использовала шаблон, обычно с `-f`, который соответствует самому процессу Claude Code, поэтому Claude Code отказывает в команде вместо того, чтобы позволить ей завершить сеанс. Claude Code тестирует шаблон с помощью `pgrep` перед запуском `pkill` и отказывает, когда его собственный ID процесса находится в результате. Проверка выполняется только на Linux; на macOS `pkill` работает без изменений. До версии 2.1.214 команда выполнялась, и соответствующий шаблон убивал сеанс Claude Code в середине хода.

```text theme={null}
pkill: refusing to run — this pattern matches the Claude CLI process (PID 12345). Narrow the pattern, or target your own children with `pkill -P $$ ...`.
```

Отказ появляется в результате инструмента Bash, а не как баннер в вашем терминале, и Claude обычно самостоятельно корректирует команду.

**Что делать:**

* Сузьте шаблон так, чтобы он соответствовал только предполагаемому процессу, например полному пути целевого двоичного файла, а не короткой подстроке
* Чтобы остановить процессы, запущенные текущей оболочкой, используйте `pkill -P $$` с шаблоном, который ограничивает совпадение дочерними процессами оболочки

<h3 id="failed-to-write-to-a-teammate-inbox">
  Failed to write to a teammate's inbox
</h3>

Claude Code не смог написать сообщение в файл почтового ящика товарища под `~/.claude/teams/{team-name}/inboxes/`, поэтому получатель ничего не получил. Запись не удаётся, когда Claude Code не может создать или обновить файл, например потому что диск заполнен, каталог не доступен для записи или другой агент держит блокировку входящих сообщений слишком долго. До версии 2.1.224 Claude Code сообщал сообщение как отправленное даже когда запись не удалась.

Ошибка появляется в результате инструмента отправляющего агента, а не как баннер в вашем терминале, и его текст говорит Claude повторить попытку:

```text theme={null}
Failed to write to researcher's inbox — nothing was sent. Try again, or message the lead.
```

Структурированные сообщения протокола [команды агентов](/docs/ru/agent-teams) терпят неудачу таким же образом, и ошибка называет недоставленное сообщение: когда Claude Code не может написать одобрение плана, отклонение плана, запрос на завершение или отклонение завершения, ошибка читается как `Failed to write the <message> to <name>'s inbox — nothing was sent`. `plan approval` в этом списке — это решение лидера, одобряющее план товарища; отправка плана товарищем — это отдельное сообщение `plan approval request`. Это сообщение и два других сообщения протокола несут свой собственный текст сообщения и последствие:

* `Failed to write the plan approval request to the lead's inbox — plan not submitted; try again`: план товарища никогда не достиг лидера, и товарищ остаётся в режиме плана до успешной повторной отправки
* `The permission request could not be delivered to the team lead (mailbox write failed)`: запрос разрешения товарища никогда не достиг лидера, поэтому никто не одобрил вызов инструмента
* `The confirmation could not be written to team-lead's inbox.`: само одобрение завершения вступило в силу и товарищ выходит; только подтверждение лидеру отсутствует

Когда вы сами отправляете сообщение товарищу, вводя `@name` с последующим сообщением в сеансе лидера, та же ошибка появляется как уведомление, `Couldn't write to @name's inbox — message not sent. Try again.`, и Claude Code сохраняет ваш текст в поле подсказки, чтобы вы могли отправить его снова.

**Что делать:**

* Попросите отправителя повторно отправить сообщение; конкуренция за блокировку входящих сообщений является временной и разрешается при повторной попытке
* Проверьте свободное место на диске и убедитесь, что `~/.claude/teams` и файлы под ним доступны для записи вашим пользователем

<h3 id="teammate-agent-definition-not-restored">
  Teammate's agent definition was not restored
</h3>

Claude отправил сообщение остановленному товарищу [команды агентов](/docs/ru/agent-teams), и Claude Code вернул его без повторного применения [определения подагента](/docs/ru/agent-teams#use-subagent-definitions-for-teammates), из которого он был порождён, потому что его файл определения поступил из папки без сохранённого доверия. Уведомление следует за отчётом о возобновлении в результате инструмента отправляющего агента:

```text wrap theme={null}
Its agent definition was not restored: the folder its definition file came from is not trusted (source: projectSettings), so the teammate is running with the team-essential tools and no custom instructions. To restore it, the user needs to run Claude Code in that folder once and accept the trust dialog (the --debug log names the folder); do not change trust settings on the user's behalf.
```

Проверка применяется к определению в каталоге `.claude/agents/` проекта или каталога `--add-dir`, и принятие диалога доверия для родительской папки не удовлетворяет это.

**Что делать:**

* Запустите `claude` в папке, которую называет [журнал отладки](/docs/ru/debug-your-config), и примите диалог доверия. Определение повторно применяется в следующий раз, когда Claude Code вернёт товарища; вам не нужно перезагружать сеанс лидера
* Или установите запись `hasTrustDialogAccepted` на `true` в `~/.claude.json`, используя точный ключ `projects["<path>"]`, который печатает журнал отладки

<h3 id="message-too-large-for-cross-session-delivery">
  Message too large for cross-session delivery
</h3>

[Кросс-сеансовое сообщение](/docs/ru/cross-session-messaging) Claude другому вашему сеансу на этой машине было слишком длинным для отправки. Claude Code отказал в нём, и получающий сеанс ничего не получил. Отказ появляется в результате инструмента отправляющего сеанса, а не как баннер в вашем терминале. Он называет оба размера и как сделать сообщение подходящим:

```text wrap theme={null}
Failed to send to api-worker: Message too large for cross-session delivery: the serialized message is 1,203,844 characters and the limit is 1,048,576. Shorten the message text — put bulk content in a file the recipient can read rather than in the message — or split it into smaller messages.
```

Повторная отправка того же текста терпит неудачу таким же образом.

**Что делать:**

* Попросите Claude резюмировать сообщение или поместить объёмное содержимое в файл и отправить путь файла
* Попросите Claude разделить содержимое на несколько более коротких сообщений

До версии 2.1.235 Claude Code сообщал об увеличенном сообщении как отправленном. Получающий сеанс отбросил его непрочитанным.

<h3 id="too-many-messages-to-this-session-just-now">
  Too many messages to this session just now
</h3>

Claude отправил быстрый всплеск [кросс-сеансовых сообщений](/docs/ru/cross-session-messaging) одному из ваших сеансов на этой машине, и всплеск достиг того, что этот сеанс принимает. Claude Code отказал в следующей отправке, и получающий сеанс ничего не получил от неё. Отказ появляется в результате инструмента отправляющего сеанса, а не как баннер в вашем терминале:

```text wrap theme={null}
Failed to send to api-worker: Too many messages to this session just now: 30 were sent recently and more would be dropped by its rate limit, so this one was not sent. Batch what remains into one message, or wait a little before sending more.
```

**Что делать:**

* Обычно ничего: Claude объединяет оставшееся содержимое в одно сообщение или ждёт перед отправкой большего
* Если вы сами вызвали всплеск, попросите Claude объединить то, что осталось, в одно сообщение

До версии 2.1.236 Claude Code сообщал об этих отправках как отправленных. Получающий сеанс отбросил их непрочитанными.

<h3 id="refusing-to-send-a-cross-session-message">
  Refusing to send a cross-session message
</h3>

Перед тем как Claude Code напишет [кросс-сеансовое сообщение](/docs/ru/cross-session-messaging) другому вашему сеансу на этой машине, он проверяет, что сокет входящих сообщений целевого сеанса является конечной точкой, на которую было адресовано сообщение. Когда проверка не удаётся, Claude Code отказывает в отправке в отправляющем сеансе, и целевой сеанс ничего не получает. Для сообщения, которое отправляет Claude, отказ появляется в результате инструмента отправляющего сеанса:

```text theme={null}
Failed to send to api-worker: Refusing to send: reply target is a symlink
```

Текст после `Refusing to send:` называет проверку, которая не удалась:

* `reply target is a symlink`: символическая ссылка находится на пути сокета целевого сеанса. Claude Code не доставляет через неё, потому что ссылка там может перенаправить сообщение на конечную точку, которую целевой сеанс не создавал.
* `cannot vet reply target`: Claude Code не смог проверить целевой путь вообще, например потому что чтение не удалось с ошибкой разрешения.
* `connected endpoint is not the expected process`: процесс, держащий сокет, не является сеансом, на который было адресовано сообщение, поэтому адрес устарел или другой процесс заменил сокет.
* `connected endpoint identity could not be read`: Claude Code подключился, но не смог прочитать, какой процесс держит другой конец, поэтому он не смог подтвердить цель. Это может быть временным.
* `connected endpoint is not owned by this user`: процесс, держащий сокет, работает под другой учётной записью пользователя, поэтому это не один из ваших сеансов.
* `connected endpoint owner could not be read`: Claude Code подключился, но не смог прочитать, какая учётная запись пользователя владеет другим концом, поэтому он не смог подтвердить, что конечная точка ваша.
* `connected endpoint is a different process with the expected pid`: ID процесса соответствует тому, на который было адресовано сообщение, но Claude Code не смог подтвердить, что это тот же процесс. Обычно этот сеанс вышел и операционная система переиспользовала его ID процесса, поэтому адрес устарел.

**Что делать:**

* Обычно ничего: проверки предотвращают достижение сообщением конечной точки, отличной от сеанса, на который оно было адресовано, и ничего не было отправлено
* Попросите Claude снова перечислить ваши сеансы и повторно отправить; отказ, вызванный устаревшим адресом, разрешается после того как Claude отправляет текущему
* Если `reply target is a symlink` повторяется для одного сеанса, проверьте, что создало ссылку на пути сокета этого сеанса, показанном в его `/status` под `Peer address`
* Для `connected endpoint identity could not be read`, повторно отправьте; условие может быть временным
* Если `connected endpoint is not owned by this user` появляется на общей машине, сеанс по этому адресу работает под учётной записью другого пользователя, поэтому Claude не может отправить ему сообщение из вашей

До версии 2.1.248 Claude Code не проверял владельца конечной точки или время запуска процесса, поэтому отказы, которые называют эти проверки, не появляются в более ранних версиях.

<h3 id="refusing-after-a-symlink-changed">
  Refusing to read, write, or search a path
</h3>

Claude Code проверяет [правила разрешений](/docs/ru/permissions#read-and-edit) пути файла, затем подтверждает это разрешение снова, когда инструмент открывает файл или начинает поиск. Когда он не может подтвердить, что путь по-прежнему ведёт к месту, которое проверка одобрила, Claude Code отказывает в операции вместо того, чтобы следовать ей. Отказ появляется в результате инструмента:

```text wrap theme={null}
Refusing to read /path/to/file: its symlink resolution changed after permission was checked (a link on the way now leads somewhere the check did not see). If a link in the working directory is being rewritten concurrently, stop that and retry.
```

Каждый отказ называет его причину:

* `its symlink resolution changed after permission was checked`: символическая ссылка вдоль пути или в корне поиска Grep или Glob была заменена между проверкой разрешения и операцией. В отказе чтения фраза в скобках называет, какое сравнение не удалось.
* `its parent-directory symlink resolution changed after permission was checked`: каталог, через который проходит путь записи, больше не разрешается в одобренное место
* `it is a symbolic link. Write to the link's target path instead`: символическая ссылка находится в одобренном месте записи, например `CLAUDE.md`, который является символической ссылкой на `AGENTS.md`; сообщение направляет Claude к цели ссылки
* `Refusing to write through symlink: <path>. Resolve the symlink and pass the real target path explicitly.`: то же самое условие, обнаруженное, когда другой писатель открывает файл, например запись в символически связанный `.mcp.json`
* `Refusing to write into symlinked directory: <path>`: каталог, который содержит файл, сам является символической ссылкой, например каталог `.claude/` проекта, связанный с другим местоположением
* `a path one of its Read deny rules is written through changed while the search was being prepared. Retry.`: правило отказа `Read` для поиска называет путь, который проходит через символическую ссылку, и эта ссылка изменилась, пока Claude Code готовил поиск
* `it could not be opened (EACCES) — it is unreadable, or is being replaced concurrently.`: корень поиска существует, но не смог быть открыт; код в скобках — это ошибка операционной системы
* `its permission check expired before it ran (too many concurrent file operations). Retry.`: Claude Code вытеснил запись одобрения при множественных одновременных операциях с файлами перед использованием инструментом; повторная попытка выполняет свежую проверку разрешения
* `ripgrep was found only by name on PATH, and a search outside the working directory cannot apply your Read deny rules in that configuration`: Claude Code не смог разрешить двоичный файл `rg` в абсолютный путь, поэтому он отказывает в поисках вне рабочего каталога, а не выполняет один, который ваши правила отказа не охватывают

**Что делать:**

* Обычно ничего: отказ достигает Claude как результат инструмента, и отказанная операция не выполняется
* Если отказ символической ссылки повторяется на одном пути, найдите, что продолжает переписывать ссылку там, например инструмент сборки или наблюдатель файлов, или попросите Claude использовать разрешённый путь файла вместо связанного
* Если этот отказ появляется для каждого файла, пока Claude Code работает на Windows внутри AppContainer или песочницы с ограниченным токеном, обновитесь до версии 2.1.265 или позже
* Если отказ чтения появляется на macOS для файла, который ничто не переписывает, например скриншота, перетащенного в подсказку, обновитесь до версии 2.1.273 или позже
* Для отказа ripgrep установите ripgrep с помощью вашего менеджера пакетов, чтобы `rg` разрешался в абсолютный путь на `PATH`, или сохраняйте поиски под рабочим каталогом

До версии 2.1.251 Claude Code повторно проверял разрешение пути только для записей файлов, поэтому ссылка, заменённая после проверки разрешения, могла перенаправить чтение или поиск в другое место без сообщения. Из этих отказов только отказ записи родительского каталога, через символическую ссылку и в символически связанный каталог появляются в более ранних версиях.

<h3 id="task-output-swap-refused">
  Task output swap refused
</h3>

Claude Code сохраняет вывод каждой команды Bash в файл в своём временном каталоге. Каждый раз, когда он открывает один из этих файлов, он проверяет, что путь по-прежнему ведёт к файлу, который он создал, без символической ссылки, дополнительной жёсткой ссылки или перемещённого каталога, перенаправляющего его. Это сообщение означает, что проверка не удалась, поэтому Claude Code отказал в операции, а не писал или читал вывод через этот путь. Сообщение появляется в результате инструмента Bash:

```text wrap theme={null}
task output swap refused (tasks dir moved or linked): /private/tmp/claude-501/-Users-you-my-project/1f0e62dc-4b0a-4f5e-9c2d-8a7b6c5d4e3f/tasks/b7k2f9m3q.output. To recover: restart Claude Code with CLAUDE_CODE_TMPDIR set to a fresh directory; or, if /private/tmp/claude-501/-Users-you-my-project is a stray directory or a symbolic link that should not be there, remove that entry itself (not what it points to) and restart.
```

Текст в скобках называет проверку, которая не удалась. Причины, такие как `output symlink was re-pointed`, `output file identity changed` и `not a regular file`, все сообщают об одном и том же условии: что-то на пути вывода или вдоль него больше не является файлом, который создал Claude Code. Только некоторые причины несут предложение `To recover:`.

Если проверка не удаётся, пока команда всё ещё выполняется, Claude Code останавливает команду, и её результат сообщает:

```text theme={null}
Command killed: its output file was replaced or could no longer be verified
```

**Что делать:**

* Обновитесь до версии 2.1.260 или позже. Более ранние версии иногда показывали это сообщение, когда ссылка или перемещённый каталог не были присутствуют
* Перезагрузите Claude Code с [`CLAUDE_CODE_TMPDIR`](/docs/ru/env-vars), установленным в свежий каталог
* Или проверьте каталог вашего проекта в временном каталоге Claude Code, `/private/tmp/claude-501/-Users-you-my-project` в примере сообщения. Если этот путь является символической ссылкой или каталогом, который не должен быть там, удалите саму ссылку или каталог, а не цель ссылки, и перезагрузитесь
* Если отказ повторяется, процесс заменяет, связывает или удаляет записи в временном каталоге Claude Code, пока сеанс работает. Установите [`CLAUDE_CODE_TMPDIR`](/docs/ru/env-vars) в каталог, который ничто другое не управляет, и перезагрузитесь

<h3 id="the-source-file-is-not-valid-utf-8-text">
  The source file is not valid UTF-8 text
</h3>

Claude попытался опубликовать [артефакт](/docs/ru/artifacts) из файла, чьи байты не декодируются как текст, или чей текст уже содержит символ замены `U+FFFD`, поэтому Claude Code отказал в публикации перед загрузкой чего-либо. Сообщение появляется в результате инструмента Artifact и называет первую позицию для исправления:

```text wrap theme={null}
file_path: the source file is not valid UTF-8 text (first invalid byte at line 12, column 40). It may be saved in another encoding or contain binary data. Rewrite it as UTF-8, then publish again. Nothing was published.

file_path: the source file has the replacement character U+FFFD at line 12, column 40, usually left where an earlier edit or paste lost a character. Replace it with the intended text (in HTML, write an intended U+FFFD as &#xFFFD;), then publish again. Nothing was published.
```

Claude Code декодирует файл как UTF-8 или как UTF-16, когда он начинается с метки порядка байтов UTF-16 с прямым порядком байтов. Когда такой файл UTF-16 не декодируется, первое сообщение называет `UTF-16` и по-прежнему говорит вам переписать файл как UTF-8. Когда после названной позиции следуют дополнительные позиции, сообщение добавляет счётчик, такой как `(+2 more)`, после позиции.

**Что делать:**

* Обычно ничего: Claude переписывает файл и публикует снова
* Если файл — это тот, который вы написали или экспортировали, сохраните его снова как UTF-8 и замените каждый `U+FFFD` на символ, который более ранний редактор, вставка или преобразование потеряло
* Чтобы показать намеренный `U+FFFD` на странице, напишите его как `&#xFFFD;` в HTML вместо буквального символа

До версии 2.1.267 Claude Code загружал такой файл без проверки, и сервер отказывал в публикации вместо этого.

<h3 id="reading-a-local-file-from-outside-the-connected-folders">
  Reading a local file from outside the connected folders in a Cowork session
</h3>

В сеансе [Cowork](https://claude.com/docs/cowork/overview), работающем на вашей машине в приложении Claude Desktop, Claude назвал локальный файл для [артефакта](/docs/ru/artifacts). Claude Code не смог подтвердить, что файл является простым файлом внутри подключённых папок сеанса: путь находится вне этих папок, проходит через символическую ссылку или написан таким образом, что может назвать другой файл, чем он кажется. Чтение такого файла требует вашего одобрения, и в сеансе, который не может показать вам карточку одобрения, например в сеансе, установленном на пропуск всех одобрений, Claude Code отказывает в чтении.

Отказ появляется в результате инструмента Artifact; когда файл не смог быть проверен вообще, он называет эту ошибку вместо этого:

```text wrap theme={null}
Reading a local file from outside this session's connected folders, or through a link, needs the approval card, and no one can answer it in this Cowork session. Use a plain file inside the connected folders; do not retry this file in this session.

cannot read file_path (ENOENT) — the file could not be examined, and no one can answer the approval card in this Cowork session. Check that the file exists as a plain file inside the connected folders, then retry with that path.
```

**Что делать:**

* Обычно ничего: сообщение говорит Claude использовать простой файл внутри подключённых папок вместо этого
* Чтобы поместить этот точный файл в артефакт, скопируйте его в одну из подключённых папок сеанса как обычный файл, а не символическую ссылку, и попросите снова

<h3 id="webfetch-cannot-fetch-localhost">
  WebFetch cannot fetch localhost
</h3>

Claude вызвал [WebFetch](/docs/ru/tools-reference#webfetch-tool-behavior) с URL-адресом, чьё имя хоста не содержит точку, например `http://localhost:3000` или простое имя интранета, например `http://wiki/`. WebFetch отказывает в этих URL-адресах перед выполнением любого запроса:

```text wrap theme={null}
WebFetch cannot fetch localhost or other hostnames without a dot. To reach a local server, use Bash with curl instead.
```

**Что делать:**

* Обычно ничего: сообщение указывает Claude на `curl` через инструмент Bash, который может достичь локальных и интранет-серверов

До версии 2.1.268 WebFetch сообщал об этих URL-адресах с общей ошибкой `Invalid URL`.

<h2 id="background-session-errors">
  Ошибки фоновых сеансов
</h2>

[Фоновые сеансы](/docs/ru/agent-view) работают без собственного интерактивного терминала, поэтому команды, которым он требуется, ведут себя там иначе. Эти сообщения появляются в стенограмме фонового сеанса, в терминале, подключённом к нему, в сеансе или оболочке, из которой вы его запустили, или, для [записей worktree-guard](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) ниже, в любом сеансе, изолированном в worktree, или в работающем изолированном подагенте worktree; где сообщение относится к одной поверхности, его запись это указывает.

<h3 id="commands-refused-in-a-background-session">
  Команды, отклонённые в фоновом сеансе
</h3>

Команды, которые открывают интерактивный диалог, не могут это делать, пока к фоновому сеансу не подключён терминал. `/install-github-app`, список параметров `/mcp` и действия аутентификации в меню сервера MCP отвечают сообщением, и сеанс появляется под **Needs input** в [представлении агента](/docs/ru/agent-view), чтобы вы могли его найти, подключиться и запустить команду снова. Пока терминал подключён, эти команды работают нормально.

До версии 2.1.216 сеанс не появлялся под **Needs input** после одного из этих отказов. В версиях 2.1.213–2.1.215 команды всё ещё работали, пока был подключён терминал, и сообщение об отказе говорило вам подключиться и запустить команду снова. С версии 2.1.208 по 2.1.212 Claude Code отклонял их даже при подключённом терминале с сообщением вроде `Can't open MCP settings in a background session`; на этих версиях запустите команду из обычного сеанса `claude` вместо этого или обновитесь. До версии 2.1.208 они открывали свой диалог внутри фонового сеанса. Только в версии 2.1.208 Claude Code также отклонял средство выбора `/model` в фоновом сеансе, и `/upgrade` выводил URL обновления вместо открытия браузера.

Формулировка называет команду. Список параметров `/mcp` сообщает:

```text theme={null}
Can't open MCP settings while no terminal is attached to this background session. This session now shows "needs input" in agent view — open it and run /mcp to manage servers, or use `/mcp enable|disable|reconnect <server>` to steer without the panel.
```

**Что делать:**

* Подключитесь к сеансу из представления агента, где он указан под **Needs input**, и запустите команду снова
* Или используйте форму, которую называет сообщение, например `/mcp reconnect <server>`, `/mcp enable` или `/mcp disable`, которые работают без подключения

<h3 id="write-or-command-blocked-because-the-path-cannot-be-safely-resolved">
  Запись или команда заблокирована, потому что путь не может быть безопасно разрешён
</h3>

Claude обратился к файлу или рабочему каталогу через написание, которое [защита изоляции worktree](/docs/ru/agent-view#how-file-edits-are-isolated) не может разрешить в одно проверяемое место. Защита проверяет записи и рабочие каталоги команд в [любом сеансе, изолированном в worktree](/docs/ru/worktrees#how-claude-code-enforces-isolation), интерактивном или фоновом, и в [изолированных подагентах worktree](/docs/ru/worktrees#isolate-subagents-with-worktrees). Она разрешает символические ссылки перед проверкой того, что операция не достигает общей проверки, и когда разрешение не удаётся, она блокирует операцию вместо того, чтобы позволить ей туда попасть. Сообщение называет формы пути, которые она отклоняет, и как повторить попытку:

```text theme={null}
This write was blocked because the path is spelled in a form that cannot be safely resolved (for example through a symlink storing a raw dot segment, a network-share or device-namespace shape, or an unreadable ancestor directory). If the file is inside the worktree /path/to/worktree, address it by its direct symlink-free path instead.
```

Заблокированная команда сообщает ту же причину для своего рабочего каталога и заканчивается `re-run the command from its direct symlink-free path`. До версии 2.1.217 защита сравнивала написания пути без разрешения символических ссылок, поэтому эти написания не блокировались и запись, маршрутизированная через символическую ссылку, могла попасть в общую проверку.

**Что делать:**

* Обычно ничего: полное сообщение идёт к Claude как ошибка инструмента, и Claude повторяет попытку с прямым путём, который он называет. Для заблокированного редактирования файла представление разговора показывает только короткую строку `Error editing file`; полное сообщение появляется в представлении стенограммы, которое вы открываете с помощью `Ctrl+O`. Заблокированная команда выводит его в выходные данные команды.
* Если блокировка повторяется на том же файле, путь, вероятно, проходит через зафиксированную символическую ссылку, цель которой содержит `..`, например `docs/current -> ../README.md`; попросите Claude отредактировать целевой файл по его реальному пути вместо ссылки

<h3 id="write-or-command-blocked-because-the-path-names-a-network-location">
  Запись или команда заблокирована, потому что путь называет сетевое расположение
</h3>

Claude обратился к файлу или рабочему каталогу через путь, который называет диск, которого нет на вашей машине, общую папку UNC, например `\\server\share\file`, или путь автомонтирования `/net`, в то время как проверка сеанса находится на локальном диске. Та же [защита изоляции worktree](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) не может проверить, что такой путь остаётся вне общей проверки, поэтому она блокирует операцию. Изоляция сеанса в worktree не снимает блокировку. Сообщение называет форму пути для использования вместо этого:

```text theme={null}
This write was blocked because the path is network-shaped (a UNC share or /net automount spelling) while this session's checkout is local. Isolating cannot unblock it. If the file is genuinely inside the worktree /path/to/worktree, address it by its local, plainly-spelled path instead.
```

Заблокированная команда сообщает ту же причину для своего рабочего каталога и заканчивается `re-run the command from its local, plainly-spelled path`. До версии 2.1.217 защита сравнивала только текст пути, поэтому обращение к файлу внутри проверки через путь UNC или `/net` не блокировалось.

**Что делать:**

* Обычно ничего: Claude повторяет попытку с локальным написанием, которое просит сообщение
* Если файл находится на сетевом ресурсе, а не на локальном файле, написанном с сетевым путём, он находится вне локального рабочего пространства сеанса; отредактируйте его из обычного интерактивного сеанса вместо этого

<h3 id="command-blocked-by-the-worktree-isolation-checks">
  Команда заблокирована проверками изоляции worktree
</h3>

Claude запустил команду Bash или Monitor в [сеансе, изолированном в worktree](/docs/ru/worktrees#how-claude-code-enforces-isolation), и Claude Code отклонил её по одной из двух причин:

* Команда указывает git на основную проверку.
* Claude Code не может проверить из текста команды, что любой git, который запускает команда, остаётся внутри worktree. Команда, которая никогда не называет git, всё ещё может быть отклонена по этой причине, потому что расширение косвенного обращения переменной, такого как `${!name}`, или запуск подстановки функции Bash, такой как `${ command; }`, производит значение во время выполнения, которое само может быть командой.

Середина сообщения называет то, что не могло быть проверено:

```text wrap theme={null}
This session is isolated in the worktree /path/to/worktree, but this command evaluates ${!x@P} arithmetically inside a construct too complex to verify, which can run a command hidden in a variable's value. Refusing to run it — a worktree-isolated session's git operations must target its own worktree. Split it into plain, separate commands and run them from /path/to/worktree.
```

**Что делать:**

* Обычно ничего: Claude читает сообщение и переписывает команду так, как просит его последнее предложение
* Если команда, которую вы просили, продолжает отклоняться, напишите отмеченное значение буквально: замените косвенное обращение или подстановку его значением и запустите git как отдельную простую команду из внутри worktree
* Чтобы действовать на основной проверке намеренно, запустите команду самостоятельно в терминале вне сеанса

<h3 id="this-session-has-no-saved-transcript">
  Этот сеанс не имеет сохранённой стенограммы
</h3>

Вы подключились к остановленному [фоновому сеансу](/docs/ru/agent-view), который был отправлен в фон из другого разговора с `←` или `/background` и остановлен до завершения его первого ответа. До завершения этого первого ответа разговор всё ещё существует только в сеансе, из которого он был отправлен в фон, поэтому `claude attach` отказывает в запуске остановленного сеанса вместо того, чтобы начать пустой разговор с тем же ID сеанса. Сообщение заканчивается командой `claude respawn` для этого сеанса:

```text theme={null}
This session has no saved transcript — it was stopped before its first response finished. If it was backgrounded from another conversation, that one is still intact; `claude respawn <id>` starts this one fresh.
```

Открытие той же строки сеанса в [представлении агента](/docs/ru/agent-view) показывает `Press enter again to restart this session fresh` ниже списка, и второе нажатие `Enter` на строке перезапускает сеанс с пустым разговором. До версии 2.1.212 открытие строки показывало сообщение об отказе без способа перезапуска из представления агента. До версии 2.1.211 открытие остановленного сеанса молча запускало этот пустой разговор и могло повторно запустить исходный запрос сеанса.

**Что делать:**

* Разговор, который вы отправили в фон, остаётся нетронутым: возобновите его с помощью [`claude --resume`](/docs/ru/sessions) или продолжайте работать в нём
* Чтобы запустить остановленный сеанс заново, запустите `claude respawn <id>` с ID из сообщения или нажмите `Enter` дважды на его строке в представлении агента
* Если сеанс завершил ответ и вы всё ещё видите этот отказ в версии до 2.1.214, нечитаемая папка в `~/.claude/projects` могла заставить сканирование стенограммы пропустить сохранённый разговор; обновитесь до версии 2.1.214 или позже, которая допускает нечитаемые папки во время сканирования

<h3 id="this-session-is-running-in-another-terminal">
  Этот сеанс работает в другом терминале
</h3>

Вы открыли строку остановленного сеанса в [представлении агента](/docs/ru/agent-view), и его сохранённый разговор уже открыт в другом живом процессе Claude Code на этой машине, поэтому Claude Code отказывает в запуске второго процесса, который писал бы в ту же стенограмму. Какое сообщение вы видите, зависит от [того, что держит разговор](/docs/ru/agent-view#opening-a-session-says-the-conversation-is-already-open):

```text theme={null}
Can't open — this session is running in another terminal
This conversation is already open in another running Claude session — use that one, or close it and try again
```

* **`running in another terminal`**: терминал держит разговор, например тот, где вы возобновили его с помощью `claude --resume` или `/resume`. Строка также показывает `Open in a terminal`.
* **`already open in another running Claude session`**: другой неинтерактивный процесс Claude Code держит его, например процесс [фонового сеанса](/docs/ru/agent-view#the-supervisor-process) для того же разговора, который ещё не вышел.

Claude Code сохраняет ответ, который вы напечатали при открытии строки, и отправляет его как следующий запрос сеанса, когда сеанс в следующий раз запустится.

**Что делать:**

* Продолжите разговор в процессе, который его имеет, или выйдите из этого процесса и откройте строку снова

До версии 2.1.248 существовал только отказ `already open in another running Claude session`: разговор, возобновленный в терминале, не считался открытым, и открытие строки запускало второй процесс Claude Code, пишущий в тот же разговор.

<h3 id="this-sessions-saved-conversation-is-no-longer-on-disk">
  Сохранённый разговор этого сеанса больше не находится на диске
</h3>

Вы открыли [фоновый сеанс](/docs/ru/agent-view), который завершился, пока фоновая служба была отключена, и [очистка стенограммы](/docs/ru/settings-reference#cleanupperioddays) с тех пор удалила его сохранённый разговор, например после того, как машина была отключена в течение недель. Открытие такой строки обычно [возобновляет её сохранённый разговор](/docs/ru/agent-view#sessions-show-as-failed-after-shutdown). Когда нечего возобновлять, Claude Code отказывает вместо того, чтобы повторно запустить исходный запрос сеанса без вопроса:

```text theme={null}
This session's saved conversation is no longer on disk (it ended while the background service was off, and old transcripts are cleaned up), so there is nothing to resume. `claude rm 7c5dcf5d` deletes the row; `claude respawn 7c5dcf5d` runs its original prompt again instead.
```

`claude attach <id>` выводит этот текст. В представлении агента нижний колонтитул короче и заканчивается `ctrl+x deletes the row`.

**Что делать:**

* Запустите `claude rm <id>` для удаления строки. Когда применяется один из [сохраняемых случаев](/docs/ru/agent-view#what-deleting-a-session-removes), `claude rm` сохраняет строку и worktree вместо этого и называет причину
* Чтобы запустить исходный запрос сеанса снова как новый разговор, запустите `claude respawn <id>`

До версии 2.1.248 открытие такой строки повторно запускало исходный запрос сеанса вместо отказа, вытягивая задачу, которая была неделю назад, на передний план.

<h3 id="worktree-has-commits-that-are-not-pushed-anywhere">
  Worktree имеет коммиты, которые не отправлены никуда
</h3>

Вы попытались удалить [фоновый сеанс](/docs/ru/agent-view#what-deleting-a-session-removes), чей worktree содержит коммиты, которые Claude Code не может подтвердить, что они сохранены где-то ещё. Claude Code сохраняет worktree и строку сеанса вместо того, чтобы уничтожить коммиты без присмотра. `claude rm` называет ветвь и неотправленные коммиты и говорит, как действовать:

```text theme={null}
kept 7c5dcf5d — its worktree is still at "/home/you/project/.claude/worktrees/fix-login"
  2 unpushed commits on "claude/fix-login": a1b2c3d "Fix login flow" and 1 more. They exist on no remote, so deleting the worktree would lose them.
  push them and run 'claude rm 7c5dcf5d' again, or discard the worktree and its commits: claude rm 7c5dcf5d --discard-unpushed a1b2c3d000000000000000000000000000000000@0123456789abcdef0123456789abcdef
```

Когда Claude Code не может суммировать коммиты, сообщение читается `The worktree has unpushed commits` вместо этого. В [представлении агента](/docs/ru/agent-view) строка сеанса показывает `not deleted` с той же причиной.

Коммиты на удалённом хранилище не блокируют удаление. Также не блокируют коммиты на локальной копии ветви по умолчанию вашего удалённого хранилища `origin`, пока эта ветвь проверена в вашей основной проверке, в самом каталоге репозитория, а не в worktree.

**Что делать:**

* Чтобы сохранить коммиты, отправьте ветвь worktree или объедините её в ветвь по умолчанию, проверенную в вашей основной проверке, затем удалите сеанс снова
* Чтобы отклонить коммиты, запустите команду `claude rm <id> --discard-unpushed`, которую выводит сообщение, или нажмите `Ctrl+X` дважды на строке сеанса в представлении агента снова. Это удаляет сеанс и worktree вместе с его ветвью, неотправленными коммитами и любыми незафиксированными изменениями. Если worktree получил коммит с момента отказа, Claude Code сохраняет его снова и показывает обновленное состояние
* Когда сообщение говорит, что worktree также записан другим завершённым сеансом, удаление снова не отклоняет его: отправьте коммиты, затем удалите сеанс снова

До версии 2.1.268 `claude rm` помещал сводку коммитов на саму строку `kept`. Когда `claude rm` не мог суммировать коммиты, строка `kept` читалась `worktree has commits that are not pushed anywhere` вместо сводки.

До версии 2.1.260 сообщение не называло ветвь или коммиты, и удаление снова отклонялось так же: удаление сеанса без отправки означало удаление worktree самостоятельно с помощью `git worktree remove --force <path>`, затем запуск `claude rm <id>` снова.

До версии 2.1.248 ветвь по умолчанию, проверенная в вашей основной проверке, не считалась: ветвь, которую вы уже объединили там, всё ещё вызывала этот отказ, пока её коммиты не достигали удалённого хранилища.

<h3 id="terminal-host-process-died">
  Процесс хоста терминала умер
</h3>

Каждый терминал [фонового сеанса](/docs/ru/agent-view) работает в процессе хоста под фоновой службой, и этот процесс умер, пока служба всё ещё держала его соединение, поэтому сеанс не мог быть достигнут.

На Linux и WSL фоновая служба проверяет каждый процесс хоста каждые несколько секунд, отмечает сеанс как неудачный, когда процесс вышел, но его соединение со службой никогда не закрывалось, и показывает причину на его строке в [представлении агента](/docs/ru/agent-view#read-session-state):

```text theme={null}
terminal host process died — press Enter to restart
```

Если вы откроете строку до запуска проверки, нижний колонтитул показывает `This session's terminal host process died (the conversation is saved) — press Enter to restart it` и строка становится неудачной.

Из оболочки `claude attach <id>` перезапускает сеанс, уже отмеченный как неудачный для мёртвого хоста, и в противном случае выводит причину и выходит:

```text theme={null}
Couldn't attach to <id> — This session's terminal host process died (the conversation is saved) — run `claude attach <id>` again to restart it on a fresh host.
```

Разговор сохраняется в любом случае.

Строка, работающая с [командой оболочки](/docs/ru/agent-view#run-a-shell-command) вместо этого, показывает `terminal host process died — its output is gone; the command was not run again`, и `claude attach` выводит `This command's terminal host process died — its output is gone and the command was not run again`. Claude Code никогда не повторно запускает команду для вас.

**Что делать:**

* В представлении агента нажмите `Enter` на неудачной строке; сеанс перезапускается на свежем процессе хоста и разговор возобновляется
* Из оболочки запустите `claude attach <id>` снова. Claude Code выводит `Session <id>'s terminal host died — restarting it on a fresh one…` и повторно открывает сеанс
* Вы не можете перезапустить строку shell-command таким образом; отправьте команду снова для её повторного запуска

До версии 2.1.247 мёртвый процесс хоста мог пройти каждую проверку живучести, которую запускала фоновая служба, поэтому открытие сеанса показывало `opening… · esc to cancel` бесконечно и `claude attach <id>` ждал без сообщения об ошибке.

<h3 id="session-isnt-responding">
  Сеанс не отвечает
</h3>

Вы открыли [фоновый сеанс](/docs/ru/agent-view) и фоновая служба приняла открытие, но никакой выход не прибыл примерно за десять секунд, поэтому Claude Code заключает, что процесс, передающий терминал сеанса, не может доставить выход, и заканчивает попытку вместо ожидания.

В представлении агента Claude Code предлагает перезапуск в нижнем колонтитуле:

```text theme={null}
Press enter again to restart this session — it isn't responding (its conversation is saved and resumes).
```

Из оболочки `claude attach <id>` выводит причину и выходит:

```text theme={null}
Couldn't attach to <id> — Session isn't responding — `claude stop <id>`, then `claude attach <id>` restarts it (the conversation is saved).
```

Claude Code никогда не перезапускает строку, работающую с [командой оболочки](/docs/ru/agent-view#run-a-shell-command) для вас, потому что перезапуск запустил бы команду снова.

**Что делать:**

* В представлении агента нажмите `Enter` на той же строке снова. Claude Code останавливает неответивший процесс и перезапускает сеанс, и разговор возобновляется. Ничего не останавливается без этого второго нажатия
* Из оболочки запустите `claude stop <id>`, затем `claude attach <id>`
* Для строки shell-command нажмите `Ctrl+X` в представлении агента или запустите `claude stop <id>` для её остановки; отправьте команду снова для её повторного запуска

<h3 id="session-was-stopped-while-the-respawn-was-in-flight">
  Сеанс был остановлен, пока перезапуск был в полёте
</h3>

Вы открыли [фоновый сеанс](/docs/ru/agent-view), чей процесс не работал, и пока Claude Code его перезапускал, другой процесс Claude Code остановил его, например `claude stop` в другом терминале. Claude Code держит сеанс остановленным:

```text theme={null}
Session <id> was stopped while the respawn was in flight
```

Открытие сеанса, который вы только что отправили, пока его процесс всё ещё запускается, ждёт процесса вместо этого. До версии 2.1.246 открытие его в этот момент могло остановить его и показать это сообщение.

**Что делать:**

* Если вы не остановили сеанс, откройте его строку снова в представлении агента или запустите `claude respawn <id>` для его перезапуска
* Если вы остановили его сами, ничего не остаётся делать: сеанс остаётся остановленным

<h3 id="session-agent-no-longer-available">
  Агент сеанса больше не доступен
</h3>

Вы возобновили сеанс, который работал [пользовательским агентом](/docs/ru/sub-agents#invoke-subagents-explicitly), запущенным с `--agent` или параметром `agent`, и Claude Code не нашёл агента с таким именем. Он сначала ищет в исходном каталоге сеанса, когда вы [доверили это рабочее пространство](/docs/ru/permissions#project-allow-rules-and-workspace-trust), затем в каталоге, из которого вы возобновляете. Сеанс всё ещё возобновляется, но с инструментами по умолчанию, поэтому ограничения инструментов агента больше не применяются:

```text theme={null}
This session was running agent 'code-reviewer', which is no longer available (no agent by that name in /home/you/project). Continuing with the default tools and system prompt — the agent's tool restrictions no longer apply. To restore it, re-create the agent, or resume with an explicit --agent <name>.
```

Предупреждение называет только каталоги, которые Claude Code искал, и оно появляется в возобновленном разговоре, пробудите ли вы [фоновый сеанс](/docs/ru/agent-view), запустите `/resume` или `claude --resume`, или возобновите в [неинтерактивном режиме](/docs/ru/headless), где оно также идёт в stderr. Сеансы, использующие `--input-format stream-json`, не показывают его, потому что Agent SDK поставляет агентов после запуска.

Claude Code не сохраняет откат в сеанс, поэтому предупреждение повторяется при каждом возобновлении, пока вы не действуете. Встроенный агент `claude` не вызывает предупреждение, так как откат к инструментам по умолчанию ничего не меняет для него. До версии 2.1.216 Claude Code молча продолжал как агент по умолчанию, и поиск охватывал только каталог, из которого вы возобновляли, поэтому агент, ограниченный проектом, был потерян при любом возобновлении из другого каталога.

**Что делать:**

* Пересоздайте файл агента в `.claude/agents/<name>.md` в проекте сеанса или в `~/.claude/agents/<name>.md` для личного агента, затем возобновите снова
* Или возобновите с `--agent <name>`, называя агента, который действительно существует, для запуска сеанса как этого агента вместо этого
* Если агент ограничен проектом и вы не доверили исходному каталогу сеанса, запустите Claude Code там один раз, примите диалог доверия, затем возобновите снова

<h3 id="claude_code_process_wrapper-launcher-errors">
  Ошибки средства запуска CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/ru/corporate-launcher) установлен, и его значение не может быть использовано, поэтому Claude Code отказывает в запуске затронутого процесса вместо того, чтобы запустить его без средства запуска. Проблемы конфигурации сообщаются сообщением, которое начинается с имени переменной и указывает причину, например:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Средство запуска, которое запускается, но выходит без замены себя на Claude Code, не удаётся сеансу, который оно запускало, и строка сеанса в представлении агента сообщает, что средство запуска `must exec, not daemonize`, за которым следует всё, что выводило средство запуска. Сеанс, который не может запуститься или достичь фоновую службу из-за средства запуска, сообщает проблему средства запуска как причину внутри `Couldn't reach the background service (...)`.

**Что делать:**

* Установите переменную на абсолютный путь исполняемого файла, который заканчивается вызовом `exec "$@"`. Смотрите [контракт средства запуска](/docs/ru/corporate-launcher#the-launcher-contract) для полного контракта
* Проверьте `/status`, который показывает разрешённую команду запуска в его записи Self-exec и предупреждает, когда работающая фоновая служба не совпадает с ней, или запустите `claude daemon status` из оболочки
* После исправления значения в блоке `env` [параметров](/docs/ru/corporate-launcher#set-up-the-launcher), перезапустите фоновую службу с помощью `claude daemon stop --any`, чтобы следующая отправка запустила завёрнутую

<h3 id="eunknown-when-starting-a-background-session">
  EUNKNOWN при запуске фонового сеанса
</h3>

Windows отказала в запуске программы с кодом ошибки, который не имеет стандартного имени, поэтому сбой появляется как `EUNKNOWN`. Обычный триггер — это политика ограничения программного обеспечения, такая как Group Policy или AppLocker, блокирующая запускаемую программу. Ошибка появляется, когда вы запускаете [фоновый сеанс](/docs/ru/agent-view) с `/background` или `claude --bg`:

```text theme={null}
Couldn't reach the background service (spawn background service: EUNKNOWN: unknown error, uv_spawn) — run 'claude daemon status'
```

На некоторых учётных записях сообщение говорит `daemon` вместо `background service`.

При установке npm ошибка `EUNKNOWN`, которая появляется, пока `npm install -g @anthropic-ai/claude-code` заменяет двоичный файл, имеет ту же причину, что и [`EACCES` при переустановке](#eacces-when-starting-a-background-session), и очищается, когда вы повторяете попытку после завершения установки.

Claude Code запускает фоновую службу через PowerShell, чтобы служба пережила закрытие терминала, используя PowerShell 7, когда он установлен, и Windows PowerShell 5.1 в противном случае. Когда ни один PowerShell не может работать, Claude Code запускает службу напрямую вместо этого, поэтому политика, которая блокирует только PowerShell, не вызывает эту ошибку. Если вы видите её, пока не запущена установка npm, политика блокирует сам исполняемый файл Claude Code.

До версии 2.1.212 Claude Code использовал только Windows PowerShell 5.1 для запуска службы, поэтому любая машина, где Group Policy блокировал PowerShell 5.1, не удавалась с `Couldn't start the session — EUNKNOWN: unknown error, uv_spawn`, даже с установленным PowerShell 7.

**Что делать:**

* Если сообщение читается `Couldn't start the session`, обновитесь до версии 2.1.212 или позже. На более ранних версиях вы также можете запустить `claude daemon run` в отдельном терминале сначала, затем запустить фоновый сеанс снова. Эта команда запускает фоновую службу на переднем плане терминала, поэтому служба длится только столько, сколько этот терминал остаётся открытым.
* Если установка npm заменяла двоичный файл, дождитесь её завершения, затем запустите фоновый сеанс снова
* Если ошибка появляется в версии 2.1.212 или позже, пока не запущена установка npm, попросите администратора Windows разрешить исполняемый файл Claude Code в политике ограничения
* Если фоновая служба останавливается при закрытии терминала, Claude Code запустил её без PowerShell. Установите PowerShell 7 или попросите администратора разблокировать PowerShell, чтобы служба могла пережить терминал.

<h3 id="eacces-when-starting-a-background-session">
  EACCES при запуске фонового сеанса
</h3>

Claude Code не мог запустить свой собственный двоичный файл для запуска [фоновой службы](/docs/ru/agent-view#the-supervisor-process), которая размещает фоновые сеансы. При установке npm это обычно означает, что `npm install -g @anthropic-ai/claude-code` заменял двоичный файл в этот момент, запустили ли вы его или [автоматическое обновление](/docs/ru/setup#auto-updates) сделало это. Ошибка появляется, когда вы открываете сеанс из [представления агента](/docs/ru/agent-view):

```text theme={null}
Couldn't start the background service — spawn background service: EACCES: permission denied, posix_spawn '/usr/local/lib/node_modules/@anthropic-ai/claude-code/bin/claude'
```

Когда вы запускаете сеанс с `/background` или `claude --bg`, та же причина появляется внутри `Couldn't reach the background service (...)`. Во время того же окна переустановки ошибка может называть другой код вместо этого, например `ENOENT` или `ENOEXEC`, или `EUNKNOWN` или `EPERM` на Windows; `EUNKNOWN`, который сохраняется при повторных попытках, имеет [другую причину](#eunknown-when-starting-a-background-session).

При установке npm Claude Code ждёт завершения переустановки и повторяет попытку самостоятельно: до десяти секунд и до двух минут, пока установка npm Claude Code явно всё ещё работает на машине, что охватывает другой процесс Claude Code, загружающий обновление. Когда установка превышает это ожидание, сбой называет обновление вместо простого кода ошибки:

```text theme={null}
Claude Code is being updated by npm on this machine (still not runnable after 2 min, EACCES) — try again when the update finishes
```

До версии 2.1.257 ожидание остановилось в десять секунд в каждом случае, поэтому эта ошибка появилась, пока другой процесс Claude Code всё ещё загружал обновление. До версии 2.1.246 Claude Code не удавался сразу же, без ожидания.

**Что делать:**

* Подождите несколько секунд, затем откройте сеанс или отправьте снова. Когда сообщение говорит, что Claude Code обновляется, повторите попытку после завершения обновления.
* Если ошибка сохраняется, пока не запущена установка npm, ваш пользователь не может запустить установленный двоичный файл. Проверьте его разрешения и его каталога, или переустановите Claude Code.

<h3 id="background-service-exited-before-it-became-reachable">
  Фоновая служба вышла до того, как стала доступной
</h3>

Процесс, который Claude Code запустил как [фоновую службу](/docs/ru/agent-view#the-supervisor-process), вышел до того, как он принял соединения, поэтому Claude Code не мог открыть ваш сеанс. Когда служба выводила ошибку перед выходом, причина в скобках даёт код выхода или сигнал и первую строку, которую служба выводила, которая называет то, что её остановило:

```text theme={null}
Couldn't reach the background service (background service exited before it became reachable (exit code N): <the service's first error line>) — run 'claude daemon status'
```

Когда вы открываете сеанс из [представления агента](/docs/ru/agent-view), та же причина следует `Couldn't start the background service —`. Когда служба ничего не выводила перед выходом, сообщение говорит `nothing on stderr` вместо этого.

Claude Code сообщает об ошибке с линией ошибки службы. До версии 2.1.246 ошибка появлялась только после ожидания в 45 секунд, как `background service did not become reachable within 45s`, без линии ошибки службы.

Две цитируемые причины имеют известные причины:

* `Error: claude native binary not installed.`: установка npm заменяла двоичный файл Claude Code в этот момент, поэтому служба запускала заполнитель npm вместо этого. Повторите попытку после завершения установки; если строка сохраняется без запущенной установки, [завершите установку npm](/docs/ru/troubleshoot-install#native-binary-not-found-after-npm-install). До версии 2.1.257 самообновление npm на macOS производило эту ошибку при каждом запуске во время окна установки.
* `nothing on stderr` с кодом выхода 1, при каждом запуске, на Windows: `daemon.lock` называет процесс, который Claude Code не может ни сигнализировать, ни доказать, что он ушёл, поэтому каждая новая служба заключает, что другая держит блокировку и выходит. Блокировка, чей писатель Claude Code может доказать, что ушёл, заменяется самостоятельно и не производит эту ошибку. Когда ошибка повторяется при каждом запуске, удалите `~/.claude/daemon.lock`, затем откройте сеанс или отправьте снова. До версии 2.1.257 такая блокировка блокировала каждый запуск, пока вы не удалили файл.

**Что делать:**

* Если сообщение цитирует строку, исправьте то, что она называет, затем откройте сеанс или отправьте снова. Следующая попытка запускает службу снова
* Запустите `claude daemon status` для проверки, работает ли служба сейчас

<h3 id="working-directory-no-longer-exists-when-starting-a-background-session">
  Рабочий каталог больше не существует при запуске фонового сеанса
</h3>

Вы попытались запустить [фоновый сеанс](/docs/ru/agent-view) в каталоге, который больше не существует. Это происходит, когда вы отправляете из представления агента или запускаете `/background` после того, как каталог, в котором вы работаете, был удалён или перемещён. Это также происходит, когда вы подключаетесь к или перезапускаете сеанс, чей процесс вышел и чей каталог ушёл, потому что новый процесс запустился бы в том же каталоге. Claude Code не запускает сеанс, и сообщение называет отсутствующий каталог:

```text theme={null}
Couldn't start a background session (working directory no longer exists or is not accessible: /tmp/demo)
```

До версии 2.1.257 сеанс казался запущенным и затем показывался в представлении агента как неудачная строка с той же причиной.

**Что делать:**

* Пересоздайте каталог, который называет сообщение, или отправьте из каталога, который существует, затем попробуйте снова

<h2 id="wrapper-and-ide-errors">
  Ошибки обёртки и IDE
</h2>

Эти ошибки исходят от программы, которая запустила Claude Code для вас, такой как расширение IDE или приложение [Agent SDK](/docs/ru/agent-sdk/overview), а не от самого Claude Code.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code process exited with code N
</h3>

Базовый процесс `claude` завершился с ненулевым кодом. Сам код выхода не говорит, что не удалось: реальная ошибка находится в собственном выводе процесса, который обёртка добавляет, если она что-то захватила, в противном случае она хранит это в своих логах.

```text theme={null}
Error: Claude Code process exited with code 1
```

На Windows встроенная сборка может завершиться с кодом `4294967295` сразу после завершения хода. Когда этот выход происходит на границе хода, без ожидающего сообщения и без выполняющейся фоновой задачи, [расширение VS Code](/docs/ru/vs-code) закрывает сеанс без уведомления вместо отображения этой ошибки. Ваше следующее сообщение возобновляет разговор.

До версии 2.1.273 расширение показывало ошибку для этого выхода на каждой границе хода, хотя ничего не было потеряно.

**Что делать:**

* В VS Code перейдите по ссылке **View output logs**, показанной с ошибкой, чтобы увидеть основную ошибку
* В приложении Agent SDK перехватите ошибку вокруг вашего цикла сообщений. Записи в разделе [CLI process exit](/docs/ru/agent-sdk/troubleshooting#cli-process-exit) охватывают то, что ваш код получает в каждом языке SDK.
* Запустите `claude` в терминале в том же проекте. Ошибка обычно воспроизводится там с её реальным сообщением об ошибке, которое вы затем можете найти на этой странице.
* Запустите `claude doctor` в терминале, чтобы проверить установку и конфигурацию

<h3 id="could-not-locate-the-claude-cli-on-path">
  Could not locate the Claude CLI on PATH
</h3>

[Расширение VS Code](/docs/ru/vs-code) показывает эту ошибку в Windows, когда вы открываете Claude Code в интегрированном терминале, оболочка терминала — это PowerShell, и расширение не может найти установленный исполняемый файл `claude` в PATH. Расширение отказывается запускать Claude Code, пока не найдёт установленный `claude` в PATH.

```text theme={null}
Failed to run Claude Code: Error: Could not locate the Claude CLI on PATH. Launching by name in a PowerShell terminal would run a 'claude' from the open folder instead of the installed CLI, so the launch was blocked. Make sure the Claude CLI's install directory is on your system PATH (not only your PowerShell profile), then restart VS Code and try again. VS Code reads PATH when it starts, so PATH changes take effect only after a restart.
```

**Что делать:**

* Откройте новое окно PowerShell вне VS Code и запустите `where.exe claude`. Если оно не выводит путь, CLI не находится в вашем PATH: добавьте его каталог установки, следуя [Verify your PATH](/docs/ru/troubleshoot-install#verify-your-path). Если оно выводит путь, запись поступает из вашего профиля PowerShell или из изменения PATH, которое VS Code ещё не подхватил; следующие два шага охватывают эти случаи.
* Установите запись PATH как переменную окружения пользователя или системы, а не в вашем профиле PowerShell. Расширение не запускает ваш профиль, поэтому редактирование PATH, которое существует только там, никогда до него не доходит.
* Перезагрузите VS Code после изменения PATH. Расширение проверяет PATH, который VS Code захватил при запуске, поэтому изменение PATH вступает в силу только после перезагрузки.

<h3 id="the-connection-to-claude-code-ended-before-this-message-completed">
  The connection to Claude Code ended before this message completed
</h3>

[Расширение VS Code](/docs/ru/vs-code) отправило ваше сообщение процессу `claude`, и соединение завершилось без ошибки до того, как процесс подтвердил или завершил его. Расширение не может определить, было ли сообщение обработано, поэтому оно просит вас отправить его снова:

```text theme={null}
The connection to Claude Code ended before this message completed — it may not have been processed, so please send it again.
```

**Что делать:**

* Отправьте сообщение снова. Следующее сообщение запускает свежий процесс `claude`, который возобновляет разговор.
* Если это повторяется, запустите `claude` в терминале в том же проекте. Ошибка, которая продолжает завершать процесс, обычно воспроизводится там с её реальным сообщением об ошибке.

<h2 id="rewind-warnings-and-errors">
  Предупреждения и ошибки Rewind
</h2>

Эти сообщения поступают из восстановления кода [`/rewind`](/docs/ru/checkpointing). `Restored the code, but skipped N files` — это предупреждение о том, что Claude Code пропустил некоторые пути. `No files were restored` — это ошибка, которая означает, что ничего не было восстановлено.

<h3 id="restored-the-code-but-skipped-files">
  Restored the code, but skipped files
</h3>

Восстановление кода `/rewind` пропустило один или несколько отслеживаемых путей вместо того, чтобы записать или удалить их. Claude Code пропускает путь, когда:

* это символическая ссылка, жёсткая ссылка или другой нерегулярный файл, или он стал таким
* его директория изменилась с момента создания контрольной точки
* его резервную копию невозможно безопасно прочитать

Пропущенные пути сохраняют своё текущее содержимое. До версии 2.1.216 `/rewind` записывал и удалял данные через ссылки на отслеживаемых путях и не сообщал о частичном восстановлении.

```text theme={null}
Restored the code, but skipped 2 files: the tracked path is (or became) a link or other non-regular file, its directory changed since the checkpoint, or its backup could not be safely read. Skipped files were left untouched — run with --debug for the paths.
```

**Что делать:**

* Определите, какие файлы были пропущены, чтобы вы могли обработать каждый из них, следуя приведённым ниже шагам. Сообщение содержит только количество; журнал отладки в `~/.claude/debug/<session-id>.txt` указывает каждый пропущенный путь по мере выполнения восстановления, поэтому включите логирование отладки с помощью `/debug` перед следующим восстановлением. На macOS или Linux вы можете вместо этого найти ссылки напрямую: `find . -type l` для символических ссылок и `find . -type f -links +1` для жёстко связанных файлов.
* Если пропущенный файл — это ссылка, которую вы создали намеренно, например файл конфигурации, управляемый менеджером dotfile, или файл, жёстко связанный инструментами вроде pnpm, то rewind оставил его содержимое без изменений. Чтобы отменить изменения сеанса в этом файле, попросите Claude отменить редактирование или отредактируйте файл самостоятельно
* Если вы не создавали ссылку, проверьте путь перед тем, как доверять его содержимому: что-то заменило файл после создания контрольной точки

<h3 id="no-files-were-restored">
  No files were restored
</h3>

Claude Code показывает это сообщение, когда вы восстанавливаете код с помощью [`/rewind`](/docs/ru/checkpointing) и Claude Code не может восстановить ни один из файлов в этой контрольной точке. Для каждого файла либо резервная копия, которую Claude Code сохранил перед редактированием, отсутствует, либо Claude Code не смог записать или удалить файл.

```text theme={null}
Failed to restore the code:
No files were restored: 1 file failed (backup missing, or the file could not be updated)
```

Claude Code удаляет резервные копии сеанса при [очистке хранения](/docs/ru/claude-directory#cleaned-up-automatically), по умолчанию примерно через 30 дней после того, как сеанс последний раз их сохранил. Если вы возобновите сеанс после этого, `/rewind` по-прежнему будет отображать его контрольные точки, но восстановление одной из них может завершиться ошибкой. Если сообщение также говорит `N paths were skipped for link safety`, см. раздел [Restored the code, but skipped files](#restored-the-code-but-skipped-files) для этих путей.

Когда вы создаёте ветвь сеанса, например с помощью [`--fork-session`](/docs/ru/cli-reference#cli-flags) или [`/branch`](/docs/ru/sessions#branch-a-session), Claude Code копирует резервные копии исходного сеанса в ветвь. Когда Claude Code не может скопировать резервную копию, например потому что диск заполнен, эта резервная копия отсутствует в ветви. Восстановление контрольной точки, которая её требует, может завершиться этой ошибкой.

**Что делать:**

* Отмените изменения другим способом: попросите Claude отменить его редактирование или восстановите файлы из системы контроля версий. Когда резервные копии удалены, повторный запуск `/rewind` завершится с той же ошибкой.
* Если Claude Code не смог записать или удалить файл, исправьте то, что блокирует запись, например разрешения на файл, а затем снова запустите `/rewind`.
* Чтобы сохранять резервные копии дольше в будущих сеансах, увеличьте значение [`cleanupPeriodDays`](/docs/ru/settings-reference#cleanupperioddays).

До версии 2.1.260 Claude Code молча пропускал файлы, резервные копии которых отсутствовали, и восстановление казалось успешным.

<h2 id="session-saving-warnings">
  Предупреждения о сохранении сеанса
</h2>

Claude Code показывает эти предупреждения на постоянной строке ниже поля ввода, когда он не сохраняет транскрипт вашего сеанса. Сеанс продолжает работать в любом случае; предупреждения говорят вам, что сеанс может отсутствовать при [`--resume`](/docs/ru/sessions) позже.

<h3 id="transcript-writes-are-failing">
  Запись транскрипта не удаётся
</h3>

Claude Code сохраняет транскрипт на диск по мере работы, и его записи в [файл транскрипта](/docs/ru/sessions#where-transcripts-are-stored) не удаются. Сообщение указывает причину с кодом базовой ошибки, например полный диск:

```text theme={null}
Transcript writes are failing (disk full — ENOSPC) · recent messages may not be saved for resume
```

Предупреждение появляется в разных точках в зависимости от ошибки:

* При первом сбое для условий, которые не исчезают сами по себе: полный диск, превышена квота диска, файловая система только для чтения, путь превышает лимит длины файловой системы, или на macOS и Linux — ошибка разрешения
* После повторных сбоев, длящихся не менее минуты, для всего остального, включая ошибки разрешения на Windows, где сканирование антивируса может привести к сбою одной записи, которая затем успешно выполняется при повторной попытке

До версии 2.1.217 Claude Code отбрасывал неудачные записи без предупреждения, и позже отсутствие недавних сообщений при `--resume` было первым признаком.

**Что делать:**

* Исправьте условие, которое указывает код ошибки: освободите место на диске для `ENOSPC`; повысьте или очистите квоту для `EDQUOT`; восстановите доступ на запись в расположение транскрипта для `EACCES`, `EPERM` или `EROFS`
* Предупреждение исчезает само по себе при следующей успешной записи; перезагрузка не требуется
* Сообщения, отправленные во время отображения предупреждения, могут по-прежнему отсутствовать при возобновлении сеанса позже

<h3 id="transcript-saving-is-off-skip-prompt-history">
  Сохранение транскрипта отключено, потому что установлена переменная CLAUDE\_CODE\_SKIP\_PROMPT\_HISTORY
</h3>

Этот сеанс начался с установленной переменной [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/ru/env-vars), поэтому Claude Code не записывает для него транскрипт или историю подсказок:

```text theme={null}
Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set · --resume will not find this session; if unintended, unset it and restart
```

Переменная является преднамеренным отказом для эфемерных скриптовых сеансов, но она также может достичь сеанса через профиль оболочки, скрипт-обёртку или родительский процесс, который её экспортировал.

**Что делать:**

* Если вы установили переменную намеренно, никаких действий не требуется; уведомление подтверждает, что сеанс не будет отображаться в `--resume`, `--continue` или истории стрелки вверх
* Если нет, удалите переменную из оболочки или скрипта, который запускает `claude`, затем начните новый сеанс. Сообщения из текущего сеанса не сохраняются задним числом.

<h3 id="transcript-saving-is-off-child-session-marker">
  Сохранение транскрипта отключено из-за унаследованного маркера CLAUDE\_CODE\_CHILD\_SESSION
</h3>

Claude Code устанавливает [`CLAUDE_CODE_CHILD_SESSION`](/docs/ru/env-vars) в подпроцессах, которые он порождает, и рассматривает интерактивный сеанс, который его наследует, как вложенный: Claude Code не сохраняет для него транскрипт, поэтому сеансы, которые сам Claude запускает, не заполняют ваш список `--resume`. Это уведомление означает, что ваш текущий сеанс унаследовал маркер:

```text theme={null}
Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker · restart with CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1 to keep future transcripts
```

Уведомление ожидается, когда вы запустили `claude` изнутри другого сеанса Claude Code; оно сигнализирует о неправильной классификации, когда маркер просочился через долгоживущий посредник, например терминал, сеанс `screen` или средство запуска, которое первоначально запустил сеанс Claude Code.

Внутри tmux Claude Code обнаруживает маркер, который прибыл через глобальную среду сервера tmux, и продолжает сохранять, поэтому это уведомление не появляется для этого случая.

**Что делать:**

* Если вы запустили этот сеанс изнутри другого сеанса Claude Code намеренно, никаких действий не требуется
* Если это сеанс верхнего уровня, выйдите и перезагрузитесь с установленной переменной [`CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1`](/docs/ru/env-vars). Сохранение применяется с момента перезагрузки, поэтому сообщения, отправленные до неё, не сохраняются.
* Чтобы исправить будущие запуски из того же терминала или средства запуска, удалите `CLAUDE_CODE_CHILD_SESSION` из его среды

<h2 id="configuration-warnings">
  Предупреждения о конфигурации
</h2>

Claude Code записывает большинство этих сообщений в stderr, а не в беседу, и выводит большинство из них при запуске. Запись указывает, когда её сообщение появляется в другом месте, например в журнале отладки или как уведомление при запуске в представлении беседы, или в другое время, например [диагностическая строка unrecognized-model](#unrecognized-model-id-on-a-request) во время запроса.

<h3 id="fullscreen-failed-start-notice">
  Fullscreen renderer didn't finish starting
</h3>

Предыдущий сеанс [fullscreen](/docs/ru/fullscreen) на этом компьютере завершился до того, как закончил запускаться, поэтому Claude Code запускает этот сеанс на классическом renderer и выводит одно из этих уведомлений:

```text theme={null}
Claude Code's fullscreen renderer didn't finish starting last time on this machine, so this launch is using the classic renderer. It will try fullscreen again next launch; /tui default keeps the classic renderer.

Claude Code's fullscreen renderer has repeatedly failed to start on this machine, so it has been turned off here. Run /tui fullscreen to try it again (this also resets after an update).
```

**Что делать:**

* Следуйте инструкциям [Fullscreen rendering](/docs/ru/fullscreen#fullscreen-renderer-didnt-finish-starting). Там указано, какое уведомление вы получите, что Claude Code делает в последующих сеансах и как снова попробовать fullscreen или оставить классический renderer.
* Если сеанс, который завершился, вывел сообщение об выходе, см. [Claude Code exited after an unrecoverable interface error](#exited-after-an-unrecoverable-interface-error) для получения информации о том, что оно называет.

До версии 2.1.236 Claude Code не выводил уведомление и продолжал запускать сеансы в fullscreen rendering после неудачного запуска.

<h3 id="exited-after-an-unrecoverable-interface-error">
  Claude Code exited after an unrecoverable interface error
</h3>

Claude Code выводит это сообщение при выходе, потому что его интерфейс терминала столкнулся с ошибкой, от которой он не может восстановиться, в любом из renderer. Второе предложение появляется только когда ошибка произошла во время запуска [fullscreen](/docs/ru/fullscreen) renderer:

```text theme={null}
Claude Code exited after an unrecoverable interface error (<error>). It happened while the fullscreen renderer was starting, so the next launch will use the classic renderer (CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1 forces that any time).
```

**Что делать:**

* Запустите Claude Code снова. Чтобы продолжить беседу, выполните `claude --resume` в том же каталоге.
* Если сообщение называет fullscreen renderer, [Fullscreen rendering](/docs/ru/fullscreen#fullscreen-renderer-didnt-finish-starting) говорит, что делает следующий запуск, что зависит от того, как вы включили fullscreen, и как снова попробовать fullscreen или оставить классический renderer.

До версии 2.1.236 Claude Code выходил без вывода сообщения после этого вида ошибки.

<h3 id="agent-descriptions-are-over-the-15000-token-limit">
  Agent descriptions are over the 15.0k-token limit
</h3>

Claude Code показывает это предупреждение как уведомление при запуске в представлении беседы, а не на stderr. Объединённые описания ваших [subagents](/docs/ru/sub-agents), кроме встроенных, превышают 15 000 токенов в соответствии с оценкой Claude Code. Каждый агент считает своё имя плюс его frontmatter `description`. Claude Code загружает каждого агента независимо от того, превышен ли лимит, поэтому предупреждение не меняет то, что загружается.

```text theme={null}
Agent descriptions are over the 15.0k-token limit (~16.2k tokens) · ask Claude to trim agent descriptions in .claude/agents/
```

**Что делать:**

* Сократите frontmatter `description` ваших файлов агентов или попросите Claude сократить их для вас.
* Удалите файлы агентов, которые вы больше не используете.

<h3 id="workspace-has-not-been-trusted">
  Workspace has not been trusted
</h3>

Claude Code обнаружил правила `permissions.allow` или записи `permissions.additionalDirectories` в файле `.claude/settings.json` или `.claude/settings.local.json` проекта и не применил их, потому что [правила allow из параметров проекта требуют доверия рабочей области](/docs/ru/permissions#project-allow-rules-and-workspace-trust). Количество, имя параметра и имя файла в сообщении варьируются в зависимости от вашей конфигурации. На правила `deny` и `ask` это не влияет.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Что делать:**

* Выполните `claude` в каталоге и примите диалог доверия. [Project allow rules and workspace trust](/docs/ru/permissions#project-allow-rules-and-workspace-trust) говорит, какую папку охватывает это принятие.
* В [non-interactive mode](/docs/ru/headless) с `-p` диалог не показывается. Установите запись `hasTrustDialogAccepted` в `~/.claude.json`, используя точный ключ `projects`, который выводит сообщение.
* Если сообщение называет `.claude/settings.local.json` и вы запустили Claude Code вне репозитория git или в вашем домашнем каталоге, обновитесь до версии 2.1.200 или позже. Версии 2.1.196 по 2.1.199 рассматривали ваш собственный `.claude/settings.local.json` как предоставленный репозиторием в этих рабочих областях. На версии 2.1.207 и позже обновления недостаточно вне репозитория git, если вы не доверили папке: определение того, что папка не находится внутри репозитория, запускает git, и Claude Code запускает эту проверку только после того, как вы примете диалог доверия, поэтому используйте первый шаг. Ваш домашний каталог и любой другой [configuration home](/docs/ru/permissions#project-allow-rules-and-workspace-trust) освобождены и не ждут диалога. См. [Project allow rules and workspace trust](/docs/ru/permissions#project-allow-rules-and-workspace-trust).

<h3 id="working-directory-is-a-network-path">
  Working directory is a network path
</h3>

Claude Code не добавляет сетевые пути как рабочие каталоги. Поиск сетевого пути может связаться с хостом, который он называет, и на Windows этот контакт может отправить хосту ваши учётные данные, поэтому Claude Code отказывает в пути без его поиска. Вы видите это сообщение при выполнении `/add-dir` с таким путём или как предупреждение при запуске. Когда оно появляется при запуске, Claude Code запускается без этого каталога.

```text theme={null}
\\server\share is a network path, which cannot be added as a working directory. On Windows, map the share to a drive letter and pass it at launch with --add-dir (a drive letter added mid-session does not yet carry remote-read trust).
```

Пути, которые Claude Code отказывает таким образом, включают:

* UNC shares такие как `\\server\share`
* Automount paths такие как `/net/<host>`, если вы не запустили Claude Code из каталога под automount этого хоста
* Локальные пути, которые достигают сетевого расположения через символическую ссылку или junction

Сопоставленные буквы дисков и пути `\\wsl$` не считаются сетевыми путями.

**Что делать:**

* На Windows сопоставьте share с буквой диска, например с помощью `net use Z: \\server\share`, и передайте диск при запуске с `claude --add-dir Z:\`.
* На macOS или Linux смонтируйте share в локальный путь и добавьте этот путь вместо этого.
* Если путь находится в `permissions.additionalDirectories`, удалите его из файла параметров, который его перечисляет.

До версии 2.1.257 Claude Code принимал достижимый сетевой путь как рабочий каталог.

<h3 id="remote-managed-settings-failed-to-load">
  Remote managed settings failed to load
</h3>

Ваш сеанс имеет право на [server-managed settings](/docs/ru/server-managed-settings), но Claude Code не смог их получить, поэтому он показывает это предупреждение в интерактивных сеансах. Причина в скобках называет то, что не удалось, например `network error`, `request timed out` или `authentication rejected (401)`, а остальная часть строки говорит, какую политику запускает сеанс:

* **Settings cached from an earlier successful fetch**: Claude Code запускает сеанс на этой кэшированной политике, кроме [withheld environment variables](/docs/ru/server-managed-settings#fetch-and-caching-behavior), и строка читается `using cached policy`.
* **No cache**: Claude Code запускает сеанс без server-managed settings, и строка читается `no remote policy applied`.

**Что делать:**

* Действуйте в соответствии с причиной, которую называет сообщение: для сетевой причины проверьте, что эта машина может достичь `api.anthropic.com`; для причины аутентификации проверьте вашу регистрацию с помощью `/status`
* Выполните `/status` или `claude doctor` для полной диагностики

До версии 2.1.248 Claude Code сообщал о неудачной выборке параметров только в журнале отладки.

<h3 id="managed-settings-were-not-approved">
  Managed settings were not approved
</h3>

[server-managed settings](/docs/ru/server-managed-settings) вашей организации включают параметры, которые требуют вашего одобрения, и вы отклонили [security approval dialog](/docs/ru/server-managed-settings#security-approval-dialogs), поэтому Claude Code выходит без их применения:

```text theme={null}
Managed settings were not approved; exiting without applying them.
```

**Что делать:**

* Запустите Claude Code снова и одобрите диалог, чтобы продолжить в соответствии с параметрами вашей организации. Отклонённый диалог не запоминается, поэтому он появляется снова при следующем запуске.
* Если вы не уверены в параметре, который перечисляет диалог, спросите того, кто поддерживает управляемые параметры вашей организации, перед одобрением

<h3 id="mcp-server-is-blocked-by-enterprise-managed-policy">
  MCP server is blocked by enterprise managed policy
</h3>

Вы выбрали **Reconnect** на сервере в `/mcp` или снова включили отключённый сервер там, и параметр, который [restricts MCP servers](/docs/ru/managed-mcp) блокирует этот сервер. Claude Code отказывает в подключении и показывает:

```text theme={null}
MCP server <name> is blocked by enterprise managed policy
```

Любой из этих параметров может создать сообщение:

* Запись [`deniedMcpServers`](/docs/ru/managed-mcp#policy-based-control-with-allowlists-and-denylists), которая соответствует серверу, включая одну в вашем собственном `~/.claude/settings.json` или в `.claude/settings.json` проекта
* Список [`allowedMcpServers`](/docs/ru/managed-mcp#policy-based-control-with-allowlists-and-denylists), которому сервер не соответствует
* [`strictPluginOnlyCustomization`](/docs/ru/settings-reference#strictpluginonlycustomization) с заблокированным `mcp`, который блокирует серверы, настроенные в `~/.claude.json` и `.mcp.json`
* [`disableClaudeAiConnectors`](/docs/ru/mcp#disable-claude-ai-connectors), когда сервер является claude.ai connector

**Что делать:**

* Проверьте ваши собственные файлы параметров пользователя и проекта на наличие одного из этих параметров и измените или удалите его
* Если ни один из ваших собственных параметров не объясняет блокировку, спросите вашего администратора, какой управляемый параметр блокирует сервер

До версии 2.1.257 **Reconnect** и повторное включение в `/mcp` могли подключить сервер, который обновление политики в середине сеанса заблокировало.

<h3 id="managed-settings-document-could-not-be-parsed">
  Managed settings document could not be parsed
</h3>

Ваша организация развёртывает [managed settings](/docs/ru/managed-settings), и один из развёрнутых документов присутствует, но не может быть проанализирован как объект JSON, поэтому Claude Code выходит с кодом 1 при запуске вместо запуска без политики, которую несёт документ. Строка называет неудачный источник перед сообщением:

```text theme={null}
/Library/Application Support/ClaudeCode/managed-settings.json: Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.
```

Источник является одним из:

* Путь файла `managed-settings.json` или drop-in файла под `managed-settings.d`
* Профиль управляемых параметров macOS, `per-user managed preferences` или `device-level managed preferences`
* Значение реестра Windows, `Registry: HKLM\SOFTWARE\Policies\ClaudeCode\Settings`

[Find entries Claude Code dropped](/docs/ru/managed-settings#find-entries-claude-code-dropped) перечисляет то, что делает каждый источник непарсируемым.

Claude Code отказывает в запуске даже когда другой источник администратора доставляет действительную политику. Вы видите эту ошибку в интерактивных сеансах, `claude -p`, сеансах Agent SDK, [background sessions](/docs/ru/agent-view) и большинстве подкоманд, включая `claude doctor`. Отказ закрывается намеренно: параметры в документе, который Claude Code не может проанализировать, не могут быть применены, и запуск в любом случае запустил бы сеансы без элементов управления организации.

Проблема схемы в парсируемом документе не создаёт эту ошибку. [Find entries Claude Code dropped](/docs/ru/managed-settings#find-entries-claude-code-dropped) охватывает то, что Claude Code делает с одной.

Когда каталог `managed-settings.d/` существует, но не может быть перечислен, Claude Code сообщает `Managed settings drop-in directory could not be read:` с последующей базовой ошибкой вместо этого. [Find entries Claude Code dropped](/docs/ru/managed-settings#find-entries-claude-code-dropped) охватывает, когда отказ при чтении выходит при запуске.

**Что делать:**

* Если вы администрируете машину, исправьте названный документ так, чтобы он анализировался как объект JSON, или удалите файл, профиль или значение реестра. Пустой `managed-settings.json` считается `{}` и не блокирует запуск.
* Если нет, попросите вашего администратора исправить развёрнутый документ. Ничто в ваших собственных файлах параметров не вызывает и не очищает эту ошибку.

<h3 id="otelheadershelper-failed">
  otelHeadersHelper failed
</h3>

Claude Code показывает это предупреждение как уведомление в интерфейсе терминала, один раз за интерактивный сеанс, когда скрипт [`otelHeadersHelper`](/docs/ru/settings-reference#otelheadershelper) не удаётся или выводит результат, который не соответствует [требованиям скрипта](/docs/ru/monitoring-usage#script-requirements).

Пока скрипт продолжает не удаваться, экспорты не удаются и ваш backend телеметрии не получает ничего из сеанса.

Текст после `See /status:` говорит, что не удалось, например код выхода скрипта, за которым следует его вывод ошибки:

```text theme={null}
otelHeadersHelper failed; telemetry is not being exported. See /status: exited 1: token service unreachable
```

**Что делать:**

* Выполните `/status` для чтения деталей отказа.
* Исправьте скрипт так, чтобы он выходил 0 в течение 30 секунд и выводил объект JSON значений заголовков строк на stdout. См. [требования скрипта](/docs/ru/monitoring-usage#script-requirements).
* Если ваша организация развёртывает скрипт через [managed settings](/docs/ru/managed-settings), попросите того, кто их поддерживает, исправить это.

В [non-interactive mode](/docs/ru/headless) с `-p`, тот же отказ появляется на stderr как `otelHeadersHelper failed (OpenTelemetry export headers unavailable): <error>` вместо этого.

<h3 id="headershelper-not-run">
  headersHelper not run
</h3>

Claude Code подключил MCP сервер с его статическими `headers` только и пропустил [`headersHelper`](/docs/ru/mcp#use-dynamic-headers-for-custom-authentication) сервера, потому что помощник является shell командой и папка не имеет сохранённого доверия. Папка получает сохранённое доверие, когда вы устанавливаете её запись в `~/.claude.json` вручную или, вне вашего домашнего каталога, когда вы принимаете диалог доверия для неё в интерактивном сеансе. См. [Trust a folder before its headersHelper runs](/docs/ru/mcp#trust-a-folder-before-its-headershelper-runs) для того, какие серверы применяется эта проверка.

Claude Code записывает эту строку в [non-interactive mode](/docs/ru/headless) только, один раз на сервер. В интерактивном сеансе он записывает тот же отказ в журнал отладки вместо этого.

```text theme={null}
MCP server 'internal-api': headersHelper not run — this workspace has no persisted trust; accept the trust dialog here once interactively, or set projects["/Users/you/project"].hasTrustDialogAccepted in /Users/you/.claude.json.
```

Ключ `projects`, который выводит сообщение, является папкой, на которой [Project allow rules and workspace trust](/docs/ru/permissions#project-allow-rules-and-workspace-trust) говорит Claude Code ключи доверия. Принятие диалога доверия для родительской папки не удовлетворяет проверку, и сеанс `-p` или SDK не удовлетворяет её либо.

**Что делать:**

* Выполните `claude` в папке, которую называет сообщение, примите диалог доверия, затем выполните вашу команду `-p` или SDK снова
* Установите запись `hasTrustDialogAccepted` в `~/.claude.json` самостоятельно, используя точный ключ `projects`, который выводит сообщение
* Если вы запустили сеанс в вашем домашнем каталоге, работайте из каталога проекта, которому вы доверяете. Когда вы принимаете диалог доверия в вашем домашнем каталоге, Claude Code держит это доверие только для текущего сеанса.

<h3 id="malformed-tool-content-rule">
  Malformed Tool(content) rule
</h3>

[permission rule](/docs/ru/permissions#permission-rule-syntax) в одном из ваших файлов параметров не имеет формы `Tool` или `Tool(content)`, например потому что текст следует за закрывающей скобкой или одна из скобок отсутствует. Claude Code пропускает правило и перечисляет его в диалоге недействительных параметров при запуске интерактивного сеанса и в выводе [`claude doctor`](/docs/ru/debug-your-config#check-resolved-settings):

```text theme={null}
Invalid permission rule "Bash(ls) x" was skipped: Malformed Tool(content) rule. Rules take the form Tool or Tool(content) and must end at the closing ")"; parentheses inside the content are literal
```

**Что делать:**

* В файле параметров, указанном в сообщении, переписать правило так, чтобы оно заканчивалось на закрывающей скобке, например `Bash(ls *)` вместо `Bash(ls) x`
* Оставьте скобки внутри содержимого как они есть. Они буквальные, поэтому правило такое как `Edit(./Finance (2024)/**)` действительно без экранирования

До версии 2.1.260 Claude Code сообщал о правиле с несовпадающими скобками как `Mismatched parentheses`.

<h3 id="is-not-matched-by-file-permission-checks">
  Is not matched by file permission checks
</h3>

Claude Code обнаружил `Write`, `NotebookEdit`, `MultiEdit` или `Glob` [permission rule](/docs/ru/permissions#read-and-edit) с путём в одном из ваших [settings files](/docs/ru/settings#where-settings-live), в [managed settings](/docs/ru/managed-settings) или в значении флага `--allowedTools`, `--disallowedTools` или `--settings`. Он проверяет разрешения файлов только против правил `Edit` и `Read`, поэтому он никогда не консультирует правило пути, которое называет один из других инструментов файлов. Он сохраняет правило и ничего больше не меняет; предупреждение называет правило, его источник в скобках и замену для записи:

```text theme={null}
Permission deny rule (.claude/settings.json): Write(docs/**) is not matched by file permission checks — only Edit(path) rules are. Use Edit(docs/**) instead (Edit rules cover all file-editing tools).
```

**Что делать:**

* Замените `Write(path)`, `NotebookEdit(path)` и устаревшие `MultiEdit(path)` правила на `Edit(path)`. Правила `Edit` охватывают все инструменты редактирования файлов.
* Кроме как в `--allowedTools`, где Claude Code принимает правило `Glob` без предупреждения, замените правила `Glob(path)` на `Read(path)`.
* Исправьте правило в источнике, который называет предупреждение в скобках: путь файла параметров или сам флаг для `--allowed-tools` и `--disallowed-tools`. Путь `claude-settings-<hash>.json`, который не существует на диске, обозначает встроенное значение `--settings`. Исправьте JSON, который вы передаёте этому флагу.
* Оставьте простые правила имён инструментов такие как `Write` или `Glob` в покое. Claude Code соответствует им на [tool level](/docs/ru/permissions#match-all-uses-of-a-tool) и не предупреждает о них.
* Если источник читает `managed policy settings`, перенаправьте предупреждение тому, кто поддерживает ваши управляемые параметры, так как вы не можете очистить его самостоятельно.

В [background session](/docs/ru/agent-view) или с `--output-format json` или `stream-json`, Claude Code записывает предупреждение в журнал отладки вместо stderr, поэтому вывод, читаемый машиной, остаётся чистым. Выполните с `--debug` для захвата его в `~/.claude/debug/<session-id>.txt`. До версии 2.1.210 Claude Code принимал эти правила без предупреждения.

<h3 id="has-a-wildcard-before-the-rest-of-the-command">
  Has a wildcard before the rest of the command
</h3>

Claude Code обнаружил правило allow `Bash`, чей `*` приходит перед более поздним словом, которое определяет, какая это команда, например `Bash(git * main)` или `Bash(git -C * status *)`, в одном из ваших [settings files](/docs/ru/settings#where-settings-live), в [managed settings](/docs/ru/managed-settings) или в значении флага `--allowedTools` или `--settings`. `*` соответствует любому тексту, включая опции, вставленные в этой позиции: `Bash(git * main)` также одобряет `git -c core.fsmonitor=<script> diff main`, где `-c` заставляет git запустить программу, которую называет команда. [Wildcard patterns](/docs/ru/permissions#wildcard-patterns) показывает правила соответствия.

Предупреждение существует, чтобы вы могли сузить правило, чей подстановочный знак шире, чем вы предполагали. Claude Code сохраняет правило и ничего не меняет о том, как оно соответствует; предупреждение называет правило и его источник в скобках:

```text theme={null}
Permission allow rule (.claude/settings.json): Bash(git -C * status *) has a wildcard before the rest of the command, so it also matches any options inserted at that position and approves them without a prompt. For git, options such as -c and --exec-path can run arbitrary commands. Replace that * with the exact value you mean, or only use * after the subcommand (for example Bash(git status *)).
```

**Что делать:**

* Замените `*` перед подкомандой на точное значение, которое вы имеете в виду: `Bash(git checkout main)` вместо `Bash(git * main)`.
* Переместите каждый `*` после подкоманды: `Bash(git status *)` вместо `Bash(git -C * status *)`. Напишите одно правило на подкоманду, которую вы хотите разрешить.
* Исправьте правило в источнике, который называет предупреждение в скобках: путь файла параметров или сам флаг `--allowed-tools`. Путь `claude-settings-<hash>.json`, который не существует на диске, обозначает встроенное значение `--settings`. Исправьте JSON, который вы передаёте этому флагу.
* Если источник читает `managed policy settings`, перенаправьте предупреждение тому, кто поддерживает ваши управляемые параметры, так как вы не можете очистить его самостоятельно.

Claude Code не предупреждает о правилах deny и ask с той же формой: он отказывает или запрашивает дополнительные команды, которые они соответствуют, а не одобряет их. Он также не предупреждает о правилах, чья подкоманда приходит перед первым `*`, например `Bash(git commit *)`, или правилах, в которых ни одно слово, кроме опции, не следует за `*`, например `Bash(git *)`, или о правилах префикса `:*` такие как `Bash(git:*)`.

В [background session](/docs/ru/agent-view) или с `--output-format json` или `stream-json`, Claude Code записывает предупреждение в журнал отладки вместо stderr, поэтому вывод, читаемый машиной, остаётся чистым. Выполните с `--debug` для захвата его в `~/.claude/debug/<session-id>.txt`. До версии 2.1.246 Claude Code принимал эти правила без предупреждения.

<h3 id="crosssessioninbound-must-be-one-of-accept-hold-refuse">
  crossSessionInbound must be one of accept, hold, refuse
</h3>

Файл параметров устанавливает [`crossSessionInbound`](/docs/ru/settings-reference#crosssessioninbound) на значение, которое Claude Code не распознаёт, например опечатка `"reject"`. Второе предложение предупреждения зависит от того, какой файл содержит значение; в пользовательском, проектном, локальном или файле `--settings` оно читается:

```text theme={null}
"crossSessionInbound" must be one of "accept", "hold", "refuse"; received "reject". This value was ignored; while it is present, cross-session messages are held for your approval instead of being delivered. Set it to one of the values above.
```

В [managed settings](/docs/ru/managed-settings) Claude Code рассматривает нераспознанное значение как `refuse`, наиболее ограничивающее значение, и предупреждение говорит, что сообщения между сеансами отклоняются до тех пор, пока администратор не исправит это. Для того, как hold объединяется со значениями в ваших других файлах параметров, см. [`crossSessionInbound`](/docs/ru/settings-reference#crosssessioninbound).

**Что делать:**

* Установите ключ на `"accept"`, `"hold"` или `"refuse"` или удалите его
* Когда предупреждение называет управляемые параметры, попросите администратора исправить значение

До версии 2.1.248 Claude Code игнорировал нераспознанное значение без предупреждения.

<h3 id="the-200k-limit-isnt-enforced">
  The 200K limit isn't enforced
</h3>

Вы установили [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/ru/env-vars), что обычно заставляет [auto-compaction](/docs/ru/model-config#default-auto-compact-thresholds) держать сеансы на 1M-context моделях в окне 200K, но нет порога compaction, который ограничивает этот сеанс на или ниже 200K, поэтому беседа может расти за его пределы.

```text theme={null}
CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced for <model>, so this session can grow past it. To enforce it, set CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000 (or the autoCompactWindow setting).
```

Claude Code применяет лимит 200K самостоятельно для каждой модели, которую он распознаёт как имеющую собственное окно 1M, и для ID моделей, которые он не распознаёт, он compacts в окне, которое он предполагает. Предупреждение появляется, когда другая конфигурация побеждает это применение:

* ID модели не является одним, который Claude Code распознаёт, например [LLM gateway](/docs/ru/llm-gateway) alias, и вы установили [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/ru/env-vars) или повысили предполагаемое окно выше 200K с [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/ru/env-vars). В этом случае сообщение также предлагает `or update to a Claude Code version that recognizes <model>` как средство.
* Beta `context-1m` запрошенный через [`ANTHROPIC_BETAS`](/docs/ru/env-vars) или флаг [`--betas`](/docs/ru/cli-reference#cli-flags) всё ещё запрашивает API для окна 1M на модели, которая принимает эту бету, в то время как ничего не compacts сеанс на 200K

**Что делать:**

* Установите [`CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000`](/docs/ru/env-vars) или параметр [`autoCompactWindow`](/docs/ru/settings-reference#autocompactwindow) на `200000`, чтобы auto-compaction compacts на границе 200K
* Если сообщение называет ID модели, который эта версия не распознаёт, выполните `claude update`. Версия, которая распознаёт ID как 1M-context модель, применяет лимит без дополнительной конфигурации.
* Если вы хотите, чтобы сеанс использовал полное окно модели вместо этого, отмените `CLAUDE_CODE_DISABLE_1M_CONTEXT`; предупреждение сообщает только, что лимит 200K не применяется

В [background session](/docs/ru/agent-view) или с `--output-format json` или `stream-json`, Claude Code записывает предупреждение в журнал отладки вместо stderr.

<h3 id="unrecognized-model-id-on-a-request">
  Unrecognized model ID on a request
</h3>

Claude Code отправил запрос для ID модели, который ваша версия Claude Code не распознаёт, и не нашёл запись [`modelOverrides`](/docs/ru/model-config#override-model-ids-per-version), которая отображает этот ID на модель, которую он распознаёт. Claude Code всё ещё отправляет запрос с ID, как вы его настроили, и не выходит и не переключает модели.

```text theme={null}
[claude-code:unrecognized_model] {"model":"my-proxy-model","query_source":"sdk"}
```

В скрипте или harness, который читает stderr, совпадайте с префиксом `[claude-code:unrecognized_model]`. После префикса и одного пробела Claude Code записывает однострочный объект JSON. Claude Code может добавлять поля к нему в более поздней версии, поэтому игнорируйте любое поле, которое вы не ожидаете. Он записывает по крайней мере эти два:

* `model`: строка модели, как вы её настроили
* `query_source`: путь запроса, который использовал модель. Claude Code сообщает `sdk` для запуска `-p` и значение, которое начинается с `agent:` для subagent.

Claude Code записывает строку в одно из двух мест, в зависимости от того, как вы его запускаете:

* В [non-interactive mode](/docs/ru/headless) с `-p`, Claude Code записывает её в stderr под каждым `--output-format`, поэтому вы можете анализировать stdout без фильтрации строки
* В интерактивном сеансе или [background session](/docs/ru/agent-view), Claude Code записывает её в журнал отладки вместо этого; выполните с `--debug` для захвата её в `~/.claude/debug/<session-id>.txt`

Claude Code записывает строку один раз на строку модели на процесс. Он записывает отдельную строку для каждого дополнительного нераспознанного ID, например одного, который [subagent](/docs/ru/sub-agents#choose-a-model) или [background functionality](/docs/ru/costs#background-token-usage) использует.

Claude Code не записывает строку для ID поставщиков, которые он разрешает на модель, которую он распознаёт, например Amazon Bedrock `us.anthropic.claude-...` ID, ID Google Cloud's Agent Platform с суффиксом версии `@` и имена развёртывания Microsoft Foundry, которые содержат ID модели Claude. Claude Code проверяет модель позади Amazon Bedrock [application inference profile ARN](/docs/ru/amazon-bedrock#map-each-model-version-to-an-inference-profile) а не сам ARN. Он не записывает строку для ARN, который он не может разрешить, например неправильно введённый.

**Что делать:**

* Если вы установили ID намеренно, например [LLM gateway](/docs/ru/llm-gateway) alias, добавьте запись [`modelOverrides`](/docs/ru/model-config#override-model-ids-per-version) в ваш [settings file](/docs/ru/settings#where-settings-live) с ID как его значение. Используйте ID модели Anthropic как ключ, а не семейный alias такой как `opus`. Для `my-proxy-model` из примера строки добавьте эту запись:

  ```json theme={null}
  {
    "modelOverrides": {
      "claude-opus-4-6": "my-proxy-model"
    }
  }
  ```

  Claude Code затем рассматривает `my-proxy-model` как `claude-opus-4-6` и прекращает записывать строку.

* Если ID называет модель новее, чем ваша версия Claude Code, выполните `claude update`

* Если ID является опечаткой, исправьте его в любом из [places you can set a model](/docs/ru/model-config#setting-your-model) или [alias variables](/docs/ru/model-config#environment-variables), который его содержит. Если `query_source` начинается с `agent:`, исправьте его там, где вы устанавливаете [subagent's model](/docs/ru/sub-agents#choose-a-model) вместо этого.

До версии 2.1.233 Claude Code не записывал строку, когда отправлял запрос для ID модели, который он не распознавал.

<h3 id="stale-sandbox-mask-files-left-by-a-killed-session">
  Stale sandbox mask files left by a killed session
</h3>

`claude doctor` выводит это предупреждение в своей диагностике, и `/status` перечисляет ту же строку. Оно появляется на Linux и WSL2, когда [sandboxing](/docs/ru/sandboxing) включена с изоляцией файловой системы.

Пока выполняется sandboxed команда, sandbox держит отказ в записи на файл, который ещё не существует, создавая 0-байтовый заполнитель только для чтения там, и удаляет его впоследствии. Сеанс, убитый перед тем, как эта очистка запустится, например SIGKILL, оставляет заполнители позади. Более поздние сеансы привязывают их только для чтения снова при каждом запуске, поэтому запись параметров такая как сохранение "Yes, and don't ask again" не удаётся там, где она сидит.

```text theme={null}
- Stale sandbox mask files left by a killed session: /home/you/project/.claude/settings.local.json
  Fix: Remove each with `rm <path>` while no other Claude Code session is running in that project — a 0-byte read-only file where a settings file belongs makes "Yes, and don't ask again" fail to save, and the sandbox binds it read-only again on every start
```

**Что делать:**

* Выйдите из любого другого сеанса Claude Code, работающего в этом проекте, затем удалите каждый перечисленный файл с `rm`. Предупреждение называет до трёх файлов и считает остальное, поэтому переустановите `claude doctor` после удаления до тех пор, пока предупреждение больше не появляется. Заполнитель, который sandbox другого сеанса всё ещё использует, является живой частью защиты от записи этого сеанса
* Если выбор разрешения, который вы сохранили с "Yes, and don't ask again" не прижился, сохраните его снова после удаления заполнителя

До версии 2.1.257 `claude doctor` не отмечал эти файлы; более ранние версии оставляют те же заполнители позади, когда сеанс убивается.

<h2 id="responses-seem-lower-quality-than-usual">
  Ответы кажутся ниже обычного качества
</h2>

Если ответы Claude кажутся менее способными, чем вы ожидаете, но ошибка не отображается, причина обычно в состоянии разговора, а не в самой модели. Claude Code не молча меняет версии модели. Он может переключиться на резервную модель в трёх конкретных случаях:

* Настроенный [`--fallback-model`](/docs/ru/cli-reference#cli-flags) берёт на себя управление после ошибки доступности только для этого хода с уведомлением в расшифровке
* Проверка запуска Amazon Bedrock или Google Cloud's Agent Platform обнаруживает, что ваша модель по умолчанию недоступна
* [Автоматический переход на резервную модель](/docs/ru/model-config#automatic-model-fallback) на Fable 5.1, Fable 5, Opus 5.5 и Opus 5 переводит сеанс на резервную модель категории с флагом, когда у этой категории она есть, и показывает уведомление в расшифровке

Проверка выбора модели ниже ловит второй и третий случаи; первый появляется как уведомление в расшифровке, а не как изменение `/model`. [Конфигурация модели](/docs/ru/model-config) объясняет, когда применяется каждый переход на резервную модель.

Сначала проверьте следующее:

* **Выбор модели**: запустите `/model`, чтобы подтвердить, что вы используете ожидаемую модель. Предыдущий выбор `/model` или переменная окружения `ANTHROPIC_MODEL` могут привести вас к меньшей модели, чем вы предполагали.
* **Уровень усилий**: запустите `/effort`, чтобы проверить текущий уровень рассуждений и повысить его для сложной отладки или проектной работы. Значения по умолчанию варьируются в зависимости от модели, поэтому проверьте перед тем, как предполагать, что вы ниже максимума. См. [Отрегулируйте уровень усилий](/docs/ru/model-config#adjust-effort-level) для значений по умолчанию для каждой модели и сокращение `ultrathink`.
* **Давление контекста**: запустите `/context`, чтобы увидеть, насколько заполнено окно. Если оно близко к ёмкости, запустите `/compact` в естественной точке разрыва или `/clear`, чтобы начать заново. См. [Изучите окно контекста](/docs/ru/context-window), чтобы узнать, как auto-compact влияет на предыдущие ходы.
* **Устаревшие инструкции**: большие или устаревшие файлы `CLAUDE.md` и определения инструментов MCP потребляют контекст и могут направлять ответы. Проверка `/doctor` отмечает файлы памяти большого размера и неиспользуемые расширения, а `/context` показывает использование токенов инструментов MCP. До версии 2.1.205 `/doctor` открывал экран диагностики, который отмечал файлы памяти большого размера и определения подагентов.

Когда ответ идёт неправильно, откат обычно работает лучше, чем ответ с исправлениями. Нажмите Esc дважды или запустите `/rewind`, чтобы вернуться к моменту перед неправильным ходом, затем переформулируйте подсказку с большей конкретикой. Исправление в потоке сохраняет неправильную попытку в контексте, что может привязать более поздние ответы к ней. См. [Контрольные точки](/docs/ru/checkpointing).

Если качество всё ещё кажется неправильным после проверки вышеуказанного, запустите `/feedback` и опишите, что вы ожидали в сравнении с тем, что вы получили. Обратная связь, отправленная таким образом, включает расшифровку разговора, что является самым быстрым способом для Anthropic диагностировать реальную регрессию. См. [Сообщить об ошибке](#report-an-error), если `/feedback` недоступен в вашей среде.

Если Claude предупреждает о подозреваемой инъекции подсказки или отказывает в запросе из-за подозреваемой инъекции, и текст, который называет предупреждение, — это контекст, который Claude Code добавляет к разговору автоматически, а не содержимое файла или веб-сайта, запустите `claude update` и повторите попытку. Если предупреждение повторяется после обновления, [сообщите об этом](#report-an-error) вместо того, чтобы вставлять отмеченное содержимое обратно в подсказку. До версии 2.1.201 Sonnet 5 отказывал в некоторых запросах таким же образом.

<h2 id="report-an-error">
  Сообщить об ошибке
</h2>

Для ошибок компонентов, которые не рассматриваются на этой странице, см. соответствующее руководство:

* Серверу MCP не удалось подключиться или пройти аутентификацию: [MCP](/docs/ru/mcp)
* Скрипт hook не выполнился или заблокировал инструмент: [Debug hooks](/docs/ru/hooks#debug-hooks)
* Отказано в доступе или ошибки файловой системы при установке: [Troubleshoot installation and login](/docs/ru/troubleshoot-install)

Если ошибка не указана здесь или предложенное исправление не помогает:

* Запустите `/feedback` внутри Claude Code, чтобы отправить стенограмму и описание в Anthropic. Команда также предлагает открыть предварительно заполненную проблему GitHub. Отправка в Anthropic требует [аутентификации](/docs/ru/authentication). На Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry и других сторонних поставщиков, или когда учетные данные Anthropic не настроены, `/feedback` сохраняет локальный архив, который вы можете отправить представителю вашей учетной записи Anthropic.
* Запустите `claude doctor` из вашей оболочки для диагностики только для чтения вашей установки или запустите проверку `/doctor` внутри Claude Code, чтобы найти и исправить проблемы настройки
* Проверьте [status.claude.com](https://status.claude.com) на наличие активных инцидентов
* Поищите [существующие проблемы](https://github.com/anthropics/claude-code/issues) на GitHub
