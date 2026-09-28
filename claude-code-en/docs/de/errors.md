> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Fehlerreferenz

> Schlagen Sie Claude Code-Laufzeitfehlermeldungen nach und erfahren Sie, was jede bedeutet und wie Sie sie beheben.

Diese Seite listet Laufzeitfehler auf, die Claude Code anzeigt, und wie Sie sich von jedem erholen können. Außerdem erfahren Sie, was Sie überprüfen sollten, wenn Antworten ohne Fehler seltsam wirken. Für Installationsfehler wie `command not found` oder TLS-Fehler während des Setups siehe [Troubleshoot installation and login](/docs/de/troubleshoot-install).

Mit Ausnahme von [Wrapper and IDE errors](#wrapper-and-ide-errors), die das startende Programm ausgibt und nicht Claude Code selbst, gelten diese Fehler und Wiederherstellungsbefehle über die CLI, die [Desktop-App](/docs/de/desktop) und [Claude Code im Web](/docs/de/claude-code-on-the-web), da alle drei die gleiche Claude Code CLI verwenden. Für andere oberflächenspezifische Probleme siehe den Abschnitt zur Fehlerbehebung auf der Seite dieser Oberfläche.

<Note>
  Claude Code ruft die Claude API für Modellantworten auf, daher werden die meisten Laufzeitfehler einem zugrunde liegenden API-Fehlercode zugeordnet. Diese Seite behandelt, was jeder Fehler in Claude Code bedeutet und wie Sie sich davon erholen. Für die rohen HTTP-Statuscode-Definitionen siehe die [Claude Platform error reference](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Fehler finden
</h2>

Ordnen Sie die angezeigte Meldung einem Abschnitt unten zu.

| Meldung                                                                                                                                                                                                                                                              | Abschnitt                                                                                                                        |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `API Error: 500 Internal server error`                                                                                                                                                                                                                               | [Serverfehler](#api-error-500-internal-server-error)                                                                             |
| `API Error: Repeated 529 Overloaded errors`                                                                                                                                                                                                                          | [Serverfehler](#api-error-repeated-529-overloaded-errors)                                                                        |
| `Request timed out`                                                                                                                                                                                                                                                  | [Serverfehler](#request-timed-out), oder [Netzwerk](#unable-to-connect-to-api), wenn die Meldung Ihre Internetverbindung erwähnt |
| `API Error: No response from API`                                                                                                                                                                                                                                    | [Serverfehler](#no-response-from-api)                                                                                            |
| `Server error mid-response. The response above may be incomplete.`                                                                                                                                                                                                   | [Serverfehler](#the-response-above-may-be-incomplete)                                                                            |
| `Connection lost mid-response` / `Your computer went to sleep mid-response` / `The response stopped arriving`                                                                                                                                                        | [Serverfehler](#the-response-above-may-be-incomplete)                                                                            |
| `Connection closed mid-response` / `Response stalled mid-stream`                                                                                                                                                                                                     | [Serverfehler](#the-response-above-may-be-incomplete)                                                                            |
| `Connection lost before a response was produced` / `Your computer went to sleep before a response was produced` / `The response stalled before a response was produced`                                                                                              | [Automatische Wiederholungen](#automatic-retries)                                                                                |
| `Connection closed while thinking` / `Response stalled while thinking`                                                                                                                                                                                               | [Automatische Wiederholungen](#automatic-retries)                                                                                |
| `Connection lost while your computer was asleep`                                                                                                                                                                                                                     | [Automatische Wiederholungen](#automatic-retries)                                                                                |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`                                                                                                                                                                                 | [Serverfehler](#auto-mode-cannot-determine-the-safety-of-an-action)                                                              |
| `Auto mode could not evaluate this action and is blocking it for safety`                                                                                                                                                                                             | [Serverfehler](#auto-mode-cannot-determine-the-safety-of-an-action)                                                              |
| `Auto mode classifier transcript exceeded context window`                                                                                                                                                                                                            | [Serverfehler](#auto-mode-cannot-determine-the-safety-of-an-action)                                                              |
| `Agent aborted: auto mode classifier request refused by the safety safeguard`                                                                                                                                                                                        | [Serverfehler](#auto-mode-cannot-determine-the-safety-of-an-action)                                                              |
| `The server-side auto mode classifier gave no verdict`                                                                                                                                                                                                               | [Serverfehler](#the-server-returned-no-safety-verdict)                                                                           |
| `Auto mode is unavailable — the server returned no safety verdict for the last 10 responses`                                                                                                                                                                         | [Serverfehler](#the-server-returned-no-safety-verdict)                                                                           |
| `Agent terminated early due to an API error`                                                                                                                                                                                                                         | [Serverfehler](#agent-terminated-early-due-to-an-api-error)                                                                      |
| `You've hit your session limit` / `You've hit your weekly limit` / `You've hit your Opus limit` / `You've hit your Sonnet limit`                                                                                                                                     | [Nutzungslimits](#youve-hit-your-session-limit)                                                                                  |
| `Usage credits required for 1M context`                                                                                                                                                                                                                              | [Nutzungslimits](#usage-credits-required-for-1m-context)                                                                         |
| `the prompt to confirm went unanswered — nothing was sent`                                                                                                                                                                                                           | [Nutzungslimits](#the-prompt-to-confirm-went-unanswered)                                                                         |
| `Server is temporarily limiting requests`                                                                                                                                                                                                                            | [Nutzungslimits](#server-is-temporarily-limiting-requests)                                                                       |
| `Request rejected (429)`                                                                                                                                                                                                                                             | [Nutzungslimits](#request-rejected-429)                                                                                          |
| `Credit balance is too low`                                                                                                                                                                                                                                          | [Nutzungslimits](#credit-balance-is-too-low)                                                                                     |
| `You've hit your monthly spend limit` / `You've hit your individual spend limit` / `You've hit your org's monthly spend limit` / `You've hit your channel's monthly spend limit` / `You've hit your team's shared budget` / `You've hit your individual usage limit` | [Nutzungslimits](#youve-hit-your-monthly-spend-limit)                                                                            |
| `Could not update your spend limit`                                                                                                                                                                                                                                  | [Nutzungslimits](#could-not-update-your-spend-limit)                                                                             |
| `spend limit reached` / `spend limit unavailable`                                                                                                                                                                                                                    | [Nutzungslimits](#spend-limit-reached)                                                                                           |
| `Not logged in · Please run /login`                                                                                                                                                                                                                                  | [Authentifizierung](#not-logged-in)                                                                                              |
| `Could not resolve authentication method`                                                                                                                                                                                                                            | [Authentifizierung](#could-not-resolve-authentication-method)                                                                    |
| `Invalid API key`                                                                                                                                                                                                                                                    | [Authentifizierung](#invalid-api-key)                                                                                            |
| `Your apiKeyHelper script is failing`                                                                                                                                                                                                                                | [Authentifizierung](#your-apikeyhelper-script-is-failing)                                                                        |
| `Invalid auth token · Fix external auth token`                                                                                                                                                                                                                       | [Authentifizierung](#invalid-request-header-value)                                                                               |
| `Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable`                                                                                                                                                                                                    | [Authentifizierung](#invalid-request-header-value)                                                                               |
| `Invalid request header from the environment · Fix the environment variable`                                                                                                                                                                                         | [Authentifizierung](#invalid-request-header-value)                                                                               |
| `This organization has been disabled`                                                                                                                                                                                                                                | [Authentifizierung](#this-organization-has-been-disabled)                                                                        |
| `Your organization has disabled API key authentication`                                                                                                                                                                                                              | [Authentifizierung](#your-organization-has-disabled-api-key-authentication)                                                      |
| `Your organization has disabled Claude subscription access`                                                                                                                                                                                                          | [Authentifizierung](#your-organization-has-disabled-claude-subscription-access)                                                  |
| `Routines are disabled by your organization's policy`                                                                                                                                                                                                                | [Authentifizierung](#routines-are-disabled-by-your-organizations-policy)                                                         |
| `Remote Control is only available when using Claude via api.anthropic.com`                                                                                                                                                                                           | [Authentifizierung](#remote-control-requires-the-anthropic-api)                                                                  |
| `OAuth token refresh failed — run /login to re-authenticate`                                                                                                                                                                                                         | [Authentifizierung](#remote-control-couldnt-refresh-your-login)                                                                  |
| `JWT refresh failed: no OAuth token — run /login`                                                                                                                                                                                                                    | [Authentifizierung](#remote-control-couldnt-refresh-your-login)                                                                  |
| `Claude.ai login expired`                                                                                                                                                                                                                                            | [Authentifizierung](#remote-control-couldnt-refresh-your-login)                                                                  |
| `Claude.ai login was rejected — run /login, then /remote-control`                                                                                                                                                                                                    | [Authentifizierung](#remote-control-couldnt-refresh-your-login)                                                                  |
| `OAuth token unavailable — run /login to restore Remote Control`                                                                                                                                                                                                     | [Authentifizierung](#remote-control-couldnt-refresh-your-login)                                                                  |
| `Signed out of Claude — run /login, then /remote-control`                                                                                                                                                                                                            | [Authentifizierung](#remote-control-couldnt-refresh-your-login)                                                                  |
| `signed-in claude.ai account or organization changed on this machine`                                                                                                                                                                                                | [Authentifizierung](#remote-control-stopped-because-the-signed-in-account-changed)                                               |
| `Remote Control stopped — the app running this session is now signed in to a different Claude account`                                                                                                                                                               | [Authentifizierung](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)                 |
| `Remote Control stopped — the app running this session is signed out of Claude`                                                                                                                                                                                      | [Authentifizierung](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts)                 |
| `OAuth token revoked` / `OAuth token has expired`                                                                                                                                                                                                                    | [Authentifizierung](#oauth-token-revoked-or-expired)                                                                             |
| `API Error: 401 Invalid authentication credentials`                                                                                                                                                                                                                  | [Authentifizierung](#api-error-401-invalid-authentication-credentials)                                                           |
| `Login expired · Please run /login`                                                                                                                                                                                                                                  | [Authentifizierung](#login-expired)                                                                                              |
| `Claude login not accepted · Run /login, then try again`                                                                                                                                                                                                             | [Authentifizierung](#claude-login-not-accepted)                                                                                  |
| `Artifacts need a claude.ai login`                                                                                                                                                                                                                                   | [Authentifizierung](#artifacts-need-a-claude-ai-login)                                                                           |
| `Not signed in to the Cloud gateway — run /login.`                                                                                                                                                                                                                   | [Authentifizierung](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                      |
| `Administrator policy requires a Cloud gateway sign-in on this machine`                                                                                                                                                                                              | [Authentifizierung](#administrator-policy-requires-a-cloud-gateway-sign-in)                                                      |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                                                                                                                                                                                           | [Authentifizierung](#login-expired)                                                                                              |
| `Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                            | [Authentifizierung](#your-account-is-on-hold)                                                                                    |
| `Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                     | [Authentifizierung](#your-account-is-on-hold)                                                                                    |
| `Anthropic profile login expired · Re-authenticate your Anthropic profile`                                                                                                                                                                                           | [Authentifizierung](#anthropic-profile-login-expired)                                                                            |
| `Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile`                                                                                                                                                 | [Authentifizierung](#anthropic-profile-login-expired)                                                                            |
| `does not meet scope requirement user:profile`                                                                                                                                                                                                                       | [Authentifizierung](#oauth-scope-requirement)                                                                                    |
| `claude.ai rejected the session token` / `session token rejected`                                                                                                                                                                                                    | [Authentifizierung](#claude-ai-rejected-the-session-token)                                                                       |
| `MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)`                                                                                                                                                                                       | [Authentifizierung](#mcp-server-needs-you-to-sign-in-again)                                                                      |
| `rejected the credential from its headersHelper` / `rejected the Authorization header in its config`                                                                                                                                                                 | [Authentifizierung](#mcp-server-needs-you-to-sign-in-again)                                                                      |
| `MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate`                                                                                                                                                                  | [Authentifizierung](#mcp-server-needs-you-to-sign-in-again)                                                                      |
| `MCP server "<name>" requires re-authorization (token expired)`                                                                                                                                                                                                      | [Authentifizierung](#mcp-server-needs-you-to-sign-in-again)                                                                      |
| `Issuer mismatch in authorization response (RFC 9207)`                                                                                                                                                                                                               | [Authentifizierung](#issuer-mismatch-in-authorization-response)                                                                  |
| `Cloud gateway session expired — run /login to reconnect.`                                                                                                                                                                                                           | [Authentifizierung](#cloud-gateway-session-expired)                                                                              |
| `Cloud gateway <url> no longer accepts this session`                                                                                                                                                                                                                 | [Authentifizierung](#cloud-gateway-session-expired)                                                                              |
| `Sign-in timed out while waiting for you to continue. Try again.`                                                                                                                                                                                                    | [Authentifizierung](#sign-in-timed-out-while-waiting-for-you-to-continue)                                                        |
| `AWS credentials expired or invalid`                                                                                                                                                                                                                                 | [Authentifizierung](#aws-credentials-expired-or-invalid)                                                                         |
| `AWS authentication failed`                                                                                                                                                                                                                                          | [Authentifizierung](#aws-authentication-failed)                                                                                  |
| `Google Cloud credentials expired or invalid`                                                                                                                                                                                                                        | [Authentifizierung](#google-cloud-credentials-expired-or-invalid)                                                                |
| `Google Cloud authentication failed`                                                                                                                                                                                                                                 | [Authentifizierung](#google-cloud-authentication-failed)                                                                         |
| `Microsoft Foundry authentication failed`                                                                                                                                                                                                                            | [Authentifizierung](#microsoft-foundry-authentication-failed)                                                                    |
| `Gateway refused the request`                                                                                                                                                                                                                                        | [Authentifizierung](#gateway-refused-the-request)                                                                                |
| `Could not load AWS credentials` / `Could not load Google Cloud credentials`                                                                                                                                                                                         | [Authentifizierung](#could-not-load-aws-or-google-cloud-credentials)                                                             |
| `AWS default-chain credential resolve timed out`                                                                                                                                                                                                                     | [Authentifizierung](#aws-default-chain-credential-resolve-timed-out)                                                             |
| `Timed out after 60s waiting for AWS`                                                                                                                                                                                                                                | [Authentifizierung](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                       |
| `A request to AWS timed out. Check your network and proxy settings, then try again.`                                                                                                                                                                                 | [Authentifizierung](#bedrock-setup-verification-timed-out-waiting-for-aws)                                                       |
| `Could not load the default credentials` auf Google Clouds Agent Platform                                                                                                                                                                                            | [Authentifizierung](#could-not-load-aws-or-google-cloud-credentials)                                                             |
| `Unable to connect to API`                                                                                                                                                                                                                                           | [Netzwerk](#unable-to-connect-to-api)                                                                                            |
| `Connection refused —` / `Can't reach the API server —` / `No internet route —` / `Couldn't connect through your proxy` / `Connection dropped`, jeweils mit einem Fehlercode in Klammern                                                                             | [Netzwerk](#unable-to-connect-to-api)                                                                                            |
| `Unable to connect to Anthropic services` während des Setups                                                                                                                                                                                                         | [Netzwerk](#unable-to-connect-to-anthropic-services)                                                                             |
| `Socket is closed`                                                                                                                                                                                                                                                   | [Netzwerk](#socket-is-closed)                                                                                                    |
| `Waiting for API response · will retry in`                                                                                                                                                                                                                           | [Automatische Wiederholungen](#automatic-retries), oder [Netzwerk](#unable-to-connect-to-api), wenn es anhält                    |
| `API returned an empty or malformed response`                                                                                                                                                                                                                        | [Netzwerk](#api-returned-an-empty-or-malformed-response)                                                                         |
| `Streaming response ended before any complete data was received`                                                                                                                                                                                                     | [Netzwerk](#streaming-response-ended-before-any-complete-data-was-received)                                                      |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"`                                                                                                                                                                   | [Netzwerk](#bedrock-streaming-response-has-an-unexpected-content-type)                                                           |
| `SSL certificate verification failed`                                                                                                                                                                                                                                | [Netzwerk](#ssl-certificate-errors)                                                                                              |
| `SSL certificate error (...)` während des Logins oder Starts                                                                                                                                                                                                         | [Netzwerk](#ssl-certificate-errors)                                                                                              |
| `unable to get local issuer certificate`                                                                                                                                                                                                                             | [Netzwerk](#ssl-certificate-errors)                                                                                              |
| `403` mit `x-deny-reason: host_not_allowed` in einer Cloud- oder Routine-Sitzung                                                                                                                                                                                     | [Netzwerk](#host-not-allowed-in-a-cloud-session)                                                                                 |
| `proxy refused the connection`                                                                                                                                                                                                                                       | [Netzwerk](#the-proxy-refused-the-connection)                                                                                    |
| `403` mit `This GraphQL query is not enabled for this session` in einer Cloud-Sitzung                                                                                                                                                                                | [GitHub-Proxy](/docs/de/cloud-environments#github-proxy)                                                                              |
| `The cloud environments service returned an empty response` / `The cloud environments service returned a response in an unexpected format`                                                                                                                           | [Netzwerk](#the-cloud-environments-service-returned-an-empty-or-unexpected-response)                                             |
| `Couldn't reconnect to your Remote Control session`                                                                                                                                                                                                                  | [Netzwerk](#couldnt-reconnect-to-your-remote-control-session)                                                                    |
| `N sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.`                                                                                                                                               | [Netzwerk](#sessions-ended-while-this-machine-was-offline)                                                                       |
| `Couldn't share the transcript.`                                                                                                                                                                                                                                     | [Netzwerk](#couldnt-share-the-transcript)                                                                                        |
| `Prompt is too long` / `Input is too long for requested model`                                                                                                                                                                                                       | [Anfragefehler](#prompt-is-too-long)                                                                                             |
| `Prompt is too long · automatic compaction failed:`                                                                                                                                                                                                                  | [Anfragefehler](#prompt-is-too-long)                                                                                             |
| `Prompt is too long · this conversation is a single exchange` / `A single-exchange conversation cannot be compacted`                                                                                                                                                 | [Anfragefehler](#prompt-is-too-long)                                                                                             |
| `Context limit reached · /compact or /clear to continue`                                                                                                                                                                                                             | [Anfragefehler](#prompt-is-too-long)                                                                                             |
| `Context limit reached · /clear to continue`                                                                                                                                                                                                                         | [Anfragefehler](#prompt-is-too-long)                                                                                             |
| `capability_rejected: prompt_too_long` auf einer Claude-Apps-Gateway-Sitzung                                                                                                                                                                                         | [Anfragefehler](#prompt-is-too-long)                                                                                             |
| `upstream rejected the request` / `request too large for this upstream` auf einer Claude-Apps-Gateway-Sitzung                                                                                                                                                        | [Upstream-Fehlermeldungen](/docs/de/claude-apps-gateway-config#upstream-error-messages)                                               |
| `upstream rate limit exceeded` auf einer Claude-Apps-Gateway-Sitzung                                                                                                                                                                                                 | [Upstream-Fehlermeldungen](/docs/de/claude-apps-gateway-config#upstream-error-messages)                                               |
| `all upstreams failed (N attempted)` auf einer Claude-Apps-Gateway-Sitzung                                                                                                                                                                                           | [Upstream-Fehlermeldungen](/docs/de/claude-apps-gateway-config#upstream-error-messages)                                               |
| `Claude Code may not be enabled for your organization` nach einer Claude-Apps-Gateway-Anmeldung                                                                                                                                                                      | [Fehlerbehebung für Claude-Apps-Gateway](/docs/de/claude-apps-gateway-deploy#troubleshooting)                                         |
| `Context exceeds the ...-token limit by ... tokens` in `/context`-Ausgabe                                                                                                                                                                                            | [Anfragefehler](#context-exceeds-the-token-limit)                                                                                |
| `Error during compaction: Conversation too long`                                                                                                                                                                                                                     | [Anfragefehler](#error-during-compaction-conversation-too-long)                                                                  |
| `Request too large`                                                                                                                                                                                                                                                  | [Anfragefehler](#request-too-large)                                                                                              |
| `Request too large for the API's 32MB request limit`                                                                                                                                                                                                                 | [Anfragefehler](#request-too-large)                                                                                              |
| `Image was too large`                                                                                                                                                                                                                                                | [Anfragefehler](#image-was-too-large)                                                                                            |
| `Unable to resize image`                                                                                                                                                                                                                                             | [Anfragefehler](#unable-to-resize-image)                                                                                         |
| `PDF too large` / `PDF is password protected`                                                                                                                                                                                                                        | [Anfragefehler](#pdf-errors)                                                                                                     |
| `Extra inputs are not permitted`                                                                                                                                                                                                                                     | [Anfragefehler](#extra-inputs-are-not-permitted)                                                                                 |
| `API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid` / `Property keys should match pattern`                                                                                                                                                      | [Anfragefehler](#tool-input-schema-is-invalid)                                                                                   |
| `There's an issue with the selected model`                                                                                                                                                                                                                           | [Anfragefehler](#theres-an-issue-with-the-selected-model)                                                                        |
| `Model ... is not a recognized model id`                                                                                                                                                                                                                             | [Anfragefehler](#model-is-not-a-recognized-model-id)                                                                             |
| `Model ... not found`                                                                                                                                                                                                                                                | [Anfragefehler](#model-not-found)                                                                                                |
| `Claude Opus is not available with the Claude Pro plan`                                                                                                                                                                                                              | [Anfragefehler](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                          |
| `Claude Code ... does not support this model; version ... or newer is required`                                                                                                                                                                                      | [Anfragefehler](#claude-code-does-not-support-this-model)                                                                        |
| `Claude Code ... is older than the minimum version required by your organization's policy`                                                                                                                                                                           | [Anfragefehler](#claude-code-does-not-support-this-model)                                                                        |
| `Model ... is restricted by your organization's settings`                                                                                                                                                                                                            | [Anfragefehler](#model-is-restricted-by-your-organizations-settings)                                                             |
| `Model switch ... blocked by a PreModelSwitch hook`                                                                                                                                                                                                                  | [Anfragefehler](#model-switch-was-blocked-by-a-premodelswitch-hook)                                                              |
| `couldn't save it as your default` / `couldn't confirm it was saved as your default`                                                                                                                                                                                 | [Anfragefehler](#couldnt-save-it-as-your-default)                                                                                |
| `thinking.type.enabled is not supported for this model`                                                                                                                                                                                                              | [Anfragefehler](#thinking-type-enabled-is-not-supported-for-this-model)                                                          |
| `Effort '<level>' isn't available with thinking turned off on this model`                                                                                                                                                                                            | [Anfragefehler](#effort-isnt-available-with-thinking-turned-off)                                                                 |
| `effort '<level>' is not supported when thinking is disabled`                                                                                                                                                                                                        | [Anfragefehler](#effort-isnt-available-with-thinking-turned-off)                                                                 |
| `max_tokens must be greater than thinking.budget_tokens`                                                                                                                                                                                                             | [Anfragefehler](#thinking-budget-exceeds-output-limit)                                                                           |
| `API Error: 400 due to tool use concurrency issues`                                                                                                                                                                                                                  | [Anfragefehler](#tool-use-or-thinking-block-mismatch)                                                                            |
| `API Error: 400 orphaned tool_result in conversation history`                                                                                                                                                                                                        | [Anfragefehler](#tool-use-or-thinking-block-mismatch)                                                                            |
| `API Error: 400 duplicate tool_use ID in conversation history`                                                                                                                                                                                                       | [Anfragefehler](#tool-use-or-thinking-block-mismatch)                                                                            |
| `[Unsupported tool content removed]`                                                                                                                                                                                                                                 | [Anfragefehler](#unsupported-tool-content-removed)                                                                               |
| `role 'system' must precede an 'assistant' message`                                                                                                                                                                                                                  | [Anfragefehler](#role-system-must-precede-an-assistant-message)                                                                  |
| `Invalid encrypted_content in search_result block` / `Invalid encrypted_index in text block` / `Failed to decrypt web search result content`                                                                                                                         | [Anfragefehler](#invalid-encrypted-content-in-search-result-block)                                                               |
| `server_tool_use.name: Input should be` auf jedem Turn einer fortgesetzten Sitzung                                                                                                                                                                                   | [Anfragefehler](#unsupported-tool-content-removed)                                                                               |
| `<model> can't help with this. Start a new session to continue`                                                                                                                                                                                                      | [Anfragefehler](#usage-policy-refusal)                                                                                           |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`                                                                                                                                                                        | [Anfragefehler](#usage-policy-refusal)                                                                                           |
| `<model>'s safeguards flagged this message`                                                                                                                                                                                                                          | [Anfragefehler](#safety-measures-flagged-a-cybersecurity-topic)                                                                  |
| `Opus 5.5's safeguards flagged this session`                                                                                                                                                                                                                         | [Anfragefehler](#safety-measures-flagged-a-cybersecurity-topic)                                                                  |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                                                                                                                                                                                    | [Anfragefehler](#safety-measures-flagged-a-cybersecurity-topic)                                                                  |
| `Installation was killed before it could finish (exit code 137)`                                                                                                                                                                                                     | [Installationsfehler](#installation-was-killed-before-it-could-finish)                                                           |
| `The connection dropped while downloading the update`                                                                                                                                                                                                                | [Installationsfehler](#the-connection-dropped-while-downloading-the-update)                                                      |
| `Download timed out: exceeded the total deadline`                                                                                                                                                                                                                    | [Installationsfehler](#the-connection-dropped-while-downloading-the-update)                                                      |
| `--bg and --print conflict`                                                                                                                                                                                                                                          | [Befehlszeilenfehler](#command-line-errors)                                                                                      |
| `Cloud sessions cannot be created from a --restricted session`                                                                                                                                                                                                       | [Befehlszeilenfehler](#cloud-sessions-cannot-be-created-from-a-restricted-session)                                               |
| `Cloud sessions are disabled by your organization's policy`                                                                                                                                                                                                          | [Befehlszeilenfehler](#cloud-sessions-are-disabled-by-your-organizations-policy)                                                 |
| `Couldn't verify your organization's policy for cloud sessions`                                                                                                                                                                                                      | [Befehlszeilenfehler](#cloud-sessions-are-disabled-by-your-organizations-policy)                                                 |
| `Error: --json-schema is not a valid JSON Schema`                                                                                                                                                                                                                    | [Befehlszeilenfehler](#command-line-errors)                                                                                      |
| `Error: Invalid --agents configuration:`                                                                                                                                                                                                                             | [Befehlszeilenfehler](#invalid-agents-configuration)                                                                             |
| `Error: Settings file exceeds the 2MiB limit`                                                                                                                                                                                                                        | [Befehlszeilenfehler](#settings-file-exceeds-the-2mib-limit)                                                                     |
| `The current directory no longer exists (it was deleted or moved)` / `Can't read the current directory`                                                                                                                                                              | [Befehlszeilenfehler](#the-current-directory-no-longer-exists)                                                                   |
| `Temp directory <dir> ... Refusing to use it` / `ENOSPC: no space left on device, mkdir '<dir>'`                                                                                                                                                                     | [Befehlszeilenfehler](#temp-directory-refused-or-cannot-be-created)                                                              |
| `couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded`                                                                                                                                                                        | [Befehlszeilenfehler](#directory-couldnt-be-resolved-to-a-real-location)                                                         |
| `Error: Workspace not trusted` beim Starten von Remote Control                                                                                                                                                                                                       | [Befehlszeilenfehler](#workspace-not-trusted-when-starting-remote-control)                                                       |
| `` `<flag>` before `remote-control` is not carried over to the sessions Remote Control starts ``                                                                                                                                                                     | [Befehlszeilenfehler](#not-carried-over-to-the-sessions-remote-control-starts)                                                   |
| `` `claude import` is not yet available in this build ``                                                                                                                                                                                                             | [Befehlszeilenfehler](#claude-import-is-not-yet-available-in-this-build)                                                         |
| `Could not read Claude Code config`                                                                                                                                                                                                                                  | [Befehlszeilenfehler](#could-not-read-claude-code-config)                                                                        |
| `Could not import <server>: <reason>`                                                                                                                                                                                                                                | [Befehlszeilenfehler](#could-not-import-a-server-from-claude-desktop)                                                            |
| `Cannot add MCP server to scope: managed`                                                                                                                                                                                                                            | [Befehlszeilenfehler](#cannot-add-mcp-server-to-the-managed-scope)                                                               |
| `is Anthropic-hosted and doesn't support local OAuth`                                                                                                                                                                                                                | [Befehlszeilenfehler](#anthropic-hosted-and-doesnt-support-local-oauth)                                                          |
| `Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes`                                                                                                                                                                                      | [Befehlszeilenfehler](#cant-read-mcp-json)                                                                                       |
| `Server rejected the Authorization header minted by the configured headersHelper`                                                                                                                                                                                    | [Befehlszeilenfehler](#server-rejected-the-authorization-header-minted-by-the-configured-headershelper)                          |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                                                                                                                                                                                             | [Befehlszeilenfehler](#mcp-permission-prompt-tool-not-found)                                                                     |
| `OAuth callback port <port> is already in use — another process may be holding it`                                                                                                                                                                                   | [Befehlszeilenfehler](#oauth-callback-port-is-already-in-use)                                                                    |
| `No available ports for OAuth redirect`                                                                                                                                                                                                                              | [Befehlszeilenfehler](#no-available-ports-for-oauth-redirect)                                                                    |
| `Shell command failed for pattern "..."`, von `/security-review` oder einem Skill, der dynamischen Kontext injiziert                                                                                                                                                 | [Befehlszeilenfehler](#security-review-fails-without-origin-head)                                                                |
| `Shell command permission check failed for pattern "..."`, von einem Skill, der dynamischen Kontext injiziert                                                                                                                                                        | [Befehlszeilenfehler](#security-review-fails-without-origin-head)                                                                |
| ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``                                                                                                                                                                             | [Befehlszeilenfehler](#security-review-fails-without-origin-head)                                                                |
| `Input must be provided either through stdin or as a prompt argument when using --print`                                                                                                                                                                             | [Befehlszeilenfehler](#input-must-be-provided-when-using-print)                                                                  |
| `Error: Input contained only whitespace`                                                                                                                                                                                                                             | [Befehlszeilenfehler](#input-contained-only-whitespace)                                                                          |
| `Blank prompt — the message was only whitespace, so nothing was sent to the model.`                                                                                                                                                                                  | [Befehlszeilenfehler](#input-contained-only-whitespace)                                                                          |
| `Error: stream-json input carried over 256M characters with no newline`                                                                                                                                                                                              | [Befehlszeilenfehler](#stream-json-input-carried-over-256m-characters-with-no-newline)                                           |
| `Unknown command: /<name>`, mit oder ohne einen `Did you mean`-Vorschlag                                                                                                                                                                                             | [Befehlszeilenfehler](#unknown-command)                                                                                          |
| `Diff is too large for ultrareview` / `PR #<N> is too large for ultrareview`                                                                                                                                                                                         | [Befehlszeilenfehler](#diff-is-too-large-for-ultrareview)                                                                        |
| `Could not find merge-base with <branch>`                                                                                                                                                                                                                            | [Befehlszeilenfehler](#could-not-find-merge-base-with-the-base-branch)                                                           |
| `Your checkout has no branches (detached HEAD only)`                                                                                                                                                                                                                 | [Befehlszeilenfehler](#your-checkout-has-no-branches)                                                                            |
| `Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected`                                                                                                                                     | [Befehlszeilenfehler](#no-github-account-is-connected-to-your-claude-account)                                                    |
| `Your connected GitHub account can't see <owner>/<repo>`                                                                                                                                                                                                             | [Befehlszeilenfehler](#your-connected-github-account-cant-see-the-repository)                                                    |
| `The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead`                                                                                                                                           | [Befehlszeilenfehler](#the-github-app-preflight-failed-transiently)                                                              |
| `GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud`                                                                                                                                                                     | [Befehlszeilenfehler](#github-isnt-connected-to-your-claude-account)                                                             |
| `Single sign-on authorization needed`                                                                                                                                                                                                                                | [Befehlszeilenfehler](#single-sign-on-authorization-needed)                                                                      |
| `Failed to resume the conversation`                                                                                                                                                                                                                                  | [Befehlszeilenfehler](#failed-to-resume-the-conversation)                                                                        |
| `No conversation found with session ID: <session-id>`                                                                                                                                                                                                                | [Befehlszeilenfehler](#no-conversation-found-with-the-session-id)                                                                |
| `Cannot switch renderers in this session`                                                                                                                                                                                                                            | [Befehlszeilenfehler](#cannot-switch-renderers-in-this-session)                                                                  |
| `Cannot switch renderers while work is running in the background`                                                                                                                                                                                                    | [Befehlszeilenfehler](#cannot-switch-renderers-in-this-session)                                                                  |
| `Couldn't open Claude Desktop`                                                                                                                                                                                                                                       | [Befehlszeilenfehler](#couldnt-open-claude-desktop)                                                                              |
| `Failed to open Claude Desktop. Please try opening it manually.`                                                                                                                                                                                                     | [Befehlszeilenfehler](#couldnt-open-claude-desktop)                                                                              |
| `Couldn't read your Zed keymap` / `Couldn't back up your Zed keymap` / `Couldn't update your Zed keymap`                                                                                                                                                             | [Befehlszeilenfehler](#terminal-setup-left-your-zed-keymap-unchanged)                                                            |
| `Your Zed keymap isn't a readable list of keybindings`                                                                                                                                                                                                               | [Befehlszeilenfehler](#terminal-setup-left-your-zed-keymap-unchanged)                                                            |
| `Skill usage reports are not available on this connection.`                                                                                                                                                                                                          | [Befehlszeilenfehler](#skill-usage-reports-are-not-available-on-this-connection)                                                 |
| `Custom output styles can't be selected over Remote Control or from a relayed message`                                                                                                                                                                               | [Befehlszeilenfehler](#custom-output-styles-cant-be-selected-over-remote-control)                                                |
| `Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load`                                                                                                                                                           | [Befehlszeilenfehler](#output-styles-are-saved-to-local-settings-which-this-session-doesnt-load)                                 |
| `` `plugin eval` is currently in early access `` / `` `plugin eval` is currently unavailable ``                                                                                                                                                                      | [Plugin-Fehler](#plugin-eval-is-currently-in-early-access)                                                                       |
| `Marketplace "<name>" is registered from an untrusted source`                                                                                                                                                                                                        | [Plugin-Fehler](#marketplace-is-registered-from-an-untrusted-source)                                                             |
| `Marketplace "<name>" is already added from a different source`                                                                                                                                                                                                      | [Plugin-Fehler](#marketplace-is-already-added-from-a-different-source)                                                           |
| `"<name>" is another spelling of "<reserved>", a reserved marketplace name`                                                                                                                                                                                          | [Plugin-Fehler](#marketplace-name-is-another-spelling-of-a-reserved-name)                                                        |
| `references ${user_config.*} in a shell-form command`                                                                                                                                                                                                                | [Plugin-Fehler](#plugin-command-references-user-config)                                                                          |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                                                                                                                                                                                   | [Plugin-Fehler](#plugin-command-references-user-config)                                                                          |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                                                                                                                                                                                  | [Plugin-Fehler](#plugin-command-references-user-config)                                                                          |
| `Plugin archive integrity check failed`                                                                                                                                                                                                                              | [Plugin-Fehler](#plugin-archive-integrity-check-failed)                                                                          |
| `path escapes plugin directory`                                                                                                                                                                                                                                      | [Plugin-Fehler](#path-escapes-plugin-directory)                                                                                  |
| `path could not be checked`                                                                                                                                                                                                                                          | [Plugin-Fehler](#path-could-not-be-checked)                                                                                      |
| `its marketplace entry path does not stay inside the marketplace directory`                                                                                                                                                                                          | [Plugin-Fehler](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                          |
| `Plugin source path refused`                                                                                                                                                                                                                                         | [Plugin-Fehler](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                                          |
| `Failed to load marketplace configuration`                                                                                                                                                                                                                           | [Plugin-Fehler](#failed-to-load-marketplace-configuration)                                                                       |
| `Marketplace configuration file is corrupted`                                                                                                                                                                                                                        | [Plugin-Fehler](#failed-to-load-marketplace-configuration)                                                                       |
| `Plugin "<name>@synced" is required by your organization and can't be disabled here`                                                                                                                                                                                 | [Plugin-Fehler](#plugin-is-required-by-your-organization)                                                                        |
| `would be spawned with zero tools — refusing`                                                                                                                                                                                                                        | [Werkzeugfehler](#agent-would-be-spawned-with-zero-tools)                                                                        |
| `File is covered by a Read deny rule in your permission settings`                                                                                                                                                                                                    | [Werkzeugfehler](#file-is-covered-by-a-read-deny-rule)                                                                           |
| `subagent_type is required: the general-purpose agent is not available in this session`                                                                                                                                                                              | [Werkzeugfehler](#subagent-type-is-required)                                                                                     |
| `Error: this write left the memory index at MEMORY.md at ..., over its ... read limit`                                                                                                                                                                               | [Werkzeugfehler](#memory-index-is-over-its-read-limit)                                                                           |
| `pkill: refusing to run`                                                                                                                                                                                                                                             | [Werkzeugfehler](#pkill-pattern-matches-the-claude-code-process)                                                                 |
| `Failed to write to <name>'s inbox — nothing was sent`                                                                                                                                                                                                               | [Werkzeugfehler](#failed-to-write-to-a-teammate-inbox)                                                                           |
| `Failed to write the plan approval request to the lead's inbox — plan not submitted`                                                                                                                                                                                 | [Werkzeugfehler](#failed-to-write-to-a-teammate-inbox)                                                                           |
| `Its agent definition was not restored: the folder its definition file came from is not trusted`                                                                                                                                                                     | [Werkzeugfehler](#teammate-agent-definition-not-restored)                                                                        |
| `Message too large for cross-session delivery`                                                                                                                                                                                                                       | [Werkzeugfehler](#message-too-large-for-cross-session-delivery)                                                                  |
| `Too many messages to this session just now`                                                                                                                                                                                                                         | [Werkzeugfehler](#too-many-messages-to-this-session-just-now)                                                                    |
| `Refusing to send: reply target is a symlink` / `Refusing to send: cannot vet reply target`                                                                                                                                                                          | [Werkzeugfehler](#refusing-to-send-a-cross-session-message)                                                                      |
| `Refusing to send: connected endpoint is not the expected process` / `Refusing to send: connected endpoint identity could not be read`                                                                                                                               | [Werkzeugfehler](#refusing-to-send-a-cross-session-message)                                                                      |
| `Refusing to send: connected endpoint is not owned by this user` / `Refusing to send: connected endpoint owner could not be read`                                                                                                                                    | [Werkzeugfehler](#refusing-to-send-a-cross-session-message)                                                                      |
| `Refusing to send: connected endpoint is a different process with the expected pid`                                                                                                                                                                                  | [Werkzeugfehler](#refusing-to-send-a-cross-session-message)                                                                      |
| `Refusing to read <path>: its symlink resolution changed after permission was checked (<reason>)` / `Refusing to search <path>: its symlink resolution changed after permission was checked`                                                                         | [Werkzeugfehler](#refusing-after-a-symlink-changed)                                                                              |
| `Refusing to write <path>: its parent-directory symlink resolution changed after permission was checked` / `Refusing to write <path>: it is a symbolic link. Write to the link's target path instead`                                                                | [Werkzeugfehler](#refusing-after-a-symlink-changed)                                                                              |
| `Refusing to write through symlink: <path>` / `Refusing to write into symlinked directory: <path>`                                                                                                                                                                   | [Werkzeugfehler](#refusing-after-a-symlink-changed)                                                                              |
| `Refusing to search <path>: a path one of its Read deny rules is written through changed while the search was being prepared` / `Refusing to search <path>: it could not be opened`                                                                                  | [Werkzeugfehler](#refusing-after-a-symlink-changed)                                                                              |
| `its permission check expired before it ran (too many concurrent file operations)` / `ripgrep was found only by name on PATH`                                                                                                                                        | [Werkzeugfehler](#refusing-after-a-symlink-changed)                                                                              |
| `task output swap refused (tasks dir moved or linked)`                                                                                                                                                                                                               | [Werkzeugfehler](#task-output-swap-refused)                                                                                      |
| `Command killed: its output file was replaced or could no longer be verified`                                                                                                                                                                                        | [Werkzeugfehler](#task-output-swap-refused)                                                                                      |
| `the source file is not valid UTF-8 text` / `the source file is not valid UTF-16 text`                                                                                                                                                                               | [Werkzeugfehler](#the-source-file-is-not-valid-utf-8-text)                                                                       |
| `the source file has the replacement character U+FFFD`                                                                                                                                                                                                               | [Werkzeugfehler](#the-source-file-is-not-valid-utf-8-text)                                                                       |
| `Reading a local file from outside this session's connected folders, or through a link, needs the approval card`                                                                                                                                                     | [Werkzeugfehler](#reading-a-local-file-from-outside-the-connected-folders)                                                       |
| `cannot read file_path (...) — the file could not be examined, and no one can answer the approval card`                                                                                                                                                              | [Werkzeugfehler](#reading-a-local-file-from-outside-the-connected-folders)                                                       |
| `WebFetch cannot fetch localhost or other hostnames without a dot`                                                                                                                                                                                                   | [Werkzeugfehler](#webfetch-cannot-fetch-localhost)                                                                               |
| `Can't open MCP settings while no terminal is attached to this background session`                                                                                                                                                                                   | [Fehler in Hintergrund-Sitzungen](#commands-refused-in-a-background-session)                                                     |
| `Can't open MCP settings in a background session`                                                                                                                                                                                                                    | [Fehler in Hintergrund-Sitzungen](#commands-refused-in-a-background-session)                                                     |
| `blocked because the path is spelled in a form that cannot be safely resolved`                                                                                                                                                                                       | [Fehler in Hintergrund-Sitzungen](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved)                          |
| `blocked because the path is network-shaped`                                                                                                                                                                                                                         | [Fehler in Hintergrund-Sitzungen](#write-or-command-blocked-because-the-path-names-a-network-location)                           |
| `is isolated in the worktree <path>, but this command <reason>. Refusing to run it`                                                                                                                                                                                  | [Fehler in Hintergrund-Sitzungen](#command-blocked-by-the-worktree-isolation-checks)                                             |
| `too complex to verify that it stays inside the worktree`                                                                                                                                                                                                            | [Fehler in Hintergrund-Sitzungen](#command-blocked-by-the-worktree-isolation-checks)                                             |
| `This session has no saved transcript`                                                                                                                                                                                                                               | [Fehler in Hintergrund-Sitzungen](#this-session-has-no-saved-transcript)                                                         |
| `Can't open — this session is running in another terminal`                                                                                                                                                                                                           | [Fehler in Hintergrund-Sitzungen](#this-session-is-running-in-another-terminal)                                                  |
| `This conversation is already open in another running Claude session`                                                                                                                                                                                                | [Fehler in Hintergrund-Sitzungen](#this-session-is-running-in-another-terminal)                                                  |
| `This session's saved conversation is no longer on disk`                                                                                                                                                                                                             | [Fehler in Hintergrund-Sitzungen](#this-sessions-saved-conversation-is-no-longer-on-disk)                                        |
| `kept <id> — its worktree is still at <path>`                                                                                                                                                                                                                        | [Fehler in Hintergrund-Sitzungen](#worktree-has-commits-that-are-not-pushed-anywhere)                                            |
| `kept <id> — <n> unpushed commits on <branch>`                                                                                                                                                                                                                       | [Fehler in Hintergrund-Sitzungen](#worktree-has-commits-that-are-not-pushed-anywhere)                                            |
| `kept <id> — worktree has commits that are not pushed anywhere`                                                                                                                                                                                                      | [Fehler in Hintergrund-Sitzungen](#worktree-has-commits-that-are-not-pushed-anywhere)                                            |
| `terminal host process died — press Enter to restart` / `This session's terminal host process died`                                                                                                                                                                  | [Fehler in Hintergrund-Sitzungen](#terminal-host-process-died)                                                                   |
| `Session isn't responding` / `Press enter again to restart this session — it isn't responding`                                                                                                                                                                       | [Fehler in Hintergrund-Sitzungen](#session-isnt-responding)                                                                      |
| `Session <id> was stopped while the respawn was in flight`                                                                                                                                                                                                           | [Fehler in Hintergrund-Sitzungen](#session-was-stopped-while-the-respawn-was-in-flight)                                          |
| `This session was running agent '<name>', which is no longer available`                                                                                                                                                                                              | [Fehler in Hintergrund-Sitzungen](#session-agent-no-longer-available)                                                            |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                                                                                                                                                                                          | [Fehler in Hintergrund-Sitzungen](#claude_code_process_wrapper-launcher-errors)                                                  |
| `EUNKNOWN: unknown error, uv_spawn`                                                                                                                                                                                                                                  | [Fehler in Hintergrund-Sitzungen](#eunknown-when-starting-a-background-session)                                                  |
| `EACCES: permission denied, posix_spawn`                                                                                                                                                                                                                             | [Fehler in Hintergrund-Sitzungen](#eacces-when-starting-a-background-session)                                                    |
| `exited before it became reachable`                                                                                                                                                                                                                                  | [Fehler in Hintergrund-Sitzungen](#background-service-exited-before-it-became-reachable)                                         |
| `Couldn't start a background session (working directory no longer exists or is not accessible: ...)`                                                                                                                                                                 | [Fehler in Hintergrund-Sitzungen](#working-directory-no-longer-exists-when-starting-a-background-session)                        |
| `Claude Code is being updated by npm on this machine (still not runnable after 2 min, ...)`                                                                                                                                                                          | [Fehler in Hintergrund-Sitzungen](#eacces-when-starting-a-background-session)                                                    |
| `Claude Code process exited with code N`                                                                                                                                                                                                                             | [Wrapper- und IDE-Fehler](#claude-code-process-exited-with-code-n)                                                               |
| `The connection to Claude Code ended before this message completed`                                                                                                                                                                                                  | [Wrapper- und IDE-Fehler](#the-connection-to-claude-code-ended-before-this-message-completed)                                    |
| `Could not locate the Claude CLI on PATH`                                                                                                                                                                                                                            | [Wrapper- und IDE-Fehler](#could-not-locate-the-claude-cli-on-path)                                                              |
| `Restored the code, but skipped N files`                                                                                                                                                                                                                             | [Rewind-Warnungen und Fehler](#restored-the-code-but-skipped-files)                                                              |
| `No files were restored: N files failed (backup missing, or the file could not be updated)`                                                                                                                                                                          | [Rewind-Warnungen und Fehler](#no-files-were-restored)                                                                           |
| `Transcript writes are failing (...)`                                                                                                                                                                                                                                | [Warnungen zum Speichern von Sitzungen](#transcript-writes-are-failing)                                                          |
| `Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set`                                                                                                                                                                                                  | [Warnungen zum Speichern von Sitzungen](#transcript-saving-is-off-skip-prompt-history)                                           |
| `Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker`                                                                                                                                                                                              | [Warnungen zum Speichern von Sitzungen](#transcript-saving-is-off-child-session-marker)                                          |
| `Claude Code's fullscreen renderer didn't finish starting last time on this machine` / `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`                                                                                            | [Konfigurationswarnungen](#fullscreen-failed-start-notice)                                                                       |
| `Claude Code exited after an unrecoverable interface error (...)`                                                                                                                                                                                                    | [Konfigurationswarnungen](#exited-after-an-unrecoverable-interface-error)                                                        |
| `Agent descriptions are over the 15.0k-token limit`                                                                                                                                                                                                                  | [Konfigurationswarnungen](#agent-descriptions-are-over-the-15000-token-limit)                                                    |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                                                                                                                                                                                  | [Konfigurationswarnungen](#workspace-has-not-been-trusted)                                                                       |
| `is a network path, which cannot be added as a working directory`                                                                                                                                                                                                    | [Konfigurationswarnungen](#working-directory-is-a-network-path)                                                                  |
| `Remote managed settings failed to load (<cause>)`                                                                                                                                                                                                                   | [Konfigurationswarnungen](#remote-managed-settings-failed-to-load)                                                               |
| `Managed settings were not approved; exiting without applying them.`                                                                                                                                                                                                 | [Konfigurationswarnungen](#managed-settings-were-not-approved)                                                                   |
| `MCP server <name> is blocked by enterprise managed policy`                                                                                                                                                                                                          | [Konfigurationswarnungen](#mcp-server-is-blocked-by-enterprise-managed-policy)                                                   |
| `Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.`                                                                                                                                              | [Konfigurationswarnungen](#managed-settings-document-could-not-be-parsed)                                                        |
| `Managed settings drop-in directory could not be read`                                                                                                                                                                                                               | [Konfigurationswarnungen](#managed-settings-document-could-not-be-parsed)                                                        |
| `otelHeadersHelper failed; telemetry is not being exported. See /status: ...`                                                                                                                                                                                        | [Konfigurationswarnungen](#otelheadershelper-failed)                                                                             |
| `"crossSessionInbound" must be one of "accept", "hold", "refuse"`                                                                                                                                                                                                    | [Konfigurationswarnungen](#crosssessioninbound-must-be-one-of-accept-hold-refuse)                                                |
| `headersHelper not run — this workspace has no persisted trust`                                                                                                                                                                                                      | [Konfigurationswarnungen](#headershelper-not-run)                                                                                |
| `Invalid permission rule "..." was skipped: Malformed Tool(content) rule`                                                                                                                                                                                            | [Konfigurationswarnungen](#malformed-tool-content-rule)                                                                          |
| `... is not matched by file permission checks`                                                                                                                                                                                                                       | [Konfigurationswarnungen](#is-not-matched-by-file-permission-checks)                                                             |
| `... has a wildcard before the rest of the command`                                                                                                                                                                                                                  | [Konfigurationswarnungen](#has-a-wildcard-before-the-rest-of-the-command)                                                        |
| `CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced`                                                                                                                                                                                           | [Konfigurationswarnungen](#the-200k-limit-isnt-enforced)                                                                         |
| `[claude-code:unrecognized_model]`                                                                                                                                                                                                                                   | [Konfigurationswarnungen](#unrecognized-model-id-on-a-request)                                                                   |
| `Stale sandbox mask files left by a killed session`                                                                                                                                                                                                                  | [Konfigurationswarnungen](#stale-sandbox-mask-files-left-by-a-killed-session)                                                    |
| Antworten scheinen von geringerer Qualität als üblich                                                                                                                                                                                                                | [Antwortqualität](#responses-seem-lower-quality-than-usual)                                                                      |

<h2 id="automatic-retries">
  Automatische Wiederholungen
</h2>

Claude Code wiederholt vorübergehende Fehler bis zu 10-mal mit exponentiellem Backoff, bevor dir ein Fehler angezeigt wird. Es wiederholt nicht immer einen Fehler, der während einer unvollständigen Antwort von Claude auftritt. Wenn du einen der Fehler auf dieser Seite siehst, hat Claude Code bereits alle anwendbaren Wiederholungen für diesen Fehler durchgeführt; die Listen unten zeigen, welche Fehler das volle Budget erhalten, welche ein kleineres erhalten, und welche keines erhalten.

Claude Code wiederholt diese Fehler:

* Serverfehler, überladene Antworten und Request-Timeouts, die ankommen, bevor Claude mit dem Streamen seiner Antwort begonnen hat.
* Unterbrochene Verbindungen. Wenn eine Verbindung während eines Requests unterbrochen wird, bevor Claude einen Teil seiner Antwort abgeschlossen hat, einschließlich seines Denkens, sendet Claude Code den Request mit demselben Backoff erneut aus und die Runde wird fortgesetzt, auch wenn bereits etwas Text zu streamen begonnen hatte. Wenn die Verbindung unterbrochen wird, nachdem Claude das Denken abgeschlossen hat, aber bevor es einen Text oder Tool-Aufruf gestartet hat, sendet Claude Code den Request stattdessen bis zu zweimal schnell hintereinander erneut aus und beendet die Runde mit `Connection lost before a response was produced`, wenn die Verbindung an diesem Punkt weiterhin unterbrochen wird.
* Eine Verbindung, die Claude Code als unterbrochen erkannt hat, weil dein Computer während eines Requests in den Ruhezustand versetzt wurde. Claude Code zählt sie als unterbrochene Verbindung nach den obigen Regeln; sobald das Wiederholungs-Label den spezifischen Grund benennt, lautet es `Connection lost while your computer was asleep`, und wenn die Runde endet, nachdem Claude das Denken abgeschlossen hat, aber bevor ein Text oder Tool-Aufruf erfolgt, lautet die Nachricht `Your computer went to sleep before a response was produced`.
* Ein stagnierender Antwort-Stream, wenn die Antwort-Header angekommen sind, aber keiner von Claudes Antwort angekommen ist, oder wenn Claude das Denken abgeschlossen hat, aber keinen Text oder Tool-Aufruf gestartet hat: Claude Code bricht die stagnierende Verbindung ab und sendet den Request höchstens einmal erneut aus, außerhalb des oben genannten 10-Versuch-Budgets. Wenn der Response ein zweites Mal stagniert, nachdem Claude das Denken abgeschlossen hat, aber bevor ein Text oder Tool-Aufruf erfolgt, beendet Claude Code die Runde mit `The response stalled before a response was produced`.
* Ein Streaming-Request, auf den die API nie mit Response-Headern antwortet, auf einer Verbindung, auf der die [first-byte deadline läuft](/docs/de/network-config#streaming-idle-watchdogs): Claude Code bricht ihn bei der Deadline ab und sendet ihn höchstens einmal pro Model-Request erneut aus, innerhalb des Wiederholungs-Budgets, und beendet dann die Runde mit [No response from API](#no-response-from-api), wenn dieser Versuch auch unbeantwortet bleibt. Bei anderen Verbindungen wartet der Request auf `API_TIMEOUT_MS`. Wenn du `CLAUDE_CODE_RETRY_WATCHDOG` setzt, gilt die Einfach-Wiederholung-Obergrenze nicht.
* Temporäre 429-Drosselungen, aber nicht die Ausgabenlimit-`429` eines Gateways, die keine Drosselung ist; siehe [Spend limit reached](#spend-limit-reached).
  * Wenn du mit einem claude.ai-Abonnement angemeldet bist, umfasst dies 429-Drosselungen, die die Quota-Header deines Plans nicht enthalten. Vor v2.1.199 wiederholte Claude Code diese Drosselungen nur für API-Schlüssel- und Enterprise-Anmeldungen.
* Ein Request, der abgelehnt wird, weil die Eingabe plus `max_tokens` das Kontext-Limit überschreitet. Das Erneut-Senden unverändert würde auf die gleiche Weise fehlschlagen, daher wiederholt Claude Code mit reduziertem `max_tokens` und stoppt die Wiederholung und komprimiert stattdessen in zwei Fällen:
  * Wenn keine Reduktion passt, zum Beispiel wenn das Gespräch selbst das Kontext-Fenster fast ausfüllt.
  * Wenn eine Wiederholung `max_tokens` nicht weiter verringern kann. Vor v2.1.218 konnte Claude Code einen reduzierten Request erneut senden, der immer noch nicht passte, z. B. wenn das Extended-Thinking-Budget das verbleibende Kontext überschritt, bis das Wiederholungs-Budget aufgebraucht war.
* Ein abgelaufenes oder fehlendes Google Cloud-Credential auf [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), oder AWS-Credentials, die auf deinem Computer nicht geladen werden können. Claude Code verwirft seine zwischengespeicherten Credentials und wiederholt bis zu zweimal, dann meldet es den Fehler, damit du dich sofort erneut authentifizieren kannst, wie unter [Could not load AWS or Google Cloud credentials](#could-not-load-aws-or-google-cloud-credentials) beschrieben. Vor v2.1.228 wiederholte Claude Code ein fehlgeschlagenes Google Cloud-Credential durch das volle Wiederholungs-Budget, bevor der Fehler angezeigt wurde.
* Ein `401` oder `403` von der Anthropic API, direkt oder über ein [LLM gateway](/docs/de/llm-gateway), während ein [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper)-Skript die Credential liefert. Claude Code führt das Skript erneut aus und wiederholt mit seiner frischen Ausgabe, innerhalb des vollen Wiederholungs-Budgets. Wenn das Skript selbst beim erneuten Ausführen fehlschlägt, zeigt Claude Code stattdessen [Your apiKeyHelper script is failing](#your-apikeyhelper-script-is-failing) an.

Vor v2.1.227 lautete `Connection lost before a response was produced` `Connection closed while thinking, before producing a response` und `The response stalled before a response was produced` lautete `Response stalled while thinking, before producing a response`.

Claude Code wiederholt diese Fehler nicht:

* Ein TLS-Zertifikatvalidierungsfehler, z. B. ein TLS-inspizierender Proxy, ein fehlendes `NODE_EXTRA_CA_CERTS`-Bundle oder ein abgelaufenes Zertifikat. Claude Code meldet den Fehler beim ersten Versuch, damit du die Zertifikat-Einrichtung sofort beheben kannst; siehe [SSL certificate errors](#ssl-certificate-errors). Claude Code wiederholt immer noch vorübergehende TLS-Bedingungen wie ein Handshake-Timeout. Vor v2.1.199 wiederholte Claude Code Zertifikatfehler durch das volle Wiederholungs-Budget, bevor der Fehler angezeigt wurde.
* Ein Serverfehler, eine unterbrochene Verbindung oder ein stagnierender Stream, der ankommt, nachdem Claude einen Textblock oder einen Tool-Aufruf abgeschlossen hat, oder einen gestartet hat, nachdem es das Denken beendet hat, aber bevor es die Antwort beendet. Claude Code führt den Request nicht erneut aus, da dies die gleichen Tool-Aufrufe zweimal ausführen könnte. Es behält das bei, was Claude abgeschlossen hat, führt alle Tool-Aufrufe aus, die Claude beendet hat, und setzt die Runde von ihren Ergebnissen fort. Für das, was du in einer interaktiven Sitzung und in einer nicht-interaktiven siehst, lies [The response above may be incomplete](#the-response-above-may-be-incomplete). Vor v2.1.199 verwarf Claude Code die teilweise Ausgabe und meldete die ganze Runde als Fehler, wenn ein Serverfehler während des Streams ankam.
* Ein Fehler, der ankommt, nachdem Claude die Antwort beendet hat: Es muss nichts wiederholt werden, daher behält Claude Code die vollständige Antwort und beendet die Runde normal.
* Eine [Amazon Bedrock Streaming-Antwort mit unerwartetem Content-Type](#bedrock-streaming-response-has-an-unexpected-content-type), da das Gateway oder der Proxy, der die Antwort umschreibt, die Wiederholung auf die gleiche Weise umschreiben würde. Erfordert Claude Code v2.1.208 oder später.
* Ein nicht-Streaming-Wiederholung eines fehlgeschlagenen Streaming-Requests, der einen Erfolgsstatus erhält, aber [keine Claude API-Nachricht im Body](#api-returned-an-empty-or-malformed-response). Claude Code beendet die Runde mit diesem Fehler.
* Ein Request, den die Richtlinienprüfung deiner Organisation abgelehnt hat, die sich als `API Error:`-Zeile mit der Ablehnungsnachricht darstellt. Die Administratoren deiner Organisation richten die Prüfung mit [Inference hooks](https://platform.claude.com/docs/en/manage-claude/inference-hooks) ein, einer Claude Enterprise-Funktion, und die Nachricht endet mit den Anweisungen, die sie konfiguriert haben, oder teilt dir standardmäßig mit, sie zu kontaktieren. Claude Code sendet den abgelehnten Request nicht erneut an das gleiche Modell oder an ein [fallback model](/docs/de/model-config#fallback-model-chains), da die Ablehnung den Inhalt des Requests betrifft, nicht das Modell. Vor v2.1.239 konnte Claude Code einen abgelehnten Request erneut senden, ohne Streaming oder auf einem konfigurierten Fallback-Modell, bevor dir die Ablehnung angezeigt wurde.

<h3 id="what-you-see-while-claude-code-retries-or-waits">
  Was du siehst, während Claude Code wiederholt oder wartet
</h3>

Während der Wiederholung zeigt der Spinner einen `Retrying in Ns · attempt x/y`-Countdown nach einem Fehler-Label. Das Label benennt den spezifischen Grund vom ersten Versuch für Fehler, auf die du sofort reagieren kannst: Das Netzwerk ist ausgefallen, ein TLS-Handshake ist fehlgeschlagen, oder du hast ein Rate-Limit erreicht. Für andere Fehler lautet es zunächst `API error`. Ab v2.1.198 wechselt es zum spezifischen Grund vom dritten Versuch, oder beim letzten Versuch, wenn `CLAUDE_CODE_MAX_RETRIES` weniger als drei erlaubt; frühere Versionen wechseln nur beim letzten Versuch.

Ab v2.1.198 wird der übliche Spinner-Tipp während Wiederholungen unterdrückt. Sobald der Fehlergrund offenbart wird, wenn der Fehler eine 529-Überladung ist, benennt die Zeile unter dem Countdown auch, wo der Service-Status überprüft werden kann: `status.claude.com` auf der Anthropic API, oder der Provider- oder Gateway-Host, der in der Nachricht auf anderen Konfigurationen benannt ist.

Wenn 20 Sekunden lang keine Daten im Response-Stream ankommen, während ein Request noch ausstehend ist, zeigt der Spinner `Waiting for API response · will retry in … · check your network` an, bevor eine Wiederholung gestartet hat. Der Request ist noch nicht fehlgeschlagen: Der Countdown läuft bis zu dem Punkt, an dem Claude Code die stagnierende Verbindung abbricht. Nach dem Abbruch hängt das, was du siehst, davon ab, wie weit die Antwort gekommen war:

* Bevor Claude einen Textblock oder einen Tool-Aufruf abgeschlossen hat, oder einen gestartet hat, nachdem es das Denken beendet hat, wiederholt Claude Code den Request oder beendet die Runde mit einem Fehler. [Automatic retries](#automatic-retries) sagt, welche Stagnationen es wiederholt und wie oft.
* Nachdem Claude einen Textblock oder einen Tool-Aufruf abgeschlossen hat, oder einen gestartet hat, nachdem es das Denken beendet hat, aber bevor Claude die Antwort beendet hat, behält Claude Code das bei, was Claude abgeschlossen hat, setzt die Runde von allen Tool-Aufrufen fort, die Claude beendet hat, und zeigt [The response above may be incomplete](#the-response-above-may-be-incomplete). In einer nicht-interaktiven Sitzung und für die Antwort eines Subagenten in jeder Sitzung kann Claude Code zuerst Claude auffordern, die Antwort fortzusetzen; dieser Eintrag sagt, wann es das tut und wann du die Mitteilung dort immer noch siehst.
* Nachdem Claude die Antwort beendet hat, beendet Claude Code die Runde normal.

Das Banner wird automatisch gelöscht, sobald Daten wieder ankommen oder eine Wiederholung erfolgreich ist. Wenn es bei jedem Versuch erneut angezeigt wird, behandle es als [network issue](#unable-to-connect-to-api). Vor v2.1.185 erschien das Banner nach 10 Sekunden mit anderer Formulierung.

Während Claude den [advisor](/docs/de/advisor) konsultiert, erscheint das Banner nach 90 Sekunden ohne Daten statt 20, da eine lange Advisor-Überprüfung für gut über 20 Sekunden nichts senden kann. Vor v2.1.214 galt die 20-Sekunden-Schwelle auch während Advisor-Aufrufen, daher erschien das Banner während Advisor-Überprüfungen, auch wenn nichts falsch war.

<h3 id="tune-retry-behavior">
  Wiederholungsverhalten anpassen
</h3>

Du kannst das Wiederholungsverhalten mit diesen Umgebungsvariablen anpassen:

| Variable                                              | Standard      | Effekt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :---------------------------------------------------- | :------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/de/env-vars)             | 10            | Anzahl der Wiederholungsversuche. Ab v2.1.186 auf 15 begrenzt; ab v2.1.199 erhöht `CLAUDE_CODE_RETRY_WATCHDOG` den Standard und entfernt die Obergrenze. Senke es, um Fehler in Skripten schneller zu zeigen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/de/env-vars)          | nicht gesetzt | Setze auf `1` in unbeaufsichtigten Sitzungen wie CI-Jobs, um `429`- und `529`-Kapazitätsfehler unbegrenzt zu wiederholen, statt nach `CLAUDE_CODE_MAX_RETRIES`-Versuchen zu fehlschlagen. Claude Code schlägt sofort fehl, wenn ein Standard-Speed-Request einen `429` erhält, der ein Ausgabenlimit oder erschöpfte Nutzungsguthaben meldet, auch einen von einer [gateway spend cap](#spend-limit-reached), die nach einem Zeitplan zurückgesetzt wird. Vor v2.1.239 wiederholte der Watchdog diese unbegrenzt. Für Fast-Mode-Requests siehe [Handle rate limits](/docs/de/fast-mode#handle-rate-limits). Auf v2.1.199 oder später erhöht es auch die Standard-Wiederholungsanzahl für andere vorübergehende Fehler, wie Serverfehler, Timeouts und unterbrochene Verbindungen, auf 300, ungefähr drei Stunden Backoff, und entfernt die Obergrenze von 15 auf `CLAUDE_CODE_MAX_RETRIES`, wenn du diese Variable explizit setzt. |
| [`API_TIMEOUT_MS`](/docs/de/env-vars)                      | 600000        | Pro-Request-Timeout in Millisekunden. Erhöhe es für langsame Netzwerke oder Proxies. Es begrenzt auch, wie lange Claude Code auf Response-Header wartet, beschrieben in [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/de/env-vars) | nicht gesetzt | Deadline in Millisekunden für das erste Response-Byte eines Streaming-Requests. Erfordert Claude Code v2.1.242 oder später. Wie Claude Code die Deadline auswählt, wenn dies nicht gesetzt ist, siehe [No response from API](#no-response-from-api).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |

<h2 id="server-errors">
  Serverfehler
</h2>

Die meisten dieser Fehler stammen vom Inferenzanbieter: Anthropic's Service auf der Anthropic API und dem Service hinter dem Endpunkt dieses Anbieters auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder einem benutzerdefinierten Gateway. [Auto-Modus kann die Sicherheit einer Aktion nicht bestimmen](#auto-mode-cannot-determine-the-safety-of-an-action) und [Agent wurde vorzeitig aufgrund eines API-Fehlers beendet](#agent-terminated-early-due-to-an-api-error) behandeln auch Ursachen auf Ihrer Seite, wie z. B. ein Amazon Bedrock-Konto, das das Klassifizierungsmodell nicht aufrufen kann, oder einen Subagenten, der ein Nutzungslimit erreicht hat.

<h3 id="api-error-500-internal-server-error">
  API-Fehler: 500 Interner Serverfehler
</h3>

Claude Code zeigt den Statuscode und die Fehlermeldung der API für jede 5xx-Antwort an. Das folgende Beispiel zeigt eine 500-Antwort auf der Anthropic API:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

Der abschließende Satz nennt den Ort, an dem die Serviceintegrität überprüft werden kann, und variiert je nach Anbieter. Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry-Konfigurationen nennen die Servicestatuseite dieses Anbieters. Eine benutzerdefinierte `ANTHROPIC_BASE_URL` nennt den Gateway-Host.

Dies zeigt einen unerwarteten Fehler innerhalb der API an. Er wird nicht durch Ihren Prompt, Ihre Einstellungen oder Ihr Konto verursacht.

**Was zu tun ist:**

* Überprüfen Sie [status.claude.com](https://status.claude.com) oder die im Fehler genannte Anbieter-Statusseite auf aktive Vorfälle
* Warten Sie eine Minute und senden Sie Ihre Nachricht erneut. Ihre ursprüngliche Nachricht befindet sich noch in der Konversation, sodass Sie bei einem langen Prompt `try again` eingeben können, anstatt alles erneut einzufügen.
* Wenn der Fehler ohne einen veröffentlichten Vorfall weiterhin auftritt, führen Sie `/feedback` aus, damit Anthropic Ihre Anfrageinformationen untersuchen kann. Siehe [Fehler melden](#report-an-error), wenn `/feedback` in Ihrer Umgebung nicht verfügbar ist.

<h3 id="api-error-repeated-529-overloaded-errors">
  API-Fehler: Wiederholte 529 Overloaded-Fehler
</h3>

Die API ist vorübergehend über alle Benutzer hinweg ausgelastet. Claude Code hat bereits mehrmals versucht, diese Nachricht anzuzeigen, bevor sie angezeigt wird:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

Der abschließende Satz variiert je nach Anbieter auf die gleiche Weise wie der 500-Fehler oben.

Ein 529-Fehler ist nicht Ihr Nutzungslimit und wird nicht auf Ihr Kontingent angerechnet.

**Was zu tun ist:**

* Überprüfen Sie [status.claude.com](https://status.claude.com) oder die im Fehler genannte Anbieter-Statusseite auf Kapazitätsmitteilungen
* Versuchen Sie es in ein paar Minuten erneut
* Führen Sie `/model` aus und wechseln Sie zu einem anderen Modell, um weiterarbeiten zu können, da die Kapazität pro Modell verfolgt wird. Claude Code fordert Sie dazu auf, wenn ein Modell unter besonders hoher Last steht, z. B. `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Anfrage hat das Zeitlimit überschritten
</h3>

Die API hat nicht vor der Verbindungsfrist geantwortet.

```text theme={null}
Request timed out
```

Dies kann während Zeiten hoher Last auftreten oder wenn das Modell eine sehr große Antwort generiert. Das Standard-Anfrage-Zeitlimit beträgt 10 Minuten.

**Was zu tun ist:**

* Wiederholen Sie die Anfrage
* Teilen Sie die Arbeit bei langwierigen Aufgaben in kleinere Prompts auf
* Wenn eine langsame Netzwerk- oder Proxy-Verbindung die Ursache ist, erhöhen Sie `API_TIMEOUT_MS` wie in [Automatische Wiederholungen](#automatic-retries) beschrieben
* Wenn Zeitüberschreitungen häufig auftreten und Ihr Netzwerk ansonsten fehlerfrei ist, siehe [Netzwerk- und Verbindungsfehler](#network-and-connection-errors) unten

<h3 id="no-response-from-api">
  Keine Antwort von der API
</h3>

Claude Code hat eine Streaming-Anfrage gesendet und die API hat keine Antwortheader innerhalb der Frist für das erste Byte zurückgegeben, sodass Claude Code die Anfrage abgebrochen hat, anstatt auf das vollständige `API_TIMEOUT_MS`-Anfrage-Zeitlimit von standardmäßig 10 Minuten zu warten. Claude Code sendet die Anfrage höchstens einmal erneut, wenn das [Wiederholungsbudget](#tune-retry-behavior) dies zulässt. Wenn die Wiederholung auch unbeantwortet bleibt, endet der Zug mit dieser Nachricht, die anzeigt, wie lange jeder Versuch gewartet hat. Wenn Sie [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/de/env-vars) setzen, gilt die Einfach-Wiederholung nicht und Claude Code versucht es unter dem in [Wiederholungsverhalten abstimmen](#tune-retry-behavior) beschriebenen Budget erneut.

```text theme={null}
API Error: No response from API (waited 3m, then 10m on the retry). If a proxy or gateway on your network holds responses until they complete, raise API_TIMEOUT_MS or CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS to wait longer.
```

Claude Code setzt die Wartezeit für Antwortheader des ersten Versuchs und die Wiederholung separat:

* **Erster Versuch**: [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/de/env-vars), wenn Sie es auf 1 oder mehr setzen, begrenzt auf zwischen 10 Sekunden und 30 Minuten. Andernfalls verwendet Claude Code das Byte-Level-Watchdog-Zeitlimit, das in [Streaming-Idle-Watchdogs](/docs/de/network-config#streaming-idle-watchdogs) aufgelistet ist, sodass die Variablen, die dieses Zeitlimit ändern, auch diese Wartezeit ändern. In jedem Fall fügt Claude Code eine Sekunde für alle 32 KB des Anfragekörpers hinzu.
* **Wiederholung**: eine Sekunde weniger als `API_TIMEOUT_MS`, knapp unter 10 Minuten standardmäßig, damit die Wiederholung länger als ein Proxy oder Gateway dauern kann, das die Antwort bis zum Abschluss der Generierung hält. Bei Amazon Bedrock verwendet die Wiederholung die gleiche Frist wie der erste Versuch, und die Nachricht zeigt eine Dauer statt zwei an.

Keine Wartezeit überschreitet eine Sekunde weniger als ein positives `API_TIMEOUT_MS`, und ein positives `API_TIMEOUT_MS` unter 11 Sekunden deaktiviert die Frist. Der Byte-Level-Watchdog startet erst, nachdem die Antwortheader ankommen, sodass eine Antwort, die nach diesem Punkt das Senden von Bytes stoppt, stattdessen den [Stalled-Stream-Regeln](#automatic-retries) folgt.

**Was zu tun ist:**

* Senden Sie Ihre Nachricht erneut. Ihre ursprüngliche Nachricht befindet sich noch in der Konversation, sodass Sie bei einem langen Prompt `try again` eingeben können, anstatt alles erneut einzufügen.
* Wenn es sich wiederholt, behandeln Sie es als [Netzwerk- oder Proxy-Problem](#unable-to-connect-to-api). Ein Proxy, der die Verbindung akzeptiert und die Anfrage nie weiterleitet, erzeugt diesen Fehler bei jedem Versuch.
* Wenn ein Proxy oder Gateway in Ihrem Netzwerk Antworten bis zum Abschluss hält, erhöhen Sie `API_TIMEOUT_MS`, damit die Wiederholung länger wartet. Bei Amazon Bedrock erhöhen Sie auch `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`.
* Wenn der erste Versuch immer das Zeitlimit überschreitet und die Wiederholung dann erfolgreich ist, erhöhen Sie `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`, damit der erste Versuch auch lange genug wartet.

Vor v2.1.242 wartete Claude Code auf das vollständige `API_TIMEOUT_MS`-Anfrage-Zeitlimit von standardmäßig 10 Minuten, bevor eine unbeantwortete Streaming-Anfrage fehlschlug. Vor v2.1.261 wartete die Wiederholung auf die gleiche Frist wie der erste Versuch und die Nachricht zeigte keine Dauern an.

<h3 id="the-response-above-may-be-incomplete">
  Die obige Antwort kann unvollständig sein
</h3>

Eine Streaming-Anfrage ist fehlgeschlagen, während die Antwort noch in Bearbeitung war, nachdem Claude einen Textblock oder einen Toolaufruf abgeschlossen hatte oder einen nach Abschluss des Denkens gestartet hatte. Das erneute Senden der Anfrage könnte die gleichen Toolaufrufe zweimal ausführen, sodass Claude Code die Ausgabe behält, die Claude abgeschlossen hat, und stattdessen diese Mitteilung anfügt, anstatt den Zug zu verwerfen. Welche Variante Sie sehen, nennt die Ursache:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection lost mid-response. The response above may be incomplete.
API Error: Your computer went to sleep mid-response. The response above may be incomplete.
API Error: The response stopped arriving. The response above may be incomplete.
```

* `Server error mid-response`: ein Mid-Stream-Overload- oder 5xx-Serverfehler. Diese Variante erfordert Claude Code v2.1.199 oder später; davor verwarf dieser Fall die Teilausgabe und meldete den gesamten Zug als Fehler.
* `Connection lost mid-response`: die Verbindung wurde unterbrochen.
* `Your computer went to sleep mid-response`: Claude Code hat erkannt, dass Ihr Computer in den Ruhezustand gegangen ist, während die Antwort gestreamt wurde. Sobald Ihr Computer aufwacht, behandelt Claude Code die Verbindung als unterbrochen und stoppt das Lesen von ihr.
* `The response stopped arriving`: die Verbindung blieb offen, aber stoppte die Datenlieferung, sodass der Streaming-Idle-Watchdog sie abgebrochen hat. Vor v2.1.222 konnte Claude Code diesen Fehler auch auf [Gateway](/docs/de/gateways)-Verbindungen melden, die über `ANTHROPIC_BASE_URL` oder `ANTHROPIC_AWS_BASE_URL` erreicht wurden, während die Keep-Alive-Pings des Servers noch ankamen, da es dort nur geparste Antwortereignisse zählte; ein Upgrade stoppt diese falschen Zeitüberschreitungen auf diesen Routen. Gateways, die über eine Anbieter-Basis-URL wie `ANTHROPIC_BEDROCK_BASE_URL` erreicht werden, sind nicht vom Byte-Watchdog umhüllt; siehe [Streaming-Idle-Watchdogs](/docs/de/network-config#streaming-idle-watchdogs).

Vor v2.1.227 las `Connection lost mid-response` `Connection closed mid-response` und `The response stopped arriving` las `Response stalled mid-stream`.

In vier Fällen behandelt Claude Code den Fehler, ohne diese Mitteilung sofort anzuzeigen:

* Früher in der Antwort versucht Claude Code entweder, den Fehler erneut zu versuchen, oder beendet den Zug mit einem anderen Fehler. Siehe [Automatische Wiederholungen](#automatic-retries).
* Wenn einer dieser Fehler ankommt, nachdem Claude die Antwort abgeschlossen hat, behält Claude Code die vollständige Antwort und beendet den Zug normal, ohne diese Mitteilung. Vor v2.1.222 zeigte Claude Code diese Mitteilung an, wenn die Verbindung nach Abschluss der Antwort unterbrochen wurde oder stagnierte, und meldete den Zug als Fehler, obwohl die Antwort vollständig war.
* In einer [nicht-interaktiven Sitzung](/docs/de/headless), wie z. B. ein `-p`-Lauf, ein [Agent SDK](/docs/de/agent-sdk/overview)-Lauf oder eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web), müssen Sie nicht selbst `continue` senden, wenn die abgeschnittene Antwort in der Hauptkonversation ist und Text, aber keine Toolaufrufe enthält: Claude Code behält die Teilausgabe und fordert Claude auf, von dort aus fortzufahren, wo es gestoppt hat, bis zu dreimal hintereinander. Sie sehen diese Mitteilung für eine solche Antwort nur, wenn Claude Code diese Fortsetzungen aufgebraucht hat. Vor v2.1.246 beendete Claude Code einen nicht-interaktiven Zug mit dieser Mitteilung beim ersten Abschnitt.
* In einem [Subagenten](/docs/de/sub-agents#api-errors-in-subagents), unabhängig davon, ob die Sitzung interaktiv ist oder nicht: Wenn seine abgeschnittene Antwort Text, aber keine Toolaufrufe enthält, fordert Claude Code den Subagenten auf, fortzufahren. Die Mitteilung wird zur letzten Nachricht des Subagenten erst, wenn diese Fortsetzungen aufgebraucht sind. Vor v2.1.257 zeigte ein Subagent diese Mitteilung beim ersten Abschnitt an.

**Was zu tun ist:**

* Lesen Sie in einer interaktiven Sitzung die Antwort, die auf dem Bildschirm verbleibt: Claude Code behält jeden Block, den Claude vor dem Fehler abgeschlossen hat, aber verwirft einen unterbrochenen letzten Block, wenn der Zug endet, sodass die letzten Sätze oder Toolaufrufe möglicherweise fehlen. Antworten Sie mit `continue`, damit Claude von seinem letzten abgeschlossenen Block aus fortfährt.
* Im [nicht-interaktiven Modus](/docs/de/headless) (`-p`):
  * Mit der Standard-Textausgabe druckt Claude Code den letzten abgeschlossenen Textblock, den es noch von früher im Zug hält, gefolgt von dieser Nachricht. Wenn es keinen hält, druckt Claude Code diese Nachricht allein, z. B. weil Claude Code die Konversation mitten im Zug komprimiert und diesen Text gelöscht hat. Vor v2.1.219 druckte Claude Code nur diese Nachricht in `-p`-Textausgabe und verwarf die bereits erzeugte Antwort.
  * Mit `--output-format json` oder `stream-json` meldet Claude Code diese Nachricht im `result`-Feld.
  * Um den Zug fortzusetzen, sobald die Verbindung stabil ist, setzen Sie die Sitzung fort und senden Sie `continue` wie in [Konversationen fortsetzen](/docs/de/headless#continue-conversations) beschrieben.

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto-Modus kann die Sicherheit einer Aktion nicht bestimmen
</h3>

Das Modell, das der [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) zum Klassifizieren von Aktionen verwendet, konnte keine Entscheidung treffen, sodass der Auto-Modus die Aktion nicht automatisch genehmigt hat. Die Nachricht, die Sie sehen, hängt davon ab, wie der Klassifizierer fehlgeschlagen ist.

Lesevorgänge, Suchen und Bearbeitungen in Ihrem Arbeitsverzeichnis überspringen den Klassifizierer, sodass sie in all diesen Fällen weiterhin funktionieren.

Wenn das Klassifizierungsmodell nicht verfügbar ist:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait a moment and then try this action again.
```

Wenn Claude Code die Fehlerkategorie bestimmen kann, nennt es die Kategorie in Klammern nach `temporarily unavailable`, z. B. `<model> is temporarily unavailable (rate-limited), so auto mode cannot determine the safety of <tool> right now`. Die Kategorien sind `(rate-limited)`, `(overloaded)`, `(server error)`, `(timed out)` und `(connection failed)`. Rate-Limited-, Overload- und Serverfehler sind vorübergehend, und ein erneuter Versuch funktioniert. Wenn `(timed out)` oder `(connection failed)` sich wiederholt, überprüfen Sie Ihre Verbindung; siehe [Kann keine Verbindung zur API herstellen](#unable-to-connect-to-api). Vor v2.1.229 nannte die Nachricht nie eine Kategorie und las `Wait briefly and then try this action again`.

Wenn keine Kategorie passt, wird die Nachricht ohne Kategorie in Klammern angezeigt; mehr als ein Fehler erzeugt diese Form. Bei [Amazon Bedrock](/docs/de/amazon-bedrock), einschließlich des [Mantle-Endpunkts](/docs/de/amazon-bedrock#use-the-mantle-endpoint), wird es auch angezeigt, wenn Ihr AWS-Konto das im Fehler genannte Modell nicht aufrufen kann, und dieser Fehler wiederholt sich bei jedem Wiederholungsversuch, bis Ihrem Konto Zugriff auf das Modell gewährt wird.

**Was zu tun ist:**

* Versuchen Sie es nach ein paar Sekunden erneut; Claude sieht die gleiche Nachricht und versucht normalerweise automatisch erneut. Ein vorübergehender Fehler ist nicht mit der [Auto-Modus-Berechtigung](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) verbunden; Sie müssen die Einstellungen nicht ändern
* Wenn Wiederholungsversuche weiterhin fehlschlagen, fahren Sie mit schreibgeschützten Aufgaben fort und kehren Sie später zur blockierten Aktion zurück
* Bei Amazon Bedrock, wenn die Nachricht bei jedem Wiederholungsversuch zurückkommt, überprüfen Sie, dass Ihr Konto das im Fehler genannte Modell aufrufen kann: Bestätigen Sie für Standard-Amazon Bedrock-Modelle, dass Ihre [IAM-Richtlinie](/docs/de/amazon-bedrock#iam-configuration) das Aufrufen zulässt; für Mantle-Modell-IDs [kontaktieren Sie Ihr AWS-Kontoteam](/docs/de/amazon-bedrock#mantle-endpoint-errors)

Wenn eine Klassifiziereranfrage fehlschlägt, weil Ihr OAuth-Token abgelaufen ist oder von einer anderen Sitzung rotiert wurde, aktualisiert Claude Code das Token und versucht die Anfrage einmal erneut, sodass ein routinemäßiger Token-Ablauf nicht als diese Nachricht angezeigt wird. Vor v2.1.216 fehlte jede Klassifiziereranfrage mit einem abgelaufenen oder rotierten Token, und der Auto-Modus verweigerte jede überprüfte Aktion mit dieser Nachricht, bis das Token aktualisiert wurde.

Wenn der Klassifizierer eine nicht analysierbare Antwort zurückgab:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Was zu tun ist:**

* Versuchen Sie die Aktion erneut; dies ist normalerweise beim nächsten Versuch erfolgreich
* Führen Sie `claude --debug` aus und wiederholen Sie die Aktion, um die zugrunde liegende Klassifiziererantwort im Debug-Protokoll zu sehen

Wenn eine separate API-Sicherheitsprüfung die Klassifiziereranfrage aufgrund früherer Konversationsinhalte blockiert hat:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

Claude Code verweigert die Aktion, teilt Claude aber mit, dass dies keine Beurteilung ist, dass die Aktion unsicher ist, und fordert auf, mit anderen Aufgaben fortzufahren, anstatt zu wiederholen. Diese Verweigerungen zählen nicht zu den [Auto-Modus-Pausenschwellen](/docs/de/permission-modes#when-auto-mode-falls-back). In einem [nicht-interaktiven](/docs/de/headless) `-p`-Lauf stoppt Claude Code den Lauf nicht. Was Claude erhält, hängt davon ab, wo es die Aktion angefordert hat:

* Zu einem [Hintergrund-Subagenten](/docs/de/sub-agents#run-subagents-in-foreground-or-background) in einem `-p`-Lauf ohne `--input-format stream-json` gibt Claude Code ein Fehlerergebnis zurück, das `Agent aborted: auto mode classifier request refused by the safety safeguard in headless mode` enthält
* Überall sonst, einschließlich interaktiver Sitzungen und der Hauptkonversation eines `-p`-Laufs, gibt Claude Code diese Verweigerung an Claude zurück

Vor v2.1.225 zählte Claude Code diese Verweigerungen zu den Pausenschwellen und gab die gleiche Ablehnungsmeldung wie ein echter Klassifiziererblock zurück.

**Was zu tun ist:**

* Dies ist keine Entscheidung über Ihre Aktion. Inhalte, die bereits in Ihrer Konversation vorhanden sind, haben einen Sicherheitsfilter auf der API ausgelöst, als der Auto-Modus die Konversation an den Klassifizierer sendete
* Ein erneuter Versuch hilft nicht; der gleiche Konversationsinhalt wird den Filter erneut auslösen
* Wechseln Sie in einer interaktiven Sitzung zu einem anderen [Berechtigungsmodus](/docs/de/permission-modes), damit Sie die Aktion bei Aufforderung genehmigen können
* Starten Sie eine neue Konversation ohne den auslösenden Inhalt

Wenn die Konversation größer als das Kontextfenster des Klassifizierers geworden ist:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

Was mit der Aktion geschieht, hängt davon ab, wo Claude sie angefordert hat:

* In einer interaktiven Sitzung fällt der Auto-Modus auf eine normale Berechtigungsaufforderung für diese Aktion zurück, damit Sie sie manuell genehmigen oder ablehnen können
* Zu einem [Hintergrund-Subagenten](/docs/de/sub-agents#run-subagents-in-foreground-or-background) in einem [nicht-interaktiven](/docs/de/headless) `-p`-Lauf ohne `--input-format stream-json` gibt Claude Code ein Fehlerergebnis zurück, das `Agent aborted: auto mode classifier transcript exceeded context window in headless mode` enthält, und der Lauf wird fortgesetzt
* Anderswo in einem `-p`-Lauf ohne [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags) gibt es keine Aufforderung, auf die zurückgegriffen werden kann, sodass die Aktion nicht ausgeführt wird und der Lauf fortgesetzt wird

**Was zu tun ist:**

* Genehmigen oder lehnen Sie die Aktion in der angezeigten Aufforderung ab
* Führen Sie in einer interaktiven Sitzung `/compact` aus, um die Konversationsgröße zu reduzieren, damit nachfolgende Aktionen wieder in das Klassifizierer-Fenster passen

<h3 id="the-server-returned-no-safety-verdict">
  Der Server hat kein Sicherheitsurteil zurückgegeben
</h3>

Unter [serverseitiger Klassifiziererüberprüfung](/docs/de/permission-modes#server-side-classifier-review) verweigert der Auto-Modus eine Aktion, wenn der Server kein Urteil dafür gibt. Die Verweigerung nennt eine Kategorie in Klammern, wenn Claude Code eine bestimmen kann, wie z. B. `(timed out)`:

```text theme={null}
The server-side auto mode classifier gave no verdict (timed out), so auto mode cannot determine the safety of <tool>.
```

Der Rest der Nachricht teilt Claude mit, ob ein Wiederholungsversuch helfen kann. Vor einigen dieser Verweigerungen wartet Claude Code, damit Claudes nächster Versuch nicht sofort folgt. Während des Wartens in einer interaktiven Sitzung zeigt der Spinner `Auto mode check unavailable` mit einem Countdown an, und das Drücken von `Esc` unterbricht den Zug.

Nach zehn Antworten hintereinander ohne Urteil stoppt der Auto-Modus den Zug:

```text theme={null}
Auto mode is unavailable — the server returned no safety verdict for the last 10 responses, so Claude stopped. Send a message to try again, or switch out of auto mode.
```

Die Stopmeldung wird an einer anderen Stelle in jeder Art von Sitzung angezeigt:

* In einer interaktiven Sitzung wird die Nachricht als Warnung im Transkript angezeigt und der Zug endet
* In einem [nicht-interaktiven](/docs/de/headless) `-p`-Lauf endet der Lauf und meldet einen Ausführungsfehler. Mit der Standard-Textausgabe wird die Nachricht auf stderr gedruckt.
* Wenn ein [Subagent](/docs/de/sub-agents) das Limit erreicht hat, stoppt der Subagent vor Abschluss, und Claude erhält das, was er mit einer Notiz produziert hat, dass der Auto-Modus ihn gestoppt hat

**Was zu tun ist:**

* Senden Sie eine weitere Nachricht, damit Claude es erneut versucht. Die Anzahl der Antworten beginnt von vorne.
* Wenn der Stopp sich wiederholt und Ihre Anfragen durch ein [LLM-Gateway oder einen Proxy](/docs/de/llm-gateway) gehen, überprüfen Sie, ob dieser Streaming-Antworten kürzt oder umschreibt. [Serverseitige Klassifiziererüberprüfung](/docs/de/permission-modes#server-side-classifier-review) sagt, welches Gateway-Verhalten Verweigerungen verursacht, und der [Gateway-Kompatibilitätsleitfaden](/docs/de/llm-gateway-protocol#feature-pass-through) listet auf, was unverändert durchgeleitet werden muss.
* Setzen Sie `CLAUDE_CODE_AUTO_MODE_SERVER=0`, bevor Sie Claude Code starten, um stattdessen seine eigenen Klassifiziereranfragen zu verwenden. Vor v2.1.281 las Claude Code die Variable nicht bei einer direkten Verbindung zur Anthropic API.
* Um die Aktionen selbst zu genehmigen, [wechseln Sie aus dem Auto-Modus](/docs/de/permission-modes#switch-permission-modes)

Vor v2.1.280 verweigerte Claude Code jede Aktion aus einer Antwort ohne Urteil sofort und stoppte den Zug nie.

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent wurde vorzeitig aufgrund eines API-Fehlers beendet
</h3>

Eine [Subagenten](/docs/de/sub-agents)-API-Anfrage ist terminal fehlgeschlagen, z. B. weil ein Nutzungslimit erreicht wurde oder Wiederholungsversuche für einen Serverfehler aufgebraucht wurden, sodass der Subagent vor Abschluss seiner Aufgabe gestoppt hat. Diese Nachricht erfordert Claude Code v2.1.199 oder später; davor wurde der API-Fehlertext an Claude zurückgegeben, als wäre er das Ergebnis des Subagenten.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Was zu tun ist:**

* Ordnen Sie die Fehlerdetails nach dem Doppelpunkt einem eigenen Abschnitt auf dieser Seite zu, wie z. B. [Nutzungslimits](#usage-limits) oder [Serverfehler](#server-errors), und folgen Sie den Schritten dieses Abschnitts
* Sobald der zugrunde liegende Fehler behoben ist, bitten Sie Claude, die Aufgabe zu wiederholen oder [den Subagenten fortzusetzen](/docs/de/sub-agents#resume-subagents)

Wenn eine Ratenbegrenzung, ein Overload oder ein Serverfehler einen Vordergrund-Subagenten unterbricht, der bereits Textausgabe erzeugt hat, erhält Claude diese Teilausgabe als unvollständig markiert, anstatt diesen Fehler zu erhalten. Ein Subagent, dessen einzige Ausgabe Toolaufrufe waren, erhält auch diesen Fehler; in v2.1.199 gab dies stattdessen ein leeres Teilergebnis zurück. Siehe [API-Fehler in Subagenten](/docs/de/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Nutzungslimits
</h2>

Die meisten Fehler in diesem Abschnitt bedeuten, dass ein Kontingent, das an Ihr Konto oder Ihren Plan gebunden ist, erreicht wurde. Drei funktionieren anders: [`Server is temporarily limiting requests`](#server-is-temporarily-limiting-requests) ist eine serverseitige Drosselung, die nicht mit Ihrem Plan-Kontingent zusammenhängt, [`Usage credits required for 1M context`](#usage-credits-required-for-1m-context) ist eine Berechtigungsprüfung statt eines erschöpften Kontingents, und [`The prompt to confirm went unanswered`](#the-prompt-to-confirm-went-unanswered) bedeutet, dass eine Bestätigungsaufforderung für Nutzungsguthaben unbeantwortet geschlossen wurde, unabhängig davon, ob ein Kontingent erreicht wurde.

<h3 id="youve-hit-your-session-limit">
  You've hit your session limit
</h3>

Abonnementpläne enthalten ein rollendes Nutzungskontingent. Wenn dieses aufgebraucht ist, sehen Sie eine dieser Meldungen:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
You've hit your Sonnet limit · resets 3:45pm
```

Claude Code blockiert weitere Anfragen bis zum in der Meldung angezeigten Zurücksetzzeitpunkt. Die Sitzungs- und Wochenlimits werden über alle Modelle hinweg gemeinsam genutzt, daher stellt das Wechsel der Modelle den Zugriff nicht wieder her. Die Opus- und Sonnet-Limits gelten jeweils nur für Anfragen an diese Modellfamilie, daher können Sie mit `/model` zu einem Modell außerhalb der Familie wechseln und weiterarbeiten.

In einer interaktiven Sitzung, die mit einem claude.ai-Abonnement angemeldet ist, kann Claude Code auch in der offenen Sitzung warten und die unterbrochene Aufgabe kurz nach dem Zurücksetzen fortsetzen. Während es wartet, wird eine Zeile am unteren Rand der Sitzung angezeigt: `Usage limit reached · continuing automatically at 3:45pm · esc to cancel`. Drücken Sie `Esc` bei einer leeren Eingabeaufforderung, um das Warten abzubrechen. Siehe [Wait for a usage limit to reset](/docs/de/interactive-mode#wait-for-a-usage-limit-to-reset) für das, was Sie sehen, wie Sie ein Warten starten oder abbrechen, und wie Sie das automatische Fortsetzen ausschalten. Vor v2.1.234 bot Claude Code dieses Warten nicht an.

Die Nutzung wird gleichzeitig gegen die Sitzungs- und Wochenkontingente angerechnet. Ein einzelner Ausbruch intensiver Aktivität, wie z. B. ein großer Workflow-Fanout, kann das Wochenkontingent aufbrauchen, bevor das Sitzungsfenster zurückgesetzt wird.

**Was zu tun ist:**

* Warten Sie auf den in der Fehlermeldung angezeigten Zurücksetzzeitpunkt
* In der Registerkarte Code der [Desktop-App](/docs/de/desktop) bietet die Sitzungslimit-Karte ein Kontrollkästchen **Auto-continue when limits reset**. Die Wochenlimit-Karte bietet dies nicht. Wenn es aktiviert ist, versucht die Desktop-App die unterbrochene Runde nach dem Zurücksetzen erneut und zeigt die Wiederholungszeit auf der Karte an. Das Desktop-Kontrollkästchen und die Einstellung **Continue automatically at usage limit** der CLI in `/config` sind unabhängig, daher schalten Sie jede einzeln aus.
* Für das Opus- oder Sonnet-Limit führen Sie `/model` aus und wechseln Sie zu einem Modell außerhalb dieser Familie, um weiterarbeiten zu können. Jedes Modell hat seinen eigenen Prompt-Cache, daher liest die nächste Anfrage das gesamte Gespräch ohne Cache-Treffer erneut; siehe [Switching models](/docs/de/prompt-caching#switching-models)
* Führen Sie `/usage` aus, um Ihre Plan-Limits und deren Zurücksetzzeitpunkte anzuzeigen
* Führen Sie `/usage-credits` aus, um zusätzliche Nutzung auf Pro und Max zu kaufen, oder um sie von Ihrem Administrator auf Team und Enterprise anzufordern. Siehe [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) für die Abrechnung.
* Um Ihren Plan für höhere Basislimits zu aktualisieren, siehe [claude.com/pricing](https://claude.com/pricing)

Bevor ein Fenster aufgebraucht wird, kann Claude Code Sie warnen, dass Sie den größten Teil davon verwendet haben, mit einer Meldung wie `You've used 85% of your session limit · resets 3:45pm`. Um Ihr verbleibendes Kontingent kontinuierlich zu überwachen, fügen Sie die `rate_limits`-Felder zu einer [benutzerdefinierten Statuszeile](/docs/de/statusline#rate-limit-usage) hinzu, oder klicken Sie in der Desktop-App auf den [Nutzungsring](/docs/de/desktop#check-usage) neben dem Modellwähler.

<h3 id="usage-credits-required-for-1m-context">
  Usage credits required for 1M context
</h3>

Das ausgewählte Modell verwendet das 1M-Token-Fenster mit erweitertem Kontext, und Ihr Plan enthält es nur über Nutzungsguthaben.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on (they take effect after you restart Claude Code), or /model to switch to standard context
```

Dies ist eine Berechtigungsprüfung, keine Kontingenterschöpfung. Sie wird auch dann ausgelöst, wenn Ihre Sitzungs- und Wochenkontingente noch Kapazität haben. Siehe [Extended context](/docs/de/model-config#extended-context) für die Pläne, die 1M-Kontext direkt enthalten, und welche Nutzungsguthaben erfordern. Claude Code führt diese Prüfung durch, wenn Sie das Modell mit `/model` auswählen, und nur bei einer direkten Verbindung zur Anthropic-API; wenn Sie `ANTHROPIC_BASE_URL` auf ein [LLM-Gateway](/docs/de/llm-gateway) verweisen, erlaubt `/model` die `[1m]`-Auswahl und das Gateway entscheidet, ob die Anfrage erfolgreich ist.

Wenn dieser Fehler mitten in einem Gespräch auftritt, weil der Kontext über 200K Token gewachsen ist, komprimiert Claude Code das Gespräch automatisch zurück unter das Standard-Kontextlimit und behält die Sitzung danach auf diesem Limit, daher ist keine Aktion erforderlich. In Versionen vor v2.1.172 wiederholte sich der Fehler bei jeder nachfolgenden Anfrage einschließlich `/compact`; führen Sie `/clear` auf diesen Versionen aus, um die Wiederherstellung durchzuführen. Die folgenden Schritte gelten, wenn Sie explizit ein `[1m]`-Modell ausgewählt haben.

**Was zu tun ist:**

* Führen Sie `/model` aus und wählen Sie die Variante ohne das `[1m]`-Suffix, um auf das Standard-Kontextfenster zurückzufallen
* Wo die Meldung `/usage-credits` nennt, führen Sie es aus, um die getaktete Abrechnung für die 1M-Variante auf Pro und Max zu aktivieren, oder um Nutzungsguthaben von Ihrem Administrator auf Team und Enterprise anzufordern. Starten Sie Claude Code neu, sobald Nutzungsguthaben aktiviert sind. Bis Sie neu starten, bleibt die Sitzung auf dem Standard-Kontextlimit.
* Wenn der Fehler nach `/model` weiterhin besteht, kann eine 1M-Modell-ID an anderer Stelle gesetzt sein. Siehe [Setting your model](/docs/de/model-config#setting-your-model) für die zu überprüfenden Konfigurationsorte in Prioritätsreihenfolge.
* Um 1M-Varianten vollständig aus dem Modellwähler zu entfernen, setzen Sie [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/de/env-vars)

Vor v2.1.268 endete die Meldung mit `run /usage-credits to turn them on, or /model to switch to standard context` und erwähnte das Neustarten nicht.

<h3 id="the-prompt-to-confirm-went-unanswered">
  The prompt to confirm went unanswered
</h3>

Wenn Ihr Konto die [Fable usage-credits consent](/docs/de/model-config#fable-and-usage-credits) erfordert, fragt Claude Code Sie zur Bestätigung auf, bevor eine Fable-Anfrage Nutzungsguthaben abrechnet. Wenn niemand diese Bestätigungsaufforderung in einer Sitzung beantwortet, die möglicherweise niemanden an ihrem Terminal hat, schließt Claude Code die Aufforderung und beendet die Runde mit einer dieser Meldungen:

```text theme={null}
Fable limit reached · continuing on Fable 5.1 uses usage credits, and the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
Fable 5.1 now uses usage credits · the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
```

Die Meldungen nennen das Fable-Modell der Sitzung, daher lesen sie auf Fable 5 `continuing on Fable 5` und `Fable 5 now uses usage credits`. Vor v2.1.257 begann die erste Meldung mit `Fable 5 limit reached`.

Dies geschieht in [Remote Control](/docs/de/remote-control)-Sitzungen, [Hintergrund-Sitzungen](/docs/de/agent-view) und [Agent-Team](/docs/de/agent-teams)-Kollegensitzungen. Claude Code zeigt die Bestätigungsaufforderung nur in der interaktiven Ansicht der Sitzung an: das Terminal, in dem sie ausgeführt wird, oder für eine Hintergrund-Sitzung die [Agenten-Ansicht](/docs/de/agent-view), sobald Sie sie anhängen. Ein Remote-Control-Client kann sie nicht anzeigen. Claude Code schließt die Aufforderung bei der [`dialogExpiry`](/docs/de/settings-reference#dialogexpiry)-Frist, standardmäßig fünf Minuten, oder sobald eine neue Aufforderung ankommt, während niemand an diesem Terminal eingegeben hat, wie z. B. eine Aufforderung, die von einem Remote-Control-Client gesendet wird. Das Eingeben am Terminal, in dem die Sitzung ausgeführt wird, bricht die Frist ab, und Claude Code wartet auf Ihre Antwort. In der angehängten Ansicht einer Hintergrund-Sitzung bricht das Eingeben die Frist nicht ab, und eine neue Aufforderung schließt immer noch die Bestätigungsaufforderung, daher antworten Sie, bevor eines davon geschieht. Claude Code sendet nichts und behält Ihr Modell, daher zeigt Claude Code die Bestätigungsaufforderung erneut an, wenn Sie Ihre nächste Aufforderung senden.

**Was zu tun ist:**

* Am Terminal, in dem die Sitzung ausgeführt wird, senden Sie eine weitere Aufforderung und beantworten Sie die Bestätigungsaufforderung, wenn sie erneut angezeigt wird. Für eine Hintergrund-Sitzung hängen Sie sie zuerst von der [Agenten-Ansicht](/docs/de/agent-view) an. Das erneute Senden von einem Remote-Control-Client zeigt diese Meldung erneut an, da der Client die Aufforderung nicht anzeigen kann.
* Führen Sie `/model` aus, um zu einem Modell zu wechseln, das keine Nutzungsguthaben abrechnet
* Um sich mehr Zeit zu geben, um dieses Terminal zu erreichen, setzen Sie [`dialogExpiry`](/docs/de/settings-reference#dialogexpiry) auf einen längeren Wert oder `"never"`

Vor v2.1.236 erschien diese Meldung nicht: Während ein Remote-Control-Client verbunden war, wartete Claude Code 60 Sekunden auf eine Antwort und setzte dann die Runde auf Ihrem Standardmodell fort.

<h3 id="server-is-temporarily-limiting-requests">
  Server is temporarily limiting requests
</h3>

Die API hat eine kurzlebige Drosselung angewendet, die nicht mit Ihrem Plan-Kontingent zusammenhängt.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code unterscheidet diese von Ihrem Plan-Limit durch das Fehlen der einheitlichen Quota-Header, die eine echte Limit-Antwort trägt. Ab v2.1.199 wird dies [automatisch erneut versucht](#automatic-retries) mit Backoff, bevor es angezeigt wird, unabhängig davon, wie Sie sich authentifizieren. In früheren Versionen schlug eine Sitzung, die mit einem claude.ai-Abonnement angemeldet war, beim ersten Auftreten fehl; nur API-Schlüssel und Enterprise-Anmeldungen wiederholten es.

**Was zu tun ist:**

* Warten Sie kurz und versuchen Sie es erneut
* Überprüfen Sie [status.claude.com](https://status.claude.com), wenn es weiterhin besteht

<h3 id="request-rejected-429">
  Request rejected (429)
</h3>

Sie haben das für Ihren API-Schlüssel, Ihr Amazon-Bedrock-Projekt oder Ihr Google-Cloud-Projekt konfigurierte Ratenlimit erreicht.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

Der nachfolgende Satz nennt, wo die Dienststabilität überprüft werden soll, und variiert je nach Anbieter. Amazon Bedrock, Google Clouds Agent Platform und Microsoft Foundry-Konfigurationen nennen stattdessen die Dienststatus-Seite dieses Anbieters anstelle der Anthropic-Statusseite. Eine benutzerdefinierte `ANTHROPIC_BASE_URL` nennt den Gateway-Host.

**Was zu tun ist:**

* Führen Sie `/status` aus und bestätigen Sie, dass die aktive Anmeldedaten die sind, die Sie erwarten. Ein verwaister `ANTHROPIC_API_KEY` in Ihrer Umgebung kann Anfragen durch einen Low-Tier-Schlüssel statt durch Ihr Abonnement leiten.
* Überprüfen Sie Ihre Anbieter-Konsole auf die aktiven Limits und fordern Sie einen höheren Tier an, falls erforderlich
* Für Anthropic-API-Schlüssel siehe die [rate limits reference](https://platform.claude.com/docs/en/api/rate-limits) für die Funktionsweise von Tiers und wie man Pro-Workspace-Obergrenzen setzt
* Reduzieren Sie die Parallelität: senken Sie [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/de/env-vars), vermeiden Sie das Ausführen vieler paralleler Subagenten, oder wechseln Sie mit `/model` zu einem kleineren Modell für Läufe mit hohem Volumen

<h3 id="youve-hit-your-monthly-spend-limit">
  You've hit your monthly spend limit
</h3>

Das in Ihrem Plan enthaltene Nutzungskontingent kann diese Anfrage nicht abdecken, und die [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), die sie sonst bezahlen würden, haben eine Ausgabenbegrenzung erreicht. Dies geschieht, wenn eines der Nutzungsfenster Ihres Plans aufgebraucht ist, oder wenn die Anfrage eine ist, die nur Nutzungsguthaben bezahlen, wie z. B. eine Anfrage an ein Modell, das [zu Nutzungsguthaben abgerechnet wird](/docs/de/model-config#fable-and-usage-credits). Die Meldung nennt, welches Limit Sie blockiert hat. Der Text nach dem `·` sagt, wie Sie dieses Limit erhöhen können, und variiert je nach Ihrem Plan und ob Sie die Abrechnung verwalten:

```text theme={null}
You've hit your monthly spend limit · raise it at claude.ai/settings/usage
You've hit your individual spend limit · ask your admin for a higher limit
You've hit your org's monthly spend limit · visit claude.ai/admin-settings/usage to raise it
You've hit your team's shared budget · ask your admin to raise it at claude.ai/admin-settings/usage
You've hit your channel's monthly spend limit · an org owner or channel manager can raise it in the channel's Claude settings
```

`team's shared budget` ist ein gepooltes Budget, das ein Administrator einer Gruppe zugewiesen hat, der Sie angehören; die Meldung nennt die Gruppe nicht. `channel's monthly spend limit` ist das Budget des einen Slack-Kanals, in dem die Sitzung ausgeführt wird, daher kann Ihre Organisation immer noch Budget außerhalb davon haben.

Wenn eines der Fenster Ihres Plans das ist, das aufgebraucht wurde, sagt die Meldung auch, wann dieses Fenster zurückgesetzt wird, zum Beispiel `· your session limit resets 3:45pm`, und der Zugriff wird dann ohne Erhöhung des Limits zurückgegeben. Bei Organisationen mit nutzungsbasierter Abrechnung sagt die Meldung `usage limit` anstelle von `spend limit`, wie in `You've hit your individual usage limit`.

Vor v2.1.239 nannte die Meldung nicht die Zurücksetzzeitpunkt des Plan-Fensters. Vor v2.1.268 erzeugte das gepoolte Budget einer Gruppe die Meldung `individual spend limit` anstelle von `team's shared budget`.

Wenn Sie sich über ein Claude-Apps-Gateway verbinden und `spend limit reached` in Kleinbuchstaben sehen, ist das die Obergrenze Ihres Gateway-Betreibers; siehe [Spend limit reached](#spend-limit-reached).

**Was zu tun ist:**

* Auf Pro und Max erhöhen Sie Ihre monatliche Ausgabenbegrenzung in [**Settings > Usage**](https://claude.ai/settings/usage) auf claude.ai, oder führen Sie `/usage-credits` aus
* Auf Team und Enterprise erhöhen Sie das Limit in [**Admin settings > Usage**](https://claude.ai/admin-settings/usage), wenn Sie die Abrechnung verwalten, oder bitten Sie einen Administrator. `/usage-credits` sendet diese Anfrage an Ihren Administrator für Sie
* Für ein Kanal-Limit bitten Sie einen Org-Besitzer oder den Manager des Kanals, es auf claude.ai zu erhöhen. Siehe [Per-channel limits](https://claude.com/docs/claude-tag/admins/set-spend-limit#per-channel-limits) in der Claude Tag-Dokumentation
* Wenn die Meldung eine Zurücksetzzeitpunkt für das Fenster Ihres Plans nennt, können Sie stattdessen darauf warten
* Führen Sie `/usage` aus, um die Fenster Ihres Plans und deren Zurücksetzzeitpunkte anzuzeigen

<h3 id="spend-limit-reached">
  Spend limit reached
</h3>

Sie verbinden sich über ein [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) und haben eine [Ausgabenbegrenzung](/docs/de/claude-apps-gateway-spend-limits) überschritten, die Ihr Gateway-Betreiber gesetzt hat. Das Gateway blockiert Ihre Anfragen, bis die benannte Periode zurückgesetzt wird oder der Betreiber die Obergrenze erhöht. Es markiert jede blockierte `429`-Antwort mit `x-should-retry: false`, daher zeigt Claude Code diese Meldung ohne Wiederholung an.

```text theme={null}
spend limit reached (daily; resets 2026-08-09 00:00 UTC)
```

Die Meldung nennt die Periode der Obergrenze und die Zurücksetzzeitpunkt, und wenn der Betreiber eine `blocked_message` konfiguriert hat, folgen seine Anweisungen. Vor v2.1.225 lautete die Meldung nur `spend limit reached`; ein Gateway auf einer älteren Version sendet immer noch diese kürzere Form.

**Was zu tun ist:**

* Warten Sie auf den Zurücksetzzeitpunkt, den die Meldung nennt, oder folgen Sie den Anweisungen des Betreibers, wenn die Meldung diese trägt
* Bitten Sie Ihren Gateway-Betreiber, die Obergrenze zu erhöhen, wenn Sie sie regelmäßig erreichen

Eine verwandte Meldung, `spend limit unavailable`, bedeutet, dass das Gateway seine Ausgabendatensätze nicht lesen konnte und die Anfrage als Vorsichtsmaßnahme statt über Ihre Obergrenze blockiert hat. Sie wird normalerweise von selbst gelöscht; wenn sie weiterhin besteht, teilen Sie dies Ihrem Gateway-Betreiber mit.

<h3 id="credit-balance-is-too-low">
  Credit balance is too low
</h3>

Ihre Console-Organisation hat ihre vorausbezahlten Guthaben aufgebraucht, oder Claude Code sendet Ihre Anfragen mit einem Console-API-Schlüssel, wenn Sie Ihr Abonnement verwenden wollten.

```text theme={null}
Credit balance is too low
```

**Was zu tun ist:**

* Wenn Sie einen Pro-, Max-, Team- oder Enterprise-Plan haben und dies sehen, führen Sie `/status` aus und überprüfen Sie die Zeile `API key`. Ein genehmigter `ANTHROPIC_API_KEY` in Ihrer Umgebung leitet Anfragen durch diesen Schlüssel statt durch Ihr Abonnement. Heben Sie die Einstellung in der aktuellen Shell auf und entfernen Sie sie aus Ihrem Shell-Profil, starten Sie dann `claude` neu. Führen Sie `/login` aus, wenn Sie sich noch nicht mit Ihrem Abonnement angemeldet haben.
* Fügen Sie Guthaben unter [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing) hinzu, und erwägen Sie, dort das automatische Neuladen zu aktivieren, damit der Saldo aufgefüllt wird, bevor er null erreicht
* Setzen Sie Pro-Workspace-Ausgabengrenzen in der Console, um zu verhindern, dass ein einzelnes Projekt das Org-Guthaben aufbraucht. Siehe [Manage costs effectively](/docs/de/costs).

<h3 id="could-not-update-your-spend-limit">
  Could not update your spend limit
</h3>

Der Server lehnte eine Ausgabenbegrenzungsänderung ab, die Sie von der Eingabeaufforderung aus vorgenommen haben, die angezeigt wird, wenn Sie Ihre Ausgabenbegrenzung erreichen.

```text theme={null}
Could not update your spend limit: <reason from the server>
```

Wenn der Server die Ablehnung erklärt, endet die Meldung mit diesem Grund, und das erneute Versuchen desselben Werts schlägt erneut fehl. Wenn der Fehler keinen vom Server bereitgestellten Grund hat, wie z. B. eine unterbrochene Verbindung, lautet die Meldung `Could not update your spend limit. Press Enter to retry.` und das erneute Versuchen kann erfolgreich sein. Vor v2.1.216 zeigte Claude Code die generische Form für jeden Fehler an.

**Was zu tun ist:**

* Wenn die Meldung einen Grund enthält, wählen Sie ein Limit, das ihn erfüllt, wie z. B. einen niedrigeren Betrag
* Wenn die Meldung nur die generische Form anzeigt, versuchen Sie es erneut; der Fehler kann vorübergehend sein
* Wenn die Änderung weiterhin fehlschlägt, nehmen Sie sie stattdessen von Ihren [claude.ai-Abrechnungseinstellungen](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) im Browser vor

<h2 id="authentication-errors">
  Authentifizierungsfehler
</h2>

Diese Fehler bedeuten, dass Claude Code nicht nachweisen kann, wer Sie gegenüber der API sind. Führen Sie jederzeit `/status` aus, um zu sehen, welche Anmeldedaten derzeit aktiv sind.

<h3 id="not-logged-in">
  Nicht angemeldet
</h3>

Für diese Sitzung ist keine gültige Anmeldedaten verfügbar.

```text theme={null}
Not logged in · Please run /login
```

**Was zu tun ist:**

* Führen Sie `/login` aus, um sich mit Ihrem Claude-Abonnement oder Ihrem Console-Konto zu authentifizieren
* Wenn Sie erwartet haben, dass eine Umgebungsvariable Sie authentifiziert, bestätigen Sie, dass `ANTHROPIC_API_KEY` in der Shell gesetzt und exportiert ist, in der Sie `claude` gestartet haben
* Für CI oder Automatisierung, bei der interaktive Anmeldung nicht möglich ist, konfigurieren Sie ein [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper)-Skript, das beim Start einen Schlüssel abruft
* Siehe [Authentifizierungspriorität](/docs/de/authentication#authentication-precedence), um zu verstehen, welche Anmeldedaten Claude Code verwendet, wenn mehrere vorhanden sind

Wenn Sie wiederholt aufgefordert werden, sich anzumelden, siehe [Nicht angemeldet oder Token abgelaufen](/docs/de/troubleshoot-install#not-logged-in-or-token-expired) für Systemuhr-Überprüfungen und macOS-Wiederherstellungsschritte für die Anmeldedatenspeicherung.

<h3 id="could-not-resolve-authentication-method">
  Authentifizierungsmethode konnte nicht aufgelöst werden
</h3>

Die Sitzung erreichte den API-Client ohne Anmeldedaten. [Hintergrundsitzungen](/docs/de/agent-view) und Cloud-Sitzungen zeigen diese Meldung an, wenn der Worker ohne Anmeldedaten startet. Interaktive, `-p`- und Agent SDK-Ausführungen melden denselben Zustand wie [Nicht angemeldet](#not-logged-in) und schreiben diese Zeichenkette nur in ihr Debug-Protokoll. Wenn Sie sie dort gefunden haben, folgen Sie stattdessen diesem Eintrag.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

In aktuellen Versionen bedeutet der Fehler, dass dem Worker-Prozess keine Anmeldedaten verfügbar waren. Vor v2.1.174 konnte eine Hintergrundsitzung, die einem untätigen vorinitialisiertem Worker zugewiesen war, auf diese Weise fehlschlagen, selbst wenn gültige Anmeldedaten konfiguriert waren. Vor v2.1.176 konnte auch eine Cloud-Sitzung, die untätig war, bevor sie beansprucht wurde, dies tun. Führen Sie ein Upgrade durch, um dies zu beheben.

**Was zu tun ist:**

* Führen Sie ein Upgrade auf v2.1.176 oder später durch, wenn dies in einer Hintergrund- oder Cloud-Sitzung angezeigt wird und Ihre Anmeldedaten bereits konfiguriert sind
* Bestätigen Sie, dass `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` oder Ihre Cloud-Provider-Anmeldedaten in der Umgebung gesetzt sind, die den Worker startet, nicht nur in Ihrer interaktiven Shell
* Für das Agent SDK siehe [Authentifizierungseinrichtung in der Schnellstartanleitung](/docs/de/agent-sdk/quickstart#setup)
* Führen Sie `/status` in einer interaktiven Sitzung in derselben Umgebung aus, um zu bestätigen, welche Anmeldedatenquelle aufgelöst wird

<h3 id="invalid-api-key">
  Ungültiger API-Schlüssel
</h3>

Die Umgebungsvariable `ANTHROPIC_API_KEY` oder das `apiKeyHelper`-Skript hat einen Schlüssel zurückgegeben, den die API abgelehnt hat, oder Claude Code hat einen Schlüssel von `ANTHROPIC_API_KEY` blockiert, bevor er ihn gesendet hat.

```text theme={null}
Invalid API key · Fix external API key
```

Wenn die Meldung nach `Fix external API key` mit einer Beschreibung wie `Invalid X-Api-Key header value from ANTHROPIC_API_KEY: it contains a line break at character 41 (120 characters on 2 lines).` fortgesetzt wird, hat die API den Schlüssel nie gesehen. Claude Code hat ein Zeichen gefunden, das HTTP-Header nicht tragen können, und die Anfrage gestoppt, bevor sie gesendet wurde. Siehe [Ungültiger Request-Header-Wert](#invalid-request-header-value), um zu erfahren, wie Sie die Beschreibung lesen und den Wert korrigieren.

**Was zu tun ist:**

* Überprüfen Sie auf Tippfehler und bestätigen Sie, dass der Schlüssel nicht in der [Console](https://platform.claude.com/settings/keys) widerrufen wurde
* Führen Sie in derselben Shell `env | grep ANTHROPIC` aus, oder in PowerShell `Get-ChildItem Env:ANTHROPIC*`. Tools wie direnv, dotenv-Shell-Plugins und IDE-Terminals können einen veralteten Schlüssel aus einer `.env`-Datei in Ihrem Projekt laden, ohne dass Sie ihn explizit setzen
* Heben Sie `ANTHROPIC_API_KEY` auf und führen Sie `/login` aus, um stattdessen Abonnement-Authentifizierung zu verwenden
* Wenn der Schlüssel von einem [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper)-Skript stammt, führen Sie das Skript direkt aus, um zu bestätigen, dass es einen gültigen Schlüssel auf stdout ausgibt
* Führen Sie `/status` aus, um zu bestätigen, welche Anmeldedatenquelle Claude Code tatsächlich verwendet

<h3 id="your-apikeyhelper-script-is-failing">
  Ihr apiKeyHelper-Skript schlägt fehl
</h3>

Claude Code hat den Befehl in Ihrer [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper)-Einstellung ausgeführt und keinen Schlüssel zurückbekommen. Ohne einen erreicht die Anfrage die API mit einer Platzhalter-Anmeldedaten, und die API lehnt sie mit `401` ab. Das `Authentication`-Panel im Terminal zeigt, welches dieser Ereignisse passiert ist:

* Der Befehl ist mit einem Fehler beendet worden oder hat das Zeitlimit überschritten
* Der Befehl hat nichts auf stdout ausgegeben
* Der Befehl hat etwas anderes als den Schlüssel ausgegeben, z. B. ein Login-Banner oder eine Protokollzeile. Das Panel zeigt `returned output that cannot be used as an API key` und sagt, was falsch ist, ohne die Ausgabe zu wiederholen. Vor v2.1.227 hat Claude Code das gesendet, was der Befehl ausgegeben hat, nach dem Trimmen von umgebender Leerraum.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

In [nicht-interaktivem Modus](/docs/de/headless) trägt stderr auch den spezifischen Grund, mit dem Präfix `apiKeyHelper failed:`.

Claude Code führt das Skript erneut aus und versucht die Anfrage bis zu zwei weitere Male, bevor diese Meldung angezeigt wird, sodass der Fehler innerhalb von drei Versuchen auftritt. Vor v2.1.208 hat Claude Code das gesamte [Wiederholungsbudget](#automatic-retries) aufgewendet, um die Anfrage mit der Platzhalter-Anmeldedaten erneut zu senden, und dann einen generischen `401`-Authentifizierungsfehler statt des Skriptfehlers gemeldet.

Das Ausführen von `/login` hilft hier nicht: die Ausgabe des Helpers [hat Vorrang](/docs/de/authentication#authentication-precedence) vor einer gespeicherten Anmeldung, solange die Einstellung vorhanden ist.

**Was zu tun ist:**

* Führen Sie den in `apiKeyHelper` konfigurierten Befehl direkt in Ihrer Shell aus, um den Fehler zu reproduzieren
* Wenn der Befehl von einer abgelaufenen Sitzung berichtet, authentifizieren Sie sich erneut bei Ihrem Anmeldedaten-Provider, z. B. indem Sie sich erneut bei Ihrem SSO oder Ihrem Secrets-Tresor anmelden
* Korrigieren Sie den Befehl so, dass er nur den Schlüssel auf stdout ausgibt, als ein einzelnes Token aus druckbarem ASCII bis zu 16.384 Zeichen, und mit Code 0 beendet wird. Siehe [Anmeldedaten mit apiKeyHelper rotieren](/docs/de/llm-gateway-connect#rotate-credentials-with-apikeyhelper) für eine funktionierende Einrichtung.
* Führen Sie `/status` aus, um zu bestätigen, dass `apiKeyHelper` die aktive Anmeldedatenquelle ist. Die `apiKeyHelper`-Zeile zeigt `Failing` mit dem Detail des letzten Fehlers, z. B. dem Exit-Code und der Fehlerausgabe des Befehls, und verschwindet nach der nächsten erfolgreichen Ausführung. Vor v2.1.274 zeigte `/status` nur die Anmeldedatenquelle, nicht den Fehler.
* Jedes Mal, wenn der Befehl fehlschlägt, erscheinen sein Exit-Code und die Fehlerausgabe auch in einem `Authentication`-Panel im Terminal. Vor v2.1.212 war das Panel mit `Cloud authentication` betitelt.

<h3 id="invalid-request-header-value">
  Ungültiger Request-Header-Wert
</h3>

Ein Wert, den Claude Code als Request-Header senden wollte, enthält ein Zeichen, das HTTP-Header nicht tragen können: einen Zeilenumbruch, ein NUL-Byte oder ein Zeichen über `U+00FF`, z. B. ein gekrümmtes Anführungszeichen oder ein Leerzeichen mit Nullbreite. Claude Code stoppt die Anfrage, bevor etwas gesendet wird, und benennt die Variable oder Einstellung, die korrigiert werden muss. Die übliche Ursache ist eine Anmeldedaten, die aus einem Dokument oder Chat eingefügt wurde und ein unsichtbares Zeichen oder einen verirrten Zeilenumbruch trug.

Claude Code führt diese Überprüfung durch, wenn es Anfragen direkt an die Claude API oder durch ein [LLM-Gateway](/docs/de/llm-gateway) sendet. Bei einem Drittanbieter-Cloud-Provider wie [Amazon Bedrock](/docs/de/amazon-bedrock) führt Claude Code dies nicht vor dem Senden durch.

```text theme={null}
Invalid auth token · Fix external auth token
Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable
Invalid request header from the environment · Fix the environment variable
```

Der erste Teil der Meldung hängt davon ab, woher der fehlerhafte Wert stammt:

* `Invalid auth token`: ein Bearer-Token von [`ANTHROPIC_AUTH_TOKEN`](/docs/de/env-vars) oder [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/de/env-vars)
* `Invalid ANTHROPIC_CUSTOM_HEADERS`: ein Header-Name oder -Wert, den Sie in [`ANTHROPIC_CUSTOM_HEADERS`](/docs/de/env-vars) gesetzt haben. Die Beschreibung zählt, welches `Name: Value`-Paar fehlerhaft ist, z. B. `distinct header 2 of 3 parsed from ANTHROPIC_CUSTOM_HEADERS`, ohne den Namen oder Wert zu wiederholen, da Sie beide gewählt haben.
* `Invalid request header from the environment`: ein Wert, den Claude Code aus einer anderen Umgebungsvariable in einen Request-Header kopiert, z. B. `CLAUDE_AGENT_SDK_CLIENT_APP`. Die Beschreibung benennt die Variable, die korrigiert werden muss.

Claude Code meldet einen fehlerhaften `ANTHROPIC_API_KEY`, der von dieser Überprüfung erfasst wird, als [Ungültiger API-Schlüssel](#invalid-api-key), mit derselben nachfolgenden Beschreibung. Es meldet eine fehlerhafte gespeicherte `/login`-Anmeldedaten als [Nicht angemeldet](#not-logged-in) statt; führen Sie `/login` aus, um eine neue zu speichern. Die Ausgabe eines [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper)-Skripts erreicht diese Überprüfung nie: Claude Code validiert sie, wenn das Skript ausgeführt wird, und die Ausgabe, die ein HTTP-Header nicht tragen kann, schlägt mit [Ihr apiKeyHelper-Skript schlägt fehl](#your-apikeyhelper-script-is-failing) fehl.

Nach dem zweiten `·` beschreibt die Meldung das Problem, wie in diesem vollständigen Beispiel:

```text theme={null}
Invalid auth token · Fix external auth token · Invalid Authorization header value from ANTHROPIC_AUTH_TOKEN: it contains a line break at character 41 (120 characters on 2 lines).
```

Positionen zählen Zeichen ab eins. Die Beschreibung wird aus festen Phrasen und Zeichenzählungen erstellt, sodass sie niemals den Wert selbst enthält. Sie benennt das fehlerhafte Zeichen nur, wenn es ein bekanntes unsichtbares oder typografisches Zeichen ist, z. B. eine Byte-Order-Marke, ein Leerzeichen mit Nullbreite oder ein gekrümmtes Anführungszeichen, und meldet alles andere als `a non-ASCII character`.

**Was zu tun ist:**

* Setzen Sie die Variable oder Einstellung, die die Meldung benennt, neu, indem Sie die Zeichen um die gemeldete Position neu eingeben, anstatt aus derselben Quelle erneut einzufügen
* Für `ANTHROPIC_CUSTOM_HEADERS` behalten Sie ein `Name: Value`-Paar pro Zeile und schreiben Sie das Paar neu, das die Meldung zählt
* Führen Sie `/status` aus, um zu bestätigen, welche Anmeldedatenquelle aktiv ist

<h3 id="this-organization-has-been-disabled">
  Diese Organisation wurde deaktiviert
</h3>

Claude Code verwendet einen veralteten `ANTHROPIC_API_KEY` von einer deaktivierten Console-Organisation. Wenn Sie eine gespeicherte Abonnement-Anmeldung haben, setzt der Schlüssel diese außer Kraft.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your subscription instead
Your ANTHROPIC_API_KEY belongs to a disabled organization · Update or unset the environment variable
API Error: 400 ... This organization has been disabled.
```

Der Hinweis nach dem `·` hängt von Ihren gespeicherten Anmeldedaten ab: die erste Form erscheint, wenn eine gespeicherte `/login` übernehmen kann, nachdem Sie den Schlüssel aufgehoben haben, und die zweite, wenn der Schlüssel Ihre einzige Anmeldedaten ist.

Umgebungsvariablen haben Vorrang vor `/login`, sodass ein Schlüssel, der in Ihrem Shell-Profil exportiert oder aus einer `.env`-Datei geladen wird, verwendet wird, selbst wenn Sie ein funktionierendes Pro- oder Max-Abonnement haben. Im nicht-interaktiven Modus (`-p`) wird der Schlüssel immer verwendet, wenn er vorhanden ist.

**Was zu tun ist:**

* Heben Sie `ANTHROPIC_API_KEY` in der aktuellen Shell auf und entfernen Sie es aus Ihrem Shell-Profil, dann starten Sie `claude` neu
* Wenn die Meldung `Update or unset` sagt, haben Sie keine gespeicherte Anmeldung, auf die Sie zurückgreifen können. Heben Sie den Schlüssel auf und führen Sie `/login` aus, oder ersetzen Sie den Schlüssel durch einen von einer aktiven Console-Organisation.
* Führen Sie `/status` danach aus, um zu bestätigen, dass die aktive Anmeldedaten Ihr Abonnement ist
* Wenn keine Umgebungsvariable gesetzt ist und der Fehler weiterhin besteht, ist die deaktivierte Organisation diejenige, die an Ihre `/login` gebunden ist. Kontaktieren Sie den Support oder melden Sie sich mit einem anderen Konto an.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Ihre Organisation hat die API-Schlüssel-Authentifizierung deaktiviert
</h3>

Diese Meldung erfordert Claude Code v2.1.169 oder später. Der Administrator Ihrer Console-Organisation hat die API-Schlüssel-Authentifizierung deaktiviert, sodass die API den Schlüssel ablehnt, den Claude Code sendet. Der Wiederherstellungshinweis nach dem `·` variiert je nachdem, woher der Schlüssel stammt:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Umgebungsvariablen und `apiKeyHelper` haben Vorrang vor `/login`, sodass das alleinige Ausführen von `/login` nicht hilft, während einer von ihnen noch einen Schlüssel liefert. Siehe [Authentifizierungspriorität](/docs/de/authentication#authentication-precedence).

**Was zu tun ist:**

* Wenn die Meldung `ANTHROPIC_API_KEY` benennt, heben Sie es in der aktuellen Shell auf und entfernen Sie es aus Ihrem Shell-Profil oder `.env`-Datei, dann starten Sie `claude` neu
* Wenn die Meldung `apiKeyHelper` benennt, entfernen Sie die [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper)-Einstellung aus Ihrer `settings.json`
* Führen Sie `/login` aus, um sich mit Ihrem claude.ai-Konto anzumelden
* Führen Sie `/status` danach aus, um zu bestätigen, dass die aktive Anmeldedaten Ihr Abonnement ist, nicht ein API-Schlüssel
* Wenn Sie die API-Schlüssel-Authentifizierung für Automatisierung benötigen, bitten Sie Ihren Organisations-Administrator, sie in der Console erneut zu aktivieren

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Ihre Organisation hat den Claude-Abonnement-Zugriff deaktiviert
</h3>

Ihre Claude-Organisation erlaubt nicht, sich mit einer Abonnement-Anmeldung bei Claude Code anzumelden. Das erneute Ausführen von `/login` mit demselben Konto gibt denselben Fehler zurück.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Dies ist eine serverseitige Organisations-Einstellung, sodass sie nicht von lokalen Einstellungen, Umgebungsvariablen oder CLI-Flags außer Kraft gesetzt werden kann.

Das Agent SDK und der `-p` nicht-interaktive Modus zeigen dies als den `oauth_org_not_allowed`-Fehlercode.

**Was zu tun ist:**

* Bitten Sie Ihren Administrator, den Claude Code-Zugriff für Ihre Organisation zu aktivieren
* Authentifizieren Sie sich mit einem Console-API-Schlüssel statt mit Ihrem Abonnement. Siehe [Claude Console-Authentifizierung](/docs/de/authentication#claude-console-authentication) für die Einrichtung.
* Wenn Sie der Administrator sind und keine Option zum Aktivieren des Zugriffs sehen, kontaktieren Sie den [Anthropic-Support](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Routinen sind durch die Richtlinie Ihrer Organisation deaktiviert
</h3>

Ein Eigentümer in Ihrer Team- oder Enterprise-Organisation hat Routinen auf Organisations-Ebene deaktiviert. Der Fehler erscheint, wenn Sie versuchen, eine Routine zu erstellen oder auszuführen, z. B. aus der [Routinen](/docs/de/routines)-Benutzeroberfläche auf claude.ai/code. In Claude Code v2.1.227 oder später [verbirgt die gleiche Einstellung auch `/schedule`](/docs/de/routines#troubleshooting) in der CLI.

```text theme={null}
Routines are disabled by your organization's policy.
```

Dies ist eine serverseitige Einstellung, sodass sie nicht von lokalen Einstellungen, Umgebungsvariablen oder CLI-Flags außer Kraft gesetzt werden kann.

**Was zu tun ist:**

* Bitten Sie einen Eigentümer in Ihrer Organisation, den **Routinen**-Schalter unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) zu aktivieren
* Für einmalige geplante Arbeiten, die keine Organisations-Routinen erfordern, siehe [geplante Aufgaben](/docs/de/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control erfordert die Anthropic API
</h3>

Die Sitzung spricht nicht direkt mit der Anthropic API, sodass es kein claude.ai-Backend für [Remote Control](/docs/de/remote-control) gibt, mit dem es sich verbinden kann.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com. CLAUDE_CODE_USE_BEDROCK is set, so this session is using Amazon Bedrock — unset it (or run in a shell without it) to use Remote Control.
```

Ein zweiter Satz erklärt, was die Sitzung von der Anthropic API weg geleitet hat; vor v2.1.219 war die Meldung nur der erste Satz. Je nach Ursache benennt die Meldung:

* Eine `CLAUDE_CODE_USE_*`-Provider-Variable, z. B. `CLAUDE_CODE_USE_BEDROCK` für [Amazon Bedrock](/docs/de/amazon-bedrock) oder `CLAUDE_CODE_USE_VERTEX` für [Google Cloud's Agent Platform](/docs/de/google-vertex-ai)
* [`ANTHROPIC_BASE_URL`](/docs/de/env-vars), die auf einen anderen Host als `api.anthropic.com` zeigt, z. B. ein [LLM-Gateway](/docs/de/llm-gateway) oder Proxy, selbst wenn Sie sich mit claude.ai anmelden; vor v2.1.196 blockierte eine benutzerdefinierte Basis-URL Remote Control nicht
* `ANTHROPIC_UNIX_SOCKET` gesetzt, sodass die Sitzung ihre Anfragen durch einen lokalen Socket statt an `api.anthropic.com` sendet
* Eine Enterprise-[Cloud-Gateway](/docs/de/claude-apps-gateway)-Anmeldung, die durch `/login` erfolgt ist, die Remote Control nicht unterstützt und keine Variable zum Aufheben hat

**Was zu tun ist:**

* Heben Sie die Variable auf, die die Meldung benennt, z. B. `CLAUDE_CODE_USE_BEDROCK` oder `ANTHROPIC_BASE_URL`, und starten Sie die Sitzung neu, oder starten Sie Remote Control von einer Sitzung, die direkt mit der Anthropic API spricht
* Wenn die Variable nicht in Ihrer Shell gesetzt ist, überprüfen Sie den `env`-Schlüssel in Ihren [Einstellungsdateien](/docs/de/settings#where-settings-live), die Umgebungsvariablen auf jede Sitzung anwenden
* Für diese und die anderen Remote Control-Startup-Meldungen siehe [Remote Control beheben](/docs/de/remote-control#troubleshooting)

<h3 id="remote-control-couldnt-refresh-your-login">
  Remote Control konnte Ihre Anmeldung nicht aktualisieren
</h3>

Claude Code führt eine Live-[Remote Control](/docs/de/remote-control)-Verbindung auf kurzlebigen Anmeldedaten aus, die es mit Ihrer gespeicherten claude.ai-Anmeldung erhält und erneuert. Wenn claude.ai diese Anmeldung nicht mehr akzeptiert oder Claude Code keine gespeicherte Anmeldung mehr hat, stoppt Claude Code Remote Control und benötigt Sie, sich erneut anzumelden. Jeder Fehler kann auftreten, während Claude Code noch verbunden wird, oder später, wenn es die Anmeldedaten erneuert.

Wenn Claude Code den Anmeldedienst auffordert, Ihre gespeicherte Anmeldung zu aktualisieren, und keine Antwort erhält, hält es Remote Control am Laufen und versucht die Aktualisierung erneut, während die Anmeldedaten der Verbindung noch gültig sind. Eine Aktualisierung erhält keine Antwort, wenn Claude Code den Anmeldedienst nicht erreichen kann, die Anfrage das Zeitlimit überschreitet oder der Dienst fehlschlägt, ohne Ihre Anmeldung abzulehnen. Wenn der Anmeldedienst immer noch nicht antwortet, wenn diese Anmeldedaten ablaufen, stoppt Claude Code Remote Control und meldet `OAuth token refresh failed`.

Wenn Claude Code Remote Control stoppt, zeigt es den Grund in einer Warnung und in einer Transkriptzeile an, die mit `Remote Control disconnected` beginnt. Ihre lokale Sitzung läuft ohne Remote Control weiter. Dieser Abschnitt behandelt diese Zeilen:

```text theme={null}
Remote Control disconnected — Claude.ai login expired — run /login to restore Remote Control
Remote Control disconnected — Claude.ai login expired — run /login, then /remote-control
Remote Control disconnected — Claude.ai login was rejected — run /login, then /remote-control
Remote Control disconnected — OAuth token unavailable — run /login to restore Remote Control
Remote Control disconnected — OAuth token refresh failed — run /login to re-authenticate
Remote Control disconnected — JWT refresh failed: no OAuth token — run /login
Remote Control disconnected — Signed out of Claude — run /login, then /remote-control
```

Claude Code benennt die Ursache in der Mitte der Meldung:

* `Claude.ai login expired` und `Claude.ai login was rejected`: claude.ai akzeptiert Ihr gespeichertes Anmelde-Token nicht mehr, weil es abgelaufen oder widerrufen wurde
* `OAuth token unavailable`: Claude Code hatte kein gespeichertes Anmelde-Token, als die Anmeldedaten der Verbindung zur Erneuerung fällig wurden
* `OAuth token refresh failed`: claude.ai lehnte Ihr gespeichertes Anmelde-Token ab, während Claude Code sich erneut verbunden hat, und das Aktualisieren des Tokens hat kein neues produziert
* `JWT refresh failed: no OAuth token`: Claude Code hat kein gespeichertes Anmelde-Token zum Erneuern gefunden
* `Signed out of Claude`: Sie haben sich auf diesem Computer abgemeldet, z. B. durch Ausführung von `/logout` in einem anderen Terminal, sodass Claude Code keine gespeicherte Anmeldung mehr hat, um die Verbindung mit zu erneuern

**Was zu tun ist:**

* Führen Sie `/login` aus, um sich erneut anzumelden
* Führen Sie `/remote-control` aus, um die Sitzung erneut zu verbinden. Meldungen, die mit `run /login to restore Remote Control` enden, benötigen diesen Schritt nicht: Claude Code verbindet sich automatisch erneut, sobald Sie sich anmelden.

Vor v2.1.224 las `OAuth token refresh failed — run /login to re-authenticate` `OAuth token refresh failed — re-authenticate, then re-enable Remote Control`, und `JWT refresh failed: no OAuth token — run /login` las `no OAuth token available for recovery (code <N>)`. Die Meldungen `Claude.ai login expired`, `Claude.ai login was rejected` und `OAuth token unavailable` wurden in v2.1.225 hinzugefügt.

Vor v2.1.238 meldete Claude Code die Fälle, die jetzt `Signed out of Claude` sagen, als `JWT refresh failed: no OAuth token — run /login`, und stoppt Remote Control mit `Claude.ai login expired — run /login to restore Remote Control`, sobald eine Anmelde-Aktualisierung keine Antwort erhielt.

<h3 id="remote-control-stopped-because-the-signed-in-account-changed">
  Remote Control wurde gestoppt, weil sich das angemeldete Konto geändert hat
</h3>

Claude Code zeigt diese Zeile während einer [Remote Control](/docs/de/remote-control)-Sitzung an, wenn Sie sich auf diesem Computer bei einem anderen claude.ai-Konto oder einer anderen Organisation anmelden. Sie haben den Wechsel außerhalb der Claude Code-Sitzung vorgenommen, z. B. durch Ausführung von `/login` in einem anderen Terminal.

Eine Remote Control-Sitzung, die Sie gestartet haben, während Sie sich durch `/login` angemeldet haben, gehört zum claude.ai-Konto und zur Organisation, die zum Zeitpunkt des Starts angemeldet waren.

```text theme={null}
Remote Control disconnected — signed-in claude.ai account or organization changed on this machine — run /remote-control to start a session for the current account, or /login to switch back, then /remote-control
```

Claude Code stoppt die Remote Control-Sitzung, sobald claude.ai bestätigt, dass sich das Konto oder die Organisation geändert hat. Ihre lokale Sitzung läuft ohne Remote Control weiter.

**Was zu tun ist:**

* Führen Sie `/remote-control` aus, um eine neue Remote Control-Sitzung unter dem aktuellen Konto oder der aktuellen Organisation zu starten
* Um zurückzuwechseln, führen Sie `/login` aus und melden Sie sich erneut bei dem vorherigen Konto oder der vorherigen Organisation an. Führen Sie dann `/remote-control` aus.

Vor v2.1.234 bemerkte Claude Code nicht, wenn Sie außerhalb der Claude Code-Sitzung zu einem anderen Konto oder einer anderen Organisation wechselten. Claude Code hielt die Remote Control-Sitzung verbunden, bis eine spätere Anfrage an den Remote Control-Server mit `Remote Control server rejected the request (HTTP 404)` fehlschlug. Dieser Fehler konnte Stunden nach dem Wechsel auftreten.

<h3 id="remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts">
  Remote Control wurde gestoppt, weil sich die App, die die Sitzung ausführt, abgemeldet oder Konten gewechselt hat
</h3>

Wenn die Claude-Desktop-App oder eine IDE Ihre Sitzung hostet, erhält Claude Code sein Anmelde-Token von dieser App statt von `/login`. Wenn claude.ai dieses Token ablehnt, fordert Claude Code die App auf, ein neues zu erhalten. Wenn die App antwortet, dass sie abgemeldet ist, oder dass sie jetzt bei einem anderen Claude-Konto angemeldet ist, beendet Claude Code die [Remote Control](/docs/de/remote-control)-Sitzung und sendet der App eine dieser Zeilen:

```text theme={null}
Remote Control stopped — the app running this session is now signed in to a different Claude account
Remote Control stopped — the app running this session is signed out of Claude. Sign in there, then turn Remote Control back on
```

Ihre lokale Sitzung läuft ohne Remote Control weiter.

**Was zu tun ist:**

* Wenn die App abgemeldet ist, melden Sie sich erneut an und schalten Sie Remote Control dann wieder ein
* Wenn die App Konten gewechselt hat, kann Claude Code die beendete Sitzung nicht unter dem neuen Konto fortsetzen. Starten Sie eine neue Remote Control-Sitzung unter diesem Konto.

Vor v2.1.238 sendete Claude Code der App die unter [Remote Control konnte Ihre Anmeldung nicht aktualisieren](#remote-control-couldnt-refresh-your-login) aufgelisteten `run /login`-Meldungen in beiden Fällen.

<h3 id="oauth-token-revoked-or-expired">
  OAuth-Token widerrufen oder abgelaufen
</h3>

Ihre gespeicherte Anmeldung ist nicht mehr gültig. Ein widerrufenes Token bedeutet, dass Sie sich überall abgemeldet haben oder ein Administrator den Zugriff entfernt hat; ein abgelaufenes Token bedeutet, dass die automatische Aktualisierung während der Sitzung fehlgeschlagen ist.

Beide Meldungen melden eine Ablehnung, die die API für eine Anfrage zurückgegeben hat, die Claude Code gesendet hat. Wenn die gespeicherte Anmeldung bereits nach einer fehlgeschlagenen Aktualisierung gelöscht wurde, sehen Sie stattdessen [Anmeldung abgelaufen](#login-expired). Wenn Sie sich mit einem langlebigen Token in [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/de/env-vars) authentifizieren, sehen Sie die gleichen Meldungen, wenn dieses Token abläuft oder widerrufen wird.

```text theme={null}
OAuth token revoked · Please run /login
Please run /login · API Error: 401 OAuth token has expired ...
```

**Was zu tun ist:**

* Führen Sie `/login` aus, um sich erneut anzumelden
* Wenn der Fehler nach der erneuten Authentifizierung in derselben Sitzung zurückkehrt, führen Sie zuerst `/logout` aus, um das gespeicherte Token vollständig zu löschen, dann `/login`
* Wenn Sie sich mit der Umgebungsvariable `CLAUDE_CODE_OAUTH_TOKEN` authentifizieren, sendet Claude Code weiterhin den Wert, den Sie nach einem Anfragefehler mit 401 gesetzt haben, anstatt zu einem gespeicherten Anmelde-Token zu wechseln. [`/status`](/docs/de/commands) zeigt diese Anmeldedaten als eine `Auth token`-Zeile an, die `CLAUDE_CODE_OAUTH_TOKEN` liest. Generieren Sie ein frisches Token mit [`claude setup-token`](/docs/de/authentication#generate-a-long-lived-token) und starten Sie damit neu, oder heben Sie die Variable auf und führen Sie `/login` aus. Vor v2.1.225 konnte Claude Code den Wert der Variable während der Sitzung durch das kurzlebige Zugriffs-Token aus einer gespeicherten Anmeldung ersetzen, und die Sitzung schlug mit 401-Fehlern erneut fehl, sobald dieses Token abgelaufen war.
* Für wiederholte Anmeldungsaufforderungen über Starts hinweg siehe die Systemuhr-Überprüfungen und macOS-Wiederherstellungsschritte in [Fehlerbehebung](/docs/de/troubleshoot-install#not-logged-in-or-token-expired)
* Für andere Fehler, einschließlich `403 Forbidden` und OAuth-Browser-Probleme, siehe [Anmeldung und Authentifizierung](/docs/de/troubleshoot-install#login-and-authentication)

<h3 id="api-error-401-invalid-authentication-credentials">
  API-Fehler: 401 Ungültige Authentifizierungsanmeldedaten
</h3>

Die API erkannte das Format Ihrer Anmeldedaten, lehnte aber das Konto oder die Organisation dahinter ab. Anthropic gibt diese Meldung zurück, wenn eine Anmeldedaten kürzlich widerrufen wurde, wenn eine Organisation deaktiviert wurde oder Ihren Zugriff entfernt hat, oder wenn das Konto selbst deaktiviert wurde, sodass ein abgelaufenes Token nicht die Ursache ist. Die Anmeldedaten können Ihre gespeicherte Anmeldung oder ein genehmigter `ANTHROPIC_API_KEY` sein, und die Behebung unterscheidet sich, also beginnen Sie mit dem Ausführen von `/status`, um zu sehen, welche aktiv ist.

```text theme={null}
Please run /login · API Error: 401 Invalid authentication credentials
```

**Was zu tun ist:**

* Wenn `/status` eine `API key`-Zeile zeigt, die nicht als nicht in Gebrauch markiert ist, ist ein genehmigter [`ANTHROPIC_API_KEY`](/docs/de/authentication#authentication-precedence) die aktive Anmeldedaten und hat Vorrang vor Ihrer Anmeldung, sodass `/login` ihn nicht ersetzt. Rotieren Sie den Schlüssel in der Claude Console, oder greifen Sie auf Ihr Abonnement zurück, indem Sie `unset ANTHROPIC_API_KEY` ausführen, oder in PowerShell `Remove-Item Env:ANTHROPIC_API_KEY`.
* Wenn `/status` nur Ihre Anmeldung zeigt, führen Sie `/login` einmal aus. Wenn die Anmeldedaten widerrufen wurden, ersetzt eine frische Anmeldung sie.
* Wenn die gleiche Meldung für das gleiche Anmeldekonto zurückkehrt, ist das Konto oder die Organisation nicht mehr aktiv. Überprüfen Sie das Konto und die Organisation, die `/status` meldet, und bitten Sie Ihren Organisations-Administrator, den Zugriff wiederherzustellen.
* Wenn [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) auf ein [LLM-Gateway](/docs/de/llm-gateway) zeigt, ist der Text nach `401` die Meldung Ihres Gateways statt Anthropic's, und `/login` ändert ihn nicht. Korrigieren Sie stattdessen die Anmeldedaten, die Ihr Gateway erwartet.

<h3 id="login-expired">
  Anmeldung abgelaufen
</h3>

Claude Code versuchte, Ihre gespeicherte claude.ai- oder Claude Console-Anmeldung zu erneuern, und der OAuth-Dienst lehnte das gespeicherte Aktualisierungs-Token ab, sodass Claude Code die gespeicherten Anmeldedaten gelöscht hat. Danach stoppt jede Modellanfrage lokal mit dieser Meldung, bevor sie die API erreicht, weil nur `/login` neue Anmeldedaten erstellen kann.

Vor v2.1.206 sendete Claude Code die Modellanfrage trotzdem mit allen verbleibenden Anmeldedaten in der Umgebung, und jedes Modell schlug dann mit [Es gibt ein Problem mit dem ausgewählten Modell](#theres-an-issue-with-the-selected-model) oder einem 401 statt einer Aufforderung zur Anmeldung fehl.

```text theme={null}
Login expired · Please run /login
```

Im [nicht-interaktiven Modus](/docs/de/headless) (`-p`) und dem [Agent SDK](/docs/de/agent-sdk/overview) liest sich die Meldung wie folgt, und der strukturierte Fehlercode ist `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Dies ist nicht der gleiche Zustand wie [OAuth-Token widerrufen oder abgelaufen](#oauth-token-revoked-or-expired). Diese Meldungen melden eine Ablehnung, die die API zurückgegeben hat. Claude Code selbst produziert `Login expired` für eine Anmeldung, die es bereits nicht erneuern konnte, sodass es keine Anfrage sendet. Wenn die Erneuerung fehlschlägt, weil das Konto selbst ausgesetzt ist, statt dass die Anmeldung veraltet ist, zeigt Claude Code stattdessen [Ihr Konto ist gesperrt](#your-account-is-on-hold).

Sitzungen, die sich mit einem API-Schlüssel, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/de/env-vars) oder einem Drittanbieter-Provider authentifizieren, verwenden nicht die gespeicherte Anmeldung und sehen diese Meldung nie.

Sie können diesen Zustand vor einem Anfragefehler überprüfen: [`/status`](/docs/de/commands) zeigt eine `Login`-Zeile an, die `Expired — log in again` liest, plus die Organisation und E-Mail, die sie für die abgelaufene Anmeldung gespeichert hat. Die Zeile erscheint nur, wenn die gespeicherte Anmeldung Ihre aktive Anmeldedaten ist und nicht mehr erneuert werden kann. Sitzungen, die sich auf andere Weise authentifizieren, zeigen die Zeile nicht, selbst wenn eine abgelaufene Anmeldung gespeichert bleibt. Vor v2.1.210 gab `/status` in diesem Zustand keinen Hinweis darauf, dass eine Anmeldung jemals existiert hatte, weil die gelöschte Anmeldedaten es nichts zu melden ließ.

**Was zu tun ist:**

* Führen Sie `/login` aus, um sich erneut anzumelden. Das Wiederholen ohne Anmeldung zeigt die gleiche Meldung bei jeder Anfrage.
* Im nicht-interaktiven Modus führen Sie `claude` in der gleichen Umgebung aus, führen Sie `/login` aus, dann führen Sie Ihren Befehl erneut aus. Für Automatisierung, die sich nicht interaktiv anmelden kann, authentifizieren Sie sich mit `ANTHROPIC_API_KEY` oder [generieren Sie ein langlebiges Token mit `claude setup-token`](/docs/de/authentication#generate-a-long-lived-token).
* Wenn die Anmeldung weiterhin fehlschlägt, siehe [Anmeldung und Authentifizierung](/docs/de/troubleshoot-install#login-and-authentication)

<h3 id="claude-login-not-accepted">
  Claude-Anmeldung nicht akzeptiert
</h3>

Sie versuchten, eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web) zu starten, und der Server weigerte sich, sie mit einem 401 zu erstellen: er akzeptierte die Claude-Anmeldung, die dieser Computer gesendet hat, nicht, normalerweise weil die Anmeldung abgelaufen oder widerrufen wurde.

Der erste Teil der Zeile ist der eigene Grund des Servers, wenn er einen gibt. Andernfalls liest sich die Zeile:

```text theme={null}
Claude login not accepted · Run /login, then try again
```

**Was zu tun ist:**

* Führen Sie `/login` aus, schließen Sie die Anmeldung ab, dann starten Sie die Sitzung erneut

<h3 id="artifacts-need-a-claude-ai-login">
  Artefakte benötigen eine claude.ai-Anmeldung
</h3>

Claude Code weigerte sich, ein [Artefakt](/docs/de/artifacts) zu veröffentlichen oder zu lesen, weil die Sitzung keine claude.ai-Anmeldung hat, die es für Artefakte verwenden kann.

Jede Form der Meldung beginnt mit den gleichen Worten, gefolgt von einem Mittel, das davon abhängt, wie sich Ihre Sitzung authentifiziert. Ohne konkurrierende Anmeldedaten liest es sich:

```text theme={null}
Artifacts need a claude.ai login. Run /login and select "Claude account with subscription", then retry — the "Anthropic Console account" option does not provide claude.ai credentials.
```

**Was zu tun ist:**

* Führen Sie `/login` aus und wählen Sie **Claude account with subscription**. Die Option **Anthropic Console account** bietet keine claude.ai-Anmeldedaten.
* Wenn die Meldung eine Anmeldedaten benennt, die Vorrang hat, z. B. `ANTHROPIC_API_KEY`, eine `apiKeyHelper`-Einstellung oder einen Console-Schlüssel, der durch eine vorherige `/login` gespeichert wurde, entfernen Sie ihn auf die Weise, die die Meldung sagt, dann führen Sie `/login` aus
* Wenn die Meldung sagt, dass diese Remote-Sitzung sich durch den Computer authentifiziert, der sie gestartet hat, melden Sie sich auf diesem Computer bei claude.ai an, dann verbinden Sie die Sitzung erneut
* Wenn die Meldung sagt, dass die Anmeldedaten von der Sitzungs-Host-Umgebung injiziert werden, können Sie sie in dieser Sitzung nicht ändern; starten Sie eine Sitzung, die bei claude.ai angemeldet ist
* Siehe [Verfügbarkeit](/docs/de/artifacts#availability) für die anderen Anforderungen, die Artefakte haben, z. B. Plan, Modell-Provider und Organisations-Richtlinie

<h3 id="administrator-policy-requires-a-cloud-gateway-sign-in">
  Administrator-Richtlinie erfordert eine Cloud-Gateway-Anmeldung
</h3>

Ein Administrator's [verwaltete Einstellungen](/docs/de/managed-settings) auf diesem Computer setzen [`forceLoginMethod`](/docs/de/settings-reference#forceloginmethod) auf `"gateway"` oder setzen [`forceLoginGatewayUrl`](/docs/de/settings-reference#forcelogingatewayurl). Sofern Sie nicht einen Cloud-Provider durch eine Variable wie `CLAUDE_CODE_USE_BEDROCK` auswählen, akzeptiert Claude Code dann nur die [Claude apps gateway](/docs/de/claude-apps-gateway)-Anmeldung. Sie sehen eine von zwei Meldungen:

```text theme={null}
Not signed in to the Cloud gateway — run /login.
```

Modellanfragen schlagen mit dieser Meldung fehl, wenn die Sitzung keine Gateway-Anmeldung hat, z. B. weil Sie `/login` nicht seit der Ankunft der Richtlinie auf dem Computer ausgeführt haben.

Wenn Sie auch einen `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder `apiKeyHelper`-Anmeldedaten konfiguriert haben und die verwalteten Einstellungen `forceLoginMethod` setzen, beendet Claude Code beim Start statt mit einer Meldung, die beginnt:

```text theme={null}
Administrator policy requires a Cloud gateway sign-in on this machine; the
Anthropic-issued credential configured here (ANTHROPIC_API_KEY,
ANTHROPIC_AUTH_TOKEN, or apiKeyHelper) is not used.
```

**Was zu tun ist:**

* Führen Sie `/login` aus und schließen Sie die Anmeldung auf dem **Cloud gateway**-Bildschirm ab
* Für die Startup-Meldung entfernen Sie die `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder `apiKeyHelper`-Einstellung, die Sie konfiguriert haben, starten Sie dann `claude` und führen Sie `/login` aus
* Wenn Sie glauben, dass der Computer das Gateway nicht erfordern sollte, bitten Sie den Administrator, der ihn verwaltet, `forceLoginMethod` und `forceLoginGatewayUrl` aus seinen verwalteten Einstellungen zu entfernen

In v2.1.265 zeigte eine Regression auch die erste Meldung in einigen LLM-Gateway- und Proxy-Konfigurationen, die sich mit einem API-Schlüssel, `apiKeyHelper` oder benutzerdefinierten Headern authentifizieren, selbst ohne Administrator-Anforderung auf dem Computer. Aktualisieren Sie auf v2.1.266 oder später. Sie müssen Ihre Konfiguration nicht ändern.

Vor v2.1.261 verwendete Claude Code auf Computern, die `forceLoginMethod` auf `"gateway"` setzen, eine verbleibende gespeicherte Anmeldung statt Modellanfragen fehlschlagen zu lassen, und meldete eine konfigurierte Umgebungs-Anmeldedaten mit `This machine's managed settings require a first-party login` statt der Startup-Meldung. Vor v2.1.265 erforderte ein Computer, dessen verwaltete Einstellungen nur `forceLoginGatewayUrl` setzen, nicht die Gateway-Anmeldung, und Claude Code verwendete eine verbleibende Anmeldedaten dort.

<h3 id="your-account-is-on-hold">
  Ihr Konto ist gesperrt
</h3>

Das Claude-Konto hinter Ihrer Anmeldung wurde ausgesetzt. Claude Code zeigt die erste Meldung, wenn es versucht, Ihre gespeicherte Anmeldung zu erneuern und von der Sperrung erfährt, und die zweite, wenn eine Anmeldung, die Sie im Browser abschließen, dies meldet:

```text theme={null}
Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted
Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted
```

Die erneute Anmeldung mit dem gleichen Konto löscht die Meldung nicht, weil die Sperrung auf dem Konto liegt, nicht auf der Anmeldung. Im [nicht-interaktiven Modus](/docs/de/headless) (`-p`) und dem [Agent SDK](/docs/de/agent-sdk/overview) ist der strukturierte Fehlercode `account_on_hold`. Vor v2.1.235 meldete Claude Code ein gesperrtes Konto als [Anmeldung abgelaufen · Bitte führen Sie /login aus](#login-expired), dessen Wiederherstellungsschritte eine Sperrung nicht löschen können.

**Was zu tun ist:**

* Öffnen Sie den Link in der Meldung, um die Details der Sperrung anzuzeigen oder dagegen Einspruch zu erheben
* Wenn Sie ein anderes Claude-Konto oder einen API-Schlüssel haben, der von der Sperrung nicht betroffen ist, können Sie weiterarbeiten, während die Sperrung gelöst wird: führen Sie `/login` mit diesem Konto aus, oder setzen Sie den Schlüssel mit `ANTHROPIC_API_KEY`

<h3 id="anthropic-profile-login-expired">
  Anthropic-Profil-Anmeldung abgelaufen
</h3>

Claude Code authentifiziert sich durch ein Anthropic-Anmeldedaten-Profil, dessen gespeicherte Anmeldungs-Anmeldedaten abgelaufen ist, und das Profil hält keine Aktualisierungs-Anmeldedaten, die Claude Code verwenden kann, um sie zu erneuern. Claude Code stoppt jede Anfrage lokal, ohne zu wiederholen, weil eine Wiederholung die gleiche abgelaufene Anmeldedaten lesen würde.

```text theme={null}
Anthropic profile login expired · Re-authenticate your Anthropic profile
Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile
```

Dies erscheint nur, wenn die aktive Anmeldedaten von einem Anthropic-Anmeldedaten-Profil stammt, das Sie mit der Umgebungsvariable `ANTHROPIC_PROFILE` auswählen, das Claude Code als das aktive Profil in Ihrem Anthropic-Konfigurationsverzeichnis entdeckt, oder das Claude Code schrieb, als Sie sich [ohne API-Schlüssel anmeldeten](/docs/de/authentication#sign-in-without-an-api-key). Sitzungen, die sich mit der claude.ai-Option von `/login`, einem API-Schlüssel, einem Bearer-Token wie `ANTHROPIC_AUTH_TOKEN` oder einem Drittanbieter-Provider authentifizieren, sehen diese Meldung nie.

Auf einem Computer, der [die schlüssellose Anmeldung anbietet](/docs/de/authentication#sign-in-without-an-api-key), führen Sie `/login` aus, wählen Sie das Anthropic Console-Konto und melden Sie sich erneut an, um ein Profil zu erneuern, das die schlüssellose Console-Anmeldung oder die Claude Platform CLI's `ant auth login` schrieb. Claude Code ersetzt die abgelaufene Anmeldedaten in diesem Profil. Für ein Verbund-Profil oder eines, das ein anderes Tool erstellt hat, erneuert `/login` die Anmeldedaten nicht. Welche Form Sie sehen, hängt davon ab, ob Sie das Profil ausgewählt haben oder Claude Code es entdeckt hat:

* Wenn Sie `ANTHROPIC_PROFILE` explizit setzen, endet die Meldung mit `Re-authenticate your Anthropic profile`.
* Wenn Claude Code das Profil aus Ihrem Konfigurationsverzeichnis entdeckt hat, bietet die Meldung `/login` an, weil Claude Code einer funktionierenden `/login` Vorrang vor dem entdeckten Profil gibt und sich dann stattdessen mit Ihrem claude.ai- oder Console-Konto authentifiziert. Vor v2.1.234 zeigte Claude Code in diesem Fall auch die Form `Re-authenticate your Anthropic profile`.

**Was zu tun ist:**

* Melden Sie sich erneut bei dem Profil an, dann versuchen Sie es erneut: Auf einem Computer, der [die schlüssellose Anmeldung anbietet](/docs/de/authentication#sign-in-without-an-api-key), führen Sie `/login` aus und wählen Sie das Anthropic Console-Konto für ein Profil, das die schlüssellose Console-Anmeldung oder die Claude Platform CLI's `ant auth login` schrieb; für andere Profile verwenden Sie das Tool, das sie erstellt hat
* Wenn ein Administrator das Profil-Anmeldedaten bereitgestellt hat, bitten Sie ihn, ein neues auszustellen
* Führen Sie `/status` aus, um die aktive Anmeldedatenquelle und den Profilnamen zu bestätigen
* Um die Verwendung des Profils zu beenden, heben Sie `ANTHROPIC_PROFILE` auf, wenn Sie es gesetzt haben, dann authentifizieren Sie sich auf andere Weise, z. B. `/login` oder `ANTHROPIC_API_KEY`

<h3 id="oauth-scope-requirement">
  OAuth-Bereichsanforderung
</h3>

Das gespeicherte Token stammt von vor einem Berechtigungsbereich, den ein neueres Feature benötigt. Sie sehen dies am häufigsten von `/usage` und dem Nutzungsindikator der Statuszeile:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**Was zu tun ist:**

* Führen Sie `/login` aus, um ein neues Token mit den aktuellen Bereichen zu erhalten. Sie müssen sich nicht zuerst abmelden.

<h3 id="claude-ai-rejected-the-session-token">
  claude.ai lehnte das Sitzungs-Token ab
</h3>

Eine [claude.ai-Connector](/docs/de/mcp#use-mcp-servers-from-claude-ai)-Anfrage schlug fehl, weil claude.ai das Token aus Ihrer Claude Code-Anmeldung ablehnte, normalerweise eine Anmeldung, die abgelaufen ist und nicht erneuert werden konnte. Das abgelehnte Token ist Ihre Anmeldung, nicht die eigene Autorisierung des Connectors in claude.ai, sodass die erneute Autorisierung des Connectors es nicht behebt. In `/mcp` zeigt der Connector als `connected · session token rejected` und seine Detailansicht liest:

```text theme={null}
claude.ai rejected the session token. Run /login, then reconnect.
```

**Was zu tun ist:**

* Führen Sie `/login` aus, um sich erneut anzumelden
* Verbinden Sie den Connector von `/mcp` erneut, oder führen Sie `/mcp reconnect <server>` aus. Das Erneute Verbinden, bevor Sie sich erneut anmelden, lässt den Connector im gleichen Zustand. Die Option **Reconnect** des `/mcp`-Panels meldet `your claude.ai session token was rejected`; das eingegebene `/mcp reconnect <server>`-Formular meldet eine erfolgreiche Wiederverbindung, obwohl das Token immer noch abgelehnt wird.

Vor v2.1.222 markierte Claude Code den Connector als Authentifizierung erforderlich, was Sie auf den Autorisierungsfluss des Connectors hinwies, obwohl das Abschließen ihn nicht behob.

<h3 id="mcp-server-needs-you-to-sign-in-again">
  MCP-Server benötigt Sie, sich erneut anzumelden
</h3>

Ein Remote-[MCP-Server](/docs/de/mcp) lehnte die Anmeldedaten bei einem Tool-Aufruf während der Sitzung ab, normalerweise weil eine Anmeldung oder ein Token abgelaufen ist oder weil das Token einen Berechtigungsbereich fehlt, den das Tool benötigt. Der Tool-Aufruf schlägt fehl, und `/mcp` markiert den Server als [Authentifizierung erforderlich](/docs/de/mcp#authenticate-with-remote-mcp-servers).

Für einen Server, bei dem Sie sich von Claude Code aus anmelden, einschließlich eines claude.ai-Connectors, ist die Anmeldung abgelaufen oder wurde widerrufen:

```text theme={null}
MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)
```

Führen Sie `/mcp` aus, wählen Sie den Server und melden Sie sich erneut von seinem Menü an.

Für einen Server, der mit einem [`headersHelper`](/docs/de/mcp#use-dynamic-headers-for-custom-authentication)-Skript konfiguriert ist, hat Claude Code das Helper-Skript bereits erneut ausgeführt und den Aufruf einmal erneut versucht, bevor dies angezeigt wird:

```text theme={null}
MCP server "<name>" rejected the credential from its headersHelper (check the helper and run /mcp to reconnect, or to authenticate if the server also uses OAuth)
```

Überprüfen Sie, dass der Helper eine Anmeldedaten zurückgibt, die der Server akzeptiert, dann verbinden Sie sich von `/mcp` erneut, das den Helper erneut ausführt.

Für einen Server mit einem statischen `Authorization`-Header in seiner Konfiguration:

```text theme={null}
MCP server "<name>" rejected the Authorization header in its config (update it, then run /mcp to reconnect)
```

Aktualisieren Sie den Header-Wert, wo der Server konfiguriert ist, dann verbinden Sie sich von `/mcp` erneut.

Vor v2.1.273 zeigten alle drei Fälle `MCP server "<name>" requires re-authorization (token expired)`.

Ein Server kann auch einen Tool-Aufruf mit HTTP 403 `insufficient_scope` ablehnen, um Sie aufzufordern, einen Bereich zu autorisieren, manchmal einen, den Ihr Token bereits auflistet. Die Meldung benennt diesen Bereich:

```text theme={null}
MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate
```

Führen Sie `/mcp` aus, wählen Sie den Server und authentifizieren Sie sich erneut von seinem Menü.

Wenn die Konfiguration des Servers weder [`oauth.scopes`](/docs/de/mcp#restrict-oauth-scopes) noch [`authServerMetadataUrl`](/docs/de/mcp#override-oauth-metadata-discovery) setzt, fordert Claude Code den Bereich an, den der Server benannt hat. Mit einer der beiden Einstellungen fordert Claude Code stattdessen die Bereiche dieser Einstellung an. Wenn Sie `oauth.scopes` angeheftet haben, fügen Sie den fehlenden Bereich zu dieser Liste hinzu, bevor Sie sich erneut authentifizieren.

Vor v2.1.274 zeigte dieser Fall die `needs you to sign in again`-Meldung, und vor v2.1.273 zeigte er `requires re-authorization (token expired)` wie die anderen Fälle.

<h3 id="issuer-mismatch-in-authorization-response">
  Aussteller-Nichtübereinstimmung in der Autorisierungsantwort
</h3>

Während einer [MCP OAuth-Anmeldung](/docs/de/mcp#authenticate-with-remote-mcp-servers) leitete der Autorisierungsserver mit einem `iss`-Parameter zu Claude Code zurück, der nicht den Aussteller benennt, den Claude Code aus den OAuth-Metadaten des Servers erwartet. Ein falscher Aussteller in diesem Schritt ist, wie ein Autorisierungsserver-Mix-up-Angriff aussieht, sodass Claude Code die Anmeldung fehlschlägt, statt den Autorisierungscode auszutauschen. Claude Code zeigt den Fehler im `/mcp`-Server-Menü nach der Browser-Anmeldung:

```text theme={null}
Issuer mismatch in authorization response (RFC 9207): expected "https://auth.example.com", received "https://other.example.com"
```

`expected` ist der Aussteller aus den OAuth-Metadaten des Servers, und `received` ist der `iss`-Wert, den die Umleitung trug. Eine Anmeldung, deren Umleitung keinen `iss`-Parameter trägt, besteht die Überprüfung, es sei denn, die Metadaten des Servers setzen `authorization_response_iss_parameter_supported`, in welchem Fall Claude Code die Anmeldung fehlschlägt.

**Was zu tun ist:**

* Versuchen Sie die Anmeldung erneut von `/mcp`
* Wenn der Fehler wiederholt wird, melden Sie ihn dem Server-Betreiber. Die Behebung ist serverseitig: Der Autorisierungsserver muss den gleichen Aussteller im `iss`-Parameter zurückgeben, den er in seinen Metadaten bewirbt
* Um eine Verbindung herzustellen, während der Server behoben wird, starten Sie Claude Code mit [`MCP_SDK_GENERATION=v1`](/docs/de/env-vars), dessen [Runtime](/docs/de/mcp#mcp-client-runtimes) diese Überprüfung nicht ausführt. Dies entfernt einen Schutz vor Mix-up-Angriffen, daher bevorzugen Sie die serverseitige Behebung

Vor v2.1.232 verwendete Claude Code die v2-Runtime nur in einem schrittweisen Rollout oder wenn Sie `MCP_SDK_GENERATION=v2` setzten.

<h3 id="aws-credentials-expired-or-invalid">
  AWS-Anmeldedaten abgelaufen oder ungültig
</h3>

Ihr AWS-Sitzungs-Token ist abgelaufen oder wurde abgelehnt. Diese Meldung erscheint auf einem 401 von [Claude Platform on AWS](/docs/de/claude-platform-on-aws) oder dem [Mantle-Endpunkt](/docs/de/amazon-bedrock#use-the-mantle-endpoint), wie diese Provider ein abgelaufenes Sicherheits-Token melden.

Der Aktionshinweis in der Mitte variiert mit Ihrem Setup. Der stabile Teil ist der führende `AWS credentials expired or invalid`:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Vor v2.1.273 erschien diese Meldung nur, wenn `awsAuthRefresh` konfiguriert war.

**Was zu tun ist:**

* Wenn der Hinweis sagt, dass Anmeldedaten von dieser Umgebung verwaltet werden, besitzt die App, die Claude Code gestartet hat, die Anmeldedaten und die anderen Schritte hier gelten nicht: versuchen Sie es erneut, oder kontaktieren Sie Ihren Administrator
* Wenn [`awsAuthRefresh`](/docs/de/amazon-bedrock#advanced-credential-configuration) gesetzt ist, führen Sie den in der Meldung benannten Befehl aus, z. B. `aws sso login --profile myprofile`, in einem anderen Terminal aus und schließen Sie die Browser-Anmeldung ab, dann versuchen Sie es erneut. Andernfalls aktualisieren Sie die AWS-Anmeldedaten, die Sie selbst verwenden: Ihre SSO-Anmeldung, Zugriffschlüssel, API-Schlüssel oder Proxy-Token
* Mit `awsAuthRefresh` in einer interaktiven Sitzung können Sie stattdessen `/login` ausführen, **3rd-party platform** wählen, dann **Claude Platform on AWS · refresh credentials** unter **Using 3rd-party platforms** wählen, um den gleichen Befehl auszuführen, ohne Claude Code neu zu starten. Siehe [AWS-Anmeldedaten konfigurieren](/docs/de/claude-platform-on-aws#1-configure-aws-credentials)
* Wenn der Fehler nach dem erfolgreichen Aktualisierungsbefehl wiederholt wird, bestätigen Sie, dass die Identität außerhalb von Claude Code mit `aws sts get-caller-identity` in der gleichen Shell und dem gleichen Profil gültig ist

<h3 id="aws-authentication-failed">
  AWS-Authentifizierung fehlgeschlagen
</h3>

Ihr AWS-Provider hat einen 403 zurückgegeben, oder [Amazon Bedrock](/docs/de/amazon-bedrock) hat einen 401 zurückgegeben.

Amazon Bedrock meldet ein abgelaufenes Sicherheits-Token als 403, aber ein 403 ist auch, wie es eine Autorisierungsverweigerung meldet, z. B. eine `AccessDeniedException` von einer fehlenden IAM-Berechtigung. Claude Code kann diese beiden Ursachen nicht unterscheiden.

Ein 401 von Amazon Bedrock landet auch hier statt unter [AWS-Anmeldedaten abgelaufen oder ungültig](#aws-credentials-expired-or-invalid), weil Amazon Bedrock ein abgelaufenes Token nicht als 401 meldet. Ein 401 von diesem Endpunkt kommt normalerweise von etwas anderem im Anfragepfad, z. B. einem Unternehmens-Proxy.

Eine Anmeldedaten-Aktualisierung behebt ein abgelaufenes Token und kann die anderen Ursachen nicht beheben, sodass die Meldung beide anbietet:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

Der Aktionshinweis in der Mitte variiert mit Ihrem Setup. Der stabile Teil ist der führende `AWS authentication failed`.

Wenn der 403 Amazon Bedrock's Antwort ist, dass Sie keinen Zugriff auf das Modell mit der angegebenen Modell-ID haben, sagt der Hinweis stattdessen, dass Sie das Modell für Ihr Konto und Ihre Region in der Amazon Bedrock-Konsole aktivieren.

Vor v2.1.273 erschien diese Meldung nur, wenn `awsAuthRefresh` konfiguriert war.

**Was zu tun ist:**

* Wenn der Hinweis sagt, dass Anmeldedaten von dieser Umgebung verwaltet werden, besitzt die App, die Claude Code gestartet hat, die Anmeldedaten und die anderen Schritte hier gelten nicht: versuchen Sie es erneut, oder kontaktieren Sie Ihren Administrator
* Aktualisieren Sie Ihre AWS-Anmeldedaten, falls ein abgelaufenes Anmeldedaten die Ursache ist: führen Sie den [`awsAuthRefresh`](/docs/de/amazon-bedrock#advanced-credential-configuration)-Befehl aus, der in der Meldung benannt ist, wenn einer gesetzt ist, oder aktualisieren Sie Ihre SSO-Anmeldung, Zugriffschlüssel, API-Schlüssel oder Proxy-Token selbst
* Wenn Ihre Anmeldedaten aktuell sind, bestätigen Sie die IAM-Berechtigungen in [IAM-Konfiguration](/docs/de/amazon-bedrock#iam-configuration), die an die Identität angehängt sind, die Sie verwenden, und dass das ausgewählte Modell für Ihr Konto und Ihre Region aktiviert ist
* Führen Sie `aws sts get-caller-identity` aus, um zu bestätigen, welche Identität Ihre Anfragen verwenden; ein veraltetes `AWS_PROFILE` oder Standardprofil ist eine häufige Ursache für eine Berechtigungsnichtübereinstimmung

<h3 id="google-cloud-credentials-expired-or-invalid">
  Google Cloud-Anmeldedaten abgelaufen oder ungültig
</h3>

Ihre Google Cloud-Anmeldedaten für [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) sind abgelaufen oder wurden abgelehnt: die Anfrage hat einen 401 zurückgegeben, wie Agent Platform Anmeldedaten-Ablauf meldet.

Der Aktionshinweis in der Mitte variiert mit Ihrem Setup. Der stabile Teil ist der führende `Google Cloud credentials expired or invalid`:

```text theme={null}
Google Cloud credentials expired or invalid · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · API Error: 401 ...
```

**Was zu tun ist:**

* Wenn der Hinweis sagt, dass Anmeldedaten von dieser Umgebung verwaltet werden, besitzt die App, die Claude Code gestartet hat, die Anmeldedaten und die anderen Schritte hier gelten nicht: versuchen Sie es erneut, oder kontaktieren Sie Ihren Administrator
* Wenn Sie sich mit Anwendungsstandardanmeldedaten authentifizieren, führen Sie den [`gcpAuthRefresh`](/docs/de/google-vertex-ai#advanced-credential-configuration)-Befehl aus, der in der Meldung benannt ist, oder `gcloud auth application-default login`, und schließen Sie die Anmeldung ab, dann versuchen Sie es erneut
* Wenn Sie durch ein [LLM-Gateway](/docs/de/llm-gateway) mit `CLAUDE_CODE_SKIP_VERTEX_AUTH` gesetzt leiten, aktualisieren Sie das Gateway-Token in `ANTHROPIC_AUTH_TOKEN` oder `ANTHROPIC_CUSTOM_HEADERS`, dann versuchen Sie es erneut
* Wenn Sie sich mit einer Service-Account-Schlüsseldatei authentifizieren, bestätigen Sie, dass `GOOGLE_APPLICATION_CREDENTIALS` auf einen gültigen Schlüssel zeigt. Siehe [GCP-Anmeldedaten konfigurieren](/docs/de/google-vertex-ai#3-configure-gcp-credentials)
* Wenn der Fehler nach einer Aktualisierung wiederholt wird, bestätigen Sie, dass die Identität außerhalb von Claude Code mit `gcloud auth application-default print-access-token` in der gleichen Shell funktioniert

Vor v2.1.273 zeigte ein 401 von Agent Platform die generische `Please run /login`- oder `Failed to authenticate`-Meldung statt, die Google Cloud-Anmeldedaten nicht aktualisieren kann.

<h3 id="google-cloud-authentication-failed">
  Google Cloud-Authentifizierung fehlgeschlagen
</h3>

[Google Cloud's Agent Platform](/docs/de/google-vertex-ai) hat einen 403 zurückgegeben, den es für Autorisierungsverweigerungen statt abgelaufener Anmeldedaten verwendet. Normalerweise fehlt der Identität, mit der Sie sich authentifizieren, eine IAM-Berechtigung, oder das Modell ist nicht für Ihr Projekt aktiviert.

Der Aktionshinweis in der Mitte variiert mit Ihrem Setup. Der stabile Teil ist der führende `Google Cloud authentication failed`:

```text theme={null}
Google Cloud authentication failed · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · if credentials are current, check GCP IAM permissions and Vertex AI model access · API Error: 403 ...
```

**Was zu tun ist:**

* Wenn der Hinweis sagt, dass Anmeldedaten von dieser Umgebung verwaltet werden, besitzt die App, die Claude Code gestartet hat, die Anmeldedaten und die anderen Schritte hier gelten nicht: versuchen Sie es erneut, oder kontaktieren Sie Ihren Administrator
* Bestätigen Sie die Rollen in [IAM-Konfiguration](/docs/de/google-vertex-ai#iam-configuration), die der Identität gewährt sind, mit der Sie sich authentifizieren
* Bestätigen Sie, dass das Modell für Ihr Projekt aktiviert ist. Siehe [Modellzugriff anfordern](/docs/de/google-vertex-ai#2-request-model-access)

Vor v2.1.273 zeigte ein 403 von Agent Platform die generische `Please run /login`- oder `Failed to authenticate`-Meldung statt, die Google Cloud-Anmeldedaten nicht aktualisieren kann.

<h3 id="microsoft-foundry-authentication-failed">
  Microsoft Foundry-Authentifizierung fehlgeschlagen
</h3>

[Microsoft Foundry](/docs/de/microsoft-foundry) hat einen 401 oder 403 zurückgegeben: die Azure-Anmeldedaten auf der Anfrage wurden abgelehnt, oder die Identität dahinter hat keinen Zugriff auf die Foundry-Ressource. `/login` kann keine Azure-Anmeldedaten prägen. Der Aktionshinweis in der Mitte variiert mit Ihrem Setup. Der stabile Teil ist der führende `Microsoft Foundry authentication failed`:

```text theme={null}
Microsoft Foundry authentication failed · refresh your Foundry credential (ANTHROPIC_FOUNDRY_AUTH_TOKEN, ANTHROPIC_FOUNDRY_API_KEY, Azure sign-in for Entra, or your proxy token) and retry · if credentials are current, check access to the Foundry resource · API Error: 401 ...
```

**Was zu tun ist:**

* Wenn der Hinweis sagt, dass Anmeldedaten von dieser Umgebung verwaltet werden, besitzt die App, die Claude Code gestartet hat, die Anmeldedaten und die anderen Schritte hier gelten nicht: versuchen Sie es erneut, oder kontaktieren Sie Ihren Administrator
* Aktualisieren Sie die Anmeldedaten, die Sie in [Azure-Anmeldedaten konfigurieren](/docs/de/microsoft-foundry#2-configure-azure-credentials) konfiguriert haben: rotieren Sie `ANTHROPIC_FOUNDRY_API_KEY`, prägen Sie ein frisches `ANTHROPIC_FOUNDRY_AUTH_TOKEN`, oder führen Sie `az login` aus, damit die Standard-Microsoft Entra-Anmeldedaten-Chain sich erneut anmelden kann
* Wenn die Anmeldedaten aktuell sind, bestätigen Sie, dass die Identität Zugriff auf die Foundry-Ressource hat. Siehe [Azure RBAC-Konfiguration](/docs/de/microsoft-foundry#azure-rbac-configuration)

Vor v2.1.273 zeigte ein 401 oder 403 von Microsoft Foundry die generische `Please run /login`- oder `Failed to authenticate`-Meldung statt, die Azure-Anmeldedaten nicht aktualisieren kann.

<h3 id="could-not-load-aws-or-google-cloud-credentials">
  AWS- oder Google Cloud-Anmeldedaten konnten nicht geladen werden
</h3>

Claude Code konnte keine verwendbaren Anmeldedaten aus der AWS-Anmeldedaten-Provider-Chain oder aus Ihren Google-Anwendungsstandardanmeldedaten auf dem Computer, auf dem es ausgeführt wird, erhalten, sodass keine Anfrage Ihren Cloud-Provider erreichte. Claude Code löscht seine zwischengespeicherten Anmeldedaten und versucht zweimal erneut, bevor diese Meldung angezeigt wird. Das Detail nach dem `·` benennt die spezifische Ursache, z. B. eine abgelaufene SSO-Sitzung, fehlende Anwendungsstandardanmeldedaten, die als `Could not load the default credentials` gemeldet werden, oder eine widerrufene Anmeldung, die als `invalid_grant` gemeldet wird:

```text theme={null}
API Error: Could not load AWS credentials · Could not load credentials from any providers. Check or refresh your AWS credentials and try again.
API Error: Could not load Google Cloud credentials · invalid_grant. Check or refresh your Google Cloud credentials and try again.
```

Im [nicht-interaktiven Modus](/docs/de/headless) mit `-p` und im [Agent SDK](/docs/de/agent-sdk/overview) ist der strukturierte Fehlercode `cloud_credential_error`. Vor v2.1.267 zeigte die Meldung nur den Detail-Text nach `API Error:`, und der strukturierte Code war `server_error` oder `unknown`.

**Was zu tun ist:**

* Führen Sie den Anmeldungsbefehl Ihres Providers aus, z. B. `aws sso login --profile myprofile` oder `gcloud auth application-default login`, dann versuchen Sie es erneut. [Bedrock, Agent Platform oder Foundry-Anmeldedaten werden nicht geladen](/docs/de/troubleshoot-install#bedrock-agent-platform-or-foundry-credentials-not-loading) zeigt, wie Sie die Anmeldedaten außerhalb von Claude Code bestätigen
* Wenn das Detail `AWS default-chain credential resolve timed out` liest, hat die Chain gehangen statt fehlgeschlagen, also folgen Sie stattdessen [AWS-Standard-Chain-Anmeldedaten-Auflösung hat das Zeitlimit überschritten](#aws-default-chain-credential-resolve-timed-out)

<h3 id="aws-default-chain-credential-resolve-timed-out">
  AWS-Standard-Chain-Anmeldedaten-Auflösung hat das Zeitlimit überschritten
</h3>

Die AWS-Standard-Anmeldedaten-Provider-Chain hat Anmeldedaten nicht innerhalb von 60 Sekunden produziert, sodass Claude Code die Auflösung gestoppt und die Anfrage fehlgeschlagen hat. Dieses Zeitlimit ist eine Ursache von [AWS- oder Google Cloud-Anmeldedaten konnten nicht geladen werden](#could-not-load-aws-or-google-cloud-credentials). Der Fehler ist lokale Anmeldedaten-Auflösung: die Anfrage erreichte nie [Amazon Bedrock](/docs/de/amazon-bedrock), [Claude Platform on AWS](/docs/de/claude-platform-on-aws) oder den [Mantle-Endpunkt](/docs/de/amazon-bedrock#use-the-mantle-endpoint). Claude Code löscht seinen [Anmeldedaten-Cache](/docs/de/amazon-bedrock#credential-caching-and-resolution-timeout) und versucht es erneut, bevor dieser Fehler auftritt, sodass die Chain bei wiederholten Versuchen steckengeblieben ist, wenn Sie ihn sehen.

```text theme={null}
API Error: Could not load AWS credentials · AWS default-chain credential resolve timed out. Check or refresh your AWS credentials and try again.
```

Häufige Ursachen sind ein `credential_process`-Befehl in Ihrem AWS-Profil, der auf Eingabe wartet, die er nicht erhalten kann, und ein Container oder eine VM, deren Instance-Metadaten-Dienst (IMDS) nie auf die Probe der Chain antwortet.

Vor v2.1.267 lautete die Meldung `API Error: AWS default-chain credential resolve timed out`.
Vor v2.1.207 ließ eine steckengebliebene Chain die Anfrage auf unbestimmte Zeit warten, statt mit dieser Meldung fehlzuschlagen.

**Was zu tun ist:**

* Führen Sie `aws sts get-caller-identity` in der gleichen Shell mit dem gleichen `AWS_PROFILE` aus. Wenn es auch hängt, beheben Sie das Profil; ein `credential_process`-Befehl, der interaktiv auffordert, ist eine häufige Ursache.
* Schließen Sie den Anmeldeschritt ab, bevor Sie Claude Code starten, z. B. `aws sso login --profile myprofile`, sodass die Chain aus dem lokalen SSO-Cache aufgelöst wird, statt auf einen Browser-Flow zu warten
* Wenn Ihre Chain eine interaktive Anmeldung ausführt, die legitim mehr als 60 Sekunden benötigt, z. B. SSO mit MFA durch einen Wrapper wie `aws-vault`, erhöhen Sie das Limit in Millisekunden mit [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/de/env-vars)

<h3 id="bedrock-setup-verification-timed-out-waiting-for-aws">
  Bedrock-Setup-Verifizierung hat das Zeitlimit überschritten, während auf AWS gewartet wurde
</h3>

Ein Aufruf an AWS während des [Bedrock-Setup-Assistenten](/docs/de/amazon-bedrock#sign-in-with-bedrock)'s Anmeldedaten-Verifizierung, z. B. die Anmeldedaten-Suche oder die Identitätsprüfung, hat nicht innerhalb der 60-Sekunden-Grenze beendet. Der Assistent stoppt das Warten und schlägt den Verifizierungsschritt fehl:

```text theme={null}
Timed out after 60s waiting for AWS. Check your network and proxy settings; if a credential helper needs longer to prompt you, raise CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS.
```

Die Zahl spiegelt Ihr Limit wider: 60 Sekunden standardmäßig, oder der Wert, den Sie in [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/de/env-vars) setzen.

Häufige Ursachen sind ein Netzwerk oder Proxy, das Anfragen an AWS staut, einschließlich der SSO-Token-Aktualisierung, und ein Anmeldedaten-Helper, der immer noch auf Eingabe wartet, die Sie nicht sehen können. Erhöhen Sie das Limit nur, wenn der Helper legitim mehr Zeit benötigt.

Eine einzelne steckengebliebene Anfrage an AWS kann auch auf ihrem eigenen Pro-Anfrage-Zeitlimit fehlschlagen, das eine kürzere Meldung auf dem gleichen Schritt zeigt:

```text theme={null}
A request to AWS timed out. Check your network and proxy settings, then try again.
```

Wenn die gleichen Zeitüberschreitungen auf dem Modell-Pin-Schritt auftreten, markiert der Assistent ein Modell als `unreachable` statt eine der beiden Meldungen zu zeigen.

**Was zu tun ist:**

* Führen Sie `aws sts get-caller-identity` in der gleichen Shell aus. Wenn es auch hängt, ist die Stagnation außerhalb von Claude Code, in Ihrem Netzwerk, Ihrem Proxy oder dem Anmeldedaten-Helper in Ihrem AWS-Profil; beheben Sie das zuerst.
* Schließen Sie jede interaktive Anmeldung ab, bevor Sie den Assistenten öffnen, z. B. `aws sso login --profile myprofile`
* Wenn ein Anmeldedaten-Helper in Ihrem AWS-Profil legitim länger als 60 Sekunden benötigt, um Sie aufzufordern, erhöhen Sie das Limit in Millisekunden mit [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/de/env-vars)

<h3 id="cloud-gateway-session-expired">
  Cloud-Gateway-Sitzung abgelaufen
</h3>

Sie haben sich durch ein [Claude apps gateway](/docs/de/claude-apps-gateway) angemeldet, und die auf diesem Computer gespeicherte Gateway-Sitzung ist abgelaufen und konnte nicht erneuert werden, oder das Gateway akzeptiert sie nicht mehr, z. B. nachdem das Gateway's [JWT-Geheimnis rotiert wurde](/docs/de/claude-apps-gateway-deploy#jwt-secret-rotation). Wenn Sie diese Zeile sehen, wenn Sie `claude` interaktiv starten, hat die Sitzung ohne Gateway-Anmeldung geöffnet:

```text theme={null}
Cloud gateway session expired — run /login to reconnect.
```

Die gleiche Zeile kann während der Sitzung erscheinen, wenn die Gateway-Anmeldedaten ablaufen und Claude Code sie nicht erneuern kann.

In einem [nicht-interaktiven](/docs/de/headless) Lauf, einer Hintergrund- oder anderen unbeaufsichtigten Sitzung oder einem `claude`-Unterbefehl außer `claude auth` beendet Claude Code statt mit dieser Meldung:

```text theme={null}
Cloud gateway <url> no longer accepts this session. Start `claude` and sign in again with /login.
```

**Was zu tun ist:**

* Führen Sie `/login` in der Sitzung aus und schließen Sie die Browser-Anmeldung ab
* Für einen nicht-interaktiven Start starten Sie `claude` in der gleichen Umgebung, führen Sie `/login` aus, dann führen Sie Ihren Befehl erneut aus

<h3 id="sign-in-timed-out-while-waiting-for-you-to-continue">
  Anmeldung hat das Zeitlimit überschritten, während auf Ihre Bestätigung gewartet wurde
</h3>

Während einer [Claude apps gateway](/docs/de/claude-apps-gateway)-Anmeldung benannte das Gateway das Konto, das sich angemeldet hat, und Claude Code forderte Sie auf, es zu bestätigen, bevor die Anmeldedaten gespeichert wurden. Sie ließen die Bestätigung offen, nachdem die Anmeldung selbst abgelaufen war, und das Gateway gab kein Aktualisierungs-Token aus, das es erneuern konnte, sodass Claude Code nichts speicherte, als Sie fortfuhren:

```text theme={null}
Sign-in timed out while waiting for you to continue. Try again.
```

**Was zu tun ist:**

* Führen Sie `/login` erneut aus und bestätigen Sie das Konto, bevor die Anmeldung abläuft

<h3 id="gateway-refused-the-request">
  Gateway lehnte die Anfrage ab
</h3>

Sie sind durch ein [Claude apps gateway](/docs/de/claude-apps-gateway) angemeldet, und eine Anfrage hat einen 403 zurückgegeben: das Gateway oder das Upstream dahinter hat sie abgelehnt. Die erneute Anmeldung ändert eine Ablehnung nicht, sodass die Meldung auf Ihren Gateway-Administrator zeigt:

```text theme={null}
Gateway refused the request · signing in again won't change this — check with your gateway administrator · API Error: 403 ...
```

**Was zu tun ist:**

* Bitten Sie Ihren Gateway-Administrator, die Anfrage nachzuschlagen. Der `API Error:`-Schwanz trägt die Ablehnung, die das Gateway zurückgegeben hat
* Für Administratoren: eine [Zugriffskontrollregel](/docs/de/claude-apps-gateway-config#http-tuning) auf dem Gateway gibt einen 403 zurück, den das [Audit-Protokoll](/docs/de/claude-apps-gateway-deploy#logs) mit seinem Grund aufzeichnet, und eine Upstream-Autorisierungsverweigerung wird pro [Upstream-Fehlermeldungen](/docs/de/claude-apps-gateway-config#upstream-error-messages) weitergeleitet

Vor v2.1.273 zeigte ein 403 auf einer Gateway-Sitzung die generische `Please run /login`- oder `Failed to authenticate`-Meldung statt, und die erneute Anmeldung löscht die Ablehnung nicht.

<h2 id="network-and-connection-errors">
  Netzwerk- und Verbindungsfehler
</h2>

Die meisten dieser Fehler bedeuten, dass eine Netzwerkanfrage von Claude Code ihr Ziel nicht erreicht hat oder etwas zwischen Claude Code und der API die Antwort auf dem Rückweg verändert hat. Wenn ein Eintrag auch eine lokale Ursache hat, wie z. B. einen fehlgeschlagenen Archivschreibvorgang, wird dies im Text angegeben. Sie entstehen normalerweise in Ihrem lokalen Netzwerk, Proxy oder Firewall oder in der Netzwerkrichtlinie der Cloud-Umgebung.

<h3 id="unable-to-connect-to-api">
  Unable to connect to API
</h3>

Die TCP-Verbindung zur API ist fehlgeschlagen oder wurde nie abgeschlossen. Bei den häufigen Verbindungsfehlercodes benennt die Nachricht die Art des Fehlers und behält den Code in Klammern:

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

Ein Code, den Claude Code nicht erkennt, wird als `Unable to connect to API` angezeigt, gefolgt vom Code in Klammern. Einige dieser Meldungen können mehr als einen Code anzeigen: `Connection refused` kann `ConnectionRefused` oder `ECONNREFUSED` anzeigen, und `Can't reach the API server` kann `ENOTFOUND` oder `FailedToOpenSocket` anzeigen.

Vor v2.1.227 las sich jede dieser codierten Meldungen als `Unable to connect to API` gefolgt vom Code, z. B. `Unable to connect to API (ECONNREFUSED)`.

Häufige Ursachen sind fehlender Internetzugang, ein VPN, das `api.anthropic.com` blockiert, oder ein erforderlicher Unternehmens-Proxy, der nicht konfiguriert ist.

**Was zu tun ist:**

* Bestätigen Sie, dass Sie den API-Host aus derselben Shell erreichen können, indem Sie `curl -I https://api.anthropic.com` ausführen. Verwenden Sie unter Windows PowerShell `curl.exe -I https://api.anthropic.com`, damit der integrierte `Invoke-WebRequest`-Alias nicht verwendet wird.
* Wenn Sie sich hinter einem Unternehmens-Proxy befinden, setzen Sie `HTTPS_PROXY` vor dem Starten von Claude Code und siehe [Netzwerkkonfiguration](/docs/de/network-config)
* Wenn Sie über ein LLM-Gateway oder Relay weiterleiten, setzen Sie [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) auf dessen Adresse. Siehe [Claude Code mit einem LLM-Gateway verbinden](/docs/de/llm-gateway-connect) für die Einrichtung.
* Stellen Sie sicher, dass Ihre Firewall die in [Netzwerkzugriffsanforderungen](/docs/de/network-config#network-access-requirements) aufgelisteten Hosts zulässt
* Intermittierende Fehler werden [automatisch wiederholt](#automatic-retries); anhaltende Fehler deuten auf ein lokales Netzwerkproblem hin

Wenn `curl` erfolgreich ist, aber Claude Code immer noch fehlschlägt, liegt die Ursache normalerweise in etwas zwischen der Laufzeit und dem Netzwerk, nicht im Netzwerk selbst:

* Unter Linux und WSL überprüfen Sie `/etc/resolv.conf` auf einen unerreichbaren Nameserver. WSL kann insbesondere einen fehlerhaften Resolver vom Host erben.
* Unter macOS kann ein VPN-Client, der getrennt oder deinstalliert wurde, eine Tunnel-Schnittstelle oder Routing-Regel hinterlassen. Überprüfen Sie `ifconfig` auf veraltete `utun`-Schnittstellen und entfernen Sie die Netzwerkerweiterung des VPN in den Systemeinstellungen.
* Docker Desktop und ähnliche Container-Laufzeiten können ausgehenden Datenverkehr abfangen. Beenden Sie diese und versuchen Sie es erneut, um dies auszuschließen.

<h3 id="unable-to-connect-to-anthropic-services">
  Unable to connect to Anthropic services
</h3>

Während der Ersteinrichtung überprüft Claude Code, ob es `api.anthropic.com` und `platform.claude.com` erreichen kann, bevor der Anmeldeschritt angezeigt wird. Wenn eine der Überprüfungen fehlschlägt, druckt Claude Code den Grund aus und beendet sich.

```text theme={null}
Unable to connect to Anthropic services
Failed to connect to api.anthropic.com: ECONNREFUSED
Connection to api.anthropic.com timed out after 10 seconds
A proxy is configured via HTTPS_PROXY. Check that it allows connections to the host above.
```

Claude Code sendet die Überprüfung durch die gleiche [Proxy-Konfiguration](/docs/de/network-config) wie API-Anfragen und gibt jeder Sonde 10 Sekunden. Wenn die fehlgeschlagene Sonde durch einen Proxy ging, benennt die Nachricht die Umgebungsvariable, die ihn konfiguriert hat, wie z. B. `HTTPS_PROXY`. Vor v2.1.222 verwendete die Überprüfung einen anderen Proxy-Transport ohne Timeout: Hinter einer Proxy-URL mit dem `https://`-Schema könnte sie auf `Checking connectivity...` unbegrenzt steckenbleiben und dann fehlschlagen, obwohl API-Anfragen durch denselben Proxy erfolgreich sind.

Claude Code überspringt diese Überprüfung, wenn eine [verwaltete Einstellungsdatei, MDM-Richtlinie oder Richtlinien-Hilfsprogramm](/docs/de/managed-settings) [`forceLoginMethod`](/docs/de/settings-reference#forceloginmethod) auf `"gateway"` setzt oder [`forceLoginGatewayUrl`](/docs/de/settings-reference#forcelogingatewayurl) ohne `forceLoginMethod` setzt. Mit einer dieser Konfigurationen öffnet Claude Code den Anmeldeschritt auf dem **Cloud-Gateway**-Bildschirm statt einer Anthropic-Anmeldemethode. Claude Code überspringt die Überprüfung auch, wenn eine verwaltete Einstellungsquelle auf dem Computer vorhanden ist, aber nicht gelesen werden kann, da diese Quelle die Gateway-Konfiguration enthalten kann. Vor v2.1.247 führte Claude Code die Überprüfung auch unter dieser Konfiguration aus und beendete sich mit diesem Fehler, wenn die Anthropic-Endpunkte unerreichbar waren.

**Was zu tun ist:**

* Wenn die Nachricht eine Proxy-Variable benennt, überprüfen Sie, dass ihr Wert auf den richtigen Proxy verweist, und bitten Sie Ihr Netzwerk-Team, HTTPS-Verbindungen durch ihn zum Host in der Nachricht zuzulassen. Siehe [Netzwerkkonfiguration](/docs/de/network-config).
* Arbeiten Sie die Überprüfungen in [Unable to connect to API](#unable-to-connect-to-api) durch. Der `curl`-Test und die Firewall-Anleitung dort gelten auch für diese Überprüfung.
* Wenn sich Ihre Organisation über ein [Cloud-Gateway](/docs/de/claude-apps-gateway) anmeldet und dieser Fehler beim ersten Ausführen angezeigt wird, aktualisieren Sie auf Claude Code v2.1.247 oder später.
* Wenn Ihr Netzwerk offen ist und der Fehler weiterhin besteht, ist Claude Code möglicherweise nicht [in Ihrem Land verfügbar](https://www.anthropic.com/supported-countries)

<h3 id="socket-is-closed">
  Socket is closed
</h3>

`Socket is closed` bedeutet, dass die Verbindung, die eine Streaming-Antwort übertrug, geschlossen wurde, während die Antwort noch ankam. Die häufigste Ursache ist ein Unternehmens-Proxy unter Windows, der einen etablierten Tunnel mitten in der Antwort abbricht.

Je nachdem, wie weit die Antwort fortgeschritten war, wiederholt Claude Code die Anfrage, behält das, was Claude produziert hat, oder beendet den Zug. Siehe [Automatische Wiederholungen](#automatic-retries).

Vor v2.1.214 wiederholte Claude Code diesen Fehler nicht, und der Zug stoppte mit einem Fehler, der `Socket is closed` enthielt.

**Was zu tun ist:**

* Wenn Sie diesen Fehler sehen, aktualisieren Sie mit `claude update` auf v2.1.214 oder später und senden Sie Ihre Nachricht erneut
* Wenn Züge hinter demselben Proxy nach dem Update weiterhin fehlschlagen, arbeiten Sie [Unable to connect to API](#unable-to-connect-to-api) durch und überprüfen Sie die Proxy-Einrichtung in [Netzwerkkonfiguration](/docs/de/network-config)

<h3 id="api-returned-an-empty-or-malformed-response">
  API returned an empty or malformed response
</h3>

Claude Code zeigt diesen Fehler an, wenn sein Nicht-Streaming-Wiederholungsversuch einer fehlgeschlagenen Streaming-Anfrage einen HTTP-Erfolgsstatus erhält, aber der Text keine Claude-API-Nachricht ist: häufig eine HTML-Fehler- oder Anmeldungsseite, ein leerer Text oder JSON in einem anderen Format. Ein Proxy, Gateway oder eine Netzwerk-Anmeldungsseite, die an der Stelle der API antwortet, ist die übliche Quelle. Claude Code wiederholt die Anfrage nicht, und der Zug endet mit diesem Fehler.

```text theme={null}
API returned an empty or malformed response (HTTP 200) — check for a proxy or gateway intercepting the request.
```

Nach dieser Eröffnung meldet die Nachricht, was zurückkam und welche Anfrage fehlgeschlagen ist:

* Eine `Response:`-Klausel mit dem Inhaltstyp, der Art des Textes, wie z. B. `body is an HTML page` oder `empty body`, seiner Größe in Bytes und ob die Antwort eine Anthropic-Anfrage-ID trug. Wenn die Antwort einen erkennbaren Server benennt, wie z. B. `nginx` oder `cloudflare`, oder Zwischenkopfzeilen trägt, wie z. B. `cf-ray` oder `via`, listet die Klausel diese auch auf.
* Ein Satz, der die ID der fehlgeschlagenen Streaming-Anfrage und den Fehler benennt, der die Wiederholung ausgelöst hat. Wenn ein Stream geöffnet wurde, bevor der Fehler auftrat, meldet er auch, wie viele Stream-Ereignisse ankamen und, falls vorhanden, wie lange der Stream stumm war, als der Versuch fehlschlug.

Vor v2.1.234 endete die Nachricht nach `intercepting the request`.

Vor v2.1.271 endete auch eine Antwort, die eine gültige API-Nachricht unter einem nicht-JSON-Inhaltstyp wie `text/plain` trug, den Zug mit diesem Fehler. Einige LLM-Gateways verwenden diesen Inhaltstyp für die Nicht-Streaming-Antwort.

**Was zu tun ist:**

* Lesen Sie die `Response:`-Klausel, um zu sehen, welches System geantwortet hat. Ein HTML-Text, keine Anthropic-Anfrage-ID oder ein benannter Server wie `nginx` oder `cloudflare` bedeutet, dass etwas zwischen Claude Code und der API an seiner Stelle geantwortet hat
* Wenn Sie über ein [LLM-Gateway](/docs/de/llm-gateway-connect#troubleshoot-gateway-errors) weiterleiten, testen Sie die Route mit einer direkten Anfrage und beheben Sie den Hop, der die Nicht-API-Antwort zurückgibt
* Führen Sie in einem Netzwerk mit einer Anmeldungsseite, wie z. B. Gast-Wi-Fi, die Anmeldung in einem Browser durch und versuchen Sie es erneut
* Wenn nur die Nicht-Streaming-Route durch Ihr Gateway fehlerhaft ist, setzen Sie [`CLAUDE_CODE_DISABLE_NONSTREAMING_FALLBACK=1`](/docs/de/env-vars#variables), damit eine Anfrage, die mitten im Stream fehlschlägt, zum normalen Wiederholungspfad geht, anstatt zu diesem Fallback, außer wenn der Streaming-Endpunkt selbst `404` zurückgibt, wo Claude Code immer noch zurückfällt

<h3 id="streaming-response-ended-before-any-complete-data-was-received">
  Streaming response ended before any complete data was received
</h3>

Eine Streaming-Antwort von Ihrem Modell-Provider wurde abgeschlossen, ohne brauchbare Daten zu liefern, daher hat Claude Code die Anfrage ohne Streaming erneut gesendet, um den Zug zu beenden. Claude Code zeigt die Warnung einmal pro Sitzung, nur in interaktiven Sitzungen. Vor v2.1.239 wiederholte Claude Code stillschweigend ohne Streaming.

```text theme={null}
Streaming response ended before any complete data was received. Retrying without streaming. If this keeps happening, check any proxy or gateway between Claude Code and your model provider.
```

Claude Code sendet jede betroffene Anfrage zweimal: den leeren Streaming-Versuch und die Wiederholung. Die übliche Ursache ist ein Proxy oder Gateway, das den Streaming-Antwort-Text auf dem Rückweg verbraucht oder transformiert.

**Was zu tun ist:**

* Konfigurieren Sie jeden Proxy oder Gateway zwischen Claude Code und Ihrem Modell-Provider so, dass Streaming-Antwort-Texte und deren Header unverändert durchgeleitet werden
* Auf [Amazon Bedrock](/docs/de/amazon-bedrock) siehe [Streaming-Fehler hinter einem Gateway oder Proxy](/docs/de/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy) für die Header- und Text-Anforderungen

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock streaming response has an unexpected content-type
</h3>

Ein Gateway oder Proxy zwischen Claude Code und [Amazon Bedrock](/docs/de/amazon-bedrock) transformiert den Streaming-Antwort-Text oder seinen `Content-Type`-Header. Amazon Bedrock streamt Antworten als `application/vnd.amazon.eventstream`. Anstatt einen Text zu dekodieren, den es nicht lesen kann, lehnt Claude Code eine erfolgreiche Streaming-Antwort ab, die einen anderen Inhaltstyp meldet. Claude Code wiederholt die Anfrage nicht.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Vor v2.1.208 tauchte die gleiche Fehlkonfiguration als `API Error: Truncated event message received` auf, nachdem der gesamte Text gepuffert worden war.

**Was zu tun ist:**

* Konfigurieren Sie das Gateway so, dass der `InvokeModelWithResponseStream`-Antwort-Text und sein `Content-Type`-Header unverändert durchgeleitet werden. Ein Vermittler, der den Stream als Server-Sent-Events erneut aussendet, ist eine häufige Ursache.
* Das Setzen von [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/de/env-vars) verbirgt diesen Fehler, aber Claude Code dekodiert keinen binären Text unter einem umgeschriebenen Header, daher fallen diese Anfragen auf einen langsameren Nicht-Streaming-Pfad zurück. Siehe [Streaming-Fehler hinter einem Gateway oder Proxy](/docs/de/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  SSL certificate errors
</h3>

Ein Proxy oder Sicherheitsgerät in Ihrem Netzwerk fängt TLS-Datenverkehr mit seinem eigenen Zertifikat ab, und Claude Code vertraut ihm nicht.

```text theme={null}
Unable to connect to API: SSL certificate verification failed (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
Unable to connect to API: Self-signed certificate detected (SELF_SIGNED_CERT_IN_CHAIN). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
```

Vor v2.1.273 endeten beide Meldungen bei `Check your proxy or corporate SSL certificates`, ohne den OpenSSL-Code oder den `NODE_EXTRA_CA_CERTS`-Hinweis.

Ab v2.1.199 wird ein Zertifikatvalidierungsfehler nicht wiederholt, daher wird dieser Fehler beim ersten Versuch angezeigt, anstatt nach dem vollständigen [Wiederholungsbudget](#automatic-retries). Frühere Versionen verbrachten einige Minuten mit Wiederholungen, bevor sie ihn zeigten. Vorübergehende TLS-Bedingungen, wie z. B. ein Handshake-Timeout, werden immer noch wiederholt.

Während `/login` und der Startup-Konnektivitätsprüfung wird der gleiche Fehler mit einer anderen Nachricht gemeldet:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

Auf [Amazon Bedrock](/docs/de/amazon-bedrock) hängen die Anfragen, die Claude Code selbst an AWS sendet, wie z. B. die STS- und SSO-Rollenkredential-Aufrufe, Modellermittlung und die Überprüfungen des Setup-Assistenten, von der gleichen Zertifikatskonfiguration ab. Siehe [Zertifikatsfehler hinter einem TLS-inspizierenden Proxy](/docs/de/amazon-bedrock#certificate-errors-behind-a-tls-inspecting-proxy).

**Was zu tun ist:**

* Exportieren Sie das CA-Bundle Ihrer Organisation und verweisen Sie Claude Code mit `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem` darauf
* Siehe [Netzwerkkonfiguration](/docs/de/network-config#custom-ca-certificates) für vollständige Einrichtungsanweisungen
* Setzen Sie nicht `NODE_TLS_REJECT_UNAUTHORIZED=0`, was die Zertifikatvalidierung vollständig deaktiviert

<h3 id="host-not-allowed-in-a-cloud-session">
  Host not allowed in a cloud session
</h3>

Eine ausgehende HTTP-Anfrage aus einer Cloud-Sitzung oder Routine wurde durch die Netzwerkrichtlinie der Umgebung blockiert.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Sie können auch ein TLS-Zertifikat sehen, das nicht dem echten Zertifikat des Ziels entspricht. Cloud-Sitzungen leiten ausgehenden Datenverkehr durch einen Proxy weiter, der die Netzwerkrichtlinie durchsetzt, daher bedeutet ein nicht übereinstimmendes Zertifikat, dass der Proxy die Verbindung beendet hat, nicht das Ziel.

Dies ist kein clientseitiges Netzwerkproblem. Cloud-Sitzungen und [Routinen](/docs/de/routines) laufen in einer sandboxierten VM, deren ausgehender Datenverkehr durch das Netzwerk der Sitzung auf die [Zulassungsliste der Cloud-Umgebung](/docs/de/cloud-environments) gefiltert wird; [GitHub-Operationen](/docs/de/cloud-environments#github-proxy) und MCP-Connector-Datenverkehr verwenden separate Kanäle, weshalb sie weiterhin funktionieren können, während andere Hosts blockiert sind. Die **Standard**-Umgebung verwendet **Vertrauenswürdigen** Zugriff, der die [Standard-Zulassungsliste](/docs/de/cloud-environments#default-allowed-domains) von Paket-Registries, Cloud-Provider-APIs, Container-Registries und häufigen Entwicklungsdomänen zulässt und andere Domänen auf diesem Pfad blockiert.

**Was zu tun ist:**

Diese Schritte ändern eine Ihrer eigenen Umgebungen. Eine [organisationsweit gemeinsame Umgebung](/docs/de/cloud-environments#organization-shared-environments) wird im Selector schreibgeschützt geöffnet, daher bitten Sie einen Besitzer, ihren Netzwerkzugriff von der Seite **Cloud-Umgebungen** in den [Admin-Einstellungen](https://claude.ai/admin-settings) zu ändern.

* Öffnen Sie die Routine zum Bearbeiten oder starten Sie eine Cloud-Sitzung. Wählen Sie das Cloud-Symbol, das den Namen Ihrer Umgebung anzeigt, wie z. B. **Standard**, um den Selector zu öffnen. Bewegen Sie den Mauszeiger über Ihre Umgebung und klicken Sie auf das Einstellungssymbol.
* Im Dialog **Cloud-Umgebung aktualisieren** ändern Sie **Netzwerkzugriff** von **Vertrauenswürdig** zu **Benutzerdefiniert** und fügen dann die blockierte Domäne zu **Zulässige Domänen** hinzu. Geben Sie eine Domäne pro Zeile ein. Aktivieren Sie **Auch Standard-Liste häufiger Paketmanager einschließen**, um die [Standard-Zulassungsliste](/docs/de/cloud-environments#default-allowed-domains) neben Ihren benutzerdefinierten Domänen zu behalten. Wählen Sie stattdessen **Vollständig**, wenn Sie uneingeschränkten Zugriff möchten.
* Klicken Sie auf **Änderungen speichern**. Die nächste Ausführung verwendet die aktualisierte Zulassungsliste.

Siehe [Netzwerkzugriff](/docs/de/cloud-environments#network-access) für Zugriffsstufen und die Standard-Zulassungsliste. Lokale CLI-Sitzungen sind nicht von dieser Richtlinie betroffen.

<h3 id="the-proxy-refused-the-connection">
  The proxy refused the connection
</h3>

Sie sehen diese Nachricht, wenn Claude ein [Artefakt](/docs/de/artifacts) durch den Proxy liest, den Sie in `HTTPS_PROXY` oder einer verwandten [Proxy-Variable](/docs/de/network-config#environment-variables) setzen. Artefakt-Inhalte stammen von `*.frame.claudeusercontent.com`, daher sendet Claude Code zuerst dem Proxy eine `CONNECT`-Anfrage, um ihn zu bitten, einen Tunnel zu diesem Host zu öffnen. Wenn der Proxy sich weigert, erreicht nichts den Host, und die Nachricht trägt den HTTP-Status des Proxys:

```text theme={null}
artifact content fetch failed (proxy refused the connection: HTTP 407)
artifact content fetch failed (proxy refused the connection: HTTP 403)
the proxy refused the connection to the artifact's content host (HTTP 502)
```

Der Status ist die Antwort des Proxys auf die `CONNECT`. Der Host hat nie geantwortet, daher weist jeder Status auf eine andere Behebung hin:

* `HTTP 407`: Der Proxy benötigt Anmeldedaten, die er nicht erhalten hat. Geben Sie diese in die Proxy-URL ein, wie [Basic-Authentifizierung](/docs/de/network-config#basic-authentication) zeigt.
* `HTTP 403`: Der Proxy weigert sich, zu `*.frame.claudeusercontent.com` zu tunneln. Bitten Sie denjenigen, der den Proxy betreibt, diesen Host zuzulassen, den [Netzwerkzugriffsanforderungen](/docs/de/network-config#network-access-requirements) auflistet.
* Jeder andere Status, wie z. B. `HTTP 502`: Der Proxy hat den Tunnel aus eigenem Grund nicht geöffnet, wie z. B. Fehler beim Erreichen des Hosts. Schlagen Sie den Status in den Protokollen des Proxys nach.
* `unreadable reply` anstelle eines Status: Was sich unter der Proxy-Adresse befindet, hat nicht mit einer HTTP-Statuszeile geantwortet. Überprüfen Sie, dass die Adresse ein HTTP-Proxy ist.

**Was zu tun ist:**

* Überprüfen Sie die Adresse und Anmeldedaten in der Proxy-Variable, wie [Proxy-Konfiguration](/docs/de/network-config#proxy-configuration) beschreibt, und führen Sie dann `curl -x http://proxy.example.com:8080 -I https://api.anthropic.com` aus der Shell aus, in der Sie Claude Code starten, mit Ihrer eigenen Proxy-URL. Unter Windows PowerShell führen Sie `curl.exe` aus. Wenn diese Sonde auf die gleiche Weise fehlschlägt, beheben Sie zuerst die Proxy-Einrichtung. Wenn sie erfolgreich ist, ist die Weigerung spezifisch für den Artefakt-Host.
* Wenn Ihr Netzwerk Claude Code den direkten Zugriff auf den Artefakt-Host ermöglicht, fügen Sie `.frame.claudeusercontent.com` zu [`NO_PROXY`](/docs/de/network-config#environment-variables) hinzu. Halten Sie den Eintrag eng: Ein breiterer `.claudeusercontent.com`-Eintrag umgeht auch den Proxy für `bridge.claudeusercontent.com`, das Organisationen mit [IP-Zulassungslisten](/docs/de/network-config#organization-ip-allowlists-and-proxy-egress) auf dem Proxy behalten müssen.

Vor v2.1.238 meldete Claude Code einen abgelehnten Tunnel als generischen Netzwerkfehler.

<h3 id="the-cloud-environments-service-returned-an-empty-or-unexpected-response">
  The cloud environments service returned an empty or unexpected response
</h3>

Claude Code fordert Ihre [Cloud-Umgebungen](/docs/de/cloud-environments)-Liste an mehreren Stellen an, z. B. wenn Sie eine Cloud-Sitzung aus der CLI erstellen oder [`/remote-env`](/docs/de/cloud-environments#select-an-environment-from-the-cli) ausführen. Wenn es die Antwort des Servers nicht lesen kann, zeigt es eine dieser Meldungen:

```text theme={null}
The cloud environments service returned an empty response (HTTP 200 with no body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 with a non-JSON body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 without a usable environments list). This is usually temporary — try again in a moment.
```

Der Server akzeptierte die Anfrage, antwortete aber mit einem Text, der nicht die Umgebungsliste ist: leer, nicht JSON oder JSON ohne die Liste. Dies begleitet normalerweise eine Störung auf der Serverseite und klärt sich von selbst. Je nachdem, welche Oberfläche die Liste anforderte, kann Claude Code ein Präfix hinzufügen, wie z. B. `couldn't list environments:` im `/remote-env`-Dialog.

**Was zu tun ist:**

* Wiederholen Sie die Aktion. Claude Code fordert die Liste jedes Mal erneut an
* Wenn die Nachricht weiterhin angezeigt wird, überprüfen Sie [status.claude.com](https://status.claude.com) auf aktive Vorfälle

Vor v2.1.236 zeigte Claude Code stattdessen einen rohen JavaScript-TypeError.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Couldn't reconnect to your Remote Control session
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

Das Fortsetzen mit `claude --resume` oder `claude --continue` stellt die Verbindung zur [Remote Control](/docs/de/remote-control)-Sitzung wieder her, die in dieser Konversation aufgezeichnet wurde. Diese Nachricht bedeutet, dass die Wiederverbindung aus einem Grund fehlgeschlagen ist, der vorübergehend sein kann, wie z. B. eine Netzwerkunterbrechung oder ein Serverfehler, daher kann Claude Code nicht bestätigen, ob die Remote-Sitzung noch vorhanden ist. Ihre lokale Sitzung läuft ohne Remote Control weiter.

**Was zu tun ist:**

* Führen Sie `/remote-control` aus, um die Verbindung erneut zu versuchen
* Starten Sie eine neue Sitzung mit `claude --remote-control`, um eine neue Remote Control-Sitzung zu erstellen
* Für andere Remote Control-Startup-Meldungen siehe [Remote Control beheben](/docs/de/remote-control#troubleshooting)

Wenn der Server stattdessen meldet, dass die vorherige Sitzung weg ist, sehen Sie diese Nachricht nicht. Claude Code startet eine neue Sitzung an ihrer Stelle oder zeigt [`Previous session is unavailable — run /remote-control to start a new one`](/docs/de/remote-control#previous-session-is-unavailable), je nach [dem Wiederverbindungsdatensatz der Konversation](/docs/de/remote-control#resume-outcomes). Von v2.1.227 bis v2.1.231 zeigte Claude Code stattdessen eine Nachricht, die mit `Remote Control could not resume the previous session under the current login` beginnt, und [frühere Versionen verhielten sich wieder anders](/docs/de/remote-control#reconnect-history).

<h3 id="sessions-ended-while-this-machine-was-offline">
  Sessions ended while this machine was offline
</h3>

Claude Code zeigt diese Nachricht im Terminal, das [`claude remote-control`](/docs/de/remote-control#start-a-remote-control-session) ausführt, nachdem Ihr Computer lange genug offline war, dass der Server die Remote Control-Umgebung bereinigt hat, die Ihr Computer bediente. Die Sitzungen in dieser Umgebung endeten, und Sie können sie nicht fortsetzen. Die Anzahl ist die Anzahl der Sitzungen, die endeten.

```text theme={null}
2 sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.
```

**Was zu tun ist:**

* Wenn Claude Code beibehaltene Worktrees unter dieser Nachricht auflistet, holen Sie sich alle nicht committeten Arbeiten von ihnen
* Führen Sie `claude remote-control` aus, um eine frische Umgebung zu starten

<h3 id="couldnt-share-the-transcript">
  Couldn't share the transcript
</h3>

Nachdem Sie sich einigen, Ihr Sitzungs-Transkript aus einer Umfrage-Aufforderung zu teilen, wie z. B. der [Sitzungsqualitäts-Umfrage](/docs/de/data-usage#session-quality-surveys), lädt Claude Code es zu Anthropic hoch oder speichert stattdessen ein lokales Archiv bei Drittanbietern, bei [Claude-Apps-Gateway](/docs/de/claude-apps-gateway)-Sitzungen und wenn keine Anthropic-Anmeldedaten verfügbar sind. Diese Nachricht bedeutet, dass die Freigabe nicht abgeschlossen wurde.

```text theme={null}
Couldn't share the transcript.
```

Der Upload muss in ein 8-MiB-Limit passen. Bei einer langen Sitzung löscht Claude Code progressiv Teile der Freigabe, zuerst die Modelleinstellungen der letzten Anfrage, dann die strukturierte Konversation und Subagent-Transkripte, und zeigt diese Nachricht nur an, wenn keine reduzierte Version gesendet werden kann oder ein Netzwerk- oder Serverfehler den Upload stoppt. Wenn Claude Code stattdessen ein lokales Archiv speichert, bedeutet die Nachricht, dass es das Archiv nicht schreiben konnte.

**Was zu tun ist:**

* Führen Sie `/feedback` aus, um das Transkript mit einer Beschreibung dessen zu senden, was passiert ist. Siehe [Fehler melden](#report-an-error), wenn `/feedback` in Ihrer Umgebung nicht verfügbar ist
* Wenn auch andere Anfragen fehlschlagen, überprüfen Sie Ihre Netzwerkverbindung und siehe [Unable to connect to API](#unable-to-connect-to-api)

<h2 id="request-errors">
  Anfragefehler
</h2>

Diese Fehler beziehen sich auf den Inhalt Ihrer Anfrage. Die meisten werden von der API zurückgegeben, nachdem sie die Anfrage abgelehnt hat; einige werden lokal von Claude Code erzeugt, bevor eine Anfrage gesendet wird.

<h3 id="prompt-is-too-long">
  Eingabeaufforderung ist zu lang
</h3>

Das Gespräch plus angehängte Dateien überschreitet das Kontextfenster des Modells.

```text theme={null}
Prompt is too long
```

In einer interaktiven Sitzung zeigt Claude Code diesen Fehler als:

```text theme={null}
Context limit reached · /compact or /clear to continue
```

Die Zeile nennt nur `/clear`, wenn [`DISABLE_COMPACT`](/docs/de/env-vars) gesetzt ist. Längere Formen des Fehlers, wie die unten aufgeführte Komprimierungsfehlform, behalten die Formulierung `Prompt is too long ·` bei. In der `-p`-Ausgabe und dem Transkript bleibt der Text `Prompt is too long`.

Wenn Sie die automatische Komprimierung in Ihren [Benutzereinstellungen](/docs/de/settings-reference#autocompactenabled) deaktiviert haben, sagt die Zeile auch:

```text theme={null}
Context limit reached · /compact or /clear to continue · auto-compact is off · /config to turn it on
```

Der Schalter **Auto-compact** in `/config` schreibt `autoCompactEnabled` in die Benutzereinstellungen. Der Hinweis wird nur angezeigt, wenn eine `/config`-Änderung wirksam wird. Beispielsweise wird er nicht angezeigt, wenn [`DISABLE_AUTO_COMPACT`](/docs/de/env-vars) oder [`DISABLE_COMPACT`](/docs/de/env-vars) die automatische Komprimierung deaktiviert haben. Er wird auch nicht angezeigt, wenn ein Bereich mit höherer Priorität, wie Projekt- oder verwaltete Einstellungen, `autoCompactEnabled` auf `false` setzt. Vor v2.1.235 enthielt die Zeile keinen Hinweis zur automatischen Komprimierung.

Amazon Bedrock meldet diese Bedingung als `Input is too long for requested model.`, was Claude Code auf die gleiche Weise behandelt. Vor v2.1.217 erkannte Claude Code die Bedrock-Formulierung nicht, daher wurde die automatische Komprimierung nie ausgelöst und `/compact` schlug mit demselben Fehler fehl.

Ein [Claude-Apps-Gateway](/docs/de/claude-apps-gateway-config#upstream-error-messages) meldet diese Bedingung als `capability_rejected: prompt_too_long`, wenn ein Cloud-Upstream die Anfrage in der eigenen Fehlerform des Anbieters ablehnt. Claude Code behandelt das Token genauso wie `Prompt is too long`. Vor v2.1.228 erkannte Claude Code das Token nicht, daher wurde die automatische Komprimierung nicht ausgelöst.

Wenn die automatische Komprimierung in dieser Runde ausgeführt wurde und bei einem zugrunde liegenden Fehler fehlgeschlagen ist, wie z. B. ein nicht verfügbares Modell oder ein Authentifizierungsfehler, wird dieser Fehler nach einem Trennzeichen benannt:

```text theme={null}
Prompt is too long · automatic compaction failed: <the underlying error>
```

Beheben Sie zunächst den benannten Fehler; `/compact` schlägt mit demselben Fehler fehl, bis Sie dies tun. Vor v2.1.229 zeigte eine fehlgeschlagene automatische Komprimierung `Prompt is too long` ohne die Ursache an.

Wenn die automatische Komprimierung bei diesem Fehler ausgeführt wird, fasst sie normalerweise Ihre ältesten Austausche zusammen und behält die neuesten. Als letzter Ausweg fasst Claude Code anders zusammen:

* Wenn es keinen ganzen Austausch zusammenfassen kann, behält Claude Code Ihre neueste Eingabeaufforderung wörtlich und fasst alles davor zusammen.
* In diesem Fall, wenn das Gespräch nicht mit Ihrer Eingabeaufforderung endet, fasst Claude Code stattdessen das gesamte Gespräch zusammen.

Claude Code überspringt diese Wiederherstellung, wenn der Inhalt, den es weitergeben würde, keine Modellantworte enthält und weniger als etwa 1.000 Token Ihres eigenen Textes, wie z. B. eine kurze Wiederholung nach einem übergroßen Einfügen. Führen Sie `/clear` aus, um neu zu beginnen. Vor v2.1.269 schlug die Komprimierung fehl, wenn sie keinen ganzen Austausch zusammenfassen konnte, daher traf eine Sitzung in diesem Zustand bei jedem Durchgang diesen Fehler erneut.

Ein Gespräch mit nur einem Austausch hat keine früheren Runden zum Zusammenfassen. Wenn die automatische Komprimierung auf einem ausgeführt worden wäre, überspringt Claude Code den Versuch und erklärt stattdessen, was die Anfrage ausfüllt. Wenn die API keine Token-Zählungen in ihrem Fehler meldet, lautet die Meldung:

```text theme={null}
Prompt is too long · this conversation is a single exchange and cannot be compacted — the request size comes mostly from system prompt, tool definitions, or attachments.
```

Wenn die API Token-Zählungen in ihrem Fehler meldet, vergleicht Claude Code diese mit seiner eigenen Schätzung der Gesprächsgröße, um zu bestimmen, welcher Teil die meiste Anfrage ausmacht: der Inhalt des Gesprächs selbst oder die Systemaufforderung, Werkzeugdefinitionen und Anhang-Inhalte, die Claude Code damit sendet. Wenn der Inhalt des Gesprächs selbst die meiste Anfrage ausmacht, lautet die Meldung:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) and this conversation's own content is most of it. A single-exchange conversation cannot be compacted; start with less content (smaller files or pasted text).
```

Wenn die meiste Anfrage außerhalb des Gesprächs liegt, lautet die Meldung:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) but this conversation is only ~<conversation tokens> tokens — the rest is system prompt, tool definitions, and attachment content. A single-exchange conversation cannot be compacted; reduce attached files/tools or start with less context.
```

Vor v2.1.162 versuchte Claude Code die Komprimierung trotzdem und zeigte das bloße `Prompt is too long` an, wenn es fehlschlug.

**Was zu tun ist:**

* Führen Sie `/compact` aus, um frühere Runden zusammenzufassen und Platz freizugeben, oder `/clear`, um neu zu beginnen. Wenn `/compact` mit `Not enough messages to compact.` antwortet, ist das Gespräch ein einzelner Austausch ohne frühere Inhalte zum Zusammenfassen, daher wird der Platz von dieser einen Eingabeaufforderung und dem, was Claude Code mit jeder Anfrage sendet, verbraucht: Führen Sie `/clear` aus und senden Sie erneut mit weniger eingefügtem Text oder kleineren Anhängen, oder reduzieren Sie die Werkzeugdefinitionen und Speicherdateien mit den folgenden Schritten
* Führen Sie `/context` aus, um eine Aufschlüsselung zu sehen, was das Fenster verbraucht: Systemaufforderung, Werkzeuge, Speicherdateien und Nachrichten
* Deaktivieren Sie MCP-Server, die Sie nicht verwenden, mit `/mcp disable <name>`, um ihre Werkzeugdefinitionen aus dem Kontext zu entfernen
* Kürzen Sie große `CLAUDE.md`-Speicherdateien, oder verschieben Sie Anweisungen in [pfadgebundene Regeln](/docs/de/memory#path-specific-rules), die nur bei Bedarf geladen werden
* Subagenten erben jede MCP-Werkzeugdefinition von der übergeordneten Sitzung, was ihr Kontextfenster ausfüllen kann, bevor die erste Runde beginnt. Deaktivieren Sie MCP-Server, die Sie nicht verwenden, bevor Sie Subagenten spawnen.
* Die automatische Komprimierung ist standardmäßig aktiviert und verhindert normalerweise diesen Fehler. Wenn Sie sie in `/config` oder mit [`DISABLE_AUTO_COMPACT`](/docs/de/env-vars) deaktiviert haben, schalten Sie sie wieder ein. Wenn Sie sie deaktiviert lassen, führen Sie `/compact` selbst aus, bevor das Fenster voll wird.

Siehe [Erkunden Sie das Kontextfenster](/docs/de/context-window) für eine interaktive Ansicht, wie sich der Kontext ausfüllt.

<h3 id="context-exceeds-the-token-limit">
  Kontext überschreitet das Token-Limit
</h3>

`/context` zeigt diese Warnung oben in seiner Ausgabe an, wenn das Gespräch das Kontextfenster des Modells überschritten hat. Anfragen schlagen mit [`Prompt is too long`](#prompt-is-too-long) fehl, bis Sie Platz freigeben. Eine interaktive Sitzung zeigt diesen Fehler als die Zeile `Context limit reached` an.

```text theme={null}
Context exceeds the 200k-token limit by 94k tokens — run /compact or /clear to continue.
```

Wenn das Limit, das Sie überschritten haben, ein Komprimierungsfenster ist, das kleiner als das Kontextfenster des Modells ist, wie z. B. die 200K-Grenze bei 1M-Kontext-Modellen, lautet die Warnung anders. Ein Komprimierungsfenster kann unter dem Kontextfenster des Modells liegen, daher können Anfragen über ihm noch erfolgreich sein.

```text theme={null}
Context is 94k tokens past the 200k-token compaction window — run /compact to reduce usage.
```

Beide Formen nennen `/clear` statt `/compact`, wenn Sie [`DISABLE_COMPACT`](/docs/de/env-vars) gesetzt haben.

**Was zu tun ist:**

* Führen Sie in einem Gespräch mit mehreren Runden `/compact` aus, um frühere Runden zusammenzufassen und Platz freizugeben. Um stattdessen neu zu beginnen, führen Sie `/clear` aus
* Weitere Möglichkeiten zur Reduzierung der Nutzung finden Sie unter [Prompt is too long](#prompt-is-too-long)

Vor v2.1.216 zeigte `/context` die Nutzung über 100% ohne Warnzeile an, die erklärte, was das bedeutet oder wie man sich erholt.

<h3 id="error-during-compaction-conversation-too-long">
  Fehler während der Komprimierung: Gespräch zu lang
</h3>

`/compact` selbst ist fehlgeschlagen, weil nicht genug freier Kontext vorhanden ist, um die Zusammenfassung zu halten, die es erzeugt.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Dies kann passieren, wenn das Fenster bereits voll ist, wenn die automatische Komprimierung ausgelöst wird, oder wenn Sie `/compact` ausführen, nachdem Sie [`Prompt is too long`](#prompt-is-too-long) gesehen haben. In einer interaktiven Sitzung ist dieser Fehler die Zeile `Context limit reached`.

**Was zu tun ist:**

* Drücken Sie zweimal Esc, um die Nachrichtenliste zu öffnen und mehrere Runden zurückzugehen. Dies entfernt die neuesten Nachrichten aus dem Kontext. Führen Sie dann `/compact` erneut aus.
* Wenn das Zurückgehen nicht genug Platz freigeben kann, führen Sie `/clear` aus, um eine neue Sitzung zu starten. Ihr vorheriges Gespräch wird beibehalten und kann mit `/resume` erneut geöffnet werden.

Diese Meldung und andere `/compact`-Fehler werden in Fehlerformatierung angezeigt. Vor v2.1.216 wurden sie im gleichen schwachen Stil wie erfolgreiche Befehlsausgabe gerendert, daher könnten Sie eine fehlgeschlagene Komprimierung als Erfolg lesen.

<h3 id="request-too-large">
  Anfrage zu groß
</h3>

Der rohe Anfragekörper überschritt das 32-MB-Limit der API vor der Tokenisierung, normalerweise wegen großer eingefügter Inhalte, Werkzeugergebnisse oder Anhänge. Dieses Limit ist separat vom [Kontextfenster](#prompt-is-too-long).

```text theme={null}
Request too large (max 32MB). Accumulated images and attachments in the conversation pushed the request over the limit. Run /compact, or double press esc to go back and remove attachments.
```

Wenn die Anfrage direkt zur Claude-API ging und die API selbst sie ablehnte, misst Claude Code das Gespräch und formuliert die Meldung danach, ob die Wiederherstellung funktionieren kann. Über einen Proxy, ein Gateway oder einen Cloud-Anbieter erhalten Sie die allgemeine Meldung. Die gemessenen Formen:

* `Request too large (max 32MB; 20.1MB of about 33.4MB is images or documents).`: Bilder oder Dokumente haben die Anfrage über das Limit hinausgetrieben. Claude Code versucht es erneut, ohne sie.
* `Request too large for the API's 32MB request limit`: Die Nachrichten allein sind über dem Limit, daher sagt die Meldung `compacting cannot make it fit` und Claude Code versucht es nicht erneut. Im [nicht-interaktiven Modus](/docs/de/headless) sagt die Meldung Ihnen, die Eingabe zu reduzieren oder stattdessen eine neue Sitzung zu starten.

Vor v2.1.212 schlugen Gespräche mit genug angesammelten Bildern bei jedem Durchgang mit `Request too large (max 32MB). Double press esc to go back and try with a smaller file.` fehl. Vor v2.1.229 zeigte Claude Code den Anhang-Rat für jede Ablehnung an, auch wenn die Komprimierung nicht helfen konnte.

**Was zu tun ist:**

* Wenn die Meldung sagt `compacting cannot make it fit`, drücken Sie zweimal Esc, um über die Runde zurückzugehen, die den großen Inhalt hinzugefügt hat, oder führen Sie `/clear` aus, um neu zu beginnen
* Führen Sie andernfalls `/compact` aus, das angesammelte Bilder und Anhänge entfernt
* Referenzieren Sie große Dateien nach Pfad, anstatt ihren Inhalt einzufügen, damit Claude sie in Chunks lesen kann
* Für Bilder siehe [Bild war zu groß](#image-was-too-large) unten

<h3 id="image-was-too-large">
  Bild war zu groß
</h3>

Ein eingefügtes oder angehängtes Bild überschreitet die Größen- oder Dimensionslimits der API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code ersetzt das nicht verarbeitbare Bild durch einen Textplatzhalter und versucht es erneut, daher sind nachfolgende Nachrichten erfolgreich. In Versionen vor 2.1.142 konnte ein eingefügtes Bild im Gespräch bleiben und denselben Fehler bei jeder nachfolgenden Nachricht wiederholen. Um sich auf diesen Versionen zu erholen, drücken Sie zweimal Esc und gehen Sie über die Runde zurück, in der das Bild hinzugefügt wurde.

**Was zu tun ist:**

* Ändern Sie die Größe des Bildes, bevor Sie es einfügen. Die API akzeptiert Bilder bis zu 8000 Pixeln auf der längsten Kante für ein einzelnes Bild oder 2000 Pixel, wenn viele Bilder im Kontext sind.
* Machen Sie einen engeren Screenshot des relevanten Bereichs statt des gesamten Bildschirms

<h3 id="unable-to-resize-image">
  Bild konnte nicht in der Größe geändert werden
</h3>

Claude Code konnte ein angehängtes Bild nicht herunterskalieren, bevor es zur API gesendet wurde.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
Unable to resize image — it is a CMYK JPEG, which Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Re-save it as an RGB PNG or JPEG and try again.
Unable to resize image — it is an animated WebP whose first frame Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Save its first frame as a PNG or JPEG and try again.
Unable to resize image — its pixels could not be decoded (the file may be damaged, or use an encoding Claude Code cannot read), and it is over the … API limit (… raw, … base64), so it cannot be sent. Re-save it as a PNG or JPEG and try again.
```

Claude Code ändert normalerweise die Größe großer Bilder automatisch. Diese Fehler bedeuten, dass das Bild nicht dekodiert oder in der Größe geändert werden konnte, um in die API-Limits zu passen.

**Was zu tun ist:**

* Wenn die Meldung Sie auffordert, das Bild zu konvertieren, konvertieren Sie es in PNG, JPEG, GIF oder WebP und hängen Sie es erneut an. Claude Code kann Dimensionen für diese Formate überprüfen, ohne das Bild zu dekodieren.
* Wenn die Meldung ein Dimensions- oder Größenlimit meldet, ändern Sie die Größe oder komprimieren Sie das Bild unter diesem Limit, bevor Sie es anhängen.
* Wenn die Meldung eine Ursache nennt, wie z. B. ein CMYK JPEG, ein animiertes WebP oder eine möglicherweise beschädigte Datei, speichern Sie das Bild in dem Format, das die Meldung vorschlägt, und hängen Sie es erneut an.

<h3 id="pdf-errors">
  PDF-Fehler
</h3>

Das PDF, das Sie angehängt haben, konnte nicht verarbeitet werden. Die Meldungen werden hier in ihrer nicht-interaktiven Form angezeigt; in einer interaktiven Sitzung fordern sie Sie stattdessen auf, zweimal Esc zu drücken und es erneut zu versuchen.

```text theme={null}
PDF too large (max 100 pages, 20MB). Try reading the file a different way (e.g., extract text with pdftotext).
PDF is password protected. Try using a CLI tool to extract or convert the PDF.
The PDF file was not valid. Try converting it to text first (e.g., pdftotext).
```

**Was zu tun ist:**

* Für übergroße PDFs bitten Sie Claude, einen Seitenbereich mit dem Read-Werkzeug zu lesen, anstatt die ganze Datei anzuhängen, oder extrahieren Sie Text mit einem Werkzeug wie `pdftotext` und referenzieren Sie die Ausgabedatei nach Pfad
* Für geschützte oder ungültige PDFs entfernen Sie das Passwort oder exportieren Sie die Datei erneut aus ihrer Quellanwendung, dann versuchen Sie es erneut

<h3 id="extra-inputs-are-not-permitted">
  Zusätzliche Eingaben sind nicht zulässig
</h3>

Ein Proxy oder LLM-Gateway zwischen Claude Code und der API hat den `anthropic-beta`-Anfragekopf entfernt, daher lehnte die API Felder ab, die davon abhängen.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code sendet Beta-only-Felder wie `context_management` und `effort` zusammen mit einem `anthropic-beta`-Kopf, der sie aktiviert. Wenn ein Gateway den Körper weiterleitet, aber den Kopf entfernt, sieht die API Felder, die sie nicht erkennt.

**Was zu tun ist:**

* Konfigurieren Sie Ihr Gateway, um den `anthropic-beta`-Kopf weiterzuleiten. Siehe [Feature-Durchleitung](/docs/de/llm-gateway-protocol#feature-pass-through) für das, was Gateways weiterleiten müssen.
* Setzen Sie als Fallback [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/de/env-vars) vor dem Start. [Deaktivieren Sie Pre-Release-Funktionen](/docs/de/llm-gateway-protocol#disable-pre-release-capabilities) behandelt den genauen Umfang.

<h3 id="tool-input-schema-is-invalid">
  Werkzeug-Eingabeschema ist ungültig
</h3>

Ein Werkzeug in der Anfrage deklarierte ein `input_schema`, das die JSON-Schema-Validierung der API nicht besteht, daher lehnte die API die ganze Anfrage ab. Die Nummer nach `tools.` ist die Position des fehlgeschlagenen Werkzeugs in der Werkzeugliste der Anfrage, nicht ein Name, den Sie nachschlagen können.

```text theme={null}
API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid
API Error: 400 ... tools.N.custom.input_schema.properties: Property keys should match pattern '^[a-zA-Z0-9_.-]{1,64}$'
```

Die erste Form bedeutet, dass das Schema nicht gültiges JSON Schema draft 2020-12 ist. Die zweite bedeutet, dass ein Eigenschaftsname auf oberster Ebene nicht dem Muster entspricht, das die Meldung zitiert.

Claude Code [schließt MCP-Werkzeuge aus, deren Eingabeschema diese Validierung nicht bestehen würde](/docs/de/mcp#tools-with-invalid-input-schemas), wenn es die Werkzeuge eines Servers lädt, daher enthalten Anfragen normalerweise nie eines.

Auf einer [Bereitstellung, bei der das Flag-Abrufen ausgeschaltet ist](/docs/de/env-vars#features-that-need-feature-flag-fetching), oder auf einer Maschine, deren Flags nie angekommen sind, zeichnet Claude Code im Serverprotokoll auf, welches Werkzeug abgelehnt würde, sendet es aber trotzdem, daher kann dieser Fehler immer noch auftreten.

Der Fehler kann auch für ein Werkzeug auftreten, dessen Schema ein JSON-Schema-Dialekt deklariert, der nicht draft 2020-12 in `$schema` ist. Claude Code überprüft diese Schemas nicht gegen das JSON-Schema-Meta-Schema, obwohl die Überprüfung des Eigenschaftsnamens auf oberster Ebene immer noch gilt.

Vor v2.1.216 führte keine Bereitstellung die Ausschlussüberprüfungen durch.

**Was zu tun ist:**

* Wenn Ihre Claude-Code-Version älter als v2.1.216 ist, führen Sie `claude update` aus.
* Entfernen oder [deaktivieren](/docs/de/mcp#disable-a-server-without-removing-it) Sie den MCP-Server, der das ungültige Schema deklariert. Der Fehler nennt das Werkzeug nur nach Position. Auf v2.1.216 oder später überprüfen Sie das Protokoll jedes Servers auf eine Zeile, die ein Werkzeug nennt, dessen Eingabeschema abgelehnt würde. Wenn kein Protokoll eines nennt, deaktivieren Sie Server nacheinander.
* Wenn Sie den Server verwalten, beheben Sie das `input_schema` des Werkzeugs. Das Schema muss gültiges JSON-Schema sein, und Eigenschaftsnamen auf oberster Ebene müssen 1 bis 64 Zeichen lang sein und nur ASCII-Buchstaben und Ziffern, `_`, `.` und `-` verwenden. Siehe [Werkzeuge mit ungültigen Eingabeschemas](/docs/de/mcp#tools-with-invalid-input-schemas).

<h3 id="theres-an-issue-with-the-selected-model">
  Es gibt ein Problem mit dem ausgewählten Modell
</h3>

Der konfigurierte Modellname wurde nicht erkannt oder Ihr Konto hat keinen Zugriff darauf. Ab v2.1.160 variiert der nachfolgende Hinweis, der hier in seiner interaktiven Form angezeigt wird, je nach Oberfläche.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Was zu tun ist:**

* **Interaktive CLI**: Führen Sie `/model` aus, um aus Modellen auszuwählen, die für Ihr Konto verfügbar sind.
* **Nicht-interaktiver Modus (`-p`)**: Übergeben Sie `--model` mit einem gültigen Alias oder einer ID, oder setzen Sie [`ANTHROPIC_MODEL`](/docs/de/env-vars). Der Fehlertext zeigt `Run --model` auf dieser Oberfläche.
* **Agent SDK**: Der Fehlertext lässt den Hinweis weg, da das Modell programmgesteuert gesetzt wird. Setzen Sie [`model` auf `Options`](/docs/de/agent-sdk/typescript#options) in TypeScript oder [`ClaudeAgentOptions(model=...)`](/docs/de/agent-sdk/python#claudeagentoptions) in Python, und behandeln Sie den strukturierten `model_not_found`-Fehler, um Ihre eigene Wiederholung oder Modellauswahl anzuzeigen.
* Verwenden Sie einen Alias wie `sonnet` oder `opus` statt einer vollständigen versionierten ID. Aliase werden zu einem verwalteten Standard aufgelöst, daher werden sie nicht veraltet. Siehe [Modellkonfiguration](/docs/de/model-config).
* Wenn das falsche Modell in der CLI immer wieder zurückkommt, ist eine veraltete ID irgendwo gesetzt. Überprüfen Sie die Orte, an denen Sie ein Modell in [Prioritätsreihenfolge](/docs/de/model-config#setting-your-model) setzen können, und entfernen Sie den veralteten Wert.
* Ein neu gestartetes Modell kann auf der Anthropic-API verfügbar sein, bevor Amazon Bedrock, Google Clouds Agent Platform oder Microsoft Foundry es anbietet. Wenn Sie eine neue Modell-ID auf einem dieser Anbieter angeheftet haben und diesen Fehler sehen, überprüfen Sie den Modellkatalog Ihres Anbieters auf Verfügbarkeit in Ihrer Region, und behalten Sie die vorherige Version angeheftet, bis die neue dort erscheint.
* Claude Code meldet einen abgelaufenen claude.ai-Login als [Login abgelaufen](#login-expired), nicht als dieser Fehler. Vor v2.1.206 schlug ein abgelaufener Login, der nicht mehr aktualisiert werden konnte, bei jedem Modell mit diesem Fehler fehl; führen Sie `/login` aus, wenn Sie das auf einer älteren Version sehen.
* Für Google Clouds Agent Platform-Bereitstellungen siehe [Google Cloud Agent Platform-Fehlerbehebung](/docs/de/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Modell ist keine erkannte Modell-ID
</h3>

Die Modellzeichenkette, die Sie an einen Modellwechsel übergeben haben, ist kein Modellalias, keine Modell-ID, die diese Claude-Code-Version kennt, oder keine ID, die mit `claude-` beginnt. Die üblichen Ursachen sind ein Tippfehler in der ID, ein Anzeigename wie `Sonnet 5`, bei dem die ID `claude-sonnet-5` erwartet wird, oder ein Alias, den nur neuere Claude-Code-Versionen erkennen. Claude Code lehnt den Wechsel sofort ab. Vor v2.1.200 speicherte Claude Code die Zeichenkette und schlug bei der nächsten Anfrage mit [Es gibt ein Problem mit dem ausgewählten Modell](#theres-an-issue-with-the-selected-model) fehl.

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

Der nachfolgende Hinweis nennt den nächsten passenden Alias oder die nächste Modell-ID. Wenn nichts nah genug ist, lautet es stattdessen `Run /model to see available models.`

Claude Code erzeugt diesen Fehler lokal in dem Moment, in dem der Wechsel angefordert wird, bevor eine API-Anfrage gestellt wird. Er gilt, wenn ein Modell durch die [Agent SDK](/docs/de/agent-sdk/typescript) `setModel()`-Methode, durch eine App wie die [Desktop-App](/docs/de/desktop), die die Claude-Code-CLI für Sie ausführt, oder wenn Sie ein Modell von einem Gerät auswählen, das über [Remote Control](/docs/de/remote-control) verbunden ist, gesetzt wird. Vor v2.1.260 deckte die Überprüfung Remote Control-Auswahlen nicht ab, daher wendete Claude Code die Auswahl an und die nächste Anfrage schlug mit [Es gibt ein Problem mit dem ausgewählten Modell](#theres-an-issue-with-the-selected-model) fehl.

**Was zu tun ist:**

* Führen Sie `/model` ohne Argument aus, um die Auswahl zu öffnen und aus den Modellen auszuwählen, die für Ihr Konto verfügbar sind, dann übergeben Sie den dort angezeigten Alias oder die ID
* Wenn Sie einen Alias verwendet haben, den eine neuere Claude-Code-Version unterstützt, führen Sie `claude update` aus. Eine vollständige ID, die mit `claude-` beginnt, besteht diese lokale Überprüfung, auch wenn das Modell neuer als Ihre Claude-Code-Version ist. Der Server kann immer noch eine Mindestversion für dieses Modell erfordern; siehe [Claude Code unterstützt dieses Modell nicht](#claude-code-does-not-support-this-model).
* Ein Modell, das vor v2.1.200 gespeichert wurde, wird durch diese Überprüfung nicht repariert. Wenn ein veralteter Wert immer wieder zurückkommt, entfernen Sie ihn aus den unter [Setzen Sie Ihr Modell](/docs/de/model-config#setting-your-model) aufgelisteten Orten.
* Die Überprüfung wird nur auf der Anthropic-API ausgeführt. Auf jedem anderen Anbieter oder Gateway, einschließlich eines benutzerdefinierten `ANTHROPIC_BASE_URL`, definiert der Anbieter die Modellnamen, daher akzeptiert Claude Code jede Zeichenkette und leitet sie durch. Claude Code kann immer noch die [nicht erkannte Modell-Diagnosezeile](#unrecognized-model-id-on-a-request) zur Anfragetime auf jedem Anbieter schreiben.

<h3 id="model-not-found">
  Modell nicht gefunden
</h3>

Sie haben ein Modell mit `/model <name>` ausgewählt und Claude Code konnte nicht bestätigen, dass ein Modell mit diesem Namen existiert. Wenn der Name kein [Modellalias](/docs/de/model-config#model-aliases) oder eine andere Schreibweise ist, die Claude Code lokal akzeptiert, überprüft `/model` ihn mit einer minimalen API-Anfrage, und dieser Fehler ist normalerweise die Antwort Ihres API-Endpunkts. Ein Name, der überhaupt keine Modell-ID sein kann, wie einer mit Leerzeichen, erhält die gleiche Meldung.

```text theme={null}
Model 'claude-opus-9' not found
```

Bei Anbietern mit anbieterspezifischen Modell-IDs kann die Meldung einen `Try '...' instead`-Vorschlag hinzufügen, der die ID Ihres Anbieters für ein Fallback-Modell nennt.

**Was zu tun ist:**

* Führen Sie `/model` ohne Argument aus und wählen Sie aus den Modellen, die für Ihr Konto verfügbar sind, oder verwenden Sie einen [Modellalias](/docs/de/model-config#model-aliases) wie `sonnet`, der zu einem verwalteten Standard aufgelöst wird
* Wenn Sie eine vollständige ID eingegeben haben, überprüfen Sie sie gegen den Modellkatalog Ihres Anbieters. Ein neu gestartetes Modell kann auf der Anthropic-API verfügbar sein, bevor Ihr Anbieter oder Ihre Region es anbietet.
* Vor v2.1.265 lehnte `/model` auch die `opusplan[1m]`-Alias-Schreibweise mit diesem Fehler ab. Auf diesen Versionen aktualisieren Sie Claude Code, oder setzen Sie das Modell in [Einstellungen](/docs/de/model-config#setting-your-model) oder mit `--model` statt.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus ist nicht mit dem Claude Pro-Plan verfügbar
</h3>

Ihr aktiver Abonnementplan enthält nicht das Modell, das Sie ausgewählt haben.

```text theme={null}
Claude Opus is not available with the Claude Pro plan. If you have updated your subscription plan recently, run /logout and /login for the plan to take effect.
```

**Was zu tun ist:**

* Führen Sie `/model` aus und wählen Sie ein Modell, das Ihr Plan enthält
* Wenn Sie Ihren Plan kürzlich aktualisiert haben und dies immer noch sehen, führen Sie `/logout` und dann `/login` aus. Das gespeicherte Token spiegelt Ihren Plan zum Zeitpunkt der Anmeldung wider, daher wird ein Upgrade im Web erst wirksam, wenn Sie sich in einer bestehenden Sitzung erneut authentifizieren.
* Siehe [claude.com/pricing](https://claude.com/pricing) für die Modelle, die jeder Plan enthält

<h3 id="claude-code-does-not-support-this-model">
  Claude Code unterstützt dieses Modell nicht
</h3>

Die API lehnte die Anfrage mit einem 400 ab, weil Ihre Claude-Code-Version unter einem erforderlichen Minimum liegt. Entweder erfordert das Modell, das Sie ausgewählt haben, eine neuere Version, die der Server pro Modell überprüft, oder die Richtlinie Ihrer Organisation erfordert eine. Der 400 trägt den Fehlercode `claude_code_version_too_old`, und die Meldung sagt, welches Minimum gilt.

```text theme={null}
API Error: 400 Claude Code 2.1.219 does not support this model; version 2.1.255 or newer is required. Run 'claude update', or update the Claude desktop app, then try again.
```

Die Richtlinien-Formulierung der Organisation lautet:

```text theme={null}
API Error: 400 Claude Code 2.1.240 is older than the minimum version required by your organization's policy. Run 'claude update', or update the Claude desktop app, to continue.
```

**Was zu tun ist:**

* Führen Sie `claude update` aus, oder aktualisieren Sie die Claude-Desktop-App, dann starten Sie eine neue Sitzung
* Für die Pro-Modell-Formulierung können Sie in der aktuellen Sitzung weitermachen, indem Sie mit `/model` zu einem anderen Modell wechseln
* Für die Organisationsrichtlinien-Formulierung aktualisieren Sie, bevor Sie fortfahren

<h3 id="model-is-restricted-by-your-organizations-settings">
  Modell ist durch die Einstellungen Ihrer Organisation eingeschränkt
</h3>

Ihr Organisationsadministrator hat dieses Modell in der claude.ai-Administratorkonsole deaktiviert, oder es ist durch eine [`availableModels`](/docs/de/model-config#restrict-model-selection)-Zulassungsliste in verwalteten Einstellungen ausgeschlossen. Wenn das eingeschränkte Modell mit `--model`, `ANTHROPIC_MODEL` oder der `model`-Einstellung gesetzt wurde, ersetzt Claude Code ein zulässiges Modell und fährt fort. Das Eingeben von `/model <name>` für ein eingeschränktes Modell wird mit `Run /model to choose a different model.` abgelehnt und die Sitzung behält ihr aktuelles Modell. Die Ersetzungsmitteilung kann auch mid-session nach einem Admin-Deaktivieren des Modells, auf dem eine Sitzung läuft, in der claude.ai-Administratorkonsole angezeigt werden.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Ein Hinweis mit dem Präfix eines Agenten, einer Fähigkeit oder eines Befehls bedeutet, dass die Einschränkung auf das [angeforderte Modell des Subagenten](/docs/de/sub-agents#choose-a-model) angewendet wurde: Der Subagent wird auf dem ersetzten Modell ausgeführt und das Modell Ihrer Sitzung ist unverändert. Vor v2.1.223 zeigte Claude Code den Hinweis nur für Subagenten, die mit dem Agent-Werkzeug gestartet wurden.

Claude Code behandelt einen Modellfamilien-Alias, einen von `opus`, `sonnet`, `haiku` oder `fable`, als Anfrage für diese Familie statt für ihre neueste Version. Auf der Anthropic-API und auf [Claude Platform on AWS](/docs/de/claude-platform-on-aws) wird ein eingeschränkter Familien-Alias zur neuesten Version der Familie aufgelöst, die Ihre Organisation und die `availableModels`-Zulassungsliste zulassen, und der Ersetzungshinweis nennt diese Version. Claude Code lehnt `/model <alias>` nur ab, wenn jede Version der Familie eingeschränkt ist. Vor v2.1.205 wurde ein Familien-Alias basierend auf seiner neuesten Version allein ersetzt oder abgelehnt, auch wenn eine ältere Version derselben Familie zulässig war.

**Was zu tun ist:**

* Führen Sie `/model` aus, um aus den Modellen auszuwählen, die Ihre Organisation zulässt. Eingeschränkte Modelle sind in der Auswahl verborgen.
* Wenn das eingeschränkte Modell in `--model`, `ANTHROPIC_MODEL`, dem `model`-Feld einer Einstellungsdatei oder dem `model`-Frontmatter eines [Subagenten](/docs/de/sub-agents#choose-a-model), einer Fähigkeit oder eines Befehls gesetzt wurde, entfernen oder aktualisieren Sie diesen Wert, damit der Hinweis nicht erneut auftritt
* Wenn Sie Zugriff auf das eingeschränkte Modell benötigen, bitten Sie Ihren Organisationsadministrator, es zu aktivieren. Siehe [Organisationsmodell-Einschränkungen](/docs/de/model-config#organization-model-restrictions).

<h3 id="model-switch-was-blocked-by-a-premodelswitch-hook">
  Modellwechsel wurde durch einen PreModelSwitch-Hook blockiert
</h3>

Ein [PreModelSwitch-Hook](/docs/de/hooks#premodelswitch) hat den Modellwechsel, den Sie oder ein Client angefordert haben, nicht genehmigt, daher behält die Sitzung ihr aktuelles Modell. Wenn der Wechsel von einem [Agent SDK](/docs/de/agent-sdk/overview)-Host oder [Remote Control](/docs/de/remote-control) statt von einem Befehl kam, den Sie eingegeben haben, lautet die Meldung `Model switch blocked by a PreModelSwitch hook` ohne Nennung des Zielmodells.

```text theme={null}
Model switch to Opus 4.6 was blocked by a PreModelSwitch hook: Opus 4.6 is retired for this project. Use a newer model.
```

Der Grund nach dem Doppelpunkt sagt, was den Wechsel ablehnte:

* **Ein Grund, den ein Hook geschrieben hat**: Ein PreModelSwitch-Hook lieferte diesen Grund, wenn er [den Wechsel ablehnte oder um Bestätigung bat](/docs/de/hooks#premodelswitch-decision-control). Beheben Sie, was er verlangt, oder wählen Sie ein Modell, das Ihre Hooks zulassen.
* **`PreModelSwitch hook <name> did not respond before its timeout`**: Ein Hook, der nicht vor seinem [Timeout](/docs/de/hooks#timeouts) antwortet, blockiert den Wechsel. Beheben Sie den hängenden Befehl oder erhöhen Sie das `timeout` dieses Hooks, dann wechseln Sie erneut.
* **`confirmation required, and this session cannot ask`**: Ein Hook antwortete `ask` ohne Grund, und eine Kontrollabfrage hat keine Möglichkeit, die Bestätigungsaufforderung anzuzeigen. Ein `/model`-Befehl in einem [`-p`-Lauf](/docs/de/headless) meldet die gleiche Bedingung mit `(run /model interactively to confirm)` nach dem Grund. Machen Sie den Wechsel von einer interaktiven Sitzung, oder ändern Sie die Entscheidung des Hooks für dieses Modell.
* **`so organization-managed PreModelSwitch hooks could not be checked`**: Claude Code konnte nicht sagen, welche PreModelSwitch-Hooks Ihre Organisation [verwaltete Plugins](/docs/de/settings-reference#enabledplugins) liefern, zum Beispiel weil ein verwaltetes Plugin nicht geladen werden konnte. Einer dieser Hooks könnte den Wechsel blockieren, daher weigert sich Claude Code, statt den Wechsel unkontrolliert anzuwenden. Der Anfang des Grundes nennt, was fehlgeschlagen ist. Claude Code überprüft bei jedem Wechselversuch erneut, daher stoppt ein Fehler, der seitdem geklärt wurde, das Blockieren; wenn es weiterhin fehlschlägt, führen Sie `claude --debug` aus und wechseln Sie erneut, um die Details zu erfassen, dann beheben Sie das Plugin oder bitten Sie Ihren Admin, es zu beheben.
* **`a PreModelSwitch hook failed before answering`** oder **`PreModelSwitch hooks were cancelled (the control stream closed) before answering`**: Der Hook-Lauf endete ohne Urteil, und Claude Code behandelt das nicht als Genehmigung. Führen Sie `claude --debug` aus, um zu sehen, was fehlgeschlagen ist, dann wechseln Sie erneut.

Vor v2.1.260 lautete die verwaltete Plugin-Ablehnung `plugin hooks could not be loaded, so PreModelSwitch hooks could not be checked; see the debug log`. Claude Code wiederholte das Plugin-Laden einmal und lehnte dann später Wechsel in der Sitzung ab, auch wenn Ihre Organisation keine Plugins verwaltete. Starten Sie die Sitzung auf diesen Versionen neu, um das Plugin-Laden erneut auszuführen.

<h3 id="couldnt-save-it-as-your-default">
  Konnte es nicht als Standard speichern
</h3>

Sie haben ein Modell ausgewählt, um es als Standard zu speichern, zum Beispiel mit `/model <name>` oder `Enter` in der `/model`-Auswahl, und Claude Code konnte die Auswahl nicht in Ihre Benutzereinstellungsdatei `~/.claude/settings.json` schreiben. Der Wechsel selbst wurde angewendet, daher läuft die aktuelle Sitzung auf dem Modell, das Sie ausgewählt haben, aber Ihr Standard ist unverändert und die nächste Sitzung startet auf dem alten Wert.

```text theme={null}
Set model to Fable 5.1 for this session only · couldn't save it as your default: ~/.claude/settings.json can't be written (EROFS)
```

Der Grund nach dem Dateipfad sagt, was fehlgeschlagen ist:

* **`can't be written (<code>)`**: Der Schreibvorgang schlug mit dem Betriebssystemfehlercode in Klammern fehl, wie `EROFS`, wenn die Datei oder die Datei, auf die sie verweist, auf einem Dateisystem sitzt, das Schreibvorgänge ablehnt. Machen Sie die Datei beschreibbar und wechseln Sie erneut. Wenn ein anderes Werkzeug die Datei erzeugt, setzen Sie stattdessen den `model`-Schlüssel in diesem Werkzeug; siehe [Eine Änderung, die Sie in Claude Code gemacht haben, geht in neuen Sitzungen verloren](/docs/de/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **`isn't valid JSON`**: Die Datei auf der Festplatte wird nicht geparst, und Claude Code lässt sie unverändert, statt Inhalte zu überschreiben, die es nicht zurücklesen kann. Beheben Sie den Syntaxfehler, dann wechseln Sie erneut; siehe [Beheben Sie eine kaputte Einstellungsdatei](/docs/de/settings#fix-a-broken-settings-file).

Ein Hinweis, der mit `couldn't confirm it was saved as your default (~/.claude/settings.json is still being written)` endet, bedeutet, dass der Schreibvorgang nach drei Sekunden nicht beendet war. Er wird im Hintergrund fortgesetzt, daher kann der Standard immer noch gespeichert werden; überprüfen Sie, mit welchem Modell Ihre nächste Sitzung startet, oder führen Sie `/model <name>` erneut aus.

Vor v2.1.265 sagte der Hinweis, das Modell sei `saved as your default for new sessions`, auch wenn der Schreibvorgang fehlgeschlagen war.

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled wird für dieses Modell nicht unterstützt
</h3>

Ihre Claude-Code-Version ist älter als das Minimum für das ausgewählte Modell. Die CLI hat eine Thinking-Konfiguration gesendet, die das Modell nicht mehr akzeptiert.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Was zu tun ist:**

* Führen Sie `claude update` aus und starten Sie Claude Code neu. Opus 4.7 benötigt v2.1.111 oder später. Opus 4.8 benötigt v2.1.154 oder später. Sonnet 5 benötigt v2.1.197 oder später. Opus 5 benötigt v2.1.219 oder später. Opus 5.5 benötigt v2.1.280 oder später
* Wenn Sie nicht aktualisieren können, führen Sie `/model` aus und wählen Sie stattdessen Opus 4.6 oder Sonnet 4.6
* Wenn Sie dies im [Agent SDK](/docs/de/agent-sdk/overview) treffen, aktualisieren Sie stattdessen das SDK-Paket. Opus 4.8 benötigt TypeScript SDK v0.3.154 oder später und Python SDK v0.2.88 oder später. Sonnet 5 benötigt TypeScript SDK v0.3.197 oder später. Opus 5 benötigt TypeScript SDK v0.3.219 oder später. Opus 5.5 benötigt TypeScript SDK v0.3.280 oder später

<h3 id="effort-isnt-available-with-thinking-turned-off">
  Effort ist nicht verfügbar, wenn Thinking ausgeschaltet ist
</h3>

Sie haben [erweitertes Thinking](/docs/de/model-config#extended-thinking) ausgeschaltet und führen auf einer [Effort-Stufe](/docs/de/model-config#adjust-effort-level) über `high` aus. Das Modell akzeptiert diese Kombination nicht, daher lehnte die API die Anfrage ab.

```text theme={null}
API Error: Effort 'xhigh' isn't available with thinking turned off on this model · run /effort high to continue, or turn thinking back on (unset MAX_THINKING_TOKENS=0)
```

**Was zu tun ist:**

* [Senken Sie die Effort-Stufe](/docs/de/model-config#set-the-effort-level) auf `high` oder darunter.
* Schalten Sie Thinking wieder ein, z. B. durch Aufheben von [`MAX_THINKING_TOKENS`](/docs/de/env-vars) oder Entfernen von [`"alwaysThinkingEnabled": false`](/docs/de/settings-reference#alwaysthinkingenabled) aus Ihren Einstellungen.

Vor v2.1.242 zeigte Claude Code die eigene Meldung der API: `API Error: 400 output_config.effort 'xhigh' is not supported when thinking is disabled on this model. Use effort 'high' or below, or enable thinking.` Vor v2.1.251 sendete Claude Code die Anfrage auf der Effort-Stufe, die Sie gesetzt haben, daher lehnte Opus 5 jede Anfrage über `high` mit ausgeschaltetem Thinking ab. Claude Code sendet jetzt Effort `high` stattdessen an Modelle, von denen es weiß, dass sie die Kombination ablehnen, wie Opus 5, daher erreicht dieser Fehler Sie auf v2.1.251 oder später nur von einem Modell, das Claude Code nicht kennt, das es ablehnt.

<h3 id="thinking-budget-exceeds-output-limit">
  Thinking-Budget überschreitet Ausgabelimit
</h3>

Das konfigurierte erweiterte Thinking-Budget überschreitet die maximale Antwortlänge, daher bleibt kein Platz für die tatsächliche Antwort.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code passt diese Werte automatisch auf der Anthropic-API an. Sie sehen diesen Fehler normalerweise auf Amazon Bedrock oder Google Clouds Agent Platform, wenn [`MAX_THINKING_TOKENS`](/docs/de/env-vars) höher als das Ausgabelimit des Anbieters gesetzt ist, oder wenn der Plan-Modus das Thinking-Budget erhöht.

**Was zu tun ist:**

* Senken Sie `MAX_THINKING_TOKENS`, oder erhöhen Sie [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/de/env-vars) über das Thinking-Budget
* Siehe [Erweitertes Thinking](/docs/de/model-config#extended-thinking) für die Interaktion des Budgets mit der Ausgabelänge

<h3 id="tool-use-or-thinking-block-mismatch">
  Werkzeugverwendung oder Thinking-Block-Nichtübereinstimmung
</h3>

Die Gesprächshistorie erreichte die API in einem inkonsistenten Zustand, normalerweise nachdem ein Werkzeugaufruf unterbrochen oder eine Runde während des Streams bearbeitet wurde.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 orphaned tool_result in conversation history. Run /rewind to recover the conversation.
API Error: 400 duplicate tool_use ID in conversation history. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Alle Varianten bedeuten dasselbe: Die Abfolge von `tool_use`-, `tool_result`- und `thinking`-Blöcken in der Historie stimmt nicht mehr mit dem überein, was die API erwartet.

**Was zu tun ist:**

* Wenn Sie Opus 4.7 oder Opus 4.8 verwenden, führen Sie zuerst `claude update` aus. Versionen vor v2.1.156 können diesen Fehler während normaler Werkzeugverwendung auslösen, und `/rewind` löscht ihn nicht.
* Führen Sie `/rewind` aus, oder drücken Sie zweimal Esc, um zu einem Checkpoint vor der beschädigten Runde zurückzugehen und von dort aus fortzufahren. Siehe [Checkpointing](/docs/de/checkpointing) für die Erstellung und Wiederherstellung von Checkpoints.

<h3 id="unsupported-tool-content-removed">
  Nicht unterstützter Werkzeuginhalt entfernt
</h3>

Wenn Claude Code direkt mit der Anthropic-API verbunden ist und eine gespeicherte Sitzung lädt oder in der Vorschau anzeigt, entfernt es Werkzeuginhalte, die die Anthropic-API nicht akzeptiert, und lässt diese Zeile dort, wo entfernter Inhalt zwischen zwei Thinking-Blöcken saß:

```text theme={null}
[Unsupported tool content removed]
```

Solcher Inhalt erreicht eine Sitzungsdatei, wenn etwas anderes als die Anthropic-API im Format der API antwortet, normalerweise ein Drittanbieter-Proxy, der durch [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) gesetzt ist und die Werkzeugaufrufe eines anderen Anbieters übersetzt. Claude Code entfernt ihn nur, wenn die Sitzung direkt mit der Anthropic-API verbunden ist, und lädt die gespeicherte Historie, wie sie ist, wenn die Sitzung über einen Proxy oder auf einem anderen Anbieter läuft. Vor v2.1.246 sendete Claude Code die Werkzeugverwendung und ihr Ergebnis zurück zur API, und jede Runde der wiederaufgenommenen Sitzung schlug mit einem 400-Fehler wie `messages.1.content.0.server_tool_use.name: Input should be 'web_search', 'web_fetch', ...` fehl.

**Was zu tun ist:**

* Keine Aktion erforderlich, wenn Sie die Platzhalterzeile sehen. Die Sitzung wird ohne den entfernten Inhalt fortgesetzt.
* Wenn jede Runde einer wiederaufgenommenen Sitzung stattdessen mit dem 400-Fehler fehlschlägt, führen Sie `claude update` aus und nehmen Sie die Sitzung erneut auf. Versionen vor v2.1.246 entfernen den Inhalt nicht.

<h3 id="role-system-must-precede-an-assistant-message">
  role 'system' must precede an 'assistant' message
</h3>

Die API lehnte die Anfrage mit einem 400 ab, weil eine Systemnachricht an einer Position im Gespräch sitzt, die sie nicht akzeptiert:

```text theme={null}
API Error: 400 messages.6: role 'system' must precede an 'assistant' message or end the array; ...
```

Claude Code sendet einige seiner Erinnerungs- und Anhang-Texte als Systemnachrichten innerhalb des Gesprächs. Wenn die API die Position einer ablehnt, versucht Claude Code die Anfrage einmal erneut mit diesem Text, der stattdessen als gewöhnliche Benutzernachrichten gesendet wird. Die Geschwister-Platzierungs-Formulierungen der API, wie `use the top-level 'system' parameter for the initial system prompt`, erhalten die gleiche Wiederherstellung.

Wenn der Fehler angezeigt wird, ist die abgelehnte Systemnachricht nicht eine, die Claude Code entfernen kann. Das bedeutet normalerweise, dass ein Proxy oder [LLM-Gateway](/docs/de/llm-gateway) zwischen Claude Code und der API eine Systemnachricht hinzugefügt oder das Gespräch neu angeordnet hat.

**Was zu tun ist:**

* Führen Sie `/clear` aus, um ein neues Gespräch zu starten. Wenn der Fehler dort auch zurückkommt, ist die Ursache auf dem Anfragepfad, nicht im gespeicherten Gespräch.
* Wenn der Fehler sich bei jedem Durchgang hinter einem Proxy oder Gateway wiederholt, das durch [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) konfiguriert ist, verbinden Sie sich ohne den Proxy, um die Quelle zu bestätigen, und melden Sie den Fehler demjenigen, der ihn betreibt

Vor v2.1.280 erkannte Claude Code diese Formulierung nicht, daher erschien der Fehler auch, wenn die abgelehnte Systemnachricht eine war, die Claude Code selbst sendete, und jeder spätere Durchgang des Gesprächs schlug auf die gleiche Weise fehl.

<h3 id="invalid-encrypted-content-in-search-result-block">
  Invalid encrypted\_content in search\_result block
</h3>

Die API lehnte die Anfrage mit einem 400 ab, weil die Gesprächshistorie gehostete Web-Such-Inhalte enthält, die sie nicht entschlüsseln kann. Die Formulierung nennt das Feld, das sie nicht lesen kann:

```text theme={null}
API Error: 400 messages.21.content.0: Invalid `encrypted_content` in `search_result` block
API Error: 400 messages.21.content.3.citations.0: Invalid `encrypted_index` in `text` block
API Error: 400 Failed to decrypt web search result content
```

Ergebnisse aus dem gehosteten [Web-Such-Werkzeug](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool) der API enthalten verschlüsselte Felder, die nur die API lesen kann. Die API lehnt eine Anfrage ab, die Inhalte wiederholt, die sie nicht entschlüsseln kann, wie Inhalte, die für eine andere Organisation erzeugt wurden.

Das eigene [WebSearch-Werkzeug](/docs/de/tools-reference#websearch-tool-behavior) von Claude Code zeichnet Suchergebnisse als Klartext auf, daher erreichen diese Blöcke ein Gespräch normalerweise über einen Proxy oder [LLM-Gateway](/docs/de/llm-gateway), der selbst gehostete Web-Suche ausgeführt hat.

Die abgelehnten Blöcke bleiben in der Gesprächshistorie, daher schlagen jeder spätere Durchgang und `/compact` auf die gleiche Weise fehl.

**Was zu tun ist:**

* Führen Sie `/clear` aus oder starten Sie eine neue Sitzung; das neue Gespräch trägt nicht die abgelehnten Blöcke
* Wenn Sie Claude Code hinter einem Proxy oder Gateway ausführen, melden Sie den Fehler demjenigen, der ihn betreibt

<h3 id="usage-policy-refusal">
  Nutzungsrichtlinie-Ablehnung
</h3>

Die API lehnte es ab zu antworten, da Inhalte im Gespräch eine [Nutzungsrichtlinie](https://www.anthropic.com/legal/aup)-Überprüfung auslösten. Die Meldung enthält eine Anfrage-ID, die Sie dem Support zitieren können, wenn Sie glauben, dass die Ablehnung falsch ist.

```text theme={null}
API Error: Opus 4.6 can't help with this. Start a new session to continue.

Send feedback with /feedback or learn more: https://www.anthropic.com/legal/aup
```

Die Meldung nennt das Modell, das ablehnte, oder `Claude`, wenn kein Modell aufgezeichnet ist.

Die Überprüfung bewertet das gesamte Gespräch, nicht nur Ihre neueste Eingabeaufforderung, daher führt das Senden einer neuen Nachricht in derselben Sitzung normalerweise zur gleichen Ablehnung. Dasselbe gilt nach dem Beenden und erneuten Öffnen der Sitzung mit `--continue` oder `--resume`, da das Transkript auf der Festplatte immer noch den auslösenden Inhalt enthält. Auf [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Clouds Agent Platform](/docs/de/google-vertex-ai) und [Microsoft Foundry](/docs/de/microsoft-foundry) behandelt diese Meldung auch Anfragen, die die Sicherheitsmaßnahmen des Modells als Cybersicherheitsthema gekennzeichnet haben. Siehe [Sicherheitsmaßnahmen haben ein Cybersicherheitsthema gekennzeichnet](#safety-measures-flagged-a-cybersecurity-topic).

Vor v2.1.219 lautete die Meldung `Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.`

**Was zu tun ist:**

* Drücken Sie zweimal Esc oder führen Sie `/rewind` aus, um zu einem Checkpoint vor der Runde zurückzugehen, die die Ablehnung auslöste, dann formulieren Sie um oder versuchen Sie einen anderen Ansatz. Siehe [Checkpointing](/docs/de/checkpointing).
* Wenn Sie nicht identifizieren können, welche Runde es verursacht hat, führen Sie `/clear` aus, um ein neues Gespräch im selben Projekt zu starten. Ihr vorheriges Gespräch wird auf der Festplatte beibehalten und bleibt in `/resume` verfügbar.
* Im [nicht-interaktiven Modus](/docs/de/headless) (`-p`), wo Rewind nicht verfügbar ist, versuchen Sie es erneut mit einer umformulierten Eingabeaufforderung in einer neuen Sitzung ohne `--continue`. Richtlinienüberprüfungen variieren je nach Modell, daher kann ein Wechsel zu einem anderen Modell mit `--model` die Ablehnung in einigen Fällen auch beheben.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Sicherheitsmaßnahmen haben ein Cybersicherheitsthema gekennzeichnet
</h3>

Die Sicherheitsmaßnahmen des Modells haben Inhalte im Gespräch als Cybersicherheitsthema gekennzeichnet. Die Meldung nennt das Modell, das die Anfrage gekennzeichnet hat:

```text theme={null}
API Error: Opus 4.8's safeguards flagged this message. Our intentionally broad safeguards allow us to deliver more capabilities faster, but can sometimes flag legitimate cybersecurity work. Apply to the Cyber Verification Program to reduce these interruptions. Send feedback with /feedback or learn more: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude
```

Die Meldung verlinkt auf das [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), das Zugriff für legitime Cybersicherheitsarbeit gewährt. Auf Opus 5.5, das v2.1.280 oder später erfordert, beginnt die Meldung mit `Opus 5.5's safeguards flagged this session` statt. Wenn die gekennzeichnete Kategorie ein verfügbares Fallback-Modell hat, [wechselt Claude Code Modelle](/docs/de/model-config#automatic-model-fallback), statt diesen Fehler anzuzeigen.

Auf [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Clouds Agent Platform](/docs/de/google-vertex-ai) und [Microsoft Foundry](/docs/de/microsoft-foundry) erzeugt eine Cybersicherheits-Kennzeichnung stattdessen die [Nutzungsrichtlinie-Ablehnung](#usage-policy-refusal)-Meldung.

Die Sicherheitsmaßnahme selbst ist serverseitig und stammt vor v2.1.203; Client-Releases seitdem haben nur die Formulierung der Meldung geändert.
Von v2.1.203 bis v2.1.218 lautete die Meldung `<model> has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center:` gefolgt vom gleichen Help-Center-Link, und interaktive Sitzungen hängten `If you were not engaging in a cybersecurity topic, please send feedback via /feedback.` an.
Vor v2.1.203 lautete es `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` gefolgt von einem Ausnahmeantragsformular-Link.

**Was zu tun ist:**

* Wenn Ihre Arbeit diesen Inhalt erfordert, beantragen Sie Zugriff durch das [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Wenn Ihre Anfrage nicht über ein Cybersicherheitsthema war, führen Sie `/feedback` aus, um das falsch positive Ergebnis zu melden
* Um in derselben Sitzung weiterzuarbeiten, drücken Sie zweimal Esc oder führen Sie `/rewind` aus, um zu einem Checkpoint vor der Runde zurückzugehen, die die Kennzeichnung auslöste, dann versuchen Sie einen anderen Ansatz. Siehe [Checkpointing](/docs/de/checkpointing).

<h2 id="installation-errors">
  Installationsfehler
</h2>

Diese Fehler treten bei der Installation oder Aktualisierung von Claude Code auf, entweder über das [Installationsskript](/docs/de/setup#install-claude-code), `claude install` oder `claude update`. Für Probleme mit `command not found`, PATH, Berechtigungen und TLS-Fehlern während der Einrichtung siehe [Installationen und Anmeldung beheben](/docs/de/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  Installation wurde beendet, bevor sie abgeschlossen werden konnte
</h3>

Das Installationsskript meldet, wenn der `claude install`-Schritt durch ein Signal beendet wird. Unter Linux bedeutet Exit-Code 137, dass der Prozess SIGKILL erhalten hat, und auf einem Host mit wenig Speicher ist das normalerweise der Kernel-Out-of-Memory-Killer (OOM). Das Skript gibt diese Erklärung aus und beendet sich mit Code 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Für jedes andere tödliche Signal und für Exit-Code 137 auf macOS gibt das Skript `Installation was killed before it could finish (exit code <N>)` mit dem tatsächlichen Exit-Code aus und lässt die Out-of-Memory-Erklärung weg. Die Meldung stammt vom Installationsskript, das macOS und Linux verwenden, das auch Installationen innerhalb von WSL abdeckt; die nativen Windows-Installationsskripte geben es nie aus. Vor v2.1.200 beendete sich das Skript nur mit der bloßen `Killed`-Zeile der Shell.

**Was zu tun ist:**

* Beenden Sie andere Prozesse, um Speicher freizugeben, und führen Sie dann das Installationsprogramm erneut aus
* Fügen Sie Swap-Speicher hinzu oder wechseln Sie zu einer größeren Instanz. Siehe [Installation auf Linux-Servern mit wenig Speicher beendet](/docs/de/troubleshoot-install#install-killed-on-low-memory-linux-servers) für die Swap-Datei-Befehle.

<h3 id="the-connection-dropped-while-downloading-the-update">
  Die Verbindung wurde unterbrochen, während die Aktualisierung heruntergeladen wurde
</h3>

Die Verbindung zum Download-Server wurde geschlossen, während `claude install`, `claude update` oder der [automatische Updater](/docs/de/setup#auto-updates) die Claude Code-Binärdatei abrief, und die Wiederholungen konnten sich nicht erholen. Claude Code wiederholt den Download, wenn die Verbindung abbricht, die Übertragung steckenbleibt oder die heruntergeladene Datei ihre Prüfsumme nicht besteht, insgesamt bis zu drei Versuche. Ein abgeschlossener HTTP-Fehler, wie z. B. ein 404, wird nicht wiederholt, da der Server bereits geantwortet hat. Vor v2.1.202 führte eine einzelne unterbrochene Verbindung sofort zum Fehlschlag des Downloads mit dem bloßen Fehler `aborted` statt zu wiederholen.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

Der Text in Klammern nennt, welcher Versuch fehlgeschlagen ist, und den zugrunde liegenden Netzwerkfehler. `claude update` stellt der Meldung `Error: Failed to install native update` auf stderr voran.

Ein Download, der verbunden bleibt, aber nicht innerhalb von 10 Minuten abgeschlossen wird, schlägt mit `Download timed out: exceeded the total deadline` fehl. Claude Code wiederholt einen abgelaufenen Download nicht, da eine Verbindung, die zu langsam ist, um innerhalb der Frist abgeschlossen zu werden, auch bei einer sofortigen Wiederholung nicht abgeschlossen wird. Die folgenden Schritte gelten für beide Meldungen.

Die häufigste Ursache ist ein Proxy oder Gateway, das eine lange Übertragung beendet, bevor sie abgeschlossen ist. Die Claude Code-Binärdatei ist ein großer Download, daher kann eine Proxy-Verbindungsbegrenzung, die normalen API-Verkehr nie beeinflusst, sie dennoch unterbrechen.

**Was zu tun ist:**

* Führen Sie `claude update` erneut aus. Bei einem ansonsten gesunden Netzwerk ist der Download normalerweise beim nächsten Durchlauf erfolgreich. Für die Timeout-Meldung führen Sie es erneut aus einem schnelleren oder weniger gedrosselten Netzwerk aus.
* Wenn Ihr Netzwerk einen Proxy erfordert, setzen Sie `HTTPS_PROXY` vor dem Ausführen des Installationsprogramms oder `claude update`. Siehe [Netzwerkkonnektivität überprüfen](/docs/de/troubleshoot-install#check-network-connectivity).
* Wenn ein Unternehmens-Proxy die Übertragung immer wieder beendet, bitten Sie Ihr Netzwerk-Team, den vollständigen Download von `downloads.claude.ai` zuzulassen. Siehe [Netzwerkzugriffsanforderungen](/docs/de/network-config#network-access-requirements).
* Führen Sie `claude doctor` aus Ihrer Shell aus, um Installationsdiagnosen zu erhalten

<h2 id="command-line-errors">
  Befehlszeilenfehler
</h2>

Diese Fehler stammen aus dem `claude`-Befehl und seinen Unterbefehlen, aus einem Befehlsnamen, den Sie an der Eingabeaufforderung eingeben, und aus Befehlen wie `/security-review`, die Kontext durch Ausführung von Shell-Befehlen sammeln, bevor ihre Eingabeaufforderung ausgeführt wird. Sie stammen auch von `/tui`, das die CLI neu startet.

<h3 id="conflict-between-bg-and-print">
  Konflikt zwischen --bg und --print
</h3>

Diese Meldung erfordert Claude Code v2.1.198 oder später. Sie haben `--bg` mit `-p` oder `--print` in derselben `claude`-Invokation kombiniert. `--bg` startet eine [Hintergrund-Sitzung](/docs/de/agent-view#from-your-shell), die Sie später mit `claude agents` anfügen, während `--print` [nicht interaktiv](/docs/de/headless) ausgeführt wird und nie die interaktive Sitzung startet, die `claude agents` anfügt. Vor v2.1.198 hat diese Kombination stillschweigend einen Hintergrund-Job erstellt, der nie angefügt werden konnte.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**Was zu tun ist:**

* Lassen Sie `-p` oder `--print` weg. `--bg` nimmt die Eingabeaufforderung als sein Positionsargument, also ist `claude --bg "<task>"` der vollständige Befehl. Siehe [Neue Agenten von Ihrer Shell aus versenden](/docs/de/agent-view#from-your-shell).
* Um die Eingabeaufforderung nicht interaktiv auszuführen und das Ergebnis auszudrucken, anstatt eine Hintergrund-Sitzung zu erstellen, lassen Sie `--bg` weg und führen Sie `claude -p "<task>"` aus

<h3 id="invalid-agents-configuration">
  Ungültige --agents-Konfiguration
</h3>

Der Wert, den Sie an `--agents` übergeben haben, ist ungültig, daher beendet `claude` mit Code 1, anstatt die Sitzung zu starten. Wenn Sie `--safe-mode`, `--resume` oder `--continue` übergeben oder [`CLAUDE_CODE_SAFE_MODE`](/docs/de/env-vars#variables) setzen, überprüft Claude Code den Wert nicht und startet die Sitzung. Vor v2.1.242 hat Claude Code die Sitzung trotzdem gestartet und die Definitionen weggelassen, die nicht geladen werden konnten.

```text theme={null}
Error: Invalid --agents configuration:
<what failed>
```

Was nach der ersten Zeile folgt, hängt davon ab, wie der Wert fehlgeschlagen ist. Claude Code führt diese Überprüfungen der Reihe nach aus und stoppt bei der ersten, die fehlschlägt. Wenn Ihr Wert zwei Arten von Problemen hat, sehen Sie die zweite erst, nachdem Sie die erste behoben haben:

1. Wenn der Wert nicht als JSON analysiert wird, druckt Claude Code eine `invalid JSON:`-Zeile mit der eigenen Meldung des JSON-Parsers
2. Wenn er analysiert wird, aber eine Agent-Definition nicht dem Schema für [CLI-definierte Subagenten](/docs/de/sub-agents#choose-the-subagent-scope) entspricht, druckt Claude Code eine Zeile pro Problem
3. Wenn ein Agent-Name mit `-` beginnt, druckt Claude Code `<name>: agent names must not start with '-'`

Wenn es mehr als 20 Problemzeilen gibt, druckt Claude Code die ersten 20 und ersetzt den Rest mit `…and N more`.

**Was zu tun ist:**

* Beheben Sie jedes Problem, das die Meldung auflistet, und führen Sie dann den Befehl erneut aus. Siehe [die Felder, die ein CLI-definierter Subagent benötigt](/docs/de/sub-agents#choose-the-subagent-scope).

<h3 id="cloud-sessions-cannot-be-created-from-a-restricted-session">
  Cloud-Sitzungen können nicht aus einer --restricted-Sitzung erstellt werden
</h3>

Wenn Sie eine Sitzung mit [`--restricted`](/docs/de/cli-reference#cli-flags) starten, weigert sich Claude Code, [Cloud-Sitzungen](/docs/de/claude-code-on-the-web#from-terminal-to-cloud) daraus zu erstellen, da die neue Sitzung außerhalb des eingeschränkten Prozesses ausgeführt würde und den eingeschränkten Modus nicht erzwingen würde. Claude Code weigert sich auf der Client-Seite, bevor der Server kontaktiert wird, daher wird keine Cloud-Sitzung erstellt:

```text theme={null}
Cloud sessions cannot be created from a --restricted session: they would not enforce it.
```

**Was zu tun ist:**

* Führen Sie die Aufgabe lokal in der eingeschränkten Sitzung aus
* Wenn Sie kontrollieren, wie die Sitzung gestartet wurde, starten Sie eine neue `claude`-Sitzung ohne `--restricted` und erstellen Sie die Cloud-Sitzung von dort aus

Vor v2.1.248 hatte Claude Code kein `--restricted`-Flag; frühere Versionen lehnen das Flag selbst mit einem Fehler für unbekannte Optionen ab.

<h3 id="cloud-sessions-are-disabled-by-your-organizations-policy">
  Cloud-Sitzungen sind durch die Richtlinie Ihrer Organisation deaktiviert
</h3>

Die `allow_remote_sessions`-Richtlinie Ihrer Organisation ist deaktiviert, daher sind [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) und die Befehle, die sie verwenden, nicht verfügbar:

```text theme={null}
Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.
```

Die Meldung wird angezeigt, wenn Sie [eine Cloud-Sitzung vom Terminal aus erstellen](/docs/de/claude-code-on-the-web#from-terminal-to-cloud) und wenn Sie einen Befehl eingeben, der Cloud-Sitzungen benötigt, wie `/teleport`, `/remote-env` oder `/web-setup`. Vor v2.1.268 gab die Eingabe eines dieser Befehle stattdessen [`Unknown command`](#unknown-command) zurück.

Dies ist eine serverseitige Organisationsrichtlinie, daher kann sie nicht aus lokalen Einstellungen, Umgebungsvariablen oder CLI-Flags überschrieben werden.

Wenn Claude Code die Richtlinie Ihrer Organisation noch nicht geladen hat oder sie nicht abrufen kann, antworten diese Befehle stattdessen mit `Couldn't verify your organization's policy for cloud sessions. Check your network connection, then restart Claude Code and try again.`

**Was zu tun ist:**

* Bitten Sie einen [Besitzer](/docs/de/server-managed-settings#access-control) in Ihrer Organisation, Cloud-Sitzungen in den Claude Code-Administratoreinstellungen unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) zu aktivieren
* Wenn die Meldung besagt, dass die Richtlinie nicht überprüft werden konnte, überprüfen Sie Ihre Netzwerkverbindung, starten Sie Claude Code neu und versuchen Sie es erneut

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  Der --json-schema-Wert ist kein gültiges JSON-Schema
</h3>

Das Schema, das Sie an [`--json-schema`](/docs/de/cli-reference#cli-flags) in [nicht interaktivem Modus](/docs/de/headless#get-structured-output) übergeben haben, ist bei der JSON-Schema-Kompilierung fehlgeschlagen, daher beendet `claude` mit Code 1, anstatt die Eingabeaufforderung auszuführen. Vor v2.1.205 hat ein ungültiges Schema unstrukturierte Ausgabe ohne Fehler erzeugt, und jedes Schema, das das `format`-Schlüsselwort verwendet hat, wurde als ungültig behandelt.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

Der Text nach dem zweiten Doppelpunkt ist die Diagnose des Validators und benennt das Schlüsselwort oder die Position, die fehlgeschlagen ist. Schemas, die das `format`-Schlüsselwort verwenden, wie `"format": "email"`, sind gültig: Claude Code akzeptiert `format` als Anmerkung und erzwingt es nicht.

Claude Code führt zwei Überprüfungen vor der Schema-Kompilierung durch: Es lehnt einen Wert ab, der nicht als JSON analysierbar ist, mit `Error: --json-schema is not valid JSON`, und gültiges JSON, das kein Objekt ist, mit `Error: --json-schema must be a JSON object`.

**Was zu tun ist:**

* Beheben Sie den Teil des Schemas, den die Diagnose benennt, und führen Sie dann den Befehl erneut aus
* Wenn die Diagnose `schema too large` ist, reduzieren Sie die Verschachtelung und `$ref`-Wiederverwendung des Schemas
* Siehe [Strukturierte Ausgabe abrufen](/docs/de/headless#get-structured-output) für ein funktionierendes Schema und einen Befehl

<h3 id="settings-file-exceeds-the-2mib-limit">
  Einstellungsdatei überschreitet das 2-MiB-Limit
</h3>

Die Datei, die Sie an [`--settings`](/docs/de/cli-reference#cli-flags) übergeben haben, ist größer als 2 MiB, daher beendet `claude` mit Code 1 beim Start, anstatt sie zu laden. Eine Einstellungsdatei ist ein kleines JSON-Dokument, daher bedeutet eine Datei dieser Größe normalerweise, dass der Pfad auf die falsche Datei zeigt. Vor v2.1.214 hat Claude Code die Datei ohne Größenprüfung gelesen, und eine mehrere Gigabyte große Datei oder eine Gerätedatei wie `/dev/zero` hat den Speicher unbegrenzt vergrößert.

```text theme={null}
Error: Settings file exceeds the 2MiB limit: /path/to/settings.json
```

Claude Code lehnt einen `--settings`-Pfad, der keine reguläre Datei ist, auf die gleiche Weise ab: Ein Gerät, FIFO oder Socket meldet `Error: Cannot use settings file (Not a regular file (device, FIFO, or socket))` gefolgt vom Pfad, und ein Verzeichnis meldet einen `EISDIR`-Grund.

**Was zu tun ist:**

* Zeigen Sie `--settings` auf eine reguläre JSON-Einstellungsdatei unter 2 MiB. Siehe [Einstellungen](/docs/de/settings) für das Format.

<h3 id="the-current-directory-no-longer-exists">
  Das aktuelle Verzeichnis existiert nicht mehr
</h3>

Sie haben `claude` aus einem Verzeichnis gestartet, das nach dem Betreten durch Ihre Shell gelöscht oder verschoben wurde, beispielsweise ein Worktree oder ein temporäres Verzeichnis, das eine andere Shell entfernt hat. Claude Code kann sein Arbeitsverzeichnis nicht lesen, daher beendet es mit Code 1 vor dem Start der Sitzung, sowohl im interaktiven als auch im [nicht interaktiven](/docs/de/headless) Modus. Vor v2.1.239 ist Claude Code mit minifiziertem Bundle-Quellcode und einem rohen `ENOENT ... uv_cwd`-Stack auf stderr abgestürzt, anstatt diese Meldung anzuzeigen.

```text theme={null}
The current directory no longer exists (it was deleted or moved). Start Claude Code from an existing directory.
error: The current working directory was deleted, so that command didn't work. Please cd into a different directory and try again.
```

Die Ursache und die Lösung sind für beide Formen gleich.

Wenn Claude Code das Arbeitsverzeichnis aus einem anderen Grund nicht lesen kann, z. B. aufgrund einer Berechtigungsänderung, benennt die Meldung stattdessen den Fehlercode: `Can't read the current directory (EACCES). Start Claude Code from a different directory.`

Auf macOS bedeutet `EPERM` für ein Verzeichnis in `~/Desktop`, `~/Documents`, `~/Downloads` oder iCloud Drive normalerweise, dass macOS Ihre Terminal-App von diesem Ordner blockiert. Andere Befehle, die diesen Ordner lesen, schlagen auf die gleiche Weise fehl: `ls` dort meldet `Operation not permitted`, auch mit `sudo`.

**Was zu tun ist:**

* Wechseln Sie zu einem Verzeichnis, das existiert, z. B. Ihr Home- oder Projektverzeichnis, und führen Sie dann `claude` erneut aus
* Wenn das Verzeichnis unter demselben Pfad neu erstellt wurde, hält Ihre Shell immer noch das gelöschte. Führen Sie `cd "$PWD"` aus oder verlassen Sie das Verzeichnis und betreten Sie es erneut, dann führen Sie `claude` erneut aus
* Für `EPERM` auf macOS beenden Sie Ihre Terminal-App mit Cmd+Q, öffnen Sie sie erneut, kehren Sie zu diesem Ordner zurück und führen Sie `claude` aus. Wenn `ls` in diesem Ordner immer noch fehlschlägt, öffnen Sie **Systemeinstellungen > Datenschutz & Sicherheit > Dateien und Ordner**, aktivieren Sie den Ordner für Ihre Terminal-App und öffnen Sie das Terminal erneut

<h3 id="temp-directory-refused-or-cannot-be-created">
  Temp-Verzeichnis verweigert oder kann nicht erstellt werden
</h3>

Auf macOS und Linux erstellt Claude Code beim Start ein privates Temp-Verzeichnis, `claude-<uid>` unter dem System-Temp-Verzeichnis oder der [`CLAUDE_CODE_TMPDIR`](/docs/de/env-vars)-Überschreibung. Wenn das Verzeichnis nicht erstellt werden kann oder ein Eintrag, der bereits unter diesem Pfad existiert, die Sicherheitsprüfungen nicht besteht, druckt Claude Code den Fehler auf stderr und beendet sich mit Code 1, anstatt die Sitzung zu starten:

```text wrap theme={null}
ENOSPC: no space left on device, mkdir '/tmp/claude-501'

Temp directory /tmp/claude-501 is not a directory (may be an attacker-planted symlink). Refusing to use it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is owned by uid 502, expected 501. Refusing to use it — another user may have pre-created it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is not readable (its mode may have been altered, or a path component denies search). Refusing to use it — restore its permissions (chmod 0700) or remove it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.
```

**Was zu tun ist:**

* Für `ENOSPC` geben Sie Speicherplatz auf dem Volume frei, das das Temp-Verzeichnis enthält
* Für die `Refusing to use it`-Formen entfernen Sie den benannten Eintrag selbst, nicht das, auf das ein Link zeigt, und starten Sie Claude Code erneut; für die `owned by uid`-Form kann nur ein Administrator oder dieser Benutzer ihn entfernen
* Für `is not readable` führen Sie `chmod 0700` auf dem benannten Verzeichnis aus oder entfernen Sie es und starten Sie erneut
* Setzen Sie in jedem dieser Fälle [`CLAUDE_CODE_TMPDIR`](/docs/de/env-vars) auf ein Verzeichnis, das Sie kontrollieren, und starten Sie Claude Code erneut, wobei Sie den abgelehnten Pfad allein lassen

<h3 id="directory-couldnt-be-resolved-to-a-real-location">
  Verzeichnis konnte nicht zu einem echten Ort aufgelöst werden
</h3>

Sie haben `/add-dir` für ein Unterverzeichnis Ihres Arbeitsverzeichnisses ausgeführt, und Claude Code konnte das Verzeichnis nicht zu seinem echten Ort auflösen.

Sie haben bereits Dateizugriff auf ein Unterverzeichnis des Arbeitsverzeichnisses, daher lädt `/add-dir` nur seine Skills, Befehle und Agenten. Bevor sie geladen werden, überprüft Claude Code, dass der echte Ort des Verzeichnisses, mit aufgelösten Symlinks, sich im Arbeitsverzeichnis befindet. Wenn Claude Code diesen Ort nicht auflösen kann, lädt es nichts und zeigt diese Meldung:

```text theme={null}
packages/app couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded. Check that it is a directory inside the working directory and try again.
```

**Was zu tun ist:**

* Überprüfen Sie, dass der Pfad ein echtes Verzeichnis im Arbeitsverzeichnis benennt, und führen Sie dann `/add-dir` erneut aus
* Die Meldung ändert Ihren Dateizugriff nicht; sie meldet nur, dass der `.claude/`-Inhalt des Verzeichnisses nicht geladen wurde

Vor v2.1.261 erschien diese Meldung auch für jeden `/add-dir <subdirectory>`, wenn das Arbeitsverzeichnis auf einem `/net/<host>`-Automount war, wo Claude Code sich weigert, Pfade nach Design aufzulösen; das Verzeichnis war in Ordnung und ein erneuter Versuch konnte nicht helfen.

<h3 id="workspace-not-trusted-when-starting-remote-control">
  Arbeitsbereich nicht vertraut beim Starten der Remote-Steuerung
</h3>

Sie haben den [Remote-Steuerungs](/docs/de/remote-control)-Servermodus mit `claude remote-control` oder seinem `claude rc`-Alias in einem Verzeichnis gestartet, das Sie nicht vertraut haben. Der Befehl zeigt den Dialog zur Arbeitsbereichsvertrauenswürdigkeit selbst nicht an, daher beendet er mit Code 1 und benennt die Lösung:

```text theme={null}
Error: Workspace not trusted. Please run `claude` in /Users/you/project first to review and accept the workspace trust dialog.
```

In Ihrem Home-Verzeichnis ist die Meldung anders, da der Dialog zur Arbeitsbereichsvertrauenswürdigkeit nie Vertrauen für das Home-Verzeichnis speichert, daher kann das Akzeptieren dort diese Überprüfung nicht erfüllen. Vor v2.1.214 zeigte das Home-Verzeichnis die obige Meldung, deren Rat dort nicht erfolgreich sein kann.

```text theme={null}
Error: Workspace not trusted. /Users/you is your home directory, and for security home-directory trust is never saved, so running `claude` here first won't help. Run `claude rc` from a project directory instead (run `claude` there once to accept the trust dialog).
```

**Was zu tun ist:**

* Führen Sie `claude` im Verzeichnis aus, akzeptieren Sie den [Dialog zur Arbeitsbereichsvertrauenswürdigkeit](/docs/de/permissions#project-allow-rules-and-workspace-trust), und führen Sie dann `claude remote-control` erneut aus
* Wechseln Sie in Ihrem Home-Verzeichnis zu einem Projektverzeichnis und starten Sie die Remote-Steuerung dort

<h3 id="not-carried-over-to-the-sessions-remote-control-starts">
  Nicht übertragen auf die Sitzungen, die Remote-Steuerung startet
</h3>

Sie haben [Remote-Steuerung](/docs/de/remote-control) mit einem globalen `claude`-Flag vor dem `remote-control`-Verb gestartet, eines, das die Sitzungen einschränken oder konfigurieren würde, die Remote-Steuerung startet, wie `--settings`, `--setting-sources`, `--permission-mode`, `--disallowed-tools` oder `--mcp-config`. Ein Flag, das vor dem Verb platziert ist, erreicht diese Sitzungen nie. Claude Code weigert sich zu starten, anstatt es zu starten, und benennt das Flag:

```text theme={null}
Error: `--settings` before `remote-control` is not carried over to the sessions Remote Control starts, so Remote Control refuses to start rather than drop it — remove it, and give Remote Control's own options after the verb (see `claude remote-control --help`).
```

Claude Code weigert sich nicht bei globalen Flags, die harmlos zu löschen sind, wie `--verbose`, `--model` oder ein von einem Wrapper injiziertes `--session-id` oder `--plugin-dir`: Es ignoriert sie und Remote-Steuerung startet.

Claude Code weigert sich auch zu starten für ein globales Flag, das es noch nicht als harmlos erkennt, daher kann ein Flag, das in einer neueren Version hinzugefügt wurde, in dieser Meldung erscheinen, bis eine spätere Version es als harmlos markiert.

**Was zu tun ist:**

* Entfernen Sie das Flag vor dem Verb und übergeben Sie [Remote-Steuerungs eigene Optionen](/docs/de/remote-control#start-a-remote-control-session) danach; `claude remote-control --help` listet sie auf
* Wenn das abgelehnte Flag `--permission-mode` ist, führen Sie `claude remote-control --permission-mode <mode>` aus, um den Berechtigungsmodus für die Sitzungen festzulegen, die Remote-Steuerung startet

Vor v2.1.248 akzeptierte `claude remote-control` seine eigenen Flags nicht, wenn ein globales Flag zuerst kam, und der Befehl schlug mit einem Fehler für unbekannte Optionen fehl.

<h3 id="claude-import-is-not-yet-available-in-this-build">
  claude import ist in diesem Build noch nicht verfügbar
</h3>

Sie haben [`claude import`](/docs/de/cli-reference#cli-commands) ausgeführt, und Claude Code hat den Import-Flow ausgeschaltet gefunden, daher beendet der Befehl mit Code 1, anstatt den Import zu starten. Vor v2.1.222 hat ein Build mit ausgeschaltetem Import-Flow `import` als Eingabeaufforderung behandelt und eine interaktive Sitzung gestartet, anstatt diese Meldung zu drucken.

```text theme={null}
`claude import` is not yet available in this build. Run `claude` and use /mcp or edit ~/.claude/settings.json directly.
```

Claude Code schaltet `claude import` durch ein Feature-Flag ein, das es von Anthropic abruft und auf der Festplatte zwischenspeichert. Diese Meldung bedeutet, dass der zwischengespeicherte Wert ausgeschaltet ist. Die Ursache ist normalerweise eine der folgenden:

* Sie haben seit der Installation keine Sitzung gestartet, daher hat Claude Code das Flag noch nicht abgerufen. Der erste `claude import` kann dies drucken, auch wenn die Funktion für Sie verfügbar ist.
* Sie verwenden Claude Code über Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder Claude Platform auf AWS, oder über ein [Claude-Apps-Gateway](/docs/de/claude-apps-gateway#availability-and-limitations). Claude Code ruft Feature-Flags in diesen Sitzungen nicht ab, daher bleibt `claude import` nicht verfügbar.
* Sie haben `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `DISABLE_GROWTHBOOK` oder [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/de/env-vars) gesetzt, was das Abrufen von Feature-Flags deaktiviert, daher bleibt `claude import` nicht verfügbar.

**Was zu tun ist:**

* Starten Sie bei einer Neuinstallation `claude`, warten Sie, bis die Sitzung geladen ist, beenden Sie sie, und führen Sie `claude import` erneut aus
* Wo das Abrufen von Feature-Flags ausgeschaltet bleibt, richten Sie die Konfiguration selbst ein: Fügen Sie MCP-Server mit [`claude mcp add`](/docs/de/mcp#installing-mcp-servers) hinzu, und erstellen Sie die [`CLAUDE.md`-Dateien](/docs/de/memory#how-claude-md-files-load), [Skills und Befehle](/docs/de/skills#where-skills-live) und [Subagenten](/docs/de/sub-agents#choose-the-subagent-scope), die Sie übertragen möchten. Die Meldung benennt auch `~/.claude/settings.json`. Von der Konfiguration, die `claude import` überträgt, enthält diese Datei nur den [Berechtigungsmodus](/docs/de/settings-reference#permission-settings); Claude Code liest MCP-Server nicht daraus.

<h3 id="could-not-read-claude-code-config">
  Claude Code-Konfiguration konnte nicht gelesen werden
</h3>

Sie haben [`claude import`](/docs/de/cli-reference#cli-commands) ausgeführt, während Claude Code `~/.claude.json` nicht analysieren konnte, die Datei, in der es Ihren Login und den projektspezifischen Status speichert. Der Unterbefehl liest diese Datei, um die Verfügbarkeit zu überprüfen, zeigt aber nicht den Wiederherstellungsdialog an, den die interaktive Sitzung zeigt, daher beendet er mit Code 1. Vor v2.1.222 hat `claude import` mit einer nicht lesbaren Konfigurationsdatei eine interaktive Sitzung gestartet, deren Wiederherstellungsdialog die Datei behandelt hat.

```text theme={null}
Could not read Claude Code config — run `claude` with no arguments to recover it.
```

**Was zu tun ist:**

* Führen Sie `claude` ohne Argumente aus. Claude Code erkennt die ungültige Datei und bietet an, sie zurückzusetzen. Führen Sie dann `claude import` erneut aus.
* Um manuelle Änderungen zu behalten, die Sie vorgenommen haben, beheben Sie die JSON-Syntax in `~/.claude.json` in einem Editor und führen Sie dann `claude import` erneut aus

<h3 id="could-not-import-a-server-from-claude-desktop">
  Server konnte nicht aus Claude Desktop importiert werden
</h3>

Claude Code konnte einen der Server, die Sie in `claude mcp add-from-claude-desktop` ausgewählt haben, nicht hinzufügen. Der Befehl importiert immer noch die anderen ausgewählten Server und druckt eine Zeile pro Server, den er nicht hinzufügen konnte. Vor v2.1.205 hat der erste Server, der fehlgeschlagen ist, den Import gestoppt und keiner der ausgewählten Server wurde hinzugefügt.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

Der Text nach dem Servernamen ist der Grund. Der häufigste ist die Namensüberprüfung: Claude Desktop erlaubt Zeichen in Servernamen, wie Leerzeichen und Punkte, die `claude mcp` auf Buchstaben, Zahlen, Bindestriche und Unterstriche beschränkt. Andere Gründe sind eine Serverkonfiguration, die die Validierung nicht besteht, und ein Server, der durch die [MCP-Richtlinie](/docs/de/managed-mcp) Ihrer Organisation blockiert wird.

**Was zu tun ist:**

* Benennen Sie den Server in `claude_desktop_config.json` um, um nur Buchstaben, Zahlen, Bindestriche und Unterstriche zu verwenden, und führen Sie dann `claude mcp add-from-claude-desktop` erneut aus
* Fügen Sie diesen Server direkt mit `claude mcp add` oder `claude mcp add-json` unter einem gültigen Namen hinzu. Siehe [MCP-Server aus Claude Desktop importieren](/docs/de/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="cannot-add-mcp-server-to-the-managed-scope">
  MCP-Server kann nicht zum verwalteten Bereich hinzugefügt werden
</h3>

Sie haben `claude mcp add` oder `claude mcp add-json` mit `--scope managed` ausgeführt. Dieser Bereich enthält die Server, die Ihre Organisation über die verwaltete Einstellung [`managedMcpServers`](/docs/de/settings-reference#managedmcpservers) bereitstellt. Claude Code liest sie nur aus verwalteten Einstellungen, daher kann der Befehl keinen Server in diesen Bereich schreiben.

```text theme={null}
Cannot add MCP server to scope: managed
```

**Was zu tun ist:**

* Fügen Sie den Server zu einem Bereich hinzu, in den Sie schreiben können: `local`, `user` oder `project`. Ohne `--scope` verwendet der Befehl `local`. Siehe [MCP-Installationsbereiche](/docs/de/mcp#mcp-installation-scopes)
* Um den Server jedem Benutzer in Ihrer Organisation bereitzustellen, fügen Sie ihn zu [`managedMcpServers`](/docs/de/settings-reference#managedmcpservers) in den verwalteten Einstellungen hinzu, die Sie bereitstellen

<h3 id="cant-read-mcp-json">
  .mcp.json kann nicht gelesen werden
</h3>

Ein Befehl, der das Projekt [`.mcp.json`](/docs/de/mcp#project-scope) liest, wie `claude mcp add` oder `claude mcp add-json` mit `--scope project` oder `claude mcp remove`, hat festgestellt, dass die Datei in Ihrem aktuellen Verzeichnis keine reguläre Datei ist oder größer als 2 MiB ist, daher beendet er mit diesem Fehler, anstatt die Datei zu lesen.

```text theme={null}
Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes. Fix or remove it, then run the command again.
```

Vor v2.1.257 ließ ein FIFO bei `.mcp.json` den Befehl für immer ohne Ausgabe warten, und ein Symlink zu einer Gerätedatei wie `/dev/zero` vergrößerte den Speicher, bis der Prozess beendet wurde.

**Was zu tun ist:**

* Überprüfen Sie, was sich bei `.mcp.json` in Ihrem aktuellen Verzeichnis befindet. Ersetzen Sie es durch eine gewöhnliche JSON-Datei im [Projektbereich-Format](/docs/de/mcp#project-scope) oder löschen Sie es, und führen Sie dann den Befehl erneut aus.

<h3 id="anthropic-hosted-and-doesnt-support-local-oauth">
  Server ist von Anthropic gehostet und unterstützt lokales OAuth nicht
</h3>

Sie haben eine Anmeldung für einen MCP-Server gestartet, dessen URL auf einen von Anthropic gehosteten Connector-Host zeigt, der sich über einen Drittanbieter-Identitätsanbieter authentifiziert. Diese Hosts umfassen `microsoft365.mcp.claude.com`, `gmail.mcp.claude.com` und `gcal.mcp.claude.com`. Claude Code weigert sich, seinen lokalen OAuth-Flow für diese Hosts sowohl vom `/mcp`-Panel als auch von `claude mcp login` zu starten, da [ihre Anmeldung nur über claude.ai funktioniert](/docs/de/mcp#use-mcp-servers-from-claude-ai).

```text theme={null}
"gmail" is Anthropic-hosted and doesn't support local OAuth. Connect it via Settings → Connectors on claude.ai (requires `claude login`), then it'll be available here automatically.
```

Claude Code gleicht diese Hosts nach URL ab, daher erscheint die Meldung, wenn ein Server, den Sie mit `claude mcp add` oder in `.mcp.json` hinzugefügt haben, auf einen von ihnen zeigt.

**Was zu tun ist:**

* Entfernen Sie Ihren Eintrag mit `claude mcp remove <name>`, damit er den claude.ai-Connector unter derselben URL nicht verbergen kann
* Nachdem Sie ihn entfernt haben, verbinden Sie den Service unter [claude.ai/customize/connectors](https://claude.ai/customize/connectors), während Sie bei dem Konto angemeldet sind, das Sie in Claude Code verwenden. Nach der Verbindung [erscheint der Connector automatisch in Claude Code](/docs/de/mcp#use-mcp-servers-from-claude-ai), wenn Ihre aktive Authentifizierungsmethode eine claude.ai-Abonnement-Anmeldung ist

<h3 id="server-rejected-the-authorization-header-minted-by-the-configured-headershelper">
  Server lehnte den vom konfigurierten headersHelper erstellten Authorization-Header ab
</h3>

Ein MCP-Server, dessen [`headersHelper`](/docs/de/mcp#use-dynamic-headers-for-custom-authentication) den `Authorization`-Header bereitstellt, hat die Verbindung mit HTTP 401 oder 403 beantwortet, daher meldet Claude Code die Verbindung als fehlgeschlagen. Da der Helper den `Authorization`-Header bereitstellt, [fällt Claude Code nicht auf OAuth zurück](/docs/de/mcp#authenticate-with-remote-mcp-servers) für den Server:

```text theme={null}
Server rejected the Authorization header minted by the configured headersHelper (HTTP 401). Check that the helper command returns a valid credential for this MCP endpoint — OAuth fallback is disabled when the helper supplies Authorization.
```

Claude Code führt den Helper bei jedem Verbindungsversuch erneut aus, daher kann ein erneuter Versuch nach einer vorübergehenden Ablehnung, wie einer Token-Rotations-Rasse, mit einer frischen Anmeldeinformation erfolgreich sein.

**Was zu tun ist:**

* Führen Sie den `headersHelper`-Befehl selbst so aus, wie Claude Code ihn ausführt: aus dem [Verzeichnis, in dem Claude Code ihn ausführt](/docs/de/mcp#where-the-helper-runs), mit den [Umgebungsvariablen, die Claude Code für ihn setzt](/docs/de/mcp#use-dynamic-headers-for-custom-authentication), und ohne die [Anmeldeinformationsvariablen, die Claude Code entfernt](/docs/de/mcp#which-variables-a-helper-can-read) für einen Server aus einer Projekt `.mcp.json`, einem Plugin oder einer Projekt-Agent-Datei. Überprüfen Sie, dass er einen `Authorization`-Wert druckt, den der Endpunkt des Servers akzeptiert
* Nachdem Sie den Helper oder seine Anmeldeinformationsquelle behoben haben, wählen Sie den Server in `/mcp` aus und wählen Sie **Reconnect**

Vor v2.1.248 führte Claude Code OAuth-Erkennung für einen Server durch, dessen Helper den `Authorization`-Header bereitstellte. Diese Erkennung konnte mit `Incompatible auth server: does not support dynamic client registration` fehlschlagen, anstatt die abgelehnte Anmeldeinformation zu melden.

<h3 id="mcp-permission-prompt-tool-not-found">
  MCP-Berechtigungsaufforderungs-Tool nicht gefunden
</h3>

Das Tool, das Sie an [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags) übergeben haben, war nicht unter den verbundenen MCP-Tools, als der Lauf zuerst eine Berechtigungsentscheidung benötigte, entweder weil sein Server nie verbunden wurde oder weil kein verbundener Server ein Tool mit diesem Namen verfügbar macht. Claude Code sendet Ihre Eingabeaufforderung immer noch: Der [nicht interaktive](/docs/de/headless) Lauf beendet sich mit diesem Fehler und Exit-Code 1 beim ersten Tool-Aufruf, der Genehmigung benötigt, daher erzeugt er keine Antwort, obwohl die Anfrage gestellt wurde. Vor dem ersten Prompt wartet Claude Code bis zu dem pro-Server-Verbindungs-Timeout von 30 Sekunden, der durch [`MCP_TIMEOUT`](/docs/de/env-vars) gesetzt ist, damit dieser Server verbunden wird. Vor v2.1.206 wartete der Start nicht, bis der Server die Verbindung beendete, daher erzeugte ein langsam startender, aber gesunder Server auch diesen Fehler.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

Die Liste nach `Available MCP tools:` benennt die MCP-Tools, die verbunden waren, als das Warten endete.

**Was zu tun ist:**

* Überprüfen Sie, dass der Server startet und verbunden bleibt: Führen Sie `claude mcp list` im selben Verzeichnis aus und bestätigen Sie, dass der Server als verbunden aufgelistet ist
* Bestätigen Sie, dass der Tool-Name dem `mcp__<server>__<tool>`-Namen entspricht, den der Server verfügbar macht
* Wenn der Server länger als 30 Sekunden zum Starten benötigt, erhöhen Sie [`MCP_TIMEOUT`](/docs/de/env-vars)

<h3 id="oauth-callback-port-is-already-in-use">
  OAuth-Callback-Port wird bereits verwendet
</h3>

Wenn Sie sich bei einem Remote-MCP-Server mit OAuth anmelden, startet Claude Code einen lokalen Listener, um den Anmelde-Callback zu empfangen. Wenn der Port, den dieser Listener benötigt, von einem anderen Prozess gehalten wird, schlägt die Anmeldung mit dieser Meldung fehl. Dies geschieht hauptsächlich mit einem [festen Callback-Port](/docs/de/mcp#use-a-fixed-oauth-callback-port), der durch die Variable [`MCP_OAUTH_CALLBACK_PORT`](/docs/de/env-vars) oder `--callback-port` gesetzt ist, da Claude Code ohne einen verfügbaren Port auswählt.

```text theme={null}
OAuth callback port <port> is already in use — another process may be holding it. Run `lsof -ti:<port> -sTCP:LISTEN` to find it.
```

Unter Windows ist der vorgeschlagene Befehl stattdessen `netstat -ano | findstr :<port>`.

**Was zu tun ist:**

* Führen Sie den Befehl aus der Meldung aus, um den Prozess zu finden, der den Port hält, und stoppen Sie ihn oder warten Sie, bis er fertig ist
* Wenn ein anderes Programm diesen Port dauerhaft benötigt, registrieren Sie einen anderen Redirect-URI beim Server und setzen Sie seinen Port mit `MCP_OAUTH_CALLBACK_PORT` oder `--callback-port`, je nachdem, was Sie verwenden
* Starten Sie dann die Anmeldung erneut, z. B. indem Sie den Server in `/mcp` auswählen

<h3 id="no-available-ports-for-oauth-redirect">
  Keine verfügbaren Ports für OAuth-Umleitung
</h3>

Wenn Sie sich bei einem Remote-MCP-Server mit [OAuth](/docs/de/mcp#authenticate-with-remote-mcp-servers) anmelden, startet Claude Code einen lokalen Listener, um den Anmelde-Callback zu empfangen. Die Anmeldung schlägt mit dieser Meldung fehl, wenn Claude Code keinen lokalen Port dafür binden kann. Etwas auf der Maschine verhindert, dass es auf `127.0.0.1` lauscht, z. B. Sicherheitssoftware oder eine Sandbox-Richtlinie, die lokale Listener verweigert.

```text theme={null}
No available ports for OAuth redirect
```

Vor v2.1.268 fiel Claude Code nicht auf einen vom Betriebssystem zugewiesenen Port zurück, daher erschien die Meldung auch, wenn nur die selbst ausgewählten Ports nicht gebunden werden konnten. Dies kann auf Windows-Hosts geschehen, wo Hyper-V Port-Bereiche reserviert, die die Ports abdecken, die Claude Code auswählt.

**Was zu tun ist:**

* Überprüfen Sie, ob Sicherheitssoftware oder eine Sandbox-Richtlinie Prozesse daran hindert, auf `127.0.0.1` zu lauschen, und erlauben Sie Claude Code, einen lokalen Port zu binden
* Starten Sie dann die Anmeldung erneut, z. B. indem Sie den Server in `/mcp` auswählen

<h3 id="security-review-fails-without-origin-head">
  /security-review schlägt ohne origin/HEAD fehl
</h3>

[`/security-review`](/docs/de/commands#all-commands) erstellt seinen Review-Kontext, indem es Ihren Branch gegen `origin/HEAD` vergleicht, die lokale Ref, die aufzeichnet, welcher Branch der Standard auf Ihrem `origin`-Remote ist. Wenn diese Ref nicht existiert, schlagen die Git-Befehle, die den Diff sammeln, fehl und die Review stoppt, bevor sie beginnt.

```text theme={null}
Error: Shell command failed for pattern "!`git diff --name-only origin/HEAD...`": [stderr]
fatal: ambiguous argument 'origin/HEAD...': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
```

Die Meldung kann stattdessen `git log` oder einen anderen `git diff` zitieren. Git erstellt `origin/HEAD` nur, wenn der Remote einen Standard-Branch bewirbt und Ihr Fetch-Refspec ihn abdeckt, was ein vollständiger `git clone` eines Remote mit Commits tut. Die Ref fehlt in diesen Setups:

* Ein Single-Branch- oder CI-Checkout, der einen zu engen Refspec abruft
* Ein Remote, dessen serverseitige HEAD auf einen Branch zeigt, den niemand gepusht hat
* Ein Repository ohne `origin`-Remote oder eines, das Sie nie abgerufen haben

Claude Code zeigt denselben Fehler für jeden Skill, der [dynamischen Kontext injiziert](/docs/de/skills#when-an-injected-command-fails), und ein fehlgeschlagener injizierter Befehl bricht die Invokation dieses Skills ab. Zwei Geschwister-Strings werden vor dem Befehl ausgelöst:

* `Shell command permission check failed for pattern "..."`: Die Berechtigungsprüfung des Befehls hat ihn nicht erlaubt. [Berechtigungsprüfungen auf injizierten Befehlen](/docs/de/skills#permission-checks-on-injected-commands) behandelt, welche Ergebnisse in jedem Berechtigungsmodus abbrechen und wie Sie einen Befehl mit `allowed-tools` vorab genehmigen
* ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``: Die Frontmatter des Skills verlangt Bash auf einer Maschine ohne ihn. Installieren Sie Git für Windows oder ändern Sie die Frontmatter zu `shell: powershell`. Siehe [Wie injizierte Befehle ausgeführt werden](/docs/de/skills#how-injected-commands-run)

**Was zu tun ist:**

* Erstellen Sie die Ref, indem Sie den Standard-Branch Ihres Remote benennen: `git remote set-head origin <default-branch>`. Dies funktioniert, wenn die lokale Tracking-Ref `origin/<default-branch>` existiert. Wenn nicht, wie in Single-Branch-Klonen, rufen Sie den Branch zuerst ab: Führen Sie `git remote set-branches --add origin <branch>` aus, dann `git fetch origin`, dann führen Sie den set-head-Befehl erneut aus. Führen Sie `/security-review` erneut aus.
* Wenn Sie den Branch nicht benennen möchten, führen Sie `git fetch origin` aus und dann `git remote set-head origin --auto`, das den Remote fragt, welcher Branch sein Standard ist. Es schlägt mit `error: Cannot determine remote HEAD` fehl, wenn der Remote keinen Standard-Branch bewirbt, weil er leer ist oder seine HEAD auf einen Branch zeigt, den niemand gepusht hat; benennen Sie den Branch stattdessen explizit. Es schlägt mit `error: Not a valid ref` fehl, wenn Ihr Klon diesen Branch nicht abruft; erweitern Sie den Refspec zuerst wie oben.
* Wenn das Repository keinen Remote hat, fügen Sie einen mit `git remote add origin <url>` hinzu und rufen Sie ab, bevor Sie die Ref erstellen. Wenn der Remote leer ist, pushen Sie Ihren Branch zuerst mit `git push -u origin HEAD` und benennen Sie diesen Branch im set-head-Befehl; `origin/HEAD` zeigt dann auf den Branch, den Sie gerade gepusht haben, daher sieht `/security-review` einen leeren Diff, bis der Branch von ihm abweicht.

<h3 id="input-must-be-provided-when-using-print">
  Eingabe muss bereitgestellt werden, wenn --print verwendet wird
</h3>

Bare `claude` benötigt stdout, um ein Terminal zu sein, um die interaktive UI zu starten. Wenn stdout umgeleitet wird oder die Konsole kein echtes Terminal ist, wie PowerShell ISE und einige IDE-Ausgabebereiche, führt `claude` stattdessen [nicht interaktiv](/docs/de/headless) aus. Das ist derselbe Modus wie `claude -p`, der eine Eingabeaufforderung erfordert, daher benennt die Meldung `--print`, auch wenn Sie das Flag nicht übergeben haben. Das Übergeben von `-p`/`--print` ohne Eingabeaufforderung und nichts auf stdin erzeugt überall denselben Fehler.

```text theme={null}
Error: Input must be provided either through stdin or as a prompt argument when using --print
```

**Was zu tun ist:**

* Für interaktive Verwendung führen Sie `claude` in einem echten Terminal aus: Windows Terminal oder die PowerShell-Konsole statt ISE, und das integrierte Terminal Ihrer IDE statt eines Ausgabebereichs
* Für einmalige Verwendung übergeben Sie die Eingabeaufforderung: `claude -p "your question"`, oder pipen Sie sie mit `echo "your question" | claude -p`

<h3 id="input-contained-only-whitespace">
  Eingabe enthielt nur Leerzeichen
</h3>

Im [nicht interaktiven Modus](/docs/de/headless) weigert sich Claude Code, eine Eingabeaufforderung zu akzeptieren, die nur aus Leerzeichen, Tabulatoren oder Zeilenumbrüchen besteht, anstatt sie zu senden, da die API Nachrichten ohne sichtbaren Text ablehnt. Welche Meldung Sie sehen, hängt davon ab, woher die leere Eingabeaufforderung kam:

* **Eingabeaufforderungs-Argument oder gepipte stdin für `claude -p`**: `claude` beendet sich mit `Error: Input contained only whitespace. Provide a prompt with text through stdin or as a prompt argument when using --print`
* **Nachricht eingereicht an eine laufende `--input-format stream-json` oder [Agent SDK](/docs/de/agent-sdk/overview)-Sitzung**: Claude Code beendet die Runde ohne Aufruf des Modells und die Sitzung bleibt nutzbar. Die Ablehnung kommt als informative Meldung und als Ergebnis-Text der Runde: `Blank prompt — the message was only whitespace, so nothing was sent to the model.`

Vor v2.1.229 hat Claude Code die Whitespace-only-Nachricht an die API gesendet, die die Anfrage mit einem 400-Fehler abgelehnt hat.

**Was zu tun ist:**

* Fügen Sie sichtbaren Text in die Eingabeaufforderung ein. Wenn ein Skript die Eingabeaufforderung aus einer Variablen oder Datei erstellt, überprüfen Sie, dass die Quelle nicht leer ist, bevor Sie Claude Code aufrufen.

<h3 id="stream-json-input-carried-over-256m-characters-with-no-newline">
  stream-json-Eingabe trug über 256M Zeichen ohne Zeilenumbruch
</h3>

Ihr Programm hat mehr als 268.435.456 Zeichen auf stdin ohne Zeilenumbruch an einen `claude -p --input-format stream-json`-Lauf gesendet, daher druckt Claude Code diesen Fehler auf stderr und beendet sich mit Code 1, anstatt mehr Eingabe zu puffern. Die Meldung gibt dieses Budget als `256M` an. Vor v2.1.257 hat Claude Code solche Eingaben ohne Limit gepuffert, was den Speicher vergrößert hat, bis der Prozess abstürzte oder beendet wurde.

```text theme={null}
Error: stream-json input carried over 256M characters with no newline. Each stream-json message must be a single newline-terminated JSON line: either the producer is not newline-terminating its messages, or one message exceeded this budget.
```

Eingabe dieser Länge ohne Zeilenumbruch bedeutet normalerweise, dass der Producer überhaupt kein stream-json-Producer ist, wie eine Binärdatei oder einfache Log-Ausgabe, die versehentlich gepipet wird. Eine einzelne Nachricht über dem Budget schlägt dieselbe Überprüfung fehl.

**Was zu tun ist:**

* Überprüfen Sie, was auf stdin gepipet wird. Mit [`--input-format stream-json`](/docs/de/cli-reference#cli-flags) muss jede Nachricht eine Zeilenumbruch-terminierte JSON-Zeile sein
* Um stattdessen einfachen Text zu senden, lassen Sie `--input-format stream-json` weg; `claude -p` liest standardmäßig eine einfache Text-Eingabeaufforderung von stdin

<h3 id="unknown-command">
  Unbekannter Befehl
</h3>

In einer interaktiven Terminal-Sitzung haben Sie einen `/`-Namen eingereicht, der keinem Befehl in dieser Sitzung entspricht, daher meldet Claude Code den Namen, anstatt etwas auszuführen:

```text theme={null}
Unknown command: /hepl. Did you mean /help?
```

Claude Code schlägt den nächsten Befehlsnamen oder Alias vor, den das Menü in dieser Sitzung auflistet. Wenn nichts nah ist, endet die Meldung nach dem Namen. Die Ursache ist normalerweise eine der folgenden:

* Ein Tippfehler, wie `/hepl` für `/help`. [Wie das Befehlsmenü das, was Sie eingeben, abgleicht](/docs/de/commands#how-the-command-menu-matches-what-you-type) behandelt das Auswählen einer nahen Übereinstimmung, bevor Sie einreichen
* Ein Befehl, der existiert, aber in dieser Sitzung nicht verfügbar ist, weil eine Anforderung nicht erfüllt ist, wie Ihre Plattform, Ihr Plan oder Ihre Authentifizierungsmethode. Die Troubleshooting-Einträge für [`/web-setup`](/docs/de/web-quickstart#web-setup-shows-no-commands-match-or-unknown-command) und [`/schedule`](/docs/de/routines#schedule-returns-unknown-command) gehen durch zwei häufige Fälle. Einige Befehle antworten mit ihrer eigenen Meldung, wenn die Richtlinie Ihrer Organisation sie deaktiviert, wie [`Cloud sessions are disabled by your organization's policy`](#cloud-sessions-are-disabled-by-your-organizations-policy)
* Ein Befehl aus einem [Plugin](/docs/de/plugins/overview) oder [MCP-Server](/docs/de/mcp#use-mcp-prompts-as-commands), der in dieser Sitzung nicht installiert oder verbunden ist

Claude Code antwortet auf einen nicht übereinstimmenden `/`-Namen auf diese Weise nur in einer interaktiven Terminal-Sitzung. In jeder anderen Sitzung sendet es die Eingabeaufforderung als normale Nachricht an Claude, mit einer Notiz, dass der Befehl nicht ausgeführt wurde, und einer Liste von Befehlen, die Claude in der Sitzung ausführen kann. Diese Sitzungen umfassen:

* `-p`-Läufe
* [Agent SDK](/docs/de/agent-sdk/overview)-Anwendungen
* Die Code-Registerkarte der [Desktop-App](/docs/de/desktop)
* Das Chat-Panel der [VS Code-Erweiterung](/docs/de/vs-code)
* [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) und [Routinen](/docs/de/routines)

Für einen integrierten Befehl, der in einer dieser Sitzungen nicht ausgeführt werden kann, antwortet Claude Code immer noch, dass der Befehl nicht verfügbar ist, anstatt ihn an Claude zu senden. Vor v2.1.274 sendeten nur Cloud-Sitzungen und Routinen einen nicht übereinstimmenden Namen an Claude. Vor v2.1.273 antworteten sie auch mit `Unknown command`.

Claude Code behandelt nicht jeden Prompt, der mit `/` beginnt, als Befehl. Es sendet die Eingabeaufforderung als normale Nachricht an Claude, wenn das erste Wort nach dem `/` mit Interpunktion beginnt, wie das `/--`, das einen Lean-Doc-Kommentar öffnet, oder ein Pfad wie `/var/log/syslog`.

Vor v2.1.236 führte Claude Code, wenn Sie die Eingabetaste drückten, während das Befehlsmenü eine nahe Übereinstimmung für den Namen auflistete, den Sie eingegeben haben, diese Übereinstimmung aus, daher führte ein Tippfehler wie `/hepl` `/help` aus, anstatt diese Meldung zu erzeugen.

**Was zu tun ist:**

* Führen Sie den vorgeschlagenen Namen aus, oder geben Sie `/` gefolgt von Teil des Namens ein, um zu sehen, was in dieser Sitzung verfügbar ist
* Wenn Claude Code einen dokumentierten Befehl als unbekannt meldet, überprüfen Sie seine Zeile in der [Befehlsreferenz](/docs/de/commands) auf die Anforderung, die er benennt

<h3 id="diff-is-too-large-for-ultrareview">
  Diff ist zu groß für ultrareview
</h3>

Der Diff zwischen Ihrem Branch und dem Base-Branch, einschließlich nicht committeter und gestaffelter Änderungen, überschreitet die Größenlimits für eine [ultrareview](/docs/de/ultrareview), daher weigern sich `/code-review ultra` und der `claude ultrareview`-Unterbefehl die Review, bevor die Cloud-Sitzung startet. Eine abgelehnte Review verwendet keinen kostenlosen Lauf und belastet keine Nutzungsguthaben. Die Meldung benennt die geltenden Limits, die Größe Ihres Diff und die Dateien, die die meisten geänderten Zeilen beitragen. Vor v2.1.216 zeigte die Meldung nur die rohen Diff-Statistiken.

```text theme={null}
Diff is too large for ultrareview: 812 files, 96,410 lines changed (limits: 500 files, 8,000 lines). Largest files: package-lock.json (41,904 lines), dist/bundle.js (18,210 lines), src/generated/api.ts (9,876 lines). Pass a closer base branch (`/code-review ultra <branch>`) to narrow the scope, or split the change.
```

Das Überprüfen eines Pull Request wendet dieselben Limits an; diese Form der Meldung beginnt mit `PR #<N> is too large for ultrareview` und benennt die Datei und Zeilenzahlen des PR.

**Was zu tun ist:**

* Übergeben Sie einen Base-Branch, der näher an Ihrer Arbeit liegt, wie `/code-review ultra develop`, damit die Review nur den Diff gegen diesen Branch abdeckt
* Teilen Sie die Änderung in kleinere Branches auf und überprüfen Sie jeden. Die Dateien, die die Meldung benennt, tragen die meisten geänderten Zeilen bei, daher beginnen Sie damit, diese in ihren eigenen Branch zu verschieben.

<h3 id="could-not-find-merge-base-with-the-base-branch">
  Merge-Base mit dem Base-Branch konnte nicht gefunden werden
</h3>

`/code-review ultra` und der `claude ultrareview`-Unterbefehl überprüfen den Diff zwischen Ihrem Branch und einem Base-Branch, was einen Commit erfordert, den die beiden teilen. Wenn `git merge-base` keinen findet, weigert sich Claude Code, die Review zu starten, bevor die Cloud-Sitzung startet. Bei einem Klon, den Claude Code als vollständig überprüfen kann, mit mindestens einem Branch, fällt es stattdessen auf [Überprüfung jeder verfolgten Datei](/docs/de/ultrareview#diff-limits-and-fallbacks) zurück. Sie sehen diese Ablehnung, wenn der Base-Branch überhaupt nicht gefunden werden kann, wenn Claude Code nicht überprüfen kann, dass Ihr Klon vollständig ist, oder im seltenen Repository, wo der Whole-Tree-Diff nicht möglich ist, wie das SHA-256-Objektformat.

```text theme={null}
Could not find merge-base with main. Pass the base branch explicitly (e.g. `/code-review ultra develop`) or make sure you're in a git repo with a main branch.
```

Der Hinweis nach dem ersten Satz hängt davon ab, was Claude Code beobachtet hat:

* **Sie haben keinen Base-Branch übergeben**: Claude Code verglich gegen den Standard-Branch des Repository und schlägt vor, Ihren Base explizit zu übergeben, wie im Beispiel oben
* **Sie haben einen Base-Branch übergeben, der bereits in Ihrem Klon war**: Der Hinweis lautet ``Make sure <branch> exists locally or on origin (try `git fetch origin <branch>`)``
* **Sie haben einen Base-Branch übergeben, der nicht in Ihrem Klon war**: Claude Code hat ihn von origin abgerufen, bevor es verglich. Der Hinweis lautet ``<branch> was fetched from origin but shares no history with HEAD. If another branch is your real base, pass it explicitly (`/code-review ultra <branch>`)``; wenn Claude Code nicht sagen kann, ob Ihr Klon flach ist, schlägt es stattdessen `git fetch --unshallow origin` vor. Vor v2.1.221 schlug der Hinweis `git fetch --unshallow origin` für jeden abgerufenen Base-Branch vor, und bei einem vollständigen Klon schlägt dieser Befehl mit `fatal: --unshallow on a complete repository does not make sense` fehl.

**Was zu tun ist:**

* Wenn ein anderer Branch Ihr echter Base ist, übergeben Sie ihn explizit: `/code-review ultra <branch>`
* Wenn Ihr Klon möglicherweise keine vollständige Historie hat, führen Sie `git fetch --unshallow origin` aus und führen Sie die Review erneut aus

<h3 id="your-checkout-has-no-branches">
  Ihr Checkout hat keine Branches
</h3>

Ein Checkout kann Commits haben, aber keine Branches: Wenn Sie `git init` gefolgt von `git fetch <url>` und `git checkout FETCH_HEAD` ausführen, erhalten Sie einen detached HEAD ohne Refs. Claude Code packt Ihr Repository als Git-Bundle, um es für eine [ultrareview](/docs/de/ultrareview) hochzuladen, und es kann ein Repository, das keine Branches oder andere Refs hat, nicht bündeln, daher weigern sich `/code-review ultra` und der `claude ultrareview`-Unterbefehl die Review, bevor die Cloud-Sitzung startet.

```text theme={null}
Your checkout has no branches (detached HEAD only), which cloud review can't bundle. Create one first — `git checkout -b <name>` — then rerun /code-review ultra.
```

Vor v2.1.221 versuchte Claude Code, jede verffolgte Datei in diesem Checkout zu überprüfen, und der Upload schlug fehl.

**Was zu tun ist:**

* Erstellen Sie einen Branch bei Ihrem aktuellen Commit mit `git checkout -b <name>`, dann führen Sie die Review erneut aus

<h3 id="no-github-account-is-connected-to-your-claude-account">
  Kein GitHub-Konto ist mit Ihrem Claude-Konto verbunden
</h3>

Sie haben `/code-review ultra <PR#>` oder `claude ultrareview <PR#>` ausgeführt, und bevor die Cloud-Sitzung erstellt wird, fragt Claude Code den Server, ob [das GitHub-Konto, das mit Ihrem Claude-Konto verbunden ist](/docs/de/ultrareview#review-a-pull-request), das Repository des PR erreichen kann. Kein Konto ist verbunden oder die Verbindung ist abgelaufen, daher würde der Cloud-Klon fehlschlagen und Claude Code weigert sich zu starten. Claude Code gibt keinen kostenlosen Lauf aus und belastet keine Nutzungsguthaben für einen abgelehnten Start.

```text theme={null}
Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected (or the connection expired). To fix: run /web-setup to reuse your GitHub CLI login, or connect an account at https://claude.ai/connect-github — then re-run /code-review ultra 1234 (allow a minute after connecting).
```

Wenn [`/web-setup`](/docs/de/web-quickstart#connect-from-your-terminal) in Ihrer Sitzung nicht verfügbar ist, benennt die Meldung nur den claude.ai-Link.

**Was zu tun ist:**

* Führen Sie `/web-setup` aus, um Ihre GitHub CLI-Anmeldung mit Ihrem Claude-Konto zu verbinden, oder verbinden Sie ein Konto unter [claude.ai/connect-github](https://claude.ai/connect-github)
* Führen Sie die Review eine Minute nach der Verbindung erneut aus

Vor v2.1.248 überprüfte Claude Code dies nicht vor dem Start.

<h3 id="your-connected-github-account-cant-see-the-repository">
  Ihr verbundenes GitHub-Konto kann das Repository nicht sehen
</h3>

Sie haben `/code-review ultra <PR#>` oder `claude ultrareview <PR#>` ausgeführt, und [das GitHub-Konto, das mit Ihrem Claude-Konto verbunden ist](/docs/de/ultrareview#review-a-pull-request), kann das Repository des PR nicht lesen, daher würde der Cloud-Klon fehlschlagen und Claude Code weigert sich zu starten. Claude Code gibt keinen kostenlosen Lauf aus und belastet keine Nutzungsguthaben für einen abgelehnten Start.

```text theme={null}
Your connected GitHub account can't see <owner>/<repo> — usually the Claude GitHub app isn't installed on <owner> or wasn't granted this repo (web-connected accounts need it for private repos), or a different GitHub account is connected. To fix: run /web-setup to reuse your GitHub CLI login, or install the app at https://github.com/apps/claude/installations/new — then re-run /code-review ultra 1234.
```

Wenn [`/web-setup`](/docs/de/web-quickstart#connect-from-your-terminal) in Ihrer Sitzung nicht verfügbar ist, benennt die Meldung nur die App-Installation.

**Was zu tun ist:**

* Wenn Ihre lokale `gh`-CLI das Repository lesen kann, führen Sie `/web-setup` aus, um diese Anmeldung mit Ihrem Claude-Konto zu verbinden
* Führen Sie die Review nach der Änderung erneut aus

Vor v2.1.248 überprüfte Claude Code dies nicht vor dem Start.

<h3 id="the-github-app-preflight-failed-transiently">
  Der GitHub-App-Preflight ist vorübergehend fehlgeschlagen
</h3>

Sie haben eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web) aus einem lokalen Repository gestartet, und zwei Schritte sind zusammen fehlgeschlagen. Claude Code konnte das Bundle Ihres Repository nicht erstellen oder hochladen. Bevor der Upload, überprüfte es, ob der Cloud-Service das Repository von GitHub klonen kann, und anstatt einer definitiven Antwort endete diese Überprüfung in einem Fehler, den ein erneuter Versuch klären könnte, wie ein Netzwerkfehler, ein Timeout oder ein vorübergehender Serverfehler. Die vollständige Meldung beginnt mit dem, was das Bundle gestoppt hat, z. B. `Could not upload repo bundle (<error>)`, und endet mit dem Preflight-Satz:

```text theme={null}
Could not upload repo bundle (<error>). The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead
```

**Was zu tun ist:**

* Führen Sie den Befehl nach einem Moment erneut aus. Wenn die GitHub-Überprüfung besteht, kann Claude Code die Sitzung von einem GitHub-Klon starten, daher blockiert der fehlgeschlagene Upload den Start nicht mehr
* Wenn erneute Versuche weiterhin fehlschlagen, benennt der Anfang der Meldung, was den Upload gestoppt hat. Wenn diese Ursache etwas ist, das Sie beheben können, beheben Sie es, damit die Sitzung stattdessen von Ihrem lokalen Repository starten kann

Vor v2.1.251 endete Claude Code die Meldung mit `Please set up GitHub on https://claude.ai/code`, auch wenn die GitHub-Überprüfung nur vorübergehend fehlgeschlagen ist, und Setup-Ratschläge können einen vorübergehenden Fehler nicht klären.

<h3 id="github-isnt-connected-to-your-claude-account">
  GitHub ist nicht mit Ihrem Claude-Konto verbunden
</h3>

Sie haben eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web) aus Ihrem lokalen Repository gestartet, z. B. mit `/autofix-pr`. Kein GitHub-Konto ist mit Ihrem Claude-Konto verbunden oder die Verbindung ist abgelaufen, daher weigert sich Claude Code zu starten:

```text theme={null}
GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud. Run /web-setup to connect with your GitHub CLI login, or connect on the web at https://claude.ai/connect-github
```

Wenn Sie eine Routine mit [`/schedule`](/docs/de/routines) erstellen, erscheint dieselbe Meldung als Setup-Notiz, die das Repository benennt; die Notiz blockiert nicht das Erstellen der Routine.

**Was zu tun ist:**

* Führen Sie `/web-setup` aus, um Ihre GitHub CLI-Anmeldung mit Ihrem Claude-Konto zu verbinden, oder verbinden Sie ein Konto unter [claude.ai/connect-github](https://claude.ai/connect-github). Siehe [GitHub-Authentifizierungsoptionen](/docs/de/claude-code-on-the-web#github-authentication-options) für wie die beiden sich unterscheiden.
* Führen Sie den Befehl eine Minute nach der Verbindung erneut aus

Vor v2.1.268 meldete Claude Code dies als vorübergehenden Fehler der Claude GitHub-App-Überprüfung und schlug vor, erneut zu versuchen oder die App zu installieren; keiner verbindet ein GitHub-Konto.

<h3 id="single-sign-on-authorization-needed">
  Single-Sign-On-Autorisierung erforderlich
</h3>

Sie haben [`/install-github-app`](/docs/de/github-actions#quick-setup) ausgeführt und ein Repository ausgewählt, dessen Organisation SAML-Single-Sign-On erzwingt. Bevor das Setup, überprüft Claude Code Ihren Zugriff auf das Repository mit der GitHub CLI, und GitHub lehnte diese Überprüfung ab, weil Ihr `gh`-Token noch nicht für die Organisation autorisiert ist. Der Assistent zeigt die Warnung mit den Schritten zur Autorisierung:

```text theme={null}
Single sign-on authorization needed
<owner>/<repo> belongs to an organization that enforces SAML single sign-on, and your GitHub CLI token isn't authorized for it yet.
```

**Was zu tun ist:**

* Autorisieren Sie Ihre GitHub CLI-Anmeldung mit den `repo`- und `workflow`-Bereichen erneut, indem Sie `gh auth refresh -h github.com -s repo,workflow` ausführen, und autorisieren Sie die Organisation, wenn GitHub zur Single-Sign-On auffordert
* Wenn Sie sich mit einem persönlichen Zugangstoken in `GH_TOKEN` authentifizieren, öffnen Sie [github.com/settings/tokens](https://github.com/settings/tokens), wählen Sie **Configure SSO** auf dem Token und autorisieren Sie die Organisation
* Führen Sie `/install-github-app` erneut aus

Vor v2.1.273 zeigte Claude Code die Warnung `Admin permissions required` für diese Bedingung statt.

<h3 id="failed-to-resume-the-conversation">
  Konversation konnte nicht fortgesetzt werden
</h3>

Claude Code konnte das gespeicherte Transkript für die Sitzung, die Sie aus der [`claude --resume`-Auswahl](/docs/de/sessions#use-the-session-picker) ausgewählt haben, nicht lesen oder verarbeiten, daher beendet es den Prozess, anstatt in einem teilweise geladenen Zustand fortzufahren. Die Meldung enthält den Befehl zum erneuten Versuchen:

```text theme={null}
Failed to resume the conversation.
Run claude --resume <session-id> to retry, or claude to start a new session.
```

Claude Code beendet sich mit Code 1 nach Anzeige der Meldung. Die `/resume`-Auswahl in einer laufenden Sitzung meldet `Failed to resume conversation` in der Konversation statt, und Ihre aktuelle Sitzung läuft weiter. Vor v2.1.216 blieb ein fehlgeschlagenes Resume aus der `claude --resume`-Auswahl auf dem `Resuming conversation…`-Spinner unbegrenzt, anstatt diese Meldung anzuzeigen.

**Was zu tun ist:**

* Führen Sie `claude --resume <session-id>` mit der Sitzungs-ID aus der Meldung aus, um erneut zu versuchen
* Wenn jeder erneute Versuch auf die gleiche Weise fehlschlägt, führen Sie `claude update` aus und setzen Sie fort. Versionen vor v2.1.275 schlagen das Resume fehl, wenn das gespeicherte Transkript einen Eintrag enthält, den sie nicht lesen können.
* Wenn der erneute Versuch erneut fehlschlägt, führen Sie `claude` aus, um eine neue Sitzung zu starten

<h3 id="no-conversation-found-with-the-session-id">
  Keine Konversation mit der Sitzungs-ID gefunden
</h3>

Sie haben eine Sitzungs-ID an `claude --resume <session-id>` übergeben und kein gespeichertes Transkript stimmte überein:

```text theme={null}
No conversation found with session ID: <session-id>
```

Claude Code beendet sich mit Code 1 nach Anzeige der Meldung. Claude Code [durchsucht zuerst das aktuelle Projekt, dann jedes andere Projekt auf dieser Maschine](/docs/de/sessions#resume-a-session) nach der ID. Vor v2.1.223 stoppte die Suche im aktuellen Projektverzeichnis und seinen Git-Worktrees, daher Resume aus dem Verzeichnis, in dem die Sitzung zuletzt funktionierte.

Häufige Ursachen:

* **Falsch eingegebene ID**: Für einen nicht interaktiven Lauf ist die ID das `session_id`-Feld der [`--output-format json`-Ausgabe](/docs/de/headless#get-structured-output)
* **Gelöschtes Transkript**: Claude Code entfernt Transkripte nach der [Aufbewahrungsfrist](/docs/de/sessions#where-transcripts-are-stored), 30 Tage standardmäßig, nach den [Aufbewahrungssweep-Regeln](/docs/de/claude-directory#cleaned-up-automatically)
* **Andere Maschine**: Claude Code speichert Transkripte lokal, daher Resume die Sitzung auf der Maschine, auf der sie ausgeführt wurde
* **Doppelte Kopien**: Wenn Sie ein Projektverzeichnis unter `~/.claude/projects` kopiert haben, so dass zwei Transkripte dieselbe ID tragen, meldet Claude Code diese Meldung, anstatt eine Kopie willkürlich fortzusetzen

**Was zu tun ist:**

* Für eine interaktive Sitzung öffnen Sie die [Sitzungsauswahl](/docs/de/sessions#use-the-session-picker) mit `claude --resume` und drücken Sie `Ctrl+A`, um sie auf jedes Projekt auf dieser Maschine zu erweitern, dann wählen Sie die Sitzung
* Sitzungen, die mit `claude -p` oder dem [Agent SDK](/docs/de/agent-sdk/overview) erstellt wurden, erscheinen nicht in der Auswahl, daher überprüfen Sie die ID erneut gegen die `session_id`, die Ihr ursprünglicher Lauf gedruckt hat

<h3 id="cannot-switch-renderers-in-this-session">
  Renderer können in dieser Sitzung nicht gewechselt werden
</h3>

Wenn Sie Renderer wechseln, startet Claude Code seinen Prozess neu. Sie haben [`/tui`](/docs/de/fullscreen#enable-fullscreen-rendering) in einer Sitzung ausgeführt, die Claude Code ablehnt neu zu starten, daher wechselt es nicht und speichert nichts. Welche Meldung Sie sehen, sagt Ihnen die Ursache:

* `Cannot switch renderers while work is running in the background`: Sie haben Hintergrundarbeit, die ein Neustart aufgeben würde, wie eine Background-Shell oder einen Subagenten. Warten Sie, bis die Arbeit fertig ist, oder stoppen Sie sie mit [`/tasks`](/docs/de/commands), dann führen Sie `/tui fullscreen` oder `/tui default` erneut aus
*

`Cannot switch renderers in this session`: Die Sitzung hat Einschränkungen, die Claude Code nicht an den neu gestarteten Prozess übergeben kann. Vor v2.1.234 startete Claude Code trotzdem neu und die neu gestartete Sitzung lief ohne sie

In der Einschränkungsmeldung benennt der Teil in Klammern die Einschränkungen, die Claude Code gefunden hat:

```text theme={null}
Cannot switch renderers in this session — it has restrictions a restart can't carry over (permission rules set for this session only). Nothing was changed. Running /tui fullscreen in a session started without them switches every later session too.
```

Jeder Grund, den die Meldung in Klammern zeigen kann:

* `launch flags: a custom system prompt, a tool allowlist, or restricted settings`: Sie haben die Sitzung mit einem Flag gestartet, das Claude Code nicht an den neu gestarteten Prozess zurückgibt. Diese Flags umfassen [`--system-prompt`](/docs/de/cli-reference#cli-flags), `--system-prompt-file`, `--append-system-prompt-file`, eine [`--tools`](/docs/de/cli-reference#cli-flags)-Allowlist, [`--setting-sources`](/docs/de/cli-reference#cli-flags) und [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags)
* `permission rules set for this session only`: Ein [Berechtigungsupdate](/docs/de/hooks#permission-update-entries) aus einem Hook oder SDK-Aufrufer hat Deny- oder Ask-Regeln mit dem `session`-Ziel hinzugefügt. Sitzungsspezifische Allow-Regeln lösen die Ablehnung nicht aus. Ein Neustart löscht sie, und Claude Code fragt stattdessen erneut
* `ask-before-running rules with no command-line form`: Ein Berechtigungsupdate aus einem Hook oder SDK-Aufrufer hat Ask-Regeln neben den Regeln hinzugefügt, die Claude Code als `--allowed-tools` und `--disallowed-tools` zurückgibt. Kein Flag existiert für Ask-Regeln
* `permission rules a command line cannot carry intact` und `added directories a command line cannot carry intact`: Ein Berechtigungsupdate hat eine Regel oder einen Verzeichnispfad mid-session hinzugefügt. Die Befehlszeile des neu gestarteten Prozesses kann seinen Text nicht als denselben Wert tragen

**Was zu tun ist:**

* Führen Sie in einer Sitzung, die ohne diese Einschränkungen gestartet wurde, `/tui fullscreen` oder `/tui default` aus, um zurückzuwechseln. Claude Code speichert die [`tui`-Einstellung](/docs/de/settings-reference#tui) dort

<h3 id="couldnt-open-claude-desktop">
  Couldn't open Claude Desktop
</h3>

Sie haben [`/desktop`](/docs/de/desktop#coming-from-the-cli) oder seinen Alias `/app` ausgeführt, und der System-Befehl, den Claude Code zum Öffnen von Claude Desktop verwendet, ist fehlgeschlagen. Die Sitzung bleibt im Terminal.

```text theme={null}
Error: Couldn't open Claude Desktop (`open` exited 1: LSOpenURLsWithRole() failed for the URL claude://resume?session=<session-id> with error -10814). Open Claude Desktop and run /desktop again.
```

**Was zu tun ist:**

* Öffnen Sie Claude Desktop selbst, dann führen Sie `/desktop` erneut aus
* Um die vollständige Fehlerausgabe dieses Befehls zu lesen, aktivieren Sie Debug-Protokollierung mit `/debug`, führen Sie `/desktop` erneut aus, und überprüfen Sie das Debug-Protokoll

Vor v2.1.275 war die Meldung `Failed to open Claude Desktop. Please try opening it manually.` und sagte nicht, was fehlgeschlagen ist.

<h3 id="terminal-setup-left-your-zed-keymap-unchanged">
  /terminal-setup hat Ihre Zed-Tastaturzuordnung unverändert gelassen
</h3>

Sie haben [`/terminal-setup`](/docs/de/terminal-config#enter-multiline-prompts) in Zed ausgeführt, und Claude Code konnte das Update zu Ihrer Zed `keymap.json` nicht abschließen, daher hat es die Datei unverändert gelassen.

Jede Meldung benennt den Pfad zu Ihrer Tastaturzuordnung und endet mit dem Tastaturzuordnungsblock, den Sie selbst hinzufügen:

```text theme={null}
Couldn't update your Zed keymap, so it was left unchanged.
To add the binding yourself, add this block to the keymap array in <path to keymap.json>:
{ "context": "Terminal", "bindings": { "shift-enter": ["terminal::SendText", "\u001b\r"] } }
```

Die erste Zeile der Meldung benennt die Ursache:

* `Couldn't read your Zed keymap, so it was left unchanged.`: Claude Code konnte die Datei nicht lesen, z. B. wegen Dateiberechtigungen
* `Your Zed keymap isn't a readable list of keybindings, so it was left unchanged.`: Die Datei las sich gut, aber analysiert nicht als Array von Tastaturzuordnungsblöcken, auch mit `//`-Kommentaren und nachfolgenden Kommas erlaubt
* `Couldn't back up your Zed keymap; not modifying it.`: Claude Code konnte die Datei nicht zu einer `.bak`-Sicherung daneben kopieren, daher hat es nichts geändert
* `Couldn't update your Zed keymap, so it was left unchanged.`: Das zusammengeführte Ergebnis überprüfte nicht als gültige Tastaturzuordnung, die die Bindung trägt, daher verwarf Claude Code es, anstatt zu schreiben. Ein Tastaturzuordnungsblock mit einem duplizierten Schlüssel kann dies verursachen

**Was zu tun ist:**

* Kopieren Sie den Block aus der Meldung in das Top-Level-Array in Ihrer `keymap.json` unter dem Pfad, den die Meldung benennt
* Für `isn't a readable list of keybindings`, beheben Sie den Syntaxfehler oder machen Sie den Top-Level-Wert der Datei ein Array, dann führen Sie `/terminal-setup` erneut aus

Vor v2.1.247 konnte `/terminal-setup` eine Zed-Tastaturzuordnung, die `//`-Kommentare oder nachfolgende Kommas verwendete, nicht analysieren, und es ersetzte die gesamte Datei mit nur seiner eigenen Bindung, während es die Bindung als installiert meldete. Um eine Tastaturzuordnung wiederherzustellen, die eine frühere Version ersetzte, verwenden Sie die `.bak`-Sicherungsdatei, die unter [Mehrzeilige Eingabeaufforderungen eingeben](/docs/de/terminal-config#enter-multiline-prompts) beschrieben ist.

<h3 id="skill-usage-reports-are-not-available-on-this-connection">
  Skill-Nutzungsberichte sind auf dieser Verbindung nicht verfügbar
</h3>

Sie haben [`/skill-doctor`](/docs/de/skills#find-unused-skills) über [Remote-Steuerung](/docs/de/remote-control) ausgeführt, von Ihrem Telefon oder Browser. Claude Code sendet den Skill-Nutzungsbericht nicht über Remote-Steuerung und antwortet stattdessen mit dieser Meldung:

```text theme={null}
Skill usage reports are not available on this connection.
```

**Was zu tun ist:**

* Führen Sie `/skill-doctor` im Terminal auf der Maschine aus, auf der die Sitzung ausgeführt wird, oder führen Sie `claude -p "/skill-doctor"` dort aus

<h3 id="custom-output-styles-cant-be-selected-over-remote-control">
  Benutzerdefinierte Ausgabestile können nicht über Remote-Steuerung ausgewählt werden
</h3>

Sie haben [`/output-style`](/docs/de/output-styles#change-your-output-style) von der mobilen App oder dem Web über [Remote-Steuerung](/docs/de/remote-control) ausgeführt, oder der Befehl kam in einer Nachricht an, die in die Sitzung weitergeleitet wurde. Da ein solcher Zug möglicherweise nicht vom Kontoinhaber kommt, listet Claude Code nur [integrierte Stile](/docs/de/output-styles#built-in-output-styles) auf und wählt sie aus, und fügt diese Notiz hinzu, wenn der Befehl die Stile auflistet oder den Namen, den Sie angegeben haben, nicht erkennt. Ein [benutzerdefinierter Stil](/docs/de/output-styles#create-a-custom-output-style)-Name erhält die gleiche Antwort wie ein Name, der nicht existiert:

```text theme={null}
Custom output styles can't be selected over Remote Control or from a relayed message. Select one in the session itself, or pick a built-in style here.
```

**Was zu tun ist:**

* Wählen Sie einen integrierten Stil, z. B. `/output-style concise`
* Um einen benutzerdefinierten Stil zu verwenden, setzen Sie [`outputStyle`](/docs/de/settings-reference#outputstyle) in der Datei `.claude/settings.local.json` des Projekts, oder führen Sie `/output-style <style>` im Terminal der Sitzung selbst aus, wenn es eines hat

<h3 id="output-styles-are-saved-to-local-settings-which-this-session-doesnt-load">
  Ausgabestile werden in lokalen Einstellungen gespeichert, die diese Sitzung nicht lädt
</h3>

Sie haben versucht, [Ausgabestile](/docs/de/output-styles) mit `/output-style <style>` oder `/config outputStyle=<style>` in einer Sitzung zu wechseln, deren Einstellungsquellen `local` ausschließen. Beispiele sind eine [Agent SDK](/docs/de/agent-sdk/typescript)-Sitzung, deren [`settingSources`](/docs/de/agent-sdk/typescript#options) `"local"` auslässt, und eine CLI-Sitzung, die mit einem [`--setting-sources`](/docs/de/cli-reference#cli-flags)-Wert gestartet wurde, der `local` auslässt. Beide Befehle speichern den Stil in `.claude/settings.local.json`, eine Datei, die eine solche Sitzung nie zurückliest, daher weigert sich Claude Code, anstatt eine Einstellung zu schreiben, die keine Auswirkung hätte:

```text theme={null}
Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load, so the style can't be changed here.
```

**Was zu tun ist:**

* Fügen Sie `local` zu den Einstellungsquellen der Sitzung hinzu und wechseln Sie erneut
* Setzen Sie den [`outputStyle`](/docs/de/settings-reference#outputstyle)-Schlüssel in einer Einstellungsdatei, die die Sitzung lädt, wie `.claude/settings.json` im Projekt oder `~/.claude/settings.json`. Im TypeScript SDK setzen Sie stattdessen `outputStyle` im Inline-`settings`-Objekt; siehe [Aktivieren Sie einen Ausgabestil](/docs/de/agent-sdk/modifying-system-prompts#activate-an-output-style)

<h2 id="plugin-errors">
  Plugin-Fehler
</h2>

Diese Fehler stammen aus der [Plugin](/docs/de/plugins/overview)- und [Marketplace](/docs/de/plugins/overview)-Konfiguration. Bei Plugin-Problemen, die keine der Meldungen auf dieser Seite erzeugen, wie z. B. eine Marketplace-URL, die nicht geladen wird, oder ein Plugin, das installiert wird, aber nicht angezeigt wird, siehe [Plugin-Fehlerbehebung](/docs/de/plugins/troubleshooting).

<h3 id="plugin-eval-is-currently-in-early-access">
  plugin eval ist derzeit in Early Access
</h3>

Sie haben [`claude plugin eval`](/docs/de/plugin-evals) oder `claude plugin eval init` ausgeführt und es wurde mit Exit-Code 1 beendet, bevor es etwas tat, mit einer dieser Meldungen:

```text theme={null}
`plugin eval` is currently in early access
```

```text theme={null}
`plugin eval` is currently unavailable
```

Die erste Meldung bedeutet, dass Ihr Build älter als v2.1.269 ist, der ersten Version, in der der Befehl allgemein verfügbar ist. Die zweite bedeutet, dass Anthropic den Befehl serverseitig deaktiviert hat; nichts auf Ihrem Computer schaltet ihn wieder ein.

**Was zu tun ist:**

* Führen Sie `claude --version` aus, dann `claude update`, und führen Sie den Befehl erneut in einer neuen Sitzung aus. Siehe die [Anforderungen für Plugin-Evals](/docs/de/plugin-evals#requirements)
* Wenn Sie die zweite Meldung auf einem aktuellen Build sehen, versuchen Sie es später erneut nach einem weiteren `claude update`

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace ist von einer nicht vertrauenswürdigen Quelle registriert
</h3>

Der Marketplace ist unter einem Namen registriert, der [für offizielle Anthropic-Marketplaces reserviert ist](/docs/de/plugins/marketplace-reference#marketplace-file), aber seine registrierte Quelle ist kein `anthropics` GitHub-Repository. Claude Code überprüft reservierte Namen jedes Mal, wenn es einen Marketplace lädt oder aktualisiert, sodass der Marketplace und die von ihm installierten Plugins nicht mehr geladen werden. Vor v2.1.205 wurde der Name nur überprüft, wenn der Marketplace hinzugefügt wurde, sodass ein Eintrag, der registriert wurde, bevor sein Name reserviert wurde, weiterhin geladen wurde.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

Für einen Marketplace, dessen Quelle kein GitHub-Repository oder eine Git-URL ist, wie z. B. ein lokales Verzeichnis, lautet der mittlere Satz `can only be used with GitHub sources from the 'anthropics' organization` statt dessen. `claude plugin marketplace add` führt die gleiche Überprüfung durch und lehnt einen reservierten Namen mit `Failed to add marketplace:` gefolgt von demselben reservierten Namen-Satz ab.

**Was zu tun ist:**

* Wenn der Marketplace bereits registriert ist, führen Sie `claude plugin marketplace remove <name>` aus und fügen Sie ihn dann erneut aus dem offiziellen `github.com/anthropics`-Repository hinzu
* Wenn Sie einen Drittanbieter-Marketplace veröffentlichen, der den Namen verwendet hat, bevor er reserviert wurde, benennen Sie ihn um und bitten Sie Benutzer, ihn von Ihrer Quelle erneut hinzuzufügen
* Siehe die Liste der reservierten Namen unter [Marketplace-Schema](/docs/de/plugins/marketplace-reference#marketplace-file)

<h3 id="marketplace-name-is-another-spelling-of-a-reserved-name">
  Marketplace-Name ist eine andere Schreibweise eines reservierten Namens
</h3>

Der Name des Marketplace ist selbst kein reservierter Name, aber Claude Code behandelt ihn als eine andere Schreibweise eines solchen. [Reservierte Namen](/docs/de/plugins/marketplace-reference#reserved-name-spellings) listet auf, welche Schreibweisen als reservierter Name gelten. Claude Code lehnt einen solchen Namen ab, wenn Sie den Marketplace hinzufügen:

```text theme={null}
Failed to add marketplace: "claude.code.plugins" is another spelling of "claude-code-plugins", a reserved marketplace name.
```

Wenn ein Marketplace bereits unter einem solchen Namen registriert ist, wird sein Eintrag nicht mehr geladen, und `/plugin`, `claude plugin install` und `claude plugin update` warnen:

```text wrap theme={null}
known_marketplaces.json has an entry named "claude.code.plugins", another spelling of the reserved marketplace name "claude-code-plugins", so it is ignored. Remove it with: claude plugin marketplace remove claude.code.plugins
```

Wenn der Name Shell-Quoting benötigen würde, lautet die Ablehnung beim Hinzufügen `This marketplace's name is another spelling of "<reserved>", a reserved marketplace name. It is not exactly the reserved name it appears to be.`

**Was zu tun ist:**

* Benennen Sie den Marketplace in einen Namen um, der keine reservierte Schreibweise darstellt, und fügen Sie ihn erneut hinzu
* Für die Warnung zu ignoriertem Eintrag führen Sie den `claude plugin marketplace remove`-Befehl aus, den sie angibt, oder entfernen Sie den Eintrag aus `~/.claude/plugins/known_marketplaces.json`

<h3 id="marketplace-is-already-added-from-a-different-source">
  Marketplace ist bereits von einer anderen Quelle hinzugefügt
</h3>

Sie haben das Hinzufügen eines Marketplace durch [`/plugin install <plugin> --marketplace <source>`](/docs/de/plugins/install#add-a-marketplace-and-install-in-one-command) bestätigt, und der Katalog, den Claude Code aus dieser Quelle abgerufen hat, nennt sich selbst genauso wie ein Marketplace, den Sie bereits von einer anderen Quelle hinzugefügt haben. Claude Code behält den vorhandenen Marketplace bei, anstatt ihn zu ersetzen, und das Plugin wird nicht installiert.

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

**Was zu tun ist:**

* Wenn der Marketplace, den Sie bereits hinzugefügt haben, der ist, den Sie möchten, installieren Sie ihn nach Name: `/plugin install <plugin>@<name>`
* Um zur neuen Quelle zu wechseln, führen Sie `/plugin marketplace remove <name>` aus und versuchen Sie dann die Installation erneut

<h3 id="plugin-command-references-user-config">
  Plugin-Befehl referenziert user\_config in einem Shell-Befehl
</h3>

Ein Plugin-Hook, [monitor](/docs/de/plugins/components#monitors) oder MCP [`headersHelper`](/docs/de/mcp#use-dynamic-headers-for-custom-authentication)-Befehl referenziert eine `${user_config.KEY}` [Plugin-Option](/docs/de/plugins/manifest-reference#user-configuration), und die ersetzte Zeichenkette würde an eine Shell übergeben. Ein konfigurierter Wert, der `$(...)`, Backticks oder `;` enthält, würde dort als Code ausgeführt, daher weigert sich Claude Code, die Komponente zu starten, anstatt den Wert zu ersetzen. Die Überprüfung läuft auf der Befehlsvorlage, daher wird der Fehler angezeigt, auch wenn noch kein Wert konfiguriert ist. Vor v2.1.207 wurde der Wert in den Shell-Befehl ersetzt.

Die Formulierung hängt davon ab, welche Oberfläche die Option referenziert hat. Ein Shell-Form-Hook meldet:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Ein Monitor meldet:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

Ein MCP `headersHelper` meldet:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Was zu tun ist:**

* Für einen Hook fügen Sie ein `args`-Array hinzu, damit es in [Exec-Form](/docs/de/hooks#exec-form-and-shell-form) ausgeführt wird, wobei jedes `${user_config.KEY}` zu einem Argument wird, ohne dass eine Shell dazwischen liegt. Oder lassen Sie die Referenz weg und lesen Sie die `$CLAUDE_PLUGIN_OPTION_<KEY>`-Umgebungsvariable innerhalb des Skripts
* Für einen Monitor lassen Sie die Referenz weg und lassen Sie das Monitor-Skript den Wert aus einer Konfigurationsdatei lesen
* Für einen `headersHelper` verschieben Sie `${user_config.KEY}` in das `headers`-Feld des Servers, das nicht shell-geparst wird, oder lesen Sie den Wert innerhalb des Helper-Skripts

<h3 id="plugin-archive-integrity-check-failed">
  Plugin-Archiv-Integritätsprüfung fehlgeschlagen
</h3>

Der Marketplace-Eintrag des Plugins verwendet eine [`archive`-Quelle](/docs/de/plugins/marketplace-reference#archive-plugin-source) mit einem `sha256`-Pin, und der Digest der heruntergeladenen Datei stimmt nicht mit dem Pin überein. Claude Code lehnt die Installation ab, daher ändert sich nichts im Plugin-Cache. Die Nichtübereinstimmung hat drei mögliche Ursachen:

* Die Datei unter der URL hat sich geändert, nachdem der Autor den Pin berechnet hat
* Der Autor hat den falschen Digest im Marketplace-Eintrag eingegeben
* Die URL stellt eine andere Datei bereit als der Autor gepinnt hat

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

**Was zu tun ist:**

* Wenn Sie das Plugin veröffentlichen, berechnen Sie den Digest der genauen Datei, die die URL bereitstellt, z. B. mit `shasum -a 256 my-plugin.zip` oder `Get-FileHash -Algorithm SHA256 my-plugin.zip` in PowerShell, und aktualisieren Sie den `sha256` im Marketplace-Eintrag
* Wenn Sie das Plugin installieren, führen Sie `/plugin marketplace update <name>` aus, um den Katalog zu aktualisieren, falls der Eintrag korrigiert wurde, und versuchen Sie dann die Installation erneut
* Wenn die Digests nach einer Aktualisierung immer noch nicht übereinstimmen, fragen Sie den Marketplace-Besitzer, welche Datei er vor der Installation gepinnt hat

<h3 id="path-escapes-plugin-directory">
  Pfad entweicht dem Plugin-Verzeichnis
</h3>

Ein Plugin-Komponentenpfad, der im `plugin.json` des Plugins oder in seinem [Marketplace-Eintrag](/docs/de/plugins/marketplace-reference#plugin-entries) deklariert ist, wird außerhalb des eigenen Verzeichnisses des Plugins aufgelöst. Claude Code verwirft diesen Pfad und lädt den Rest des Plugins. Der Komponentenname in der Meldung, wie z. B. `commands` oder `hooks`, benennt das Feld, das den Pfad deklariert hat.

```text theme={null}
commands path escapes plugin directory: ./../shared.md
```

In der `claude plugin`-Befehlsausgabe liest sich derselbe Fehler als `Path escapes plugin directory: ./../shared.md (commands)`.

Claude Code lehnt sowohl einen Pfad ab, der außerhalb des Plugins zeigt, wie geschrieben, z. B. `../shared-utils`, als auch einen Symlink, der außerhalb des Plugins führt und nicht einer der [Marketplace-Symlink-Regeln](/docs/de/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks) entspricht. Für einen Symlink sagt die Meldung auch, wo der Pfad aufgelöst wird:

```text theme={null}
commands path escapes plugin directory: ./commands/deploy.md — it resolves to /home/user/shared/deploy.md, outside the plugin directory
```

Auf macOS und Linux lehnt Claude Code auch einen Komponentenpfad ab, der an irgendeiner Stelle einen Backslash enthält, auch wenn der Pfad im Plugin bleibt. Ein Plugin, dessen Komponentenpfade Windows-ähnliche Trennzeichen verwenden, wird auf Windows geladen und löst diese Ablehnung auf den anderen Plattformen aus:

```text theme={null}
commands path escapes plugin directory: ./commands\deploy.md — its path contains a backslash, which is not resolved reliably on this platform
```

Vor v2.1.251 lud Claude Code einen `commands`-Pfad, der in einem Marketplace-Eintrag deklariert war, auch wenn er außerhalb des Plugin-Verzeichnisses zeigte. Claude Code lehnte bereits Pfade ab, die in `plugin.json` deklariert waren, und die anderen Komponentenpfade in einem Marketplace-Eintrag.

Vor v2.1.257 überprüfte die Kontrolle nur die Schreibweise des Pfads, nicht wo ein Symlink führt.

**Was zu tun ist:**

* Verschieben Sie die referenzierte Datei in das Plugin-Verzeichnis und zeigen Sie mit einem `./` relativen Pfad darauf
* Wenn der Pfad ein Symlink zu einer Datei außerhalb des Plugins ist, ersetzen Sie den Symlink durch eine Kopie der Datei
* Wenn die Meldung sagt, dass der Pfad einen Backslash enthält, schreiben Sie den Pfad mit Schrägstrichen, z. B. `./commands/deploy.md`
* Um Dateien mit anderen Plugins im selben Marketplace zu teilen, verlinken Sie sie mit einem Symlink im Plugin-Verzeichnis, gemäß den [Symlink-Regeln](/docs/de/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)

<h3 id="path-could-not-be-checked">
  Pfad konnte nicht überprüft werden
</h3>

Claude Code fragte das Betriebssystem, ob ein Plugin-Pfad existiert, und erhielt einen Fehler, der nicht „nicht gefunden" ist, daher wird nicht geladen, was der Pfad benennt. Wie viel des Plugins geladen wird, hängt davon ab, welcher Pfad fehlgeschlagen ist:

* Einer der [Standard-Komponentenordner](/docs/de/plugins/manifest-reference#standard-layout) eines Plugins, wie z. B. der `skills/`-Ordner, die `monitors/monitors.json`-Datei oder eine [`SKILL.md` im Plugin-Stammverzeichnis](/docs/de/plugins/components#skills): Die anderen Komponenten des Plugins werden weiterhin geladen
* Das eigene Verzeichnis des Plugins: Nichts aus diesem Plugin wird geladen

Sie sehen diesen Fehler nicht für einen Pfad, der überhaupt nicht existiert. In `/plugin` wird der Fehler unter dem Plugin angezeigt und benennt den Pfad und den Code, den das Betriebssystem zurückgegeben hat:

```text theme={null}
skills path could not be checked: /home/user/my-plugin/skills (ELOOP)
```

In `claude plugin list` liest sich derselbe Fehler als `Path not found: /home/user/my-plugin/skills (skills, ELOOP)`.

Ursachen, die diesen Fehler erzeugen, sind:

* `ELOOP`: Ein Symlink im Pfad zeigt auf sich selbst oder bildet eine Schleife
* `EIO` oder `ESTALE`: Der Pfad befindet sich auf einer unterbrochenen oder veralteten Netzwerkbereitstellung
* `EACCES`: Eines der Verzeichnisse über dem Pfad verweigert Ihnen die Berechtigung, es zu durchqueren

**Was zu tun ist:**

* Ersetzen Sie einen Symlink, der auf sich selbst zeigt, durch einen echten Ordner, oder löschen Sie ihn
* Wenn sich der Pfad auf einer Netzwerkbereitstellung befindet, hängen Sie die Freigabe erneut ein
* Wenn der Code `EACCES` ist, stellen Sie Ihre Ausführungsberechtigung für die Verzeichnisse über dem Pfad wieder her
* Führen Sie `/reload-plugins` aus, nachdem Sie den Pfad behoben haben, oder starten Sie Claude Code neu, um das Plugin oder die Komponente zu laden

Vor v2.1.265 behandelte Claude Code einen Standard-Komponentenordner, den es nicht überprüfen konnte, als abwesend und lud das Plugin ohne diese Komponente, ohne Fehler.

<h3 id="marketplace-entry-path-does-not-stay-inside-the-marketplace-directory">
  Marketplace-Eintragspfad bleibt nicht im Marketplace-Verzeichnis
</h3>

Der [Marketplace-Eintrag](/docs/de/plugins/marketplace-reference#plugin-entries) des Plugins deklariert einen Quellpfad, den Claude Code nicht zu einem Ort im eigenen Verzeichnis des Marketplace auflösen kann, daher wird das Plugin nicht installiert oder geladen. Die Ablehnung umfasst:

* Ein Eintrags-Pfad, der absolut ist, mit `..` aus dem Marketplace klettert oder wie ein Netzwerkpfad geschrieben ist
* Auf macOS und Linux einen Eintrags-Pfad, der an irgendeiner Stelle nach dem führenden `./` einen Backslash enthält
* Ein Eintrag in einem Marketplace, der aus einer Remote-Quelle wie Git oder einer URL abgerufen wird, der sein Ziel durch einen Symlink erreicht, der außerhalb des Marketplace-Verzeichnisses aufgelöst wird
* Ein relativer Eintrag in einem Marketplace, der von einer direkten URL zu seiner `marketplace.json` hinzugefügt wurde: Claude Code lädt nur diese Datei herunter, daher existieren keine lokalen Plugin-Dateien für den Pfad zum Benennen. Siehe [Plugins mit relativen Pfaden schlagen in URL-basierten Marketplaces fehl](/docs/de/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)

`claude plugin install` meldet die Ablehnung wie folgt:

```text theme={null}
Cannot install my-plugin@my-marketplace: its marketplace entry path does not stay inside the marketplace directory (an absolute, climbing, network-shaped, backslash-containing or link-traversing entry, an entry of a fetched marketplace that resolves or opens outside its tree — or a relative entry in a url-catalog marketplace, which has no local directory)
```

Wenn ein bereits installiertes Plugin-Eintrag die gleiche Überprüfung nicht besteht, zeigt `claude plugin list` das Plugin als `failed to load` mit:

```text theme={null}
Plugin source path refused: ./my-plugin does not stay inside its marketplace directory. Check that the marketplace entry has a plain relative path.
```

**Was zu tun ist:**

* Wenn Sie den Marketplace verwalten, schreiben Sie den `source` des Eintrags als einen einfachen relativen Pfad mit Schrägstrichen, wie z. B. `./plugins/my-plugin`, und halten Sie jeden Symlink, den er kreuzt, auf das Marketplace-Verzeichnis gerichtet
* Wenn Sie den Marketplace von einer direkten URL hinzugefügt haben, können relative Einträge nicht aufgelöst werden. Bitten Sie den Marketplace-Autor, [eine andere Plugin-Quelle](/docs/de/plugins/marketplace-reference#plugin-sources) zu verwenden, oder fügen Sie den Marketplace stattdessen aus seinem Git-Repository hinzu

<h3 id="failed-to-load-marketplace-configuration">
  Fehler beim Laden der Marketplace-Konfiguration
</h3>

Claude Code speichert die Marketplaces, die Sie hinzugefügt haben, in einer Registrierungsdatei unter `~/.claude/plugins/known_marketplaces.json`. Ein Plugin-Befehl, der die Registrierung benötigt, wie z. B. `claude plugin install`, schlägt mit einer von zwei Meldungen fehl, wenn Claude Code die Datei nicht verwenden kann:

* `Failed to load marketplace configuration`: Die Datei ist kein gültiges JSON oder kann nicht gelesen werden. Eine leere Datei schlägt auf diese Weise fehl.
* `Marketplace configuration file is corrupted`: Die Datei ist gültiges JSON, aber ihr Inhalt stimmt nicht mit dem Registrierungsschema überein.

Eine fehlende Datei ist kein Fehler: Claude Code behandelt sie als eine Registrierung ohne Marketplaces.

Mit einer leeren Datei meldet `claude plugin install`:

```text theme={null}
✘ Failed to install plugin "my-plugin": Failed to load marketplace configuration: JSON Parse error: Unexpected EOF
```

Vor v2.1.246 meldete `claude plugin install` diesen Fehler nicht.

**Was zu tun ist:**

* Öffnen Sie `~/.claude/plugins/known_marketplaces.json` und reparieren Sie das JSON, oder beheben Sie die Einträge, die die Meldung als nicht dem Registrierungsschema entsprechend benennt
* Wenn Sie es nicht reparieren können, löschen Sie die Datei oder ersetzen Sie ihren Inhalt durch `{}`, und fügen Sie dann jeden Marketplace mit `claude plugin marketplace add <source>` erneut hinzu. Claude Code registriert die Marketplaces, die Ihre Benutzer- oder verwalteten Einstellungen in [`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces) deklarieren, das nächste Mal, wenn Sie es in einem Ordner starten, dem Sie vertraut haben.

<h3 id="plugin-is-required-by-your-organization">
  Plugin ist von Ihrer Organisation erforderlich
</h3>

Sie haben `claude plugin disable` ausgeführt oder die `/plugin` **Installiert**-Registerkarte verwendet, um ein [Plugin, das von claude.ai synchronisiert wird](/docs/de/plugins/loading#synced-plugins), auszuschalten, das Ihre Organisation als erforderlich kennzeichnet:

```text theme={null}
Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.
```

Claude Code speichert nichts und das Plugin bleibt aktiviert.

Wenn Sie versuchen, ein Plugin zu deaktivieren, das ein erforderliches Plugin benötigt, weigert sich Claude Code auf die gleiche Weise, mit einer Meldung, die das erforderliche Plugin benennt, das es benötigt.

**Was zu tun ist:**

* Bitten Sie einen Administrator Ihrer claude.ai-Organisation, den erforderlichen Status des Plugins auf claude.ai zu ändern

<h2 id="tool-errors">
  Tool-Fehler
</h2>

Diese Fehler stammen von Claudes integrierten Tools. Claude korrigiert die meisten Tool-Fehler automatisch. Wenn eine Änderung von Ihnen erforderlich ist, gibt die Liste **Was zu tun ist** für diesen Fehler an, was zu ändern ist.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent würde mit null Tools erzeugt
</h3>

Jeder Eintrag in der [`tools`-Liste](/docs/de/sub-agents#supported-frontmatter-fields) des Subagenten konnte mit keinem verwendbaren Tool abgeglichen werden, daher weigerte sich Claude Code, den Subagenten zu starten: Ohne Tools konnte er nicht handeln. Die Nachricht gruppiert Ihre Einträge nach dem, was schiefgelaufen ist:

* **Unbekannt**: Der Eintrag stimmt mit keinem Tool-Namen überein, normalerweise ein Tippfehler wie `Grpe` für `Grep`.
* **Nicht für Subagenten verfügbar**: Der Eintrag benennt ein echtes Tool, das [Subagenten nicht verwenden können](/docs/de/sub-agents#available-tools). Hintergrund-Subagenten behalten einen kleineren integrierten Tool-Satz, daher landet ein Eintrag, den nur ein Vordergrund-Subagent verwenden kann, hier, wenn der Subagent im Hintergrund ausgeführt würde, was die Standardeinstellung ist. Wenn Sie `Agent` auflisten, meldet die Nachricht ihn stattdessen unter der nächsten Gruppe.
* **Stimmt mit keinen Tools in dieser Sitzung überein**: Der Eintrag ist gültig, aber kein Tool in der aktuellen Sitzung stimmt gerade damit überein, wie `mcp__github__*` ohne verbundenen GitHub-MCP-Server oder `Agent` für einen Subagenten am [Tiefenlimit](/docs/de/sub-agents#let-subagents-spawn-their-own-subagents).

Das Weglassen des `tools`-Feldes löst diese Weigerung nie aus. Wenn Sie die `tools`-Liste leer lassen oder `disallowedTools` jeden Eintrag darin entfernt, überspringt Claude Code die Weigerung auch und startet den Subagenten ohne Tools.

Vor v2.1.208 wurde der Subagent ohne Tools gestartet und konnte ein leeres oder verwirrendes Ergebnis zurückgeben.

```text theme={null}
Agent 'code-reviewer' würde mit null Tools erzeugt — Weigerung. Seine Tools-Liste wurde zu nichts aufgelöst: unbekannt [Grpe]. Beheben Sie die Tools-Frontmatter des Agenten oder übergeben Sie einen anderen subagent_type.
```

**Was zu tun ist:**

* Korrigieren Sie jeden Eintrag, den der Fehler benennt, anhand der [für Subagenten verfügbaren Tools](/docs/de/sub-agents#available-tools)
* Entfernen Sie Einträge für Tools, die die Sitzung nicht hat, wie MCP-Tools von einem Server, der nicht verbunden ist
* Für ein Tool, das [Hintergrund-Subagenten ablegen](/docs/de/sub-agents#available-tools), wie `CronCreate`, entfernen Sie den Eintrag. Um das Tool zu behalten, [schalten Sie den Fork-Modus aus](/docs/de/sub-agents#turn-fork-mode-on-or-off) und bitten Sie Claude, den Subagenten im Vordergrund auszuführen
* Löschen Sie das `tools`-Feld, anstatt Tools aufzulisten, um dem Subagenten alle [für Subagenten verfügbaren Tools](/docs/de/sub-agents#available-tools) zu geben
* Für eine `tools`-Liste, die nur `Agent` enthält, erhöhen Sie das [Tiefenlimit](/docs/de/sub-agents#let-subagents-spawn-their-own-subagents) oder geben Sie dem Agenten mindestens ein anderes Tool: Claude Code behält `Agent` bei diesem Limit zurück, daher wird eine Liste mit nichts anderem darin zu keinen Tools aufgelöst

<h3 id="file-is-covered-by-a-read-deny-rule">
  Datei wird durch eine Read-Ablehnungsregel abgedeckt
</h3>

Das Edit- oder Write-Tool wurde auf einem Pfad aufgerufen, der einer [`Read`-Ablehnungsregel](/docs/de/permissions#read-and-edit) entspricht, einschließlich der Erstellung einer neuen Datei unter diesem Pfad. Beide Tools ändern Inhalte, die Claude lesen können muss, daher weigert sich Claude Code, den Aufruf vor jedem Dateizugriff zu tätigen. NotebookEdit wird nicht durch `Read`-Ablehnungsregeln abgedeckt. Vor v2.1.228 blockierte die Regel nur das Edit-Tool, und vor v2.1.208 blockierte nur eine `Edit`-Ablehnungsregel Bearbeitungen.

```text theme={null}
Datei wird durch eine Read-Ablehnungsregel in Ihren Berechtigungseinstellungen abgedeckt und kann nicht bearbeitet werden.
```

Wenn Claude Code das Write-Tool ablehnt, endet die Nachricht stattdessen mit `und kann nicht geschrieben werden`.

**Was zu tun ist:**

* Wenn Claude die Datei ändern können sollte, entfernen oder verengen Sie die `Read`-Ablehnungsregel in `/permissions` oder in [Einstellungen](/docs/de/settings-reference#permission-settings)
* Wenn die Datei unverändert bleiben muss, behalten Sie die Regel und fügen Sie eine `Edit`-Ablehnungsregel für denselben Pfad hinzu, um auch das NotebookEdit-Tool zu blockieren

<h3 id="subagent-type-is-required">
  subagent\_type ist erforderlich
</h3>

```text theme={null}
subagent_type ist erforderlich: Der allgemeine Agent ist in dieser Sitzung nicht verfügbar. Verfügbare Agenten: ...
```

Claude hat das [Agent-Tool](/docs/de/tools-reference#agent-tool-behavior) ohne `subagent_type` aufgerufen, und diese Sitzung hat keinen [allgemeinen Subagenten](/docs/de/sub-agents#built-in-subagents) als Fallback. Das ist in zwei Setups der Fall:

* [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/de/env-vars) ist im nicht-interaktiven Modus gesetzt, was jeden integrierten Subagenten entfernt
* Der Hauptthread-Agent der Sitzung hat eine [`tools: Agent(...)`-Zulassungsliste](/docs/de/sub-agents#restrict-which-subagents-can-be-spawned), die `general-purpose` ausschließt

**Was zu tun ist:**

* Normalerweise nichts: Die Nachricht listet die Subagenten auf, die die Sitzung hat, daher kann Claude mit einem von ihnen erneut versuchen
* Wenn Claude weiterhin fehlschlägt, fügen Sie `general-purpose` zur `tools: Agent(...)`-Zulassungsliste hinzu, oder heben Sie `CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS` auf

Vor v2.1.235 schlug derselbe Aufruf mit `Agent-Typ 'general-purpose' nicht gefunden` fehl.

<h3 id="memory-index-is-over-its-read-limit">
  Memory-Index überschreitet sein Lesenlimit
</h3>

Claude schrieb in den [Auto-Memory](/docs/de/memory#auto-memory)-Index `MEMORY.md` und ließ ihn über eines seiner Lesenlimits hinaus: 200 Zeilen oder 25 KB. Der Schreibvorgang war erfolgreich, aber nur die ersten 200 Zeilen oder 25 KB, je nachdem, was zuerst kommt, werden zu Beginn einer Sitzung geladen, daher wird alles über dem Limit jedes Mal gelöscht, wenn der Index gelesen wird. Vor v2.1.210 wurde ein über dem Limit liegender Index beim nächsten Laden stillschweigend gekürzt, ohne ein Schreib-Zeit-Signal.

```text theme={null}
Fehler: Dieser Schreibvorgang ließ den Memory-Index bei MEMORY.md bei 214 Zeilen, über seinem 200-Zeilen-Lesenlimit. Der Schreibvorgang war erfolgreich, aber alles über dem Limit wird stillschweigend gelöscht, jedes Mal wenn der Index geladen wird — Einträge am Ende sind bereits für Leser unsichtbar. Schreiben Sie ihn jetzt auf unter 140 Zeilen um: Behalten Sie eine Zeile pro Eintrag, verschieben Sie Details in Topic-Dateien, und führen Sie stale Einträge zusammen oder löschen Sie sie.
```

Nur der Inhalt, der geladen wird, zählt zu den Limits. YAML-Frontmatter und Block-Level-HTML-Kommentare werden entfernt, bevor der Index geladen wird, daher sind sie von der Messung ausgeschlossen. Vor v2.1.211 maß Claude Code die Rohdatei, und Frontmatter oder Kommentare könnten diesen Fehler auslösen, selbst wenn der geladene Inhalt passte.

Claude Code liefert den Fehler an Claude nach dem Schreibvorgang, anstatt ihn als Banner in Ihrem Terminal zu drucken, daher bemerken Sie ihn möglicherweise nur im Transkript.

Wenn Claudes Schreibvorgang die Datei einem Limit nahe bringt, ohne es zu überschreiten, gibt Claude Code stattdessen eine mildere Erinnerung zurück, um den Index zu komprimieren, anstatt diesen Fehler.

**Was zu tun ist:**

* Lassen Sie Claude `MEMORY.md` umschreiben, oder bitten Sie ihn dazu: Behalten Sie eine Zeile pro Eintrag, verschieben Sie Details in Topic-Dateien, und führen Sie stale Einträge zusammen oder löschen Sie sie
* Um den Index selbst zu kürzen, siehe [Audit und Bearbeitung Ihres Memory](/docs/de/memory#audit-and-edit-your-memory)

<h3 id="pkill-pattern-matches-the-claude-code-process">
  pkill-Muster stimmt mit dem Claude Code-Prozess überein
</h3>

Ein `pkill`-Befehl in einem Bash-Tool-Aufruf verwendete ein Muster, normalerweise mit `-f`, das mit dem Claude Code-Prozess selbst übereinstimmt, daher weigert sich Claude Code, den Befehl auszuführen, anstatt die Sitzung zu beenden. Claude Code testet das Muster mit `pgrep`, bevor `pkill` ausgeführt wird, und weigert sich, wenn seine eigene Prozess-ID im Ergebnis ist. Die Überprüfung läuft nur unter Linux; auf macOS wird `pkill` unverändert ausgeführt. Vor v2.1.214 wurde der Befehl ausgeführt, und ein übereinstimmendes Muster beendete die Claude Code-Sitzung mitten im Zug.

```text theme={null}
pkill: Weigerung auszuführen — dieses Muster stimmt mit dem Claude CLI-Prozess überein (PID 12345). Verengen Sie das Muster, oder zielen Sie auf Ihre eigenen untergeordneten Prozesse mit `pkill -P $$ ...` ab.
```

Die Weigerung erscheint im Bash-Tool-Ergebnis, anstatt als Banner in Ihrem Terminal, und Claude passt den Befehl normalerweise selbst an.

**Was zu tun ist:**

* Verengen Sie das Muster, damit es nur den beabsichtigten Prozess abgleicht, zum Beispiel den vollständigen Pfad der Zieldatei anstelle einer kurzen Teilzeichenfolge
* Um Prozesse zu stoppen, die von der aktuellen Shell gestartet wurden, verwenden Sie `pkill -P $$` mit dem Muster, das die Übereinstimmung auf die untergeordneten Prozesse der Shell selbst beschränkt

<h3 id="failed-to-write-to-a-teammate-inbox">
  Fehler beim Schreiben in den Posteingang eines Teamkollegen
</h3>

Claude Code konnte keine Nachricht in die Postfachdatei eines Teamkollegen unter `~/.claude/teams/{team-name}/inboxes/` schreiben, daher erhielt der Empfänger nichts. Der Schreibvorgang schlägt fehl, wenn Claude Code die Datei nicht erstellen oder aktualisieren kann, zum Beispiel weil die Festplatte voll ist, das Verzeichnis nicht beschreibbar ist oder ein anderer Agent die Inbox-Sperre zu lange hält. Vor v2.1.224 meldete Claude Code die Nachricht als gesendet, selbst wenn der Schreibvorgang fehlschlug.

Der Fehler erscheint im Tool-Ergebnis des sendenden Agenten, anstatt als Banner in Ihrem Terminal, und sein Text teilt Claude mit, es erneut zu versuchen:

```text theme={null}
Fehler beim Schreiben in den Posteingang des Forschers — nichts wurde gesendet. Versuchen Sie es erneut, oder kontaktieren Sie den Lead.
```

Strukturierte [Agent-Team](/docs/de/agent-teams)-Protokollnachrichten schlagen auf die gleiche Weise fehl, und der Fehler benennt die unzugestellte Nachricht: Wenn Claude Code eine Plan-Genehmigung, Plan-Ablehnung, Shutdown-Anfrage oder Shutdown-Ablehnung nicht schreiben kann, liest sich der Fehler `Fehler beim Schreiben der <Nachricht> in den Posteingang von <Name> — nichts wurde gesendet`. Die `Plan-Genehmigung` in dieser Liste ist die Entscheidung des Leads, den Plan eines Teamkollegen zu genehmigen; die Plan-Einreichung des Teamkollegen ist die separate `Plan-Genehmigungsanfrage`-Nachricht. Diese Nachricht und zwei weitere Protokollnachrichten tragen ihren eigenen Nachrichtentext und ihre Konsequenzen:

* `Fehler beim Schreiben der Plan-Genehmigungsanfrage in den Posteingang des Leads — Plan nicht eingereicht; versuchen Sie es erneut`: Der Plan des Teamkollegen erreichte den Lead nie, und der Teamkollege bleibt im Plan-Modus, bis eine Neueinreichung erfolgreich ist
* `Die Berechtigungsanfrage konnte nicht an den Team-Lead zugestellt werden (Postfach-Schreibfehler)`: Die Berechtigungsanfrage des Teamkollegen erreichte den Lead nie, daher genehmigte niemand den Tool-Aufruf
* `Die Bestätigung konnte nicht in den Posteingang des Team-Leads geschrieben werden.`: Die Shutdown-Genehmigung selbst trat in Kraft und der Teamkollege beendet; nur die Bestätigung an den Lead fehlt

Wenn Sie selbst einen Teamkollegen kontaktieren, indem Sie `@name` gefolgt von der Nachricht in der Lead-Sitzung eingeben, erscheint derselbe Fehler als Benachrichtigung, `Konnte nicht in den Posteingang von @name schreiben — Nachricht nicht gesendet. Versuchen Sie es erneut.`, und Claude Code behält Ihren Text im Prompt-Feld, damit Sie ihn erneut senden können.

**Was zu tun ist:**

* Bitten Sie den Absender, die Nachricht erneut zu senden; Contention für die Inbox-Sperre ist vorübergehend und wird beim Wiederholen gelöscht
* Überprüfen Sie den freien Speicherplatz, und überprüfen Sie, dass `~/.claude/teams` und die Dateien darunter von Ihrem Benutzer beschreibbar sind

<h3 id="teammate-agent-definition-not-restored">
  Agent-Definition des Teamkollegen wurde nicht wiederhergestellt
</h3>

Claude kontaktierte einen gestoppten [Agent-Team](/docs/de/agent-teams)-Teamkollegen, und Claude Code brachte ihn zurück, ohne die [Subagenten-Definition](/docs/de/agent-teams#use-subagent-definitions-for-teammates) erneut anzuwenden, von der er erzeugt wurde, weil seine Definitionsdatei aus einem Ordner ohne gespeichertes Vertrauen kam. Die Benachrichtigung folgt dem Wiederaufnahmebericht im Tool-Ergebnis des sendenden Agenten:

```text wrap theme={null}
Seine Agent-Definition wurde nicht wiederhergestellt: Der Ordner, aus dem seine Definitionsdatei stammt, ist nicht vertraut (Quelle: projectSettings), daher läuft der Teamkollege mit den Team-essentiellen Tools und ohne benutzerdefinierte Anweisungen. Um ihn wiederherzustellen, muss der Benutzer Claude Code in diesem Ordner ausführen und den Vertrauensdialog akzeptieren (das --debug-Protokoll benennt den Ordner); ändern Sie die Vertrauenseinstellungen nicht im Namen des Benutzers.
```

Die Überprüfung gilt für eine Definition im `.claude/agents/`-Verzeichnis des Projekts oder eines `--add-dir`-Verzeichnisses, und das Akzeptieren des Vertrauensdialogs für einen übergeordneten Ordner erfüllt ihn nicht.

**Was zu tun ist:**

* Führen Sie `claude` in dem Ordner aus, den das [Debug-Protokoll](/docs/de/debug-your-config) benennt, und akzeptieren Sie den Vertrauensdialog. Die Definition wird erneut angewendet, wenn Claude Code den Teamkollegen das nächste Mal zurückbringt; Sie müssen die Lead-Sitzung nicht neu starten
* Oder setzen Sie den `hasTrustDialogAccepted`-Eintrag auf `true` in `~/.claude.json`, wobei Sie den genauen `projects["<path>"]`-Schlüssel verwenden, den das Debug-Protokoll druckt

<h3 id="message-too-large-for-cross-session-delivery">
  Nachricht zu groß für sitzungsübergreifende Zustellung
</h3>

Claudes [sitzungsübergreifende Nachricht](/docs/de/cross-session-messaging) an eine andere Ihrer Sitzungen auf dieser Maschine war zu lang zum Senden. Claude Code weigerte sich, und die empfangende Sitzung erhielt nichts. Die Weigerung erscheint im Tool-Ergebnis der sendenden Sitzung, nicht als Banner in Ihrem Terminal. Sie benennt beide Größen und wie die Nachricht passt:

```text wrap theme={null}
Fehler beim Senden an api-worker: Nachricht zu groß für sitzungsübergreifende Zustellung: Die serialisierte Nachricht ist 1.203.844 Zeichen und das Limit ist 1.048.576. Kürzen Sie den Nachrichtentext — legen Sie Masseninhalt in eine Datei, die der Empfänger lesen kann, anstatt in die Nachricht — oder teilen Sie ihn in kleinere Nachrichten auf.
```

Das erneute Senden desselben Textes schlägt auf die gleiche Weise fehl.

**Was zu tun ist:**

* Bitten Sie Claude, die Nachricht zusammenzufassen, oder legen Sie den Masseninhalt in eine Datei und senden Sie den Dateipfad
* Bitten Sie Claude, den Inhalt auf mehrere kürzere Nachrichten zu verteilen

Vor v2.1.235 meldete Claude Code eine übergroße Nachricht als gesendet. Die empfangende Sitzung ließ sie ungelesen fallen.

<h3 id="too-many-messages-to-this-session-just-now">
  Zu viele Nachrichten an diese Sitzung gerade eben
</h3>

Claude sendete einen schnellen Schub von [sitzungsübergreifenden Nachrichten](/docs/de/cross-session-messaging) an eine Ihrer Sitzungen auf dieser Maschine, und der Schub erreichte das, was diese Sitzung akzeptiert. Claude Code weigerte sich, die nächste zu senden, und die empfangende Sitzung erhielt nichts davon. Die Weigerung erscheint im Tool-Ergebnis der sendenden Sitzung, nicht als Banner in Ihrem Terminal:

```text wrap theme={null}
Fehler beim Senden an api-worker: Zu viele Nachrichten an diese Sitzung gerade eben: 30 wurden kürzlich gesendet und mehr würden durch sein Ratenlimit gelöscht, daher wurde diese nicht gesendet. Fassen Sie das Verbleibende in eine Nachricht zusammen, oder warten Sie ein wenig, bevor Sie mehr senden.
```

**Was zu tun ist:**

* Normalerweise nichts: Claude fasst den verbleibenden Inhalt in eine Nachricht zusammen, oder wartet, bevor mehr gesendet wird
* Wenn Sie den Schub selbst ausgelöst haben, bitten Sie Claude, das Verbleibende in eine einzelne Nachricht zu kombinieren

Vor v2.1.236 meldete Claude Code diese Sends als gesendet. Die empfangende Sitzung ließ sie ungelesen fallen.

<h3 id="refusing-to-send-a-cross-session-message">
  Weigerung, eine sitzungsübergreifende Nachricht zu senden
</h3>

Bevor Claude Code eine [sitzungsübergreifende Nachricht](/docs/de/cross-session-messaging) an eine andere Ihrer Sitzungen auf dieser Maschine schreibt, überprüft es, dass die Inbox-Socket der Ziel-Sitzung der Endpunkt ist, an den die Nachricht adressiert wurde. Wenn eine Überprüfung fehlschlägt, weigert sich Claude Code, den Send in der sendenden Sitzung zu tätigen, und die Ziel-Sitzung erhält nichts. Für eine Nachricht, die Claude sendet, erscheint die Weigerung im Tool-Ergebnis der sendenden Sitzung:

```text theme={null}
Fehler beim Senden an api-worker: Weigerung zu senden: Antwortziel ist ein Symlink
```

Der Text nach `Weigerung zu senden:` benennt die Überprüfung, die fehlgeschlagen ist:

* `Antwortziel ist ein Symlink`: Ein symbolischer Link sitzt am Socket-Pfad der Ziel-Sitzung. Claude Code liefert nicht durch ihn, weil ein Link dort die Nachricht zu einem Endpunkt umleiten könnte, den die Ziel-Sitzung nicht erstellt hat.
* `Antwortziel kann nicht überprüft werden`: Claude Code konnte den Zielpfad überhaupt nicht überprüfen, zum Beispiel weil das Lesen mit einem Berechtigungsfehler fehlschlug.
* `Verbundener Endpunkt ist nicht der erwartete Prozess`: Der Prozess, der den Socket hält, ist nicht die Sitzung, an die die Nachricht adressiert wurde, daher ist die Adresse veraltet oder ein anderer Prozess hat den Socket ersetzt.
* `Verbundene Endpunkt-Identität konnte nicht gelesen werden`: Claude Code verbunden, konnte aber nicht lesen, welcher Prozess das andere Ende hält, daher konnte es das Ziel nicht bestätigen. Dies kann vorübergehend sein.
* `Verbundener Endpunkt wird nicht von diesem Benutzer besessen`: Der Prozess, der den Socket hält, läuft unter einem anderen Benutzerkonto, daher ist es nicht eine Ihrer Sitzungen.
* `Verbundene Endpunkt-Besitzer konnte nicht gelesen werden`: Claude Code verbunden, konnte aber nicht lesen, welches Benutzerkonto das andere Ende besitzt, daher konnte es nicht bestätigen, dass der Endpunkt Ihnen gehört.
* `Verbundener Endpunkt ist ein anderer Prozess mit der erwarteten PID`: Die Prozess-ID stimmt mit der überein, an die die Nachricht adressiert wurde, aber Claude Code konnte nicht bestätigen, dass es derselbe Prozess ist. Normalerweise hat diese Sitzung beendet und das Betriebssystem hat ihre Prozess-ID wiederverwendet, daher ist die Adresse veraltet.

**Was zu tun ist:**

* Normalerweise nichts: Die Überprüfungen verhindern, dass eine Nachricht einen anderen Endpunkt als die Sitzung erreicht, an die sie adressiert wurde, und nichts wurde gesendet
* Bitten Sie Claude, Ihre Sitzungen erneut aufzulisten und erneut zu senden; eine Weigerung, die durch eine veraltete Adresse verursacht wird, wird gelöscht, sobald Claude an die aktuelle sendet
* Wenn `Antwortziel ist ein Symlink` für eine Sitzung wiederholt wird, überprüfen Sie, was einen Link am Socket-Pfad dieser Sitzung erstellt hat, angezeigt in ihrem `/status` unter `Peer address`
* Für `Verbundene Endpunkt-Identität konnte nicht gelesen werden`, erneut senden; die Bedingung kann vorübergehend sein
* Wenn `Verbundener Endpunkt wird nicht von diesem Benutzer besessen` auf einer gemeinsamen Maschine erscheint, läuft die Sitzung unter diesem Endpunkt unter einem anderen Benutzerkonto, daher kann Claude sie nicht von Ihrem aus kontaktieren

Vor v2.1.248 überprüfte Claude Code nicht den Besitzer des Endpunkts oder die Prozess-Startzeit, daher erscheinen die Verweigerungen, die diese Überprüfungen benennen, nicht auf früheren Versionen.

<h3 id="refusing-after-a-symlink-changed">
  Weigerung, einen Pfad zu lesen, zu schreiben oder zu durchsuchen
</h3>

Claude Code überprüft die [Berechtigungsregeln](/docs/de/permissions#read-and-edit) eines Dateipfads, bestätigt dann diese Auflösung erneut, wenn das Tool die Datei öffnet oder die Suche startet. Wenn es nicht bestätigen kann, dass der Pfad immer noch zu dem Ort führt, den die Überprüfung genehmigt hat, weigert sich Claude Code, die Operation auszuführen, anstatt sie zu folgen. Die Weigerung erscheint im Tool-Ergebnis:

```text wrap theme={null}
Weigerung zu lesen /path/to/file: Seine Symlink-Auflösung änderte sich nach der Berechtigungsprüfung (ein Link auf dem Weg führt jetzt irgendwohin, das die Überprüfung nicht sah). Wenn ein Link im Arbeitsverzeichnis gleichzeitig umgeschrieben wird, stoppen Sie das und versuchen Sie es erneut.
```

Jede Weigerung benennt ihren Grund:

* `Seine Symlink-Auflösung änderte sich nach der Berechtigungsprüfung`: Ein Symlink entlang des Pfads, oder bei einer Grep- oder Glob-Suchwurzel, wurde zwischen der Berechtigungsprüfung und der Operation ersetzt. In einer Leseverweigerung benennt der eingeklammerte Satz, welcher Vergleich fehlschlug.
* `Seine Symlink-Auflösung des übergeordneten Verzeichnisses änderte sich nach der Berechtigungsprüfung`: Ein Verzeichnis, das der Schreibpfad durchläuft, wird nicht mehr zu dem genehmigten Ort aufgelöst
* `Es ist ein symbolischer Link. Schreiben Sie stattdessen zum Ziel-Pfad des Links`: Ein Symlink sitzt am genehmigten Schreibort selbst, zum Beispiel eine `CLAUDE.md`, die ein Symlink zu `AGENTS.md` ist; die Nachricht leitet Claude zum Ziel des Links
* `Weigerung, durch Symlink zu schreiben: <path>. Lösen Sie den Symlink auf und übergeben Sie den echten Ziel-Pfad explizit.`: dieselbe Bedingung, die erfasst wird, wenn ein anderer Schreiber die Datei öffnet, wie ein Schreiben zu einer verlinkten `.mcp.json`
* `Weigerung, in ein verlinktes Verzeichnis zu schreiben: <path>`: Das Verzeichnis, das die Datei hält, ist selbst ein symbolischer Link, zum Beispiel das `.claude/`-Verzeichnis eines Projekts, das mit einem anderen Ort verlinkt ist
* `Ein Pfad, durch den eine seiner Read-Ablehnungsregeln geschrieben wird, änderte sich, während die Suche vorbereitet wurde. Versuchen Sie es erneut.`: Eine `Read`-Ablehnungsregel für die Suche benennt einen Pfad, der durch einen Symlink führt, und dieser Link änderte sich, während Claude Code die Suche vorbereitete
* `Es konnte nicht geöffnet werden (EACCES) — es ist nicht lesbar, oder wird gleichzeitig ersetzt.`: Die Suchwurzel existiert, konnte aber nicht geöffnet werden; der eingeklammerte Code ist der Betriebssystemfehler
* `Seine Berechtigungsprüfung lief ab, bevor sie lief (zu viele gleichzeitige Dateivorgänge). Versuchen Sie es erneut.`: Claude Code räumte den Genehmigungsdatensatz unter vielen gleichzeitigen Dateivorgängen auf, bevor das Tool ihn verwendete; das Wiederholen führt eine frische Berechtigungsprüfung durch
* `ripgrep wurde nur nach Name auf PATH gefunden, und eine Suche außerhalb des Arbeitsverzeichnisses kann Ihre Read-Ablehnungsregeln in dieser Konfiguration nicht anwenden`: Claude Code konnte die `rg`-Binärdatei nicht zu einem absoluten Pfad auflösen, daher weigert es sich, Suchen außerhalb des Arbeitsverzeichnisses auszuführen, anstatt eine auszuführen, die Ihre Ablehnungsregeln nicht abdecken

**Was zu tun ist:**

* Normalerweise nichts: Die Weigerung erreicht Claude als das Tool-Ergebnis, und die abgelehnte Operation wird nicht ausgeführt
* Wenn eine Symlink-Weigerung auf einem Pfad wiederholt wird, finden Sie, was einen Link dort ständig umschreibt, wie ein Build-Tool oder File-Watcher, oder bitten Sie Claude, den aufgelösten Pfad der Datei anstelle des verlinkten zu verwenden
* Wenn diese Weigerung für jede Datei erscheint, während Claude Code unter Windows in einem AppContainer oder Sandbox mit eingeschränktem Token läuft, aktualisieren Sie auf v2.1.265 oder später
* Wenn eine Leseverweigerung auf macOS für eine Datei erscheint, die nichts umschreibt, wie ein Screenshot, der in den Prompt gezogen wird, aktualisieren Sie auf v2.1.273 oder später
* Für die ripgrep-Weigerung installieren Sie ripgrep mit Ihrem Paketmanager, damit `rg` zu einem absoluten Pfad auf `PATH` aufgelöst wird, oder halten Sie Suchen unter dem Arbeitsverzeichnis

Vor v2.1.251 überprüfte Claude Code die Auflösung eines Pfads nur für Dateischreibvorgänge erneut, daher konnte ein Link, der nach der Berechtigungsprüfung ersetzt wurde, eine Lese- oder Suche zu einem anderen Ort umleiten, ohne eine Nachricht. Von diesen Verweigerungen erscheint nur die Schreibverweigerung des übergeordneten Verzeichnisses auf früheren Versionen.

<h3 id="task-output-swap-refused">
  Task-Ausgabe-Swap abgelehnt
</h3>

Claude Code speichert die Ausgabe jedes Bash-Befehls in einer Datei unter seinem Temp-Verzeichnis. Jedes Mal, wenn es eine dieser Dateien öffnet, überprüft es, dass der Pfad immer noch zu der Datei führt, die es erstellt hat, ohne Symlink, zusätzlichen Hard-Link oder verschobenes Verzeichnis, das ihn umleitet. Diese Nachricht bedeutet, dass diese Überprüfung fehlgeschlagen ist, daher weigerte sich Claude Code, die Operation auszuführen, anstatt Ausgabe durch diesen Pfad zu schreiben oder zu lesen. Die Nachricht erscheint im Bash-Tool-Ergebnis:

```text wrap theme={null}
Task-Ausgabe-Swap abgelehnt (Tasks-Verzeichnis verschoben oder verlinkt): /private/tmp/claude-501/-Users-you-my-project/1f0e62dc-4b0a-4f5e-9c2d-8a7b6c5d4e3f/tasks/b7k2f9m3q.output. Zur Wiederherstellung: Starten Sie Claude Code mit CLAUDE_CODE_TMPDIR neu, das auf ein frisches Verzeichnis gesetzt ist; oder, wenn /private/tmp/claude-501/-Users-you-my-project ein verwaistes Verzeichnis oder ein Symlink ist, der nicht dort sein sollte, entfernen Sie diesen Eintrag selbst (nicht das, worauf er zeigt) und starten Sie neu.
```

Der eingeklammerte Text benennt die Überprüfung, die fehlgeschlagen ist. Gründe wie `Ausgabe-Symlink wurde umgeleitet`, `Ausgabedatei-Identität änderte sich` und `keine reguläre Datei` melden alle dieselbe Bedingung: Etwas am oder entlang des Ausgabepfads ist nicht mehr die Datei, die Claude Code erstellt hat. Nur einige Gründe tragen einen `Zur Wiederherstellung:`-Satz.

Wenn die Überprüfung fehlschlägt, während ein Befehl noch läuft, stoppt Claude Code den Befehl, und sein Ergebnis meldet:

```text theme={null}
Befehl beendet: Seine Ausgabedatei wurde ersetzt oder konnte nicht mehr überprüft werden
```

**Was zu tun ist:**

* Aktualisieren Sie auf v2.1.260 oder später. Frühere Versionen zeigten diese Nachricht manchmal, wenn kein Link oder verschobenes Verzeichnis vorhanden war
* Starten Sie Claude Code mit [`CLAUDE_CODE_TMPDIR`](/docs/de/env-vars) neu, das auf ein frisches Verzeichnis gesetzt ist
* Oder überprüfen Sie Ihr Projektverzeichnis unter dem Claude Code-Temp-Verzeichnis, `/private/tmp/claude-501/-Users-you-my-project` in der Beispielnachricht. Wenn dieser Pfad ein Symlink ist, oder ein Verzeichnis, das nicht dort sein sollte, entfernen Sie den Link oder das Verzeichnis selbst, anstatt das Ziel des Links, und starten Sie Claude Code neu
* Wenn die Weigerung wiederholt wird, ersetzt, verlinkt oder entfernt ein Prozess Einträge unter Claudes Temp-Verzeichnis, während die Sitzung läuft. Setzen Sie [`CLAUDE_CODE_TMPDIR`](/docs/de/env-vars) auf ein Verzeichnis, das nichts anderes verwaltet, und starten Sie neu

<h3 id="the-source-file-is-not-valid-utf-8-text">
  Die Quelldatei ist kein gültiger UTF-8-Text
</h3>

Claude versuchte, ein [Artefakt](/docs/de/artifacts) aus einer Datei zu veröffentlichen, deren Bytes nicht als Text dekodiert werden, oder deren Text bereits das Ersatzzeichen `U+FFFD` enthält, daher weigerte sich Claude Code, die Veröffentlichung zu tätigen, bevor etwas hochgeladen wurde. Die Nachricht erscheint im Artifact-Tool-Ergebnis und benennt die erste Position zum Beheben:

```text wrap theme={null}
file_path: Die Quelldatei ist kein gültiger UTF-8-Text (erstes ungültiges Byte bei Zeile 12, Spalte 40). Sie kann in einer anderen Kodierung gespeichert sein oder Binärdaten enthalten. Schreiben Sie sie als UTF-8 um, dann veröffentlichen Sie erneut. Nichts wurde veröffentlicht.

file_path: Die Quelldatei hat das Ersatzzeichen U+FFFD bei Zeile 12, Spalte 40, normalerweise dort gelassen, wo eine frühere Bearbeitung oder ein Einfügen ein Zeichen verlor. Ersetzen Sie es durch den beabsichtigten Text (in HTML schreiben Sie ein beabsichtigtes U+FFFD als &#xFFFD;), dann veröffentlichen Sie erneut. Nichts wurde veröffentlicht.
```

Claude Code dekodiert die Datei als UTF-8, oder als UTF-16, wenn sie mit einer Little-Endian-UTF-16-Byte-Order-Mark beginnt. Wenn eine solche UTF-16-Datei nicht dekodiert, benennt die erste Nachricht `UTF-16` und teilt Ihnen immer noch mit, die Datei als UTF-8 umzuschreiben. Wenn mehr Positionen der benannten folgen, fügt die Nachricht eine Zählung wie `(+2 weitere)` nach der Position hinzu.

**Was zu tun ist:**

* Normalerweise nichts: Claude schreibt die Datei um und veröffentlicht erneut
* Wenn die Datei eine ist, die Sie geschrieben oder exportiert haben, speichern Sie sie erneut als UTF-8, und ersetzen Sie jedes `U+FFFD` durch das Zeichen, das eine frühere Bearbeitung, ein Einfügen oder eine Konvertierung verlor
* Um ein beabsichtigtes `U+FFFD` auf der Seite anzuzeigen, schreiben Sie es als `&#xFFFD;` im HTML, anstatt das Literalzeichen

Vor v2.1.267 lud Claude Code eine solche Datei ohne Überprüfung hoch, und der Server lehnte die Veröffentlichung stattdessen ab.

<h3 id="reading-a-local-file-from-outside-the-connected-folders">
  Lesen einer lokalen Datei von außerhalb der verbundenen Ordner in einer Cowork-Sitzung
</h3>

In einer [Cowork](https://claude.com/docs/cowork/overview)-Sitzung, die auf Ihrer Maschine in der Claude Desktop-App läuft, benannte Claude eine lokale Datei für ein [Artefakt](/docs/de/artifacts). Claude Code konnte nicht bestätigen, dass die Datei eine einfache Datei in den verbundenen Ordnern der Sitzung ist: Der Pfad sitzt außerhalb dieser Ordner, führt durch einen Symlink, oder ist so geschrieben, dass er eine andere Datei als die, die er zu sein scheint, benennen kann. Das Lesen einer solchen Datei benötigt Ihre Genehmigung, und in einer Sitzung, die Ihnen die Genehmigungskarte nicht zeigen kann, wie eine, die auf das Überspringen aller Genehmigungen eingestellt ist, weigert sich Claude Code, die Datei zu lesen.

Die Weigerung erscheint im Artifact-Tool-Ergebnis; wenn die Datei überhaupt nicht untersucht werden konnte, benennt sie stattdessen diesen Fehler:

```text wrap theme={null}
Das Lesen einer lokalen Datei von außerhalb der verbundenen Ordner dieser Sitzung, oder durch einen Link, benötigt die Genehmigungskarte, und niemand kann sie in dieser Cowork-Sitzung beantworten. Verwenden Sie eine einfache Datei in den verbundenen Ordnern; versuchen Sie diese Datei nicht erneut in dieser Sitzung.

Kann file_path nicht lesen (ENOENT) — die Datei konnte nicht untersucht werden, und niemand kann die Genehmigungskarte in dieser Cowork-Sitzung beantworten. Überprüfen Sie, dass die Datei als einfache Datei in den verbundenen Ordnern existiert, dann versuchen Sie es mit diesem Pfad erneut.
```

**Was zu tun ist:**

* Normalerweise nichts: Die Nachricht teilt Claude mit, stattdessen eine einfache Datei in den verbundenen Ordnern zu verwenden
* Um diese genaue Datei in das Artefakt zu legen, kopieren Sie sie als reguläre Datei, keinen Symlink, in einen der verbundenen Ordner der Sitzung, und fragen Sie erneut

<h3 id="webfetch-cannot-fetch-localhost">
  WebFetch kann localhost nicht abrufen
</h3>

Claude hat [WebFetch](/docs/de/tools-reference#webfetch-tool-behavior) mit einer URL aufgerufen, deren Hostname keinen Punkt hat, wie `http://localhost:3000` oder einen bloßen Intranet-Namen wie `http://wiki/`. WebFetch weigert sich, diese URLs vor jedem Anfrage zu machen:

```text wrap theme={null}
WebFetch kann localhost oder andere Hostnamen ohne Punkt nicht abrufen. Um einen lokalen Server zu erreichen, verwenden Sie stattdessen Bash mit curl.
```

**Was zu tun ist:**

* Normalerweise nichts: Die Nachricht zeigt Claude auf `curl` durch das Bash-Tool, das lokale und Intranet-Server erreichen kann

Vor v2.1.268 meldete WebFetch diese URLs mit einem generischen `Ungültige URL`-Fehler.

<h2 id="background-session-errors">
  Fehler in Hintergrund-Sitzungen
</h2>

[Hintergrund-Sitzungen](/docs/de/agent-view) laufen ohne eigenes interaktives Terminal, daher verhalten sich Befehle, die eines benötigen, dort anders. Diese Meldungen erscheinen im Transkript einer Hintergrund-Sitzung, im Terminal, das sich an eine angehängt, in der Sitzung oder Shell, von der Sie sie entsenden, oder für die [worktree-guard-Einträge](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) unten in jeder Sitzung, die in einem Worktree isoliert ist oder einen Worktree-isolierten Subagenten ausführt; wenn eine Meldung spezifisch für eine Oberfläche ist, gibt ihr Eintrag das an.

<h3 id="commands-refused-in-a-background-session">
  Befehle, die in einer Hintergrund-Sitzung abgelehnt werden
</h3>

Befehle, die einen interaktiven Dialog öffnen, können dies nicht tun, während kein Terminal an eine Hintergrund-Sitzung angehängt ist. `/install-github-app`, die `/mcp`-Einstellungsliste und die Authentifizierungsaktionen im MCP-Server-Menü antworten mit einer Meldung, und die Sitzung wird unter **Needs input** in der [Agent-Ansicht](/docs/de/agent-view) angezeigt, damit Sie sie finden, anhängen und den Befehl erneut ausführen können. Während ein Terminal angehängt ist, funktionieren diese Befehle normal.

Vor v2.1.216 wurde die Sitzung nach einer dieser Ablehnungen nicht unter **Needs input** angezeigt. In v2.1.213 bis v2.1.215 funktionieren die Befehle weiterhin, während ein Terminal angehängt ist, und die Ablehnungsmeldung sagte Ihnen, dass Sie anhängen und den Befehl erneut ausführen sollen. Von v2.1.208 bis v2.1.212 lehnte Claude Code sie sogar ab, während ein Terminal angehängt war, mit einer Meldung wie `Can't open MCP settings in a background session`; auf diesen Versionen führen Sie den Befehl stattdessen aus einer regulären `claude`-Sitzung aus oder aktualisieren. Vor v2.1.208 öffneten sie ihren Dialog innerhalb der Hintergrund-Sitzung. In v2.1.208 nur lehnte Claude Code auch die `/model`-Auswahl in einer Hintergrund-Sitzung ab, und `/upgrade` druckte die Upgrade-URL aus, anstatt einen Browser zu öffnen.

Die Formulierung nennt den Befehl. Die `/mcp`-Einstellungsliste meldet:

```text theme={null}
Can't open MCP settings while no terminal is attached to this background session. This session now shows "needs input" in agent view — open it and run /mcp to manage servers, or use `/mcp enable|disable|reconnect <server>` to steer without the panel.
```

**Was zu tun ist:**

* Hängen Sie sich von der Agent-Ansicht an die Sitzung an, wo sie unter **Needs input** aufgelistet ist, und führen Sie den Befehl erneut aus
* Oder verwenden Sie das Formular, das die Meldung nennt, wie `/mcp reconnect <server>`, `/mcp enable` oder `/mcp disable`, die ohne Anhängen funktionieren

<h3 id="write-or-command-blocked-because-the-path-cannot-be-safely-resolved">
  Schreib- oder Befehlsoperation blockiert, da der Pfad nicht sicher aufgelöst werden kann
</h3>

Claude hat auf eine Datei oder ein Arbeitsverzeichnis durch eine Schreibweise zugegriffen, die der [Worktree-Isolations-Guard](/docs/de/agent-view#how-file-edits-are-isolated) nicht zu einem verifizierbaren Ort auflösen kann. Der Guard überprüft Schreibvorgänge und Befehlsarbeitsverzeichnisse in [jeder Sitzung, die in einem Worktree isoliert ist](/docs/de/worktrees#how-claude-code-enforces-isolation), interaktiv oder im Hintergrund, und in [Worktree-isolierten Subagenten](/docs/de/worktrees#isolate-subagents-with-worktrees). Er löst Symlinks auf, bevor er überprüft, dass die Operation den gemeinsamen Checkout nicht erreicht, und wenn die Auflösung fehlschlägt, blockiert er die Operation, anstatt sie dort landen zu lassen. Die Meldung nennt die Pfadformen, die er ablehnt, und wie Sie erneut versuchen:

```text theme={null}
This write was blocked because the path is spelled in a form that cannot be safely resolved (for example through a symlink storing a raw dot segment, a network-share or device-namespace shape, or an unreadable ancestor directory). If the file is inside the worktree /path/to/worktree, address it by its direct symlink-free path instead.
```

Ein blockierter Befehl meldet die gleiche Ursache für sein Arbeitsverzeichnis und endet mit `re-run the command from its direct symlink-free path`. Vor v2.1.217 verglich der Guard Pfadschreibweisen ohne Symlinks aufzulösen, daher wurden diese Schreibweisen nicht blockiert und ein Schreibvorgang, der durch einen Symlink geleitet wurde, konnte im gemeinsamen Checkout landen.

**Was zu tun ist:**

* Normalerweise nichts: Die vollständige Meldung geht an Claude als Werkzeugfehler, und Claude versucht es erneut mit dem direkten Pfad, den sie nennt. Für einen blockierten Datei-Edit zeigt die Gesprächsansicht nur eine kurze `Error editing file`-Zeile; die vollständige Meldung erscheint in der Transkript-Ansicht, die Sie mit `Ctrl+O` öffnen. Ein blockierter Befehl druckt ihn in seiner Befehlsausgabe.
* Wenn die Blockierung bei derselben Datei wiederholt wird, läuft der Pfad wahrscheinlich durch einen committeten Symlink, dessen Ziel `..` enthält, wie `docs/current -> ../README.md`; bitten Sie Claude, die Zieldatei stattdessen über ihren echten Pfad zu bearbeiten, anstatt über den Link

<h3 id="write-or-command-blocked-because-the-path-names-a-network-location">
  Schreib- oder Befehlsoperation blockiert, da der Pfad einen Netzwerkort benennt
</h3>

Claude hat auf eine Datei oder ein Arbeitsverzeichnis durch einen Pfad zugegriffen, der ein Laufwerk benennt, das sich nicht auf Ihrem Computer befindet, eine UNC-Freigabe wie `\\server\share\file` oder einen `/net`-Automount-Pfad, während der Checkout der Sitzung auf einer lokalen Festplatte ist. Der gleiche [Worktree-Isolations-Guard](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved) kann nicht überprüfen, dass ein solcher Pfad aus dem gemeinsamen Checkout herausbleibt, daher blockiert er die Operation. Das Isolieren der Sitzung in einem Worktree hebt die Blockierung nicht auf. Die Meldung nennt die Pfadform, die stattdessen verwendet werden soll:

```text theme={null}
This write was blocked because the path is network-shaped (a UNC share or /net automount spelling) while this session's checkout is local. Isolating cannot unblock it. If the file is genuinely inside the worktree /path/to/worktree, address it by its local, plainly-spelled path instead.
```

Ein blockierter Befehl meldet die gleiche Ursache für sein Arbeitsverzeichnis und endet mit `re-run the command from its local, plainly-spelled path`. Vor v2.1.217 verglich der Guard nur Pfadtext, daher wurde das Adressieren einer Datei innerhalb des Checkouts über einen UNC- oder `/net`-Pfad nicht blockiert.

**Was zu tun ist:**

* Normalerweise nichts: Claude versucht es erneut mit der lokalen Schreibweise, die die Meldung verlangt
* Wenn sich die Datei auf einer Netzwerkfreigabe befindet, anstatt eine lokale Datei mit einem Netzwerkpfad zu schreiben, befindet sie sich außerhalb des lokalen Arbeitsbereichs der Sitzung; bearbeiten Sie sie stattdessen aus einer regulären interaktiven Sitzung

<h3 id="command-blocked-by-the-worktree-isolation-checks">
  Befehl blockiert durch die Worktree-Isolationsprüfungen
</h3>

Claude hat einen Bash- oder Monitor-Befehl in einer [Sitzung ausgeführt, die in einem Worktree isoliert ist](/docs/de/worktrees#how-claude-code-enforces-isolation), und Claude Code hat ihn aus einem von zwei Gründen abgelehnt:

* Der Befehl zeigt git auf den Haupt-Checkout.
* Claude Code kann aus dem Befehlstext nicht überprüfen, dass jedes git, das der Befehl ausführt, innerhalb des Worktree bleibt. Ein Befehl, der git nie benennt, kann trotzdem aus diesem Grund abgelehnt werden, da das Erweitern einer Variablenumleitung wie `${!name}` oder das Ausführen einer Bash-Funktionssubstitution wie `${ command; }` einen Wert zur Laufzeit erzeugt, der selbst ein Befehl sein kann.

Die Mitte der Meldung nennt, was nicht überprüft werden konnte:

```text wrap theme={null}
This session is isolated in the worktree /path/to/worktree, but this command evaluates ${!x@P} arithmetically inside a construct too complex to verify, which can run a command hidden in a variable's value. Refusing to run it — a worktree-isolated session's git operations must target its own worktree. Split it into plain, separate commands and run them from /path/to/worktree.
```

**Was zu tun ist:**

* Normalerweise nichts: Claude liest die Meldung und schreibt den Befehl so um, wie sein letzter Satz verlangt
* Wenn ein Befehl, den Sie angefordert haben, weiterhin abgelehnt wird, schreiben Sie den gekennzeichneten Wert wörtlich: ersetzen Sie die Umleitung oder Substitution durch ihren Wert, und führen Sie git als seinen eigenen einfachen Befehl von innerhalb des Worktree aus
* Um auf den Haupt-Checkout absichtlich zu handeln, führen Sie den Befehl selbst in einem Terminal außerhalb der Sitzung aus

<h3 id="this-session-has-no-saved-transcript">
  Diese Sitzung hat kein gespeichertes Transkript
</h3>

Sie haben sich an eine gestoppte [Hintergrund-Sitzung](/docs/de/agent-view) angehängt, die mit `←` oder `/background` von einem anderen Gespräch in den Hintergrund verschoben wurde und gestoppt wurde, bevor ihre erste Antwort fertig war. Bis diese erste Antwort fertig ist, lebt das Gespräch noch nur in der Sitzung, von der es in den Hintergrund verschoben wurde, daher lehnt `claude attach` ab, die gestoppte Sitzung zu starten, anstatt ein leeres Gespräch unter derselben Sitzungs-ID zu beginnen. Die Meldung endet mit dem `claude respawn`-Befehl für diese Sitzung:

```text theme={null}
This session has no saved transcript — it was stopped before its first response finished. If it was backgrounded from another conversation, that one is still intact; `claude respawn <id>` starts this one fresh.
```

Das Öffnen der gleichen Sitzungszeile in der [Agent-Ansicht](/docs/de/agent-view) zeigt `Press enter again to restart this session fresh` unter der Liste an, und ein zweites `Enter` auf der Zeile startet die Sitzung mit einem leeren Gespräch neu. Vor v2.1.212 zeigte das Öffnen der gestoppten Sitzung die Ablehnungsmeldung ohne Möglichkeit, sie von der Agent-Ansicht aus neu zu starten. Vor v2.1.211 startete das Öffnen der gestoppten Sitzung stillschweigend dieses leere Gespräch und konnte die ursprüngliche Eingabeaufforderung der Sitzung erneut ausführen.

**Was zu tun ist:**

* Das Gespräch, das Sie in den Hintergrund verschoben haben, ist intakt: setzen Sie es mit [`claude --resume`](/docs/de/sessions) fort oder arbeiten Sie weiterhin darin
* Um die gestoppte Sitzung trotzdem neu zu starten, führen Sie `claude respawn <id>` mit der ID aus der Meldung aus, oder drücken Sie `Enter` zweimal auf ihrer Zeile in der Agent-Ansicht
* Wenn die Sitzung eine Antwort fertig gestellt hat und Sie diese Ablehnung immer noch auf einer Version vor v2.1.214 sehen, könnte ein unlesbarer Ordner in `~/.claude/projects` dazu führen, dass der Transkript-Scan die gespeicherte Konversation verpasst; aktualisieren Sie auf v2.1.214 oder später, das unlesbare Ordner während des Scans toleriert

<h3 id="this-session-is-running-in-another-terminal">
  Diese Sitzung läuft in einem anderen Terminal
</h3>

Sie haben die Zeile einer gestoppten Sitzung in der [Agent-Ansicht](/docs/de/agent-view) geöffnet, und ihr gespeichertes Gespräch ist bereits in einem anderen aktiven Claude Code-Prozess auf diesem Computer offen, daher lehnt Claude Code ab, einen zweiten Prozess zu starten, der in das gleiche Transkript schreiben würde. Welche Meldung Sie sehen, hängt davon ab, [was das Gespräch hält](/docs/de/agent-view#opening-a-session-says-the-conversation-is-already-open):

```text theme={null}
Can't open — this session is running in another terminal
This conversation is already open in another running Claude session — use that one, or close it and try again
```

* **`running in another terminal`**: Ein Terminal hält das Gespräch, zum Beispiel eines, in dem Sie es mit `claude --resume` oder `/resume` fortgesetzt haben. Die Zeile zeigt auch `Open in a terminal`.
* **`already open in another running Claude session`**: Ein anderer nicht-interaktiver Claude Code-Prozess hält es, zum Beispiel ein [Hintergrund-Sitzungs](/docs/de/agent-view#the-supervisor-process)-Prozess für das gleiche Gespräch, das noch nicht beendet ist.

Claude Code speichert eine Antwort, die Sie beim Öffnen der Zeile eingegeben haben, und sendet sie als nächste Eingabeaufforderung der Sitzung, wenn die Sitzung das nächste Mal startet.

**Was zu tun ist:**

* Setzen Sie das Gespräch in dem Prozess fort, der es offen hat, oder beenden Sie diesen Prozess und öffnen Sie die Zeile erneut

Vor v2.1.248 existierte nur die `already open in another running Claude session`-Ablehnung: Ein Gespräch, das in einem Terminal fortgesetzt wurde, zählte nicht als offen, und das Öffnen der Zeile startete einen zweiten Claude Code-Prozess, der in das gleiche Gespräch schrieb.

<h3 id="this-sessions-saved-conversation-is-no-longer-on-disk">
  Das gespeicherte Gespräch dieser Sitzung ist nicht mehr auf der Festplatte
</h3>

Sie haben eine [Hintergrund-Sitzung](/docs/de/agent-view) geöffnet, die endete, während der Hintergrund-Service aus war, und die [Transkript-Bereinigung](/docs/de/settings-reference#cleanupperioddays) hat seitdem sein gespeichertes Gespräch entfernt, zum Beispiel nachdem der Computer für Wochen aus war. Das Öffnen einer solchen Zeile [setzt normalerweise sein gespeichertes Gespräch fort](/docs/de/agent-view#sessions-show-as-failed-after-shutdown). Mit nichts zum Fortsetzen lehnt Claude Code ab, anstatt die ursprüngliche Eingabeaufforderung der Sitzung ohne Nachfrage erneut auszuführen:

```text theme={null}
This session's saved conversation is no longer on disk (it ended while the background service was off, and old transcripts are cleaned up), so there is nothing to resume. `claude rm 7c5dcf5d` deletes the row; `claude respawn 7c5dcf5d` runs its original prompt again instead.
```

`claude attach <id>` druckt diesen Text. In der Agent-Ansicht ist die Fußzeile kürzer und endet mit `ctrl+x deletes the row`.

**Was zu tun ist:**

* Führen Sie `claude rm <id>` aus, um die Zeile zu löschen. Wenn einer der [beibehaltenen Fälle](/docs/de/agent-view#what-deleting-a-session-removes) zutrifft, behält `claude rm` die Zeile und den Worktree stattdessen bei und nennt den Grund
* Um die ursprüngliche Eingabeaufforderung der Sitzung erneut als frisches Gespräch auszuführen, führen Sie `claude respawn <id>` aus

Vor v2.1.248 führte das Öffnen einer solchen Zeile die ursprüngliche Eingabeaufforderung der Sitzung erneut aus, anstatt abzulehnen, und zog eine Wochen alte Aufgabe in den Vordergrund.

<h3 id="worktree-has-commits-that-are-not-pushed-anywhere">
  Worktree hat Commits, die nirgendwo gepusht werden
</h3>

Sie haben versucht, eine [Hintergrund-Sitzung](/docs/de/agent-view#what-deleting-a-session-removes) zu löschen, deren Worktree Commits enthält, die Claude Code nicht bestätigen kann, dass sie anderswo gespeichert sind. Claude Code behält den Worktree und die Sitzungszeile bei, anstatt die Commits zu zerstören. `claude rm` nennt den Branch und die nicht gepushten Commits und sagt, wie Sie vorgehen:

```text theme={null}
kept 7c5dcf5d — its worktree is still at "/home/you/project/.claude/worktrees/fix-login"
  2 unpushed commits on "claude/fix-login": a1b2c3d "Fix login flow" and 1 more. They exist on no remote, so deleting the worktree would lose them.
  push them and run 'claude rm 7c5dcf5d' again, or discard the worktree and its commits: claude rm 7c5dcf5d --discard-unpushed a1b2c3d000000000000000000000000000000000@0123456789abcdef0123456789abcdef
```

Wenn Claude Code die Commits nicht zusammenfassen kann, liest sich die Detailzeile `The worktree has unpushed commits` stattdessen. In der [Agent-Ansicht](/docs/de/agent-view) zeigt die Sitzungszeile `not deleted` mit dem gleichen Grund.

Commits auf einem Remote blockieren die Löschung nicht. Auch nicht Commits auf der lokalen Kopie des Standard-Branches Ihres `origin`-Remote, solange dieser Branch in Ihrem Haupt-Checkout ausgecheckt ist, dem Repository-Verzeichnis selbst, nicht einem Worktree.

**Was zu tun ist:**

* Um die Commits zu behalten, pushen Sie den Branch des Worktree, oder mergen Sie ihn in den Standard-Branch, der in Ihrem Haupt-Checkout ausgecheckt ist, dann löschen Sie die Sitzung erneut
* Um die Commits zu verwerfen, führen Sie den `claude rm <id> --discard-unpushed`-Befehl aus, den die Meldung druckte, oder drücken Sie `Ctrl+X` zweimal auf der Sitzungszeile in der Agent-Ansicht erneut. Dies entfernt die Sitzung und den Worktree zusammen mit seinem Branch, den nicht gepushten Commits und allen nicht committeten Änderungen. Wenn der Worktree seit der Ablehnung einen Commit gewonnen hat, behält Claude Code ihn erneut und zeigt den aktualisierten Status
* Wenn die Meldung sagt, dass der Worktree auch von einer anderen beendeten Sitzung aufgezeichnet wird, blockiert das Löschen erneut nicht, ihn zu verwerfen: pushen Sie die Commits, dann löschen Sie die Sitzung erneut

Vor v2.1.268 setzte `claude rm` die Commit-Zusammenfassung auf die `kept`-Zeile selbst. Wenn `claude rm` die Commits nicht zusammenfassen konnte, las sich die `kept`-Zeile `worktree has commits that are not pushed anywhere` anstelle der Zusammenfassung.

Vor v2.1.260 nannte die Meldung nicht den Branch oder die Commits, und das Löschen erneut wurde auf die gleiche Weise abgelehnt: das Löschen der Sitzung ohne Pushen bedeutete das Entfernen des Worktree selbst mit `git worktree remove --force <path>`, dann das erneute Ausführen von `claude rm <id>`.

Vor v2.1.248 zählte der Standard-Branch, der in Ihrem Haupt-Checkout ausgecheckt ist, nicht: Ein Branch, den Sie bereits dort gemergt haben, löste diese Ablehnung immer noch aus, bis seine Commits einen Remote erreichten.

<h3 id="terminal-host-process-died">
  Terminal-Host-Prozess ist gestorben
</h3>

Jedes [Hintergrund-Sitzungs](/docs/de/agent-view)-Terminal läuft in einem Host-Prozess unter dem Hintergrund-Service, und dieser Prozess ist gestorben, während der Service seine Verbindung noch hielt, daher konnte die Sitzung nicht erreicht werden.

Unter Linux und WSL überprüft der Hintergrund-Service jeden Host-Prozess alle paar Sekunden, markiert die Sitzung als fehlgeschlagen, wenn der Prozess beendet wurde, aber seine Verbindung zum Service nie geschlossen wurde, und zeigt den Grund auf ihrer Zeile in der [Agent-Ansicht](/docs/de/agent-view#read-session-state):

```text theme={null}
terminal host process died — press Enter to restart
```

Wenn Sie die Zeile öffnen, bevor die Überprüfung läuft, zeigt die Fußzeile `This session's terminal host process died (the conversation is saved) — press Enter to restart it` und die Zeile wird fehlgeschlagen.

Aus der Shell startet `claude attach <id>` eine Sitzung, die bereits als fehlgeschlagen für einen toten Host markiert ist, neu, und druckt ansonsten den Grund und beendet sich:

```text theme={null}
Couldn't attach to <id> — This session's terminal host process died (the conversation is saved) — run `claude attach <id>` again to restart it on a fresh host.
```

Das Gespräch ist in jedem Fall gespeichert.

Eine Zeile, die einen [Shell-Befehl](/docs/de/agent-view#run-a-shell-command) ausführt, zeigt stattdessen `terminal host process died — its output is gone; the command was not run again`, und `claude attach` druckt `This command's terminal host process died — its output is gone and the command was not run again`. Claude Code führt den Befehl niemals für Sie erneut aus.

**Was zu tun ist:**

* Drücken Sie in der Agent-Ansicht `Enter` auf der fehlgeschlagenen Zeile; die Sitzung startet auf einem frischen Host-Prozess neu und das Gespräch wird fortgesetzt
* Führen Sie aus der Shell `claude attach <id>` erneut aus. Claude Code druckt `Session <id>'s terminal host died — restarting it on a fresh one…` und öffnet die Sitzung erneut
* Sie können eine Shell-Befehl-Zeile auf diese Weise nicht neu starten; entsenden Sie den Befehl erneut, um ihn erneut auszuführen

Vor v2.1.247 konnte ein toter Host-Prozess jede Liveness-Überprüfung bestehen, die der Hintergrund-Service ausführte, daher zeigte das Öffnen der Sitzung `opening… · esc to cancel` unbegrenzt und `claude attach <id>` wartete ohne Fehlermeldung.

<h3 id="session-isnt-responding">
  Sitzung antwortet nicht
</h3>

Sie haben eine [Hintergrund-Sitzung](/docs/de/agent-view) geöffnet und der Hintergrund-Service hat das Öffnen akzeptiert, aber keine Ausgabe kam für etwa zehn Sekunden an, daher schließt Claude Code, dass der Prozess, der das Terminal der Sitzung weiterleitet, keine Ausgabe liefern kann, und beendet den Versuch, anstatt zu warten.

In der Agent-Ansicht bietet Claude Code einen Neustart in der Fußzeile an:

```text theme={null}
Press enter again to restart this session — it isn't responding (its conversation is saved and resumes).
```

Aus der Shell druckt `claude attach <id>` den Grund und beendet sich:

```text theme={null}
Couldn't attach to <id> — Session isn't responding — `claude stop <id>`, then `claude attach <id>` restarts it (the conversation is saved).
```

Claude Code startet eine Zeile, die einen [Shell-Befehl](/docs/de/agent-view#run-a-shell-command) ausführt, niemals für Sie neu, da ein Neustart den Befehl erneut ausführen würde.

**Was zu tun ist:**

* Drücken Sie in der Agent-Ansicht `Enter` auf der gleichen Zeile erneut. Claude Code stoppt den nicht reagierenden Prozess und startet die Sitzung neu, und das Gespräch wird fortgesetzt. Nichts wird ohne diesen zweiten Druck gestoppt
* Führen Sie aus der Shell `claude stop <id>` aus, dann `claude attach <id>`
* Drücken Sie für eine Shell-Befehl-Zeile `Ctrl+X` in der Agent-Ansicht oder führen Sie `claude stop <id>` aus, um sie zu stoppen; entsenden Sie den Befehl erneut, um ihn erneut auszuführen

<h3 id="session-was-stopped-while-the-respawn-was-in-flight">
  Sitzung wurde gestoppt, während der Respawn im Flug war
</h3>

Sie haben eine [Hintergrund-Sitzung](/docs/de/agent-view) geöffnet, deren Prozess nicht lief, und während Claude Code sie neu startete, stoppte ein anderer Claude Code-Prozess sie, zum Beispiel `claude stop` in einem anderen Terminal. Claude Code hält die Sitzung gestoppt:

```text theme={null}
Session <id> was stopped while the respawn was in flight
```

Das Öffnen einer Sitzung, die Sie gerade entsandt haben, während ihr Prozess noch startet, wartet auf den Prozess stattdessen. Vor v2.1.246 konnte das Öffnen zu diesem Zeitpunkt sie stoppen und diese Meldung anzeigen.

**Was zu tun ist:**

* Wenn Sie die Sitzung nicht gestoppt haben, öffnen Sie ihre Zeile erneut in der Agent-Ansicht oder führen Sie `claude respawn <id>` aus, um sie neu zu starten
* Wenn Sie sie selbst gestoppt haben, bleibt nichts zu tun: die Sitzung bleibt gestoppt

<h3 id="session-agent-no-longer-available">
  Sitzungs-Agent nicht mehr verfügbar
</h3>

Sie haben eine Sitzung fortgesetzt, die einen [benutzerdefinierten Agent](/docs/de/sub-agents#invoke-subagents-explicitly) ausführte, gestartet mit `--agent` oder der `agent`-Einstellung, und Claude Code hat keinen Agent mit diesem Namen gefunden. Es durchsucht zuerst das ursprüngliche Verzeichnis der Sitzung, wenn Sie [diesen Arbeitsbereich vertraut haben](/docs/de/permissions#project-allow-rules-and-workspace-trust), dann das Verzeichnis, von dem aus Sie fortsetzen. Die Sitzung wird immer noch fortgesetzt, aber mit den Standard-Tools und der Standard-Systemaufforderung, daher gelten die Tool-Einschränkungen des Agenten nicht mehr:

```text theme={null}
This session was running agent 'code-reviewer', which is no longer available (no agent by that name in /home/you/project). Continuing with the default tools and system prompt — the agent's tool restrictions no longer apply. To restore it, re-create the agent, or resume with an explicit --agent <name>.
```

Die Warnung nennt nur die Verzeichnisse, die Claude Code durchsucht hat, und sie erscheint im fortgesetzten Gespräch, ob Sie eine [Hintergrund-Sitzung](/docs/de/agent-view) aufwecken, `/resume` oder `claude --resume` ausführen, oder im [nicht-interaktiven Modus](/docs/de/headless) fortsetzen, wo sie auch zu stderr geht. Sitzungen, die `--input-format stream-json` verwenden, zeigen sie nicht, da das Agent SDK Agenten nach dem Start bereitstellt.

Claude Code speichert den Fallback nicht in der Sitzung, daher wiederholt sich die Warnung bei jedem Fortsetzen, bis Sie handeln. Der eingebaute `claude`-Agent löst die Warnung nicht aus, da das Zurückfallen auf den Standard-Toolset für ihn nichts ändert. Vor v2.1.216 setzte Claude Code stillschweigend als Standard-Agent fort, und die Suche deckte nur das Verzeichnis ab, von dem aus Sie fortgesetzt haben, daher ging ein Projekt-Agent bei jedem Fortsetzen von einem anderen Verzeichnis verloren.

**Was zu tun ist:**

* Erstellen Sie die Agent-Datei unter `.claude/agents/<name>.md` im Projekt der Sitzung oder unter `~/.claude/agents/<name>.md` für einen persönlichen Agent neu, dann setzen Sie erneut fort
* Oder setzen Sie mit `--agent <name>` fort, das einen Agent benennt, der existiert, um die Sitzung stattdessen als dieser Agent auszuführen
* Wenn der Agent Projekt-Agent ist und Sie das ursprüngliche Verzeichnis der Sitzung nicht vertraut haben, führen Sie Claude Code dort einmal aus, akzeptieren Sie den Vertrauensdialog, dann setzen Sie erneut fort

<h3 id="claude_code_process_wrapper-launcher-errors">
  CLAUDE\_CODE\_PROCESS\_WRAPPER Launcher-Fehler
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/de/corporate-launcher) ist gesetzt, und sein Wert kann nicht verwendet werden, daher lehnt Claude Code ab, den betroffenen Prozess zu starten, anstatt ihn ohne den Launcher auszuführen. Konfigurationsprobleme werden mit einer Meldung gemeldet, die mit dem Variablennamen beginnt und den Grund angibt, zum Beispiel:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Ein Launcher, der startet, aber beendet wird, ohne sich selbst durch Claude Code zu ersetzen, schlägt die Sitzung fehl, die er startete, und die Sitzungszeile in der Agent-Ansicht meldet, dass der Launcher `must exec, not daemonize`, gefolgt von allem, was der Launcher druckte. Eine Sitzung, die nicht starten oder den Hintergrund-Service nicht erreichen kann, weil des Launchers, meldet das Launcher-Problem als Grund innerhalb von `Couldn't reach the background service (...)`.

**Was zu tun ist:**

* Setzen Sie die Variable auf den absoluten Pfad einer ausführbaren Datei, die mit `exec "$@"` endet. Siehe [den Launcher-Vertrag](/docs/de/corporate-launcher#the-launcher-contract) für den vollständigen Vertrag
* Überprüfen Sie `/status`, das den aufgelösten Start-Befehl in seinem Self-exec-Eintrag anzeigt und warnt, wenn der laufende Hintergrund-Service nicht damit übereinstimmt, oder führen Sie `claude daemon status` aus einer Shell aus
* Nach dem Beheben des Wertes im `env`-Block von [Einstellungen](/docs/de/corporate-launcher#set-up-the-launcher) starten Sie den Hintergrund-Service mit `claude daemon stop --any` neu, damit der nächste Dispatch einen umschlossenen startet

<h3 id="eunknown-when-starting-a-background-session">
  EUNKNOWN beim Starten einer Hintergrund-Sitzung
</h3>

Windows lehnte ab, ein Programm mit einem Fehlercode zu starten, der keinen Standardnamen hat, daher wird der Fehler als `EUNKNOWN` angezeigt. Der übliche Auslöser ist eine Softwarebeschränkungsrichtlinie, wie Gruppenrichtlinie oder AppLocker, die das gestartete Programm blockiert. Der Fehler erscheint, wenn Sie eine [Hintergrund-Sitzung](/docs/de/agent-view) mit `/background` oder `claude --bg` starten:

```text theme={null}
Couldn't reach the background service (spawn background service: EUNKNOWN: unknown error, uv_spawn) — run 'claude daemon status'
```

Bei einigen Konten sagt die Meldung `daemon` anstelle von `background service`.

Bei einer npm-Installation hat ein `EUNKNOWN`, das erscheint, während `npm install -g @anthropic-ai/claude-code` die Binärdatei ersetzt, die gleiche Ursache wie [`EACCES` beim Neustart einer Hintergrund-Sitzung](#eacces-when-starting-a-background-session) und wird gelöscht, wenn Sie nach Abschluss der Installation erneut versuchen.

Claude Code startet den Hintergrund-Service über PowerShell, damit der Service das Schließen des Terminals überlebt, wobei PowerShell 7 verwendet wird, wenn es installiert ist, und Windows PowerShell 5.1 ansonsten. Wenn weder PowerShell laufen kann, startet Claude Code den Service stattdessen direkt, daher verursacht eine Richtlinie, die nur PowerShell blockiert, diesen Fehler nicht. Wenn Sie ihn sehen, während keine npm-Installation läuft, blockiert die Richtlinie die Claude Code-Ausführungsdatei selbst.

Vor v2.1.212 verwendete Claude Code nur Windows PowerShell 5.1, um den Service zu starten, daher schlugen alle Maschinen fehl, auf denen Gruppenrichtlinie PowerShell 5.1 blockierte, mit `Couldn't start the session — EUNKNOWN: unknown error, uv_spawn`, sogar mit PowerShell 7 installiert.

**Was zu tun ist:**

* Wenn die Meldung `Couldn't start the session` liest, aktualisieren Sie auf v2.1.212 oder später. Auf früheren Versionen können Sie auch `claude daemon run` in einem separaten Terminal zuerst ausführen, dann die Hintergrund-Sitzung erneut starten. Dieser Befehl führt den Hintergrund-Service im Vordergrund des Terminals aus, daher dauert der Service nur so lange wie dieses Terminal offen bleibt.
* Wenn eine npm-Installation die Binärdatei ersetzt, warten Sie, bis sie fertig ist, dann starten Sie die Hintergrund-Sitzung erneut
* Wenn der Fehler auf v2.1.212 oder später erscheint, während keine npm-Installation läuft, bitten Sie Ihren Windows-Administrator, die Claude Code-Ausführungsdatei in der Beschränkungsrichtlinie zuzulassen
* Wenn der Hintergrund-Service stoppt, wenn Sie das Terminal schließen, startete Claude Code ihn ohne PowerShell. Installieren Sie PowerShell 7, oder bitten Sie Ihren Administrator, PowerShell freizugeben, damit der Service das Terminal überleben kann.

<h3 id="eacces-when-starting-a-background-session">
  EACCES beim Starten einer Hintergrund-Sitzung
</h3>

Claude Code konnte seine eigene Binärdatei nicht ausführen, um den [Hintergrund-Service](/docs/de/agent-view#the-supervisor-process) zu starten, der Hintergrund-Sitzungen hostet. Bei einer npm-Installation bedeutet dies normalerweise, dass `npm install -g @anthropic-ai/claude-code` die Binärdatei zu diesem Zeitpunkt ersetzt, ob Sie es ausführten oder der [Auto-Updater](/docs/de/setup#auto-updates) es tat. Der Fehler erscheint, wenn Sie eine Sitzung aus der [Agent-Ansicht](/docs/de/agent-view) öffnen:

```text theme={null}
Couldn't start the background service — spawn background service: EACCES: permission denied, posix_spawn '/usr/local/lib/node_modules/@anthropic-ai/claude-code/bin/claude'
```

Wenn Sie eine Sitzung mit `/background` oder `claude --bg` starten, erscheint der gleiche Grund innerhalb von `Couldn't reach the background service (...)`. Während des gleichen Neuinstallationsfensters kann der Fehler einen anderen Code benennen, wie `ENOENT` oder `ENOEXEC`, oder `EUNKNOWN` oder `EPERM` auf Windows; ein `EUNKNOWN`, das über Wiederholungen hinweg anhält, hat eine [andere Ursache](#eunknown-when-starting-a-background-session).

Bei einer npm-Installation wartet Claude Code auf die Neuinstallation und versucht es automatisch erneut: bis zu zehn Sekunden, und bis zu zwei Minuten, während eine npm-Installation von Claude Code auf der Maschine sichtbar noch läuft, was einen anderen Claude Code-Prozess abdeckt, der ein Update herunterlädt. Wenn die Installation länger als diese Wartezeit dauert, benennt der Fehler das Update stattdessen des bloßen Fehlercodes:

```text theme={null}
Claude Code is being updated by npm on this machine (still not runnable after 2 min, EACCES) — try again when the update finishes
```

Vor v2.1.257 stoppte die Wartezeit bei zehn Sekunden in jedem Fall, daher erschien dieser Fehler, während ein anderer Claude Code-Prozess noch ein Update herunterlädt. Vor v2.1.246 schlug Claude Code sofort fehl, ohne zu warten.

**Was zu tun ist:**

* Warten Sie ein paar Sekunden, dann öffnen Sie die Sitzung oder entsenden Sie erneut. Wenn die Meldung sagt, Claude Code wird aktualisiert, versuchen Sie es erneut, nachdem das Update fertig ist.
* Wenn der Fehler anhält, während keine npm-Installation läuft, kann Ihr Benutzer die installierte Binärdatei nicht ausführen. Überprüfen Sie ihre Berechtigungen und die ihres Verzeichnisses, oder installieren Sie Claude Code neu.

<h3 id="background-service-exited-before-it-became-reachable">
  Hintergrund-Service beendet, bevor er erreichbar wurde
</h3>

Der Prozess, den Claude Code als [Hintergrund-Service](/docs/de/agent-view#the-supervisor-process) startete, beendet sich, bevor er Verbindungen akzeptierte, daher konnte Claude Code Ihre Sitzung nicht öffnen. Wenn der Service einen Fehler druckte, bevor er beendet wurde, gibt der Grund in Klammern den Exit-Code oder das Signal und die erste Zeile an, die der Service druckte, die benennt, was ihn stoppte:

```text theme={null}
Couldn't reach the background service (background service exited before it became reachable (exit code N): <the service's first error line>) — run 'claude daemon status'
```

Wenn Sie eine Sitzung aus der [Agent-Ansicht](/docs/de/agent-view) öffnen, folgt der gleiche Grund `Couldn't start the background service —`. Wenn der Service nichts druckte, bevor er beendet wurde, sagt die Meldung `nothing on stderr` stattdessen.

Claude Code meldet den Fehler mit der Fehlerzeile des Service. Vor v2.1.246 wurde der Fehler nur nach einer 45-Sekunden-Wartezeit angezeigt, als `background service did not become reachable within 45s`, ohne die Fehlerzeile des Service.

Zwei zitierte Gründe haben bekannte Ursachen:

* `Error: claude native binary not installed.`: eine npm-Installation ersetzt die Claude Code-Binärdatei zu diesem Zeitpunkt, daher führte der Service stattdessen npm-Platzhalter aus. Versuchen Sie es erneut, nachdem die Installation fertig ist; wenn die Zeile mit keiner Installation läuft anhält, [schließen Sie die npm-Installation ab](/docs/de/troubleshoot-install#native-binary-not-found-after-npm-install). Vor v2.1.257 produzierte eine macOS npm-Selbstaktualisierung diesen Fehler bei jedem Start während des Installationsfensters.
* `nothing on stderr` mit Exit-Code 1, bei jedem Start, auf Windows: `daemon.lock` benennt einen Prozess, den Claude Code weder signalisieren noch beweisen kann, dass er weg ist, daher schließt jeder neue Service, dass ein anderer die Sperre hält und beendet sich. Eine Sperre, deren Schreiber Claude Code beweisen kann, dass er weg ist, wird von selbst ersetzt und produziert diesen Fehler nicht. Wenn der Fehler bei jedem Start wiederholt wird, löschen Sie `~/.claude/daemon.lock`, dann öffnen Sie die Sitzung oder entsenden Sie erneut. Vor v2.1.257 blockierte eine solche Sperre jeden Start, bis Sie die Datei löschten.

**Was zu tun ist:**

* Wenn die Meldung eine Zeile zitiert, beheben Sie, was sie benennt, dann öffnen Sie die Sitzung oder entsenden Sie erneut. Der nächste Versuch startet den Service erneut
* Führen Sie `claude daemon status` aus, um zu überprüfen, ob ein Service jetzt läuft

<h3 id="working-directory-no-longer-exists-when-starting-a-background-session">
  Arbeitsverzeichnis existiert nicht mehr beim Starten einer Hintergrund-Sitzung
</h3>

Sie haben versucht, eine [Hintergrund-Sitzung](/docs/de/agent-view) in einem Verzeichnis zu starten, das nicht mehr existiert. Dies geschieht, wenn Sie aus der Agent-Ansicht entsenden oder `/background` ausführen, nachdem das Verzeichnis, in dem Sie arbeiten, gelöscht oder verschoben wurde. Es geschieht auch, wenn Sie sich an eine Sitzung anhängen oder eine neu starten, deren Prozess beendet wurde und deren Verzeichnis weg ist, da der neue Prozess in diesem gleichen Verzeichnis starten würde. Claude Code startet die Sitzung nicht, und die Meldung nennt das fehlende Verzeichnis:

```text theme={null}
Couldn't start a background session (working directory no longer exists or is not accessible: /tmp/demo)
```

Vor v2.1.257 schien die Sitzung zu starten und zeigte sich dann in der Agent-Ansicht als fehlgeschlagene Zeile mit dem gleichen Grund.

**Was zu tun ist:**

* Erstellen Sie das Verzeichnis neu, das die Meldung nennt, oder entsenden Sie aus einem Verzeichnis, das existiert, dann versuchen Sie es erneut

<h2 id="wrapper-and-ide-errors">
  Fehler bei Wrapper und IDE
</h2>

Diese Fehler stammen von dem Programm, das Claude Code für Sie gestartet hat, z. B. eine IDE-Erweiterung oder eine [Agent SDK](/docs/de/agent-sdk/overview)-Anwendung, und nicht von Claude Code selbst.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code-Prozess mit Code N beendet
</h3>

Der zugrunde liegende `claude`-Prozess wurde mit einem Nicht-Null-Code beendet. Der Exit-Code allein sagt nicht, was fehlgeschlagen ist: Der eigentliche Fehler befindet sich in der eigenen Ausgabe des Prozesses, die der Wrapper anfügt, wenn er etwas erfasst hat, und ansonsten in seinen Protokollen behält.

```text theme={null}
Error: Claude Code process exited with code 1
```

Unter Windows kann der native Build mit Code `4294967295` direkt nach Abschluss eines Durchlaufs beendet werden. Wenn dieser Exit an einer Durchlauf-Grenze landet, ohne dass eine Nachricht wartet und keine Hintergrundaufgabe ausgeführt wird, schließt die [VS Code-Erweiterung](/docs/de/vs-code) die Sitzung stillschweigend, anstatt diesen Fehler anzuzeigen. Ihre nächste Nachricht setzt das Gespräch fort.

Vor v2.1.273 zeigte die Erweiterung den Fehler für diesen Exit bei jeder Durchlauf-Grenze an, obwohl nichts verloren ging.

**Was zu tun ist:**

* In VS Code folgen Sie dem Link **Ausgabeprotokolle anzeigen**, der mit dem Fehler angezeigt wird, um den zugrunde liegenden Fehler zu sehen
* In einer Agent SDK-Anwendung fangen Sie den Fehler um Ihre Nachrichtenschleife ab. Die Einträge unter [CLI-Prozess-Exit](/docs/de/agent-sdk/troubleshooting#cli-process-exit) behandeln, was Ihr Code in jeder SDK-Sprache erhält.
* Führen Sie `claude` in einem Terminal im selben Projekt aus. Der Fehler wird dort normalerweise mit seiner echten Fehlermeldung reproduziert, die Sie dann auf dieser Seite nachschlagen können.
* Führen Sie `claude doctor` in einem Terminal aus, um die Installation und Konfiguration zu überprüfen

<h3 id="could-not-locate-the-claude-cli-on-path">
  Claude CLI auf PATH nicht gefunden
</h3>

Die [VS Code-Erweiterung](/docs/de/vs-code) zeigt diesen Fehler unter Windows an, wenn Sie Claude Code im integrierten Terminal öffnen, die Shell des Terminals PowerShell ist und die Erweiterung die installierte `claude`-Ausführungsdatei auf PATH nicht finden kann. Die Erweiterung weigert sich, Claude Code zu starten, bis sie die installierte `claude` auf PATH findet.

```text theme={null}
Failed to run Claude Code: Error: Could not locate the Claude CLI on PATH. Launching by name in a PowerShell terminal would run a 'claude' from the open folder instead of the installed CLI, so the launch was blocked. Make sure the Claude CLI's install directory is on your system PATH (not only your PowerShell profile), then restart VS Code and try again. VS Code reads PATH when it starts, so PATH changes take effect only after a restart.
```

**Was zu tun ist:**

* Öffnen Sie ein neues PowerShell-Fenster außerhalb von VS Code und führen Sie `where.exe claude` aus. Wenn es keinen Pfad ausgibt, befindet sich die CLI nicht auf Ihrem PATH: Fügen Sie sein Installationsverzeichnis hinzu, indem Sie [Überprüfen Sie Ihren PATH](/docs/de/troubleshoot-install#verify-your-path) befolgen. Wenn es einen Pfad ausgibt, stammt der Eintrag aus Ihrem PowerShell-Profil oder aus einer PATH-Änderung, die VS Code noch nicht aufgegriffen hat; die nächsten zwei Schritte behandeln diese Fälle.
* Legen Sie den PATH-Eintrag als Benutzer- oder Systemumgebungsvariable fest, nicht in Ihrem PowerShell-Profil. Die Erweiterung führt Ihr Profil nicht aus, daher erreicht eine PATH-Bearbeitung, die nur dort vorhanden ist, sie nie.
* Starten Sie VS Code nach dem Ändern von PATH neu. Die Erweiterung überprüft den PATH, den VS Code beim Start erfasst hat, daher wird eine PATH-Änderung erst nach einem Neustart wirksam.

<h3 id="the-connection-to-claude-code-ended-before-this-message-completed">
  Die Verbindung zu Claude Code wurde beendet, bevor diese Nachricht abgeschlossen wurde
</h3>

Die [VS Code-Erweiterung](/docs/de/vs-code) hat Ihre Nachricht an den `claude`-Prozess gesendet, und die Verbindung wurde ohne Fehler beendet, bevor der Prozess sie bestätigt oder abgeschlossen hat. Die Erweiterung kann nicht feststellen, ob die Nachricht verarbeitet wurde, daher fordert sie Sie auf, sie erneut zu senden:

```text theme={null}
The connection to Claude Code ended before this message completed — it may not have been processed, so please send it again.
```

**Was zu tun ist:**

* Senden Sie die Nachricht erneut. Die nächste Nachricht startet einen neuen `claude`-Prozess, der das Gespräch fortsetzt.
* Wenn es sich wiederholt, führen Sie `claude` in einem Terminal im selben Projekt aus. Ein Fehler, der den Prozess immer wieder beendet, wird dort normalerweise mit seiner echten Fehlermeldung reproduziert.

<h2 id="rewind-warnings-and-errors">
  Rewind-Warnungen und Fehler
</h2>

Diese Meldungen stammen von einer [`/rewind`](/docs/de/checkpointing) Code-Wiederherstellung. `Restored the code, but skipped N files` ist eine Warnung, dass Claude Code einige Pfade übersprungen hat. `No files were restored` ist ein Fehler, der bedeutet, dass nichts wiederhergestellt wurde.

<h3 id="restored-the-code-but-skipped-files">
  Restored the code, but skipped files
</h3>

Eine `/rewind` Code-Wiederherstellung hat einen oder mehrere überwachte Pfade übersprungen, anstatt sie zu schreiben oder zu löschen. Claude Code überspringt einen Pfad, wenn:

* er ist oder wurde ein Symlink, Hard Link oder eine andere nicht-reguläre Datei
* sein Verzeichnis sich seit dem Checkpoint geändert hat
* seine Sicherung nicht sicher gelesen werden kann

Übersprungene Pfade behalten ihren aktuellen Inhalt. Vor v2.1.216 schrieb und löschte `/rewind` durch Links bei überwachten Pfaden und meldete keine teilweise Wiederherstellung.

```text theme={null}
Restored the code, but skipped 2 files: the tracked path is (or became) a link or other non-regular file, its directory changed since the checkpoint, or its backup could not be safely read. Skipped files were left untouched — run with --debug for the paths.
```

**Was zu tun ist:**

* Identifizieren Sie, welche Dateien übersprungen wurden, damit Sie jede mit den folgenden Schritten behandeln können. Die Meldung gibt nur eine Anzahl an; das Debug-Protokoll unter `~/.claude/debug/<session-id>.txt` nennt jeden übersprungenen Pfad während der Wiederherstellung, also aktivieren Sie Debug-Protokollierung mit `/debug` vor Ihrer nächsten Wiederherstellung. Unter macOS oder Linux können Sie die Links stattdessen direkt finden: `find . -type l` für Symlinks und `find . -type f -links +1` für Hard-linked-Dateien.
* Wenn eine übersprungene Datei ein Link ist, den Sie absichtlich erstellt haben, z. B. eine Konfigurationsdatei, die von einem Dotfile-Manager verwaltet wird, oder eine Datei, die von Tools wie pnpm Hard-linked ist, hat der Rewind ihren Inhalt allein gelassen. Um die Änderungen der Sitzung rückgängig zu machen, bitten Sie Claude, die Bearbeitung rückgängig zu machen oder bearbeiten Sie die Datei selbst
* Wenn Sie den Link nicht erstellt haben, überprüfen Sie den Pfad, bevor Sie seinem Inhalt vertrauen: etwas hat die Datei nach dem Checkpoint ersetzt

<h3 id="no-files-were-restored">
  No files were restored
</h3>

Claude Code zeigt diese Meldung an, wenn Sie Code mit [`/rewind`](/docs/de/checkpointing) wiederherstellen und keine der Dateien in diesem Checkpoint wiederherstellen kann. Für jede Datei fehlt entweder die Sicherung, die Claude Code vor der Bearbeitung gespeichert hat, oder Claude Code konnte die Datei nicht schreiben oder löschen.

```text theme={null}
Failed to restore the code:
No files were restored: 1 file failed (backup missing, or the file could not be updated)
```

Claude Code löscht die Sicherungen einer Sitzung beim [Aufräumen](/docs/de/claude-directory#cleaned-up-automatically), standardmäßig etwa 30 Tage, nachdem die Sitzung eine zuletzt gespeichert hat. Wenn Sie eine Sitzung danach fortsetzen, listet `/rewind` ihre Checkpoints immer noch auf, aber das Zurückspulen zu einem von ihnen kann mit diesem Fehler fehlschlagen. Wenn die Meldung auch `N paths were skipped for link safety` sagt, siehe [Restored the code, but skipped files](#restored-the-code-but-skipped-files) für diese Pfade.

Wenn Sie eine Sitzung verzweigen, z. B. mit [`--fork-session`](/docs/de/cli-reference#cli-flags) oder [`/branch`](/docs/de/sessions#branch-a-session), kopiert Claude Code die Sicherungen der ursprünglichen Sitzung in die Verzweigung. Wenn Claude Code eine Sicherung nicht kopieren kann, z. B. weil der Speicherplatz voll ist, fehlt diese Sicherung in der Verzweigung. Das Zurückspulen zu einem Checkpoint, der sie benötigt, kann mit diesem Fehler fehlschlagen.

**Was zu tun ist:**

* Machen Sie die Änderungen auf andere Weise rückgängig: Bitten Sie Claude, seine Bearbeitungen rückgängig zu machen, oder stellen Sie die Dateien aus der Versionskontrolle wieder her. Wenn die Sicherungen weg sind, schlägt das erneute Ausführen von `/rewind` auf die gleiche Weise fehl.
* Wenn Claude Code eine Datei nicht schreiben oder löschen konnte, beheben Sie das, was den Schreibvorgang blockiert, z. B. Dateiberechtigungen, und führen Sie dann `/rewind` erneut aus.
* Um Sicherungen in zukünftigen Sitzungen länger zu behalten, erhöhen Sie [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays).

Vor v2.1.260 übersprangen Claude Code stillschweigend Dateien, deren Sicherungen fehlten, und die Wiederherstellung schien erfolgreich zu sein.

<h2 id="session-saving-warnings">
  Warnungen zum Speichern von Sitzungen
</h2>

Claude Code zeigt diese Warnungen auf einer persistenten Zeile unter dem Eingabefeld an, wenn die Sitzungstranskription nicht gespeichert wird. Die Sitzung funktioniert in beiden Fällen weiter; die Warnungen teilen Ihnen mit, dass die Sitzung möglicherweise später bei [`--resume`](/docs/de/sessions) fehlt.

<h3 id="transcript-writes-are-failing">
  Transkriptschreibvorgänge schlagen fehl
</h3>

Claude Code speichert das Transkript während der Arbeit auf der Festplatte, und die Schreibvorgänge in [die Transkriptdatei](/docs/de/sessions#where-transcripts-are-stored) schlagen fehl. Die Meldung benennt die Ursache mit dem zugrunde liegenden Fehlercode, beispielsweise eine volle Festplatte:

```text theme={null}
Transcript writes are failing (disk full — ENOSPC) · recent messages may not be saved for resume
```

Die Warnung wird je nach Fehler an verschiedenen Stellen angezeigt:

* Beim ersten Fehler für Bedingungen, die sich nicht von selbst beheben: eine volle Festplatte, ein überschrittenes Festplattenkontingent, ein schreibgeschütztes Dateisystem, ein Pfad, der die Längenbeschränkung des Dateisystems überschreitet, oder auf macOS und Linux ein Berechtigungsfehler
* Nach wiederholten Fehlern, die sich über mindestens eine Minute erstrecken, für alles andere, einschließlich Berechtigungsfehlern unter Windows, bei denen ein Antivirenscan einen einzelnen Schreibvorgang fehlschlagen lassen kann, der dann beim erneuten Versuch erfolgreich ist

Vor v2.1.217 verwarf Claude Code die fehlgeschlagenen Schreibvorgänge ohne Warnung, und ein späteres `--resume` mit fehlenden aktuellen Meldungen war das erste Zeichen.

**Was zu tun ist:**

* Beheben Sie die Bedingung, die der Fehlercode benennt: Geben Sie Festplattenspeicher für `ENOSPC` frei; erhöhen oder löschen Sie das Kontingent für `EDQUOT`; stellen Sie Schreibzugriff auf den Transkriptspeicherort für `EACCES`, `EPERM` oder `EROFS` wieder her
* Die Warnung wird beim nächsten erfolgreichen Schreibvorgang automatisch gelöscht; kein Neustart ist erforderlich
* Meldungen, die während der Anzeige der Warnung gesendet wurden, können später beim Fortsetzen der Sitzung immer noch fehlen

<h3 id="transcript-saving-is-off-skip-prompt-history">
  Transkriptspeicherung ist deaktiviert, da CLAUDE\_CODE\_SKIP\_PROMPT\_HISTORY gesetzt ist
</h3>

Diese Sitzung wurde mit [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/de/env-vars) gestartet, daher schreibt Claude Code keine Transkription oder Eingabeaufforderungsverlauf dafür:

```text theme={null}
Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set · --resume will not find this session; if unintended, unset it and restart
```

Die Variable ist ein absichtliches Opt-out für kurzlebige Skriptsitzungen, kann aber auch eine Sitzung durch ein Shell-Profil, ein Wrapper-Skript oder einen übergeordneten Prozess erreichen, der sie exportiert hat.

**Was zu tun ist:**

* Wenn Sie die Variable absichtlich gesetzt haben, ist keine Aktion erforderlich; die Benachrichtigung bestätigt, dass die Sitzung nicht in `--resume`, `--continue` oder im Aufwärts-Pfeil-Verlauf angezeigt wird
* Wenn nicht, entfernen Sie die Variable aus der Shell oder dem Skript, das `claude` startet, und starten Sie dann eine neue Sitzung. Meldungen aus der aktuellen Sitzung werden nicht rückwirkend gespeichert.

<h3 id="transcript-saving-is-off-child-session-marker">
  Transkriptspeicherung ist deaktiviert, da ein vererbter CLAUDE\_CODE\_CHILD\_SESSION-Marker vorhanden ist
</h3>

Claude Code setzt [`CLAUDE_CODE_CHILD_SESSION`](/docs/de/env-vars) in den Unterprozessen, die es startet, und behandelt eine interaktive Sitzung, die ihn erbt, als verschachtelt: Claude Code speichert keine Transkription dafür, daher füllen Sitzungen, die Claude selbst startet, nicht Ihre `--resume`-Liste. Diese Benachrichtigung bedeutet, dass Ihre aktuelle Sitzung den Marker geerbt hat:

```text theme={null}
Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker · restart with CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1 to keep future transcripts
```

Die Benachrichtigung ist zu erwarten, wenn Sie `claude` von innen aus einer anderen Claude Code-Sitzung ausgeführt haben; sie signalisiert eine Fehlklassifizierung, wenn der Marker durch einen langlebigen Vermittler durchgesickert ist, beispielsweise ein Terminal, eine `screen`-Sitzung oder ein Launcher, den eine Claude Code-Sitzung ursprünglich gestartet hat.

Innerhalb von tmux erkennt Claude Code einen Marker, der durch die globale Umgebung des tmux-Servers angekommen ist, und setzt das Speichern fort, daher wird diese Benachrichtigung für diesen Fall nicht angezeigt.

**Was zu tun ist:**

* Wenn Sie diese Sitzung absichtlich von innen aus einer anderen Claude Code-Sitzung gestartet haben, ist keine Aktion erforderlich
* Wenn dies eine Sitzung auf oberster Ebene ist, beenden Sie sie und starten Sie sie mit [`CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1`](/docs/de/env-vars) neu. Das Speichern wird ab dem Neustart angewendet, daher werden Meldungen, die davor gesendet wurden, nicht gespeichert.
* Um zukünftige Starts vom selben Terminal oder Launcher zu beheben, entfernen Sie `CLAUDE_CODE_CHILD_SESSION` aus seiner Umgebung

<h2 id="configuration-warnings">
  Konfigurationswarnungen
</h2>

Claude Code schreibt die meisten dieser Meldungen auf stderr, nicht in die Konversation, und schreibt die meisten beim Start. Ein Eintrag gibt an, wenn seine Meldung an anderer Stelle erscheint, z. B. im Debug-Protokoll oder als Startnachricht in der Konversationsansicht, oder zu einem anderen Zeitpunkt, z. B. die [Zeile zur Diagnose nicht erkannter Modelle](#unrecognized-model-id-on-a-request) zur Anfragezeitpunkt.

<h3 id="fullscreen-failed-start-notice">
  Fullscreen-Renderer konnte nicht vollständig starten
</h3>

Eine vorherige [Fullscreen](/docs/de/fullscreen)-Sitzung auf diesem Computer wurde beendet, bevor sie vollständig gestartet wurde, daher startet Claude Code diese Sitzung auf dem klassischen Renderer und gibt eine dieser Meldungen aus:

```text theme={null}
Claude Code's fullscreen renderer didn't finish starting last time on this machine, so this launch is using the classic renderer. It will try fullscreen again next launch; /tui default keeps the classic renderer.

Claude Code's fullscreen renderer has repeatedly failed to start on this machine, so it has been turned off here. Run /tui fullscreen to try it again (this also resets after an update).
```

**Was zu tun ist:**

* Folgen Sie [Fullscreen-Rendering](/docs/de/fullscreen#fullscreen-renderer-didnt-finish-starting). Es sagt, welche Meldung Sie erhalten, was Claude Code in späteren Sitzungen tut, und wie Sie Fullscreen erneut versuchen oder den klassischen Renderer beibehalten können.
* Wenn die Sitzung, die beendet wurde, eine Ausstiegsmeldung ausgegeben hat, siehe [Claude Code wurde nach einem nicht behebbaren Schnittstellenfehler beendet](#exited-after-an-unrecoverable-interface-error), um zu sehen, was es benennt.

Vor v2.1.236 gab Claude Code keine Meldung aus und startete Sitzungen nach einem fehlgeschlagenen Start weiterhin im Fullscreen-Rendering.

<h3 id="exited-after-an-unrecoverable-interface-error">
  Claude Code wurde nach einem nicht behebbaren Schnittstellenfehler beendet
</h3>

Claude Code gibt diese Meldung aus, wenn es beendet wird, weil seine Terminalschnittstelle auf einen Fehler stößt, von dem sie sich nicht erholen kann, in beiden Renderern. Der zweite Satz erscheint nur, wenn der Fehler aufgetreten ist, während der [Fullscreen](/docs/de/fullscreen)-Renderer gestartet wurde:

```text theme={null}
Claude Code exited after an unrecoverable interface error (<error>). It happened while the fullscreen renderer was starting, so the next launch will use the classic renderer (CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1 forces that any time).
```

**Was zu tun ist:**

* Starten Sie Claude Code erneut. Um die Konversation fortzusetzen, führen Sie `claude --resume` im selben Verzeichnis aus.
* Wenn die Meldung den Fullscreen-Renderer benennt, sagt [Fullscreen-Rendering](/docs/de/fullscreen#fullscreen-renderer-didnt-finish-starting), was der nächste Start tut, was davon abhängt, wie Sie Fullscreen aktiviert haben, und wie Sie Fullscreen erneut versuchen oder den klassischen Renderer beibehalten können.

Vor v2.1.236 wurde Claude Code nach dieser Art von Fehler ohne Meldung beendet.

<h3 id="agent-descriptions-are-over-the-15000-token-limit">
  Agent-Beschreibungen überschreiten das Limit von 15.000 Token
</h3>

Claude Code zeigt diese Warnung als Startnachricht in der Konversationsansicht statt auf stderr an. Die kombinierten Beschreibungen Ihrer [Subagenten](/docs/de/sub-agents), außer den integrierten, überschreiten 15.000 Token, wie Claude Code sie schätzt. Jeder Agent zählt seinen Namen plus sein `description`-Frontmatter. Claude Code lädt jeden Agent, unabhängig davon, ob die Gesamtzahl das Limit überschreitet, daher ändert die Warnung nicht, was geladen wird.

```text theme={null}
Agent descriptions are over the 15.0k-token limit (~16.2k tokens) · ask Claude to trim agent descriptions in .claude/agents/
```

**Was zu tun ist:**

* Kürzen Sie das `description`-Frontmatter Ihrer Agent-Dateien, oder bitten Sie Claude, diese für Sie zu kürzen.
* Entfernen Sie Agent-Dateien, die Sie nicht mehr verwenden.

<h3 id="workspace-has-not-been-trusted">
  Arbeitsbereich wurde nicht vertraut
</h3>

Claude Code fand `permissions.allow`-Regeln oder `permissions.additionalDirectories`-Einträge in der `.claude/settings.json` oder `.claude/settings.local.json` des Projekts und wendete sie nicht an, weil [Allow-Regeln aus Projekteinstellungen Arbeitsbereichsvertrauen erfordern](/docs/de/permissions#project-allow-rules-and-workspace-trust). Die Anzahl, der Einstellungsname und die in der Meldung benannte Datei variieren je nach Ihrer Konfiguration. `deny`- und `ask`-Regeln sind nicht betroffen.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Was zu tun ist:**

* Führen Sie `claude` im Verzeichnis aus und akzeptieren Sie den Vertrauensdialog. [Projekterlaubnisregeln und Arbeitsbereichsvertrauen](/docs/de/permissions#project-allow-rules-and-workspace-trust) sagt, welcher Ordner diese Akzeptanz abdeckt.
* Im [nicht-interaktiven Modus](/docs/de/headless) mit `-p` wird kein Dialog angezeigt. Legen Sie den `hasTrustDialogAccepted`-Eintrag in `~/.claude.json` mit dem genauen `projects`-Schlüssel fest, den die Meldung ausgibt.
* Wenn die Meldung `.claude/settings.local.json` benennt und Sie Claude Code außerhalb eines Git-Repositorys oder in Ihrem Home-Verzeichnis gestartet haben, aktualisieren Sie auf v2.1.200 oder später. Die Versionen 2.1.196 bis 2.1.199 behandelten Ihre eigene `.claude/settings.local.json` als vom Repository bereitgestellt in diesen Arbeitsbereichen. Auf v2.1.207 und später reicht eine Aktualisierung außerhalb eines Git-Repositorys nicht aus, wenn Sie den Ordner nicht vertraut haben: Die Feststellung, dass sich ein Ordner nicht in einem Repository befindet, führt Git aus, und Claude Code führt diese Überprüfung nur durch, nachdem Sie den Vertrauensdialog akzeptieren, daher verwenden Sie den ersten Schritt. Ihr Home-Verzeichnis und alle anderen [Konfigurationshome](/docs/de/permissions#project-allow-rules-and-workspace-trust) sind ausgenommen und warten nicht auf den Dialog. Siehe [Projekterlaubnisregeln und Arbeitsbereichsvertrauen](/docs/de/permissions#project-allow-rules-and-workspace-trust).

<h3 id="working-directory-is-a-network-path">
  Arbeitsverzeichnis ist ein Netzwerkpfad
</h3>

Claude Code fügt Netzwerkpfade nicht als Arbeitsverzeichnisse hinzu. Das Nachschlagen eines Netzwerkpfads kann den Host kontaktieren, den er benennt, und unter Windows kann dieser Kontakt dem Host Ihre Anmeldedaten senden, daher lehnt Claude Code den Pfad ab, ohne ihn nachzuschlagen. Sie sehen diese Meldung, wenn Sie `/add-dir` mit einem solchen Pfad ausführen, oder als Warnung beim Start. Wenn es beim Start erscheint, startet Claude Code ohne dieses Verzeichnis.

```text theme={null}
\\server\share is a network path, which cannot be added as a working directory. On Windows, map the share to a drive letter and pass it at launch with --add-dir (a drive letter added mid-session does not yet carry remote-read trust).
```

Pfade, die Claude Code auf diese Weise ablehnt, umfassen:

* UNC-Freigaben wie `\\server\share`
* Automount-Pfade wie `/net/<host>`, es sei denn, Sie haben Claude Code aus einem Verzeichnis unter dem Automount dieses Hosts gestartet
* Lokale Pfade, die einen Netzwerkort über einen symbolischen Link oder eine Verknüpfung erreichen

Zugeordnete Laufwerksbuchstaben und `\\wsl$`-Pfade zählen nicht als Netzwerkpfade.

**Was zu tun ist:**

* Unter Windows ordnen Sie die Freigabe einem Laufwerksbuchstaben zu, z. B. mit `net use Z: \\server\share`, und übergeben Sie das Laufwerk beim Start mit `claude --add-dir Z:\`.
* Unter macOS oder Linux mounten Sie die Freigabe unter einem lokalen Pfad und fügen Sie stattdessen diesen Pfad hinzu.
* Wenn sich der Pfad in `permissions.additionalDirectories` befindet, entfernen Sie ihn aus der Einstellungsdatei, die ihn auflistet.

Vor v2.1.257 akzeptierte Claude Code einen erreichbaren Netzwerkpfad als Arbeitsverzeichnis.

<h3 id="remote-managed-settings-failed-to-load">
  Remote verwaltete Einstellungen konnten nicht geladen werden
</h3>

Ihre Sitzung ist berechtigt für [servergesteuerte Einstellungen](/docs/de/server-managed-settings), aber Claude Code konnte sie nicht abrufen, daher zeigt es diese Warnung in interaktiven Sitzungen an. Die eingeklammerte Ursache benennt, was fehlgeschlagen ist, z. B. `network error`, `request timed out` oder `authentication rejected (401)`, und der Rest der Zeile sagt, welche Richtlinie die Sitzung ausführt:

* **Einstellungen aus einem früheren erfolgreichen Abruf zwischengespeichert**: Claude Code führt die Sitzung auf dieser zwischengespeicherten Richtlinie aus, außer den [zurückhaltenen Umgebungsvariablen](/docs/de/server-managed-settings#fetch-and-caching-behavior), und die Zeile liest `using cached policy`.
* **Kein Cache**: Claude Code führt die Sitzung ohne servergesteuerte Einstellungen aus, und die Zeile liest `no remote policy applied`.

**Was zu tun ist:**

* Handeln Sie nach der Ursache, die die Meldung benennt: Überprüfen Sie für eine Netzwerkursache, dass dieser Computer `api.anthropic.com` erreichen kann; für eine Authentifizierungsursache überprüfen Sie Ihre Anmeldung mit `/status`
* Führen Sie `/status` oder `claude doctor` aus, um die vollständige Diagnose zu erhalten

Vor v2.1.248 meldete Claude Code einen fehlgeschlagenen Einstellungsabruf nur im Debug-Protokoll.

<h3 id="managed-settings-were-not-approved">
  Verwaltete Einstellungen wurden nicht genehmigt
</h3>

Die [servergesteuerten Einstellungen](/docs/de/server-managed-settings) Ihrer Organisation enthalten Einstellungen, die Ihre Genehmigung benötigen, und Sie haben den [Sicherheitsgenehmigungsdialog](/docs/de/server-managed-settings#security-approval-dialogs) abgelehnt, daher wird Claude Code beendet, ohne sie anzuwenden:

```text theme={null}
Managed settings were not approved; exiting without applying them.
```

**Was zu tun ist:**

* Starten Sie Claude Code erneut und genehmigen Sie den Dialog, um unter den Einstellungen Ihrer Organisation fortzufahren. Ein abgelehnter Dialog wird nicht gespeichert, daher erscheint er beim nächsten Start erneut.
* Wenn Sie sich über eine Einstellung unsicher sind, die der Dialog auflistet, fragen Sie denjenigen, der die verwalteten Einstellungen Ihrer Organisation verwaltet, bevor Sie genehmigen

<h3 id="mcp-server-is-blocked-by-enterprise-managed-policy">
  MCP-Server wird durch Unternehmensrichtlinie blockiert
</h3>

Sie haben **Reconnect** auf einem Server in `/mcp` ausgewählt oder einen deaktivierten Server dort wieder aktiviert, und eine Einstellung, die [MCP-Server einschränkt](/docs/de/managed-mcp), blockiert diesen Server. Claude Code weigert sich, ihn zu verbinden, und zeigt:

```text theme={null}
MCP server <name> is blocked by enterprise managed policy
```

Jede dieser Einstellungen kann die Meldung erzeugen:

* Ein [`deniedMcpServers`](/docs/de/managed-mcp#policy-based-control-with-allowlists-and-denylists)-Eintrag, der dem Server entspricht, einschließlich eines in Ihrer eigenen `~/.claude/settings.json` oder der `.claude/settings.json` des Projekts
* Eine [`allowedMcpServers`](/docs/de/managed-mcp#policy-based-control-with-allowlists-and-denylists)-Liste, der der Server nicht entspricht
* [`strictPluginOnlyCustomization`](/docs/de/settings-reference#strictpluginonlycustomization) mit `mcp` gesperrt, das Server blockiert, die in `~/.claude.json` und `.mcp.json` konfiguriert sind
* [`disableClaudeAiConnectors`](/docs/de/mcp#disable-claude-ai-connectors), wenn der Server ein claude.ai-Connector ist

**Was zu tun ist:**

* Überprüfen Sie Ihre eigenen Benutzer- und Projekteinstellungsdateien auf eine dieser Einstellungen und ändern oder entfernen Sie sie
* Wenn keine Ihrer eigenen Einstellungen die Blockade erklärt, fragen Sie Ihren Administrator, welche verwaltete Einstellung den Server blockiert

Vor v2.1.257 konnten **Reconnect** und Reaktivierung in `/mcp` einen Server verbinden, den eine Richtlinienaktualisierung während der Sitzung blockierte.

<h3 id="managed-settings-document-could-not-be-parsed">
  Verwaltetes Einstellungsdokument konnte nicht analysiert werden
</h3>

Ihre Organisation stellt [verwaltete Einstellungen](/docs/de/managed-settings) bereit, und eines der bereitgestellten Dokumente ist vorhanden, kann aber nicht als JSON-Objekt analysiert werden, daher wird Claude Code beim Start mit Code 1 beendet, anstatt ohne die Richtlinie ausgeführt zu werden, die das Dokument trägt. Die Zeile benennt die fehlgeschlagene Quelle vor der Meldung:

```text theme={null}
/Library/Application Support/ClaudeCode/managed-settings.json: Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.
```

Die Quelle ist eine der folgenden:

* Der Pfad der `managed-settings.json`-Datei oder eine Drop-in-Datei unter `managed-settings.d`
* Das macOS-Verwaltungseinstellungsprofil, `per-user managed preferences` oder `device-level managed preferences`
* Der Windows-Registrierungswert, `Registry: HKLM\SOFTWARE\Policies\ClaudeCode\Settings`

[Einträge suchen, die Claude Code gelöscht hat](/docs/de/managed-settings#find-entries-claude-code-dropped) listet auf, was jede Quelle nicht analysierbar macht.

Claude Code weigert sich zu starten, auch wenn eine andere Admin-Quelle eine gültige Richtlinie liefert. Sie sehen diesen Fehler in interaktiven Sitzungen, `claude -p`, Agent SDK-Sitzungen, [Hintergrundsitzungen](/docs/de/agent-view) und den meisten Unterbefehlen, einschließlich `claude doctor`. Die Weigerung schließt absichtlich: Einstellungen in einem Dokument, das Claude Code nicht analysieren kann, können nicht erzwungen werden, und das Starten würde Sitzungen ohne die Kontrollen der Organisation ausführen.

Ein Schemaproblem in einem analysierbaren Dokument erzeugt diesen Fehler nicht. [Einträge suchen, die Claude Code gelöscht hat](/docs/de/managed-settings#find-entries-claude-code-dropped) behandelt, was Claude Code damit tut.

Wenn ein `managed-settings.d/`-Verzeichnis vorhanden ist, aber nicht aufgelistet werden kann, meldet Claude Code `Managed settings drop-in directory could not be read:` gefolgt vom zugrunde liegenden Fehler statt. [Einträge suchen, die Claude Code gelöscht hat](/docs/de/managed-settings#find-entries-claude-code-dropped) behandelt, wann ein Lesefehler beim Start beendet wird.

**Was zu tun ist:**

* Wenn Sie den Computer verwalten, beheben Sie das benannte Dokument, damit es als JSON-Objekt analysiert wird, oder entfernen Sie die Datei, das Profil oder den Registrierungswert. Ein leeres `managed-settings.json` zählt als `{}` und blockiert den Start nicht.
* Wenn nicht, bitten Sie Ihren Administrator, das bereitgestellte Dokument zu beheben. Nichts in Ihren eigenen Einstellungsdateien verursacht oder löscht diesen Fehler.

<h3 id="otelheadershelper-failed">
  otelHeadersHelper fehlgeschlagen
</h3>

Claude Code zeigt diese Warnung als Benachrichtigung in der Terminalschnittstelle an, einmal pro interaktiver Sitzung, wenn das [`otelHeadersHelper`](/docs/de/settings-reference#otelheadershelper)-Skript fehlschlägt oder eine Ausgabe druckt, die nicht den [Skriptanforderungen](/docs/de/monitoring-usage#script-requirements) entspricht.

Während das Skript weiterhin fehlschlägt, schlagen Exporte fehl und Ihr Telemetrie-Backend empfängt nichts aus der Sitzung.

Der Text nach `See /status:` sagt, was fehlgeschlagen ist, z. B. der Exit-Code des Skripts gefolgt von seiner Fehlerausgabe:

```text theme={null}
otelHeadersHelper failed; telemetry is not being exported. See /status: exited 1: token service unreachable
```

**Was zu tun ist:**

* Führen Sie `/status` aus, um das Fehlerdetail zu lesen.
* Beheben Sie das Skript, damit es mit 0 beendet wird, innerhalb von 30 Sekunden, und drucken Sie ein JSON-Objekt von String-Header-Werten auf stdout. Siehe [Skriptanforderungen](/docs/de/monitoring-usage#script-requirements).
* Wenn Ihre Organisation das Skript durch [verwaltete Einstellungen](/docs/de/managed-settings) bereitstellt, bitten Sie denjenigen, der sie verwaltet, es zu beheben.

Im [nicht-interaktiven Modus](/docs/de/headless) mit `-p` erscheint der gleiche Fehler auf stderr statt als `otelHeadersHelper failed (OpenTelemetry export headers unavailable): <error>`.

<h3 id="headershelper-not-run">
  headersHelper nicht ausgeführt
</h3>

Claude Code verbundene einen MCP-Server nur mit seinen statischen `headers` und übersprung den [`headersHelper`](/docs/de/mcp#use-dynamic-headers-for-custom-authentication) des Servers, weil der Helper ein Shell-Befehl ist und der Ordner kein gespeichertes Vertrauen hat. Ein Ordner erhält gespeichertes Vertrauen, wenn Sie seinen Eintrag in `~/.claude.json` von Hand setzen oder, außerhalb Ihres Home-Verzeichnisses, wenn Sie den Vertrauensdialog dafür in einer interaktiven Sitzung akzeptieren. Siehe [Vertrauen Sie einem Ordner, bevor sein headersHelper ausgeführt wird](/docs/de/mcp#trust-a-folder-before-its-headershelper-runs), für welche Server diese Überprüfung gilt.

Claude Code schreibt diese Zeile nur im [nicht-interaktiven Modus](/docs/de/headless), einmal pro Server. In einer interaktiven Sitzung schreibt es stattdessen die gleiche Weigerung in das Debug-Protokoll.

```text theme={null}
MCP server 'internal-api': headersHelper not run — this workspace has no persisted trust; accept the trust dialog here once interactively, or set projects["/Users/you/project"].hasTrustDialogAccepted in /Users/you/.claude.json.
```

Der `projects`-Schlüssel, den die Meldung ausgibt, ist der Ordner, auf den [Projekterlaubnisregeln und Arbeitsbereichsvertrauen](/docs/de/permissions#project-allow-rules-and-workspace-trust) sagt, dass Claude Code das Vertrauen basiert. Das Akzeptieren des Vertrauensdialogs für einen übergeordneten Ordner erfüllt die Überprüfung nicht, und eine `-p`- oder SDK-Sitzung erfüllt sie auch nicht.

**Was zu tun ist:**

* Führen Sie `claude` im Ordner aus, den die Meldung benennt, akzeptieren Sie den Vertrauensdialog, führen Sie dann Ihren `-p`- oder SDK-Befehl erneut aus
* Legen Sie den `hasTrustDialogAccepted`-Eintrag in `~/.claude.json` selbst fest, mit dem genauen `projects`-Schlüssel, den die Meldung ausgibt
* Wenn Sie die Sitzung in Ihrem Home-Verzeichnis gestartet haben, arbeiten Sie aus einem Projektverzeichnis, dem Sie vertraut haben. Wenn Sie den Vertrauensdialog in Ihrem Home-Verzeichnis akzeptieren, behält Claude Code dieses Vertrauen nur für die aktuelle Sitzung.

<h3 id="malformed-tool-content-rule">
  Malformed Tool(content) rule
</h3>

Eine [Berechtigungsregel](/docs/de/permissions#permission-rule-syntax) in einer Ihrer Einstellungsdateien hat nicht die Form `Tool` oder `Tool(content)`, z. B. weil Text nach der schließenden Klammer folgt oder eine der Klammern fehlt. Claude Code überspringt die Regel und listet sie im Dialog für ungültige Einstellungen auf, wenn eine interaktive Sitzung startet, und in der Ausgabe von [`claude doctor`](/docs/de/debug-your-config#check-resolved-settings):

```text theme={null}
Invalid permission rule "Bash(ls) x" was skipped: Malformed Tool(content) rule. Rules take the form Tool or Tool(content) and must end at the closing ")"; parentheses inside the content are literal
```

**Was zu tun ist:**

* Schreiben Sie in der mit der Meldung aufgelisteten Einstellungsdatei die Regel so um, dass sie bei ihrer schließenden Klammer endet, z. B. `Bash(ls *)` statt `Bash(ls) x`
* Lassen Sie Klammern im Inhalt wie sie sind. Sie sind literal, daher ist eine Regel wie `Edit(./Finance (2024)/**)` gültig ohne Escaping

Vor v2.1.260 meldete Claude Code eine Regel mit nicht übereinstimmenden Klammern als `Mismatched parentheses`.

<h3 id="is-not-matched-by-file-permission-checks">
  Wird nicht durch Dateiberechtigungsprüfungen abgeglichen
</h3>

Claude Code fand eine `Write`-, `NotebookEdit`-, `MultiEdit`- oder `Glob`-[Berechtigungsregel](/docs/de/permissions#read-and-edit) mit einem Pfad in einer Ihrer [Einstellungsdateien](/docs/de/settings#where-settings-live), in [verwalteten Einstellungen](/docs/de/managed-settings) oder in einem `--allowedTools`-, `--disallowedTools`- oder `--settings`-Flagwert. Es überprüft Dateiberechtigungen nur gegen `Edit`- und `Read`-Regeln, daher konsultiert es niemals eine Pfadregel, die einen der anderen Dateiwerkzeuge benennt. Es behält die Regel und ändert nichts anderes; die Warnung benennt die Regel, ihre Quelle in Klammern und den Ersatz zum Schreiben:

```text theme={null}
Permission deny rule (.claude/settings.json): Write(docs/**) is not matched by file permission checks — only Edit(path) rules are. Use Edit(docs/**) instead (Edit rules cover all file-editing tools).
```

**Was zu tun ist:**

* Ersetzen Sie `Write(path)`-, `NotebookEdit(path)`- und Legacy-`MultiEdit(path)`-Regeln durch `Edit(path)`. `Edit`-Regeln decken alle Dateieditierungswerkzeuge ab.
* Außer in `--allowedTools`, wo Claude Code eine `Glob`-Regel ohne Warnung akzeptiert, ersetzen Sie `Glob(path)`-Regeln durch `Read(path)`.
* Beheben Sie die Regel an der Quelle, die die Warnung in Klammern benennt: ein Einstellungsdateipfad oder das Flag selbst für `--allowed-tools` und `--disallowed-tools`. Ein `claude-settings-<hash>.json`-Pfad, der nicht auf der Festplatte vorhanden ist, steht für einen Inline-`--settings`-Wert. Beheben Sie das JSON, das Sie an dieses Flag übergeben.
* Lassen Sie bloße Werkzeugnamen-Regeln wie `Write` oder `Glob` allein. Claude Code gleicht sie auf der [Werkzeugebene](/docs/de/permissions#match-all-uses-of-a-tool) ab und warnt nicht davor.
* Wenn die Quelle `managed policy settings` liest, leiten Sie die Warnung an denjenigen weiter, der Ihre verwalteten Einstellungen verwaltet, da Sie sie nicht selbst löschen können.

In einer [Hintergrundsitzung](/docs/de/agent-view) oder mit `--output-format json` oder `stream-json` schreibt Claude Code die Warnung statt auf stderr in das Debug-Protokoll, damit die maschinenlesbare Ausgabe sauber bleibt. Führen Sie mit `--debug` aus, um sie unter `~/.claude/debug/<session-id>.txt` zu erfassen. Vor v2.1.210 akzeptierte Claude Code diese Regeln ohne Warnung.

<h3 id="has-a-wildcard-before-the-rest-of-the-command">
  Hat einen Platzhalter vor dem Rest des Befehls
</h3>

Claude Code fand eine `Bash`-Allow-Regel, deren `*` vor einem späteren Wort kommt, das bestimmt, welcher Befehl es ist, z. B. `Bash(git * main)` oder `Bash(git -C * status *)`, in einer Ihrer [Einstellungsdateien](/docs/de/settings#where-settings-live), in [verwalteten Einstellungen](/docs/de/managed-settings) oder in einem `--allowedTools`- oder `--settings`-Flagwert. Der `*` gleicht jeden Text ab, einschließlich Optionen, die an dieser Position eingefügt werden: `Bash(git * main)` genehmigt auch `git -c core.fsmonitor=<script> diff main`, wobei `-c` git veranlasst, ein Programm auszuführen, das der Befehl benennt. [Wildcard-Muster](/docs/de/permissions#wildcard-patterns) zeigt die Abgleichsregeln.

Die Warnung existiert, damit Sie eine Regel eingrenzen können, deren Platzhalter breiter ist als beabsichtigt. Claude Code behält die Regel und ändert nichts daran, wie sie abgleicht; die Warnung benennt die Regel und ihre Quelle in Klammern:

```text theme={null}
Permission allow rule (.claude/settings.json): Bash(git -C * status *) has a wildcard before the rest of the command, so it also matches any options inserted at that position and approves them without a prompt. For git, options such as -c and --exec-path can run arbitrary commands. Replace that * with the exact value you mean, or only use * after the subcommand (for example Bash(git status *)).
```

**Was zu tun ist:**

* Ersetzen Sie den `*` vor dem Unterbefehl durch den genauen Wert, den Sie meinen: `Bash(git checkout main)` statt `Bash(git * main)`.
* Verschieben Sie jeden `*` nach dem Unterbefehl: `Bash(git status *)` statt `Bash(git -C * status *)`. Schreiben Sie eine Regel pro Unterbefehl, den Sie zulassen möchten.
* Beheben Sie die Regel an der Quelle, die die Warnung in Klammern benennt: ein Einstellungsdateipfad oder das `--allowed-tools`-Flag selbst. Ein `claude-settings-<hash>.json`-Pfad, der nicht auf der Festplatte vorhanden ist, steht für einen Inline-`--settings`-Wert. Beheben Sie das JSON, das Sie an dieses Flag übergeben.
* Wenn die Quelle `managed policy settings` liest, leiten Sie die Warnung an denjenigen weiter, der Ihre verwalteten Einstellungen verwaltet, da Sie sie nicht selbst löschen können.

Claude Code warnt nicht vor Deny- und Ask-Regeln mit der gleichen Form: Es weigert sich oder fordert auf für die zusätzlichen Befehle, die sie abgleichen, anstatt sie zu genehmigen. Es warnt auch nicht vor Regeln, deren Unterbefehl vor dem ersten `*` kommt, z. B. `Bash(git commit *)`, oder Regeln, in denen kein anderes Wort als eine Option dem `*` folgt, z. B. `Bash(git *)`, oder vor `:*`-Präfix-Regeln wie `Bash(git:*)`.

In einer [Hintergrundsitzung](/docs/de/agent-view) oder mit `--output-format json` oder `stream-json` schreibt Claude Code die Warnung statt auf stderr in das Debug-Protokoll, damit die maschinenlesbare Ausgabe sauber bleibt. Führen Sie mit `--debug` aus, um sie unter `~/.claude/debug/<session-id>.txt` zu erfassen. Vor v2.1.246 akzeptierte Claude Code diese Regeln ohne Warnung.

<h3 id="crosssessioninbound-must-be-one-of-accept-hold-refuse">
  crossSessionInbound muss eines von accept, hold, refuse sein
</h3>

Eine Einstellungsdatei setzt [`crossSessionInbound`](/docs/de/settings-reference#crosssessioninbound) auf einen Wert, den Claude Code nicht erkennt, z. B. den Tippfehler `"reject"`. Der zweite Satz der Warnung hängt davon ab, welche Datei den Wert hält; in einer Benutzer-, Projekt-, lokalen oder `--settings`-Datei liest er:

```text theme={null}
"crossSessionInbound" must be one of "accept", "hold", "refuse"; received "reject". This value was ignored; while it is present, cross-session messages are held for your approval instead of being delivered. Set it to one of the values above.
```

In [verwalteten Einstellungen](/docs/de/managed-settings) behandelt Claude Code den nicht erkannten Wert als `refuse`, den restriktivsten Wert, und die Warnung sagt, dass Sitzungsübergreifende Meldungen abgelehnt werden, bis ein Administrator es behebt. Wie die Hold mit Werten in Ihren anderen Einstellungsdateien kombiniert wird, siehe [`crossSessionInbound`](/docs/de/settings-reference#crosssessioninbound).

**Was zu tun ist:**

* Setzen Sie den Schlüssel auf `"accept"`, `"hold"` oder `"refuse"`, oder entfernen Sie ihn
* Wenn die Warnung verwaltete Einstellungen benennt, bitten Sie den Administrator, den Wert zu beheben

Vor v2.1.248 ignorierte Claude Code einen nicht erkannten Wert ohne Warnung.

<h3 id="the-200k-limit-isnt-enforced">
  Das 200K-Limit wird nicht erzwungen
</h3>

Sie setzen [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/de/env-vars), das normalerweise [Auto-Komprimierung](/docs/de/model-config#default-auto-compact-thresholds) dazu bringt, Sitzungen auf 1M-Kontext-Modellen auf ein 200K-Fenster zu halten, aber kein Komprimierungsschwellenwert begrenzt diese Sitzung auf oder unter 200K, daher kann die Konversation über sie hinauswachsen.

```text theme={null}
CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced for <model>, so this session can grow past it. To enforce it, set CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000 (or the autoCompactWindow setting).
```

Claude Code erzwingt das 200K-Limit auf eigene Faust für jedes Modell, das es als ein Modell mit nativem 1M-Fenster erkennt, und für Modell-IDs, die es nicht erkennt, komprimiert es im Fenster, das es annimmt. Die Warnung erscheint, wenn andere Konfiguration diese Erzwingung besiegt:

* Die Modell-ID ist nicht eine, die Claude Code erkennt, z. B. ein [LLM-Gateway](/docs/de/llm-gateway)-Alias, und Sie setzen [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/de/env-vars) oder erhöhten das angenommene Fenster über 200K mit [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/de/env-vars). In diesem Fall bietet die Meldung auch `or update to a Claude Code version that recognizes <model>` als Abhilfe an.
* Ein `context-1m`-Beta, das durch [`ANTHROPIC_BETAS`](/docs/de/env-vars) oder das [`--betas`](/docs/de/cli-reference#cli-flags)-Flag angefordert wird, fragt die API immer noch nach dem 1M-Fenster auf einem Modell an, das dieses Beta akzeptiert, während nichts die Sitzung bei 200K komprimiert

**Was zu tun ist:**

* Setzen Sie [`CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000`](/docs/de/env-vars), oder die [`autoCompactWindow`](/docs/de/settings-reference#autocompactwindow)-Einstellung auf `200000`, damit Auto-Komprimierung bei der 200K-Grenze komprimiert
* Wenn die Meldung eine Modell-ID benennt, die diese Version nicht erkennt, führen Sie `claude update` aus. Eine Version, die die ID als 1M-Kontext-Modell erkennt, erzwingt das Limit ohne weitere Konfiguration.
* Wenn Sie möchten, dass die Sitzung das vollständige Fenster des Modells verwendet, heben Sie `CLAUDE_CODE_DISABLE_1M_CONTEXT` auf; die Warnung meldet nur, dass das 200K-Limit nicht erzwungen wird

In einer [Hintergrundsitzung](/docs/de/agent-view) oder mit `--output-format json` oder `stream-json` schreibt Claude Code die Warnung statt auf stderr in das Debug-Protokoll.

<h3 id="unrecognized-model-id-on-a-request">
  Nicht erkannte Modell-ID bei einer Anfrage
</h3>

Claude Code sendete eine Anfrage für eine Modell-ID, die Ihre Claude Code-Version nicht erkennt, und fand keinen [`modelOverrides`](/docs/de/model-config#override-model-ids-per-version)-Eintrag, der diese ID einem Modell zuordnet, das es erkennt. Claude Code sendet die Anfrage immer noch mit der ID, wie Sie sie konfiguriert haben, und beendet sich nicht oder wechselt Modelle.

```text theme={null}
[claude-code:unrecognized_model] {"model":"my-proxy-model","query_source":"sdk"}
```

In einem Skript oder einer Harness, die stderr liest, gleichen Sie das Präfix `[claude-code:unrecognized_model]` ab. Nach dem Präfix und einem Leerzeichen schreibt Claude Code ein einzeiliges JSON-Objekt. Claude Code kann in einer späteren Version Felder hinzufügen, daher ignorieren Sie alle Felder, die Sie nicht erwarten. Es schreibt mindestens diese zwei:

* `model`: die Modellzeichenkette, wie Sie sie konfiguriert haben
* `query_source`: der Anfragepfad, der das Modell verwendet. Claude Code meldet `sdk` für einen `-p`-Lauf und einen Wert, der mit `agent:` beginnt, für einen Subagenten.

Claude Code schreibt die Zeile an einen von zwei Orten, je nachdem, wie Sie es ausführen:

* Im [nicht-interaktiven Modus](/docs/de/headless) mit `-p` schreibt Claude Code es unter jedem `--output-format` auf stderr, daher können Sie stdout analysieren, ohne die Zeile herauszufiltern
* In einer interaktiven Sitzung oder einer [Hintergrundsitzung](/docs/de/agent-view) schreibt Claude Code es statt in das Debug-Protokoll; führen Sie mit `--debug` aus, um es unter `~/.claude/debug/<session-id>.txt` zu erfassen

Claude Code schreibt die Zeile einmal pro Modellzeichenkette pro Prozess. Es schreibt eine separate Zeile für jede weitere nicht erkannte ID, z. B. eine, die ein [Subagent](/docs/de/sub-agents#choose-a-model) oder [Hintergrundfunktionalität](/docs/de/costs#background-token-usage) verwendet.

Claude Code schreibt die Zeile nicht für Provider-IDs, die es zu einem Modell auflöst, das es erkennt, z. B. Amazon Bedrock `us.anthropic.claude-...`-IDs, Google Cloud's Agent Platform-IDs mit einem `@`-Versionssuffix und Microsoft Foundry-Bereitstellungsnamen, die eine Claude-Modell-ID enthalten. Claude Code überprüft das Modell hinter einem Amazon Bedrock-[Anwendungs-Inferenzprofil-ARN](/docs/de/amazon-bedrock#map-each-model-version-to-an-inference-profile) statt des ARN selbst. Es schreibt keine Zeile für einen ARN, den es nicht auflösen kann, z. B. einen falsch geschriebenen.

**Was zu tun ist:**

* Wenn Sie die ID absichtlich setzen, z. B. einen [LLM-Gateway](/docs/de/llm-gateway)-Alias, fügen Sie einen [`modelOverrides`](/docs/de/model-config#override-model-ids-per-version)-Eintrag zu Ihrer [Einstellungsdatei](/docs/de/settings#where-settings-live) mit der ID als Wert hinzu. Verwenden Sie eine Anthropic-Modell-ID als Schlüssel, nicht einen Familien-Alias wie `opus`. Für `my-proxy-model` aus der Beispielzeile fügen Sie diesen Eintrag hinzu:

  ```json theme={null}
  {
    "modelOverrides": {
      "claude-opus-4-6": "my-proxy-model"
    }
  }
  ```

  Claude Code behandelt dann `my-proxy-model` als `claude-opus-4-6` und stoppt das Schreiben der Zeile.

* Wenn die ID ein Modell benennt, das neuer als Ihre Claude Code-Version ist, führen Sie `claude update` aus

* Wenn die ID ein Tippfehler ist, beheben Sie sie in welchem der [Orte, an denen Sie ein Modell setzen können](/docs/de/model-config#setting-your-model) oder [Alias-Variablen](/docs/de/model-config#environment-variables) es hält. Wenn `query_source` mit `agent:` beginnt, beheben Sie es statt dort, wo Sie das [Modell des Subagenten](/docs/de/sub-agents#choose-a-model) setzen.

Vor v2.1.233 schrieb Claude Code keine Zeile, wenn es eine Anfrage für eine Modell-ID sendete, die es nicht erkannte.

<h3 id="stale-sandbox-mask-files-left-by-a-killed-session">
  Veraltete Sandbox-Maskierungsdateien, die von einer beendeten Sitzung hinterlassen wurden
</h3>

`claude doctor` gibt diese Warnung in seinen Diagnosen aus, und `/status` listet die gleiche Zeile auf. Sie erscheint unter Linux und WSL2, wenn [Sandboxing](/docs/de/sandboxing) mit Dateisystem-Isolation aktiviert ist.

Während ein sandboxierter Befehl ausgeführt wird, hält die Sandbox eine Schreibverweigerung auf einer Datei, die noch nicht existiert, indem sie dort einen 0-Byte-Platzhalter mit Lesezugriff erstellt, und entfernt ihn danach. Eine Sitzung, die vor dieser Bereinigung beendet wird, z. B. durch SIGKILL, hinterlässt die Platzhalter. Spätere Sitzungen binden sie bei jedem Start erneut schreibgeschützt, daher schlägt ein Einstellungsschreiben wie das Speichern von „Ja, und nicht mehr fragen" fehl, wo einer sitzt.

```text theme={null}
- Stale sandbox mask files left by a killed session: /home/you/project/.claude/settings.local.json
  Fix: Remove each with `rm <path>` while no other Claude Code session is running in that project — a 0-byte read-only file where a settings file belongs makes "Yes, and don't ask again" fail to save, and the sandbox binds it read-only again on every start
```

**Was zu tun ist:**

* Beenden Sie alle anderen Claude Code-Sitzungen, die in diesem Projekt ausgeführt werden, löschen Sie dann jede aufgelistete Datei mit `rm`. Die Warnung listet bis zu drei Dateien auf und zählt den Rest, daher führen Sie `claude doctor` erneut aus, nachdem Sie gelöscht haben, bis die Warnung nicht mehr erscheint. Ein Platzhalter, den die Sandbox einer anderen Sitzung noch verwendet, ist ein aktiver Teil des Schreibschutzes dieser Sitzung
* Wenn eine Berechtigungswahl, die Sie mit „Ja, und nicht mehr fragen" gespeichert haben, nicht haften blieb, speichern Sie sie erneut, nachdem Sie den Platzhalter gelöscht haben

Vor v2.1.257 kennzeichnete `claude doctor` diese Dateien nicht; frühere Versionen hinterlassen die gleichen Platzhalter, wenn eine Sitzung beendet wird.

<h2 id="responses-seem-lower-quality-than-usual">
  Antworten scheinen von geringerer Qualität als üblich
</h2>

Wenn Claudes Antworten weniger leistungsfähig erscheinen als erwartet, aber kein Fehler angezeigt wird, liegt die Ursache normalerweise im Gesprächszustand und nicht im Modell selbst. Claude Code ändert nicht stillschweigend Modellversionen. Es kann in drei spezifischen Fällen zu einem Fallback-Modell wechseln:

* Ein konfiguriertes [`--fallback-model`](/docs/de/cli-reference#cli-flags) übernimmt nach einem Verfügbarkeitsfehler für diesen Zug nur mit einer Notiz im Transkript
* Eine Verfügbarkeitsprüfung von Amazon Bedrock oder Google Cloud's Agent Platform stellt fest, dass Ihr Standardmodell nicht verfügbar ist
* [Automatisches Modell-Fallback](/docs/de/model-config#automatic-model-fallback) auf Fable 5.1, Fable 5, Opus 5.5 und Opus 5 verschiebt die Sitzung zum Fallback-Modell der gekennzeichneten Kategorie, wenn diese Kategorie über einen verfügt, und zeigt eine Notiz im Transkript an

Die Modellauswahlprüfung unten erfasst den zweiten und dritten Fall; der erste erscheint als Transkriptnotiz statt als `/model`-Änderung. [Modellkonfiguration](/docs/de/model-config) erklärt, wann jedes Fallback angewendet wird.

Überprüfen Sie diese zuerst:

* **Modellauswahl**: Führen Sie `/model` aus, um zu bestätigen, dass Sie das erwartete Modell verwenden. Eine vorherige `/model`-Auswahl oder eine `ANTHROPIC_MODEL`-Umgebungsvariable kann Sie auf einem kleineren Modell als beabsichtigt platzieren.
* **Aufwandsstufe**: Führen Sie `/effort` aus, um die aktuelle Reasoning-Stufe zu überprüfen und sie für schwieriges Debugging oder Design-Arbeit zu erhöhen. Die Standardwerte variieren je nach Modell, daher überprüfen Sie, bevor Sie davon ausgehen, dass Sie unter dem Maximum liegen. Siehe [Aufwandsstufe anpassen](/docs/de/model-config#adjust-effort-level) für modellspezifische Standardwerte und die `ultrathink`-Verknüpfung.
* **Kontextdruck**: Führen Sie `/context` aus, um zu sehen, wie voll das Fenster ist. Wenn es sich der Kapazität nähert, führen Sie `/compact` an einem natürlichen Haltepunkt oder `/clear` aus, um neu zu beginnen. Siehe [Erkunden Sie das Kontextfenster](/docs/de/context-window), um zu erfahren, wie Auto-Compact frühere Züge beeinflusst.
* **Veraltete Anweisungen**: Große oder veraltete `CLAUDE.md`-Dateien und MCP-Tool-Definitionen verbrauchen Kontext und können Antworten lenken. Die `/doctor`-Überprüfung kennzeichnet übergroße Speicherdateien und ungenutzte Erweiterungen, und `/context` zeigt die MCP-Tool-Token-Nutzung an. Vor v2.1.205 öffnete `/doctor` einen Diagnose-Bildschirm, der übergroße Speicherdateien und Subagent-Definitionen kennzeichnete.

Wenn eine Antwort schiefgeht, funktioniert das Zurückspulen normalerweise besser als das Antworten mit Korrektionen. Drücken Sie Esc zweimal oder führen Sie `/rewind` aus, um vor den fehlerhaften Zug zurückzugehen, und formulieren Sie dann die Eingabeaufforderung mit mehr Spezifika neu. Korrigieren im Thread behält den falschen Versuch im Kontext, was spätere Antworten daran verankern kann. Siehe [Checkpointing](/docs/de/checkpointing).

Wenn die Qualität nach Überprüfung der obigen Punkte immer noch schlecht erscheint, führen Sie `/feedback` aus und beschreiben Sie, was Sie erwartet haben im Vergleich zu dem, was Sie erhalten haben. Auf diese Weise eingereichte Rückmeldungen enthalten das Gesprächstranskript, das die schnellste Möglichkeit für Anthropic ist, eine echte Regression zu diagnostizieren. Siehe [Fehler melden](#report-an-error), wenn `/feedback` in Ihrer Umgebung nicht verfügbar ist.

Wenn Claude vor einer vermuteten Prompt-Injection warnt oder eine Anfrage wegen einer vermuteten Injection ablehnt, und der Text, den die Warnung benennt, Kontext ist, den Claude Code automatisch zum Gespräch hinzufügt, anstatt Datei- oder Webinhalte, führen Sie `claude update` aus und versuchen Sie es erneut. Wenn die Warnung nach dem Update wiederholt wird, [melden Sie es](#report-an-error), anstatt den gekennzeichneten Inhalt zurück in die Eingabeaufforderung einzufügen. Vor v2.1.201 lehnten Sonnet 5 einige Anfragen auf die gleiche Weise ab.

<h2 id="report-an-error">
  Fehler melden
</h2>

Für Fehler von Komponenten, die auf dieser Seite nicht behandelt werden, siehe die relevanten Anleitungen:

* MCP-Server konnte sich nicht verbinden oder authentifizieren: [MCP](/docs/de/mcp)
* Hook-Skript ist fehlgeschlagen oder hat ein Tool blockiert: [Debug-Hooks](/docs/de/hooks#debug-hooks)
* Berechtigung verweigert oder Dateisystemfehler während der Installation: [Troubleshoot-Installation und -Anmeldung](/docs/de/troubleshoot-install)

Wenn ein Fehler hier nicht aufgeführt ist oder die vorgeschlagene Lösung nicht hilft:

* Führen Sie `/feedback` in Claude Code aus, um das Transkript und eine Beschreibung an Anthropic zu senden. Der Befehl bietet auch an, ein vorausgefülltes GitHub-Issue zu öffnen. Das Senden an Anthropic erfordert [Authentifizierung](/docs/de/authentication). Bei Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und anderen Drittanbieter-Plattformen oder wenn keine Anthropic-Anmeldedaten konfiguriert sind, speichert `/feedback` ein lokales Archiv, das Sie stattdessen an Ihren Anthropic-Kontorepräsentanten senden können.
* Führen Sie `claude doctor` aus Ihrer Shell aus, um eine schreibgeschützte Diagnose Ihrer Installation zu erhalten, oder führen Sie die `/doctor`-Überprüfung in Claude Code aus, um Setup-Probleme zu finden und zu beheben
* Überprüfen Sie [status.claude.com](https://status.claude.com) auf aktive Vorfälle
* Suchen Sie nach [bestehenden Issues](https://github.com/anthropics/claude-code/issues) auf GitHub
