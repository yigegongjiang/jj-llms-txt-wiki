> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência de erros

> Procure mensagens de erro de tempo de execução do Claude Code com o que cada uma significa e como corrigi-la.

Esta página lista erros de tempo de execução que o Claude Code exibe e como se recuperar de cada um, além do que verificar quando as respostas parecem estar erradas sem um erro. Para erros de instalação como `command not found` ou falhas de TLS durante a configuração, consulte [Solucionar problemas de instalação e login](/docs/pt/troubleshoot-install).

Exceto pelos [erros de Wrapper e IDE](#wrapper-and-ide-errors), que o programa de inicialização imprime em vez do próprio Claude Code, esses erros e comandos de recuperação se aplicam em toda a CLI, ao [aplicativo Desktop](/docs/pt/desktop) e [sessões em nuvem](/docs/pt/claude-code-on-the-web), já que todos os três envolvem a mesma CLI do Claude Code. Para outros problemas específicos da superfície, consulte a seção de solução de problemas na página dessa superfície.

<Note>
  Claude Code chama a API Claude para respostas do modelo, portanto, a maioria dos erros de tempo de execução mapeia para um código de erro de API subjacente. Esta página cobre o que cada erro significa dentro do Claude Code e como se recuperar. Para as definições de código de status HTTP bruto, consulte a [referência de erro da Plataforma Claude](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Encontre seu erro
</h2>

Corresponda a mensagem que você vê a uma seção abaixo.

| Mensagem                                                                                                                                                                                                                                                             | Seção                                                                                                                           |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `API Error: 500 Internal server error`                                                                                                                                                                                                                               | [Server errors](#api-error-500-internal-server-error)                                                                           |
| `API Error: Repeated 529 Overloaded errors`                                                                                                                                                                                                                          | [Server errors](#api-error-repeated-529-overloaded-errors)                                                                      |
| `Request timed out`                                                                                                                                                                                                                                                  | [Server errors](#request-timed-out), ou [Network](#unable-to-connect-to-api) se a mensagem mencionar sua conexão com a internet |
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
| `Connection refused —` / `Can't reach the API server —` / `No internet route —` / `Couldn't connect through your proxy` / `Connection dropped`, cada um com um código de erro entre parênteses                                                                       | [Network](#unable-to-connect-to-api)                                                                                            |
| `Unable to connect to Anthropic services` durante a configuração                                                                                                                                                                                                     | [Network](#unable-to-connect-to-anthropic-services)                                                                             |
| `Socket is closed`                                                                                                                                                                                                                                                   | [Network](#socket-is-closed)                                                                                                    |
| `Waiting for API response · will retry in`                                                                                                                                                                                                                           | [Automatic retries](#automatic-retries), ou [Network](#unable-to-connect-to-api) se persistir                                   |
| `API returned an empty or malformed response`                                                                                                                                                                                                                        | [Network](#api-returned-an-empty-or-malformed-response)                                                                         |
| `Streaming response ended before any complete data was received`                                                                                                                                                                                                     | [Network](#streaming-response-ended-before-any-complete-data-was-received)                                                      |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"`                                                                                                                                                                   | [Network](#bedrock-streaming-response-has-an-unexpected-content-type)                                                           |
| `SSL certificate verification failed`                                                                                                                                                                                                                                | [Network](#ssl-certificate-errors)                                                                                              |
| `SSL certificate error (...)` durante login ou inicialização                                                                                                                                                                                                         | [Network](#ssl-certificate-errors)                                                                                              |
| `unable to get local issuer certificate`                                                                                                                                                                                                                             | [Network](#ssl-certificate-errors)                                                                                              |
| `403` com `x-deny-reason: host_not_allowed` em uma sessão de nuvem ou rotina                                                                                                                                                                                         | [Network](#host-not-allowed-in-a-cloud-session)                                                                                 |
| `proxy refused the connection`                                                                                                                                                                                                                                       | [Network](#the-proxy-refused-the-connection)                                                                                    |
| `403` com `This GraphQL query is not enabled for this session` em uma sessão de nuvem                                                                                                                                                                                | [GitHub proxy](/docs/pt/cloud-environments#github-proxy)                                                                             |
| `The cloud environments service returned an empty response` / `The cloud environments service returned a response in an unexpected format`                                                                                                                           | [Network](#the-cloud-environments-service-returned-an-empty-or-unexpected-response)                                             |
| `Couldn't reconnect to your Remote Control session`                                                                                                                                                                                                                  | [Network](#couldnt-reconnect-to-your-remote-control-session)                                                                    |
| `N sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.`                                                                                                                                               | [Network](#sessions-ended-while-this-machine-was-offline)                                                                       |
| `Couldn't share the transcript.`                                                                                                                                                                                                                                     | [Network](#couldnt-share-the-transcript)                                                                                        |
| `Prompt is too long` / `Input is too long for requested model`                                                                                                                                                                                                       | [Request errors](#prompt-is-too-long)                                                                                           |
| `Prompt is too long · automatic compaction failed:`                                                                                                                                                                                                                  | [Request errors](#prompt-is-too-long)                                                                                           |
| `Prompt is too long · this conversation is a single exchange` / `A single-exchange conversation cannot be compacted`                                                                                                                                                 | [Request errors](#prompt-is-too-long)                                                                                           |
| `Context limit reached · /compact or /clear to continue`                                                                                                                                                                                                             | [Request errors](#prompt-is-too-long)                                                                                           |
| `Context limit reached · /clear to continue`                                                                                                                                                                                                                         | [Request errors](#prompt-is-too-long)                                                                                           |
| `capability_rejected: prompt_too_long` em uma sessão de gateway de aplicativos Claude                                                                                                                                                                                | [Request errors](#prompt-is-too-long)                                                                                           |
| `upstream rejected the request` / `request too large for this upstream` em uma sessão de gateway de aplicativos Claude                                                                                                                                               | [Upstream error messages](/docs/pt/claude-apps-gateway-config#upstream-error-messages)                                               |
| `upstream rate limit exceeded` em uma sessão de gateway de aplicativos Claude                                                                                                                                                                                        | [Upstream error messages](/docs/pt/claude-apps-gateway-config#upstream-error-messages)                                               |
| `all upstreams failed (N attempted)` em uma sessão de gateway de aplicativos Claude                                                                                                                                                                                  | [Upstream error messages](/docs/pt/claude-apps-gateway-config#upstream-error-messages)                                               |
| `Claude Code may not be enabled for your organization` após um login de gateway de aplicativos Claude                                                                                                                                                                | [Claude apps gateway troubleshooting](/docs/pt/claude-apps-gateway-deploy#troubleshooting)                                           |
| `Context exceeds the ...-token limit by ... tokens` na saída `/context`                                                                                                                                                                                              | [Request errors](#context-exceeds-the-token-limit)                                                                              |
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
| `server_tool_use.name: Input should be` em cada turno de uma sessão retomada                                                                                                                                                                                         | [Request errors](#unsupported-tool-content-removed)                                                                             |
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
| `Error: Workspace not trusted` ao iniciar Remote Control                                                                                                                                                                                                             | [Command-line errors](#workspace-not-trusted-when-starting-remote-control)                                                      |
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
| `Shell command failed for pattern "..."`, de `/security-review` ou qualquer skill que injete contexto dinâmico                                                                                                                                                       | [Command-line errors](#security-review-fails-without-origin-head)                                                               |
| `Shell command permission check failed for pattern "..."`, de um skill que injete contexto dinâmico                                                                                                                                                                  | [Command-line errors](#security-review-fails-without-origin-head)                                                               |
| ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``                                                                                                                                                                             | [Command-line errors](#security-review-fails-without-origin-head)                                                               |
| `Input must be provided either through stdin or as a prompt argument when using --print`                                                                                                                                                                             | [Command-line errors](#input-must-be-provided-when-using-print)                                                                 |
| `Error: Input contained only whitespace`                                                                                                                                                                                                                             | [Command-line errors](#input-contained-only-whitespace)                                                                         |
| `Blank prompt — the message was only whitespace, so nothing was sent to the model.`                                                                                                                                                                                  | [Command-line errors](#input-contained-only-whitespace)                                                                         |
| `Error: stream-json input carried over 256M characters with no newline`                                                                                                                                                                                              | [Command-line errors](#stream-json-input-carried-over-256m-characters-with-no-newline)                                          |
| `Unknown command: /<name>`, com ou sem uma sugestão `Did you mean`                                                                                                                                                                                                   | [Command-line errors](#unknown-command)                                                                                         |
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
| As respostas parecem ter qualidade inferior ao normal                                                                                                                                                                                                                | [Response quality](#responses-seem-lower-quality-than-usual)                                                                    |

<h2 id="automatic-retries">
  Tentativas automáticas
</h2>

Claude Code tenta novamente falhas transitórias até 10 vezes com backoff exponencial antes de mostrar um erro. Nem sempre tenta novamente uma falha que chega no meio da resposta do Claude. Quando você vê um dos erros nesta página, Claude Code já fez as tentativas que se aplicam a essa falha; as listas abaixo dizem quais falhas recebem o orçamento completo, quais recebem um menor e quais não recebem nenhum.

Claude Code tenta novamente estas falhas:

* Erros de servidor, respostas sobrecarregadas e timeouts de solicitação que chegam antes de qualquer parte da resposta do Claude ter sido transmitida.
* Conexões perdidas. Quando uma conexão cai no meio de uma solicitação antes de Claude ter completado qualquer parte de sua resposta, incluindo seu thinking, Claude Code reemite a solicitação com o mesmo backoff e a rodada continua, mesmo que algum texto já tivesse começado a ser transmitido. Quando cai depois que Claude terminou de pensar mas antes de ter iniciado qualquer texto ou chamada de ferramenta, Claude Code em vez disso reemite a solicitação até duas vezes em rápida sucessão, e encerra a rodada com `Connection lost before a response was produced` se a conexão continuar caindo nesse ponto.
* Uma conexão que Claude Code detecta foi quebrada pelo seu computador entrando em modo de suspensão no meio de uma solicitação. Claude Code a conta como uma conexão perdida sob as regras acima; uma vez que o rótulo de tentativa nomeia a razão específica, ele lê `Connection lost while your computer was asleep`, e se a rodada terminar depois que Claude terminou de pensar mas antes de qualquer texto ou chamada de ferramenta, a mensagem lê `Your computer went to sleep before a response was produced`.
* Um fluxo de resposta travado, quando os cabeçalhos de resposta chegaram mas nenhuma parte da resposta do Claude chegou, ou quando Claude terminou de pensar mas não iniciou qualquer texto ou chamada de ferramenta: Claude Code aborta a conexão travada e reemite a solicitação no máximo uma vez, fora do orçamento de 10 tentativas acima. Se a resposta travar uma segunda vez depois que Claude terminou de pensar mas antes de qualquer texto ou chamada de ferramenta, Claude Code encerra a rodada com `The response stalled before a response was produced`.
* Uma solicitação de streaming que a API nunca responde com cabeçalhos de resposta, em uma conexão onde o [prazo de primeiro byte é executado](/docs/pt/network-config#streaming-idle-watchdogs): Claude Code a aborta no prazo e a reenvia no máximo uma vez por solicitação de modelo, dentro do orçamento de tentativas, depois encerra a rodada com [No response from API](#no-response-from-api) se essa tentativa também ficar sem resposta. Em outras conexões, a solicitação aguarda `API_TIMEOUT_MS`. Quando você define `CLAUDE_CODE_RETRY_WATCHDOG`, o limite de uma tentativa não se aplica.
* Throttles 429 temporários, mas não o `429` de limite de gastos de um gateway, que não é um throttle; veja [Spend limit reached](#spend-limit-reached).
  * Quando você está conectado com uma assinatura claude.ai, isso inclui throttles 429 que não carregam os cabeçalhos de cota do seu plano. Antes da v2.1.199, Claude Code tentava novamente esses throttles apenas para assinatura de chave de API e Enterprise.
* Uma solicitação rejeitada porque a entrada mais `max_tokens` excede o limite de contexto. Reenviar sem alterações falharia da mesma forma, então Claude Code tenta novamente com um `max_tokens` reduzido, e para de tentar novamente e compacta em dois casos:
  * Quando nenhuma redução pode caber, por exemplo quando a conversa em si quase preenche a janela de contexto.
  * Quando uma tentativa não pode encolher `max_tokens` ainda mais. Antes da v2.1.218, Claude Code poderia reenviar uma solicitação reduzida que ainda não cabia, como quando o orçamento de thinking estendido excedia o contexto restante, até o orçamento de tentativas se esgotar.
* Uma credencial expirada ou ausente do Google Cloud na [Plataforma de Agentes do Google Cloud](/docs/pt/google-vertex-ai), ou credenciais AWS que falham ao carregar em sua máquina. Claude Code descarta suas credenciais em cache e tenta novamente até duas vezes, depois relata o erro para que você possa se autenticar novamente imediatamente, conforme descrito em [Could not load AWS or Google Cloud credentials](#could-not-load-aws-or-google-cloud-credentials). Antes da v2.1.228, Claude Code tentava novamente uma credencial do Google Cloud falhando através do orçamento de tentativas completo antes de mostrar o erro.
* Um `401` ou `403` da API Anthropic, diretamente ou através de um [gateway LLM](/docs/pt/llm-gateway), enquanto um script [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) fornece a credencial. Claude Code executa novamente o script e tenta novamente com sua saída fresca, dentro do orçamento de tentativas completo. Quando o script em si falha na re-execução, Claude Code mostra [Your apiKeyHelper script is failing](#your-apikeyhelper-script-is-failing) em vez disso.

Antes da v2.1.227, `Connection lost before a response was produced` lia `Connection closed while thinking, before producing a response` e `The response stalled before a response was produced` lia `Response stalled while thinking, before producing a response`.

Claude Code não tenta novamente estas falhas:

* Uma falha de validação de certificado TLS, como um proxy inspecionando TLS, um pacote `NODE_EXTRA_CA_CERTS` ausente, ou um certificado expirado. Claude Code relata o erro na primeira tentativa, para que você possa corrigir a configuração do certificado imediatamente; veja [SSL certificate errors](#ssl-certificate-errors). Claude Code ainda tenta novamente condições TLS transitórias como um timeout de handshake. Antes da v2.1.199, Claude Code tentava novamente falhas de certificado através do orçamento de tentativas completo antes de mostrar o erro.
* Um erro de servidor, conexão perdida, ou fluxo travado que chega depois que Claude completou um bloco de texto ou uma chamada de ferramenta, ou iniciou um depois de terminar seu thinking, mas antes de terminar a resposta. Claude Code não executa novamente a solicitação, porque isso poderia executar as mesmas chamadas de ferramenta duas vezes. Ele mantém o que Claude completou, executa qualquer chamada de ferramenta que Claude terminou, e continua a rodada a partir de seus resultados. Para o que você vê em uma sessão interativa e em uma não-interativa, leia [The response above may be incomplete](#the-response-above-may-be-incomplete). Antes da v2.1.199, Claude Code descartava a saída parcial e relatava toda a rodada como um erro quando um erro de servidor chegava no meio do fluxo.
* Uma falha que chega depois que Claude terminou a resposta: nada precisa ser tentado novamente, então Claude Code mantém a resposta completa e encerra a rodada normalmente.
* Uma [resposta de streaming do Amazon Bedrock com um tipo de conteúdo inesperado](#bedrock-streaming-response-has-an-unexpected-content-type), porque o gateway ou proxy reescrevendo a resposta reescreveria a tentativa da mesma forma. Requer Claude Code v2.1.208 ou posterior.
* Uma tentativa não-streaming de uma solicitação de streaming falhada que recebe um status de sucesso mas [nenhuma mensagem de API Claude no corpo](#api-returned-an-empty-or-malformed-response). Claude Code encerra a rodada com esse erro.
* Uma solicitação que a verificação de política da sua organização negou, que aparece como uma linha `API Error:` carregando a mensagem de negação. Os administradores da sua organização configuraram a verificação com [Inference hooks](https://platform.claude.com/docs/en/manage-claude/inference-hooks), um recurso Claude Enterprise, e a mensagem termina com as instruções que eles configuraram, ou por padrão diz para você entrar em contato com eles. Claude Code não reenvia a solicitação negada para o mesmo modelo ou para um [modelo de fallback](/docs/pt/model-config#fallback-model-chains), porque a negação é sobre o conteúdo da solicitação em vez do modelo. Antes da v2.1.239, Claude Code poderia reenviar uma solicitação negada, sem streaming ou em um modelo de fallback configurado, antes de mostrar a negação.

<h3 id="what-you-see-while-claude-code-retries-or-waits">
  O que você vê enquanto Claude Code tenta novamente ou aguarda
</h3>

Ao tentar novamente, o spinner mostra uma contagem regressiva `Retrying in Ns · attempt x/y` após um rótulo de erro. O rótulo nomeia a razão específica da primeira tentativa para falhas que você pode agir imediatamente: a rede está inativa, um handshake TLS falhou, ou você atingiu um limite de taxa. Para outros erros, ele lê `API error` no início. A partir da v2.1.198, ele muda para a razão específica da terceira tentativa, ou na tentativa final quando `CLAUDE_CODE_MAX_RETRIES` permite menos de três; versões anteriores mudam apenas na tentativa final.

A partir da v2.1.198, a dica de spinner usual é suprimida durante tentativas. Uma vez que a razão do erro é revelada, se a falha é uma sobrecarga 529, a linha abaixo da contagem regressiva também nomeia onde verificar o status do serviço: `status.claude.com` na API Anthropic, ou o host do provedor ou gateway nomeado na mensagem em outras configurações.

Se nenhum dado chegar no fluxo de resposta por 20 segundos enquanto uma solicitação ainda está pendente, o spinner mostra `Waiting for API response · will retry in … · check your network` antes de qualquer tentativa ter começado. A solicitação ainda não falhou: a contagem regressiva é executada até o ponto onde Claude Code aborta a conexão travada. Após o aborto, o que você vê depende de quão longe a resposta tinha chegado:

* Antes de Claude ter completado um bloco de texto ou uma chamada de ferramenta, ou iniciado um depois de terminar seu thinking, Claude Code tenta novamente a solicitação ou encerra a rodada com um erro. [Automatic retries](#automatic-retries) diz quais travamentos ele tenta novamente e quantas vezes.
* Depois que Claude completou um bloco de texto ou uma chamada de ferramenta, ou iniciou um depois de terminar seu thinking, mas antes de Claude ter terminado a resposta, Claude Code mantém o que Claude completou, continua a rodada a partir de qualquer chamada de ferramenta que Claude terminou, e mostra [The response above may be incomplete](#the-response-above-may-be-incomplete). Em uma sessão não-interativa, e para a resposta de um subagenteemqualquer sessão, Claude Code pode primeiro solicitar ao Claude para continuar a resposta; essa entrada diz quando faz isso e quando você ainda vê o aviso lá.
* Depois que Claude terminou a resposta, Claude Code encerra a rodada normalmente.

O banner se limpa automaticamente uma vez que os dados retomam ou uma tentativa é bem-sucedida. Se reaparecer em cada tentativa, trate-o como um [problema de rede](#unable-to-connect-to-api). Antes da v2.1.185, o banner aparecia após 10 segundos com redação diferente.

Enquanto Claude está consultando o [advisor](/docs/pt/advisor), o banner aparece após 90 segundos sem dados em vez de 20, porque uma revisão longa do advisor pode não enviar nada por bem mais de 20 segundos. Antes da v2.1.214, o limite de 20 segundos se aplicava durante chamadas de advisor também, então o banner aparecia durante revisões de advisor mesmo quando nada estava errado.

<h3 id="tune-retry-behavior">
  Ajustar comportamento de tentativa
</h3>

Você pode ajustar o comportamento de tentativa com estas variáveis de ambiente:

| Variável                                              | Padrão       | Efeito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------------------------------------------- | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/pt/env-vars)             | 10           | Número de tentativas de tentativa. Limitado a 15 a partir da v2.1.186; a partir da v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` aumenta o padrão e remove o limite. Reduza-o para superficializar falhas mais rapidamente em scripts.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/pt/env-vars)          | não definido | Defina como `1` em sessões não-supervisionadas como trabalhos de CI para tentar novamente `429` e `529` erros de capacidade indefinidamente em vez de falhar após `CLAUDE_CODE_MAX_RETRIES` tentativas. Claude Code falha imediatamente quando uma solicitação de velocidade padrão recebe um `429` que relata um limite de gastos ou créditos de uso esgotados, mesmo um de um [limite de gastos de gateway](#spend-limit-reached) que redefine em um cronograma. Antes da v2.1.239, o watchdog tentava novamente esses indefinidamente. Para solicitações de modo rápido, veja [Handle rate limits](/docs/pt/fast-mode#handle-rate-limits). Na v2.1.199 ou posterior, também aumenta a contagem de tentativas padrão para outros erros transitórios, como erros de servidor, timeouts e conexões perdidas, para 300, aproximadamente três horas de backoff, e remove o limite de 15 em `CLAUDE_CODE_MAX_RETRIES` se você definir essa variável explicitamente. |
| [`API_TIMEOUT_MS`](/docs/pt/env-vars)                      | 600000       | Timeout por solicitação em milissegundos. Aumente-o para redes lentas ou proxies. Também limita quanto tempo Claude Code aguarda cabeçalhos de resposta, descrito em [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/pt/env-vars) | não definido | Prazo em milissegundos para o primeiro byte de resposta de uma solicitação de streaming. Requer Claude Code v2.1.242 ou posterior. Para como Claude Code escolhe o prazo quando isso não está definido, veja [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

<h2 id="server-errors">
  Erros do servidor
</h2>

A maioria desses erros vem do provedor de inferência: o serviço da Anthropic na API Anthropic e o serviço por trás do endpoint desse provedor no Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou um gateway personalizado. [Auto mode cannot determine the safety of an action](#auto-mode-cannot-determine-the-safety-of-an-action) e [Agent terminated early due to an API error](#agent-terminated-early-due-to-an-api-error) também cobrem causas do seu lado, como uma conta Amazon Bedrock que não consegue invocar o modelo classificador ou um subagent que atingiu um limite de uso.

<h3 id="api-error-500-internal-server-error">
  API Error: 500 Internal server error
</h3>

Claude Code mostra o código de status e a mensagem de erro da API para qualquer resposta 5xx. O exemplo abaixo mostra uma resposta 500 na API Anthropic:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

A frase final nomeia onde verificar a saúde do serviço e varia por provedor. As configurações do Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry nomeiam a página de status desse provedor. Um `ANTHROPIC_BASE_URL` personalizado nomeia o host do gateway.

Isso indica uma falha inesperada dentro da API. Não é causado pelo seu prompt, configurações ou conta.

**O que fazer:**

* Verifique [status.claude.com](https://status.claude.com) ou a página de status do provedor nomeada na mensagem para incidentes ativos
* Aguarde um minuto e envie sua mensagem novamente. Sua mensagem original ainda está na conversa, então para um prompt longo você pode digitar `try again` em vez de colar tudo novamente.
* Se o erro persistir sem nenhum incidente postado, execute `/feedback` para que a Anthropic possa investigar com os detalhes da sua solicitação. Veja [Report an error](#report-an-error) se `/feedback` não estiver disponível no seu ambiente.

<h3 id="api-error-repeated-529-overloaded-errors">
  API Error: Repeated 529 Overloaded errors
</h3>

A API está temporariamente em capacidade máxima em todos os usuários. Claude Code já tentou novamente várias vezes antes de mostrar esta mensagem:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

A frase final varia por provedor da mesma forma que o erro 500 acima.

Um 529 não é seu limite de uso e não conta contra sua cota.

**O que fazer:**

* Verifique [status.claude.com](https://status.claude.com) ou a página de status do provedor nomeada na mensagem para avisos de capacidade
* Tente novamente em alguns minutos
* Execute `/model` e mude para um modelo diferente para continuar trabalhando, já que a capacidade é rastreada por modelo. Claude Code o solicita fazer isso quando um modelo está sob carga particularmente alta, por exemplo `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Request timed out
</h3>

A API não respondeu antes do prazo de conexão.

```text theme={null}
Request timed out
```

Isso pode acontecer durante períodos de alta carga ou quando o modelo está gerando uma resposta muito grande. O tempo limite de solicitação padrão é de 10 minutos.

**O que fazer:**

* Tente novamente a solicitação
* Para tarefas de longa duração, divida o trabalho em prompts menores
* Se uma rede lenta ou proxy for a causa, aumente `API_TIMEOUT_MS` conforme descrito em [Automatic retries](#automatic-retries)
* Se os tempos limite forem frequentes e sua rede estiver saudável, veja [Network and connection errors](#network-and-connection-errors) abaixo

<h3 id="no-response-from-api">
  No response from API
</h3>

Claude Code enviou uma solicitação de streaming e a API não retornou cabeçalhos de resposta dentro do prazo para o primeiro byte, então Claude Code abortou a solicitação em vez de aguardar o tempo limite de solicitação completo `API_TIMEOUT_MS`, 10 minutos por padrão. Claude Code envia a solicitação novamente no máximo uma vez, se o [retry budget](#tune-retry-behavior) permitir. Quando a tentativa novamente fica sem resposta, o turno termina com esta mensagem, que mostra quanto tempo cada tentativa aguardou. Quando você define [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/pt/env-vars), o limite de uma tentativa não se aplica e Claude Code tenta novamente sob o orçamento descrito em [Tune retry behavior](#tune-retry-behavior).

```text theme={null}
API Error: No response from API (waited 3m, then 10m on the retry). If a proxy or gateway on your network holds responses until they complete, raise API_TIMEOUT_MS or CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS to wait longer.
```

Claude Code define o tempo de espera para cabeçalhos de resposta da primeira tentativa e a espera da tentativa novamente separadamente:

* **Primeira tentativa**: [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/pt/env-vars) quando você o define como 1 ou mais, limitado entre 10 segundos e 30 minutos. Caso contrário, Claude Code usa o tempo limite do watchdog de nível de byte listado em [Streaming idle watchdogs](/docs/pt/network-config#streaming-idle-watchdogs), então as variáveis que alteram esse tempo limite alteram essa espera também. De qualquer forma, Claude Code adiciona um segundo para cada 32KB do corpo da solicitação.
* **Tentativa novamente**: um segundo a menos que `API_TIMEOUT_MS`, pouco menos de 10 minutos por padrão, para que a tentativa novamente possa durar mais que um proxy ou gateway que mantém a resposta até que a geração seja concluída. No Amazon Bedrock, a tentativa novamente usa o mesmo prazo que a primeira tentativa, e a mensagem mostra uma duração em vez de duas.

Nenhuma espera excede um segundo a menos que um `API_TIMEOUT_MS` positivo, e um `API_TIMEOUT_MS` positivo inferior a 11 segundos desativa o prazo. O watchdog de nível de byte começa apenas depois que os cabeçalhos de resposta chegam, então uma resposta que para de enviar bytes depois disso segue as [regras de fluxo interrompido](#automatic-retries) em vez deste prazo.

**O que fazer:**

* Envie sua mensagem novamente. Sua mensagem original ainda está na conversa, então para um prompt longo você pode digitar `try again` em vez de colar tudo novamente.
* Se se repetir, trate como um [problema de rede ou proxy](#unable-to-connect-to-api). Um proxy que aceita a conexão e nunca encaminha a solicitação produz este erro em cada tentativa.
* Se um proxy ou gateway em sua rede mantém respostas até que sejam concluídas, aumente `API_TIMEOUT_MS` para que a tentativa novamente aguarde mais. No Amazon Bedrock, aumente `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` também.
* Se a primeira tentativa continuar expirando e a tentativa novamente tiver sucesso, aumente `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` para que a primeira tentativa também aguarde o tempo suficiente.

Antes da v2.1.242, Claude Code aguardava o tempo limite de solicitação completo `API_TIMEOUT_MS`, 10 minutos por padrão, antes de falhar em uma solicitação de streaming sem resposta. Antes da v2.1.261, a tentativa novamente aguardava o mesmo prazo que a primeira tentativa e a mensagem não mostrava durações.

<h3 id="the-response-above-may-be-incomplete">
  The response above may be incomplete
</h3>

Uma solicitação de streaming falhou enquanto a resposta ainda estava em andamento, depois que Claude completou um bloco de texto ou uma chamada de ferramenta, ou havia iniciado um após terminar seu pensamento. Reenviar a solicitação pode executar as mesmas chamadas de ferramenta duas vezes, então Claude Code mantém a saída que Claude completou e anexa este aviso em vez de descartar o turno. Qual variante você vê nomeia a causa:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection lost mid-response. The response above may be incomplete.
API Error: Your computer went to sleep mid-response. The response above may be incomplete.
API Error: The response stopped arriving. The response above may be incomplete.
```

* `Server error mid-response`: um erro de servidor sobrecarregado ou 5xx no meio do fluxo. Esta variante requer Claude Code v2.1.199 ou posterior; antes disso, esse caso descartava a saída parcial e relatava todo o turno como um erro.
* `Connection lost mid-response`: a conexão foi interrompida.
* `Your computer went to sleep mid-response`: Claude Code detectou que seu computador entrou em modo de suspensão enquanto a resposta estava sendo transmitida. Depois que seu computador acordar, Claude Code trata a conexão como quebrada e para de ler dela.
* `The response stopped arriving`: a conexão permaneceu aberta mas parou de entregar dados, então o watchdog de inatividade de streaming a abortou. Antes da v2.1.222, Claude Code também poderia relatar essa falha em conexões de [gateway](/docs/pt/gateways) alcançadas através de `ANTHROPIC_BASE_URL` ou `ANTHROPIC_AWS_BASE_URL` enquanto os pings de keep-alive do servidor ainda estavam chegando, porque contava apenas eventos de resposta analisados lá; atualizar para a versão mais recente interrompe esses tempos limite espúrios nessas rotas. Gateways alcançados através de uma URL de base de provedor como `ANTHROPIC_BEDROCK_BASE_URL` não são envolvidos pelo watchdog de byte; veja [Streaming idle watchdogs](/docs/pt/network-config#streaming-idle-watchdogs).

Antes da v2.1.227, `Connection lost mid-response` lia `Connection closed mid-response` e `The response stopped arriving` lia `Response stalled mid-stream`.

Em quatro casos, Claude Code lida com a falha sem mostrar este aviso imediatamente:

* Anteriormente na resposta, Claude Code ou tenta novamente a falha ou termina o turno com um erro diferente. Veja [Automatic retries](#automatic-retries).
* Quando uma dessas falhas chega depois que Claude terminou a resposta, Claude Code mantém a resposta completa e termina o turno normalmente, sem este aviso. Antes da v2.1.222, Claude Code mostrava este aviso quando a conexão era interrompida ou travava após a resposta terminar, e relatava o turno como um erro mesmo que a resposta fosse completa.
* Em uma [sessão não interativa](/docs/pt/headless), como uma execução `-p`, uma execução do [Agent SDK](/docs/pt/agent-sdk/overview) ou uma [sessão em nuvem](/docs/pt/claude-code-on-the-web), você não precisa enviar `continue` você mesmo quando a resposta cortada está na conversa principal e contém texto mas nenhuma chamada de ferramenta: Claude Code mantém a saída parcial e solicita a Claude continuar de onde parou, até três vezes seguidas. Você vê este aviso para tal resposta apenas uma vez que Claude Code tenha usado essas continuações. Antes da v2.1.246, Claude Code terminava um turno não interativo com este aviso na primeira interrupção.
* Em um [subagent](/docs/pt/sub-agents#api-errors-in-subagents), seja a sessão interativa ou não: quando sua resposta cortada contém texto mas nenhuma chamada de ferramenta, Claude Code solicita ao subagent continuar. O aviso se torna a última mensagem do subagent apenas uma vez que essas continuações sejam usadas. Antes da v2.1.257, um subagent mostrava este aviso na primeira interrupção.

**O que fazer:**

* Em uma sessão interativa, leia a resposta que permanece na tela: Claude Code mantém cada bloco que Claude completou antes do erro, mas descarta um bloco final interrompido quando o turno termina, então as frases ou chamadas de ferramenta finais podem estar faltando. Responda com `continue` para que Claude retome seu último bloco completado.
* Em [modo não interativo](/docs/pt/headless) (`-p`):
  * Com a saída de texto padrão, Claude Code imprime o último bloco de texto completado que ainda mantém de antes no turno, seguido por esta mensagem. Quando não mantém nenhum, Claude Code imprime apenas esta mensagem, por exemplo porque Claude Code compactou a conversa no meio do turno e limpou esse texto. Antes da v2.1.219, Claude Code imprimia apenas esta mensagem na saída de texto `-p` e descartava a resposta que já havia produzido.
  * Com `--output-format json` ou `stream-json`, Claude Code relata esta mensagem no campo `result`.
  * Para continuar o turno uma vez que a conexão esteja estável, retome a sessão e envie `continue` conforme descrito em [Continue conversations](/docs/pt/headless#continue-conversations).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto mode cannot determine the safety of an action
</h3>

O modelo que [auto mode](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) usa para classificar ações não conseguiu produzir uma decisão, então auto mode não aprovou a ação automaticamente. A mensagem que você vê depende de como o classificador falhou.

Leituras, buscas e edições dentro do seu diretório de trabalho pulam o classificador, então continuam funcionando em todos esses casos.

Quando o modelo classificador está indisponível:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait a moment and then try this action again.
```

Quando Claude Code pode determinar a categoria de falha, ele nomeia a categoria entre parênteses após `temporarily unavailable`, por exemplo `<model> is temporarily unavailable (rate-limited), so auto mode cannot determine the safety of <tool> right now`. As categorias são `(rate-limited)`, `(overloaded)`, `(server error)`, `(timed out)` e `(connection failed)`. Taxa limitada, sobrecarregado e erros de servidor são transitórios, e tentar novamente funciona. Se `(timed out)` ou `(connection failed)` se repetir, verifique sua conexão; veja [Unable to connect to API](#unable-to-connect-to-api). Antes da v2.1.229, a mensagem nunca nomeava uma categoria e lia `Wait briefly and then try this action again`.

Quando nenhuma categoria se encaixa, a mensagem aparece sem categoria entre parênteses; mais de uma falha produz essa forma. No [Amazon Bedrock](/docs/pt/amazon-bedrock), incluindo o [endpoint Mantle](/docs/pt/amazon-bedrock#use-the-mantle-endpoint), também aparece quando sua conta AWS não consegue invocar o modelo nomeado na mensagem, e essa falha se repete em cada tentativa até que sua conta receba acesso ao modelo.

**O que fazer:**

* Tente novamente após alguns segundos; Claude vê a mesma mensagem e geralmente tenta novamente por conta própria. Uma falha transitória não está relacionada à [elegibilidade de auto mode](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode); você não precisa alterar configurações
* Se as tentativas continuarem falhando, continue com tarefas somente leitura e volte à ação bloqueada mais tarde
* No Amazon Bedrock, se a mensagem retornar em cada tentativa, verifique se sua conta pode invocar o modelo que ela nomeia: para modelos padrão do Amazon Bedrock, confirme que sua [política IAM](/docs/pt/amazon-bedrock#iam-configuration) permite invocá-lo; para IDs de modelo Mantle, [entre em contato com sua equipe de conta AWS](/docs/pt/amazon-bedrock#mantle-endpoint-errors)

Quando uma solicitação de classificador falha porque seu token OAuth expirou ou foi rotacionado por outra sessão, Claude Code atualiza o token e tenta novamente a solicitação uma vez, então uma expiração de token rotineira não aparece como esta mensagem. Antes da v2.1.216, um token expirado ou rotacionado falhava em cada solicitação de classificador, e auto mode negava cada ação verificada com esta mensagem até que o token fosse atualizado.

Quando o classificador retornou uma resposta não analisável:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**O que fazer:**

* Tente novamente a ação; isso geralmente tem sucesso na próxima tentativa
* Execute `claude --debug` e repita a ação para ver a resposta do classificador subjacente no log de depuração

Quando uma verificação de segurança de API separada bloqueou a solicitação do classificador por causa do conteúdo anterior da conversa:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

Claude Code nega a ação mas diz a Claude que isso não é um julgamento de que a ação é insegura, e continuar com outras tarefas em vez de tentar novamente. Essas negações não contam para [limites de pausa de auto mode](/docs/pt/permission-modes#when-auto-mode-falls-back). Em uma execução [`-p` não interativa](/docs/pt/headless), Claude Code não interrompe a execução. O que Claude recebe depende de onde solicitou a ação:

* Para um [subagent em segundo plano](/docs/pt/sub-agents#run-subagents-in-foreground-or-background) em uma execução `-p` sem `--input-format stream-json`, Claude Code retorna um resultado de erro contendo `Agent aborted: auto mode classifier request refused by the safety safeguard in headless mode`
* Em todos os outros lugares, incluindo sessões interativas e a conversa principal de uma execução `-p`, Claude Code retorna essa negação a Claude

Antes da v2.1.225, Claude Code contava essas recusas para os limites de pausa e retornava a mesma mensagem de rejeição que um bloco de classificador genuíno.

**O que fazer:**

* Isso não é uma decisão sobre sua ação. O conteúdo já em sua conversa acionou um filtro de segurança na API quando auto mode enviou a conversa para o classificador
* Tentar novamente não ajudará; o mesmo conteúdo de conversa acionará o filtro novamente
* Em uma sessão interativa, mude para um [modo de permissão](/docs/pt/permission-modes) diferente para que você possa aprovar a ação quando solicitado
* Inicie uma conversa nova sem o conteúdo acionador

Quando a conversa cresceu além da janela de contexto do classificador:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

O que acontece com a ação depende de onde Claude a solicitou:

* Em uma sessão interativa, auto mode volta para um prompt de permissão normal para essa ação para que você possa aprová-la ou negá-la manualmente
* Para um [subagent em segundo plano](/docs/pt/sub-agents#run-subagents-in-foreground-or-background) em uma execução [`-p` não interativa](/docs/pt/headless) sem `--input-format stream-json`, Claude Code retorna um resultado de erro contendo `Agent aborted: auto mode classifier transcript exceeded context window in headless mode`, e a execução continua
* Em outro lugar em uma execução `-p` sem um [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags), não há prompt para voltar, então a ação não é executada e a execução continua

**O que fazer:**

* Em uma sessão interativa, aprove ou negue a ação no prompt que aparece
* Em uma sessão interativa, execute `/compact` para reduzir o tamanho da conversa para que ações subsequentes se encaixem novamente na janela do classificador

<h3 id="the-server-returned-no-safety-verdict">
  The server returned no safety verdict
</h3>

Sob [revisão de classificador do lado do servidor](/docs/pt/permission-modes#server-side-classifier-review), auto mode nega uma ação quando o servidor não dá um veredicto para ela. A negação nomeia uma categoria entre parênteses quando Claude Code pode determinar uma, como `(timed out)`:

```text theme={null}
The server-side auto mode classifier gave no verdict (timed out), so auto mode cannot determine the safety of <tool>.
```

O resto da mensagem diz a Claude se uma tentativa pode ajudar. Antes de algumas dessas negações, Claude Code aguarda para que a próxima tentativa de Claude não siga imediatamente. Durante a espera em uma sessão interativa, o spinner mostra `Auto mode check unavailable` com uma contagem regressiva, e pressionar `Esc` interrompe o turno.

Após dez respostas seguidas sem veredicto, auto mode interrompe o turno:

```text theme={null}
Auto mode is unavailable — the server returned no safety verdict for the last 10 responses, so Claude stopped. Send a message to try again, or switch out of auto mode.
```

A mensagem de parada aparece em um lugar diferente em cada tipo de sessão:

* Em uma sessão interativa, a mensagem aparece como um aviso na transcrição e o turno termina
* Em uma execução [`-p` não interativa](/docs/pt/headless), a execução termina e relata um erro de execução. Com a saída de texto padrão, a mensagem é impressa em stderr.
* Quando um [subagent](/docs/pt/sub-agents) atingiu o limite, o subagent para antes de terminar, e Claude recebe o que produziu com uma nota de que auto mode o interrompeu

**O que fazer:**

* Envie outra mensagem para que Claude tente novamente. A contagem de respostas começa do zero.
* Se a parada se repetir e suas solicitações passarem por um [gateway ou proxy LLM](/docs/pt/llm-gateway), verifique se ele corta respostas de streaming ou as reescreve. [Revisão de classificador do lado do servidor](/docs/pt/permission-modes#server-side-classifier-review) diz qual comportamento de gateway causa negações, e o [guia de compatibilidade de gateway](/docs/pt/llm-gateway-protocol#feature-pass-through) lista o que passar inalterado.
* Defina `CLAUDE_CODE_AUTO_MODE_SERVER=0` antes de iniciar Claude Code para usar suas próprias solicitações de classificador. Antes da v2.1.281, Claude Code não lia a variável em uma conexão direta com a API Anthropic.
* Para aprovar as ações você mesmo, [mude para fora de auto mode](/docs/pt/permission-modes#switch-permission-modes)

Antes da v2.1.280, Claude Code negava cada ação de uma resposta sem veredicto imediatamente e nunca interrompia o turno.

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent terminated early due to an API error
</h3>

A solicitação de API de um [subagent](/docs/pt/sub-agents) falhou terminalmente, por exemplo porque um limite de uso foi atingido ou as tentativas de um erro de servidor se esgotaram, então o subagent parou antes de terminar sua tarefa. Esta mensagem requer Claude Code v2.1.199 ou posterior; antes disso, o texto de erro da API era retornado a Claude como se fosse o resultado do subagent.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**O que fazer:**

* Corresponda o detalhe do erro após os dois pontos à sua própria seção nesta página, como [Usage limits](#usage-limits) ou [Server errors](#server-errors), e siga as etapas dessa seção
* Uma vez que o erro subjacente seja resolvido, peça a Claude para tentar novamente a tarefa ou [retomar o subagent](/docs/pt/sub-agents#resume-subagents)

Quando uma taxa limite, sobrecarga ou erro de servidor interrompe um subagent em primeiro plano que já produziu saída de texto, Claude recebe essa saída parcial marcada como incompleta em vez deste erro. Um subagent cuja única saída foram chamadas de ferramenta também recebe este erro; na v2.1.199 essa forma retornava um resultado parcial vazio. Veja [API errors in subagents](/docs/pt/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Limites de uso
</h2>

A maioria dos erros nesta seção significa que uma cota vinculada à sua conta ou plano foi atingida. Três funcionam de forma diferente: [`Server is temporarily limiting requests`](#server-is-temporarily-limiting-requests) é um throttle do lado do servidor não relacionado à sua cota de plano, [`Usage credits required for 1M context`](#usage-credits-required-for-1m-context) é uma verificação de direito em vez de uma cota esgotada, e [`The prompt to confirm went unanswered`](#the-prompt-to-confirm-went-unanswered) significa que um prompt de consentimento de créditos de uso foi fechado sem resposta, independentemente de uma cota ter sido atingida.

<h3 id="youve-hit-your-session-limit">
  You've hit your session limit
</h3>

Os planos de assinatura incluem uma permissão de uso contínua. Quando ela se esgota, você vê uma destas mensagens:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
You've hit your Sonnet limit · resets 3:45pm
```

Claude Code bloqueia outras solicitações até o horário de reset mostrado na mensagem. Os limites de sessão e semanal são compartilhados entre todos os modelos, portanto, trocar de modelo não restaura o acesso. Os limites de Opus e Sonnet se aplicam apenas a solicitações para essa família de modelos, portanto, trocar para um modelo fora da família com `/model` mantém você trabalhando.

Em uma sessão interativa conectada com uma assinatura claude.ai, Claude Code também pode aguardar na sessão aberta e continuar a tarefa interrompida logo após o reset. Enquanto aguarda, uma linha na parte inferior da sessão lê `Usage limit reached · continuing automatically at 3:45pm · esc to cancel`. Pressione `Esc` em um prompt vazio para cancelar a espera. Veja [Wait for a usage limit to reset](/docs/pt/interactive-mode#wait-for-a-usage-limit-to-reset) para o que você vê, como iniciar ou cancelar uma espera e como desativar a continuação automática. Antes da v2.1.234, Claude Code não oferecia essa espera.

O uso é contado contra as permissões de sessão e semanal ao mesmo tempo. Uma única rajada de atividade pesada, como um grande fanout de fluxo de trabalho, pode esgotar a permissão semanal antes da janela de sessão ser resetada.

**O que fazer:**

* Aguarde o horário de reset mostrado no erro
* Na aba Code do [Desktop app](/docs/pt/desktop), o cartão de limite de sessão oferece uma caixa de seleção **Auto-continue when limits reset**. O cartão de limite semanal não oferece. Quando marcada, o Desktop app tenta novamente a volta interrompida após o reset e mostra o horário da tentativa no cartão. A caixa de seleção do Desktop e a configuração **Continue automatically at usage limit** da CLI em `/config` são separadas, portanto, desative cada uma por conta própria.
* Para o limite de Opus ou Sonnet, execute `/model` e mude para um modelo fora dessa família para continuar trabalhando. Cada modelo tem seu próprio cache de prompt, portanto, a próxima solicitação relê toda a conversa sem acertos de cache; veja [Switching models](/docs/pt/prompt-caching#switching-models)
* Execute `/usage` para ver seus limites de plano e quando eles são resetados
* Execute `/usage-credits` para comprar uso adicional em Pro e Max, ou para solicitá-lo ao seu administrador em Team e Enterprise. Veja [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) para saber como isso é cobrado.
* Para atualizar seu plano para limites base mais altos, veja [claude.com/pricing](https://claude.com/pricing)

Antes de uma janela se esgotar, Claude Code pode avisá-lo de que você usou a maior parte dela, com uma mensagem como `You've used 85% of your session limit · resets 3:45pm`. Para monitorar sua permissão restante continuamente, adicione os campos `rate_limits` a uma [custom status line](/docs/pt/statusline#rate-limit-usage), ou no Desktop app clique no [usage ring](/docs/pt/desktop#check-usage) ao lado do seletor de modelo.

<h3 id="usage-credits-required-for-1m-context">
  Usage credits required for 1M context
</h3>

O modelo selecionado usa a janela de contexto estendido de 1M tokens, e seu plano só o inclui através de créditos de uso.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on (they take effect after you restart Claude Code), or /model to switch to standard context
```

Esta é uma verificação de direito, não um esgotamento de cota. Ela é acionada mesmo quando suas permissões de sessão e semanal têm capacidade restante. Veja [Extended context](/docs/pt/model-config#extended-context) para saber quais planos incluem contexto de 1M diretamente e quais exigem créditos de uso. Claude Code executa essa verificação quando você escolhe o modelo com `/model`, e apenas em uma conexão direta com a API Anthropic; se você apontar `ANTHROPIC_BASE_URL` para um [LLM gateway](/docs/pt/llm-gateway), `/model` permite a seleção `[1m]` e o gateway decide se a solicitação é bem-sucedida.

Quando este erro aparece no meio da conversa porque o contexto cresceu além de 200K tokens, Claude Code compacta automaticamente a conversa de volta para o limite de contexto padrão e mantém a sessão nesse limite depois, portanto, nenhuma ação é necessária. Em versões anteriores à v2.1.172, o erro se repetia em cada solicitação subsequente, incluindo `/compact`; execute `/clear` nessas versões para recuperar. Os passos abaixo se aplicam quando você selecionou explicitamente um modelo `[1m]`.

**O que fazer:**

* Execute `/model` e selecione a variante sem o sufixo `[1m]` para voltar à janela de contexto padrão
* Onde a mensagem menciona `/usage-credits`, execute-a para ativar a cobrança medida para a variante 1M em Pro e Max, ou para solicitar créditos de uso ao seu administrador em Team e Enterprise. Depois que os créditos de uso estiverem ativados, reinicie Claude Code ou inicie uma nova sessão, o que a mensagem disser. Até então, a sessão permanece no limite de contexto padrão.
* Se o erro persistir após `/model`, um ID de modelo 1M pode estar definido em outro lugar. Veja [Setting your model](/docs/pt/model-config#setting-your-model) para os locais de configuração a verificar em ordem de prioridade.
* Para remover variantes 1M do seletor de modelo completamente, defina [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/pt/env-vars)

Antes da v2.1.268, a mensagem terminava com `run /usage-credits to turn them on, or /model to switch to standard context` e não mencionava reiniciar.

<h3 id="the-prompt-to-confirm-went-unanswered">
  The prompt to confirm went unanswered
</h3>

Se sua conta exigir o [Fable usage-credits consent](/docs/pt/model-config#fable-and-usage-credits), Claude Code pede que você confirme antes de uma solicitação Fable cobrar créditos de uso. Quando ninguém responde esse prompt de consentimento em uma sessão que pode não ter ninguém em seu terminal, Claude Code fecha o prompt e encerra a volta com uma destas mensagens:

```text theme={null}
Fable limit reached · continuing on Fable 5.1 uses usage credits, and the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
Fable 5.1 now uses usage credits · the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
```

As mensagens nomeiam o modelo Fable da sessão, portanto, em Fable 5 elas leem `continuing on Fable 5` e `Fable 5 now uses usage credits`. Antes da v2.1.257, a primeira mensagem começava `Fable 5 limit reached`.

Isso acontece em sessões [Remote Control](/docs/pt/remote-control), [background sessions](/docs/pt/agent-view) e sessões de colegas de [agent team](/docs/pt/agent-teams). Claude Code mostra o prompt de consentimento apenas na visualização interativa da própria sessão: o terminal onde ela é executada, ou, para uma sessão em background, a [agents view](/docs/pt/agent-view) depois que você se conecta. Um cliente Remote Control não pode exibi-lo. Claude Code fecha o prompt no prazo [`dialogExpiry`](/docs/pt/settings-reference#dialogexpiry), cinco minutos por padrão, ou assim que um novo prompt chega enquanto ninguém digitou naquele terminal, como um prompt enviado de um cliente Remote Control. Digitar no terminal onde a sessão é executada cancela o prazo, e Claude Code aguarda sua resposta. Na visualização anexada de uma sessão em background, digitar não cancela o prazo, e um novo prompt ainda fecha o prompt de consentimento, portanto, responda antes que qualquer um deles aconteça. Claude Code não envia nada e mantém seu modelo, portanto, quando você enviar seu próximo prompt, Claude Code mostra o prompt de consentimento novamente.

**O que fazer:**

* No terminal onde a sessão é executada, envie outro prompt e responda o prompt de consentimento quando ele reaparecer. Para uma sessão em background, anexe-a primeiro da [agents view](/docs/pt/agent-view). Reenviar de um cliente Remote Control mostra esta mensagem novamente, porque o cliente não pode exibir o prompt.
* Execute `/model` para mudar para um modelo que não cobra créditos de uso
* Para dar a si mesmo mais tempo para alcançar aquele terminal, defina [`dialogExpiry`](/docs/pt/settings-reference#dialogexpiry) para um valor mais longo ou `"never"`

Antes da v2.1.236, esta mensagem não aparecia: enquanto um cliente Remote Control estava conectado, Claude Code aguardava 60 segundos por uma resposta e depois continuava a volta em seu modelo padrão.

<h3 id="server-is-temporarily-limiting-requests">
  Server is temporarily limiting requests
</h3>

A API aplicou um throttle de curta duração que não está relacionado à sua cota de plano.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code diferencia estes da sua cota de plano pela ausência dos cabeçalhos de cota unificada que uma resposta de limite real carrega. A partir da v2.1.199, isto é [retried automatically](#automatic-retries) com backoff antes de ser mostrado, independentemente de como você se autentica. Em versões anteriores, uma sessão conectada com uma assinatura claude.ai falhava na volta na primeira ocorrência; apenas as entradas de chave API e Enterprise a retentavam.

**O que fazer:**

* Aguarde brevemente e tente novamente
* Verifique [status.claude.com](https://status.claude.com) se persistir

<h3 id="request-rejected-429">
  Request rejected (429)
</h3>

Você atingiu o limite de taxa configurado para sua chave API, projeto Amazon Bedrock ou projeto Google Cloud.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

A frase final nomeia onde verificar a saúde do serviço e varia por provedor. Amazon Bedrock, Agent Platform do Google Cloud e configurações Microsoft Foundry nomeiam o status do serviço daquele provedor em vez da página de status Anthropic. Um `ANTHROPIC_BASE_URL` personalizado nomeia o host do gateway.

**O que fazer:**

* Execute `/status` e confirme que a credencial ativa é a que você espera. Um `ANTHROPIC_API_KEY` perdido em seu ambiente pode rotear solicitações através de uma chave de nível inferior em vez de sua assinatura.
* Verifique seu console de provedor para os limites ativos e solicite um nível mais alto se necessário
* Para chaves API Anthropic, veja a [rate limits reference](https://platform.claude.com/docs/en/api/rate-limits) para saber como os níveis funcionam e como definir limites por workspace
* Reduza a concorrência: diminua [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/pt/env-vars), evite executar muitos subagentos paralelos, ou mude para um modelo menor com `/model` para execuções de script de alto volume

<h3 id="youve-hit-your-monthly-spend-limit">
  You've hit your monthly spend limit
</h3>

O uso incluído do seu plano não pode cobrir esta solicitação, e os [usage credits](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) que de outra forma pagariam por isso atingiram um limite de gastos. Isso acontece quando uma das janelas de uso do seu plano se esgotou, ou quando a solicitação é uma que apenas créditos de uso pagam, como uma solicitação para um modelo que [bills to usage credits](/docs/pt/model-config#fable-and-usage-credits). A mensagem nomeia cujo limite o bloqueou. O texto após o `·` diz como aumentar esse limite e varia com seu plano e se você gerencia a cobrança:

```text theme={null}
You've hit your monthly spend limit · raise it at claude.ai/settings/usage
You've hit your individual spend limit · ask your admin for a higher limit
You've hit your org's monthly spend limit · visit claude.ai/admin-settings/usage to raise it
You've hit your team's shared budget · ask your admin to raise it at claude.ai/admin-settings/usage
You've hit your channel's monthly spend limit · an org owner or channel manager can raise it in the channel's Claude settings
```

`team's shared budget` é um orçamento agrupado que um administrador atribuiu a um grupo ao qual você pertence; a mensagem não nomeia o grupo. `channel's monthly spend limit` é o orçamento do único canal Slack em que a sessão é executada, portanto, sua organização ainda pode ter orçamento fora dele.

Quando uma das janelas do seu plano é o que se esgotou, a mensagem também diz quando essa janela é resetada, por exemplo `· your session limit resets 3:45pm`, e o acesso retorna então sem que ninguém aumente o limite. Em organizações com cobrança baseada em uso, a mensagem diz `usage limit` em vez de `spend limit`, como em `You've hit your individual usage limit`.

Antes da v2.1.239, a mensagem não nomeava o horário de reset da janela do plano. Antes da v2.1.268, o orçamento agrupado de um grupo produzia a mensagem `individual spend limit` em vez de `team's shared budget`.

Se você se conectar através de um gateway de aplicativos Claude e vir `spend limit reached` em minúsculas, esse é o limite do seu operador de gateway; veja [Spend limit reached](#spend-limit-reached).

**O que fazer:**

* Em Pro e Max, aumente seu limite de gastos mensais em [**Settings > Usage**](https://claude.ai/settings/usage) em claude.ai, ou execute `/usage-credits`
* Em Team e Enterprise, aumente o limite em [**Admin settings > Usage**](https://claude.ai/admin-settings/usage) se você gerencia a cobrança, ou peça a um administrador. `/usage-credits` envia essa solicitação ao seu administrador para você
* Para o limite de um canal, peça a um proprietário da organização ou ao gerenciador do canal para aumentá-lo em claude.ai. Veja [Per-channel limits](https://claude.com/docs/claude-tag/admins/set-spend-limit#per-channel-limits) na documentação Claude Tag
* Se a mensagem nomear um horário de reset para a janela do seu plano, você pode aguardar em vez disso
* Execute `/usage` para ver as janelas do seu plano e quando cada uma é resetada

<h3 id="spend-limit-reached">
  Spend limit reached
</h3>

Você se conecta através de um [Claude apps gateway](/docs/pt/claude-apps-gateway) e passou por um [spend cap](/docs/pt/claude-apps-gateway-spend-limits) que seu operador de gateway definiu. O gateway bloqueia suas solicitações até que o período nomeado seja resetado ou o operador aumente o limite. Ele marca cada resposta `429` bloqueada com `x-should-retry: false`, portanto, Claude Code mostra esta mensagem sem tentar novamente.

```text theme={null}
spend limit reached (daily; resets 2026-08-09 00:00 UTC)
```

A mensagem nomeia o período do limite e o horário de reset, e quando o operador configurou um `blocked_message`, suas instruções o seguem. Antes da v2.1.225, a mensagem lia apenas `spend limit reached`; um gateway em uma versão mais antiga ainda envia essa forma mais curta.

**O que fazer:**

* Aguarde o horário de reset que a mensagem nomeia, ou siga as instruções do operador se a mensagem as carregar
* Peça ao seu operador de gateway para aumentar o limite se você o atingir rotineiramente

Uma mensagem relacionada, `spend limit unavailable`, significa que o gateway não conseguiu ler seus registros de gastos e bloqueou a solicitação como precaução em vez de sobre seu limite. Geralmente se limpa por conta própria; se persistir, informe seu operador de gateway.

<h3 id="credit-balance-is-too-low">
  Credit balance is too low
</h3>

Sua organização Console ficou sem créditos pré-pagos, ou Claude Code está enviando suas solicitações com uma chave API Console quando você pretendia usar sua assinatura.

```text theme={null}
Credit balance is too low
```

**O que fazer:**

* Se você tem um plano Pro, Max, Team ou Enterprise e vê isso, execute `/status` e verifique a linha `API key`. Um `ANTHROPIC_API_KEY` aprovado em seu ambiente roteia solicitações através dessa chave em vez de sua assinatura. Desdefina-o no shell atual e remova-o do seu perfil de shell, depois relance `claude`. Execute `/login` se você ainda não se conectou com sua assinatura.
* Adicione créditos em [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), e considere ativar o auto-reload lá para que o saldo seja recarregado antes de atingir zero
* Defina limites de gastos por workspace no Console para evitar que um único projeto drene o saldo da organização. Veja [Manage costs effectively](/docs/pt/costs).

<h3 id="could-not-update-your-spend-limit">
  Could not update your spend limit
</h3>

O servidor rejeitou uma alteração de limite de gastos que você fez a partir do prompt que aparece quando você atinge seu limite de gastos.

```text theme={null}
Could not update your spend limit: <reason from the server>
```

Quando o servidor explica a rejeição, a mensagem termina com essa razão, e tentar novamente o mesmo valor falha novamente. Quando a falha não tem razão fornecida pelo servidor, como uma conexão perdida, a mensagem lê `Could not update your spend limit. Press Enter to retry.` e tentar novamente pode ter sucesso. Antes da v2.1.216, Claude Code mostrava a forma genérica para cada falha.

**O que fazer:**

* Se a mensagem incluir uma razão, escolha um limite que a satisfaça, como um valor menor
* Se a mensagem mostrar apenas a forma genérica, tente novamente; a falha pode ser transitória
* Se a alteração continuar falhando, faça-a a partir de suas [claude.ai billing settings](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) no navegador em vez disso

<h2 id="authentication-errors">
  Erros de autenticação
</h2>

Esses erros significam que Claude Code não consegue provar sua identidade para a API. Execute `/status` a qualquer momento para ver qual credencial está ativa no momento.

<h3 id="not-logged-in">
  Não conectado
</h3>

Nenhuma credencial válida está disponível para esta sessão.

```text theme={null}
Not logged in · Please run /login
```

**O que fazer:**

* Execute `/login` para autenticar com sua assinatura Claude ou conta Console
* Se você esperava que uma variável de ambiente o autenticasse, confirme que `ANTHROPIC_API_KEY` está definida e exportada no shell onde você iniciou `claude`
* Para CI ou automação onde login interativo não é possível, configure um script [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) que busque uma chave na inicialização
* Consulte [Precedência de autenticação](/docs/pt/authentication#authentication-precedence) para entender qual credencial Claude Code usa quando várias estão presentes

Se você for solicitado a fazer login repetidamente, consulte [Não conectado ou token expirado](/docs/pt/troubleshoot-install#not-logged-in-or-token-expired) para verificações de relógio do sistema e etapas de recuperação de armazenamento de credenciais do macOS.

<h3 id="could-not-resolve-authentication-method">
  Não foi possível resolver o método de autenticação
</h3>

A sessão chegou ao cliente da API sem nenhuma credencial. [Sessões em segundo plano](/docs/pt/agent-view) e sessões na nuvem mostram esta mensagem quando o worker inicia sem uma credencial. Execuções interativas, `-p` e Agent SDK relatam a mesma condição que [Não conectado](#not-logged-in) e escrevem esta string apenas no log de depuração, portanto, se você a encontrou lá, siga essa entrada.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

Nas versões atuais, o erro significa que nenhuma credencial estava disponível para o processo worker. Antes da v2.1.174, uma sessão em segundo plano atribuída a um worker pré-inicializado ocioso poderia falhar dessa forma mesmo quando credenciais válidas foram configuradas. Antes da v2.1.176, uma sessão na nuvem que ficou ociosa antes de ser reivindicada também poderia. Atualize para recuperar.

**O que fazer:**

* Atualize para v2.1.176 ou posterior se isso aparecer em uma sessão em segundo plano ou na nuvem e suas credenciais já estiverem configuradas
* Confirme que `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` ou suas credenciais do provedor de nuvem estão definidas no ambiente que inicia o worker, não apenas no seu shell interativo
* Para o Agent SDK, consulte [configuração de autenticação no guia de início rápido](/docs/pt/agent-sdk/quickstart#setup)
* Execute `/status` em uma sessão interativa no mesmo ambiente para confirmar qual fonte de credencial é resolvida

<h3 id="invalid-api-key">
  Chave de API inválida
</h3>

A variável de ambiente `ANTHROPIC_API_KEY` ou o script `apiKeyHelper` retornou uma chave que a API rejeitou, ou Claude Code bloqueou uma chave de `ANTHROPIC_API_KEY` antes de enviá-la.

```text theme={null}
Invalid API key · Fix external API key
```

Quando a mensagem continua após `Fix external API key` com uma descrição como `Invalid X-Api-Key header value from ANTHROPIC_API_KEY: it contains a line break at character 41 (120 characters on 2 lines).`, a API nunca viu a chave. Claude Code encontrou um caractere que os cabeçalhos HTTP não conseguem carregar e parou a solicitação antes de enviá-la. Consulte [Valor de cabeçalho de solicitação inválido](#invalid-request-header-value) para saber como ler a descrição e corrigir o valor.

**O que fazer:**

* Verifique se há erros de digitação e confirme que a chave não foi revogada no [Console](https://platform.claude.com/settings/keys)
* No mesmo shell, execute `env | grep ANTHROPIC`, ou no PowerShell `Get-ChildItem Env:ANTHROPIC*`. Ferramentas como direnv, plugins de shell dotenv e terminais IDE podem carregar uma chave obsoleta de um arquivo `.env` em seu projeto sem você defini-la explicitamente.
* Desdefina `ANTHROPIC_API_KEY` e execute `/login` para usar autenticação de assinatura
* Se a chave vem de um script [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper), execute o script diretamente para confirmar que ele imprime uma chave válida em stdout
* Execute `/status` para confirmar qual fonte de credencial Claude Code está realmente usando

<h3 id="your-apikeyhelper-script-is-failing">
  Seu script apiKeyHelper está falhando
</h3>

Claude Code executou o comando em sua configuração [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) e não obteve uma chave de volta. Sem uma, a solicitação chega à API com uma credencial de espaço reservado, e a API a rejeita com `401`. O painel `Authentication` no terminal mostra qual destes aconteceu:

* O comando saiu com um erro ou expirou
* O comando não imprimiu nada em stdout
* O comando imprimiu algo além da chave, como um banner de login ou uma linha de log. O painel mostra `returned output that cannot be used as an API key` e diz o que está errado, sem repetir a saída. Antes da v2.1.227, Claude Code enviava o que o comando imprimia, após aparar espaços em branco ao redor.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Em [modo não interativo](/docs/pt/headless), stderr também carrega o motivo específico, prefixado com `apiKeyHelper failed:`.

Claude Code executa novamente o script e tenta a solicitação até mais duas vezes antes de mostrar esta mensagem, portanto a falha aparece dentro de três tentativas. Antes da v2.1.208, Claude Code gastava o [orçamento de retry](#automatic-retries) completo reenviando a solicitação com a credencial de espaço reservado e depois relatava um erro de autenticação genérico `401` em vez da falha do script.

Executar `/login` não ajuda aqui: a saída do helper [tem precedência](/docs/pt/authentication#authentication-precedence) sobre um login salvo enquanto a configuração estiver presente.

**O que fazer:**

* Execute o comando configurado em `apiKeyHelper` diretamente no seu shell para reproduzir a falha
* Se o comando relatar uma sessão expirada, autentique-se novamente com seu provedor de credencial, por exemplo, fazendo login em seu SSO ou cofre de segredos novamente
* Corrija o comando para que ele imprima apenas a chave em stdout, como um único token de ASCII imprimível até 16.384 caracteres, e saia com código 0. Consulte [girar credenciais com apiKeyHelper](/docs/pt/llm-gateway-connect#rotate-credentials-with-apikeyhelper) para uma configuração funcional.
* Execute `/status` para ver a falha e confirmar que `apiKeyHelper` é a fonte de credencial ativa. A linha `apiKeyHelper` mostra `Failing` com o detalhe da última falha, como o código de saída e a saída de erro do comando, e desaparece após a próxima execução bem-sucedida. Antes da v2.1.274, `/status` mostrava apenas a fonte de credencial, não a falha.
* Cada vez que o comando falha, seu código de saída e saída de erro também aparecem em um painel `Authentication` no terminal. Antes da v2.1.212, o painel era intitulado `Cloud authentication`.

<h3 id="invalid-request-header-value">
  Valor de cabeçalho de solicitação inválido
</h3>

Um valor que Claude Code estava prestes a enviar como cabeçalho de solicitação contém um caractere que os cabeçalhos HTTP não conseguem carregar: uma quebra de linha, um byte NUL ou um caractere acima de `U+00FF`, como uma aspas curva ou um espaço de largura zero. Claude Code para a solicitação antes de qualquer coisa ser enviada e nomeia a variável ou configuração a ser corrigida. A causa usual é uma credencial colada de um documento ou chat que carregava um caractere invisível ou uma quebra de linha perdida.

Claude Code executa essa verificação quando envia solicitações para a API Claude diretamente ou através de um [gateway LLM](/docs/pt/llm-gateway). Em um provedor de nuvem de terceiros, como [Amazon Bedrock](/docs/pt/amazon-bedrock), Claude Code não a executa antes de enviar.

```text theme={null}
Invalid auth token · Fix external auth token
Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable
Invalid request header from the environment · Fix the environment variable
```

A primeira parte da mensagem depende de onde o valor ruim veio:

* `Invalid auth token`: um token de portador de [`ANTHROPIC_AUTH_TOKEN`](/docs/pt/env-vars) ou [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/pt/env-vars)
* `Invalid ANTHROPIC_CUSTOM_HEADERS`: um nome ou valor de cabeçalho que você definiu em [`ANTHROPIC_CUSTOM_HEADERS`](/docs/pt/env-vars). A descrição conta qual par `Name: Value` está em falta, como `distinct header 2 of 3 parsed from ANTHROPIC_CUSTOM_HEADERS`, sem repetir o nome ou valor, já que você escolheu ambos.
* `Invalid request header from the environment`: um valor que Claude Code copia em um cabeçalho de solicitação de outra variável de ambiente, como `CLAUDE_AGENT_SDK_CLIENT_APP`. A descrição nomeia a variável a ser corrigida.

Claude Code relata um `ANTHROPIC_API_KEY` ruim capturado por essa verificação como [Chave de API inválida](#invalid-api-key), com a mesma descrição final. Ele relata uma credencial `/login` salva ruim como [Não conectado](#not-logged-in); execute `/login` para salvar uma nova. A saída de um script [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) nunca chega a essa verificação: Claude Code a valida quando o script é executado, e a saída que um cabeçalho HTTP não consegue carregar falha com [Seu script apiKeyHelper está falhando](#your-apikeyhelper-script-is-failing).

Após o segundo `·`, a mensagem descreve o problema, como neste exemplo completo:

```text theme={null}
Invalid auth token · Fix external auth token · Invalid Authorization header value from ANTHROPIC_AUTH_TOKEN: it contains a line break at character 41 (120 characters on 2 lines).
```

As posições contam caracteres começando em um. A descrição é construída a partir de frases fixas e contagens de caracteres, portanto nunca inclui o valor em si. Ele nomeia o caractere ofensivo apenas quando é um caractere invisível ou tipográfico bem conhecido, como uma marca de ordem de byte, um espaço de largura zero ou uma aspas curva, e relata qualquer outra coisa como `a non-ASCII character`.

**O que fazer:**

* Redefina a variável ou configuração que a mensagem nomeia, digitando novamente os caracteres ao redor da posição relatada em vez de colar da mesma fonte novamente
* Para `ANTHROPIC_CUSTOM_HEADERS`, mantenha um par `Name: Value` por linha e reescreva o par que a mensagem conta
* Execute `/status` para confirmar qual fonte de credencial está ativa

<h3 id="this-organization-has-been-disabled">
  Esta organização foi desabilitada
</h3>

Claude Code está usando um `ANTHROPIC_API_KEY` obsoleto de uma organização Console desabilitada. Quando você tem um login de assinatura salvo, a chave o substitui.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your subscription instead
Your ANTHROPIC_API_KEY belongs to a disabled organization · Update or unset the environment variable
API Error: 400 ... This organization has been disabled.
```

A dica após o `·` depende de suas credenciais salvas: a primeira forma aparece quando um `/login` armazenado pode assumir depois que você desdefine a chave, e a segunda quando a chave é sua única credencial.

As variáveis de ambiente têm precedência sobre `/login`, portanto uma chave exportada no seu perfil de shell ou carregada de um arquivo `.env` é usada mesmo quando você tem uma assinatura Pro ou Max funcional. Em modo não interativo (`-p`), a chave é sempre usada quando presente.

**O que fazer:**

* Desdefina `ANTHROPIC_API_KEY` no shell atual e remova-a do seu perfil de shell, depois reinicie `claude`
* Se a mensagem disser `Update or unset`, você não tem login salvo para recorrer. Desdefina a chave e execute `/login`, ou substitua a chave por uma de uma organização Console ativa.
* Execute `/status` depois para confirmar que a credencial ativa é sua assinatura
* Se nenhuma variável de ambiente estiver definida e o erro persistir, a organização desabilitada é a vinculada ao seu `/login`. Entre em contato com o suporte ou faça login com uma conta diferente.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Sua organização desabilitou a autenticação por chave de API
</h3>

Esta mensagem requer Claude Code v2.1.169 ou posterior. O administrador da sua organização Console desativou a autenticação por chave de API, portanto a API rejeita a chave que Claude Code está enviando. A dica de recuperação após o `·` varia dependendo de onde a chave veio:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

As variáveis de ambiente e `apiKeyHelper` têm precedência sobre `/login`, portanto executar `/login` sozinho não ajuda enquanto qualquer um deles ainda estiver fornecendo uma chave. Consulte [Precedência de autenticação](/docs/pt/authentication#authentication-precedence).

**O que fazer:**

* Se a mensagem nomear `ANTHROPIC_API_KEY`, desdefina-a no shell atual e remova-a do seu perfil de shell ou arquivo `.env`, depois reinicie `claude`
* Se a mensagem nomear `apiKeyHelper`, remova a configuração [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) do seu `settings.json`
* Execute `/login` para fazer login com sua conta claude.ai
* Execute `/status` depois para confirmar que a credencial ativa é sua assinatura em vez de uma chave de API
* Se você precisar de autenticação por chave de API para automação, peça ao administrador da sua organização para reabilitá-la no Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Sua organização desabilitou o acesso à assinatura Claude
</h3>

Sua organização Claude não permite fazer login em Claude Code com um login de assinatura. Executar `/login` novamente com a mesma conta retorna o mesmo erro.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Esta é uma configuração de organização do lado do servidor, portanto não pode ser substituída por configurações locais, variáveis de ambiente ou sinalizadores CLI.

O Agent SDK e o modo não interativo `-p` apresentam isso como o código de erro `oauth_org_not_allowed`.

**O que fazer:**

* Peça ao seu administrador para habilitar o acesso a Claude Code para sua organização
* Autentique-se com uma chave de API do Console em vez de sua assinatura. Consulte [Autenticação do Claude Console](/docs/pt/authentication#claude-console-authentication) para configuração.
* Se você é o administrador e não vê uma opção para habilitar o acesso, entre em contato com [suporte da Anthropic](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Rotinas são desabilitadas pela política da sua organização
</h3>

Um Proprietário em sua organização Team ou Enterprise desativou rotinas no nível da organização. O erro aparece quando você tenta criar ou executar uma rotina, por exemplo, da [interface de Rotinas](/docs/pt/routines) em claude.ai/code. No Claude Code v2.1.227 ou posterior, a mesma configuração também [oculta `/schedule`](/docs/pt/routines#troubleshooting) no CLI.

```text theme={null}
Routines are disabled by your organization's policy.
```

Esta é uma configuração do lado do servidor, portanto não pode ser substituída por configurações locais, variáveis de ambiente ou sinalizadores CLI.

**O que fazer:**

* Peça a um Proprietário em sua organização para habilitar o alternador **Routines** em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Para trabalho agendado único que não requer rotinas no nível da organização, consulte [tarefas agendadas](/docs/pt/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control requer a API Anthropic
</h3>

A sessão não está falando com a API Anthropic diretamente, portanto não há backend claude.ai para [Remote Control](/docs/pt/remote-control) emparelhar.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com. CLAUDE_CODE_USE_BEDROCK is set, so this session is using Amazon Bedrock — unset it (or run in a shell without it) to use Remote Control.
```

Uma segunda sentença explica o que roteou a sessão para longe da API Anthropic; antes da v2.1.219, a mensagem era apenas a primeira sentença. Dependendo da causa, a mensagem nomeia:

* Uma variável de provedor `CLAUDE_CODE_USE_*`, como `CLAUDE_CODE_USE_BEDROCK` para [Amazon Bedrock](/docs/pt/amazon-bedrock) ou `CLAUDE_CODE_USE_VERTEX` para [Agent Platform do Google Cloud](/docs/pt/google-vertex-ai)
* [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) apontando para um host diferente de `api.anthropic.com`, como um [gateway LLM](/docs/pt/llm-gateway) ou proxy, mesmo quando você faz login com claude.ai; antes da v2.1.196, uma URL base personalizada não bloqueava Remote Control
* `ANTHROPIC_UNIX_SOCKET` definido, portanto a sessão envia suas solicitações através de um socket local em vez de para `api.anthropic.com`
* Um login de [gateway de nuvem](/docs/pt/claude-apps-gateway) corporativo feito através de `/login`, que não suporta Remote Control e não tem variável para desdefini-la

**O que fazer:**

* Desdefina a variável que a mensagem nomeia, como `CLAUDE_CODE_USE_BEDROCK` ou `ANTHROPIC_BASE_URL`, e reinicie a sessão, ou inicie Remote Control de uma sessão que fale com a API Anthropic diretamente
* Se a variável não estiver definida no seu shell, verifique a chave `env` em seus [arquivos de configuração](/docs/pt/settings#where-settings-live), que aplica variáveis de ambiente a cada sessão
* Para esta e as outras mensagens de inicialização do Remote Control, consulte [Solucionar problemas do Remote Control](/docs/pt/remote-control#troubleshooting)

<h3 id="remote-control-couldnt-refresh-your-login">
  Remote Control não conseguiu atualizar seu login
</h3>

Claude Code executa uma conexão [Remote Control](/docs/pt/remote-control) ao vivo em credenciais de curta duração que obtém e renova usando seu login claude.ai salvo. Quando claude.ai para de aceitar esse login, ou Claude Code não tem mais login salvo, Claude Code para Remote Control e precisa que você faça login novamente. Qualquer falha pode acontecer enquanto Claude Code ainda está se conectando ou depois, quando renova as credenciais.

Quando Claude Code pede ao serviço de login para atualizar seu login salvo e não recebe resposta, ele mantém Remote Control em execução e tenta a atualização novamente enquanto a credencial atual da conexão ainda é válida. Uma atualização não recebe resposta quando Claude Code não consegue alcançar o serviço de login, a solicitação expira ou o serviço falha sem rejeitar seu login. Se o serviço de login ainda não estiver respondendo quando essa credencial expirar, Claude Code para Remote Control e relata `OAuth token refresh failed`.

Quando Claude Code para Remote Control, ele mostra o motivo em um aviso e em uma linha de transcrição que começa com `Remote Control disconnected`. Sua sessão local continua em execução sem Remote Control. Esta seção cobre estas linhas:

```text theme={null}
Remote Control disconnected — Claude.ai login expired — run /login to restore Remote Control
Remote Control disconnected — Claude.ai login expired — run /login, then /remote-control
Remote Control disconnected — Claude.ai login was rejected — run /login, then /remote-control
Remote Control disconnected — OAuth token unavailable — run /login to restore Remote Control
Remote Control disconnected — OAuth token refresh failed — run /login to re-authenticate
Remote Control disconnected — JWT refresh failed: no OAuth token — run /login
Remote Control disconnected — Signed out of Claude — run /login, then /remote-control
```

Claude Code nomeia a causa no meio da mensagem:

* `Claude.ai login expired` e `Claude.ai login was rejected`: claude.ai não aceita mais seu token de login salvo, porque expirou ou foi revogado
* `OAuth token unavailable`: Claude Code não tinha token de login salvo quando a credencial da conexão venceu para renovação
* `OAuth token refresh failed`: claude.ai rejeitou seu token de login salvo enquanto Claude Code estava se reconectando, e atualizar o token não produziu um novo
* `JWT refresh failed: no OAuth token`: Claude Code não encontrou token de login salvo para renovar
* `Signed out of Claude`: você saiu nesta máquina, por exemplo, executando `/logout` em outro terminal, portanto Claude Code não tem login salvo para renovar a conexão

**O que fazer:**

* Execute `/login` para fazer login novamente
* Execute `/remote-control` para reconectar a sessão. Mensagens terminando `run /login to restore Remote Control` não precisam desta etapa: Claude Code se reconecta automaticamente depois que você faz login.

Antes da v2.1.224, `OAuth token refresh failed — run /login to re-authenticate` lia `OAuth token refresh failed — re-authenticate, then re-enable Remote Control`, e `JWT refresh failed: no OAuth token — run /login` lia `no OAuth token available for recovery (code <N>)`. As mensagens `Claude.ai login expired`, `Claude.ai login was rejected` e `OAuth token unavailable` foram adicionadas na v2.1.225.

Antes da v2.1.238, Claude Code relatava os casos que agora dizem `Signed out of Claude` como `JWT refresh failed: no OAuth token — run /login`, e parava Remote Control com `Claude.ai login expired — run /login to restore Remote Control` assim que uma atualização de login não recebia resposta.

<h3 id="remote-control-stopped-because-the-signed-in-account-changed">
  Remote Control parou porque a conta conectada mudou
</h3>

Claude Code mostra esta linha durante uma sessão [Remote Control](/docs/pt/remote-control) quando você faz login em uma conta ou organização claude.ai diferente nesta máquina. Você fez a mudança fora da sessão Claude Code, por exemplo, executando `/login` em outro terminal.

Uma sessão Remote Control que você iniciou enquanto estava conectado através de `/login` pertence à conta e organização claude.ai que estavam conectadas no momento.

```text theme={null}
Remote Control disconnected — signed-in claude.ai account or organization changed on this machine — run /remote-control to start a session for the current account, or /login to switch back, then /remote-control
```

Claude Code para a sessão Remote Control assim que claude.ai confirma que a conta ou organização mudou. Sua sessão local continua em execução sem Remote Control.

**O que fazer:**

* Execute `/remote-control` para iniciar uma nova sessão Remote Control sob a conta ou organização atual
* Para voltar, execute `/login` e faça login na conta ou organização anterior novamente. Depois execute `/remote-control`.

Antes da v2.1.234, Claude Code não notava quando você mudava para uma conta ou organização diferente fora da sessão Claude Code. Claude Code mantinha a sessão Remote Control conectada até uma solicitação posterior ao servidor Remote Control falhar com `Remote Control server rejected the request (HTTP 404)`. Essa falha poderia vir horas após a mudança.

<h3 id="remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts">
  Remote Control parou porque o aplicativo que executa a sessão saiu ou mudou de contas
</h3>

Quando o aplicativo de desktop Claude ou um IDE hospeda sua sessão, Claude Code obtém seu token de login desse aplicativo em vez de `/login`. Quando claude.ai rejeita esse token, Claude Code pede ao aplicativo um novo. Se o aplicativo responder que está desconectado ou que agora está conectado a uma conta Claude diferente, Claude Code encerra a sessão [Remote Control](/docs/pt/remote-control) e envia ao aplicativo uma destas linhas:

```text theme={null}
Remote Control stopped — the app running this session is now signed in to a different Claude account
Remote Control stopped — the app running this session is signed out of Claude. Sign in there, then turn Remote Control back on
```

Sua sessão local continua em execução sem Remote Control.

**O que fazer:**

* Se o aplicativo estiver desconectado, faça login nele novamente e depois ative Remote Control novamente no aplicativo
* Se o aplicativo mudou de contas, Claude Code não consegue continuar a sessão encerrada sob a nova conta. Inicie uma nova sessão Remote Control sob essa conta.

Antes da v2.1.238, Claude Code enviava ao aplicativo as mensagens `run /login` listadas em [Remote Control não conseguiu atualizar seu login](#remote-control-couldnt-refresh-your-login) em ambos os casos.

<h3 id="oauth-token-revoked-or-expired">
  Token OAuth revogado ou expirado
</h3>

Seu login salvo não é mais válido. Um token revogado significa que você saiu em todos os lugares ou um administrador removeu o acesso; um token expirado significa que a atualização automática falhou no meio da sessão.

Ambas as mensagens relatam uma rejeição que a API retornou para uma solicitação que Claude Code enviou. Quando o login salvo já foi limpo após uma atualização falhada, você vê [Login expirado](#login-expired). Se você autenticar com um token de longa duração em [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/pt/env-vars), você vê as mesmas mensagens quando esse token expira ou é revogado.

```text theme={null}
OAuth token revoked · Please run /login
Please run /login · API Error: 401 OAuth token has expired ...
```

**O que fazer:**

* Execute `/login` para fazer login novamente
* Se o erro retornar na mesma sessão após autenticar novamente, execute `/logout` primeiro para limpar completamente o token armazenado, depois `/login`
* Se você autenticar com a variável de ambiente `CLAUDE_CODE_OAUTH_TOKEN`, Claude Code continua enviando o valor que você definiu após uma solicitação falhar com um 401, em vez de mudar para o token de um login salvo. [`/status`](/docs/pt/commands) mostra essa credencial como uma linha `Auth token` lendo `CLAUDE_CODE_OAUTH_TOKEN`. Gere um token novo com [`claude setup-token`](/docs/pt/authentication#generate-a-long-lived-token) e reinicie com ele, ou desdefina a variável e execute `/login`. Antes da v2.1.225, Claude Code poderia substituir o valor da variável no meio da sessão pelo token de acesso de curta duração de um login salvo, e a sessão falhava com erros 401 novamente depois que esse token expirava.
* Para prompts repetidos para fazer login entre inicializações, consulte as verificações de relógio do sistema e etapas de recuperação de armazenamento de credenciais do macOS em [Solução de problemas](/docs/pt/troubleshoot-install#not-logged-in-or-token-expired)
* Para outras falhas, incluindo `403 Forbidden` e problemas de navegador OAuth, consulte [Login e autenticação](/docs/pt/troubleshoot-install#login-and-authentication)

<h3 id="api-error-401-invalid-authentication-credentials">
  API Error: 401 Credenciais de autenticação inválidas
</h3>

A API reconheceu o formato de sua credencial, mas rejeitou a conta ou organização por trás dela. Anthropic retorna esta mensagem quando uma credencial foi revogada recentemente, quando uma organização foi desabilitada ou removeu seu acesso, ou quando a conta em si foi desativada, portanto um token expirado não é a causa. A credencial pode ser seu login salvo ou um `ANTHROPIC_API_KEY` aprovado, e a correção difere, portanto comece executando `/status` para ver qual está ativa.

```text theme={null}
Please run /login · API Error: 401 Invalid authentication credentials
```

**O que fazer:**

* Se `/status` mostrar uma linha `API key` que não esteja marcada como não em uso, um [`ANTHROPIC_API_KEY`](/docs/pt/authentication#authentication-precedence) aprovado é a credencial ativa e tem precedência sobre seu login, portanto `/login` não a substitui. Gire a chave no Claude Console, ou volte para sua assinatura executando `unset ANTHROPIC_API_KEY`, ou no PowerShell `Remove-Item Env:ANTHROPIC_API_KEY`.
* Se `/status` mostrar apenas seu login, execute `/login` uma vez. Se a credencial foi revogada, um novo login a substitui.
* Se a mesma mensagem retornar para a mesma conta de login, a conta ou organização não está mais ativa. Verifique a conta e organização que `/status` relata e peça ao administrador da sua organização para restaurar o acesso.
* Se [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) apontar para um [gateway LLM](/docs/pt/llm-gateway), o texto após `401` é a mensagem do seu gateway em vez da Anthropic, e `/login` não a altera. Corrija a credencial que seu gateway espera.

<h3 id="login-expired">
  Login expirado
</h3>

Claude Code tentou renovar seu login claude.ai ou Claude Console salvo e o serviço OAuth rejeitou o token de atualização armazenado, portanto Claude Code limpou as credenciais salvas. Depois disso, cada solicitação de modelo para localmente com esta mensagem antes de chegar à API, porque apenas `/login` pode criar novas credenciais.

Antes da v2.1.206, Claude Code enviava a solicitação de modelo de qualquer forma com qualquer credencial que permanecesse no ambiente, e cada modelo falhava com [Há um problema com o modelo selecionado](#theres-an-issue-with-the-selected-model) ou um 401 em vez de um prompt para fazer login.

```text theme={null}
Login expired · Please run /login
```

Em [modo não interativo](/docs/pt/headless) (`-p`) e no [Agent SDK](/docs/pt/agent-sdk/overview), a mensagem lê como segue, e o código de erro estruturado é `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Este não é o mesmo estado que [Token OAuth revogado ou expirado](#oauth-token-revoked-or-expired). Essas mensagens relatam uma rejeição que a API retornou. Claude Code em si produz `Login expired` para um login que já falhou em renovar, portanto não envia solicitação. Quando a renovação falha porque a conta em si está suspensa em vez do login estar obsoleto, Claude Code mostra [Sua conta está em espera](#your-account-is-on-hold).

Sessões autenticadas com uma chave de API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/pt/env-vars) ou um provedor de terceiros não usam o login salvo e nunca veem esta mensagem.

Você pode verificar este estado antes de uma solicitação falhar: [`/status`](/docs/pt/commands) mostra uma linha `Login` lendo `Expired — log in again`, mais a organização e email que tem salvo para o login expirado. A linha aparece apenas quando o login salvo é sua credencial ativa e não pode mais ser atualizado. Sessões autenticadas de outra forma não mostram a linha, mesmo que um login expirado permaneça salvo. Antes da v2.1.210, `/status` não dava indicação neste estado de que um login já havia existido, porque a credencial limpa deixou nada para relatar.

**O que fazer:**

* Execute `/login` para fazer login novamente. Tentar novamente sem fazer login mostra a mesma mensagem em cada solicitação.
* Em modo não interativo, execute `claude` no mesmo ambiente, complete `/login`, depois execute novamente seu comando. Para automação que não consegue fazer login interativamente, autentique com `ANTHROPIC_API_KEY` ou [gere um token de longa duração com `claude setup-token`](/docs/pt/authentication#generate-a-long-lived-token).
* Se fazer login continuar falhando, consulte [Login e autenticação](/docs/pt/troubleshoot-install#login-and-authentication)

<h3 id="claude-login-not-accepted">
  Login Claude não aceito
</h3>

Você tentou iniciar uma [sessão na nuvem](/docs/pt/claude-code-on-the-web), e o servidor recusou criá-la com um 401: ele não aceitou o login Claude que esta máquina enviou, geralmente porque o login expirou ou foi revogado.

A primeira parte da linha é o próprio motivo do servidor quando ele fornece um. Caso contrário, a linha lê:

```text theme={null}
Claude login not accepted · Run /login, then try again
```

**O que fazer:**

* Execute `/login`, complete o login e tente iniciar a sessão novamente

<h3 id="artifacts-need-a-claude-ai-login">
  Artefatos precisam de um login claude.ai
</h3>

Claude Code recusou uma publicação ou leitura de [artefato](/docs/pt/artifacts) porque a sessão não tem login claude.ai que possa usar para artefatos.

Cada forma da mensagem começa com as mesmas palavras, seguida por um remédio que depende de como sua sessão se autentica. Sem credencial concorrente, lê:

```text theme={null}
Artifacts need a claude.ai login. Run /login and select "Claude account with subscription", then retry — the "Anthropic Console account" option does not provide claude.ai credentials.
```

**O que fazer:**

* Execute `/login` e selecione **Claude account with subscription**. A opção **Anthropic Console account** não fornece credenciais claude.ai.
* Quando a mensagem nomeia uma credencial que tem precedência, como `ANTHROPIC_API_KEY`, uma configuração `apiKeyHelper` ou uma chave Console salva por um `/login` anterior, remova-a da forma que a mensagem diz, depois execute `/login`
* Quando a mensagem diz que esta sessão remota se autentica através da máquina que a iniciou, faça login em claude.ai nessa máquina e depois reconecte a sessão
* Quando a mensagem diz que a credencial é injetada pelo ambiente host da sessão, você não consegue alterá-la nessa sessão; inicie uma sessão que está conectada a claude.ai
* Consulte [Disponibilidade](/docs/pt/artifacts#availability) para os outros requisitos que artefatos têm, como plano, provedor de modelo e política de organização

<h3 id="administrator-policy-requires-a-cloud-gateway-sign-in">
  A política do administrador requer um login do Cloud gateway
</h3>

As [configurações gerenciadas](/docs/pt/managed-settings) de um administrador nesta máquina definem [`forceLoginMethod`](/docs/pt/settings-reference#forceloginmethod) como `"gateway"` ou definem [`forceLoginGatewayUrl`](/docs/pt/settings-reference#forcelogingatewayurl). A menos que você selecione um provedor de nuvem através de uma variável como `CLAUDE_CODE_USE_BEDROCK`, Claude Code então aceita apenas o login do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway). Você vê uma de duas mensagens:

```text theme={null}
Not signed in to the Cloud gateway — run /login.
```

Solicitações de modelo falham com esta mensagem quando a sessão não tem login de gateway, por exemplo, porque você não executou `/login` desde que a política chegou à máquina.

Se você também tiver uma credencial `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` configurada e as configurações gerenciadas definirem `forceLoginMethod`, Claude Code sai na inicialização com uma mensagem que começa:

```text theme={null}
Administrator policy requires a Cloud gateway sign-in on this machine; the
Anthropic-issued credential configured here (ANTHROPIC_API_KEY,
ANTHROPIC_AUTH_TOKEN, or apiKeyHelper) is not used.
```

**O que fazer:**

* Execute `/login` e complete o login na tela **Cloud gateway**
* Para a mensagem de inicialização, remova a configuração `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` que você configurou, depois inicie `claude` e execute `/login`
* Se você acredita que a máquina não deveria exigir o gateway, peça ao administrador que a gerencia para remover `forceLoginMethod` e `forceLoginGatewayUrl` de suas configurações gerenciadas

Na v2.1.265, uma regressão também mostrou a primeira mensagem em algumas configurações de gateway LLM e proxy que se autenticam com uma chave de API, `apiKeyHelper` ou cabeçalhos personalizados, mesmo sem requisito de administrador na máquina. Atualize para v2.1.266 ou posterior. Você não precisa alterar sua configuração.

Antes da v2.1.261, em máquinas que definem `forceLoginMethod` como `"gateway"`, Claude Code usava um login salvo restante em vez de falhar solicitações de modelo, e relatava uma credencial de ambiente configurada com `This machine's managed settings require a first-party login` em vez da mensagem de inicialização. Antes da v2.1.265, uma máquina cujas configurações gerenciadas definem apenas `forceLoginGatewayUrl` não exigia o login de gateway, e Claude Code usava uma credencial restante lá.

<h3 id="your-account-is-on-hold">
  Sua conta está em espera
</h3>

A conta Claude por trás do seu login foi suspensa. Claude Code mostra a primeira mensagem quando tenta renovar seu login salvo e aprende sobre a espera, e a segunda quando um login que você completa no navegador a relata:

```text theme={null}
Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted
Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted
```

Fazer login novamente com a mesma conta não limpa a mensagem, porque a espera está na conta em vez do login. Em [modo não interativo](/docs/pt/headless) (`-p`) e no [Agent SDK](/docs/pt/agent-sdk/overview), o código de erro estruturado é `account_on_hold`. Antes da v2.1.235, Claude Code relatava uma conta em espera como [Login expirado · Please run /login](#login-expired), cujas etapas de recuperação não conseguem limpar uma espera.

**O que fazer:**

* Abra o link na mensagem para visualizar os detalhes da espera ou apelá-la
* Se você tiver outra conta Claude ou uma chave de API que não seja afetada pela espera, você pode continuar trabalhando enquanto a espera é resolvida: execute `/login` com essa conta, ou defina a chave com `ANTHROPIC_API_KEY`

<h3 id="anthropic-profile-login-expired">
  Login do perfil Anthropic expirado
</h3>

Claude Code está se autenticando através de um perfil de credencial Anthropic cuja credencial de login salva expirou, e o perfil não contém credencial de atualização que Claude Code possa usar para renová-la. Claude Code para cada solicitação localmente sem tentar novamente, porque uma tentativa novamente leria a mesma credencial expirada.

```text theme={null}
Anthropic profile login expired · Re-authenticate your Anthropic profile
Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile
```

Isso aparece apenas quando a credencial ativa vem de um perfil de credencial Anthropic, um que você seleciona com a variável de ambiente `ANTHROPIC_PROFILE`, que Claude Code descobre como o perfil ativo em seu diretório de configuração Anthropic, ou que Claude Code escreveu quando você [fez login sem uma chave de API](/docs/pt/authentication#sign-in-without-an-api-key). Sessões que se autenticam com a opção claude.ai do `/login`, uma chave de API, um token de portador como `ANTHROPIC_AUTH_TOKEN` ou um provedor de terceiros nunca veem esta mensagem.

Em uma máquina que [oferece o login sem chave](/docs/pt/authentication#sign-in-without-an-api-key), execute `/login`, escolha a conta Anthropic Console e faça login novamente para renovar um perfil que o login Console sem chave ou o `ant auth login` da CLI da Plataforma Claude escreveu. Claude Code substitui a credencial expirada nesse perfil. Para um perfil de federação ou um que outra ferramenta criou, `/login` não renova a credencial. Qual forma você vê depende se você selecionou o perfil ou Claude Code o descobriu:

* Quando você define `ANTHROPIC_PROFILE` explicitamente, a mensagem termina com `Re-authenticate your Anthropic profile`.
* Quando Claude Code descobriu o perfil do seu diretório de configuração, a mensagem oferece `/login`, porque Claude Code dá precedência a um `/login` funcional sobre o perfil descoberto e depois se autentica com sua conta claude.ai ou Console. Antes da v2.1.234, Claude Code mostrava o formulário `Re-authenticate your Anthropic profile` neste caso também.

**O que fazer:**

* Faça login no perfil novamente, depois tente novamente: em uma máquina que [oferece o login sem chave](/docs/pt/authentication#sign-in-without-an-api-key), execute `/login` e escolha a conta Anthropic Console para um perfil que o login Console sem chave ou o `ant auth login` da CLI da Plataforma Claude escreveu; para outros perfis, use a ferramenta que os criou
* Se um administrador provisionou a credencial do perfil, peça a ele para emitir uma nova
* Execute `/status` para confirmar a fonte de credencial ativa e o nome do perfil
* Para parar de usar o perfil, desdefina `ANTHROPIC_PROFILE` se você o definiu, depois autentique de outra forma, como `/login` ou `ANTHROPIC_API_KEY`

<h3 id="oauth-scope-requirement">
  Requisito de escopo OAuth
</h3>

O token armazenado é anterior a um escopo de permissão que um recurso mais novo precisa. Você vê isso com mais frequência de `/usage` e o indicador de uso da linha de status:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**O que fazer:**

* Execute `/login` para obter um novo token com os escopos atuais. Você não precisa fazer logout primeiro.

<h3 id="claude-ai-rejected-the-session-token">
  claude.ai rejeitou o token da sessão
</h3>

Uma solicitação de [conector claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) falhou porque claude.ai rejeitou o token do seu login Claude Code, geralmente um login que expirou e não conseguiu ser atualizado. O token rejeitado é seu login, não a autorização própria do conector em claude.ai, portanto autorizar o conector novamente não o resolve. Em `/mcp`, o conector mostra como `connected · session token rejected` e sua visualização de detalhes lê:

```text theme={null}
claude.ai rejected the session token. Run /login, then reconnect.
```

**O que fazer:**

* Execute `/login` para fazer login novamente
* Reconecte o conector de `/mcp`, ou execute `/mcp reconnect <server>`. Reconectar antes de fazer login novamente deixa o conector no mesmo estado. A opção **Reconnect** do painel `/mcp` relata `your claude.ai session token was rejected`; o formulário digitado `/mcp reconnect <server>` relata uma reconexão bem-sucedida mesmo que o token ainda seja rejeitado.

Antes da v2.1.222, Claude Code marcava o conector como precisando de autenticação, o que apontava você para o fluxo de autorização do conector mesmo que completá-lo não resolvesse o estado.

<h3 id="mcp-server-needs-you-to-sign-in-again">
  Servidor MCP precisa que você faça login novamente
</h3>

Um [servidor MCP](/docs/pt/mcp) remoto rejeitou a credencial em uma chamada de ferramenta no meio da sessão, geralmente porque um login ou token expirou ou porque o token carece de uma permissão que a ferramenta precisa. A chamada de ferramenta falha, e `/mcp` marca o servidor como [precisando de autenticação](/docs/pt/mcp#authenticate-with-remote-mcp-servers).

Para um servidor que você faz login de Claude Code, incluindo um conector claude.ai, o login expirou ou foi revogado:

```text theme={null}
MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)
```

Execute `/mcp`, selecione o servidor e faça login novamente de seu menu.

Para um servidor configurado com um script [`headersHelper`](/docs/pt/mcp#use-dynamic-headers-for-custom-authentication), Claude Code já executou novamente o helper e tentou novamente a chamada uma vez antes de mostrar isso:

```text theme={null}
MCP server "<name>" rejected the credential from its headersHelper (check the helper and run /mcp to reconnect, or to authenticate if the server also uses OAuth)
```

Verifique se o helper retorna uma credencial que o servidor aceita, depois reconecte de `/mcp`, que executa o helper novamente.

Para um servidor com um cabeçalho `Authorization` estático em sua configuração:

```text theme={null}
MCP server "<name>" rejected the Authorization header in its config (update it, then run /mcp to reconnect)
```

Atualize o valor do cabeçalho onde o servidor está configurado, depois reconecte de `/mcp`.

Antes da v2.1.273, os casos de login expirado, `headersHelper` e cabeçalho `Authorization` todos mostravam `MCP server "<name>" requires re-authorization (token expired)`.

Um servidor também pode recusar uma chamada de ferramenta com HTTP 403 `insufficient_scope` para pedir que você autorize um escopo, às vezes um que seu token já lista. A mensagem nomeia esse escopo:

```text theme={null}
MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate
```

Execute `/mcp`, selecione o servidor e autentique novamente de seu menu.

Quando a configuração do servidor não define [`oauth.scopes`](/docs/pt/mcp#restrict-oauth-scopes) nem [`authServerMetadataUrl`](/docs/pt/mcp#override-oauth-metadata-discovery), Claude Code solicita o escopo que o servidor nomeou. Com qualquer configuração, Claude Code solicita os escopos dessa configuração. Se você fixou `oauth.scopes`, adicione o escopo ausente a essa lista antes de autenticar novamente.

Antes da v2.1.274, este caso mostrava a mensagem `needs you to sign in again`, e antes da v2.1.273 mostrava `requires re-authorization (token expired)` como os outros casos.

<h3 id="issuer-mismatch-in-authorization-response">
  Incompatibilidade de emissor na resposta de autorização
</h3>

Durante um [login OAuth do MCP](/docs/pt/mcp#authenticate-with-remote-mcp-servers), o servidor de autorização redirecionou de volta para Claude Code com um parâmetro `iss` que não nomeia o emissor que Claude Code esperava dos metadados OAuth do servidor. Um emissor errado nesta etapa é como um ataque de mistura de servidor de autorização se parece, portanto Claude Code falha no login em vez de trocar o código de autorização. Claude Code mostra o erro no menu do servidor `/mcp` após o login do navegador:

```text theme={null}
Issuer mismatch in authorization response (RFC 9207): expected "https://auth.example.com", received "https://other.example.com"
```

`expected` é o emissor dos metadados OAuth do servidor, e `received` é o valor `iss` que o redirecionamento carregava. Um login cujo redirecionamento não carrega parâmetro `iss` passa na verificação, a menos que os metadados do servidor definam `authorization_response_iss_parameter_supported`, nesse caso Claude Code falha no login.

**O que fazer:**

* Tente o login novamente de `/mcp`
* Se o erro se repetir, relate-o ao operador do servidor. A correção é do lado do servidor: o servidor de autorização deve retornar o mesmo emissor no parâmetro `iss` que ele anuncia em seus metadados
* Para conectar enquanto o servidor está sendo corrigido, inicie Claude Code com [`MCP_SDK_GENERATION=v1`](/docs/pt/env-vars), cujo [runtime](/docs/pt/mcp#mcp-client-runtimes) não executa essa verificação. Isso remove uma proteção contra ataques de mistura, portanto prefira a correção do lado do servidor

Antes da v2.1.232, Claude Code usava o runtime v2 apenas em um lançamento gradual ou quando você definia `MCP_SDK_GENERATION=v2`.

<h3 id="aws-credentials-expired-or-invalid">
  Credenciais AWS expiradas ou inválidas
</h3>

Seu token de sessão AWS expirou ou foi rejeitado. Esta mensagem aparece em um 401 de [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) ou do [endpoint Mantle](/docs/pt/amazon-bedrock#use-the-mantle-endpoint), que é como esses provedores relatam um token de segurança expirado.

A dica de ação no meio varia com sua configuração. A parte estável é o `AWS credentials expired or invalid` inicial:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Antes da v2.1.273, esta mensagem aparecia apenas quando `awsAuthRefresh` estava configurado.

**O que fazer:**

* Se a dica disser que as credenciais são gerenciadas por este ambiente, o aplicativo que iniciou Claude Code possui a credencial e os outros passos aqui não se aplicam: tente novamente ou entre em contato com seu administrador
* Se [`awsAuthRefresh`](/docs/pt/amazon-bedrock#advanced-credential-configuration) estiver definido, execute o comando nomeado na mensagem, como `aws sso login --profile myprofile`, em outro terminal e complete o login do navegador, depois tente novamente. Caso contrário, atualize a credencial AWS que você usa: seu login SSO, chaves de acesso, chave de API ou token de proxy
* Com `awsAuthRefresh` definido em uma sessão interativa, você pode executar `/login`, escolher **3rd-party platform**, depois selecionar **Claude Platform on AWS · refresh credentials** em **Using 3rd-party platforms** para executar o mesmo comando sem reiniciar Claude Code. Consulte [Configure AWS credentials](/docs/pt/claude-platform-on-aws#1-configure-aws-credentials)
* Se o erro se repetir após o comando de atualização ter sucesso, confirme que a identidade é válida fora de Claude Code com `aws sts get-caller-identity` no mesmo shell e perfil

<h3 id="aws-authentication-failed">
  Falha na autenticação AWS
</h3>

Seu provedor AWS retornou um 403, ou [Amazon Bedrock](/docs/pt/amazon-bedrock) retornou um 401.

Amazon Bedrock relata um token de segurança expirado como um 403, mas um 403 também é como ele relata uma negação de autorização, como um `AccessDeniedException` de uma permissão IAM ausente. Claude Code não consegue distinguir essas duas causas.

Um 401 do Amazon Bedrock também chega aqui em vez de em [Credenciais AWS expiradas ou inválidas](#aws-credentials-expired-or-invalid), porque Amazon Bedrock não relata um token expirado como um 401. Um 401 desse endpoint geralmente vem de algo mais no caminho da solicitação, como um proxy corporativo.

Uma atualização de credencial corrige um token expirado e não consegue corrigir as outras causas, portanto a mensagem oferece ambas:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

A dica de ação no meio varia com sua configuração. A parte estável é o `AWS authentication failed` inicial.

Quando o 403 é a resposta do Amazon Bedrock de que você não tem acesso ao modelo com o ID de modelo especificado, a dica em vez disso diz para você habilitar o modelo para sua conta e região no console Amazon Bedrock.

Antes da v2.1.273, esta mensagem aparecia apenas quando `awsAuthRefresh` estava configurado.

**O que fazer:**

* Se a dica disser que as credenciais são gerenciadas por este ambiente, o aplicativo que iniciou Claude Code possui a credencial e os outros passos aqui não se aplicam: tente novamente ou entre em contato com seu administrador
* Atualize suas credenciais AWS em caso de uma credencial expirada ser a causa: execute o comando [`awsAuthRefresh`](/docs/pt/amazon-bedrock#advanced-credential-configuration) nomeado na mensagem quando um estiver definido, ou atualize seu login SSO, chaves de acesso, chave de API ou token de proxy você mesmo
* Se suas credenciais estão atuais, confirme as permissões IAM em [Configuração IAM](/docs/pt/amazon-bedrock#iam-configuration) estão anexadas à identidade que você está usando e que o modelo selecionado está habilitado para sua conta e região
* Execute `aws sts get-caller-identity` para confirmar qual identidade suas solicitações usam; um `AWS_PROFILE` obsoleto ou perfil padrão é uma causa comum de incompatibilidade de permissão

<h3 id="google-cloud-credentials-expired-or-invalid">
  Credenciais do Google Cloud expiradas ou inválidas
</h3>

Suas credenciais do Google Cloud para [Agent Platform do Google Cloud](/docs/pt/google-vertex-ai) expiraram ou foram rejeitadas: a solicitação retornou um 401, que é como Agent Platform relata expiração de credencial.

A dica de ação no meio varia com sua configuração. A parte estável é o `Google Cloud credentials expired or invalid` inicial:

```text theme={null}
Google Cloud credentials expired or invalid · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · API Error: 401 ...
```

**O que fazer:**

* Se a dica disser que as credenciais são gerenciadas por este ambiente, o aplicativo que iniciou Claude Code possui a credencial e os outros passos aqui não se aplicam: tente novamente ou entre em contato com seu administrador
* Se você se autentica com credenciais padrão de aplicativo, execute o comando [`gcpAuthRefresh`](/docs/pt/google-vertex-ai#advanced-credential-configuration) nomeado na mensagem, ou `gcloud auth application-default login`, e complete o login, depois tente novamente
* Se você roteia através de um [gateway LLM](/docs/pt/llm-gateway) com `CLAUDE_CODE_SKIP_VERTEX_AUTH` definido, atualize o token de gateway em `ANTHROPIC_AUTH_TOKEN` ou `ANTHROPIC_CUSTOM_HEADERS`, depois tente novamente
* Se você se autentica com um arquivo de chave de conta de serviço, confirme que `GOOGLE_APPLICATION_CREDENTIALS` aponta para uma chave válida. Consulte [Configure GCP credentials](/docs/pt/google-vertex-ai#3-configure-gcp-credentials)
* Se o erro se repetir após uma atualização, confirme que a identidade funciona fora de Claude Code com `gcloud auth application-default print-access-token` no mesmo shell

Antes da v2.1.273, um 401 do Agent Platform mostrava a mensagem genérica `Please run /login` ou `Failed to authenticate`, que não consegue atualizar credenciais do Google Cloud.

<h3 id="google-cloud-authentication-failed">
  Falha na autenticação do Google Cloud
</h3>

[Agent Platform do Google Cloud](/docs/pt/google-vertex-ai) retornou um 403, que usa para negações de autorização em vez de credenciais expiradas. Geralmente a identidade com a qual você se autentica está faltando uma permissão IAM, ou o modelo não está habilitado para seu projeto.

A dica de ação no meio varia com sua configuração. A parte estável é o `Google Cloud authentication failed` inicial:

```text theme={null}
Google Cloud authentication failed · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · if credentials are current, check GCP IAM permissions and Vertex AI model access · API Error: 403 ...
```

**O que fazer:**

* Se a dica disser que as credenciais são gerenciadas por este ambiente, o aplicativo que iniciou Claude Code possui a credencial e os outros passos aqui não se aplicam: tente novamente ou entre em contato com seu administrador
* Confirme as funções em [Configuração IAM](/docs/pt/google-vertex-ai#iam-configuration) são concedidas à identidade com a qual você se autentica
* Confirme que o modelo está habilitado para seu projeto. Consulte [Request model access](/docs/pt/google-vertex-ai#2-request-model-access)

Antes da v2.1.273, um 403 do Agent Platform mostrava a mensagem genérica `Please run /login` ou `Failed to authenticate`, que não consegue atualizar credenciais do Google Cloud.

<h3 id="microsoft-foundry-authentication-failed">
  Falha na autenticação do Microsoft Foundry
</h3>

[Microsoft Foundry](/docs/pt/microsoft-foundry) retornou um 401 ou 403: a credencial Azure na solicitação foi rejeitada, ou a identidade por trás dela não tem acesso ao recurso Foundry. `/login` não consegue cunhar credenciais Azure. A dica de ação no meio varia com sua configuração. A parte estável é o `Microsoft Foundry authentication failed` inicial:

```text theme={null}
Microsoft Foundry authentication failed · refresh your Foundry credential (ANTHROPIC_FOUNDRY_AUTH_TOKEN, ANTHROPIC_FOUNDRY_API_KEY, Azure sign-in for Entra, or your proxy token) and retry · if credentials are current, check access to the Foundry resource · API Error: 401 ...
```

**O que fazer:**

* Se a dica disser que as credenciais são gerenciadas por este ambiente, o aplicativo que iniciou Claude Code possui a credencial e os outros passos aqui não se aplicam: tente novamente ou entre em contato com seu administrador
* Atualize a credencial que você configurou em [Configure Azure credentials](/docs/pt/microsoft-foundry#2-configure-azure-credentials): gire `ANTHROPIC_FOUNDRY_API_KEY`, cunhe um novo `ANTHROPIC_FOUNDRY_AUTH_TOKEN`, ou execute `az login` para que a cadeia de credencial padrão do Microsoft Entra possa fazer login novamente
* Se a credencial está atual, confirme que a identidade tem acesso ao recurso Foundry. Consulte [Azure RBAC configuration](/docs/pt/microsoft-foundry#azure-rbac-configuration)

Antes da v2.1.273, um 401 ou 403 do Microsoft Foundry mostrava a mensagem genérica `Please run /login` ou `Failed to authenticate`, que não consegue atualizar credenciais Azure.

<h3 id="could-not-load-aws-or-google-cloud-credentials">
  Não foi possível carregar credenciais AWS ou Google Cloud
</h3>

Claude Code não conseguiu obter credenciais utilizáveis da cadeia de provedor de credenciais AWS ou de suas credenciais padrão de aplicativo Google na máquina em que é executado, portanto nenhuma solicitação chegou ao seu provedor de nuvem. Claude Code limpa suas credenciais em cache e tenta novamente duas vezes antes de mostrar esta mensagem. O detalhe após o `·` nomeia a causa específica, como uma sessão SSO expirada, credenciais padrão ausentes relatadas como `Could not load the default credentials`, ou um login revogado relatado como `invalid_grant`:

```text theme={null}
API Error: Could not load AWS credentials · Could not load credentials from any providers. Check or refresh your AWS credentials and try again.
API Error: Could not load Google Cloud credentials · invalid_grant. Check or refresh your Google Cloud credentials and try again.
```

Em [modo não interativo](/docs/pt/headless) com `-p` e no [Agent SDK](/docs/pt/agent-sdk/overview), o código de erro estruturado é `cloud_credential_error`. Antes da v2.1.267, a mensagem mostrava apenas o texto de detalhe após `API Error:`, e o código estruturado era `server_error` ou `unknown`.

**O que fazer:**

* Execute o comando de login do seu provedor, como `aws sso login --profile myprofile` ou `gcloud auth application-default login`, depois tente novamente. [Credenciais do Bedrock, Agent Platform ou Foundry não carregando](/docs/pt/troubleshoot-install#bedrock-agent-platform-or-foundry-credentials-not-loading) mostra como confirmar as credenciais fora de Claude Code
* Se o detalhe lê `AWS default-chain credential resolve timed out`, a cadeia travou em vez de falhar, portanto siga [Resolução de credencial de cadeia padrão AWS expirou](#aws-default-chain-credential-resolve-timed-out)

<h3 id="aws-default-chain-credential-resolve-timed-out">
  Resolução de credencial de cadeia padrão AWS expirou
</h3>

A cadeia de provedor de credencial padrão AWS não produziu credenciais dentro de 60 segundos, portanto Claude Code parou a resolução e falhou a solicitação. Este tempo limite é uma causa de [Não foi possível carregar credenciais AWS ou Google Cloud](#could-not-load-aws-or-google-cloud-credentials). A falha é resolução de credencial local: a solicitação nunca chegou a [Amazon Bedrock](/docs/pt/amazon-bedrock), [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) ou ao [endpoint Mantle](/docs/pt/amazon-bedrock#use-the-mantle-endpoint). Claude Code limpa seu [cache de credencial](/docs/pt/amazon-bedrock#credential-caching-and-resolution-timeout) e tenta novamente antes desta mensagem de erro aparecer, portanto no momento em que você a vê a cadeia travou em tentativas repetidas.

```text theme={null}
API Error: Could not load AWS credentials · AWS default-chain credential resolve timed out. Check or refresh your AWS credentials and try again.
```

As causas comuns são um comando `credential_process` em seu perfil AWS que aguarda entrada que não consegue receber, e um contêiner ou VM cuja instância de serviço de metadados (IMDS) nunca responde à sonda da cadeia.

Antes da v2.1.267, a mensagem lia `API Error: AWS default-chain credential resolve timed out`.
Antes da v2.1.207, uma cadeia travada deixava a solicitação esperando indefinidamente em vez de falhar.

**O que fazer:**

* Execute `aws sts get-caller-identity` no mesmo shell com o mesmo `AWS_PROFILE`. Se também travar, corrija o perfil; um comando `credential_process` que solicita interativamente é uma causa comum.
* Complete a etapa de login antes de iniciar Claude Code, por exemplo `aws sso login --profile myprofile`, para que a cadeia seja resolvida do cache SSO local em vez de aguardar um fluxo de navegador
* Se sua cadeia executa um login interativo que legitimamente precisa de mais de 60 segundos, como SSO com MFA através de um wrapper como `aws-vault`, aumente o limite em milissegundos com [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/pt/env-vars)

<h3 id="bedrock-setup-verification-timed-out-waiting-for-aws">
  Tempo limite de verificação de configuração do Bedrock aguardando AWS
</h3>

Uma chamada para AWS durante o [assistente de configuração do Bedrock](/docs/pt/amazon-bedrock#sign-in-with-bedrock), como a busca de credencial ou a verificação de identidade, não terminou dentro do limite de 60 segundos. O assistente para de aguardar e falha a etapa de verificação:

```text theme={null}
Timed out after 60s waiting for AWS. Check your network and proxy settings; if a credential helper needs longer to prompt you, raise CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS.
```

O número reflete seu limite: 60 segundos por padrão, ou o valor que você define em [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/pt/env-vars).

As causas comuns são uma rede ou proxy que trava solicitações para AWS, incluindo a atualização de token SSO, e um helper de credencial ainda aguardando entrada que você não consegue ver. Aumente o limite apenas quando o helper legitimamente precisa de mais tempo.

Uma única solicitação travada para AWS também pode falhar em seu próprio tempo limite por solicitação, que mostra uma mensagem mais curta na mesma etapa:

```text theme={null}
A request to AWS timed out. Check your network and proxy settings, then try again.
```

Quando os mesmos tempos limite ocorrem na etapa de fixação de modelo, o assistente marca um modelo como `unreachable` em vez de mostrar uma das duas mensagens.

**O que fazer:**

* Execute `aws sts get-caller-identity` no mesmo shell. Se também travar, a travação está fora de Claude Code, em sua rede, seu proxy ou o helper de credencial em seu perfil AWS; corrija isso primeiro.
* Complete qualquer login interativo antes de abrir o assistente, por exemplo `aws sso login --profile myprofile`
* Se um helper de credencial em seu perfil AWS legitimamente precisa de mais de 60 segundos para solicitá-lo, aumente o limite em milissegundos com [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/pt/env-vars)

<h3 id="cloud-gateway-session-expired">
  Sessão de gateway de nuvem expirada
</h3>

Você fez login através de um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), e a sessão de gateway salva nesta máquina expirou e não conseguiu ser renovada, ou o gateway não a aceita mais, por exemplo, após a [rotação do segredo JWT](/docs/pt/claude-apps-gateway-deploy#jwt-secret-rotation) do gateway. Se você vir esta linha quando inicia `claude` interativamente, a sessão abriu desconectada do gateway:

```text theme={null}
Cloud gateway session expired — run /login to reconnect.
```

A mesma linha pode aparecer no meio da sessão quando a credencial de gateway expira e Claude Code não consegue renová-la.

Em uma execução [não interativa](/docs/pt/headless), uma sessão em segundo plano ou outra sessão desatendida, ou um subcomando `claude` diferente de `claude auth`, Claude Code sai com esta mensagem em vez disso quando o gateway não aceita mais a sessão:

```text theme={null}
Cloud gateway <url> no longer accepts this session. Start `claude` and sign in again with /login.
```

**O que fazer:**

* Execute `/login` na sessão e complete o login do navegador
* Para um lançamento não interativo, inicie `claude` no mesmo ambiente, execute `/login`, depois execute novamente seu comando

<h3 id="sign-in-timed-out-while-waiting-for-you-to-continue">
  Tempo limite de login enquanto aguardava você continuar
</h3>

Durante um login do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), o gateway nomeou a conta que fez login, e Claude Code pediu que você a confirmasse antes de salvar a credencial. Você deixou a confirmação aberta após a expiração do próprio login, e o gateway não emitiu token de atualização que pudesse renová-lo, portanto Claude Code não armazenou nada quando você continuou:

```text theme={null}
Sign-in timed out while waiting for you to continue. Try again.
```

**O que fazer:**

* Execute `/login` novamente e confirme a conta antes do login expirar

<h3 id="gateway-refused-the-request">
  Gateway recusou a solicitação
</h3>

Você está conectado através de um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), e uma solicitação retornou um 403: o gateway, ou o upstream por trás dele, a recusou. Fazer login novamente não altera uma recusa, portanto a mensagem aponta para seu administrador de gateway:

```text theme={null}
Gateway refused the request · signing in again won't change this — check with your gateway administrator · API Error: 403 ...
```

**O que fazer:**

* Peça ao seu administrador de gateway para procurar a solicitação. A cauda `API Error:` carrega a recusa que o gateway retornou
* Para administradores: uma [regra de controle de acesso](/docs/pt/claude-apps-gateway-config#http-tuning) no gateway retorna um 403 que o [log de auditoria](/docs/pt/claude-apps-gateway-deploy#logs) registra com seu motivo, e uma negação de autorização de um upstream passa através de [Mensagens de erro de upstream](/docs/pt/claude-apps-gateway-config#upstream-error-messages)

Antes da v2.1.273, um 403 em uma sessão de gateway mostrava a mensagem genérica `Please run /login` ou `Failed to authenticate`, e fazer login novamente não limpava a recusa.

<h2 id="network-and-connection-errors">
  Erros de rede e conexão
</h2>

A maioria desses erros significa que uma solicitação de rede do Claude Code falhou ao alcançar seu destino, ou algo entre Claude Code e a API alterou a resposta no caminho de volta; quando uma entrada também tem uma causa local, como uma falha na escrita de arquivo, seu corpo diz isso. Geralmente originam-se em sua rede local, proxy ou firewall, ou na política de rede do ambiente de nuvem.

<h3 id="unable-to-connect-to-api">
  Unable to connect to API
</h3>

A conexão TCP com a API falhou ou nunca foi concluída. Para os códigos de erro de conexão comuns, o nome da mensagem indica o tipo de falha e mantém o código entre parênteses:

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

Um código que Claude Code não reconhece aparece como `Unable to connect to API` seguido pelo código entre parênteses. Algumas dessas mensagens podem mostrar mais de um código: `Connection refused` pode mostrar `ConnectionRefused` ou `ECONNREFUSED`, por exemplo, e `Can't reach the API server` pode mostrar `ENOTFOUND` ou `FailedToOpenSocket`.

Antes da v2.1.227, cada uma dessas mensagens codificadas lia `Unable to connect to API` seguida pelo código, por exemplo `Unable to connect to API (ECONNREFUSED)`.

As causas comuns incluem sem acesso à internet, uma VPN que bloqueia `api.anthropic.com`, ou um proxy corporativo necessário que não está configurado.

**O que fazer:**

* Confirme que você pode alcançar o host da API a partir do mesmo shell executando `curl -I https://api.anthropic.com`. No Windows PowerShell use `curl.exe -I https://api.anthropic.com` para que o alias `Invoke-WebRequest` integrado não seja usado.
* Se você estiver atrás de um proxy corporativo, defina `HTTPS_PROXY` antes de iniciar Claude Code e veja [Network configuration](/docs/pt/network-config)
* Se você rotear através de um gateway LLM ou relay, defina [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) para seu endereço. Veja [Connect Claude Code to an LLM gateway](/docs/pt/llm-gateway-connect) para configuração.
* Certifique-se de que seu firewall permite os hosts listados em [Network access requirements](/docs/pt/network-config#network-access-requirements)
* Falhas intermitentes são [retentadas automaticamente](#automatic-retries); falhas persistentes apontam para um problema de rede local

Se `curl` funcionar mas Claude Code ainda falhar, a causa geralmente é algo entre o runtime e a rede em vez da rede em si:

* No Linux e WSL, verifique `/etc/resolv.conf` para um nameserver inacessível. WSL em particular pode herdar um resolver quebrado do host.
* No macOS, um cliente VPN que foi desconectado ou desinstalado pode deixar uma interface de túnel ou regra de roteamento para trás. Verifique `ifconfig` para interfaces `utun` obsoletas e remova a extensão de rede da VPN em Configurações do Sistema.
* Docker Desktop e runtimes de contêiner similares podem interceptar tráfego de saída. Saia deles e tente novamente para descartar isso.

<h3 id="unable-to-connect-to-anthropic-services">
  Unable to connect to Anthropic services
</h3>

Durante a configuração de primeira execução, Claude Code verifica se consegue alcançar `api.anthropic.com` e `platform.claude.com` antes de mostrar a etapa de login. Quando qualquer uma das verificações falha, Claude Code imprime o motivo e sai.

```text theme={null}
Unable to connect to Anthropic services
Failed to connect to api.anthropic.com: ECONNREFUSED
Connection to api.anthropic.com timed out after 10 seconds
A proxy is configured via HTTPS_PROXY. Check that it allows connections to the host above.
```

Claude Code envia a verificação através da mesma [proxy configuration](/docs/pt/network-config) que as solicitações de API e dá a cada sonda 10 segundos. Quando a sonda com falha passou por um proxy, a mensagem nomeia a variável de ambiente que a configurou, como `HTTPS_PROXY`. Antes da v2.1.222, a verificação usava um transporte de proxy diferente sem timeout: atrás de uma URL de proxy com o esquema `https://`, ela poderia travar em `Checking connectivity...` indefinidamente e depois falhar mesmo que as solicitações de API através do mesmo proxy funcionem.

Claude Code pula essa verificação quando um [managed settings file, MDM policy, ou policy helper](/docs/pt/managed-settings) define [`forceLoginMethod`](/docs/pt/settings-reference#forceloginmethod) como `"gateway"`, ou define [`forceLoginGatewayUrl`](/docs/pt/settings-reference#forcelogingatewayurl) sem `forceLoginMethod`. Com qualquer uma das configurações, Claude Code abre a etapa de login na tela **Cloud gateway** em vez de um método de login Anthropic. Claude Code também pula a verificação quando uma fonte de configurações gerenciadas na máquina existe mas não pode ser lida, já que essa fonte pode conter a configuração do gateway. Antes da v2.1.247, Claude Code executava a verificação sob essa configuração também, e saía com esse erro quando os endpoints da Anthropic eram inacessíveis.

**O que fazer:**

* Se a mensagem nomear uma variável de proxy, verifique se seu valor aponta para o proxy correto e peça ao seu time de rede para permitir conexões HTTPS através dele para o host na mensagem. Veja [Network configuration](/docs/pt/network-config).
* Trabalhe através das verificações em [Unable to connect to API](#unable-to-connect-to-api). O teste `curl` e a orientação de firewall lá se aplicam a essa verificação também.
* Se sua organização faz login através de um [cloud gateway](/docs/pt/claude-apps-gateway) e esse erro aparece na primeira execução, atualize para Claude Code v2.1.247 ou posterior.
* Se sua rede está aberta e a falha persiste, Claude Code pode não estar [disponível em seu país](https://www.anthropic.com/supported-countries)

<h3 id="socket-is-closed">
  Socket is closed
</h3>

`Socket is closed` significa que a conexão que carregava uma resposta de streaming foi fechada enquanto a resposta ainda estava chegando. A causa mais comum é um proxy corporativo no Windows descartando um túnel estabelecido no meio da resposta.

Dependendo de quão longe a resposta havia progredido, Claude Code retenta a solicitação, mantém o que Claude produziu, ou encerra o turno. Veja [Automatic retries](#automatic-retries).

Antes da v2.1.214, Claude Code não retentava essa falha, e o turno parava com um erro contendo `Socket is closed`.

**O que fazer:**

* Se você vir esse erro, atualize para v2.1.214 ou posterior com `claude update`, depois envie sua mensagem novamente
* Se os turnos continuarem falhando atrás do mesmo proxy após atualizar, trabalhe através de [Unable to connect to API](#unable-to-connect-to-api) e verifique a configuração do proxy em [Network configuration](/docs/pt/network-config)

<h3 id="api-returned-an-empty-or-malformed-response">
  API returned an empty or malformed response
</h3>

Claude Code mostra esse erro quando sua retentativa sem streaming de uma solicitação de streaming com falha obtém um status de sucesso HTTP mas o corpo não é uma mensagem de API Claude: comumente uma página de erro HTML ou login, um corpo vazio, ou JSON em outro formato. Um proxy, gateway, ou página de login de rede respondendo no lugar da API é a fonte usual. Claude Code não retenta a solicitação, e o turno termina com esse erro.

```text theme={null}
API returned an empty or malformed response (HTTP 200) — check for a proxy or gateway intercepting the request.
```

Após essa abertura, a mensagem relata o que voltou e qual solicitação falhou:

* Uma cláusula `Response:` com o tipo de conteúdo, o tipo de corpo, como `body is an HTML page` ou `empty body`, seu tamanho em bytes, e se a resposta carregava um id de solicitação Anthropic. Quando a resposta nomeia um servidor reconhecível, como `nginx` ou `cloudflare`, ou carrega headers intermediários, como `cf-ray` ou `via`, a cláusula lista esses também.
* Uma sentença nomeando o id da solicitação de streaming com falha e a falha que acionou a retentativa. Quando um stream havia aberto antes da falha, também relata quantos eventos de stream chegaram e, se algum chegou, quanto tempo o stream havia ficado silencioso quando a tentativa falhou.

Antes da v2.1.234, a mensagem terminava após `intercepting the request`.

Antes da v2.1.271, uma resposta que carregava uma mensagem de API válida sob um tipo de conteúdo não-JSON como `text/plain` também terminava o turno com esse erro. Alguns gateways LLM usam esse tipo de conteúdo para a resposta sem streaming.

**O que fazer:**

* Leia a cláusula `Response:` para ver qual sistema respondeu. Um corpo HTML, nenhum id de solicitação Anthropic, ou um servidor nomeado como `nginx` ou `cloudflare` significa que algo entre Claude Code e a API respondeu em seu lugar
* Se você rotear através de um [LLM gateway](/docs/pt/llm-gateway-connect#troubleshoot-gateway-errors), teste a rota com uma solicitação direta e corrija o hop que retorna a resposta não-API
* Em uma rede com uma página de login, como Wi-Fi de convidado, complete o login em um navegador, depois tente novamente
* Se apenas a rota sem streaming através de seu gateway está quebrada, defina [`CLAUDE_CODE_DISABLE_NONSTREAMING_FALLBACK=1`](/docs/pt/env-vars#variables) para que uma solicitação que falha no meio do stream vá para o caminho de retentativa normal em vez desse fallback, exceto quando o endpoint de streaming em si retorna `404`, onde Claude Code ainda faz fallback

<h3 id="streaming-response-ended-before-any-complete-data-was-received">
  Streaming response ended before any complete data was received
</h3>

Uma resposta de streaming de seu provedor de modelo foi concluída sem entregar nenhum dado utilizável, então Claude Code reenviou a solicitação sem streaming para terminar o turno. Claude Code mostra o aviso uma vez por sessão, apenas em sessões interativas. Antes da v2.1.239, Claude Code retentava silenciosamente sem streaming.

```text theme={null}
Streaming response ended before any complete data was received. Retrying without streaming. If this keeps happening, check any proxy or gateway between Claude Code and your model provider.
```

Claude Code envia cada solicitação afetada duas vezes: a tentativa de streaming vazia e a retentativa. A causa usual é um proxy ou gateway que consome ou transforma o corpo da resposta de streaming no caminho de volta.

**O que fazer:**

* Configure qualquer proxy ou gateway entre Claude Code e seu provedor de modelo para passar corpos de resposta de streaming e seus headers através sem modificação
* Em [Amazon Bedrock](/docs/pt/amazon-bedrock), veja [Streaming errors behind a gateway or proxy](/docs/pt/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy) para os requisitos de header e corpo

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock streaming response has an unexpected content-type
</h3>

Um gateway ou proxy entre Claude Code e [Amazon Bedrock](/docs/pt/amazon-bedrock) está transformando o corpo da resposta de streaming ou seu header `Content-Type`. Amazon Bedrock transmite respostas como `application/vnd.amazon.eventstream`. Em vez de decodificar um corpo que não consegue ler, Claude Code rejeita uma resposta de streaming bem-sucedida que relata um content-type diferente. Claude Code não retenta a solicitação.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Antes da v2.1.208, a mesma configuração incorreta aparecia como `API Error: Truncated event message received` após todo o corpo ter sido armazenado em buffer.

**O que fazer:**

* Configure o gateway para passar o corpo da resposta `InvokeModelWithResponseStream` e seu header `Content-Type` através sem modificação. Um intermediário que re-emite o stream como server-sent events é uma causa comum.
* Definir [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/pt/env-vars) oculta esse erro, mas Claude Code não decodifica um corpo binário sob um header reescrito, então essas solicitações fazem fallback para um caminho sem streaming mais lento. Veja [Streaming errors behind a gateway or proxy](/docs/pt/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  SSL certificate errors
</h3>

Um proxy ou appliance de segurança em sua rede está interceptando tráfego TLS com seu próprio certificado, e Claude Code não confia nele.

```text theme={null}
Unable to connect to API: SSL certificate verification failed (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
Unable to connect to API: Self-signed certificate detected (SELF_SIGNED_CERT_IN_CHAIN). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
```

Antes da v2.1.273, ambas as mensagens terminavam em `Check your proxy or corporate SSL certificates`, sem o código OpenSSL ou a dica `NODE_EXTRA_CA_CERTS`.

A partir da v2.1.199, uma falha de validação de certificado não é retentada, então esse erro aparece na primeira tentativa em vez de após o [retry budget](#automatic-retries) completo. Versões anteriores gastavam alguns minutos retentando antes de mostrá-lo. Condições TLS transitórias, como um timeout de handshake, ainda retentam.

Durante `/login` e a verificação de conectividade de inicialização, a mesma falha produz uma mensagem diferente:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

Em [Amazon Bedrock](/docs/pt/amazon-bedrock), as solicitações que Claude Code em si envia para AWS, como as chamadas de credencial de função STS e SSO, descoberta de modelo, e as verificações do assistente de configuração, dependem da mesma configuração de certificado. Veja [Certificate errors behind a TLS-inspecting proxy](/docs/pt/amazon-bedrock#certificate-errors-behind-a-tls-inspecting-proxy).

**O que fazer:**

* Exporte o bundle de CA de sua organização e aponte Claude Code para ele com `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Veja [Network configuration](/docs/pt/network-config#custom-ca-certificates) para instruções de configuração completa
* Não defina `NODE_TLS_REJECT_UNAUTHORIZED=0`, que desabilita a validação de certificado inteiramente

<h3 id="host-not-allowed-in-a-cloud-session">
  Host not allowed in a cloud session
</h3>

Uma solicitação HTTP de saída de uma sessão de nuvem ou rotina foi bloqueada pela política de rede do ambiente.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Você também pode ver um certificado TLS que não corresponde ao certificado real do destino. Sessões de nuvem rotam tráfego de saída através de um proxy que aplica a política de rede, então um certificado incompatível significa que o proxy terminou a conexão, não o destino.

Isso não é um problema de rede do lado do cliente. Sessões de nuvem e [routines](/docs/pt/routines) executam dentro de uma VM em sandbox cuja tráfego de saída através da rede da sessão é filtrado para a [allowlist do ambiente de nuvem](/docs/pt/cloud-environments); [operações GitHub](/docs/pt/cloud-environments#github-proxy) e tráfego do conector MCP usam canais separados, é por isso que podem continuar funcionando enquanto outros hosts são bloqueados. O ambiente **Default** usa acesso **Trusted**, que permite a [allowlist padrão](/docs/pt/cloud-environments#default-allowed-domains) de registros de pacotes, APIs de provedores de nuvem, registros de contêiner, e domínios de desenvolvimento comuns e bloqueia outros domínios nesse caminho.

**O que fazer:**

Essas etapas alteram um de seus próprios ambientes. Um [organization-shared environment](/docs/pt/cloud-environments#organization-shared-environments) abre como somente leitura no seletor, então peça a um Owner para alterar seu acesso de rede de **Trusted** para **Custom** na página **Cloud environments** em [admin settings](https://claude.ai/admin-settings).

* Abra a rotina para edição, ou inicie uma sessão de nuvem. Selecione o ícone de nuvem mostrando o nome do seu ambiente, como **Default**, para abrir o seletor. Passe o mouse sobre seu ambiente e clique no ícone de configurações.
* No diálogo **Update cloud environment**, altere **Network access** de **Trusted** para **Custom**, depois adicione o domínio bloqueado a **Allowed domains**. Digite um domínio por linha. Marque **Also include default list of common package managers** para manter a [allowlist padrão](/docs/pt/cloud-environments#default-allowed-domains) junto com seus domínios personalizados. Selecione **Full** em vez disso se você quiser acesso irrestrito.
* Clique em **Save changes**. A próxima execução usa a allowlist atualizada.

Veja [Network access](/docs/pt/cloud-environments#network-access) para níveis de acesso e a allowlist padrão. Sessões de CLI local não são afetadas por essa política.

<h3 id="the-proxy-refused-the-connection">
  The proxy refused the connection
</h3>

Você vê essa mensagem quando Claude lê um [artifact](/docs/pt/artifacts) através do proxy que você definiu em `HTTPS_PROXY` ou uma [proxy variable](/docs/pt/network-config#environment-variables) relacionada. O conteúdo do artifact vem de `*.frame.claudeusercontent.com`, então Claude Code primeiro envia ao proxy uma solicitação `CONNECT` pedindo para abrir um túnel para esse host. Quando o proxy recusa, nada alcança o host, e a mensagem carrega o status HTTP do proxy:

```text theme={null}
artifact content fetch failed (proxy refused the connection: HTTP 407)
artifact content fetch failed (proxy refused the connection: HTTP 403)
the proxy refused the connection to the artifact's content host (HTTP 502)
```

O status é a resposta do proxy para o `CONNECT`. O host nunca respondeu, então cada status aponta para um fix diferente:

* `HTTP 407`: o proxy requer credenciais que não recebeu. Coloque-as na URL do proxy, como [Basic authentication](/docs/pt/network-config#basic-authentication) mostra.
* `HTTP 403`: o proxy recusa fazer túnel para `*.frame.claudeusercontent.com`. Peça a quem executa o proxy para permitir esse host, que [Network access requirements](/docs/pt/network-config#network-access-requirements) lista.
* Qualquer outro status, como `HTTP 502`: o proxy não abriu o túnel por seu próprio motivo, como falhar ao alcançar o host. Procure o status nos logs do proxy.
* `unreadable reply` no lugar de um status: o que está no endereço do proxy não respondeu com uma linha de status HTTP. Verifique se o endereço é um proxy HTTP.

**O que fazer:**

* Verifique o endereço e credenciais na variável de proxy, como [Proxy configuration](/docs/pt/network-config#proxy-configuration) descreve, depois execute `curl -x http://proxy.example.com:8080 -I https://api.anthropic.com` a partir do shell em que você inicia Claude Code, usando sua própria URL de proxy. No Windows PowerShell, execute `curl.exe`. Se essa sonda falhar da mesma forma, corrija a configuração do proxy primeiro. Se funcionar, a recusa é específica para o host do artifact.
* Se sua rede deixa Claude Code alcançar o host do artifact diretamente, adicione `.frame.claudeusercontent.com` a [`NO_PROXY`](/docs/pt/network-config#environment-variables). Mantenha a entrada estreita: uma entrada `.claudeusercontent.com` mais ampla também contorna o proxy para `bridge.claudeusercontent.com`, que organizações com [IP allowlisting](/docs/pt/network-config#organization-ip-allowlists-and-proxy-egress) precisam manter no proxy.

Antes da v2.1.238, Claude Code relatava um túnel recusado como um erro de rede genérico.

<h3 id="the-cloud-environments-service-returned-an-empty-or-unexpected-response">
  The cloud environments service returned an empty or unexpected response
</h3>

Claude Code solicita sua lista de [cloud environments](/docs/pt/cloud-environments) em vários pontos, como quando você cria uma sessão de nuvem a partir da CLI ou executa [`/remote-env`](/docs/pt/cloud-environments#select-an-environment-from-the-cli). Quando não consegue ler a resposta do servidor, mostra uma dessas mensagens:

```text theme={null}
The cloud environments service returned an empty response (HTTP 200 with no body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 with a non-JSON body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 without a usable environments list). This is usually temporary — try again in a moment.
```

O servidor aceitou a solicitação mas respondeu com um corpo que não é a lista de ambientes: vazio, não JSON, ou JSON sem a lista. Isso geralmente acompanha uma disrupção do lado do serviço e se limpa por conta própria. Dependendo da superfície que solicitou a lista, Claude Code pode adicionar um prefixo, como `couldn't list environments:` no diálogo `/remote-env`.

**O que fazer:**

* Tente novamente a ação. Claude Code solicita a lista novamente cada vez
* Se a mensagem continuar aparecendo, verifique [status.claude.com](https://status.claude.com) para incidentes ativos

Antes da v2.1.236, Claude Code mostrava um TypeError JavaScript bruto em vez dessas mensagens.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Couldn't reconnect to your Remote Control session
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

Retomar com `claude --resume` ou `claude --continue` reconecta à sessão [Remote Control](/docs/pt/remote-control) registrada nessa conversa. Essa mensagem significa que a reconexão falhou por um motivo que pode ser temporário, como uma interrupção de rede ou um erro de servidor, então Claude Code não consegue confirmar se a sessão remota ainda existe. Sua sessão local continua executando sem Remote Control.

**O que fazer:**

* Execute `/remote-control` para tentar novamente a conexão
* Inicie uma nova sessão com `claude --remote-control` para criar uma nova sessão Remote Control
* Para outras mensagens de inicialização do Remote Control, veja [Troubleshoot Remote Control](/docs/pt/remote-control#troubleshooting)

Se o servidor relatar em vez disso que a sessão anterior se foi, você não vê essa mensagem. Claude Code inicia uma nova sessão em seu lugar ou mostra [`Previous session is unavailable — run /remote-control to start a new one`](/docs/pt/remote-control#previous-session-is-unavailable), dependendo do [registro de reconexão da conversa](/docs/pt/remote-control#resume-outcomes). De v2.1.227 através v2.1.231, Claude Code mostrou uma mensagem que começa com `Remote Control could not resume the previous session under the current login` em vez disso, e [versões anteriores se comportaram diferentemente novamente](/docs/pt/remote-control#reconnect-history).

<h3 id="sessions-ended-while-this-machine-was-offline">
  Sessions ended while this machine was offline
</h3>

Claude Code mostra essa mensagem no terminal executando [`claude remote-control`](/docs/pt/remote-control#start-a-remote-control-session) depois que sua máquina estava offline tempo suficiente para que o servidor limpasse o ambiente Remote Control que sua máquina estava servindo. As sessões nesse ambiente terminaram, e você não consegue retomá-las. A contagem é o número de sessões que terminaram.

```text theme={null}
2 sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.
```

**O que fazer:**

* Quando Claude Code lista worktrees mantidas sob essa mensagem, pegue qualquer trabalho não commitado delas
* Execute `claude remote-control` para iniciar um ambiente fresco

<h3 id="couldnt-share-the-transcript">
  Couldn't share the transcript
</h3>

Depois que você concorda em compartilhar sua transcrição de sessão de um prompt de pesquisa, como a [session quality survey](/docs/pt/data-usage#session-quality-surveys), Claude Code a carrega para Anthropic, ou salva um arquivo local em vez disso em provedores de terceiros, em sessões [Claude apps gateway](/docs/pt/claude-apps-gateway), e quando nenhuma credencial Anthropic está disponível. Essa mensagem significa que o compartilhamento não foi concluído.

```text theme={null}
Couldn't share the transcript.
```

O upload deve caber em um limite de 8 MiB. Em uma sessão longa, Claude Code progressivamente descarta partes do compartilhamento, as configurações de modelo da última solicitação primeiro, depois a conversa estruturada e transcrições de subagent, e mostra essa mensagem apenas quando nenhuma versão reduzida pode ser enviada ou um erro de rede ou servidor para o upload. Quando Claude Code salva um arquivo local em vez disso, a mensagem significa que não conseguiu escrever o arquivo.

**O que fazer:**

* Execute `/feedback` para enviar a transcrição com uma descrição do que aconteceu. Veja [Report an error](#report-an-error) se `/feedback` não estiver disponível em seu ambiente
* Se outras solicitações também estão falhando, verifique sua conexão de rede e veja [Unable to connect to API](#unable-to-connect-to-api)

<h2 id="request-errors">
  Erros de solicitação
</h2>

Esses erros estão relacionados ao conteúdo da sua solicitação. A maioria retorna da API depois que ela rejeita a solicitação; alguns são produzidos localmente pelo Claude Code antes de qualquer solicitação ser enviada.

<h3 id="prompt-is-too-long">
  Prompt é muito longo
</h3>

A conversa mais os arquivos anexados excedem a janela de contexto do modelo.

```text theme={null}
Prompt is too long
```

Em uma sessão interativa, Claude Code mostra esse erro como:

```text theme={null}
Context limit reached · /compact or /clear to continue
```

A linha nomeia apenas `/clear` quando [`DISABLE_COMPACT`](/docs/pt/env-vars) está definido. Formas mais longas do erro, como a forma de falha de compactação abaixo, mantêm a redação `Prompt is too long ·`. Na saída `-p` e na transcrição, o texto permanece `Prompt is too long`.

Quando você desativou o auto-compact nas suas [configurações de usuário](/docs/pt/settings-reference#autocompactenabled), a linha também diz:

```text theme={null}
Context limit reached · /compact or /clear to continue · auto-compact is off · /config to turn it on
```

O botão **Auto-compact** em `/config` escreve `autoCompactEnabled` nas configurações de usuário. A dica aparece apenas quando uma mudança `/config` teria efeito. Por exemplo, não aparece quando [`DISABLE_AUTO_COMPACT`](/docs/pt/env-vars) ou [`DISABLE_COMPACT`](/docs/pt/env-vars) desativou o auto-compact. Também não aparece quando um escopo de precedência mais alta, como configurações de projeto ou gerenciadas, define `autoCompactEnabled` como `false`. Antes da v2.1.235, a linha não tinha dica de auto-compact.

Amazon Bedrock relata essa condição como `Input is too long for requested model.`, que Claude Code trata da mesma forma. Antes da v2.1.217, Claude Code não reconhecia a redação do Bedrock, então o auto-compact nunca era acionado e `/compact` falhava com o mesmo erro.

Um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway-config#upstream-error-messages) relata essa condição como `capability_rejected: prompt_too_long` quando um upstream de nuvem rejeita a solicitação na forma de erro própria do provedor. Claude Code trata o token da mesma forma que `Prompt is too long`. Antes da v2.1.228, Claude Code não reconhecia o token, então o auto-compact não era acionado.

Quando a compactação automática foi executada nessa volta e falhou em um erro subjacente, como um modelo indisponível ou uma falha de autenticação, a mensagem nomeia esse erro após um separador:

```text theme={null}
Prompt is too long · automatic compaction failed: <the underlying error>
```

Resolva o erro nomeado primeiro; `/compact` falha no mesmo erro até você fazer isso. Antes da v2.1.229, uma compactação automática falhada exibia `Prompt is too long` sem a causa.

Quando a compactação automática é executada nesse erro, ela normalmente resume suas trocas mais antigas e mantém as mais novas. Como último recurso, Claude Code resume de forma diferente:

* Quando não consegue resumir nenhuma troca completa, Claude Code mantém seu prompt mais novo palavra por palavra e resume tudo antes dele.
* Nesse caso, quando a conversa não termina com seu prompt, Claude Code resume a conversa inteira.

Claude Code pula essa recuperação quando o conteúdo que ele carregaria adiante não contém resposta do modelo e menos de cerca de 1.000 tokens do seu próprio texto, como uma tentativa curta enviada após uma colagem de tamanho excessivo. Execute `/clear` para começar do zero. Antes da v2.1.269, a compactação falhava sempre que não conseguia resumir uma troca completa, então uma sessão nesse estado atingia esse erro novamente a cada volta.

Uma conversa de troca única não tem voltas anteriores para resumir. Quando a compactação automática teria sido executada em uma, Claude Code pula a tentativa e explica o que preenche a solicitação. Quando a API não relata contagens de tokens em seu erro, a mensagem lê:

```text theme={null}
Prompt is too long · this conversation is a single exchange and cannot be compacted — the request size comes mostly from system prompt, tool definitions, or attachments.
```

Quando a API relata contagens de tokens em seu erro, Claude Code as compara com sua própria estimativa do tamanho da conversa para dizer qual é a maior parte da solicitação: o conteúdo próprio da conversa ou o prompt do sistema, definições de ferramentas e conteúdo de anexo que Claude Code envia com ela. Quando o conteúdo próprio da conversa é a maior parte da solicitação, a mensagem lê:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) and this conversation's own content is most of it. A single-exchange conversation cannot be compacted; start with less content (smaller files or pasted text).
```

Quando a maior parte da solicitação está fora da conversa, a mensagem lê:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) but this conversation is only ~<conversation tokens> tokens — the rest is system prompt, tool definitions, and attachment content. A single-exchange conversation cannot be compacted; reduce attached files/tools or start with less context.
```

Antes da v2.1.162, Claude Code tentava a compactação mesmo assim e exibia o `Prompt is too long` simples quando falhava.

**O que fazer:**

* Execute `/compact` para resumir voltas anteriores e liberar espaço, ou `/clear` para começar do zero. Se `/compact` responder `Not enough messages to compact.`, a conversa é uma troca única sem nada anterior para resumir, então o espaço é ocupado por esse prompt e o que Claude Code envia com cada solicitação: execute `/clear` e reenvie com menos texto colado ou anexos menores, ou reduza as definições de ferramentas e arquivos de memória usando as etapas abaixo
* Execute `/context` para ver um detalhamento do que está consumindo a janela: prompt do sistema, ferramentas, arquivos de memória e mensagens
* Desabilite servidores MCP que você não está usando com `/mcp disable <name>` para remover suas definições de ferramentas do contexto
* Reduza arquivos de memória `CLAUDE.md` grandes ou mova instruções para [regras com escopo de caminho](/docs/pt/memory#path-specific-rules) que carregam apenas quando relevante
* Suagentes herdam todas as definições de ferramentas MCP da sessão pai, que podem preencher sua janela de contexto antes da primeira volta. Desabilite servidores MCP que você não está usando antes de gerar suagentes.
* O auto-compact está ativado por padrão e normalmente previne esse erro. Se você o desativou em `/config` ou com [`DISABLE_AUTO_COMPACT`](/docs/pt/env-vars), ative-o novamente. Se você mantê-lo desativado, execute `/compact` você mesmo antes da janela se preencher.

Veja [Explore the context window](/docs/pt/context-window) para uma visualização interativa de como o contexto se preenche.

<h3 id="context-exceeds-the-token-limit">
  O contexto excede o limite de tokens
</h3>

`/context` mostra esse aviso no topo de sua saída quando a conversa cresceu além da janela de contexto do modelo. As solicitações falham com [`Prompt is too long`](#prompt-is-too-long) até você liberar espaço. Uma sessão interativa mostra esse erro como a linha `Context limit reached`.

```text theme={null}
Context exceeds the 200k-token limit by 94k tokens — run /compact or /clear to continue.
```

Quando o limite que você excedeu é uma janela de compactação, como o limite de 200K em modelos de contexto 1M, o aviso lê diferentemente. Uma janela de compactação pode ficar abaixo da janela de contexto do modelo, então solicitações após ela ainda podem ter sucesso.

```text theme={null}
Context is 94k tokens past the 200k-token compaction window — run /compact to reduce usage.
```

Ambas as formas nomeiam `/clear` em vez de `/compact` quando você definiu [`DISABLE_COMPACT`](/docs/pt/env-vars).

**O que fazer:**

* Em uma conversa com múltiplas voltas, execute `/compact` para resumir voltas anteriores e liberar espaço. Para começar do zero, execute `/clear`
* Para mais formas de reduzir o uso, veja [Prompt is too long](#prompt-is-too-long)

Antes da v2.1.216, `/context` mostrava uso acima de 100% sem linha de aviso explicando o que isso significava ou como se recuperar.

<h3 id="error-during-compaction-conversation-too-long">
  Erro durante compactação: Conversa muito longa
</h3>

`/compact` em si falhou porque não há contexto livre suficiente para manter o resumo que produz.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Isso pode acontecer quando a janela já está cheia no momento em que o auto-compact é acionado, ou quando você executa `/compact` depois de ver [`Prompt is too long`](#prompt-is-too-long). Em uma sessão interativa, esse erro é a linha `Context limit reached`.

**O que fazer:**

* Pressione Esc duas vezes para abrir a lista de mensagens e voltar várias voltas. Isso remove as mensagens mais recentes do contexto. Então execute `/compact` novamente.
* Se voltar não liberar espaço suficiente, execute `/clear` para iniciar uma sessão nova. Sua conversa anterior é preservada e pode ser reabierta com `/resume`.

Essa mensagem e outras falhas `/compact` são exibidas em estilo de erro. Antes da v2.1.216, elas eram renderizadas no mesmo estilo atenuado que a saída de comando bem-sucedida, então você poderia ler uma compactação falhada como um sucesso.

<h3 id="request-too-large">
  Solicitação muito grande
</h3>

O corpo da solicitação bruta excedeu o limite de 32MB da API antes da tokenização, geralmente por causa de conteúdo colado grande, resultados de ferramentas ou anexos. Esse limite é separado da [janela de contexto](#prompt-is-too-long).

```text theme={null}
Request too large (max 32MB). Accumulated images and attachments in the conversation pushed the request over the limit. Run /compact, or double press esc to go back and remove attachments.
```

Quando a solicitação foi direto para a API Claude e a própria API a rejeitou, Claude Code mede a conversa e redige a mensagem se a recuperação pode funcionar. Através de um proxy, gateway ou provedor de nuvem você obtém a mensagem geral. As formas medidas:

* `Request too large (max 32MB; 20.1MB of about 33.4MB is images or documents).`: imagens ou documentos empurraram a solicitação além do limite. Claude Code tenta novamente com eles removidos.
* `Request too large for the API's 32MB request limit`: as mensagens sozinhas estão além do limite, então a mensagem diz `compacting cannot make it fit` e Claude Code não tenta novamente. Em [modo não interativo](/docs/pt/headless), a mensagem diz para você reduzir a entrada ou iniciar uma nova sessão.

Antes da v2.1.212, conversas com imagens acumuladas suficientes falhavam a cada volta com `Request too large (max 32MB). Double press esc to go back and try with a smaller file.` Antes da v2.1.229, Claude Code mostrava o conselho de anexo para cada rejeição, mesmo quando a compactação não podia ajudar.

**O que fazer:**

* Se a mensagem diz `compacting cannot make it fit`, pressione Esc duas vezes para voltar além da volta que adicionou o conteúdo grande, ou execute `/clear` para começar do zero
* Caso contrário, execute `/compact`, que remove imagens e anexos acumulados
* Referencie arquivos grandes por caminho em vez de colar seu conteúdo, para que Claude possa lê-los em pedaços
* Para imagens, veja [Image was too large](#image-was-too-large) abaixo

<h3 id="image-was-too-large">
  Imagem era muito grande
</h3>

Uma imagem colada ou anexada excede os limites de tamanho ou dimensão da API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code substitui a imagem não processável por um espaço reservado de texto e tenta novamente, então mensagens subsequentes têm sucesso. Em versões antes de 2.1.142, uma imagem colada poderia permanecer na conversa e repetir o mesmo erro em cada mensagem subsequente. Para se recuperar nessas versões, pressione Esc duas vezes e volte além da volta onde a imagem foi adicionada.

**O que fazer:**

* Redimensione a imagem antes de colar. A API aceita imagens até 8000 pixels na borda mais longa para uma única imagem, ou 2000 pixels quando muitas imagens estão em contexto.
* Faça uma captura de tela mais apertada da região relevante em vez da tela inteira

<h3 id="unable-to-resize-image">
  Não foi possível redimensionar a imagem
</h3>

Claude Code não conseguiu reduzir uma imagem anexada antes de enviá-la para a API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
Unable to resize image — it is a CMYK JPEG, which Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Re-save it as an RGB PNG or JPEG and try again.
Unable to resize image — it is an animated WebP whose first frame Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Save its first frame as a PNG or JPEG and try again.
Unable to resize image — its pixels could not be decoded (the file may be damaged, or use an encoding Claude Code cannot read), and it is over the … API limit (… raw, … base64), so it cannot be sent. Re-save it as a PNG or JPEG and try again.
```

Claude Code normalmente redimensiona imagens grandes automaticamente. Esses erros significam que a imagem não pôde ser decodificada ou redimensionada para caber dentro dos limites da API.

**O que fazer:**

* Se a mensagem pedir para você converter a imagem, converta-a para PNG, JPEG, GIF ou WebP e anexe-a novamente. Claude Code pode verificar dimensões para esses formatos a partir do cabeçalho do arquivo, sem decodificar a imagem.
* Se a mensagem relatar um limite de dimensão ou tamanho, redimensione ou recomprima a imagem abaixo desse limite antes de anexar.
* Se a mensagem nomear uma causa, como um JPEG CMYK, um WebP animado ou um arquivo possivelmente danificado, salve novamente a imagem no formato que a mensagem sugere e anexe-a novamente.

<h3 id="pdf-errors">
  Erros de PDF
</h3>

O PDF que você anexou não pôde ser processado. As mensagens são mostradas aqui em sua forma não interativa; em uma sessão interativa, elas solicitam que você pressione esc duas vezes e tente novamente.

```text theme={null}
PDF too large (max 100 pages, 20MB). Try reading the file a different way (e.g., extract text with pdftotext).
PDF is password protected. Try using a CLI tool to extract or convert the PDF.
The PDF file was not valid. Try converting it to text first (e.g., pdftotext).
```

**O que fazer:**

* Para PDFs de tamanho excessivo, peça a Claude para ler um intervalo de páginas com a ferramenta Read em vez de anexar o arquivo inteiro, ou extraia texto com uma ferramenta como `pdftotext` e referencie o arquivo de saída por caminho
* Para PDFs protegidos ou inválidos, remova a senha ou re-exporte o arquivo de seu aplicativo de origem, depois tente novamente

<h3 id="extra-inputs-are-not-permitted">
  Entradas extras não são permitidas
</h3>

Um proxy ou gateway LLM entre Claude Code e a API removeu o cabeçalho de solicitação `anthropic-beta`, então a API rejeitou campos que dependem dele.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code envia campos apenas de beta como `context_management` e `effort` junto com um cabeçalho `anthropic-beta` que os habilita. Quando um gateway encaminha o corpo mas remove o cabeçalho, a API vê campos que não reconhece.

**O que fazer:**

* Configure seu gateway para encaminhar o cabeçalho `anthropic-beta`. Veja [feature pass-through](/docs/pt/llm-gateway-protocol#feature-pass-through) para o que os gateways devem encaminhar.
* Como fallback, defina [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/pt/env-vars) antes de iniciar. [Disable pre-release capabilities](/docs/pt/llm-gateway-protocol#disable-pre-release-capabilities) cobre o escopo exato.

<h3 id="tool-input-schema-is-invalid">
  O esquema de entrada da ferramenta é inválido
</h3>

Uma ferramenta na solicitação declarou um `input_schema` que falha na validação JSON Schema da API, então a API rejeitou a solicitação inteira. O número após `tools.` é a posição da ferramenta falhada na lista de ferramentas da solicitação, não um nome que você possa procurar.

```text theme={null}
API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid
API Error: 400 ... tools.N.custom.input_schema.properties: Property keys should match pattern '^[a-zA-Z0-9_.-]{1,64}$'
```

A primeira forma significa que o esquema não é um JSON Schema draft 2020-12 válido. A segunda significa que um nome de propriedade de nível superior não corresponde ao padrão que a mensagem cita.

Claude Code [exclui ferramentas MCP cujo esquema de entrada falharia nessa validação](/docs/pt/mcp#tools-with-invalid-input-schemas) quando carrega as ferramentas de um servidor, então solicitações normalmente nunca incluem uma.

Em uma [implantação onde a busca de sinalizadores está desativada](/docs/pt/env-vars#features-that-need-feature-flag-fetching), ou em uma máquina cujos sinalizadores nunca chegaram, Claude Code registra no log do servidor qual ferramenta seria rejeitada mas a envia mesmo assim, então esse erro ainda pode ocorrer.

O erro também pode ocorrer para uma ferramenta cujo esquema declara um dialeto JSON Schema diferente de draft 2020-12 em `$schema`. Claude Code não verifica esses esquemas contra o meta-esquema JSON Schema, embora a verificação de nome de propriedade de nível superior ainda se aplique.

Antes da v2.1.216, nenhuma implantação executava as verificações de exclusão.

**O que fazer:**

* Se sua versão do Claude Code é anterior à v2.1.216, execute `claude update`.
* Remova ou [desabilite](/docs/pt/mcp#disable-a-server-without-removing-it) o servidor MCP que declara o esquema inválido. O erro nomeia a ferramenta apenas por posição. Na v2.1.216 ou posterior, verifique o log de cada servidor para uma linha nomeando uma ferramenta cujo esquema de entrada seria rejeitado. Se nenhum log nomear uma, desabilite servidores um de cada vez.
* Se você mantém o servidor, corrija o `input_schema` da ferramenta. O esquema deve ser um JSON Schema válido, e nomes de propriedade de nível superior devem ter 1 a 64 caracteres e usar apenas letras ASCII e dígitos, `_`, `.` e `-`. Veja [Tools with invalid input schemas](/docs/pt/mcp#tools-with-invalid-input-schemas).

<h3 id="theres-an-issue-with-the-selected-model">
  Há um problema com o modelo selecionado
</h3>

O nome do modelo configurado não foi reconhecido ou sua conta não tem acesso a ele. A partir da v2.1.160, a dica final, mostrada aqui em sua forma interativa, varia por superfície.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**O que fazer:**

* **CLI interativa**: execute `/model` para escolher entre modelos disponíveis para sua conta.
* **Modo não interativo (`-p`)**: passe `--model` com um alias ou ID válido, ou defina [`ANTHROPIC_MODEL`](/docs/pt/env-vars). O texto de erro mostra `Run --model` nessa superfície.
* **Agent SDK**: o texto de erro omite a dica porque o modelo é definido programaticamente. Defina [`model` em `Options`](/docs/pt/agent-sdk/typescript#options) em TypeScript ou [`ClaudeAgentOptions(model=...)`](/docs/pt/agent-sdk/python#claudeagentoptions) em Python, e trate o erro estruturado `model_not_found` para exibir seu próprio retry ou seletor de modelo.
* Use um alias como `sonnet` ou `opus` em vez de um ID versionado completo. Aliases resolvem para um padrão mantido para que não fiquem obsoletos. Veja [Model configuration](/docs/pt/model-config).
* Se o modelo errado continua voltando na CLI, um ID obsoleto está definido em algum lugar. Verifique os locais onde você pode definir um modelo em [ordem de prioridade](/docs/pt/model-config#setting-your-model) e remova o valor obsoleto.
* Um modelo recém-lançado pode estar disponível na API Anthropic antes de Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry oferecer. Se você fixou um novo ID de modelo em um desses provedores e vê esse erro, verifique o catálogo de modelos do seu provedor para disponibilidade em sua região e mantenha a versão anterior fixada até que a nova apareça lá.
* Claude Code relata um login claude.ai expirado como [Login expired](#login-expired), não como esse erro. Antes da v2.1.206, um login expirado que não podia mais ser atualizado falhava em cada modelo com esse erro; execute `/login` se você vir isso em uma versão mais antiga.
* Para implantações do Google Cloud's Agent Platform, veja [Google Cloud's Agent Platform troubleshooting](/docs/pt/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Model is not a recognized model id
</h3>

A string de modelo que você passou para uma mudança de modelo não é um alias de modelo, um ID de modelo que essa versão do Claude Code conhece, ou um ID que começa com `claude-`. As causas usuais são um erro de digitação no ID, um nome de exibição como `Sonnet 5` onde o ID `claude-sonnet-5` é esperado, ou um alias que apenas versões mais novas do Claude Code reconhecem. Claude Code rejeita a mudança imediatamente. Antes da v2.1.200, Claude Code salvava a string e falhava na próxima solicitação com [There's an issue with the selected model](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

A dica final nomeia o alias ou ID de modelo mais próximo. Quando nada é próximo o suficiente, lê `Run /model to see available models.` em vez disso.

Claude Code produz esse erro localmente no momento em que a mudança é solicitada, antes de qualquer solicitação de API ser feita. Aplica-se quando um modelo é definido através do método [Agent SDK](/docs/pt/agent-sdk/typescript) `setModel()`, por um aplicativo como o [Desktop app](/docs/pt/desktop) que executa o CLI do Claude Code para você, ou quando você escolhe um modelo de um dispositivo conectado através de [Remote Control](/docs/pt/remote-control). Antes da v2.1.260, a verificação não cobria escolhas de Remote Control, então Claude Code aplicava a escolha e a próxima solicitação falhava com [There's an issue with the selected model](#theres-an-issue-with-the-selected-model).

**O que fazer:**

* Execute `/model` sem argumento para abrir o seletor e escolher entre os modelos disponíveis para sua conta, depois passe o alias ou ID mostrado lá
* Se você usou um alias que uma versão mais nova do Claude Code suporta, execute `claude update`. Um ID completo que começa com `claude-` passa nessa verificação local mesmo quando o modelo é mais novo que sua versão do Claude Code. O servidor ainda pode exigir uma versão mínima para esse modelo; veja [Claude Code does not support this model](#claude-code-does-not-support-this-model).
* Um modelo salvo antes da v2.1.200 não é reparado por essa verificação. Se um valor obsoleto continua voltando, remova-o dos locais listados em [Setting your model](/docs/pt/model-config#setting-your-model).
* A verificação é executada apenas na API Anthropic. Em qualquer outro provedor ou gateway, incluindo um `ANTHROPIC_BASE_URL` customizado, o provedor define os nomes de modelo, então Claude Code aceita qualquer string e a passa. Claude Code ainda pode escrever a [linha de diagnóstico de modelo não reconhecido](#unrecognized-model-id-on-a-request) no tempo de solicitação, em cada provedor.

<h3 id="model-not-found">
  Modelo não encontrado
</h3>

Você escolheu um modelo com `/model <name>` e Claude Code não conseguiu confirmar que um modelo com esse nome existe. Quando o nome não é um [alias de modelo](/docs/pt/model-config#model-aliases) ou outra ortografia que Claude Code aceita localmente, `/model` o verifica com uma solicitação mínima de API, e esse erro é geralmente a resposta do seu endpoint de API. Um nome que não pode ser um ID de modelo, como um contendo espaços, recebe a mesma mensagem.

```text theme={null}
Model 'claude-opus-9' not found
```

Em provedores com IDs de modelo específicos do provedor, a mensagem pode adicionar uma sugestão `Try '...' instead` que nomeia o ID do seu provedor para um modelo de fallback.

**O que fazer:**

* Execute `/model` sem argumento e escolha entre os modelos disponíveis para sua conta, ou use um [alias de modelo](/docs/pt/model-config#model-aliases) como `sonnet`, que resolve para um padrão mantido
* Se você digitou um ID completo, verifique-o contra o catálogo de modelos do seu provedor. Um modelo recém-lançado pode estar disponível na API Anthropic antes de seu provedor ou região oferecer.
* Antes da v2.1.265, `/model` também rejeitava a ortografia do alias `opusplan[1m]` com esse erro. Nessas versões, atualize Claude Code ou defina o modelo em [settings](/docs/pt/model-config#setting-your-model) ou com `--model` em vez disso.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus não está disponível com o plano Claude Pro
</h3>

Seu plano de assinatura ativo não inclui o modelo que você selecionou.

```text theme={null}
Claude Opus is not available with the Claude Pro plan. If you have updated your subscription plan recently, run /logout and /login for the plan to take effect.
```

**O que fazer:**

* Execute `/model` e selecione um modelo que seu plano inclui
* Se você atualizou seu plano recentemente e ainda vê isso, execute `/logout` depois `/login`. O token armazenado reflete seu plano no momento em que você se conectou, então atualizar em claude.ai não entra em vigor em uma sessão existente até você se autenticar novamente.
* Veja [claude.com/pricing](https://claude.com/pricing) para quais modelos cada plano inclui

<h3 id="claude-code-does-not-support-this-model">
  Claude Code não suporta este modelo
</h3>

A API recusou a solicitação com um 400 porque sua versão do Claude Code está abaixo de um mínimo necessário. Ou o modelo que você selecionou requer uma versão mais nova, que o servidor verifica por modelo, ou a política da sua organização requer uma. O 400 carrega o código de erro `claude_code_version_too_old`, e a mensagem diz qual mínimo se aplica.

```text theme={null}
API Error: 400 Claude Code 2.1.219 does not support this model; version 2.1.255 or newer is required. Run 'claude update', or update the Claude desktop app, then try again.
```

A redação da política organizacional lê:

```text theme={null}
API Error: 400 Claude Code 2.1.240 is older than the minimum version required by your organization's policy. Run 'claude update', or update the Claude desktop app, to continue.
```

**O que fazer:**

* Execute `claude update` ou atualize o aplicativo Claude desktop, depois inicie uma nova sessão
* Para a redação por modelo, você pode continuar trabalhando na sessão atual alternando para outro modelo com `/model`
* Para a redação da política organizacional, atualize antes de continuar

<h3 id="model-is-restricted-by-your-organizations-settings">
  O modelo é restringido pelas configurações da sua organização
</h3>

Seu administrador de organização desabilitou este modelo no console de administração claude.ai, ou ele é excluído por uma lista de permissões [`availableModels`](/docs/pt/model-config#restrict-model-selection) em configurações gerenciadas. Quando o modelo restringido foi definido com `--model`, `ANTHROPIC_MODEL` ou a configuração `model`, Claude Code substitui um modelo permitido e continua. Digitar `/model <name>` para um modelo restringido é rejeitado com `Run /model to choose a different model.` e a sessão mantém seu modelo atual. O aviso de substituição também pode aparecer no meio da sessão após um administrador desabilitar o modelo em que uma sessão está sendo executada no console de administração claude.ai.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Um aviso prefixado com um nome de agente, skill ou comando significa que a restrição se aplicou ao [modelo solicitado do suagente](/docs/pt/sub-agents#choose-a-model): o suagente é executado no modelo substituído e o modelo da sua sessão não é alterado. Antes da v2.1.223, Claude Code mostrava o aviso apenas para suagentes lançados com a ferramenta Agent.

Claude Code trata um alias de família de modelo, um de `opus`, `sonnet`, `haiku` ou `fable`, como uma solicitação para essa família em vez de sua versão mais nova. Na API Anthropic e em [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), um alias de família restringido resolve para a versão mais nova da família que sua organização e a lista de permissões `availableModels` permitem, e o aviso de substituição nomeia essa versão. Claude Code rejeita `/model <alias>` apenas quando cada versão da família é restringida. Antes da v2.1.205, um alias de família era substituído ou rejeitado com base apenas em sua versão mais nova, mesmo quando uma versão mais antiga da mesma família era permitida.

**O que fazer:**

* Execute `/model` para escolher entre os modelos que sua organização permite. Modelos restritos estão ocultos do seletor.
* Se o modelo restringido foi definido em `--model`, `ANTHROPIC_MODEL`, o campo `model` de um arquivo de configurações ou o frontmatter `model` de um [suagente](/docs/pt/sub-agents#choose-a-model), skill ou comando, remova ou atualize esse valor para que o aviso não recorra
* Se você precisa de acesso ao modelo restringido, peça ao administrador da sua organização para habilitá-lo. Veja [Organization model restrictions](/docs/pt/model-config#organization-model-restrictions).

<h3 id="model-switch-was-blocked-by-a-premodelswitch-hook">
  A mudança de modelo foi bloqueada por um hook PreModelSwitch
</h3>

Um [hook PreModelSwitch](/docs/pt/hooks#premodelswitch) não aprovou a mudança de modelo que você ou um cliente solicitou, então a sessão mantém seu modelo atual. Quando a mudança veio de um host [Agent SDK](/docs/pt/agent-sdk/overview) ou [Remote Control](/docs/pt/remote-control) em vez de um comando que você digitou, a mensagem lê `Model switch blocked by a PreModelSwitch hook` sem nomear o modelo de destino.

```text theme={null}
Model switch to Opus 4.6 was blocked by a PreModelSwitch hook: Opus 4.6 is retired for this project. Use a newer model.
```

A razão após os dois pontos diz o que recusou a mudança:

* **Uma razão que um hook escreveu**: um hook PreModelSwitch forneceu essa razão quando [negou a mudança ou pediu confirmação](/docs/pt/hooks#premodelswitch-decision-control). Aborde o que ele pede ou escolha um modelo que seus hooks permitem.
* **`PreModelSwitch hook <name> did not respond before its timeout`**: um hook que não responde antes de seu [timeout](/docs/pt/hooks#timeouts) bloqueia a mudança. Corrija o comando pendurado ou aumente o `timeout` desse hook, depois mude novamente.
* **`confirmation required, and this session cannot ask`**: um hook respondeu `ask` sem uma razão, e uma solicitação de controle não tem forma de mostrar o prompt de confirmação. Um comando `/model` em uma execução [`-p`](/docs/pt/headless) relata a mesma condição com `(run /model interactively to confirm)` após a razão. Faça a mudança de uma sessão interativa ou altere a decisão do hook para este modelo.
* **`so organization-managed PreModelSwitch hooks could not be checked`**: Claude Code não conseguiu dizer quais hooks PreModelSwitch seus [plugins gerenciados](/docs/pt/settings-reference#enabledplugins) da organização entregam, por exemplo porque um plugin gerenciado falhou ao carregar. Um desses hooks pode bloquear a mudança, então Claude Code recusa em vez de aplicar a mudança desmarcada. O início da razão nomeia o que falhou. Claude Code re-verifica a cada tentativa de mudança, então uma falha que desde então foi limpa para de bloquear; se continuar falhando, execute `claude --debug` e mude novamente para capturar os detalhes, depois corrija o plugin ou peça ao seu administrador para corrigi-lo.
* **`a PreModelSwitch hook failed before answering`** ou **`PreModelSwitch hooks were cancelled (the control stream closed) before answering`**: a execução do hook terminou sem um veredicto, e Claude Code não trata isso como aprovação. Execute `claude --debug` para ver o que falhou, depois mude novamente.

Antes da v2.1.260, a recusa de plugin gerenciado lia `plugin hooks could not be loaded, so PreModelSwitch hooks could not be checked; see the debug log`. Claude Code tentou novamente o carregamento do plugin uma vez e depois recusou mudanças posteriores na sessão, mesmo quando sua organização não gerenciava plugins. Reinicie a sessão para executar o carregamento do plugin novamente nessas versões.

<h3 id="couldnt-save-it-as-your-default">
  Não foi possível salvá-lo como seu padrão
</h3>

Você escolheu um modelo para salvar como seu padrão, por exemplo com `/model <name>` ou `Enter` no seletor `/model`, e Claude Code não conseguiu escrever a escolha no arquivo de configurações do usuário, `~/.claude/settings.json`. A mudança em si foi aplicada, então a sessão atual é executada no modelo que você escolheu, mas seu padrão não é alterado e a próxima sessão começa no valor antigo.

```text theme={null}
Set model to Fable 5.1 for this session only · couldn't save it as your default: ~/.claude/settings.json can't be written (EROFS)
```

A razão após o caminho do arquivo diz o que falhou:

* **`can't be written (<code>)`**: a escrita falhou com o código de erro do sistema operacional entre parênteses, como `EROFS` quando o arquivo, ou o arquivo para o qual ele aponta, fica em um sistema de arquivos que recusa escritas. Torne o arquivo gravável e mude novamente. Se outra ferramenta gera o arquivo, defina a chave `model` nessa ferramenta em vez disso; veja [A change you made in Claude Code is lost in new sessions](/docs/pt/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **`isn't valid JSON`**: o arquivo no disco não analisa, e Claude Code o deixa intocado em vez de sobrescrever conteúdo que não consegue ler de volta. Corrija o erro de sintaxe, depois mude novamente; veja [Fix a broken settings file](/docs/pt/settings#fix-a-broken-settings-file).

Um aviso terminando `couldn't confirm it was saved as your default (~/.claude/settings.json is still being written)` significa que a escrita não havia terminado após três segundos. Continua em segundo plano, então o padrão ainda pode ser salvo; verifique qual modelo sua próxima sessão começa ou execute `/model <name>` novamente.

Antes da v2.1.265, o aviso dizia que o modelo foi `saved as your default for new sessions` mesmo quando a escrita falhou.

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled não é suportado para este modelo
</h3>

Sua versão do Claude Code é mais antiga que o mínimo para o modelo selecionado. O CLI enviou uma configuração de pensamento que o modelo não aceita mais.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**O que fazer:**

* Execute `claude update` e reinicie Claude Code. Opus 4.7 precisa da v2.1.111 ou posterior. Opus 4.8 precisa da v2.1.154 ou posterior. Sonnet 5 precisa da v2.1.197 ou posterior. Opus 5 precisa da v2.1.219 ou posterior. Opus 5.5 precisa da v2.1.280 ou posterior
* Se você não conseguir atualizar, execute `/model` e selecione Opus 4.6 ou Sonnet 4.6 em vez disso
* Se você atingir isso no [Agent SDK](/docs/pt/agent-sdk/overview), atualize o pacote SDK em vez disso. Opus 4.8 precisa do SDK TypeScript v0.3.154 ou posterior e SDK Python v0.2.88 ou posterior. Sonnet 5 precisa do SDK TypeScript v0.3.197 ou posterior. Opus 5 precisa do SDK TypeScript v0.3.219 ou posterior. Opus 5.5 precisa do SDK TypeScript v0.3.280 ou posterior

<h3 id="effort-isnt-available-with-thinking-turned-off">
  Effort não está disponível com o pensamento desativado
</h3>

Você desativou o [pensamento estendido](/docs/pt/model-config#extended-thinking) e executou em um [nível de esforço](/docs/pt/model-config#adjust-effort-level) acima de `high`. O modelo não aceita essa combinação, então a API rejeitou a solicitação.

```text theme={null}
API Error: Effort 'xhigh' isn't available with thinking turned off on this model · run /effort high to continue, or turn thinking back on (unset MAX_THINKING_TOKENS=0)
```

**O que fazer:**

* [Abaixe o nível de esforço](/docs/pt/model-config#set-the-effort-level) para `high` ou abaixo.
* Ative o pensamento novamente, por exemplo desativando [`MAX_THINKING_TOKENS`](/docs/pt/env-vars) ou removendo [`"alwaysThinkingEnabled": false`](/docs/pt/settings-reference#alwaysthinkingenabled) de suas configurações.

Antes da v2.1.242, Claude Code mostrava a própria mensagem da API: `API Error: 400 output_config.effort 'xhigh' is not supported when thinking is disabled on this model. Use effort 'high' or below, or enable thinking.` Antes da v2.1.251, Claude Code enviava a solicitação no nível de esforço que você definiu, então Opus 5 rejeitava cada solicitação acima de `high` com pensamento desativado. Claude Code agora envia esforço `high` em vez disso para modelos que sabe rejeitarem a combinação, como Opus 5, então na v2.1.251 ou posterior esse erro chega a você apenas de um modelo que Claude Code não sabe rejeitar.

<h3 id="thinking-budget-exceeds-output-limit">
  O orçamento de pensamento excede o limite de saída
</h3>

O orçamento de pensamento estendido configurado excede o comprimento máximo de resposta, então não há espaço deixado para a resposta real.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code ajusta esses valores automaticamente na API Anthropic. Você normalmente vê esse erro em Amazon Bedrock ou Google Cloud's Agent Platform quando [`MAX_THINKING_TOKENS`](/docs/pt/env-vars) está definido mais alto que o limite de saída do provedor, ou quando o modo de plano aumenta o orçamento de pensamento.

**O que fazer:**

* Abaixe `MAX_THINKING_TOKENS` ou aumente [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/pt/env-vars) acima do orçamento de pensamento
* Veja [Extended thinking](/docs/pt/model-config#extended-thinking) para como o orçamento interage com o comprimento de saída

<h3 id="tool-use-or-thinking-block-mismatch">
  Incompatibilidade de bloco de uso de ferramenta ou pensamento
</h3>

O histórico de conversa chegou à API em um estado inconsistente, geralmente após uma chamada de ferramenta ser interrompida ou uma volta ser editada no meio do fluxo.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 orphaned tool_result in conversation history. Run /rewind to recover the conversation.
API Error: 400 duplicate tool_use ID in conversation history. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Todas as variantes significam a mesma coisa: a sequência de blocos `tool_use`, `tool_result` e `thinking` no histórico não corresponde mais ao que a API espera.

**O que fazer:**

* Se você está usando Opus 4.7 ou Opus 4.8, execute `claude update` primeiro. Versões antes da v2.1.156 podem acionar esse erro durante o uso normal de ferramentas, e `/rewind` não o limpa.
* Execute `/rewind` ou pressione Esc duas vezes para voltar a um checkpoint antes da volta corrompida e continuar de lá. Veja [Checkpointing](/docs/pt/checkpointing) para como checkpoints são criados e restaurados.

<h3 id="unsupported-tool-content-removed">
  Conteúdo de ferramenta não suportado removido
</h3>

Quando Claude Code se conecta diretamente à API Anthropic e carrega ou visualiza uma sessão salva, ele remove conteúdo de ferramenta que a API Anthropic não aceita e deixa essa linha onde o conteúdo removido estava entre dois blocos de pensamento:

```text theme={null}
[Unsupported tool content removed]
```

Tal conteúdo chega a um arquivo de sessão quando algo diferente da API Anthropic responde no formato da API, tipicamente um proxy de terceiros definido através de [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) que traduz chamadas de ferramentas de outro provedor. Claude Code o remove apenas quando a sessão se conecta diretamente à API Anthropic e carrega o histórico salvo como está quando a sessão é executada através de um proxy ou em outro provedor. Antes da v2.1.246, Claude Code enviava o uso de ferramenta e seu resultado de volta para a API, e cada volta da sessão retomada falhava com um erro 400 como `messages.1.content.0.server_tool_use.name: Input should be 'web_search', 'web_fetch', ...`.

**O que fazer:**

* Nenhuma ação necessária quando você vê a linha de espaço reservado. A sessão continua sem o conteúdo removido.
* Se cada volta de uma sessão retomada falhar com o erro 400 em vez disso, execute `claude update` e retome a sessão novamente. Versões antes da v2.1.246 não removem o conteúdo.

<h3 id="role-system-must-precede-an-assistant-message">
  role 'system' deve preceder uma mensagem 'assistant'
</h3>

A API recusou a solicitação com um 400 porque uma mensagem de sistema fica em uma posição na conversa que ela não aceita:

```text theme={null}
API Error: 400 messages.6: role 'system' must precede an 'assistant' message or end the array; ...
```

Claude Code envia algum de seu texto de lembrete e anexo como mensagens de sistema dentro da conversa. Quando a API recusa a posição de uma, Claude Code tenta novamente a solicitação uma vez com esse texto enviado como mensagens de usuário ordinárias em vez disso. As redações de posicionamento irmão da API, como `use the top-level 'system' parameter for the initial system prompt`, recebem a mesma recuperação.

Quando o erro aparece, a mensagem de sistema recusada não é uma que Claude Code possa remover. Isso geralmente significa um proxy ou [gateway LLM](/docs/pt/llm-gateway) entre Claude Code e a API adicionou uma mensagem de sistema própria ou reordenou a conversa.

**O que fazer:**

* Execute `/clear` para iniciar uma conversa nova. Se o erro retornar lá também, a causa está no caminho da solicitação, não na conversa salva.
* Se o erro se repete a cada volta atrás de um proxy ou gateway configurado através de [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars), conecte sem o proxy para confirmar a fonte e relate o erro a quem o opera

Antes da v2.1.280, Claude Code não reconhecia essa redação, então o erro também aparecia quando a mensagem de sistema recusada era uma que Claude Code mesmo enviou, e cada volta posterior da conversa falhava da mesma forma.

<h3 id="invalid-encrypted-content-in-search-result-block">
  Conteúdo criptografado inválido no bloco search\_result
</h3>

A API recusou a solicitação com um 400 porque o histórico de conversa contém conteúdo de busca na web hospedado que ela não consegue descriptografar. A redação nomeia o campo que ela não consegue ler:

```text theme={null}
API Error: 400 messages.21.content.0: Invalid `encrypted_content` in `search_result` block
API Error: 400 messages.21.content.3.citations.0: Invalid `encrypted_index` in `text` block
API Error: 400 Failed to decrypt web search result content
```

Resultados da [ferramenta de busca na web](/docs/pt/tools-reference#websearch-tool-behavior) hospedada da API carregam campos criptografados que apenas a API pode ler. A API recusa uma solicitação que reproduz conteúdo que ela não consegue descriptografar, como conteúdo produzido para uma organização diferente.

A própria ferramenta [WebSearch](/docs/pt/tools-reference#websearch-tool-behavior) do Claude Code registra resultados de busca como texto simples, então esses blocos geralmente chegam a uma conversa através de um proxy ou [gateway LLM](/docs/pt/llm-gateway) que executou busca na web hospedada em si.

Os blocos recusados permanecem no histórico de conversa, então cada volta posterior e `/compact` falham da mesma forma.

**O que fazer:**

* Execute `/clear` ou inicie uma nova sessão; a nova conversa não carrega os blocos recusados
* Se você executa Claude Code atrás de um proxy ou gateway, relate o erro a quem o opera

<h3 id="usage-policy-refusal">
  Recusa de Política de Uso
</h3>

A API recusou responder porque conteúdo na conversa acionou uma verificação de [Política de Uso](https://www.anthropic.com/legal/aup). A mensagem inclui um ID de Solicitação que você pode citar para suporte se acreditar que a recusa está incorreta.

```text theme={null}
API Error: Opus 4.6 can't help with this. Start a new session to continue.

Send feedback with /feedback or learn more: https://www.anthropic.com/legal/aup
```

A mensagem nomeia o modelo que recusou, ou `Claude` quando nenhum modelo é registrado.

A verificação avalia a conversa completa, não apenas seu prompt mais recente, então enviar uma nova mensagem na mesma sessão geralmente re-aciona a mesma recusa. O mesmo se aplica após sair e reabrir a sessão com `--continue` ou `--resume`, já que a transcrição no disco ainda contém o conteúdo acionador. Em [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) e [Microsoft Foundry](/docs/pt/microsoft-foundry), essa mensagem também cobre solicitações que as medidas de segurança do modelo sinalizaram como um tópico de cibersegurança. Veja [Safety measures flagged a cybersecurity topic](#safety-measures-flagged-a-cybersecurity-topic).

Antes da v2.1.219, a mensagem lia `Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.`

**O que fazer:**

* Pressione Esc duas vezes ou execute `/rewind` para voltar a um checkpoint antes da volta que acionou a recusa, depois reformule ou tome uma abordagem diferente. Veja [Checkpointing](/docs/pt/checkpointing).
* Se você não conseguir identificar qual volta causou, execute `/clear` para iniciar uma conversa nova no mesmo projeto. Sua conversa anterior é preservada no disco e permanece disponível em `/resume`.
* Em [modo não interativo](/docs/pt/headless) (`-p`), onde rewind não está disponível, tente novamente com um prompt reformulado em uma nova sessão sem `--continue`. Verificações de política variam por modelo, então alternar para um modelo diferente com `--model` também pode resolver a recusa em alguns casos.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  As medidas de segurança sinalizaram um tópico de cibersegurança
</h3>

As medidas de segurança do modelo sinalizaram conteúdo na conversa como um tópico de cibersegurança. A mensagem nomeia o modelo que sinalizou a solicitação:

```text theme={null}
API Error: Opus 4.8's safeguards flagged this message. Our intentionally broad safeguards allow us to deliver more capabilities faster, but can sometimes flag legitimate cybersecurity work. Apply to the Cyber Verification Program to reduce these interruptions. Send feedback with /feedback or learn more: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude
```

A mensagem vincula ao [Programa de Verificação Cibernética](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), que concede acesso para trabalho de cibersegurança legítimo. Em Opus 5.5, que requer v2.1.280 ou posterior, a mensagem abre com `Opus 5.5's safeguards flagged this session` em vez disso. Quando a categoria sinalizada tem um modelo de fallback disponível, Claude Code [alterna modelos](/docs/pt/model-config#automatic-model-fallback) em vez de mostrar esse erro.

Em [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) e [Microsoft Foundry](/docs/pt/microsoft-foundry), uma sinalização de cibersegurança produz a mensagem de [recusa de Política de Uso](#usage-policy-refusal) em vez disso.

A proteção em si é do lado do servidor e antecede v2.1.203; lançamentos de cliente desde então mudaram apenas a redação da mensagem.
De v2.1.203 a v2.1.218, a mensagem lia `<model> has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center:` seguido pelo mesmo link do centro de ajuda, e sessões interativas acrescentavam `If you were not engaging in a cybersecurity topic, please send feedback via /feedback.`
Antes da v2.1.203, lia `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` seguido por um link de formulário de isenção.

**O que fazer:**

* Se seu trabalho requer esse conteúdo, solicite acesso através do [Programa de Verificação Cibernética](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Se sua solicitação não era sobre um tópico de cibersegurança, execute `/feedback` para relatar o falso positivo
* Para continuar trabalhando na mesma sessão, pressione Esc duas vezes ou execute `/rewind` para voltar a um checkpoint antes da volta que acionou a sinalização, depois tome uma abordagem diferente. Veja [Checkpointing](/docs/pt/checkpointing).

<h2 id="installation-errors">
  Erros de instalação
</h2>

Esses erros aparecem durante a instalação ou atualização do Claude Code, a partir do [script de instalação](/docs/pt/setup#install-claude-code), `claude install`, ou `claude update`. Para problemas de `command not found`, PATH, permissão e TLS durante a configuração, consulte [Solucionar problemas de instalação e login](/docs/pt/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  A instalação foi interrompida antes de ser concluída
</h3>

O script de instalação relata quando a etapa `claude install` é encerrada por um sinal. No Linux, o código de saída 137 significa que o processo recebeu SIGKILL, e em um host com pouca memória, geralmente é o killer de falta de memória (OOM) do kernel. O script imprime esta explicação e sai com o código 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Para qualquer outro sinal fatal, e para o código de saída 137 no macOS, o script imprime `Installation was killed before it could finish (exit code <N>)` com o código de saída real e omite a explicação de falta de memória. A mensagem vem do script de instalação que macOS e Linux usam, que também cobre instalações dentro do WSL; os scripts de instalação nativos do Windows nunca a imprimem. Antes da v2.1.200, o script saía apenas com a linha `Killed` nua do shell.

**O que fazer:**

* Interrompa outros processos para liberar memória e execute novamente o instalador
* Adicione espaço de swap ou mude para uma instância maior. Consulte [Install killed on low-memory Linux servers](/docs/pt/troubleshoot-install#install-killed-on-low-memory-linux-servers) para os comandos do arquivo de swap.

<h3 id="the-connection-dropped-while-downloading-the-update">
  A conexão foi interrompida durante o download da atualização
</h3>

A conexão com o servidor de download foi fechada enquanto `claude install`, `claude update`, ou o [atualizador automático](/docs/pt/setup#auto-updates) estava buscando o binário do Claude Code, e as tentativas de repetição não se recuperaram. Claude Code tenta novamente o download quando a conexão cai, a transferência trava ou o arquivo baixado falha em sua soma de verificação, até três tentativas no total. Um erro HTTP concluído, como um 404, não é repetido porque o servidor já respondeu. Antes da v2.1.202, uma única conexão interrompida falhava no download imediatamente com o erro nú `aborted` em vez de tentar novamente.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

O texto entre parênteses nomeia qual tentativa falhou e o erro de rede subjacente. `claude update` precede a mensagem com `Error: Failed to install native update` no stderr.

Um download que permanece conectado mas não é concluído em 10 minutos falha com `Download timed out: exceeded the total deadline` em vez disso. Claude Code não tenta novamente um download que expirou, porque uma conexão muito lenta para terminar dentro do prazo não terminará em uma tentativa imediata novamente. As etapas abaixo se aplicam a ambas as mensagens.

A causa usual é um proxy ou gateway que fecha uma transferência longa antes de ser concluída. O binário do Claude Code é um download grande, portanto um limite de conexão de proxy que nunca afeta o tráfego normal da API ainda pode interrompê-lo.

**O que fazer:**

* Execute `claude update` novamente. Em uma rede caso contrário saudável, o download geralmente é bem-sucedido na próxima execução. Para a mensagem de tempo limite, execute-a novamente de uma rede mais rápida ou menos limitada.
* Se sua rede exigir um proxy, defina `HTTPS_PROXY` antes de executar o instalador ou `claude update`. Consulte [Check network connectivity](/docs/pt/troubleshoot-install#check-network-connectivity).
* Se um proxy corporativo continuar fechando a transferência, peça à sua equipe de rede para permitir o download completo de `downloads.claude.ai`. Consulte [Network access requirements](/docs/pt/network-config#network-access-requirements).
* Execute `claude doctor` do seu shell para diagnósticos de instalação

<h2 id="command-line-errors">
  Erros de linha de comando
</h2>

Esses erros vêm do comando `claude` e seus subcomandos, de um nome de comando que você envia no prompt e de comandos como `/security-review` que reúnem contexto executando comandos shell antes de seu prompt ser executado. Eles também vêm de `/tui`, que relança a CLI.

<h3 id="conflict-between-bg-and-print">
  Conflito entre --bg e --print
</h3>

Esta mensagem requer Claude Code v2.1.198 ou posterior. Você combinou `--bg` com `-p` ou `--print` na mesma invocação de `claude`. `--bg` inicia uma [sessão em background](/docs/pt/agent-view#from-your-shell) que você depois anexa com `claude agents`, enquanto `--print` executa [não interativamente](/docs/pt/headless) e nunca inicia a sessão interativa que `claude agents` anexa. Antes da v2.1.198, essa combinação criava silenciosamente um trabalho em background que nunca poderia ser anexado.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**O que fazer:**

* Remova `-p` ou `--print`. `--bg` toma o prompt como seu argumento posicional, então `claude --bg "<task>"` é o comando completo. Veja [Dispatch new agents from your shell](/docs/pt/agent-view#from-your-shell).
* Para executar o prompt não interativamente e imprimir o resultado em vez de criar uma sessão em background, remova `--bg` e execute `claude -p "<task>"`

<h3 id="invalid-agents-configuration">
  Configuração inválida de --agents
</h3>

O valor que você passou para `--agents` é inválido, então `claude` sai com código 1 em vez de iniciar a sessão. Quando você passa `--safe-mode`, `--resume` ou `--continue`, ou define [`CLAUDE_CODE_SAFE_MODE`](/docs/pt/env-vars#variables), Claude Code não verifica o valor e inicia a sessão. Antes da v2.1.242, Claude Code iniciava a sessão mesmo assim e deixava de fora as definições que não conseguia carregar.

```text theme={null}
Error: Invalid --agents configuration:
<what failed>
```

O que segue a primeira linha depende de como o valor falhou. Claude Code executa essas verificações em ordem e para na primeira que falha. Se seu valor tiver dois tipos de problema, você verá o segundo apenas depois de corrigir o primeiro:

1. Quando o valor não é analisado como JSON, Claude Code imprime uma linha `invalid JSON:` com a mensagem do próprio analisador JSON
2. Quando é analisado mas uma definição de agente não corresponde ao esquema para [subagentes definidos por CLI](/docs/pt/sub-agents#choose-the-subagent-scope), Claude Code imprime uma linha por problema
3. Quando um nome de agente começa com `-`, Claude Code imprime `<name>: agent names must not start with '-'`

Quando há mais de 20 linhas de problema, Claude Code imprime as primeiras 20 e substitui o resto por `…and N more`.

**O que fazer:**

* Corrija cada problema que a mensagem lista, depois execute o comando novamente. Veja [the fields a CLI-defined subagent takes](/docs/pt/sub-agents#choose-the-subagent-scope).

<h3 id="cloud-sessions-cannot-be-created-from-a-restricted-session">
  Sessões em nuvem não podem ser criadas a partir de uma sessão --restricted
</h3>

Quando você inicia uma sessão com [`--restricted`](/docs/pt/cli-reference#cli-flags), Claude Code recusa criar [sessões em nuvem](/docs/pt/claude-code-on-the-web#from-terminal-to-cloud) a partir dela, porque a nova sessão seria executada fora do processo restrito e não aplicaria o modo restrito. Claude Code recusa no cliente, antes de contatar o servidor, então nenhuma sessão em nuvem é criada:

```text theme={null}
Cloud sessions cannot be created from a --restricted session: they would not enforce it.
```

**O que fazer:**

* Execute a tarefa localmente na sessão restrita
* Se você controlar como a sessão foi iniciada, inicie uma nova sessão `claude` sem `--restricted` e crie a sessão em nuvem a partir daí

Antes da v2.1.248, Claude Code não tinha a flag `--restricted`; versões anteriores rejeitam a flag em si com um erro de opção desconhecida.

<h3 id="cloud-sessions-are-disabled-by-your-organizations-policy">
  Sessões em nuvem estão desabilitadas pela política da sua organização
</h3>

A política `allow_remote_sessions` da sua organização está desativada, então [sessões em nuvem](/docs/pt/claude-code-on-the-web) e os comandos que as usam não estão disponíveis:

```text theme={null}
Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.
```

A mensagem aparece quando você [cria uma sessão em nuvem a partir do terminal](/docs/pt/claude-code-on-the-web#from-terminal-to-cloud) e quando você envia um comando que precisa de sessões em nuvem, como `/teleport`, `/remote-env` ou `/web-setup`. Antes da v2.1.268, enviar um desses comandos retornava [`Unknown command`](#unknown-command) em vez disso.

Esta é uma política de organização do lado do servidor, então não pode ser substituída por configurações locais, variáveis de ambiente ou flags de CLI.

Se Claude Code ainda não carregou a política da sua organização ou não conseguir buscá-la, esses comandos respondem `Couldn't verify your organization's policy for cloud sessions. Check your network connection, then restart Claude Code and try again.` em vez disso.

**O que fazer:**

* Peça a um [Owner](/docs/pt/server-managed-settings#access-control) em sua organização para habilitar sessões em nuvem nas configurações de administrador do Claude Code em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Se a mensagem disser que não conseguiu verificar a política, verifique sua conexão de rede, depois reinicie Claude Code e tente novamente

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  O valor --json-schema não é um JSON Schema válido
</h3>

O esquema que você passou para [`--json-schema`](/docs/pt/cli-reference#cli-flags) em [modo não interativo](/docs/pt/headless#get-structured-output) falhou na compilação do JSON Schema, então `claude` sai com código 1 em vez de executar o prompt. Antes da v2.1.205, um esquema inválido produzia saída não estruturada sem erro, e qualquer esquema que usasse a palavra-chave `format` era tratado como inválido.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

O texto após o segundo dois-pontos é o diagnóstico do validador e nomeia a palavra-chave ou localização que falhou. Esquemas que usam a palavra-chave `format`, como `"format": "email"`, são válidos: Claude Code aceita `format` como uma anotação e não a aplica.

Claude Code executa duas verificações antes da compilação do esquema: rejeita um valor que não é JSON analisável com `Error: --json-schema is not valid JSON`, e JSON válido que não é um objeto com `Error: --json-schema must be a JSON object`.

**O que fazer:**

* Corrija a parte do esquema que o diagnóstico nomeia, depois execute o comando novamente
* Se o diagnóstico for `schema too large`, reduza o aninhamento do esquema e a reutilização de `$ref`
* Veja [Get structured output](/docs/pt/headless#get-structured-output) para um esquema funcionando e comando

<h3 id="settings-file-exceeds-the-2mib-limit">
  Arquivo de configurações excede o limite de 2MiB
</h3>

O arquivo que você passou para [`--settings`](/docs/pt/cli-reference#cli-flags) é maior que 2 MiB, então `claude` sai com código 1 na inicialização em vez de carregá-lo. Um arquivo de configurações é um pequeno documento JSON, então um arquivo desse tamanho geralmente significa que o caminho aponta para o arquivo errado. Antes da v2.1.214, Claude Code lia o arquivo sem verificação de tamanho, e um arquivo de vários gigabytes ou um arquivo de dispositivo como `/dev/zero` crescia em memória sem limite.

```text theme={null}
Error: Settings file exceeds the 2MiB limit: /path/to/settings.json
```

Claude Code rejeita um caminho `--settings` que não é um arquivo regular da mesma forma: um dispositivo, FIFO ou socket relata `Error: Cannot use settings file (Not a regular file (device, FIFO, or socket))` seguido do caminho, e um diretório relata um motivo `EISDIR`.

**O que fazer:**

* Aponte `--settings` para um arquivo JSON de configurações regular com menos de 2 MiB. Veja [Settings](/docs/pt/settings) para o formato.

<h3 id="the-current-directory-no-longer-exists">
  O diretório atual não existe mais
</h3>

Você iniciou `claude` a partir de um diretório que foi deletado ou movido depois que seu shell entrou nele, por exemplo um worktree ou diretório temporário que outro shell removeu. Claude Code não consegue ler seu diretório de trabalho, então sai com código 1 antes de iniciar a sessão, em modo interativo e [não interativo](/docs/pt/headless) igualmente. Antes da v2.1.239, Claude Code travava com fonte de bundle minificada e um `ENOENT ... uv_cwd` bruto no stderr em vez dessa mensagem.

```text theme={null}
The current directory no longer exists (it was deleted or moved). Start Claude Code from an existing directory.
error: The current working directory was deleted, so that command didn't work. Please cd into a different directory and try again.
```

A causa e a correção são as mesmas para ambas as formas.

Quando Claude Code não consegue ler o diretório de trabalho por um motivo diferente, como uma mudança de permissões, a mensagem nomeia o código de erro em vez disso: `Can't read the current directory (EACCES). Start Claude Code from a different directory.`

No macOS, `EPERM` para um diretório em `~/Desktop`, `~/Documents`, `~/Downloads` ou iCloud Drive geralmente significa que macOS está bloqueando seu aplicativo de terminal dessa pasta. Outros comandos que leem essa pasta falham da mesma forma: `ls` lá relata `Operation not permitted`, mesmo com `sudo`.

**O que fazer:**

* Mude para um diretório que existe, como seu diretório inicial ou de projeto, depois execute `claude` novamente
* Se o diretório foi recriado no mesmo caminho, seu shell ainda mantém o deletado. Execute `cd "$PWD"` ou saia e re-entre no diretório, depois execute `claude` novamente
* Para `EPERM` no macOS, saia do seu aplicativo de terminal com Cmd+Q, abra-o novamente, retorne a essa pasta e execute `claude`. Se `ls` nessa pasta ainda falhar, abra **System Settings > Privacy & Security > Files and Folders**, ative a pasta para seu aplicativo de terminal, depois reabra o terminal

<h3 id="temp-directory-refused-or-cannot-be-created">
  Diretório temporário recusado ou não pode ser criado
</h3>

No macOS e Linux, Claude Code cria um diretório temporário privado na inicialização, `claude-<uid>` sob o diretório temporário do sistema ou a substituição [`CLAUDE_CODE_TMPDIR`](/docs/pt/env-vars). Quando o diretório não pode ser criado, ou uma entrada já naquele caminho falha nas verificações de segurança, Claude Code imprime a falha no stderr e sai com código 1 em vez de iniciar a sessão:

```text wrap theme={null}
ENOSPC: no space left on device, mkdir '/tmp/claude-501'

Temp directory /tmp/claude-501 is not a directory (may be an attacker-planted symlink). Refusing to use it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is owned by uid 502, expected 501. Refusing to use it — another user may have pre-created it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is not readable (its mode may have been altered, or a path component denies search). Refusing to use it — restore its permissions (chmod 0700) or remove it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.
```

**O que fazer:**

* Para `ENOSPC`, libere espaço em disco no volume que contém o diretório temporário
* Para as formas `Refusing to use it`, remova a entrada nomeada em si, não o que um link aponta, e inicie Claude Code novamente; para a forma `owned by uid`, apenas um administrador ou esse usuário pode removê-la
* Para `is not readable`, execute `chmod 0700` no diretório nomeado, ou remova-o e inicie novamente
* Em qualquer um desses casos, defina [`CLAUDE_CODE_TMPDIR`](/docs/pt/env-vars) para um diretório que você controla e inicie Claude Code novamente, deixando o caminho recusado sozinho

<h3 id="directory-couldnt-be-resolved-to-a-real-location">
  Diretório não pôde ser resolvido para um local real
</h3>

Você executou `/add-dir` para um subdiretório do seu diretório de trabalho, e Claude Code não conseguiu resolver o diretório para seu local real.

Você já tem acesso a arquivo para um subdiretório do diretório de trabalho, então `/add-dir` apenas carrega suas skills, comandos e agentes. Antes de carregá-los, Claude Code verifica que o local real do diretório, com quaisquer symlinks resolvidos, está dentro do diretório de trabalho. Quando Claude Code não consegue resolver esse local, não carrega nada e mostra essa mensagem:

```text theme={null}
packages/app couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded. Check that it is a directory inside the working directory and try again.
```

**O que fazer:**

* Verifique se o caminho nomeia um diretório real dentro do diretório de trabalho, depois execute `/add-dir` novamente
* A mensagem não muda seu acesso a arquivo; ela apenas relata que o conteúdo `.claude/` do diretório não foi carregado

Antes da v2.1.261, essa mensagem também aparecia para cada `/add-dir <subdirectory>` quando o diretório de trabalho estava em um automount `/net/<host>`, onde Claude Code recusa resolver caminhos por design; o diretório estava bem e tentar novamente não poderia ajudar.

<h3 id="workspace-not-trusted-when-starting-remote-control">
  Workspace não confiável ao iniciar Remote Control
</h3>

Você iniciou o modo servidor [Remote Control](/docs/pt/remote-control) com `claude remote-control` ou seu alias `claude rc` em um diretório que você não confiou. O comando não mostra o diálogo de confiança do workspace em si, então sai com código 1 e nomeia a correção:

```text theme={null}
Error: Workspace not trusted. Please run `claude` in /Users/you/project first to review and accept the workspace trust dialog.
```

Em seu diretório inicial a mensagem é diferente, porque o diálogo de confiança do workspace nunca salva confiança para o diretório inicial, então aceitá-lo lá não pode satisfazer essa verificação. Antes da v2.1.214, o diretório inicial mostrava a mensagem acima, cujo conselho não pode ter sucesso lá.

```text theme={null}
Error: Workspace not trusted. /Users/you is your home directory, and for security home-directory trust is never saved, so running `claude` here first won't help. Run `claude rc` from a project directory instead (run `claude` there once to accept the trust dialog).
```

**O que fazer:**

* Execute `claude` no diretório, aceite o [diálogo de confiança do workspace](/docs/pt/permissions#project-allow-rules-and-workspace-trust), depois execute `claude remote-control` novamente
* Em seu diretório inicial, mude para um diretório de projeto e inicie Remote Control lá

<h3 id="not-carried-over-to-the-sessions-remote-control-starts">
  Não levado para as sessões que Remote Control inicia
</h3>

Você iniciou [Remote Control](/docs/pt/remote-control) com uma flag global `claude` antes do verbo `remote-control`, uma que restringiria ou configuraria as sessões que Remote Control inicia, como `--settings`, `--setting-sources`, `--permission-mode`, `--disallowed-tools` ou `--mcp-config`. Uma flag colocada antes do verbo nunca chega a essas sessões. Claude Code recusa iniciar em vez disso, nomeando a flag:

```text theme={null}
Error: `--settings` before `remote-control` is not carried over to the sessions Remote Control starts, so Remote Control refuses to start rather than drop it — remove it, and give Remote Control's own options after the verb (see `claude remote-control --help`).
```

Claude Code não recusa flags globais que são inofensivas de descartar, como `--verbose`, `--model` ou um `--session-id` ou `--plugin-dir` injetado por wrapper: ele as ignora e Remote Control inicia.

Claude Code também recusa iniciar para uma flag global que ainda não reconhece como inofensiva, então uma flag adicionada em uma versão mais recente pode aparecer nessa mensagem até uma versão posterior marcá-la como inofensiva.

**O que fazer:**

* Remova a flag de antes do verbo e passe [as opções próprias do Remote Control](/docs/pt/remote-control#start-a-remote-control-session) depois dele; `claude remote-control --help` as lista
* Quando a flag recusada é `--permission-mode`, execute `claude remote-control --permission-mode <mode>` para definir o modo de permissão para as sessões que Remote Control inicia

Antes da v2.1.248, `claude remote-control` não aceitava suas próprias flags quando uma flag global vinha primeiro, e o comando falhava com um erro de opção desconhecida.

<h3 id="claude-import-is-not-yet-available-in-this-build">
  claude import ainda não está disponível nesta compilação
</h3>

Você executou [`claude import`](/docs/pt/cli-reference#cli-commands), e Claude Code encontrou o fluxo de importação desativado, então o comando sai com código 1 em vez de iniciar a importação. Antes da v2.1.222, uma compilação com o fluxo de importação desativado tratava `import` como um prompt e iniciava uma sessão interativa em vez de imprimir essa mensagem.

```text theme={null}
`claude import` is not yet available in this build. Run `claude` and use /mcp or edit ~/.claude/settings.json directly.
```

Claude Code ativa `claude import` através de uma flag de recurso que busca da Anthropic e armazena em cache no disco. Essa mensagem significa que o valor em cache está desativado. A causa geralmente é uma das seguintes:

* Você não iniciou uma sessão desde a instalação, então Claude Code ainda não buscou a flag. O primeiro `claude import` pode imprimir isso mesmo quando o recurso está disponível para você.
* Você usa Claude Code através do Amazon Bedrock, da Agent Platform do Google Cloud, do Microsoft Foundry ou do Claude Platform na AWS, ou através de um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway#availability-and-limitations). Claude Code não busca flags de recurso nessas sessões, então `claude import` permanece indisponível.
* Você definiu `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `DISABLE_GROWTHBOOK` ou [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars), que desativam a busca de flags de recurso, então `claude import` permanece indisponível.

**O que fazer:**

* Em uma instalação nova, inicie `claude`, aguarde o carregamento da sessão, saia e execute `claude import` novamente
* Onde a busca de flags de recurso permanece desativada, configure você mesmo: adicione servidores MCP com [`claude mcp add`](/docs/pt/mcp#installing-mcp-servers) e crie os [arquivos `CLAUDE.md`](/docs/pt/memory#how-claude-md-files-load), [skills e comandos](/docs/pt/skills#where-skills-live) e [subagentes](/docs/pt/sub-agents#choose-the-subagent-scope) que você quer levar. A mensagem também nomeia `~/.claude/settings.json`. Da configuração que `claude import` leva, esse arquivo contém apenas o [modo de permissão](/docs/pt/settings-reference#permission-settings); Claude Code não lê servidores MCP dele.

<h3 id="could-not-read-claude-code-config">
  Não foi possível ler a configuração do Claude Code
</h3>

Você executou [`claude import`](/docs/pt/cli-reference#cli-commands) enquanto Claude Code não conseguia analisar `~/.claude.json`, o arquivo onde armazena seu login e estado por projeto. O subcomando lê esse arquivo para verificar disponibilidade mas não mostra o diálogo de recuperação que a sessão interativa mostra, então sai com código 1. Antes da v2.1.222, `claude import` com um arquivo de configuração ilegível iniciava uma sessão interativa, cujo diálogo de recuperação tratava o arquivo.

```text theme={null}
Could not read Claude Code config — run `claude` with no arguments to recover it.
```

**O que fazer:**

* Execute `claude` sem argumentos. Claude Code detecta o arquivo inválido e oferece redefini-lo. Depois execute `claude import` novamente.
* Para manter edições manuais que você fez, corrija a sintaxe JSON em `~/.claude.json` em um editor em vez disso, depois execute `claude import` novamente

<h3 id="could-not-import-a-server-from-claude-desktop">
  Não foi possível importar um servidor do Claude Desktop
</h3>

Claude Code não conseguiu adicionar um dos servidores que você selecionou em `claude mcp add-from-claude-desktop`. O comando ainda importa os outros servidores selecionados e imprime uma linha por servidor que não conseguiu adicionar. Antes da v2.1.205, o primeiro servidor que falhou parou a importação e nenhum dos servidores selecionados foi adicionado.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

O texto após o nome do servidor é o motivo. O mais comum é a verificação de nome: Claude Desktop permite caracteres em nomes de servidor, como espaços e períodos, que `claude mcp` restringe a letras, números, hífens e underscores. Outros motivos incluem uma configuração de servidor que falha na validação e um servidor bloqueado pela [política MCP](/docs/pt/managed-mcp) da sua organização.

**O que fazer:**

* Renomeie o servidor em `claude_desktop_config.json` para usar apenas letras, números, hífens e underscores, depois execute `claude mcp add-from-claude-desktop` novamente
* Adicione esse servidor diretamente com `claude mcp add` ou `claude mcp add-json` sob um nome válido. Veja [Import MCP servers from Claude Desktop](/docs/pt/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="cannot-add-mcp-server-to-the-managed-scope">
  Não é possível adicionar servidor MCP ao escopo gerenciado
</h3>

Você executou `claude mcp add` ou `claude mcp add-json` com `--scope managed`. Esse escopo contém os servidores que sua organização fornece através da configuração gerenciada [`managedMcpServers`](/docs/pt/settings-reference#managedmcpservers). Claude Code os lê apenas de configurações gerenciadas, então o comando não consegue escrever um servidor nesse escopo.

```text theme={null}
Cannot add MCP server to scope: managed
```

**O que fazer:**

* Adicione o servidor a um escopo que você pode escrever: `local`, `user` ou `project`. Sem `--scope`, o comando usa `local`. Veja [MCP installation scopes](/docs/pt/mcp#mcp-installation-scopes)
* Para fornecer o servidor a cada usuário em sua organização, adicione-o a [`managedMcpServers`](/docs/pt/settings-reference#managedmcpservers) nas configurações gerenciadas que você implanta

<h3 id="cant-read-mcp-json">
  Não é possível ler .mcp.json
</h3>

Um comando que lê o [`.mcp.json`](/docs/pt/mcp#project-scope) do projeto, como `claude mcp add` ou `claude mcp add-json` com `--scope project`, ou `claude mcp remove`, descobriu que o arquivo em seu diretório atual não é um arquivo regular ou é maior que 2 MiB, então sai com esse erro em vez de ler o arquivo.

```text theme={null}
Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes. Fix or remove it, then run the command again.
```

Antes da v2.1.257, um FIFO em `.mcp.json` deixava o comando esperando para sempre sem saída, e um symlink para um arquivo de dispositivo como `/dev/zero` crescia em memória até o processo ser morto.

**O que fazer:**

* Verifique o que está em `.mcp.json` em seu diretório atual. Substitua-o por um arquivo JSON ordinário no [formato de escopo de projeto](/docs/pt/mcp#project-scope), ou delete-o, depois execute o comando novamente.

<h3 id="anthropic-hosted-and-doesnt-support-local-oauth">
  Servidor é hospedado pela Anthropic e não suporta OAuth local
</h3>

Você iniciou um sign-in para um servidor MCP cuja URL aponta para um host de conector hospedado pela Anthropic que autentica através de um provedor de identidade de terceiros. Esses hosts incluem `microsoft365.mcp.claude.com`, `gmail.mcp.claude.com` e `gcal.mcp.claude.com`. Claude Code recusa iniciar seu fluxo OAuth local para esses hosts tanto do painel `/mcp` quanto de `claude mcp login`, porque [seu sign-in funciona apenas através de claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai).

```text theme={null}
"gmail" is Anthropic-hosted and doesn't support local OAuth. Connect it via Settings → Connectors on claude.ai (requires `claude login`), then it'll be available here automatically.
```

Claude Code corresponde a esses hosts por URL, então a mensagem aparece quando um servidor que você adicionou com `claude mcp add` ou em `.mcp.json` aponta para um deles.

**O que fazer:**

* Remova sua entrada com `claude mcp remove <name>`, para que não possa ocultar o conector claude.ai na mesma URL
* Depois de removê-la, conecte o serviço em [claude.ai/customize/connectors](https://claude.ai/customize/connectors), enquanto conectado à conta que você usa em Claude Code. Uma vez conectado, [o conector aparece em Claude Code automaticamente](/docs/pt/mcp#use-mcp-servers-from-claude-ai) se seu método de autenticação ativo for um login de assinatura claude.ai

<h3 id="server-rejected-the-authorization-header-minted-by-the-configured-headershelper">
  Servidor rejeitou o cabeçalho Authorization cunhado pelo headersHelper configurado
</h3>

Um servidor MCP cujo [`headersHelper`](/docs/pt/mcp#use-dynamic-headers-for-custom-authentication) fornece o cabeçalho `Authorization` respondeu a conexão com HTTP 401 ou 403, então Claude Code relata a conexão como falha. Porque o helper fornece o cabeçalho `Authorization`, Claude Code [não volta para OAuth](/docs/pt/mcp#authenticate-with-remote-mcp-servers) para o servidor:

```text theme={null}
Server rejected the Authorization header minted by the configured headersHelper (HTTP 401). Check that the helper command returns a valid credential for this MCP endpoint — OAuth fallback is disabled when the helper supplies Authorization.
```

Claude Code re-executa o helper em cada tentativa de conexão, então uma tentativa novamente após uma rejeição transitória, como uma corrida de rotação de token, pode ter sucesso com uma credencial nova.

**O que fazer:**

* Execute o comando `headersHelper` você mesmo da forma que Claude Code o executa: do [diretório onde Claude Code o executa](/docs/pt/mcp#where-the-helper-runs), com as [variáveis de ambiente que Claude Code define para ele](/docs/pt/mcp#use-dynamic-headers-for-custom-authentication), e sem as [variáveis de credencial que Claude Code remove](/docs/pt/mcp#which-variables-a-helper-can-read) para um servidor de um `.mcp.json` de projeto, um plugin ou um arquivo de agente de projeto. Verifique se ele imprime um valor `Authorization` que o endpoint do servidor aceita
* Depois de corrigir o helper ou sua fonte de credencial, selecione o servidor em `/mcp` e escolha **Reconnect**

Antes da v2.1.248, Claude Code executava descoberta OAuth para um servidor cujo helper fornecia o cabeçalho `Authorization`. Essa descoberta poderia falhar com `Incompatible auth server: does not support dynamic client registration` em vez de relatar a credencial rejeitada.

<h3 id="mcp-permission-prompt-tool-not-found">
  Ferramenta de prompt de permissão MCP não encontrada
</h3>

A ferramenta que você passou para [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags) não estava entre as ferramentas MCP conectadas quando a execução primeiro precisou de uma decisão de permissão, seja porque seu servidor nunca se conectou ou porque nenhum servidor conectado expõe uma ferramenta com esse nome. Claude Code ainda envia seu prompt: a execução [não interativa](/docs/pt/headless) sai com esse erro e código de saída 1, na primeira chamada de ferramenta que precisa de aprovação, então não produz resposta mesmo que a solicitação tenha sido feita. Antes do primeiro prompt, Claude Code aguarda até o tempo limite de conexão por servidor de 30 segundos definido por [`MCP_TIMEOUT`](/docs/pt/env-vars) para que esse servidor se conecte. Antes da v2.1.206, a inicialização não aguardava o servidor terminar de se conectar, então um servidor que inicia lentamente mas saudável produzia esse erro também.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

A lista após `Available MCP tools:` nomeia as ferramentas MCP que estavam conectadas quando a espera terminou.

**O que fazer:**

* Verifique se o servidor inicia e permanece conectado: execute `claude mcp list` no mesmo diretório e confirme se o servidor está listado como conectado
* Confirme se o nome da ferramenta corresponde ao nome `mcp__<server>__<tool>` que o servidor expõe
* Se o servidor precisa de mais de 30 segundos para iniciar, aumente [`MCP_TIMEOUT`](/docs/pt/env-vars)

<h3 id="oauth-callback-port-is-already-in-use">
  Porta de callback OAuth já está em uso
</h3>

Quando você faz sign-in em um servidor MCP remoto com OAuth, Claude Code inicia um listener local para receber o callback de sign-in. Se a porta que esse listener precisa está sendo mantida por outro processo, o sign-in falha com essa mensagem. Isso acontece principalmente com uma [porta de callback fixa](/docs/pt/mcp#use-a-fixed-oauth-callback-port) definida através da variável [`MCP_OAUTH_CALLBACK_PORT`](/docs/pt/env-vars) ou `--callback-port`, já que sem uma Claude Code escolhe uma porta disponível.

```text theme={null}
OAuth callback port <port> is already in use — another process may be holding it. Run `lsof -ti:<port> -sTCP:LISTEN` to find it.
```

No Windows, o comando sugerido é `netstat -ano | findstr :<port>` em vez disso.

**O que fazer:**

* Execute o comando da mensagem para encontrar o processo que mantém a porta e pare-o ou aguarde que termine
* Se outro programa precisa dessa porta permanentemente, registre um URI de redirecionamento diferente com o servidor e defina sua porta com `MCP_OAUTH_CALLBACK_PORT` ou `--callback-port`, o que você usar
* Depois inicie o sign-in novamente, por exemplo selecionando o servidor em `/mcp`

<h3 id="no-available-ports-for-oauth-redirect">
  Nenhuma porta disponível para redirecionamento OAuth
</h3>

Quando você faz sign-in em um servidor MCP remoto com [OAuth](/docs/pt/mcp#authenticate-with-remote-mcp-servers), Claude Code inicia um listener local para receber o callback de sign-in. O sign-in falha com essa mensagem quando Claude Code não consegue vincular uma porta local para isso. Algo na máquina está impedindo que ele ouça em `127.0.0.1`, por exemplo software de segurança ou uma política de sandbox que nega listeners locais.

```text theme={null}
No available ports for OAuth redirect
```

Antes da v2.1.268, Claude Code não voltava para uma porta atribuída pelo sistema operacional, então a mensagem também aparecia quando apenas suas portas auto-escolhidas não podiam ser vinculadas. Isso pode acontecer em hosts Windows onde Hyper-V reserva intervalos de porta que cobrem as portas que Claude Code escolhe.

**O que fazer:**

* Verifique se software de segurança ou uma política de sandbox bloqueia processos de ouvirem em `127.0.0.1` e permita que Claude Code vincule uma porta local
* Depois inicie o sign-in novamente, por exemplo selecionando o servidor em `/mcp`

<h3 id="security-review-fails-without-origin-head">
  /security-review falha sem origin/HEAD
</h3>

[`/security-review`](/docs/pt/commands#all-commands) constrói seu contexto de revisão fazendo diff de seu branch contra `origin/HEAD`, a ref local que registra qual branch é o padrão em seu remote `origin`. Quando essa ref não existe, os comandos git que reúnem o diff falham e a revisão para antes de começar.

```text theme={null}
Error: Shell command failed for pattern "!`git diff --name-only origin/HEAD...`": [stderr]
fatal: ambiguous argument 'origin/HEAD...': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
```

A mensagem pode citar `git log` ou um `git diff` diferente em vez disso. Git cria `origin/HEAD` apenas quando o remote anuncia um branch padrão e seu refspec de busca o cobre, o que um `git clone` completo de um remote com commits faz. A ref está faltando nessas configurações:

* Um checkout de branch único ou CI, que busca um refspec muito estreito
* Um remote cujo HEAD do lado do servidor aponta para um branch que ninguém fez push
* Um repositório sem remote `origin`, ou um que você nunca buscou

Claude Code mostra o mesmo erro para qualquer skill que [injeta contexto dinâmico](/docs/pt/skills#when-an-injected-command-fails), e um comando injetado que falha aborta a invocação dessa skill. Duas strings irmãs disparam antes do comando ser executado:

* `Shell command permission check failed for pattern "..."`: a verificação de permissão do comando não o permitiu. [Permission checks on injected commands](/docs/pt/skills#permission-checks-on-injected-commands) cobre quais resultados abortam em cada modo de permissão e como pré-aprovar um comando com `allowed-tools`
* ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``: o frontmatter da skill exige bash em uma máquina sem ele. Instale Git para Windows ou mude o frontmatter para `shell: powershell`. Veja [How injected commands run](/docs/pt/skills#how-injected-commands-run)

**O que fazer:**

* Crie a ref nomeando o branch padrão do seu remote: `git remote set-head origin <default-branch>`. Isso funciona sempre que a ref de rastreamento local `origin/<default-branch>` existe. Se não existir, como em clones de branch único, busque o branch primeiro: execute `git remote set-branches --add origin <branch>`, depois `git fetch origin`, depois execute novamente o comando set-head. Execute `/security-review` novamente.
* Se você preferir não nomear o branch, execute `git fetch origin` e depois `git remote set-head origin --auto`, que pergunta ao remote qual branch é seu padrão. Falha com `error: Cannot determine remote HEAD` quando o remote não anuncia um branch padrão, porque está vazio ou seu HEAD aponta para um branch que ninguém fez push; nomeie o branch explicitamente em vez disso. Falha com `error: Not a valid ref` quando seu clone não busca esse branch; amplie o refspec como acima primeiro.
* Se o repositório não tem remote, adicione um com `git remote add origin <url>` e busque antes de criar a ref. Se o remote está vazio, faça push de seu branch primeiro com `git push -u origin HEAD` e nomeie esse branch no comando set-head; `origin/HEAD` então aponta para o branch que você acabou de fazer push, então `/security-review` vê um diff vazio até o branch divergir dele.

<h3 id="input-must-be-provided-when-using-print">
  Entrada deve ser fornecida ao usar --print
</h3>

`claude` simples precisa que stdout seja um terminal para iniciar a UI interativa. Quando stdout é redirecionado, ou o console não é um terminal real, como PowerShell ISE e alguns painéis de saída de IDE, `claude` executa [não interativamente](/docs/pt/headless) em vez disso. Esse é o mesmo modo que `claude -p`, que requer um prompt, então a mensagem nomeia `--print` mesmo quando você não passou a flag. Passar `-p`/`--print` sem prompt e nada canalizado em stdin produz o mesmo erro em qualquer lugar.

```text theme={null}
Error: Input must be provided either through stdin or as a prompt argument when using --print
```

**O que fazer:**

* Para uso interativo, execute `claude` em um terminal real: Windows Terminal ou o console PowerShell em vez de ISE, e o terminal integrado do seu IDE em vez de um painel de saída
* Para uso único, passe o prompt: `claude -p "your question"`, ou canalize-o com `echo "your question" | claude -p`

<h3 id="input-contained-only-whitespace">
  Entrada continha apenas espaço em branco
</h3>

Em [modo não interativo](/docs/pt/headless), Claude Code recusa um prompt feito inteiramente de espaços, abas ou quebras de linha em vez de enviá-lo, porque a API rejeita mensagens sem texto visível. Qual mensagem você vê depende de onde o prompt em branco veio:

* **Argumento de prompt ou stdin canalizado para `claude -p`**: `claude` sai com `Error: Input contained only whitespace. Provide a prompt with text through stdin or as a prompt argument when using --print`
* **Mensagem enviada para uma sessão `--input-format stream-json` ou [Agent SDK](/docs/pt/agent-sdk/overview) em execução**: Claude Code termina a volta sem chamar o modelo e a sessão permanece utilizável. A recusa chega como uma mensagem informativa e como o texto de resultado da volta: `Blank prompt — the message was only whitespace, so nothing was sent to the model.`

Antes da v2.1.229, Claude Code enviava a mensagem apenas com espaço em branco para a API, que rejeitava a solicitação com um erro 400.

**O que fazer:**

* Inclua texto visível no prompt. Se um script constrói o prompt a partir de uma variável ou arquivo, verifique se a fonte não está vazia antes de chamar Claude Code.

<h3 id="stream-json-input-carried-over-256m-characters-with-no-newline">
  entrada stream-json levou mais de 256M caracteres sem quebra de linha
</h3>

Seu programa enviou mais de 268.435.456 caracteres em stdin sem uma quebra de linha para uma execução `claude -p --input-format stream-json`, então Claude Code imprime esse erro no stderr e sai com código 1 em vez de armazenar mais entrada. A mensagem declara esse orçamento como `256M`. Antes da v2.1.257, Claude Code armazenava tal entrada sem limite, crescendo em memória até o processo travar ou ser morto.

```text theme={null}
Error: stream-json input carried over 256M characters with no newline. Each stream-json message must be a single newline-terminated JSON line: either the producer is not newline-terminating its messages, or one message exceeded this budget.
```

Entrada tão longa sem uma quebra de linha geralmente significa que o produtor não é um produtor stream-json, como um arquivo binário ou saída de log simples canalizada por acidente. Uma única mensagem acima do orçamento falha na mesma verificação.

**O que fazer:**

* Verifique o que está canalizado para stdin. Com [`--input-format stream-json`](/docs/pt/cli-reference#cli-flags), cada mensagem deve ser uma linha JSON terminada por quebra de linha
* Para enviar texto simples em vez disso, remova `--input-format stream-json`; `claude -p` lê um prompt de texto simples de stdin por padrão

<h3 id="unknown-command">
  Comando desconhecido
</h3>

Em uma sessão de terminal interativa, você enviou um nome `/` que não corresponde a nenhum comando nessa sessão, então Claude Code relata o nome em vez de executar qualquer coisa:

```text theme={null}
Unknown command: /hepl. Did you mean /help?
```

Claude Code sugere o nome de comando ou alias mais próximo que o menu lista nessa sessão. Quando nada está próximo, a mensagem termina após o nome. A causa geralmente é uma das seguintes:

* Um erro de digitação, como `/hepl` para `/help`. [How the command menu matches what you type](/docs/pt/commands#how-the-command-menu-matches-what-you-type) cobre escolher uma correspondência próxima antes de enviar
* Um comando que existe mas não está disponível nessa sessão porque um requisito não é atendido, como sua plataforma, plano ou método de autenticação. As entradas de solução de problemas para [`/web-setup`](/docs/pt/web-quickstart#web-setup-shows-no-commands-match-or-unknown-command) e [`/schedule`](/docs/pt/routines#schedule-returns-unknown-command) percorrem dois casos comuns. Alguns comandos respondem com sua própria mensagem quando a política da sua organização os desabilita, como [`Cloud sessions are disabled by your organization's policy`](#cloud-sessions-are-disabled-by-your-organizations-policy)
* Um comando de um [plugin](/docs/pt/plugins/overview) ou [servidor MCP](/docs/pt/mcp#use-mcp-prompts-as-commands) que não está instalado ou conectado nessa sessão

Claude Code responde a um nome `/` não correspondido dessa forma apenas em uma sessão de terminal interativa. Em todas as outras sessões, ele envia o prompt para Claude como uma mensagem normal em vez disso, com uma nota de que o comando não foi executado e uma lista de comandos que Claude pode executar na sessão. Essas sessões incluem:

* execuções `-p`
* aplicações [Agent SDK](/docs/pt/agent-sdk/overview)
* A aba Code do [aplicativo Desktop](/docs/pt/desktop)
* O painel de chat da [extensão VS Code](/docs/pt/vs-code)
* [Sessões em nuvem](/docs/pt/claude-code-on-the-web) e [rotinas](/docs/pt/routines)

Para um comando integrado que não consegue executar em uma dessas sessões, Claude Code ainda responde que o comando não está disponível em vez de enviá-lo para Claude. Antes da v2.1.274, apenas sessões em nuvem e rotinas enviavam um nome não correspondido para Claude. Antes da v2.1.273, elas também respondiam `Unknown command`.

Claude Code não trata cada prompt que começa com `/` como um comando. Ele envia o prompt para Claude como uma mensagem normal quando a primeira palavra após o `/` começa com pontuação, como o `/--` que abre um comentário de doc Lean, ou é um caminho como `/var/log/syslog`.

Antes da v2.1.236, se você pressionasse `Enter` enquanto o menu de comando listava uma correspondência próxima para o nome que você digitou, Claude Code executava essa correspondência, então um erro de digitação como `/hepl` executava `/help` em vez de produzir essa mensagem.

**O que fazer:**

* Execute o nome sugerido, ou digite `/` seguido de parte do nome para ver o que está disponível nessa sessão
* Se Claude Code relata um comando documentado como desconhecido, verifique sua linha na [referência de comandos](/docs/pt/commands) para o requisito que nomeia

<h3 id="diff-is-too-large-for-ultrareview">
  Diff é muito grande para ultrareview
</h3>

O diff entre seu branch e o branch base, incluindo mudanças não confirmadas e preparadas, excede os limites de tamanho para um [ultrareview](/docs/pt/ultrareview), então `/code-review ultra` e o subcomando `claude ultrareview` recusam a revisão antes da sessão em nuvem iniciar. Uma revisão recusada não usa uma execução gratuita e não cobra créditos de uso. A mensagem nomeia os limites em vigor, o tamanho do seu diff e os arquivos que contribuem com a maioria das linhas alteradas. Antes da v2.1.216, a mensagem mostrava apenas as estatísticas de diff bruto.

```text theme={null}
Diff is too large for ultrareview: 812 files, 96,410 lines changed (limits: 500 files, 8,000 lines). Largest files: package-lock.json (41,904 lines), dist/bundle.js (18,210 lines), src/generated/api.ts (9,876 lines). Pass a closer base branch (`/code-review ultra <branch>`) to narrow the scope, or split the change.
```

Revisar um pull request aplica os mesmos limites; essa forma da mensagem começa `PR #<N> is too large for ultrareview` e nomeia as contagens de arquivo e linha do PR.

**O que fazer:**

* Passe um branch base mais próximo do seu trabalho, como `/code-review ultra develop`, para que a revisão cubra apenas o diff contra esse branch
* Divida a mudança em branches menores e revise cada uma. Os arquivos que a mensagem nomeia contribuem com a maioria das linhas alteradas, então comece movendo-os para seu próprio branch.

<h3 id="could-not-find-merge-base-with-the-base-branch">
  Não foi possível encontrar merge-base com o branch base
</h3>

`/code-review ultra` e o subcomando `claude ultrareview` revisam o diff entre seu branch e um branch base, o que precisa de um commit que os dois compartilham. Quando `git merge-base` não encontra nenhum, Claude Code recusa a revisão antes da sessão em nuvem iniciar. Em um clone que Claude Code consegue verificar que é completo, com pelo menos um branch, ele volta para [revisar cada arquivo rastreado](/docs/pt/ultrareview#diff-limits-and-fallbacks) em vez de recusar. Você vê essa recusa quando o branch base não consegue ser encontrado, quando Claude Code não consegue verificar que seu clone é completo, ou no raro repositório onde o diff de árvore inteira não é possível, como o formato de objeto SHA-256.

```text theme={null}
Could not find merge-base with main. Pass the base branch explicitly (e.g. `/code-review ultra develop`) or make sure you're in a git repo with a main branch.
```

A dica após a primeira sentença depende do que Claude Code observou:

* **Você não passou um branch base**: Claude Code comparou contra o branch padrão do repositório e sugere passar seu base explicitamente, como no exemplo acima
* **Você passou um branch base que já estava em seu clone**: a dica lê ``Make sure <branch> exists locally or on origin (try `git fetch origin <branch>`)``
* **Você passou um branch base que não estava em seu clone**: Claude Code o buscou de origin antes de comparar. A dica lê ``<branch> was fetched from origin but shares no history with HEAD. If another branch is your real base, pass it explicitly (`/code-review ultra <branch>`)``; quando Claude Code não consegue dizer se seu clone é raso, sugere `git fetch --unshallow origin` em vez disso. Antes da v2.1.221, a dica sugeria `git fetch --unshallow origin` para cada branch base buscado, e em um clone completo esse comando falha com `fatal: --unshallow on a complete repository does not make sense`.

**O que fazer:**

* Se outro branch é seu base real, passe-o explicitamente: `/code-review ultra <branch>`
* Se seu clone pode não ter histórico completo, execute `git fetch --unshallow origin` e execute a revisão novamente

<h3 id="your-checkout-has-no-branches">
  Seu checkout não tem branches
</h3>

Um checkout pode ter commits mas nenhum branch: se você executar `git init` seguido de `git fetch <url>` e `git checkout FETCH_HEAD`, você obtém um HEAD desanexado sem refs. Claude Code empacota seu repositório como um bundle git para carregá-lo para um [ultrareview](/docs/pt/ultrareview), e não consegue empacotar um repositório que não tem branches ou outras refs, então `/code-review ultra` e o subcomando `claude ultrareview` recusam a revisão antes da sessão em nuvem iniciar.

```text theme={null}
Your checkout has no branches (detached HEAD only), which cloud review can't bundle. Create one first — `git checkout -b <name>` — then rerun /code-review ultra.
```

Antes da v2.1.221, Claude Code tentava revisar cada arquivo rastreado nesse checkout, e o carregamento falhava.

**O que fazer:**

* Crie um branch em seu commit atual com `git checkout -b <name>`, depois execute a revisão novamente

<h3 id="no-github-account-is-connected-to-your-claude-account">
  Nenhuma conta GitHub está conectada à sua conta Claude
</h3>

Você executou `/code-review ultra <PR#>` ou `claude ultrareview <PR#>`, e antes de criar a sessão em nuvem Claude Code pergunta ao servidor se [a conta GitHub conectada à sua conta Claude](/docs/pt/ultrareview#review-a-pull-request) consegue alcançar o repositório do PR. Nenhuma conta está conectada, ou a conexão expirou, então o clone em nuvem falharia e Claude Code recusa o lançamento. Claude Code não gasta uma execução gratuita ou cobra créditos de uso para um lançamento recusado.

```text theme={null}
Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected (or the connection expired). To fix: run /web-setup to reuse your GitHub CLI login, or connect an account at https://claude.ai/connect-github — then re-run /code-review ultra 1234 (allow a minute after connecting).
```

Quando [`/web-setup`](/docs/pt/web-quickstart#connect-from-your-terminal) não está disponível em sua sessão, a mensagem nomeia apenas o link claude.ai.

**O que fazer:**

* Execute `/web-setup` para conectar seu login GitHub CLI à sua conta Claude, ou conecte uma conta em [claude.ai/connect-github](https://claude.ai/connect-github)
* Execute a revisão novamente um minuto após conectar

Antes da v2.1.248, Claude Code não verificava isso antes do lançamento.

<h3 id="your-connected-github-account-cant-see-the-repository">
  Sua conta GitHub conectada não consegue ver o repositório
</h3>

Você executou `/code-review ultra <PR#>` ou `claude ultrareview <PR#>`, e [a conta GitHub conectada à sua conta Claude](/docs/pt/ultrareview#review-a-pull-request) não consegue ler o repositório do PR, então o clone em nuvem falharia e Claude Code recusa o lançamento. Claude Code não gasta uma execução gratuita ou cobra créditos de uso para um lançamento recusado.

```text theme={null}
Your connected GitHub account can't see <owner>/<repo> — usually the Claude GitHub app isn't installed on <owner> or wasn't granted this repo (web-connected accounts need it for private repos), or a different GitHub account is connected. To fix: run /web-setup to reuse your GitHub CLI login, or install the app at https://github.com/apps/claude/installations/new — then re-run /code-review ultra 1234.
```

Quando [`/web-setup`](/docs/pt/web-quickstart#connect-from-your-terminal) não está disponível em sua sessão, a mensagem nomeia apenas a instalação do app.

**O que fazer:**

* Se seu CLI `gh` local consegue ler o repositório, execute `/web-setup` para conectar esse login à sua conta Claude
* Execute a revisão após a mudança

Antes da v2.1.248, Claude Code não verificava isso antes do lançamento.

<h3 id="the-github-app-preflight-failed-transiently">
  A verificação prévia do GitHub App falhou transitoriamente
</h3>

Você iniciou uma [sessão em nuvem](/docs/pt/claude-code-on-the-web) a partir de um repositório local, e duas etapas falharam juntas. Claude Code não conseguiu construir ou carregar o bundle do seu repositório. Antes do carregamento, ele verificou se o serviço em nuvem consegue clonar o repositório do GitHub, e em vez de uma resposta definitiva, essa verificação terminou em um erro que uma tentativa novamente poderia limpar, como um erro de rede, um tempo limite ou um erro de servidor temporário. A mensagem completa começa com o que parou o bundle, por exemplo `Could not upload repo bundle (<error>)`, e termina com a sentença de verificação prévia:

```text theme={null}
Could not upload repo bundle (<error>). The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead
```

**O que fazer:**

* Execute o comando novamente após um momento. Quando a verificação do GitHub passa, Claude Code consegue iniciar a sessão a partir de um clone do GitHub, então o carregamento que falhou não bloqueia mais o lançamento
* Se as tentativas novamente continuarem falhando, o início da mensagem nomeia o que parou o carregamento. Quando essa causa é algo que você consegue corrigir, corrija-a para que a sessão consegua iniciar a partir do seu repositório local em vez disso

Antes da v2.1.251, Claude Code terminava a mensagem com `Please set up GitHub on https://claude.ai/code` mesmo quando a verificação do GitHub falhou apenas transitoriamente, e o conselho de configuração não consegue limpar uma falha transitória.

<h3 id="github-isnt-connected-to-your-claude-account">
  GitHub não está conectado à sua conta Claude
</h3>

Você iniciou uma [sessão em nuvem](/docs/pt/claude-code-on-the-web) a partir do seu repositório local, por exemplo com `/autofix-pr`. Nenhuma conta GitHub está conectada à sua conta Claude, ou a conexão expirou, então Claude Code recusa o lançamento:

```text theme={null}
GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud. Run /web-setup to connect with your GitHub CLI login, or connect on the web at https://claude.ai/connect-github
```

Quando você cria uma rotina com [`/schedule`](/docs/pt/routines), a mesma mensagem aparece como uma nota de configuração que nomeia o repositório; a nota não bloqueia a criação da rotina.

**O que fazer:**

* Execute `/web-setup` para conectar seu login GitHub CLI à sua conta Claude, ou conecte uma conta em [claude.ai/connect-github](https://claude.ai/connect-github). Veja [GitHub authentication options](/docs/pt/claude-code-on-the-web#github-authentication-options) para como os dois diferem.
* Execute o comando novamente um minuto após conectar

Antes da v2.1.268, Claude Code relatava isso como uma falha temporária da verificação do Claude GitHub App e sugeria tentar novamente ou instalar o app; nenhum dos dois conecta uma conta GitHub.

<h3 id="single-sign-on-authorization-needed">
  Autorização de single sign-on necessária
</h3>

Você executou [`/install-github-app`](/docs/pt/github-actions#quick-setup) e escolheu um repositório cuja organização aplica single sign-on SAML. Antes da configuração, Claude Code verifica seu acesso ao repositório com o GitHub CLI, e GitHub recusou essa verificação porque seu token `gh` ainda não está autorizado para a organização. O assistente mostra o aviso com as etapas para autorizar:

```text theme={null}
Single sign-on authorization needed
<owner>/<repo> belongs to an organization that enforces SAML single sign-on, and your GitHub CLI token isn't authorized for it yet.
```

**O que fazer:**

* Re-autorize seu login GitHub CLI com os escopos `repo` e `workflow` executando `gh auth refresh -h github.com -s repo,workflow`, e autorize a organização quando GitHub solicitar single sign-on
* Se você autentica com um token de acesso pessoal em `GH_TOKEN`, abra [github.com/settings/tokens](https://github.com/settings/tokens), selecione **Configure SSO** no token e autorize a organização
* Execute `/install-github-app` novamente

Antes da v2.1.273, Claude Code mostrava o aviso `Admin permissions required` para essa condição em vez disso.

<h3 id="failed-to-resume-the-conversation">
  Falha ao retomar a conversa
</h3>

Claude Code não conseguiu ler ou processar a transcrição salva para a sessão que você selecionou do [seletor `claude --resume`](/docs/pt/sessions#use-the-session-picker), então encerra o processo em vez de continuar em um estado parcialmente carregado. A mensagem inclui o comando para tentar novamente:

```text theme={null}
Failed to resume the conversation.
Run claude --resume <session-id> to retry, or claude to start a new session.
```

Claude Code sai com código 1 após mostrar a mensagem. O seletor `/resume` dentro de uma sessão em execução relata `Failed to resume conversation` na conversa em vez disso, e sua sessão atual continua em execução. Antes da v2.1.216, uma retomada que falhou do seletor `claude --resume` permanecia no spinner `Resuming conversation…` indefinidamente em vez de mostrar essa mensagem.

**O que fazer:**

* Execute `claude --resume <session-id>` com o ID da sessão da mensagem para tentar novamente
* Se cada tentativa novamente falhar da mesma forma, execute `claude update` e retome novamente. Versões antes da v2.1.275 falham a retomada quando a transcrição salva contém uma entrada que não conseguem ler.
* Se a tentativa novamente falhar, execute `claude` para iniciar uma nova sessão

<h3 id="no-conversation-found-with-the-session-id">
  Nenhuma conversa encontrada com o ID da sessão
</h3>

Você passou um ID de sessão para `claude --resume <session-id>` e nenhuma transcrição salva correspondeu:

```text theme={null}
No conversation found with session ID: <session-id>
```

Claude Code sai com código 1 após mostrar a mensagem. Claude Code [busca o projeto atual primeiro, depois cada outro projeto nesta máquina](/docs/pt/sessions#resume-a-session) para o ID. Antes da v2.1.223, a busca parava no diretório do projeto atual e seus git worktrees, então retome do diretório onde a sessão trabalhou pela última vez.

Causas comuns:

* **ID digitado incorretamente**: para uma execução não interativa, o ID é o campo `session_id` da saída [`--output-format json`](/docs/pt/headless#get-structured-output)
* **Transcrição deletada**: Claude Code remove transcrições após o [período de retenção](/docs/pt/sessions#where-transcripts-are-stored), 30 dias por padrão, seguindo as [regras de limpeza de retenção](/docs/pt/claude-directory#cleaned-up-automatically)
* **Máquina diferente**: Claude Code armazena transcrições localmente, então retome a sessão na máquina onde foi executada
* **Cópias duplicadas**: se você copiou um diretório de projeto sob `~/.claude/projects` para que duas transcrições carreguem o mesmo ID, Claude Code relata essa mensagem em vez de retomar uma cópia arbitrariamente

**O que fazer:**

* Para uma sessão interativa, abra o [seletor de sessão](/docs/pt/sessions#use-the-session-picker) com `claude --resume` e pressione `Ctrl+A` para ampliá-lo para cada projeto nesta máquina, depois selecione a sessão
* Sessões criadas com `claude -p` ou o [Agent SDK](/docs/pt/agent-sdk/overview) não aparecem no seletor, então re-verifique o ID contra o `session_id` que sua execução original imprimiu

<h3 id="cannot-switch-renderers-in-this-session">
  Não é possível alternar renderizadores nesta sessão
</h3>

Quando você alterna renderizadores, Claude Code reinicia seu processo. Você executou [`/tui`](/docs/pt/fullscreen#enable-fullscreen-rendering) em uma sessão que Claude Code recusa reiniciar, então não alterna e não salva nada. Qual mensagem você vê diz a você a causa:

* `Cannot switch renderers while work is running in the background`: você tem trabalho em background em execução que uma reinicialização abandonaria, como um shell em background ou um subagente. Aguarde o trabalho terminar ou pare-o com [`/tasks`](/docs/pt/commands), depois execute `/tui fullscreen` ou `/tui default` novamente
* `Cannot switch renderers in this session`: a sessão tem restrições que Claude Code não consegue passar para o processo reiniciado. Antes da v2.1.234, Claude Code reiniciava mesmo assim e a sessão relançada executava sem elas

Na mensagem de restrições, a parte entre parênteses nomeia as restrições que Claude Code encontrou:

```text theme={null}
Cannot switch renderers in this session — it has restrictions a restart can't carry over (permission rules set for this session only). Nothing was changed. Running /tui fullscreen in a session started without them switches every later session too.
```

Cada motivo que a mensagem pode mostrar entre parênteses:

* `launch flags: a custom system prompt, a tool allowlist, or restricted settings`: você iniciou a sessão com uma flag que Claude Code não passa de volta para o processo reiniciado. Essas flags incluem [`--system-prompt`](/docs/pt/cli-reference#cli-flags), `--system-prompt-file`, `--append-system-prompt-file`, uma lista de permissão [`--tools`](/docs/pt/cli-reference#cli-flags), [`--setting-sources`](/docs/pt/cli-reference#cli-flags) e [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags)
* `permission rules set for this session only`: uma [atualização de permissão](/docs/pt/hooks#permission-update-entries) de um hook ou chamador SDK adicionou regras de negação ou pergunta com o destino `session`. Regras de permissão de escopo de sessão não disparam a recusa. Uma reinicialização as remove, e Claude Code solicita novamente em vez disso
* `ask-before-running rules with no command-line form`: uma atualização de permissão de um hook ou chamador SDK adicionou regras de pergunta ao lado das regras que Claude Code passa de volta como `--allowed-tools` e `--disallowed-tools`. Nenhuma flag existe para regras de pergunta
* `permission rules a command line cannot carry intact` e `added directories a command line cannot carry intact`: uma atualização de permissão adicionou uma regra ou caminho de diretório no meio da sessão. A linha de comando do processo reiniciado não consegue carregar seu texto como o mesmo valor

**O que fazer:**

* Em uma sessão iniciada sem essas restrições, execute `/tui fullscreen`, ou `/tui default` para alternar de volta. Claude Code salva a configuração [`tui`](/docs/pt/settings-reference#tui) lá

<h3 id="couldnt-open-claude-desktop">
  Não foi possível abrir Claude Desktop
</h3>

Você executou [`/desktop`](/docs/pt/desktop#coming-from-the-cli), ou seu alias `/app`, e o comando do sistema que Claude Code usa para abrir Claude Desktop falhou. A sessão permanece no terminal.

```text theme={null}
Error: Couldn't open Claude Desktop (`open` exited 1: LSOpenURLsWithRole() failed for the URL claude://resume?session=<session-id> with error -10814). Open Claude Desktop and run /desktop again.
```

**O que fazer:**

* Abra Claude Desktop você mesmo, depois execute `/desktop` novamente
* Para ler a saída de erro completa desse comando, ative o log de debug com `/debug`, execute `/desktop` novamente e verifique o log de debug

Antes da v2.1.275, a mensagem era `Failed to open Claude Desktop. Please try opening it manually.` e não dizia o que falhou.

<h3 id="terminal-setup-left-your-zed-keymap-unchanged">
  /terminal-setup deixou seu mapa de teclas Zed inalterado
</h3>

Você executou [`/terminal-setup`](/docs/pt/terminal-config#enter-multiline-prompts) em Zed, e Claude Code não conseguiu completar a atualização do seu `keymap.json` do Zed, então deixou o arquivo como estava.

Cada mensagem nomeia o caminho para seu mapa de teclas e termina com o bloco de atalho de teclado para adicionar você mesmo:

```text theme={null}
Couldn't update your Zed keymap, so it was left unchanged.
To add the binding yourself, add this block to the keymap array in <path to keymap.json>:
{ "context": "Terminal", "bindings": { "shift-enter": ["terminal::SendText", "\u001b\r"] } }
```

A primeira linha da mensagem nomeia a causa:

* `Couldn't read your Zed keymap, so it was left unchanged.`: Claude Code não conseguiu ler o arquivo, por exemplo por causa de permissões de arquivo
* `Your Zed keymap isn't a readable list of keybindings, so it was left unchanged.`: o arquivo leu bem mas não é analisado como um array de blocos de atalho de teclado, mesmo com comentários `//` e vírgulas finais permitidas
* `Couldn't back up your Zed keymap; not modifying it.`: Claude Code não conseguiu copiar o arquivo para um backup `.bak` ao lado dele, então não mudou nada
* `Couldn't update your Zed keymap, so it was left unchanged.`: o resultado mesclado não verificou como um mapa de teclas válido carregando o atalho de teclado, então Claude Code o descartou em vez de escrever. Um bloco de atalho de teclado com uma chave duplicada pode causar isso

**O que fazer:**

* Copie o bloco da mensagem para o array de nível superior em seu `keymap.json` no caminho que a mensagem nomeia
* Para `isn't a readable list of keybindings`, corrija o erro de sintaxe, ou faça o valor de nível superior do arquivo um array, depois execute `/terminal-setup` novamente

Antes da v2.1.247, `/terminal-setup` não conseguia analisar um mapa de teclas Zed que usava comentários `//` ou vírgulas finais, e substituía o arquivo inteiro por apenas seu próprio atalho de teclado enquanto relatava o atalho de teclado como instalado. Para restaurar um mapa de teclas que uma versão anterior substituiu, use o arquivo de backup `.bak` descrito em [Enter multiline prompts](/docs/pt/terminal-config#enter-multiline-prompts).

<h3 id="skill-usage-reports-are-not-available-on-this-connection">
  Relatórios de uso de skill não estão disponíveis nesta conexão
</h3>

Você executou [`/skill-doctor`](/docs/pt/skills#find-unused-skills) sobre [Remote Control](/docs/pt/remote-control), do seu telefone ou navegador. Claude Code não envia o relatório de uso de skill sobre Remote Control e responde com essa mensagem em vez disso:

```text theme={null}
Skill usage reports are not available on this connection.
```

**O que fazer:**

* Execute `/skill-doctor` no terminal na máquina onde a sessão está em execução, ou execute `claude -p "/skill-doctor"` lá

<h3 id="custom-output-styles-cant-be-selected-over-remote-control">
  Estilos de saída personalizados não podem ser selecionados sobre Remote Control
</h3>

Você executou [`/output-style`](/docs/pt/output-styles#change-your-output-style) do aplicativo móvel ou web via [Remote Control](/docs/pt/remote-control), ou o comando chegou em uma mensagem retransmitida para a sessão. Porque tal volta pode não vir do proprietário da conta, Claude Code lista e seleciona apenas [estilos integrados](/docs/pt/output-styles#built-in-output-styles) nela, e adiciona esse aviso sempre que o comando lista os estilos ou não reconhece o nome que você deu. Um nome de [estilo personalizado](/docs/pt/output-styles#create-a-custom-output-style) obtém a mesma resposta que um nome que não existe:

```text theme={null}
Custom output styles can't be selected over Remote Control or from a relayed message. Select one in the session itself, or pick a built-in style here.
```

**O que fazer:**

* Escolha um estilo integrado, por exemplo `/output-style concise`
* Para usar um estilo personalizado, defina [`outputStyle`](/docs/pt/settings-reference#outputstyle) em `.claude/settings.local.json` do projeto, ou execute `/output-style <style>` no terminal da própria sessão se tiver um

<h3 id="output-styles-are-saved-to-local-settings-which-this-session-doesnt-load">
  Estilos de saída são salvos em configurações locais que esta sessão não carrega
</h3>

Você tentou alternar [estilos de saída](/docs/pt/output-styles) com `/output-style <style>` ou `/config outputStyle=<style>` em uma sessão cujas fontes de configuração excluem `local`. Exemplos são uma sessão [Agent SDK](/docs/pt/agent-sdk/typescript) cujo [`settingSources`](/docs/pt/agent-sdk/typescript#options) deixa de fora `"local"` e uma sessão CLI iniciada com um valor [`--setting-sources`](/docs/pt/cli-reference#cli-flags) que deixa de fora `local`. Ambos os comandos salvam o estilo em `.claude/settings.local.json`, um arquivo que tal sessão nunca lê de volta, então Claude Code recusa em vez de escrever uma configuração que não teria efeito:

```text theme={null}
Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load, so the style can't be changed here.
```

**O que fazer:**

* Adicione `local` às fontes de configuração da sessão e alterne novamente
* Defina a chave [`outputStyle`](/docs/pt/settings-reference#outputstyle) em um arquivo de configurações que a sessão carrega, como `.claude/settings.json` no projeto ou `~/.claude/settings.json`. No SDK TypeScript, defina `outputStyle` dentro do objeto `settings` inline em vez disso; veja [Activate an output style](/docs/pt/agent-sdk/modifying-system-prompts#activate-an-output-style)

<h2 id="plugin-errors">
  Erros de plugin
</h2>

Esses erros vêm da configuração de [plugin](/docs/pt/plugins/overview) e [marketplace](/docs/pt/plugins/overview). Para problemas de plugin que não produzem uma das mensagens nesta página, como uma URL de marketplace que não carrega ou um plugin que é instalado mas não aparece, consulte [Solução de problemas de plugin](/docs/pt/plugins/troubleshooting).

<h3 id="plugin-eval-is-currently-in-early-access">
  plugin eval is currently in early access
</h3>

Você executou [`claude plugin eval`](/docs/pt/plugin-evals) ou `claude plugin eval init` e ele saiu com código 1 com uma dessas mensagens antes de fazer qualquer coisa:

```text theme={null}
`plugin eval` is currently in early access
```

```text theme={null}
`plugin eval` is currently unavailable
```

A primeira mensagem significa que sua compilação é mais antiga que v2.1.269, a primeira versão em que o comando está geralmente disponível. A segunda significa que a Anthropic desativou o comando no servidor; nada em sua máquina o ativa novamente.

**O que fazer:**

* Execute `claude --version`, depois `claude update`, e execute o comando novamente em uma nova sessão. Consulte os [requisitos para plugin evals](/docs/pt/plugin-evals#requirements)
* Se você vir a segunda mensagem em uma compilação atual, tente novamente mais tarde após outro `claude update`

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace is registered from an untrusted source
</h3>

O marketplace está registrado sob um nome que é [reservado para marketplaces oficiais da Anthropic](/docs/pt/plugins/marketplace-reference#marketplace-file), mas sua fonte registrada não é um repositório GitHub `anthropics`. Claude Code verifica novamente os nomes reservados toda vez que carrega ou atualiza um marketplace, então o marketplace e os plugins instalados a partir dele param de carregar. Antes da v2.1.205, o nome era verificado apenas quando o marketplace era adicionado, então uma entrada registrada antes de seu nome ficar reservado continuava carregando.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

Para um marketplace cuja fonte não é um repositório GitHub ou uma URL Git, como um diretório local, a frase do meio lê `can only be used with GitHub sources from the 'anthropics' organization` em vez disso. `claude plugin marketplace add` executa a mesma verificação e recusa um nome reservado com `Failed to add marketplace:` seguido pela mesma frase de nome reservado.

**O que fazer:**

* Se o marketplace já está registrado, execute `claude plugin marketplace remove <name>`, depois adicione-o novamente do repositório oficial `github.com/anthropics`
* Se você publicar um marketplace de terceiros que usou o nome antes de ficar reservado, renomeie-o e peça aos usuários para adicioná-lo novamente de sua fonte
* Consulte a lista de nomes reservados em [Marketplace schema](/docs/pt/plugins/marketplace-reference#marketplace-file)

<h3 id="marketplace-name-is-another-spelling-of-a-reserved-name">
  Marketplace name is another spelling of a reserved name
</h3>

O nome do marketplace não é em si um nome reservado, mas Claude Code o trata como outra grafia de um. [Nomes reservados](/docs/pt/plugins/marketplace-reference#reserved-name-spellings) lista quais grafias contam como um nome reservado. Claude Code recusa tal nome quando você adiciona o marketplace:

```text theme={null}
Failed to add marketplace: "claude.code.plugins" is another spelling of "claude-code-plugins", a reserved marketplace name.
```

Quando um marketplace já está registrado sob tal nome, sua entrada para de carregar, e `/plugin`, `claude plugin install`, e `claude plugin update` avisam:

```text wrap theme={null}
known_marketplaces.json has an entry named "claude.code.plugins", another spelling of the reserved marketplace name "claude-code-plugins", so it is ignored. Remove it with: claude plugin marketplace remove claude.code.plugins
```

Quando o nome precisaria de aspas de shell, a recusa no momento da adição lê `This marketplace's name is another spelling of "<reserved>", a reserved marketplace name. It is not exactly the reserved name it appears to be.`

**O que fazer:**

* Renomeie o marketplace para um nome que não soletra um nome reservado e adicione-o novamente
* Para o aviso de entrada ignorada, execute o comando `claude plugin marketplace remove` que ele fornece, ou remova a entrada de `~/.claude/plugins/known_marketplaces.json`

<h3 id="marketplace-is-already-added-from-a-different-source">
  Marketplace is already added from a different source
</h3>

Você confirmou adicionar um marketplace através de [`/plugin install <plugin> --marketplace <source>`](/docs/pt/plugins/install#add-a-marketplace-and-install-in-one-command), e o catálogo que Claude Code buscou dessa fonte nomeia a si mesmo igual a um marketplace que você já adicionou de uma fonte diferente. Claude Code mantém o marketplace existente em vez de substituí-lo, e o plugin não é instalado.

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

**O que fazer:**

* Se o marketplace que você já adicionou é o que você quer, instale a partir dele pelo nome: `/plugin install <plugin>@<name>`
* Para mudar para a nova fonte, execute `/plugin marketplace remove <name>`, depois tente a instalação novamente

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

Um hook de plugin, [monitor](/docs/pt/plugins/components#monitors), ou comando MCP [`headersHelper`](/docs/pt/mcp#use-dynamic-headers-for-custom-authentication) referencia uma opção de plugin `${user_config.KEY}` [plugin option](/docs/pt/plugins/manifest-reference#user-configuration), e a string substituída seria passada para um shell. Um valor configurado contendo `$(...)`, backticks, ou `;` seria executado como código lá, então Claude Code recusa iniciar o componente em vez de substituir o valor. A verificação é executada no modelo de comando, então o erro aparece mesmo quando nenhum valor está configurado ainda. Antes da v2.1.207, o valor era substituído no comando shell.

A redação depende de qual superfície referenciou a opção. Um hook de forma shell relata:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Um monitor relata:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

Um MCP `headersHelper` relata:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**O que fazer:**

* Para um hook, adicione um array `args` para que ele seja executado em [exec form](/docs/pt/hooks#exec-form-and-shell-form), onde cada `${user_config.KEY}` se torna um argumento sem shell no meio. Ou remova a referência e leia a variável de ambiente `$CLAUDE_PLUGIN_OPTION_<KEY>` dentro do script
* Para um monitor, remova a referência e faça o script do monitor ler o valor de um arquivo de configuração
* Para um `headersHelper`, mova `${user_config.KEY}` para o campo `headers` do servidor, que não é analisado por shell, ou leia o valor dentro do script helper

<h3 id="plugin-archive-integrity-check-failed">
  Plugin archive integrity check failed
</h3>

A entrada do marketplace do plugin usa uma [fonte `archive`](/docs/pt/plugins/marketplace-reference#archive-plugin-source) com um pin `sha256`, e o digest do arquivo baixado não corresponde ao pin. Claude Code recusa a instalação, então nada muda no cache de plugin. A incompatibilidade tem três possíveis causas:

* O arquivo na URL mudou depois que o autor computou o pin
* O autor inseriu o digest errado na entrada do marketplace
* A URL serve um arquivo diferente do que o autor fixou

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

**O que fazer:**

* Se você publica o plugin, recompute o digest do arquivo exato que a URL serve, por exemplo com `shasum -a 256 my-plugin.zip`, ou `Get-FileHash -Algorithm SHA256 my-plugin.zip` no PowerShell, e atualize o `sha256` na entrada do marketplace
* Se você instala o plugin, execute `/plugin marketplace update <name>` para atualizar o catálogo caso a entrada tenha sido corrigida, depois tente a instalação novamente
* Se os digests ainda discordarem após uma atualização, pergunte ao proprietário do marketplace qual arquivo eles fixaram antes de instalar

<h3 id="path-escapes-plugin-directory">
  Path escapes plugin directory
</h3>

Um caminho de componente de plugin, declarado no `plugin.json` do plugin ou em sua [entrada de marketplace](/docs/pt/plugins/marketplace-reference#plugin-entries), resolve fora do diretório do próprio plugin. Claude Code descarta esse caminho e carrega o resto do plugin. O nome do componente na mensagem, como `commands` ou `hooks`, nomeia o campo que declarou o caminho.

```text theme={null}
commands path escapes plugin directory: ./../shared.md
```

Na saída do comando `claude plugin`, o mesmo erro lê `Path escapes plugin directory: ./../shared.md (commands)`.

Claude Code rejeita tanto um caminho que aponta para fora do plugin conforme escrito, como `../shared-utils`, e um symlink que leva para fora do plugin e não é um que as [regras de symlink do marketplace](/docs/pt/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks) permitem. Para um symlink, a mensagem também diz para onde o caminho resolve:

```text theme={null}
commands path escapes plugin directory: ./commands/deploy.md — it resolves to /home/user/shared/deploy.md, outside the plugin directory
```

No macOS e Linux, Claude Code também rejeita um caminho de componente que contém uma barra invertida em qualquer lugar, mesmo quando o caminho fica dentro do plugin. Um plugin cujos caminhos de componente usam separadores de estilo Windows carrega no Windows e dispara essa rejeição nas outras plataformas:

```text theme={null}
commands path escapes plugin directory: ./commands\deploy.md — its path contains a backslash, which is not resolved reliably on this platform
```

Antes da v2.1.251, Claude Code carregava um caminho `commands` declarado em uma entrada de marketplace mesmo quando apontava para fora do diretório do plugin. Claude Code já rejeitava caminhos declarados em `plugin.json` e os outros caminhos de componente em uma entrada de marketplace.

Antes da v2.1.257, a verificação olhava apenas para a grafia do caminho, não para onde um symlink leva.

**O que fazer:**

* Mova o arquivo referenciado dentro do diretório do plugin e aponte o caminho para ele com um caminho relativo `./`
* Se o caminho é um symlink para um arquivo fora do plugin, substitua o symlink por uma cópia do arquivo
* Se a mensagem diz que o caminho contém uma barra invertida, escreva o caminho com barras para frente, por exemplo `./commands/deploy.md`
* Para compartilhar arquivos com outros plugins no mesmo marketplace, vincule-os com um symlink dentro do diretório do plugin, seguindo as [regras de symlink](/docs/pt/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)

<h3 id="path-could-not-be-checked">
  Path could not be checked
</h3>

Claude Code perguntou ao sistema operacional se um caminho de plugin existe e recebeu um erro diferente de "não encontrado", então ele não carrega o que o caminho nomeia. Quanto do plugin carrega depende de qual caminho falhou:

* Um dos [locais de componente padrão](/docs/pt/plugins/manifest-reference#standard-layout) de um plugin, como a pasta `skills/`, o arquivo `monitors/monitors.json`, ou um [`SKILL.md` na raiz do plugin](/docs/pt/plugins/components#skills): os outros componentes do plugin ainda carregam
* O próprio diretório do plugin: nada desse plugin carrega

Você não vê esse erro para um caminho que não existe em absoluto. Em `/plugin`, o erro aparece sob o plugin e nomeia o caminho e o código que o sistema operacional retornou:

```text theme={null}
skills path could not be checked: /home/user/my-plugin/skills (ELOOP)
```

Em `claude plugin list`, o mesmo erro lê `Path not found: /home/user/my-plugin/skills (skills, ELOOP)`.

As causas que produzem esse erro incluem:

* `ELOOP`: um symlink no caminho aponta para si mesmo ou forma um loop
* `EIO` ou `ESTALE`: o caminho está em uma montagem de rede que está quebrada ou obsoleta
* `EACCES`: um dos diretórios acima do caminho nega a você permissão para atravessá-lo

**O que fazer:**

* Substitua um symlink que aponta para si mesmo por uma pasta real, ou delete-o
* Se o caminho está em uma montagem de rede, remonte o compartilhamento
* Se o código é `EACCES`, restaure sua permissão de execução nos diretórios acima do caminho
* Execute `/reload-plugins` após corrigir o caminho, ou reinicie Claude Code, para carregar o plugin ou componente

Antes da v2.1.265, Claude Code tratava uma pasta de componente padrão que não conseguia verificar como ausente e carregava o plugin sem esse componente, sem erro.

<h3 id="marketplace-entry-path-does-not-stay-inside-the-marketplace-directory">
  Marketplace entry path does not stay inside the marketplace directory
</h3>

A [entrada de marketplace](/docs/pt/plugins/marketplace-reference#plugin-entries) do plugin declara um caminho de origem que Claude Code não consegue resolver para um local dentro do próprio diretório do marketplace, então o plugin não instala ou carrega. A recusa cobre:

* Um caminho de entrada que é absoluto, sobe para fora do marketplace com `..`, ou é soletrado como um caminho de rede
* No macOS e Linux, um caminho de entrada que contém uma barra invertida em qualquer lugar após o `./` inicial
* Uma entrada em um marketplace buscado de uma fonte remota, como git ou uma URL, que alcança seu alvo através de um symlink resolvendo fora do diretório do marketplace
* Uma entrada relativa em um marketplace adicionado de uma URL direta para seu `marketplace.json`: Claude Code baixa apenas esse arquivo, então nenhum arquivo de plugin local existe para o caminho nomear. Consulte [Plugins with relative paths fail in URL-based marketplaces](/docs/pt/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)

`claude plugin install` relata a recusa assim:

```text theme={null}
Cannot install my-plugin@my-marketplace: its marketplace entry path does not stay inside the marketplace directory (an absolute, climbing, network-shaped, backslash-containing or link-traversing entry, an entry of a fetched marketplace that resolves or opens outside its tree — or a relative entry in a url-catalog marketplace, which has no local directory)
```

Quando uma entrada de plugin já instalado falha na mesma verificação, `claude plugin list` mostra o plugin como `failed to load` com:

```text theme={null}
Plugin source path refused: ./my-plugin does not stay inside its marketplace directory. Check that the marketplace entry has a plain relative path.
```

**O que fazer:**

* Se você mantém o marketplace, escreva o `source` da entrada como um caminho relativo simples com barras para frente, como `./plugins/my-plugin`, e mantenha qualquer symlink que ele cruze apontado dentro do diretório do marketplace
* Se você adicionou o marketplace de uma URL direta, entradas relativas não conseguem resolver. Peça ao autor do marketplace para usar [outra fonte de plugin](/docs/pt/plugins/marketplace-reference#plugin-sources), ou adicione o marketplace de seu repositório git em vez disso

<h3 id="failed-to-load-marketplace-configuration">
  Failed to load marketplace configuration
</h3>

Claude Code mantém os marketplaces de plugin que você adicionou em um arquivo de registro em `~/.claude/plugins/known_marketplaces.json`. Um comando de plugin que precisa do registro, como `claude plugin install`, falha com uma de duas mensagens quando Claude Code não consegue usar o arquivo:

* `Failed to load marketplace configuration`: o arquivo não é JSON válido, ou não pode ser lido. Um arquivo vazio falha dessa forma também.
* `Marketplace configuration file is corrupted`: o arquivo é JSON válido mas seu conteúdo não corresponde ao esquema do registro.

Um arquivo ausente não é uma falha: Claude Code o trata como um registro sem marketplaces.

Com um arquivo vazio, `claude plugin install` relata:

```text theme={null}
✘ Failed to install plugin "my-plugin": Failed to load marketplace configuration: JSON Parse error: Unexpected EOF
```

Antes da v2.1.246, `claude plugin install` não relatava essa falha.

**O que fazer:**

* Abra `~/.claude/plugins/known_marketplaces.json` e repare o JSON, ou corrija as entradas que a mensagem nomeia como não correspondendo ao esquema do registro
* Se você não conseguir repará-lo, delete o arquivo ou substitua seu conteúdo por `{}`, depois adicione novamente cada marketplace com `claude plugin marketplace add <source>`. Claude Code re-registra os marketplaces que suas configurações de usuário ou gerenciadas declaram em [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces) na próxima vez que você o inicia em uma pasta que você confiou.

<h3 id="plugin-is-required-by-your-organization">
  Plugin is required by your organization
</h3>

Você executou `claude plugin disable`, ou usou a aba **Installed** do `/plugin`, para desativar um [plugin sincronizado de claude.ai](/docs/pt/plugins/loading#synced-plugins) que sua organização marca como obrigatório:

```text theme={null}
Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.
```

Claude Code não salva nada e o plugin permanece ativado.

Quando você tenta desativar um plugin que um plugin obrigatório depende, Claude Code recusa da mesma forma, com uma mensagem nomeando o plugin obrigatório que precisa dele.

**O que fazer:**

* Peça a um administrador de sua organização claude.ai para mudar o status obrigatório do plugin em claude.ai

<h2 id="tool-errors">
  Erros de ferramentas
</h2>

Esses erros vêm das ferramentas integradas do Claude. Claude corrige a maioria dos erros de ferramentas por conta própria. Quando um deles precisa de uma mudança sua, a lista **O que fazer** desse erro diz o que mudar.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent seria iniciado com zero ferramentas
</h3>

Cada entrada na lista [`tools` do subagent](/docs/pt/sub-agents#supported-frontmatter-fields) falhou em corresponder a uma ferramenta utilizável, então Claude Code recusou iniciar o subagent: sem ferramentas, ele não poderia agir. A mensagem agrupa suas entradas pelo que deu errado:

* **Unrecognized**: a entrada não corresponde a nenhum nome de ferramenta, geralmente um erro de digitação como `Grpe` para `Grep`.
* **Not available to subagents**: a entrada nomeia uma ferramenta real que [subagents não podem usar](/docs/pt/sub-agents#available-tools). Subagents em background mantêm um conjunto de ferramentas integradas menor, então uma entrada que apenas um subagent em foreground pode usar fica aqui quando o subagent seria executado em background, que é o padrão. Se você listar `Agent`, a mensagem o relata no próximo grupo.
* **Matched no tools in this session**: a entrada é válida mas nenhuma ferramenta na sessão atual corresponde a ela agora, como `mcp__github__*` sem servidor GitHub MCP conectado, ou `Agent` para um subagent no [limite de profundidade](/docs/pt/sub-agents#let-subagents-spawn-their-own-subagents).

Omitir o campo `tools` nunca dispara essa recusa. Se você deixar a lista `tools` vazia, ou `disallowedTools` remover cada entrada nela, Claude Code também pula a recusa e inicia o subagent sem ferramentas.

Antes da v2.1.208, o subagent era iniciado sem ferramentas e poderia retornar um resultado vazio ou confuso.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**O que fazer:**

* Corrija cada entrada que o erro nomeia contra as [ferramentas disponíveis para subagents](/docs/pt/sub-agents#available-tools)
* Remova entradas para ferramentas que a sessão não tem, como ferramentas MCP de um servidor que não está conectado
* Para uma ferramenta que [subagents em background descartam](/docs/pt/sub-agents#available-tools), como `CronCreate`, remova a entrada. Para manter a ferramenta, [desative o fork mode](/docs/pt/sub-agents#turn-fork-mode-on-or-off) e peça ao Claude para executar o subagent em foreground
* Delete o campo `tools` em vez de listar ferramentas para dar ao subagent cada [ferramenta disponível para subagents](/docs/pt/sub-agents#available-tools)
* Para uma lista `tools` que contém apenas `Agent`, aumente o [limite de profundidade](/docs/pt/sub-agents#let-subagents-spawn-their-own-subagents) ou dê ao agente pelo menos uma outra ferramenta: Claude Code retém `Agent` nesse limite, então uma lista com nada mais nela se resolve para nenhuma ferramenta

<h3 id="file-is-covered-by-a-read-deny-rule">
  Arquivo é coberto por uma regra de negação Read
</h3>

A ferramenta Edit ou Write foi chamada em um caminho correspondido por uma [regra de negação `Read`](/docs/pt/permissions#read-and-edit), incluindo criar um novo arquivo nesse caminho. Ambas as ferramentas mudam conteúdo que Claude tem que ser capaz de ler de volta, então Claude Code recusa a chamada antes de qualquer acesso a arquivo. NotebookEdit não é coberto por regras de negação `Read`. Antes da v2.1.228, a regra bloqueava apenas a ferramenta Edit, e antes da v2.1.208, apenas uma regra de negação `Edit` bloqueava edições.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

Quando Claude Code recusa a ferramenta Write, a mensagem termina `and cannot be written` em vez disso.

**O que fazer:**

* Se Claude deveria ser capaz de mudar o arquivo, remova ou estreite a regra de negação `Read` em `/permissions` ou em [settings](/docs/pt/settings-reference#permission-settings)
* Se o arquivo deve permanecer intocado, mantenha a regra e adicione uma regra de negação `Edit` para o mesmo caminho para bloquear a ferramenta NotebookEdit também

<h3 id="subagent-type-is-required">
  subagent\_type é obrigatório
</h3>

```text theme={null}
subagent_type is required: the general-purpose agent is not available in this session. Available agents: ...
```

Claude chamou a [ferramenta Agent](/docs/pt/tools-reference#agent-tool-behavior) sem um `subagent_type`, e essa sessão não tem [subagent de propósito geral](/docs/pt/sub-agents#built-in-subagents) para recorrer. Esse é o caso em duas configurações:

* [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/pt/env-vars) está definido em modo não-interativo, que remove cada subagent integrado
* O agente de thread principal da sessão tem uma [lista de permissão `tools: Agent(...)`](/docs/pt/sub-agents#restrict-which-subagents-can-be-spawned) que deixa de fora `general-purpose`

**O que fazer:**

* Geralmente nada: a mensagem lista os subagents que a sessão tem, então Claude pode tentar novamente com um deles
* Se Claude continuar falhando, adicione `general-purpose` à lista de permissão `tools: Agent(...)`, ou desdefina `CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS`

Antes da v2.1.235, a mesma chamada falhava com `Agent type 'general-purpose' not found`.

<h3 id="memory-index-is-over-its-read-limit">
  Índice de memória está acima de seu limite de leitura
</h3>

Claude escreveu no índice de [memória automática](/docs/pt/memory#auto-memory) `MEMORY.md` e o deixou acima de um de seus limites de leitura: 200 linhas ou 25KB. A escrita foi bem-sucedida, mas apenas as primeiras 200 linhas ou 25KB, o que vier primeiro, carregam no início de uma sessão, então tudo além do limite é descartado cada vez que o índice é lido. Antes da v2.1.210, um índice acima do limite era silenciosamente truncado no próximo carregamento sem sinal de tempo de escrita.

```text theme={null}
Error: this write left the memory index at MEMORY.md at 214 lines, over its 200-line read limit. The write succeeded, but everything past the limit is silently dropped each time the index is loaded — entries at the end are already invisible to readers. Rewrite it to under 140 lines now: keep one line per entry, move detail into topic files, and merge or drop stale entries.
```

Apenas o conteúdo que carrega conta para os limites. Frontmatter YAML e comentários HTML em nível de bloco são removidos antes do índice ser carregado, então são excluídos da medição. Antes da v2.1.211, Claude Code media o arquivo bruto, e frontmatter ou comentários poderiam disparar esse erro mesmo quando o conteúdo carregado se encaixasse.

Claude Code entrega o erro ao Claude após a escrita em vez de imprimi-lo como um banner em seu terminal, então você pode notar apenas na transcrição.

Quando a escrita do Claude traz o arquivo perto de um limite sem cruzá-lo, Claude Code retorna um lembrete mais suave para compactar o índice em vez desse erro.

**O que fazer:**

* Deixe Claude reescrever `MEMORY.md`, ou peça a ele: mantenha uma linha por entrada, mova detalhes para arquivos de tópico, e mescle ou descarte entradas obsoletas
* Para aparar o índice você mesmo, veja [Audit and edit your memory](/docs/pt/memory#audit-and-edit-your-memory)

<h3 id="pkill-pattern-matches-the-claude-code-process">
  Padrão pkill corresponde ao processo Claude Code
</h3>

Um comando `pkill` em uma chamada de ferramenta Bash usou um padrão, tipicamente com `-f`, que corresponde ao próprio processo Claude Code, então Claude Code recusa o comando em vez de deixá-lo encerrar a sessão. Claude Code testa o padrão com `pgrep` antes de executar `pkill` e recusa quando seu próprio ID de processo está no resultado. A verificação é executada apenas em Linux; em macOS, `pkill` é executado sem modificações. Antes da v2.1.214, o comando era executado, e um padrão correspondente matava a sessão Claude Code no meio de um turno.

```text theme={null}
pkill: refusing to run — this pattern matches the Claude CLI process (PID 12345). Narrow the pattern, or target your own children with `pkill -P $$ ...`.
```

A recusa aparece no resultado da ferramenta Bash em vez de como um banner em seu terminal, e Claude geralmente ajusta o comando por conta própria.

**O que fazer:**

* Estreite o padrão para que corresponda apenas ao processo pretendido, por exemplo o caminho completo do binário alvo em vez de uma substring curta
* Para parar processos iniciados pelo shell atual, use `pkill -P $$` com o padrão, que limita a correspondência aos processos filhos do próprio shell

<h3 id="failed-to-write-to-a-teammate-inbox">
  Falha ao escrever na caixa de entrada de um colega
</h3>

Claude Code não conseguiu escrever uma mensagem no arquivo de caixa de correio de um colega sob `~/.claude/teams/{team-name}/inboxes/`, então o destinatário não recebeu nada. A escrita falha quando Claude Code não consegue criar ou atualizar o arquivo, por exemplo porque o disco está cheio, o diretório não é gravável, ou outro agente mantém o bloqueio da caixa de entrada por muito tempo. Antes da v2.1.224, Claude Code relatava a mensagem como enviada mesmo quando a escrita falhava.

O erro aparece no resultado da ferramenta do agente remetente em vez de como um banner em seu terminal, e seu texto diz ao Claude para tentar novamente:

```text theme={null}
Failed to write to researcher's inbox — nothing was sent. Try again, or message the lead.
```

Mensagens de protocolo estruturadas de [equipe de agentes](/docs/pt/agent-teams) falham da mesma forma, e o erro nomeia a mensagem não entregue: quando Claude Code não consegue escrever uma aprovação de plano, rejeição de plano, solicitação de encerramento, ou rejeição de encerramento, o erro lê `Failed to write the <message> to <name>'s inbox — nothing was sent`. O `plan approval` nessa lista é a decisão do líder aprovando o plano de um colega; a submissão de plano do colega é a mensagem separada `plan approval request`. Essa mensagem e duas outras mensagens de protocolo carregam seu próprio texto de mensagem e consequência:

* `Failed to write the plan approval request to the lead's inbox — plan not submitted; try again`: o plano do colega nunca chegou ao líder, e o colega permanece em modo de plano até uma resubmissão bem-sucedida
* `The permission request could not be delivered to the team lead (mailbox write failed)`: a solicitação de permissão do colega nunca chegou ao líder, então ninguém aprovou a chamada de ferramenta
* `The confirmation could not be written to team-lead's inbox.`: a aprovação de encerramento em si entrou em vigor e o colega sai; apenas a confirmação para o líder está faltando

Quando você messagem um colega você mesmo, digitando `@name` seguido pela mensagem na sessão de liderança, a mesma falha aparece como uma notificação, `Couldn't write to @name's inbox — message not sent. Try again.`, e Claude Code mantém seu texto na caixa de prompt para que você possa enviá-lo novamente.

**O que fazer:**

* Peça ao remetente para reenviar a mensagem; contenção para o bloqueio da caixa de entrada é transitória e se limpa na tentativa novamente
* Verifique espaço em disco livre, e verifique que `~/.claude/teams` e os arquivos sob ele são graváveis pelo seu usuário

<h3 id="teammate-agent-definition-not-restored">
  Definição de agente do colega não foi restaurada
</h3>

Claude messageou um colega de [equipe de agentes](/docs/pt/agent-teams) parado, e Claude Code o trouxe de volta sem reaplicar a [definição de subagent](/docs/pt/agent-teams#use-subagent-definitions-for-teammates) de que foi gerado, porque seu arquivo de definição veio de uma pasta sem confiança salva. O aviso segue o relatório de retomada no resultado da ferramenta do agente remetente:

```text wrap theme={null}
Its agent definition was not restored: the folder its definition file came from is not trusted (source: projectSettings), so the teammate is running with the team-essential tools and no custom instructions. To restore it, the user needs to run Claude Code in that folder once and accept the trust dialog (the --debug log names the folder); do not change trust settings on the user's behalf.
```

A verificação se aplica a uma definição no diretório `.claude/agents/` do projeto ou de um diretório `--add-dir`, e aceitar o diálogo de confiança para uma pasta pai não a satisfaz.

**O que fazer:**

* Execute `claude` na pasta que o [log de debug](/docs/pt/debug-your-config) nomeia e aceite o diálogo de confiança. A definição é reaplicada na próxima vez que Claude Code traz o colega de volta; você não precisa reiniciar a sessão de liderança
* Ou defina a entrada `hasTrustDialogAccepted` para `true` em `~/.claude.json`, usando a chave exata `projects["<path>"]` que o log de debug imprime

<h3 id="message-too-large-for-cross-session-delivery">
  Mensagem muito grande para entrega entre sessões
</h3>

A [mensagem entre sessões](/docs/pt/cross-session-messaging) do Claude para outra de suas sessões nesta máquina era muito longa para enviar. Claude Code a recusou, e a sessão receptora não recebeu nada. A recusa aparece no resultado da ferramenta da sessão remetente, não como um banner em seu terminal. Ela nomeia ambos os tamanhos e como fazer a mensagem caber:

```text wrap theme={null}
Failed to send to api-worker: Message too large for cross-session delivery: the serialized message is 1,203,844 characters and the limit is 1,048,576. Shorten the message text — put bulk content in a file the recipient can read rather than in the message — or split it into smaller messages.
```

Reenviar o mesmo texto falha da mesma forma.

**O que fazer:**

* Peça ao Claude para resumir a mensagem, ou para colocar o conteúdo em massa em um arquivo e enviar o caminho do arquivo
* Peça ao Claude para dividir o conteúdo em várias mensagens mais curtas

Antes da v2.1.235, Claude Code relatava uma mensagem superdimensionada como enviada. A sessão receptora a descartava sem ler.

<h3 id="too-many-messages-to-this-session-just-now">
  Muitas mensagens para essa sessão agora
</h3>

Claude enviou uma rajada rápida de [mensagens entre sessões](/docs/pt/cross-session-messaging) para uma de suas sessões nesta máquina, e a rajada atingiu o que essa sessão aceita. Claude Code recusou o próximo envio, e a sessão receptora não recebeu nada dele. A recusa aparece no resultado da ferramenta da sessão remetente, não como um banner em seu terminal:

```text wrap theme={null}
Failed to send to api-worker: Too many messages to this session just now: 30 were sent recently and more would be dropped by its rate limit, so this one was not sent. Batch what remains into one message, or wait a little before sending more.
```

**O que fazer:**

* Geralmente nada: Claude agrupa o conteúdo restante em uma mensagem, ou espera antes de enviar mais
* Se você mesmo disparou a rajada, peça ao Claude para combinar o que resta em uma única mensagem

Antes da v2.1.236, Claude Code relatava esses envios como enviados. A sessão receptora os descartava sem ler.

<h3 id="refusing-to-send-a-cross-session-message">
  Recusando enviar uma mensagem entre sessões
</h3>

Antes de Claude Code escrever uma [mensagem entre sessões](/docs/pt/cross-session-messaging) para outra de suas sessões nesta máquina, ele verifica que o socket da caixa de entrada da sessão alvo é o endpoint para o qual a mensagem foi endereçada. Quando uma verificação falha, Claude Code recusa o envio na sessão remetente, e a sessão alvo não recebe nada. Para uma mensagem que Claude envia, a recusa aparece no resultado da ferramenta da sessão remetente:

```text theme={null}
Failed to send to api-worker: Refusing to send: reply target is a symlink
```

O texto após `Refusing to send:` nomeia a verificação que falhou:

* `reply target is a symlink`: um link simbólico fica no caminho do socket da sessão alvo. Claude Code não entrega através dele, porque um link lá poderia redirecionar a mensagem para um endpoint que a sessão alvo não criou.
* `cannot vet reply target`: Claude Code não conseguiu inspecionar o caminho alvo em tudo, por exemplo porque lê-lo falhou com um erro de permissão.
* `connected endpoint is not the expected process`: o processo que mantém o socket não é a sessão para a qual a mensagem foi endereçada, então o endereço é obsoleto ou outro processo substituiu o socket.
* `connected endpoint identity could not be read`: Claude Code se conectou mas não conseguiu ler qual processo mantém a outra extremidade, então não conseguiu confirmar o alvo. Isso pode ser transitório.
* `connected endpoint is not owned by this user`: o processo que mantém o socket é executado como uma conta de usuário diferente, então não é uma de suas sessões.
* `connected endpoint owner could not be read`: Claude Code se conectou mas não conseguiu ler qual conta de usuário possui a outra extremidade, então não conseguiu confirmar que o endpoint é seu.
* `connected endpoint is a different process with the expected pid`: o ID do processo corresponde ao que a mensagem foi endereçada, mas Claude Code não conseguiu confirmar que é o mesmo processo. Geralmente essa sessão saiu e o sistema operacional reutilizou seu ID de processo, então o endereço é obsoleto.

**O que fazer:**

* Geralmente nada: as verificações mantêm uma mensagem de chegar a um endpoint diferente da sessão para a qual foi endereçada, e nada foi enviado
* Peça ao Claude para listar suas sessões novamente e reenviar; uma recusa causada por um endereço obsoleto se limpa uma vez que Claude envia para o atual
* Se `reply target is a symlink` se repete para uma sessão, verifique o que criou um link no caminho do socket dessa sessão, mostrado em seu `/status` sob `Peer address`
* Para `connected endpoint identity could not be read`, reenvie; a condição pode ser transitória
* Se `connected endpoint is not owned by this user` aparece em uma máquina compartilhada, a sessão nesse endereço é executada sob a conta de usuário de outro, então Claude não pode messageá-la da sua

Antes da v2.1.248, Claude Code não verificava o usuário proprietário do endpoint ou o tempo de início do processo, então as recusas que nomeiam essas verificações não aparecem em versões anteriores.

<h3 id="refusing-after-a-symlink-changed">
  Recusando ler, escrever, ou pesquisar um caminho
</h3>

Claude Code verifica as [regras de permissão](/docs/pt/permissions#read-and-edit) de um caminho de arquivo, então confirma essa resolução novamente quando a ferramenta abre o arquivo ou inicia a pesquisa. Quando não consegue confirmar que o caminho ainda leva ao local que a verificação aprovou, Claude Code recusa a operação em vez de segui-la. A recusa aparece no resultado da ferramenta:

```text wrap theme={null}
Refusing to read /path/to/file: its symlink resolution changed after permission was checked (a link on the way now leads somewhere the check did not see). If a link in the working directory is being rewritten concurrently, stop that and retry.
```

Cada recusa nomeia sua razão:

* `its symlink resolution changed after permission was checked`: um symlink ao longo do caminho, ou em uma raiz de pesquisa Grep ou Glob, foi substituído entre a verificação de permissão e a operação. Em uma recusa de leitura, a frase entre parênteses nomeia qual comparação falhou.
* `its parent-directory symlink resolution changed after permission was checked`: um diretório pelo qual o caminho de escrita passa não se resolve mais para o local aprovado
* `it is a symbolic link. Write to the link's target path instead`: um link simbólico fica no local de escrita aprovado em si, por exemplo um `CLAUDE.md` que é um symlink para `AGENTS.md`; a mensagem direciona Claude para o alvo do link
* `Refusing to write through symlink: <path>. Resolve the symlink and pass the real target path explicitly.`: a mesma condição capturada quando outro escritor abre o arquivo, como uma escrita para um `.mcp.json` symlinked
* `Refusing to write into symlinked directory: <path>`: o diretório que mantém o arquivo é em si um link simbólico, por exemplo o diretório `.claude/` de um projeto vinculado a outro local
* `a path one of its Read deny rules is written through changed while the search was being prepared. Retry.`: uma regra de negação `Read` para a pesquisa nomeia um caminho que passa através de um symlink, e esse link mudou enquanto Claude Code estava preparando a pesquisa
* `it could not be opened (EACCES) — it is unreadable, or is being replaced concurrently.`: a raiz de pesquisa existe mas não conseguiu ser aberta; o código entre parênteses é o erro do sistema operacional
* `its permission check expired before it ran (too many concurrent file operations). Retry.`: Claude Code despejou o registro de aprovação sob muitas operações de arquivo simultâneas antes da ferramenta usá-lo; tentar novamente executa uma verificação de permissão fresca
* `ripgrep was found only by name on PATH, and a search outside the working directory cannot apply your Read deny rules in that configuration`: Claude Code não conseguiu resolver o binário `rg` para um caminho absoluto, então recusa pesquisas fora do diretório de trabalho em vez de executar uma que suas regras de negação não cobrem

**O que fazer:**

* Geralmente nada: a recusa chega ao Claude como o resultado da ferramenta, e a operação recusada não é executada
* Se uma recusa de symlink se repete em um caminho, encontre o que continua reescrevendo um link lá, como uma ferramenta de construção ou observador de arquivo, ou peça ao Claude para usar o caminho resolvido do arquivo em vez do vinculado
* Se essa recusa aparece para cada arquivo enquanto Claude Code é executado no Windows dentro de um AppContainer ou sandbox de token restrito, atualize para v2.1.265 ou posterior
* Se uma recusa de leitura aparece em macOS para um arquivo que nada está reescrevendo, como uma captura de tela arrastada para o prompt, atualize para v2.1.273 ou posterior
* Para a recusa de ripgrep, instale ripgrep com seu gerenciador de pacotes para que `rg` se resolva para um caminho absoluto em `PATH`, ou mantenha pesquisas sob o diretório de trabalho

Antes da v2.1.251, Claude Code verificava novamente a resolução de um caminho apenas para escritas de arquivo, então um link substituído após a verificação de permissão poderia redirecionar uma leitura ou pesquisa para um local diferente sem uma mensagem. Desses, apenas as recusas de escrita de diretório pai, através de symlink, e diretório symlinked aparecem em versões anteriores.

<h3 id="task-output-swap-refused">
  Troca de saída de tarefa recusada
</h3>

Claude Code salva a saída de cada comando Bash em um arquivo sob seu diretório temporário. Toda vez que abre um desses arquivos, verifica que o caminho ainda leva ao arquivo que criou, sem link simbólico, link físico extra, ou diretório movido redirecionando-o. Essa mensagem significa que essa verificação falhou, então Claude Code recusou a operação em vez de escrever ou ler saída através desse caminho. A mensagem aparece no resultado da ferramenta Bash:

```text wrap theme={null}
task output swap refused (tasks dir moved or linked): /private/tmp/claude-501/-Users-you-my-project/1f0e62dc-4b0a-4f5e-9c2d-8a7b6c5d4e3f/tasks/b7k2f9m3q.output. To recover: restart Claude Code with CLAUDE_CODE_TMPDIR set to a fresh directory; or, if /private/tmp/claude-501/-Users-you-my-project is a stray directory or a symbolic link that should not be there, remove that entry itself (not what it points to) and restart.
```

O texto entre parênteses nomeia a verificação que falhou. Razões como `output symlink was re-pointed`, `output file identity changed`, e `not a regular file` todas relatam a mesma condição: algo no ou ao longo do caminho de saída não é mais o arquivo que Claude Code criou. Apenas algumas razões carregam uma sentença `To recover:`.

Se a verificação falhar enquanto um comando ainda está em execução, Claude Code para o comando, e seu resultado relata:

```text theme={null}
Command killed: its output file was replaced or could no longer be verified
```

**O que fazer:**

* Atualize para v2.1.260 ou posterior. Versões anteriores às vezes mostravam essa mensagem quando nenhum link ou diretório movido estava presente
* Reinicie Claude Code com [`CLAUDE_CODE_TMPDIR`](/docs/pt/env-vars) definido para um diretório fresco
* Ou verifique o diretório do seu projeto sob o diretório temporário Claude Code, `/private/tmp/claude-501/-Users-you-my-project` na mensagem de exemplo. Se esse caminho é um link simbólico, ou um diretório que não deveria estar lá, remova o link ou diretório em si em vez do alvo do link, e reinicie Claude Code
* Se a recusa se repete, um processo está substituindo, vinculando, ou removendo entradas sob o diretório temporário Claude Code enquanto a sessão é executada. Defina [`CLAUDE_CODE_TMPDIR`](/docs/pt/env-vars) para um diretório que nada mais gerencia e reinicie

<h3 id="the-source-file-is-not-valid-utf-8-text">
  O arquivo de origem não é texto UTF-8 válido
</h3>

Claude tentou publicar um [artefato](/docs/pt/artifacts) de um arquivo cujos bytes não decodificam como texto, ou cujo texto já contém o caractere de substituição `U+FFFD`, então Claude Code recusou a publicação antes de fazer upload de qualquer coisa. A mensagem aparece no resultado da ferramenta Artifact e nomeia a primeira posição a corrigir:

```text wrap theme={null}
file_path: the source file is not valid UTF-8 text (first invalid byte at line 12, column 40). It may be saved in another encoding or contain binary data. Rewrite it as UTF-8, then publish again. Nothing was published.

file_path: the source file has the replacement character U+FFFD at line 12, column 40, usually left where an earlier edit or paste lost a character. Replace it with the intended text (in HTML, write an intended U+FFFD as &#xFFFD;), then publish again. Nothing was published.
```

Claude Code decodifica o arquivo como UTF-8, ou como UTF-16 quando começa com uma marca de ordem de byte UTF-16 little-endian. Quando tal arquivo UTF-16 não decodifica, a primeira mensagem nomeia `UTF-16` e ainda diz a você para reescrever o arquivo como UTF-8. Quando mais posições seguem a nomeada, a mensagem adiciona uma contagem como `(+2 more)` após a posição.

**O que fazer:**

* Geralmente nada: Claude reescreve o arquivo e publica novamente
* Se o arquivo é um que você escreveu ou exportou, salve-o novamente como UTF-8, e substitua cada `U+FFFD` pelo caractere que uma edição, cola, ou conversão anterior perdeu
* Para mostrar um `U+FFFD` intencional na página, escreva-o como `&#xFFFD;` no HTML em vez do caractere literal

Antes da v2.1.267, Claude Code fazia upload de tal arquivo sem verificá-lo, e o servidor recusava a publicação em vez disso.

<h3 id="reading-a-local-file-from-outside-the-connected-folders">
  Lendo um arquivo local de fora das pastas conectadas em uma sessão Cowork
</h3>

Em uma sessão [Cowork](https://claude.com/docs/cowork/overview) em execução em sua máquina no aplicativo Claude Desktop, Claude nomeou um arquivo local para um [artefato](/docs/pt/artifacts). Claude Code não conseguiu confirmar que o arquivo é um arquivo simples dentro das pastas conectadas da sessão: o caminho fica fora dessas pastas, passa através de um link simbólico, ou é soletrado de uma forma que pode nomear um arquivo diferente do que parece. Ler tal arquivo precisa de sua aprovação, e em uma sessão que não pode mostrar a você o cartão de aprovação, como uma definida para pular todas as aprovações, Claude Code recusa a leitura.

A recusa aparece no resultado da ferramenta Artifact; quando o arquivo não conseguiu ser examinado em tudo, nomeia essa falha em vez disso:

```text wrap theme={null}
Reading a local file from outside this session's connected folders, or through a link, needs the approval card, and no one can answer it in this Cowork session. Use a plain file inside the connected folders; do not retry this file in this session.

cannot read file_path (ENOENT) — the file could not be examined, and no one can answer the approval card in this Cowork session. Check that the file exists as a plain file inside the connected folders, then retry with that path.
```

**O que fazer:**

* Geralmente nada: a mensagem diz ao Claude para usar um arquivo simples dentro das pastas conectadas em vez disso
* Para colocar esse arquivo exato no artefato, copie-o para uma das pastas conectadas da sessão como um arquivo regular, não um symlink, e pergunte novamente

<h3 id="webfetch-cannot-fetch-localhost">
  WebFetch não consegue buscar localhost
</h3>

Claude chamou [WebFetch](/docs/pt/tools-reference#webfetch-tool-behavior) com uma URL cujo nome de host não tem ponto, como `http://localhost:3000` ou um nome de intranet simples como `http://wiki/`. WebFetch recusa essas URLs antes de fazer qualquer solicitação:

```text wrap theme={null}
WebFetch cannot fetch localhost or other hostnames without a dot. To reach a local server, use Bash with curl instead.
```

**O que fazer:**

* Geralmente nada: a mensagem aponta Claude para `curl` através da ferramenta Bash, que pode alcançar servidores locais e intranet

Antes da v2.1.268, WebFetch relatava essas URLs com um erro genérico `Invalid URL`.

<h2 id="background-session-errors">
  Erros de sessão em background
</h2>

As [sessões em background](/docs/pt/agent-view) são executadas sem um terminal interativo próprio, portanto os comandos que precisam de um se comportam de forma diferente lá. Essas mensagens aparecem na transcrição de uma sessão em background, no terminal que se conecta a uma, na sessão ou shell de onde você a despacha, ou, para as [entradas worktree-guard](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) abaixo, em qualquer sessão isolada em uma worktree ou executando um subagente isolado em worktree; quando uma mensagem é específica de uma superfície, sua entrada diz isso.

<h3 id="commands-refused-in-a-background-session">
  Comandos recusados em uma sessão em background
</h3>

Comandos que abrem um diálogo interativo não podem fazer isso enquanto nenhum terminal está conectado a uma sessão em background. `/install-github-app`, a lista de configurações `/mcp`, e as ações de autenticação no menu do servidor MCP respondem com uma mensagem, e a sessão aparece sob **Needs input** na [agent view](/docs/pt/agent-view) para que você possa encontrá-la, conectar e executar o comando novamente. Enquanto um terminal está conectado, esses comandos funcionam normalmente.

Antes da v2.1.216, a sessão não aparecia sob **Needs input** após uma dessas recusas. Na v2.1.213 até v2.1.215, os comandos ainda funcionavam enquanto um terminal estava conectado, e a mensagem de recusa dizia para você conectar e executar o comando novamente. De v2.1.208 até v2.1.212, Claude Code recusava-os mesmo enquanto um terminal estava conectado, com uma mensagem como `Can't open MCP settings in a background session`; nessas versões, execute o comando de uma sessão `claude` regular, ou atualize. Antes da v2.1.208, eles abriam seu diálogo dentro da sessão em background. Na v2.1.208 apenas, Claude Code também recusava o seletor `/model` em uma sessão em background, e `/upgrade` imprimia a URL de atualização em vez de abrir um navegador.

A redação nomeia o comando. A lista de configurações `/mcp` relata:

```text theme={null}
Can't open MCP settings while no terminal is attached to this background session. This session now shows "needs input" in agent view — open it and run /mcp to manage servers, or use `/mcp enable|disable|reconnect <server>` to steer without the panel.
```

**O que fazer:**

* Conecte à sessão a partir da agent view, onde ela está listada sob **Needs input**, e execute o comando novamente
* Ou use o formulário que a mensagem nomeia, como `/mcp reconnect <server>`, `/mcp enable`, ou `/mcp disable`, que funcionam sem conectar

<h3 id="write-or-command-blocked-because-the-path-cannot-be-safely-resolved">
  Write ou command bloqueado porque o caminho não pode ser resolvido com segurança
</h3>

Claude abordou um arquivo ou diretório de trabalho através de uma grafia que o [worktree-isolation guard](/docs/pt/agent-view#how-file-edits-are-isolated) não consegue resolver para um local verificável. O guard verifica writes e diretórios de trabalho de comando em [qualquer sessão isolada em uma worktree](/docs/pt/worktrees#how-claude-code-enforces-isolation), interativa ou background, e em [subagentes isolados em worktree](/docs/pt/worktrees#isolate-subagents-with-worktrees). Ele resolve symlinks antes de verificar que a operação não atinge o checkout compartilhado, e quando a resolução falha, ele bloqueia a operação em vez de deixá-la chegar lá. A mensagem nomeia as formas de caminho que recusa e como tentar novamente:

```text theme={null}
This write was blocked because the path is spelled in a form that cannot be safely resolved (for example through a symlink storing a raw dot segment, a network-share or device-namespace shape, or an unreadable ancestor directory). If the file is inside the worktree /path/to/worktree, address it by its direct symlink-free path instead.
```

Um comando bloqueado relata a mesma causa para seu diretório de trabalho e termina com `re-run the command from its direct symlink-free path`. Antes da v2.1.217, o guard comparava grafias de caminho sem resolver symlinks, portanto essas grafias não eram bloqueadas e um write roteado através de um symlink poderia chegar ao checkout compartilhado.

**O que fazer:**

* Geralmente nada: a mensagem completa vai para Claude como um erro de ferramenta, e Claude tenta novamente com o caminho direto que nomeia. Para um edit de arquivo bloqueado, a visualização de conversa mostra apenas uma linha curta `Error editing file`; a mensagem completa aparece na visualização de transcrição, que você abre com `Ctrl+O`. Um comando bloqueado a imprime em sua saída de comando.
* Se o bloqueio se repetir no mesmo arquivo, o caminho provavelmente passa por um symlink confirmado cujo alvo contém `..`, como `docs/current -> ../README.md`; peça a Claude para editar o arquivo de destino por seu caminho real em vez de através do link

<h3 id="write-or-command-blocked-because-the-path-names-a-network-location">
  Write ou command bloqueado porque o caminho nomeia um local de rede
</h3>

Claude abordou um arquivo ou diretório de trabalho através de um caminho que nomeia uma unidade que não está em sua máquina, um compartilhamento UNC como `\\server\share\file` ou um caminho de automontagem `/net`, enquanto o checkout da sessão está em um disco local. O mesmo [worktree-isolation guard](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) não consegue verificar que tal caminho fica fora do checkout compartilhado, portanto bloqueia a operação. Isolar a sessão em uma worktree não levanta o bloqueio. A mensagem nomeia a forma de caminho a usar em vez disso:

```text theme={null}
This write was blocked because the path is network-shaped (a UNC share or /net automount spelling) while this session's checkout is local. Isolating cannot unblock it. If the file is genuinely inside the worktree /path/to/worktree, address it by its local, plainly-spelled path instead.
```

Um comando bloqueado relata a mesma causa para seu diretório de trabalho e termina com `re-run the command from its local, plainly-spelled path`. Antes da v2.1.217, o guard comparava apenas texto de caminho, portanto endereçar um arquivo dentro do checkout através de um caminho UNC ou `/net` não era bloqueado.

**O que fazer:**

* Geralmente nada: Claude tenta novamente com a grafia local que a mensagem pede
* Se o arquivo está em um compartilhamento de rede em vez de um arquivo local escrito com um caminho de rede, está fora do workspace local da sessão; edite-o de uma sessão interativa regular em vez disso

<h3 id="command-blocked-by-the-worktree-isolation-checks">
  Comando bloqueado pelas verificações de isolamento de worktree
</h3>

Claude executou um comando Bash ou Monitor em uma [sessão isolada em uma worktree](/docs/pt/worktrees#how-claude-code-enforces-isolation), e Claude Code recusou-o por uma de duas razões:

* O comando aponta git para o checkout principal.
* Claude Code não consegue verificar a partir do texto do comando que qualquer git que o comando executa fica dentro da worktree. Um comando que nunca nomeia git ainda pode ser recusado por essa razão, porque expandir uma indireção de variável como `${!name}` ou executar uma substituição de função Bash como `${ command; }` produz um valor em tempo de execução que pode ser um comando em si.

O meio da mensagem nomeia o que não pôde ser verificado:

```text wrap theme={null}
This session is isolated in the worktree /path/to/worktree, but this command evaluates ${!x@P} arithmetically inside a construct too complex to verify, which can run a command hidden in a variable's value. Refusing to run it — a worktree-isolated session's git operations must target its own worktree. Split it into plain, separate commands and run them from /path/to/worktree.
```

**O que fazer:**

* Geralmente nada: Claude lê a mensagem e reescreve o comando da forma que sua sentença final pede
* Se um comando que você pediu continua sendo recusado, escreva o valor sinalizado literalmente: substitua a indireção ou substituição por seu valor, e execute git como seu próprio comando simples de dentro da worktree
* Para agir no checkout principal propositalmente, execute o comando você mesmo em um terminal fora da sessão

<h3 id="this-session-has-no-saved-transcript">
  Esta sessão não tem transcrição salva
</h3>

Você conectou a uma [sessão em background](/docs/pt/agent-view) parada que foi colocada em background de outra conversa com `←` ou `/background` e parada antes de sua primeira resposta terminar. Até que essa primeira resposta termine, a conversa ainda vive apenas na sessão de onde foi colocada em background, portanto `claude attach` recusa iniciar a sessão parada em vez de começar uma conversa em branco sob o mesmo ID de sessão. A mensagem termina com o comando `claude respawn` para esta sessão:

```text theme={null}
This session has no saved transcript — it was stopped before its first response finished. If it was backgrounded from another conversation, that one is still intact; `claude respawn <id>` starts this one fresh.
```

Abrir a mesma linha de sessão na [agent view](/docs/pt/agent-view) mostra `Press enter again to restart this session fresh` abaixo da lista, e um segundo `Enter` na linha reinicia a sessão com uma conversa vazia. Antes da v2.1.212, abrir a linha mostrava a mensagem de recusa sem forma de reiniciar a partir da agent view. Antes da v2.1.211, abrir a sessão parada silenciosamente iniciava essa conversa em branco e poderia re-executar o prompt original da sessão.

**O que fazer:**

* A conversa que você colocou em background está intacta: retome-a com [`claude --resume`](/docs/pt/sessions) ou continue trabalhando nela
* Para iniciar a sessão parada do zero mesmo assim, execute `claude respawn <id>` com o ID da mensagem, ou pressione `Enter` duas vezes na sua linha na agent view
* Se a sessão terminou uma resposta e você ainda vê essa recusa em uma versão anterior à v2.1.214, uma pasta ilegível em `~/.claude/projects` poderia fazer a varredura de transcrição perder a conversa salva; atualize para v2.1.214 ou posterior, que tolera pastas ilegíveis durante a varredura

<h3 id="this-session-is-running-in-another-terminal">
  Esta sessão está sendo executada em outro terminal
</h3>

Você abriu a linha de uma sessão parada na [agent view](/docs/pt/agent-view), e sua conversa salva já está aberta em outro processo Claude Code ao vivo nesta máquina, portanto Claude Code recusa iniciar um segundo processo que escreveria na mesma transcrição. Qual mensagem você vê depende de [o que mantém a conversa](/docs/pt/agent-view#opening-a-session-says-the-conversation-is-already-open):

```text theme={null}
Can't open — this session is running in another terminal
This conversation is already open in another running Claude session — use that one, or close it and try again
```

* **`running in another terminal`**: um terminal mantém a conversa, por exemplo um onde você a retomou com `claude --resume` ou `/resume`. A linha também mostra `Open in a terminal`.
* **`already open in another running Claude session`**: outro processo Claude Code não-interativo a mantém, por exemplo um processo de [sessão em background](/docs/pt/agent-view#the-supervisor-process) para a mesma conversa que ainda não saiu.

Claude Code salva uma resposta que você digitou ao abrir a linha e a envia como o próximo prompt da sessão quando a sessão iniciar novamente.

**O que fazer:**

* Continue a conversa no processo que a tem aberta, ou saia desse processo e abra a linha novamente

Antes da v2.1.248, apenas a recusa `already open in another running Claude session` existia: uma conversa retomada em um terminal não contava como aberta, e abrir a linha iniciava um segundo processo Claude Code escrevendo na mesma conversa.

<h3 id="this-sessions-saved-conversation-is-no-longer-on-disk">
  A conversa salva desta sessão não está mais no disco
</h3>

Você abriu uma [sessão em background](/docs/pt/agent-view) que terminou enquanto o serviço em background estava desligado, e [limpeza de transcrição](/docs/pt/settings-reference#cleanupperioddays) desde então removeu sua conversa salva, por exemplo após a máquina estar desligada por semanas. Abrir tal linha normalmente [retoma sua conversa salva](/docs/pt/agent-view#sessions-show-as-failed-after-shutdown). Sem nada para retomar, Claude Code recusa em vez de re-executar o prompt original da sessão sem perguntar:

```text theme={null}
This session's saved conversation is no longer on disk (it ended while the background service was off, and old transcripts are cleaned up), so there is nothing to resume. `claude rm 7c5dcf5d` deletes the row; `claude respawn 7c5dcf5d` runs its original prompt again instead.
```

`claude attach <id>` imprime este texto. Na agent view, o rodapé é mais curto e termina com `ctrl+x deletes the row`.

**O que fazer:**

* Execute `claude rm <id>` para deletar a linha. Quando um dos [casos mantidos](/docs/pt/agent-view#what-deleting-a-session-removes) se aplica, `claude rm` mantém a linha e a worktree em vez disso e nomeia a razão
* Para executar o prompt original da sessão novamente como uma conversa fresca, execute `claude respawn <id>`

Antes da v2.1.248, abrir tal linha re-executava o prompt original da sessão em vez de recusar, puxando uma tarefa de semanas atrás para o primeiro plano.

<h3 id="worktree-has-commits-that-are-not-pushed-anywhere">
  Worktree tem commits que não foram enviados para lugar nenhum
</h3>

Você tentou deletar uma [sessão em background](/docs/pt/agent-view#what-deleting-a-session-removes) cuja worktree contém commits que Claude Code não consegue confirmar que estão salvos em outro lugar. Claude Code mantém a worktree e a linha de sessão em vez de destruir os commits sem vê-los. `claude rm` nomeia o branch e os commits não enviados, e diz como proceder:

```text theme={null}
kept 7c5dcf5d — its worktree is still at "/home/you/project/.claude/worktrees/fix-login"
  2 unpushed commits on "claude/fix-login": a1b2c3d "Fix login flow" and 1 more. They exist on no remote, so deleting the worktree would lose them.
  push them and run 'claude rm 7c5dcf5d' again, or discard the worktree and its commits: claude rm 7c5dcf5d --discard-unpushed a1b2c3d000000000000000000000000000000000@0123456789abcdef0123456789abcdef
```

Quando Claude Code não consegue resumir os commits, a linha de detalhe lê `The worktree has unpushed commits` em vez disso. Na [agent view](/docs/pt/agent-view), a linha da sessão mostra `not deleted` com a mesma razão.

Commits em um remote não bloqueiam o delete. Nem commits na cópia local do branch padrão do seu remote `origin`, desde que esse branch esteja verificado no seu checkout principal, o diretório do repositório em si em vez de uma worktree.

**O que fazer:**

* Para manter os commits, envie o branch da worktree, ou mescle-o no branch padrão verificado no seu checkout principal, depois delete a sessão novamente
* Para descartar os commits, execute o comando `claude rm <id> --discard-unpushed` que a mensagem imprimiu, ou pressione `Ctrl+X` duas vezes na linha da sessão na agent view novamente. Isso remove a sessão e a worktree junto com seu branch, os commits não enviados, e quaisquer mudanças não confirmadas. Se a worktree ganhou um commit desde a recusa, Claude Code a mantém novamente e mostra o estado atualizado
* Quando a mensagem diz que a worktree também é registrada por outra sessão terminada, deletar novamente não a descarta: envie os commits, depois delete a sessão novamente

Antes da v2.1.268, `claude rm` colocava o resumo de commit na linha `kept` em si. Quando `claude rm` não conseguia resumir os commits, a linha `kept` lia `worktree has commits that are not pushed anywhere` no lugar do resumo.

Antes da v2.1.260, a mensagem não nomeava o branch ou os commits, e deletar novamente era recusado da mesma forma: deletar a sessão sem enviar significava remover a worktree você mesmo com `git worktree remove --force <path>`, depois executar `claude rm <id>` novamente.

Antes da v2.1.248, o branch padrão verificado no seu checkout principal não contava: um branch que você já tinha mesclado lá ainda acionava essa recusa até seus commits chegarem a um remote.

<h3 id="terminal-host-process-died">
  Processo host do terminal morreu
</h3>

Cada terminal de [sessão em background](/docs/pt/agent-view) é executado em um processo host sob o serviço em background, e esse processo morreu enquanto o serviço ainda mantinha sua conexão, portanto a sessão não pôde ser alcançada.

No Linux e WSL, o serviço em background verifica cada processo host a cada poucos segundos, marca a sessão como falha quando o processo saiu mas sua conexão com o serviço nunca fechou, e mostra a razão em sua linha na [agent view](/docs/pt/agent-view#read-session-state):

```text theme={null}
terminal host process died — press Enter to restart
```

Se você abrir a linha antes da verificação ser executada, o rodapé mostra `This session's terminal host process died (the conversation is saved) — press Enter to restart it` e a linha fica falha.

Do shell, `claude attach <id>` reinicia uma sessão já marcada como falha por um host morto, e caso contrário imprime a causa e sai:

```text theme={null}
Couldn't attach to <id> — This session's terminal host process died (the conversation is saved) — run `claude attach <id>` again to restart it on a fresh host.
```

A conversa está salva de qualquer forma.

Uma linha executando um [comando shell](/docs/pt/agent-view#run-a-shell-command) em vez disso mostra `terminal host process died — its output is gone; the command was not run again`, e `claude attach` imprime `This command's terminal host process died — its output is gone and the command was not run again`. Claude Code nunca re-executa o comando para você.

**O que fazer:**

* Na agent view, pressione `Enter` na linha falha; a sessão reinicia em um novo processo host e a conversa retoma
* Do shell, execute `claude attach <id>` novamente. Claude Code imprime `Session <id>'s terminal host died — restarting it on a fresh one…` e reabre a sessão
* Você não consegue reiniciar uma linha de comando shell dessa forma; despache o comando novamente para re-executá-lo

Antes da v2.1.247, um processo host morto poderia passar em cada verificação de vivacidade que o serviço em background executava, portanto abrir a sessão mostrava `opening… · esc to cancel` indefinidamente e `claude attach <id>` esperava sem relatar um erro.

<h3 id="session-isnt-responding">
  Sessão não está respondendo
</h3>

Você abriu uma [sessão em background](/docs/pt/agent-view) e o serviço em background aceitou a abertura, mas nenhuma saída chegou por cerca de dez segundos, portanto Claude Code conclui que o processo retransmitindo o terminal da sessão não consegue entregar saída, e encerra a tentativa em vez de esperar.

Na agent view, Claude Code oferece um reinício no rodapé:

```text theme={null}
Press enter again to restart this session — it isn't responding (its conversation is saved and resumes).
```

Do shell, `claude attach <id>` imprime a causa e sai:

```text theme={null}
Couldn't attach to <id> — Session isn't responding — `claude stop <id>`, then `claude attach <id>` restarts it (the conversation is saved).
```

Claude Code nunca reinicia uma linha executando um [comando shell](/docs/pt/agent-view#run-a-shell-command) para você, porque um reinício executaria o comando novamente.

**O que fazer:**

* Na agent view, pressione `Enter` na mesma linha novamente. Claude Code para o processo que não responde e reinicia a sessão, e a conversa retoma. Nada é parado sem esse segundo pressionamento
* Do shell, execute `claude stop <id>`, depois `claude attach <id>`
* Para uma linha de comando shell, pressione `Ctrl+X` na agent view ou execute `claude stop <id>` para pará-la; despache o comando novamente para re-executá-lo

<h3 id="session-was-stopped-while-the-respawn-was-in-flight">
  Sessão foi parada enquanto o respawn estava em voo
</h3>

Você abriu uma [sessão em background](/docs/pt/agent-view) cujo processo não estava em execução, e enquanto Claude Code estava reiniciando-a, outro processo Claude Code a parou, por exemplo `claude stop` em outro terminal. Claude Code mantém a sessão parada:

```text theme={null}
Session <id> was stopped while the respawn was in flight
```

Abrir uma sessão que você acabou de despachar, enquanto seu processo ainda está iniciando, espera pelo processo em vez disso. Antes da v2.1.246, abri-la naquele momento poderia pará-la e mostrar essa mensagem.

**O que fazer:**

* Se você não parou a sessão, abra sua linha novamente na agent view ou execute `claude respawn <id>` para reiniciá-la
* Se você a parou você mesmo, nada permanece a fazer: a sessão fica parada

<h3 id="session-agent-no-longer-available">
  Agente de sessão não está mais disponível
</h3>

Você retomou uma sessão que estava executando um [agente customizado](/docs/pt/sub-agents#invoke-subagents-explicitly), iniciado com `--agent` ou a configuração `agent`, e Claude Code não encontrou um agente com esse nome. Ele procura no diretório original da sessão primeiro, quando você [confiou naquele workspace](/docs/pt/permissions#project-allow-rules-and-workspace-trust), depois no diretório de onde você retoma. A sessão ainda retoma, mas com as ferramentas padrão, portanto as restrições de ferramenta do agente não se aplicam mais:

```text theme={null}
This session was running agent 'code-reviewer', which is no longer available (no agent by that name in /home/you/project). Continuing with the default tools and system prompt — the agent's tool restrictions no longer apply. To restore it, re-create the agent, or resume with an explicit --agent <name>.
```

O aviso nomeia apenas os diretórios que Claude Code procurou, e aparece na conversa retomada se você acordar uma [sessão em background](/docs/pt/agent-view), executar `/resume` ou `claude --resume`, ou retomar em [modo não-interativo](/docs/pt/headless), onde também vai para stderr. Sessões usando `--input-format stream-json` não o mostram, porque o Agent SDK fornece agentes após a inicialização.

Claude Code não salva o fallback na sessão, portanto o aviso se repete em cada retomada até você agir. O agente `claude` integrado não aciona o aviso, já que fazer fallback para o conjunto de ferramentas padrão não muda nada para ele. Antes da v2.1.216, Claude Code silenciosamente continuava como o agente padrão, e a busca cobria apenas o diretório de onde você retomava, portanto um agente com escopo de projeto era perdido em qualquer retomada de outro diretório.

**O que fazer:**

* Re-crie o arquivo de agente em `.claude/agents/<name>.md` no projeto da sessão, ou em `~/.claude/agents/<name>.md` para um agente pessoal, depois retome novamente
* Ou retome com `--agent <name>` nomeando um agente que existe, para executar a sessão como esse agente em vez disso
* Se o agente tem escopo de projeto e você não confiou no diretório original da sessão, execute Claude Code lá uma vez, aceite o diálogo de confiança, depois retome novamente

<h3 id="claude_code_process_wrapper-launcher-errors">
  Erros do launcher CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/pt/corporate-launcher) está definido, e seu valor não pode ser usado, portanto Claude Code recusa iniciar o processo afetado em vez de executá-lo sem o launcher. Problemas de configuração são relatados com uma mensagem que começa com o nome da variável e declara a razão, por exemplo:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Um launcher que inicia mas sai sem se substituir por Claude Code falha a sessão que estava iniciando, e a linha da sessão na agent view relata que o launcher `must exec, not daemonize`, seguido por qualquer coisa que o launcher imprimiu. Uma sessão que não consegue iniciar ou alcançar o serviço em background por causa do launcher relata o problema do launcher como a razão dentro de `Couldn't reach the background service (...)`.

**O que fazer:**

* Defina a variável para o caminho absoluto de um executável que termina chamando `exec "$@"`. Veja [o contrato do launcher](/docs/pt/corporate-launcher#the-launcher-contract) para o contrato completo
* Verifique `/status`, que mostra o comando de inicialização resolvido em sua entrada Self-exec e avisa quando o serviço em background em execução não corresponde a ele, ou execute `claude daemon status` de um shell
* Após corrigir o valor no bloco `env` de [settings](/docs/pt/corporate-launcher#set-up-the-launcher), reinicie o serviço em background com `claude daemon stop --any` para que o próximo despacho inicie um envolvido

<h3 id="eunknown-when-starting-a-background-session">
  EUNKNOWN ao iniciar uma sessão em background
</h3>

Windows recusou iniciar um programa com um código de erro que não tem nome padrão, portanto a falha aparece como `EUNKNOWN`. O gatilho usual é uma política de restrição de software, como Group Policy ou AppLocker, bloqueando o programa sendo iniciado. O erro aparece quando você inicia uma [sessão em background](/docs/pt/agent-view) com `/background` ou `claude --bg`:

```text theme={null}
Couldn't reach the background service (spawn background service: EUNKNOWN: unknown error, uv_spawn) — run 'claude daemon status'
```

Em algumas contas a mensagem diz `daemon` no lugar de `background service`.

Em uma instalação npm, um `EUNKNOWN` que aparece enquanto `npm install -g @anthropic-ai/claude-code` está substituindo o binário tem a mesma causa que [`EACCES` durante uma reinstalação](#eacces-when-starting-a-background-session) e limpa quando você tenta novamente após a instalação terminar.

Claude Code inicia o serviço em background através do PowerShell para que o serviço sobreviva ao fechamento do terminal, usando PowerShell 7 quando está instalado e Windows PowerShell 5.1 caso contrário. Quando nenhum PowerShell consegue executar, Claude Code inicia o serviço diretamente em vez disso, portanto uma política que bloqueia apenas PowerShell não causa esse erro. Se você o vê enquanto nenhuma instalação npm está em execução, a política está bloqueando o executável Claude Code em si.

Antes da v2.1.212, Claude Code usava apenas Windows PowerShell 5.1 para iniciar o serviço, portanto qualquer máquina onde Group Policy bloqueava PowerShell 5.1 falhava com `Couldn't start the session — EUNKNOWN: unknown error, uv_spawn`, mesmo com PowerShell 7 instalado.

**O que fazer:**

* Se a mensagem lê `Couldn't start the session`, atualize para v2.1.212 ou posterior. Em versões anteriores você também pode executar `claude daemon run` em um terminal separado primeiro, depois iniciar a sessão em background novamente. Esse comando executa o serviço em background no primeiro plano do terminal, portanto o serviço dura apenas enquanto esse terminal fica aberto.
* Se uma instalação npm estava substituindo o binário, espere por ela terminar, depois inicie a sessão em background novamente
* Se o erro aparece em v2.1.212 ou posterior enquanto nenhuma instalação npm está em execução, peça ao seu administrador Windows para permitir o executável Claude Code na política de restrição
* Se o serviço em background para quando você fecha o terminal, Claude Code o iniciou sem PowerShell. Instale PowerShell 7, ou peça ao seu administrador para desbloquear PowerShell, para que o serviço possa sobreviver ao terminal.

<h3 id="eacces-when-starting-a-background-session">
  EACCES ao iniciar uma sessão em background
</h3>

Claude Code não conseguiu executar seu próprio binário para iniciar o [serviço em background](/docs/pt/agent-view#the-supervisor-process) que hospeda sessões em background. Em uma instalação npm, isso geralmente significa que `npm install -g @anthropic-ai/claude-code` estava substituindo o binário naquele momento, se você o executou ou o [auto-updater](/docs/pt/setup#auto-updates) fez. O erro aparece quando você abre uma sessão da [agent view](/docs/pt/agent-view):

```text theme={null}
Couldn't start the background service — spawn background service: EACCES: permission denied, posix_spawn '/usr/local/lib/node_modules/@anthropic-ai/claude-code/bin/claude'
```

Quando você inicia uma sessão com `/background` ou `claude --bg`, a mesma razão aparece dentro de `Couldn't reach the background service (...)`. Durante a mesma janela de reinstalação o erro pode nomear outro código em vez disso, como `ENOENT` ou `ENOEXEC`, ou `EUNKNOWN` ou `EPERM` no Windows; um `EUNKNOWN` que persiste através de tentativas tem uma [causa diferente](#eunknown-when-starting-a-background-session).

Em uma instalação npm, Claude Code espera a reinstalação terminar e tenta novamente por conta própria: até dez segundos, e até dois minutos enquanto uma instalação npm de Claude Code está visivelmente ainda em execução na máquina, o que cobre outro processo Claude Code baixando uma atualização. Quando a instalação dura mais que essa espera, a falha nomeia a atualização em vez do código de erro simples:

```text theme={null}
Claude Code is being updated by npm on this machine (still not runnable after 2 min, EACCES) — try again when the update finishes
```

Antes da v2.1.257, a espera parava em dez segundos em cada caso, portanto esse erro aparecia enquanto outro processo Claude Code ainda estava baixando uma atualização. Antes da v2.1.246, Claude Code falhava imediatamente, sem esperar.

**O que fazer:**

* Espere alguns segundos, depois abra a sessão ou despache novamente. Quando a mensagem diz que Claude Code está sendo atualizado, tente novamente após a atualização terminar.
* Se o erro persiste enquanto nenhuma instalação npm está em execução, seu usuário não consegue executar o binário instalado. Verifique suas permissões e as de seu diretório, ou reinstale Claude Code.

<h3 id="background-service-exited-before-it-became-reachable">
  Serviço em background saiu antes de se tornar alcançável
</h3>

O processo que Claude Code iniciou como o [serviço em background](/docs/pt/agent-view#the-supervisor-process) saiu antes de aceitar conexões, portanto Claude Code não conseguiu abrir sua sessão. Quando o serviço imprimiu um erro antes de sair, a razão entre parênteses dá o código de saída ou sinal e a primeira linha que o serviço imprimiu, que nomeia o que o parou:

```text theme={null}
Couldn't reach the background service (background service exited before it became reachable (exit code N): <the service's first error line>) — run 'claude daemon status'
```

Quando você abre uma sessão da [agent view](/docs/pt/agent-view), a mesma razão segue `Couldn't start the background service —`. Quando o serviço não imprimiu nada antes de sair, a mensagem diz `nothing on stderr` em vez disso.

Claude Code relata a falha com a linha de erro do serviço. Antes da v2.1.246, a falha aparecia apenas após uma espera de 45 segundos, como `background service did not become reachable within 45s`, sem a linha de erro do serviço.

Duas razões citadas têm causas conhecidas:

* `Error: claude native binary not installed.`: uma instalação npm estava substituindo o binário Claude Code naquele momento, portanto o serviço executou o placeholder do npm em vez disso. Tente novamente após a instalação terminar; se a linha persiste sem nenhuma instalação em execução, [complete a instalação npm](/docs/pt/troubleshoot-install#native-binary-not-found-after-npm-install). Antes da v2.1.257, uma auto-atualização npm do macOS produzia essa falha em cada início durante a janela de instalação.
* `nothing on stderr` com código de saída 1, em cada início, no Windows: `daemon.lock` nomeia um processo que Claude Code não consegue sinalizar nem provar que se foi, portanto cada novo serviço conclui que outro mantém o lock e sai. Um lock cujo escritor Claude Code consegue provar que se foi é substituído por conta própria e não produz essa falha. Quando a falha se repete em cada início, delete `~/.claude/daemon.lock`, depois abra a sessão ou despache novamente. Antes da v2.1.257, tal lock bloqueava cada início até você deletar o arquivo.

**O que fazer:**

* Se a mensagem cita uma linha, corrija o que ela nomeia, depois abra a sessão ou despache novamente. A próxima tentativa inicia o serviço novamente
* Execute `claude daemon status` para verificar se um serviço está em execução agora

<h3 id="working-directory-no-longer-exists-when-starting-a-background-session">
  Diretório de trabalho não existe mais ao iniciar uma sessão em background
</h3>

Você tentou iniciar uma [sessão em background](/docs/pt/agent-view) em um diretório que não existe mais. Isso acontece quando você despacha da agent view ou executa `/background` após o diretório em que você estava trabalhando ser deletado ou movido. Também acontece quando você conecta a ou reinicia uma sessão cujo processo saiu e cujo diretório se foi, porque o novo processo iniciaria naquele mesmo diretório. Claude Code não inicia a sessão, e a mensagem nomeia o diretório faltante:

```text theme={null}
Couldn't start a background session (working directory no longer exists or is not accessible: /tmp/demo)
```

Antes da v2.1.257, a sessão parecia iniciar e depois mostrava na agent view como uma linha falha com a mesma razão.

**O que fazer:**

* Recrie o diretório que a mensagem nomeia, ou despache de um diretório que existe, depois tente novamente

<h2 id="wrapper-and-ide-errors">
  Erros de wrapper e IDE
</h2>

Esses erros vêm do programa que iniciou o Claude Code para você, como uma extensão de IDE ou uma aplicação [Agent SDK](/docs/pt/agent-sdk/overview), em vez de virem do próprio Claude Code.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code process exited with code N
</h3>

O processo `claude` subjacente saiu com um código diferente de zero. O código de saída por si só não diz o que falhou: o erro real está na saída do próprio processo, que o wrapper anexa quando capturou alguma coisa e caso contrário mantém em seus logs.

```text theme={null}
Error: Claude Code process exited with code 1
```

No Windows, a compilação nativa pode sair com o código `4294967295` logo após um turno ser concluído. Quando essa saída ocorre em um limite de turno, sem mensagem aguardando e sem tarefa em segundo plano em execução, a [extensão VS Code](/docs/pt/vs-code) fecha a sessão silenciosamente em vez de mostrar esse erro. Sua próxima mensagem retoma a conversa.

Antes da v2.1.273, a extensão mostrava o erro para essa saída em cada limite de turno, mesmo que nada fosse perdido.

**O que fazer:**

* No VS Code, siga o link **View output logs** mostrado com o erro para ver a falha subjacente
* Em uma aplicação Agent SDK, capture o erro em torno de seu loop de mensagens. As entradas em [CLI process exit](/docs/pt/agent-sdk/troubleshooting#cli-process-exit) cobrem o que seu código recebe em cada linguagem SDK.
* Execute `claude` em um terminal no mesmo projeto. A falha geralmente se reproduz lá com sua mensagem de erro real, que você pode então procurar nesta página.
* Execute `claude doctor` em um terminal para verificar a instalação e configuração

<h3 id="could-not-locate-the-claude-cli-on-path">
  Could not locate the Claude CLI on PATH
</h3>

A [extensão VS Code](/docs/pt/vs-code) mostra esse erro no Windows quando você abre Claude Code no terminal integrado, o shell do terminal é PowerShell e a extensão não consegue encontrar o executável `claude` instalado no PATH. A extensão se recusa a iniciar Claude Code até encontrar o `claude` instalado no PATH.

```text theme={null}
Failed to run Claude Code: Error: Could not locate the Claude CLI on PATH. Launching by name in a PowerShell terminal would run a 'claude' from the open folder instead of the installed CLI, so the launch was blocked. Make sure the Claude CLI's install directory is on your system PATH (not only your PowerShell profile), then restart VS Code and try again. VS Code reads PATH when it starts, so PATH changes take effect only after a restart.
```

**O que fazer:**

* Abra uma nova janela PowerShell fora do VS Code e execute `where.exe claude`. Se não imprimir um caminho, a CLI não está no seu PATH: adicione seu diretório de instalação seguindo [Verify your PATH](/docs/pt/troubleshoot-install#verify-your-path). Se imprimir um caminho, a entrada vem de seu perfil PowerShell ou de uma mudança de PATH que o VS Code ainda não captou; os próximos dois passos cobrem esses casos.
* Defina a entrada PATH como uma variável de ambiente de usuário ou sistema, não em seu perfil PowerShell. A extensão não executa seu perfil, portanto uma edição de PATH que existe apenas lá nunca a alcança.
* Reinicie o VS Code após alterar PATH. A extensão verifica o PATH que o VS Code capturou na inicialização, portanto uma mudança de PATH entra em vigor apenas após uma reinicialização.

<h3 id="the-connection-to-claude-code-ended-before-this-message-completed">
  The connection to Claude Code ended before this message completed
</h3>

A [extensão VS Code](/docs/pt/vs-code) enviou sua mensagem para o processo `claude`, e a conexão terminou sem um erro antes do processo reconhecer ou concluir. A extensão não consegue dizer se a mensagem foi processada, portanto pede que você a envie novamente:

```text theme={null}
The connection to Claude Code ended before this message completed — it may not have been processed, so please send it again.
```

**O que fazer:**

* Envie a mensagem novamente. A próxima mensagem inicia um novo processo `claude` que retoma a conversa.
* Se se repetir, execute `claude` em um terminal no mesmo projeto. Uma falha que continua encerrando o processo geralmente se reproduz lá com sua mensagem de erro real.

<h2 id="rewind-warnings-and-errors">
  Avisos e erros de Rewind
</h2>

Estas mensagens vêm de uma restauração de código [`/rewind`](/docs/pt/checkpointing). `Restored the code, but skipped N files` é um aviso de que Claude Code pulou alguns caminhos. `No files were restored` é um erro que significa que nada foi restaurado.

<h3 id="restored-the-code-but-skipped-files">
  Restored the code, but skipped files
</h3>

Uma restauração de código `/rewind` pulou um ou mais caminhos rastreados em vez de escrever ou deletar através deles. Claude Code pula um caminho quando:

* é, ou se tornou, um symlink, hard link ou outro arquivo não-regular
* seu diretório mudou desde o checkpoint
* seu backup não pode ser lido com segurança

Caminhos pulados mantêm seu conteúdo atual. Antes da v2.1.216, `/rewind` escrevia e deletava através de links em caminhos rastreados e não relatava uma restauração parcial.

```text theme={null}
Restored the code, but skipped 2 files: the tracked path is (or became) a link or other non-regular file, its directory changed since the checkpoint, or its backup could not be safely read. Skipped files were left untouched — run with --debug for the paths.
```

**O que fazer:**

* Identifique quais arquivos foram pulados para que você possa lidar com cada um com as etapas abaixo. A mensagem fornece apenas uma contagem; o log de debug em `~/.claude/debug/<session-id>.txt` nomeia cada caminho pulado conforme a restauração é executada, então ative o log de debug com `/debug` antes de sua próxima restauração. No macOS ou Linux, você pode encontrar os links diretamente: `find . -type l` para symlinks e `find . -type f -links +1` para arquivos hard-linked.
* Se um arquivo pulado é um link que você criou propositalmente, como um arquivo de configuração gerenciado por um gerenciador de dotfiles ou um arquivo hard-linked por ferramentas como pnpm, o rewind deixou seu conteúdo intacto. Para desfazer as alterações da sessão nele, peça a Claude para reverter a edição ou edite o arquivo você mesmo
* Se você não criou o link, inspecione o caminho antes de confiar em seu conteúdo: algo substituiu o arquivo após o checkpoint

<h3 id="no-files-were-restored">
  No files were restored
</h3>

Claude Code mostra esta mensagem quando você restaura código com [`/rewind`](/docs/pt/checkpointing) e não consegue restaurar nenhum dos arquivos naquele checkpoint. Para cada arquivo, ou o backup que Claude Code salvou antes de editá-lo está faltando, ou Claude Code não conseguiu escrever ou deletar o arquivo.

```text theme={null}
Failed to restore the code:
No files were restored: 1 file failed (backup missing, or the file could not be updated)
```

Claude Code deleta os backups de uma sessão na [retention sweep](/docs/pt/claude-directory#cleaned-up-automatically), por padrão cerca de 30 dias após a sessão salvar um pela última vez. Se você retomar uma sessão depois disso, `/rewind` ainda lista seus checkpoints, mas reverter para um deles pode falhar com este erro. Se a mensagem também disser `N paths were skipped for link safety`, veja [Restored the code, but skipped files](#restored-the-code-but-skipped-files) para esses caminhos.

Quando você faz fork de uma sessão, por exemplo com [`--fork-session`](/docs/pt/cli-reference#cli-flags) ou [`/branch`](/docs/pt/sessions#branch-a-session), Claude Code copia os backups da sessão original para o fork. Quando Claude Code não consegue copiar um backup, por exemplo porque o disco está cheio, esse backup está faltando no fork. Reverter para um checkpoint que precisa dele pode falhar com este erro.

**O que fazer:**

* Desfaça as alterações de outra forma: peça a Claude para reverter suas edições ou restaure os arquivos do controle de versão. Quando os backups se foram, executar `/rewind` novamente falha da mesma forma.
* Se Claude Code não conseguiu escrever ou deletar um arquivo, corrija o que bloqueia a escrita, como permissões de arquivo, então execute `/rewind` novamente.
* Para manter backups por mais tempo em futuras sessões, aumente [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays).

Antes da v2.1.260, Claude Code pulava silenciosamente arquivos cujos backups estavam faltando, e o rewind parecia ter sucesso.

<h2 id="session-saving-warnings">
  Avisos de salvamento de sessão
</h2>

Claude Code mostra estes avisos em uma linha persistente abaixo da caixa de entrada quando não está salvando sua transcrição de sessão. A sessão continua funcionando de qualquer forma; os avisos informam que a sessão pode estar faltando em [`--resume`](/docs/pt/sessions) mais tarde.

<h3 id="transcript-writes-are-failing">
  Falhas nas gravações de transcrição
</h3>

Claude Code salva a transcrição em disco conforme você trabalha, e suas gravações no [arquivo de transcrição](/docs/pt/sessions#where-transcripts-are-stored) estão falhando. A mensagem nomeia a causa com o código de erro subjacente, por exemplo um disco cheio:

```text theme={null}
Transcript writes are failing (disk full — ENOSPC) · recent messages may not be saved for resume
```

O aviso aparece em diferentes pontos dependendo do erro:

* Na primeira falha para condições que não se limpam por conta própria: um disco cheio, uma cota de disco excedida, um sistema de arquivos somente leitura, um caminho acima do limite de comprimento do sistema de arquivos, ou, em macOS e Linux, um erro de permissão
* Após falhas repetidas abrangendo pelo menos um minuto para tudo mais, incluindo erros de permissão no Windows, onde uma verificação de antivírus pode falhar em uma única gravação que depois é bem-sucedida na tentativa novamente

Antes da v2.1.217, Claude Code descartava as gravações com falha sem um aviso, e um `--resume` posterior faltando mensagens recentes era o primeiro sinal.

**O que fazer:**

* Corrija a condição que o código de erro nomeia: libere espaço em disco para `ENOSPC`; aumente ou limpe a cota para `EDQUOT`; restaure o acesso de escrita para o local da transcrição para `EACCES`, `EPERM`, ou `EROFS`
* O aviso se limpa por conta própria na próxima gravação bem-sucedida; nenhuma reinicialização é necessária
* Mensagens enviadas enquanto o aviso estava sendo exibido ainda podem estar faltando quando você retomar a sessão mais tarde

<h3 id="transcript-saving-is-off-skip-prompt-history">
  Salvamento de transcrição está desativado porque CLAUDE\_CODE\_SKIP\_PROMPT\_HISTORY está definido
</h3>

Esta sessão começou com [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/pt/env-vars) definido, então Claude Code não escreve nenhuma transcrição ou histórico de prompt para ela:

```text theme={null}
Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set · --resume will not find this session; if unintended, unset it and restart
```

A variável é uma exclusão intencional para sessões de script efêmeras, mas também pode chegar a uma sessão através de um perfil de shell, um script wrapper, ou um processo pai que a exportou.

**O que fazer:**

* Se você definiu a variável propositalmente, nenhuma ação é necessária; o aviso confirma que a sessão não aparecerá em `--resume`, `--continue`, ou histórico de seta para cima
* Se você não fez, remova a variável do shell ou script que inicia `claude`, depois inicie uma nova sessão. Mensagens da sessão atual não são salvas retroativamente.

<h3 id="transcript-saving-is-off-child-session-marker">
  Salvamento de transcrição está desativado por causa de um marcador CLAUDE\_CODE\_CHILD\_SESSION herdado
</h3>

Claude Code define [`CLAUDE_CODE_CHILD_SESSION`](/docs/pt/env-vars) nos subprocessos que ele gera, e trata uma sessão interativa que o herda como aninhada: Claude Code não salva nenhuma transcrição para ela, então sessões que o próprio Claude inicia não preenchem sua lista de `--resume`. Este aviso significa que sua sessão atual herdou o marcador:

```text theme={null}
Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker · restart with CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1 to keep future transcripts
```

O aviso é esperado quando você executou `claude` de dentro de outra sessão de Claude Code; ele sinaliza uma classificação incorreta quando o marcador vazou através de um intermediário de longa duração, por exemplo um terminal, sessão `screen`, ou inicializador que uma sessão de Claude Code originalmente iniciou.

Dentro do tmux, Claude Code detecta um marcador que chegou através do ambiente global do servidor tmux e continua salvando, então este aviso não aparece para esse caso.

**O que fazer:**

* Se você iniciou esta sessão de dentro de outra sessão de Claude Code propositalmente, nenhuma ação é necessária
* Se esta é uma sessão de nível superior, saia e reinicie com [`CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1`](/docs/pt/env-vars) definido. O salvamento se aplica a partir da reinicialização, então mensagens enviadas antes dela não são salvas.
* Para corrigir futuras inicializações do mesmo terminal ou inicializador, remova `CLAUDE_CODE_CHILD_SESSION` de seu ambiente

<h2 id="configuration-warnings">
  Avisos de configuração
</h2>

Claude Code escreve a maioria dessas mensagens em stderr, não na conversa, e escreve a maioria delas na inicialização. Uma entrada diz assim quando sua mensagem aparece em outro lugar, como no log de depuração ou como um aviso de inicialização na visualização de conversa, ou em outro momento, como a [linha de diagnóstico de modelo não reconhecido](#unrecognized-model-id-on-a-request) no momento da solicitação.

<h3 id="fullscreen-failed-start-notice">
  O renderizador de tela cheia não terminou de iniciar
</h3>

Uma sessão anterior de [tela cheia](/docs/pt/fullscreen) nesta máquina saiu antes de terminar de iniciar, então Claude Code inicia esta sessão no renderizador clássico e imprime um destes avisos:

```text theme={null}
Claude Code's fullscreen renderer didn't finish starting last time on this machine, so this launch is using the classic renderer. It will try fullscreen again next launch; /tui default keeps the classic renderer.

Claude Code's fullscreen renderer has repeatedly failed to start on this machine, so it has been turned off here. Run /tui fullscreen to try it again (this also resets after an update).
```

**O que fazer:**

* Siga [Renderização de tela cheia](/docs/pt/fullscreen#fullscreen-renderer-didnt-finish-starting). Ele diz qual aviso você recebe, o que Claude Code faz em sessões posteriores e como tentar tela cheia novamente ou manter o renderizador clássico.
* Se a sessão que falhou imprimiu uma mensagem de saída, consulte [Claude Code saiu após um erro de interface irrecuperável](#exited-after-an-unrecoverable-interface-error) para ver o que ela nomeia.

Antes da v2.1.236, Claude Code não imprimia aviso e continuava iniciando sessões em renderização de tela cheia após uma falha de inicialização.

<h3 id="exited-after-an-unrecoverable-interface-error">
  Claude Code saiu após um erro de interface irrecuperável
</h3>

Claude Code imprime esta mensagem quando sai porque sua interface de terminal atingiu um erro do qual não pode se recuperar, em qualquer renderizador. A segunda frase aparece apenas quando o erro aconteceu enquanto o renderizador de [tela cheia](/docs/pt/fullscreen) estava iniciando:

```text theme={null}
Claude Code exited after an unrecoverable interface error (<error>). It happened while the fullscreen renderer was starting, so the next launch will use the classic renderer (CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1 forces that any time).
```

**O que fazer:**

* Inicie Claude Code novamente. Para retomar a conversa, execute `claude --resume` no mesmo diretório.
* Se a mensagem nomear o renderizador de tela cheia, [Renderização de tela cheia](/docs/pt/fullscreen#fullscreen-renderer-didnt-finish-starting) diz o que o próximo lançamento faz, o que depende de como você ativou tela cheia e como tentar tela cheia novamente ou manter o renderizador clássico.

Antes da v2.1.236, Claude Code saía sem imprimir uma mensagem após este tipo de erro.

<h3 id="agent-descriptions-are-over-the-15000-token-limit">
  As descrições do agente excedem o limite de 15.0k tokens
</h3>

Claude Code mostra este aviso como um aviso de inicialização na visualização de conversa em vez de em stderr. As descrições combinadas de seus [subagentes](/docs/pt/sub-agents), exceto os integrados, excedem 15.000 tokens conforme Claude Code as estima. Cada agente conta seu nome mais seu frontmatter `description`. Claude Code carrega cada agente independentemente de o total estar acima do limite, então o aviso não muda o que é carregado.

```text theme={null}
Agent descriptions are over the 15.0k-token limit (~16.2k tokens) · ask Claude to trim agent descriptions in .claude/agents/
```

**O que fazer:**

* Encurte o frontmatter `description` de seus arquivos de agente, ou peça a Claude para aparar para você.
* Remova arquivos de agente que você não usa mais.

<h3 id="workspace-has-not-been-trusted">
  O espaço de trabalho não foi confiável
</h3>

Claude Code encontrou regras `permissions.allow` ou entradas `permissions.additionalDirectories` no `.claude/settings.json` ou `.claude/settings.local.json` do projeto e não as aplicou, porque [as regras de permissão do projeto requerem confiança do espaço de trabalho](/docs/pt/permissions#project-allow-rules-and-workspace-trust). A contagem, o nome da configuração e o arquivo nomeado na mensagem variam com sua configuração. As regras `deny` e `ask` não são afetadas.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**O que fazer:**

* Execute `claude` no diretório e aceite a caixa de diálogo de confiança. [Regras de permissão do projeto e confiança do espaço de trabalho](/docs/pt/permissions#project-allow-rules-and-workspace-trust) diz qual pasta essa aceitação cobre.
* No [modo não interativo](/docs/pt/headless) com `-p`, nenhuma caixa de diálogo é mostrada. Defina a entrada `hasTrustDialogAccepted` em `~/.claude.json` usando a chave `projects` exata que a mensagem imprime.
* Se a mensagem nomear `.claude/settings.local.json` e você iniciou Claude Code fora de um repositório git ou em seu diretório inicial, atualize para v2.1.200 ou posterior. As versões 2.1.196 a 2.1.199 tratavam seu próprio `.claude/settings.local.json` como fornecido pelo repositório nesses espaços de trabalho. Na v2.1.207 e posterior, atualizar não é suficiente fora de um repositório git se você não confiou na pasta: determinar que uma pasta não está dentro de um repositório executa git, e Claude Code executa essa verificação apenas depois que você aceita a caixa de diálogo de confiança, então use a primeira etapa. Seu diretório inicial e qualquer outro [diretório inicial de configuração](/docs/pt/permissions#project-allow-rules-and-workspace-trust) estão isentos e não esperam pela caixa de diálogo. Consulte [Regras de permissão do projeto e confiança do espaço de trabalho](/docs/pt/permissions#project-allow-rules-and-workspace-trust).

<h3 id="working-directory-is-a-network-path">
  O diretório de trabalho é um caminho de rede
</h3>

Claude Code não adiciona caminhos de rede como diretórios de trabalho. Procurar um caminho de rede pode entrar em contato com o host que ele nomeia, e no Windows esse contato pode enviar suas credenciais ao host, então Claude Code recusa o caminho sem procurá-lo. Você vê esta mensagem quando executa `/add-dir` com tal caminho, ou como um aviso na inicialização. Quando aparece na inicialização, Claude Code inicia sem esse diretório.

```text theme={null}
\\server\share is a network path, which cannot be added as a working directory. On Windows, map the share to a drive letter and pass it at launch with --add-dir (a drive letter added mid-session does not yet carry remote-read trust).
```

Os caminhos que Claude Code recusa desta forma incluem:

* Compartilhamentos UNC como `\\server\share`
* Caminhos de montagem automática como `/net/<host>`, a menos que você tenha iniciado Claude Code de um diretório sob a montagem automática desse host
* Caminhos locais que alcançam um local de rede através de um link simbólico ou junção

Letras de unidade mapeadas e caminhos `\\wsl$` não contam como caminhos de rede.

**O que fazer:**

* No Windows, mapeie o compartilhamento para uma letra de unidade, por exemplo com `net use Z: \\server\share`, e passe a unidade na inicialização com `claude --add-dir Z:\`.
* No macOS ou Linux, monte o compartilhamento em um caminho local e adicione esse caminho.
* Se o caminho estiver em `permissions.additionalDirectories`, remova-o do arquivo de configurações que o lista.

Antes da v2.1.257, Claude Code aceitava um caminho de rede acessível como diretório de trabalho.

<h3 id="remote-managed-settings-failed-to-load">
  Falha ao carregar configurações gerenciadas remotamente
</h3>

Sua sessão é elegível para [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings), mas Claude Code não conseguiu buscá-las, então mostra este aviso em sessões interativas. A causa entre parênteses nomeia o que falhou, como `network error`, `request timed out` ou `authentication rejected (401)`, e o resto da linha diz qual política a sessão executa:

* **Configurações em cache de uma busca anterior bem-sucedida**: Claude Code executa a sessão nessa política em cache, exceto as [variáveis de ambiente retidas](/docs/pt/server-managed-settings#fetch-and-caching-behavior), e a linha lê `using cached policy`.
* **Sem cache**: Claude Code executa a sessão sem configurações gerenciadas pelo servidor, e a linha lê `no remote policy applied`.

**O que fazer:**

* Aja sobre a causa que a mensagem nomeia: para uma causa de rede, verifique se esta máquina pode alcançar `api.anthropic.com`; para uma causa de autenticação, verifique seu login com `/status`
* Execute `/status` ou `claude doctor` para o diagnóstico completo

Antes da v2.1.248, Claude Code relatava uma busca de configurações falhada apenas no log de depuração.

<h3 id="managed-settings-were-not-approved">
  As configurações gerenciadas não foram aprovadas
</h3>

As [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) de sua organização incluem configurações que precisam de sua aprovação, e você recusou a [caixa de diálogo de aprovação de segurança](/docs/pt/server-managed-settings#security-approval-dialogs), então Claude Code sai sem aplicá-las:

```text theme={null}
Managed settings were not approved; exiting without applying them.
```

**O que fazer:**

* Inicie Claude Code novamente e aprove a caixa de diálogo para continuar sob as configurações de sua organização. Uma caixa de diálogo recusada não é lembrada, então aparece novamente na próxima inicialização.
* Se você não tiver certeza sobre uma configuração que a caixa de diálogo lista, pergunte a quem mantém as configurações gerenciadas de sua organização antes de aprovar

<h3 id="mcp-server-is-blocked-by-enterprise-managed-policy">
  O servidor MCP é bloqueado pela política gerenciada da empresa
</h3>

Você selecionou **Reconnect** em um servidor em `/mcp`, ou ativou um servidor desabilitado novamente lá, e uma configuração que [restringe servidores MCP](/docs/pt/managed-mcp) bloqueia esse servidor. Claude Code recusa conectá-lo e mostra:

```text theme={null}
MCP server <name> is blocked by enterprise managed policy
```

Qualquer uma dessas configurações pode produzir a mensagem:

* Uma entrada [`deniedMcpServers`](/docs/pt/managed-mcp#policy-based-control-with-allowlists-and-denylists) que corresponde ao servidor, incluindo uma em seu próprio `~/.claude/settings.json` ou no `.claude/settings.json` do projeto
* Uma lista [`allowedMcpServers`](/docs/pt/managed-mcp#policy-based-control-with-allowlists-and-denylists) que o servidor não corresponde
* [`strictPluginOnlyCustomization`](/docs/pt/settings-reference#strictpluginonlycustomization) com `mcp` bloqueado, que bloqueia servidores configurados em `~/.claude.json` e `.mcp.json`
* [`disableClaudeAiConnectors`](/docs/pt/mcp#disable-claude-ai-connectors), quando o servidor é um conector claude.ai

**O que fazer:**

* Verifique seus próprios arquivos de configurações de usuário e projeto para uma dessas configurações e altere ou remova-a
* Se nenhuma de suas próprias configurações explicar o bloqueio, pergunte ao seu administrador qual configuração gerenciada bloqueia o servidor

Antes da v2.1.257, **Reconnect** e re-habilitar em `/mcp` poderiam conectar um servidor que uma atualização de política no meio da sessão bloqueou.

<h3 id="managed-settings-document-could-not-be-parsed">
  O documento de configurações gerenciadas não pôde ser analisado
</h3>

Sua organização implanta [configurações gerenciadas](/docs/pt/managed-settings), e um dos documentos implantados está presente mas não pode ser analisado como um objeto JSON, então Claude Code sai com código 1 na inicialização em vez de executar sem a política que o documento carrega. A linha nomeia a fonte falhada antes da mensagem:

```text theme={null}
/Library/Application Support/ClaudeCode/managed-settings.json: Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.
```

A fonte é uma de:

* O caminho do arquivo `managed-settings.json` ou um arquivo drop-in sob `managed-settings.d`
* O perfil de preferências gerenciadas do macOS, `per-user managed preferences` ou `device-level managed preferences`
* O valor do registro do Windows, `Registry: HKLM\SOFTWARE\Policies\ClaudeCode\Settings`

[Encontre entradas que Claude Code descartou](/docs/pt/managed-settings#find-entries-claude-code-dropped) lista o que torna cada fonte não analisável.

Claude Code recusa iniciar mesmo quando outra fonte de administrador entrega uma política válida. Você vê este erro em sessões interativas, `claude -p`, sessões do Agent SDK, [sessões em segundo plano](/docs/pt/agent-view) e a maioria dos subcomandos, `claude doctor` incluído. A recusa falha fechada propositalmente: as configurações em um documento que Claude Code não pode analisar não podem ser aplicadas, e iniciar mesmo assim executaria sessões sem os controles da organização.

Um problema de esquema em um documento analisável não produz este erro. [Encontre entradas que Claude Code descartou](/docs/pt/managed-settings#find-entries-claude-code-dropped) cobre o que Claude Code faz com um.

Quando um diretório `managed-settings.d/` existe mas não pode ser listado, Claude Code relata `Managed settings drop-in directory could not be read:` seguido pelo erro subjacente. [Encontre entradas que Claude Code descartou](/docs/pt/managed-settings#find-entries-claude-code-dropped) cobre quando uma falha de leitura sai na inicialização.

**O que fazer:**

* Se você administra a máquina, corrija o documento nomeado para que seja analisado como um objeto JSON, ou remova o arquivo, perfil ou valor do registro. Um `managed-settings.json` vazio conta como `{}` e não bloqueia o lançamento.
* Se não, peça ao seu administrador para corrigir o documento implantado. Nada em seus próprios arquivos de configurações causa ou limpa este erro.

<h3 id="otelheadershelper-failed">
  otelHeadersHelper falhou
</h3>

Claude Code mostra este aviso como uma notificação na interface do terminal, uma vez por sessão interativa, quando o script [`otelHeadersHelper`](/docs/pt/settings-reference#otelheadershelper) falha ou imprime saída que não atende aos [requisitos do script](/docs/pt/monitoring-usage#script-requirements).

Enquanto o script continua falhando, as exportações falham e seu backend de telemetria não recebe nada da sessão.

O texto após `See /status:` diz o que falhou, como o código de saída do script seguido por sua saída de erro:

```text theme={null}
otelHeadersHelper failed; telemetry is not being exported. See /status: exited 1: token service unreachable
```

**O que fazer:**

* Execute `/status` para ler o detalhe da falha.
* Corrija o script para que saia com 0 em 30 segundos e imprima um objeto JSON de valores de cabeçalho de string em stdout. Consulte [requisitos do script](/docs/pt/monitoring-usage#script-requirements).
* Se sua organização implanta o script através de [configurações gerenciadas](/docs/pt/managed-settings), peça a quem as mantém para corrigi-lo.

No [modo não interativo](/docs/pt/headless) com `-p`, a mesma falha aparece em stderr como `otelHeadersHelper failed (OpenTelemetry export headers unavailable): <error>` em vez disso.

<h3 id="headershelper-not-run">
  headersHelper não executado
</h3>

Claude Code conectou um servidor MCP com seus `headers` estáticos sozinhos e pulou o [`headersHelper`](/docs/pt/mcp#use-dynamic-headers-for-custom-authentication) do servidor, porque o auxiliar é um comando shell e a pasta não tem confiança salva. Uma pasta obtém confiança salva quando você define sua entrada em `~/.claude.json` manualmente ou, fora de seu diretório inicial, quando você aceita a caixa de diálogo de confiança para ela em uma sessão interativa. Consulte [Confie em uma pasta antes de seu headersHelper ser executado](/docs/pt/mcp#trust-a-folder-before-its-headershelper-runs) para ver quais servidores essa verificação se aplica.

Claude Code escreve esta linha no [modo não interativo](/docs/pt/headless) apenas, uma vez por servidor. Em uma sessão interativa, escreve a mesma recusa no log de depuração.

```text theme={null}
MCP server 'internal-api': headersHelper not run — this workspace has no persisted trust; accept the trust dialog here once interactively, or set projects["/Users/you/project"].hasTrustDialogAccepted in /Users/you/.claude.json.
```

A chave `projects` que a mensagem imprime é a pasta [Regras de permissão do projeto e confiança do espaço de trabalho](/docs/pt/permissions#project-allow-rules-and-workspace-trust) diz que Claude Code a chave na confiança. Aceitar a caixa de diálogo de confiança para uma pasta pai não satisfaz a verificação, e uma sessão `-p` ou SDK também não satisfaz.

**O que fazer:**

* Execute `claude` na pasta que a mensagem nomeia, aceite a caixa de diálogo de confiança, então execute seu comando `-p` ou SDK novamente
* Defina a entrada `hasTrustDialogAccepted` em `~/.claude.json` você mesmo, usando a chave `projects` exata que a mensagem imprime
* Se você iniciou a sessão em seu diretório inicial, trabalhe a partir de um diretório de projeto que você confiou. Quando você aceita a caixa de diálogo de confiança em seu diretório inicial, Claude Code mantém essa confiança apenas para a sessão atual.

<h3 id="malformed-tool-content-rule">
  Regra Tool(content) malformada
</h3>

Uma [regra de permissão](/docs/pt/permissions#permission-rule-syntax) em um de seus arquivos de configurações não tem a forma `Tool` ou `Tool(content)`, por exemplo porque o texto segue o parêntese de fechamento ou um dos parênteses está faltando. Claude Code pula a regra e a lista na caixa de diálogo de configurações inválidas quando uma sessão interativa inicia, e na saída de [`claude doctor`](/docs/pt/debug-your-config#check-resolved-settings):

```text theme={null}
Invalid permission rule "Bash(ls) x" was skipped: Malformed Tool(content) rule. Rules take the form Tool or Tool(content) and must end at the closing ")"; parentheses inside the content are literal
```

**O que fazer:**

* No arquivo de configurações listado com a mensagem, reescreva a regra para que termine em seu parêntese de fechamento, por exemplo `Bash(ls *)` em vez de `Bash(ls) x`
* Deixe os parênteses dentro do conteúdo como estão. Eles são literais, então uma regra como `Edit(./Finance (2024)/**)` é válida sem escape

Antes da v2.1.260, Claude Code relatava uma regra com parênteses não correspondentes como `Mismatched parentheses`.

<h3 id="is-not-matched-by-file-permission-checks">
  Não é correspondido pelas verificações de permissão de arquivo
</h3>

Claude Code encontrou uma regra `Write`, `NotebookEdit`, `MultiEdit` ou `Glob` [permissão](/docs/pt/permissions#read-and-edit) com um caminho em um de seus [arquivos de configurações](/docs/pt/settings#where-settings-live), em [configurações gerenciadas](/docs/pt/managed-settings), ou em um valor de flag `--allowedTools`, `--disallowedTools` ou `--settings`. Ele verifica permissões de arquivo apenas contra regras `Edit` e `Read`, então nunca consulta uma regra de caminho que nomeia uma das outras ferramentas de arquivo. Ele mantém a regra e não muda mais nada; o aviso nomeia a regra, sua fonte entre parênteses e a substituição a escrever:

```text theme={null}
Permission deny rule (.claude/settings.json): Write(docs/**) is not matched by file permission checks — only Edit(path) rules are. Use Edit(docs/**) instead (Edit rules cover all file-editing tools).
```

**O que fazer:**

* Substitua as regras `Write(path)`, `NotebookEdit(path)` e `MultiEdit(path)` legadas por `Edit(path)`. As regras `Edit` cobrem todas as ferramentas de edição de arquivo.
* Exceto em `--allowedTools`, onde Claude Code aceita uma regra `Glob` sem aviso, substitua as regras `Glob(path)` por `Read(path)`.
* Corrija a regra na fonte que o aviso nomeia entre parênteses: um caminho de arquivo de configurações, ou o próprio flag para `--allowed-tools` e `--disallowed-tools`. Um caminho `claude-settings-<hash>.json` que não existe no disco representa um valor `--settings` inline. Corrija o JSON que você passa para esse flag.
* Deixe as regras de nome de ferramenta simples como `Write` ou `Glob` sozinhas. Claude Code as corresponde no [nível de ferramenta](/docs/pt/permissions#match-all-uses-of-a-tool) e não avisa sobre elas.
* Se a fonte lê `managed policy settings`, encaminhe o aviso a quem mantém suas configurações gerenciadas, já que você não pode limpá-lo você mesmo.

Em uma [sessão em segundo plano](/docs/pt/agent-view) ou com `--output-format json` ou `stream-json`, Claude Code escreve o aviso no log de depuração em vez de stderr, então a saída lida por máquina fica limpa. Execute com `--debug` para capturá-lo em `~/.claude/debug/<session-id>.txt`. Antes da v2.1.210, Claude Code aceitava essas regras sem aviso.

<h3 id="has-a-wildcard-before-the-rest-of-the-command">
  Tem um curinga antes do resto do comando
</h3>

Claude Code encontrou uma regra de permissão `Bash` allow cujo `*` vem antes de uma palavra posterior que determina qual comando é, como `Bash(git * main)` ou `Bash(git -C * status *)`, em um de seus [arquivos de configurações](/docs/pt/settings#where-settings-live), em [configurações gerenciadas](/docs/pt/managed-settings), ou em um valor de flag `--allowedTools` ou `--settings`. O `*` corresponde a qualquer texto, incluindo opções inseridas nessa posição: `Bash(git * main)` também aprova `git -c core.fsmonitor=<script> diff main`, onde `-c` faz git executar um programa que o comando nomeia. [Padrões de curinga](/docs/pt/permissions#wildcard-patterns) mostra as regras de correspondência.

O aviso existe para que você possa estreitar uma regra cujo curinga é mais amplo do que você pretendia. Claude Code mantém a regra e não muda nada sobre como ela corresponde; o aviso nomeia a regra e sua fonte entre parênteses:

```text theme={null}
Permission allow rule (.claude/settings.json): Bash(git -C * status *) has a wildcard before the rest of the command, so it also matches any options inserted at that position and approves them without a prompt. For git, options such as -c and --exec-path can run arbitrary commands. Replace that * with the exact value you mean, or only use * after the subcommand (for example Bash(git status *)).
```

**O que fazer:**

* Substitua o `*` antes do subcomando pelo valor exato que você quer dizer: `Bash(git checkout main)` em vez de `Bash(git * main)`.
* Mova cada `*` após o subcomando: `Bash(git status *)` em vez de `Bash(git -C * status *)`. Escreva uma regra por subcomando que você quer permitir.
* Corrija a regra na fonte que o aviso nomeia entre parênteses: um caminho de arquivo de configurações, ou o próprio flag `--allowed-tools`. Um caminho `claude-settings-<hash>.json` que não existe no disco representa um valor `--settings` inline. Corrija o JSON que você passa para esse flag.
* Se a fonte lê `managed policy settings`, encaminhe o aviso a quem mantém suas configurações gerenciadas, já que você não pode limpá-lo você mesmo.

Claude Code não avisa sobre regras de negação e pergunta com a mesma forma: ele recusa ou solicita os comandos extras que eles correspondem em vez de aprová-los. Também não avisa sobre regras cujo subcomando vem antes do primeiro `*`, como `Bash(git commit *)`, ou regras em que nenhuma palavra além de uma opção segue o `*`, como `Bash(git *)`, ou sobre regras de prefixo `:*` como `Bash(git:*)`.

Em uma [sessão em segundo plano](/docs/pt/agent-view) ou com `--output-format json` ou `stream-json`, Claude Code escreve o aviso no log de depuração em vez de stderr, então a saída lida por máquina fica limpa. Execute com `--debug` para capturá-lo em `~/.claude/debug/<session-id>.txt`. Antes da v2.1.246, Claude Code aceitava essas regras sem aviso.

<h3 id="crosssessioninbound-must-be-one-of-accept-hold-refuse">
  crossSessionInbound deve ser um de accept, hold, refuse
</h3>

Um arquivo de configurações define [`crossSessionInbound`](/docs/pt/settings-reference#crosssessioninbound) para um valor que Claude Code não reconhece, como o erro de digitação `"reject"`. A segunda frase do aviso depende de qual arquivo contém o valor; em um arquivo de usuário, projeto, local ou `--settings`, lê:

```text theme={null}
"crossSessionInbound" must be one of "accept", "hold", "refuse"; received "reject". This value was ignored; while it is present, cross-session messages are held for your approval instead of being delivered. Set it to one of the values above.
```

Em [configurações gerenciadas](/docs/pt/managed-settings), Claude Code trata o valor não reconhecido como `refuse`, o valor mais restritivo, e o aviso diz que as mensagens entre sessões são rejeitadas até que um administrador corrija. Para como a retenção se combina com valores em seus outros arquivos de configurações, consulte [`crossSessionInbound`](/docs/pt/settings-reference#crosssessioninbound).

**O que fazer:**

* Defina a chave para `"accept"`, `"hold"` ou `"refuse"`, ou remova-a
* Quando o aviso nomear configurações gerenciadas, peça ao administrador para corrigir o valor

Antes da v2.1.248, Claude Code ignorava um valor não reconhecido sem aviso.

<h3 id="the-200k-limit-isnt-enforced">
  O limite de 200K não é aplicado
</h3>

Você definiu [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/pt/env-vars), que normalmente faz [auto-compactação](/docs/pt/model-config#default-auto-compact-thresholds) manter sessões em modelos de contexto 1M em uma janela de 200K, mas nenhum limite de compactação limita esta sessão em ou abaixo de 200K, então a conversa pode crescer além disso.

```text theme={null}
CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced for <model>, so this session can grow past it. To enforce it, set CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000 (or the autoCompactWindow setting).
```

Claude Code aplica o limite de 200K por conta própria para cada modelo que reconhece como tendo uma janela nativa de 1M, e para IDs de modelo que não reconhece, ele compacta na janela que assume. O aviso aparece quando outra configuração derrota essa aplicação:

* O ID do modelo não é um que Claude Code reconhece, como um alias de [gateway LLM](/docs/pt/llm-gateway), e você definiu [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/pt/env-vars) ou aumentou a janela assumida além de 200K com [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/pt/env-vars). Neste caso, a mensagem também oferece `or update to a Claude Code version that recognizes <model>` como um remédio.
* Um beta `context-1m` solicitado através de [`ANTHROPIC_BETAS`](/docs/pt/env-vars) ou o flag [`--betas`](/docs/pt/cli-reference#cli-flags) ainda pede à API a janela de 1M em um modelo que aceita esse beta, enquanto nada compacta a sessão em 200K

**O que fazer:**

* Defina [`CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000`](/docs/pt/env-vars), ou a configuração [`autoCompactWindow`](/docs/pt/settings-reference#autocompactwindow) para `200000`, para que a auto-compactação compacte no limite de 200K
* Se a mensagem nomear um ID de modelo que esta versão não reconhece, execute `claude update`. Uma versão que reconhece o ID como um modelo de contexto 1M aplica o limite sem configuração adicional.
* Se você quer que a sessão use a janela completa do modelo, desdefina `CLAUDE_CODE_DISABLE_1M_CONTEXT`; o aviso relata apenas que o limite de 200K não é aplicado

Em uma [sessão em segundo plano](/docs/pt/agent-view) ou com `--output-format json` ou `stream-json`, Claude Code escreve o aviso no log de depuração em vez de stderr.

<h3 id="unrecognized-model-id-on-a-request">
  ID de modelo não reconhecido em uma solicitação
</h3>

Claude Code enviou uma solicitação para um ID de modelo que sua versão de Claude Code não reconhece, e não encontrou nenhuma entrada [`modelOverrides`](/docs/pt/model-config#override-model-ids-per-version) que mapeie esse ID para um modelo que reconhece. Claude Code ainda envia a solicitação com o ID conforme você o configurou, e não sai ou muda de modelos.

```text theme={null}
[claude-code:unrecognized_model] {"model":"my-proxy-model","query_source":"sdk"}
```

Em um script ou arnês que lê stderr, corresponda ao prefixo `[claude-code:unrecognized_model]`. Após o prefixo e um espaço, Claude Code escreve um objeto JSON de uma linha. Claude Code pode adicionar campos a ele em uma versão posterior, então ignore qualquer campo que você não espera. Ele escreve pelo menos estes dois:

* `model`: a string do modelo conforme você a configurou
* `query_source`: o caminho da solicitação que usou o modelo. Claude Code relata `sdk` para uma execução `-p` e um valor que começa com `agent:` para um subagente.

Claude Code escreve a linha em um de dois lugares, dependendo de como você a executa:

* No [modo não interativo](/docs/pt/headless) com `-p`, Claude Code escreve em stderr sob cada `--output-format`, para que você possa analisar stdout sem filtrar a linha
* Em uma sessão interativa ou uma [sessão em segundo plano](/docs/pt/agent-view), Claude Code escreve no log de depuração; execute com `--debug` para capturá-lo em `~/.claude/debug/<session-id>.txt`

Claude Code escreve a linha uma vez por string de modelo por processo. Escreve uma linha separada para cada ID não reconhecido adicional, como um que um [subagente](/docs/pt/sub-agents#choose-a-model) ou [funcionalidade em segundo plano](/docs/pt/costs#background-token-usage) usa.

Claude Code não escreve a linha para IDs de provedor que resolve para um modelo que reconhece, como IDs Amazon Bedrock `us.anthropic.claude-...`, IDs da plataforma de agentes do Google Cloud com um sufixo de versão `@`, e nomes de implantação do Microsoft Foundry que contêm um ID de modelo Claude. Claude Code verifica o modelo atrás de um [ARN de perfil de inferência de aplicação](/docs/pt/amazon-bedrock#map-each-model-version-to-an-inference-profile) do Amazon Bedrock em vez do próprio ARN. Escreve nenhuma linha para um ARN que não pode resolver, como um digitado incorretamente.

**O que fazer:**

* Se você definiu o ID propositalmente, como um alias de [gateway LLM](/docs/pt/llm-gateway), adicione uma entrada [`modelOverrides`](/docs/pt/model-config#override-model-ids-per-version) ao seu [arquivo de configurações](/docs/pt/settings#where-settings-live) com o ID como seu valor. Use um ID de modelo Anthropic como a chave, não um alias de família como `opus`. Para `my-proxy-model` da linha de exemplo, adicione esta entrada:

  ```json theme={null}
  {
    "modelOverrides": {
      "claude-opus-4-6": "my-proxy-model"
    }
  }
  ```

  Claude Code então trata `my-proxy-model` como `claude-opus-4-6` e para de escrever a linha.

* Se o ID nomeia um modelo mais novo que sua versão de Claude Code, execute `claude update`

* Se o ID é um erro de digitação, corrija-o em qualquer um dos [lugares onde você pode definir um modelo](/docs/pt/model-config#setting-your-model) ou [variáveis de alias](/docs/pt/model-config#environment-variables) que o contém. Se `query_source` começa com `agent:`, corrija-o onde você define o [modelo do subagente](/docs/pt/sub-agents#choose-a-model).

Antes da v2.1.233, Claude Code não escrevia nenhuma linha quando enviava uma solicitação para um ID de modelo que não reconhecia.

<h3 id="stale-sandbox-mask-files-left-by-a-killed-session">
  Arquivos de máscara de sandbox obsoletos deixados por uma sessão morta
</h3>

`claude doctor` imprime este aviso em seus diagnósticos, e `/status` lista a mesma linha. Aparece no Linux e WSL2 quando [sandboxing](/docs/pt/sandboxing) está habilitado com isolamento de sistema de arquivos ativado.

Enquanto um comando em sandbox é executado, a sandbox mantém uma negação de escrita em um arquivo que ainda não existe criando um espaço reservado de leitura 0-byte lá, e o remove depois. Uma sessão morta antes dessa limpeza ser executada, por exemplo por SIGKILL, deixa os espaços reservados para trás. Sessões posteriores os vinculam novamente como somente leitura em cada inicialização, então uma escrita de configurações como salvar "Sim, e não pergunte novamente" falha onde um fica.

```text theme={null}
- Stale sandbox mask files left by a killed session: /home/you/project/.claude/settings.local.json
  Fix: Remove each with `rm <path>` while no other Claude Code session is running in that project — a 0-byte read-only file where a settings file belongs makes "Yes, and don't ask again" fail to save, and the sandbox binds it read-only again on every start
```

**O que fazer:**

* Saia de qualquer outra sessão de Claude Code em execução nesse projeto, depois delete cada arquivo listado com `rm`. O aviso nomeia até três arquivos e conta o resto, então execute `claude doctor` novamente após deletar até que o aviso não apareça mais. Um espaço reservado que a sandbox de outra sessão ainda está usando é uma parte viva da proteção de escrita dessa sessão.
* Se uma escolha de permissão que você salvou com "Sim, e não pergunte novamente" não ficou, salve-a novamente após deletar o espaço reservado.

Antes da v2.1.257, `claude doctor` não sinalizava esses arquivos; versões anteriores deixam os mesmos espaços reservados para trás quando uma sessão é morta.

<h2 id="responses-seem-lower-quality-than-usual">
  As respostas parecem ter qualidade inferior ao usual
</h2>

Se as respostas do Claude parecerem menos capazes do que você espera, mas nenhum erro for exibido, a causa geralmente é o estado da conversa em vez do modelo em si. Claude Code não muda silenciosamente versões de modelo. Ele pode mudar para um modelo de fallback em três casos específicos:

* Um [`--fallback-model`](/docs/pt/cli-reference#cli-flags) configurado assume o controle após um erro de disponibilidade, apenas para esse turno, com um aviso na transcrição
* Uma verificação de inicialização do Amazon Bedrock ou da Agent Platform do Google Cloud encontra seu modelo padrão indisponível
* [Fallback automático de modelo](/docs/pt/model-config#automatic-model-fallback) no Fable 5.1, Fable 5, Opus 5.5 e Opus 5 move a sessão para o modelo de fallback da categoria sinalizada, quando essa categoria tem um, e mostra um aviso na transcrição

A verificação de seleção de modelo abaixo captura o segundo e terceiro casos; o primeiro aparece como um aviso de transcrição em vez de uma mudança de `/model`. [Configuração de modelo](/docs/pt/model-config) explica quando cada fallback se aplica.

Verifique estes primeiro:

* **Seleção de modelo**: execute `/model` para confirmar que você está no modelo que espera. Uma escolha anterior de `/model` ou uma variável de ambiente `ANTHROPIC_MODEL` pode ter você em um modelo menor do que pretendia.
* **Nível de esforço**: execute `/effort` para verificar o nível de raciocínio atual e aumentá-lo para depuração difícil ou trabalho de design. Os padrões variam por modelo, então verifique antes de assumir que você está abaixo do máximo. Veja [Ajustar nível de esforço](/docs/pt/model-config#adjust-effort-level) para padrões por modelo e o atalho `ultrathink`.
* **Pressão de contexto**: execute `/context` para ver o quão cheio está a janela. Se estiver próximo da capacidade, execute `/compact` em um ponto natural ou `/clear` para começar do zero. Veja [Explorar a janela de contexto](/docs/pt/context-window) para como auto-compact afeta turnos anteriores.
* **Instruções obsoletas**: arquivos `CLAUDE.md` grandes ou desatualizados e definições de ferramentas MCP consomem contexto e podem orientar respostas. A verificação `/doctor` sinaliza arquivos de memória superdimensionados e extensões não utilizadas, e `/context` mostra o uso de tokens de ferramentas MCP. Antes da v2.1.205, `/doctor` abria uma tela de diagnósticos que sinalizava arquivos de memória superdimensionados e definições de subagente.

Quando uma resposta sai errada, retroceder geralmente funciona melhor do que responder com correções. Pressione Esc duas vezes ou execute `/rewind` para voltar antes do turno ruim, depois reformule o prompt com mais especificidades. Corrigir na thread mantém a tentativa errada no contexto, o que pode ancorar respostas posteriores a ela. Veja [Checkpointing](/docs/pt/checkpointing).

Se a qualidade ainda parecer inadequada após verificar o acima, execute `/feedback` e descreva o que você esperava versus o que obteve. O feedback enviado desta forma inclui a transcrição da conversa, que é a forma mais rápida para a Anthropic diagnosticar uma regressão real. Veja [Relatar um erro](#report-an-error) se `/feedback` não estiver disponível em seu ambiente.

Se Claude avisar sobre uma injeção de prompt suspeita, ou recusar uma solicitação por causa de uma injeção suspeita, e o texto que o aviso nomeia for contexto que Claude Code adiciona à conversa automaticamente em vez de conteúdo de arquivo ou web, execute `claude update` e tente novamente. Se o aviso se repetir após atualizar, [relate-o](#report-an-error) em vez de colar o conteúdo sinalizado de volta no prompt. Antes da v2.1.201, Sonnet 5 recusava algumas solicitações da mesma forma.

<h2 id="report-an-error">
  Relatar um erro
</h2>

Para erros de componentes que esta página não cobre, consulte o guia relevante:

* Servidor MCP falhou ao conectar ou autenticar: [MCP](/docs/pt/mcp)
* Script de hook falhou ou bloqueou uma ferramenta: [Debug hooks](/docs/pt/hooks#debug-hooks)
* Permissão negada ou erros do sistema de arquivos durante a instalação: [Solucionar problemas de instalação e login](/docs/pt/troubleshoot-install)

Se um erro não estiver listado aqui ou a correção sugerida não ajudar:

* Execute `/feedback` dentro do Claude Code para enviar a transcrição e uma descrição para a Anthropic. O comando também oferece abrir um problema do GitHub pré-preenchido. O envio para a Anthropic requer [autenticação](/docs/pt/authentication). No Amazon Bedrock, na plataforma de agentes do Google Cloud, no Microsoft Foundry e em outros provedores terceirizados, ou quando nenhuma credencial da Anthropic está configurada, `/feedback` salva um arquivo local que você pode enviar para seu representante de conta da Anthropic.
* Execute `claude doctor` do seu shell para um diagnóstico somente leitura da sua instalação, ou execute o checkup `/doctor` dentro do Claude Code para encontrar e corrigir problemas de configuração
* Verifique [status.claude.com](https://status.claude.com) para incidentes ativos
* Pesquise [problemas existentes](https://github.com/anthropics/claude-code/issues) no GitHub
