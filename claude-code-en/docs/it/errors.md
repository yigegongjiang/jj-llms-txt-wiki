> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento degli errori

> Consulta i messaggi di errore runtime di Claude Code con il significato di ciascuno e come risolverli.

Questa pagina elenca gli errori runtime che Claude Code visualizza e come recuperare da ciascuno, più cosa controllare quando le risposte sembrano non corrette senza un errore. Per gli errori di installazione come `command not found` o errori TLS durante la configurazione, vedi [Risoluzione dei problemi di installazione e accesso](/docs/it/troubleshoot-install).

Ad eccezione degli [errori di Wrapper e IDE](#wrapper-and-ide-errors), che il programma di avvio stampa piuttosto che Claude Code stesso, questi errori e comandi di recupero si applicano su CLI, l'[app Desktop](/docs/it/desktop) e le [sessioni cloud](/docs/it/claude-code-on-the-web), poiché tutti e tre avvolgono lo stesso CLI di Claude Code. Per altri problemi specifici della superficie, vedi la sezione di risoluzione dei problemi nella pagina di quella superficie.

<Note>
  Claude Code chiama l'API Claude per le risposte del modello, quindi la maggior parte degli errori runtime si mappano a un codice di errore API sottostante. Questa pagina copre cosa significa ogni errore all'interno di Claude Code e come recuperare. Per le definizioni del codice di stato HTTP grezzo, vedi il [riferimento degli errori della piattaforma Claude](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Trovare il vostro errore
</h2>

Abbinate il messaggio che vedete a una sezione qui sotto.

| Messaggio                                                                                                                                                                                                                                                            | Sezione                                                                                                                                   |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `API Error: 500 Internal server error`                                                                                                                                                                                                                               | [Errori del server](#api-error-500-internal-server-error)                                                                                 |
| `API Error: Repeated 529 Overloaded errors`                                                                                                                                                                                                                          | [Errori del server](#api-error-repeated-529-overloaded-errors)                                                                            |
| `Request timed out`                                                                                                                                                                                                                                                  | [Errori del server](#request-timed-out), oppure [Rete](#unable-to-connect-to-api) se il messaggio menziona la vostra connessione internet |
| `API Error: No response from API`                                                                                                                                                                                                                                    | [Errori del server](#no-response-from-api)                                                                                                |
| `Server error mid-response. The response above may be incomplete.`                                                                                                                                                                                                   | [Errori del server](#the-response-above-may-be-incomplete)                                                                                |
| `Connection lost mid-response` / `Your computer went to sleep mid-response` / `The response stopped arriving`                                                                                                                                                        | [Errori del server](#the-response-above-may-be-incomplete)                                                                                |
| `Connection closed mid-response` / `Response stalled mid-stream`                                                                                                                                                                                                     | [Errori del server](#the-response-above-may-be-incomplete)                                                                                |
| `Connection lost before a response was produced` / `Your computer went to sleep before a response was produced` / `The response stalled before a response was produced`                                                                                              | [Tentativi automatici](#automatic-retries)                                                                                                |
| `Connection closed while thinking` / `Response stalled while thinking`                                                                                                                                                                                               | [Tentativi automatici](#automatic-retries)                                                                                                |
| `Connection lost while your computer was asleep`                                                                                                                                                                                                                     | [Tentativi automatici](#automatic-retries)                                                                                                |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`                                                                                                                                                                                 | [Errori del server](#auto-mode-cannot-determine-the-safety-of-an-action)                                                                  |
| `Auto mode could not evaluate this action and is blocking it for safety`                                                                                                                                                                                             | [Errori del server](#auto-mode-cannot-determine-the-safety-of-an-action)                                                                  |
| `Auto mode classifier transcript exceeded context window`                                                                                                                                                                                                            | [Errori del server](#auto-mode-cannot-determine-the-safety-of-an-action)                                                                  |
| `Agent aborted: auto mode classifier request refused by the safety safeguard`                                                                                                                                                                                        | [Errori del server](#auto-mode-cannot-determine-the-safety-of-an-action)                                                                  |
| `The server-side auto mode classifier gave no verdict`                                                                                                                                                                                                               | [Errori del server](#the-server-returned-no-safety-verdict)                                                                               |
| `Auto mode is unavailable — the server returned no safety verdict for the last 10 responses`                                                                                                                                                                         | [Errori del server](#the-server-returned-no-safety-verdict)                                                                               |
| `Agent terminated early due to an API error`                                                                                                                                                                                                                         | [Errori del server](#agent-terminated-early-due-to-an-api-error)                                                                          |
| `You've hit your session limit` / `You've hit your weekly limit` / `You've hit your Opus limit` / `You've hit your Sonnet limit`                                                                                                                                     | [Limiti di utilizzo](#youve-hit-your-session-limit)                                                                                       |
| `Usage credits required for 1M context`                                                                                                                                                                                                                              | [Limiti di utilizzo](#usage-credits-required-for-1m-context)                                                                              |
| `the prompt to confirm went unanswered — nothing was sent`                                                                                                                                                                                                           | [Limiti di utilizzo](#the-prompt-to-confirm-went-unanswered)                                                                              |
| `Server is temporarily limiting requests`                                                                                                                                                                                                                            | [Limiti di utilizzo](#server-is-temporarily-limiting-requests)                                                                            |
| `Request rejected (429)`                                                                                                                                                                                                                                             | [Limiti di utilizzo](#request-rejected-429)                                                                                               |
| `Credit balance is too low`                                                                                                                                                                                                                                          | [Limiti di utilizzo](#credit-balance-is-too-low)                                                                                          |
| `You've hit your monthly spend limit` / `You've hit your individual spend limit` / `You've hit your org's monthly spend limit` / `You've hit your channel's monthly spend limit` / `You've hit your team's shared budget` / `You've hit your individual usage limit` | [Limiti di utilizzo](#youve-hit-your-monthly-spend-limit)                                                                                 |
| `Could not update your spend limit`                                                                                                                                                                                                                                  | [Limiti di utilizzo](#could-not-update-your-spend-limit)                                                                                  |
| `spend limit reached` / `spend limit unavailable`                                                                                                                                                                                                                    | [Limiti di utilizzo](#spend-limit-reached)                                                                                                |
| `Not logged in · Please run /login`                                                                                                                                                                                                                                  | [Autenticazione](#not-logged-in)                                                                                                          |
| `Could not resolve authentication method`                                                                                                                                                                                                                            | [Autenticazione](#could-not-resolve-authentication-method)                                                                                |
| `Invalid API key`                                                                                                                                                                                                                                                    | [Autenticazione](#invalid-api-key)                                                                                                        |
| `Your apiKeyHelper script is failing`                                                                                                                                                                                                                                | [Autenticazione](#your-apikeyhelper-script-is-failing)                                                                                    |
| `Invalid auth token · Fix external auth token`                                                                                                                                                                                                                       | [Autenticazione](#invalid-request-header-value)                                                                                           |
| `Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable`                                                                                                                                                                                                    | [Autenticazione](#invalid-request-header-value)                                                                                           |
| `Invalid request header from the environment · Fix the environment variable`                                                                                                                                                                                         | [Autenticazione](#invalid-request-header-value)                                                                                           |
| `This organization has been disabled`                                                                                                                                                                                                                                | [Autenticazione](#this-organization-has-been-disabled)                                                                                    |
| `Your organization has disabled API key authentication`                                                                                                                                                                                                              | [Autenticazione](#your-organization-has-disabled-api-key-authentication)                                                                  |
| `Your organization has disabled Claude subscription access`                                                                                                                                                                                                          | [Autenticazione](#your-organization-has-disabled-claude-subscription-access)                                                              |
| `Routines are disabled by your organization's policy`                                                                                                                                                                                                                | [Autenticazione](#routines-are-disabled-by-your-organizations-policy)                                                                     |
| `Remote Control is only available when using Claude via api.anthropic.com`                                                                                                                                                                                           | [Autenticazione](#remote-control-requires-the-anthropic-api)                                                                              |
| `OAuth token refresh failed — run /login to re-authenticate`                                                                                                                                                                                                         | [Autenticazione](#remote-control-couldnt-refresh-your-login)                                                                              |
| `JWT refresh failed: no OAuth token — run /login`                                                                                                                                                                                                                    | [Autenticazione](#remote-control-couldnt-refresh-your-login)                                                                              |
| `Claude.ai login expired`                                                                                                                                                                                                                                            | [Autenticazione](#remote-control-couldnt-refresh-your-login)                                                                              |
| `Claude.ai login was rejected — run /login, then /remote-control`                                                                                                                                                                                                    | [Autenticazione](#remote-control-couldnt-refresh-your-login)                                                                              |
| `OAuth token unavailable — run /login to restore Remote Control`                                                                                                                                                                                                     | [Autenticazione](#remote-control-couldnt-refresh-your-login)                                                                              |
| `Signed out of Claude — run /login, then /remote-control`                                                                                                                                                                                                            | [Autenticazione](#remote-control-couldnt-refresh-your-login)                                                                              |
| `signed-in claude.ai account or organization changed on this machine`                                                                                                                                                                                                | [Autenticazione](#remote-control-stopped-because-the-signed-in-account-changed)                                                           |
| `Remote Control stopped — the app running this session is now signed in to a different Claude account`                                                                                                                                                               | [Autenticazione](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)                             |
| `Remote Control stopped — the app running this session is signed out of Claude`                                                                                                                                                                                      | [Autenticazione](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)                             |
| `OAuth token revoked` / `OAuth token has expired`                                                                                                                                                                                                                    | [Autenticazione](#oauth-token-revoked-or-expired)                                                                                         |
| `API Error: 401 Invalid authentication credentials`                                                                                                                                                                                                                  | [Autenticazione](#api-error-401-invalid-authentication-credentials)                                                                       |
| `Login expired · Please run /login`                                                                                                                                                                                                                                  | [Autenticazione](#login-expired)                                                                                                          |
| `Claude login not accepted · Run /login, then try again`                                                                                                                                                                                                             | [Autenticazione](#claude-login-not-accepted)                                                                                              |
| `Artifacts need a claude.ai login`                                                                                                                                                                                                                                   | [Autenticazione](#artifacts-need-a-claude-ai-login)                                                                                       |
| `Not signed in to the Cloud gateway — run /login.`                                                                                                                                                                                                                   | [Autenticazione](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                                  |
| `Administrator policy requires a Cloud gateway sign-in on this machine`                                                                                                                                                                                              | [Autenticazione](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                                  |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                                                                                                                                                                                           | [Autenticazione](#login-expired)                                                                                                          |
| `Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                            | [Autenticazione](#your-account-is-on-hold)                                                                                                |
| `Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                     | [Autenticazione](#your-account-is-on-hold)                                                                                                |
| `Anthropic profile login expired · Re-authenticate your Anthropic profile`                                                                                                                                                                                           | [Autenticazione](#anthropic-profile-login-expired)                                                                                        |
| `Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile`                                                                                                                                                 | [Autenticazione](#anthropic-profile-login-expired)                                                                                        |
| `does not meet scope requirement user:profile`                                                                                                                                                                                                                       | [Autenticazione](#oauth-scope-requirement)                                                                                                |
| `claude.ai rejected the session token` / `session token rejected`                                                                                                                                                                                                    | [Autenticazione](#claude-ai-rejected-the-session-token)                                                                                   |
| `MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)`                                                                                                                                                                                       | [Autenticazione](#mcp-server-needs-you-to-sign-in-again)                                                                                  |
| `rejected the credential from its headersHelper` / `rejected the Authorization header in its config`                                                                                                                                                                 | [Autenticazione](#mcp-server-needs-you-to-sign-in-again)                                                                                  |
| `MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate`                                                                                                                                                                  | [Autenticazione](#mcp-server-needs-you-to-sign-in-again)                                                                                  |
| `MCP server "<name>" requires re-authorization (token expired)`                                                                                                                                                                                                      | [Autenticazione](#mcp-server-needs-you-to-sign-in-again)                                                                                  |
| `Issuer mismatch in authorization response (RFC 9207)`                                                                                                                                                                                                               | [Autenticazione](#issuer-mismatch-in-authorization-response)                                                                              |
| `Cloud gateway session expired — run /login to reconnect.`                                                                                                                                                                                                           | [Autenticazione](#cloud-gateway-session-expired)                                                                                          |
| `Cloud gateway <url> no longer accepts this session`                                                                                                                                                                                                                 | [Autenticazione](#cloud-gateway-session-expired)                                                                                          |
| `Sign-in timed out while waiting for you to continue. Try again.`                                                                                                                                                                                                    | [Autenticazione](#sign-in-timed-out-while-waiting-for-you-to-continue)                                                                    |
| `AWS credentials expired or invalid`                                                                                                                                                                                                                                 | [Autenticazione](#aws-credentials-expired-or-invalid)                                                                                     |
| `AWS authentication failed`                                                                                                                                                                                                                                          | [Autenticazione](#aws-authentication-failed)                                                                                              |
| `Google Cloud credentials expired or invalid`                                                                                                                                                                                                                        | [Autenticazione](#google-cloud-credentials-expired-or-invalid)                                                                            |
| `Google Cloud authentication failed`                                                                                                                                                                                                                                 | [Autenticazione](#google-cloud-authentication-failed)                                                                                     |
| `Microsoft Foundry authentication failed`                                                                                                                                                                                                                            | [Autenticazione](#microsoft-foundry-authentication-failed)                                                                                |
| `Gateway refused the request`                                                                                                                                                                                                                                        | [Autenticazione](#gateway-refused-the-request)                                                                                            |
| `Could not load AWS credentials` / `Could not load Google Cloud credentials`                                                                                                                                                                                         | [Autenticazione](#could-not-load-aws-or-google-cloud-credentials)                                                                         |
| `AWS default-chain credential resolve timed out`                                                                                                                                                                                                                     | [Autenticazione](#aws-default-chain-credential-resolve-timed-out)                                                                         |
| `Timed out after 60s waiting for AWS`                                                                                                                                                                                                                                | [Autenticazione](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                                   |
| `A request to AWS timed out. Check your network and proxy settings, then try again.`                                                                                                                                                                                 | [Autenticazione](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                                   |
| `Could not load the default credentials` on Google Cloud's Agent Platform                                                                                                                                                                                            | [Autenticazione](#could-not-load-aws-or-google-cloud-credentials)                                                                         |
| `Unable to connect to API`                                                                                                                                                                                                                                           | [Rete](#unable-to-connect-to-api)                                                                                                         |
| `Connection refused —` / `Can't reach the API server —` / `No internet route —` / `Couldn't connect through your proxy` / `Connection dropped`, each with an error code in parentheses                                                                               | [Rete](#unable-to-connect-to-api)                                                                                                         |
| `Unable to connect to Anthropic services` during setup                                                                                                                                                                                                               | [Rete](#unable-to-connect-to-anthropic-services)                                                                                          |
| `Socket is closed`                                                                                                                                                                                                                                                   | [Rete](#socket-is-closed)                                                                                                                 |
| `Waiting for API response · will retry in`                                                                                                                                                                                                                           | [Tentativi automatici](#automatic-retries), oppure [Rete](#unable-to-connect-to-api) se persiste                                          |
| `API returned an empty or malformed response`                                                                                                                                                                                                                        | [Rete](#api-returned-an-empty-or-malformed-response)                                                                                      |
| `Streaming response ended before any complete data was received`                                                                                                                                                                                                     | [Rete](#streaming-response-ended-before-any-complete-data-was-received)                                                                   |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"`                                                                                                                                                                   | [Rete](#bedrock-streaming-response-has-an-unexpected-content-type)                                                                        |
| `SSL certificate verification failed`                                                                                                                                                                                                                                | [Rete](#ssl-certificate-errors)                                                                                                           |
| `SSL certificate error (...)` during login or startup                                                                                                                                                                                                                | [Rete](#ssl-certificate-errors)                                                                                                           |
| `unable to get local issuer certificate`                                                                                                                                                                                                                             | [Rete](#ssl-certificate-errors)                                                                                                           |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                                                                                                                                                                                           | [Rete](#host-not-allowed-in-a-cloud-session)                                                                                              |
| `proxy refused the connection`                                                                                                                                                                                                                                       | [Rete](#the-proxy-refused-the-connection)                                                                                                 |
| `403` with `This GraphQL query is not enabled for this session` in a cloud session                                                                                                                                                                                   | [GitHub proxy](/docs/it/cloud-environments#github-proxy)                                                                                       |
| `The cloud environments service returned an empty response` / `The cloud environments service returned a response in an unexpected format`                                                                                                                           | [Rete](#the-cloud-environments-service-returned-an-empty-or-unexpected-response)                                                          |
| `Couldn't reconnect to your Remote Control session`                                                                                                                                                                                                                  | [Rete](#couldnt-reconnect-to-your-remote-control-session)                                                                                 |
| `N sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.`                                                                                                                                               | [Rete](#sessions-ended-while-this-machine-was-offline)                                                                                    |
| `Couldn't share the transcript.`                                                                                                                                                                                                                                     | [Rete](#couldnt-share-the-transcript)                                                                                                     |
| `Prompt is too long` / `Input is too long for requested model`                                                                                                                                                                                                       | [Errori di richiesta](#prompt-is-too-long)                                                                                                |
| `Prompt is too long · automatic compaction failed:`                                                                                                                                                                                                                  | [Errori di richiesta](#prompt-is-too-long)                                                                                                |
| `Prompt is too long · this conversation is a single exchange` / `A single-exchange conversation cannot be compacted`                                                                                                                                                 | [Errori di richiesta](#prompt-is-too-long)                                                                                                |
| `Context limit reached · /compact or /clear to continue`                                                                                                                                                                                                             | [Errori di richiesta](#prompt-is-too-long)                                                                                                |
| `Context limit reached · /clear to continue`                                                                                                                                                                                                                         | [Errori di richiesta](#prompt-is-too-long)                                                                                                |
| `capability_rejected: prompt_too_long` on a Claude apps gateway session                                                                                                                                                                                              | [Errori di richiesta](#prompt-is-too-long)                                                                                                |
| `upstream rejected the request` / `request too large for this upstream` on a Claude apps gateway session                                                                                                                                                             | [Messaggi di errore upstream](/docs/it/claude-apps-gateway-config#upstream-error-messages)                                                     |
| `upstream rate limit exceeded` on a Claude apps gateway session                                                                                                                                                                                                      | [Messaggi di errore upstream](/docs/it/claude-apps-gateway-config#upstream-error-messages)                                                     |
| `all upstreams failed (N attempted)` on a Claude apps gateway session                                                                                                                                                                                                | [Messaggi di errore upstream](/docs/it/claude-apps-gateway-config#upstream-error-messages)                                                     |
| `Claude Code may not be enabled for your organization` after a Claude apps gateway sign-in                                                                                                                                                                           | [Risoluzione dei problemi del gateway delle app Claude](/docs/it/claude-apps-gateway-deploy#troubleshooting)                                   |
| `Context exceeds the ...-token limit by ... tokens` in `/context` output                                                                                                                                                                                             | [Errori di richiesta](#context-exceeds-the-token-limit)                                                                                   |
| `Error during compaction: Conversation too long`                                                                                                                                                                                                                     | [Errori di richiesta](#error-during-compaction-conversation-too-long)                                                                     |
| `Request too large`                                                                                                                                                                                                                                                  | [Errori di richiesta](#request-too-large)                                                                                                 |
| `Request too large for the API's 32MB request limit`                                                                                                                                                                                                                 | [Errori di richiesta](#request-too-large)                                                                                                 |
| `Image was too large`                                                                                                                                                                                                                                                | [Errori di richiesta](#image-was-too-large)                                                                                               |
| `Unable to resize image`                                                                                                                                                                                                                                             | [Errori di richiesta](#unable-to-resize-image)                                                                                            |
| `PDF too large` / `PDF is password protected`                                                                                                                                                                                                                        | [Errori di richiesta](#pdf-errors)                                                                                                        |
| `Extra inputs are not permitted`                                                                                                                                                                                                                                     | [Errori di richiesta](#extra-inputs-are-not-permitted)                                                                                    |
| `API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid` / `Property keys should match pattern`                                                                                                                                                      | [Errori di richiesta](#tool-input-schema-is-invalid)                                                                                      |
| `There's an issue with the selected model`                                                                                                                                                                                                                           | [Errori di richiesta](#theres-an-issue-with-the-selected-model)                                                                           |
| `Model ... is not a recognized model id`                                                                                                                                                                                                                             | [Errori di richiesta](#model-is-not-a-recognized-model-id)                                                                                |
| `Model ... not found`                                                                                                                                                                                                                                                | [Errori di richiesta](#model-not-found)                                                                                                   |
| `Claude Opus is not available with the Claude Pro plan`                                                                                                                                                                                                              | [Errori di richiesta](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                             |
| `Claude Code ... does not support this model; version ... or newer is required`                                                                                                                                                                                      | [Errori di richiesta](#claude-code-does-not-support-this-model)                                                                           |
| `Claude Code ... is older than the minimum version required by your organization's policy`                                                                                                                                                                           | [Errori di richiesta](#claude-code-does-not-support-this-model)                                                                           |
| `Model ... is restricted by your organization's settings`                                                                                                                                                                                                            | [Errori di richiesta](#model-is-restricted-by-your-organizations-settings)                                                                |
| `Model switch ... blocked by a PreModelSwitch hook`                                                                                                                                                                                                                  | [Errori di richiesta](#model-switch-was-blocked-by-a-premodelswitch-hook)                                                                 |
| `couldn't save it as your default` / `couldn't confirm it was saved as your default`                                                                                                                                                                                 | [Errori di richiesta](#couldnt-save-it-as-your-default)                                                                                   |
| `thinking.type.enabled is not supported for this model`                                                                                                                                                                                                              | [Errori di richiesta](#thinking-type-enabled-is-not-supported-for-this-model)                                                             |
| `Effort '<level>' isn't available with thinking turned off on this model`                                                                                                                                                                                            | [Errori di richiesta](#effort-isnt-available-with-thinking-turned-off)                                                                    |
| `effort '<level>' is not supported when thinking is disabled`                                                                                                                                                                                                        | [Errori di richiesta](#effort-isnt-available-with-thinking-turned-off)                                                                    |
| `max_tokens must be greater than thinking.budget_tokens`                                                                                                                                                                                                             | [Errori di richiesta](#thinking-budget-exceeds-output-limit)                                                                              |
| `API Error: 400 due to tool use concurrency issues`                                                                                                                                                                                                                  | [Errori di richiesta](#tool-use-or-thinking-block-mismatch)                                                                               |
| `API Error: 400 orphaned tool_result in conversation history`                                                                                                                                                                                                        | [Errori di richiesta](#tool-use-or-thinking-block-mismatch)                                                                               |
| `API Error: 400 duplicate tool_use ID in conversation history`                                                                                                                                                                                                       | [Errori di richiesta](#tool-use-or-thinking-block-mismatch)                                                                               |
| `[Unsupported tool content removed]`                                                                                                                                                                                                                                 | [Errori di richiesta](#unsupported-tool-content-removed)                                                                                  |
| `role 'system' must precede an 'assistant' message`                                                                                                                                                                                                                  | [Errori di richiesta](#role-system-must-precede-an-assistant-message)                                                                     |
| `Invalid encrypted_content in search_result block` / `Invalid encrypted_index in text block` / `Failed to decrypt web search result content`                                                                                                                         | [Errori di richiesta](#invalid-encrypted-content-in-search-result-block)                                                                  |
| `server_tool_use.name: Input should be` on every turn of a resumed session                                                                                                                                                                                           | [Errori di richiesta](#unsupported-tool-content-removed)                                                                                  |
| `<model> can't help with this. Start a new session to continue`                                                                                                                                                                                                      | [Errori di richiesta](#usage-policy-refusal)                                                                                              |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`                                                                                                                                                                        | [Errori di richiesta](#usage-policy-refusal)                                                                                              |
| `<model>'s safeguards flagged this message`                                                                                                                                                                                                                          | [Errori di richiesta](#safety-measures-flagged-a-cybersecurity-topic)                                                                     |
| `Opus 5.5's safeguards flagged this session`                                                                                                                                                                                                                         | [Errori di richiesta](#safety-measures-flagged-a-cybersecurity-topic)                                                                     |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                                                                                                                                                                                    | [Errori di richiesta](#safety-measures-flagged-a-cybersecurity-topic)                                                                     |
| `Installation was killed before it could finish (exit code 137)`                                                                                                                                                                                                     | [Errori di installazione](#installation-was-killed-before-it-could-finish)                                                                |
| `The connection dropped while downloading the update`                                                                                                                                                                                                                | [Errori di installazione](#the-connection-dropped-while-downloading-the-update)                                                           |
| `Download timed out: exceeded the total deadline`                                                                                                                                                                                                                    | [Errori di installazione](#the-connection-dropped-while-downloading-the-update)                                                           |
| `--bg and --print conflict`                                                                                                                                                                                                                                          | [Errori della riga di comando](#command-line-errors)                                                                                      |
| `Cloud sessions cannot be created from a --restricted session`                                                                                                                                                                                                       | [Errori della riga di comando](#cloud-sessions-cannot-be-created-from-a-restricted-session)                                               |
| `Cloud sessions are disabled by your organization's policy`                                                                                                                                                                                                          | [Errori della riga di comando](#cloud-sessions-are-disabled-by-your-organizations-policy)                                                 |
| `Couldn't verify your organization's policy for cloud sessions`                                                                                                                                                                                                      | [Errori della riga di comando](#cloud-sessions-are-disabled-by-your-organizations-policy)                                                 |
| `Error: --json-schema is not a valid JSON Schema`                                                                                                                                                                                                                    | [Errori della riga di comando](#command-line-errors)                                                                                      |
| `Error: Invalid --agents configuration:`                                                                                                                                                                                                                             | [Errori della riga di comando](#invalid-agents-configuration)                                                                             |
| `Error: Settings file exceeds the 2MiB limit`                                                                                                                                                                                                                        | [Errori della riga di comando](#settings-file-exceeds-the-2mib-limit)                                                                     |
| `The current directory no longer exists (it was deleted or moved)` / `Can't read the current directory`                                                                                                                                                              | [Errori della riga di comando](#the-current-directory-no-longer-exists)                                                                   |
| `Temp directory <dir> ... Refusing to use it` / `ENOSPC: no space left on device, mkdir '<dir>'`                                                                                                                                                                     | [Errori della riga di comando](#temp-directory-refused-or-cannot-be-created)                                                              |
| `couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded`                                                                                                                                                                        | [Errori della riga di comando](#directory-couldnt-be-resolved-to-a-real-location)                                                         |
| `Error: Workspace not trusted` when starting Remote Control                                                                                                                                                                                                          | [Errori della riga di comando](#workspace-not-trusted-when-starting-remote-control)                                                       |
| `` `<flag>` before `remote-control` is not carried over to the sessions Remote Control starts ``                                                                                                                                                                     | [Errori della riga di comando](#not-carried-over-to-the-sessions-remote-control-starts)                                                   |
| `` `claude import` is not yet available in this build ``                                                                                                                                                                                                             | [Errori della riga di comando](#claude-import-is-not-yet-available-in-this-build)                                                         |
| `Could not read Claude Code config`                                                                                                                                                                                                                                  | [Errori della riga di comando](#could-not-read-claude-code-config)                                                                        |
| `Could not import <server>: <reason>`                                                                                                                                                                                                                                | [Errori della riga di comando](#could-not-import-a-server-from-claude-desktop)                                                            |
| `Cannot add MCP server to scope: managed`                                                                                                                                                                                                                            | [Errori della riga di comando](#cannot-add-mcp-server-to-the-managed-scope)                                                               |
| `is Anthropic-hosted and doesn't support local OAuth`                                                                                                                                                                                                                | [Errori della riga di comando](#anthropic-hosted-and-doesnt-support-local-oauth)                                                          |
| `Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes`                                                                                                                                                                                      | [Errori della riga di comando](#cant-read-mcp-json)                                                                                       |
| `Server rejected the Authorization header minted by the configured headersHelper`                                                                                                                                                                                    | [Errori della riga di comando](#server-rejected-the-authorization-header-minted-by-the-configured-headershelper)                          |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                                                                                                                                                                                             | [Errori della riga di comando](#mcp-permission-prompt-tool-not-found)                                                                     |
| `OAuth callback port <port> is already in use — another process may be holding it`                                                                                                                                                                                   | [Errori della riga di comando](#oauth-callback-port-is-already-in-use)                                                                    |
| `No available ports for OAuth redirect`                                                                                                                                                                                                                              | [Errori della riga di comando](#no-available-ports-for-oauth-redirect)                                                                    |
| `Shell command failed for pattern "..."`, from `/security-review` or any skill that injects dynamic context                                                                                                                                                          | [Errori della riga di comando](#security-review-fails-without-origin-head)                                                                |
| `Shell command permission check failed for pattern "..."`, from a skill that injects dynamic context                                                                                                                                                                 | [Errori della riga di comando](#security-review-fails-without-origin-head)                                                                |
| ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``                                                                                                                                                                             | [Errori della riga di comando](#security-review-fails-without-origin-head)                                                                |
| `Input must be provided either through stdin or as a prompt argument when using --print`                                                                                                                                                                             | [Errori della riga di comando](#input-must-be-provided-when-using-print)                                                                  |
| `Error: Input contained only whitespace`                                                                                                                                                                                                                             | [Errori della riga di comando](#input-contained-only-whitespace)                                                                          |
| `Blank prompt — the message was only whitespace, so nothing was sent to the model.`                                                                                                                                                                                  | [Errori della riga di comando](#input-contained-only-whitespace)                                                                          |
| `Error: stream-json input carried over 256M characters with no newline`                                                                                                                                                                                              | [Errori della riga di comando](#stream-json-input-carried-over-256m-characters-with-no-newline)                                           |
| `Unknown command: /<name>`, with or without a `Did you mean` suggestion                                                                                                                                                                                              | [Errori della riga di comando](#unknown-command)                                                                                          |
| `Diff is too large for ultrareview` / `PR #<N> is too large for ultrareview`                                                                                                                                                                                         | [Errori della riga di comando](#diff-is-too-large-for-ultrareview)                                                                        |
| `Could not find merge-base with <branch>`                                                                                                                                                                                                                            | [Errori della riga di comando](#could-not-find-merge-base-with-the-base-branch)                                                           |
| `Your checkout has no branches (detached HEAD only)`                                                                                                                                                                                                                 | [Errori della riga di comando](#your-checkout-has-no-branches)                                                                            |
| `Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected`                                                                                                                                     | [Errori della riga di comando](#no-github-account-is-connected-to-your-claude-account)                                                    |
| `Your connected GitHub account can't see <owner>/<repo>`                                                                                                                                                                                                             | [Errori della riga di comando](#your-connected-github-account-cant-see-the-repository)                                                    |
| `The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead`                                                                                                                                           | [Errori della riga di comando](#the-github-app-preflight-failed-transiently)                                                              |
| `GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud`                                                                                                                                                                     | [Errori della riga di comando](#github-isnt-connected-to-your-claude-account)                                                             |
| `Single sign-on authorization needed`                                                                                                                                                                                                                                | [Errori della riga di comando](#single-sign-on-authorization-needed)                                                                      |
| `Failed to resume the conversation`                                                                                                                                                                                                                                  | [Errori della riga di comando](#failed-to-resume-the-conversation)                                                                        |
| `No conversation found with session ID: <session-id>`                                                                                                                                                                                                                | [Errori della riga di comando](#no-conversation-found-with-the-session-id)                                                                |
| `Cannot switch renderers in this session`                                                                                                                                                                                                                            | [Errori della riga di comando](#cannot-switch-renderers-in-this-session)                                                                  |
| `Cannot switch renderers while work is running in the background`                                                                                                                                                                                                    | [Errori della riga di comando](#cannot-switch-renderers-in-this-session)                                                                  |
| `Couldn't open Claude Desktop`                                                                                                                                                                                                                                       | [Errori della riga di comando](#couldnt-open-claude-desktop)                                                                              |
| `Failed to open Claude Desktop. Please try opening it manually.`                                                                                                                                                                                                     | [Errori della riga di comando](#couldnt-open-claude-desktop)                                                                              |
| `Couldn't read your Zed keymap` / `Couldn't back up your Zed keymap` / `Couldn't update your Zed keymap`                                                                                                                                                             | [Errori della riga di comando](#terminal-setup-left-your-zed-keymap-unchanged)                                                            |
| `Your Zed keymap isn't a readable list of keybindings`                                                                                                                                                                                                               | [Errori della riga di comando](#terminal-setup-left-your-zed-keymap-unchanged)                                                            |
| `Skill usage reports are not available on this connection.`                                                                                                                                                                                                          | [Errori della riga di comando](#skill-usage-reports-are-not-available-on-this-connection)                                                 |
| `Custom output styles can't be selected over Remote Control or from a relayed message`                                                                                                                                                                               | [Errori della riga di comando](#custom-output-styles-cant-be-selected-over-remote-control)                                                |
| `Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load`                                                                                                                                                           | [Errori della riga di comando](#output-styles-are-saved-to-local-settings-which-this-session-doesnt-load)                                 |
| `` `plugin eval` is currently in early access `` / `` `plugin eval` is currently unavailable ``                                                                                                                                                                      | [Errori dei plugin](#plugin-eval-is-currently-in-early-access)                                                                            |
| `Marketplace "<name>" is registered from an untrusted source`                                                                                                                                                                                                        | [Errori dei plugin](#marketplace-is-registered-from-an-untrusted-source)                                                                  |
| `Marketplace "<name>" is already added from a different source`                                                                                                                                                                                                      | [Errori dei plugin](#marketplace-is-already-added-from-a-different-source)                                                                |
| `"<name>" is another spelling of "<reserved>", a reserved marketplace name`                                                                                                                                                                                          | [Errori dei plugin](#marketplace-name-is-another-spelling-of-a-reserved-name)                                                             |
| `references ${user_config.*} in a shell-form command`                                                                                                                                                                                                                | [Errori dei plugin](#plugin-command-references-user-config)                                                                               |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                                                                                                                                                                                   | [Errori dei plugin](#plugin-command-references-user-config)                                                                               |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                                                                                                                                                                                  | [Errori dei plugin](#plugin-command-references-user-config)                                                                               |
| `Plugin archive integrity check failed`                                                                                                                                                                                                                              | [Errori dei plugin](#plugin-archive-integrity-check-failed)                                                                               |
| `path escapes plugin directory`                                                                                                                                                                                                                                      | [Errori dei plugin](#path-escapes-plugin-directory)                                                                                       |
| `path could not be checked`                                                                                                                                                                                                                                          | [Errori dei plugin](#path-could-not-be-checked)                                                                                           |
| `its marketplace entry path does not stay inside the marketplace directory`                                                                                                                                                                                          | [Errori dei plugin](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                               |
| `Plugin source path refused`                                                                                                                                                                                                                                         | [Errori dei plugin](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                               |
| `Failed to load marketplace configuration`                                                                                                                                                                                                                           | [Errori dei plugin](#failed-to-load-marketplace-configuration)                                                                            |
| `Marketplace configuration file is corrupted`                                                                                                                                                                                                                        | [Errori dei plugin](#failed-to-load-marketplace-configuration)                                                                            |
| `Plugin "<name>@synced" is required by your organization and can't be disabled here`                                                                                                                                                                                 | [Errori dei plugin](#plugin-is-required-by-your-organization)                                                                             |
| `would be spawned with zero tools — refusing`                                                                                                                                                                                                                        | [Errori degli strumenti](#agent-would-be-spawned-with-zero-tools)                                                                         |
| `File is covered by a Read deny rule in your permission settings`                                                                                                                                                                                                    | [Errori degli strumenti](#file-is-covered-by-a-read-deny-rule)                                                                            |
| `subagent_type is required: the general-purpose agent is not available in this session`                                                                                                                                                                              | [Errori degli strumenti](#subagent-type-is-required)                                                                                      |
| `Error: this write left the memory index at MEMORY.md at ..., over its ... read limit`                                                                                                                                                                               | [Errori degli strumenti](#memory-index-is-over-its-read-limit)                                                                            |
| `pkill: refusing to run`                                                                                                                                                                                                                                             | [Errori degli strumenti](#pkill-pattern-matches-the-claude-code-process)                                                                  |
| `Failed to write to <name>'s inbox — nothing was sent`                                                                                                                                                                                                               | [Errori degli strumenti](#failed-to-write-to-a-teammate-inbox)                                                                            |
| `Failed to write the plan approval request to the lead's inbox — plan not submitted`                                                                                                                                                                                 | [Errori degli strumenti](#failed-to-write-to-a-teammate-inbox)                                                                            |
| `Its agent definition was not restored: the folder its definition file came from is not trusted`                                                                                                                                                                     | [Errori degli strumenti](#teammate-agent-definition-not-restored)                                                                         |
| `Message too large for cross-session delivery`                                                                                                                                                                                                                       | [Errori degli strumenti](#message-too-large-for-cross-session-delivery)                                                                   |
| `Too many messages to this session just now`                                                                                                                                                                                                                         | [Errori degli strumenti](#too-many-messages-to-this-session-just-now)                                                                     |
| `Refusing to send: reply target is a symlink` / `Refusing to send: cannot vet reply target`                                                                                                                                                                          | [Errori degli strumenti](#refusing-to-send-a-cross-session-message)                                                                       |
| `Refusing to send: connected endpoint is not the expected process` / `Refusing to send: connected endpoint identity could not be read`                                                                                                                               | [Errori degli strumenti](#refusing-to-send-a-cross-session-message)                                                                       |
| `Refusing to send: connected endpoint is not owned by this user` / `Refusing to send: connected endpoint owner could not be read`                                                                                                                                    | [Errori degli strumenti](#refusing-to-send-a-cross-session-message)                                                                       |
| `Refusing to send: connected endpoint is a different process with the expected pid`                                                                                                                                                                                  | [Errori degli strumenti](#refusing-to-send-a-cross-session-message)                                                                       |
| `Refusing to read <path>: its symlink resolution changed after permission was checked (<reason>)` / `Refusing to search <path>: its symlink resolution changed after permission was checked`                                                                         | [Errori degli strumenti](#refusing-after-a-symlink-changed)                                                                               |
| `Refusing to write <path>: its parent-directory symlink resolution changed after permission was checked` / `Refusing to write <path>: it is a symbolic link. Write to the link's target path instead`                                                                | [Errori degli strumenti](#refusing-after-a-symlink-changed)                                                                               |
| `Refusing to write through symlink: <path>` / `Refusing to write into symlinked directory: <path>`                                                                                                                                                                   | [Errori degli strumenti](#refusing-after-a-symlink-changed)                                                                               |
| `Refusing to search <path>: a path one of its Read deny rules is written through changed while the search was being prepared` / `Refusing to search <path>: it could not be opened`                                                                                  | [Errori degli strumenti](#refusing-after-a-symlink-changed)                                                                               |
| `its permission check expired before it ran (too many concurrent file operations)` / `ripgrep was found only by name on PATH`                                                                                                                                        | [Errori degli strumenti](#refusing-after-a-symlink-changed)                                                                               |
| `task output swap refused (tasks dir moved or linked)`                                                                                                                                                                                                               | [Errori degli strumenti](#task-output-swap-refused)                                                                                       |
| `Command killed: its output file was replaced or could no longer be verified`                                                                                                                                                                                        | [Errori degli strumenti](#task-output-swap-refused)                                                                                       |
| `the source file is not valid UTF-8 text` / `the source file is not valid UTF-16 text`                                                                                                                                                                               | [Errori degli strumenti](#the-source-file-is-not-valid-utf-8-text)                                                                        |
| `the source file has the replacement character U+FFFD`                                                                                                                                                                                                               | [Errori degli strumenti](#the-source-file-is-not-valid-utf-8-text)                                                                        |
| `Reading a local file from outside this session's connected folders, or through a link, needs the approval card`                                                                                                                                                     | [Errori degli strumenti](#reading-a-local-file-from-outside-the-connected-folders)                                                        |
| `cannot read file_path (...) — the file could not be examined, and no one can answer the approval card`                                                                                                                                                              | [Errori degli strumenti](#reading-a-local-file-from-outside-the-connected-folders)                                                        |
| `WebFetch cannot fetch localhost or other hostnames without a dot`                                                                                                                                                                                                   | [Errori degli strumenti](#webfetch-cannot-fetch-localhost)                                                                                |
| `Can't open MCP settings while no terminal is attached to this background session`                                                                                                                                                                                   | [Errori della sessione in background](#commands-refused-in-a-background-session)                                                          |
| `Can't open MCP settings in a background session`                                                                                                                                                                                                                    | [Errori della sessione in background](#commands-refused-in-a-background-session)                                                          |
| `blocked because the path is spelled in a form that cannot be safely resolved`                                                                                                                                                                                       | [Errori della sessione in background](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved)                               |
| `blocked because the path is network-shaped`                                                                                                                                                                                                                         | [Errori della sessione in background](#write-or-command-blocked-because-the-path-names-a-network-location)                                |
| `is isolated in the worktree <path>, but this command <reason>. Refusing to run it`                                                                                                                                                                                  | [Errori della sessione in background](#command-blocked-by-the-worktree-isolation-checks)                                                  |
| `too complex to verify that it stays inside the worktree`                                                                                                                                                                                                            | [Errori della sessione in background](#command-blocked-by-the-worktree-isolation-checks)                                                  |
| `This session has no saved transcript`                                                                                                                                                                                                                               | [Errori della sessione in background](#this-session-has-no-saved-transcript)                                                              |
| `Can't open — this session is running in another terminal`                                                                                                                                                                                                           | [Errori della sessione in background](#this-session-is-running-in-another-terminal)                                                       |
| `This conversation is already open in another running Claude session`                                                                                                                                                                                                | [Errori della sessione in background](#this-session-is-running-in-another-terminal)                                                       |
| `This session's saved conversation is no longer on disk`                                                                                                                                                                                                             | [Errori della sessione in background](#this-sessions-saved-conversation-is-no-longer-on-disk)                                             |
| `kept <id> — its worktree is still at <path>`                                                                                                                                                                                                                        | [Errori della sessione in background](#worktree-has-commits-that-are-not-pushed-anywhere)                                                 |
| `kept <id> — <n> unpushed commits on <branch>`                                                                                                                                                                                                                       | [Errori della sessione in background](#worktree-has-commits-that-are-not-pushed-anywhere)                                                 |
| `kept <id> — worktree has commits that are not pushed anywhere`                                                                                                                                                                                                      | [Errori della sessione in background](#worktree-has-commits-that-are-not-pushed-anywhere)                                                 |
| `terminal host process died — press Enter to restart` / `This session's terminal host process died`                                                                                                                                                                  | [Errori della sessione in background](#terminal-host-process-died)                                                                        |
| `Session isn't responding` / `Press enter again to restart this session — it isn't responding`                                                                                                                                                                       | [Errori della sessione in background](#session-isnt-responding)                                                                           |
| `Session <id> was stopped while the respawn was in flight`                                                                                                                                                                                                           | [Errori della sessione in background](#session-was-stopped-while-the-respawn-was-in-flight)                                               |
| `This session was running agent '<name>', which is no longer available`                                                                                                                                                                                              | [Errori della sessione in background](#session-agent-no-longer-available)                                                                 |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                                                                                                                                                                                          | [Errori della sessione in background](#claude_code_process_wrapper-launcher-errors)                                                       |
| `EUNKNOWN: unknown error, uv_spawn`                                                                                                                                                                                                                                  | [Errori della sessione in background](#eunknown-when-starting-a-background-session)                                                       |
| `EACCES: permission denied, posix_spawn`                                                                                                                                                                                                                             | [Errori della sessione in background](#eacces-when-starting-a-background-session)                                                         |
| `exited before it became reachable`                                                                                                                                                                                                                                  | [Errori della sessione in background](#background-service-exited-before-it-became-reachable)                                              |
| `Couldn't start a background session (working directory no longer exists or is not accessible: ...)`                                                                                                                                                                 | [Errori della sessione in background](#working-directory-no-longer-exists-when-starting-a-background-session)                             |
| `Claude Code is being updated by npm on this machine (still not runnable after 2 min, ...)`                                                                                                                                                                          | [Errori della sessione in background](#eacces-when-starting-a-background-session)                                                         |
| `Claude Code process exited with code N`                                                                                                                                                                                                                             | [Errori del wrapper e dell'IDE](#claude-code-process-exited-with-code-n)                                                                  |
| `The connection to Claude Code ended before this message completed`                                                                                                                                                                                                  | [Errori del wrapper e dell'IDE](#the-connection-to-claude-code-ended-before-this-message-completed)                                       |
| `Could not locate the Claude CLI on PATH`                                                                                                                                                                                                                            | [Errori del wrapper e dell'IDE](#could-not-locate-the-claude-cli-on-path)                                                                 |
| `Restored the code, but skipped N files`                                                                                                                                                                                                                             | [Avvisi e errori di Rewind](#restored-the-code-but-skipped-files)                                                                         |
| `No files were restored: N files failed (backup missing, or the file could not be updated)`                                                                                                                                                                          | [Avvisi e errori di Rewind](#no-files-were-restored)                                                                                      |
| `Transcript writes are failing (...)`                                                                                                                                                                                                                                | [Avvisi di salvataggio della sessione](#transcript-writes-are-failing)                                                                    |
| `Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set`                                                                                                                                                                                                  | [Avvisi di salvataggio della sessione](#transcript-saving-is-off-skip-prompt-history)                                                     |
| `Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker`                                                                                                                                                                                              | [Avvisi di salvataggio della sessione](#transcript-saving-is-off-child-session-marker)                                                    |
| `Claude Code's fullscreen renderer didn't finish starting last time on this machine` / `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`                                                                                            | [Avvisi di configurazione](#fullscreen-failed-start-notice)                                                                               |
| `Claude Code exited after an unrecoverable interface error (...)`                                                                                                                                                                                                    | [Avvisi di configurazione](#exited-after-an-unrecoverable-interface-error)                                                                |
| `Agent descriptions are over the 15.0k-token limit`                                                                                                                                                                                                                  | [Avvisi di configurazione](#agent-descriptions-are-over-the-15000-token-limit)                                                            |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                                                                                                                                                                                  | [Avvisi di configurazione](#workspace-has-not-been-trusted)                                                                               |
| `is a network path, which cannot be added as a working directory`                                                                                                                                                                                                    | [Avvisi di configurazione](#working-directory-is-a-network-path)                                                                          |
| `Remote managed settings failed to load (<cause>)`                                                                                                                                                                                                                   | [Avvisi di configurazione](#remote-managed-settings-failed-to-load)                                                                       |
| `Managed settings were not approved; exiting without applying them.`                                                                                                                                                                                                 | [Avvisi di configurazione](#managed-settings-were-not-approved)                                                                           |
| `MCP server <name> is blocked by enterprise managed policy`                                                                                                                                                                                                          | [Avvisi di configurazione](#mcp-server-is-blocked-by-enterprise-managed-policy)                                                           |
| `Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.`                                                                                                                                              | [Avvisi di configurazione](#managed-settings-document-could-not-be-parsed)                                                                |
| `Managed settings drop-in directory could not be read`                                                                                                                                                                                                               | [Avvisi di configurazione](#managed-settings-document-could-not-be-parsed)                                                                |
| `otelHeadersHelper failed; telemetry is not being exported. See /status: ...`                                                                                                                                                                                        | [Avvisi di configurazione](#otelheadershelper-failed)                                                                                     |
| `"crossSessionInbound" must be one of "accept", "hold", "refuse"`                                                                                                                                                                                                    | [Avvisi di configurazione](#crosssessioninbound-must-be-one-of-accept-hold-refuse)                                                        |
| `headersHelper not run — this workspace has no persisted trust`                                                                                                                                                                                                      | [Avvisi di configurazione](#headershelper-not-run)                                                                                        |
| `Invalid permission rule "..." was skipped: Malformed Tool(content) rule`                                                                                                                                                                                            | [Avvisi di configurazione](#malformed-tool-content-rule)                                                                                  |
| `... is not matched by file permission checks`                                                                                                                                                                                                                       | [Avvisi di configurazione](#is-not-matched-by-file-permission-checks)                                                                     |
| `... has a wildcard before the rest of the command`                                                                                                                                                                                                                  | [Avvisi di configurazione](#has-a-wildcard-before-the-rest-of-the-command)                                                                |
| `CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced`                                                                                                                                                                                           | [Avvisi di configurazione](#the-200k-limit-isnt-enforced)                                                                                 |
| `[claude-code:unrecognized_model]`                                                                                                                                                                                                                                   | [Avvisi di configurazione](#unrecognized-model-id-on-a-request)                                                                           |
| `Stale sandbox mask files left by a killed session`                                                                                                                                                                                                                  | [Avvisi di configurazione](#stale-sandbox-mask-files-left-by-a-killed-session)                                                            |
| Le risposte sembrano di qualità inferiore al solito                                                                                                                                                                                                                  | [Qualità della risposta](#responses-seem-lower-quality-than-usual)                                                                        |

<h2 id="automatic-retries">
  Tentativi automatici
</h2>

Claude Code ritenta i guasti transitori fino a 10 volte con backoff esponenziale prima di mostrarti un errore. Non sempre ritenta un guasto che arriva a metà della risposta di Claude. Quando vedi uno degli errori in questa pagina, Claude Code ha già effettuato i tentativi che si applicano a quel guasto; gli elenchi sottostanti indicano quali guasti ottengono il budget completo, quali ne ottengono uno più piccolo e quali non ne ottengono nessuno.

Claude Code ritenta questi guasti:

* Errori del server, risposte sovraccariche e timeout delle richieste che arrivano prima che una qualsiasi risposta di Claude sia stata trasmessa.
* Connessioni interrotte. Quando una connessione si interrompe a metà di una richiesta prima che Claude abbia completato una qualsiasi parte della sua risposta, incluso il suo thinking, Claude Code invia nuovamente la richiesta con lo stesso backoff e il turno continua, anche se del testo aveva già iniziato a essere trasmesso. Quando si interrompe dopo che Claude ha finito di pensare ma prima di aver iniziato un testo o una chiamata di strumento, Claude Code invece invia nuovamente la richiesta fino a due volte in rapida successione, e termina il turno con `Connection lost before a response was produced` se la connessione continua a interrompersi a quel punto.
* Una connessione che Claude Code rileva è stata interrotta dal tuo computer che si è addormentato a metà di una richiesta. Claude Code la conta come una connessione interrotta secondo le regole sopra; una volta che l'etichetta di riprovazione nomina il motivo specifico, legge `Connection lost while your computer was asleep`, e se il turno termina dopo che Claude ha finito di pensare ma prima di un testo o di una chiamata di strumento, il messaggio legge `Your computer went to sleep before a response was produced`.
* Un flusso di risposta bloccato, quando le intestazioni di risposta sono arrivate ma nessuna risposta di Claude è arrivata, o quando Claude ha finito di pensare ma non ha iniziato un testo o una chiamata di strumento: Claude Code interrompe la connessione bloccata e invia nuovamente la richiesta al massimo una volta, al di fuori del budget di 10 tentativi sopra. Se la risposta si blocca una seconda volta dopo che Claude ha finito di pensare ma prima di un testo o di una chiamata di strumento, Claude Code termina il turno con `The response stalled before a response was produced`.
* Una richiesta di streaming a cui l'API non risponde mai con intestazioni di risposta, su una connessione dove il [first-byte deadline runs](/docs/it/network-config#streaming-idle-watchdogs): Claude Code la interrompe alla scadenza e la invia nuovamente al massimo una volta per richiesta di modello, entro il budget di riprovazione, quindi termina il turno con [No response from API](#no-response-from-api) se anche quel tentativo rimane senza risposta. Su altre connessioni, la richiesta attende `API_TIMEOUT_MS`. Quando imposti `CLAUDE_CODE_RETRY_WATCHDOG`, il limite di un tentativo non si applica.
* Throttle 429 temporanei, ma non il `429` del limite di spesa di un gateway, che non è un throttle; vedi [Spend limit reached](#spend-limit-reached).
  * Quando sei connesso con un abbonamento claude.ai, questo include throttle 429 che non portano le intestazioni di quota del tuo piano. Prima della v2.1.199, Claude Code ritentava questi throttle solo per le chiavi API e gli accessi Enterprise.
* Una richiesta rifiutata perché l'input più `max_tokens` supera il limite di contesto. Inviarla nuovamente invariata fallirebbe allo stesso modo, quindi Claude Code ritenta con un `max_tokens` ridotto, e smette di ritentare e compatta invece in due casi:
  * Quando nessuna riduzione può adattarsi, ad esempio quando la conversazione stessa riempie quasi la finestra di contesto.
  * Quando un tentativo non può ridurre ulteriormente `max_tokens`. Prima della v2.1.218, Claude Code poteva inviare nuovamente una richiesta ridotta che ancora non si adattava, ad esempio quando il budget di thinking esteso superava il contesto rimanente, fino a quando il budget di riprovazione non si esauriva.
* Una credenziale Google Cloud scaduta o mancante su [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), o credenziali AWS che non riescono a caricarsi sulla tua macchina. Claude Code scarta le sue credenziali memorizzate nella cache e ritenta fino a due volte, quindi segnala l'errore in modo che tu possa autenticarti di nuovo subito, come descritto in [Could not load AWS or Google Cloud credentials](#could-not-load-aws-or-google-cloud-credentials). Prima della v2.1.228, Claude Code ritentava una credenziale Google Cloud non riuscita attraverso il budget di riprovazione completo prima di mostrare l'errore.
* Un `401` o `403` dall'API Anthropic, direttamente o attraverso un [LLM gateway](/docs/it/llm-gateway), mentre uno script [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) fornisce la credenziale. Claude Code esegue nuovamente lo script e ritenta con il suo output aggiornato, entro il budget di riprovazione completo. Quando lo script stesso fallisce al nuovo tentativo, Claude Code mostra [Your apiKeyHelper script is failing](#your-apikeyhelper-script-is-failing) invece.

Prima della v2.1.227, `Connection lost before a response was produced` leggeva `Connection closed while thinking, before producing a response` e `The response stalled before a response was produced` leggeva `Response stalled while thinking, before producing a response`.

Claude Code non ritenta questi guasti:

* Un errore di convalida del certificato TLS, come un proxy che ispeziona TLS, un bundle `NODE_EXTRA_CA_CERTS` mancante, o un certificato scaduto. Claude Code segnala l'errore al primo tentativo, in modo che tu possa correggere subito la configurazione del certificato; vedi [SSL certificate errors](#ssl-certificate-errors). Claude Code ritenta comunque condizioni TLS transitorie come un timeout di handshake. Prima della v2.1.199, Claude Code ritentava i guasti dei certificati attraverso il budget di riprovazione completo prima di mostrare l'errore.
* Un errore del server, una connessione interrotta, o un flusso bloccato che arriva dopo che Claude ha completato un blocco di testo o una chiamata di strumento, o ne ha iniziato uno dopo aver finito il suo thinking, ma prima di finire la risposta. Claude Code non esegue nuovamente la richiesta, perché ciò potrebbe eseguire le stesse chiamate di strumento due volte. Mantiene ciò che Claude ha completato, esegue le chiamate di strumento che Claude ha finito, e continua il turno dai loro risultati. Per ciò che vedi in una sessione interattiva e in una non interattiva, leggi [The response above may be incomplete](#the-response-above-may-be-incomplete). Prima della v2.1.199, Claude Code scartava l'output parziale e segnalava l'intero turno come un errore quando un errore del server arrivava a metà del flusso.
* Un guasto che arriva dopo che Claude ha finito la risposta: non c'è nulla da ritentare, quindi Claude Code mantiene la risposta completa e termina il turno normalmente.
* Una [Amazon Bedrock streaming response with an unexpected content-type](#bedrock-streaming-response-has-an-unexpected-content-type), perché il gateway o il proxy che riscrive la risposta riscriverebbero il tentativo allo stesso modo. Richiede Claude Code v2.1.208 o successivo.
* Un tentativo non in streaming di una richiesta in streaming non riuscita che ottiene uno stato di successo ma [no Claude API message in the body](#api-returned-an-empty-or-malformed-response). Claude Code termina il turno con quell'errore.
* Una richiesta che il controllo della politica della tua organizzazione ha negato, che emerge come una riga `API Error:` che porta il messaggio di negazione. Gli amministratori della tua organizzazione hanno configurato il controllo con [Inference hooks](https://platform.claude.com/docs/en/manage-claude/inference-hooks), una funzione Claude Enterprise, e il messaggio termina con le istruzioni che hanno configurato, o per impostazione predefinita ti dice di contattarli. Claude Code non invia nuovamente la richiesta negata allo stesso modello o a un [fallback model](/docs/it/model-config#fallback-model-chains), perché il diniego riguarda il contenuto della richiesta piuttosto che il modello. Prima della v2.1.239, Claude Code poteva inviare nuovamente una richiesta negata, senza streaming o su un fallback model configurato, prima di mostrarti il diniego.

<h3 id="what-you-see-while-claude-code-retries-or-waits">
  Cosa vedi mentre Claude Code ritenta o attende
</h3>

Durante il tentativo, lo spinner mostra un countdown `Retrying in Ns · attempt x/y` dopo un'etichetta di errore. L'etichetta nomina il motivo specifico dal primo tentativo per i guasti su cui puoi agire subito: la rete è inattiva, un handshake TLS non è riuscito, o hai raggiunto un limite di velocità. Per altri errori legge `API error` all'inizio. A partire dalla v2.1.198 passa al motivo specifico dal terzo tentativo, o al tentativo finale quando `CLAUDE_CODE_MAX_RETRIES` consente meno di tre; le versioni precedenti passano solo al tentativo finale.

A partire dalla v2.1.198, il suggerimento dello spinner usuale è soppresso durante i tentativi. Una volta rivelato il motivo dell'errore, se il guasto è un sovraccarico 529 la riga sotto il countdown nomina anche dove controllare lo stato del servizio: `status.claude.com` sull'API Anthropic, o l'host del provider o del gateway nominato nel messaggio su altre configurazioni.

Se nessun dato arriva sul flusso di risposta per 20 secondi mentre una richiesta è ancora in sospeso, lo spinner mostra `Waiting for API response · will retry in … · check your network` prima che sia iniziato un tentativo. La richiesta non è ancora fallita: il countdown corre fino al punto in cui Claude Code interrompe la connessione bloccata. Dopo l'interruzione, ciò che vedi dipende da quanto lontano era arrivata la risposta:

* Prima che Claude abbia completato un blocco di testo o una chiamata di strumento, o ne abbia iniziato uno dopo aver finito il suo thinking, Claude Code ritenta la richiesta o termina il turno con un errore. [Automatic retries](#automatic-retries) dice quali blocchi ritenta e quante volte.
* Dopo che Claude ha completato un blocco di testo o una chiamata di strumento, o ne ha iniziato uno dopo aver finito il suo thinking, ma prima che Claude abbia finito la risposta, Claude Code mantiene ciò che Claude ha completato, continua il turno da qualsiasi chiamata di strumento che Claude ha finito, e mostra [The response above may be incomplete](#the-response-above-may-be-incomplete). In una sessione non interattiva, e per la risposta di un subagent in qualsiasi sessione, Claude Code potrebbe prima chiedere a Claude di continuare la risposta; quella voce dice quando lo fa e quando vedi ancora l'avviso lì.
* Dopo che Claude ha finito la risposta, Claude Code termina il turno normalmente.

Il banner si cancella da solo una volta che i dati riprendono o un tentativo ha successo. Se riappare ad ogni tentativo, trattalo come un [network issue](#unable-to-connect-to-api). Prima della v2.1.185, il banner appariva dopo 10 secondi con una formulazione diversa.

Mentre Claude sta consultando l'[advisor](/docs/it/advisor), il banner appare dopo 90 secondi senza dati invece di 20, perché una lunga revisione dell'advisor può non inviare nulla per ben oltre 20 secondi. Prima della v2.1.214, la soglia di 20 secondi si applicava anche durante le chiamate dell'advisor, quindi il banner appariva durante le revisioni dell'advisor anche quando non c'era nulla di sbagliato.

<h3 id="tune-retry-behavior">
  Sintonizza il comportamento dei tentativi
</h3>

Puoi sintonizzare il comportamento dei tentativi con queste variabili di ambiente:

| Variable                                              | Default | Effect                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------------------------------------------- | :------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/it/env-vars)             | 10      | Numero di tentativi di riprovazione. Limitato a 15 a partire dalla v2.1.186; a partire dalla v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` aumenta il valore predefinito e rimuove il limite. Abbassalo per far emergere i guasti più velocemente negli script.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/it/env-vars)          | unset   | Imposta su `1` in sessioni non presenziate come i lavori CI per ritentare gli errori di capacità `429` e `529` indefinitamente invece di fallire dopo `CLAUDE_CODE_MAX_RETRIES` tentativi. Claude Code fallisce immediatamente su un `429` che segnala un limite di spesa o crediti di utilizzo esauriti, anche uno da un [gateway spend cap](#spend-limit-reached) che si ripristina secondo una pianificazione. Prima della v2.1.239, il watchdog ritentava questi indefinitamente. Per le richieste in fast mode, vedi [Handle rate limits](/docs/it/fast-mode#handle-rate-limits). Sulla v2.1.199 o successivo aumenta anche il conteggio dei tentativi predefinito per altri errori transitori, come errori del server, timeout e connessioni interrotte, a 300, approssimativamente tre ore di backoff, e rimuove il limite di 15 su `CLAUDE_CODE_MAX_RETRIES` se imposti esplicitamente quella variabile. |
| [`API_TIMEOUT_MS`](/docs/it/env-vars)                      | 600000  | Timeout per richiesta in millisecondi. Aumentalo per reti lente o proxy. Limita anche quanto a lungo Claude Code attende le intestazioni di risposta, descritto in [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/it/env-vars) | unset   | Scadenza in millisecondi per il primo byte di risposta di una richiesta di streaming. Richiede Claude Code v2.1.242 o successivo. Per come Claude Code sceglie la scadenza quando questo non è impostato, vedi [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

<h2 id="server-errors">
  Errori del server
</h2>

La maggior parte di questi errori proviene dal provider di inferenza: il servizio Anthropic su Anthropic API e il servizio dietro l'endpoint di quel provider su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o un gateway personalizzato. [Auto mode cannot determine the safety of an action](#auto-mode-cannot-determine-the-safety-of-an-action) e [Agent terminated early due to an API error](#agent-terminated-early-due-to-an-api-error) coprono anche cause dal vostro lato, come un account Amazon Bedrock che non può invocare il modello di classificazione o un subagent che ha raggiunto un limite di utilizzo.

<h3 id="api-error-500-internal-server-error">
  API Error: 500 Internal server error
</h3>

Claude Code mostra il codice di stato e il messaggio di errore dell'API per qualsiasi risposta 5xx. L'esempio seguente mostra una risposta 500 su Anthropic API:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

La frase finale indica dove controllare lo stato del servizio e varia in base al provider. Le configurazioni di Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry indicano lo stato del servizio di quel provider. Un `ANTHROPIC_BASE_URL` personalizzato indica l'host del gateway.

Questo indica un errore imprevisto all'interno dell'API. Non è causato dal vostro prompt, dalle impostazioni o dall'account.

**Cosa fare:**

* Controllate [status.claude.com](https://status.claude.com) o la pagina di stato del provider indicata nel messaggio per gli incidenti attivi
* Aspettate un minuto, quindi inviate di nuovo il vostro messaggio. Il vostro messaggio originale è ancora nella conversazione, quindi per un prompt lungo potete digitare `try again` invece di incollare l'intera cosa.
* Se l'errore persiste senza alcun incidente pubblicato, eseguite `/feedback` in modo che Anthropic possa investigare con i dettagli della vostra richiesta. Vedete [Report an error](#report-an-error) se `/feedback` non è disponibile nel vostro ambiente.

<h3 id="api-error-repeated-529-overloaded-errors">
  API Error: Repeated 529 Overloaded errors
</h3>

L'API è temporaneamente al massimo della capacità per tutti gli utenti. Claude Code ha già riprovato più volte prima di mostrare questo messaggio:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

La frase finale varia in base al provider nello stesso modo dell'errore 500 sopra.

Un 529 non è il vostro limite di utilizzo e non conta rispetto alla vostra quota.

**Cosa fare:**

* Controllate [status.claude.com](https://status.claude.com) o la pagina di stato del provider indicata nel messaggio per gli avvisi di capacità
* Riprovate tra pochi minuti
* Eseguite `/model` e passate a un modello diverso per continuare a lavorare, poiché la capacità è tracciata per modello. Claude Code vi chiede di farlo quando un modello è sotto un carico particolarmente elevato, ad esempio `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Request timed out
</h3>

L'API non ha risposto prima della scadenza della connessione.

```text theme={null}
Request timed out
```

Questo può accadere durante periodi di carico elevato o quando il modello sta generando una risposta molto grande. Il timeout di richiesta predefinito è di 10 minuti.

**Cosa fare:**

* Riprovate la richiesta
* Per attività di lunga durata, suddividete il lavoro in prompt più piccoli
* Se la causa è una rete lenta o un proxy, aumentate `API_TIMEOUT_MS` come descritto in [Automatic retries](#automatic-retries)
* Se i timeout sono frequenti e la vostra rete è altrimenti sana, vedete [Network and connection errors](#network-and-connection-errors) di seguito

<h3 id="no-response-from-api">
  No response from API
</h3>

Claude Code ha inviato una richiesta di streaming e l'API non ha restituito intestazioni di risposta entro la scadenza per il primo byte, quindi Claude Code ha interrotto la richiesta invece di aspettare il timeout di richiesta completo `API_TIMEOUT_MS`, 10 minuti per impostazione predefinita. Claude Code invia di nuovo la richiesta al massimo una volta, se il [retry budget](#tune-retry-behavior) lo consente. Quando il nuovo tentativo rimane senza risposta, il turno termina con questo messaggio, che mostra quanto tempo ha aspettato ogni tentativo. Quando impostate [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/it/env-vars), il limite di un nuovo tentativo non si applica e Claude Code riprova secondo il budget descritto in [Tune retry behavior](#tune-retry-behavior).

```text theme={null}
API Error: No response from API (waited 3m, then 10m on the retry). If a proxy or gateway on your network holds responses until they complete, raise API_TIMEOUT_MS or CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS to wait longer.
```

Claude Code imposta l'attesa per le intestazioni di risposta del primo tentativo e l'attesa del nuovo tentativo separatamente:

* **First attempt**: [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/it/env-vars) quando lo impostate su 1 o più, limitato tra 10 secondi e 30 minuti. Altrimenti Claude Code utilizza il timeout del watchdog a livello di byte elencato in [Streaming idle watchdogs](/docs/it/network-config#streaming-idle-watchdogs), quindi le variabili che cambiano quel timeout cambiano anche questa attesa. In entrambi i casi, Claude Code aggiunge un secondo per ogni 32KB del corpo della richiesta.
* **Retry**: un secondo in meno di `API_TIMEOUT_MS`, poco meno di 10 minuti per impostazione predefinita, in modo che il nuovo tentativo possa durare più a lungo di un proxy o gateway che tiene la risposta fino al completamento della generazione. Su Amazon Bedrock, il nuovo tentativo utilizza la stessa scadenza del primo tentativo e il messaggio mostra una durata invece di due.

Nessuna attesa supera un secondo in meno di un `API_TIMEOUT_MS` positivo, e un `API_TIMEOUT_MS` positivo inferiore a 11 secondi disattiva la scadenza. Il watchdog a livello di byte inizia solo una volta che le intestazioni di risposta arrivano, quindi una risposta che smette di inviare byte dopo quello segue le [stalled-stream rules](#automatic-retries) invece di questa scadenza.

**Cosa fare:**

* Inviate di nuovo il vostro messaggio. Il vostro messaggio originale è ancora nella conversazione, quindi per un prompt lungo potete digitare `try again` invece di incollare l'intera cosa.
* Se si ripete, trattarlo come un [network or proxy problem](#unable-to-connect-to-api). Un proxy che accetta la connessione e non invia mai la richiesta produce questo errore ad ogni tentativo.
* Se un proxy o gateway sulla vostra rete tiene le risposte fino al completamento, aumentate `API_TIMEOUT_MS` in modo che il nuovo tentativo aspetti più a lungo. Su Amazon Bedrock, aumentate anche `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`.
* Se il primo tentativo continua a scadere e il nuovo tentativo ha successo, aumentate `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` in modo che il primo tentativo aspetti abbastanza a lungo.

Prima della v2.1.242, Claude Code aspettava il timeout di richiesta completo `API_TIMEOUT_MS`, 10 minuti per impostazione predefinita, prima di fallire una richiesta di streaming senza risposta. Prima della v2.1.261, il nuovo tentativo aspettava la stessa scadenza del primo tentativo e il messaggio non mostrava durate.

<h3 id="the-response-above-may-be-incomplete">
  The response above may be incomplete
</h3>

Una richiesta di streaming non è riuscita mentre la risposta era ancora in corso, dopo che Claude aveva completato un blocco di testo o una chiamata di strumento, o ne aveva iniziato uno dopo aver terminato il suo pensiero. L'invio di nuovo della richiesta potrebbe eseguire le stesse chiamate di strumento due volte, quindi Claude Code mantiene l'output che Claude ha completato e aggiunge questo avviso invece di scartare il turno. Quale variante vedete indica la causa:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection lost mid-response. The response above may be incomplete.
API Error: Your computer went to sleep mid-response. The response above may be incomplete.
API Error: The response stopped arriving. The response above may be incomplete.
```

* `Server error mid-response`: un errore del server di sovraccarico o 5xx a metà flusso. Questa variante richiede Claude Code v2.1.199 o successivo; prima di allora quel caso scartava l'output parziale e segnalava l'intero turno come errore.
* `Connection lost mid-response`: la connessione è stata interrotta.
* `Your computer went to sleep mid-response`: Claude Code ha rilevato che il vostro computer si è addormentato mentre la risposta era in streaming. Una volta che il vostro computer si sveglia, Claude Code tratta la connessione come interrotta e smette di leggerla.
* `The response stopped arriving`: la connessione è rimasta aperta ma ha smesso di consegnare dati, quindi il watchdog di inattività dello streaming l'ha interrotta. Prima della v2.1.222, Claude Code poteva anche segnalare questo errore su connessioni [gateway](/docs/it/gateways) raggiunte tramite `ANTHROPIC_BASE_URL` o `ANTHROPIC_AWS_BASE_URL` mentre i ping keep-alive del server stavano ancora arrivando, perché contava solo gli eventi di risposta analizzati lì; l'aggiornamento interrompe quei timeout spuri su quelle rotte. I gateway raggiunti tramite un URL di base del provider come `ANTHROPIC_BEDROCK_BASE_URL` non sono avvolti dal watchdog di byte; vedete [Streaming idle watchdogs](/docs/it/network-config#streaming-idle-watchdogs).

Prima della v2.1.227, `Connection lost mid-response` leggeva `Connection closed mid-response` e `The response stopped arriving` leggeva `Response stalled mid-stream`.

In quattro casi, Claude Code gestisce l'errore senza mostrare questo avviso subito:

* Più in alto nella risposta, Claude Code riprova l'errore o termina il turno con un errore diverso. Vedete [Automatic retries](#automatic-retries).
* Quando uno di questi errori arriva dopo che Claude ha terminato la risposta, Claude Code mantiene la risposta completa e termina il turno normalmente, senza questo avviso. Prima della v2.1.222, Claude Code mostrava questo avviso quando la connessione veniva interrotta o si bloccava dopo il completamento della risposta e segnalava il turno come errore anche se la risposta era completa.
* In una [non-interactive session](/docs/it/headless), come una esecuzione `-p`, un'esecuzione [Agent SDK](/docs/it/agent-sdk/overview) o una [cloud session](/docs/it/claude-code-on-the-web), non dovete inviare `continue` voi stessi quando la risposta tagliata è nella conversazione principale e contiene testo ma nessuna chiamata di strumento: Claude Code mantiene l'output parziale e chiede a Claude di continuare da dove si è fermato, fino a tre volte di seguito. Vedete questo avviso per tale risposta solo una volta che Claude Code ha esaurito quelle continuazioni. Prima della v2.1.246, Claude Code terminava un turno non interattivo con questo avviso al primo taglio.
* In un [subagent](/docs/it/sub-agents#api-errors-in-subagents), indipendentemente dal fatto che la sessione sia interattiva o meno: quando la sua risposta tagliata contiene testo ma nessuna chiamata di strumento, Claude Code chiede al subagent di continuare. L'avviso diventa l'ultimo messaggio del subagent solo una volta che quelle continuazioni sono esaurite. Prima della v2.1.257, un subagent mostrava questo avviso al primo taglio.

**Cosa fare:**

* In una sessione interattiva, leggete la risposta che rimane sullo schermo: Claude Code mantiene ogni blocco che Claude ha completato prima dell'errore, ma scarta un blocco finale interrotto quando il turno termina, quindi le frasi o le chiamate di strumento finali potrebbero mancare. Rispondete con `continue` per far riprendere a Claude dal suo ultimo blocco completato.
* In [non-interactive mode](/docs/it/headless) (`-p`):
  * Con l'output di testo predefinito, Claude Code stampa l'ultimo blocco di testo completato che ancora mantiene da prima nel turno, seguito da questo messaggio. Quando non ne mantiene nessuno, Claude Code stampa solo questo messaggio, ad esempio perché Claude Code ha compattato la conversazione a metà turno e ha cancellato quel testo. Prima della v2.1.219, Claude Code stampava solo questo messaggio nell'output di testo `-p` e scartava la risposta che aveva già prodotto.
  * Con `--output-format json` o `stream-json`, Claude Code segnala questo messaggio nel campo `result`.
  * Per continuare il turno una volta che la connessione è stabile, riprendete la sessione e inviate `continue` come descritto in [Continue conversations](/docs/it/headless#continue-conversations).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto mode cannot determine the safety of an action
</h3>

Il modello che [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) utilizza per classificare le azioni non poteva produrre una decisione, quindi auto mode non ha approvato l'azione automaticamente. Il messaggio che vedete dipende da come il classificatore non è riuscito.

Le letture, le ricerche e le modifiche all'interno della vostra directory di lavoro saltano il classificatore, quindi continuano a funzionare in tutti questi casi.

Quando il modello di classificazione non è disponibile:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait a moment and then try this action again.
```

Quando Claude Code può determinare la categoria di errore, la nomina tra parentesi dopo `temporarily unavailable`, ad esempio `<model> is temporarily unavailable (rate-limited), so auto mode cannot determine the safety of <tool> right now`. Le categorie sono `(rate-limited)`, `(overloaded)`, `(server error)`, `(timed out)` e `(connection failed)`. I limiti di velocità, il sovraccarico e gli errori del server sono transitori e il nuovo tentativo funziona. Se `(timed out)` o `(connection failed)` si ripete, controllate la vostra connessione; vedete [Unable to connect to API](#unable-to-connect-to-api). Prima della v2.1.229, il messaggio non nominava mai una categoria e leggeva `Wait briefly and then try this action again`.

Quando nessuna categoria si adatta, il messaggio appare senza categoria tra parentesi; più di un errore produce quella forma. Su [Amazon Bedrock](/docs/it/amazon-bedrock), incluso l'[Mantle endpoint](/docs/it/amazon-bedrock#use-the-mantle-endpoint), appare anche quando il vostro account AWS non può invocare il modello indicato nel messaggio e quel fallimento si ripete ad ogni nuovo tentativo fino a quando al vostro account non viene concesso l'accesso al modello.

**Cosa fare:**

* Riprovate dopo pochi secondi; Claude vede lo stesso messaggio e di solito riprova da solo. Un errore transitorio non è correlato all'[auto mode eligibility](/docs/it/permission-modes#eliminate-prompts-with-auto-mode); non dovete cambiare le impostazioni
* Se i nuovi tentativi continuano a fallire, continuate con attività di sola lettura e tornate all'azione bloccata in seguito
* Su Amazon Bedrock, se il messaggio ritorna ad ogni nuovo tentativo, controllate che il vostro account possa invocare il modello che nomina: per i modelli Amazon Bedrock standard, confermate che la vostra [IAM policy](/docs/it/amazon-bedrock#iam-configuration) consente di invocarlo; per gli ID modello Mantle, [contattate il vostro team di account AWS](/docs/it/amazon-bedrock#mantle-endpoint-errors)

Quando una richiesta di classificazione non riesce perché il vostro token OAuth è scaduto o è stato ruotato da un'altra sessione, Claude Code aggiorna il token e riprova la richiesta una volta, quindi una scadenza di token di routine non emerge come questo messaggio. Prima della v2.1.216, un token scaduto o ruotato non riusciva ad ogni richiesta di classificazione e auto mode negava ogni azione controllata con questo messaggio fino a quando il token non veniva aggiornato.

Quando il classificatore ha restituito una risposta non analizzabile:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Cosa fare:**

* Riprovate l'azione; questo di solito ha successo al tentativo successivo
* Eseguite `claude --debug` e ripetete l'azione per vedere la risposta del classificatore sottostante nel registro di debug

Quando un controllo di sicurezza API separato ha bloccato la richiesta del classificatore a causa del contenuto della conversazione precedente:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

Claude Code nega l'azione ma dice a Claude che questo non è un giudizio che l'azione sia pericolosa e di continuare con altri compiti piuttosto che riprovare. Questi rifiuti non contano verso le [auto mode's pause thresholds](/docs/it/permission-modes#when-auto-mode-falls-back). In un'esecuzione `-p` [non-interactive](/docs/it/headless), Claude Code non interrompe l'esecuzione. Quello che Claude riceve dipende da dove ha richiesto l'azione:

* A un [background subagent](/docs/it/sub-agents#run-subagents-in-foreground-or-background) in un'esecuzione `-p` senza `--input-format stream-json`, Claude Code restituisce un risultato di errore contenente `Agent aborted: auto mode classifier request refused by the safety safeguard in headless mode`
* Ovunque, incluse le sessioni interattive e la conversazione principale di un'esecuzione `-p`, Claude Code restituisce quel rifiuto a Claude

Prima della v2.1.225, Claude Code contava questi rifiuti verso le soglie di pausa e restituiva lo stesso messaggio di rifiuto di un blocco di classificatore genuino.

**Cosa fare:**

* Questo non è un giudizio sulla vostra azione. Il contenuto già nella vostra conversazione ha attivato un filtro di sicurezza sull'API quando auto mode ha inviato la conversazione al classificatore
* Riprovare non aiuterà; lo stesso contenuto della conversazione attiverà di nuovo il filtro
* In una sessione interattiva, passate a una [permission mode](/docs/it/permission-modes) diversa in modo da poter approvare l'azione quando richiesto
* Iniziate una conversazione nuova senza il contenuto che attiva

Quando la conversazione è cresciuta più grande della finestra di contesto del classificatore:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

Quello che accade all'azione dipende da dove Claude l'ha richiesta:

* In una sessione interattiva, auto mode torna a un normale prompt di autorizzazione per quell'azione in modo da poter approvarla o negarla manualmente
* A un [background subagent](/docs/it/sub-agents#run-subagents-in-foreground-or-background) in un'esecuzione `-p` [non-interactive](/docs/it/headless) senza `--input-format stream-json`, Claude Code restituisce un risultato di errore contenente `Agent aborted: auto mode classifier transcript exceeded context window in headless mode` e l'esecuzione continua
* Altrove in un'esecuzione `-p` senza un [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags), non c'è alcun prompt a cui tornare, quindi l'azione non viene eseguita e l'esecuzione continua

**Cosa fare:**

* In una sessione interattiva, approvate o negate l'azione nel prompt che appare
* In una sessione interattiva, eseguite `/compact` per ridurre la dimensione della conversazione in modo che le azioni successive si adattino di nuovo alla finestra del classificatore

<h3 id="the-server-returned-no-safety-verdict">
  The server returned no safety verdict
</h3>

Sotto [server-side classifier review](/docs/it/permission-modes#server-side-classifier-review), auto mode nega un'azione quando il server non dà un verdetto per essa. Il rifiuto nomina una categoria tra parentesi quando Claude Code può determinarne una, come `(timed out)`:

```text theme={null}
The server-side auto mode classifier gave no verdict (timed out), so auto mode cannot determine the safety of <tool>.
```

Il resto del messaggio dice a Claude se un nuovo tentativo può aiutare. Prima di alcuni di questi rifiuti, Claude Code aspetta in modo che il prossimo tentativo di Claude non segua subito. Durante l'attesa in una sessione interattiva, lo spinner mostra `Auto mode check unavailable` con un conto alla rovescia, e premere `Esc` interrompe il turno.

Dopo dieci risposte di fila senza verdetto, auto mode interrompe il turno:

```text theme={null}
Auto mode is unavailable — the server returned no safety verdict for the last 10 responses, so Claude stopped. Send a message to try again, or switch out of auto mode.
```

Il messaggio di arresto appare in un posto diverso in ogni tipo di sessione:

* In una sessione interattiva, il messaggio appare come un avviso nella trascrizione e il turno termina
* In un'esecuzione `-p` [non-interactive](/docs/it/headless), l'esecuzione termina e segnala un errore di esecuzione. Con l'output di testo predefinito, il messaggio stampa su stderr.
* Quando un [subagent](/docs/it/sub-agents) ha raggiunto il limite, il subagent si ferma prima di finire, e Claude riceve quello che ha prodotto con una nota che auto mode l'ha fermato

**Cosa fare:**

* Inviate un altro messaggio per far riprovare a Claude. Il conteggio delle risposte ricomincia da capo.
* Se l'arresto si ripete e le vostre richieste passano attraverso un [LLM gateway or proxy](/docs/it/llm-gateway), controllate se taglia le risposte di streaming corte o le riscrive. [Server-side classifier review](/docs/it/permission-modes#server-side-classifier-review) dice quale comportamento del gateway causa rifiuti, e la [gateway compatibility guide](/docs/it/llm-gateway-protocol#feature-pass-through) elenca cosa passare attraverso invariato.
* Impostate `CLAUDE_CODE_AUTO_MODE_SERVER=0` prima di avviare Claude Code per utilizzare le sue richieste di classificatore. Prima della v2.1.281, Claude Code non leggeva la variabile su una connessione diretta a Anthropic API.
* Per approvare le azioni voi stessi, [switch out of auto mode](/docs/it/permission-modes#switch-permission-modes)

Prima della v2.1.280, Claude Code negava ogni azione da una risposta senza verdetto immediatamente e non interrompeva mai il turno.

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent terminated early due to an API error
</h3>

Una richiesta API di un [subagent](/docs/it/sub-agents) non è riuscita in modo terminale, ad esempio perché è stato raggiunto un limite di utilizzo o i nuovi tentativi per un errore del server sono esauriti, quindi il subagent si è fermato prima di completare il suo compito. Questo messaggio richiede Claude Code v2.1.199 o successivo; prima di allora il testo di errore dell'API veniva restituito a Claude come se fosse il risultato del subagent.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Cosa fare:**

* Abbinate il dettaglio dell'errore dopo i due punti alla sua sezione su questa pagina, come [Usage limits](#usage-limits) o [Server errors](#server-errors), e seguite i passaggi di quella sezione
* Una volta che l'errore sottostante si risolve, chiedete a Claude di riprovare il compito o di [resume the subagent](/docs/it/sub-agents#resume-subagents)

Quando un limite di velocità, sovraccarico o errore del server interrompe un subagent in primo piano che ha già prodotto output di testo, Claude riceve quell'output parziale contrassegnato come incompleto invece di questo errore. Un subagent il cui unico output era chiamate di strumento riceve anche questo errore; nella v2.1.199 quella forma restituiva un risultato parziale vuoto. Vedete [API errors in subagents](/docs/it/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Limiti di utilizzo
</h2>

La maggior parte degli errori in questa sezione significa che è stata raggiunta una quota associata al tuo account o al tuo piano. Tre funzionano diversamente: [`Server is temporarily limiting requests`](#server-is-temporarily-limiting-requests) è una limitazione lato server non correlata alla quota del tuo piano, [`Usage credits required for 1M context`](#usage-credits-required-for-1m-context) è un controllo di diritto piuttosto che una quota esaurita, e [`The prompt to confirm went unanswered`](#the-prompt-to-confirm-went-unanswered) significa che un prompt di consenso per i crediti di utilizzo è stato chiuso senza risposta, indipendentemente dal fatto che sia stata raggiunta una quota.

<h3 id="youve-hit-your-session-limit">
  You've hit your session limit
</h3>

I piani di abbonamento includono un'indennità di utilizzo mobile. Quando si esaurisce, vedrai uno di questi messaggi:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
You've hit your Sonnet limit · resets 3:45pm
```

Claude Code blocca ulteriori richieste fino all'ora di ripristino mostrata nel messaggio. I limiti di sessione e settimanali sono condivisi tra tutti i modelli, quindi il cambio di modelli non ripristina l'accesso. I limiti Opus e Sonnet si applicano ciascuno solo alle richieste a quella famiglia di modelli, quindi il passaggio a un modello al di fuori della famiglia con `/model` ti mantiene al lavoro.

In una sessione interattiva con accesso tramite abbonamento claude.ai, Claude Code può anche attendere nella sessione aperta e continuare l'attività interrotta poco dopo il ripristino. Mentre attende, una riga in fondo alla sessione legge `Usage limit reached · continuing automatically at 3:45pm · esc to cancel`. Premi `Esc` a un prompt vuoto per annullare l'attesa. Vedi [Wait for a usage limit to reset](/docs/it/interactive-mode#wait-for-a-usage-limit-to-reset) per quello che vedi, come avviare o annullare un'attesa e come disattivare la continuazione automatica. Prima della v2.1.234, Claude Code non offriva questa attesa.

L'utilizzo conta sia per le indennità di sessione che settimanali contemporaneamente. Un singolo picco di attività intensa, come un grande fanout di flusso di lavoro, può esaurire l'indennità settimanale prima che la finestra di sessione si ripristini.

**Cosa fare:**

* Attendi l'ora di ripristino mostrata nell'errore
* Nella scheda Code dell'[app Desktop](/docs/it/desktop), la scheda session-limit offre una casella di controllo **Auto-continue when limits reset**. La scheda weekly-limit non lo fa. Quando è selezionata, l'app Desktop ritenta il turno interrotto dopo il ripristino e mostra l'ora del nuovo tentativo sulla scheda. La casella di controllo Desktop e l'impostazione **Continue automatically at usage limit** della CLI in `/config` sono separate, quindi disattiva ciascuna per conto proprio.
* Per il limite Opus o Sonnet, esegui `/model` e passa a un modello al di fuori di quella famiglia per continuare a lavorare. Ogni modello ha la propria cache di prompt, quindi la richiesta successiva rilegge l'intera conversazione senza hit della cache; vedi [Switching models](/docs/it/prompt-caching#switching-models)
* Esegui `/usage` per vedere i limiti del tuo piano e quando si ripristinano
* Esegui `/usage-credits` per acquistare utilizzo aggiuntivo su Pro e Max, o per richiederlo al tuo amministratore su Team ed Enterprise. Vedi [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) per come viene fatturato.
* Per aggiornare il tuo piano per limiti di base più elevati, vedi [claude.com/pricing](https://claude.com/pricing)

Prima che una finestra si esaurisca, Claude Code può avvertirti che hai utilizzato la maggior parte di essa, con un messaggio come `You've used 85% of your session limit · resets 3:45pm`. Per monitorare l'indennità rimanente continuamente, aggiungi i campi `rate_limits` a una [riga di stato personalizzata](/docs/it/statusline#rate-limit-usage), oppure nell'app Desktop fai clic sull'[anello di utilizzo](/docs/it/desktop#check-usage) accanto al selettore di modelli.

<h3 id="usage-credits-required-for-1m-context">
  Usage credits required for 1M context
</h3>

Il modello selezionato utilizza la finestra di contesto estesa da 1M token e il tuo piano lo include solo tramite crediti di utilizzo.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on (they take effect after you restart Claude Code), or /model to switch to standard context
```

Questo è un controllo di diritto, non un esaurimento della quota. Si attiva anche quando le tue indennità di sessione e settimanali hanno capacità rimanente. Vedi [Extended context](/docs/it/model-config#extended-context) per quali piani includono il contesto 1M direttamente e quali richiedono crediti di utilizzo. Claude Code esegue questo controllo quando scegli il modello con `/model`, e solo su una connessione diretta all'API Anthropic; se punti `ANTHROPIC_BASE_URL` a un [gateway LLM](/docs/it/llm-gateway), `/model` consente la selezione `[1m]` e il gateway decide se la richiesta ha successo.

Quando questo errore appare a metà conversazione perché il contesto è cresciuto oltre 200K token, Claude Code compatta automaticamente la conversazione al di sotto del limite di contesto standard e mantiene la sessione a quel limite in seguito, quindi non è necessaria alcuna azione. Nelle versioni precedenti alla v2.1.172, l'errore si ripeteva su ogni richiesta successiva incluso `/compact`; esegui `/clear` su quelle versioni per recuperare. I passaggi seguenti si applicano quando hai esplicitamente selezionato un modello `[1m]`.

**Cosa fare:**

* Esegui `/model` e seleziona la variante senza il suffisso `[1m]` per tornare alla finestra di contesto standard
* Dove il messaggio nomina `/usage-credits`, eseguilo per attivare la fatturazione a consumo per la variante 1M su Pro e Max, o per richiedere crediti di utilizzo al tuo amministratore su Team ed Enterprise. Una volta che i crediti di utilizzo sono attivati, riavvia Claude Code o avvia una nuova sessione, a seconda di quello che dice il messaggio. Fino al riavvio, la sessione rimane al limite di contesto standard.
* Se l'errore persiste dopo `/model`, un ID modello 1M potrebbe essere impostato altrove. Vedi [Setting your model](/docs/it/model-config#setting-your-model) per i percorsi di configurazione da controllare in ordine di priorità.
* Per rimuovere completamente le varianti 1M dal selettore di modelli, imposta [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/it/env-vars)

Prima della v2.1.268, il messaggio terminava con `run /usage-credits to turn them on, or /model to switch to standard context` e non menzionava il riavvio.

<h3 id="the-prompt-to-confirm-went-unanswered">
  The prompt to confirm went unanswered
</h3>

Se il tuo account richiede il [consenso per i crediti di utilizzo Fable](/docs/it/model-config#fable-and-usage-credits), Claude Code ti chiede di confermare prima che una richiesta Fable fatturi i crediti di utilizzo. Quando nessuno risponde a quel prompt di consenso in una sessione che potrebbe non avere nessuno al suo terminale, Claude Code chiude il prompt e termina il turno con uno di questi messaggi:

```text theme={null}
Fable limit reached · continuing on Fable 5.1 uses usage credits, and the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
Fable 5.1 now uses usage credits · the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
```

I messaggi nominano il modello Fable della sessione, quindi su Fable 5 leggono `continuing on Fable 5` e `Fable 5 now uses usage credits`. Prima della v2.1.257, il primo messaggio iniziava `Fable 5 limit reached`.

Questo accade nelle sessioni [Remote Control](/docs/it/remote-control), [sessioni in background](/docs/it/agent-view) e sessioni compagni [team agente](/docs/it/agent-teams). Claude Code mostra il prompt di consenso solo nella vista interattiva della sessione: il terminale dove viene eseguito, o, per una sessione in background, la [vista agenti](/docs/it/agent-view) una volta che ti colleghi. Un client Remote Control non può visualizzarlo. Claude Code chiude il prompt alla scadenza [`dialogExpiry`](/docs/it/settings-reference#dialogexpiry), cinque minuti per impostazione predefinita, o non appena arriva un nuovo prompt mentre nessuno ha digitato a quel terminale, come un prompt inviato da un client Remote Control. Digitare al terminale dove viene eseguita la sessione annulla la scadenza, e Claude Code attende la tua risposta. Nella vista allegata di una sessione in background, digitare non annulla la scadenza, e un nuovo prompt chiude comunque il prompt di consenso, quindi rispondi prima che accada uno dei due. Claude Code non invia nulla e mantiene il tuo modello, quindi quando invii il tuo prossimo prompt, Claude Code mostra di nuovo il prompt di consenso.

**Cosa fare:**

* Al terminale dove viene eseguita la sessione, invia un altro prompt e rispondi al prompt di consenso quando riappare. Per una sessione in background, collegati prima dalla [vista agenti](/docs/it/agent-view). Reinviare da un client Remote Control mostra di nuovo questo messaggio, perché il client non può visualizzare il prompt.
* Esegui `/model` per passare a un modello che non fattura i crediti di utilizzo
* Per darti più tempo per raggiungere quel terminale, imposta [`dialogExpiry`](/docs/it/settings-reference#dialogexpiry) su un valore più lungo o `"never"`

Prima della v2.1.236, questo messaggio non appariva: mentre un client Remote Control era connesso, Claude Code attendeva 60 secondi per una risposta e poi continuava il turno sul tuo modello predefinito.

<h3 id="server-is-temporarily-limiting-requests">
  Server is temporarily limiting requests
</h3>

L'API ha applicato una limitazione di breve durata non correlata alla quota del tuo piano.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code distingue questi dalla tua quota di piano per l'assenza delle intestazioni di quota unificata che una vera risposta di limite porta. A partire dalla v2.1.199 questo viene [ritentato automaticamente](#automatic-retries) con backoff prima di essere mostrato, indipendentemente da come ti autentichi. Nelle versioni precedenti, una sessione con accesso tramite abbonamento claude.ai ha fallito il turno alla prima occorrenza; solo le chiavi API e gli accessi Enterprise lo hanno ritentato.

**Cosa fare:**

* Attendi brevemente e riprova
* Controlla [status.claude.com](https://status.claude.com) se persiste

<h3 id="request-rejected-429">
  Request rejected (429)
</h3>

Hai raggiunto il limite di velocità configurato per la tua chiave API, il progetto Amazon Bedrock o il progetto Google Cloud.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

La frase finale nomina dove controllare l'integrità del servizio e varia in base al provider. Le configurazioni Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry nominano lo stato del servizio di quel provider invece della pagina di stato Anthropic. Un `ANTHROPIC_BASE_URL` personalizzato nomina l'host del gateway.

**Cosa fare:**

* Esegui `/status` e conferma che la credenziale attiva è quella che ti aspetti. Un `ANTHROPIC_API_KEY` casuale nel tuo ambiente può instradare le richieste attraverso una chiave di livello inferiore invece del tuo abbonamento.
* Controlla la console del tuo provider per i limiti attivi e richiedi un livello più elevato se necessario
* Per le chiavi API Anthropic, vedi il [riferimento ai limiti di velocità](https://platform.claude.com/docs/en/api/rate-limits) per come funzionano i livelli e come impostare i limiti di spesa per workspace
* Riduci la concorrenza: abbassa [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/it/env-vars), evita di eseguire molti subagenzi paralleli, o passa a un modello più piccolo con `/model` per esecuzioni script ad alto volume

<h3 id="youve-hit-your-monthly-spend-limit">
  You've hit your monthly spend limit
</h3>

L'utilizzo incluso nel tuo piano non può coprire questa richiesta, e i [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) che altrimenti la pagherebbero hanno raggiunto un limite di spesa. Ciò accade quando una delle finestre di utilizzo del tuo piano si è esaurita, o quando la richiesta è una che solo i crediti di utilizzo pagano, come una richiesta a un modello che [fattura ai crediti di utilizzo](/docs/it/model-config#fable-and-usage-credits). Il messaggio nomina il limite che ti ha bloccato. Il testo dopo il `·` dice come aumentare quel limite, e varia in base al tuo piano e se gestisci la fatturazione:

```text theme={null}
You've hit your monthly spend limit · raise it at claude.ai/settings/usage
You've hit your individual spend limit · ask your admin for a higher limit
You've hit your org's monthly spend limit · visit claude.ai/admin-settings/usage to raise it
You've hit your team's shared budget · ask your admin to raise it at claude.ai/admin-settings/usage
You've hit your channel's monthly spend limit · an org owner or channel manager can raise it in the channel's Claude settings
```

`team's shared budget` è un budget in pool che un amministratore ha assegnato a un gruppo a cui appartieni; il messaggio non nomina il gruppo. `channel's monthly spend limit` è il budget del canale Slack in cui viene eseguita la sessione, quindi la tua organizzazione potrebbe ancora avere budget al di fuori di esso.

Quando una delle finestre del tuo piano è quella che si è esaurita, il messaggio dice anche quando quella finestra si ripristina, ad esempio `· your session limit resets 3:45pm`, e l'accesso ritorna allora senza che nessuno aumenti il limite. Nelle organizzazioni con fatturazione basata sull'utilizzo, il messaggio dice `usage limit` al posto di `spend limit`, come in `You've hit your individual usage limit`.

Prima della v2.1.239, il messaggio non nominava l'ora di ripristino della finestra del piano. Prima della v2.1.268, il budget in pool di un gruppo produceva il messaggio `individual spend limit` invece di `team's shared budget`.

Se ti connetti tramite un gateway di app Claude e vedi `spend limit reached` minuscolo, quello è il limite dell'operatore del gateway invece; vedi [Spend limit reached](#spend-limit-reached).

**Cosa fare:**

* Su Pro e Max, aumenta il tuo limite di spesa mensile in [**Settings > Usage**](https://claude.ai/settings/usage) su claude.ai, o esegui `/usage-credits`
* Su Team ed Enterprise, aumenta il limite in [**Admin settings > Usage**](https://claude.ai/admin-settings/usage) se gestisci la fatturazione, o chiedi a un amministratore di farlo. `/usage-credits` invia quella richiesta al tuo amministratore per te
* Per il limite di un canale, chiedi a un proprietario dell'organizzazione o al manager del canale di aumentarlo su claude.ai. Vedi [Per-channel limits](https://claude.com/docs/claude-tag/admins/set-spend-limit#per-channel-limits) nella documentazione Claude Tag
* Se il messaggio nomina un'ora di ripristino per la finestra del tuo piano, puoi aspettare invece
* Esegui `/usage` per vedere le finestre del tuo piano e quando ciascuna si ripristina

<h3 id="spend-limit-reached">
  Spend limit reached
</h3>

Ti connetti tramite un [gateway di app Claude](/docs/it/claude-apps-gateway) e hai superato un [limite di spesa](/docs/it/claude-apps-gateway-spend-limits) impostato dall'operatore del gateway. Il gateway blocca le tue richieste fino a quando il periodo denominato non si ripristina o l'operatore non aumenta il limite. Contrassegna ogni risposta `429` bloccata con `x-should-retry: false`, quindi Claude Code mostra questo messaggio senza ritentare.

```text theme={null}
spend limit reached (daily; resets 2026-08-09 00:00 UTC)
```

Il messaggio nomina il periodo del limite e l'ora di ripristino, e quando l'operatore ha configurato un `blocked_message`, le sue istruzioni lo seguono. Prima della v2.1.225, il messaggio leggeva solo `spend limit reached`; un gateway su una versione precedente invia ancora quella forma più breve.

**Cosa fare:**

* Attendi l'ora di ripristino che il messaggio nomina, o segui le istruzioni dell'operatore se il messaggio le contiene
* Chiedi al tuo operatore del gateway di aumentare il limite se lo raggiungi regolarmente

Un messaggio correlato, `spend limit unavailable`, significa che il gateway non poteva leggere i suoi record di spesa e ha bloccato la richiesta come precauzione piuttosto che per il tuo limite. Di solito si risolve da solo; se persiste, comunica al tuo operatore del gateway.

<h3 id="credit-balance-is-too-low">
  Credit balance is too low
</h3>

L'organizzazione della tua Console ha esaurito i crediti prepagati, o Claude Code sta inviando le tue richieste con una chiave API Console quando intendevi usare il tuo abbonamento.

```text theme={null}
Credit balance is too low
```

**Cosa fare:**

* Se hai un piano Pro, Max, Team o Enterprise e vedi questo, esegui `/status` e controlla la riga `API key`. Un `ANTHROPIC_API_KEY` approvato nel tuo ambiente instrada le richieste attraverso quella chiave invece del tuo abbonamento. Annullalo nella shell corrente e rimuovilo dal tuo profilo shell, quindi riavvia `claude`. Esegui `/login` se non hai ancora effettuato l'accesso con il tuo abbonamento.
* Aggiungi crediti su [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), e considera di abilitare il ricaricamento automatico lì in modo che il saldo si riempia prima di raggiungere lo zero
* Imposta i limiti di spesa per workspace nella Console per evitare che un singolo progetto dreni il saldo dell'organizzazione. Vedi [Manage costs effectively](/docs/it/costs).

<h3 id="could-not-update-your-spend-limit">
  Could not update your spend limit
</h3>

Il server ha rifiutato una modifica del limite di spesa che hai effettuato dal prompt che appare quando raggiungi il tuo limite di spesa.

```text theme={null}
Could not update your spend limit: <reason from the server>
```

Quando il server spiega il rifiuto, il messaggio termina con quel motivo, e ritentare lo stesso valore fallisce di nuovo. Quando il fallimento non ha un motivo fornito dal server, come una connessione interrotta, il messaggio legge `Could not update your spend limit. Press Enter to retry.` e ritentare può avere successo. Prima della v2.1.216, Claude Code mostrava la forma generica per ogni fallimento.

**Cosa fare:**

* Se il messaggio include un motivo, scegli un limite che lo soddisfi, come un importo inferiore
* Se il messaggio mostra solo la forma generica, ritenta; il fallimento potrebbe essere transitorio
* Se la modifica continua a fallire, effettuala dalle tue [impostazioni di fatturazione claude.ai](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) nel browser invece

<h2 id="authentication-errors">
  Errori di autenticazione
</h2>

Questi errori significano che Claude Code non può provare la Vostra identità all'API. Eseguite `/status` in qualsiasi momento per vedere quale credenziale è attualmente attiva.

<h3 id="not-logged-in">
  Non connesso
</h3>

Nessuna credenziale valida è disponibile per questa sessione.

```text theme={null}
Not logged in · Please run /login
```

**Cosa fare:**

* Eseguite `/login` per autenticarvi con il Vostro abbonamento Claude o l'account Console
* Se vi aspettavate che una variabile d'ambiente vi autenticasse, confermate che `ANTHROPIC_API_KEY` sia impostata ed esportata nella shell dove avete lanciato `claude`
* Per CI o automazione dove il login interattivo non è possibile, configurate uno script [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) che recuperi una chiave all'avvio
* Vedete [Precedenza dell'autenticazione](/docs/it/authentication#authentication-precedence) per capire quale credenziale Claude Code utilizza quando sono presenti più credenziali

Se vi viene chiesto di accedere ripetutamente, vedete [Non connesso o token scaduto](/docs/it/troubleshoot-install#not-logged-in-or-token-expired) per i controlli dell'orologio di sistema e i passaggi di recupero dell'archiviazione delle credenziali di macOS.

<h3 id="could-not-resolve-authentication-method">
  Impossibile risolvere il metodo di autenticazione
</h3>

La sessione ha raggiunto il client API senza alcuna credenziale. Le [sessioni in background](/docs/it/agent-view) e le sessioni cloud mostrano questo messaggio quando il worker si avvia senza una credenziale. Le esecuzioni interattive, `-p` e Agent SDK segnalano la stessa condizione di [Non connesso](#not-logged-in) e scrivono questa stringa solo nel loro log di debug, quindi se l'avete trovata lì, seguite quella voce invece.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

Sulle versioni attuali l'errore significa che nessuna credenziale era disponibile al processo worker. Prima della v2.1.174, una sessione in background assegnata a un worker pre-inizializzato inattivo poteva fallire in questo modo anche quando le credenziali valide erano configurate. Prima della v2.1.176, anche una sessione cloud che era rimasta inattiva prima di essere rivendicata poteva farlo. Aggiornate per recuperare.

**Cosa fare:**

* Aggiornate alla v2.1.176 o successiva se questo appare in una sessione in background o cloud e le Vostre credenziali sono già configurate
* Confermate che `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` o le Vostre credenziali del provider cloud siano impostati nell'ambiente che avvia il worker, non solo nella Vostra shell interattiva
* Per Agent SDK, vedete [configurazione dell'autenticazione nella guida rapida](/docs/it/agent-sdk/quickstart#setup)
* Eseguite `/status` in una sessione interattiva nello stesso ambiente per confermare quale fonte di credenziale si risolve

<h3 id="invalid-api-key">
  Chiave API non valida
</h3>

La variabile d'ambiente `ANTHROPIC_API_KEY` o lo script `apiKeyHelper` ha restituito una chiave che l'API ha rifiutato, oppure Claude Code ha bloccato una chiave da `ANTHROPIC_API_KEY` prima di inviarla.

```text theme={null}
Invalid API key · Fix external API key
```

Quando il messaggio continua oltre `Fix external API key` con una descrizione come `Invalid X-Api-Key header value from ANTHROPIC_API_KEY: it contains a line break at character 41 (120 characters on 2 lines).`, l'API non ha mai visto la chiave. Claude Code ha trovato un carattere che le intestazioni HTTP non possono trasportare e ha fermato la richiesta prima di inviarla. Vedete [Valore di intestazione di richiesta non valido](#invalid-request-header-value) per come leggere la descrizione e correggere il valore.

**Cosa fare:**

* Controllate gli errori di battitura e confermate che la chiave non sia stata revocata nella [Console](https://platform.claude.com/settings/keys)
* Nella stessa shell, eseguite `env | grep ANTHROPIC`, oppure in PowerShell `Get-ChildItem Env:ANTHROPIC*`. Strumenti come direnv, plugin shell dotenv e terminali IDE possono caricare una chiave obsoleta da un file `.env` nel Vostro progetto senza che la impostiate esplicitamente.
* Annullate l'impostazione di `ANTHROPIC_API_KEY` ed eseguite `/login` per utilizzare invece l'autenticazione dell'abbonamento
* Se la chiave proviene da uno script [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper), eseguite lo script direttamente per confermare che stampi una chiave valida su stdout
* Eseguite `/status` per confermare quale fonte di credenziale Claude Code sta effettivamente utilizzando

<h3 id="your-apikeyhelper-script-is-failing">
  Lo script apiKeyHelper non funziona
</h3>

Claude Code ha eseguito il comando nella Vostra impostazione [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) e non ha ottenuto una chiave indietro. Senza una, la richiesta raggiunge l'API con una credenziale segnaposto e l'API la rifiuta con `401`. Il pannello `Authentication` nel terminale mostra quale di questi è accaduto:

* Il comando è uscito con un errore o è scaduto
* Il comando non ha stampato nulla su stdout
* Il comando ha stampato qualcosa di diverso dalla chiave, come un banner di login o una riga di log. Il pannello mostra `returned output that cannot be used as an API key` e dice cosa c'è di sbagliato, senza ripetere l'output. Prima della v2.1.227, Claude Code inviava tutto ciò che il comando stampava, dopo aver tagliato gli spazi bianchi circostanti.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

In [modalità non interattiva](/docs/it/headless), stderr trasporta anche il motivo specifico, con il prefisso `apiKeyHelper failed:`.

Claude Code riesegue lo script e ritenta la richiesta fino a due volte in più prima di mostrare questo messaggio, quindi l'errore emerge entro tre tentativi. Prima della v2.1.208, Claude Code spendeva l'intero [budget di ripetizione](#automatic-retries) reinviando la richiesta con la credenziale segnaposto e poi segnalava un errore di autenticazione generico `401` invece dell'errore dello script.

L'esecuzione di `/login` non aiuta qui: l'output dell'helper [ha la precedenza](/docs/it/authentication#authentication-precedence) su un login salvato finché l'impostazione è presente.

**Cosa fare:**

* Eseguite il comando configurato in `apiKeyHelper` direttamente nella Vostra shell per riprodurre l'errore
* Se il comando segnala una sessione scaduta, riauthenticate con il Vostro provider di credenziali, ad esempio accedendo di nuovo al Vostro SSO o vault di segreti
* Correggete il comando in modo che stampi solo la chiave su stdout, come un singolo token di ASCII stampabile fino a 16.384 caratteri, e uscite con codice 0. Vedete [ruotare le credenziali con apiKeyHelper](/docs/it/llm-gateway-connect#rotate-credentials-with-apikeyhelper) per una configurazione funzionante.
* Eseguite `/status` per vedere l'errore e confermare che `apiKeyHelper` sia la fonte di credenziale attiva. La riga `apiKeyHelper` mostra `Failing` con il dettaglio dell'ultimo errore, come il codice di uscita e l'output di errore del comando, e scompare dopo il prossimo esecuzione riuscita. Prima della v2.1.274, `/status` mostrava solo la fonte di credenziale, non l'errore.
* Ogni volta che il comando fallisce, il suo codice di uscita e l'output di errore appaiono anche in un pannello `Authentication` nel terminale. Prima della v2.1.212, il pannello era intitolato `Cloud authentication`.

<h3 id="invalid-request-header-value">
  Valore di intestazione di richiesta non valido
</h3>

Un valore che Claude Code stava per inviare come intestazione di richiesta contiene un carattere che le intestazioni HTTP non possono trasportare: un'interruzione di riga, un byte NUL o un carattere sopra `U+00FF`, come una virgoletta ricurva o uno spazio di larghezza zero. Claude Code ferma la richiesta prima che qualsiasi cosa sia inviata e nomina la variabile o l'impostazione da correggere. La causa usuale è una credenziale incollata da un documento o chat che trasportava un carattere invisibile o un'interruzione di riga errata.

Claude Code esegue questo controllo quando invia richieste all'API Claude direttamente o attraverso un [gateway LLM](/docs/it/llm-gateway). Su un provider cloud di terze parti come [Amazon Bedrock](/docs/it/amazon-bedrock), Claude Code non lo esegue prima di inviare.

```text theme={null}
Invalid auth token · Fix external auth token
Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable
Invalid request header from the environment · Fix the environment variable
```

La prima parte del messaggio dipende da dove proviene il valore errato:

* `Invalid auth token`: un token bearer da [`ANTHROPIC_AUTH_TOKEN`](/docs/it/env-vars) o [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/it/env-vars)
* `Invalid ANTHROPIC_CUSTOM_HEADERS`: un nome o valore di intestazione che avete impostato in [`ANTHROPIC_CUSTOM_HEADERS`](/docs/it/env-vars). La descrizione conta quale coppia `Name: Value` è in colpa, come `distinct header 2 of 3 parsed from ANTHROPIC_CUSTOM_HEADERS`, senza ripetere il nome o il valore, poiché li avete scelti entrambi.
* `Invalid request header from the environment`: un valore che Claude Code copia in un'intestazione di richiesta da un'altra variabile d'ambiente, come `CLAUDE_AGENT_SDK_CLIENT_APP`. La descrizione nomina la variabile da correggere.

Claude Code segnala un `ANTHROPIC_API_KEY` errato catturato da questo controllo come [Chiave API non valida](#invalid-api-key), con la stessa descrizione finale. Segnala una credenziale `/login` salvata errata come [Non connesso](#not-logged-in) invece; eseguite `/login` per salvarne una nuova. L'output di uno script [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) non raggiunge mai questo controllo: Claude Code lo convalida quando lo script viene eseguito e l'output che un'intestazione HTTP non può trasportare fallisce con [Lo script apiKeyHelper non funziona](#your-apikeyhelper-script-is-failing).

Dopo il secondo `·`, il messaggio descrive il problema, come in questo esempio completo:

```text theme={null}
Invalid auth token · Fix external auth token · Invalid Authorization header value from ANTHROPIC_AUTH_TOKEN: it contains a line break at character 41 (120 characters on 2 lines).
```

Le posizioni contano i caratteri a partire da uno. La descrizione è costruita da frasi fisse e conteggi di caratteri, quindi non include mai il valore stesso. Nomina il carattere offensivo solo quando è un carattere invisibile o tipografico ben noto, come un byte order mark, uno spazio di larghezza zero o una virgoletta ricurva, e segnala tutto il resto come `a non-ASCII character`.

**Cosa fare:**

* Reimpostate la variabile o l'impostazione che il messaggio nomina, riscrivendo i caratteri intorno alla posizione segnalata piuttosto che incollando dalla stessa fonte di nuovo
* Per `ANTHROPIC_CUSTOM_HEADERS`, mantenete una coppia `Name: Value` per riga e riscritte la coppia che il messaggio conta
* Eseguite `/status` per confermare quale fonte di credenziale è attiva

<h3 id="this-organization-has-been-disabled">
  Questa organizzazione è stata disabilitata
</h3>

Claude Code sta utilizzando un `ANTHROPIC_API_KEY` obsoleto da un'organizzazione Console disabilitata. Quando avete un login di abbonamento salvato, la chiave lo sostituisce.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your subscription instead
Your ANTHROPIC_API_KEY belongs to a disabled organization · Update or unset the environment variable
API Error: 400 ... This organization has been disabled.
```

Il suggerimento dopo il `·` dipende dalle Vostre credenziali salvate: la prima forma appare quando un `/login` memorizzato può subentrare dopo aver annullato l'impostazione della chiave, e la seconda quando la chiave è la Vostra unica credenziale.

Le variabili d'ambiente hanno la precedenza su `/login`, quindi una chiave esportata nel Vostro profilo shell o caricata da un file `.env` è utilizzata anche quando avete un abbonamento Pro o Max funzionante. In modalità non interattiva (`-p`), la chiave è sempre utilizzata quando presente.

**Cosa fare:**

* Annullate l'impostazione di `ANTHROPIC_API_KEY` nella shell corrente e rimuovetela dal Vostro profilo shell, quindi riavviate `claude`
* Se il messaggio dice `Update or unset`, non avete un login salvato su cui ricadere. Annullate l'impostazione della chiave ed eseguite `/login`, oppure sostituite la chiave con una da un'organizzazione Console attiva.
* Eseguite `/status` in seguito per confermare che la credenziale attiva sia il Vostro abbonamento
* Se nessuna variabile d'ambiente è impostata e l'errore persiste, l'organizzazione disabilitata è quella legata al Vostro `/login`. Contattate il supporto o accedete con un account diverso.

<h3 id="your-organization-has-disabled-api-key-authentication">
  La Vostra organizzazione ha disabilitato l'autenticazione con chiave API
</h3>

Questo messaggio richiede Claude Code v2.1.169 o successiva. L'amministratore dell'organizzazione Console ha disattivato l'autenticazione con chiave API, quindi l'API rifiuta la chiave che Claude Code sta inviando. Il suggerimento di recupero dopo il `·` varia a seconda di dove proviene la chiave:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Le variabili d'ambiente e `apiKeyHelper` hanno la precedenza su `/login`, quindi eseguire `/login` da solo non aiuta mentre uno dei due sta ancora fornendo una chiave. Vedete [Precedenza dell'autenticazione](/docs/it/authentication#authentication-precedence).

**Cosa fare:**

* Se il messaggio nomina `ANTHROPIC_API_KEY`, annullate l'impostazione nella shell corrente e rimuovetela dal Vostro profilo shell o file `.env`, quindi riavviate `claude`
* Se il messaggio nomina `apiKeyHelper`, rimuovete l'impostazione [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) dal Vostro `settings.json`
* Eseguite `/login` per accedere con il Vostro account claude.ai
* Eseguite `/status` in seguito per confermare che la credenziale attiva sia il Vostro abbonamento piuttosto che una chiave API
* Se avete bisogno dell'autenticazione con chiave API per l'automazione, chiedete all'amministratore dell'organizzazione di riattivarla nella Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  La Vostra organizzazione ha disabilitato l'accesso all'abbonamento Claude
</h3>

La Vostra organizzazione Claude non consente l'accesso a Claude Code con un login di abbonamento. L'esecuzione di `/login` di nuovo con lo stesso account restituisce lo stesso errore.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Questa è un'impostazione dell'organizzazione lato server, quindi non può essere ignorata dalle impostazioni locali, dalle variabili d'ambiente o dai flag CLI.

Agent SDK e la modalità non interattiva `-p` presentano questo come il codice di errore `oauth_org_not_allowed`.

**Cosa fare:**

* Chiedete al Vostro amministratore di abilitare l'accesso a Claude Code per la Vostra organizzazione
* Autenticate con una chiave API Console invece del Vostro abbonamento. Vedete [Autenticazione Claude Console](/docs/it/authentication#claude-console-authentication) per la configurazione.
* Se siete l'amministratore e non vedete un'opzione per abilitare l'accesso, contattate il [supporto Anthropic](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Le routine sono disabilitate dalla politica dell'organizzazione
</h3>

Un Proprietario nella Vostra organizzazione Team o Enterprise ha disattivato le routine a livello di organizzazione. L'errore appare quando tentate di creare o eseguire una routine, ad esempio dall'interfaccia utente [Routine](/docs/it/routines) su claude.ai/code. Su Claude Code v2.1.227 o successiva, la stessa impostazione [nasconde anche `/schedule`](/docs/it/routines#troubleshooting) nella CLI.

```text theme={null}
Routines are disabled by your organization's policy.
```

Questa è un'impostazione lato server, quindi non può essere ignorata dalle impostazioni locali, dalle variabili d'ambiente o dai flag CLI.

**Cosa fare:**

* Chiedete a un Proprietario nella Vostra organizzazione di abilitare l'interruttore **Routines** su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Per lavoro programmato una tantum che non richiede routine a livello di organizzazione, vedete [attività programmate](/docs/it/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control richiede l'API Anthropic
</h3>

La sessione non sta parlando direttamente all'API Anthropic, quindi non c'è un backend claude.ai per [Remote Control](/docs/it/remote-control) con cui associarsi.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com. CLAUDE_CODE_USE_BEDROCK is set, so this session is using Amazon Bedrock — unset it (or run in a shell without it) to use Remote Control.
```

Una seconda frase spiega cosa ha instradato la sessione lontano dall'API Anthropic; prima della v2.1.219, il messaggio era solo la prima frase. A seconda della causa, il messaggio nomina:

* Una variabile del provider `CLAUDE_CODE_USE_*`, come `CLAUDE_CODE_USE_BEDROCK` per [Amazon Bedrock](/docs/it/amazon-bedrock) o `CLAUDE_CODE_USE_VERTEX` per [Agent Platform di Google Cloud](/docs/it/google-vertex-ai)
* [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) che punta a un host diverso da `api.anthropic.com`, come un [gateway LLM](/docs/it/llm-gateway) o proxy, anche quando vi accedete con claude.ai; prima della v2.1.196, un URL di base personalizzato non bloccava Remote Control
* `ANTHROPIC_UNIX_SOCKET` impostato, quindi la sessione invia le sue richieste attraverso un socket locale piuttosto che a `api.anthropic.com`
* Un accesso [gateway cloud](/docs/it/claude-apps-gateway) aziendale effettuato tramite `/login`, che non supporta Remote Control e non ha alcuna variabile da annullare

**Cosa fare:**

* Annullate l'impostazione della variabile che il messaggio nomina, come `CLAUDE_CODE_USE_BEDROCK` o `ANTHROPIC_BASE_URL`, e riavviate la sessione, oppure avviate Remote Control da una sessione che parla direttamente all'API Anthropic
* Se la variabile non è impostata nella Vostra shell, controllate la chiave `env` nei Vostri [file di impostazioni](/docs/it/settings#where-settings-live), che applica le variabili d'ambiente a ogni sessione
* Per questo e gli altri messaggi di avvio di Remote Control, vedete [Risolvere i problemi di Remote Control](/docs/it/remote-control#troubleshooting)

<h3 id="remote-control-couldnt-refresh-your-login">
  Remote Control non ha potuto aggiornare il Vostro login
</h3>

Claude Code esegue una connessione [Remote Control](/docs/it/remote-control) dal vivo su credenziali di breve durata che ottiene e rinnova utilizzando il Vostro login claude.ai salvato. Quando claude.ai smette di accettare quel login, o Claude Code non ha più alcun login salvato, Claude Code ferma Remote Control e ha bisogno che vi accediate di nuovo. Uno qualsiasi dei due errori può accadere mentre Claude Code sta ancora connettendosi o più tardi, quando rinnova le credenziali.

Quando Claude Code chiede al servizio di login di aggiornare il Vostro login salvato e non riceve risposta, mantiene Remote Control in esecuzione e ritenta l'aggiornamento mentre la credenziale corrente della connessione è ancora valida. Un aggiornamento non riceve risposta quando Claude Code non può raggiungere il servizio di login, la richiesta scade o il servizio fallisce senza rifiutare il Vostro login. Se il servizio di login non sta ancora rispondendo quando quella credenziale scade, Claude Code ferma Remote Control e segnala `OAuth token refresh failed`.

Quando Claude Code ferma Remote Control, mostra il motivo in un avviso e in una riga di trascrizione che inizia con `Remote Control disconnected`. La Vostra sessione locale continua a funzionare senza Remote Control. Questa sezione copre queste righe:

```text theme={null}
Remote Control disconnected — Claude.ai login expired — run /login to restore Remote Control
Remote Control disconnected — Claude.ai login expired — run /login, then /remote-control
Remote Control disconnected — Claude.ai login was rejected — run /login, then /remote-control
Remote Control disconnected — OAuth token unavailable — run /login to restore Remote Control
Remote Control disconnected — OAuth token refresh failed — run /login to re-authenticate
Remote Control disconnected — JWT refresh failed: no OAuth token — run /login
Remote Control disconnected — Signed out of Claude — run /login, then /remote-control
```

Claude Code nomina la causa nel mezzo del messaggio:

* `Claude.ai login expired` e `Claude.ai login was rejected`: claude.ai non accetta più il Vostro token di login salvato, perché è scaduto o è stato revocato
* `OAuth token unavailable`: Claude Code non aveva alcun token di login salvato quando la credenziale della connessione è scaduta per il rinnovo
* `OAuth token refresh failed`: claude.ai ha rifiutato il Vostro token di login salvato mentre Claude Code stava riconnettendosi e l'aggiornamento del token non ha prodotto uno nuovo
* `JWT refresh failed: no OAuth token`: Claude Code non ha trovato alcun token di login salvato per rinnovare
* `Signed out of Claude`: vi siete disconnessi su questa macchina, ad esempio eseguendo `/logout` in un altro terminale, quindi Claude Code non ha alcun login salvato rimasto per rinnovare la connessione

**Cosa fare:**

* Eseguite `/login` per accedere di nuovo
* Eseguite `/remote-control` per riconnettere la sessione. I messaggi che terminano con `run /login to restore Remote Control` non hanno bisogno di questo passaggio: Claude Code si riconnette automaticamente una volta che vi siete acceduti.

Prima della v2.1.224, `OAuth token refresh failed — run /login to re-authenticate` leggeva `OAuth token refresh failed — re-authenticate, then re-enable Remote Control`, e `JWT refresh failed: no OAuth token — run /login` leggeva `no OAuth token available for recovery (code <N>)`. I messaggi `Claude.ai login expired`, `Claude.ai login was rejected` e `OAuth token unavailable` sono stati aggiunti nella v2.1.225.

Prima della v2.1.238, Claude Code segnalava i casi che ora dicono `Signed out of Claude` come `JWT refresh failed: no OAuth token — run /login`, e fermava Remote Control con `Claude.ai login expired — run /login to restore Remote Control` non appena un aggiornamento di login non riceveva risposta.

<h3 id="remote-control-stopped-because-the-signed-in-account-changed">
  Remote Control si è fermato perché l'account connesso è cambiato
</h3>

Claude Code mostra questa riga durante una sessione [Remote Control](/docs/it/remote-control) quando vi accedete a un account claude.ai diverso o a un'organizzazione diversa su questa macchina. Avete effettuato il cambio al di fuori della sessione Claude Code, ad esempio eseguendo `/login` in un altro terminale.

Una sessione Remote Control che avete avviato mentre eravate connessi tramite `/login` appartiene all'account claude.ai e all'organizzazione che erano connessi al momento.

```text theme={null}
Remote Control disconnected — signed-in claude.ai account or organization changed on this machine — run /remote-control to start a session for the current account, or /login to switch back, then /remote-control
```

Claude Code ferma la sessione Remote Control non appena claude.ai conferma che l'account o l'organizzazione è cambiato. La Vostra sessione locale continua a funzionare senza Remote Control.

**Cosa fare:**

* Eseguite `/remote-control` per avviare una nuova sessione Remote Control con l'account o l'organizzazione corrente
* Per tornare indietro, eseguite `/login` e accedete di nuovo all'account o all'organizzazione precedente. Quindi eseguite `/remote-control`.

Prima della v2.1.234, Claude Code non notava quando vi passavate a un account o un'organizzazione diversa al di fuori della sessione Claude Code. Claude Code manteneva la sessione Remote Control connessa fino a quando una richiesta successiva al server Remote Control non falliva con `Remote Control server rejected the request (HTTP 404)`. Quel fallimento potrebbe arrivare ore dopo il cambio.

<h3 id="remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts">
  Remote Control si è fermato perché l'app che esegue la sessione si è disconnessa o ha cambiato account
</h3>

Quando l'app desktop Claude o un IDE ospita la Vostra sessione, Claude Code ottiene il Vostro token di login da quell'app piuttosto che da `/login`. Quando claude.ai rifiuta quel token, Claude Code chiede all'app uno nuovo. Se l'app risponde che è disconnessa, o che è ora connessa a un account Claude diverso, Claude Code termina la sessione [Remote Control](/docs/it/remote-control) e invia all'app una di queste righe:

```text theme={null}
Remote Control stopped — the app running this session is now signed in to a different Claude account
Remote Control stopped — the app running this session is signed out of Claude. Sign in there, then turn Remote Control back on
```

La Vostra sessione locale continua a funzionare senza Remote Control.

**Cosa fare:**

* Se l'app è disconnessa, accedete di nuovo, quindi riattivate Remote Control nell'app
* Se l'app ha cambiato account, Claude Code non può continuare la sessione terminata con il nuovo account. Avviate una nuova sessione Remote Control con quell'account.

Prima della v2.1.238, Claude Code inviava all'app i messaggi `run /login` elencati sotto [Remote Control non ha potuto aggiornare il Vostro login](#remote-control-couldnt-refresh-your-login) in entrambi i casi.

<h3 id="oauth-token-revoked-or-expired">
  Token OAuth revocato o scaduto
</h3>

Il Vostro login salvato non è più valido. Un token revocato significa che vi siete disconnessi ovunque o un amministratore ha rimosso l'accesso; un token scaduto significa che l'aggiornamento automatico è fallito a metà sessione.

Entrambi i messaggi segnalano un rifiuto che l'API ha restituito per una richiesta che Claude Code ha inviato. Quando il login salvato è già stato cancellato dopo un aggiornamento fallito, vedete [Login scaduto](#login-expired) invece. Se vi autenticate con un token di lunga durata in [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/it/env-vars), vedete gli stessi messaggi quando quel token scade o viene revocato.

```text theme={null}
OAuth token revoked · Please run /login
Please run /login · API Error: 401 OAuth token has expired ...
```

**Cosa fare:**

* Eseguite `/login` per accedere di nuovo
* Se l'errore ritorna nella stessa sessione dopo la riauthenticazione, eseguite prima `/logout` per cancellare completamente il token memorizzato, quindi `/login`
* Se vi autenticate con la variabile d'ambiente `CLAUDE_CODE_OAUTH_TOKEN`, Claude Code continua a inviare il valore che avete impostato dopo che una richiesta fallisce con un 401, piuttosto che passare al token di un login salvato. [`/status`](/docs/it/commands) mostra questa credenziale come una riga `Auth token` che legge `CLAUDE_CODE_OAUTH_TOKEN`. Generare un token fresco con [`claude setup-token`](/docs/it/authentication#generate-a-long-lived-token) e riavviare con esso, oppure annullate l'impostazione della variabile ed eseguite `/login`. Prima della v2.1.225, Claude Code poteva sostituire il valore della variabile a metà sessione con il token di accesso di breve durata da un login salvato, e la sessione falliva di nuovo con errori 401 una volta che quel token scadeva.
* Per i prompt ripetuti di accesso tra i lanci, vedete i controlli dell'orologio di sistema e i passaggi di recupero dell'archiviazione delle credenziali di macOS in [Risoluzione dei problemi](/docs/it/troubleshoot-install#not-logged-in-or-token-expired)
* Per altri errori inclusi `403 Forbidden` e problemi del browser OAuth, vedete [Login e autenticazione](/docs/it/troubleshoot-install#login-and-authentication)

<h3 id="api-error-401-invalid-authentication-credentials">
  API Error: 401 Credenziali di autenticazione non valide
</h3>

L'API ha riconosciuto il formato della Vostra credenziale ma ha rifiutato l'account o l'organizzazione dietro di essa. Anthropic restituisce questo messaggio quando una credenziale è stata revocata di recente, quando un'organizzazione è stata disabilitata o ha rimosso il Vostro accesso, o quando l'account stesso è stato disattivato, quindi un token scaduto non è la causa. La credenziale può essere il Vostro login salvato o un `ANTHROPIC_API_KEY` approvato, e la correzione differisce, quindi iniziate eseguendo `/status` per vedere quale è attivo.

```text theme={null}
Please run /login · API Error: 401 Invalid authentication credentials
```

**Cosa fare:**

* Se `/status` mostra una riga `API key` che non è contrassegnata come non in uso, un [`ANTHROPIC_API_KEY`](/docs/it/authentication#authentication-precedence) approvato è la credenziale attiva e ha la precedenza sul Vostro login, quindi `/login` non lo sostituisce. Ruotate la chiave nella Console Claude, oppure ricadete sul Vostro abbonamento eseguendo `unset ANTHROPIC_API_KEY`, oppure in PowerShell `Remove-Item Env:ANTHROPIC_API_KEY`.
* Se `/status` mostra solo il Vostro login, eseguite `/login` una volta. Se la credenziale è stata revocata, un login fresco la sostituisce.
* Se lo stesso messaggio ritorna per lo stesso account di login, l'account o l'organizzazione non è più attivo. Controllate l'account e l'organizzazione che `/status` segnala, e chiedete al Vostro amministratore dell'organizzazione di ripristinare l'accesso.
* Se [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) punta a un [gateway LLM](/docs/it/llm-gateway), il testo dopo `401` è il messaggio del Vostro gateway piuttosto che di Anthropic, e `/login` non lo cambia. Correggete invece la credenziale che il Vostro gateway si aspetta.

<h3 id="login-expired">
  Login scaduto
</h3>

Claude Code ha tentato di rinnovare il Vostro login claude.ai o Claude Console salvato e il servizio OAuth ha rifiutato il token di aggiornamento memorizzato, quindi Claude Code ha cancellato le credenziali salvate. Dopo di che, ogni richiesta di modello si ferma localmente con questo messaggio prima di raggiungere l'API, perché solo `/login` può creare nuove credenziali.

Prima della v2.1.206, Claude Code inviava comunque la richiesta del modello con qualsiasi credenziale rimanesse nell'ambiente, e ogni modello falliva con [C'è un problema con il modello selezionato](#theres-an-issue-with-the-selected-model) o un 401 invece di un prompt per accedere.

```text theme={null}
Login expired · Please run /login
```

In [modalità non interattiva](/docs/it/headless) (`-p`) e [Agent SDK](/docs/it/agent-sdk/overview), il messaggio legge come segue, e il codice di errore strutturato è `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Questo non è lo stesso stato di [Token OAuth revocato o scaduto](#oauth-token-revoked-or-expired). Quei messaggi segnalano un rifiuto che l'API ha restituito. Claude Code stesso produce `Login expired` per un login che ha già fallito di rinnovare, quindi non invia alcuna richiesta. Quando il rinnovo fallisce perché l'account stesso è sospeso piuttosto che il login essere obsoleto, Claude Code mostra [Il Vostro account è in sospeso](#your-account-is-on-hold) invece.

Le sessioni autenticate con una chiave API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/it/env-vars) o un provider di terze parti non utilizzano il login salvato e non vedono mai questo messaggio.

Potete controllare questo stato prima che una richiesta fallisca: [`/status`](/docs/it/commands) mostra una riga `Login` che legge `Expired — log in again`, più l'organizzazione e l'email che ha salvato per il login scaduto. La riga appare solo quando il login salvato è la Vostra credenziale attiva e non può più essere rinnovato. Le sessioni autenticate in un altro modo non mostrano la riga, anche se un login scaduto rimane salvato. Prima della v2.1.210, `/status` non dava alcuna indicazione in questo stato che un login fosse mai esistito, perché la credenziale cancellata non le lasciava nulla da segnalare.

**Cosa fare:**

* Eseguite `/login` per accedere di nuovo. Riprovare senza accedere mostra lo stesso messaggio su ogni richiesta.
* In modalità non interattiva, eseguite `claude` nello stesso ambiente, completate `/login`, quindi rieseguite il Vostro comando. Per l'automazione che non può accedere in modo interattivo, autenticate con `ANTHROPIC_API_KEY` o [generate un token di lunga durata con `claude setup-token`](/docs/it/authentication#generate-a-long-lived-token).
* Se l'accesso continua a fallire, vedete [Login e autenticazione](/docs/it/troubleshoot-install#login-and-authentication)

<h3 id="claude-login-not-accepted">
  Login Claude non accettato
</h3>

Avete tentato di avviare una [sessione cloud](/docs/it/claude-code-on-the-web), e il server ha rifiutato di crearla con un 401: non ha accettato il login Claude che questa macchina ha inviato, di solito perché il login è scaduto o è stato revocato.

La prima parte della riga è il motivo del server quando ne fornisce uno. Altrimenti la riga legge:

```text theme={null}
Claude login not accepted · Run /login, then try again
```

**Cosa fare:**

* Eseguite `/login`, completate l'accesso, quindi avviate di nuovo la sessione

<h3 id="artifacts-need-a-claude-ai-login">
  Artifacts hanno bisogno di un login claude.ai
</h3>

Claude Code ha rifiutato una pubblicazione o lettura di [artifact](/docs/it/artifacts) perché la sessione non ha alcun login claude.ai che possa utilizzare per gli artifact.

Ogni forma del messaggio inizia con le stesse parole, seguita da un rimedio che dipende da come la Vostra sessione si autentica. Senza alcuna credenziale in competizione legge:

```text theme={null}
Artifacts need a claude.ai login. Run /login and select "Claude account with subscription", then retry — the "Anthropic Console account" option does not provide claude.ai credentials.
```

**Cosa fare:**

* Eseguite `/login` e selezionate **Claude account with subscription**. L'opzione **Anthropic Console account** non fornisce credenziali claude.ai.
* Quando il messaggio nomina una credenziale che ha la precedenza, come `ANTHROPIC_API_KEY`, un'impostazione `apiKeyHelper` o una chiave Console salvata da un `/login` precedente, rimuovetela nel modo che il messaggio dice, quindi eseguite `/login`
* Quando il messaggio dice che questa sessione remota si autentica attraverso la macchina che l'ha lanciata, accedete a claude.ai su quella macchina, quindi riconnettete la sessione
* Quando il messaggio dice che la credenziale è iniettata dall'ambiente host della sessione, non potete cambiarla in quella sessione; avviate una sessione che è connessa a claude.ai
* Vedete [Disponibilità](/docs/it/artifacts#availability) per gli altri requisiti che gli artifact hanno, come piano, provider di modello e politica dell'organizzazione

<h3 id="administrator-policy-requires-a-cloud-gateway-sign-in">
  La politica dell'amministratore richiede un accesso al gateway Cloud
</h3>

Un [impostazione gestita](/docs/it/managed-settings) di un amministratore su questa macchina ha impostato [`forceLoginMethod`](/docs/it/settings-reference#forceloginmethod) a `"gateway"` o ha impostato [`forceLoginGatewayUrl`](/docs/it/settings-reference#forcelogingatewayurl). A meno che non selezioniate un provider cloud attraverso una variabile come `CLAUDE_CODE_USE_BEDROCK`, Claude Code accetta quindi solo l'accesso [gateway delle app Claude](/docs/it/claude-apps-gateway). Vedete uno di due messaggi:

```text theme={null}
Not signed in to the Cloud gateway — run /login.
```

Le richieste di modello falliscono con questo messaggio quando la sessione non ha alcun accesso al gateway, ad esempio perché non avete eseguito `/login` da quando la politica ha raggiunto la macchina.

Se avete anche una credenziale `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper` configurata e le impostazioni gestite impostano `forceLoginMethod`, Claude Code esce all'avvio invece con un messaggio che inizia:

```text theme={null}
Administrator policy requires a Cloud gateway sign-in on this machine; the
Anthropic-issued credential configured here (ANTHROPIC_API_KEY,
ANTHROPIC_AUTH_TOKEN, or apiKeyHelper) is not used.
```

**Cosa fare:**

* Eseguite `/login` e completate l'accesso sulla schermata **Cloud gateway**
* Per il messaggio di avvio, rimuovete l'impostazione `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` o `apiKeyHelper` che avete configurato, quindi avviate `claude` ed eseguite `/login`
* Se ritenete che la macchina non dovrebbe richiedere il gateway, chiedete all'amministratore che la gestisce di rimuovere `forceLoginMethod` e `forceLoginGatewayUrl` dalle sue impostazioni gestite

Su v2.1.265, una regressione ha anche mostrato il primo messaggio in alcune configurazioni di gateway LLM e proxy che si autenticano con una chiave API, `apiKeyHelper` o intestazioni personalizzate, anche senza alcun requisito di amministratore sulla macchina. Aggiornate alla v2.1.266 o successiva. Non è necessario modificare la Vostra configurazione.

Prima della v2.1.261, su macchine che impostano `forceLoginMethod` a `"gateway"`, Claude Code utilizzava un login salvato rimasto invece di fallire le richieste di modello, e segnalava una credenziale d'ambiente configurata con `This machine's managed settings require a first-party login` invece del messaggio di avvio. Prima della v2.1.265, una macchina le cui impostazioni gestite impostano solo `forceLoginGatewayUrl` non richiedeva l'accesso al gateway, e Claude Code utilizzava una credenziale rimasta lì.

<h3 id="your-account-is-on-hold">
  Il Vostro account è in sospeso
</h3>

L'account Claude dietro il Vostro login è stato sospeso. Claude Code mostra il primo messaggio quando tenta di rinnovare il Vostro login salvato e apprende della sospensione, e il secondo quando un accesso che completate nel browser lo segnala:

```text theme={null}
Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted
Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted
```

L'accesso di nuovo con lo stesso account non cancella il messaggio, perché la sospensione è sull'account piuttosto che sul login. In [modalità non interattiva](/docs/it/headless) (`-p`) e [Agent SDK](/docs/it/agent-sdk/overview), il codice di errore strutturato è `account_on_hold`. Prima della v2.1.235, Claude Code segnalava un account sospeso come [Login scaduto · Please run /login](#login-expired), i cui passaggi di recupero non possono cancellare una sospensione.

**Cosa fare:**

* Aprite il link nel messaggio per visualizzare i dettagli della sospensione o presentare ricorso
* Se avete un altro account Claude o una chiave API che non è interessata dalla sospensione, potete continuare a lavorare mentre la sospensione viene risolta: eseguite `/login` con quell'account, oppure impostate la chiave con `ANTHROPIC_API_KEY`

<h3 id="anthropic-profile-login-expired">
  Login del profilo Anthropic scaduto
</h3>

Claude Code si sta autenticando attraverso un profilo di credenziale Anthropic la cui credenziale di login salvata è scaduta, e il profilo non contiene alcuna credenziale di aggiornamento che Claude Code possa utilizzare per rinnovarla. Claude Code ferma ogni richiesta localmente senza riprovare, perché un nuovo tentativo leggerebbe la stessa credenziale scaduta.

```text theme={null}
Anthropic profile login expired · Re-authenticate your Anthropic profile
Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile
```

Questo appare solo quando la credenziale attiva proviene da un profilo di credenziale Anthropic, uno che selezionate con la variabile d'ambiente `ANTHROPIC_PROFILE`, che Claude Code scopre come il profilo attivo nella Vostra directory di configurazione Anthropic, o che Claude Code ha scritto quando vi siete [acceduti senza una chiave API](/docs/it/authentication#sign-in-without-an-api-key). Le sessioni che si autenticano con l'opzione claude.ai di `/login`, una chiave API, un token bearer come `ANTHROPIC_AUTH_TOKEN` o un provider di terze parti non vedono mai questo messaggio.

Su una macchina che [offre l'accesso senza chiave](/docs/it/authentication#sign-in-without-an-api-key), eseguite `/login`, scegliete l'account Anthropic Console e accedete di nuovo per rinnovare un profilo che l'accesso Console senza chiave o il CLI della Claude Platform `ant auth login` ha scritto. Claude Code sostituisce la credenziale scaduta in quel profilo. Per un profilo di federazione o uno che un altro strumento ha creato, `/login` non rinnova la credenziale. Quale forma vedete dipende dal fatto che abbiate selezionato il profilo o Claude Code l'abbia scoperto:

* Quando impostate `ANTHROPIC_PROFILE` esplicitamente, il messaggio termina con `Re-authenticate your Anthropic profile`.
* Quando Claude Code ha scoperto il profilo dalla Vostra directory di configurazione, il messaggio offre `/login`, perché Claude Code dà la precedenza a un `/login` funzionante rispetto al profilo scoperto e quindi si autentica con il Vostro account claude.ai o Console invece. Prima della v2.1.234, Claude Code mostrava il modulo `Re-authenticate your Anthropic profile` anche in questo caso.

**Cosa fare:**

* Accedete di nuovo al profilo, quindi riprovate: su una macchina che [offre l'accesso senza chiave](/docs/it/authentication#sign-in-without-an-api-key), eseguite `/login` e scegliete l'account Anthropic Console per un profilo che l'accesso Console senza chiave o il CLI della Claude Platform `ant auth login` ha scritto; per altri profili, utilizzate lo strumento che li ha creati
* Se un amministratore ha fornito la credenziale del profilo, chiedetegli di emetterne una nuova
* Eseguite `/status` per confermare la fonte di credenziale attiva e il nome del profilo
* Per smettere di utilizzare il profilo, annullate l'impostazione di `ANTHROPIC_PROFILE` se l'avete impostato, quindi autenticate in un altro modo, come `/login` o `ANTHROPIC_API_KEY`

<h3 id="oauth-scope-requirement">
  Requisito di ambito OAuth
</h3>

Il token memorizzato precede un ambito di autorizzazione che una funzione più recente necessita. Vedete questo più spesso da `/usage` e dall'indicatore di utilizzo della riga di stato:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**Cosa fare:**

* Eseguite `/login` per ottenere un nuovo token con gli ambiti attuali. Non è necessario disconnettervi prima.

<h3 id="claude-ai-rejected-the-session-token">
  claude.ai ha rifiutato il token della sessione
</h3>

Una richiesta [connettore claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) è fallita perché claude.ai ha rifiutato il token dal Vostro login Claude Code, di solito un login che è scaduto e non ha potuto essere rinnovato. Il token rifiutato è il Vostro login, non l'autorizzazione del connettore in claude.ai, quindi autorizzare di nuovo il connettore non lo risolve. In `/mcp`, il connettore mostra come `connected · session token rejected` e la sua vista dettagliata legge:

```text theme={null}
claude.ai rejected the session token. Run /login, then reconnect.
```

**Cosa fare:**

* Eseguite `/login` per accedere di nuovo
* Riconnettete il connettore da `/mcp`, oppure eseguite `/mcp reconnect <server>`. Riconnettere prima di accedere di nuovo lascia il connettore nello stesso stato. L'opzione **Reconnect** del pannello `/mcp` segnala `your claude.ai session token was rejected`; il modulo `/mcp reconnect <server>` digitato segnala una riconnessione riuscita anche se il token è ancora rifiutato.

Prima della v2.1.222, Claude Code contrassegnava il connettore come necessitante di autenticazione invece, che vi indicava il flusso di autorizzazione del connettore anche se completarlo non risolveva lo stato.

<h3 id="mcp-server-needs-you-to-sign-in-again">
  MCP server ha bisogno che vi accediate di nuovo
</h3>

Un server [MCP](/docs/it/mcp) remoto ha rifiutato la credenziale su una chiamata di strumento a metà sessione, di solito perché un accesso o un token è scaduto o perché il token manca di un'autorizzazione che lo strumento necessita. La chiamata di strumento fallisce, e `/mcp` contrassegna il server come [necessitante di autenticazione](/docs/it/mcp#authenticate-with-remote-mcp-servers).

Per un server a cui vi accedete da Claude Code, incluso un connettore claude.ai, l'accesso è scaduto o è stato revocato:

```text theme={null}
MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)
```

Eseguite `/mcp`, selezionate il server e accedete di nuovo dal suo menu.

Per un server configurato con uno script [`headersHelper`](/docs/it/mcp#use-dynamic-headers-for-custom-authentication), Claude Code ha già rieseguito l'helper e ritentato la chiamata una volta prima di mostrare questo:

```text theme={null}
MCP server "<name>" rejected the credential from its headersHelper (check the helper and run /mcp to reconnect, or to authenticate if the server also uses OAuth)
```

Controllate che l'helper restituisca una credenziale che il server accetta, quindi riconnettete da `/mcp`, che riesegue l'helper.

Per un server con un'intestazione `Authorization` statica nella sua configurazione:

```text theme={null}
MCP server "<name>" rejected the Authorization header in its config (update it, then run /mcp to reconnect)
```

Aggiornate il valore dell'intestazione dove il server è configurato, quindi riconnettete da `/mcp`.

Prima della v2.1.273, i casi di accesso scaduto, `headersHelper` e intestazione `Authorization` mostravano tutti `MCP server "<name>" requires re-authorization (token expired)`.

Un server può anche rifiutare una chiamata di strumento con HTTP 403 `insufficient_scope` per chiedere di autorizzare un ambito, a volte uno che il Vostro token già elenca. Il messaggio nomina quell'ambito:

```text theme={null}
MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate
```

Eseguite `/mcp`, selezionate il server e autenticate di nuovo dal suo menu.

Quando la configurazione del server non imposta né [`oauth.scopes`](/docs/it/mcp#restrict-oauth-scopes) né [`authServerMetadataUrl`](/docs/it/mcp#override-oauth-metadata-discovery), Claude Code richiede l'ambito che il server ha nominato. Con una delle due impostazioni, Claude Code richiede gli ambiti di quell'impostazione invece. Se avete fissato `oauth.scopes`, aggiungete l'ambito mancante a quell'elenco prima di autenticare di nuovo.

Prima della v2.1.274, questo caso mostrava il messaggio `needs you to sign in again`, e prima della v2.1.273 mostrava `requires re-authorization (token expired)` come gli altri casi.

<h3 id="issuer-mismatch-in-authorization-response">
  Mancata corrispondenza dell'emittente nella risposta di autorizzazione
</h3>

Durante un [accesso MCP OAuth](/docs/it/mcp#authenticate-with-remote-mcp-servers), il server di autorizzazione ha reindirizzato di nuovo a Claude Code con un parametro `iss` che non nomina l'emittente che Claude Code si aspettava dai metadati OAuth del server. Un emittente sbagliato a questo passaggio è come appare un attacco di mix-up del server di autorizzazione, quindi Claude Code fallisce l'accesso invece di scambiare il codice di autorizzazione. Claude Code mostra l'errore nel menu del server `/mcp` dopo l'accesso del browser:

```text theme={null}
Issuer mismatch in authorization response (RFC 9207): expected "https://auth.example.com", received "https://other.example.com"
```

`expected` è l'emittente dai metadati OAuth del server, e `received` è il valore `iss` che il reindirizzamento ha trasportato. Un accesso il cui reindirizzamento non trasporta alcun parametro `iss` passa il controllo, a meno che i metadati del server non impostino `authorization_response_iss_parameter_supported`, nel qual caso Claude Code fallisce l'accesso.

**Cosa fare:**

* Riprovate l'accesso da `/mcp`
* Se l'errore si ripete, segnalarlo all'operatore del server. La correzione è lato server: il server di autorizzazione deve restituire lo stesso emittente nel parametro `iss` che pubblicizza nei suoi metadati
* Per connettervi mentre il server viene corretto, avviate Claude Code con [`MCP_SDK_GENERATION=v1`](/docs/it/env-vars), il cui [runtime](/docs/it/mcp#mcp-client-runtimes) non esegue questo controllo. Questo rimuove una protezione contro gli attacchi di mix-up, quindi preferite la correzione lato server

Prima della v2.1.232, Claude Code utilizzava il runtime v2 solo in un rollout graduale o quando impostavate `MCP_SDK_GENERATION=v2`.

<h3 id="aws-credentials-expired-or-invalid">
  Credenziali AWS scadute o non valide
</h3>

Il Vostro token di sessione AWS è scaduto o è stato rifiutato. Questo messaggio appare su un 401 da [Claude Platform su AWS](/docs/it/claude-platform-on-aws) o dall'[endpoint Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint), che è come quei provider segnalano un token di sicurezza scaduto.

Il suggerimento di azione nel mezzo varia a seconda della Vostra configurazione. La parte stabile è il `AWS credentials expired or invalid` iniziale:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Prima della v2.1.273, questo messaggio appariva solo quando `awsAuthRefresh` era configurato.

**Cosa fare:**

* Se il suggerimento dice che le credenziali sono gestite da questo ambiente, l'app che ha lanciato Claude Code possiede la credenziale e gli altri passaggi qui non si applicano: riprovate, o contattate il Vostro amministratore
* Se [`awsAuthRefresh`](/docs/it/amazon-bedrock#advanced-credential-configuration) è impostato, eseguite il comando nominato nel messaggio, come `aws sso login --profile myprofile`, in un altro terminale e completate l'accesso del browser, quindi riprovate. Altrimenti aggiornate la credenziale AWS che utilizzate voi stessi: il Vostro accesso SSO, le chiavi di accesso, la chiave API o il token proxy
* Con `awsAuthRefresh` impostato in una sessione interattiva, potete invece eseguire `/login`, scegliere **3rd-party platform**, quindi selezionare **Claude Platform on AWS · refresh credentials** sotto **Using 3rd-party platforms** per eseguire lo stesso comando senza riavviare Claude Code. Vedete [Configurare le credenziali AWS](/docs/it/claude-platform-on-aws#1-configure-aws-credentials)
* Se l'errore si ripete dopo che il comando di aggiornamento ha avuto successo, confermate che l'identità è valida al di fuori di Claude Code con `aws sts get-caller-identity` nella stessa shell e profilo

<h3 id="aws-authentication-failed">
  Autenticazione AWS non riuscita
</h3>

Il Vostro provider AWS ha restituito un 403, oppure [Amazon Bedrock](/docs/it/amazon-bedrock) ha restituito un 401.

Amazon Bedrock segnala un token di sicurezza scaduto come un 403, ma un 403 è anche come segnala un rifiuto di autorizzazione, come un `AccessDeniedException` da un'autorizzazione IAM mancante. Claude Code non può dire quale causa avete colpito.

Un 401 da Amazon Bedrock atterra anche qui piuttosto che sotto [Credenziali AWS scadute o non valide](#aws-credentials-expired-or-invalid), perché Amazon Bedrock non segnala un token scaduto come un 401. Un 401 da quell'endpoint di solito proviene da qualcos'altro nel percorso della richiesta, come un proxy aziendale.

Un aggiornamento delle credenziali corregge un token scaduto e non può correggere le altre cause, quindi il messaggio offre entrambi:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

Il suggerimento di azione nel mezzo varia a seconda della Vostra configurazione. La parte stabile è il `AWS authentication failed` iniziale.

Quando il 403 è la risposta di Amazon Bedrock che non avete accesso al modello con l'ID modello specificato, il suggerimento invece vi dice di abilitare il modello per il Vostro account e la Vostra regione nella console Amazon Bedrock.

Prima della v2.1.273, questo messaggio appariva solo quando `awsAuthRefresh` era configurato.

**Cosa fare:**

* Se il suggerimento dice che le credenziali sono gestite da questo ambiente, l'app che ha lanciato Claude Code possiede la credenziale e gli altri passaggi qui non si applicano: riprovate, o contattate il Vostro amministratore
* Aggiornate le Vostre credenziali AWS nel caso in cui una credenziale scaduta sia la causa: eseguite il comando [`awsAuthRefresh`](/docs/it/amazon-bedrock#advanced-credential-configuration) nominato nel messaggio quando uno è impostato, o aggiornate il Vostro accesso SSO, le chiavi di accesso, la chiave API o il token proxy voi stessi
* Se le Vostre credenziali sono attuali, confermate che le autorizzazioni IAM in [Configurazione IAM](/docs/it/amazon-bedrock#iam-configuration) siano allegate all'identità che state utilizzando e che il modello selezionato sia abilitato per il Vostro account e la Vostra regione
* Eseguite `aws sts get-caller-identity` per confermare quale identità le Vostre richieste utilizzano; un `AWS_PROFILE` obsoleto o un profilo predefinito è una causa comune di una mancata corrispondenza di autorizzazione

<h3 id="google-cloud-credentials-expired-or-invalid">
  Credenziali Google Cloud scadute o non valide
</h3>

Le Vostre credenziali Google Cloud per [Agent Platform di Google Cloud](/docs/it/google-vertex-ai) sono scadute o sono state rifiutate: la richiesta ha restituito un 401, che è come Agent Platform segnala la scadenza delle credenziali.

Il suggerimento di azione nel mezzo varia a seconda della Vostra configurazione. La parte stabile è il `Google Cloud credentials expired or invalid` iniziale:

```text theme={null}
Google Cloud credentials expired or invalid · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · API Error: 401 ...
```

**Cosa fare:**

* Se il suggerimento dice che le credenziali sono gestite da questo ambiente, l'app che ha lanciato Claude Code possiede la credenziale e gli altri passaggi qui non si applicano: riprovate, o contattate il Vostro amministratore
* Se vi autenticate con le credenziali predefinite dell'applicazione, eseguite il comando [`gcpAuthRefresh`](/docs/it/google-vertex-ai#advanced-credential-configuration) nominato nel messaggio, o `gcloud auth application-default login`, e completate l'accesso, quindi riprovate
* Se instradare attraverso un [gateway LLM](/docs/it/llm-gateway) con `CLAUDE_CODE_SKIP_VERTEX_AUTH` impostato, aggiornate il token del gateway in `ANTHROPIC_AUTH_TOKEN` o `ANTHROPIC_CUSTOM_HEADERS`, quindi riprovate
* Se vi autenticate con un file di chiave dell'account di servizio, confermate che `GOOGLE_APPLICATION_CREDENTIALS` punti a una chiave valida. Vedete [Configurare le credenziali GCP](/docs/it/google-vertex-ai#3-configure-gcp-credentials)
* Se l'errore si ripete dopo un aggiornamento, confermate che l'identità funziona al di fuori di Claude Code con `gcloud auth application-default print-access-token` nella stessa shell

Prima della v2.1.273, un 401 da Agent Platform mostrava il messaggio generico `Please run /login` o `Failed to authenticate` invece, che non può aggiornare le credenziali Google Cloud.

<h3 id="google-cloud-authentication-failed">
  Autenticazione Google Cloud non riuscita
</h3>

[Agent Platform di Google Cloud](/docs/it/google-vertex-ai) ha restituito un 403, che utilizza per i rifiuti di autorizzazione piuttosto che per le credenziali scadute. Di solito l'identità con cui vi autenticate manca di un'autorizzazione IAM, oppure il modello non è abilitato per il Vostro progetto.

Il suggerimento di azione nel mezzo varia a seconda della Vostra configurazione. La parte stabile è il `Google Cloud authentication failed` iniziale:

```text theme={null}
Google Cloud authentication failed · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · if credentials are current, check GCP IAM permissions and Vertex AI model access · API Error: 403 ...
```

**Cosa fare:**

* Se il suggerimento dice che le credenziali sono gestite da questo ambiente, l'app che ha lanciato Claude Code possiede la credenziale e gli altri passaggi qui non si applicano: riprovate, o contattate il Vostro amministratore
* Confermate che i ruoli in [Configurazione IAM](/docs/it/google-vertex-ai#iam-configuration) siano concessi all'identità con cui vi autenticate
* Confermate che il modello sia abilitato per il Vostro progetto. Vedete [Richiedere l'accesso al modello](/docs/it/google-vertex-ai#2-request-model-access)

Prima della v2.1.273, un 403 da Agent Platform mostrava il messaggio generico `Please run /login` o `Failed to authenticate` invece, che non può aggiornare le credenziali Google Cloud.

<h3 id="microsoft-foundry-authentication-failed">
  Autenticazione Microsoft Foundry non riuscita
</h3>

[Microsoft Foundry](/docs/it/microsoft-foundry) ha restituito un 401 o 403: la credenziale Azure sulla richiesta è stata rifiutata, oppure l'identità dietro di essa non ha accesso alla risorsa Foundry. `/login` non può coniare credenziali Azure. Il suggerimento di azione nel mezzo varia a seconda della Vostra configurazione. La parte stabile è il `Microsoft Foundry authentication failed` iniziale:

```text theme={null}
Microsoft Foundry authentication failed · refresh your Foundry credential (ANTHROPIC_FOUNDRY_AUTH_TOKEN, ANTHROPIC_FOUNDRY_API_KEY, Azure sign-in for Entra, or your proxy token) and retry · if credentials are current, check access to the Foundry resource · API Error: 401 ...
```

**Cosa fare:**

* Se il suggerimento dice che le credenziali sono gestite da questo ambiente, l'app che ha lanciato Claude Code possiede la credenziale e gli altri passaggi qui non si applicano: riprovate, o contattate il Vostro amministratore
* Aggiornate la credenziale che avete configurato in [Configurare le credenziali Azure](/docs/it/microsoft-foundry#2-configure-azure-credentials): ruotate `ANTHROPIC_FOUNDRY_API_KEY`, coniate un `ANTHROPIC_FOUNDRY_AUTH_TOKEN` fresco, o eseguite `az login` in modo che la catena di credenziali Microsoft Entra predefinita possa accedere di nuovo
* Se la credenziale è attuale, confermate che l'identità ha accesso alla risorsa Foundry. Vedete [Configurazione RBAC di Azure](/docs/it/microsoft-foundry#azure-rbac-configuration)

Prima della v2.1.273, un 401 o 403 da Microsoft Foundry mostrava il messaggio generico `Please run /login` o `Failed to authenticate` invece, che non può aggiornare le credenziali Azure.

<h3 id="could-not-load-aws-or-google-cloud-credentials">
  Impossibile caricare le credenziali AWS o Google Cloud
</h3>

Claude Code non ha potuto ottenere credenziali utilizzabili dalla catena del provider di credenziali AWS o dalle Vostre credenziali predefinite dell'applicazione Google sulla macchina su cui viene eseguito, quindi nessuna richiesta ha raggiunto il Vostro provider cloud. Claude Code cancella le Vostre credenziali memorizzate nella cache e ritenta due volte prima di mostrare questo messaggio. Il dettaglio dopo il `·` nomina la causa specifica, come una sessione SSO scaduta, credenziali predefinite mancanti segnalate come `Could not load the default credentials`, o un accesso revocato segnalato come `invalid_grant`:

```text theme={null}
API Error: Could not load AWS credentials · Could not load credentials from any providers. Check or refresh your AWS credentials and try again.
API Error: Could not load Google Cloud credentials · invalid_grant. Check or refresh your Google Cloud credentials and try again.
```

In [modalità non interattiva](/docs/it/headless) con `-p` e in [Agent SDK](/docs/it/agent-sdk/overview), il codice di errore strutturato è `cloud_credential_error`. Prima della v2.1.267, il messaggio mostrava solo il testo di dettaglio dopo `API Error:`, e il codice strutturato era `server_error` o `unknown`.

**Cosa fare:**

* Eseguite il comando di accesso del Vostro provider, come `aws sso login --profile myprofile` o `gcloud auth application-default login`, quindi riprovate. [Credenziali Bedrock, Agent Platform o Foundry non si caricano](/docs/it/troubleshoot-install#bedrock-agent-platform-or-foundry-credentials-not-loading) mostra come confermare le credenziali al di fuori di Claude Code
* Se il dettaglio legge `AWS default-chain credential resolve timed out`, la catena si è bloccata piuttosto che fallire, quindi seguite [Timeout della risoluzione delle credenziali della catena predefinita AWS](#aws-default-chain-credential-resolve-timed-out) invece

<h3 id="aws-default-chain-credential-resolve-timed-out">
  Timeout della risoluzione delle credenziali della catena predefinita AWS
</h3>

La catena del provider di credenziali predefinite AWS non ha prodotto credenziali entro 60 secondi, quindi Claude Code ha fermato la risoluzione e ha fallito la richiesta. Questo timeout è una causa di [Impossibile caricare le credenziali AWS o Google Cloud](#could-not-load-aws-or-google-cloud-credentials). L'errore è la risoluzione delle credenziali locali: la richiesta non ha mai raggiunto [Amazon Bedrock](/docs/it/amazon-bedrock), [Claude Platform su AWS](/docs/it/claude-platform-on-aws) o l'[endpoint Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint). Claude Code cancella la Vostra [cache delle credenziali](/docs/it/amazon-bedrock#credential-caching-and-resolution-timeout) e ritenta prima che questo errore emerga, quindi al momento in cui lo vedete la catena si è bloccata su tentativi ripetuti.

```text theme={null}
API Error: Could not load AWS credentials · AWS default-chain credential resolve timed out. Check or refresh your AWS credentials and try again.
```

Le cause comuni sono un comando `credential_process` nel Vostro profilo AWS che attende un input che non può ricevere, e un contenitore o una VM il cui servizio di metadati dell'istanza (IMDS) non risponde mai al probe della catena.

Prima della v2.1.267, il messaggio leggeva `API Error: AWS default-chain credential resolve timed out`.
Prima della v2.1.207, una catena bloccata lasciava la richiesta in attesa indefinitamente invece di fallire.

**Cosa fare:**

* Eseguite `aws sts get-caller-identity` nella stessa shell con lo stesso `AWS_PROFILE`. Se si blocca anche, correggete il profilo; un comando `credential_process` che richiede in modo interattivo è una causa comune.
* Completate il passaggio di accesso prima di avviare Claude Code, ad esempio `aws sso login --profile myprofile`, in modo che la catena si risolva dalla cache SSO locale invece di attendere un flusso del browser
* Se la Vostra catena esegue un accesso interattivo che legittimamente ha bisogno di più di 60 secondi, come SSO con MFA attraverso un wrapper come `aws-vault`, aumentate il limite in millisecondi con [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/it/env-vars)

<h3 id="bedrock-setup-verification-timed-out-waiting-for-aws">
  Timeout della verifica della configurazione di Bedrock in attesa di AWS
</h3>

Una chiamata ad AWS durante la [procedura guidata di configurazione di Bedrock](/docs/it/amazon-bedrock#sign-in-with-bedrock), come la ricerca delle credenziali o il controllo dell'identità, non è stata completata entro il limite di 60 secondi. La procedura guidata smette di attendere e fallisce il passaggio di verifica:

```text theme={null}
Timed out after 60s waiting for AWS. Check your network and proxy settings; if a credential helper needs longer to prompt you, raise CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS.
```

Il numero riflette il Vostro limite: 60 secondi per impostazione predefinita, o il valore che impostate in [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/it/env-vars).

Le cause comuni sono una rete o un proxy che blocca le richieste ad AWS, incluso l'aggiornamento del token SSO, e un helper di credenziali ancora in attesa di input che non potete vedere. Aumentate il limite solo quando l'helper legittimamente ha bisogno di più tempo.

Una singola richiesta bloccata ad AWS può anche fallire sul suo timeout per richiesta, che mostra un messaggio più breve sullo stesso passaggio:

```text theme={null}
A request to AWS timed out. Check your network and proxy settings, then try again.
```

Quando gli stessi timeout si verificano sul passaggio di pin del modello, la procedura guidata contrassegna un modello come `unreachable` invece di mostrare uno dei due messaggi.

**Cosa fare:**

* Eseguite `aws sts get-caller-identity` nella stessa shell. Se si blocca anche, il blocco è al di fuori di Claude Code, nella Vostra rete, nel Vostro proxy o nell'helper di credenziali nel Vostro profilo AWS; correggete prima quello.
* Completate qualsiasi accesso interattivo prima di aprire la procedura guidata, ad esempio `aws sso login --profile myprofile`
* Se un helper di credenziali nel Vostro profilo AWS legittimamente ha bisogno di più di 60 secondi per richiedervi, aumentate il limite in millisecondi con [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/it/env-vars)

<h3 id="cloud-gateway-session-expired">
  Sessione del gateway cloud scaduta
</h3>

Vi siete acceduti attraverso un [gateway delle app Claude](/docs/it/claude-apps-gateway), e la sessione del gateway salvata su questa macchina è scaduta e non ha potuto essere rinnovata, oppure il gateway non la accetta più, ad esempio dopo che il [JWT secret del gateway è stato sostituito](/docs/it/claude-apps-gateway-deploy#jwt-secret-rotation). Se vedete questa riga quando avviate `claude` in modo interattivo, la sessione si è aperta disconnessa dal gateway:

```text theme={null}
Cloud gateway session expired — run /login to reconnect.
```

La stessa riga può apparire a metà sessione quando la credenziale del gateway scade e Claude Code non può rinnovarla.

In un'esecuzione [non interattiva](/docs/it/headless), una sessione in background o altra sessione incustodita, o un sottocomando `claude` diverso da `claude auth`, Claude Code esce con questo messaggio invece quando il gateway non accetta più la sessione:

```text theme={null}
Cloud gateway <url> no longer accepts this session. Start `claude` and sign in again with /login.
```

**Cosa fare:**

* Eseguite `/login` nella sessione e completate l'accesso del browser
* Per un lancio non interattivo, avviate `claude` nello stesso ambiente, eseguite `/login`, quindi rieseguite il Vostro comando

<h3 id="sign-in-timed-out-while-waiting-for-you-to-continue">
  Sign-in scaduto mentre vi aspettava di continuare
</h3>

Durante un [accesso al gateway delle app Claude](/docs/it/claude-apps-gateway), il gateway ha nominato l'account che ha effettuato l'accesso, e Claude Code vi ha chiesto di confermarlo prima di salvare la credenziale. Avete lasciato la conferma aperta oltre la scadenza dell'accesso stesso, e il gateway non ha emesso alcun token di aggiornamento che potesse rinnovarlo, quindi Claude Code non ha memorizzato nulla quando avete continuato:

```text theme={null}
Sign-in timed out while waiting for you to continue. Try again.
```

**Cosa fare:**

* Eseguite `/login` di nuovo e confermate l'account prima che l'accesso scada

<h3 id="gateway-refused-the-request">
  Il gateway ha rifiutato la richiesta
</h3>

Siete connessi attraverso un [gateway delle app Claude](/docs/it/claude-apps-gateway), e una richiesta ha restituito un 403: il gateway, o l'upstream dietro di esso, l'ha rifiutata. L'accesso di nuovo non cambia un rifiuto, quindi il messaggio punta al Vostro amministratore del gateway:

```text theme={null}
Gateway refused the request · signing in again won't change this — check with your gateway administrator · API Error: 403 ...
```

**Cosa fare:**

* Chiedete al Vostro amministratore del gateway di cercare la richiesta. La coda `API Error:` trasporta il rifiuto che il gateway ha restituito
* Per gli amministratori: una [regola di controllo dell'accesso](/docs/it/claude-apps-gateway-config#http-tuning) sul gateway restituisce un 403 che il [log di audit](/docs/it/claude-apps-gateway-deploy#logs) registra con il suo motivo, e un rifiuto di autorizzazione dell'upstream passa attraverso per [Messaggi di errore dell'upstream](/docs/it/claude-apps-gateway-config#upstream-error-messages)

Prima della v2.1.273, un 403 su una sessione del gateway mostrava il messaggio generico `Please run /login` o `Failed to authenticate` invece, e l'accesso di nuovo non cancellava il rifiuto.

<h2 id="network-and-connection-errors">
  Errori di rete e connessione
</h2>

La maggior parte di questi errori significa che una richiesta di rete da Claude Code non ha raggiunto la sua destinazione, oppure qualcosa tra Claude Code e l'API ha alterato la risposta durante il percorso; quando una voce ha anche una causa locale, come un'archivio fallito, il corpo lo specifica. Di solito originano dalla tua rete locale, proxy o firewall, oppure dalla politica di rete dell'ambiente cloud.

<h3 id="unable-to-connect-to-api">
  Unable to connect to API
</h3>

La connessione TCP all'API non è riuscita o non si è mai completata. Per i codici di errore di connessione comuni, il nome del messaggio specifica il tipo di errore e mantiene il codice tra parentesi:

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

Un codice che Claude Code non riconosce appare come `Unable to connect to API` seguito dal codice tra parentesi. Alcuni di questi messaggi possono mostrare più di un codice: `Connection refused` può mostrare `ConnectionRefused` o `ECONNREFUSED`, ad esempio, e `Can't reach the API server` può mostrare `ENOTFOUND` o `FailedToOpenSocket`.

Prima della v2.1.227, ognuno di questi messaggi codificati leggeva `Unable to connect to API` seguito dal codice, ad esempio `Unable to connect to API (ECONNREFUSED)`.

Le cause comuni includono nessun accesso a Internet, una VPN che blocca `api.anthropic.com`, o un proxy aziendale richiesto che non è configurato.

**Cosa fare:**

* Conferma di poter raggiungere l'host API dalla stessa shell eseguendo `curl -I https://api.anthropic.com`. Su Windows PowerShell usa `curl.exe -I https://api.anthropic.com` in modo che l'alias `Invoke-WebRequest` integrato non sia utilizzato.
* Se sei dietro un proxy aziendale, imposta `HTTPS_PROXY` prima di avviare Claude Code e vedi [Network configuration](/docs/it/network-config)
* Se instrada attraverso un gateway LLM o un relay, imposta [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) al suo indirizzo. Vedi [Connect Claude Code to an LLM gateway](/docs/it/llm-gateway-connect) per la configurazione.
* Assicurati che il tuo firewall consenta gli host elencati in [Network access requirements](/docs/it/network-config#network-access-requirements)
* I guasti intermittenti vengono [ritentati automaticamente](#automatic-retries); i guasti persistenti indicano un problema di rete locale

Se `curl` ha successo ma Claude Code continua a fallire, la causa è solitamente qualcosa tra il runtime e la rete piuttosto che la rete stessa:

* Su Linux e WSL, controlla `/etc/resolv.conf` per un nameserver non raggiungibile. WSL in particolare può ereditare un resolver rotto dall'host.
* Su macOS, un client VPN che è stato disconnesso o disinstallato può lasciare dietro un'interfaccia tunnel o una regola di routing. Controlla `ifconfig` per interfacce `utun` stantie e rimuovi l'estensione di rete della VPN in Impostazioni di Sistema.
* Docker Desktop e runtime di container simili possono intercettare il traffico in uscita. Chiudili e riprova per escludere questa possibilità.

<h3 id="unable-to-connect-to-anthropic-services">
  Unable to connect to Anthropic services
</h3>

Durante la configurazione della prima esecuzione, Claude Code verifica di poter raggiungere `api.anthropic.com` e `platform.claude.com` prima di mostrare il passaggio di accesso. Quando uno dei controlli fallisce, Claude Code stampa il motivo ed esce.

```text theme={null}
Unable to connect to Anthropic services
Failed to connect to api.anthropic.com: ECONNREFUSED
Connection to api.anthropic.com timed out after 10 seconds
A proxy is configured via HTTPS_PROXY. Check that it allows connections to the host above.
```

Claude Code invia il controllo attraverso la stessa [proxy configuration](/docs/it/network-config) delle richieste API e assegna a ogni sonda 10 secondi. Quando la sonda fallita è passata attraverso un proxy, il messaggio nomina la variabile di ambiente che l'ha configurata, come `HTTPS_PROXY`. Prima della v2.1.222, il controllo utilizzava un diverso trasporto proxy senza timeout: dietro un URL proxy con lo schema `https://`, potrebbe bloccarsi su `Checking connectivity...` indefinitamente e poi fallire anche se le richieste API attraverso lo stesso proxy hanno successo.

Claude Code salta questo controllo quando un [managed settings file, MDM policy, o policy helper](/docs/it/managed-settings) imposta [`forceLoginMethod`](/docs/it/settings-reference#forceloginmethod) a `"gateway"`, o imposta [`forceLoginGatewayUrl`](/docs/it/settings-reference#forcelogingatewayurl) senza `forceLoginMethod`. Con entrambe le configurazioni, Claude Code apre il passaggio di accesso sulla schermata **Cloud gateway** piuttosto che su un metodo di accesso Anthropic. Claude Code salta anche il controllo quando una fonte di managed settings sulla macchina esiste ma non può essere letta, poiché quella fonte potrebbe contenere la configurazione del gateway. Prima della v2.1.247, Claude Code eseguiva il controllo anche sotto questa configurazione e usciva con questo errore quando gli endpoint di Anthropic non erano raggiungibili.

**Cosa fare:**

* Se il messaggio nomina una variabile proxy, controlla che il suo valore punti al proxy giusto e chiedi al tuo team di rete di consentire connessioni HTTPS attraverso di esso all'host nel messaggio. Vedi [Network configuration](/docs/it/network-config).
* Lavora attraverso i controlli in [Unable to connect to API](#unable-to-connect-to-api). Il test `curl` e la guida del firewall lì si applicano anche a questo controllo.
* Se la tua organizzazione accede attraverso un [cloud gateway](/docs/it/claude-apps-gateway) e questo errore appare al primo avvio, aggiorna a Claude Code v2.1.247 o successivo.
* Se la tua rete è aperta e il guasto persiste, Claude Code potrebbe non essere [disponibile nel tuo paese](https://www.anthropic.com/supported-countries)

<h3 id="socket-is-closed">
  Socket is closed
</h3>

`Socket is closed` significa che la connessione che trasporta una risposta in streaming è stata chiusa mentre la risposta stava ancora arrivando. La causa più comune è un proxy aziendale su Windows che interrompe un tunnel stabilito a metà risposta.

A seconda di quanto la risposta era progredita, Claude Code ritenta la richiesta, mantiene ciò che Claude ha prodotto, o termina il turno. Vedi [Automatic retries](#automatic-retries).

Prima della v2.1.214, Claude Code non ritentava questo guasto e il turno si fermava con un errore contenente `Socket is closed`.

**Cosa fare:**

* Se vedi questo errore, aggiorna a v2.1.214 o successivo con `claude update`, quindi invia di nuovo il tuo messaggio
* Se i turni continuano a fallire dietro lo stesso proxy dopo l'aggiornamento, lavora attraverso [Unable to connect to API](#unable-to-connect-to-api) e controlla la configurazione del proxy in [Network configuration](/docs/it/network-config)

<h3 id="api-returned-an-empty-or-malformed-response">
  API returned an empty or malformed response
</h3>

Claude Code mostra questo errore quando il suo ritentativo non in streaming di una richiesta in streaming fallita ottiene uno stato HTTP di successo ma il corpo non è un messaggio API Claude: comunemente una pagina di errore HTML o di accesso, un corpo vuoto, o JSON in un altro formato. Un proxy, gateway, o pagina di accesso di rete che risponde al posto dell'API è la solita fonte. Claude Code non ritenta la richiesta e il turno termina con questo errore.

```text theme={null}
API returned an empty or malformed response (HTTP 200) — check for a proxy or gateway intercepting the request.
```

Dopo quell'apertura, il messaggio segnala ciò che è tornato e quale richiesta ha fallito:

* Una clausola `Response:` con il tipo di contenuto, il tipo di corpo, come `body is an HTML page` o `empty body`, la sua dimensione in byte, e se la risposta ha portato un id di richiesta Anthropic. Quando la risposta nomina un server riconoscibile, come `nginx` o `cloudflare`, o porta intestazioni intermediarie, come `cf-ray` o `via`, la clausola elenca anche quelli.
* Una frase che nomina l'id della richiesta in streaming fallita e il guasto che ha attivato il ritentativo. Quando uno stream si era aperto prima del guasto, segnala anche quanti eventi di stream sono arrivati e, se ce ne sono stati, quanto tempo lo stream era stato silenzioso quando il tentativo è fallito.

Prima della v2.1.234, il messaggio terminava dopo `intercepting the request`.

Prima della v2.1.271, una risposta che portava un messaggio API valido sotto un tipo di contenuto non JSON come `text/plain` terminava anche il turno con questo errore. Alcuni gateway LLM utilizzano quel tipo di contenuto per la risposta non in streaming.

**Cosa fare:**

* Leggi la clausola `Response:` per vedere quale sistema ha risposto. Un corpo HTML, nessun id di richiesta Anthropic, o un server nominato come `nginx` o `cloudflare` significa che qualcosa tra Claude Code e l'API ha risposto al suo posto
* Se instrada attraverso un [LLM gateway](/docs/it/llm-gateway-connect#troubleshoot-gateway-errors), testa il percorso con una richiesta diretta e correggi l'hop che restituisce la risposta non-API
* Su una rete con una pagina di accesso, come Wi-Fi ospite, completa l'accesso in un browser, quindi riprova
* Se solo il percorso non in streaming attraverso il tuo gateway è rotto, imposta [`CLAUDE_CODE_DISABLE_NONSTREAMING_FALLBACK=1`](/docs/it/env-vars#variables) in modo che una richiesta che fallisce a metà stream vada al percorso di ritentativo normale invece di questo fallback, tranne quando l'endpoint di streaming stesso restituisce `404`, dove Claude Code continua comunque a fare fallback

<h3 id="streaming-response-ended-before-any-complete-data-was-received">
  Streaming response ended before any complete data was received
</h3>

Una risposta in streaming dal tuo provider di modelli è stata completata senza fornire dati utilizzabili, quindi Claude Code ha reinviato la richiesta senza streaming per terminare il turno. Claude Code mostra l'avviso una volta per sessione, solo in sessioni interattive. Prima della v2.1.239, Claude Code ritentava silenziosamente senza streaming.

```text theme={null}
Streaming response ended before any complete data was received. Retrying without streaming. If this keeps happening, check any proxy or gateway between Claude Code and your model provider.
```

Claude Code invia ogni richiesta interessata due volte: il tentativo di streaming vuoto e il ritentativo. La causa solita è un proxy o gateway che consuma o trasforma il corpo della risposta in streaming durante il percorso di ritorno.

**Cosa fare:**

* Configura qualsiasi proxy o gateway tra Claude Code e il tuo provider di modelli per passare i corpi della risposta in streaming e le loro intestazioni senza modifiche
* Su [Amazon Bedrock](/docs/it/amazon-bedrock), vedi [Streaming errors behind a gateway or proxy](/docs/it/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy) per i requisiti di intestazione e corpo

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock streaming response has an unexpected content-type
</h3>

Un gateway o proxy tra Claude Code e [Amazon Bedrock](/docs/it/amazon-bedrock) sta trasformando il corpo della risposta in streaming o la sua intestazione `Content-Type`. Amazon Bedrock trasmette le risposte come `application/vnd.amazon.eventstream`. Piuttosto che decodificare un corpo che non può leggere, Claude Code rifiuta una risposta in streaming riuscita che segnala un content-type diverso. Claude Code non ritenta la richiesta.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Prima della v2.1.208, la stessa configurazione errata è emersa come `API Error: Truncated event message received` dopo che l'intera risposta era stata memorizzata nel buffer.

**Cosa fare:**

* Configura il gateway per passare il corpo della risposta `InvokeModelWithResponseStream` e la sua intestazione `Content-Type` senza modifiche. Un intermediario che ri-emette lo stream come server-sent events è una causa comune.
* Impostare [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/it/env-vars) nasconde questo errore, ma Claude Code non decodifica un corpo binario sotto un'intestazione riscritta, quindi quelle richieste ricadono in un percorso più lento non in streaming. Vedi [Streaming errors behind a gateway or proxy](/docs/it/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  SSL certificate errors
</h3>

Un proxy o appliance di sicurezza sulla tua rete sta intercettando il traffico TLS con il suo certificato, e Claude Code non lo ritiene attendibile.

```text theme={null}
Unable to connect to API: SSL certificate verification failed (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
Unable to connect to API: Self-signed certificate detected (SELF_SIGNED_CERT_IN_CHAIN). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
```

Prima della v2.1.273, entrambi i messaggi terminavano a `Check your proxy or corporate SSL certificates`, senza il codice OpenSSL o il suggerimento `NODE_EXTRA_CA_CERTS`.

A partire dalla v2.1.199, un guasto di convalida del certificato non viene ritentato, quindi questo errore appare al primo tentativo invece che dopo il [retry budget](#automatic-retries) completo. Le versioni precedenti spendevano alcuni minuti ritentando prima di mostrarlo. Le condizioni TLS transitorie, come un timeout di handshake, continuano a ritentare.

Durante `/login` e il controllo di connettività all'avvio, lo stesso guasto produce un messaggio diverso:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

Su [Amazon Bedrock](/docs/it/amazon-bedrock), le richieste che Claude Code stesso invia ad AWS, come le chiamate di credenziale di ruolo STS e SSO, la scoperta del modello, e i controlli della procedura guidata di configurazione, dipendono dalla stessa configurazione del certificato. Vedi [Certificate errors behind a TLS-inspecting proxy](/docs/it/amazon-bedrock#certificate-errors-behind-a-tls-inspecting-proxy).

**Cosa fare:**

* Esporta il bundle CA della tua organizzazione e punta Claude Code ad esso con `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Vedi [Network configuration](/docs/it/network-config#custom-ca-certificates) per le istruzioni di configurazione complete
* Non impostare `NODE_TLS_REJECT_UNAUTHORIZED=0`, che disabilita completamente la convalida del certificato

<h3 id="host-not-allowed-in-a-cloud-session">
  Host not allowed in a cloud session
</h3>

Una richiesta HTTP in uscita da una sessione cloud o routine è stata bloccata dalla politica di rete dell'ambiente.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Potresti anche vedere un certificato TLS che non corrisponde al certificato reale della destinazione. Le sessioni cloud instradano il traffico in uscita attraverso un proxy che applica la politica di rete, quindi un certificato non corrispondente significa che il proxy ha terminato la connessione, non la destinazione.

Questo non è un problema di rete lato client. Le sessioni cloud e [routines](/docs/it/routines) vengono eseguite all'interno di una VM sandbox la cui rete di traffico in uscita attraverso la rete della sessione è filtrata alla [allowlist dell'ambiente cloud](/docs/it/cloud-environments); le [operazioni GitHub](/docs/it/cloud-environments#github-proxy) e il traffico del connettore MCP utilizzano canali separati, motivo per cui possono continuare a funzionare mentre altri host sono bloccati. L'ambiente **Default** utilizza accesso **Trusted**, che consente la [allowlist predefinita](/docs/it/cloud-environments#default-allowed-domains) di registri di pacchetti, API di provider cloud, registri di container, e domini di sviluppo comuni e blocca altri domini su quel percorso.

**Cosa fare:**

Questi passaggi cambiano uno dei tuoi ambienti. Un [organization-shared environment](/docs/it/cloud-environments#organization-shared-environments) si apre in sola lettura nel selettore, quindi chiedi a un Owner di cambiare il suo accesso di rete dalla pagina **Cloud environments** in [admin settings](https://claude.ai/admin-settings).

* Apri la routine per la modifica, o avvia una sessione cloud. Seleziona l'icona cloud che mostra il nome del tuo ambiente, come **Default**, per aprire il selettore. Passa il mouse sopra il tuo ambiente e fai clic sull'icona delle impostazioni.
* Nella finestra di dialogo **Update cloud environment**, cambia **Network access** da **Trusted** a **Custom**, quindi aggiungi il dominio bloccato a **Allowed domains**. Inserisci un dominio per riga. Seleziona **Also include default list of common package managers** per mantenere la [allowlist predefinita](/docs/it/cloud-environments#default-allowed-domains) insieme ai tuoi domini personalizzati. Seleziona **Full** invece se desideri accesso senza restrizioni.
* Fai clic su **Save changes**. La prossima esecuzione utilizza l'allowlist aggiornata.

Vedi [Network access](/docs/it/cloud-environments#network-access) per i livelli di accesso e l'allowlist predefinita. Le sessioni CLI locali non sono interessate da questa politica.

<h3 id="the-proxy-refused-the-connection">
  The proxy refused the connection
</h3>

Vedi questo messaggio quando Claude legge un [artifact](/docs/it/artifacts) attraverso il proxy che hai impostato in `HTTPS_PROXY` o una [proxy variable](/docs/it/network-config#environment-variables) correlata. Il contenuto dell'artifact proviene da `*.frame.claudeusercontent.com`, quindi Claude Code invia prima al proxy una richiesta `CONNECT` chiedendogli di aprire un tunnel a quell'host. Quando il proxy rifiuta, nulla raggiunge l'host, e il messaggio porta lo stato HTTP del proxy:

```text theme={null}
artifact content fetch failed (proxy refused the connection: HTTP 407)
artifact content fetch failed (proxy refused the connection: HTTP 403)
the proxy refused the connection to the artifact's content host (HTTP 502)
```

Lo stato è la risposta del proxy al `CONNECT`. L'host non ha mai risposto, quindi ogni stato punta a una correzione diversa:

* `HTTP 407`: il proxy richiede credenziali che non ha ricevuto. Mettile nell'URL del proxy, come mostra [Basic authentication](/docs/it/network-config#basic-authentication).
* `HTTP 403`: il proxy rifiuta di fare tunnel a `*.frame.claudeusercontent.com`. Chiedi a chiunque gestisca il proxy di consentire quell'host, che [Network access requirements](/docs/it/network-config#network-access-requirements) elenca.
* Qualsiasi altro stato, come `HTTP 502`: il proxy non ha aperto il tunnel per suo motivo, come il mancato raggiungimento dell'host. Cerca lo stato nei log del proxy.
* `unreadable reply` al posto di uno stato: qualunque cosa sia all'indirizzo del proxy non ha risposto con una riga di stato HTTP. Controlla che l'indirizzo sia un proxy HTTP.

**Cosa fare:**

* Controlla l'indirizzo e le credenziali nella variabile proxy, come descrive [Proxy configuration](/docs/it/network-config#proxy-configuration), quindi esegui `curl -x http://proxy.example.com:8080 -I https://api.anthropic.com` dalla shell in cui avvii Claude Code, usando il tuo URL proxy. Su Windows PowerShell, esegui `curl.exe`. Se questa sonda fallisce allo stesso modo, correggi prima la configurazione del proxy. Se ha successo, il rifiuto è specifico dell'host dell'artifact.
* Se la tua rete consente a Claude Code di raggiungere l'host dell'artifact direttamente, aggiungi `.frame.claudeusercontent.com` a [`NO_PROXY`](/docs/it/network-config#environment-variables). Mantieni la voce stretta: una voce `.claudeusercontent.com` più ampia bypassa anche il proxy per `bridge.claudeusercontent.com`, che le organizzazioni con [IP allowlisting](/docs/it/network-config#organization-ip-allowlists-and-proxy-egress) devono mantenere sul proxy.

Prima della v2.1.238, Claude Code segnalava un tunnel rifiutato come un errore di rete generico.

<h3 id="the-cloud-environments-service-returned-an-empty-or-unexpected-response">
  The cloud environments service returned an empty or unexpected response
</h3>

Claude Code richiede il tuo elenco di [cloud environments](/docs/it/cloud-environments) in diversi punti, come quando crei una sessione cloud dalla CLI o esegui [`/remote-env`](/docs/it/cloud-environments#select-an-environment-from-the-cli). Quando non riesce a leggere la risposta del server, mostra uno di questi messaggi:

```text theme={null}
The cloud environments service returned an empty response (HTTP 200 with no body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 with a non-JSON body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 without a usable environments list). This is usually temporary — try again in a moment.
```

Il server ha accettato la richiesta ma ha risposto con un corpo che non è l'elenco degli ambienti: vuoto, non JSON, o JSON senza l'elenco. Questo di solito accompagna un'interruzione lato servizio e si risolve da solo. A seconda della superficie che ha richiesto l'elenco, Claude Code può aggiungere un prefisso, come `couldn't list environments:` nella finestra di dialogo `/remote-env`.

**Cosa fare:**

* Ritenta l'azione. Claude Code richiede di nuovo l'elenco ogni volta
* Se il messaggio continua ad apparire, controlla [status.claude.com](https://status.claude.com) per gli incidenti attivi

Prima della v2.1.236, Claude Code mostrava un TypeError JavaScript grezzo invece di questi messaggi.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Couldn't reconnect to your Remote Control session
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

La ripresa con `claude --resume` o `claude --continue` si riconnette alla sessione [Remote Control](/docs/it/remote-control) registrata in quella conversazione. Questo messaggio significa che la riconnessione è fallita per un motivo che potrebbe essere temporaneo, come un'interruzione di rete o un errore del server, quindi Claude Code non può confermare se la sessione remota esiste ancora. La tua sessione locale continua a funzionare senza Remote Control.

**Cosa fare:**

* Esegui `/remote-control` per ritentare la connessione
* Avvia una nuova sessione con `claude --remote-control` per creare una nuova sessione Remote Control
* Per altri messaggi di avvio di Remote Control, vedi [Troubleshoot Remote Control](/docs/it/remote-control#troubleshooting)

Se il server segnala invece che la sessione precedente è scomparsa, non vedi questo messaggio. Claude Code avvia una nuova sessione al suo posto o mostra [`Previous session is unavailable — run /remote-control to start a new one`](/docs/it/remote-control#previous-session-is-unavailable), a seconda del [record di riconnessione della conversazione](/docs/it/remote-control#resume-outcomes). Dalla v2.1.227 alla v2.1.231, Claude Code mostrava un messaggio che inizia con `Remote Control could not resume the previous session under the current login` invece, e le [versioni precedenti si comportavano diversamente di nuovo](/docs/it/remote-control#reconnect-history).

<h3 id="sessions-ended-while-this-machine-was-offline">
  Sessions ended while this machine was offline
</h3>

Claude Code mostra questo messaggio nel terminale che esegue [`claude remote-control`](/docs/it/remote-control#start-a-remote-control-session) dopo che la tua macchina è stata offline abbastanza a lungo che il server ha pulito l'ambiente Remote Control che la tua macchina stava servendo. Le sessioni in quell'ambiente sono terminate e non puoi riprendere. Il conteggio è il numero di sessioni che sono terminate.

```text theme={null}
2 sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.
```

**Cosa fare:**

* Quando Claude Code elenca i worktrees mantenuti sotto questo messaggio, raccogli qualsiasi lavoro non committato da loro
* Esegui `claude remote-control` per avviare un ambiente nuovo

<h3 id="couldnt-share-the-transcript">
  Couldn't share the transcript
</h3>

Dopo che accetti di condividere la trascrizione della tua sessione da un prompt di sondaggio, come il [session quality survey](/docs/it/data-usage#session-quality-surveys), Claude Code la carica su Anthropic, o salva un archivio locale invece su provider di terze parti, su sessioni [Claude apps gateway](/docs/it/claude-apps-gateway), e quando nessuna credenziale Anthropic è disponibile. Questo messaggio significa che la condivisione non è stata completata.

```text theme={null}
Couldn't share the transcript.
```

Il caricamento deve rientrare in un limite di 8 MiB. Su una sessione lunga, Claude Code progressivamente elimina parti della condivisione, le impostazioni del modello dell'ultima richiesta per prime, quindi la conversazione strutturata e le trascrizioni dei subagent, e mostra questo messaggio solo quando nessuna versione ridotta può essere inviata o un errore di rete o server interrompe il caricamento. Quando Claude Code salva un archivio locale invece, il messaggio significa che non poteva scrivere l'archivio.

**Cosa fare:**

* Esegui `/feedback` per inviare la trascrizione con una descrizione di ciò che è accaduto. Vedi [Report an error](#report-an-error) se `/feedback` non è disponibile nel tuo ambiente
* Se anche altre richieste stanno fallendo, controlla la tua connessione di rete e vedi [Unable to connect to API](#unable-to-connect-to-api)

<h2 id="request-errors">
  Errori di richiesta
</h2>

Questi errori riguardano il contenuto della Vostra richiesta. La maggior parte proviene dall'API dopo che ha rifiutato la richiesta; alcuni sono prodotti localmente da Claude Code prima che venga inviata qualsiasi richiesta.

<h3 id="prompt-is-too-long">
  Prompt troppo lungo
</h3>

La conversazione più i file allegati superano la finestra di contesto del modello.

```text theme={null}
Prompt is too long
```

In una sessione interattiva, Claude Code mostra questo errore come:

```text theme={null}
Context limit reached · /compact or /clear to continue
```

La riga nomina solo `/clear` quando [`DISABLE_COMPACT`](/docs/it/env-vars) è impostato. Le forme più lunghe dell'errore, come la forma di compattazione non riuscita di seguito, mantengono la dicitura `Prompt is too long ·`. Nell'output `-p` e nella trascrizione, il testo rimane `Prompt is too long`.

Quando avete disattivato la compattazione automatica nelle Vostre [impostazioni utente](/docs/it/settings-reference#autocompactenabled), la riga dice anche:

```text theme={null}
Context limit reached · /compact or /clear to continue · auto-compact is off · /config to turn it on
```

L'interruttore **Auto-compact** in `/config` scrive `autoCompactEnabled` nelle impostazioni utente. L'hint appare solo quando una modifica `/config` avrebbe effetto. Ad esempio, non appare quando [`DISABLE_AUTO_COMPACT`](/docs/it/env-vars) o [`DISABLE_COMPACT`](/docs/it/env-vars) ha disattivato la compattazione automatica. Non appare nemmeno quando un ambito di precedenza superiore, come le impostazioni di progetto o gestite, ha impostato `autoCompactEnabled` su `false`. Prima della v2.1.235, la riga non conteneva alcun hint di compattazione automatica.

Amazon Bedrock segnala questa condizione come `Input is too long for requested model.`, che Claude Code gestisce allo stesso modo. Prima della v2.1.217, Claude Code non riconosceva la dicitura di Bedrock, quindi la compattazione automatica non si attivava mai e `/compact` falliva con lo stesso errore.

Un [gateway di app Claude](/docs/it/claude-apps-gateway-config#upstream-error-messages) segnala questa condizione come `capability_rejected: prompt_too_long` quando un upstream cloud rifiuta la richiesta nella forma di errore propria del provider. Claude Code tratta il token come `Prompt is too long`. Prima della v2.1.228, Claude Code non riconosceva il token, quindi la compattazione automatica non si attivava.

Quando la compattazione automatica è stata eseguita su questo turno e ha fallito su un errore sottostante, come un modello non disponibile o un errore di autenticazione, il messaggio nomina quell'errore dopo un separatore:

```text theme={null}
Prompt is too long · automatic compaction failed: <the underlying error>
```

Risolvete prima l'errore nominato; `/compact` fallisce sullo stesso errore finché non lo fate. Prima della v2.1.229, una compattazione automatica non riuscita mostrava `Prompt is too long` senza la causa.

Quando la compattazione automatica viene eseguita su questo errore, normalmente riassume i Vostri scambi più vecchi e mantiene i più recenti. Come ultima risorsa, Claude Code riassume diversamente:

* Quando non può riassumere alcuno scambio completo, Claude Code mantiene il Vostro prompt più recente parola per parola e riassume tutto prima di esso.
* In quel caso, quando la conversazione non termina con il Vostro prompt, Claude Code riassume l'intera conversazione.

Claude Code salta questo recupero quando il contenuto che porterebbe avanti non contiene alcuna risposta del modello e meno di circa 1.000 token del Vostro testo, come un breve ritentativo inviato dopo un incollamento di grandi dimensioni. Eseguite `/clear` per ricominciare da capo. Prima della v2.1.269, la compattazione falliva ogni volta che non poteva riassumere uno scambio completo, quindi una sessione in quello stato colpiva questo errore di nuovo ad ogni turno.

Una conversazione a singolo scambio non ha turni precedenti da riassumere. Quando la compattazione automatica si sarebbe eseguita su uno, Claude Code salta il tentativo e spiega cosa riempie la richiesta. Quando l'API non segnala i conteggi dei token nel suo errore, il messaggio recita:

```text theme={null}
Prompt is too long · this conversation is a single exchange and cannot be compacted — the request size comes mostly from system prompt, tool definitions, or attachments.
```

Quando l'API segnala i conteggi dei token nel suo errore, Claude Code li confronta con la sua stima della dimensione della conversazione per dire quale è la maggior parte della richiesta: il contenuto della conversazione stessa, o il prompt di sistema, le definizioni degli strumenti e il contenuto degli allegati che Claude Code invia con essa. Quando il contenuto della conversazione è la maggior parte della richiesta, il messaggio recita:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) and this conversation's own content is most of it. A single-exchange conversation cannot be compacted; start with less content (smaller files or pasted text).
```

Quando la maggior parte della richiesta è al di fuori della conversazione, il messaggio recita:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) but this conversation is only ~<conversation tokens> tokens — the rest is system prompt, tool definitions, and attachment content. A single-exchange conversation cannot be compacted; reduce attached files/tools or start with less context.
```

Prima della v2.1.162, Claude Code tentava comunque la compattazione e mostrava il semplice `Prompt is too long` quando falliva.

**Cosa fare:**

* Eseguite `/compact` per riassumere i turni precedenti e liberare spazio, oppure `/clear` per ricominciare da capo. Se `/compact` risponde `Not enough messages to compact.`, la conversazione è un singolo scambio senza nulla prima da riassumere, quindi lo spazio è occupato da quel singolo prompt e da ciò che Claude Code invia con ogni richiesta: eseguite `/clear` e rinviate con meno testo incollato o allegati più piccoli, oppure riducete le definizioni degli strumenti e i file di memoria usando i passaggi di seguito
* Eseguite `/context` per vedere una suddivisione di ciò che consuma la finestra: prompt di sistema, strumenti, file di memoria e messaggi
* Disabilitate i server MCP che non state utilizzando con `/mcp disable <name>` per rimuovere le loro definizioni di strumenti dal contesto
* Tagliate i file di memoria `CLAUDE.md` di grandi dimensioni, oppure spostate le istruzioni in [regole con ambito di percorso](/docs/it/memory#path-specific-rules) che si caricano solo quando rilevanti
* I subagent ereditano ogni definizione di strumento MCP dalla sessione padre, che può riempire la loro finestra di contesto prima del primo turno. Disabilitate i server MCP che non state utilizzando prima di generare subagent.
* La compattazione automatica è attiva per impostazione predefinita e normalmente previene questo errore. Se l'avete disattivata in `/config` o con [`DISABLE_AUTO_COMPACT`](/docs/it/env-vars), riattivatela. Se la mantenete disattivata, eseguite `/compact` voi stessi prima che la finestra si riempia.

Vedete [Explore the context window](/docs/it/context-window) per una visualizzazione interattiva di come il contesto si riempie.

<h3 id="context-exceeds-the-token-limit">
  Il contesto supera il limite di token
</h3>

`/context` mostra questo avviso in cima al suo output quando la conversazione ha superato la finestra di contesto del modello. Le richieste falliscono con [`Prompt is too long`](#prompt-is-too-long) finché non liberate spazio. Una sessione interattiva mostra quell'errore come la riga `Context limit reached`.

```text theme={null}
Context exceeds the 200k-token limit by 94k tokens — run /compact or /clear to continue.
```

Quando il limite che avete superato è una finestra di compattazione, come il limite di 200K su modelli con contesto 1M, l'avviso recita diversamente. Una finestra di compattazione può stare al di sotto della finestra di contesto del modello, quindi le richieste oltre di essa possono ancora avere successo.

```text theme={null}
Context is 94k tokens past the 200k-token compaction window — run /compact to reduce usage.
```

Entrambe le forme nominano `/clear` invece di `/compact` quando avete impostato [`DISABLE_COMPACT`](/docs/it/env-vars).

**Cosa fare:**

* In una conversazione multi-turno, eseguite `/compact` per riassumere i turni precedenti e liberare spazio. Per ricominciare da capo, eseguite `/clear`
* Per altri modi di ridurre l'utilizzo, vedete [Prompt is too long](#prompt-is-too-long)

Prima della v2.1.216, `/context` mostrava l'utilizzo sopra il 100% senza una riga di avviso che spiegasse cosa significava o come recuperare.

<h3 id="error-during-compaction-conversation-too-long">
  Errore durante la compattazione: Conversazione troppo lunga
</h3>

`/compact` stesso ha fallito perché non c'è abbastanza contesto libero per contenere il riassunto che produce.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Questo può accadere quando la finestra è già piena nel momento in cui la compattazione automatica si attiva, oppure quando eseguite `/compact` dopo aver visto [`Prompt is too long`](#prompt-is-too-long). In una sessione interattiva, quell'errore è la riga `Context limit reached`.

**Cosa fare:**

* Premete Esc due volte per aprire l'elenco dei messaggi e tornare indietro di diversi turni. Questo elimina i messaggi più recenti dal contesto. Quindi eseguite `/compact` di nuovo.
* Se tornare indietro non libera abbastanza spazio, eseguite `/clear` per avviare una sessione nuova. La Vostra conversazione precedente è preservata e può essere riaperta con `/resume`.

Questo messaggio e altri fallimenti di `/compact` vengono visualizzati nello stile di errore. Prima della v2.1.216, venivano renderizzati nello stesso stile attenuato dell'output di comando riuscito, quindi potevate leggere una compattazione non riuscita come un successo.

<h3 id="request-too-large">
  Richiesta troppo grande
</h3>

Il corpo della richiesta grezza ha superato il limite di 32MB dell'API prima della tokenizzazione, solitamente a causa di contenuto incollato di grandi dimensioni, risultati di strumenti o allegati. Questo limite è separato dalla [finestra di contesto](#prompt-is-too-long).

```text theme={null}
Request too large (max 32MB). Accumulated images and attachments in the conversation pushed the request over the limit. Run /compact, or double press esc to go back and remove attachments.
```

Quando la richiesta è andata direttamente all'API Claude e l'API stessa l'ha rifiutata, Claude Code misura la conversazione e formula il messaggio in base al fatto che il recupero possa funzionare. Attraverso un proxy, gateway o provider cloud ottenete il messaggio generale. Le forme misurate:

* `Request too large (max 32MB; 20.1MB of about 33.4MB is images or documents).`: le immagini o i documenti hanno spinto la richiesta oltre il limite. Claude Code ritenta con essi rimossi.
* `Request too large for the API's 32MB request limit`: i messaggi da soli superano il limite, quindi il messaggio dice `compacting cannot make it fit` e Claude Code non ritenta. In [modalità non interattiva](/docs/it/headless), il messaggio vi dice di ridurre l'input o avviare una nuova sessione.

Prima della v2.1.212, le conversazioni con abbastanza immagini accumulate fallivano ad ogni turno con `Request too large (max 32MB). Double press esc to go back and try with a smaller file.` Prima della v2.1.229, Claude Code mostrava il consiglio di allegato per ogni rifiuto, anche quando la compattazione non poteva aiutare.

**Cosa fare:**

* Se il messaggio dice `compacting cannot make it fit`, premete Esc due volte per tornare indietro oltre il turno che ha aggiunto il contenuto di grandi dimensioni, oppure eseguite `/clear` per ricominciare da capo
* Altrimenti, eseguite `/compact`, che elimina le immagini e gli allegati accumulati
* Fate riferimento ai file di grandi dimensioni per percorso invece di incollarne i contenuti, in modo che Claude possa leggerli in blocchi
* Per le immagini, vedete [Image was too large](#image-was-too-large) di seguito

<h3 id="image-was-too-large">
  L'immagine era troppo grande
</h3>

Un'immagine incollata o allegata supera i limiti di dimensione o dimensione dell'API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code sostituisce l'immagine non elaborabile con un segnaposto di testo e ritenta, quindi i messaggi successivi hanno successo. Nelle versioni precedenti alla 2.1.142, un'immagine incollata poteva rimanere nella conversazione e ripetere lo stesso errore ad ogni messaggio successivo. Per recuperare su quelle versioni, premete Esc due volte e tornate indietro oltre il turno in cui l'immagine è stata aggiunta.

**Cosa fare:**

* Ridimensionate l'immagine prima di incollarla. L'API accetta immagini fino a 8000 pixel sul lato più lungo per una singola immagine, o 2000 pixel quando molte immagini sono nel contesto.
* Fate uno screenshot più stretto della regione rilevante invece dello schermo intero

<h3 id="unable-to-resize-image">
  Impossibile ridimensionare l'immagine
</h3>

Claude Code non ha potuto ridimensionare un'immagine allegata prima di inviarla all'API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
Unable to resize image — it is a CMYK JPEG, which Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Re-save it as an RGB PNG or JPEG and try again.
Unable to resize image — it is an animated WebP whose first frame Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Save its first frame as a PNG or JPEG and try again.
Unable to resize image — its pixels could not be decoded (the file may be damaged, or use an encoding Claude Code cannot read), and it is over the … API limit (… raw, … base64), so it cannot be sent. Re-save it as a PNG or JPEG and try again.
```

Claude Code normalmente ridimensiona automaticamente le immagini di grandi dimensioni. Questi errori significano che l'immagine non poteva essere decodificata o ridimensionata per rientrare nei limiti dell'API.

**Cosa fare:**

* Se il messaggio vi chiede di convertire l'immagine, convertitela in PNG, JPEG, GIF o WebP e allegatela di nuovo. Claude Code può verificare le dimensioni per questi formati dall'intestazione del file, senza decodificare l'immagine.
* Se il messaggio segnala un limite di dimensione o dimensione, ridimensionate o ricomprimete l'immagine al di sotto di quel limite prima di allegarla.
* Se il messaggio nomina una causa, come un JPEG CMYK, un WebP animato o un file possibilmente danneggiato, risalvate l'immagine nel formato che il messaggio suggerisce e allegatela di nuovo.

<h3 id="pdf-errors">
  Errori PDF
</h3>

Il PDF che avete allegato non poteva essere elaborato. I messaggi sono mostrati qui nella loro forma non interattiva; in una sessione interattiva invece vi chiedono di premere esc due volte e riprovare.

```text theme={null}
PDF too large (max 100 pages, 20MB). Try reading the file a different way (e.g., extract text with pdftotext).
PDF is password protected. Try using a CLI tool to extract or convert the PDF.
The PDF file was not valid. Try converting it to text first (e.g., pdftotext).
```

**Cosa fare:**

* Per i PDF di grandi dimensioni, chiedete a Claude di leggere un intervallo di pagine con lo strumento Read invece di allegare l'intero file, oppure estraete il testo con uno strumento come `pdftotext` e fate riferimento al file di output per percorso
* Per i PDF protetti o non validi, rimuovete la password o riesportate il file dall'applicazione sorgente, quindi riprovate

<h3 id="extra-inputs-are-not-permitted">
  Gli input extra non sono consentiti
</h3>

Un proxy o gateway LLM tra Claude Code e l'API ha rimosso l'intestazione della richiesta `anthropic-beta`, quindi l'API ha rifiutato i campi che dipendono da essa.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code invia campi solo beta come `context_management` e `effort` insieme a un'intestazione `anthropic-beta` che li abilita. Quando un gateway inoltra il corpo ma elimina l'intestazione, l'API vede campi che non riconosce.

**Cosa fare:**

* Configurate il Vostro gateway per inoltrare l'intestazione `anthropic-beta`. Vedete [feature pass-through](/docs/it/llm-gateway-protocol#feature-pass-through) per ciò che i gateway devono inoltrare.
* Come fallback, impostate [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/it/env-vars) prima di avviare. [Disable pre-release capabilities](/docs/it/llm-gateway-protocol#disable-pre-release-capabilities) copre l'ambito esatto.

<h3 id="tool-input-schema-is-invalid">
  Lo schema di input dello strumento non è valido
</h3>

Uno strumento nella richiesta ha dichiarato un `input_schema` che non supera la convalida JSON Schema dell'API, quindi l'API ha rifiutato l'intera richiesta. Il numero dopo `tools.` è la posizione dello strumento che fallisce nell'elenco degli strumenti della richiesta, non un nome che potete cercare.

```text theme={null}
API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid
API Error: 400 ... tools.N.custom.input_schema.properties: Property keys should match pattern '^[a-zA-Z0-9_.-]{1,64}$'
```

La prima forma significa che lo schema non è uno schema JSON valido draft 2020-12. La seconda significa che un nome di proprietà di livello superiore non corrisponde al pattern che il messaggio cita.

Claude Code [esclude gli strumenti MCP il cui schema di input fallirebbe questa convalida](/docs/it/mcp#tools-with-invalid-input-schemas) quando carica gli strumenti di un server, quindi le richieste normalmente non ne includono mai uno.

Su un [deployment dove il recupero dei flag è disattivato](/docs/it/env-vars#features-that-need-feature-flag-fetching), o su una macchina i cui flag non sono mai arrivati, Claude Code registra nel log del server quale strumento verrebbe rifiutato ma lo invia comunque, quindi questo errore può ancora verificarsi.

L'errore può anche verificarsi per uno strumento il cui schema dichiara un dialetto JSON Schema diverso da draft 2020-12 in `$schema`. Claude Code non controlla questi schemi rispetto al meta-schema JSON Schema, anche se il controllo del nome della proprietà di livello superiore si applica comunque.

Prima della v2.1.216, nessun deployment eseguiva i controlli di esclusione.

**Cosa fare:**

* Se la Vostra versione di Claude Code è precedente alla v2.1.216, eseguite `claude update`.
* Rimuovete o [disabilitate](/docs/it/mcp#disable-a-server-without-removing-it) il server MCP che dichiara lo schema non valido. L'errore nomina lo strumento solo per posizione. Sulla v2.1.216 o successiva, controllate il log di ogni server per una riga che nomina uno strumento il cui schema di input verrebbe rifiutato. Se nessun log ne nomina uno, disabilitate i server uno alla volta.
* Se mantenete il server, correggete l'`input_schema` dello strumento. Lo schema deve essere uno schema JSON valido, e i nomi delle proprietà di livello superiore devono essere da 1 a 64 caratteri e usare solo lettere ASCII e cifre, `_`, `.` e `-`. Vedete [Tools with invalid input schemas](/docs/it/mcp#tools-with-invalid-input-schemas).

<h3 id="theres-an-issue-with-the-selected-model">
  C'è un problema con il modello selezionato
</h3>

Il nome del modello configurato non è stato riconosciuto o il Vostro account non ha accesso ad esso. A partire dalla v2.1.160 l'hint finale, mostrato qui nella sua forma interattiva, varia per superficie.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Cosa fare:**

* **CLI interattiva**: eseguite `/model` per scegliere dai modelli disponibili per il Vostro account.
* **Modalità non interattiva (`-p`)**: passate `--model` con un alias o ID valido, oppure impostate [`ANTHROPIC_MODEL`](/docs/it/env-vars). Il testo di errore mostra `Run --model` su questa superficie.
* **Agent SDK**: il testo di errore omette l'hint perché il modello è impostato a livello di programmazione. Impostate [`model` su `Options`](/docs/it/agent-sdk/typescript#options) in TypeScript o [`ClaudeAgentOptions(model=...)`](/docs/it/agent-sdk/python#claudeagentoptions) in Python, e gestite l'errore strutturato `model_not_found` per mostrare il Vostro proprio ritentativo o selettore di modello.
* Usate un alias come `sonnet` o `opus` invece di un ID completo con versione. Gli alias si risolvono in un valore predefinito mantenuto in modo che non diventino obsoleti. Vedete [Model configuration](/docs/it/model-config).
* Se il modello sbagliato continua a tornare nella CLI, un ID obsoleto è impostato da qualche parte. Controllate i posti in cui potete impostare un modello in [ordine di priorità](/docs/it/model-config#setting-your-model) e rimuovete il valore obsoleto.
* Un modello appena lanciato può essere disponibile sull'API Anthropic prima che Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry lo offra. Se avete fissato un nuovo ID modello su uno di questi provider e vedete questo errore, controllate il catalogo dei modelli del Vostro provider per la disponibilità nella Vostra regione, e mantenete la versione precedente fissata finché quella nuova non appare lì.
* Claude Code segnala un login claude.ai scaduto come [Login expired](#login-expired), non come questo errore. Prima della v2.1.206, un login scaduto che non poteva più essere aggiornato falliva ad ogni modello con questo errore; eseguite `/login` se lo vedete su una versione più vecchia.
* Per i deployment di Google Cloud's Agent Platform, vedete [Google Cloud's Agent Platform troubleshooting](/docs/it/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Il modello non è un ID modello riconosciuto
</h3>

La stringa di modello che avete passato a un cambio di modello non è un alias di modello, un ID di modello che questa versione di Claude Code conosce, o un ID che inizia con `claude-`. Le cause solite sono un errore di battitura nell'ID, un nome visualizzato come `Sonnet 5` dove è previsto l'ID `claude-sonnet-5`, o un alias che solo le versioni più recenti di Claude Code riconoscono. Claude Code rifiuta il cambio immediatamente. Prima della v2.1.200, Claude Code salvava la stringa e falliva alla richiesta successiva con [There's an issue with the selected model](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

L'hint finale nomina l'alias o l'ID modello più simile. Quando nulla è abbastanza simile, recita `Run /model to see available models.` invece.

Claude Code produce questo errore localmente nel momento in cui il cambio è richiesto, prima che venga effettuata qualsiasi richiesta API. Si applica quando un modello è impostato attraverso il metodo [Agent SDK](/docs/it/agent-sdk/typescript) `setModel()`, da un'app come l'[app Desktop](/docs/it/desktop) che esegue la CLI di Claude Code per voi, o quando scegliete un modello da un dispositivo connesso attraverso [Remote Control](/docs/it/remote-control). Prima della v2.1.260, il controllo non copriva le scelte di Remote Control, quindi Claude Code applicava la scelta e la richiesta successiva falliva con [There's an issue with the selected model](#theres-an-issue-with-the-selected-model).

**Cosa fare:**

* Eseguite `/model` senza argomenti per aprire il selettore e scegliere dai modelli disponibili per il Vostro account, quindi passate l'alias o l'ID mostrato lì
* Se avete usato un alias che una versione più recente di Claude Code supporta, eseguite `claude update`. Un ID completo che inizia con `claude-` passa questo controllo locale anche quando il modello è più recente della Vostra versione di Claude Code. Il server può comunque richiedere una versione minima per quel modello; vedete [Claude Code does not support this model](#claude-code-does-not-support-this-model).
* Un modello salvato prima della v2.1.200 non viene riparato da questo controllo. Se un valore obsoleto continua a tornare, rimuovetelo dalle posizioni elencate sotto [Setting your model](/docs/it/model-config#setting-your-model).
* Il controllo viene eseguito solo sull'API Anthropic. Su qualsiasi altro provider o gateway, incluso un `ANTHROPIC_BASE_URL` personalizzato, il provider definisce i nomi dei modelli, quindi Claude Code accetta qualsiasi stringa e lo passa. Claude Code può comunque scrivere la [riga diagnostica di modello non riconosciuto](#unrecognized-model-id-on-a-request) al momento della richiesta, su ogni provider.

<h3 id="model-not-found">
  Modello non trovato
</h3>

Avete scelto un modello con `/model <name>` e Claude Code non ha potuto confermare che esista un modello con quel nome. Quando il nome non è un [alias di modello](/docs/it/model-config#model-aliases) o un'altra ortografia che Claude Code accetta localmente, `/model` lo verifica con una richiesta API minima, e questo errore è solitamente la risposta del Vostro endpoint API. Un nome che non può essere un ID modello affatto, come uno contenente spazi, riceve lo stesso messaggio.

```text theme={null}
Model 'claude-opus-9' not found
```

Su provider con ID modello specifici del provider, il messaggio può aggiungere un suggerimento `Try '...' instead` che nomina l'ID del Vostro provider per un modello di fallback.

**Cosa fare:**

* Eseguite `/model` senza argomenti e scegliete dai modelli disponibili per il Vostro account, oppure usate un [alias di modello](/docs/it/model-config#model-aliases) come `sonnet`, che si risolve in un valore predefinito mantenuto
* Se avete digitato un ID completo, controllarlo rispetto al catalogo dei modelli del Vostro provider. Un modello appena lanciato può essere disponibile sull'API Anthropic prima che il Vostro provider o regione lo offra.
* Prima della v2.1.265, `/model` rifiutava anche l'ortografia dell'alias `opusplan[1m]` con questo errore. Su quelle versioni, aggiornate Claude Code, oppure impostate il modello in [settings](/docs/it/model-config#setting-your-model) o con `--model` invece.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus non è disponibile con il piano Claude Pro
</h3>

Il Vostro piano di abbonamento attivo non include il modello che avete selezionato.

```text theme={null}
Claude Opus is not available with the Claude Pro plan. If you have updated your subscription plan recently, run /logout and /login for the plan to take effect.
```

**Cosa fare:**

* Eseguite `/model` e selezionate un modello che il Vostro piano include
* Se avete aggiornato il Vostro piano di recente e lo vedete ancora, eseguite `/logout` quindi `/login`. Il token memorizzato riflette il Vostro piano al momento in cui avete effettuato l'accesso, quindi l'aggiornamento sul web non ha effetto in una sessione esistente finché non vi autenticate di nuovo.
* Vedete [claude.com/pricing](https://claude.com/pricing) per quali modelli ogni piano include

<h3 id="claude-code-does-not-support-this-model">
  Claude Code non supporta questo modello
</h3>

L'API ha rifiutato la richiesta con un 400 perché la Vostra versione di Claude Code è inferiore a un minimo richiesto. O il modello che avete selezionato richiede una versione più recente, che il server controlla per modello, oppure la politica della Vostra organizzazione ne richiede una. Il 400 porta il codice di errore `claude_code_version_too_old`, e il messaggio dice quale minimo si applica.

```text theme={null}
API Error: 400 Claude Code 2.1.219 does not support this model; version 2.1.255 or newer is required. Run 'claude update', or update the Claude desktop app, then try again.
```

La dicitura della politica organizzativa recita:

```text theme={null}
API Error: 400 Claude Code 2.1.240 is older than the minimum version required by your organization's policy. Run 'claude update', or update the Claude desktop app, to continue.
```

**Cosa fare:**

* Eseguite `claude update`, oppure aggiornate l'app Claude desktop, quindi avviate una nuova sessione
* Per la dicitura per modello, potete continuare a lavorare nella sessione corrente passando a un altro modello con `/model`
* Per la dicitura della politica organizzativa, aggiornate prima di continuare

<h3 id="model-is-restricted-by-your-organizations-settings">
  Il modello è limitato dalle impostazioni della Vostra organizzazione
</h3>

L'amministratore della Vostra organizzazione ha disabilitato questo modello nella console di amministrazione claude.ai, oppure è escluso da un elenco di consentiti [`availableModels`](/docs/it/model-config#restrict-model-selection) nelle impostazioni gestite. Quando il modello limitato è stato impostato con `--model`, `ANTHROPIC_MODEL` o l'impostazione `model`, Claude Code sostituisce un modello consentito e continua. Digitare `/model <name>` per un modello limitato viene rifiutato con `Run /model to choose a different model.` e la sessione mantiene il suo modello corrente. L'avviso di sostituzione può anche apparire a metà sessione dopo che un amministratore disabilita il modello su cui una sessione è in esecuzione nella console di amministrazione claude.ai.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Un avviso con prefisso un nome di agente, skill o comando significa che la restrizione si è applicata al [modello richiesto di quel subagent](/docs/it/sub-agents#choose-a-model): il subagent viene eseguito sul modello sostituito e il modello della Vostra sessione rimane invariato. Prima della v2.1.223, Claude Code mostrava l'avviso solo per i subagent lanciati con lo strumento Agent.

Claude Code tratta un alias di famiglia di modelli, uno di `opus`, `sonnet`, `haiku` o `fable`, come una richiesta per quella famiglia piuttosto che per la sua versione più recente. Sull'API Anthropic e su [Claude Platform on AWS](/docs/it/claude-platform-on-aws), un alias di famiglia limitato si risolve nella versione più recente della famiglia che la Vostra organizzazione e l'elenco di consentiti `availableModels` consentono, e l'avviso di sostituzione nomina quella versione. Claude Code rifiuta `/model <alias>` solo quando ogni versione della famiglia è limitata. Prima della v2.1.205, un alias di famiglia veniva sostituito o rifiutato in base alla sua versione più recente sola, anche quando una versione più vecchia della stessa famiglia era consentita.

**Cosa fare:**

* Eseguite `/model` per scegliere dai modelli che la Vostra organizzazione consente. I modelli limitati sono nascosti dal selettore.
* Se il modello limitato è stato impostato in `--model`, `ANTHROPIC_MODEL`, il campo `model` di un file di impostazioni, o il frontmatter `model` di un [subagent](/docs/it/sub-agents#choose-a-model), skill o comando, rimuovete o aggiornate quel valore in modo che l'avviso non si ripeta
* Se avete bisogno di accesso al modello limitato, chiedete all'amministratore della Vostra organizzazione di abilitarlo. Vedete [Organization model restrictions](/docs/it/model-config#organization-model-restrictions).

<h3 id="model-switch-was-blocked-by-a-premodelswitch-hook">
  Il cambio di modello è stato bloccato da un hook PreModelSwitch
</h3>

Un [hook PreModelSwitch](/docs/it/hooks#premodelswitch) non ha approvato il cambio di modello che voi o un client avete richiesto, quindi la sessione mantiene il suo modello corrente. Quando il cambio è venuto da un host [Agent SDK](/docs/it/agent-sdk/overview) o [Remote Control](/docs/it/remote-control) piuttosto che da un comando che avete digitato, il messaggio recita `Model switch blocked by a PreModelSwitch hook` senza nominare il modello di destinazione.

```text theme={null}
Model switch to Opus 4.6 was blocked by a PreModelSwitch hook: Opus 4.6 is retired for this project. Use a newer model.
```

La ragione dopo i due punti dice cosa ha rifiutato il cambio:

* **Una ragione che un hook ha scritto**: un hook PreModelSwitch ha fornito quella ragione quando ha [negato il cambio o chiesto conferma](/docs/it/hooks#premodelswitch-decision-control). Affrontate ciò che chiede, oppure scegliete un modello che i Vostri hook consentono.
* **`PreModelSwitch hook <name> did not respond before its timeout`**: un hook che non risponde prima del suo [timeout](/docs/it/hooks#timeouts) blocca il cambio. Correggete il comando sospeso o aumentate il `timeout` di quell'hook, quindi cambiate di nuovo.
* **`confirmation required, and this session cannot ask`**: un hook ha risposto `ask` senza una ragione, e una richiesta di controllo non ha modo di mostrare il prompt di conferma. Un comando `/model` in un'esecuzione [`-p`](/docs/it/headless) segnala la stessa condizione con `(run /model interactively to confirm)` dopo la ragione. Effettuate il cambio da una sessione interattiva, oppure cambiate la decisione dell'hook per questo modello.
* **`so organization-managed PreModelSwitch hooks could not be checked`**: Claude Code non ha potuto dire quali hook PreModelSwitch i [plugin gestiti](/docs/it/settings-reference#enabledplugins) della Vostra organizzazione forniscono, ad esempio perché un plugin gestito non ha caricato. Uno di questi hook potrebbe bloccare il cambio, quindi Claude Code rifiuta piuttosto che applicare il cambio non controllato. L'inizio della ragione nomina cosa ha fallito. Claude Code ricontrolla ad ogni tentativo di cambio, quindi un fallimento che da allora si è chiarito smette di bloccare; se continua a fallire, eseguite `claude --debug` e cambiate di nuovo per catturare i dettagli, quindi correggete il plugin o chiedete al Vostro admin di correggerlo.
* **`a PreModelSwitch hook failed before answering`** o **`PreModelSwitch hooks were cancelled (the control stream closed) before answering`**: l'esecuzione dell'hook è terminata senza un verdetto, e Claude Code non lo tratta come approvazione. Eseguite `claude --debug` per vedere cosa ha fallito, quindi cambiate di nuovo.

Prima della v2.1.260, il rifiuto del plugin gestito recitava `plugin hooks could not be loaded, so PreModelSwitch hooks could not be checked; see the debug log`. Claude Code ha ritentato il caricamento del plugin una volta e poi ha rifiutato i cambi successivi nella sessione, anche quando la Vostra organizzazione non gestiva alcun plugin. Riavviate la sessione per eseguire il caricamento del plugin di nuovo su quelle versioni.

<h3 id="couldnt-save-it-as-your-default">
  Non è stato possibile salvarlo come predefinito
</h3>

Avete scelto un modello da salvare come predefinito, ad esempio con `/model <name>` o `Enter` nel selettore `/model`, e Claude Code non ha potuto scrivere la scelta nel Vostro file di impostazioni utente, `~/.claude/settings.json`. Il cambio stesso si è applicato, quindi la sessione corrente viene eseguita sul modello che avete scelto, ma il Vostro predefinito rimane invariato e la sessione successiva inizia sul valore precedente.

```text theme={null}
Set model to Fable 5.1 for this session only · couldn't save it as your default: ~/.claude/settings.json can't be written (EROFS)
```

La ragione dopo il percorso del file dice cosa ha fallito:

* **`can't be written (<code>)`**: la scrittura ha fallito con il codice di errore del sistema operativo tra parentesi, come `EROFS` quando il file, o il file a cui si collega, si trova su un filesystem che rifiuta le scritture. Rendete il file scrivibile e cambiate di nuovo. Se un altro strumento genera il file, impostate la chiave `model` in quello strumento; vedete [A change you made in Claude Code is lost in new sessions](/docs/it/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **`isn't valid JSON`**: il file su disco non analizza, e Claude Code lo lascia intatto piuttosto che sovrascrivere il contenuto che non può leggere di nuovo. Correggete l'errore di sintassi, quindi cambiate di nuovo; vedete [Fix a broken settings file](/docs/it/settings#fix-a-broken-settings-file).

Un avviso che termina `couldn't confirm it was saved as your default (~/.claude/settings.json is still being written)` significa che la scrittura non era terminata dopo tre secondi. Continua in background, quindi il predefinito potrebbe comunque essere salvato; controllate quale modello la Vostra sessione successiva inizia, oppure eseguite `/model <name>` di nuovo.

Prima della v2.1.265, l'avviso diceva che il modello era `saved as your default for new sessions` anche quando la scrittura falliva.

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled non è supportato per questo modello
</h3>

La Vostra versione di Claude Code è più vecchia del minimo per il modello selezionato. La CLI ha inviato una configurazione di thinking che il modello non accetta più.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Cosa fare:**

* Eseguite `claude update` e riavviate Claude Code. Opus 4.7 ha bisogno della v2.1.111 o successiva. Opus 4.8 ha bisogno della v2.1.154 o successiva. Sonnet 5 ha bisogno della v2.1.197 o successiva. Opus 5 ha bisogno della v2.1.219 o successiva. Opus 5.5 ha bisogno della v2.1.280 o successiva
* Se non potete aggiornare, eseguite `/model` e selezionate Opus 4.6 o Sonnet 4.6 invece
* Se lo incontrate nell'[Agent SDK](/docs/it/agent-sdk/overview), aggiornate il pacchetto SDK. Opus 4.8 ha bisogno di TypeScript SDK v0.3.154 o successiva e Python SDK v0.2.88 o successiva. Sonnet 5 ha bisogno di TypeScript SDK v0.3.197 o successiva. Opus 5 ha bisogno di TypeScript SDK v0.3.219 o successiva. Opus 5.5 ha bisogno di TypeScript SDK v0.3.280 o successiva

<h3 id="effort-isnt-available-with-thinking-turned-off">
  Effort non è disponibile con thinking disattivato
</h3>

Avete disattivato il [thinking esteso](/docs/it/model-config#extended-thinking) e avete eseguito a un [livello di effort](/docs/it/model-config#adjust-effort-level) superiore a `high`. Il modello non accetta quella combinazione, quindi l'API ha rifiutato la richiesta.

```text theme={null}
API Error: Effort 'xhigh' isn't available with thinking turned off on this model · run /effort high to continue, or turn thinking back on (unset MAX_THINKING_TOKENS=0)
```

**Cosa fare:**

* [Abbassate il livello di effort](/docs/it/model-config#set-the-effort-level) a `high` o inferiore.
* Riattivate il thinking, ad esempio annullando [`MAX_THINKING_TOKENS`](/docs/it/env-vars) o rimuovendo [`"alwaysThinkingEnabled": false`](/docs/it/settings-reference#alwaysthinkingenabled) dalle Vostre impostazioni.

Prima della v2.1.242, Claude Code mostrava il messaggio proprio dell'API: `API Error: 400 output_config.effort 'xhigh' is not supported when thinking is disabled on this model. Use effort 'high' or below, or enable thinking.` Prima della v2.1.251, Claude Code inviava la richiesta al livello di effort che avete impostato, quindi Opus 5 rifiutava ogni richiesta superiore a `high` con thinking disattivato. Claude Code ora invia effort `high` invece ai modelli che sa rifiutano la combinazione, come Opus 5, quindi sulla v2.1.251 o successiva questo errore vi raggiunge solo da un modello che Claude Code non sa rifiuta.

<h3 id="thinking-budget-exceeds-output-limit">
  Il budget di thinking supera il limite di output
</h3>

Il budget di thinking esteso configurato supera la lunghezza massima della risposta, quindi non c'è spazio rimasto per la risposta effettiva.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code regola automaticamente questi valori sull'API Anthropic. Solitamente vedete questo errore su Amazon Bedrock o Google Cloud's Agent Platform quando [`MAX_THINKING_TOKENS`](/docs/it/env-vars) è impostato più alto del limite di output del provider, oppure quando la modalità piano aumenta il budget di thinking.

**Cosa fare:**

* Abbassate `MAX_THINKING_TOKENS`, oppure aumentate [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/it/env-vars) sopra il budget di thinking
* Vedete [Extended thinking](/docs/it/model-config#extended-thinking) per come il budget interagisce con la lunghezza di output

<h3 id="tool-use-or-thinking-block-mismatch">
  Mancata corrispondenza di blocco di tool use o thinking
</h3>

La cronologia della conversazione ha raggiunto l'API in uno stato incoerente, solitamente dopo che una chiamata di strumento è stata interrotta o un turno è stato modificato a metà flusso.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 orphaned tool_result in conversation history. Run /rewind to recover the conversation.
API Error: 400 duplicate tool_use ID in conversation history. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Tutte le varianti significano la stessa cosa: la sequenza di blocchi `tool_use`, `tool_result` e `thinking` nella cronologia non corrisponde più a ciò che l'API si aspetta.

**Cosa fare:**

* Se state usando Opus 4.7 o Opus 4.8, eseguite `claude update` prima. Le versioni precedenti alla v2.1.156 possono attivare questo errore durante il normale uso di strumenti, e `/rewind` non lo cancella.
* Eseguite `/rewind`, oppure premete Esc due volte, per tornare indietro a un checkpoint prima del turno corrotto e continuare da lì. Vedete [Checkpointing](/docs/it/checkpointing) per come i checkpoint vengono creati e ripristinati.

<h3 id="unsupported-tool-content-removed">
  Contenuto di strumento non supportato rimosso
</h3>

Quando Claude Code si connette direttamente all'API Anthropic e carica o visualizza un'anteprima di una sessione salvata, rimuove il contenuto dello strumento che l'API Anthropic non accetta e lascia questa riga dove il contenuto rimosso si trovava tra due blocchi di thinking:

```text theme={null}
[Unsupported tool content removed]
```

Tale contenuto raggiunge un file di sessione quando qualcosa di diverso dall'API Anthropic risponde nel formato dell'API, tipicamente un proxy di terze parti impostato attraverso [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) che traduce le chiamate di strumento di un altro provider. Claude Code lo rimuove solo quando la sessione si connette direttamente all'API Anthropic, e carica la cronologia salvata come è quando la sessione viene eseguita attraverso un proxy o su un altro provider. Prima della v2.1.246, Claude Code inviava il tool use e il suo risultato di nuovo all'API, e ogni turno della sessione ripresa falliva con un errore 400 come `messages.1.content.0.server_tool_use.name: Input should be 'web_search', 'web_fetch', ...`.

**Cosa fare:**

* Nessuna azione necessaria quando vedete la riga segnaposto. La sessione continua senza il contenuto rimosso.
* Se ogni turno di una sessione ripresa fallisce con l'errore 400 invece, eseguite `claude update` e riprendete la sessione di nuovo. Le versioni precedenti alla v2.1.246 non rimuovono il contenuto.

<h3 id="role-system-must-precede-an-assistant-message">
  role 'system' deve precedere un messaggio 'assistant'
</h3>

L'API ha rifiutato la richiesta con un 400 perché un messaggio di sistema si trova in una posizione nella conversazione che non accetta:

```text theme={null}
API Error: 400 messages.6: role 'system' must precede an 'assistant' message or end the array; ...
```

Claude Code invia parte del suo testo di promemoria e allegato come messaggi di sistema all'interno della conversazione. Quando l'API rifiuta la posizione di uno, Claude Code ritenta la richiesta una volta con quel testo inviato come messaggi utente ordinari. Le diciture di posizionamento fratello dell'API, come `use the top-level 'system' parameter for the initial system prompt`, ricevono lo stesso recupero.

Quando l'errore appare, il messaggio di sistema rifiutato non è uno che Claude Code può rimuovere. Questo solitamente significa che un proxy o [gateway LLM](/docs/it/llm-gateway) tra Claude Code e l'API ha aggiunto un messaggio di sistema proprio o ha riordinato la conversazione.

**Cosa fare:**

* Eseguite `/clear` per avviare una conversazione nuova. Se l'errore ritorna anche lì, la causa è sul percorso della richiesta, non nella conversazione salvata.
* Se l'errore si ripete ad ogni turno dietro un proxy o gateway configurato attraverso [`ANTHROPIC_BASE_URL`](/docs/it/env-vars), connettetevi senza il proxy per confermare la fonte, e segnalate l'errore a chi lo gestisce

Prima della v2.1.280, Claude Code non riconosceva questa dicitura, quindi l'errore appariva anche quando il messaggio di sistema rifiutato era uno che Claude Code stesso inviava, e ogni turno successivo della conversazione falliva allo stesso modo.

<h3 id="invalid-encrypted-content-in-search-result-block">
  Contenuto crittografato non valido nel blocco search\_result
</h3>

L'API ha rifiutato la richiesta con un 400 perché la cronologia della conversazione contiene contenuto di ricerca web ospitato che non può decrittare. La dicitura nomina il campo che non può leggere:

```text theme={null}
API Error: 400 messages.21.content.0: Invalid `encrypted_content` in `search_result` block
API Error: 400 messages.21.content.3.citations.0: Invalid `encrypted_index` in `text` block
API Error: 400 Failed to decrypt web search result content
```

I risultati dallo [strumento di ricerca web](/docs/it/tools-reference#websearch-tool-behavior) ospitato dell'API portano campi crittografati che solo l'API può leggere. L'API rifiuta una richiesta che riproduce contenuto che non può decrittare, come contenuto prodotto per un'organizzazione diversa.

Lo strumento [WebSearch](/docs/it/tools-reference#websearch-tool-behavior) proprio di Claude Code registra i risultati di ricerca come testo semplice, quindi questi blocchi solitamente raggiungono una conversazione attraverso un proxy o [gateway LLM](/docs/it/llm-gateway) che ha eseguito la ricerca web ospitata stesso.

I blocchi rifiutati rimangono nella cronologia della conversazione, quindi ogni turno successivo e `/compact` falliscono allo stesso modo.

**Cosa fare:**

* Eseguite `/clear` o avviate una nuova sessione; la nuova conversazione non porta i blocchi rifiutati
* Se eseguite Claude Code dietro un proxy o gateway, segnalate l'errore a chi lo gestisce

<h3 id="usage-policy-refusal">
  Rifiuto della politica di utilizzo
</h3>

L'API ha rifiutato di rispondere perché il contenuto nella conversazione ha attivato un controllo della [Politica di utilizzo](https://www.anthropic.com/legal/aup). Il messaggio include un ID richiesta che potete citare al supporto se ritenete che il rifiuto sia scorretto.

```text theme={null}
API Error: Opus 4.6 can't help with this. Start a new session to continue.

Send feedback with /feedback or learn more: https://www.anthropic.com/legal/aup
```

Il messaggio nomina il modello che ha rifiutato, o `Claude` quando nessun modello è registrato.

Il controllo valuta l'intera conversazione, non solo il Vostro prompt più recente, quindi inviare un nuovo messaggio nella stessa sessione solitamente riattiva lo stesso rifiuto. Lo stesso si applica dopo aver uscito e riaperto la sessione con `--continue` o `--resume`, poiché la trascrizione su disco contiene ancora il contenuto che attiva. Su [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) e [Microsoft Foundry](/docs/it/microsoft-foundry), questo messaggio copre anche le richieste che le misure di sicurezza del modello hanno contrassegnato come un argomento di cibersicurezza. Vedete [Safety measures flagged a cybersecurity topic](#safety-measures-flagged-a-cybersecurity-topic).

Prima della v2.1.219, il messaggio recitava `Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.`

**Cosa fare:**

* Premete Esc due volte o eseguite `/rewind` per tornare indietro a un checkpoint prima del turno che ha attivato il rifiuto, quindi riformulate o prendete un approccio diverso. Vedete [Checkpointing](/docs/it/checkpointing).
* Se non riuscite a identificare quale turno l'ha causato, eseguite `/clear` per avviare una conversazione nuova nello stesso progetto. La Vostra conversazione precedente è preservata su disco e rimane disponibile in `/resume`.
* In [modalità non interattiva](/docs/it/headless) (`-p`), dove il rewind non è disponibile, ritentate con un prompt riformulato in una nuova sessione senza `--continue`. I controlli della politica variano per modello, quindi passare a un modello diverso con `--model` può anche risolvere il rifiuto in alcuni casi.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Le misure di sicurezza hanno contrassegnato un argomento di cibersicurezza
</h3>

Le misure di sicurezza del modello hanno contrassegnato il contenuto nella conversazione come un argomento di cibersicurezza. Il messaggio nomina il modello che ha contrassegnato la richiesta:

```text theme={null}
API Error: Opus 4.8's safeguards flagged this message. Our intentionally broad safeguards allow us to deliver more capabilities faster, but can sometimes flag legitimate cybersecurity work. Apply to the Cyber Verification Program to reduce these interruptions. Send feedback with /feedback or learn more: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude
```

Il messaggio si collega al [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), che concede accesso per il lavoro di cibersicurezza legittimo. Su Opus 5.5, che richiede v2.1.280 o successiva, il messaggio si apre con `Opus 5.5's safeguards flagged this session` invece. Quando la categoria contrassegnata ha un modello di fallback disponibile, Claude Code [cambia modelli](/docs/it/model-config#automatic-model-fallback) piuttosto che mostrare questo errore.

Su [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) e [Microsoft Foundry](/docs/it/microsoft-foundry), un contrassegno di cibersicurezza produce il messaggio di [rifiuto della politica di utilizzo](#usage-policy-refusal) invece.

La salvaguardia stessa è lato server e precede la v2.1.203; i rilasci client da allora hanno cambiato solo la formulazione del messaggio.
Dalla v2.1.203 alla v2.1.218, il messaggio recitava `<model> has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center:` seguito dallo stesso link del centro assistenza, e le sessioni interattive aggiungevano `If you were not engaging in a cybersecurity topic, please send feedback via /feedback.`
Prima della v2.1.203, recitava `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` seguito da un link di modulo di esenzione.

**Cosa fare:**

* Se il Vostro lavoro richiede questo contenuto, applicate per l'accesso attraverso il [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Se la Vostra richiesta non riguardava un argomento di cibersicurezza, eseguite `/feedback` per segnalare il falso positivo
* Per continuare a lavorare nella stessa sessione, premete Esc due volte o eseguite `/rewind` per tornare indietro a un checkpoint prima del turno che ha attivato il contrassegno, quindi prendete un approccio diverso. Vedete [Checkpointing](/docs/it/checkpointing).

<h2 id="installation-errors">
  Errori di installazione
</h2>

Questi errori compaiono durante l'installazione o l'aggiornamento di Claude Code, dallo [script di installazione](/docs/it/setup#install-claude-code), `claude install`, o `claude update`. Per i problemi di `command not found`, PATH, permessi e TLS durante la configurazione, vedere [Risoluzione dei problemi di installazione e accesso](/docs/it/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  L'installazione è stata interrotta prima di poter terminare
</h3>

Lo script di installazione segnala quando il passaggio `claude install` viene terminato da un segnale. Su Linux, il codice di uscita 137 significa che il processo ha ricevuto SIGKILL, e su un host con poca memoria è solitamente il killer out-of-memory (OOM) del kernel. Lo script stampa questa spiegazione ed esce con il codice 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Per qualsiasi altro segnale fatale, e per il codice di uscita 137 su macOS, lo script stampa `Installation was killed before it could finish (exit code <N>)` con il codice di uscita effettivo e omette la spiegazione della memoria insufficiente. Il messaggio proviene dallo script di installazione che macOS e Linux utilizzano, che copre anche le installazioni all'interno di WSL; gli script di installazione nativi di Windows non lo stampano mai. Prima della v2.1.200, lo script usciva con solo la riga `Killed` nuda della shell.

**Cosa fare:**

* Interrompere altri processi per liberare memoria, quindi eseguire nuovamente il programma di installazione
* Aggiungere spazio di swap o passare a un'istanza più grande. Vedere [Installazione interrotta su server Linux con poca memoria](/docs/it/troubleshoot-install#install-killed-on-low-memory-linux-servers) per i comandi del file di swap.

<h3 id="the-connection-dropped-while-downloading-the-update">
  La connessione è stata interrotta durante il download dell'aggiornamento
</h3>

La connessione al server di download si è chiusa mentre `claude install`, `claude update`, o l'[aggiornamento automatico](/docs/it/setup#auto-updates) stava recuperando il binario di Claude Code, e i tentativi di ripetizione non hanno recuperato. Claude Code ritenta il download quando la connessione si interrompe, il trasferimento si blocca, o il file scaricato non supera il checksum, fino a tre tentativi in totale. Un errore HTTP completato, come un 404, non viene ritentato perché il server ha già risposto. Prima della v2.1.202, una singola connessione interrotta faceva fallire il download immediatamente con il semplice errore `aborted` invece di ritentare.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

Il testo tra parentesi nomina quale tentativo ha fallito e l'errore di rete sottostante. `claude update` precede il messaggio con `Error: Failed to install native update` su stderr.

Un download che rimane connesso ma non termina entro 10 minuti fallisce con `Download timed out: exceeded the total deadline` invece. Claude Code non ritenta un download scaduto, perché una connessione troppo lenta per terminare entro il limite non terminerà nemmeno con un tentativo immediato. I passaggi seguenti si applicano a entrambi i messaggi.

La causa più comune è un proxy o un gateway che chiude un trasferimento lungo prima che termini. Il binario di Claude Code è un download di grandi dimensioni, quindi un limite di connessione proxy che non influisce mai sul traffico API normale può comunque interromperlo.

**Cosa fare:**

* Eseguire `claude update` di nuovo. Su una rete altrimenti sana, il download di solito ha successo alla prossima esecuzione. Per il messaggio di timeout, eseguirlo di nuovo da una rete più veloce o meno limitata.
* Se la rete richiede un proxy, impostare `HTTPS_PROXY` prima di eseguire il programma di installazione o `claude update`. Vedere [Verificare la connettività di rete](/docs/it/troubleshoot-install#check-network-connectivity).
* Se un proxy aziendale continua a chiudere il trasferimento, chiedere al team di rete di consentire il download completo da `downloads.claude.ai`. Vedere [Requisiti di accesso alla rete](/docs/it/network-config#network-access-requirements).
* Eseguire `claude doctor` dalla shell per la diagnostica dell'installazione

<h2 id="command-line-errors">
  Errori da riga di comando
</h2>

Questi errori provengono dal comando `claude` da riga di comando e dai suoi sottocomandi, da un nome di comando che invii al prompt, e da comandi come `/security-review` che raccolgono il contesto eseguendo comandi shell prima che il loro prompt venga eseguito. Provengono anche da `/tui`, che riavvia la CLI.

<h3 id="conflict-between-bg-and-print">
  Conflitto tra --bg e --print
</h3>

Questo messaggio richiede Claude Code v2.1.198 o successivo. Hai combinato `--bg` con `-p` o `--print` nella stessa invocazione di `claude`. `--bg` avvia una [sessione in background](/docs/it/agent-view#from-your-shell) a cui ti colleghi successivamente con `claude agents`, mentre `--print` esegue [in modo non interattivo](/docs/it/headless) e non avvia mai la sessione interattiva a cui `claude agents` si collega. Prima della v2.1.198 questa combinazione creava silenziosamente un lavoro in background che non poteva mai essere collegato.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**Cosa fare:**

* Rimuovi `-p` o `--print`. `--bg` accetta il prompt come argomento posizionale, quindi `claude --bg "<task>"` è il comando completo. Vedi [Dispatch new agents from your shell](/docs/it/agent-view#from-your-shell).
* Per eseguire il prompt in modo non interattivo e stampare il risultato invece di creare una sessione in background, rimuovi `--bg` ed esegui `claude -p "<task>"`

<h3 id="invalid-agents-configuration">
  Configurazione --agents non valida
</h3>

Il valore che hai passato a `--agents` non è valido, quindi `claude` esce con codice 1 invece di avviare la sessione. Quando passi `--safe-mode`, `--resume`, o `--continue`, o imposti [`CLAUDE_CODE_SAFE_MODE`](/docs/it/env-vars#variables), Claude Code non controlla il valore e avvia la sessione. Prima della v2.1.242, Claude Code avviava comunque la sessione e ometteva le definizioni che non poteva caricare.

```text theme={null}
Error: Invalid --agents configuration:
<what failed>
```

Quello che segue la prima riga dipende da come il valore ha fallito. Claude Code esegue questi controlli in ordine e si ferma al primo che fallisce. Se il tuo valore ha due tipi di problema, vedi il secondo solo dopo aver corretto il primo:

1. Quando il valore non viene analizzato come JSON, Claude Code stampa una riga `invalid JSON:` con il messaggio del parser JSON stesso
2. Quando viene analizzato ma una definizione di agente non corrisponde allo schema per [subagenti definiti da CLI](/docs/it/sub-agents#choose-the-subagent-scope), Claude Code stampa una riga per problema
3. Quando un nome di agente inizia con `-`, Claude Code stampa `<name>: agent names must not start with '-'`

Quando ci sono più di 20 righe di problema, Claude Code stampa le prime 20 e sostituisce il resto con `…and N more`.

**Cosa fare:**

* Correggi ogni problema che il messaggio elenca, quindi esegui di nuovo il comando. Vedi [i campi che un subagente definito da CLI accetta](/docs/it/sub-agents#choose-the-subagent-scope).

<h3 id="cloud-sessions-cannot-be-created-from-a-restricted-session">
  Le sessioni cloud non possono essere create da una sessione --restricted
</h3>

Quando avvii una sessione con [`--restricted`](/docs/it/cli-reference#cli-flags), Claude Code rifiuta di creare [sessioni cloud](/docs/it/claude-code-on-the-web#from-terminal-to-cloud) da essa, perché la nuova sessione verrebbe eseguita al di fuori del processo ristretto e non farebbe rispettare la modalità ristretta. Claude Code rifiuta sul client, prima di contattare il server, quindi nessuna sessione cloud viene creata:

```text theme={null}
Cloud sessions cannot be created from a --restricted session: they would not enforce it.
```

**Cosa fare:**

* Esegui l'attività localmente nella sessione ristretta
* Se controlli come è stata avviata la sessione, avvia una nuova sessione `claude` senza `--restricted` e crea la sessione cloud da lì

Prima della v2.1.248, Claude Code non aveva il flag `--restricted`; le versioni precedenti rifiutano il flag stesso con un errore di opzione sconosciuta.

<h3 id="cloud-sessions-are-disabled-by-your-organizations-policy">
  Le sessioni cloud sono disabilitate dalla politica della tua organizzazione
</h3>

La politica `allow_remote_sessions` della tua organizzazione è disattivata, quindi [sessioni cloud](/docs/it/claude-code-on-the-web) e i comandi che le utilizzano non sono disponibili:

```text theme={null}
Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.
```

Il messaggio appare quando [crei una sessione cloud dal terminale](/docs/it/claude-code-on-the-web#from-terminal-to-cloud) e quando invii un comando che ha bisogno di sessioni cloud, come `/teleport`, `/remote-env`, o `/web-setup`. Prima della v2.1.268, l'invio di uno di questi comandi restituiva [`Unknown command`](#unknown-command) invece.

Questa è una politica organizzativa lato server, quindi non può essere ignorata dalle impostazioni locali, variabili di ambiente o flag CLI.

Se Claude Code non ha ancora caricato la politica della tua organizzazione o non può recuperarla, questi comandi rispondono `Couldn't verify your organization's policy for cloud sessions. Check your network connection, then restart Claude Code and try again.` invece.

**Cosa fare:**

* Chiedi a un [Owner](/docs/it/server-managed-settings#access-control) nella tua organizzazione di abilitare le sessioni cloud nelle impostazioni di amministrazione di Claude Code su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Se il messaggio dice che non potrebbe verificare la politica, controlla la tua connessione di rete, quindi riavvia Claude Code e riprova

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  Il valore --json-schema non è un JSON Schema valido
</h3>

Lo schema che hai passato a [`--json-schema`](/docs/it/cli-reference#cli-flags) in [modalità non interattiva](/docs/it/headless#get-structured-output) ha fallito la compilazione di JSON Schema, quindi `claude` esce con codice 1 invece di eseguire il prompt. Prima della v2.1.205, uno schema non valido produceva output non strutturato senza errore, e qualsiasi schema che utilizzava la parola chiave `format` era trattato come non valido.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

Il testo dopo il secondo due punti è la diagnostica del validatore e nomina la parola chiave o la posizione che ha fallito. Gli schemi che utilizzano la parola chiave `format`, come `"format": "email"`, sono validi: Claude Code accetta `format` come annotazione e non la applica.

Claude Code esegue due controlli prima della compilazione dello schema: rifiuta un valore che non è JSON analizzabile con `Error: --json-schema is not valid JSON`, e JSON valido che non è un oggetto con `Error: --json-schema must be a JSON object`.

**Cosa fare:**

* Correggi la parte dello schema che la diagnostica nomina, quindi riesegui il comando
* Se la diagnostica è `schema too large`, riduci l'annidamento dello schema e il riutilizzo di `$ref`
* Vedi [Get structured output](/docs/it/headless#get-structured-output) per uno schema funzionante e un comando

<h3 id="settings-file-exceeds-the-2mib-limit">
  Il file di impostazioni supera il limite di 2MiB
</h3>

Il file che hai passato a [`--settings`](/docs/it/cli-reference#cli-flags) è più grande di 2 MiB, quindi `claude` esce con codice 1 all'avvio invece di caricarlo. Un file di impostazioni è un piccolo documento JSON, quindi un file di questa dimensione di solito significa che il percorso punta al file sbagliato. Prima della v2.1.214, Claude Code leggeva il file senza controllo delle dimensioni, e un file di più gigabyte o un file di dispositivo come `/dev/zero` faceva crescere la memoria senza limiti.

```text theme={null}
Error: Settings file exceeds the 2MiB limit: /path/to/settings.json
```

Claude Code rifiuta un percorso `--settings` che non è un file regolare allo stesso modo: un dispositivo, FIFO o socket segnala `Error: Cannot use settings file (Not a regular file (device, FIFO, or socket))` seguito dal percorso, e una directory segnala un motivo `EISDIR`.

**Cosa fare:**

* Punta `--settings` a un file JSON di impostazioni regolare sotto 2 MiB. Vedi [Settings](/docs/it/settings) per il formato.

<h3 id="the-current-directory-no-longer-exists">
  La directory corrente non esiste più
</h3>

Hai avviato `claude` da una directory che è stata eliminata o spostata dopo che la tua shell vi è entrata, ad esempio una worktree o una directory temporanea che un'altra shell ha rimosso. Claude Code non può leggere la sua directory di lavoro, quindi esce con codice 1 prima di avviare la sessione, sia in modalità interattiva che [non interattiva](/docs/it/headless). Prima della v2.1.239, Claude Code si bloccava con il codice sorgente del bundle minimizzato e uno stack `ENOENT ... uv_cwd` grezzo su stderr invece di questo messaggio.

```text theme={null}
The current directory no longer exists (it was deleted or moved). Start Claude Code from an existing directory.
error: The current working directory was deleted, so that command didn't work. Please cd into a different directory and try again.
```

La causa e la correzione sono le stesse per entrambe le forme.

Quando Claude Code non può leggere la directory di lavoro per un motivo diverso, come un cambio di permessi, il messaggio nomina il codice di errore invece: `Can't read the current directory (EACCES). Start Claude Code from a different directory.`

Su macOS, `EPERM` per una directory in `~/Desktop`, `~/Documents`, `~/Downloads`, o iCloud Drive di solito significa che macOS sta bloccando l'accesso della tua app terminale a quella cartella. Altri comandi che leggono quella cartella falliscono allo stesso modo: `ls` lì segnala `Operation not permitted`, anche con `sudo`.

**Cosa fare:**

* Cambia a una directory che esiste, come la tua home o la directory del progetto, quindi esegui di nuovo `claude`
* Se la directory è stata ricreata nello stesso percorso, la tua shell ne tiene ancora quella eliminata. Esegui `cd "$PWD"` o esci e rientra nella directory, quindi esegui di nuovo `claude`
* Per `EPERM` su macOS, esci dalla tua app terminale con Cmd+Q, aprila di nuovo, torna a quella cartella, ed esegui `claude`. Se `ls` in quella cartella continua a fallire, apri **System Settings > Privacy & Security > Files and Folders**, attiva la cartella per la tua app terminale, quindi riapri il terminale

<h3 id="temp-directory-refused-or-cannot-be-created">
  La directory temporanea è stata rifiutata o non può essere creata
</h3>

Su macOS e Linux, Claude Code crea una directory temporanea privata all'avvio, `claude-<uid>` sotto la directory temporanea di sistema o l'override [`CLAUDE_CODE_TMPDIR`](/docs/it/env-vars). Quando la directory non può essere creata, o una voce già in quel percorso fallisce i controlli di sicurezza, Claude Code stampa il fallimento su stderr e esce con codice 1 piuttosto che avviare la sessione:

```text wrap theme={null}
ENOSPC: no space left on device, mkdir '/tmp/claude-501'

Temp directory /tmp/claude-501 is not a directory (may be an attacker-planted symlink). Refusing to use it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is owned by uid 502, expected 501. Refusing to use it — another user may have pre-created it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is not readable (its mode may have been altered, or a path component denies search). Refusing to use it — restore its permissions (chmod 0700) or remove it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.
```

**Cosa fare:**

* Per `ENOSPC`, libera spazio su disco sul volume che contiene la directory temporanea
* Per le forme `Refusing to use it`, rimuovi la voce denominata stessa, non quello a cui un link punta, e avvia di nuovo Claude Code; per la forma `owned by uid`, solo un amministratore o quell'utente può rimuoverla
* Per `is not readable`, esegui `chmod 0700` sulla directory denominata, o rimuovila e avvia di nuovo
* In uno qualsiasi di questi casi, imposta [`CLAUDE_CODE_TMPDIR`](/docs/it/env-vars) a una directory che controlli e avvia di nuovo Claude Code, lasciando il percorso rifiutato da solo

<h3 id="directory-couldnt-be-resolved-to-a-real-location">
  La directory non potrebbe essere risolta a una posizione reale
</h3>

Hai eseguito `/add-dir` per una sottodirectory della tua directory di lavoro, e Claude Code non potrebbe risolvere la directory a una posizione reale.

Hai già accesso ai file a una sottodirectory della directory di lavoro, quindi `/add-dir` carica solo le sue skills, comandi e agenti. Prima di caricarli, Claude Code verifica che la posizione reale della directory, con eventuali symlink risolti, sia all'interno della directory di lavoro. Quando Claude Code non può risolvere quella posizione, non carica nulla e mostra questo messaggio:

```text theme={null}
packages/app couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded. Check that it is a directory inside the working directory and try again.
```

**Cosa fare:**

* Verifica che il percorso nomini una directory reale all'interno della directory di lavoro, quindi esegui di nuovo `/add-dir`
* Il messaggio non cambia il tuo accesso ai file; riporta solo che il contenuto `.claude/` della directory non è stato caricato

Prima della v2.1.261, questo messaggio appariva anche per ogni `/add-dir <subdirectory>` quando la directory di lavoro era su un automount `/net/<host>`, dove Claude Code rifiuta di risolvere i percorsi per progettazione; la directory era fine e riprovare non poteva aiutare.

<h3 id="workspace-not-trusted-when-starting-remote-control">
  Workspace non attendibile all'avvio di Remote Control
</h3>

Hai avviato la modalità server [Remote Control](/docs/it/remote-control) con `claude remote-control` o il suo alias `claude rc` in una directory che non hai attendibile. Il comando non mostra il dialogo di attendibilità dell'area di lavoro stesso, quindi esce con codice 1 e nomina la correzione:

```text theme={null}
Error: Workspace not trusted. Please run `claude` in /Users/you/project first to review and accept the workspace trust dialog.
```

Nella tua home directory il messaggio è diverso, perché il dialogo di attendibilità dell'area di lavoro non salva mai l'attendibilità per la home directory, quindi accettarlo lì non può soddisfare questo controllo. Prima della v2.1.214, la home directory mostrava il messaggio sopra, il cui consiglio non può avere successo lì.

```text theme={null}
Error: Workspace not trusted. /Users/you is your home directory, and for security home-directory trust is never saved, so running `claude` here first won't help. Run `claude rc` from a project directory instead (run `claude` there once to accept the trust dialog).
```

**Cosa fare:**

* Esegui `claude` nella directory, accetta il [dialogo di attendibilità dell'area di lavoro](/docs/it/permissions#project-allow-rules-and-workspace-trust), quindi esegui di nuovo `claude remote-control`
* Nella tua home directory, cambia a una directory del progetto e avvia Remote Control lì

<h3 id="not-carried-over-to-the-sessions-remote-control-starts">
  Non trasportato alle sessioni che Remote Control avvia
</h3>

Hai avviato [Remote Control](/docs/it/remote-control) con un flag `claude` globale prima del verbo `remote-control`, uno che limiterebbe o configurerebbe le sessioni che Remote Control avvia, come `--settings`, `--setting-sources`, `--permission-mode`, `--disallowed-tools`, o `--mcp-config`. Un flag posizionato prima del verbo non raggiunge mai quelle sessioni. Claude Code rifiuta di avviare invece, nominando il flag:

```text theme={null}
Error: `--settings` before `remote-control` is not carried over to the sessions Remote Control starts, so Remote Control refuses to start rather than drop it — remove it, and give Remote Control's own options after the verb (see `claude remote-control --help`).
```

Claude Code non rifiuta i flag globali che sono innocui da eliminare, come `--verbose`, `--model`, o un `--session-id` o `--plugin-dir` iniettato dal wrapper: li ignora e Remote Control si avvia.

Claude Code rifiuta anche di avviare per un flag globale che non riconosce ancora come innocuo, quindi un flag aggiunto in una versione più recente può apparire in questo messaggio fino a quando una versione successiva non lo contrassegna come innocuo.

**Cosa fare:**

* Rimuovi il flag da prima del verbo e passa [le opzioni proprie di Remote Control](/docs/it/remote-control#start-a-remote-control-session) dopo di esso; `claude remote-control --help` le elenca
* Quando il flag rifiutato è `--permission-mode`, esegui `claude remote-control --permission-mode <mode>` per impostare la modalità di permesso per le sessioni che Remote Control avvia

Prima della v2.1.248, `claude remote-control` non accettava i suoi flag quando un flag globale veniva per primo, e il comando falliva con un errore `unknown option`.

<h3 id="claude-import-is-not-yet-available-in-this-build">
  claude import non è ancora disponibile in questa build
</h3>

Hai eseguito [`claude import`](/docs/it/cli-reference#cli-commands), e Claude Code ha trovato il flusso di importazione disattivato, quindi il comando esce con codice 1 invece di avviare l'importazione. Prima della v2.1.222, una build con il flusso di importazione disattivato trattava `import` come un prompt e avviava una sessione interattiva invece di stampare questo messaggio.

```text theme={null}
`claude import` is not yet available in this build. Run `claude` and use /mcp or edit ~/.claude/settings.json directly.
```

Claude Code attiva `claude import` attraverso un feature flag che recupera da Anthropic e memorizza nella cache su disco. Questo messaggio significa che il valore memorizzato nella cache è disattivato. La causa è di solito una delle seguenti:

* Non hai avviato una sessione dall'installazione, quindi Claude Code non ha ancora recuperato il flag. Il primo `claude import` può stampare questo anche quando la funzione è disponibile per te.
* Usi Claude Code attraverso Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o Claude Platform su AWS, o attraverso un [gateway di app Claude](/docs/it/claude-apps-gateway#availability-and-limitations). Claude Code non recupera i feature flag in queste sessioni, quindi `claude import` rimane non disponibile.
* Hai impostato `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `DISABLE_GROWTHBOOK`, o [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars), che disattivano il recupero dei feature flag, quindi `claude import` rimane non disponibile.

**Cosa fare:**

* Su un'installazione nuova, avvia `claude`, attendi che la sessione si carichi, esci, ed esegui di nuovo `claude import`
* Dove il recupero dei feature flag rimane disattivato, configura tu stesso: aggiungi server MCP con [`claude mcp add`](/docs/it/mcp#installing-mcp-servers), e crea i file [`CLAUDE.md`](/docs/it/memory#how-claude-md-files-load), [skills e comandi](/docs/it/skills#where-skills-live), e [subagenti](/docs/it/sub-agents#choose-the-subagent-scope) che vuoi trasportare. Il messaggio nomina anche `~/.claude/settings.json`. Della configurazione che `claude import` trasporta, quel file contiene solo la [modalità di permesso](/docs/it/settings-reference#permission-settings); Claude Code non legge i server MCP da esso.

<h3 id="could-not-read-claude-code-config">
  Non potrebbe leggere la configurazione di Claude Code
</h3>

Hai eseguito [`claude import`](/docs/it/cli-reference#cli-commands) mentre Claude Code non potrebbe analizzare `~/.claude.json`, il file dove memorizza il tuo login e lo stato per progetto. Il sottocomando legge quel file per controllare la disponibilità ma non mostra il dialogo di recupero che la sessione interattiva mostra, quindi esce con codice 1. Prima della v2.1.222, `claude import` con un file di configurazione illeggibile avviava una sessione interattiva, il cui dialogo di recupero gestiva il file.

```text theme={null}
Could not read Claude Code config — run `claude` with no arguments to recover it.
```

**Cosa fare:**

* Esegui `claude` senza argomenti. Claude Code rileva il file non valido e offre di ripristinarlo. Quindi esegui di nuovo `claude import`.
* Per mantenere le modifiche manuali che hai fatto, correggi la sintassi JSON in `~/.claude.json` in un editor invece, quindi riesegui `claude import`

<h3 id="could-not-import-a-server-from-claude-desktop">
  Non potrebbe importare un server da Claude Desktop
</h3>

Claude Code non potrebbe aggiungere uno dei server che hai selezionato in `claude mcp add-from-claude-desktop`. Il comando importa comunque gli altri server selezionati e stampa una riga per server che non potrebbe aggiungere. Prima della v2.1.205, il primo server che falliva fermava l'importazione e nessuno dei server selezionati veniva aggiunto.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

Il testo dopo il nome del server è il motivo. Il più comune è il controllo del nome: Claude Desktop consente caratteri nei nomi dei server, come spazi e punti, che `claude mcp` limita a lettere, numeri, trattini e sottolineature. Altri motivi includono una configurazione del server che fallisce la validazione e un server bloccato dalla [politica MCP](/docs/it/managed-mcp) della tua organizzazione.

**Cosa fare:**

* Rinomina il server in `claude_desktop_config.json` per usare solo lettere, numeri, trattini e sottolineature, quindi esegui di nuovo `claude mcp add-from-claude-desktop`
* Aggiungi quel server direttamente con `claude mcp add` o `claude mcp add-json` con un nome valido. Vedi [Import MCP servers from Claude Desktop](/docs/it/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="cannot-add-mcp-server-to-the-managed-scope">
  Non potrebbe aggiungere il server MCP allo scope gestito
</h3>

Hai eseguito `claude mcp add` o `claude mcp add-json` con `--scope managed`. Quello scope contiene i server che la tua organizzazione fornisce attraverso l'impostazione gestita [`managedMcpServers`](/docs/it/settings-reference#managedmcpservers). Claude Code li legge solo dalle impostazioni gestite, quindi il comando non può scrivere un server in quello scope.

```text theme={null}
Cannot add MCP server to scope: managed
```

**Cosa fare:**

* Aggiungi il server a uno scope in cui puoi scrivere: `local`, `user`, o `project`. Senza `--scope`, il comando usa `local`. Vedi [MCP installation scopes](/docs/it/mcp#mcp-installation-scopes)
* Per fornire il server a ogni utente nella tua organizzazione, aggiungilo a [`managedMcpServers`](/docs/it/settings-reference#managedmcpservers) nelle impostazioni gestite che distribuisci

<h3 id="cant-read-mcp-json">
  Non potrebbe leggere .mcp.json
</h3>

Un comando che legge il [`.mcp.json`](/docs/it/mcp#project-scope) del progetto, come `claude mcp add` o `claude mcp add-json` con `--scope project`, o `claude mcp remove`, ha trovato che il file nella tua directory corrente non è un file regolare o è più grande di 2 MiB, quindi esce con questo errore invece di leggere il file.

```text theme={null}
Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes. Fix or remove it, then run the command again.
```

Prima della v2.1.257, un FIFO a `.mcp.json` lasciava il comando in attesa per sempre senza output, e un symlink a un file di dispositivo come `/dev/zero` faceva crescere la memoria fino a quando il processo veniva ucciso.

**Cosa fare:**

* Controlla cosa si trova a `.mcp.json` nella tua directory corrente. Sostituiscilo con un file JSON ordinario nel [formato project-scope](/docs/it/mcp#project-scope), o eliminalo, quindi esegui di nuovo il comando.

<h3 id="anthropic-hosted-and-doesnt-support-local-oauth">
  Il server è ospitato da Anthropic e non supporta OAuth locale
</h3>

Hai avviato un accesso per un server MCP il cui URL punta a un host connettore ospitato da Anthropic che si autentica attraverso un provider di identità di terze parti. Questi host includono `microsoft365.mcp.claude.com`, `gmail.mcp.claude.com`, e `gcal.mcp.claude.com`. Claude Code rifiuta di avviare il suo flusso OAuth locale per questi host sia dal pannello `/mcp` che da `claude mcp login`, perché [il loro accesso funziona solo attraverso claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai).

```text theme={null}
"gmail" is Anthropic-hosted and doesn't support local OAuth. Connect it via Settings → Connectors on claude.ai (requires `claude login`), then it'll be available here automatically.
```

Claude Code corrisponde a questi host per URL, quindi il messaggio appare quando un server che hai aggiunto con `claude mcp add` o in `.mcp.json` punta a uno di loro.

**Cosa fare:**

* Rimuovi la tua voce con `claude mcp remove <name>`, così non può nascondere il connettore claude.ai allo stesso URL
* Dopo averlo rimosso, connetti il servizio su [claude.ai/customize/connectors](https://claude.ai/customize/connectors), mentre sei connesso all'account che usi in Claude Code. Una volta connesso, [il connettore appare in Claude Code automaticamente](/docs/it/mcp#use-mcp-servers-from-claude-ai) se il tuo metodo di autenticazione attivo è un accesso di sottoscrizione claude.ai

<h3 id="server-rejected-the-authorization-header-minted-by-the-configured-headershelper">
  Il server ha rifiutato l'intestazione Authorization coniata dal headersHelper configurato
</h3>

Un server MCP il cui [`headersHelper`](/docs/it/mcp#use-dynamic-headers-for-custom-authentication) fornisce l'intestazione `Authorization` ha risposto alla connessione con HTTP 401 o 403, quindi Claude Code segnala la connessione come fallita. Poiché l'helper fornisce l'intestazione `Authorization`, Claude Code [non ricade su OAuth](/docs/it/mcp#authenticate-with-remote-mcp-servers) per il server:

```text theme={null}
Server rejected the Authorization header minted by the configured headersHelper (HTTP 401). Check that the helper command returns a valid credential for this MCP endpoint — OAuth fallback is disabled when the helper supplies Authorization.
```

Claude Code riesegue l'helper ad ogni tentativo di connessione, quindi un nuovo tentativo dopo un rifiuto transitorio, come una gara di rotazione del token, può avere successo con una credenziale nuova.

**Cosa fare:**

* Esegui il comando `headersHelper` tu stesso nel modo in cui Claude Code lo esegue: dalla [directory in cui Claude Code lo esegue](/docs/it/mcp#where-the-helper-runs), con le [variabili di ambiente che Claude Code imposta per esso](/docs/it/mcp#use-dynamic-headers-for-custom-authentication), e senza le [variabili di credenziale che Claude Code rimuove](/docs/it/mcp#which-variables-a-helper-can-read) per un server da un `.mcp.json` del progetto, un plugin, o un file agente del progetto. Controlla che stampi un valore `Authorization` che l'endpoint del server accetta
* Dopo aver corretto l'helper o la sua fonte di credenziale, seleziona il server in `/mcp` e scegli **Reconnect**

Prima della v2.1.248, Claude Code eseguiva la scoperta OAuth per un server il cui helper forniva l'intestazione `Authorization`. Quella scoperta potrebbe fallire con `Incompatible auth server: does not support dynamic client registration` invece di segnalare la credenziale rifiutata.

<h3 id="mcp-permission-prompt-tool-not-found">
  Strumento di prompt di permesso MCP non trovato
</h3>

Lo strumento che hai passato a [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags) non era tra gli strumenti MCP connessi quando l'esecuzione ha avuto bisogno per la prima volta di una decisione di permesso, perché il suo server non si è mai connesso o perché nessun server connesso espone uno strumento con quel nome. Claude Code invia comunque il tuo prompt: l'esecuzione [non interattiva](/docs/it/headless) esce con questo errore, e codice di uscita 1, alla prima chiamata di strumento che necessita di approvazione, quindi non produce alcuna risposta anche se la richiesta è stata fatta. Prima del primo prompt, Claude Code attende fino al timeout di connessione per server di 30 secondi impostato da [`MCP_TIMEOUT`](/docs/it/env-vars) affinché quel server si connetta. Prima della v2.1.206, l'avvio non attendeva che il server finisse di connettersi, quindi un server che si avvia lentamente ma sano produceva questo errore anche.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

L'elenco dopo `Available MCP tools:` nomina gli strumenti MCP che erano connessi quando l'attesa è terminata.

**Cosa fare:**

* Controlla che il server si avvii e rimanga connesso: esegui `claude mcp list` nella stessa directory e conferma che il server è elencato come connesso
* Conferma che il nome dello strumento corrisponda al nome `mcp__<server>__<tool>` che il server espone
* Se il server ha bisogno di più di 30 secondi per avviarsi, aumenta [`MCP_TIMEOUT`](/docs/it/env-vars)

<h3 id="oauth-callback-port-is-already-in-use">
  La porta di callback OAuth è già in uso
</h3>

Quando accedi a un server MCP remoto con OAuth, Claude Code avvia un listener locale per ricevere il callback di accesso. Se la porta di cui quel listener ha bisogno è tenuta da un altro processo, l'accesso fallisce con questo messaggio. Questo accade principalmente con una [porta di callback fissa](/docs/it/mcp#use-a-fixed-oauth-callback-port) impostata attraverso la variabile [`MCP_OAUTH_CALLBACK_PORT`](/docs/it/env-vars) o `--callback-port`, poiché senza una Claude Code sceglie una porta disponibile.

```text theme={null}
OAuth callback port <port> is already in use — another process may be holding it. Run `lsof -ti:<port> -sTCP:LISTEN` to find it.
```

Su Windows, il comando suggerito è `netstat -ano | findstr :<port>` invece.

**Cosa fare:**

* Esegui il comando dal messaggio per trovare il processo che tiene la porta, e fermalo o attendi che finisca
* Se un altro programma ha bisogno permanentemente di quella porta, registra un URI di reindirizzamento diverso con il server e imposta la sua porta con `MCP_OAUTH_CALLBACK_PORT` o `--callback-port`, a seconda di quale usi
* Quindi avvia di nuovo l'accesso, ad esempio selezionando il server in `/mcp`

<h3 id="no-available-ports-for-oauth-redirect">
  Nessuna porta disponibile per il reindirizzamento OAuth
</h3>

Quando accedi a un server MCP remoto con [OAuth](/docs/it/mcp#authenticate-with-remote-mcp-servers), Claude Code avvia un listener locale per ricevere il callback di accesso. L'accesso fallisce con questo messaggio quando Claude Code non può associare una porta locale per esso. Qualcosa sulla macchina sta impedendo di ascoltare su `127.0.0.1`, ad esempio software di sicurezza o una politica sandbox che nega i listener locali.

```text theme={null}
No available ports for OAuth redirect
```

Prima della v2.1.268, Claude Code non ricadeva su una porta assegnata dal sistema operativo, quindi il messaggio appariva anche quando solo le sue porte auto-scelte non potevano essere associate. Questo può accadere su host Windows dove Hyper-V riserva intervalli di porte che coprono le porte che Claude Code sceglie.

**Cosa fare:**

* Controlla se il software di sicurezza o una politica sandbox blocca i processi dall'ascolto su `127.0.0.1`, e consenti a Claude Code di associare una porta locale
* Quindi avvia di nuovo l'accesso, ad esempio selezionando il server in `/mcp`

<h3 id="security-review-fails-without-origin-head">
  /security-review fallisce senza origin/HEAD
</h3>

[`/security-review`](/docs/it/commands#all-commands) costruisce il suo contesto di revisione facendo il diff del tuo branch rispetto a `origin/HEAD`, il ref locale che registra quale branch è il predefinito sul tuo remote `origin`. Quando quel ref non esiste, i comandi git che raccolgono il diff falliscono e la revisione si ferma prima di iniziare.

```text theme={null}
Error: Shell command failed for pattern "!`git diff --name-only origin/HEAD...`": [stderr]
fatal: ambiguous argument 'origin/HEAD...': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
```

Il messaggio può citare `git log` o un diverso `git diff` invece. Git crea `origin/HEAD` solo quando il remote pubblicizza un branch predefinito e il tuo refspec di fetch lo copre, che un `git clone` completo di un remote con commit fa. Il ref è mancante in questi setup:

* Un checkout single-branch o CI, che recupera un refspec troppo stretto
* Un remote il cui server-side HEAD punta a un branch che nessuno ha spinto
* Un repository senza un remote `origin`, o uno da cui non hai mai recuperato

Claude Code mostra lo stesso errore per qualsiasi skill che [inietta contesto dinamico](/docs/it/skills#when-an-injected-command-fails), e un comando iniettato fallito interrompe l'invocazione di quella skill. Due stringhe sibling si attivano prima che il comando venga eseguito affatto:

* `Shell command permission check failed for pattern "..."`: il controllo di permesso del comando non ha consentito. [Permission checks on injected commands](/docs/it/skills#permission-checks-on-injected-commands) copre quali risultati interrompono in ogni modalità di permesso e come pre-approvare un comando con `allowed-tools`
* ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``: il frontmatter della skill richiede bash su una macchina senza di esso. Installa Git per Windows o cambia il frontmatter a `shell: powershell`. Vedi [How injected commands run](/docs/it/skills#how-injected-commands-run)

**Cosa fare:**

* Crea il ref nominando il branch predefinito del tuo remote: `git remote set-head origin <default-branch>`. Questo funziona ogni volta che il ref di tracciamento locale `origin/<default-branch>` esiste. Se non esiste, come nei cloni single-branch, recupera prima il branch: esegui `git remote set-branches --add origin <branch>`, quindi `git fetch origin`, quindi riesegui il comando set-head. Riesegui `/security-review`.
* Se preferisci non nominare il branch, esegui `git fetch origin` e quindi `git remote set-head origin --auto`, che chiede al remote quale branch è il suo predefinito. Fallisce con `error: Cannot determine remote HEAD` quando il remote non pubblicizza un branch predefinito, perché è vuoto o il suo HEAD punta a un branch che nessuno ha spinto; nomina il branch esplicitamente invece. Fallisce con `error: Not a valid ref` quando il tuo clone non recupera quel branch; allarga il refspec come sopra prima.
* Se il repository non ha un remote, aggiungine uno con `git remote add origin <url>` e recupera prima di creare il ref. Se il remote è vuoto, spinge il tuo branch prima con `git push -u origin HEAD` e nomina quel branch nel comando set-head; `origin/HEAD` quindi punta al branch che hai appena spinto, quindi `/security-review` vede un diff vuoto fino a quando il branch non diverge da esso.

<h3 id="input-must-be-provided-when-using-print">
  L'input deve essere fornito quando si usa --print
</h3>

Bare `claude` ha bisogno che stdout sia un terminale per avviare l'interfaccia utente interattiva. Quando stdout viene reindirizzato, o la console non è un vero terminale, come PowerShell ISE e alcuni riquadri di output IDE, `claude` esegue [in modo non interattivo](/docs/it/headless) invece. Questo è lo stesso modo di `claude -p`, che richiede un prompt, quindi il messaggio nomina `--print` anche se non hai passato il flag. Passare `-p`/`--print` senza prompt e nulla piped su stdin produce lo stesso errore ovunque.

```text theme={null}
Error: Input must be provided either through stdin or as a prompt argument when using --print
```

**Cosa fare:**

* Per l'uso interattivo, esegui `claude` in un vero terminale: Windows Terminal o la console PowerShell piuttosto che ISE, e il terminale integrato del tuo IDE piuttosto che un riquadro di output
* Per l'uso una tantum, passa il prompt: `claude -p "your question"`, o piped con `echo "your question" | claude -p`

<h3 id="input-contained-only-whitespace">
  L'input conteneva solo spazi bianchi
</h3>

In [modalità non interattiva](/docs/it/headless), Claude Code rifiuta un prompt composto interamente da spazi, tabulazioni o newline invece di inviarlo, perché l'API rifiuta i messaggi senza testo visibile. Quale messaggio vedi dipende da dove è venuto il prompt vuoto:

* **Argomento prompt o stdin piped per `claude -p`**: `claude` esce con `Error: Input contained only whitespace. Provide a prompt with text through stdin or as a prompt argument when using --print`
* **Messaggio inviato a una sessione `--input-format stream-json` o [Agent SDK](/docs/it/agent-sdk/overview) in esecuzione**: Claude Code termina il turno senza chiamare il modello e la sessione rimane utilizzabile. Il rifiuto arriva come messaggio informativo e come testo del risultato del turno: `Blank prompt — the message was only whitespace, so nothing was sent to the model.`

Prima della v2.1.229, Claude Code inviava il messaggio solo spazi bianchi all'API, che rifiutava la richiesta con un errore 400.

**Cosa fare:**

* Includi testo visibile nel prompt. Se uno script costruisce il prompt da una variabile o file, controlla che la fonte non sia vuota prima di chiamare Claude Code.

<h3 id="stream-json-input-carried-over-256m-characters-with-no-newline">
  stream-json input ha trasportato oltre 256M caratteri senza newline
</h3>

Il tuo programma ha inviato più di 268.435.456 caratteri su stdin senza newline a un'esecuzione `claude -p --input-format stream-json`, quindi Claude Code stampa questo errore su stderr e esce con codice 1 invece di bufferizzare più input. Il messaggio dichiara quel budget come `256M`. Prima della v2.1.257, Claude Code bufferizzava tale input senza limiti, facendo crescere la memoria fino a quando il processo si bloccava o veniva ucciso.

```text theme={null}
Error: stream-json input carried over 256M characters with no newline. Each stream-json message must be a single newline-terminated JSON line: either the producer is not newline-terminating its messages, or one message exceeded this budget.
```

L'input così lungo senza newline di solito significa che il produttore non è affatto un produttore stream-json, come un file binario o output di log semplice piped per errore. Un singolo messaggio oltre il budget fallisce lo stesso controllo.

**Cosa fare:**

* Controlla cosa è piped su stdin. Con [`--input-format stream-json`](/docs/it/cli-reference#cli-flags), ogni messaggio deve essere una singola riga JSON terminata da newline
* Per inviare testo semplice invece, rimuovi `--input-format stream-json`; `claude -p` legge un prompt di testo semplice da stdin per impostazione predefinita

<h3 id="unknown-command">
  Comando sconosciuto
</h3>

In una sessione terminale interattiva, hai inviato un nome `/` che non corrisponde a nessun comando in questa sessione, quindi Claude Code segnala il nome invece di eseguire qualcosa:

```text theme={null}
Unknown command: /hepl. Did you mean /help?
```

Claude Code suggerisce il nome di comando o alias più vicino che il menu elenca in questa sessione. Quando nulla è vicino, il messaggio termina dopo il nome. La causa è di solito una delle seguenti:

* Un errore di battitura, come `/hepl` per `/help`. [How the command menu matches what you type](/docs/it/commands#how-the-command-menu-matches-what-you-type) copre la scelta di una corrispondenza vicina prima di inviare
* Un comando che esiste ma non è disponibile in questa sessione perché un requisito non è soddisfatto, come la tua piattaforma, piano, o metodo di autenticazione. Le voci di risoluzione dei problemi per [`/web-setup`](/docs/it/web-quickstart#web-setup-shows-no-commands-match-or-unknown-command) e [`/schedule`](/docs/it/routines#schedule-returns-unknown-command) affrontano due casi comuni. Alcuni comandi rispondono con il loro messaggio quando la politica della tua organizzazione li disabilita, come [`Cloud sessions are disabled by your organization's policy`](#cloud-sessions-are-disabled-by-your-organizations-policy)
* Un comando da un [plugin](/docs/it/plugins/overview) o [server MCP](/docs/it/mcp#use-mcp-prompts-as-commands) che non è installato o connesso in questa sessione

Claude Code risponde a un nome `/` non corrispondente in questo modo solo in una sessione terminale interattiva. In ogni altra sessione, invia il prompt a Claude come messaggio normale invece, con una nota che il comando non è stato eseguito e un elenco di comandi che Claude può eseguire nella sessione. Quelle sessioni includono:

* Esecuzioni `-p`
* Applicazioni [Agent SDK](/docs/it/agent-sdk/overview)
* La scheda Code dell'[app Desktop](/docs/it/desktop)
* Il pannello chat dell'[estensione VS Code](/docs/it/vs-code)
* [Sessioni cloud](/docs/it/claude-code-on-the-web) e [routine](/docs/it/routines)

Per un comando integrato che non può essere eseguito in una di quelle sessioni, Claude Code risponde comunque che il comando non è disponibile invece di inviarlo a Claude. Prima della v2.1.274, solo le sessioni cloud e le routine inviavano un nome non corrispondente a Claude. Prima della v2.1.273, rispondevano `Unknown command` anche.

Claude Code non tratta ogni prompt che inizia con `/` come un comando. Invia il prompt a Claude come messaggio normale quando la prima parola dopo il `/` inizia con punteggiatura, come il `/--` che apre un commento Lean doc, o è un percorso come `/var/log/syslog`.

Prima della v2.1.236, se premevi `Enter` mentre il menu dei comandi elencava una corrispondenza vicina per il nome che hai digitato, Claude Code eseguiva quella corrispondenza, quindi un errore di battitura come `/hepl` eseguiva `/help` invece di produrre questo messaggio.

**Cosa fare:**

* Esegui il nome suggerito, o digita `/` seguito da parte del nome per vedere cosa è disponibile in questa sessione
* Se Claude Code segnala un comando documentato come sconosciuto, controlla la sua riga nel [riferimento dei comandi](/docs/it/commands) per il requisito che nomina

<h3 id="diff-is-too-large-for-ultrareview">
  Il diff è troppo grande per ultrareview
</h3>

Il diff tra il tuo branch e il branch base, incluse le modifiche non committate e staged, supera i limiti di dimensione per un [ultrareview](/docs/it/ultrareview), quindi `/code-review ultra` e il sottocomando `claude ultrareview` rifiutano la revisione prima che la sessione cloud si avvii. Una revisione rifiutata non usa un'esecuzione gratuita e non fattura crediti di utilizzo. Il messaggio nomina i limiti in vigore, la dimensione del tuo diff, e i file che contribuiscono il maggior numero di righe modificate. Prima della v2.1.216, il messaggio mostrava solo le statistiche di diff grezze.

```text theme={null}
Diff is too large for ultrareview: 812 files, 96,410 lines changed (limits: 500 files, 8,000 lines). Largest files: package-lock.json (41,904 lines), dist/bundle.js (18,210 lines), src/generated/api.ts (9,876 lines). Pass a closer base branch (`/code-review ultra <branch>`) to narrow the scope, or split the change.
```

La revisione di una pull request applica gli stessi limiti; quella forma del messaggio inizia `PR #<N> is too large for ultrareview` e nomina i conteggi di file e righe della PR.

**Cosa fare:**

* Passa un branch base più vicino al tuo lavoro, come `/code-review ultra develop`, così la revisione copre solo il diff rispetto a quel branch
* Dividi il cambiamento in branch più piccoli e rivedi ognuno. I file che il messaggio nomina contribuiscono il maggior numero di righe modificate, quindi inizia spostando quelli nel loro proprio branch.

<h3 id="could-not-find-merge-base-with-the-base-branch">
  Non potrebbe trovare merge-base con il branch base
</h3>

`/code-review ultra` e il sottocomando `claude ultrareview` rivedono il diff tra il tuo branch e un branch base, che ha bisogno di un commit che i due condividono. Quando `git merge-base` non ne trova nessuno, Claude Code rifiuta la revisione prima che la sessione cloud si avvii. Su un clone che Claude Code può verificare è completo, con almeno un branch, ricade a [rivedere ogni file tracciato](/docs/it/ultrareview#diff-limits-and-fallbacks) invece di rifiutare. Vedi questo rifiuto quando il branch base non può essere trovato affatto, quando Claude Code non può verificare che il tuo clone è completo, o nel raro repository dove il diff dell'intero albero non è possibile, come il formato di oggetto SHA-256.

```text theme={null}
Could not find merge-base with main. Pass the base branch explicitly (e.g. `/code-review ultra develop`) or make sure you're in a git repo with a main branch.
```

L'indizio dopo la prima frase dipende da cosa Claude Code ha osservato:

* **Non hai passato un branch base**: Claude Code ha confrontato rispetto al branch predefinito del repository e suggerisce di passare il tuo base esplicitamente, come nell'esempio sopra
* **Hai passato un branch base che era già nel tuo clone**: l'indizio legge ``Make sure <branch> exists locally or on origin (try `git fetch origin <branch>`)``
* **Hai passato un branch base che non era nel tuo clone**: Claude Code lo ha recuperato da origin prima di confrontare. L'indizio legge ``<branch> was fetched from origin but shares no history with HEAD. If another branch is your real base, pass it explicitly (`/code-review ultra <branch>`)``; quando Claude Code non può dire se il tuo clone è shallow, suggerisce `git fetch --unshallow origin` invece. Prima della v2.1.221, l'indizio suggeriva `git fetch --unshallow origin` per ogni branch base recuperato, e su un clone completo quel comando fallisce con `fatal: --unshallow on a complete repository does not make sense`.

**Cosa fare:**

* Se un altro branch è il tuo vero base, passalo esplicitamente: `/code-review ultra <branch>`
* Se il tuo clone potrebbe non avere la cronologia completa, esegui `git fetch --unshallow origin` e riesegui la revisione

<h3 id="your-checkout-has-no-branches">
  Il tuo checkout non ha branch
</h3>

Un checkout può avere commit ma nessun branch: se esegui `git init` seguito da `git fetch <url>` e `git checkout FETCH_HEAD`, ottieni un HEAD staccato senza refs. Claude Code pacchetto il tuo repository come un git bundle per caricarlo per un [ultrareview](/docs/it/ultrareview), e non può fare il bundle di un repository che non ha branch o altri refs, quindi `/code-review ultra` e il sottocomando `claude ultrareview` rifiutano la revisione prima che la sessione cloud si avvii.

```text theme={null}
Your checkout has no branches (detached HEAD only), which cloud review can't bundle. Create one first — `git checkout -b <name>` — then rerun /code-review ultra.
```

Prima della v2.1.221, Claude Code tentava di rivedere ogni file tracciato in questo checkout, e il caricamento falliva.

**Cosa fare:**

* Crea un branch al tuo commit corrente con `git checkout -b <name>`, quindi riesegui la revisione

<h3 id="no-github-account-is-connected-to-your-claude-account">
  Nessun account GitHub è connesso al tuo account Claude
</h3>

Hai eseguito `/code-review ultra <PR#>` o `claude ultrareview <PR#>`, e prima di creare la sessione cloud Claude Code chiede al server se [l'account GitHub connesso al tuo account Claude](/docs/it/ultrareview#review-a-pull-request) può raggiungere il repository della PR. Nessun account è connesso, o la connessione è scaduta, quindi il clone cloud fallirebbe e Claude Code rifiuta il lancio. Claude Code non spende un'esecuzione gratuita o fattura crediti di utilizzo per un lancio rifiutato.

```text theme={null}
Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected (or the connection expired). To fix: run /web-setup to reuse your GitHub CLI login, or connect an account at https://claude.ai/connect-github — then re-run /code-review ultra 1234 (allow a minute after connecting).
```

Quando [`/web-setup`](/docs/it/web-quickstart#connect-from-your-terminal) non è disponibile nella tua sessione, il messaggio nomina solo il link claude.ai.

**Cosa fare:**

* Esegui `/web-setup` per connettere il tuo login GitHub CLI al tuo account Claude, o connetti un account su [claude.ai/connect-github](https://claude.ai/connect-github)
* Riesegui la revisione un minuto dopo la connessione

Prima della v2.1.248, Claude Code non controllava questo prima del lancio.

<h3 id="your-connected-github-account-cant-see-the-repository">
  Il tuo account GitHub connesso non può vedere il repository
</h3>

Hai eseguito `/code-review ultra <PR#>` o `claude ultrareview <PR#>`, e [l'account GitHub connesso al tuo account Claude](/docs/it/ultrareview#review-a-pull-request) non può leggere il repository della PR, quindi il clone cloud fallirebbe e Claude Code rifiuta il lancio. Claude Code non spende un'esecuzione gratuita o fattura crediti di utilizzo per un lancio rifiutato.

```text theme={null}
Your connected GitHub account can't see <owner>/<repo> — usually the Claude GitHub app isn't installed on <owner> or wasn't granted this repo (web-connected accounts need it for private repos), or a different GitHub account is connected. To fix: run /web-setup to reuse your GitHub CLI login, or install the app at https://github.com/apps/claude/installations/new — then re-run /code-review ultra 1234.
```

Quando [`/web-setup`](/docs/it/web-quickstart#connect-from-your-terminal) non è disponibile nella tua sessione, il messaggio nomina solo l'installazione dell'app.

**Cosa fare:**

* Se il tuo CLI `gh` locale può leggere il repository, esegui `/web-setup` per connettere quel login al tuo account Claude
* Riesegui la revisione dopo il cambiamento

Prima della v2.1.248, Claude Code non controllava questo prima del lancio.

<h3 id="the-github-app-preflight-failed-transiently">
  Il preflight dell'app GitHub ha fallito transitoriamente
</h3>

Hai avviato una [sessione cloud](/docs/it/claude-code-on-the-web) da un repository locale, e due passaggi hanno fallito insieme. Claude Code non potrebbe costruire o caricare il bundle del tuo repository. Prima del caricamento, ha controllato se il servizio cloud può clonare il repository da GitHub, e piuttosto che una risposta definitiva, quel controllo è terminato in un errore che un nuovo tentativo potrebbe chiarire, come un errore di rete, un timeout, o un errore di server temporaneo. Il messaggio completo inizia con cosa ha fermato il bundle, ad esempio `Could not upload repo bundle (<error>)`, e termina con la frase di preflight:

```text theme={null}
Could not upload repo bundle (<error>). The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead
```

**Cosa fare:**

* Riesegui il comando dopo un momento. Quando il controllo GitHub passa, Claude Code può avviare la sessione da un clone GitHub, quindi il caricamento fallito non blocca più il lancio
* Se i nuovi tentativi continuano a fallire, l'inizio del messaggio nomina cosa ha fermato il caricamento. Quando quella causa è qualcosa che puoi correggere, correggila così la sessione può avviarsi dal tuo repository locale invece

Prima della v2.1.251, Claude Code terminava il messaggio con `Please set up GitHub on https://claude.ai/code` anche quando il controllo GitHub falliva solo transitoriamente, e il consiglio di configurazione non può chiarire un fallimento transitorio.

<h3 id="github-isnt-connected-to-your-claude-account">
  GitHub non è connesso al tuo account Claude
</h3>

Hai avviato una [sessione cloud](/docs/it/claude-code-on-the-web) dal tuo repository locale, ad esempio con `/autofix-pr`. Nessun account GitHub è connesso al tuo account Claude, o la connessione è scaduta, quindi Claude Code rifiuta il lancio:

```text theme={null}
GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud. Run /web-setup to connect with your GitHub CLI login, or connect on the web at https://claude.ai/connect-github
```

Quando crei una routine con [`/schedule`](/docs/it/routines), lo stesso messaggio appare come una nota di configurazione che nomina il repository; la nota non blocca la creazione della routine.

**Cosa fare:**

* Esegui `/web-setup` per connettere il tuo login GitHub CLI al tuo account Claude, o connetti un account su [claude.ai/connect-github](https://claude.ai/connect-github). Vedi [GitHub authentication options](/docs/it/claude-code-on-the-web#github-authentication-options) per come i due differiscono.
* Riesegui il comando un minuto dopo la connessione

Prima della v2.1.268, Claude Code segnalava questo come un fallimento temporaneo del controllo dell'app GitHub di Claude e suggeriva di riprovare o installare l'app; nessuno dei due connette un account GitHub.

<h3 id="single-sign-on-authorization-needed">
  Autorizzazione single sign-on necessaria
</h3>

Hai eseguito [`/install-github-app`](/docs/it/github-actions#quick-setup) e scelto un repository la cui organizzazione applica single sign-on SAML. Prima della configurazione, Claude Code controlla il tuo accesso al repository con la CLI GitHub, e GitHub ha rifiutato quel controllo perché il tuo token `gh` non è ancora autorizzato per l'organizzazione. La procedura guidata mostra l'avviso con i passaggi per autorizzare:

```text theme={null}
Single sign-on authorization needed
<owner>/<repo> belongs to an organization that enforces SAML single sign-on, and your GitHub CLI token isn't authorized for it yet.
```

**Cosa fare:**

* Riautentica il tuo login GitHub CLI con gli scope `repo` e `workflow` eseguendo `gh auth refresh -h github.com -s repo,workflow`, e autorizza l'organizzazione quando GitHub ti chiede il single sign-on
* Se ti autentichi con un token di accesso personale in `GH_TOKEN`, apri [github.com/settings/tokens](https://github.com/settings/tokens), seleziona **Configure SSO** sul token, e autorizza l'organizzazione
* Esegui di nuovo `/install-github-app`

Prima della v2.1.273, Claude Code mostrava l'avviso `Admin permissions required` per questa condizione invece.

<h3 id="failed-to-resume-the-conversation">
  Impossibile riprendere la conversazione
</h3>

Claude Code non potrebbe leggere o elaborare la trascrizione salvata per la sessione che hai selezionato dal [picker `claude --resume`](/docs/it/sessions#use-the-session-picker), quindi termina il processo piuttosto che continuare in uno stato parzialmente caricato. Il messaggio include il comando per riprovare:

```text theme={null}
Failed to resume the conversation.
Run claude --resume <session-id> to retry, or claude to start a new session.
```

Claude Code esce con codice 1 dopo aver mostrato il messaggio. Il picker `/resume` all'interno di una sessione in esecuzione segnala `Failed to resume conversation` nella conversazione invece, e la tua sessione corrente continua a funzionare. Prima della v2.1.216, una ripresa fallita dal picker `claude --resume` rimase sullo spinner `Resuming conversation…` indefinitamente invece di mostrare questo messaggio.

**Cosa fare:**

* Esegui `claude --resume <session-id>` con l'ID della sessione dal messaggio per riprovare
* Se ogni nuovo tentativo fallisce allo stesso modo, esegui `claude update` e riprendi di nuovo. Le versioni prima della v2.1.275 falliscono la ripresa quando la trascrizione salvata contiene una voce che non possono leggere.
* Se il nuovo tentativo fallisce di nuovo, esegui `claude` per avviare una nuova sessione

<h3 id="no-conversation-found-with-the-session-id">
  Nessuna conversazione trovata con l'ID della sessione
</h3>

Hai passato un ID della sessione a `claude --resume <session-id>` e nessuna trascrizione salvata lo ha corrisposto:

```text theme={null}
No conversation found with session ID: <session-id>
```

Claude Code esce con codice 1 dopo aver mostrato il messaggio. Claude Code [cerca prima il progetto corrente, quindi ogni altro progetto su questa macchina](/docs/it/sessions#resume-a-session) per l'ID. Prima della v2.1.223, la ricerca si fermava alla directory del progetto corrente e ai suoi git worktrees, quindi riprendi dalla directory in cui la sessione ha lavorato l'ultima volta.

Cause comuni:

* **ID digitato male**: per un'esecuzione non interattiva, l'ID è il campo `session_id` dell'output [`--output-format json`](/docs/it/headless#get-structured-output)
* **Trascrizione eliminata**: Claude Code rimuove le trascrizioni dopo il [periodo di conservazione](/docs/it/sessions#where-transcripts-are-stored), 30 giorni per impostazione predefinita, seguendo le [regole di pulizia della conservazione](/docs/it/claude-directory#cleaned-up-automatically)
* **Macchina diversa**: Claude Code memorizza le trascrizioni localmente, quindi riprendi la sessione sulla macchina dove è stata eseguita
* **Copie duplicate**: se hai copiato una directory di progetto sotto `~/.claude/projects` così due trascrizioni portano lo stesso ID, Claude Code segnala questo messaggio piuttosto che riprendere una copia arbitrariamente

**Cosa fare:**

* Per una sessione interattiva, apri il [picker della sessione](/docs/it/sessions#use-the-session-picker) con `claude --resume` e premi `Ctrl+A` per allargarlo a ogni progetto su questa macchina, quindi seleziona la sessione
* Le sessioni create con `claude -p` o l'[Agent SDK](/docs/it/agent-sdk/overview) non appaiono nel picker, quindi ri-controlla l'ID rispetto al `session_id` che la tua esecuzione originale ha stampato

<h3 id="cannot-switch-renderers-in-this-session">
  Impossibile cambiare renderer in questa sessione
</h3>

Quando cambi renderer, Claude Code riavvia il suo processo. Hai eseguito [`/tui`](/docs/it/fullscreen#enable-fullscreen-rendering) in una sessione che Claude Code rifiuta di riavviare, quindi non cambia e non salva nulla. Quale messaggio vedi ti dice la causa:

* `Cannot switch renderers while work is running in the background`: hai lavoro in background in esecuzione che un riavvio abbandonarebbe, come una shell in background o un subagente. Attendi che il lavoro finisca o fermalo con [`/tasks`](/docs/it/commands), quindi esegui di nuovo `/tui fullscreen` o `/tui default`
* `Cannot switch renderers in this session`: la sessione ha restrizioni che Claude Code non può passare al processo riavviato. Prima della v2.1.234, Claude Code riavviava comunque e la sessione riavviata veniva eseguita senza di esse

Nel messaggio delle restrizioni, la parte tra parentesi nomina le restrizioni che Claude Code ha trovato:

```text theme={null}
Cannot switch renderers in this session — it has restrictions a restart can't carry over (permission rules set for this session only). Nothing was changed. Running /tui fullscreen in a session started without them switches every later session too.
```

Ogni motivo che il messaggio può mostrare tra parentesi:

* `launch flags: a custom system prompt, a tool allowlist, or restricted settings`: hai avviato la sessione con un flag che Claude Code non passa di nuovo al processo riavviato. Questi flag includono [`--system-prompt`](/docs/it/cli-reference#cli-flags), `--system-prompt-file`, `--append-system-prompt-file`, un allowlist [`--tools`](/docs/it/cli-reference#cli-flags), [`--setting-sources`](/docs/it/cli-reference#cli-flags), e [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags)
* `permission rules set for this session only`: un [aggiornamento di permesso](/docs/it/hooks#permission-update-entries) da un hook o chiamante SDK ha aggiunto regole deny o ask con la destinazione `session`. Le regole di allow con scope di sessione non attivano il rifiuto. Un riavvio le elimina, e Claude Code chiede di nuovo invece
* `ask-before-running rules with no command-line form`: un aggiornamento di permesso ha aggiunto regole ask insieme alle regole che Claude Code passa di nuovo come `--allowed-tools` e `--disallowed-tools`. Nessun flag esiste per le regole ask
* `permission rules a command line cannot carry intact` e `added directories a command line cannot carry intact`: un aggiornamento di permesso ha aggiunto una regola o un percorso di directory a metà sessione. La riga di comando del processo riavviato non può portare il suo testo come lo stesso valore

**Cosa fare:**

* In una sessione avviata senza quelle restrizioni, esegui `/tui fullscreen`, o `/tui default` per cambiare di nuovo. Claude Code salva l'impostazione [`tui`](/docs/it/settings-reference#tui) lì

<h3 id="couldnt-open-claude-desktop">
  Couldn't open Claude Desktop
</h3>

Hai eseguito [`/desktop`](/docs/it/desktop#coming-from-the-cli), o il suo alias `/app`, e il comando di sistema che Claude Code usa per aprire Claude Desktop ha fallito. La sessione rimane nel terminale.

```text theme={null}
Error: Couldn't open Claude Desktop (`open` exited 1: LSOpenURLsWithRole() failed for the URL claude://resume?session=<session-id> with error -10814). Open Claude Desktop and run /desktop again.
```

**Cosa fare:**

* Apri Claude Desktop tu stesso, quindi esegui di nuovo `/desktop`
* Per leggere l'output di errore completo di quel comando, attiva il debug logging con `/debug`, esegui di nuovo `/desktop`, e controlla il log di debug

Prima della v2.1.275, il messaggio era `Failed to open Claude Desktop. Please try opening it manually.` e non diceva cosa era fallito.

<h3 id="terminal-setup-left-your-zed-keymap-unchanged">
  /terminal-setup ha lasciato la tua keymap Zed invariata
</h3>

Hai eseguito [`/terminal-setup`](/docs/it/terminal-config#enter-multiline-prompts) in Zed, e Claude Code non potrebbe completare l'aggiornamento al tuo Zed `keymap.json`, quindi ha lasciato il file come era.

Ogni messaggio nomina il percorso della tua keymap e termina con il blocco di scorciatoie da tastiera da aggiungere tu stesso:

```text theme={null}
Couldn't update your Zed keymap, so it was left unchanged.
To add the binding yourself, add this block to the keymap array in <path to keymap.json>:
{ "context": "Terminal", "bindings": { "shift-enter": ["terminal::SendText", "\u001b\r"] } }
```

La prima riga del messaggio nomina la causa:

* `Couldn't read your Zed keymap, so it was left unchanged.`: Claude Code non potrebbe leggere il file, ad esempio a causa di permessi di file
* `Your Zed keymap isn't a readable list of keybindings, so it was left unchanged.`: il file è stato letto bene ma non viene analizzato come un array di blocchi di scorciatoie da tastiera, anche con commenti `//` e virgole finali consentiti
* `Couldn't back up your Zed keymap; not modifying it.`: Claude Code non potrebbe copiare il file in un backup `.bak` accanto ad esso, quindi non ha cambiato nulla
* `Couldn't update your Zed keymap, so it was left unchanged.`: il risultato unito non ha verificato come una keymap valida che porta la scorciatoia da tastiera, quindi Claude Code lo ha scartato invece di scrivere. Un blocco di scorciatoia da tastiera con una chiave duplicata può causare questo

**Cosa fare:**

* Copia il blocco dal messaggio nell'array di livello superiore nel tuo `keymap.json` al percorso che il messaggio nomina
* Per `isn't a readable list of keybindings`, correggi l'errore di sintassi, o rendi il valore di livello superiore del file un array, quindi esegui di nuovo `/terminal-setup`

Prima della v2.1.247, `/terminal-setup` non potrebbe analizzare una keymap Zed che utilizzava commenti `//` o virgole finali, e ha sostituito l'intero file con solo la sua scorciatoia da tastiera mentre segnalava la scorciatoia da tastiera come installata. Per ripristinare una keymap che una versione precedente ha sostituito, usa il file di backup `.bak` descritto sotto [Enter multiline prompts](/docs/it/terminal-config#enter-multiline-prompts).

<h3 id="skill-usage-reports-are-not-available-on-this-connection">
  I rapporti di utilizzo delle skill non sono disponibili su questa connessione
</h3>

Hai eseguito [`/skill-doctor`](/docs/it/skills#find-unused-skills) su [Remote Control](/docs/it/remote-control), dal tuo telefono o browser. Claude Code non invia il rapporto di utilizzo delle skill su Remote Control e risponde con questo messaggio invece:

```text theme={null}
Skill usage reports are not available on this connection.
```

**Cosa fare:**

* Esegui `/skill-doctor` nel terminale sulla macchina dove la sessione è in esecuzione, o esegui `claude -p "/skill-doctor"` lì

<h3 id="custom-output-styles-cant-be-selected-over-remote-control">
  Gli stili di output personalizzati non possono essere selezionati su Remote Control
</h3>

Hai eseguito [`/output-style`](/docs/it/output-styles#change-your-output-style) dall'app mobile o web tramite [Remote Control](/docs/it/remote-control), o il comando è arrivato in un messaggio inoltrato nella sessione. Poiché tale turno potrebbe non provenire dal proprietario dell'account, Claude Code elenca e seleziona solo [stili integrati](/docs/it/output-styles#built-in-output-styles) su di esso, e aggiunge questo avviso ogni volta che il comando elenca gli stili o non riconosce il nome che hai fornito. Un nome di [stile personalizzato](/docs/it/output-styles#create-a-custom-output-style) riceve la stessa risposta di un nome che non esiste:

```text theme={null}
Custom output styles can't be selected over Remote Control or from a relayed message. Select one in the session itself, or pick a built-in style here.
```

**Cosa fare:**

* Scegli uno stile integrato, ad esempio `/output-style concise`
* Per usare uno stile personalizzato, imposta [`outputStyle`](/docs/it/settings-reference#outputstyle) nel `.claude/settings.local.json` del progetto, o esegui `/output-style <style>` al terminale della sessione stessa se ne ha uno

<h3 id="output-styles-are-saved-to-local-settings-which-this-session-doesnt-load">
  Gli stili di output vengono salvati nelle impostazioni locali che questa sessione non carica
</h3>

Hai provato a cambiare [stili di output](/docs/it/output-styles) con `/output-style <style>` o `/config outputStyle=<style>` in una sessione le cui fonti di impostazione escludono `local`. Gli esempi sono una sessione [Agent SDK](/docs/it/agent-sdk/typescript) il cui [`settingSources`](/docs/it/agent-sdk/typescript#options) lascia fuori `"local"` e una sessione CLI avviata con un valore [`--setting-sources`](/docs/it/cli-reference#cli-flags) che lascia fuori `local`. Entrambi i comandi salvano lo stile in `.claude/settings.local.json`, un file che tale sessione non legge mai, quindi Claude Code rifiuta invece di scrivere un'impostazione che non avrebbe effetto:

```text theme={null}
Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load, so the style can't be changed here.
```

**Cosa fare:**

* Aggiungi `local` alle fonti di impostazione della sessione e cambia di nuovo
* Imposta la chiave [`outputStyle`](/docs/it/settings-reference#outputstyle) in un file di impostazioni che la sessione carica, come `.claude/settings.json` nel progetto o `~/.claude/settings.json`. Nel TypeScript SDK, imposta `outputStyle` all'interno dell'oggetto `settings` inline invece; vedi [Activate an output style](/docs/it/agent-sdk/modifying-system-prompts#activate-an-output-style)

<h2 id="plugin-errors">
  Errori dei plugin
</h2>

Questi errori provengono dalla configurazione di [plugin](/docs/it/plugins/overview) e [marketplace](/docs/it/plugins/overview). Per i problemi dei plugin che non producono uno dei messaggi in questa pagina, come un URL del marketplace che non si carica o un plugin che si installa ma non appare, vedere [Risoluzione dei problemi dei plugin](/docs/it/plugins/troubleshooting).

<h3 id="plugin-eval-is-currently-in-early-access">
  plugin eval is currently in early access
</h3>

Avete eseguito [`claude plugin eval`](/docs/it/plugin-evals) o `claude plugin eval init` e ha terminato con codice 1 con uno di questi messaggi prima di fare qualsiasi cosa:

```text theme={null}
`plugin eval` is currently in early access
```

```text theme={null}
`plugin eval` is currently unavailable
```

Il primo messaggio significa che la vostra build è più vecchia della v2.1.269, la prima versione in cui il comando è generalmente disponibile. Il secondo significa che Anthropic ha disattivato il comando lato server; nulla sulla vostra macchina lo riattiva.

**Cosa fare:**

* Eseguite `claude --version`, quindi `claude update`, ed eseguite il comando di nuovo in una nuova sessione. Vedere i [requisiti per plugin evals](/docs/it/plugin-evals#requirements)
* Se vedete il secondo messaggio su una build attuale, riprovate più tardi dopo un altro `claude update`

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace is registered from an untrusted source
</h3>

Il marketplace è registrato con un nome che è [riservato per i marketplace ufficiali di Anthropic](/docs/it/plugins/marketplace-reference#marketplace-file), ma la sua fonte registrata non è un repository GitHub di `anthropics`. Claude Code ri-controlla i nomi riservati ogni volta che carica o aggiorna un marketplace, quindi il marketplace e i plugin installati da esso smettono di caricarsi. Prima della v2.1.205, il nome era controllato solo quando il marketplace veniva aggiunto, quindi una voce registrata prima che il suo nome diventasse riservato continuava a caricarsi.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

Per un marketplace la cui fonte non è un repository GitHub o un URL Git, come una directory locale, la frase centrale recita `can only be used with GitHub sources from the 'anthropics' organization` invece. `claude plugin marketplace add` esegue lo stesso controllo e rifiuta un nome riservato con `Failed to add marketplace:` seguito dalla stessa frase del nome riservato.

**Cosa fare:**

* Se il marketplace è già registrato, eseguite `claude plugin marketplace remove <name>`, quindi aggiungetelo di nuovo dal repository ufficiale `github.com/anthropics`
* Se pubblicate un marketplace di terze parti che ha utilizzato il nome prima che diventasse riservato, rinominatelo e chiedete agli utenti di aggiungerlo di nuovo dalla vostra fonte
* Vedere l'elenco dei nomi riservati in [Marketplace schema](/docs/it/plugins/marketplace-reference#marketplace-file)

<h3 id="marketplace-name-is-another-spelling-of-a-reserved-name">
  Marketplace name is another spelling of a reserved name
</h3>

Il nome del marketplace non è di per sé un nome riservato, ma Claude Code lo tratta come un'altra ortografia di uno. [Reserved names](/docs/it/plugins/marketplace-reference#reserved-name-spellings) elenca quali ortografie contano come un nome riservato. Claude Code rifiuta tale nome quando aggiungete il marketplace:

```text theme={null}
Failed to add marketplace: "claude.code.plugins" is another spelling of "claude-code-plugins", a reserved marketplace name.
```

Quando un marketplace è già registrato con tale nome, la sua voce smette di caricarsi, e `/plugin`, `claude plugin install`, e `claude plugin update` avvertono:

```text wrap theme={null}
known_marketplaces.json has an entry named "claude.code.plugins", another spelling of the reserved marketplace name "claude-code-plugins", so it is ignored. Remove it with: claude plugin marketplace remove claude.code.plugins
```

Quando il nome avrebbe bisogno di quoting della shell, il rifiuto al momento dell'aggiunta recita `This marketplace's name is another spelling of "<reserved>", a reserved marketplace name. It is not exactly the reserved name it appears to be.`

**Cosa fare:**

* Rinominate il marketplace con un nome che non ortografia un nome riservato e aggiungetelo di nuovo
* Per l'avvertimento di voce ignorata, eseguite il comando `claude plugin marketplace remove` che fornisce, o rimuovete la voce da `~/.claude/plugins/known_marketplaces.json`

<h3 id="marketplace-is-already-added-from-a-different-source">
  Marketplace is already added from a different source
</h3>

Avete confermato l'aggiunta di un marketplace tramite [`/plugin install <plugin> --marketplace <source>`](/docs/it/plugins/install#add-a-marketplace-and-install-in-one-command), e il catalogo che Claude Code ha recuperato da quella fonte nomina se stesso con lo stesso nome di un marketplace che avete già aggiunto da una fonte diversa. Claude Code mantiene il marketplace esistente invece di sostituirlo, e il plugin non viene installato.

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

**Cosa fare:**

* Se il marketplace che avete già aggiunto è quello che desiderate, installate da esso per nome: `/plugin install <plugin>@<name>`
* Per passare alla nuova fonte, eseguite `/plugin marketplace remove <name>`, quindi riprovate l'installazione

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

Un hook del plugin, [monitor](/docs/it/plugins/components#monitors), o un comando MCP [`headersHelper`](/docs/it/mcp#use-dynamic-headers-for-custom-authentication) fa riferimento a un'[opzione del plugin](/docs/it/plugins/manifest-reference#user-configuration) `${user_config.KEY}`, e la stringa sostituita verrebbe passata a una shell. Un valore configurato contenente `$(...)`, backtick, o `;` verrebbe eseguito come codice lì, quindi Claude Code rifiuta di avviare il componente invece di sostituire il valore. Il controllo viene eseguito sul modello di comando, quindi l'errore appare anche quando nessun valore è ancora configurato. Prima della v2.1.207, il valore veniva sostituito nel comando della shell.

La formulazione dipende da quale superficie ha fatto riferimento all'opzione. Un hook in forma shell segnala:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Un monitor segnala:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

Un MCP `headersHelper` segnala:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Cosa fare:**

* Per un hook, aggiungete un array `args` in modo che venga eseguito in [forma exec](/docs/it/hooks#exec-form-and-shell-form), dove ogni `${user_config.KEY}` diventa un argomento senza shell in mezzo. Oppure eliminate il riferimento e leggete la variabile di ambiente `$CLAUDE_PLUGIN_OPTION_<KEY>` all'interno dello script
* Per un monitor, eliminate il riferimento e fate in modo che lo script del monitor legga il valore da un file di configurazione
* Per un `headersHelper`, spostate `${user_config.KEY}` nel campo `headers` del server, che non viene analizzato dalla shell, oppure leggete il valore all'interno dello script helper

<h3 id="plugin-archive-integrity-check-failed">
  Plugin archive integrity check failed
</h3>

La voce del marketplace del plugin utilizza una [fonte `archive`](/docs/it/plugins/marketplace-reference#archive-plugin-source) con un pin `sha256`, e il digest del file scaricato non corrisponde al pin. Claude Code rifiuta l'installazione, quindi nulla cambia nella cache del plugin. La mancata corrispondenza ha tre possibili cause:

* Il file all'URL è cambiato dopo che l'autore ha calcolato il pin
* L'autore ha inserito il digest sbagliato nella voce del marketplace
* L'URL serve un file diverso da quello che l'autore ha fissato

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

**Cosa fare:**

* Se pubblicate il plugin, ricalcolate il digest del file esatto che l'URL serve, ad esempio con `shasum -a 256 my-plugin.zip`, o `Get-FileHash -Algorithm SHA256 my-plugin.zip` in PowerShell, e aggiornate `sha256` nella voce del marketplace
* Se installate il plugin, eseguite `/plugin marketplace update <name>` per aggiornare il catalogo nel caso in cui la voce sia stata corretta, quindi riprovate l'installazione
* Se i digest continuano a non corrispondere dopo un aggiornamento, chiedete al proprietario del marketplace quale file hanno fissato prima di installare

<h3 id="path-escapes-plugin-directory">
  Path escapes plugin directory
</h3>

Un percorso del componente del plugin, dichiarato nel `plugin.json` del plugin o nella sua [voce del marketplace](/docs/it/plugins/marketplace-reference#plugin-entries), si risolve al di fuori della directory del plugin stesso. Claude Code elimina quel percorso e carica il resto del plugin. Il nome del componente nel messaggio, come `commands` o `hooks`, nomina il campo che ha dichiarato il percorso.

```text theme={null}
commands path escapes plugin directory: ./../shared.md
```

Nell'output del comando `claude plugin`, lo stesso errore recita `Path escapes plugin directory: ./../shared.md (commands)`.

Claude Code rifiuta sia un percorso che punta al di fuori del plugin come scritto, come `../shared-utils`, sia un symlink che porta al di fuori del plugin e non è uno che le [regole dei symlink del marketplace](/docs/it/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks) consentono. Per un symlink, il messaggio dice anche dove il percorso si risolve:

```text theme={null}
commands path escapes plugin directory: ./commands/deploy.md — it resolves to /home/user/shared/deploy.md, outside the plugin directory
```

Su macOS e Linux, Claude Code rifiuta anche un percorso del componente che contiene una barra rovesciata in qualsiasi punto, anche quando il percorso rimane all'interno del plugin. Un plugin i cui percorsi dei componenti utilizzano separatori in stile Windows si carica su Windows e attiva questo rifiuto sulle altre piattaforme:

```text theme={null}
commands path escapes plugin directory: ./commands\deploy.md — its path contains a backslash, which is not resolved reliably on this platform
```

Prima della v2.1.251, Claude Code caricava un percorso `commands` dichiarato in una voce del marketplace anche quando puntava al di fuori della directory del plugin. Claude Code ha già rifiutato i percorsi dichiarati in `plugin.json` e gli altri percorsi dei componenti in una voce del marketplace.

Prima della v2.1.257, il controllo guardava solo l'ortografia del percorso, non dove un symlink porta.

**Cosa fare:**

* Spostate il file referenziato all'interno della directory del plugin e puntate il percorso ad esso con un percorso relativo `./`
* Se il percorso è un symlink a un file al di fuori del plugin, sostituite il symlink con una copia del file
* Se il messaggio dice che il percorso contiene una barra rovesciata, scrivete il percorso con barre in avanti, ad esempio `./commands/deploy.md`
* Per condividere file con altri plugin nello stesso marketplace, collegateli con un symlink all'interno della directory del plugin, seguendo le [regole dei symlink](/docs/it/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)

<h3 id="path-could-not-be-checked">
  Path could not be checked
</h3>

Claude Code ha chiesto al sistema operativo se un percorso del plugin esiste e ha ricevuto un errore diverso da "non trovato", quindi non carica ciò che il percorso nomina. Quanto del plugin si carica dipende da quale percorso ha fallito:

* Una delle [cartelle dei componenti predefinite](/docs/it/plugins/manifest-reference#standard-layout) di un plugin, come la cartella `skills/`, il file `monitors/monitors.json`, o uno [`SKILL.md` alla radice del plugin](/docs/it/plugins/components#skills): gli altri componenti del plugin si caricano comunque
* La directory del plugin stesso: nulla da quel plugin si carica

Non vedete questo errore per un percorso che non esiste affatto. In `/plugin`, l'errore appare sotto il plugin e nomina il percorso e il codice che il sistema operativo ha restituito:

```text theme={null}
skills path could not be checked: /home/user/my-plugin/skills (ELOOP)
```

In `claude plugin list`, lo stesso errore recita `Path not found: /home/user/my-plugin/skills (skills, ELOOP)`.

Le cause che producono questo errore includono:

* `ELOOP`: un symlink nel percorso punta a se stesso o forma un ciclo
* `EIO` o `ESTALE`: il percorso è su un mount di rete che è rotto o stantio
* `EACCES`: una delle directory sopra il percorso nega il permesso di attraversarla

**Cosa fare:**

* Sostituite un symlink che punta a se stesso con una cartella reale, oppure eliminatelo
* Se il percorso è su un mount di rete, rimontate la condivisione
* Se il codice è `EACCES`, ripristinate il vostro permesso di esecuzione sulle directory sopra il percorso
* Eseguite `/reload-plugins` dopo aver corretto il percorso, o riavviate Claude Code, per caricare il plugin o il componente

Prima della v2.1.265, Claude Code trattava una cartella dei componenti predefinita che non poteva controllare come assente e caricava il plugin senza quel componente, senza errore.

<h3 id="marketplace-entry-path-does-not-stay-inside-the-marketplace-directory">
  Marketplace entry path does not stay inside the marketplace directory
</h3>

La [voce del marketplace](/docs/it/plugins/marketplace-reference#plugin-entries) del plugin dichiara un percorso di origine che Claude Code non può risolvere a una posizione all'interno della directory del marketplace stesso, quindi il plugin non si installa o non si carica. Il rifiuto copre:

* Un percorso di voce che è assoluto, esce dal marketplace con `..`, o è scritto come un percorso di rete
* Su macOS e Linux, un percorso di voce che contiene una barra rovesciata in qualsiasi punto dopo il `./` iniziale
* Una voce in un marketplace recuperato da una fonte remota, come git o un URL, che raggiunge il suo target attraverso un symlink che si risolve al di fuori della directory del marketplace
* Una voce relativa in un marketplace aggiunto da un URL diretto al suo `marketplace.json`: Claude Code scarica solo quel file, quindi nessun file di plugin locale esiste per il percorso da nominare. Vedere [Plugins with relative paths fail in URL-based marketplaces](/docs/it/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)

`claude plugin install` segnala il rifiuto così:

```text theme={null}
Cannot install my-plugin@my-marketplace: its marketplace entry path does not stay inside the marketplace directory (an absolute, climbing, network-shaped, backslash-containing or link-traversing entry, an entry of a fetched marketplace that resolves or opens outside its tree — or a relative entry in a url-catalog marketplace, which has no local directory)
```

Quando la voce di un plugin già installato fallisce lo stesso controllo, `claude plugin list` mostra il plugin come `failed to load` con:

```text theme={null}
Plugin source path refused: ./my-plugin does not stay inside its marketplace directory. Check that the marketplace entry has a plain relative path.
```

**Cosa fare:**

* Se mantenete il marketplace, scrivete il `source` della voce come un percorso relativo semplice con barre in avanti, come `./plugins/my-plugin`, e mantenete qualsiasi symlink che attraversa puntato all'interno della directory del marketplace
* Se avete aggiunto il marketplace da un URL diretto, le voci relative non possono risolversi. Chiedete all'autore del marketplace di utilizzare [un'altra fonte di plugin](/docs/it/plugins/marketplace-reference#plugin-sources), o aggiungete il marketplace dal suo repository git invece

<h3 id="failed-to-load-marketplace-configuration">
  Failed to load marketplace configuration
</h3>

Claude Code mantiene i marketplace dei plugin che avete aggiunto in un file di registro in `~/.claude/plugins/known_marketplaces.json`. Un comando di plugin che ha bisogno del registro, come `claude plugin install`, fallisce con uno di due messaggi quando Claude Code non può utilizzare il file:

* `Failed to load marketplace configuration`: il file non è JSON valido, o non può essere letto. Un file vuoto fallisce in questo modo.
* `Marketplace configuration file is corrupted`: il file è JSON valido ma i suoi contenuti non corrispondono allo schema del registro.

Un file mancante non è un fallimento: Claude Code lo tratta come un registro senza marketplace.

Con un file vuoto, `claude plugin install` segnala:

```text theme={null}
✘ Failed to install plugin "my-plugin": Failed to load marketplace configuration: JSON Parse error: Unexpected EOF
```

Prima della v2.1.246, `claude plugin install` non segnalava questo fallimento.

**Cosa fare:**

* Aprite `~/.claude/plugins/known_marketplaces.json` e riparate il JSON, o correggete le voci che il messaggio nomina come non corrispondenti allo schema del registro
* Se non potete ripararla, eliminate il file o sostituite i suoi contenuti con `{}`, quindi aggiungete di nuovo ogni marketplace con `claude plugin marketplace add <source>`. Claude Code ri-registra i marketplace che le vostre impostazioni utente o gestite dichiarano in [`extraKnownMarketplaces`](/docs/it/settings-reference#extraknownmarketplaces) la prossima volta che lo avviate in una cartella che avete considerato attendibile.

<h3 id="plugin-is-required-by-your-organization">
  Plugin is required by your organization
</h3>

Avete eseguito `claude plugin disable`, o utilizzato la scheda **Installed** di `/plugin`, per disattivare un [plugin sincronizzato da claude.ai](/docs/it/plugins/loading#synced-plugins) che la vostra organizzazione contrassegna come obbligatorio:

```text theme={null}
Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.
```

Claude Code non salva nulla e il plugin rimane abilitato.

Quando tentate di disattivare un plugin da cui dipende un plugin obbligatorio, Claude Code rifiuta allo stesso modo, con un messaggio che nomina il plugin obbligatorio che ne ha bisogno.

**Cosa fare:**

* Chiedete a un amministratore della vostra organizzazione claude.ai di cambiare lo stato obbligatorio del plugin su claude.ai

<h2 id="tool-errors">
  Errori degli strumenti
</h2>

Questi errori provengono dagli strumenti integrati di Claude. Claude corregge la maggior parte degli errori degli strumenti autonomamente. Quando è necessario un cambiamento da parte vostra, l'elenco **Cosa fare** di quell'errore specifica cosa cambiare.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent would be spawned with zero tools
</h3>

Ogni voce nell'elenco [`tools` del subagent](/docs/it/sub-agents#supported-frontmatter-fields) non ha corrisposto a uno strumento utilizzabile, quindi Claude Code ha rifiutato di avviare il subagent: senza strumenti, non poteva agire. Il messaggio raggruppa le vostre voci in base a cosa è andato storto:

* **Unrecognized**: la voce non corrisponde a nessun nome di strumento, di solito un errore di battitura come `Grpe` per `Grep`.
* **Not available to subagents**: la voce nomina uno strumento reale che [i subagent non possono usare](/docs/it/sub-agents#available-tools). I subagent in background mantengono un set di strumenti integrati più piccolo, quindi una voce che solo un subagent in foreground può usare finisce qui quando il subagent verrebbe eseguito in background, che è l'impostazione predefinita. Se elencate `Agent`, il messaggio lo segnala nel gruppo successivo.
* **Matched no tools in this session**: la voce è valida ma nessuno strumento nella sessione corrente la corrisponde in questo momento, come `mcp__github__*` senza server MCP GitHub connesso, o `Agent` per un subagent al [limite di profondità](/docs/it/sub-agents#let-subagents-spawn-their-own-subagents).

Omettere il campo `tools` non attiva mai questo rifiuto. Se lasciate l'elenco `tools` vuoto, o `disallowedTools` rimuove ogni voce in esso, Claude Code salta anche il rifiuto e avvia il subagent senza strumenti.

Prima della v2.1.208, il subagent veniva avviato senza strumenti e poteva restituire un risultato vuoto o confuso.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**Cosa fare:**

* Correggete ogni voce che l'errore nomina rispetto agli [strumenti disponibili per i subagent](/docs/it/sub-agents#available-tools)
* Rimuovete le voci per gli strumenti che la sessione non ha, come gli strumenti MCP da un server che non è connesso
* Per uno strumento che [i subagent in background eliminano](/docs/it/sub-agents#available-tools), come `CronCreate`, rimuovete la voce. Per mantenere lo strumento, [disattivate la fork mode](/docs/it/sub-agents#turn-fork-mode-on-or-off) e chiedete a Claude di eseguire il subagent in foreground
* Eliminate il campo `tools` invece di elencare gli strumenti per dare al subagent ogni [strumento disponibile per i subagent](/docs/it/sub-agents#available-tools)
* Per un elenco `tools` che contiene solo `Agent`, aumentate il [limite di profondità](/docs/it/sub-agents#let-subagents-spawn-their-own-subagents) o date all'agent almeno uno strumento aggiuntivo: Claude Code trattiene `Agent` a quel limite, quindi un elenco con nient'altro in esso si risolve in nessuno strumento

<h3 id="file-is-covered-by-a-read-deny-rule">
  File is covered by a Read deny rule
</h3>

Lo strumento Edit o Write è stato chiamato su un percorso corrispondente a una [regola di negazione `Read`](/docs/it/permissions#read-and-edit), inclusa la creazione di un nuovo file in quel percorso. Entrambi gli strumenti cambiano il contenuto che Claude deve essere in grado di leggere di nuovo, quindi Claude Code rifiuta la chiamata prima di qualsiasi accesso ai file. NotebookEdit non è coperto dalle regole di negazione `Read`. Prima della v2.1.228, la regola bloccava solo lo strumento Edit, e prima della v2.1.208, solo una regola di negazione `Edit` bloccava le modifiche.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

Quando Claude Code rifiuta lo strumento Write, il messaggio termina con `and cannot be written` invece.

**Cosa fare:**

* Se Claude dovrebbe essere in grado di cambiare il file, rimuovete o restringete la regola di negazione `Read` in `/permissions` o nelle [impostazioni](/docs/it/settings-reference#permission-settings)
* Se il file deve rimanere intatto, mantenete la regola e aggiungete una regola di negazione `Edit` per lo stesso percorso per bloccare anche lo strumento NotebookEdit

<h3 id="subagent-type-is-required">
  subagent\_type is required
</h3>

```text theme={null}
subagent_type is required: the general-purpose agent is not available in this session. Available agents: ...
```

Claude ha chiamato lo [strumento Agent](/docs/it/tools-reference#agent-tool-behavior) senza un `subagent_type`, e questa sessione non ha un [subagent per uso generale](/docs/it/sub-agents#built-in-subagents) su cui fare affidamento. Questo è il caso in due configurazioni:

* [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/it/env-vars) è impostato in modalità non interattiva, che rimuove ogni subagent integrato
* L'agent del thread principale della sessione ha una [lista di consentiti `tools: Agent(...)`](/docs/it/sub-agents#restrict-which-subagents-can-be-spawned) che esclude `general-purpose`

**Cosa fare:**

* Di solito nulla: il messaggio elenca i subagent che la sessione ha, quindi Claude può riprovare con uno di essi
* Se Claude continua a fallire, aggiungete `general-purpose` alla lista di consentiti `tools: Agent(...)`, o disattivate `CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS`

Prima della v2.1.235, la stessa chiamata falliva con `Agent type 'general-purpose' not found`.

<h3 id="memory-index-is-over-its-read-limit">
  Memory index is over its read limit
</h3>

Claude ha scritto nell'indice di [memoria automatica](/docs/it/memory#auto-memory) `MEMORY.md` e lo ha lasciato oltre uno dei suoi limiti di lettura: 200 righe o 25KB. La scrittura è riuscita, ma solo le prime 200 righe o 25KB, a seconda di quale viene raggiunto per primo, vengono caricate all'inizio di una sessione, quindi tutto ciò che supera il limite viene eliminato ogni volta che l'indice viene letto. Prima della v2.1.210, un indice oltre il limite veniva silenziosamente troncato al caricamento successivo senza segnale al momento della scrittura.

```text theme={null}
Error: this write left the memory index at MEMORY.md at 214 lines, over its 200-line read limit. The write succeeded, but everything past the limit is silently dropped each time the index is loaded — entries at the end are already invisible to readers. Rewrite it to under 140 lines now: keep one line per entry, move detail into topic files, and merge or drop stale entries.
```

Solo il contenuto che viene caricato conta verso i limiti. Il frontmatter YAML e i commenti HTML a livello di blocco vengono rimossi prima che l'indice venga caricato, quindi sono esclusi dalla misurazione. Prima della v2.1.211, Claude Code misurava il file grezzo, e il frontmatter o i commenti potevano attivare questo errore anche quando il contenuto caricato si adattava.

Claude Code consegna l'errore a Claude dopo la scrittura piuttosto che stamparlo come banner nel vostro terminale, quindi potreste notarlo solo nella trascrizione.

Quando la scrittura di Claude avvicina il file a un limite senza superarlo, Claude Code restituisce un promemoria più mite per compattare l'indice invece di questo errore.

**Cosa fare:**

* Lasciate che Claude riscrivi `MEMORY.md`, o chiedetegli di farlo: mantenete una riga per voce, spostate i dettagli in file di argomenti e unite o eliminate le voci obsolete
* Per ridurre l'indice voi stessi, vedete [Audit and edit your memory](/docs/it/memory#audit-and-edit-your-memory)

<h3 id="pkill-pattern-matches-the-claude-code-process">
  pkill pattern matches the Claude Code process
</h3>

Un comando `pkill` in una chiamata dello strumento Bash ha usato un pattern, tipicamente con `-f`, che corrisponde al processo Claude Code stesso, quindi Claude Code rifiuta il comando invece di permettergli di terminare la sessione. Claude Code testa il pattern con `pgrep` prima di eseguire `pkill` e rifiuta quando il suo ID di processo è nel risultato. Il controllo viene eseguito solo su Linux; su macOS, `pkill` viene eseguito senza modifiche. Prima della v2.1.214, il comando veniva eseguito e un pattern corrispondente terminava la sessione Claude Code a metà turno.

```text theme={null}
pkill: refusing to run — this pattern matches the Claude CLI process (PID 12345). Narrow the pattern, or target your own children with `pkill -P $$ ...`.
```

Il rifiuto appare nel risultato dello strumento Bash piuttosto che come banner nel vostro terminale, e Claude di solito regola il comando autonomamente.

**Cosa fare:**

* Restringete il pattern in modo che corrisponda solo al processo previsto, ad esempio il percorso completo del binario di destinazione piuttosto che una breve sottostringa
* Per arrestare i processi avviati dalla shell corrente, usate `pkill -P $$` con il pattern, che limita la corrispondenza ai processi figli della shell stessa

<h3 id="failed-to-write-to-a-teammate-inbox">
  Failed to write to a teammate's inbox
</h3>

Claude Code non ha potuto scrivere un messaggio nella casella di posta di un compagno di squadra sotto `~/.claude/teams/{team-name}/inboxes/`, quindi il destinatario non ha ricevuto nulla. La scrittura fallisce quando Claude Code non può creare o aggiornare il file, ad esempio perché il disco è pieno, la directory non è scrivibile, o un altro agent tiene il blocco della casella di posta troppo a lungo. Prima della v2.1.224, Claude Code segnalava il messaggio come inviato anche quando la scrittura falliva.

L'errore appare nel risultato dello strumento dell'agent mittente piuttosto che come banner nel vostro terminale, e il suo testo dice a Claude di riprovare:

```text theme={null}
Failed to write to researcher's inbox — nothing was sent. Try again, or message the lead.
```

I messaggi del protocollo strutturato del [team di agent](/docs/it/agent-teams) falliscono allo stesso modo, e l'errore nomina il messaggio non consegnato: quando Claude Code non può scrivere un'approvazione del piano, un rifiuto del piano, una richiesta di arresto o un rifiuto di arresto, l'errore recita `Failed to write the <message> to <name>'s inbox — nothing was sent`. L'`approvazione del piano` in quell'elenco è la decisione del lead che approva il piano di un compagno di squadra; la presentazione del piano del compagno di squadra è il messaggio separato `richiesta di approvazione del piano`. Quel messaggio e altri due messaggi del protocollo portano il loro testo di messaggio e conseguenza:

* `Failed to write the plan approval request to the lead's inbox — plan not submitted; try again`: il piano del compagno di squadra non ha mai raggiunto il lead, e il compagno di squadra rimane in modalità piano fino a quando una nuova presentazione non riesce
* `The permission request could not be delivered to the team lead (mailbox write failed)`: la richiesta di autorizzazione del compagno di squadra non ha mai raggiunto il lead, quindi nessuno ha approvato la chiamata dello strumento
* `The confirmation could not be written to team-lead's inbox.`: l'approvazione dell'arresto stesso ha avuto effetto e il compagno di squadra esce; solo la conferma al lead manca

Quando voi stessi inviate un messaggio a un compagno di squadra, digitando `@name` seguito dal messaggio nella sessione del lead, lo stesso errore appare come notifica, `Couldn't write to @name's inbox — message not sent. Try again.`, e Claude Code mantiene il vostro testo nella casella del prompt in modo che possiate inviarlo di nuovo.

**Cosa fare:**

* Chiedete al mittente di inviare di nuovo il messaggio; la contesa per il blocco della casella di posta è transitoria e si risolve al nuovo tentativo
* Controllate lo spazio libero su disco e verificate che `~/.claude/teams` e i file sotto di esso siano scrivibili dal vostro utente

<h3 id="teammate-agent-definition-not-restored">
  Teammate's agent definition was not restored
</h3>

Claude ha inviato un messaggio a un compagno di squadra del [team di agent](/docs/it/agent-teams) arrestato, e Claude Code lo ha riportato in vita senza riapplicare la [definizione del subagent](/docs/it/agent-teams#use-subagent-definitions-for-teammates) da cui era stato generato, perché il suo file di definizione proveniva da una cartella senza fiducia salvata. L'avviso segue il rapporto di ripresa nel risultato dello strumento dell'agent mittente:

```text wrap theme={null}
Its agent definition was not restored: the folder its definition file came from is not trusted (source: projectSettings), so the teammate is running with the team-essential tools and no custom instructions. To restore it, the user needs to run Claude Code in that folder once and accept the trust dialog (the --debug log names the folder); do not change trust settings on the user's behalf.
```

Il controllo si applica a una definizione nella directory `.claude/agents/` del progetto o di una directory `--add-dir`, e accettare la finestra di dialogo di fiducia per una cartella padre non la soddisfa.

**Cosa fare:**

* Eseguite `claude` nella cartella che il [log di debug](/docs/it/debug-your-config) nomina e accettate la finestra di dialogo di fiducia. La definizione viene riapplicata la prossima volta che Claude Code riporta in vita il compagno di squadra; non è necessario riavviare la sessione del lead
* Oppure impostate la voce `hasTrustDialogAccepted` su `true` in `~/.claude.json`, usando la chiave esatta `projects["<path>"]` che il log di debug stampa

<h3 id="message-too-large-for-cross-session-delivery">
  Message too large for cross-session delivery
</h3>

Il [messaggio tra sessioni](/docs/it/cross-session-messaging) di Claude a un'altra delle vostre sessioni su questa macchina era troppo lungo per essere inviato. Claude Code lo ha rifiutato e la sessione ricevente non ha ricevuto nulla. Il rifiuto appare nel risultato dello strumento della sessione mittente, non come banner nel vostro terminale. Nomina entrambe le dimensioni e come fare in modo che il messaggio si adatti:

```text wrap theme={null}
Failed to send to api-worker: Message too large for cross-session delivery: the serialized message is 1,203,844 characters and the limit is 1,048,576. Shorten the message text — put bulk content in a file the recipient can read rather than in the message — or split it into smaller messages.
```

L'invio dello stesso testo fallisce allo stesso modo.

**Cosa fare:**

* Chiedete a Claude di riassumere il messaggio, o di mettere il contenuto in massa in un file e inviare il percorso del file
* Chiedete a Claude di dividere il contenuto in diversi messaggi più brevi

Prima della v2.1.235, Claude Code segnalava un messaggio di dimensioni eccessive come inviato. La sessione ricevente lo eliminava senza leggerlo.

<h3 id="too-many-messages-to-this-session-just-now">
  Too many messages to this session just now
</h3>

Claude ha inviato una raffica rapida di [messaggi tra sessioni](/docs/it/cross-session-messaging) a una delle vostre sessioni su questa macchina, e la raffica ha raggiunto ciò che quella casella di posta della sessione accetta. Claude Code ha rifiutato l'invio successivo e la sessione ricevente non ha ricevuto nulla da esso. Il rifiuto appare nel risultato dello strumento della sessione mittente, non come banner nel vostro terminale:

```text wrap theme={null}
Failed to send to api-worker: Too many messages to this session just now: 30 were sent recently and more would be dropped by its rate limit, so this one was not sent. Batch what remains into one message, or wait a little before sending more.
```

**Cosa fare:**

* Di solito nulla: Claude raggruppa il contenuto rimanente in un messaggio, o aspetta prima di inviare di più
* Se avete voi stessi richiesto la raffica, chiedete a Claude di combinare ciò che rimane in un singolo messaggio

Prima della v2.1.236, Claude Code segnalava questi invii come inviati. La sessione ricevente li eliminava senza leggerli.

<h3 id="refusing-to-send-a-cross-session-message">
  Refusing to send a cross-session message
</h3>

Prima che Claude Code scriva un [messaggio tra sessioni](/docs/it/cross-session-messaging) a un'altra delle vostre sessioni su questa macchina, verifica che la socket della casella di posta della sessione di destinazione sia l'endpoint a cui il messaggio era indirizzato. Quando un controllo fallisce, Claude Code rifiuta l'invio nella sessione mittente e la sessione di destinazione non riceve nulla. Per un messaggio che Claude invia, il rifiuto appare nel risultato dello strumento della sessione mittente:

```text theme={null}
Failed to send to api-worker: Refusing to send: reply target is a symlink
```

Il testo dopo `Refusing to send:` nomina il controllo che ha fallito:

* `reply target is a symlink`: un collegamento simbolico si trova nel percorso della socket della sessione di destinazione. Claude Code non consegna attraverso di esso, perché un collegamento lì potrebbe reindirizzare il messaggio a un endpoint che la sessione di destinazione non ha creato.
* `cannot vet reply target`: Claude Code non ha potuto ispezionare il percorso di destinazione affatto, ad esempio perché la lettura è fallita con un errore di autorizzazione.
* `connected endpoint is not the expected process`: il processo che tiene la socket non è la sessione a cui il messaggio era indirizzato, quindi l'indirizzo è obsoleto o un altro processo ha sostituito la socket.
* `connected endpoint identity could not be read`: Claude Code si è connesso ma non ha potuto leggere quale processo tiene l'altro capo, quindi non ha potuto confermare la destinazione. Questo può essere transitorio.
* `connected endpoint is not owned by this user`: il processo che tiene la socket viene eseguito con un account utente diverso, quindi non è una delle vostre sessioni.
* `connected endpoint owner could not be read`: Claude Code si è connesso ma non ha potuto leggere quale account utente possiede l'altro capo, quindi non ha potuto confermare che l'endpoint è vostro.
* `connected endpoint is a different process with the expected pid`: l'ID del processo corrisponde a quello a cui il messaggio era indirizzato, ma Claude Code non ha potuto confermare che è lo stesso processo. Di solito quella sessione è uscita e il sistema operativo ha riutilizzato il suo ID di processo, quindi l'indirizzo è obsoleto.

**Cosa fare:**

* Di solito nulla: i controlli impediscono a un messaggio di raggiungere un endpoint diverso dalla sessione a cui era indirizzato, e nulla è stato inviato
* Chiedete a Claude di elencare di nuovo le vostre sessioni e di inviare di nuovo; un rifiuto causato da un indirizzo obsoleto si risolve una volta che Claude invia a quello corrente
* Se `reply target is a symlink` si ripete per una sessione, controllate cosa ha creato un collegamento nel percorso della socket di quella sessione, mostrato nel suo `/status` sotto `Peer address`
* Per `connected endpoint identity could not be read`, inviate di nuovo; la condizione può essere transitoria
* Se `connected endpoint is not owned by this user` appare su una macchina condivisa, la sessione a quell'indirizzo viene eseguita con l'account di un altro utente, quindi Claude non può inviarle messaggi dal vostro

Prima della v2.1.248, Claude Code non controllava l'utente proprietario dell'endpoint o l'ora di inizio del processo, quindi i rifiuti che nominano quei controlli non appaiono nelle versioni precedenti.

<h3 id="refusing-after-a-symlink-changed">
  Refusing to read, write, or search a path
</h3>

Claude Code controlla le [regole di autorizzazione](/docs/it/permissions#read-and-edit) di un percorso di file, quindi conferma di nuovo quella risoluzione quando lo strumento apre il file o avvia la ricerca. Quando non può confermare che il percorso conduce ancora alla posizione che il controllo ha approvato, Claude Code rifiuta l'operazione invece di seguirla. Il rifiuto appare nel risultato dello strumento:

```text wrap theme={null}
Refusing to read /path/to/file: its symlink resolution changed after permission was checked (a link on the way now leads somewhere the check did not see). If a link in the working directory is being rewritten concurrently, stop that and retry.
```

Ogni rifiuto nomina il suo motivo:

* `its symlink resolution changed after permission was checked`: un collegamento simbolico lungo il percorso, o in una radice di ricerca Grep o Glob, è stato sostituito tra il controllo di autorizzazione e l'operazione. In un rifiuto di lettura, la frase tra parentesi nomina quale confronto è fallito.
* `its parent-directory symlink resolution changed after permission was checked`: una directory attraverso cui passa il percorso di scrittura non si risolve più nella posizione approvata
* `it is a symbolic link. Write to the link's target path instead`: un collegamento simbolico si trova nella posizione di scrittura approvata stessa, ad esempio un `CLAUDE.md` che è un collegamento simbolico a `AGENTS.md`; il messaggio dirige Claude al target del collegamento
* `Refusing to write through symlink: <path>. Resolve the symlink and pass the real target path explicitly.`: la stessa condizione catturata quando un altro writer apre il file, come una scrittura a un `.mcp.json` collegato simbolicamente
* `Refusing to write into symlinked directory: <path>`: la directory che contiene il file è essa stessa un collegamento simbolico, ad esempio la directory `.claude/` di un progetto collegata a un'altra posizione
* `a path one of its Read deny rules is written through changed while the search was being prepared. Retry.`: una regola di negazione `Read` per la ricerca nomina un percorso che passa attraverso un collegamento simbolico, e quel collegamento è cambiato mentre Claude Code stava preparando la ricerca
* `it could not be opened (EACCES) — it is unreadable, or is being replaced concurrently.`: la radice di ricerca esiste ma non ha potuto essere aperta; il codice tra parentesi è l'errore del sistema operativo
* `its permission check expired before it ran (too many concurrent file operations). Retry.`: Claude Code ha eliminato il record di approvazione in molte operazioni di file simultanee prima che lo strumento lo usasse; riprovare esegue un controllo di autorizzazione fresco
* `ripgrep was found only by name on PATH, and a search outside the working directory cannot apply your Read deny rules in that configuration`: Claude Code non ha potuto risolvere il binario `rg` a un percorso assoluto, quindi rifiuta le ricerche al di fuori della directory di lavoro piuttosto che eseguirne una che le vostre regole di negazione non coprono

**Cosa fare:**

* Di solito nulla: il rifiuto raggiunge Claude come risultato dello strumento, e l'operazione rifiutata non viene eseguita
* Se un rifiuto di collegamento simbolico si ripete su un percorso, trovate cosa continua a riscrivere un collegamento lì, come uno strumento di compilazione o un file watcher, o chiedete a Claude di usare il percorso risolto del file invece di quello collegato
* Se questo rifiuto appare per ogni file mentre Claude Code viene eseguito su Windows all'interno di un AppContainer o sandbox con token limitato, aggiornate alla v2.1.265 o successiva
* Se un rifiuto di lettura appare su macOS per un file che nulla sta riscrivendo, come uno screenshot trascinato nel prompt, aggiornate alla v2.1.273 o successiva
* Per il rifiuto di ripgrep, installate ripgrep con il vostro gestore di pacchetti in modo che `rg` si risolva a un percorso assoluto su `PATH`, o mantenete le ricerche sotto la directory di lavoro

Prima della v2.1.251, Claude Code ri-controllava la risoluzione di un percorso solo per le scritture di file, quindi un collegamento sostituito dopo il controllo di autorizzazione poteva reindirizzare una lettura o una ricerca a una posizione diversa senza un messaggio. Di questi, solo il rifiuto di scrittura della directory padre, attraverso-collegamento e directory-collegata appaiono nelle versioni precedenti.

<h3 id="task-output-swap-refused">
  Task output swap refused
</h3>

Claude Code salva l'output di ogni comando Bash in un file sotto la sua directory temporanea. Ogni volta che apre uno di questi file, controlla che il percorso conduca ancora al file che ha creato, senza collegamento simbolico, collegamento fisico aggiuntivo o directory spostata che lo reindirizza. Questo messaggio significa che quel controllo è fallito, quindi Claude Code ha rifiutato l'operazione piuttosto che scrivere o leggere l'output attraverso quel percorso. Il messaggio appare nel risultato dello strumento Bash:

```text wrap theme={null}
task output swap refused (tasks dir moved or linked): /private/tmp/claude-501/-Users-you-my-project/1f0e62dc-4b0a-4f5e-9c2d-8a7b6c5d4e3f/tasks/b7k2f9m3q.output. To recover: restart Claude Code with CLAUDE_CODE_TMPDIR set to a fresh directory; or, if /private/tmp/claude-501/-Users-you-my-project is a stray directory or a symbolic link that should not be there, remove that entry itself (not what it points to) and restart.
```

Il testo tra parentesi nomina il controllo che ha fallito. Motivi come `output symlink was re-pointed`, `output file identity changed`, e `not a regular file` segnalano tutti la stessa condizione: qualcosa nel percorso di output o lungo di esso non è più il file che Claude Code ha creato. Solo alcuni motivi portano una frase `To recover:`.

Se il controllo fallisce mentre un comando è ancora in esecuzione, Claude Code arresta il comando e il suo risultato segnala:

```text theme={null}
Command killed: its output file was replaced or could no longer be verified
```

**Cosa fare:**

* Aggiornate alla v2.1.260 o successiva. Le versioni precedenti a volte mostravano questo messaggio quando nessun collegamento o directory spostata era presente
* Riavviate Claude Code con [`CLAUDE_CODE_TMPDIR`](/docs/it/env-vars) impostato su una directory fresca
* O controllate la directory del vostro progetto sotto la directory temporanea di Claude Code, `/private/tmp/claude-501/-Users-you-my-project` nel messaggio di esempio. Se quel percorso è un collegamento simbolico, o una directory che non dovrebbe essere lì, rimuovete il collegamento o la directory stessa piuttosto che la destinazione del collegamento, e riavviate Claude Code
* Se il rifiuto si ripete, un processo sta sostituendo, collegando o rimuovendo voci sotto la directory temporanea di Claude Code mentre la sessione viene eseguita. Impostate [`CLAUDE_CODE_TMPDIR`](/docs/it/env-vars) su una directory che nient'altro gestisce e riavviate

<h3 id="the-source-file-is-not-valid-utf-8-text">
  The source file is not valid UTF-8 text
</h3>

Claude ha tentato di pubblicare un [artifact](/docs/it/artifacts) da un file i cui byte non si decodificano come testo, o il cui testo contiene già il carattere di sostituzione `U+FFFD`, quindi Claude Code ha rifiutato la pubblicazione prima di caricare qualsiasi cosa. Il messaggio appare nel risultato dello strumento Artifact e nomina la prima posizione da correggere:

```text wrap theme={null}
file_path: the source file is not valid UTF-8 text (first invalid byte at line 12, column 40). It may be saved in another encoding or contain binary data. Rewrite it as UTF-8, then publish again. Nothing was published.

file_path: the source file has the replacement character U+FFFD at line 12, column 40, usually left where an earlier edit or paste lost a character. Replace it with the intended text (in HTML, write an intended U+FFFD as &#xFFFD;), then publish again. Nothing was published.
```

Claude Code decodifica il file come UTF-8, o come UTF-16 quando inizia con un byte-order mark UTF-16 little-endian. Quando un file UTF-16 di questo tipo non si decodifica, il primo messaggio nomina `UTF-16` e vi dice comunque di riscrivere il file come UTF-8. Quando seguono più posizioni di quella nominata, il messaggio aggiunge un conteggio come `(+2 more)` dopo la posizione.

**Cosa fare:**

* Di solito nulla: Claude riscrive il file e pubblica di nuovo
* Se il file è uno che avete scritto o esportato, salvatelo di nuovo come UTF-8, e sostituite ogni `U+FFFD` con il carattere che un'edizione, incolla o conversione precedente ha perso
* Per mostrare un `U+FFFD` intenzionale sulla pagina, scrivetelo come `&#xFFFD;` nell'HTML invece del carattere letterale

Prima della v2.1.267, Claude Code caricava un file di questo tipo senza controllarlo, e il server rifiutava la pubblicazione invece.

<h3 id="reading-a-local-file-from-outside-the-connected-folders">
  Reading a local file from outside the connected folders in a Cowork session
</h3>

In una sessione [Cowork](https://claude.com/docs/cowork/overview) in esecuzione sulla vostra macchina nell'app Claude Desktop, Claude ha nominato un file locale per un [artifact](/docs/it/artifacts). Claude Code non ha potuto confermare che il file è un file semplice all'interno delle cartelle connesse della sessione: il percorso si trova al di fuori di quelle cartelle, passa attraverso un collegamento simbolico, o è scritto in un modo che può nominare un file diverso da come appare. Leggere un file di questo tipo richiede la vostra approvazione, e in una sessione che non può mostrarvi la scheda di approvazione, come una impostata per saltare tutte le approvazioni, Claude Code rifiuta la lettura.

Il rifiuto appare nel risultato dello strumento Artifact; quando il file non ha potuto essere esaminato affatto, nomina invece quel fallimento:

```text wrap theme={null}
Reading a local file from outside this session's connected folders, or through a link, needs the approval card, and no one can answer it in this Cowork session. Use a plain file inside the connected folders; do not retry this file in this session.

cannot read file_path (ENOENT) — the file could not be examined, and no one can answer the approval card in this Cowork session. Check that the file exists as a plain file inside the connected folders, then retry with that path.
```

**Cosa fare:**

* Di solito nulla: il messaggio dice a Claude di usare un file semplice all'interno delle cartelle connesse invece
* Per mettere quel file esatto nell'artifact, copiatelo in una delle cartelle connesse della sessione come file regolare, non un symlink, e chiedete di nuovo

<h3 id="webfetch-cannot-fetch-localhost">
  WebFetch cannot fetch localhost
</h3>

Claude ha chiamato [WebFetch](/docs/it/tools-reference#webfetch-tool-behavior) con un URL il cui hostname non ha un punto, come `http://localhost:3000` o un nome intranet semplice come `http://wiki/`. WebFetch rifiuta questi URL prima di fare qualsiasi richiesta:

```text wrap theme={null}
WebFetch cannot fetch localhost or other hostnames without a dot. To reach a local server, use Bash with curl instead.
```

**Cosa fare:**

* Di solito nulla: il messaggio indica a Claude lo strumento `curl` attraverso Bash, che può raggiungere server locali e intranet

Prima della v2.1.268, WebFetch segnalava questi URL con un errore generico `Invalid URL`.

<h2 id="background-session-errors">
  Errori di sessione in background
</h2>

Le [sessioni in background](/docs/it/agent-view) vengono eseguite senza un terminale interattivo proprio, quindi i comandi che ne richiedono uno si comportano diversamente lì. Questi messaggi appaiono nella trascrizione di una sessione in background, nel terminale che si collega a una, nella sessione o shell da cui si invia, o, per le [voci worktree-guard](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) di seguito, in qualsiasi sessione isolata in un worktree o che esegue un subagent isolato da worktree; dove un messaggio è specifico di una superficie, la sua voce lo dice.

<h3 id="commands-refused-in-a-background-session">
  Comandi rifiutati in una sessione in background
</h3>

I comandi che aprono una finestra di dialogo interattiva non possono farlo mentre nessun terminale è collegato a una sessione in background. `/install-github-app`, l'elenco delle impostazioni `/mcp` e le azioni di autenticazione nel menu del server MCP rispondono con un messaggio, e la sessione appare sotto **Needs input** nella [vista agente](/docs/it/agent-view) in modo che tu possa trovarla, collegarti ed eseguire di nuovo il comando. Mentre un terminale è collegato, questi comandi funzionano normalmente.

Prima della v2.1.216, la sessione non appariva sotto **Needs input** dopo uno di questi rifiuti. Nella v2.1.213 attraverso v2.1.215, i comandi funzionavano ancora mentre un terminale era collegato, e il messaggio di rifiuto ti diceva di collegarti ed eseguire di nuovo il comando. Dalla v2.1.208 attraverso v2.1.212, Claude Code li rifiutava anche mentre un terminale era collegato, con un messaggio come `Can't open MCP settings in a background session`; su quelle versioni, esegui il comando da una sessione `claude` regolare invece, o esegui l'upgrade. Prima della v2.1.208, aprivano la loro finestra di dialogo all'interno della sessione in background. Solo nella v2.1.208, Claude Code ha anche rifiutato il selettore `/model` in una sessione in background, e `/upgrade` ha stampato l'URL di upgrade invece di aprire un browser.

La formulazione nomina il comando. L'elenco delle impostazioni `/mcp` riporta:

```text theme={null}
Can't open MCP settings while no terminal is attached to this background session. This session now shows "needs input" in agent view — open it and run /mcp to manage servers, or use `/mcp enable|disable|reconnect <server>` to steer without the panel.
```

**Cosa fare:**

* Collegati alla sessione dalla vista agente, dove è elencata sotto **Needs input**, ed esegui di nuovo il comando
* Oppure usa il modulo che il messaggio nomina, come `/mcp reconnect <server>`, `/mcp enable`, o `/mcp disable`, che funzionano senza collegarsi

<h3 id="write-or-command-blocked-because-the-path-cannot-be-safely-resolved">
  Scrittura o comando bloccato perché il percorso non può essere risolto in modo sicuro
</h3>

Claude ha indirizzato un file o una directory di lavoro attraverso un'ortografia che la [guardia di isolamento worktree](/docs/it/agent-view#how-file-edits-are-isolated) non può risolvere in un'unica posizione verificabile. La guardia controlla le scritture e le directory di lavoro dei comandi in [qualsiasi sessione isolata in un worktree](/docs/it/worktrees#how-claude-code-enforces-isolation), interattiva o in background, e in [subagent isolati da worktree](/docs/it/worktrees#isolate-subagents-with-worktrees). Risolve i symlink prima di controllare che l'operazione non raggiunga il checkout condiviso, e quando la risoluzione fallisce, blocca l'operazione piuttosto che lasciarla atterrare lì. Il messaggio nomina le forme di percorso che rifiuta e come riprovare:

```text theme={null}
This write was blocked because the path is spelled in a form that cannot be safely resolved (for example through a symlink storing a raw dot segment, a network-share or device-namespace shape, or an unreadable ancestor directory). If the file is inside the worktree /path/to/worktree, address it by its direct symlink-free path instead.
```

Un comando bloccato riporta la stessa causa per la sua directory di lavoro e termina con `re-run the command from its direct symlink-free path`. Prima della v2.1.217, la guardia confrontava le ortografie dei percorsi senza risolvere i symlink, quindi queste ortografie non erano bloccate e una scrittura instradata attraverso un symlink poteva atterrare nel checkout condiviso.

**Cosa fare:**

* Di solito nulla: il messaggio completo va a Claude come errore dello strumento, e Claude riprova con il percorso diretto che nomina. Per una modifica di file bloccata, la vista della conversazione mostra solo una breve riga `Error editing file`; il messaggio completo appare nella vista della trascrizione, che apri con `Ctrl+O`. Un comando bloccato lo stampa nel suo output di comando.
* Se il blocco si ripete sullo stesso file, il percorso probabilmente passa attraverso un symlink committato il cui target contiene `..`, come `docs/current -> ../README.md`; chiedi a Claude di modificare il file target dal suo percorso reale invece che attraverso il link

<h3 id="write-or-command-blocked-because-the-path-names-a-network-location">
  Scrittura o comando bloccato perché il percorso nomina una posizione di rete
</h3>

Claude ha indirizzato un file o una directory di lavoro attraverso un percorso che nomina un'unità che non è sulla tua macchina, una condivisione UNC come `\\server\share\file` o un percorso di automount `/net`, mentre il checkout della sessione è su un disco locale. La stessa [guardia di isolamento worktree](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) non può verificare che tale percorso rimanga fuori dal checkout condiviso, quindi blocca l'operazione. Isolare la sessione in un worktree non solleva il blocco. Il messaggio nomina la forma di percorso da usare invece:

```text theme={null}
This write was blocked because the path is network-shaped (a UNC share or /net automount spelling) while this session's checkout is local. Isolating cannot unblock it. If the file is genuinely inside the worktree /path/to/worktree, address it by its local, plainly-spelled path instead.
```

Un comando bloccato riporta la stessa causa per la sua directory di lavoro e termina con `re-run the command from its local, plainly-spelled path`. Prima della v2.1.217, la guardia confrontava solo il testo del percorso, quindi indirizzare un file all'interno del checkout attraverso un percorso UNC o `/net` non era bloccato.

**Cosa fare:**

* Di solito nulla: Claude riprova con l'ortografia locale che il messaggio chiede
* Se il file è su una condivisione di rete piuttosto che un file locale scritto con un percorso di rete, è al di fuori dell'area di lavoro locale della sessione; modificalo da una sessione interattiva regolare invece

<h3 id="command-blocked-by-the-worktree-isolation-checks">
  Comando bloccato dai controlli di isolamento worktree
</h3>

Claude ha eseguito un comando Bash o Monitor in una [sessione isolata in un worktree](/docs/it/worktrees#how-claude-code-enforces-isolation), e Claude Code l'ha rifiutato per uno di due motivi:

* Il comando punta git al checkout principale.
* Claude Code non può verificare dal testo del comando che qualsiasi git che il comando esegue rimanga all'interno del worktree. Un comando che non nomina mai git può comunque essere rifiutato per questo motivo, perché espandere un'indirezione di variabile come `${!name}` o eseguire una sostituzione di funzione Bash come `${ command; }` produce un valore in fase di esecuzione che può essere esso stesso un comando.

Il mezzo del messaggio nomina cosa non poteva essere verificato:

```text wrap theme={null}
This session is isolated in the worktree /path/to/worktree, but this command evaluates ${!x@P} arithmetically inside a construct too complex to verify, which can run a command hidden in a variable's value. Refusing to run it — a worktree-isolated session's git operations must target its own worktree. Split it into plain, separate commands and run them from /path/to/worktree.
```

**Cosa fare:**

* Di solito nulla: Claude legge il messaggio e riscrive il comando nel modo che la sua frase finale chiede
* Se un comando che hai chiesto continua ad essere rifiutato, scrivi il valore contrassegnato letteralmente: sostituisci l'indirezione o la sostituzione con il suo valore, ed esegui git come suo proprio comando semplice dall'interno del worktree
* Per agire sul checkout principale di proposito, esegui il comando tu stesso in un terminale al di fuori della sessione

<h3 id="this-session-has-no-saved-transcript">
  Questa sessione non ha una trascrizione salvata
</h3>

Hai collegato una [sessione in background](/docs/it/agent-view) interrotta che è stata messa in background da un'altra conversazione con `←` o `/background` e interrotta prima che la sua prima risposta finisse. Fino a quando quella prima risposta non finisce, la conversazione vive ancora solo nella sessione da cui è stata messa in background, quindi `claude attach` rifiuta di avviare la sessione interrotta piuttosto che iniziare una conversazione vuota con lo stesso ID di sessione. Il messaggio termina con il comando `claude respawn` per questa sessione:

```text theme={null}
This session has no saved transcript — it was stopped before its first response finished. If it was backgrounded from another conversation, that one is still intact; `claude respawn <id>` starts this one fresh.
```

Aprire la stessa riga della sessione nella [vista agente](/docs/it/agent-view) mostra `Press enter again to restart this session fresh` sotto l'elenco invece, e un secondo `Enter` sulla riga riavvia la sessione con una conversazione vuota. Prima della v2.1.212, aprire la riga mostrava il messaggio di rifiuto senza modo di riavviare dalla vista agente. Prima della v2.1.211, aprire la sessione interrotta avviava silenziosamente quella conversazione vuota e poteva rieseguire il prompt originale della sessione.

**Cosa fare:**

* La conversazione che hai messo in background è intatta: riprendi con [`claude --resume`](/docs/it/sessions) o continua a lavorarci
* Per avviare la sessione interrotta da zero comunque, esegui `claude respawn <id>` con l'ID dal messaggio, o premi `Enter` due volte sulla sua riga nella vista agente
* Se la sessione ha finito una risposta e vedi ancora questo rifiuto su una versione prima della v2.1.214, una cartella illeggibile in `~/.claude/projects` potrebbe far sì che la scansione della trascrizione perda la conversazione salvata; aggiorna alla v2.1.214 o successiva, che tollera le cartelle illeggibili durante la scansione

<h3 id="this-session-is-running-in-another-terminal">
  Questa sessione è in esecuzione in un altro terminale
</h3>

Hai aperto la riga di una sessione interrotta nella [vista agente](/docs/it/agent-view), e la sua conversazione salvata è già aperta in un altro processo Claude Code attivo su questa macchina, quindi Claude Code rifiuta di avviare un secondo processo che scriverebbe sulla stessa trascrizione. Quale messaggio vedi dipende da [cosa tiene la conversazione](/docs/it/agent-view#opening-a-session-says-the-conversation-is-already-open):

```text theme={null}
Can't open — this session is running in another terminal
This conversation is already open in another running Claude session — use that one, or close it and try again
```

* **`running in another terminal`**: un terminale tiene la conversazione, ad esempio uno in cui l'hai ripresa con `claude --resume` o `/resume`. La riga mostra anche `Open in a terminal`.
* **`already open in another running Claude session`**: un altro processo Claude Code non interattivo la tiene, ad esempio un processo [sessione in background](/docs/it/agent-view#the-supervisor-process) per la stessa conversazione che non è ancora uscito.

Claude Code salva una risposta che hai digitato quando apri la riga e la invia come il prossimo prompt della sessione quando la sessione si avvia di nuovo.

**Cosa fare:**

* Continua la conversazione nel processo che l'ha aperta, o esci da quel processo e apri di nuovo la riga

Prima della v2.1.248, esisteva solo il rifiuto `already open in another running Claude session`: una conversazione ripresa in un terminale non contava come aperta, e aprire la riga avviava un secondo processo Claude Code che scriveva sulla stessa conversazione.

<h3 id="this-sessions-saved-conversation-is-no-longer-on-disk">
  La conversazione salvata di questa sessione non è più su disco
</h3>

Hai aperto una [sessione in background](/docs/it/agent-view) che è terminata mentre il servizio in background era spento, e la [pulizia della trascrizione](/docs/it/settings-reference#cleanupperioddays) ha da allora rimosso la sua conversazione salvata, ad esempio dopo che la macchina è stata spenta per settimane. Aprire una tale riga normalmente [riprende la sua conversazione salvata](/docs/it/agent-view#sessions-show-as-failed-after-shutdown). Non avendo nulla da riprendere, Claude Code rifiuta piuttosto che rieseguire il prompt originale della sessione senza chiedere:

```text theme={null}
This session's saved conversation is no longer on disk (it ended while the background service was off, and old transcripts are cleaned up), so there is nothing to resume. `claude rm 7c5dcf5d` deletes the row; `claude respawn 7c5dcf5d` runs its original prompt again instead.
```

`claude attach <id>` stampa questo testo. Nella vista agente, il piè di pagina è più breve e termina con `ctrl+x deletes the row`.

**Cosa fare:**

* Esegui `claude rm <id>` per eliminare la riga. Quando uno dei [casi mantenuti](/docs/it/agent-view#what-deleting-a-session-removes) si applica, `claude rm` mantiene la riga e il worktree invece e nomina il motivo
* Per eseguire di nuovo il prompt originale della sessione come una conversazione fresca, esegui `claude respawn <id>`

Prima della v2.1.248, aprire una tale riga rieseguiva il prompt originale della sessione invece di rifiutare, tirando un compito di settimane fa in primo piano.

<h3 id="worktree-has-commits-that-are-not-pushed-anywhere">
  Worktree ha commit che non sono spinti da nessuna parte
</h3>

Hai provato a eliminare una [sessione in background](/docs/it/agent-view#what-deleting-a-session-removes) il cui worktree contiene commit che Claude Code non può confermare siano salvati altrove. Claude Code mantiene il worktree e la riga della sessione piuttosto che distruggere i commit senza vederli. `claude rm` nomina il ramo e i commit non spinti, e dice come procedere:

```text theme={null}
kept 7c5dcf5d — its worktree is still at "/home/you/project/.claude/worktrees/fix-login"
  2 unpushed commits on "claude/fix-login": a1b2c3d "Fix login flow" and 1 more. They exist on no remote, so deleting the worktree would lose them.
  push them and run 'claude rm 7c5dcf5d' again, or discard the worktree and its commits: claude rm 7c5dcf5d --discard-unpushed a1b2c3d000000000000000000000000000000000@0123456789abcdef0123456789abcdef
```

Quando Claude Code non può riassumere i commit, la riga di dettaglio legge `The worktree has unpushed commits` invece. Nella [vista agente](/docs/it/agent-view), la riga della sessione mostra `not deleted` con lo stesso motivo.

I commit su un remote non bloccano l'eliminazione. Nemmeno i commit sulla copia locale del ramo predefinito del tuo remote `origin`, purché quel ramo sia estratto nel tuo checkout principale, la directory del repository stesso piuttosto che un worktree.

**Cosa fare:**

* Per mantenere i commit, spingi il ramo del worktree, o uniscilo al ramo predefinito estratto nel tuo checkout principale, quindi elimina di nuovo la sessione
* Per scartare i commit, esegui il comando `claude rm <id> --discard-unpushed` che il messaggio ha stampato, o premi `Ctrl+X` due volte sulla riga della sessione nella vista agente di nuovo. Questo rimuove la sessione e il worktree insieme al suo ramo, ai commit non spinti e a qualsiasi modifica non committata. Se il worktree ha guadagnato un commit dal rifiuto, Claude Code lo mantiene di nuovo e mostra lo stato aggiornato
* Quando il messaggio dice che il worktree è anche registrato da un'altra sessione terminata, eliminare di nuovo non lo scarta: spingi i commit, quindi elimina di nuovo la sessione

Prima della v2.1.268, `claude rm` metteva il riassunto del commit sulla riga `kept` stessa. Quando `claude rm` non poteva riassumere i commit, la riga `kept` leggeva `worktree has commits that are not pushed anywhere` al posto del riassunto.

Prima della v2.1.260, il messaggio non nominava il ramo o i commit, e eliminare di nuovo era rifiutato allo stesso modo: eliminare la sessione senza spingere significava rimuovere il worktree tu stesso con `git worktree remove --force <path>`, quindi eseguire di nuovo `claude rm <id>`.

Prima della v2.1.248, il ramo predefinito estratto nel tuo checkout principale non contava: un ramo che avevi già unito lì attivava ancora questo rifiuto fino a quando i suoi commit non raggiungevano un remote.

<h3 id="terminal-host-process-died">
  Il processo host del terminale è morto
</h3>

Ogni [terminale della sessione in background](/docs/it/agent-view) viene eseguito in un processo host sotto il servizio in background, e quel processo è morto mentre il servizio manteneva ancora la sua connessione, quindi la sessione non poteva essere raggiunta.

Su Linux e WSL, il servizio in background controlla ogni processo host ogni pochi secondi, contrassegna la sessione come fallita quando il processo è uscito ma la sua connessione al servizio non si è mai chiusa, e mostra il motivo sulla sua riga nella [vista agente](/docs/it/agent-view#read-session-state):

```text theme={null}
terminal host process died — press Enter to restart
```

Se apri la riga prima che il controllo venga eseguito, il piè di pagina mostra `This session's terminal host process died (the conversation is saved) — press Enter to restart it` e la riga diventa fallita.

Dalla shell, `claude attach <id>` riavvia una sessione già contrassegnata come fallita per un host morto, e altrimenti stampa il motivo e esce:

```text theme={null}
Couldn't attach to <id> — This session's terminal host process died (the conversation is saved) — run `claude attach <id>` again to restart it on a fresh host.
```

La conversazione è salvata comunque.

Una riga che esegue un [comando shell](/docs/it/agent-view#run-a-shell-command) invece mostra `terminal host process died — its output is gone; the command was not run again`, e `claude attach` stampa `This command's terminal host process died — its output is gone and the command was not run again`. Claude Code non riesegue mai il comando per te.

**Cosa fare:**

* Nella vista agente, premi `Enter` sulla riga fallita; la sessione si riavvia su un nuovo processo host e la conversazione riprende
* Dalla shell, esegui di nuovo `claude attach <id>`. Claude Code stampa `Session <id>'s terminal host died — restarting it on a fresh one…` e riapre la sessione
* Non puoi riavviare una riga di comando shell in questo modo; invia di nuovo il comando per rieseguirlo

Prima della v2.1.247, un processo host morto poteva passare ogni controllo di vitalità che il servizio in background eseguiva, quindi aprire la sessione mostrava `opening… · esc to cancel` indefinitamente e `claude attach <id>` aspettava senza segnalare un errore.

<h3 id="session-isnt-responding">
  La sessione non sta rispondendo
</h3>

Hai aperto una [sessione in background](/docs/it/agent-view) e il servizio in background ha accettato l'apertura, ma nessun output è arrivato per circa dieci secondi, quindi Claude Code conclude che il processo che trasmette il terminale della sessione non può fornire output, e termina il tentativo invece di aspettare.

Nella vista agente, Claude Code offre un riavvio nel piè di pagina:

```text theme={null}
Press enter again to restart this session — it isn't responding (its conversation is saved and resumes).
```

Dalla shell, `claude attach <id>` stampa il motivo e esce:

```text theme={null}
Couldn't attach to <id> — Session isn't responding — `claude stop <id>`, then `claude attach <id>` restarts it (the conversation is saved).
```

Claude Code non riavvia mai una riga che esegue un [comando shell](/docs/it/agent-view#run-a-shell-command) per te, perché un riavvio rieseguirebbe il comando.

**Cosa fare:**

* Nella vista agente, premi `Enter` sulla stessa riga di nuovo. Claude Code interrompe il processo che non risponde e riavvia la sessione, e la conversazione riprende. Nulla viene interrotto senza quella seconda pressione
* Dalla shell, esegui `claude stop <id>`, quindi `claude attach <id>`
* Per una riga di comando shell, premi `Ctrl+X` nella vista agente o esegui `claude stop <id>` per interromperla; invia di nuovo il comando per rieseguirlo

<h3 id="session-was-stopped-while-the-respawn-was-in-flight">
  La sessione è stata interrotta mentre il respawn era in volo
</h3>

Hai aperto una [sessione in background](/docs/it/agent-view) il cui processo non era in esecuzione, e mentre Claude Code la stava riavviando, un altro processo Claude Code l'ha interrotta, ad esempio `claude stop` in un altro terminale. Claude Code mantiene la sessione interrotta:

```text theme={null}
Session <id> was stopped while the respawn was in flight
```

Aprire una sessione che hai appena inviato, mentre il suo processo è ancora in avvio, aspetta il processo invece. Prima della v2.1.246, aprirla in quel momento poteva interromperla e mostrare questo messaggio.

**Cosa fare:**

* Se non hai interrotto la sessione, apri di nuovo la sua riga nella vista agente o esegui `claude respawn <id>` per riavviarla
* Se l'hai interrotta tu stesso, non rimane nulla da fare: la sessione rimane interrotta

<h3 id="session-agent-no-longer-available">
  Agente della sessione non più disponibile
</h3>

Hai ripreso una sessione che stava eseguendo un [agente personalizzato](/docs/it/sub-agents#invoke-subagents-explicitly), avviato con `--agent` o l'impostazione `agent`, e Claude Code non ha trovato un agente con quel nome. Cerca prima nella directory originale della sessione, quando hai [fiducia in quell'area di lavoro](/docs/it/permissions#project-allow-rules-and-workspace-trust), quindi nella directory da cui riprendi. La sessione riprende comunque, ma con gli strumenti predefiniti, quindi le restrizioni dello strumento dell'agente non si applicano più:

```text theme={null}
This session was running agent 'code-reviewer', which is no longer available (no agent by that name in /home/you/project). Continuing with the default tools and system prompt — the agent's tool restrictions no longer apply. To restore it, re-create the agent, or resume with an explicit --agent <name>.
```

L'avviso nomina solo le directory che Claude Code ha cercato, e appare nella conversazione ripresa sia che tu svegli una [sessione in background](/docs/it/agent-view), esegua `/resume` o `claude --resume`, o riprenda in [modalità non interattiva](/docs/it/headless), dove va anche a stderr. Le sessioni che usano `--input-format stream-json` non lo mostrano, perché l'Agent SDK fornisce agenti dopo l'avvio.

Claude Code non salva il fallback nella sessione, quindi l'avviso si ripete ad ogni ripresa fino a quando non agisci. L'agente `claude` integrato non attiva l'avviso, poiché il fallback al set di strumenti predefinito non cambia nulla per esso. Prima della v2.1.216, Claude Code continuava silenziosamente come l'agente predefinito, e la ricerca copriva solo la directory da cui riprendevi, quindi un agente con ambito di progetto era perso ad ogni ripresa da un'altra directory.

**Cosa fare:**

* Ricrea il file dell'agente in `.claude/agents/<name>.md` nel progetto della sessione, o in `~/.claude/agents/<name>.md` per un agente personale, quindi riprendi di nuovo
* Oppure riprendi con `--agent <name>` nominando un agente che esiste, per eseguire la sessione come quell'agente invece
* Se l'agente ha ambito di progetto e non hai fiducia nella directory originale della sessione, esegui Claude Code lì una volta, accetta la finestra di dialogo di fiducia, quindi riprendi di nuovo

<h3 id="claude_code_process_wrapper-launcher-errors">
  Errori del launcher CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/it/corporate-launcher) è impostato, e il suo valore non può essere usato, quindi Claude Code rifiuta di avviare il processo interessato piuttosto che eseguirlo senza il launcher. I problemi di configurazione sono segnalati con un messaggio che inizia con il nome della variabile e dichiara il motivo, ad esempio:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Un launcher che si avvia ma esce senza sostituirsi con Claude Code fallisce la sessione che stava avviando, e la riga della sessione nella vista agente riporta che il launcher `must exec, not daemonize`, seguito da qualsiasi cosa il launcher abbia stampato. Una sessione che non può avviarsi o raggiungere il servizio in background a causa del launcher riporta il problema del launcher come motivo all'interno di `Couldn't reach the background service (...)`.

**Cosa fare:**

* Imposta la variabile al percorso assoluto di un eseguibile che termina chiamando `exec "$@"`. Vedi [il contratto del launcher](/docs/it/corporate-launcher#the-launcher-contract) per il contratto completo
* Controlla `/status`, che mostra il comando di avvio risolto nella sua voce Self-exec e avverte quando il servizio in background in esecuzione non corrisponde, o esegui `claude daemon status` da una shell
* Dopo aver corretto il valore nel blocco `env` delle [impostazioni](/docs/it/corporate-launcher#set-up-the-launcher), riavvia il servizio in background con `claude daemon stop --any` in modo che il prossimo invio avvii uno avvolto

<h3 id="eunknown-when-starting-a-background-session">
  EUNKNOWN quando si avvia una sessione in background
</h3>

Windows ha rifiutato di avviare un programma con un codice di errore che non ha un nome standard, quindi l'errore emerge come `EUNKNOWN`. Il trigger solito è una politica di restrizione del software, come Group Policy o AppLocker, che blocca il programma in fase di avvio. L'errore appare quando avvii una [sessione in background](/docs/it/agent-view) con `/background` o `claude --bg`:

```text theme={null}
Couldn't reach the background service (spawn background service: EUNKNOWN: unknown error, uv_spawn) — run 'claude daemon status'
```

Su alcuni account il messaggio dice `daemon` al posto di `background service`.

Su un'installazione npm, un `EUNKNOWN` che appare mentre `npm install -g @anthropic-ai/claude-code` sta sostituendo il binario ha la stessa causa di [`EACCES` durante una reinstallazione](#eacces-when-starting-a-background-session) e si cancella quando riprovi dopo che l'installazione finisce.

Claude Code avvia il servizio in background attraverso PowerShell in modo che il servizio sopravviva alla chiusura del terminale, usando PowerShell 7 quando è installato e Windows PowerShell 5.1 altrimenti. Quando nessun PowerShell può essere eseguito, Claude Code avvia il servizio direttamente invece, quindi una politica che blocca solo PowerShell non causa questo errore. Se lo vedi mentre nessun npm install è in esecuzione, la politica sta bloccando l'eseguibile Claude Code stesso.

Prima della v2.1.212, Claude Code usava solo Windows PowerShell 5.1 per avviare il servizio, quindi qualsiasi macchina dove Group Policy bloccava PowerShell 5.1 falliva con `Couldn't start the session — EUNKNOWN: unknown error, uv_spawn`, anche con PowerShell 7 installato.

**Cosa fare:**

* Se il messaggio legge `Couldn't start the session`, aggiorna alla v2.1.212 o successiva. Su versioni precedenti puoi anche eseguire `claude daemon run` in un terminale separato per primo, quindi avviare di nuovo la sessione in background. Quel comando esegue il servizio in background in primo piano del terminale, quindi il servizio dura solo finché quel terminale rimane aperto.
* Se un npm install stava sostituendo il binario, aspetta che finisca, quindi avvia di nuovo la sessione in background
* Se l'errore appare su v2.1.212 o successiva mentre nessun npm install è in esecuzione, chiedi al tuo amministratore Windows di consentire l'eseguibile Claude Code nella politica di restrizione
* Se il servizio in background si interrompe quando chiudi il terminale, Claude Code l'ha avviato senza PowerShell. Installa PowerShell 7, o chiedi al tuo amministratore di sbloccare PowerShell, in modo che il servizio possa sopravvivere al terminale.

<h3 id="eacces-when-starting-a-background-session">
  EACCES quando si avvia una sessione in background
</h3>

Claude Code non poteva eseguire il suo stesso binario per avviare il [servizio in background](/docs/it/agent-view#the-supervisor-process) che ospita le sessioni in background. Su un'installazione npm, questo di solito significa che `npm install -g @anthropic-ai/claude-code` stava sostituendo il binario in quel momento, sia che l'abbia eseguito tu che l'[auto-updater](/docs/it/setup#auto-updates). L'errore appare quando apri una sessione dalla [vista agente](/docs/it/agent-view):

```text theme={null}
Couldn't start the background service — spawn background service: EACCES: permission denied, posix_spawn '/usr/local/lib/node_modules/@anthropic-ai/claude-code/bin/claude'
```

Quando avvii una sessione con `/background` o `claude --bg`, lo stesso motivo appare all'interno di `Couldn't reach the background service (...)`. Durante la stessa finestra di reinstallazione l'errore può nominare un altro codice invece, come `ENOENT` o `ENOEXEC`, o `EUNKNOWN` o `EPERM` su Windows; un `EUNKNOWN` che persiste attraverso i tentativi ha una [causa diversa](#eunknown-when-starting-a-background-session).

Su un'installazione npm, Claude Code aspetta che la reinstallazione finisca e riprova da solo: fino a dieci secondi, e fino a due minuti mentre un npm install di Claude Code è visibilmente ancora in esecuzione sulla macchina, che copre un altro processo Claude Code che scarica un aggiornamento. Quando l'installazione dura più di quella attesa, l'errore nomina l'aggiornamento invece del codice di errore nudo:

```text theme={null}
Claude Code is being updated by npm on this machine (still not runnable after 2 min, EACCES) — try again when the update finishes
```

Prima della v2.1.257, l'attesa si fermava a dieci secondi in ogni caso, quindi questo errore appariva mentre un altro processo Claude Code stava ancora scaricando un aggiornamento. Prima della v2.1.246, Claude Code falliva subito, senza aspettare.

**Cosa fare:**

* Aspetta alcuni secondi, quindi apri di nuovo la sessione o invia di nuovo. Quando il messaggio dice che Claude Code è in fase di aggiornamento, riprova dopo che l'aggiornamento finisce.
* Se l'errore persiste mentre nessun npm install è in esecuzione, il tuo utente non può eseguire il binario installato. Controlla i suoi permessi e quelli della sua directory, o reinstalla Claude Code.

<h3 id="background-service-exited-before-it-became-reachable">
  Il servizio in background è uscito prima di diventare raggiungibile
</h3>

Il processo che Claude Code ha avviato come [servizio in background](/docs/it/agent-view#the-supervisor-process) è uscito prima di accettare connessioni, quindi Claude Code non poteva aprire la tua sessione. Quando il servizio ha stampato un errore prima di uscire, il motivo tra parentesi fornisce il codice di uscita o il segnale e la prima riga che il servizio ha stampato, che nomina cosa l'ha fermato:

```text theme={null}
Couldn't reach the background service (background service exited before it became reachable (exit code N): <the service's first error line>) — run 'claude daemon status'
```

Quando apri una sessione dalla [vista agente](/docs/it/agent-view), lo stesso motivo segue `Couldn't start the background service —`. Quando il servizio non ha stampato nulla prima di uscire, il messaggio dice `nothing on stderr` invece.

Claude Code riporta l'errore con la riga di errore del servizio. Prima della v2.1.246, l'errore emergeva solo dopo un'attesa di 45 secondi, come `background service did not become reachable within 45s`, senza la riga di errore del servizio.

Due motivi citati hanno cause note:

* `Error: claude native binary not installed.`: un npm install stava sostituendo il binario Claude Code in quel momento, quindi il servizio ha eseguito il placeholder di npm invece. Riprova dopo che l'installazione finisce; se la riga persiste senza nessun install in esecuzione, [completa l'npm install](/docs/it/troubleshoot-install#native-binary-not-found-after-npm-install). Prima della v2.1.257, un auto-aggiornamento npm di macOS ha prodotto questo errore ad ogni avvio durante la finestra di installazione.
* `nothing on stderr` con codice di uscita 1, ad ogni avvio, su Windows: `daemon.lock` nomina un processo che Claude Code non può né segnalare né provare sia andato, quindi ogni nuovo servizio conclude che un altro lo tiene e esce. Un lock il cui scrittore Claude Code può provare sia andato viene sostituito da solo e non produce questo errore. Quando l'errore si ripete ad ogni avvio, elimina `~/.claude/daemon.lock`, quindi apri di nuovo la sessione o invia di nuovo. Prima della v2.1.257, tale lock bloccava ogni avvio fino a quando non eliminavi il file.

**Cosa fare:**

* Se il messaggio cita una riga, correggi quello che nomina, quindi apri di nuovo la sessione o invia di nuovo. Il prossimo tentativo avvia di nuovo il servizio
* Esegui `claude daemon status` per controllare se un servizio è in esecuzione ora

<h3 id="working-directory-no-longer-exists-when-starting-a-background-session">
  La directory di lavoro non esiste più quando si avvia una sessione in background
</h3>

Hai provato ad avviare una [sessione in background](/docs/it/agent-view) in una directory che non esiste più. Questo accade quando invii dalla vista agente o esegui `/background` dopo che la directory in cui stavi lavorando è stata eliminata o spostata. Accade anche quando ti colleghi a o riavvii una sessione il cui processo è uscito e la cui directory è scomparsa, perché il nuovo processo si avvierebbe in quella stessa directory. Claude Code non avvia la sessione, e il messaggio nomina la directory mancante:

```text theme={null}
Couldn't start a background session (working directory no longer exists or is not accessible: /tmp/demo)
```

Prima della v2.1.257, la sessione sembrava avviarsi e poi mostrava nella vista agente come una riga fallita con lo stesso motivo.

**Cosa fare:**

* Ricrea la directory che il messaggio nomina, o invia da una directory che esiste, quindi riprova

<h2 id="wrapper-and-ide-errors">
  Errori del wrapper e dell'IDE
</h2>

Questi errori provengono dal programma che ha avviato Claude Code per voi, come un'estensione IDE o un'applicazione [Agent SDK](/docs/it/agent-sdk/overview), piuttosto che da Claude Code stesso.

<h3 id="claude-code-process-exited-with-code-n">
  Il processo Claude Code è uscito con codice N
</h3>

Il processo `claude` sottostante è uscito con un codice diverso da zero. Il codice di uscita da solo non dice cosa è fallito: l'errore reale si trova nell'output del processo stesso, che il wrapper allega quando lo ha catturato e altrimenti mantiene nei suoi log.

```text theme={null}
Error: Claude Code process exited with code 1
```

Su Windows, la build nativa può uscire con codice `4294967295` subito dopo il completamento di un turno. Quando quell'uscita si verifica al confine di un turno, senza alcun messaggio in attesa e nessuna attività in background in esecuzione, l'[estensione VS Code](/docs/it/vs-code) chiude la sessione silenziosamente invece di mostrare questo errore. Il vostro messaggio successivo riprende la conversazione.

Prima della v2.1.273, l'estensione mostrava l'errore per quell'uscita ad ogni confine di turno, anche se nulla era andato perso.

**Cosa fare:**

* In VS Code, seguite il collegamento **View output logs** mostrato con l'errore per vedere il guasto sottostante
* In un'applicazione Agent SDK, catturate l'errore intorno al vostro ciclo di messaggi. Le voci sotto [CLI process exit](/docs/it/agent-sdk/troubleshooting#cli-process-exit) coprono ciò che il vostro codice riceve in ogni linguaggio SDK.
* Eseguite `claude` in un terminale nello stesso progetto. Il guasto di solito si riproduce lì con il suo messaggio di errore reale, che potete quindi cercare su questa pagina.
* Eseguite `claude doctor` in un terminale per verificare l'installazione e la configurazione

<h3 id="could-not-locate-the-claude-cli-on-path">
  Could not locate the Claude CLI on PATH
</h3>

L'[estensione VS Code](/docs/it/vs-code) mostra questo errore su Windows quando aprite Claude Code nel terminale integrato, la shell del terminale è PowerShell e l'estensione non riesce a trovare l'eseguibile `claude` installato su PATH. L'estensione si rifiuta di avviare Claude Code finché non trova il `claude` installato su PATH.

```text theme={null}
Failed to run Claude Code: Error: Could not locate the Claude CLI on PATH. Launching by name in a PowerShell terminal would run a 'claude' from the open folder instead of the installed CLI, so the launch was blocked. Make sure the Claude CLI's install directory is on your system PATH (not only your PowerShell profile), then restart VS Code and try again. VS Code reads PATH when it starts, so PATH changes take effect only after a restart.
```

**Cosa fare:**

* Aprite una nuova finestra PowerShell al di fuori di VS Code ed eseguite `where.exe claude`. Se non stampa un percorso, la CLI non è su PATH: aggiungete la sua directory di installazione seguendo [Verify your PATH](/docs/it/troubleshoot-install#verify-your-path). Se stampa un percorso, la voce proviene dal vostro profilo PowerShell o da una modifica di PATH che VS Code non ha ancora raccolto; i prossimi due passaggi coprono questi casi.
* Impostate la voce PATH come variabile di ambiente utente o di sistema, non nel vostro profilo PowerShell. L'estensione non esegue il vostro profilo, quindi una modifica di PATH che vive solo lì non la raggiunge mai.
* Riavviate VS Code dopo aver modificato PATH. L'estensione controlla il PATH che VS Code ha catturato all'avvio, quindi una modifica di PATH ha effetto solo dopo un riavvio.

<h3 id="the-connection-to-claude-code-ended-before-this-message-completed">
  The connection to Claude Code ended before this message completed
</h3>

L'[estensione VS Code](/docs/it/vs-code) ha inviato il vostro messaggio al processo `claude`, e la connessione è terminata senza un errore prima che il processo lo riconoscesse o lo completasse. L'estensione non può dire se il messaggio è stato elaborato, quindi vi chiede di inviarlo di nuovo:

```text theme={null}
The connection to Claude Code ended before this message completed — it may not have been processed, so please send it again.
```

**Cosa fare:**

* Inviate il messaggio di nuovo. Il messaggio successivo avvia un nuovo processo `claude` che riprende la conversazione.
* Se si ripete, eseguite `claude` in un terminale nello stesso progetto. Un guasto che continua a terminare il processo di solito si riproduce lì con il suo messaggio di errore reale.

<h2 id="rewind-warnings-and-errors">
  Avvisi e errori di Rewind
</h2>

Questi messaggi provengono da un ripristino del codice [`/rewind`](/docs/it/checkpointing). `Restored the code, but skipped N files` è un avviso che indica che Claude Code ha saltato alcuni percorsi. `No files were restored` è un errore che significa che non ha ripristinato nulla.

<h3 id="restored-the-code-but-skipped-files">
  Restored the code, but skipped files
</h3>

Un ripristino del codice `/rewind` ha saltato uno o più percorsi tracciati invece di scrivere o eliminare attraverso di essi. Claude Code salta un percorso quando:

* è, o è diventato, un symlink, hard link, o altro file non regolare
* la sua directory è cambiata dal checkpoint
* il suo backup non può essere letto in modo sicuro

I percorsi saltati mantengono i loro contenuti attuali. Prima della v2.1.216, `/rewind` scriveva e eliminava attraverso i link nei percorsi tracciati e non segnalava un ripristino parziale.

```text theme={null}
Restored the code, but skipped 2 files: the tracked path is (or became) a link or other non-regular file, its directory changed since the checkpoint, or its backup could not be safely read. Skipped files were left untouched — run with --debug for the paths.
```

**Cosa fare:**

* Identificare quali file sono stati saltati in modo da poter gestire ognuno con i passaggi seguenti. Il messaggio fornisce solo un conteggio; il log di debug in `~/.claude/debug/<session-id>.txt` nomina ogni percorso saltato mentre il ripristino viene eseguito, quindi attivare la registrazione di debug con `/debug` prima del prossimo ripristino. Su macOS o Linux, è possibile invece trovare i link direttamente: `find . -type l` per i symlink e `find . -type f -links +1` per i file con hard link.
* Se un file saltato è un link che hai creato intenzionalmente, come un file di configurazione gestito da un gestore dotfile o un file con hard link da strumenti come pnpm, il rewind ha lasciato i suoi contenuti intatti. Per annullare le modifiche della sessione ad esso, chiedi a Claude di invertire la modifica o modifica il file tu stesso
* Se non hai creato il link, ispeziona il percorso prima di fidarti dei suoi contenuti: qualcosa ha sostituito il file dopo il checkpoint

<h3 id="no-files-were-restored">
  No files were restored
</h3>

Claude Code mostra questo messaggio quando ripristini il codice con [`/rewind`](/docs/it/checkpointing) e non riesce a ripristinare nessuno dei file in quel checkpoint. Per ogni file, il backup che Claude Code ha salvato prima di modificarlo è mancante, oppure Claude Code non ha potuto scrivere o eliminare il file.

```text theme={null}
Failed to restore the code:
No files were restored: 1 file failed (backup missing, or the file could not be updated)
```

Claude Code elimina i backup di una sessione nella [retention sweep](/docs/it/claude-directory#cleaned-up-automatically), per impostazione predefinita circa 30 giorni dopo l'ultimo salvataggio della sessione. Se riprendi una sessione dopo questo periodo, `/rewind` elenca ancora i suoi checkpoint, ma il ripristino a uno di essi può fallire con questo errore. Se il messaggio dice anche `N paths were skipped for link safety`, vedi [Restored the code, but skipped files](#restored-the-code-but-skipped-files) per quei percorsi.

Quando esegui il fork di una sessione, ad esempio con [`--fork-session`](/docs/it/cli-reference#cli-flags) o [`/branch`](/docs/it/sessions#branch-a-session), Claude Code copia i backup della sessione originale nel fork. Quando Claude Code non riesce a copiare un backup, ad esempio perché il disco è pieno, quel backup è mancante nel fork. Il ripristino a un checkpoint che ne ha bisogno può fallire con questo errore.

**Cosa fare:**

* Annulla le modifiche in un altro modo: chiedi a Claude di invertire le sue modifiche, o ripristina i file dal controllo versione. Quando i backup sono spariti, l'esecuzione di `/rewind` di nuovo fallisce allo stesso modo.
* Se Claude Code non ha potuto scrivere o eliminare un file, correggi ciò che blocca la scrittura, come i permessi dei file, quindi esegui `/rewind` di nuovo.
* Per mantenere i backup più a lungo nelle sessioni future, aumenta [`cleanupPeriodDays`](/docs/it/settings-reference#cleanupperioddays).

Prima della v2.1.260, Claude Code saltava silenziosamente i file i cui backup erano mancanti, e il rewind sembrava avere successo.

<h2 id="session-saving-warnings">
  Avvisi di salvataggio della sessione
</h2>

Claude Code mostra questi avvisi su una riga persistente sotto la casella di input quando non sta salvando la trascrizione della sessione. La sessione continua a funzionare comunque; gli avvisi indicano che la sessione potrebbe mancare da [`--resume`](/docs/it/sessions) in seguito.

<h3 id="transcript-writes-are-failing">
  I salvataggi della trascrizione stanno fallendo
</h3>

Claude Code salva la trascrizione su disco mentre lavori, e i suoi salvataggi nel [file della trascrizione](/docs/it/sessions#where-transcripts-are-stored) stanno fallendo. Il messaggio nomina la causa con il codice di errore sottostante, ad esempio un disco pieno:

```text theme={null}
Transcript writes are failing (disk full — ENOSPC) · recent messages may not be saved for resume
```

L'avviso appare in diversi punti a seconda dell'errore:

* Al primo fallimento per condizioni che non si risolvono da sole: un disco pieno, un quota disco superata, un filesystem di sola lettura, un percorso che supera il limite di lunghezza del filesystem, o, su macOS e Linux, un errore di permesso
* Dopo fallimenti ripetuti che durano almeno un minuto per tutto il resto, inclusi errori di permesso su Windows, dove una scansione antivirus può far fallire un singolo salvataggio che poi riesce al nuovo tentativo

Prima della v2.1.217, Claude Code scartava i salvataggi falliti senza un avviso, e un successivo `--resume` mancante di messaggi recenti era il primo segno.

**Cosa fare:**

* Correggere la condizione che il codice di errore nomina: liberare spazio su disco per `ENOSPC`; aumentare o cancellare la quota per `EDQUOT`; ripristinare l'accesso in scrittura alla posizione della trascrizione per `EACCES`, `EPERM`, o `EROFS`
* L'avviso si cancella da solo al prossimo salvataggio riuscito; non è necessario riavviare
* I messaggi inviati mentre l'avviso era visualizzato potrebbero comunque mancare quando riprendi la sessione in seguito

<h3 id="transcript-saving-is-off-skip-prompt-history">
  Il salvataggio della trascrizione è disattivato perché CLAUDE\_CODE\_SKIP\_PROMPT\_HISTORY è impostato
</h3>

Questa sessione è stata avviata con [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/it/env-vars) impostato, quindi Claude Code non scrive alcuna trascrizione o cronologia dei prompt per essa:

```text theme={null}
Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set · --resume will not find this session; if unintended, unset it and restart
```

La variabile è un'esclusione intenzionale per sessioni script effimere, ma può anche raggiungere una sessione attraverso un profilo shell, uno script wrapper, o un processo padre che l'ha esportata.

**Cosa fare:**

* Se hai impostato la variabile di proposito, non è necessaria alcuna azione; l'avviso conferma che la sessione non apparirà in `--resume`, `--continue`, o nella cronologia della freccia su
* Se non l'hai fatto, rimuovi la variabile dalla shell o dallo script che avvia `claude`, quindi avvia una nuova sessione. I messaggi della sessione corrente non vengono salvati retroattivamente.

<h3 id="transcript-saving-is-off-child-session-marker">
  Il salvataggio della trascrizione è disattivato a causa di un marcatore CLAUDE\_CODE\_CHILD\_SESSION ereditato
</h3>

Claude Code imposta [`CLAUDE_CODE_CHILD_SESSION`](/docs/it/env-vars) nei sottoprocessi che genera, e tratta una sessione interattiva che lo eredita come annidata: Claude Code non salva alcuna trascrizione per essa, quindi le sessioni che Claude stesso avvia non riempiono il tuo elenco `--resume`. Questo avviso significa che la tua sessione corrente ha ereditato il marcatore:

```text theme={null}
Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker · restart with CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1 to keep future transcripts
```

L'avviso è previsto quando hai eseguito `claude` dall'interno di un'altra sessione Claude Code; segnala una classificazione errata quando il marcatore è trapelato attraverso un intermediario di lunga durata, ad esempio un terminale, una sessione `screen`, o un launcher che una sessione Claude Code ha originariamente avviato.

All'interno di tmux, Claude Code rileva un marcatore che è arrivato attraverso l'ambiente globale del server tmux e continua a salvare, quindi questo avviso non appare per quel caso.

**Cosa fare:**

* Se hai avviato questa sessione dall'interno di un'altra sessione Claude Code di proposito, non è necessaria alcuna azione
* Se questa è una sessione di primo livello, esci e riavvia con [`CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1`](/docs/it/env-vars) impostato. Il salvataggio si applica dal riavvio, quindi i messaggi inviati prima non vengono salvati.
* Per correggere i futuri avvii dallo stesso terminale o launcher, rimuovi `CLAUDE_CODE_CHILD_SESSION` dal suo ambiente

<h2 id="configuration-warnings">
  Avvisi di configurazione
</h2>

Claude Code scrive la maggior parte di questi messaggi su stderr, non nella conversazione, e scrive la maggior parte di essi all'avvio. Una voce lo dice quando il suo messaggio appare altrove, ad esempio nel log di debug o come avviso di avvio nella vista della conversazione, o in un altro momento, ad esempio la [riga diagnostica modello non riconosciuto](#unrecognized-model-id-on-a-request) al momento della richiesta.

<h3 id="fullscreen-failed-start-notice">
  Il renderer fullscreen non ha finito di avviarsi
</h3>

Una precedente sessione [fullscreen](/docs/it/fullscreen) su questa macchina è uscita prima di finire di avviarsi, quindi Claude Code avvia questa sessione sul renderer classico e stampa uno di questi avvisi:

```text theme={null}
Claude Code's fullscreen renderer didn't finish starting last time on this machine, so this launch is using the classic renderer. It will try fullscreen again next launch; /tui default keeps the classic renderer.

Claude Code's fullscreen renderer has repeatedly failed to start on this machine, so it has been turned off here. Run /tui fullscreen to try it again (this also resets after an update).
```

**Cosa fare:**

* Seguire [Fullscreen rendering](/docs/it/fullscreen#fullscreen-renderer-didnt-finish-starting). Dice quale avviso ricevete, cosa Claude Code fa nelle sessioni successive, e come provare di nuovo fullscreen o mantenere il renderer classico.
* Se la sessione che è morta ha stampato un messaggio di uscita, vedere [Claude Code è uscito dopo un errore di interfaccia irrecuperabile](#exited-after-an-unrecoverable-interface-error) per quello che nomina.

Prima della v2.1.236, Claude Code non stampava alcun avviso e continuava ad avviare sessioni nel rendering fullscreen dopo un avvio fallito.

<h3 id="exited-after-an-unrecoverable-interface-error">
  Claude Code è uscito dopo un errore di interfaccia irrecuperabile
</h3>

Claude Code stampa questo messaggio quando esce perché la sua interfaccia terminale ha riscontrato un errore da cui non può recuperare, in uno dei due renderer. La seconda frase appare solo quando l'errore si è verificato mentre il renderer [fullscreen](/docs/it/fullscreen) si stava avviando:

```text theme={null}
Claude Code exited after an unrecoverable interface error (<error>). It happened while the fullscreen renderer was starting, so the next launch will use the classic renderer (CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1 forces that any time).
```

**Cosa fare:**

* Avviare di nuovo Claude Code. Per riprendere la conversazione, eseguire `claude --resume` nella stessa directory.
* Se il messaggio nomina il renderer fullscreen, [Fullscreen rendering](/docs/it/fullscreen#fullscreen-renderer-didnt-finish-starting) dice cosa fa il prossimo avvio, che dipende da come avete attivato fullscreen, e come provare di nuovo fullscreen o mantenere il renderer classico.

Prima della v2.1.236, Claude Code usciva senza stampare un messaggio dopo questo tipo di errore.

<h3 id="agent-descriptions-are-over-the-15000-token-limit">
  Le descrizioni degli agenti superano il limite di 15.0k token
</h3>

Claude Code mostra questo avviso come avviso di avvio nella vista della conversazione piuttosto che su stderr. Le descrizioni combinate dei vostri [subagenti](/docs/it/sub-agents), ad eccezione di quelli incorporati, superano 15.000 token come Claude Code le stima. Ogni agente conta il suo nome più il suo frontmatter `description`. Claude Code carica ogni agente indipendentemente dal fatto che il totale superi il limite, quindi l'avviso non cambia cosa viene caricato.

```text theme={null}
Agent descriptions are over the 15.0k-token limit (~16.2k tokens) · ask Claude to trim agent descriptions in .claude/agents/
```

**Cosa fare:**

* Accorciare il frontmatter `description` dei vostri file di agente, o chiedere a Claude di tagliarli per voi.
* Rimuovere i file di agente che non usate più.

<h3 id="workspace-has-not-been-trusted">
  Lo spazio di lavoro non è stato considerato attendibile
</h3>

Claude Code ha trovato regole `permissions.allow` o voci `permissions.additionalDirectories` nel `.claude/settings.json` o `.claude/settings.local.json` del progetto e non le ha applicate, perché [le regole allow dalle impostazioni del progetto richiedono la fiducia dello spazio di lavoro](/docs/it/permissions#project-allow-rules-and-workspace-trust). Il conteggio, il nome dell'impostazione e il file nominato nel messaggio variano con la vostra configurazione. Le regole `deny` e `ask` non sono interessate.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Cosa fare:**

* Eseguire `claude` nella directory e accettare la finestra di dialogo di fiducia. [Project allow rules and workspace trust](/docs/it/permissions#project-allow-rules-and-workspace-trust) dice quale cartella copre tale accettazione.
* In [modalità non interattiva](/docs/it/headless) con `-p` nessuna finestra di dialogo viene mostrata. Impostare la voce `hasTrustDialogAccepted` in `~/.claude.json` usando la chiave `projects` esatta che il messaggio stampa.
* Se il messaggio nomina `.claude/settings.local.json` e avete avviato Claude Code al di fuori di un repository git o nella vostra home directory, aggiornare alla v2.1.200 o successiva. Le versioni 2.1.196 attraverso 2.1.199 hanno trattato il vostro `.claude/settings.local.json` come fornito dal repository in quegli spazi di lavoro. Sulla v2.1.207 e successiva, l'aggiornamento non è sufficiente al di fuori di un repository git se non avete considerato attendibile la cartella: determinare che una cartella non è all'interno di un repository esegue git, e Claude Code esegue quel controllo solo dopo che accettate la finestra di dialogo di fiducia, quindi usate il primo passaggio. La vostra home directory e qualsiasi altra [configuration home](/docs/it/permissions#project-allow-rules-and-workspace-trust) sono esenti e non aspettano la finestra di dialogo. Vedere [Project allow rules and workspace trust](/docs/it/permissions#project-allow-rules-and-workspace-trust).

<h3 id="working-directory-is-a-network-path">
  La directory di lavoro è un percorso di rete
</h3>

Claude Code non aggiunge percorsi di rete come directory di lavoro. Cercare un percorso di rete può contattare l'host che nomina, e su Windows quel contatto può inviare all'host le vostre credenziali, quindi Claude Code rifiuta il percorso senza cercarlo. Vedete questo messaggio quando eseguite `/add-dir` con tale percorso, o come avviso all'avvio. Quando appare all'avvio, Claude Code si avvia senza quella directory.

```text theme={null}
\\server\share is a network path, which cannot be added as a working directory. On Windows, map the share to a drive letter and pass it at launch with --add-dir (a drive letter added mid-session does not yet carry remote-read trust).
```

I percorsi che Claude Code rifiuta in questo modo includono:

* Condivisioni UNC come `\\server\share`
* Percorsi di montaggio automatico come `/net/<host>`, a meno che non abbiate avviato Claude Code da una directory sotto il montaggio automatico di quell'host
* Percorsi locali che raggiungono una posizione di rete attraverso un collegamento simbolico o una giunzione

Le lettere di unità mappate e i percorsi `\\wsl$` non contano come percorsi di rete.

**Cosa fare:**

* Su Windows, mappare la condivisione a una lettera di unità, ad esempio con `net use Z: \\server\share`, e passare l'unità all'avvio con `claude --add-dir Z:\`.
* Su macOS o Linux, montare la condivisione in un percorso locale e aggiungere quel percorso invece.
* Se il percorso è in `permissions.additionalDirectories`, rimuoverlo dal file di impostazioni che lo elenca.

Prima della v2.1.257, Claude Code accettava un percorso di rete raggiungibile come directory di lavoro.

<h3 id="remote-managed-settings-failed-to-load">
  Le impostazioni gestite da remoto non hanno potuto essere caricate
</h3>

La vostra sessione è idonea per [impostazioni gestite dal server](/docs/it/server-managed-settings), ma Claude Code non ha potuto recuperarle, quindi mostra questo avviso nelle sessioni interattive. La causa tra parentesi nomina cosa è fallito, come `network error`, `request timed out`, o `authentication rejected (401)`, e il resto della riga dice quale politica la sessione esegue:

* **Impostazioni memorizzate nella cache da un recupero precedente riuscito**: Claude Code esegue la sessione su quella politica memorizzata nella cache, ad eccezione delle [variabili di ambiente trattenute](/docs/it/server-managed-settings#fetch-and-caching-behavior), e la riga legge `using cached policy`.
* **Nessuna cache**: Claude Code esegue la sessione senza impostazioni gestite dal server, e la riga legge `no remote policy applied`.

**Cosa fare:**

* Agire sulla causa che il messaggio nomina: per una causa di rete, verificare che questa macchina possa raggiungere `api.anthropic.com`; per una causa di autenticazione, controllare il vostro accesso con `/status`
* Eseguire `/status` o `claude doctor` per la diagnostica completa

Prima della v2.1.248, Claude Code segnalava un recupero di impostazioni fallito solo nel log di debug.

<h3 id="managed-settings-were-not-approved">
  Le impostazioni gestite non sono state approvate
</h3>

Le [impostazioni gestite dal server](/docs/it/server-managed-settings) della vostra organizzazione includono impostazioni che necessitano della vostra approvazione, e avete rifiutato la [finestra di dialogo di approvazione della sicurezza](/docs/it/server-managed-settings#security-approval-dialogs), quindi Claude Code esce senza applicarle:

```text theme={null}
Managed settings were not approved; exiting without applying them.
```

**Cosa fare:**

* Avviare di nuovo Claude Code e approvare la finestra di dialogo per continuare secondo le impostazioni della vostra organizzazione. Una finestra di dialogo rifiutata non viene ricordata, quindi appare di nuovo al prossimo avvio.
* Se siete incerti su un'impostazione che la finestra di dialogo elenca, chiedete a chi mantiene le impostazioni gestite della vostra organizzazione prima di approvare

<h3 id="mcp-server-is-blocked-by-enterprise-managed-policy">
  Il server MCP è bloccato dalla politica gestita aziendale
</h3>

Avete selezionato **Reconnect** su un server in `/mcp`, o riattivato un server disabilitato lì, e un'impostazione che [limita i server MCP](/docs/it/managed-mcp) blocca quel server. Claude Code rifiuta di connetterlo e mostra:

```text theme={null}
MCP server <name> is blocked by enterprise managed policy
```

Una qualsiasi di queste impostazioni può produrre il messaggio:

* Una voce [`deniedMcpServers`](/docs/it/managed-mcp#policy-based-control-with-allowlists-and-denylists) che corrisponde al server, inclusa una nel vostro `~/.claude/settings.json` o nel `.claude/settings.json` del progetto
* Un elenco [`allowedMcpServers`](/docs/it/managed-mcp#policy-based-control-with-allowlists-and-denylists) che il server non corrisponde
* [`strictPluginOnlyCustomization`](/docs/it/settings-reference#strictpluginonlycustomization) con `mcp` bloccato, che blocca i server configurati in `~/.claude.json` e `.mcp.json`
* [`disableClaudeAiConnectors`](/docs/it/mcp#disable-claude-ai-connectors), quando il server è un connettore claude.ai

**Cosa fare:**

* Controllare i vostri file di impostazioni utente e progetto per una di queste impostazioni e cambiarla o rimuoverla
* Se nessuna delle vostre impostazioni spiega il blocco, chiedete al vostro amministratore quale impostazione gestita blocca il server

Prima della v2.1.257, **Reconnect** e ri-abilitare in `/mcp` potevano connettere un server che un aggiornamento di politica mid-session bloccava.

<h3 id="managed-settings-document-could-not-be-parsed">
  Il documento delle impostazioni gestite non ha potuto essere analizzato
</h3>

La vostra organizzazione distribuisce [impostazioni gestite](/docs/it/managed-settings), e uno dei documenti distribuiti è presente ma non può essere analizzato come un oggetto JSON, quindi Claude Code esce con codice 1 all'avvio invece di eseguire senza la politica che il documento contiene. La riga nomina la fonte fallita prima del messaggio:

```text theme={null}
/Library/Application Support/ClaudeCode/managed-settings.json: Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.
```

La fonte è una di:

* Il percorso del file `managed-settings.json` o un file drop-in sotto `managed-settings.d`
* Il profilo delle preferenze gestite macOS, `per-user managed preferences` o `device-level managed preferences`
* Il valore del registro Windows, `Registry: HKLM\SOFTWARE\Policies\ClaudeCode\Settings`

[Find entries Claude Code dropped](/docs/it/managed-settings#find-entries-claude-code-dropped) elenca cosa rende ogni fonte non analizzabile.

Claude Code rifiuta di avviarsi anche quando un'altra fonte amministrativa fornisce una politica valida. Vedete questo errore nelle sessioni interattive, `claude -p`, sessioni Agent SDK, [sessioni in background](/docs/it/agent-view), e la maggior parte dei sottocomandi, `claude doctor` incluso. Il rifiuto fallisce chiuso di proposito: le impostazioni in un documento che Claude Code non può analizzare non possono essere applicate, e avviarsi comunque eseguirebbe sessioni senza i controlli dell'organizzazione.

Un problema di schema in un documento analizzabile non produce questo errore. [Find entries Claude Code dropped](/docs/it/managed-settings#find-entries-claude-code-dropped) copre cosa Claude Code fa con uno.

Quando una directory `managed-settings.d/` esiste ma non può essere elencata, Claude Code segnala `Managed settings drop-in directory could not be read:` seguito dall'errore sottostante invece. [Find entries Claude Code dropped](/docs/it/managed-settings#find-entries-claude-code-dropped) copre quando un errore di lettura esce all'avvio.

**Cosa fare:**

* Se amministrate la macchina, correggere il documento nominato in modo che si analizzi come un oggetto JSON, o rimuovere il file, il profilo o il valore del registro. Un `managed-settings.json` vuoto conta come `{}` e non blocca l'avvio.
* Se non lo fate, chiedete al vostro amministratore di correggere il documento distribuito. Nulla nei vostri file di impostazioni causa o cancella questo errore.

<h3 id="otelheadershelper-failed">
  otelHeadersHelper non riuscito
</h3>

Claude Code mostra questo avviso come una notifica nell'interfaccia terminale, una volta per sessione interattiva, quando lo script [`otelHeadersHelper`](/docs/it/settings-reference#otelheadershelper) fallisce o stampa output che non soddisfa i [requisiti dello script](/docs/it/monitoring-usage#script-requirements).

Mentre lo script continua a fallire, le esportazioni falliscono e il vostro backend di telemetria non riceve nulla dalla sessione.

Il testo dopo `See /status:` dice cosa è fallito, come il codice di uscita dello script seguito dal suo output di errore:

```text theme={null}
otelHeadersHelper failed; telemetry is not being exported. See /status: exited 1: token service unreachable
```

**Cosa fare:**

* Eseguire `/status` per leggere il dettaglio del fallimento.
* Correggere lo script in modo che esca 0 entro 30 secondi e stampi un oggetto JSON di valori di intestazione stringa su stdout. Vedere [requisiti dello script](/docs/it/monitoring-usage#script-requirements).
* Se la vostra organizzazione distribuisce lo script attraverso [impostazioni gestite](/docs/it/managed-settings), chiedete a chi le mantiene di correggerlo.

In [modalità non interattiva](/docs/it/headless) con `-p`, lo stesso fallimento appare su stderr come `otelHeadersHelper failed (OpenTelemetry export headers unavailable): <error>` invece.

<h3 id="headershelper-not-run">
  headersHelper non eseguito
</h3>

Claude Code ha connesso un server MCP con i suoi `headers` statici soli e ha saltato il [`headersHelper`](/docs/it/mcp#use-dynamic-headers-for-custom-authentication) del server, perché l'helper è un comando shell e la cartella non ha fiducia salvata. Una cartella ottiene fiducia salvata quando impostate la sua voce in `~/.claude.json` a mano o, al di fuori della vostra home directory, quando accettate la finestra di dialogo di fiducia per essa in una sessione interattiva. Vedere [Trust a folder before its headersHelper runs](/docs/it/mcp#trust-a-folder-before-its-headershelper-runs) per quali server questo controllo si applica.

Claude Code scrive questa riga in [modalità non interattiva](/docs/it/headless) solo, una volta per server. In una sessione interattiva scrive lo stesso rifiuto al log di debug invece.

```text theme={null}
MCP server 'internal-api': headersHelper not run — this workspace has no persisted trust; accept the trust dialog here once interactively, or set projects["/Users/you/project"].hasTrustDialogAccepted in /Users/you/.claude.json.
```

La chiave `projects` che il messaggio stampa è la cartella [Project allow rules and workspace trust](/docs/it/permissions#project-allow-rules-and-workspace-trust) dice Claude Code chiavi la fiducia su. Accettare la finestra di dialogo di fiducia per una cartella genitore non soddisfa il controllo, e una sessione `-p` o SDK non la soddisfa nemmeno.

**Cosa fare:**

* Eseguire `claude` nella cartella che il messaggio nomina, accettare la finestra di dialogo di fiducia, quindi eseguire di nuovo il vostro comando `-p` o SDK
* Impostare la voce `hasTrustDialogAccepted` in `~/.claude.json` voi stessi, usando la chiave `projects` esatta che il messaggio stampa
* Se avete avviato la sessione nella vostra home directory, lavorare da una directory di progetto che avete considerato attendibile. Quando accettate la finestra di dialogo di fiducia nella vostra home directory, Claude Code mantiene quella fiducia per la sessione corrente solo.

<h3 id="malformed-tool-content-rule">
  Regola Tool(content) malformata
</h3>

Una [regola di permesso](/docs/it/permissions#permission-rule-syntax) in uno dei vostri file di impostazioni non ha la forma `Tool` o `Tool(content)`, ad esempio perché il testo segue la parentesi di chiusura o una delle parentesi manca. Claude Code salta la regola e la elenca nella finestra di dialogo delle impostazioni non valide quando una sessione interattiva si avvia, e nell'output di [`claude doctor`](/docs/it/debug-your-config#check-resolved-settings):

```text theme={null}
Invalid permission rule "Bash(ls) x" was skipped: Malformed Tool(content) rule. Rules take the form Tool or Tool(content) and must end at the closing ")"; parentheses inside the content are literal
```

**Cosa fare:**

* Nel file di impostazioni elencato con il messaggio, riscrivere la regola in modo che termini alla sua parentesi di chiusura, ad esempio `Bash(ls *)` al posto di `Bash(ls) x`
* Lasciare le parentesi all'interno del contenuto come sono. Sono letterali, quindi una regola come `Edit(./Finance (2024)/**)` è valida senza escape

Prima della v2.1.260, Claude Code segnalava una regola con parentesi non abbinate come `Mismatched parentheses`.

<h3 id="is-not-matched-by-file-permission-checks">
  Non è abbinato dai controlli di permesso dei file
</h3>

Claude Code ha trovato una regola di permesso `Write`, `NotebookEdit`, `MultiEdit`, o `Glob` [permission rule](/docs/it/permissions#read-and-edit) con un percorso in uno dei vostri [file di impostazioni](/docs/it/settings#where-settings-live), in [impostazioni gestite](/docs/it/managed-settings), o in un valore di flag `--allowedTools`, `--disallowedTools`, o `--settings`. Controlla i permessi dei file rispetto alle regole `Edit` e `Read` solo, quindi non consulta mai una regola di percorso che nomina uno degli altri strumenti di file. Mantiene la regola e non cambia nient'altro; l'avviso nomina la regola, la sua fonte tra parentesi, e la sostituzione da scrivere:

```text theme={null}
Permission deny rule (.claude/settings.json): Write(docs/**) is not matched by file permission checks — only Edit(path) rules are. Use Edit(docs/**) instead (Edit rules cover all file-editing tools).
```

**Cosa fare:**

* Sostituire le regole `Write(path)`, `NotebookEdit(path)`, e legacy `MultiEdit(path)` con `Edit(path)`. Le regole `Edit` coprono tutti gli strumenti di modifica dei file.
* Ad eccezione di `--allowedTools`, dove Claude Code accetta una regola `Glob` senza avviso, sostituire le regole `Glob(path)` con `Read(path)`.
* Correggere la regola alla fonte che l'avviso nomina tra parentesi: un percorso di file di impostazioni, o il flag stesso per `--allowed-tools` e `--disallowed-tools`. Un percorso `claude-settings-<hash>.json` che non esiste su disco rappresenta un valore `--settings` inline. Correggere il JSON che passate a quel flag.
* Lasciare sole le regole di nome di strumento nudo come `Write` o `Glob`. Claude Code le abbina a livello di [tool level](/docs/it/permissions#match-all-uses-of-a-tool) e non avvisa su di esse.
* Se la fonte legge `managed policy settings`, inoltrare l'avviso a chi mantiene le vostre impostazioni gestite, poiché non potete cancellarlo voi stessi.

In una [sessione in background](/docs/it/agent-view) o con `--output-format json` o `stream-json`, Claude Code scrive l'avviso al log di debug invece di stderr, quindi l'output letto dalla macchina rimane pulito. Eseguire con `--debug` per catturarlo in `~/.claude/debug/<session-id>.txt`. Prima della v2.1.210, Claude Code accettava queste regole senza un avviso.

<h3 id="has-a-wildcard-before-the-rest-of-the-command">
  Ha un carattere jolly prima del resto del comando
</h3>

Claude Code ha trovato una regola allow `Bash` il cui `*` viene prima di una parola successiva che determina quale comando è, come `Bash(git * main)` o `Bash(git -C * status *)`, in uno dei vostri [file di impostazioni](/docs/it/settings#where-settings-live), in [impostazioni gestite](/docs/it/managed-settings), o in un valore di flag `--allowedTools` o `--settings`. Il `*` corrisponde a qualsiasi testo, incluse le opzioni inserite in quella posizione: `Bash(git * main)` approva anche `git -c core.fsmonitor=<script> diff main`, dove `-c` fa eseguire a git un programma che il comando nomina. [Wildcard patterns](/docs/it/permissions#wildcard-patterns) mostra le regole di corrispondenza.

L'avviso esiste in modo che possiate restringere una regola il cui carattere jolly è più ampio di quanto intendete. Claude Code mantiene la regola e non cambia nulla su come corrisponde; l'avviso nomina la regola e la sua fonte tra parentesi:

```text theme={null}
Permission allow rule (.claude/settings.json): Bash(git -C * status *) has a wildcard before the rest of the command, so it also matches any options inserted at that position and approves them without a prompt. For git, options such as -c and --exec-path can run arbitrary commands. Replace that * with the exact value you mean, or only use * after the subcommand (for example Bash(git status *)).
```

**Cosa fare:**

* Sostituire il `*` prima del sottocomando con il valore esatto che intendete: `Bash(git checkout main)` al posto di `Bash(git * main)`.
* Spostare ogni `*` dopo il sottocomando: `Bash(git status *)` al posto di `Bash(git -C * status *)`. Scrivere una regola per sottocomando che volete permettere.
* Correggere la regola alla fonte che l'avviso nomina tra parentesi: un percorso di file di impostazioni, o il flag `--allowed-tools` stesso. Un percorso `claude-settings-<hash>.json` che non esiste su disco rappresenta un valore `--settings` inline. Correggere il JSON che passate a quel flag.
* Se la fonte legge `managed policy settings`, inoltrare l'avviso a chi mantiene le vostre impostazioni gestite, poiché non potete cancellarlo voi stessi.

Claude Code non avvisa su regole deny e ask con la stessa forma: rifiuta o chiede i comandi extra che corrispondono piuttosto che approvarli. Non avvisa nemmeno su regole il cui sottocomando viene prima del primo `*`, come `Bash(git commit *)`, o regole in cui nessuna parola diversa da un'opzione segue il `*`, come `Bash(git *)`, o su regole di prefisso `:*` come `Bash(git:*)`.

In una [sessione in background](/docs/it/agent-view) o con `--output-format json` o `stream-json`, Claude Code scrive l'avviso al log di debug invece di stderr, quindi l'output letto dalla macchina rimane pulito. Eseguire con `--debug` per catturarlo in `~/.claude/debug/<session-id>.txt`. Prima della v2.1.246, Claude Code accettava queste regole senza un avviso.

<h3 id="crosssessioninbound-must-be-one-of-accept-hold-refuse">
  crossSessionInbound deve essere uno di accept, hold, refuse
</h3>

Un file di impostazioni imposta [`crossSessionInbound`](/docs/it/settings-reference#crosssessioninbound) a un valore che Claude Code non riconosce, come il typo `"reject"`. La seconda frase dell'avviso dipende da quale file contiene il valore; in un file utente, progetto, locale, o `--settings` legge:

```text theme={null}
"crossSessionInbound" must be one of "accept", "hold", "refuse"; received "reject". This value was ignored; while it is present, cross-session messages are held for your approval instead of being delivered. Set it to one of the values above.
```

In [impostazioni gestite](/docs/it/managed-settings), Claude Code tratta il valore non riconosciuto come `refuse`, il valore più restrittivo, e l'avviso dice che i messaggi cross-session vengono rifiutati fino a quando un amministratore non lo corregge. Per come l'hold si combina con i valori nei vostri altri file di impostazioni, vedere [`crossSessionInbound`](/docs/it/settings-reference#crosssessioninbound).

**Cosa fare:**

* Impostare la chiave a `"accept"`, `"hold"`, o `"refuse"`, o rimuoverla
* Quando l'avviso nomina impostazioni gestite, chiedere all'amministratore di correggere il valore

Prima della v2.1.248, Claude Code ignorava un valore non riconosciuto senza avviso.

<h3 id="the-200k-limit-isnt-enforced">
  Il limite di 200K non è applicato
</h3>

Avete impostato [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/it/env-vars), che normalmente fa sì che [auto-compaction](/docs/it/model-config#default-auto-compact-thresholds) mantenga le sessioni su modelli con contesto 1M a una finestra di 200K, ma nessuna soglia di compattazione limita questa sessione a o sotto 200K, quindi la conversazione può crescere oltre.

```text theme={null}
CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced for <model>, so this session can grow past it. To enforce it, set CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000 (or the autoCompactWindow setting).
```

Claude Code applica il limite di 200K da solo per ogni modello che riconosce come avente una finestra nativa di 1M, e per ID di modello che non riconosce compatta alla finestra che assume. L'avviso appare quando altra configurazione sconfigge tale applicazione:

* L'ID del modello non è uno che Claude Code riconosce, come un alias di [LLM gateway](/docs/it/llm-gateway), e avete impostato [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/it/env-vars) o aumentato la finestra assunta oltre 200K con [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/it/env-vars). In questo caso il messaggio offre anche `or update to a Claude Code version that recognizes <model>` come rimedio.
* Un beta `context-1m` richiesto attraverso [`ANTHROPIC_BETAS`](/docs/it/env-vars) o il flag [`--betas`](/docs/it/cli-reference#cli-flags) chiede ancora all'API la finestra 1M su un modello che accetta quel beta, mentre nulla compatta la sessione a 200K

**Cosa fare:**

* Impostare [`CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000`](/docs/it/env-vars), o l'impostazione [`autoCompactWindow`](/docs/it/settings-reference#autocompactwindow) a `200000`, in modo che auto-compaction compatti al confine di 200K
* Se il messaggio nomina un ID di modello che questa versione non riconosce, eseguire `claude update`. Una versione che riconosce l'ID come modello con contesto 1M applica il limite senza ulteriore configurazione.
* Se volete che la sessione usi la finestra completa del modello invece, disimpostare `CLAUDE_CODE_DISABLE_1M_CONTEXT`; l'avviso segnala solo che il limite di 200K non è applicato

In una [sessione in background](/docs/it/agent-view) o con `--output-format json` o `stream-json`, Claude Code scrive l'avviso al log di debug invece di stderr.

<h3 id="unrecognized-model-id-on-a-request">
  ID modello non riconosciuto su una richiesta
</h3>

Claude Code ha inviato una richiesta per un ID di modello che la vostra versione di Claude Code non riconosce, e non ha trovato alcuna voce [`modelOverrides`](/docs/it/model-config#override-model-ids-per-version) che mappa quell'ID a un modello che riconosce. Claude Code invia comunque la richiesta con l'ID come lo avete configurato, e non esce o cambia modelli.

```text theme={null}
[claude-code:unrecognized_model] {"model":"my-proxy-model","query_source":"sdk"}
```

In uno script o harness che legge stderr, abbinare il prefisso `[claude-code:unrecognized_model]`. Dopo il prefisso e uno spazio, Claude Code scrive un oggetto JSON su una riga. Claude Code può aggiungere campi ad esso in una versione successiva, quindi ignorare qualsiasi campo che non vi aspettate. Scrive almeno questi due:

* `model`: la stringa del modello come l'avete configurata
* `query_source`: il percorso della richiesta che ha usato il modello. Claude Code segnala `sdk` per un'esecuzione `-p` e un valore che inizia con `agent:` per un subagente.

Claude Code scrive la riga in uno di due posti, a seconda di come la eseguite:

* In [modalità non interattiva](/docs/it/headless) con `-p`, Claude Code la scrive su stderr sotto ogni `--output-format`, in modo che possiate analizzare stdout senza filtrare la riga
* In una sessione interattiva o una [sessione in background](/docs/it/agent-view), Claude Code la scrive al log di debug invece; eseguire con `--debug` per catturarla in `~/.claude/debug/<session-id>.txt`

Claude Code scrive la riga una volta per stringa di modello per processo. Scrive una riga separata per ogni ulteriore ID non riconosciuto, come uno che un [subagente](/docs/it/sub-agents#choose-a-model) o [funzionalità in background](/docs/it/costs#background-token-usage) usa.

Claude Code non scrive la riga per ID di provider che risolve a un modello che riconosce, come ID Amazon Bedrock `us.anthropic.claude-...`, ID di Google Cloud's Agent Platform con un suffisso di versione `@`, e nomi di distribuzione Microsoft Foundry che contengono un ID di modello Claude. Claude Code controlla il modello dietro un [ARN del profilo di inferenza dell'applicazione](/docs/it/amazon-bedrock#map-each-model-version-to-an-inference-profile) di Amazon Bedrock piuttosto che l'ARN stesso. Non scrive alcuna riga per un ARN che non può risolvere, come uno digitato male.

**Cosa fare:**

* Se avete impostato l'ID di proposito, come un alias di [LLM gateway](/docs/it/llm-gateway), aggiungere una voce [`modelOverrides`](/docs/it/model-config#override-model-ids-per-version) al vostro [file di impostazioni](/docs/it/settings#where-settings-live) con l'ID come suo valore. Usare un ID di modello Anthropic come chiave, non un alias di famiglia come `opus`. Per `my-proxy-model` dalla riga di esempio, aggiungere questa voce:

  ```json theme={null}
  {
    "modelOverrides": {
      "claude-opus-4-6": "my-proxy-model"
    }
  }
  ```

  Claude Code allora tratta `my-proxy-model` come `claude-opus-4-6` e smette di scrivere la riga.

* Se l'ID nomina un modello più nuovo della vostra versione di Claude Code, eseguire `claude update`

* Se l'ID è un typo, correggerlo in quale dei [posti dove potete impostare un modello](/docs/it/model-config#setting-your-model) o [variabili di alias](/docs/it/model-config#environment-variables) lo contiene. Se `query_source` inizia con `agent:`, correggerlo dove impostate il [modello del subagente](/docs/it/sub-agents#choose-a-model) invece.

Prima della v2.1.233, Claude Code non scriveva alcuna riga quando inviava una richiesta per un ID di modello che non riconosceva.

<h3 id="stale-sandbox-mask-files-left-by-a-killed-session">
  File di maschera sandbox stantii lasciati da una sessione uccisa
</h3>

`claude doctor` stampa questo avviso nei suoi diagnostici, e `/status` elenca la stessa riga. Appare su Linux e WSL2 quando [sandboxing](/docs/it/sandboxing) è abilitato con isolamento del filesystem attivo.

Mentre un comando sandboxato esegue, la sandbox mantiene un rifiuto di scrittura su un file che non esiste ancora creando un segnaposto di lettura sola di 0 byte lì, e lo rimuove dopo. Una sessione uccisa prima che quella pulizia esegua, ad esempio da SIGKILL, lascia i segnaposti dietro. Sessioni successive li legano di sola lettura di nuovo ad ogni avvio, quindi una scrittura di impostazioni come salvare "Sì, e non chiedere di nuovo" fallisce dove uno siede.

```text theme={null}
- Stale sandbox mask files left by a killed session: /home/you/project/.claude/settings.local.json
  Fix: Remove each with `rm <path>` while no other Claude Code session is running in that project — a 0-byte read-only file where a settings file belongs makes "Yes, and don't ask again" fail to save, and the sandbox binds it read-only again on every start
```

**Cosa fare:**

* Chiudere qualsiasi altra sessione di Claude Code in esecuzione in quel progetto, quindi eliminare ogni file elencato con `rm`. L'avviso nomina fino a tre file e conta il resto, quindi rieseguire `claude doctor` dopo l'eliminazione fino a quando l'avviso non appare più. Un segnaposto che la sandbox di un'altra sessione sta ancora usando è una parte viva della protezione di scrittura di quella sessione
* Se una scelta di permesso che avete salvato con "Sì, e non chiedere di nuovo" non è rimasta, salvarla di nuovo dopo aver eliminato il segnaposto

Prima della v2.1.257, `claude doctor` non contrassegnava questi file; le versioni precedenti lasciano gli stessi segnaposti dietro quando una sessione viene uccisa.

<h2 id="responses-seem-lower-quality-than-usual">
  Le risposte sembrano di qualità inferiore al solito
</h2>

Se le risposte di Claude sembrano meno capaci di quanto ti aspetti ma non viene mostrato alcun errore, la causa è solitamente lo stato della conversazione piuttosto che il modello stesso. Claude Code non cambia silenziosamente le versioni del modello. Può passare a un modello di fallback in tre casi specifici:

* Un [`--fallback-model`](/docs/it/cli-reference#cli-flags) configurato subentra dopo un errore di disponibilità, solo per quel turno, con un avviso nella trascrizione
* Un controllo di avvio di Amazon Bedrock o della piattaforma Agent di Google Cloud trova il tuo modello predefinito non disponibile
* Il [fallback automatico del modello](/docs/it/model-config#automatic-model-fallback) su Fable 5.1, Fable 5, Opus 5.5 e Opus 5 sposta la sessione al modello di fallback della categoria contrassegnata, quando quella categoria ne ha uno, e mostra un avviso nella trascrizione

Il controllo della selezione del modello di seguito cattura il secondo e il terzo caso; il primo appare come un avviso nella trascrizione piuttosto che come un cambio `/model`. La [configurazione del modello](/docs/it/model-config) spiega quando si applica ogni fallback.

Controlla prima questi elementi:

* **Selezione del modello**: esegui `/model` per confermare che sei sul modello che ti aspetti. Una scelta `/model` precedente o una variabile di ambiente `ANTHROPIC_MODEL` potrebbe averti messo su un modello più piccolo di quello che intendevi.
* **Livello di sforzo**: esegui `/effort` per controllare il livello di ragionamento attuale e aumentarlo per il debug difficile o il lavoro di progettazione. I valori predefiniti variano in base al modello, quindi controlla prima di assumere che sei al di sotto del massimo. Vedi [Regola il livello di sforzo](/docs/it/model-config#adjust-effort-level) per i valori predefiniti per modello e il collegamento `ultrathink`.
* **Pressione del contesto**: esegui `/context` per vedere quanto è pieno il window. Se è vicino alla capacità, esegui `/compact` a un punto naturale o `/clear` per ricominciare da capo. Vedi [Esplora la finestra di contesto](/docs/it/context-window) per come auto-compact influisce sui turni precedenti.
* **Istruzioni obsolete**: file `CLAUDE.md` grandi o obsoleti e definizioni di strumenti MCP consumano contesto e possono indirizzare le risposte. Il controllo `/doctor` contrassegna i file di memoria sovradimensionati e le estensioni inutilizzate, e `/context` mostra l'utilizzo dei token degli strumenti MCP. Prima della v2.1.205, `/doctor` apriva una schermata di diagnostica che contrassegnava i file di memoria sovradimensionati e le definizioni dei subagent.

Quando una risposta va male, il rewind di solito funziona meglio che rispondere con correzioni. Premi Esc due volte o esegui `/rewind` per tornare indietro prima del turno sbagliato, quindi riformula il prompt con più specifiche. Correggere nel thread mantiene il tentativo sbagliato nel contesto, il che può ancorare le risposte successive ad esso. Vedi [Checkpointing](/docs/it/checkpointing).

Se la qualità sembra ancora non corretta dopo aver controllato quanto sopra, esegui `/feedback` e descrivi cosa ti aspettavi rispetto a quello che hai ottenuto. Il feedback inviato in questo modo include la trascrizione della conversazione, che è il modo più veloce per Anthropic per diagnosticare una vera regressione. Vedi [Segnala un errore](#report-an-error) se `/feedback` non è disponibile nel tuo ambiente.

Se Claude ti avverte di un sospetto prompt injection, o rifiuta una richiesta a causa di un sospetto injection, e il testo che l'avviso nomina è contesto che Claude Code aggiunge automaticamente alla conversazione piuttosto che contenuto di file o web, esegui `claude update` e riprova. Se l'avviso si ripete dopo l'aggiornamento, [segnalalo](#report-an-error) piuttosto che incollare il contenuto contrassegnato di nuovo nel prompt. Prima della v2.1.201, Sonnet 5 rifiutava alcune richieste allo stesso modo.

<h2 id="report-an-error">
  Segnalare un errore
</h2>

Per gli errori dei componenti non trattati in questa pagina, consultare la guida pertinente:

* Il server MCP non è riuscito a connettersi o autenticarsi: [MCP](/docs/it/mcp)
* Lo script hook non è riuscito o ha bloccato uno strumento: [Debug hooks](/docs/it/hooks#debug-hooks)
* Permesso negato o errori del filesystem durante l'installazione: [Risoluzione dei problemi di installazione e accesso](/docs/it/troubleshoot-install)

Se un errore non è elencato qui o la correzione suggerita non aiuta:

* Eseguire `/feedback` all'interno di Claude Code per inviare la trascrizione e una descrizione ad Anthropic. Il comando offre anche di aprire un problema GitHub precompilato. L'invio ad Anthropic richiede l'[autenticazione](/docs/it/authentication). Su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e altri provider di terze parti, o quando non sono configurate credenziali Anthropic, `/feedback` salva un archivio locale che è possibile inviare al rappresentante dell'account Anthropic.
* Eseguire `claude doctor` dalla shell per una diagnostica di sola lettura dell'installazione, oppure eseguire il checkup `/doctor` all'interno di Claude Code per trovare e risolvere i problemi di configurazione
* Controllare [status.claude.com](https://status.claude.com) per gli incidenti attivi
* Cercare i [problemi esistenti](https://github.com/anthropics/claude-code/issues) su GitHub
