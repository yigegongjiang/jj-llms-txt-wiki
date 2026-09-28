> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 오류 참조

> Claude Code 런타임 오류 메시지를 조회하고 각 오류의 의미와 해결 방법을 확인합니다.

이 페이지에는 Claude Code가 표시하는 런타임 오류와 각 오류에서 복구하는 방법, 그리고 오류 없이 응답이 이상해 보일 때 확인할 사항이 나열되어 있습니다. 설정 중 `command not found` 또는 TLS 오류와 같은 설치 오류는 [설치 및 로그인 문제 해결](/docs/ko/troubleshoot-install)을 참조하십시오.

[래퍼 및 IDE 오류](#wrapper-and-ide-errors)를 제외하고, 이는 Claude Code 자체가 아닌 실행 프로그램이 출력하는 오류이며, 이러한 오류 및 복구 명령은 CLI, [데스크톱 앱](/docs/ko/desktop), [웹의 Claude Code](/docs/ko/claude-code-on-the-web)에 모두 적용됩니다. 세 가지 모두 동일한 Claude Code CLI를 래핑하기 때문입니다. 다른 표면별 문제는 해당 표면의 페이지에 있는 문제 해결 섹션을 참조하십시오.

<Note>
  Claude Code는 모델 응답을 위해 Claude API를 호출하므로 대부분의 런타임 오류는 기본 API 오류 코드에 매핑됩니다. 이 페이지에서는 Claude Code 내에서 각 오류의 의미와 복구 방법을 다룹니다. 원본 HTTP 상태 코드 정의는 [Claude Platform 오류 참조](https://platform.claude.com/docs/en/api/errors)를 참조하십시오.
</Note>

<h2 id="find-your-error">
  오류 찾기
</h2>

아래에서 보이는 메시지와 일치하는 섹션을 찾으세요.

| 메시지                                                                                                                                                                                                                                                                  | 섹션                                                                                                |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| `API Error: 500 Internal server error`                                                                                                                                                                                                                               | [서버 오류](#api-error-500-internal-server-error)                                                     |
| `API Error: Repeated 529 Overloaded errors`                                                                                                                                                                                                                          | [서버 오류](#api-error-repeated-529-overloaded-errors)                                                |
| `Request timed out`                                                                                                                                                                                                                                                  | [서버 오류](#request-timed-out), 또는 메시지에 인터넷 연결이 언급된 경우 [네트워크](#unable-to-connect-to-api)             |
| `API Error: No response from API`                                                                                                                                                                                                                                    | [서버 오류](#no-response-from-api)                                                                    |
| `Server error mid-response. The response above may be incomplete.`                                                                                                                                                                                                   | [서버 오류](#the-response-above-may-be-incomplete)                                                    |
| `Connection lost mid-response` / `Your computer went to sleep mid-response` / `The response stopped arriving`                                                                                                                                                        | [서버 오류](#the-response-above-may-be-incomplete)                                                    |
| `Connection closed mid-response` / `Response stalled mid-stream`                                                                                                                                                                                                     | [서버 오류](#the-response-above-may-be-incomplete)                                                    |
| `Connection lost before a response was produced` / `Your computer went to sleep before a response was produced` / `The response stalled before a response was produced`                                                                                              | [자동 재시도](#automatic-retries)                                                                      |
| `Connection closed while thinking` / `Response stalled while thinking`                                                                                                                                                                                               | [자동 재시도](#automatic-retries)                                                                      |
| `Connection lost while your computer was asleep`                                                                                                                                                                                                                     | [자동 재시도](#automatic-retries)                                                                      |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`                                                                                                                                                                                 | [서버 오류](#auto-mode-cannot-determine-the-safety-of-an-action)                                      |
| `Auto mode could not evaluate this action and is blocking it for safety`                                                                                                                                                                                             | [서버 오류](#auto-mode-cannot-determine-the-safety-of-an-action)                                      |
| `Auto mode classifier transcript exceeded context window`                                                                                                                                                                                                            | [서버 오류](#auto-mode-cannot-determine-the-safety-of-an-action)                                      |
| `Agent aborted: auto mode classifier request refused by the safety safeguard`                                                                                                                                                                                        | [서버 오류](#auto-mode-cannot-determine-the-safety-of-an-action)                                      |
| `The server-side auto mode classifier gave no verdict`                                                                                                                                                                                                               | [서버 오류](#the-server-returned-no-safety-verdict)                                                   |
| `Auto mode is unavailable — the server returned no safety verdict for the last 10 responses`                                                                                                                                                                         | [서버 오류](#the-server-returned-no-safety-verdict)                                                   |
| `Agent terminated early due to an API error`                                                                                                                                                                                                                         | [서버 오류](#agent-terminated-early-due-to-an-api-error)                                              |
| `You've hit your session limit` / `You've hit your weekly limit` / `You've hit your Opus limit` / `You've hit your Sonnet limit`                                                                                                                                     | [사용 제한](#youve-hit-your-session-limit)                                                            |
| `Usage credits required for 1M context`                                                                                                                                                                                                                              | [사용 제한](#usage-credits-required-for-1m-context)                                                   |
| `the prompt to confirm went unanswered — nothing was sent`                                                                                                                                                                                                           | [사용 제한](#the-prompt-to-confirm-went-unanswered)                                                   |
| `Server is temporarily limiting requests`                                                                                                                                                                                                                            | [사용 제한](#server-is-temporarily-limiting-requests)                                                 |
| `Request rejected (429)`                                                                                                                                                                                                                                             | [사용 제한](#request-rejected-429)                                                                    |
| `Credit balance is too low`                                                                                                                                                                                                                                          | [사용 제한](#credit-balance-is-too-low)                                                               |
| `You've hit your monthly spend limit` / `You've hit your individual spend limit` / `You've hit your org's monthly spend limit` / `You've hit your channel's monthly spend limit` / `You've hit your team's shared budget` / `You've hit your individual usage limit` | [사용 제한](#youve-hit-your-monthly-spend-limit)                                                      |
| `Could not update your spend limit`                                                                                                                                                                                                                                  | [사용 제한](#could-not-update-your-spend-limit)                                                       |
| `spend limit reached` / `spend limit unavailable`                                                                                                                                                                                                                    | [사용 제한](#spend-limit-reached)                                                                     |
| `Not logged in · Please run /login`                                                                                                                                                                                                                                  | [인증](#not-logged-in)                                                                              |
| `Could not resolve authentication method`                                                                                                                                                                                                                            | [인증](#could-not-resolve-authentication-method)                                                    |
| `Invalid API key`                                                                                                                                                                                                                                                    | [인증](#invalid-api-key)                                                                            |
| `Your apiKeyHelper script is failing`                                                                                                                                                                                                                                | [인증](#your-apikeyhelper-script-is-failing)                                                        |
| `Invalid auth token · Fix external auth token`                                                                                                                                                                                                                       | [인증](#invalid-request-header-value)                                                               |
| `Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable`                                                                                                                                                                                                    | [인증](#invalid-request-header-value)                                                               |
| `Invalid request header from the environment · Fix the environment variable`                                                                                                                                                                                         | [인증](#invalid-request-header-value)                                                               |
| `This organization has been disabled`                                                                                                                                                                                                                                | [인증](#this-organization-has-been-disabled)                                                        |
| `Your organization has disabled API key authentication`                                                                                                                                                                                                              | [인증](#your-organization-has-disabled-api-key-authentication)                                      |
| `Your organization has disabled Claude subscription access`                                                                                                                                                                                                          | [인증](#your-organization-has-disabled-claude-subscription-access)                                  |
| `Routines are disabled by your organization's policy`                                                                                                                                                                                                                | [인증](#routines-are-disabled-by-your-organizations-policy)                                         |
| `Remote Control is only available when using Claude via api.anthropic.com`                                                                                                                                                                                           | [인증](#remote-control-requires-the-anthropic-api)                                                  |
| `OAuth token refresh failed — run /login to re-authenticate`                                                                                                                                                                                                         | [인증](#remote-control-couldnt-refresh-your-login)                                                  |
| `JWT refresh failed: no OAuth token — run /login`                                                                                                                                                                                                                    | [인증](#remote-control-couldnt-refresh-your-login)                                                  |
| `Claude.ai login expired`                                                                                                                                                                                                                                            | [인증](#remote-control-couldnt-refresh-your-login)                                                  |
| `Claude.ai login was rejected — run /login, then /remote-control`                                                                                                                                                                                                    | [인증](#remote-control-couldnt-refresh-your-login)                                                  |
| `OAuth token unavailable — run /login to restore Remote Control`                                                                                                                                                                                                     | [인증](#remote-control-couldnt-refresh-your-login)                                                  |
| `Signed out of Claude — run /login, then /remote-control`                                                                                                                                                                                                            | [인증](#remote-control-couldnt-refresh-your-login)                                                  |
| `signed-in claude.ai account or organization changed on this machine`                                                                                                                                                                                                | [인증](#remote-control-stopped-because-the-signed-in-account-changed)                               |
| `Remote Control stopped — the app running this session is now signed in to a different Claude account`                                                                                                                                                               | [인증](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts) |
| `Remote Control stopped — the app running this session is signed out of Claude`                                                                                                                                                                                      | [인증](#remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts) |
| `OAuth token revoked` / `OAuth token has expired`                                                                                                                                                                                                                    | [인증](#oauth-token-revoked-or-expired)                                                             |
| `API Error: 401 Invalid authentication credentials`                                                                                                                                                                                                                  | [인증](#api-error-401-invalid-authentication-credentials)                                           |
| `Login expired · Please run /login`                                                                                                                                                                                                                                  | [인증](#login-expired)                                                                              |
| `Claude login not accepted · Run /login, then try again`                                                                                                                                                                                                             | [인증](#claude-login-not-accepted)                                                                  |
| `Artifacts need a claude.ai login`                                                                                                                                                                                                                                   | [인증](#artifacts-need-a-claude-ai-login)                                                           |
| `Not signed in to the Cloud gateway — run /login.`                                                                                                                                                                                                                   | [인증](#administrator-policy-requires-a-cloud-gateway-sign-in)                                      |
| `Administrator policy requires a Cloud gateway sign-in on this machine`                                                                                                                                                                                              | [인증](#administrator-policy-requires-a-cloud-gateway-sign-in)                                      |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                                                                                                                                                                                           | [인증](#login-expired)                                                                              |
| `Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                            | [인증](#your-account-is-on-hold)                                                                    |
| `Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted`                                                                                                                                                     | [인증](#your-account-is-on-hold)                                                                    |
| `Anthropic profile login expired · Re-authenticate your Anthropic profile`                                                                                                                                                                                           | [인증](#anthropic-profile-login-expired)                                                            |
| `Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile`                                                                                                                                                 | [인증](#anthropic-profile-login-expired)                                                            |
| `does not meet scope requirement user:profile`                                                                                                                                                                                                                       | [인증](#oauth-scope-requirement)                                                                    |
| `claude.ai rejected the session token` / `session token rejected`                                                                                                                                                                                                    | [인증](#claude-ai-rejected-the-session-token)                                                       |
| `MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)`                                                                                                                                                                                       | [인증](#mcp-server-needs-you-to-sign-in-again)                                                      |
| `rejected the credential from its headersHelper` / `rejected the Authorization header in its config`                                                                                                                                                                 | [인증](#mcp-server-needs-you-to-sign-in-again)                                                      |
| `MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate`                                                                                                                                                                  | [인증](#mcp-server-needs-you-to-sign-in-again)                                                      |
| `MCP server "<name>" requires re-authorization (token expired)`                                                                                                                                                                                                      | [인증](#mcp-server-needs-you-to-sign-in-again)                                                      |
| `Issuer mismatch in authorization response (RFC 9207)`                                                                                                                                                                                                               | [인증](#issuer-mismatch-in-authorization-response)                                                  |
| `Cloud gateway session expired — run /login to reconnect.`                                                                                                                                                                                                           | [인증](#cloud-gateway-session-expired)                                                              |
| `Cloud gateway <url> no longer accepts this session`                                                                                                                                                                                                                 | [인증](#cloud-gateway-session-expired)                                                              |
| `Sign-in timed out while waiting for you to continue. Try again.`                                                                                                                                                                                                    | [인증](#sign-in-timed-out-while-waiting-for-you-to-continue)                                        |
| `AWS credentials expired or invalid`                                                                                                                                                                                                                                 | [인증](#aws-credentials-expired-or-invalid)                                                         |
| `AWS authentication failed`                                                                                                                                                                                                                                          | [인증](#aws-authentication-failed)                                                                  |
| `Google Cloud credentials expired or invalid`                                                                                                                                                                                                                        | [인증](#google-cloud-credentials-expired-or-invalid)                                                |
| `Google Cloud authentication failed`                                                                                                                                                                                                                                 | [인증](#google-cloud-authentication-failed)                                                         |
| `Microsoft Foundry authentication failed`                                                                                                                                                                                                                            | [인증](#microsoft-foundry-authentication-failed)                                                    |
| `Gateway refused the request`                                                                                                                                                                                                                                        | [인증](#gateway-refused-the-request)                                                                |
| `Could not load AWS credentials` / `Could not load Google Cloud credentials`                                                                                                                                                                                         | [인증](#could-not-load-aws-or-google-cloud-credentials)                                             |
| `AWS default-chain credential resolve timed out`                                                                                                                                                                                                                     | [인증](#aws-default-chain-credential-resolve-timed-out)                                             |
| `Timed out after 60s waiting for AWS`                                                                                                                                                                                                                                | [인증](#bedrock-setup-verification-timed-out-waiting-for-aws)                                       |
| `A request to AWS timed out. Check your network and proxy settings, then try again.`                                                                                                                                                                                 | [인증](#bedrock-setup-verification-timed-out-waiting-for-aws)                                       |
| `Could not load the default credentials` on Google Cloud's Agent Platform                                                                                                                                                                                            | [인증](#could-not-load-aws-or-google-cloud-credentials)                                             |
| `Unable to connect to API`                                                                                                                                                                                                                                           | [네트워크](#unable-to-connect-to-api)                                                                 |
| `Connection refused —` / `Can't reach the API server —` / `No internet route —` / `Couldn't connect through your proxy` / `Connection dropped`, 각각 괄호 안의 오류 코드로 끝남                                                                                                   | [네트워크](#unable-to-connect-to-api)                                                                 |
| `Unable to connect to Anthropic services` during setup                                                                                                                                                                                                               | [네트워크](#unable-to-connect-to-anthropic-services)                                                  |
| `Socket is closed`                                                                                                                                                                                                                                                   | [네트워크](#socket-is-closed)                                                                         |
| `Waiting for API response · will retry in`                                                                                                                                                                                                                           | [자동 재시도](#automatic-retries), 또는 지속되는 경우 [네트워크](#unable-to-connect-to-api)                        |
| `API returned an empty or malformed response`                                                                                                                                                                                                                        | [네트워크](#api-returned-an-empty-or-malformed-response)                                              |
| `Streaming response ended before any complete data was received`                                                                                                                                                                                                     | [네트워크](#streaming-response-ended-before-any-complete-data-was-received)                           |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"`                                                                                                                                                                   | [네트워크](#bedrock-streaming-response-has-an-unexpected-content-type)                                |
| `SSL certificate verification failed`                                                                                                                                                                                                                                | [네트워크](#ssl-certificate-errors)                                                                   |
| `SSL certificate error (...)` during login or startup                                                                                                                                                                                                                | [네트워크](#ssl-certificate-errors)                                                                   |
| `unable to get local issuer certificate`                                                                                                                                                                                                                             | [네트워크](#ssl-certificate-errors)                                                                   |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                                                                                                                                                                                           | [네트워크](#host-not-allowed-in-a-cloud-session)                                                      |
| `proxy refused the connection`                                                                                                                                                                                                                                       | [네트워크](#the-proxy-refused-the-connection)                                                         |
| `403` with `This GraphQL query is not enabled for this session` in a cloud session                                                                                                                                                                                   | [GitHub proxy](/docs/ko/cloud-environments#github-proxy)                                               |
| `The cloud environments service returned an empty response` / `The cloud environments service returned a response in an unexpected format`                                                                                                                           | [네트워크](#the-cloud-environments-service-returned-an-empty-or-unexpected-response)                  |
| `Couldn't reconnect to your Remote Control session`                                                                                                                                                                                                                  | [네트워크](#couldnt-reconnect-to-your-remote-control-session)                                         |
| `N sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.`                                                                                                                                               | [네트워크](#sessions-ended-while-this-machine-was-offline)                                            |
| `Couldn't share the transcript.`                                                                                                                                                                                                                                     | [네트워크](#couldnt-share-the-transcript)                                                             |
| `Prompt is too long` / `Input is too long for requested model`                                                                                                                                                                                                       | [요청 오류](#prompt-is-too-long)                                                                      |
| `Prompt is too long · automatic compaction failed:`                                                                                                                                                                                                                  | [요청 오류](#prompt-is-too-long)                                                                      |
| `Prompt is too long · this conversation is a single exchange` / `A single-exchange conversation cannot be compacted`                                                                                                                                                 | [요청 오류](#prompt-is-too-long)                                                                      |
| `Context limit reached · /compact or /clear to continue`                                                                                                                                                                                                             | [요청 오류](#prompt-is-too-long)                                                                      |
| `Context limit reached · /clear to continue`                                                                                                                                                                                                                         | [요청 오류](#prompt-is-too-long)                                                                      |
| `capability_rejected: prompt_too_long` on a Claude apps gateway session                                                                                                                                                                                              | [요청 오류](#prompt-is-too-long)                                                                      |
| `upstream rejected the request` / `request too large for this upstream` on a Claude apps gateway session                                                                                                                                                             | [업스트림 오류 메시지](/docs/ko/claude-apps-gateway-config#upstream-error-messages)                             |
| `upstream rate limit exceeded` on a Claude apps gateway session                                                                                                                                                                                                      | [업스트림 오류 메시지](/docs/ko/claude-apps-gateway-config#upstream-error-messages)                             |
| `all upstreams failed (N attempted)` on a Claude apps gateway session                                                                                                                                                                                                | [업스트림 오류 메시지](/docs/ko/claude-apps-gateway-config#upstream-error-messages)                             |
| `Claude Code may not be enabled for your organization` after a Claude apps gateway sign-in                                                                                                                                                                           | [Claude apps gateway 문제 해결](/docs/ko/claude-apps-gateway-deploy#troubleshooting)                       |
| `Context exceeds the ...-token limit by ... tokens` in `/context` output                                                                                                                                                                                             | [요청 오류](#context-exceeds-the-token-limit)                                                         |
| `Error during compaction: Conversation too long`                                                                                                                                                                                                                     | [요청 오류](#error-during-compaction-conversation-too-long)                                           |
| `Request too large`                                                                                                                                                                                                                                                  | [요청 오류](#request-too-large)                                                                       |
| `Request too large for the API's 32MB request limit`                                                                                                                                                                                                                 | [요청 오류](#request-too-large)                                                                       |
| `Image was too large`                                                                                                                                                                                                                                                | [요청 오류](#image-was-too-large)                                                                     |
| `Unable to resize image`                                                                                                                                                                                                                                             | [요청 오류](#unable-to-resize-image)                                                                  |
| `PDF too large` / `PDF is password protected`                                                                                                                                                                                                                        | [요청 오류](#pdf-errors)                                                                              |
| `Extra inputs are not permitted`                                                                                                                                                                                                                                     | [요청 오류](#extra-inputs-are-not-permitted)                                                          |
| `API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid` / `Property keys should match pattern`                                                                                                                                                      | [요청 오류](#tool-input-schema-is-invalid)                                                            |
| `There's an issue with the selected model`                                                                                                                                                                                                                           | [요청 오류](#theres-an-issue-with-the-selected-model)                                                 |
| `Model ... is not a recognized model id`                                                                                                                                                                                                                             | [요청 오류](#model-is-not-a-recognized-model-id)                                                      |
| `Model ... not found`                                                                                                                                                                                                                                                | [요청 오류](#model-not-found)                                                                         |
| `Claude Opus is not available with the Claude Pro plan`                                                                                                                                                                                                              | [요청 오류](#claude-opus-is-not-available-with-the-claude-pro-plan)                                   |
| `Claude Code ... does not support this model; version ... or newer is required`                                                                                                                                                                                      | [요청 오류](#claude-code-does-not-support-this-model)                                                 |
| `Claude Code ... is older than the minimum version required by your organization's policy`                                                                                                                                                                           | [요청 오류](#claude-code-does-not-support-this-model)                                                 |
| `Model ... is restricted by your organization's settings`                                                                                                                                                                                                            | [요청 오류](#model-is-restricted-by-your-organizations-settings)                                      |
| `Model switch ... blocked by a PreModelSwitch hook`                                                                                                                                                                                                                  | [요청 오류](#model-switch-was-blocked-by-a-premodelswitch-hook)                                       |
| `couldn't save it as your default` / `couldn't confirm it was saved as your default`                                                                                                                                                                                 | [요청 오류](#couldnt-save-it-as-your-default)                                                         |
| `thinking.type.enabled is not supported for this model`                                                                                                                                                                                                              | [요청 오류](#thinking-type-enabled-is-not-supported-for-this-model)                                   |
| `Effort '<level>' isn't available with thinking turned off on this model`                                                                                                                                                                                            | [요청 오류](#effort-isnt-available-with-thinking-turned-off)                                          |
| `effort '<level>' is not supported when thinking is disabled`                                                                                                                                                                                                        | [요청 오류](#effort-isnt-available-with-thinking-turned-off)                                          |
| `max_tokens must be greater than thinking.budget_tokens`                                                                                                                                                                                                             | [요청 오류](#thinking-budget-exceeds-output-limit)                                                    |
| `API Error: 400 due to tool use concurrency issues`                                                                                                                                                                                                                  | [요청 오류](#tool-use-or-thinking-block-mismatch)                                                     |
| `API Error: 400 orphaned tool_result in conversation history`                                                                                                                                                                                                        | [요청 오류](#tool-use-or-thinking-block-mismatch)                                                     |
| `API Error: 400 duplicate tool_use ID in conversation history`                                                                                                                                                                                                       | [요청 오류](#tool-use-or-thinking-block-mismatch)                                                     |
| `[Unsupported tool content removed]`                                                                                                                                                                                                                                 | [요청 오류](#unsupported-tool-content-removed)                                                        |
| `role 'system' must precede an 'assistant' message`                                                                                                                                                                                                                  | [요청 오류](#role-system-must-precede-an-assistant-message)                                           |
| `Invalid encrypted_content in search_result block` / `Invalid encrypted_index in text block` / `Failed to decrypt web search result content`                                                                                                                         | [요청 오류](#invalid-encrypted-content-in-search-result-block)                                        |
| `server_tool_use.name: Input should be` on every turn of a resumed session                                                                                                                                                                                           | [요청 오류](#unsupported-tool-content-removed)                                                        |
| `<model> can't help with this. Start a new session to continue`                                                                                                                                                                                                      | [요청 오류](#usage-policy-refusal)                                                                    |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`                                                                                                                                                                        | [요청 오류](#usage-policy-refusal)                                                                    |
| `<model>'s safeguards flagged this message`                                                                                                                                                                                                                          | [요청 오류](#safety-measures-flagged-a-cybersecurity-topic)                                           |
| `Opus 5.5's safeguards flagged this session`                                                                                                                                                                                                                         | [요청 오류](#safety-measures-flagged-a-cybersecurity-topic)                                           |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                                                                                                                                                                                    | [요청 오류](#safety-measures-flagged-a-cybersecurity-topic)                                           |
| `Installation was killed before it could finish (exit code 137)`                                                                                                                                                                                                     | [설치 오류](#installation-was-killed-before-it-could-finish)                                          |
| `The connection dropped while downloading the update`                                                                                                                                                                                                                | [설치 오류](#the-connection-dropped-while-downloading-the-update)                                     |
| `Download timed out: exceeded the total deadline`                                                                                                                                                                                                                    | [설치 오류](#the-connection-dropped-while-downloading-the-update)                                     |
| `--bg and --print conflict`                                                                                                                                                                                                                                          | [명령줄 오류](#command-line-errors)                                                                    |
| `Cloud sessions cannot be created from a --restricted session`                                                                                                                                                                                                       | [명령줄 오류](#cloud-sessions-cannot-be-created-from-a-restricted-session)                             |
| `Cloud sessions are disabled by your organization's policy`                                                                                                                                                                                                          | [명령줄 오류](#cloud-sessions-are-disabled-by-your-organizations-policy)                               |
| `Couldn't verify your organization's policy for cloud sessions`                                                                                                                                                                                                      | [명령줄 오류](#cloud-sessions-are-disabled-by-your-organizations-policy)                               |
| `Error: --json-schema is not a valid JSON Schema`                                                                                                                                                                                                                    | [명령줄 오류](#command-line-errors)                                                                    |
| `Error: Invalid --agents configuration:`                                                                                                                                                                                                                             | [명령줄 오류](#invalid-agents-configuration)                                                           |
| `Error: Settings file exceeds the 2MiB limit`                                                                                                                                                                                                                        | [명령줄 오류](#settings-file-exceeds-the-2mib-limit)                                                   |
| `The current directory no longer exists (it was deleted or moved)` / `Can't read the current directory`                                                                                                                                                              | [명령줄 오류](#the-current-directory-no-longer-exists)                                                 |
| `Temp directory <dir> ... Refusing to use it` / `ENOSPC: no space left on device, mkdir '<dir>'`                                                                                                                                                                     | [명령줄 오류](#temp-directory-refused-or-cannot-be-created)                                            |
| `couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded`                                                                                                                                                                        | [명령줄 오류](#directory-couldnt-be-resolved-to-a-real-location)                                       |
| `Error: Workspace not trusted` when starting Remote Control                                                                                                                                                                                                          | [명령줄 오류](#workspace-not-trusted-when-starting-remote-control)                                     |
| `` `<flag>` before `remote-control` is not carried over to the sessions Remote Control starts ``                                                                                                                                                                     | [명령줄 오류](#not-carried-over-to-the-sessions-remote-control-starts)                                 |
| `` `claude import` is not yet available in this build ``                                                                                                                                                                                                             | [명령줄 오류](#claude-import-is-not-yet-available-in-this-build)                                       |
| `Could not read Claude Code config`                                                                                                                                                                                                                                  | [명령줄 오류](#could-not-read-claude-code-config)                                                      |
| `Could not import <server>: <reason>`                                                                                                                                                                                                                                | [명령줄 오류](#could-not-import-a-server-from-claude-desktop)                                          |
| `Cannot add MCP server to scope: managed`                                                                                                                                                                                                                            | [명령줄 오류](#cannot-add-mcp-server-to-the-managed-scope)                                             |
| `is Anthropic-hosted and doesn't support local OAuth`                                                                                                                                                                                                                | [명령줄 오류](#anthropic-hosted-and-doesnt-support-local-oauth)                                        |
| `Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes`                                                                                                                                                                                      | [명령줄 오류](#cant-read-mcp-json)                                                                     |
| `Server rejected the Authorization header minted by the configured headersHelper`                                                                                                                                                                                    | [명령줄 오류](#server-rejected-the-authorization-header-minted-by-the-configured-headershelper)        |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                                                                                                                                                                                             | [명령줄 오류](#mcp-permission-prompt-tool-not-found)                                                   |
| `OAuth callback port <port> is already in use — another process may be holding it`                                                                                                                                                                                   | [명령줄 오류](#oauth-callback-port-is-already-in-use)                                                  |
| `No available ports for OAuth redirect`                                                                                                                                                                                                                              | [명령줄 오류](#no-available-ports-for-oauth-redirect)                                                  |
| `Shell command failed for pattern "..."`, from `/security-review` or any skill that injects dynamic context                                                                                                                                                          | [명령줄 오류](#security-review-fails-without-origin-head)                                              |
| `Shell command permission check failed for pattern "..."`, from a skill that injects dynamic context                                                                                                                                                                 | [명령줄 오류](#security-review-fails-without-origin-head)                                              |
| ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``                                                                                                                                                                             | [명령줄 오류](#security-review-fails-without-origin-head)                                              |
| `Input must be provided either through stdin or as a prompt argument when using --print`                                                                                                                                                                             | [명령줄 오류](#input-must-be-provided-when-using-print)                                                |
| `Error: Input contained only whitespace`                                                                                                                                                                                                                             | [명령줄 오류](#input-contained-only-whitespace)                                                        |
| `Blank prompt — the message was only whitespace, so nothing was sent to the model.`                                                                                                                                                                                  | [명령줄 오류](#input-contained-only-whitespace)                                                        |
| `Error: stream-json input carried over 256M characters with no newline`                                                                                                                                                                                              | [명령줄 오류](#stream-json-input-carried-over-256m-characters-with-no-newline)                         |
| `Unknown command: /<name>`, with or without a `Did you mean` suggestion                                                                                                                                                                                              | [명령줄 오류](#unknown-command)                                                                        |
| `Diff is too large for ultrareview` / `PR #<N> is too large for ultrareview`                                                                                                                                                                                         | [명령줄 오류](#diff-is-too-large-for-ultrareview)                                                      |
| `Could not find merge-base with <branch>`                                                                                                                                                                                                                            | [명령줄 오류](#could-not-find-merge-base-with-the-base-branch)                                         |
| `Your checkout has no branches (detached HEAD only)`                                                                                                                                                                                                                 | [명령줄 오류](#your-checkout-has-no-branches)                                                          |
| `Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected`                                                                                                                                     | [명령줄 오류](#no-github-account-is-connected-to-your-claude-account)                                  |
| `Your connected GitHub account can't see <owner>/<repo>`                                                                                                                                                                                                             | [명령줄 오류](#your-connected-github-account-cant-see-the-repository)                                  |
| `The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead`                                                                                                                                           | [명령줄 오류](#the-github-app-preflight-failed-transiently)                                            |
| `GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud`                                                                                                                                                                     | [명령줄 오류](#github-isnt-connected-to-your-claude-account)                                           |
| `Single sign-on authorization needed`                                                                                                                                                                                                                                | [명령줄 오류](#single-sign-on-authorization-needed)                                                    |
| `Failed to resume the conversation`                                                                                                                                                                                                                                  | [명령줄 오류](#failed-to-resume-the-conversation)                                                      |
| `No conversation found with session ID: <session-id>`                                                                                                                                                                                                                | [명령줄 오류](#no-conversation-found-with-the-session-id)                                              |
| `Cannot switch renderers in this session`                                                                                                                                                                                                                            | [명령줄 오류](#cannot-switch-renderers-in-this-session)                                                |
| `Cannot switch renderers while work is running in the background`                                                                                                                                                                                                    | [명령줄 오류](#cannot-switch-renderers-in-this-session)                                                |
| `Couldn't open Claude Desktop`                                                                                                                                                                                                                                       | [명령줄 오류](#couldnt-open-claude-desktop)                                                            |
| `Failed to open Claude Desktop. Please try opening it manually.`                                                                                                                                                                                                     | [명령줄 오류](#couldnt-open-claude-desktop)                                                            |
| `Couldn't read your Zed keymap` / `Couldn't back up your Zed keymap` / `Couldn't update your Zed keymap`                                                                                                                                                             | [명령줄 오류](#terminal-setup-left-your-zed-keymap-unchanged)                                          |
| `Your Zed keymap isn't a readable list of keybindings`                                                                                                                                                                                                               | [명령줄 오류](#terminal-setup-left-your-zed-keymap-unchanged)                                          |
| `Skill usage reports are not available on this connection.`                                                                                                                                                                                                          | [명령줄 오류](#skill-usage-reports-are-not-available-on-this-connection)                               |
| `Custom output styles can't be selected over Remote Control or from a relayed message`                                                                                                                                                                               | [명령줄 오류](#custom-output-styles-cant-be-selected-over-remote-control)                              |
| `Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load`                                                                                                                                                           | [명령줄 오류](#output-styles-are-saved-to-local-settings-which-this-session-doesnt-load)               |
| `` `plugin eval` is currently in early access `` / `` `plugin eval` is currently unavailable ``                                                                                                                                                                      | [플러그인 오류](#plugin-eval-is-currently-in-early-access)                                              |
| `Marketplace "<name>" is registered from an untrusted source`                                                                                                                                                                                                        | [플러그인 오류](#marketplace-is-registered-from-an-untrusted-source)                                    |
| `Marketplace "<name>" is already added from a different source`                                                                                                                                                                                                      | [플러그인 오류](#marketplace-is-already-added-from-a-different-source)                                  |
| `"<name>" is another spelling of "<reserved>", a reserved marketplace name`                                                                                                                                                                                          | [플러그인 오류](#marketplace-name-is-another-spelling-of-a-reserved-name)                               |
| `references ${user_config.*} in a shell-form command`                                                                                                                                                                                                                | [플러그인 오류](#plugin-command-references-user-config)                                                 |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                                                                                                                                                                                   | [플러그인 오류](#plugin-command-references-user-config)                                                 |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                                                                                                                                                                                  | [플러그인 오류](#plugin-command-references-user-config)                                                 |
| `Plugin archive integrity check failed`                                                                                                                                                                                                                              | [플러그인 오류](#plugin-archive-integrity-check-failed)                                                 |
| `path escapes plugin directory`                                                                                                                                                                                                                                      | [플러그인 오류](#path-escapes-plugin-directory)                                                         |
| `path could not be checked`                                                                                                                                                                                                                                          | [플러그인 오류](#path-could-not-be-checked)                                                             |
| `its marketplace entry path does not stay inside the marketplace directory`                                                                                                                                                                                          | [플러그인 오류](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                 |
| `Plugin source path refused`                                                                                                                                                                                                                                         | [플러그인 오류](#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory)                 |
| `Failed to load marketplace configuration`                                                                                                                                                                                                                           | [플러그인 오류](#failed-to-load-marketplace-configuration)                                              |
| `Marketplace configuration file is corrupted`                                                                                                                                                                                                                        | [플러그인 오류](#failed-to-load-marketplace-configuration)                                              |
| `Plugin "<name>@synced" is required by your organization and can't be disabled here`                                                                                                                                                                                 | [플러그인 오류](#plugin-is-required-by-your-organization)                                               |
| `would be spawned with zero tools — refusing`                                                                                                                                                                                                                        | [도구 오류](#agent-would-be-spawned-with-zero-tools)                                                  |
| `File is covered by a Read deny rule in your permission settings`                                                                                                                                                                                                    | [도구 오류](#file-is-covered-by-a-read-deny-rule)                                                     |
| `subagent_type is required: the general-purpose agent is not available in this session`                                                                                                                                                                              | [도구 오류](#subagent-type-is-required)                                                               |
| `Error: this write left the memory index at MEMORY.md at ..., over its ... read limit`                                                                                                                                                                               | [도구 오류](#memory-index-is-over-its-read-limit)                                                     |
| `pkill: refusing to run`                                                                                                                                                                                                                                             | [도구 오류](#pkill-pattern-matches-the-claude-code-process)                                           |
| `Failed to write to <name>'s inbox — nothing was sent`                                                                                                                                                                                                               | [도구 오류](#failed-to-write-to-a-teammate-inbox)                                                     |
| `Failed to write the plan approval request to the lead's inbox — plan not submitted`                                                                                                                                                                                 | [도구 오류](#failed-to-write-to-a-teammate-inbox)                                                     |
| `Its agent definition was not restored: the folder its definition file came from is not trusted`                                                                                                                                                                     | [도구 오류](#teammate-agent-definition-not-restored)                                                  |
| `Message too large for cross-session delivery`                                                                                                                                                                                                                       | [도구 오류](#message-too-large-for-cross-session-delivery)                                            |
| `Too many messages to this session just now`                                                                                                                                                                                                                         | [도구 오류](#too-many-messages-to-this-session-just-now)                                              |
| `Refusing to send: reply target is a symlink` / `Refusing to send: cannot vet reply target`                                                                                                                                                                          | [도구 오류](#refusing-to-send-a-cross-session-message)                                                |
| `Refusing to send: connected endpoint is not the expected process` / `Refusing to send: connected endpoint identity could not be read`                                                                                                                               | [도구 오류](#refusing-to-send-a-cross-session-message)                                                |
| `Refusing to send: connected endpoint is not owned by this user` / `Refusing to send: connected endpoint owner could not be read`                                                                                                                                    | [도구 오류](#refusing-to-send-a-cross-session-message)                                                |
| `Refusing to send: connected endpoint is a different process with the expected pid`                                                                                                                                                                                  | [도구 오류](#refusing-to-send-a-cross-session-message)                                                |
| `Refusing to read <path>: its symlink resolution changed after permission was checked (<reason>)` / `Refusing to search <path>: its symlink resolution changed after permission was checked`                                                                         | [도구 오류](#refusing-after-a-symlink-changed)                                                        |
| `Refusing to write <path>: its parent-directory symlink resolution changed after permission was checked` / `Refusing to write <path>: it is a symbolic link. Write to the link's target path instead`                                                                | [도구 오류](#refusing-after-a-symlink-changed)                                                        |
| `Refusing to write through symlink: <path>` / `Refusing to write into symlinked directory: <path>`                                                                                                                                                                   | [도구 오류](#refusing-after-a-symlink-changed)                                                        |
| `Refusing to search <path>: a path one of its Read deny rules is written through changed while the search was being prepared` / `Refusing to search <path>: it could not be opened`                                                                                  | [도구 오류](#refusing-after-a-symlink-changed)                                                        |
| `its permission check expired before it ran (too many concurrent file operations)` / `ripgrep was found only by name on PATH`                                                                                                                                        | [도구 오류](#refusing-after-a-symlink-changed)                                                        |
| `task output swap refused (tasks dir moved or linked)`                                                                                                                                                                                                               | [도구 오류](#task-output-swap-refused)                                                                |
| `Command killed: its output file was replaced or could no longer be verified`                                                                                                                                                                                        | [도구 오류](#task-output-swap-refused)                                                                |
| `the source file is not valid UTF-8 text` / `the source file is not valid UTF-16 text`                                                                                                                                                                               | [도구 오류](#the-source-file-is-not-valid-utf-8-text)                                                 |
| `the source file has the replacement character U+FFFD`                                                                                                                                                                                                               | [도구 오류](#the-source-file-is-not-valid-utf-8-text)                                                 |
| `Reading a local file from outside this session's connected folders, or through a link, needs the approval card`                                                                                                                                                     | [도구 오류](#reading-a-local-file-from-outside-the-connected-folders)                                 |
| `cannot read file_path (...) — the file could not be examined, and no one can answer the approval card`                                                                                                                                                              | [도구 오류](#reading-a-local-file-from-outside-the-connected-folders)                                 |
| `WebFetch cannot fetch localhost or other hostnames without a dot`                                                                                                                                                                                                   | [도구 오류](#webfetch-cannot-fetch-localhost)                                                         |
| `Can't open MCP settings while no terminal is attached to this background session`                                                                                                                                                                                   | [백그라운드 세션 오류](#commands-refused-in-a-background-session)                                          |
| `Can't open MCP settings in a background session`                                                                                                                                                                                                                    | [백그라운드 세션 오류](#commands-refused-in-a-background-session)                                          |
| `blocked because the path is spelled in a form that cannot be safely resolved`                                                                                                                                                                                       | [백그라운드 세션 오류](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved)               |
| `blocked because the path is network-shaped`                                                                                                                                                                                                                         | [백그라운드 세션 오류](#write-or-command-blocked-because-the-path-names-a-network-location)                |
| `is isolated in the worktree <path>, but this command <reason>. Refusing to run it`                                                                                                                                                                                  | [백그라운드 세션 오류](#command-blocked-by-the-worktree-isolation-checks)                                  |
| `too complex to verify that it stays inside the worktree`                                                                                                                                                                                                            | [백그라운드 세션 오류](#command-blocked-by-the-worktree-isolation-checks)                                  |
| `This session has no saved transcript`                                                                                                                                                                                                                               | [백그라운드 세션 오류](#this-session-has-no-saved-transcript)                                              |
| `Can't open — this session is running in another terminal`                                                                                                                                                                                                           | [백그라운드 세션 오류](#this-session-is-running-in-another-terminal)                                       |
| `This conversation is already open in another running Claude session`                                                                                                                                                                                                | [백그라운드 세션 오류](#this-session-is-running-in-another-terminal)                                       |
| `This session's saved conversation is no longer on disk`                                                                                                                                                                                                             | [백그라운드 세션 오류](#this-sessions-saved-conversation-is-no-longer-on-disk)                             |
| `kept <id> — its worktree is still at <path>`                                                                                                                                                                                                                        | [백그라운드 세션 오류](#worktree-has-commits-that-are-not-pushed-anywhere)                                 |
| `kept <id> — <n> unpushed commits on <branch>`                                                                                                                                                                                                                       | [백그라운드 세션 오류](#worktree-has-commits-that-are-not-pushed-anywhere)                                 |
| `kept <id> — worktree has commits that are not pushed anywhere`                                                                                                                                                                                                      | [백그라운드 세션 오류](#worktree-has-commits-that-are-not-pushed-anywhere)                                 |
| `terminal host process died — press Enter to restart` / `This session's terminal host process died`                                                                                                                                                                  | [백그라운드 세션 오류](#terminal-host-process-died)                                                        |
| `Session isn't responding` / `Press enter again to restart this session — it isn't responding`                                                                                                                                                                       | [백그라운드 세션 오류](#session-isnt-responding)                                                           |
| `Session <id> was stopped while the respawn was in flight`                                                                                                                                                                                                           | [백그라운드 세션 오류](#session-was-stopped-while-the-respawn-was-in-flight)                               |
| `This session was running agent '<name>', which is no longer available`                                                                                                                                                                                              | [백그라운드 세션 오류](#session-agent-no-longer-available)                                                 |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                                                                                                                                                                                          | [백그라운드 세션 오류](#claude_code_process_wrapper-launcher-errors)                                       |
| `EUNKNOWN: unknown error, uv_spawn`                                                                                                                                                                                                                                  | [백그라운드 세션 오류](#eunknown-when-starting-a-background-session)                                       |
| `EACCES: permission denied, posix_spawn`                                                                                                                                                                                                                             | [백그라운드 세션 오류](#eacces-when-starting-a-background-session)                                         |
| `exited before it became reachable`                                                                                                                                                                                                                                  | [백그라운드 세션 오류](#background-service-exited-before-it-became-reachable)                              |
| `Couldn't start a background session (working directory no longer exists or is not accessible: ...)`                                                                                                                                                                 | [백그라운드 세션 오류](#working-directory-no-longer-exists-when-starting-a-background-session)             |
| `Claude Code is being updated by npm on this machine (still not runnable after 2 min, ...)`                                                                                                                                                                          | [백그라운드 세션 오류](#eacces-when-starting-a-background-session)                                         |
| `Claude Code process exited with code N`                                                                                                                                                                                                                             | [래퍼 및 IDE 오류](#claude-code-process-exited-with-code-n)                                            |
| `The connection to Claude Code ended before this message completed`                                                                                                                                                                                                  | [래퍼 및 IDE 오류](#the-connection-to-claude-code-ended-before-this-message-completed)                 |
| `Could not locate the Claude CLI on PATH`                                                                                                                                                                                                                            | [래퍼 및 IDE 오류](#could-not-locate-the-claude-cli-on-path)                                           |
| `Restored the code, but skipped N files`                                                                                                                                                                                                                             | [되돌리기 경고 및 오류](#restored-the-code-but-skipped-files)                                              |
| `No files were restored: N files failed (backup missing, or the file could not be updated)`                                                                                                                                                                          | [되돌리기 경고 및 오류](#no-files-were-restored)                                                           |
| `Transcript writes are failing (...)`                                                                                                                                                                                                                                | [세션 저장 경고](#transcript-writes-are-failing)                                                        |
| `Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set`                                                                                                                                                                                                  | [세션 저장 경고](#transcript-saving-is-off-skip-prompt-history)                                         |
| `Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker`                                                                                                                                                                                              | [세션 저장 경고](#transcript-saving-is-off-child-session-marker)                                        |
| `Claude Code's fullscreen renderer didn't finish starting last time on this machine` / `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`                                                                                            | [구성 경고](#fullscreen-failed-start-notice)                                                          |
| `Claude Code exited after an unrecoverable interface error (...)`                                                                                                                                                                                                    | [구성 경고](#exited-after-an-unrecoverable-interface-error)                                           |
| `Agent descriptions are over the 15.0k-token limit`                                                                                                                                                                                                                  | [구성 경고](#agent-descriptions-are-over-the-15000-token-limit)                                       |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                                                                                                                                                                                  | [구성 경고](#workspace-has-not-been-trusted)                                                          |
| `is a network path, which cannot be added as a working directory`                                                                                                                                                                                                    | [구성 경고](#working-directory-is-a-network-path)                                                     |
| `Remote managed settings failed to load (<cause>)`                                                                                                                                                                                                                   | [구성 경고](#remote-managed-settings-failed-to-load)                                                  |
| `Managed settings were not approved; exiting without applying them.`                                                                                                                                                                                                 | [구성 경고](#managed-settings-were-not-approved)                                                      |
| `MCP server <name> is blocked by enterprise managed policy`                                                                                                                                                                                                          | [구성 경고](#mcp-server-is-blocked-by-enterprise-managed-policy)                                      |
| `Managed settings document could not be parsed as a JSON object; none of its settings are in effect. Fix or remove it.`                                                                                                                                              | [구성 경고](#managed-settings-document-could-not-be-parsed)                                           |
| `Managed settings drop-in directory could not be read`                                                                                                                                                                                                               | [구성 경고](#managed-settings-document-could-not-be-parsed)                                           |
| `otelHeadersHelper failed; telemetry is not being exported. See /status: ...`                                                                                                                                                                                        | [구성 경고](#otelheadershelper-failed)                                                                |
| `"crossSessionInbound" must be one of "accept", "hold", "refuse"`                                                                                                                                                                                                    | [구성 경고](#crosssessioninbound-must-be-one-of-accept-hold-refuse)                                   |
| `headersHelper not run — this workspace has no persisted trust`                                                                                                                                                                                                      | [구성 경고](#headershelper-not-run)                                                                   |
| `Invalid permission rule "..." was skipped: Malformed Tool(content) rule`                                                                                                                                                                                            | [구성 경고](#malformed-tool-content-rule)                                                             |
| `... is not matched by file permission checks`                                                                                                                                                                                                                       | [구성 경고](#is-not-matched-by-file-permission-checks)                                                |
| `... has a wildcard before the rest of the command`                                                                                                                                                                                                                  | [구성 경고](#has-a-wildcard-before-the-rest-of-the-command)                                           |
| `CLAUDE_CODE_DISABLE_1M_CONTEXT is set, but the 200K limit isn't enforced`                                                                                                                                                                                           | [구성 경고](#the-200k-limit-isnt-enforced)                                                            |
| `[claude-code:unrecognized_model]`                                                                                                                                                                                                                                   | [구성 경고](#unrecognized-model-id-on-a-request)                                                      |
| `Stale sandbox mask files left by a killed session`                                                                                                                                                                                                                  | [구성 경고](#stale-sandbox-mask-files-left-by-a-killed-session)                                       |
| 응답 품질이 평소보다 낮아 보임                                                                                                                                                                                                                                                    | [응답 품질](#responses-seem-lower-quality-than-usual)                                                 |

<h2 id="automatic-retries">
  자동 재시도
</h2>

Claude Code는 오류를 표시하기 전에 지수 백오프를 사용하여 일시적 오류를 최대 10회까지 재시도합니다. Claude의 응답 중간에 도착한 오류는 항상 재시도하지 않습니다. 이 페이지의 오류 중 하나를 보면 Claude Code는 이미 해당 오류에 적용되는 모든 재시도를 수행했습니다. 아래 목록은 어떤 오류가 전체 예산을 받는지, 어떤 오류가 더 작은 예산을 받는지, 어떤 오류가 재시도되지 않는지를 나타냅니다.

Claude Code는 다음 오류를 재시도합니다:

* Claude의 응답이 스트리밍되기 전에 도착하는 서버 오류, 과부하 응답 및 요청 시간 초과.
* 끊어진 연결. Claude가 응답의 어떤 부분도 완료하기 전에 요청 중간에 연결이 끊어지면, 생각 포함하여 Claude Code는 동일한 백오프로 요청을 다시 발행하고 일부 텍스트가 이미 스트리밍되기 시작했더라도 턴이 계속됩니다. Claude가 생각을 마친 후 텍스트나 도구 호출을 시작하기 전에 끊어지면 Claude Code는 대신 요청을 빠르게 연속으로 최대 2회까지 다시 발행하고, 연결이 계속 끊어지면 `Connection lost before a response was produced`로 턴을 종료합니다.
* Claude Code가 요청 중간에 컴퓨터가 절전 모드로 전환되어 끊어진 연결을 감지합니다. Claude Code는 이를 위의 규칙에 따른 끊어진 연결로 계산합니다. 재시도 레이블이 특정 이유를 명시하면 `Connection lost while your computer was asleep`로 읽히며, Claude가 생각을 마친 후 텍스트나 도구 호출을 시작하기 전에 턴이 종료되면 메시지는 `Your computer went to sleep before a response was produced`로 읽힙니다.
* 응답 헤더는 도착했지만 Claude의 응답이 도착하지 않았거나, Claude가 생각을 마쳤지만 텍스트나 도구 호출을 시작하지 않은 경우의 정체된 응답 스트림: Claude Code는 정체된 연결을 중단하고 위의 10회 시도 예산 외에 최대 1회까지 요청을 다시 발행합니다. Claude가 생각을 마친 후 텍스트나 도구 호출을 시작하기 전에 응답이 두 번째로 정체되면 Claude Code는 `The response stalled before a response was produced`로 턴을 종료합니다.
* API가 [첫 바이트 기한이 실행되는](/docs/ko/network-config#streaming-idle-watchdogs) 연결에서 응답 헤더로 응답하지 않는 스트리밍 요청: Claude Code는 기한에서 중단하고 재시도 예산 내에서 모델 요청당 최대 1회까지 다시 보낸 후, 해당 시도도 응답이 없으면 [No response from API](#no-response-from-api)로 턴을 종료합니다. 다른 연결에서는 요청이 `API_TIMEOUT_MS`를 기다립니다. `CLAUDE_CODE_RETRY_WATCHDOG`를 설정하면 1회 재시도 제한이 적용되지 않습니다.
* 임시 429 스로틀, 하지만 게이트웨이의 지출 한도 `429`는 아닙니다. 이는 스로틀이 아닙니다. [Spend limit reached](#spend-limit-reached)를 참조하세요.
  * claude.ai 구독으로 로그인한 경우, 여기에는 플랜의 할당량 헤더를 전달하지 않는 429 스로틀이 포함됩니다. v2.1.199 이전에는 Claude Code가 API 키 및 Enterprise 로그인에 대해서만 해당 스로틀을 재시도했습니다.
* 입력 더하기 `max_tokens`이 컨텍스트 한도를 초과하기 때문에 거부된 요청. 변경하지 않고 다시 보내면 같은 방식으로 실패하므로 Claude Code는 감소된 `max_tokens`으로 재시도하고, 두 가지 경우에 재시도를 중지하고 대신 압축합니다:
  * 감소가 맞지 않을 때, 예를 들어 대화 자체가 컨텍스트 윈도우를 거의 채울 때.
  * 재시도가 `max_tokens`을 더 이상 줄일 수 없을 때. v2.1.218 이전에는 Claude Code가 여전히 맞지 않는 감소된 요청을 다시 보낼 수 있었습니다. 예를 들어 확장 생각 예산이 남은 컨텍스트를 초과했을 때, 재시도 예산이 소진될 때까지입니다.
* [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai)에서 만료되었거나 누락된 Google Cloud 자격증명, 또는 컴퓨터에서 로드하지 못한 AWS 자격증명. Claude Code는 캐시된 자격증명을 버리고 최대 2회까지 재시도한 후 [Could not load AWS or Google Cloud credentials](#could-not-load-aws-or-google-cloud-credentials)에 설명된 대로 오류를 보고하여 즉시 다시 인증할 수 있습니다. v2.1.228 이전에는 Claude Code가 실패한 Google Cloud 자격증명을 전체 재시도 예산을 통해 재시도한 후 오류를 표시했습니다.
* [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 스크립트가 자격증명을 제공하는 동안 Anthropic API에서 직접 또는 [LLM gateway](/docs/ko/llm-gateway)를 통해 `401` 또는 `403`. Claude Code는 스크립트를 다시 실행하고 전체 재시도 예산 내에서 새로운 출력으로 재시도합니다. 스크립트 자체가 재실행 시 실패하면 Claude Code는 [Your apiKeyHelper script is failing](#your-apikeyhelper-script-is-failing) 대신 표시합니다.

v2.1.227 이전에는 `Connection lost before a response was produced`가 `Connection closed while thinking, before producing a response`로 읽혔고 `The response stalled before a response was produced`가 `Response stalled while thinking, before producing a response`로 읽혔습니다.

Claude Code는 다음 오류를 재시도하지 않습니다:

* TLS 인증서 검증 실패, 예를 들어 TLS 검사 프록시, 누락된 `NODE_EXTRA_CA_CERTS` 번들 또는 만료된 인증서. Claude Code는 첫 번째 시도에서 오류를 보고하므로 인증서 설정을 즉시 수정할 수 있습니다. [SSL certificate errors](#ssl-certificate-errors)를 참조하세요. Claude Code는 여전히 핸드셰이크 시간 초과와 같은 일시적 TLS 조건을 재시도합니다. v2.1.199 이전에는 Claude Code가 인증서 실패를 전체 재시도 예산을 통해 재시도한 후 오류를 표시했습니다.
* Claude가 텍스트 블록이나 도구 호출을 완료했거나, 생각을 마친 후 시작했지만 응답을 마치기 전에 도착한 서버 오류, 끊어진 연결 또는 정체된 스트림. Claude Code는 요청을 다시 실행하지 않습니다. 같은 도구 호출을 두 번 실행할 수 있기 때문입니다. Claude가 완료한 것을 유지하고, Claude가 완료한 도구 호출을 실행하고, 그 결과에서 턴을 계속합니다. 대화형 세션과 비대화형 세션에서 보는 것에 대해서는 [The response above may be incomplete](#the-response-above-may-be-incomplete)를 읽으세요. v2.1.199 이전에는 Claude Code가 부분 출력을 버리고 서버 오류가 스트림 중간에 도착했을 때 전체 턴을 오류로 보고했습니다.
* Claude가 응답을 마친 후 도착한 오류: 재시도할 것이 없으므로 Claude Code는 완전한 응답을 유지하고 턴을 정상적으로 종료합니다.
* [Amazon Bedrock streaming response with an unexpected content-type](#bedrock-streaming-response-has-an-unexpected-content-type), 게이트웨이 또는 프록시가 응답을 다시 쓰면 재시도도 같은 방식으로 다시 쓸 것이기 때문입니다. Claude Code v2.1.208 이상이 필요합니다.
* 실패한 스트리밍 요청의 비스트리밍 재시도가 성공 상태를 받지만 [no Claude API message in the body](#api-returned-an-empty-or-malformed-response). Claude Code는 해당 오류로 턴을 종료합니다.
* 조직의 정책 검사가 거부한 요청, 이는 거부 메시지를 전달하는 `API Error:` 줄로 표시됩니다. 조직의 관리자는 Claude Enterprise 기능인 [Inference hooks](https://platform.claude.com/docs/en/manage-claude/inference-hooks)로 검사를 설정하고, 메시지는 구성한 지침으로 끝나거나 기본적으로 연락하도록 지시합니다. Claude Code는 거부가 모델이 아닌 요청의 내용에 관한 것이므로 거부된 요청을 동일한 모델이나 [fallback model](/docs/ko/model-config#fallback-model-chains)로 다시 보내지 않습니다. v2.1.239 이전에는 Claude Code가 거부된 요청을 스트리밍 없이 또는 구성된 폴백 모델에서 다시 보낼 수 있었고, 거부를 표시하기 전에 다시 보낼 수 있었습니다.

<h3 id="what-you-see-while-claude-code-retries-or-waits">
  Claude Code가 재시도하거나 대기하는 동안 보는 것
</h3>

재시도하는 동안 스피너는 오류 레이블 후에 `Retrying in Ns · attempt x/y` 카운트다운을 표시합니다. 레이블은 즉시 조치할 수 있는 오류의 첫 번째 시도에서 특정 이유를 명시합니다: 네트워크가 다운되었거나, TLS 핸드셰이크가 실패했거나, 속도 제한에 도달했습니다. 다른 오류의 경우 처음에는 `API error`로 읽힙니다. v2.1.198부터는 세 번째 시도에서 특정 이유로 전환되거나, `CLAUDE_CODE_MAX_RETRIES`가 3회 미만을 허용할 때 최종 시도에서 전환됩니다. 이전 버전은 최종 시도에서만 전환됩니다.

v2.1.198부터 일반적인 스피너 팁은 재시도 중에 억제됩니다. 오류 이유가 드러나면, 실패가 529 과부하인 경우 카운트다운 아래의 줄도 서비스 상태를 확인할 위치를 명시합니다: Anthropic API의 `status.claude.com` 또는 다른 구성의 메시지에 명시된 제공자 또는 게이트웨이 호스트.

요청이 여전히 보류 중인 동안 응답 스트림에 20초 동안 데이터가 도착하지 않으면 스피너는 재시도가 시작되기 전에 `Waiting for API response · will retry in … · check your network`를 표시합니다. 요청은 아직 실패하지 않았습니다: 카운트다운은 Claude Code가 정체된 연결을 중단하는 지점까지 실행됩니다. 중단 후 보는 것은 응답이 얼마나 진행되었는지에 따라 달라집니다:

* Claude가 텍스트 블록이나 도구 호출을 완료하기 전에, 또는 생각을 마친 후 시작하기 전에 Claude Code는 요청을 재시도하거나 오류로 턴을 종료합니다. [Automatic retries](#automatic-retries)는 어떤 정체를 재시도하고 몇 번 재시도하는지 말합니다.
* Claude가 텍스트 블록이나 도구 호출을 완료한 후, 또는 생각을 마친 후 시작했지만 Claude가 응답을 마치기 전에 Claude Code는 Claude가 완료한 것을 유지하고, Claude가 완료한 도구 호출에서 턴을 계속하고, [The response above may be incomplete](#the-response-above-may-be-incomplete)를 표시합니다. 비대화형 세션에서, 그리고 모든 세션에서 서브에이전트의 응답에 대해 Claude Code는 먼저 Claude에 응답을 계속하도록 프롬프트할 수 있습니다. 해당 항목은 언제 수행하는지, 언제 여전히 거기에 공지를 보는지를 말합니다.
* Claude가 응답을 마친 후 Claude Code는 턴을 정상적으로 종료합니다.

배너는 데이터가 재개되거나 재시도가 성공하면 자동으로 지워집니다. 모든 시도에서 다시 나타나면 [network issue](#unable-to-connect-to-api)로 취급하세요. v2.1.185 이전에는 배너가 10초 후에 다른 표현으로 나타났습니다.

Claude가 [advisor](/docs/ko/advisor)를 참조하는 동안 배너는 20초 대신 90초 후에 데이터 없이 나타납니다. 긴 advisor 검토가 20초 이상 아무것도 보낼 수 없기 때문입니다. v2.1.214 이전에는 20초 임계값이 advisor 호출 중에도 적용되었으므로 배너가 아무것도 잘못되지 않았을 때도 advisor 검토 중에 나타났습니다.

<h3 id="tune-retry-behavior">
  재시도 동작 조정
</h3>

다음 환경 변수로 재시도 동작을 조정할 수 있습니다:

| 변수                                                    | 기본값    | 효과                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :---------------------------------------------------- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/ko/env-vars)             | 10     | 재시도 시도 횟수. v2.1.186부터 15로 제한됩니다. v2.1.199부터 `CLAUDE_CODE_RETRY_WATCHDOG`는 기본값을 높이고 제한을 제거합니다. 스크립트에서 오류를 더 빨리 표시하려면 낮추세요.                                                                                                                                                                                                                                                                                                                                                                                                   |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/ko/env-vars)          | 설정 안 됨 | CI 작업과 같은 무인 세션에서 `1`로 설정하여 `CLAUDE_CODE_MAX_RETRIES` 시도 후 실패하는 대신 `429` 및 `529` 용량 오류를 무한정 재시도합니다. Claude Code는 표준 속도 요청이 지출 한도를 보고하거나 사용된 사용 크레딧을 보고하는 `429`를 받으면 즉시 실패합니다. 일정에 따라 재설정되는 [gateway spend cap](#spend-limit-reached)의 경우도 마찬가지입니다. v2.1.239 이전에는 watchdog이 이를 무한정 재시도했습니다. 빠른 모드 요청의 경우 [Handle rate limits](/docs/ko/fast-mode#handle-rate-limits)를 참조하세요. v2.1.199 이상에서는 서버 오류, 시간 초과 및 끊어진 연결과 같은 다른 일시적 오류에 대한 기본 재시도 횟수를 300으로 높입니다. 대략 3시간의 백오프이며, 변수를 명시적으로 설정하면 `CLAUDE_CODE_MAX_RETRIES`의 15 제한을 제거합니다. |
| [`API_TIMEOUT_MS`](/docs/ko/env-vars)                      | 600000 | 요청당 시간 초과(밀리초). 느린 네트워크 또는 프록시의 경우 높이세요. 또한 Claude Code가 응답 헤더를 기다리는 시간을 제한합니다. [No response from API](#no-response-from-api)에 설명되어 있습니다.                                                                                                                                                                                                                                                                                                                                                                                   |
| [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/ko/env-vars) | 설정 안 됨 | 스트리밍 요청의 첫 응답 바이트에 대한 기한(밀리초). Claude Code v2.1.242 이상이 필요합니다. 이것이 설정되지 않았을 때 Claude Code가 기한을 선택하는 방법에 대해서는 [No response from API](#no-response-from-api)를 참조하세요.                                                                                                                                                                                                                                                                                                                                                          |

<h2 id="server-errors">
  서버 오류
</h2>

이러한 오류의 대부분은 추론 제공자에서 발생합니다: Anthropic API의 Anthropic 서비스, Amazon Bedrock의 해당 제공자 엔드포인트 뒤의 서비스, Google Cloud의 Agent Platform, Microsoft Foundry 또는 사용자 정의 게이트웨이입니다. [자동 모드가 작업의 안전성을 결정할 수 없음](#auto-mode-cannot-determine-the-safety-of-an-action) 및 [API 오류로 인해 에이전트가 조기에 종료됨](#agent-terminated-early-due-to-an-api-error)은 또한 사용자 측의 원인을 다룹니다. 예를 들어 분류자 모델을 호출할 수 없는 Amazon Bedrock 계정이나 사용 한도에 도달한 하위 에이전트입니다.

<h3 id="api-error-500-internal-server-error">
  API 오류: 500 내부 서버 오류
</h3>

Claude Code는 모든 5xx 응답에 대해 상태 코드와 API의 오류 메시지를 표시합니다. 아래 예제는 Anthropic API의 500 응답을 보여줍니다:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

뒤따르는 문장은 서비스 상태를 확인할 위치를 명시하며 제공자에 따라 다릅니다. Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry 구성은 해당 제공자의 서비스 상태를 명시합니다. 사용자 정의 `ANTHROPIC_BASE_URL`은 게이트웨이 호스트를 명시합니다.

이는 API 내부의 예상치 못한 실패를 나타냅니다. 이는 사용자의 프롬프트, 설정 또는 계정으로 인해 발생하지 않습니다.

**수행할 작업:**

* [status.claude.com](https://status.claude.com) 또는 메시지에 명시된 제공자 상태 페이지에서 활성 인시던트를 확인합니다
* 1분 기다린 후 메시지를 다시 보냅니다. 원본 메시지는 여전히 대화에 있으므로 긴 프롬프트의 경우 전체를 다시 붙여넣는 대신 `try again`을 입력할 수 있습니다.
* 오류가 게시된 인시던트 없이 계속되면 `/feedback`을 실행하여 Anthropic이 요청 세부 정보로 조사할 수 있도록 합니다. 환경에서 `/feedback`을 사용할 수 없는 경우 [오류 보고](#report-an-error)를 참조하세요.

<h3 id="api-error-repeated-529-overloaded-errors">
  API 오류: 반복된 529 오버로드 오류
</h3>

API가 모든 사용자에게 일시적으로 용량이 부족합니다. Claude Code는 이 메시지를 표시하기 전에 이미 여러 번 재시도했습니다:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

뒤따르는 문장은 위의 500 오류와 동일한 방식으로 제공자에 따라 다릅니다.

529는 사용 한도가 아니며 할당량에 대해 계산되지 않습니다.

**수행할 작업:**

* [status.claude.com](https://status.claude.com) 또는 메시지에 명시된 제공자 상태 페이지에서 용량 공지를 확인합니다
* 몇 분 후에 다시 시도합니다
* `/model`을 실행하고 다른 모델로 전환하여 계속 작업합니다. 용량은 모델별로 추적되기 때문입니다. Claude Code는 한 모델이 특히 높은 부하를 받을 때 이를 수행하도록 프롬프트합니다. 예를 들어 `Opus is experiencing high load, please use /model to switch to Sonnet`입니다.

<h3 id="request-timed-out">
  요청 시간 초과
</h3>

API가 연결 마감 시간 전에 응답하지 않았습니다.

```text theme={null}
Request timed out
```

이는 높은 부하 기간 동안 또는 모델이 매우 큰 응답을 생성할 때 발생할 수 있습니다. 기본 요청 시간 초과는 10분입니다.

**수행할 작업:**

* 요청을 다시 시도합니다
* 장기 실행 작업의 경우 작업을 더 작은 프롬프트로 나눕니다
* 느린 네트워크 또는 프록시가 원인인 경우 [자동 재시도](#automatic-retries)에 설명된 대로 `API_TIMEOUT_MS`를 높입니다
* 시간 초과가 빈번하고 네트워크가 정상인 경우 아래의 [네트워크 및 연결 오류](#network-and-connection-errors)를 참조하세요

<h3 id="no-response-from-api">
  API에서 응답 없음
</h3>

Claude Code가 스트리밍 요청을 보냈고 API가 첫 바이트의 마감 시간 내에 응답 헤더를 반환하지 않아 Claude Code가 전체 `API_TIMEOUT_MS` 요청 시간 초과(기본값 10분)를 기다리는 대신 요청을 중단했습니다. Claude Code는 [재시도 예산](#tune-retry-behavior)이 허용하는 경우 최대 한 번 요청을 다시 보냅니다. 재시도도 응답이 없을 때 턴이 이 메시지로 끝나며, 각 시도가 대기한 시간을 표시합니다. [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/ko/env-vars)을 설정하면 일회 재시도 상한이 적용되지 않으며 Claude Code는 [재시도 동작 조정](#tune-retry-behavior)에 설명된 예산 내에서 재시도합니다.

```text theme={null}
API Error: No response from API (waited 3m, then 10m on the retry). If a proxy or gateway on your network holds responses until they complete, raise API_TIMEOUT_MS or CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS to wait longer.
```

Claude Code는 첫 시도의 응답 헤더 대기와 재시도의 대기를 별도로 설정합니다:

* **첫 시도**: 1 이상으로 설정할 때 [`CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`](/docs/ko/env-vars), 10초에서 30분 사이로 제한됩니다. 그렇지 않으면 Claude Code는 [스트리밍 유휴 감시자](/docs/ko/network-config#streaming-idle-watchdogs)에 나열된 바이트 수준 감시자 시간 초과를 사용하므로 해당 시간 초과를 변경하는 변수가 이 대기도 변경합니다. 어느 쪽이든 Claude Code는 요청 본문의 32KB마다 1초를 추가합니다.
* **재시도**: `API_TIMEOUT_MS`보다 1초 적게, 기본값으로 거의 10분이므로 재시도가 응답을 생성이 완료될 때까지 보유하는 프록시 또는 게이트웨이를 초과할 수 있습니다. Amazon Bedrock에서 재시도는 첫 시도와 동일한 마감 시간을 사용하며 메시지는 두 가지 대신 하나의 기간을 표시합니다.

어느 대기도 양수 `API_TIMEOUT_MS`보다 1초 적게 초과하지 않으며, 11초 미만의 양수 `API_TIMEOUT_MS`는 마감 시간을 끕니다. 바이트 수준 감시자는 응답 헤더가 도착한 후에만 시작되므로 그 후 바이트 전송을 중지하는 응답은 이 마감 시간 대신 [정지된 스트림 규칙](#automatic-retries)을 따릅니다.

**수행할 작업:**

* 메시지를 다시 보냅니다. 원본 메시지는 여전히 대화에 있으므로 긴 프롬프트의 경우 전체를 다시 붙여넣는 대신 `try again`을 입력할 수 있습니다.
* 반복되면 [네트워크 또는 프록시 문제](#unable-to-connect-to-api)로 취급합니다. 연결을 수락하고 요청을 절대 전달하지 않는 프록시는 모든 시도에서 이 오류를 생성합니다.
* 네트워크의 프록시 또는 게이트웨이가 응답을 생성이 완료될 때까지 보유하는 경우 `API_TIMEOUT_MS`를 높여 재시도가 더 오래 대기하도록 합니다. Amazon Bedrock에서 `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`도 높입니다.
* 첫 시도가 계속 시간 초과되고 재시도가 성공하면 `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`를 높여 첫 시도도 충분히 오래 대기하도록 합니다.

v2.1.242 이전에는 Claude Code가 응답 없는 스트리밍 요청이 실패하기 전에 전체 `API_TIMEOUT_MS` 요청 시간 초과(기본값 10분)를 기다렸습니다. v2.1.261 이전에는 재시도가 첫 시도와 동일한 마감 시간을 기다렸고 메시지는 기간을 표시하지 않았습니다.

<h3 id="the-response-above-may-be-incomplete">
  위의 응답이 불완전할 수 있음
</h3>

스트리밍 요청이 응답이 진행 중일 때 실패했습니다. Claude가 텍스트 블록 또는 도구 호출을 완료한 후 또는 생각을 마친 후 하나를 시작했습니다. 요청을 다시 보내면 동일한 도구 호출을 두 번 실행할 수 있으므로 Claude Code는 Claude가 완료한 출력을 유지하고 턴을 버리는 대신 이 공지를 추가합니다. 표시되는 변형은 원인을 명시합니다:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection lost mid-response. The response above may be incomplete.
API Error: Your computer went to sleep mid-response. The response above may be incomplete.
API Error: The response stopped arriving. The response above may be incomplete.
```

* `Server error mid-response`: 스트림 중간 오버로드 또는 5xx 서버 오류입니다. 이 변형은 Claude Code v2.1.199 이상이 필요합니다. 그 이전에는 부분 출력을 버리고 전체 턴을 오류로 보고했습니다.
* `Connection lost mid-response`: 연결이 끊어졌습니다.
* `Your computer went to sleep mid-response`: Claude Code가 응답이 스트리밍되는 동안 컴퓨터가 절전 모드로 전환되었음을 감지했습니다. 컴퓨터가 깨어나면 Claude Code는 연결을 끊어진 것으로 취급하고 읽기를 중지합니다.
* `The response stopped arriving`: 연결은 열려 있었지만 데이터 전달을 중지했으므로 스트리밍 유휴 감시자가 중단했습니다. v2.1.222 이전에는 Claude Code가 `ANTHROPIC_BASE_URL` 또는 `ANTHROPIC_AWS_BASE_URL`을 통해 도달한 [게이트웨이](/docs/ko/gateways) 연결에서 서버의 킵얼라이브 핑이 여전히 도착하는 동안 이 실패를 보고할 수 있었습니다. 파싱된 응답 이벤트만 계산했기 때문입니다. 업그레이드하면 해당 경로에서 이러한 거짓 시간 초과를 중지합니다. `ANTHROPIC_BEDROCK_BASE_URL`과 같은 제공자 기본 URL을 통해 도달한 게이트웨이는 바이트 감시자로 래핑되지 않습니다. [스트리밍 유휴 감시자](/docs/ko/network-config#streaming-idle-watchdogs)를 참조하세요.

v2.1.227 이전에는 `Connection lost mid-response`가 `Connection closed mid-response`로 읽혔고 `The response stopped arriving`이 `Response stalled mid-stream`으로 읽혔습니다.

4가지 경우에 Claude Code는 이 공지를 즉시 표시하지 않고 실패를 처리합니다:

* 응답의 앞부분에서 Claude Code는 실패를 재시도하거나 다른 오류로 턴을 종료합니다. [자동 재시도](#automatic-retries)를 참조하세요.
* 이러한 실패 중 하나가 Claude가 응답을 마친 후에 도착하면 Claude Code는 완전한 응답을 유지하고 이 공지 없이 턴을 정상적으로 종료합니다. v2.1.222 이전에는 Claude Code가 응답이 완료된 후 연결이 끊어지거나 정지되었을 때 이 공지를 표시했고 응답이 완전했음에도 불구하고 턴을 오류로 보고했습니다.
* [비대화형 세션](/docs/ko/headless)(예: `-p` 실행, [Agent SDK](/docs/ko/agent-sdk/overview) 실행 또는 [클라우드 세션](/docs/ko/claude-code-on-the-web))에서 잘린 응답이 주 대화에 있고 텍스트를 포함하지만 도구 호출이 없는 경우 `continue`를 직접 보낼 필요가 없습니다: Claude Code는 부분 출력을 유지하고 Claude에게 중단된 위치에서 계속하도록 프롬프트합니다. 최대 3번 연속으로. 이 공지는 Claude Code가 해당 연속을 모두 사용한 후에만 이러한 응답에 대해 표시됩니다. v2.1.246 이전에는 Claude Code가 비대화형 턴을 첫 번째 잘림에서 이 공지로 종료했습니다.
* [하위 에이전트](/docs/ko/sub-agents#api-errors-in-subagents)에서, 세션이 대화형인지 여부와 관계없이: 잘린 응답이 텍스트를 포함하지만 도구 호출이 없을 때 Claude Code는 하위 에이전트에게 계속하도록 프롬프트합니다. 공지는 해당 연속이 사용될 때까지만 하위 에이전트의 마지막 메시지가 됩니다. v2.1.257 이전에는 하위 에이전트가 첫 번째 잘림에서 이 공지를 표시했습니다.

**수행할 작업:**

* 대화형 세션에서 화면에 남아 있는 응답을 읽습니다: Claude Code는 오류 전에 Claude가 완료한 모든 블록을 유지하지만 턴이 끝날 때 중단된 최종 블록을 버립니다. 따라서 최종 문장 또는 도구 호출이 누락될 수 있습니다. `continue`로 회신하여 Claude가 마지막으로 완료한 블록에서 계속하도록 합니다.
* [비대화형 모드](/docs/ko/headless)(`-p`):
  * 기본 텍스트 출력을 사용하면 Claude Code는 턴의 앞부분에서 여전히 보유한 마지막 완료된 텍스트 블록을 인쇄한 후 이 메시지를 인쇄합니다. 보유하지 않으면 Claude Code는 이 메시지만 인쇄합니다. 예를 들어 Claude Code가 턴 중간에 대화를 압축하고 해당 텍스트를 지웠기 때문입니다. v2.1.219 이전에는 Claude Code가 `-p` 텍스트 출력에서만 이 메시지를 인쇄했고 이미 생성한 응답을 버렸습니다.
  * `--output-format json` 또는 `stream-json`을 사용하면 Claude Code는 이 메시지를 `result` 필드에 보고합니다.
  * 연결이 안정적이면 턴을 계속하려면 세션을 재개하고 [대화 계속](/docs/ko/headless#continue-conversations)에 설명된 대로 `continue`를 보냅니다.

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  자동 모드가 작업의 안전성을 결정할 수 없음
</h3>

[자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)가 작업을 분류하는 데 사용하는 모델이 결정을 내릴 수 없어 자동 모드가 작업을 자동으로 승인하지 않았습니다. 표시되는 메시지는 분류자가 실패한 방식에 따라 다릅니다.

작업 디렉토리 내의 읽기, 검색 및 편집은 분류자를 건너뛰므로 이러한 모든 경우에 계속 작동합니다.

분류자 모델을 사용할 수 없을 때:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait a moment and then try this action again.
```

Claude Code가 실패 범주를 결정할 수 있을 때 `temporarily unavailable` 뒤의 괄호에 범주를 명시합니다. 예를 들어 `<model> is temporarily unavailable (rate-limited), so auto mode cannot determine the safety of <tool> right now`입니다. 범주는 `(rate-limited)`, `(overloaded)`, `(server error)`, `(timed out)` 및 `(connection failed)`입니다. 속도 제한, 오버로드 및 서버 오류는 일시적이며 재시도가 작동합니다. `(timed out)` 또는 `(connection failed)`가 반복되면 연결을 확인하세요. [API에 연결할 수 없음](#unable-to-connect-to-api)을 참조하세요. v2.1.229 이전에는 메시지가 범주를 명시하지 않았고 `Wait briefly and then try this action again`으로 읽혔습니다.

범주가 맞지 않으면 메시지는 괄호에 범주 없이 나타납니다. 둘 이상의 실패가 해당 형식을 생성합니다. [Amazon Bedrock](/docs/ko/amazon-bedrock)에서, [Mantle 엔드포인트](/docs/ko/amazon-bedrock#use-the-mantle-endpoint) 포함, AWS 계정이 메시지에 명시된 모델을 호출할 수 없을 때도 나타나며, 계정에 모델에 대한 액세스 권한이 부여될 때까지 모든 재시도에서 해당 실패가 반복됩니다.

**수행할 작업:**

* 몇 초 후 재시도합니다. Claude는 동일한 메시지를 보고 일반적으로 자동으로 재시도합니다. 일시적 실패는 [자동 모드 적격성](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)과 무관합니다. 설정을 변경할 필요가 없습니다
* 재시도가 계속 실패하면 읽기 전용 작업을 계속하고 나중에 차단된 작업으로 돌아옵니다
* Amazon Bedrock에서 메시지가 모든 재시도에서 반환되면 계정이 명시된 모델을 호출할 수 있는지 확인합니다: 표준 Amazon Bedrock 모델의 경우 [IAM 정책](/docs/ko/amazon-bedrock#iam-configuration)이 호출을 허용하는지 확인합니다. Mantle 모델 ID의 경우 [AWS 계정 팀에 문의](/docs/ko/amazon-bedrock#mantle-endpoint-errors)합니다

분류자 요청이 OAuth 토큰이 만료되었거나 다른 세션에서 회전되었기 때문에 실패할 때 Claude Code는 토큰을 새로 고치고 요청을 한 번 재시도하므로 일상적인 토큰 만료는 이 메시지로 표시되지 않습니다. v2.1.216 이전에는 만료되었거나 회전된 토큰이 각 분류자 요청을 실패했고 토큰이 새로 고쳐질 때까지 자동 모드가 확인된 모든 작업을 거부했습니다.

분류자가 파싱할 수 없는 응답을 반환했을 때:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**수행할 작업:**

* 작업을 재시도합니다. 이는 일반적으로 다음 시도에서 성공합니다
* `claude --debug`를 실행하고 작업을 반복하여 디버그 로그에서 기본 분류자 응답을 확인합니다

별도의 API 안전 검사가 이전 대화 내용으로 인해 분류자 요청을 차단했을 때:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

Claude Code는 작업을 거부하지만 Claude에게 이것이 작업이 안전하지 않다는 판단이 아니며 재시도하는 대신 다른 작업을 계속하도록 알립니다. 이러한 거부는 [자동 모드의 일시 중지 임계값](/docs/ko/permission-modes#when-auto-mode-falls-back)에 대해 계산되지 않습니다. [비대화형](/docs/ko/headless) `-p` 실행에서 Claude Code는 실행을 중지하지 않습니다. Claude가 수신하는 내용은 작업을 요청한 위치에 따라 다릅니다:

* `-p` 실행 중 `--input-format stream-json` 없이 [백그라운드 하위 에이전트](/docs/ko/sub-agents#run-subagents-in-foreground-or-background)에 Claude Code는 `Agent aborted: auto mode classifier request refused by the safety safeguard in headless mode`를 포함하는 오류 결과를 반환합니다
* 대화형 세션 및 `-p` 실행의 주 대화를 포함한 다른 모든 곳에서 Claude Code는 해당 거부를 Claude에게 반환합니다

v2.1.225 이전에는 Claude Code가 이러한 거부를 일시 중지 임계값에 대해 계산했고 진정한 분류자 블록과 동일한 거부 메시지를 반환했습니다.

**수행할 작업:**

* 이것은 작업에 대한 결정이 아닙니다. 대화에 이미 있는 내용이 Claude Code가 대화를 분류자에게 보낼 때 API의 안전 필터를 트리거했습니다
* 재시도는 도움이 되지 않습니다. 동일한 대화 내용이 필터를 다시 트리거합니다
* 대화형 세션에서 다른 [권한 모드](/docs/ko/permission-modes)로 전환하여 프롬프트될 때 작업을 승인할 수 있습니다
* 트리거 내용 없이 새 대화를 시작합니다

대화가 분류자의 컨텍스트 윈도우보다 커졌을 때:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

작업에 발생하는 일은 Claude가 요청한 위치에 따라 다릅니다:

* 대화형 세션에서 자동 모드는 해당 작업에 대해 일반 권한 프롬프트로 폴백하므로 수동으로 승인하거나 거부할 수 있습니다
* [비대화형](/docs/ko/headless) `-p` 실행 중 `--input-format stream-json` 없이 [백그라운드 하위 에이전트](/docs/ko/sub-agents#run-subagents-in-foreground-or-background)에 Claude Code는 `Agent aborted: auto mode classifier transcript exceeded context window in headless mode`를 포함하는 오류 결과를 반환하고 실행을 계속합니다
* [`--permission-prompt-tool`](/docs/ko/cli-reference#cli-flags) 없이 `-p` 실행의 다른 곳에서 폴백할 프롬프트가 없으므로 작업이 실행되지 않고 실행이 계속됩니다

**수행할 작업:**

* 대화형 세션에서 나타나는 프롬프트에서 작업을 승인하거나 거부합니다
* 대화형 세션에서 `/compact`를 실행하여 대화 크기를 줄여 후속 작업이 분류자 윈도우에 맞도록 합니다

<h3 id="the-server-returned-no-safety-verdict">
  서버가 안전 판정을 반환하지 않음
</h3>

[서버 측 분류자 검토](/docs/ko/permission-modes#server-side-classifier-review) 하에서 자동 모드는 서버가 판정을 제공하지 않을 때 작업을 거부합니다. 거부는 Claude Code가 하나를 결정할 수 있을 때 괄호에 범주를 명시합니다. 예를 들어 `(timed out)`:

```text theme={null}
The server-side auto mode classifier gave no verdict (timed out), so auto mode cannot determine the safety of <tool>.
```

메시지의 나머지 부분은 Claude에게 한 번의 재시도가 도움이 될 수 있는지 알려줍니다. 이러한 거부 중 일부 전에 Claude Code는 대기하므로 Claude의 다음 시도가 즉시 따르지 않습니다. 대화형 세션에서 대기 중에 스피너는 `Auto mode check unavailable`을 카운트다운과 함께 표시하며, `Esc`를 누르면 턴이 중단됩니다.

10개 응답이 연속으로 판정이 없은 후 자동 모드는 턴을 중지합니다:

```text theme={null}
Auto mode is unavailable — the server returned no safety verdict for the last 10 responses, so Claude stopped. Send a message to try again, or switch out of auto mode.
```

중지 메시지는 각 세션 종류에서 다른 위치에 나타납니다:

* 대화형 세션에서 메시지는 대화 기록에 경고로 나타나고 턴이 끝납니다
* [비대화형](/docs/ko/headless) `-p` 실행에서 실행이 끝나고 실행 오류를 보고합니다. 기본 텍스트 출력을 사용하면 메시지가 stderr에 인쇄됩니다.
* [하위 에이전트](/docs/ko/sub-agents)가 한도에 도달했을 때 하위 에이전트는 완료 전에 중지되고 Claude는 자동 모드가 중지했다는 메모와 함께 생성한 것을 받습니다

**수행할 작업:**

* 다른 메시지를 보내 Claude가 다시 시도하도록 합니다. 응답 수 계산이 다시 시작됩니다.
* 중지가 반복되고 요청이 [LLM 게이트웨이 또는 프록시](/docs/ko/llm-gateway)를 통과하면 스트리밍 응답을 자르거나 다시 쓰는지 확인합니다. [서버 측 분류자 검토](/docs/ko/permission-modes#server-side-classifier-review)는 어떤 게이트웨이 동작이 거부를 유발하는지 말하며, [게이트웨이 호환성 가이드](/docs/ko/llm-gateway-protocol#feature-pass-through)는 변경되지 않은 상태로 통과할 내용을 나열합니다.
* Claude Code를 시작하기 전에 `CLAUDE_CODE_AUTO_MODE_SERVER=0`을 설정하여 대신 자체 분류자 요청을 사용합니다. v2.1.281 이전에는 Claude Code가 Anthropic API에 대한 직접 연결에서 변수를 읽지 않았습니다.
* 대신 작업을 직접 승인하려면 [자동 모드를 전환](/docs/ko/permission-modes#switch-permission-modes)합니다

v2.1.280 이전에는 Claude Code가 판정이 없는 응답의 각 작업을 즉시 거부했고 턴을 중지하지 않았습니다.

<h3 id="agent-terminated-early-due-to-an-api-error">
  API 오류로 인해 에이전트가 조기에 종료됨
</h3>

[하위 에이전트](/docs/ko/sub-agents)의 API 요청이 사용 한도에 도달했거나 서버 오류에 대한 재시도가 소진되었기 때문에 터미널로 실패했으므로 하위 에이전트가 작업을 마치기 전에 중지했습니다. 이 메시지는 Claude Code v2.1.199 이상이 필요합니다. 그 이전에는 API 오류 텍스트가 하위 에이전트의 결과인 것처럼 Claude에게 반환되었습니다.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**수행할 작업:**

* 콜론 뒤의 오류 세부 정보를 이 페이지의 자체 섹션(예: [사용 한도](#usage-limits) 또는 [서버 오류](#server-errors))과 일치시키고 해당 섹션의 단계를 따릅니다
* 기본 오류가 해결되면 Claude에게 작업을 재시도하거나 [하위 에이전트를 재개](/docs/ko/sub-agents#resume-subagents)하도록 요청합니다

속도 제한, 오버로드 또는 서버 오류가 이미 텍스트 출력을 생성한 포그라운드 하위 에이전트를 중단할 때 Claude는 이 오류 대신 불완전으로 표시된 부분 출력을 수신합니다. 유일한 출력이 도구 호출인 하위 에이전트도 이 오류를 받습니다. v2.1.199에서는 해당 형태가 빈 부분 결과를 대신 반환했습니다. [하위 에이전트의 API 오류](/docs/ko/sub-agents#api-errors-in-subagents)를 참조하세요.

<h2 id="usage-limits">
  사용 한도
</h2>

이 섹션의 대부분의 오류는 계정 또는 플랜에 연결된 할당량에 도달했음을 의미합니다. 세 가지는 다르게 작동합니다: [`Server is temporarily limiting requests`](#server-is-temporarily-limiting-requests)는 플랜 할당량과 무관한 서버 측 스로틀이고, [`Usage credits required for 1M context`](#usage-credits-required-for-1m-context)는 소진된 할당량이 아닌 자격 확인이며, [`The prompt to confirm went unanswered`](#the-prompt-to-confirm-went-unanswered)는 할당량 도달 여부와 관계없이 사용 크레딧 동의 프롬프트가 응답 없이 종료되었음을 의미합니다.

<h3 id="youve-hit-your-session-limit">
  세션 한도에 도달했습니다
</h3>

구독 플랜에는 롤링 사용 허용량이 포함됩니다. 허용량이 소진되면 다음 메시지 중 하나가 표시됩니다:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
You've hit your Sonnet limit · resets 3:45pm
```

Claude Code는 메시지에 표시된 재설정 시간까지 추가 요청을 차단합니다. 세션 및 주간 한도는 모든 모델에서 공유되므로 모델을 전환해도 액세스가 복구되지 않습니다. Opus 및 Sonnet 한도는 각각 해당 모델 제품군에 대한 요청에만 적용되므로 `/model`을 사용하여 제품군 외부의 모델로 전환하면 계속 작업할 수 있습니다.

claude.ai 구독으로 로그인한 대화형 세션에서 Claude Code는 열린 세션에서 대기할 수 있으며 재설정 직후 중단된 작업을 계속할 수 있습니다. 대기 중에 세션 하단의 줄에 `Usage limit reached · continuing automatically at 3:45pm · esc to cancel`이 표시됩니다. 빈 프롬프트에서 `Esc`를 눌러 대기를 취소합니다. 표시되는 내용, 대기를 시작하거나 취소하는 방법, 자동 계속을 끄는 방법은 [사용 한도 재설정 대기](/docs/ko/interactive-mode#wait-for-a-usage-limit-to-reset)를 참조하십시오. v2.1.234 이전에는 Claude Code가 이 대기 기능을 제공하지 않았습니다.

사용량은 세션 및 주간 허용량에 동시에 계산됩니다. 대규모 워크플로우 팬아웃과 같은 대량의 활동이 한 번에 발생하면 세션 창이 재설정되기 전에 주간 허용량이 소진될 수 있습니다.

**수행할 작업:**

* 오류에 표시된 재설정 시간까지 대기합니다
* [Desktop app](/docs/ko/desktop)의 Code 탭에서 세션 한도 카드는 **Auto-continue when limits reset** 체크박스를 제공합니다. 주간 한도 카드는 제공하지 않습니다. 체크되어 있으면 Desktop app은 재설정 후 중단된 턴을 다시 시도하고 카드에 재시도 시간을 표시합니다. Desktop 체크박스와 CLI의 `/config`의 **Continue automatically at usage limit** 설정은 별개이므로 각각 개별적으로 끕니다.
* Opus 또는 Sonnet 한도의 경우 `/model`을 실행하고 해당 제품군 외부의 모델로 전환하여 계속 작업합니다. 각 모델에는 자체 프롬프트 캐시가 있으므로 다음 요청은 캐시 히트 없이 전체 대화를 다시 읽습니다. [모델 전환](/docs/ko/prompt-caching#switching-models)을 참조하십시오
* `/usage`를 실행하여 플랜 한도 및 재설정 시간을 확인합니다
* `/usage-credits`를 실행하여 Pro 및 Max에서 추가 사용량을 구매하거나 Team 및 Enterprise에서 관리자에게 요청합니다. 이것이 청구되는 방식은 [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)을 참조하십시오.
* 더 높은 기본 한도를 위해 플랜을 업그레이드하려면 [claude.com/pricing](https://claude.com/pricing)을 참조하십시오

한도에 도달하기 전에 Claude Code는 남은 허용량을 대부분 사용했다는 경고를 표시할 수 있으며, `You've used 85% of your session limit · resets 3:45pm`과 같은 메시지가 표시됩니다. 남은 허용량을 지속적으로 모니터링하려면 [custom status line](/docs/ko/statusline#rate-limit-usage)에 `rate_limits` 필드를 추가하거나 Desktop app에서 모델 선택기 옆의 [usage ring](/docs/ko/desktop#check-usage)을 클릭합니다.

<h3 id="usage-credits-required-for-1m-context">
  1M 컨텍스트에 필요한 사용 크레딧
</h3>

선택한 모델은 1M 토큰 확장 컨텍스트 윈도우를 사용하며 플랜에는 사용 크레딧을 통해서만 포함됩니다.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on (they take effect after you restart Claude Code), or /model to switch to standard context
```

이는 할당량 소진이 아닌 자격 확인입니다. 세션 및 주간 허용량에 용량이 남아 있어도 발생합니다. 1M 컨텍스트를 직접 포함하는 플랜과 사용 크레딧이 필요한 플랜은 [확장 컨텍스트](/docs/ko/model-config#extended-context)를 참조하십시오. Claude Code는 `/model`로 모델을 선택할 때 이 확인을 실행하며 Anthropic API에 대한 직접 연결에서만 실행됩니다. `ANTHROPIC_BASE_URL`을 [LLM gateway](/docs/ko/llm-gateway)로 지정하면 `/model`은 `[1m]` 선택을 허용하고 게이트웨이가 요청 성공 여부를 결정합니다.

이 오류가 컨텍스트가 200K 토큰을 초과하여 대화 중간에 나타나면 Claude Code는 자동으로 대화를 표준 컨텍스트 한도 아래로 압축하고 이후 세션을 해당 한도로 유지하므로 조치가 필요하지 않습니다. v2.1.172 이전 버전에서는 `/compact`를 포함한 모든 후속 요청에서 오류가 반복되었습니다. 해당 버전에서 복구하려면 `/clear`를 실행합니다. 아래 단계는 명시적으로 `[1m]` 모델을 선택한 경우에 적용됩니다.

**수행할 작업:**

* `/model`을 실행하고 `[1m]` 접미사 없는 변형을 선택하여 표준 컨텍스트 윈도우로 폴백합니다
* 메시지가 `/usage-credits`를 지정하는 경우 이를 실행하여 Pro 및 Max에서 1M 변형에 대한 측정 청구를 켜거나 Team 및 Enterprise에서 관리자에게 사용 크레딧을 요청합니다. 사용 크레딧이 켜진 후 Claude Code를 다시 시작하거나 새 세션을 시작합니다(메시지가 지정하는 대로). 그때까지 세션은 표준 컨텍스트 한도로 유지됩니다.
* `/model` 후에도 오류가 지속되면 1M 모델 ID가 다른 곳에 설정되어 있을 수 있습니다. 우선순위 순서로 확인할 구성 위치는 [모델 설정](/docs/ko/model-config#setting-your-model)을 참조하십시오.
* 모델 선택기에서 1M 변형을 완전히 제거하려면 [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/ko/env-vars)을 설정합니다

v2.1.268 이전에는 메시지가 `run /usage-credits to turn them on, or /model to switch to standard context`로 끝났으며 다시 시작을 언급하지 않았습니다.

<h3 id="the-prompt-to-confirm-went-unanswered">
  확인 프롬프트에 응답이 없습니다
</h3>

계정에 [Fable 사용 크레딧 동의](/docs/ko/model-config#fable-and-usage-credits)가 필요한 경우 Claude Code는 Fable 요청이 사용 크레딧을 청구하기 전에 확인하도록 요청합니다. 터미널이 없을 수 있는 세션에서 아무도 해당 동의 프롬프트에 응답하지 않으면 Claude Code는 프롬프트를 닫고 다음 메시지 중 하나로 턴을 종료합니다:

```text theme={null}
Fable limit reached · continuing on Fable 5.1 uses usage credits, and the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
Fable 5.1 now uses usage credits · the prompt to confirm went unanswered — nothing was sent · answer it where this session is running, or /model to change
```

메시지는 세션의 Fable 모델을 지정하므로 Fable 5에서는 `continuing on Fable 5`와 `Fable 5 now uses usage credits`로 읽습니다. v2.1.257 이전에는 첫 번째 메시지가 `Fable 5 limit reached`로 시작했습니다.

이는 [Remote Control](/docs/ko/remote-control) 세션, [background sessions](/docs/ko/agent-view), [agent team](/docs/ko/agent-teams) 팀원 세션에서 발생합니다. Claude Code는 세션의 자체 대화형 보기에서만 동의 프롬프트를 표시합니다: 실행되는 터미널 또는 백그라운드 세션의 경우 연결한 후 [agents view](/docs/ko/agent-view)입니다. Remote Control 클라이언트는 이를 표시할 수 없습니다. Claude Code는 [`dialogExpiry`](/docs/ko/settings-reference#dialogexpiry) 기한(기본값 5분) 또는 Remote Control 클라이언트에서 보낸 프롬프트와 같이 해당 터미널에서 아무도 입력하지 않은 상태에서 새 프롬프트가 도착하는 즉시 프롬프트를 닫습니다. 세션이 실행되는 터미널에서 입력하면 기한이 취소되고 Claude Code는 답변을 기다립니다. 백그라운드 세션의 연결된 보기에서 입력해도 기한이 취소되지 않으며 새 프롬프트가 동의 프롬프트를 여전히 닫으므로 둘 중 하나가 발생하기 전에 답변합니다. Claude Code는 아무것도 보내지 않고 모델을 유지하므로 다음 프롬프트를 보낼 때 Claude Code는 동의 프롬프트를 다시 표시합니다.

**수행할 작업:**

* 세션이 실행되는 터미널에서 다른 프롬프트를 보내고 다시 나타나면 동의 프롬프트에 답변합니다. 백그라운드 세션의 경우 먼저 [agents view](/docs/ko/agent-view)에서 연결합니다. Remote Control 클라이언트에서 다시 보내면 클라이언트가 프롬프트를 표시할 수 없으므로 이 메시지가 다시 나타납니다.
* `/model`을 실행하여 사용 크레딧을 청구하지 않는 모델로 전환합니다
* 해당 터미널에 도달할 시간을 더 주려면 [`dialogExpiry`](/docs/ko/settings-reference#dialogexpiry)를 더 긴 값 또는 `"never"`로 설정합니다

v2.1.236 이전에는 이 메시지가 나타나지 않았습니다: Remote Control 클라이언트가 연결되어 있는 동안 Claude Code는 답변을 60초 동안 기다린 후 기본 모델에서 턴을 계속했습니다.

<h3 id="server-is-temporarily-limiting-requests">
  서버가 일시적으로 요청을 제한하고 있습니다
</h3>

API가 플랜 할당량과 무관한 단기 스로틀을 적용했습니다.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code는 실제 한도 응답이 전달하는 통합 할당량 헤더의 부재로 이를 플랜 한도와 구분합니다. v2.1.199부터 이는 인증 방식에 관계없이 [자동으로 재시도](#automatic-retries)되며 백오프로 표시되기 전에 실행됩니다. 이전 버전에서는 claude.ai 구독으로 로그인한 세션이 첫 번째 발생에서 턴에 실패했습니다. API 키 및 Enterprise 로그인만 재시도했습니다.

**수행할 작업:**

* 잠시 기다렸다가 다시 시도합니다
* 지속되면 [status.claude.com](https://status.claude.com)을 확인합니다

<h3 id="request-rejected-429">
  요청 거부됨 (429)
</h3>

API 키, Amazon Bedrock 프로젝트 또는 Google Cloud 프로젝트에 대해 구성된 속도 제한에 도달했습니다.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

뒤따르는 문장은 서비스 상태를 확인할 위치를 지정하며 공급자에 따라 다릅니다. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 구성은 Anthropic 상태 페이지 대신 해당 공급자의 서비스 상태를 지정합니다. 사용자 정의 `ANTHROPIC_BASE_URL`은 게이트웨이 호스트를 지정합니다.

**수행할 작업:**

* `/status`를 실행하고 활성 자격 증명이 예상한 것인지 확인합니다. 환경의 잘못된 `ANTHROPIC_API_KEY`는 구독 대신 저가형 키를 통해 요청을 라우팅할 수 있습니다.
* 공급자 콘솔에서 활성 한도를 확인하고 필요한 경우 더 높은 계층을 요청합니다
* Anthropic API 키의 경우 계층이 작동하는 방식과 워크스페이스별 상한을 설정하는 방법은 [rate limits reference](https://platform.claude.com/docs/en/api/rate-limits)를 참조하십시오
* 동시성 감소: [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/ko/env-vars)를 낮추고, 많은 병렬 서브에이전트 실행을 피하거나, 대량 스크립트 실행을 위해 `/model`로 더 작은 모델로 전환합니다

<h3 id="youve-hit-your-monthly-spend-limit">
  월간 지출 한도에 도달했습니다
</h3>

플랜의 포함된 사용량이 이 요청을 충당할 수 없으며, 그렇지 않으면 이를 지불할 [사용 크레딧](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)이 지출 한도에 도달했습니다. 이는 플랜의 사용 창 중 하나가 소진되었거나 요청이 [사용 크레딧에 청구되는](/docs/ko/model-config#fable-and-usage-credits) 모델에 대한 요청과 같이 사용 크레딧만 지불하는 요청일 때 발생합니다. 메시지는 어느 한도가 사용자를 차단했는지 지정합니다. `·` 뒤의 텍스트는 해당 한도를 높이는 방법을 설명하며 플랜 및 청구 관리 여부에 따라 다릅니다:

```text theme={null}
You've hit your monthly spend limit · raise it at claude.ai/settings/usage
You've hit your individual spend limit · ask your admin for a higher limit
You've hit your org's monthly spend limit · visit claude.ai/admin-settings/usage to raise it
You've hit your team's shared budget · ask your admin to raise it at claude.ai/admin-settings/usage
You've hit your channel's monthly spend limit · an org owner or channel manager can raise it in the channel's Claude settings
```

`team's shared budget`는 관리자가 속한 그룹에 할당한 풀링된 예산입니다. 메시지는 그룹을 지정하지 않습니다. `channel's monthly spend limit`은 세션이 실행되는 Slack 채널의 예산이므로 조직은 여전히 그 외부에 예산이 있을 수 있습니다.

플랜의 창 중 하나가 소진된 경우 메시지는 해당 창이 재설정될 때를 나타내며, 예를 들어 `· your session limit resets 3:45pm`이고 아무도 한도를 높이지 않으면 액세스가 그때 반환됩니다. 사용량 기반 청구가 있는 조직에서 메시지는 `spend limit` 대신 `usage limit`을 나타내며, `You've hit your individual usage limit`과 같습니다.

v2.1.239 이전에는 메시지가 플랜 창의 재설정 시간을 지정하지 않았습니다. v2.1.268 이전에는 그룹의 풀링된 예산이 `individual spend limit` 메시지 대신 `team's shared budget`을 생성했습니다.

Claude apps 게이트웨이를 통해 연결하고 소문자 `spend limit reached`를 보면 그것은 게이트웨이 운영자의 상한입니다. [지출 한도 도달](#spend-limit-reached)을 참조하십시오.

**수행할 작업:**

* Pro 및 Max에서 claude.ai의 [**Settings > Usage**](https://claude.ai/settings/usage)에서 월간 지출 한도를 높이거나 `/usage-credits`를 실행합니다
* Team 및 Enterprise에서 청구를 관리하면 [**Admin settings > Usage**](https://claude.ai/admin-settings/usage)에서 한도를 높이거나 관리자에게 요청합니다. `/usage-credits`는 관리자에게 해당 요청을 보냅니다
* 채널의 한도의 경우 조직 소유자 또는 채널의 관리자에게 claude.ai에서 이를 높이도록 요청합니다. Claude Tag 문서의 [Per-channel limits](https://claude.com/docs/claude-tag/admins/set-spend-limit#per-channel-limits)를 참조하십시오
* 메시지가 플랜의 창에 대한 재설정 시간을 지정하면 대신 기다릴 수 있습니다
* `/usage`를 실행하여 플랜의 창과 각각 재설정될 때를 확인합니다

<h3 id="spend-limit-reached">
  지출 한도 도달
</h3>

[Claude apps gateway](/docs/ko/claude-apps-gateway)를 통해 연결하고 게이트웨이 운영자가 설정한 [spend cap](/docs/ko/claude-apps-gateway-spend-limits)을 초과했습니다. 게이트웨이는 명명된 기간이 재설정되거나 운영자가 상한을 높을 때까지 요청을 차단합니다. 각 차단된 `429` 응답을 `x-should-retry: false`로 표시하므로 Claude Code는 재시도 없이 이 메시지를 표시합니다.

```text theme={null}
spend limit reached (daily; resets 2026-08-09 00:00 UTC)
```

메시지는 상한의 기간과 재설정 시간을 지정하며 운영자가 `blocked_message`를 구성한 경우 해당 지침이 뒤따릅니다. v2.1.225 이전에는 메시지가 `spend limit reached`만 읽었습니다. 이전 버전의 게이트웨이는 여전히 더 짧은 형식을 보냅니다.

**수행할 작업:**

* 메시지가 지정하는 재설정 시간까지 기다리거나 메시지에 지침이 포함된 경우 운영자의 지침을 따릅니다
* 정기적으로 상한에 도달하면 게이트웨이 운영자에게 상한을 높이도록 요청합니다

관련 메시지인 `spend limit unavailable`은 게이트웨이가 지출 기록을 읽을 수 없었고 상한을 초과하기보다는 예방 조치로 요청을 차단했음을 의미합니다. 일반적으로 자동으로 해결됩니다. 지속되면 게이트웨이 운영자에게 알립니다.

<h3 id="credit-balance-is-too-low">
  크레딧 잔액이 너무 낮습니다
</h3>

Console 조직이 선불 크레딧을 소진했거나 Claude Code가 구독 대신 Console API 키로 요청을 보내고 있습니다.

```text theme={null}
Credit balance is too low
```

**수행할 작업:**

* Pro, Max, Team 또는 Enterprise 플랜이 있고 이것을 보면 `/status`를 실행하고 `API key` 행을 확인합니다. 환경의 승인된 `ANTHROPIC_API_KEY`는 구독 대신 해당 키를 통해 요청을 라우팅합니다. 현재 셸에서 설정을 해제하고 셸 프로필에서 제거한 후 `claude`를 다시 시작합니다. 아직 구독으로 로그인하지 않았으면 `/login`을 실행합니다.
* [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing)에서 크레딧을 추가하고 잔액이 0에 도달하기 전에 자동 재로드를 활성화하는 것을 고려합니다
* Console에서 워크스페이스별 지출 상한을 설정하여 단일 프로젝트가 조직 잔액을 소진하는 것을 방지합니다. [비용 효과적으로 관리](/docs/ko/costs)를 참조하십시오.

<h3 id="could-not-update-your-spend-limit">
  지출 한도를 업데이트할 수 없습니다
</h3>

서버가 지출 한도에 도달할 때 나타나는 프롬프트에서 수행한 지출 한도 변경을 거부했습니다.

```text theme={null}
Could not update your spend limit: <reason from the server>
```

서버가 거부를 설명할 때 메시지는 해당 이유로 끝나고 동일한 값을 재시도하면 다시 실패합니다. 연결 끊김과 같이 실패에 서버 제공 이유가 없으면 메시지는 `Could not update your spend limit. Press Enter to retry.`로 읽고 재시도하면 성공할 수 있습니다. v2.1.216 이전에는 Claude Code가 모든 실패에 대해 일반 형식을 표시했습니다.

**수행할 작업:**

* 메시지에 이유가 포함되면 더 낮은 금액과 같이 이를 만족하는 한도를 선택합니다
* 메시지가 일반 형식만 표시하면 재시도합니다. 실패가 일시적일 수 있습니다
* 변경이 계속 실패하면 브라우저의 [claude.ai billing settings](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)에서 대신 변경합니다

<h2 id="authentication-errors">
  인증 오류
</h2>

이러한 오류는 Claude Code가 API에 대해 사용자의 신원을 증명할 수 없음을 의미합니다. 언제든지 `/status`를 실행하여 현재 활성화된 자격증명을 확인하십시오.

<h3 id="not-logged-in">
  로그인하지 않음
</h3>

이 세션에 유효한 자격증명이 없습니다.

```text theme={null}
Not logged in · Please run /login
```

**수행할 작업:**

* `/login`을 실행하여 Claude 구독 또는 Console 계정으로 인증합니다.
* 환경 변수가 사용자를 인증할 것으로 예상했다면, `ANTHROPIC_API_KEY`가 `claude`를 실행한 셸에서 설정되고 내보내졌는지 확인합니다.
* CI 또는 자동화에서 대화형 로그인이 불가능한 경우, 시작 시 키를 가져오는 [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 스크립트를 구성합니다.
* [인증 우선순위](/docs/ko/authentication#authentication-precedence)를 참조하여 여러 자격증명이 있을 때 Claude Code가 어떤 자격증명을 사용하는지 이해합니다.

반복적으로 로그인하라는 메시지가 표시되면, 시스템 시계 확인 및 macOS 자격증명 저장소 복구 단계에 대해 [로그인하지 않음 또는 토큰 만료](/docs/ko/troubleshoot-install#not-logged-in-or-token-expired)를 참조하십시오.

<h3 id="could-not-resolve-authentication-method">
  인증 방법을 확인할 수 없음
</h3>

세션이 자격증명 없이 API 클라이언트에 도달했습니다. [백그라운드 세션](/docs/ko/agent-view) 및 클라우드 세션은 워커가 자격증명 없이 시작될 때 이 메시지를 표시합니다. 대화형, `-p` 및 Agent SDK 실행은 [로그인하지 않음](#not-logged-in)과 동일한 조건을 보고하며 이 문자열을 디버그 로그에만 기록하므로, 거기서 찾은 경우 대신 해당 항목을 따릅니다.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

현재 버전에서 오류는 워커 프로세스에 사용 가능한 자격증명이 없음을 의미합니다. v2.1.174 이전에는 유휴 사전 초기화된 워커에 할당된 백그라운드 세션이 유효한 자격증명이 구성되어 있어도 이런 식으로 실패할 수 있었습니다. v2.1.176 이전에는 요청되기 전에 유휴 상태였던 클라우드 세션도 마찬가지였습니다. 업그레이드하여 복구합니다.

**수행할 작업:**

* 백그라운드 또는 클라우드 세션에 이것이 나타나고 자격증명이 이미 구성되어 있으면 v2.1.176 이상으로 업그레이드합니다.
* `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` 또는 클라우드 공급자 자격증명이 대화형 셸뿐만 아니라 워커를 실행하는 환경에 설정되어 있는지 확인합니다.
* Agent SDK의 경우, [빠른 시작의 인증 설정](/docs/ko/agent-sdk/quickstart#setup)을 참조하십시오.
* 동일한 환경의 대화형 세션에서 `/status`를 실행하여 어떤 자격증명 소스가 확인되는지 확인합니다.

<h3 id="invalid-api-key">
  잘못된 API 키
</h3>

`ANTHROPIC_API_KEY` 환경 변수 또는 `apiKeyHelper` 스크립트가 API가 거부한 키를 반환했습니다. 또는 Claude Code가 `ANTHROPIC_API_KEY`의 키를 전송하기 전에 차단했습니다.

```text theme={null}
Invalid API key · Fix external API key
```

메시지가 `Fix external API key` 이후로 `Invalid X-Api-Key header value from ANTHROPIC_API_KEY: it contains a line break at character 41 (120 characters on 2 lines).`와 같은 설명으로 계속되면, API는 키를 보지 못했습니다. Claude Code가 HTTP 헤더가 전달할 수 없는 문자를 찾았고 전송하기 전에 요청을 중지했습니다. 설명을 읽고 값을 수정하는 방법은 [잘못된 요청 헤더 값](#invalid-request-header-value)을 참조하십시오.

**수행할 작업:**

* 오타를 확인하고 [Console](https://platform.claude.com/settings/keys)에서 키가 취소되지 않았는지 확인합니다.
* 동일한 셸에서 `env | grep ANTHROPIC`을 실행하거나, PowerShell에서 `Get-ChildItem Env:ANTHROPIC*`을 실행합니다. direnv, dotenv 셸 플러그인 및 IDE 터미널과 같은 도구는 명시적으로 설정하지 않고도 프로젝트의 `.env` 파일에서 오래된 키를 로드할 수 있습니다.
* `ANTHROPIC_API_KEY`를 설정 해제하고 `/login`을 실행하여 대신 구독 인증을 사용합니다.
* 키가 [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 스크립트에서 오는 경우, 스크립트를 직접 실행하여 stdout에 유효한 키를 인쇄하는지 확인합니다.
* `/status`를 실행하여 Claude Code가 실제로 사용 중인 자격증명 소스를 확인합니다.

<h3 id="your-apikeyhelper-script-is-failing">
  apiKeyHelper 스크립트가 실패함
</h3>

Claude Code가 [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 설정의 명령을 실행했지만 키를 다시 받지 못했습니다. 키 없이는 요청이 자리 표시자 자격증명으로 API에 도달하고, API가 `401`로 거부합니다. 터미널의 `Authentication` 패널은 다음 중 어떤 일이 발생했는지 보여줍니다:

* 명령이 오류로 종료되었거나 시간 초과됨
* 명령이 stdout에 아무것도 인쇄하지 않음
* 명령이 로그인 배너 또는 로그 라인과 같은 키 이외의 것을 인쇄했습니다. 패널은 `returned output that cannot be used as an API key`를 표시하고 무엇이 잘못되었는지 말하며, 출력을 반복하지 않습니다. v2.1.227 이전에는 Claude Code가 주변 공백을 자른 후 명령이 인쇄한 모든 것을 전송했습니다.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

[비대화형 모드](/docs/ko/headless)에서 stderr도 `apiKeyHelper failed:`로 접두사가 붙은 특정 이유를 전달합니다.

Claude Code는 스크립트를 다시 실행하고 이 메시지를 표시하기 전에 요청을 최대 2회 더 재시도하므로, 실패는 3번의 시도 내에 표시됩니다. v2.1.208 이전에는 Claude Code가 전체 [재시도 예산](#automatic-retries)을 자리 표시자 자격증명으로 요청을 재전송하는 데 사용한 후 일반적인 `401` 인증 오류 대신 스크립트 실패를 보고했습니다.

`/login`을 실행하는 것은 여기서 도움이 되지 않습니다: 헬퍼의 출력은 설정이 있는 한 저장된 로그인보다 [우선합니다](/docs/ko/authentication#authentication-precedence).

**수행할 작업:**

* `apiKeyHelper`에 구성된 명령을 셸에서 직접 실행하여 실패를 재현합니다.
* 명령이 만료된 세션을 보고하면, 예를 들어 SSO 또는 비밀 자격증명 모음에 다시 로그인하여 자격증명 공급자로 다시 인증합니다.
* 명령이 stdout에만 키를 인쇄하도록 수정합니다. 최대 16,384자의 인쇄 가능한 ASCII의 단일 토큰으로, 종료 코드 0으로 종료합니다. 작동하는 설정은 [apiKeyHelper로 자격증명 회전](/docs/ko/llm-gateway-connect#rotate-credentials-with-apikeyhelper)을 참조하십시오.
* `/status`를 실행하여 `apiKeyHelper`가 활성 자격증명 소스인지 확인합니다. `apiKeyHelper` 행은 `Failing`을 표시하며 마지막 실패의 세부 정보(예: 종료 코드 및 명령의 오류 출력)를 표시하고, 다음 성공적인 실행 후 사라집니다. v2.1.274 이전에는 `/status`가 자격증명 소스만 표시했고 실패를 표시하지 않았습니다.
* 명령이 실패할 때마다 종료 코드와 오류 출력이 터미널의 `Authentication` 패널에 나타납니다. v2.1.212 이전에는 패널의 제목이 `Cloud authentication`이었습니다.

<h3 id="invalid-request-header-value">
  잘못된 요청 헤더 값
</h3>

Claude Code가 요청 헤더로 전송하려던 값에 HTTP 헤더가 전달할 수 없는 문자가 포함되어 있습니다: 줄 바꿈, NUL 바이트 또는 `U+00FF` 위의 문자(예: 곡선 따옴표 또는 너비가 0인 공백). Claude Code는 아무것도 전송되기 전에 요청을 중지하고 수정할 변수 또는 설정의 이름을 지정합니다. 일반적인 원인은 보이지 않는 문자 또는 잘못된 줄 바꿈을 전달한 문서 또는 채팅에서 붙여넣은 자격증명입니다.

Claude Code는 Claude API에 직접 또는 [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 요청을 전송할 때 이 확인을 실행합니다. [Amazon Bedrock](/docs/ko/amazon-bedrock)과 같은 타사 클라우드 공급자에서 Claude Code는 전송하기 전에 실행하지 않습니다.

```text theme={null}
Invalid auth token · Fix external auth token
Invalid ANTHROPIC_CUSTOM_HEADERS · Fix the environment variable
Invalid request header from the environment · Fix the environment variable
```

메시지의 첫 번째 부분은 잘못된 값이 어디에서 왔는지에 따라 달라집니다:

* `Invalid auth token`: [`ANTHROPIC_AUTH_TOKEN`](/docs/ko/env-vars) 또는 [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/ko/env-vars)의 베어러 토큰
* `Invalid ANTHROPIC_CUSTOM_HEADERS`: [`ANTHROPIC_CUSTOM_HEADERS`](/docs/ko/env-vars)에서 설정한 헤더 이름 또는 값입니다. 설명은 `distinct header 2 of 3 parsed from ANTHROPIC_CUSTOM_HEADERS`와 같이 어떤 `Name: Value` 쌍이 잘못되었는지 세지만, 이름이나 값을 반복하지 않습니다.
* `Invalid request header from the environment`: Claude Code가 `CLAUDE_AGENT_SDK_CLIENT_APP`과 같은 다른 환경 변수에서 요청 헤더로 복사하는 값입니다. 설명은 수정할 변수의 이름을 지정합니다.

Claude Code는 이 확인으로 포착된 잘못된 `ANTHROPIC_API_KEY`를 [잘못된 API 키](#invalid-api-key)로 보고하며, 동일한 후행 설명이 있습니다. 저장된 `/login` 자격증명이 잘못된 경우 [로그인하지 않음](#not-logged-in)으로 보고합니다. 대신 `/login`을 실행하여 새 자격증명을 저장합니다. [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 스크립트의 출력은 이 확인에 도달하지 않습니다: Claude Code는 스크립트가 실행될 때 유효성을 검사하고, HTTP 헤더가 전달할 수 없는 출력은 [apiKeyHelper 스크립트가 실패함](#your-apikeyhelper-script-is-failing)으로 실패합니다.

두 번째 `·` 이후에 메시지는 다음과 같은 전체 예제에서 문제를 설명합니다:

```text theme={null}
Invalid auth token · Fix external auth token · Invalid Authorization header value from ANTHROPIC_AUTH_TOKEN: it contains a line break at character 41 (120 characters on 2 lines).
```

위치는 1부터 시작하는 문자를 세십시오. 설명은 고정된 구문과 문자 수로 구성되므로 값 자체를 포함하지 않습니다. 바이트 순서 표시, 너비가 0인 공백 또는 곡선 따옴표와 같은 잘 알려진 보이지 않는 또는 인쇄 문자인 경우에만 잘못된 문자의 이름을 지정하고, 다른 모든 것을 `a non-ASCII character`로 보고합니다.

**수행할 작업:**

* 메시지가 이름을 지정한 변수 또는 설정을 다시 설정하고, 동일한 소스에서 붙여넣지 않고 보고된 위치 주변의 문자를 다시 입력합니다.
* `ANTHROPIC_CUSTOM_HEADERS`의 경우, 한 줄에 하나의 `Name: Value` 쌍을 유지하고 메시지가 세는 쌍을 다시 작성합니다.
* `/status`를 실행하여 어떤 자격증명 소스가 활성인지 확인합니다.

<h3 id="this-organization-has-been-disabled">
  이 조직이 비활성화됨
</h3>

Claude Code가 비활성화된 Console 조직의 오래된 `ANTHROPIC_API_KEY`를 사용하고 있습니다. 저장된 구독 로그인이 있으면 키가 이를 재정의합니다.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your subscription instead
Your ANTHROPIC_API_KEY belongs to a disabled organization · Update or unset the environment variable
API Error: 400 ... This organization has been disabled.
```

`·` 이후의 힌트는 저장된 자격증명에 따라 달라집니다: 첫 번째 형식은 저장된 `/login`이 키를 설정 해제한 후 인수할 수 있을 때 나타나고, 두 번째는 키가 유일한 자격증명일 때 나타납니다.

환경 변수는 `/login`보다 우선하므로, 셸 프로필에서 내보낸 키 또는 `.env` 파일에서 로드된 키는 작동하는 Pro 또는 Max 구독이 있어도 사용됩니다. 비대화형 모드(`-p`)에서는 키가 있을 때 항상 사용됩니다.

**수행할 작업:**

* 현재 셸에서 `ANTHROPIC_API_KEY`를 설정 해제하고 셸 프로필에서 제거한 후 `claude`를 다시 실행합니다.
* 메시지가 `Update or unset`이라고 말하면, 폴백할 저장된 로그인이 없습니다. 키를 설정 해제하고 `/login`을 실행하거나, 활성 Console 조직의 키로 바꿉니다.
* 그 후 `/status`를 실행하여 활성 자격증명이 구독인지 확인합니다.
* 환경 변수가 설정되지 않았고 오류가 지속되면, 비활성화된 조직은 `/login`에 연결된 조직입니다. 지원팀에 문의하거나 다른 계정으로 로그인합니다.

<h3 id="your-organization-has-disabled-api-key-authentication">
  조직이 API 키 인증을 비활성화함
</h3>

이 메시지에는 Claude Code v2.1.169 이상이 필요합니다. Console 조직의 관리자가 API 키 인증을 끄었으므로 API가 Claude Code가 전송하는 키를 거부합니다. 복구 힌트는 `·` 이후에 키가 어디에서 왔는지에 따라 달라집니다:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

환경 변수와 `apiKeyHelper`는 `/login`보다 우선하므로, `/login`을 단독으로 실행하는 것은 둘 중 하나가 여전히 키를 공급하는 동안 도움이 되지 않습니다. [인증 우선순위](/docs/ko/authentication#authentication-precedence)를 참조하십시오.

**수행할 작업:**

* 메시지가 `ANTHROPIC_API_KEY`의 이름을 지정하면, 현재 셸에서 설정을 해제하고 셸 프로필 또는 `.env` 파일에서 제거한 후 `claude`를 다시 실행합니다.
* 메시지가 `apiKeyHelper`의 이름을 지정하면, `settings.json`에서 [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 설정을 제거합니다.
* `/login`을 실행하여 claude.ai 계정으로 로그인합니다.
* 그 후 `/status`를 실행하여 활성 자격증명이 API 키가 아닌 구독인지 확인합니다.
* 자동화를 위해 API 키 인증이 필요한 경우, 조직 관리자에게 Console에서 다시 활성화하도록 요청합니다.

<h3 id="your-organization-has-disabled-claude-subscription-access">
  조직이 Claude 구독 액세스를 비활성화함
</h3>

Claude 조직이 구독 로그인으로 Claude Code에 로그인하는 것을 허용하지 않습니다. 동일한 계정으로 `/login`을 다시 실행하면 동일한 오류가 반환됩니다.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

이것은 서버 측 조직 설정이므로 로컬 설정, 환경 변수 또는 CLI 플래그에서 재정의할 수 없습니다.

Agent SDK 및 `-p` 비대화형 모드는 이를 `oauth_org_not_allowed` 오류 코드로 표시합니다.

**수행할 작업:**

* 조직의 관리자에게 조직에 대한 Claude Code 액세스를 활성화하도록 요청합니다.
* 구독 대신 Console API 키로 인증합니다. 설정은 [Claude Console 인증](/docs/ko/authentication#claude-console-authentication)을 참조하십시오.
* 관리자이고 액세스를 활성화하는 옵션이 보이지 않으면, [Anthropic 지원](https://support.claude.com)에 문의합니다.

<h3 id="routines-are-disabled-by-your-organizations-policy">
  루틴이 조직의 정책에 의해 비활성화됨
</h3>

Team 또는 Enterprise 조직의 Owner가 조직 수준에서 루틴을 끄었습니다. 오류는 예를 들어 claude.ai/code의 [루틴](/docs/ko/routines) UI에서 루틴을 만들거나 실행하려고 할 때 나타납니다. Claude Code v2.1.227 이상에서는 동일한 설정이 CLI에서 [`/schedule`도 숨깁니다](/docs/ko/routines#troubleshooting).

```text theme={null}
Routines are disabled by your organization's policy.
```

이것은 서버 측 설정이므로 로컬 설정, 환경 변수 또는 CLI 플래그에서 재정의할 수 없습니다.

**수행할 작업:**

* 조직의 Owner에게 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)에서 **루틴** 토글을 활성화하도록 요청합니다.
* 조직 수준의 루틴이 필요하지 않은 일회성 예약 작업의 경우, [예약된 작업](/docs/ko/scheduled-tasks)을 참조하십시오.

<h3 id="remote-control-requires-the-anthropic-api">
  원격 제어에는 Anthropic API가 필요함
</h3>

세션이 Anthropic API와 직접 통신하지 않으므로 [원격 제어](/docs/ko/remote-control)가 쌍을 이룰 claude.ai 백엔드가 없습니다.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com. CLAUDE_CODE_USE_BEDROCK is set, so this session is using Amazon Bedrock — unset it (or run in a shell without it) to use Remote Control.
```

두 번째 문장은 세션을 Anthropic API에서 멀어지게 한 것을 설명합니다. v2.1.219 이전에는 메시지가 첫 번째 문장만 있었습니다. 원인에 따라 메시지는 다음의 이름을 지정합니다:

* [Amazon Bedrock](/docs/ko/amazon-bedrock)의 `CLAUDE_CODE_USE_BEDROCK` 또는 [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai)의 `CLAUDE_CODE_USE_VERTEX`와 같은 `CLAUDE_CODE_USE_*` 공급자 변수
* [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)이 `api.anthropic.com` 이외의 호스트를 가리키고 있습니다. 예를 들어 [LLM 게이트웨이](/docs/ko/llm-gateway) 또는 프록시이며, claude.ai로 로그인할 때도 마찬가지입니다. v2.1.196 이전에는 사용자 정의 기본 URL이 원격 제어를 차단하지 않았습니다.
* `ANTHROPIC_UNIX_SOCKET`이 설정되어 있으므로 세션이 `api.anthropic.com`이 아닌 로컬 소켓을 통해 요청을 전송합니다.
* `/login`을 통한 엔터프라이즈 [클라우드 게이트웨이](/docs/ko/claude-apps-gateway) 로그인으로, 원격 제어를 지원하지 않으며 설정 해제할 변수가 없습니다.

**수행할 작업:**

* 메시지가 이름을 지정한 변수(예: `CLAUDE_CODE_USE_BEDROCK` 또는 `ANTHROPIC_BASE_URL`)를 설정 해제하고 세션을 다시 시작하거나, Anthropic API와 직접 통신하는 세션에서 원격 제어를 시작합니다.
* 변수가 셸에 설정되지 않았으면, [설정 파일](/docs/ko/settings#where-settings-live)의 `env` 키를 확인하십시오. 이는 모든 세션에 환경 변수를 적용합니다.
* 이 및 다른 원격 제어 시작 메시지의 경우, [원격 제어 문제 해결](/docs/ko/remote-control#troubleshooting)을 참조하십시오.

<h3 id="remote-control-couldnt-refresh-your-login">
  원격 제어가 로그인을 새로 고칠 수 없음
</h3>

Claude Code는 저장된 claude.ai 로그인을 사용하여 얻고 갱신하는 단기 자격증명에서 라이브 [원격 제어](/docs/ko/remote-control) 연결을 실행합니다. claude.ai가 해당 로그인을 더 이상 수락하지 않거나 Claude Code에 저장된 로그인이 남아 있지 않으면, Claude Code는 원격 제어를 중지하고 다시 로그인하도록 요청합니다. 두 실패 모두 Claude Code가 여전히 연결 중이거나 나중에 자격증명을 갱신할 때 발생할 수 있습니다.

Claude Code가 로그인 서비스에 저장된 로그인을 새로 고치도록 요청하고 응답을 받지 못하면, 원격 제어를 계속 실행하고 연결의 현재 자격증명이 여전히 유효한 동안 새로 고침을 다시 시도합니다. 새로 고침이 응답을 받지 못하는 경우는 Claude Code가 로그인 서비스에 도달할 수 없거나, 요청이 시간 초과되거나, 서비스가 로그인을 거부하지 않고 실패할 때입니다. 로그인 서비스가 해당 자격증명이 만료될 때까지 여전히 응답하지 않으면, Claude Code는 원격 제어를 중지하고 `OAuth token refresh failed`를 보고합니다.

Claude Code가 원격 제어를 중지하면, 경고 및 `Remote Control disconnected`로 시작하는 기록 라인에 이유를 표시합니다. 로컬 세션은 원격 제어 없이 계속 실행됩니다. 이 섹션은 다음 라인을 다룹니다:

```text theme={null}
Remote Control disconnected — Claude.ai login expired — run /login to restore Remote Control
Remote Control disconnected — Claude.ai login expired — run /login, then /remote-control
Remote Control disconnected — Claude.ai login was rejected — run /login, then /remote-control
Remote Control disconnected — OAuth token unavailable — run /login to restore Remote Control
Remote Control disconnected — OAuth token refresh failed — run /login to re-authenticate
Remote Control disconnected — JWT refresh failed: no OAuth token — run /login
Remote Control disconnected — Signed out of Claude — run /login, then /remote-control
```

Claude Code는 메시지의 중간에 원인의 이름을 지정합니다:

* ` Claude.ai login expired` 및 `Claude.ai login was rejected`: claude.ai가 더 이상 저장된 로그인 토큰을 수락하지 않습니다. 만료되었거나 취소되었기 때문입니다.
* ` OAuth token unavailable`: Claude Code가 연결의 자격증명이 갱신될 때 저장된 로그인 토큰이 없었습니다.
* `OAuth token refresh failed`: claude.ai가 Claude Code가 다시 연결할 때 저장된 로그인 토큰을 거부했고, 토큰을 새로 고치면 새 토큰이 생성되지 않았습니다.
* `JWT refresh failed: no OAuth token`: Claude Code가 갱신할 저장된 로그인 토큰을 찾지 못했습니다.
* ` Signed out of Claude`: 예를 들어 다른 터미널에서 `/logout`을 실행하여 이 머신에서 로그아웃했으므로 Claude Code가 연결을 갱신할 저장된 로그인이 없습니다.

**수행할 작업:**

* `/login`을 실행하여 다시 로그인합니다.
* `/remote-control`을 실행하여 세션을 다시 연결합니다. ` run /login to restore Remote Control`으로 끝나는 메시지는 이 단계가 필요하지 않습니다: Claude Code는 로그인하면 자동으로 다시 연결됩니다.

v2.1.224 이전에는 `OAuth token refresh failed — run /login to re-authenticate`가 `OAuth token refresh failed — re-authenticate, then re-enable Remote Control`으로 읽혔고, `JWT refresh failed: no OAuth token — run /login`이 `no OAuth token available for recovery (code <N>)`으로 읽혔습니다. ` Claude.ai login expired`, `Claude.ai login was rejected` 및 `OAuth token unavailable` 메시지는 v2.1.225에서 추가되었습니다.

v2.1.238 이전에는 Claude Code가 현재 `Signed out of Claude`라고 말하는 경우를 `JWT refresh failed: no OAuth token — run /login`으로 보고했고, 한 번의 로그인 새로 고침이 응답을 받지 못하자마자 `Claude.ai login expired — run /login to restore Remote Control`으로 원격 제어를 중지했습니다.

<h3 id="remote-control-stopped-because-the-signed-in-account-changed">
  원격 제어가 로그인한 계정이 변경되어 중지됨
</h3>

Claude Code는 [원격 제어](/docs/ko/remote-control) 세션 중에 이 라인을 표시합니다. 이 머신에서 다른 claude.ai 계정 또는 조직으로 로그인합니다. 예를 들어 다른 터미널에서 `/login`을 실행하여 Claude Code 세션 외부에서 전환했습니다.

`/login`을 통해 로그인하는 동안 시작한 원격 제어 세션은 당시에 로그인한 claude.ai 계정 및 조직에 속합니다.

```text theme={null}
Remote Control disconnected — signed-in claude.ai account or organization changed on this machine — run /remote-control to start a session for the current account, or /login to switch back, then /remote-control
```

Claude Code는 claude.ai가 계정 또는 조직이 변경되었음을 확인하는 즉시 원격 제어 세션을 중지합니다. 로컬 세션은 원격 제어 없이 계속 실행됩니다.

**수행할 작업:**

* `/remote-control`을 실행하여 현재 계정 또는 조직에서 새 원격 제어 세션을 시작합니다.
* 다시 전환하려면 `/login`을 실행하고 이전 계정 또는 조직으로 다시 로그인합니다. 그런 다음 `/remote-control`을 실행합니다.

v2.1.234 이전에는 Claude Code가 Claude Code 세션 외부에서 다른 계정 또는 조직으로 전환할 때 알아차리지 못했습니다. Claude Code는 원격 제어 서버에 대한 나중의 요청이 `Remote Control server rejected the request (HTTP 404)`로 실패할 때까지 원격 제어 세션을 연결된 상태로 유지했습니다. 이 실패는 전환 후 몇 시간 후에 올 수 있습니다.

<h3 id="remote-control-stopped-because-the-app-running-the-session-signed-out-or-switched-accounts">
  원격 제어가 세션을 실행하는 앱이 로그아웃하거나 계정을 전환했기 때문에 중지됨
</h3>

Claude 데스크톱 앱 또는 IDE가 세션을 호스팅할 때, Claude Code는 `/login` 대신 해당 앱에서 로그인 토큰을 가져옵니다. claude.ai가 해당 토큰을 거부할 때, Claude Code는 앱에 새 토큰을 요청합니다. 앱이 로그아웃했거나 이제 다른 Claude 계정으로 로그인했다고 응답하면, Claude Code는 [원격 제어](/docs/ko/remote-control) 세션을 종료하고 앱에 다음 라인 중 하나를 보냅니다:

```text theme={null}
Remote Control stopped — the app running this session is now signed in to a different Claude account
Remote Control stopped — the app running this session is signed out of Claude. Sign in there, then turn Remote Control back on
```

로컬 세션은 원격 제어 없이 계속 실행됩니다.

**수행할 작업:**

* 앱이 로그아웃되면, 다시 로그인한 후 앱에서 원격 제어를 다시 켭니다.
* 앱이 계정을 전환하면, Claude Code는 새 계정에서 종료된 세션을 계속할 수 없습니다. 해당 계정에서 새 원격 제어 세션을 시작합니다.

v2.1.238 이전에는 Claude Code가 두 경우 모두에서 [원격 제어가 로그인을 새로 고칠 수 없음](#remote-control-couldnt-refresh-your-login)에 나열된 `run /login` 메시지를 앱에 보냈습니다.

<h3 id="oauth-token-revoked-or-expired">
  OAuth 토큰이 취소되었거나 만료됨
</h3>

저장된 로그인이 더 이상 유효하지 않습니다. 취소된 토큰은 어디서나 로그아웃했거나 관리자가 액세스를 제거했음을 의미합니다. 만료된 토큰은 자동 새로 고침이 세션 중에 실패했음을 의미합니다.

두 메시지 모두 Claude Code가 전송한 요청에 대해 API가 반환한 거부를 보고합니다. 저장된 로그인이 실패한 새로 고침 후 이미 지워진 경우, 대신 [로그인 만료됨](#login-expired)을 참조하십시오. [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/ko/env-vars)의 장기 토큰으로 인증하면, 해당 토큰이 만료되거나 취소될 때 동일한 메시지가 표시됩니다.

```text theme={null}
OAuth token revoked · Please run /login
Please run /login · API Error: 401 OAuth token has expired ...
```

**수행할 작업:**

* `/login`을 실행하여 다시 로그인합니다.
* 오류가 다시 인증한 후 동일한 세션 내에서 반환되면, 먼저 `/logout`을 실행하여 저장된 토큰을 완전히 지운 후 `/login`을 실행합니다.
* ` CLAUDE_CODE_OAUTH_TOKEN` 환경 변수로 인증하면, Claude Code는 요청이 401로 실패한 후 설정한 값을 계속 전송하고, 저장된 로그인의 토큰으로 전환하지 않습니다. [`/status`](/docs/ko/commands)는 이 자격증명을 `Auth token` 행으로 표시하며 `CLAUDE_CODE_OAUTH_TOKEN`을 읽습니다. [`claude setup-token`](/docs/ko/authentication#generate-a-long-lived-token)으로 새 토큰을 생성하고 이를 사용하여 다시 시작하거나, 변수를 설정 해제하고 `/login`을 실행합니다. v2.1.225 이전에는 Claude Code가 세션 중에 변수의 값을 저장된 로그인의 단기 액세스 토큰으로 바꿀 수 있었고, 해당 토큰이 만료되면 세션이 다시 401 오류로 실패했습니다.
* 시작 간 반복되는 로그인 프롬프트의 경우, [문제 해결](/docs/ko/troubleshoot-install#not-logged-in-or-token-expired)의 시스템 시계 확인 및 macOS 자격증명 저장소 복구 단계를 참조하십시오.
* `403 Forbidden` 및 OAuth 브라우저 문제를 포함한 다른 실패의 경우, [로그인 및 인증](/docs/ko/troubleshoot-install#login-and-authentication)을 참조하십시오.

<h3 id="api-error-401-invalid-authentication-credentials">
  API 오류: 401 잘못된 인증 자격증명
</h3>

API가 자격증명의 형식을 인식했지만 뒤에 있는 계정 또는 조직을 거부했습니다. Anthropic은 자격증명이 최근에 취소되었을 때, 조직이 비활성화되었거나 액세스를 제거했을 때, 또는 계정 자체가 비활성화되었을 때 이 메시지를 반환하므로, 만료된 토큰이 원인이 아닙니다. 자격증명은 저장된 로그인 또는 승인된 `ANTHROPIC_API_KEY`일 수 있으며, 수정이 다르므로 `/status`를 실행하여 어떤 것이 활성인지 확인하여 시작합니다.

```text theme={null}
Please run /login · API Error: 401 Invalid authentication credentials
```

**수행할 작업:**

* `/status`가 사용 중이 아닌 것으로 표시되지 않은 `API key` 행을 표시하면, 승인된 [`ANTHROPIC_API_KEY`](/docs/ko/authentication#authentication-precedence)가 활성 자격증명이고 로그인보다 우선하므로 `/login`이 이를 바꾸지 않습니다. Claude Console에서 키를 회전하거나, `unset ANTHROPIC_API_KEY`를 실행하거나, PowerShell에서 `Remove-Item Env:ANTHROPIC_API_KEY`를 실행하여 구독으로 폴백합니다.
* `/status`가 로그인만 표시하면, `/login`을 한 번 실행합니다. 자격증명이 취소되었으면, 새 로그인이 이를 바꿉니다.
* 동일한 로그인 계정에 대해 동일한 메시지가 반환되면, 계정 또는 조직이 더 이상 활성화되지 않습니다. `/status`가 보고하는 계정 및 조직을 확인하고, 조직 관리자에게 액세스를 복구하도록 요청합니다.
* [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)이 [LLM 게이트웨이](/docs/ko/llm-gateway)를 가리키면, `401` 이후의 텍스트는 Anthropic의 메시지가 아닌 게이트웨이의 메시지이고, `/login`이 이를 변경하지 않습니다. 대신 게이트웨이가 예상하는 자격증명을 수정합니다.

<h3 id="login-expired">
  로그인 만료됨
</h3>

Claude Code가 저장된 claude.ai 또는 Claude Console 로그인을 갱신하려고 시도했고 OAuth 서비스가 저장된 새로 고침 토큰을 거부했으므로, Claude Code가 저장된 자격증명을 지웠습니다. 그 후, 각 모델 요청은 API에 도달하기 전에 로컬에서 이 메시지로 중지됩니다. `/login`만 새 자격증명을 만들 수 있기 때문입니다.

v2.1.206 이전에는 Claude Code가 환경에 남아 있는 모든 자격증명으로 모델 요청을 어쨌든 전송했고, 모든 모델이 로그인하라는 프롬프트 대신 [선택한 모델에 문제가 있음](#theres-an-issue-with-the-selected-model) 또는 401로 실패했습니다.

```text theme={null}
Login expired · Please run /login
```

[비대화형 모드](/docs/ko/headless)(`-p`) 및 [Agent SDK](/docs/ko/agent-sdk/overview)에서 메시지는 다음과 같이 읽으며, 구조화된 오류 코드는 `authentication_failed`입니다:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

이것은 [OAuth 토큰이 취소되었거나 만료됨](#oauth-token-revoked-or-expired)과 동일한 상태가 아닙니다. 이러한 메시지는 API가 반환한 거부를 보고합니다. Claude Code 자체는 이미 갱신하지 못한 로그인에 대해 `Login expired`를 생성하므로, 요청을 전송하지 않습니다. 갱신이 로그인이 오래되었기 때문이 아니라 계정 자체가 일시 중지되었기 때문에 실패하면, Claude Code는 대신 [계정이 보류 중임](#your-account-is-on-hold)을 표시합니다.

API 키, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/ko/env-vars) 또는 타사 공급자로 인증된 세션은 저장된 로그인을 사용하지 않으며 이 메시지를 절대 보지 않습니다.

` /status`를 실행하여 요청이 실패하기 전에 이 상태를 확인할 수 있습니다: 로그인 행을 표시하며 `Expired — log in again`을 읽고, 만료된 로그인에 대해 저장한 조직 및 이메일을 표시합니다. 행은 저장된 로그인이 활성 자격증명이고 더 이상 갱신할 수 없을 때만 나타납니다. 다른 방식으로 인증된 세션은 만료된 로그인이 남아 있어도 행을 표시하지 않습니다. v2.1.210 이전에는 `/status`가 이 상태에서 로그인이 존재했던 적이 있다는 표시를 주지 않았습니다. 지워진 자격증명이 보고할 것이 없었기 때문입니다.

**수행할 작업:**

* `/login`을 실행하여 다시 로그인합니다. 로그인하지 않고 재시도하면 모든 요청에서 동일한 메시지가 표시됩니다.
* 비대화형 모드에서 동일한 환경에서 `claude`를 실행하고, `/login`을 완료한 후 명령을 다시 실행합니다. 대화형으로 로그인할 수 없는 자동화의 경우, `ANTHROPIC_API_KEY` 또는 [`claude setup-token`으로 장기 토큰 생성](/docs/ko/authentication#generate-a-long-lived-token)으로 인증합니다.
* 로그인이 계속 실패하면, [로그인 및 인증](/docs/ko/troubleshoot-install#login-and-authentication)을 참조하십시오.

<h3 id="claude-login-not-accepted">
  Claude 로그인이 수락되지 않음
</h3>

[클라우드 세션](/docs/ko/claude-code-on-the-web)을 시작하려고 시도했고, 서버가 401로 생성을 거부했습니다: 이 머신이 보낸 Claude 로그인을 수락하지 않았습니다. 일반적으로 로그인이 만료되었거나 취소되었기 때문입니다.

라인의 첫 번째 부분은 서버가 제공할 때 서버의 자체 이유입니다. 그렇지 않으면 라인은 다음과 같이 읽습니다:

```text theme={null}
Claude login not accepted · Run /login, then try again
```

**수행할 작업:**

* `/login`을 실행하고, 로그인을 완료한 후 세션을 다시 시작합니다.

<h3 id="artifacts-need-a-claude-ai-login">
  아티팩트에 claude.ai 로그인이 필요함
</h3>

Claude Code가 [아티팩트](/docs/ko/artifacts) 게시 또는 읽기를 거부했습니다. 세션에 아티팩트에 사용할 수 있는 claude.ai 로그인이 없기 때문입니다.

메시지의 모든 형식은 동일한 단어로 시작하고, 세션이 인증하는 방식에 따라 달라지는 해결책이 뒤따릅니다. 경쟁하는 자격증명이 없으면 다음과 같이 읽습니다:

```text theme={null}
Artifacts need a claude.ai login. Run /login and select "Claude account with subscription", then retry — the "Anthropic Console account" option does not provide claude.ai credentials.
```

**수행할 작업:**

* `/login`을 실행하고 **Claude account with subscription**을 선택합니다. **Anthropic Console account** 옵션은 claude.ai 자격증명을 제공하지 않습니다.
* 메시지가 `ANTHROPIC_API_KEY`, `apiKeyHelper` 설정 또는 이전 `/login`으로 저장된 Console 키와 같이 우선하는 자격증명의 이름을 지정하면, 메시지가 말하는 방식으로 제거한 후 `/login`을 실행합니다.
* 메시지가 이 원격 세션이 이를 실행한 머신을 통해 인증된다고 말하면, 해당 머신에서 claude.ai에 로그인한 후 세션을 다시 연결합니다.
* 메시지가 자격증명이 세션의 호스트 환경에 의해 주입된다고 말하면, 해당 세션에서 변경할 수 없습니다. claude.ai에 로그인한 세션을 시작합니다.
* [가용성](/docs/ko/artifacts#availability)에서 계획, 모델 공급자 및 조직 정책과 같은 아티팩트가 가진 다른 요구 사항을 참조하십시오.

<h3 id="administrator-policy-requires-a-cloud-gateway-sign-in">
  관리자 정책에 클라우드 게이트웨이 로그인이 필요함
</h3>

관리자의 [관리 설정](/docs/ko/managed-settings)이 이 머신에서 [`forceLoginMethod`](/docs/ko/settings-reference#forceloginmethod)를 `"gateway"`로 설정했거나 [`forceLoginGatewayUrl`](/docs/ko/settings-reference#forcelogingatewayurl)을 설정했습니다. `CLAUDE_CODE_USE_BEDROCK`과 같은 변수를 통해 클라우드 공급자를 선택하지 않으면, Claude Code는 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 로그인만 수락합니다. 두 메시지 중 하나가 표시됩니다:

```text theme={null}
Not signed in to the Cloud gateway — run /login.
```

세션에 게이트웨이 로그인이 없을 때 모델 요청이 이 메시지로 실패합니다. 예를 들어 정책이 머신에 도달한 이후 `/login`을 실행하지 않았기 때문입니다.

`ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` 또는 `apiKeyHelper` 자격증명이 구성되어 있고 관리 설정이 `forceLoginMethod`를 설정하면, Claude Code는 대신 시작 시 다음과 같이 시작하는 메시지로 종료됩니다:

```text theme={null}
Administrator policy requires a Cloud gateway sign-in on this machine; the
Anthropic-issued credential configured here (ANTHROPIC_API_KEY,
ANTHROPIC_AUTH_TOKEN, or apiKeyHelper) is not used.
```

**수행할 작업:**

* `/login`을 실행하고 **클라우드 게이트웨이** 화면에서 로그인을 완료합니다.
* 시작 메시지의 경우, 구성한 `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` 또는 `apiKeyHelper` 설정을 제거한 후 `claude`를 시작하고 `/login`을 실행합니다.
* 머신이 게이트웨이를 요구하지 않아야 한다고 생각하면, 관리자에게 관리 설정에서 `forceLoginMethod` 및 `forceLoginGatewayUrl`을 제거하도록 요청합니다.

v2.1.265에서는 회귀로 인해 API 키, `apiKeyHelper` 또는 사용자 정의 헤더로 인증하는 일부 LLM 게이트웨이 및 프록시 구성에서도 첫 번째 메시지가 표시되었습니다. 머신에 관리자 요구 사항이 없어도 마찬가지입니다. v2.1.266 이상으로 업그레이드합니다. 구성을 변경할 필요가 없습니다.

v2.1.261 이전에는 `forceLoginMethod`를 `"gateway"`로 설정한 머신에서 Claude Code가 모델 요청을 실패하는 대신 남은 저장된 로그인을 사용했고, 구성된 환경 자격증명을 시작 메시지 대신 `This machine's managed settings require a first-party login`으로 보고했습니다. v2.1.265 이전에는 관리 설정이 `forceLoginGatewayUrl`만 설정한 머신이 게이트웨이 로그인을 요구하지 않았고, Claude Code가 거기서 남은 자격증명을 사용했습니다.

<h3 id="your-account-is-on-hold">
  계정이 보류 중임
</h3>

Claude 계정이 로그인 뒤에 일시 중지되었습니다. Claude Code는 저장된 로그인을 갱신하려고 시도할 때 보류를 알게 되면 첫 번째 메시지를 표시하고, 브라우저에서 완료한 로그인이 이를 보고하면 두 번째 메시지를 표시합니다:

```text theme={null}
Your account is on hold and can't use Claude Code. View details or appeal: https://claude.ai/restricted
Your account is on hold and can't sign in to Claude Code. View details or appeal: https://claude.ai/restricted
```

동일한 계정으로 다시 로그인하면 메시지가 지워지지 않습니다. 보류가 로그인이 아닌 계정에 있기 때문입니다. [비대화형 모드](/docs/ko/headless)(`-p`) 및 [Agent SDK](/docs/ko/agent-sdk/overview)에서 구조화된 오류 코드는 `account_on_hold`입니다. v2.1.235 이전에는 Claude Code가 보류된 계정을 [로그인 만료됨 · /login을 실행하십시오](#login-expired)로 보고했으며, 복구 단계가 보류를 지울 수 없습니다.

**수행할 작업:**

* 메시지의 링크를 열어 보류의 세부 정보를 보거나 이의를 제기합니다.
* 보류의 영향을 받지 않는 다른 Claude 계정 또는 API 키가 있으면, 보류가 해결되는 동안 계속 작업할 수 있습니다: 해당 계정으로 `/login`을 실행하거나, `ANTHROPIC_API_KEY`로 키를 설정합니다.

<h3 id="anthropic-profile-login-expired">
  Anthropic 프로필 로그인 만료됨
</h3>

Claude Code가 저장된 로그인 자격증명이 만료된 Anthropic 자격증명 프로필을 통해 인증하고 있으며, 프로필이 갱신하는 데 사용할 수 있는 새로 고침 자격증명을 보유하지 않습니다. Claude Code는 동일한 만료된 자격증명을 읽을 재시도가 하므로 각 요청을 로컬에서 중지합니다.

```text theme={null}
Anthropic profile login expired · Re-authenticate your Anthropic profile
Anthropic profile login expired · Run /login to use your claude.ai account instead, or re-authenticate the profile
```

이것은 활성 자격증명이 Anthropic 자격증명 프로필에서 오는 경우에만 나타납니다. 하나는 `ANTHROPIC_PROFILE` 환경 변수로 선택하고, Claude Code가 Anthropic 구성 디렉토리에서 활성 프로필로 발견하거나, Claude Code가 [API 키 없이 로그인](/docs/ko/authentication#sign-in-without-an-api-key)할 때 작성했습니다. `/login`의 claude.ai 옵션, API 키, `ANTHROPIC_AUTH_TOKEN`과 같은 베어러 토큰 또는 타사 공급자로 인증된 세션은 이 메시지를 절대 보지 않습니다.

[키 없는 로그인을 제공](/docs/ko/authentication#sign-in-without-an-api-key)하는 머신에서 `/login`을 실행하고, Anthropic Console 계정을 선택하고, 다시 로그인하여 키 없는 Console 로그인 또는 Claude Platform CLI의 `ant auth login`이 작성한 프로필을 갱신합니다. Claude Code는 해당 프로필의 만료된 자격증명을 바꿉니다. 페더레이션 프로필 또는 다른 도구가 만든 프로필의 경우, `/login`이 자격증명을 갱신하지 않습니다. 어떤 형식이 표시되는지는 프로필을 선택했는지 또는 Claude Code가 발견했는지에 따라 달라집니다:

* `ANTHROPIC_PROFILE`을 명시적으로 설정하면, 메시지는 `Re-authenticate your Anthropic profile`로 끝납니다.
* Claude Code가 구성 디렉토리에서 프로필을 발견하면, 메시지는 `/login`을 제공합니다. Claude Code가 작동하는 `/login`을 발견된 프로필보다 우선하고 대신 claude.ai 또는 Console 계정으로 인증하기 때문입니다. v2.1.234 이전에는 Claude Code가 이 경우에도 `Re-authenticate your Anthropic profile` 형식을 표시했습니다.

**수행할 작업:**

* 프로필을 다시 인증한 후 재시도합니다: [키 없는 로그인을 제공](/docs/ko/authentication#sign-in-without-an-api-key)하는 머신에서, 키 없는 Console 로그인 또는 Claude Platform CLI의 `ant auth login`이 작성한 프로필의 경우 `/login`을 실행하고 Anthropic Console 계정을 선택합니다. 다른 프로필의 경우, 프로필을 만든 도구를 사용합니다.
* 관리자가 프로필의 자격증명을 프로비저닝했으면, 새 자격증명을 발급하도록 요청합니다.
* `/status`를 실행하여 활성 자격증명 소스 및 프로필 이름을 확인합니다.
* 프로필 사용을 중지하려면, 설정했으면 `ANTHROPIC_PROFILE`을 설정 해제한 후 `/login` 또는 `ANTHROPIC_API_KEY`와 같은 다른 방식으로 인증합니다.

<h3 id="oauth-scope-requirement">
  OAuth 범위 요구 사항
</h3>

저장된 토큰이 최신 기능이 필요한 권한 범위보다 앞서 있습니다. 이것은 `/usage` 및 상태 라인 사용 표시기에서 가장 자주 표시됩니다:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**수행할 작업:**

* `/login`을 실행하여 현재 범위가 있는 새 토큰을 가져옵니다. 먼저 로그아웃할 필요가 없습니다.

<h3 id="claude-ai-rejected-the-session-token">
  claude.ai가 세션 토큰을 거부함
</h3>

[claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai) 요청이 실패했습니다. claude.ai가 Claude Code 로그인의 토큰을 거부했습니다. 일반적으로 만료되었고 새로 고칠 수 없는 로그인입니다. 거부된 토큰은 로그인이지, 커넥터의 자체 claude.ai 인증이 아니므로, 커넥터를 다시 인증해도 해결되지 않습니다. `/mcp`에서 커넥터는 `connected · session token rejected`로 표시되고 세부 정보 보기는 다음과 같이 읽습니다:

```text theme={null}
claude.ai rejected the session token. Run /login, then reconnect.
```

**수행할 작업:**

* `/login`을 실행하여 다시 로그인합니다.
* `/mcp`에서 커넥터를 다시 연결하거나, `/mcp reconnect <server>`를 실행합니다. 다시 로그인하기 전에 다시 연결하면 커넥터가 동일한 상태로 유지됩니다. `/mcp` 패널의 **다시 연결** 옵션은 `your claude.ai session token was rejected`를 보고합니다. 입력된 `/mcp reconnect <server>` 형식은 토큰이 여전히 거부되었음에도 불구하고 성공적인 다시 연결을 보고합니다.

v2.1.222 이전에는 Claude Code가 커넥터를 인증이 필요한 것으로 표시했으며, 이는 완료해도 상태를 해결하지 않는 커넥터의 인증 흐름을 가리켰습니다.

<h3 id="mcp-server-needs-you-to-sign-in-again">
  MCP 서버가 다시 로그인하도록 요청함
</h3>

원격 [MCP 서버](/docs/ko/mcp)가 세션 중에 도구 호출에 대한 자격증명을 거부했습니다. 일반적으로 로그인 또는 토큰이 만료되었거나 취소되었거나 토큰이 도구가 필요한 권한이 부족하기 때문입니다. 도구 호출이 실패하고 `/mcp`는 서버를 [인증이 필요한 것](/docs/ko/mcp#authenticate-with-remote-mcp-servers)으로 표시합니다.

Claude Code에서 로그인하는 서버(claude.ai 커넥터 포함)의 경우, 로그인이 만료되었거나 취소되었습니다:

```text theme={null}
MCP server "<name>" needs you to sign in again (run /mcp to re-authenticate)
```

`/mcp`를 실행하고, 서버를 선택하고, 메뉴에서 다시 로그인합니다.

[`headersHelper`](/docs/ko/mcp#use-dynamic-headers-for-custom-authentication) 스크립트로 구성된 서버의 경우, Claude Code가 이미 헬퍼를 다시 실행하고 호출을 한 번 재시도한 후 이것을 표시합니다:

```text theme={null}
MCP server "<name>" rejected the credential from its headersHelper (check the helper and run /mcp to reconnect, or to authenticate if the server also uses OAuth)
```

헬퍼가 서버가 수락하는 자격증명을 반환하는지 확인한 후, `/mcp`에서 다시 연결합니다. 이는 헬퍼를 다시 실행합니다.

구성에 정적 `Authorization` 헤더가 있는 서버의 경우:

```text theme={null}
MCP server "<name>" rejected the Authorization header in its config (update it, then run /mcp to reconnect)
```

서버가 구성된 곳에서 헤더 값을 업데이트한 후 `/mcp`에서 다시 연결합니다.

v2.1.273 이전에는 만료된 로그인, `headersHelper` 및 `Authorization` 헤더 경우가 모두 `MCP server "<name>" requires re-authorization (token expired)`를 표시했습니다.

서버는 HTTP 403 `insufficient_scope`로 도구 호출을 거부하여 범위를 인증하도록 요청할 수도 있습니다. 때로는 토큰이 이미 나열한 범위입니다. 메시지는 해당 범위의 이름을 지정합니다:

```text theme={null}
MCP server "<name>" needs additional permissions (scope: "<scope>") — run /mcp to re-authenticate
```

`/mcp`를 실행하고, 서버를 선택하고, 메뉴에서 다시 인증합니다.

서버의 구성이 [`oauth.scopes`](/docs/ko/mcp#restrict-oauth-scopes) 또는 [`authServerMetadataUrl`](/docs/ko/mcp#override-oauth-metadata-discovery)을 설정하지 않으면, Claude Code가 서버가 이름을 지정한 범위를 요청합니다. 어느 설정이든 Claude Code가 해당 설정의 범위를 요청합니다. `oauth.scopes`를 고정했으면, 다시 인증하기 전에 누락된 범위를 해당 목록에 추가합니다.

v2.1.274 이전에는 이 경우가 `needs you to sign in again` 메시지를 표시했고, v2.1.273 이전에는 다른 경우처럼 `requires re-authorization (token expired)`를 표시했습니다.

<h3 id="issuer-mismatch-in-authorization-response">
  인증 응답의 발급자 불일치
</h3>

[MCP OAuth 로그인](/docs/ko/mcp#authenticate-with-remote-mcp-servers) 중에 인증 서버가 Claude Code로 리디렉션되었으며, `iss` 매개변수가 Claude Code가 서버의 OAuth 메타데이터에서 예상한 발급자의 이름을 지정하지 않습니다. 이 단계에서 잘못된 발급자는 인증 서버 혼합 공격이 어떻게 보이는지이므로, Claude Code는 인증 코드를 교환하는 대신 로그인을 실패합니다. Claude Code는 브라우저 로그인 후 `/mcp` 서버 메뉴에 오류를 표시합니다:

```text theme={null}
Issuer mismatch in authorization response (RFC 9207): expected "https://auth.example.com", received "https://other.example.com"
```

`expected`는 서버의 OAuth 메타데이터의 발급자이고, `received`는 리디렉션이 전달한 `iss` 값입니다. 리디렉션이 `iss` 매개변수를 전달하지 않는 로그인은 확인을 통과합니다. 서버의 메타데이터가 `authorization_response_iss_parameter_supported`를 설정하지 않으면, 이 경우 Claude Code는 로그인을 실패합니다.

**수행할 작업:**

* `/mcp`에서 로그인을 다시 시도합니다.
* 오류가 반복되면, 서버 운영자에게 보고합니다. 수정은 서버 측입니다: 인증 서버는 메타데이터에서 광고하는 것과 동일한 발급자를 `iss` 매개변수에서 반환해야 합니다.
* 서버가 수정되는 동안 연결하려면, [`MCP_SDK_GENERATION=v1`](/docs/ko/env-vars)로 Claude Code를 시작합니다. 이 [런타임](/docs/ko/mcp#mcp-client-runtimes)은 이 확인을 실행하지 않습니다. 이는 혼합 공격에 대한 보호를 제거하므로 서버 측 수정을 선호합니다.

v2.1.232 이전에는 Claude Code가 점진적 롤아웃에서만 또는 `MCP_SDK_GENERATION=v2`를 설정할 때 v2 런타임을 사용했습니다.

<h3 id="aws-credentials-expired-or-invalid">
  AWS 자격증명이 만료되었거나 유효하지 않음
</h3>

AWS 세션 토큰이 만료되었거나 거부되었습니다. 이 메시지는 [Claude Platform on AWS](/docs/ko/claude-platform-on-aws) 또는 [Mantle 엔드포인트](/docs/ko/amazon-bedrock#use-the-mantle-endpoint)의 401에 나타나며, 이는 해당 공급자가 만료된 보안 토큰을 보고하는 방법입니다.

중간의 작업 힌트는 설정에 따라 달라집니다. 안정적인 부분은 선행 `AWS credentials expired or invalid`입니다:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

v2.1.273 이전에는 `awsAuthRefresh`가 구성되었을 때만 이 메시지가 나타났습니다.

**수행할 작업:**

* 힌트가 자격증명이 이 환경에서 관리된다고 말하면, 앱이 Claude Code를 실행하고 여기의 다른 단계가 적용되지 않습니다: 재시도하거나, 관리자에게 문의합니다.
* [`awsAuthRefresh`](/docs/ko/amazon-bedrock#advanced-credential-configuration)가 설정되면, 메시지에서 이름을 지정한 명령(예: `aws sso login --profile myprofile`)을 다른 터미널에서 실행하고 브라우저 로그인을 완료한 후 재시도합니다. 그렇지 않으면 직접 사용하는 AWS 자격증명을 새로 고칩니다: SSO 로그인, 액세스 키, API 키 또는 프록시 토큰
* 대화형 세션에서 `awsAuthRefresh`가 설정되면, 대신 `/login`을 실행하고, **3rd-party platform**을 선택한 후, **Using 3rd-party platforms** 아래에서 **Claude Platform on AWS · refresh credentials**를 선택하여 Claude Code를 다시 시작하지 않고 동일한 명령을 실행할 수 있습니다. [AWS 자격증명 구성](/docs/ko/claude-platform-on-aws#1-configure-aws-credentials)을 참조하십시오.
* 새로 고침 명령이 성공한 후 오류가 반복되면, 동일한 셸 및 프로필에서 `aws sts get-caller-identity`로 Claude Code 외부에서 ID가 유효한지 확인합니다.

<h3 id="aws-authentication-failed">
  AWS 인증 실패
</h3>

AWS 공급자가 403을 반환했거나, [Amazon Bedrock](/docs/ko/amazon-bedrock)이 401을 반환했습니다.

Amazon Bedrock은 만료된 보안 토큰을 403으로 보고하지만, 403은 또한 누락된 IAM 권한과 같은 `AccessDeniedException`의 인증 거부를 보고하는 방법입니다. Claude Code는 이 두 원인을 구분할 수 없습니다.

Amazon Bedrock의 401은 또한 [AWS 자격증명이 만료되었거나 유효하지 않음](#aws-credentials-expired-or-invalid) 아래가 아닌 여기에 도달합니다. Amazon Bedrock이 만료된 토큰을 401로 보고하지 않기 때문입니다. 해당 엔드포인트의 401은 일반적으로 요청 경로의 다른 것(예: 회사 프록시)에서 옵니다.

자격증명 새로 고침은 만료된 토큰을 수정하고 다른 원인을 수정할 수 없으므로, 메시지는 둘 다 제공합니다:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

중간의 작업 힌트는 설정에 따라 달라집니다. 안정적인 부분은 선행 `AWS authentication failed`입니다.

403이 지정된 모델 ID로 모델에 액세스할 수 없다는 Amazon Bedrock의 답변일 때, 힌트는 대신 Amazon Bedrock 콘솔에서 계정 및 지역에 대해 모델을 활성화하도록 지시합니다.

v2.1.273 이전에는 `awsAuthRefresh`가 구성되었을 때만 이 메시지가 나타났습니다.

**수행할 작업:**

* 힌트가 자격증명이 이 환경에서 관리된다고 말하면, 앱이 Claude Code를 실행하고 여기의 다른 단계가 적용되지 않습니다: 재시도하거나, 관리자에게 문의합니다.
* 만료된 자격증명이 원인일 수 있으므로 AWS 자격증명을 새로 고칩니다: 설정되면 [`awsAuthRefresh`](/docs/ko/amazon-bedrock#advanced-credential-configuration)에서 이름을 지정한 명령을 실행하거나, SSO 로그인, 액세스 키, API 키 또는 프록시 토큰을 직접 새로 고칩니다.
* 자격증명이 최신이면, [IAM 구성](/docs/ko/amazon-bedrock#iam-configuration)의 권한이 사용 중인 ID에 연결되어 있고 선택한 모델이 계정 및 지역에 대해 활성화되어 있는지 확인합니다.
* `aws sts get-caller-identity`를 실행하여 요청이 어떤 ID를 사용하는지 확인합니다. 오래된 `AWS_PROFILE` 또는 기본 프로필은 권한 불일치의 일반적인 원인입니다.

<h3 id="google-cloud-credentials-expired-or-invalid">
  Google Cloud 자격증명이 만료되었거나 유효하지 않음
</h3>

[Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai)의 Google Cloud 자격증명이 만료되었거나 거부되었습니다: 요청이 401을 반환했으며, 이는 Agent Platform이 자격증명 만료를 보고하는 방법입니다.

중간의 작업 힌트는 설정에 따라 달라집니다. 안정적인 부분은 선행 `Google Cloud credentials expired or invalid`입니다:

```text theme={null}
Google Cloud credentials expired or invalid · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · API Error: 401 ...
```

**수행할 작업:**

* 힌트가 자격증명이 이 환경에서 관리된다고 말하면, 앱이 Claude Code를 실행하고 여기의 다른 단계가 적용되지 않습니다: 재시도하거나, 관리자에게 문의합니다.
* 애플리케이션 기본 자격증명으로 인증하면, 메시지에서 이름을 지정한 [`gcpAuthRefresh`](/docs/ko/google-vertex-ai#advanced-credential-configuration) 명령 또는 `gcloud auth application-default login`을 실행하고 로그인을 완료한 후 재시도합니다.
* `CLAUDE_CODE_SKIP_VERTEX_AUTH`가 설정된 [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 라우팅하면, `ANTHROPIC_AUTH_TOKEN` 또는 `ANTHROPIC_CUSTOM_HEADERS`의 게이트웨이 토큰을 새로 고친 후 재시도합니다.
* 서비스 계정 키 파일로 인증하면, `GOOGLE_APPLICATION_CREDENTIALS`가 유효한 키를 가리키는지 확인합니다. [GCP 자격증명 구성](/docs/ko/google-vertex-ai#3-configure-gcp-credentials)을 참조하십시오.
* 새로 고침 후 오류가 반복되면, 동일한 셸에서 `gcloud auth application-default print-access-token`으로 Claude Code 외부에서 ID가 작동하는지 확인합니다.

v2.1.273 이전에는 Agent Platform의 401이 대신 일반적인 `Please run /login` 또는 `Failed to authenticate` 메시지를 표시했으며, Google Cloud 자격증명을 새로 고칠 수 없습니다.

<h3 id="google-cloud-authentication-failed">
  Google Cloud 인증 실패
</h3>

[Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai)이 403을 반환했으며, 이는 만료된 자격증명이 아닌 인증 거부에 사용합니다. 일반적으로 인증하는 ID에 IAM 권한이 누락되었거나 모델이 프로젝트에 대해 활성화되지 않았습니다.

중간의 작업 힌트는 설정에 따라 달라집니다. 안정적인 부분은 선행 `Google Cloud authentication failed`입니다:

```text theme={null}
Google Cloud authentication failed · refresh your Google Cloud credentials (application default sign-in, or the key file in GOOGLE_APPLICATION_CREDENTIALS) and retry · if credentials are current, check GCP IAM permissions and Vertex AI model access · API Error: 403 ...
```

**수행할 작업:**

* 힌트가 자격증명이 이 환경에서 관리된다고 말하면, 앱이 Claude Code를 실행하고 여기의 다른 단계가 적용되지 않습니다: 재시도하거나, 관리자에게 문의합니다.
* [IAM 구성](/docs/ko/google-vertex-ai#iam-configuration)의 역할이 인증하는 ID에 부여되었는지 확인합니다.
* 모델이 프로젝트에 대해 활성화되었는지 확인합니다. [모델 액세스 요청](/docs/ko/google-vertex-ai#2-request-model-access)을 참조하십시오.

v2.1.273 이전에는 Agent Platform의 403이 대신 일반적인 `Please run /login` 또는 `Failed to authenticate` 메시지를 표시했으며, Google Cloud 자격증명을 새로 고칠 수 없습니다.

<h3 id="microsoft-foundry-authentication-failed">
  Microsoft Foundry 인증 실패
</h3>

[Microsoft Foundry](/docs/ko/microsoft-foundry)가 401 또는 403을 반환했습니다: 요청의 Azure 자격증명이 거부되었거나, 뒤에 있는 ID가 Foundry 리소스에 액세스할 수 없습니다. `/login`은 Azure 자격증명을 발급할 수 없습니다. 중간의 작업 힌트는 설정에 따라 달라집니다. 안정적인 부분은 선행 `Microsoft Foundry authentication failed`입니다:

```text theme={null}
Microsoft Foundry authentication failed · refresh your Foundry credential (ANTHROPIC_FOUNDRY_AUTH_TOKEN, ANTHROPIC_FOUNDRY_API_KEY, Azure sign-in for Entra, or your proxy token) and retry · if credentials are current, check access to the Foundry resource · API Error: 401 ...
```

**수행할 작업:**

* 힌트가 자격증명이 이 환경에서 관리된다고 말하면, 앱이 Claude Code를 실행하고 여기의 다른 단계가 적용되지 않습니다: 재시도하거나, 관리자에게 문의합니다.
* [Azure 자격증명 구성](/docs/ko/microsoft-foundry#2-configure-azure-credentials)에서 구성한 자격증명을 새로 고칩니다: `ANTHROPIC_FOUNDRY_API_KEY`를 회전하거나, 새 `ANTHROPIC_FOUNDRY_AUTH_TOKEN`을 발급하거나, `az login`을 실행하여 기본 Microsoft Entra 자격증명 체인이 다시 로그인할 수 있도록 합니다.
* 자격증명이 최신이면, ID가 Foundry 리소스에 액세스할 수 있는지 확인합니다. [Azure RBAC 구성](/docs/ko/microsoft-foundry#azure-rbac-configuration)을 참조하십시오.

v2.1.273 이전에는 Microsoft Foundry의 401 또는 403이 대신 일반적인 `Please run /login` 또는 `Failed to authenticate` 메시지를 표시했으며, Azure 자격증명을 새로 고칠 수 없습니다.

<h3 id="could-not-load-aws-or-google-cloud-credentials">
  AWS 또는 Google Cloud 자격증명을 로드할 수 없음
</h3>

Claude Code가 실행되는 머신의 AWS 자격증명 공급자 체인 또는 Google 애플리케이션 기본 자격증명에서 사용 가능한 자격증명을 얻을 수 없으므로, 클라우드 공급자에 도달한 요청이 없습니다. Claude Code는 캐시된 자격증명을 지우고 이 메시지를 표시하기 전에 두 번 재시도합니다. `·` 이후의 세부 정보는 만료된 SSO 세션, `Could not load the default credentials`로 보고된 누락된 애플리케이션 기본 자격증명 또는 `invalid_grant`로 보고된 취소된 로그인과 같은 특정 원인의 이름을 지정합니다:

```text theme={null}
API Error: Could not load AWS credentials · Could not load credentials from any providers. Check or refresh your AWS credentials and try again.
API Error: Could not load Google Cloud credentials · invalid_grant. Check or refresh your Google Cloud credentials and try again.
```

[비대화형 모드](/docs/ko/headless)에서 `-p`를 사용하고 [Agent SDK](/docs/ko/agent-sdk/overview)에서 구조화된 오류 코드는 `cloud_credential_error`입니다. v2.1.267 이전에는 메시지가 `API Error:` 이후의 세부 정보 텍스트만 표시했고, 구조화된 코드는 `server_error` 또는 `unknown`이었습니다.

**수행할 작업:**

* `aws sso login --profile myprofile` 또는 `gcloud auth application-default login`과 같은 공급자의 로그인 명령을 실행한 후 재시도합니다. [Bedrock, Agent Platform 또는 Foundry 자격증명이 로드되지 않음](/docs/ko/troubleshoot-install#bedrock-agent-platform-or-foundry-credentials-not-loading)은 Claude Code 외부에서 자격증명을 확인하는 방법을 보여줍니다.
* 세부 정보가 `AWS default-chain credential resolve timed out`을 읽으면, 체인이 실패하지 않고 중단되었으므로, 대신 [AWS 기본 체인 자격증명 확인 시간 초과](#aws-default-chain-credential-resolve-timed-out)를 따릅니다.

<h3 id="aws-default-chain-credential-resolve-timed-out">
  AWS 기본 체인 자격증명 확인 시간 초과
</h3>

AWS 기본 자격증명 공급자 체인이 60초 내에 자격증명을 생성하지 않았으므로, Claude Code가 확인을 중지하고 요청을 실패했습니다. 이 시간 초과는 [AWS 또는 Google Cloud 자격증명을 로드할 수 없음](#could-not-load-aws-or-google-cloud-credentials)의 한 원인입니다. 실패는 로컬 자격증명 확인입니다: 요청이 [Amazon Bedrock](/docs/ko/amazon-bedrock), [Claude Platform on AWS](/docs/ko/claude-platform-on-aws) 또는 [Mantle 엔드포인트](/docs/ko/amazon-bedrock#use-the-mantle-endpoint)에 도달하지 않았습니다. Claude Code는 이 오류가 표시되기 전에 반복된 시도에서 [자격증명 캐시](/docs/ko/amazon-bedrock#credential-caching-and-resolution-timeout)를 지우고 재시도하므로, 이 시점에서 체인이 반복된 시도에서 중단되었습니다.

```text theme={null}
API Error: Could not load AWS credentials · AWS default-chain credential resolve timed out. Check or refresh your AWS credentials and try again.
```

일반적인 원인은 AWS 프로필의 `credential_process` 명령이 받을 수 없는 입력을 기다리고 있으며, 컨테이너 또는 VM의 인스턴스 메타데이터 서비스(IMDS)가 체인의 프로브에 응답하지 않습니다.

v2.1.267 이전에는 메시지가 `API Error: AWS default-chain credential resolve timed out`을 읽었습니다.
v2.1.207 이전에는 중단된 체인이 요청을 무한정 기다리게 했습니다.

**수행할 작업:**

* 동일한 셸에서 동일한 `AWS_PROFILE`로 `aws sts get-caller-identity`를 실행합니다. 또한 중단되면, 프로필을 수정합니다. 대화형으로 프롬프트하는 `credential_process` 명령이 일반적인 원인입니다.
* Claude Code를 시작하기 전에 로그인 단계를 완료합니다. 예를 들어 `aws sso login --profile myprofile`을 실행하여 체인이 로컬 SSO 캐시에서 확인되도록 합니다.
* 체인이 `aws-vault`와 같은 래퍼를 통한 MFA를 사용하는 SSO와 같이 60초 이상 필요로 하는 대화형 로그인을 실행하면, [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/ko/env-vars)에서 밀리초 단위로 제한을 높입니다.

<h3 id="bedrock-setup-verification-timed-out-waiting-for-aws">
  Bedrock 설정 확인이 AWS를 기다리다가 시간 초과됨
</h3>

[Bedrock 설정 마법사](/docs/ko/amazon-bedrock#sign-in-with-bedrock)의 자격증명 확인 중 AWS 호출(예: 자격증명 조회 또는 ID 확인)이 60초 제한 내에 완료되지 않았습니다. 마법사가 기다리기를 중지하고 확인 단계를 실패합니다:

```text theme={null}
Timed out after 60s waiting for AWS. Check your network and proxy settings; if a credential helper needs longer to prompt you, raise CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS.
```

숫자는 제한을 반영합니다: 기본적으로 60초 또는 [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/ko/env-vars)에서 설정한 값입니다.

일반적인 원인은 AWS에 대한 요청을 중단하는 네트워크 또는 프록시(SSO 토큰 새로 고침 포함) 및 입력을 기다리는 자격증명 헬퍼입니다. 헬퍼가 합법적으로 더 많은 시간이 필요한 경우에만 제한을 높입니다.

AWS에 대한 단일 중단된 요청도 자체 요청별 시간 초과에서 실패할 수 있으며, 동일한 단계에서 더 짧은 메시지를 표시합니다:

```text theme={null}
A request to AWS timed out. Check your network and proxy settings, then try again.
```

동일한 시간 초과가 모델 핀 단계에서 발생하면, 마법사는 모델을 `unreachable`로 표시하는 대신 두 메시지 중 하나를 표시합니다.

**수행할 작업:**

* 동일한 셸에서 `aws sts get-caller-identity`를 실행합니다. 또한 중단되면, 중단이 Claude Code 외부에 있습니다. 네트워크, 프록시 또는 AWS 프로필의 자격증명 헬퍼에서 먼저 수정합니다.
* 마법사를 열기 전에 대화형 로그인을 완료합니다. 예를 들어 `aws sso login --profile myprofile`을 실행합니다.
* AWS 프로필의 자격증명 헬퍼가 합법적으로 60초 이상 필요하여 프롬프트하면, [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/ko/env-vars)에서 밀리초 단위로 제한을 높입니다.

<h3 id="cloud-gateway-session-expired">
  클라우드 게이트웨이 세션 만료됨
</h3>

[Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)를 통해 로그인했고, 이 머신에 저장된 게이트웨이 세션이 만료되었고 갱신할 수 없거나, 게이트웨이가 더 이상 수락하지 않습니다. 예를 들어 게이트웨이의 [JWT 비밀이 교체된](/docs/ko/claude-apps-gateway-deploy#jwt-secret-rotation) 후입니다. 대화형으로 `claude`를 시작할 때 이 라인이 표시되면, 세션이 게이트웨이에서 로그아웃된 상태로 열렸습니다:

```text theme={null}
Cloud gateway session expired — run /login to reconnect.
```

동일한 라인은 게이트웨이 자격증명이 만료되고 Claude Code가 갱신할 수 없을 때 세션 중에 나타날 수 있습니다.

[비대화형](/docs/ko/headless) 실행, 백그라운드 또는 기타 무인 세션 또는 `claude auth` 이외의 `claude` 하위 명령에서, 게이트웨이가 더 이상 세션을 수락하지 않으면 Claude Code는 대신 이 메시지로 종료됩니다:

```text theme={null}
Cloud gateway <url> no longer accepts this session. Start `claude` and sign in again with /login.
```

**수행할 작업:**

* 세션에서 `/login`을 실행하고 브라우저 로그인을 완료합니다.
* 비대화형 실행의 경우, 동일한 환경에서 `claude`를 시작하고, `/login`을 실행한 후 명령을 다시 실행합니다.

<h3 id="sign-in-timed-out-while-waiting-for-you-to-continue">
  로그인 시간 초과 중 계속 대기
</h3>

[Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 로그인 중에 게이트웨이가 로그인한 계정의 이름을 지정했고, Claude Code가 저장하기 전에 확인하도록 요청했습니다. 로그인의 자체 만료를 지나 확인을 열어 두었고, 게이트웨이가 새로 고침 토큰을 발급하지 않았으므로, Claude Code는 계속할 때 아무것도 저장하지 않았습니다:

```text theme={null}
Sign-in timed out while waiting for you to continue. Try again.
```

**수행할 작업:**

* `/login`을 다시 실행하고 로그인이 만료되기 전에 계정을 확인합니다.

<h3 id="gateway-refused-the-request">
  게이트웨이가 요청을 거부함
</h3>

[Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)를 통해 로그인했고, 요청이 403을 반환했습니다: 게이트웨이 또는 뒤의 업스트림이 거부했습니다. 다시 로그인하면 거부가 변경되지 않으므로, 메시지는 게이트웨이 관리자를 가리킵니다:

```text theme={null}
Gateway refused the request · signing in again won't change this — check with your gateway administrator · API Error: 403 ...
```

**수행할 작업:**

* 게이트웨이 관리자에게 요청을 조회하도록 요청합니다. `API Error:` 꼬리는 게이트웨이가 반환한 거부를 전달합니다.
* 관리자의 경우: 게이트웨이의 [액세스 제어 규칙](/docs/ko/claude-apps-gateway-config#http-tuning)이 [감사 로그](/docs/ko/claude-apps-gateway-deploy#logs)가 이유와 함께 기록하는 403을 반환하고, 업스트림의 인증 거부는 [업스트림 오류 메시지](/docs/ko/claude-apps-gateway-config#upstream-error-messages)에 따라 통과합니다.

v2.1.273 이전에는 게이트웨이 세션의 403이 대신 일반적인 `Please run /login` 또는 `Failed to authenticate` 메시지를 표시했고, 다시 로그인해도 거부가 지워지지 않았습니다.

<h2 id="network-and-connection-errors">
  네트워크 및 연결 오류
</h2>

이러한 오류의 대부분은 Claude Code의 네트워크 요청이 목적지에 도달하지 못했거나, Claude Code와 API 사이의 무언가가 반환 경로에서 응답을 변경했음을 의미합니다. 항목에 실패한 아카이브 쓰기와 같은 로컬 원인도 있는 경우, 본문에 명시되어 있습니다. 이러한 오류는 일반적으로 로컬 네트워크, 프록시 또는 방화벽, 또는 클라우드 환경의 네트워크 정책에서 발생합니다.

<h3 id="unable-to-connect-to-api">
  API에 연결할 수 없음
</h3>

API에 대한 TCP 연결이 실패했거나 완료되지 않았습니다. 일반적인 연결 오류 코드의 경우, 메시지는 실패의 종류를 명시하고 괄호 안에 코드를 유지합니다:

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

Claude Code가 인식하지 못하는 코드는 `Unable to connect to API`로 표시되고 괄호 안에 코드가 나타납니다. 이러한 메시지 중 일부는 두 개 이상의 코드를 표시할 수 있습니다. 예를 들어 `Connection refused`는 `ConnectionRefused` 또는 `ECONNREFUSED`를 표시할 수 있고, `Can't reach the API server`는 `ENOTFOUND` 또는 `FailedToOpenSocket`을 표시할 수 있습니다.

v2.1.227 이전에는 이러한 각 코드화된 메시지가 `Unable to connect to API` 다음에 코드를 읽었습니다. 예를 들어 `Unable to connect to API (ECONNREFUSED)`.

일반적인 원인으로는 인터넷 접근 불가, `api.anthropic.com`을 차단하는 VPN, 또는 구성되지 않은 필수 회사 프록시가 있습니다.

**수행할 작업:**

* 동일한 셸에서 `curl -I https://api.anthropic.com`을 실행하여 API 호스트에 도달할 수 있는지 확인합니다. Windows PowerShell에서는 `curl.exe -I https://api.anthropic.com`을 사용하여 기본 제공 `Invoke-WebRequest` 별칭이 사용되지 않도록 합니다.
* 회사 프록시 뒤에 있는 경우, Claude Code를 시작하기 전에 `HTTPS_PROXY`를 설정하고 [네트워크 구성](/docs/ko/network-config)을 참조합니다.
* LLM 게이트웨이 또는 릴레이를 통해 라우팅하는 경우, [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)을 해당 주소로 설정합니다. 설정은 [Claude Code를 LLM 게이트웨이에 연결](/docs/ko/llm-gateway-connect)을 참조합니다.
* 방화벽이 [네트워크 액세스 요구 사항](/docs/ko/network-config#network-access-requirements)에 나열된 호스트를 허용하는지 확인합니다.
* 간헐적 실패는 [자동으로 재시도](#automatic-retries)됩니다. 지속적인 실패는 로컬 네트워크 문제를 나타냅니다.

`curl`이 성공하지만 Claude Code가 여전히 실패하는 경우, 원인은 일반적으로 네트워크 자체가 아니라 런타임과 네트워크 사이의 무언가입니다:

* Linux 및 WSL에서 `/etc/resolv.conf`에서 도달할 수 없는 네임서버를 확인합니다. 특히 WSL은 호스트에서 손상된 리졸버를 상속할 수 있습니다.
* macOS에서 연결이 끊어지거나 제거된 VPN 클라이언트는 터널 인터페이스 또는 라우팅 규칙을 남길 수 있습니다. `ifconfig`에서 오래된 `utun` 인터페이스를 확인하고 시스템 설정에서 VPN의 네트워크 확장을 제거합니다.
* Docker Desktop 및 유사한 컨테이너 런타임은 아웃바운드 트래픽을 가로챌 수 있습니다. 이를 배제하기 위해 종료하고 다시 시도합니다.

<h3 id="unable-to-connect-to-anthropic-services">
  Anthropic 서비스에 연결할 수 없음
</h3>

첫 실행 설정 중에 Claude Code는 로그인 단계를 표시하기 전에 `api.anthropic.com` 및 `platform.claude.com`에 도달할 수 있는지 확인합니다. 두 확인 중 하나라도 실패하면 Claude Code는 이유를 인쇄하고 종료합니다.

```text theme={null}
Unable to connect to Anthropic services
Failed to connect to api.anthropic.com: ECONNREFUSED
Connection to api.anthropic.com timed out after 10 seconds
A proxy is configured via HTTPS_PROXY. Check that it allows connections to the host above.
```

Claude Code는 API 요청과 동일한 [프록시 구성](/docs/ko/network-config)을 통해 확인을 보내고 각 프로브에 10초를 제공합니다. 실패한 프로브가 프록시를 통과한 경우, 메시지는 `HTTPS_PROXY`와 같이 이를 구성한 환경 변수의 이름을 지정합니다. v2.1.222 이전에는 확인이 타임아웃이 없는 다른 프록시 전송을 사용했습니다. `https://` 스키마가 있는 프록시 URL 뒤에서 `Checking connectivity...`에서 무한정 정지될 수 있었고, 동일한 프록시를 통한 API 요청이 성공하더라도 실패할 수 있었습니다.

Claude Code는 [관리되는 설정 파일, MDM 정책 또는 정책 도우미](/docs/ko/managed-settings)가 [`forceLoginMethod`](/docs/ko/settings-reference#forceloginmethod)를 `"gateway"`로 설정하거나 `forceLoginMethod` 없이 [`forceLoginGatewayUrl`](/docs/ko/settings-reference#forcelogingatewayurl)을 설정할 때 이 확인을 건너뜁니다. 두 구성 중 하나를 사용하면 Claude Code는 Anthropic 로그인 방법이 아닌 **클라우드 게이트웨이** 화면에서 로그인 단계를 엽니다. Claude Code는 또한 머신에 관리되는 설정 소스가 존재하지만 읽을 수 없을 때 확인을 건너뜁니다. 해당 소스가 게이트웨이 구성을 보유할 수 있기 때문입니다. v2.1.247 이전에는 Claude Code가 이 구성에서도 확인을 실행했고, Anthropic의 엔드포인트에 도달할 수 없을 때 이 오류로 종료했습니다.

**수행할 작업:**

* 메시지가 프록시 변수의 이름을 지정하는 경우, 해당 값이 올바른 프록시를 가리키는지 확인하고 네트워크 팀에 메시지의 호스트에 대한 HTTPS 연결을 허용하도록 요청합니다. [네트워크 구성](/docs/ko/network-config)을 참조합니다.
* [API에 연결할 수 없음](#unable-to-connect-to-api)의 확인을 진행합니다. 거기의 `curl` 테스트 및 방화벽 지침이 이 확인에도 적용됩니다.
* 조직이 [클라우드 게이트웨이](/docs/ko/claude-apps-gateway)를 통해 로그인하고 이 오류가 첫 실행에 나타나는 경우, Claude Code v2.1.247 이상으로 업데이트합니다.
* 네트워크가 열려 있고 실패가 지속되는 경우, Claude Code가 [귀국에서 사용 가능하지 않을 수 있습니다](https://www.anthropic.com/supported-countries).

<h3 id="socket-is-closed">
  소켓이 닫혔음
</h3>

`Socket is closed`는 스트리밍 응답을 전달하는 연결이 응답이 여전히 도착하는 동안 닫혔음을 의미합니다. 가장 일반적인 원인은 Windows의 회사 프록시가 응답 중간에 설정된 터널을 삭제하는 것입니다.

응답이 진행된 정도에 따라 Claude Code는 요청을 재시도하거나, Claude가 생성한 내용을 유지하거나, 턴을 종료합니다. [자동 재시도](#automatic-retries)를 참조합니다.

v2.1.214 이전에는 Claude Code가 이 실패를 재시도하지 않았고, 턴이 `Socket is closed`를 포함하는 오류로 중지되었습니다.

**수행할 작업:**

* 이 오류가 표시되면 `claude update`로 v2.1.214 이상으로 업데이트한 다음 메시지를 다시 보냅니다.
* 업데이트 후 동일한 프록시 뒤에서 턴이 계속 실패하는 경우, [API에 연결할 수 없음](#unable-to-connect-to-api)을 진행하고 [네트워크 구성](/docs/ko/network-config)에서 프록시 설정을 확인합니다.

<h3 id="api-returned-an-empty-or-malformed-response">
  API가 빈 응답 또는 형식이 잘못된 응답을 반환했습니다
</h3>

Claude Code는 실패한 스트리밍 요청의 비스트리밍 재시도가 HTTP 성공 상태를 받지만 본문이 Claude API 메시지가 아닐 때 이 오류를 표시합니다. 일반적으로 HTML 오류 또는 로그인 페이지, 빈 본문 또는 다른 형식의 JSON입니다. 프록시, 게이트웨이 또는 네트워크 로그인 페이지가 API 대신 응답하는 것이 일반적인 원인입니다. Claude Code는 요청을 재시도하지 않으며, 턴이 이 오류로 종료됩니다.

```text theme={null}
API returned an empty or malformed response (HTTP 200) — check for a proxy or gateway intercepting the request.
```

그 시작 후, 메시지는 반환된 내용과 실패한 요청을 보고합니다:

* 콘텐츠 유형, `body is an HTML page` 또는 `empty body`와 같은 본문의 종류, 바이트 단위의 크기, 응답이 Anthropic 요청 ID를 전달했는지 여부를 포함하는 `Response:` 절. 응답이 `nginx` 또는 `cloudflare`와 같은 인식 가능한 서버의 이름을 지정하거나 `cf-ray` 또는 `via`와 같은 중간 헤더를 전달하는 경우, 절은 이들도 나열합니다.
* 실패한 스트리밍 요청의 ID와 재시도를 트리거한 실패의 이름을 지정하는 문장. 스트림이 실패 전에 열린 경우, 도착한 스트림 이벤트의 수와 도움이 된 경우 시도가 실패했을 때 스트림이 얼마나 오래 침묵했는지도 보고합니다.

v2.1.234 이전에는 메시지가 `intercepting the request` 후에 종료되었습니다.

v2.1.271 이전에는 `text/plain`과 같은 비 JSON 콘텐츠 유형 아래에서 유효한 API 메시지를 전달한 회신도 이 오류로 턴을 종료했습니다. 일부 LLM 게이트웨이는 비스트리밍 회신에 해당 콘텐츠 유형을 사용합니다.

**수행할 작업:**

* `Response:` 절을 읽어 어느 시스템이 응답했는지 확인합니다. HTML 본문, Anthropic 요청 ID 없음, 또는 `nginx` 또는 `cloudflare`와 같은 명명된 서버는 Claude Code와 API 사이의 무언가가 대신 응답했음을 의미합니다.
* [LLM 게이트웨이](/docs/ko/llm-gateway-connect#troubleshoot-gateway-errors)를 통해 라우팅하는 경우, 직접 요청으로 경로를 테스트하고 비 API 응답을 반환하는 홉을 수정합니다.
* 게스트 Wi-Fi와 같은 로그인 페이지가 있는 네트워크에서 브라우저에서 로그인을 완료한 다음 다시 시도합니다.
* 게이트웨이를 통한 비스트리밍 경로만 손상된 경우, [`CLAUDE_CODE_DISABLE_NONSTREAMING_FALLBACK=1`](/docs/ko/env-vars#variables)을 설정하여 스트림 중간에 실패한 요청이 스트리밍 엔드포인트 자체가 `404`를 반환할 때를 제외하고 일반 재시도 경로로 이동하도록 합니다. 여기서 Claude Code는 여전히 폴백합니다.

<h3 id="streaming-response-ended-before-any-complete-data-was-received">
  스트리밍 응답이 완전한 데이터를 받기 전에 종료됨
</h3>

모델 공급자의 스트리밍 응답이 사용 가능한 데이터를 전달하지 않고 완료되었으므로 Claude Code는 턴을 완료하기 위해 스트리밍 없이 요청을 다시 보냈습니다. Claude Code는 경고를 세션당 한 번, 대화형 세션에서만 표시합니다. v2.1.239 이전에는 Claude Code가 스트리밍 없이 자동으로 재시도했습니다.

```text theme={null}
Streaming response ended before any complete data was received. Retrying without streaming. If this keeps happening, check any proxy or gateway between Claude Code and your model provider.
```

Claude Code는 각 영향을 받는 요청을 두 번 보냅니다: 빈 스트리밍 시도 및 재시도. 일반적인 원인은 반환 경로에서 스트리밍 응답 본문을 소비하거나 변환하는 프록시 또는 게이트웨이입니다.

**수행할 작업:**

* Claude Code와 모델 공급자 사이의 프록시 또는 게이트웨이를 구성하여 스트리밍 응답 본문과 해당 헤더를 수정되지 않은 상태로 전달합니다.
* [Amazon Bedrock](/docs/ko/amazon-bedrock)에서 헤더 및 본문 요구 사항에 대해 [게이트웨이 또는 프록시 뒤의 스트리밍 오류](/docs/ko/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy)를 참조합니다.

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock 스트리밍 응답에 예상치 못한 content-type이 있습니다
</h3>

Claude Code와 [Amazon Bedrock](/docs/ko/amazon-bedrock) 사이의 게이트웨이 또는 프록시가 스트리밍 응답 본문 또는 해당 `Content-Type` 헤더를 변환하고 있습니다. Amazon Bedrock은 응답을 `application/vnd.amazon.eventstream`으로 스트리밍합니다. Claude Code는 읽을 수 없는 본문을 디코딩하는 대신 다른 content-type을 보고하는 성공적인 스트리밍 응답을 거부합니다. Claude Code는 요청을 재시도하지 않습니다.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

v2.1.208 이전에는 동일한 잘못된 구성이 전체 응답이 버퍼링된 후 `API Error: Truncated event message received`로 표시되었습니다.

**수행할 작업:**

* 게이트웨이를 구성하여 `InvokeModelWithResponseStream` 응답 본문과 해당 `Content-Type` 헤더를 수정되지 않은 상태로 전달합니다. 스트림을 서버 전송 이벤트로 다시 내보내는 중간자가 일반적인 원인입니다.
* [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/ko/env-vars)을 설정하면 이 오류가 숨겨지지만, Claude Code는 다시 작성된 헤더 아래에서 이진 본문을 디코딩하지 않으므로 이러한 요청은 더 느린 비스트리밍 경로로 폴백합니다. [게이트웨이 또는 프록시 뒤의 스트리밍 오류](/docs/ko/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy)를 참조합니다.

<h3 id="ssl-certificate-errors">
  SSL 인증서 오류
</h3>

네트워크의 프록시 또는 보안 어플라이언스가 자체 인증서로 TLS 트래픽을 가로채고 있으며, Claude Code가 이를 신뢰하지 않습니다.

```text theme={null}
Unable to connect to API: SSL certificate verification failed (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
Unable to connect to API: Self-signed certificate detected (SELF_SIGNED_CERT_IN_CHAIN). The certificate comes from an authority Claude Code doesn't trust, usually a TLS-inspecting corporate proxy or a gateway signed by a private CA: set NODE_EXTRA_CA_CERTS to that CA bundle, or add it to the system certificate store · see https://code.claude.com/docs/en/network-config
```

v2.1.273 이전에는 두 메시지 모두 OpenSSL 코드 또는 `NODE_EXTRA_CA_CERTS` 힌트 없이 `Check your proxy or corporate SSL certificates`에서 종료되었습니다.

v2.1.199부터 인증서 검증 실패는 재시도되지 않으므로 이 오류는 전체 [재시도 예산](#automatic-retries) 후가 아닌 첫 시도에 나타납니다. 이전 버전은 표시하기 전에 몇 분 동안 재시도했습니다. 핸드셰이크 타임아웃과 같은 일시적 TLS 조건은 여전히 재시도됩니다.

`/login` 및 시작 연결 확인 중에 동일한 실패는 다른 메시지를 생성합니다:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

[Amazon Bedrock](/docs/ko/amazon-bedrock)에서 Claude Code 자체가 AWS로 보내는 요청(예: STS 및 SSO 역할 자격 증명 호출, 모델 검색 및 설정 마법사의 확인)은 동일한 인증서 구성에 따라 달라집니다. [TLS 검사 프록시 뒤의 인증서 오류](/docs/ko/amazon-bedrock#certificate-errors-behind-a-tls-inspecting-proxy)를 참조합니다.

**수행할 작업:**

* 조직의 CA 번들을 내보내고 `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`으로 Claude Code를 가리킵니다.
* 전체 설정 지침은 [네트워크 구성](/docs/ko/network-config#custom-ca-certificates)을 참조합니다.
* `NODE_TLS_REJECT_UNAUTHORIZED=0`을 설정하지 마십시오. 이는 인증서 검증을 완전히 비활성화합니다.

<h3 id="host-not-allowed-in-a-cloud-session">
  클라우드 세션에서 호스트가 허용되지 않음
</h3>

클라우드 세션 또는 루틴의 아웃바운드 HTTP 요청이 환경의 네트워크 정책에 의해 차단되었습니다.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

대상의 실제 인증서와 일치하지 않는 TLS 인증서도 볼 수 있습니다. 클라우드 세션은 네트워크 정책을 적용하는 프록시를 통해 아웃바운드 트래픽을 라우팅하므로, 일치하지 않는 인증서는 프록시가 연결을 종료했음을 의미하며, 대상이 아닙니다.

이것은 클라이언트 측 네트워크 문제가 아닙니다. 클라우드 세션 및 [루틴](/docs/ko/routines)은 세션의 네트워크를 통한 아웃바운드 트래픽이 [클라우드 환경의](/docs/ko/cloud-environments) 허용 목록으로 필터링되는 샌드박스 VM 내에서 실행됩니다. [GitHub 작업](/docs/ko/cloud-environments#github-proxy) 및 MCP 커넥터 트래픽은 별도의 채널을 사용하므로 다른 호스트가 차단되는 동안 계속 작동할 수 있습니다. **기본** 환경은 **신뢰할 수 있는** 액세스를 사용하며, 이는 패키지 레지스트리, 클라우드 공급자 API, 컨테이너 레지스트리 및 일반적인 개발 도메인의 [기본 허용 목록](/docs/ko/cloud-environments#default-allowed-domains)을 허용하고 해당 경로의 다른 도메인을 차단합니다.

**수행할 작업:**

이러한 단계는 자신의 환경 중 하나를 변경합니다. [조직 공유 환경](/docs/ko/cloud-environments#organization-shared-environments)은 선택기에서 읽기 전용으로 열리므로, [관리 설정](https://claude.ai/admin-settings)의 **클라우드 환경** 페이지에서 소유자에게 네트워크 액세스를 변경하도록 요청합니다.

* 루틴을 편집하기 위해 열거나 클라우드 세션을 시작합니다. 환경의 이름(예: **기본**)을 표시하는 클라우드 아이콘을 선택하여 선택기를 엽니다. 환경 위에 마우스를 올리고 설정 아이콘을 클릭합니다.
* **클라우드 환경 업데이트** 대화 상자에서 **네트워크 액세스**를 **신뢰할 수 있는**에서 **사용자 정의**로 변경한 다음 차단된 도메인을 **허용된 도메인**에 추가합니다. 한 줄에 하나의 도메인을 입력합니다. **또한 일반적인 패키지 관리자의 기본 목록 포함**을 확인하여 사용자 정의 도메인과 함께 [기본 허용 목록](/docs/ko/cloud-environments#default-allowed-domains)을 유지합니다. 제한 없는 액세스를 원하는 경우 대신 **전체**를 선택합니다.
* **변경 사항 저장**을 클릭합니다. 다음 실행은 업데이트된 허용 목록을 사용합니다.

액세스 수준 및 기본 허용 목록은 [네트워크 액세스](/docs/ko/cloud-environments#network-access)를 참조합니다. 로컬 CLI 세션은 이 정책의 영향을 받지 않습니다.

<h3 id="the-proxy-refused-the-connection">
  프록시가 연결을 거부했습니다
</h3>

`HTTPS_PROXY` 또는 관련 [프록시 변수](/docs/ko/network-config#environment-variables)에서 설정한 프록시를 통해 Claude가 [아티팩트](/docs/ko/artifacts)를 읽을 때 이 메시지가 표시됩니다. 아티팩트 콘텐츠는 `*.frame.claudeusercontent.com`에서 제공되므로 Claude Code는 먼저 프록시에 해당 호스트에 대한 터널을 열도록 요청하는 `CONNECT` 요청을 보냅니다. 프록시가 거부하면 아무것도 호스트에 도달하지 않으며, 메시지는 프록시의 HTTP 상태를 전달합니다:

```text theme={null}
artifact content fetch failed (proxy refused the connection: HTTP 407)
artifact content fetch failed (proxy refused the connection: HTTP 403)
the proxy refused the connection to the artifact's content host (HTTP 502)
```

상태는 `CONNECT`에 대한 프록시의 응답입니다. 호스트는 응답하지 않았으므로 각 상태는 다른 수정을 가리킵니다:

* `HTTP 407`: 프록시가 받지 못한 자격 증명이 필요합니다. [기본 인증](/docs/ko/network-config#basic-authentication)이 표시하는 대로 프록시 URL에 넣습니다.
* `HTTP 403`: 프록시가 `*.frame.claudeusercontent.com`으로의 터널링을 거부합니다. 프록시를 실행하는 사람에게 해당 호스트를 허용하도록 요청합니다. [네트워크 액세스 요구 사항](/docs/ko/network-config#network-access-requirements)에 나열되어 있습니다.
* `HTTP 502`와 같은 다른 상태: 프록시가 호스트에 도달하지 못하는 등의 자체 이유로 터널을 열지 않았습니다. 프록시의 로그에서 상태를 찾습니다.
* 상태 대신 `unreadable reply`: 프록시 주소의 무언가가 HTTP 상태 줄로 응답하지 않았습니다. 주소가 HTTP 프록시인지 확인합니다.

**수행할 작업:**

* [프록시 구성](/docs/ko/network-config#proxy-configuration)이 설명하는 대로 프록시 변수의 주소와 자격 증명을 확인한 다음 Claude Code를 시작하는 셸에서 `curl -x http://proxy.example.com:8080 -I https://api.anthropic.com`을 실행합니다. 자신의 프록시 URL을 사용합니다. Windows PowerShell에서 `curl.exe`를 실행합니다. 이 프로브가 동일한 방식으로 실패하면 먼저 프록시 설정을 수정합니다. 성공하면 거부는 아티팩트 호스트에만 해당됩니다.
* 네트워크가 Claude Code가 아티팩트 호스트에 직접 도달하도록 허용하는 경우, `.frame.claudeusercontent.com`을 [`NO_PROXY`](/docs/ko/network-config#environment-variables)에 추가합니다. 항목을 좁게 유지합니다: 더 넓은 `.claudeusercontent.com` 항목도 [IP 허용 목록](/docs/ko/network-config#organization-ip-allowlists-and-proxy-egress)이 있는 조직이 프록시에 유지해야 하는 `bridge.claudeusercontent.com`에 대한 프록시를 우회합니다.

v2.1.238 이전에는 Claude Code가 거부된 터널을 일반 네트워크 오류로 보고했습니다.

<h3 id="the-cloud-environments-service-returned-an-empty-or-unexpected-response">
  클라우드 환경 서비스가 빈 응답 또는 예상치 못한 응답을 반환했습니다
</h3>

Claude Code는 CLI에서 클라우드 세션을 만들거나 [`/remote-env`](/docs/ko/cloud-environments#select-an-environment-from-the-cli)를 실행할 때와 같은 여러 지점에서 [클라우드 환경](/docs/ko/cloud-environments) 목록을 요청합니다. 서버의 응답을 읽을 수 없으면 다음 메시지 중 하나를 표시합니다:

```text theme={null}
The cloud environments service returned an empty response (HTTP 200 with no body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 with a non-JSON body). This is usually temporary — try again in a moment.
The cloud environments service returned a response in an unexpected format (HTTP 200 without a usable environments list). This is usually temporary — try again in a moment.
```

서버는 요청을 수락했지만 환경 목록이 아닌 본문으로 응답했습니다: 비어 있음, JSON이 아님, 또는 목록이 없는 JSON. 이는 일반적으로 서비스 측 중단을 동반하며 자체적으로 해결됩니다. 목록을 요청한 표면에 따라 Claude Code는 `/remote-env` 대화 상자에서 `couldn't list environments:`와 같은 접두사를 추가할 수 있습니다.

**수행할 작업:**

* 작업을 다시 시도합니다. Claude Code는 매번 목록을 다시 요청합니다.
* 메시지가 계속 나타나면 활성 인시던트에 대해 [status.claude.com](https://status.claude.com)을 확인합니다.

v2.1.236 이전에는 Claude Code가 이러한 메시지 대신 원시 JavaScript TypeError를 표시했습니다.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Remote Control 세션에 다시 연결할 수 없습니다
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

`claude --resume` 또는 `claude --continue`로 재개하면 해당 대화에 기록된 [Remote Control](/docs/ko/remote-control) 세션에 다시 연결됩니다. 이 메시지는 네트워크 중단 또는 서버 오류와 같이 일시적일 수 있는 이유로 재연결이 실패했음을 의미하므로 Claude Code는 원격 세션이 여전히 존재하는지 확인할 수 없습니다. 로컬 세션은 Remote Control 없이 계속 실행됩니다.

**수행할 작업:**

* `/remote-control`을 실행하여 연결을 다시 시도합니다.
* `claude --remote-control`로 새 세션을 시작하여 새 Remote Control 세션을 만듭니다.
* 다른 Remote Control 시작 메시지는 [Remote Control 문제 해결](/docs/ko/remote-control#troubleshooting)을 참조합니다.

서버가 대신 이전 세션이 없다고 보고하면 이 메시지가 표시되지 않습니다. Claude Code는 [대화의 재연결 기록](/docs/ko/remote-control#resume-outcomes)에 따라 새 세션을 시작하거나 [`Previous session is unavailable — run /remote-control to start a new one`](/docs/ko/remote-control#previous-session-is-unavailable)을 표시합니다. v2.1.227부터 v2.1.231까지 Claude Code는 `Remote Control could not resume the previous session under the current login`으로 시작하는 메시지를 표시했으며, [이전 버전은 다시 다르게 작동했습니다](/docs/ko/remote-control#reconnect-history).

<h3 id="sessions-ended-while-this-machine-was-offline">
  이 머신이 오프라인 상태인 동안 세션이 종료됨
</h3>

Claude Code는 머신이 오프라인 상태인 동안 서버가 머신이 제공하던 Remote Control 환경을 정리할 만큼 충분히 오래 있었던 후 [`claude remote-control`](/docs/ko/remote-control#start-a-remote-control-session)을 실행하는 터미널에 이 메시지를 표시합니다. 해당 환경의 세션이 종료되었으며 재개할 수 없습니다. 개수는 종료된 세션의 수입니다.

```text theme={null}
2 sessions ended while this machine was offline — the environment was cleaned up on the server and can't be resumed.
```

**수행할 작업:**

* Claude Code가 이 메시지 아래에 유지된 worktree를 나열할 때 이들에서 커밋되지 않은 작업을 선택합니다.
* `claude remote-control`을 실행하여 새로운 환경을 시작합니다.

<h3 id="couldnt-share-the-transcript">
  트랜스크립트를 공유할 수 없습니다
</h3>

[세션 품질 설문 조사](/docs/ko/data-usage#session-quality-surveys)와 같은 설문 조사 프롬프트에서 세션 트랜스크립트를 공유하기로 동의한 후 Claude Code는 이를 Anthropic에 업로드하거나 타사 공급자, [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션 및 Anthropic 자격 증명을 사용할 수 없을 때 로컬 아카이브를 대신 저장합니다. 이 메시지는 공유가 완료되지 않았음을 의미합니다.

```text theme={null}
Couldn't share the transcript.
```

업로드는 8 MiB 제한에 맞아야 합니다. 긴 세션에서 Claude Code는 점진적으로 공유의 일부를 삭제합니다. 마지막 요청의 모델 설정이 먼저, 그 다음 구조화된 대화 및 서브에이전트 트랜스크립트이며, 축소된 버전을 보낼 수 없거나 네트워크 또는 서버 오류가 업로드를 중지할 때만 이 메시지를 표시합니다. Claude Code가 로컬 아카이브를 대신 저장할 때, 메시지는 아카이브를 쓸 수 없었음을 의미합니다.

**수행할 작업:**

* `/feedback`을 실행하여 발생한 상황에 대한 설명과 함께 트랜스크립트를 보냅니다. 환경에서 `/feedback`을 사용할 수 없는 경우 [오류 보고](#report-an-error)를 참조합니다.
* 다른 요청도 실패하는 경우, 네트워크 연결을 확인하고 [API에 연결할 수 없음](#unable-to-connect-to-api)을 참조합니다.

<h2 id="request-errors">
  요청 오류
</h2>

이러한 오류는 요청의 내용과 관련이 있습니다. 대부분은 API가 요청을 거부한 후 반환되며, 일부는 요청이 전송되기 전에 Claude Code에서 로컬로 생성됩니다.

<h3 id="prompt-is-too-long">
  프롬프트가 너무 깁니다
</h3>

대화와 첨부된 파일이 모델의 컨텍스트 윈도우를 초과합니다.

```text theme={null}
Prompt is too long
```

대화형 세션에서 Claude Code는 이 오류를 다음과 같이 표시합니다:

```text theme={null}
Context limit reached · /compact or /clear to continue
```

[`DISABLE_COMPACT`](/docs/ko/env-vars)가 설정된 경우에만 줄에 `/clear`만 표시됩니다. 아래의 압축 실패 형식과 같은 더 긴 오류 형식은 `Prompt is too long ·` 표현을 유지합니다. `-p` 출력 및 기록에서 텍스트는 `Prompt is too long`으로 유지됩니다.

[사용자 설정](/docs/ko/settings-reference#autocompactenabled)에서 자동 압축을 끈 경우, 줄에도 다음과 같이 표시됩니다:

```text theme={null}
Context limit reached · /compact or /clear to continue · auto-compact is off · /config to turn it on
```

`/config`의 **Auto-compact** 토글은 사용자 설정에 `autoCompactEnabled`를 씁니다. 힌트는 `/config` 변경이 적용될 때만 나타납니다. 예를 들어, [`DISABLE_AUTO_COMPACT`](/docs/ko/env-vars) 또는 [`DISABLE_COMPACT`](/docs/ko/env-vars)가 자동 압축을 끈 경우에는 나타나지 않습니다. 또한 프로젝트 또는 관리 설정과 같은 더 높은 우선순위 범위가 `autoCompactEnabled`를 `false`로 설정한 경우에도 나타나지 않습니다. v2.1.235 이전에는 줄에 자동 압축 힌트가 없었습니다.

Amazon Bedrock은 이 조건을 `Input is too long for requested model.`로 보고하며, Claude Code는 동일한 방식으로 처리합니다. v2.1.217 이전에는 Claude Code가 Bedrock 표현을 인식하지 못했으므로 자동 압축이 트리거되지 않았고 `/compact`는 동일한 오류로 실패했습니다.

[Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway-config#upstream-error-messages)는 클라우드 업스트림이 제공자의 자체 오류 형식으로 요청을 거부할 때 이 조건을 `capability_rejected: prompt_too_long`으로 보고합니다. Claude Code는 토큰을 `Prompt is too long`과 동일하게 처리합니다. v2.1.228 이전에는 Claude Code가 토큰을 인식하지 못했으므로 자동 압축이 트리거되지 않았습니다.

자동 압축이 이 턴에서 실행되었지만 사용할 수 없는 모델 또는 인증 실패와 같은 기본 오류에서 실패한 경우, 메시지는 구분 기호 뒤에 해당 오류를 이름으로 지정합니다:

```text theme={null}
Prompt is too long · automatic compaction failed: <the underlying error>
```

명명된 오류를 먼저 해결하십시오. `/compact`는 해결할 때까지 동일한 오류로 실패합니다. v2.1.229 이전에는 실패한 자동 압축이 원인 없이 `Prompt is too long`을 표시했습니다.

자동 압축이 이 오류에서 실행될 때, 일반적으로 가장 오래된 교환을 요약하고 가장 최신의 것을 유지합니다. 최후의 수단으로 Claude Code는 다르게 요약합니다:

* 전체 교환을 요약할 수 없을 때, Claude Code는 가장 최신 프롬프트를 그대로 유지하고 그 이전의 모든 것을 요약합니다.
* 그 경우, 대화가 프롬프트로 끝나지 않으면 Claude Code는 전체 대화를 대신 요약합니다.

Claude Code는 전달할 내용이 모델 응답을 보유하지 않고 짧은 재시도와 같은 약 1,000개 토큰 미만의 자신의 텍스트를 보유할 때 이 복구를 건너뜁니다. `/clear`를 실행하여 새로 시작합니다. v2.1.269 이전에는 전체 교환을 요약할 수 없을 때마다 압축이 실패했으므로 이 상태의 세션은 모든 턴에서 이 오류를 다시 맞았습니다.

단일 교환 대화에는 요약할 이전 턴이 없습니다. 자동 압축이 하나에서 실행되었을 때, Claude Code는 시도를 건너뛰고 요청을 채우는 것을 설명합니다. API가 오류에서 토큰 수를 보고하지 않으면 메시지는 다음과 같이 읽힙니다:

```text theme={null}
Prompt is too long · this conversation is a single exchange and cannot be compacted — the request size comes mostly from system prompt, tool definitions, or attachments.
```

API가 오류에서 토큰 수를 보고하면 Claude Code는 이를 대화 크기의 자체 추정치와 비교하여 요청의 대부분을 차지하는 것이 무엇인지 알려줍니다: 대화 자체의 내용, 또는 Claude Code가 함께 보내는 시스템 프롬프트, 도구 정의 및 첨부 내용입니다. 대화 자체의 내용이 요청의 대부분인 경우 메시지는 다음과 같이 읽힙니다:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) and this conversation's own content is most of it. A single-exchange conversation cannot be compacted; start with less content (smaller files or pasted text).
```

요청의 대부분이 대화 외부에 있으면 메시지는 다음과 같이 읽힙니다:

```text theme={null}
Prompt is too long · the request is ~<request tokens> tokens (limit <limit>) but this conversation is only ~<conversation tokens> tokens — the rest is system prompt, tool definitions, and attachment content. A single-exchange conversation cannot be compacted; reduce attached files/tools or start with less context.
```

v2.1.162 이전에는 Claude Code가 압축을 시도했으며 실패했을 때 기본 `Prompt is too long`을 표시했습니다.

**할 일:**

* 다중 턴 대화에서 `/compact`를 실행하여 이전 턴을 요약하고 공간을 확보하거나, `/clear`를 실행하여 새로 시작합니다. `/compact`가 `Not enough messages to compact.`로 응답하면, 대화는 요약할 이전 내용이 없는 단일 교환이므로 공간은 해당 프롬프트와 Claude Code가 모든 요청과 함께 보내는 것으로 차지됩니다: `/clear`를 실행하고 붙여넣은 텍스트가 적거나 첨부 파일이 작은 상태로 다시 보내거나, 아래 단계를 사용하여 도구 정의 및 메모리 파일을 줄입니다
* `/context`를 실행하여 윈도우를 소비하는 것의 분석을 확인합니다: 시스템 프롬프트, 도구, 메모리 파일 및 메시지
* `/mcp disable <name>`으로 사용하지 않는 MCP 서버를 비활성화하여 컨텍스트에서 도구 정의를 제거합니다
* 큰 `CLAUDE.md` 메모리 파일을 정리하거나, 지침을 관련이 있을 때만 로드되는 [경로 범위 규칙](/docs/ko/memory#path-specific-rules)으로 이동합니다
* 서브에이전트는 부모 세션에서 모든 MCP 도구 정의를 상속하므로 첫 번째 턴 전에 컨텍스트 윈도우를 채울 수 있습니다. 서브에이전트를 생성하기 전에 사용하지 않는 MCP 서버를 비활성화합니다
* 자동 압축은 기본적으로 켜져 있으며 일반적으로 이 오류를 방지합니다. `/config`에서 또는 [`DISABLE_AUTO_COMPACT`](/docs/ko/env-vars)로 끈 경우 다시 켭니다. 끈 상태로 유지하면 윈도우가 채워지기 전에 `/compact`를 직접 실행합니다.

[컨텍스트 윈도우 탐색](/docs/ko/context-window)에서 컨텍스트가 어떻게 채워지는지에 대한 대화형 보기를 참조하십시오.

<h3 id="context-exceeds-the-token-limit">
  컨텍스트가 토큰 제한을 초과합니다
</h3>

`/context`는 대화가 모델의 컨텍스트 윈도우를 초과했을 때 출력 상단에 이 경고를 표시합니다. [`Prompt is too long`](#prompt-is-too-long)을 해제할 때까지 요청이 실패합니다. 대화형 세션은 해당 오류를 `Context limit reached` 줄로 표시합니다.

```text theme={null}
Context exceeds the 200k-token limit by 94k tokens — run /compact or /clear to continue.
```

초과한 제한이 1M 컨텍스트 모델의 200K 경계와 같은 모델의 컨텍스트 윈도우보다 작은 압축 윈도우인 경우, 경고는 다르게 읽힙니다. 압축 윈도우는 모델의 컨텍스트 윈도우 아래에 있을 수 있으므로 그것을 지난 요청은 여전히 성공할 수 있습니다.

```text theme={null}
Context is 94k tokens past the 200k-token compaction window — run /compact to reduce usage.
```

두 형식 모두 [`DISABLE_COMPACT`](/docs/ko/env-vars)를 설정한 경우 `/compact` 대신 `/clear`를 이름으로 지정합니다.

**할 일:**

* 다중 턴 대화에서 `/compact`를 실행하여 이전 턴을 요약하고 공간을 확보합니다. 대신 새로 시작하려면 `/clear`를 실행합니다
* 사용량을 줄이는 더 많은 방법은 [Prompt is too long](#prompt-is-too-long)을 참조하십시오

v2.1.216 이전에는 `/context`가 100% 이상의 사용량을 표시했으며 그것이 의미하는 바 또는 복구 방법을 설명하는 경고 줄이 없었습니다.

<h3 id="error-during-compaction-conversation-too-long">
  압축 중 오류: 대화가 너무 깁니다
</h3>

`/compact` 자체가 실패했습니다. 생성하는 요약을 보유할 충분한 여유 컨텍스트가 없기 때문입니다.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

이는 윈도우가 자동 압축이 트리거되는 순간 이미 가득 찬 경우 또는 [`Prompt is too long`](#prompt-is-too-long)을 본 후 `/compact`를 실행할 때 발생할 수 있습니다. 대화형 세션에서 해당 오류는 `Context limit reached` 줄입니다.

**할 일:**

* Esc를 두 번 눌러 메시지 목록을 열고 여러 턴을 뒤로 이동합니다. 이렇게 하면 컨텍스트에서 가장 최근 메시지가 제거됩니다. 그런 다음 `/compact`를 다시 실행합니다.
* 뒤로 이동해도 충분한 공간이 확보되지 않으면 `/clear`를 실행하여 새 세션을 시작합니다. 이전 대화는 보존되며 `/resume`으로 다시 열 수 있습니다.

이 메시지 및 기타 `/compact` 실패는 오류 스타일로 표시됩니다. v2.1.216 이전에는 성공한 명령 출력과 동일한 흐릿한 스타일로 렌더링되었으므로 실패한 압축을 성공으로 읽을 수 있었습니다.

<h3 id="request-too-large">
  요청이 너무 큽니다
</h3>

원본 요청 본문이 토큰화 전에 API의 32MB 제한을 초과했습니다. 일반적으로 큰 붙여넣은 내용, 도구 결과 또는 첨부 파일 때문입니다. 이 제한은 [컨텍스트 윈도우](#prompt-is-too-long)와 별개입니다.

```text theme={null}
Request too large (max 32MB). Accumulated images and attachments in the conversation pushed the request over the limit. Run /compact, or double press esc to go back and remove attachments.
```

요청이 Claude API로 직접 이동했고 API 자체가 거부한 경우, Claude Code는 대화를 측정하고 복구가 작동할 수 있는지 여부에 따라 메시지를 표현합니다. 프록시, 게이트웨이 또는 클라우드 제공자를 통해 일반 메시지를 받습니다. 측정된 형식:

* `Request too large (max 32MB; 20.1MB of about 33.4MB is images or documents).`: 이미지 또는 문서가 요청을 초과했습니다. Claude Code는 이를 제거하고 다시 시도합니다.
* `Request too large for the API's 32MB request limit`: 메시지 자체가 제한을 초과하므로 메시지는 `compacting cannot make it fit`이라고 말하고 Claude Code는 다시 시도하지 않습니다. [비대화형 모드](/docs/ko/headless)에서 메시지는 입력을 줄이거나 대신 새 세션을 시작하도록 알려줍니다.

v2.1.212 이전에는 충분한 누적 이미지가 있는 대화가 `Request too large (max 32MB). Double press esc to go back and try with a smaller file.`로 모든 턴에서 실패했습니다. v2.1.229 이전에는 Claude Code가 압축이 도움이 될 수 없을 때도 모든 거부에 대해 첨부 조언을 표시했습니다.

**할 일:**

* 메시지가 `compacting cannot make it fit`이라고 말하면 큰 내용을 추가한 턴을 지나 Esc를 두 번 눌러 뒤로 이동하거나 `/clear`를 실행하여 새로 시작합니다
* 그렇지 않으면 `/compact`를 실행하여 누적된 이미지 및 첨부 파일을 제거합니다
* 내용을 붙여넣는 대신 경로로 큰 파일을 참조하여 Claude가 청크 단위로 읽을 수 있도록 합니다
* 이미지의 경우 아래의 [Image was too large](#image-was-too-large)를 참조하십시오

<h3 id="image-was-too-large">
  이미지가 너무 큽니다
</h3>

붙여넣거나 첨부한 이미지가 API의 크기 또는 치수 제한을 초과합니다.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code는 처리할 수 없는 이미지를 텍스트 자리 표시자로 바꾸고 다시 시도하므로 후속 메시지가 성공합니다. 2.1.142 이전 버전에서는 붙여넣은 이미지가 대화에 남아 있을 수 있으며 후속 모든 메시지에서 동일한 오류를 반복합니다. 이러한 버전에서 복구하려면 Esc를 두 번 눌러 이미지가 추가된 턴을 지나 뒤로 이동합니다.

**할 일:**

* 붙여넣기 전에 이미지 크기를 조정합니다. API는 단일 이미지의 경우 가장 긴 가장자리에서 최대 8000픽셀을 허용하거나 많은 이미지가 컨텍스트에 있을 때 2000픽셀을 허용합니다.
* 전체 화면 대신 관련 영역의 더 타이트한 스크린샷을 찍습니다

<h3 id="unable-to-resize-image">
  이미지 크기를 조정할 수 없습니다
</h3>

Claude Code가 API로 보내기 전에 첨부된 이미지를 축소할 수 없었습니다.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
Unable to resize image — it is a CMYK JPEG, which Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Re-save it as an RGB PNG or JPEG and try again.
Unable to resize image — it is an animated WebP whose first frame Claude Code cannot decode, and at …px it is over the 2000x2000px limit, so it cannot be sent. Save its first frame as a PNG or JPEG and try again.
Unable to resize image — its pixels could not be decoded (the file may be damaged, or use an encoding Claude Code cannot read), and it is over the … API limit (… raw, … base64), so it cannot be sent. Re-save it as a PNG or JPEG and try again.
```

Claude Code는 일반적으로 큰 이미지를 자동으로 크기 조정합니다. 이러한 오류는 이미지를 디코딩하거나 API 제한 내에 맞게 크기 조정할 수 없음을 의미합니다.

**할 일:**

* 메시지가 이미지를 변환하도록 요청하면 PNG, JPEG, GIF 또는 WebP로 변환하고 다시 첨부합니다. Claude Code는 이미지를 디코딩하지 않고 파일 헤더에서 이러한 형식의 치수를 확인할 수 있습니다.
* 메시지가 치수 또는 크기 제한을 보고하면 해당 제한 아래로 이미지 크기를 조정하거나 다시 압축한 후 첨부합니다.
* 메시지가 CMYK JPEG, 애니메이션 WebP 또는 손상된 파일과 같은 원인을 이름으로 지정하면 메시지가 제안하는 형식으로 이미지를 다시 저장하고 첨부합니다.

<h3 id="pdf-errors">
  PDF 오류
</h3>

첨부한 PDF를 처리할 수 없었습니다. 메시지는 비대화형 형식으로 표시됩니다. 대화형 세션에서는 대신 Esc를 두 번 눌러 다시 시도하도록 요청합니다.

```text theme={null}
PDF too large (max 100 pages, 20MB). Try reading the file a different way (e.g., extract text with pdftotext).
PDF is password protected. Try using a CLI tool to extract or convert the PDF.
The PDF file was not valid. Try converting it to text first (e.g., pdftotext).
```

**할 일:**

* 크기가 큰 PDF의 경우 전체 파일을 첨부하는 대신 Read 도구로 Claude에게 페이지 범위를 읽도록 요청하거나 `pdftotext`와 같은 도구로 텍스트를 추출하고 출력 파일을 경로로 참조합니다
* 보호되거나 유효하지 않은 PDF의 경우 암호를 제거하거나 소스 애플리케이션에서 파일을 다시 내보낸 후 다시 시도합니다

<h3 id="extra-inputs-are-not-permitted">
  추가 입력은 허용되지 않습니다
</h3>

Claude Code와 API 사이의 프록시 또는 LLM 게이트웨이가 `anthropic-beta` 요청 헤더를 제거했으므로 API가 이에 따라 달라지는 필드를 거부했습니다.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code는 `context_management` 및 `effort`와 같은 베타 전용 필드를 이를 활성화하는 `anthropic-beta` 헤더와 함께 보냅니다. 게이트웨이가 본문을 전달하지만 헤더를 제거하면 API는 인식하지 못하는 필드를 봅니다.

**할 일:**

* `anthropic-beta` 헤더를 전달하도록 게이트웨이를 구성합니다. 게이트웨이가 전달해야 하는 것은 [기능 통과](/docs/ko/llm-gateway-protocol#feature-pass-through)를 참조하십시오.
* 대체로 시작하기 전에 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/ko/env-vars)을 설정합니다. [사전 릴리스 기능 비활성화](/docs/ko/llm-gateway-protocol#disable-pre-release-capabilities)는 정확한 범위를 다룹니다.

<h3 id="tool-input-schema-is-invalid">
  도구 입력 스키마가 유효하지 않습니다
</h3>

요청의 도구가 API의 JSON Schema 검증에 실패하는 `input_schema`를 선언했으므로 API가 전체 요청을 거부했습니다. `tools.` 뒤의 숫자는 요청의 도구 목록에서 실패한 도구의 위치이며, 조회할 수 있는 이름이 아닙니다.

```text theme={null}
API Error: 400 ... tools.N.custom.input_schema: JSON schema is invalid
API Error: 400 ... tools.N.custom.input_schema.properties: Property keys should match pattern '^[a-zA-Z0-9_.-]{1,64}$'
```

첫 번째 형식은 스키마가 유효한 JSON Schema draft 2020-12가 아님을 의미합니다. 두 번째는 최상위 속성 이름이 메시지가 인용하는 패턴과 일치하지 않음을 의미합니다.

Claude Code는 로드할 때 [이 검증에 실패할 입력 스키마가 있는 MCP 도구를 제외](/docs/ko/mcp#tools-with-invalid-input-schemas)하므로 요청은 일반적으로 하나를 포함하지 않습니다.

[플래그 가져오기가 꺼진 배포](/docs/ko/env-vars#features-that-need-feature-flag-fetching)에서 또는 플래그가 도착한 적이 없는 머신에서 Claude Code는 서버의 로그에 거부될 도구를 기록하지만 어쨌든 보내므로 이 오류가 여전히 발생할 수 있습니다.

오류는 또한 스키마가 `$schema`에서 draft 2020-12 이외의 JSON Schema 방언을 선언하는 도구에 대해 발생할 수 있습니다. Claude Code는 이러한 스키마를 JSON Schema 메타 스키마에 대해 확인하지 않지만 최상위 속성 이름 확인은 여전히 적용됩니다.

v2.1.216 이전에는 배포가 제외 확인을 실행하지 않았습니다.

**할 일:**

* Claude Code 버전이 v2.1.216보다 이전이면 `claude update`를 실행합니다.
* 유효하지 않은 스키마를 선언하는 MCP 서버를 제거하거나 [비활성화](/docs/ko/mcp#disable-a-server-without-removing-it)합니다. 오류는 도구를 위치로만 이름으로 지정합니다. v2.1.216 이상에서는 각 서버의 로그에서 입력 스키마가 거부될 도구를 이름으로 지정하는 줄을 확인합니다. 로그가 하나를 이름으로 지정하지 않으면 서버를 하나씩 비활성화합니다.
* 서버를 유지 관리하면 도구의 `input_schema`를 수정합니다. 스키마는 유효한 JSON Schema여야 하며 최상위 속성 이름은 1\~64자 길이여야 하고 ASCII 문자와 숫자, `_`, `.` 및 `-`만 사용해야 합니다. [유효하지 않은 입력 스키마가 있는 도구](/docs/ko/mcp#tools-with-invalid-input-schemas)를 참조하십시오.

<h3 id="theres-an-issue-with-the-selected-model">
  선택한 모델에 문제가 있습니다
</h3>

구성된 모델 이름이 인식되지 않았거나 계정에 액세스 권한이 없습니다. v2.1.160부터 뒤따르는 힌트는 표시 표면에 따라 다르며 여기에 대화형 형식으로 표시됩니다.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**할 일:**

* **대화형 CLI**: `/model`을 실행하여 계정에서 사용 가능한 모델 중에서 선택합니다.
* **비대화형 모드(`-p`)**: 유효한 별칭 또는 ID로 `--model`을 전달하거나 [`ANTHROPIC_MODEL`](/docs/ko/env-vars)을 설정합니다. 오류 텍스트는 이 표면에서 `Run --model`을 표시합니다.
* **Agent SDK**: 모델이 프로그래밍 방식으로 설정되므로 오류 텍스트는 힌트를 생략합니다. TypeScript에서 [`Options`의 `model`](/docs/ko/agent-sdk/typescript#options)을 설정하거나 Python에서 [`ClaudeAgentOptions(model=...)`](/docs/ko/agent-sdk/python#claudeagentoptions)을 설정하고 구조화된 `model_not_found` 오류를 처리하여 자신의 재시도 또는 모델 선택기를 표시합니다.
* `claude-...`의 전체 버전 ID 대신 `sonnet` 또는 `opus`와 같은 별칭을 사용합니다. 별칭은 유지 관리되는 기본값으로 확인되므로 오래되지 않습니다. [모델 구성](/docs/ko/model-config)을 참조하십시오.
* CLI에서 잘못된 모델이 계속 돌아오면 어딘가에 오래된 ID가 설정되어 있습니다. [우선순위 순서](/docs/ko/model-config#setting-your-model)로 모델을 설정할 수 있는 위치를 확인하고 오래된 값을 제거합니다.
* 새로 출시된 모델은 Anthropic API에서 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry가 제공하기 전에 사용 가능할 수 있습니다. 이러한 제공자 중 하나에서 새 모델 ID를 고정했고 이 오류가 표시되면 제공자의 모델 카탈로그에서 지역의 가용성을 확인하고 새 모델이 나타날 때까지 이전 버전을 고정된 상태로 유지합니다.
* Claude Code는 만료된 claude.ai 로그인을 [Login expired](#login-expired)로 보고하며, 이 오류로는 보고하지 않습니다. v2.1.206 이전에는 더 이상 새로 고칠 수 없는 만료된 로그인이 모든 모델에서 실패했습니다. 이전 버전에서 이를 보면 `/login`을 실행합니다.
* Google Cloud의 Agent Platform 배포의 경우 [Google Cloud의 Agent Platform 문제 해결](/docs/ko/google-vertex-ai#troubleshooting)을 참조하십시오.

<h3 id="model-is-not-a-recognized-model-id">
  모델이 인식된 모델 ID가 아닙니다
</h3>

모델 스위치에 전달한 모델 문자열이 모델 별칭, 이 Claude Code 버전이 알고 있는 모델 ID 또는 `claude-`로 시작하는 ID가 아닙니다. 일반적인 원인은 ID의 오타, `Sonnet 5`와 같은 표시 이름(ID `claude-sonnet-5` 필요) 또는 최신 Claude Code 버전만 인식하는 별칭입니다. Claude Code는 스위치를 즉시 거부합니다. v2.1.200 이전에는 Claude Code가 문자열을 저장했고 [선택한 모델에 문제가 있습니다](#theres-an-issue-with-the-selected-model)로 다음 요청에서 실패했습니다.

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

뒤따르는 힌트는 가장 가까운 일치하는 별칭 또는 모델 ID를 이름으로 지정합니다. 충분히 가까운 것이 없으면 `Run /model to see available models.`로 읽힙니다.

Claude Code는 API 요청이 이루어지기 전에 스위치가 요청되는 순간 로컬에서 이 오류를 생성합니다. [Agent SDK](/docs/ko/agent-sdk/typescript) `setModel()` 메서드를 통해 모델이 설정되거나 [Desktop 앱](/docs/ko/desktop)과 같은 앱이 Claude Code CLI를 실행하거나 [Remote Control](/docs/ko/remote-control)을 통해 연결된 장치에서 모델을 선택할 때 적용됩니다. v2.1.260 이전에는 확인이 Remote Control 선택을 다루지 않았으므로 Claude Code가 선택을 적용했고 다음 요청이 [선택한 모델에 문제가 있습니다](#theres-an-issue-with-the-selected-model)로 실패했습니다.

**할 일:**

* 인수 없이 `/model`을 실행하여 선택기를 열고 계정에서 사용 가능한 모델 중에서 선택한 다음 거기에 표시된 별칭 또는 ID를 전달합니다
* 최신 Claude Code 버전이 지원하는 별칭을 사용한 경우 `claude update`를 실행합니다. `claude-`로 시작하는 전체 ID는 모델이 Claude Code 버전보다 최신이어도 이 로컬 확인을 통과합니다. 서버는 여전히 해당 모델에 대한 최소 버전을 요구할 수 있습니다. [Claude Code가 이 모델을 지원하지 않습니다](#claude-code-does-not-support-this-model)를 참조하십시오.
* v2.1.200 이전에 저장된 모델은 이 확인으로 복구되지 않습니다. 오래된 값이 계속 돌아오면 [모델 설정](/docs/ko/model-config#setting-your-model) 아래에 나열된 위치에서 제거합니다.
* 확인은 Anthropic API에서만 실행됩니다. 사용자 정의 `ANTHROPIC_BASE_URL`을 포함한 다른 제공자 또는 게이트웨이에서 제공자는 모델 이름을 정의하므로 Claude Code는 모든 문자열을 수락하고 통과합니다. Claude Code는 여전히 요청 시간에 모든 제공자에서 [인식되지 않은 모델 진단 줄](#unrecognized-model-id-on-a-request)을 쓸 수 있습니다.

<h3 id="model-not-found">
  모델을 찾을 수 없습니다
</h3>

`/model <name>`으로 모델을 선택했고 Claude Code가 해당 이름의 모델이 존재하는지 확인할 수 없었습니다. 이름이 [모델 별칭](/docs/ko/model-config#model-aliases) 또는 Claude Code가 로컬에서 수락하는 다른 철자가 아닌 경우 `/model`은 최소 API 요청으로 확인하고 이 오류는 일반적으로 API 엔드포인트의 답변입니다. 공백을 포함하는 것과 같이 모델 ID가 될 수 없는 이름은 동일한 메시지를 받습니다.

```text theme={null}
Model 'claude-opus-9' not found
```

제공자별 모델 ID가 있는 제공자에서 메시지는 대체 모델에 대한 제공자의 ID를 이름으로 지정하는 `Try '...' instead` 제안을 추가할 수 있습니다.

**할 일:**

* 인수 없이 `/model`을 실행하고 계정에서 사용 가능한 모델 중에서 선택하거나 `sonnet`과 같은 [모델 별칭](/docs/ko/model-config#model-aliases)을 사용합니다. 이는 유지 관리되는 기본값으로 확인됩니다
* 전체 ID를 입력한 경우 제공자의 모델 카탈로그에 대해 확인합니다. 새로 출시된 모델은 제공자 또는 지역이 제공하기 전에 Anthropic API에서 사용 가능할 수 있습니다.
* v2.1.265 이전에는 `/model`이 `opusplan[1m]` 별칭 철자를 이 오류로 거부했습니다. 이러한 버전에서는 Claude Code를 업데이트하거나 [설정](/docs/ko/model-config#setting-your-model) 또는 `--model`에서 모델을 설정합니다.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus는 Claude Pro 플랜에서 사용할 수 없습니다
</h3>

활성 구독 플랜에 선택한 모델이 포함되지 않습니다.

```text theme={null}
Claude Opus is not available with the Claude Pro plan. If you have updated your subscription plan recently, run /logout and /login for the plan to take effect.
```

**할 일:**

* `/model`을 실행하고 플랜에 포함된 모델을 선택합니다
* 최근에 플랜을 업그레이드했는데도 여전히 이를 보면 `/logout`을 실행한 다음 `/login`을 실행합니다. 저장된 토큰은 로그인 시 플랜을 반영하므로 웹에서 업그레이드해도 기존 세션에서 다시 인증할 때까지 적용되지 않습니다.
* [claude.com/pricing](https://claude.com/pricing)에서 각 플랜에 포함된 모델을 참조하십시오

<h3 id="claude-code-does-not-support-this-model">
  Claude Code가 이 모델을 지원하지 않습니다
</h3>

API가 Claude Code 버전이 필요한 최소값 아래에 있기 때문에 400으로 요청을 거부했습니다. 선택한 모델이 최신 버전을 요구하거나(서버가 모델별로 확인) 조직의 정책이 하나를 요구합니다. 400은 오류 코드 `claude_code_version_too_old`를 전달하고 메시지는 어느 최소값이 적용되는지 말합니다.

```text theme={null}
API Error: 400 Claude Code 2.1.219 does not support this model; version 2.1.255 or newer is required. Run 'claude update', or update the Claude desktop app, then try again.
```

조직 정책 표현은 다음과 같이 읽힙니다:

```text theme={null}
API Error: 400 Claude Code 2.1.240 is older than the minimum version required by your organization's policy. Run 'claude update', or update the Claude desktop app, to continue.
```

**할 일:**

* `claude update`를 실행하거나 Claude 데스크톱 앱을 업데이트한 다음 새 세션을 시작합니다
* 모델별 표현의 경우 `/model`로 다른 모델로 전환하여 현재 세션에서 계속 작업할 수 있습니다
* 조직 정책 표현의 경우 계속하기 전에 업데이트합니다

<h3 id="model-is-restricted-by-your-organizations-settings">
  모델이 조직의 설정으로 제한됩니다
</h3>

조직 관리자가 claude.ai 관리 콘솔에서 이 모델을 비활성화했거나 관리 설정의 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 허용 목록으로 제외되었습니다. 제한된 모델이 `--model`, `ANTHROPIC_MODEL` 또는 `model` 설정으로 설정된 경우 Claude Code는 허용된 모델을 대체하고 계속합니다. 제한된 모델에 대해 `/model <name>`을 입력하면 `Run /model to choose a different model.`로 거부되고 세션은 현재 모델을 유지합니다. 대체 공지는 세션이 실행 중인 모델을 관리자가 claude.ai 관리 콘솔에서 비활성화한 후 세션 중간에 나타날 수도 있습니다.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

에이전트, 스킬 또는 명령 이름이 앞에 붙은 공지는 제한이 해당 [서브에이전트의 요청된 모델](/docs/ko/sub-agents#choose-a-model)에 적용되었음을 의미합니다: 서브에이전트는 대체 모델에서 실행되고 세션의 모델은 변경되지 않습니다. v2.1.223 이전에는 Claude Code가 Agent 도구로 시작된 서브에이전트에 대해서만 공지를 표시했습니다.

Claude Code는 모델 패밀리 별칭(`opus`, `sonnet`, `haiku` 또는 `fable` 중 하나)을 최신 버전에 대한 요청이 아닌 해당 패밀리에 대한 요청으로 취급합니다. Anthropic API 및 [Claude Platform on AWS](/docs/ko/claude-platform-on-aws)에서 제한된 패밀리 별칭은 조직과 `availableModels` 허용 목록이 허용하는 패밀리의 최신 버전으로 확인되고 대체 공지는 해당 버전을 이름으로 지정합니다. Claude Code는 패밀리의 모든 버전이 제한된 경우에만 `/model <alias>`를 거부합니다. v2.1.205 이전에는 패밀리 별칭이 같은 패밀리의 이전 버전이 허용된 경우에도 최신 버전만을 기반으로 대체되거나 거부되었습니다.

**할 일:**

* `/model`을 실행하여 조직이 허용하는 모델 중에서 선택합니다. 제한된 모델은 선택기에서 숨겨집니다.
* 제한된 모델이 `--model`, `ANTHROPIC_MODEL`, 설정 파일의 `model` 필드 또는 [서브에이전트](/docs/ko/sub-agents#choose-a-model), 스킬 또는 명령의 `model` 프론트매터에 설정된 경우 해당 값을 제거하거나 업데이트하여 공지가 다시 나타나지 않도록 합니다
* 제한된 모델에 액세스해야 하면 조직 관리자에게 활성화를 요청합니다. [조직 모델 제한](/docs/ko/model-config#organization-model-restrictions)을 참조하십시오.

<h3 id="model-switch-was-blocked-by-a-premodelswitch-hook">
  모델 스위치가 PreModelSwitch 훅으로 차단되었습니다
</h3>

[PreModelSwitch 훅](/docs/ko/hooks#premodelswitch)이 사용자 또는 클라이언트가 요청한 모델 스위치를 승인하지 않았으므로 세션은 현재 모델을 유지합니다. 스위치가 입력한 명령이 아닌 [Agent SDK](/docs/ko/agent-sdk/overview) 호스트 또는 [Remote Control](/docs/ko/remote-control)에서 온 경우 메시지는 대상 모델을 이름으로 지정하지 않고 `Model switch blocked by a PreModelSwitch hook`을 읽습니다.

```text theme={null}
Model switch to Opus 4.6 was blocked by a PreModelSwitch hook: Opus 4.6 is retired for this project. Use a newer model.
```

콜론 뒤의 이유는 스위치를 거부한 것을 말합니다:

* **훅이 작성한 이유**: PreModelSwitch 훅이 [스위치를 거부하거나 확인을 요청](/docs/ko/hooks#premodelswitch-decision-control)할 때 해당 이유를 제공했습니다. 요청하는 것을 해결하거나 훅이 허용하는 모델을 선택합니다.
* **`PreModelSwitch hook <name> did not respond before its timeout`**: [타임아웃](/docs/ko/hooks#timeouts) 전에 응답하지 않는 훅이 스위치를 차단합니다. 행(hung) 명령을 수정하거나 해당 훅의 `timeout`을 높인 다음 다시 전환합니다.
* **`confirmation required, and this session cannot ask`**: 훅이 이유 없이 `ask`로 응답했고 제어 요청이 확인 프롬프트를 표시할 방법이 없습니다. [`-p` 실행](/docs/ko/headless)의 모델 스위치는 이유 뒤에 `(run /model interactively to confirm)`으로 동일한 조건을 보고합니다. 대화형 세션에서 스위치를 만들거나 이 모델에 대한 훅의 결정을 변경합니다.
* **`so organization-managed PreModelSwitch hooks could not be checked`**: Claude Code가 조직의 [관리 플러그인](/docs/ko/settings-reference#enabledplugins)이 제공하는 PreModelSwitch 훅을 알 수 없었습니다. 예를 들어 관리 플러그인이 로드되지 않았기 때문입니다. 이러한 훅 중 하나가 스위치를 차단할 수 있으므로 Claude Code는 확인되지 않은 스위치를 적용하기보다는 거부합니다. 이유의 시작은 실패한 것을 이름으로 지정합니다. Claude Code는 모든 스위치 시도에서 다시 확인하므로 이후 지워진 실패는 차단을 중지합니다. 계속 실패하면 `claude --debug`를 실행하고 다시 전환하여 세부 정보를 캡처한 다음 플러그인을 수정하거나 관리자에게 수정을 요청합니다.
* **`a PreModelSwitch hook failed before answering`** 또는 **`PreModelSwitch hooks were cancelled (the control stream closed) before answering`**: 훅 실행이 평결 없이 종료되었고 Claude Code는 이를 승인으로 취급하지 않습니다. `claude --debug`를 실행하여 실패한 것을 확인한 다음 다시 전환합니다.

v2.1.260 이전에는 관리 플러그인 거부가 `plugin hooks could not be loaded, so PreModelSwitch hooks could not be checked; see the debug log`로 읽혔습니다. Claude Code는 플러그인 로드를 한 번 재시도한 다음 조직이 플러그인을 관리하지 않은 경우에도 세션의 이후 스위치를 거부했습니다. 이러한 버전에서 플러그인 로드를 다시 실행하려면 세션을 다시 시작합니다.

<h3 id="couldnt-save-it-as-your-default">
  기본값으로 저장할 수 없습니다
</h3>

모델을 선택하여 기본값으로 저장했습니다. 예를 들어 `/model <name>` 또는 `/model` 선택기의 `Enter`를 사용하고 Claude Code가 사용자 설정 파일 `~/.claude/settings.json`에 선택을 쓸 수 없었습니다. 스위치 자체가 적용되었으므로 현재 세션은 선택한 모델에서 실행되지만 기본값은 변경되지 않으며 다음 세션은 이전 값에서 시작됩니다.

```text theme={null}
Set model to Fable 5.1 for this session only · couldn't save it as your default: ~/.claude/settings.json can't be written (EROFS)
```

파일 경로 뒤의 이유는 실패한 것을 말합니다:

* **`can't be written (<code>)`**: 쓰기가 괄호의 `EROFS`와 같은 운영 체제 오류 코드로 실패했습니다. 파일이 링크하는 파일이 쓰기를 거부하는 파일 시스템에 있을 때입니다. 파일을 쓰기 가능하게 만들고 다시 전환합니다. 다른 도구가 파일을 생성하면 해당 도구에서 `model` 키를 설정합니다. [Claude Code에서 만든 변경 사항이 새 세션에서 손실됨](/docs/ko/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions)을 참조하십시오.
* **`isn't valid JSON`**: 디스크의 파일이 구문 분석되지 않으며 Claude Code는 읽을 수 없는 내용을 덮어쓰기보다는 그대로 둡니다. 구문 오류를 수정한 다음 다시 전환합니다. [손상된 설정 파일 수정](/docs/ko/settings#fix-a-broken-settings-file)을 참조하십시오.

`couldn't confirm it was saved as your default (~/.claude/settings.json is still being written)`로 끝나는 공지는 3초 후에 쓰기가 완료되지 않았음을 의미합니다. 백그라운드에서 계속되므로 기본값이 여전히 저장될 수 있습니다. 다음 세션이 시작되는 모델을 확인하거나 `/model <name>`을 다시 실행합니다.

v2.1.265 이전에는 공지가 쓰기가 실패했을 때도 모델이 `saved as your default for new sessions`이라고 말했습니다.

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled는 이 모델에서 지원되지 않습니다
</h3>

Claude Code 버전이 선택한 모델에 필요한 최소값보다 오래되었습니다. CLI가 모델이 더 이상 수락하지 않는 사고 구성을 보냈습니다.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**할 일:**

* `claude update`를 실행하고 Claude Code를 다시 시작합니다. Opus 4.7은 v2.1.111 이상이 필요합니다. Opus 4.8은 v2.1.154 이상이 필요합니다. Sonnet 5는 v2.1.197 이상이 필요합니다. Opus 5는 v2.1.219 이상이 필요합니다. Opus 5.5는 v2.1.280 이상이 필요합니다
* 업그레이드할 수 없으면 `/model`을 실행하고 대신 Opus 4.6 또는 Sonnet 4.6을 선택합니다
* [Agent SDK](/docs/ko/agent-sdk/overview)에서 이를 맞으면 SDK 패키지를 대신 업그레이드합니다. Opus 4.8은 TypeScript SDK v0.3.154 이상 및 Python SDK v0.2.88 이상이 필요합니다. Sonnet 5는 TypeScript SDK v0.3.197 이상이 필요합니다. Opus 5는 TypeScript SDK v0.3.219 이상이 필요합니다. Opus 5.5는 TypeScript SDK v0.3.280 이상이 필요합니다

<h3 id="effort-isnt-available-with-thinking-turned-off">
  사고가 꺼져 있을 때 노력을 사용할 수 없습니다
</h3>

[확장 사고](/docs/ko/model-config#extended-thinking)를 끄고 `high` 이상의 [노력 수준](/docs/ko/model-config#adjust-effort-level)에서 실행했습니다. 모델이 해당 조합을 수락하지 않으므로 API가 요청을 거부했습니다.

```text theme={null}
API Error: Effort 'xhigh' isn't available with thinking turned off on this model · run /effort high to continue, or turn thinking back on (unset MAX_THINKING_TOKENS=0)
```

**할 일:**

* [노력 수준을 낮춥니다](/docs/ko/model-config#set-the-effort-level) `high` 이하로.
* 사고를 다시 켭니다. 예를 들어 [`MAX_THINKING_TOKENS`](/docs/ko/env-vars)을 설정 해제하거나 설정에서 [`"alwaysThinkingEnabled": false`](/docs/ko/settings-reference#alwaysthinkingenabled)를 제거합니다.

v2.1.242 이전에는 Claude Code가 API의 자체 메시지를 표시했습니다: `API Error: 400 output_config.effort 'xhigh' is not supported when thinking is disabled on this model. Use effort 'high' or below, or enable thinking.` v2.1.251 이전에는 Claude Code가 설정한 노력 수준에서 요청을 보냈으므로 Opus 5는 사고가 꺼져 있을 때 `high` 이상의 모든 요청을 거부했습니다. Claude Code는 이제 Opus 5와 같이 조합을 거부하는 것으로 알고 있는 모델에 노력 `high`를 보내므로 v2.1.251 이상에서 이 오류는 Claude Code가 거부하는 것을 알지 못하는 모델에서만 도달합니다.

<h3 id="thinking-budget-exceeds-output-limit">
  사고 예산이 출력 제한을 초과합니다
</h3>

구성된 확장 사고 예산이 최대 응답 길이를 초과하므로 실제 답변을 위한 공간이 남지 않습니다.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code는 Anthropic API에서 이러한 값을 자동으로 조정합니다. 일반적으로 [`MAX_THINKING_TOKENS`](/docs/ko/env-vars)이 제공자의 출력 제한보다 높게 설정되었거나 계획 모드가 사고 예산을 높일 때 Amazon Bedrock 또는 Google Cloud의 Agent Platform에서 이 오류를 봅니다.

**할 일:**

* `MAX_THINKING_TOKENS`를 낮추거나 [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/ko/env-vars)를 사고 예산 이상으로 높입니다
* [확장 사고](/docs/ko/model-config#extended-thinking)에서 예산이 출력 길이와 상호 작용하는 방식을 참조하십시오

<h3 id="tool-use-or-thinking-block-mismatch">
  도구 사용 또는 사고 블록 불일치
</h3>

대화 기록이 일관성 없는 상태로 API에 도달했습니다. 일반적으로 도구 호출이 중단되거나 턴이 스트림 중간에 편집된 후입니다.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 orphaned tool_result in conversation history. Run /rewind to recover the conversation.
API Error: 400 duplicate tool_use ID in conversation history. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

모든 변형은 동일한 의미입니다: 기록의 `tool_use`, `tool_result` 및 `thinking` 블록의 순서가 더 이상 API가 예상하는 것과 일치하지 않습니다.

**할 일:**

* Opus 4.7 또는 Opus 4.8을 사용하는 경우 먼저 `claude update`를 실행합니다. v2.1.156 이전 버전은 정상적인 도구 사용 중에 이 오류를 트리거할 수 있으며 `/rewind`는 이를 지우지 않습니다.
* `/rewind`를 실행하거나 Esc를 두 번 눌러 손상된 턴 전의 체크포인트로 뒤로 이동하고 거기서 계속합니다. [체크포인팅](/docs/ko/checkpointing)에서 체크포인트가 생성되고 복원되는 방식을 참조하십시오.

<h3 id="unsupported-tool-content-removed">
  지원되지 않는 도구 내용이 제거되었습니다
</h3>

Claude Code가 Anthropic API에 직접 연결되고 저장된 세션을 로드하거나 미리 볼 때 Anthropic API가 수락하지 않는 도구 내용을 제거하고 두 사고 블록 사이에 제거된 내용이 있던 위치에 이 줄을 남깁니다:

```text theme={null}
[Unsupported tool content removed]
```

이러한 내용은 일반적으로 [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)을 통해 설정된 타사 프록시가 다른 제공자의 도구 호출을 API 형식으로 변환할 때 API 형식으로 세션 파일에 도달합니다. Claude Code는 세션이 Anthropic API에 직접 연결될 때만 제거하고 세션이 프록시를 통해 또는 다른 제공자에서 실행될 때 저장된 기록을 있는 그대로 로드합니다. v2.1.246 이전에는 Claude Code가 도구 사용 및 그 결과를 API로 다시 보냈고 재개된 세션의 모든 턴이 `messages.1.content.0.server_tool_use.name: Input should be 'web_search', 'web_fetch', ...`와 같은 400 오류로 실패했습니다.

**할 일:**

* 자리 표시자 줄을 볼 때 아무것도 필요하지 않습니다. 세션은 제거된 내용 없이 계속됩니다.
* 재개된 세션의 모든 턴이 대신 400 오류로 실패하면 `claude update`를 실행하고 세션을 다시 재개합니다. v2.1.246 이전 버전은 내용을 제거하지 않습니다.

<h3 id="role-system-must-precede-an-assistant-message">
  role 'system' must precede an 'assistant' message
</h3>

API가 대화에서 수락하지 않는 위치에 시스템 메시지가 있기 때문에 400으로 요청을 거부했습니다:

```text theme={null}
API Error: 400 messages.6: role 'system' must precede an 'assistant' message or end the array; ...
```

Claude Code는 일부 미리 알림 및 첨부 텍스트를 대화 내 시스템 메시지로 보냅니다. API가 하나의 위치를 거부하면 Claude Code는 요청을 한 번 다시 시도하고 해당 텍스트를 일반 사용자 메시지로 대신 보냅니다. `top-level 'system' parameter for the initial system prompt` 사용과 같은 API의 형제 배치 표현은 동일한 복구를 받습니다.

오류가 나타나면 거부된 시스템 메시지는 Claude Code가 제거할 수 있는 것이 아닙니다. 이는 일반적으로 Claude Code와 API 사이의 프록시 또는 [LLM 게이트웨이](/docs/ko/llm-gateway)가 시스템 메시지를 추가했거나 대화를 재정렬했음을 의미합니다.

**할 일:**

* `/clear`를 실행하여 새 대화를 시작합니다. 오류가 거기서도 돌아오면 원인은 저장된 대화가 아닌 요청 경로에 있습니다.
* [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)을 통해 구성된 프록시 또는 게이트웨이 뒤에서 오류가 모든 턴에서 반복되면 프록시 없이 연결하여 원인을 확인하고 오류를 운영하는 사람에게 보고합니다

v2.1.280 이전에는 Claude Code가 이 표현을 인식하지 못했으므로 거부된 시스템 메시지가 Claude Code 자체가 보낸 것일 때도 오류가 나타났고 대화의 모든 이후 턴이 동일한 방식으로 실패했습니다.

<h3 id="invalid-encrypted-content-in-search-result-block">
  Invalid encrypted\_content in search\_result block
</h3>

API가 대화 기록이 해독할 수 없는 호스팅된 웹 검색 내용을 보유하고 있기 때문에 400으로 요청을 거부했습니다. 표현은 읽을 수 없는 필드를 이름으로 지정합니다:

```text theme={null}
API Error: 400 messages.21.content.0: Invalid `encrypted_content` in `search_result` block
API Error: 400 messages.21.content.3.citations.0: Invalid `encrypted_index` in `text` block
API Error: 400 Failed to decrypt web search result content
```

API의 호스팅된 [웹 검색 도구](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool)의 결과는 API만 읽을 수 있는 암호화된 필드를 전달합니다. API는 다른 조직을 위해 생성된 내용과 같이 해독할 수 없는 내용을 재생하는 요청을 거부합니다.

Claude Code의 자체 [WebSearch 도구](/docs/ko/tools-reference#websearch-tool-behavior)는 검색 결과를 일반 텍스트로 기록하므로 이러한 블록은 일반적으로 호스팅된 웹 검색을 자체적으로 실행한 프록시 또는 [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 대화에 도달합니다.

거부된 블록은 대화 기록에 남아 있으므로 모든 이후 턴과 `/compact`는 동일한 방식으로 실패합니다.

**할 일:**

* `/clear`를 실행하거나 새 세션을 시작합니다. 새 대화는 거부된 블록을 전달하지 않습니다
* Claude Code를 프록시 또는 게이트웨이 뒤에서 실행하면 오류를 운영하는 사람에게 보고합니다

<h3 id="usage-policy-refusal">
  사용 정책 거부
</h3>

API가 대화의 내용이 [사용 정책](https://www.anthropic.com/legal/aup) 확인을 트리거했기 때문에 응답을 거부했습니다. 메시지에는 거부가 잘못되었다고 생각하면 지원팀에 인용할 수 있는 요청 ID가 포함됩니다.

```text theme={null}
API Error: Opus 4.6 can't help with this. Start a new session to continue.

Send feedback with /feedback or learn more: https://www.anthropic.com/legal/aup
```

메시지는 거부한 모델을 이름으로 지정하거나 모델이 기록되지 않으면 `Claude`를 이름으로 지정합니다.

확인은 최신 프롬프트뿐만 아니라 전체 대화를 평가하므로 동일한 세션에서 새 메시지를 보내면 일반적으로 동일한 거부를 다시 트리거합니다. 동일한 내용이 디스크의 기록에 여전히 포함되어 있으므로 세션을 종료하고 `--continue` 또는 `--resume`으로 다시 열 때도 적용됩니다. [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 및 [Microsoft Foundry](/docs/ko/microsoft-foundry)에서 이 메시지는 또한 모델의 안전 조치가 사이버 보안 주제로 플래그한 요청을 다룹니다. [안전 조치가 사이버 보안 주제를 플래그했습니다](#safety-measures-flagged-a-cybersecurity-topic)를 참조하십시오.

v2.1.219 이전에는 메시지가 `Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.`로 읽혔습니다.

**할 일:**

* Esc를 두 번 누르거나 `/rewind`를 실행하여 거부를 트리거한 턴 전의 체크포인트로 뒤로 이동한 다음 다시 표현하거나 다른 접근 방식을 취합니다. [체크포인팅](/docs/ko/checkpointing)을 참조하십시오.
* 어느 턴이 원인인지 식별할 수 없으면 `/clear`를 실행하여 동일한 프로젝트에서 새 대화를 시작합니다. 이전 대화는 디스크에 보존되며 `/resume`에서 사용 가능합니다.
* [비대화형 모드](/docs/ko/headless)(`-p`)에서 되감기를 사용할 수 없으므로 `--continue` 없이 새 세션에서 다시 표현된 프롬프트로 다시 시도합니다. 정책 확인은 모델에 따라 다르므로 `--model`로 다른 모델로 전환하면 일부 경우에 거부를 해결할 수도 있습니다.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  안전 조치가 사이버 보안 주제를 플래그했습니다
</h3>

모델의 안전 조치가 대화의 내용을 사이버 보안 주제로 플래그했습니다. 메시지는 요청을 플래그한 모델을 이름으로 지정합니다:

```text theme={null}
API Error: Opus 4.8's safeguards flagged this message. Our intentionally broad safeguards allow us to deliver more capabilities faster, but can sometimes flag legitimate cybersecurity work. Apply to the Cyber Verification Program to reduce these interruptions. Send feedback with /feedback or learn more: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude
```

메시지는 [사이버 검증 프로그램](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)에 연결되며, 이는 합법적인 사이버 보안 작업에 대한 액세스를 부여합니다. Opus 5.5에서는 v2.1.280 이상이 필요하며, 메시지는 `Opus 5.5's safeguards flagged this session` 대신 시작됩니다. 플래그된 카테고리에 대체 모델을 사용할 수 있으면 Claude Code는 이 오류를 표시하는 대신 [모델을 전환](/docs/ko/model-config#automatic-model-fallback)합니다.

[Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 및 [Microsoft Foundry](/docs/ko/microsoft-foundry)에서 사이버 보안 플래그는 대신 [사용 정책 거부](#usage-policy-refusal) 메시지를 생성합니다.

안전 조치 자체는 서버 측이며 v2.1.203보다 앞서 있습니다. 이후 클라이언트 릴리스는 메시지의 표현만 변경했습니다.
v2.1.203부터 v2.1.218까지 메시지는 `<model> has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center:`로 읽혔고 동일한 도움말 센터 링크가 뒤따랐으며 대화형 세션은 `If you were not engaging in a cybersecurity topic, please send feedback via /feedback.`를 추가했습니다.
v2.1.203 이전에는 `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:`로 읽혔고 면제 양식 링크가 뒤따랐습니다.

**할 일:**

* 작업에 이 내용이 필요하면 [사이버 검증 프로그램](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)을 통해 액세스를 신청합니다
* 요청이 사이버 보안 주제가 아닌 경우 `/feedback`을 실행하여 거짓 양성을 보고합니다
* 동일한 세션에서 계속 작업하려면 Esc를 두 번 누르거나 `/rewind`를 실행하여 플래그를 트리거한 턴 전의 체크포인트로 뒤로 이동한 다음 다른 접근 방식을 취합니다. [체크포인팅](/docs/ko/checkpointing)을 참조하십시오.

<h2 id="installation-errors">
  설치 오류
</h2>

이러한 오류는 Claude Code를 설치하거나 업데이트할 때 [설치 스크립트](/docs/ko/setup#install-claude-code), `claude install` 또는 `claude update`에서 나타납니다. 설정 중 `command not found`, PATH, 권한 및 TLS 문제의 경우 [설치 및 로그인 문제 해결](/docs/ko/troubleshoot-install)을 참조하십시오.

<h3 id="installation-was-killed-before-it-could-finish">
  설치가 완료되기 전에 중단되었습니다
</h3>

설치 스크립트는 `claude install` 단계가 신호에 의해 종료될 때 보고합니다. Linux에서 종료 코드 137은 프로세스가 SIGKILL을 수신했음을 의미하며, 메모리가 부족한 호스트에서는 일반적으로 커널 메모리 부족(OOM) 킬러입니다. 스크립트는 이 설명을 출력하고 코드 137로 종료됩니다:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

다른 치명적 신호의 경우, 그리고 macOS의 종료 코드 137의 경우, 스크립트는 `Installation was killed before it could finish (exit code <N>)`을 출력하며 실제 종료 코드를 포함하고 메모리 부족 설명을 생략합니다. 메시지는 macOS 및 Linux가 사용하는 설치 스크립트에서 나오며, WSL 내부의 설치도 포함합니다. 네이티브 Windows 설치 스크립트는 절대 이를 출력하지 않습니다. v2.1.200 이전에는 스크립트가 셸의 단순한 `Killed` 줄로만 종료되었습니다.

**수행할 작업:**

* 다른 프로세스를 중지하여 메모리를 확보한 후 설치 프로그램을 다시 실행합니다
* 스왑 공간을 추가하거나 더 큰 인스턴스로 이동합니다. 스왑 파일 명령은 [메모리 부족 Linux 서버에서 설치 중단됨](/docs/ko/troubleshoot-install#install-killed-on-low-memory-linux-servers)을 참조하십시오.

<h3 id="the-connection-dropped-while-downloading-the-update">
  업데이트를 다운로드하는 동안 연결이 끊어졌습니다
</h3>

`claude install`, `claude update` 또는 [자동 업데이터](/docs/ko/setup#auto-updates)가 Claude Code 바이너리를 가져오는 동안 다운로드 서버로의 연결이 끊어졌으며, 재시도로 복구되지 않았습니다. Claude Code는 연결이 끊어지거나, 전송이 중단되거나, 다운로드된 파일이 체크섬에 실패할 때 다운로드를 재시도하며, 총 3번까지 시도합니다. 404와 같은 완료된 HTTP 오류는 서버가 이미 응답했기 때문에 재시도되지 않습니다. v2.1.202 이전에는 단일 연결 끊김이 재시도 대신 단순한 오류 `aborted`로 다운로드를 즉시 실패했습니다.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

괄호의 텍스트는 어느 시도가 실패했는지와 기본 네트워크 오류를 나타냅니다. `claude update`는 stderr에서 메시지 앞에 `Error: Failed to install native update`를 붙입니다.

연결된 상태이지만 10분 이내에 완료되지 않는 다운로드는 `Download timed out: exceeded the total deadline` 메시지로 실패합니다. Claude Code는 시간 초과된 다운로드를 재시도하지 않습니다. 왜냐하면 기한 내에 완료할 수 없을 정도로 느린 연결은 즉시 재시도에서도 완료되지 않기 때문입니다. 아래 단계는 두 메시지 모두에 적용됩니다.

일반적인 원인은 긴 전송을 완료하기 전에 닫는 프록시 또는 게이트웨이입니다. Claude Code 바이너리는 큰 다운로드이므로, 일반 API 트래픽에는 영향을 주지 않는 프록시 연결 제한이 여전히 이를 중단할 수 있습니다.

**수행할 작업:**

* `claude update`를 다시 실행합니다. 정상적인 네트워크에서는 다운로드가 일반적으로 다음 실행에서 성공합니다. 시간 초과 메시지의 경우 더 빠르거나 제한이 적은 네트워크에서 다시 실행합니다.
* 네트워크에 프록시가 필요한 경우 설치 프로그램 또는 `claude update`를 실행하기 전에 `HTTPS_PROXY`를 설정합니다. [네트워크 연결 확인](/docs/ko/troubleshoot-install#check-network-connectivity)을 참조하십시오.
* 회사 프록시가 계속 전송을 닫는 경우 네트워크 팀에 `downloads.claude.ai`에서 전체 다운로드를 허용하도록 요청합니다. [네트워크 액세스 요구 사항](/docs/ko/network-config#network-access-requirements)을 참조하십시오.
* 설치 진단을 위해 셸에서 `claude doctor`를 실행합니다

<h2 id="command-line-errors">
  명령줄 오류
</h2>

이러한 오류는 `claude` 명령줄과 그 하위 명령어에서 발생하며, 프롬프트에서 제출한 명령어 이름에서도 발생합니다. 또한 셸 명령어를 실행하여 컨텍스트를 수집한 후 프롬프트를 실행하는 `/security-review` 같은 명령어에서도 발생합니다. 이들은 CLI를 다시 시작하는 `/tui`에서도 발생합니다.

<h3 id="conflict-between-bg-and-print">
  \--bg와 --print 간의 충돌
</h3>

이 메시지는 Claude Code v2.1.198 이상이 필요합니다. 동일한 `claude` 호출에서 `--bg`를 `-p` 또는 `--print`와 결합했습니다. `--bg`는 나중에 `claude agents`로 연결할 수 있는 [백그라운드 세션](/docs/ko/agent-view#from-your-shell)을 시작하는 반면, `--print`는 [비대화형](/docs/ko/headless)으로 실행되며 `claude agents`가 연결할 수 있는 대화형 세션을 시작하지 않습니다. v2.1.198 이전에는 이 조합이 연결할 수 없는 백그라운드 작업을 조용히 생성했습니다.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**해야 할 일:**

* `-p` 또는 `--print`를 제거하세요. `--bg`는 프롬프트를 위치 인수로 사용하므로 `claude --bg "<task>"`가 완전한 명령어입니다. [셸에서 새 에이전트 디스패치](/docs/ko/agent-view#from-your-shell)를 참조하세요.
* 프롬프트를 비대화형으로 실행하고 백그라운드 세션을 생성하는 대신 결과를 인쇄하려면 `--bg`를 제거하고 `claude -p "<task>"`를 실행하세요.

<h3 id="invalid-agents-configuration">
  잘못된 --agents 구성
</h3>

`--agents`에 전달한 값이 유효하지 않아서 `claude`가 세션을 시작하는 대신 코드 1로 종료됩니다. `--safe-mode`, `--resume`, 또는 `--continue`를 전달하거나 [`CLAUDE_CODE_SAFE_MODE`](/docs/ko/env-vars#variables)를 설정하면 Claude Code는 값을 확인하지 않고 세션을 시작합니다. v2.1.242 이전에는 Claude Code가 어쨌든 세션을 시작했고 로드할 수 없는 정의를 생략했습니다.

```text theme={null}
Error: Invalid --agents configuration:
<what failed>
```

첫 번째 줄 다음에 오는 내용은 값이 어떻게 실패했는지에 따라 다릅니다. Claude Code는 이러한 확인을 순서대로 실행하고 실패하는 첫 번째 확인에서 중지합니다. 값에 두 가지 문제가 있으면 첫 번째를 수정한 후에만 두 번째를 볼 수 있습니다:

1. 값이 JSON으로 파싱되지 않으면 Claude Code는 JSON 파서의 메시지를 포함하는 `invalid JSON:` 줄 하나를 인쇄합니다.
2. 파싱되지만 에이전트 정의가 [CLI 정의 하위 에이전트](/docs/ko/sub-agents#choose-the-subagent-scope)의 스키마와 일치하지 않으면 Claude Code는 문제당 한 줄을 인쇄합니다.
3. 에이전트 이름이 `-`로 시작하면 Claude Code는 `<name>: agent names must not start with '-'`를 인쇄합니다.

문제 줄이 20개를 초과하면 Claude Code는 처음 20개를 인쇄하고 나머지를 `…and N more`로 바꿉니다.

**해야 할 일:**

* 메시지가 나열한 각 문제를 수정한 후 명령어를 다시 실행하세요. [CLI 정의 하위 에이전트가 사용하는 필드](/docs/ko/sub-agents#choose-the-subagent-scope)를 참조하세요.

<h3 id="cloud-sessions-cannot-be-created-from-a-restricted-session">
  클라우드 세션을 --restricted 세션에서 생성할 수 없음
</h3>

[`--restricted`](/docs/ko/cli-reference#cli-flags)로 세션을 시작하면 Claude Code는 이 세션에서 [클라우드 세션](/docs/ko/claude-code-on-the-web#from-terminal-to-cloud)을 생성하기를 거부합니다. 새 세션이 제한된 프로세스 외부에서 실행되고 제한된 모드를 적용하지 않기 때문입니다. Claude Code는 서버에 연결하기 전에 클라이언트에서 거부하므로 클라우드 세션이 생성되지 않습니다:

```text theme={null}
Cloud sessions cannot be created from a --restricted session: they would not enforce it.
```

**해야 할 일:**

* 제한된 세션에서 로컬로 작업을 실행하세요.
* 세션이 어떻게 시작되었는지 제어할 수 있으면 `--restricted` 없이 새 `claude` 세션을 시작하고 거기서 클라우드 세션을 생성하세요.

v2.1.248 이전에는 Claude Code에 `--restricted` 플래그가 없었으며, 이전 버전은 알 수 없는 옵션 오류로 플래그 자체를 거부합니다.

<h3 id="cloud-sessions-are-disabled-by-your-organizations-policy">
  조직의 정책에 의해 클라우드 세션이 비활성화됨
</h3>

조직의 `allow_remote_sessions` 정책이 꺼져 있어서 [클라우드 세션](/docs/ko/claude-code-on-the-web)과 이를 사용하는 명령어를 사용할 수 없습니다:

```text theme={null}
Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.
```

이 메시지는 [터미널에서 클라우드 세션을 생성](/docs/ko/claude-code-on-the-web#from-terminal-to-cloud)할 때 나타나며 `/teleport`, `/remote-env`, 또는 `/web-setup` 같은 클라우드 세션이 필요한 명령어를 제출할 때도 나타납니다. v2.1.268 이전에는 이러한 명령어 중 하나를 제출하면 대신 [`Unknown command`](#unknown-command)를 반환했습니다.

이것은 서버 측 조직 정책이므로 로컬 설정, 환경 변수 또는 CLI 플래그에서 재정의할 수 없습니다.

Claude Code가 조직의 정책을 아직 로드하지 않았거나 가져올 수 없으면 이러한 명령어는 대신 `Couldn't verify your organization's policy for cloud sessions. Check your network connection, then restart Claude Code and try again.`으로 응답합니다.

**해야 할 일:**

* 조직의 [Owner](/docs/ko/server-managed-settings#access-control)에게 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)의 Claude Code 관리자 설정에서 클라우드 세션을 활성화하도록 요청하세요.
* 메시지가 정책을 확인할 수 없다고 하면 네트워크 연결을 확인한 후 Claude Code를 다시 시작하고 다시 시도하세요.

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  \--json-schema 값이 유효한 JSON Schema가 아님
</h3>

[비대화형 모드](/docs/ko/headless#get-structured-output)에서 [`--json-schema`](/docs/ko/cli-reference#cli-flags)에 전달한 스키마가 JSON Schema 컴파일에 실패했으므로 `claude`가 프롬프트를 실행하는 대신 코드 1로 종료됩니다. v2.1.205 이전에는 유효하지 않은 스키마가 오류 없이 구조화되지 않은 출력을 생성했으며, `format` 키워드를 사용한 모든 스키마는 유효하지 않은 것으로 처리되었습니다.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

두 번째 콜론 뒤의 텍스트는 검증자의 진단이며 실패한 키워드 또는 위치를 이름 지정합니다. `"format": "email"` 같은 `format` 키워드를 사용하는 스키마는 유효합니다: Claude Code는 `format`을 주석으로 수용하고 적용하지 않습니다.

Claude Code는 스키마 컴파일 전에 두 가지 확인을 실행합니다: 파싱 불가능한 JSON 값을 `Error: --json-schema is not valid JSON`으로 거부하고, 객체가 아닌 유효한 JSON을 `Error: --json-schema must be a JSON object`로 거부합니다.

**해야 할 일:**

* 진단이 이름 지정한 스키마 부분을 수정한 후 명령어를 다시 실행하세요.
* 진단이 `schema too large`이면 스키마의 중첩과 `$ref` 재사용을 줄이세요.
* [구조화된 출력 가져오기](/docs/ko/headless#get-structured-output)에서 작동하는 스키마와 명령어를 참조하세요.

<h3 id="settings-file-exceeds-the-2mib-limit">
  설정 파일이 2MiB 제한을 초과함
</h3>

[`--settings`](/docs/ko/cli-reference#cli-flags)에 전달한 파일이 2MiB보다 크므로 `claude`가 로드하는 대신 시작 시 코드 1로 종료됩니다. 설정 파일은 작은 JSON 문서이므로 이 크기의 파일은 보통 경로가 잘못된 파일을 가리킵니다. v2.1.214 이전에는 Claude Code가 크기 확인 없이 파일을 읽었으며, 수 기가바이트 파일이나 `/dev/zero` 같은 장치 파일이 메모리를 무한정 증가시켰습니다.

```text theme={null}
Error: Settings file exceeds the 2MiB limit: /path/to/settings.json
```

Claude Code는 일반 파일이 아닌 `--settings` 경로를 같은 방식으로 거부합니다: 장치, FIFO 또는 소켓은 `Error: Cannot use settings file (Not a regular file (device, FIFO, or socket))`을 보고하고 경로를 따르며, 디렉토리는 `EISDIR` 이유를 보고합니다.

**해야 할 일:**

* `--settings`를 2MiB 미만의 일반 JSON 설정 파일로 지정하세요. 형식은 [설정](/docs/ko/settings)을 참조하세요.

<h3 id="the-current-directory-no-longer-exists">
  현재 디렉토리가 더 이상 존재하지 않음
</h3>

셸이 디렉토리에 들어간 후 삭제되거나 이동된 디렉토리에서 `claude`를 시작했습니다. 예를 들어 worktree 또는 다른 셸이 제거한 임시 디렉토리입니다. Claude Code가 작업 디렉토리를 읽을 수 없어서 대화형 및 [비대화형](/docs/ko/headless) 모드 모두에서 세션을 시작하기 전에 코드 1로 종료됩니다. v2.1.239 이전에는 Claude Code가 축소된 번들 소스와 stderr의 원본 `ENOENT ... uv_cwd` 스택으로 충돌했습니다.

```text theme={null}
The current directory no longer exists (it was deleted or moved). Start Claude Code from an existing directory.
error: The current working directory was deleted, so that command didn't work. Please cd into a different directory and try again.
```

원인과 해결책은 두 형식 모두 동일합니다.

Claude Code가 다른 이유(예: 권한 변경)로 작업 디렉토리를 읽을 수 없으면 메시지가 오류 코드를 이름 지정합니다: `Can't read the current directory (EACCES). Start Claude Code from a different directory.`

macOS에서 `~/Desktop`, `~/Documents`, `~/Downloads` 또는 iCloud Drive의 디렉토리에 대한 `EPERM`은 보통 macOS가 터미널 앱을 해당 폴더에서 차단하고 있다는 의미입니다. 해당 폴더를 읽는 다른 명령어도 같은 방식으로 실패합니다: `ls`는 `sudo`를 사용해도 `Operation not permitted`를 보고합니다.

**해야 할 일:**

* 홈 또는 프로젝트 디렉토리 같은 존재하는 디렉토리로 변경한 후 `claude`를 다시 실행하세요.
* 디렉토리가 같은 경로에서 다시 생성되었으면 셸이 여전히 삭제된 디렉토리를 보유하고 있습니다. `cd "$PWD"`를 실행하거나 디렉토리를 나갔다가 다시 들어간 후 `claude`를 실행하세요.
* macOS에서 `EPERM`의 경우 Cmd+Q로 터미널 앱을 종료하고 다시 열어서 해당 폴더로 돌아가 `claude`를 실행하세요. 해당 폴더의 `ls`가 여전히 실패하면 **System Settings > Privacy & Security > Files and Folders**를 열고 터미널 앱에 대한 폴더를 켠 후 터미널을 다시 열어세요.

<h3 id="temp-directory-refused-or-cannot-be-created">
  임시 디렉토리가 거부되었거나 생성할 수 없음
</h3>

macOS 및 Linux에서 Claude Code는 시작 시 개인 임시 디렉토리를 생성합니다. 시스템 임시 디렉토리 또는 [`CLAUDE_CODE_TMPDIR`](/docs/ko/env-vars) 재정의 아래의 `claude-<uid>`입니다. 디렉토리를 생성할 수 없거나 해당 경로의 항목이 안전 확인에 실패하면 Claude Code는 실패를 stderr에 인쇄하고 세션을 시작하는 대신 코드 1로 종료됩니다:

```text wrap theme={null}
ENOSPC: no space left on device, mkdir '/tmp/claude-501'

Temp directory /tmp/claude-501 is not a directory (may be an attacker-planted symlink). Refusing to use it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is owned by uid 502, expected 501. Refusing to use it — another user may have pre-created it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.

Temp directory /tmp/claude-501 is not readable (its mode may have been altered, or a path component denies search). Refusing to use it — restore its permissions (chmod 0700) or remove it. Set CLAUDE_CODE_TMPDIR to a directory you control, or ask an administrator to remove it.
```

**해야 할 일:**

* `ENOSPC`의 경우 임시 디렉토리를 보유한 볼륨의 디스크 공간을 확보하세요.
* `Refusing to use it` 형식의 경우 링크가 가리키는 것이 아닌 이름 지정된 항목 자체를 제거하고 Claude Code를 다시 시작하세요. `owned by uid` 형식의 경우 관리자 또는 해당 사용자만 제거할 수 있습니다.
* `is not readable`의 경우 이름 지정된 디렉토리에서 `chmod 0700`을 실행하거나 제거한 후 다시 시작하세요.
* 이러한 경우 중 하나에서 [`CLAUDE_CODE_TMPDIR`](/docs/ko/env-vars)을 제어하는 디렉토리로 설정하고 Claude Code를 시작하세요. 거부된 경로는 그대로 두세요.

<h3 id="directory-couldnt-be-resolved-to-a-real-location">
  디렉토리를 실제 위치로 확인할 수 없음
</h3>

작업 디렉토리의 하위 디렉토리에 대해 `/add-dir`을 실행했는데 Claude Code가 디렉토리를 실제 위치로 확인할 수 없습니다.

작업 디렉토리의 하위 디렉토리에 대한 파일 액세스가 이미 있으므로 `/add-dir`은 해당 스킬, 명령어 및 에이전트만 로드합니다. 로드하기 전에 Claude Code는 심볼릭 링크가 확인된 디렉토리의 실제 위치가 작업 디렉토리 내부에 있는지 확인합니다. Claude Code가 해당 위치를 확인할 수 없으면 아무것도 로드하지 않고 이 메시지를 표시합니다:

```text theme={null}
packages/app couldn't be resolved to a real location, so its skills, commands, and agents weren't loaded. Check that it is a directory inside the working directory and try again.
```

**해야 할 일:**

* 경로가 작업 디렉토리 내부의 실제 디렉토리를 이름 지정하는지 확인한 후 `/add-dir`을 다시 실행하세요.
* 메시지는 파일 액세스를 변경하지 않습니다. 디렉토리의 `.claude/` 콘텐츠가 로드되지 않았음을 보고할 뿐입니다.

v2.1.261 이전에는 작업 디렉토리가 `/net/<host>` 자동 마운트에 있을 때마다 이 메시지가 모든 `/add-dir <subdirectory>`에 대해 나타났습니다. Claude Code는 설계상 경로를 확인하기를 거부합니다. 디렉토리는 정상이었고 재시도할 수 없었습니다.

<h3 id="workspace-not-trusted-when-starting-remote-control">
  Remote Control 시작 시 작업 공간을 신뢰하지 않음
</h3>

신뢰하지 않은 디렉토리에서 `claude remote-control` 또는 그 `claude rc` 별칭으로 [Remote Control](/docs/ko/remote-control) 서버 모드를 시작했습니다. 명령어는 작업 공간 신뢰 대화를 표시하지 않으므로 코드 1로 종료되고 수정 사항을 이름 지정합니다:

```text theme={null}
Error: Workspace not trusted. Please run `claude` in /Users/you/project first to review and accept the workspace trust dialog.
```

홈 디렉토리에서 메시지가 다릅니다. 작업 공간 신뢰 대화가 홈 디렉토리에 대한 신뢰를 저장하지 않기 때문입니다. v2.1.214 이전에는 홈 디렉토리가 위의 메시지를 표시했으며, 그 조언은 거기서 성공할 수 없습니다.

```text theme={null}
Error: Workspace not trusted. /Users/you is your home directory, and for security home-directory trust is never saved, so running `claude` here first won't help. Run `claude rc` from a project directory instead (run `claude` there once to accept the trust dialog).
```

**해야 할 일:**

* 디렉토리에서 `claude`를 실행하고 [작업 공간 신뢰 대화](/docs/ko/permissions#project-allow-rules-and-workspace-trust)를 수락한 후 `claude remote-control`을 다시 실행하세요.
* 홈 디렉토리에서 프로젝트 디렉토리로 변경하고 거기서 Remote Control을 시작하세요.

<h3 id="not-carried-over-to-the-sessions-remote-control-starts">
  Remote Control이 시작하는 세션으로 이월되지 않음
</h3>

`remote-control` 동사 앞에 전역 `claude` 플래그로 [Remote Control](/docs/ko/remote-control)을 시작했습니다. Remote Control이 시작하는 세션을 제한하거나 구성할 플래그입니다. 예를 들어 `--settings`, `--setting-sources`, `--permission-mode`, `--disallowed-tools` 또는 `--mcp-config`입니다. 동사 앞에 배치된 플래그는 절대 이러한 세션에 도달하지 않습니다. Claude Code는 플래그를 이름 지정하는 대신 시작을 거부합니다:

```text theme={null}
Error: `--settings` before `remote-control` is not carried over to the sessions Remote Control starts, so Remote Control refuses to start rather than drop it — remove it, and give Remote Control's own options after the verb (see `claude remote-control --help`).
```

Claude Code는 `--verbose`, `--model` 또는 래퍼 주입 `--session-id` 또는 `--plugin-dir` 같은 드롭하기에 무해한 전역 플래그를 거부하지 않습니다: 이를 무시하고 Remote Control이 시작됩니다.

Claude Code는 또한 아직 무해한 것으로 인식하지 못하는 전역 플래그를 거부하므로 최신 릴리스에 추가된 플래그는 이후 릴리스가 이를 무해한 것으로 표시할 때까지 이 메시지에 나타날 수 있습니다.

**해야 할 일:**

* 동사 앞에서 플래그를 제거하고 [Remote Control의 자체 옵션](/docs/ko/remote-control#start-a-remote-control-session)을 그 뒤에 전달하세요. `claude remote-control --help`가 이를 나열합니다.
* 거부된 플래그가 `--permission-mode`이면 `claude remote-control --permission-mode <mode>`를 실행하여 Remote Control이 시작하는 세션의 권한 모드를 설정하세요.

v2.1.248 이전에는 `claude remote-control`이 전역 플래그가 먼저 올 때 자체 플래그를 수용하지 않았으며, 명령어가 알 수 없는 옵션 오류로 실패했습니다.

<h3 id="claude-import-is-not-yet-available-in-this-build">
  claude import는 이 빌드에서 아직 사용할 수 없음
</h3>

[`claude import`](/docs/ko/cli-reference#cli-commands)를 실행했는데 Claude Code가 가져오기 흐름이 꺼져 있음을 발견했으므로 명령어가 이 메시지를 인쇄하는 대신 코드 1로 종료됩니다. v2.1.222 이전에는 가져오기 흐름이 꺼진 빌드가 `import`를 프롬프트로 취급하고 이 메시지를 인쇄하는 대신 대화형 세션을 시작했습니다.

```text theme={null}
`claude import` is not yet available in this build. Run `claude` and use /mcp or edit ~/.claude/settings.json directly.
```

Claude Code는 Anthropic에서 가져오고 디스크에 캐시하는 기능 플래그를 통해 `claude import`를 켭니다. 이 메시지는 캐시된 값이 꺼져 있다는 의미입니다. 원인은 보통 다음 중 하나입니다:

* 설치 후 세션을 시작하지 않았으므로 Claude Code가 플래그를 아직 가져오지 않았습니다. 첫 번째 `claude import`는 기능을 사용할 수 있을 때도 이를 인쇄할 수 있습니다.
* Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry, Claude Platform on AWS를 통해 Claude Code를 사용하거나 [Claude apps gateway](/docs/ko/claude-apps-gateway#availability-and-limitations)를 통해 사용합니다. Claude Code는 이러한 세션에서 기능 플래그를 가져오지 않으므로 `claude import`는 사용할 수 없습니다.
* `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `DISABLE_GROWTHBOOK` 또는 [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ko/env-vars)를 설정했으므로 기능 플래그 가져오기가 꺼져 있고 `claude import`는 사용할 수 없습니다.

**해야 할 일:**

* 새로 설치한 경우 `claude`를 시작하고 세션이 로드될 때까지 기다린 후 종료하고 `claude import`를 다시 실행하세요.
* 기능 플래그 가져오기가 꺼진 경우 구성을 직접 설정하세요: [`claude mcp add`](/docs/ko/mcp#installing-mcp-servers)로 MCP 서버를 추가하고 [CLAUDE.md 파일](/docs/ko/memory#how-claude-md-files-load), [스킬 및 명령어](/docs/ko/skills#where-skills-live) 및 [하위 에이전트](/docs/ko/sub-agents#choose-the-subagent-scope)를 생성하세요. 메시지는 또한 `~/.claude/settings.json`을 이름 지정합니다. `claude import`가 이월하는 구성 중에서 해당 파일은 [권한 모드](/docs/ko/settings-reference#permission-settings)만 보유합니다. Claude Code는 이 파일에서 MCP 서버를 읽지 않습니다.

<h3 id="could-not-read-claude-code-config">
  Claude Code 구성을 읽을 수 없음
</h3>

Claude Code가 로그인 및 프로젝트별 상태를 저장하는 파일인 `~/.claude.json`을 파싱할 수 없는 동안 [`claude import`](/docs/ko/cli-reference#cli-commands)를 실행했습니다. 하위 명령어는 가용성을 확인하기 위해 해당 파일을 읽지만 대화형 세션이 표시하는 복구 대화를 표시하지 않으므로 코드 1로 종료됩니다. v2.1.222 이전에는 읽을 수 없는 구성 파일이 있는 `claude import`가 대화형 세션을 시작했으며, 복구 대화가 파일을 처리했습니다.

```text theme={null}
Could not read Claude Code config — run `claude` with no arguments to recover it.
```

**해야 할 일:**

* 인수 없이 `claude`를 실행하세요. Claude Code가 유효하지 않은 파일을 감지하고 재설정을 제안합니다. 그런 다음 `claude import`를 다시 실행하세요.
* 수동으로 편집한 내용을 유지하려면 편집기에서 `~/.claude.json`의 JSON 구문을 수정한 후 `claude import`를 다시 실행하세요.

<h3 id="could-not-import-a-server-from-claude-desktop">
  Claude Desktop에서 서버를 가져올 수 없음
</h3>

Claude Code가 `claude mcp add-from-claude-desktop`에서 선택한 서버 중 하나를 추가할 수 없습니다. 명령어는 여전히 다른 선택된 서버를 가져오고 추가할 수 없는 각 서버당 한 줄을 인쇄합니다. v2.1.205 이전에는 실패한 첫 번째 서버가 가져오기를 중지했고 선택된 서버 중 어느 것도 추가되지 않았습니다.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

서버 이름 뒤의 텍스트가 이유입니다. 가장 일반적인 것은 이름 확인입니다: Claude Desktop은 서버 이름에 공백과 마침표 같은 문자를 허용하지만 `claude mcp`는 문자, 숫자, 하이픈 및 밑줄로 제한합니다. 다른 이유로는 검증에 실패한 서버 구성과 조직의 [MCP 정책](/docs/ko/managed-mcp)에 의해 차단된 서버가 있습니다.

**해야 할 일:**

* `claude_desktop_config.json`에서 서버 이름을 문자, 숫자, 하이픈 및 밑줄만 사용하도록 변경한 후 `claude mcp add-from-claude-desktop`을 다시 실행하세요.
* 유효한 이름으로 `claude mcp add` 또는 `claude mcp add-json`을 사용하여 해당 서버를 직접 추가하세요. [Claude Desktop에서 MCP 서버 가져오기](/docs/ko/mcp#import-mcp-servers-from-claude-desktop)를 참조하세요.

<h3 id="cannot-add-mcp-server-to-the-managed-scope">
  MCP 서버를 관리 범위에 추가할 수 없음
</h3>

`--scope managed`로 `claude mcp add` 또는 `claude mcp add-json`을 실행했습니다. 해당 범위는 조직이 [`managedMcpServers`](/docs/ko/settings-reference#managedmcpservers) 관리 설정을 통해 제공하는 서버를 보유합니다. Claude Code는 관리 설정에서만 이를 읽으므로 명령어가 해당 범위에 서버를 쓸 수 없습니다.

```text theme={null}
Cannot add MCP server to scope: managed
```

**해야 할 일:**

* 쓸 수 있는 범위에 서버를 추가하세요: `local`, `user` 또는 `project`. `--scope` 없이 명령어는 `local`을 사용합니다. [MCP 설치 범위](/docs/ko/mcp#mcp-installation-scopes)를 참조하세요.
* 조직의 모든 사용자에게 서버를 제공하려면 배포하는 관리 설정의 [`managedMcpServers`](/docs/ko/settings-reference#managedmcpservers)에 추가하세요.

<h3 id="cant-read-mcp-json">
  .mcp.json을 읽을 수 없음
</h3>

프로젝트의 [`.mcp.json`](/docs/ko/mcp#project-scope)을 읽는 명령어(예: `--scope project`로 `claude mcp add` 또는 `claude mcp add-json`, 또는 `claude mcp remove`)가 현재 디렉토리의 파일이 일반 파일이 아니거나 2MiB보다 크다는 것을 발견했으므로 파일을 읽는 대신 이 오류로 종료됩니다.

```text theme={null}
Can't read .mcp.json: it isn't a regular file or is larger than 2097152 bytes. Fix or remove it, then run the command again.
```

v2.1.257 이전에는 `.mcp.json`의 FIFO가 명령어를 출력 없이 영원히 기다리게 했으며, `/dev/zero` 같은 장치 파일로의 심볼릭 링크가 프로세스가 종료될 때까지 메모리를 증가시켰습니다.

**해야 할 일:**

* 현재 디렉토리의 `.mcp.json`에 무엇이 있는지 확인하세요. [프로젝트 범위 형식](/docs/ko/mcp#project-scope)의 일반 JSON 파일로 바꾸거나 삭제한 후 명령어를 다시 실행하세요.

<h3 id="anthropic-hosted-and-doesnt-support-local-oauth">
  서버는 Anthropic 호스팅이며 로컬 OAuth를 지원하지 않음
</h3>

URL이 타사 ID 공급자를 통해 인증하는 Anthropic 호스팅 커넥터 호스트를 가리키는 MCP 서버에 대한 로그인을 시작했습니다. 이러한 호스트에는 `microsoft365.mcp.claude.com`, `gmail.mcp.claude.com` 및 `gcal.mcp.claude.com`이 포함됩니다. Claude Code는 `/mcp` 패널과 `claude mcp login` 모두에서 이러한 호스트에 대한 로컬 OAuth 흐름을 시작하기를 거부합니다. [이들의 로그인은 claude.ai를 통해서만 작동](/docs/ko/mcp#use-mcp-servers-from-claude-ai)하기 때문입니다.

```text theme={null}
"gmail" is Anthropic-hosted and doesn't support local OAuth. Connect it via Settings → Connectors on claude.ai (requires `claude login`), then it'll be available here automatically.
```

Claude Code는 URL로 이러한 호스트를 일치시키므로 `claude mcp add` 또는 `.mcp.json`에 추가한 서버가 이 중 하나를 가리킬 때 메시지가 나타납니다.

**해야 할 일:**

* `claude mcp remove <name>`으로 항목을 제거하여 같은 URL의 claude.ai 커넥터를 숨길 수 없도록 하세요.
* 제거한 후 Claude Code에 로그인한 계정을 사용하는 동안 [claude.ai/customize/connectors](https://claude.ai/customize/connectors)에서 서비스를 연결하세요. 연결되면 활성 인증 방법이 claude.ai 구독 로그인이면 [커넥터가 Claude Code에 자동으로 나타납니다](/docs/ko/mcp#use-mcp-servers-from-claude-ai).

<h3 id="server-rejected-the-authorization-header-minted-by-the-configured-headershelper">
  서버가 구성된 headersHelper에 의해 발행된 Authorization 헤더를 거부함
</h3>

[`headersHelper`](/docs/ko/mcp#use-dynamic-headers-for-custom-authentication)가 `Authorization` 헤더를 제공하는 MCP 서버가 HTTP 401 또는 403으로 연결에 응답했으므로 Claude Code는 연결을 실패로 보고합니다. 헬퍼가 `Authorization` 헤더를 제공하므로 Claude Code는 [OAuth로 폴백하지 않습니다](/docs/ko/mcp#authenticate-with-remote-mcp-servers):

```text theme={null}
Server rejected the Authorization header minted by the configured headersHelper (HTTP 401). Check that the helper command returns a valid credential for this MCP endpoint — OAuth fallback is disabled when the helper supplies Authorization.
```

Claude Code는 각 연결 시도에서 헬퍼를 다시 실행하므로 토큰 회전 경쟁 같은 일시적 거부 후 재시도가 새로운 자격 증명으로 성공할 수 있습니다.

**해야 할 일:**

* Claude Code가 실행하는 방식으로 `headersHelper` 명령어를 직접 실행하세요: [Claude Code가 실행하는 디렉토리](/docs/ko/mcp#where-the-helper-runs)에서, [Claude Code가 설정하는 환경 변수](/docs/ko/mcp#use-dynamic-headers-for-custom-authentication)를 사용하여, 프로젝트 `.mcp.json`, 플러그인 또는 프로젝트 에이전트 파일의 서버에 대해 [Claude Code가 제거하는 자격 증명 변수](/docs/ko/mcp#which-variables-a-helper-can-read) 없이. 서버의 엔드포인트가 수용하는 `Authorization` 값을 인쇄하는지 확인하세요.
* 헬퍼 또는 자격 증명 소스를 수정한 후 `/mcp`에서 서버를 선택하고 **Reconnect**를 선택하세요.

v2.1.248 이전에는 Claude Code가 헬퍼가 `Authorization` 헤더를 제공하는 서버에 대해 OAuth 검색을 실행했습니다. 해당 검색이 거부된 자격 증명을 보고하는 대신 `Incompatible auth server: does not support dynamic client registration`으로 실패할 수 있습니다.

<h3 id="mcp-permission-prompt-tool-not-found">
  MCP 권한 프롬프트 도구를 찾을 수 없음
</h3>

[`--permission-prompt-tool`](/docs/ko/cli-reference#cli-flags)에 전달한 도구가 실행이 처음 권한 결정이 필요할 때 연결된 MCP 도구 중에 없습니다. 서버가 연결되지 않았거나 연결된 서버가 해당 이름의 도구를 노출하지 않기 때문입니다. Claude Code는 여전히 프롬프트를 보냅니다: [비대화형](/docs/ko/headless) 실행이 승인이 필요한 첫 번째 도구 호출에서 이 오류로 종료되고 코드 1로 종료되므로 요청이 이루어졌음에도 불구하고 답변을 생성하지 않습니다. 첫 번째 프롬프트 전에 Claude Code는 [`MCP_TIMEOUT`](/docs/ko/env-vars)으로 설정된 서버당 연결 타임아웃 30초까지 해당 서버가 연결될 때까지 기다립니다. v2.1.206 이전에는 시작이 서버가 연결을 마칠 때까지 기다리지 않았으므로 느리게 시작하지만 정상인 서버가 이 오류를 생성했습니다.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

`Available MCP tools:` 뒤의 목록은 대기가 끝났을 때 연결된 MCP 도구를 이름 지정합니다.

**해야 할 일:**

* 서버가 시작되고 연결된 상태를 유지하는지 확인하세요: 같은 디렉토리에서 `claude mcp list`를 실행하고 서버가 연결됨으로 나열되는지 확인하세요.
* 도구 이름이 서버가 노출하는 `mcp__<server>__<tool>` 이름과 일치하는지 확인하세요.
* 서버가 시작하는 데 30초 이상 필요하면 [`MCP_TIMEOUT`](/docs/ko/env-vars)을 높이세요.

<h3 id="oauth-callback-port-is-already-in-use">
  OAuth 콜백 포트가 이미 사용 중
</h3>

OAuth를 사용하여 원격 MCP 서버에 로그인하면 Claude Code는 로그인 콜백을 수신하기 위해 로컬 리스너를 시작합니다. 해당 리스너가 필요한 포트가 다른 프로세스에 의해 보유되면 로그인이 이 메시지로 실패합니다. 이는 주로 [`MCP_OAUTH_CALLBACK_PORT`](/docs/ko/env-vars) 변수 또는 `--callback-port`를 통해 설정된 [고정 콜백 포트](/docs/ko/mcp#use-a-fixed-oauth-callback-port)에서 발생합니다. 없으면 Claude Code가 사용 가능한 포트를 선택하기 때문입니다.

```text theme={null}
OAuth callback port <port> is already in use — another process may be holding it. Run `lsof -ti:<port> -sTCP:LISTEN` to find it.
```

Windows에서 제안된 명령어는 대신 `netstat -ano | findstr :<port>`입니다.

**해야 할 일:**

* 메시지의 명령어를 실행하여 포트를 보유한 프로세스를 찾고 중지하거나 완료될 때까지 기다리세요.
* 다른 프로그램이 해당 포트를 영구적으로 필요로 하면 서버에 다른 리디렉션 URI를 등록하고 `MCP_OAUTH_CALLBACK_PORT` 또는 `--callback-port`(사용하는 것)로 포트를 설정하세요.
* 그런 다음 로그인을 다시 시작하세요. 예를 들어 `/mcp`에서 서버를 선택하세요.

<h3 id="no-available-ports-for-oauth-redirect">
  OAuth 리디렉션에 사용 가능한 포트 없음
</h3>

[OAuth](/docs/ko/mcp#authenticate-with-remote-mcp-servers)를 사용하여 원격 MCP 서버에 로그인하면 Claude Code는 로그인 콜백을 수신하기 위해 로컬 리스너를 시작합니다. Claude Code가 로컬 포트를 바인드할 수 없을 때 로그인이 이 메시지로 실패합니다. 머신의 무언가가 `127.0.0.1`에서 수신 대기하는 것을 방지합니다. 예를 들어 보안 소프트웨어 또는 로컬 리스너를 거부하는 샌드박스 정책입니다.

```text theme={null}
No available ports for OAuth redirect
```

v2.1.268 이전에는 Claude Code가 운영 체제 할당 포트로 폴백하지 않았으므로 메시지는 Claude Code가 선택한 포트만 바인드할 수 없을 때도 나타났습니다. 이는 Hyper-V가 Claude Code가 선택하는 포트를 포함하는 포트 범위를 예약하는 Windows 호스트에서 발생할 수 있습니다.

**해야 할 일:**

* 보안 소프트웨어 또는 샌드박스 정책이 프로세스가 `127.0.0.1`에서 수신 대기하는 것을 차단하는지 확인하고 Claude Code가 로컬 포트를 바인드하도록 허용하세요.
* 그런 다음 로그인을 다시 시작하세요. 예를 들어 `/mcp`에서 서버를 선택하세요.

<h3 id="security-review-fails-without-origin-head">
  /security-review가 origin/HEAD 없이 실패함
</h3>

[`/security-review`](/docs/ko/commands#all-commands)는 `origin/HEAD`에 대해 분기를 비교하여 검토 컨텍스트를 구축합니다. `origin/HEAD`는 `origin` 원격의 기본 분기가 무엇인지 기록하는 로컬 ref입니다. 해당 ref가 없으면 diff를 수집하는 git 명령어가 실패하고 검토가 시작하기 전에 중지됩니다.

```text theme={null}
Error: Shell command failed for pattern "!`git diff --name-only origin/HEAD...`": [stderr]
fatal: ambiguous argument 'origin/HEAD...': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
```

메시지는 `git log` 또는 다른 `git diff`를 인용할 수 있습니다. Git은 원격이 기본 분기를 광고하고 fetch refspec이 이를 포함할 때만 `origin/HEAD`를 생성합니다. 전체 `git clone`이 커밋이 있는 원격을 수행합니다. ref는 이러한 설정에서 누락됩니다:

* 단일 분기 또는 CI 체크아웃(너무 좁은 refspec을 가져옴)
* 서버 측 HEAD가 아무도 푸시하지 않은 분기를 가리키는 원격
* `origin` 원격이 없거나 절대 가져오지 않은 저장소

Claude Code는 [동적 컨텍스트를 주입](/docs/ko/skills#when-an-injected-command-fails)하는 모든 스킬에 대해 같은 오류를 표시하며, 실패한 주입 명령어는 해당 스킬의 호출을 중단합니다. 명령어가 실행되기 전에 두 개의 형제 문자열이 발생합니다:

* `Shell command permission check failed for pattern "..."`: 명령어의 권한 확인이 이를 허용하지 않았습니다. [주입 명령어의 권한 확인](/docs/ko/skills#permission-checks-on-injected-commands)은 각 권한 모드에서 어떤 결과가 중단되는지 그리고 `allowed-tools`로 명령어를 사전 승인하는 방법을 다룹니다.
* ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``: 스킬의 frontmatter가 bash가 없는 머신에서 bash를 요구합니다. Git for Windows를 설치하거나 frontmatter를 `shell: powershell`로 변경하세요. [주입 명령어가 실행되는 방식](/docs/ko/skills#how-injected-commands-run)을 참조하세요.

**해야 할 일:**

* 원격의 기본 분기를 이름 지정하여 ref를 생성하세요: `git remote set-head origin <default-branch>`. 이는 로컬 추적 ref `origin/<default-branch>`가 존재할 때마다 작동합니다. 단일 분기 클론처럼 없으면 먼저 분기를 가져오세요: `git remote set-branches --add origin <branch>`를 실행한 후 `git fetch origin`을 실행한 후 set-head 명령어를 다시 실행하세요. `/security-review`를 다시 실행하세요.
* 분기를 이름 지정하지 않으려면 `git fetch origin`을 실행한 후 `git remote set-head origin --auto`를 실행하세요. 이는 원격에 기본 분기가 무엇인지 묻습니다. 원격이 비어 있거나 HEAD가 아무도 푸시하지 않은 분기를 가리킬 때 `error: Cannot determine remote HEAD`로 실패합니다. 분기를 명시적으로 이름 지정하세요. 클론이 해당 분기를 가져오지 않을 때 `error: Not a valid ref`로 실패합니다. 위에서 refspec을 확대하세요.
* 저장소에 원격이 없으면 `git remote add origin <url>`로 추가하고 ref를 생성하기 전에 가져오세요. 원격이 비어 있으면 `git push -u origin HEAD`로 분기를 먼저 푸시하고 set-head 명령어에서 해당 분기를 이름 지정하세요. `origin/HEAD`는 방금 푸시한 분기를 가리키므로 `/security-review`는 분기가 이로부터 분기될 때까지 빈 diff를 봅니다.

<h3 id="input-must-be-provided-when-using-print">
  \--print 사용 시 입력을 제공해야 함
</h3>

베어 `claude`는 대화형 UI를 시작하기 위해 stdout이 터미널이어야 합니다. stdout이 리디렉션되거나 PowerShell ISE 및 일부 IDE 출력 창 같은 실제 터미널이 아닐 때 `claude`는 [비대화형](/docs/ko/headless) 모드로 실행됩니다. 이는 프롬프트가 필요한 `claude -p`와 같은 모드이므로 메시지는 플래그를 전달하지 않았을 때도 `--print`를 이름 지정합니다. 프롬프트 없이 `-p`/`--print`를 전달하고 stdin에 파이프된 것이 없으면 어디서나 같은 오류를 생성합니다.

```text theme={null}
Error: Input must be provided either through stdin or as a prompt argument when using --print
```

**해야 할 일:**

* 대화형 사용의 경우 실제 터미널에서 `claude`를 실행하세요: PowerShell ISE가 아닌 Windows Terminal 또는 PowerShell 콘솔, IDE의 출력 창이 아닌 통합 터미널.
* 일회용 사용의 경우 프롬프트를 전달하세요: `claude -p "your question"`, 또는 `echo "your question" | claude -p`로 파이프하세요.

<h3 id="input-contained-only-whitespace">
  입력에 공백만 포함됨
</h3>

[비대화형 모드](/docs/ko/headless)에서 Claude Code는 API가 보이는 텍스트가 없는 메시지를 거부하기 때문에 공백, 탭 또는 줄 바꿈으로만 구성된 프롬프트를 보내는 대신 거부합니다. 어떤 메시지를 보는지는 빈 프롬프트가 어디서 왔는지에 따라 다릅니다:

* **`claude -p`의 프롬프트 인수 또는 파이프된 stdin**: `claude`가 `Error: Input contained only whitespace. Provide a prompt with text through stdin or as a prompt argument when using --print`로 종료됩니다.
* **실행 중인 `--input-format stream-json` 또는 [Agent SDK](/docs/ko/agent-sdk/overview) 세션에 제출된 메시지**: Claude Code는 모델을 호출하지 않고 턴을 종료하며 세션은 사용 가능한 상태로 유지됩니다. 거부는 정보 메시지로 그리고 턴의 결과 텍스트로 도착합니다: `Blank prompt — the message was only whitespace, so nothing was sent to the model.`

v2.1.229 이전에는 Claude Code가 공백 전용 메시지를 API로 보냈으며, API가 400 오류로 요청을 거부했습니다.

**해야 할 일:**

* 프롬프트에 보이는 텍스트를 포함하세요. 스크립트가 변수 또는 파일에서 프롬프트를 구축하면 Claude Code를 호출하기 전에 소스가 비어 있지 않은지 확인하세요.

<h3 id="stream-json-input-carried-over-256m-characters-with-no-newline">
  stream-json 입력이 줄 바꿈 없이 256M 문자를 초과함
</h3>

프로그램이 `claude -p --input-format stream-json` 실행에 stdin으로 줄 바꿈 없이 268,435,456자 이상을 보냈으므로 Claude Code는 이 오류를 stderr에 인쇄하고 더 많은 입력을 버퍼링하는 대신 코드 1로 종료됩니다. 메시지는 해당 예산을 `256M`으로 명시합니다. v2.1.257 이전에는 Claude Code가 이러한 입력을 제한 없이 버퍼링했으며, 프로세스가 충돌하거나 종료될 때까지 메모리를 증가시켰습니다.

```text theme={null}
Error: stream-json input carried over 256M characters with no newline. Each stream-json message must be a single newline-terminated JSON line: either the producer is not newline-terminating its messages, or one message exceeded this budget.
```

줄 바꿈 없이 이 정도로 긴 입력은 보통 생산자가 stream-json 생산자가 아니라는 의미입니다. 예를 들어 실수로 파이프된 바이너리 파일 또는 일반 로그 출력입니다. 예산을 초과하는 단일 메시지가 같은 확인에 실패합니다.

**해야 할 일:**

* stdin으로 파이프되는 것을 확인하세요. [`--input-format stream-json`](/docs/ko/cli-reference#cli-flags)을 사용하면 모든 메시지는 하나의 줄 바꿈으로 끝나는 JSON 줄이어야 합니다.
* 일반 텍스트를 대신 보내려면 `--input-format stream-json`을 제거하세요. `claude -p`는 기본적으로 stdin에서 일반 텍스트 프롬프트를 읽습니다.

<h3 id="unknown-command">
  알 수 없는 명령어
</h3>

대화형 터미널 세션에서 이 세션의 명령어와 일치하지 않는 `/` 이름을 제출했으므로 Claude Code는 아무것도 실행하지 않고 이름을 보고합니다:

```text theme={null}
Unknown command: /hepl. Did you mean /help?
```

Claude Code는 이 세션의 메뉴가 나열하는 가장 가까운 명령어 이름 또는 별칭을 제안합니다. 가까운 것이 없으면 메시지가 이름 뒤에서 끝납니다. 원인은 보통 다음 중 하나입니다:

* `/hepl`을 `/help`로 오타. [명령어 메뉴가 입력과 일치하는 방식](/docs/ko/commands#how-the-command-menu-matches-what-you-type)은 제출하기 전에 가까운 일치를 선택하는 것을 다룹니다.
* 존재하지만 플랫폼, 계획 또는 인증 방법 같은 요구 사항이 충족되지 않아 이 세션에서 사용할 수 없는 명령어. [`/web-setup`](/docs/ko/web-quickstart#web-setup-shows-no-commands-match-or-unknown-command) 및 [`/schedule`](/docs/ko/routines#schedule-returns-unknown-command)의 문제 해결 항목이 두 가지 일반적인 경우를 안내합니다. 일부 명령어는 조직의 정책이 이를 비활성화할 때 [`Cloud sessions are disabled by your organization's policy`](#cloud-sessions-are-disabled-by-your-organizations-policy) 같은 자체 메시지로 응답합니다.
* [플러그인](/docs/ko/plugins) 또는 [MCP 서버](/docs/ko/mcp#use-mcp-prompts-as-commands)의 명령어가 이 세션에 설치되거나 연결되지 않음.

Claude Code는 대화형 터미널 세션에서만 일치하지 않는 `/` 이름에 이 방식으로 응답합니다. 다른 모든 세션에서는 프롬프트를 Claude에 일반 메시지로 보냅니다. 명령어가 실행되지 않았고 Claude가 세션에서 실행할 수 있는 명령어 목록이 있다는 참고 사항이 포함됩니다. 이러한 세션에는 다음이 포함됩니다:

* `-p` 실행
* [Agent SDK](/docs/ko/agent-sdk/overview) 애플리케이션
* [Desktop app](/docs/ko/desktop)의 Code 탭
* [VS Code extension](/docs/ko/vs-code)의 채팅 패널
* [클라우드 세션](/docs/ko/claude-code-on-the-web) 및 [루틴](/docs/ko/routines)

이러한 세션 중 하나에서 실행할 수 없는 기본 제공 명령어의 경우 Claude Code는 여전히 명령어를 Claude로 보내는 대신 사용할 수 없다고 응답합니다. v2.1.274 이전에는 클라우드 세션과 루틴만 일치하지 않는 이름을 Claude로 보냈습니다. v2.1.273 이전에는 `Unknown command`로도 응답했습니다.

Claude Code는 `/`로 시작하는 모든 프롬프트를 명령어로 취급하지 않습니다. `/` 뒤의 첫 번째 단어가 Lean doc 주석을 여는 `/--` 같은 구두점으로 시작하거나 `/var/log/syslog` 같은 경로일 때 프롬프트를 Claude에 일반 메시지로 보냅니다.

v2.1.236 이전에는 입력한 이름에 대해 명령어 메뉴가 가까운 일치를 나열하는 동안 Enter를 누르면 Claude Code가 해당 일치를 실행했으므로 `/hepl` 같은 오타가 이 메시지를 생성하는 대신 `/help`를 실행했습니다.

**해야 할 일:**

* 제안된 이름을 실행하거나 `/`를 입력한 후 이름의 일부를 입력하여 이 세션에서 사용 가능한 것을 확인하세요.
* Claude Code가 문서화된 명령어를 알 수 없는 것으로 보고하면 [명령어 참조](/docs/ko/commands)의 행에서 이름 지정하는 요구 사항을 확인하세요.

<h3 id="diff-is-too-large-for-ultrareview">
  Diff가 ultrareview에 너무 큼
</h3>

분기와 기본 분기 간의 diff(커밋되지 않은 변경 사항 및 스테이징된 변경 사항 포함)가 [ultrareview](/docs/ko/ultrareview)의 크기 제한을 초과하므로 `/code-review ultra` 및 `claude ultrareview` 하위 명령어가 클라우드 세션이 시작되기 전에 검토를 거부합니다. 거부된 검토는 무료 실행을 사용하지 않으며 사용 크레딧을 청구하지 않습니다. 메시지는 적용 중인 제한, diff의 크기 및 가장 많은 변경된 줄에 기여하는 파일을 이름 지정합니다. v2.1.216 이전에는 메시지가 원본 diff 통계만 표시했습니다.

```text theme={null}
Diff is too large for ultrareview: 812 files, 96,410 lines changed (limits: 500 files, 8,000 lines). Largest files: package-lock.json (41,904 lines), dist/bundle.js (18,210 lines), src/generated/api.ts (9,876 lines). Pass a closer base branch (`/code-review ultra <branch>`) to narrow the scope, or split the change.
```

풀 요청을 검토하면 같은 제한이 적용됩니다. 해당 형식의 메시지는 `PR #<N> is too large for ultrareview`로 시작하고 PR의 파일 및 줄 수를 이름 지정합니다.

**해야 할 일:**

* 기본 분기를 더 가깝게 전달하세요. 예를 들어 `/code-review ultra develop`이므로 검토가 해당 분기에 대한 diff만 포함합니다.
* 변경을 더 작은 분기로 분할하고 각각을 검토하세요. 메시지가 이름 지정한 파일이 가장 많은 변경된 줄에 기여하므로 이를 자체 분기로 이동하여 시작하세요.

<h3 id="could-not-find-merge-base-with-the-base-branch">
  기본 분기와의 병합 기반을 찾을 수 없음
</h3>

`/code-review ultra` 및 `claude ultrareview` 하위 명령어는 분기와 기본 분기 간의 diff를 검토하며, 이는 두 분기가 공유하는 커밋이 필요합니다. `git merge-base`가 없음을 발견하면 Claude Code는 클라우드 세션이 시작되기 전에 검토를 거부합니다. Claude Code가 완전한 것으로 확인할 수 있는 클론에서 최소 하나의 분기가 있으면 대신 [모든 추적된 파일을 검토](/docs/ko/ultrareview#diff-limits-and-fallbacks)로 폴백합니다. 기본 분기를 전혀 찾을 수 없을 때, Claude Code가 클론이 완전한지 확인할 수 없을 때 또는 SHA-256 객체 형식 같은 드문 저장소에서 전체 트리 diff가 불가능할 때 이 거부를 봅니다.

```text theme={null}
Could not find merge-base with main. Pass the base branch explicitly (e.g. `/code-review ultra develop`) or make sure you're in a git repo with a main branch.
```

첫 번째 문장 뒤의 힌트는 Claude Code가 관찰한 것에 따라 다릅니다:

* **기본 분기를 전달하지 않음**: Claude Code가 저장소의 기본 분기와 비교했으며 위의 예처럼 기본을 명시적으로 전달하도록 제안합니다.
* **이미 클론에 있는 기본 분기를 전달함**: 힌트는 ``Make sure <branch> exists locally or on origin (try `git fetch origin <branch>`)``를 읽습니다.
* **클론에 없는 기본 분기를 전달함**: Claude Code가 비교하기 전에 origin에서 가져왔습니다. 힌트는 ``<branch> was fetched from origin but shares no history with HEAD. If another branch is your real base, pass it explicitly (`/code-review ultra <branch>`)``를 읽습니다. Claude Code가 클론이 얕은지 확인할 수 없을 때 대신 `git fetch --unshallow origin`을 제안합니다. v2.1.221 이전에는 모든 가져온 기본 분기에 대해 `git fetch --unshallow origin`을 제안했으며, 완전한 클론에서 해당 명령어는 `fatal: --unshallow on a complete repository does not make sense`로 실패합니다.

**해야 할 일:**

* 다른 분기가 실제 기본이면 명시적으로 전달하세요: `/code-review ultra <branch>`
* 클론이 전체 기록을 갖지 않을 수 있으면 `git fetch --unshallow origin`을 실행하고 검토를 다시 실행하세요.

<h3 id="your-checkout-has-no-branches">
  체크아웃에 분기가 없음
</h3>

체크아웃은 커밋을 가질 수 있지만 분기는 없습니다: `git init` 다음 `git fetch <url>` 및 `git checkout FETCH_HEAD`를 실행하면 분기가 없는 분리된 HEAD를 얻습니다. Claude Code는 저장소를 git 번들로 패키징하여 [ultrareview](/docs/ko/ultrareview)를 위해 업로드하며, 분기나 다른 ref가 없는 저장소를 번들할 수 없으므로 `/code-review ultra` 및 `claude ultrareview` 하위 명령어가 클라우드 세션이 시작되기 전에 검토를 거부합니다.

```text theme={null}
Your checkout has no branches (detached HEAD only), which cloud review can't bundle. Create one first — `git checkout -b <name>` — then rerun /code-review ultra.
```

v2.1.221 이전에는 Claude Code가 이 체크아웃에서 모든 추적된 파일을 검토하려고 시도했으며, 업로드가 실패했습니다.

**해야 할 일:**

* 현재 커밋에서 `git checkout -b <name>`으로 분기를 생성한 후 검토를 다시 실행하세요.

<h3 id="no-github-account-is-connected-to-your-claude-account">
  Claude 계정에 연결된 GitHub 계정이 없음
</h3>

`/code-review ultra <PR#>` 또는 `claude ultrareview <PR#>`을 실행했으며, 클라우드 세션을 생성하기 전에 Claude Code는 서버에 [Claude 계정에 연결된 GitHub 계정](/docs/ko/ultrareview#review-a-pull-request)이 PR의 저장소에 도달할 수 있는지 묻습니다. 계정이 연결되지 않았거나 연결이 만료되었으므로 클라우드 클론이 실패하고 Claude Code는 시작을 거부합니다. Claude Code는 거부된 시작에 대해 무료 실행을 사용하거나 사용 크레딧을 청구하지 않습니다.

```text theme={null}
Ultrareview clones <owner>/<repo> in the cloud with the GitHub account connected to your Claude account, and none is connected (or the connection expired). To fix: run /web-setup to reuse your GitHub CLI login, or connect an account at https://claude.ai/connect-github — then re-run /code-review ultra 1234 (allow a minute after connecting).
```

[`/web-setup`](/docs/ko/web-quickstart#connect-from-your-terminal)을 세션에서 사용할 수 없으면 메시지는 claude.ai 링크만 이름 지정합니다.

**해야 할 일:**

* `/web-setup`을 실행하여 GitHub CLI 로그인을 Claude 계정에 연결하거나 [claude.ai/connect-github](https://claude.ai/connect-github)에서 계정을 연결하세요.
* 연결 1분 후 검토를 다시 실행하세요.

v2.1.248 이전에는 Claude Code가 시작 전에 이를 확인하지 않았습니다.

<h3 id="your-connected-github-account-cant-see-the-repository">
  연결된 GitHub 계정이 저장소를 볼 수 없음
</h3>

`/code-review ultra <PR#>` 또는 `claude ultrareview <PR#>`을 실행했으며, [Claude 계정에 연결된 GitHub 계정](/docs/ko/ultrareview#review-a-pull-request)이 PR의 저장소를 읽을 수 없으므로 클라우드 클론이 실패하고 Claude Code는 시작을 거부합니다. Claude Code는 거부된 시작에 대해 무료 실행을 사용하거나 사용 크레딧을 청구하지 않습니다.

```text theme={null}
Your connected GitHub account can't see <owner>/<repo> — usually the Claude GitHub app isn't installed on <owner> or wasn't granted this repo (web-connected accounts need it for private repos), or a different GitHub account is connected. To fix: run /web-setup to reuse your GitHub CLI login, or install the app at https://github.com/apps/claude/installations/new — then re-run /code-review ultra 1234.
```

[`/web-setup`](/docs/ko/web-quickstart#connect-from-your-terminal)을 세션에서 사용할 수 없으면 메시지는 앱 설치만 이름 지정합니다.

**해야 할 일:**

* 로컬 `gh` CLI가 저장소를 읽을 수 있으면 `/web-setup`을 실행하여 해당 로그인을 Claude 계정에 연결하세요.
* 변경 후 검토를 다시 실행하세요.

v2.1.248 이전에는 Claude Code가 시작 전에 이를 확인하지 않았습니다.

<h3 id="the-github-app-preflight-failed-transiently">
  GitHub App 사전 점검이 일시적으로 실패함
</h3>

로컬 저장소에서 [클라우드 세션](/docs/ko/claude-code-on-the-web)을 시작했으며, 두 단계가 함께 실패했습니다. Claude Code가 저장소 번들을 구축하거나 업로드할 수 없습니다. 업로드 전에 클라우드 서비스가 GitHub에서 저장소를 클론할 수 있는지 확인했으며, 명확한 답변 대신 재시도가 지울 수 있는 오류로 끝났습니다. 예를 들어 네트워크 오류, 타임아웃 또는 임시 서버 오류입니다. 전체 메시지는 번들을 중지한 것으로 시작합니다. 예를 들어 `Could not upload repo bundle (<error>)`이고 사전 점검 문장으로 끝납니다:

```text theme={null}
Could not upload repo bundle (<error>). The GitHub App preflight failed transiently (network or service hiccup) — retry in a moment to start from GitHub instead
```

**해야 할 일:**

* 잠시 후 명령어를 다시 실행하세요. GitHub 확인이 통과하면 Claude Code가 GitHub 클론에서 시작할 수 있으므로 실패한 업로드가 더 이상 시작을 차단하지 않습니다.
* 재시도가 계속 실패하면 메시지의 시작이 업로드를 중지한 것을 이름 지정합니다. 해당 원인이 수정할 수 있는 것이면 수정하여 세션이 로컬 저장소에서 시작할 수 있도록 하세요.

v2.1.251 이전에는 Claude Code가 GitHub 확인이 일시적으로만 실패했을 때도 `Please set up GitHub on https://claude.ai/code`로 메시지를 끝냈으며, 설정 조언은 일시적 실패를 지울 수 없습니다.

<h3 id="github-isnt-connected-to-your-claude-account">
  GitHub가 Claude 계정에 연결되지 않음
</h3>

로컬 저장소에서 [클라우드 세션](/docs/ko/claude-code-on-the-web)을 시작했습니다. 예를 들어 `/autofix-pr`을 사용합니다. Claude 계정에 연결된 GitHub 계정이 없거나 연결이 만료되었으므로 Claude Code는 시작을 거부합니다:

```text theme={null}
GitHub isn't connected to your Claude account, so this repository can't be cloned in the cloud. Run /web-setup to connect with your GitHub CLI login, or connect on the web at https://claude.ai/connect-github
```

[`/schedule`](/docs/ko/routines)로 루틴을 생성할 때 같은 메시지가 저장소를 이름 지정하는 설정 참고로 나타납니다. 참고는 루틴 생성을 차단하지 않습니다.

**해야 할 일:**

* `/web-setup`을 실행하여 GitHub CLI 로그인을 Claude 계정에 연결하거나 [claude.ai/connect-github](https://claude.ai/connect-github)에서 계정을 연결하세요. [GitHub 인증 옵션](/docs/ko/claude-code-on-the-web#github-authentication-options)을 참조하여 두 가지가 어떻게 다른지 확인하세요.
* 연결 1분 후 명령어를 다시 실행하세요.

v2.1.268 이전에는 Claude Code가 이를 Claude GitHub App 확인의 임시 실패로 보고했으며 재시도하거나 앱을 설치하도록 제안했습니다. 둘 다 GitHub 계정을 연결하지 않습니다.

<h3 id="single-sign-on-authorization-needed">
  단일 로그인 인증 필요
</h3>

[`/install-github-app`](/docs/ko/github-actions#quick-setup)을 실행했으며 SAML 단일 로그인을 적용하는 조직의 저장소를 선택했습니다. 설정 전에 Claude Code는 GitHub CLI로 저장소에 대한 액세스를 확인하며, GitHub는 `gh` 토큰이 아직 조직에 대해 인증되지 않았기 때문에 해당 확인을 거부했습니다. 마법사는 단계와 함께 경고를 표시합니다:

```text theme={null}
Single sign-on authorization needed
<owner>/<repo> belongs to an organization that enforces SAML single sign-on, and your GitHub CLI token isn't authorized for it yet.
```

**해야 할 일:**

* `repo` 및 `workflow` 범위로 GitHub CLI 로그인을 다시 인증하세요. `gh auth refresh -h github.com -s repo,workflow`를 실행하고 GitHub가 단일 로그인을 요청할 때 조직을 인증하세요.
* `GH_TOKEN`에서 개인 액세스 토큰으로 인증하면 [github.com/settings/tokens](https://github.com/settings/tokens)를 열고 토큰에서 **Configure SSO**를 선택한 후 조직을 인증하세요.
* `/install-github-app`을 다시 실행하세요.

v2.1.273 이전에는 Claude Code가 이 조건에 대해 `Admin permissions required` 경고를 표시했습니다.

<h3 id="failed-to-resume-the-conversation">
  대화를 재개할 수 없음
</h3>

Claude Code가 [`claude --resume` 선택기](/docs/ko/sessions#use-the-session-picker)에서 선택한 세션의 저장된 기록을 읽거나 처리할 수 없어서 부분적으로 로드된 상태에서 계속하는 대신 프로세스를 종료합니다. 메시지는 재시도 명령어를 포함합니다:

```text theme={null}
Failed to resume the conversation.
Run claude --resume <session-id> to retry, or claude to start a new session.
```

Claude Code는 메시지를 표시한 후 코드 1로 종료됩니다. 실행 중인 세션 내의 `/resume` 선택기는 대화에서 `Failed to resume conversation`을 보고하며 현재 세션은 계속 실행됩니다. v2.1.216 이전에는 `claude --resume` 선택기에서 실패한 재개가 `Resuming conversation…` 스피너에 무한정 머물렀습니다.

**해야 할 일:**

* 메시지의 세션 ID로 `claude --resume <session-id>`를 실행하여 재시도하세요.
* 재시도가 다시 실패하면 `claude update`를 실행하고 다시 재개하세요. v2.1.275 이전 버전은 저장된 기록에 읽을 수 없는 항목이 포함되어 있으면 재개에 실패합니다.
* 재시도가 다시 실패하면 `claude`를 실행하여 새 세션을 시작하세요.

<h3 id="no-conversation-found-with-the-session-id">
  세션 ID와 일치하는 대화를 찾을 수 없음
</h3>

세션 ID를 `claude --resume <session-id>`에 전달했는데 저장된 기록이 일치하지 않습니다:

```text theme={null}
No conversation found with session ID: <session-id>
```

Claude Code는 메시지를 표시한 후 코드 1로 종료됩니다. Claude Code는 [현재 프로젝트를 먼저 검색한 후 이 머신의 다른 모든 프로젝트를 검색](/docs/ko/sessions#resume-a-session)합니다. v2.1.223 이전에는 조회가 현재 프로젝트 디렉토리와 git worktree에서 중지되었으므로 세션이 마지막으로 작동한 디렉토리에서 재개하세요.

일반적인 원인:

* **잘못된 ID**: 비대화형 실행의 경우 ID는 [`--output-format json` 출력](/docs/ko/headless#get-structured-output)의 `session_id` 필드입니다.
* **삭제된 기록**: Claude Code는 [보존 기간](/docs/ko/sessions#where-transcripts-are-stored) 후 기록을 제거합니다. 기본값은 30일이며, [보존 스윕 규칙](/docs/ko/claude-directory#cleaned-up-automatically)을 따릅니다.
* **다른 머신**: Claude Code는 기록을 로컬로 저장하므로 세션이 실행된 머신에서 재개하세요.
* **중복 복사본**: 프로젝트 디렉토리를 `~/.claude/projects` 아래로 복사했으므로 두 기록이 같은 ID를 가지면 Claude Code는 이 메시지를 보고하는 대신 임의로 하나의 복사본을 재개합니다.

**해야 할 일:**

* 대화형 세션의 경우 [`claude --resume`](/docs/ko/sessions#use-the-session-picker)으로 [세션 선택기](/docs/ko/sessions#use-the-session-picker)를 열고 `Ctrl+A`를 눌러 이 머신의 모든 프로젝트로 확대한 후 세션을 선택하세요.
* `claude -p` 또는 [Agent SDK](/docs/ko/agent-sdk/overview)로 생성된 세션은 선택기에 나타나지 않으므로 원본 실행이 인쇄한 `session_id`에 대해 ID를 다시 확인하세요.

<h3 id="cannot-switch-renderers-in-this-session">
  이 세션에서 렌더러를 전환할 수 없음
</h3>

렌더러를 전환하면 Claude Code가 프로세스를 다시 시작합니다. Claude Code가 다시 시작하기를 거부하는 세션에서 [`/tui`](/docs/ko/fullscreen#enable-fullscreen-rendering)를 실행했으므로 전환되지 않고 아무것도 저장되지 않습니다. 어떤 메시지를 보는지는 원인을 알려줍니다:

* `Cannot switch renderers while work is running in the background`: 백그라운드 셸이나 하위 에이전트 같은 백그라운드에서 실행 중인 작업이 있습니다. 재시작이 이를 중단할 것입니다. 작업이 완료될 때까지 기다리거나 [`/tasks`](/docs/ko/commands)로 중지한 후 `/tui fullscreen` 또는 `/tui default`를 다시 실행하세요.
* `Cannot switch renderers in this session`: 세션에 다시 시작된 프로세스로 전달할 수 없는 제한이 있습니다. v2.1.234 이전에는 Claude Code가 어쨌든 다시 시작했으며 다시 시작된 세션이 없이 실행되었습니다.

제한 메시지에서 괄호 안의 부분이 Claude Code가 발견한 제한을 이름 지정합니다:

```text theme={null}
Cannot switch renderers in this session — it has restrictions a restart can't carry over (permission rules set for this session only). Nothing was changed. Running /tui fullscreen in a session started without them switches every later session too.
```

메시지가 괄호 안에 표시할 수 있는 각 이유:

* `launch flags: a custom system prompt, a tool allowlist, or restricted settings`: 다시 시작된 프로세스로 전달하지 않는 플래그로 세션을 시작했습니다. 이러한 플래그에는 [`--system-prompt`](/docs/ko/cli-reference#cli-flags), `--system-prompt-file`, `--append-system-prompt-file`, [`--tools`](/docs/ko/cli-reference#cli-flags) 허용 목록, [`--setting-sources`](/docs/ko/cli-reference#cli-flags) 및 [`--permission-prompt-tool`](/docs/ko/cli-reference#cli-flags)이 포함됩니다.
* `permission rules set for this session only`: 훅 또는 SDK 호출자의 [권한 업데이트](/docs/ko/hooks#permission-update-entries)가 `session` 대상으로 거부 또는 요청 규칙을 추가했습니다. 세션 범위 허용 규칙은 거부를 트리거하지 않습니다. 재시작이 이를 제거하고 Claude Code가 대신 다시 프롬프트합니다.
* `ask-before-running rules with no command-line form`: 훅 또는 SDK 호출자의 권한 업데이트가 Claude Code가 `--allowed-tools` 및 `--disallowed-tools`로 전달하는 규칙과 함께 요청 규칙을 추가했습니다. 요청 규칙에 대한 플래그는 없습니다.
* `permission rules a command line cannot carry intact` 및 `added directories a command line cannot carry intact`: 권한 업데이트가 세션 중간에 규칙 또는 디렉토리 경로를 추가했습니다. 다시 시작된 프로세스의 명령줄이 같은 값으로 텍스트를 전달할 수 없습니다.

**해야 할 일:**

* 이러한 제한 없이 시작된 세션에서 `/tui fullscreen`을 실행하거나 다시 전환하려면 `/tui default`를 실행하세요. Claude Code는 [`tui` 설정](/docs/ko/settings-reference#tui)을 거기에 저장합니다.

<h3 id="couldnt-open-claude-desktop">
  Claude Desktop을 열 수 없음
</h3>

[`/desktop`](/docs/ko/desktop#coming-from-the-cli) 또는 그 별칭 `/app`을 실행했는데 Claude Desktop을 열기 위해 Claude Code가 사용하는 시스템 명령어가 실패했습니다. 세션은 터미널에 남아 있습니다.

```text theme={null}
Error: Couldn't open Claude Desktop (`open` exited 1: LSOpenURLsWithRole() failed for the URL claude://resume?session=<session-id> with error -10814). Open Claude Desktop and run /desktop again.
```

**해야 할 일:**

* Claude Desktop을 직접 열고 `/desktop`을 다시 실행하세요.
* 해당 명령어의 전체 오류 출력을 읽으려면 `/debug`로 디버그 로깅을 켜고 `/desktop`을 다시 실행한 후 디버그 로그를 확인하세요.

v2.1.275 이전에는 메시지가 `Failed to open Claude Desktop. Please try opening it manually.`였으며 무엇이 실패했는지 말하지 않았습니다.

<h3 id="terminal-setup-left-your-zed-keymap-unchanged">
  /terminal-setup이 Zed 키맵을 변경하지 않음
</h3>

Zed에서 [`/terminal-setup`](/docs/ko/terminal-config#enter-multiline-prompts)을 실행했는데 Claude Code가 Zed `keymap.json`에 대한 업데이트를 완료할 수 없어서 파일을 그대로 두었습니다.

각 메시지는 키맵 경로를 이름 지정하고 직접 추가할 키바인딩 블록으로 끝납니다:

```text theme={null}
Couldn't update your Zed keymap, so it was left unchanged.
To add the binding yourself, add this block to the keymap array in <path to keymap.json>:
{ "context": "Terminal", "bindings": { "shift-enter": ["terminal::SendText", "\u001b\r"] } }
```

메시지의 첫 줄이 원인을 이름 지정합니다:

* `Couldn't read your Zed keymap, so it was left unchanged.`: Claude Code가 파일을 읽을 수 없습니다. 예를 들어 파일 권한 때문입니다.
* `Your Zed keymap isn't a readable list of keybindings, so it was left unchanged.`: 파일이 잘 읽혔지만 `//` 주석 및 후행 쉼표가 허용되어도 키바인딩 블록 배열로 파싱되지 않습니다.
* `Couldn't back up your Zed keymap; not modifying it.`: Claude Code가 파일을 옆에 `.bak` 백업으로 복사할 수 없어서 아무것도 변경하지 않았습니다.
* `Couldn't update your Zed keymap, so it was left unchanged.`: 병합된 결과가 바인딩을 전달하는 유효한 키맵으로 확인되지 않아서 Claude Code가 쓰는 대신 버렸습니다. 중복된 키가 있는 키바인딩 블록이 이를 유발할 수 있습니다.

**해야 할 일:**

* 메시지의 블록을 메시지가 이름 지정한 경로의 `keymap.json`의 최상위 배열에 복사하세요.
* `isn't a readable list of keybindings`의 경우 구문 오류를 수정하거나 파일의 최상위 값을 배열로 만든 후 `/terminal-setup`을 다시 실행하세요.

v2.1.247 이전에는 `/terminal-setup`이 `//` 주석 또는 후행 쉼표를 사용하는 Zed 키맵을 파싱할 수 없었으며, 설치된 것으로 보고하면서 자체 바인딩만으로 전체 파일을 바꿨습니다. 이전 버전이 바꾼 키맵을 복원하려면 [멀티라인 프롬프트 입력](/docs/ko/terminal-config#enter-multiline-prompts)에 설명된 `.bak` 백업 파일을 사용하세요.

<h3 id="skill-usage-reports-are-not-available-on-this-connection">
  이 연결에서 스킬 사용 보고서를 사용할 수 없음
</h3>

[Remote Control](/docs/ko/remote-control)을 통해 [`/skill-doctor`](/docs/ko/skills#find-unused-skills)를 실행했습니다. 휴대폰 또는 브라우저에서 실행했습니다. Claude Code는 Remote Control을 통해 스킬 사용 보고서를 보내지 않으며 대신 이 메시지로 응답합니다:

```text theme={null}
Skill usage reports are not available on this connection.
```

**해야 할 일:**

* 세션이 실행 중인 머신의 터미널에서 `/skill-doctor`를 실행하거나 거기서 `claude -p "/skill-doctor"`를 실행하세요.

<h3 id="custom-output-styles-cant-be-selected-over-remote-control">
  Remote Control 또는 릴레이된 메시지에서 사용자 정의 출력 스타일을 선택할 수 없음
</h3>

모바일 앱 또는 웹을 통해 [Remote Control](/docs/ko/remote-control)에서 [`/output-style`](/docs/ko/output-styles#change-your-output-style)을 실행했거나, 명령어가 세션으로 릴레이된 메시지에 도착했습니다. 이러한 턴은 계정 소유자에게서 오지 않을 수 있으므로 Claude Code는 [기본 제공 스타일](/docs/ko/output-styles#built-in-output-styles)만 나열하고 선택합니다. [사용자 정의 스타일](/docs/ko/output-styles#create-a-custom-output-style) 이름은 존재하지 않는 이름과 같은 응답을 받습니다:

```text theme={null}
Custom output styles can't be selected over Remote Control or from a relayed message. Select one in the session itself, or pick a built-in style here.
```

**해야 할 일:**

* 기본 제공 스타일을 선택하세요. 예를 들어 `/output-style concise`
* 사용자 정의 스타일을 사용하려면 프로젝트의 `.claude/settings.local.json`에서 [`outputStyle`](/docs/ko/settings-reference#outputstyle)을 설정하거나, 세션이 자체 터미널을 가지고 있으면 거기서 `/output-style <style>`을 실행하세요.

<h3 id="output-styles-are-saved-to-local-settings-which-this-session-doesnt-load">
  출력 스타일이 이 세션이 로드하지 않는 로컬 설정에 저장됨
</h3>

`/output-style <style>` 또는 `/config outputStyle=<style>`으로 [출력 스타일](/docs/ko/output-styles)을 전환하려고 했습니다. 설정 소스가 `local`을 제외하는 세션에서 실행했습니다. 예를 들어 [`settingSources`](/docs/ko/agent-sdk/typescript#options)가 `"local"`을 생략하는 [Agent SDK](/docs/ko/agent-sdk/typescript) 세션과 [`--setting-sources`](/docs/ko/cli-reference#cli-flags) 값이 `local`을 생략하는 CLI 세션입니다. 두 명령어 모두 스타일을 `.claude/settings.local.json`에 저장합니다. 이러한 세션은 절대 이 파일을 다시 읽지 않으므로 Claude Code는 효과가 없을 설정을 쓰는 대신 거부합니다:

```text theme={null}
Output styles are saved to local settings (.claude/settings.local.json), which this session doesn't load, so the style can't be changed here.
```

**해야 할 일:**

* 세션의 설정 소스에 `local`을 추가하고 다시 전환하세요.
* 세션이 로드하는 설정 파일(예: 프로젝트의 `.claude/settings.json` 또는 `~/.claude/settings.json`)에서 [`outputStyle`](/docs/ko/settings-reference#outputstyle) 키를 설정하세요. TypeScript SDK에서는 대신 인라인 `settings` 객체 내에 `outputStyle`을 설정하세요. [출력 스타일 활성화](/docs/ko/agent-sdk/modifying-system-prompts#activate-an-output-style)를 참조하세요.

<h2 id="plugin-errors">
  플러그인 오류
</h2>

이러한 오류는 [플러그인](/docs/ko/plugins/overview) 및 [마켓플레이스](/docs/ko/plugins/overview) 구성에서 발생합니다. 이 페이지의 메시지 중 하나를 생성하지 않는 플러그인 문제(예: 로드되지 않는 마켓플레이스 URL 또는 설치되지만 나타나지 않는 플러그인)의 경우 [플러그인 문제 해결](/docs/ko/plugins/troubleshooting)을 참조하십시오.

<h3 id="plugin-eval-is-currently-in-early-access">
  plugin eval is currently in early access
</h3>

[`claude plugin eval`](/docs/ko/plugin-evals) 또는 `claude plugin eval init`을 실행했으며 아무것도 수행하기 전에 다음 메시지 중 하나와 함께 종료 코드 1로 종료되었습니다:

```text theme={null}
`plugin eval` is currently in early access
```

```text theme={null}
`plugin eval` is currently unavailable
```

첫 번째 메시지는 빌드가 v2.1.269보다 오래되었음을 의미합니다. v2.1.269는 명령이 일반적으로 사용 가능한 첫 번째 버전입니다. 두 번째 메시지는 Anthropic이 서버 측에서 명령을 비활성화했음을 의미합니다. 머신의 아무것도 이를 다시 켤 수 없습니다.

**수행할 작업:**

* `claude --version`을 실행한 다음 `claude update`를 실행하고 새 세션에서 명령을 다시 실행하십시오. [플러그인 평가 요구 사항](/docs/ko/plugin-evals#requirements)을 참조하십시오.
* 현재 빌드에서 두 번째 메시지가 표시되면 다른 `claude update` 후 나중에 다시 시도하십시오.

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace is registered from an untrusted source
</h3>

마켓플레이스가 [공식 Anthropic 마켓플레이스용으로 예약된](/docs/ko/plugins/marketplace-reference#marketplace-file) 이름으로 등록되어 있지만 등록된 소스가 `anthropics` GitHub 저장소가 아닙니다. Claude Code는 마켓플레이스를 로드하거나 새로 고칠 때마다 예약된 이름을 다시 확인하므로 마켓플레이스와 여기서 설치된 플러그인이 로드되지 않습니다. v2.1.205 이전에는 마켓플레이스가 추가될 때만 이름이 확인되었으므로 이름이 예약되기 전에 등록된 항목이 계속 로드되었습니다.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

소스가 GitHub 저장소 또는 Git URL이 아닌 마켓플레이스(예: 로컬 디렉터리)의 경우 중간 문장은 `can only be used with GitHub sources from the 'anthropics' organization` 대신 읽습니다. `claude plugin marketplace add`는 동일한 확인을 실행하며 예약된 이름을 `Failed to add marketplace:` 다음에 동일한 예약된 이름 문장으로 거부합니다.

**수행할 작업:**

* 마켓플레이스가 이미 등록된 경우 `claude plugin marketplace remove <name>`을 실행한 다음 공식 `github.com/anthropics` 저장소에서 다시 추가하십시오.
* 이름이 예약되기 전에 이름을 사용한 타사 마켓플레이스를 게시하는 경우 이름을 바꾸고 사용자에게 소스에서 다시 추가하도록 요청하십시오.
* [마켓플레이스 스키마](/docs/ko/plugins/marketplace-reference#marketplace-file)에서 예약된 이름 목록을 참조하십시오.

<h3 id="marketplace-name-is-another-spelling-of-a-reserved-name">
  Marketplace name is another spelling of a reserved name
</h3>

마켓플레이스의 이름 자체는 예약된 이름이 아니지만 Claude Code는 이를 예약된 이름의 다른 철자로 취급합니다. [예약된 이름](/docs/ko/plugins/marketplace-reference#reserved-name-spellings)은 어떤 철자가 예약된 이름으로 간주되는지 나열합니다. Claude Code는 마켓플레이스를 추가할 때 이러한 이름을 거부합니다:

```text theme={null}
Failed to add marketplace: "claude.code.plugins" is another spelling of "claude-code-plugins", a reserved marketplace name.
```

마켓플레이스가 이미 이러한 이름으로 등록된 경우 해당 항목이 로드되지 않으며 `/plugin`, `claude plugin install` 및 `claude plugin update`는 경고합니다:

```text wrap theme={null}
known_marketplaces.json has an entry named "claude.code.plugins", another spelling of the reserved marketplace name "claude-code-plugins", so it is ignored. Remove it with: claude plugin marketplace remove claude.code.plugins
```

이름에 셸 인용이 필요한 경우 추가 시간 거부는 `This marketplace's name is another spelling of "<reserved>", a reserved marketplace name. It is not exactly the reserved name it appears to be.`로 읽습니다.

**수행할 작업:**

* 마켓플레이스의 이름을 예약된 이름의 철자를 지정하지 않는 이름으로 바꾸고 다시 추가하십시오.
* 무시된 항목 경고의 경우 제공하는 `claude plugin marketplace remove` 명령을 실행하거나 `~/.claude/plugins/known_marketplaces.json`에서 항목을 제거하십시오.

<h3 id="marketplace-is-already-added-from-a-different-source">
  Marketplace is already added from a different source
</h3>

[`/plugin install <plugin> --marketplace <source>`](/docs/ko/plugins/install#add-a-marketplace-and-install-in-one-command)를 통해 마켓플레이스 추가를 확인했으며 해당 소스에서 Claude Code가 가져온 카탈로그가 이미 다른 소스에서 추가한 마켓플레이스와 동일한 이름으로 지정합니다. Claude Code는 기존 마켓플레이스를 유지하고 이를 대체하지 않으므로 플러그인이 설치되지 않습니다.

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

**수행할 작업:**

* 이미 추가한 마켓플레이스가 원하는 마켓플레이스인 경우 이름으로 설치하십시오: `/plugin install <plugin>@<name>`
* 새 소스로 전환하려면 `/plugin marketplace remove <name>`을 실행한 다음 설치를 다시 시도하십시오.

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

플러그인 훅, [모니터](/docs/ko/plugins/components#monitors) 또는 MCP [`headersHelper`](/docs/ko/mcp#use-dynamic-headers-for-custom-authentication) 명령이 `${user_config.KEY}` [플러그인 옵션](/docs/ko/plugins/manifest-reference#user-configuration)을 참조하며 대체된 문자열이 셸에 전달됩니다. 구성된 값에 `$(...)`, 백틱 또는 `;`이 포함되면 여기서 코드로 실행되므로 Claude Code는 값을 대체하는 대신 구성 요소를 시작하기를 거부합니다. 확인은 명령 템플릿에서 실행되므로 아직 값이 구성되지 않았을 때도 오류가 나타납니다. v2.1.207 이전에는 값이 셸 명령으로 대체되었습니다.

표현은 어느 표면이 옵션을 참조했는지에 따라 다릅니다. 셸 형식 훅은 다음을 보고합니다:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

모니터는 다음을 보고합니다:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

MCP `headersHelper`는 다음을 보고합니다:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**수행할 작업:**

* 훅의 경우 `args` 배열을 추가하여 [exec 형식](/docs/ko/hooks#exec-form-and-shell-form)에서 실행되도록 하십시오. 여기서 각 `${user_config.KEY}`는 그 사이에 셸이 없는 하나의 인수가 됩니다. 또는 참조를 제거하고 스크립트 내에서 `$CLAUDE_PLUGIN_OPTION_<KEY>` 환경 변수를 읽으십시오.
* 모니터의 경우 참조를 제거하고 모니터 스크립트가 구성 파일에서 값을 읽도록 하십시오.
* `headersHelper`의 경우 `${user_config.KEY}`를 셸 구문 분석되지 않는 서버의 `headers` 필드로 이동하거나 헬퍼 스크립트 내에서 값을 읽으십시오.

<h3 id="plugin-archive-integrity-check-failed">
  Plugin archive integrity check failed
</h3>

플러그인의 마켓플레이스 항목이 `sha256` 핀이 있는 [`archive` 소스](/docs/ko/plugins/marketplace-reference#archive-plugin-source)를 사용하며 다운로드된 파일의 다이제스트가 핀과 일치하지 않습니다. Claude Code는 설치를 거부하므로 플러그인 캐시에서 아무것도 변경되지 않습니다. 불일치에는 세 가지 가능한 원인이 있습니다:

* 작성자가 핀을 계산한 후 URL의 파일이 변경됨
* 작성자가 마켓플레이스 항목에 잘못된 다이제스트를 입력함
* URL이 작성자가 핀한 파일과 다른 파일을 제공함

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

**수행할 작업:**

* 플러그인을 게시하는 경우 URL이 제공하는 정확한 파일의 다이제스트를 다시 계산하십시오. 예를 들어 `shasum -a 256 my-plugin.zip` 또는 PowerShell에서 `Get-FileHash -Algorithm SHA256 my-plugin.zip`을 사용하고 마켓플레이스 항목의 `sha256`을 업데이트하십시오.
* 플러그인을 설치하는 경우 `/plugin marketplace update <name>`을 실행하여 항목이 수정된 경우 카탈로그를 새로 고친 다음 설치를 다시 시도하십시오.
* 새로 고침 후에도 다이제스트가 계속 불일치하면 설치하기 전에 마켓플레이스 소유자에게 어떤 파일을 핀했는지 물어보십시오.

<h3 id="path-escapes-plugin-directory">
  Path escapes plugin directory
</h3>

플러그인 구성 요소 경로는 플러그인의 `plugin.json` 또는 [마켓플레이스 항목](/docs/ko/plugins/marketplace-reference#plugin-entries)에서 선언되며 플러그인의 자체 디렉터리 외부로 확인됩니다. Claude Code는 해당 경로를 삭제하고 플러그인의 나머지 부분을 로드합니다. 메시지의 구성 요소 이름(예: `commands` 또는 `hooks`)은 경로를 선언한 필드의 이름을 지정합니다.

```text theme={null}
commands path escapes plugin directory: ./../shared.md
```

`claude plugin` 명령 출력에서 동일한 오류는 `Path escapes plugin directory: ./../shared.md (commands)`로 읽습니다.

Claude Code는 `../shared-utils`와 같이 플러그인 외부를 가리키는 경로와 [마켓플레이스 심볼릭 링크 규칙](/docs/ko/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)이 허용하지 않는 플러그인 외부로 이어지는 심볼릭 링크를 모두 거부합니다. 심볼릭 링크의 경우 메시지는 경로가 확인되는 위치도 표시합니다:

```text theme={null}
commands path escapes plugin directory: ./commands/deploy.md — it resolves to /home/user/shared/deploy.md, outside the plugin directory
```

macOS 및 Linux에서 Claude Code는 경로가 플러그인 내부에 머물러 있을 때도 경로의 어디든지 백슬래시를 포함하는 구성 요소 경로를 거부합니다. 구성 요소 경로가 Windows 스타일 구분 기호를 사용하는 플러그인은 Windows에서 로드되고 다른 플랫폼에서 이 거부를 트리거합니다:

```text theme={null}
commands path escapes plugin directory: ./commands\deploy.md — its path contains a backslash, which is not resolved reliably on this platform
```

v2.1.251 이전에는 Claude Code가 마켓플레이스 항목에서 선언된 `commands` 경로를 플러그인 디렉터리 외부를 가리킬 때도 로드했습니다. Claude Code는 이미 `plugin.json`에서 선언된 경로와 마켓플레이스 항목의 다른 구성 요소 경로를 거부했습니다.

v2.1.257 이전에는 확인이 경로의 철자만 보았으며 심볼릭 링크가 이어지는 위치는 보지 않았습니다.

**수행할 작업:**

* 참조된 파일을 플러그인 디렉터리 내부로 이동하고 `./` 상대 경로로 경로를 가리키십시오.
* 경로가 플러그인 외부의 파일에 대한 심볼릭 링크인 경우 심볼릭 링크를 파일의 복사본으로 바꾸십시오.
* 메시지가 경로에 백슬래시가 포함되어 있다고 말하면 경로를 정방향 슬래시로 작성하십시오. 예를 들어 `./commands/deploy.md`
* 동일한 마켓플레이스의 다른 플러그인과 파일을 공유하려면 플러그인 디렉터리 내의 심볼릭 링크로 연결하고 [심볼릭 링크 규칙](/docs/ko/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)을 따르십시오.

<h3 id="path-could-not-be-checked">
  Path could not be checked
</h3>

Claude Code는 플러그인 경로가 존재하는지 운영 체제에 물었고 "찾을 수 없음" 이외의 오류를 받았으므로 경로가 이름을 지정하는 것을 로드하지 않습니다. 플러그인의 얼마나 많은 부분이 로드되는지는 어느 경로가 실패했는지에 따라 다릅니다:

* 플러그인의 [기본 구성 요소 위치](/docs/ko/plugins/manifest-reference#standard-layout) 중 하나(예: `skills/` 폴더, `monitors/monitors.json` 파일 또는 [플러그인 루트의 `SKILL.md`](/docs/ko/plugins/components#skills)): 플러그인의 다른 구성 요소는 여전히 로드됨
* 플러그인의 자체 디렉터리: 해당 플러그인에서 아무것도 로드되지 않음

전혀 존재하지 않는 경로에 대해서는 이 오류가 표시되지 않습니다. `/plugin`에서 오류는 플러그인 아래에 나타나고 경로와 운영 체제가 반환한 코드의 이름을 지정합니다:

```text theme={null}
skills path could not be checked: /home/user/my-plugin/skills (ELOOP)
```

`claude plugin list`에서 동일한 오류는 `Path not found: /home/user/my-plugin/skills (skills, ELOOP)`로 읽습니다.

이 오류를 생성하는 원인은 다음을 포함합니다:

* `ELOOP`: 경로의 심볼릭 링크가 자신을 가리키거나 루프를 형성함
* `EIO` 또는 `ESTALE`: 경로가 끊어지거나 오래된 네트워크 마운트에 있음
* `EACCES`: 경로 위의 디렉터리 중 하나가 이를 통과할 권한을 거부함

**수행할 작업:**

* 자신을 가리키는 심볼릭 링크를 실제 폴더로 바꾸거나 삭제하십시오.
* 경로가 네트워크 마운트에 있으면 공유를 다시 마운트하십시오.
* 코드가 `EACCES`이면 경로 위의 디렉터리에 대한 실행 권한을 복원하십시오.
* 경로를 수정한 후 `/reload-plugins`을 실행하거나 Claude Code를 다시 시작하여 플러그인 또는 구성 요소를 로드하십시오.

v2.1.265 이전에는 Claude Code가 확인할 수 없는 기본 구성 요소 폴더를 없는 것으로 취급하고 오류 없이 해당 구성 요소 없이 플러그인을 로드했습니다.

<h3 id="marketplace-entry-path-does-not-stay-inside-the-marketplace-directory">
  Marketplace entry path does not stay inside the marketplace directory
</h3>

플러그인의 [마켓플레이스 항목](/docs/ko/plugins/marketplace-reference#plugin-entries)이 Claude Code가 마켓플레이스의 자체 디렉터리 내부의 위치로 확인할 수 없는 소스 경로를 선언하므로 플러그인이 설치되거나 로드되지 않습니다. 거부는 다음을 포함합니다:

* 절대 경로, `..`로 마켓플레이스를 벗어나거나 네트워크 경로처럼 철자된 항목 경로
* macOS 및 Linux에서 선행 `./` 이후 어디든지 백슬래시를 포함하는 항목 경로
* git 또는 URL과 같은 원격 소스에서 가져온 마켓플레이스의 항목이 마켓플레이스 디렉터리 외부로 확인되는 심볼릭 링크를 통해 대상에 도달함
* 마켓플레이스의 `marketplace.json`에 대한 직접 URL에서 추가된 마켓플레이스의 상대 항목: Claude Code는 해당 파일만 다운로드하므로 경로가 이름을 지정할 로컬 플러그인 파일이 없습니다. [URL 기반 마켓플레이스에서 상대 경로가 있는 플러그인 실패](/docs/ko/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)를 참조하십시오.

`claude plugin install`은 다음과 같이 거부를 보고합니다:

```text theme={null}
Cannot install my-plugin@my-marketplace: its marketplace entry path does not stay inside the marketplace directory (an absolute, climbing, network-shaped, backslash-containing or link-traversing entry, an entry of a fetched marketplace that resolves or opens outside its tree — or a relative entry in a url-catalog marketplace, which has no local directory)
```

이미 설치된 플러그인의 항목이 동일한 확인에 실패하면 `claude plugin list`는 플러그인을 `failed to load`로 표시합니다:

```text theme={null}
Plugin source path refused: ./my-plugin does not stay inside its marketplace directory. Check that the marketplace entry has a plain relative path.
```

**수행할 작업:**

* 마켓플레이스를 유지하는 경우 항목의 `source`를 `./plugins/my-plugin`과 같은 일반 상대 경로로 작성하고 이를 통과하는 모든 심볼릭 링크가 마켓플레이스 디렉터리 내부를 가리키도록 하십시오.
* 마켓플레이스를 직접 URL에서 추가한 경우 상대 항목을 확인할 수 없습니다. 마켓플레이스 작성자에게 [다른 플러그인 소스](/docs/ko/plugins/marketplace-reference#plugin-sources)를 사용하도록 요청하거나 git 저장소에서 마켓플레이스를 추가하십시오.

<h3 id="failed-to-load-marketplace-configuration">
  Failed to load marketplace configuration
</h3>

Claude Code는 `~/.claude/plugins/known_marketplaces.json`의 레지스트리 파일에 추가한 플러그인 마켓플레이스를 유지합니다. `claude plugin install`과 같이 레지스트리가 필요한 플러그인 명령은 Claude Code가 파일을 사용할 수 없을 때 두 가지 메시지 중 하나로 실패합니다:

* `Failed to load marketplace configuration`: 파일이 유효한 JSON이 아니거나 읽을 수 없습니다. 빈 파일도 이런 식으로 실패합니다.
* `Marketplace configuration file is corrupted`: 파일이 유효한 JSON이지만 내용이 레지스트리 스키마와 일치하지 않습니다.

누락된 파일은 실패가 아닙니다: Claude Code는 이를 마켓플레이스가 없는 레지스트리로 취급합니다.

빈 파일의 경우 `claude plugin install`은 다음을 보고합니다:

```text theme={null}
✘ Failed to install plugin "my-plugin": Failed to load marketplace configuration: JSON Parse error: Unexpected EOF
```

v2.1.246 이전에는 `claude plugin install`이 이 실패를 보고하지 않았습니다.

**수행할 작업:**

* `~/.claude/plugins/known_marketplaces.json`을 열고 JSON을 복구하거나 메시지가 레지스트리 스키마와 일치하지 않는 것으로 이름을 지정한 항목을 수정하십시오.
* 복구할 수 없으면 파일을 삭제하거나 내용을 `{}`로 바꾼 다음 `claude plugin marketplace add <source>`로 각 마켓플레이스를 다시 추가하십시오. Claude Code는 신뢰한 폴더에서 다음에 시작할 때 사용자 또는 관리 설정이 [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces)에서 선언한 마켓플레이스를 다시 등록합니다.

<h3 id="plugin-is-required-by-your-organization">
  Plugin is required by your organization
</h3>

`claude plugin disable`을 실행했거나 `/plugin` **설치됨** 탭을 사용하여 조직이 필수로 표시한 [claude.ai에서 동기화된](/docs/ko/plugins/loading#synced-plugins) 플러그인을 비활성화하려고 했습니다:

```text theme={null}
Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.
```

Claude Code는 아무것도 저장하지 않으며 플러그인은 활성화된 상태로 유지됩니다.

필수 플러그인이 의존하는 플러그인을 비활성화하려고 하면 Claude Code는 필수 플러그인이 필요로 하는 플러그인의 이름을 지정하는 메시지와 함께 동일한 방식으로 거부합니다.

**수행할 작업:**

* claude.ai 조직의 관리자에게 claude.ai에서 플러그인의 필수 상태를 변경하도록 요청하십시오.

<h2 id="tool-errors">
  도구 오류
</h2>

이러한 오류는 Claude의 기본 제공 도구에서 발생합니다. Claude는 대부분의 도구 오류를 자동으로 수정합니다. 변경이 필요한 경우 해당 오류의 **할 일** 목록에 변경할 사항이 명시되어 있습니다.

<h3 id="agent-would-be-spawned-with-zero-tools">
  에이전트가 도구 없이 생성됨
</h3>

서브에이전트의 [`tools` 목록](/docs/ko/sub-agents#supported-frontmatter-fields)의 모든 항목이 사용 가능한 도구와 일치하지 않아 Claude Code가 서브에이전트 시작을 거부했습니다. 도구가 없으면 작동할 수 없기 때문입니다. 메시지는 항목을 잘못된 이유별로 그룹화합니다:

* **인식되지 않음**: 항목이 도구 이름과 일치하지 않으며, 보통 `Grep`을 `Grpe`로 잘못 입력한 것과 같은 오타입니다.
* **서브에이전트에서 사용 불가**: 항목이 [서브에이전트가 사용할 수 없는](/docs/ko/sub-agents#available-tools) 실제 도구를 지정합니다. 백그라운드 서브에이전트는 더 작은 기본 제공 도구 세트를 유지하므로, 포그라운드 서브에이전트만 사용할 수 있는 항목은 서브에이전트가 기본값인 백그라운드에서 실행될 때 여기에 표시됩니다. `Agent`를 나열하면 메시지가 대신 다음 그룹 아래에 보고됩니다.
* **이 세션의 도구와 일치하지 않음**: 항목은 유효하지만 현재 세션의 도구가 지금 일치하지 않습니다. 예를 들어 GitHub MCP 서버가 연결되지 않은 `mcp__github__*` 또는 [깊이 제한](/docs/ko/sub-agents#let-subagents-spawn-their-own-subagents)에 있는 서브에이전트의 `Agent`입니다.

`tools` 필드를 생략하면 이 거부가 트리거되지 않습니다. `tools` 목록을 비워두거나 `disallowedTools`가 목록의 모든 항목을 제거하면 Claude Code도 거부를 건너뛰고 도구 없이 서브에이전트를 시작합니다.

v2.1.208 이전에는 서브에이전트가 도구 없이 시작되었고 빈 결과 또는 혼란스러운 결과를 반환할 수 있었습니다.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**할 일:**

* 오류가 지정하는 각 항목을 [서브에이전트에서 사용 가능한 도구](/docs/ko/sub-agents#available-tools)에 대해 수정합니다.
* 세션에 없는 도구의 항목을 제거합니다. 예를 들어 연결되지 않은 서버의 MCP 도구입니다.
* [백그라운드 서브에이전트가 제거하는](/docs/ko/sub-agents#available-tools) 도구(예: `CronCreate`)의 경우 항목을 제거합니다. 도구를 유지하려면 [포크 모드를 끕니다](/docs/ko/sub-agents#turn-fork-mode-on-or-off) 그리고 Claude에 서브에이전트를 포그라운드에서 실행하도록 요청합니다.
* 도구를 나열하는 대신 `tools` 필드를 삭제하여 서브에이전트에 [서브에이전트에서 사용 가능한 모든 도구](/docs/ko/sub-agents#available-tools)를 제공합니다.
* `Agent`만 포함하는 `tools` 목록의 경우 [깊이 제한](/docs/ko/sub-agents#let-subagents-spawn-their-own-subagents)을 높이거나 에이전트에 최소한 하나의 다른 도구를 제공합니다. Claude Code는 해당 제한에서 `Agent`를 보류하므로 다른 항목이 없는 목록은 도구 없음으로 해석됩니다.

<h3 id="file-is-covered-by-a-read-deny-rule">
  파일이 Read 거부 규칙으로 적용됨
</h3>

Edit 또는 Write 도구가 [`Read` 거부 규칙](/docs/ko/permissions#read-and-edit)과 일치하는 경로에서 호출되었습니다. 여기에는 해당 경로에 새 파일을 만드는 것도 포함됩니다. 두 도구 모두 Claude가 다시 읽을 수 있어야 하는 콘텐츠를 변경하므로 Claude Code는 파일 액세스 전에 호출을 거부합니다. NotebookEdit은 `Read` 거부 규칙의 적용을 받지 않습니다. v2.1.228 이전에는 규칙이 Edit 도구만 차단했고, v2.1.208 이전에는 `Edit` 거부 규칙만 편집을 차단했습니다.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

Claude Code가 Write 도구를 거부할 때 메시지는 대신 `and cannot be written`으로 끝납니다.

**할 일:**

* Claude가 파일을 변경할 수 있어야 하면 `/permissions`의 `Read` 거부 규칙을 제거하거나 좁힙니다. 또는 [설정](/docs/ko/settings-reference#permission-settings)에서 제거합니다.
* 파일이 그대로 유지되어야 하면 규칙을 유지하고 NotebookEdit 도구도 차단하기 위해 동일한 경로에 `Edit` 거부 규칙을 추가합니다.

<h3 id="subagent-type-is-required">
  subagent\_type이 필수입니다
</h3>

```text theme={null}
subagent_type is required: the general-purpose agent is not available in this session. Available agents: ...
```

Claude가 `subagent_type` 없이 [Agent 도구](/docs/ko/tools-reference#agent-tool-behavior)를 호출했고, 이 세션에는 폴백할 [범용 서브에이전트](/docs/ko/sub-agents#built-in-subagents)가 없습니다. 이는 두 가지 설정에서 발생합니다:

* [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/ko/env-vars)이 비대화형 모드에서 설정되어 모든 기본 제공 서브에이전트를 제거합니다.
* 세션의 메인 스레드 에이전트에 `general-purpose`를 제외하는 [`tools: Agent(...)` 허용 목록](/docs/ko/sub-agents#restrict-which-subagents-can-be-spawned)이 있습니다.

**할 일:**

* 보통 아무것도 하지 않습니다: 메시지는 세션이 가진 서브에이전트를 나열하므로 Claude는 그 중 하나로 다시 시도할 수 있습니다.
* Claude가 계속 실패하면 `tools: Agent(...)` 허용 목록에 `general-purpose`를 추가하거나 `CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS`를 설정 해제합니다.

v2.1.235 이전에는 동일한 호출이 `Agent type 'general-purpose' not found`로 실패했습니다.

<h3 id="memory-index-is-over-its-read-limit">
  메모리 인덱스가 읽기 제한을 초과함
</h3>

Claude가 [자동 메모리](/docs/ko/memory#auto-memory) 인덱스 `MEMORY.md`에 썼고 읽기 제한 중 하나를 초과했습니다: 200줄 또는 25KB입니다. 쓰기는 성공했지만 처음 200줄 또는 25KB(둘 중 먼저 오는 것)만 세션 시작 시 로드되므로 제한을 초과하는 모든 것은 인덱스를 읽을 때마다 삭제됩니다. v2.1.210 이전에는 제한을 초과하는 인덱스가 다음 로드 시 쓰기 시간 신호 없이 자동으로 잘렸습니다.

```text theme={null}
Error: this write left the memory index at MEMORY.md at 214 lines, over its 200-line read limit. The write succeeded, but everything past the limit is silently dropped each time the index is loaded — entries at the end are already invisible to readers. Rewrite it to under 140 lines now: keep one line per entry, move detail into topic files, and merge or drop stale entries.
```

로드되는 콘텐츠만 제한에 포함됩니다. YAML frontmatter 및 블록 수준 HTML 주석은 인덱스가 로드되기 전에 제거되므로 측정에서 제외됩니다. v2.1.211 이전에는 Claude Code가 원본 파일을 측정했고 frontmatter 또는 주석이 로드된 콘텐츠가 맞아도 이 오류를 트리거할 수 있었습니다.

Claude Code는 터미널에 배너로 인쇄하는 대신 쓰기 후 Claude에 오류를 전달하므로 트랜스크립트에서만 알아차릴 수 있습니다.

Claude의 쓰기가 파일을 제한에 가깝게 가져가지만 넘지 않으면 Claude Code는 이 오류 대신 인덱스를 압축하라는 더 온화한 알림을 반환합니다.

**할 일:**

* Claude가 `MEMORY.md`를 다시 쓰도록 하거나 요청합니다: 항목당 한 줄을 유지하고, 세부 정보를 주제 파일로 이동하고, 오래된 항목을 병합하거나 삭제합니다.
* 인덱스를 직접 정리하려면 [메모리 감사 및 편집](/docs/ko/memory#audit-and-edit-your-memory)을 참조합니다.

<h3 id="pkill-pattern-matches-the-claude-code-process">
  pkill 패턴이 Claude Code 프로세스와 일치함
</h3>

Bash 도구 호출의 `pkill` 명령이 패턴(보통 `-f` 포함)을 사용했고 Claude Code 프로세스 자체와 일치하므로 Claude Code는 세션을 종료하는 대신 명령을 거부합니다. Claude Code는 `pkill`을 실행하기 전에 `pgrep`으로 패턴을 테스트하고 자신의 프로세스 ID가 결과에 있으면 거부합니다. 확인은 Linux에서만 실행됩니다. macOS에서는 `pkill`이 수정되지 않고 실행됩니다. v2.1.214 이전에는 명령이 실행되었고 일치하는 패턴이 Claude Code 세션을 턴 중간에 종료했습니다.

```text theme={null}
pkill: refusing to run — this pattern matches the Claude CLI process (PID 12345). Narrow the pattern, or target your own children with `pkill -P $$ ...`.
```

거부는 터미널의 배너가 아닌 Bash 도구 결과에 나타나며 Claude는 보통 명령을 자동으로 조정합니다.

**할 일:**

* 패턴을 좁혀서 의도한 프로세스만 일치하도록 합니다. 예를 들어 짧은 부분 문자열이 아닌 대상 바이너리의 전체 경로입니다.
* 현재 셸에서 시작한 프로세스를 중지하려면 패턴과 함께 `pkill -P $$`를 사용합니다. 이는 일치를 셸의 자식 프로세스로 제한합니다.

<h3 id="failed-to-write-to-a-teammate-inbox">
  팀원의 받은편지함에 쓰기 실패
</h3>

Claude Code가 `~/.claude/teams/{team-name}/inboxes/` 아래의 팀원 메일박스 파일에 메시지를 쓸 수 없어서 수신자가 아무것도 받지 못했습니다. 쓰기는 Claude Code가 파일을 만들거나 업데이트할 수 없을 때 실패합니다. 예를 들어 디스크가 가득 찼거나, 디렉토리를 쓸 수 없거나, 다른 에이전트가 받은편지함 잠금을 너무 오래 유지하는 경우입니다. v2.1.224 이전에는 Claude Code가 쓰기가 실패했을 때도 메시지를 보낸 것으로 보고했습니다.

오류는 터미널의 배너가 아닌 전송 에이전트의 도구 결과에 나타나며 Claude에 다시 시도하도록 지시합니다:

```text theme={null}
Failed to write to researcher's inbox — nothing was sent. Try again, or message the lead.
```

구조화된 [에이전트 팀](/docs/ko/agent-teams) 프로토콜 메시지는 동일한 방식으로 실패하며 오류는 전달되지 않은 메시지의 이름을 지정합니다: Claude Code가 계획 승인, 계획 거부, 종료 요청 또는 종료 거부를 쓸 수 없을 때 오류는 `Failed to write the <message> to <name>'s inbox — nothing was sent`로 읽습니다. 해당 목록의 `plan approval`은 팀원의 계획을 승인하는 리드의 결정입니다. 팀원의 계획 제출은 별도의 `plan approval request` 메시지입니다. 해당 메시지와 두 개의 다른 프로토콜 메시지는 자신의 메시지 텍스트와 결과를 전달합니다:

* `Failed to write the plan approval request to the lead's inbox — plan not submitted; try again`: 팀원의 계획이 리드에 도달하지 않았고 팀원은 재제출이 성공할 때까지 계획 모드에 남아 있습니다.
* `The permission request could not be delivered to the team lead (mailbox write failed)`: 팀원의 권한 요청이 리드에 도달하지 않았으므로 아무도 도구 호출을 승인하지 않았습니다.
* `The confirmation could not be written to team-lead's inbox.`: 종료 승인 자체가 적용되었고 팀원이 종료됩니다. 리드에 대한 확인만 누락되었습니다.

리드 세션에서 `@name`을 입력한 후 메시지를 입력하여 팀원에게 직접 메시지를 보낼 때 동일한 실패가 알림으로 나타납니다: `Couldn't write to @name's inbox — message not sent. Try again.` Claude Code는 텍스트를 프롬프트 상자에 유지하므로 다시 보낼 수 있습니다.

**할 일:**

* 발신자에게 메시지를 다시 보내도록 요청합니다. 받은편지함 잠금에 대한 경합은 일시적이며 재시도 시 해결됩니다.
* 여유 디스크 공간을 확인하고 `~/.claude/teams` 및 그 아래의 파일이 사용자가 쓸 수 있는지 확인합니다.

<h3 id="teammate-agent-definition-not-restored">
  팀원의 에이전트 정의가 복원되지 않음
</h3>

Claude가 중지된 [에이전트 팀](/docs/ko/agent-teams) 팀원에게 메시지를 보냈고 Claude Code가 [서브에이전트 정의](/docs/ko/agent-teams#use-subagent-definitions-for-teammates)를 다시 적용하지 않고 복구했습니다. 정의 파일이 저장된 신뢰가 없는 폴더에서 나왔기 때문입니다. 알림은 전송 에이전트의 도구 결과에서 재개 보고서를 따릅니다:

```text wrap theme={null}
Its agent definition was not restored: the folder its definition file came from is not trusted (source: projectSettings), so the teammate is running with the team-essential tools and no custom instructions. To restore it, the user needs to run Claude Code in that folder once and accept the trust dialog (the --debug log names the folder); do not change trust settings on the user's behalf.
```

확인은 프로젝트의 `.claude/agents/` 디렉토리 또는 `--add-dir` 디렉토리의 정의에 적용되며 부모 폴더에 대한 신뢰 대화를 수락하는 것은 충족하지 않습니다.

**할 일:**

* [디버그 로그](/docs/ko/debug-your-config)가 지정하는 폴더에서 `claude`를 실행하고 신뢰 대화를 수락합니다. 정의는 Claude Code가 팀원을 다시 복구할 때 다시 적용됩니다. 리드 세션을 다시 시작할 필요가 없습니다.
* 또는 `~/.claude.json`에서 `hasTrustDialogAccepted` 항목을 `true`로 설정합니다. 디버그 로그가 인쇄하는 정확한 `projects["<path>"]` 키를 사용합니다.

<h3 id="message-too-large-for-cross-session-delivery">
  교차 세션 전달에 너무 큼
</h3>

Claude의 [교차 세션 메시지](/docs/ko/cross-session-messaging)가 이 머신의 다른 세션으로 너무 길어서 보낼 수 없었습니다. Claude Code가 거부했고 수신 세션이 아무것도 받지 못했습니다. 거부는 터미널의 배너가 아닌 전송 세션의 도구 결과에 나타납니다. 두 크기와 메시지를 맞추는 방법을 지정합니다:

```text wrap theme={null}
Failed to send to api-worker: Message too large for cross-session delivery: the serialized message is 1,203,844 characters and the limit is 1,048,576. Shorten the message text — put bulk content in a file the recipient can read rather than in the message — or split it into smaller messages.
```

동일한 텍스트를 다시 보내면 동일한 방식으로 실패합니다.

**할 일:**

* Claude에 메시지를 요약하거나 대량 콘텐츠를 파일에 넣고 수신자가 읽을 수 있도록 파일의 경로를 보내도록 요청합니다.
* Claude에 콘텐츠를 여러 개의 더 짧은 메시지로 나누도록 요청합니다.

v2.1.235 이전에는 Claude Code가 과도한 크기의 메시지를 보낸 것으로 보고했습니다. 수신 세션이 읽지 않고 삭제했습니다.

<h3 id="too-many-messages-to-this-session-just-now">
  이 세션으로 너무 많은 메시지
</h3>

Claude가 이 머신의 한 세션으로 빠른 [교차 세션 메시지](/docs/ko/cross-session-messaging) 버스트를 보냈고 버스트가 해당 세션의 받은편지함이 수락하는 것에 도달했습니다. Claude Code가 다음 전송을 거부했고 수신 세션이 아무것도 받지 못했습니다. 거부는 터미널의 배너가 아닌 전송 세션의 도구 결과에 나타납니다:

```text wrap theme={null}
Failed to send to api-worker: Too many messages to this session just now: 30 were sent recently and more would be dropped by its rate limit, so this one was not sent. Batch what remains into one message, or wait a little before sending more.
```

**할 일:**

* 보통 아무것도 하지 않습니다: Claude가 남은 콘텐츠를 한 메시지로 일괄 처리하거나 더 보내기 전에 기다립니다.
* 버스트를 직접 프롬프트했으면 Claude에 남은 것을 단일 메시지로 결합하도록 요청합니다.

v2.1.236 이전에는 Claude Code가 이러한 전송을 보낸 것으로 보고했습니다. 수신 세션이 읽지 않고 삭제했습니다.

<h3 id="refusing-to-send-a-cross-session-message">
  교차 세션 메시지 전송 거부
</h3>

Claude Code가 이 머신의 다른 세션으로 [교차 세션 메시지](/docs/ko/cross-session-messaging)를 쓰기 전에 대상 세션의 받은편지함 소켓이 메시지가 주소 지정된 엔드포인트인지 확인합니다. 확인이 실패하면 Claude Code가 전송 세션에서 전송을 거부하고 대상 세션이 아무것도 받지 못합니다. Claude가 보내는 메시지의 경우 거부는 전송 세션의 도구 결과에 나타납니다:

```text theme={null}
Failed to send to api-worker: Refusing to send: reply target is a symlink
```

`Refusing to send:` 뒤의 텍스트는 실패한 확인의 이름을 지정합니다:

* `reply target is a symlink`: 기호 링크가 대상 세션의 소켓 경로에 있습니다. Claude Code는 링크가 대상 세션이 만들지 않은 엔드포인트로 메시지를 리디렉션할 수 있으므로 이를 통해 전달하지 않습니다.
* `cannot vet reply target`: Claude Code가 대상 경로를 전혀 검사할 수 없습니다. 예를 들어 권한 오류로 읽기가 실패한 경우입니다.
* `connected endpoint is not the expected process`: 소켓을 보유한 프로세스가 메시지가 주소 지정된 세션이 아니므로 주소가 오래되었거나 다른 프로세스가 소켓을 대체했습니다.
* `connected endpoint identity could not be read`: Claude Code가 연결했지만 소켓의 다른 쪽을 보유한 프로세스를 읽을 수 없어서 대상을 확인할 수 없습니다. 이는 일시적일 수 있습니다.
* `connected endpoint is not owned by this user`: 소켓을 보유한 프로세스가 다른 사용자 계정으로 실행되므로 세션 중 하나가 아닙니다.
* `connected endpoint owner could not be read`: Claude Code가 연결했지만 다른 쪽을 소유한 사용자 계정을 읽을 수 없어서 엔드포인트가 당신의 것인지 확인할 수 없습니다.
* `connected endpoint is a different process with the expected pid`: 프로세스 ID가 메시지가 주소 지정된 것과 일치하지만 Claude Code가 동일한 프로세스인지 확인할 수 없습니다. 보통 해당 세션이 종료되었고 운영 체제가 프로세스 ID를 재사용했으므로 주소가 오래되었습니다.

**할 일:**

* 보통 아무것도 하지 않습니다: 확인은 메시지가 주소 지정된 세션 이외의 엔드포인트에 도달하는 것을 방지하며 아무것도 보내지지 않았습니다.
* Claude에 세션을 다시 나열하고 다시 보내도록 요청합니다. 오래된 주소로 인한 거부는 Claude가 현재 세션으로 보낸 후 해결됩니다.
* 한 세션에 대해 `reply target is a symlink`가 반복되면 해당 세션의 소켓 경로에 링크를 만든 것을 확인합니다. 이는 `/status` 아래의 `Peer address`에 표시됩니다.
* `connected endpoint identity could not be read`의 경우 다시 보냅니다. 조건은 일시적일 수 있습니다.
* 공유 머신에서 `connected endpoint is not owned by this user`가 나타나면 해당 주소의 세션이 다른 사용자의 계정으로 실행되므로 Claude가 당신의 계정에서 메시지를 보낼 수 없습니다.

v2.1.248 이전에는 Claude Code가 엔드포인트의 소유 사용자 또는 프로세스 시작 시간을 확인하지 않았으므로 이러한 확인의 이름을 지정하는 거부는 이전 버전에 나타나지 않습니다.

<h3 id="refusing-after-a-symlink-changed">
  경로 읽기, 쓰기 또는 검색 거부
</h3>

Claude Code는 파일 경로의 [권한 규칙](/docs/ko/permissions#read-and-edit)을 확인한 다음 도구가 파일을 열거나 검색을 시작할 때 해석을 다시 확인합니다. 경로가 여전히 확인이 승인한 위치로 이어지는지 확인할 수 없으면 Claude Code는 따르는 대신 작업을 거부합니다. 거부는 도구 결과에 나타납니다:

```text wrap theme={null}
Refusing to read /path/to/file: its symlink resolution changed after permission was checked (a link on the way now leads somewhere the check did not see). If a link in the working directory is being rewritten concurrently, stop that and retry.
```

각 거부는 이유를 지정합니다:

* `its symlink resolution changed after permission was checked`: 경로를 따라 또는 Grep 또는 Glob 검색 루트에서 기호 링크가 권한 확인과 작업 사이에 대체되었습니다. 읽기 거부에서 괄호 안의 구문은 어느 비교가 실패했는지 지정합니다.
* `its parent-directory symlink resolution changed after permission was checked`: 쓰기 경로가 통과하는 디렉토리가 더 이상 승인된 위치로 해석되지 않습니다.
* `it is a symbolic link. Write to the link's target path instead`: 기호 링크가 승인된 쓰기 위치 자체에 있습니다. 예를 들어 `CLAUDE.md`가 `AGENTS.md`로의 기호 링크입니다. 메시지는 Claude를 링크의 대상으로 지시합니다.
* `Refusing to write through symlink: <path>. Resolve the symlink and pass the real target path explicitly.`: 다른 작성자가 파일을 열 때 같은 조건이 포착됩니다. 예를 들어 기호 링크된 `.mcp.json`에 대한 쓰기입니다.
* `Refusing to write into symlinked directory: <path>`: 파일을 보유한 디렉토리 자체가 기호 링크입니다. 예를 들어 프로젝트의 `.claude/` 디렉토리가 다른 위치로 연결되어 있습니다.
* `a path one of its Read deny rules is written through changed while the search was being prepared. Retry.`: `Read` 거부 규칙이 기호 링크를 통과하는 경로의 이름을 지정했고 Claude Code가 검색을 준비하는 동안 해당 링크가 변경되었습니다.
* `it could not be opened (EACCES) — it is unreadable, or is being replaced concurrently.`: 검색 루트가 존재하지만 열 수 없습니다. 괄호 안의 코드는 운영 체제 오류입니다.
* `its permission check expired before it ran (too many concurrent file operations). Retry.`: Claude Code가 많은 동시 파일 작업 중에 도구가 사용하기 전에 승인 기록을 제거했습니다. 다시 시도하면 새로운 권한 확인을 실행합니다.
* `ripgrep was found only by name on PATH, and a search outside the working directory cannot apply your Read deny rules in that configuration`: Claude Code가 `rg` 바이너리를 절대 경로로 해석할 수 없어서 작업 디렉토리 외부의 검색을 거부하는 것이 거부 규칙을 적용하지 않는 것보다 낫습니다.

**할 일:**

* 보통 아무것도 하지 않습니다: 거부는 Claude에 도구 결과로 도달하고 거부된 작업은 실행되지 않습니다.
* 한 경로에서 기호 링크 거부가 반복되면 빌드 도구 또는 파일 감시자와 같이 링크를 계속 다시 쓰는 것을 찾거나 Claude에 링크된 경로 대신 파일의 해석된 경로를 사용하도록 요청합니다.
* 이 거부가 Claude Code가 AppContainer 또는 제한된 토큰 샌드박스 내의 Windows에서 실행될 때 모든 파일에 대해 나타나면 v2.1.265 이상으로 업그레이드합니다.
* 읽기 거부가 macOS에서 스크린샷을 프롬프트로 드래그한 것처럼 아무것도 다시 쓰지 않는 파일에 대해 나타나면 v2.1.273 이상으로 업그레이드합니다.
* ripgrep 거부의 경우 패키지 관리자로 ripgrep을 설치하여 `rg`가 `PATH`의 절대 경로로 해석되거나 검색을 작업 디렉토리 아래로 유지합니다.

v2.1.251 이전에는 Claude Code가 파일 쓰기에 대해서만 경로의 해석을 다시 확인했으므로 권한 확인 후 대체된 링크가 메시지 없이 읽기 또는 검색을 다른 위치로 리디렉션할 수 있었습니다. 이러한 거부 중에서 부모 디렉토리, 기호 링크를 통한, 그리고 기호 링크된 디렉토리 쓰기 거부만 이전 버전에 나타납니다.

<h3 id="task-output-swap-refused">
  작업 출력 스왑 거부
</h3>

Claude Code는 각 Bash 명령의 출력을 임시 디렉토리 아래의 파일에 저장합니다. 이러한 파일 중 하나를 열 때마다 경로가 여전히 생성한 파일로 이어지는지 확인합니다. 기호 링크, 추가 하드 링크 또는 이동된 디렉토리가 리디렉션하지 않습니다. 이 메시지는 확인이 실패했음을 의미하므로 Claude Code는 해당 경로를 통해 쓰거나 읽는 대신 작업을 거부했습니다. 메시지는 Bash 도구 결과에 나타납니다:

```text wrap theme={null}
task output swap refused (tasks dir moved or linked): /private/tmp/claude-501/-Users-you-my-project/1f0e62dc-4b0a-4f5e-9c2d-8a7b6c5d4e3f/tasks/b7k2f9m3q.output. To recover: restart Claude Code with CLAUDE_CODE_TMPDIR set to a fresh directory; or, if /private/tmp/claude-501/-Users-you-my-project is a stray directory or a symbolic link that should not be there, remove that entry itself (not what it points to) and restart.
```

괄호 안의 텍스트는 실패한 확인의 이름을 지정합니다. `output symlink was re-pointed`, `output file identity changed`, `not a regular file`과 같은 이유는 모두 동일한 조건을 보고합니다: 출력 경로의 또는 따라 무언가가 더 이상 Claude Code가 생성한 파일이 아닙니다. 일부 이유만 `To recover:` 문장을 전달합니다.

명령이 여전히 실행 중일 때 확인이 실패하면 Claude Code가 명령을 중지하고 결과는 다음을 보고합니다:

```text theme={null}
Command killed: its output file was replaced or could no longer be verified
```

**할 일:**

* v2.1.260 이상으로 업그레이드합니다. 이전 버전은 링크 또는 이동된 디렉토리가 없을 때도 이 메시지를 표시하기도 했습니다.
* [`CLAUDE_CODE_TMPDIR`](/docs/ko/env-vars)을 새 디렉토리로 설정하여 Claude Code를 다시 시작합니다.
* 또는 Claude Code 임시 디렉토리 아래의 프로젝트 디렉토리를 확인합니다. 예제 메시지의 `/private/tmp/claude-501/-Users-you-my-project`입니다. 해당 경로가 기호 링크이거나 거기에 있으면 안 되는 디렉토리이면 링크의 대상이 아닌 링크 또는 디렉토리 자체를 제거하고 Claude Code를 다시 시작합니다.
* 거부가 반복되면 프로세스가 세션이 실행되는 동안 Claude Code의 임시 디렉토리 아래의 항목을 대체, 링크 또는 제거합니다. [`CLAUDE_CODE_TMPDIR`](/docs/ko/env-vars)을 다른 것이 관리하지 않는 디렉토리로 설정하고 다시 시작합니다.

<h3 id="the-source-file-is-not-valid-utf-8-text">
  소스 파일이 유효한 UTF-8 텍스트가 아님
</h3>

Claude가 바이트가 텍스트로 디코딩되지 않거나 텍스트가 이미 대체 문자 `U+FFFD`를 포함하는 파일에서 [아티팩트](/docs/ko/artifacts)를 게시하려고 했으므로 Claude Code는 아무것도 업로드하기 전에 게시를 거부했습니다. 메시지는 Artifact 도구 결과에 나타나고 수정할 첫 번째 위치의 이름을 지정합니다:

```text wrap theme={null}
file_path: the source file is not valid UTF-8 text (first invalid byte at line 12, column 40). It may be saved in another encoding or contain binary data. Rewrite it as UTF-8, then publish again. Nothing was published.

file_path: the source file has the replacement character U+FFFD at line 12, column 40, usually left where an earlier edit or paste lost a character. Replace it with the intended text (in HTML, write an intended U+FFFD as &#xFFFD;), then publish again. Nothing was published.
```

Claude Code는 파일을 UTF-8로 디코딩하거나 리틀 엔디안 UTF-16 바이트 순서 표시로 시작할 때 UTF-16으로 디코딩합니다. 그러한 UTF-16 파일이 디코딩되지 않으면 첫 번째 메시지는 `UTF-16`의 이름을 지정하고 여전히 파일을 UTF-8로 다시 쓰도록 지시합니다. 명명된 위치 뒤에 더 많은 위치가 따르면 메시지는 위치 뒤에 `(+2 more)`과 같은 개수를 추가합니다.

**할 일:**

* 보통 아무것도 하지 않습니다: Claude가 파일을 다시 쓰고 게시합니다.
* 파일이 당신이 작성하거나 내보낸 것이면 UTF-8로 다시 저장하고 각 `U+FFFD`를 이전 편집, 붙여넣기 또는 변환이 손실한 문자로 바꿉니다.
* 페이지에 의도적인 `U+FFFD`를 표시하려면 리터럴 문자 대신 HTML에서 `&#xFFFD;`로 작성합니다.

v2.1.267 이전에는 Claude Code가 그러한 파일을 확인 없이 업로드했고 서버가 대신 게시를 거부했습니다.

<h3 id="reading-a-local-file-from-outside-the-connected-folders">
  Cowork 세션에서 연결된 폴더 외부의 로컬 파일 읽기
</h3>

Claude Desktop 앱에서 머신에서 실행 중인 [Cowork](https://claude.com/docs/cowork/overview) 세션에서 Claude가 [아티팩트](/docs/ko/artifacts)에 대한 로컬 파일의 이름을 지정했습니다. Claude Code가 파일이 세션의 연결된 폴더 내의 일반 파일인지 확인할 수 없습니다: 경로가 해당 폴더 외부에 있거나, 기호 링크를 통과하거나, 나타나는 것과 다른 파일의 이름을 지정할 수 있는 방식으로 철자가 지정되었습니다. 그러한 파일을 읽으려면 승인이 필요하며, 승인 카드를 표시할 수 없는 세션(예: 모든 승인을 건너뛰도록 설정된 세션)에서 Claude Code는 읽기를 거부합니다.

거부는 Artifact 도구 결과에 나타납니다. 파일을 전혀 검사할 수 없을 때 대신 해당 실패의 이름을 지정합니다:

```text wrap theme={null}
Reading a local file from outside this session's connected folders, or through a link, needs the approval card, and no one can answer it in this Cowork session. Use a plain file inside the connected folders; do not retry this file in this session.

cannot read file_path (ENOENT) — the file could not be examined, and no one can answer the approval card in this Cowork session. Check that the file exists as a plain file inside the connected folders, then retry with that path.
```

**할 일:**

* 보통 아무것도 하지 않습니다: 메시지는 Claude에 연결된 폴더 내의 일반 파일을 대신 사용하도록 지시합니다.
* 정확한 파일을 아티팩트에 넣으려면 세션의 연결된 폴더 중 하나에 일반 파일(기호 링크 아님)로 복사하고 다시 요청합니다.

<h3 id="webfetch-cannot-fetch-localhost">
  WebFetch는 localhost를 가져올 수 없음
</h3>

Claude가 `http://localhost:3000` 또는 `http://wiki/`와 같은 인트라넷 이름처럼 점이 없는 호스트명을 가진 URL로 [WebFetch](/docs/ko/tools-reference#webfetch-tool-behavior)를 호출했습니다. WebFetch는 요청을 하기 전에 이러한 URL을 거부합니다:

```text wrap theme={null}
WebFetch cannot fetch localhost or other hostnames without a dot. To reach a local server, use Bash with curl instead.
```

**할 일:**

* 보통 아무것도 하지 않습니다: 메시지는 Claude를 Bash 도구를 통한 `curl`로 지시하며, 이는 로컬 및 인트라넷 서버에 도달할 수 있습니다.

v2.1.268 이전에는 WebFetch가 이러한 URL을 일반 `Invalid URL` 오류로 보고했습니다.

<h2 id="background-session-errors">
  백그라운드 세션 오류
</h2>

[백그라운드 세션](/docs/ko/agent-view)은 자체 대화형 터미널 없이 실행되므로 터미널이 필요한 명령은 다르게 작동합니다. 이러한 메시지는 백그라운드 세션의 기록, 백그라운드 세션에 연결된 터미널, 디스패치하는 세션 또는 셸에 나타나거나, 아래의 [worktree-guard 항목](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved)의 경우 worktree에 격리된 모든 세션이나 worktree 격리 서브에이전트를 실행하는 세션에 나타납니다. 메시지가 특정 표면에만 해당하는 경우 해당 항목에 명시됩니다.

<h3 id="commands-refused-in-a-background-session">
  백그라운드 세션에서 거부된 명령
</h3>

대화형 대화 상자를 여는 명령은 백그라운드 세션에 터미널이 연결되지 않은 상태에서는 그렇게 할 수 없습니다. `/install-github-app`, `/mcp` 설정 목록 및 MCP 서버 메뉴의 인증 작업은 메시지로 응답하며, 세션은 [에이전트 뷰](/docs/ko/agent-view)의 **입력 필요** 아래에 나타나므로 찾아서 연결하고 명령을 다시 실행할 수 있습니다. 터미널이 연결되어 있는 동안 이러한 명령은 정상적으로 작동합니다.

v2.1.216 이전에는 이러한 거부 중 하나 후에 세션이 **입력 필요** 아래에 나타나지 않았습니다. v2.1.213부터 v2.1.215까지는 터미널이 연결되어 있는 동안 명령이 여전히 작동했으며, 거부 메시지는 연결하고 명령을 다시 실행하도록 지시했습니다. v2.1.208부터 v2.1.212까지는 Claude Code가 터미널이 연결되어 있는 동안에도 이를 거부했으며, `Can't open MCP settings in a background session`과 같은 메시지가 표시되었습니다. 이러한 버전에서는 일반 `claude` 세션에서 명령을 실행하거나 업그레이드하십시오. v2.1.208 이전에는 백그라운드 세션 내에서 대화 상자를 열었습니다. v2.1.208에서만 Claude Code는 백그라운드 세션에서 `/model` 선택기도 거부했으며, `/upgrade`는 브라우저를 열지 않고 업그레이드 URL을 인쇄했습니다.

표현은 명령의 이름을 지정합니다. `/mcp` 설정 목록은 다음을 보고합니다:

```text theme={null}
Can't open MCP settings while no terminal is attached to this background session. This session now shows "needs input" in agent view — open it and run /mcp to manage servers, or use `/mcp enable|disable|reconnect <server>` to steer without the panel.
```

**할 일:**

* 에이전트 뷰에서 **입력 필요** 아래에 나열된 세션에 연결하고 명령을 다시 실행합니다.
* 또는 `/mcp reconnect <server>`, `/mcp enable` 또는 `/mcp disable`과 같이 메시지가 명시하는 형식을 사용합니다. 이는 연결하지 않고도 작동합니다.

<h3 id="write-or-command-blocked-because-the-path-cannot-be-safely-resolved">
  경로를 안전하게 확인할 수 없어서 쓰기 또는 명령이 차단됨
</h3>

Claude가 [worktree 격리 가드](/docs/ko/agent-view#how-file-edits-are-isolated)가 하나의 확인 가능한 위치로 확인할 수 없는 철자로 파일 또는 작업 디렉토리를 처리했습니다. 가드는 [worktree에 격리된 모든 세션](/docs/ko/worktrees#how-claude-code-enforces-isolation)(대화형 또는 백그라운드)과 [worktree 격리 서브에이전트](/docs/ko/worktrees#isolate-subagents-with-worktrees)에서 쓰기 및 명령 작업 디렉토리를 확인합니다. 공유 체크아웃에 도달하지 않는지 확인하기 전에 심볼릭 링크를 확인하며, 확인이 실패하면 공유 체크아웃에 도달하도록 하는 대신 작업을 차단합니다. 메시지는 거부하는 경로 형식과 재시도 방법을 명시합니다:

```text theme={null}
This write was blocked because the path is spelled in a form that cannot be safely resolved (for example through a symlink storing a raw dot segment, a network-share or device-namespace shape, or an unreadable ancestor directory). If the file is inside the worktree /path/to/worktree, address it by its direct symlink-free path instead.
```

차단된 명령은 작업 디렉토리에 대해 동일한 원인을 보고하고 `re-run the command from its direct symlink-free path`로 끝납니다. v2.1.217 이전에는 가드가 심볼릭 링크를 확인하지 않고 경로 철자를 비교했으므로 이러한 철자는 차단되지 않았으며 심볼릭 링크를 통한 쓰기는 공유 체크아웃에 도달할 수 있었습니다.

**할 일:**

* 일반적으로 아무것도 하지 않습니다: 전체 메시지는 Claude에 도구 오류로 전달되며 Claude는 명시하는 직접 경로로 재시도합니다. 차단된 파일 편집의 경우 대화 뷰에는 짧은 `Error editing file` 줄만 표시됩니다. 전체 메시지는 `Ctrl+O`로 열 수 있는 기록 뷰에 나타납니다. 차단된 명령은 명령 출력에 인쇄합니다.
* 같은 파일에서 차단이 반복되면 경로가 `docs/current -> ../README.md`와 같이 `..`를 포함하는 대상을 가진 커밋된 심볼릭 링크를 통해 실행될 가능성이 높습니다. Claude에 링크를 통하지 않고 실제 경로로 대상 파일을 편집하도록 요청합니다.

<h3 id="write-or-command-blocked-because-the-path-names-a-network-location">
  경로가 네트워크 위치를 명시하기 때문에 쓰기 또는 명령이 차단됨
</h3>

Claude가 머신에 없는 드라이브, `\\server\share\file`과 같은 UNC 공유 또는 `/net` 자동 마운트 경로를 명시하는 경로를 통해 파일 또는 작업 디렉토리를 처리했으며, 세션의 체크아웃은 로컬 디스크에 있습니다. 동일한 [worktree 격리 가드](#write-or-command-blocked-because-the-path-cannot-be-safely-resolved)는 그러한 경로가 공유 체크아웃을 벗어나는지 확인할 수 없으므로 작업을 차단합니다. 세션을 worktree에 격리해도 차단이 해제되지 않습니다. 메시지는 대신 사용할 경로 형식을 명시합니다:

```text theme={null}
This write was blocked because the path is network-shaped (a UNC share or /net automount spelling) while this session's checkout is local. Isolating cannot unblock it. If the file is genuinely inside the worktree /path/to/worktree, address it by its local, plainly-spelled path instead.
```

차단된 명령은 작업 디렉토리에 대해 동일한 원인을 보고하고 `re-run the command from its local, plainly-spelled path`로 끝납니다. v2.1.217 이전에는 가드가 경로 텍스트만 비교했으므로 UNC 또는 `/net` 경로를 통해 체크아웃 내부의 파일을 처리하는 것은 차단되지 않았습니다.

**할 일:**

* 일반적으로 아무것도 하지 않습니다: Claude는 메시지가 요청하는 로컬 철자로 재시도합니다.
* 파일이 로컬 파일이 아닌 네트워크 공유에 있는 경우 네트워크 경로로 철자된 경우 세션의 로컬 작업 공간 외부에 있습니다. 일반 대화형 세션에서 편집합니다.

<h3 id="command-blocked-by-the-worktree-isolation-checks">
  Worktree 격리 확인으로 인해 명령이 차단됨
</h3>

Claude가 [worktree에 격리된 세션](/docs/ko/worktrees#how-claude-code-enforces-isolation)에서 Bash 또는 Monitor 명령을 실행했으며, Claude Code가 다음 두 가지 이유 중 하나로 거부했습니다:

* 명령이 git을 주 체크아웃으로 지정합니다.
* Claude Code는 명령 텍스트에서 명령이 실행하는 모든 git이 worktree 내에 머물러 있는지 확인할 수 없습니다. git을 명시하지 않는 명령도 이 이유로 거부될 수 있습니다. `${!name}`과 같은 변수 간접 참조를 확장하거나 `${ command; }`와 같은 Bash 함수 치환을 실행하면 자체가 명령일 수 있는 런타임 값이 생성되기 때문입니다.

메시지의 중간은 확인할 수 없는 것을 명시합니다:

```text wrap theme={null}
This session is isolated in the worktree /path/to/worktree, but this command evaluates ${!x@P} arithmetically inside a construct too complex to verify, which can run a command hidden in a variable's value. Refusing to run it — a worktree-isolated session's git operations must target its own worktree. Split it into plain, separate commands and run them from /path/to/worktree.
```

**할 일:**

* 일반적으로 아무것도 하지 않습니다: Claude는 메시지를 읽고 최종 문장이 요청하는 방식으로 명령을 다시 작성합니다.
* 요청한 명령이 계속 거부되면 플래그된 값을 문자 그대로 철자합니다: 간접 참조 또는 치환을 해당 값으로 바꾸고 worktree 내에서 git을 자체 일반 명령으로 실행합니다.
* 의도적으로 주 체크아웃에 작용하려면 세션 외부의 터미널에서 명령을 직접 실행합니다.

<h3 id="this-session-has-no-saved-transcript">
  이 세션에는 저장된 기록이 없습니다
</h3>

`←` 또는 `/background`로 다른 대화에서 백그라운드로 처리되고 첫 번째 응답이 완료되기 전에 중지된 [백그라운드 세션](/docs/ko/agent-view)에 연결했습니다. 첫 번째 응답이 완료될 때까지 대화는 백그라운드로 처리된 세션에만 존재하므로 `claude attach`는 같은 세션 ID로 빈 대화를 시작하는 대신 중지된 세션을 시작하기를 거부합니다. 메시지는 이 세션에 대한 `claude respawn` 명령으로 끝납니다:

```text theme={null}
This session has no saved transcript — it was stopped before its first response finished. If it was backgrounded from another conversation, that one is still intact; `claude respawn <id>` starts this one fresh.
```

[에이전트 뷰](/docs/ko/agent-view)에서 같은 세션의 행을 열면 목록 아래에 `Press enter again to restart this session fresh`가 표시되며, 행에서 두 번째 `Enter`는 빈 대화로 세션을 다시 시작합니다. v2.1.212 이전에는 행을 열면 재시작할 방법이 없는 거부 메시지가 표시되었습니다. v2.1.211 이전에는 중지된 세션을 열면 자동으로 빈 대화를 시작했으며 세션의 원래 프롬프트를 다시 실행할 수 있었습니다.

**할 일:**

* 백그라운드로 처리한 대화는 그대로 유지됩니다: [`claude --resume`](/docs/ko/sessions)으로 재개하거나 계속 작업합니다.
* 중지된 세션을 새로 시작하려면 메시지의 ID로 `claude respawn <id>`를 실행하거나 에이전트 뷰의 행에서 `Enter`를 두 번 누릅니다.
* 세션이 응답을 완료했는데도 v2.1.214 이전 버전에서 이 거부가 표시되면 `~/.claude/projects`의 읽을 수 없는 폴더로 인해 기록 스캔이 저장된 대화를 놓칠 수 있습니다. v2.1.214 이상으로 업데이트하면 스캔 중에 읽을 수 없는 폴더를 허용합니다.

<h3 id="this-session-is-running-in-another-terminal">
  이 세션이 다른 터미널에서 실행 중입니다
</h3>

[에이전트 뷰](/docs/ko/agent-view)에서 중지된 세션의 행을 열었으며, 저장된 대화가 이미 이 머신의 다른 라이브 Claude Code 프로세스에서 열려 있으므로 Claude Code는 같은 기록에 쓸 두 번째 프로세스를 시작하기를 거부합니다. 표시되는 메시지는 [대화를 보유한 것](/docs/ko/agent-view#opening-a-session-says-the-conversation-is-already-open)에 따라 다릅니다:

```text theme={null}
Can't open — this session is running in another terminal
This conversation is already open in another running Claude session — use that one, or close it and try again
```

* **`running in another terminal`**: 터미널이 대화를 보유합니다. 예를 들어 `claude --resume` 또는 `/resume`으로 재개한 터미널입니다. 행에는 `Open in a terminal`도 표시됩니다.
* **`already open in another running Claude session`**: 다른 비대화형 Claude Code 프로세스가 이를 보유합니다. 예를 들어 같은 대화에 대한 [백그라운드 세션](/docs/ko/agent-view#the-supervisor-process) 프로세스가 아직 종료되지 않았습니다.

Claude Code는 행을 열 때 입력한 응답을 저장하고 세션이 다음에 시작할 때 세션의 다음 프롬프트로 전송합니다.

**할 일:**

* 열려 있는 프로세스에서 대화를 계속하거나 해당 프로세스를 종료하고 행을 다시 엽니다.

v2.1.248 이전에는 `already open in another running Claude session` 거부만 존재했습니다: 터미널에서 재개된 대화는 열려 있는 것으로 계산되지 않았으며, 행을 열면 같은 대화에 쓰는 두 번째 Claude Code 프로세스가 시작되었습니다.

<h3 id="this-sessions-saved-conversation-is-no-longer-on-disk">
  이 세션의 저장된 대화가 더 이상 디스크에 없습니다
</h3>

백그라운드 서비스가 꺼져 있는 동안 종료된 [백그라운드 세션](/docs/ko/agent-view)을 열었으며, [기록 정리](/docs/ko/settings-reference#cleanupperioddays)가 저장된 대화를 제거했습니다. 예를 들어 머신이 몇 주 동안 꺼져 있었습니다. 일반적으로 이러한 행을 열면 [저장된 대화를 재개](/docs/ko/agent-view#sessions-show-as-failed-after-shutdown)합니다. 재개할 것이 없으면 Claude Code는 세션의 원래 프롬프트를 다시 실행하도록 요청하지 않고 거부합니다:

```text theme={null}
This session's saved conversation is no longer on disk (it ended while the background service was off, and old transcripts are cleaned up), so there is nothing to resume. `claude rm 7c5dcf5d` deletes the row; `claude respawn 7c5dcf5d` runs its original prompt again instead.
```

`claude attach <id>`는 이 텍스트를 인쇄합니다. 에이전트 뷰에서 바닥글은 더 짧으며 `ctrl+x deletes the row`로 끝납니다.

**할 일:**

* `claude rm <id>`를 실행하여 행을 삭제합니다. [유지된 경우](/docs/ko/agent-view#what-deleting-a-session-removes) 중 하나가 적용되면 `claude rm`은 행과 worktree를 유지하고 이유를 명시합니다.
* 세션의 원래 프롬프트를 새 대화로 다시 실행하려면 `claude respawn <id>`를 실행합니다.

v2.1.248 이전에는 이러한 행을 열면 거부하는 대신 세션의 원래 프롬프트를 다시 실행했으며, 몇 주 된 작업을 전경으로 끌어올렸습니다.

<h3 id="worktree-has-commits-that-are-not-pushed-anywhere">
  Worktree에 어디에도 푸시되지 않은 커밋이 있습니다
</h3>

Claude Code가 다른 곳에 저장되었는지 확인할 수 없는 커밋을 보유한 [백그라운드 세션](/docs/ko/agent-view#what-deleting-a-session-removes)을 삭제하려고 했습니다. Claude Code는 커밋을 보지 않고 파괴하는 대신 worktree와 세션 행을 유지합니다. `claude rm`은 분기와 푸시되지 않은 커밋을 명시하고 진행 방법을 설명합니다:

```text theme={null}
kept 7c5dcf5d — its worktree is still at "/home/you/project/.claude/worktrees/fix-login"
  2 unpushed commits on "claude/fix-login": a1b2c3d "Fix login flow" and 1 more. They exist on no remote, so deleting the worktree would lose them.
  push them and run 'claude rm 7c5dcf5d' again, or discard the worktree and its commits: claude rm 7c5dcf5d --discard-unpushed a1b2c3d000000000000000000000000000000000@0123456789abcdef0123456789abcdef
```

Claude Code가 커밋을 요약할 수 없으면 메시지는 `The worktree has unpushed commits`를 대신 읽습니다. [에이전트 뷰](/docs/ko/agent-view)에서 세션의 행은 `not deleted`를 표시하고 같은 이유를 표시합니다.

원격의 커밋은 삭제를 차단하지 않습니다. 로컬 `origin` 원격의 기본 분기의 로컬 복사본의 커밋도 차단하지 않습니다. 해당 분기가 주 체크아웃(저장소 디렉토리 자체, worktree가 아님)에서 체크아웃되어 있는 한.

**할 일:**

* 커밋을 유지하려면 worktree의 분기를 푸시하거나 주 체크아웃에서 체크아웃된 기본 분기로 병합한 다음 세션을 다시 삭제합니다.
* 커밋을 버리려면 메시지가 인쇄한 `claude rm <id> --discard-unpushed` 명령을 실행하거나 에이전트 뷰의 세션 행에서 `Ctrl+X`를 두 번 누릅니다. 이는 세션과 worktree를 분기, 푸시되지 않은 커밋 및 커밋되지 않은 변경 사항과 함께 제거합니다. worktree가 거부 이후 커밋을 얻으면 Claude Code는 다시 유지하고 업데이트된 상태를 표시합니다.
* 메시지가 worktree가 다른 완료된 세션에 의해서도 기록되었다고 말하면 다시 삭제해도 버리지 않습니다: 커밋을 푸시한 다음 세션을 다시 삭제합니다.

v2.1.268 이전에는 `claude rm`이 커밋 요약을 `kept` 줄 자체에 배치했습니다. `claude rm`이 커밋을 요약할 수 없으면 `kept` 줄은 요약 대신 `worktree has commits that are not pushed anywhere`를 읽었습니다.

v2.1.260 이전에는 메시지가 분기 또는 커밋을 명시하지 않았으며, 다시 삭제하는 것은 같은 방식으로 거부되었습니다: 푸시하지 않고 세션을 삭제하는 것은 `git worktree remove --force <path>`로 worktree를 직접 제거한 다음 `claude rm <id>`를 다시 실행하는 것을 의미했습니다.

v2.1.248 이전에는 주 체크아웃에서 체크아웃된 기본 분기가 계산되지 않았습니다: 이미 병합한 분기는 커밋이 원격에 도달할 때까지 이 거부를 트리거했습니다.

<h3 id="terminal-host-process-died">
  터미널 호스트 프로세스가 종료됨
</h3>

각 [백그라운드 세션](/docs/ko/agent-view)의 터미널은 백그라운드 서비스 아래의 호스트 프로세스에서 실행되며, 서비스가 여전히 연결을 유지하는 동안 해당 프로세스가 종료되어 세션에 도달할 수 없었습니다.

Linux 및 WSL에서 백그라운드 서비스는 몇 초마다 각 호스트 프로세스를 확인하고, 프로세스가 종료되었지만 서비스에 대한 연결이 닫히지 않으면 세션을 실패로 표시하고 [에이전트 뷰](/docs/ko/agent-view#read-session-state)의 행에 이유를 표시합니다:

```text theme={null}
terminal host process died — press Enter to restart
```

확인이 실행되기 전에 행을 열면 바닥글에 `This session's terminal host process died (the conversation is saved) — press Enter to restart it`가 표시되고 행이 실패로 변합니다.

셸에서 `claude attach <id>`는 이미 죽은 호스트로 표시된 세션을 다시 시작하고, 그렇지 않으면 원인을 인쇄하고 종료합니다:

```text theme={null}
Couldn't attach to <id> — This session's terminal host process died (the conversation is saved) — run `claude attach <id>` again to restart it on a fresh host.
```

어느 쪽이든 대화가 저장됩니다.

[셸 명령](/docs/ko/agent-view#run-a-shell-command)을 실행하는 행은 대신 `terminal host process died — its output is gone; the command was not run again`을 표시하고, `claude attach`는 `This command's terminal host process died — its output is gone and the command was not run again`을 인쇄합니다. Claude Code는 절대 명령을 다시 실행하지 않습니다.

**할 일:**

* 에이전트 뷰에서 실패한 행에 `Enter`를 누릅니다. 세션이 새 호스트 프로세스에서 다시 시작되고 대화가 재개됩니다.
* 셸에서 `claude attach <id>`를 다시 실행합니다. Claude Code는 `Session <id>'s terminal host died — restarting it on a fresh one…`을 인쇄하고 세션을 다시 엽니다.
* 이 방식으로 셸 명령 행을 다시 시작할 수 없습니다. 명령을 다시 디스패치하여 다시 실행합니다.

v2.1.247 이전에는 죽은 호스트 프로세스가 백그라운드 서비스가 실행한 모든 생존 확인을 통과할 수 있었으므로 세션을 열면 `opening… · esc to cancel`이 무한정 표시되고 `claude attach <id>`는 오류를 보고하지 않고 대기했습니다.

<h3 id="session-isnt-responding">
  세션이 응답하지 않습니다
</h3>

[백그라운드 세션](/docs/ko/agent-view)을 열었으며 백그라운드 서비스가 열기를 수락했지만 약 10초 동안 출력이 도착하지 않아 Claude Code는 세션의 터미널을 중계하는 프로세스가 출력을 전달할 수 없다고 결론짓고 대기하는 대신 시도를 종료합니다.

에이전트 뷰에서 Claude Code는 바닥글에 재시작을 제공합니다:

```text theme={null}
Press enter again to restart this session — it isn't responding (its conversation is saved and resumes).
```

셸에서 `claude attach <id>`는 원인을 인쇄하고 종료합니다:

```text theme={null}
Couldn't attach to <id> — Session isn't responding — `claude stop <id>`, then `claude attach <id>` restarts it (the conversation is saved).
```

Claude Code는 [셸 명령](/docs/ko/agent-view#run-a-shell-command)을 실행하는 행을 절대 다시 시작하지 않습니다. 재시작하면 명령이 다시 실행되기 때문입니다.

**할 일:**

* 에이전트 뷰에서 같은 행에 `Enter`를 다시 누릅니다. Claude Code는 응답하지 않는 프로세스를 중지하고 세션을 다시 시작하며 대화가 재개됩니다. 두 번째 누름 없이는 아무것도 중지되지 않습니다.
* 셸에서 `claude stop <id>`를 실행한 다음 `claude attach <id>`를 실행합니다.
* 셸 명령 행의 경우 에이전트 뷰에서 `Ctrl+X`를 누르거나 `claude stop <id>`를 실행하여 중지합니다. 명령을 다시 디스패치하여 다시 실행합니다.

<h3 id="session-was-stopped-while-the-respawn-was-in-flight">
  재시작이 진행 중인 동안 세션이 중지됨
</h3>

[백그라운드 세션](/docs/ko/agent-view)을 열었으며 해당 프로세스가 실행 중이 아니었고, Claude Code가 이를 다시 시작하는 동안 다른 Claude Code 프로세스가 이를 중지했습니다. 예를 들어 다른 터미널에서 `claude stop`을 실행했습니다. Claude Code는 세션을 중지된 상태로 유지합니다:

```text theme={null}
Session <id> was stopped while the respawn was in flight
```

방금 디스패치한 세션을 열면서 프로세스가 여전히 시작 중인 동안 프로세스가 시작될 때까지 대기합니다. v2.1.246 이전에는 그 순간에 열면 중지되고 이 메시지가 표시될 수 있었습니다.

**할 일:**

* 세션을 중지하지 않았으면 에이전트 뷰에서 행을 다시 열거나 `claude respawn <id>`를 실행하여 다시 시작합니다.
* 직접 중지했으면 남은 것이 없습니다: 세션은 중지된 상태로 유지됩니다.

<h3 id="session-agent-no-longer-available">
  세션 에이전트를 더 이상 사용할 수 없습니다
</h3>

[사용자 정의 에이전트](/docs/ko/sub-agents#invoke-subagents-explicitly)를 실행 중이던 세션을 재개했으며, `--agent` 또는 `agent` 설정으로 시작했으며, Claude Code가 해당 이름의 에이전트를 찾지 못했습니다. 세션의 원래 디렉토리를 먼저 검색하며, [해당 작업 공간을 신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust)한 경우, 그 다음 재개하는 디렉토리를 검색합니다. 세션은 여전히 재개되지만 기본 도구를 사용하므로 에이전트의 도구 제한이 더 이상 적용되지 않습니다:

```text theme={null}
This session was running agent 'code-reviewer', which is no longer available (no agent by that name in /home/you/project). Continuing with the default tools and system prompt — the agent's tool restrictions no longer apply. To restore it, re-create the agent, or resume with an explicit --agent <name>.
```

경고는 Claude Code가 검색한 디렉토리만 명시하며, [백그라운드 세션](/docs/ko/agent-view)을 깨우거나, `/resume` 또는 `claude --resume`을 실행하거나, [비대화형 모드](/docs/ko/headless)에서 재개할 때 재개된 대화에 나타나며, 여기서 stderr로도 이동합니다. `--input-format stream-json`을 사용하는 세션은 Agent SDK가 시작 후 에이전트를 제공하므로 표시하지 않습니다.

Claude Code는 폴백을 세션에 저장하지 않으므로 경고는 조치할 때까지 각 재개에서 반복됩니다. 기본 제공 `claude` 에이전트는 기본 도구 세트로 폴백해도 아무것도 변경되지 않으므로 경고를 트리거하지 않습니다. v2.1.216 이전에는 Claude Code가 기본 에이전트로 자동으로 계속했으며, 조회는 재개하는 디렉토리만 포함했으므로 프로젝트 범위 에이전트는 다른 디렉토리에서 재개할 때 손실되었습니다.

**할 일:**

* 세션의 프로젝트에서 `.claude/agents/<name>.md`에 또는 개인 에이전트의 경우 `~/.claude/agents/<name>.md`에 에이전트 파일을 다시 만든 다음 다시 재개합니다.
* 또는 존재하는 에이전트를 명시하는 `--agent <name>`으로 재개하여 대신 해당 에이전트로 세션을 실행합니다.
* 에이전트가 프로젝트 범위이고 세션의 원래 디렉토리를 신뢰하지 않았으면 거기서 Claude Code를 한 번 실행하고 신뢰 대화를 수락한 다음 다시 재개합니다.

<h3 id="claude_code_process_wrapper-launcher-errors">
  CLAUDE\_CODE\_PROCESS\_WRAPPER 런처 오류
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/ko/corporate-launcher)가 설정되었으며 해당 값을 사용할 수 없으므로 Claude Code는 런처 없이 실행하는 대신 영향을 받는 프로세스를 시작하기를 거부합니다. 구성 문제는 변수 이름으로 시작하고 이유를 명시하는 메시지로 보고됩니다. 예를 들어:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

시작하지만 Claude Code로 자신을 대체하지 않고 종료하는 런처는 시작 중이던 세션을 실패하게 하며, 에이전트 뷰의 세션 행은 런처가 `must exec, not daemonize`라고 보고하고 런처가 인쇄한 모든 것을 따릅니다. 런처 때문에 시작할 수 없거나 백그라운드 서비스에 도달할 수 없는 세션은 `Couldn't reach the background service (...)`의 이유로 런처 문제를 보고합니다.

**할 일:**

* 변수를 `exec "$@"`를 호출하여 끝나는 실행 파일의 절대 경로로 설정합니다. 전체 계약은 [런처 계약](/docs/ko/corporate-launcher#the-launcher-contract)을 참조하십시오.
* `/status`를 확인하면 자체 실행 항목에서 확인된 시작 명령을 표시하고 실행 중인 백그라운드 서비스가 일치하지 않을 때 경고하거나, 셸에서 `claude daemon status`를 실행합니다.
* [설정](/docs/ko/corporate-launcher#set-up-the-launcher)의 `env` 블록에서 값을 수정한 후 `claude daemon stop --any`로 백그라운드 서비스를 다시 시작하여 다음 디스패치가 래핑된 것을 시작하도록 합니다.

<h3 id="eunknown-when-starting-a-background-session">
  백그라운드 세션을 시작할 때 EUNKNOWN
</h3>

Windows가 표준 이름이 없는 오류 코드로 프로그램을 시작하기를 거부했으므로 실패는 `EUNKNOWN`으로 표시됩니다. 일반적인 트리거는 시작 중인 프로그램을 차단하는 그룹 정책 또는 AppLocker와 같은 소프트웨어 제한 정책입니다. 오류는 `/background` 또는 `claude --bg`로 [백그라운드 세션](/docs/ko/agent-view)을 시작할 때 나타납니다:

```text theme={null}
Couldn't reach the background service (spawn background service: EUNKNOWN: unknown error, uv_spawn) — run 'claude daemon status'
```

일부 계정에서 메시지는 `background service` 대신 `daemon`을 말합니다.

npm 설치에서 `npm install -g @anthropic-ai/claude-code`가 바이너리를 대체하는 동안 나타나는 `EUNKNOWN`은 [`EACCES` during a reinstall](#eacces-when-starting-a-background-session)과 같은 원인을 가지며 설치가 완료된 후 재시도할 때 지워집니다.

Claude Code는 서비스가 터미널을 닫을 때 생존하도록 PowerShell을 통해 백그라운드 서비스를 시작하며, PowerShell 7이 설치되어 있으면 PowerShell 7을 사용하고 그렇지 않으면 Windows PowerShell 5.1을 사용합니다. PowerShell이 실행될 수 없으면 Claude Code는 대신 서비스를 직접 시작하므로 PowerShell만 차단하는 정책은 이 오류를 발생시키지 않습니다. npm 설치가 실행 중이 아닌 동안 이를 보면 정책이 Claude Code 실행 파일 자체를 차단합니다.

v2.1.212 이전에는 Claude Code가 Windows PowerShell 5.1만 사용하여 서비스를 시작했으므로 그룹 정책이 PowerShell 5.1을 차단한 모든 머신이 `Couldn't start the session — EUNKNOWN: unknown error, uv_spawn`으로 실패했으며, PowerShell 7이 설치되어 있었습니다.

**할 일:**

* 메시지가 `Couldn't start the session`을 읽으면 v2.1.212 이상으로 업그레이드합니다. 이전 버전에서는 별도의 터미널에서 `claude daemon run`을 먼저 실행한 다음 백그라운드 세션을 다시 시작할 수 있습니다. 해당 명령은 백그라운드 서비스를 터미널의 전경에서 실행하므로 서비스는 해당 터미널이 열려 있는 동안만 지속됩니다.
* npm 설치가 바이너리를 대체 중이면 완료될 때까지 기다린 다음 백그라운드 세션을 다시 시작합니다.
* npm 설치가 실행 중이 아닌 동안 v2.1.212 이상에서 오류가 나타나면 Windows 관리자에게 제한 정책에서 Claude Code 실행 파일을 허용하도록 요청합니다.
* 터미널을 닫을 때 백그라운드 서비스가 중지되면 Claude Code는 PowerShell 없이 시작했습니다. PowerShell 7을 설치하거나 관리자에게 PowerShell을 차단 해제하도록 요청하여 서비스가 터미널을 초과할 수 있도록 합니다.

<h3 id="eacces-when-starting-a-background-session">
  백그라운드 세션을 시작할 때 EACCES
</h3>

Claude Code는 [백그라운드 서비스](/docs/ko/agent-view#the-supervisor-process)를 시작하기 위해 자신의 바이너리를 실행할 수 없었습니다. npm 설치에서 이는 일반적으로 `npm install -g @anthropic-ai/claude-code`가 그 순간에 바이너리를 대체했다는 의미입니다. 직접 실행했든 [자동 업데이터](/docs/ko/setup#auto-updates)가 실행했든 상관없습니다. 오류는 [에이전트 뷰](/docs/ko/agent-view)에서 세션을 열 때 나타납니다:

```text theme={null}
Couldn't start the background service — spawn background service: EACCES: permission denied, posix_spawn '/usr/local/lib/node_modules/@anthropic-ai/claude-code/bin/claude'
```

`/background` 또는 `claude --bg`로 세션을 시작하면 같은 이유가 `Couldn't reach the background service (...)`에 나타납니다. 같은 재설치 창 동안 오류는 `ENOENT` 또는 `ENOEXEC`과 같은 다른 코드를 명시할 수 있으며, Windows에서는 `EUNKNOWN` 또는 `EPERM`을 명시할 수 있습니다. 재시도 전체에서 지속되는 `EUNKNOWN`은 [다른 원인](#eunknown-when-starting-a-background-session)을 가집니다.

npm 설치에서 Claude Code는 재설치가 완료될 때까지 대기하고 자동으로 재시도합니다: 최대 10초, 그리고 Claude Code의 npm 설치가 머신에서 여전히 명백히 실행 중인 동안 최대 2분입니다. 이는 다른 Claude Code 프로세스가 업데이트를 다운로드하는 것을 포함합니다. 설치가 해당 대기를 초과하면 실패는 베어 오류 코드 대신 업데이트를 명시합니다:

```text theme={null}
Claude Code is being updated by npm on this machine (still not runnable after 2 min, EACCES) — try again when the update finishes
```

v2.1.257 이전에는 대기가 모든 경우에 10초에서 중지되었으므로 다른 Claude Code 프로세스가 여전히 업데이트를 다운로드하는 동안 이 오류가 나타났습니다. v2.1.246 이전에는 Claude Code가 즉시 실패했으며 대기하지 않았습니다.

**할 일:**

* 몇 초 기다린 다음 세션을 열거나 다시 디스패치합니다. 메시지가 Claude Code가 업데이트 중이라고 말하면 업데이트가 완료된 후 재시도합니다.
* npm 설치가 실행 중이 아닌 동안 오류가 지속되면 사용자가 설치된 바이너리를 실행할 수 없습니다. 권한과 디렉토리를 확인하거나 Claude Code를 다시 설치합니다.

<h3 id="background-service-exited-before-it-became-reachable">
  백그라운드 서비스가 도달 가능해지기 전에 종료됨
</h3>

Claude Code가 [백그라운드 서비스](/docs/ko/agent-view#the-supervisor-process)로 시작한 프로세스가 연결을 수락하기 전에 종료되어 Claude Code가 세션을 열 수 없었습니다. 서비스가 종료되기 전에 오류를 인쇄했으면 괄호의 이유는 종료 코드 또는 신호와 서비스가 인쇄한 첫 번째 줄을 제공하며, 이는 중지된 것을 명시합니다:

```text theme={null}
Couldn't reach the background service (background service exited before it became reachable (exit code N): <the service's first error line>) — run 'claude daemon status'
```

[에이전트 뷰](/docs/ko/agent-view)에서 세션을 열 때 같은 이유가 `Couldn't start the background service —`를 따릅니다. 서비스가 종료되기 전에 아무것도 인쇄하지 않으면 메시지는 `nothing on stderr`을 대신 말합니다.

Claude Code는 서비스의 오류 줄로 실패를 보고합니다. v2.1.246 이전에는 실패가 45초 대기 후에만 표시되었으며, `background service did not become reachable within 45s`로 서비스의 오류 줄 없이 표시되었습니다.

두 인용된 이유는 알려진 원인을 가집니다:

* `Error: claude native binary not installed.`: npm 설치가 그 순간에 Claude Code 바이너리를 대체했으므로 서비스가 npm의 자리 표시자를 대신 실행했습니다. 설치가 완료된 후 재시도합니다. 설치가 실행 중이 아닌 동안 줄이 지속되면 [npm 설치를 완료](/docs/ko/troubleshoot-install#native-binary-not-found-after-npm-install)합니다. v2.1.257 이전에는 macOS npm 자체 업데이트가 설치 창 동안 모든 시작에서 이 실패를 생성했습니다.
* Windows에서 모든 시작에서 종료 코드 1로 `nothing on stderr`: `daemon.lock`은 Claude Code가 신호를 보낼 수도 없고 종료되었음을 증명할 수도 없는 프로세스를 명시하므로 각 새 서비스는 다른 서비스가 잠금을 보유한다고 결론짓고 종료합니다. Claude Code가 종료되었음을 증명할 수 있는 잠금은 자동으로 대체되며 이 실패를 생성하지 않습니다. 실패가 모든 시작에서 반복되면 `~/.claude/daemon.lock`을 삭제한 다음 세션을 열거나 다시 디스패치합니다. v2.1.257 이전에는 그러한 잠금이 파일을 삭제할 때까지 모든 시작을 차단했습니다.

**할 일:**

* 메시지가 줄을 인용하면 명시하는 것을 수정한 다음 세션을 열거나 다시 디스패치합니다. 다음 시도는 서비스를 다시 시작합니다.
* `claude daemon status`를 실행하여 서비스가 지금 실행 중인지 확인합니다.

<h3 id="working-directory-no-longer-exists-when-starting-a-background-session">
  백그라운드 세션을 시작할 때 작업 디렉토리가 더 이상 존재하지 않음
</h3>

더 이상 존재하지 않는 디렉토리에서 [백그라운드 세션](/docs/ko/agent-view)을 시작하려고 했습니다. 이는 에이전트 뷰에서 디스패치하거나 작업 중인 디렉토리가 삭제되거나 이동된 후 `/background`를 실행할 때 발생합니다. 또한 프로세스가 종료되고 디렉토리가 없는 세션에 연결하거나 다시 시작할 때 발생합니다. 새 프로세스가 같은 디렉토리에서 시작되기 때문입니다. Claude Code는 세션을 시작하지 않으며 메시지는 누락된 디렉토리를 명시합니다:

```text theme={null}
Couldn't start a background session (working directory no longer exists or is not accessible: /tmp/demo)
```

v2.1.257 이전에는 세션이 시작된 것처럼 보였다가 에이전트 뷰에서 같은 이유로 실패한 행으로 표시되었습니다.

**할 일:**

* 메시지가 명시하는 디렉토리를 다시 만들거나 존재하는 디렉토리에서 디스패치한 다음 다시 시도합니다.

<h2 id="wrapper-and-ide-errors">
  래퍼 및 IDE 오류
</h2>

이러한 오류는 Claude Code 자체가 아니라 IDE 확장 프로그램이나 [Agent SDK](/docs/ko/agent-sdk/overview) 애플리케이션과 같이 Claude Code를 실행한 프로그램에서 발생합니다.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code 프로세스가 코드 N으로 종료됨
</h3>

기본 `claude` 프로세스가 0이 아닌 코드로 종료되었습니다. 종료 코드만으로는 무엇이 실패했는지 알 수 없습니다. 실제 오류는 프로세스 자체의 출력에 있으며, 래퍼가 캡처한 경우 이를 추가하고 그렇지 않으면 로그에 유지합니다.

```text theme={null}
Error: Claude Code process exited with code 1
```

Windows에서 기본 빌드는 턴이 완료된 직후 코드 `4294967295`로 종료될 수 있습니다. 해당 종료가 턴 경계에서 발생하고 대기 중인 메시지가 없으며 백그라운드 작업이 실행 중이 아닐 때 [VS Code 확장 프로그램](/docs/ko/vs-code)은 이 오류를 표시하지 않고 세션을 조용히 닫습니다. 다음 메시지는 대화를 재개합니다.

v2.1.273 이전에는 확장 프로그램이 아무것도 손실되지 않았음에도 불구하고 모든 턴 경계에서 해당 종료에 대한 오류를 표시했습니다.

**수행할 작업:**

* VS Code에서 오류와 함께 표시되는 **View output logs** 링크를 따라 기본 오류를 확인합니다.
* Agent SDK 애플리케이션에서 메시지 루프 주변의 오류를 캡처합니다. [CLI process exit](/docs/ko/agent-sdk/troubleshooting#cli-process-exit) 아래의 항목은 각 SDK 언어에서 코드가 수신하는 내용을 다룹니다.
* 터미널에서 동일한 프로젝트에서 `claude`를 실행합니다. 오류는 일반적으로 실제 오류 메시지와 함께 재현되며, 이를 이 페이지에서 조회할 수 있습니다.
* 터미널에서 `claude doctor`를 실행하여 설치 및 구성을 확인합니다.

<h3 id="could-not-locate-the-claude-cli-on-path">
  PATH에서 Claude CLI를 찾을 수 없음
</h3>

[VS Code 확장 프로그램](/docs/ko/vs-code)은 Windows에서 통합 터미널에서 Claude Code를 열 때, 터미널의 셸이 PowerShell이고 확장 프로그램이 PATH에서 설치된 `claude` 실행 파일을 찾을 수 없을 때 이 오류를 표시합니다. 확장 프로그램은 PATH에서 설치된 `claude`를 찾을 때까지 Claude Code 실행을 거부합니다.

```text theme={null}
Failed to run Claude Code: Error: Could not locate the Claude CLI on PATH. Launching by name in a PowerShell terminal would run a 'claude' from the open folder instead of the installed CLI, so the launch was blocked. Make sure the Claude CLI's install directory is on your system PATH (not only your PowerShell profile), then restart VS Code and try again. VS Code reads PATH when it starts, so PATH changes take effect only after a restart.
```

**수행할 작업:**

* VS Code 외부에서 새 PowerShell 창을 열고 `where.exe claude`를 실행합니다. 경로를 출력하지 않으면 CLI가 PATH에 없습니다. [Verify your PATH](/docs/ko/troubleshoot-install#verify-your-path)를 따라 설치 디렉터리를 추가합니다. 경로를 출력하면 항목은 PowerShell 프로필에서 오거나 VS Code가 아직 선택하지 않은 PATH 변경에서 옵니다. 다음 두 단계는 이러한 경우를 다룹니다.
* PowerShell 프로필이 아닌 사용자 또는 시스템 환경 변수로 PATH 항목을 설정합니다. 확장 프로그램은 프로필을 실행하지 않으므로 거기에만 있는 PATH 편집은 절대 도달하지 않습니다.
* PATH를 변경한 후 VS Code를 다시 시작합니다. 확장 프로그램은 VS Code가 시작할 때 캡처한 PATH를 확인하므로 PATH 변경은 다시 시작한 후에만 적용됩니다.

<h3 id="the-connection-to-claude-code-ended-before-this-message-completed">
  Claude Code로의 연결이 이 메시지가 완료되기 전에 종료됨
</h3>

[VS Code 확장 프로그램](/docs/ko/vs-code)이 메시지를 `claude` 프로세스로 보냈고, 프로세스가 이를 인정하거나 완료하기 전에 오류 없이 연결이 종료되었습니다. 확장 프로그램은 메시지가 처리되었는지 여부를 알 수 없으므로 다시 보내도록 요청합니다.

```text theme={null}
The connection to Claude Code ended before this message completed — it may not have been processed, so please send it again.
```

**수행할 작업:**

* 메시지를 다시 보냅니다. 다음 메시지는 대화를 재개하는 새로운 `claude` 프로세스를 시작합니다.
* 반복되면 동일한 프로젝트의 터미널에서 `claude`를 실행합니다. 프로세스를 계속 종료하는 오류는 일반적으로 실제 오류 메시지와 함께 재현됩니다.

<h2 id="rewind-warnings-and-errors">
  되돌리기 경고 및 오류
</h2>

이 메시지들은 [`/rewind`](/docs/ko/checkpointing) 코드 복원에서 나옵니다. `Restored the code, but skipped N files`는 Claude Code가 일부 경로를 건너뛰었다는 경고입니다. `No files were restored`는 아무것도 복원하지 못했다는 의미의 오류입니다.

<h3 id="restored-the-code-but-skipped-files">
  코드는 복원되었지만 파일을 건너뜀
</h3>

`/rewind` 코드 복원이 추적된 경로 중 하나 이상을 작성하거나 삭제하지 않고 건너뛰었습니다. Claude Code는 다음과 같은 경우에 경로를 건너뜁니다:

* 심볼릭 링크, 하드 링크 또는 기타 일반 파일이 아닌 경우
* 체크포인트 이후 디렉토리가 변경된 경우
* 백업을 안전하게 읽을 수 없는 경우

건너뛴 경로는 현재 내용을 유지합니다. v2.1.216 이전에는 `/rewind`가 추적된 경로의 링크를 통해 작성하고 삭제했으며, 부분 복원을 보고하지 않았습니다.

```text theme={null}
Restored the code, but skipped 2 files: the tracked path is (or became) a link or other non-regular file, its directory changed since the checkpoint, or its backup could not be safely read. Skipped files were left untouched — run with --debug for the paths.
```

**해야 할 일:**

* 건너뛴 파일을 식별하여 아래 단계로 각각을 처리할 수 있도록 합니다. 메시지는 개수만 제공하므로, 복원이 실행될 때 건너뛴 각 경로를 이름으로 나열하는 디버그 로그 `~/.claude/debug/<session-id>.txt`를 확인하고, 다음 복원 전에 `/debug`로 디버그 로깅을 켭니다. macOS 또는 Linux에서는 대신 링크를 직접 찾을 수 있습니다: 심볼릭 링크는 `find . -type l`, 하드 링크된 파일은 `find . -type f -links +1`을 사용합니다.
* 건너뛴 파일이 dotfile 관리자가 관리하는 설정 파일이나 pnpm과 같은 도구로 하드 링크된 파일처럼 의도적으로 만든 링크인 경우, 되돌리기는 해당 내용을 그대로 두었습니다. 세션의 변경 사항을 취소하려면 Claude에게 편집을 되돌리도록 요청하거나 파일을 직접 편집합니다.
* 링크를 만들지 않았다면, 해당 경로의 내용을 신뢰하기 전에 검사합니다: 체크포인트 이후 무언가가 파일을 대체했을 수 있습니다.

<h3 id="no-files-were-restored">
  파일이 복원되지 않음
</h3>

Claude Code는 [`/rewind`](/docs/ko/checkpointing)로 코드를 복원할 때 해당 체크포인트의 파일 중 어느 것도 복원할 수 없으면 이 메시지를 표시합니다. 각 파일에 대해 Claude Code가 편집하기 전에 저장한 백업이 누락되었거나, Claude Code가 파일을 작성하거나 삭제할 수 없습니다.

```text theme={null}
Failed to restore the code:
No files were restored: 1 file failed (backup missing, or the file could not be updated)
```

Claude Code는 [보존 스윕](/docs/ko/claude-directory#cleaned-up-automatically)에서 세션의 백업을 삭제하며, 기본적으로 세션이 마지막으로 저장한 후 약 30일 후입니다. 그 이후에 세션을 재개하면 `/rewind`는 여전히 체크포인트를 나열하지만, 그 중 하나로 되돌리면 이 오류로 실패할 수 있습니다. 메시지에 `N paths were skipped for link safety`도 표시되면, 해당 경로에 대해 [코드는 복원되었지만 파일을 건너뜀](#restored-the-code-but-skipped-files)을 참조합니다.

세션을 포크할 때, 예를 들어 [`--fork-session`](/docs/ko/cli-reference#cli-flags) 또는 [`/branch`](/docs/ko/sessions#branch-a-session)를 사용하면, Claude Code는 원본 세션의 백업을 포크로 복사합니다. Claude Code가 백업을 복사할 수 없을 때, 예를 들어 디스크가 가득 찼을 때, 해당 백업은 포크에서 누락됩니다. 이를 필요로 하는 체크포인트로 되돌리면 이 오류로 실패할 수 있습니다.

**해야 할 일:**

* 다른 방법으로 변경 사항을 취소합니다: Claude에게 편집을 되돌리도록 요청하거나, 버전 제어에서 파일을 복원합니다. 백업이 없으면 `/rewind`를 다시 실행해도 같은 방식으로 실패합니다.
* Claude Code가 파일을 작성하거나 삭제할 수 없으면, 파일 권한과 같이 작성을 차단하는 것을 수정한 후 `/rewind`를 다시 실행합니다.
* 향후 세션에서 백업을 더 오래 유지하려면 [`cleanupPeriodDays`](/docs/ko/settings-reference#cleanupperioddays)를 높입니다.

v2.1.260 이전에는 Claude Code가 백업이 누락된 파일을 자동으로 건너뛰었으며, 되돌리기가 성공한 것처럼 보였습니다.

<h2 id="session-saving-warnings">
  세션 저장 경고
</h2>

Claude Code는 세션 트랜스크립트를 저장하지 않을 때 입력 상자 아래의 지속적인 줄에 이러한 경고를 표시합니다. 세션은 어느 쪽이든 계속 작동합니다. 경고는 나중에 [`--resume`](/docs/ko/sessions)에서 세션이 누락될 수 있음을 알려줍니다.

<h3 id="transcript-writes-are-failing">
  트랜스크립트 쓰기가 실패하고 있습니다
</h3>

Claude Code는 작업할 때 트랜스크립트를 디스크에 저장하며, [트랜스크립트 파일](/docs/ko/sessions#where-transcripts-are-stored)에 대한 쓰기가 실패하고 있습니다. 메시지는 원인을 기본 오류 코드와 함께 이름 지정합니다. 예를 들어 디스크가 가득 찬 경우:

```text theme={null}
Transcript writes are failing (disk full — ENOSPC) · recent messages may not be saved for resume
```

경고는 오류에 따라 다른 지점에 나타납니다:

* 자체적으로 해결되지 않는 조건의 첫 번째 실패: 디스크 가득 참, 디스크 할당량 초과, 읽기 전용 파일 시스템, 파일 시스템의 길이 제한을 초과하는 경로, 또는 macOS 및 Linux에서 권한 오류
* 최소 1분에 걸친 반복 실패(다른 모든 경우 포함): Windows의 권한 오류(바이러스 백신 스캔이 단일 쓰기를 실패하게 한 후 재시도 시 성공할 수 있음)

v2.1.217 이전에는 Claude Code가 경고 없이 실패한 쓰기를 삭제했으며, 나중에 `--resume`에서 최근 메시지가 누락된 것이 첫 번째 신호였습니다.

**할 일:**

* 오류 코드가 이름 지정하는 조건을 수정합니다: `ENOSPC`의 경우 디스크 공간 확보; `EDQUOT`의 경우 할당량 올리기 또는 지우기; `EACCES`, `EPERM` 또는 `EROFS`의 경우 트랜스크립트 위치에 대한 쓰기 액세스 복구
* 경고는 다음 성공적인 쓰기에서 자동으로 지워집니다. 재시작이 필요하지 않습니다.
* 경고가 표시되는 동안 전송된 메시지는 나중에 세션을 재개할 때 여전히 누락될 수 있습니다.

<h3 id="transcript-saving-is-off-skip-prompt-history">
  CLAUDE\_CODE\_SKIP\_PROMPT\_HISTORY가 설정되어 있어서 트랜스크립트 저장이 꺼져 있습니다
</h3>

이 세션은 [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/ko/env-vars)가 설정된 상태로 시작되었으므로 Claude Code는 이에 대한 트랜스크립트나 프롬프트 기록을 작성하지 않습니다:

```text theme={null}
Transcript saving is off — CLAUDE_CODE_SKIP_PROMPT_HISTORY is set · --resume will not find this session; if unintended, unset it and restart
```

이 변수는 임시 스크립팅된 세션에 대한 의도적인 옵트아웃이지만, 셸 프로필, 래퍼 스크립트 또는 이를 내보낸 부모 프로세스를 통해 세션에 도달할 수도 있습니다.

**할 일:**

* 의도적으로 변수를 설정한 경우 조치가 필요하지 않습니다. 공지는 세션이 `--resume`, `--continue` 또는 위쪽 화살표 기록에 나타나지 않음을 확인합니다.
* 그렇지 않은 경우 `claude`를 시작하는 셸 또는 스크립트에서 변수를 제거한 다음 새 세션을 시작합니다. 현재 세션의 메시지는 소급하여 저장되지 않습니다.

<h3 id="transcript-saving-is-off-child-session-marker">
  상속된 CLAUDE\_CODE\_CHILD\_SESSION 마커로 인해 트랜스크립트 저장이 꺼져 있습니다
</h3>

Claude Code는 생성하는 서브프로세스에서 [`CLAUDE_CODE_CHILD_SESSION`](/docs/ko/env-vars)을 설정하고, 이를 상속하는 대화형 세션을 중첩된 것으로 취급합니다: Claude Code는 이에 대한 트랜스크립트를 저장하지 않으므로 Claude 자체가 시작하는 세션이 `--resume` 목록을 채우지 않습니다. 이 공지는 현재 세션이 마커를 상속했음을 의미합니다:

```text theme={null}
Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker · restart with CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1 to keep future transcripts
```

공지는 다른 Claude Code 세션 내부에서 `claude`를 실행했을 때 예상되며, 마커가 오래 지속되는 중간 매개체(예: 터미널, `screen` 세션 또는 Claude Code 세션이 원래 시작한 런처)를 통해 누출되었을 때 오분류를 신호합니다.

tmux 내에서 Claude Code는 tmux 서버의 전역 환경을 통해 도착한 마커를 감지하고 저장을 계속하므로 이 경우 이 공지가 나타나지 않습니다.

**할 일:**

* 의도적으로 다른 Claude Code 세션 내부에서 이 세션을 시작한 경우 조치가 필요하지 않습니다.
* 이것이 최상위 세션인 경우 [`CLAUDE_CODE_FORCE_SESSION_PERSISTENCE=1`](/docs/ko/env-vars)을 설정하여 종료하고 재시작합니다. 저장은 재시작부터 적용되므로 이전에 전송된 메시지는 저장되지 않습니다.
* 동일한 터미널 또는 런처에서 향후 시작을 수정하려면 해당 환경에서 `CLAUDE_CODE_CHILD_SESSION`을 제거합니다.

<h2 id="configuration-warnings">
  구성 경고
</h2>

Claude Code는 이러한 메시지의 대부분을 stderr에 기록하며, 대부분을 시작 시에 기록합니다. 항목이 디버그 로그나 대화 보기의 시작 알림 같은 다른 곳에 나타나거나 [인식되지 않은 모델 진단 라인](#unrecognized-model-id-on-a-request)처럼 요청 시간에 나타나는 경우 그렇게 표시됩니다.

<h3 id="fullscreen-failed-start-notice">
  전체 화면 렌더러가 시작을 완료하지 못함
</h3>

이 머신의 이전 [전체 화면](/docs/ko/fullscreen) 세션이 시작을 완료하기 전에 종료되었으므로 Claude Code는 이 세션을 클래식 렌더러에서 시작하고 다음 알림 중 하나를 출력합니다:

```text theme={null}
Claude Code의 전체 화면 렌더러가 지난번 이 머신에서 시작을 완료하지 못했으므로 이번 실행은 클래식 렌더러를 사용합니다. 다음 실행에서 전체 화면을 다시 시도할 것입니다. /tui default는 클래식 렌더러를 유지합니다.

Claude Code의 전체 화면 렌더러가 이 머신에서 반복적으로 시작에 실패했으므로 여기서 비활성화되었습니다. /tui fullscreen을 실행하여 다시 시도하세요(이는 업데이트 후에도 재설정됩니다).
```

**할 일:**

* [전체 화면 렌더링](/docs/ko/fullscreen#fullscreen-renderer-didnt-finish-starting)을 따르세요. 어떤 알림을 받는지, Claude Code가 이후 세션에서 무엇을 하는지, 그리고 전체 화면을 다시 시도하거나 클래식 렌더러를 유지하는 방법을 설명합니다.
* 종료된 세션이 종료 메시지를 출력했다면 [복구할 수 없는 인터페이스 오류 후 Claude Code 종료](#exited-after-an-unrecoverable-interface-error)를 참조하여 이름이 지정된 내용을 확인하세요.

v2.1.236 이전에는 Claude Code가 알림을 출력하지 않았고 실패한 시작 후에도 계속 전체 화면 렌더링에서 세션을 시작했습니다.

<h3 id="exited-after-an-unrecoverable-interface-error">
  복구할 수 없는 인터페이스 오류 후 Claude Code 종료
</h3>

Claude Code는 터미널 인터페이스가 복구할 수 없는 오류에 부딪혔을 때 종료되며, 두 렌더러 중 하나에서 이 메시지를 출력합니다. 두 번째 문장은 [전체 화면](/docs/ko/fullscreen) 렌더러가 시작되는 동안 오류가 발생했을 때만 나타납니다:

```text theme={null}
Claude Code가 복구할 수 없는 인터페이스 오류(<error>)로 인해 종료되었습니다. 전체 화면 렌더러가 시작되는 동안 발생했으므로 다음 실행은 클래식 렌더러를 사용할 것입니다(CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1은 언제든지 이를 강제합니다).
```

**할 일:**

* Claude Code를 다시 시작하세요. 대화를 다시 시작하려면 같은 디렉토리에서 `claude --resume`을 실행하세요.
* 메시지가 전체 화면 렌더러를 언급하면 [전체 화면 렌더링](/docs/ko/fullscreen#fullscreen-renderer-didnt-finish-starting)에서 다음 실행이 무엇을 하는지, 전체 화면을 어떻게 켰는지에 따라 달라지는지, 그리고 전체 화면을 다시 시도하거나 클래식 렌더러를 유지하는 방법을 설명합니다.

v2.1.236 이전에는 Claude Code가 이런 종류의 오류 후 메시지를 출력하지 않고 종료되었습니다.

<h3 id="agent-descriptions-are-over-the-15000-token-limit">
  에이전트 설명이 15.0k 토큰 제한을 초과함
</h3>

Claude Code는 stderr가 아닌 대화 보기의 시작 알림으로 이 경고를 표시합니다. 기본 제공 에이전트를 제외한 [서브에이전트](/docs/ko/sub-agents)의 결합된 설명이 Claude Code가 추정하는 15,000 토큰을 초과합니다. 각 에이전트는 이름과 `description` frontmatter를 계산합니다. Claude Code는 총합이 제한을 초과하는지 여부와 관계없이 모든 에이전트를 로드하므로 경고는 로드되는 내용을 변경하지 않습니다.

```text theme={null}
에이전트 설명이 15.0k 토큰 제한을 초과함(~16.2k 토큰) · Claude에게 .claude/agents/의 에이전트 설명을 정리하도록 요청하세요
```

**할 일:**

* 에이전트 파일의 `description` frontmatter를 단축하거나 Claude에게 정리하도록 요청하세요.
* 더 이상 사용하지 않는 에이전트 파일을 제거하세요.

<h3 id="workspace-has-not-been-trusted">
  작업 공간이 신뢰되지 않음
</h3>

Claude Code는 프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json`에서 `permissions.allow` 규칙 또는 `permissions.additionalDirectories` 항목을 찾았지만 [프로젝트 설정의 allow 규칙은 작업 공간 신뢰가 필요](/docs/ko/permissions#project-allow-rules-and-workspace-trust)하기 때문에 적용하지 않았습니다. 개수, 설정 이름, 메시지에 명시된 파일은 구성에 따라 다릅니다. `deny` 및 `ask` 규칙은 영향을 받지 않습니다.

```text theme={null}
.claude/settings.local.json의 2개 permissions.allow 항목을 무시합니다: 이 작업 공간이 신뢰되지 않았습니다. 여기서 Claude Code를 대화형으로 한 번 실행하고 신뢰 대화를 수락하거나 /Users/you/.claude.json에서 projects["/Users/you/project"].hasTrustDialogAccepted: true를 설정하세요.
```

**할 일:**

* 디렉토리에서 `claude`를 실행하고 신뢰 대화를 수락하세요. [프로젝트 allow 규칙 및 작업 공간 신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust)에서 해당 수락이 어느 폴더를 포함하는지 설명합니다.
* [비대화형 모드](/docs/ko/headless)에서 `-p`를 사용하면 대화가 표시되지 않습니다. 메시지가 출력하는 정확한 `projects` 키를 사용하여 `~/.claude.json`에서 `hasTrustDialogAccepted` 항목을 설정하세요.
* 메시지가 `.claude/settings.local.json`을 언급하고 git 저장소 외부 또는 홈 디렉토리에서 Claude Code를 시작했다면 v2.1.200 이상으로 업데이트하세요. 버전 2.1.196부터 2.1.199까지는 해당 작업 공간에서 자신의 `.claude/settings.local.json`을 저장소 제공으로 취급했습니다. v2.1.207 이상에서는 git 저장소 외부에서 폴더를 신뢰하지 않은 경우 업데이트만으로는 충분하지 않습니다: 폴더가 저장소 내부에 있지 않은지 확인하면 git을 실행하고 Claude Code는 신뢰 대화를 수락한 후에만 해당 검사를 실행하므로 첫 번째 단계를 사용하세요. 홈 디렉토리 및 기타 [구성 홈](/docs/ko/permissions#project-allow-rules-and-workspace-trust)은 면제되며 대화를 기다리지 않습니다. [프로젝트 allow 규칙 및 작업 공간 신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust)를 참조하세요.

<h3 id="working-directory-is-a-network-path">
  작업 디렉토리가 네트워크 경로임
</h3>

Claude Code는 네트워크 경로를 작업 디렉토리로 추가하지 않습니다. 네트워크 경로를 조회하면 이름이 지정된 호스트에 연결할 수 있으며, Windows에서는 해당 연결이 호스트에 자격 증명을 보낼 수 있으므로 Claude Code는 조회 없이 경로를 거부합니다. `/add-dir`을 이러한 경로로 실행할 때 또는 시작 시 경고로 이 메시지를 봅니다. 시작 시 나타나면 Claude Code는 해당 디렉토리 없이 시작합니다.

```text theme={null}
\\server\share는 네트워크 경로이므로 작업 디렉토리로 추가할 수 없습니다. Windows에서는 공유를 드라이브 문자로 매핑하고 --add-dir로 실행 시 전달하세요(세션 중간에 추가된 드라이브 문자는 아직 원격 읽기 신뢰를 수행하지 않습니다).
```

Claude Code가 이 방식으로 거부하는 경로는 다음을 포함합니다:

* `\\server\share` 같은 UNC 공유
* `/net/<host>` 같은 자동 마운트 경로(해당 호스트의 자동 마운트 아래 디렉토리에서 Claude Code를 실행한 경우 제외)
* 기호 링크 또는 접합을 통해 네트워크 위치에 도달하는 로컬 경로

매핑된 드라이브 문자 및 `\\wsl$` 경로는 네트워크 경로로 계산되지 않습니다.

**할 일:**

* Windows에서는 공유를 드라이브 문자로 매핑하세요(예: `net use Z: \\server\share`). 그리고 `claude --add-dir Z:\`로 실행 시 드라이브를 전달하세요.
* macOS 또는 Linux에서는 공유를 로컬 경로에 마운트하고 해당 경로를 대신 추가하세요.
* 경로가 `permissions.additionalDirectories`에 있으면 이를 나열하는 설정 파일에서 제거하세요.

v2.1.257 이전에는 Claude Code가 도달 가능한 네트워크 경로를 작업 디렉토리로 수락했습니다.

<h3 id="remote-managed-settings-failed-to-load">
  원격 관리 설정을 로드하지 못함
</h3>

세션이 [서버 관리 설정](/docs/ko/server-managed-settings)에 적합하지만 Claude Code가 이를 가져올 수 없어서 대화형 세션에 이 경고를 표시합니다. 괄호로 묶인 원인은 `network error`, `request timed out`, 또는 `authentication rejected (401)` 같은 실패한 내용을 명시하고, 줄의 나머지는 세션이 실행되는 정책을 나타냅니다:

* **이전 성공적인 가져오기에서 캐시된 설정**: Claude Code는 [제외된 환경 변수](/docs/ko/server-managed-settings#fetch-and-caching-behavior)를 제외하고 캐시된 정책에서 세션을 실행하며, 줄은 `using cached policy`를 읽습니다.
* **캐시 없음**: Claude Code는 서버 관리 설정 없이 세션을 실행하며, 줄은 `no remote policy applied`를 읽습니다.

**할 일:**

* 메시지가 명시하는 원인에 대해 조치하세요: 네트워크 원인의 경우 이 머신이 `api.anthropic.com`에 도달할 수 있는지 확인하세요. 인증 원인의 경우 `/status`로 로그인을 확인하세요.
* 전체 진단을 위해 `/status` 또는 `claude doctor`를 실행하세요.

v2.1.248 이전에는 Claude Code가 실패한 설정 가져오기를 디버그 로그에만 보고했습니다.

<h3 id="managed-settings-were-not-approved">
  관리 설정이 승인되지 않음
</h3>

조직의 [서버 관리 설정](/docs/ko/server-managed-settings)에 승인이 필요한 설정이 포함되어 있고 [보안 승인 대화](/docs/ko/server-managed-settings#security-approval-dialogs)를 거부했으므로 Claude Code는 이를 적용하지 않고 종료됩니다:

```text theme={null}
관리 설정이 승인되지 않았습니다. 이를 적용하지 않고 종료합니다.
```

**할 일:**

* Claude Code를 다시 시작하고 대화를 승인하여 조직의 설정에서 계속하세요. 거부된 대화는 기억되지 않으므로 다음 시작 시 다시 나타납니다.
* 대화가 나열하는 설정에 대해 확실하지 않으면 승인하기 전에 조직의 관리 설정을 유지하는 사람에게 문의하세요.

<h3 id="mcp-server-is-blocked-by-enterprise-managed-policy">
  MCP 서버가 엔터프라이즈 관리 정책에 의해 차단됨
</h3>

`/mcp`의 서버에서 **다시 연결**을 선택했거나 비활성화된 서버를 다시 켰는데, [MCP 서버를 제한](/docs/ko/managed-mcp)하는 설정이 해당 서버를 차단합니다. Claude Code는 이를 연결하기를 거부하고 다음을 표시합니다:

```text theme={null}
MCP 서버 <name>이(가) 엔터프라이즈 관리 정책에 의해 차단되었습니다
```

다음 설정 중 하나가 메시지를 생성할 수 있습니다:

* 자신의 `~/.claude/settings.json` 또는 프로젝트의 `.claude/settings.json`에 있는 것을 포함하여 서버와 일치하는 [`deniedMcpServers`](/docs/ko/managed-mcp#policy-based-control-with-allowlists-and-denylists) 항목
* 서버가 일치하지 않는 [`allowedMcpServers`](/docs/ko/managed-mcp#policy-based-control-with-allowlists-and-denylists) 목록
* `mcp` 잠금이 있는 [`strictPluginOnlyCustomization`](/docs/ko/settings-reference#strictpluginonlycustomization)으로, `~/.claude.json` 및 `.mcp.json`에서 구성된 서버를 차단합니다.
* 서버가 claude.ai 커넥터일 때 [`disableClaudeAiConnectors`](/docs/ko/mcp#disable-claude-ai-connectors)

**할 일:**

* 자신의 사용자 및 프로젝트 설정 파일에서 이러한 설정 중 하나를 확인하고 변경하거나 제거하세요.
* 자신의 설정이 차단을 설명하지 않으면 관리자에게 어떤 관리 설정이 서버를 차단하는지 문의하세요.

v2.1.257 이전에는 `/mcp`의 **다시 연결** 및 다시 활성화가 중간 세션 정책 업데이트가 차단한 서버를 연결할 수 있었습니다.

<h3 id="managed-settings-document-could-not-be-parsed">
  관리 설정 문서를 구문 분석할 수 없음
</h3>

조직이 [관리 설정](/docs/ko/managed-settings)을 배포하고, 배포된 문서 중 하나가 있지만 JSON 객체로 구문 분석할 수 없어서 Claude Code는 정책을 실행하지 않고 시작 시 코드 1로 종료됩니다. 줄은 메시지 앞에 실패한 소스를 명시합니다:

```text theme={null}
/Library/Application Support/ClaudeCode/managed-settings.json: 관리 설정 문서를 JSON 객체로 구문 분석할 수 없습니다. 해당 설정이 적용되지 않습니다. 수정하거나 제거하세요.
```

소스는 다음 중 하나입니다:

* `managed-settings.json` 파일의 경로 또는 `managed-settings.d` 아래의 드롭인 파일
* macOS 관리 기본 설정 프로필, `per-user managed preferences` 또는 `device-level managed preferences`
* Windows 레지스트리 값, `Registry: HKLM\SOFTWARE\Policies\ClaudeCode\Settings`

[Claude Code가 삭제한 항목 찾기](/docs/ko/managed-settings#find-entries-claude-code-dropped)에서 각 소스를 구문 분석할 수 없게 만드는 것을 나열합니다.

Claude Code는 다른 관리 소스가 유효한 정책을 제공하더라도 시작을 거부합니다. 대화형 세션, `claude -p`, Agent SDK 세션, [백그라운드 세션](/docs/ko/agent-view), 및 `claude doctor`를 포함한 대부분의 하위 명령에서 이 오류를 봅니다. 거부는 의도적으로 폐쇄됩니다: Claude Code가 구문 분석할 수 없는 문서의 설정은 적용될 수 없으며, 계속 진행하면 조직의 제어 없이 세션이 실행됩니다.

구문 분석 가능한 문서의 스키마 문제는 이 오류를 생성하지 않습니다. [Claude Code가 삭제한 항목 찾기](/docs/ko/managed-settings#find-entries-claude-code-dropped)에서 Claude Code가 하나를 사용하는 것을 다룹니다.

`managed-settings.d/` 디렉토리가 있지만 나열할 수 없으면 Claude Code는 `Managed settings drop-in directory could not be read:` 다음에 기본 오류를 보고합니다. [Claude Code가 삭제한 항목 찾기](/docs/ko/managed-settings#find-entries-claude-code-dropped)에서 읽기 실패가 시작 시 종료되는 경우를 다룹니다.

**할 일:**

* 머신을 관리하면 명시된 문서를 JSON 객체로 구문 분석하도록 수정하거나 파일, 프로필 또는 레지스트리 값을 제거하세요. 빈 `managed-settings.json`은 `{}`로 계산되며 실행을 차단하지 않습니다.
* 그렇지 않으면 관리자에게 배포된 문서를 수정하도록 요청하세요. 자신의 설정 파일의 아무것도 이 오류를 야기하거나 지우지 않습니다.

<h3 id="otelheadershelper-failed">
  otelHeadersHelper 실패
</h3>

Claude Code는 [`otelHeadersHelper`](/docs/ko/settings-reference#otelheadershelper) 스크립트가 실패하거나 [스크립트 요구 사항](/docs/ko/monitoring-usage#script-requirements)을 충족하지 않는 출력을 출력할 때 대화형 세션당 한 번 터미널 인터페이스에 알림으로 이 경고를 표시합니다.

스크립트가 계속 실패하는 동안 내보내기가 실패하고 원격 분석 백엔드는 세션에서 아무것도 받지 못합니다.

`See /status:` 뒤의 텍스트는 스크립트의 종료 코드 다음에 오류 출력 같은 실패한 내용을 명시합니다:

```text theme={null}
otelHeadersHelper 실패. 원격 분석이 내보내지지 않습니다. /status 참조: exited 1: token service unreachable
```

**할 일:**

* `/status`를 실행하여 실패 세부 정보를 읽으세요.
* 스크립트를 수정하여 30초 이내에 종료 0으로 종료하고 stdout에 문자열 헤더 값의 JSON 객체를 출력하도록 하세요. [스크립트 요구 사항](/docs/ko/monitoring-usage#script-requirements)을 참조하세요.
* 조직이 [관리 설정](/docs/ko/managed-settings)을 통해 스크립트를 배포하면 이를 유지하는 사람에게 수정하도록 요청하세요.

[비대화형 모드](/docs/ko/headless)에서 `-p`를 사용하면 같은 실패가 stderr에 `otelHeadersHelper failed (OpenTelemetry export headers unavailable): <error>` 대신 나타납니다.

<h3 id="headershelper-not-run">
  headersHelper가 실행되지 않음
</h3>

Claude Code는 정적 `headers`만으로 MCP 서버를 연결했고 서버의 [`headersHelper`](/docs/ko/mcp#use-dynamic-headers-for-custom-authentication)를 건너뛰었습니다. 헬퍼는 셸 명령이고 폴더에 저장된 신뢰가 없기 때문입니다. 폴더는 `~/.claude.json`에서 항목을 손으로 설정하거나, 홈 디렉토리 외부에서 대화형 세션에서 신뢰 대화를 수락할 때 저장된 신뢰를 얻습니다. [headersHelper가 실행되기 전에 폴더 신뢰](/docs/ko/mcp#trust-a-folder-before-its-headershelper-runs)에서 이 검사가 적용되는 서버를 참조하세요.

Claude Code는 [비대화형 모드](/docs/ko/headless)에서만 이 줄을 기록하며, 서버당 한 번입니다. 대화형 세션에서는 같은 거부를 디버그 로그에 기록합니다.

```text theme={null}
MCP 서버 'internal-api': headersHelper가 실행되지 않음 — 이 작업 공간에 지속된 신뢰가 없습니다. 여기서 대화형으로 신뢰 대화를 한 번 수락하거나 /Users/you/.claude.json에서 projects["/Users/you/project"].hasTrustDialogAccepted를 설정하세요.
```

메시지가 출력하는 `projects` 키는 [프로젝트 allow 규칙 및 작업 공간 신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust)가 Claude Code가 신뢰를 키하는 폴더입니다. 부모 폴더에 대한 신뢰 대화를 수락하는 것은 검사를 만족하지 않으며, `-p` 또는 SDK 세션도 만족하지 않습니다.

**할 일:**

* 메시지가 명시하는 폴더에서 `claude`를 실행하고 신뢰 대화를 수락한 다음 `-p` 또는 SDK 명령을 다시 실행하세요.
* 메시지가 출력하는 정확한 `projects` 키를 사용하여 `~/.claude.json`에서 `hasTrustDialogAccepted` 항목을 직접 설정하세요.
* 홈 디렉토리에서 세션을 시작했으면 신뢰한 프로젝트 디렉토리에서 작업하세요. 홈 디렉토리에서 신뢰 대화를 수락하면 Claude Code는 현재 세션에만 해당 신뢰를 유지합니다.

<h3 id="malformed-tool-content-rule">
  잘못된 Tool(content) 규칙
</h3>

설정 파일의 [권한 규칙](/docs/ko/permissions#permission-rule-syntax)이 `Tool` 또는 `Tool(content)` 형태를 갖지 않습니다. 예를 들어 닫는 괄호 뒤에 텍스트가 있거나 괄호 중 하나가 누락되었습니다. Claude Code는 규칙을 건너뛰고 대화형 세션이 시작될 때 유효하지 않은 설정 대화에 나열하며, [`claude doctor`](/docs/ko/debug-your-config#check-resolved-settings) 출력에도 나열합니다:

```text theme={null}
유효하지 않은 권한 규칙 "Bash(ls) x"가 건너뛰어졌습니다: 잘못된 Tool(content) 규칙. 규칙은 Tool 또는 Tool(content) 형태를 가지며 닫는 ")"에서 끝나야 합니다. 내용 내의 괄호는 리터럴입니다.
```

**할 일:**

* 메시지와 함께 나열된 설정 파일에서 규칙을 다시 작성하여 닫는 괄호에서 끝나도록 하세요. 예를 들어 `Bash(ls) x` 대신 `Bash(ls *)`
* 내용 내의 괄호는 그대로 두세요. 이들은 리터럴이므로 `Edit(./Finance (2024)/*)`와 같은 규칙은 이스케이프 없이 유효합니다.

v2.1.260 이전에는 Claude Code가 일치하지 않는 괄호가 있는 규칙을 `Mismatched parentheses`로 보고했습니다.

<h3 id="is-not-matched-by-file-permission-checks">
  파일 권한 검사와 일치하지 않음
</h3>

Claude Code는 [설정 파일](/docs/ko/settings#where-settings-live), [관리 설정](/docs/ko/managed-settings), 또는 `--allowedTools`, `--disallowedTools`, 또는 `--settings` 플래그 값에서 경로가 있는 `Write`, `NotebookEdit`, `MultiEdit`, 또는 `Glob` [권한 규칙](/docs/ko/permissions#read-and-edit)을 찾았습니다. 파일 권한을 `Edit` 및 `Read` 규칙에 대해서만 검사하므로 다른 파일 도구 중 하나를 명시하는 경로 규칙을 절대 참조하지 않습니다. 규칙을 유지하고 다른 것은 변경하지 않습니다. 경고는 규칙, 괄호의 소스, 그리고 작성할 대체를 명시합니다:

```text theme={null}
권한 deny 규칙(.claude/settings.json): Write(docs/**)는 파일 권한 검사와 일치하지 않습니다 — Edit(path) 규칙만 사용됩니다. 대신 Edit(docs/**)를 사용하세요(Edit 규칙은 모든 파일 편집 도구를 포함합니다).
```

**할 일:**

* `Write(path)`, `NotebookEdit(path)`, 및 레거시 `MultiEdit(path)` 규칙을 `Edit(path)`로 바꾸세요. `Edit` 규칙은 모든 파일 편집 도구를 포함합니다.
* `--allowedTools`를 제외하고, Claude Code가 경고 없이 `Glob` 규칙을 수락하며, `Glob(path)` 규칙을 `Read(path)`로 바꾸세요.
* 경고가 괄호에 명시하는 소스에서 규칙을 수정하세요: 설정 파일 경로, 또는 `--allowed-tools` 및 `--disallowed-tools`의 플래그 자체. 디스크에 존재하지 않는 `claude-settings-<hash>.json` 경로는 인라인 `--settings` 값을 나타냅니다. 해당 플래그에 전달하는 JSON을 수정하세요.
* `Write` 또는 `Glob` 같은 베어 도구 이름 규칙은 그대로 두세요. Claude Code는 [도구 수준](/docs/ko/permissions#match-all-uses-of-a-tool)에서 일치시키고 이들에 대해 경고하지 않습니다.
* 소스가 `managed policy settings`를 읽으면 경고를 관리 설정을 유지하는 사람에게 전달하세요. 자신이 직접 지울 수 없기 때문입니다.

[백그라운드 세션](/docs/ko/agent-view) 또는 `--output-format json` 또는 `stream-json`에서 Claude Code는 경고를 stderr 대신 디버그 로그에 기록하므로 머신 읽기 출력이 깨끗합니다. `~/.claude/debug/<session-id>.txt`에서 캡처하려면 `--debug`로 실행하세요. v2.1.210 이전에는 Claude Code가 경고 없이 이러한 규칙을 수락했습니다.

<h3 id="has-a-wildcard-before-the-rest-of-the-command">
  명령의 나머지 부분 앞에 와일드카드가 있음
</h3>

Claude Code는 `Bash(git * main)` 또는 `Bash(git -C * status *)`처럼 `*`가 명령을 결정하는 나중 단어 앞에 오는 `Bash` allow 규칙을 찾았습니다. 이는 [설정 파일](/docs/ko/settings#where-settings-live), [관리 설정](/docs/ko/managed-settings), 또는 `--allowedTools` 또는 `--settings` 플래그 값에 있습니다. `*`는 해당 위치에 삽입된 옵션을 포함한 모든 텍스트와 일치합니다: `Bash(git * main)`은 또한 `-c`가 명령이 명시하는 프로그램을 실행하게 하는 `git -c core.fsmonitor=<script> diff main`을 승인합니다. [와일드카드 패턴](/docs/ko/permissions#wildcard-patterns)은 일치 규칙을 보여줍니다.

경고는 의도한 것보다 와일드카드가 더 넓은 규칙을 좁힐 수 있도록 존재합니다. Claude Code는 규칙을 유지하고 일치 방식을 변경하지 않습니다. 경고는 규칙과 괄호의 소스를 명시합니다:

```text theme={null}
권한 allow 규칙(.claude/settings.json): Bash(git -C * status *)는 명령의 나머지 부분 앞에 와일드카드가 있으므로 해당 위치에 삽입된 모든 옵션과도 일치하고 프롬프트 없이 승인합니다. git의 경우 -c 및 --exec-path 같은 옵션은 임의의 명령을 실행할 수 있습니다. 해당 *를 의도한 정확한 값으로 바꾸거나 *를 하위 명령 뒤에만 사용하세요(예: Bash(git status *)).
```

**할 일:**

* 하위 명령 앞의 `*`를 의도한 정확한 값으로 바꾸세요: `Bash(git * main)` 대신 `Bash(git checkout main)`.
* 모든 `*`를 하위 명령 뒤로 이동하세요: `Bash(git -C * status *)` 대신 `Bash(git status *)`. 허용하려는 하위 명령당 하나의 규칙을 작성하세요.
* 경고가 괄호에 명시하는 소스에서 규칙을 수정하세요: 설정 파일 경로, 또는 `--allowed-tools` 플래그 자체. 디스크에 존재하지 않는 `claude-settings-<hash>.json` 경로는 인라인 `--settings` 값을 나타냅니다. 해당 플래그에 전달하는 JSON을 수정하세요.
* 소스가 `managed policy settings`를 읽으면 경고를 관리 설정을 유지하는 사람에게 전달하세요. 자신이 직접 지울 수 없기 때문입니다.

Claude Code는 같은 형태의 deny 및 ask 규칙에 대해 경고하지 않습니다: 승인하는 대신 일치하는 추가 명령을 거부하거나 프롬프트합니다. 또한 `Bash(git commit *)`처럼 하위 명령이 첫 `*` 앞에 오는 규칙이나 `Bash(git *)`처럼 `*` 뒤에 옵션 이외의 단어가 없는 규칙, 또는 `Bash(git:*)`처럼 `:*` 접두사 규칙에 대해서도 경고하지 않습니다.

[백그라운드 세션](/docs/ko/agent-view) 또는 `--output-format json` 또는 `stream-json`에서 Claude Code는 경고를 stderr 대신 디버그 로그에 기록하므로 머신 읽기 출력이 깨끗합니다. `~/.claude/debug/<session-id>.txt`에서 캡처하려면 `--debug`로 실행하세요. v2.1.246 이전에는 Claude Code가 경고 없이 이러한 규칙을 수락했습니다.

<h3 id="crosssessioninbound-must-be-one-of-accept-hold-refuse">
  crossSessionInbound는 accept, hold, refuse 중 하나여야 함
</h3>

설정 파일이 [`crossSessionInbound`](/docs/ko/settings-reference#crosssessioninbound)를 Claude Code가 인식하지 못하는 값(예: 오타 `"reject"`)으로 설정합니다. 경고의 두 번째 문장은 어느 파일이 값을 보유하는지에 따라 다릅니다. 사용자, 프로젝트, 로컬, 또는 `--settings` 파일에서는 다음을 읽습니다:

```text theme={null}
"crossSessionInbound"는 "accept", "hold", "refuse" 중 하나여야 합니다. "reject"를 받았습니다. 이 값은 무시되었습니다. 존재하는 동안 교차 세션 메시지는 전달되는 대신 승인을 위해 보류됩니다. 위의 값 중 하나로 설정하세요.
```

[관리 설정](/docs/ko/managed-settings)에서 Claude Code는 인식되지 않는 값을 가장 제한적인 값인 `refuse`로 취급하고 경고는 관리자가 수정할 때까지 교차 세션 메시지가 거부된다고 말합니다. hold가 다른 설정 파일의 값과 결합되는 방식은 [`crossSessionInbound`](/docs/ko/settings-reference#crosssessioninbound)를 참조하세요.

**할 일:**

* 키를 `"accept"`, `"hold"`, 또는 `"refuse"`로 설정하거나 제거하세요.
* 경고가 관리 설정을 명시하면 관리자에게 값을 수정하도록 요청하세요.

v2.1.248 이전에는 Claude Code가 인식되지 않는 값을 경고 없이 무시했습니다.

<h3 id="the-200k-limit-isnt-enforced">
  200K 제한이 적용되지 않음
</h3>

[`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/ko/env-vars)을 설정했으며, 이는 일반적으로 [자동 압축](/docs/ko/model-config#default-auto-compact-thresholds)이 1M 컨텍스트 모델의 세션을 200K 윈도우에 유지하게 하지만, 이 세션을 200K 이하로 제한하는 압축 임계값이 없어서 대화가 이를 초과할 수 있습니다.

```text theme={null}
CLAUDE_CODE_DISABLE_1M_CONTEXT가 설정되었지만 <model>에 대해 200K 제한이 적용되지 않으므로 이 세션은 이를 초과할 수 있습니다. 이를 적용하려면 CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000(또는 autoCompactWindow 설정)을 설정하세요.
```

Claude Code는 네이티브 1M 윈도우를 가진 것으로 인식하는 모든 모델과 인식하지 못하는 모델 ID에 대해 가정하는 윈도우에서 압축하는 200K 제한을 자체적으로 적용합니다. 경고는 다른 구성이 해당 적용을 무효화할 때 나타납니다:

* 모델 ID가 Claude Code가 인식하지 못하는 것입니다. 예를 들어 [LLM 게이트웨이](/docs/ko/llm-gateway) 별칭이고, [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/ko/env-vars)을 설정했거나 [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/ko/env-vars)로 가정 윈도우를 200K 이상으로 올렸습니다. 이 경우 메시지는 또한 `또는 <model>을(를) 인식하는 Claude Code 버전으로 업데이트`를 해결책으로 제공합니다.
* [`ANTHROPIC_BETAS`](/docs/ko/env-vars) 또는 [`--betas`](/docs/ko/cli-reference#cli-flags) 플래그를 통해 요청된 `context-1m` 베타는 여전히 해당 베타를 수락하는 모델에서 API에 1M 윈도우를 요청하는 동안 아무것도 200K에서 세션을 압축하지 않습니다.

**할 일:**

* [`CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000`](/docs/ko/env-vars)을 설정하거나 [`autoCompactWindow`](/docs/ko/settings-reference#autocompactwindow) 설정을 `200000`으로 설정하여 자동 압축이 200K 경계에서 압축하도록 하세요.
* 메시지가 이 버전이 인식하지 못하는 모델 ID를 명시하면 `claude update`를 실행하세요. ID를 1M 컨텍스트 모델로 인식하는 버전은 추가 구성 없이 제한을 적용합니다.
* 세션이 모델의 전체 윈도우를 대신 사용하기를 원하면 `CLAUDE_CODE_DISABLE_1M_CONTEXT`를 설정 해제하세요. 경고는 200K 제한이 적용되지 않는다는 것만 보고합니다.

[백그라운드 세션](/docs/ko/agent-view) 또는 `--output-format json` 또는 `stream-json`에서 Claude Code는 경고를 stderr 대신 디버그 로그에 기록합니다.

<h3 id="unrecognized-model-id-on-a-request">
  요청에서 인식되지 않은 모델 ID
</h3>

Claude Code는 Claude Code 버전이 인식하지 못하는 모델 ID에 대한 요청을 보냈고, 해당 ID를 인식하는 모델로 매핑하는 [`modelOverrides`](/docs/ko/model-config#override-model-ids-per-version) 항목을 찾지 못했습니다. Claude Code는 여전히 구성한 대로 ID를 사용하여 요청을 보내며, 종료하거나 모델을 전환하지 않습니다.

```text theme={null}
[claude-code:unrecognized_model] {"model":"my-proxy-model","query_source":"sdk"}
```

stderr를 읽는 스크립트 또는 하네스에서 `[claude-code:unrecognized_model]` 접두사와 일치하세요. 접두사와 한 공백 뒤에 Claude Code는 한 줄 JSON 객체를 기록합니다. Claude Code는 나중 버전에서 필드를 추가할 수 있으므로 예상하지 못한 필드는 무시하세요. 최소한 다음 두 가지를 기록합니다:

* `model`: 구성한 대로 모델 문자열
* `query_source`: 모델을 사용한 요청 경로. Claude Code는 `-p` 실행에 대해 `sdk`를 보고하고 서브에이전트에 대해 `agent:`로 시작하는 값을 보고합니다.

Claude Code는 실행 방식에 따라 두 곳 중 하나에 줄을 기록합니다:

* [비대화형 모드](/docs/ko/headless)에서 `-p`를 사용하면 Claude Code는 모든 `--output-format` 아래 stderr에 기록하므로 줄을 필터링하지 않고 stdout을 구문 분석할 수 있습니다.
* 대화형 세션 또는 [백그라운드 세션](/docs/ko/agent-view)에서 Claude Code는 디버그 로그에 기록합니다. `~/.claude/debug/<session-id>.txt`에서 캡처하려면 `--debug`로 실행하세요.

Claude Code는 프로세스당 모델 문자열당 한 번 줄을 기록합니다. [서브에이전트](/docs/ko/sub-agents#choose-a-model) 또는 [백그라운드 기능](/docs/ko/costs#background-token-usage)이 사용하는 것처럼 각 추가 인식되지 않은 ID에 대해 별도의 줄을 기록합니다.

Claude Code는 인식하는 모델로 해석하는 공급자 ID에 대해 줄을 기록하지 않습니다. 예를 들어 Amazon Bedrock `us.anthropic.claude-...` ID, Google Cloud의 `@` 버전 접미사가 있는 Agent Platform ID, Claude 모델 ID를 포함하는 Microsoft Foundry 배포 이름입니다. Claude Code는 ARN 자체가 아닌 Amazon Bedrock [애플리케이션 추론 프로필 ARN](/docs/ko/amazon-bedrock#map-each-model-version-to-an-inference-profile) 뒤의 모델을 확인합니다. 잘못된 것처럼 해석할 수 없는 ARN에 대해 줄을 기록하지 않습니다.

**할 일:**

* 의도적으로 ID를 설정한 경우(예: [LLM 게이트웨이](/docs/ko/llm-gateway) 별칭), ID를 값으로 하는 [`modelOverrides`](/docs/ko/model-config#override-model-ids-per-version) 항목을 [설정 파일](/docs/ko/settings#where-settings-live)에 추가하세요. 키로 `opus` 같은 패밀리 별칭이 아닌 Anthropic 모델 ID를 사용하세요. 예제 줄의 `my-proxy-model`에 대해 이 항목을 추가하세요:

  ```json theme={null}
  {
    "modelOverrides": {
      "claude-opus-4-6": "my-proxy-model"
    }
  }
  ```

  Claude Code는 `my-proxy-model`을 `claude-opus-4-6`으로 취급하고 줄 기록을 중지합니다.

* ID가 Claude Code 버전보다 새로운 모델을 명시하면 `claude update`를 실행하세요.

* ID가 오타이면 [모델을 설정할 수 있는 위치](/docs/ko/model-config#setting-your-model) 또는 [별칭 변수](/docs/ko/model-config#environment-variables) 중 이를 보유하는 곳에서 수정하세요. `query_source`가 `agent:`로 시작하면 [서브에이전트의 모델](/docs/ko/sub-agents#choose-a-model)을 설정하는 곳에서 대신 수정하세요.

v2.1.233 이전에는 Claude Code가 인식하지 못하는 모델 ID에 대한 요청을 보낼 때 줄을 기록하지 않았습니다.

<h3 id="stale-sandbox-mask-files-left-by-a-killed-session">
  종료된 세션이 남긴 오래된 샌드박스 마스크 파일
</h3>

`claude doctor`는 진단에서 이 경고를 출력하고 `/status`는 같은 줄을 나열합니다. [샌드박싱](/docs/ko/sandboxing)이 파일 시스템 격리를 켜서 활성화되면 Linux 및 WSL2에 나타납니다.

샌드박스된 명령이 실행되는 동안 샌드박스는 아직 존재하지 않는 파일에 대한 쓰기 거부를 0바이트 읽기 전용 자리 표시자를 만들어 유지하고 나중에 제거합니다. SIGKILL 같은 것으로 정리 실행 전에 종료된 세션은 자리 표시자를 남깁니다. 이후 세션은 모든 시작에서 읽기 전용으로 다시 바인드하므로 "Yes, and don't ask again" 저장 같은 설정 쓰기가 하나가 있는 곳에서 실패합니다.

```text theme={null}
- 종료된 세션이 남긴 오래된 샌드박스 마스크 파일: /home/you/project/.claude/settings.local.json
  수정: 해당 프로젝트에서 다른 Claude Code 세션이 실행되지 않는 동안 각각을 `rm <path>`로 제거하세요 — 설정 파일이 속한 곳의 0바이트 읽기 전용 파일은 "Yes, and don't ask again" 저장을 실패하게 하고 샌드박스는 모든 시작에서 읽기 전용으로 다시 바인드합니다.
```

**할 일:**

* 해당 프로젝트에서 실행 중인 다른 Claude Code 세션을 종료한 다음 `rm`으로 각 나열된 파일을 삭제하세요. 경고는 최대 3개 파일을 명시하고 나머지를 계산하므로 경고가 더 이상 나타나지 않을 때까지 삭제 후 `claude doctor`를 다시 실행하세요. 다른 세션의 샌드박스가 여전히 사용 중인 자리 표시자는 해당 세션의 쓰기 보호의 라이브 부분입니다.
* "Yes, and don't ask again"으로 저장한 권한 선택이 고정되지 않았으면 자리 표시자를 삭제한 후 다시 저장하세요.

v2.1.257 이전에는 `claude doctor`가 이러한 파일을 플래그하지 않았습니다. 이전 버전은 세션이 종료될 때 같은 자리 표시자를 남깁니다.

<h2 id="responses-seem-lower-quality-than-usual">
  응답 품질이 평소보다 낮아 보입니다
</h2>

Claude의 답변이 예상보다 덜 능력 있어 보이지만 오류가 표시되지 않는 경우, 원인은 일반적으로 모델 자체가 아니라 대화 상태입니다. Claude Code는 모델 버전을 자동으로 변경하지 않습니다. 세 가지 특정 경우에만 폴백 모델로 전환할 수 있습니다:

* 구성된 [`--fallback-model`](/docs/ko/cli-reference#cli-flags)은 가용성 오류 후 해당 턴에만 제어를 인수받으며, 트랜스크립트에 공지가 표시됩니다
* Amazon Bedrock 또는 Google Cloud의 Agent Platform 시작 확인에서 기본 모델을 사용할 수 없음을 발견합니다
* [자동 모델 폴백](/docs/ko/model-config#automatic-model-fallback)은 Fable 5.1, Fable 5, Opus 5.5, Opus 5에서 세션을 플래그된 카테고리의 폴백 모델로 이동하며, 해당 카테고리에 폴백 모델이 있을 때 트랜스크립트에 공지를 표시합니다

아래의 모델 선택 확인은 두 번째와 세 번째 경우를 포착합니다. 첫 번째는 `/model` 변경이 아니라 트랜스크립트 공지로 나타납니다. [모델 구성](/docs/ko/model-config)에서 각 폴백이 적용되는 시기를 설명합니다.

먼저 다음을 확인하십시오:

* **모델 선택**: `/model`을 실행하여 예상하는 모델에 있는지 확인합니다. 이전 `/model` 선택 또는 `ANTHROPIC_MODEL` 환경 변수로 인해 의도한 것보다 작은 모델에 있을 수 있습니다.
* **노력 수준**: `/effort`를 실행하여 현재 추론 수준을 확인하고 어려운 디버깅 또는 설계 작업을 위해 높입니다. 기본값은 모델에 따라 다르므로 최대값 이하에 있다고 가정하기 전에 확인하십시오. [노력 수준 조정](/docs/ko/model-config#adjust-effort-level)에서 모델별 기본값과 `ultrathink` 바로가기를 참조하십시오.
* **컨텍스트 압력**: `/context`를 실행하여 윈도우가 얼마나 찼는지 확인합니다. 용량에 가까우면 자연스러운 지점에서 `/compact`를 실행하거나 `/clear`를 실행하여 새로 시작합니다. [컨텍스트 윈도우 탐색](/docs/ko/context-window)에서 자동 압축이 이전 턴에 어떻게 영향을 미치는지 확인하십시오.
* **오래된 지침**: 크거나 오래된 `CLAUDE.md` 파일과 MCP 도구 정의는 컨텍스트를 소비하고 응답을 조종할 수 있습니다. `/doctor` 점검은 크기가 큰 메모리 파일과 사용하지 않는 확장을 플래그하며, `/context`는 MCP 도구 토큰 사용량을 표시합니다. v2.1.205 이전에는 `/doctor`가 크기가 큰 메모리 파일과 서브에이전트 정의를 플래그하는 진단 화면을 열었습니다.

응답이 잘못되면 수정으로 회신하는 것보다 되감기가 일반적으로 더 잘 작동합니다. Esc를 두 번 누르거나 `/rewind`를 실행하여 잘못된 턴 이전으로 돌아간 다음 더 구체적인 내용으로 프롬프트를 다시 표현합니다. 스레드 내에서 수정하면 잘못된 시도가 컨텍스트에 남아 있어 나중의 답변을 고정할 수 있습니다. [체크포인팅](/docs/ko/checkpointing)을 참조하십시오.

위의 항목을 확인한 후에도 품질이 여전히 좋지 않으면 `/feedback`을 실행하고 예상한 것과 얻은 것을 설명합니다. 이 방식으로 제출된 피드백에는 대화 트랜스크립트가 포함되어 있으며, 이는 Anthropic이 실제 회귀를 진단하는 가장 빠른 방법입니다. 환경에서 `/feedback`을 사용할 수 없는 경우 [오류 보고](#report-an-error)를 참조하십시오.

Claude가 의심되는 프롬프트 주입에 대해 경고하거나 의심되는 주입으로 인해 요청을 거부하고, 경고가 명명하는 텍스트가 파일 또는 웹 콘텐츠가 아니라 Claude Code가 대화에 자동으로 추가하는 컨텍스트인 경우 `claude update`를 실행하고 다시 시도합니다. 업데이트 후 경고가 반복되면 플래그된 콘텐츠를 프롬프트에 다시 붙여넣는 대신 [보고](#report-an-error)합니다. v2.1.201 이전에는 Sonnet 5가 같은 방식으로 일부 요청을 거부했습니다.

<h2 id="report-an-error">
  오류 보고
</h2>

이 페이지에서 다루지 않는 구성 요소의 오류는 관련 가이드를 참조하십시오:

* MCP 서버 연결 또는 인증 실패: [MCP](/docs/ko/mcp)
* 훅 스크립트 실패 또는 도구 차단: [훅 디버깅](/docs/ko/hooks#debug-hooks)
* 설치 중 권한 거부 또는 파일 시스템 오류: [설치 및 로그인 문제 해결](/docs/ko/troubleshoot-install)

오류가 여기에 나열되지 않았거나 제안된 해결 방법이 도움이 되지 않는 경우:

* Claude Code 내에서 `/feedback`을 실행하여 기록 및 설명을 Anthropic에 전송하십시오. 이 명령은 미리 작성된 GitHub 이슈를 열 수 있는 옵션도 제공합니다. Anthropic에 전송하려면 [인증](/docs/ko/authentication)이 필요합니다. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 및 기타 타사 제공자에서 또는 Anthropic 자격 증명이 구성되지 않은 경우, `/feedback`은 대신 Anthropic 계정 담당자에게 보낼 수 있는 로컬 아카이브를 저장합니다.
* 셸에서 `claude doctor`를 실행하여 설치의 읽기 전용 진단을 수행하거나, Claude Code 내에서 `/doctor` 점검을 실행하여 설정 문제를 찾고 수정하십시오
* [status.claude.com](https://status.claude.com)에서 활성 인시던트를 확인하십시오
* GitHub의 [기존 이슈](https://github.com/anthropics/claude-code/issues)를 검색하십시오
