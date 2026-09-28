> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence des erreurs

> Consultez les messages d'erreur d'exécution de Claude Code avec leur signification et comment les corriger.

Cette page répertorie les erreurs d'exécution que Claude Code affiche et comment récupérer de chacune d'elles, plus ce qu'il faut vérifier lorsque les réponses semblent incorrectes sans erreur. Pour les erreurs d'installation telles que `command not found` ou les défaillances TLS lors de la configuration, consultez [Dépannage de l'installation et de la connexion](/docs/fr/troubleshoot-install).

À l'exception des [erreurs de wrapper et d'IDE](#wrapper-and-ide-errors), que le programme de lancement imprime plutôt que Claude Code lui-même, ces erreurs et commandes de récupération s'appliquent sur l'ensemble de l'interface CLI, de l'[application Desktop](/docs/fr/desktop) et des [sessions cloud](/docs/fr/claude-code-on-the-web), car les trois encapsulent le même CLI Claude Code. Pour les autres problèmes spécifiques à la surface, consultez la section dépannage sur la page de cette surface.

<Note>
  Claude Code appelle l'API Claude pour les réponses du modèle, donc la plupart des erreurs d'exécution correspondent à un code d'erreur API sous-jacent. Cette page couvre ce que chaque erreur signifie dans Claude Code et comment récupérer. Pour les définitions brutes du code de statut HTTP, consultez la [référence des erreurs de la plateforme Claude](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Trouvez votre erreur
</h2>

Faites correspondre le message que vous voyez à une section ci-dessous.

| Message                                                                                                                                                                                                                                                              | Section                                                                                                                        |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `API Error: 500 Internal server error`                                                                                                                                                                                                                               | [Erreurs serveur](#api-error-500-internal-server-error)                                                                        |
| `API Error: Repeated 529 Overloaded errors`                                                                                                                                                                                                                          | [Erreurs serveur](#api-error-repeated-529-overloaded-errors)                                                                   |
| `Request timed out`                                                                                                                                                                                                                                                  | [Erreurs serveur](#request-timed-out), ou [Réseau](#unable-to-connect-to-api) si le message mentionne votre connexion Internet |
| `API Error: No response from API`                                                                                                                                                                                                                                    | [Erreurs serveur](#no-response-from-api)                                                                                       |
| `Server error mid-response. The response above may be incomplete.`                                                                                                                                                                                                   | [Erreurs serveur](#the-response-above-may-be-incomplete)                                                                       |
| `Connection lost mid-response` / `Your computer went to sleep mid-response` / `The response stopped arriving`                                                                                                                                                        | [Erreurs serveur](#the-response-above-may-be-incomplete)                                                                       |
| `Connection closed mid-response` / `Response stalled mid-stream`                                                                                                                                                                                                     | [Erreurs serveur](#the-response-above-may-be-incomplete)                                                                       |
| `Connection lost before a response was produced` / `Your computer went to sleep before a response was produced` / `The response stalled before a response was produced`                                                                                              | [Tentatives automatiques](#automatic-retries)                                                                                  |
| `Connection closed while thinking` / `Response stalled while thinking`                                                                                                                                                                                               | [Tentatives automatiques](#automatic-retries)                                                                                  |
| `Connection lost while your computer was asleep`                                                                                                                                                                                                                     | [Tentatives automatiques](#automatic-retries)                                                                                  |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`                                                                                                                                                                                 | [Erreurs serveur](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Auto mode could not evaluate this action and is blocking it for safety`                                                                                                                                                                                             | [Erreurs serveur](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Auto mode classifier transcript exceeded context window`                                                                                                                                                                                                            | [Erreurs serveur](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Agent aborted: auto mode classifier request refused by the safety safeguard`                                                                                                                                                                                        | [Erreurs serveur](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `The server-side auto mode classifier gave no verdict`                                                                                                                                                                                                               | [Erreurs serveur](#the-server-returned-no-safety-verdict)                                                                      |
| `Auto mode is unavailable — the server returned no safety verdict for the last 10 responses`                                                                                                                                                                         | [Erreurs serveur](#the-server-returned-no-safety-verdict)                                                                      |
| `Agent terminated early due to an API error`                                                                                                                                                                                                                         | [Erreurs serveur](#agent-terminated-early-due-to-an-api-error)                                                                 |
| `You've hit your session limit` / `You've hit your weekly limit` / `You've hit your Opus limit` / `You've hit your Sonnet limit`                                                                                                                                     | [Limites d'utilisation](#youve-hit-your-session-limit)                                                                         |
| `Usage credits required for 1M context`                                                                                                                                                                                                                              | [Limites d'utilisation](#usage-credits-required-for-1m-context)                                                                |
| `the prompt to confirm went unanswered — nothing was sent`                                                                                                                                                                                                           | [Limites d'utilisation](#the-prompt-to-confirm-went-unanswered)                                                                |
| `Server is temporarily limiting requests`                                                                                                                                                                                                                            | [Limites d'utilisation](#server-is-temporarily-limiting-requests)                                                              |
| `Request rejected (429)`                                                                                                                                                                                                                                             | [Limites d'utilisation](#request-rejected-429)                                                                                 |
| `Credit balance is too low`                                                                                                                                                                                                                                          | [Limites d'utilisation](#credit-balance-is-too-low)                                                                            |
| `You've hit your monthly spend limit` / `You've hit your individual spend limit` / `You've hit your org's monthly spend limit` / `You've hit your channel's monthly spend limit` / `You've hit your team's shared budget` / `You've hit your individual usage limit` | [Limites d'utilisation](#youve-hit-your-monthly-spend-limit)                                                                   |
| `Could not update your spend limit`                                                                                                                                                                                                                                  | [Limites d'utilisation](#could-not-update-your-spend-limit)                                                                    |
| `spend limit reached` / `spend limit unavailable`                                                                                                                                                                                                                    | [Limites d'utilisation](#spend-limit-reached)                                                                                  |
| `Not logged in · Please run /login`                                                                                                                                                                                                                                  | [Authentification](#not-logged-in)                                                                                             |
| `Could not resolve authentication method`                                                                                                                                                                                                                            | [Authentification](#could-not-resolve-authentication-method)                                                                   |
| `Invalid API key`                                                                                                                                                                                                                                                    | [Authentification](#invalid-api-key)                                                                                           |
| `Your apiKeyHelper script is failing`                                                                                                                                                                                                                                | [Authentification](#your-apikeyhelper-script-is-failing)                                                                       |
| `Invalid auth token · Fix external auth token`                                                                                                                                                                                                                       | [Authentification](#invalid-request-header-value)                                                                              |
| `Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable`                                                                                                                                                                                                    | [Authentification](#invalid-request-header-value)                                                                              |
| `Invalid request header from the environment · Fix the environment variable`                                                                                                                                                                                         | [Authentification](#invalid-request-header-value)                                                                              |
| `This organization has been disabled`                                                                                                                                                                                                                                | [Authentification](#this-organization-has-been-disabled)                                                                       |
| `Your organization has disabled API key authentication`                                                                                                                                                                                                              | [Authentification](#your-organization-has-disabled-api-key-authentication)                                                     |
| `Your organization has disabled Claude subscription access`                                                                                                                                                                                                          | [Authentification](#your-organization-has-disabled-claude-subscription-access)                                                 |
| `Routines are disabled by your organization's policy`                                                                                                                                                                                                                | [Authentification](#routines-are-disabled-by-your-organizations-policy)                                                        |
| `Remote Control is only available when using Claude via api.anthropic.com`                                                                                                                                                                                           | [Authentification](#remote-control-requires-the-anthropic-api)                                                                 |
| `OAuth token refresh failed — run /login to re-authenticate`                                                                                                                                                                                                         | [Authentification](#remote-control-couldnt-refresh-your-login)                                                                 |
| `JWT refresh failed: no OAuth token — run /login`                                                                                                                                                                                                                    | [Authentification](#remote-control-couldnt-refresh-your-login)                                                                 |
| `Claude.ai login expired`                                                                                                                                                                                                                                            | [Authentification](#remote-control-couldnt-refresh-your-login)                                                                 |
| `Claude.ai login was rejected — run /login, then /remote-control`                                                                                                                                                                                                    | [Authentification](#remote-control-couldnt-refresh-your-login)                                                                 |
| `OAuth token unavailable — run /login to restore Remote Control`                                                                                                                                                                                                     | [Authentification](#remote-control-couldnt-refresh-your-login)                                                                 |
| `Signed out of Claude — run /login, then /remote-control`                                                                                                                                                                                                            | [Authentification](#remote-control-couldnt-refresh-your-login)                                                                 |
| `signed-in claude.ai account or organization changed on this machine`                                                                                                                                                                                                | [Authentification](#remote-control-stopped-because-the-signed-in-account-changed)                                              |
| `Remote Control stopped — the app running this session is now signed in to a different Claude account`                                                                                                                                                               | [Authentification](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)                |
| `Remote Control stopped — the app running this session is signed out of Claude`                                                                                                                                                                                      | [Authentification](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)                |
| `OAuth token revoked` / `OAuth token has expired`                                                                                                                                                                                                                    | [Authentification](#oauth-token-revoked-or-expired)                                                                            |
| `API Error: 401 Invalid authentication credentials`                                                                                                                                                                                                                  | [Authentification](#api-error-401-invalid-authentication-credentials)                                                          |
| `Login expired · Please run /login`                                                                                                                                                                                                                                  | [Authentification](#login-expired)                                                                                             |
| `Claude login not accepted · Run /login, then try again`                                                                                                                                                                                                             | [Authentification](#claude-login-not-accepted)                                                                                 |
| `Artifacts need a claude.ai login`                                                                                                                                                                                                                                   | [Authentification](#artifacts-need-a-claude-ai-login)                                                                          |
| `Not signed in to the Cloud gateway — run /login.`                                                                                                                                                                                                                   | [Authentification](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                     |
| `Administrator policy requires a Cloud gateway sign-in on this machine`                                                                                                                                                                                              | [Authentification](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                     |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                                                                                                                                                                                           | [Authentification](#login-expired)                                                                                             |
| `Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                            | [Authentification](#your-account-is-on-hold)                                                                                   |
| `Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                     | [Authentification](#your-account-is-on-hold)                                                                                   |
| `Anthropic profile login expired · Re-authenticate your Anthropic profile`                                                                                                                                                                                           | [Authentification](#anthropic-profile-login-expired)                                                                           |
| `Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile`                                                                                                                                                 | [Authentification](#anthropic-profile-login-expired)                                                                           |
| `does not meet scope requirement user:profile`                                                                                                                                                                                                                       | [Authentification](#oauth-scope-requirement)                                                                                   |
| `claude.ai rejected the session token` / `session token rejected`                                                                                                                                                                                                    | [Authentification](#claude-ai-rejected-the-session-token)                                                                      |
| `MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)`                                                                                                                                                                                       | [Authentification](#mcp-server-needs-you-to-sign-in-again)                                                                     |
| `rejected the credential from its headersHelper` / `rejected the Authorization header in its config`                                                                                                                                                                 | [Authentification](#mcp-server-needs-you-to-sign-in-again)                                                                     |
| `MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate`                                                                                                                                                                  | [Authentification](#mcp-server-needs-you-to-sign-in-again)                                                                     |
| `MCP server "<name>" requires re-authorization (token expired)`                                                                                                                                                                                                      | [Authentification](#mcp-server-needs-you-to-sign-in-again)                                                                     |
| `Issuer mismatch in authorization response (RFC 9207)`                                                                                                                                                                                                               | [Authentification](#issuer-mismatch-in-authorization-response)                                                                 |
| `Cloud gateway session expired — run /login to reconnect.`                                                                                                                                                                                                           | [Authentification](#cloud-gateway-session-expired)                                                                             |
| `Cloud gateway <url> no longer accepts this session`                                                                                                                                                                                                                 | [Authentification](#cloud-gateway-session-expired)                                                                             |
| `Sign-in timed out while waiting for you to continue. Try again.`                                                                                                                                                                                                    | [Authentification](#sign-in-timed-out-while-waiting-for-you-to-continue)                                                       |
| `AWS credentials expired or invalid`                                                                                                                                                                                                                                 | [Authentification](#aws-credentials-expired-or-invalid)                                                                        |
| `AWS authentication failed`                                                                                                                                                                                                                                          | [Authentification](#aws-authentication-failed)                                                                                 |
| `Google Cloud credentials expired or invalid`                                                                                                                                                                                                                        | [Authentification](#google-cloud-credentials-expired-or-invalid)                                                               |
| `Google Cloud authentication failed`                                                                                                                                                                                                                                 | [Authentification](#google-cloud-authentication-failed)                                                                        |
| `Microsoft Foundry authentication failed`                                                                                                                                                                                                                            | [Authentification](#microsoft-foundry-authentication-failed)                                                                   |
| `Gateway refused the request`                                                                                                                                                                                                                                        | [Authentification](#gateway-refused-the-request)                                                                               |
| `Could not load AWS credentials` / `Could not load Google Cloud credentials`                                                                                                                                                                                         | [Authentification](#could-not-load-aws-or-google-cloud-credentials)                                                            |
| `AWS default-chain credential resolve timed out`                                                                                                                                                                                                                     | [Authentification](#aws-default-chain-credential-resolve-timed-out)                                                            |
| `Timed out after 60s waiting for AWS`                                                                                                                                                                                                                                | [Authentification](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                      |
| `A request to AWS timed out. Check your network and proxy settings, then try again.`                                                                                                                                                                                 | [Authentification](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                      |
| `Could not load the default credentials` on Google Cloud's Agent Platform                                                                                                                                                                                            | [Authentification](#could-not-load-aws-or-google-cloud-credentials)                                                            |
| `Unable to connect to API`                                                                                                                                                                                                                                           | [Réseau](#unable-to-connect-to-api)                                                                                            |
| `Connection refused —` / `Can't reach the API server —` / `No internet route —` / `Couldn't connect through your proxy` / `Connection dropped`, each with an error code in parentheses                                                                               | [Réseau](#unable-to-connect-to-api)                                                                                            |
| `Unable to connect to Anthropic services` during setup                                                                                                                                                                                                               | [Réseau](#unable-to-connect-to-anthropic-services)                                                                             |
| `Socket is closed`                                                                                                                                                                                                                                                   | [Réseau](#socket-is-closed)                                                                                                    |
| `Waiting for API response · will retry in`                                                                                                                                                                                                                           | [Tentatives automatiques](#automatic-retries), ou [Réseau](#unable-to-connect-to-api) si cela persiste                         |
| `API returned an empty or malformed response`                                                                                                                                                                                                                        | [Réseau](#api-returned-an-empty-or-malformed-response)                                                                         |
| `Streaming response ended before any complete data was received`                                                                                                                                                                                                     | [Réseau](#streaming-response-ended-before-any-complete-data-was-received)                                                      |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"`                                                                                                                                                                   | [Réseau](#bedrock-streaming-response-has-an-unexpected-content-type)                                                           |
| `SSL certificate verification failed`                                                                                                                                                                                                                                | [Réseau](#ssl-certificate-errors)                                                                                              |
| `SSL certificate error (...)` during login or startup                                                                                                                                                                                                                | [Réseau](#ssl-certificate-errors)                                                                                              |
| `unable to get local issuer certificate`                                                                                                                                                                                                                             | [Réseau](#ssl-certificate-errors)                                                                                              |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                                                                                                                                                                                           | [Réseau](#host-not-allowed-in-a-cloud-session)                                                                                 |
| `proxy refused the connection`                                                                                                                                                                                                                                       | [Réseau](#the-proxy-refused-the-connection)                                                                                    |
| `403` with `This GraphQL query is not enabled for this session` in a cloud session                                                                                                                                                                                   | [GitHub proxy](/docs/fr/cloud-environments#github-proxy)                                                                            |
| `The cloud environments service returned an empty response` / `The cloud environments service returned a response in an unexpected format`                                                                                                                           | [Réseau](#the-cloud-environments-service-returned-an-empty-or-unexpected-response)                                             |
| `Couldn't reconnect to your Remote Control session`                                                                                                                                                                                                                  | [Réseau](#couldnt-reconnect-to-your-remote-control-session)                                                                    |
| `N sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.`                                                                                                                                               | [Réseau](#sessions-ended-while-this-machine-was-offline)                                                                       |
| `Couldn't share the transcript.`                                                                                                                                                                                                                                     | [Réseau](#couldnt-share-the-transcript)                                                                                        |
| `Prompt is too long` / `Input is too long for requested model`                                                                                                                                                                                                       | [Erreurs de requête](#prompt-is-too-long)                                                                                      |
| `Prompt is too long · automatic compaction failed:`                                                                                                                                                                                                                  | [Erreurs de requête](#prompt-is-too-long)                                                                                      |
| `Prompt is too long · this conversation is a single exchange` / `A single-exchange conversation cannot be compacted`                                                                                                                                                 | [Erreurs de requête](#prompt-is-too-long)                                                                                      |
| `Context limit reached · /compact or /clear to continue`                                                                                                                                                                                                             | [Erreurs de requête](#prompt-is-too-long)                                                                                      |
| `Context limit reached · /clear to continue`                                                                                                                                                                                                                         | [Erreurs de requête](#prompt-is-too-long)                                                                                      |
| `capability_rejected: prompt_too_long` on a Claude apps gateway session                                                                                                                                                                                              | [Erreurs de requête](#prompt-is-too-long)                                                                                      |
| `upstream rejected the request` / `request too large for this upstream` on a Claude apps gateway session                                                                                                                                                             | [Messages d'erreur en amont](/docs/fr/claude-apps-gateway-config#upstream-error-messages)                                           |
| `upstream rate limit exceeded` on a Claude apps gateway session                                                                                                                                                                                                      | [Messages d'erreur en amont](/docs/fr/claude-apps-gateway-config#upstream-error-messages)                                           |
| `all upstreams failed (N attempted)` on a Claude apps gateway session                                                                                                                                                                                                | [Messages d'erreur en amont](/docs/fr/claude-apps-gateway-config#upstream-error-messages)                                           |
| `Claude Code may not be enabled for your organization` after a Claude apps gateway sign-in                                                                                                                                                                           | [Dépannage de la passerelle Claude apps](/docs/fr/claude-apps-gateway-deploy#troubleshooting)                                       |
| `Context exceeds the ...-token limit by ... tokens` in `/context` output                                                                                                                                                                                             | [Erreurs de requête](#context-exceeds-the-token-limit)                                                                         |
| `Error during compaction: Conversation too long`                                                                                                                                                                                                                     | [Erreurs de requête](#error-during-compaction-conversation-too-long)                                                           |
| `Request too large`                                                                                                                                                                                                                                                  | [Erreurs de requête](#request-too-large)                                                                                       |
| `Request too large for the API's 32MB request limit`                                                                                                                                                                                                                 | [Erreurs de requête](#request-too-large)                                                                                       |
| `Image was too large`                                                                                                                                                                                                                                                | [Erreurs de requête](#image-was-too-large)                                                                                     |
| `Unable to resize image`                                                                                                                                                                                                                                             | [Erreurs de requête](#unable-to-resize-image)                                                                                  |
| `PDF too large` / `PDF is password protected`                                                                                                                                                                                                                        | [Erreurs de requête](#pdf-errors)                                                                                              |
| `Extra inputs are not permitted`                                                                                                                                                                                                                                     | [Erreurs de requête](#extra-inputs-are-not-permitted)                                                                          |
| `API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid` / `Property keys should match pattern`                                                                                                                                                      | [Erreurs de requête](#tool-input-schema-is-invalid)                                                                            |
| `There's an issue with the selected model`                                                                                                                                                                                                                           | [Erreurs de requête](#theres-an-issue-with-the-selected-model)                                                                 |
| `Model ... is not a recognized model id`                                                                                                                                                                                                                             | [Erreurs de requête](#model-is-not-a-recognized-model-id)                                                                      |
| `Model ... not found`                                                                                                                                                                                                                                                | [Erreurs de requête](#model-not-found)                                                                                         |
| `Claude Opus is not available with the Claude Pro plan`                                                                                                                                                                                                              | [Erreurs de requête](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                   |
| `Claude Code ... does not support this model; version ... or newer is required`                                                                                                                                                                                      | [Erreurs de requête](#claude-code-does-not-support-this-model)                                                                 |
| `Claude Code ... is older than the minimum version required by your organization's policy`                                                                                                                                                                           | [Erreurs de requête](#claude-code-does-not-support-this-model)                                                                 |
| `Model ... is restricted by your organization's settings`                                                                                                                                                                                                            | [Erreurs de requête](#model-is-restricted-by-your-organizations-settings)                                                      |
| `Model switch ... blocked by a PreModelSwitch hook`                                                                                                                                                                                                                  | [Erreurs de requête](#model-switch-was-blocked-by-a-premodelswitch-hook)                                                       |
| `couldn't save it as your default` / `couldn't confirm it was saved as your default`                                                                                                                                                                                 | [Erreurs de requête](#couldnt-save-it-as-your-default)                                                                         |
| `thinking.type.enabled is not supported for this model`                                                                                                                                                                                                              | [Erreurs de requête](#thinking-type-enabled-is-not-supported-for-this-model)                                                   |
| `Effort '<level>' isn't available with thinking turned off on this model`                                                                                                                                                                                            | [Erreurs de requête](#effort-isnt-available-with-thinking-turned-off)                                                          |
| `effort '<level>' is not supported when thinking is disabled`                                                                                                                                                                                                        | [Erreurs de requête](#effort-isnt-available-with-thinking-turned-off)                                                          |
| `max_tokens must be greater than thinking.budget_tokens`                                                                                                                                                                                                             | [Erreurs de requête](#thinking-budget-exceeds-output-limit)                                                                    |
| `API Error: 400 due to tool use concurrency issues`                                                                                                                                                                                                                  | [Erreurs de requête](#tool-use-or-thinking-block-mismatch)                                                                     |
| `API Error: 400 orphaned tool_result in conversation history`                                                                                                                                                                                                        | [Erreurs de requête](#tool-use-or-thinking-block-mismatch)                                                                     |
| `API Error: 400 duplicate tool_use ID in conversation history`                                                                                                                                                                                                       | [Erreurs de requête](#tool-use-or-thinking-block-mismatch)                                                                     |
| `[Unsupported tool content removed]`                                                                                                                                                                                                                                 | [Erreurs de requête](#unsupported-tool-content-removed)                                                                        |
| `role 'system' must precede an 'assistant' message`                                                                                                                                                                                                                  | [Erreurs de requête](#role-system-must-precede-an-assistant-message)                                                           |
| `Invalid encrypted_content in search_result block` / `Invalid encrypted_index in text block` / `Failed to decrypt web search result content`                                                                                                                         | [Erreurs de requête](#invalid-encrypted-content-in-search-result-block)                                                        |
| `server_tool_use.name: Input should be` on every turn of a resumed session                                                                                                                                                                                           | [Erreurs de requête](#unsupported-tool-content-removed)                                                                        |
| `<model> can't help with this. Start a new session to continue`                                                                                                                                                                                                      | [Erreurs de requête](#usage-policy-refusal)                                                                                    |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`                                                                                                                                                                        | [Erreurs de requête](#usage-policy-refusal)                                                                                    |
| `<model>'s safeguards flagged this message`                                                                                                                                                                                                                          | [Erreurs de requête](#safety-measures-flagged-a-cybersecurity-topic)                                                           |
| `Opus 5.5's safeguards flagged this session`                                                                                                                                                                                                                         | [Erreurs de requête](#safety-measures-flagged-a-cybersecurity-topic)                                                           |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                                                                                                                                                                                    | [Erreurs de requête](#safety-measures-flagged-a-cybersecurity-topic)                                                           |
| `Installation was killed before it could finish (exit code 137)`                                                                                                                                                                                                     | [Erreurs d'installation](#installation-was-killed-before-it-could-finish)                                                      |
| `The connection dropped while downloading the update`                                                                                                                                                                                                                | [Erreurs d'installation](#the-connection-dropped-while-downloading-the-update)                                                 |
| `Download timed out: exceeded the total deadline`                                                                                                                                                                                                                    | [Erreurs d'installation](#the-connection-dropped-while-downloading-the-update)                                                 |
| `--bg and --print conflict`                                                                                                                                                                                                                                          | [Erreurs de ligne de commande](#command-line-errors)                                                                           |
| `Cloud sessions cannot be created from a --restricted session`                                                                                                                                                                                                       | [Erreurs de ligne de commande](#cloud-sessions-cannot-be-created-from-a-restricted-session)                                    |
| `Cloud sessions are disabled by your organization's policy`                                                                                                                                                                                                          | [Erreurs de ligne de commande](#cloud-sessions-are-disabled-by-your-organizations-policy)                                      |
| `Couldn't verify your organization's policy for cloud sessions`                                                                                                                                                                                                      | [Erreurs de ligne de commande](#cloud-sessions-are-disabled-by-your-organizations-policy)                                      |
| `Error: --json-schema is not a valid JSON Schema`                                                                                                                                                                                                                    | [Erreurs de ligne de commande](#command-line-errors)                                                                           |
| `Error: Invalid --agents configuration:`                                                                                                                                                                                                                             | [Erreurs de ligne de commande](#invalid-agents-configuration)                                                                  |
| `Error: Settings file exceeds the 2MiB limit`                                                                                                                                                                                                                        | [Erreurs de ligne de commande](#settings-file-exceeds-the-2mib-limit)                                                          |
| `The current directory no longer exists (it was deleted or moved)` / `Can't read the current directory`                                                                                                                                                              | [Erreurs de ligne de commande](#the-current-directory-no-longer-exists)                                                        |
| `Temp directory <dir> ... Refusing to use it` / `ENOSPC: no space left on device, mkdir '<dir>'`                                                                                                                                                                     | [Erreurs de ligne de commande](#temp-directory-refused-or-cannot-be-created)                                                   |
| `couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded`                                                                                                                                                                        | [Erreurs de ligne de commande](#directory-couldnt-be-resolved-to-a-real-location)                                              |
| `Error: Workspace not trusted` when starting Remote Control                                                                                                                                                                                                          | [Erreurs de ligne de commande](#workspace-not-trusted-when-starting-remote-control)                                            |
| `` `<flag>` before `remote-control` is not carried over to the sessions Remote Control starts ``                                                                                                                                                                     | [Erreurs de ligne de commande](#not-carried-over-to-the-sessions-remote-control-starts)                                        |
| `` `claude import` is not yet available in this build ``                                                                                                                                                                                                             | [Erreurs de ligne de commande](#claude-import-is-not-yet-available-in-this-build)                                              |
| `Could not read Claude Code config`                                                                                                                                                                                                                                  | [Erreurs de ligne de commande](#could-not-read-claude-code-config)                                                             |
| `Could not import <server>: <reason>`                                                                                                                                                                                                                                | [Erreurs de ligne de commande](#could-not-import-a-server-from-claude-desktop)                                                 |
| `Cannot add MCP server to scope: managed`                                                                                                                                                                                                                            | [Erreurs de ligne de commande](#cannot-add-mcp-server-to-the-managed-scope)                                                    |
| `is Anthropic-hosted and doesn't support local OAuth`                                                                                                                                                                                                                | [Erreurs de ligne de commande](#anthropic-hosted-and-doesnt-support-local-oauth)                                               |
| `Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes`                                                                                                                                                                                      | [Erreurs de ligne de commande](#cant-read-mcp-json)                                                                            |
| `Server rejected the Authorization header minted by the configured headersHelper`                                                                                                                                                                                    | [Erreurs de ligne de commande](#server-rejected-the-authorization-header-minted-by-the-configured-headershelper)               |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                                                                                                                                                                                             | [Erreurs de ligne de commande](#mcp-permission-prompt-tool-not-found)                                                          |
| `OAuth callback port <port> is already in use — another process may be holding it`                                                                                                                                                                                   | [Erreurs de ligne de commande](#oauth-callback-port-is-already-in-use)                                                         |
| `No available ports for OAuth redirect`                                                                                                                                                                                                                              | [Erreurs de ligne de commande](#no-available-ports-for-oauth-redirect)                                                         |
| `Shell command failed for pattern "..."`, from `/security-review` or any skill that injects dynamic context                                                                                                                                                          | [Erreurs de ligne de commande](#security-review-fails-without-origin-head)                                                     |
| `Shell command permission check failed for pattern "..."`, from a skill that injects dynamic context                                                                                                                                                                 | [Erreurs de ligne de commande](#security-review-fails-without-origin-head)                                                     |
| ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``                                                                                                                                                                             | [Erreurs de ligne de commande](#security-review-fails-without-origin-head)                                                     |
| `Input must be provided either through stdin or as a prompt argument when using --print`                                                                                                                                                                             | [Erreurs de ligne de commande](#input-must-be-provided-when-using-print)                                                       |
| `Error: Input contained only whitespace`                                                                                                                                                                                                                             | [Erreurs de ligne de commande](#input-contained-only-whitespace)                                                               |
| `Blank prompt — the message was only whitespace, so nothing was sent to the model.`                                                                                                                                                                                  | [Erreurs de ligne de commande](#input-contained-only-whitespace)                                                               |
| `Error: stream-json input carried over 256M characters with no newline`                                                                                                                                                                                              | [Erreurs de ligne de commande](#stream-json-input-carried-over-256m-characters-with-no-newline)                                |
| `Unknown command: /<name>`, with or without a `Did you mean` suggestion                                                                                                                                                                                              | [Erreurs de ligne de commande](#unknown-command)                                                                               |
| `Diff is too large for ultrareview` / `PR #<N> is too large for ultrareview`                                                                                                                                                                                         | [Erreurs de ligne de commande](#diff-is-too-large-for-ultrareview)                                                             |
| `Could not find merge-base with <branch>`                                                                                                                                                                                                                            | [Erreurs de ligne de commande](#could-not-find-merge-base-with-the-base-branch)                                                |
| `Your checkout has no branches (detached HEAD only)`                                                                                                                                                                                                                 | [Erreurs de ligne de commande](#your-checkout-has-no-branches)                                                                 |
| `Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected`                                                                                                                                     | [Erreurs de ligne de commande](#no-github-account-is-connected-to-your-claude-account)                                         |
| `Your connected GitHub account can't see <owner>/<repo>`                                                                                                                                                                                                             | [Erreurs de ligne de commande](#your-connected-github-account-cant-see-the-repository)                                         |
| `The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead`                                                                                                                                           | [Erreurs de ligne de commande](#the-github-app-preflight-failed-transiently)                                                   |
| `GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud`                                                                                                                                                                     | [Erreurs de ligne de commande](#github-isnt-connected-to-your-claude-account)                                                  |
| `Single sign-on authorization needed`                                                                                                                                                                                                                                | [Erreurs de ligne de commande](#single-sign-on-authorization-needed)                                                           |
| `Failed to resume the conversation`                                                                                                                                                                                                                                  | [Erreurs de ligne de commande](#failed-to-resume-the-conversation)                                                             |
| `No conversation found with session ID: <session-id>`                                                                                                                                                                                                                | [Erreurs de ligne de commande](#no-conversation-found-with-the-session-id)                                                     |
| `Cannot switch renderers in this session`                                                                                                                                                                                                                            | [Erreurs de ligne de commande](#cannot-switch-renderers-in-this-session)                                                       |
| `Cannot switch renderers while work is running in the background`                                                                                                                                                                                                    | [Erreurs de ligne de commande](#cannot-switch-renderers-in-this-session)                                                       |
| `Couldn't open Claude Desktop`                                                                                                                                                                                                                                       | [Erreurs de ligne de commande](#couldnt-open-claude-desktop)                                                                   |
| `Failed to open Claude Desktop. Please try opening it manually.`                                                                                                                                                                                                     | [Erreurs de ligne de commande](#couldnt-open-claude-desktop)                                                                   |
| `Couldn't read your Zed keymap` / `Couldn't back up your Zed keymap` / `Couldn't update your Zed keymap`                                                                                                                                                             | [Erreurs de ligne de commande](#terminal-setup-left-your-zed-keymap-unchanged)                                                 |
| `Your Zed keymap isn't a readable list of keybindings`                                                                                                                                                                                                               | [Erreurs de ligne de commande](#terminal-setup-left-your-zed-keymap-unchanged)                                                 |
| `Skill usage reports are not available on this connection.`                                                                                                                                                                                                          | [Erreurs de ligne de commande](#skill-usage-reports-are-not-available-on-this-connection)                                      |
| `Custom output styles can't be selected over Remote Control or from a relayed message`                                                                                                                                                                               | [Erreurs de ligne de commande](#custom-output-styles-cant-be-selected-over-remote-control)                                     |
| `Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load`                                                                                                                                                           | [Erreurs de ligne de commande](#output-styles-are-saved-to-local-settings-which-this-session-doesnt-load)                      |
| `` `plugin eval` is currently in early access `` / `` `plugin eval` is currently unavailable ``                                                                                                                                                                      | [Erreurs de plugin](#plugin-eval-is-currently-in-early-access)                                                                 |
| `Marketplace "<name>" is registered from an untrusted source`                                                                                                                                                                                                        | [Erreurs de plugin](#marketplace-is-registered-from-an-untrusted-source)                                                       |
| `Marketplace "<name>" is already added from a different source`                                                                                                                                                                                                      | [Erreurs de plugin](#marketplace-is-already-added-from-a-different-source)                                                     |
| `"<name>" is another spelling of "<reserved>", a reserved marketplace name`                                                                                                                                                                                          | [Erreurs de plugin](#marketplace-name-is-another-spelling-of-a-reserved-name)                                                  |
| `references ${user_config.*} in a shell-form command`                                                                                                                                                                                                                | [Erreurs de plugin](#plugin-command-references-user-config)                                                                    |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                                                                                                                                                                                   | [Erreurs de plugin](#plugin-command-references-user-config)                                                                    |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                                                                                                                                                                                  | [Erreurs de plugin](#plugin-command-references-user-config)                                                                    |
| `Plugin archive integrity check failed`                                                                                                                                                                                                                              | [Erreurs de plugin](#plugin-archive-integrity-check-failed)                                                                    |
| `path escapes plugin directory`                                                                                                                                                                                                                                      | [Erreurs de plugin](#path-escapes-plugin-directory)                                                                            |
| `path could not be checked`                                                                                                                                                                                                                                          | [Erreurs de plugin](#path-could-not-be-checked)                                                                                |
| `its marketplace entry path does not stay inside the marketplace directory`                                                                                                                                                                                          | [Erreurs de plugin](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                    |
| `Plugin source path refused`                                                                                                                                                                                                                                         | [Erreurs de plugin](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                    |
| `Failed to load marketplace configuration`                                                                                                                                                                                                                           | [Erreurs de plugin](#failed-to-load-marketplace-configuration)                                                                 |
| `Marketplace configuration file is corrupted`                                                                                                                                                                                                                        | [Erreurs de plugin](#failed-to-load-marketplace-configuration)                                                                 |
| `Plugin "<name>@synced" is required by your organization and can't be disabled here`                                                                                                                                                                                 | [Erreurs de plugin](#plugin-is-required-by-your-organization)                                                                  |
| `would be spawned with zero tools — refusing`                                                                                                                                                                                                                        | [Erreurs d'outil](#agent-would-be-spawned-with-zero-tools)                                                                     |
| `File is covered by a Read deny rule in your permission settings`                                                                                                                                                                                                    | [Erreurs d'outil](#file-is-covered-by-a-read-deny-rule)                                                                        |
| `subagent_type is required: the general-purpose agent is not available in this session`                                                                                                                                                                              | [Erreurs d'outil](#subagent-type-is-required)                                                                                  |
| `Error: this write left the memory index at MEMORY.md at ..., over its ... read limit`                                                                                                                                                                               | [Erreurs d'outil](#memory-index-is-over-its-read-limit)                                                                        |
| `pkill: refusing to run`                                                                                                                                                                                                                                             | [Erreurs d'outil](#pkill-pattern-matches-the-claude-code-process)                                                              |
| `Failed to write to <name>'s inbox — nothing was sent`                                                                                                                                                                                                               | [Erreurs d'outil](#failed-to-write-to-a-teammate-inbox)                                                                        |
| `Failed to write the plan approval request to the lead's inbox — plan not submitted`                                                                                                                                                                                 | [Erreurs d'outil](#failed-to-write-to-a-teammate-inbox)                                                                        |
| `Its agent definition was not restored: the folder its definition file came from is not trusted`                                                                                                                                                                     | [Erreurs d'outil](#teammate-agent-definition-not-restored)                                                                     |
| `Message too large for cross-session delivery`                                                                                                                                                                                                                       | [Erreurs d'outil](#message-too-large-for-cross-session-delivery)                                                               |
| `Too many messages to this session just now`                                                                                                                                                                                                                         | [Erreurs d'outil](#too-many-messages-to-this-session-just-now)                                                                 |
| `Refusing to send: reply target is a symlink` / `Refusing to send: cannot vet reply target`                                                                                                                                                                          | [Erreurs d'outil](#refusing-to-send-a-cross-session-message)                                                                   |
| `Refusing to send: connected endpoint is not the expected process` / `Refusing to send: connected endpoint identity could not be read`                                                                                                                               | [Erreurs d'outil](#refusing-to-send-a-cross-session-message)                                                                   |
| `Refusing to send: connected endpoint is not owned by this user` / `Refusing to send: connected endpoint owner could not be read`                                                                                                                                    | [Erreurs d'outil](#refusing-to-send-a-cross-session-message)                                                                   |
| `Refusing to send: connected endpoint is a different process with the expected pid`                                                                                                                                                                                  | [Erreurs d'outil](#refusing-to-send-a-cross-session-message)                                                                   |
| `Refusing to read <path>: its symlink resolution changed after permission was checked (<reason>)` / `Refusing to search <path>: its symlink resolution changed after permission was checked`                                                                         | [Erreurs d'outil](#refusing-after-a-symlink-changed)                                                                           |
| `Refusing to write <path>: its parent-directory symlink resolution changed after permission was checked` / `Refusing to write <path>: it is a symbolic link. Write to the link's target path instead`                                                                | [Erreurs d'outil](#refusing-after-a-symlink-changed)                                                                           |
| `Refusing to write through symlink: <path>` / `Refusing to write into symlinked directory: <path>`                                                                                                                                                                   | [Erreurs d'outil](#refusing-after-a-symlink-changed)                                                                           |
| `Refusing to search <path>: a path one of its Read deny rules is written through changed while the search was being prepared` / `Refusing to search <path>: it could not be opened`                                                                                  | [Erreurs d'outil](#refusing-after-a-symlink-changed)                                                                           |
| `its permission check expired before it ran (too many concurrent file operations)` / `ripgrep was found only by name on PATH`                                                                                                                                        | [Erreurs d'outil](#refusing-after-a-symlink-changed)                                                                           |
| `task output swap refused (tasks dir moved or linked)`                                                                                                                                                                                                               | [Erreurs d'outil](#task-output-swap-refused)                                                                                   |
| `Command killed: its output file was replaced or could no longer be verified`                                                                                                                                                                                        | [Erreurs d'outil](#task-output-swap-refused)                                                                                   |
| `the source file is not valid UTF-8 text` / `the source file is not valid UTF-16 text`                                                                                                                                                                               | [Erreurs d'outil](#the-source-file-is-not-valid-utf-8-text)                                                                    |
| `the source file has the replacement character U+FFFD`                                                                                                                                                                                                               | [Erreurs d'outil](#the-source-file-is-not-valid-utf-8-text)                                                                    |
| `Reading a local file from outside this session's connected folders, or through a link, needs the approval card`                                                                                                                                                     | [Erreurs d'outil](#reading-a-local-file-from-outside-the-connected-folders)                                                    |
| `cannot read file_path (...) — the file could not be examined, and no one can answer the approval card`                                                                                                                                                              | [Erreurs d'outil](#reading-a-local-file-from-outside-the-connected-folders)                                                    |
| `WebFetch cannot fetch localhost or other hostnames without a dot`                                                                                                                                                                                                   | [Erreurs d'outil](#webfetch-cannot-fetch-localhost)                                                                            |
| `Can't open MCP settings while no terminal is attached to this background session`                                                                                                                                                                                   | [Erreurs de session en arrière-plan](#commands-refused-in-a-background-session)                                                |
| `Can't open MCP settings in a background session`                                                                                                                                                                                                                    | [Erreurs de session en arrière-plan](#commands-refused-in-a-background-session)                                                |
| `blocked because the path is spelled in a form that cannot be safely resolved`                                                                                                                                                                                       | [Erreurs de session en arrière-plan](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved)                     |
| `blocked because the path is network-shaped`                                                                                                                                                                                                                         | [Erreurs de session en arrière-plan](#write-or-command-blocked-because-the-path-names-a-network-location)                      |
| `is isolated in the worktree <path>, but this command <reason>. Refusing to run it`                                                                                                                                                                                  | [Erreurs de session en arrière-plan](#command-blocked-by-the-worktree-isolation-checks)                                        |
| `too complex to verify that it stays inside the worktree`                                                                                                                                                                                                            | [Erreurs de session en arrière-plan](#command-blocked-by-the-worktree-isolation-checks)                                        |
| `This session has no saved transcript`                                                                                                                                                                                                                               | [Erreurs de session en arrière-plan](#this-session-has-no-saved-transcript)                                                    |
| `Can't open — this session is running in another terminal`                                                                                                                                                                                                           | [Erreurs de session en arrière-plan](#this-session-is-running-in-another-terminal)                                             |
| `This conversation is already open in another running Claude session`                                                                                                                                                                                                | [Erreurs de session en arrière-plan](#this-session-is-running-in-another-terminal)                                             |
| `This session's saved conversation is no longer on disk`                                                                                                                                                                                                             | [Erreurs de session en arrière-plan](#this-sessions-saved-conversation-is-no-longer-on-disk)                                   |
| `kept <id> — its worktree is still at <path>`                                                                                                                                                                                                                        | [Erreurs de session en arrière-plan](#worktree-has-commits-that-are-not-pushed-anywhere)                                       |
| `kept <id> — <n> unpushed commits on <branch>`                                                                                                                                                                                                                       | [Erreurs de session en arrière-plan](#worktree-has-commits-that-are-not-pushed-anywhere)                                       |
| `kept <id> — worktree has commits that are not pushed anywhere`                                                                                                                                                                                                      | [Erreurs de session en arrière-plan](#worktree-has-commits-that-are-not-pushed-anywhere)                                       |
| `terminal host process died — press Enter to restart` / `This session's terminal host process died`                                                                                                                                                                  | [Erreurs de session en arrière-plan](#terminal-host-process-died)                                                              |
| `Session isn't responding` / `Press enter again to restart this session — it isn't responding`                                                                                                                                                                       | [Erreurs de session en arrière-plan](#session-isnt-responding)                                                                 |
| `Session <id> was stopped while the respawn was in flight`                                                                                                                                                                                                           | [Erreurs de session en arrière-plan](#session-was-stopped-while-the-respawn-was-in-flight)                                     |
| `This session was running agent '<name>', which is no longer available`                                                                                                                                                                                              | [Erreurs de session en arrière-plan](#session-agent-no-longer-available)                                                       |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                                                                                                                                                                                          | [Erreurs de session en arrière-plan](#claude_code_process_wrapper-launcher-errors)                                             |
| `EUNKNOWN: unknown error, uv_spawn`                                                                                                                                                                                                                                  | [Erreurs de session en arrière-plan](#eunknown-when-starting-a-background-session)                                             |
| `EACCES: permission denied, posix_spawn`                                                                                                                                                                                                                             | [Erreurs de session en arrière-plan](#eacces-when-starting-a-background-session)                                               |
| `exited before it became reachable`                                                                                                                                                                                                                                  | [Erreurs de session en arrière-plan](#background-service-exited-before-it-became-reachable)                                    |
| `Couldn't start a background session (working directory no longer exists or is not accessible: ...)`                                                                                                                                                                 | [Erreurs de session en arrière-plan](#working-directory-no-longer-exists-when-starting-a-background-session)                   |
| `Claude Code is being updated by npm on this machine (still not runnable after 2 min, ...)`                                                                                                                                                                          | [Erreurs de session en arrière-plan](#eacces-when-starting-a-background-session)                                               |
| `Claude Code process exited with code N`                                                                                                                                                                                                                             | [Erreurs de wrapper et d'IDE](#claude-code-process-exited-with-code-n)                                                         |
| `The connection to Claude Code ended before this message completed`                                                                                                                                                                                                  | [Erreurs de wrapper et d'IDE](#the-connection-to-claude-code-ended-before-this-message-completed)                              |
| `Could not locate the Claude CLI on PATH`                                                                                                                                                                                                                            | [Erreurs de wrapper et d'IDE](#could-not-locate-the-claude-cli-on-path)                                                        |
| `Restored the code, but skipped N files`                                                                                                                                                                                                                             | [Avertissements et erreurs de rembobinage](#restored-the-code-but-skipped-files)                                               |
| `No files were restored: N files failed (backup missing, or the file could not be updated)`                                                                                                                                                                          | [Avertissements et erreurs de rembobinage](#no-files-were-restored)                                                            |
| `Transcript writes are failing (...)`                                                                                                                                                                                                                                | [Avertissements d'enregistrement de session](#transcript-writes-are-failing)                                                   |
| `Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set`                                                                                                                                                                                                  | [Avertissements d'enregistrement de session](#transcript-saving-is-off-skip-prompt-history)                                    |
| `Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker`                                                                                                                                                                                              | [Avertissements d'enregistrement de session](#transcript-saving-is-off-child-session-marker)                                   |
| `Claude Code's fullscreen renderer didn't finish starting last time on this machine` / `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`                                                                                            | [Avertissements de configuration](#fullscreen-failed-start-notice)                                                             |
| `Claude Code exited after an unrecoverable interface error (...)`                                                                                                                                                                                                    | [Avertissements de configuration](#exited-after-an-unrecoverable-interface-error)                                              |
| `Agent descriptions are over the 15.0k-token limit`                                                                                                                                                                                                                  | [Avertissements de configuration](#agent-descriptions-are-over-the-15000-token-limit)                                          |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                                                                                                                                                                                  | [Avertissements de configuration](#workspace-has-not-been-trusted)                                                             |
| `is a network path, which cannot be added as a working directory`                                                                                                                                                                                                    | [Avertissements de configuration](#working-directory-is-a-network-path)                                                        |
| `Remote managed settings failed to load (<cause>)`                                                                                                                                                                                                                   | [Avertissements de configuration](#remote-managed-settings-failed-to-load)                                                     |
| `Managed settings were not approved; exiting without applying them.`                                                                                                                                                                                                 | [Avertissements de configuration](#managed-settings-were-not-approved)                                                         |
| `MCP server <name> is blocked by enterprise managed policy`                                                                                                                                                                                                          | [Avertissements de configuration](#mcp-server-is-blocked-by-enterprise-managed-policy)                                         |
| `Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.`                                                                                                                                              | [Avertissements de configuration](#managed-settings-document-could-not-be-parsed)                                              |
| `Managed settings drop-in directory could not be read`                                                                                                                                                                                                               | [Avertissements de configuration](#managed-settings-document-could-not-be-parsed)                                              |
| `otelHeadersHelper failed; telemetry is not being exported. See /status: ...`                                                                                                                                                                                        | [Avertissements de configuration](#otelheadershelper-failed)                                                                   |
| `"crossSessionInbound" must be one of "accept", "hold", "refuse"`                                                                                                                                                                                                    | [Avertissements de configuration](#crosssessioninbound-must-be-one-of-accept-hold-refuse)                                      |
| `headersHelper not run — this workspace has no persisted trust`                                                                                                                                                                                                      | [Avertissements de configuration](#headershelper-not-run)                                                                      |
| `Invalid permission rule "..." was skipped: Malformed Tool(content) rule`                                                                                                                                                                                            | [Avertissements de configuration](#malformed-tool-content-rule)                                                                |
| `... is not matched by file permission checks`                                                                                                                                                                                                                       | [Avertissements de configuration](#is-not-matched-by-file-permission-checks)                                                   |
| `... has a wildcard before the rest of the command`                                                                                                                                                                                                                  | [Avertissements de configuration](#has-a-wildcard-before-the-rest-of-the-command)                                              |
| `CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced`                                                                                                                                                                                           | [Avertissements de configuration](#the-200k-limit-isnt-enforced)                                                               |
| `[claude-code:unrecognized_model]`                                                                                                                                                                                                                                   | [Avertissements de configuration](#unrecognized-model-id-on-a-request)                                                         |
| `Stale sandbox mask files left by a killed session`                                                                                                                                                                                                                  | [Avertissements de configuration](#stale-sandbox-mask-files-left-by-a-killed-session)                                          |
| Les réponses semblent être de qualité inférieure à la normale                                                                                                                                                                                                        | [Qualité des réponses](#responses-seem-lower-quality-than-usual)                                                               |

<h2 id="automatic-retries">
  Tentatives automatiques
</h2>

Claude Code réessaie les défaillances transitoires jusqu'à 10 fois avec un backoff exponentiel avant de vous afficher une erreur. Il ne réessaie pas toujours une défaillance qui arrive au milieu de la réponse de Claude. Lorsque vous voyez l'une des erreurs de cette page, Claude Code a déjà effectué les tentatives qui s'appliquent à cette défaillance ; les listes ci-dessous indiquent quelles défaillances bénéficient du budget complet, lesquelles en bénéficient d'un plus petit, et lesquelles n'en bénéficient pas.

Claude Code réessaie ces défaillances :

* Les erreurs serveur, les réponses surchargées et les délais d'attente de requête qui arrivent avant que l'une des réponses de Claude ne soit diffusée.
* Les connexions interrompues. Lorsqu'une connexion s'interrompt au milieu d'une requête avant que Claude n'ait complété une partie de sa réponse, y compris sa réflexion, Claude Code rémet la requête avec le même backoff et le tour continue, même si du texte avait déjà commencé à être diffusé. Lorsqu'elle s'interrompt après que Claude a terminé sa réflexion mais avant qu'il n'ait commencé un texte ou un appel d'outil, Claude Code rémet plutôt la requête jusqu'à deux fois en succession rapide, et termine le tour avec `Connection lost before a response was produced` si la connexion continue à s'interrompre à ce stade.
* Une connexion que Claude Code détecte comme ayant été interrompue par votre ordinateur qui s'endort au milieu d'une requête. Claude Code la compte comme une connexion interrompue selon les règles ci-dessus ; une fois que l'étiquette de tentative nomme la raison spécifique, elle lit `Connection lost while your computer was asleep`, et si le tour se termine après que Claude a terminé sa réflexion mais avant un texte ou un appel d'outil, le message lit `Your computer went to sleep before a response was produced`.
* Un flux de réponse bloqué, lorsque les en-têtes de réponse sont arrivés mais aucune de la réponse de Claude n'est arrivée, ou lorsque Claude a terminé sa réflexion mais n'a pas commencé un texte ou un appel d'outil : Claude Code abandonne la connexion bloquée et rémet la requête au maximum une fois, en dehors du budget de 10 tentatives ci-dessus. Si la réponse se bloque une deuxième fois après que Claude a terminé sa réflexion mais avant un texte ou un appel d'outil, Claude Code termine le tour avec `The response stalled before a response was produced`.
* Une requête de diffusion à laquelle l'API ne répond jamais avec des en-têtes de réponse, sur une connexion où le [délai de premier octet s'exécute](/docs/fr/network-config#streaming-idle-watchdogs) : Claude Code l'abandonne à la date limite et le renvoie au maximum une fois par requête de modèle, dans le budget de tentatives, puis termine le tour avec [No response from API](#no-response-from-api) si cette tentative reste sans réponse aussi. Sur d'autres connexions, la requête attend `API_TIMEOUT_MS`. Lorsque vous définissez `CLAUDE_CODE_RETRY_WATCHDOG`, le plafond d'une tentative ne s'applique pas.
* Les throttles 429 temporaires, mais pas le `429` de limite de dépenses d'une passerelle, qui n'est pas un throttle ; voir [Spend limit reached](#spend-limit-reached).
  * Lorsque vous êtes connecté avec un abonnement claude.ai, cela inclut les throttles 429 qui ne portent pas les en-têtes de quota de votre plan. Avant v2.1.199, Claude Code ne réessayait ces throttles que pour les connexions par clé API et Enterprise.
* Une requête rejetée parce que l'entrée plus `max_tokens` dépasse la limite de contexte. La renvoyer inchangée échouerait de la même manière, donc Claude Code réessaie avec un `max_tokens` réduit, et arrête de réessayer et compacte à la place dans deux cas :
  * Lorsqu'aucune réduction ne peut tenir, par exemple lorsque la conversation elle-même remplit presque la fenêtre de contexte.
  * Lorsqu'une tentative ne peut pas réduire davantage `max_tokens`. Avant v2.1.218, Claude Code pouvait renvoyer une requête réduite qui ne tenait toujours pas, par exemple lorsque le budget de réflexion étendue dépassait le contexte restant, jusqu'à ce que le budget de tentatives s'épuise.
* Une credential Google Cloud expirée ou manquante sur [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), ou des credentials AWS qui ne se chargent pas sur votre machine. Claude Code rejette ses credentials en cache et réessaie jusqu'à deux fois, puis signale l'erreur pour que vous puissiez vous réauthentifier immédiatement, comme décrit sous [Could not load AWS or Google Cloud credentials](#could-not-load-aws-or-google-cloud-credentials). Avant v2.1.228, Claude Code réessayait une credential Google Cloud défaillante à travers le budget de tentatives complet avant d'afficher l'erreur.
* Un `401` ou `403` de l'API Anthropic, directement ou via une [passerelle LLM](/docs/fr/llm-gateway), tandis qu'un script [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) fournit la credential. Claude Code réexécute le script et réessaie avec sa sortie fraîche, dans le budget de tentatives complet. Lorsque le script lui-même échoue à la réexécution, Claude Code affiche [Your apiKeyHelper script is failing](#your-apikeyhelper-script-is-failing) à la place.

Avant v2.1.227, `Connection lost before a response was produced` lisait `Connection closed while thinking, before producing a response` et `The response stalled before a response was produced` lisait `Response stalled while thinking, before producing a response`.

Claude Code ne réessaie pas ces défaillances :

* Une défaillance de validation de certificat TLS, telle qu'un proxy inspectant TLS, un bundle `NODE_EXTRA_CA_CERTS` manquant, ou un certificat expiré. Claude Code signale l'erreur à la première tentative, pour que vous puissiez corriger la configuration du certificat immédiatement ; voir [SSL certificate errors](#ssl-certificate-errors). Claude Code réessaie toujours les conditions TLS transitoires telles qu'un délai d'attente de poignée de main. Avant v2.1.199, Claude Code réessayait les défaillances de certificat à travers le budget de tentatives complet avant d'afficher l'erreur.
* Une erreur serveur, une connexion interrompue, ou un flux bloqué qui arrive après que Claude a complété un bloc de texte ou un appel d'outil, ou en a commencé un après avoir terminé sa réflexion, mais avant de terminer la réponse. Claude Code ne rémet pas la requête, car cela pourrait exécuter les mêmes appels d'outil deux fois. Il conserve ce que Claude a complété, exécute tous les appels d'outil que Claude a terminés, et continue le tour à partir de leurs résultats. Pour ce que vous voyez dans une session interactive et dans une session non-interactive, lisez [The response above may be incomplete](#the-response-above-may-be-incomplete). Avant v2.1.199, Claude Code rejetait la sortie partielle et signalait le tour entier comme une erreur lorsqu'une erreur serveur arrivait au milieu du flux.
* Une défaillance qui arrive après que Claude a terminé la réponse : rien n'a besoin de réessai, donc Claude Code conserve la réponse complète et termine le tour normalement.
* Une [réponse de diffusion Amazon Bedrock avec un type de contenu inattendu](#bedrock-streaming-response-has-an-unexpected-content-type), parce que la passerelle ou le proxy réécrivant la réponse réécrirait le réessai de la même manière. Nécessite Claude Code v2.1.208 ou ultérieur.
* Un réessai non-diffusé d'une requête de diffusion défaillante qui obtient un statut de succès mais [aucun message API Claude dans le corps](#api-returned-an-empty-or-malformed-response). Claude Code termine le tour avec cette erreur.
* Une requête que la vérification de politique de votre organisation a refusée, qui apparaît comme une ligne `API Error:` portant le message de refus. Les administrateurs de votre organisation configurent la vérification avec [Inference hooks](https://platform.claude.com/docs/en/manage-claude/inference-hooks), une fonctionnalité Claude Enterprise, et le message se termine par les instructions qu'ils ont configurées, ou par défaut vous dit de les contacter. Claude Code ne renvoie pas la requête refusée au même modèle ou à un [modèle de secours](/docs/fr/model-config#fallback-model-chains), parce que le refus concerne le contenu de la requête plutôt que le modèle. Avant v2.1.239, Claude Code pouvait renvoyer une requête refusée, sans diffusion ou sur un modèle de secours configuré, avant de vous afficher le refus.

<h3 id="what-you-see-while-claude-code-retries-or-waits">
  Ce que vous voyez pendant que Claude Code réessaie ou attend
</h3>

Pendant le réessai, le spinner affiche un compte à rebours `Retrying in Ns · attempt x/y` après une étiquette d'erreur. L'étiquette nomme la raison spécifique de la première tentative pour les défaillances sur lesquelles vous pouvez agir immédiatement : le réseau est en panne, une poignée de main TLS a échoué, ou vous avez atteint une limite de débit. Pour les autres erreurs, elle lit `API error` au début. À partir de v2.1.198, elle bascule vers la raison spécifique de la troisième tentative, ou à la tentative finale lorsque `CLAUDE_CODE_MAX_RETRIES` permet moins de trois ; les versions antérieures ne basculent qu'à la tentative finale.

À partir de v2.1.198, le conseil du spinner habituel est supprimé pendant les réessais. Une fois que la raison de l'erreur est révélée, si la défaillance est une surcharge 529, la ligne en dessous du compte à rebours nomme également où vérifier l'état du service : `status.claude.com` sur l'API Anthropic, ou l'hôte du fournisseur ou de la passerelle nommé dans le message sur d'autres configurations.

Si aucune donnée n'arrive sur le flux de réponse pendant 20 secondes tandis qu'une requête est toujours en attente, le spinner affiche `Waiting for API response · will retry in … · check your network` avant que tout réessai n'ait commencé. La requête n'a pas encore échoué : le compte à rebours s'exécute jusqu'au point où Claude Code abandonne la connexion bloquée. Après l'abandon, ce que vous voyez dépend de la distance parcourue par la réponse :

* Avant que Claude n'ait complété un bloc de texte ou un appel d'outil, ou en ait commencé un après avoir terminé sa réflexion, Claude Code réessaie la requête ou termine le tour avec une erreur. [Automatic retries](#automatic-retries) dit quels blocages il réessaie et combien de fois.
* Après que Claude a complété un bloc de texte ou un appel d'outil, ou en a commencé un après avoir terminé sa réflexion, mais avant que Claude n'ait terminé la réponse, Claude Code conserve ce que Claude a complété, continue le tour à partir de tous les appels d'outil que Claude a terminés, et affiche [The response above may be incomplete](#the-response-above-may-be-incomplete). Dans une session non-interactive, et pour la réponse d'un sous-agent dans toute session, Claude Code peut d'abord inviter Claude à continuer la réponse ; cette entrée dit quand il le fait et quand vous voyez toujours l'avis là.
* Après que Claude a terminé la réponse, Claude Code termine le tour normalement.

La bannière s'efface d'elle-même une fois que les données reprennent ou qu'un réessai réussit. Si elle réapparaît à chaque tentative, traitez-la comme un [problème réseau](#unable-to-connect-to-api). Avant v2.1.185, la bannière apparaissait après 10 secondes avec un libellé différent.

Pendant que Claude consulte le [conseiller](/docs/fr/advisor), la bannière apparaît après 90 secondes sans données au lieu de 20, parce qu'un long examen du conseiller peut ne rien envoyer pendant bien plus de 20 secondes. Avant v2.1.214, le seuil de 20 secondes s'appliquait également pendant les appels du conseiller, donc la bannière apparaissait pendant les examens du conseiller même lorsque rien n'allait mal.

<h3 id="tune-retry-behavior">
  Ajuster le comportement de réessai
</h3>

Vous pouvez ajuster le comportement de réessai avec ces variables d'environnement :

| Variable                                              | Par défaut | Effet                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :---------------------------------------------------- | :--------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/fr/env-vars)             | 10         | Nombre de tentatives de réessai. Plafonné à 15 à partir de v2.1.186 ; à partir de v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` augmente la valeur par défaut et supprime le plafond. Réduisez-le pour afficher les défaillances plus rapidement dans les scripts.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/fr/env-vars)          | non défini | Définissez sur `1` dans les sessions sans surveillance telles que les travaux CI pour réessayer les erreurs de capacité `429` et `529` indéfiniment au lieu d'échouer après `CLAUDE_CODE_MAX_RETRIES` tentatives. Claude Code échoue immédiatement lorsqu'une requête à vitesse standard obtient un `429` qui signale une limite de dépenses ou des crédits d'utilisation épuisés, même un provenant d'un [plafond de dépenses de passerelle](#spend-limit-reached) qui se réinitialise selon un calendrier. Avant v2.1.239, le watchdog réessayait ces indéfiniment. Pour les requêtes en mode rapide, voir [Handle rate limits](/docs/fr/fast-mode#handle-rate-limits). Sur v2.1.199 ou ultérieur, il augmente également le nombre de tentatives par défaut pour les autres erreurs transitoires, telles que les erreurs serveur, les délais d'attente et les connexions interrompues, à 300, environ trois heures de backoff, et supprime le plafond de 15 sur `CLAUDE_CODE_MAX_RETRIES` si vous définissez explicitement cette variable. |
| [`API_TIMEOUT_MS`](/docs/fr/env-vars)                      | 600000     | Délai d'attente par requête en millisecondes. Augmentez-le pour les réseaux lents ou les proxies. Il plafonne également la durée pendant laquelle Claude Code attend les en-têtes de réponse, décrite dans [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/fr/env-vars) | non défini | Délai en millisecondes pour le premier octet de réponse d'une requête de diffusion. Nécessite Claude Code v2.1.242 ou ultérieur. Pour savoir comment Claude Code choisit le délai lorsque ceci n'est pas défini, voir [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

<h2 id="server-errors">
  Erreurs serveur
</h2>

La plupart de ces erreurs proviennent du fournisseur d'inférence : le service Anthropic sur l'API Anthropic, et le service derrière le point de terminaison de ce fournisseur sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou une passerelle personnalisée. [Le mode auto ne peut pas déterminer la sécurité d'une action](#auto-mode-cannot-determine-the-safety-of-an-action) et [L'agent s'est arrêté prématurément en raison d'une erreur API](#agent-terminated-early-due-to-an-api-error) couvrent également les causes de votre côté, comme un compte Amazon Bedrock qui ne peut pas invoquer le modèle de classification ou un sous-agent qui a atteint une limite d'utilisation.

<h3 id="api-error-500-internal-server-error">
  Erreur API : 500 Erreur serveur interne
</h3>

Claude Code affiche le code d'état et le message d'erreur de l'API pour toute réponse 5xx. L'exemple ci-dessous montre une réponse 500 sur l'API Anthropic :

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

La phrase finale indique où vérifier l'état du service et varie selon le fournisseur. Les configurations Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry nomment l'état du service de ce fournisseur. Une `ANTHROPIC_BASE_URL` personnalisée nomme l'hôte de la passerelle.

Cela indique une défaillance inattendue à l'intérieur de l'API. Elle n'est pas causée par votre prompt, vos paramètres ou votre compte.

**Que faire :**

* Vérifiez [status.claude.com](https://status.claude.com), ou la page d'état du fournisseur nommée dans le message, pour les incidents actifs
* Attendez une minute, puis renvoyez votre message. Votre message original est toujours dans la conversation, donc pour un long prompt vous pouvez taper `try again` au lieu de coller le tout.
* Si l'erreur persiste sans incident signalé, exécutez `/feedback` pour qu'Anthropic puisse enquêter avec les détails de votre demande. Voir [Signaler une erreur](#report-an-error) si `/feedback` n'est pas disponible dans votre environnement.

<h3 id="api-error-repeated-529-overloaded-errors">
  Erreur API : Erreurs 529 Overloaded répétées
</h3>

L'API est temporairement à capacité pour tous les utilisateurs. Claude Code a déjà réessayé plusieurs fois avant d'afficher ce message :

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

La phrase finale varie selon le fournisseur de la même manière que l'erreur 500 ci-dessus.

Un 529 n'est pas votre limite d'utilisation et ne compte pas contre votre quota.

**Que faire :**

* Vérifiez [status.claude.com](https://status.claude.com), ou la page d'état du fournisseur nommée dans le message, pour les avis de capacité
* Réessayez dans quelques minutes
* Exécutez `/model` et basculez vers un modèle différent pour continuer à travailler, car la capacité est suivie par modèle. Claude Code vous invite à le faire quand un modèle est sous une charge particulièrement élevée, par exemple `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Délai d'attente de la demande dépassé
</h3>

L'API n'a pas répondu avant la date limite de connexion.

```text theme={null}
Request timed out
```

Cela peut se produire pendant les périodes de charge élevée ou quand le modèle génère une réponse très volumineuse. Le délai d'attente de demande par défaut est de 10 minutes.

**Que faire :**

* Réessayez la demande
* Pour les tâches longues, divisez le travail en prompts plus petits
* Si une connexion réseau lente ou un proxy en est la cause, augmentez `API_TIMEOUT_MS` comme décrit dans [Tentatives automatiques](#automatic-retries)
* Si les délais d'attente sont fréquents et votre réseau est par ailleurs sain, voir [Erreurs réseau et de connexion](#network-and-connection-errors) ci-dessous

<h3 id="no-response-from-api">
  Aucune réponse de l'API
</h3>

Claude Code a envoyé une demande de streaming et l'API n'a retourné aucun en-tête de réponse avant la date limite du premier octet, donc Claude Code a annulé la demande au lieu d'attendre le délai d'attente de demande `API_TIMEOUT_MS` complet, 10 minutes par défaut. Claude Code renvoie la demande au maximum une fois, si le [budget de tentatives](#tune-retry-behavior) le permet. Quand la tentative de réessai reste sans réponse, le tour se termine par ce message, qui montre combien de temps chaque tentative a attendu. Quand vous définissez [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/fr/env-vars), le plafond d'une tentative ne s'applique pas et Claude Code réessaie selon le budget décrit dans [Ajuster le comportement de tentative](#tune-retry-behavior).

```text theme={null}
API Error: No response from API (waited 3m, then 10m on the retry). If a proxy or gateway on your network holds responses until they complete, raise API_TIMEOUT_MS or CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS to wait longer.
```

Claude Code définit le délai d'attente des en-têtes de réponse de la première tentative et le délai d'attente de la tentative de réessai séparément :

* **Première tentative** : [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/fr/env-vars) quand vous le définissez à 1 ou plus, limité entre 10 secondes et 30 minutes. Sinon, Claude Code utilise le délai d'attente du watchdog au niveau des octets listé dans [Watchdogs d'inactivité de streaming](/docs/fr/network-config#streaming-idle-watchdogs), donc les variables qui changent ce délai d'attente changent aussi cette attente. De toute façon, Claude Code ajoute une seconde pour chaque 32 Ko de corps de demande.
* **Tentative de réessai** : une seconde de moins que `API_TIMEOUT_MS`, juste sous 10 minutes par défaut, afin que la tentative de réessai puisse dépasser une passerelle qui retient la réponse jusqu'à ce que la génération soit terminée. Sur Amazon Bedrock, la tentative de réessai utilise la même date limite que la première tentative, et le message affiche une durée au lieu de deux.

Aucune attente ne dépasse une seconde de moins qu'un `API_TIMEOUT_MS` positif, et un `API_TIMEOUT_MS` positif inférieur à 11 secondes désactive la date limite. Le watchdog au niveau des octets ne démarre qu'une fois les en-têtes de réponse arrivés, donc une réponse qui arrête d'envoyer des octets après cela suit les [règles de flux interrompu](#automatic-retries) au lieu de cette date limite.

**Que faire :**

* Renvoyez votre message. Votre message original est toujours dans la conversation, donc pour un long prompt vous pouvez taper `try again` au lieu de coller le tout.
* Si cela se répète, traitez-le comme un [problème réseau ou proxy](#unable-to-connect-to-api). Un proxy qui accepte la connexion et ne transfère jamais la demande produit cette erreur à chaque tentative.
* Si une passerelle ou un proxy sur votre réseau retient les réponses jusqu'à ce qu'elles soient terminées, augmentez `API_TIMEOUT_MS` afin que la tentative de réessai attende plus longtemps. Sur Amazon Bedrock, augmentez également `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`.
* Si la première tentative continue à expirer et que la tentative de réessai réussit ensuite, augmentez `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` afin que la première tentative attende assez longtemps aussi.

Avant v2.1.242, Claude Code attendait le délai d'attente de demande `API_TIMEOUT_MS` complet, 10 minutes par défaut, avant d'échouer une demande de streaming sans réponse. Avant v2.1.261, la tentative de réessai attendait la même date limite que la première tentative et le message n'affichait aucune durée.

<h3 id="the-response-above-may-be-incomplete">
  La réponse ci-dessus peut être incomplète
</h3>

Une demande de streaming a échoué alors que la réponse était toujours en cours, après que Claude ait terminé un bloc de texte ou un appel d'outil, ou en ait commencé un après avoir terminé sa réflexion. Le renvoi de la demande pourrait exécuter les mêmes appels d'outil deux fois, donc Claude Code conserve la sortie que Claude a terminée et ajoute cet avis au lieu de rejeter le tour. La variante que vous voyez nomme la cause :

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection lost mid-response. The response above may be incomplete.
API Error: Your computer went to sleep mid-response. The response above may be incomplete.
API Error: The response stopped arriving. The response above may be incomplete.
```

* `Server error mid-response` : une erreur serveur surchargée ou 5xx en milieu de flux. Cette variante nécessite Claude Code v2.1.199 ou ultérieur ; avant cela, ce cas rejetait la sortie partielle et signalait le tour entier comme une erreur.
* `Connection lost mid-response` : la connexion a été interrompue.
* `Your computer went to sleep mid-response` : Claude Code a détecté que votre ordinateur s'est endormi pendant que la réponse était en streaming. Une fois que votre ordinateur se réveille, Claude Code traite la connexion comme cassée et arrête de la lire.
* `The response stopped arriving` : la connexion est restée ouverte mais a arrêté de livrer des données, donc le watchdog d'inactivité de streaming l'a annulée. Avant v2.1.222, Claude Code pouvait également signaler cet échec sur les connexions de [passerelle](/docs/fr/gateways) atteintes via `ANTHROPIC_BASE_URL` ou `ANTHROPIC_AWS_BASE_URL` tandis que les pings de maintien de connexion du serveur arrivaient toujours, car il ne comptait que les événements de réponse analysés là ; la mise à niveau arrête ces faux positifs sur ces routes. Les passerelles atteintes via une URL de base de fournisseur telle que `ANTHROPIC_BEDROCK_BASE_URL` ne sont pas enveloppées par le watchdog d'octet ; voir [Watchdogs d'inactivité de streaming](/docs/fr/network-config#streaming-idle-watchdogs).

Avant v2.1.227, `Connection lost mid-response` lisait `Connection closed mid-response` et `The response stopped arriving` lisait `Response stalled mid-stream`.

Dans quatre cas, Claude Code gère l'échec sans afficher cet avis immédiatement :

* Plus tôt dans la réponse, Claude Code réessaie l'échec ou termine le tour avec une erreur différente. Voir [Tentatives automatiques](#automatic-retries).
* Quand l'un de ces échecs arrive après que Claude ait terminé la réponse, Claude Code conserve la réponse complète et termine le tour normalement, sans cet avis. Avant v2.1.222, Claude Code affichait cet avis quand la connexion était interrompue ou bloquée après la fin de la réponse, et signalait le tour comme une erreur même si la réponse était complète.
* Dans une [session non-interactive](/docs/fr/headless), comme une exécution `-p`, une exécution [Agent SDK](/docs/fr/agent-sdk/overview), ou une [session cloud](/docs/fr/claude-code-on-the-web), vous n'avez pas à envoyer `continue` vous-même quand la réponse coupée est dans la conversation principale et contient du texte mais pas d'appels d'outil : Claude Code conserve la sortie partielle et invite Claude à continuer à partir d'où il s'est arrêté, jusqu'à trois fois de suite. Vous voyez cet avis pour une telle réponse seulement une fois que Claude Code a épuisé ces continuations. Avant v2.1.246, Claude Code terminait un tour non-interactif avec cet avis à la première coupure.
* Dans un [sous-agent](/docs/fr/sub-agents#api-errors-in-subagents), que la session soit interactive ou non : quand sa réponse coupée contient du texte mais pas d'appels d'outil, Claude Code invite le sous-agent à continuer. L'avis devient le dernier message du sous-agent seulement une fois que ces continuations sont épuisées. Avant v2.1.257, un sous-agent affichait cet avis à la première coupure.

**Que faire :**

* Dans une session interactive, lisez la réponse qui reste à l'écran : Claude Code conserve chaque bloc que Claude a terminé avant l'erreur, mais rejette un bloc final interrompu quand le tour se termine, donc les dernières phrases ou appels d'outil peuvent manquer. Répondez avec `continue` pour que Claude reprenne à partir de son dernier bloc terminé.
* En [mode non-interactif](/docs/fr/headless) (`-p`) :
  * Avec la sortie texte par défaut, Claude Code imprime le dernier bloc de texte terminé qu'il détient toujours du début du tour, suivi de ce message. Quand il n'en détient aucun, Claude Code imprime ce message seul, par exemple parce que Claude Code a compacté la conversation en milieu de tour et a effacé ce texte. Avant v2.1.219, Claude Code n'imprimait que ce message dans la sortie texte `-p` et rejetait la réponse qu'il avait déjà produite.
  * Avec `--output-format json` ou `stream-json`, Claude Code signale ce message dans le champ `result`.
  * Pour continuer le tour une fois la connexion stable, reprenez la session et envoyez `continue` comme décrit dans [Continuer les conversations](/docs/fr/headless#continue-conversations).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Le mode auto ne peut pas déterminer la sécurité d'une action
</h3>

Le modèle que le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) utilise pour classifier les actions n'a pas pu produire une décision, donc le mode auto n'a pas approuvé l'action automatiquement. Le message que vous voyez dépend de la façon dont le classificateur a échoué.

Les lectures, recherches et modifications à l'intérieur de votre répertoire de travail ignorent le classificateur, donc elles continuent à fonctionner dans tous ces cas.

Quand le modèle de classification est indisponible :

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait a moment and then try this action again.
```

Quand Claude Code peut déterminer la catégorie d'échec, il nomme la catégorie entre parenthèses après `temporarily unavailable`, par exemple `<model> is temporarily unavailable (rate-limited), so auto mode cannot determine the safety of <tool> right now`. Les catégories sont `(rate-limited)`, `(overloaded)`, `(server error)`, `(timed out)`, et `(connection failed)`. Les limites de débit, les surcharges et les erreurs serveur sont transitoires, et les tentatives fonctionnent. Si `(timed out)` ou `(connection failed)` se répète, vérifiez votre connexion ; voir [Impossible de se connecter à l'API](#unable-to-connect-to-api). Avant v2.1.229, le message ne nommait jamais une catégorie et lisait `Wait briefly and then try this action again`.

Quand aucune catégorie ne convient, le message apparaît sans catégorie entre parenthèses ; plus d'un échec produit cette forme. Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), y compris le [point de terminaison Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint), il apparaît également quand votre compte AWS ne peut pas invoquer le modèle nommé dans le message, et cet échec se répète à chaque tentative jusqu'à ce que votre compte soit autorisé à accéder au modèle.

**Que faire :**

* Réessayez après quelques secondes ; Claude voit le même message et réessaie généralement de lui-même. Un échec transitoire n'est pas lié à l'[admissibilité du mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) ; vous n'avez pas besoin de modifier les paramètres
* Si les tentatives continuent à échouer, continuez avec les tâches en lecture seule et revenez à l'action bloquée plus tard
* Sur Amazon Bedrock, si le message revient à chaque tentative, vérifiez que votre compte peut invoquer le modèle qu'il nomme : pour les modèles Amazon Bedrock standard, confirmez que votre [politique IAM](/docs/fr/amazon-bedrock#iam-configuration) permet de l'invoquer ; pour les ID de modèle Mantle, [contactez votre équipe de compte AWS](/docs/fr/amazon-bedrock#mantle-endpoint-errors)

Quand une demande de classificateur échoue parce que votre jeton OAuth a expiré ou a été pivoté par une autre session, Claude Code actualise le jeton et réessaie la demande une fois, donc une expiration de jeton de routine ne fait pas surface comme ce message. Avant v2.1.216, un jeton expiré ou pivoté échouait à chaque demande de classificateur, et le mode auto refusait chaque action vérifiée avec ce message jusqu'à ce que le jeton soit actualisé.

Quand le classificateur a retourné une réponse non analysable :

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Que faire :**

* Réessayez l'action ; cela réussit généralement à la tentative suivante
* Exécutez `claude --debug` et répétez l'action pour voir la réponse du classificateur sous-jacente dans le journal de débogage

Quand une vérification de sécurité API distincte a bloqué la demande du classificateur en raison du contenu de la conversation antérieure :

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

Claude Code refuse l'action mais dit à Claude que ce n'est pas un jugement que l'action est dangereuse, et de continuer avec d'autres tâches plutôt que de réessayer. Ces refus ne comptent pas vers les [seuils de pause du mode auto](/docs/fr/permission-modes#when-auto-mode-falls-back). Dans une exécution `-p` [non-interactive](/docs/fr/headless), Claude Code n'arrête pas l'exécution. Ce que Claude reçoit dépend de l'endroit où il a demandé l'action :

* À un [sous-agent en arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) dans une exécution `-p` sans `--input-format stream-json`, Claude Code retourne un résultat d'erreur contenant `Agent aborted: auto mode classifier request refused by the safety safeguard in headless mode`
* Partout ailleurs, y compris les sessions interactives et la conversation principale d'une exécution `-p`, Claude Code retourne ce refus à Claude

Avant v2.1.225, Claude Code comptait ces refus vers les seuils de pause et retournait le même message de rejet qu'un bloc de classificateur authentique.

**Que faire :**

* Ce n'est pas une décision concernant votre action. Le contenu déjà dans votre conversation a déclenché un filtre de sécurité sur l'API quand le mode auto a envoyé la conversation au classificateur
* Réessayer ne servira à rien ; le même contenu de conversation déclenchera le filtre à nouveau
* Dans une session interactive, basculez vers un [mode de permission](/docs/fr/permission-modes) différent afin de pouvoir approuver l'action quand vous y êtes invité
* Commencez une nouvelle conversation sans le contenu déclencheur

Quand la conversation a grandi plus que la fenêtre de contexte du classificateur :

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

Ce qui se passe à l'action dépend de l'endroit où Claude l'a demandée :

* Dans une session interactive, le mode auto revient à une invite de permission normale pour cette action afin que vous puissiez l'approuver ou la refuser manuellement
* À un [sous-agent en arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) dans une exécution `-p` [non-interactive](/docs/fr/headless) sans `--input-format stream-json`, Claude Code retourne un résultat d'erreur contenant `Agent aborted: auto mode classifier transcript exceeded context window in headless mode`, et l'exécution continue
* Ailleurs dans une exécution `-p` sans [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags), il n'y a pas d'invite pour revenir, donc l'action ne s'exécute pas et l'exécution continue

**Que faire :**

* Dans une session interactive, approuvez ou refusez l'action dans l'invite qui apparaît
* Dans une session interactive, exécutez `/compact` pour réduire la taille de la conversation afin que les actions suivantes s'adaptent à nouveau à la fenêtre du classificateur

<h3 id="the-server-returned-no-safety-verdict">
  Le serveur n'a retourné aucun verdict de sécurité
</h3>

Sous [examen du classificateur côté serveur](/docs/fr/permission-modes#server-side-classifier-review), le mode auto refuse une action quand le serveur ne donne aucun verdict pour elle. Le refus nomme une catégorie entre parenthèses quand Claude Code peut en déterminer une, comme `(timed out)` :

```text theme={null}
The server-side auto mode classifier gave no verdict (timed out), so auto mode cannot determine the safety of <tool>.
```

Le reste du message dit à Claude si une tentative peut aider. Avant certains de ces refus, Claude Code attend afin que la tentative suivante de Claude ne suive pas immédiatement. Pendant l'attente dans une session interactive, le spinner affiche `Auto mode check unavailable` avec un compte à rebours, et appuyer sur `Esc` interrompt le tour.

Après dix réponses de suite sans verdict, le mode auto arrête le tour :

```text theme={null}
Auto mode is unavailable — the server returned no safety verdict for the last 10 responses, so Claude stopped. Send a message to try again, or switch out of auto mode.
```

Le message d'arrêt apparaît à un endroit différent dans chaque type de session :

* Dans une session interactive, le message apparaît comme un avertissement dans la transcription et le tour se termine
* Dans une exécution `-p` [non-interactive](/docs/fr/headless), l'exécution se termine et signale une erreur d'exécution. Avec la sortie texte par défaut, le message s'imprime sur stderr.
* Quand un [sous-agent](/docs/fr/sub-agents) a atteint la limite, le sous-agent s'arrête avant de terminer, et Claude reçoit ce qu'il a produit avec une note que le mode auto l'a arrêté

**Que faire :**

* Envoyez un autre message pour que Claude réessaie. Le compte de réponses recommence.
* Si l'arrêt se répète et que vos demandes passent par une [passerelle LLM ou un proxy](/docs/fr/llm-gateway), vérifiez s'il coupe les réponses de streaming ou les réécrit. [L'examen du classificateur côté serveur](/docs/fr/permission-modes#server-side-classifier-review) indique quel comportement de passerelle cause les refus, et le [guide de compatibilité de passerelle](/docs/fr/llm-gateway-protocol#feature-pass-through) liste ce qu'il faut transmettre inchangé.
* Définissez `CLAUDE_CODE_AUTO_MODE_SERVER=0` avant de démarrer Claude Code pour utiliser ses propres demandes de classificateur à la place. Avant v2.1.281, Claude Code ne lisait pas la variable sur une connexion directe à l'API Anthropic.
* Pour approuver les actions vous-même à la place, [basculez hors du mode auto](/docs/fr/permission-modes#switch-permission-modes)

Avant v2.1.280, Claude Code refusait chaque action d'une réponse sans verdict immédiatement et n'arrêtait jamais le tour.

<h3 id="agent-terminated-early-due-to-an-api-error">
  L'agent s'est arrêté prématurément en raison d'une erreur API
</h3>

Une demande API d'un [sous-agent](/docs/fr/sub-agents) a échoué de manière terminale, par exemple parce qu'une limite d'utilisation a été atteinte ou que les tentatives pour une erreur serveur ont épuisé, donc le sous-agent s'est arrêté avant de terminer sa tâche. Ce message nécessite Claude Code v2.1.199 ou ultérieur ; avant cela, le texte d'erreur API était retourné à Claude comme s'il s'agissait du résultat du sous-agent.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Que faire :**

* Faites correspondre le détail d'erreur après les deux points à sa propre section sur cette page, comme [Limites d'utilisation](#usage-limits) ou [Erreurs serveur](#server-errors), et suivez les étapes de cette section
* Une fois que l'erreur sous-jacente est résolue, demandez à Claude de réessayer la tâche ou de [reprendre le sous-agent](/docs/fr/sub-agents#resume-subagents)

Quand une limite de débit, une surcharge ou une erreur serveur interrompt un sous-agent au premier plan qui a déjà produit une sortie texte, Claude reçoit cette sortie partielle marquée comme incomplète au lieu de cette erreur. Un sous-agent dont la seule sortie était des appels d'outil reçoit également cette erreur ; dans v2.1.199 cette forme retournait un résultat partiel vide à la place. Voir [Erreurs API dans les sous-agents](/docs/fr/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Limites d'utilisation
</h2>

La plupart des erreurs de cette section signifient qu'un quota lié à votre compte ou à votre plan a été atteint. Trois fonctionnent différemment : [`Server is temporarily limiting requests`](#server-is-temporarily-limiting-requests) est un throttle côté serveur sans rapport avec votre quota de plan, [`Usage credits required for 1M context`](#usage-credits-required-for-1m-context) est une vérification de droit plutôt qu'un quota épuisé, et [`The prompt to confirm went unanswered`](#the-prompt-to-confirm-went-unanswered) signifie qu'une invite de consentement pour les crédits d'utilisation s'est fermée sans réponse, que le quota ait été atteint ou non.

<h3 id="youve-hit-your-session-limit">
  You've hit your session limit
</h3>

Les plans d'abonnement incluent une allocation d'utilisation continue. Quand elle s'épuise, vous voyez l'un de ces messages :

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
You've hit your Sonnet limit · resets 3:45pm
```

Claude Code bloque les demandes supplémentaires jusqu'à l'heure de réinitialisation indiquée dans le message. Les limites de session et hebdomadaires sont partagées entre tous les modèles, donc changer de modèle ne restaure pas l'accès. Les limites Opus et Sonnet s'appliquent chacune uniquement aux demandes adressées à cette famille de modèles, donc passer à un modèle en dehors de la famille avec `/model` vous permet de continuer à travailler.

Dans une session interactive connectée avec un abonnement claude.ai, Claude Code peut également attendre dans la session ouverte et continuer la tâche interrompue peu après la réinitialisation. Pendant qu'il attend, une ligne au bas de la session indique `Usage limit reached · continuing automatically at 3:45pm · esc to cancel`. Appuyez sur `Esc` à une invite vide pour annuler l'attente. Consultez [Wait for a usage limit to reset](/docs/fr/interactive-mode#wait-for-a-usage-limit-to-reset) pour voir ce que vous voyez, comment démarrer ou annuler une attente, et comment désactiver la continuation automatique. Avant la v2.1.234, Claude Code n'offrait pas cette attente.

L'utilisation compte à la fois pour les allocations de session et hebdomadaires. Une seule rafale d'activité intensive, comme un grand fanout de flux de travail, peut épuiser l'allocation hebdomadaire avant que la fenêtre de session ne se réinitialise.

**Ce qu'il faut faire :**

* Attendez l'heure de réinitialisation indiquée dans l'erreur
* Dans l'onglet Code de l'[application de bureau](/docs/fr/desktop), la carte de limite de session offre une case à cocher **Auto-continue when limits reset**. La carte de limite hebdomadaire ne l'offre pas. Quand elle est cochée, l'application de bureau réessaie le tour interrompu après la réinitialisation et affiche l'heure de nouvelle tentative sur la carte. La case à cocher de l'application de bureau et le paramètre **Continue automatically at usage limit** de la CLI dans `/config` sont séparés, donc désactivez chacun indépendamment.
* Pour la limite Opus ou Sonnet, exécutez `/model` et basculez vers un modèle en dehors de cette famille pour continuer à travailler. Chaque modèle a son propre cache de prompt, donc la demande suivante relit toute la conversation sans accès au cache ; consultez [Switching models](/docs/fr/prompt-caching#switching-models)
* Exécutez `/usage` pour voir vos limites de plan et quand elles se réinitialisent
* Exécutez `/usage-credits` pour acheter une utilisation supplémentaire sur Pro et Max, ou pour la demander à votre administrateur sur Team et Enterprise. Consultez [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) pour savoir comment cela est facturé.
* Pour mettre à niveau votre plan pour des limites de base plus élevées, consultez [claude.com/pricing](https://claude.com/pricing)

Avant qu'une fenêtre ne s'épuise, Claude Code peut vous avertir que vous avez utilisé la plupart de celle-ci, avec un message tel que `You've used 85% of your session limit · resets 3:45pm`. Pour surveiller votre allocation restante en continu, ajoutez les champs `rate_limits` à une [ligne d'état personnalisée](/docs/fr/statusline#rate-limit-usage), ou dans l'application de bureau, cliquez sur l'[anneau d'utilisation](/docs/fr/desktop#check-usage) à côté du sélecteur de modèle.

<h3 id="usage-credits-required-for-1m-context">
  Usage credits required for 1M context
</h3>

Le modèle sélectionné utilise la fenêtre de contexte étendue de 1M tokens, et votre plan ne l'inclut que via les crédits d'utilisation.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on (they take effect after you restart Claude Code), or /model to switch to standard context
```

C'est une vérification de droit, pas un épuisement de quota. Elle se déclenche même quand vos allocations de session et hebdomadaires ont de la capacité restante. Consultez [Extended context](/docs/fr/model-config#extended-context) pour voir quels plans incluent le contexte 1M directement et lesquels nécessitent des crédits d'utilisation. Claude Code exécute cette vérification quand vous choisissez le modèle avec `/model`, et uniquement sur une connexion directe à l'API Anthropic ; si vous pointez `ANTHROPIC_BASE_URL` vers une [passerelle LLM](/docs/fr/llm-gateway), `/model` permet la sélection `[1m]` et la passerelle décide si la demande réussit.

Quand cette erreur apparaît au milieu d'une conversation parce que le contexte a dépassé 200K tokens, Claude Code compacte automatiquement la conversation en dessous de la limite de contexte standard et maintient la session à cette limite par la suite, donc aucune action n'est nécessaire. Sur les versions antérieures à v2.1.172, l'erreur s'est répétée à chaque demande suivante, y compris `/compact` ; exécutez `/clear` sur ces versions pour récupérer. Les étapes ci-dessous s'appliquent quand vous avez explicitement sélectionné un modèle `[1m]`.

**Ce qu'il faut faire :**

* Exécutez `/model` et sélectionnez la variante sans le suffixe `[1m]` pour revenir à la fenêtre de contexte standard
* Où le message nomme `/usage-credits`, exécutez-le pour activer la facturation à l'usage pour la variante 1M sur Pro et Max, ou pour demander des crédits d'utilisation à votre administrateur sur Team et Enterprise. Une fois que les crédits d'utilisation sont activés, redémarrez Claude Code ou démarrez une nouvelle session, selon ce que le message indique. Jusqu'à ce moment, la session reste à la limite de contexte standard.
* Si l'erreur persiste après `/model`, un ID de modèle 1M peut être défini ailleurs. Consultez [Setting your model](/docs/fr/model-config#setting-your-model) pour les emplacements de configuration à vérifier par ordre de priorité.
* Pour supprimer complètement les variantes 1M du sélecteur de modèle, définissez [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/fr/env-vars)

Avant v2.1.268, le message se terminait par `run /usage-credits to turn them on, or /model to switch to standard context` et ne mentionnait pas le redémarrage.

<h3 id="the-prompt-to-confirm-went-unanswered">
  The prompt to confirm went unanswered
</h3>

Si votre compte nécessite le [consentement des crédits d'utilisation Fable](/docs/fr/model-config#fable-and-usage-credits), Claude Code vous demande de confirmer avant qu'une demande Fable ne facture les crédits d'utilisation. Quand personne ne répond à cette invite de consentement dans une session qui peut ne pas avoir quelqu'un à son terminal, Claude Code ferme l'invite et termine le tour avec l'un de ces messages :

```text theme={null}
Fable limit reached · continuing on Fable 5.1 uses usage credits, and the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
Fable 5.1 now uses usage credits · the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
```

Les messages nomment le modèle Fable de la session, donc sur Fable 5, ils lisent `continuing on Fable 5` et `Fable 5 now uses usage credits`. Avant v2.1.257, le premier message commençait par `Fable 5 limit reached`.

Cela se produit dans les sessions [Remote Control](/docs/fr/remote-control), les [sessions en arrière-plan](/docs/fr/agent-view), et les sessions de coéquipiers [agent team](/docs/fr/agent-teams). Claude Code affiche l'invite de consentement uniquement dans la vue interactive de la session : le terminal où elle s'exécute, ou, pour une session en arrière-plan, la [vue des agents](/docs/fr/agent-view) une fois que vous vous attachez. Un client Remote Control ne peut pas l'afficher. Claude Code ferme l'invite à la date limite [`dialogExpiry`](/docs/fr/settings-reference#dialogexpiry), cinq minutes par défaut, ou dès qu'une nouvelle invite arrive alors que personne n'a tapé à ce terminal, comme une invite envoyée par un client Remote Control. Taper au terminal où la session s'exécute annule la date limite, et Claude Code attend votre réponse. Dans la vue attachée d'une session en arrière-plan, taper n'annule pas la date limite, et une nouvelle invite ferme toujours l'invite de consentement, donc répondez avant que l'un ou l'autre ne se produise. Claude Code n'envoie rien et conserve votre modèle, donc quand vous envoyez votre prochaine invite, Claude Code affiche à nouveau l'invite de consentement.

**Ce qu'il faut faire :**

* Au terminal où la session s'exécute, envoyez une autre invite et répondez à l'invite de consentement quand elle réapparaît. Pour une session en arrière-plan, attachez-vous d'abord à partir de la [vue des agents](/docs/fr/agent-view). Renvoyer depuis un client Remote Control affiche ce message à nouveau, car le client ne peut pas afficher l'invite.
* Exécutez `/model` pour basculer vers un modèle qui ne facture pas les crédits d'utilisation
* Pour vous donner plus de temps pour atteindre ce terminal, définissez [`dialogExpiry`](/docs/fr/settings-reference#dialogexpiry) sur une valeur plus longue ou `"never"`

Avant v2.1.236, ce message n'apparaissait pas : pendant qu'un client Remote Control était connecté, Claude Code attendait 60 secondes une réponse, puis continuait le tour sur votre modèle par défaut.

<h3 id="server-is-temporarily-limiting-requests">
  Server is temporarily limiting requests
</h3>

L'API a appliqué un throttle de courte durée sans rapport avec votre quota de plan.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code les distingue de votre limite de plan par l'absence des en-têtes de quota unifiés qu'une réponse de limite réelle porte. À partir de v2.1.199, ceci est [réessayé automatiquement](#automatic-retries) avec backoff avant d'être affiché, quelle que soit votre méthode d'authentification. Sur les versions antérieures, une session connectée avec un abonnement claude.ai échouait le tour à la première occurrence ; seules les connexions par clé API et Enterprise le réessayaient.

**Ce qu'il faut faire :**

* Attendez brièvement et réessayez
* Vérifiez [status.claude.com](https://status.claude.com) si cela persiste

<h3 id="request-rejected-429">
  Request rejected (429)
</h3>

Vous avez atteint la limite de débit configurée pour votre clé API, votre projet Amazon Bedrock ou votre projet Google Cloud.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

La phrase finale nomme où vérifier la santé du service et varie selon le fournisseur. Les configurations Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry nomment le statut du service de ce fournisseur au lieu de la page de statut Anthropic. Un `ANTHROPIC_BASE_URL` personnalisé nomme l'hôte de la passerelle.

**Ce qu'il faut faire :**

* Exécutez `/status` et confirmez que les identifiants actifs sont ceux que vous attendez. Un `ANTHROPIC_API_KEY` égaré dans votre environnement peut acheminer les demandes via une clé de niveau inférieur au lieu de votre abonnement.
* Vérifiez votre console de fournisseur pour les limites actives et demandez un niveau supérieur si nécessaire
* Pour les clés API Anthropic, consultez la [référence des limites de débit](https://platform.claude.com/docs/en/api/rate-limits) pour savoir comment fonctionnent les niveaux et comment définir des plafonds par espace de travail
* Réduisez la concurrence : abaissez [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/fr/env-vars), évitez d'exécuter de nombreux sous-agents parallèles, ou basculez vers un modèle plus petit avec `/model` pour les exécutions scriptées à haut volume

<h3 id="youve-hit-your-monthly-spend-limit">
  You've hit your monthly spend limit
</h3>

L'utilisation incluse de votre plan ne peut pas couvrir cette demande, et les [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) qui paieraient autrement pour cela ont atteint une limite de dépenses. Cela se produit quand l'une des fenêtres d'utilisation de votre plan s'est épuisée, ou quand la demande est une demande que seuls les crédits d'utilisation paient, comme une demande à un modèle qui [facture aux crédits d'utilisation](/docs/fr/model-config#fable-and-usage-credits). Le message nomme la limite qui vous a bloqué. Le texte après le `·` indique comment augmenter cette limite, et varie selon votre plan et si vous gérez la facturation :

```text theme={null}
You've hit your monthly spend limit · raise it at claude.ai/settings/usage
You've hit your individual spend limit · ask your admin for a higher limit
You've hit your org's monthly spend limit · visit claude.ai/admin-settings/usage to raise it
You've hit your team's shared budget · ask your admin to raise it at claude.ai/admin-settings/usage
You've hit your channel's monthly spend limit · an org owner or channel manager can raise it in the channel's Claude settings
```

`team's shared budget` est un budget groupé qu'un administrateur a attribué à un groupe auquel vous appartenez ; le message ne nomme pas le groupe. `channel's monthly spend limit` est le budget du seul canal Slack dans lequel la session s'exécute, donc votre organisation peut toujours avoir un budget en dehors de celui-ci.

Quand l'une des fenêtres de votre plan est ce qui s'est épuisé, le message indique également quand cette fenêtre se réinitialise, par exemple `· your session limit resets 3:45pm`, et l'accès revient alors sans que personne n'augmente la limite. Sur les organisations avec facturation basée sur l'utilisation, le message dit `usage limit` à la place de `spend limit`, comme dans `You've hit your individual usage limit`.

Avant v2.1.239, le message ne nommait pas l'heure de réinitialisation de la fenêtre du plan. Avant v2.1.268, le budget groupé d'un groupe produisait le message `individual spend limit` au lieu de `team's shared budget`.

Si vous vous connectez via une passerelle d'applications Claude et voyez `spend limit reached` en minuscules, c'est le plafond de votre opérateur de passerelle ; consultez [Spend limit reached](#spend-limit-reached).

**Ce qu'il faut faire :**

* Sur Pro et Max, augmentez votre limite de dépenses mensuelles dans [**Settings > Usage**](https://claude.ai/settings/usage) sur claude.ai, ou exécutez `/usage-credits`
* Sur Team et Enterprise, augmentez la limite dans [**Admin settings > Usage**](https://claude.ai/admin-settings/usage) si vous gérez la facturation, ou demandez à un administrateur de le faire. `/usage-credits` envoie cette demande à votre administrateur pour vous
* Pour la limite d'un canal, demandez à un propriétaire d'organisation ou au gestionnaire du canal de l'augmenter sur claude.ai. Consultez [Per-channel limits](https://claude.com/docs/claude-tag/admins/set-spend-limit#per-channel-limits) dans la documentation Claude Tag
* Si le message nomme une heure de réinitialisation pour la fenêtre de votre plan, vous pouvez l'attendre à la place
* Exécutez `/usage` pour voir les fenêtres de votre plan et quand chacune se réinitialise

<h3 id="spend-limit-reached">
  Spend limit reached
</h3>

Vous vous connectez via une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) et avez dépassé un [plafond de dépenses](/docs/fr/claude-apps-gateway-spend-limits) que votre opérateur de passerelle a défini. La passerelle bloque vos demandes jusqu'à ce que la période nommée se réinitialise ou que l'opérateur augmente le plafond. Elle marque chaque réponse `429` bloquée `x-should-retry: false`, donc Claude Code affiche ce message sans réessayer.

```text theme={null}
spend limit reached (daily; resets 2026-08-09 00:00 UTC)
```

Le message nomme la période du plafond et l'heure de réinitialisation, et quand l'opérateur a configuré un `blocked_message`, ses instructions le suivent. Avant v2.1.225, le message lisait seulement `spend limit reached` ; une passerelle sur une version plus ancienne envoie toujours cette forme plus courte.

**Ce qu'il faut faire :**

* Attendez l'heure de réinitialisation que le message nomme, ou suivez les instructions de l'opérateur si le message les porte
* Demandez à votre opérateur de passerelle d'augmenter le plafond si vous le dépassez régulièrement

Un message connexe, `spend limit unavailable`, signifie que la passerelle n'a pas pu lire ses enregistrements de dépenses et a bloqué la demande par précaution plutôt que sur votre plafond. Cela s'efface généralement de lui-même ; si cela persiste, informez votre opérateur de passerelle.

<h3 id="credit-balance-is-too-low">
  Credit balance is too low
</h3>

Votre organisation Console a épuisé ses crédits prépayés, ou Claude Code envoie vos demandes avec une clé API Console quand vous aviez l'intention d'utiliser votre abonnement.

```text theme={null}
Credit balance is too low
```

**Ce qu'il faut faire :**

* Si vous avez un plan Pro, Max, Team ou Enterprise et voyez ceci, exécutez `/status` et vérifiez la ligne `API key`. Un `ANTHROPIC_API_KEY` approuvé dans votre environnement achemine les demandes via cette clé au lieu de votre abonnement. Désactivez-le dans le shell actuel et supprimez-le de votre profil de shell, puis relancez `claude`. Exécutez `/login` si vous ne vous êtes pas encore connecté avec votre abonnement.
* Ajoutez des crédits à [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), et envisagez d'activer le rechargement automatique là-bas pour que le solde se remplisse avant d'atteindre zéro
* Définissez des plafonds de dépenses par espace de travail dans la Console pour empêcher un seul projet de drainer le solde de l'organisation. Consultez [Manage costs effectively](/docs/fr/costs).

<h3 id="could-not-update-your-spend-limit">
  Could not update your spend limit
</h3>

Le serveur a rejeté un changement de limite de dépenses que vous avez effectué à partir de l'invite qui apparaît quand vous atteignez votre limite de dépenses.

```text theme={null}
Could not update your spend limit: <reason from the server>
```

Quand le serveur explique le rejet, le message se termine par cette raison, et réessayer la même valeur échoue à nouveau. Quand l'échec n'a pas de raison fournie par le serveur, comme une connexion interrompue, le message lit `Could not update your spend limit. Press Enter to retry.` et réessayer peut réussir. Avant v2.1.216, Claude Code affichait la forme générique pour chaque échec.

**Ce qu'il faut faire :**

* Si le message inclut une raison, choisissez une limite qui la satisfait, comme un montant inférieur
* Si le message affiche uniquement la forme générique, réessayez ; l'échec peut être transitoire
* Si le changement continue d'échouer, effectuez-le à partir de vos [paramètres de facturation claude.ai](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) dans le navigateur à la place

<h2 id="authentication-errors">
  Erreurs d'authentification
</h2>

Ces erreurs signifient que Claude Code ne peut pas prouver votre identité à l'API. Exécutez `/status` à tout moment pour voir quelle credential est actuellement active.

<h3 id="not-logged-in">
  Non connecté
</h3>

Aucune credential valide n'est disponible pour cette session.

```text theme={null}
Not logged in · Please run /login
```

**À faire :**

* Exécutez `/login` pour vous authentifier avec votre abonnement Claude ou votre compte Console
* Si vous vous attendiez à ce qu'une variable d'environnement vous authentifie, confirmez que `ANTHROPIC_API_KEY` est définie et exportée dans le shell où vous avez lancé `claude`
* Pour l'intégration continue ou l'automatisation où la connexion interactive n'est pas possible, configurez un script [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) qui récupère une clé au démarrage
* Consultez [Précédence d'authentification](/docs/fr/authentication#authentication-precedence) pour comprendre quelle credential Claude Code utilise quand plusieurs sont présentes

Si vous êtes invité à vous connecter à plusieurs reprises, consultez [Non connecté ou token expiré](/docs/fr/troubleshoot-install#not-logged-in-or-token-expired) pour les vérifications de l'horloge système et les étapes de récupération du stockage des credentials macOS.

<h3 id="could-not-resolve-authentication-method">
  Impossible de résoudre la méthode d'authentification
</h3>

La session a atteint le client API sans aucune credential. Les [sessions en arrière-plan](/docs/fr/agent-view) et les sessions cloud affichent ce message quand le worker démarre sans credential. Les exécutions interactives, `-p` et Agent SDK signalent la même condition que [Non connecté](#not-logged-in) et écrivent cette chaîne uniquement dans leur journal de débogage, donc si vous l'avez trouvée là, suivez cette entrée à la place.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

Sur les versions actuelles, l'erreur signifie qu'aucune credential n'était disponible pour le processus worker. Avant v2.1.174, une session en arrière-plan assignée à un worker pré-initialisé inactif pouvait échouer de cette façon même quand des credentials valides étaient configurées. Avant v2.1.176, une session cloud qui restait inactive avant d'être réclamée pouvait aussi. Mettez à jour pour récupérer.

**À faire :**

* Mettez à jour vers v2.1.176 ou ultérieur si cela apparaît dans une session en arrière-plan ou cloud et que vos credentials sont déjà configurées
* Confirmez que `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` ou vos credentials du fournisseur cloud sont définis dans l'environnement qui lance le worker, pas seulement dans votre shell interactif
* Pour l'Agent SDK, consultez [configuration de l'authentification dans le guide de démarrage](/docs/fr/agent-sdk/quickstart#setup)
* Exécutez `/status` dans une session interactive dans le même environnement pour confirmer quelle source de credential se résout

<h3 id="invalid-api-key">
  Clé API invalide
</h3>

La variable d'environnement `ANTHROPIC_API_KEY` ou le script `apiKeyHelper` a renvoyé une clé que l'API a rejetée, ou Claude Code a bloqué une clé de `ANTHROPIC_API_KEY` avant de l'envoyer.

```text theme={null}
Invalid API key · Fix external API key
```

Quand le message continue après `Fix external API key` avec une description telle que `Invalid X-Api-Key header value from ANTHROPIC_API_KEY: it contains a line break at character 41 (120 characters on 2 lines).`, l'API n'a jamais vu la clé. Claude Code a trouvé un caractère que les en-têtes HTTP ne peuvent pas transporter et a arrêté la requête avant de l'envoyer. Consultez [Valeur d'en-tête de requête invalide](#invalid-request-header-value) pour savoir comment lire la description et corriger la valeur.

**À faire :**

* Vérifiez les fautes de frappe et confirmez que la clé n'a pas été révoquée dans la [Console](https://platform.claude.com/settings/keys)
* Dans le même shell, exécutez `env | grep ANTHROPIC`, ou dans PowerShell `Get-ChildItem Env:ANTHROPIC*`. Des outils comme direnv, les plugins dotenv shell et les terminaux IDE peuvent charger une clé obsolète à partir d'un fichier `.env` dans votre projet sans que vous la définissiez explicitement.
* Déconfigurez `ANTHROPIC_API_KEY` et exécutez `/login` pour utiliser l'authentification par abonnement à la place
* Si la clé provient d'un script [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper), exécutez le script directement pour confirmer qu'il imprime une clé valide sur stdout
* Exécutez `/status` pour confirmer quelle source de credential Claude Code utilise réellement

<h3 id="your-apikeyhelper-script-is-failing">
  Votre script apiKeyHelper échoue
</h3>

Claude Code a exécuté la commande dans votre paramètre [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) et n'a pas obtenu de clé en retour. Sans une, la requête atteint l'API avec une credential d'espace réservé, et l'API la rejette avec `401`. Le panneau `Authentication` dans le terminal affiche lequel de ces événements s'est produit :

* La commande s'est terminée avec une erreur ou a expiré
* La commande n'a rien imprimé sur stdout
* La commande a imprimé quelque chose d'autre que la clé, comme une bannière de connexion ou une ligne de journal. Le panneau affiche `returned output that cannot be used as an API key` et indique ce qui ne va pas, sans répéter la sortie. Avant v2.1.227, Claude Code envoyait tout ce que la commande imprimait, après suppression des espaces blancs environnants.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

En [mode non-interactif](/docs/fr/headless), stderr porte également la raison spécifique, préfixée par `apiKeyHelper failed:`.

Claude Code réexécute le script et réessaie la requête jusqu'à deux fois de plus avant d'afficher ce message, donc l'échec apparaît dans les trois tentatives. Avant v2.1.208, Claude Code dépensait le [budget de retry](#automatic-retries) complet en renvoyant la requête avec la credential d'espace réservé, puis signalait une erreur d'authentification générique `401` au lieu de l'échec du script.

L'exécution de `/login` n'aide pas ici : la sortie du helper [prend la priorité](/docs/fr/authentication#authentication-precedence) sur une connexion enregistrée tant que le paramètre est présent.

**À faire :**

* Exécutez la commande configurée dans `apiKeyHelper` directement dans votre shell pour reproduire l'échec
* Si la commande signale une session expirée, réauthentifiez-vous auprès de votre fournisseur de credentials, par exemple en vous reconnectant à votre SSO ou à votre coffre-fort de secrets
* Corrigez la commande pour qu'elle imprime uniquement la clé sur stdout, en tant que jeton unique d'ASCII imprimable jusqu'à 16 384 caractères, et se termine avec le code 0. Consultez [rotation des credentials avec apiKeyHelper](/docs/fr/llm-gateway-connect#rotate-credentials-with-apikeyhelper) pour une configuration fonctionnelle.
* Exécutez `/status` pour voir l'échec et confirmez que `apiKeyHelper` est la source de credential active. La ligne `apiKeyHelper` affiche `Failing` avec le détail du dernier échec, comme le code de sortie et la sortie d'erreur de la commande, et disparaît après la prochaine exécution réussie. Avant v2.1.274, `/status` affichait uniquement la source de credential, pas l'échec.
* Chaque fois que la commande échoue, son code de sortie et sa sortie d'erreur apparaissent également dans un panneau `Authentication` dans le terminal. Avant v2.1.212, le panneau était intitulé `Cloud authentication`.

<h3 id="invalid-request-header-value">
  Valeur d'en-tête de requête invalide
</h3>

Une valeur que Claude Code s'apprêtait à envoyer en tant qu'en-tête de requête contient un caractère que les en-têtes HTTP ne peuvent pas transporter : un saut de ligne, un octet NUL ou un caractère au-dessus de `U+00FF`, comme un guillemet courbe ou un espace de largeur zéro. Claude Code arrête la requête avant que quoi que ce soit ne soit envoyé et nomme la variable ou le paramètre à corriger. La cause habituelle est une credential collée à partir d'un document ou d'une conversation qui portait un caractère invisible ou un saut de ligne égaré.

Claude Code exécute cette vérification quand il envoie des requêtes à l'API Claude directement ou via une [passerelle LLM](/docs/fr/llm-gateway). Sur un fournisseur cloud tiers comme [Amazon Bedrock](/docs/fr/amazon-bedrock), Claude Code ne l'exécute pas avant d'envoyer.

```text theme={null}
Invalid auth token · Fix external auth token
Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable
Invalid request header from the environment · Fix the environment variable
```

La première partie du message dépend de la provenance de la mauvaise valeur :

* `Invalid auth token` : un jeton bearer de [`ANTHROPIC_AUTH_TOKEN`](/docs/fr/env-vars) ou [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/fr/env-vars)
* `Invalid ANTHROPIC_CUSTOM_HEADERS` : un nom ou une valeur d'en-tête que vous avez défini dans [`ANTHROPIC_CUSTOM_HEADERS`](/docs/fr/env-vars). La description compte quelle paire `Name: Value` est en faute, comme `distinct header 2 of 3 parsed from ANTHROPIC_CUSTOM_HEADERS`, sans répéter le nom ou la valeur, puisque vous avez choisi les deux.
* `Invalid request header from the environment` : une valeur que Claude Code copie dans un en-tête de requête à partir d'une autre variable d'environnement, comme `CLAUDE_AGENT_SDK_CLIENT_APP`. La description nomme la variable à corriger.

Claude Code signale une mauvaise `ANTHROPIC_API_KEY` capturée par cette vérification comme [Clé API invalide](#invalid-api-key), avec la même description de fin. Il signale une mauvaise credential `/login` enregistrée comme [Non connecté](#not-logged-in) à la place ; exécutez `/login` pour en enregistrer une nouvelle. La sortie d'un script [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) n'atteint jamais cette vérification : Claude Code la valide quand le script s'exécute, et la sortie qu'un en-tête HTTP ne peut pas transporter échoue avec [Votre script apiKeyHelper échoue](#your-apikeyhelper-script-is-failing).

Après le deuxième `·`, le message décrit le problème, comme dans cet exemple complet :

```text theme={null}
Invalid auth token · Fix external auth token · Invalid Authorization header value from ANTHROPIC_AUTH_TOKEN: it contains a line break at character 41 (120 characters on 2 lines).
```

Les positions comptent les caractères en commençant par un. La description est construite à partir de phrases fixes et de comptages de caractères, donc elle n'inclut jamais la valeur elle-même. Elle nomme le caractère offensant uniquement quand il s'agit d'un caractère invisible ou typographique bien connu, comme une marque d'ordre des octets, un espace de largeur zéro ou un guillemet courbe, et signale tout le reste comme `a non-ASCII character`.

**À faire :**

* Redéfinissez la variable ou le paramètre que le message nomme, en retapant les caractères autour de la position signalée plutôt que de coller à partir de la même source
* Pour `ANTHROPIC_CUSTOM_HEADERS`, conservez une paire `Name: Value` par ligne et réécrivez la paire que le message compte
* Exécutez `/status` pour confirmer quelle source de credential est active

<h3 id="this-organization-has-been-disabled">
  Cette organisation a été désactivée
</h3>

Claude Code utilise une `ANTHROPIC_API_KEY` obsolète d'une organisation Console désactivée. Quand vous avez une connexion d'abonnement enregistrée, la clé la remplace.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your subscription instead
Your ANTHROPIC_API_KEY belongs to a disabled organization · Update or unset the environment variable
API Error: 400 ... This organization has been disabled.
```

L'indice après le `·` dépend de vos credentials enregistrées : la première forme apparaît quand une `/login` enregistrée peut prendre le relais après que vous ayez déconfiguré la clé, et la deuxième quand la clé est votre seule credential.

Les variables d'environnement prennent la priorité sur `/login`, donc une clé exportée dans votre profil shell ou chargée à partir d'un fichier `.env` est utilisée même quand vous avez un abonnement Pro ou Max fonctionnant. En mode non-interactif (`-p`), la clé est toujours utilisée quand elle est présente.

**À faire :**

* Déconfigurez `ANTHROPIC_API_KEY` dans le shell actuel et supprimez-la de votre profil shell, puis relancez `claude`
* Si le message dit `Update or unset`, vous n'avez pas de connexion enregistrée sur laquelle vous rabattre. Déconfigurez la clé et exécutez `/login`, ou remplacez la clé par une d'une organisation Console active.
* Exécutez `/status` après pour confirmer que la credential active est votre abonnement
* Si aucune variable d'environnement n'est définie et l'erreur persiste, l'organisation désactivée est celle liée à votre `/login`. Contactez le support ou connectez-vous avec un compte différent.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Votre organisation a désactivé l'authentification par clé API
</h3>

Ce message nécessite Claude Code v2.1.169 ou ultérieur. L'administrateur de votre organisation Console a désactivé l'authentification par clé API, donc l'API rejette la clé que Claude Code envoie. L'indice de récupération après le `·` varie selon la provenance de la clé :

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Les variables d'environnement et `apiKeyHelper` prennent la priorité sur `/login`, donc exécuter `/login` seul n'aide pas tant que l'un ou l'autre fournit toujours une clé. Consultez [Précédence d'authentification](/docs/fr/authentication#authentication-precedence).

**À faire :**

* Si le message nomme `ANTHROPIC_API_KEY`, déconfigurez-la dans le shell actuel et supprimez-la de votre profil shell ou fichier `.env`, puis relancez `claude`
* Si le message nomme `apiKeyHelper`, supprimez le paramètre [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) de votre `settings.json`
* Exécutez `/login` pour vous connecter avec votre compte claude.ai
* Exécutez `/status` après pour confirmer que la credential active est votre abonnement plutôt qu'une clé API
* Si vous avez besoin de l'authentification par clé API pour l'automatisation, demandez à l'administrateur de votre organisation de la réactiver dans la Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Votre organisation a désactivé l'accès à l'abonnement Claude
</h3>

Votre organisation Claude ne permet pas de se connecter à Claude Code avec une connexion d'abonnement. L'exécution de `/login` à nouveau avec le même compte retourne la même erreur.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

C'est un paramètre d'organisation côté serveur, donc il ne peut pas être remplacé à partir des paramètres locaux, des variables d'environnement ou des drapeaux CLI.

L'Agent SDK et le mode non-interactif `-p` présentent cela comme le code d'erreur `oauth_org_not_allowed`.

**À faire :**

* Demandez à votre administrateur d'activer l'accès à Claude Code pour votre organisation
* Authentifiez-vous avec une clé API Console au lieu de votre abonnement. Consultez [Authentification Claude Console](/docs/fr/authentication#claude-console-authentication) pour la configuration.
* Si vous êtes l'administrateur et ne voyez pas d'option pour activer l'accès, contactez le [support Anthropic](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Les routines sont désactivées par la politique de votre organisation
</h3>

Un propriétaire de votre organisation Team ou Enterprise a désactivé les routines au niveau de l'organisation. L'erreur apparaît quand vous essayez de créer ou d'exécuter une routine, par exemple à partir de l'interface utilisateur [Routines](/docs/fr/routines) sur claude.ai/code. Sur Claude Code v2.1.227 ou ultérieur, le même paramètre [masque également `/schedule`](/docs/fr/routines#troubleshooting) dans le CLI.

```text theme={null}
Routines are disabled by your organization's policy.
```

C'est un paramètre côté serveur, donc il ne peut pas être remplacé à partir des paramètres locaux, des variables d'environnement ou des drapeaux CLI.

**À faire :**

* Demandez à un propriétaire de votre organisation d'activer le bouton bascule **Routines** sur [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Pour un travail ponctuel programmé qui ne nécessite pas de routines au niveau de l'organisation, consultez [tâches programmées](/docs/fr/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control nécessite l'API Anthropic
</h3>

La session ne parle pas directement à l'API Anthropic, donc il n'y a pas de backend claude.ai pour que [Remote Control](/docs/fr/remote-control) s'apparie avec.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com. CLAUDE_CODE_USE_BEDROCK is set, so this session is using Amazon Bedrock — unset it (or run in a shell without it) to use Remote Control.
```

Une deuxième phrase explique ce qui a acheminé la session loin de l'API Anthropic ; avant v2.1.219, le message était la première phrase seule. Selon la cause, le message nomme :

* Une variable de fournisseur `CLAUDE_CODE_USE_*`, comme `CLAUDE_CODE_USE_BEDROCK` pour [Amazon Bedrock](/docs/fr/amazon-bedrock) ou `CLAUDE_CODE_USE_VERTEX` pour [Agent Platform de Google Cloud](/docs/fr/google-vertex-ai)
* [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) pointant vers un hôte autre que `api.anthropic.com`, comme une [passerelle LLM](/docs/fr/llm-gateway) ou un proxy, même quand vous vous connectez avec claude.ai ; avant v2.1.196, une URL de base personnalisée ne bloquait pas Remote Control
* `ANTHROPIC_UNIX_SOCKET` défini, donc la session envoie ses requêtes via un socket local plutôt qu'à `api.anthropic.com`
* Une connexion [passerelle cloud](/docs/fr/claude-apps-gateway) d'entreprise effectuée via `/login`, qui ne supporte pas Remote Control et n'a pas de variable à déconfigurez

**À faire :**

* Déconfigurez la variable que le message nomme, comme `CLAUDE_CODE_USE_BEDROCK` ou `ANTHROPIC_BASE_URL`, et redémarrez la session, ou démarrez Remote Control à partir d'une session qui parle directement à l'API Anthropic
* Si la variable n'est pas définie dans votre shell, vérifiez la clé `env` dans vos [fichiers de paramètres](/docs/fr/settings#where-settings-live), qui applique les variables d'environnement à chaque session
* Pour ce message et les autres messages de démarrage de Remote Control, consultez [Dépannage de Remote Control](/docs/fr/remote-control#troubleshooting)

<h3 id="remote-control-couldnt-refresh-your-login">
  Remote Control n'a pas pu rafraîchir votre connexion
</h3>

Claude Code exécute une connexion [Remote Control](/docs/fr/remote-control) en direct sur des credentials de courte durée qu'il obtient et renouvelle en utilisant votre connexion claude.ai enregistrée. Quand claude.ai arrête d'accepter cette connexion, ou que Claude Code n'a plus de connexion enregistrée, Claude Code arrête Remote Control et vous demande de vous reconnecter. L'une ou l'autre défaillance peut se produire pendant que Claude Code se connecte toujours ou plus tard, quand il renouvelle les credentials.

Quand Claude Code demande au service de connexion de rafraîchir votre connexion enregistrée et n'obtient pas de réponse, il garde Remote Control en cours d'exécution et réessaie le rafraîchissement pendant que la credential actuelle de la connexion est toujours valide. Un rafraîchissement n'obtient pas de réponse quand Claude Code ne peut pas atteindre le service de connexion, la requête expire, ou le service échoue sans rejeter votre connexion. Si le service de connexion ne répond toujours pas quand cette credential expire, Claude Code arrête Remote Control et signale `OAuth token refresh failed`.

Quand Claude Code arrête Remote Control, il affiche la raison dans un avertissement et dans une ligne de transcription qui commence par `Remote Control disconnected`. Votre session locale continue de s'exécuter sans Remote Control. Cette section couvre ces lignes :

```text theme={null}
Remote Control disconnected — Claude.ai login expired — run /login to restore Remote Control
Remote Control disconnected — Claude.ai login expired — run /login, then /remote-control
Remote Control disconnected — Claude.ai login was rejected — run /login, then /remote-control
Remote Control disconnected — OAuth token unavailable — run /login to restore Remote Control
Remote Control disconnected — OAuth token refresh failed — run /login to re-authenticate
Remote Control disconnected — JWT refresh failed: no OAuth token — run /login
Remote Control disconnected — Signed out of Claude — run /login, then /remote-control
```

Claude Code nomme la cause au milieu du message :

* ` Claude.ai login expired` et `Claude.ai login was rejected` : claude.ai n'accepte plus votre jeton de connexion enregistré, car il a expiré ou a été révoqué
* ` OAuth token unavailable` : Claude Code n'avait pas de jeton de connexion enregistré quand la credential de la connexion était due pour le renouvellement
* `OAuth token refresh failed` : claude.ai a rejeté votre jeton de connexion enregistré pendant que Claude Code se reconnectait, et le rafraîchissement du jeton n'en a produit aucun nouveau
* `JWT refresh failed: no OAuth token` : Claude Code n'a trouvé aucun jeton de connexion enregistré pour renouveler avec
* ` Signed out of Claude` : vous vous êtes déconnecté sur cette machine, par exemple en exécutant `/logout` dans un autre terminal, donc Claude Code n'a pas de connexion enregistrée pour renouveler la connexion avec

**À faire :**

* Exécutez `/login` pour vous reconnecter
* Exécutez `/remote-control` pour reconnecter la session. Les messages se terminant par `run /login to restore Remote Control` n'ont pas besoin de cette étape : Claude Code se reconnecte automatiquement une fois que vous vous êtes connecté.

Avant v2.1.224, `OAuth token refresh failed — run /login to re-authenticate` lisait `OAuth token refresh failed — re-authenticate, then re-enable Remote Control`, et `JWT refresh failed: no OAuth token — run /login` lisait `no OAuth token available for recovery (code <N>)`. Les messages `Claude.ai login expired`, `Claude.ai login was rejected` et `OAuth token unavailable` ont été ajoutés dans v2.1.225.

Avant v2.1.238, Claude Code signalait les cas qui disent maintenant `Signed out of Claude` comme `JWT refresh failed: no OAuth token — run /login`, et arrêtait Remote Control avec `Claude.ai login expired — run /login to restore Remote Control` dès qu'un rafraîchissement de connexion n'obtenait pas de réponse.

<h3 id="remote-control-stopped-because-the-signed-in-account-changed">
  Remote Control s'est arrêté car le compte connecté a changé
</h3>

Claude Code affiche cette ligne pendant une session [Remote Control](/docs/fr/remote-control) quand vous vous connectez à un compte ou une organisation claude.ai différent sur cette machine. Vous avez effectué le changement en dehors de la session Claude Code, par exemple en exécutant `/login` dans un autre terminal.

Une session Remote Control que vous avez démarrée alors que vous étiez connecté via `/login` appartient au compte et à l'organisation claude.ai qui étaient connectés à ce moment-là.

```text theme={null}
Remote Control disconnected — signed-in claude.ai account or organization changed on this machine — run /remote-control to start a session for the current account, or /login to switch back, then /remote-control
```

Claude Code arrête la session Remote Control dès que claude.ai confirme que le compte ou l'organisation a changé. Votre session locale continue de s'exécuter sans Remote Control.

**À faire :**

* Exécutez `/remote-control` pour démarrer une nouvelle session Remote Control sous le compte ou l'organisation actuel
* Pour revenir en arrière, exécutez `/login` et reconnectez-vous au compte ou à l'organisation précédent. Puis exécutez `/remote-control`.

Avant v2.1.234, Claude Code ne remarquait pas quand vous basculiez vers un compte ou une organisation différent en dehors de la session Claude Code. Claude Code gardait la session Remote Control connectée jusqu'à ce qu'une requête ultérieure au serveur Remote Control échoue avec `Remote Control server rejected the request (HTTP 404)`. Cet échec pouvait survenir des heures après le changement.

<h3 id="remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts">
  Remote Control s'est arrêté car l'application exécutant la session s'est déconnectée ou a changé de compte
</h3>

Quand l'application de bureau Claude ou un IDE héberge votre session, Claude Code obtient son jeton de connexion de cette application plutôt que de `/login`. Quand claude.ai rejette ce jeton, Claude Code demande à l'application un nouveau. Si l'application répond qu'elle est déconnectée, ou qu'elle est maintenant connectée à un compte Claude différent, Claude Code termine la session [Remote Control](/docs/fr/remote-control) et envoie à l'application l'une de ces lignes :

```text theme={null}
Remote Control stopped — the app running this session is now signed in to a different Claude account
Remote Control stopped — the app running this session is signed out of Claude. Sign in there, then turn Remote Control back on
```

Votre session locale continue de s'exécuter sans Remote Control.

**À faire :**

* Si l'application est déconnectée, reconnectez-vous à celle-ci, puis réactivez Remote Control dans l'application
* Si l'application a changé de compte, Claude Code ne peut pas continuer la session terminée sous le nouveau compte. Démarrez une nouvelle session Remote Control sous ce compte.

Avant v2.1.238, Claude Code envoyait à l'application les messages `/login` listés sous [Remote Control n'a pas pu rafraîchir votre connexion](#remote-control-couldnt-refresh-your-login) dans les deux cas.

<h3 id="oauth-token-revoked-or-expired">
  Jeton OAuth révoqué ou expiré
</h3>

Votre connexion enregistrée n'est plus valide. Un jeton révoqué signifie que vous vous êtes déconnecté partout ou qu'un administrateur a supprimé l'accès ; un jeton expiré signifie que le rafraîchissement automatique a échoué en cours de session.

Les deux messages signalent un rejet que l'API a retourné pour une requête que Claude Code a envoyée. Quand la connexion enregistrée a déjà été effacée après un rafraîchissement échoué, vous voyez [Connexion expirée](#login-expired) à la place. Si vous vous authentifiez avec un jeton de longue durée dans [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/fr/env-vars), vous voyez les mêmes messages quand ce jeton expire ou est révoqué.

```text theme={null}
OAuth token revoked · Please run /login
Please run /login · API Error: 401 OAuth token has expired ...
```

**À faire :**

* Exécutez `/login` pour vous reconnecter
* Si l'erreur revient dans la même session après réauthentification, exécutez d'abord `/logout` pour effacer complètement le jeton enregistré, puis `/login`
* Si vous vous authentifiez avec la variable d'environnement `CLAUDE_CODE_OAUTH_TOKEN`, Claude Code continue d'envoyer la valeur que vous avez définie après l'échec d'une requête avec un 401, plutôt que de basculer vers le jeton d'une connexion enregistrée. [`/status`](/docs/fr/commands) affiche cette credential comme une ligne `Auth token` lisant `CLAUDE_CODE_OAUTH_TOKEN`. Générez un jeton frais avec [`claude setup-token`](/docs/fr/authentication#generate-a-long-lived-token) et redémarrez avec, ou déconfigurez la variable et exécutez `/login`. Avant v2.1.225, Claude Code pouvait remplacer la valeur de la variable en cours de session par le jeton d'accès de courte durée d'une connexion enregistrée, et la session échouait à nouveau avec des erreurs 401 une fois ce jeton expiré.
* Pour les invites répétées de connexion entre les lancements, consultez les vérifications de l'horloge système et les étapes de récupération du stockage des credentials macOS dans [Dépannage](/docs/fr/troubleshoot-install#not-logged-in-or-token-expired)
* Pour les autres défaillances incluant `403 Forbidden` et les problèmes du navigateur OAuth, consultez [Connexion et authentification](/docs/fr/troubleshoot-install#login-and-authentication)

<h3 id="api-error-401-invalid-authentication-credentials">
  Erreur API : 401 Credentials d'authentification invalides
</h3>

L'API a reconnu le format de votre credential mais a rejeté le compte ou l'organisation derrière. Anthropic retourne ce message quand une credential a été récemment révoquée, quand une organisation a été désactivée ou a supprimé votre accès, ou quand le compte lui-même a été désactivé, donc un jeton expiré n'est pas la cause. La credential peut être votre connexion enregistrée ou une `ANTHROPIC_API_KEY` approuvée, et la correction diffère, donc commencez par exécuter `/status` pour voir laquelle est active.

```text theme={null}
Please run /login · API Error: 401 Invalid authentication credentials
```

**À faire :**

* Si `/status` affiche une ligne `API key` qui n'est pas marquée comme non utilisée, une [`ANTHROPIC_API_KEY`](/docs/fr/authentication#authentication-precedence) approuvée est la credential active et prend la priorité sur votre connexion, donc `/login` ne la remplace pas. Faites tourner la clé dans la Console Claude, ou revenez à votre abonnement en exécutant `unset ANTHROPIC_API_KEY`, ou dans PowerShell `Remove-Item Env:ANTHROPIC_API_KEY`.
* Si `/status` affiche uniquement votre connexion, exécutez `/login` une fois. Si la credential a été révoquée, une connexion fraîche la remplace.
* Si le même message revient pour le même compte de connexion, le compte ou l'organisation n'est plus actif. Vérifiez le compte et l'organisation que `/status` signale, et demandez à l'administrateur de votre organisation de restaurer l'accès.
* Si [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) pointe vers une [passerelle LLM](/docs/fr/llm-gateway), le texte après `401` est le message de votre passerelle plutôt que celui d'Anthropic, et `/login` ne le change pas. Corrigez plutôt la credential que votre passerelle attend.

<h3 id="login-expired">
  Connexion expirée
</h3>

Claude Code a essayé de renouveler votre connexion claude.ai ou Claude Console enregistrée et le service OAuth a rejeté le jeton d'actualisation enregistré, donc Claude Code a effacé les credentials enregistrées. Après cela, chaque requête de modèle s'arrête localement avec ce message avant d'atteindre l'API, car seul `/login` peut créer de nouvelles credentials.

Avant v2.1.206, Claude Code envoyait quand même la requête de modèle avec quelle que soit la credential restante dans l'environnement, et chaque modèle échouait alors avec [Il y a un problème avec le modèle sélectionné](#theres-an-issue-with-the-selected-model) ou un 401 au lieu d'une invite de connexion.

```text theme={null}
Login expired · Please run /login
```

En [mode non-interactif](/docs/fr/headless) (`-p`) et l'[Agent SDK](/docs/fr/agent-sdk/overview), le message se lit comme suit, et le code d'erreur structuré est `authentication_failed` :

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Ce n'est pas le même état que [Jeton OAuth révoqué ou expiré](#oauth-token-revoked-or-expired). Ces messages signalent un rejet que l'API a retourné. Claude Code lui-même produit `Login expired` pour une connexion qu'il a déjà échoué à renouveler, donc il n'envoie pas de requête. Quand le renouvellement échoue parce que le compte lui-même est suspendu plutôt que la connexion étant obsolète, Claude Code affiche [Votre compte est en attente](#your-account-is-on-hold) à la place.

Les sessions authentifiées avec une clé API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/fr/env-vars) ou un fournisseur tiers n'utilisent pas la connexion enregistrée et ne voient jamais ce message.

Vous pouvez vérifier cet état avant l'échec d'une requête : [`/status`](/docs/fr/commands) affiche une ligne `Login` lisant `Expired — log in again`, plus l'organisation et l'e-mail qu'il a enregistrés pour la connexion expirée. La ligne n'apparaît que quand la connexion enregistrée est votre credential active et ne peut plus être rafraîchie. Les sessions authentifiées d'une autre manière n'affichent pas la ligne, même si une connexion expirée reste enregistrée. Avant v2.1.210, `/status` ne donnait aucune indication dans cet état qu'une connexion avait jamais existé, car la credential effacée n'avait rien à signaler.

**À faire :**

* Exécutez `/login` pour vous reconnecter. Réessayer sans vous connecter affiche le même message à chaque requête.
* En mode non-interactif, exécutez `claude` dans le même environnement, complétez `/login`, puis réexécutez votre commande. Pour l'automatisation qui ne peut pas se connecter de manière interactive, authentifiez-vous avec `ANTHROPIC_API_KEY` ou [générez un jeton de longue durée avec `claude setup-token`](/docs/fr/authentication#generate-a-long-lived-token).
* Si la connexion continue d'échouer, consultez [Connexion et authentification](/docs/fr/troubleshoot-install#login-and-authentication)

<h3 id="claude-login-not-accepted">
  Connexion Claude non acceptée
</h3>

Vous avez essayé de démarrer une [session cloud](/docs/fr/claude-code-on-the-web), et le serveur a refusé de la créer avec un 401 : il n'a pas accepté la connexion Claude que cette machine a envoyée, généralement parce que la connexion a expiré ou a été révoquée.

La première partie de la ligne est la propre raison du serveur quand il en donne une. Sinon, la ligne se lit :

```text theme={null}
Claude login not accepted · Run /login, then try again
```

**À faire :**

* Exécutez `/login`, complétez la connexion, puis démarrez la session à nouveau

<h3 id="artifacts-need-a-claude-ai-login">
  Les artefacts ont besoin d'une connexion claude.ai
</h3>

Claude Code a refusé une publication ou une lecture d'[artefact](/docs/fr/artifacts) car la session n'a pas de connexion claude.ai qu'elle peut utiliser pour les artefacts.

Chaque forme du message commence par les mêmes mots, suivis d'un remède qui dépend de la façon dont votre session s'authentifie. Sans credential concurrente, il se lit :

```text theme={null}
Artifacts need a claude.ai login. Run /login and select "Claude account with subscription", then retry — the "Anthropic Console account" option does not provide claude.ai credentials.
```

**À faire :**

* Exécutez `/login` et sélectionnez **Claude account with subscription**. L'option **Anthropic Console account** ne fournit pas de credentials claude.ai.
* Quand le message nomme une credential qui prend la priorité, comme `ANTHROPIC_API_KEY`, un paramètre `apiKeyHelper` ou une clé Console enregistrée par une `/login` précédente, supprimez-la de la façon que le message dit, puis exécutez `/login`
* Quand le message dit que cette session distante s'authentifie via la machine qui l'a lancée, connectez-vous à claude.ai sur cette machine, puis reconnectez la session
* Quand le message dit que la credential est injectée par l'environnement hôte de la session, vous ne pouvez pas la modifier dans cette session ; démarrez une session qui est connectée à claude.ai
* Consultez [Disponibilité](/docs/fr/artifacts#availability) pour les autres exigences que les artefacts ont, comme le plan, le fournisseur de modèle et la politique d'organisation

<h3 id="administrator-policy-requires-a-cloud-gateway-sign-in">
  La politique de l'administrateur nécessite une connexion à la passerelle Cloud
</h3>

Un [paramètre géré](/docs/fr/managed-settings) d'un administrateur sur cette machine a défini [`forceLoginMethod`](/docs/fr/settings-reference#forceloginmethod) à `"gateway"` ou a défini [`forceLoginGatewayUrl`](/docs/fr/settings-reference#forcelogingatewayurl). À moins que vous sélectionniez un fournisseur cloud via une variable comme `CLAUDE_CODE_USE_BEDROCK`, Claude Code n'accepte alors que la connexion [passerelle d'applications Claude](/docs/fr/claude-apps-gateway). Vous voyez l'un de deux messages :

```text theme={null}
Not signed in to the Cloud gateway — run /login.
```

Les requêtes de modèle échouent avec ce message quand la session n'a pas de connexion à la passerelle, par exemple parce que vous n'avez pas exécuté `/login` depuis que la politique a atteint la machine.

Si vous avez également une credential `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` configurée et que les paramètres gérés définissent `forceLoginMethod`, Claude Code se termine au démarrage à la place avec un message qui commence par :

```text theme={null}
Administrator policy requires a Cloud gateway sign-in on this machine; the
Anthropic-issued credential configured here (ANTHROPIC_API_KEY,
ANTHROPIC_AUTH_TOKEN, or apiKeyHelper) is not used.
```

**À faire :**

* Exécutez `/login` et complétez la connexion sur l'écran **Cloud gateway**
* Pour le message de démarrage, supprimez le paramètre `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` ou `apiKeyHelper` que vous avez configuré, puis démarrez `claude` et exécutez `/login`
* Si vous pensez que la machine ne devrait pas nécessiter la passerelle, demandez à l'administrateur qui la gère de supprimer `forceLoginMethod` et `forceLoginGatewayUrl` de ses paramètres gérés

Sur v2.1.265, une régression a également affiché le premier message dans certaines configurations de passerelle LLM et proxy qui s'authentifient avec une clé API, `apiKeyHelper` ou des en-têtes personnalisés, même sans exigence d'administrateur sur la machine. Mettez à jour vers v2.1.266 ou ultérieur. Vous n'avez pas besoin de modifier votre configuration.

Avant v2.1.261, sur les machines qui définissent `forceLoginMethod` à `"gateway"`, Claude Code utilisait une connexion enregistrée restante au lieu d'échouer les requêtes de modèle, et signalait une credential d'environnement configurée avec `This machine's managed settings require a first-party login` au lieu du message de démarrage. Avant v2.1.265, une machine dont les paramètres gérés définissaient uniquement `forceLoginGatewayUrl` ne nécessitait pas la connexion à la passerelle, et Claude Code utilisait une credential restante là.

<h3 id="your-account-is-on-hold">
  Votre compte est en attente
</h3>

Le compte Claude derrière votre connexion a été suspendu. Claude Code affiche le premier message quand il essaie de renouveler votre connexion enregistrée et apprend de la suspension, et le deuxième quand une connexion que vous complétez dans le navigateur la signale :

```text theme={null}
Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted
Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted
```

Se reconnecter avec le même compte n'efface pas le message, car la suspension est sur le compte plutôt que sur la connexion. En [mode non-interactif](/docs/fr/headless) (`-p`) et l'[Agent SDK](/docs/fr/agent-sdk/overview), le code d'erreur structuré est `account_on_hold`. Avant v2.1.235, Claude Code signalait un compte suspendu comme [Connexion expirée · Veuillez exécuter /login](#login-expired), dont les étapes de récupération ne peuvent pas effacer une suspension.

**À faire :**

* Ouvrez le lien dans le message pour afficher les détails de la suspension ou l'appeler
* Si vous avez un autre compte Claude ou une clé API qui n'est pas affectée par la suspension, vous pouvez continuer à travailler pendant que la suspension est résolue : exécutez `/login` avec ce compte, ou définissez la clé avec `ANTHROPIC_API_KEY`

<h3 id="anthropic-profile-login-expired">
  Connexion au profil Anthropic expirée
</h3>

Claude Code s'authentifie via un profil de credential Anthropic dont la credential de connexion enregistrée a expiré, et le profil ne contient pas de credential d'actualisation que Claude Code peut utiliser pour la renouveler. Claude Code arrête chaque requête localement sans réessayer, car un réessai lirait la même credential expirée.

```text theme={null}
Anthropic profile login expired · Re-authenticate your Anthropic profile
Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile
```

Cela n'apparaît que quand la credential active provient d'un profil de credential Anthropic, un que vous sélectionnez avec la variable d'environnement `ANTHROPIC_PROFILE`, que Claude Code découvre comme le profil actif dans votre répertoire de configuration Anthropic, ou que Claude Code a écrit quand vous [vous êtes connecté sans clé API](/docs/fr/authentication#sign-in-without-an-api-key). Les sessions qui s'authentifient avec l'option claude.ai de `/login`, une clé API, un jeton bearer comme `ANTHROPIC_AUTH_TOKEN` ou un fournisseur tiers ne voient jamais ce message.

Sur une machine qui [offre la connexion sans clé](/docs/fr/authentication#sign-in-without-an-api-key), exécutez `/login`, choisissez le compte Anthropic Console et reconnectez-vous pour renouveler un profil que la connexion Console sans clé ou la CLI Claude Platform `ant auth login` a écrit. Claude Code remplace la credential expirée dans ce profil. Pour un profil de fédération ou un créé par un autre outil, `/login` ne renouvelle pas la credential. La forme que vous voyez dépend de si vous avez sélectionné le profil ou si Claude Code l'a découvert :

* Quand vous définissez `ANTHROPIC_PROFILE` explicitement, le message se termine par `Re-authenticate your Anthropic profile`.
* Quand Claude Code a découvert le profil à partir de votre répertoire de configuration, le message offre `/login`, car Claude Code donne la priorité à une `/login` fonctionnelle sur le profil découvert et s'authentifie ensuite avec votre compte claude.ai ou Console à la place. Avant v2.1.234, Claude Code affichait la forme `Re-authenticate your Anthropic profile` dans ce cas aussi.

**À faire :**

* Reconnectez-vous au profil, puis réessayez : sur une machine qui [offre la connexion sans clé](/docs/fr/authentication#sign-in-without-an-api-key), exécutez `/login` et choisissez le compte Anthropic Console pour un profil que la connexion Console sans clé ou la CLI Claude Platform `ant auth login` a écrit ; pour les autres profils, utilisez l'outil qui les a créés
* Si un administrateur a provisionné la credential du profil, demandez-lui d'en émettre une nouvelle
* Exécutez `/status` pour confirmer la source de credential active et le nom du profil
* Pour arrêter d'utiliser le profil, déconfigurez `ANTHROPIC_PROFILE` si vous l'avez défini, puis authentifiez-vous d'une autre manière, comme `/login` ou `ANTHROPIC_API_KEY`

<h3 id="oauth-scope-requirement">
  Exigence de portée OAuth
</h3>

Le jeton enregistré est antérieur à une portée de permission qu'une fonctionnalité plus récente nécessite. Vous voyez cela le plus souvent de `/usage` et l'indicateur d'utilisation de la ligne d'état :

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**À faire :**

* Exécutez `/login` pour obtenir un nouveau jeton avec les portées actuelles. Vous n'avez pas besoin de vous déconnecter d'abord.

<h3 id="claude-ai-rejected-the-session-token">
  claude.ai a rejeté le jeton de session
</h3>

Une requête de [connecteur claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) a échoué car claude.ai a rejeté le jeton de votre connexion Claude Code, généralement une connexion qui a expiré et n'a pas pu être rafraîchie. Le jeton rejeté est votre connexion, pas l'autorisation propre du connecteur dans claude.ai, donc autoriser le connecteur à nouveau ne le résout pas. Dans `/mcp`, le connecteur s'affiche comme `connected · session token rejected` et sa vue de détail se lit :

```text theme={null}
claude.ai rejected the session token. Run /login, then reconnect.
```

**À faire :**

* Exécutez `/login` pour vous reconnecter
* Reconnectez le connecteur à partir de `/mcp`, ou exécutez `/mcp reconnect <server>`. La reconnexion avant de vous reconnecter laisse le connecteur dans le même état. L'option **Reconnect** du panneau `/mcp` signale `your claude.ai session token was rejected` ; la forme `/mcp reconnect <server>` tapée signale une reconnexion réussie même si le jeton est toujours rejeté.

Avant v2.1.222, Claude Code marquait le connecteur comme ayant besoin d'authentification à la place, ce qui vous pointait vers le flux d'autorisation du connecteur même si le compléter ne résolvait pas l'état.

<h3 id="mcp-server-needs-you-to-sign-in-again">
  Le serveur MCP a besoin que vous vous reconnectiez
</h3>

Un [serveur MCP](/docs/fr/mcp) distant a rejeté la credential sur un appel d'outil en cours de session, généralement parce qu'une connexion ou un jeton a expiré ou parce que le jeton manque d'une permission que l'outil nécessite. L'appel d'outil échoue, et `/mcp` marque le serveur comme [ayant besoin d'authentification](/docs/fr/mcp#authenticate-with-remote-mcp-servers).

Pour un serveur auquel vous vous connectez à partir de Claude Code, y compris un connecteur claude.ai, la connexion a expiré ou a été révoquée :

```text theme={null}
MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)
```

Exécutez `/mcp`, sélectionnez le serveur et reconnectez-vous à partir de son menu.

Pour un serveur configuré avec un script [`headersHelper`](/docs/fr/mcp#use-dynamic-headers-for-custom-authentication), Claude Code a déjà réexécuté le helper et réessayé l'appel une fois avant d'afficher ceci :

```text theme={null}
MCP server "<name>" rejected the credential from its headersHelper (check the helper and run /mcp to reconnect, or to authenticate if the server also uses OAuth)
```

Vérifiez que le helper retourne une credential que le serveur accepte, puis reconnectez à partir de `/mcp`, qui réexécute le helper.

Pour un serveur avec un en-tête `Authorization` statique dans sa configuration :

```text theme={null}
MCP server "<name>" rejected the Authorization header in its config (update it, then run /mcp to reconnect)
```

Mettez à jour la valeur d'en-tête où le serveur est configuré, puis reconnectez à partir de `/mcp`.

Avant v2.1.273, les cas de connexion expirée, `headersHelper` et d'en-tête `Authorization` affichaient tous `MCP server "<name>" requires re-authorization (token expired)`.

Un serveur peut également refuser un appel d'outil avec HTTP 403 `insufficient_scope` pour vous demander d'autoriser une portée, parfois une que votre jeton liste déjà. Le message nomme cette portée :

```text theme={null}
MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate
```

Exécutez `/mcp`, sélectionnez le serveur et authentifiez-vous à nouveau à partir de son menu.

Quand la configuration du serveur ne définit ni [`oauth.scopes`](/docs/fr/mcp#restrict-oauth-scopes) ni [`authServerMetadataUrl`](/docs/fr/mcp#override-oauth-metadata-discovery), Claude Code demande la portée que le serveur a nommée. Avec l'un ou l'autre paramètre, Claude Code demande plutôt les portées de ce paramètre. Si vous avez épinglé `oauth.scopes`, ajoutez la portée manquante à cette liste avant de vous authentifier à nouveau.

Avant v2.1.274, ce cas affichait le message `needs you to sign in again`, et avant v2.1.273 il affichait `requires re-authorization (token expired)` comme les autres cas.

<h3 id="issuer-mismatch-in-authorization-response">
  Incompatibilité d'émetteur dans la réponse d'autorisation
</h3>

Pendant une [connexion OAuth MCP](/docs/fr/mcp#authenticate-with-remote-mcp-servers), le serveur d'autorisation a redirigé vers Claude Code avec un paramètre `iss` qui ne nomme pas l'émetteur que Claude Code attendait des métadonnées OAuth du serveur. Un mauvais émetteur à cette étape est à quoi ressemble une attaque de mélange de serveur d'autorisation, donc Claude Code échoue la connexion au lieu d'échanger le code d'autorisation. Claude Code affiche l'erreur dans le menu du serveur `/mcp` après la connexion du navigateur :

```text theme={null}
Issuer mismatch in authorization response (RFC 9207): expected "https://auth.example.com", received "https://other.example.com"
```

`expected` est l'émetteur des métadonnées OAuth du serveur, et `received` est la valeur `iss` que la redirection portait. Une connexion dont la redirection ne porte pas de paramètre `iss` réussit la vérification, à moins que les métadonnées du serveur définissent `authorization_response_iss_parameter_supported`, auquel cas Claude Code échoue la connexion.

**À faire :**

* Réessayez la connexion à partir de `/mcp`
* Si l'erreur se répète, signalez-la à l'opérateur du serveur. La correction est côté serveur : le serveur d'autorisation doit retourner le même émetteur dans le paramètre `iss` qu'il annonce dans ses métadonnées
* Pour vous connecter pendant que le serveur est en cours de correction, démarrez Claude Code avec [`MCP_SDK_GENERATION=v1`](/docs/fr/env-vars), dont le [runtime](/docs/fr/mcp#mcp-client-runtimes) n'exécute pas cette vérification. Cela supprime une protection contre les attaques de mélange, donc préférez la correction côté serveur

Avant v2.1.232, Claude Code utilisait le runtime v2 uniquement dans un déploiement progressif ou quand vous définissiez `MCP_SDK_GENERATION=v2`.

<h3 id="aws-credentials-expired-or-invalid">
  Les credentials AWS ont expiré ou sont invalides
</h3>

Votre jeton de session AWS a expiré ou a été rejeté. Ce message apparaît sur un 401 de [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws) ou du [point de terminaison Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint), c'est ainsi que ces fournisseurs signalent un jeton de sécurité expiré.

L'indice d'action au milieu varie selon votre configuration. La partie stable est le début `AWS credentials expired or invalid` :

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Avant v2.1.273, ce message n'apparaissait que quand `awsAuthRefresh` était configuré.

**À faire :**

* Si l'indice dit que les credentials sont gérées par cet environnement, l'application qui a lancé Claude Code possède la credential et les autres étapes ici ne s'appliquent pas : réessayez, ou contactez votre administrateur
* Si [`awsAuthRefresh`](/docs/fr/amazon-bedrock#advanced-credential-configuration) est défini, exécutez la commande nommée dans le message, comme `aws sso login --profile myprofile`, dans un autre terminal et complétez la connexion du navigateur, puis réessayez. Sinon, rafraîchissez la credential AWS que vous utilisez vous-même : votre connexion SSO, les clés d'accès, la clé API ou le jeton proxy
* Avec `awsAuthRefresh` défini dans une session interactive, vous pouvez à la place exécuter `/login`, choisir **3rd-party platform**, puis sélectionner **Claude Platform on AWS · refresh credentials** sous **Using 3rd-party platforms** pour exécuter la même commande sans redémarrer Claude Code. Consultez [Configurer les credentials AWS](/docs/fr/claude-platform-on-aws#1-configure-aws-credentials)
* Si l'erreur se répète après la réussite de la commande de rafraîchissement, confirmez que l'identité est valide en dehors de Claude Code avec `aws sts get-caller-identity` dans le même shell et profil

<h3 id="aws-authentication-failed">
  L'authentification AWS a échoué
</h3>

Votre fournisseur AWS a retourné un 403, ou [Amazon Bedrock](/docs/fr/amazon-bedrock) a retourné un 401.

Amazon Bedrock signale un jeton de sécurité expiré comme un 403, mais un 403 est aussi comment il signale un refus d'autorisation, comme un `AccessDeniedException` d'une permission IAM manquante. Claude Code ne peut pas distinguer ces deux causes.

Un 401 d'Amazon Bedrock atterrit aussi ici plutôt que sous [Les credentials AWS ont expiré ou sont invalides](#aws-credentials-expired-or-invalid), car Amazon Bedrock ne signale pas un jeton expiré comme un 401. Un 401 de ce point de terminaison provient généralement de quelque chose d'autre dans le chemin de la requête, comme un proxy d'entreprise.

Un rafraîchissement de credential corrige un jeton expiré et ne peut pas corriger les autres causes, donc le message offre les deux :

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

L'indice d'action au milieu varie selon votre configuration. La partie stable est le début `AWS authentication failed`.

Quand le 403 est la réponse d'Amazon Bedrock que vous n'avez pas accès au modèle avec l'ID de modèle spécifié, l'indice vous dit plutôt d'activer le modèle pour votre compte et région dans la console Amazon Bedrock.

Avant v2.1.273, ce message n'apparaissait que quand `awsAuthRefresh` était configuré.

**À faire :**

* Si l'indice dit que les credentials sont gérées par cet environnement, l'application qui a lancé Claude Code possède la credential et les autres étapes ici ne s'appliquent pas : réessayez, ou contactez votre administrateur
* Rafraîchissez vos credentials AWS au cas où une credential expirée serait la cause : exécutez la commande [`awsAuthRefresh`](/docs/fr/amazon-bedrock#advanced-credential-configuration) nommée dans le message quand une est définie, ou rafraîchissez votre connexion SSO, les clés d'accès, la clé API ou le jeton proxy vous-même
* Si vos credentials sont actuelles, confirmez que les permissions IAM dans [Configuration IAM](/docs/fr/amazon-bedrock#iam-configuration) sont attachées à l'identité que vous utilisez et que le modèle sélectionné est activé pour votre compte et région
* Exécutez `aws sts get-caller-identity` pour confirmer quelle identité vos requêtes utilisent ; un `AWS_PROFILE` obsolète ou un profil par défaut est une cause courante d'une incompatibilité de permission

<h3 id="google-cloud-credentials-expired-or-invalid">
  Les credentials Google Cloud ont expiré ou sont invalides
</h3>

Vos credentials Google Cloud pour [Agent Platform de Google Cloud](/docs/fr/google-vertex-ai) ont expiré ou ont été rejetées : la requête a retourné un 401, c'est ainsi qu'Agent Platform signale l'expiration des credentials.

L'indice d'action au milieu varie selon votre configuration. La partie stable est le début `Google Cloud credentials expired or invalid` :

```text theme={null}
Google Cloud credentials expired or invalid · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · API Error: 401 ...
```

**À faire :**

* Si l'indice dit que les credentials sont gérées par cet environnement, l'application qui a lancé Claude Code possède la credential et les autres étapes ici ne s'appliquent pas : réessayez, ou contactez votre administrateur
* Si vous vous authentifiez avec les credentials par défaut de l'application, exécutez la commande [`gcpAuthRefresh`](/docs/fr/google-vertex-ai#advanced-credential-configuration) nommée dans le message, ou `gcloud auth application-default login`, et complétez la connexion, puis réessayez
* Si vous acheminez via une [passerelle LLM](/docs/fr/llm-gateway) avec `CLAUDE_CODE_SKIP_VERTEX_AUTH` défini, rafraîchissez le jeton de la passerelle dans `ANTHROPIC_AUTH_TOKEN` ou `ANTHROPIC_CUSTOM_HEADERS`, puis réessayez
* Si vous vous authentifiez avec un fichier de clé de compte de service, confirmez que `GOOGLE_APPLICATION_CREDENTIALS` pointe vers une clé valide. Consultez [Configurer les credentials GCP](/docs/fr/google-vertex-ai#3-configure-gcp-credentials)
* Si l'erreur se répète après un rafraîchissement, confirmez que l'identité fonctionne en dehors de Claude Code avec `gcloud auth application-default print-access-token` dans le même shell

Avant v2.1.273, un 401 d'Agent Platform affichait le message générique `Please run /login` ou `Failed to authenticate` à la place, qui ne peut pas rafraîchir les credentials Google Cloud.

<h3 id="google-cloud-authentication-failed">
  L'authentification Google Cloud a échoué
</h3>

[Agent Platform de Google Cloud](/docs/fr/google-vertex-ai) a retourné un 403, qu'il utilise pour les refus d'autorisation plutôt que les credentials expirées. Généralement, l'identité avec laquelle vous vous authentifiez manque d'une permission IAM, ou le modèle n'est pas activé pour votre projet.

L'indice d'action au milieu varie selon votre configuration. La partie stable est le début `Google Cloud authentication failed` :

```text theme={null}
Google Cloud authentication failed · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · if credentials are current, check GCP IAM permissions and Vertex AI model access · API Error: 403 ...
```

**À faire :**

* Si l'indice dit que les credentials sont gérées par cet environnement, l'application qui a lancé Claude Code possède la credential et les autres étapes ici ne s'appliquent pas : réessayez, ou contactez votre administrateur
* Confirmez que les rôles dans [Configuration IAM](/docs/fr/google-vertex-ai#iam-configuration) sont accordés à l'identité avec laquelle vous vous authentifiez
* Confirmez que le modèle est activé pour votre projet. Consultez [Demander l'accès au modèle](/docs/fr/google-vertex-ai#2-request-model-access)

Avant v2.1.273, un 403 d'Agent Platform affichait le message générique `Please run /login` ou `Failed to authenticate` à la place, qui ne peut pas rafraîchir les credentials Google Cloud.

<h3 id="microsoft-foundry-authentication-failed">
  L'authentification Microsoft Foundry a échoué
</h3>

[Microsoft Foundry](/docs/fr/microsoft-foundry) a retourné un 401 ou 403 : la credential Azure sur la requête a été rejetée, ou l'identité derrière n'a pas accès à la ressource Foundry. `/login` ne peut pas émettre de credentials Azure. L'indice d'action au milieu varie selon votre configuration. La partie stable est le début `Microsoft Foundry authentication failed` :

```text theme={null}
Microsoft Foundry authentication failed · refresh your Foundry credential (ANTHROPIC_FOUNDRY_AUTH_TOKEN, ANTHROPIC_FOUNDRY_API_KEY, Azure sign-in for Entra, or your proxy token) and retry · if credentials are current, check access to the Foundry resource · API Error: 401 ...
```

**À faire :**

* Si l'indice dit que les credentials sont gérées par cet environnement, l'application qui a lancé Claude Code possède la credential et les autres étapes ici ne s'appliquent pas : réessayez, ou contactez votre administrateur
* Rafraîchissez la credential que vous avez configurée dans [Configurer les credentials Azure](/docs/fr/microsoft-foundry#2-configure-azure-credentials) : faites tourner `ANTHROPIC_FOUNDRY_API_KEY`, émettez un `ANTHROPIC_FOUNDRY_AUTH_TOKEN` frais, ou exécutez `az login` pour que la chaîne de credential Microsoft Entra par défaut puisse se reconnecter
* Si la credential est actuelle, confirmez que l'identité a accès à la ressource Foundry. Consultez [Configuration Azure RBAC](/docs/fr/microsoft-foundry#azure-rbac-configuration)

Avant v2.1.273, un 401 ou 403 de Microsoft Foundry affichait le message générique `Please run /login` ou `Failed to authenticate` à la place, qui ne peut pas rafraîchir les credentials Azure.

<h3 id="could-not-load-aws-or-google-cloud-credentials">
  Impossible de charger les credentials AWS ou Google Cloud
</h3>

Claude Code n'a pas pu obtenir de credentials utilisables à partir de la chaîne de fournisseur de credentials AWS ou de vos credentials par défaut de l'application Google sur la machine sur laquelle il s'exécute, donc aucune requête n'a atteint votre fournisseur cloud. Claude Code efface ses credentials en cache et réessaie deux fois avant d'afficher ce message. Le détail après le `·` nomme la cause spécifique, comme une session SSO expirée, des credentials par défaut manquantes signalées comme `Could not load the default credentials`, ou une connexion révoquée signalée comme `invalid_grant` :

```text theme={null}
API Error: Could not load AWS credentials · Could not load credentials from any providers. Check or refresh your AWS credentials and try again.
API Error: Could not load Google Cloud credentials · invalid_grant. Check or refresh your Google Cloud credentials and try again.
```

En [mode non-interactif](/docs/fr/headless) avec `-p` et dans l'[Agent SDK](/docs/fr/agent-sdk/overview), le code d'erreur structuré est `cloud_credential_error`. Avant v2.1.267, le message affichait uniquement le texte de détail après `API Error:`, et le code structuré était `server_error` ou `unknown`.

**À faire :**

* Exécutez la commande de connexion de votre fournisseur, comme `aws sso login --profile myprofile` ou `gcloud auth application-default login`, puis réessayez. [Les credentials Bedrock, Agent Platform ou Foundry ne se chargent pas](/docs/fr/troubleshoot-install#bedrock-agent-platform-or-foundry-credentials-not-loading) montre comment confirmer les credentials en dehors de Claude Code
* Si le détail se lit `AWS default-chain credential resolve timed out`, la chaîne a bloqué plutôt que d'échouer, donc suivez [Le délai d'expiration de la résolution des credentials de la chaîne par défaut AWS](#aws-default-chain-credential-resolve-timed-out) à la place

<h3 id="aws-default-chain-credential-resolve-timed-out">
  Le délai d'expiration de la résolution des credentials de la chaîne par défaut AWS
</h3>

La chaîne de fournisseur de credentials par défaut AWS n'a pas produit de credentials dans les 60 secondes, donc Claude Code a arrêté la résolution et a échoué la requête. Ce délai d'expiration est une cause de [Impossible de charger les credentials AWS ou Google Cloud](#could-not-load-aws-or-google-cloud-credentials). L'échec est la résolution locale des credentials : la requête n'a jamais atteint [Amazon Bedrock](/docs/fr/amazon-bedrock), [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws) ou le [point de terminaison Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint). Claude Code efface son [cache de credentials](/docs/fr/amazon-bedrock#credential-caching-and-resolution-timeout) et réessaie avant que cette erreur ne fasse surface, donc au moment où vous la voyez, la chaîne s'est bloquée sur des tentatives répétées.

```text theme={null}
API Error: Could not load AWS credentials · AWS default-chain credential resolve timed out. Check or refresh your AWS credentials and try again.
```

Les causes courantes sont une commande `credential_process` dans votre profil AWS qui attend une entrée qu'elle ne peut pas recevoir, et un conteneur ou une VM dont le service de métadonnées d'instance (IMDS) ne répond jamais à la sonde de la chaîne.

Avant v2.1.267, le message se lisait `API Error: AWS default-chain credential resolve timed out`.
Avant v2.1.207, une chaîne bloquée laissait la requête attendre indéfiniment au lieu d'échouer.

**À faire :**

* Exécutez `aws sts get-caller-identity` dans le même shell avec le même `AWS_PROFILE`. S'il bloque aussi, corrigez le profil ; une commande `credential_process` qui demande de manière interactive est une cause courante.
* Complétez l'étape de connexion avant de démarrer Claude Code, par exemple `aws sso login --profile myprofile`, pour que la chaîne se résolve à partir du cache SSO local au lieu d'attendre un flux de navigateur
* Si votre chaîne exécute une connexion interactive qui a légitimement besoin de plus de 60 secondes, comme SSO avec MFA via un wrapper comme `aws-vault`, augmentez la limite en millisecondes avec [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/fr/env-vars)

<h3 id="bedrock-setup-verification-timed-out-waiting-for-aws">
  La vérification de la configuration de Bedrock a expiré en attendant AWS
</h3>

Un appel à AWS pendant l'[assistant de configuration de Bedrock](/docs/fr/amazon-bedrock#sign-in-with-bedrock), comme la recherche de credentials ou la vérification d'identité, n'a pas terminé dans la limite de 60 secondes. L'assistant arrête d'attendre et échoue l'étape de vérification :

```text theme={null}
Timed out after 60s waiting for AWS. Check your network and proxy settings; if a credential helper needs longer to prompt you, raise CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS.
```

Le nombre reflète votre limite : 60 secondes par défaut, ou la valeur que vous avez définie dans [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/fr/env-vars).

Les causes courantes sont un réseau ou un proxy qui bloque les requêtes à AWS, y compris le rafraîchissement du jeton SSO, et un helper de credential qui attend toujours une entrée que vous ne pouvez pas voir. Augmentez la limite uniquement quand l'helper a légitimement besoin de plus de temps.

Une seule requête bloquée à AWS peut aussi échouer sur son propre délai d'expiration par requête, qui affiche un message plus court sur la même étape :

```text theme={null}
A request to AWS timed out. Check your network and proxy settings, then try again.
```

Quand les mêmes délais d'expiration se produisent sur l'étape d'épinglage de modèle, l'assistant marque un modèle comme `unreachable` au lieu d'afficher l'un ou l'autre message.

**À faire :**

* Exécutez `aws sts get-caller-identity` dans le même shell. S'il bloque aussi, le blocage est en dehors de Claude Code, dans votre réseau, votre proxy ou l'helper de credential dans votre profil AWS ; corrigez cela d'abord.
* Complétez toute connexion interactive avant d'ouvrir l'assistant, par exemple `aws sso login --profile myprofile`
* Si un helper de credential dans votre profil AWS a légitimement besoin de plus de 60 secondes pour vous demander, augmentez la limite en millisecondes avec [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/fr/env-vars)

<h3 id="cloud-gateway-session-expired">
  La session de la passerelle cloud a expiré
</h3>

Vous vous êtes connecté via une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), et la session de la passerelle enregistrée sur cette machine a expiré et n'a pas pu être renouvelée, ou la passerelle ne l'accepte plus, par exemple après que le [secret JWT de la passerelle soit remplacé](/docs/fr/claude-apps-gateway-deploy#jwt-secret-rotation). Si vous voyez cette ligne quand vous démarrez `claude` de manière interactive, la session s'est ouverte déconnectée de la passerelle :

```text theme={null}
Cloud gateway session expired — run /login to reconnect.
```

La même ligne peut apparaître en cours de session quand la credential de la passerelle expire et Claude Code ne peut pas la renouveler.

Dans une exécution [non-interactive](/docs/fr/headless), une session en arrière-plan ou autre sans surveillance, ou une sous-commande `claude` autre que `claude auth`, Claude Code se termine avec ce message à la place quand la passerelle n'accepte plus la session :

```text theme={null}
Cloud gateway <url> no longer accepts this session. Start `claude` and sign in again with /login.
```

**À faire :**

* Exécutez `/login` dans la session et complétez la connexion du navigateur
* Pour un lancement non-interactif, démarrez `claude` dans le même environnement, exécutez `/login`, puis réexécutez votre commande

<h3 id="sign-in-timed-out-while-waiting-for-you-to-continue">
  La passerelle a refusé la requête
</h3>

Vous êtes connecté via une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), et une requête a retourné un 403 : la passerelle, ou l'amont derrière, l'a refusée. Se reconnecter ne change pas un refus, donc le message pointe vers votre administrateur de passerelle :

```text theme={null}
Gateway refused the request · signing in again won't change this — check with your gateway administrator · API Error: 403 ...
```

**À faire :**

* Demandez à votre administrateur de passerelle de rechercher la requête. La queue `API Error:` porte le refus que la passerelle a retourné
* Pour les administrateurs : une [règle de contrôle d'accès](/docs/fr/claude-apps-gateway-config#http-tuning) sur la passerelle retourne un 403 que le [journal d'audit](/docs/fr/claude-apps-gateway-deploy#logs) enregistre avec sa raison, et un refus d'autorisation en amont passe par [Messages d'erreur en amont](/docs/fr/claude-apps-gateway-config#upstream-error-messages)

Avant v2.1.273, un 403 sur une session de passerelle affichait le message générique `Please run /login` ou `Failed to authenticate` à la place, et se reconnecter ne changeait pas le refus.

<h3 id="gateway-refused-the-request">
  Connexion non acceptée à la passerelle Cloud
</h3>

Vous avez essayé de démarrer une [session cloud](/docs/fr/claude-code-on-the-web), et le serveur a refusé de la créer avec un 401 : il n'a pas accepté la connexion Claude que cette machine a envoyée, généralement parce que la connexion a expiré ou a été révoquée.

La première partie de la ligne est la propre raison du serveur quand il en donne une. Sinon, la ligne se lit :

```text theme={null}
Claude login not accepted · Run /login, then try again
```

**À faire :**

* Exécutez `/login`, complétez la connexion, puis démarrez la session à nouveau

<h2 id="network-and-connection-errors">
  Erreurs de réseau et de connexion
</h2>

La plupart de ces erreurs signifient qu'une requête réseau de Claude Code n'a pas pu atteindre sa destination, ou que quelque chose entre Claude Code et l'API a modifié la réponse en chemin ; lorsqu'une entrée a également une cause locale, comme une écriture d'archive échouée, son corps l'indique. Elles proviennent généralement de votre réseau local, proxy ou pare-feu, ou de la politique réseau de l'environnement cloud.

<h3 id="unable-to-connect-to-api">
  Impossible de se connecter à l'API
</h3>

La connexion TCP à l'API a échoué ou ne s'est jamais complétée. Pour les codes d'erreur de connexion courants, le message nomme le type d'échec et conserve le code entre parenthèses :

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

Un code que Claude Code ne reconnaît pas apparaît comme `Unable to connect to API` suivi du code entre parenthèses. Certains de ces messages peuvent afficher plus d'un code : `Connection refused` peut afficher `ConnectionRefused` ou `ECONNREFUSED`, par exemple, et `Can't reach the API server` peut afficher `ENOTFOUND` ou `FailedToOpenSocket`.

Avant la v2.1.227, chacun de ces messages codés lisait `Unable to connect to API` suivi du code, par exemple `Unable to connect to API (ECONNREFUSED)`.

Les causes courantes incluent l'absence d'accès à Internet, un VPN qui bloque `api.anthropic.com`, ou un proxy d'entreprise requis qui n'est pas configuré.

**À faire :**

* Confirmez que vous pouvez atteindre l'hôte API à partir du même shell en exécutant `curl -I https://api.anthropic.com`. Sur Windows PowerShell, utilisez `curl.exe -I https://api.anthropic.com` pour que l'alias `Invoke-WebRequest` intégré ne soit pas utilisé.
* Si vous êtes derrière un proxy d'entreprise, définissez `HTTPS_PROXY` avant de lancer Claude Code et consultez [Configuration réseau](/docs/fr/network-config)
* Si vous routez via une passerelle LLM ou un relais, définissez [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) sur son adresse. Consultez [Connecter Claude Code à une passerelle LLM](/docs/fr/llm-gateway-connect) pour la configuration.
* Assurez-vous que votre pare-feu autorise les hôtes listés dans [Exigences d'accès réseau](/docs/fr/network-config#network-access-requirements)
* Les défaillances intermittentes sont [automatiquement réessayées](#automatic-retries) ; les défaillances persistantes pointent vers un problème réseau local

Si `curl` réussit mais que Claude Code échoue toujours, la cause est généralement quelque chose entre le runtime et le réseau plutôt que le réseau lui-même :

* Sur Linux et WSL, vérifiez `/etc/resolv.conf` pour un serveur de noms inaccessible. WSL en particulier peut hériter d'un résolveur cassé de l'hôte.
* Sur macOS, un client VPN qui a été déconnecté ou désinstallé peut laisser une interface de tunnel ou une règle de routage. Vérifiez `ifconfig` pour les interfaces `utun` obsolètes et supprimez l'extension réseau du VPN dans les Paramètres système.
* Docker Desktop et les runtimes de conteneurs similaires peuvent intercepter le trafic sortant. Quittez-les et réessayez pour exclure cette possibilité.

<h3 id="unable-to-connect-to-anthropic-services">
  Impossible de se connecter aux services Anthropic
</h3>

Lors de la configuration initiale, Claude Code vérifie qu'il peut atteindre `api.anthropic.com` et `platform.claude.com` avant d'afficher l'étape de connexion. Lorsque l'une des vérifications échoue, Claude Code imprime la raison et se ferme.

```text theme={null}
Unable to connect to Anthropic services
Failed to connect to api.anthropic.com: ECONNREFUSED
Connection to api.anthropic.com timed out after 10 seconds
A proxy is configured via HTTPS_PROXY. Check that it allows connections to the host above.
```

Claude Code envoie la vérification via la même [configuration proxy](/docs/fr/network-config) que les requêtes API et donne à chaque sonde 10 secondes. Lorsque la sonde échouée a traversé un proxy, le message nomme la variable d'environnement qui l'a configurée, comme `HTTPS_PROXY`. Avant la v2.1.222, la vérification utilisait un transport proxy différent sans délai d'expiration : derrière une URL proxy avec le schéma `https://`, elle pouvait se bloquer sur `Checking connectivity...` indéfiniment puis échouer même si les requêtes API via le même proxy réussissent.

Claude Code ignore cette vérification lorsqu'un [fichier de paramètres gérés, une politique MDM ou un assistant de politique](/docs/fr/managed-settings) définit [`forceLoginMethod`](/docs/fr/settings-reference#forceloginmethod) sur `"gateway"`, ou définit [`forceLoginGatewayUrl`](/docs/fr/settings-reference#forcelogingatewayurl) sans `forceLoginMethod`. Avec l'une ou l'autre configuration, Claude Code ouvre l'étape de connexion sur l'écran **Cloud gateway** plutôt qu'une méthode de connexion Anthropic. Claude Code ignore également la vérification lorsqu'une source de paramètres gérés sur la machine existe mais ne peut pas être lue, car cette source peut contenir la configuration de la passerelle. Avant la v2.1.247, Claude Code exécutait la vérification sous cette configuration aussi, et se fermait avec cette erreur lorsque les points de terminaison d'Anthropic étaient inaccessibles.

**À faire :**

* Si le message nomme une variable proxy, vérifiez que sa valeur pointe vers le bon proxy et demandez à votre équipe réseau d'autoriser les connexions HTTPS via celui-ci vers l'hôte du message. Consultez [Configuration réseau](/docs/fr/network-config).
* Parcourez les vérifications dans [Impossible de se connecter à l'API](#unable-to-connect-to-api). Le test `curl` et les conseils de pare-feu là s'appliquent à cette vérification aussi.
* Si votre organisation se connecte via une [passerelle cloud](/docs/fr/claude-apps-gateway) et cette erreur apparaît au premier lancement, mettez à jour vers Claude Code v2.1.247 ou ultérieur.
* Si votre réseau est ouvert et l'échec persiste, Claude Code peut ne pas être [disponible dans votre pays](https://www.anthropic.com/supported-countries)

<h3 id="socket-is-closed">
  Socket is closed
</h3>

`Socket is closed` signifie que la connexion transportant une réponse en streaming a été fermée alors que la réponse arrivait toujours. La cause la plus courante est un proxy d'entreprise sur Windows qui abandonne un tunnel établi au milieu de la réponse.

Selon la progression de la réponse, Claude Code réessaye la requête, conserve ce que Claude a produit, ou termine le tour. Consultez [Réessais automatiques](#automatic-retries).

Avant la v2.1.214, Claude Code ne réessayait pas cet échec, et le tour s'arrêtait avec une erreur contenant `Socket is closed`.

**À faire :**

* Si vous voyez cette erreur, mettez à jour vers v2.1.214 ou ultérieur avec `claude update`, puis renvoyez votre message
* Si les tours continuent d'échouer derrière le même proxy après la mise à jour, parcourez [Impossible de se connecter à l'API](#unable-to-connect-to-api) et vérifiez la configuration du proxy dans [Configuration réseau](/docs/fr/network-config)

<h3 id="api-returned-an-empty-or-malformed-response">
  L'API a retourné une réponse vide ou malformée
</h3>

Claude Code affiche cette erreur lorsque sa nouvelle tentative sans streaming d'une requête en streaming échouée obtient un statut HTTP de succès mais le corps n'est pas un message API Claude : généralement une erreur HTML ou une page de connexion, un corps vide, ou du JSON dans un autre format. Un proxy, une passerelle ou une page de connexion réseau répondant à la place de l'API est la source habituelle. Claude Code ne réessaye pas la requête, et le tour se termine avec cette erreur.

```text theme={null}
API returned an empty or malformed response (HTTP 200) — check for a proxy or gateway intercepting the request.
```

Après cette ouverture, le message rapporte ce qui est revenu et quelle requête a échoué :

* Une clause `Response:` avec le type de contenu, le type de corps, comme `body is an HTML page` ou `empty body`, sa taille en octets, et si la réponse portait un id de requête Anthropic. Lorsque la réponse nomme un serveur reconnaissable, comme `nginx` ou `cloudflare`, ou porte des en-têtes intermédiaires, comme `cf-ray` ou `via`, la clause les liste aussi.
* Une phrase nommant l'id de la requête en streaming échouée et l'échec qui a déclenché la nouvelle tentative. Lorsqu'un flux s'était ouvert avant l'échec, il rapporte également combien d'événements de flux sont arrivés et, s'il y en avait, combien de temps le flux avait été silencieux lorsque la tentative a échoué.

Avant la v2.1.234, le message se terminait après `intercepting the request`.

Avant la v2.1.271, une réponse qui portait un message API valide sous un type de contenu non-JSON comme `text/plain` terminait également le tour avec cette erreur. Certaines passerelles LLM utilisent ce type de contenu pour la réponse sans streaming.

**À faire :**

* Lisez la clause `Response:` pour voir quel système a répondu. Un corps HTML, pas d'id de requête Anthropic, ou un serveur nommé comme `nginx` ou `cloudflare` signifie que quelque chose entre Claude Code et l'API a répondu à sa place
* Si vous routez via une [passerelle LLM](/docs/fr/llm-gateway-connect#troubleshoot-gateway-errors), testez la route avec une requête directe et corrigez le saut qui retourne la réponse non-API
* Sur un réseau avec une page de connexion, comme le Wi-Fi invité, complétez la connexion dans un navigateur, puis réessayez
* Si seule la route sans streaming via votre passerelle est cassée, définissez [`CLAUDE_CODE_DISABLE_NONSTREAMING_FALLBACK=1`](/docs/fr/env-vars#variables) pour qu'une requête qui échoue au milieu du flux aille au chemin de nouvelle tentative normal au lieu de ce secours, sauf lorsque le point de terminaison en streaming lui-même retourne `404`, où Claude Code se replie toujours

<h3 id="streaming-response-ended-before-any-complete-data-was-received">
  La réponse en streaming s'est terminée avant que des données complètes ne soient reçues
</h3>

Une réponse en streaming de votre fournisseur de modèle s'est complétée sans livrer de données utilisables, donc Claude Code a renvoyé la requête sans streaming pour terminer le tour. Claude Code affiche l'avertissement une fois par session, dans les sessions interactives uniquement. Avant la v2.1.239, Claude Code réessayait silencieusement sans streaming.

```text theme={null}
Streaming response ended before any complete data was received. Retrying without streaming. If this keeps happening, check any proxy or gateway between Claude Code and your model provider.
```

Claude Code envoie chaque requête affectée deux fois : la tentative en streaming vide et la nouvelle tentative. La cause habituelle est un proxy ou une passerelle qui consomme ou transforme le corps de la réponse en streaming en chemin.

**À faire :**

* Configurez tout proxy ou passerelle entre Claude Code et votre fournisseur de modèle pour passer les corps de réponse en streaming et leurs en-têtes sans modification
* Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), consultez [Erreurs de streaming derrière une passerelle ou un proxy](/docs/fr/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy) pour les exigences d'en-tête et de corps

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  La réponse en streaming Bedrock a un content-type inattendu
</h3>

Une passerelle ou un proxy entre Claude Code et [Amazon Bedrock](/docs/fr/amazon-bedrock) transforme le corps de la réponse en streaming ou son en-tête `Content-Type`. Amazon Bedrock diffuse les réponses en tant que `application/vnd.amazon.eventstream`. Plutôt que de décoder un corps qu'il ne peut pas lire, Claude Code rejette une réponse en streaming réussie qui rapporte un content-type différent. Claude Code ne réessaye pas la requête.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Avant la v2.1.208, la même mauvaise configuration s'affichait comme `API Error: Truncated event message received` après que la réponse entière ait été mise en mémoire tampon.

**À faire :**

* Configurez la passerelle pour passer le corps de la réponse `InvokeModelWithResponseStream` et son en-tête `Content-Type` sans modification. Un intermédiaire qui réemet le flux en tant qu'événements envoyés par le serveur est une cause courante.
* Définir [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/fr/env-vars) masque cette erreur, mais Claude Code ne décode pas un corps binaire sous un en-tête réécrit, donc ces requêtes se replient sur un chemin plus lent sans streaming. Consultez [Erreurs de streaming derrière une passerelle ou un proxy](/docs/fr/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  Erreurs de certificat SSL
</h3>

Un proxy ou un appareil de sécurité sur votre réseau intercepte le trafic TLS avec son propre certificat, et Claude Code ne lui fait pas confiance.

```text theme={null}
Unable to connect to API: SSL certificate verification failed (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
Unable to connect to API: Self-signed certificate detected (SELF_SIGNED_CERT_IN_CHAIN). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
```

Avant la v2.1.273, les deux messages se terminaient à `Check your proxy or corporate SSL certificates`, sans le code OpenSSL ou l'indice `NODE_EXTRA_CA_CERTS`.

À partir de la v2.1.199, une défaillance de validation de certificat n'est pas réessayée, donc cette erreur apparaît à la première tentative au lieu d'après le [budget de nouvelle tentative](#automatic-retries) complet. Les versions antérieures passaient quelques minutes à réessayer avant de l'afficher. Les conditions TLS transitoires, comme un délai d'expiration de poignée de main, réessaient toujours.

Pendant `/login` et la vérification de connectivité au démarrage, la même défaillance produit un message différent :

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), les requêtes que Claude Code lui-même envoie à AWS, comme les appels de rôle STS et SSO, la découverte de modèle, et les vérifications de l'assistant de configuration, dépendent de la même configuration de certificat. Consultez [Erreurs de certificat derrière un proxy qui inspecte TLS](/docs/fr/amazon-bedrock#certificate-errors-behind-a-tls-inspecting-proxy).

**À faire :**

* Exportez le bundle CA de votre organisation et pointez Claude Code vers celui-ci avec `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Consultez [Configuration réseau](/docs/fr/network-config#custom-ca-certificates) pour les instructions de configuration complètes
* Ne définissez pas `NODE_TLS_REJECT_UNAUTHORIZED=0`, qui désactive entièrement la validation de certificat

<h3 id="host-not-allowed-in-a-cloud-session">
  L'hôte n'est pas autorisé dans une session cloud
</h3>

Une requête HTTP sortante d'une session cloud ou d'une routine a été bloquée par la politique réseau de l'environnement.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Vous pouvez également voir un certificat TLS qui ne correspond pas au certificat réel de la destination. Les sessions cloud routent le trafic sortant via un proxy qui applique la politique réseau, donc un certificat non-correspondant signifie que le proxy a terminé la connexion, pas la destination.

Ce n'est pas un problème réseau côté client. Les sessions cloud et les [routines](/docs/fr/routines) s'exécutent à l'intérieur d'une VM en sandbox dont le trafic sortant via le réseau de la session est filtré selon la [liste d'autorisation de l'environnement cloud](/docs/fr/cloud-environments) ; les [opérations GitHub](/docs/fr/cloud-environments#github-proxy) et le trafic du connecteur MCP utilisent des canaux séparés, c'est pourquoi ils peuvent continuer à fonctionner tandis que d'autres hôtes sont bloqués. L'environnement **Default** utilise l'accès **Trusted**, qui permet la [liste d'autorisation par défaut](/docs/fr/cloud-environments#default-allowed-domains) des registres de paquets, des API de fournisseurs cloud, des registres de conteneurs et des domaines de développement courants et bloque les autres domaines sur ce chemin.

**À faire :**

Ces étapes modifient l'un de vos propres environnements. Un [environnement partagé par l'organisation](/docs/fr/cloud-environments#organization-shared-environments) s'ouvre en lecture seule dans le sélecteur, donc demandez à un propriétaire de modifier son accès réseau à partir de la page **Cloud environments** dans les [paramètres d'administration](https://claude.ai/admin-settings).

* Ouvrez la routine pour l'édition, ou démarrez une session cloud. Sélectionnez l'icône cloud affichant le nom de votre environnement, comme **Default**, pour ouvrir le sélecteur. Survolez votre environnement et cliquez sur l'icône des paramètres.
* Dans la boîte de dialogue **Update cloud environment**, changez **Network access** de **Trusted** à **Custom**, puis ajoutez le domaine bloqué à **Allowed domains**. Entrez un domaine par ligne. Cochez **Also include default list of common package managers** pour conserver la [liste d'autorisation par défaut](/docs/fr/cloud-environments#default-allowed-domains) aux côtés de vos domaines personnalisés. Sélectionnez **Full** à la place si vous voulez un accès sans restriction.
* Cliquez sur **Save changes**. La prochaine exécution utilise la liste d'autorisation mise à jour.

Consultez [Network access](/docs/fr/cloud-environments#network-access) pour les niveaux d'accès et la liste d'autorisation par défaut. Les sessions CLI locales ne sont pas affectées par cette politique.

<h3 id="the-proxy-refused-the-connection">
  Le proxy a refusé la connexion
</h3>

Vous voyez ce message lorsque Claude lit un [artifact](/docs/fr/artifacts) via le proxy que vous avez défini dans `HTTPS_PROXY` ou une [variable proxy](/docs/fr/network-config#environment-variables) associée. Le contenu des artifacts provient de `*.frame.claudeusercontent.com`, donc Claude Code envoie d'abord au proxy une requête `CONNECT` lui demandant d'ouvrir un tunnel vers cet hôte. Lorsque le proxy refuse, rien n'atteint l'hôte, et le message porte le statut HTTP du proxy :

```text theme={null}
artifact content fetch failed (proxy refused the connection: HTTP 407)
artifact content fetch failed (proxy refused the connection: HTTP 403)
the proxy refused the connection to the artifact's content host (HTTP 502)
```

Le statut est la réponse du proxy au `CONNECT`. L'hôte n'a jamais répondu, donc chaque statut pointe vers un correctif différent :

* `HTTP 407` : le proxy nécessite des identifiants qu'il n'a pas reçus. Mettez-les dans l'URL du proxy, comme [Basic authentication](/docs/fr/network-config#basic-authentication) le montre.
* `HTTP 403` : le proxy refuse de tunneler vers `*.frame.claudeusercontent.com`. Demandez à celui qui gère le proxy d'autoriser cet hôte, que [Network access requirements](/docs/fr/network-config#network-access-requirements) liste.
* Tout autre statut, comme `HTTP 502` : le proxy n'a pas ouvert le tunnel pour sa propre raison, comme l'échec à atteindre l'hôte. Recherchez le statut dans les journaux du proxy.
* `unreadable reply` à la place d'un statut : tout ce qui se trouve à l'adresse du proxy n'a pas répondu avec une ligne de statut HTTP. Vérifiez que l'adresse est un proxy HTTP.

**À faire :**

* Vérifiez l'adresse et les identifiants dans la variable proxy, comme [Proxy configuration](/docs/fr/network-config#proxy-configuration) le décrit, puis exécutez `curl -x http://proxy.example.com:8080 -I https://api.anthropic.com` à partir du shell dans lequel vous démarrez Claude Code, en utilisant votre propre URL de proxy. Sur Windows PowerShell, exécutez `curl.exe`. Si cette sonde échoue de la même manière, corrigez d'abord la configuration du proxy. Si elle réussit, le refus est spécifique à l'hôte des artifacts.
* Si votre réseau permet à Claude Code d'atteindre l'hôte des artifacts directement, ajoutez `.frame.claudeusercontent.com` à [`NO_PROXY`](/docs/fr/network-config#environment-variables). Gardez l'entrée étroite : une entrée `.claudeusercontent.com` plus large contourne également le proxy pour `bridge.claudeusercontent.com`, que les organisations avec [IP allowlisting](/docs/fr/network-config#organization-ip-allowlists-and-proxy-egress) doivent garder sur le proxy.

Avant la v2.1.238, Claude Code rapportait un tunnel refusé comme une erreur réseau générique.

<h3 id="the-cloud-environments-service-returned-an-empty-or-unexpected-response">
  Le service des environnements cloud a retourné une réponse vide ou inattendue
</h3>

Claude Code demande votre liste d'[environnements cloud](/docs/fr/cloud-environments) à plusieurs points, comme lorsque vous créez une session cloud à partir de la CLI ou exécutez [`/remote-env`](/docs/fr/cloud-environments#select-an-environment-from-the-cli). Lorsqu'il ne peut pas lire la réponse du serveur, il affiche l'un de ces messages :

```text theme={null}
The cloud environments service returned an empty response (HTTP 200 with no body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 with a non-JSON body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 without a usable environments list). This is usually temporary — try again in a moment.
```

Le serveur a accepté la requête mais a répondu avec un corps qui n'est pas la liste des environnements : vide, pas JSON, ou JSON sans la liste. Cela accompagne généralement une perturbation côté service et s'efface de lui-même. Selon la surface qui a demandé la liste, Claude Code peut ajouter un préfixe, comme `couldn't list environments:` dans la boîte de dialogue `/remote-env`.

**À faire :**

* Réessayez l'action. Claude Code demande la liste à nouveau chaque fois
* Si le message continue d'apparaître, vérifiez [status.claude.com](https://status.claude.com) pour les incidents actifs

Avant la v2.1.236, Claude Code affichait une TypeError JavaScript brute au lieu de ces messages.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Impossible de se reconnecter à votre session Remote Control
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

La reprise avec `claude --resume` ou `claude --continue` se reconnecte à la session [Remote Control](/docs/fr/remote-control) enregistrée dans cette conversation. Ce message signifie que la reconnexion a échoué pour une raison qui peut être temporaire, comme une interruption réseau ou une erreur serveur, donc Claude Code ne peut pas confirmer si la session distante existe toujours. Votre session locale continue de s'exécuter sans Remote Control.

**À faire :**

* Exécutez `/remote-control` pour réessayer la connexion
* Démarrez une nouvelle session avec `claude --remote-control` pour créer une nouvelle session Remote Control
* Pour les autres messages de démarrage Remote Control, consultez [Troubleshoot Remote Control](/docs/fr/remote-control#troubleshooting)

Si le serveur rapporte à la place que la session précédente est partie, vous ne voyez pas ce message. Claude Code démarre une nouvelle session à sa place ou affiche [`Previous session is unavailable — run /remote-control to start a new one`](/docs/fr/remote-control#previous-session-is-unavailable), selon [l'enregistrement de reconnexion de la conversation](/docs/fr/remote-control#resume-outcomes). De la v2.1.227 à la v2.1.231, Claude Code affichait un message qui commence par `Remote Control could not resume the previous session under the current login` à la place, et [les versions antérieures se comportaient différemment à nouveau](/docs/fr/remote-control#reconnect-history).

<h3 id="sessions-ended-while-this-machine-was-offline">
  Les sessions se sont terminées alors que cette machine était hors ligne
</h3>

Claude Code affiche ce message dans le terminal exécutant [`claude remote-control`](/docs/fr/remote-control#start-a-remote-control-session) après que votre machine ait été hors ligne assez longtemps pour que le serveur nettoie l'environnement Remote Control que votre machine servait. Les sessions dans cet environnement se sont terminées, et vous ne pouvez pas les reprendre. Le nombre est le nombre de sessions qui se sont terminées.

```text theme={null}
2 sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.
```

**À faire :**

* Lorsque Claude Code liste les worktrees conservés sous ce message, récupérez tout travail non validé à partir d'eux
* Exécutez `claude remote-control` pour démarrer un environnement frais

<h3 id="couldnt-share-the-transcript">
  Impossible de partager la transcription
</h3>

Après que vous ayez accepté de partager votre transcription de session à partir d'une invite d'enquête, comme l'[enquête de qualité de session](/docs/fr/data-usage#session-quality-surveys), Claude Code la télécharge vers Anthropic, ou enregistre une archive locale à la place sur les fournisseurs tiers, sur les sessions de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), et lorsqu'aucune identifiant Anthropic n'est disponible. Ce message signifie que le partage ne s'est pas complété.

```text theme={null}
Couldn't share the transcript.
```

Le téléchargement doit tenir dans une limite de 8 MiB. Sur une longue session, Claude Code supprime progressivement des parties du partage, les paramètres du modèle de la dernière requête d'abord, puis la conversation structurée et les transcriptions des sous-agents, et affiche ce message uniquement lorsqu'aucune version réduite ne peut être envoyée ou qu'une erreur réseau ou serveur arrête le téléchargement. Lorsque Claude Code enregistre une archive locale à la place, le message signifie qu'il n'a pas pu écrire l'archive.

**À faire :**

* Exécutez `/feedback` pour envoyer la transcription avec une description de ce qui s'est passé. Consultez [Report an error](#report-an-error) si `/feedback` n'est pas disponible dans votre environnement
* Si d'autres requêtes échouent aussi, vérifiez votre connexion réseau et consultez [Impossible de se connecter à l'API](#unable-to-connect-to-api)

<h2 id="request-errors">
  Erreurs de requête
</h2>

Ces erreurs concernent le contenu de votre requête. La plupart proviennent de l'API après qu'elle ait rejeté la requête ; quelques-unes sont produites localement par Claude Code avant l'envoi de toute requête.

<h3 id="prompt-is-too-long">
  L'invite est trop longue
</h3>

La conversation plus les fichiers joints dépasse la fenêtre de contexte du modèle.

```text theme={null}
Prompt is too long
```

Dans une session interactive, Claude Code affiche cette erreur comme :

```text theme={null}
Context limit reached · /compact or /clear to continue
```

La ligne nomme uniquement `/clear` quand [`DISABLE_COMPACT`](/docs/fr/env-vars) est défini. Les formes plus longues de l'erreur, comme la forme d'échec de compaction ci-dessous, conservent le libellé `Prompt is too long ·`. Dans la sortie `-p` et la transcription, le texte reste `Prompt is too long`.

Quand vous avez désactivé la compaction automatique dans vos [paramètres utilisateur](/docs/fr/settings-reference#autocompactenabled), la ligne dit aussi :

```text theme={null}
Context limit reached · /compact or /clear to continue · auto-compact is off · /config to turn it on
```

Le bouton **Auto-compact** dans `/config` écrit `autoCompactEnabled` dans les paramètres utilisateur. L'indice n'apparaît que quand une modification `/config` prendrait effet. Par exemple, il n'apparaît pas quand [`DISABLE_AUTO_COMPACT`](/docs/fr/env-vars) ou [`DISABLE_COMPACT`](/docs/fr/env-vars) a désactivé la compaction automatique. Il n'apparaît pas non plus quand une portée de priorité plus élevée, comme les paramètres de projet ou gérés, définit `autoCompactEnabled` à `false`. Avant v2.1.235, la ligne ne contenait aucun indice de compaction automatique.

Amazon Bedrock signale cette condition comme `Input is too long for requested model.`, que Claude Code traite de la même manière. Avant v2.1.217, Claude Code ne reconnaissait pas le libellé Bedrock, donc la compaction automatique ne s'est jamais déclenchée et `/compact` a échoué avec la même erreur.

Une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway-config#upstream-error-messages) signale cette condition comme `capability_rejected: prompt_too_long` quand une source cloud en amont rejette la requête dans la forme d'erreur propre du fournisseur. Claude Code traite le jeton de la même manière que `Prompt is too long`. Avant v2.1.228, Claude Code ne reconnaissait pas le jeton, donc la compaction automatique ne s'est pas déclenchée.

Quand la compaction automatique s'est exécutée sur ce tour et a échoué sur une erreur sous-jacente, comme un modèle indisponible ou une défaillance d'authentification, le message nomme cette erreur après un séparateur :

```text theme={null}
Prompt is too long · automatic compaction failed: <the underlying error>
```

Résolvez d'abord l'erreur nommée ; `/compact` échoue sur la même erreur jusqu'à ce que vous le fassiez. Avant v2.1.229, une compaction automatique échouée affichait `Prompt is too long` sans la cause.

Quand la compaction automatique s'exécute sur cette erreur, elle résume normalement vos échanges les plus anciens et conserve les plus récents. En dernier recours, Claude Code résume différemment :

* Quand il ne peut pas résumer un échange complet, Claude Code conserve votre invite la plus récente mot pour mot et résume tout ce qui la précède.
* Dans ce cas, quand la conversation ne se termine pas par votre invite, Claude Code résume la conversation entière à la place.

Claude Code ignore cette récupération quand le contenu qu'il porterait en avant ne contient aucune réponse du modèle et moins d'environ 1 000 jetons de votre propre texte, comme une courte nouvelle tentative envoyée après un collage surdimensionné. Exécutez `/clear` pour recommencer. Avant v2.1.269, la compaction échouait chaque fois qu'elle ne pouvait pas résumer un échange complet, donc une session dans cet état rencontrait cette erreur à chaque tour.

Une conversation à un seul échange n'a pas de tours antérieurs à résumer. Quand la compaction automatique aurait dû s'exécuter sur un, Claude Code ignore la tentative et explique ce qui remplit la requête à la place. Quand l'API ne signale pas les nombres de jetons dans son erreur, le message se lit :

```text theme={null}
Prompt is too long · this conversation is a single exchange and cannot be compacted — the request size comes mostly from system prompt, tool definitions, or attachments.
```

Quand l'API signale les nombres de jetons dans son erreur, Claude Code les compare avec sa propre estimation de la taille de la conversation pour dire lequel est la majorité de la requête : le contenu propre de la conversation, ou le contenu du message système, des définitions d'outils et des pièces jointes que Claude Code envoie avec. Quand le contenu propre de la conversation est la majorité de la requête, le message se lit :

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) and this conversation's own content is most of it. A single-exchange conversation cannot be compacted; start with less content (smaller files or pasted text).
```

Quand la majorité de la requête est en dehors de la conversation, le message se lit :

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) but this conversation is only ~<conversation tokens> tokens — the rest is system prompt, tool definitions, and attachment content. A single-exchange conversation cannot be compacted; reduce attached files/tools or start with less context.
```

Avant v2.1.162, Claude Code tentait la compaction de toute façon et affichait le `Prompt is too long` nu quand il échouait.

**Que faire :**

* Exécutez `/compact` pour résumer les tours antérieurs et libérer de l'espace, ou `/clear` pour recommencer. Si `/compact` répond `Not enough messages to compact.`, la conversation est un échange unique sans rien d'antérieur à résumer, donc l'espace est occupé par cette invite et ce que Claude Code envoie avec chaque requête : exécutez `/clear` et renvoyez avec moins de texte collé ou des pièces jointes plus petites, ou réduisez les définitions d'outils et les fichiers mémoire en utilisant les étapes ci-dessous
* Exécutez `/context` pour voir une ventilation de ce qui consomme la fenêtre : message système, outils, fichiers mémoire et messages
* Désactivez les serveurs MCP que vous n'utilisez pas avec `/mcp disable <name>` pour supprimer leurs définitions d'outils du contexte
* Réduisez les fichiers mémoire `CLAUDE.md` volumineux, ou déplacez les instructions dans les [règles à portée de chemin](/docs/fr/memory#path-specific-rules) qui se chargent uniquement quand pertinent
* Les sous-agents héritent de chaque définition d'outil MCP de la session parent, ce qui peut remplir leur fenêtre de contexte avant le premier tour. Désactivez les serveurs MCP que vous n'utilisez pas avant de générer des sous-agents.
* La compaction automatique est activée par défaut et prévient normalement cette erreur. Si vous l'avez désactivée dans `/config` ou avec [`DISABLE_AUTO_COMPACT`](/docs/fr/env-vars), réactivez-la. Si vous la gardez désactivée, exécutez `/compact` vous-même avant que la fenêtre se remplisse.

Voir [Explorez la fenêtre de contexte](/docs/fr/context-window) pour une vue interactive de la façon dont le contexte se remplit.

<h3 id="context-exceeds-the-token-limit">
  Le contexte dépasse la limite de jetons
</h3>

`/context` affiche cet avertissement en haut de sa sortie quand la conversation a dépassé la fenêtre de contexte du modèle. Les requêtes échouent avec [`Prompt is too long`](#prompt-is-too-long) jusqu'à ce que vous libériez de l'espace. Une session interactive affiche cette erreur comme la ligne `Context limit reached`.

```text theme={null}
Context exceeds the 200k-token limit by 94k tokens — run /compact or /clear to continue.
```

Quand la limite que vous avez dépassée est une fenêtre de compaction, comme la limite 200K sur les modèles 1M-contexte, l'avertissement se lit différemment. Une fenêtre de compaction peut se situer en dessous de la fenêtre de contexte du modèle, donc les requêtes au-delà peuvent encore réussir.

```text theme={null}
Context is 94k tokens past the 200k-token compaction window — run /compact to reduce usage.
```

Les deux formes nomment `/clear` au lieu de `/compact` quand vous avez défini [`DISABLE_COMPACT`](/docs/fr/env-vars).

**Que faire :**

* Dans une conversation multi-tours, exécutez `/compact` pour résumer les tours antérieurs et libérer de l'espace. Pour recommencer à la place, exécutez `/clear`
* Pour plus de façons de réduire l'utilisation, voir [L'invite est trop longue](#prompt-is-too-long)

Avant v2.1.216, `/context` affichait l'utilisation au-dessus de 100 % sans ligne d'avertissement expliquant ce que cela signifiait ou comment récupérer.

<h3 id="error-during-compaction-conversation-too-long">
  Erreur lors de la compaction : Conversation trop longue
</h3>

`/compact` lui-même a échoué parce qu'il n'y a pas assez de contexte libre pour contenir le résumé qu'il produit.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Cela peut se produire quand la fenêtre est déjà pleine au moment où la compaction automatique se déclenche, ou quand vous exécutez `/compact` après avoir vu [`Prompt is too long`](#prompt-is-too-long). Dans une session interactive, cette erreur est la ligne `Context limit reached`.

**Que faire :**

* Appuyez deux fois sur Échap pour ouvrir la liste des messages et revenir plusieurs tours en arrière. Cela supprime les messages les plus récents du contexte. Puis exécutez `/compact` à nouveau.
* Si revenir en arrière ne libère pas assez d'espace, exécutez `/clear` pour démarrer une nouvelle session. Votre conversation précédente est préservée et peut être rouverte avec `/resume`.

Ce message et d'autres défaillances `/compact` s'affichent dans le style d'erreur. Avant v2.1.216, ils s'affichaient dans le même style atténué que la sortie de commande réussie, donc vous pouviez lire une compaction échouée comme un succès.

<h3 id="request-too-large">
  Requête trop grande
</h3>

Le corps de la requête brute a dépassé la limite de 32 Mo de l'API avant la tokenisation, généralement en raison de contenu collé volumineux, de résultats d'outils ou de pièces jointes. Cette limite est distincte de la [fenêtre de contexte](#prompt-is-too-long).

```text theme={null}
Request too large (max 32MB). Accumulated images and attachments in the conversation pushed the request over the limit. Run /compact, or double press esc to go back and remove attachments.
```

Quand la requête est allée directement à l'API Claude et que l'API elle-même l'a rejetée, Claude Code mesure la conversation et formule le message selon que la récupération peut fonctionner. Via un proxy, une passerelle ou un fournisseur cloud, vous obtenez le message général. Les formes mesurées :

* `Request too large (max 32MB; 20.1MB of about 33.4MB is images or documents).` : les images ou documents ont poussé la requête au-delà de la limite. Claude Code réessaie en les supprimant.
* `Request too large for the API's 32MB request limit` : les messages seuls dépassent la limite, donc le message dit `compacting cannot make it fit` et Claude Code ne réessaie pas. En [mode non-interactif](/docs/fr/headless), le message vous dit de réduire l'entrée ou de démarrer une nouvelle session à la place.

Avant v2.1.212, les conversations avec assez d'images accumulées échouaient à chaque tour avec `Request too large (max 32MB). Double press esc to go back and try with a smaller file.` Avant v2.1.229, Claude Code affichait le conseil sur les pièces jointes pour chaque rejet, même quand la compaction ne pouvait pas aider.

**Que faire :**

* Si le message dit `compacting cannot make it fit`, appuyez deux fois sur Échap pour revenir en arrière au-delà du tour qui a ajouté le contenu volumineux, ou exécutez `/clear` pour recommencer
* Sinon, exécutez `/compact`, qui supprime les images et pièces jointes accumulées
* Référencez les fichiers volumineux par chemin au lieu de coller leur contenu, afin que Claude puisse les lire par morceaux
* Pour les images, voir [L'image était trop grande](#image-was-too-large) ci-dessous

<h3 id="image-was-too-large">
  L'image était trop grande
</h3>

Une image collée ou jointe dépasse les limites de taille ou de dimension de l'API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code remplace l'image non traitée par un espace réservé textuel et réessaie, donc les messages suivants réussissent. Sur les versions antérieures à 2.1.142, une image collée pouvait rester dans la conversation et répéter la même erreur à chaque message suivant. Pour récupérer sur ces versions, appuyez deux fois sur Échap et revenir en arrière au-delà du tour où l'image a été ajoutée.

**Que faire :**

* Redimensionnez l'image avant de la coller. L'API accepte les images jusqu'à 8 000 pixels sur le côté le plus long pour une seule image, ou 2 000 pixels quand de nombreuses images sont en contexte.
* Prenez une capture d'écran plus serrée de la région pertinente au lieu de l'écran complet

<h3 id="unable-to-resize-image">
  Impossible de redimensionner l'image
</h3>

Claude Code n'a pas pu réduire une image jointe avant de l'envoyer à l'API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
Unable to resize image — it is a CMYK JPEG, which Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Re-save it as an RGB PNG or JPEG and try again.
Unable to resize image — it is an animated WebP whose first frame Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Save its first frame as a PNG or JPEG and try again.
Unable to resize image — its pixels could not be decoded (the file may be damaged, or use an encoding Claude Code cannot read), and it is over the … API limit (… raw, … base64), so it cannot be sent. Re-save it as a PNG or JPEG and try again.
```

Claude Code redimensionne normalement les grandes images automatiquement. Ces erreurs signifient que l'image n'a pas pu être décodée ou redimensionnée pour tenir dans les limites de l'API.

**Que faire :**

* Si le message vous demande de convertir l'image, convertissez-la en PNG, JPEG, GIF ou WebP et joignez-la à nouveau. Claude Code peut vérifier les dimensions pour ces formats à partir de l'en-tête du fichier, sans décoder l'image.
* Si le message signale une limite de dimension ou de taille, redimensionnez ou recompressez l'image en dessous de cette limite avant de la joindre.
* Si le message nomme une cause, comme un JPEG CMYK, un WebP animé ou un fichier possiblement endommagé, réenregistrez l'image dans le format que le message suggère et joignez-la à nouveau.

<h3 id="pdf-errors">
  Erreurs PDF
</h3>

Le PDF que vous avez joint n'a pas pu être traité. Les messages sont affichés ici dans leur forme non-interactive ; dans une session interactive, ils vous invitent plutôt à appuyer deux fois sur Échap et à réessayer.

```text theme={null}
PDF too large (max 100 pages, 20MB). Try reading the file a different way (e.g., extract text with pdftotext).
PDF is password protected. Try using a CLI tool to extract or convert the PDF.
The PDF file was not valid. Try converting it to text first (e.g., pdftotext).
```

**Que faire :**

* Pour les PDF surdimensionnés, demandez à Claude de lire une plage de pages avec l'outil Read au lieu de joindre le fichier entier, ou extrayez le texte avec un outil comme `pdftotext` et référencez le fichier de sortie par chemin
* Pour les PDF protégés ou invalides, supprimez le mot de passe ou réexportez le fichier depuis son application source, puis réessayez

<h3 id="extra-inputs-are-not-permitted">
  Les entrées supplémentaires ne sont pas autorisées
</h3>

Un proxy ou une passerelle LLM entre Claude Code et l'API a supprimé l'en-tête de requête `anthropic-beta`, donc l'API a rejeté les champs qui en dépendent.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code envoie des champs bêta uniquement comme `context_management` et `effort` aux côtés d'un en-tête `anthropic-beta` qui les active. Quand une passerelle transfère le corps mais supprime l'en-tête, l'API voit des champs qu'elle ne reconnaît pas.

**Que faire :**

* Configurez votre passerelle pour transférer l'en-tête `anthropic-beta`. Voir [transmission de fonctionnalités](/docs/fr/llm-gateway-protocol#feature-pass-through) pour ce que les passerelles doivent transférer.
* En dernier recours, définissez [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/fr/env-vars) avant de lancer. [Désactiver les capacités de pré-version](/docs/fr/llm-gateway-protocol#disable-pre-release-capabilities) couvre la portée exacte.

<h3 id="tool-input-schema-is-invalid">
  Le schéma d'entrée de l'outil est invalide
</h3>

Un outil dans la requête a déclaré un `input_schema` qui échoue la validation JSON Schema de l'API, donc l'API a rejeté la requête entière. Le nombre après `tools.` est la position de l'outil défaillant dans la liste d'outils de la requête, pas un nom que vous pouvez rechercher.

```text theme={null}
API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid
API Error: 400 ... tools.N.custom.input_schema.properties: Property keys should match pattern '^[a-zA-Z0-9_.-]{1,64}$'
```

La première forme signifie que le schéma n'est pas un brouillon JSON Schema valide 2020-12. La seconde signifie qu'un nom de propriété de niveau supérieur ne correspond pas au motif que le message cite.

Claude Code [exclut les outils MCP dont le schéma d'entrée échouerait cette validation](/docs/fr/mcp#tools-with-invalid-input-schemas) quand il charge les outils d'un serveur, donc les requêtes ne contiennent normalement jamais un.

Sur un [déploiement où la récupération de drapeaux est désactivée](/docs/fr/env-vars#features-that-need-feature-flag-fetching), ou sur une machine dont les drapeaux ne sont jamais arrivés, Claude Code enregistre dans le journal du serveur quel outil serait rejeté mais l'envoie de toute façon, donc cette erreur peut toujours se produire.

L'erreur peut aussi se produire pour un outil dont le schéma déclare un dialecte JSON Schema autre que le brouillon 2020-12 dans `$schema`. Claude Code ne vérifie pas ces schémas par rapport au méta-schéma JSON Schema, bien que la vérification du nom de propriété de niveau supérieur s'applique toujours.

Avant v2.1.216, aucun déploiement n'exécutait les vérifications d'exclusion.

**Que faire :**

* Si votre version de Claude Code est antérieure à v2.1.216, exécutez `claude update`.
* Supprimez ou [désactivez](/docs/fr/mcp#disable-a-server-without-removing-it) le serveur MCP qui déclare le schéma invalide. L'erreur nomme l'outil uniquement par position. Sur v2.1.216 ou ultérieur, vérifiez le journal de chaque serveur pour une ligne nommant un outil dont le schéma d'entrée serait rejeté. Si aucun journal n'en nomme un, désactivez les serveurs un par un.
* Si vous maintenez le serveur, corrigez le `input_schema` de l'outil. Le schéma doit être un JSON Schema valide, et les noms de propriété de niveau supérieur doivent faire 1 à 64 caractères et utiliser uniquement des lettres ASCII et des chiffres, `_`, `.` et `-`. Voir [Outils avec schémas d'entrée invalides](/docs/fr/mcp#tools-with-invalid-input-schemas).

<h3 id="theres-an-issue-with-the-selected-model">
  Il y a un problème avec le modèle sélectionné
</h3>

Le nom du modèle configuré n'a pas été reconnu ou votre compte n'a pas accès à celui-ci. À partir de v2.1.160, l'indice de fin, affiché ici dans sa forme interactive, varie selon la surface.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Que faire :**

* **CLI interactif** : exécutez `/model` pour choisir parmi les modèles disponibles pour votre compte.
* **Mode non-interactif (`-p`)** : passez `--model` avec un alias ou un ID valide, ou définissez [`ANTHROPIC_MODEL`](/docs/fr/env-vars). Le texte d'erreur affiche `Run --model` sur cette surface.
* **Agent SDK** : le texte d'erreur omet l'indice car le modèle est défini par programmation. Définissez [`model` sur `Options`](/docs/fr/agent-sdk/typescript#options) en TypeScript ou [`ClaudeAgentOptions(model=...)`](/docs/fr/agent-sdk/python#claudeagentoptions) en Python, et gérez l'erreur structurée `model_not_found` pour afficher votre propre nouvelle tentative ou sélecteur de modèle.
* Utilisez un alias comme `sonnet` ou `opus` au lieu d'un ID complet versionné. Les alias se résolvent à une valeur par défaut maintenue afin qu'ils ne deviennent pas obsolètes. Voir [Configuration du modèle](/docs/fr/model-config).
* Si le mauvais modèle continue de revenir dans le CLI, un ID obsolète est défini quelque part. Vérifiez les endroits où vous pouvez définir un modèle dans [l'ordre de priorité](/docs/fr/model-config#setting-your-model) et supprimez la valeur obsolète.
* Un modèle nouvellement lancé peut être disponible sur l'API Anthropic avant qu'Amazon Bedrock, la plateforme d'agent de Google Cloud ou Microsoft Foundry ne l'offre. Si vous avez épinglé un nouvel ID de modèle sur l'un de ces fournisseurs et voyez cette erreur, vérifiez le catalogue de modèles de votre fournisseur pour la disponibilité dans votre région, et gardez la version précédente épinglée jusqu'à ce que la nouvelle apparaisse là.
* Claude Code signale une connexion claude.ai expirée comme [Connexion expirée](#login-expired), pas comme cette erreur. Avant v2.1.206, une connexion expirée qui ne pouvait plus être actualisée échouait chaque modèle avec cette erreur ; exécutez `/login` si vous voyez cela sur une version plus ancienne.
* Pour les déploiements de la plateforme d'agent de Google Cloud, voir [Dépannage de la plateforme d'agent de Google Cloud](/docs/fr/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Le modèle n'est pas un ID de modèle reconnu
</h3>

La chaîne de modèle que vous avez passée à un changement de modèle n'est pas un alias de modèle, un ID de modèle que cette version de Claude Code connaît, ou un ID qui commence par `claude-`. Les causes habituelles sont une faute de frappe dans l'ID, un nom d'affichage comme `Sonnet 5` où l'ID `claude-sonnet-5` est attendu, ou un alias que seules les versions plus récentes de Claude Code reconnaissent. Claude Code rejette le changement immédiatement. Avant v2.1.200, Claude Code enregistrait la chaîne et échouait à la requête suivante avec [Il y a un problème avec le modèle sélectionné](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

L'indice de fin nomme l'alias ou l'ID de modèle le plus proche. Quand rien n'est assez proche, il se lit `Run /model to see available models.` à la place.

Claude Code produit cette erreur localement au moment où le changement est demandé, avant toute requête API. Elle s'applique quand un modèle est défini via la méthode [Agent SDK](/docs/fr/agent-sdk/typescript) `setModel()`, par une application comme l'[application de bureau](/docs/fr/desktop) qui exécute le CLI Claude Code pour vous, ou quand vous choisissez un modèle à partir d'un appareil connecté via [Contrôle à distance](/docs/fr/remote-control). Avant v2.1.260, la vérification ne couvrait pas les choix de contrôle à distance, donc Claude Code appliquait le choix et la requête suivante échouait avec [Il y a un problème avec le modèle sélectionné](#theres-an-issue-with-the-selected-model).

**Que faire :**

* Exécutez `/model` sans argument pour ouvrir le sélecteur et choisir parmi les modèles disponibles pour votre compte, puis passez l'alias ou l'ID affiché là
* Si vous avez utilisé un alias qu'une version plus récente de Claude Code supporte, exécutez `claude update`. Un ID complet qui commence par `claude-` passe cette vérification locale même quand le modèle est plus récent que votre version de Claude Code. Le serveur peut toujours exiger une version minimale pour ce modèle ; voir [Claude Code ne supporte pas ce modèle](#claude-code-does-not-support-this-model).
* Un modèle enregistré avant v2.1.200 n'est pas réparé par cette vérification. Si une valeur obsolète continue de revenir, supprimez-la des emplacements listés sous [Définir votre modèle](/docs/fr/model-config#setting-your-model).
* La vérification s'exécute uniquement sur l'API Anthropic. Sur tout autre fournisseur ou passerelle, y compris un `ANTHROPIC_BASE_URL` personnalisé, le fournisseur définit les noms de modèles, donc Claude Code accepte n'importe quelle chaîne et la transmet. Claude Code peut toujours écrire la [ligne de diagnostic de modèle non reconnu](#unrecognized-model-id-on-a-request) au moment de la requête, sur chaque fournisseur.

<h3 id="model-not-found">
  Modèle non trouvé
</h3>

Vous avez choisi un modèle avec `/model <name>` et Claude Code n'a pas pu confirmer qu'un modèle avec ce nom existe. Quand le nom n'est pas un [alias de modèle](/docs/fr/model-config#model-aliases) ou une autre orthographe que Claude Code accepte localement, `/model` le vérifie avec une requête API minimale, et cette erreur est généralement la réponse de votre point de terminaison API. Un nom qui ne peut pas du tout être un ID de modèle, comme un contenant des espaces, obtient le même message.

```text theme={null}
Model 'claude-opus-9' not found
```

Sur les fournisseurs avec des ID de modèle spécifiques au fournisseur, le message peut ajouter une suggestion `Try '...' instead` qui nomme l'ID de votre fournisseur pour un modèle de secours.

**Que faire :**

* Exécutez `/model` sans argument et choisissez parmi les modèles disponibles pour votre compte, ou utilisez un [alias de modèle](/docs/fr/model-config#model-aliases) comme `sonnet`, qui se résout à une valeur par défaut maintenue
* Si vous avez tapé un ID complet, vérifiez-le par rapport au catalogue de modèles de votre fournisseur. Un modèle nouvellement lancé peut être disponible sur l'API Anthropic avant que votre fournisseur ou région ne l'offre.
* Avant v2.1.265, `/model` rejetait aussi l'orthographe d'alias `opusplan[1m]` avec cette erreur. Sur ces versions, mettez à jour Claude Code, ou définissez le modèle dans [paramètres](/docs/fr/model-config#setting-your-model) ou avec `--model` à la place.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus n'est pas disponible avec le plan Claude Pro
</h3>

Votre plan d'abonnement actif n'inclut pas le modèle que vous avez sélectionné.

```text theme={null}
Claude Opus is not available with the Claude Pro plan. If you have updated your subscription plan recently, run /logout and /login for the plan to take effect.
```

**Que faire :**

* Exécutez `/model` et sélectionnez un modèle que votre plan inclut
* Si vous avez mis à niveau votre plan récemment et voyez toujours cela, exécutez `/logout` puis `/login`. Le jeton stocké reflète votre plan au moment où vous vous êtes connecté, donc la mise à niveau sur claude.ai ne prend effet dans une session existante que jusqu'à ce que vous vous réauthentifiiez.
* Voir [claude.com/pricing](https://claude.com/pricing) pour savoir quels modèles chaque plan inclut

<h3 id="claude-code-does-not-support-this-model">
  Claude Code ne supporte pas ce modèle
</h3>

L'API a refusé la requête avec un 400 parce que votre version de Claude Code est en dessous d'un minimum requis. Soit le modèle que vous avez sélectionné nécessite une version plus récente, que le serveur vérifie par modèle, soit la politique de votre organisation en nécessite une. Le 400 porte le code d'erreur `claude_code_version_too_old`, et le message dit quel minimum s'applique.

```text theme={null}
API Error: 400 Claude Code 2.1.219 does not support this model; version 2.1.255 or newer is required. Run 'claude update', or update the Claude desktop app, then try again.
```

Le libellé de la politique organisationnelle se lit :

```text theme={null}
API Error: 400 Claude Code 2.1.240 is older than the minimum version required by your organization's policy. Run 'claude update', or update the Claude desktop app, to continue.
```

**Que faire :**

* Exécutez `claude update`, ou mettez à jour l'application de bureau Claude, puis démarrez une nouvelle session
* Pour le libellé par modèle, vous pouvez continuer à travailler dans la session actuelle en basculant vers un autre modèle avec `/model`
* Pour le libellé de la politique organisationnelle, mettez à jour avant de continuer

<h3 id="model-is-restricted-by-your-organizations-settings">
  Le modèle est restreint par les paramètres de votre organisation
</h3>

Votre administrateur d'organisation a désactivé ce modèle dans la console d'administration claude.ai, ou il est exclu par une liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) dans les paramètres gérés. Quand le modèle restreint a été défini avec `--model`, `ANTHROPIC_MODEL` ou le paramètre `model`, Claude Code substitue un modèle autorisé et continue. Taper `/model <name>` pour un modèle restreint est rejeté avec `Run /model to choose a different model.` et la session garde son modèle actuel. L'avis de substitution peut aussi apparaître en milieu de session après qu'un administrateur désactive le modèle sur lequel une session s'exécute dans la console d'administration claude.ai.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Un avis préfixé avec un nom d'agent, de compétence ou de commande signifie que la restriction s'appliquait au [modèle demandé du sous-agent](/docs/fr/sub-agents#choose-a-model) : le sous-agent s'exécute sur le modèle substitué et le modèle de votre session est inchangé. Avant v2.1.223, Claude Code affichait l'avis uniquement pour les sous-agents lancés avec l'outil Agent.

Claude Code traite un alias de famille de modèles, l'un de `opus`, `sonnet`, `haiku` ou `fable`, comme une demande pour cette famille plutôt que pour sa version la plus récente. Sur l'API Anthropic et sur [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), un alias de famille restreint se résout à la version la plus récente de la famille que votre organisation et la liste d'autorisation `availableModels` permettent, et l'avis de substitution nomme cette version. Claude Code rejette `/model <alias>` uniquement quand chaque version de la famille est restreinte. Avant v2.1.205, un alias de famille était substitué ou rejeté en fonction de sa version la plus récente seule, même quand une version plus ancienne de la même famille était autorisée.

**Que faire :**

* Exécutez `/model` pour choisir parmi les modèles que votre organisation autorise. Les modèles restreints sont masqués du sélecteur.
* Si le modèle restreint a été défini dans `--model`, `ANTHROPIC_MODEL`, le champ `model` d'un fichier de paramètres, ou le frontmatter `model` d'un [sous-agent](/docs/fr/sub-agents#choose-a-model), d'une compétence ou d'une commande, supprimez ou mettez à jour cette valeur afin que l'avis ne se reproduise pas
* Si vous avez besoin d'accès au modèle restreint, demandez à votre administrateur d'organisation de l'activer. Voir [Restrictions de modèle organisationnel](/docs/fr/model-config#organization-model-restrictions).

<h3 id="model-switch-was-blocked-by-a-premodelswitch-hook">
  Le changement de modèle a été bloqué par un hook PreModelSwitch
</h3>

Un hook [PreModelSwitch](/docs/fr/hooks#premodelswitch) n'a pas approuvé le changement de modèle que vous ou un client avez demandé, donc la session garde son modèle actuel. Quand le changement provenait d'un hôte [Agent SDK](/docs/fr/agent-sdk/overview) ou [Contrôle à distance](/docs/fr/remote-control) plutôt que d'une commande que vous avez tapée, le message se lit `Model switch blocked by a PreModelSwitch hook` sans nommer le modèle cible.

```text theme={null}
Model switch to Opus 4.6 was blocked by a PreModelSwitch hook: Opus 4.6 is retired for this project. Use a newer model.
```

La raison après le deux-points dit ce qui a refusé le changement :

* **Une raison qu'un hook a écrite** : un hook PreModelSwitch a fourni cette raison quand il a [refusé le changement ou demandé une confirmation](/docs/fr/hooks#premodelswitch-decision-control). Adressez ce qu'il demande, ou choisissez un modèle que vos hooks autorisent.
* **`PreModelSwitch hook <name> did not respond before its timeout`** : un hook qui ne répond pas avant son [délai d'expiration](/docs/fr/hooks#timeouts) bloque le changement. Corrigez la commande qui pend ou augmentez le `timeout` de ce hook, puis changez à nouveau.
* **`confirmation required, and this session cannot ask`** : un hook a répondu `ask` sans raison, et une demande de contrôle n'a aucun moyen d'afficher l'invite de confirmation. Un changement `/model` dans une exécution [`-p`](/docs/fr/headless) signale la même condition avec `(run /model interactively to confirm)` après la raison. Effectuez le changement à partir d'une session interactive, ou changez la décision du hook pour ce modèle.
* **`so organization-managed PreModelSwitch hooks could not be checked`** : Claude Code n'a pas pu dire quels hooks PreModelSwitch vos [plugins gérés](/docs/fr/settings-reference#enabledplugins) d'organisation livrent, par exemple parce qu'un plugin géré n'a pas pu se charger. L'un de ces hooks pourrait bloquer le changement, donc Claude Code refuse plutôt que d'appliquer le changement non vérifié. Le début de la raison nomme ce qui a échoué. Claude Code re-vérifie à chaque tentative de changement, donc une défaillance qui a depuis été effacée cesse de bloquer ; si elle continue d'échouer, exécutez `claude --debug` et changez à nouveau pour capturer les détails, puis corrigez le plugin ou demandez à votre administrateur de le corriger.
* **`a PreModelSwitch hook failed before answering`** ou **`PreModelSwitch hooks were cancelled (the control stream closed) before answering`** : l'exécution du hook s'est terminée sans verdict, et Claude Code ne traite pas cela comme une approbation. Exécutez `claude --debug` pour voir ce qui a échoué, puis changez à nouveau.

Avant v2.1.260, le refus du plugin géré se lisait `plugin hooks could not be loaded, so PreModelSwitch hooks could not be checked; see the debug log`. Claude Code a réessayé le chargement du plugin une fois puis a refusé les changements ultérieurs dans la session, même quand votre organisation ne gérait aucun plugin. Redémarrez la session pour exécuter le chargement du plugin à nouveau sur ces versions.

<h3 id="couldnt-save-it-as-your-default">
  Impossible de l'enregistrer comme valeur par défaut
</h3>

Vous avez choisi un modèle à enregistrer comme valeur par défaut, par exemple avec `/model <name>` ou `Entrée` dans le sélecteur `/model`, et Claude Code n'a pas pu écrire le choix dans votre fichier de paramètres utilisateur, `~/.claude/settings.json`. Le changement lui-même s'est appliqué, donc la session actuelle s'exécute sur le modèle que vous avez choisi, mais votre valeur par défaut est inchangée et la session suivante démarre sur l'ancienne valeur.

```text theme={null}
Set model to Fable 5.1 for this session only · couldn't save it as your default: ~/.claude/settings.json can't be written (EROFS)
```

La raison après le chemin du fichier dit ce qui a échoué :

* **`can't be written (<code>)`** : l'écriture a échoué avec le code d'erreur du système d'exploitation entre parenthèses, comme `EROFS` quand le fichier, ou le fichier auquel il se lie, se trouve sur un système de fichiers qui refuse les écritures. Rendez le fichier inscriptible et changez à nouveau. Si un autre outil génère le fichier, définissez la clé `model` dans cet outil à la place ; voir [Un changement que vous avez fait dans Claude Code est perdu dans les nouvelles sessions](/docs/fr/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **`isn't valid JSON`** : le fichier sur le disque ne s'analyse pas, et Claude Code le laisse intact plutôt que de remplacer le contenu qu'il ne peut pas relire. Corrigez l'erreur de syntaxe, puis changez à nouveau ; voir [Corriger un fichier de paramètres cassé](/docs/fr/settings#fix-a-broken-settings-file).

Un avis se terminant par `couldn't confirm it was saved as your default (~/.claude/settings.json is still being written)` signifie que l'écriture n'avait pas terminé après trois secondes. Elle continue en arrière-plan, donc la valeur par défaut peut toujours être enregistrée ; vérifiez quel modèle votre session suivante démarre, ou exécutez `/model <name>` à nouveau.

Avant v2.1.265, l'avis disait que le modèle était `saved as your default for new sessions` même quand l'écriture a échoué.

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled n'est pas supporté pour ce modèle
</h3>

Votre version de Claude Code est plus ancienne que le minimum pour le modèle sélectionné. Le CLI a envoyé une configuration de réflexion que le modèle n'accepte plus.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Que faire :**

* Exécutez `claude update` et redémarrez Claude Code. Opus 4.7 nécessite v2.1.111 ou ultérieur. Opus 4.8 nécessite v2.1.154 ou ultérieur. Sonnet 5 nécessite v2.1.197 ou ultérieur. Opus 5 nécessite v2.1.219 ou ultérieur. Opus 5.5 nécessite v2.1.280 ou ultérieur
* Si vous ne pouvez pas mettre à jour, exécutez `/model` et sélectionnez Opus 4.6 ou Sonnet 4.6 à la place
* Si vous rencontrez cela dans l'[Agent SDK](/docs/fr/agent-sdk/overview), mettez à jour le package SDK à la place. Opus 4.8 nécessite TypeScript SDK v0.3.154 ou ultérieur et Python SDK v0.2.88 ou ultérieur. Sonnet 5 nécessite TypeScript SDK v0.3.197 ou ultérieur. Opus 5 nécessite TypeScript SDK v0.3.219 ou ultérieur. Opus 5.5 nécessite TypeScript SDK v0.3.280 ou ultérieur

<h3 id="effort-isnt-available-with-thinking-turned-off">
  L'effort n'est pas disponible avec la réflexion désactivée
</h3>

Vous avez désactivé la [réflexion étendue](/docs/fr/model-config#extended-thinking) et avez exécuté à un [niveau d'effort](/docs/fr/model-config#adjust-effort-level) au-dessus de `high`. Le modèle n'accepte pas cette combinaison, donc l'API a rejeté la requête.

```text theme={null}
API Error: Effort 'xhigh' isn't available with thinking turned off on this model · run /effort high to continue, or turn thinking back on (unset MAX_THINKING_TOKENS=0)
```

**Que faire :**

* [Abaissez le niveau d'effort](/docs/fr/model-config#set-the-effort-level) à `high` ou en dessous.
* Réactivez la réflexion, par exemple en désactivant [`MAX_THINKING_TOKENS`](/docs/fr/env-vars) ou en supprimant [`"alwaysThinkingEnabled": false`](/docs/fr/settings-reference#alwaysthinkingenabled) de vos paramètres.

Avant v2.1.242, Claude Code affichait le message propre de l'API : `API Error: 400 output_config.effort 'xhigh' is not supported when thinking is disabled on this model. Use effort 'high' or below, or enable thinking.` Avant v2.1.251, Claude Code envoyait la requête au niveau d'effort que vous avez défini, donc Opus 5 rejetait chaque requête au-dessus de `high` avec la réflexion désactivée. Claude Code envoie maintenant l'effort `high` à la place aux modèles qu'il sait rejeter la combinaison, comme Opus 5, donc sur v2.1.251 ou ultérieur cette erreur vous atteint uniquement à partir d'un modèle que Claude Code ne sait pas rejeter.

<h3 id="thinking-budget-exceeds-output-limit">
  Le budget de réflexion dépasse la limite de sortie
</h3>

Le budget de réflexion étendue configuré dépasse la longueur de réponse maximale, donc il n'y a pas de place pour la réponse réelle.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code ajuste ces valeurs automatiquement sur l'API Anthropic. Vous voyez généralement cette erreur sur Amazon Bedrock ou la plateforme d'agent de Google Cloud quand [`MAX_THINKING_TOKENS`](/docs/fr/env-vars) est défini plus haut que la limite de sortie du fournisseur, ou quand le mode plan augmente le budget de réflexion.

**Que faire :**

* Abaissez `MAX_THINKING_TOKENS`, ou augmentez [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/fr/env-vars) au-dessus du budget de réflexion
* Voir [Réflexion étendue](/docs/fr/model-config#extended-thinking) pour la façon dont le budget interagit avec la longueur de sortie

<h3 id="tool-use-or-thinking-block-mismatch">
  Décalage de bloc d'utilisation d'outil ou de réflexion
</h3>

L'historique de conversation a atteint l'API dans un état incohérent, généralement après qu'un appel d'outil ait été interrompu ou qu'un tour ait été édité en milieu de flux.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 orphaned tool_result in conversation history. Run /rewind to recover the conversation.
API Error: 400 duplicate tool_use ID in conversation history. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Tous les variantes signifient la même chose : la séquence de blocs `tool_use`, `tool_result` et `thinking` dans l'historique ne correspond plus à ce que l'API attend.

**Que faire :**

* Si vous utilisez Opus 4.7 ou Opus 4.8, exécutez d'abord `claude update`. Les versions antérieures à v2.1.156 peuvent déclencher cette erreur lors de l'utilisation normale d'outils, et `/rewind` ne la supprime pas.
* Exécutez `/rewind`, ou appuyez deux fois sur Échap, pour revenir à un point de contrôle avant le tour corrompu et continuer à partir de là. Voir [Points de contrôle](/docs/fr/checkpointing) pour la façon dont les points de contrôle sont créés et restaurés.

<h3 id="unsupported-tool-content-removed">
  Contenu d'outil non supporté supprimé
</h3>

Quand Claude Code se connecte directement à l'API Anthropic et charge ou prévisualise une session enregistrée, il supprime le contenu d'outil que l'API Anthropic n'accepte pas et laisse cette ligne où le contenu supprimé s'asseyait entre deux blocs de réflexion :

```text theme={null}
[Unsupported tool content removed]
```

Un tel contenu atteint un fichier de session quand quelque chose d'autre que l'API Anthropic a répondu dans le format de l'API, généralement un proxy tiers défini via [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) qui traduit les appels d'outils d'un autre fournisseur. Claude Code le supprime uniquement quand la session se connecte directement à l'API Anthropic, et charge l'historique enregistré tel qu'il est quand la session s'exécute via un proxy ou sur un autre fournisseur. Avant v2.1.246, Claude Code renvoyait l'utilisation d'outil et son résultat à l'API, et chaque tour de la session reprise échouait avec une erreur 400 comme `messages.1.content.0.server_tool_use.name: Input should be 'web_search', 'web_fetch', ...`.

**Que faire :**

* Aucune action nécessaire quand vous voyez la ligne d'espace réservé. La session continue sans le contenu supprimé.
* Si chaque tour d'une session reprise échoue avec l'erreur 400 à la place, exécutez `claude update` et reprenez la session à nouveau. Les versions antérieures à v2.1.246 ne suppriment pas le contenu.

<h3 id="role-system-must-precede-an-assistant-message">
  role 'system' doit précéder un message 'assistant'
</h3>

L'API a refusé la requête avec un 400 parce qu'un message système se trouve à une position dans la conversation qu'elle n'accepte pas :

```text theme={null}
API Error: 400 messages.6: role 'system' must precede an 'assistant' message or end the array; ...
```

Claude Code envoie une partie de son texte de rappel et de pièce jointe comme des messages système à l'intérieur de la conversation. Quand l'API refuse la position d'un, Claude Code réessaie la requête une fois avec ce texte envoyé comme des messages utilisateur ordinaires à la place. Les libellés de placement frère de l'API, comme `use the top-level 'system' parameter for the initial system prompt`, obtiennent la même récupération.

Quand l'erreur apparaît, le message système refusé n'est pas un que Claude Code peut supprimer. Cela signifie généralement qu'un proxy ou une [passerelle LLM](/docs/fr/llm-gateway-protocol) entre Claude Code et l'API a ajouté un message système de son propre ou réordonné la conversation.

**Que faire :**

* Exécutez `/clear` pour démarrer une conversation nouvelle. Si l'erreur revient là aussi, la cause est sur le chemin de la requête, pas dans la conversation enregistrée.
* Si l'erreur se répète à chaque tour derrière un proxy ou une passerelle configurée via [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars), connectez-vous sans le proxy pour confirmer la source, et signalez l'erreur à celui qui l'exploite

Avant v2.1.280, Claude Code ne reconnaissait pas ce libellé, donc l'erreur apparaissait aussi quand le message système refusé était un que Claude Code lui-même envoyait, et chaque tour ultérieur de la conversation échouait de la même manière.

<h3 id="invalid-encrypted-content-in-search-result-block">
  Contenu chiffré invalide dans le bloc search\_result
</h3>

L'API a refusé la requête avec un 400 parce que l'historique de conversation contient du contenu de recherche web hébergé qu'elle ne peut pas déchiffrer. Le libellé nomme le champ qu'elle ne peut pas lire :

```text theme={null}
API Error: 400 messages.21.content.0: Invalid `encrypted_content` in `search_result` block
API Error: 400 messages.21.content.3.citations.0: Invalid `encrypted_index` in `text` block
API Error: 400 Failed to decrypt web search result content
```

Les résultats de l'[outil de recherche web hébergé](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool) de l'API portent des champs chiffrés que seule l'API peut lire. L'API refuse une requête qui rejoue du contenu qu'elle ne peut pas déchiffrer, comme du contenu produit pour une organisation différente.

L'[outil WebSearch](/docs/fr/tools-reference#websearch-tool-behavior) propre de Claude Code enregistre les résultats de recherche en texte brut, donc ces blocs atteignent généralement une conversation via un proxy ou une [passerelle LLM](/docs/fr/llm-gateway-protocol) qui a exécuté la recherche web hébergée elle-même.

Les blocs refusés restent dans l'historique de conversation, donc chaque tour ultérieur et `/compact` échouent de la même manière.

**Que faire :**

* Exécutez `/clear` ou démarrez une nouvelle session ; la nouvelle conversation ne porte pas les blocs refusés
* Si vous exécutez Claude Code derrière un proxy ou une passerelle, signalez l'erreur à celui qui l'exploite

<h3 id="usage-policy-refusal">
  Refus de la politique d'utilisation
</h3>

L'API a refusé de répondre parce que le contenu de la conversation a déclenché une vérification de la [Politique d'utilisation](https://www.anthropic.com/legal/aup). Le message inclut un ID de requête que vous pouvez citer au support si vous pensez que le refus est incorrect.

```text theme={null}
API Error: Opus 4.6 can't help with this. Start a new session to continue.

Send feedback with /feedback or learn more: https://www.anthropic.com/legal/aup
```

Le message nomme le modèle qui a refusé, ou `Claude` quand aucun modèle n'est enregistré.

La vérification évalue la conversation complète, pas seulement votre invite la plus récente, donc envoyer un nouveau message dans la même session réactive généralement le même refus. La même chose s'applique après la sortie et la réouverture de la session avec `--continue` ou `--resume`, puisque la transcription sur le disque contient toujours le contenu déclencheur. Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [la plateforme d'agent de Google Cloud](/docs/fr/google-vertex-ai) et [Microsoft Foundry](/docs/fr/microsoft-foundry), ce message couvre aussi les requêtes que les mesures de sécurité du modèle ont signalées comme un sujet de cybersécurité. Voir [Les mesures de sécurité ont signalé un sujet de cybersécurité](#safety-measures-flagged-a-cybersecurity-topic).

Avant v2.1.219, le message se lisait `Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.`

**Que faire :**

* Appuyez deux fois sur Échap ou exécutez `/rewind` pour revenir à un point de contrôle avant le tour qui a déclenché le refus, puis reformulez ou prenez une approche différente. Voir [Points de contrôle](/docs/fr/checkpointing).
* Si vous ne pouvez pas identifier quel tour l'a causé, exécutez `/clear` pour démarrer une conversation nouvelle dans le même projet. Votre conversation précédente est préservée sur le disque et reste disponible dans `/resume`.
* En [mode non-interactif](/docs/fr/headless) (`-p`), où la rembobinage est indisponible, réessayez avec une invite reformulée dans une nouvelle session sans `--continue`. Les vérifications de politique varient selon le modèle, donc basculer vers un modèle différent avec `--model` peut aussi résoudre le refus dans certains cas.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Les mesures de sécurité ont signalé un sujet de cybersécurité
</h3>

Les mesures de sécurité du modèle ont signalé le contenu de la conversation comme un sujet de cybersécurité. Le message nomme le modèle qui a signalé la requête :

```text theme={null}
API Error: Opus 4.8's safeguards flagged this message. Our intentionally broad safeguards allow us to deliver more capabilities faster, but can sometimes flag legitimate cybersecurity work. Apply to the Cyber Verification Program to reduce these interruptions. Send feedback with /feedback or learn more: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude
```

Le message se lie au [Programme de vérification de cybersécurité](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), qui accorde l'accès pour le travail de cybersécurité légitime. Sur Opus 5.5, qui nécessite v2.1.280 ou ultérieur, le message s'ouvre avec `Opus 5.5's safeguards flagged this session` à la place. Quand la catégorie signalée a un modèle de secours disponible, Claude Code [bascule les modèles](/docs/fr/model-config#automatic-model-fallback) plutôt que d'afficher cette erreur.

Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [la plateforme d'agent de Google Cloud](/docs/fr/google-vertex-ai) et [Microsoft Foundry](/docs/fr/microsoft-foundry), un drapeau de cybersécurité produit le message de [refus de la politique d'utilisation](#usage-policy-refusal) à la place.

La protection elle-même est côté serveur et antérieure à v2.1.203 ; les versions client depuis lors ont changé uniquement le libellé du message.
De v2.1.203 à v2.1.218, le message se lisait `<model> has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center:` suivi du même lien du centre d'aide, et les sessions interactives ajoutaient `If you were not engaging in a cybersecurity topic, please send feedback via /feedback.`
Avant v2.1.203, il se lisait `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` suivi d'un lien de formulaire d'exemption.

**Que faire :**

* Si votre travail nécessite ce contenu, postulez pour l'accès via le [Programme de vérification de cybersécurité](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Si votre requête n'était pas sur un sujet de cybersécurité, exécutez `/feedback` pour signaler le faux positif
* Pour continuer à travailler dans la même session, appuyez deux fois sur Échap ou exécutez `/rewind` pour revenir à un point de contrôle avant le tour qui a déclenché le drapeau, puis prenez une approche différente. Voir [Points de contrôle](/docs/fr/checkpointing).

<h2 id="installation-errors">
  Erreurs d'installation
</h2>

Ces erreurs apparaissent lors de l'installation ou de la mise à jour de Claude Code, à partir du [script d'installation](/docs/fr/setup#install-claude-code), `claude install`, ou `claude update`. Pour les problèmes de `command not found`, PATH, permission et TLS lors de la configuration, consultez [Dépannage de l'installation et de la connexion](/docs/fr/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  L'installation a été interrompue avant de pouvoir se terminer
</h3>

Le script d'installation signale quand l'étape `claude install` est terminée par un signal. Sur Linux, le code de sortie 137 signifie que le processus a reçu SIGKILL, et sur un hôte avec peu de mémoire, c'est généralement le tueur de mémoire insuffisante (OOM) du noyau. Le script affiche cette explication et se termine avec le code 137 :

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Pour tout autre signal fatal, et pour le code de sortie 137 sur macOS, le script affiche `Installation was killed before it could finish (exit code <N>)` avec le code de sortie réel et omet l'explication sur la mémoire insuffisante. Le message provient du script d'installation que macOS et Linux utilisent, qui couvre également les installations à l'intérieur de WSL ; les scripts d'installation Windows natifs ne l'affichent jamais. Avant la v2.1.200, le script se terminait avec seulement la ligne `Killed` brute du shell.

**Que faire :**

* Arrêtez les autres processus pour libérer de la mémoire, puis relancez le programme d'installation
* Ajoutez de l'espace d'échange ou passez à une instance plus grande. Consultez [Installation interrompue sur les serveurs Linux avec peu de mémoire](/docs/fr/troubleshoot-install#install-killed-on-low-memory-linux-servers) pour les commandes de fichier d'échange.

<h3 id="the-connection-dropped-while-downloading-the-update">
  La connexion s'est interrompue lors du téléchargement de la mise à jour
</h3>

La connexion au serveur de téléchargement s'est fermée pendant que `claude install`, `claude update`, ou le [programme de mise à jour automatique](/docs/fr/setup#auto-updates) téléchargeait le binaire Claude Code, et les tentatives de reconnexion n'ont pas fonctionné. Claude Code réessaie le téléchargement quand la connexion s'interrompt, le transfert s'arrête, ou le fichier téléchargé échoue sa somme de contrôle, jusqu'à trois tentatives au total. Une erreur HTTP complète, comme un 404, n'est pas réessayée car le serveur a déjà répondu. Avant la v2.1.202, une seule connexion interrompue échouait le téléchargement immédiatement avec l'erreur brute `aborted` au lieu de réessayer.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

Le texte entre parenthèses indique quelle tentative a échoué et l'erreur réseau sous-jacente. `claude update` précède le message avec `Error: Failed to install native update` sur stderr.

Un téléchargement qui reste connecté mais ne se termine pas dans les 10 minutes échoue avec `Download timed out: exceeded the total deadline` à la place. Claude Code ne réessaie pas un téléchargement qui a expiré, car une connexion trop lente pour se terminer dans le délai imparti ne se terminera pas lors d'une tentative immédiate non plus. Les étapes ci-dessous s'appliquent aux deux messages.

La cause habituelle est un proxy ou une passerelle qui ferme un long transfert avant qu'il ne se termine. Le binaire Claude Code est un gros téléchargement, donc une limite de connexion proxy qui n'affecte jamais le trafic API normal peut quand même l'interrompre.

**Que faire :**

* Exécutez `claude update` à nouveau. Sur un réseau par ailleurs sain, le téléchargement réussit généralement à la prochaine exécution. Pour le message d'expiration, exécutez-le à nouveau à partir d'un réseau plus rapide ou moins limité.
* Si votre réseau nécessite un proxy, définissez `HTTPS_PROXY` avant d'exécuter le programme d'installation ou `claude update`. Consultez [Vérifier la connectivité réseau](/docs/fr/troubleshoot-install#check-network-connectivity).
* Si un proxy d'entreprise continue de fermer le transfert, demandez à votre équipe réseau d'autoriser le téléchargement complet depuis `downloads.claude.ai`. Consultez [Exigences d'accès réseau](/docs/fr/network-config#network-access-requirements).
* Exécutez `claude doctor` à partir de votre shell pour les diagnostics d'installation

<h2 id="command-line-errors">
  Erreurs de ligne de commande
</h2>

Ces erreurs proviennent de la ligne de commande `claude` et de ses sous-commandes, d'un nom de commande que vous soumettez à l'invite, et de commandes telles que `/security-review` qui rassemblent le contexte en exécutant des commandes shell avant l'exécution de leur invite. Elles proviennent également de `/tui`, qui relance l'interface de ligne de commande.

<h3 id="conflict-between-bg-and-print">
  Conflit entre --bg et --print
</h3>

Ce message nécessite Claude Code v2.1.198 ou version ultérieure. Vous avez combiné `--bg` avec `-p` ou `--print` dans la même invocation `claude`. `--bg` démarre une [session en arrière-plan](/docs/fr/agent-view#from-your-shell) à laquelle vous vous connectez ultérieurement avec `claude agents`, tandis que `--print` s'exécute [de manière non interactive](/docs/fr/headless) et ne démarre jamais la session interactive à laquelle `claude agents` se connecte. Avant la v2.1.198, cette combinaison créait silencieusement une tâche en arrière-plan qui ne pouvait jamais être attachée.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**Que faire :**

* Supprimez `-p` ou `--print`. `--bg` prend l'invite comme argument positionnel, donc `claude --bg "<task>"` est la commande complète. Voir [Dispatcher de nouveaux agents depuis votre shell](/docs/fr/agent-view#from-your-shell).
* Pour exécuter l'invite de manière non interactive et imprimer le résultat au lieu de créer une session en arrière-plan, supprimez `--bg` et exécutez `claude -p "<task>"`

<h3 id="invalid-agents-configuration">
  Configuration --agents invalide
</h3>

La valeur que vous avez transmise à `--agents` est invalide, donc `claude` se termine avec le code 1 au lieu de démarrer la session. Lorsque vous transmettez `--safe-mode`, `--resume`, ou `--continue`, ou définissez [`CLAUDE_CODE_SAFE_MODE`](/docs/fr/env-vars#variables), Claude Code ne vérifie pas la valeur et démarre la session. Avant la v2.1.242, Claude Code démarrait la session de toute façon et omettait les définitions qu'il ne pouvait pas charger.

```text theme={null}
Error: Invalid --agents configuration:
<what failed>
```

Ce qui suit la première ligne dépend de la façon dont la valeur a échoué. Claude Code exécute ces vérifications dans l'ordre et s'arrête à la première qui échoue. Si votre valeur a deux types de problème, vous ne voyez le second qu'après avoir corrigé le premier :

1. Lorsque la valeur ne s'analyse pas en JSON, Claude Code imprime une ligne `invalid JSON:` portant le message du parseur JSON lui-même
2. Lorsqu'elle s'analyse mais qu'une définition d'agent ne correspond pas au schéma pour les [sous-agents définis par CLI](/docs/fr/sub-agents#choose-the-subagent-scope), Claude Code imprime une ligne par problème
3. Lorsqu'un nom d'agent commence par `-`, Claude Code imprime `<name>: agent names must not start with '-'`

Lorsqu'il y a plus de 20 lignes de problème, Claude Code imprime les 20 premières et remplace le reste par `…and N more`.

**Que faire :**

* Corrigez chaque problème que le message énumère, puis exécutez la commande à nouveau. Voir [les champs qu'un sous-agent défini par CLI prend](/docs/fr/sub-agents#choose-the-subagent-scope).

<h3 id="cloud-sessions-cannot-be-created-from-a-restricted-session">
  Les sessions cloud ne peuvent pas être créées à partir d'une session --restricted
</h3>

Lorsque vous démarrez une session avec [`--restricted`](/docs/fr/cli-reference#cli-flags), Claude Code refuse de créer des [sessions cloud](/docs/fr/claude-code-on-the-web#from-terminal-to-cloud) à partir de celle-ci, car la nouvelle session s'exécuterait en dehors du processus restreint et n'appliquerait pas le mode restreint. Claude Code refuse du côté client, avant de contacter le serveur, donc aucune session cloud n'est créée :

```text theme={null}
Cloud sessions cannot be created from a --restricted session: they would not enforce it.
```

**Que faire :**

* Exécutez la tâche localement dans la session restreinte
* Si vous contrôlez la façon dont la session a été lancée, démarrez une nouvelle session `claude` sans `--restricted` et créez la session cloud à partir de là

Avant la v2.1.248, Claude Code n'avait pas d'indicateur `--restricted` ; les versions antérieures rejettent l'indicateur lui-même avec une erreur d'option inconnue.

<h3 id="cloud-sessions-are-disabled-by-your-organizations-policy">
  Les sessions cloud sont désactivées par la politique de votre organisation
</h3>

La politique `allow_remote_sessions` de votre organisation est désactivée, donc les [sessions cloud](/docs/fr/claude-code-on-the-web) et les commandes qui les utilisent ne sont pas disponibles :

```text theme={null}
Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.
```

Le message apparaît lorsque vous [créez une session cloud à partir du terminal](/docs/fr/claude-code-on-the-web#from-terminal-to-cloud) et lorsque vous soumettez une commande qui a besoin de sessions cloud, telle que `/teleport`, `/remote-env`, ou `/web-setup`. Avant la v2.1.268, soumettre l'une de ces commandes renvoyait [`Unknown command`](#unknown-command) à la place.

Il s'agit d'une politique d'organisation côté serveur, elle ne peut donc pas être remplacée par des paramètres locaux, des variables d'environnement ou des indicateurs CLI.

Si Claude Code n'a pas encore chargé la politique de votre organisation ou ne peut pas la récupérer, ces commandes répondent `Couldn't verify your organization's policy for cloud sessions. Check your network connection, then restart Claude Code and try again.` à la place.

**Que faire :**

* Demandez à un [Propriétaire](/docs/fr/server-managed-settings#access-control) de votre organisation d'activer les sessions cloud dans les paramètres d'administration Claude Code à [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Si le message indique qu'il n'a pas pu vérifier la politique, vérifiez votre connexion réseau, puis redémarrez Claude Code et réessayez

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  La valeur --json-schema n'est pas un schéma JSON valide
</h3>

Le schéma que vous avez transmis à [`--json-schema`](/docs/fr/cli-reference#cli-flags) en [mode non interactif](/docs/fr/headless#get-structured-output) a échoué la compilation du schéma JSON, donc `claude` se termine avec le code 1 au lieu d'exécuter l'invite. Avant la v2.1.205, un schéma invalide produisait une sortie non structurée sans erreur, et tout schéma utilisant le mot-clé `format` était traité comme invalide.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

Le texte après le deuxième deux-points est le diagnostic du validateur et nomme le mot-clé ou l'emplacement qui a échoué. Les schémas qui utilisent le mot-clé `format`, tels que `"format": "email"`, sont valides : Claude Code accepte `format` comme annotation et ne l'applique pas.

Claude Code exécute deux vérifications avant la compilation du schéma : il rejette une valeur qui n'est pas analysable en JSON avec `Error: --json-schema is not valid JSON`, et un JSON valide qui n'est pas un objet avec `Error: --json-schema must be a JSON object`.

**Que faire :**

* Corrigez la partie du schéma que le diagnostic nomme, puis réexécutez la commande
* Si le diagnostic est `schema too large`, réduisez l'imbrication du schéma et la réutilisation de `$ref`
* Voir [Obtenir une sortie structurée](/docs/fr/headless#get-structured-output) pour un schéma et une commande fonctionnels

<h3 id="settings-file-exceeds-the-2mib-limit">
  Le fichier de paramètres dépasse la limite de 2 Mio
</h3>

Le fichier que vous avez transmis à [`--settings`](/docs/fr/cli-reference#cli-flags) est plus grand que 2 Mio, donc `claude` se termine avec le code 1 au démarrage au lieu de le charger. Un fichier de paramètres est un petit document JSON, donc un fichier de cette taille signifie généralement que le chemin pointe vers le mauvais fichier. Avant la v2.1.214, Claude Code lisait le fichier sans vérification de taille, et un fichier de plusieurs gigaoctets ou un fichier de périphérique tel que `/dev/zero` augmentait la mémoire sans limite.

```text theme={null}
Error: Settings file exceeds the 2MiB limit: /path/to/settings.json
```

Claude Code rejette un chemin `--settings` qui n'est pas un fichier régulier de la même manière : un périphérique, FIFO ou socket signale `Error: Cannot use settings file (Not a regular file (device, FIFO, or socket))` suivi du chemin, et un répertoire signale une raison `EISDIR`.

**Que faire :**

* Pointez `--settings` vers un fichier de paramètres JSON régulier inférieur à 2 Mio. Voir [Paramètres](/docs/fr/settings) pour le format.

<h3 id="the-current-directory-no-longer-exists">
  Le répertoire courant n'existe plus
</h3>

Vous avez démarré `claude` à partir d'un répertoire qui a été supprimé ou déplacé après que votre shell y soit entré, par exemple un worktree ou un répertoire temporaire qu'un autre shell a supprimé. Claude Code ne peut pas lire son répertoire de travail, donc il se termine avec le code 1 avant de démarrer la session, en mode interactif et [non interactif](/docs/fr/headless) également. Avant la v2.1.239, Claude Code s'écrasait avec une source de bundle minifiée et une pile `ENOENT ... uv_cwd` brute sur stderr au lieu de ce message.

```text theme={null}
The current directory no longer exists (it was deleted or moved). Start Claude Code from an existing directory.
error: The current working directory was deleted, so that command didn't work. Please cd into a different directory and try again.
```

La cause et la correction sont les mêmes pour les deux formes.

Lorsque Claude Code ne peut pas lire le répertoire de travail pour une autre raison, telle qu'un changement de permissions, le message nomme le code d'erreur à la place : `Can't read the current directory (EACCES). Start Claude Code from a different directory.`

Sur macOS, `EPERM` pour un répertoire dans `~/Desktop`, `~/Documents`, `~/Downloads`, ou iCloud Drive signifie généralement que macOS bloque votre application terminal de ce dossier. D'autres commandes qui lisent ce dossier échouent de la même manière : `ls` là-bas signale `Operation not permitted`, même avec `sudo`.

**Que faire :**

* Changez vers un répertoire qui existe, tel que votre répertoire personnel ou de projet, puis exécutez `claude` à nouveau
* Si le répertoire a été recréé au même chemin, votre shell tient toujours le répertoire supprimé. Exécutez `cd "$PWD"` ou quittez et réentrez le répertoire, puis exécutez `claude` à nouveau
* Pour `EPERM` sur macOS, quittez votre application terminal avec Cmd+Q, ouvrez-la à nouveau, retournez à ce dossier, et exécutez `claude`. Si `ls` dans ce dossier échoue toujours, ouvrez **Paramètres système > Confidentialité et sécurité > Fichiers et dossiers**, activez le dossier pour votre application terminal, puis rouvrez le terminal

<h3 id="temp-directory-refused-or-cannot-be-created">
  Le répertoire temporaire a été refusé ou ne peut pas être créé
</h3>

Sur macOS et Linux, Claude Code crée un répertoire temporaire privé au démarrage, `claude-<uid>` sous le répertoire temporaire du système ou le remplacement [`CLAUDE_CODE_TMPDIR`](/docs/fr/env-vars). Lorsque le répertoire ne peut pas être créé, ou qu'une entrée existant déjà à ce chemin échoue les vérifications de sécurité, Claude Code imprime l'échec sur stderr et se termine avec le code 1 plutôt que de démarrer la session :

```text wrap theme={null}
ENOSPC: no space left on device, mkdir '/tmp/claude-501'

Temp directory /tmp/claude-501 is not a directory (may be an attacker-planted symlink). Refusing to use it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is owned by uid 502, expected 501. Refusing to use it — another user may have pre-created it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is not readable (its mode may have been altered, or a path component denies search). Refusing to use it — restore its permissions (chmod 0700) or remove it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.
```

**Que faire :**

* Pour `ENOSPC`, libérez de l'espace disque sur le volume qui contient le répertoire temporaire
* Pour les formes `Refusing to use it`, supprimez l'entrée nommée elle-même, pas ce vers quoi un lien pointe, et démarrez Claude Code à nouveau ; pour la forme `owned by uid`, seul un administrateur ou cet utilisateur peut la supprimer
* Pour `is not readable`, exécutez `chmod 0700` sur le répertoire nommé, ou supprimez-le et redémarrez
* Dans l'un de ces cas, définissez [`CLAUDE_CODE_TMPDIR`](/docs/fr/env-vars) sur un répertoire que vous contrôlez et démarrez Claude Code à nouveau, en laissant le chemin refusé seul

<h3 id="directory-couldnt-be-resolved-to-a-real-location">
  Le répertoire n'a pas pu être résolu à un emplacement réel
</h3>

Vous avez exécuté `/add-dir` pour un sous-répertoire de votre répertoire de travail, et Claude Code n'a pas pu résoudre le répertoire à son emplacement réel.

Vous avez déjà accès aux fichiers d'un sous-répertoire du répertoire de travail, donc `/add-dir` charge uniquement ses skills, commandes et agents. Avant de les charger, Claude Code vérifie que l'emplacement réel du répertoire, avec tous les liens symboliques résolus, se trouve à l'intérieur du répertoire de travail. Lorsque Claude Code ne peut pas résoudre cet emplacement, il ne charge rien et affiche ce message :

```text theme={null}
packages/app couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded. Check that it is a directory inside the working directory and try again.
```

**Que faire :**

* Vérifiez que le chemin nomme un répertoire réel à l'intérieur du répertoire de travail, puis exécutez `/add-dir` à nouveau
* Le message ne change pas votre accès aux fichiers ; il signale uniquement que le contenu `.claude/` du répertoire n'a pas été chargé

Avant la v2.1.261, ce message apparaissait également pour chaque `/add-dir <subdirectory>` lorsque le répertoire de travail était sur un automontage `/net/<host>`, où Claude Code refuse de résoudre les chemins par conception ; le répertoire était correct et réessayer ne pouvait pas aider.

<h3 id="workspace-not-trusted-when-starting-remote-control">
  Espace de travail non approuvé au démarrage du contrôle à distance
</h3>

Vous avez démarré le mode serveur [Contrôle à distance](/docs/fr/remote-control) avec `claude remote-control` ou son alias `claude rc` dans un répertoire que vous n'avez pas approuvé. La commande n'affiche pas elle-même la boîte de dialogue d'approbation de l'espace de travail, elle se termine donc avec le code 1 et nomme la correction :

```text theme={null}
Error: Workspace not trusted. Please run `claude` in /Users/you/project first to review and accept the workspace trust dialog.
```

Dans votre répertoire personnel, le message est différent, car la boîte de dialogue d'approbation de l'espace de travail ne sauvegarde jamais l'approbation pour le répertoire personnel, donc l'accepter là-bas ne peut pas satisfaire cette vérification. Avant la v2.1.214, le répertoire personnel affichait le message ci-dessus, dont les conseils ne peuvent pas réussir là-bas.

```text theme={null}
Error: Workspace not trusted. /Users/you is your home directory, and for security home-directory trust is never saved, so running `claude` here first won't help. Run `claude rc` from a project directory instead (run `claude` there once to accept the trust dialog).
```

**Que faire :**

* Exécutez `claude` dans le répertoire, acceptez la [boîte de dialogue d'approbation de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust), puis exécutez `claude remote-control` à nouveau
* Dans votre répertoire personnel, changez vers un répertoire de projet et démarrez le contrôle à distance là-bas

<h3 id="not-carried-over-to-the-sessions-remote-control-starts">
  Non reporté aux sessions que le contrôle à distance démarre
</h3>

Vous avez démarré [Contrôle à distance](/docs/fr/remote-control) avec un indicateur global `claude` avant le verbe `remote-control`, un qui restreindrait ou configurerait les sessions que le contrôle à distance démarre, tel que `--settings`, `--setting-sources`, `--permission-mode`, `--disallowed-tools`, ou `--mcp-config`. Un indicateur placé avant le verbe n'atteint jamais ces sessions. Claude Code refuse de démarrer à la place, en nommant l'indicateur :

```text theme={null}
Error: `--settings` before `remote-control` is not carried over to the sessions Remote Control starts, so Remote Control refuses to start rather than drop it — remove it, and give Remote Control's own options after the verb (see `claude remote-control --help`).
```

Claude Code ne refuse pas les indicateurs globaux qui sont inoffensifs à supprimer, tels que `--verbose`, `--model`, ou un `--session-id` ou `--plugin-dir` injecté par wrapper : il les ignore et le contrôle à distance démarre.

Claude Code refuse également de démarrer pour un indicateur global qu'il ne reconnaît pas encore comme inoffensif, donc un indicateur ajouté dans une version plus récente peut apparaître dans ce message jusqu'à ce qu'une version ultérieure le marque comme inoffensif.

**Que faire :**

* Supprimez l'indicateur avant le verbe et transmettez [les options propres du contrôle à distance](/docs/fr/remote-control#start-a-remote-control-session) après ; `claude remote-control --help` les énumère
* Lorsque l'indicateur refusé est `--permission-mode`, exécutez `claude remote-control --permission-mode <mode>` pour définir le mode de permission pour les sessions que le contrôle à distance démarre

Avant la v2.1.248, `claude remote-control` n'acceptait pas ses propres indicateurs lorsqu'un indicateur global venait en premier, et la commande échouait avec une erreur d'option inconnue.

<h3 id="claude-import-is-not-yet-available-in-this-build">
  claude import n'est pas encore disponible dans cette version
</h3>

Vous avez exécuté [`claude import`](/docs/fr/cli-reference#cli-commands), et Claude Code a trouvé le flux d'importation désactivé, donc la commande se termine avec le code 1 au lieu de démarrer l'importation. Avant la v2.1.222, une version avec le flux d'importation désactivé traitait `import` comme une invite et démarrait une session interactive au lieu d'imprimer ce message.

```text theme={null}
`claude import` is not yet available in this build. Run `claude` and use /mcp or edit ~/.claude/settings.json directly.
```

Claude Code active `claude import` via un indicateur de fonctionnalité qu'il récupère auprès d'Anthropic et met en cache sur le disque. Ce message signifie que la valeur mise en cache est désactivée. La cause est généralement l'une des suivantes :

* Vous n'avez pas démarré de session depuis l'installation, donc Claude Code n'a pas encore récupéré l'indicateur. Le premier `claude import` peut imprimer ceci même lorsque la fonctionnalité vous est disponible.
* Vous utilisez Claude Code via Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou Claude Platform sur AWS, ou via une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway#availability-and-limitations). Claude Code ne récupère pas les indicateurs de fonctionnalité dans ces sessions, donc `claude import` reste indisponible.
* Vous avez défini `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `DISABLE_GROWTHBOOK`, ou [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars), qui désactivent la récupération des indicateurs de fonctionnalité, donc `claude import` reste indisponible.

**Que faire :**

* Sur une installation nouvelle, démarrez `claude`, attendez que la session se charge, quittez, et exécutez `claude import` à nouveau
* Où la récupération des indicateurs de fonctionnalité reste désactivée, configurez la configuration vous-même : ajoutez des serveurs MCP avec [`claude mcp add`](/docs/fr/mcp#installing-mcp-servers), et créez les [fichiers `CLAUDE.md`](/docs/fr/memory#how-claude-md-files-load), [skills et commandes](/docs/fr/skills#where-skills-live), et [sous-agents](/docs/fr/sub-agents#choose-the-subagent-scope) que vous souhaitez reporter. Le message nomme également `~/.claude/settings.json`. De la configuration que `claude import` reporte, ce fichier ne contient que le [mode de permission](/docs/fr/settings-reference#permission-settings) ; Claude Code ne lit pas les serveurs MCP à partir de celui-ci.

<h3 id="could-not-read-claude-code-config">
  Impossible de lire la configuration Claude Code
</h3>

Vous avez exécuté [`claude import`](/docs/fr/cli-reference#cli-commands) tandis que Claude Code ne pouvait pas analyser `~/.claude.json`, le fichier où il stocke votre connexion et l'état par projet. La sous-commande lit ce fichier pour vérifier la disponibilité mais n'affiche pas la boîte de dialogue de récupération que la session interactive affiche, elle se termine donc avec le code 1. Avant la v2.1.222, `claude import` avec un fichier de configuration illisible démarrait une session interactive, dont la boîte de dialogue de récupération gérait le fichier.

```text theme={null}
Could not read Claude Code config — run `claude` with no arguments to recover it.
```

**Que faire :**

* Exécutez `claude` sans arguments. Claude Code détecte le fichier invalide et propose de le réinitialiser. Puis exécutez `claude import` à nouveau.
* Pour conserver les modifications manuelles que vous avez apportées, corrigez la syntaxe JSON dans `~/.claude.json` dans un éditeur à la place, puis réexécutez `claude import`

<h3 id="could-not-import-a-server-from-claude-desktop">
  Impossible d'importer un serveur depuis Claude Desktop
</h3>

Claude Code n'a pas pu ajouter l'un des serveurs que vous avez sélectionnés dans `claude mcp add-from-claude-desktop`. La commande importe toujours les autres serveurs sélectionnés et imprime une ligne par serveur qu'elle n'a pas pu ajouter. Avant la v2.1.205, le premier serveur qui a échoué a arrêté l'importation et aucun des serveurs sélectionnés n'a été ajouté.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

Le texte après le nom du serveur est la raison. La plus courante est la vérification du nom : Claude Desktop autorise les caractères dans les noms de serveur, tels que les espaces et les points, que `claude mcp` restreint aux lettres, chiffres, traits d'union et traits de soulignement. D'autres raisons incluent une configuration de serveur qui échoue la validation et un serveur bloqué par la [politique MCP](/docs/fr/managed-mcp) de votre organisation.

**Que faire :**

* Renommez le serveur dans `claude_desktop_config.json` pour utiliser uniquement des lettres, des chiffres, des traits d'union et des traits de soulignement, puis exécutez `claude mcp add-from-claude-desktop` à nouveau
* Ajoutez ce serveur directement avec `claude mcp add` ou `claude mcp add-json` sous un nom valide. Voir [Importer les serveurs MCP depuis Claude Desktop](/docs/fr/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="cannot-add-mcp-server-to-the-managed-scope">
  Impossible d'ajouter un serveur MCP à la portée gérée
</h3>

Vous avez exécuté `claude mcp add` ou `claude mcp add-json` avec `--scope managed`. Cette portée contient les serveurs que votre organisation fournit via le paramètre géré [`managedMcpServers`](/docs/fr/settings-reference#managedmcpservers). Claude Code les lit à partir des paramètres gérés uniquement, donc la commande ne peut pas écrire un serveur dans cette portée.

```text theme={null}
Cannot add MCP server to scope: managed
```

**Que faire :**

* Ajoutez le serveur à une portée dans laquelle vous pouvez écrire : `local`, `user`, ou `project`. Sans `--scope`, la commande utilise `local`. Voir [Portées d'installation MCP](/docs/fr/mcp#mcp-installation-scopes)
* Pour fournir le serveur à chaque utilisateur de votre organisation, ajoutez-le à [`managedMcpServers`](/docs/fr/settings-reference#managedmcpservers) dans les paramètres gérés que vous déployez

<h3 id="cant-read-mcp-json">
  Impossible de lire .mcp.json
</h3>

Une commande qui lit le [`.mcp.json`](/docs/fr/mcp#project-scope) du projet, telle que `claude mcp add` ou `claude mcp add-json` avec `--scope project`, ou `claude mcp remove`, a trouvé que le fichier dans votre répertoire courant n'est pas un fichier régulier ou est plus grand que 2 Mio, elle se termine donc avec cette erreur au lieu de lire le fichier.

```text theme={null}
Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes. Fix or remove it, then run the command again.
```

Avant la v2.1.257, un FIFO à `.mcp.json` laissait la commande attendre indéfiniment sans sortie, et un lien symbolique vers un fichier de périphérique tel que `/dev/zero` augmentait la mémoire jusqu'à ce que le processus soit tué.

**Que faire :**

* Vérifiez ce qui se trouve à `.mcp.json` dans votre répertoire courant. Remplacez-le par un fichier JSON ordinaire au [format de portée de projet](/docs/fr/mcp#project-scope), ou supprimez-le, puis exécutez la commande à nouveau.

<h3 id="anthropic-hosted-and-doesnt-support-local-oauth">
  Le serveur est hébergé par Anthropic et ne supporte pas OAuth local
</h3>

Vous avez démarré une connexion pour un serveur MCP dont l'URL pointe vers un hôte de connecteur hébergé par Anthropic qui s'authentifie via un fournisseur d'identité tiers. Ces hôtes incluent `microsoft365.mcp.claude.com`, `gmail.mcp.claude.com`, et `gcal.mcp.claude.com`. Claude Code refuse de démarrer son flux OAuth local pour ces hôtes à partir du panneau `/mcp` et de `claude mcp login`, car [leur connexion fonctionne uniquement via claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai).

```text theme={null}
"gmail" is Anthropic-hosted and doesn't support local OAuth. Connect it via Settings → Connectors on claude.ai (requires `claude login`), then it'll be available here automatically.
```

Claude Code correspond à ces hôtes par URL, donc le message apparaît lorsqu'un serveur que vous avez ajouté avec `claude mcp add` ou dans `.mcp.json` pointe vers l'un d'eux.

**Que faire :**

* Supprimez votre entrée avec `claude mcp remove <name>`, afin qu'elle ne puisse pas masquer le connecteur claude.ai à la même URL
* Après l'avoir supprimée, connectez le service à [claude.ai/customize/connectors](https://claude.ai/customize/connectors), tout en étant connecté au compte que vous utilisez dans Claude Code. Une fois connecté, [le connecteur apparaît dans Claude Code automatiquement](/docs/fr/mcp#use-mcp-servers-from-claude-ai) si votre méthode d'authentification active est une connexion d'abonnement claude.ai

<h3 id="server-rejected-the-authorization-header-minted-by-the-configured-headershelper">
  Le serveur a rejeté l'en-tête Authorization créé par le headersHelper configuré
</h3>

Un serveur MCP dont le [`headersHelper`](/docs/fr/mcp#use-dynamic-headers-for-custom-authentication) fournit l'en-tête `Authorization` a répondu à la connexion avec HTTP 401 ou 403, donc Claude Code signale la connexion comme échouée. Parce que le helper fournit l'en-tête `Authorization`, Claude Code [ne revient pas à OAuth](/docs/fr/mcp#authenticate-with-remote-mcp-servers) pour le serveur :

```text theme={null}
Server rejected the Authorization header minted by the configured headersHelper (HTTP 401). Check that the helper command returns a valid credential for this MCP endpoint — OAuth fallback is disabled when the helper supplies Authorization.
```

Claude Code réexécute le helper à chaque tentative de connexion, donc une nouvelle tentative après un rejet transitoire, tel qu'une course de rotation de jeton, peut réussir avec une nouvelle credential.

**Que faire :**

* Exécutez la commande `headersHelper` vous-même de la façon que Claude Code l'exécute : à partir du [répertoire où Claude Code l'exécute](/docs/fr/mcp#where-the-helper-runs), avec les [variables d'environnement que Claude Code définit pour elle](/docs/fr/mcp#use-dynamic-headers-for-custom-authentication), et sans les [variables de credential que Claude Code supprime](/docs/fr/mcp#which-variables-a-helper-can-read) pour un serveur à partir d'un `.mcp.json` de projet, d'un plugin, ou d'un fichier d'agent de projet. Vérifiez qu'elle imprime une valeur `Authorization` que le point de terminaison du serveur accepte
* Après avoir corrigé le helper ou sa source de credential, sélectionnez le serveur dans `/mcp` et choisissez **Reconnect**

Avant la v2.1.248, Claude Code exécutait la découverte OAuth pour un serveur dont le helper fournissait l'en-tête `Authorization`. Cette découverte pouvait échouer avec `Incompatible auth server: does not support dynamic client registration` au lieu de signaler la credential rejetée.

<h3 id="mcp-permission-prompt-tool-not-found">
  Outil d'invite de permission MCP non trouvé
</h3>

L'outil que vous avez transmis à [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags) ne figurait pas parmi les outils MCP connectés lorsque l'exécution a d'abord eu besoin d'une décision de permission, soit parce que son serveur ne s'est jamais connecté, soit parce qu'aucun serveur connecté n'expose un outil portant ce nom. Claude Code envoie toujours votre invite : l'exécution [non interactive](/docs/fr/headless) se termine avec cette erreur, et le code de sortie 1, au premier appel d'outil qui a besoin d'approbation, donc elle ne produit aucune réponse même si la demande a été faite. Avant la première invite, Claude Code attend jusqu'au délai d'expiration de la connexion par serveur de 30 secondes défini par [`MCP_TIMEOUT`](/docs/fr/env-vars) pour que ce serveur se connecte. Avant la v2.1.206, le démarrage n'attendait pas que le serveur finisse de se connecter, donc un serveur qui démarre lentement mais sain produisait également cette erreur.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

La liste après `Available MCP tools:` nomme les outils MCP qui étaient connectés lorsque l'attente s'est terminée.

**Que faire :**

* Vérifiez que le serveur démarre et reste connecté : exécutez `claude mcp list` dans le même répertoire et confirmez que le serveur est listé comme connecté
* Confirmez que le nom de l'outil correspond au nom `mcp__<server>__<tool>` que le serveur expose
* Si le serveur a besoin de plus de 30 secondes pour démarrer, augmentez [`MCP_TIMEOUT`](/docs/fr/env-vars)

<h3 id="oauth-callback-port-is-already-in-use">
  Le port de rappel OAuth est déjà en cours d'utilisation
</h3>

Lorsque vous vous connectez à un serveur MCP distant avec OAuth, Claude Code démarre un écouteur local pour recevoir le rappel de connexion. Si le port dont cet écouteur a besoin est détenu par un autre processus, la connexion échoue avec ce message. Cela se produit principalement avec un [port de rappel fixe](/docs/fr/mcp#use-a-fixed-oauth-callback-port) défini via la variable [`MCP_OAUTH_CALLBACK_PORT`](/docs/fr/env-vars) ou `--callback-port`, car sans celui-ci Claude Code choisit un port disponible.

```text theme={null}
OAuth callback port <port> is already in use — another process may be holding it. Run `lsof -ti:<port> -sTCP:LISTEN` to find it.
```

Sur Windows, la commande suggérée est `netstat -ano | findstr :<port>` à la place.

**Que faire :**

* Exécutez la commande du message pour trouver le processus qui détient le port, et arrêtez-le ou attendez qu'il se termine
* Si un autre programme a besoin de ce port de manière permanente, enregistrez un URI de redirection différent auprès du serveur et définissez son port avec `MCP_OAUTH_CALLBACK_PORT` ou `--callback-port`, selon celui que vous utilisez
* Puis démarrez la connexion à nouveau, par exemple en sélectionnant le serveur dans `/mcp`

<h3 id="no-available-ports-for-oauth-redirect">
  Aucun port disponible pour la redirection OAuth
</h3>

Lorsque vous vous connectez à un serveur MCP distant avec [OAuth](/docs/fr/mcp#authenticate-with-remote-mcp-servers), Claude Code démarre un écouteur local pour recevoir le rappel de connexion. La connexion échoue avec ce message lorsque Claude Code ne peut pas lier un port local pour cela. Quelque chose sur la machine empêche d'écouter sur `127.0.0.1`, par exemple un logiciel de sécurité ou une politique de sandbox qui refuse les écouteurs locaux.

```text theme={null}
No available ports for OAuth redirect
```

Avant la v2.1.268, Claude Code ne revenait pas à un port assigné par le système d'exploitation, donc le message apparaissait également lorsque seuls ses ports auto-sélectionnés ne pouvaient pas être liés. Cela peut se produire sur les hôtes Windows où Hyper-V réserve des plages de ports qui couvrent les ports que Claude Code choisit.

**Que faire :**

* Vérifiez si un logiciel de sécurité ou une politique de sandbox bloque les processus d'écoute sur `127.0.0.1`, et autorisez Claude Code à lier un port local
* Puis démarrez la connexion à nouveau, par exemple en sélectionnant le serveur dans `/mcp`

<h3 id="security-review-fails-without-origin-head">
  /security-review échoue sans origin/HEAD
</h3>

[`/security-review`](/docs/fr/commands#all-commands) construit son contexte d'examen en comparant votre branche avec `origin/HEAD`, la référence locale qui enregistre quelle branche est la branche par défaut sur votre télécommande `origin`. Lorsque cette référence n'existe pas, les commandes git qui rassemblent la comparaison échouent et l'examen s'arrête avant de commencer.

```text theme={null}
Error: Shell command failed for pattern "!`git diff --name-only origin/HEAD...`": [stderr]
fatal: ambiguous argument 'origin/HEAD...': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
```

Le message peut citer `git log` ou une `git diff` différente à la place. Git crée `origin/HEAD` uniquement lorsque la télécommande annonce une branche par défaut et que votre refspec de récupération la couvre, ce qu'un `git clone` complet d'une télécommande avec des commits fait. La référence manque dans ces configurations :

* Un checkout à branche unique ou CI, qui récupère un refspec trop étroit
* Une télécommande dont le HEAD côté serveur pointe vers une branche que personne n'a poussée
* Un référentiel sans télécommande `origin`, ou une que vous n'avez jamais récupérée

Claude Code affiche la même erreur pour tout skill qui [injecte du contexte dynamique](/docs/fr/skills#when-an-injected-command-fails), et une commande injectée échouée abandonne l'invocation de ce skill. Deux chaînes sœurs se déclenchent avant l'exécution de la commande :

* `Shell command permission check failed for pattern "..."`: la vérification de permission de la commande ne l'a pas autorisée. [Les vérifications de permission sur les commandes injectées](/docs/fr/skills#permission-checks-on-injected-commands) couvrent quels résultats abandonnent dans chaque mode de permission et comment pré-approuver une commande avec `allowed-tools`
* ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``: le frontmatter du skill exige bash sur une machine sans celui-ci. Installez Git pour Windows ou changez le frontmatter en `shell: powershell`. Voir [Comment les commandes injectées s'exécutent](/docs/fr/skills#how-injected-commands-run)

**Que faire :**

* Créez la référence en nommant la branche par défaut de votre télécommande : `git remote set-head origin <default-branch>`. Cela fonctionne chaque fois que la référence de suivi locale `origin/<default-branch>` existe. Si ce n'est pas le cas, comme dans les clones à branche unique, récupérez d'abord la branche : exécutez `git remote set-branches --add origin <branch>`, puis `git fetch origin`, puis réexécutez la commande set-head. Réexécutez `/security-review`.
* Si vous préférez ne pas nommer la branche, exécutez `git fetch origin` puis `git remote set-head origin --auto`, qui demande à la télécommande quelle branche est sa branche par défaut. Elle échoue avec `error: Cannot determine remote HEAD` lorsque la télécommande n'annonce aucune branche par défaut, car elle est vide ou son HEAD pointe vers une branche que personne n'a poussée ; nommez la branche explicitement à la place. Elle échoue avec `error: Not a valid ref` lorsque votre clone ne récupère pas cette branche ; élargissez le refspec comme ci-dessus d'abord.
* Si le référentiel n'a pas de télécommande, ajoutez-en une avec `git remote add origin <url>` et récupérez avant de créer la référence. Si la télécommande est vide, poussez votre branche d'abord avec `git push -u origin HEAD` et nommez cette branche dans la commande set-head ; `origin/HEAD` pointe alors vers la branche que vous venez de pousser, donc `/security-review` voit une comparaison vide jusqu'à ce que la branche diverge de celle-ci.

<h3 id="input-must-be-provided-when-using-print">
  L'entrée doit être fournie lors de l'utilisation de --print
</h3>

Le `claude` nu a besoin que stdout soit un terminal pour démarrer l'interface utilisateur interactive. Lorsque stdout est redirigé, ou que la console n'est pas un vrai terminal, tel que PowerShell ISE et certains volets de sortie IDE, `claude` s'exécute [de manière non interactive](/docs/fr/headless) à la place. C'est le même mode que `claude -p`, qui nécessite une invite, donc le message nomme `--print` même si vous n'avez pas transmis l'indicateur. Transmettre `-p`/`--print` sans invite et rien canalisé sur stdin produit la même erreur n'importe où.

```text theme={null}
Error: Input must be provided either through stdin or as a prompt argument when using --print
```

**Que faire :**

* Pour une utilisation interactive, exécutez `claude` dans un vrai terminal : Windows Terminal ou la console PowerShell plutôt que ISE, et le terminal intégré de votre IDE plutôt qu'un volet de sortie
* Pour une utilisation ponctuelle, transmettez l'invite : `claude -p "your question"`, ou canalisez-la avec `echo "your question" | claude -p`

<h3 id="input-contained-only-whitespace">
  L'entrée contenait uniquement des espaces blancs
</h3>

En [mode non interactif](/docs/fr/headless), Claude Code refuse une invite composée entièrement d'espaces, de tabulations ou de sauts de ligne au lieu de l'envoyer, car l'API rejette les messages sans texte visible. Le message que vous voyez dépend de l'endroit d'où provient l'invite vide :

* **Argument d'invite ou stdin canalisé pour `claude -p`** : `claude` se termine avec `Error: Input contained only whitespace. Provide a prompt with text through stdin or as a prompt argument when using --print`
* **Message soumis à une session `--input-format stream-json` ou [Agent SDK](/docs/fr/agent-sdk/overview) en cours d'exécution** : Claude Code termine le tour sans appeler le modèle et la session reste utilisable. Le refus arrive comme un message informatif et comme le texte de résultat du tour : `Blank prompt — the message was only whitespace, so nothing was sent to the model.`

Avant la v2.1.229, Claude Code envoyait le message contenant uniquement des espaces blancs à l'API, qui rejetait la demande avec une erreur 400.

**Que faire :**

* Incluez du texte visible dans l'invite. Si un script construit l'invite à partir d'une variable ou d'un fichier, vérifiez que la source n'est pas vide avant d'appeler Claude Code.

<h3 id="stream-json-input-carried-over-256m-characters-with-no-newline">
  L'entrée stream-json a porté plus de 256 M caractères sans nouvelle ligne
</h3>

Votre programme a envoyé plus de 268 435 456 caractères sur stdin sans nouvelle ligne à une exécution `claude -p --input-format stream-json`, donc Claude Code imprime cette erreur sur stderr et se termine avec le code 1 au lieu de mettre en mémoire tampon plus d'entrée. Le message énonce ce budget comme `256M`. Avant la v2.1.257, Claude Code mettait en mémoire tampon une telle entrée sans limite, augmentant la mémoire jusqu'à ce que le processus s'écrase ou soit tué.

```text theme={null}
Error: stream-json input carried over 256M characters with no newline. Each stream-json message must be a single newline-terminated JSON line: either the producer is not newline-terminating its messages, or one message exceeded this budget.
```

Une entrée aussi longue sans nouvelle ligne signifie généralement que le producteur n'est pas du tout un producteur stream-json, tel qu'un fichier binaire ou une sortie de journal ordinaire canalisée par accident. Un seul message dépassant le budget échoue la même vérification.

**Que faire :**

* Vérifiez ce qui est canalisé sur stdin. Avec [`--input-format stream-json`](/docs/fr/cli-reference#cli-flags), chaque message doit être une ligne JSON terminée par une nouvelle ligne
* Pour envoyer du texte ordinaire à la place, supprimez `--input-format stream-json` ; `claude -p` lit une invite en texte ordinaire à partir de stdin par défaut

<h3 id="unknown-command">
  Commande inconnue
</h3>

Dans une session de terminal interactive, vous avez soumis un nom `/` qui ne correspond à aucune commande dans cette session, donc Claude Code signale le nom au lieu d'exécuter quoi que ce soit :

```text theme={null}
Unknown command: /hepl. Did you mean /help?
```

Claude Code suggère le nom de commande ou l'alias le plus proche que le menu énumère dans cette session. Lorsque rien n'est proche, le message se termine après le nom. La cause est généralement l'une des suivantes :

* Une faute de frappe, telle que `/hepl` pour `/help`. [Comment le menu de commande correspond à ce que vous tapez](/docs/fr/commands#how-the-command-menu-matches-what-you-type) couvre le choix d'une correspondance proche avant de soumettre
* Une commande qui existe mais n'est pas disponible dans cette session car une exigence n'est pas satisfaite, telle que votre plateforme, plan ou méthode d'authentification. Les entrées de dépannage pour [`/web-setup`](/docs/fr/web-quickstart#web-setup-shows-no-commands-match-or-unknown-command) et [`/schedule`](/docs/fr/routines#schedule-returns-unknown-command) parcourent deux cas courants. Certaines commandes répondent avec leur propre message lorsque la politique de votre organisation les désactive, telles que [`Cloud sessions are disabled by your organization's policy`](#cloud-sessions-are-disabled-by-your-organizations-policy)
* Une commande d'un [plugin](/docs/fr/plugins/overview) ou [serveur MCP](/docs/fr/mcp#use-mcp-prompts-as-commands) qui n'est pas installé ou connecté dans cette session

Claude Code répond à un nom `/` non appairé de cette manière uniquement dans une session de terminal interactive. Dans toute autre session, il envoie l'invite à Claude comme un message normal à la place, avec une note que la commande n'a pas s'exécutée et une liste de commandes que Claude peut exécuter dans la session. Ces sessions incluent :

* Les exécutions `-p`
* Les applications [Agent SDK](/docs/fr/agent-sdk/overview)
* L'onglet Code de l'[application Desktop](/docs/fr/desktop)
* Le panneau de chat de l'[extension VS Code](/docs/fr/vs-code)
* Les [sessions cloud](/docs/fr/claude-code-on-the-web) et [routines](/docs/fr/routines)

Pour une commande intégrée qui ne peut pas s'exécuter dans l'une de ces sessions, Claude Code répond toujours que la commande n'est pas disponible au lieu de l'envoyer à Claude. Avant la v2.1.274, seules les sessions cloud et les routines envoyaient un nom non appairé à Claude. Avant la v2.1.273, elles répondaient également `Unknown command`.

Claude Code ne traite pas chaque invite qui commence par `/` comme une commande. Il envoie l'invite à Claude comme un message normal lorsque le premier mot après le `/` commence par la ponctuation, telle que le `/--` qui ouvre un commentaire de document Lean, ou est un chemin tel que `/var/log/syslog`.

Avant la v2.1.236, si vous aviez appuyé sur `Entrée` tandis que le menu de commande énumérait une correspondance proche du nom que vous aviez tapé, Claude Code exécutait cette correspondance, donc une faute de frappe telle que `/hepl` exécutait `/help` au lieu de produire ce message.

**Que faire :**

* Exécutez le nom suggéré, ou tapez `/` suivi d'une partie du nom pour voir ce qui est disponible dans cette session
* Si Claude Code signale une commande documentée comme inconnue, vérifiez sa ligne dans la [référence des commandes](/docs/fr/commands) pour l'exigence qu'elle nomme

<h3 id="diff-is-too-large-for-ultrareview">
  La comparaison est trop grande pour ultrareview
</h3>

La comparaison entre votre branche et la branche de base, y compris les modifications non validées et mises en scène, dépasse les limites de taille pour un [ultrareview](/docs/fr/ultrareview), donc `/code-review ultra` et la sous-commande `claude ultrareview` refusent l'examen avant le démarrage de la session cloud. Un examen refusé n'utilise pas une exécution gratuite et ne facture pas les crédits d'utilisation. Le message nomme les limites en vigueur, la taille de votre comparaison et les fichiers qui contribuent le plus de lignes modifiées. Avant la v2.1.216, le message affichait uniquement les statistiques de comparaison brutes.

```text theme={null}
Diff is too large for ultrareview: 812 files, 96,410 lines changed (limits: 500 files, 8,000 lines). Largest files: package-lock.json (41,904 lines), dist/bundle.js (18,210 lines), src/generated/api.ts (9,876 lines). Pass a closer base branch (`/code-review ultra <branch>`) to narrow the scope, or split the change.
```

L'examen d'une demande de tirage applique les mêmes limites ; cette forme du message commence par `PR #<N> is too large for ultrareview` et nomme les comptes de fichiers et de lignes de la demande de tirage.

**Que faire :**

* Transmettez une branche de base plus proche de votre travail, telle que `/code-review ultra develop`, afin que l'examen couvre uniquement la comparaison par rapport à cette branche
* Divisez la modification en branches plus petites et examinez chacune. Les fichiers que le message nomme contribuent le plus de lignes modifiées, donc commencez par déplacer ceux-ci vers leur propre branche.

<h3 id="could-not-find-merge-base-with-the-base-branch">
  Impossible de trouver la base de fusion avec la branche de base
</h3>

`/code-review ultra` et la sous-commande `claude ultrareview` examinent la comparaison entre votre branche et une branche de base, ce qui nécessite un commit que les deux partagent. Lorsque `git merge-base` n'en trouve aucun, Claude Code refuse l'examen avant le démarrage de la session cloud. Sur un clone que Claude Code peut vérifier comme complet, avec au moins une branche, il revient à [examiner chaque fichier suivi](/docs/fr/ultrareview#diff-limits-and-fallbacks) au lieu de refuser. Vous voyez ce refus lorsque la branche de base ne peut pas être trouvée du tout, lorsque Claude Code ne peut pas vérifier que votre clone est complet, ou dans le rare référentiel où la comparaison de l'arborescence entière n'est pas possible, telle que le format d'objet SHA-256.

```text theme={null}
Could not find merge-base with main. Pass the base branch explicitly (e.g. `/code-review ultra develop`) or make sure you're in a git repo with a main branch.
```

L'indice après la première phrase dépend de ce que Claude Code a observé :

* **Vous n'avez pas transmis une branche de base** : Claude Code a comparé par rapport à la branche par défaut du référentiel et suggère de transmettre votre base explicitement, comme dans l'exemple ci-dessus
* **Vous avez transmis une branche de base qui était déjà dans votre clone** : l'indice lit ``Make sure <branch> exists locally or on origin (try `git fetch origin <branch>`)``
* **Vous avez transmis une branche de base qui n'était pas dans votre clone** : Claude Code l'a récupérée à partir de origin avant de comparer. L'indice lit ``<branch> was fetched from origin but shares no history with HEAD. If another branch is your real base, pass it explicitly (`/code-review ultra <branch>`)``; lorsque Claude Code ne peut pas dire si votre clone est superficiel, il suggère `git fetch --unshallow origin` à la place. Avant la v2.1.221, l'indice suggérait `git fetch --unshallow origin` pour chaque branche de base récupérée, et sur un clone complet cette commande échoue avec `fatal: --unshallow on a complete repository does not make sense`.

**Que faire :**

* Si une autre branche est votre vraie base, transmettez-la explicitement : `/code-review ultra <branch>`
* Si votre clone pourrait ne pas avoir l'historique complet, exécutez `git fetch --unshallow origin` et réexécutez l'examen

<h3 id="your-checkout-has-no-branches">
  Votre checkout n'a pas de branches
</h3>

Un checkout peut avoir des commits mais pas de branches : si vous exécutez `git init` suivi de `git fetch <url>` et `git checkout FETCH_HEAD`, vous obtenez un HEAD détaché sans références. Claude Code empaquette votre référentiel en tant que bundle git pour le télécharger pour un [ultrareview](/docs/fr/ultrareview), et il ne peut pas empaqueter un référentiel qui n'a pas de branches ou d'autres références, donc `/code-review ultra` et la sous-commande `claude ultrareview` refusent l'examen avant le démarrage de la session cloud.

```text theme={null}
Your checkout has no branches (detached HEAD only), which cloud review can't bundle. Create one first — `git checkout -b <name>` — then rerun /code-review ultra.
```

Avant la v2.1.221, Claude Code tentait d'examiner chaque fichier suivi dans ce checkout, et le téléchargement échouait.

**Que faire :**

* Créez une branche à votre commit courant avec `git checkout -b <name>`, puis réexécutez l'examen

<h3 id="no-github-account-is-connected-to-your-claude-account">
  Aucun compte GitHub n'est connecté à votre compte Claude
</h3>

Vous avez exécuté `/code-review ultra <PR#>` ou `claude ultrareview <PR#>`, et avant de créer la session cloud Claude Code demande au serveur si [le compte GitHub connecté à votre compte Claude](/docs/fr/ultrareview#review-a-pull-request) peut atteindre le référentiel de la demande de tirage. Aucun compte n'est connecté, ou la connexion a expiré, donc le clone cloud échouerait et Claude Code refuse le lancement. Claude Code ne dépense pas une exécution gratuite ou ne facture pas les crédits d'utilisation pour un lancement refusé.

```text theme={null}
Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected (or the connection expired). To fix: run /web-setup to reuse your GitHub CLI login, or connect an account at https://claude.ai/connect-github — then re-run /code-review ultra 1234 (allow a minute after connecting).
```

Lorsque [`/web-setup`](/docs/fr/web-quickstart#connect-from-your-terminal) n'est pas disponible dans votre session, le message nomme uniquement le lien claude.ai.

**Que faire :**

* Exécutez `/web-setup` pour connecter votre connexion GitHub CLI à votre compte Claude, ou connectez un compte à [claude.ai/connect-github](https://claude.ai/connect-github)
* Réexécutez l'examen une minute après la connexion

Avant la v2.1.248, Claude Code ne vérifiait pas cela avant le lancement.

<h3 id="your-connected-github-account-cant-see-the-repository">
  Votre compte GitHub connecté ne peut pas voir le référentiel
</h3>

Vous avez exécuté `/code-review ultra <PR#>` ou `claude ultrareview <PR#>`, et [le compte GitHub connecté à votre compte Claude](/docs/fr/ultrareview#review-a-pull-request) ne peut pas lire le référentiel de la demande de tirage, donc le clone cloud échouerait et Claude Code refuse le lancement. Claude Code ne dépense pas une exécution gratuite ou ne facture pas les crédits d'utilisation pour un lancement refusé.

```text theme={null}
Your connected GitHub account can't see <owner>/<repo> — usually the Claude GitHub app isn't installed on <owner> or wasn't granted this repo (web-connected accounts need it for private repos), or a different GitHub account is connected. To fix: run /web-setup to reuse your GitHub CLI login, or install the app at https://github.com/apps/claude/installations/new — then re-run /code-review ultra 1234.
```

Lorsque [`/web-setup`](/docs/fr/web-quickstart#connect-from-your-terminal) n'est pas disponible dans votre session, le message nomme uniquement l'installation de l'application.

**Que faire :**

* Si votre CLI `gh` local peut lire le référentiel, exécutez `/web-setup` pour connecter cette connexion à votre compte Claude
* Réexécutez l'examen après la modification

Avant la v2.1.248, Claude Code ne vérifiait pas cela avant le lancement.

<h3 id="the-github-app-preflight-failed-transiently">
  La vérification préalable de l'application GitHub a échoué de manière transitoire
</h3>

Vous avez démarré une [session cloud](/docs/fr/claude-code-on-the-web) à partir d'un référentiel local, et deux étapes ont échoué ensemble. Claude Code n'a pas pu construire ou télécharger le bundle de votre référentiel. Avant le téléchargement, il a vérifié si le service cloud peut cloner le référentiel à partir de GitHub, et plutôt qu'une réponse définitive, cette vérification s'est terminée par une erreur qu'une nouvelle tentative pourrait clarifier, telle qu'une erreur réseau, un délai d'expiration ou une erreur serveur temporaire. Le message complet commence par ce qui a arrêté le bundle, par exemple `Could not upload repo bundle (<error>)`, et se termine par la phrase de vérification préalable :

```text theme={null}
Could not upload repo bundle (<error>). The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead
```

**Que faire :**

* Réexécutez la commande après un moment. Lorsque la vérification GitHub réussit, Claude Code peut démarrer la session à partir d'un clone GitHub, donc le téléchargement échoué ne bloque plus le lancement
* Si les nouvelles tentatives continuent d'échouer, le début du message nomme ce qui a arrêté le téléchargement. Lorsque cette cause est quelque chose que vous pouvez corriger, corrigez-la afin que la session puisse démarrer à partir de votre référentiel local à la place

Avant la v2.1.251, Claude Code terminait le message avec `Please set up GitHub on https://claude.ai/code` même lorsque la vérification GitHub a échoué uniquement de manière transitoire, et les conseils de configuration ne peuvent pas clarifier un échec transitoire.

<h3 id="github-isnt-connected-to-your-claude-account">
  GitHub n'est pas connecté à votre compte Claude
</h3>

Vous avez démarré une [session cloud](/docs/fr/claude-code-on-the-web) à partir de votre référentiel local, par exemple avec `/autofix-pr`. Aucun compte GitHub n'est connecté à votre compte Claude, ou la connexion a expiré, donc Claude Code refuse le lancement :

```text theme={null}
GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud. Run /web-setup to connect with your GitHub CLI login, or connect on the web at https://claude.ai/connect-github
```

Lorsque vous créez une routine avec [`/schedule`](/docs/fr/routines), le même message apparaît comme une note de configuration qui nomme le référentiel ; la note ne bloque pas la création de la routine.

**Que faire :**

* Exécutez `/web-setup` pour connecter votre connexion GitHub CLI à votre compte Claude, ou connectez un compte à [claude.ai/connect-github](https://claude.ai/connect-github). Voir [Options d'authentification GitHub](/docs/fr/claude-code-on-the-web#github-authentication-options) pour voir comment les deux diffèrent.
* Réexécutez la commande une minute après la connexion

Avant la v2.1.268, Claude Code signalait ceci comme un échec temporaire de la vérification de l'application GitHub Claude et suggérait de réessayer ou d'installer l'application ; aucun des deux ne connecte un compte GitHub.

<h3 id="single-sign-on-authorization-needed">
  Autorisation d'authentification unique requise
</h3>

Vous avez exécuté [`/install-github-app`](/docs/fr/github-actions#quick-setup) et choisi un référentiel dont l'organisation applique l'authentification unique SAML. Avant la configuration, Claude Code vérifie votre accès au référentiel avec l'interface de ligne de commande GitHub, et GitHub a refusé cette vérification car votre jeton `gh` n'est pas encore autorisé pour l'organisation. L'assistant affiche l'avertissement avec les étapes à suivre :

```text theme={null}
Single sign-on authorization needed
<owner>/<repo> belongs to an organization that enforces SAML single sign-on, and your GitHub CLI token isn't authorized for it yet.
```

**Que faire :**

* Réautorisez votre connexion GitHub CLI avec les portées `repo` et `workflow` en exécutant `gh auth refresh -h github.com -s repo,workflow`, et autorisez l'organisation lorsque GitHub vous invite à l'authentification unique
* Si vous vous authentifiez avec un jeton d'accès personnel dans `GH_TOKEN`, ouvrez [github.com/settings/tokens](https://github.com/settings/tokens), sélectionnez **Configure SSO** sur le jeton, et autorisez l'organisation
* Exécutez `/install-github-app` à nouveau

Avant la v2.1.273, Claude Code affichait l'avertissement `Admin permissions required` pour cette condition à la place.

<h3 id="failed-to-resume-the-conversation">
  Impossible de reprendre la conversation
</h3>

Claude Code n'a pas pu lire ou traiter la transcription enregistrée pour la session que vous avez sélectionnée dans le [sélecteur `claude --resume`](/docs/fr/sessions#use-the-session-picker), il termine donc le processus plutôt que de continuer dans un état partiellement chargé. Le message inclut la commande pour réessayer :

```text theme={null}
Failed to resume the conversation.
Run claude --resume <session-id> to retry, or claude to start a new session.
```

Claude Code se termine avec le code 1 après avoir affiché le message. Le sélecteur `/resume` à l'intérieur d'une session en cours signale `Failed to resume conversation` dans la conversation à la place, et votre session actuelle continue de s'exécuter. Avant la v2.1.216, une reprise échouée du sélecteur `claude --resume` restait sur le spinner `Resuming conversation…` indéfiniment au lieu d'afficher ce message.

**Que faire :**

* Exécutez `claude --resume <session-id>` avec l'ID de session du message pour réessayer
* Si chaque nouvelle tentative échoue de la même manière, exécutez `claude update` et reprenez à nouveau. Les versions antérieures à v2.1.275 échouent la reprise lorsque la transcription enregistrée contient une entrée qu'elles ne peuvent pas lire.
* Si la nouvelle tentative échoue à nouveau, exécutez `claude` pour démarrer une nouvelle session

<h3 id="no-conversation-found-with-the-session-id">
  Aucune conversation trouvée avec l'ID de session
</h3>

Vous avez transmis un ID de session à `claude --resume <session-id>` et aucune transcription enregistrée ne l'a appairé :

```text theme={null}
No conversation found with session ID: <session-id>
```

Claude Code se termine avec le code 1 après avoir affiché le message. Claude Code [recherche d'abord le projet courant, puis chaque autre projet sur cette machine](/docs/fr/sessions#resume-a-session) pour l'ID. Avant la v2.1.223, la recherche s'arrêtait au répertoire du projet courant et ses worktrees git, donc reprendre à partir du répertoire où la session a travaillé en dernier.

Les causes courantes :

* **ID mal saisi** : pour une exécution non interactive, l'ID est le champ `session_id` de la sortie [`--output-format json`](/docs/fr/headless#get-structured-output)
* **Transcription supprimée** : Claude Code supprime les transcriptions après la [période de rétention](/docs/fr/sessions#where-transcripts-are-stored), 30 jours par défaut, suivant les [règles de balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically)
* **Machine différente** : Claude Code stocke les transcriptions localement, donc reprenez la session sur la machine où elle s'est exécutée
* **Copies en double** : si vous avez copié un répertoire de projet sous `~/.claude/projects` afin que deux transcriptions portent le même ID, Claude Code signale ce message plutôt que de reprendre une copie arbitrairement

**Que faire :**

* Pour une session interactive, ouvrez le [sélecteur de session](/docs/fr/sessions#use-the-session-picker) avec `claude --resume` et appuyez sur `Ctrl+A` pour l'élargir à chaque projet sur cette machine, puis sélectionnez la session
* Les sessions créées avec `claude -p` ou le [Agent SDK](/docs/fr/agent-sdk/overview) n'apparaissent pas dans le sélecteur, donc revérifiez l'ID par rapport au `session_id` que votre exécution d'origine a imprimé

<h3 id="cannot-switch-renderers-in-this-session">
  Impossible de changer de renderers dans cette session
</h3>

Lorsque vous changez de renderers, Claude Code redémarre son processus. Vous avez exécuté [`/tui`](/docs/fr/fullscreen#enable-fullscreen-rendering) dans une session que Claude Code refuse de redémarrer, elle ne change donc pas et ne sauvegarde rien. Le message que vous voyez vous indique la cause :

* `Cannot switch renderers while work is running in the background` : vous avez du travail en arrière-plan en cours d'exécution qu'un redémarrage abandonnerait, tel qu'un shell en arrière-plan ou un sous-agent. Attendez que le travail se termine ou arrêtez-le avec [`/tasks`](/docs/fr/commands), puis exécutez `/tui fullscreen` ou `/tui default` à nouveau
* `Cannot switch renderers in this session` : la session a des restrictions que Claude Code ne peut pas transmettre au processus redémarré. Avant la v2.1.234, Claude Code redémarrait de toute façon et la session relancée s'exécutait sans elles

Dans le message des restrictions, la partie entre parenthèses nomme les restrictions que Claude Code a trouvées :

```text theme={null}
Cannot switch renderers in this session — it has restrictions a restart can't carry over (permission rules set for this session only). Nothing was changed. Running /tui fullscreen in a session started without them switches every later session too.
```

Chaque raison que le message peut afficher entre parenthèses :

* `launch flags: a custom system prompt, a tool allowlist, or restricted settings` : vous avez démarré la session avec un indicateur que Claude Code ne transmet pas au processus redémarré. Ces indicateurs incluent [`--system-prompt`](/docs/fr/cli-reference#cli-flags), `--system-prompt-file`, `--append-system-prompt-file`, une liste d'autorisation [`--tools`](/docs/fr/cli-reference#cli-flags), [`--setting-sources`](/docs/fr/cli-reference#cli-flags), et [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags)
* `permission rules set for this session only` : une [mise à jour de permission](/docs/fr/hooks#permission-update-entries) d'un hook ou d'un appelant SDK a ajouté des règles de refus ou de demande avec la destination `session`. Les règles d'autorisation à portée de session ne déclenchent pas le refus. Un redémarrage les supprime, et Claude Code demande à nouveau à la place
* `ask-before-running rules with no command-line form` : une mise à jour de permission d'un hook ou d'un appelant SDK a ajouté des règles de demande aux côtés des règles que Claude Code transmet comme `--allowed-tools` et `--disallowed-tools`. Aucun indicateur n'existe pour les règles de demande
* `permission rules a command line cannot carry intact` et `added directories a command line cannot carry intact` : une mise à jour de permission a ajouté une règle ou un chemin de répertoire en milieu de session. La ligne de commande du processus redémarré ne peut pas porter son texte comme la même valeur

**Que faire :**

* Dans une session démarrée sans ces restrictions, exécutez `/tui fullscreen`, ou `/tui default` pour revenir. Claude Code sauvegarde le [paramètre `tui`](/docs/fr/settings-reference#tui) là-bas

<h3 id="couldnt-open-claude-desktop">
  Impossible d'ouvrir Claude Desktop
</h3>

Vous avez exécuté [`/desktop`](/docs/fr/desktop#coming-from-the-cli), ou son alias `/app`, et la commande système que Claude Code utilise pour ouvrir Claude Desktop a échoué. La session reste dans le terminal.

```text theme={null}
Error: Couldn't open Claude Desktop (`open` exited 1: LSOpenURLsWithRole() failed for the URL claude://resume?session=<session-id> with error -10814). Open Claude Desktop and run /desktop again.
```

**Que faire :**

* Ouvrez Claude Desktop vous-même, puis exécutez `/desktop` à nouveau
* Pour lire la sortie d'erreur complète de cette commande, activez la journalisation de débogage avec `/debug`, exécutez `/desktop` à nouveau, et vérifiez le journal de débogage

Avant la v2.1.275, le message était `Failed to open Claude Desktop. Please try opening it manually.` et ne disait pas ce qui a échoué.

<h3 id="terminal-setup-left-your-zed-keymap-unchanged">
  /terminal-setup a laissé votre keymap Zed inchangée
</h3>

Vous avez exécuté [`/terminal-setup`](/docs/fr/terminal-config#enter-multiline-prompts) dans Zed, et Claude Code n'a pas pu terminer la mise à jour de votre Zed `keymap.json`, il a donc laissé le fichier tel qu'il était.

Chaque message nomme le chemin vers votre keymap et se termine avec le bloc de liaison de clé à ajouter vous-même :

```text theme={null}
Couldn't update your Zed keymap, so it was left unchanged.
To add the binding yourself, add this block to the keymap array in <path to keymap.json>:
{ "context": "Terminal", "bindings": { "shift-enter": ["terminal::SendText", "\u001b\r"] } }
```

La première ligne du message nomme la cause :

* `Couldn't read your Zed keymap, so it was left unchanged.` : Claude Code n'a pas pu lire le fichier, par exemple en raison de permissions de fichier
* `Your Zed keymap isn't a readable list of keybindings, so it was left unchanged.` : le fichier s'est bien lu mais ne s'analyse pas comme un tableau de blocs de liaison de clé, même avec les commentaires `//` et les virgules finales autorisés
* `Couldn't back up your Zed keymap; not modifying it.` : Claude Code n'a pas pu copier le fichier vers une sauvegarde `.bak` à côté, il n'a donc rien changé
* `Couldn't update your Zed keymap, so it was left unchanged.` : le résultat fusionné n'a pas vérifié comme un keymap valide portant la liaison, donc Claude Code l'a rejeté au lieu de l'écrire. Un bloc de liaison de clé avec une clé dupliquée peut causer ceci

**Que faire :**

* Copiez le bloc du message dans le tableau de niveau supérieur dans votre `keymap.json` au chemin que le message nomme
* Pour `isn't a readable list of keybindings`, corrigez l'erreur de syntaxe, ou rendez la valeur de niveau supérieur du fichier un tableau, puis exécutez `/terminal-setup` à nouveau

Avant la v2.1.247, `/terminal-setup` ne pouvait pas analyser un keymap Zed qui utilisait des commentaires `//` ou des virgules finales, et il remplaçait le fichier entier par uniquement sa propre liaison tout en signalant la liaison comme installée. Pour restaurer un keymap qu'une version antérieure a remplacé, utilisez le fichier de sauvegarde `.bak` décrit sous [Entrer des invites multiligne](/docs/fr/terminal-config#enter-multiline-prompts).

<h3 id="skill-usage-reports-are-not-available-on-this-connection">
  Les rapports d'utilisation des skills ne sont pas disponibles sur cette connexion
</h3>

Vous avez exécuté [`/skill-doctor`](/docs/fr/skills#find-unused-skills) sur [Contrôle à distance](/docs/fr/remote-control), à partir de votre téléphone ou navigateur. Claude Code n'envoie pas le rapport d'utilisation des skills sur le contrôle à distance et répond avec ce message à la place :

```text theme={null}
Skill usage reports are not available on this connection.
```

**Que faire :**

* Exécutez `/skill-doctor` dans le terminal sur la machine où la session s'exécute, ou exécutez `claude -p "/skill-doctor"` là-bas

<h3 id="custom-output-styles-cant-be-selected-over-remote-control">
  Les styles de sortie personnalisés ne peuvent pas être sélectionnés sur le contrôle à distance
</h3>

Vous avez exécuté [`/output-style`](/docs/fr/output-styles#change-your-output-style) à partir de l'application mobile ou web via [Contrôle à distance](/docs/fr/remote-control), ou la commande est arrivée dans un message relayé dans la session. Parce qu'un tel tour peut ne pas provenir du propriétaire du compte, Claude Code énumère et sélectionne uniquement les [styles intégrés](/docs/fr/output-styles#built-in-output-styles) sur celui-ci, et ajoute cet avis chaque fois que la commande énumère les styles ou ne reconnaît pas le nom que vous avez donné. Un nom de [style personnalisé](/docs/fr/output-styles#create-a-custom-output-style) reçoit la même réponse qu'un nom qui n'existe pas :

```text theme={null}
Custom output styles can't be selected over Remote Control or from a relayed message. Select one in the session itself, or pick a built-in style here.
```

**Que faire :**

* Choisissez un style intégré, par exemple `/output-style concise`
* Pour utiliser un style personnalisé, définissez [`outputStyle`](/docs/fr/settings-reference#outputstyle) dans le `.claude/settings.local.json` du projet, ou exécutez `/output-style <style>` au terminal propre de la session s'il en a un

<h3 id="output-styles-are-saved-to-local-settings-which-this-session-doesnt-load">
  Les styles de sortie sont enregistrés dans les paramètres locaux que cette session ne charge pas
</h3>

Vous avez essayé de changer les [styles de sortie](/docs/fr/output-styles) avec `/output-style <style>` ou `/config outputStyle=<style>` dans une session dont les sources de paramètres excluent `local`. Les exemples sont une session [Agent SDK](/docs/fr/agent-sdk/typescript) dont [`settingSources`](/docs/fr/agent-sdk/typescript#options) laisse de côté `"local"` et une session CLI démarrée avec une valeur [`--setting-sources`](/docs/fr/cli-reference#cli-flags) qui laisse de côté `local`. Les deux commandes enregistrent le style dans `.claude/settings.local.json`, un fichier qu'une telle session ne relit jamais, donc Claude Code refuse au lieu d'écrire un paramètre qui n'aurait aucun effet :

```text theme={null}
Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load, so the style can't be changed here.
```

**Que faire :**

* Ajoutez `local` aux sources de paramètres de la session et changez à nouveau
* Définissez la clé [`outputStyle`](/docs/fr/settings-reference#outputstyle) dans un fichier de paramètres que la session charge, tel que `.claude/settings.json` dans le projet ou `~/.claude/settings.json`. Dans le SDK TypeScript, définissez `outputStyle` à l'intérieur de l'objet `settings` en ligne à la place ; voir [Activer un style de sortie](/docs/fr/agent-sdk/modifying-system-prompts#activate-an-output-style)

<h2 id="plugin-errors">
  Erreurs de plugin
</h2>

Ces erreurs proviennent de la configuration des [plugins](/docs/fr/plugins/overview) et des [marketplaces](/docs/fr/plugins/overview). Pour les problèmes de plugin qui ne produisent pas l'un des messages de cette page, comme une URL de marketplace qui ne se charge pas ou un plugin qui s'installe mais n'apparaît pas, consultez [Dépannage des plugins](/docs/fr/plugins/troubleshooting).

<h3 id="plugin-eval-is-currently-in-early-access">
  plugin eval is currently in early access
</h3>

Vous avez exécuté [`claude plugin eval`](/docs/fr/plugin-evals) ou `claude plugin eval init` et il a quitté avec le code 1 avec l'un de ces messages avant de faire quoi que ce soit :

```text theme={null}
`plugin eval` is currently in early access
```

```text theme={null}
`plugin eval` is currently unavailable
```

Le premier message signifie que votre build est plus ancien que v2.1.269, la première version où la commande est généralement disponible. Le second signifie qu'Anthropic a désactivé la commande côté serveur ; rien sur votre machine ne la réactive.

**Que faire :**

* Exécutez `claude --version`, puis `claude update`, et exécutez la commande à nouveau dans une nouvelle session. Consultez les [exigences pour les évaluations de plugins](/docs/fr/plugin-evals#requirements)
* Si vous voyez le second message sur une build actuelle, réessayez plus tard après un autre `claude update`

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace is registered from an untrusted source
</h3>

La marketplace est enregistrée sous un nom qui est [réservé aux marketplaces officielles d'Anthropic](/docs/fr/plugins/marketplace-reference#marketplace-file), mais sa source enregistrée n'est pas un référentiel GitHub `anthropics`. Claude Code revérifie les noms réservés chaque fois qu'il charge ou actualise une marketplace, donc la marketplace et les plugins installés à partir de celle-ci cessent de se charger. Avant v2.1.205, le nom n'était vérifié que lorsque la marketplace était ajoutée, donc une entrée enregistrée avant que son nom ne soit réservé continuait à se charger.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

Pour une marketplace dont la source n'est pas un référentiel GitHub ou une URL Git, comme un répertoire local, la phrase du milieu se lit `can only be used with GitHub sources from the 'anthropics' organization` à la place. `claude plugin marketplace add` exécute la même vérification et refuse un nom réservé avec `Failed to add marketplace:` suivi de la même phrase de nom réservé.

**Que faire :**

* Si la marketplace est déjà enregistrée, exécutez `claude plugin marketplace remove <name>`, puis ajoutez-la à nouveau à partir du référentiel officiel `github.com/anthropics`
* Si vous publiez une marketplace tierce qui utilisait le nom avant qu'il ne soit réservé, renommez-la et demandez aux utilisateurs de la rajouter à partir de votre source
* Consultez la liste des noms réservés sous [Marketplace schema](/docs/fr/plugins/marketplace-reference#marketplace-file)

<h3 id="marketplace-name-is-another-spelling-of-a-reserved-name">
  Marketplace name is another spelling of a reserved name
</h3>

Le nom de la marketplace n'est pas lui-même un nom réservé, mais Claude Code le traite comme une autre orthographe d'un. [Reserved names](/docs/fr/plugins/marketplace-reference#reserved-name-spellings) énumère les orthographes qui comptent comme un nom réservé. Claude Code refuse un tel nom quand vous ajoutez la marketplace :

```text theme={null}
Failed to add marketplace: "claude.code.plugins" is another spelling of "claude-code-plugins", a reserved marketplace name.
```

Quand une marketplace est déjà enregistrée sous un tel nom, son entrée cesse de se charger, et `/plugin`, `claude plugin install`, et `claude plugin update` avertissent :

```text wrap theme={null}
known_marketplaces.json has an entry named "claude.code.plugins", another spelling of the reserved marketplace name "claude-code-plugins", so it is ignored. Remove it with: claude plugin marketplace remove claude.code.plugins
```

Quand le nom aurait besoin de guillemets shell, le refus au moment de l'ajout se lit `This marketplace's name is another spelling of "<reserved>", a reserved marketplace name. It is not exactly the reserved name it appears to be.`

**Que faire :**

* Renommez la marketplace en un nom qui n'orthographie pas un nom réservé et ajoutez-la à nouveau
* Pour l'avertissement d'entrée ignorée, exécutez la commande `claude plugin marketplace remove` qu'il donne, ou supprimez l'entrée de `~/.claude/plugins/known_marketplaces.json`

<h3 id="marketplace-is-already-added-from-a-different-source">
  Marketplace is already added from a different source
</h3>

Vous avez confirmé l'ajout d'une marketplace via [`/plugin install <plugin> --marketplace <source>`](/docs/fr/plugins/install#add-a-marketplace-and-install-in-one-command), et le catalogue que Claude Code a récupéré à partir de cette source se nomme lui-même de la même façon qu'une marketplace que vous avez déjà ajoutée à partir d'une source différente. Claude Code conserve la marketplace existante au lieu de la remplacer, et le plugin n'est pas installé.

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

**Que faire :**

* Si la marketplace que vous avez déjà ajoutée est celle que vous voulez, installez à partir de celle-ci par nom : `/plugin install <plugin>@<name>`
* Pour basculer vers la nouvelle source, exécutez `/plugin marketplace remove <name>`, puis réessayez l'installation

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

Un hook de plugin, un [monitor](/docs/fr/plugins/components#monitors), ou une commande MCP [`headersHelper`](/docs/fr/mcp#use-dynamic-headers-for-custom-authentication) référence une [option de plugin](/docs/fr/plugins/manifest-reference#user-configuration) `${user_config.KEY}`, et la chaîne substituée serait passée à un shell. Une valeur configurée contenant `$(...)`, des backticks, ou `;` s'exécuterait comme du code là-bas, donc Claude Code refuse de démarrer le composant au lieu de substituer la valeur. La vérification s'exécute sur le modèle de commande, donc l'erreur apparaît même quand aucune valeur n'est encore configurée. Avant v2.1.207, la valeur était substituée dans la commande shell.

La formulation dépend de quelle surface a référencé l'option. Un hook de forme shell rapporte :

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Un monitor rapporte :

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

Un MCP `headersHelper` rapporte :

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Que faire :**

* Pour un hook, ajoutez un tableau `args` pour qu'il s'exécute en [forme exec](/docs/fr/hooks#exec-form-and-shell-form), où chaque `${user_config.KEY}` devient un argument sans shell entre les deux. Ou supprimez la référence et lisez la variable d'environnement `$CLAUDE_PLUGIN_OPTION_<KEY>` à l'intérieur du script
* Pour un monitor, supprimez la référence et faites en sorte que le script monitor lise la valeur à partir d'un fichier de configuration
* Pour un `headersHelper`, déplacez `${user_config.KEY}` dans le champ `headers` du serveur, qui n'est pas analysé par shell, ou lisez la valeur à l'intérieur du script helper

<h3 id="plugin-archive-integrity-check-failed">
  Plugin archive integrity check failed
</h3>

L'entrée de marketplace du plugin utilise une [source `archive`](/docs/fr/plugins/marketplace-reference#archive-plugin-source) avec une épingle `sha256`, et le digest du fichier téléchargé ne correspond pas à l'épingle. Claude Code refuse l'installation, donc rien ne change dans le cache du plugin. L'inadéquation a trois causes possibles :

* Le fichier à l'URL a changé après que l'auteur ait calculé l'épingle
* L'auteur a entré le mauvais digest dans l'entrée de marketplace
* L'URL sert un fichier différent de celui que l'auteur a épinglé

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

**Que faire :**

* Si vous publiez le plugin, recalculez le digest du fichier exact que l'URL sert, par exemple avec `shasum -a 256 my-plugin.zip`, ou `Get-FileHash -Algorithm SHA256 my-plugin.zip` dans PowerShell, et mettez à jour le `sha256` dans l'entrée de marketplace
* Si vous installez le plugin, exécutez `/plugin marketplace update <name>` pour actualiser le catalogue au cas où l'entrée aurait été corrigée, puis réessayez l'installation
* Si les digests ne correspondent toujours pas après une actualisation, demandez au propriétaire de la marketplace quel fichier il a épinglé avant d'installer

<h3 id="path-escapes-plugin-directory">
  Path escapes plugin directory
</h3>

Un chemin de composant de plugin, déclaré dans le `plugin.json` du plugin ou dans son [entrée de marketplace](/docs/fr/plugins/marketplace-reference#plugin-entries), se résout en dehors du répertoire du plugin. Claude Code supprime ce chemin et charge le reste du plugin. Le nom du composant dans le message, comme `commands` ou `hooks`, nomme le champ qui a déclaré le chemin.

```text theme={null}
commands path escapes plugin directory: ./../shared.md
```

Dans la sortie de la commande `claude plugin`, la même erreur se lit `Path escapes plugin directory: ./../shared.md (commands)`.

Claude Code rejette à la fois un chemin qui pointe en dehors du plugin tel qu'écrit, comme `../shared-utils`, et un lien symbolique qui mène en dehors du plugin et n'en est pas un que les [règles de lien symbolique de marketplace](/docs/fr/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks) permettent. Pour un lien symbolique, le message indique également où le chemin se résout :

```text theme={null}
commands path escapes plugin directory: ./commands/deploy.md — it resolves to /home/user/shared/deploy.md, outside the plugin directory
```

Sur macOS et Linux, Claude Code rejette également un chemin de composant qui contient une barre oblique inverse n'importe où dedans, même quand le chemin reste à l'intérieur du plugin. Un plugin dont les chemins de composant utilisent des séparateurs de style Windows se charge sur Windows et déclenche ce rejet sur les autres plates-formes :

```text theme={null}
commands path escapes plugin directory: ./commands\deploy.md — its path contains a backslash, which is not resolved reliably on this platform
```

Avant v2.1.251, Claude Code chargeait un chemin `commands` déclaré dans une entrée de marketplace même quand il pointait en dehors du répertoire du plugin. Claude Code rejetait déjà les chemins déclarés dans `plugin.json` et les autres chemins de composant dans une entrée de marketplace.

Avant v2.1.257, la vérification ne regardait que l'orthographe du chemin, pas où un lien symbolique mène.

**Que faire :**

* Déplacez le fichier référencé à l'intérieur du répertoire du plugin et pointez le chemin vers lui avec un chemin relatif `./`
* Si le chemin est un lien symbolique vers un fichier en dehors du plugin, remplacez le lien symbolique par une copie du fichier
* Si le message dit que le chemin contient une barre oblique inverse, écrivez le chemin avec des barres obliques avant, par exemple `./commands/deploy.md`
* Pour partager des fichiers avec d'autres plugins dans la même marketplace, liez-les avec un lien symbolique à l'intérieur du répertoire du plugin, en suivant les [règles de lien symbolique](/docs/fr/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)

<h3 id="path-could-not-be-checked">
  Path could not be checked
</h3>

Claude Code a demandé au système d'exploitation si un chemin de plugin existe et a reçu une erreur autre que « non trouvé », donc il ne charge pas ce que le chemin nomme. La quantité du plugin qui se charge dépend du chemin qui a échoué :

* L'un des [emplacements de composant par défaut](/docs/fr/plugins/manifest-reference#standard-layout) d'un plugin, comme le dossier `skills/`, le fichier `monitors/monitors.json`, ou un [`SKILL.md` à la racine du plugin](/docs/fr/plugins/components#skills) : les autres composants du plugin se chargent toujours
* Le répertoire du plugin lui-même : rien de ce plugin ne se charge

Vous ne voyez pas cette erreur pour un chemin qui n'existe pas du tout. Dans `/plugin`, l'erreur apparaît sous le plugin et nomme le chemin et le code que le système d'exploitation a retourné :

```text theme={null}
skills path could not be checked: /home/user/my-plugin/skills (ELOOP)
```

Dans `claude plugin list`, la même erreur se lit `Path not found: /home/user/my-plugin/skills (skills, ELOOP)`.

Les causes qui produisent cette erreur incluent :

* `ELOOP` : un lien symbolique dans le chemin pointe sur lui-même ou forme une boucle
* `EIO` ou `ESTALE` : le chemin est sur un montage réseau qui est cassé ou obsolète
* `EACCES` : l'un des répertoires au-dessus du chemin vous refuse la permission de le traverser

**Que faire :**

* Remplacez un lien symbolique qui pointe sur lui-même par un vrai dossier, ou supprimez-le
* Si le chemin est sur un montage réseau, remontez le partage
* Si le code est `EACCES`, restaurez votre permission d'exécution sur les répertoires au-dessus du chemin
* Exécutez `/reload-plugins` après avoir corrigé le chemin, ou redémarrez Claude Code, pour charger le plugin ou le composant

Avant v2.1.265, Claude Code traitait un dossier de composant par défaut qu'il ne pouvait pas vérifier comme absent et chargeait le plugin sans ce composant, sans erreur.

<h3 id="marketplace-entry-path-does-not-stay-inside-the-marketplace-directory">
  Marketplace entry path does not stay inside the marketplace directory
</h3>

L'[entrée de marketplace](/docs/fr/plugins/marketplace-reference#plugin-entries) du plugin déclare un chemin source que Claude Code ne peut pas résoudre à un emplacement à l'intérieur du répertoire de la marketplace elle-même, donc le plugin ne s'installe pas ou ne se charge pas. Le refus couvre :

* Un chemin d'entrée qui est absolu, grimpe en dehors de la marketplace avec `..`, ou est orthographié comme un chemin réseau
* Sur macOS et Linux, un chemin d'entrée qui contient une barre oblique inverse n'importe où après le `./` initial
* Une entrée dans une marketplace récupérée à partir d'une source distante, comme git ou une URL, qui atteint sa cible via un lien symbolique se résolvant en dehors du répertoire de la marketplace
* Une entrée relative dans une marketplace ajoutée à partir d'une URL directe vers son `marketplace.json` : Claude Code télécharge uniquement ce fichier, donc aucun fichier de plugin local n'existe pour que le chemin nomme. Consultez [Plugins with relative paths fail in URL-based marketplaces](/docs/fr/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)

`claude plugin install` rapporte le refus comme ceci :

```text theme={null}
Cannot install my-plugin@my-marketplace: its marketplace entry path does not stay inside the marketplace directory (an absolute, climbing, network-shaped, backslash-containing or link-traversing entry, an entry of a fetched marketplace that resolves or opens outside its tree — or a relative entry in a url-catalog marketplace, which has no local directory)
```

Quand une entrée d'un plugin déjà installé échoue la même vérification, `claude plugin list` affiche le plugin comme `failed to load` avec :

```text theme={null}
Plugin source path refused: ./my-plugin does not stay inside its marketplace directory. Check that the marketplace entry has a plain relative path.
```

**Que faire :**

* Si vous maintenez la marketplace, écrivez la `source` de l'entrée comme un chemin relatif simple avec des barres obliques avant, comme `./plugins/my-plugin`, et gardez tout lien symbolique qu'il traverse pointé à l'intérieur du répertoire de la marketplace
* Si vous avez ajouté la marketplace à partir d'une URL directe, les entrées relatives ne peuvent pas se résoudre. Demandez à l'auteur de la marketplace d'utiliser [une autre source de plugin](/docs/fr/plugins/marketplace-reference#plugin-sources), ou ajoutez la marketplace à partir de son référentiel git à la place

<h3 id="failed-to-load-marketplace-configuration">
  Failed to load marketplace configuration
</h3>

Claude Code garde les marketplaces de plugins que vous avez ajoutées dans un fichier de registre à `~/.claude/plugins/known_marketplaces.json`. Une commande de plugin qui a besoin du registre, comme `claude plugin install`, échoue avec l'un de deux messages quand Claude Code ne peut pas utiliser le fichier :

* `Failed to load marketplace configuration` : le fichier n'est pas un JSON valide, ou ne peut pas être lu. Un fichier vide échoue de cette façon aussi.
* `Marketplace configuration file is corrupted` : le fichier est un JSON valide mais son contenu ne correspond pas au schéma du registre.

Un fichier manquant n'est pas un échec : Claude Code le traite comme un registre sans marketplaces.

Avec un fichier vide, `claude plugin install` rapporte :

```text theme={null}
✘ Failed to install plugin "my-plugin": Failed to load marketplace configuration: JSON Parse error: Unexpected EOF
```

Avant v2.1.246, `claude plugin install` ne rapportait pas cet échec.

**Que faire :**

* Ouvrez `~/.claude/plugins/known_marketplaces.json` et réparez le JSON, ou corrigez les entrées que le message nomme comme ne correspondant pas au schéma du registre
* Si vous ne pouvez pas le réparer, supprimez le fichier ou remplacez son contenu par `{}`, puis rajoutez chaque marketplace avec `claude plugin marketplace add <source>`. Claude Code réenregistre les marketplaces que vos paramètres utilisateur ou gérés déclarent dans [`extraKnownMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces) la prochaine fois que vous le démarrez dans un dossier que vous avez approuvé.

<h3 id="plugin-is-required-by-your-organization">
  Plugin is required by your organization
</h3>

Vous avez exécuté `claude plugin disable`, ou utilisé l'onglet **Installed** de `/plugin`, pour désactiver un [plugin synchronisé à partir de claude.ai](/docs/fr/plugins/loading#synced-plugins) que votre organisation marque comme requis :

```text theme={null}
Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.
```

Claude Code ne sauvegarde rien et le plugin reste activé.

Quand vous essayez de désactiver un plugin dont un plugin requis dépend, Claude Code refuse de la même façon, avec un message nommant le plugin requis qui en a besoin.

**Que faire :**

* Demandez à un administrateur de votre organisation claude.ai de modifier le statut requis du plugin sur claude.ai

<h2 id="tool-errors">
  Erreurs d'outils
</h2>

Ces erreurs proviennent des outils intégrés de Claude. Claude corrige la plupart des erreurs d'outils de lui-même. Quand l'une d'elles nécessite une modification de votre part, la liste **Que faire** de cette erreur indique ce qu'il faut modifier.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent would be spawned with zero tools
</h3>

Chaque entrée de la liste [`tools` du sous-agent](/docs/fr/sub-agents#supported-frontmatter-fields) n'a pas correspondu à un outil utilisable, donc Claude Code a refusé de lancer le sous-agent : sans outils, il ne pouvait pas agir. Le message regroupe vos entrées par ce qui s'est mal passé :

* **Unrecognized** : l'entrée ne correspond à aucun nom d'outil, généralement une faute de frappe comme `Grpe` pour `Grep`.
* **Not available to subagents** : l'entrée nomme un outil réel que [les sous-agents ne peuvent pas utiliser](/docs/fr/sub-agents#available-tools). Les sous-agents en arrière-plan conservent un ensemble d'outils intégrés plus petit, donc une entrée qui ne peut être utilisée que par un sous-agent au premier plan se retrouve ici quand le sous-agent s'exécuterait en arrière-plan, ce qui est le comportement par défaut. Si vous listez `Agent`, le message le signale dans le groupe suivant à la place.
* **Matched no tools in this session** : l'entrée est valide mais aucun outil de la session actuelle ne correspond actuellement, comme `mcp__github__*` sans serveur MCP GitHub connecté, ou `Agent` pour un sous-agent à la [limite de profondeur](/docs/fr/sub-agents#let-subagents-spawn-their-own-subagents).

Omettre le champ `tools` ne déclenche jamais ce refus. Si vous laissez la liste `tools` vide, ou si `disallowedTools` supprime chaque entrée, Claude Code ignore également le refus et lance le sous-agent sans outils.

Avant la v2.1.208, le sous-agent était lancé sans outils et pouvait retourner un résultat vide ou confus.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**Que faire :**

* Corrigez chaque entrée que l'erreur nomme par rapport aux [outils disponibles pour les sous-agents](/docs/fr/sub-agents#available-tools)
* Supprimez les entrées pour les outils que la session n'a pas, comme les outils MCP d'un serveur qui n'est pas connecté
* Pour un outil que [les sous-agents en arrière-plan abandonnent](/docs/fr/sub-agents#available-tools), comme `CronCreate`, supprimez l'entrée. Pour conserver l'outil, [désactivez le mode fork](/docs/fr/sub-agents#turn-fork-mode-on-or-off) et demandez à Claude d'exécuter le sous-agent au premier plan
* Supprimez le champ `tools` au lieu de lister les outils pour donner au sous-agent chaque [outil disponible pour les sous-agents](/docs/fr/sub-agents#available-tools)
* Pour une liste `tools` qui contient uniquement `Agent`, augmentez la [limite de profondeur](/docs/fr/sub-agents#let-subagents-spawn-their-own-subagents) ou donnez à l'agent au moins un autre outil : Claude Code retient `Agent` à cette limite, donc une liste avec rien d'autre dedans se résout à aucun outil

<h3 id="file-is-covered-by-a-read-deny-rule">
  File is covered by a Read deny rule
</h3>

L'outil Edit ou Write a été appelé sur un chemin correspondant à une [règle de refus `Read`](/docs/fr/permissions#read-and-edit), y compris la création d'un nouveau fichier à ce chemin. Les deux outils modifient le contenu que Claude doit pouvoir relire, donc Claude Code refuse l'appel avant tout accès au fichier. NotebookEdit n'est pas couvert par les règles de refus `Read`. Avant la v2.1.228, la règle bloquait uniquement l'outil Edit, et avant la v2.1.208, seule une règle de refus `Edit` bloquait les modifications.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

Quand Claude Code refuse l'outil Write, le message se termine par `and cannot be written` à la place.

**Que faire :**

* Si Claude doit pouvoir modifier le fichier, supprimez ou réduisez la règle de refus `Read` dans `/permissions` ou dans [les paramètres](/docs/fr/settings-reference#permission-settings)
* Si le fichier doit rester inchangé, conservez la règle et ajoutez une règle de refus `Edit` pour le même chemin pour bloquer également l'outil NotebookEdit

<h3 id="subagent-type-is-required">
  subagent\_type is required
</h3>

```text theme={null}
subagent_type is required: the general-purpose agent is not available in this session. Available agents: ...
```

Claude a appelé l'[outil Agent](/docs/fr/tools-reference#agent-tool-behavior) sans `subagent_type`, et cette session n'a pas de [sous-agent polyvalent](/docs/fr/sub-agents#built-in-subagents) sur lequel se replier. C'est le cas dans deux configurations :

* [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/fr/env-vars) est défini en mode non interactif, ce qui supprime chaque sous-agent intégré
* L'agent du thread principal de la session a une [liste d'autorisation `tools: Agent(...)`](/docs/fr/sub-agents#restrict-which-subagents-can-be-spawned) qui exclut `general-purpose`

**Que faire :**

* Généralement rien : le message liste les sous-agents que la session a, donc Claude peut réessayer avec l'un d'eux
* Si Claude continue d'échouer, ajoutez `general-purpose` à la liste d'autorisation `tools: Agent(...)`, ou désactivez `CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS`

Avant la v2.1.235, le même appel échouait avec `Agent type 'general-purpose' not found`.

<h3 id="memory-index-is-over-its-read-limit">
  Memory index is over its read limit
</h3>

Claude a écrit dans l'index de [mémoire automatique](/docs/fr/memory#auto-memory) `MEMORY.md` et l'a laissé au-delà de l'une de ses limites de lecture : 200 lignes ou 25 Ko. L'écriture a réussi, mais seules les 200 premières lignes ou 25 Ko, selon ce qui vient en premier, se chargent au début d'une session, donc tout ce qui dépasse la limite est supprimé à chaque fois que l'index est lu. Avant la v2.1.210, un index dépassant la limite était silencieusement tronqué au prochain chargement sans signal au moment de l'écriture.

```text theme={null}
Error: this write left the memory index at MEMORY.md at 214 lines, over its 200-line read limit. The write succeeded, but everything past the limit is silently dropped each time the index is loaded — entries at the end are already invisible to readers. Rewrite it to under 140 lines now: keep one line per entry, move detail into topic files, and merge or drop stale entries.
```

Seul le contenu qui se charge compte pour les limites. Le frontmatter YAML et les commentaires HTML au niveau des blocs sont supprimés avant le chargement de l'index, donc ils sont exclus de la mesure. Avant la v2.1.211, Claude Code mesurait le fichier brut, et le frontmatter ou les commentaires pouvaient déclencher cette erreur même quand le contenu chargé s'adaptait.

Claude Code remet l'erreur à Claude après l'écriture plutôt que de l'imprimer comme une bannière dans votre terminal, donc vous ne la remarquerez peut-être que dans la transcription.

Quand l'écriture de Claude rapproche le fichier d'une limite sans la dépasser, Claude Code retourne un rappel plus doux pour compacter l'index au lieu de cette erreur.

**Que faire :**

* Laissez Claude réécrire `MEMORY.md`, ou demandez-lui : gardez une ligne par entrée, déplacez les détails dans les fichiers de sujet, et fusionnez ou supprimez les entrées obsolètes
* Pour réduire l'index vous-même, voir [Audit and edit your memory](/docs/fr/memory#audit-and-edit-your-memory)

<h3 id="pkill-pattern-matches-the-claude-code-process">
  pkill pattern matches the Claude Code process
</h3>

Une commande `pkill` dans un appel d'outil Bash a utilisé un motif, généralement avec `-f`, qui correspond au processus Claude Code lui-même, donc Claude Code refuse la commande au lieu de laisser la session se terminer. Claude Code teste le motif avec `pgrep` avant d'exécuter `pkill` et refuse quand son propre ID de processus est dans le résultat. La vérification s'exécute uniquement sur Linux ; sur macOS, `pkill` s'exécute sans modification. Avant la v2.1.214, la commande s'exécutait, et un motif correspondant tuait la session Claude Code en cours de tour.

```text theme={null}
pkill: refusing to run — this pattern matches the Claude CLI process (PID 12345). Narrow the pattern, or target your own children with `pkill -P $$ ...`.
```

Le refus apparaît dans le résultat de l'outil Bash plutôt que comme une bannière dans votre terminal, et Claude ajuste généralement la commande de lui-même.

**Que faire :**

* Réduisez le motif pour qu'il ne corresponde qu'au processus prévu, par exemple le chemin complet du binaire cible plutôt qu'une courte sous-chaîne
* Pour arrêter les processus démarrés par le shell actuel, utilisez `pkill -P $$` avec le motif, ce qui limite la correspondance aux processus enfants du shell

<h3 id="failed-to-write-to-a-teammate-inbox">
  Failed to write to a teammate's inbox
</h3>

Claude Code n'a pas pu écrire un message dans la boîte aux lettres d'un coéquipier sous `~/.claude/teams/{team-name}/inboxes/`, donc le destinataire n'a rien reçu. L'écriture échoue quand Claude Code ne peut pas créer ou mettre à jour le fichier, par exemple parce que le disque est plein, le répertoire n'est pas accessible en écriture, ou un autre agent détient le verrou de la boîte aux lettres trop longtemps. Avant la v2.1.224, Claude Code signalait le message comme envoyé même quand l'écriture échouait.

L'erreur apparaît dans le résultat de l'outil de l'agent d'envoi plutôt que comme une bannière dans votre terminal, et son texte indique à Claude de réessayer :

```text theme={null}
Failed to write to researcher's inbox — nothing was sent. Try again, or message the lead.
```

Les messages de protocole structurés de l'[équipe d'agents](/docs/fr/agent-teams) échouent de la même manière, et l'erreur nomme le message non livré : quand Claude Code ne peut pas écrire une approbation de plan, un rejet de plan, une demande d'arrêt, ou un rejet d'arrêt, l'erreur se lit `Failed to write the <message> to <name>'s inbox — nothing was sent`. L'`plan approval` dans cette liste est la décision du responsable approuvant le plan d'un coéquipier ; la soumission du plan du coéquipier est le message séparé `plan approval request`. Ce message et deux autres messages de protocole portent leur propre texte de message et conséquence :

* `Failed to write the plan approval request to the lead's inbox — plan not submitted; try again` : le plan du coéquipier n'a jamais atteint le responsable, et le coéquipier reste en mode plan jusqu'à ce qu'une resoumission réussisse
* `The permission request could not be delivered to the team lead (mailbox write failed)` : la demande de permission du coéquipier n'a jamais atteint le responsable, donc personne n'a approuvé l'appel d'outil
* `The confirmation could not be written to team-lead's inbox.` : l'approbation d'arrêt elle-même a pris effet et le coéquipier sort ; seule la confirmation au responsable manque

Quand vous messagez vous-même un coéquipier, en tapant `@name` suivi du message dans la session du responsable, le même échec apparaît comme une notification, `Couldn't write to @name's inbox — message not sent. Try again.`, et Claude Code garde votre texte dans la boîte de saisie pour que vous puissiez l'envoyer à nouveau.

**Que faire :**

* Demandez à l'expéditeur de renvoyer le message ; la contention pour le verrou de la boîte aux lettres est transitoire et s'efface à la nouvelle tentative
* Vérifiez l'espace disque libre, et vérifiez que `~/.claude/teams` et les fichiers sous celui-ci sont accessibles en écriture par votre utilisateur

<h3 id="teammate-agent-definition-not-restored">
  Teammate's agent definition was not restored
</h3>

Claude a messagé un coéquipier d'[équipe d'agents](/docs/fr/agent-teams) arrêté, et Claude Code l'a ramené sans réappliquer la [définition du sous-agent](/docs/fr/agent-teams#use-subagent-definitions-for-teammates) à partir de laquelle il a été généré, parce que son fichier de définition provenait d'un dossier sans confiance enregistrée. L'avis suit le rapport de reprise dans le résultat de l'outil de l'agent d'envoi :

```text wrap theme={null}
Its agent definition was not restored: the folder its definition file came from is not trusted (source: projectSettings), so the teammate is running with the team-essential tools and no custom instructions. To restore it, the user needs to run Claude Code in that folder once and accept the trust dialog (the --debug log names the folder); do not change trust settings on the user's behalf.
```

La vérification s'applique à une définition dans le répertoire `.claude/agents/` du projet ou d'un répertoire `--add-dir`, et accepter le dialogue de confiance pour un dossier parent ne le satisfait pas.

**Que faire :**

* Exécutez `claude` dans le dossier que le [journal de débogage](/docs/fr/debug-your-config) nomme et acceptez le dialogue de confiance. La définition est réappliquée la prochaine fois que Claude Code ramène le coéquipier ; vous n'avez pas besoin de redémarrer la session du responsable
* Ou définissez l'entrée `hasTrustDialogAccepted` à `true` dans `~/.claude.json`, en utilisant la clé exacte `projects["<path>"]` que le journal de débogage imprime

<h3 id="message-too-large-for-cross-session-delivery">
  Message too large for cross-session delivery
</h3>

Le [message inter-sessions](/docs/fr/cross-session-messaging) de Claude à une autre de vos sessions sur cette machine était trop long à envoyer. Claude Code l'a refusé, et la session de réception n'a rien reçu. Le refus apparaît dans le résultat de l'outil de la session d'envoi, pas comme une bannière dans votre terminal. Il nomme les deux tailles et comment faire tenir le message :

```text wrap theme={null}
Failed to send to api-worker: Message too large for cross-session delivery: the serialized message is 1,203,844 characters and the limit is 1,048,576. Shorten the message text — put bulk content in a file the recipient can read rather than in the message — or split it into smaller messages.
```

Renvoyer le même texte échoue de la même manière.

**Que faire :**

* Demandez à Claude de résumer le message, ou de mettre le contenu en masse dans un fichier et d'envoyer le chemin du fichier
* Demandez à Claude de diviser le contenu sur plusieurs messages plus courts

Avant la v2.1.235, Claude Code signalait un message surdimensionné comme envoyé. La session de réception l'a supprimé sans le lire.

<h3 id="too-many-messages-to-this-session-just-now">
  Too many messages to this session just now
</h3>

Claude a envoyé une rafale rapide de [messages inter-sessions](/docs/fr/cross-session-messaging) à l'une de vos sessions sur cette machine, et la rafale a atteint ce que la boîte aux lettres de cette session accepte. Claude Code a refusé l'envoi suivant, et la session de réception n'a rien reçu de celui-ci. Le refus apparaît dans le résultat de l'outil de la session d'envoi, pas comme une bannière dans votre terminal :

```text wrap theme={null}
Failed to send to api-worker: Too many messages to this session just now: 30 were sent recently and more would be dropped by its rate limit, so this one was not sent. Batch what remains into one message, or wait a little before sending more.
```

**Que faire :**

* Généralement rien : Claude regroupe le contenu restant en un seul message, ou attend avant d'envoyer plus
* Si vous avez vous-même déclenché la rafale, demandez à Claude de combiner ce qui reste en un seul message

Avant la v2.1.236, Claude Code signalait ces envois comme envoyés. La session de réception les a supprimés sans les lire.

<h3 id="refusing-to-send-a-cross-session-message">
  Refusing to send a cross-session message
</h3>

Avant que Claude Code écrive un [message inter-sessions](/docs/fr/cross-session-messaging) à une autre de vos sessions sur cette machine, il vérifie que la socket de la boîte aux lettres de la session cible est le point de terminaison auquel le message a été adressé. Quand une vérification échoue, Claude Code refuse l'envoi dans la session d'envoi, et la session cible ne reçoit rien. Pour un message que Claude envoie, le refus apparaît dans le résultat de l'outil de la session d'envoi :

```text theme={null}
Failed to send to api-worker: Refusing to send: reply target is a symlink
```

Le texte après `Refusing to send:` nomme la vérification qui a échoué :

* `reply target is a symlink` : un lien symbolique se trouve au chemin de la socket de la session cible. Claude Code ne livre pas à travers, parce qu'un lien là-bas pourrait rediriger le message vers un point de terminaison que la session cible n'a pas créé.
* `cannot vet reply target` : Claude Code n'a pas pu inspecter le chemin cible du tout, par exemple parce que la lecture a échoué avec une erreur de permission.
* `connected endpoint is not the expected process` : le processus tenant la socket n'est pas la session à laquelle le message a été adressé, donc l'adresse est obsolète ou un autre processus a remplacé la socket.
* `connected endpoint identity could not be read` : Claude Code s'est connecté mais n'a pas pu lire quel processus tient l'autre extrémité, donc il n'a pas pu confirmer la cible. Cela peut être transitoire.
* `connected endpoint is not owned by this user` : le processus tenant la socket s'exécute sous un compte utilisateur différent, donc ce n'est pas l'une de vos sessions.
* `connected endpoint owner could not be read` : Claude Code s'est connecté mais n'a pas pu lire quel compte utilisateur possède l'autre extrémité, donc il n'a pas pu confirmer que le point de terminaison est le vôtre.
* `connected endpoint is a different process with the expected pid` : l'ID de processus correspond à celui auquel le message a été adressé, mais Claude Code n'a pas pu confirmer que c'est le même processus. Généralement cette session a quitté et le système d'exploitation a réutilisé son ID de processus, donc l'adresse est obsolète.

**Que faire :**

* Généralement rien : les vérifications empêchent un message d'atteindre un point de terminaison autre que la session à laquelle il a été adressé, et rien n'a été envoyé
* Demandez à Claude de lister vos sessions à nouveau et de renvoyer ; un refus causé par une adresse obsolète s'efface une fois que Claude envoie à la session actuelle
* Si `reply target is a symlink` se répète pour une session, vérifiez ce qui a créé un lien à ce chemin de socket de session, montré dans son `/status` sous `Peer address`
* Pour `connected endpoint identity could not be read`, renvoyez ; la condition peut être transitoire
* Si `connected endpoint is not owned by this user` apparaît sur une machine partagée, la session à cette adresse s'exécute sous le compte d'un autre utilisateur, donc Claude ne peut pas la messager à partir du vôtre

Avant la v2.1.248, Claude Code ne vérifiait pas l'utilisateur propriétaire du point de terminaison ou l'heure de démarrage du processus, donc les refus qui nomment ces vérifications n'apparaissent pas sur les versions antérieures.

<h3 id="refusing-after-a-symlink-changed">
  Refusing to read, write, or search a path
</h3>

Claude Code vérifie les [règles de permission](/docs/fr/permissions#read-and-edit) du chemin d'un fichier, puis confirme cette résolution à nouveau quand l'outil ouvre le fichier ou démarre la recherche. Quand il ne peut pas confirmer que le chemin mène toujours à l'emplacement que la vérification a approuvé, Claude Code refuse l'opération au lieu de la suivre. Le refus apparaît dans le résultat de l'outil :

```text wrap theme={null}
Refusing to read /path/to/file: its symlink resolution changed after permission was checked (a link on the way now leads somewhere the check did not see). If a link in the working directory is being rewritten concurrently, stop that and retry.
```

Chaque refus nomme sa raison :

* `its symlink resolution changed after permission was checked` : un lien symbolique le long du chemin, ou à une racine de recherche Grep ou Glob, a été remplacé entre la vérification de permission et l'opération. Dans un refus de lecture, la phrase entre parenthèses nomme quelle comparaison a échoué.
* `its parent-directory symlink resolution changed after permission was checked` : un répertoire par lequel le chemin d'écriture passe ne se résout plus à l'emplacement approuvé
* `it is a symbolic link. Write to the link's target path instead` : un lien symbolique se trouve à l'emplacement d'écriture approuvé lui-même, par exemple un `CLAUDE.md` qui est un lien symbolique vers `AGENTS.md` ; le message dirige Claude vers la cible du lien
* `Refusing to write through symlink: <path>. Resolve the symlink and pass the real target path explicitly.` : la même condition attrapée quand un autre écrivain ouvre le fichier, comme une écriture vers un `.mcp.json` symlinké
* `Refusing to write into symlinked directory: <path>` : le répertoire qui contient le fichier est lui-même un lien symbolique, par exemple le répertoire `.claude/` d'un projet lié à un autre emplacement
* `a path one of its Read deny rules is written through changed while the search was being prepared. Retry.` : une règle de refus `Read` pour la recherche nomme un chemin qui passe par un lien symbolique, et ce lien a changé pendant que Claude Code préparait la recherche
* `it could not be opened (EACCES) — it is unreadable, or is being replaced concurrently.` : la racine de recherche existe mais n'a pas pu être ouverte ; le code entre parenthèses est l'erreur du système d'exploitation
* `its permission check expired before it ran (too many concurrent file operations). Retry.` : Claude Code a évincé l'enregistrement d'approbation sous de nombreuses opérations de fichiers simultanées avant que l'outil l'utilise ; réessayer exécute une vérification de permission fraîche
* `ripgrep was found only by name on PATH, and a search outside the working directory cannot apply your Read deny rules in that configuration` : Claude Code n'a pas pu résoudre le binaire `rg` à un chemin absolu, donc il refuse les recherches en dehors du répertoire de travail plutôt que d'en exécuter une que vos règles de refus ne couvrent pas

**Que faire :**

* Généralement rien : le refus atteint Claude comme le résultat de l'outil, et l'opération refusée ne s'exécute pas
* Si un refus de lien symbolique se répète sur un chemin, trouvez ce qui continue de réécrire un lien là-bas, comme un outil de construction ou un observateur de fichiers, ou demandez à Claude d'utiliser le chemin résolu du fichier au lieu du lien
* Si ce refus apparaît pour chaque fichier pendant que Claude Code s'exécute sur Windows à l'intérieur d'un AppContainer ou d'un sandbox à jeton restreint, mettez à niveau vers la v2.1.265 ou ultérieure
* Si un refus de lecture apparaît sur macOS pour un fichier que rien ne réécrit, comme une capture d'écran glissée dans l'invite, mettez à niveau vers la v2.1.273 ou ultérieure
* Pour le refus ripgrep, installez ripgrep avec votre gestionnaire de paquets pour que `rg` se résout à un chemin absolu sur `PATH`, ou gardez les recherches sous le répertoire de travail

Avant la v2.1.251, Claude Code ne revérifiait la résolution d'un chemin que pour les écritures de fichiers, donc un lien remplacé après la vérification de permission pouvait rediriger une lecture ou une recherche vers un emplacement différent sans message. Parmi ceux-ci, seuls les refus d'écriture du répertoire parent, à travers le lien symbolique, et du répertoire symlinké apparaissent sur les versions antérieures.

<h3 id="task-output-swap-refused">
  Task output swap refused
</h3>

Claude Code enregistre la sortie de chaque commande Bash dans un fichier sous son répertoire temporaire. Chaque fois qu'il ouvre l'un de ces fichiers, il vérifie que le chemin mène toujours au fichier qu'il a créé, sans lien symbolique, lien physique supplémentaire, ou répertoire déplacé le redirigeant. Ce message signifie que cette vérification a échoué, donc Claude Code a refusé l'opération plutôt que d'écrire ou de lire la sortie à travers ce chemin. Le message apparaît dans le résultat de l'outil Bash :

```text wrap theme={null}
task output swap refused (tasks dir moved or linked): /private/tmp/claude-501/-Users-you-my-project/1f0e62dc-4b0a-4f5e-9c2d-8a7b6c5d4e3f/tasks/b7k2f9m3q.output. To recover: restart Claude Code with CLAUDE_CODE_TMPDIR set to a fresh directory; or, if /private/tmp/claude-501/-Users-you-my-project is a stray directory or a symbolic link that should not be there, remove that entry itself (not what it points to) and restart.
```

Le texte entre parenthèses nomme la vérification qui a échoué. Les raisons telles que `output symlink was re-pointed`, `output file identity changed`, et `not a regular file` signalent toutes la même condition : quelque chose au chemin de sortie ou le long de celui-ci n'est plus le fichier que Claude Code a créé. Seules certaines raisons portent une phrase `To recover:`.

Si la vérification échoue pendant qu'une commande s'exécute toujours, Claude Code arrête la commande, et son résultat signale :

```text theme={null}
Command killed: its output file was replaced or could no longer be verified
```

**Que faire :**

* Mettez à niveau vers la v2.1.260 ou ultérieure. Les versions antérieures affichaient parfois ce message quand aucun lien ou répertoire déplacé n'était présent
* Redémarrez Claude Code avec [`CLAUDE_CODE_TMPDIR`](/docs/fr/env-vars) défini sur un répertoire frais
* Ou vérifiez le répertoire de votre projet sous le répertoire temporaire de Claude Code, `/private/tmp/claude-501/-Users-you-my-project` dans le message d'exemple. Si ce chemin est un lien symbolique, ou un répertoire qui ne devrait pas être là, supprimez le lien ou le répertoire lui-même plutôt que la cible du lien, et redémarrez Claude Code
* Si le refus se répète, un processus remplace, lie, ou supprime des entrées sous le répertoire temporaire de Claude Code pendant que la session s'exécute. Définissez [`CLAUDE_CODE_TMPDIR`](/docs/fr/env-vars) sur un répertoire que rien d'autre ne gère et redémarrez

<h3 id="the-source-file-is-not-valid-utf-8-text">
  The source file is not valid UTF-8 text
</h3>

Claude a essayé de publier un [artifact](/docs/fr/artifacts) à partir d'un fichier dont les octets ne se décodent pas en texte, ou dont le texte contient déjà le caractère de remplacement `U+FFFD`, donc Claude Code a refusé la publication avant de télécharger quoi que ce soit. Le message apparaît dans le résultat de l'outil Artifact et nomme la première position à corriger :

```text wrap theme={null}
file_path: the source file is not valid UTF-8 text (first invalid byte at line 12, column 40). It may be saved in another encoding or contain binary data. Rewrite it as UTF-8, then publish again. Nothing was published.

file_path: the source file has the replacement character U+FFFD at line 12, column 40, usually left where an earlier edit or paste lost a character. Replace it with the intended text (in HTML, write an intended U+FFFD as &#xFFFD;), then publish again. Nothing was published.
```

Claude Code décode le fichier en UTF-8, ou en UTF-16 quand il commence par une marque d'ordre des octets UTF-16 little-endian. Quand un tel fichier UTF-16 ne se décode pas, le premier message nomme `UTF-16` et vous dit toujours de réécrire le fichier en UTF-8. Quand plus de positions suivent celle nommée, le message ajoute un compte tel que `(+2 more)` après la position.

**Que faire :**

* Généralement rien : Claude réécrit le fichier et publie à nouveau
* Si le fichier est un que vous avez écrit ou exporté, enregistrez-le à nouveau en UTF-8, et remplacez chaque `U+FFFD` par le caractère qu'une édition, un collage, ou une conversion antérieure a perdu
* Pour afficher un `U+FFFD` intentionnel sur la page, écrivez-le comme `&#xFFFD;` dans le HTML au lieu du caractère littéral

Avant la v2.1.267, Claude Code téléchargeait un tel fichier sans le vérifier, et le serveur refusait la publication à la place.

<h3 id="reading-a-local-file-from-outside-the-connected-folders">
  Reading a local file from outside the connected folders in a Cowork session
</h3>

Dans une session [Cowork](https://claude.com/docs/cowork/overview) s'exécutant sur votre machine dans l'application Claude Desktop, Claude a nommé un fichier local pour un [artifact](/docs/fr/artifacts). Claude Code n'a pas pu confirmer que le fichier est un fichier ordinaire à l'intérieur des dossiers connectés de la session : le chemin se trouve en dehors de ces dossiers, passe par un lien symbolique, ou est orthographié d'une manière qui peut nommer un fichier différent de celui qu'il semble être. Lire un tel fichier nécessite votre approbation, et dans une session qui ne peut pas vous montrer la carte d'approbation, comme une définie pour ignorer toutes les approbations, Claude Code refuse la lecture.

Le refus apparaît dans le résultat de l'outil Artifact ; quand le fichier n'a pas pu être examiné du tout, il nomme cet échec à la place :

```text wrap theme={null}
Reading a local file from outside this session's connected folders, or through a link, needs the approval card, and no one can answer it in this Cowork session. Use a plain file inside the connected folders; do not retry this file in this session.

cannot read file_path (ENOENT) — the file could not be examined, and no one can answer the approval card in this Cowork session. Check that the file exists as a plain file inside the connected folders, then retry with that path.
```

**Que faire :**

* Généralement rien : le message indique à Claude d'utiliser un fichier ordinaire à l'intérieur des dossiers connectés à la place
* Pour mettre ce fichier exact dans l'artifact, copiez-le dans l'un des dossiers connectés de la session en tant que fichier régulier, pas un lien symbolique, et demandez à nouveau

<h3 id="webfetch-cannot-fetch-localhost">
  WebFetch cannot fetch localhost
</h3>

Claude a appelé [WebFetch](/docs/fr/tools-reference#webfetch-tool-behavior) avec une URL dont le nom d'hôte n'a pas de point, comme `http://localhost:3000` ou un nom intranet nu comme `http://wiki/`. WebFetch refuse ces URL avant de faire une demande :

```text wrap theme={null}
WebFetch cannot fetch localhost or other hostnames without a dot. To reach a local server, use Bash with curl instead.
```

**Que faire :**

* Généralement rien : le message pointe Claude vers `curl` à travers l'outil Bash, qui peut atteindre les serveurs locaux et intranet

Avant la v2.1.268, WebFetch signalait ces URL avec une erreur générique `Invalid URL`.

<h2 id="background-session-errors">
  Erreurs de session en arrière-plan
</h2>

Les [sessions en arrière-plan](/docs/fr/agent-view) s'exécutent sans terminal interactif qui leur soit propre, de sorte que les commandes qui en ont besoin se comportent différemment là-bas. Ces messages apparaissent dans la transcription d'une session en arrière-plan, dans le terminal qui s'y attache, dans la session ou le shell à partir duquel vous envoyez, ou, pour les [entrées worktree-guard](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) ci-dessous, dans toute session isolée dans un worktree ou exécutant un sous-agent isolé par worktree ; lorsqu'un message est spécifique à une surface, son entrée le précise.

<h3 id="commands-refused-in-a-background-session">
  Commandes refusées dans une session en arrière-plan
</h3>

Les commandes qui ouvrent un dialogue interactif ne peuvent pas le faire tant qu'aucun terminal n'est attaché à une session en arrière-plan. `/install-github-app`, la liste des paramètres `/mcp`, et les actions d'authentification dans le menu du serveur MCP répondent par un message, et la session apparaît sous **Needs input** dans la [vue agent](/docs/fr/agent-view) pour que vous puissiez la trouver, l'attacher et exécuter la commande à nouveau. Tant qu'un terminal est attaché, ces commandes fonctionnent normalement.

Avant v2.1.216, la session n'apparaissait pas sous **Needs input** après l'un de ces refus. Dans v2.1.213 à v2.1.215, les commandes fonctionnaient toujours tant qu'un terminal était attaché, et le message de refus vous disait d'attacher et d'exécuter la commande à nouveau. De v2.1.208 à v2.1.212, Claude Code les refusait même tant qu'un terminal était attaché, avec un message tel que `Can't open MCP settings in a background session` ; sur ces versions, exécutez la commande à partir d'une session `claude` ordinaire à la place, ou mettez à jour. Avant v2.1.208, ils ouvraient leur dialogue à l'intérieur de la session en arrière-plan. En v2.1.208 uniquement, Claude Code refusait également le sélecteur `/model` dans une session en arrière-plan, et `/upgrade` imprimait l'URL de mise à jour au lieu d'ouvrir un navigateur.

La formulation nomme la commande. La liste des paramètres `/mcp` rapporte :

```text theme={null}
Can't open MCP settings while no terminal is attached to this background session. This session now shows "needs input" in agent view — open it and run /mcp to manage servers, or use `/mcp enable|disable|reconnect <server>` to steer without the panel.
```

**Ce qu'il faut faire :**

* Attachez-vous à la session à partir de la vue agent, où elle est listée sous **Needs input**, et exécutez la commande à nouveau
* Ou utilisez le formulaire que le message nomme, tel que `/mcp reconnect <server>`, `/mcp enable`, ou `/mcp disable`, qui fonctionnent sans attacher

<h3 id="write-or-command-blocked-because-the-path-cannot-be-safely-resolved">
  Écriture ou commande bloquée car le chemin ne peut pas être résolu de manière sûre
</h3>

Claude a adressé un fichier ou un répertoire de travail par une orthographe que la [garde d'isolation worktree](/docs/fr/agent-view#how-file-edits-are-isolated) ne peut pas résoudre en un seul emplacement vérifiable. La garde vérifie les écritures et les répertoires de travail des commandes dans [toute session isolée dans un worktree](/docs/fr/worktrees#how-claude-code-enforces-isolation), interactive ou en arrière-plan, et dans les [sous-agents isolés par worktree](/docs/fr/worktrees#isolate-subagents-with-worktrees). Elle résout les liens symboliques avant de vérifier que l'opération n'atteint pas le checkout partagé, et lorsque la résolution échoue, elle bloque l'opération plutôt que de la laisser y atterrir. Le message nomme les formes de chemin qu'elle refuse et comment réessayer :

```text theme={null}
This write was blocked because the path is spelled in a form that cannot be safely resolved (for example through a symlink storing a raw dot segment, a network-share or device-namespace shape, or an unreadable ancestor directory). If the file is inside the worktree /path/to/worktree, address it by its direct symlink-free path instead.
```

Une commande bloquée rapporte la même cause pour son répertoire de travail et se termine par `re-run the command from its direct symlink-free path`. Avant v2.1.217, la garde comparait les orthographes de chemin sans résoudre les liens symboliques, de sorte que ces orthographes n'étaient pas bloquées et une écriture acheminée par un lien symbolique pouvait atterrir dans le checkout partagé.

**Ce qu'il faut faire :**

* Généralement rien : le message complet va à Claude comme une erreur d'outil, et Claude réessaye avec le chemin direct qu'il nomme. Pour une édition de fichier bloquée, la vue de conversation affiche uniquement une courte ligne `Error editing file` ; le message complet apparaît dans la vue de transcription, que vous ouvrez avec `Ctrl+O`. Une commande bloquée l'imprime dans sa sortie de commande.
* Si le bloc se répète sur le même fichier, le chemin passe probablement par un lien symbolique commis dont la cible contient `..`, tel que `docs/current -> ../README.md` ; demandez à Claude d'éditer le fichier cible par son chemin réel au lieu de passer par le lien

<h3 id="write-or-command-blocked-because-the-path-names-a-network-location">
  Écriture ou commande bloquée car le chemin nomme un emplacement réseau
</h3>

Claude a adressé un fichier ou un répertoire de travail par un chemin qui nomme un lecteur qui n'est pas sur votre machine, un partage UNC tel que `\\server\share\file` ou un chemin d'automontage `/net`, tandis que le checkout de la session est sur un disque local. La même [garde d'isolation worktree](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) ne peut pas vérifier qu'un tel chemin reste en dehors du checkout partagé, de sorte qu'elle bloque l'opération. L'isolation de la session dans un worktree ne lève pas le bloc. Le message nomme la forme de chemin à utiliser à la place :

```text theme={null}
This write was blocked because the path is network-shaped (a UNC share or /net automount spelling) while this session's checkout is local. Isolating cannot unblock it. If the file is genuinely inside the worktree /path/to/worktree, address it by its local, plainly-spelled path instead.
```

Une commande bloquée rapporte la même cause pour son répertoire de travail et se termine par `re-run the command from its local, plainly-spelled path`. Avant v2.1.217, la garde comparait uniquement le texte du chemin, de sorte que l'adressage d'un fichier à l'intérieur du checkout par un chemin UNC ou `/net` n'était pas bloqué.

**Ce qu'il faut faire :**

* Généralement rien : Claude réessaye avec l'orthographe locale que le message demande
* Si le fichier est sur un partage réseau plutôt qu'un fichier local orthographié avec un chemin réseau, il est en dehors de l'espace de travail local de la session ; éditez-le à partir d'une session interactive ordinaire à la place

<h3 id="command-blocked-by-the-worktree-isolation-checks">
  Commande bloquée par les vérifications d'isolation worktree
</h3>

Claude a exécuté une commande Bash ou Monitor dans une [session isolée dans un worktree](/docs/fr/worktrees#how-claude-code-enforces-isolation), et Claude Code l'a refusée pour l'une de deux raisons :

* La commande pointe git vers le checkout principal.
* Claude Code ne peut pas vérifier à partir du texte de la commande que tout git que la commande exécute reste à l'intérieur du worktree. Une commande qui ne nomme jamais git peut toujours être refusée pour cette raison, car l'expansion d'une indirection de variable telle que `${!name}` ou l'exécution d'une substitution de fonction Bash telle que `${ command; }` produit une valeur à l'exécution qui peut elle-même être une commande.

Le milieu du message nomme ce qui n'a pas pu être vérifié :

```text wrap theme={null}
This session is isolated in the worktree /path/to/worktree, but this command evaluates ${!x@P} arithmetically inside a construct too complex to verify, which can run a command hidden in a variable's value. Refusing to run it — a worktree-isolated session's git operations must target its own worktree. Split it into plain, separate commands and run them from /path/to/worktree.
```

**Ce qu'il faut faire :**

* Généralement rien : Claude lit le message et réécrit la commande de la manière que sa phrase finale demande
* Si une commande que vous avez demandée continue d'être refusée, orthographiez la valeur signalée littéralement : remplacez l'indirection ou la substitution par sa valeur, et exécutez git comme sa propre commande simple à partir de l'intérieur du worktree
* Pour agir sur le checkout principal à dessein, exécutez la commande vous-même dans un terminal en dehors de la session

<h3 id="this-session-has-no-saved-transcript">
  Cette session n'a pas de transcription enregistrée
</h3>

Vous vous êtes attaché à une [session en arrière-plan](/docs/fr/agent-view) arrêtée qui a été mise en arrière-plan à partir d'une autre conversation avec `←` ou `/background` et arrêtée avant que sa première réponse ne soit terminée. Jusqu'à ce que cette première réponse soit terminée, la conversation vit toujours uniquement dans la session à partir de laquelle elle a été mise en arrière-plan, de sorte que `claude attach` refuse de démarrer la session arrêtée plutôt que de commencer une conversation vierge sous le même ID de session. Le message se termine par la commande `claude respawn` pour cette session :

```text theme={null}
This session has no saved transcript — it was stopped before its first response finished. If it was backgrounded from another conversation, that one is still intact; `claude respawn <id>` starts this one fresh.
```

L'ouverture de la même ligne de session dans la [vue agent](/docs/fr/agent-view) affiche `Press enter again to restart this session fresh` sous la liste à la place, et une deuxième `Entrée` sur la ligne redémarre la session avec une conversation vide. Avant v2.1.212, l'ouverture de la session arrêtée affichait le message de refus sans aucun moyen de redémarrer à partir de la vue agent. Avant v2.1.211, l'ouverture de la session arrêtée démarrait silencieusement cette conversation vierge et pouvait réexécuter l'invite originale de la session.

**Ce qu'il faut faire :**

* La conversation que vous avez mise en arrière-plan est intacte : reprenez-la avec [`claude --resume`](/docs/fr/sessions) ou continuez à travailler dedans
* Pour démarrer la session arrêtée à nouveau, exécutez `claude respawn <id>` avec l'ID du message, ou appuyez deux fois sur `Entrée` sur sa ligne dans la vue agent
* Si la session a terminé une réponse et vous voyez toujours ce refus sur une version antérieure à v2.1.214, un dossier illisible dans `~/.claude/projects` pourrait faire manquer à l'analyse de transcription la conversation enregistrée ; mettez à jour vers v2.1.214 ou ultérieur, qui tolère les dossiers illisibles lors de l'analyse

<h3 id="this-session-is-running-in-another-terminal">
  Cette session s'exécute dans un autre terminal
</h3>

Vous avez ouvert la ligne d'une session arrêtée dans la [vue agent](/docs/fr/agent-view), et sa conversation enregistrée est déjà ouverte dans un autre processus Claude Code en direct sur cette machine, de sorte que Claude Code refuse de démarrer un deuxième processus qui écrirait dans la même transcription. Le message que vous voyez dépend de [ce qui détient la conversation](/docs/fr/agent-view#opening-a-session-says-the-conversation-is-already-open) :

```text theme={null}
Can't open — this session is running in another terminal
This conversation is already open in another running Claude session — use that one, or close it and try again
```

* **`running in another terminal`** : un terminal détient la conversation, par exemple celui où vous l'avez reprise avec `claude --resume` ou `/resume`. La ligne affiche également `Open in a terminal`.
* **`already open in another running Claude session`** : un autre processus Claude Code non interactif la détient, par exemple un processus de [session en arrière-plan](/docs/fr/agent-view#the-supervisor-process) pour la même conversation qui n'a pas encore quitté.

Claude Code enregistre une réponse que vous avez tapée lors de l'ouverture de la ligne et l'envoie comme l'invite suivante de la session lorsque la session démarre ensuite.

**Ce qu'il faut faire :**

* Continuez la conversation dans le processus qui la détient, ou quittez ce processus et ouvrez la ligne à nouveau

Avant v2.1.248, seul le refus `already open in another running Claude session` existait : une conversation reprise dans un terminal ne comptait pas comme ouverte, et l'ouverture de la ligne démarrait un deuxième processus Claude Code écrivant dans la même conversation.

<h3 id="this-sessions-saved-conversation-is-no-longer-on-disk">
  La conversation enregistrée de cette session n'est plus sur le disque
</h3>

Vous avez ouvert une [session en arrière-plan](/docs/fr/agent-view) qui s'est terminée alors que le service en arrière-plan était arrêté, et le [nettoyage de transcription](/docs/fr/settings-reference#cleanupperioddays) a depuis supprimé sa conversation enregistrée, par exemple après que la machine ait été éteinte pendant des semaines. L'ouverture d'une telle ligne [reprend normalement sa conversation enregistrée](/docs/fr/agent-view#sessions-show-as-failed-after-shutdown). Sans rien à reprendre, Claude Code refuse plutôt que de réexécuter l'invite originale de la session sans demander :

```text theme={null}
This session's saved conversation is no longer on disk (it ended while the background service was off, and old transcripts are cleaned up), so there is nothing to resume. `claude rm 7c5dcf5d` deletes the row; `claude respawn 7c5dcf5d` runs its original prompt again instead.
```

`claude attach <id>` imprime ce texte. Dans la vue agent, le pied de page est plus court et se termine par `ctrl+x deletes the row`.

**Ce qu'il faut faire :**

* Exécutez `claude rm <id>` pour supprimer la ligne. Lorsque l'un des [cas conservés](/docs/fr/agent-view#what-deleting-a-session-removes) s'applique, `claude rm` conserve la ligne et le worktree à la place et nomme la raison
* Pour exécuter à nouveau l'invite originale de la session en tant que conversation nouvelle, exécutez `claude respawn <id>`

Avant v2.1.248, l'ouverture d'une telle ligne réexécutait l'invite originale de la session au lieu de refuser, ramenant une tâche vieille de plusieurs semaines au premier plan.

<h3 id="worktree-has-commits-that-are-not-pushed-anywhere">
  Le worktree a des commits qui ne sont poussés nulle part
</h3>

Vous avez essayé de supprimer une [session en arrière-plan](/docs/fr/agent-view#what-deleting-a-session-removes) dont le worktree contient des commits que Claude Code ne peut pas confirmer sont enregistrés ailleurs. Claude Code conserve le worktree et la ligne de session plutôt que de détruire les commits sans les voir. `claude rm` nomme la branche et les commits non poussés, et dit comment procéder :

```text theme={null}
kept 7c5dcf5d — its worktree is still at "/home/you/project/.claude/worktrees/fix-login"
  2 unpushed commits on "claude/fix-login": a1b2c3d "Fix login flow" and 1 more. They exist on no remote, so deleting the worktree would lose them.
  push them and run 'claude rm 7c5dcf5d' again, or discard the worktree and its commits: claude rm 7c5dcf5d --discard-unpushed a1b2c3d000000000000000000000000000000000@0123456789abcdef0123456789abcdef
```

Lorsque Claude Code ne peut pas résumer les commits, la ligne de détail lit `The worktree has unpushed commits` à la place. Dans la [vue agent](/docs/fr/agent-view), la ligne de la session affiche `not deleted` avec la même raison.

Les commits sur une télécommande ne bloquent pas la suppression. Non plus les commits sur la copie locale de la branche par défaut de votre télécommande `origin`, tant que cette branche est extraite dans votre checkout principal, le répertoire du référentiel lui-même plutôt qu'un worktree.

**Ce qu'il faut faire :**

* Pour conserver les commits, poussez la branche du worktree, ou fusionnez-la dans la branche par défaut extraite dans votre checkout principal, puis supprimez la session à nouveau
* Pour abandonner les commits, exécutez la commande `claude rm <id> --discard-unpushed` que le message a imprimée, ou appuyez deux fois sur `Ctrl+X` sur la ligne de la session dans la vue agent à nouveau. Cela supprime la session et le worktree ainsi que sa branche, les commits non poussés et les modifications non validées. Si le worktree a gagné un commit depuis le refus, Claude Code le conserve à nouveau et affiche l'état mis à jour
* Lorsque le message dit que le worktree est également enregistré par une autre session terminée, la suppression à nouveau ne le supprime pas : poussez les commits, puis supprimez la session à nouveau

Avant v2.1.268, `claude rm` mettait le résumé du commit sur la ligne `kept` elle-même. Lorsque `claude rm` ne pouvait pas résumer les commits, la ligne `kept` lisait `worktree has commits that are not pushed anywhere` à la place du résumé.

Avant v2.1.260, le message ne nommait pas la branche ou les commits, et la suppression à nouveau était refusée de la même manière : supprimer la session sans pousser signifiait supprimer le worktree vous-même avec `git worktree remove --force <path>`, puis exécuter `claude rm <id>` à nouveau.

Avant v2.1.248, la branche par défaut extraite dans votre checkout principal ne comptait pas : une branche que vous aviez déjà fusionnée là-bas déclenchait toujours ce refus jusqu'à ce que ses commits atteignent une télécommande.

<h3 id="terminal-host-process-died">
  Le processus hôte du terminal est mort
</h3>

Chaque terminal de [session en arrière-plan](/docs/fr/agent-view) s'exécute dans un processus hôte sous le service en arrière-plan, et ce processus est mort alors que le service maintenait toujours sa connexion, de sorte que la session n'a pas pu être atteinte.

Sur Linux et WSL, le service en arrière-plan vérifie chaque processus hôte toutes les quelques secondes, marque la session comme échouée lorsque le processus a quitté mais sa connexion au service ne s'est jamais fermée, et affiche la raison sur sa ligne dans la [vue agent](/docs/fr/agent-view#read-session-state) :

```text theme={null}
terminal host process died — press Enter to restart
```

Si vous ouvrez la ligne avant que la vérification ne s'exécute, le pied de page affiche `This session's terminal host process died (the conversation is saved) — press Enter to restart it` et la ligne devient échouée.

À partir du shell, `claude attach <id>` redémarre une session déjà marquée comme échouée pour un hôte mort, et sinon imprime la cause et quitte :

```text theme={null}
Couldn't attach to <id> — This session's terminal host process died (the conversation is saved) — run `claude attach <id>` again to restart it on a fresh host.
```

La conversation est enregistrée de toute façon.

Une ligne exécutant une [commande shell](/docs/fr/agent-view#run-a-shell-command) à la place affiche `terminal host process died — its output is gone; the command was not run again`, et `claude attach` imprime `This command's terminal host process died — its output is gone and the command was not run again`. Claude Code ne réexécute jamais la commande pour vous.

**Ce qu'il faut faire :**

* Dans la vue agent, appuyez sur `Entrée` sur la ligne échouée ; la session redémarre sur un processus hôte nouveau et la conversation reprend
* À partir du shell, exécutez `claude attach <id>` à nouveau. Claude Code imprime `Session <id>'s terminal host died — restarting it on a fresh one…` et rouvre la session
* Vous ne pouvez pas redémarrer une ligne de commande shell de cette manière ; envoyez la commande à nouveau pour la réexécuter

Avant v2.1.247, un processus hôte mort pouvait passer chaque vérification de vivacité que le service en arrière-plan exécutait, de sorte que l'ouverture de la session affichait `opening… · esc to cancel` indéfiniment et `claude attach <id>` attendait sans signaler une erreur.

<h3 id="session-isnt-responding">
  La session ne répond pas
</h3>

Vous avez ouvert une [session en arrière-plan](/docs/fr/agent-view) et le service en arrière-plan a accepté l'ouverture, mais aucune sortie n'est arrivée pendant environ dix secondes, de sorte que Claude Code conclut que le processus relayant le terminal de la session ne peut pas fournir de sortie, et termine la tentative au lieu d'attendre.

Dans la vue agent, Claude Code propose un redémarrage dans le pied de page :

```text theme={null}
Press enter again to restart this session — it isn't responding (its conversation is saved and resumes).
```

À partir du shell, `claude attach <id>` imprime la cause et quitte :

```text theme={null}
Couldn't attach to <id> — Session isn't responding — `claude stop <id>`, then `claude attach <id>` restarts it (the conversation is saved).
```

Claude Code ne redémarre jamais une ligne exécutant une [commande shell](/docs/fr/agent-view#run-a-shell-command) pour vous, car un redémarrage réexécuterait la commande.

**Ce qu'il faut faire :**

* Dans la vue agent, appuyez sur `Entrée` sur la même ligne à nouveau. Claude Code arrête le processus qui ne répond pas et redémarre la session, et la conversation reprend. Rien n'est arrêté sans cette deuxième pression
* À partir du shell, exécutez `claude stop <id>`, puis `claude attach <id>`
* Pour une ligne de commande shell, appuyez sur `Ctrl+X` dans la vue agent ou exécutez `claude stop <id>` pour l'arrêter ; envoyez la commande à nouveau pour la réexécuter

<h3 id="session-was-stopped-while-the-respawn-was-in-flight">
  La session a été arrêtée pendant que le respawn était en vol
</h3>

Vous avez ouvert une [session en arrière-plan](/docs/fr/agent-view) dont le processus n'était pas en cours d'exécution, et tandis que Claude Code la redémarrait, un autre processus Claude Code l'a arrêtée, par exemple `claude stop` dans un autre terminal. Claude Code garde la session arrêtée :

```text theme={null}
Session <id> was stopped while the respawn was in flight
```

L'ouverture d'une session que vous venez de dispatcher, tandis que son processus démarre toujours, attend le processus à la place. Avant v2.1.246, l'ouverture à ce moment-là pouvait l'arrêter et afficher ce message.

**Ce qu'il faut faire :**

* Si vous n'avez pas arrêté la session, ouvrez sa ligne à nouveau dans la vue agent ou exécutez `claude respawn <id>` pour la redémarrer
* Si vous l'avez arrêtée vous-même, rien ne reste à faire : la session reste arrêtée

<h3 id="session-agent-no-longer-available">
  L'agent de session n'est plus disponible
</h3>

Vous avez repris une session qui exécutait un [agent personnalisé](/docs/fr/sub-agents#invoke-subagents-explicitly), démarré avec `--agent` ou le paramètre `agent`, et Claude Code n'a pas trouvé d'agent portant ce nom. Il recherche d'abord le répertoire original de la session, lorsque vous avez [approuvé cet espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust), puis le répertoire à partir duquel vous reprenez. La session reprend toujours, mais avec les outils par défaut, de sorte que les restrictions d'outils de l'agent ne s'appliquent plus :

```text theme={null}
This session was running agent 'code-reviewer', which is no longer available (no agent by that name in /home/you/project). Continuing with the default tools and system prompt — the agent's tool restrictions no longer apply. To restore it, re-create the agent, or resume with an explicit --agent <name>.
```

L'avertissement nomme uniquement les répertoires que Claude Code a recherchés, et il apparaît dans la conversation reprise que vous réveilliez une [session en arrière-plan](/docs/fr/agent-view), exécutiez `/resume` ou `claude --resume`, ou repreniez en [mode non interactif](/docs/fr/headless), où il va également à stderr. Les sessions utilisant `--input-format stream-json` ne l'affichent pas, car le SDK Agent fournit les agents après le démarrage.

Claude Code n'enregistre pas le repli à la session, de sorte que l'avertissement se répète à chaque reprise jusqu'à ce que vous agissiez. L'agent `claude` intégré ne déclenche pas l'avertissement, puisque le repli à l'ensemble d'outils par défaut ne change rien pour lui. Avant v2.1.216, Claude Code continuait silencieusement en tant qu'agent par défaut, et la recherche couvrait uniquement le répertoire à partir duquel vous repreniez, de sorte qu'un agent limité au projet était perdu à chaque reprise à partir d'un autre répertoire.

**Ce qu'il faut faire :**

* Recréez le fichier d'agent à `.claude/agents/<name>.md` dans le projet de la session, ou à `~/.claude/agents/<name>.md` pour un agent personnel, puis reprenez à nouveau
* Ou reprenez avec `--agent <name>` nommant un agent qui existe, pour exécuter la session en tant que cet agent à la place
* Si l'agent est limité au projet et vous n'avez pas approuvé le répertoire original de la session, exécutez Claude Code là une fois, acceptez le dialogue de confiance, puis reprenez à nouveau

<h3 id="claude_code_process_wrapper-launcher-errors">
  Erreurs du lanceur CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/fr/corporate-launcher) est défini, et sa valeur ne peut pas être utilisée, de sorte que Claude Code refuse de démarrer le processus affecté plutôt que de l'exécuter sans le lanceur. Les problèmes de configuration sont signalés avec un message qui commence par le nom de la variable et énonce la raison, par exemple :

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Un lanceur qui démarre mais quitte sans se remplacer par Claude Code échoue la session qu'il démarrait, et la ligne de la session dans la vue agent rapporte que le lanceur `must exec, not daemonize`, suivi de tout ce que le lanceur a imprimé. Une session qui ne peut pas démarrer ou atteindre le service en arrière-plan à cause du lanceur rapporte le problème du lanceur comme la raison à l'intérieur de `Couldn't reach the background service (...)`.

**Ce qu'il faut faire :**

* Définissez la variable sur le chemin absolu d'un exécutable qui se termine en appelant `exec "$@"`. Voir [le contrat du lanceur](/docs/fr/corporate-launcher#the-launcher-contract) pour le contrat complet
* Vérifiez `/status`, qui affiche la commande de lancement résolue dans son entrée Self-exec et avertit lorsque le service en arrière-plan en cours d'exécution ne correspond pas, ou exécutez `claude daemon status` à partir d'un shell
* Après avoir corrigé la valeur dans le bloc `env` des [paramètres](/docs/fr/corporate-launcher#set-up-the-launcher), redémarrez le service en arrière-plan avec `claude daemon stop --any` de sorte que le prochain envoi démarre un service enveloppé

<h3 id="eunknown-when-starting-a-background-session">
  EUNKNOWN au démarrage d'une session en arrière-plan
</h3>

Windows a refusé de démarrer un programme avec un code d'erreur qui n'a pas de nom standard, de sorte que l'échec apparaît comme `EUNKNOWN`. Le déclencheur habituel est une politique de restriction logicielle, telle que Group Policy ou AppLocker, bloquant le programme en cours de démarrage. L'erreur apparaît lorsque vous démarrez une [session en arrière-plan](/docs/fr/agent-view) avec `/background` ou `claude --bg` :

```text theme={null}
Couldn't reach the background service (spawn background service: EUNKNOWN: unknown error, uv_spawn) — run 'claude daemon status'
```

Sur certains comptes, le message dit `daemon` à la place de `background service`.

Sur une installation npm, un `EUNKNOWN` qui apparaît tandis que `npm install -g @anthropic-ai/claude-code` remplace le binaire a la même cause que [`EACCES` lors d'une réinstallation](#eacces-when-starting-a-background-session) et s'efface lorsque vous réessayez après la fin de l'installation.

Claude Code démarre le service en arrière-plan via PowerShell de sorte que le service survive à la fermeture du terminal, en utilisant PowerShell 7 lorsqu'il est installé et Windows PowerShell 5.1 sinon. Lorsqu'aucun PowerShell ne peut s'exécuter, Claude Code démarre le service directement à la place, de sorte qu'une politique qui bloque uniquement PowerShell ne cause pas cette erreur. Si vous la voyez tandis qu'aucune installation npm n'est en cours, la politique bloque l'exécutable Claude Code lui-même.

Avant v2.1.212, Claude Code utilisait uniquement Windows PowerShell 5.1 pour démarrer le service, de sorte que toute machine où Group Policy bloquait PowerShell 5.1 échouait avec `Couldn't start the session — EUNKNOWN: unknown error, uv_spawn`, même avec PowerShell 7 installé.

**Ce qu'il faut faire :**

* Si le message lit `Couldn't start the session`, mettez à jour vers v2.1.212 ou ultérieur. Sur les versions antérieures, vous pouvez également exécuter `claude daemon run` dans un terminal séparé en premier, puis démarrer la session en arrière-plan à nouveau. Cette commande exécute le service en arrière-plan au premier plan du terminal, de sorte que le service dure uniquement tant que ce terminal reste ouvert.
* Si une installation npm remplaçait le binaire, attendez qu'elle se termine, puis démarrez la session en arrière-plan à nouveau
* Si l'erreur apparaît sur v2.1.212 ou ultérieur tandis qu'aucune installation npm n'est en cours, demandez à votre administrateur Windows d'autoriser l'exécutable Claude Code dans la politique de restriction
* Si le service en arrière-plan s'arrête lorsque vous fermez le terminal, Claude Code l'a démarré sans PowerShell. Installez PowerShell 7, ou demandez à votre administrateur de débloquer PowerShell, de sorte que le service puisse survivre au terminal.

<h3 id="eacces-when-starting-a-background-session">
  EACCES au démarrage d'une session en arrière-plan
</h3>

Claude Code n'a pas pu exécuter son propre binaire pour démarrer le [service en arrière-plan](/docs/fr/agent-view#the-supervisor-process) qui héberge les sessions en arrière-plan. Sur une installation npm, cela signifie généralement que `npm install -g @anthropic-ai/claude-code` remplaçait le binaire à ce moment-là, que vous l'ayez exécuté ou que le [mise à jour automatique](/docs/fr/setup#auto-updates) l'ait fait. L'erreur apparaît lorsque vous ouvrez une session à partir de la [vue agent](/docs/fr/agent-view) :

```text theme={null}
Couldn't start the background service — spawn background service: EACCES: permission denied, posix_spawn '/usr/local/lib/node_modules/@anthropic-ai/claude-code/bin/claude'
```

Lorsque vous démarrez une session avec `/background` ou `claude --bg`, la même raison apparaît à l'intérieur de `Couldn't reach the background service (...)`. Pendant la même fenêtre de réinstallation, l'erreur peut nommer un autre code à la place, tel que `ENOENT` ou `ENOEXEC`, ou `EUNKNOWN` ou `EPERM` sur Windows ; un `EUNKNOWN` qui persiste à travers les tentatives a une [cause différente](#eunknown-when-starting-a-background-session).

Sur une installation npm, Claude Code attend que la réinstallation se termine et réessaye de lui-même : jusqu'à dix secondes, et jusqu'à deux minutes tandis qu'une installation npm de Claude Code est visiblement toujours en cours sur la machine, ce qui couvre un autre processus Claude Code téléchargeant une mise à jour. Lorsque l'installation dépasse cette attente, l'échec nomme la mise à jour au lieu du code d'erreur nu :

```text theme={null}
Claude Code is being updated by npm on this machine (still not runnable after 2 min, EACCES) — try again when the update finishes
```

Avant v2.1.257, l'attente s'arrêtait à dix secondes dans tous les cas, de sorte que cette erreur apparaissait tandis qu'un autre processus Claude Code téléchargeait toujours une mise à jour. Avant v2.1.246, Claude Code échouait immédiatement, sans attendre.

**Ce qu'il faut faire :**

* Attendez quelques secondes, puis ouvrez la session ou envoyez à nouveau. Lorsque le message dit que Claude Code est en cours de mise à jour, réessayez après la fin de la mise à jour.
* Si l'erreur persiste tandis qu'aucune installation npm n'est en cours, votre utilisateur ne peut pas exécuter le binaire installé. Vérifiez ses permissions et celles de son répertoire, ou réinstallez Claude Code.

<h3 id="background-service-exited-before-it-became-reachable">
  Le service en arrière-plan a quitté avant de devenir accessible
</h3>

Le processus que Claude Code a démarré en tant que [service en arrière-plan](/docs/fr/agent-view#the-supervisor-process) a quitté avant d'accepter les connexions, de sorte que Claude Code n'a pas pu ouvrir votre session. Lorsque le service a imprimé une erreur avant de quitter, la raison entre parenthèses donne le code de sortie ou le signal et la première ligne que le service a imprimée, qui nomme ce qui l'a arrêté :

```text theme={null}
Couldn't reach the background service (background service exited before it became reachable (exit code N): <the service's first error line>) — run 'claude daemon status'
```

Lorsque vous ouvrez une session à partir de la [vue agent](/docs/fr/agent-view), la même raison suit `Couldn't start the background service —`. Lorsque le service n'a rien imprimé avant de quitter, le message dit `nothing on stderr` à la place.

Claude Code rapporte l'échec avec la ligne d'erreur du service. Avant v2.1.246, l'échec n'apparaissait qu'après une attente de 45 secondes, comme `background service did not become reachable within 45s`, sans la ligne d'erreur du service.

Deux raisons citées ont des causes connues :

* `Error: claude native binary not installed.` : une installation npm remplaçait le binaire Claude Code à ce moment-là, de sorte que le service exécutait l'espace réservé npm à la place. Réessayez après la fin de l'installation ; si la ligne persiste sans installation en cours, [complétez l'installation npm](/docs/fr/troubleshoot-install#native-binary-not-found-after-npm-install). Avant v2.1.257, une auto-mise à jour npm macOS produisait cet échec à chaque démarrage pendant la fenêtre d'installation.
* `nothing on stderr` avec le code de sortie 1, à chaque démarrage, sur Windows : `daemon.lock` nomme un processus que Claude Code ne peut ni signaler ni prouver qu'il est parti, de sorte que chaque nouveau service conclut qu'un autre détient le verrou et quitte. Un verrou dont l'auteur Claude Code peut prouver qu'il est parti est remplacé de lui-même et ne produit pas cet échec. Lorsque l'échec se répète à chaque démarrage, supprimez `~/.claude/daemon.lock`, puis ouvrez la session ou envoyez à nouveau. Avant v2.1.257, un tel verrou bloquait chaque démarrage jusqu'à ce que vous supprimiez le fichier.

**Ce qu'il faut faire :**

* Si le message cite une ligne, corrigez ce qu'elle nomme, puis ouvrez la session ou envoyez à nouveau. La tentative suivante démarre le service à nouveau
* Exécutez `claude daemon status` pour vérifier si un service s'exécute maintenant

<h3 id="working-directory-no-longer-exists-when-starting-a-background-session">
  Le répertoire de travail n'existe plus au démarrage d'une session en arrière-plan
</h3>

Vous avez essayé de démarrer une [session en arrière-plan](/docs/fr/agent-view) dans un répertoire qui n'existe plus. Cela se produit lorsque vous envoyez à partir de la vue agent ou exécutez `/background` après que le répertoire dans lequel vous travailliez ait été supprimé ou déplacé. Cela se produit également lorsque vous vous attachez à ou redémarrez une session dont le processus a quitté et dont le répertoire est parti, car le nouveau processus démarrerait dans ce même répertoire. Claude Code ne démarre pas la session, et le message nomme le répertoire manquant :

```text theme={null}
Couldn't start a background session (working directory no longer exists or is not accessible: /tmp/demo)
```

Avant v2.1.257, la session semblait démarrer puis s'affichait dans la vue agent comme une ligne échouée avec la même raison.

**Ce qu'il faut faire :**

* Recréez le répertoire que le message nomme, ou envoyez à partir d'un répertoire qui existe, puis réessayez

<h2 id="wrapper-and-ide-errors">
  Erreurs du wrapper et de l'IDE
</h2>

Ces erreurs proviennent du programme qui a lancé Claude Code pour vous, comme une extension IDE ou une application [Agent SDK](/docs/fr/agent-sdk/overview), plutôt que de Claude Code lui-même.

<h3 id="claude-code-process-exited-with-code-n">
  Le processus Claude Code s'est fermé avec le code N
</h3>

Le processus `claude` sous-jacent s'est fermé avec un code non nul. Le code de sortie seul ne dit pas ce qui a échoué : l'erreur réelle se trouve dans la sortie du processus lui-même, que le wrapper ajoute s'il en a capturé une, sinon il la conserve dans ses journaux.

```text theme={null}
Error: Claude Code process exited with code 1
```

Sur Windows, la version native peut se fermer avec le code `4294967295` juste après la fin d'un tour. Quand cette sortie se produit à une limite de tour, sans message en attente et sans tâche de fond en cours d'exécution, l'[extension VS Code](/docs/fr/vs-code) ferme la session silencieusement au lieu d'afficher cette erreur. Votre message suivant reprend la conversation.

Avant la v2.1.273, l'extension affichait l'erreur pour cette sortie à chaque limite de tour, même si rien n'était perdu.

**Ce qu'il faut faire :**

* Dans VS Code, suivez le lien **View output logs** affiché avec l'erreur pour voir l'échec sous-jacent
* Dans une application Agent SDK, capturez l'erreur autour de votre boucle de messages. Les entrées sous [CLI process exit](/docs/fr/agent-sdk/troubleshooting#cli-process-exit) couvrent ce que votre code reçoit dans chaque langage SDK.
* Exécutez `claude` dans un terminal dans le même projet. L'échec se reproduit généralement là avec son vrai message d'erreur, que vous pouvez ensuite rechercher sur cette page.
* Exécutez `claude doctor` dans un terminal pour vérifier l'installation et la configuration

<h3 id="could-not-locate-the-claude-cli-on-path">
  Impossible de localiser la CLI Claude sur PATH
</h3>

L'[extension VS Code](/docs/fr/vs-code) affiche cette erreur sur Windows quand vous ouvrez Claude Code dans le terminal intégré, le shell du terminal est PowerShell, et l'extension ne peut pas trouver l'exécutable `claude` installé sur PATH. L'extension refuse de lancer Claude Code jusqu'à ce qu'elle trouve le `claude` installé sur PATH.

```text theme={null}
Failed to run Claude Code: Error: Could not locate the Claude CLI on PATH. Launching by name in a PowerShell terminal would run a 'claude' from the open folder instead of the installed CLI, so the launch was blocked. Make sure the Claude CLI's install directory is on your system PATH (not only your PowerShell profile), then restart VS Code and try again. VS Code reads PATH when it starts, so PATH changes take effect only after a restart.
```

**Ce qu'il faut faire :**

* Ouvrez une nouvelle fenêtre PowerShell en dehors de VS Code et exécutez `where.exe claude`. Si elle n'affiche pas de chemin, la CLI n'est pas sur votre PATH : ajoutez son répertoire d'installation en suivant [Verify your PATH](/docs/fr/troubleshoot-install#verify-your-path). Si elle affiche un chemin, l'entrée provient de votre profil PowerShell ou d'une modification de PATH que VS Code n'a pas encore détectée ; les deux étapes suivantes couvrent ces cas.
* Définissez l'entrée PATH comme variable d'environnement utilisateur ou système, pas dans votre profil PowerShell. L'extension n'exécute pas votre profil, donc une modification de PATH qui ne se trouve que là ne l'atteint jamais.
* Redémarrez VS Code après avoir modifié PATH. L'extension vérifie le PATH que VS Code a capturé au démarrage, donc une modification de PATH ne prend effet qu'après un redémarrage.

<h3 id="the-connection-to-claude-code-ended-before-this-message-completed">
  La connexion à Claude Code s'est terminée avant la fin de ce message
</h3>

L'[extension VS Code](/docs/fr/vs-code) a envoyé votre message au processus `claude`, et la connexion s'est terminée sans erreur avant que le processus l'acknowledge ou le termine. L'extension ne peut pas dire si le message a été traité, donc elle vous demande de l'envoyer à nouveau :

```text theme={null}
The connection to Claude Code ended before this message completed — it may not have been processed, so please send it again.
```

**Ce qu'il faut faire :**

* Envoyez le message à nouveau. Le message suivant démarre un nouveau processus `claude` qui reprend la conversation.
* Si cela se répète, exécutez `claude` dans un terminal dans le même projet. Un échec qui continue à terminer le processus se reproduit généralement là avec son vrai message d'erreur.

<h2 id="rewind-warnings-and-errors">
  Avertissements et erreurs de rembobinage
</h2>

Ces messages proviennent d'une restauration de code [`/rewind`](/docs/fr/checkpointing). « Restored the code, but skipped N files » est un avertissement indiquant que Claude Code a ignoré certains chemins. « No files were restored » est une erreur signifiant qu'aucun fichier n'a été restauré.

<h3 id="restored-the-code-but-skipped-files">
  Restored the code, but skipped files
</h3>

Une restauration de code `/rewind` a ignoré un ou plusieurs chemins suivis au lieu de les écrire ou de les supprimer. Claude Code ignore un chemin quand :

* il est, ou est devenu, un lien symbolique, un lien physique, ou un autre fichier non régulier
* son répertoire a changé depuis le point de contrôle
* sa sauvegarde ne peut pas être lue en toute sécurité

Les chemins ignorés conservent leur contenu actuel. Avant la v2.1.216, `/rewind` écrivait et supprimait à travers les liens aux chemins suivis, et ne signalait pas une restauration partielle.

```text theme={null}
Restored the code, but skipped 2 files: the tracked path is (or became) a link or other non-regular file, its directory changed since the checkpoint, or its backup could not be safely read. Skipped files were left untouched — run with --debug for the paths.
```

**Que faire :**

* Identifiez les fichiers qui ont été ignorés pour pouvoir traiter chacun d'eux avec les étapes ci-dessous. Le message ne donne qu'un nombre ; le journal de débogage à `~/.claude/debug/<session-id>.txt` nomme chaque chemin ignoré lors de l'exécution de la restauration, donc activez la journalisation de débogage avec `/debug` avant votre prochain rembobinage. Sur macOS ou Linux, vous pouvez plutôt trouver les liens directement : `find . -type l` pour les liens symboliques et `find . -type f -links +1` pour les fichiers liés physiquement.
* Si un fichier ignoré est un lien que vous avez créé intentionnellement, comme un fichier de configuration géré par un gestionnaire de dotfiles ou un fichier lié physiquement par des outils comme pnpm, le rembobinage a laissé son contenu intact. Pour annuler les modifications de la session, demandez à Claude d'inverser la modification ou modifiez le fichier vous-même
* Si vous n'avez pas créé le lien, inspectez le chemin avant de faire confiance à son contenu : quelque chose a remplacé le fichier après le point de contrôle

<h3 id="no-files-were-restored">
  No files were restored
</h3>

Claude Code affiche ce message quand vous restaurez du code avec [`/rewind`](/docs/fr/checkpointing) et qu'il ne peut restaurer aucun des fichiers de ce point de contrôle. Pour chaque fichier, soit la sauvegarde que Claude Code a enregistrée avant de le modifier est manquante, soit Claude Code n'a pas pu écrire ou supprimer le fichier.

```text theme={null}
Failed to restore the code:
No files were restored: 1 file failed (backup missing, or the file could not be updated)
```

Claude Code supprime les sauvegardes d'une session lors du [balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically), par défaut environ 30 jours après que la session en ait enregistré une pour la dernière fois. Si vous reprenez une session après cela, `/rewind` liste toujours ses points de contrôle, mais le rembobinage vers l'un d'eux peut échouer avec cette erreur. Si le message dit aussi « N paths were skipped for link safety », consultez [Restored the code, but skipped files](#restored-the-code-but-skipped-files) pour ces chemins.

Quand vous créez une branche d'une session, par exemple avec [`--fork-session`](/docs/fr/cli-reference#cli-flags) ou [`/branch`](/docs/fr/sessions#branch-a-session), Claude Code copie les sauvegardes de la session d'origine dans la branche. Quand Claude Code ne peut pas copier une sauvegarde, par exemple parce que le disque est plein, cette sauvegarde est manquante dans la branche. Le rembobinage vers un point de contrôle qui en a besoin peut échouer avec cette erreur.

**Que faire :**

* Annulez les modifications d'une autre manière : demandez à Claude d'inverser ses modifications, ou restaurez les fichiers à partir du contrôle de version. Quand les sauvegardes sont parties, exécuter `/rewind` à nouveau échoue de la même manière.
* Si Claude Code n'a pas pu écrire ou supprimer un fichier, corrigez ce qui bloque l'écriture, comme les permissions de fichier, puis exécutez `/rewind` à nouveau.
* Pour conserver les sauvegardes plus longtemps dans les futures sessions, augmentez [`cleanupPeriodDays`](/docs/fr/settings-reference#cleanupperioddays).

Avant la v2.1.260, Claude Code ignorait silencieusement les fichiers dont les sauvegardes étaient manquantes, et le rembobinage semblait réussir.

<h2 id="session-saving-warnings">
  Avertissements d'enregistrement de session
</h2>

Claude Code affiche ces avertissements sur une ligne persistante sous la zone de saisie lorsqu'il n'enregistre pas votre transcription de session. La session continue de fonctionner de toute façon ; les avertissements vous indiquent que la session peut être manquante lors d'une utilisation ultérieure de [`--resume`](/docs/fr/sessions).

<h3 id="transcript-writes-are-failing">
  Les écritures de transcription échouent
</h3>

Claude Code enregistre la transcription sur le disque au fur et à mesure que vous travaillez, et ses écritures dans [le fichier de transcription](/docs/fr/sessions#where-transcripts-are-stored) échouent. Le message nomme la cause avec le code d'erreur sous-jacent, par exemple un disque plein :

```text theme={null}
Transcript writes are failing (disk full — ENOSPC) · recent messages may not be saved for resume
```

L'avertissement apparaît à différents moments selon l'erreur :

* À la première défaillance pour les conditions qui ne s'effacent pas d'elles-mêmes : un disque plein, un quota de disque dépassé, un système de fichiers en lecture seule, un chemin dépassant la limite de longueur du système de fichiers, ou, sur macOS et Linux, une erreur de permission
* Après des défaillances répétées s'étendant sur au moins une minute pour tout le reste, y compris les erreurs de permission sur Windows, où une analyse antivirus peut échouer une seule écriture qui réussit ensuite à la nouvelle tentative

Avant la v2.1.217, Claude Code supprimait les écritures défaillantes sans avertissement, et une `--resume` ultérieure manquant de messages récents était le premier signe.

**Que faire :**

* Corrigez la condition que le code d'erreur nomme : libérez l'espace disque pour `ENOSPC` ; augmentez ou effacez le quota pour `EDQUOT` ; restaurez l'accès en écriture à l'emplacement de la transcription pour `EACCES`, `EPERM`, ou `EROFS`
* L'avertissement s'efface de lui-même à la prochaine écriture réussie ; aucun redémarrage n'est nécessaire
* Les messages envoyés pendant que l'avertissement s'affichait peuvent toujours être manquants lorsque vous reprenez la session ultérieurement

<h3 id="transcript-saving-is-off-skip-prompt-history">
  L'enregistrement de la transcription est désactivé car CLAUDE\_CODE\_SKIP\_PROMPT\_HISTORY est défini
</h3>

Cette session a démarré avec [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/fr/env-vars) défini, donc Claude Code n'écrit aucune transcription ou historique d'invite pour celle-ci :

```text theme={null}
Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set · --resume will not find this session; if unintended, unset it and restart
```

La variable est une exclusion intentionnelle pour les sessions scriptées éphémères, mais elle peut également atteindre une session via un profil shell, un script wrapper, ou un processus parent qui l'a exportée.

**Que faire :**

* Si vous avez défini la variable intentionnellement, aucune action n'est nécessaire ; l'avis confirme que la session n'apparaîtra pas dans `--resume`, `--continue`, ou l'historique de la flèche vers le haut
* Si ce n'est pas le cas, supprimez la variable du shell ou du script qui lance `claude`, puis démarrez une nouvelle session. Les messages de la session actuelle ne sont pas enregistrés rétroactivement.

<h3 id="transcript-saving-is-off-child-session-marker">
  L'enregistrement de la transcription est désactivé en raison d'un marqueur CLAUDE\_CODE\_CHILD\_SESSION hérité
</h3>

Claude Code définit [`CLAUDE_CODE_CHILD_SESSION`](/docs/fr/env-vars) dans les sous-processus qu'il génère, et traite une session interactive qui l'hérite comme imbriquée : Claude Code n'enregistre aucune transcription pour celle-ci, donc les sessions que Claude lui-même démarre ne remplissent pas votre liste `--resume`. Cet avis signifie que votre session actuelle a hérité du marqueur :

```text theme={null}
Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker · restart with CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1 to keep future transcripts
```

L'avis est attendu lorsque vous avez exécuté `claude` depuis l'intérieur d'une autre session Claude Code ; il signale une mauvaise classification lorsque le marqueur a fui à travers un intermédiaire de longue durée, par exemple un terminal, une session `screen`, ou un lanceur qu'une session Claude Code a initialement démarré.

À l'intérieur de tmux, Claude Code détecte un marqueur qui est arrivé via l'environnement global du serveur tmux et continue d'enregistrer, donc cet avis n'apparaît pas pour ce cas.

**Que faire :**

* Si vous avez démarré cette session depuis l'intérieur d'une autre session Claude Code intentionnellement, aucune action n'est nécessaire
* Si c'est une session de niveau supérieur, quittez et redémarrez avec [`CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1`](/docs/fr/env-vars) défini. L'enregistrement s'applique à partir du redémarrage, donc les messages envoyés avant celui-ci ne sont pas enregistrés.
* Pour corriger les lancements futurs depuis le même terminal ou lanceur, supprimez `CLAUDE_CODE_CHILD_SESSION` de son environnement

<h2 id="configuration-warnings">
  Avertissements de configuration
</h2>

Claude Code écrit la plupart de ces messages sur stderr, et non dans la conversation, et les écrit principalement au démarrage. Une entrée le précise quand son message apparaît ailleurs, par exemple dans le journal de débogage ou comme un avis de démarrage dans la vue de conversation, ou à un autre moment, par exemple la [ligne de diagnostic de modèle non reconnu](#unrecognized-model-id-on-a-request) au moment de la requête.

<h3 id="fullscreen-failed-start-notice">
  Le rendu en plein écran n'a pas terminé le démarrage
</h3>

Une session [plein écran](/docs/fr/fullscreen) précédente sur cette machine s'est fermée avant de terminer le démarrage, donc Claude Code démarre cette session sur le rendu classique et imprime l'un de ces avis :

```text theme={null}
Claude Code's fullscreen renderer didn't finish starting last time on this machine, so this launch is using the classic renderer. It will try fullscreen again next launch; /tui default keeps the classic renderer.

Claude Code's fullscreen renderer has repeatedly failed to start on this machine, so it has been turned off here. Run /tui fullscreen to try it again (this also resets after an update).
```

**À faire :**

* Suivez [Rendu en plein écran](/docs/fr/fullscreen#fullscreen-renderer-didnt-finish-starting). Cela indique quel avis vous recevez, ce que Claude Code fait dans les sessions ultérieures, et comment réessayer le plein écran ou conserver le rendu classique.
* Si la session qui s'est fermée a imprimé un message de sortie, consultez [Claude Code s'est fermé après une erreur d'interface irrécupérable](#exited-after-an-unrecoverable-interface-error) pour voir ce qu'elle nomme.

Avant la v2.1.236, Claude Code n'imprimait aucun avis et continuait à démarrer les sessions en rendu plein écran après un démarrage échoué.

<h3 id="exited-after-an-unrecoverable-interface-error">
  Claude Code s'est fermé après une erreur d'interface irrécupérable
</h3>

Claude Code imprime ce message quand il se ferme parce que son interface de terminal a rencontré une erreur dont elle ne peut pas se rétablir, dans l'un ou l'autre rendu. La deuxième phrase n'apparaît que si l'erreur s'est produite pendant le démarrage du rendu [plein écran](/docs/fr/fullscreen) :

```text theme={null}
Claude Code exited after an unrecoverable interface error (<error>). It happened while the fullscreen renderer was starting, so the next launch will use the classic renderer (CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1 forces that any time).
```

**À faire :**

* Redémarrez Claude Code. Pour reprendre la conversation, exécutez `claude --resume` dans le même répertoire.
* Si le message nomme le rendu plein écran, [Rendu en plein écran](/docs/fr/fullscreen#fullscreen-renderer-didnt-finish-starting) indique ce que le prochain lancement fait, ce qui dépend de la façon dont vous avez activé le plein écran, et comment réessayer le plein écran ou conserver le rendu classique.

Avant la v2.1.236, Claude Code se fermait sans imprimer de message après ce type d'erreur.

<h3 id="agent-descriptions-are-over-the-15000-token-limit">
  Les descriptions d'agent dépassent la limite de 15 000 jetons
</h3>

Claude Code affiche cet avertissement comme un avis de démarrage dans la vue de conversation plutôt que sur stderr. Les descriptions combinées de vos [sous-agents](/docs/fr/sub-agents), à l'exception des agents intégrés, dépassent 15 000 jetons selon l'estimation de Claude Code. Chaque agent compte son nom plus son frontmatter `description`. Claude Code charge chaque agent, que le total dépasse ou non la limite, donc l'avertissement ne change pas ce qui se charge.

```text theme={null}
Agent descriptions are over the 15.0k-token limit (~16.2k tokens) · ask Claude to trim agent descriptions in .claude/agents/
```

**À faire :**

* Raccourcissez le frontmatter `description` de vos fichiers d'agent, ou demandez à Claude de les raccourcir pour vous.
* Supprimez les fichiers d'agent que vous n'utilisez plus.

<h3 id="workspace-has-not-been-trusted">
  L'espace de travail n'a pas été approuvé
</h3>

Claude Code a trouvé des règles `permissions.allow` ou des entrées `permissions.additionalDirectories` dans le fichier `.claude/settings.json` ou `.claude/settings.local.json` du projet et ne les a pas appliquées, car [les règles d'autorisation des paramètres du projet nécessitent l'approbation de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust). Le nombre, le nom du paramètre et le fichier nommé dans le message varient selon votre configuration. Les règles `deny` et `ask` ne sont pas affectées.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**À faire :**

* Exécutez `claude` dans le répertoire et acceptez la boîte de dialogue d'approbation. [Règles d'autorisation du projet et approbation de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust) indique quel dossier cette acceptation couvre.
* En [mode non interactif](/docs/fr/headless) avec `-p`, aucune boîte de dialogue n'est affichée. Définissez l'entrée `hasTrustDialogAccepted` dans `~/.claude.json` en utilisant la clé `projects` exacte que le message imprime.
* Si le message nomme `.claude/settings.local.json` et que vous avez démarré Claude Code en dehors d'un dépôt git ou dans votre répertoire personnel, mettez à jour vers la v2.1.200 ou ultérieure. Les versions 2.1.196 à 2.1.199 ont traité votre propre `.claude/settings.local.json` comme fourni par le dépôt dans ces espaces de travail. Sur la v2.1.207 et ultérieure, la mise à jour ne suffit pas en dehors d'un dépôt git si vous n'avez pas approuvé le dossier : déterminer qu'un dossier ne se trouve pas dans un dépôt exécute git, et Claude Code n'exécute cette vérification qu'après que vous acceptiez la boîte de dialogue d'approbation, donc utilisez la première étape. Votre répertoire personnel et tout autre [répertoire de configuration](/docs/fr/permissions#project-allow-rules-and-workspace-trust) sont exempts et n'attendent pas la boîte de dialogue. Consultez [Règles d'autorisation du projet et approbation de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust).

<h3 id="working-directory-is-a-network-path">
  Le répertoire de travail est un chemin réseau
</h3>

Claude Code n'ajoute pas les chemins réseau comme répertoires de travail. La recherche d'un chemin réseau peut contacter l'hôte qu'il nomme, et sur Windows, ce contact peut envoyer vos identifiants à l'hôte, donc Claude Code refuse le chemin sans le rechercher. Vous voyez ce message quand vous exécutez `/add-dir` avec un tel chemin, ou comme un avertissement au démarrage. Quand il apparaît au démarrage, Claude Code démarre sans ce répertoire.

```text theme={null}
\\server\share is a network path, which cannot be added as a working directory. On Windows, map the share to a drive letter and pass it at launch with --add-dir (a drive letter added mid-session does not yet carry remote-read trust).
```

Les chemins que Claude Code refuse de cette façon incluent :

* Les partages UNC tels que `\\server\share`
* Les chemins de montage automatique tels que `/net/<host>`, sauf si vous avez lancé Claude Code à partir d'un répertoire sous le montage automatique de cet hôte
* Les chemins locaux qui atteignent un emplacement réseau via un lien symbolique ou une jonction

Les lettres de lecteur mappées et les chemins `\\wsl$` ne comptent pas comme des chemins réseau.

**À faire :**

* Sur Windows, mappez le partage à une lettre de lecteur, par exemple avec `net use Z: \\server\share`, et passez le lecteur au lancement avec `claude --add-dir Z:\`.
* Sur macOS ou Linux, montez le partage à un chemin local et ajoutez ce chemin à la place.
* Si le chemin se trouve dans `permissions.additionalDirectories`, supprimez-le du fichier de paramètres qui le répertorie.

Avant la v2.1.257, Claude Code acceptait un chemin réseau accessible comme répertoire de travail.

<h3 id="remote-managed-settings-failed-to-load">
  Les paramètres gérés à distance n'ont pas pu être chargés
</h3>

Votre session est éligible pour les [paramètres gérés par le serveur](/docs/fr/server-managed-settings), mais Claude Code n'a pas pu les récupérer, donc il affiche cet avertissement dans les sessions interactives. La cause entre parenthèses nomme ce qui a échoué, par exemple `network error`, `request timed out`, ou `authentication rejected (401)`, et le reste de la ligne indique la politique sur laquelle la session s'exécute :

* **Paramètres mis en cache à partir d'une récupération antérieure réussie** : Claude Code exécute la session sur cette politique mise en cache, sauf les [variables d'environnement retenues](/docs/fr/server-managed-settings#fetch-and-caching-behavior), et la ligne lit `using cached policy`.
* **Pas de cache** : Claude Code exécute la session sans paramètres gérés par le serveur, et la ligne lit `no remote policy applied`.

**À faire :**

* Agissez sur la cause que le message nomme : pour une cause réseau, vérifiez que cette machine peut atteindre `api.anthropic.com` ; pour une cause d'authentification, vérifiez votre connexion avec `/status`
* Exécutez `/status` ou `claude doctor` pour le diagnostic complet

Avant la v2.1.248, Claude Code ne signalait une récupération de paramètres échouée que dans le journal de débogage.

<h3 id="managed-settings-were-not-approved">
  Les paramètres gérés n'ont pas été approuvés
</h3>

Les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) de votre organisation incluent des paramètres qui nécessitent votre approbation, et vous avez refusé la [boîte de dialogue d'approbation de sécurité](/docs/fr/server-managed-settings#security-approval-dialogs), donc Claude Code se ferme sans les appliquer :

```text theme={null}
Managed settings were not approved; exiting without applying them.
```

**À faire :**

* Redémarrez Claude Code et approuvez la boîte de dialogue pour continuer selon les paramètres de votre organisation. Une boîte de dialogue refusée n'est pas mémorisée, donc elle réapparaît au prochain démarrage.
* Si vous n'êtes pas sûr d'un paramètre que la boîte de dialogue répertorie, demandez à celui qui maintient les paramètres gérés de votre organisation avant d'approuver

<h3 id="mcp-server-is-blocked-by-enterprise-managed-policy">
  Le serveur MCP est bloqué par la politique gérée par l'entreprise
</h3>

Vous avez sélectionné **Reconnect** sur un serveur dans `/mcp`, ou vous avez réactivé un serveur désactivé là, et un paramètre qui [restreint les serveurs MCP](/docs/fr/managed-mcp) bloque ce serveur. Claude Code refuse de le connecter et affiche :

```text theme={null}
MCP server <name> is blocked by enterprise managed policy
```

N'importe lequel de ces paramètres peut produire le message :

* Une entrée [`deniedMcpServers`](/docs/fr/managed-mcp#policy-based-control-with-allowlists-and-denylists) qui correspond au serveur, y compris une dans votre propre `~/.claude/settings.json` ou le `.claude/settings.json` du projet
* Une liste [`allowedMcpServers`](/docs/fr/managed-mcp#policy-based-control-with-allowlists-and-denylists) que le serveur ne correspond pas
* [`strictPluginOnlyCustomization`](/docs/fr/settings-reference#strictpluginonlycustomization) avec `mcp` verrouillé, qui bloque les serveurs configurés dans `~/.claude.json` et `.mcp.json`
* [`disableClaudeAiConnectors`](/docs/fr/mcp#disable-claude-ai-connectors), quand le serveur est un connecteur claude.ai

**À faire :**

* Vérifiez vos propres fichiers de paramètres utilisateur et projet pour l'un de ces paramètres et modifiez-le ou supprimez-le
* Si aucun de vos propres paramètres n'explique le blocage, demandez à votre administrateur quel paramètre géré bloque le serveur

Avant la v2.1.257, **Reconnect** et réactiver dans `/mcp` pouvaient connecter un serveur qu'une mise à jour de politique en cours de session avait bloqué.

<h3 id="managed-settings-document-could-not-be-parsed">
  Le document des paramètres gérés n'a pas pu être analysé
</h3>

Votre organisation déploie des [paramètres gérés](/docs/fr/managed-settings), et l'un des documents déployés est présent mais ne peut pas être analysé comme un objet JSON, donc Claude Code se ferme avec le code 1 au démarrage au lieu de s'exécuter sans la politique que le document porte. La ligne nomme la source échouée avant le message :

```text theme={null}
/Library/Application Support/ClaudeCode/managed-settings.json: Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.
```

La source est l'une des suivantes :

* Le chemin du fichier `managed-settings.json` ou un fichier drop-in sous `managed-settings.d`
* Le profil des préférences gérées macOS, `per-user managed preferences` ou `device-level managed preferences`
* La valeur du registre Windows, `Registry: HKLM\SOFTWARE\Policies\ClaudeCode\Settings`

[Trouver les entrées que Claude Code a supprimées](/docs/fr/managed-settings#find-entries-claude-code-dropped) répertorie ce qui rend chaque source non analysable.

Claude Code refuse de démarrer même quand une autre source d'administrateur fournit une politique valide. Vous voyez cette erreur dans les sessions interactives, `claude -p`, les sessions du SDK Agent, les [sessions en arrière-plan](/docs/fr/agent-view), et la plupart des sous-commandes, y compris `claude doctor`. Le refus échoue fermé à dessein : les paramètres dans un document que Claude Code ne peut pas analyser ne peuvent pas être appliqués, et le démarrage quand même exécuterait les sessions sans les contrôles de l'organisation.

Un problème de schéma dans un document analysable ne produit pas cette erreur. [Trouver les entrées que Claude Code a supprimées](/docs/fr/managed-settings#find-entries-claude-code-dropped) couvre ce que Claude Code fait avec un.

Quand un répertoire `managed-settings.d/` existe mais ne peut pas être répertorié, Claude Code signale `Managed settings drop-in directory could not be read:` suivi de l'erreur sous-jacente à la place. [Trouver les entrées que Claude Code a supprimées](/docs/fr/managed-settings#find-entries-claude-code-dropped) couvre quand une défaillance de lecture se ferme au démarrage.

**À faire :**

* Si vous administrez la machine, corrigez le document nommé pour qu'il s'analyse comme un objet JSON, ou supprimez le fichier, le profil ou la valeur du registre. Un `managed-settings.json` vide compte comme `{}` et ne bloque pas le lancement.
* Si vous ne le faites pas, demandez à votre administrateur de corriger le document déployé. Rien dans vos propres fichiers de paramètres ne cause ou n'efface cette erreur.

<h3 id="otelheadershelper-failed">
  otelHeadersHelper a échoué
</h3>

Claude Code affiche cet avertissement comme une notification dans l'interface du terminal, une fois par session interactive, quand le script [`otelHeadersHelper`](/docs/fr/settings-reference#otelheadershelper) échoue ou imprime une sortie qui ne répond pas aux [exigences du script](/docs/fr/monitoring-usage#script-requirements).

Pendant que le script continue d'échouer, les exportations échouent et votre backend de télémétrie ne reçoit rien de la session.

Le texte après `See /status:` indique ce qui a échoué, par exemple le code de sortie du script suivi de sa sortie d'erreur :

```text theme={null}
otelHeadersHelper failed; telemetry is not being exported. See /status: exited 1: token service unreachable
```

**À faire :**

* Exécutez `/status` pour lire le détail de l'échec.
* Corrigez le script pour qu'il se termine avec 0 en moins de 30 secondes et imprime un objet JSON de valeurs d'en-tête de chaîne sur stdout. Consultez [exigences du script](/docs/fr/monitoring-usage#script-requirements).
* Si votre organisation déploie le script via les [paramètres gérés](/docs/fr/managed-settings), demandez à celui qui les maintient de le corriger.

En [mode non interactif](/docs/fr/headless) avec `-p`, le même échec apparaît sur stderr comme `otelHeadersHelper failed (OpenTelemetry export headers unavailable): <error>` à la place.

<h3 id="headershelper-not-run">
  headersHelper non exécuté
</h3>

Claude Code a connecté un serveur MCP avec ses `headers` statiques seuls et a ignoré le [`headersHelper`](/docs/fr/mcp#use-dynamic-headers-for-custom-authentication) du serveur, car l'assistant est une commande shell et le dossier n'a pas d'approbation enregistrée. Un dossier obtient une approbation enregistrée quand vous définissez son entrée dans `~/.claude.json` à la main ou, en dehors de votre répertoire personnel, quand vous acceptez la boîte de dialogue d'approbation pour lui dans une session interactive. Consultez [Approuver un dossier avant l'exécution de son headersHelper](/docs/fr/mcp#trust-a-folder-before-its-headershelper-runs) pour savoir quels serveurs cette vérification s'applique à.

Claude Code écrit cette ligne en [mode non interactif](/docs/fr/headless) uniquement, une fois par serveur. Dans une session interactive, il écrit le même refus au journal de débogage à la place.

```text theme={null}
MCP server 'internal-api': headersHelper not run — this workspace has no persisted trust; accept the trust dialog here once interactively, or set projects["/Users/you/project"].hasTrustDialogAccepted in /Users/you/.claude.json.
```

La clé `projects` que le message imprime est le dossier [Règles d'autorisation du projet et approbation de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust) indique que Claude Code clé l'approbation sur. Accepter la boîte de dialogue d'approbation pour un dossier parent ne satisfait pas la vérification, et une session `-p` ou SDK ne la satisfait pas non plus.

**À faire :**

* Exécutez `claude` dans le dossier que le message nomme, acceptez la boîte de dialogue d'approbation, puis exécutez à nouveau votre commande `-p` ou SDK
* Définissez vous-même l'entrée `hasTrustDialogAccepted` dans `~/.claude.json`, en utilisant la clé `projects` exacte que le message imprime
* Si vous avez démarré la session dans votre répertoire personnel, travaillez à partir d'un répertoire de projet que vous avez approuvé. Quand vous acceptez la boîte de dialogue d'approbation dans votre répertoire personnel, Claude Code conserve cette approbation pour la session actuelle uniquement.

<h3 id="malformed-tool-content-rule">
  Règle Tool(content) mal formée
</h3>

Une [règle de permission](/docs/fr/permissions#permission-rule-syntax) dans l'un de vos fichiers de paramètres n'a pas la forme `Tool` ou `Tool(content)`, par exemple parce que du texte suit la parenthèse fermante ou l'une des parenthèses manque. Claude Code ignore la règle et la répertorie dans la boîte de dialogue des paramètres invalides quand une session interactive démarre, et dans la sortie de [`claude doctor`](/docs/fr/debug-your-config#check-resolved-settings) :

```text theme={null}
Invalid permission rule "Bash(ls) x" was skipped: Malformed Tool(content) rule. Rules take the form Tool or Tool(content) and must end at the closing ")"; parentheses inside the content are literal
```

**À faire :**

* Dans le fichier de paramètres répertorié avec le message, réécrivez la règle pour qu'elle se termine à sa parenthèse fermante, par exemple `Bash(ls *)` à la place de `Bash(ls) x`
* Laissez les parenthèses à l'intérieur du contenu telles qu'elles sont. Elles sont littérales, donc une règle telle que `Edit(./Finance (2024)/**)` est valide sans échappement

Avant la v2.1.260, Claude Code signalait une règle avec des parenthèses non appariées comme `Mismatched parentheses`.

<h3 id="is-not-matched-by-file-permission-checks">
  N'est pas mis en correspondance par les vérifications de permission de fichier
</h3>

Claude Code a trouvé une règle `Write`, `NotebookEdit`, `MultiEdit`, ou `Glob` [permission](/docs/fr/permissions#read-and-edit) avec un chemin dans l'un de vos [fichiers de paramètres](/docs/fr/settings#where-settings-live), dans les [paramètres gérés](/docs/fr/managed-settings), ou dans une valeur de drapeau `--allowedTools`, `--disallowedTools`, ou `--settings`. Il vérifie les permissions de fichier uniquement par rapport aux règles `Edit` et `Read`, donc il ne consulte jamais une règle de chemin qui nomme l'un des autres outils de fichier. Il conserve la règle et ne change rien d'autre ; l'avertissement nomme la règle, sa source entre parenthèses, et le remplacement à écrire :

```text theme={null}
Permission deny rule (.claude/settings.json): Write(docs/**) is not matched by file permission checks — only Edit(path) rules are. Use Edit(docs/**) instead (Edit rules cover all file-editing tools).
```

**À faire :**

* Remplacez les règles `Write(path)`, `NotebookEdit(path)`, et `MultiEdit(path)` héritées par `Edit(path)`. Les règles `Edit` couvrent tous les outils d'édition de fichiers.
* Sauf dans `--allowedTools`, où Claude Code accepte une règle `Glob` sans avertissement, remplacez les règles `Glob(path)` par `Read(path)`.
* Corrigez la règle à la source que l'avertissement nomme entre parenthèses : un chemin de fichier de paramètres, ou le drapeau lui-même pour `--allowed-tools` et `--disallowed-tools`. Un chemin `claude-settings-<hash>.json` qui n'existe pas sur le disque représente une valeur `--settings` en ligne. Corrigez le JSON que vous passez à ce drapeau.
* Laissez les règles de nom d'outil nu telles que `Write` ou `Glob` seules. Claude Code les met en correspondance au [niveau de l'outil](/docs/fr/permissions#match-all-uses-of-a-tool) et ne les avertit pas.
* Si la source lit `managed policy settings`, transmettez l'avertissement à celui qui maintient vos paramètres gérés, car vous ne pouvez pas l'effacer vous-même.

Dans une [session en arrière-plan](/docs/fr/agent-view) ou avec `--output-format json` ou `stream-json`, Claude Code écrit l'avertissement au journal de débogage au lieu de stderr, donc la sortie lue par machine reste propre. Exécutez avec `--debug` pour la capturer à `~/.claude/debug/<session-id>.txt`. Avant la v2.1.210, Claude Code acceptait ces règles sans avertissement.

<h3 id="has-a-wildcard-before-the-rest-of-the-command">
  A un caractère générique avant le reste de la commande
</h3>

Claude Code a trouvé une règle d'autorisation `Bash` dont le `*` vient avant un mot ultérieur qui détermine quelle commande c'est, par exemple `Bash(git * main)` ou `Bash(git -C * status *)`, dans l'un de vos [fichiers de paramètres](/docs/fr/settings#where-settings-live), dans les [paramètres gérés](/docs/fr/managed-settings), ou dans une valeur de drapeau `--allowedTools` ou `--settings`. Le `*` correspond à n'importe quel texte, y compris les options insérées à cette position : `Bash(git * main)` approuve également `git -c core.fsmonitor=<script> diff main`, où `-c` fait exécuter à git un programme que la commande nomme. [Modèles de caractères génériques](/docs/fr/permissions#wildcard-patterns) montre les règles de correspondance.

L'avertissement existe pour que vous puissiez affiner une règle dont le caractère générique est plus large que vous ne l'aviez prévu. Claude Code conserve la règle et ne change rien à la façon dont elle correspond ; l'avertissement nomme la règle et sa source entre parenthèses :

```text theme={null}
Permission allow rule (.claude/settings.json): Bash(git -C * status *) has a wildcard before the rest of the command, so it also matches any options inserted at that position and approves them without a prompt. For git, options such as -c and --exec-path can run arbitrary commands. Replace that * with the exact value you mean, or only use * after the subcommand (for example Bash(git status *)).
```

**À faire :**

* Remplacez le `*` avant la sous-commande par la valeur exacte que vous entendez : `Bash(git checkout main)` à la place de `Bash(git * main)`.
* Déplacez chaque `*` après la sous-commande : `Bash(git status *)` à la place de `Bash(git -C * status *)`. Écrivez une règle par sous-commande que vous voulez autoriser.
* Corrigez la règle à la source que l'avertissement nomme entre parenthèses : un chemin de fichier de paramètres, ou le drapeau `--allowed-tools` lui-même. Un chemin `claude-settings-<hash>.json` qui n'existe pas sur le disque représente une valeur `--settings` en ligne. Corrigez le JSON que vous passez à ce drapeau.
* Si la source lit `managed policy settings`, transmettez l'avertissement à celui qui maintient vos paramètres gérés, car vous ne pouvez pas l'effacer vous-même.

Claude Code n'avertit pas les règles de refus et de demande avec la même forme : il refuse ou demande les commandes supplémentaires qu'elles correspondent plutôt que de les approuver. Il n'avertit pas non plus les règles dont la sous-commande vient avant le premier `*`, par exemple `Bash(git commit *)`, ou les règles dans lesquelles aucun mot autre qu'une option ne suit le `*`, par exemple `Bash(git *)`, ou les règles de préfixe `:*` telles que `Bash(git:*)`.

Dans une [session en arrière-plan](/docs/fr/agent-view) ou avec `--output-format json` ou `stream-json`, Claude Code écrit l'avertissement au journal de débogage au lieu de stderr, donc la sortie lue par machine reste propre. Exécutez avec `--debug` pour la capturer à `~/.claude/debug/<session-id>.txt`. Avant la v2.1.246, Claude Code acceptait ces règles sans avertissement.

<h3 id="crosssessioninbound-must-be-one-of-accept-hold-refuse">
  crossSessionInbound doit être l'un de accept, hold, refuse
</h3>

Un fichier de paramètres définit [`crossSessionInbound`](/docs/fr/settings-reference#crosssessioninbound) à une valeur que Claude Code ne reconnaît pas, par exemple la faute de frappe `"reject"`. La deuxième phrase de l'avertissement dépend du fichier qui contient la valeur ; dans un fichier utilisateur, projet, local, ou `--settings`, elle lit :

```text theme={null}
"crossSessionInbound" must be one of "accept", "hold", "refuse"; received "reject". This value was ignored; while it is present, cross-session messages are held for your approval instead of being delivered. Set it to one of the values above.
```

Dans les [paramètres gérés](/docs/fr/managed-settings), Claude Code traite la valeur non reconnue comme `refuse`, la valeur la plus restrictive, et l'avertissement dit que les messages entre sessions sont rejetés jusqu'à ce qu'un administrateur le corrige. Pour savoir comment la retenue se combine avec les valeurs dans vos autres fichiers de paramètres, consultez [`crossSessionInbound`](/docs/fr/settings-reference#crosssessioninbound).

**À faire :**

* Définissez la clé à `"accept"`, `"hold"`, ou `"refuse"`, ou supprimez-la
* Quand l'avertissement nomme les paramètres gérés, demandez à l'administrateur de corriger la valeur

Avant la v2.1.248, Claude Code ignorait une valeur non reconnue sans avertissement.

<h3 id="the-200k-limit-isnt-enforced">
  La limite de 200K n'est pas appliquée
</h3>

Vous avez défini [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/fr/env-vars), ce qui fait normalement que la [compaction automatique](/docs/fr/model-config#default-auto-compact-thresholds) maintient les sessions sur les modèles à contexte 1M à une fenêtre de 200K, mais aucun seuil de compaction ne limite cette session à ou en dessous de 200K, donc la conversation peut dépasser.

```text theme={null}
CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced for <model>, so this session can grow past it. To enforce it, set CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000 (or the autoCompactWindow setting).
```

Claude Code applique la limite de 200K de lui-même pour chaque modèle qu'il reconnaît comme ayant une fenêtre native de 1M, et pour les ID de modèle qu'il ne reconnaît pas, il compacte à la fenêtre qu'il suppose. L'avertissement apparaît quand une autre configuration défait cette application :

* L'ID du modèle n'est pas un que Claude Code reconnaît, par exemple un alias de [passerelle LLM](/docs/fr/llm-gateway), et vous avez défini [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/fr/env-vars) ou augmenté la fenêtre supposée au-delà de 200K avec [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/fr/env-vars). Dans ce cas, le message offre également `or update to a Claude Code version that recognizes <model>` comme un remède.
* Un `context-1m` bêta demandé via [`ANTHROPIC_BETAS`](/docs/fr/env-vars) ou le drapeau [`--betas`](/docs/fr/cli-reference#cli-flags) demande toujours à l'API la fenêtre 1M sur un modèle qui accepte cette bêta, tandis que rien ne compacte la session à 200K

**À faire :**

* Définissez [`CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000`](/docs/fr/env-vars), ou le paramètre [`autoCompactWindow`](/docs/fr/settings-reference#autocompactwindow) à `200000`, pour que la compaction automatique compacte à la limite de 200K
* Si le message nomme un ID de modèle que cette version ne reconnaît pas, exécutez `claude update`. Une version qui reconnaît l'ID comme un modèle à contexte 1M applique la limite sans configuration supplémentaire.
* Si vous voulez que la session utilise la fenêtre complète du modèle à la place, désactivez `CLAUDE_CODE_DISABLE_1M_CONTEXT` ; l'avertissement signale uniquement que la limite de 200K n'est pas appliquée

Dans une [session en arrière-plan](/docs/fr/agent-view) ou avec `--output-format json` ou `stream-json`, Claude Code écrit l'avertissement au journal de débogage au lieu de stderr.

<h3 id="unrecognized-model-id-on-a-request">
  ID de modèle non reconnu sur une requête
</h3>

Claude Code a envoyé une requête pour un ID de modèle que votre version de Claude Code ne reconnaît pas, et n'a trouvé aucune entrée [`modelOverrides`](/docs/fr/model-config#override-model-ids-per-version) qui mappe cet ID à un modèle qu'elle reconnaît. Claude Code envoie toujours la requête avec l'ID tel que vous l'avez configuré, et ne se ferme pas ou ne change pas de modèle.

```text theme={null}
[claude-code:unrecognized_model] {"model":"my-proxy-model","query_source":"sdk"}
```

Dans un script ou un harnais qui lit stderr, mettez en correspondance le préfixe `[claude-code:unrecognized_model]`. Après le préfixe et un espace, Claude Code écrit un objet JSON d'une ligne. Claude Code peut ajouter des champs à celui-ci dans une version ultérieure, donc ignorez tout champ que vous n'attendez pas. Il écrit au moins ces deux :

* `model` : la chaîne de modèle telle que vous l'avez configurée
* `query_source` : le chemin de requête qui a utilisé le modèle. Claude Code signale `sdk` pour une exécution `-p` et une valeur qui commence par `agent:` pour un sous-agent.

Claude Code écrit la ligne à l'un de deux endroits, selon la façon dont vous l'exécutez :

* En [mode non interactif](/docs/fr/headless) avec `-p`, Claude Code l'écrit sur stderr sous chaque `--output-format`, pour que vous puissiez analyser stdout sans filtrer la ligne
* Dans une session interactive ou une [session en arrière-plan](/docs/fr/agent-view), Claude Code l'écrit au journal de débogage à la place ; exécutez avec `--debug` pour la capturer à `~/.claude/debug/<session-id>.txt`

Claude Code écrit la ligne une fois par chaîne de modèle par processus. Il écrit une ligne séparée pour chaque ID non reconnu supplémentaire, par exemple un qu'un [sous-agent](/docs/fr/sub-agents#choose-a-model) ou une [fonctionnalité en arrière-plan](/docs/fr/costs#background-token-usage) utilise.

Claude Code n'écrit pas la ligne pour les ID de fournisseur qu'il résout à un modèle qu'il reconnaît, par exemple les ID Amazon Bedrock `us.anthropic.claude-...`, les ID de la plateforme d'agent de Google Cloud avec un suffixe de version `@`, et les noms de déploiement Microsoft Foundry qui contiennent un ID de modèle Claude. Claude Code vérifie le modèle derrière un [ARN de profil d'inférence d'application](/docs/fr/amazon-bedrock#map-each-model-version-to-an-inference-profile) Amazon Bedrock plutôt que l'ARN lui-même. Il n'écrit pas de ligne pour un ARN qu'il ne peut pas résoudre, par exemple un mal orthographié.

**À faire :**

* Si vous avez défini l'ID à dessein, par exemple un alias de [passerelle LLM](/docs/fr/llm-gateway), ajoutez une entrée [`modelOverrides`](/docs/fr/model-config#override-model-ids-per-version) à votre [fichier de paramètres](/docs/fr/settings#where-settings-live) avec l'ID comme sa valeur. Utilisez un ID de modèle Anthropic comme clé, pas un alias de famille tel que `opus`. Pour `my-proxy-model` de la ligne d'exemple, ajoutez cette entrée :

  ```json theme={null}
  {
    "modelOverrides": {
      "claude-opus-4-6": "my-proxy-model"
    }
  }
  ```

  Claude Code traite alors `my-proxy-model` comme `claude-opus-4-6` et arrête d'écrire la ligne.

* Si l'ID nomme un modèle plus récent que votre version de Claude Code, exécutez `claude update`

* Si l'ID est une faute de frappe, corrigez-le dans l'un des [endroits où vous pouvez définir un modèle](/docs/fr/model-config#setting-your-model) ou les [variables d'alias](/docs/fr/model-config#environment-variables) qui le contiennent. Si `query_source` commence par `agent:`, corrigez-le plutôt où vous définissez le [modèle du sous-agent](/docs/fr/sub-agents#choose-a-model).

Avant la v2.1.233, Claude Code n'écrivait pas de ligne quand il envoyait une requête pour un ID de modèle qu'il ne reconnaissait pas.

<h3 id="stale-sandbox-mask-files-left-by-a-killed-session">
  Fichiers de masque de sandbox obsolètes laissés par une session tuée
</h3>

`claude doctor` imprime cet avertissement dans ses diagnostics, et `/status` répertorie la même ligne. Il apparaît sur Linux et WSL2 quand le [sandboxing](/docs/fr/sandboxing) est activé avec l'isolation du système de fichiers activée.

Pendant qu'une commande en sandbox s'exécute, le sandbox maintient un refus d'écriture sur un fichier qui n'existe pas encore en créant un espace réservé de 0 octet en lecture seule là, et le supprime après. Une session tuée avant que ce nettoyage s'exécute, par exemple par SIGKILL, laisse les espaces réservés derrière. Les sessions ultérieures les lient en lecture seule à nouveau à chaque démarrage, donc une écriture de paramètres telle que l'enregistrement de « Oui, et ne me le demande plus » échoue où l'un se trouve.

```text theme={null}
- Stale sandbox mask files left by a killed session: /home/you/project/.claude/settings.local.json
  Fix: Remove each with `rm <path>` while no other Claude Code session is running in that project — a 0-byte read-only file where a settings file belongs makes "Yes, and don't ask again" fail to save, and the sandbox binds it read-only again on every start
```

**À faire :**

* Quittez toute autre session Claude Code s'exécutant dans ce projet, puis supprimez chaque fichier répertorié avec `rm`. L'avertissement nomme jusqu'à trois fichiers et compte le reste, donc réexécutez `claude doctor` après la suppression jusqu'à ce que l'avertissement ne réapparaisse plus. Un espace réservé que le sandbox d'une autre session utilise toujours est une partie vivante de la protection en écriture de cette session
* Si un choix de permission que vous avez enregistré avec « Oui, et ne me le demande plus » n'a pas collé, enregistrez-le à nouveau après la suppression de l'espace réservé

Avant la v2.1.257, `claude doctor` ne signalait pas ces fichiers ; les versions antérieures laissent les mêmes espaces réservés derrière quand une session est tuée.

<h2 id="responses-seem-lower-quality-than-usual">
  Les réponses semblent de qualité inférieure à la normale
</h2>

Si les réponses de Claude semblent moins performantes que prévu mais qu'aucune erreur n'est affichée, la cause est généralement l'état de la conversation plutôt que le modèle lui-même. Claude Code ne change pas silencieusement les versions de modèle. Il peut basculer vers un modèle de secours dans trois cas spécifiques :

* Un [`--fallback-model`](/docs/fr/cli-reference#cli-flags) configuré prend le relais après une erreur de disponibilité, pour ce tour uniquement, avec un avis dans la transcription
* Une vérification de démarrage d'Amazon Bedrock ou de la plateforme Agent de Google Cloud détecte que votre modèle par défaut n'est pas disponible
* Le [basculement automatique du modèle](/docs/fr/model-config#automatic-model-fallback) sur Fable 5.1, Fable 5, Opus 5.5 et Opus 5 déplace la session vers le modèle de secours de la catégorie signalée, lorsque cette catégorie en possède un, et affiche un avis dans la transcription

La vérification de sélection du modèle ci-dessous détecte les deuxième et troisième cas ; le premier apparaît comme un avis de transcription plutôt qu'un changement de `/model`. La [configuration du modèle](/docs/fr/model-config) explique quand chaque basculement s'applique.

Vérifiez d'abord ceci :

* **Sélection du modèle** : exécutez `/model` pour confirmer que vous êtes sur le modèle attendu. Un choix `/model` précédent ou une variable d'environnement `ANTHROPIC_MODEL` peut vous placer sur un modèle plus petit que prévu.
* **Niveau d'effort** : exécutez `/effort` pour vérifier le niveau de raisonnement actuel et l'augmenter pour le débogage difficile ou le travail de conception. Les valeurs par défaut varient selon le modèle, vérifiez donc avant de supposer que vous êtes en dessous du maximum. Consultez [Ajuster le niveau d'effort](/docs/fr/model-config#adjust-effort-level) pour les valeurs par défaut par modèle et le raccourci `ultrathink`.
* **Pression contextuelle** : exécutez `/context` pour voir le remplissage de la fenêtre. S'il est proche de la capacité, exécutez `/compact` à un point naturel ou `/clear` pour recommencer. Consultez [Explorer la fenêtre contextuelle](/docs/fr/context-window) pour voir comment l'auto-compact affecte les tours précédents.
* **Instructions obsolètes** : les fichiers `CLAUDE.md` volumineux ou obsolètes et les définitions d'outils MCP consomment du contexte et peuvent orienter les réponses. La vérification `/doctor` signale les fichiers mémoire surdimensionnés et les extensions inutilisées, et `/context` affiche l'utilisation des jetons des outils MCP. Avant la v2.1.205, `/doctor` ouvrait un écran de diagnostics qui signalait les fichiers mémoire surdimensionnés et les définitions de sous-agents.

Lorsqu'une réponse s'avère incorrecte, revenir en arrière fonctionne généralement mieux que de répondre avec des corrections. Appuyez deux fois sur Échap ou exécutez `/rewind` pour revenir avant le mauvais tour, puis reformulez l'invite avec plus de détails. Corriger dans le fil de discussion conserve la mauvaise tentative en contexte, ce qui peut ancrer les réponses ultérieures à celle-ci. Consultez [Checkpointing](/docs/fr/checkpointing).

Si la qualité semble toujours incorrecte après vérification des éléments ci-dessus, exécutez `/feedback` et décrivez ce que vous attendiez par rapport à ce que vous avez obtenu. Les commentaires soumis de cette manière incluent la transcription de la conversation, ce qui est le moyen le plus rapide pour Anthropic de diagnostiquer une véritable régression. Consultez [Signaler une erreur](#report-an-error) si `/feedback` n'est pas disponible dans votre environnement.

Si Claude vous avertit d'une injection d'invite suspectée, ou refuse une demande en raison d'une injection suspectée, et que le texte nommé par l'avertissement est un contexte que Claude Code ajoute automatiquement à la conversation plutôt que du contenu de fichier ou web, exécutez `claude update` et réessayez. Si l'avertissement se répète après la mise à jour, [signalez-le](#report-an-error) plutôt que de coller le contenu signalé dans l'invite. Avant la v2.1.201, Sonnet 5 refusait certaines demandes de la même manière.

<h2 id="report-an-error">
  Signaler une erreur
</h2>

Pour les erreurs provenant de composants non couverts par cette page, consultez le guide pertinent :

* Le serveur MCP n'a pas pu se connecter ou s'authentifier : [MCP](/docs/fr/mcp)
* Le script hook a échoué ou a bloqué un outil : [Déboguer les hooks](/docs/fr/hooks#debug-hooks)
* Erreur de permission ou erreurs du système de fichiers lors de l'installation : [Dépanner l'installation et la connexion](/docs/fr/troubleshoot-install)

Si une erreur n'est pas répertoriée ici ou si la correction suggérée ne vous aide pas :

* Exécutez `/feedback` dans Claude Code pour envoyer la transcription et une description à Anthropic. La commande propose également d'ouvrir un problème GitHub prérempli. L'envoi à Anthropic nécessite une [authentification](/docs/fr/authentication). Sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry et d'autres fournisseurs tiers, ou lorsqu'aucune identifiant Anthropic n'est configuré, `/feedback` enregistre une archive locale que vous pouvez envoyer à votre représentant de compte Anthropic à la place.
* Exécutez `claude doctor` depuis votre shell pour un diagnostic en lecture seule de votre installation, ou exécutez la vérification `/doctor` dans Claude Code pour trouver et corriger les problèmes de configuration
* Vérifiez [status.claude.com](https://status.claude.com) pour les incidents actifs
* Recherchez les [problèmes existants](https://github.com/anthropics/claude-code/issues) sur GitHub
