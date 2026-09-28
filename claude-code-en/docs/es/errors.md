> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia de errores

> Busque mensajes de error en tiempo de ejecución de Claude Code con lo que cada uno significa y cómo solucionarlo.

Esta página enumera los errores en tiempo de ejecución que Claude Code muestra y cómo recuperarse de cada uno, además de qué verificar cuando las respuestas parecen estar mal sin un error. Para errores de instalación como `command not found` o fallos de TLS durante la configuración, consulte [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install).

Excepto por [Errores de Wrapper e IDE](#wrapper-and-ide-errors), que el programa de lanzamiento imprime en lugar de Claude Code en sí, estos errores y comandos de recuperación se aplican en toda la CLI, la [aplicación de escritorio](/docs/es/desktop) y [Claude Code en la web](/docs/es/claude-code-on-the-web), ya que los tres envuelven el mismo CLI de Claude Code. Para otros problemas específicos de la superficie, consulte la sección de solución de problemas en la página de esa superficie.

<Note>
  Claude Code llama a la API de Claude para respuestas del modelo, por lo que la mayoría de los errores en tiempo de ejecución se asignan a un código de error de API subyacente. Esta página cubre lo que cada error significa dentro de Claude Code y cómo recuperarse. Para las definiciones de código de estado HTTP sin procesar, consulte la [referencia de errores de la plataforma Claude](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Encuentre su error
</h2>

Haga coincidir el mensaje que ve con una sección a continuación.

| Mensaje                                                                                                                                                                                                                                                              | Sección                                                                                                                      |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `API Error: 500 Internal server error`                                                                                                                                                                                                                               | [Errores del servidor](#api-error-500-internal-server-error)                                                                 |
| `API Error: Repeated 529 Overloaded errors`                                                                                                                                                                                                                          | [Errores del servidor](#api-error-repeated-529-overloaded-errors)                                                            |
| `Request timed out`                                                                                                                                                                                                                                                  | [Errores del servidor](#request-timed-out), o [Red](#unable-to-connect-to-api) si el mensaje menciona su conexión a internet |
| `API Error: No response from API`                                                                                                                                                                                                                                    | [Errores del servidor](#no-response-from-api)                                                                                |
| `Server error mid-response. The response above may be incomplete.`                                                                                                                                                                                                   | [Errores del servidor](#the-response-above-may-be-incomplete)                                                                |
| `Connection lost mid-response` / `Your computer went to sleep mid-response` / `The response stopped arriving`                                                                                                                                                        | [Errores del servidor](#the-response-above-may-be-incomplete)                                                                |
| `Connection closed mid-response` / `Response stalled mid-stream`                                                                                                                                                                                                     | [Errores del servidor](#the-response-above-may-be-incomplete)                                                                |
| `Connection lost before a response was produced` / `Your computer went to sleep before a response was produced` / `The response stalled before a response was produced`                                                                                              | [Reintentos automáticos](#automatic-retries)                                                                                 |
| `Connection closed while thinking` / `Response stalled while thinking`                                                                                                                                                                                               | [Reintentos automáticos](#automatic-retries)                                                                                 |
| `Connection lost while your computer was asleep`                                                                                                                                                                                                                     | [Reintentos automáticos](#automatic-retries)                                                                                 |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`                                                                                                                                                                                 | [Errores del servidor](#auto-mode-cannot-determine-the-safety-of-an-action)                                                  |
| `Auto mode could not evaluate this action and is blocking it for safety`                                                                                                                                                                                             | [Errores del servidor](#auto-mode-cannot-determine-the-safety-of-an-action)                                                  |
| `Auto mode classifier transcript exceeded context window`                                                                                                                                                                                                            | [Errores del servidor](#auto-mode-cannot-determine-the-safety-of-an-action)                                                  |
| `Agent aborted: auto mode classifier request refused by the safety safeguard`                                                                                                                                                                                        | [Errores del servidor](#auto-mode-cannot-determine-the-safety-of-an-action)                                                  |
| `The server-side auto mode classifier gave no verdict`                                                                                                                                                                                                               | [Errores del servidor](#the-server-returned-no-safety-verdict)                                                               |
| `Auto mode is unavailable — the server returned no safety verdict for the last 10 responses`                                                                                                                                                                         | [Errores del servidor](#the-server-returned-no-safety-verdict)                                                               |
| `Agent terminated early due to an API error`                                                                                                                                                                                                                         | [Errores del servidor](#agent-terminated-early-due-to-an-api-error)                                                          |
| `You've hit your session limit` / `You've hit your weekly limit` / `You've hit your Opus limit` / `You've hit your Sonnet limit`                                                                                                                                     | [Límites de uso](#youve-hit-your-session-limit)                                                                              |
| `Usage credits required for 1M context`                                                                                                                                                                                                                              | [Límites de uso](#usage-credits-required-for-1m-context)                                                                     |
| `the prompt to confirm went unanswered — nothing was sent`                                                                                                                                                                                                           | [Límites de uso](#the-prompt-to-confirm-went-unanswered)                                                                     |
| `Server is temporarily limiting requests`                                                                                                                                                                                                                            | [Límites de uso](#server-is-temporarily-limiting-requests)                                                                   |
| `Request rejected (429)`                                                                                                                                                                                                                                             | [Límites de uso](#request-rejected-429)                                                                                      |
| `Credit balance is too low`                                                                                                                                                                                                                                          | [Límites de uso](#credit-balance-is-too-low)                                                                                 |
| `You've hit your monthly spend limit` / `You've hit your individual spend limit` / `You've hit your org's monthly spend limit` / `You've hit your channel's monthly spend limit` / `You've hit your team's shared budget` / `You've hit your individual usage limit` | [Límites de uso](#youve-hit-your-monthly-spend-limit)                                                                        |
| `Could not update your spend limit`                                                                                                                                                                                                                                  | [Límites de uso](#could-not-update-your-spend-limit)                                                                         |
| `spend limit reached` / `spend limit unavailable`                                                                                                                                                                                                                    | [Límites de uso](#spend-limit-reached)                                                                                       |
| `Not logged in · Please run /login`                                                                                                                                                                                                                                  | [Autenticación](#not-logged-in)                                                                                              |
| `Could not resolve authentication method`                                                                                                                                                                                                                            | [Autenticación](#could-not-resolve-authentication-method)                                                                    |
| `Invalid API key`                                                                                                                                                                                                                                                    | [Autenticación](#invalid-api-key)                                                                                            |
| `Your apiKeyHelper script is failing`                                                                                                                                                                                                                                | [Autenticación](#your-apikeyhelper-script-is-failing)                                                                        |
| `Invalid auth token · Fix external auth token`                                                                                                                                                                                                                       | [Autenticación](#invalid-request-header-value)                                                                               |
| `Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable`                                                                                                                                                                                                    | [Autenticación](#invalid-request-header-value)                                                                               |
| `Invalid request header from the environment · Fix the environment variable`                                                                                                                                                                                         | [Autenticación](#invalid-request-header-value)                                                                               |
| `This organization has been disabled`                                                                                                                                                                                                                                | [Autenticación](#this-organization-has-been-disabled)                                                                        |
| `Your organization has disabled API key authentication`                                                                                                                                                                                                              | [Autenticación](#your-organization-has-disabled-api-key-authentication)                                                      |
| `Your organization has disabled Claude subscription access`                                                                                                                                                                                                          | [Autenticación](#your-organization-has-disabled-claude-subscription-access)                                                  |
| `Routines are disabled by your organization's policy`                                                                                                                                                                                                                | [Autenticación](#routines-are-disabled-by-your-organizations-policy)                                                         |
| `Remote Control is only available when using Claude via api.anthropic.com`                                                                                                                                                                                           | [Autenticación](#remote-control-requires-the-anthropic-api)                                                                  |
| `OAuth token refresh failed — run /login to re-authenticate`                                                                                                                                                                                                         | [Autenticación](#remote-control-couldnt-refresh-your-login)                                                                  |
| `JWT refresh failed: no OAuth token — run /login`                                                                                                                                                                                                                    | [Autenticación](#remote-control-couldnt-refresh-your-login)                                                                  |
| `Claude.ai login expired`                                                                                                                                                                                                                                            | [Autenticación](#remote-control-couldnt-refresh-your-login)                                                                  |
| `Claude.ai login was rejected — run /login, then /remote-control`                                                                                                                                                                                                    | [Autenticación](#remote-control-couldnt-refresh-your-login)                                                                  |
| `OAuth token unavailable — run /login to restore Remote Control`                                                                                                                                                                                                     | [Autenticación](#remote-control-couldnt-refresh-your-login)                                                                  |
| `Signed out of Claude — run /login, then /remote-control`                                                                                                                                                                                                            | [Autenticación](#remote-control-couldnt-refresh-your-login)                                                                  |
| `signed-in claude.ai account or organization changed on this machine`                                                                                                                                                                                                | [Autenticación](#remote-control-stopped-because-the-signed-in-account-changed)                                               |
| `Remote Control stopped — the app running this session is now signed in to a different Claude account`                                                                                                                                                               | [Autenticación](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)                 |
| `Remote Control stopped — the app running this session is signed out of Claude`                                                                                                                                                                                      | [Autenticación](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)                 |
| `OAuth token revoked` / `OAuth token has expired`                                                                                                                                                                                                                    | [Autenticación](#oauth-token-revoked-or-expired)                                                                             |
| `API Error: 401 Invalid authentication credentials`                                                                                                                                                                                                                  | [Autenticación](#api-error-401-invalid-authentication-credentials)                                                           |
| `Login expired · Please run /login`                                                                                                                                                                                                                                  | [Autenticación](#login-expired)                                                                                              |
| `Claude login not accepted · Run /login, then try again`                                                                                                                                                                                                             | [Autenticación](#claude-login-not-accepted)                                                                                  |
| `Artifacts need a claude.ai login`                                                                                                                                                                                                                                   | [Autenticación](#artifacts-need-a-claude-ai-login)                                                                           |
| `Not signed in to the Cloud gateway — run /login.`                                                                                                                                                                                                                   | [Autenticación](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                      |
| `Administrator policy requires a Cloud gateway sign-in on this machine`                                                                                                                                                                                              | [Autenticación](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                      |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                                                                                                                                                                                           | [Autenticación](#login-expired)                                                                                              |
| `Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                            | [Autenticación](#your-account-is-on-hold)                                                                                    |
| `Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                     | [Autenticación](#your-account-is-on-hold)                                                                                    |
| `Anthropic profile login expired · Re-authenticate your Anthropic profile`                                                                                                                                                                                           | [Autenticación](#anthropic-profile-login-expired)                                                                            |
| `Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile`                                                                                                                                                 | [Autenticación](#anthropic-profile-login-expired)                                                                            |
| `does not meet scope requirement user:profile`                                                                                                                                                                                                                       | [Autenticación](#oauth-scope-requirement)                                                                                    |
| `claude.ai rejected the session token` / `session token rejected`                                                                                                                                                                                                    | [Autenticación](#claude-ai-rejected-the-session-token)                                                                       |
| `MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)`                                                                                                                                                                                       | [Autenticación](#mcp-server-needs-you-to-sign-in-again)                                                                      |
| `rejected the credential from its headersHelper` / `rejected the Authorization header in its config`                                                                                                                                                                 | [Autenticación](#mcp-server-needs-you-to-sign-in-again)                                                                      |
| `MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate`                                                                                                                                                                  | [Autenticación](#mcp-server-needs-you-to-sign-in-again)                                                                      |
| `MCP server "<name>" requires re-authorization (token expired)`                                                                                                                                                                                                      | [Autenticación](#mcp-server-needs-you-to-sign-in-again)                                                                      |
| `Issuer mismatch in authorization response (RFC 9207)`                                                                                                                                                                                                               | [Autenticación](#issuer-mismatch-in-authorization-response)                                                                  |
| `Cloud gateway session expired — run /login to reconnect.`                                                                                                                                                                                                           | [Autenticación](#cloud-gateway-session-expired)                                                                              |
| `Cloud gateway <url> no longer accepts this session`                                                                                                                                                                                                                 | [Autenticación](#cloud-gateway-session-expired)                                                                              |
| `Sign-in timed out while waiting for you to continue. Try again.`                                                                                                                                                                                                    | [Autenticación](#sign-in-timed-out-while-waiting-for-you-to-continue)                                                        |
| `AWS credentials expired or invalid`                                                                                                                                                                                                                                 | [Autenticación](#aws-credentials-expired-or-invalid)                                                                         |
| `AWS authentication failed`                                                                                                                                                                                                                                          | [Autenticación](#aws-authentication-failed)                                                                                  |
| `Google Cloud credentials expired or invalid`                                                                                                                                                                                                                        | [Autenticación](#google-cloud-credentials-expired-or-invalid)                                                                |
| `Google Cloud authentication failed`                                                                                                                                                                                                                                 | [Autenticación](#google-cloud-authentication-failed)                                                                         |
| `Microsoft Foundry authentication failed`                                                                                                                                                                                                                            | [Autenticación](#microsoft-foundry-authentication-failed)                                                                    |
| `Gateway refused the request`                                                                                                                                                                                                                                        | [Autenticación](#gateway-refused-the-request)                                                                                |
| `Could not load AWS credentials` / `Could not load Google Cloud credentials`                                                                                                                                                                                         | [Autenticación](#could-not-load-aws-or-google-cloud-credentials)                                                             |
| `AWS default-chain credential resolve timed out`                                                                                                                                                                                                                     | [Autenticación](#aws-default-chain-credential-resolve-timed-out)                                                             |
| `Timed out after 60s waiting for AWS`                                                                                                                                                                                                                                | [Autenticación](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                       |
| `A request to AWS timed out. Check your network and proxy settings, then try again.`                                                                                                                                                                                 | [Autenticación](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                       |
| `Could not load the default credentials` on Google Cloud's Agent Platform                                                                                                                                                                                            | [Autenticación](#could-not-load-aws-or-google-cloud-credentials)                                                             |
| `Unable to connect to API`                                                                                                                                                                                                                                           | [Red](#unable-to-connect-to-api)                                                                                             |
| `Connection refused —` / `Can't reach the API server —` / `No internet route —` / `Couldn't connect through your proxy` / `Connection dropped`, each with an error code in parentheses                                                                               | [Red](#unable-to-connect-to-api)                                                                                             |
| `Unable to connect to Anthropic services` during setup                                                                                                                                                                                                               | [Red](#unable-to-connect-to-anthropic-services)                                                                              |
| `Socket is closed`                                                                                                                                                                                                                                                   | [Red](#socket-is-closed)                                                                                                     |
| `Waiting for API response · will retry in`                                                                                                                                                                                                                           | [Reintentos automáticos](#automatic-retries), o [Red](#unable-to-connect-to-api) si persiste                                 |
| `API returned an empty or malformed response`                                                                                                                                                                                                                        | [Red](#api-returned-an-empty-or-malformed-response)                                                                          |
| `Streaming response ended before any complete data was received`                                                                                                                                                                                                     | [Red](#streaming-response-ended-before-any-complete-data-was-received)                                                       |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"`                                                                                                                                                                   | [Red](#bedrock-streaming-response-has-an-unexpected-content-type)                                                            |
| `SSL certificate verification failed`                                                                                                                                                                                                                                | [Red](#ssl-certificate-errors)                                                                                               |
| `SSL certificate error (...)` during login or startup                                                                                                                                                                                                                | [Red](#ssl-certificate-errors)                                                                                               |
| `unable to get local issuer certificate`                                                                                                                                                                                                                             | [Red](#ssl-certificate-errors)                                                                                               |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                                                                                                                                                                                           | [Red](#host-not-allowed-in-a-cloud-session)                                                                                  |
| `proxy refused the connection`                                                                                                                                                                                                                                       | [Red](#the-proxy-refused-the-connection)                                                                                     |
| `403` with `This GraphQL query is not enabled for this session` in a cloud session                                                                                                                                                                                   | [GitHub proxy](/docs/es/cloud-environments#github-proxy)                                                                          |
| `The cloud environments service returned an empty response` / `The cloud environments service returned a response in an unexpected format`                                                                                                                           | [Red](#the-cloud-environments-service-returned-an-empty-or-unexpected-response)                                              |
| `Couldn't reconnect to your Remote Control session`                                                                                                                                                                                                                  | [Red](#couldnt-reconnect-to-your-remote-control-session)                                                                     |
| `N sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.`                                                                                                                                               | [Red](#sessions-ended-while-this-machine-was-offline)                                                                        |
| `Couldn't share the transcript.`                                                                                                                                                                                                                                     | [Red](#couldnt-share-the-transcript)                                                                                         |
| `Prompt is too long` / `Input is too long for requested model`                                                                                                                                                                                                       | [Errores de solicitud](#prompt-is-too-long)                                                                                  |
| `Prompt is too long · automatic compaction failed:`                                                                                                                                                                                                                  | [Errores de solicitud](#prompt-is-too-long)                                                                                  |
| `Prompt is too long · this conversation is a single exchange` / `A single-exchange conversation cannot be compacted`                                                                                                                                                 | [Errores de solicitud](#prompt-is-too-long)                                                                                  |
| `Context limit reached · /compact or /clear to continue`                                                                                                                                                                                                             | [Errores de solicitud](#prompt-is-too-long)                                                                                  |
| `Context limit reached · /clear to continue`                                                                                                                                                                                                                         | [Errores de solicitud](#prompt-is-too-long)                                                                                  |
| `capability_rejected: prompt_too_long` on a Claude apps gateway session                                                                                                                                                                                              | [Errores de solicitud](#prompt-is-too-long)                                                                                  |
| `upstream rejected the request` / `request too large for this upstream` on a Claude apps gateway session                                                                                                                                                             | [Mensajes de error ascendentes](/docs/es/claude-apps-gateway-config#upstream-error-messages)                                      |
| `upstream rate limit exceeded` on a Claude apps gateway session                                                                                                                                                                                                      | [Mensajes de error ascendentes](/docs/es/claude-apps-gateway-config#upstream-error-messages)                                      |
| `all upstreams failed (N attempted)` on a Claude apps gateway session                                                                                                                                                                                                | [Mensajes de error ascendentes](/docs/es/claude-apps-gateway-config#upstream-error-messages)                                      |
| `Claude Code may not be enabled for your organization` after a Claude apps gateway sign-in                                                                                                                                                                           | [Solución de problemas de la puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway-deploy#troubleshooting)        |
| `Context exceeds the ...-token limit by ... tokens` in `/context` output                                                                                                                                                                                             | [Errores de solicitud](#context-exceeds-the-token-limit)                                                                     |
| `Error during compaction: Conversation too long`                                                                                                                                                                                                                     | [Errores de solicitud](#error-during-compaction-conversation-too-long)                                                       |
| `Request too large`                                                                                                                                                                                                                                                  | [Errores de solicitud](#request-too-large)                                                                                   |
| `Request too large for the API's 32MB request limit`                                                                                                                                                                                                                 | [Errores de solicitud](#request-too-large)                                                                                   |
| `Image was too large`                                                                                                                                                                                                                                                | [Errores de solicitud](#image-was-too-large)                                                                                 |
| `Unable to resize image`                                                                                                                                                                                                                                             | [Errores de solicitud](#unable-to-resize-image)                                                                              |
| `PDF too large` / `PDF is password protected`                                                                                                                                                                                                                        | [Errores de solicitud](#pdf-errors)                                                                                          |
| `Extra inputs are not permitted`                                                                                                                                                                                                                                     | [Errores de solicitud](#extra-inputs-are-not-permitted)                                                                      |
| `API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid` / `Property keys should match pattern`                                                                                                                                                      | [Errores de solicitud](#tool-input-schema-is-invalid)                                                                        |
| `There's an issue with the selected model`                                                                                                                                                                                                                           | [Errores de solicitud](#theres-an-issue-with-the-selected-model)                                                             |
| `Model ... is not a recognized model id`                                                                                                                                                                                                                             | [Errores de solicitud](#model-is-not-a-recognized-model-id)                                                                  |
| `Model ... not found`                                                                                                                                                                                                                                                | [Errores de solicitud](#model-not-found)                                                                                     |
| `Claude Opus is not available with the Claude Pro plan`                                                                                                                                                                                                              | [Errores de solicitud](#claude-opus-is-not-available-with-the-claude-pro-plan)                                               |
| `Claude Code ... does not support this model; version ... or newer is required`                                                                                                                                                                                      | [Errores de solicitud](#claude-code-does-not-support-this-model)                                                             |
| `Claude Code ... is older than the minimum version required by your organization's policy`                                                                                                                                                                           | [Errores de solicitud](#claude-code-does-not-support-this-model)                                                             |
| `Model ... is restricted by your organization's settings`                                                                                                                                                                                                            | [Errores de solicitud](#model-is-restricted-by-your-organizations-settings)                                                  |
| `Model switch ... blocked by a PreModelSwitch hook`                                                                                                                                                                                                                  | [Errores de solicitud](#model-switch-was-blocked-by-a-premodelswitch-hook)                                                   |
| `couldn't save it as your default` / `couldn't confirm it was saved as your default`                                                                                                                                                                                 | [Errores de solicitud](#couldnt-save-it-as-your-default)                                                                     |
| `thinking.type.enabled is not supported for this model`                                                                                                                                                                                                              | [Errores de solicitud](#thinking-type-enabled-is-not-supported-for-this-model)                                               |
| `Effort '<level>' isn't available with thinking turned off on this model`                                                                                                                                                                                            | [Errores de solicitud](#effort-isnt-available-with-thinking-turned-off)                                                      |
| `effort '<level>' is not supported when thinking is disabled`                                                                                                                                                                                                        | [Errores de solicitud](#effort-isnt-available-with-thinking-turned-off)                                                      |
| `max_tokens must be greater than thinking.budget_tokens`                                                                                                                                                                                                             | [Errores de solicitud](#thinking-budget-exceeds-output-limit)                                                                |
| `API Error: 400 due to tool use concurrency issues`                                                                                                                                                                                                                  | [Errores de solicitud](#tool-use-or-thinking-block-mismatch)                                                                 |
| `API Error: 400 orphaned tool_result in conversation history`                                                                                                                                                                                                        | [Errores de solicitud](#tool-use-or-thinking-block-mismatch)                                                                 |
| `API Error: 400 duplicate tool_use ID in conversation history`                                                                                                                                                                                                       | [Errores de solicitud](#tool-use-or-thinking-block-mismatch)                                                                 |
| `[Unsupported tool content removed]`                                                                                                                                                                                                                                 | [Errores de solicitud](#unsupported-tool-content-removed)                                                                    |
| `role 'system' must precede an 'assistant' message`                                                                                                                                                                                                                  | [Errores de solicitud](#role-system-must-precede-an-assistant-message)                                                       |
| `Invalid encrypted_content in search_result block` / `Invalid encrypted_index in text block` / `Failed to decrypt web search result content`                                                                                                                         | [Errores de solicitud](#invalid-encrypted-content-in-search-result-block)                                                    |
| `server_tool_use.name: Input should be` on every turn of a resumed session                                                                                                                                                                                           | [Errores de solicitud](#unsupported-tool-content-removed)                                                                    |
| `<model> can't help with this. Start a new session to continue`                                                                                                                                                                                                      | [Errores de solicitud](#usage-policy-refusal)                                                                                |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`                                                                                                                                                                        | [Errores de solicitud](#usage-policy-refusal)                                                                                |
| `<model>'s safeguards flagged this message`                                                                                                                                                                                                                          | [Errores de solicitud](#safety-measures-flagged-a-cybersecurity-topic)                                                       |
| `Opus 5.5's safeguards flagged this session`                                                                                                                                                                                                                         | [Errores de solicitud](#safety-measures-flagged-a-cybersecurity-topic)                                                       |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                                                                                                                                                                                    | [Errores de solicitud](#safety-measures-flagged-a-cybersecurity-topic)                                                       |
| `Installation was killed before it could finish (exit code 137)`                                                                                                                                                                                                     | [Errores de instalación](#installation-was-killed-before-it-could-finish)                                                    |
| `The connection dropped while downloading the update`                                                                                                                                                                                                                | [Errores de instalación](#the-connection-dropped-while-downloading-the-update)                                               |
| `Download timed out: exceeded the total deadline`                                                                                                                                                                                                                    | [Errores de instalación](#the-connection-dropped-while-downloading-the-update)                                               |
| `--bg and --print conflict`                                                                                                                                                                                                                                          | [Errores de línea de comandos](#command-line-errors)                                                                         |
| `Cloud sessions cannot be created from a --restricted session`                                                                                                                                                                                                       | [Errores de línea de comandos](#cloud-sessions-cannot-be-created-from-a-restricted-session)                                  |
| `Cloud sessions are disabled by your organization's policy`                                                                                                                                                                                                          | [Errores de línea de comandos](#cloud-sessions-are-disabled-by-your-organizations-policy)                                    |
| `Couldn't verify your organization's policy for cloud sessions`                                                                                                                                                                                                      | [Errores de línea de comandos](#cloud-sessions-are-disabled-by-your-organizations-policy)                                    |
| `Error: --json-schema is not a valid JSON Schema`                                                                                                                                                                                                                    | [Errores de línea de comandos](#command-line-errors)                                                                         |
| `Error: Invalid --agents configuration:`                                                                                                                                                                                                                             | [Errores de línea de comandos](#invalid-agents-configuration)                                                                |
| `Error: Settings file exceeds the 2MiB limit`                                                                                                                                                                                                                        | [Errores de línea de comandos](#settings-file-exceeds-the-2mib-limit)                                                        |
| `The current directory no longer exists (it was deleted or moved)` / `Can't read the current directory`                                                                                                                                                              | [Errores de línea de comandos](#the-current-directory-no-longer-exists)                                                      |
| `Temp directory <dir> ... Refusing to use it` / `ENOSPC: no space left on device, mkdir '<dir>'`                                                                                                                                                                     | [Errores de línea de comandos](#temp-directory-refused-or-cannot-be-created)                                                 |
| `couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded`                                                                                                                                                                        | [Errores de línea de comandos](#directory-couldnt-be-resolved-to-a-real-location)                                            |
| `Error: Workspace not trusted` when starting Remote Control                                                                                                                                                                                                          | [Errores de línea de comandos](#workspace-not-trusted-when-starting-remote-control)                                          |
| `` `<flag>` before `remote-control` is not carried over to the sessions Remote Control starts ``                                                                                                                                                                     | [Errores de línea de comandos](#not-carried-over-to-the-sessions-remote-control-starts)                                      |
| `` `claude import` is not yet available in this build ``                                                                                                                                                                                                             | [Errores de línea de comandos](#claude-import-is-not-yet-available-in-this-build)                                            |
| `Could not read Claude Code config`                                                                                                                                                                                                                                  | [Errores de línea de comandos](#could-not-read-claude-code-config)                                                           |
| `Could not import <server>: <reason>`                                                                                                                                                                                                                                | [Errores de línea de comandos](#could-not-import-a-server-from-claude-desktop)                                               |
| `Cannot add MCP server to scope: managed`                                                                                                                                                                                                                            | [Errores de línea de comandos](#cannot-add-mcp-server-to-the-managed-scope)                                                  |
| `is Anthropic-hosted and doesn't support local OAuth`                                                                                                                                                                                                                | [Errores de línea de comandos](#anthropic-hosted-and-doesnt-support-local-oauth)                                             |
| `Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes`                                                                                                                                                                                      | [Errores de línea de comandos](#cant-read-mcp-json)                                                                          |
| `Server rejected the Authorization header minted by the configured headersHelper`                                                                                                                                                                                    | [Errores de línea de comandos](#server-rejected-the-authorization-header-minted-by-the-configured-headershelper)             |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                                                                                                                                                                                             | [Errores de línea de comandos](#mcp-permission-prompt-tool-not-found)                                                        |
| `OAuth callback port <port> is already in use — another process may be holding it`                                                                                                                                                                                   | [Errores de línea de comandos](#oauth-callback-port-is-already-in-use)                                                       |
| `No available ports for OAuth redirect`                                                                                                                                                                                                                              | [Errores de línea de comandos](#no-available-ports-for-oauth-redirect)                                                       |
| `Shell command failed for pattern "..."`, from `/security-review` or any skill that injects dynamic context                                                                                                                                                          | [Errores de línea de comandos](#security-review-fails-without-origin-head)                                                   |
| `Shell command permission check failed for pattern "..."`, from a skill that injects dynamic context                                                                                                                                                                 | [Errores de línea de comandos](#security-review-fails-without-origin-head)                                                   |
| ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``                                                                                                                                                                             | [Errores de línea de comandos](#security-review-fails-without-origin-head)                                                   |
| `Input must be provided either through stdin or as a prompt argument when using --print`                                                                                                                                                                             | [Errores de línea de comandos](#input-must-be-provided-when-using-print)                                                     |
| `Error: Input contained only whitespace`                                                                                                                                                                                                                             | [Errores de línea de comandos](#input-contained-only-whitespace)                                                             |
| `Blank prompt — the message was only whitespace, so nothing was sent to the model.`                                                                                                                                                                                  | [Errores de línea de comandos](#input-contained-only-whitespace)                                                             |
| `Error: stream-json input carried over 256M characters with no newline`                                                                                                                                                                                              | [Errores de línea de comandos](#stream-json-input-carried-over-256m-characters-with-no-newline)                              |
| `Unknown command: /<name>`, with or without a `Did you mean` suggestion                                                                                                                                                                                              | [Errores de línea de comandos](#unknown-command)                                                                             |
| `Diff is too large for ultrareview` / `PR #<N> is too large for ultrareview`                                                                                                                                                                                         | [Errores de línea de comandos](#diff-is-too-large-for-ultrareview)                                                           |
| `Could not find merge-base with <branch>`                                                                                                                                                                                                                            | [Errores de línea de comandos](#could-not-find-merge-base-with-the-base-branch)                                              |
| `Your checkout has no branches (detached HEAD only)`                                                                                                                                                                                                                 | [Errores de línea de comandos](#your-checkout-has-no-branches)                                                               |
| `Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected`                                                                                                                                     | [Errores de línea de comandos](#no-github-account-is-connected-to-your-claude-account)                                       |
| `Your connected GitHub account can't see <owner>/<repo>`                                                                                                                                                                                                             | [Errores de línea de comandos](#your-connected-github-account-cant-see-the-repository)                                       |
| `The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead`                                                                                                                                           | [Errores de línea de comandos](#the-github-app-preflight-failed-transiently)                                                 |
| `GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud`                                                                                                                                                                     | [Errores de línea de comandos](#github-isnt-connected-to-your-claude-account)                                                |
| `Single sign-on authorization needed`                                                                                                                                                                                                                                | [Errores de línea de comandos](#single-sign-on-authorization-needed)                                                         |
| `Failed to resume the conversation`                                                                                                                                                                                                                                  | [Errores de línea de comandos](#failed-to-resume-the-conversation)                                                           |
| `No conversation found with session ID: <session-id>`                                                                                                                                                                                                                | [Errores de línea de comandos](#no-conversation-found-with-the-session-id)                                                   |
| `Cannot switch renderers in this session`                                                                                                                                                                                                                            | [Errores de línea de comandos](#cannot-switch-renderers-in-this-session)                                                     |
| `Cannot switch renderers while work is running in the background`                                                                                                                                                                                                    | [Errores de línea de comandos](#cannot-switch-renderers-in-this-session)                                                     |
| `Couldn't open Claude Desktop`                                                                                                                                                                                                                                       | [Errores de línea de comandos](#couldnt-open-claude-desktop)                                                                 |
| `Failed to open Claude Desktop. Please try opening it manually.`                                                                                                                                                                                                     | [Errores de línea de comandos](#couldnt-open-claude-desktop)                                                                 |
| `Couldn't read your Zed keymap` / `Couldn't back up your Zed keymap` / `Couldn't update your Zed keymap`                                                                                                                                                             | [Errores de línea de comandos](#terminal-setup-left-your-zed-keymap-unchanged)                                               |
| `Your Zed keymap isn't a readable list of keybindings`                                                                                                                                                                                                               | [Errores de línea de comandos](#terminal-setup-left-your-zed-keymap-unchanged)                                               |
| `Skill usage reports are not available on this connection.`                                                                                                                                                                                                          | [Errores de línea de comandos](#skill-usage-reports-are-not-available-on-this-connection)                                    |
| `Custom output styles can't be selected over Remote Control or from a relayed message`                                                                                                                                                                               | [Errores de línea de comandos](#custom-output-styles-cant-be-selected-over-remote-control)                                   |
| `Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load`                                                                                                                                                           | [Errores de línea de comandos](#output-styles-are-saved-to-local-settings-which-this-session-doesnt-load)                    |
| `` `plugin eval` is currently in early access `` / `` `plugin eval` is currently unavailable ``                                                                                                                                                                      | [Errores de plugins](#plugin-eval-is-currently-in-early-access)                                                              |
| `Marketplace "<name>" is registered from an untrusted source`                                                                                                                                                                                                        | [Errores de plugins](#marketplace-is-registered-from-an-untrusted-source)                                                    |
| `Marketplace "<name>" is already added from a different source`                                                                                                                                                                                                      | [Errores de plugins](#marketplace-is-already-added-from-a-different-source)                                                  |
| `"<name>" is another spelling of "<reserved>", a reserved marketplace name`                                                                                                                                                                                          | [Errores de plugins](#marketplace-name-is-another-spelling-of-a-reserved-name)                                               |
| `references ${user_config.*} in a shell-form command`                                                                                                                                                                                                                | [Errores de plugins](#plugin-command-references-user-config)                                                                 |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                                                                                                                                                                                   | [Errores de plugins](#plugin-command-references-user-config)                                                                 |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                                                                                                                                                                                  | [Errores de plugins](#plugin-command-references-user-config)                                                                 |
| `Plugin archive integrity check failed`                                                                                                                                                                                                                              | [Errores de plugins](#plugin-archive-integrity-check-failed)                                                                 |
| `path escapes plugin directory`                                                                                                                                                                                                                                      | [Errores de plugins](#path-escapes-plugin-directory)                                                                         |
| `path could not be checked`                                                                                                                                                                                                                                          | [Errores de plugins](#path-could-not-be-checked)                                                                             |
| `its marketplace entry path does not stay inside the marketplace directory`                                                                                                                                                                                          | [Errores de plugins](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                 |
| `Plugin source path refused`                                                                                                                                                                                                                                         | [Errores de plugins](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                 |
| `Failed to load marketplace configuration`                                                                                                                                                                                                                           | [Errores de plugins](#failed-to-load-marketplace-configuration)                                                              |
| `Marketplace configuration file is corrupted`                                                                                                                                                                                                                        | [Errores de plugins](#failed-to-load-marketplace-configuration)                                                              |
| `Plugin "<name>@synced" is required by your organization and can't be disabled here`                                                                                                                                                                                 | [Errores de plugins](#plugin-is-required-by-your-organization)                                                               |
| `would be spawned with zero tools — refusing`                                                                                                                                                                                                                        | [Errores de herramientas](#agent-would-be-spawned-with-zero-tools)                                                           |
| `File is covered by a Read deny rule in your permission settings`                                                                                                                                                                                                    | [Errores de herramientas](#file-is-covered-by-a-read-deny-rule)                                                              |
| `subagent_type is required: the general-purpose agent is not available in this session`                                                                                                                                                                              | [Errores de herramientas](#subagent-type-is-required)                                                                        |
| `Error: this write left the memory index at MEMORY.md at ..., over its ... read limit`                                                                                                                                                                               | [Errores de herramientas](#memory-index-is-over-its-read-limit)                                                              |
| `pkill: refusing to run`                                                                                                                                                                                                                                             | [Errores de herramientas](#pkill-pattern-matches-the-claude-code-process)                                                    |
| `Failed to write to <name>'s inbox — nothing was sent`                                                                                                                                                                                                               | [Errores de herramientas](#failed-to-write-to-a-teammate-inbox)                                                              |
| `Failed to write the plan approval request to the lead's inbox — plan not submitted`                                                                                                                                                                                 | [Errores de herramientas](#failed-to-write-to-a-teammate-inbox)                                                              |
| `Its agent definition was not restored: the folder its definition file came from is not trusted`                                                                                                                                                                     | [Errores de herramientas](#teammate-agent-definition-not-restored)                                                           |
| `Message too large for cross-session delivery`                                                                                                                                                                                                                       | [Errores de herramientas](#message-too-large-for-cross-session-delivery)                                                     |
| `Too many messages to this session just now`                                                                                                                                                                                                                         | [Errores de herramientas](#too-many-messages-to-this-session-just-now)                                                       |
| `Refusing to send: reply target is a symlink` / `Refusing to send: cannot vet reply target`                                                                                                                                                                          | [Errores de herramientas](#refusing-to-send-a-cross-session-message)                                                         |
| `Refusing to send: connected endpoint is not the expected process` / `Refusing to send: connected endpoint identity could not be read`                                                                                                                               | [Errores de herramientas](#refusing-to-send-a-cross-session-message)                                                         |
| `Refusing to send: connected endpoint is not owned by this user` / `Refusing to send: connected endpoint owner could not be read`                                                                                                                                    | [Errores de herramientas](#refusing-to-send-a-cross-session-message)                                                         |
| `Refusing to send: connected endpoint is a different process with the expected pid`                                                                                                                                                                                  | [Errores de herramientas](#refusing-to-send-a-cross-session-message)                                                         |
| `Refusing to read <path>: its symlink resolution changed after permission was checked (<reason>)` / `Refusing to search <path>: its symlink resolution changed after permission was checked`                                                                         | [Errores de herramientas](#refusing-after-a-symlink-changed)                                                                 |
| `Refusing to write <path>: its parent-directory symlink resolution changed after permission was checked` / `Refusing to write <path>: it is a symbolic link. Write to the link's target path instead`                                                                | [Errores de herramientas](#refusing-after-a-symlink-changed)                                                                 |
| `Refusing to write through symlink: <path>` / `Refusing to write into symlinked directory: <path>`                                                                                                                                                                   | [Errores de herramientas](#refusing-after-a-symlink-changed)                                                                 |
| `Refusing to search <path>: a path one of its Read deny rules is written through changed while the search was being prepared` / `Refusing to search <path>: it could not be opened`                                                                                  | [Errores de herramientas](#refusing-after-a-symlink-changed)                                                                 |
| `its permission check expired before it ran (too many concurrent file operations)` / `ripgrep was found only by name on PATH`                                                                                                                                        | [Errores de herramientas](#refusing-after-a-symlink-changed)                                                                 |
| `task output swap refused (tasks dir moved or linked)`                                                                                                                                                                                                               | [Errores de herramientas](#task-output-swap-refused)                                                                         |
| `Command killed: its output file was replaced or could no longer be verified`                                                                                                                                                                                        | [Errores de herramientas](#task-output-swap-refused)                                                                         |
| `the source file is not valid UTF-8 text` / `the source file is not valid UTF-16 text`                                                                                                                                                                               | [Errores de herramientas](#the-source-file-is-not-valid-utf-8-text)                                                          |
| `the source file has the replacement character U+FFFD`                                                                                                                                                                                                               | [Errores de herramientas](#the-source-file-is-not-valid-utf-8-text)                                                          |
| `Reading a local file from outside this session's connected folders, or through a link, needs the approval card`                                                                                                                                                     | [Errores de herramientas](#reading-a-local-file-from-outside-the-connected-folders)                                          |
| `cannot read file_path (...) — the file could not be examined, and no one can answer the approval card`                                                                                                                                                              | [Errores de herramientas](#reading-a-local-file-from-outside-the-connected-folders)                                          |
| `WebFetch cannot fetch localhost or other hostnames without a dot`                                                                                                                                                                                                   | [Errores de herramientas](#webfetch-cannot-fetch-localhost)                                                                  |
| `Can't open MCP settings while no terminal is attached to this background session`                                                                                                                                                                                   | [Errores de sesión en segundo plano](#commands-refused-in-a-background-session)                                              |
| `Can't open MCP settings in a background session`                                                                                                                                                                                                                    | [Errores de sesión en segundo plano](#commands-refused-in-a-background-session)                                              |
| `blocked because the path is spelled in a form that cannot be safely resolved`                                                                                                                                                                                       | [Errores de sesión en segundo plano](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved)                   |
| `blocked because the path is network-shaped`                                                                                                                                                                                                                         | [Errores de sesión en segundo plano](#write-or-command-blocked-because-the-path-names-a-network-location)                    |
| `is isolated in the worktree <path>, but this command <reason>. Refusing to run it`                                                                                                                                                                                  | [Errores de sesión en segundo plano](#command-blocked-by-the-worktree-isolation-checks)                                      |
| `too complex to verify that it stays inside the worktree`                                                                                                                                                                                                            | [Errores de sesión en segundo plano](#command-blocked-by-the-worktree-isolation-checks)                                      |
| `This session has no saved transcript`                                                                                                                                                                                                                               | [Errores de sesión en segundo plano](#this-session-has-no-saved-transcript)                                                  |
| `Can't open — this session is running in another terminal`                                                                                                                                                                                                           | [Errores de sesión en segundo plano](#this-session-is-running-in-another-terminal)                                           |
| `This conversation is already open in another running Claude session`                                                                                                                                                                                                | [Errores de sesión en segundo plano](#this-session-is-running-in-another-terminal)                                           |
| `This session's saved conversation is no longer on disk`                                                                                                                                                                                                             | [Errores de sesión en segundo plano](#this-sessions-saved-conversation-is-no-longer-on-disk)                                 |
| `kept <id> — its worktree is still at <path>`                                                                                                                                                                                                                        | [Errores de sesión en segundo plano](#worktree-has-commits-that-are-not-pushed-anywhere)                                     |
| `kept <id> — <n> unpushed commits on <branch>`                                                                                                                                                                                                                       | [Errores de sesión en segundo plano](#worktree-has-commits-that-are-not-pushed-anywhere)                                     |
| `kept <id> — worktree has commits that are not pushed anywhere`                                                                                                                                                                                                      | [Errores de sesión en segundo plano](#worktree-has-commits-that-are-not-pushed-anywhere)                                     |
| `terminal host process died — press Enter to restart` / `This session's terminal host process died`                                                                                                                                                                  | [Errores de sesión en segundo plano](#terminal-host-process-died)                                                            |
| `Session isn't responding` / `Press enter again to restart this session — it isn't responding`                                                                                                                                                                       | [Errores de sesión en segundo plano](#session-isnt-responding)                                                               |
| `Session <id> was stopped while the respawn was in flight`                                                                                                                                                                                                           | [Errores de sesión en segundo plano](#session-was-stopped-while-the-respawn-was-in-flight)                                   |
| `This session was running agent '<name>', which is no longer available`                                                                                                                                                                                              | [Errores de sesión en segundo plano](#session-agent-no-longer-available)                                                     |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                                                                                                                                                                                          | [Errores de sesión en segundo plano](#claude_code_process_wrapper-launcher-errors)                                           |
| `EUNKNOWN: unknown error, uv_spawn`                                                                                                                                                                                                                                  | [Errores de sesión en segundo plano](#eunknown-when-starting-a-background-session)                                           |
| `EACCES: permission denied, posix_spawn`                                                                                                                                                                                                                             | [Errores de sesión en segundo plano](#eacces-when-starting-a-background-session)                                             |
| `exited before it became reachable`                                                                                                                                                                                                                                  | [Errores de sesión en segundo plano](#background-service-exited-before-it-became-reachable)                                  |
| `Couldn't start a background session (working directory no longer exists or is not accessible: ...)`                                                                                                                                                                 | [Errores de sesión en segundo plano](#working-directory-no-longer-exists-when-starting-a-background-session)                 |
| `Claude Code is being updated by npm on this machine (still not runnable after 2 min, ...)`                                                                                                                                                                          | [Errores de sesión en segundo plano](#eacces-when-starting-a-background-session)                                             |
| `Claude Code process exited with code N`                                                                                                                                                                                                                             | [Errores de contenedor e IDE](#claude-code-process-exited-with-code-n)                                                       |
| `The connection to Claude Code ended before this message completed`                                                                                                                                                                                                  | [Errores de contenedor e IDE](#the-connection-to-claude-code-ended-before-this-message-completed)                            |
| `Could not locate the Claude CLI on PATH`                                                                                                                                                                                                                            | [Errores de contenedor e IDE](#could-not-locate-the-claude-cli-on-path)                                                      |
| `Restored the code, but skipped N files`                                                                                                                                                                                                                             | [Advertencias y errores de Rewind](#restored-the-code-but-skipped-files)                                                     |
| `No files were restored: N files failed (backup missing, or the file could not be updated)`                                                                                                                                                                          | [Advertencias y errores de Rewind](#no-files-were-restored)                                                                  |
| `Transcript writes are failing (...)`                                                                                                                                                                                                                                | [Advertencias de guardado de sesión](#transcript-writes-are-failing)                                                         |
| `Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set`                                                                                                                                                                                                  | [Advertencias de guardado de sesión](#transcript-saving-is-off-skip-prompt-history)                                          |
| `Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker`                                                                                                                                                                                              | [Advertencias de guardado de sesión](#transcript-saving-is-off-child-session-marker)                                         |
| `Claude Code's fullscreen renderer didn't finish starting last time on this machine` / `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`                                                                                            | [Advertencias de configuración](#fullscreen-failed-start-notice)                                                             |
| `Claude Code exited after an unrecoverable interface error (...)`                                                                                                                                                                                                    | [Advertencias de configuración](#exited-after-an-unrecoverable-interface-error)                                              |
| `Agent descriptions are over the 15.0k-token limit`                                                                                                                                                                                                                  | [Advertencias de configuración](#agent-descriptions-are-over-the-15000-token-limit)                                          |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                                                                                                                                                                                  | [Advertencias de configuración](#workspace-has-not-been-trusted)                                                             |
| `is a network path, which cannot be added as a working directory`                                                                                                                                                                                                    | [Advertencias de configuración](#working-directory-is-a-network-path)                                                        |
| `Remote managed settings failed to load (<cause>)`                                                                                                                                                                                                                   | [Advertencias de configuración](#remote-managed-settings-failed-to-load)                                                     |
| `Managed settings were not approved; exiting without applying them.`                                                                                                                                                                                                 | [Advertencias de configuración](#managed-settings-were-not-approved)                                                         |
| `MCP server <name> is blocked by enterprise managed policy`                                                                                                                                                                                                          | [Advertencias de configuración](#mcp-server-is-blocked-by-enterprise-managed-policy)                                         |
| `Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.`                                                                                                                                              | [Advertencias de configuración](#managed-settings-document-could-not-be-parsed)                                              |
| `Managed settings drop-in directory could not be read`                                                                                                                                                                                                               | [Advertencias de configuración](#managed-settings-document-could-not-be-parsed)                                              |
| `otelHeadersHelper failed; telemetry is not being exported. See /status: ...`                                                                                                                                                                                        | [Advertencias de configuración](#otelheadershelper-failed)                                                                   |
| `"crossSessionInbound" must be one of "accept", "hold", "refuse"`                                                                                                                                                                                                    | [Advertencias de configuración](#crosssessioninbound-must-be-one-of-accept-hold-refuse)                                      |
| `headersHelper not run — this workspace has no persisted trust`                                                                                                                                                                                                      | [Advertencias de configuración](#headershelper-not-run)                                                                      |
| `Invalid permission rule "..." was skipped: Malformed Tool(content) rule`                                                                                                                                                                                            | [Advertencias de configuración](#malformed-tool-content-rule)                                                                |
| `... is not matched by file permission checks`                                                                                                                                                                                                                       | [Advertencias de configuración](#is-not-matched-by-file-permission-checks)                                                   |
| `... has a wildcard before the rest of the command`                                                                                                                                                                                                                  | [Advertencias de configuración](#has-a-wildcard-before-the-rest-of-the-command)                                              |
| `CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced`                                                                                                                                                                                           | [Advertencias de configuración](#the-200k-limit-isnt-enforced)                                                               |
| `[claude-code:unrecognized_model]`                                                                                                                                                                                                                                   | [Advertencias de configuración](#unrecognized-model-id-on-a-request)                                                         |
| `Stale sandbox mask files left by a killed session`                                                                                                                                                                                                                  | [Advertencias de configuración](#stale-sandbox-mask-files-left-by-a-killed-session)                                          |
| Las respuestas parecen ser de menor calidad que lo habitual                                                                                                                                                                                                          | [Calidad de respuesta](#responses-seem-lower-quality-than-usual)                                                             |

<h2 id="automatic-retries">
  Reintentos automáticos
</h2>

Claude Code reintenta fallos transitorios hasta 10 veces con retroceso exponencial antes de mostrarle un error. No siempre reintenta un fallo que llega a mitad de la respuesta de Claude. Cuando ve uno de los errores en esta página, Claude Code ya ha realizado los reintentos que corresponden a ese fallo; las listas a continuación indican qué fallos obtienen el presupuesto completo, cuáles obtienen uno más pequeño y cuáles no obtienen ninguno.

Claude Code reintenta estos fallos:

* Errores del servidor, respuestas sobrecargadas y tiempos de espera de solicitud que llegan antes de que cualquier parte de la respuesta de Claude haya sido transmitida.
* Conexiones perdidas. Cuando una conexión se cae a mitad de una solicitud antes de que Claude haya completado cualquier parte de su respuesta, incluido su pensamiento, Claude Code reenvía la solicitud con el mismo retroceso y el turno continúa, incluso si algo de texto ya había comenzado a transmitirse. Cuando se cae después de que Claude ha terminado de pensar pero antes de que haya comenzado cualquier texto o llamada de herramienta, Claude Code en su lugar reenvía la solicitud hasta dos veces en rápida sucesión, y termina el turno con `Connection lost before a response was produced` si la conexión sigue cayendo en ese punto.
* Una conexión que Claude Code detecta fue rota porque su computadora se durmió a mitad de una solicitud. Claude Code la cuenta como una conexión perdida bajo las reglas anteriores; una vez que la etiqueta de reintento nombra la razón específica, lee `Connection lost while your computer was asleep`, y si el turno termina después de que Claude ha terminado de pensar pero antes de cualquier texto o llamada de herramienta, el mensaje lee `Your computer went to sleep before a response was produced`.
* Un flujo de respuesta estancado, cuando los encabezados de respuesta han llegado pero ninguna parte de la respuesta de Claude ha llegado, o cuando Claude ha terminado de pensar pero no ha comenzado ningún texto o llamada de herramienta: Claude Code aborta la conexión estancada y reenvía la solicitud como máximo una vez, fuera del presupuesto de 10 intentos anterior. Si la respuesta se estanca una segunda vez después de que Claude ha terminado de pensar pero antes de cualquier texto o llamada de herramienta, Claude Code termina el turno con `The response stalled before a response was produced`.
* Una solicitud de transmisión a la que la API nunca responde con encabezados de respuesta, en una conexión donde el [plazo del primer byte se ejecuta](/docs/es/network-config#streaming-idle-watchdogs): Claude Code la aborta en el plazo y la reenvía como máximo una vez por solicitud de modelo, dentro del presupuesto de reintentos, luego termina el turno con [No response from API](#no-response-from-api) si ese intento también queda sin respuesta. En otras conexiones, la solicitud espera `API_TIMEOUT_MS`. Cuando establece `CLAUDE_CODE_RETRY_WATCHDOG`, el límite de un reintento no se aplica.
* Limitaciones temporales de 429, pero no el `429` de límite de gasto de una puerta de enlace, que no es una limitación; vea [Spend limit reached](#spend-limit-reached).
  * Cuando ha iniciado sesión con una suscripción de claude.ai, esto incluye limitaciones de 429 que no llevan los encabezados de cuota de su plan. Antes de v2.1.199, Claude Code reintentaba esas limitaciones solo para inicios de sesión con clave API y Enterprise.
* Una solicitud rechazada porque la entrada más `max_tokens` excede el límite de contexto. Reenviarla sin cambios fallaría de la misma manera, por lo que Claude Code reintenta con un `max_tokens` reducido, y deja de reintentar y compacta en su lugar en dos casos:
  * Cuando ninguna reducción puede caber, por ejemplo cuando la conversación misma casi llena la ventana de contexto.
  * Cuando un reintento no puede reducir `max_tokens` más. Antes de v2.1.218, Claude Code podría reenviar una solicitud reducida que aún no cabía, como cuando el presupuesto de pensamiento extendido excedía el contexto restante, hasta que se agotaba el presupuesto de reintentos.
* Una credencial de Google Cloud expirada o faltante en [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), o credenciales de AWS que no se pueden cargar en su máquina. Claude Code descarta sus credenciales en caché e intenta hasta dos veces, luego reporta el error para que pueda reautenticarse de inmediato, como se describe en [Could not load AWS or Google Cloud credentials](#could-not-load-aws-or-google-cloud-credentials). Antes de v2.1.228, Claude Code reintentaba una credencial de Google Cloud fallida a través del presupuesto de reintentos completo antes de mostrar el error.
* Un `401` o `403` de la API de Anthropic, directamente o a través de una [puerta de enlace LLM](/docs/es/llm-gateway), mientras un script [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper) suministra la credencial. Claude Code vuelve a ejecutar el script e intenta de nuevo con su salida fresca, dentro del presupuesto de reintentos completo. Cuando el script mismo falla en la reejecución, Claude Code muestra [Your apiKeyHelper script is failing](#your-apikeyhelper-script-is-failing) en su lugar.

Antes de v2.1.227, `Connection lost before a response was produced` leía `Connection closed while thinking, before producing a response` y `The response stalled before a response was produced` leía `Response stalled while thinking, before producing a response`.

Claude Code no reintenta estos fallos:

* Un fallo de validación de certificado TLS, como un proxy que inspecciona TLS, un paquete `NODE_EXTRA_CA_CERTS` faltante, o un certificado expirado. Claude Code reporta el error en el primer intento, para que pueda corregir la configuración del certificado de inmediato; vea [SSL certificate errors](#ssl-certificate-errors). Claude Code aún reintenta condiciones TLS transitorias como un tiempo de espera de protocolo de enlace. Antes de v2.1.199, Claude Code reintentaba fallos de certificado a través del presupuesto de reintentos completo antes de mostrar el error.
* Un error del servidor, conexión perdida, o flujo estancado que llega después de que Claude ha completado un bloque de texto o una llamada de herramienta, o ha comenzado uno después de terminar su pensamiento, pero antes de terminar la respuesta. Claude Code no vuelve a ejecutar la solicitud, porque eso podría ejecutar las mismas llamadas de herramienta dos veces. Mantiene lo que Claude completó, ejecuta cualquier llamada de herramienta que Claude terminó, y continúa el turno desde sus resultados. Para lo que ve en una sesión interactiva y en una no interactiva, lea [The response above may be incomplete](#the-response-above-may-be-incomplete). Antes de v2.1.199, Claude Code descartaba la salida parcial y reportaba todo el turno como un error cuando un error del servidor llegaba a mitad de la transmisión.
* Un fallo que llega después de que Claude ha terminado la respuesta: nada necesita reintentarse, por lo que Claude Code mantiene la respuesta completa y termina el turno normalmente.
* Una [respuesta de transmisión de Amazon Bedrock con un tipo de contenido inesperado](#bedrock-streaming-response-has-an-unexpected-content-type), porque la puerta de enlace o proxy que reescribe la respuesta reescribiría el reintento de la misma manera. Requiere Claude Code v2.1.208 o posterior.
* Un reintento sin transmisión de una solicitud de transmisión fallida que obtiene un estado de éxito pero [sin mensaje de API de Claude en el cuerpo](#api-returned-an-empty-or-malformed-response). Claude Code termina el turno con ese error.
* Una solicitud que la verificación de política de su organización rechazó, que aparece como una línea `API Error:` que lleva el mensaje de denegación. Los administradores de su organización configuran la verificación con [Inference hooks](https://platform.claude.com/docs/en/manage-claude/inference-hooks), una característica de Claude Enterprise, y el mensaje termina con las instrucciones que configuraron, o por defecto le dice que se comunique con ellos. Claude Code no reenvía la solicitud denegada al mismo modelo o a un [modelo de respaldo](/docs/es/model-config#fallback-model-chains), porque la denegación se trata del contenido de la solicitud en lugar del modelo. Antes de v2.1.239, Claude Code podría reenviar una solicitud denegada, sin transmisión o en un modelo de respaldo configurado, antes de mostrarle la denegación.

<h3 id="what-you-see-while-claude-code-retries-or-waits">
  Lo que ve mientras Claude Code reintenta o espera
</h3>

Mientras reintenta, el spinner muestra una cuenta regresiva `Retrying in Ns · attempt x/y` después de una etiqueta de error. La etiqueta nombra la razón específica del primer intento para fallos en los que puede actuar de inmediato: la red está caída, un protocolo de enlace TLS falló, o alcanzó un límite de velocidad. Para otros errores, inicialmente lee `API error`. A partir de v2.1.198, cambia a la razón específica del tercer intento, o en el intento final cuando `CLAUDE_CODE_MAX_RETRIES` permite menos de tres; las versiones anteriores cambian solo en el intento final.

A partir de v2.1.198, la sugerencia de spinner habitual se suprime durante los reintentos. Una vez que se revela la razón del error, si el fallo es una sobrecarga 529, la línea debajo de la cuenta regresiva también nombra dónde verificar el estado del servicio: `status.claude.com` en la API de Anthropic, o el host del proveedor o puerta de enlace nombrado en el mensaje en otras configuraciones.

Si no llegan datos en el flujo de respuesta durante 20 segundos mientras una solicitud aún está pendiente, el spinner muestra `Waiting for API response · will retry in … · check your network` antes de que haya comenzado ningún reintento. La solicitud aún no ha fallado: la cuenta regresiva se ejecuta hasta el punto donde Claude Code aborta la conexión estancada. Después del aborto, lo que ve depende de qué tan lejos había llegado la respuesta:

* Antes de que Claude haya completado un bloque de texto o una llamada de herramienta, o haya comenzado uno después de terminar su pensamiento, Claude Code reintenta la solicitud o termina el turno con un error. [Automatic retries](#automatic-retries) dice qué estancamientos reintenta y cuántas veces.
* Después de que Claude ha completado un bloque de texto o una llamada de herramienta, o ha comenzado uno después de terminar su pensamiento, pero antes de que Claude haya terminado la respuesta, Claude Code mantiene lo que Claude completó, continúa el turno desde cualquier llamada de herramienta que Claude terminó, y muestra [The response above may be incomplete](#the-response-above-may-be-incomplete). En una sesión no interactiva, y para la respuesta de un subagente en cualquier sesión, Claude Code puede primero solicitar a Claude que continúe la respuesta; esa entrada dice cuándo lo hace y cuándo aún ve el aviso allí.
* Después de que Claude terminó la respuesta, Claude Code termina el turno normalmente.

El banner se borra por sí solo una vez que los datos se reanudan o un reintento tiene éxito. Si reaparece en cada intento, trátelo como un [problema de red](#unable-to-connect-to-api). Antes de v2.1.185, el banner aparecía después de 10 segundos con una redacción diferente.

Mientras Claude consulta el [asesor](/docs/es/advisor), el banner aparece después de 90 segundos sin datos en lugar de 20, porque una revisión de asesor larga puede no enviar nada durante más de 20 segundos. Antes de v2.1.214, el umbral de 20 segundos se aplicaba durante las llamadas de asesor también, por lo que el banner aparecía durante las revisiones de asesor incluso cuando nada estaba mal.

<h3 id="tune-retry-behavior">
  Ajustar el comportamiento de reintentos
</h3>

Puede ajustar el comportamiento de reintentos con estas variables de entorno:

| Variable                                              | Predeterminado | Efecto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :---------------------------------------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/es/env-vars)             | 10             | Número de intentos de reintento. Limitado a 15 a partir de v2.1.186; a partir de v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` aumenta el predeterminado y elimina el límite. Redúzcalo para que los fallos aparezcan más rápido en scripts.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/es/env-vars)          | sin establecer | Establézcalo en `1` en sesiones desatendidas como trabajos de CI para reintentar errores de capacidad `429` y `529` indefinidamente en lugar de fallar después de `CLAUDE_CODE_MAX_RETRIES` intentos. Claude Code falla de inmediato en un `429` que reporta un límite de gasto o créditos de uso agotados, incluso uno de un [límite de gasto de puerta de enlace](#spend-limit-reached) que se reinicia en un horario. Antes de v2.1.239, el vigilante reintentaba estos indefinidamente. Para solicitudes de modo rápido, vea [Handle rate limits](/docs/es/fast-mode#handle-rate-limits). En v2.1.199 o posterior, también aumenta el recuento de reintentos predeterminado para otros errores transitorios, como errores del servidor, tiempos de espera y conexiones perdidas, a 300, aproximadamente tres horas de retroceso, y elimina el límite de 15 en `CLAUDE_CODE_MAX_RETRIES` si establece esa variable explícitamente. |
| [`API_TIMEOUT_MS`](/docs/es/env-vars)                      | 600000         | Tiempo de espera por solicitud en milisegundos. Aumente para redes lentas o proxies. También limita cuánto tiempo Claude Code espera los encabezados de respuesta, descrito en [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/es/env-vars) | sin establecer | Plazo en milisegundos para el primer byte de respuesta de una solicitud de transmisión. Requiere Claude Code v2.1.242 o posterior. Para cómo Claude Code elige el plazo cuando esto no está establecido, vea [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

<h2 id="server-errors">
  Errores del servidor
</h2>

La mayoría de estos errores provienen del proveedor de inferencia: el servicio de Anthropic en la API de Anthropic, y el servicio detrás del punto de conexión de ese proveedor en Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o una puerta de enlace personalizada. [Auto mode cannot determine the safety of an action](#auto-mode-cannot-determine-the-safety-of-an-action) y [Agent terminated early due to an API error](#agent-terminated-early-due-to-an-api-error) también cubren causas de su lado, como una cuenta de Amazon Bedrock que no puede invocar el modelo clasificador o un subagente que alcanzó un límite de uso.

<h3 id="api-error-500-internal-server-error">
  API Error: 500 Internal server error
</h3>

Claude Code muestra el código de estado y el mensaje de error de la API para cualquier respuesta 5xx. El ejemplo a continuación muestra una respuesta 500 en la API de Anthropic:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

La oración final indica dónde verificar la salud del servicio y varía según el proveedor. Las configuraciones de Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry nombran el estado del servicio de ese proveedor. Un `ANTHROPIC_BASE_URL` personalizado nombra el host de la puerta de enlace.

Esto indica un fallo inesperado dentro de la API. No es causado por su prompt, configuración o cuenta.

**Qué hacer:**

* Verifique [status.claude.com](https://status.claude.com), o la página de estado del proveedor nombrada en el mensaje, para incidentes activos
* Espere un minuto y luego envíe su mensaje nuevamente. Su mensaje original sigue en la conversación, así que para un prompt largo puede escribir `try again` en lugar de pegar todo de nuevo.
* Si el error persiste sin incidente publicado, ejecute `/feedback` para que Anthropic pueda investigar con los detalles de su solicitud. Consulte [Report an error](#report-an-error) si `/feedback` no está disponible en su entorno.

<h3 id="api-error-repeated-529-overloaded-errors">
  API Error: Repeated 529 Overloaded errors
</h3>

La API está temporalmente a capacidad en todos los usuarios. Claude Code ya ha reintentado varias veces antes de mostrar este mensaje:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

La oración final varía según el proveedor de la misma manera que el error 500 anterior.

Un 529 no es su límite de uso y no cuenta contra su cuota.

**Qué hacer:**

* Verifique [status.claude.com](https://status.claude.com), o la página de estado del proveedor nombrada en el mensaje, para avisos de capacidad
* Intente de nuevo en unos minutos
* Ejecute `/model` y cambie a un modelo diferente para continuar trabajando, ya que la capacidad se rastrea por modelo. Claude Code le solicita que haga esto cuando un modelo está bajo una carga particularmente alta, por ejemplo `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Request timed out
</h3>

La API no respondió antes del plazo de conexión.

```text theme={null}
Request timed out
```

Esto puede suceder durante períodos de alta carga o cuando el modelo está generando una respuesta muy grande. El tiempo de espera de solicitud predeterminado es de 10 minutos.

**Qué hacer:**

* Reintente la solicitud
* Para tareas de larga duración, divida el trabajo en prompts más pequeños
* Si la causa es una red lenta o un proxy, aumente `API_TIMEOUT_MS` como se describe en [Automatic retries](#automatic-retries)
* Si los tiempos de espera son frecuentes y su red es de otra manera saludable, consulte [Network and connection errors](#network-and-connection-errors) a continuación

<h3 id="no-response-from-api">
  No response from API
</h3>

Claude Code envió una solicitud de transmisión y la API no devolvió encabezados de respuesta dentro del plazo para el primer byte, por lo que Claude Code abortó la solicitud en lugar de esperar el tiempo de espera de solicitud completo `API_TIMEOUT_MS`, 10 minutos por defecto. Claude Code envía la solicitud nuevamente como máximo una vez, si el [retry budget](#tune-retry-behavior) lo permite. Cuando el reintento tampoco recibe respuesta, el turno termina con este mensaje, que muestra cuánto tiempo esperó cada intento. Cuando establece [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/es/env-vars), el límite de un reintento no se aplica y Claude Code reintenta bajo el presupuesto descrito en [Tune retry behavior](#tune-retry-behavior).

```text theme={null}
API Error: No response from API (waited 3m, then 10m on the retry). If a proxy or gateway on your network holds responses until they complete, raise API_TIMEOUT_MS or CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS to wait longer.
```

Claude Code establece el tiempo de espera para los encabezados de respuesta del primer intento y el reintento por separado:

* **First attempt**: [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/es/env-vars) cuando lo establece en 1 o más, limitado entre 10 segundos y 30 minutos. De lo contrario, Claude Code utiliza el tiempo de espera del vigilante a nivel de byte enumerado en [Streaming idle watchdogs](/docs/es/network-config#streaming-idle-watchdogs), por lo que las variables que cambian ese tiempo de espera cambian este tiempo de espera también. De cualquier manera, Claude Code agrega un segundo por cada 32KB del cuerpo de la solicitud.
* **Retry**: un segundo menos que `API_TIMEOUT_MS`, justo bajo 10 minutos por defecto, para que el reintento pueda superar un proxy o puerta de enlace que mantiene la respuesta hasta que se completa la generación. En Amazon Bedrock, el reintento utiliza el mismo plazo que el primer intento, y el mensaje muestra una duración en lugar de dos.

Ningún tiempo de espera excede un segundo menos que un `API_TIMEOUT_MS` positivo, y un `API_TIMEOUT_MS` positivo bajo 11 segundos desactiva el plazo. El vigilante a nivel de byte comienza solo una vez que llegan los encabezados de respuesta, por lo que una respuesta que deja de enviar bytes después de eso sigue las [stalled-stream rules](#automatic-retries) en su lugar.

**Qué hacer:**

* Envíe su mensaje nuevamente. Su mensaje original sigue en la conversación, así que para un prompt largo puede escribir `try again` en lugar de pegar todo de nuevo.
* Si se repite, trátelo como un [network or proxy problem](#unable-to-connect-to-api). Un proxy que acepta la conexión y nunca reenvía la solicitud produce este error en cada intento.
* Si un proxy o puerta de enlace en su red mantiene respuestas hasta que se completan, aumente `API_TIMEOUT_MS` para que el reintento espere más tiempo. En Amazon Bedrock, aumente `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` también.
* Si el primer intento sigue agotando el tiempo de espera y el reintento luego tiene éxito, aumente `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` para que el primer intento también espere lo suficiente.

Antes de v2.1.242, Claude Code esperaba el tiempo de espera de solicitud completo `API_TIMEOUT_MS`, 10 minutos por defecto, antes de fallar una solicitud de transmisión sin respuesta. Antes de v2.1.261, el reintento esperaba el mismo plazo que el primer intento y el mensaje no mostraba duraciones.

<h3 id="the-response-above-may-be-incomplete">
  The response above may be incomplete
</h3>

Una solicitud de transmisión falló mientras la respuesta aún estaba en progreso, después de que Claude completó un bloque de texto o una llamada de herramienta, o había comenzado uno después de terminar su pensamiento. Reenviar la solicitud podría ejecutar las mismas llamadas de herramienta dos veces, por lo que Claude Code mantiene la salida que Claude completó y agrega este aviso en lugar de descartar el turno. Qué variante ve nombra la causa:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection lost mid-response. The response above may be incomplete.
API Error: Your computer went to sleep mid-response. The response above may be incomplete.
API Error: The response stopped arriving. The response above may be incomplete.
```

* `Server error mid-response`: un error de servidor sobrecargado o 5xx a mitad de la transmisión. Esta variante requiere Claude Code v2.1.199 o posterior; antes de eso, ese caso descartaba la salida parcial e informaba todo el turno como un error.
* `Connection lost mid-response`: la conexión se perdió.
* `Your computer went to sleep mid-response`: Claude Code detectó que su computadora se durmió mientras la respuesta se transmitía. Una vez que su computadora se despierte, Claude Code trata la conexión como rota y deja de leer de ella.
* `The response stopped arriving`: la conexión se mantuvo abierta pero dejó de entregar datos, por lo que el vigilante de inactividad de transmisión la abortó. Antes de v2.1.222, Claude Code también podría informar este fallo en conexiones de [gateway](/docs/es/gateways) alcanzadas a través de `ANTHROPIC_BASE_URL` o `ANTHROPIC_AWS_BASE_URL` mientras los pings de keep-alive del servidor aún llegaban, porque contaba solo eventos de respuesta analizados allí; actualizar detiene esos tiempos de espera espurios en esas rutas. Las puertas de enlace alcanzadas a través de una URL base de proveedor como `ANTHROPIC_BEDROCK_BASE_URL` no están envueltas por el vigilante de bytes; consulte [Streaming idle watchdogs](/docs/es/network-config#streaming-idle-watchdogs).

Antes de v2.1.227, `Connection lost mid-response` decía `Connection closed mid-response` y `The response stopped arriving` decía `Response stalled mid-stream`.

En cuatro casos, Claude Code maneja el fallo sin mostrar este aviso de inmediato:

* Anteriormente en la respuesta, Claude Code reintenta el fallo o termina el turno con un error diferente. Consulte [Automatic retries](#automatic-retries).
* Cuando uno de estos fallos llega después de que Claude ha terminado la respuesta, Claude Code mantiene la respuesta completa y termina el turno normalmente, sin este aviso. Antes de v2.1.222, Claude Code mostraba este aviso cuando la conexión se perdía o se estancaba después de que la respuesta terminaba, e informaba el turno como un error aunque la respuesta fuera completa.
* En una [non-interactive session](/docs/es/headless), como una ejecución `-p`, una ejecución de [Agent SDK](/docs/es/agent-sdk/overview), o una [cloud session](/docs/es/claude-code-on-the-web), no tiene que enviar `continue` usted mismo cuando la respuesta cortada está en la conversación principal y contiene texto pero sin llamadas de herramienta: Claude Code mantiene la salida parcial e invita a Claude a continuar desde donde se detuvo, hasta tres veces seguidas. Ve este aviso para tal respuesta solo una vez que Claude Code ha agotado esas continuaciones. Antes de v2.1.246, Claude Code terminaba un turno no interactivo con este aviso en el primer corte.
* En un [subagent](/docs/es/sub-agents#api-errors-in-subagents), ya sea que la sesión sea interactiva o no: cuando su respuesta cortada contiene texto pero sin llamadas de herramienta, Claude Code invita al subagente a continuar. El aviso se convierte en el último mensaje del subagente solo una vez que esas continuaciones se agotan. Antes de v2.1.257, un subagente mostraba este aviso en el primer corte.

**Qué hacer:**

* En una sesión interactiva, lea la respuesta que permanece en la pantalla: Claude Code mantiene cada bloque que Claude completó antes del error, pero descarta un bloque final interrumpido cuando el turno termina, por lo que las oraciones finales o llamadas de herramienta pueden faltar. Responda con `continue` para que Claude continúe desde su último bloque completado.
* En [non-interactive mode](/docs/es/headless) (`-p`):
  * Con la salida de texto predeterminada, Claude Code imprime el último bloque de texto completado que aún mantiene de anteriormente en el turno, seguido de este mensaje. Cuando no mantiene ninguno, Claude Code imprime solo este mensaje, por ejemplo porque Claude Code compactó la conversación a mitad del turno y borró ese texto. Antes de v2.1.219, Claude Code imprimía solo este mensaje en la salida de texto `-p` y descartaba la respuesta que ya había producido.
  * Con `--output-format json` o `stream-json`, Claude Code informa este mensaje en el campo `result`.
  * Para continuar el turno una vez que la conexión sea estable, reanude la sesión y envíe `continue` como se describe en [Continue conversations](/docs/es/headless#continue-conversations).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto mode cannot determine the safety of an action
</h3>

El modelo que [auto mode](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) utiliza para clasificar acciones no pudo producir una decisión, por lo que auto mode no aprobó la acción automáticamente. El mensaje que ve depende de cómo falló el clasificador.

Las lecturas, búsquedas y ediciones dentro de su directorio de trabajo omiten el clasificador, por lo que continúan funcionando en todos estos casos.

Cuando el modelo clasificador no está disponible:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait a moment and then try this action again.
```

Cuando Claude Code puede determinar la categoría de fallo, nombra la categoría entre paréntesis después de `temporarily unavailable`, por ejemplo `<model> is temporarily unavailable (rate-limited), so auto mode cannot determine the safety of <tool> right now`. Las categorías son `(rate-limited)`, `(overloaded)`, `(server error)`, `(timed out)`, y `(connection failed)`. Los límites de velocidad, sobrecargas y errores de servidor son transitorios, y reintentar funciona. Si `(timed out)` o `(connection failed)` se repite, verifique su conexión; consulte [Unable to connect to API](#unable-to-connect-to-api). Antes de v2.1.229, el mensaje nunca nombraba una categoría y decía `Wait briefly and then try this action again`.

Cuando ninguna categoría encaja, el mensaje aparece sin categoría entre paréntesis; más de un fallo produce esa forma. En [Amazon Bedrock](/docs/es/amazon-bedrock), incluyendo el [Mantle endpoint](/docs/es/amazon-bedrock#use-the-mantle-endpoint), también aparece cuando su cuenta de AWS no puede invocar el modelo nombrado en el mensaje, y ese fallo se repite en cada reintento hasta que su cuenta tenga acceso al modelo.

**Qué hacer:**

* Reintente después de unos segundos; Claude ve el mismo mensaje y generalmente reintenta por su cuenta. Un fallo transitorio no está relacionado con [auto mode eligibility](/docs/es/permission-modes#eliminate-prompts-with-auto-mode); no necesita cambiar la configuración
* Si los reintentos siguen fallando, continúe con tareas de solo lectura y vuelva a la acción bloqueada más tarde
* En Amazon Bedrock, si el mensaje regresa en cada reintento, verifique que su cuenta pueda invocar el modelo que nombra: para modelos estándar de Amazon Bedrock, confirme que su [IAM policy](/docs/es/amazon-bedrock#iam-configuration) permite invocarlo; para IDs de modelo de Mantle, [contact your AWS account team](/docs/es/amazon-bedrock#mantle-endpoint-errors)

Cuando una solicitud de clasificador falla porque su token OAuth expiró o fue rotado por otra sesión, Claude Code actualiza el token e reintenta la solicitud una vez, por lo que una expiración de token rutinaria no aparece como este mensaje. Antes de v2.1.216, un token expirado o rotado fallaba en cada solicitud de clasificador, y auto mode negaba cada acción verificada con este mensaje hasta que el token se actualizara.

Cuando el clasificador devolvió una respuesta no analizable:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Qué hacer:**

* Reintente la acción; esto generalmente tiene éxito en el siguiente intento
* Ejecute `claude --debug` y repita la acción para ver la respuesta del clasificador subyacente en el registro de depuración

Cuando una verificación de seguridad de API separada bloqueó la solicitud del clasificador debido al contenido de la conversación anterior:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

Claude Code niega la acción pero le dice a Claude que esto no es un juicio de que la acción sea insegura, y que continúe con otras tareas en lugar de reintentar. Estas denegaciones no cuentan hacia [auto mode's pause thresholds](/docs/es/permission-modes#when-auto-mode-falls-back). En una ejecución `-p` [non-interactive](/docs/es/headless), Claude Code no detiene la ejecución. Lo que Claude recibe depende de dónde solicitó la acción:

* A un [background subagent](/docs/es/sub-agents#run-subagents-in-foreground-or-background) en una ejecución `-p` sin `--input-format stream-json`, Claude Code devuelve un resultado de error que contiene `Agent aborted: auto mode classifier request refused by the safety safeguard in headless mode`
* En todos los demás lugares, incluyendo sesiones interactivas y la conversación principal de una ejecución `-p`, Claude Code devuelve esa denegación a Claude

Antes de v2.1.225, Claude Code contaba estas denegaciones hacia los umbrales de pausa y devolvía el mismo mensaje de rechazo que un bloque de clasificador genuino.

**Qué hacer:**

* Esto no es una decisión sobre su acción. El contenido ya en su conversación activó un filtro de seguridad en la API cuando auto mode envió la conversación al clasificador
* Reintentar no ayudará; el mismo contenido de conversación activará el filtro nuevamente
* En una sesión interactiva, cambie a un [permission mode](/docs/es/permission-modes) diferente para que pueda aprobar la acción cuando se le solicite
* Inicie una conversación nueva sin el contenido que activa

Cuando la conversación ha crecido más que la ventana de contexto del clasificador:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

Lo que sucede con la acción depende de dónde Claude la solicitó:

* En una sesión interactiva, auto mode vuelve a un prompt de permiso normal para esa acción para que pueda aprobarla o negarla manualmente
* A un [background subagent](/docs/es/sub-agents#run-subagents-in-foreground-or-background) en una ejecución `-p` [non-interactive](/docs/es/headless) sin `--input-format stream-json`, Claude Code devuelve un resultado de error que contiene `Agent aborted: auto mode classifier transcript exceeded context window in headless mode`, y la ejecución continúa
* En otro lugar en una ejecución `-p` sin un [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags), no hay prompt para volver, por lo que la acción no se ejecuta y la ejecución continúa

**Qué hacer:**

* En una sesión interactiva, apruebe o niegue la acción en el prompt que aparece
* En una sesión interactiva, ejecute `/compact` para reducir el tamaño de la conversación para que las acciones posteriores se ajusten nuevamente dentro de la ventana del clasificador

<h3 id="the-server-returned-no-safety-verdict">
  The server returned no safety verdict
</h3>

Bajo [server-side classifier review](/docs/es/permission-modes#server-side-classifier-review), auto mode niega una acción cuando el servidor no da un veredicto para ella. La denegación nombra una categoría entre paréntesis cuando Claude Code puede determinar una, como `(timed out)`:

```text theme={null}
The server-side auto mode classifier gave no verdict (timed out), so auto mode cannot determine the safety of <tool>.
```

El resto del mensaje le dice a Claude si un reintento puede ayudar. Antes de algunas de estas denegaciones, Claude Code espera para que el siguiente intento de Claude no siga de inmediato. Durante la espera en una sesión interactiva, el spinner muestra `Auto mode check unavailable` con una cuenta regresiva, y presionar `Esc` interrumpe el turno.

Después de diez respuestas seguidas sin veredicto, auto mode detiene el turno:

```text theme={null}
Auto mode is unavailable — the server returned no safety verdict for the last 10 responses, so Claude stopped. Send a message to try again, or switch out of auto mode.
```

El mensaje de parada aparece en un lugar diferente en cada tipo de sesión:

* En una sesión interactiva, el mensaje aparece como una advertencia en la transcripción y el turno termina
* En una ejecución `-p` [non-interactive](/docs/es/headless), la ejecución termina e informa un error de ejecución. Con la salida de texto predeterminada, el mensaje se imprime en stderr.
* Cuando un [subagent](/docs/es/sub-agents) alcanzó el límite, el subagente se detiene antes de terminar, y Claude recibe lo que produjo con una nota de que auto mode lo detuvo

**Qué hacer:**

* Envíe otro mensaje para que Claude intente de nuevo. El recuento de respuestas comienza de nuevo.
* Si la parada se repite y sus solicitudes pasan a través de una [LLM gateway or proxy](/docs/es/llm-gateway), verifique si corta respuestas de transmisión o las reescribe. [Server-side classifier review](/docs/es/permission-modes#server-side-classifier-review) dice qué comportamiento de puerta de enlace causa denegaciones, y la [gateway compatibility guide](/docs/es/llm-gateway-protocol#feature-pass-through) enumera qué pasar sin cambios.
* Establezca `CLAUDE_CODE_AUTO_MODE_SERVER=0` antes de iniciar Claude Code para usar sus propias solicitudes de clasificador en su lugar. Antes de v2.1.281, Claude Code no leía la variable en una conexión directa a la API de Anthropic.
* Para aprobar las acciones usted mismo en su lugar, [switch out of auto mode](/docs/es/permission-modes#switch-permission-modes)

Antes de v2.1.280, Claude Code negaba cada acción de una respuesta sin veredicto de inmediato y nunca detenía el turno.

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent terminated early due to an API error
</h3>

La solicitud de API de un [subagent](/docs/es/sub-agents) falló terminalmente, por ejemplo porque se alcanzó un límite de uso o los reintentos de un error de servidor se agotaron, por lo que el subagente se detuvo antes de terminar su tarea. Este mensaje requiere Claude Code v2.1.199 o posterior; antes de eso, el texto de error de la API se devolvía a Claude como si fuera el resultado del subagente.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Qué hacer:**

* Haga coincidir el detalle del error después de los dos puntos con su propia sección en esta página, como [Usage limits](#usage-limits) o [Server errors](#server-errors), y siga los pasos de esa sección
* Una vez que el error subyacente se aclare, pida a Claude que reintente la tarea o [resume the subagent](/docs/es/sub-agents#resume-subagents)

Cuando una limitación de velocidad, sobrecarga o error de servidor interrumpe un subagente en primer plano que ya produjo salida de texto, Claude recibe esa salida parcial marcada como incompleta en lugar de este error. Un subagente cuya única salida fueron llamadas de herramienta también obtiene este error; en v2.1.199 eso devolvía un resultado parcial vacío en su lugar. Consulte [API errors in subagents](/docs/es/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Límites de uso
</h2>

La mayoría de los errores en esta sección significan que se ha alcanzado una cuota vinculada a su cuenta o plan. Tres funcionan de manera diferente: [`Server is temporarily limiting requests`](#server-is-temporarily-limiting-requests) es un acelerador del lado del servidor no relacionado con su cuota de plan, [`Usage credits required for 1M context`](#usage-credits-required-for-1m-context) es una verificación de derechos en lugar de una cuota agotada, y [`The prompt to confirm went unanswered`](#the-prompt-to-confirm-went-unanswered) significa que un aviso de consentimiento de créditos de uso se cerró sin respuesta, independientemente de si se alcanzó una cuota.

<h3 id="youve-hit-your-session-limit">
  You've hit your session limit
</h3>

Los planes de suscripción incluyen una asignación de uso móvil. Cuando se agota, verá uno de estos mensajes:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
You've hit your Sonnet limit · resets 3:45pm
```

Claude Code bloquea las solicitudes adicionales hasta la hora de reinicio que se muestra en el mensaje. Los límites de sesión y semanales se comparten entre todos los modelos, por lo que cambiar de modelo no restaura el acceso. Los límites de Opus y Sonnet se aplican solo a las solicitudes a esa familia de modelos, por lo que cambiar a un modelo fuera de la familia con `/model` le permite seguir trabajando.

En una sesión interactiva con una suscripción de claude.ai, Claude Code también puede esperar en la sesión abierta y continuar la tarea interrumpida poco después del reinicio. Mientras espera, una línea en la parte inferior de la sesión dice `Usage limit reached · continuing automatically at 3:45pm · esc to cancel`. Presione `Esc` en un aviso vacío para cancelar la espera. Consulte [Wait for a usage limit to reset](/docs/es/interactive-mode#wait-for-a-usage-limit-to-reset) para ver qué ve, cómo iniciar o cancelar una espera y cómo desactivar la continuación automática. Antes de v2.1.234, Claude Code no ofrecía esta espera.

El uso se cuenta contra las asignaciones de sesión y semanales al mismo tiempo. Una única ráfaga de actividad pesada, como un gran fanout de flujo de trabajo, puede agotar la asignación semanal antes de que se reinicie la ventana de sesión.

**Qué hacer:**

* Espere la hora de reinicio que se muestra en el error
* En la pestaña Code de la [aplicación de escritorio](/docs/es/desktop), la tarjeta de límite de sesión ofrece una casilla de verificación **Auto-continue when limits reset**. La tarjeta de límite semanal no. Cuando está marcada, la aplicación de escritorio reintenta el turno interrumpido después del reinicio y muestra la hora del reintento en la tarjeta. La casilla de verificación de escritorio y la configuración **Continue automatically at usage limit** de la CLI en `/config` son independientes, así que desactive cada una por separado.
* Para el límite de Opus o Sonnet, ejecute `/model` y cambie a un modelo fuera de esa familia para seguir trabajando. Cada modelo tiene su propio caché de aviso, por lo que la siguiente solicitud relee toda la conversación sin aciertos de caché; consulte [Switching models](/docs/es/prompt-caching#switching-models)
* Ejecute `/usage` para ver los límites de su plan y cuándo se reinician
* Ejecute `/usage-credits` para comprar uso adicional en Pro y Max, o para solicitarlo a su administrador en Team y Enterprise. Consulte [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) para ver cómo se factura esto.
* Para actualizar su plan para obtener límites base más altos, consulte [claude.com/pricing](https://claude.com/pricing)

Antes de que se agote una ventana, Claude Code puede advertirle que ha utilizado la mayor parte de ella, con un mensaje como `You've used 85% of your session limit · resets 3:45pm`. Para ver su asignación restante continuamente, agregue los campos `rate_limits` a una [línea de estado personalizada](/docs/es/statusline#rate-limit-usage), o en la aplicación de escritorio haga clic en el [anillo de uso](/docs/es/desktop#check-usage) junto al selector de modelo.

<h3 id="usage-credits-required-for-1m-context">
  Usage credits required for 1M context
</h3>

El modelo seleccionado utiliza la ventana de contexto extendida de 1M tokens, y su plan solo la incluye a través de créditos de uso.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on (they take effect after you restart Claude Code), or /model to switch to standard context
```

Esta es una verificación de derechos, no un agotamiento de cuota. Se activa incluso cuando sus asignaciones de sesión y semanales tienen capacidad restante. Consulte [Extended context](/docs/es/model-config#extended-context) para ver qué planes incluyen contexto de 1M directamente y cuáles requieren créditos de uso. Claude Code ejecuta esta verificación cuando elige el modelo con `/model`, y solo en una conexión directa a la API de Anthropic; si apunta `ANTHROPIC_BASE_URL` a una [puerta de enlace LLM](/docs/es/llm-gateway), `/model` permite la selección `[1m]` y la puerta de enlace decide si la solicitud tiene éxito.

Cuando este error aparece a mitad de la conversación porque el contexto creció más allá de 200K tokens, Claude Code compacta automáticamente la conversación nuevamente bajo el límite de contexto estándar y mantiene la sesión en ese límite después, por lo que no se requiere acción. En versiones anteriores a v2.1.172, el error se repetía en cada solicitud posterior, incluida `/compact`; ejecute `/clear` en esas versiones para recuperarse. Los pasos a continuación se aplican cuando seleccionó explícitamente un modelo `[1m]`.

**Qué hacer:**

* Ejecute `/model` y seleccione la variante sin el sufijo `[1m]` para volver a la ventana de contexto estándar
* Donde el mensaje menciona `/usage-credits`, ejecútelo para activar la facturación medida para la variante de 1M en Pro y Max, o para solicitar créditos de uso a su administrador en Team y Enterprise. Una vez que los créditos de uso estén activados, reinicie Claude Code o inicie una nueva sesión, lo que diga el mensaje. Hasta entonces, la sesión permanece en el límite de contexto estándar.
* Si el error persiste después de `/model`, un ID de modelo de 1M puede estar configurado en otro lugar. Consulte [Setting your model](/docs/es/model-config#setting-your-model) para ver las ubicaciones de configuración a verificar en orden de prioridad.
* Para eliminar variantes de 1M del selector de modelo por completo, establezca [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/es/env-vars)

Antes de v2.1.268, el mensaje terminaba con `run /usage-credits to turn them on, or /model to switch to standard context` y no mencionaba reiniciar.

<h3 id="the-prompt-to-confirm-went-unanswered">
  The prompt to confirm went unanswered
</h3>

Si su cuenta requiere el [consentimiento de créditos de uso de Fable](/docs/es/model-config#fable-and-usage-credits), Claude Code le pide que confirme antes de que una solicitud de Fable facture créditos de uso. Cuando nadie responde ese aviso de consentimiento en una sesión que puede no tener a nadie en su terminal, Claude Code cierra el aviso y termina el turno con uno de estos mensajes:

```text theme={null}
Fable limit reached · continuing on Fable 5.1 uses usage credits, and the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
Fable 5.1 now uses usage credits · the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
```

Los mensajes nombran el modelo Fable de la sesión, por lo que en Fable 5 leen `continuing on Fable 5` y `Fable 5 now uses usage credits`. Antes de v2.1.257, el primer mensaje comenzaba `Fable 5 limit reached`.

Esto sucede en sesiones de [Remote Control](/docs/es/remote-control), [sesiones en segundo plano](/docs/es/agent-view) y sesiones de compañeros de [equipo de agentes](/docs/es/agent-teams). Claude Code muestra el aviso de consentimiento solo en la vista interactiva de la sesión: la terminal donde se ejecuta, o, para una sesión en segundo plano, la [vista de agentes](/docs/es/agent-view) una vez que se adjunta. Un cliente de Remote Control no puede mostrarlo. Claude Code cierra el aviso en la fecha límite de [`dialogExpiry`](/docs/es/settings-reference#dialogexpiry), cinco minutos por defecto, o tan pronto como llega un nuevo aviso mientras nadie ha escrito en esa terminal, como un aviso enviado desde un cliente de Remote Control. Escribir en la terminal donde se ejecuta la sesión cancela la fecha límite, y Claude Code espera su respuesta. En la vista adjunta de una sesión en segundo plano, escribir no cancela la fecha límite, y un nuevo aviso aún cierra el aviso de consentimiento, así que responda antes de que suceda cualquiera de los dos. Claude Code no envía nada y mantiene su modelo, por lo que cuando envía su siguiente aviso, Claude Code muestra el aviso de consentimiento nuevamente.

**Qué hacer:**

* En la terminal donde se ejecuta la sesión, envíe otro aviso y responda el aviso de consentimiento cuando reaparezca. Para una sesión en segundo plano, adjúntese a ella desde la [vista de agentes](/docs/es/agent-view) primero. Reenviar desde un cliente de Remote Control muestra este mensaje nuevamente, porque el cliente no puede mostrar el aviso.
* Ejecute `/model` para cambiar a un modelo que no facture créditos de uso
* Para darse más tiempo para llegar a esa terminal, establezca [`dialogExpiry`](/docs/es/settings-reference#dialogexpiry) en un valor más largo o `"never"`

Antes de v2.1.236, este mensaje no aparecía: mientras un cliente de Remote Control estaba conectado, Claude Code esperaba 60 segundos una respuesta y luego continuaba el turno en su modelo predeterminado.

<h3 id="server-is-temporarily-limiting-requests">
  Server is temporarily limiting requests
</h3>

La API aplicó un acelerador de corta duración que no está relacionado con su cuota de plan.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code distingue estos de su límite de plan por la ausencia de los encabezados de cuota unificada que lleva una respuesta de límite real. A partir de v2.1.199, esto se [reintenta automáticamente](#automatic-retries) con retroceso antes de mostrarse, independientemente de cómo se autentique. En versiones anteriores, una sesión con una suscripción de claude.ai falló el turno en la primera ocurrencia; solo las claves API y los inicios de sesión de Enterprise lo reintentaron.

**Qué hacer:**

* Espere brevemente e intente de nuevo
* Consulte [status.claude.com](https://status.claude.com) si persiste

<h3 id="request-rejected-429">
  Request rejected (429)
</h3>

Ha alcanzado el límite de velocidad configurado para su clave API, proyecto de Amazon Bedrock o proyecto de Google Cloud.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

La oración final nombra dónde verificar el estado del servicio y varía según el proveedor. Las configuraciones de Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry nombran el estado del servicio de ese proveedor en lugar de la página de estado de Anthropic. Un `ANTHROPIC_BASE_URL` personalizado nombra el host de la puerta de enlace.

**Qué hacer:**

* Ejecute `/status` y confirme que la credencial activa es la que espera. Un `ANTHROPIC_API_KEY` extraviado en su entorno puede enrutar solicitudes a través de una clave de nivel bajo en lugar de su suscripción.
* Consulte la consola de su proveedor para ver los límites activos y solicite un nivel más alto si es necesario
* Para claves API de Anthropic, consulte la [referencia de límites de velocidad](https://platform.claude.com/docs/en/api/rate-limits) para ver cómo funcionan los niveles y cómo establecer límites por espacio de trabajo
* Reduzca la concurrencia: baje [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/es/env-vars), evite ejecutar muchos subagentes paralelos, o cambie a un modelo más pequeño con `/model` para ejecuciones de alto volumen con scripts

<h3 id="youve-hit-your-monthly-spend-limit">
  You've hit your monthly spend limit
</h3>

El uso incluido en su plan no puede cubrir esta solicitud, y los [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) que de otro modo pagarían por ella han alcanzado un límite de gasto. Eso sucede cuando una de las ventanas de uso de su plan se ha agotado, o cuando la solicitud es una que solo los créditos de uso pagan, como una solicitud a un modelo que [factura a créditos de uso](/docs/es/model-config#fable-and-usage-credits). El mensaje nombra cuyo límite lo bloqueó. El texto después del `·` dice cómo aumentar ese límite, y varía con su plan y si usted gestiona la facturación:

```text theme={null}
You've hit your monthly spend limit · raise it at claude.ai/settings/usage
You've hit your individual spend limit · ask your admin for a higher limit
You've hit your org's monthly spend limit · visit claude.ai/admin-settings/usage to raise it
You've hit your team's shared budget · ask your admin to raise it at claude.ai/admin-settings/usage
You've hit your channel's monthly spend limit · an org owner or channel manager can raise it in the channel's Claude settings
```

`team's shared budget` es un presupuesto agrupado que un administrador asignó a un grupo al que pertenece; el mensaje no nombra el grupo. `channel's monthly spend limit` es el presupuesto del único canal de Slack en el que se ejecuta la sesión, por lo que su organización aún puede tener presupuesto fuera de él.

Cuando una de las ventanas de su plan es lo que se agotó, el mensaje también dice cuándo se reinicia esa ventana, por ejemplo `· your session limit resets 3:45pm`, y el acceso regresa entonces sin que nadie aumente el límite. En organizaciones con facturación basada en el uso, el mensaje dice `usage limit` en lugar de `spend limit`, como en `You've hit your individual usage limit`.

Antes de v2.1.239, el mensaje no nombraba la hora de reinicio de la ventana del plan. Antes de v2.1.268, el presupuesto agrupado de un grupo producía el mensaje `individual spend limit` en lugar de `team's shared budget`.

Si se conecta a través de una puerta de enlace de aplicaciones Claude y ve `spend limit reached` en minúsculas, ese es el límite de su operador de puerta de enlace en su lugar; consulte [Spend limit reached](#spend-limit-reached).

**Qué hacer:**

* En Pro y Max, aumente su límite de gasto mensual en [**Settings > Usage**](https://claude.ai/settings/usage) en claude.ai, o ejecute `/usage-credits`
* En Team y Enterprise, aumente el límite en [**Admin settings > Usage**](https://claude.ai/admin-settings/usage) si gestiona la facturación, o pida a un administrador que lo haga. `/usage-credits` envía esa solicitud a su administrador por usted
* Para el límite de un canal, pida a un propietario de la organización o al gerente del canal que lo aumente en claude.ai. Consulte [Per-channel limits](https://claude.com/docs/claude-tag/admins/set-spend-limit#per-channel-limits) en la documentación de Claude Tag
* Si el mensaje nombra una hora de reinicio para la ventana de su plan, puede esperar en su lugar
* Ejecute `/usage` para ver las ventanas de su plan y cuándo se reinicia cada una

<h3 id="spend-limit-reached">
  Spend limit reached
</h3>

Se conecta a través de una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) y ha superado un [límite de gasto](/docs/es/claude-apps-gateway-spend-limits) que estableció el operador de su puerta de enlace. La puerta de enlace bloquea sus solicitudes hasta que se reinicia el período nombrado o el operador aumenta el límite. Marca cada respuesta `429` bloqueada con `x-should-retry: false`, por lo que Claude Code muestra este mensaje sin reintentar.

```text theme={null}
spend limit reached (daily; resets 2026-08-09 00:00 UTC)
```

El mensaje nombra el período del límite y la hora de reinicio, y cuando el operador configuró un `blocked_message`, sus instrucciones lo siguen. Antes de v2.1.225, el mensaje leía solo `spend limit reached`; una puerta de enlace en una versión anterior aún envía esa forma más corta.

**Qué hacer:**

* Espere la hora de reinicio que nombra el mensaje, o siga las instrucciones del operador si el mensaje las lleva
* Pida al operador de su puerta de enlace que aumente el límite si lo alcanza rutinariamente

Un mensaje relacionado, `spend limit unavailable`, significa que la puerta de enlace no pudo leer sus registros de gasto y bloqueó la solicitud como precaución en lugar de sobre su límite. Generalmente se resuelve por sí solo; si persiste, informe al operador de su puerta de enlace.

<h3 id="credit-balance-is-too-low">
  Credit balance is too low
</h3>

Su organización de Console se ha quedado sin créditos prepagados, o Claude Code está enviando sus solicitudes con una clave API de Console cuando pretendía usar su suscripción.

```text theme={null}
Credit balance is too low
```

**Qué hacer:**

* Si tiene un plan Pro, Max, Team o Enterprise y ve esto, ejecute `/status` y verifique la fila `API key`. Un `ANTHROPIC_API_KEY` aprobado en su entorno enruta solicitudes a través de esa clave en lugar de su suscripción. Desactívelo en el shell actual y elimínelo de su perfil de shell, luego reinicie `claude`. Ejecute `/login` si aún no ha iniciado sesión con su suscripción.
* Agregue créditos en [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), y considere habilitar la recarga automática allí para que el saldo se rellene antes de que llegue a cero
* Establezca límites de gasto por espacio de trabajo en la Console para evitar que un único proyecto agote el saldo de la organización. Consulte [Manage costs effectively](/docs/es/costs).

<h3 id="could-not-update-your-spend-limit">
  Could not update your spend limit
</h3>

El servidor rechazó un cambio de límite de gasto que realizó desde el aviso que aparece cuando alcanza su límite de gasto.

```text theme={null}
Could not update your spend limit: <reason from the server>
```

Cuando el servidor explica el rechazo, el mensaje termina con esa razón, y reintentar el mismo valor falla nuevamente. Cuando el fallo no tiene una razón proporcionada por el servidor, como una conexión perdida, el mensaje lee `Could not update your spend limit. Press Enter to retry.` y reintentar puede tener éxito. Antes de v2.1.216, Claude Code mostraba la forma genérica para cada fallo.

**Qué hacer:**

* Si el mensaje incluye una razón, elija un límite que la satisfaga, como una cantidad más baja
* Si el mensaje muestra solo la forma genérica, reintente; el fallo puede ser transitorio
* Si el cambio sigue fallando, hágalo desde su [configuración de facturación de claude.ai](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) en el navegador en su lugar

<h2 id="authentication-errors">
  Errores de autenticación
</h2>

Estos errores significan que Claude Code no puede probar quién es usted ante la API. Ejecute `/status` en cualquier momento para ver qué credencial está actualmente activa.

<h3 id="not-logged-in">
  No ha iniciado sesión
</h3>

No hay ninguna credencial válida disponible para esta sesión.

```text theme={null}
Not logged in · Please run /login
```

**Qué hacer:**

* Ejecute `/login` para autenticarse con su suscripción de Claude o su cuenta de Console
* Si esperaba que una variable de entorno lo autenticara, confirme que `ANTHROPIC_API_KEY` está configurada y exportada en el shell donde lanzó `claude`
* Para CI o automatización donde el inicio de sesión interactivo no es posible, configure un script [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper) que obtenga una clave al inicio
* Consulte [Precedencia de autenticación](/docs/es/authentication#authentication-precedence) para entender qué credencial usa Claude Code cuando hay varias presentes

Si se le solicita que inicie sesión repetidamente, consulte [No ha iniciado sesión o el token ha expirado](/docs/es/troubleshoot-install#not-logged-in-or-token-expired) para verificaciones del reloj del sistema y pasos de recuperación del almacenamiento de credenciales de macOS.

<h3 id="could-not-resolve-authentication-method">
  No se pudo resolver el método de autenticación
</h3>

La sesión llegó al cliente de API sin ninguna credencial. Las [sesiones en segundo plano](/docs/es/agent-view) y las sesiones en la nube muestran este mensaje cuando el worker se inicia sin una credencial. Las ejecuciones interactivas, `-p` y Agent SDK informan la misma condición que [No ha iniciado sesión](#not-logged-in) y escriben esta cadena solo en su registro de depuración, así que si la encontró allí, siga esa entrada en su lugar.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

En versiones actuales, el error significa que no había ninguna credencial disponible para el proceso worker. Antes de v2.1.174, una sesión en segundo plano asignada a un worker pre-inicializado inactivo podría fallar de esta manera incluso cuando había credenciales válidas configuradas. Antes de v2.1.176, una sesión en la nube que estuviera inactiva antes de ser reclamada también podría hacerlo. Actualice para recuperarse.

**Qué hacer:**

* Actualice a v2.1.176 o posterior si esto aparece en una sesión en segundo plano o en la nube y sus credenciales ya están configuradas
* Confirme que `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` o sus credenciales del proveedor en la nube están configuradas en el entorno que lanza el worker, no solo en su shell interactivo
* Para Agent SDK, consulte [configuración de autenticación en el inicio rápido](/docs/es/agent-sdk/quickstart#setup)
* Ejecute `/status` en una sesión interactiva en el mismo entorno para confirmar qué fuente de credencial se resuelve

<h3 id="invalid-api-key">
  Clave API inválida
</h3>

La variable de entorno `ANTHROPIC_API_KEY` o el script `apiKeyHelper` devolvieron una clave que la API rechazó, o Claude Code bloqueó una clave de `ANTHROPIC_API_KEY` antes de enviarla.

```text theme={null}
Invalid API key · Fix external API key
```

Cuando el mensaje continúa después de `Fix external API key` con una descripción como `Invalid X-Api-Key header value from ANTHROPIC_API_KEY: it contains a line break at character 41 (120 characters on 2 lines).`, la API nunca vio la clave. Claude Code encontró un carácter que los encabezados HTTP no pueden llevar y detuvo la solicitud antes de enviarla. Consulte [Valor de encabezado de solicitud inválido](#invalid-request-header-value) para saber cómo leer la descripción y corregir el valor.

**Qué hacer:**

* Verifique si hay errores tipográficos y confirme que la clave no ha sido revocada en la [Consola](https://platform.claude.com/settings/keys)
* En el mismo shell, ejecute `env | grep ANTHROPIC`, o en PowerShell `Get-ChildItem Env:ANTHROPIC*`. Herramientas como direnv, complementos de shell dotenv e IDE terminals pueden cargar una clave obsoleta de un archivo `.env` en su proyecto sin que la configure explícitamente.
* Desconfigurar `ANTHROPIC_API_KEY` y ejecutar `/login` para usar autenticación de suscripción en su lugar
* Si la clave proviene de un script [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper), ejecute el script directamente para confirmar que imprime una clave válida en stdout
* Ejecute `/status` para confirmar qué fuente de credencial está usando realmente Claude Code

<h3 id="your-apikeyhelper-script-is-failing">
  Su script apiKeyHelper está fallando
</h3>

Claude Code ejecutó el comando en su configuración [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper) y no obtuvo una clave. Sin una, la solicitud llega a la API con una credencial de marcador de posición, y la API la rechaza con `401`. El panel `Authentication` en la terminal muestra cuál de estos sucedió:

* El comando salió con un error o se agotó el tiempo de espera
* El comando no imprimió nada en stdout
* El comando imprimió algo además de la clave, como un banner de inicio de sesión o una línea de registro. El panel muestra `returned output that cannot be used as an API key` y dice qué está mal, sin repetir la salida. Antes de v2.1.227, Claude Code enviaba lo que el comando imprimía, después de recortar espacios en blanco circundantes.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

En [modo no interactivo](/docs/es/headless), stderr también lleva la razón específica, con el prefijo `apiKeyHelper failed:`.

Claude Code vuelve a ejecutar el script y reintenta la solicitud hasta dos veces más antes de mostrar este mensaje, por lo que el fallo aparece dentro de tres intentos. Antes de v2.1.208, Claude Code gastaba el [presupuesto de reintentos](#automatic-retries) completo reenviando la solicitud con la credencial de marcador de posición y luego informaba un error de autenticación genérico `401` en lugar del fallo del script.

Ejecutar `/login` no ayuda aquí: la salida del helper [tiene precedencia](/docs/es/authentication#authentication-precedence) sobre un inicio de sesión guardado mientras la configuración esté presente.

**Qué hacer:**

* Ejecute el comando configurado en `apiKeyHelper` directamente en su shell para reproducir el fallo
* Si el comando informa una sesión expirada, vuelva a autenticarse con su proveedor de credenciales, por ejemplo iniciando sesión en su SSO o bóveda de secretos nuevamente
* Corrija el comando para que imprima solo la clave en stdout, como un único token de ASCII imprimible de hasta 16.384 caracteres, y salga con código 0. Consulte [rotar credenciales con apiKeyHelper](/docs/es/llm-gateway-connect#rotate-credentials-with-apikeyhelper) para una configuración que funcione.
* Ejecute `/status` para ver el fallo y confirme que `apiKeyHelper` es la fuente de credencial activa. La fila `apiKeyHelper` muestra `Failing` con el detalle del último fallo, como el código de salida y la salida de error del comando, y desaparece después de la siguiente ejecución exitosa. Antes de v2.1.274, `/status` mostraba solo la fuente de credencial, no el fallo.
* Cada vez que el comando falla, su código de salida y salida de error también aparecen en un panel `Authentication` en la terminal. Antes de v2.1.212, el panel se titulaba `Cloud authentication`.

<h3 id="invalid-request-header-value">
  Valor de encabezado de solicitud inválido
</h3>

Un valor que Claude Code estaba a punto de enviar como encabezado de solicitud contiene un carácter que los encabezados HTTP no pueden llevar: un salto de línea, un byte NUL, o un carácter por encima de `U+00FF`, como una comilla curva o un espacio de ancho cero. Claude Code detiene la solicitud antes de que se envíe nada y nombra la variable o configuración a corregir. La causa habitual es una credencial pegada de un documento o chat que llevaba un carácter invisible o un salto de línea extraviado.

Claude Code ejecuta esta verificación cuando envía solicitudes a la API de Claude directamente o a través de una [puerta de enlace LLM](/docs/es/llm-gateway). En un proveedor en la nube de terceros como [Amazon Bedrock](/docs/es/amazon-bedrock), Claude Code no la ejecuta antes de enviar.

```text theme={null}
Invalid auth token · Fix external auth token
Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable
Invalid request header from the environment · Fix the environment variable
```

La primera parte del mensaje depende de dónde provino el valor incorrecto:

* `Invalid auth token`: un token de portador de [`ANTHROPIC_AUTH_TOKEN`](/docs/es/env-vars) o [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/es/env-vars)
* `Invalid ANTHROPIC_CUSTOM_HEADERS`: un nombre o valor de encabezado que configuró en [`ANTHROPIC_CUSTOM_HEADERS`](/docs/es/env-vars). La descripción cuenta cuál par `Name: Value` es el culpable, como `distinct header 2 of 3 parsed from ANTHROPIC_CUSTOM_HEADERS`, sin repetir el nombre o valor, ya que eligió ambos.
* `Invalid request header from the environment`: un valor que Claude Code copia en un encabezado de solicitud desde otra variable de entorno, como `CLAUDE_AGENT_SDK_CLIENT_APP`. La descripción nombra la variable a corregir.

Claude Code informa una `ANTHROPIC_API_KEY` incorrecta detectada por esta verificación como [Clave API inválida](#invalid-api-key), con la misma descripción final. Informa una credencial `/login` guardada incorrecta como [No ha iniciado sesión](#not-logged-in) en su lugar; ejecute `/login` para guardar una nueva. La salida de un script [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper) nunca llega a esta verificación: Claude Code la valida cuando se ejecuta el script, y la salida que un encabezado HTTP no puede llevar falla con [Su script apiKeyHelper está fallando](#your-apikeyhelper-script-is-failing).

Después del segundo `·`, el mensaje describe el problema, como en este ejemplo completo:

```text theme={null}
Invalid auth token · Fix external auth token · Invalid Authorization header value from ANTHROPIC_AUTH_TOKEN: it contains a line break at character 41 (120 characters on 2 lines).
```

Las posiciones cuentan caracteres comenzando en uno. La descripción se construye a partir de frases fijas y conteos de caracteres, por lo que nunca incluye el valor en sí. Nombra el carácter ofensivo solo cuando es un carácter invisible o tipográfico bien conocido, como una marca de orden de bytes, un espacio de ancho cero o una comilla curva, e informa cualquier otra cosa como `a non-ASCII character`.

**Qué hacer:**

* Vuelva a configurar la variable o configuración que el mensaje nombra, reescribiendo los caracteres alrededor de la posición informada en lugar de pegar desde la misma fuente nuevamente
* Para `ANTHROPIC_CUSTOM_HEADERS`, mantenga un par `Name: Value` por línea y reescriba el par que el mensaje cuenta
* Ejecute `/status` para confirmar qué fuente de credencial está activa

<h3 id="this-organization-has-been-disabled">
  Esta organización ha sido deshabilitada
</h3>

Claude Code está usando una `ANTHROPIC_API_KEY` obsoleta de una organización de Console deshabilitada. Cuando tiene un inicio de sesión de suscripción guardado, la clave lo anula.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your subscription instead
Your ANTHROPIC_API_KEY belongs to a disabled organization · Update or unset the environment variable
API Error: 400 ... This organization has been disabled.
```

La sugerencia después del `·` depende de sus credenciales guardadas: la primera forma aparece cuando un `/login` almacenado puede tomar el control después de desconfigurar la clave, y la segunda cuando la clave es su única credencial.

Las variables de entorno tienen precedencia sobre `/login`, por lo que una clave exportada en su perfil de shell o cargada desde un archivo `.env` se usa incluso cuando tiene una suscripción Pro o Max que funciona. En modo no interactivo (`-p`), la clave siempre se usa cuando está presente.

**Qué hacer:**

* Desconfigurar `ANTHROPIC_API_KEY` en el shell actual y eliminarla de su perfil de shell, luego relance `claude`
* Si el mensaje dice `Update or unset`, no tiene ningún inicio de sesión guardado para recurrir. Desconfigurar la clave y ejecutar `/login`, o reemplazar la clave con una de una organización de Console activa.
* Ejecute `/status` después para confirmar que la credencial activa es su suscripción
* Si no hay ninguna variable de entorno configurada y el error persiste, la organización deshabilitada es la vinculada a su `/login`. Contacte con soporte o inicie sesión con una cuenta diferente.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Su organización ha deshabilitado la autenticación por clave API
</h3>

Este mensaje requiere Claude Code v2.1.169 o posterior. El administrador de su organización de Console ha desactivado la autenticación por clave API, por lo que la API rechaza la clave que Claude Code está enviando. La sugerencia de recuperación después del `·` varía según de dónde provino la clave:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Las variables de entorno y `apiKeyHelper` tienen precedencia sobre `/login`, por lo que ejecutar `/login` solo no ayuda mientras cualquiera de ellas siga suministrando una clave. Consulte [Precedencia de autenticación](/docs/es/authentication#authentication-precedence).

**Qué hacer:**

* Si el mensaje nombra `ANTHROPIC_API_KEY`, desconfígurela en el shell actual y elimínela de su perfil de shell o archivo `.env`, luego relance `claude`
* Si el mensaje nombra `apiKeyHelper`, elimine la configuración [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper) de su `settings.json`
* Ejecute `/login` para iniciar sesión con su cuenta de claude.ai
* Ejecute `/status` después para confirmar que la credencial activa es su suscripción en lugar de una clave API
* Si necesita autenticación por clave API para automatización, pida al administrador de su organización que la vuelva a habilitar en la Consola

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Su organización ha deshabilitado el acceso a la suscripción de Claude
</h3>

Su organización de Claude no permite iniciar sesión en Claude Code con un inicio de sesión de suscripción. Ejecutar `/login` nuevamente con la misma cuenta devuelve el mismo error.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Esta es una configuración de organización del lado del servidor, por lo que no se puede anular desde configuraciones locales, variables de entorno o banderas CLI.

Agent SDK y el modo no interactivo `-p` muestran esto como el código de error `oauth_org_not_allowed`.

**Qué hacer:**

* Pida a su administrador que habilite el acceso a Claude Code para su organización
* Autentíquese con una clave API de Console en lugar de su suscripción. Consulte [Autenticación de Claude Console](/docs/es/authentication#claude-console-authentication) para la configuración.
* Si es el administrador y no ve una opción para habilitar el acceso, contacte con [soporte de Anthropic](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Las rutinas están deshabilitadas por la política de su organización
</h3>

Un Propietario en su organización de Equipo o Empresa ha desactivado las rutinas a nivel de organización. El error aparece cuando intenta crear o ejecutar una rutina, por ejemplo desde la interfaz de usuario de [Routines](/docs/es/routines) en claude.ai/code. En Claude Code v2.1.227 o posterior, la misma configuración también [oculta `/schedule`](/docs/es/routines#troubleshooting) en la CLI.

```text theme={null}
Routines are disabled by your organization's policy.
```

Esta es una configuración del lado del servidor, por lo que no se puede anular desde configuraciones locales, variables de entorno o banderas CLI.

**Qué hacer:**

* Pida a un Propietario en su organización que habilite el botón de alternancia **Routines** en [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Para trabajo programado único que no requiere rutinas a nivel de organización, consulte [tareas programadas](/docs/es/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control requiere la API de Anthropic
</h3>

La sesión no está hablando con la API de Anthropic directamente, por lo que no hay ningún backend de claude.ai para que [Remote Control](/docs/es/remote-control) se empareje.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com. CLAUDE_CODE_USE_BEDROCK is set, so this session is using Amazon Bedrock — unset it (or run in a shell without it) to use Remote Control.
```

Una segunda oración explica qué enrutó la sesión lejos de la API de Anthropic; antes de v2.1.219, el mensaje era solo la primera oración. Dependiendo de la causa, el mensaje nombra:

* Una variable de proveedor `CLAUDE_CODE_USE_*`, como `CLAUDE_CODE_USE_BEDROCK` para [Amazon Bedrock](/docs/es/amazon-bedrock) o `CLAUDE_CODE_USE_VERTEX` para [Plataforma de Agentes de Google Cloud](/docs/es/google-vertex-ai)
* [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) apuntando a un host que no sea `api.anthropic.com`, como una [puerta de enlace LLM](/docs/es/llm-gateway) o proxy, incluso cuando inicia sesión con claude.ai; antes de v2.1.196, una URL base personalizada no bloqueaba Remote Control
* `ANTHROPIC_UNIX_SOCKET` configurado, por lo que la sesión envía sus solicitudes a través de un socket local en lugar de a `api.anthropic.com`
* Un inicio de sesión de [puerta de enlace en la nube](/docs/es/claude-apps-gateway) empresarial realizado a través de `/login`, que no admite Remote Control y no tiene ninguna variable para desconfigurar

**Qué hacer:**

* Desconfigurar la variable que el mensaje nombra, como `CLAUDE_CODE_USE_BEDROCK` o `ANTHROPIC_BASE_URL`, y reiniciar la sesión, o iniciar Remote Control desde una sesión que hable con la API de Anthropic directamente
* Si la variable no está configurada en su shell, verifique la clave `env` en sus [archivos de configuración](/docs/es/settings#where-settings-live), que aplica variables de entorno a cada sesión
* Para este y los otros mensajes de inicio de Remote Control, consulte [Solucionar problemas de Remote Control](/docs/es/remote-control#troubleshooting)

<h3 id="remote-control-couldnt-refresh-your-login">
  Remote Control no pudo actualizar su inicio de sesión
</h3>

Claude Code ejecuta una conexión [Remote Control](/docs/es/remote-control) en vivo en credenciales de corta duración que obtiene y renueva usando su inicio de sesión guardado de claude.ai. Cuando claude.ai deja de aceptar ese inicio de sesión, o Claude Code no tiene ningún inicio de sesión guardado, Claude Code detiene Remote Control y necesita que inicie sesión nuevamente. Cualquiera de los dos fallos puede ocurrir mientras Claude Code aún se está conectando o más tarde, cuando renueva las credenciales.

Cuando Claude Code le pide al servicio de inicio de sesión que actualice su inicio de sesión guardado y no obtiene respuesta, mantiene Remote Control en funcionamiento e intenta la actualización nuevamente mientras la credencial actual de la conexión sigue siendo válida. Una actualización no obtiene respuesta cuando Claude Code no puede alcanzar el servicio de inicio de sesión, la solicitud se agota el tiempo de espera, o el servicio falla sin rechazar su inicio de sesión. Si el servicio de inicio de sesión aún no está respondiendo cuando esa credencial expira, Claude Code detiene Remote Control e informa `OAuth token refresh failed`.

Cuando Claude Code detiene Remote Control, muestra la razón en una advertencia y en una línea de transcripción que comienza con `Remote Control disconnected`. Su sesión local sigue ejecutándose sin Remote Control. Esta sección cubre estas líneas:

```text theme={null}
Remote Control disconnected — Claude.ai login expired — run /login to restore Remote Control
Remote Control disconnected — Claude.ai login expired — run /login, then /remote-control
Remote Control disconnected — Claude.ai login was rejected — run /login, then /remote-control
Remote Control disconnected — OAuth token unavailable — run /login to restore Remote Control
Remote Control disconnected — OAuth token refresh failed — run /login to re-authenticate
Remote Control disconnected — JWT refresh failed: no OAuth token — run /login
Remote Control disconnected — Signed out of Claude — run /login, then /remote-control
```

Claude Code nombra la causa en el medio del mensaje:

* `Claude.ai login expired` y `Claude.ai login was rejected`: claude.ai ya no acepta su token de inicio de sesión guardado, porque expiró o fue revocado
* `OAuth token unavailable`: Claude Code no tenía ningún token de inicio de sesión guardado cuando la credencial de la conexión vencía para renovación
* `OAuth token refresh failed`: claude.ai rechazó su token de inicio de sesión guardado mientras Claude Code se estaba reconectando, y actualizar el token no produjo uno nuevo
* `JWT refresh failed: no OAuth token`: Claude Code no encontró ningún token de inicio de sesión guardado para renovar
* `Signed out of Claude`: cerró sesión en esta máquina, por ejemplo ejecutando `/logout` en otra terminal, por lo que Claude Code no tiene ningún inicio de sesión guardado para renovar la conexión

**Qué hacer:**

* Ejecute `/login` para iniciar sesión nuevamente
* Ejecute `/remote-control` para reconectar la sesión. Los mensajes que terminan `run /login to restore Remote Control` no necesitan este paso: Claude Code se reconecta automáticamente una vez que inicia sesión.

Antes de v2.1.224, `OAuth token refresh failed — run /login to re-authenticate` leía `OAuth token refresh failed — re-authenticate, then re-enable Remote Control`, y `JWT refresh failed: no OAuth token — run /login` leía `no OAuth token available for recovery (code <N>)`. Los mensajes `Claude.ai login expired`, `Claude.ai login was rejected` y `OAuth token unavailable` se agregaron en v2.1.225.

Antes de v2.1.238, Claude Code informaba los casos que ahora dicen `Signed out of Claude` como `JWT refresh failed: no OAuth token — run /login`, y detenía Remote Control con `Claude.ai login expired — run /login to restore Remote Control` tan pronto como una actualización de inicio de sesión no obtuviera respuesta.

<h3 id="remote-control-stopped-because-the-signed-in-account-changed">
  Remote Control se detuvo porque la cuenta con la que inició sesión cambió
</h3>

Claude Code muestra esta línea durante una sesión de [Remote Control](/docs/es/remote-control) cuando inicia sesión en una cuenta de claude.ai diferente u organización en esta máquina. Realizó el cambio fuera de la sesión de Claude Code, por ejemplo ejecutando `/login` en otra terminal.

Una sesión de Remote Control que inició mientras estaba conectado a través de `/login` pertenece a la cuenta de claude.ai y la organización que estaban conectadas en ese momento.

```text theme={null}
Remote Control disconnected — signed-in claude.ai account or organization changed on this machine — run /remote-control to start a session for the current account, or /login to switch back, then /remote-control
```

Claude Code detiene la sesión de Remote Control tan pronto como claude.ai confirma que la cuenta u organización cambió. Su sesión local sigue ejecutándose sin Remote Control.

**Qué hacer:**

* Ejecute `/remote-control` para iniciar una nueva sesión de Remote Control bajo la cuenta u organización actual
* Para cambiar, ejecute `/login` e inicie sesión en la cuenta u organización anterior nuevamente. Luego ejecute `/remote-control`.

Antes de v2.1.234, Claude Code no notaba cuando cambiaba a una cuenta u organización diferente fuera de la sesión de Claude Code. Claude Code mantuvo la sesión de Remote Control conectada hasta que una solicitud posterior al servidor de Remote Control falló con `Remote Control server rejected the request (HTTP 404)`. Ese fallo podría venir horas después del cambio.

<h3 id="remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts">
  Remote Control se detuvo porque la aplicación que ejecuta la sesión cerró sesión o cambió de cuenta
</h3>

Cuando la aplicación de escritorio de Claude o un IDE aloja su sesión, Claude Code obtiene su token de inicio de sesión de esa aplicación en lugar de `/login`. Cuando claude.ai rechaza ese token, Claude Code le pide a la aplicación uno nuevo. Si la aplicación responde que ha cerrado sesión, o que ahora está conectada a una cuenta de Claude diferente, Claude Code finaliza la sesión de [Remote Control](/docs/es/remote-control) y envía a la aplicación una de estas líneas:

```text theme={null}
Remote Control stopped — the app running this session is now signed in to a different Claude account
Remote Control stopped — the app running this session is signed out of Claude. Sign in there, then turn Remote Control back on
```

Su sesión local sigue ejecutándose sin Remote Control.

**Qué hacer:**

* Si la aplicación ha cerrado sesión, inicie sesión en ella nuevamente, luego active Remote Control nuevamente en la aplicación
* Si la aplicación cambió de cuenta, Claude Code no puede continuar la sesión finalizada bajo la nueva cuenta. Inicie una nueva sesión de Remote Control bajo esa cuenta.

Antes de v2.1.238, Claude Code enviaba a la aplicación los mensajes `/login` enumerados en [Remote Control no pudo actualizar su inicio de sesión](#remote-control-couldnt-refresh-your-login) en ambos casos.

<h3 id="oauth-token-revoked-or-expired">
  Token OAuth revocado o expirado
</h3>

Su inicio de sesión guardado ya no es válido. Un token revocado significa que cerró sesión en todas partes o un administrador eliminó el acceso; un token expirado significa que la actualización automática falló a mitad de sesión.

Ambos mensajes informan un rechazo que la API devolvió para una solicitud que Claude Code envió. Cuando el inicio de sesión guardado ya ha sido borrado después de una actualización fallida, ve [Inicio de sesión expirado](#login-expired) en su lugar. Si se autentica con un token de larga duración en [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/es/env-vars), ve los mismos mensajes cuando ese token expira o es revocado.

```text theme={null}
OAuth token revoked · Please run /login
Please run /login · API Error: 401 OAuth token has expired ...
```

**Qué hacer:**

* Ejecute `/login` para iniciar sesión nuevamente
* Si el error se repite dentro de la misma sesión después de volver a autenticarse, ejecute `/logout` primero para borrar completamente el token almacenado, luego `/login`
* Si se autentica con la variable de entorno `CLAUDE_CODE_OAUTH_TOKEN`, Claude Code sigue enviando el valor que configuró después de que una solicitud falla con un 401, en lugar de cambiar al token de un inicio de sesión guardado. [`/status`](/docs/es/commands) muestra esta credencial como una fila `Auth token` que lee `CLAUDE_CODE_OAUTH_TOKEN`. Genere un token nuevo con [`claude setup-token`](/docs/es/authentication#generate-a-long-lived-token) y reinicie con él, o desconfigurar la variable y ejecutar `/login`. Antes de v2.1.225, Claude Code podría reemplazar el valor de la variable a mitad de sesión con el token de acceso de corta duración de un inicio de sesión guardado, y la sesión falló con errores 401 nuevamente una vez que ese token expiró.
* Para solicitudes repetidas de inicio de sesión entre lanzamientos, consulte las verificaciones del reloj del sistema y los pasos de recuperación del almacenamiento de credenciales de macOS en [Solución de problemas](/docs/es/troubleshoot-install#not-logged-in-or-token-expired)
* Para otros fallos incluyendo `403 Forbidden` y problemas del navegador OAuth, consulte [Inicio de sesión y autenticación](/docs/es/troubleshoot-install#login-and-authentication)

<h3 id="api-error-401-invalid-authentication-credentials">
  Error de API: 401 Credenciales de autenticación inválidas
</h3>

La API reconoció el formato de su credencial pero rechazó la cuenta u organización detrás de ella. Anthropic devuelve este mensaje cuando una credencial fue revocada recientemente, cuando una organización fue deshabilitada o eliminó su acceso, o cuando la cuenta misma fue desactivada, por lo que un token expirado no es la causa. La credencial puede ser su inicio de sesión guardado o una `ANTHROPIC_API_KEY` aprobada, y la solución difiere, así que comience ejecutando `/status` para ver cuál está activa.

```text theme={null}
Please run /login · API Error: 401 Invalid authentication credentials
```

**Qué hacer:**

* Si `/status` muestra una fila `API key` que no está marcada como no en uso, una [`ANTHROPIC_API_KEY`](/docs/es/authentication#authentication-precedence) aprobada es la credencial activa y tiene precedencia sobre su inicio de sesión, por lo que `/login` no la reemplaza. Rote la clave en la Consola de Claude, o recurra a su suscripción ejecutando `unset ANTHROPIC_API_KEY`, o en PowerShell `Remove-Item Env:ANTHROPIC_API_KEY`.
* Si `/status` muestra solo su inicio de sesión, ejecute `/login` una vez. Si la credencial fue revocada, un nuevo inicio de sesión la reemplaza.
* Si el mismo mensaje se repite para la misma cuenta de inicio de sesión, la cuenta u organización ya no está activa. Verifique la cuenta y organización que `/status` informa, y pida al administrador de su organización que restaure el acceso.
* Si [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) apunta a una [puerta de enlace LLM](/docs/es/llm-gateway), el texto después de `401` es el mensaje de su puerta de enlace en lugar del de Anthropic, y `/login` no lo cambia. Corrija la credencial que su puerta de enlace espera en su lugar.

<h3 id="login-expired">
  Inicio de sesión expirado
</h3>

Claude Code intentó renovar su inicio de sesión guardado de claude.ai o Claude Console y el servicio OAuth rechazó el token de actualización almacenado, por lo que Claude Code borró las credenciales guardadas. Después de eso, cada solicitud de modelo se detiene localmente con este mensaje antes de llegar a la API, porque solo `/login` puede crear nuevas credenciales.

Antes de v2.1.206, Claude Code enviaba la solicitud de modelo de todas formas con cualquier credencial que permaneciera en el entorno, y cada modelo fallaba con [Hay un problema con el modelo seleccionado](#theres-an-issue-with-the-selected-model) o un 401 en lugar de un aviso para iniciar sesión.

```text theme={null}
Login expired · Please run /login
```

En [modo no interactivo](/docs/es/headless) (`-p`) y [Agent SDK](/docs/es/agent-sdk/overview), el mensaje se lee de la siguiente manera, y el código de error estructurado es `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Este no es el mismo estado que [Token OAuth revocado o expirado](#oauth-token-revoked-or-expired). Esos mensajes informan un rechazo que la API devolvió. Claude Code mismo produce `Login expired` para un inicio de sesión que ya falló al renovar, por lo que no envía ninguna solicitud. Cuando la renovación falla porque la cuenta misma está suspendida en lugar de que el inicio de sesión sea obsoleto, Claude Code muestra [Su cuenta está en espera](#your-account-is-on-hold) en su lugar.

Las sesiones autenticadas con una clave API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/es/env-vars), o un proveedor de terceros no usan el inicio de sesión guardado y nunca ven este mensaje.

Puede verificar este estado antes de que una solicitud falle: [`/status`](/docs/es/commands) muestra una fila `Login` que lee `Expired — log in again`, más la organización y correo electrónico que tiene guardados para el inicio de sesión expirado. La fila aparece solo cuando el inicio de sesión guardado es su credencial activa y ya no se puede renovar. Las sesiones autenticadas de otra manera no muestran la fila, incluso si un inicio de sesión expirado permanece guardado. Antes de v2.1.210, `/status` no daba ninguna indicación en este estado de que un inicio de sesión hubiera existido alguna vez, porque la credencial borrada no le dejaba nada que informar.

**Qué hacer:**

* Ejecute `/login` para iniciar sesión nuevamente. Reintentar sin iniciar sesión muestra el mismo mensaje en cada solicitud.
* En modo no interactivo, ejecute `claude` en el mismo entorno, complete `/login`, luego reejecutar su comando. Para automatización que no puede iniciar sesión interactivamente, autentíquese con `ANTHROPIC_API_KEY` o [genere un token de larga duración con `claude setup-token`](/docs/es/authentication#generate-a-long-lived-token).
* Si iniciar sesión sigue fallando, consulte [Inicio de sesión y autenticación](/docs/es/troubleshoot-install#login-and-authentication)

<h3 id="claude-login-not-accepted">
  Inicio de sesión de Claude no aceptado
</h3>

Intentó iniciar una [sesión en la nube](/docs/es/claude-code-on-the-web), y el servidor rechazó crearla con un 401: no aceptó el inicio de sesión de Claude que esta máquina envió, generalmente porque el inicio de sesión expiró o fue revocado.

La primera parte de la línea es la razón propia del servidor cuando da una. De lo contrario, la línea se lee:

```text theme={null}
Claude login not accepted · Run /login, then try again
```

**Qué hacer:**

* Ejecute `/login`, complete el inicio de sesión, luego inicie la sesión nuevamente

<h3 id="artifacts-need-a-claude-ai-login">
  Los artefactos necesitan un inicio de sesión de claude.ai
</h3>

Claude Code rechazó una publicación o lectura de [artefactos](/docs/es/artifacts) porque la sesión no tiene ningún inicio de sesión de claude.ai que pueda usar para artefactos.

Cada forma del mensaje comienza con las mismas palabras, seguidas de un remedio que depende de cómo se autentica su sesión. Sin ninguna credencial competidora se lee:

```text theme={null}
Artifacts need a claude.ai login. Run /login and select "Claude account with subscription", then retry — the "Anthropic Console account" option does not provide claude.ai credentials.
```

**Qué hacer:**

* Ejecute `/login` y seleccione **Claude account with subscription**. La opción **Anthropic Console account** no proporciona credenciales de claude.ai.
* Cuando el mensaje nombra una credencial que tiene precedencia, como `ANTHROPIC_API_KEY`, una configuración `apiKeyHelper`, o una clave de Console guardada por un `/login` anterior, elimínela de la manera que el mensaje dice, luego ejecute `/login`
* Cuando el mensaje dice que esta sesión remota se autentica a través de la máquina que la lanzó, inicie sesión en claude.ai en esa máquina, luego reconecte la sesión
* Cuando el mensaje dice que la credencial es inyectada por el entorno anfitrión de la sesión, no puede cambiarla en esa sesión; inicie una sesión que esté conectada a claude.ai
* Consulte [Disponibilidad](/docs/es/artifacts#availability) para los otros requisitos que tienen los artefactos, como plan, proveedor de modelo y política de organización

<h3 id="administrator-policy-requires-a-cloud-gateway-sign-in">
  La política del administrador requiere un inicio de sesión de puerta de enlace en la nube
</h3>

La [configuración administrada](/docs/es/managed-settings) de un administrador en esta máquina estableció [`forceLoginMethod`](/docs/es/settings-reference#forceloginmethod) a `"gateway"` o estableció [`forceLoginGatewayUrl`](/docs/es/settings-reference#forcelogingatewayurl). A menos que seleccione un proveedor en la nube a través de una variable como `CLAUDE_CODE_USE_BEDROCK`, Claude Code entonces acepta solo el inicio de sesión de [puerta de enlace de aplicaciones de Claude](/docs/es/claude-apps-gateway). Ve uno de dos mensajes:

```text theme={null}
Not signed in to the Cloud gateway — run /login.
```

Las solicitudes de modelo fallan con este mensaje cuando la sesión no tiene ningún inicio de sesión de puerta de enlace, por ejemplo porque no ha ejecutado `/login` desde que la política llegó a la máquina.

Si también tiene una credencial `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper` configurada y la configuración administrada establece `forceLoginMethod`, Claude Code sale al inicio con un mensaje que comienza:

```text theme={null}
Administrator policy requires a Cloud gateway sign-in on this machine; the
Anthropic-issued credential configured here (ANTHROPIC_API_KEY,
ANTHROPIC_AUTH_TOKEN, or apiKeyHelper) is not used.
```

**Qué hacer:**

* Ejecute `/login` y complete el inicio de sesión en la pantalla **Cloud gateway**
* Para el mensaje de inicio, elimine la configuración `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper` que configuró, luego inicie `claude` y ejecute `/login`
* Si cree que la máquina no debería requerir la puerta de enlace, pida al administrador que la administra que elimine `forceLoginMethod` y `forceLoginGatewayUrl` de su configuración administrada

En v2.1.265, una regresión también mostró el primer mensaje en algunas configuraciones de puerta de enlace LLM y proxy que se autentican con una clave API, `apiKeyHelper` o encabezados personalizados, incluso sin ningún requisito de administrador en la máquina. Actualice a v2.1.266 o posterior. No necesita cambiar su configuración.

Antes de v2.1.261, en máquinas que establecen `forceLoginMethod` a `"gateway"`, Claude Code usaba un inicio de sesión guardado restante en lugar de fallar solicitudes de modelo, e informaba una credencial de entorno configurada con `This machine's managed settings require a first-party login` en lugar del mensaje de inicio. Antes de v2.1.265, una máquina cuya configuración administrada establecía solo `forceLoginGatewayUrl` no requería el inicio de sesión de puerta de enlace, y Claude Code usaba una credencial restante allí.

<h3 id="your-account-is-on-hold">
  Su cuenta está en espera
</h3>

La cuenta de Claude detrás de su inicio de sesión ha sido suspendida. Claude Code muestra el primer mensaje cuando intenta renovar su inicio de sesión guardado y se entera de la suspensión, y el segundo cuando un inicio de sesión que completa en el navegador lo informa:

```text theme={null}
Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted
Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted
```

Iniciar sesión nuevamente con la misma cuenta no borra el mensaje, porque la suspensión está en la cuenta en lugar del inicio de sesión. En [modo no interactivo](/docs/es/headless) (`-p`) y [Agent SDK](/docs/es/agent-sdk/overview), el código de error estructurado es `account_on_hold`. Antes de v2.1.235, Claude Code informaba una cuenta suspendida como [Inicio de sesión expirado · Por favor ejecute /login](#login-expired), cuyos pasos de recuperación no pueden borrar una suspensión.

**Qué hacer:**

* Abra el enlace en el mensaje para ver los detalles de la suspensión o apelarla
* Si tiene otra cuenta de Claude o una clave API que no se ve afectada por la suspensión, puede seguir trabajando mientras se resuelve la suspensión: ejecute `/login` con esa cuenta, o configure la clave con `ANTHROPIC_API_KEY`

<h3 id="anthropic-profile-login-expired">
  Inicio de sesión de perfil de Anthropic expirado
</h3>

Claude Code se está autenticando a través de un perfil de credencial de Anthropic cuya credencial de inicio de sesión guardada ha expirado, y el perfil no contiene ninguna credencial de actualización que Claude Code pueda usar para renovarla. Claude Code detiene cada solicitud localmente sin reintentar, porque un reintento leería la misma credencial expirada.

```text theme={null}
Anthropic profile login expired · Re-authenticate your Anthropic profile
Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile
```

Esto aparece solo cuando la credencial activa proviene de un perfil de credencial de Anthropic, uno que selecciona con la variable de entorno `ANTHROPIC_PROFILE`, que Claude Code descubre como el perfil activo en su directorio de configuración de Anthropic, o que Claude Code escribió cuando [inició sesión sin una clave API](/docs/es/authentication#sign-in-without-an-api-key). Las sesiones que se autentican con la opción claude.ai de `/login`, una clave API, un token de portador como `ANTHROPIC_AUTH_TOKEN`, o un proveedor de terceros nunca ven este mensaje.

En una máquina que [ofrece el inicio de sesión sin clave](/docs/es/authentication#sign-in-without-an-api-key), ejecute `/login`, elija la cuenta de Anthropic Console, e inicie sesión nuevamente para renovar un perfil que el inicio de sesión sin clave de Console o el CLI de Claude Platform `ant auth login` escribió. Claude Code reemplaza la credencial expirada en ese perfil. Para un perfil de federación u otro que creó una herramienta, `/login` no renueva la credencial. Qué forma ve depende de si seleccionó el perfil o Claude Code lo descubrió:

* Cuando establece `ANTHROPIC_PROFILE` explícitamente, el mensaje termina con `Re-authenticate your Anthropic profile`.
* Cuando Claude Code descubrió el perfil de su directorio de configuración, el mensaje ofrece `/login`, porque Claude Code da precedencia a un `/login` que funciona sobre el perfil descubierto y luego se autentica con su cuenta de claude.ai o Console en su lugar. Antes de v2.1.234, Claude Code mostró el formulario `Re-authenticate your Anthropic profile` en este caso también.

**Qué hacer:**

* Inicie sesión en el perfil nuevamente, luego reintente: en una máquina que [ofrece el inicio de sesión sin clave](/docs/es/authentication#sign-in-without-an-api-key), ejecute `/login` y elija la cuenta de Anthropic Console para un perfil que el inicio de sesión sin clave de Console o el CLI de Claude Platform `ant auth login` escribió; para otros perfiles, use la herramienta que los creó
* Si un administrador aprovisionó la credencial del perfil, pídales que emitan una nueva
* Ejecute `/status` para confirmar la fuente de credencial activa y el nombre del perfil
* Para dejar de usar el perfil, desconfigurar `ANTHROPIC_PROFILE` si lo estableció, luego autentíquese de otra manera, como `/login` o `ANTHROPIC_API_KEY`

<h3 id="oauth-scope-requirement">
  Requisito de alcance OAuth
</h3>

El token almacenado es anterior a un alcance de permiso que una característica más nueva necesita. Ve esto más a menudo de `/usage` y el indicador de uso de la línea de estado:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**Qué hacer:**

* Ejecute `/login` para obtener un token nuevo con los alcances actuales. No necesita cerrar sesión primero.

<h3 id="claude-ai-rejected-the-session-token">
  claude.ai rechazó el token de sesión
</h3>

Una solicitud de [conector de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) falló porque claude.ai rechazó el token de su inicio de sesión de Claude Code, generalmente un inicio de sesión que expiró y no se pudo renovar. El token rechazado es su inicio de sesión, no la autorización propia del conector en claude.ai, por lo que autorizar el conector nuevamente no lo resuelve. En `/mcp`, el conector se muestra como `connected · session token rejected` y su vista de detalle lee:

```text theme={null}
claude.ai rejected the session token. Run /login, then reconnect.
```

**Qué hacer:**

* Ejecute `/login` para iniciar sesión nuevamente
* Reconecte el conector desde `/mcp`, o ejecute `/mcp reconnect <server>`. Reconectar antes de iniciar sesión nuevamente deja el conector en el mismo estado. La opción **Reconnect** del panel `/mcp` informa `your claude.ai session token was rejected`; el formulario `/mcp reconnect <server>` escrito informa una reconexión exitosa incluso aunque el token aún sea rechazado.

Antes de v2.1.222, Claude Code marcaba el conector como necesitando autenticación en su lugar, lo que lo señalaba al flujo de autorización del conector incluso aunque completarlo no resolviera el estado.

<h3 id="mcp-server-needs-you-to-sign-in-again">
  El servidor MCP necesita que inicie sesión nuevamente
</h3>

Un [servidor MCP](/docs/es/mcp) remoto rechazó la credencial en una llamada de herramienta a mitad de sesión, generalmente porque un inicio de sesión o token expiró o porque el token carece de un permiso que la herramienta necesita. La llamada de herramienta falla, y `/mcp` marca el servidor como [necesitando autenticación](/docs/es/mcp#authenticate-with-remote-mcp-servers).

Para un servidor en el que inicia sesión desde Claude Code, incluyendo un conector de claude.ai, el inicio de sesión expiró o fue revocado:

```text theme={null}
MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)
```

Ejecute `/mcp`, seleccione el servidor, e inicie sesión nuevamente desde su menú.

Para un servidor configurado con un script [`headersHelper`](/docs/es/mcp#use-dynamic-headers-for-custom-authentication), Claude Code ya ha vuelto a ejecutar el helper e intentado la llamada una vez antes de mostrar esto:

```text theme={null}
MCP server "<name>" rejected the credential from its headersHelper (check the helper and run /mcp to reconnect, or to authenticate if the server also uses OAuth)
```

Verifique que el helper devuelva una credencial que el servidor acepte, luego reconecte desde `/mcp`, que ejecuta el helper nuevamente.

Para un servidor con un encabezado `Authorization` estático en su configuración:

```text theme={null}
MCP server "<name>" rejected the Authorization header in its config (update it, then run /mcp to reconnect)
```

Actualice el valor del encabezado donde el servidor está configurado, luego reconecte desde `/mcp`.

Antes de v2.1.273, los tres casos mostraban `MCP server "<name>" requires re-authorization (token expired)`.

Un servidor también puede rechazar una llamada de herramienta con HTTP 403 `insufficient_scope` para pedirle que autorice un alcance, a veces uno que su token ya enumera. El mensaje nombra ese alcance:

```text theme={null}
MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate
```

Ejecute `/mcp`, seleccione el servidor, y autentíquese nuevamente desde su menú.

Cuando la configuración del servidor no establece ni [`oauth.scopes`](/docs/es/mcp#restrict-oauth-scopes) ni [`authServerMetadataUrl`](/docs/es/mcp#override-oauth-metadata-discovery), Claude Code solicita el alcance que el servidor nombró. Con cualquiera de las dos configuraciones, Claude Code solicita los alcances de esa configuración en su lugar. Si fijó `oauth.scopes`, agregue el alcance faltante a esa lista antes de autenticarse nuevamente.

Antes de v2.1.274, este caso mostraba el mensaje `needs you to sign in again`, y antes de v2.1.273 mostraba `requires re-authorization (token expired)` como los otros casos.

<h3 id="issuer-mismatch-in-authorization-response">
  Desajuste del emisor en la respuesta de autorización
</h3>

Durante un [inicio de sesión OAuth de MCP](/docs/es/mcp#authenticate-with-remote-mcp-servers), el servidor de autorización redirigió de vuelta a Claude Code con un parámetro `iss` que no nombra al emisor que Claude Code esperaba de los metadatos OAuth del servidor. Un emisor incorrecto en este paso es cómo se ve un ataque de mezcla de servidores de autorización, por lo que Claude Code falla el inicio de sesión en lugar de intercambiar el código de autorización. Claude Code muestra el error en el menú del servidor `/mcp` después del inicio de sesión del navegador:

```text theme={null}
Issuer mismatch in authorization response (RFC 9207): expected "https://auth.example.com", received "https://other.example.com"
```

`expected` es el emisor de los metadatos OAuth del servidor, y `received` es el valor `iss` que el redireccionamiento llevaba. Un inicio de sesión cuyo redireccionamiento no lleva ningún parámetro `iss` pasa la verificación, a menos que los metadatos del servidor establezcan `authorization_response_iss_parameter_supported`, en cuyo caso Claude Code falla el inicio de sesión.

**Qué hacer:**

* Intente el inicio de sesión nuevamente desde `/mcp`
* Si el error se repite, infórmelo al operador del servidor. La solución es del lado del servidor: el servidor de autorización debe devolver el mismo emisor en el parámetro `iss` que anuncia en sus metadatos
* Para conectarse mientras se corrige el servidor, inicie Claude Code con [`MCP_SDK_GENERATION=v1`](/docs/es/env-vars), cuyo [tiempo de ejecución](/docs/es/mcp#mcp-client-runtimes) no ejecuta esta verificación. Esto elimina una protección contra ataques de mezcla, así que prefiera la solución del lado del servidor

Antes de v2.1.232, Claude Code usaba el tiempo de ejecución v2 solo en un lanzamiento gradual o cuando establecía `MCP_SDK_GENERATION=v2`.

<h3 id="aws-credentials-expired-or-invalid">
  Credenciales de AWS expiradas o inválidas
</h3>

Su token de sesión de AWS expiró o fue rechazado. Este mensaje aparece en un 401 de [Claude Platform en AWS](/docs/es/claude-platform-on-aws) o el [punto final de Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint), que es cómo esos proveedores informan un token de seguridad expirado.

La sugerencia de acción en el medio varía con su configuración. La parte estable es el `AWS credentials expired or invalid` inicial:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Antes de v2.1.273, este mensaje aparecía solo cuando `awsAuthRefresh` estaba configurado.

**Qué hacer:**

* Si la sugerencia dice que las credenciales son administradas por este entorno, la aplicación que lanzó Claude Code posee la credencial y los otros pasos aquí no se aplican: reintente, o contacte a su administrador
* Si [`awsAuthRefresh`](/docs/es/amazon-bedrock#advanced-credential-configuration) está configurado, ejecute el comando nombrado en el mensaje, como `aws sso login --profile myprofile`, en otra terminal y complete el inicio de sesión del navegador, luego reintente. De lo contrario, actualice la credencial de AWS que usa usted mismo: su inicio de sesión SSO, claves de acceso, clave API, o token de proxy
* Con `awsAuthRefresh` configurado en una sesión interactiva, puede ejecutar `/login`, elegir **3rd-party platform**, luego seleccionar **Claude Platform on AWS · refresh credentials** en **Using 3rd-party platforms** para ejecutar el mismo comando sin reiniciar Claude Code. Consulte [Configurar credenciales de AWS](/docs/es/claude-platform-on-aws#1-configure-aws-credentials)
* Si el error se repite después de que el comando de actualización tenga éxito, confirme que la identidad es válida fuera de Claude Code con `aws sts get-caller-identity` en el mismo shell y perfil

<h3 id="aws-authentication-failed">
  Falló la autenticación de AWS
</h3>

Su proveedor de AWS devolvió un 403, o [Amazon Bedrock](/docs/es/amazon-bedrock) devolvió un 401.

Amazon Bedrock informa un token de seguridad expirado como un 403, pero un 403 también es cómo informa una denegación de autorización, como un `AccessDeniedException` de un permiso de IAM faltante. Claude Code no puede distinguir esas dos causas.

Un 401 de Amazon Bedrock también llega aquí en lugar de bajo [Credenciales de AWS expiradas o inválidas](#aws-credentials-expired-or-invalid), porque Amazon Bedrock no informa un token expirado como un 401. Un 401 de ese punto final típicamente proviene de algo más en la ruta de solicitud, como un proxy corporativo.

Una actualización de credencial corrige un token expirado y no puede corregir las otras causas, por lo que el mensaje ofrece ambas:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

La sugerencia de acción en el medio varía con su configuración. La parte estable es el `AWS authentication failed` inicial.

Cuando el 403 es la respuesta de Amazon Bedrock de que no tiene acceso al modelo con el ID de modelo especificado, la sugerencia en su lugar le dice que habilite el modelo para su cuenta y región en la consola de Amazon Bedrock.

Antes de v2.1.273, este mensaje aparecía solo cuando `awsAuthRefresh` estaba configurado.

**Qué hacer:**

* Si la sugerencia dice que las credenciales son administradas por este entorno, la aplicación que lanzó Claude Code posee la credencial y los otros pasos aquí no se aplican: reintente, o contacte a su administrador
* Actualice sus credenciales de AWS en caso de que una credencial expirada sea la causa: ejecute el comando [`awsAuthRefresh`](/docs/es/amazon-bedrock#advanced-credential-configuration) nombrado en el mensaje cuando uno esté configurado, o actualice su inicio de sesión SSO, claves de acceso, clave API, o token de proxy usted mismo
* Si sus credenciales son actuales, confirme que los permisos de IAM en [Configuración de IAM](/docs/es/amazon-bedrock#iam-configuration) están adjuntos a la identidad que está usando y que el modelo seleccionado está habilitado para su cuenta y región
* Ejecute `aws sts get-caller-identity` para confirmar qué identidad usan sus solicitudes; un `AWS_PROFILE` obsoleto o perfil predeterminado es una causa común de una desajuste de permisos

<h3 id="google-cloud-credentials-expired-or-invalid">
  Credenciales de Google Cloud expiradas o inválidas
</h3>

Sus credenciales de Google Cloud para [Plataforma de Agentes de Google Cloud](/docs/es/google-vertex-ai) expiraron o fueron rechazadas: la solicitud devolvió un 401, que es cómo la Plataforma de Agentes informa la expiración de credenciales.

La sugerencia de acción en el medio varía con su configuración. La parte estable es el `Google Cloud credentials expired or invalid` inicial:

```text theme={null}
Google Cloud credentials expired or invalid · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · API Error: 401 ...
```

**Qué hacer:**

* Si la sugerencia dice que las credenciales son administradas por este entorno, la aplicación que lanzó Claude Code posee la credencial y los otros pasos aquí no se aplican: reintente, o contacte a su administrador
* Si se autentica con credenciales predeterminadas de aplicación, ejecute el comando [`gcpAuthRefresh`](/docs/es/google-vertex-ai#advanced-credential-configuration) nombrado en el mensaje, o `gcloud auth application-default login`, y complete el inicio de sesión, luego reintente
* Si enruta a través de una [puerta de enlace LLM](/docs/es/llm-gateway) con `CLAUDE_CODE_SKIP_VERTEX_AUTH` configurado, actualice el token de puerta de enlace en `ANTHROPIC_AUTH_TOKEN` o `ANTHROPIC_CUSTOM_HEADERS`, luego reintente
* Si se autentica con un archivo de clave de cuenta de servicio, confirme que `GOOGLE_APPLICATION_CREDENTIALS` apunta a una clave válida. Consulte [Configurar credenciales de GCP](/docs/es/google-vertex-ai#3-configure-gcp-credentials)
* Si el error se repite después de una actualización, confirme que la identidad funciona fuera de Claude Code con `gcloud auth application-default print-access-token` en el mismo shell

Antes de v2.1.273, un 401 de la Plataforma de Agentes mostraba el mensaje genérico `Please run /login` o `Failed to authenticate` en su lugar, que no puede actualizar credenciales de Google Cloud.

<h3 id="google-cloud-authentication-failed">
  Falló la autenticación de Google Cloud
</h3>

[Plataforma de Agentes de Google Cloud](/docs/es/google-vertex-ai) devolvió un 403, que usa para denegaciones de autorización en lugar de credenciales expiradas. Generalmente, la identidad con la que se autentica carece de un permiso de IAM, o el modelo no está habilitado para su proyecto.

La sugerencia de acción en el medio varía con su configuración. La parte estable es el `Google Cloud authentication failed` inicial:

```text theme={null}
Google Cloud authentication failed · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · if credentials are current, check GCP IAM permissions and Vertex AI model access · API Error: 403 ...
```

**Qué hacer:**

* Si la sugerencia dice que las credenciales son administradas por este entorno, la aplicación que lanzó Claude Code posee la credencial y los otros pasos aquí no se aplican: reintente, o contacte a su administrador
* Confirme que los roles en [Configuración de IAM](/docs/es/google-vertex-ai#iam-configuration) se otorgan a la identidad con la que se autentica
* Confirme que el modelo está habilitado para su proyecto. Consulte [Solicitar acceso al modelo](/docs/es/google-vertex-ai#2-request-model-access)

Antes de v2.1.273, un 403 de la Plataforma de Agentes mostraba el mensaje genérico `Please run /login` o `Failed to authenticate` en su lugar, que no puede actualizar credenciales de Google Cloud.

<h3 id="microsoft-foundry-authentication-failed">
  Falló la autenticación de Microsoft Foundry
</h3>

[Microsoft Foundry](/docs/es/microsoft-foundry) devolvió un 401 o 403: la credencial de Azure en la solicitud fue rechazada, o la identidad detrás de ella no tiene acceso al recurso de Foundry. `/login` no puede acuñar credenciales de Azure. La sugerencia de acción en el medio varía con su configuración. La parte estable es el `Microsoft Foundry authentication failed` inicial:

```text theme={null}
Microsoft Foundry authentication failed · refresh your Foundry credential (ANTHROPIC_FOUNDRY_AUTH_TOKEN, ANTHROPIC_FOUNDRY_API_KEY, Azure sign-in for Entra, or your proxy token) and retry · if credentials are current, check access to the Foundry resource · API Error: 401 ...
```

**Qué hacer:**

* Si la sugerencia dice que las credenciales son administradas por este entorno, la aplicación que lanzó Claude Code posee la credencial y los otros pasos aquí no se aplican: reintente, o contacte a su administrador
* Actualice la credencial que configuró en [Configurar credenciales de Azure](/docs/es/microsoft-foundry#2-configure-azure-credentials): rote `ANTHROPIC_FOUNDRY_API_KEY`, acuñe un nuevo `ANTHROPIC_FOUNDRY_AUTH_TOKEN`, o ejecute `az login` para que la cadena de credenciales predeterminada de Microsoft Entra pueda iniciar sesión nuevamente
* Si la credencial es actual, confirme que la identidad tiene acceso al recurso de Foundry. Consulte [Configuración de RBAC de Azure](/docs/es/microsoft-foundry#azure-rbac-configuration)

Antes de v2.1.273, un 401 o 403 de Microsoft Foundry mostraba el mensaje genérico `Please run /login` o `Failed to authenticate` en su lugar, que no puede actualizar credenciales de Azure.

<h3 id="could-not-load-aws-or-google-cloud-credentials">
  No se pudieron cargar las credenciales de AWS o Google Cloud
</h3>

Claude Code no pudo obtener credenciales utilizables de la cadena de proveedores de credenciales de AWS o de sus credenciales de aplicación predeterminada de Google en la máquina en la que se ejecuta, por lo que ninguna solicitud llegó a su proveedor en la nube. Claude Code borra sus credenciales en caché y reintenta dos veces antes de mostrar este mensaje. El detalle después del `·` nombra la causa específica, como una sesión SSO expirada, credenciales de aplicación predeterminada faltantes informadas como `Could not load the default credentials`, o un inicio de sesión revocado informado como `invalid_grant`:

```text theme={null}
API Error: Could not load AWS credentials · Could not load credentials from any providers. Check or refresh your AWS credentials and try again.
API Error: Could not load Google Cloud credentials · invalid_grant. Check or refresh your Google Cloud credentials and try again.
```

En [modo no interactivo](/docs/es/headless) con `-p` y en [Agent SDK](/docs/es/agent-sdk/overview), el código de error estructurado es `cloud_credential_error`. Antes de v2.1.267, el mensaje mostraba solo el texto de detalle después de `API Error:`, y el código estructurado era `server_error` o `unknown`.

**Qué hacer:**

* Ejecute el comando de inicio de sesión de su proveedor, como `aws sso login --profile myprofile` o `gcloud auth application-default login`, luego reintente. [Las credenciales de Bedrock, Plataforma de Agentes o Foundry no se cargan](/docs/es/troubleshoot-install#bedrock-agent-platform-or-foundry-credentials-not-loading) muestra cómo confirmar las credenciales fuera de Claude Code
* Si el detalle lee `AWS default-chain credential resolve timed out`, la cadena se colgó en lugar de fallar, así que siga [Tiempo de espera de resolución de credencial de cadena predeterminada de AWS](#aws-default-chain-credential-resolve-timed-out) en su lugar

<h3 id="aws-default-chain-credential-resolve-timed-out">
  Tiempo de espera de resolución de credencial de cadena predeterminada de AWS
</h3>

La cadena de proveedores de credenciales predeterminada de AWS no produjo credenciales dentro de 60 segundos, por lo que Claude Code detuvo la resolución y falló la solicitud. Este tiempo de espera es una causa de [No se pudieron cargar las credenciales de AWS o Google Cloud](#could-not-load-aws-or-google-cloud-credentials). El fallo es resolución de credencial local: la solicitud nunca llegó a [Amazon Bedrock](/docs/es/amazon-bedrock), [Claude Platform en AWS](/docs/es/claude-platform-on-aws), o el [punto final de Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint). Claude Code borra su [caché de credenciales](/docs/es/amazon-bedrock#credential-caching-and-resolution-timeout) y reintenta antes de que este error aparezca, por lo que en el momento en que lo ve la cadena se ha estancado en intentos repetidos.

```text theme={null}
API Error: Could not load AWS credentials · AWS default-chain credential resolve timed out. Check or refresh your AWS credentials and try again.
```

Las causas comunes son un comando `credential_process` en su perfil de AWS que espera entrada que no puede recibir, y un contenedor o VM cuyo servicio de metadatos de instancia (IMDS) nunca responde a la prueba de la cadena.

Antes de v2.1.267, el mensaje leía `API Error: AWS default-chain credential resolve timed out`.
Antes de v2.1.207, una cadena estancada dejaba la solicitud esperando indefinidamente en lugar de fallar.

**Qué hacer:**

* Ejecute `aws sts get-caller-identity` en el mismo shell con el mismo `AWS_PROFILE`. Si también se cuelga, corrija el perfil; un comando `credential_process` que solicita interactivamente es una causa común.
* Complete el paso de inicio de sesión antes de iniciar Claude Code, por ejemplo `aws sso login --profile myprofile`, para que la cadena se resuelva desde el caché SSO local en lugar de esperar un flujo del navegador
* Si su cadena ejecuta un inicio de sesión interactivo que legítimamente necesita más de 60 segundos, como SSO con MFA a través de un contenedor como `aws-vault`, aumente el límite en milisegundos con [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/es/env-vars)

<h3 id="bedrock-setup-verification-timed-out-waiting-for-aws">
  Tiempo de espera de verificación de configuración de Bedrock esperando AWS
</h3>

Una llamada a AWS durante el [asistente de configuración de Bedrock](/docs/es/amazon-bedrock#sign-in-with-bedrock), como la búsqueda de credenciales o la verificación de identidad, no se completó dentro del límite de 60 segundos. El asistente deja de esperar y falla el paso de verificación:

```text theme={null}
Timed out after 60s waiting for AWS. Check your network and proxy settings; if a credential helper needs longer to prompt you, raise CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS.
```

El número refleja su límite: 60 segundos por defecto, o el valor que estableció en [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/es/env-vars).

Las causas comunes son una red o proxy que detiene solicitudes a AWS, incluyendo la actualización del token SSO, y un asistente de credenciales aún esperando entrada que no puede ver. Aumente el límite solo cuando el asistente legítimamente necesite más tiempo.

Una única solicitud estancada a AWS también puede fallar en su propio tiempo de espera por solicitud, que muestra un mensaje más corto en el mismo paso:

```text theme={null}
A request to AWS timed out. Check your network and proxy settings, then try again.
```

Cuando los mismos tiempos de espera ocurren en el paso de fijación de modelo, el asistente marca un modelo como `unreachable` en lugar de mostrar cualquiera de los mensajes.

**Qué hacer:**

* Ejecute `aws sts get-caller-identity` en el mismo shell. Si también se cuelga, el estancamiento está fuera de Claude Code, en su red, su proxy, o el asistente de credenciales en su perfil de AWS; corrija eso primero.
* Complete cualquier inicio de sesión interactivo antes de abrir el asistente, por ejemplo `aws sso login --profile myprofile`
* Si un asistente de credenciales en su perfil de AWS legítimamente necesita más de 60 segundos para solicitarle, aumente el límite en milisegundos con [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/es/env-vars)

<h3 id="cloud-gateway-session-expired">
  Sesión de puerta de enlace en la nube expirada
</h3>

Inició sesión a través de una [puerta de enlace de aplicaciones de Claude](/docs/es/claude-apps-gateway), y la sesión de puerta de enlace guardada en esta máquina ha expirado y no se pudo renovar, o la puerta de enlace ya no la acepta, por ejemplo después de que el [secreto JWT de la puerta de enlace se reemplaza](/docs/es/claude-apps-gateway-deploy#jwt-secret-rotation). Si ve esta línea cuando inicia `claude` interactivamente, la sesión se ha abierto sin iniciar sesión en la puerta de enlace:

```text theme={null}
Cloud gateway session expired — run /login to reconnect.
```

La misma línea puede aparecer a mitad de sesión cuando la credencial de puerta de enlace expira y Claude Code no puede renovarla.

En una ejecución [no interactiva](/docs/es/headless), una sesión en segundo plano u otra desatendida, o un subcomando `claude` que no sea `claude auth`, Claude Code sale con este mensaje en su lugar cuando la puerta de enlace ya no acepta la sesión:

```text theme={null}
Cloud gateway <url> no longer accepts this session. Start `claude` and sign in again with /login.
```

**Qué hacer:**

* Ejecute `/login` en la sesión y complete el inicio de sesión del navegador
* Para un lanzamiento no interactivo, inicie `claude` en el mismo entorno, ejecute `/login`, luego reejecutar su comando

<h3 id="sign-in-timed-out-while-waiting-for-you-to-continue">
  Inicio de sesión agotado mientras esperaba que continúe
</h3>

Durante un inicio de sesión de [puerta de enlace de aplicaciones de Claude](/docs/es/claude-apps-gateway), la puerta de enlace nombró la cuenta que inició sesión, y Claude Code le pidió que la confirmara antes de guardar la credencial. Dejó la confirmación abierta más allá de la expiración del propio inicio de sesión, y la puerta de enlace no emitió ningún token de actualización que pudiera renovarlo, por lo que Claude Code no almacenó nada cuando continuó:

```text theme={null}
Sign-in timed out while waiting for you to continue. Try again.
```

**Qué hacer:**

* Ejecute `/login` nuevamente y confirme la cuenta antes de que el inicio de sesión expire

<h3 id="gateway-refused-the-request">
  La puerta de enlace rechazó la solicitud
</h3>

Está conectado a través de una [puerta de enlace de aplicaciones de Claude](/docs/es/claude-apps-gateway), y una solicitud devolvió un 403: la puerta de enlace, o la ascendente detrás de ella, la rechazó. Iniciar sesión nuevamente no cambia un rechazo, por lo que el mensaje señala a su administrador de puerta de enlace:

```text theme={null}
Gateway refused the request · signing in again won't change this — check with your gateway administrator · API Error: 403 ...
```

**Qué hacer:**

* Pida a su administrador de puerta de enlace que busque la solicitud. La cola `API Error:` lleva el rechazo que la puerta de enlace devolvió
* Para administradores: una [regla de control de acceso](/docs/es/claude-apps-gateway-config#http-tuning) en la puerta de enlace devuelve un 403 que el [registro de auditoría](/docs/es/claude-apps-gateway-deploy#logs) registra con su razón, y una denegación de autorización ascendente pasa a través de por [Mensajes de error ascendentes](/docs/es/claude-apps-gateway-config#upstream-error-messages)

Antes de v2.1.273, un 403 en una sesión de puerta de enlace mostraba el mensaje genérico `Please run /login` o `Failed to authenticate` en su lugar, e iniciar sesión nuevamente no borraba el rechazo.

<h2 id="network-and-connection-errors">
  Errores de red y conexión
</h2>

La mayoría de estos errores significan que una solicitud de red desde Claude Code no llegó a su destino, o algo entre Claude Code y la API alteró la respuesta en el camino de regreso; cuando una entrada también tiene una causa local, como una escritura de archivo fallida, su cuerpo lo indica. Generalmente se originan en su red local, proxy o firewall, o en la política de red del entorno en la nube.

<h3 id="unable-to-connect-to-api">
  No se puede conectar a la API
</h3>

La conexión TCP a la API falló o nunca se completó. Para los códigos de error de conexión comunes, el nombre del mensaje indica el tipo de fallo y mantiene el código entre paréntesis:

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

Un código que Claude Code no reconoce aparece como `Unable to connect to API` seguido del código entre paréntesis. Algunos de estos mensajes pueden mostrar más de un código: `Connection refused` puede mostrar `ConnectionRefused` o `ECONNREFUSED`, por ejemplo, y `Can't reach the API server` puede mostrar `ENOTFOUND` o `FailedToOpenSocket`.

Antes de v2.1.227, cada uno de estos mensajes codificados leía `Unable to connect to API` seguido del código, por ejemplo `Unable to connect to API (ECONNREFUSED)`.

Las causas comunes incluyen no tener acceso a internet, una VPN que bloquea `api.anthropic.com`, o un proxy corporativo requerido que no está configurado.

**Qué hacer:**

* Confirme que puede alcanzar el host de la API desde el mismo shell ejecutando `curl -I https://api.anthropic.com`. En Windows PowerShell use `curl.exe -I https://api.anthropic.com` para que no se use el alias `Invoke-WebRequest` integrado.
* Si está detrás de un proxy corporativo, establezca `HTTPS_PROXY` antes de lanzar Claude Code y consulte [Configuración de red](/docs/es/network-config)
* Si enruta a través de una puerta de enlace LLM o relé, establezca [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) en su dirección. Consulte [Conectar Claude Code a una puerta de enlace LLM](/docs/es/llm-gateway-connect) para la configuración.
* Asegúrese de que su firewall permite los hosts enumerados en [Requisitos de acceso a la red](/docs/es/network-config#network-access-requirements)
* Los fallos intermitentes se [reintentan automáticamente](#automatic-retries); los fallos persistentes apuntan a un problema de red local

Si `curl` tiene éxito pero Claude Code aún falla, la causa suele ser algo entre el tiempo de ejecución y la red en lugar de la red misma:

* En Linux y WSL, verifique `/etc/resolv.conf` para un servidor de nombres inaccesible. WSL en particular puede heredar un resolutor roto del host.
* En macOS, un cliente VPN que fue desconectado o desinstalado puede dejar una interfaz de túnel o una regla de enrutamiento. Verifique `ifconfig` para interfaces `utun` obsoletas y elimine la extensión de red de la VPN en Configuración del Sistema.
* Docker Desktop y tiempos de ejecución de contenedores similares pueden interceptar el tráfico saliente. Ciérrelos y reintente para descartar esto.

<h3 id="unable-to-connect-to-anthropic-services">
  No se puede conectar a los servicios de Anthropic
</h3>

Durante la configuración de primera ejecución, Claude Code verifica que pueda alcanzar `api.anthropic.com` y `platform.claude.com` antes de mostrar el paso de inicio de sesión. Cuando alguna verificación falla, Claude Code imprime la razón y sale.

```text theme={null}
Unable to connect to Anthropic services
Failed to connect to api.anthropic.com: ECONNREFUSED
Connection to api.anthropic.com timed out after 10 seconds
A proxy is configured via HTTPS_PROXY. Check that it allows connections to the host above.
```

Claude Code envía la verificación a través de la misma [configuración de proxy](/docs/es/network-config) que las solicitudes de API y da a cada sonda 10 segundos. Cuando la sonda fallida pasó a través de un proxy, el mensaje nombra la variable de entorno que lo configuró, como `HTTPS_PROXY`. Antes de v2.1.222, la verificación usaba un transporte de proxy diferente sin tiempo de espera: detrás de una URL de proxy con el esquema `https://`, podría estancarse en `Checking connectivity...` indefinidamente y luego fallar aunque las solicitudes de API a través del mismo proxy tengan éxito.

Claude Code omite esta verificación cuando un [archivo de configuración administrado, política MDM o asistente de política](/docs/es/managed-settings) establece [`forceLoginMethod`](/docs/es/settings-reference#forceloginmethod) en `"gateway"`, o establece [`forceLoginGatewayUrl`](/docs/es/settings-reference#forcelogingatewayurl) sin `forceLoginMethod`. Con cualquiera de estas configuraciones, Claude Code abre el paso de inicio de sesión en la pantalla **Cloud gateway** en lugar de un método de inicio de sesión de Anthropic. Claude Code también omite la verificación cuando existe una fuente de configuración administrada en la máquina pero no se puede leer, ya que esa fuente puede contener la configuración de la puerta de enlace. Antes de v2.1.247, Claude Code ejecutaba la verificación bajo esta configuración también, y salía con este error cuando los puntos finales de Anthropic eran inaccesibles.

**Qué hacer:**

* Si el mensaje nombra una variable de proxy, verifique que su valor apunte al proxy correcto y pida a su equipo de red que permita conexiones HTTPS a través de él al host en el mensaje. Consulte [Configuración de red](/docs/es/network-config).
* Trabaje a través de las verificaciones en [No se puede conectar a la API](#unable-to-connect-to-api). La prueba `curl` y la orientación de firewall allí se aplican a esta verificación también.
* Si su organización inicia sesión a través de una [puerta de enlace en la nube](/docs/es/claude-apps-gateway) y este error aparece en la primera ejecución, actualice a Claude Code v2.1.247 o posterior.
* Si su red está abierta y el fallo persiste, Claude Code puede no estar [disponible en su país](https://www.anthropic.com/supported-countries)

<h3 id="socket-is-closed">
  Socket is closed
</h3>

`Socket is closed` significa que la conexión que transportaba una respuesta de transmisión se cerró mientras la respuesta aún llegaba. La causa más común es un proxy corporativo en Windows que elimina un túnel establecido a mitad de la respuesta.

Dependiendo de cuán lejos haya progresado la respuesta, Claude Code reintenta la solicitud, mantiene lo que Claude produjo, o termina el turno. Consulte [Reintentos automáticos](#automatic-retries).

Antes de v2.1.214, Claude Code no reintentaba este fallo, y el turno se detenía con un error que contenía `Socket is closed`.

**Qué hacer:**

* Si ve este error, actualice a v2.1.214 o posterior con `claude update`, luego envíe su mensaje nuevamente
* Si los turnos siguen fallando detrás del mismo proxy después de actualizar, trabaje a través de [No se puede conectar a la API](#unable-to-connect-to-api) y verifique la configuración del proxy en [Configuración de red](/docs/es/network-config)

<h3 id="api-returned-an-empty-or-malformed-response">
  La API devolvió una respuesta vacía o malformada
</h3>

Claude Code muestra este error cuando su reintento sin transmisión de una solicitud de transmisión fallida obtiene un estado de éxito HTTP pero el cuerpo no es un mensaje de API de Claude: comúnmente una página de error HTML o de inicio de sesión, un cuerpo vacío, o JSON en otro formato. Un proxy, puerta de enlace o página de inicio de sesión de red respondiendo en lugar de la API es la fuente habitual. Claude Code no reintenta la solicitud, y el turno termina con este error.

```text theme={null}
API returned an empty or malformed response (HTTP 200) — check for a proxy or gateway intercepting the request.
```

Después de esa apertura, el mensaje informa lo que regresó y qué solicitud falló:

* Una cláusula `Response:` con el tipo de contenido, el tipo de cuerpo, como `body is an HTML page` o `empty body`, su tamaño en bytes, y si la respuesta llevaba un id de solicitud de Anthropic. Cuando la respuesta nombra un servidor reconocible, como `nginx` o `cloudflare`, o lleva encabezados intermediarios, como `cf-ray` o `via`, la cláusula también los enumera.
* Una oración que nombra el id de la solicitud de transmisión fallida y el fallo que desencadenó el reintento. Cuando una transmisión se había abierto antes del fallo, también informa cuántos eventos de transmisión llegaron y, si alguno lo hizo, cuánto tiempo la transmisión había estado en silencio cuando se realizó el intento.

Antes de v2.1.234, el mensaje terminaba después de `intercepting the request`.

Antes de v2.1.271, una respuesta que llevaba un mensaje de API válido bajo un tipo de contenido que no es JSON como `text/plain` también terminaba el turno con este error. Algunas puertas de enlace LLM usan ese tipo de contenido para la respuesta sin transmisión.

**Qué hacer:**

* Lea la cláusula `Response:` para ver qué sistema respondió. Un cuerpo HTML, sin id de solicitud de Anthropic, o un servidor nombrado como `nginx` o `cloudflare` significa que algo entre Claude Code y la API respondió en su lugar
* Si enruta a través de una [puerta de enlace LLM](/docs/es/llm-gateway-connect#troubleshoot-gateway-errors), pruebe la ruta con una solicitud directa y corrija el salto que devuelve la respuesta que no es de API
* En una red con una página de inicio de sesión, como Wi-Fi de invitados, complete el inicio de sesión en un navegador, luego reintente
* Si solo la ruta sin transmisión a través de su puerta de enlace está rota, establezca [`CLAUDE_CODE_DISABLE_NONSTREAMING_FALLBACK=1`](/docs/es/env-vars#variables) para que una solicitud que falla a mitad de la transmisión vaya a la ruta de reintento normal en lugar de este respaldo, excepto cuando el punto final de transmisión en sí devuelve `404`, donde Claude Code aún recurre al respaldo

<h3 id="streaming-response-ended-before-any-complete-data-was-received">
  La respuesta de transmisión terminó antes de que se recibiera algún dato completo
</h3>

Una respuesta de transmisión de su proveedor de modelo se completó sin entregar ningún dato utilizable, por lo que Claude Code reenviló la solicitud sin transmisión para terminar el turno. Claude Code muestra la advertencia una vez por sesión, solo en sesiones interactivas. Antes de v2.1.239, Claude Code reintentaba silenciosamente sin transmisión.

```text theme={null}
Streaming response ended before any complete data was received. Retrying without streaming. If this keeps happening, check any proxy or gateway between Claude Code and your model provider.
```

Claude Code envía cada solicitud afectada dos veces: el intento de transmisión vacío y el reintento. La causa habitual es un proxy o puerta de enlace que consume o transforma el cuerpo de la respuesta de transmisión en el camino de regreso.

**Qué hacer:**

* Configure cualquier proxy o puerta de enlace entre Claude Code y su proveedor de modelo para pasar los cuerpos de respuesta de transmisión y sus encabezados sin modificar
* En [Amazon Bedrock](/docs/es/amazon-bedrock), consulte [Errores de transmisión detrás de una puerta de enlace o proxy](/docs/es/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy) para los requisitos de encabezado y cuerpo

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  La respuesta de transmisión de Bedrock tiene un content-type inesperado
</h3>

Una puerta de enlace o proxy entre Claude Code y [Amazon Bedrock](/docs/es/amazon-bedrock) está transformando el cuerpo de la respuesta de transmisión o su encabezado `Content-Type`. Amazon Bedrock transmite respuestas como `application/vnd.amazon.eventstream`. En lugar de decodificar un cuerpo que no puede leer, Claude Code rechaza una respuesta de transmisión exitosa que informa un content-type diferente. Claude Code no reintenta la solicitud.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Antes de v2.1.208, la misma configuración incorrecta se presentaba como `API Error: Truncated event message received` después de que todo el cuerpo había sido almacenado en búfer.

**Qué hacer:**

* Configure la puerta de enlace para pasar el cuerpo de respuesta `InvokeModelWithResponseStream` y su encabezado `Content-Type` sin modificar. Un intermediario que reemite la transmisión como eventos enviados por el servidor es una causa común.
* Establecer [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/es/env-vars) oculta este error, pero Claude Code no decodifica un cuerpo binario bajo un encabezado reescrito, por lo que esas solicitudes recurren a una ruta más lenta sin transmisión. Consulte [Errores de transmisión detrás de una puerta de enlace o proxy](/docs/es/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  Errores de certificado SSL
</h3>

Un proxy o dispositivo de seguridad en su red está interceptando el tráfico TLS con su propio certificado, y Claude Code no lo confía.

```text theme={null}
Unable to connect to API: SSL certificate verification failed (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
Unable to connect to API: Self-signed certificate detected (SELF_SIGNED_CERT_IN_CHAIN). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
```

Antes de v2.1.273, ambos mensajes terminaban en `Check your proxy or corporate SSL certificates`, sin el código OpenSSL o la sugerencia `NODE_EXTRA_CA_CERTS`.

A partir de v2.1.199, un fallo de validación de certificado no se reintenta, por lo que este error aparece en el primer intento en lugar de después del [presupuesto de reintento](#automatic-retries) completo. Las versiones anteriores pasaban unos minutos reintentando antes de mostrarlo. Las condiciones TLS transitorias, como un tiempo de espera de protocolo de enlace, aún se reintentan.

Durante `/login` y la verificación de conectividad de inicio, el mismo fallo produce un mensaje diferente:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

En [Amazon Bedrock](/docs/es/amazon-bedrock), las solicitudes que Claude Code envía a AWS, como las llamadas de credencial de rol STS y SSO, descubrimiento de modelo, y las verificaciones del asistente de configuración, dependen de la misma configuración de certificado. Consulte [Errores de certificado detrás de un proxy que inspecciona TLS](/docs/es/amazon-bedrock#certificate-errors-behind-a-tls-inspecting-proxy).

**Qué hacer:**

* Exporte el paquete de CA de su organización y apunte Claude Code a él con `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Consulte [Configuración de red](/docs/es/network-config#custom-ca-certificates) para obtener instrucciones de configuración completas
* No establezca `NODE_TLS_REJECT_UNAUTHORIZED=0`, que desactiva completamente la validación de certificados

<h3 id="host-not-allowed-in-a-cloud-session">
  Host no permitido en una sesión en la nube
</h3>

Una solicitud HTTP saliente desde una sesión en la nube o rutina fue bloqueada por la política de red del entorno.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

También puede ver un certificado TLS que no coincide con el certificado real del destino. Las sesiones en la nube enrutan el tráfico saliente a través de un proxy que aplica la política de red, por lo que un certificado que no coincide significa que el proxy terminó la conexión, no el destino.

Este no es un problema de red del lado del cliente. Las sesiones en la nube y [rutinas](/docs/es/routines) se ejecutan dentro de una VM aislada cuyo tráfico saliente a través de la red de la sesión se filtra a la [lista de permitidos del entorno en la nube](/docs/es/cloud-environments); [las operaciones de GitHub](/docs/es/cloud-environments#github-proxy) y el tráfico del conector MCP usan canales separados, por lo que pueden seguir funcionando mientras otros hosts están bloqueados. El entorno **Default** usa acceso **Trusted**, que permite la [lista de permitidos predeterminada](/docs/es/cloud-environments#default-allowed-domains) de registros de paquetes, API de proveedores en la nube, registros de contenedores, y dominios de desarrollo comunes y bloquea otros dominios en esa ruta.

**Qué hacer:**

Estos pasos cambian uno de sus propios entornos. Un [entorno compartido por la organización](/docs/es/cloud-environments#organization-shared-environments) se abre como solo lectura en el selector, así que pida a un Propietario que cambie su acceso de red desde la página **Cloud environments** en [configuración de administración](https://claude.ai/admin-settings).

* Abra la rutina para editar, o inicie una sesión en la nube. Seleccione el icono de nube que muestra el nombre de su entorno, como **Default**, para abrir el selector. Pase el cursor sobre su entorno y haga clic en el icono de configuración.
* En el diálogo **Update cloud environment**, cambie **Network access** de **Trusted** a **Custom**, luego agregue el dominio bloqueado a **Allowed domains**. Ingrese un dominio por línea. Marque **Also include default list of common package managers** para mantener la [lista de permitidos predeterminada](/docs/es/cloud-environments#default-allowed-domains) junto con sus dominios personalizados. Seleccione **Full** en su lugar si desea acceso sin restricciones.
* Haga clic en **Save changes**. La siguiente ejecución usa la lista de permitidos actualizada.

Consulte [Network access](/docs/es/cloud-environments#network-access) para niveles de acceso y la lista de permitidos predeterminada. Las sesiones de CLI locales no se ven afectadas por esta política.

<h3 id="the-proxy-refused-the-connection">
  El proxy rechazó la conexión
</h3>

Ve este mensaje cuando Claude lee un [artefacto](/docs/es/artifacts) a través del proxy que estableció en `HTTPS_PROXY` o una [variable de proxy](/docs/es/network-config#environment-variables) relacionada. El contenido del artefacto proviene de `*.frame.claudeusercontent.com`, por lo que Claude Code primero envía al proxy una solicitud `CONNECT` pidiéndole que abra un túnel a ese host. Cuando el proxy rechaza, nada llega al host, y el mensaje lleva el estado HTTP del proxy:

```text theme={null}
artifact content fetch failed (proxy refused the connection: HTTP 407)
artifact content fetch failed (proxy refused the connection: HTTP 403)
the proxy refused the connection to the artifact's content host (HTTP 502)
```

El estado es la respuesta del proxy a `CONNECT`. El host nunca respondió, por lo que cada estado apunta a una corrección diferente:

* `HTTP 407`: el proxy requiere credenciales que no obtuvo. Póngalas en la URL del proxy, como muestra [Autenticación básica](/docs/es/network-config#basic-authentication).
* `HTTP 403`: el proxy rechaza hacer un túnel a `*.frame.claudeusercontent.com`. Pida a quien ejecute el proxy que permita ese host, que [Requisitos de acceso a la red](/docs/es/network-config#network-access-requirements) enumera.
* Cualquier otro estado, como `HTTP 502`: el proxy no abrió el túnel por su propia razón, como no poder alcanzar el host. Busque el estado en los registros del proxy.
* `unreadable reply` en lugar de un estado: lo que está en la dirección del proxy no respondió con una línea de estado HTTP. Verifique que la dirección sea un proxy HTTP.

**Qué hacer:**

* Verifique la dirección y las credenciales en la variable de proxy, como describe [Proxy configuration](/docs/es/network-config#proxy-configuration), luego ejecute `curl -x http://proxy.example.com:8080 -I https://api.anthropic.com` desde el shell en el que inicia Claude Code, usando su propia URL de proxy. En Windows PowerShell, ejecute `curl.exe`. Si esta sonda falla de la misma manera, corrija primero la configuración del proxy. Si tiene éxito, el rechazo es específico del host del artefacto.
* Si su red permite que Claude Code alcance el host del artefacto directamente, agregue `.frame.claudeusercontent.com` a [`NO_PROXY`](/docs/es/network-config#environment-variables). Mantenga la entrada estrecha: una entrada más amplia `.claudeusercontent.com` también omite el proxy para `bridge.claudeusercontent.com`, que las organizaciones con [lista de permitidos de IP](/docs/es/network-config#organization-ip-allowlists-and-proxy-egress) necesitan mantener en el proxy.

Antes de v2.1.238, Claude Code informaba un túnel rechazado como un error de red genérico.

<h3 id="the-cloud-environments-service-returned-an-empty-or-unexpected-response">
  El servicio de entornos en la nube devolvió una respuesta vacía o inesperada
</h3>

Claude Code solicita su lista de [entornos en la nube](/docs/es/cloud-environments) en varios puntos, como cuando crea una sesión en la nube desde la CLI o ejecuta [`/remote-env`](/docs/es/cloud-environments#select-an-environment-from-the-cli). Cuando no puede leer la respuesta del servidor, muestra uno de estos mensajes:

```text theme={null}
The cloud environments service returned an empty response (HTTP 200 with no body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 with a non-JSON body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 without a usable environments list). This is usually temporary — try again in a moment.
```

El servidor aceptó la solicitud pero respondió con un cuerpo que no es la lista de entornos: vacío, no JSON, o JSON sin la lista. Esto generalmente acompaña una interrupción del lado del servicio y se resuelve por sí solo. Dependiendo de la superficie que solicitó la lista, Claude Code puede agregar un prefijo, como `couldn't list environments:` en el diálogo `/remote-env`.

**Qué hacer:**

* Reintente la acción. Claude Code solicita la lista nuevamente cada vez
* Si el mensaje sigue apareciendo, verifique [status.claude.com](https://status.claude.com) para incidentes activos

Antes de v2.1.236, Claude Code mostraba un TypeError de JavaScript sin procesar en lugar de estos mensajes.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  No se pudo reconectar a su sesión de Remote Control
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

Reanudar con `claude --resume` o `claude --continue` se reconecta a la sesión de [Remote Control](/docs/es/remote-control) registrada en esa conversación. Este mensaje significa que la reconexión falló por una razón que puede ser temporal, como una interrupción de red o un error del servidor, por lo que Claude Code no puede confirmar si la sesión remota aún existe. Su sesión local sigue ejecutándose sin Remote Control.

**Qué hacer:**

* Ejecute `/remote-control` para reintentar la conexión
* Inicie una nueva sesión con `claude --remote-control` para crear una nueva sesión de Remote Control
* Para otros mensajes de inicio de Remote Control, consulte [Solucionar problemas de Remote Control](/docs/es/remote-control#troubleshooting)

Si el servidor informa en su lugar que la sesión anterior se ha ido, no ve este mensaje. Claude Code inicia una nueva sesión en su lugar o muestra [`Previous session is unavailable — run /remote-control to start a new one`](/docs/es/remote-control#previous-session-is-unavailable), dependiendo del [registro de reconexión de la conversación](/docs/es/remote-control#resume-outcomes). De v2.1.227 a v2.1.231, Claude Code mostró un mensaje que comienza con `Remote Control could not resume the previous session under the current login` en su lugar, y [las versiones anteriores se comportaron de manera diferente nuevamente](/docs/es/remote-control#reconnect-history).

<h3 id="sessions-ended-while-this-machine-was-offline">
  Las sesiones terminaron mientras esta máquina estaba sin conexión
</h3>

Claude Code muestra este mensaje en la terminal que ejecuta [`claude remote-control`](/docs/es/remote-control#start-a-remote-control-session) después de que su máquina estuvo sin conexión el tiempo suficiente para que el servidor limpiara el entorno de Remote Control que su máquina estaba sirviendo. Las sesiones en ese entorno terminaron, y no puede reanudarlas. El recuento es el número de sesiones que terminaron.

```text theme={null}
2 sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.
```

**Qué hacer:**

* Cuando Claude Code enumera worktrees mantenidos bajo este mensaje, recoja cualquier trabajo no confirmado de ellos
* Ejecute `claude remote-control` para iniciar un entorno nuevo

<h3 id="couldnt-share-the-transcript">
  No se pudo compartir la transcripción
</h3>

Después de que acepta compartir su transcripción de sesión desde un mensaje de encuesta, como la [encuesta de calidad de sesión](/docs/es/data-usage#session-quality-surveys), Claude Code la carga a Anthropic, o guarda un archivo local en su lugar en proveedores de terceros, en sesiones de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), y cuando no hay credenciales de Anthropic disponibles. Este mensaje significa que el intercambio no se completó.

```text theme={null}
Couldn't share the transcript.
```

La carga debe ajustarse a un límite de 8 MiB. En una sesión larga, Claude Code progresivamente elimina partes del intercambio, la configuración del modelo de la última solicitud primero, luego la conversación estructurada y las transcripciones de subagentes, y muestra este mensaje solo cuando no se puede enviar ninguna versión reducida o un error de red o servidor detiene la carga. Cuando Claude Code guarda un archivo local en su lugar, el mensaje significa que no pudo escribir el archivo.

**Qué hacer:**

* Ejecute `/feedback` para enviar la transcripción con una descripción de lo que sucedió. Consulte [Reportar un error](#report-an-error) si `/feedback` no está disponible en su entorno
* Si otras solicitudes también están fallando, verifique su conexión de red y consulte [No se puede conectar a la API](#unable-to-connect-to-api)

<h2 id="request-errors">
  Errores de solicitud
</h2>

Estos errores se relacionan con el contenido de su solicitud. La mayoría provienen de la API después de rechazarla; algunos son producidos localmente por Claude Code antes de que se envíe ninguna solicitud.

<h3 id="prompt-is-too-long">
  El prompt es demasiado largo
</h3>

La conversación más los archivos adjuntos exceden la ventana de contexto del modelo.

```text theme={null}
Prompt is too long
```

En una sesión interactiva, Claude Code muestra este error como:

```text theme={null}
Context limit reached · /compact or /clear to continue
```

La línea nombra solo `/clear` cuando [`DISABLE_COMPACT`](/docs/es/env-vars) está configurado. Las formas más largas del error, como la forma de compactación fallida a continuación, mantienen la redacción `Prompt is too long ·`. En la salida `-p` y la transcripción, el texto permanece como `Prompt is too long`.

Cuando desactivó la compactación automática en su [configuración de usuario](/docs/es/settings-reference#autocompactenabled), la línea también lo dice:

```text theme={null}
Context limit reached · /compact or /clear to continue · auto-compact is off · /config to turn it on
```

El botón **Auto-compact** en `/config` escribe `autoCompactEnabled` en la configuración del usuario. La sugerencia aparece solo cuando un cambio en `/config` tendría efecto. Por ejemplo, no aparece cuando [`DISABLE_AUTO_COMPACT`](/docs/es/env-vars) o [`DISABLE_COMPACT`](/docs/es/env-vars) desactivó la compactación automática. Tampoco aparece cuando un ámbito de mayor precedencia, como la configuración del proyecto o administrada, estableció `autoCompactEnabled` en `false`. Antes de v2.1.235, la línea no llevaba ninguna sugerencia de compactación automática.

Amazon Bedrock reporta esta condición como `Input is too long for requested model.`, que Claude Code maneja de la misma manera. Antes de v2.1.217, Claude Code no reconocía la redacción de Bedrock, por lo que la compactación automática nunca se activaba en ella y `/compact` fallaba con el mismo error.

Una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway-config#upstream-error-messages) reporta esta condición como `capability_rejected: prompt_too_long` cuando una nube ascendente rechaza la solicitud en la forma de error propia del proveedor. Claude Code trata el token igual que `Prompt is too long`. Antes de v2.1.228, Claude Code no reconocía el token, por lo que la compactación automática no se activaba en él.

Cuando la compactación automática se ejecutó en este turno y falló en un error subyacente, como un modelo no disponible o un fallo de autenticación, el mensaje nombra ese error después de un separador:

```text theme={null}
Prompt is too long · automatic compaction failed: <the underlying error>
```

Resuelva el error nombrado primero; `/compact` falla en el mismo error hasta que lo haga. Antes de v2.1.229, una compactación automática fallida mostraba `Prompt is too long` sin la causa.

Cuando la compactación automática se ejecuta en este error, normalmente resume sus intercambios más antiguos y mantiene los más nuevos. Como último recurso, Claude Code resume de manera diferente:

* Cuando no puede resumir ningún intercambio completo, Claude Code mantiene su mensaje más reciente palabra por palabra y resume todo lo anterior.
* En ese caso, cuando la conversación no termina con su mensaje, Claude Code resume toda la conversación en su lugar.

Claude Code omite esta recuperación cuando el contenido que llevaría adelante no contiene respuesta del modelo y menos de aproximadamente 1.000 tokens de su propio texto, como un reintento corto enviado después de un pegado de tamaño excesivo. Ejecute `/clear` para comenzar de nuevo. Antes de v2.1.269, la compactación fallaba siempre que no pudiera resumir un intercambio completo, por lo que una sesión en ese estado golpeaba este error de nuevo en cada turno.

Una conversación de un solo intercambio no tiene turnos anteriores para resumir. Cuando la compactación automática se habría ejecutado en uno, Claude Code omite el intento y explica qué llena la solicitud en su lugar. Cuando la API no reporta conteos de tokens en su error, el mensaje dice:

```text theme={null}
Prompt is too long · this conversation is a single exchange and cannot be compacted — the request size comes mostly from system prompt, tool definitions, or attachments.
```

Cuando la API reporta conteos de tokens en su error, Claude Code los compara con su propia estimación del tamaño de la conversación para determinar cuál es la mayor parte de la solicitud: el contenido propio de la conversación, o el prompt del sistema, definiciones de herramientas y contenido de adjuntos que Claude Code envía con ella. Cuando el contenido propio de la conversación es la mayor parte de la solicitud, el mensaje dice:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) and this conversation's own content is most of it. A single-exchange conversation cannot be compacted; start with less content (smaller files or pasted text).
```

Cuando la mayor parte de la solicitud está fuera de la conversación, el mensaje dice:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) but this conversation is only ~<conversation tokens> tokens — the rest is system prompt, tool definitions, and attachment content. A single-exchange conversation cannot be compacted; reduce attached files/tools or start with less context.
```

Antes de v2.1.162, Claude Code intentaba la compactación de todas formas y mostraba el `Prompt is too long` desnudo cuando fallaba.

**Qué hacer:**

* Ejecute `/compact` para resumir turnos anteriores y liberar espacio, o `/clear` para comenzar de nuevo. Si `/compact` responde `Not enough messages to compact.`, la conversación es un solo intercambio sin nada anterior para resumir, por lo que el espacio está ocupado por ese único mensaje y lo que Claude Code envía con cada solicitud: ejecute `/clear` y reenvíe con menos texto pegado o adjuntos más pequeños, o reduzca las definiciones de herramientas y archivos de memoria usando los pasos a continuación
* Ejecute `/context` para ver un desglose de lo que está consumiendo la ventana: prompt del sistema, herramientas, archivos de memoria y mensajes
* Desactive los servidores MCP que no está utilizando con `/mcp disable <name>` para eliminar sus definiciones de herramientas del contexto
* Recorte los archivos de memoria `CLAUDE.md` grandes, o mueva las instrucciones a [reglas con ámbito de ruta](/docs/es/memory#path-specific-rules) que se carguen solo cuando sea relevante
* Los subagentes heredan cada definición de herramienta MCP de la sesión principal, lo que puede llenar su ventana de contexto antes del primer turno. Desactive los servidores MCP que no está utilizando antes de generar subagentes.
* La compactación automática está activada de forma predeterminada y normalmente previene este error. Si la desactivó en `/config` o con [`DISABLE_AUTO_COMPACT`](/docs/es/env-vars), vuelva a activarla. Si la mantiene desactivada, ejecute `/compact` usted mismo antes de que la ventana se llene.

Consulte [Explorar la ventana de contexto](/docs/es/context-window) para una vista interactiva de cómo se llena el contexto.

<h3 id="context-exceeds-the-token-limit">
  El contexto excede el límite de tokens
</h3>

`/context` muestra esta advertencia en la parte superior de su salida cuando la conversación ha crecido más allá de la ventana de contexto del modelo. Las solicitudes fallan con [`Prompt is too long`](#prompt-is-too-long) hasta que libere espacio. Una sesión interactiva muestra ese error como la línea `Context limit reached`.

```text theme={null}
Context exceeds the 200k-token limit by 94k tokens — run /compact or /clear to continue.
```

Cuando el límite que excedió es una ventana de compactación, como el límite de 200K en modelos de contexto de 1M, la advertencia dice algo diferente. Una ventana de compactación puede estar por debajo de la ventana de contexto del modelo, por lo que las solicitudes más allá de ella aún pueden tener éxito.

```text theme={null}
Context is 94k tokens past the 200k-token compaction window — run /compact to reduce usage.
```

Ambas formas nombran `/clear` en lugar de `/compact` cuando ha establecido [`DISABLE_COMPACT`](/docs/es/env-vars).

**Qué hacer:**

* En una conversación de múltiples turnos, ejecute `/compact` para resumir turnos anteriores y liberar espacio. Para comenzar de nuevo, ejecute `/clear`
* Para más formas de reducir el uso, consulte [Prompt is too long](#prompt-is-too-long)

Antes de v2.1.216, `/context` mostraba el uso por encima del 100% sin una línea de advertencia que explicara qué significaba eso o cómo recuperarse.

<h3 id="error-during-compaction-conversation-too-long">
  Error durante la compactación: Conversación demasiado larga
</h3>

`/compact` en sí falló porque no hay suficiente contexto libre para contener el resumen que produce.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Esto puede suceder cuando la ventana ya está llena en el momento en que se activa la compactación automática, o cuando ejecuta `/compact` después de ver [`Prompt is too long`](#prompt-is-too-long). En una sesión interactiva, ese error es la línea `Context limit reached`.

**Qué hacer:**

* Presione Esc dos veces para abrir la lista de mensajes y retroceder varios turnos. Esto elimina los mensajes más recientes del contexto. Luego ejecute `/compact` de nuevo.
* Si retroceder no libera suficiente espacio, ejecute `/clear` para comenzar una sesión nueva. Su conversación anterior se conserva y se puede reabrirse con `/resume`.

Este mensaje y otros fallos de `/compact` se muestran en estilo de error. Antes de v2.1.216, se representaban en el mismo estilo atenuado que la salida de comando exitosa, por lo que podría leer una compactación fallida como un éxito.

<h3 id="request-too-large">
  Solicitud demasiado grande
</h3>

El cuerpo de solicitud sin procesar excedió el límite de 32MB de la API antes de la tokenización, generalmente debido a contenido pegado grande, resultados de herramientas o adjuntos. Este límite es separado de la [ventana de contexto](#prompt-is-too-long).

```text theme={null}
Request too large (max 32MB). Accumulated images and attachments in the conversation pushed the request over the limit. Run /compact, or double press esc to go back and remove attachments.
```

Cuando la solicitud fue directamente a la API de Claude y la API en sí la rechazó, Claude Code mide la conversación y redacta el mensaje según si la recuperación puede funcionar. A través de un proxy, puerta de enlace o proveedor de nube obtiene el mensaje general. Las formas medidas:

* `Request too large (max 32MB; 20.1MB of about 33.4MB is images or documents).`: las imágenes o documentos empujaron la solicitud más allá del límite. Claude Code reintenta con ellos eliminados.
* `Request too large for the API's 32MB request limit`: los mensajes solos están más allá del límite, por lo que el mensaje dice `compacting cannot make it fit` y Claude Code no reintenta. En [modo no interactivo](/docs/es/headless), el mensaje le dice que reduzca la entrada o comience una nueva sesión en su lugar.

Antes de v2.1.212, las conversaciones con suficientes imágenes acumuladas fallaban en cada turno con `Request too large (max 32MB). Double press esc to go back and try with a smaller file.` Antes de v2.1.229, Claude Code mostraba el consejo de adjuntos para cada rechazo, incluso cuando la compactación no podía ayudar.

**Qué hacer:**

* Si el mensaje dice `compacting cannot make it fit`, presione Esc dos veces para retroceder más allá del turno que agregó el contenido grande, o ejecute `/clear` para comenzar de nuevo
* De lo contrario, ejecute `/compact`, que elimina imágenes y adjuntos acumulados
* Haga referencia a archivos grandes por ruta en lugar de pegar su contenido, para que Claude pueda leerlos en fragmentos
* Para imágenes, consulte [Image was too large](#image-was-too-large) a continuación

<h3 id="image-was-too-large">
  La imagen era demasiado grande
</h3>

Una imagen pegada o adjunta excede los límites de tamaño o dimensión de la API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code reemplaza la imagen no procesable con un marcador de posición de texto y reintenta, por lo que los mensajes posteriores tienen éxito. En versiones anteriores a 2.1.142, una imagen pegada podría permanecer en la conversación y repetir el mismo error en cada mensaje posterior. Para recuperarse en esas versiones, presione Esc dos veces y retroceda más allá del turno donde se agregó la imagen.

**Qué hacer:**

* Cambie el tamaño de la imagen antes de pegarla. La API acepta imágenes de hasta 8000 píxeles en el borde más largo para una sola imagen, o 2000 píxeles cuando hay muchas imágenes en contexto.
* Tome una captura de pantalla más ajustada de la región relevante en lugar de la pantalla completa

<h3 id="unable-to-resize-image">
  No se pudo cambiar el tamaño de la imagen
</h3>

Claude Code no pudo reducir la escala de una imagen adjunta antes de enviarla a la API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
Unable to resize image — it is a CMYK JPEG, which Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Re-save it as an RGB PNG or JPEG and try again.
Unable to resize image — it is an animated WebP whose first frame Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Save its first frame as a PNG or JPEG and try again.
Unable to resize image — its pixels could not be decoded (the file may be damaged, or use an encoding Claude Code cannot read), and it is over the … API limit (… raw, … base64), so it cannot be sent. Re-save it as a PNG or JPEG and try again.
```

Claude Code normalmente cambia el tamaño de las imágenes grandes automáticamente. Estos errores significan que la imagen no se pudo decodificar o cambiar de tamaño para caber dentro de los límites de la API.

**Qué hacer:**

* Si el mensaje le pide que convierta la imagen, conviértala a PNG, JPEG, GIF o WebP y adjúntela de nuevo. Claude Code puede verificar dimensiones para estos formatos desde el encabezado del archivo, sin decodificar la imagen.
* Si el mensaje reporta un límite de dimensión o tamaño, cambie el tamaño o recomprima la imagen por debajo de ese límite antes de adjuntarla.
* Si el mensaje nombra una causa, como un JPEG CMYK, un WebP animado o un archivo posiblemente dañado, guarde la imagen en el formato que sugiere el mensaje y adjúntela de nuevo.

<h3 id="pdf-errors">
  Errores de PDF
</h3>

El PDF que adjuntó no se pudo procesar. Los mensajes se muestran aquí en su forma no interactiva; en una sesión interactiva, en su lugar le piden que presione esc dos veces e intente de nuevo.

```text theme={null}
PDF too large (max 100 pages, 20MB). Try reading the file a different way (e.g., extract text with pdftotext).
PDF is password protected. Try using a CLI tool to extract or convert the PDF.
The PDF file was not valid. Try converting it to text first (e.g., pdftotext).
```

**Qué hacer:**

* Para PDF de gran tamaño, pida a Claude que lea un rango de páginas con la herramienta Read en lugar de adjuntar el archivo completo, o extraiga texto con una herramienta como `pdftotext` y haga referencia al archivo de salida por ruta
* Para PDF protegidos o inválidos, elimine la contraseña o reexporte el archivo desde su aplicación de origen, luego intente de nuevo

<h3 id="extra-inputs-are-not-permitted">
  No se permiten entradas adicionales
</h3>

Un proxy o puerta de enlace LLM entre Claude Code y la API eliminó el encabezado de solicitud `anthropic-beta`, por lo que la API rechazó los campos que dependen de él.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code envía campos solo de beta como `context_management` y `effort` junto con un encabezado `anthropic-beta` que los habilita. Cuando una puerta de enlace reenvía el cuerpo pero elimina el encabezado, la API ve campos que no reconoce.

**Qué hacer:**

* Configure su puerta de enlace para reenviar el encabezado `anthropic-beta`. Consulte [feature pass-through](/docs/es/llm-gateway-protocol#feature-pass-through) para saber qué puertas de enlace deben reenviar.
* Como alternativa, establezca [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/es/env-vars) antes de lanzar. [Disable pre-release capabilities](/docs/es/llm-gateway-protocol#disable-pre-release-capabilities) cubre el alcance exacto.

<h3 id="tool-input-schema-is-invalid">
  El esquema de entrada de herramienta no es válido
</h3>

Una herramienta en la solicitud declaró un `input_schema` que falla la validación del esquema JSON de la API, por lo que la API rechazó toda la solicitud. El número después de `tools.` es la posición de la herramienta que falla en la lista de herramientas de la solicitud, no un nombre que pueda buscar.

```text theme={null}
API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid
API Error: 400 ... tools.N.custom.input_schema.properties: Property keys should match pattern '^[a-zA-Z0-9_.-]{1,64}$'
```

La primera forma significa que el esquema no es un esquema JSON válido draft 2020-12. La segunda significa que un nombre de propiedad de nivel superior no coincide con el patrón que cita el mensaje.

Claude Code [excluye herramientas MCP cuyo esquema de entrada fallaría esta validación](/docs/es/mcp#tools-with-invalid-input-schemas) cuando carga las herramientas de un servidor, por lo que las solicitudes normalmente nunca incluyen una.

En una [implementación donde la obtención de banderas está desactivada](/docs/es/env-vars#features-that-need-feature-flag-fetching), o en una máquina cuyas banderas nunca han llegado, Claude Code registra en el registro del servidor qué herramienta sería rechazada pero la envía de todas formas, por lo que este error aún puede ocurrir.

El error también puede ocurrir para una herramienta cuyo esquema declara un dialecto de esquema JSON distinto de draft 2020-12 en `$schema`. Claude Code no verifica esos esquemas contra el meta-esquema del esquema JSON, aunque la verificación del nombre de propiedad de nivel superior aún se aplica.

Antes de v2.1.216, ninguna implementación ejecutaba las verificaciones de exclusión.

**Qué hacer:**

* Si su versión de Claude Code es anterior a v2.1.216, ejecute `claude update`.
* Elimine o [desactive](/docs/es/mcp#disable-a-server-without-removing-it) el servidor MCP que declara el esquema inválido. El error nombra la herramienta solo por posición. En v2.1.216 o posterior, verifique el registro de cada servidor para una línea que nombre una herramienta cuyo esquema de entrada sería rechazado. Si ningún registro nombra una, desactive los servidores uno a la vez.
* Si mantiene el servidor, corrija el `input_schema` de la herramienta. El esquema debe ser un esquema JSON válido, y los nombres de propiedad de nivel superior deben tener entre 1 y 64 caracteres de largo y usar solo letras ASCII y dígitos, `_`, `.` y `-`. Consulte [Tools with invalid input schemas](/docs/es/mcp#tools-with-invalid-input-schemas).

<h3 id="theres-an-issue-with-the-selected-model">
  Hay un problema con el modelo seleccionado
</h3>

El nombre del modelo configurado no fue reconocido o su cuenta carece de acceso a él. A partir de v2.1.160, la sugerencia final, que se muestra aquí en su forma interactiva, varía según la superficie.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Qué hacer:**

* **CLI interactivo**: ejecute `/model` para elegir entre los modelos disponibles para su cuenta.
* **Modo no interactivo (`-p`)**: pase `--model` con un alias o ID válido, o establezca [`ANTHROPIC_MODEL`](/docs/es/env-vars). El texto de error muestra `Run --model` en esta superficie.
* **Agent SDK**: el texto de error omite la sugerencia porque el modelo se establece mediante programación. Establezca [`model` en `Options`](/docs/es/agent-sdk/typescript#options) en TypeScript o [`ClaudeAgentOptions(model=...)`](/docs/es/agent-sdk/python#claudeagentoptions) en Python, y maneje el error estructurado `model_not_found` para mostrar su propio reintento o selector de modelo.
* Use un alias como `sonnet` u `opus` en lugar de un ID completamente versionado. Los alias se resuelven a un valor predeterminado mantenido para que no se vuelvan obsoletos. Consulte [Model configuration](/docs/es/model-config).
* Si el modelo incorrecto sigue apareciendo en la CLI, un ID obsoleto está configurado en algún lugar. Verifique los lugares donde puede establecer un modelo en [orden de prioridad](/docs/es/model-config#setting-your-model) y elimine el valor obsoleto.
* Un modelo recién lanzado puede estar disponible en la API de Anthropic antes de que Amazon Bedrock, la plataforma de agentes de Google Cloud o Microsoft Foundry lo ofrezca. Si fijó un nuevo ID de modelo en uno de esos proveedores y ve este error, verifique el catálogo de modelos de su proveedor para disponibilidad en su región, y mantenga la versión anterior fijada hasta que la nueva aparezca allí.
* Claude Code reporta un inicio de sesión de claude.ai expirado como [Login expired](#login-expired), no como este error. Antes de v2.1.206, un inicio de sesión expirado que ya no se podía actualizar fallaba en cada modelo con este error; ejecute `/login` si ve eso en una versión anterior.
* Para implementaciones de la plataforma de agentes de Google Cloud, consulte [Solución de problemas de la plataforma de agentes de Google Cloud](/docs/es/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  El modelo no es un ID de modelo reconocido
</h3>

La cadena de modelo que pasó a un cambio de modelo no es un alias de modelo, un ID de modelo que esta versión de Claude Code conoce, o un ID que comienza con `claude-`. Las causas habituales son un error tipográfico en el ID, un nombre para mostrar como `Sonnet 5` donde se espera el ID `claude-sonnet-5`, o un alias que solo las versiones más nuevas de Claude Code reconocen. Claude Code rechaza el cambio inmediatamente. Antes de v2.1.200, Claude Code guardaba la cadena y fallaba en la siguiente solicitud con [There's an issue with the selected model](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

La sugerencia final nombra el alias o ID de modelo más cercano. Cuando nada es lo suficientemente cercano, dice `Run /model to see available models.` en su lugar.

Claude Code produce este error localmente en el momento en que se solicita el cambio, antes de que se realice ninguna solicitud de API. Se aplica cuando un modelo se establece a través del método [Agent SDK](/docs/es/agent-sdk/typescript) `setModel()`, por una aplicación como la [aplicación de escritorio](/docs/es/desktop) que ejecuta la CLI de Claude Code para usted, o cuando elige un modelo desde un dispositivo conectado a través de [Remote Control](/docs/es/remote-control). Antes de v2.1.260, la verificación no cubría las selecciones de Remote Control, por lo que Claude Code aplicaba la selección y la siguiente solicitud fallaba con [There's an issue with the selected model](#theres-an-issue-with-the-selected-model).

**Qué hacer:**

* Ejecute `/model` sin argumento para abrir el selector y elegir entre los modelos disponibles para su cuenta, luego pase el alias o ID que se muestra allí
* Si usó un alias que una versión más nueva de Claude Code admite, ejecute `claude update`. Un ID completo que comienza con `claude-` pasa esta verificación local incluso cuando el modelo es más nuevo que su versión de Claude Code. El servidor aún puede requerir una versión mínima para ese modelo; consulte [Claude Code does not support this model](#claude-code-does-not-support-this-model).
* Un modelo guardado antes de v2.1.200 no se repara con esta verificación. Si un valor obsoleto sigue apareciendo, elimínelo de las ubicaciones enumeradas en [Setting your model](/docs/es/model-config#setting-your-model).
* La verificación se ejecuta solo en la API de Anthropic. En cualquier otro proveedor o puerta de enlace, incluida una `ANTHROPIC_BASE_URL` personalizada, el proveedor define los nombres de modelo, por lo que Claude Code acepta cualquier cadena y la pasa. Claude Code aún puede escribir la [línea de diagnóstico de modelo no reconocido](#unrecognized-model-id-on-a-request) en el momento de la solicitud, en cada proveedor.

<h3 id="model-not-found">
  Modelo no encontrado
</h3>

Eligió un modelo con `/model <name>` y Claude Code no pudo confirmar que existe un modelo con ese nombre. Cuando el nombre no es un [alias de modelo](/docs/es/model-config#model-aliases) u otra ortografía que Claude Code acepta localmente, `/model` lo verifica con una solicitud mínima de API, y este error es generalmente la respuesta de su punto final de API. Un nombre que no puede ser un ID de modelo en absoluto, como uno que contiene espacios, obtiene el mismo mensaje.

```text theme={null}
Model 'claude-opus-9' not found
```

En proveedores con ID de modelo específicos del proveedor, el mensaje puede agregar una sugerencia `Try '...' instead` que nombra el ID de su proveedor para un modelo alternativo.

**Qué hacer:**

* Ejecute `/model` sin argumento y elija entre los modelos disponibles para su cuenta, o use un [alias de modelo](/docs/es/model-config#model-aliases) como `sonnet`, que se resuelve a un valor predeterminado mantenido
* Si escribió un ID completo, verifíquelo contra el catálogo de modelos de su proveedor. Un modelo recién lanzado puede estar disponible en la API de Anthropic antes de que su proveedor o región lo ofrezca.
* Antes de v2.1.265, `/model` también rechazaba la ortografía del alias `opusplan[1m]` con este error. En esas versiones, actualice Claude Code, o establezca el modelo en [settings](/docs/es/model-config#setting-your-model) o con `--model` en su lugar.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus no está disponible con el plan Claude Pro
</h3>

Su plan de suscripción activo no incluye el modelo que seleccionó.

```text theme={null}
Claude Opus is not available with the Claude Pro plan. If you have updated your subscription plan recently, run /logout and /login for the plan to take effect.
```

**Qué hacer:**

* Ejecute `/model` y seleccione un modelo que su plan incluya
* Si actualizó su plan recientemente y aún ve esto, ejecute `/logout` y luego `/login`. El token almacenado refleja su plan en el momento en que inició sesión, por lo que actualizar en la web no tiene efecto en una sesión existente hasta que se reautentique.
* Consulte [claude.com/pricing](https://claude.com/pricing) para ver qué modelos incluye cada plan

<h3 id="claude-code-does-not-support-this-model">
  Claude Code no admite este modelo
</h3>

La API rechazó la solicitud con un 400 porque su versión de Claude Code está por debajo de un mínimo requerido. Ya sea que el modelo que seleccionó requiera una versión más nueva, que el servidor verifica por modelo, o que la política de su organización requiera una. El 400 lleva el código de error `claude_code_version_too_old`, y el mensaje dice cuál es el mínimo que se aplica.

```text theme={null}
API Error: 400 Claude Code 2.1.219 does not support this model; version 2.1.255 or newer is required. Run 'claude update', or update the Claude desktop app, then try again.
```

La redacción de la política organizacional dice:

```text theme={null}
API Error: 400 Claude Code 2.1.240 is older than the minimum version required by your organization's policy. Run 'claude update', or update the Claude desktop app, to continue.
```

**Qué hacer:**

* Ejecute `claude update`, o actualice la aplicación de escritorio Claude, luego comience una nueva sesión
* Para la redacción por modelo, puede seguir trabajando en la sesión actual cambiando a otro modelo con `/model`
* Para la redacción de la política organizacional, actualice antes de continuar

<h3 id="model-is-restricted-by-your-organizations-settings">
  El modelo está restringido por la configuración de su organización
</h3>

Su administrador de organización ha deshabilitado este modelo en la consola de administración de claude.ai, o está excluido por una lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) en la configuración administrada. Cuando el modelo restringido se estableció con `--model`, `ANTHROPIC_MODEL` o la configuración `model`, Claude Code sustituye un modelo permitido y continúa. Escribir `/model <name>` para un modelo restringido se rechaza con `Run /model to choose a different model.` y la sesión mantiene su modelo actual. El aviso de sustitución también puede aparecer a mitad de sesión después de que un administrador desactive el modelo en el que se ejecuta una sesión en la consola de administración de claude.ai.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Un aviso prefijado con un nombre de agente, habilidad o comando significa que la restricción se aplicó a ese [modelo solicitado del subagente](/docs/es/sub-agents#choose-a-model): el subagente se ejecuta en el modelo sustituido y el modelo de su sesión no cambia. Antes de v2.1.223, Claude Code mostraba el aviso solo para subagentes lanzados con la herramienta Agent.

Claude Code trata un alias de familia de modelo, uno de `opus`, `sonnet`, `haiku` o `fable`, como una solicitud de esa familia en lugar de su versión más nueva. En la API de Anthropic y en [Claude Platform on AWS](/docs/es/claude-platform-on-aws), un alias de familia restringido se resuelve a la versión más nueva de la familia que su organización y la lista de permitidos `availableModels` permiten, y el aviso de sustitución nombra esa versión. Claude Code rechaza `/model <alias>` solo cuando cada versión de la familia está restringida. Antes de v2.1.205, un alias de familia se sustituía o rechazaba basándose solo en su versión más nueva, incluso cuando una versión anterior de la misma familia estaba permitida.

**Qué hacer:**

* Ejecute `/model` para elegir entre los modelos que su organización permite. Los modelos restringidos están ocultos en el selector.
* Si el modelo restringido se estableció en `--model`, `ANTHROPIC_MODEL`, el campo `model` de un archivo de configuración, o el frontmatter `model` de un [subagente](/docs/es/sub-agents#choose-a-model), habilidad o comando, elimine o actualice ese valor para que el aviso no se repita
* Si necesita acceso al modelo restringido, pida a su administrador de organización que lo habilite. Consulte [Organization model restrictions](/docs/es/model-config#organization-model-restrictions).

<h3 id="model-switch-was-blocked-by-a-premodelswitch-hook">
  El cambio de modelo fue bloqueado por un hook PreModelSwitch
</h3>

Un [hook PreModelSwitch](/docs/es/hooks#premodelswitch) no aprobó el cambio de modelo que usted o un cliente solicitó, por lo que la sesión mantiene su modelo actual. Cuando el cambio provino de un host [Agent SDK](/docs/es/agent-sdk/overview) o [Remote Control](/docs/es/remote-control) en lugar de un comando que escribió, el mensaje dice `Model switch blocked by a PreModelSwitch hook` sin nombrar el modelo de destino.

```text theme={null}
Model switch to Opus 4.6 was blocked by a PreModelSwitch hook: Opus 4.6 is retired for this project. Use a newer model.
```

La razón después de los dos puntos dice qué rechazó el cambio:

* **Una razón que escribió un hook**: un hook PreModelSwitch proporcionó esa razón cuando [negó el cambio o pidió confirmación](/docs/es/hooks#premodelswitch-decision-control). Aborde lo que pide, o elija un modelo que sus hooks permitan.
* **`PreModelSwitch hook <name> did not respond before its timeout`**: un hook que no responde antes de su [timeout](/docs/es/hooks#timeouts) bloquea el cambio. Corrija el comando colgado o aumente el `timeout` de ese hook, luego cambie de nuevo.
* **`confirmation required, and this session cannot ask`**: un hook respondió `ask` sin una razón, y una solicitud de control no tiene forma de mostrar el mensaje de confirmación. Una orden `/model` en una ejecución [`-p`](/docs/es/headless) reporta la misma condición con `(run /model interactively to confirm)` después de la razón. Haga el cambio desde una sesión interactiva, o cambie la decisión del hook para este modelo.
* **`so organization-managed PreModelSwitch hooks could not be checked`**: Claude Code no pudo determinar qué hooks PreModelSwitch sus [plugins administrados](/docs/es/settings-reference#enabledplugins) de la organización entregan, por ejemplo porque un plugin administrado no se cargó. Uno de esos hooks podría bloquear el cambio, por lo que Claude Code se niega en lugar de aplicar el cambio sin verificar. El inicio de la razón nombra qué falló. Claude Code vuelve a verificar en cada intento de cambio, por lo que una falla que se ha aclarado desde entonces deja de bloquear; si sigue fallando, ejecute `claude --debug` y cambie de nuevo para capturar los detalles, luego corrija el plugin o pida a su administrador que lo corrija.
* **`a PreModelSwitch hook failed before answering`** o **`PreModelSwitch hooks were cancelled (the control stream closed) before answering`**: la ejecución del hook terminó sin un veredicto, y Claude Code no trata eso como aprobación. Ejecute `claude --debug` para ver qué falló, luego cambie de nuevo.

Antes de v2.1.260, el rechazo del plugin administrado decía `plugin hooks could not be loaded, so PreModelSwitch hooks could not be checked; see the debug log`. Claude Code reintentó la carga del plugin una vez y luego rechazó cambios posteriores en la sesión, incluso cuando su organización no administraba plugins. Reinicie la sesión para ejecutar la carga del plugin de nuevo en esas versiones.

<h3 id="couldnt-save-it-as-your-default">
  No se pudo guardar como su valor predeterminado
</h3>

Eligió un modelo para guardar como su valor predeterminado, por ejemplo con `/model <name>` o `Enter` en el selector `/model`, y Claude Code no pudo escribir la selección en su archivo de configuración de usuario, `~/.claude/settings.json`. El cambio en sí se aplicó, por lo que la sesión actual se ejecuta en el modelo que eligió, pero su valor predeterminado no cambia y la siguiente sesión comienza con el valor anterior.

```text theme={null}
Set model to Fable 5.1 for this session only · couldn't save it as your default: ~/.claude/settings.json can't be written (EROFS)
```

La razón después de la ruta del archivo dice qué falló:

* **`can't be written (<code>)`**: la escritura falló con el código de error del sistema operativo entre paréntesis, como `EROFS` cuando el archivo, o el archivo al que vincula, se encuentra en un sistema de archivos que rechaza escrituras. Haga el archivo escribible y cambie de nuevo. Si otra herramienta genera el archivo, establezca la clave `model` en esa herramienta en su lugar; consulte [A change you made in Claude Code is lost in new sessions](/docs/es/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **`isn't valid JSON`**: el archivo en disco no se analiza, y Claude Code lo deja sin tocar en lugar de sobrescribir contenido que no puede leer de vuelta. Corrija el error de sintaxis, luego cambie de nuevo; consulte [Fix a broken settings file](/docs/es/settings#fix-a-broken-settings-file).

Un aviso que termina `couldn't confirm it was saved as your default (~/.claude/settings.json is still being written)` significa que la escritura no había terminado después de tres segundos. Continúa en segundo plano, por lo que el valor predeterminado aún puede guardarse; verifique qué modelo comienza su siguiente sesión, o ejecute `/model <name>` de nuevo.

Antes de v2.1.265, el aviso decía que el modelo fue `saved as your default for new sessions` incluso cuando la escritura falló.

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled no es compatible con este modelo
</h3>

Su versión de Claude Code es anterior a la mínima para el modelo seleccionado. La CLI envió una configuración de pensamiento que el modelo ya no acepta.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Qué hacer:**

* Ejecute `claude update` y reinicie Claude Code. Opus 4.7 necesita v2.1.111 o posterior. Opus 4.8 necesita v2.1.154 o posterior. Sonnet 5 necesita v2.1.197 o posterior. Opus 5 necesita v2.1.219 o posterior. Opus 5.5 necesita v2.1.280 o posterior
* Si no puede actualizar, ejecute `/model` y seleccione Opus 4.6 o Sonnet 4.6 en su lugar
* Si encuentra esto en el [Agent SDK](/docs/es/agent-sdk/overview), actualice el paquete SDK en su lugar. Opus 4.8 necesita TypeScript SDK v0.3.154 o posterior y Python SDK v0.2.88 o posterior. Sonnet 5 necesita TypeScript SDK v0.3.197 o posterior. Opus 5 necesita TypeScript SDK v0.3.219 o posterior. Opus 5.5 necesita TypeScript SDK v0.3.280 o posterior

<h3 id="effort-isnt-available-with-thinking-turned-off">
  El esfuerzo no está disponible con el pensamiento desactivado
</h3>

Desactivó el [pensamiento extendido](/docs/es/model-config#extended-thinking) y ejecutó en un [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) superior a `high`. El modelo no acepta esa combinación, por lo que la API rechazó la solicitud.

```text theme={null}
API Error: Effort 'xhigh' isn't available with thinking turned off on this model · run /effort high to continue, or turn thinking back on (unset MAX_THINKING_TOKENS=0)
```

**Qué hacer:**

* [Baje el nivel de esfuerzo](/docs/es/model-config#set-the-effort-level) a `high` o inferior.
* Active el pensamiento de nuevo, por ejemplo desestableciendo [`MAX_THINKING_TOKENS`](/docs/es/env-vars) o eliminando [`"alwaysThinkingEnabled": false`](/docs/es/settings-reference#alwaysthinkingenabled) de su configuración.

Antes de v2.1.242, Claude Code mostraba el mensaje propio de la API: `API Error: 400 output_config.effort 'xhigh' is not supported when thinking is disabled on this model. Use effort 'high' or below, or enable thinking.` Antes de v2.1.251, Claude Code enviaba la solicitud en el nivel de esfuerzo que estableció, por lo que Opus 5 rechazaba cada solicitud superior a `high` con el pensamiento desactivado. Claude Code ahora envía esfuerzo `high` en su lugar a modelos que sabe que rechazan la combinación, como Opus 5, por lo que en v2.1.251 o posterior este error le llega solo desde un modelo que Claude Code no sabe que lo rechaza.

<h3 id="thinking-budget-exceeds-output-limit">
  El presupuesto de pensamiento excede el límite de salida
</h3>

El presupuesto de pensamiento extendido configurado excede la longitud de respuesta máxima, por lo que no hay espacio para la respuesta real.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code ajusta estos valores automáticamente en la API de Anthropic. Típicamente ve este error en Amazon Bedrock o en la plataforma de agentes de Google Cloud cuando [`MAX_THINKING_TOKENS`](/docs/es/env-vars) se establece más alto que el límite de salida del proveedor, o cuando el modo de plan aumenta el presupuesto de pensamiento.

**Qué hacer:**

* Baje `MAX_THINKING_TOKENS`, o aumente [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/es/env-vars) por encima del presupuesto de pensamiento
* Consulte [Extended thinking](/docs/es/model-config#extended-thinking) para cómo el presupuesto interactúa con la longitud de salida

<h3 id="tool-use-or-thinking-block-mismatch">
  Desajuste de bloque de uso de herramienta o pensamiento
</h3>

El historial de conversación llegó a la API en un estado inconsistente, generalmente después de que una llamada de herramienta fue interrumpida o un turno fue editado a mitad de la transmisión.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 orphaned tool_result in conversation history. Run /rewind to recover the conversation.
API Error: 400 duplicate tool_use ID in conversation history. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Todas las variantes significan lo mismo: la secuencia de bloques `tool_use`, `tool_result` y `thinking` en el historial ya no coincide con lo que la API espera.

**Qué hacer:**

* Si está usando Opus 4.7 u Opus 4.8, ejecute `claude update` primero. Las versiones anteriores a v2.1.156 pueden activar este error durante el uso normal de herramientas, y `/rewind` no lo borra.
* Ejecute `/rewind`, o presione Esc dos veces, para retroceder a un punto de control antes del turno corrupto y continuar desde allí. Consulte [Checkpointing](/docs/es/checkpointing) para cómo se crean y restauran los puntos de control.

<h3 id="unsupported-tool-content-removed">
  Contenido de herramienta no compatible eliminado
</h3>

Cuando Claude Code se conecta directamente a la API de Anthropic y carga o obtiene una vista previa de una sesión guardada, elimina el contenido de herramienta que la API de Anthropic no acepta y deja esta línea donde se encontraba contenido eliminado entre dos bloques de pensamiento:

```text theme={null}
[Unsupported tool content removed]
```

Tal contenido llega a un archivo de sesión cuando algo que no es la API de Anthropic responde en el formato de la API, típicamente un proxy de terceros establecido a través de [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) que traduce llamadas de herramientas de otro proveedor. Claude Code lo elimina solo cuando la sesión se conecta directamente a la API de Anthropic, y carga el historial guardado tal como es cuando la sesión se ejecuta a través de un proxy o en otro proveedor. Antes de v2.1.246, Claude Code enviaba el uso de herramienta y su resultado de vuelta a la API, y cada turno de la sesión reanudada fallaba con un error 400 como `messages.1.content.0.server_tool_use.name: Input should be 'web_search', 'web_fetch', ...`.

**Qué hacer:**

* Ninguno necesario cuando ve la línea de marcador de posición. La sesión continúa sin el contenido eliminado.
* Si cada turno de una sesión reanudada falla con el error 400 en su lugar, ejecute `claude update` y reanude la sesión de nuevo. Las versiones anteriores a v2.1.246 no eliminan el contenido.

<h3 id="role-system-must-precede-an-assistant-message">
  role 'system' debe preceder a un mensaje 'assistant'
</h3>

La API rechazó la solicitud con un 400 porque un mensaje del sistema se encuentra en una posición en la conversación que no acepta:

```text theme={null}
API Error: 400 messages.6: role 'system' must precede an 'assistant' message or end the array; ...
```

Claude Code envía parte de su texto de recordatorio y adjuntos como mensajes del sistema dentro de la conversación. Cuando la API rechaza la posición de uno, Claude Code reintenta la solicitud una vez con ese texto enviado como mensajes de usuario ordinarios en su lugar. Las redacciones hermanas de la API, como `use the top-level 'system' parameter for the initial system prompt`, obtienen la misma recuperación.

Cuando el error aparece, el mensaje del sistema rechazado no es uno que Claude Code pueda eliminar. Eso generalmente significa que un proxy o [puerta de enlace LLM](/docs/es/llm-gateway) entre Claude Code y la API agregó un mensaje del sistema propio o reordenó la conversación.

**Qué hacer:**

* Ejecute `/clear` para comenzar una conversación nueva. Si el error regresa allí también, la causa está en la ruta de solicitud, no en la conversación guardada.
* Si el error se repite en cada turno detrás de un proxy o puerta de enlace configurada a través de [`ANTHROPIC_BASE_URL`](/docs/es/env-vars), conéctese sin el proxy para confirmar la fuente, e informe el error a quien lo opera

Antes de v2.1.280, Claude Code no reconocía esta redacción, por lo que el error también aparecía cuando el mensaje del sistema rechazado era uno que Claude Code en sí envió, y cada turno posterior de la conversación fallaba de la misma manera.

<h3 id="invalid-encrypted-content-in-search-result-block">
  Contenido\_encriptado inválido en bloque search\_result
</h3>

La API rechazó la solicitud con un 400 porque el historial de conversación contiene contenido de búsqueda web alojado que no puede descifrar. La redacción nombra el campo que no puede leer:

```text theme={null}
API Error: 400 messages.21.content.0: Invalid `encrypted_content` in `search_result` block
API Error: 400 messages.21.content.3.citations.0: Invalid `encrypted_index` in `text` block
API Error: 400 Failed to decrypt web search result content
```

Los resultados de la [herramienta de búsqueda web](/docs/es/tools-reference#websearch-tool-behavior) alojada de la API llevan campos encriptados que solo la API puede leer. La API rechaza una solicitud que reproduce contenido que no puede descifrar, como contenido producido para una organización diferente.

La propia [herramienta WebSearch](/docs/es/tools-reference#websearch-tool-behavior) de Claude Code registra los resultados de búsqueda como texto sin formato, por lo que estos bloques generalmente llegan a una conversación a través de un proxy o [puerta de enlace LLM](/docs/es/llm-gateway) que ejecutó búsqueda web alojada en sí.

Los bloques rechazados permanecen en el historial de conversación, por lo que cada turno posterior y `/compact` fallan de la misma manera.

**Qué hacer:**

* Ejecute `/clear` o comience una nueva sesión; la nueva conversación no lleva los bloques rechazados
* Si ejecuta Claude Code detrás de un proxy o puerta de enlace, informe el error a quien lo opera

<h3 id="usage-policy-refusal">
  Rechazo de política de uso
</h3>

La API se negó a responder porque el contenido en la conversación activó una verificación de [Política de uso](https://www.anthropic.com/legal/aup). El mensaje incluye un ID de solicitud que puede citar al soporte si cree que el rechazo es incorrecto.

```text theme={null}
API Error: Opus 4.6 can't help with this. Start a new session to continue.

Send feedback with /feedback or learn more: https://www.anthropic.com/legal/aup
```

El mensaje nombra el modelo que rechazó, o `Claude` cuando no se registra ningún modelo.

La verificación evalúa la conversación completa, no solo su mensaje más reciente, por lo que enviar un nuevo mensaje en la misma sesión generalmente reactiva el mismo rechazo. Lo mismo se aplica después de salir y reabrir la sesión con `--continue` o `--resume`, ya que la transcripción en disco aún contiene el contenido que activa. En [Amazon Bedrock](/docs/es/amazon-bedrock), [Plataforma de agentes de Google Cloud](/docs/es/google-vertex-ai) y [Microsoft Foundry](/docs/es/microsoft-foundry), este mensaje también cubre solicitudes que las medidas de seguridad del modelo marcaron como un tema de ciberseguridad. Consulte [Safety measures flagged a cybersecurity topic](#safety-measures-flagged-a-cybersecurity-topic).

Antes de v2.1.219, el mensaje decía `Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.`

**Qué hacer:**

* Presione Esc dos veces o ejecute `/rewind` para retroceder a un punto de control antes del turno que activó el rechazo, luego reformule o tome un enfoque diferente. Consulte [Checkpointing](/docs/es/checkpointing).
* Si no puede identificar qué turno lo causó, ejecute `/clear` para comenzar una conversación nueva en el mismo proyecto. Su conversación anterior se conserva en disco y permanece disponible en `/resume`.
* En [modo no interactivo](/docs/es/headless) (`-p`), donde el retroceso no está disponible, reintente con un mensaje reformulado en una nueva sesión sin `--continue`. Las verificaciones de política varían según el modelo, por lo que cambiar a un modelo diferente con `--model` también puede resolver el rechazo en algunos casos.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Las medidas de seguridad marcaron un tema de ciberseguridad
</h3>

Las medidas de seguridad del modelo marcaron el contenido en la conversación como un tema de ciberseguridad. El mensaje nombra el modelo que marcó la solicitud:

```text theme={null}
API Error: Opus 4.8's safeguards flagged this message. Our intentionally broad safeguards allow us to deliver more capabilities faster, but can sometimes flag legitimate cybersecurity work. Apply to the Cyber Verification Program to reduce these interruptions. Send feedback with /feedback or learn more: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude
```

El mensaje vincula al [Programa de verificación de ciberseguridad](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), que otorga acceso para trabajo de ciberseguridad legítimo. En Opus 5.5, que requiere v2.1.280 o posterior, el mensaje abre con `Opus 5.5's safeguards flagged this session` en su lugar. Cuando la categoría marcada tiene un modelo alternativo disponible, Claude Code [cambia de modelos](/docs/es/model-config#automatic-model-fallback) en lugar de mostrar este error.

En [Amazon Bedrock](/docs/es/amazon-bedrock), [Plataforma de agentes de Google Cloud](/docs/es/google-vertex-ai) y [Microsoft Foundry](/docs/es/microsoft-foundry), una bandera de ciberseguridad produce el mensaje de [rechazo de política de uso](#usage-policy-refusal) en su lugar.

La protección en sí es del lado del servidor y es anterior a v2.1.203; los lanzamientos de cliente desde entonces han cambiado solo la redacción del mensaje.
De v2.1.203 a v2.1.218, el mensaje decía `<model> has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center:` seguido del mismo enlace del centro de ayuda, y las sesiones interactivas agregaban `If you were not engaging in a cybersecurity topic, please send feedback via /feedback.`
Antes de v2.1.203, decía `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` seguido de un enlace de formulario de exención.

**Qué hacer:**

* Si su trabajo requiere este contenido, solicite acceso a través del [Programa de verificación de ciberseguridad](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Si su solicitud no era sobre un tema de ciberseguridad, ejecute `/feedback` para reportar el falso positivo
* Para seguir trabajando en la misma sesión, presione Esc dos veces o ejecute `/rewind` para retroceder a un punto de control antes del turno que activó la bandera, luego tome un enfoque diferente. Consulte [Checkpointing](/docs/es/checkpointing).

<h2 id="installation-errors">
  Errores de instalación
</h2>

Estos errores aparecen durante la instalación o actualización de Claude Code, desde el [script de instalación](/docs/es/setup#install-claude-code), `claude install`, o `claude update`. Para problemas de `command not found`, PATH, permisos y TLS durante la configuración, consulte [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  La instalación fue interrumpida antes de poder finalizar
</h3>

El script de instalación informa cuando el paso `claude install` es terminado por una señal. En Linux, el código de salida 137 significa que el proceso recibió SIGKILL, y en un host con poca memoria, generalmente es el asesino de falta de memoria (OOM) del kernel. El script imprime esta explicación y sale con el código 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Para cualquier otra señal fatal, y para el código de salida 137 en macOS, el script imprime `Installation was killed before it could finish (exit code <N>)` con el código de salida real y omite la explicación de falta de memoria. El mensaje proviene del script de instalación que usan macOS y Linux, que también cubre instalaciones dentro de WSL; los scripts de instalación nativos de Windows nunca lo imprimen. Antes de v2.1.200, el script salía solo con la línea `Killed` desnuda del shell.

**Qué hacer:**

* Detenga otros procesos para liberar memoria, luego vuelva a ejecutar el instalador
* Agregue espacio de intercambio o muévase a una instancia más grande. Consulte [Instalación interrumpida en servidores Linux con poca memoria](/docs/es/troubleshoot-install#install-killed-on-low-memory-linux-servers) para los comandos del archivo de intercambio.

<h3 id="the-connection-dropped-while-downloading-the-update">
  La conexión se interrumpió mientras se descargaba la actualización
</h3>

La conexión al servidor de descarga se cerró mientras `claude install`, `claude update`, o el [actualizador automático](/docs/es/setup#auto-updates) estaba obteniendo el binario de Claude Code, y los reintentos no se recuperaron. Claude Code reintenta la descarga cuando la conexión se interrumpe, la transferencia se estanca o el archivo descargado falla su suma de verificación, hasta tres intentos en total. Un error HTTP completado, como un 404, no se reintenta porque el servidor ya respondió. Antes de v2.1.202, una única conexión interrumpida fallaba la descarga inmediatamente con el error desnudo `aborted` en lugar de reintentar.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

El texto entre paréntesis nombra qué intento falló y el error de red subyacente. `claude update` precede el mensaje con `Error: Failed to install native update` en stderr.

Una descarga que permanece conectada pero no se completa dentro de 10 minutos falla con `Download timed out: exceeded the total deadline` en su lugar. Claude Code no reintenta una descarga agotada, porque una conexión demasiado lenta para terminar dentro del plazo no terminará en un reintento inmediato. Los pasos a continuación se aplican a ambos mensajes.

La causa habitual es un proxy o puerta de enlace que cierra una transferencia larga antes de que finalice. El binario de Claude Code es una descarga grande, por lo que un límite de conexión de proxy que nunca afecta el tráfico normal de API aún puede interrumpirlo.

**Qué hacer:**

* Ejecute `claude update` nuevamente. En una red por lo demás saludable, la descarga generalmente tiene éxito en la siguiente ejecución. Para el mensaje de tiempo agotado, ejecútelo nuevamente desde una red más rápida o menos limitada.
* Si su red requiere un proxy, establezca `HTTPS_PROXY` antes de ejecutar el instalador o `claude update`. Consulte [Verificar conectividad de red](/docs/es/troubleshoot-install#check-network-connectivity).
* Si un proxy corporativo sigue cerrando la transferencia, pida a su equipo de red que permita la descarga completa desde `downloads.claude.ai`. Consulte [Requisitos de acceso a la red](/docs/es/network-config#network-access-requirements).
* Ejecute `claude doctor` desde su shell para diagnósticos de instalación

<h2 id="command-line-errors">
  Errores de línea de comandos
</h2>

Estos errores provienen del comando `claude` de línea de comandos y sus subcomandos, de un nombre de comando que envía en el símbolo del sistema, y de comandos como `/security-review` que recopilan contexto ejecutando comandos de shell antes de que se ejecute su símbolo del sistema. También provienen de `/tui`, que relanza la CLI.

<h3 id="conflict-between-bg-and-print">
  Conflicto entre --bg y --print
</h3>

Este mensaje requiere Claude Code v2.1.198 o posterior. Combinó `--bg` con `-p` o `--print` en la misma invocación de `claude`. `--bg` inicia una [sesión en segundo plano](/docs/es/agent-view#from-your-shell) a la que se conecta posteriormente con `claude agents`, mientras que `--print` se ejecuta [de forma no interactiva](/docs/es/headless) y nunca inicia la sesión interactiva a la que `claude agents` se conecta. Antes de v2.1.198, esta combinación creaba silenciosamente un trabajo en segundo plano que nunca podría ser conectado.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**Qué hacer:**

* Elimine `-p` o `--print`. `--bg` toma el símbolo del sistema como su argumento posicional, por lo que `claude --bg "<task>"` es el comando completo. Consulte [Enviar nuevos agentes desde su shell](/docs/es/agent-view#from-your-shell).
* Para ejecutar el símbolo del sistema de forma no interactiva e imprimir el resultado en lugar de crear una sesión en segundo plano, elimine `--bg` y ejecute `claude -p "<task>"`

<h3 id="invalid-agents-configuration">
  Configuración de --agents inválida
</h3>

El valor que pasó a `--agents` es inválido, por lo que `claude` sale con código 1 en lugar de iniciar la sesión. Cuando pasa `--safe-mode`, `--resume`, o `--continue`, o establece [`CLAUDE_CODE_SAFE_MODE`](/docs/es/env-vars#variables), Claude Code no verifica el valor e inicia la sesión. Antes de v2.1.242, Claude Code iniciaba la sesión de todas formas y omitía las definiciones que no podía cargar.

```text theme={null}
Error: Invalid --agents configuration:
<what failed>
```

Lo que sigue a la primera línea depende de cómo falló el valor. Claude Code ejecuta estas comprobaciones en orden y se detiene en la primera que falla. Si su valor tiene dos tipos de problema, verá el segundo solo después de corregir el primero:

1. Cuando el valor no se analiza como JSON, Claude Code imprime una línea `invalid JSON:` que lleva el mensaje propio del analizador JSON
2. Cuando se analiza pero una definición de agente no coincide con el esquema para [subagentes definidos por CLI](/docs/es/sub-agents#choose-the-subagent-scope), Claude Code imprime una línea por problema
3. Cuando un nombre de agente comienza con `-`, Claude Code imprime `<name>: agent names must not start with '-'`

Cuando hay más de 20 líneas de problema, Claude Code imprime las primeras 20 y reemplaza el resto con `…and N more`.

**Qué hacer:**

* Corrija cada problema que enumera el mensaje, luego ejecute el comando nuevamente. Consulte [los campos que toma un subagente definido por CLI](/docs/es/sub-agents#choose-the-subagent-scope).

<h3 id="cloud-sessions-cannot-be-created-from-a-restricted-session">
  Las sesiones en la nube no se pueden crear desde una sesión --restricted
</h3>

Cuando inicia una sesión con [`--restricted`](/docs/es/cli-reference#cli-flags), Claude Code se niega a crear [sesiones en la nube](/docs/es/claude-code-on-the-web#from-terminal-to-cloud) desde ella, porque la nueva sesión se ejecutaría fuera del proceso restringido y no haría cumplir el modo restringido. Claude Code se niega en el cliente, antes de contactar al servidor, por lo que no se crea ninguna sesión en la nube:

```text theme={null}
Cloud sessions cannot be created from a --restricted session: they would not enforce it.
```

**Qué hacer:**

* Ejecute la tarea localmente en la sesión restringida
* Si controla cómo se lanzó la sesión, inicie una nueva sesión `claude` sin `--restricted` y cree la sesión en la nube desde allí

Antes de v2.1.248, Claude Code no tenía la bandera `--restricted`; las versiones anteriores rechazan la bandera en sí con un error de opción desconocida.

<h3 id="cloud-sessions-are-disabled-by-your-organizations-policy">
  Las sesiones en la nube están deshabilitadas por la política de su organización
</h3>

La política `allow_remote_sessions` de su organización está desactivada, por lo que [sesiones en la nube](/docs/es/claude-code-on-the-web) y los comandos que las utilizan no están disponibles:

```text theme={null}
Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.
```

El mensaje aparece cuando [crea una sesión en la nube desde la terminal](/docs/es/claude-code-on-the-web#from-terminal-to-cloud) y cuando envía un comando que necesita sesiones en la nube, como `/teleport`, `/remote-env`, o `/web-setup`. Antes de v2.1.268, enviar uno de esos comandos devolvía [`Unknown command`](#unknown-command) en su lugar.

Esta es una política de organización del lado del servidor, por lo que no se puede anular desde la configuración local, variables de entorno o banderas de CLI.

Si Claude Code aún no ha cargado la política de su organización o no puede obtenerla, esos comandos responden `Couldn't verify your organization's policy for cloud sessions. Check your network connection, then restart Claude Code and try again.` en su lugar.

**Qué hacer:**

* Pida a un [Propietario](/docs/es/server-managed-settings#access-control) en su organización que habilite las sesiones en la nube en la configuración de administrador de Claude Code en [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Si el mensaje dice que no pudo verificar la política, verifique su conexión de red, luego reinicie Claude Code e intente nuevamente

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  El valor de --json-schema no es un JSON Schema válido
</h3>

El esquema que pasó a [`--json-schema`](/docs/es/cli-reference#cli-flags) en [modo no interactivo](/docs/es/headless#get-structured-output) falló en la compilación de JSON Schema, por lo que `claude` sale con código 1 en lugar de ejecutar el símbolo del sistema. Antes de v2.1.205, un esquema inválido producía salida no estructurada sin error, y cualquier esquema que usara la palabra clave `format` se trataba como inválido.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

El texto después del segundo colon es el diagnóstico del validador y nombra la palabra clave o ubicación que falló. Los esquemas que usan la palabra clave `format`, como `"format": "email"`, son válidos: Claude Code acepta `format` como una anotación y no la hace cumplir.

Claude Code ejecuta dos comprobaciones antes de la compilación del esquema: rechaza un valor que no es JSON analizable con `Error: --json-schema is not valid JSON`, y JSON válido que no es un objeto con `Error: --json-schema must be a JSON object`.

**Qué hacer:**

* Corrija la parte del esquema que nombra el diagnóstico, luego vuelva a ejecutar el comando
* Si el diagnóstico es `schema too large`, reduzca el anidamiento del esquema y la reutilización de `$ref`
* Consulte [Obtener salida estructurada](/docs/es/headless#get-structured-output) para un esquema y comando que funcionen

<h3 id="settings-file-exceeds-the-2mib-limit">
  El archivo de configuración excede el límite de 2MiB
</h3>

El archivo que pasó a [`--settings`](/docs/es/cli-reference#cli-flags) es más grande que 2 MiB, por lo que `claude` sale con código 1 al inicio en lugar de cargarlo. Un archivo de configuración es un pequeño documento JSON, por lo que un archivo de este tamaño generalmente significa que la ruta apunta al archivo incorrecto. Antes de v2.1.214, Claude Code leía el archivo sin verificación de tamaño, y un archivo de varios gigabytes o un archivo de dispositivo como `/dev/zero` crecía en memoria sin límite.

```text theme={null}
Error: Settings file exceeds the 2MiB limit: /path/to/settings.json
```

Claude Code rechaza una ruta `--settings` que no es un archivo regular de la misma manera: un dispositivo, FIFO o socket reporta `Error: Cannot use settings file (Not a regular file (device, FIFO, or socket))` seguido de la ruta, y un directorio reporta una razón `EISDIR`.

**Qué hacer:**

* Apunte `--settings` a un archivo JSON de configuración regular menor a 2 MiB. Consulte [Configuración](/docs/es/settings) para el formato.

<h3 id="the-current-directory-no-longer-exists">
  El directorio actual ya no existe
</h3>

Inició `claude` desde un directorio que fue eliminado o movido después de que su shell lo ingresara, por ejemplo un worktree o directorio temporal que otro shell eliminó. Claude Code no puede leer su directorio de trabajo, por lo que sale con código 1 antes de iniciar la sesión, tanto en modo interactivo como en [modo no interactivo](/docs/es/headless). Antes de v2.1.239, Claude Code se bloqueaba con fuente de paquete minificada y un `ENOENT ... uv_cwd` sin procesar en stderr en lugar de este mensaje.

```text theme={null}
The current directory no longer exists (it was deleted or moved). Start Claude Code from an existing directory.
error: The current working directory was deleted, so that command didn't work. Please cd into a different directory and try again.
```

La causa y la solución son las mismas para ambas formas.

Cuando Claude Code no puede leer el directorio de trabajo por una razón diferente, como un cambio de permisos, el mensaje nombra el código de error en su lugar: `Can't read the current directory (EACCES). Start Claude Code from a different directory.`

En macOS, `EPERM` para un directorio en `~/Desktop`, `~/Documents`, `~/Downloads`, o iCloud Drive generalmente significa que macOS está bloqueando su aplicación de terminal desde esa carpeta. Otros comandos que leen esa carpeta fallan de la misma manera: `ls` allí reporta `Operation not permitted`, incluso con `sudo`.

**Qué hacer:**

* Cambie a un directorio que exista, como su directorio de inicio o directorio de proyecto, luego ejecute `claude` nuevamente
* Si el directorio fue recreado en la misma ruta, su shell aún mantiene el eliminado. Ejecute `cd "$PWD"` o salga y vuelva a ingresar al directorio, luego ejecute `claude` nuevamente
* Para `EPERM` en macOS, cierre su aplicación de terminal con Cmd+Q, ábrala nuevamente, regrese a esa carpeta y ejecute `claude`. Si `ls` en esa carpeta aún falla, abra **System Settings > Privacy & Security > Files and Folders**, active la carpeta para su aplicación de terminal, luego reabre la terminal

<h3 id="temp-directory-refused-or-cannot-be-created">
  El directorio temporal fue rechazado o no se puede crear
</h3>

En macOS y Linux, Claude Code crea un directorio temporal privado al inicio, `claude-<uid>` bajo el directorio temporal del sistema o la anulación [`CLAUDE_CODE_TMPDIR`](/docs/es/env-vars). Cuando el directorio no se puede crear, o una entrada ya en esa ruta falla las verificaciones de seguridad, Claude Code imprime el fallo en stderr y sale con código 1 en lugar de iniciar la sesión:

```text wrap theme={null}
ENOSPC: no space left on device, mkdir '/tmp/claude-501'

Temp directory /tmp/claude-501 is not a directory (may be an attacker-planted symlink). Refusing to use it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is owned by uid 502, expected 501. Refusing to use it — another user may have pre-created it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is not readable (its mode may have been altered, or a path component denies search). Refusing to use it — restore its permissions (chmod 0700) or remove it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.
```

**Qué hacer:**

* Para `ENOSPC`, libere espacio en disco en el volumen que contiene el directorio temporal
* Para las formas `Refusing to use it`, elimine la entrada nombrada en sí, no lo que apunta un enlace, e inicie Claude Code nuevamente; para la forma `owned by uid`, solo un administrador o ese usuario puede eliminarlo
* Para `is not readable`, ejecute `chmod 0700` en el directorio nombrado, o elimínelo e inicie nuevamente
* En cualquiera de estos casos, establezca [`CLAUDE_CODE_TMPDIR`](/docs/es/env-vars) en un directorio que controle e inicie Claude Code nuevamente, dejando la ruta rechazada sola

<h3 id="directory-couldnt-be-resolved-to-a-real-location">
  El directorio no se pudo resolver a una ubicación real
</h3>

Ejecutó `/add-dir` para un subdirectorio de su directorio de trabajo, y Claude Code no pudo resolver el directorio a su ubicación real.

Ya tiene acceso a archivos en un subdirectorio del directorio de trabajo, por lo que `/add-dir` solo carga sus skills, comandos y agentes. Antes de cargarlos, Claude Code verifica que la ubicación real del directorio, con cualquier enlace simbólico resuelto, esté dentro del directorio de trabajo. Cuando Claude Code no puede resolver esa ubicación, no carga nada y muestra este mensaje:

```text theme={null}
packages/app couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded. Check that it is a directory inside the working directory and try again.
```

**Qué hacer:**

* Verifique que la ruta nombre un directorio real dentro del directorio de trabajo, luego ejecute `/add-dir` nuevamente
* El mensaje no cambia su acceso a archivos; solo reporta que el contenido `.claude/` del directorio no fue cargado

Antes de v2.1.261, este mensaje también aparecía para cada `/add-dir <subdirectory>` cuando el directorio de trabajo estaba en un automontaje `/net/<host>`, donde Claude Code se niega a resolver rutas por diseño; el directorio estaba bien y reintentar no podía ayudar.

<h3 id="workspace-not-trusted-when-starting-remote-control">
  Espacio de trabajo no confiable al iniciar Remote Control
</h3>

Inició el modo servidor de [Remote Control](/docs/es/remote-control) con `claude remote-control` o su alias `claude rc` en un directorio que no ha confiado. El comando no muestra el diálogo de confianza del espacio de trabajo en sí, por lo que sale con código 1 y nombra la solución:

```text theme={null}
Error: Workspace not trusted. Please run `claude` in /Users/you/project first to review and accept the workspace trust dialog.
```

En su directorio de inicio el mensaje es diferente, porque el diálogo de confianza del espacio de trabajo nunca guarda confianza para el directorio de inicio, por lo que aceptarlo allí no puede satisfacer esta verificación. Antes de v2.1.214, el directorio de inicio mostraba el mensaje anterior, cuyo consejo no puede tener éxito allí.

```text theme={null}
Error: Workspace not trusted. /Users/you is your home directory, and for security home-directory trust is never saved, so running `claude` here first won't help. Run `claude rc` from a project directory instead (run `claude` there once to accept the trust dialog).
```

**Qué hacer:**

* Ejecute `claude` en el directorio, acepte el [diálogo de confianza del espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust), luego ejecute `claude remote-control` nuevamente
* En su directorio de inicio, cambie a un directorio de proyecto e inicie Remote Control allí

<h3 id="not-carried-over-to-the-sessions-remote-control-starts">
  No se trasladó a las sesiones que inicia Remote Control
</h3>

Inició [Remote Control](/docs/es/remote-control) con una bandera global `claude` antes del verbo `remote-control`, una que restringiría o configuraría las sesiones que inicia Remote Control, como `--settings`, `--setting-sources`, `--permission-mode`, `--disallowed-tools`, o `--mcp-config`. Una bandera colocada antes del verbo nunca llega a esas sesiones. Claude Code se niega a iniciar en su lugar, nombrando la bandera:

```text theme={null}
Error: `--settings` before `remote-control` is not carried over to the sessions Remote Control starts, so Remote Control refuses to start rather than drop it — remove it, and give Remote Control's own options after the verb (see `claude remote-control --help`).
```

Claude Code no se niega a banderas globales que son inofensivas de descartar, como `--verbose`, `--model`, o un `--session-id` inyectado por envoltorio o `--plugin-dir`: las ignora e inicia Remote Control.

Claude Code también se niega a iniciar para una bandera global que aún no reconoce como inofensiva, por lo que una bandera agregada en una versión más nueva puede aparecer en este mensaje hasta que una versión posterior la marque como inofensiva.

**Qué hacer:**

* Elimine la bandera de antes del verbo y pase [las opciones propias de Remote Control](/docs/es/remote-control#start-a-remote-control-session) después de él; `claude remote-control --help` las enumera
* Cuando la bandera rechazada es `--permission-mode`, ejecute `claude remote-control --permission-mode <mode>` para establecer el modo de permiso para las sesiones que inicia Remote Control

Antes de v2.1.248, `claude remote-control` no aceptaba sus propias banderas cuando una bandera global venía primero, y el comando falló con un error de `unknown option`.

<h3 id="claude-import-is-not-yet-available-in-this-build">
  claude import aún no está disponible en esta compilación
</h3>

Ejecutó [`claude import`](/docs/es/cli-reference#cli-commands), y Claude Code encontró el flujo de importación desactivado, por lo que el comando sale con código 1 en lugar de iniciar la importación. Antes de v2.1.222, una compilación con el flujo de importación desactivado trataba `import` como un símbolo del sistema e iniciaba una sesión interactiva en lugar de imprimir este mensaje.

```text theme={null}
`claude import` is not yet available in this build. Run `claude` and use /mcp or edit ~/.claude/settings.json directly.
```

Claude Code activa `claude import` a través de una bandera de característica que obtiene de Anthropic y almacena en caché en disco. Este mensaje significa que el valor almacenado en caché está desactivado. La causa es generalmente una de las siguientes:

* No ha iniciado una sesión desde la instalación, por lo que Claude Code aún no ha obtenido la bandera. El primer `claude import` puede imprimir esto incluso cuando la característica está disponible para usted.
* Usa Claude Code a través de Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o Claude Platform en AWS, o a través de una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway#availability-and-limitations). Claude Code no obtiene banderas de características en estas sesiones, por lo que `claude import` permanece no disponible.
* Estableció `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `DISABLE_GROWTHBOOK`, o [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars), que desactivan la obtención de banderas de características, por lo que `claude import` permanece no disponible.

**Qué hacer:**

* En una instalación nueva, inicie `claude`, espere a que la sesión se cargue, salga y ejecute `claude import` nuevamente
* Donde la obtención de banderas de características permanece desactivada, configure la configuración usted mismo: agregue servidores MCP con [`claude mcp add`](/docs/es/mcp#installing-mcp-servers), y cree los [archivos `CLAUDE.md`](/docs/es/memory#how-claude-md-files-load), [skills y comandos](/docs/es/skills#where-skills-live), y [subagentes](/docs/es/sub-agents#choose-the-subagent-scope) que desea trasladar. El mensaje también nombra `~/.claude/settings.json`. De la configuración que `claude import` traslada, ese archivo contiene solo el [modo de permiso](/docs/es/settings-reference#permission-settings); Claude Code no lee servidores MCP de él.

<h3 id="could-not-read-claude-code-config">
  No se pudo leer la configuración de Claude Code
</h3>

Ejecutó [`claude import`](/docs/es/cli-reference#cli-commands) mientras Claude Code no podía analizar `~/.claude.json`, el archivo donde almacena su inicio de sesión y estado por proyecto. El subcomando lee ese archivo para verificar disponibilidad pero no muestra el diálogo de recuperación que muestra la sesión interactiva, por lo que sale con código 1. Antes de v2.1.222, `claude import` con un archivo de configuración ilegible iniciaba una sesión interactiva, cuyo diálogo de recuperación manejaba el archivo.

```text theme={null}
Could not read Claude Code config — run `claude` with no arguments to recover it.
```

**Qué hacer:**

* Ejecute `claude` sin argumentos. Claude Code detecta el archivo inválido y ofrece restablecerlo. Luego ejecute `claude import` nuevamente.
* Para mantener ediciones manuales que ha realizado, corrija la sintaxis JSON en `~/.claude.json` en un editor en su lugar, luego vuelva a ejecutar `claude import`

<h3 id="could-not-import-a-server-from-claude-desktop">
  No se pudo importar un servidor desde Claude Desktop
</h3>

Claude Code no pudo agregar uno de los servidores que seleccionó en `claude mcp add-from-claude-desktop`. El comando aún importa los otros servidores seleccionados e imprime una línea por servidor que no pudo agregar. Antes de v2.1.205, el primer servidor que falló detuvo la importación y ninguno de los servidores seleccionados fue agregado.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

El texto después del nombre del servidor es la razón. La más común es la verificación de nombre: Claude Desktop permite caracteres en nombres de servidor, como espacios y períodos, que `claude mcp` restringe a letras, números, guiones y guiones bajos. Otras razones incluyen una configuración de servidor que falla en la validación y un servidor bloqueado por la [política MCP](/docs/es/managed-mcp) de su organización.

**Qué hacer:**

* Cambie el nombre del servidor en `claude_desktop_config.json` para usar solo letras, números, guiones y guiones bajos, luego ejecute `claude mcp add-from-claude-desktop` nuevamente
* Agregue ese servidor directamente con `claude mcp add` o `claude mcp add-json` bajo un nombre válido. Consulte [Importar servidores MCP desde Claude Desktop](/docs/es/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="cannot-add-mcp-server-to-the-managed-scope">
  No se puede agregar servidor MCP al alcance administrado
</h3>

Ejecutó `claude mcp add` o `claude mcp add-json` con `--scope managed`. Ese alcance contiene los servidores que su organización proporciona a través de la configuración administrada [`managedMcpServers`](/docs/es/settings-reference#managedmcpservers). Claude Code los lee solo desde configuraciones administradas, por lo que el comando no puede escribir un servidor en ese alcance.

```text theme={null}
Cannot add MCP server to scope: managed
```

**Qué hacer:**

* Agregue el servidor a un alcance en el que pueda escribir: `local`, `user`, o `project`. Sin `--scope`, el comando usa `local`. Consulte [Alcances de instalación de MCP](/docs/es/mcp#mcp-installation-scopes)
* Para proporcionar el servidor a cada usuario en su organización, agréguelo a [`managedMcpServers`](/docs/es/settings-reference#managedmcpservers) en la configuración administrada que implementa

<h3 id="cant-read-mcp-json">
  No se puede leer .mcp.json
</h3>

Un comando que lee el [`.mcp.json`](/docs/es/mcp#project-scope) del proyecto, como `claude mcp add` o `claude mcp add-json` con `--scope project`, o `claude mcp remove`, encontró que el archivo en su directorio actual no es un archivo regular o es más grande que 2 MiB, por lo que sale con este error en lugar de leer el archivo.

```text theme={null}
Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes. Fix or remove it, then run the command again.
```

Antes de v2.1.257, un FIFO en `.mcp.json` dejaba el comando esperando para siempre sin salida, y un enlace simbólico a un archivo de dispositivo como `/dev/zero` crecía en memoria hasta que el proceso fue eliminado.

**Qué hacer:**

* Verifique qué se encuentra en `.mcp.json` en su directorio actual. Reemplácelo con un archivo JSON ordinario en el [formato de alcance de proyecto](/docs/es/mcp#project-scope), o elimínelo, luego ejecute el comando nuevamente.

<h3 id="anthropic-hosted-and-doesnt-support-local-oauth">
  El servidor está alojado por Anthropic y no admite OAuth local
</h3>

Inició un inicio de sesión para un servidor MCP cuya URL apunta a un host de conector alojado por Anthropic que se autentica a través de un proveedor de identidad de terceros. Estos hosts incluyen `microsoft365.mcp.claude.com`, `gmail.mcp.claude.com`, y `gcal.mcp.claude.com`. Claude Code se niega a iniciar su flujo OAuth local para estos hosts tanto desde el panel `/mcp` como desde `claude mcp login`, porque [su inicio de sesión funciona solo a través de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai).

```text theme={null}
"gmail" is Anthropic-hosted and doesn't support local OAuth. Connect it via Settings → Connectors on claude.ai (requires `claude login`), then it'll be available here automatically.
```

Claude Code coincide con estos hosts por URL, por lo que el mensaje aparece cuando un servidor que agregó con `claude mcp add` o en `.mcp.json` apunta a uno de ellos.

**Qué hacer:**

* Elimine su entrada con `claude mcp remove <name>`, para que no pueda ocultar el conector de claude.ai en la misma URL
* Después de eliminarlo, conecte el servicio en [claude.ai/customize/connectors](https://claude.ai/customize/connectors), mientras está conectado a la cuenta que usa en Claude Code. Una vez conectado, [el conector aparece en Claude Code automáticamente](/docs/es/mcp#use-mcp-servers-from-claude-ai) si su método de autenticación activo es un inicio de sesión de suscripción de claude.ai

<h3 id="server-rejected-the-authorization-header-minted-by-the-configured-headershelper">
  El servidor rechazó el encabezado de autorización acuñado por el headersHelper configurado
</h3>

Un servidor MCP cuyo [`headersHelper`](/docs/es/mcp#use-dynamic-headers-for-custom-authentication) proporciona el encabezado `Authorization` respondió la conexión con HTTP 401 o 403, por lo que Claude Code reporta la conexión como fallida. Porque el helper proporciona el encabezado `Authorization`, Claude Code [no recurre a OAuth](/docs/es/mcp#authenticate-with-remote-mcp-servers) para el servidor:

```text theme={null}
Server rejected the Authorization header minted by the configured headersHelper (HTTP 401). Check that the helper command returns a valid credential for this MCP endpoint — OAuth fallback is disabled when the helper supplies Authorization.
```

Claude Code vuelve a ejecutar el helper en cada intento de conexión, por lo que un reintento después de un rechazo transitorio, como una carrera de rotación de token, puede tener éxito con una credencial nueva.

**Qué hacer:**

* Ejecute el comando `headersHelper` usted mismo de la manera que Claude Code lo ejecuta: desde el [directorio en el que Claude Code lo ejecuta](/docs/es/mcp#where-the-helper-runs), con las [variables de entorno que Claude Code establece para él](/docs/es/mcp#use-dynamic-headers-for-custom-authentication), y sin las [variables de credencial que Claude Code elimina](/docs/es/mcp#which-variables-a-helper-can-read) para un servidor de un `.mcp.json` de proyecto, un plugin, o un archivo de agente de proyecto. Verifique que imprima un valor `Authorization` que el punto final del servidor acepte
* Después de corregir el helper o su fuente de credencial, seleccione el servidor en `/mcp` y elija **Reconnect**

Antes de v2.1.248, Claude Code ejecutaba descubrimiento OAuth para un servidor cuyo helper proporcionaba el encabezado `Authorization`. Ese descubrimiento podría fallar con `Incompatible auth server: does not support dynamic client registration` en lugar de reportar la credencial rechazada.

<h3 id="mcp-permission-prompt-tool-not-found">
  Herramienta de solicitud de permiso MCP no encontrada
</h3>

La herramienta que pasó a [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags) no estaba entre las herramientas MCP conectadas cuando la ejecución primero necesitó una decisión de permiso, ya sea porque su servidor nunca se conectó o porque ningún servidor conectado expone una herramienta con ese nombre. Claude Code aún envía su símbolo del sistema: la ejecución [no interactiva](/docs/es/headless) sale con este error, y código de salida 1, en la primera llamada de herramienta que necesita aprobación, por lo que no produce respuesta aunque la solicitud fue realizada. Antes del primer símbolo del sistema, Claude Code espera hasta el tiempo de espera de conexión por servidor de 30 segundos establecido por [`MCP_TIMEOUT`](/docs/es/env-vars) para que ese servidor se conecte. Antes de v2.1.206, el inicio no esperaba a que el servidor terminara de conectarse, por lo que un servidor que se inicia lentamente pero saludable producía este error también.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

La lista después de `Available MCP tools:` nombra las herramientas MCP que estaban conectadas cuando terminó la espera.

**Qué hacer:**

* Verifique que el servidor se inicie y permanezca conectado: ejecute `claude mcp list` en el mismo directorio y confirme que el servidor está listado como conectado
* Confirme que el nombre de la herramienta coincida con el nombre `mcp__<server>__<tool>` que expone el servidor
* Si el servidor necesita más de 30 segundos para iniciarse, aumente [`MCP_TIMEOUT`](/docs/es/env-vars)

<h3 id="oauth-callback-port-is-already-in-use">
  El puerto de devolución de llamada OAuth ya está en uso
</h3>

Cuando inicia sesión en un servidor MCP remoto con OAuth, Claude Code inicia un oyente local para recibir la devolución de llamada de inicio de sesión. Si el puerto que necesita ese oyente está siendo utilizado por otro proceso, el inicio de sesión falla con este mensaje. Esto sucede principalmente con un [puerto de devolución de llamada fijo](/docs/es/mcp#use-a-fixed-oauth-callback-port) establecido a través de la variable [`MCP_OAUTH_CALLBACK_PORT`](/docs/es/env-vars) o `--callback-port`, ya que sin uno Claude Code elige un puerto disponible.

```text theme={null}
OAuth callback port <port> is already in use — another process may be holding it. Run `lsof -ti:<port> -sTCP:LISTEN` to find it.
```

En Windows, el comando sugerido es `netstat -ano | findstr :<port>` en su lugar.

**Qué hacer:**

* Ejecute el comando del mensaje para encontrar el proceso que mantiene el puerto, y deténgalo o espere a que termine
* Si otro programa necesita ese puerto permanentemente, registre un URI de redirección diferente con el servidor y establezca su puerto con `MCP_OAUTH_CALLBACK_PORT` o `--callback-port`, el que use
* Luego inicie el inicio de sesión nuevamente, por ejemplo seleccionando el servidor en `/mcp`

<h3 id="no-available-ports-for-oauth-redirect">
  No hay puertos disponibles para redirección OAuth
</h3>

Cuando inicia sesión en un servidor MCP remoto con [OAuth](/docs/es/mcp#authenticate-with-remote-mcp-servers), Claude Code inicia un oyente local para recibir la devolución de llamada de inicio de sesión. El inicio de sesión falla con este mensaje cuando Claude Code no puede vincular un puerto local para él. Algo en la máquina está impidiendo que escuche en `127.0.0.1`, por ejemplo software de seguridad o una política de sandbox que deniega oyentes locales.

```text theme={null}
No available ports for OAuth redirect
```

Antes de v2.1.268, Claude Code no recurría a un puerto asignado por el sistema operativo, por lo que el mensaje también aparecía cuando solo sus puertos autopickados no podían ser vinculados. Esto puede suceder en hosts Windows donde Hyper-V reserva rangos de puertos que cubren los puertos que Claude Code elige.

**Qué hacer:**

* Verifique si el software de seguridad o una política de sandbox bloquea procesos de escuchar en `127.0.0.1`, y permita que Claude Code vincule un puerto local
* Luego inicie el inicio de sesión nuevamente, por ejemplo seleccionando el servidor en `/mcp`

<h3 id="security-review-fails-without-origin-head">
  /security-review falla sin origin/HEAD
</h3>

[`/security-review`](/docs/es/commands#all-commands) construye su contexto de revisión comparando su rama contra `origin/HEAD`, la ref local que registra qué rama es la predeterminada en su remoto `origin`. Cuando esa ref no existe, los comandos git que recopilan la comparación fallan y la revisión se detiene antes de comenzar.

```text theme={null}
Error: Shell command failed for pattern "!`git diff --name-only origin/HEAD...`": [stderr]
fatal: ambiguous argument 'origin/HEAD...': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
```

El mensaje puede citar `git log` o una `git diff` diferente en su lugar. Git crea `origin/HEAD` solo cuando el remoto anuncia una rama predeterminada y su refspec de obtención la cubre, que un `git clone` completo de un remoto con commits hace. La ref falta en estas configuraciones:

* Un checkout de rama única o CI, que obtiene un refspec demasiado estrecho
* Un remoto cuyo HEAD del lado del servidor apunta a una rama que nadie envió
* Un repositorio sin remoto `origin`, o uno del que nunca obtuvo

Claude Code muestra el mismo error para cualquier skill que [inyecta contexto dinámico](/docs/es/skills#when-an-injected-command-fails), y un comando inyectado fallido aborta la invocación de ese skill. Dos cadenas hermanas se activan antes de que el comando se ejecute en absoluto:

* `Shell command permission check failed for pattern "..."`: la verificación de permiso del comando no lo permitió. [Las verificaciones de permiso en comandos inyectados](/docs/es/skills#permission-checks-on-injected-commands) cubre qué resultados abortan en cada modo de permiso y cómo pre-aprobar un comando con `allowed-tools`
* ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``: el frontmatter del skill exige bash en una máquina sin él. Instale Git para Windows o cambie el frontmatter a `shell: powershell`. Consulte [Cómo se ejecutan los comandos inyectados](/docs/es/skills#how-injected-commands-run)

**Qué hacer:**

* Cree la ref nombrando la rama predeterminada de su remoto: `git remote set-head origin <default-branch>`. Esto funciona siempre que exista la ref de seguimiento local `origin/<default-branch>`. Si no existe, como en clones de rama única, obtenga la rama primero: ejecute `git remote set-branches --add origin <branch>`, luego `git fetch origin`, luego vuelva a ejecutar el comando set-head. Vuelva a ejecutar `/security-review`.
* Si prefiere no nombrar la rama, ejecute `git fetch origin` y luego `git remote set-head origin --auto`, que pregunta al remoto qué rama es su predeterminada. Falla con `error: Cannot determine remote HEAD` cuando el remoto no anuncia una rama predeterminada, porque está vacío o su HEAD apunta a una rama que nadie envió; nombre la rama explícitamente en su lugar. Falla con `error: Not a valid ref` cuando su clon no obtiene esa rama; amplíe el refspec como se indicó anteriormente primero.
* Si el repositorio no tiene remoto, agregue uno con `git remote add origin <url>` y obtenga antes de crear la ref. Si el remoto está vacío, envíe su rama primero con `git push -u origin HEAD` y nombre esa rama en el comando set-head; `origin/HEAD` luego apunta a la rama que acaba de enviar, por lo que `/security-review` ve una comparación vacía hasta que la rama diverja de ella.

<h3 id="input-must-be-provided-when-using-print">
  Se debe proporcionar entrada cuando se usa --print
</h3>

`claude` desnudo necesita que stdout sea una terminal para iniciar la interfaz de usuario interactiva. Cuando stdout se redirige, o la consola no es una terminal real, como PowerShell ISE y algunos paneles de salida de IDE, `claude` se ejecuta en [modo no interactivo](/docs/es/headless) en su lugar. Ese es el mismo modo que `claude -p`, que requiere un símbolo del sistema, por lo que el mensaje nombra `--print` incluso cuando no pasó la bandera. Pasar `-p`/`--print` sin símbolo del sistema y nada canalizando en stdin produce el mismo error en cualquier lugar.

```text theme={null}
Error: Input must be provided either through stdin or as a prompt argument when using --print
```

**Qué hacer:**

* Para uso interactivo, ejecute `claude` en una terminal real: Windows Terminal o la consola de PowerShell en lugar de ISE, y la terminal integrada de su IDE en lugar de un panel de salida
* Para uso de una sola vez, pase el símbolo del sistema: `claude -p "your question"`, o canalícelo con `echo "your question" | claude -p`

<h3 id="input-contained-only-whitespace">
  La entrada contenía solo espacios en blanco
</h3>

En [modo no interactivo](/docs/es/headless), Claude Code rechaza un símbolo del sistema compuesto enteramente por espacios, tabulaciones o saltos de línea en lugar de enviarlo, porque la API rechaza mensajes sin texto visible. Qué mensaje ve depende de dónde vino el símbolo del sistema en blanco:

* **Argumento de símbolo del sistema o stdin canalizado para `claude -p`**: `claude` sale con `Error: Input contained only whitespace. Provide a prompt with text through stdin or as a prompt argument when using --print`
* **Mensaje enviado a una sesión `--input-format stream-json` en ejecución o [Agent SDK](/docs/es/agent-sdk/overview)**: Claude Code termina el turno sin llamar al modelo y la sesión permanece usable. El rechazo llega como un mensaje informativo y como el texto de resultado del turno: `Blank prompt — the message was only whitespace, so nothing was sent to the model.`

Antes de v2.1.229, Claude Code enviaba el mensaje solo con espacios en blanco a la API, que rechazaba la solicitud con un error 400.

**Qué hacer:**

* Incluya texto visible en el símbolo del sistema. Si un script construye el símbolo del sistema a partir de una variable o archivo, verifique que la fuente no esté vacía antes de llamar a Claude Code.

<h3 id="stream-json-input-carried-over-256m-characters-with-no-newline">
  stream-json input llevaba más de 256M caracteres sin salto de línea
</h3>

Su programa envió más de 268,435,456 caracteres en stdin sin un salto de línea a una ejecución `claude -p --input-format stream-json`, por lo que Claude Code imprime este error en stderr y sale con código 1 en lugar de almacenar más entrada en búfer. El mensaje establece ese presupuesto como `256M`. Antes de v2.1.257, Claude Code almacenaba tal entrada en búfer sin límite, creciendo en memoria hasta que el proceso se bloqueaba o era eliminado.

```text theme={null}
Error: stream-json input carried over 256M characters with no newline. Each stream-json message must be a single newline-terminated JSON line: either the producer is not newline-terminating its messages, or one message exceeded this budget.
```

La entrada de esta longitud sin un salto de línea generalmente significa que el productor no es un productor stream-json en absoluto, como un archivo binario o salida de registro simple canalizada por accidente. Un único mensaje sobre el presupuesto falla la misma verificación.

**Qué hacer:**

* Verifique qué se canaliza a stdin. Con [`--input-format stream-json`](/docs/es/cli-reference#cli-flags), cada mensaje debe ser una línea JSON terminada por salto de línea
* Para enviar texto sin formato en su lugar, elimine `--input-format stream-json`; `claude -p` lee un símbolo del sistema de texto sin formato desde stdin de forma predeterminada

<h3 id="unknown-command">
  Comando desconocido
</h3>

En una sesión de terminal interactiva, envió un nombre `/` que no coincide con ningún comando en esta sesión, por lo que Claude Code reporta el nombre en lugar de ejecutar cualquier cosa:

```text theme={null}
Unknown command: /hepl. Did you mean /help?
```

Claude Code sugiere el nombre de comando o alias más cercano que el menú enumera en esta sesión. Cuando nada está cerca, el mensaje termina después del nombre. La causa es generalmente una de las siguientes:

* Un error tipográfico, como `/hepl` para `/help`. [Cómo el menú de comandos coincide con lo que escribe](/docs/es/commands#how-the-command-menu-matches-what-you-type) cubre elegir una coincidencia cercana antes de enviar
* Un comando que existe pero no está disponible en esta sesión porque no se cumple un requisito, como su plataforma, plan o método de autenticación. Las entradas de solución de problemas para [`/web-setup`](/docs/es/web-quickstart#web-setup-shows-no-commands-match-or-unknown-command) y [`/schedule`](/docs/es/routines#schedule-returns-unknown-command) recorren dos casos comunes. Algunos comandos responden con su propio mensaje cuando la política de su organización los desactiva, como [`Cloud sessions are disabled by your organization's policy`](#cloud-sessions-are-disabled-by-your-organizations-policy)
* Un comando de un [plugin](/docs/es/plugins/overview) o [servidor MCP](/docs/es/mcp#use-mcp-prompts-as-commands) que no está instalado o conectado en esta sesión

Claude Code responde a un nombre `/` que no coincide con ningún comando de esta manera solo en una sesión de terminal interactiva. En todas las otras sesiones, envía el símbolo del sistema a Claude como un mensaje normal en su lugar, con una nota de que el comando no se ejecutó y una lista de comandos que Claude puede ejecutar en la sesión. Esas sesiones incluyen:

* Ejecuciones `-p`
* Aplicaciones [Agent SDK](/docs/es/agent-sdk/overview)
* La pestaña Code de la [aplicación Desktop](/docs/es/desktop)
* El panel de chat de la [extensión VS Code](/docs/es/vs-code)
* [Sesiones en la nube](/docs/es/claude-code-on-the-web) y [rutinas](/docs/es/routines)

Para un comando integrado que no puede ejecutarse en una de esas sesiones, Claude Code aún responde que el comando no está disponible en su lugar de enviarlo a Claude. Antes de v2.1.274, solo las sesiones en la nube y las rutinas enviaban un nombre que no coincidía a Claude. Antes de v2.1.273, también respondían `Unknown command`.

Claude Code no trata cada símbolo del sistema que comienza con `/` como un comando. Envía el símbolo del sistema a Claude como un mensaje normal cuando la primera palabra después del `/` comienza con puntuación, como el `/--` que abre un comentario de documento Lean, o es una ruta como `/var/log/syslog`.

Antes de v2.1.236, si presionaba `Enter` mientras el menú de comandos enumeraba una coincidencia cercana para el nombre que escribió, Claude Code ejecutaba esa coincidencia, por lo que un error tipográfico como `/hepl` ejecutaba `/help` en lugar de producir este mensaje.

**Qué hacer:**

* Ejecute el nombre sugerido, o escriba `/` seguido de parte del nombre para ver qué está disponible en esta sesión
* Si Claude Code reporta un comando documentado como desconocido, verifique su fila en la [referencia de comandos](/docs/es/commands) para el requisito que nombra

<h3 id="diff-is-too-large-for-ultrareview">
  La comparación es demasiado grande para ultrareview
</h3>

La comparación entre su rama y la rama base, incluidos los cambios no confirmados y preparados, excede los límites de tamaño para una [ultrareview](/docs/es/ultrareview), por lo que `/code-review ultra` y el subcomando `claude ultrareview` rechazan la revisión antes de que comience la sesión en la nube. Una revisión rechazada no usa una ejecución gratuita y no factura créditos de uso. El mensaje nombra los límites en vigor, el tamaño de su comparación y los archivos que contribuyen la mayoría de líneas cambiadas. Antes de v2.1.216, el mensaje mostraba solo las estadísticas de comparación sin procesar.

```text theme={null}
Diff is too large for ultrareview: 812 files, 96,410 lines changed (limits: 500 files, 8,000 lines). Largest files: package-lock.json (41,904 lines), dist/bundle.js (18,210 lines), src/generated/api.ts (9,876 lines). Pass a closer base branch (`/code-review ultra <branch>`) to narrow the scope, or split the change.
```

Revisar una solicitud de extracción aplica los mismos límites; esa forma del mensaje comienza `PR #<N> is too large for ultrareview` y nombra los recuentos de archivo y línea de la PR.

**Qué hacer:**

* Pase una rama base más cercana a su trabajo, como `/code-review ultra develop`, para que la revisión cubra solo la comparación contra esa rama
* Divida el cambio en ramas más pequeñas y revise cada una. Los archivos que el mensaje nombra contribuyen la mayoría de líneas cambiadas, así que comience moviendo esos a su propia rama.

<h3 id="could-not-find-merge-base-with-the-base-branch">
  No se pudo encontrar merge-base con la rama base
</h3>

`/code-review ultra` y el subcomando `claude ultrareview` revisan la comparación entre su rama y una rama base, que necesita un commit que ambas compartan. Cuando `git merge-base` no encuentra ninguno, Claude Code rechaza la revisión antes de que comience la sesión en la nube. En un clon que Claude Code puede verificar que está completo, con al menos una rama, recurre a [revisar cada archivo rastreado](/docs/es/ultrareview#diff-limits-and-fallbacks) en su lugar de rechazar. Ve este rechazo cuando la rama base no se puede encontrar en absoluto, cuando Claude Code no puede verificar que su clon está completo, o en el raro repositorio donde la comparación de árbol completo no es posible, como el formato de objeto SHA-256.

```text theme={null}
Could not find merge-base with main. Pass the base branch explicitly (e.g. `/code-review ultra develop`) or make sure you're in a git repo with a main branch.
```

La sugerencia después de la primera oración depende de lo que Claude Code observó:

* **No pasó una rama base**: Claude Code comparó contra la rama predeterminada del repositorio y sugiere pasar su base explícitamente, como en el ejemplo anterior
* **Pasó una rama base que ya estaba en su clon**: la sugerencia lee ``Make sure <branch> exists locally or on origin (try `git fetch origin <branch>`)``
* **Pasó una rama base que no estaba en su clon**: Claude Code la obtuvo de origin antes de comparar. La sugerencia lee ``<branch> was fetched from origin but shares no history with HEAD. If another branch is your real base, pass it explicitly (`/code-review ultra <branch>`)``; cuando Claude Code no puede determinar si su clon es superficial, sugiere `git fetch --unshallow origin` en su lugar. Antes de v2.1.221, la sugerencia sugería `git fetch --unshallow origin` para cada rama base obtenida, y en un clon completo ese comando falla con `fatal: --unshallow on a complete repository does not make sense`.

**Qué hacer:**

* Si otra rama es su base real, pásela explícitamente: `/code-review ultra <branch>`
* Si su clon podría no tener historial completo, ejecute `git fetch --unshallow origin` y vuelva a ejecutar la revisión

<h3 id="your-checkout-has-no-branches">
  Su checkout no tiene ramas
</h3>

Un checkout puede tener commits pero sin ramas: si ejecuta `git init` seguido de `git fetch <url>` y `git checkout FETCH_HEAD`, obtiene un HEAD desconectado sin refs. Claude Code empaqueta su repositorio como un paquete git para cargarlo para una [ultrareview](/docs/es/ultrareview), y no puede empaquetar un repositorio que no tiene ramas u otras refs, por lo que `/code-review ultra` y el subcomando `claude ultrareview` rechazan la revisión antes de que comience la sesión en la nube.

```text theme={null}
Your checkout has no branches (detached HEAD only), which cloud review can't bundle. Create one first — `git checkout -b <name>` — then rerun /code-review ultra.
```

Antes de v2.1.221, Claude Code intentaba revisar cada archivo rastreado en este checkout, y la carga fallaba.

**Qué hacer:**

* Cree una rama en su commit actual con `git checkout -b <name>`, luego vuelva a ejecutar la revisión

<h3 id="no-github-account-is-connected-to-your-claude-account">
  Ninguna cuenta de GitHub está conectada a su cuenta de Claude
</h3>

Ejecutó `/code-review ultra <PR#>` o `claude ultrareview <PR#>`, y antes de crear la sesión en la nube Claude Code pregunta al servidor si [la cuenta de GitHub conectada a su cuenta de Claude](/docs/es/ultrareview#review-a-pull-request) puede alcanzar el repositorio de la PR. Ninguna cuenta está conectada, o la conexión expiró, por lo que el clon en la nube fallaría y Claude Code rechaza el lanzamiento. Claude Code no gasta una ejecución gratuita ni factura créditos de uso para un lanzamiento rechazado.

```text theme={null}
Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected (or the connection expired). To fix: run /web-setup to reuse your GitHub CLI login, or connect an account at https://claude.ai/connect-github — then re-run /code-review ultra 1234 (allow a minute after connecting).
```

Cuando [`/web-setup`](/docs/es/web-quickstart#connect-from-your-terminal) no está disponible en su sesión, el mensaje nombra solo el enlace de claude.ai.

**Qué hacer:**

* Ejecute `/web-setup` para conectar su inicio de sesión de GitHub CLI a su cuenta de Claude, o conecte una cuenta en [claude.ai/connect-github](https://claude.ai/connect-github)
* Vuelva a ejecutar la revisión un minuto después de conectar

Antes de v2.1.248, Claude Code no verificaba esto antes del lanzamiento.

<h3 id="your-connected-github-account-cant-see-the-repository">
  Su cuenta de GitHub conectada no puede ver el repositorio
</h3>

Ejecutó `/code-review ultra <PR#>` o `claude ultrareview <PR#>`, y [la cuenta de GitHub conectada a su cuenta de Claude](/docs/es/ultrareview#review-a-pull-request) no puede leer el repositorio de la PR, por lo que el clon en la nube fallaría y Claude Code rechaza el lanzamiento. Claude Code no gasta una ejecución gratuita ni factura créditos de uso para un lanzamiento rechazado.

```text theme={null}
Your connected GitHub account can't see <owner>/<repo> — usually the Claude GitHub app isn't installed on <owner> or wasn't granted this repo (web-connected accounts need it for private repos), or a different GitHub account is connected. To fix: run /web-setup to reuse your GitHub CLI login, or install the app at https://github.com/apps/claude/installations/new — then re-run /code-review ultra 1234.
```

Cuando [`/web-setup`](/docs/es/web-quickstart#connect-from-your-terminal) no está disponible en su sesión, el mensaje nombra solo la instalación de la aplicación.

**Qué hacer:**

* Si su CLI `gh` local puede leer el repositorio, ejecute `/web-setup` para conectar ese inicio de sesión a su cuenta de Claude
* Vuelva a ejecutar la revisión después del cambio

Antes de v2.1.248, Claude Code no verificaba esto antes del lanzamiento.

<h3 id="the-github-app-preflight-failed-transiently">
  La verificación previa de la aplicación de GitHub falló transitoriamente
</h3>

Inició una [sesión en la nube](/docs/es/claude-code-on-the-web) desde un repositorio local, y dos pasos fallaron juntos. Claude Code no pudo construir o cargar el paquete de su repositorio. Antes de la carga, verificó si el servicio en la nube puede clonar el repositorio desde GitHub, y en lugar de una respuesta definitiva, esa verificación terminó en un error que un reintento podría aclarar, como un error de red, un tiempo de espera, o un error temporal del servidor. El mensaje completo comienza con lo que detuvo el paquete, por ejemplo `Could not upload repo bundle (<error>)`, y termina con la oración de verificación previa:

```text theme={null}
Could not upload repo bundle (<error>). The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead
```

**Qué hacer:**

* Vuelva a ejecutar el comando después de un momento. Cuando la verificación de GitHub pasa, Claude Code puede iniciar la sesión desde un clon de GitHub, por lo que la carga fallida ya no bloquea el lanzamiento
* Si los reintentos siguen fallando, el inicio del mensaje nombra lo que detuvo la carga. Cuando esa causa es algo que puede corregir, corríjalo para que la sesión pueda iniciarse desde su repositorio local en su lugar

Antes de v2.1.251, Claude Code terminaba el mensaje con `Please set up GitHub on https://claude.ai/code` incluso cuando la verificación de GitHub falló solo transitoriamente, y el consejo de configuración no puede aclarar una falla transitoria.

<h3 id="github-isnt-connected-to-your-claude-account">
  GitHub no está conectado a su cuenta de Claude
</h3>

Inició una [sesión en la nube](/docs/es/claude-code-on-the-web) desde su repositorio local, por ejemplo con `/autofix-pr`. Ninguna cuenta de GitHub está conectada a su cuenta de Claude, o la conexión expiró, por lo que Claude Code rechaza el lanzamiento:

```text theme={null}
GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud. Run /web-setup to connect with your GitHub CLI login, or connect on the web at https://claude.ai/connect-github
```

Cuando crea una rutina con [`/schedule`](/docs/es/routines), el mismo mensaje aparece como una nota de configuración que nombra el repositorio; la nota no bloquea la creación de la rutina.

**Qué hacer:**

* Ejecute `/web-setup` para conectar su inicio de sesión de GitHub CLI a su cuenta de Claude, o conecte una cuenta en [claude.ai/connect-github](https://claude.ai/connect-github). Consulte [Opciones de autenticación de GitHub](/docs/es/claude-code-on-the-web#github-authentication-options) para ver cómo difieren los dos.
* Vuelva a ejecutar el comando un minuto después de conectar

Antes de v2.1.268, Claude Code reportaba esto como una falla temporal de la verificación de la aplicación de GitHub de Claude y sugería reintentar o instalar la aplicación; ninguno conecta una cuenta de GitHub.

<h3 id="single-sign-on-authorization-needed">
  Autorización de inicio de sesión único necesaria
</h3>

Ejecutó [`/install-github-app`](/docs/es/github-actions#quick-setup) y eligió un repositorio cuya organización aplica inicio de sesión único SAML. Antes de la configuración, Claude Code verifica su acceso al repositorio con la CLI de GitHub, y GitHub rechazó esa verificación porque su token `gh` aún no está autorizado para la organización. El asistente muestra la advertencia con los pasos para autorizar:

```text theme={null}
Single sign-on authorization needed
<owner>/<repo> belongs to an organization that enforces SAML single sign-on, and your GitHub CLI token isn't authorized for it yet.
```

**Qué hacer:**

* Vuelva a autorizar su inicio de sesión de GitHub CLI con los alcances `repo` y `workflow` ejecutando `gh auth refresh -h github.com -s repo,workflow`, y autorice la organización cuando GitHub solicite inicio de sesión único
* Si se autentica con un token de acceso personal en `GH_TOKEN`, abra [github.com/settings/tokens](https://github.com/settings/tokens), seleccione **Configure SSO** en el token, y autorice la organización
* Ejecute `/install-github-app` nuevamente

Antes de v2.1.273, Claude Code mostraba la advertencia `Admin permissions required` para esta condición en su lugar.

<h3 id="failed-to-resume-the-conversation">
  No se pudo reanudar la conversación
</h3>

Claude Code no pudo leer o procesar la transcripción guardada para la sesión que seleccionó del [selector `claude --resume`](/docs/es/sessions#use-the-session-picker), por lo que termina el proceso en lugar de continuar en un estado parcialmente cargado. El mensaje incluye el comando para reintentar:

```text theme={null}
Failed to resume the conversation.
Run claude --resume <session-id> to retry, or claude to start a new session.
```

Claude Code sale con código 1 después de mostrar el mensaje. El selector `/resume` dentro de una sesión en ejecución reporta `Failed to resume conversation` en la conversación en su lugar, y su sesión actual sigue ejecutándose. Antes de v2.1.216, un reintento fallido desde el selector `claude --resume` permanecía en el spinner `Resuming conversation…` indefinidamente en lugar de mostrar este mensaje.

**Qué hacer:**

* Ejecute `claude --resume <session-id>` con el ID de sesión del mensaje para reintentar
* Si cada reintento falla de la misma manera, ejecute `claude update` y reanude nuevamente. Las versiones anteriores a v2.1.275 fallan en la reanudación cuando la transcripción guardada contiene una entrada que no pueden leer.
* Si el reintento falla nuevamente, ejecute `claude` para iniciar una nueva sesión

<h3 id="no-conversation-found-with-the-session-id">
  No se encontró conversación con el ID de sesión
</h3>

Pasó un ID de sesión a `claude --resume <session-id>` y ninguna transcripción guardada coincidió:

```text theme={null}
No conversation found with session ID: <session-id>
```

Claude Code sale con código 1 después de mostrar el mensaje. Claude Code [busca el proyecto actual primero, luego cada otro proyecto en esta máquina](/docs/es/sessions#resume-a-session) para el ID. Antes de v2.1.223, la búsqueda se detuvo en el directorio del proyecto actual y sus worktrees de git, por lo que reanude desde el directorio donde la sesión trabajó por última vez.

Las causas comunes:

* **ID mal escrito**: para una ejecución no interactiva, el ID es el campo `session_id` de la salida [`--output-format json`](/docs/es/headless#get-structured-output)
* **Transcripción eliminada**: Claude Code elimina transcripciones después del [período de retención](/docs/es/sessions#where-transcripts-are-stored), 30 días de forma predeterminada, siguiendo las [reglas de barrido de retención](/docs/es/claude-directory#cleaned-up-automatically)
* **Máquina diferente**: Claude Code almacena transcripciones localmente, por lo que reanude la sesión en la máquina donde se ejecutó
* **Copias duplicadas**: si copió un directorio de proyecto bajo `~/.claude/projects` para que dos transcripciones lleven el mismo ID, Claude Code reporta este mensaje en lugar de reanudar una copia arbitrariamente

**Qué hacer:**

* Para una sesión interactiva, abra el [selector de sesión](/docs/es/sessions#use-the-session-picker) con `claude --resume` y presione `Ctrl+A` para ampliarlo a cada proyecto en esta máquina, luego seleccione la sesión
* Las sesiones creadas con `claude -p` o el [Agent SDK](/docs/es/agent-sdk/overview) no aparecen en el selector, por lo que vuelva a verificar el ID contra el `session_id` que su ejecución original imprimió

<h3 id="cannot-switch-renderers-in-this-session">
  No se puede cambiar renderizadores en esta sesión
</h3>

Cuando cambia renderizadores, Claude Code reinicia su proceso. Ejecutó [`/tui`](/docs/es/fullscreen#enable-fullscreen-rendering) en una sesión que Claude Code se niega a reiniciar, por lo que no cambia y no guarda nada. Qué mensaje ve le dice la causa:

* `Cannot switch renderers while work is running in the background`: tiene trabajo en segundo plano ejecutándose que un reinicio abandonaría, como un shell en segundo plano o un subagente. Espere a que el trabajo termine o deténgalo con [`/tasks`](/docs/es/commands), luego ejecute `/tui fullscreen` o `/tui default` nuevamente
* `Cannot switch renderers in this session`: la sesión tiene restricciones que Claude Code no puede pasar al proceso reiniciado. Antes de v2.1.234, Claude Code reiniciaba de todas formas y la sesión relanzada se ejecutaba sin ellas

En el mensaje de restricciones, la parte entre paréntesis nombra las restricciones que Claude Code encontró:

```text theme={null}
Cannot switch renderers in this session — it has restrictions a restart can't carry over (permission rules set for this session only). Nothing was changed. Running /tui fullscreen in a session started without them switches every later session too.
```

Cada razón que el mensaje puede mostrar entre paréntesis:

* `launch flags: a custom system prompt, a tool allowlist, or restricted settings`: inició la sesión con una bandera que Claude Code no pasa de vuelta al proceso reiniciado. Estas banderas incluyen [`--system-prompt`](/docs/es/cli-reference#cli-flags), `--system-prompt-file`, `--append-system-prompt-file`, una lista de permitidos [`--tools`](/docs/es/cli-reference#cli-flags), [`--setting-sources`](/docs/es/cli-reference#cli-flags), y [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags)
* `permission rules set for this session only`: una [actualización de permiso](/docs/es/hooks#permission-update-entries) de un hook o llamador SDK agregó reglas de denegación o solicitud con el destino `session`. Las reglas de permiso de alcance de sesión no desencadenan el rechazo. Un reinicio las elimina, y Claude Code solicita nuevamente en su lugar
* `ask-before-running rules with no command-line form`: una actualización de permiso agregó reglas de solicitud junto con las reglas que Claude Code pasa de vuelta como `--allowed-tools` y `--disallowed-tools`. No existe bandera para reglas de solicitud
* `permission rules a command line cannot carry intact` y `added directories a command line cannot carry intact`: una actualización de permiso agregó una regla o ruta de directorio a mitad de sesión. La línea de comandos del proceso reiniciado no puede llevar su texto como el mismo valor

**Qué hacer:**

* En una sesión iniciada sin esas restricciones, ejecute `/tui fullscreen`, o `/tui default` para cambiar de vuelta. Claude Code guarda la [configuración `tui`](/docs/es/settings-reference#tui) allí

<h3 id="couldnt-open-claude-desktop">
  No se pudo abrir Claude Desktop
</h3>

Ejecutó [`/desktop`](/docs/es/desktop#coming-from-the-cli), o su alias `/app`, y el comando del sistema que Claude Code usa para abrir Claude Desktop falló. La sesión permanece en la terminal.

```text theme={null}
Error: Couldn't open Claude Desktop (`open` exited 1: LSOpenURLsWithRole() failed for the URL claude://resume?session=<session-id> with error -10814). Open Claude Desktop and run /desktop again.
```

**Qué hacer:**

* Abra Claude Desktop usted mismo, luego ejecute `/desktop` nuevamente
* Para leer la salida de error completa de ese comando, active el registro de depuración con `/debug`, ejecute `/desktop` nuevamente, y verifique el registro de depuración

Antes de v2.1.275, el mensaje era `Failed to open Claude Desktop. Please try opening it manually.` y no decía qué falló.

<h3 id="terminal-setup-left-your-zed-keymap-unchanged">
  /terminal-setup dejó su mapa de atajos de teclado de Zed sin cambios
</h3>

Ejecutó [`/terminal-setup`](/docs/es/terminal-config#enter-multiline-prompts) en Zed, y Claude Code no pudo completar la actualización a su `keymap.json` de Zed, por lo que lo dejó como estaba.

Cada mensaje nombra la ruta a su mapa de atajos de teclado y termina con el bloque de atajos de teclado para agregar usted mismo:

```text theme={null}
Couldn't update your Zed keymap, so it was left unchanged.
To add the binding yourself, add this block to the keymap array in <path to keymap.json>:
{ "context": "Terminal", "bindings": { "shift-enter": ["terminal::SendText", "\u001b\r"] } }
```

La primera línea del mensaje nombra la causa:

* `Couldn't read your Zed keymap, so it was left unchanged.`: Claude Code no pudo leer el archivo, por ejemplo debido a permisos de archivo
* `Your Zed keymap isn't a readable list of keybindings, so it was left unchanged.`: el archivo se leyó bien pero no se analiza como una matriz de bloques de atajos de teclado, incluso con comentarios `//` y comas finales permitidas
* `Couldn't back up your Zed keymap; not modifying it.`: Claude Code no pudo copiar el archivo a una copia de seguridad `.bak` junto a él, por lo que no cambió nada
* `Couldn't update your Zed keymap, so it was left unchanged.`: el resultado fusionado no se verificó como un mapa de atajos de teclado válido que lleve el atajo de teclado, por lo que Claude Code lo descartó en lugar de escribir. Un bloque de atajos de teclado con una clave duplicada puede causar esto

**Qué hacer:**

* Copie el bloque del mensaje en la matriz de nivel superior en su `keymap.json` en la ruta que nombra el mensaje
* Para `isn't a readable list of keybindings`, corrija el error de sintaxis, o haga que el valor de nivel superior del archivo sea una matriz, luego ejecute `/terminal-setup` nuevamente

Antes de v2.1.247, `/terminal-setup` no podía analizar un mapa de atajos de teclado de Zed que usara comentarios `//` o comas finales, y reemplazó el archivo completo con solo su propio atajo de teclado mientras reportaba el atajo de teclado como instalado. Para restaurar un mapa de atajos de teclado que una versión anterior reemplazó, use el archivo de copia de seguridad `.bak` descrito en [Ingrese indicadores de varias líneas](/docs/es/terminal-config#enter-multiline-prompts).

<h3 id="skill-usage-reports-are-not-available-on-this-connection">
  Los reportes de uso de skills no están disponibles en esta conexión
</h3>

Ejecutó [`/skill-doctor`](/docs/es/skills#find-unused-skills) sobre [Remote Control](/docs/es/remote-control), desde su teléfono o navegador. Claude Code no envía el informe de uso de skills sobre Remote Control y responde con este mensaje en su lugar:

```text theme={null}
Skill usage reports are not available on this connection.
```

**Qué hacer:**

* Ejecute `/skill-doctor` en la terminal en la máquina donde se ejecuta la sesión, o ejecute `claude -p "/skill-doctor"` allí

<h3 id="custom-output-styles-cant-be-selected-over-remote-control">
  Los estilos de salida personalizados no se pueden seleccionar sobre Remote Control
</h3>

Ejecutó [`/output-style`](/docs/es/output-styles#change-your-output-style) desde la aplicación móvil o web a través de [Remote Control](/docs/es/remote-control), o el comando llegó en un mensaje retransmitido a la sesión. Porque tal turno puede no provenir del propietario de la cuenta, Claude Code enumera y selecciona solo [estilos integrados](/docs/es/output-styles#built-in-output-styles) en él, y agrega este aviso siempre que el comando enumera los estilos o no reconoce el nombre que dio. Un nombre de [estilo personalizado](/docs/es/output-styles#create-a-custom-output-style) obtiene la misma respuesta que un nombre que no existe:

```text theme={null}
Custom output styles can't be selected over Remote Control or from a relayed message. Select one in the session itself, or pick a built-in style here.
```

**Qué hacer:**

* Elija un estilo integrado, por ejemplo `/output-style concise`
* Para usar un estilo personalizado, establezca [`outputStyle`](/docs/es/settings-reference#outputstyle) en `.claude/settings.local.json` del proyecto, o ejecute `/output-style <style>` en la terminal de la sesión si tiene una

<h3 id="output-styles-are-saved-to-local-settings-which-this-session-doesnt-load">
  Los estilos de salida se guardan en la configuración local que esta sesión no carga
</h3>

Intentó cambiar [estilos de salida](/docs/es/output-styles) con `/output-style <style>` o `/config outputStyle=<style>` en una sesión cuyas fuentes de configuración excluyen `local`. Los ejemplos son una sesión [Agent SDK](/docs/es/agent-sdk/typescript) cuyo [`settingSources`](/docs/es/agent-sdk/typescript#options) deja fuera `"local"` y una sesión CLI iniciada con un valor [`--setting-sources`](/docs/es/cli-reference#cli-flags) que deja fuera `local`. Ambos comandos guardan el estilo en `.claude/settings.local.json`, un archivo que tal sesión nunca vuelve a leer, por lo que Claude Code se niega en lugar de escribir una configuración que no tendría efecto:

```text theme={null}
Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load, so the style can't be changed here.
```

**Qué hacer:**

* Agregue `local` a las fuentes de configuración de la sesión y cambie nuevamente
* Establezca la clave [`outputStyle`](/docs/es/settings-reference#outputstyle) en un archivo de configuración que la sesión sí carga, como `.claude/settings.json` en el proyecto o `~/.claude/settings.json`. En el SDK de TypeScript, establezca `outputStyle` dentro del objeto `settings` en línea en su lugar; consulte [Activar un estilo de salida](/docs/es/agent-sdk/modifying-system-prompts#activate-an-output-style)

<h2 id="plugin-errors">
  Errores de plugins
</h2>

Estos errores provienen de la configuración de [plugins](/docs/es/plugins/overview) y [marketplace](/docs/es/plugins/overview). Para problemas de plugins que no producen uno de los mensajes en esta página, como una URL de marketplace que no carga o un plugin que se instala pero no aparece, consulte [Solución de problemas de plugins](/docs/es/plugins/troubleshooting).

<h3 id="plugin-eval-is-currently-in-early-access">
  plugin eval is currently in early access
</h3>

Ejecutó [`claude plugin eval`](/docs/es/plugin-evals) o `claude plugin eval init` y salió con código 1 con uno de estos mensajes antes de hacer nada:

```text theme={null}
`plugin eval` is currently in early access
```

```text theme={null}
`plugin eval` is currently unavailable
```

El primer mensaje significa que su compilación es anterior a v2.1.269, la primera versión donde el comando está disponible en general. El segundo significa que Anthropic ha desactivado el comando del lado del servidor; nada en su máquina lo vuelve a activar.

**Qué hacer:**

* Ejecute `claude --version`, luego `claude update`, y ejecute el comando nuevamente en una sesión nueva. Consulte los [requisitos para plugin evals](/docs/es/plugin-evals#requirements)
* Si ve el segundo mensaje en una compilación actual, intente nuevamente más tarde después de otro `claude update`

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace is registered from an untrusted source
</h3>

El marketplace está registrado bajo un nombre que está [reservado para marketplaces oficiales de Anthropic](/docs/es/plugins/marketplace-reference#marketplace-file), pero su fuente registrada no es un repositorio de GitHub de `anthropics`. Claude Code vuelve a verificar los nombres reservados cada vez que carga o actualiza un marketplace, por lo que el marketplace y los plugins instalados desde él dejan de cargarse. Antes de v2.1.205, el nombre se verificaba solo cuando se agregaba el marketplace, por lo que una entrada registrada antes de que su nombre se reservara seguía cargándose.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

Para un marketplace cuya fuente no es un repositorio de GitHub o una URL de Git, como un directorio local, la oración del medio dice `can only be used with GitHub sources from the 'anthropics' organization` en su lugar. `claude plugin marketplace add` ejecuta la misma verificación y rechaza un nombre reservado con `Failed to add marketplace:` seguido de la misma oración de nombre reservado.

**Qué hacer:**

* Si el marketplace ya está registrado, ejecute `claude plugin marketplace remove <name>`, luego agréguelo nuevamente desde el repositorio oficial `github.com/anthropics`
* Si publica un marketplace de terceros que utilizó el nombre antes de que se reservara, cámbielo de nombre y pida a los usuarios que lo vuelvan a agregar desde su fuente
* Consulte la lista de nombres reservados en [Marketplace schema](/docs/es/plugins/marketplace-reference#marketplace-file)

<h3 id="marketplace-name-is-another-spelling-of-a-reserved-name">
  Marketplace name is another spelling of a reserved name
</h3>

El nombre del marketplace no es en sí mismo un nombre reservado, pero Claude Code lo trata como otra ortografía de uno. [Reserved names](/docs/es/plugins/marketplace-reference#reserved-name-spellings) enumera qué ortografías cuentan como un nombre reservado. Claude Code rechaza tal nombre cuando agrega el marketplace:

```text theme={null}
Failed to add marketplace: "claude.code.plugins" is another spelling of "claude-code-plugins", a reserved marketplace name.
```

Cuando un marketplace ya está registrado bajo tal nombre, su entrada deja de cargarse, y `/plugin`, `claude plugin install`, y `claude plugin update` advierten:

```text wrap theme={null}
known_marketplaces.json has an entry named "claude.code.plugins", another spelling of the reserved marketplace name "claude-code-plugins", so it is ignored. Remove it with: claude plugin marketplace remove claude.code.plugins
```

Cuando el nombre requeriría entrecomillado de shell, la negativa en tiempo de adición dice `This marketplace's name is another spelling of "<reserved>", a reserved marketplace name. It is not exactly the reserved name it appears to be.`

**Qué hacer:**

* Cambie el nombre del marketplace a un nombre que no deletree un nombre reservado y agréguelo nuevamente
* Para la advertencia de entrada ignorada, ejecute el comando `claude plugin marketplace remove` que proporciona, o elimine la entrada de `~/.claude/plugins/known_marketplaces.json`

<h3 id="marketplace-is-already-added-from-a-different-source">
  Marketplace is already added from a different source
</h3>

Confirmó agregar un marketplace a través de [`/plugin install <plugin> --marketplace <source>`](/docs/es/plugins/install#add-a-marketplace-and-install-in-one-command), y el catálogo que Claude Code obtuvo de esa fuente se llama a sí mismo igual que un marketplace que ya agregó desde una fuente diferente. Claude Code mantiene el marketplace existente en lugar de reemplazarlo, y el plugin no se instala.

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

**Qué hacer:**

* Si el marketplace que ya agregó es el que desea, instale desde él por nombre: `/plugin install <plugin>@<name>`
* Para cambiar a la nueva fuente, ejecute `/plugin marketplace remove <name>`, luego reintente la instalación

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

Un hook de plugin, [monitor](/docs/es/plugins/components#monitors), o comando MCP [`headersHelper`](/docs/es/mcp#use-dynamic-headers-for-custom-authentication) hace referencia a una [opción de plugin](/docs/es/plugins/manifest-reference#user-configuration) `${user_config.KEY}`, y la cadena sustituida se pasaría a un shell. Un valor configurado que contenga `$(...)`, comillas invertidas o `;` se ejecutaría como código allí, por lo que Claude Code se niega a iniciar el componente en lugar de sustituir el valor. La verificación se ejecuta en la plantilla de comando, por lo que el error aparece incluso cuando aún no se ha configurado ningún valor. Antes de v2.1.207, el valor se sustituía en el comando del shell.

La redacción depende de qué superficie hizo referencia a la opción. Un hook de forma de shell informa:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Un monitor informa:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

Un `headersHelper` de MCP informa:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Qué hacer:**

* Para un hook, agregue una matriz `args` para que se ejecute en [forma exec](/docs/es/hooks#exec-form-and-shell-form), donde cada `${user_config.KEY}` se convierte en un argumento sin shell en el medio. O elimine la referencia y lea la variable de entorno `$CLAUDE_PLUGIN_OPTION_<KEY>` dentro del script
* Para un monitor, elimine la referencia y haga que el script del monitor lea el valor de un archivo de configuración
* Para un `headersHelper`, mueva `${user_config.KEY}` al campo `headers` del servidor, que no se analiza con shell, o lea el valor dentro del script del helper

<h3 id="plugin-archive-integrity-check-failed">
  Plugin archive integrity check failed
</h3>

La entrada del marketplace del plugin utiliza una [fuente `archive`](/docs/es/plugins/marketplace-reference#archive-plugin-source) con un pin `sha256`, y el resumen del archivo descargado no coincide con el pin. Claude Code rechaza la instalación, por lo que nada cambia en la caché de plugins. La falta de coincidencia tiene tres causas posibles:

* El archivo en la URL cambió después de que el autor calculó el pin
* El autor ingresó el resumen incorrecto en la entrada del marketplace
* La URL sirve un archivo diferente al que el autor fijó

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

**Qué hacer:**

* Si publica el plugin, recalcule el resumen del archivo exacto que sirve la URL, por ejemplo con `shasum -a 256 my-plugin.zip`, o `Get-FileHash -Algorithm SHA256 my-plugin.zip` en PowerShell, y actualice el `sha256` en la entrada del marketplace
* Si instala el plugin, ejecute `/plugin marketplace update <name>` para actualizar el catálogo en caso de que la entrada se haya corregido, luego reintente la instalación
* Si los resúmenes aún no coinciden después de una actualización, pregunte al propietario del marketplace qué archivo fijaron antes de instalar

<h3 id="path-escapes-plugin-directory">
  Path escapes plugin directory
</h3>

Una ruta de componente de plugin, declarada en el `plugin.json` del plugin o en su [entrada de marketplace](/docs/es/plugins/marketplace-reference#plugin-entries), se resuelve fuera del directorio del plugin. Claude Code descarta esa ruta y carga el resto del plugin. El nombre del componente en el mensaje, como `commands` o `hooks`, nombra el campo que declaró la ruta.

```text theme={null}
commands path escapes plugin directory: ./../shared.md
```

En la salida del comando `claude plugin`, el mismo error dice `Path escapes plugin directory: ./../shared.md (commands)`.

Claude Code rechaza tanto una ruta que apunta fuera del plugin tal como está escrita, como `../shared-utils`, como un enlace simbólico que conduce fuera del plugin y no es uno que las [reglas de enlace simbólico del marketplace](/docs/es/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks) permitan. Para un enlace simbólico, el mensaje también dice dónde se resuelve la ruta:

```text theme={null}
commands path escapes plugin directory: ./commands/deploy.md — it resolves to /home/user/shared/deploy.md, outside the plugin directory
```

En macOS y Linux, Claude Code también rechaza una ruta de componente que contenga una barra invertida en cualquier lugar, incluso cuando la ruta permanece dentro del plugin. Un plugin cuyas rutas de componentes utilizan separadores de estilo Windows se carga en Windows y activa este rechazo en las otras plataformas:

```text theme={null}
commands path escapes plugin directory: ./commands\deploy.md — its path contains a backslash, which is not resolved reliably on this platform
```

Antes de v2.1.251, Claude Code cargaba una ruta `commands` declarada en una entrada de marketplace incluso cuando apuntaba fuera del directorio del plugin. Claude Code ya rechazaba las rutas declaradas en `plugin.json` y las otras rutas de componentes en una entrada de marketplace.

Antes de v2.1.257, la verificación solo miraba la ortografía de la ruta, no dónde conducía un enlace simbólico.

**Qué hacer:**

* Mueva el archivo referenciado dentro del directorio del plugin y apunte la ruta a él con una ruta relativa `./`
* Si la ruta es un enlace simbólico a un archivo fuera del plugin, reemplace el enlace simbólico con una copia del archivo
* Si el mensaje dice que la ruta contiene una barra invertida, escriba la ruta con barras diagonales, por ejemplo `./commands/deploy.md`
* Para compartir archivos con otros plugins en el mismo marketplace, vincúlelos con un enlace simbólico dentro del directorio del plugin, siguiendo las [reglas de enlace simbólico](/docs/es/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)

<h3 id="path-could-not-be-checked">
  Path could not be checked
</h3>

Claude Code le preguntó al sistema operativo si existe una ruta de plugin y obtuvo un error que no sea "no encontrado", por lo que no carga lo que la ruta nombra. Cuánto del plugin se carga depende de qué ruta falló:

* Una de las [ubicaciones de componentes predeterminadas](/docs/es/plugins/manifest-reference#standard-layout) de un plugin, como la carpeta `skills/`, el archivo `monitors/monitors.json`, o un [`SKILL.md` en la raíz del plugin](/docs/es/plugins/components#skills): los otros componentes del plugin aún se cargan
* El directorio del plugin en sí: nada de ese plugin se carga

No ve este error para una ruta que no existe en absoluto. En `/plugin`, el error aparece bajo el plugin y nombra la ruta y el código que devolvió el sistema operativo:

```text theme={null}
skills path could not be checked: /home/user/my-plugin/skills (ELOOP)
```

En `claude plugin list`, el mismo error dice `Path not found: /home/user/my-plugin/skills (skills, ELOOP)`.

Las causas que producen este error incluyen:

* `ELOOP`: un enlace simbólico en la ruta apunta a sí mismo o forma un bucle
* `EIO` o `ESTALE`: la ruta está en un montaje de red que está roto o obsoleto
* `EACCES`: uno de los directorios por encima de la ruta le niega permiso para atravesarlo

**Qué hacer:**

* Reemplace un enlace simbólico que apunta a sí mismo con una carpeta real, o elimínelo
* Si la ruta está en un montaje de red, remonte el recurso compartido
* Si el código es `EACCES`, restaure su permiso de ejecución en los directorios por encima de la ruta
* Ejecute `/reload-plugins` después de corregir la ruta, o reinicie Claude Code, para cargar el plugin o componente

Antes de v2.1.265, Claude Code trataba una carpeta de componentes predeterminada que no podía verificar como ausente y cargaba el plugin sin ese componente, sin error.

<h3 id="marketplace-entry-path-does-not-stay-inside-the-marketplace-directory">
  Marketplace entry path does not stay inside the marketplace directory
</h3>

La [entrada de marketplace](/docs/es/plugins/marketplace-reference#plugin-entries) del plugin declara una ruta de fuente que Claude Code no puede resolver a una ubicación dentro del directorio del marketplace en sí, por lo que el plugin no se instala ni se carga. La negativa cubre:

* Una ruta de entrada que es absoluta, sube fuera del marketplace con `..`, o está escrita como una ruta de red
* En macOS y Linux, una ruta de entrada que contiene una barra invertida en cualquier lugar después del `./` inicial
* Una entrada en un marketplace obtenido de una fuente remota, como git o una URL, que alcanza su destino a través de un enlace simbólico que se resuelve fuera del directorio del marketplace
* Una entrada relativa en un marketplace agregado desde una URL directa a su `marketplace.json`: Claude Code descarga solo ese archivo, por lo que no existen archivos de plugin locales para que la ruta nombre. Consulte [Plugins with relative paths fail in URL-based marketplaces](/docs/es/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)

`claude plugin install` informa la negativa así:

```text theme={null}
Cannot install my-plugin@my-marketplace: its marketplace entry path does not stay inside the marketplace directory (an absolute, climbing, network-shaped, backslash-containing or link-traversing entry, an entry of a fetched marketplace that resolves or opens outside its tree — or a relative entry in a url-catalog marketplace, which has no local directory)
```

Cuando la entrada de un plugin ya instalado falla la misma verificación, `claude plugin list` muestra el plugin como `failed to load` con:

```text theme={null}
Plugin source path refused: ./my-plugin does not stay inside its marketplace directory. Check that the marketplace entry has a plain relative path.
```

**Qué hacer:**

* Si mantiene el marketplace, escriba la `source` de la entrada como una ruta relativa simple con barras diagonales, como `./plugins/my-plugin`, y mantenga cualquier enlace simbólico que cruce apuntado dentro del directorio del marketplace
* Si agregó el marketplace desde una URL directa, las entradas relativas no pueden resolverse. Pida al autor del marketplace que use [otra fuente de plugin](/docs/es/plugins/marketplace-reference#plugin-sources), o agregue el marketplace desde su repositorio de git en su lugar

<h3 id="failed-to-load-marketplace-configuration">
  Failed to load marketplace configuration
</h3>

Claude Code mantiene los marketplaces de plugins que ha agregado en un archivo de registro en `~/.claude/plugins/known_marketplaces.json`. Un comando de plugin que necesita el registro, como `claude plugin install`, falla con uno de dos mensajes cuando Claude Code no puede usar el archivo:

* `Failed to load marketplace configuration`: el archivo no es JSON válido, o no se puede leer. Un archivo vacío falla de esta manera también.
* `Marketplace configuration file is corrupted`: el archivo es JSON válido pero su contenido no coincide con el esquema del registro.

Un archivo faltante no es una falla: Claude Code lo trata como un registro sin marketplaces.

Con un archivo vacío, `claude plugin install` informa:

```text theme={null}
✘ Failed to install plugin "my-plugin": Failed to load marketplace configuration: JSON Parse error: Unexpected EOF
```

Antes de v2.1.246, `claude plugin install` no informaba esta falla.

**Qué hacer:**

* Abra `~/.claude/plugins/known_marketplaces.json` y repare el JSON, o corrija las entradas que el mensaje nombra como no coincidentes con el esquema del registro
* Si no puede repararlo, elimine el archivo o reemplace su contenido con `{}`, luego vuelva a agregar cada marketplace con `claude plugin marketplace add <source>`. Claude Code vuelve a registrar los marketplaces que su configuración de usuario o administrada declara en [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces) la próxima vez que lo inicie en una carpeta que haya confiado.

<h3 id="plugin-is-required-by-your-organization">
  Plugin is required by your organization
</h3>

Ejecutó `claude plugin disable`, o utilizó la pestaña **Installed** de `/plugin`, para desactivar un [plugin sincronizado desde claude.ai](/docs/es/plugins/loading#synced-plugins) que su organización marca como requerido:

```text theme={null}
Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.
```

Claude Code no guarda nada y el plugin permanece habilitado.

Cuando intenta desactivar un plugin del que depende un plugin requerido, Claude Code rechaza de la misma manera, con un mensaje que nombra el plugin requerido que lo necesita.

**Qué hacer:**

* Pida a un administrador de su organización claude.ai que cambie el estado requerido del plugin en claude.ai

<h2 id="tool-errors">
  Errores de herramientas
</h2>

Estos errores provienen de las herramientas integradas de Claude. Claude corrige la mayoría de los errores de herramientas por sí solo. Cuando uno requiere un cambio de su parte, la lista **Qué hacer** de ese error indica qué cambiar.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent would be spawned with zero tools
</h3>

Cada entrada en la lista [`tools` de la subagente](/docs/es/sub-agents#supported-frontmatter-fields) no coincidió con ninguna herramienta utilizable, por lo que Claude Code se negó a lanzar la subagente: sin herramientas, no podía actuar. El mensaje agrupa sus entradas por lo que salió mal:

* **Unrecognized**: la entrada no coincide con ningún nombre de herramienta, generalmente un error tipográfico como `Grpe` para `Grep`.
* **Not available to subagents**: la entrada nombra una herramienta real que [las subagentes no pueden usar](/docs/es/sub-agents#available-tools). Las subagentes de fondo mantienen un conjunto de herramientas integradas más pequeño, por lo que una entrada que solo una subagente en primer plano puede usar termina aquí cuando la subagente se ejecutaría en segundo plano, que es lo predeterminado. Si enumera `Agent`, el mensaje lo reporta bajo el siguiente grupo en su lugar.
* **Matched no tools in this session**: la entrada es válida pero ninguna herramienta en la sesión actual coincide con ella en este momento, como `mcp__github__*` sin servidor MCP de GitHub conectado, o `Agent` para una subagente en el [límite de profundidad](/docs/es/sub-agents#let-subagents-spawn-their-own-subagents).

Omitir el campo `tools` nunca activa este rechazo. Si deja la lista `tools` vacía, o `disallowedTools` elimina cada entrada en ella, Claude Code también omite el rechazo y lanza la subagente sin herramientas.

Antes de v2.1.208, la subagente se lanzaba sin herramientas y podía devolver un resultado vacío o confuso.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**Qué hacer:**

* Corrija cada entrada que el error nombra contra las [herramientas disponibles para subagentes](/docs/es/sub-agents#available-tools)
* Elimine entradas para herramientas que la sesión no tiene, como herramientas MCP de un servidor que no está conectado
* Para una herramienta que [las subagentes de fondo descartan](/docs/es/sub-agents#available-tools), como `CronCreate`, elimine la entrada. Para mantener la herramienta, [desactive el modo fork](/docs/es/sub-agents#turn-fork-mode-on-or-off) y pida a Claude que ejecute la subagente en primer plano
* Elimine el campo `tools` en lugar de enumerar herramientas para dar a la subagente cada [herramienta disponible para subagentes](/docs/es/sub-agents#available-tools)
* Para una lista `tools` que contiene solo `Agent`, aumente el [límite de profundidad](/docs/es/sub-agents#let-subagents-spawn-their-own-subagents) o dé al agente al menos otra herramienta: Claude Code retiene `Agent` en ese límite, por lo que una lista sin nada más en ella se resuelve a ninguna herramienta

<h3 id="file-is-covered-by-a-read-deny-rule">
  File is covered by a Read deny rule
</h3>

La herramienta Edit o Write fue llamada en una ruta coincidida por una [regla de denegación `Read`](/docs/es/permissions#read-and-edit), incluyendo la creación de un nuevo archivo en esa ruta. Ambas herramientas cambian contenido que Claude debe poder leer de nuevo, por lo que Claude Code se niega a la llamada antes de cualquier acceso a archivos. NotebookEdit no está cubierto por reglas de denegación `Read`. Antes de v2.1.228, la regla bloqueaba solo la herramienta Edit, y antes de v2.1.208, solo una regla de denegación `Edit` bloqueaba ediciones.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

Cuando Claude Code se niega a la herramienta Write, el mensaje termina con `and cannot be written` en su lugar.

**Qué hacer:**

* Si Claude debe poder cambiar el archivo, elimine o reduzca la regla de denegación `Read` en `/permissions` o en [configuración](/docs/es/settings-reference#permission-settings)
* Si el archivo debe permanecer intacto, mantenga la regla y agregue una regla de denegación `Edit` para la misma ruta para bloquear también la herramienta NotebookEdit

<h3 id="subagent-type-is-required">
  subagent\_type is required
</h3>

```text theme={null}
subagent_type is required: the general-purpose agent is not available in this session. Available agents: ...
```

Claude llamó a la [herramienta Agent](/docs/es/tools-reference#agent-tool-behavior) sin un `subagent_type`, y esta sesión no tiene [subagente de propósito general](/docs/es/sub-agents#built-in-subagents) como alternativa. Ese es el caso en dos configuraciones:

* [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/es/env-vars) está configurado en modo no interactivo, que elimina cada subagente integrada
* El agente de hilo principal de la sesión tiene una [lista de permisos `tools: Agent(...)`](/docs/es/sub-agents#restrict-which-subagents-can-be-spawned) que deja fuera `general-purpose`

**Qué hacer:**

* Generalmente nada: el mensaje enumera las subagentes que la sesión sí tiene, por lo que Claude puede reintentar con una de ellas
* Si Claude sigue fallando, agregue `general-purpose` a la lista de permisos `tools: Agent(...)`, o desactive `CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS`

Antes de v2.1.235, la misma llamada fallaba con `Agent type 'general-purpose' not found`.

<h3 id="memory-index-is-over-its-read-limit">
  Memory index is over its read limit
</h3>

Claude escribió en el índice de [memoria automática](/docs/es/memory#auto-memory) `MEMORY.md` y lo dejó por encima de uno de sus límites de lectura: 200 líneas o 25KB. La escritura fue exitosa, pero solo las primeras 200 líneas o 25KB, lo que sea menor, se cargan al inicio de una sesión, por lo que todo lo que está más allá del límite se descarta cada vez que se lee el índice. Antes de v2.1.210, un índice que excedía el límite se truncaba silenciosamente en la siguiente carga sin señal de escritura.

```text theme={null}
Error: this write left the memory index at MEMORY.md at 214 lines, over its 200-line read limit. The write succeeded, but everything past the limit is silently dropped each time the index is loaded — entries at the end are already invisible to readers. Rewrite it to under 140 lines now: keep one line per entry, move detail into topic files, and merge or drop stale entries.
```

Solo el contenido que se carga cuenta hacia los límites. El frontmatter YAML y los comentarios HTML a nivel de bloque se eliminan antes de que se cargue el índice, por lo que se excluyen de la medición. Antes de v2.1.211, Claude Code medía el archivo sin procesar, y el frontmatter o los comentarios podrían activar este error incluso cuando el contenido cargado se ajustaba.

Claude Code entrega el error a Claude después de la escritura en lugar de imprimirlo como un banner en su terminal, por lo que puede notarlo solo en la transcripción.

Cuando la escritura de Claude acerca el archivo a un límite sin cruzarlo, Claude Code devuelve un recordatorio más suave para compactar el índice en lugar de este error.

**Qué hacer:**

* Permita que Claude reescriba `MEMORY.md`, o pídale que lo haga: mantenga una línea por entrada, mueva los detalles a archivos de tema y fusione o descarte entradas obsoletas
* Para recortar el índice usted mismo, consulte [Auditar y editar su memoria](/docs/es/memory#audit-and-edit-your-memory)

<h3 id="pkill-pattern-matches-the-claude-code-process">
  pkill pattern matches the Claude Code process
</h3>

Un comando `pkill` en una llamada de herramienta Bash usó un patrón, típicamente con `-f`, que coincide con el proceso Claude Code en sí, por lo que Claude Code se niega al comando en lugar de permitir que termine la sesión. Claude Code prueba el patrón con `pgrep` antes de ejecutar `pkill` y se niega cuando su propio ID de proceso está en el resultado. La verificación se ejecuta solo en Linux; en macOS, `pkill` se ejecuta sin modificaciones. Antes de v2.1.214, el comando se ejecutaba, y un patrón coincidente mataba la sesión Claude Code a mitad de turno.

```text theme={null}
pkill: refusing to run — this pattern matches the Claude CLI process (PID 12345). Narrow the pattern, or target your own children with `pkill -P $$ ...`.
```

El rechazo aparece en el resultado de la herramienta Bash en lugar de como un banner en su terminal, y Claude generalmente ajusta el comando por sí solo.

**Qué hacer:**

* Reduzca el patrón para que coincida solo con el proceso previsto, por ejemplo la ruta completa del binario de destino en lugar de una subcadena corta
* Para detener procesos iniciados por el shell actual, use `pkill -P $$` con el patrón, que limita la coincidencia a los procesos secundarios del shell

<h3 id="failed-to-write-to-a-teammate-inbox">
  Failed to write to a teammate's inbox
</h3>

Claude Code no pudo escribir un mensaje en el archivo de buzón de un compañero de equipo bajo `~/.claude/teams/{team-name}/inboxes/`, por lo que el destinatario no recibió nada. La escritura falla cuando Claude Code no puede crear o actualizar el archivo, por ejemplo porque el disco está lleno, el directorio no es escribible, o otro agente mantiene el bloqueo del buzón durante demasiado tiempo. Antes de v2.1.224, Claude Code reportaba el mensaje como enviado incluso cuando la escritura fallaba.

El error aparece en el resultado de la herramienta del agente remitente en lugar de como un banner en su terminal, y su texto le dice a Claude que intente de nuevo:

```text theme={null}
Failed to write to researcher's inbox — nothing was sent. Try again, or message the lead.
```

Los mensajes de protocolo de [equipo de agentes](/docs/es/agent-teams) estructurados fallan de la misma manera, y el error nombra el mensaje no entregado: cuando Claude Code no puede escribir una aprobación de plan, rechazo de plan, solicitud de apagado o rechazo de apagado, el error dice `Failed to write the <message> to <name>'s inbox — nothing was sent`. La `plan approval` en esa lista es la decisión del líder aprobando el plan de un compañero; el envío del plan del compañero es el mensaje separado `plan approval request`. Ese mensaje y otros dos mensajes de protocolo llevan su propio texto de mensaje y consecuencia:

* `Failed to write the plan approval request to the lead's inbox — plan not submitted; try again`: el plan del compañero nunca llegó al líder, y el compañero permanece en modo de plan hasta que un reenvío sea exitoso
* `The permission request could not be delivered to the team lead (mailbox write failed)`: la solicitud de permiso del compañero nunca llegó al líder, por lo que nadie aprobó la llamada de herramienta
* `The confirmation could not be written to team-lead's inbox.`: la aprobación del apagado en sí tuvo efecto y el compañero sale; solo falta la confirmación al líder

Cuando usted mismo envía un mensaje a un compañero, escribiendo `@name` seguido del mensaje en la sesión del líder, la misma falla aparece como una notificación, `Couldn't write to @name's inbox — message not sent. Try again.`, y Claude Code mantiene su texto en el cuadro de solicitud para que pueda enviarlo de nuevo.

**Qué hacer:**

* Pida al remitente que reenvíe el mensaje; la contención por el bloqueo del buzón es transitoria y se resuelve al reintentar
* Verifique el espacio en disco libre y compruebe que `~/.claude/teams` y los archivos bajo él sean escribibles por su usuario

<h3 id="teammate-agent-definition-not-restored">
  Teammate's agent definition was not restored
</h3>

Claude envió un mensaje a un compañero de [equipo de agentes](/docs/es/agent-teams) detenido, y Claude Code lo reactivó sin volver a aplicar la [definición de subagente](/docs/es/agent-teams#use-subagent-definitions-for-teammates) desde la que fue generado, porque su archivo de definición provenía de una carpeta sin confianza guardada. El aviso sigue el informe de reanudación en el resultado de la herramienta del agente remitente:

```text wrap theme={null}
Its agent definition was not restored: the folder its definition file came from is not trusted (source: projectSettings), so the teammate is running with the team-essential tools and no custom instructions. To restore it, the user needs to run Claude Code in that folder once and accept the trust dialog (the --debug log names the folder); do not change trust settings on the user's behalf.
```

La verificación se aplica a una definición en el directorio `.claude/agents/` del proyecto o de un directorio `--add-dir`, y aceptar el diálogo de confianza para una carpeta principal no lo satisface.

**Qué hacer:**

* Ejecute `claude` en la carpeta que el [registro de depuración](/docs/es/debug-your-config) nombra y acepte el diálogo de confianza. La definición se vuelve a aplicar la próxima vez que Claude Code reactivar el compañero; no necesita reiniciar la sesión del líder
* O configure la entrada `hasTrustDialogAccepted` a `true` en `~/.claude.json`, usando la clave exacta `projects["<path>"]` que el registro de depuración imprime

<h3 id="message-too-large-for-cross-session-delivery">
  Message too large for cross-session delivery
</h3>

El [mensaje entre sesiones](/docs/es/cross-session-messaging) de Claude a otra de sus sesiones en esta máquina era demasiado largo para enviar. Claude Code se negó, y la sesión receptora no recibió nada. El rechazo aparece en el resultado de la herramienta de la sesión remitente, no como un banner en su terminal. Nombra ambos tamaños y cómo hacer que el mensaje se ajuste:

```text wrap theme={null}
Failed to send to api-worker: Message too large for cross-session delivery: the serialized message is 1,203,844 characters and the limit is 1,048,576. Shorten the message text — put bulk content in a file the recipient can read rather than in the message — or split it into smaller messages.
```

Reenviar el mismo texto falla de la misma manera.

**Qué hacer:**

* Pida a Claude que resuma el mensaje, o que ponga el contenido voluminoso en un archivo y envíe la ruta del archivo
* Pida a Claude que divida el contenido en varios mensajes más cortos

Antes de v2.1.235, Claude Code reportaba un mensaje de tamaño excesivo como enviado. La sesión receptora lo descartaba sin leer.

<h3 id="too-many-messages-to-this-session-just-now">
  Too many messages to this session just now
</h3>

Claude envió una ráfaga rápida de [mensajes entre sesiones](/docs/es/cross-session-messaging) a una de sus sesiones en esta máquina, y la ráfaga alcanzó lo que esa sesión de bandeja de entrada acepta. Claude Code se negó al siguiente envío, y la sesión receptora no recibió nada de él. El rechazo aparece en el resultado de la herramienta de la sesión remitente, no como un banner en su terminal:

```text wrap theme={null}
Failed to send to api-worker: Too many messages to this session just now: 30 were sent recently and more would be dropped by its rate limit, so this one was not sent. Batch what remains into one message, or wait a little before sending more.
```

**Qué hacer:**

* Generalmente nada: Claude agrupa el contenido restante en un mensaje, o espera antes de enviar más
* Si usted mismo solicitó la ráfaga, pida a Claude que combine lo que queda en un único mensaje

Antes de v2.1.236, Claude Code reportaba estos envíos como enviados. La sesión receptora los descartaba sin leer.

<h3 id="refusing-to-send-a-cross-session-message">
  Refusing to send a cross-session message
</h3>

Antes de que Claude Code escriba un [mensaje entre sesiones](/docs/es/cross-session-messaging) a otra de sus sesiones en esta máquina, verifica que el socket de bandeja de entrada de la sesión de destino sea el punto final al que se dirigió el mensaje. Cuando una verificación falla, Claude Code se niega al envío en la sesión remitente, y la sesión de destino no recibe nada. Para un mensaje que Claude envía, el rechazo aparece en el resultado de la herramienta de la sesión remitente:

```text theme={null}
Failed to send to api-worker: Refusing to send: reply target is a symlink
```

El texto después de `Refusing to send:` nombra la verificación que falló:

* `reply target is a symlink`: un enlace simbólico se encuentra en la ruta del socket de la sesión de destino. Claude Code no entrega a través de él, porque un enlace allí podría redirigir el mensaje a un punto final que la sesión de destino no creó.
* `cannot vet reply target`: Claude Code no pudo inspeccionar la ruta de destino en absoluto, por ejemplo porque leerla falló con un error de permiso.
* `connected endpoint is not the expected process`: el proceso que mantiene el socket no es la sesión a la que se dirigió el mensaje, por lo que la dirección es obsoleta u otro proceso reemplazó el socket.
* `connected endpoint identity could not be read`: Claude Code se conectó pero no pudo leer qué proceso mantiene el otro extremo, por lo que no pudo confirmar el destino. Esto puede ser transitorio.
* `connected endpoint is not owned by this user`: el proceso que mantiene el socket se ejecuta bajo una cuenta de usuario diferente, por lo que no es una de sus sesiones.
* `connected endpoint owner could not be read`: Claude Code se conectó pero no pudo leer qué cuenta de usuario posee el otro extremo, por lo que no pudo confirmar que el punto final sea suyo.
* `connected endpoint is a different process with the expected pid`: el ID de proceso coincide con el de la sesión a la que se dirigió el mensaje, pero Claude Code no pudo confirmar que sea el mismo proceso. Generalmente esa sesión salió y el sistema operativo reutilizó su ID de proceso, por lo que la dirección es obsoleta.

**Qué hacer:**

* Generalmente nada: las verificaciones evitan que un mensaje llegue a un punto final que no sea la sesión a la que se dirigió, y nada fue enviado
* Pida a Claude que enumere sus sesiones de nuevo y reenvíe; un rechazo causado por una dirección obsoleta se resuelve una vez que Claude envía a la actual
* Si `reply target is a symlink` se repite para una sesión, verifique qué creó un enlace en la ruta del socket de esa sesión, que se muestra en su `/status` bajo `Peer address`
* Para `connected endpoint identity could not be read`, reenvíe; la condición puede ser transitoria
* Si `connected endpoint is not owned by this user` aparece en una máquina compartida, la sesión en esa dirección se ejecuta bajo la cuenta de usuario de otro, por lo que Claude no puede enviarle mensajes desde la suya

Antes de v2.1.248, Claude Code no verificaba el usuario propietario del punto final ni la hora de inicio del proceso, por lo que los rechazos que nombran esas verificaciones no aparecen en versiones anteriores.

<h3 id="refusing-after-a-symlink-changed">
  Refusing to read, write, or search a path
</h3>

Claude Code verifica las [reglas de permiso](/docs/es/permissions#read-and-edit) de una ruta de archivo, luego confirma esa resolución nuevamente cuando la herramienta abre el archivo o inicia la búsqueda. Cuando no puede confirmar que la ruta aún conduce a la ubicación que la verificación aprobó, Claude Code se niega a la operación en lugar de seguirla. El rechazo aparece en el resultado de la herramienta:

```text wrap theme={null}
Refusing to read /path/to/file: its symlink resolution changed after permission was checked (a link on the way now leads somewhere the check did not see). If a link in the working directory is being rewritten concurrently, stop that and retry.
```

Cada rechazo nombra su razón:

* `its symlink resolution changed after permission was checked`: un enlace simbólico a lo largo de la ruta, o en una raíz de búsqueda Grep o Glob, fue reemplazado entre la verificación de permiso y la operación. En un rechazo de lectura, la frase entre paréntesis nombra qué comparación falló.
* `its parent-directory symlink resolution changed after permission was checked`: un directorio por el que pasa la ruta de escritura ya no se resuelve a la ubicación aprobada
* `it is a symbolic link. Write to the link's target path instead`: un enlace simbólico se encuentra en la ubicación de escritura aprobada en sí, por ejemplo un `CLAUDE.md` que es un enlace simbólico a `AGENTS.md`; el mensaje dirige a Claude a la ruta de destino del enlace
* `Refusing to write through symlink: <path>. Resolve the symlink and pass the real target path explicitly.`: la misma condición detectada cuando otro escritor abre el archivo, como una escritura a un `.mcp.json` enlazado simbólicamente
* `Refusing to write into symlinked directory: <path>`: el directorio que contiene el archivo es en sí mismo un enlace simbólico, por ejemplo el directorio `.claude/` de un proyecto vinculado a otra ubicación
* `a path one of its Read deny rules is written through changed while the search was being prepared. Retry.`: una regla de denegación `Read` para la búsqueda nombra una ruta que pasa a través de un enlace simbólico, y ese enlace cambió mientras Claude Code estaba preparando la búsqueda
* `it could not be opened (EACCES) — it is unreadable, or is being replaced concurrently.`: la raíz de búsqueda existe pero no pudo abrirse; el código entre paréntesis es el error del sistema operativo
* `its permission check expired before it ran (too many concurrent file operations). Retry.`: Claude Code desalojó el registro de aprobación bajo muchas operaciones de archivo simultáneas antes de que la herramienta lo usara; reintentar ejecuta una verificación de permiso nueva
* `ripgrep was found only by name on PATH, and a search outside the working directory cannot apply your Read deny rules in that configuration`: Claude Code no pudo resolver el binario `rg` a una ruta absoluta, por lo que se niega a búsquedas fuera del directorio de trabajo en lugar de ejecutar una que sus reglas de denegación no cubran

**Qué hacer:**

* Generalmente nada: el rechazo llega a Claude como el resultado de la herramienta, y la operación rechazada no se ejecuta
* Si un rechazo de enlace simbólico se repite en una ruta, encuentre qué sigue reescribiendo un enlace allí, como una herramienta de compilación o un observador de archivos, o pida a Claude que use la ruta resuelta del archivo en lugar de la vinculada
* Si este rechazo aparece para cada archivo mientras Claude Code se ejecuta en Windows dentro de un AppContainer o sandbox de token restringido, actualice a v2.1.265 o posterior
* Si un rechazo de lectura aparece en macOS para un archivo que nada está reescribiendo, como una captura de pantalla arrastrada al mensaje, actualice a v2.1.273 o posterior
* Para el rechazo de ripgrep, instale ripgrep con su administrador de paquetes para que `rg` se resuelva a una ruta absoluta en `PATH`, o mantenga búsquedas bajo el directorio de trabajo

Antes de v2.1.251, Claude Code volvía a verificar la resolución de una ruta solo para escrituras de archivo, por lo que un enlace reemplazado después de la verificación de permiso podría redirigir una lectura o búsqueda a una ubicación diferente sin un mensaje. De estos rechazos, solo el rechazo de escritura del directorio principal, a través de enlace simbólico y directorio enlazado simbólicamente aparecen en versiones anteriores.

<h3 id="task-output-swap-refused">
  Task output swap refused
</h3>

Claude Code guarda la salida de cada comando Bash en un archivo bajo su directorio temporal. Cada vez que abre uno de estos archivos, verifica que la ruta aún conduce al archivo que creó, sin enlace simbólico, enlace duro adicional o directorio movido redirigiendo. Este mensaje significa que esa verificación falló, por lo que Claude Code se negó a la operación en lugar de escribir o leer salida a través de esa ruta. El mensaje aparece en el resultado de la herramienta Bash:

```text wrap theme={null}
task output swap refused (tasks dir moved or linked): /private/tmp/claude-501/-Users-you-my-project/1f0e62dc-4b0a-4f5e-9c2d-8a7b6c5d4e3f/tasks/b7k2f9m3q.output. To recover: restart Claude Code with CLAUDE_CODE_TMPDIR set to a fresh directory; or, if /private/tmp/claude-501/-Users-you-my-project is a stray directory or a symbolic link that should not be there, remove that entry itself (not what it points to) and restart.
```

El texto entre paréntesis nombra la verificación que falló. Razones como `output symlink was re-pointed`, `output file identity changed`, y `not a regular file` todas reportan la misma condición: algo en o a lo largo de la ruta de salida ya no es el archivo que Claude Code creó. Solo algunas razones llevan una oración `To recover:`.

Si la verificación falla mientras un comando aún se está ejecutando, Claude Code detiene el comando, y su resultado reporta:

```text theme={null}
Command killed: its output file was replaced or could no longer be verified
```

**Qué hacer:**

* Actualice a v2.1.260 o posterior. Las versiones anteriores a veces mostraban este mensaje cuando no había enlace o directorio movido presente
* Reinicie Claude Code con [`CLAUDE_CODE_TMPDIR`](/docs/es/env-vars) configurado en un directorio nuevo
* O verifique el directorio de su proyecto bajo el directorio temporal de Claude Code, `/private/tmp/claude-501/-Users-you-my-project` en el mensaje de ejemplo. Si esa ruta es un enlace simbólico, o un directorio que no debería estar allí, elimine el enlace o directorio en sí en lugar del destino del enlace, y reinicie Claude Code
* Si el rechazo se repite, un proceso está reemplazando, vinculando o eliminando entradas bajo el directorio temporal de Claude Code mientras la sesión se ejecuta. Configure [`CLAUDE_CODE_TMPDIR`](/docs/es/env-vars) en un directorio que nada más administre y reinicie

<h3 id="the-source-file-is-not-valid-utf-8-text">
  The source file is not valid UTF-8 text
</h3>

Claude intentó publicar un [artefacto](/docs/es/artifacts) desde un archivo cuyos bytes no se decodifican como texto, o cuyo texto ya contiene el carácter de reemplazo `U+FFFD`, por lo que Claude Code se negó a publicar antes de cargar nada. El mensaje aparece en el resultado de la herramienta Artifact y nombra la primera posición a corregir:

```text wrap theme={null}
file_path: the source file is not valid UTF-8 text (first invalid byte at line 12, column 40). It may be saved in another encoding or contain binary data. Rewrite it as UTF-8, then publish again. Nothing was published.

file_path: the source file has the replacement character U+FFFD at line 12, column 40, usually left where an earlier edit or paste lost a character. Replace it with the intended text (in HTML, write an intended U+FFFD as &#xFFFD;), then publish again. Nothing was published.
```

Claude Code decodifica el archivo como UTF-8, o como UTF-16 cuando comienza con una marca de orden de bytes UTF-16 little-endian. Cuando tal archivo UTF-16 no se decodifica, el primer mensaje nombra `UTF-16` y aún le dice que reescriba el archivo como UTF-8. Cuando más posiciones siguen la nombrada, el mensaje agrega un conteo como `(+2 more)` después de la posición.

**Qué hacer:**

* Generalmente nada: Claude reescribe el archivo y publica de nuevo
* Si el archivo es uno que escribió o exportó, guárdelo de nuevo como UTF-8, y reemplace cada `U+FFFD` con el carácter que una edición, pegado o conversión anterior perdió
* Para mostrar un `U+FFFD` intencional en la página, escríbalo como `&#xFFFD;` en el HTML en lugar del carácter literal

Antes de v2.1.267, Claude Code cargaba tal archivo sin verificarlo, y el servidor se negaba a publicar en su lugar.

<h3 id="reading-a-local-file-from-outside-the-connected-folders">
  Reading a local file from outside the connected folders in a Cowork session
</h3>

En una sesión de [Cowork](https://claude.com/docs/cowork/overview) ejecutándose en su máquina en la aplicación Claude Desktop, Claude nombró un archivo local para un [artefacto](/docs/es/artifacts). Claude Code no pudo confirmar que el archivo es un archivo simple dentro de las carpetas conectadas de la sesión: la ruta se encuentra fuera de esas carpetas, pasa a través de un enlace simbólico, o está escrita de una manera que puede nombrar un archivo diferente al que parece. Leer tal archivo necesita su aprobación, y en una sesión que no puede mostrarle la tarjeta de aprobación, como una configurada para omitir todas las aprobaciones, Claude Code se niega a la lectura.

El rechazo aparece en el resultado de la herramienta Artifact; cuando el archivo no pudo ser examinado en absoluto, nombra esa falla en su lugar:

```text wrap theme={null}
Reading a local file from outside this session's connected folders, or through a link, needs the approval card, and no one can answer it in this Cowork session. Use a plain file inside the connected folders; do not retry this file in this session.

cannot read file_path (ENOENT) — the file could not be examined, and no one can answer the approval card in this Cowork session. Check that the file exists as a plain file inside the connected folders, then retry with that path.
```

**Qué hacer:**

* Generalmente nada: el mensaje le dice a Claude que use un archivo simple dentro de las carpetas conectadas en su lugar
* Para poner ese archivo exacto en el artefacto, cópielo en una de las carpetas conectadas de la sesión como un archivo regular, no un enlace simbólico, y pregunte de nuevo

<h3 id="webfetch-cannot-fetch-localhost">
  WebFetch cannot fetch localhost
</h3>

Claude llamó a [WebFetch](/docs/es/tools-reference#webfetch-tool-behavior) con una URL cuyo nombre de host no tiene punto, como `http://localhost:3000` o un nombre de intranet simple como `http://wiki/`. WebFetch se niega a estas URL antes de hacer cualquier solicitud:

```text wrap theme={null}
WebFetch cannot fetch localhost or other hostnames without a dot. To reach a local server, use Bash with curl instead.
```

**Qué hacer:**

* Generalmente nada: el mensaje señala a Claude hacia `curl` a través de la herramienta Bash, que puede alcanzar servidores locales e intranet

Antes de v2.1.268, WebFetch reportaba estas URL con un error genérico `Invalid URL`.

<h2 id="background-session-errors">
  Errores de sesión en segundo plano
</h2>

Las [sesiones en segundo plano](/docs/es/agent-view) se ejecutan sin su propia terminal interactiva, por lo que los comandos que necesitan una se comportan de manera diferente allí. Estos mensajes aparecen en la transcripción de una sesión en segundo plano, en la terminal que se adjunta a una, en la sesión o shell desde la que se envía, o, para las [entradas de worktree-guard](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) a continuación, en cualquier sesión aislada en un worktree o ejecutando un subagente aislado en worktree; cuando un mensaje es específico de una superficie, su entrada lo indica.

<h3 id="commands-refused-in-a-background-session">
  Comandos rechazados en una sesión en segundo plano
</h3>

Los comandos que abren un diálogo interactivo no pueden hacerlo mientras no hay terminal adjunta a una sesión en segundo plano. `/install-github-app`, la lista de configuración `/mcp`, y las acciones de autenticación en el menú del servidor MCP responden con un mensaje, y la sesión aparece bajo **Needs input** en [agent view](/docs/es/agent-view) para que pueda encontrarla, adjuntarla y ejecutar el comando nuevamente. Mientras una terminal está adjunta, estos comandos funcionan normalmente.

Antes de v2.1.216, la sesión no aparecía bajo **Needs input** después de uno de estos rechazos. En v2.1.213 a v2.1.215, los comandos aún funcionaban mientras una terminal estaba adjunta, y el mensaje de rechazo le indicaba que adjuntara y ejecutara el comando nuevamente. De v2.1.208 a v2.1.212, Claude Code los rechazaba incluso mientras una terminal estaba adjunta, con un mensaje como `Can't open MCP settings in a background session`; en esas versiones, ejecute el comando desde una sesión `claude` regular en su lugar, o actualice. Antes de v2.1.208, abrían su diálogo dentro de la sesión en segundo plano. En v2.1.208 solamente, Claude Code también rechazó el selector `/model` en una sesión en segundo plano, y `/upgrade` imprimió la URL de actualización en lugar de abrir un navegador.

La redacción nombra el comando. La lista de configuración `/mcp` informa:

```text theme={null}
Can't open MCP settings while no terminal is attached to this background session. This session now shows "needs input" in agent view — open it and run /mcp to manage servers, or use `/mcp enable|disable|reconnect <server>` to steer without the panel.
```

**Qué hacer:**

* Adjunte a la sesión desde agent view, donde aparece en **Needs input**, y ejecute el comando nuevamente
* O use la forma que el mensaje nombra, como `/mcp reconnect <server>`, `/mcp enable`, o `/mcp disable`, que funcionan sin adjuntar

<h3 id="write-or-command-blocked-because-the-path-cannot-be-safely-resolved">
  Escritura o comando bloqueado porque la ruta no se puede resolver de forma segura
</h3>

Claude dirigió un archivo o directorio de trabajo a través de una ortografía que el [worktree-isolation guard](/docs/es/agent-view#how-file-edits-are-isolated) no puede resolver a una ubicación verificable. El guard verifica escrituras y directorios de trabajo de comandos en [cualquier sesión aislada en un worktree](/docs/es/worktrees#how-claude-code-enforces-isolation), interactiva o en segundo plano, y en [subagentes aislados en worktree](/docs/es/worktrees#isolate-subagents-with-worktrees). Resuelve enlaces simbólicos antes de verificar que la operación no llegue al checkout compartido, y cuando la resolución falla, bloquea la operación en lugar de permitir que llegue allí. El mensaje nombra las formas de ruta que rechaza y cómo reintentar:

```text theme={null}
This write was blocked because the path is spelled in a form that cannot be safely resolved (for example through a symlink storing a raw dot segment, a network-share or device-namespace shape, or an unreadable ancestor directory). If the file is inside the worktree /path/to/worktree, address it by its direct symlink-free path instead.
```

Un comando bloqueado informa la misma causa para su directorio de trabajo y termina con `re-run the command from its direct symlink-free path`. Antes de v2.1.217, el guard comparaba ortografías de ruta sin resolver enlaces simbólicos, por lo que estas ortografías no se bloqueaban y una escritura enrutada a través de un enlace simbólico podría llegar al checkout compartido.

**Qué hacer:**

* Generalmente nada: el mensaje completo va a Claude como un error de herramienta, y Claude reintenta con la ruta directa que nombra. Para una edición de archivo bloqueada, la vista de conversación muestra solo una línea corta `Error editing file`; el mensaje completo aparece en la vista de transcripción, que abre con `Ctrl+O`. Un comando bloqueado lo imprime en su salida de comando.
* Si el bloqueo se repite en el mismo archivo, la ruta probablemente se ejecuta a través de un enlace simbólico confirmado cuyo destino contiene `..`, como `docs/current -> ../README.md`; pida a Claude que edite el archivo de destino por su ruta real en lugar de a través del enlace

<h3 id="write-or-command-blocked-because-the-path-names-a-network-location">
  Escritura o comando bloqueado porque la ruta nombra una ubicación de red
</h3>

Claude dirigió un archivo o directorio de trabajo a través de una ruta que nombra una unidad que no está en su máquina, un recurso compartido UNC como `\\server\share\file` o una ruta de automontaje `/net`, mientras que el checkout de la sesión está en un disco local. El mismo [worktree-isolation guard](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) no puede verificar que tal ruta se mantenga fuera del checkout compartido, por lo que bloquea la operación. Aislar la sesión en un worktree no levanta el bloqueo. El mensaje nombra la forma de ruta a usar en su lugar:

```text theme={null}
This write was blocked because the path is network-shaped (a UNC share or /net automount spelling) while this session's checkout is local. Isolating cannot unblock it. If the file is genuinely inside the worktree /path/to/worktree, address it by its local, plainly-spelled path instead.
```

Un comando bloqueado informa la misma causa para su directorio de trabajo y termina con `re-run the command from its local, plainly-spelled path`. Antes de v2.1.217, el guard comparaba solo texto de ruta, por lo que dirigir un archivo dentro del checkout a través de una ruta UNC o `/net` no se bloqueaba.

**Qué hacer:**

* Generalmente nada: Claude reintenta con la ortografía local que el mensaje solicita
* Si el archivo está en un recurso compartido de red en lugar de un archivo local deletreado con una ruta de red, está fuera del espacio de trabajo local de la sesión; edítelo desde una sesión interactiva regular en su lugar

<h3 id="command-blocked-by-the-worktree-isolation-checks">
  Comando bloqueado por las comprobaciones de aislamiento de worktree
</h3>

Claude ejecutó un comando Bash o Monitor en una [sesión aislada en un worktree](/docs/es/worktrees#how-claude-code-enforces-isolation), y Claude Code lo rechazó por una de dos razones:

* El comando apunta git al checkout principal.
* Claude Code no puede verificar a partir del texto del comando que cualquier git que ejecute el comando permanezca dentro del worktree. Un comando que nunca nombra git aún puede ser rechazado por esta razón, porque expandir una indirección de variable como `${!name}` o ejecutar una sustitución de función Bash como `${ command; }` produce un valor en tiempo de ejecución que puede ser un comando en sí.

El medio del mensaje nombra lo que no se pudo verificar:

```text wrap theme={null}
This session is isolated in the worktree /path/to/worktree, but this command evaluates ${!x@P} arithmetically inside a construct too complex to verify, which can run a command hidden in a variable's value. Refusing to run it — a worktree-isolated session's git operations must target its own worktree. Split it into plain, separate commands and run them from /path/to/worktree.
```

**Qué hacer:**

* Generalmente nada: Claude lee el mensaje y reescribe el comando de la manera que su oración final solicita
* Si un comando que solicitó sigue siendo rechazado, deletree el valor marcado literalmente: reemplace la indirección o sustitución con su valor, y ejecute git como su propio comando simple desde dentro del worktree
* Para actuar en el checkout principal a propósito, ejecute el comando usted mismo en una terminal fuera de la sesión

<h3 id="this-session-has-no-saved-transcript">
  Esta sesión no tiene transcripción guardada
</h3>

Se adjuntó a una [sesión en segundo plano](/docs/es/agent-view) detenida que fue enviada a segundo plano desde otra conversación con `←` o `/background` y se detuvo antes de que su primera respuesta terminara. Hasta que esa primera respuesta termine, la conversación aún vive solo en la sesión desde la que fue enviada a segundo plano, por lo que `claude attach` rechaza iniciar la sesión detenida en lugar de comenzar una conversación en blanco bajo el mismo ID de sesión. El mensaje termina con el comando `claude respawn` para esta sesión:

```text theme={null}
This session has no saved transcript — it was stopped before its first response finished. If it was backgrounded from another conversation, that one is still intact; `claude respawn <id>` starts this one fresh.
```

Abrir la misma fila de sesión en [agent view](/docs/es/agent-view) muestra `Press enter again to restart this session fresh` debajo de la lista en su lugar, y un segundo `Enter` en la fila reinicia la sesión con una conversación vacía. Antes de v2.1.212, abrir la fila mostró el mensaje de rechazo sin forma de reiniciar desde agent view. Antes de v2.1.211, abrir la sesión detenida iniciaba silenciosamente esa conversación en blanco y podría volver a ejecutar el prompt original de la sesión.

**Qué hacer:**

* La conversación que envió a segundo plano está intacta: reanúdela con [`claude --resume`](/docs/es/sessions) o continúe trabajando en ella
* Para iniciar la sesión detenida de nuevo, ejecute `claude respawn <id>` con el ID del mensaje, o presione `Enter` dos veces en su fila en agent view
* Si la sesión terminó una respuesta y aún ve este rechazo en una versión anterior a v2.1.214, una carpeta ilegible en `~/.claude/projects` podría hacer que el escaneo de transcripción pierda la conversación guardada; actualice a v2.1.214 o posterior, que tolera carpetas ilegibles durante el escaneo

<h3 id="this-session-is-running-in-another-terminal">
  Esta sesión se está ejecutando en otra terminal
</h3>

Abrió la fila de una sesión detenida en [agent view](/docs/es/agent-view), y su conversación guardada ya está abierta en otro proceso Claude Code activo en esta máquina, por lo que Claude Code rechaza iniciar un segundo proceso que escribiría en la misma transcripción. Qué mensaje ve depende de [qué mantiene la conversación](/docs/es/agent-view#opening-a-session-says-the-conversation-is-already-open):

```text theme={null}
Can't open — this session is running in another terminal
This conversation is already open in another running Claude session — use that one, or close it and try again
```

* **`running in another terminal`**: una terminal mantiene la conversación, por ejemplo una donde la reanudó con `claude --resume` o `/resume`. La fila también muestra `Open in a terminal`.
* **`already open in another running Claude session`**: otro proceso Claude Code no interactivo la mantiene, por ejemplo un proceso de [sesión en segundo plano](/docs/es/agent-view#the-supervisor-process) para la misma conversación que aún no ha salido.

Claude Code guarda una respuesta que escribió al abrir la fila y la envía como el siguiente prompt de la sesión cuando la sesión se inicia nuevamente.

**Qué hacer:**

* Continúe la conversación en el proceso que la tiene abierta, o salga de ese proceso y abra la fila nuevamente

Antes de v2.1.248, solo existía el rechazo `already open in another running Claude session`: una conversación reanudada en una terminal no contaba como abierta, y abrir la fila iniciaba un segundo proceso Claude Code escribiendo en la misma conversación.

<h3 id="this-sessions-saved-conversation-is-no-longer-on-disk">
  La conversación guardada de esta sesión ya no está en el disco
</h3>

Abrió una [sesión en segundo plano](/docs/es/agent-view) que terminó mientras el servicio en segundo plano estaba apagado, y [transcript cleanup](/docs/es/settings-reference#cleanupperioddays) ha eliminado desde entonces su conversación guardada, por ejemplo después de que la máquina estuvo apagada durante semanas. Abrir tal fila normalmente [reanuda su conversación guardada](/docs/es/agent-view#sessions-show-as-failed-after-shutdown). Sin nada que reanudar, Claude Code rechaza en lugar de volver a ejecutar el prompt original de la sesión sin preguntar:

```text theme={null}
This session's saved conversation is no longer on disk (it ended while the background service was off, and old transcripts are cleaned up), so there is nothing to resume. `claude rm 7c5dcf5d` deletes the row; `claude respawn 7c5dcf5d` runs its original prompt again instead.
```

`claude attach <id>` imprime este texto. En agent view, el pie de página es más corto y termina con `ctrl+x deletes the row`.

**Qué hacer:**

* Ejecute `claude rm <id>` para eliminar la fila. Cuando se aplica uno de los [casos mantenidos](/docs/es/agent-view#what-deleting-a-session-removes), `claude rm` mantiene la fila y el worktree en su lugar e indica la razón
* Para ejecutar el prompt original de la sesión nuevamente como una conversación nueva, ejecute `claude respawn <id>`

Antes de v2.1.248, abrir tal fila volvía a ejecutar el prompt original de la sesión en lugar de rechazar, trayendo una tarea de hace semanas al primer plano.

<h3 id="worktree-has-commits-that-are-not-pushed-anywhere">
  Worktree tiene commits que no se han enviado a ningún lugar
</h3>

Intentó eliminar una [sesión en segundo plano](/docs/es/agent-view#what-deleting-a-session-removes) cuyo worktree contiene commits que Claude Code no puede confirmar que se guardan en otro lugar. Claude Code mantiene el worktree y la fila de sesión en lugar de destruir los commits sin verlos. `claude rm` nombra la rama y los commits no enviados, y dice cómo proceder:

```text theme={null}
kept 7c5dcf5d — its worktree is still at "/home/you/project/.claude/worktrees/fix-login"
  2 unpushed commits on "claude/fix-login": a1b2c3d "Fix login flow" and 1 more. They exist on no remote, so deleting the worktree would lose them.
  push them and run 'claude rm 7c5dcf5d' again, or discard the worktree and its commits: claude rm 7c5dcf5d --discard-unpushed a1b2c3d000000000000000000000000000000000@0123456789abcdef0123456789abcdef
```

Cuando Claude Code no puede resumir los commits, la línea de detalle dice `The worktree has unpushed commits` en su lugar. En [agent view](/docs/es/agent-view), la fila de la sesión muestra `not deleted` con la misma razón.

Los commits en un remoto no bloquean la eliminación. Tampoco lo hacen los commits en la copia local de la rama predeterminada de su remoto `origin`, siempre que esa rama esté desprotegida en su checkout principal, el directorio del repositorio en sí en lugar de un worktree.

**Qué hacer:**

* Para mantener los commits, envíe la rama del worktree, o mérjela en la rama predeterminada desprotegida en su checkout principal, luego elimine la sesión nuevamente
* Para descartar los commits, ejecute el comando `claude rm <id> --discard-unpushed` que el mensaje imprimió, o presione `Ctrl+X` dos veces en la fila de la sesión en agent view nuevamente. Esto elimina la sesión y el worktree junto con su rama, los commits no enviados y cualquier cambio no confirmado. Si el worktree ha ganado un commit desde el rechazo, Claude Code lo mantiene nuevamente y muestra el estado actualizado
* Cuando el mensaje dice que el worktree también está registrado por otra sesión terminada, eliminar nuevamente no lo descarta: envíe los commits, luego elimine la sesión nuevamente

Antes de v2.1.268, `claude rm` ponía el resumen de commits en la línea `kept` en sí. Cuando `claude rm` no podía resumir los commits, la línea `kept` decía `worktree has commits that are not pushed anywhere` en lugar del resumen.

Antes de v2.1.260, el mensaje no nombraba la rama ni los commits, y eliminar nuevamente se rechazaba de la misma manera: eliminar la sesión sin enviar significaba eliminar el worktree usted mismo con `git worktree remove --force <path>`, luego ejecutar `claude rm <id>` nuevamente.

Antes de v2.1.248, la rama predeterminada desprotegida en su checkout principal no contaba: una rama que ya había fusionado allí aún activaba este rechazo hasta que sus commits llegaran a un remoto.

<h3 id="terminal-host-process-died">
  El proceso host de terminal murió
</h3>

Cada terminal de [sesión en segundo plano](/docs/es/agent-view) se ejecuta en un proceso host bajo el servicio en segundo plano, y ese proceso murió mientras el servicio aún mantenía su conexión, por lo que no se pudo acceder a la sesión.

En Linux y WSL, el servicio en segundo plano verifica cada proceso host cada pocos segundos, marca la sesión como fallida cuando el proceso ha salido pero su conexión al servicio nunca se cerró, y muestra la razón en su fila en [agent view](/docs/es/agent-view#read-session-state):

```text theme={null}
terminal host process died — press Enter to restart
```

Si abre la fila antes de que se ejecute la verificación, el pie de página muestra `This session's terminal host process died (the conversation is saved) — press Enter to restart it` y la fila se vuelve fallida.

Desde el shell, `claude attach <id>` reinicia una sesión ya marcada como fallida por un host muerto, y de lo contrario imprime la causa y sale:

```text theme={null}
Couldn't attach to <id> — This session's terminal host process died (the conversation is saved) — run `claude attach <id>` again to restart it on a fresh host.
```

La conversación se guarda de cualquier forma.

Una fila que ejecuta un [comando shell](/docs/es/agent-view#run-a-shell-command) en su lugar muestra `terminal host process died — its output is gone; the command was not run again`, y `claude attach` imprime `This command's terminal host process died — its output is gone and the command was not run again`. Claude Code nunca vuelve a ejecutar el comando por usted.

**Qué hacer:**

* En agent view, presione `Enter` en la fila fallida; la sesión se reinicia en un nuevo proceso host y la conversación se reanuda
* Desde el shell, ejecute `claude attach <id>` nuevamente. Claude Code imprime `Session <id>'s terminal host died — restarting it on a fresh one…` y reabre la sesión
* No puede reiniciar una fila de comando shell de esta manera; envíe el comando nuevamente para volver a ejecutarlo

Antes de v2.1.247, un proceso host muerto podría pasar cada verificación de vivacidad que ejecutaba el servicio en segundo plano, por lo que abrir la sesión mostraba `opening… · esc to cancel` indefinidamente y `claude attach <id>` esperaba sin reportar un error.

<h3 id="session-isnt-responding">
  La sesión no responde
</h3>

Abrió una [sesión en segundo plano](/docs/es/agent-view) y el servicio en segundo plano aceptó la apertura, pero no llegó salida durante aproximadamente diez segundos, por lo que Claude Code concluye que el proceso que retransmite la terminal de la sesión no puede entregar salida, y termina el intento en lugar de esperar.

En agent view, Claude Code ofrece un reinicio en el pie de página:

```text theme={null}
Press enter again to restart this session — it isn't responding (its conversation is saved and resumes).
```

Desde el shell, `claude attach <id>` imprime la causa y sale:

```text theme={null}
Couldn't attach to <id> — Session isn't responding — `claude stop <id>`, then `claude attach <id>` restarts it (the conversation is saved).
```

Claude Code nunca reinicia una fila que ejecuta un [comando shell](/docs/es/agent-view#run-a-shell-command) por usted, porque un reinicio volvería a ejecutar el comando.

**Qué hacer:**

* En agent view, presione `Enter` en la misma fila nuevamente. Claude Code detiene el proceso que no responde y reinicia la sesión, y la conversación se reanuda. Nada se detiene sin ese segundo presión
* Desde el shell, ejecute `claude stop <id>`, luego `claude attach <id>`
* Para una fila de comando shell, presione `Ctrl+X` en agent view o ejecute `claude stop <id>` para detenerla; envíe el comando nuevamente para volver a ejecutarlo

<h3 id="session-was-stopped-while-the-respawn-was-in-flight">
  La sesión se detuvo mientras el respawn estaba en vuelo
</h3>

Abrió una [sesión en segundo plano](/docs/es/agent-view) cuyo proceso no se estaba ejecutando, y mientras Claude Code la estaba reiniciando, otro proceso Claude Code la detuvo, por ejemplo `claude stop` en otra terminal. Claude Code mantiene la sesión detenida:

```text theme={null}
Session <id> was stopped while the respawn was in flight
```

Abrir una sesión que acaba de enviar, mientras su proceso aún se está iniciando, espera al proceso en su lugar. Antes de v2.1.246, abrirla en ese momento podría detenerla y mostrar este mensaje.

**Qué hacer:**

* Si no detuvo la sesión, abra su fila nuevamente en agent view o ejecute `claude respawn <id>` para reiniciarla
* Si la detuvo usted mismo, nada queda por hacer: la sesión permanece detenida

<h3 id="session-agent-no-longer-available">
  Agente de sesión ya no disponible
</h3>

Reanudó una sesión que estaba ejecutando un [agente personalizado](/docs/es/sub-agents#invoke-subagents-explicitly), iniciado con `--agent` o la configuración `agent`, y Claude Code no encontró un agente con ese nombre. Busca primero en el directorio original de la sesión, cuando ha [confiado en ese espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust), luego en el directorio desde el que reanuda. La sesión aún se reanuda, pero con las herramientas predeterminadas, por lo que las restricciones de herramientas del agente ya no se aplican:

```text theme={null}
This session was running agent 'code-reviewer', which is no longer available (no agent by that name in /home/you/project). Continuing with the default tools and system prompt — the agent's tool restrictions no longer apply. To restore it, re-create the agent, or resume with an explicit --agent <name>.
```

La advertencia nombra solo los directorios que Claude Code buscó, y aparece en la conversación reanudada ya sea que despierte una [sesión en segundo plano](/docs/es/agent-view), ejecute `/resume` o `claude --resume`, o reanude en [modo no interactivo](/docs/es/headless), donde también va a stderr. Las sesiones que usan `--input-format stream-json` no la muestran, porque el Agent SDK proporciona agentes después del inicio.

Claude Code no guarda la alternativa en la sesión, por lo que la advertencia se repite en cada reanudación hasta que actúe. El agente `claude` integrado no activa la advertencia, ya que volver a la herramienta predeterminada no cambia nada para él. Antes de v2.1.216, Claude Code continuaba silenciosamente como el agente predeterminado, y la búsqueda cubría solo el directorio desde el que reanudaba, por lo que un agente con alcance de proyecto se perdía en cualquier reanudación desde otro directorio.

**Qué hacer:**

* Vuelva a crear el archivo de agente en `.claude/agents/<name>.md` en el proyecto de la sesión, o en `~/.claude/agents/<name>.md` para un agente personal, luego reanude nuevamente
* O reanude con `--agent <name>` nombrando un agente que sí existe, para ejecutar la sesión como ese agente en su lugar
* Si el agente tiene alcance de proyecto y no ha confiado en el directorio original de la sesión, ejecute Claude Code allí una vez, acepte el diálogo de confianza, luego reanude nuevamente

<h3 id="claude_code_process_wrapper-launcher-errors">
  Errores del lanzador CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/es/corporate-launcher) está configurado, y su valor no se puede usar, por lo que Claude Code rechaza iniciar el proceso afectado en lugar de ejecutarlo sin el lanzador. Los problemas de configuración se informan con un mensaje que comienza con el nombre de la variable y establece la razón, por ejemplo:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Un lanzador que se inicia pero sale sin reemplazarse a sí mismo con Claude Code falla la sesión que estaba iniciando, y la fila de la sesión en agent view informa que el lanzador `must exec, not daemonize`, seguido de cualquier cosa que el lanzador imprimió. Una sesión que no puede iniciarse o alcanzar el servicio en segundo plano debido al lanzador informa el problema del lanzador como la razón dentro de `Couldn't reach the background service (...)`.

**Qué hacer:**

* Configure la variable a la ruta absoluta de un ejecutable que termine llamando a `exec "$@"`. Vea [el contrato del lanzador](/docs/es/corporate-launcher#the-launcher-contract) para el contrato completo
* Verifique `/status`, que muestra el comando de lanzamiento resuelto en su entrada Self-exec y advierte cuando el servicio en segundo plano en ejecución no coincide con él, o ejecute `claude daemon status` desde un shell
* Después de corregir el valor en el bloque `env` de [configuración](/docs/es/corporate-launcher#set-up-the-launcher), reinicie el servicio en segundo plano con `claude daemon stop --any` para que el siguiente envío inicie uno envuelto

<h3 id="eunknown-when-starting-a-background-session">
  EUNKNOWN al iniciar una sesión en segundo plano
</h3>

Windows rechazó iniciar un programa con un código de error que no tiene nombre estándar, por lo que el fallo aparece como `EUNKNOWN`. El disparador habitual es una política de restricción de software, como Group Policy o AppLocker, bloqueando el programa que se está iniciando. El error aparece cuando inicia una [sesión en segundo plano](/docs/es/agent-view) con `/background` o `claude --bg`:

```text theme={null}
Couldn't reach the background service (spawn background service: EUNKNOWN: unknown error, uv_spawn) — run 'claude daemon status'
```

En algunas cuentas el mensaje dice `daemon` en lugar de `background service`.

En una instalación npm, un `EUNKNOWN` que aparece mientras `npm install -g @anthropic-ai/claude-code` está reemplazando el binario tiene la misma causa que [`EACCES` durante una reinstalación](#eacces-when-starting-a-background-session) y se borra cuando reintenta después de que la instalación termina.

Claude Code inicia el servicio en segundo plano a través de PowerShell para que el servicio sobreviva al cierre de la terminal, usando PowerShell 7 cuando está instalado y Windows PowerShell 5.1 de lo contrario. Cuando ningún PowerShell puede ejecutarse, Claude Code inicia el servicio directamente en su lugar, por lo que una política que bloquea solo PowerShell no causa este error. Si lo ve mientras no se ejecuta ninguna instalación npm, la política está bloqueando el ejecutable de Claude Code en sí.

Antes de v2.1.212, Claude Code usaba solo Windows PowerShell 5.1 para iniciar el servicio, por lo que cualquier máquina donde Group Policy bloqueaba PowerShell 5.1 fallaba con `Couldn't start the session — EUNKNOWN: unknown error, uv_spawn`, incluso con PowerShell 7 instalado.

**Qué hacer:**

* Si el mensaje dice `Couldn't start the session`, actualice a v2.1.212 o posterior. En versiones anteriores también puede ejecutar `claude daemon run` en una terminal separada primero, luego iniciar la sesión en segundo plano nuevamente. Ese comando ejecuta el servicio en segundo plano en el primer plano de la terminal, por lo que el servicio dura solo mientras esa terminal permanece abierta.
* Si una instalación npm estaba reemplazando el binario, espere a que termine, luego inicie la sesión en segundo plano nuevamente
* Si el error aparece en v2.1.212 o posterior mientras no se ejecuta ninguna instalación npm, pida al administrador de Windows que permita el ejecutable de Claude Code en la política de restricción
* Si el servicio en segundo plano se detiene cuando cierra la terminal, Claude Code lo inició sin PowerShell. Instale PowerShell 7, o pida al administrador que desbloquee PowerShell, para que el servicio pueda sobrevivir a la terminal.

<h3 id="eacces-when-starting-a-background-session">
  EACCES al iniciar una sesión en segundo plano
</h3>

Claude Code no pudo ejecutar su propio binario para iniciar el [servicio en segundo plano](/docs/es/agent-view#the-supervisor-process) que aloja sesiones en segundo plano. En una instalación npm, esto generalmente significa que `npm install -g @anthropic-ai/claude-code` estaba reemplazando el binario en ese momento, ya sea que lo ejecutara o el [actualizador automático](/docs/es/setup#auto-updates) lo hizo. El error aparece cuando abre una sesión desde [agent view](/docs/es/agent-view):

```text theme={null}
Couldn't start the background service — spawn background service: EACCES: permission denied, posix_spawn '/usr/local/lib/node_modules/@anthropic-ai/claude-code/bin/claude'
```

Cuando inicia una sesión con `/background` o `claude --bg`, la misma razón aparece dentro de `Couldn't reach the background service (...)`. Durante la misma ventana de reinstalación el error puede nombrar otro código en su lugar, como `ENOENT` o `ENOEXEC`, o `EUNKNOWN` o `EPERM` en Windows; un `EUNKNOWN` que persiste en reintentos tiene una [causa diferente](#eunknown-when-starting-a-background-session).

En una instalación npm, Claude Code espera a que la reinstalación termine y reintenta por su cuenta: hasta diez segundos, y hasta dos minutos mientras una instalación npm de Claude Code aún se está ejecutando visiblemente en la máquina, lo que cubre otro proceso Claude Code descargando una actualización. Cuando la instalación supera esa espera, el fallo nombra la actualización en lugar del código de error desnudo:

```text theme={null}
Claude Code is being updated by npm on this machine (still not runnable after 2 min, EACCES) — try again when the update finishes
```

Antes de v2.1.257, la espera se detuvo a los diez segundos en todos los casos, por lo que este error apareció mientras otro proceso Claude Code aún estaba descargando una actualización. Antes de v2.1.246, Claude Code falló de inmediato, sin esperar.

**Qué hacer:**

* Espere unos segundos, luego abra la sesión o envíe nuevamente. Cuando el mensaje dice que Claude Code se está actualizando, reintente después de que la actualización termine.
* Si el error persiste mientras no se ejecuta ninguna instalación npm, su usuario no puede ejecutar el binario instalado. Verifique sus permisos y los de su directorio, o reinstale Claude Code.

<h3 id="background-service-exited-before-it-became-reachable">
  El servicio en segundo plano salió antes de que se volviera alcanzable
</h3>

El proceso que Claude Code inició como el [servicio en segundo plano](/docs/es/agent-view#the-supervisor-process) salió antes de aceptar conexiones, por lo que Claude Code no pudo abrir su sesión. Cuando el servicio imprimió un error antes de salir, la razón entre paréntesis da el código de salida o señal y la primera línea que el servicio imprimió, que nombra qué lo detuvo:

```text theme={null}
Couldn't reach the background service (background service exited before it became reachable (exit code N): <the service's first error line>) — run 'claude daemon status'
```

Cuando abre una sesión desde [agent view](/docs/es/agent-view), la misma razón sigue a `Couldn't start the background service —`. Cuando el servicio no imprimió nada antes de salir, el mensaje dice `nothing on stderr` en su lugar.

Claude Code informa el fallo con la línea de error del servicio. Antes de v2.1.246, el fallo aparecía solo después de una espera de 45 segundos, como `background service did not become reachable within 45s`, sin la línea de error del servicio.

Dos razones entrecomilladas tienen causas conocidas:

* `Error: claude native binary not installed.`: una instalación npm estaba reemplazando el binario de Claude Code en ese momento, por lo que el servicio ejecutó el marcador de posición de npm en su lugar. Reintente después de que la instalación termine; si la línea persiste sin instalación en ejecución, [complete la instalación npm](/docs/es/troubleshoot-install#native-binary-not-found-after-npm-install). Antes de v2.1.257, una actualización automática de npm en macOS produjo este fallo en cada inicio durante la ventana de instalación.
* `nothing on stderr` con código de salida 1, en cada inicio, en Windows: `daemon.lock` nombra un proceso que Claude Code no puede señalar ni probar que se ha ido, por lo que cada nuevo servicio concluye que otro mantiene el bloqueo y sale. Un bloqueo cuyo escritor Claude Code puede probar que se ha ido se reemplaza por su cuenta y no produce este fallo. Cuando el fallo se repite en cada inicio, elimine `~/.claude/daemon.lock`, luego abra la sesión o envíe nuevamente. Antes de v2.1.257, tal bloqueo bloqueaba cada inicio hasta que eliminara el archivo.

**Qué hacer:**

* Si el mensaje cita una línea, corrija lo que nombra, luego abra la sesión o envíe nuevamente. El siguiente intento inicia el servicio nuevamente
* Ejecute `claude daemon status` para verificar si un servicio se está ejecutando ahora

<h3 id="working-directory-no-longer-exists-when-starting-a-background-session">
  El directorio de trabajo ya no existe al iniciar una sesión en segundo plano
</h3>

Intentó iniciar una [sesión en segundo plano](/docs/es/agent-view) en un directorio que ya no existe. Esto sucede cuando envía desde agent view o ejecuta `/background` después de que el directorio en el que estaba trabajando fue eliminado o movido. También sucede cuando se adjunta a o reinicia una sesión cuyo proceso ha salido y cuyo directorio se ha ido, porque el nuevo proceso comenzaría en ese mismo directorio. Claude Code no inicia la sesión, y el mensaje nombra el directorio faltante:

```text theme={null}
Couldn't start a background session (working directory no longer exists or is not accessible: /tmp/demo)
```

Antes de v2.1.257, la sesión parecía iniciarse y luego se mostraba en agent view como una fila fallida con la misma razón.

**Qué hacer:**

* Recree el directorio que el mensaje nombra, o envíe desde un directorio que existe, luego intente nuevamente

<h2 id="wrapper-and-ide-errors">
  Errores de wrapper e IDE
</h2>

Estos errores provienen del programa que inició Claude Code para usted, como una extensión de IDE o una aplicación del [Agent SDK](/docs/es/agent-sdk/overview), en lugar de provenir de Claude Code en sí.

<h3 id="claude-code-process-exited-with-code-n">
  El proceso de Claude Code salió con código N
</h3>

El proceso `claude` subyacente salió con un código distinto de cero. El código de salida por sí solo no indica qué falló: el error real está en la propia salida del proceso, que el wrapper añade cuando la capturó y de lo contrario mantiene en sus registros.

```text theme={null}
Error: Claude Code process exited with code 1
```

En Windows, la compilación nativa puede salir con código `4294967295` justo después de que se completa un turno. Cuando esa salida llega a un límite de turno, sin mensaje esperando y sin tarea de fondo ejecutándose, la [extensión de VS Code](/docs/es/vs-code) cierra la sesión silenciosamente en lugar de mostrar este error. Su siguiente mensaje reanuda la conversación.

Antes de v2.1.273, la extensión mostraba el error para esa salida en cada límite de turno, aunque nada se perdiera.

**Qué hacer:**

* En VS Code, siga el enlace **Ver registros de salida** que se muestra con el error para ver el fallo subyacente
* En una aplicación del Agent SDK, capture el error alrededor de su bucle de mensajes. Las entradas bajo [Salida del proceso CLI](/docs/es/agent-sdk/troubleshooting#cli-process-exit) cubren lo que su código recibe en cada lenguaje de SDK.
* Ejecute `claude` en una terminal en el mismo proyecto. El fallo generalmente se reproduce allí con su mensaje de error real, que luego puede buscar en esta página.
* Ejecute `claude doctor` en una terminal para verificar la instalación y la configuración

<h3 id="could-not-locate-the-claude-cli-on-path">
  No se pudo localizar la CLI de Claude en PATH
</h3>

La [extensión de VS Code](/docs/es/vs-code) muestra este error en Windows cuando abre Claude Code en la terminal integrada, el shell de la terminal es PowerShell y la extensión no puede encontrar el ejecutable `claude` instalado en PATH. La extensión se niega a iniciar Claude Code hasta que encuentre el `claude` instalado en PATH.

```text theme={null}
Failed to run Claude Code: Error: Could not locate the Claude CLI on PATH. Launching by name in a PowerShell terminal would run a 'claude' from the open folder instead of the installed CLI, so the launch was blocked. Make sure the Claude CLI's install directory is on your system PATH (not only your PowerShell profile), then restart VS Code and try again. VS Code reads PATH when it starts, so PATH changes take effect only after a restart.
```

**Qué hacer:**

* Abra una nueva ventana de PowerShell fuera de VS Code y ejecute `where.exe claude`. Si no imprime una ruta, la CLI no está en su PATH: agregue su directorio de instalación siguiendo [Verifique su PATH](/docs/es/troubleshoot-install#verify-your-path). Si imprime una ruta, la entrada proviene de su perfil de PowerShell o de un cambio de PATH que VS Code aún no ha recogido; los próximos dos pasos cubren esos casos.
* Establezca la entrada de PATH como una variable de entorno de usuario o sistema, no en su perfil de PowerShell. La extensión no ejecuta su perfil, por lo que una edición de PATH que vive solo allí nunca la alcanza.
* Reinicie VS Code después de cambiar PATH. La extensión verifica el PATH que VS Code capturó al inicio, por lo que un cambio de PATH solo tiene efecto después de un reinicio.

<h3 id="the-connection-to-claude-code-ended-before-this-message-completed">
  La conexión a Claude Code terminó antes de que este mensaje se completara
</h3>

La [extensión de VS Code](/docs/es/vs-code) envió su mensaje al proceso `claude`, y la conexión terminó sin un error antes de que el proceso lo reconociera o lo completara. La extensión no puede saber si el mensaje fue procesado, por lo que le pide que lo envíe de nuevo:

```text theme={null}
The connection to Claude Code ended before this message completed — it may not have been processed, so please send it again.
```

**Qué hacer:**

* Envíe el mensaje de nuevo. El siguiente mensaje inicia un nuevo proceso `claude` que reanuda la conversación.
* Si se repite, ejecute `claude` en una terminal en el mismo proyecto. Un fallo que sigue terminando el proceso generalmente se reproduce allí con su mensaje de error real.

<h2 id="rewind-warnings-and-errors">
  Advertencias y errores de Rewind
</h2>

Estos mensajes provienen de una restauración de código [`/rewind`](/docs/es/checkpointing). `Restored the code, but skipped N files` es una advertencia que indica que Claude Code omitió algunas rutas. `No files were restored` es un error que significa que no restauró nada.

<h3 id="restored-the-code-but-skipped-files">
  Restored the code, but skipped files
</h3>

Una restauración de código `/rewind` omitió una o más rutas rastreadas en lugar de escribir o eliminar a través de ellas. Claude Code omite una ruta cuando:

* es, o se convirtió en, un enlace simbólico, enlace duro u otro archivo no regular
* su directorio cambió desde el punto de control
* su copia de seguridad no se puede leer de forma segura

Las rutas omitidas mantienen su contenido actual. Antes de v2.1.216, `/rewind` escribía y eliminaba a través de enlaces en rutas rastreadas, y no informaba de una restauración parcial.

```text theme={null}
Restored the code, but skipped 2 files: the tracked path is (or became) a link or other non-regular file, its directory changed since the checkpoint, or its backup could not be safely read. Skipped files were left untouched — run with --debug for the paths.
```

**Qué hacer:**

* Identifique qué archivos se omitieron para poder manejar cada uno con los pasos a continuación. El mensaje solo proporciona un recuento; el registro de depuración en `~/.claude/debug/<session-id>.txt` nombra cada ruta omitida mientras se ejecuta la restauración, así que active el registro de depuración con `/debug` antes de su próxima restauración. En macOS o Linux, puede encontrar los enlaces directamente: `find . -type l` para enlaces simbólicos y `find . -type f -links +1` para archivos con enlaces duros.
* Si un archivo omitido es un enlace que creó a propósito, como un archivo de configuración administrado por un gestor de dotfiles o un archivo con enlace duro por herramientas como pnpm, la restauración dejó su contenido intacto. Para deshacer los cambios de la sesión en él, pida a Claude que revierta la edición o edite el archivo usted mismo
* Si no creó el enlace, inspeccione la ruta antes de confiar en su contenido: algo reemplazó el archivo después del punto de control

<h3 id="no-files-were-restored">
  No files were restored
</h3>

Claude Code muestra este mensaje cuando restaura código con [`/rewind`](/docs/es/checkpointing) y no puede restaurar ninguno de los archivos en ese punto de control. Para cada archivo, falta la copia de seguridad que Claude Code guardó antes de editarlo, o Claude Code no pudo escribir o eliminar el archivo.

```text theme={null}
Failed to restore the code:
No files were restored: 1 file failed (backup missing, or the file could not be updated)
```

Claude Code elimina las copias de seguridad de una sesión en el [barrido de retención](/docs/es/claude-directory#cleaned-up-automatically), por defecto aproximadamente 30 días después de que la sesión guardara una por última vez. Si reanuda una sesión después de eso, `/rewind` aún enumera sus puntos de control, pero la restauración a uno de ellos puede fallar con este error. Si el mensaje también dice `N paths were skipped for link safety`, consulte [Restored the code, but skipped files](#restored-the-code-but-skipped-files) para esas rutas.

Cuando bifurca una sesión, por ejemplo con [`--fork-session`](/docs/es/cli-reference#cli-flags) o [`/branch`](/docs/es/sessions#branch-a-session), Claude Code copia las copias de seguridad de la sesión original en la bifurcación. Cuando Claude Code no puede copiar una copia de seguridad, por ejemplo porque el disco está lleno, esa copia de seguridad falta en la bifurcación. La restauración a un punto de control que la necesita puede fallar con este error.

**Qué hacer:**

* Deshaga los cambios de otra manera: pida a Claude que revierta sus ediciones, o restaure los archivos desde el control de versiones. Cuando las copias de seguridad desaparecen, ejecutar `/rewind` nuevamente falla de la misma manera.
* Si Claude Code no pudo escribir o eliminar un archivo, corrija lo que bloquea la escritura, como los permisos de archivo, luego ejecute `/rewind` nuevamente.
* Para mantener las copias de seguridad más tiempo en futuras sesiones, aumente [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays).

Antes de v2.1.260, Claude Code omitía silenciosamente los archivos cuyas copias de seguridad faltaban, y la restauración parecía tener éxito.

<h2 id="session-saving-warnings">
  Advertencias de guardado de sesión
</h2>

Claude Code muestra estas advertencias en una línea persistente debajo del cuadro de entrada cuando no está guardando la transcripción de su sesión. La sesión sigue funcionando de cualquier forma; las advertencias le indican que la sesión puede faltar en [`--resume`](/docs/es/sessions) más adelante.

<h3 id="transcript-writes-are-failing">
  Las escrituras de transcripción están fallando
</h3>

Claude Code guarda la transcripción en disco mientras trabaja, y sus escrituras en [el archivo de transcripción](/docs/es/sessions#where-transcripts-are-stored) están fallando. El mensaje nombra la causa con el código de error subyacente, por ejemplo un disco lleno:

```text theme={null}
Transcript writes are failing (disk full — ENOSPC) · recent messages may not be saved for resume
```

La advertencia aparece en diferentes puntos dependiendo del error:

* En el primer fallo para condiciones que no se resuelven por sí solas: un disco lleno, una cuota de disco excedida, un sistema de archivos de solo lectura, una ruta que supera el límite de longitud del sistema de archivos, o, en macOS y Linux, un error de permiso
* Después de fallos repetidos que abarcan al menos un minuto para todo lo demás, incluidos errores de permiso en Windows, donde un escaneo antivirus puede fallar una única escritura que luego tiene éxito al reintentar

Antes de v2.1.217, Claude Code descartaba las escrituras fallidas sin una advertencia, y un `--resume` posterior que faltaban mensajes recientes era el primer signo.

**Qué hacer:**

* Corrija la condición que nombra el código de error: libere espacio en disco para `ENOSPC`; aumente o borre la cuota para `EDQUOT`; restaure el acceso de escritura a la ubicación de la transcripción para `EACCES`, `EPERM`, o `EROFS`
* La advertencia se borra por sí sola en la siguiente escritura exitosa; no se necesita reiniciar
* Los mensajes enviados mientras se mostraba la advertencia aún pueden faltar cuando reanude la sesión más adelante

<h3 id="transcript-saving-is-off-skip-prompt-history">
  El guardado de transcripción está desactivado porque CLAUDE\_CODE\_SKIP\_PROMPT\_HISTORY está configurado
</h3>

Esta sesión comenzó con [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/es/env-vars) configurado, por lo que Claude Code no escribe transcripción ni historial de indicaciones para ella:

```text theme={null}
Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set · --resume will not find this session; if unintended, unset it and restart
```

La variable es una exclusión intencional para sesiones de scripts efímeros, pero también puede llegar a una sesión a través de un perfil de shell, un script contenedor, o un proceso padre que la exportó.

**Qué hacer:**

* Si configuró la variable a propósito, no se necesita ninguna acción; el aviso confirma que la sesión no aparecerá en `--resume`, `--continue`, o en el historial de flecha hacia arriba
* Si no lo hizo, elimine la variable del shell o script que inicia `claude`, luego inicie una nueva sesión. Los mensajes de la sesión actual no se guardan retroactivamente.

<h3 id="transcript-saving-is-off-child-session-marker">
  El guardado de transcripción está desactivado debido a un marcador CLAUDE\_CODE\_CHILD\_SESSION heredado
</h3>

Claude Code establece [`CLAUDE_CODE_CHILD_SESSION`](/docs/es/env-vars) en los subprocesos que genera, y trata una sesión interactiva que lo hereda como anidada: Claude Code no guarda transcripción para ella, por lo que las sesiones que Claude mismo inicia no llenan su lista de `--resume`. Este aviso significa que su sesión actual heredó el marcador:

```text theme={null}
Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker · restart with CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1 to keep future transcripts
```

El aviso es esperado cuando ejecutó `claude` desde dentro de otra sesión de Claude Code; señala una clasificación errónea cuando el marcador se filtró a través de un intermediario de larga duración, por ejemplo una terminal, sesión de `screen`, o lanzador que una sesión de Claude Code originalmente inició.

Dentro de tmux, Claude Code detecta un marcador que llegó a través del entorno global del servidor tmux y sigue guardando, por lo que este aviso no aparece para ese caso.

**Qué hacer:**

* Si inició esta sesión desde dentro de otra sesión de Claude Code a propósito, no se necesita ninguna acción
* Si esta es una sesión de nivel superior, salga y reinicie con [`CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1`](/docs/es/env-vars) configurado. El guardado se aplica desde el reinicio, por lo que los mensajes enviados antes no se guardan.
* Para corregir futuros lanzamientos desde la misma terminal o lanzador, elimine `CLAUDE_CODE_CHILD_SESSION` de su entorno

<h2 id="configuration-warnings">
  Advertencias de configuración
</h2>

Claude Code escribe la mayoría de estos mensajes en stderr, no en la conversación, y escribe la mayoría de ellos al iniciar. Una entrada lo indica cuando su mensaje aparece en otro lugar, como en el registro de depuración o como un aviso de inicio en la vista de conversación, o en otro momento, como la [línea de diagnóstico de modelo no reconocido](#unrecognized-model-id-on-a-request) en el momento de la solicitud.

<h3 id="fullscreen-failed-start-notice">
  El renderizador de pantalla completa no terminó de iniciarse
</h3>

Una sesión anterior de [pantalla completa](/docs/es/fullscreen) en esta máquina se cerró antes de terminar de iniciarse, por lo que Claude Code inicia esta sesión en el renderizador clásico e imprime uno de estos avisos:

```text theme={null}
Claude Code's fullscreen renderer didn't finish starting last time on this machine, so this launch is using the classic renderer. It will try fullscreen again next launch; /tui default keeps the classic renderer.

Claude Code's fullscreen renderer has repeatedly failed to start on this machine, so it has been turned off here. Run /tui fullscreen to try it again (this also resets after an update).
```

**Qué hacer:**

* Siga [Fullscreen rendering](/docs/es/fullscreen#fullscreen-renderer-didnt-finish-starting). Dice qué aviso obtiene, qué hace Claude Code en sesiones posteriores, y cómo intentar pantalla completa de nuevo o mantener el renderizador clásico.
* Si la sesión que se cerró imprimió un mensaje de salida, consulte [Claude Code se cerró después de un error de interfaz irrecuperable](#exited-after-an-unrecoverable-interface-error) para ver qué nombre tiene.

Antes de v2.1.236, Claude Code no imprimía ningún aviso y seguía iniciando sesiones en renderización de pantalla completa después de un inicio fallido.

<h3 id="exited-after-an-unrecoverable-interface-error">
  Claude Code se cerró después de un error de interfaz irrecuperable
</h3>

Claude Code imprime este mensaje cuando se cierra porque su interfaz de terminal encontró un error del que no puede recuperarse, en cualquiera de los renderizadores. La segunda oración aparece solo cuando el error ocurrió mientras el renderizador de [pantalla completa](/docs/es/fullscreen) se estaba iniciando:

```text theme={null}
Claude Code exited after an unrecoverable interface error (<error>). It happened while the fullscreen renderer was starting, so the next launch will use the classic renderer (CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1 forces that any time).
```

**Qué hacer:**

* Inicie Claude Code de nuevo. Para retomar la conversación, ejecute `claude --resume` en el mismo directorio.
* Si el mensaje menciona el renderizador de pantalla completa, [Fullscreen rendering](/docs/es/fullscreen#fullscreen-renderer-didnt-finish-starting) dice qué hace el siguiente inicio, que depende de cómo activó la pantalla completa, y cómo intentar pantalla completa de nuevo o mantener el renderizador clásico.

Antes de v2.1.236, Claude Code se cerraba sin imprimir un mensaje después de este tipo de error.

<h3 id="agent-descriptions-are-over-the-15000-token-limit">
  Las descripciones de agentes superan el límite de 15.0k tokens
</h3>

Claude Code muestra esta advertencia como un aviso de inicio en la vista de conversación en lugar de en stderr. Las descripciones combinadas de sus [subagentes](/docs/es/sub-agents), excepto las integradas, superan 15,000 tokens según las estima Claude Code. Cada agente cuenta su nombre más su frontmatter `description`. Claude Code carga cada agente independientemente de si el total supera el límite, por lo que la advertencia no cambia lo que se carga.

```text theme={null}
Agent descriptions are over the 15.0k-token limit (~16.2k tokens) · ask Claude to trim agent descriptions in .claude/agents/
```

**Qué hacer:**

* Acorte el frontmatter `description` de sus archivos de agente, o pida a Claude que los recorte por usted.
* Elimine los archivos de agente que ya no utiliza.

<h3 id="workspace-has-not-been-trusted">
  El espacio de trabajo no ha sido confiable
</h3>

Claude Code encontró reglas `permissions.allow` o entradas `permissions.additionalDirectories` en el archivo `.claude/settings.json` o `.claude/settings.local.json` del proyecto y no las aplicó, porque [las reglas de permiso del proyecto requieren confianza del espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust). El recuento, el nombre de la configuración y el archivo nombrado en el mensaje varían según su configuración. Las reglas `deny` y `ask` no se ven afectadas.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Qué hacer:**

* Ejecute `claude` en el directorio y acepte el diálogo de confianza. [Project allow rules and workspace trust](/docs/es/permissions#project-allow-rules-and-workspace-trust) dice qué carpeta cubre esa aceptación.
* En [modo no interactivo](/docs/es/headless) con `-p` no se muestra ningún diálogo. Establezca la entrada `hasTrustDialogAccepted` en `~/.claude.json` usando la clave exacta `projects` que imprime el mensaje.
* Si el mensaje menciona `.claude/settings.local.json` e inició Claude Code fuera de un repositorio git o en su directorio de inicio, actualice a v2.1.200 o posterior. Las versiones 2.1.196 a 2.1.199 trataban su propio `.claude/settings.local.json` como suministrado por el repositorio en esos espacios de trabajo. En v2.1.207 y posterior, actualizar no es suficiente fuera de un repositorio git si no ha confiado en la carpeta: determinar que una carpeta no está dentro de un repositorio ejecuta git, y Claude Code ejecuta esa verificación solo después de que acepte el diálogo de confianza, así que use el primer paso. Su directorio de inicio y cualquier otro [directorio de configuración](/docs/es/permissions#project-allow-rules-and-workspace-trust) están exentos y no esperan el diálogo. Consulte [Project allow rules and workspace trust](/docs/es/permissions#project-allow-rules-and-workspace-trust).

<h3 id="working-directory-is-a-network-path">
  El directorio de trabajo es una ruta de red
</h3>

Claude Code no agrega rutas de red como directorios de trabajo. Buscar una ruta de red puede contactar al host que nombra, y en Windows ese contacto puede enviar al host sus credenciales, por lo que Claude Code rechaza la ruta sin buscarla. Ve este mensaje cuando ejecuta `/add-dir` con tal ruta, o como una advertencia al iniciar. Cuando aparece al iniciar, Claude Code se inicia sin ese directorio.

```text theme={null}
\\server\share is a network path, which cannot be added as a working directory. On Windows, map the share to a drive letter and pass it at launch with --add-dir (a drive letter added mid-session does not yet carry remote-read trust).
```

Las rutas que Claude Code rechaza de esta manera incluyen:

* Recursos compartidos UNC como `\\server\share`
* Rutas de montaje automático como `/net/<host>`, a menos que haya iniciado Claude Code desde un directorio bajo el montaje automático de ese host
* Rutas locales que alcanzan una ubicación de red a través de un enlace simbólico o unión

Las letras de unidad asignadas y las rutas `\\wsl$` no cuentan como rutas de red.

**Qué hacer:**

* En Windows, asigne el recurso compartido a una letra de unidad, por ejemplo con `net use Z: \\server\share`, y pase la unidad al iniciar con `claude --add-dir Z:\`.
* En macOS o Linux, monte el recurso compartido en una ruta local y agregue esa ruta en su lugar.
* Si la ruta está en `permissions.additionalDirectories`, elimínela del archivo de configuración que la enumera.

Antes de v2.1.257, Claude Code aceptaba una ruta de red accesible como directorio de trabajo.

<h3 id="remote-managed-settings-failed-to-load">
  La configuración remota administrada no se pudo cargar
</h3>

Su sesión es elegible para [configuración administrada por servidor](/docs/es/server-managed-settings), pero Claude Code no pudo obtenerla, por lo que muestra esta advertencia en sesiones interactivas. La causa entre paréntesis nombra lo que falló, como `network error`, `request timed out` o `authentication rejected (401)`, y el resto de la línea dice qué política ejecuta la sesión:

* **Configuración almacenada en caché de una obtención anterior exitosa**: Claude Code ejecuta la sesión en esa política almacenada en caché, excepto las [variables de entorno retenidas](/docs/es/server-managed-settings#fetch-and-caching-behavior), y la línea dice `using cached policy`.
* **Sin caché**: Claude Code ejecuta la sesión sin configuración administrada por servidor, y la línea dice `no remote policy applied`.

**Qué hacer:**

* Actúe sobre la causa que nombra el mensaje: para una causa de red, verifique que esta máquina pueda alcanzar `api.anthropic.com`; para una causa de autenticación, verifique su inicio de sesión con `/status`
* Ejecute `/status` o `claude doctor` para el diagnóstico completo

Antes de v2.1.248, Claude Code reportaba una obtención de configuración fallida solo en el registro de depuración.

<h3 id="managed-settings-were-not-approved">
  La configuración administrada no fue aprobada
</h3>

La [configuración administrada por servidor](/docs/es/server-managed-settings) de su organización incluye configuración que necesita su aprobación, y usted rechazó el [diálogo de aprobación de seguridad](/docs/es/server-managed-settings#security-approval-dialogs), por lo que Claude Code se cierra sin aplicarla:

```text theme={null}
Managed settings were not approved; exiting without applying them.
```

**Qué hacer:**

* Inicie Claude Code de nuevo y apruebe el diálogo para continuar bajo la configuración de su organización. Un diálogo rechazado no se recuerda, por lo que aparece de nuevo en el siguiente inicio.
* Si no está seguro acerca de una configuración que enumera el diálogo, pregunte a quien mantenga la configuración administrada de su organización antes de aprobar

<h3 id="mcp-server-is-blocked-by-enterprise-managed-policy">
  El servidor MCP está bloqueado por la política administrada empresarial
</h3>

Seleccionó **Reconnect** en un servidor en `/mcp`, o activó un servidor deshabilitado allí, y una configuración que [restringe servidores MCP](/docs/es/managed-mcp) bloquea ese servidor. Claude Code se niega a conectarlo y muestra:

```text theme={null}
MCP server <name> is blocked by enterprise managed policy
```

Cualquiera de estas configuraciones puede producir el mensaje:

* Una entrada [`deniedMcpServers`](/docs/es/managed-mcp#policy-based-control-with-allowlists-and-denylists) que coincida con el servidor, incluida una en su propio `~/.claude/settings.json` o en el `.claude/settings.json` del proyecto
* Una lista [`allowedMcpServers`](/docs/es/managed-mcp#policy-based-control-with-allowlists-and-denylists) que el servidor no coincida
* [`strictPluginOnlyCustomization`](/docs/es/settings-reference#strictpluginonlycustomization) con `mcp` bloqueado, que bloquea servidores configurados en `~/.claude.json` y `.mcp.json`
* [`disableClaudeAiConnectors`](/docs/es/mcp#disable-claude-ai-connectors), cuando el servidor es un conector de claude.ai

**Qué hacer:**

* Verifique sus propios archivos de configuración de usuario y proyecto para una de estas configuraciones y cámbiela o elimínela
* Si ninguna de sus propias configuraciones explica el bloqueo, pregunte a su administrador qué configuración administrada bloquea el servidor

Antes de v2.1.257, **Reconnect** y re-habilitar en `/mcp` podían conectar un servidor que una actualización de política a mitad de sesión bloqueó.

<h3 id="managed-settings-document-could-not-be-parsed">
  El documento de configuración administrada no se pudo analizar
</h3>

Su organización implementa [configuración administrada](/docs/es/managed-settings), y uno de los documentos implementados está presente pero no se puede analizar como un objeto JSON, por lo que Claude Code se cierra con código 1 al iniciar en lugar de ejecutarse sin la política que lleva el documento. La línea nombra la fuente fallida antes del mensaje:

```text theme={null}
/Library/Application Support/ClaudeCode/managed-settings.json: Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.
```

La fuente es una de:

* La ruta del archivo `managed-settings.json` o un archivo drop-in bajo `managed-settings.d`
* El perfil de preferencias administradas de macOS, `per-user managed preferences` o `device-level managed preferences`
* El valor del registro de Windows, `Registry: HKLM\SOFTWARE\Policies\ClaudeCode\Settings`

[Find entries Claude Code dropped](/docs/es/managed-settings#find-entries-claude-code-dropped) enumera lo que hace que cada fuente sea no analizable.

Claude Code se niega a iniciar incluso cuando otra fuente de administrador entrega una política válida. Ve este error en sesiones interactivas, `claude -p`, sesiones del SDK de agente, [sesiones en segundo plano](/docs/es/agent-view), y la mayoría de subcomandos, `claude doctor` incluido. El rechazo falla cerrado a propósito: la configuración en un documento que Claude Code no puede analizar no se puede aplicar, y iniciar de todas formas ejecutaría sesiones sin los controles de la organización.

Un problema de esquema en un documento analizable no produce este error. [Find entries Claude Code dropped](/docs/es/managed-settings#find-entries-claude-code-dropped) cubre qué hace Claude Code con uno.

Cuando existe un directorio `managed-settings.d/` pero no se puede enumerar, Claude Code reporta `Managed settings drop-in directory could not be read:` seguido del error subyacente en su lugar. [Find entries Claude Code dropped](/docs/es/managed-settings#find-entries-claude-code-dropped) cubre cuándo una falla de lectura se cierra al iniciar.

**Qué hacer:**

* Si administra la máquina, corrija el documento nombrado para que se analice como un objeto JSON, o elimine el archivo, perfil o valor del registro. Un `managed-settings.json` vacío cuenta como `{}` y no bloquea el inicio.
* Si no lo hace, pida a su administrador que corrija el documento implementado. Nada en sus propios archivos de configuración causa o borra este error.

<h3 id="otelheadershelper-failed">
  otelHeadersHelper falló
</h3>

Claude Code muestra esta advertencia como una notificación en la interfaz de terminal, una vez por sesión interactiva, cuando el script [`otelHeadersHelper`](/docs/es/settings-reference#otelheadershelper) falla o imprime salida que no cumple con los [requisitos del script](/docs/es/monitoring-usage#script-requirements).

Mientras el script sigue fallando, las exportaciones fallan y su backend de telemetría no recibe nada de la sesión.

El texto después de `See /status:` dice qué falló, como el código de salida del script seguido de su salida de error:

```text theme={null}
otelHeadersHelper failed; telemetry is not being exported. See /status: exited 1: token service unreachable
```

**Qué hacer:**

* Ejecute `/status` para leer el detalle de la falla.
* Corrija el script para que salga 0 dentro de 30 segundos e imprima un objeto JSON de valores de encabezado de cadena en stdout. Consulte [requisitos del script](/docs/es/monitoring-usage#script-requirements).
* Si su organización implementa el script a través de [configuración administrada](/docs/es/managed-settings), pida a quien la mantenga que lo corrija.

En [modo no interactivo](/docs/es/headless) con `-p`, la misma falla aparece en stderr como `otelHeadersHelper failed (OpenTelemetry export headers unavailable): <error>` en su lugar.

<h3 id="headershelper-not-run">
  headersHelper no ejecutado
</h3>

Claude Code conectó un servidor MCP con solo sus `headers` estáticos y omitió el [`headersHelper`](/docs/es/mcp#use-dynamic-headers-for-custom-authentication) del servidor, porque el asistente es un comando de shell y la carpeta no tiene confianza guardada. Una carpeta obtiene confianza guardada cuando establece su entrada en `~/.claude.json` a mano o, fuera de su directorio de inicio, cuando acepta el diálogo de confianza para ella en una sesión interactiva. Consulte [Trust a folder before its headersHelper runs](/docs/es/mcp#trust-a-folder-before-its-headershelper-runs) para ver a qué servidores se aplica esta verificación.

Claude Code escribe esta línea en [modo no interactivo](/docs/es/headless) solo, una vez por servidor. En una sesión interactiva escribe el mismo rechazo en el registro de depuración en su lugar.

```text theme={null}
MCP server 'internal-api': headersHelper not run — this workspace has no persisted trust; accept the trust dialog here once interactively, or set projects["/Users/you/project"].hasTrustDialogAccepted in /Users/you/.claude.json.
```

La clave `projects` que imprime el mensaje es la carpeta en la que [Project allow rules and workspace trust](/docs/es/permissions#project-allow-rules-and-workspace-trust) dice que Claude Code basa la confianza. Aceptar el diálogo de confianza para una carpeta padre no satisface la verificación, y una sesión `-p` o SDK tampoco la satisface.

**Qué hacer:**

* Ejecute `claude` en la carpeta que nombra el mensaje, acepte el diálogo de confianza, luego ejecute su comando `-p` o SDK de nuevo
* Establezca la entrada `hasTrustDialogAccepted` en `~/.claude.json` usted mismo, usando la clave exacta `projects` que imprime el mensaje
* Si inició la sesión en su directorio de inicio, trabaje desde un directorio de proyecto en el que haya confiado. Cuando acepta el diálogo de confianza en su directorio de inicio, Claude Code mantiene esa confianza solo para la sesión actual.

<h3 id="malformed-tool-content-rule">
  Regla Tool(content) malformada
</h3>

Una [regla de permiso](/docs/es/permissions#permission-rule-syntax) en uno de sus archivos de configuración no tiene la forma `Tool` o `Tool(content)`, por ejemplo porque el texto sigue al paréntesis de cierre o falta uno de los paréntesis. Claude Code omite la regla y la enumera en el diálogo de configuración no válida cuando se inicia una sesión interactiva, y en la salida de [`claude doctor`](/docs/es/debug-your-config#check-resolved-settings):

```text theme={null}
Invalid permission rule "Bash(ls) x" was skipped: Malformed Tool(content) rule. Rules take the form Tool or Tool(content) and must end at the closing ")"; parentheses inside the content are literal
```

**Qué hacer:**

* En el archivo de configuración listado con el mensaje, reescriba la regla para que termine en su paréntesis de cierre, por ejemplo `Bash(ls *)` en lugar de `Bash(ls) x`
* Deje los paréntesis dentro del contenido como están. Son literales, por lo que una regla como `Edit(./Finance (2024)/*)` es válida sin escapar

Antes de v2.1.260, Claude Code reportaba una regla con paréntesis sin emparejar como `Mismatched parentheses`.

<h3 id="is-not-matched-by-file-permission-checks">
  No coincide con las verificaciones de permiso de archivo
</h3>

Claude Code encontró una regla de permiso `Write`, `NotebookEdit`, `MultiEdit` o `Glob` [permission rule](/docs/es/permissions#read-and-edit) con una ruta en uno de sus [archivos de configuración](/docs/es/settings#where-settings-live), en [configuración administrada](/docs/es/managed-settings), o en un valor de bandera `--allowedTools`, `--disallowedTools` o `--settings`. Verifica permisos de archivo solo contra reglas `Edit` y `Read`, por lo que nunca consulta una regla de ruta que nombre una de las otras herramientas de archivo. Mantiene la regla y no cambia nada más; la advertencia nombra la regla, su fuente entre paréntesis, y el reemplazo a escribir:

```text theme={null}
Permission deny rule (.claude/settings.json): Write(docs/**) is not matched by file permission checks — only Edit(path) rules are. Use Edit(docs/**) instead (Edit rules cover all file-editing tools).
```

**Qué hacer:**

* Reemplace las reglas `Write(path)`, `NotebookEdit(path)` y `MultiEdit(path)` heredadas con `Edit(path)`. Las reglas `Edit` cubren todas las herramientas de edición de archivos.
* Excepto en `--allowedTools`, donde Claude Code acepta una regla `Glob` sin advertencia, reemplace las reglas `Glob(path)` con `Read(path)`.
* Corrija la regla en la fuente que nombra la advertencia entre paréntesis: una ruta de archivo de configuración, o la bandera misma para `--allowed-tools` y `--disallowed-tools`. Una ruta `claude-settings-<hash>.json` que no existe en el disco representa un valor `--settings` en línea. Corrija el JSON que pasa a esa bandera.
* Deje las reglas de nombre de herramienta simple como `Write` o `Glob` solas. Claude Code las coincide en el [nivel de herramienta](/docs/es/permissions#match-all-uses-of-a-tool) y no advierte sobre ellas.
* Si la fuente dice `managed policy settings`, reenvíe la advertencia a quien mantenga su configuración administrada, ya que no puede borrarla usted mismo.

En una [sesión en segundo plano](/docs/es/agent-view) o con `--output-format json` o `stream-json`, Claude Code escribe la advertencia en el registro de depuración en lugar de stderr, por lo que la salida leída por máquina se mantiene limpia. Ejecute con `--debug` para capturarla en `~/.claude/debug/<session-id>.txt`. Antes de v2.1.210, Claude Code aceptaba estas reglas sin una advertencia.

<h3 id="has-a-wildcard-before-the-rest-of-the-command">
  Tiene un comodín antes del resto del comando
</h3>

Claude Code encontró una regla de permiso `Bash` allow cuyo `*` viene antes de una palabra posterior que determina cuál es el comando, como `Bash(git * main)` o `Bash(git -C * status *)`, en uno de sus [archivos de configuración](/docs/es/settings#where-settings-live), en [configuración administrada](/docs/es/managed-settings), o en un valor de bandera `--allowedTools` o `--settings`. El `*` coincide con cualquier texto, incluidas opciones insertadas en esa posición: `Bash(git * main)` también aprueba `git -c core.fsmonitor=<script> diff main`, donde `-c` hace que git ejecute un programa que nombra el comando. [Wildcard patterns](/docs/es/permissions#wildcard-patterns) muestra las reglas de coincidencia.

La advertencia existe para que pueda estrechar una regla cuyo comodín es más amplio de lo que pretendía. Claude Code mantiene la regla y no cambia nada sobre cómo coincide; la advertencia nombra la regla y su fuente entre paréntesis:

```text theme={null}
Permission allow rule (.claude/settings.json): Bash(git -C * status *) has a wildcard before the rest of the command, so it also matches any options inserted at that position and approves them without a prompt. For git, options such as -c and --exec-path can run arbitrary commands. Replace that * with the exact value you mean, or only use * after the subcommand (for example Bash(git status *)).
```

**Qué hacer:**

* Reemplace el `*` antes del subcomando con el valor exacto que significa: `Bash(git checkout main)` en lugar de `Bash(git * main)`.
* Mueva cada `*` después del subcomando: `Bash(git status *)` en lugar de `Bash(git -C * status *)`. Escriba una regla por subcomando que desee permitir.
* Corrija la regla en la fuente que nombra la advertencia entre paréntesis: una ruta de archivo de configuración, o la bandera `--allowed-tools` misma. Una ruta `claude-settings-<hash>.json` que no existe en el disco representa un valor `--settings` en línea. Corrija el JSON que pasa a esa bandera.
* Si la fuente dice `managed policy settings`, reenvíe la advertencia a quien mantenga su configuración administrada, ya que no puede borrarla usted mismo.

Claude Code no advierte sobre reglas de negación y solicitud con la misma forma: se niega o solicita los comandos adicionales que coinciden en lugar de aprobarlos. Tampoco advierte sobre reglas cuyo subcomando viene antes del primer `*`, como `Bash(git commit *)`, o reglas en las que ninguna palabra que no sea una opción sigue al `*`, como `Bash(git *)`, o sobre reglas de prefijo `:*` como `Bash(git:*)`.

En una [sesión en segundo plano](/docs/es/agent-view) o con `--output-format json` o `stream-json`, Claude Code escribe la advertencia en el registro de depuración en lugar de stderr, por lo que la salida leída por máquina se mantiene limpia. Ejecute con `--debug` para capturarla en `~/.claude/debug/<session-id>.txt`. Antes de v2.1.246, Claude Code aceptaba estas reglas sin una advertencia.

<h3 id="crosssessioninbound-must-be-one-of-accept-hold-refuse">
  crossSessionInbound debe ser uno de accept, hold, refuse
</h3>

Un archivo de configuración establece [`crossSessionInbound`](/docs/es/settings-reference#crosssessioninbound) en un valor que Claude Code no reconoce, como el error tipográfico `"reject"`. La segunda oración de la advertencia depende de qué archivo contiene el valor; en un archivo de usuario, proyecto, local o `--settings` dice:

```text theme={null}
"crossSessionInbound" must be one of "accept", "hold", "refuse"; received "reject". This value was ignored; while it is present, cross-session messages are held for your approval instead of being delivered. Set it to one of the values above.
```

En [configuración administrada](/docs/es/managed-settings), Claude Code trata el valor no reconocido como `refuse`, el valor más restrictivo, y la advertencia dice que los mensajes entre sesiones se rechazan hasta que un administrador lo corrija. Para cómo la retención se combina con valores en sus otros archivos de configuración, consulte [`crossSessionInbound`](/docs/es/settings-reference#crosssessioninbound).

**Qué hacer:**

* Establezca la clave en `"accept"`, `"hold"` o `"refuse"`, o elimínela
* Cuando la advertencia nombra configuración administrada, pida al administrador que corrija el valor

Antes de v2.1.248, Claude Code ignoraba un valor no reconocido sin advertencia.

<h3 id="the-200k-limit-isnt-enforced">
  El límite de 200K no se aplica
</h3>

Estableció [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/es/env-vars), que normalmente hace que [auto-compaction](/docs/es/model-config#default-auto-compact-thresholds) mantenga sesiones en modelos de contexto 1M en una ventana de 200K, pero ningún umbral de compactación limita esta sesión a o por debajo de 200K, por lo que la conversación puede crecer más allá.

```text theme={null}
CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced for <model>, so this session can grow past it. To enforce it, set CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000 (or the autoCompactWindow setting).
```

Claude Code aplica el límite de 200K por su cuenta para cada modelo que reconoce como teniendo una ventana nativa de 1M, y para ID de modelo que no reconoce compacta en la ventana que asume. La advertencia aparece cuando otra configuración derrota esa aplicación:

* El ID del modelo no es uno que Claude Code reconozca, como un alias de [LLM gateway](/docs/es/llm-gateway), y estableció [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/es/env-vars) o elevó la ventana asumida más allá de 200K con [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/es/env-vars). En este caso el mensaje también ofrece `or update to a Claude Code version that recognizes <model>` como un remedio.
* Un `context-1m` beta solicitado a través de [`ANTHROPIC_BETAS`](/docs/es/env-vars) o la bandera [`--betas`](/docs/es/cli-reference#cli-flags) aún solicita a la API la ventana de 1M en un modelo que acepta esa beta, mientras que nada compacta la sesión a 200K

**Qué hacer:**

* Establezca [`CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000`](/docs/es/env-vars), o la configuración [`autoCompactWindow`](/docs/es/settings-reference#autocompactwindow) a `200000`, para que auto-compaction compacte en el límite de 200K
* Si el mensaje nombra un ID de modelo que esta versión no reconoce, ejecute `claude update`. Una versión que reconoce el ID como un modelo de contexto 1M aplica el límite sin configuración adicional.
* Si desea que la sesión use la ventana completa del modelo en su lugar, desestablezca `CLAUDE_CODE_DISABLE_1M_CONTEXT`; la advertencia reporta solo que el límite de 200K no se aplica

En una [sesión en segundo plano](/docs/es/agent-view) o con `--output-format json` o `stream-json`, Claude Code escribe la advertencia en el registro de depuración en lugar de stderr.

<h3 id="unrecognized-model-id-on-a-request">
  ID de modelo no reconocido en una solicitud
</h3>

Claude Code envió una solicitud para un ID de modelo que su versión de Claude Code no reconoce, y no encontró ninguna entrada [`modelOverrides`](/docs/es/model-config#override-model-ids-per-version) que asigne ese ID a un modelo que sí reconoce. Claude Code aún envía la solicitud con el ID como lo configuró, y no se cierra ni cambia de modelo.

```text theme={null}
[claude-code:unrecognized_model] {"model":"my-proxy-model","query_source":"sdk"}
```

En un script o arnés que lee stderr, coincida con el prefijo `[claude-code:unrecognized_model]`. Después del prefijo y un espacio, Claude Code escribe un objeto JSON de una línea. Claude Code puede agregar campos a él en una versión posterior, así que ignore cualquier campo que no espere. Escribe al menos estos dos:

* `model`: la cadena del modelo como la configuró
* `query_source`: la ruta de solicitud que usó el modelo. Claude Code reporta `sdk` para una ejecución `-p` y un valor que comienza con `agent:` para un subagente.

Claude Code escribe la línea en uno de dos lugares, dependiendo de cómo la ejecute:

* En [modo no interactivo](/docs/es/headless) con `-p`, Claude Code la escribe en stderr bajo cada `--output-format`, para que pueda analizar stdout sin filtrar la línea
* En una sesión interactiva o una [sesión en segundo plano](/docs/es/agent-view), Claude Code la escribe en el registro de depuración en su lugar; ejecute con `--debug` para capturarla en `~/.claude/debug/<session-id>.txt`

Claude Code escribe la línea una vez por cadena de modelo por proceso. Escribe una línea separada para cada ID no reconocido adicional, como uno que un [subagente](/docs/es/sub-agents#choose-a-model) o [funcionalidad en segundo plano](/docs/es/costs#background-token-usage) usa.

Claude Code no escribe la línea para ID de proveedor que resuelve a un modelo que reconoce, como ID de Amazon Bedrock `us.anthropic.claude-...`, ID de plataforma de agente de Google Cloud con un sufijo de versión `@`, y nombres de implementación de Microsoft Foundry que contienen un ID de modelo Claude. Claude Code verifica el modelo detrás de un [ARN de perfil de inferencia de aplicación](/docs/es/amazon-bedrock#map-each-model-version-to-an-inference-profile) de Amazon Bedrock en lugar del ARN mismo. No escribe ninguna línea para un ARN que no puede resolver, como uno mal escrito.

**Qué hacer:**

* Si estableció el ID a propósito, como un alias de [LLM gateway](/docs/es/llm-gateway), agregue una entrada [`modelOverrides`](/docs/es/model-config#override-model-ids-per-version) a su [archivo de configuración](/docs/es/settings#where-settings-live) con el ID como su valor. Use un ID de modelo Anthropic como la clave, no un alias de familia como `opus`. Para `my-proxy-model` de la línea de ejemplo, agregue esta entrada:

  ```json theme={null}
  {
    "modelOverrides": {
      "claude-opus-4-6": "my-proxy-model"
    }
  }
  ```

  Claude Code entonces trata `my-proxy-model` como `claude-opus-4-6` y deja de escribir la línea.

* Si el ID nombra un modelo más nuevo que su versión de Claude Code, ejecute `claude update`

* Si el ID es un error tipográfico, corrígalo en cualquiera de los [lugares donde puede establecer un modelo](/docs/es/model-config#setting-your-model) o [variables de alias](/docs/es/model-config#environment-variables) que lo contiene. Si `query_source` comienza con `agent:`, corrígalo donde establece el [modelo del subagente](/docs/es/sub-agents#choose-a-model) en su lugar.

Antes de v2.1.233, Claude Code no escribía ninguna línea cuando enviaba una solicitud para un ID de modelo que no reconocía.

<h3 id="stale-sandbox-mask-files-left-by-a-killed-session">
  Archivos de máscara de sandbox obsoletos dejados por una sesión eliminada
</h3>

`claude doctor` imprime esta advertencia en sus diagnósticos, y `/status` enumera la misma línea. Aparece en Linux y WSL2 cuando [sandboxing](/docs/es/sandboxing) está habilitado con aislamiento del sistema de archivos activado.

Mientras se ejecuta un comando en sandbox, el sandbox mantiene una negación de escritura en un archivo que aún no existe creando un marcador de posición de lectura de 0 bytes de solo lectura allí, y lo elimina después. Una sesión eliminada antes de que se ejecute esa limpieza, por ejemplo por SIGKILL, deja los marcadores de posición atrás. Las sesiones posteriores los vinculan de solo lectura de nuevo en cada inicio, por lo que una escritura de configuración como guardar "Sí, y no preguntar de nuevo" falla donde uno se sienta.

```text theme={null}
- Stale sandbox mask files left by a killed session: /home/you/project/.claude/settings.local.json
  Fix: Remove each with `rm <path>` while no other Claude Code session is running in that project — a 0-byte read-only file where a settings file belongs makes "Yes, and don't ask again" fail to save, and the sandbox binds it read-only again on every start
```

**Qué hacer:**

* Cierre cualquier otra sesión de Claude Code ejecutándose en ese proyecto, luego elimine cada archivo listado con `rm`. La advertencia nombra hasta tres archivos y cuenta el resto, así que reejecutar `claude doctor` después de eliminar hasta que la advertencia ya no aparezca. Un marcador de posición que el sandbox de otra sesión aún está usando es una parte viva de la protección de escritura de esa sesión
* Si una opción de permiso que guardó con "Sí, y no preguntar de nuevo" no se mantuvo, guárdela de nuevo después de eliminar el marcador de posición

Antes de v2.1.257, `claude doctor` no marcaba estos archivos; las versiones anteriores dejan los mismos marcadores de posición atrás cuando se elimina una sesión.

<h2 id="responses-seem-lower-quality-than-usual">
  Las respuestas parecen de menor calidad que lo habitual
</h2>

Si las respuestas de Claude parecen menos capaces de lo que espera pero no se muestra ningún error, la causa suele ser el estado de la conversación en lugar del modelo en sí. Claude Code no cambia silenciosamente las versiones del modelo. Puede cambiar a un modelo de respaldo en tres casos específicos:

* Un [`--fallback-model`](/docs/es/cli-reference#cli-flags) configurado toma el control después de un error de disponibilidad, solo para ese turno, con un aviso en la transcripción
* Una verificación de inicio de Amazon Bedrock o de la plataforma de agentes de Google Cloud encuentra su modelo predeterminado no disponible
* El [respaldo automático de modelo](/docs/es/model-config#automatic-model-fallback) en Fable 5.1, Fable 5, Opus 5.5 y Opus 5 mueve la sesión al modelo de respaldo de la categoría marcada, cuando esa categoría tiene uno, y muestra un aviso en la transcripción

La verificación de selección de modelo a continuación detecta el segundo y tercer caso; el primero aparece como un aviso de transcripción en lugar de un cambio de `/model`. La [configuración del modelo](/docs/es/model-config) explica cuándo se aplica cada respaldo.

Verifique estos primero:

* **Selección de modelo**: ejecute `/model` para confirmar que está en el modelo que espera. Una opción anterior de `/model` o una variable de entorno `ANTHROPIC_MODEL` pueden tenerlo en un modelo más pequeño del que pretendía.
* **Nivel de esfuerzo**: ejecute `/effort` para verificar el nivel de razonamiento actual y auméntelo para depuración difícil o trabajo de diseño. Los valores predeterminados varían según el modelo, así que verifique antes de asumir que está por debajo del máximo. Consulte [Ajustar nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) para los valores predeterminados por modelo y el atajo `ultrathink`.
* **Presión de contexto**: ejecute `/context` para ver qué tan llena está la ventana. Si está cerca de la capacidad, ejecute `/compact` en un punto natural o `/clear` para comenzar de nuevo. Consulte [Explorar la ventana de contexto](/docs/es/context-window) para ver cómo auto-compact afecta los turnos anteriores.
* **Instrucciones obsoletas**: los archivos `CLAUDE.md` grandes u obsoletos y las definiciones de herramientas MCP consumen contexto y pueden dirigir las respuestas. La verificación `/doctor` marca archivos de memoria de gran tamaño y extensiones no utilizadas, y `/context` muestra el uso de tokens de herramientas MCP. Antes de v2.1.205, `/doctor` abría una pantalla de diagnósticos que marcaba archivos de memoria de gran tamaño y definiciones de subagentes.

Cuando una respuesta sale mal, retroceder generalmente funciona mejor que responder con correcciones. Presione Esc dos veces o ejecute `/rewind` para retroceder antes del turno incorrecto, luego reformule el mensaje con más especificidades. Corregir en el hilo mantiene el intento incorrecto en contexto, lo que puede anclar respuestas posteriores a él. Consulte [Checkpointing](/docs/es/checkpointing).

Si la calidad aún parece incorrecta después de verificar lo anterior, ejecute `/feedback` y describa qué esperaba versus qué obtuvo. La retroalimentación enviada de esta manera incluye la transcripción de la conversación, que es la forma más rápida para que Anthropic diagnostique una regresión real. Consulte [Reportar un error](#report-an-error) si `/feedback` no está disponible en su entorno.

Si Claude advierte sobre una inyección de mensaje sospechosa, o rechaza una solicitud debido a una inyección sospechada, y el texto que nombra la advertencia es contexto que Claude Code agrega a la conversación automáticamente en lugar de contenido de archivo o web, ejecute `claude update` e intente de nuevo. Si la advertencia se repite después de actualizar, [repórtela](#report-an-error) en lugar de pegar el contenido marcado nuevamente en el mensaje. Antes de v2.1.201, Sonnet 5 rechazaba algunas solicitudes de la misma manera.

<h2 id="report-an-error">
  Reportar un error
</h2>

Para errores de componentes que esta página no cubre, consulte la guía relevante:

* El servidor MCP no se pudo conectar o autenticar: [MCP](/docs/es/mcp)
* El script de hook falló o bloqueó una herramienta: [Depurar hooks](/docs/es/hooks#debug-hooks)
* Permiso denegado o errores del sistema de archivos durante la instalación: [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install)

Si un error no aparece aquí o la corrección sugerida no ayuda:

* Ejecute `/feedback` dentro de Claude Code para enviar la transcripción y una descripción a Anthropic. El comando también ofrece abrir un problema de GitHub rellenado previamente. El envío a Anthropic requiere [autenticación](/docs/es/authentication). En Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry y otros proveedores de terceros, o cuando no hay credenciales de Anthropic configuradas, `/feedback` guarda un archivo local que puede enviar a su representante de cuenta de Anthropic en su lugar.
* Ejecute `claude doctor` desde su shell para un diagnóstico de solo lectura de su instalación, o ejecute la verificación `/doctor` dentro de Claude Code para encontrar y solucionar problemas de configuración
* Consulte [status.claude.com](https://status.claude.com) para incidentes activos
* Busque [problemas existentes](https://github.com/anthropics/claude-code/issues) en GitHub
