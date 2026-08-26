> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 오류 참조

> Claude Code 런타임 오류 메시지를 조회하고 각 오류의 의미와 해결 방법을 확인합니다.

이 페이지에는 Claude Code가 표시하는 런타임 오류와 각 오류에서 복구하는 방법, 그리고 오류 없이 응답이 이상해 보일 때 확인할 사항이 나열되어 있습니다. `command not found` 또는 설정 중 TLS 오류와 같은 설치 오류는 [설치 및 로그인 문제 해결](/docs/ko/troubleshoot-install)을 참조하십시오.

이러한 오류 및 복구 명령은 CLI, [데스크톱 앱](/docs/ko/desktop), [웹의 Claude Code](/docs/ko/claude-code-on-the-web)에 모두 적용됩니다. 세 가지 모두 동일한 Claude Code CLI를 래핑하기 때문입니다. 표면별 문제는 해당 표면의 페이지에 있는 문제 해결 섹션을 참조하십시오.

<Note>
  Claude Code는 모델 응답을 위해 Claude API를 호출하므로 대부분의 런타임 오류는 기본 API 오류 코드에 매핑됩니다. 이 페이지에서는 Claude Code 내에서 각 오류의 의미와 복구 방법을 다룹니다. 원본 HTTP 상태 코드 정의는 [Claude Platform 오류 참조](https://platform.claude.com/docs/en/api/errors)를 참조하십시오.
</Note>

<h2 id="find-your-error">
  오류 찾기
</h2>

터미널에 표시되는 메시지를 아래 섹션과 일치시킵니다.

| 메시지                                                                                                | 섹션                                                                                    |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------ |
| `API Error: 500 Internal server error`                                                             | [서버 오류](#api-error-500-internal-server-error)                                         |
| `API Error: Repeated 529 Overloaded errors`                                                        | [서버 오류](#api-error-repeated-529-overloaded-errors)                                    |
| `Request timed out`                                                                                | [서버 오류](#request-timed-out), 또는 메시지에 인터넷 연결이 언급된 경우 [네트워크](#unable-to-connect-to-api) |
| `Server error mid-response. The response above may be incomplete.`                                 | [서버 오류](#the-response-above-may-be-incomplete)                                        |
| `Connection closed mid-response` / `Response stalled mid-stream`                                   | [서버 오류](#the-response-above-may-be-incomplete)                                        |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`               | [서버 오류](#auto-mode-cannot-determine-the-safety-of-an-action)                          |
| `Auto mode could not evaluate this action and is blocking it for safety`                           | [서버 오류](#auto-mode-cannot-determine-the-safety-of-an-action)                          |
| `Auto mode classifier transcript exceeded context window`                                          | [서버 오류](#auto-mode-cannot-determine-the-safety-of-an-action)                          |
| `Agent terminated early due to an API error`                                                       | [서버 오류](#agent-terminated-early-due-to-an-api-error)                                  |
| `You've hit your session limit` / `You've hit your weekly limit`                                   | [사용 제한](#youve-hit-your-session-limit)                                                |
| `Usage credits required for 1M context`                                                            | [사용 제한](#usage-credits-required-for-1m-context)                                       |
| `Server is temporarily limiting requests`                                                          | [사용 제한](#server-is-temporarily-limiting-requests)                                     |
| `Request rejected (429)`                                                                           | [사용 제한](#request-rejected-429)                                                        |
| `Credit balance is too low`                                                                        | [사용 제한](#credit-balance-is-too-low)                                                   |
| `Not logged in · Please run /login`                                                                | [인증](#not-logged-in)                                                                  |
| `Could not resolve authentication method`                                                          | [인증](#could-not-resolve-authentication-method)                                        |
| `Invalid API key`                                                                                  | [인증](#invalid-api-key)                                                                |
| `Your apiKeyHelper script is failing`                                                              | [인증](#your-apikeyhelper-script-is-failing)                                            |
| `This organization has been disabled`                                                              | [인증](#this-organization-has-been-disabled)                                            |
| `Your organization has disabled API key authentication`                                            | [인증](#your-organization-has-disabled-api-key-authentication)                          |
| `Your organization has disabled Claude subscription access`                                        | [인증](#your-organization-has-disabled-claude-subscription-access)                      |
| `Routines are disabled by your organization's policy`                                              | [인증](#routines-are-disabled-by-your-organizations-policy)                             |
| `Remote Control is only available when using Claude via api.anthropic.com`                         | [인증](#remote-control-requires-the-anthropic-api)                                      |
| `OAuth token revoked` / `OAuth token has expired`                                                  | [인증](#oauth-token-revoked-or-expired)                                                 |
| `Login expired · Please run /login`                                                                | [인증](#login-expired)                                                                  |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                         | [인증](#login-expired)                                                                  |
| `does not meet scope requirement user:profile`                                                     | [인증](#oauth-scope-requirement)                                                        |
| `AWS credentials expired or invalid`                                                               | [인증](#aws-credentials-expired-or-invalid)                                             |
| `AWS authentication failed`                                                                        | [인증](#aws-authentication-failed)                                                      |
| `AWS default-chain credential resolve timed out`                                                   | [인증](#aws-default-chain-credential-resolve-timed-out)                                 |
| `Unable to connect to API`                                                                         | [네트워크](#unable-to-connect-to-api)                                                     |
| `Waiting for API response · will retry in`                                                         | [자동 재시도](#automatic-retries), 또는 지속되는 경우 [네트워크](#unable-to-connect-to-api)            |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"` | [네트워크](#bedrock-streaming-response-has-an-unexpected-content-type)                    |
| `SSL certificate verification failed`                                                              | [네트워크](#ssl-certificate-errors)                                                       |
| `SSL certificate error (...)` during login or startup                                              | [네트워크](#ssl-certificate-errors)                                                       |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                         | [네트워크](#host-not-allowed-in-a-cloud-session)                                          |
| `Couldn't reconnect to your Remote Control session`                                                | [네트워크](#couldnt-reconnect-to-your-remote-control-session)                             |
| `Prompt is too long`                                                                               | [요청 오류](#prompt-is-too-long)                                                          |
| `Error during compaction: Conversation too long`                                                   | [요청 오류](#error-during-compaction-conversation-too-long)                               |
| `Request too large`                                                                                | [요청 오류](#request-too-large)                                                           |
| `Image was too large`                                                                              | [요청 오류](#image-was-too-large)                                                         |
| `Unable to resize image`                                                                           | [요청 오류](#unable-to-resize-image)                                                      |
| `PDF too large` / `PDF is password protected`                                                      | [요청 오류](#pdf-errors)                                                                  |
| `Extra inputs are not permitted`                                                                   | [요청 오류](#extra-inputs-are-not-permitted)                                              |
| `There's an issue with the selected model`                                                         | [요청 오류](#theres-an-issue-with-the-selected-model)                                     |
| `Model ... is not a recognized model id`                                                           | [요청 오류](#model-is-not-a-recognized-model-id)                                          |
| `Claude Opus is not available with the Claude Pro plan`                                            | [요청 오류](#claude-opus-is-not-available-with-the-claude-pro-plan)                       |
| `Model ... is restricted by your organization's settings`                                          | [요청 오류](#model-is-restricted-by-your-organizations-settings)                          |
| `thinking.type.enabled is not supported for this model`                                            | [요청 오류](#thinking-type-enabled-is-not-supported-for-this-model)                       |
| `max_tokens must be greater than thinking.budget_tokens`                                           | [요청 오류](#thinking-budget-exceeds-output-limit)                                        |
| `API Error: 400 due to tool use concurrency issues`                                                | [요청 오류](#tool-use-or-thinking-block-mismatch)                                         |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`      | [요청 오류](#usage-policy-refusal)                                                        |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                  | [요청 오류](#safety-measures-flagged-a-cybersecurity-topic)                               |
| `Installation was killed before it could finish (exit code 137)`                                   | [설치 오류](#installation-was-killed-before-it-could-finish)                              |
| `The connection dropped while downloading the update`                                              | [설치 오류](#the-connection-dropped-while-downloading-the-update)                         |
| `Download timed out: exceeded the total deadline`                                                  | [설치 오류](#the-connection-dropped-while-downloading-the-update)                         |
| `--bg and --print conflict`                                                                        | [명령줄 오류](#command-line-errors)                                                        |
| `Error: --json-schema is not a valid JSON Schema`                                                  | [명령줄 오류](#command-line-errors)                                                        |
| `Could not import <server>: <reason>`                                                              | [명령줄 오류](#could-not-import-a-server-from-claude-desktop)                              |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                           | [명령줄 오류](#mcp-permission-prompt-tool-not-found)                                       |
| `Marketplace "<name>" is registered from an untrusted source`                                      | [플러그인 오류](#marketplace-is-registered-from-an-untrusted-source)                        |
| `references ${user_config.*} in a shell-form command`                                              | [플러그인 오류](#plugin-command-references-user-config)                                     |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                 | [플러그인 오류](#plugin-command-references-user-config)                                     |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                | [플러그인 오류](#plugin-command-references-user-config)                                     |
| `would be spawned with zero tools — refusing`                                                      | [도구 오류](#agent-would-be-spawned-with-zero-tools)                                      |
| `File is covered by a Read deny rule in your permission settings`                                  | [도구 오류](#file-is-covered-by-a-read-deny-rule)                                         |
| `Can't open MCP settings in a background session`                                                  | [백그라운드 세션 오류](#commands-refused-in-a-background-session)                              |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                        | [백그라운드 세션 오류](#claude_code_process_wrapper-launcher-errors)                           |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                | [구성 경고](#workspace-has-not-been-trusted)                                              |
| Responses seem lower quality than usual                                                            | [응답 품질](#responses-seem-lower-quality-than-usual)                                     |

<h2 id="automatic-retries">
  자동 재시도
</h2>

Claude Code는 오류를 표시하기 전에 일시적 오류를 재시도합니다. 서버 오류, 과부하 응답, 요청 시간 초과, 임시 429 스로틀, 끊어진 연결은 모두 지수 백오프를 사용하여 최대 10회 재시도됩니다. v2.1.198부터 이는 표시되는 출력이 없기 전에 응답 중간에 끊어지는 연결을 포함합니다. Claude Code는 동일한 백오프로 요청을 다시 발급하고 연결 오류로 중지하는 대신 턴이 계속됩니다. v2.1.199부터 계획의 할당량 헤더를 전달하지 않는 임시 429 스로틀도 claude.ai 구독으로 로그인할 때 재시도됩니다. 이전 버전은 API 키 및 엔터프라이즈 로그인에만 재시도했습니다.

일부 오류 클래스는 재시도할 수 없기 때문에 재시도되지 않습니다.

* v2.1.199부터 TLS 인증서 검증 실패(예: TLS 검사 프록시, 누락된 `NODE_EXTRA_CA_CERTS` 번들 또는 만료된 인증서)는 첫 번째 시도에서 실패하므로 전체 재시도 예산 후가 아닌 즉시 수정이 나타납니다. [SSL 인증서 오류](#ssl-certificate-errors)를 참조하십시오. 핸드셰이크 시간 초과와 같은 일시적 TLS 조건은 여전히 재시도됩니다.
* v2.1.199부터 Claude가 이미 표시되는 출력을 스트리밍한 후 도착하는 서버 오류는 부분 응답을 유지하고 [불완전한 응답 공지](#the-response-above-may-be-incomplete)를 추가합니다. 동일한 도구를 두 번 실행할 수 있으므로 재시도하지 않습니다. 이전 버전은 부분 출력을 버리고 턴을 오류로 보고했습니다.
* [Amazon Bedrock 스트리밍 응답에 예상치 못한 콘텐츠 유형](#bedrock-streaming-response-has-an-unexpected-content-type)은 첫 번째 시도에서 실패합니다. 게이트웨이 또는 프록시가 응답을 다시 작성하면 재시도도 동일한 방식으로 다시 작성하기 때문입니다. Claude Code v2.1.208 이상이 필요합니다.

재시도하는 동안 스피너는 오류 레이블 뒤에 `Retrying in Ns · attempt x/y` 카운트다운을 표시합니다. 레이블은 즉시 조치할 수 있는 오류의 첫 번째 시도에서 구체적인 이유를 나타냅니다. 네트워크가 다운되었거나 TLS 핸드셰이크가 실패했거나 속도 제한에 도달했습니다. 다른 오류의 경우 처음에는 `API error`로 읽습니다. v2.1.198부터 세 번째 시도의 구체적인 이유로 전환되거나 `CLAUDE_CODE_MAX_RETRIES`가 3개 미만을 허용할 때 최종 시도에서 전환됩니다. 이전 버전은 최종 시도에서만 전환됩니다.

v2.1.198부터 일반적인 스피너 팁은 재시도 중에 억제됩니다. 오류 이유가 드러나면 실패가 529 과부하인 경우 카운트다운 아래 줄도 서비스 상태를 확인할 위치를 나타냅니다. Anthropic API의 `status.claude.com` 또는 다른 구성의 메시지에 명시된 제공자 또는 게이트웨이 호스트입니다.

요청이 여전히 대기 중인 상태에서 응답 스트림에 20초 동안 데이터가 도착하지 않으면 스피너는 재시도가 시작되기 전에 `Waiting for API response · will retry in … · check your network`를 표시합니다. 요청이 아직 실패하지 않았습니다. 카운트다운은 Claude Code가 정지된 연결을 중단하고 재시도하는 지점까지 실행되므로 데이터가 재개되거나 재시도가 성공하면 배너가 자동으로 사라집니다. v2.1.185부터 임계값은 20초입니다. 이전 버전은 다른 표현으로 10초 후에 배너를 표시합니다. 모든 시도마다 다시 나타나면 [네트워크 문제](#unable-to-connect-to-api)로 취급하십시오.

이 페이지의 오류 중 하나를 보면 해당 재시도가 이미 소진되었습니다. 인증서 검증 실패와 같이 재시도되지 않는 클래스에 속하지 않는 한 말입니다. 다음 환경 변수로 동작을 조정할 수 있습니다.

| 변수                                           | 기본값     | 효과                                                                                                                                                                                                                                                 |
| :------------------------------------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/ko/env-vars)    | 10      | 재시도 횟수입니다. v2.1.186부터 15로 제한됩니다. v2.1.199부터 `CLAUDE_CODE_RETRY_WATCHDOG`이 기본값을 높이고 상한을 제거합니다. 스크립트에서 오류를 더 빨리 표시하려면 낮추십시오.                                                                                                                         |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/ko/env-vars) | 설정되지 않음 | CI 작업과 같은 무인 세션에서 `1`로 설정하여 `CLAUDE_CODE_MAX_RETRIES` 시도 후 실패하는 대신 `429` 및 `529` 용량 오류를 무한정 재시도합니다. v2.1.199부터 서버 오류, 시간 초과, 끊어진 연결과 같은 다른 일시적 오류의 기본 재시도 횟수를 300으로 높입니다. 대략 3시간의 백오프이며 명시적으로 해당 변수를 설정하면 `CLAUDE_CODE_MAX_RETRIES`의 상한 15를 제거합니다. |
| [`API_TIMEOUT_MS`](/docs/ko/env-vars)             | 600000  | 요청당 시간 초과(밀리초)입니다. 느린 네트워크 또는 프록시의 경우 높입니다.                                                                                                                                                                                                        |

<h2 id="server-errors">
  서버 오류
</h2>

이러한 오류는 사용자의 계정이나 요청이 아닌 추론 제공자로부터 발생합니다. Anthropic API의 경우 Anthropic 인프라를 의미합니다. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 사용자 정의 게이트웨이의 경우 해당 제공자의 인프라를 의미합니다.

<h3 id="api-error-500-internal-server-error">
  API 오류: 500 내부 서버 오류
</h3>

Claude Code는 모든 5xx 응답에 대해 상태 코드와 API의 오류 메시지를 표시합니다. 아래 예제는 Anthropic API의 500 응답을 보여줍니다:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

뒤따르는 문장은 서비스 상태를 확인할 위치를 나타내며 제공자에 따라 다릅니다. Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry 구성은 해당 제공자의 서비스 상태를 나타냅니다. 사용자 정의 `ANTHROPIC_BASE_URL`은 게이트웨이 호스트를 나타냅니다.

이는 API 내부의 예기치 않은 실패를 나타냅니다. 사용자의 프롬프트, 설정 또는 계정으로 인해 발생하지 않습니다.

**수행할 작업:**

* [status.claude.com](https://status.claude.com) 또는 메시지에 명시된 제공자 상태 페이지에서 활성 인시던트를 확인합니다
* 1분 정도 기다린 후 메시지를 다시 보냅니다. 원본 메시지는 여전히 대화에 남아 있으므로 긴 프롬프트의 경우 전체 내용을 붙여넣는 대신 `try again`을 입력할 수 있습니다.
* 게시된 인시던트가 없는데도 오류가 계속되면 `/feedback`을 실행하여 Anthropic이 요청 세부 정보로 조사할 수 있도록 합니다. 환경에서 `/feedback`을 사용할 수 없는 경우 [오류 보고](#report-an-error)를 참조하세요.

<h3 id="api-error-repeated-529-overloaded-errors">
  API 오류: 반복된 529 과부하 오류
</h3>

API가 모든 사용자에게 일시적으로 용량 부족 상태입니다. Claude Code는 이 메시지를 표시하기 전에 이미 여러 번 재시도했습니다:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

뒤따르는 문장은 500 오류와 동일한 방식으로 제공자에 따라 다릅니다.

529는 사용자의 사용 한도가 아니며 할당량에 포함되지 않습니다.

**수행할 작업:**

* [status.claude.com](https://status.claude.com) 또는 메시지에 명시된 제공자 상태 페이지에서 용량 공지를 확인합니다
* 몇 분 후에 다시 시도합니다
* `/model`을 실행하고 다른 모델로 전환하여 계속 작업합니다. 용량은 모델별로 추적되기 때문입니다. Claude Code는 한 모델이 특히 높은 부하를 받을 때 이를 수행하도록 사용자에게 알립니다. 예를 들어 `Opus is experiencing high load, please use /model to switch to Sonnet`과 같습니다.

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
* 시간 초과가 자주 발생하고 네트워크가 정상인 경우 아래의 [네트워크 및 연결 오류](#network-and-connection-errors)를 참조하세요

<h3 id="the-response-above-may-be-incomplete">
  위의 응답이 불완전할 수 있습니다
</h3>

스트리밍 응답이 Claude가 이미 표시 가능한 출력을 생성한 후 실패했습니다. 요청을 다시 보내면 동일한 도구 호출이 두 번 실행될 수 있으므로 Claude Code는 이미 스트리밍된 내용을 유지하고 대신 이 공지를 추가합니다. 표시되는 변형은 원인을 나타냅니다:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection closed mid-response. The response above may be incomplete.
API Error: Response stalled mid-stream. The response above may be incomplete.
```

* }`Server error mid-response`: 스트림 중간의 과부하 또는 5xx 서버 오류입니다. 이 변형은 Claude Code v2.1.199 이상이 필요합니다. 그 이전에는 부분 출력을 버리고 전체 턴을 오류로 보고했습니다.
* `Connection closed mid-response`: 연결이 끊어졌습니다.
* `Response stalled mid-stream`: 스트림이 데이터 전송을 중지했습니다.

**수행할 작업:**

* 스트리밍된 응답을 읽습니다. 아무것도 손실되지 않았지만 마지막 문장이나 도구 호출이 누락되었을 수 있습니다.
* `continue`로 회신하여 Claude가 중단된 위치에서 계속하도록 합니다
* 표시 가능한 출력이 없기 전에 동일한 오류가 나타나면 Claude Code는 완료하지 않고 요청을 재시도합니다. [자동 재시도](#automatic-retries)를 참조하세요.

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  자동 모드가 작업의 안전성을 결정할 수 없습니다
</h3>

[자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)가 작업을 분류하는 데 사용하는 모델이 결정을 내릴 수 없어서 자동 모드가 작업을 자동으로 승인하지 않았습니다. 표시되는 메시지는 분류자가 실패한 이유에 따라 다릅니다.

작업 디렉토리 내의 읽기, 검색 및 편집은 분류자를 건너뛰므로 이 모든 경우에 계속 작동합니다.

분류자 모델이 과부하 상태일 때:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait briefly and then try this action again.
```

**수행할 작업:**

* 몇 초 후에 재시도합니다. Claude는 동일한 메시지를 보고 일반적으로 자동으로 재시도합니다
* 재시도가 계속 실패하면 읽기 전용 작업을 계속하고 나중에 차단된 작업으로 돌아옵니다
* 이는 일시적이며 [자동 모드 적격성](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)과 무관합니다. 설정을 변경할 필요가 없습니다

분류자가 구문 분석할 수 없는 응답을 반환했을 때:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**수행할 작업:**

* 작업을 재시도합니다. 일반적으로 다음 시도에서 성공합니다
* `claude --debug`를 실행하고 작업을 반복하여 디버그 로그에서 기본 분류자 응답을 확인합니다

별도의 API 안전 검사가 이전 대화 내용으로 인해 분류자 요청을 차단했을 때:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

**수행할 작업:**

* 이는 사용자의 작업에 대한 결정이 아닙니다. 대화에 이미 있는 내용이 자동 모드가 분류자에게 대화를 보낼 때 API의 안전 필터를 트리거했습니다
* 재시도는 도움이 되지 않습니다. 동일한 대화 내용이 필터를 다시 트리거합니다
* 다른 [권한 모드](/docs/ko/permission-modes)로 전환하여 메시지가 표시될 때 작업을 승인하거나 트리거 내용이 없는 새로운 대화를 시작합니다

대화가 분류자의 컨텍스트 윈도우보다 커졌을 때:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

대화형 세션에서 자동 모드는 해당 작업에 대해 일반 권한 프롬프트로 폴백하므로 수동으로 승인하거나 거부할 수 있습니다. [비대화형 모드](/docs/ko/headless)에서는 트랜스크립트만 증가하고 재시도가 성공할 수 없기 때문에 실행이 중단됩니다.

**수행할 작업:**

* 표시되는 프롬프트에서 작업을 승인하거나 거부합니다
* `/compact`를 실행하여 대화 크기를 줄여서 후속 작업이 분류자 윈도우에 맞도록 합니다

<h3 id="agent-terminated-early-due-to-an-api-error">
  에이전트가 API 오류로 인해 조기에 종료되었습니다
</h3>

[서브에이전트](/docs/ko/sub-agents)의 API 요청이 사용 한도에 도달했거나 서버 오류에 대한 재시도가 소진되었기 때문에 터미널로 실패했으므로 서브에이전트가 작업을 완료하기 전에 중지되었습니다. 이 메시지는 Claude Code v2.1.199 이상이 필요합니다. 그 이전에는 API 오류 텍스트가 서브에이전트의 결과인 것처럼 Claude에 반환되었습니다.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**수행할 작업:**

* 콜론 뒤의 오류 세부 정보를 이 페이지의 자체 섹션(예: [사용 한도](#usage-limits) 또는 [서버 오류](#server-errors))과 일치시키고 해당 섹션의 단계를 따릅니다
* 기본 오류가 해결되면 Claude에게 작업을 재시도하거나 [서브에이전트를 재개](/docs/ko/sub-agents#resume-subagents)하도록 요청합니다

속도 제한, 과부하 또는 서버 오류가 이미 텍스트 출력을 생성한 포그라운드 서브에이전트를 중단할 때 Claude는 이 오류 대신 불완전으로 표시된 부분 출력을 받습니다. 유일한 출력이 도구 호출인 서브에이전트도 이 오류를 받습니다. v2.1.199에서는 대신 빈 부분 결과를 반환했습니다. [서브에이전트의 API 오류](/docs/ko/sub-agents#api-errors-in-subagents)를 참조하세요.

<h2 id="usage-limits">
  사용 한도
</h2>

이러한 오류는 계정 또는 플랜에 연결된 할당량에 도달했음을 의미합니다. 이는 모든 사용자에게 영향을 미치는 [서버 오류](#server-errors)와는 다릅니다.

<h3 id="youve-hit-your-session-limit">
  세션 한도에 도달했습니다
</h3>

구독 플랜에는 롤링 사용 허용량이 포함됩니다. 허용량이 소진되면 다음 메시지 중 하나가 표시됩니다:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
```

Claude Code는 메시지에 표시된 재설정 시간까지 추가 요청을 차단합니다. 세션 및 주간 한도는 모든 모델에서 공유되므로 모델을 전환해도 액세스가 복구되지 않습니다. Opus 한도는 Opus 요청에만 적용되므로 `/model`로 다른 모델로 전환하면 계속 작업할 수 있습니다.

사용량은 세션 및 주간 허용량에 동시에 계산됩니다. 대규모 워크플로우 팬아웃과 같은 단일 대량 활동 버스트는 세션 윈도우가 재설정되기 전에 주간 허용량을 소진할 수 있습니다.

**수행할 작업:**

* 오류에 표시된 재설정 시간까지 기다립니다
* Opus 한도의 경우 `/model`을 실행하고 다른 모델로 전환하여 계속 작업합니다
* `/usage`를 실행하여 플랜 한도 및 재설정 시간을 확인합니다
* `/usage-credits`를 실행하여 Pro 및 Max에서 추가 사용량을 구매하거나, Team 및 Enterprise에서 관리자에게 요청합니다. 이 요금이 청구되는 방식에 대해서는 [유료 플랜의 사용 크레딧](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)을 참조하세요.
* 더 높은 기본 한도를 위해 플랜을 업그레이드하려면 [claude.com/pricing](https://claude.com/pricing)을 참조하세요.

한도에 도달하기 전에 남은 허용량을 모니터링하려면 `rate_limits` 필드를 [사용자 정의 상태 줄](/docs/ko/statusline#rate-limit-usage)에 추가하거나, Desktop 앱에서 모델 선택기 옆의 [사용 현황 링](/docs/ko/desktop#check-usage)을 클릭합니다.

<h3 id="usage-credits-required-for-1m-context">
  1M 컨텍스트에 필요한 사용 크레딧
</h3>

선택한 모델은 1M 토큰 확장 컨텍스트 윈도우를 사용하며, 플랜에는 사용 크레딧을 통해서만 포함됩니다.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on, or /model to switch to standard context
```

이는 할당량 소진이 아니라 자격 확인입니다. 세션 및 주간 허용량에 용량이 남아 있어도 발생합니다. 1M 컨텍스트를 직접 포함하는 플랜과 사용 크레딧이 필요한 플랜에 대해서는 [확장 컨텍스트](/docs/ko/model-config#extended-context)를 참조하세요.

이 오류가 컨텍스트가 200K 토큰을 초과하여 대화 중에 나타나면 Claude Code는 자동으로 대화를 표준 컨텍스트 한도 아래로 압축하고 이후 세션을 해당 한도로 유지하므로 조치가 필요하지 않습니다. v2.1.172 이전 버전에서는 `/compact`를 포함한 모든 후속 요청에서 오류가 반복되었습니다. 해당 버전에서는 `/clear`를 실행하여 복구합니다. 아래 단계는 명시적으로 `[1m]` 모델을 선택한 경우에 적용됩니다.

**수행할 작업:**

* `/model`을 실행하고 `[1m]` 접미사가 없는 변형을 선택하여 표준 컨텍스트 윈도우로 폴백합니다
* `/usage-credits`를 실행하여 Pro 및 Max에서 1M 변형에 대한 종량제 청구를 켜거나, Team 및 Enterprise에서 관리자에게 요청합니다
* `/model` 후에도 오류가 지속되면 1M 모델 ID가 다른 곳에 설정되어 있을 수 있습니다. 우선순위 순서로 확인할 구성 위치에 대해서는 [선택한 모델에 문제가 있습니다](#theres-an-issue-with-the-selected-model)를 참조하세요.
* 모델 선택기에서 1M 변형을 완전히 제거하려면 [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/ko/env-vars)을 설정합니다.

<h3 id="server-is-temporarily-limiting-requests">
  서버가 일시적으로 요청을 제한 중입니다
</h3>

API가 플랜 할당량과 무관한 단기 스로틀을 적용했습니다.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code는 실제 한도 응답이 전달하는 통합 할당량 헤더의 부재로 이를 플랜 한도와 구분합니다. v2.1.199부터 이는 인증 방식에 관계없이 [자동으로 재시도](#automatic-retries)되며 백오프를 사용한 후 표시됩니다. 이전 버전에서는 claude.ai 구독으로 로그인한 세션이 첫 번째 발생 시 턴에 실패했습니다. API 키 및 Enterprise 로그인만 재시도했습니다.

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

뒤따르는 문장은 서비스 상태를 확인할 위치를 명시하며 공급자에 따라 다릅니다. Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry 구성은 Anthropic 상태 페이지 대신 해당 공급자의 서비스 상태를 명시합니다. 사용자 정의 `ANTHROPIC_BASE_URL`은 게이트웨이 호스트를 명시합니다.

**수행할 작업:**

* `/status`를 실행하고 활성 자격 증명이 예상한 것인지 확인합니다. 환경에 있는 잘못된 `ANTHROPIC_API_KEY`는 구독 대신 저가형 키를 통해 요청을 라우팅할 수 있습니다.
* 공급자 콘솔에서 활성 한도를 확인하고 필요한 경우 더 높은 계층을 요청합니다
* Anthropic API 키의 경우 계층이 작동하는 방식 및 워크스페이스별 상한을 설정하는 방법에 대해서는 [속도 제한 참조](https://platform.claude.com/docs/en/api/rate-limits)를 참조하세요
* 동시성 감소: [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/ko/env-vars)를 낮추고, 많은 병렬 서브에이전트 실행을 피하거나, 대량 스크립팅 실행을 위해 `/model`로 더 작은 모델로 전환합니다

<h3 id="credit-balance-is-too-low">
  크레딧 잔액이 너무 낮습니다
</h3>

Console 조직의 선불 크레딧이 소진되었습니다.

```text theme={null}
Credit balance is too low
```

**수행할 작업:**

* [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing)에서 크레딧을 추가하고, 잔액이 0에 도달하기 전에 자동 충전을 활성화하는 것을 고려합니다
* Pro, Max, Team 또는 Enterprise 플랜이 있는 경우 `/login`으로 구독 인증으로 전환합니다
* Console에서 워크스페이스별 지출 상한을 설정하여 단일 프로젝트가 조직 잔액을 소진하지 않도록 합니다. [비용 효과적으로 관리](/docs/ko/costs)를 참조하세요.

<h2 id="authentication-errors">
  인증 오류
</h2>

이러한 오류는 Claude Code가 API에 대한 신원을 증명할 수 없음을 의미합니다. 언제든지 `/status`를 실행하여 현재 활성화된 자격증명을 확인할 수 있습니다.

<h3 id="not-logged-in">
  로그인되지 않음
</h3>

이 세션에 유효한 자격증명이 없습니다.

```text theme={null}
Not logged in · Please run /login
```

**수행할 작업:**

* `/login`을 실행하여 Claude 구독 또는 Console 계정으로 인증합니다.
* 환경 변수로 인증하려고 했다면 `ANTHROPIC_API_KEY`가 `claude`를 실행한 셸에서 설정되고 내보내졌는지 확인합니다.
* CI 또는 자동화에서 대화형 로그인이 불가능한 경우, 시작 시 키를 가져오는 [`apiKeyHelper`](/docs/ko/settings#available-settings) 스크립트를 구성합니다.
* [인증 우선순위](/docs/ko/authentication#authentication-precedence)를 참조하여 여러 자격증명이 있을 때 Claude Code가 사용하는 자격증명을 이해합니다.

반복적으로 로그인하라는 메시지가 표시되면 시스템 시계 및 macOS Keychain 수정 사항에 대해 [로그인되지 않음 또는 토큰 만료됨](/docs/ko/troubleshoot-install#not-logged-in-or-token-expired)을 참조합니다.

<h3 id="could-not-resolve-authentication-method">
  인증 방법을 확인할 수 없음
</h3>

세션이 자격증명 없이 API 클라이언트에 도달했습니다. 이는 [백그라운드 세션](/docs/ko/agent-view), 클라우드 세션 및 첫 번째 요청 전에 대화형 로그인 확인이 실행되지 않는 Agent SDK 컨텍스트에서 나타납니다.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

v2.1.174 이전에는 유휴 사전 초기화된 워커에 할당된 백그라운드 또는 클라우드 세션이 유효한 자격증명이 구성되어 있어도 이런 방식으로 실패할 수 있었습니다. 업그레이드하여 복구합니다. 현재 버전에서 이 오류는 워커 프로세스에 사용 가능한 자격증명이 없음을 의미합니다.

**수행할 작업:**

* 백그라운드 또는 클라우드 세션에서 이것이 나타나고 자격증명이 이미 구성되어 있으면 v2.1.174 이상으로 업그레이드합니다.
* `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` 또는 클라우드 공급자 자격증명이 대화형 셸뿐만 아니라 워커를 실행하는 환경에서 설정되어 있는지 확인합니다.
* Agent SDK의 경우 [인증 설정](/docs/ko/agent-sdk/overview#get-started)을 참조합니다.
* 동일한 환경의 대화형 세션에서 `/status`를 실행하여 어떤 자격증명 소스가 확인되는지 확인합니다.

<h3 id="invalid-api-key">
  잘못된 API 키
</h3>

`ANTHROPIC_API_KEY` 환경 변수 또는 `apiKeyHelper` 스크립트가 API에서 거부한 키를 반환했습니다.

```text theme={null}
Invalid API key · Fix external API key
```

**수행할 작업:**

* 오타를 확인하고 [Console](https://platform.claude.com/settings/keys)에서 키가 취소되지 않았는지 확인합니다.
* 동일한 셸에서 `env | grep ANTHROPIC`을 실행합니다. direnv, dotenv 셸 플러그인 및 IDE 터미널과 같은 도구는 명시적으로 설정하지 않아도 프로젝트의 `.env` 파일에서 오래된 키를 로드할 수 있습니다.
* `ANTHROPIC_API_KEY`를 설정 해제하고 `/login`을 실행하여 대신 구독 인증을 사용합니다.
* 키가 [`apiKeyHelper`](/docs/ko/settings#available-settings) 스크립트에서 오는 경우 스크립트를 직접 실행하여 stdout에 유효한 키를 인쇄하는지 확인합니다.
* `/status`를 실행하여 Claude Code가 실제로 사용 중인 자격증명 소스를 확인합니다.

<h3 id="your-apikeyhelper-script-is-failing">
  apiKeyHelper 스크립트가 실패하고 있습니다
</h3>

[`apiKeyHelper`](/docs/ko/settings#available-settings) 설정에 구성된 명령이 오류로 종료되었거나, 시간 초과되었거나, stdout에 아무것도 인쇄하지 않았습니다. 스크립트에서 키가 없으면 요청이 플레이스홀더 자격증명으로 API에 도달하고 API가 `401`로 거부합니다.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Claude Code는 스크립트를 다시 실행하고 이 메시지를 표시하기 전에 요청을 최대 2회 더 재시도하므로 실패가 3번의 시도 내에 표시됩니다. v2.1.208 이전에는 Claude Code가 전체 [재시도 예산](#automatic-retries)을 플레이스홀더 자격증명으로 요청을 재전송하는 데 사용한 후 스크립트 실패 대신 일반 `401` 인증 오류를 보고했습니다.

`/login`을 실행해도 도움이 되지 않습니다. 설정이 있는 한 헬퍼의 출력이 저장된 로그인보다 [우선순위를 가집니다](/docs/ko/authentication#authentication-precedence).

**수행할 작업:**

* `apiKeyHelper`에 구성된 명령을 셸에서 직접 실행하여 실패를 재현합니다.
* 명령이 만료된 세션을 보고하면 자격증명 공급자로 다시 인증합니다(예: SSO 또는 비밀 저장소에 다시 로그인).
* 명령이 stdout에 키를 인쇄하고 코드 0으로 종료하도록 수정합니다. [apiKeyHelper로 자격증명 회전](/docs/ko/llm-gateway-connect#rotate-credentials-with-apikeyhelper)에서 작동하는 설정을 참조합니다.
* `/status`를 실행하여 `apiKeyHelper`가 활성 자격증명 소스인지 확인합니다. 명령이 실패할 때마다 종료 코드와 오류 출력이 터미널의 `Cloud authentication` 패널에 나타납니다.

<h3 id="this-organization-has-been-disabled">
  이 조직이 비활성화되었습니다
</h3>

비활성화된 Console 조직의 오래된 `ANTHROPIC_API_KEY`가 구독 로그인을 재정의하고 있습니다.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your other credentials
API Error: 400 ... This organization has been disabled.
```

환경 변수는 `/login`보다 우선순위가 높으므로 셸 프로필에서 내보내거나 `.env` 파일에서 로드된 키는 작동하는 Pro 또는 Max 구독이 있어도 사용됩니다. 비대화형 모드(`-p`)에서는 키가 있을 때 항상 사용됩니다.

**수행할 작업:**

* 현재 셸에서 `ANTHROPIC_API_KEY`를 설정 해제하고 셸 프로필에서 제거한 후 `claude`를 다시 실행합니다.
* 그 후 `/status`를 실행하여 활성 자격증명이 구독인지 확인합니다.
* 환경 변수가 설정되지 않았는데도 오류가 지속되면 비활성화된 조직이 `/login`에 연결된 것입니다. 지원팀에 문의하거나 다른 계정으로 로그인합니다.

<h3 id="your-organization-has-disabled-api-key-authentication">
  조직에서 API 키 인증을 비활성화했습니다
</h3>

}
이 메시지는 Claude Code v2.1.169 이상이 필요합니다. Console 조직의 관리자가 API 키 인증을 비활성화했으므로 API가 Claude Code가 보내는 키를 거부합니다. `·` 뒤의 복구 힌트는 키가 어디에서 왔는지에 따라 다릅니다:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

환경 변수와 `apiKeyHelper`는 `/login`보다 우선순위가 높으므로 둘 중 하나가 여전히 키를 제공하는 동안 `/login`만 실행해도 도움이 되지 않습니다. [인증 우선순위](/docs/ko/authentication#authentication-precedence)를 참조합니다.

**수행할 작업:**

* 메시지에 `ANTHROPIC_API_KEY`가 명시되어 있으면 현재 셸에서 설정 해제하고 셸 프로필 또는 `.env` 파일에서 제거한 후 `claude`를 다시 실행합니다.
* 메시지에 `apiKeyHelper`가 명시되어 있으면 `settings.json`에서 [`apiKeyHelper`](/docs/ko/settings#available-settings) 설정을 제거합니다.
* `/login`을 실행하여 claude.ai 계정으로 로그인합니다.
* 그 후 `/status`를 실행하여 활성 자격증명이 API 키가 아닌 구독인지 확인합니다.
* 자동화를 위해 API 키 인증이 필요한 경우 조직 관리자에게 Console에서 다시 활성화하도록 요청합니다.

<h3 id="your-organization-has-disabled-claude-subscription-access">
  조직에서 Claude 구독 액세스를 비활성화했습니다
</h3>

Claude 조직이 구독 로그인으로 Claude Code에 로그인하는 것을 허용하지 않습니다. 동일한 계정으로 `/login`을 다시 실행하면 동일한 오류가 반환됩니다.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

이는 서버 측 조직 설정이므로 로컬 설정, 환경 변수 또는 CLI 플래그에서 재정의할 수 없습니다.

Agent SDK 및 `-p` 비대화형 모드는 이를 `oauth_org_not_allowed` 오류 코드로 표시합니다.

**수행할 작업:**

* 관리자에게 조직에 대해 Claude Code 액세스를 활성화하도록 요청합니다.
* 구독 대신 Console API 키로 인증합니다. 설정은 [Claude Console 인증](/docs/ko/authentication#claude-console-authentication)을 참조합니다.
* 관리자이고 액세스를 활성화하는 옵션이 보이지 않으면 [Anthropic 지원팀](https://support.claude.com)에 문의합니다.

<h3 id="routines-are-disabled-by-your-organizations-policy">
  루틴이 조직의 정책에 의해 비활성화되었습니다
</h3>

Team 또는 Enterprise 조직의 Owner가 조직 수준에서 루틴을 비활성화했습니다. 오류는 `/schedule` 및 claude.ai/code의 [루틴](/docs/ko/routines) UI를 포함하여 루틴을 생성하거나 실행하려고 할 때 나타납니다.

```text theme={null}
Routines are disabled by your organization's policy.
```

이는 서버 측 설정이므로 로컬 설정, 환경 변수 또는 CLI 플래그에서 재정의할 수 없습니다.

**수행할 작업:**

* 조직의 Owner에게 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)에서 **루틴** 토글을 활성화하도록 요청합니다.
* 조직 수준의 루틴이 필요하지 않은 일회성 예약 작업의 경우 [예약된 작업](/docs/ko/scheduled-tasks)을 참조합니다.

<h3 id="remote-control-requires-the-anthropic-api">
  원격 제어에는 Anthropic API가 필요합니다
</h3>

세션이 Anthropic API와 직접 통신하지 않으므로 [원격 제어](/docs/ko/remote-control)가 쌍을 이룰 claude.ai 백엔드가 없습니다.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com.
```

이는 Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry에서 나타납니다. v2.1.196부터는 [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)이 `api.anthropic.com` 이외의 호스트(예: [LLM 게이트웨이](/docs/ko/llm-gateway) 또는 프록시)를 가리킬 때도 나타나며, claude.ai로 로그인한 경우에도 나타납니다.

**수행할 작업:**

* `ANTHROPIC_BASE_URL`을 설정 해제하고 세션을 다시 시작하거나 Anthropic API와 직접 통신하는 세션에서 원격 제어를 시작합니다.
* 이 및 다른 원격 제어 시작 메시지의 경우 [원격 제어 문제 해결](/docs/ko/remote-control#troubleshooting)을 참조합니다.

<h3 id="oauth-token-revoked-or-expired">
  OAuth 토큰이 취소되었거나 만료되었습니다
</h3>

저장된 로그인이 더 이상 유효하지 않습니다. 취소된 토큰은 모든 곳에서 로그아웃했거나 관리자가 액세스를 제거했음을 의미하고, 만료된 토큰은 자동 새로 고침이 세션 중에 실패했음을 의미합니다.

두 메시지 모두 Claude Code가 보낸 요청에 대해 API가 반환한 거부를 보고합니다. 저장된 로그인이 실패한 새로 고침 후 이미 지워진 경우 대신 [로그인 만료됨](#login-expired)을 봅니다.

```text theme={null}
OAuth token revoked · Please run /login
OAuth token has expired · Please run /login
API Error: 401 ... authentication_error
```

**수행할 작업:**

* `/login`을 실행하여 다시 로그인합니다.
* 재인증 후 동일한 세션 내에서 오류가 반환되면 먼저 `/logout`을 실행하여 저장된 토큰을 완전히 지운 후 `/login`을 실행합니다.
* 시작 간에 반복적인 로그인 프롬프트의 경우 [문제 해결](/docs/ko/troubleshoot-install#not-logged-in-or-token-expired)의 시스템 시계 및 macOS Keychain 확인을 참조합니다.
* `403 Forbidden` 및 OAuth 브라우저 문제를 포함한 다른 실패의 경우 [로그인 및 인증](/docs/ko/troubleshoot-install#login-and-authentication)을 참조합니다.

<h3 id="login-expired">
  로그인 만료됨
</h3>

Claude Code가 저장된 claude.ai 또는 Claude Console 로그인을 갱신하려고 했고 OAuth 서비스가 저장된 새로 고침 토큰을 거부했으므로 Claude Code가 저장된 자격증명을 지웠습니다. 그 후 각 요청은 API에 도달하기 전에 로컬에서 중지됩니다. 새 자격증명을 만들 수 있는 것은 `/login`뿐이기 때문입니다. v2.1.206 이전에는 Claude Code가 환경에 남아 있는 모든 자격증명으로 요청을 어쨌든 보냈고 모든 모델이 로그인하라는 프롬프트 대신 [선택한 모델에 문제가 있습니다](#theres-an-issue-with-the-selected-model) 또는 401로 실패했습니다.

```text theme={null}
Login expired · Please run /login
```

[비대화형 모드](/docs/ko/headless)(`-p`) 및 [Agent SDK](/docs/ko/agent-sdk/overview)에서 메시지는 다음과 같이 읽히며 구조화된 오류 코드는 `authentication_failed`입니다:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

이는 [OAuth 토큰이 취소되었거나 만료되었습니다](#oauth-token-revoked-or-expired)와 동일한 상태가 아닙니다. 이러한 메시지는 API가 반환한 401을 보고합니다. Claude Code 자체는 이미 갱신하지 못한 로그인에 대해 `Login expired`를 생성하므로 요청을 보내지 않습니다.

API 키, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/ko/env-vars) 또는 타사 공급자로 인증된 세션은 저장된 로그인을 사용하지 않으며 이 메시지를 절대 보지 않습니다.

**수행할 작업:**

* `/login`을 실행하여 다시 로그인합니다. 로그인하지 않고 재시도하면 모든 요청에서 동일한 메시지가 표시됩니다.
* 비대화형 모드에서 동일한 환경에서 `claude`를 실행하고 `/login`을 완료한 후 명령을 다시 실행합니다. 대화형으로 로그인할 수 없는 자동화의 경우 `ANTHROPIC_API_KEY`로 인증하거나 [`claude setup-token`으로 장기 토큰을 생성합니다](/docs/ko/authentication#generate-a-long-lived-token).
* 로그인이 계속 실패하면 [로그인 및 인증](/docs/ko/troubleshoot-install#login-and-authentication)을 참조합니다.

<h3 id="oauth-scope-requirement">
  OAuth 범위 요구사항
</h3>

저장된 토큰이 최신 기능에 필요한 권한 범위보다 앞서 있습니다. 이는 `/usage` 및 상태 줄 사용량 표시기에서 가장 자주 나타납니다:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**수행할 작업:**

* `/login`을 실행하여 현재 범위로 새 토큰을 가져옵니다. 먼저 로그아웃할 필요가 없습니다.

<h3 id="aws-credentials-expired-or-invalid">
  AWS 자격증명이 만료되었거나 유효하지 않습니다
</h3>

이 메시지는 Claude Code v2.1.198 이상이 필요하며 설정 파일에 [`awsAuthRefresh`](/docs/ko/amazon-bedrock#advanced-credential-configuration)가 설정되어 있을 때만 나타납니다. AWS 세션 토큰이 만료되었거나 거부되었으며, Claude Code가 이미 실행한 자동 새로 고침이 API가 수락하는 자격증명을 생성하지 못했습니다. [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws) 또는 [Mantle 엔드포인트](/docs/ko/amazon-bedrock#use-the-mantle-endpoint)에서 401이 나타나며, 이는 해당 공급자가 만료된 보안 토큰을 보고하는 방식입니다.

중간의 작업 힌트는 설정의 `awsAuthRefresh` 명령을 명시하므로 다릅니다. 안정적인 부분은 선행하는 `AWS credentials expired or invalid`입니다:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

`awsAuthRefresh`가 구성되지 않으면 동일한 401이 대신 일반 `Please run /login` 메시지를 표시하며, 이는 AWS 자격증명을 새로 고칠 수 없습니다.

**수행할 작업:**

* 메시지에 명시된 `awsAuthRefresh` 명령(예: `aws sso login --profile myprofile`)을 다른 터미널에서 실행하고 브라우저 로그인을 완료한 후 다시 시도합니다.
* 대화형 세션에서 `/login`을 실행하고 **3rd-party platform**을 선택한 후 **Using 3rd-party platforms** 아래에서 **Claude Platform on AWS · refresh credentials**를 선택하여 Claude Code를 다시 시작하지 않고 동일한 명령을 실행합니다. [AWS 자격증명 구성](/docs/ko/claude-platform-on-aws#1-configure-aws-credentials)을 참조합니다.
* 새로 고침 명령이 성공한 후에도 오류가 반복되면 동일한 셸 및 프로필에서 `aws sts get-caller-identity`를 사용하여 Claude Code 외부에서 ID가 유효한지 확인합니다.

<h3 id="aws-authentication-failed">
  AWS 인증 실패
</h3>

이 메시지는 Claude Code v2.1.198 이상이 필요하며 설정 파일에 [`awsAuthRefresh`](/docs/ko/amazon-bedrock#advanced-credential-configuration)가 설정되어 있을 때만 나타납니다. AWS 공급자가 403을 반환했거나 [Amazon Bedrock](/docs/ko/amazon-bedrock)이 401을 반환했습니다.

Claude Code는 어느 원인을 맞혔는지 알 수 없습니다. Amazon Bedrock은 만료된 보안 토큰을 403으로 보고하지만, 403은 또한 IAM 권한 누락 또는 계정에 대해 활성화되지 않은 모델과 같은 `AccessDeniedException`의 권한 거부를 보고하는 방식입니다.

Amazon Bedrock의 401은 [AWS 자격증명이 만료되었거나 유효하지 않습니다](#aws-credentials-expired-or-invalid) 아래가 아닌 여기에 도달합니다. Amazon Bedrock은 만료된 토큰을 401로 보고하지 않기 때문입니다. 해당 엔드포인트의 401은 일반적으로 회사 프록시와 같은 요청 경로의 다른 것에서 옵니다.

자격증명 새로 고침은 만료된 토큰을 수정하고 다른 원인을 수정할 수 없으므로 메시지는 둘 다 제공합니다:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

중간의 작업 힌트는 설정의 `awsAuthRefresh` 명령을 명시하므로 다릅니다. 안정적인 부분은 선행하는 `AWS authentication failed`입니다.

**수행할 작업:**

* 메시지에 명시된 `awsAuthRefresh` 명령 또는 `aws sso login`을 실행합니다(만료된 자격증명이 원인일 수 있음).
* 자격증명이 최신이면 [IAM 구성](/docs/ko/amazon-bedrock#iam-configuration)의 IAM 권한이 사용 중인 ID에 연결되어 있고 선택한 모델이 계정 및 지역에 대해 활성화되어 있는지 확인합니다.
* `aws sts get-caller-identity`를 실행하여 요청이 어떤 ID를 사용하는지 확인합니다. 오래된 `AWS_PROFILE` 또는 기본 프로필은 권한 불일치의 일반적인 원인입니다.

<h3 id="aws-default-chain-credential-resolve-timed-out">
  AWS 기본 체인 자격증명 확인 시간 초과
</h3>

AWS 기본 자격증명 공급자 체인이 60초 내에 자격증명을 생성하지 못했으므로 Claude Code가 확인을 중지하고 요청을 실패했습니다. 실패는 로컬 자격증명 확인입니다. 요청이 [Amazon Bedrock](/docs/ko/amazon-bedrock), [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws) 또는 [Mantle 엔드포인트](/docs/ko/amazon-bedrock#use-the-mantle-endpoint)에 도달하지 않았습니다. Claude Code는 이 오류가 표시되기 전에 [자격증명 캐시](/docs/ko/amazon-bedrock#credential-caching-and-resolution-timeout)를 지우고 반복 시도 전에 재시도하므로 체인이 반복 시도에서 정체되었습니다.

```text theme={null}
API Error: AWS default-chain credential resolve timed out
```

일반적인 원인은 AWS 프로필의 `credential_process` 명령이 받을 수 없는 입력을 기다리는 것이고, 인스턴스 메타데이터 서비스(IMDS)가 체인의 프로브에 응답하지 않는 컨테이너 또는 VM입니다. v2.1.207 이전에는 정체된 체인이 요청을 무한정 기다리게 했으며 이 메시지로 실패하지 않았습니다.

**수행할 작업:**

* 동일한 셸에서 동일한 `AWS_PROFILE`로 `aws sts get-caller-identity`를 실행합니다. 또한 중단되면 프로필을 수정합니다. 대화형으로 프롬프트하는 `credential_process` 명령이 일반적인 원인입니다.
* Claude Code를 시작하기 전에 로그인 단계를 완료합니다(예: `aws sso login --profile myprofile`). 그러면 체인이 브라우저 흐름을 기다리지 않고 로컬 SSO 캐시에서 확인됩니다.
* 체인이 `aws-vault`와 같은 래퍼를 통한 MFA가 있는 SSO와 같이 60초 이상이 필요한 대화형 로그인을 실행하면 [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/ko/env-vars)로 밀리초 단위로 제한을 높입니다.

<h2 id="network-and-connection-errors">
  네트워크 및 연결 오류
</h2>

이러한 오류는 Claude Code의 네트워크 요청이 목적지에 도달하지 못했거나, Claude Code와 API 사이의 무언가가 응답을 변경했음을 의미합니다. 일반적으로 로컬 네트워크, 프록시 또는 방화벽, 또는 클라우드 환경의 네트워크 정책에서 발생합니다.

<h3 id="unable-to-connect-to-api">
  API에 연결할 수 없음
</h3>

API에 대한 TCP 연결이 실패했거나 완료되지 않았습니다.

```text theme={null}
Unable to connect to API. Check your internet connection
Unable to connect to API (ECONNREFUSED)
Unable to connect to API (ECONNRESET)
Unable to connect to API (ETIMEDOUT)
fetch failed
Request timed out. Check your internet connection and proxy settings
```

일반적인 원인으로는 인터넷 접근 불가, `api.anthropic.com`을 차단하는 VPN, 또는 구성되지 않은 필수 회사 프록시가 있습니다.

**수행할 작업:**

* 동일한 셸에서 `curl -I https://api.anthropic.com`을 실행하여 API 호스트에 도달할 수 있는지 확인합니다. Windows PowerShell에서는 `curl.exe -I https://api.anthropic.com`을 사용하여 기본 제공 `Invoke-WebRequest` 별칭이 사용되지 않도록 합니다.
* 회사 프록시 뒤에 있는 경우 Claude Code를 시작하기 전에 `HTTPS_PROXY`를 설정하고 [네트워크 구성](/docs/ko/network-config)을 참조합니다.
* LLM 게이트웨이 또는 릴레이를 통해 라우팅하는 경우 [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)을 해당 주소로 설정합니다. 설정은 [Claude Code를 LLM 게이트웨이에 연결](/docs/ko/llm-gateway-connect)을 참조합니다.
* 방화벽이 [네트워크 액세스 요구 사항](/docs/ko/network-config#network-access-requirements)에 나열된 호스트를 허용하는지 확인합니다.
* 간헐적 오류는 [자동으로 재시도](#automatic-retries)되며, 지속적인 오류는 로컬 네트워크 문제를 나타냅니다.

`curl`이 성공하지만 Claude Code가 여전히 실패하는 경우, 원인은 일반적으로 네트워크 자체가 아니라 런타임과 네트워크 사이의 무언가입니다:

* Linux 및 WSL에서 `/etc/resolv.conf`에서 도달할 수 없는 네임서버를 확인합니다. 특히 WSL은 호스트에서 손상된 리졸버를 상속할 수 있습니다.
* macOS에서 연결이 끊어지거나 제거된 VPN 클라이언트는 터널 인터페이스 또는 라우팅 규칙을 남길 수 있습니다. `ifconfig`에서 오래된 `utun` 인터페이스를 확인하고 시스템 설정에서 VPN의 네트워크 확장을 제거합니다.
* Docker Desktop 및 유사한 컨테이너 런타임은 아웃바운드 트래픽을 가로챌 수 있습니다. 이를 종료하고 다시 시도하여 이를 배제합니다.

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock 스트리밍 응답에 예상치 못한 content-type이 있음
</h3>

Claude Code와 [Amazon Bedrock](/docs/ko/amazon-bedrock) 사이의 게이트웨이 또는 프록시가 스트리밍 응답 본문 또는 해당 `Content-Type` 헤더를 변환하고 있습니다. Amazon Bedrock은 응답을 `application/vnd.amazon.eventstream`으로 스트리밍하며, Claude Code는 읽을 수 없는 본문을 디코딩하는 대신 다른 content-type을 보고하는 성공적인 스트리밍 응답을 거부합니다. 요청은 재시도되지 않습니다.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

v2.1.208 이전에는 동일한 잘못된 구성이 전체 응답이 버퍼링된 후 `API Error: Truncated event message received`로 나타났습니다.

**수행할 작업:**

* 게이트웨이를 구성하여 `InvokeModelWithResponseStream` 응답 본문 및 해당 `Content-Type` 헤더를 수정되지 않은 상태로 전달합니다. 스트림을 서버 전송 이벤트로 다시 내보내는 중개자가 일반적인 원인입니다.
* 게이트웨이가 헤더만 다시 쓰고 바이너리 본문을 그대로 전달하는 경우 게이트웨이가 수정될 때까지 확인을 건너뛰도록 [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/ko/env-vars)을 설정합니다. [게이트웨이 또는 프록시 뒤의 스트리밍 오류](/docs/ko/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy)를 참조합니다.

<h3 id="ssl-certificate-errors">
  SSL 인증서 오류
</h3>

네트워크의 프록시 또는 보안 어플라이언스가 자체 인증서로 TLS 트래픽을 가로채고 있으며, Claude Code가 이를 신뢰하지 않습니다.

```text theme={null}
Unable to connect to API: SSL certificate verification failed. Check your proxy or corporate SSL certificates
Unable to connect to API: Self-signed certificate detected
```

v2.1.199부터 인증서 검증 실패는 재시도되지 않으므로 이 오류는 전체 [재시도 예산](#automatic-retries) 후가 아니라 첫 번째 시도에 나타납니다. 이전 버전은 표시하기 전에 몇 분 동안 재시도했습니다. 핸드셰이크 타임아웃과 같은 일시적 TLS 조건은 여전히 재시도됩니다.

`/login` 및 시작 연결 확인 중에 동일한 오류가 OpenSSL 코드 및 인라인 수정과 함께 보고됩니다:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

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

대상의 실제 인증서와 일치하지 않는 TLS 인증서도 표시될 수 있습니다. 클라우드 환경은 아웃바운드 트래픽을 네트워크 정책을 적용하는 프록시를 통해 라우팅하므로, 일치하지 않는 인증서는 대상이 아니라 프록시가 연결을 종료했음을 의미합니다.

이는 클라이언트 측 네트워크 문제가 아닙니다. 클라우드 세션 및 [루틴](/docs/ko/routines)은 아웃바운드 트래픽이 환경의 허용 목록으로 필터링되는 샌드박스 환경 내에서 실행됩니다. **기본** 환경은 **신뢰할 수 있는** 액세스를 사용하며, 이는 패키지 레지스트리, 클라우드 공급자 API, 컨테이너 레지스트리 및 일반적인 개발 도메인의 [기본 허용 목록](/docs/ko/claude-code-on-the-web#default-allowed-domains)을 허용하지만 다른 모든 것을 차단합니다.

**수행할 작업:**

* 루틴을 편집하기 위해 열거나 클라우드 세션을 시작합니다. **기본**과 같은 환경 이름을 표시하는 클라우드 아이콘을 선택하여 선택기를 엽니다. 환경 위에 마우스를 올리고 설정 아이콘을 클릭합니다.
* **클라우드 환경 업데이트** 대화 상자에서 **네트워크 액세스**를 **신뢰할 수 있는**에서 **사용자 정의**로 변경한 다음 차단된 도메인을 **허용된 도메인**에 추가합니다. 한 줄에 하나의 도메인을 입력합니다. **일반적인 패키지 관리자의 기본 목록도 포함**을 확인하여 [기본 허용 목록](/docs/ko/claude-code-on-the-web#default-allowed-domains)을 사용자 정의 도메인과 함께 유지합니다. 제한 없는 액세스를 원하는 경우 대신 **전체**를 선택합니다.
* **변경 사항 저장**을 클릭합니다. 다음 실행은 업데이트된 허용 목록을 사용합니다.

액세스 수준 및 기본 허용 목록은 [네트워크 액세스](/docs/ko/claude-code-on-the-web#network-access)를 참조합니다. 로컬 CLI 세션은 이 정책의 영향을 받지 않습니다.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Remote Control 세션에 다시 연결할 수 없음
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

`claude --resume` 또는 `claude --continue`로 재개하면 해당 대화에 기록된 [Remote Control](/docs/ko/remote-control) 세션에 다시 연결됩니다. 이 메시지는 네트워크 중단 또는 서버 오류와 같이 일시적일 수 있는 이유로 재연결이 실패했음을 의미하므로 Claude Code는 원격 세션이 여전히 존재하는지 확인할 수 없습니다. 로컬 세션은 Remote Control 없이 계속 실행됩니다.

**수행할 작업:**

* `/remote-control`을 실행하여 연결을 재시도합니다.
* `--resume` 없이 Claude Code를 시작하여 새 Remote Control 세션을 만듭니다.
* 다른 Remote Control 시작 메시지는 [Remote Control 문제 해결](/docs/ko/remote-control#troubleshooting)을 참조합니다.

서버가 이전 세션이 더 이상 존재하지 않음을 확인하면 이 메시지가 표시되지 않습니다. Claude Code는 이 경우 새 세션을 만듭니다. v2.1.200 이전에는 모든 재연결 실패가 새 Remote Control 세션을 만들었으며, 이는 claude.ai/code의 세션 목록에 추가 세션을 남겼습니다.

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

**수행할 작업:**

* `/compact`를 실행하여 이전 턴을 요약하고 공간을 확보하거나, `/clear`를 실행하여 새로 시작합니다.
* `/context`를 실행하여 윈도우를 소비하는 항목의 분석을 확인합니다: 시스템 프롬프트, 도구, 메모리 파일 및 메시지
* `/mcp disable <name>`으로 사용하지 않는 MCP 서버를 비활성화하여 컨텍스트에서 도구 정의를 제거합니다.
* 큰 `CLAUDE.md` 메모리 파일을 정리하거나, 지침을 [경로 범위 규칙](/docs/ko/memory#path-specific-rules)으로 이동하여 관련이 있을 때만 로드합니다.
* 서브에이전트는 부모 세션의 모든 MCP 도구 정의를 상속하므로, 첫 번째 턴 전에 컨텍스트 윈도우를 채울 수 있습니다. 서브에이전트를 생성하기 전에 사용하지 않는 MCP 서버를 비활성화합니다.
* 자동 압축은 기본적으로 활성화되어 있으며 일반적으로 이 오류를 방지합니다. [`DISABLE_AUTO_COMPACT`](/docs/ko/env-vars)를 설정한 경우, 다시 활성화하거나 윈도우가 가득 차기 전에 `/compact`를 수동으로 실행합니다.

[컨텍스트 윈도우 탐색](/docs/ko/context-window)을 참조하여 컨텍스트가 어떻게 채워지는지 대화형으로 확인합니다.

<h3 id="error-during-compaction-conversation-too-long">
  압축 중 오류: 대화가 너무 깁니다
</h3>

`/compact` 자체가 실패했습니다. 생성되는 요약을 보유할 충분한 여유 컨텍스트가 없기 때문입니다.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

이는 자동 압축이 트리거되는 순간 윈도우가 이미 가득 차 있거나, `Prompt is too long`을 본 후 `/compact`를 실행할 때 발생할 수 있습니다.

**수행할 작업:**

* Esc를 두 번 눌러 메시지 목록을 열고 여러 턴을 뒤로 이동합니다. 이렇게 하면 가장 최근 메시지가 컨텍스트에서 제거됩니다. 그런 다음 `/compact`를 다시 실행합니다.
* 뒤로 이동해도 충분한 공간이 확보되지 않으면 `/clear`를 실행하여 새 세션을 시작합니다. 이전 대화는 보존되며 `/resume`으로 다시 열 수 있습니다.

<h3 id="request-too-large">
  요청이 너무 큽니다
</h3>

원본 요청 본문이 토큰화 전에 API의 바이트 제한을 초과했습니다. 일반적으로 큰 붙여넣은 파일이나 첨부 파일 때문입니다.

```text theme={null}
Request too large (max 30 MB). Double press esc to go back and remove or shrink the attached content.
```

이는 [컨텍스트 윈도우 제한](#prompt-is-too-long)과 별개인 HTTP 요청의 크기 제한입니다.

**수행할 작업:**

* Esc를 두 번 눌러 뒤로 이동하고 과도한 크기의 콘텐츠를 추가한 턴을 지나갑니다.
* 콘텐츠를 붙여넣는 대신 경로로 큰 파일을 참조하여 Claude가 청크 단위로 읽을 수 있도록 합니다.
* 이미지의 경우 아래의 [이미지가 너무 컸습니다](#image-was-too-large)를 참조합니다.

<h3 id="image-was-too-large">
  이미지가 너무 컸습니다
</h3>

붙여넣거나 첨부한 이미지가 API의 크기 또는 치수 제한을 초과합니다.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code는 처리할 수 없는 이미지를 텍스트 자리 표시자로 바꾸고 다시 시도하므로 후속 메시지가 성공합니다. 2.1.142 이전 버전에서는 붙여넣은 이미지가 대화에 남아 있을 수 있으며 후속 메시지마다 동일한 오류를 반복할 수 있습니다. 이러한 버전에서 복구하려면 Esc를 두 번 눌러 이미지가 추가된 턴을 지나갑니다.

**수행할 작업:**

* 붙여넣기 전에 이미지 크기를 조정합니다. API는 단일 이미지의 경우 가장 긴 가장자리에서 최대 8000픽셀, 많은 이미지가 컨텍스트에 있을 때 2000픽셀까지의 이미지를 허용합니다.
* 전체 화면 대신 관련 영역의 더 타이트한 스크린샷을 찍습니다.

<h3 id="unable-to-resize-image">
  이미지 크기 조정 불가
</h3>

Claude Code가 API로 보내기 전에 첨부된 이미지를 축소할 수 없었습니다.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
```

Claude Code는 일반적으로 큰 이미지를 자동으로 크기 조정합니다. 이러한 오류는 네이티브 이미지 프로세서가 로드되지 않았거나 오류를 반환했으므로 이미지를 API 제한에 맞게 크기 조정할 수 없음을 의미합니다.

**수행할 작업:**

* 메시지에서 이미지를 변환하도록 요청하면 PNG, JPEG, GIF 또는 WebP로 변환하고 다시 첨부합니다. Claude Code는 이미지 프로세서 없이 이러한 형식의 치수를 확인할 수 있습니다.
* 메시지에서 치수 또는 크기 제한을 보고하면 첨부하기 전에 이미지를 해당 제한 아래로 크기 조정하거나 다시 압축합니다.

<h3 id="pdf-errors">
  PDF 오류
</h3>

첨부한 PDF를 처리할 수 없었습니다.

```text theme={null}
PDF too large (max 100 pages, 32 MB). Try splitting it or extracting text first.
PDF is password protected. Try removing protection or extracting text first.
The PDF file was not valid. Try converting to a different format first.
```

**수행할 작업:**

* 크기가 큰 PDF의 경우 전체 파일을 첨부하는 대신 Read 도구로 Claude에게 페이지 범위를 읽도록 요청하거나, `pdftotext`와 같은 도구로 텍스트를 추출하고 경로로 출력 파일을 참조합니다.
* 보호되거나 유효하지 않은 PDF의 경우 암호를 제거하거나 원본 응용 프로그램에서 파일을 다시 내보낸 후 다시 시도합니다.

<h3 id="extra-inputs-are-not-permitted">
  추가 입력은 허용되지 않습니다
</h3>

Claude Code와 API 사이의 프록시 또는 LLM 게이트웨이가 `anthropic-beta` 요청 헤더를 제거했으므로 API가 이에 따라 달라지는 필드를 거부했습니다.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Extra inputs are not permitted ... tools.0.custom.input_examples
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code는 `context_management`, `effort` 및 도구 `input_examples`와 같은 베타 전용 필드를 이를 활성화하는 `anthropic-beta` 헤더와 함께 보냅니다. 게이트웨이가 본문을 전달하지만 헤더를 제거하면 API는 인식하지 못하는 필드를 봅니다.

**수행할 작업:**

* `anthropic-beta` 헤더를 전달하도록 게이트웨이를 구성합니다. 게이트웨이가 전달해야 하는 항목은 [기능 통과](/docs/ko/llm-gateway-protocol#feature-pass-through)를 참조합니다.
* 대체 방법으로, 시작하기 전에 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/ko/env-vars)을 설정합니다. 이렇게 하면 베타 헤더가 필요한 기능이 비활성화되므로 헤더를 전달할 수 없는 게이트웨이를 통해 요청이 성공합니다.

<h3 id="theres-an-issue-with-the-selected-model">
  선택한 모델에 문제가 있습니다
</h3>

구성된 모델 이름이 인식되지 않았거나 계정에 이에 대한 액세스 권한이 없습니다. v2.1.160 기준으로 여기에 대화형 형식으로 표시된 후행 힌트는 표면에 따라 다릅니다.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**수행할 작업:**

* **대화형 CLI**: `/model`을 실행하여 계정에서 사용 가능한 모델 중에서 선택합니다.
* **비대화형 모드(`-p`)**: `--model`을 유효한 별칭 또는 ID와 함께 전달하거나 [`ANTHROPIC_MODEL`](/docs/ko/env-vars)을 설정합니다. 오류 텍스트는 이 표면에서 `Run --model`을 표시합니다.
* **Agent SDK**: 모델이 프로그래밍 방식으로 설정되므로 오류 텍스트는 힌트를 생략합니다. TypeScript에서 [`Options`의 `model`](/docs/ko/agent-sdk/typescript#options)을 설정하거나 Python에서 [`ClaudeAgentOptions(model=...)`](/docs/ko/agent-sdk/python#claudeagentoptions)을 설정하고, 구조화된 `model_not_found` 오류를 처리하여 자신의 재시도 또는 모델 선택기를 표시합니다.
* 전체 버전이 지정된 ID 대신 `sonnet` 또는 `opus`와 같은 별칭을 사용합니다. 별칭은 유지 관리되는 기본값으로 확인되므로 오래되지 않습니다. [모델 구성](/docs/ko/model-config)을 참조합니다.
* 잘못된 모델이 CLI에서 계속 반환되면 어딘가에 오래된 ID가 설정되어 있습니다. [우선 순위 순서](/docs/ko/model-config#setting-your-model)로 확인합니다: `--model` 플래그, `ANTHROPIC_MODEL` 환경 변수, 그런 다음 `.claude/settings.local.json`의 `model` 필드, 프로젝트의 `.claude/settings.json` 및 `~/.claude/settings.json`. 오래된 값을 제거하면 Claude Code가 계정 기본값으로 폴백됩니다.
* Claude Code는 만료된 claude.ai 로그인을 [로그인 만료됨](#login-expired)으로 보고하며, 이 오류로는 보고하지 않습니다. v2.1.206 이전에는 더 이상 새로 고칠 수 없는 만료된 로그인이 이 오류로 모든 모델에 실패했습니다. 이전 버전에서 이것을 보면 `/login`을 실행합니다.
* Google Cloud의 Agent Platform 배포의 경우 [Google Cloud의 Agent Platform 문제 해결](/docs/ko/google-vertex-ai#troubleshooting)을 참조합니다.

<h3 id="model-is-not-a-recognized-model-id">
  모델이 인식된 모델 ID가 아닙니다
</h3>

모델 스위치에 전달한 모델 문자열이 모델 별칭, 이 Claude Code 버전이 알고 있는 모델 ID 또는 `claude-`로 시작하는 ID가 아닙니다. 일반적인 원인은 ID의 오타, `Sonnet 5`와 같은 표시 이름(ID `claude-sonnet-5` 필요) 또는 최신 Claude Code 버전만 인식하는 별칭입니다. Claude Code는 스위치를 즉시 거부합니다. v2.1.200 이전에는 Claude Code가 문자열을 저장하고 [선택한 모델에 문제가 있습니다](#theres-an-issue-with-the-selected-model)에서 다음 요청에 실패했습니다.

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

후행 힌트는 가장 가까운 일치하는 별칭 또는 모델 ID의 이름을 지정합니다. 충분히 가까운 것이 없으면 `Run /model to see available models.`로 읽습니다.

Claude Code는 API 요청이 이루어지기 전에 스위치가 요청되는 순간 로컬에서 이 오류를 생성합니다. [Agent SDK](/docs/ko/agent-sdk/typescript) `setModel()` 메서드를 통해 또는 Claude Code CLI를 실행하는 [Desktop app](/docs/ko/desktop)과 같은 앱에 의해 모델이 설정될 때 적용됩니다.

**수행할 작업:**

* 인수 없이 `/model`을 실행하여 선택기를 열고 계정에서 사용 가능한 모델 중에서 선택한 다음 거기에 표시된 별칭 또는 ID를 전달합니다.
* 최신 Claude Code 버전이 지원하는 별칭을 사용한 경우 `claude update`를 실행합니다. `claude-`로 시작하는 전체 ID는 모델이 Claude Code 버전보다 최신이어도 이 확인을 통과하므로 업그레이드가 필요하지 않습니다.
* v2.1.200 이전에 저장된 모델은 이 확인으로 복구되지 않습니다. 오래된 값이 계속 반환되면 [선택한 모델에 문제가 있습니다](#theres-an-issue-with-the-selected-model)에 나열된 위치에서 제거합니다.
* 확인은 Anthropic API에서만 실행됩니다. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/ko/claude-platform-on-aws) 및 [LLM 게이트웨이](/docs/ko/llm-gateway) 또는 사용자 정의 `ANTHROPIC_BASE_URL` 뒤에서 공급자 또는 게이트웨이가 모델 이름을 정의하므로 Claude Code는 모든 문자열을 허용하고 통과합니다.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus는 Claude Pro 플랜에서 사용할 수 없습니다
</h3>

활성 구독 플랜에 선택한 모델이 포함되지 않습니다.

```text theme={null}
Claude Opus is not available with the Claude Pro plan · Select a different model in /model
```

**수행할 작업:**

* `/model`을 실행하고 플랜에 포함된 모델을 선택합니다.
* 최근에 플랜을 업그레이드했는데도 이것이 표시되면 `/logout`을 실행한 다음 `/login`을 실행합니다. 저장된 토큰은 로그인 시점의 플랜을 반영하므로 웹에서 업그레이드해도 기존 세션에서 다시 인증할 때까지 적용되지 않습니다.
* 각 플랜에 포함된 모델은 [claude.com/pricing](https://claude.com/pricing)을 참조합니다.

<h3 id="model-is-restricted-by-your-organizations-settings">
  모델이 조직의 설정으로 제한됩니다
</h3>

조직 관리자가 claude.ai 관리 콘솔에서 이 모델을 비활성화했거나, 관리되는 설정의 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 허용 목록으로 제외되었습니다. 제한된 모델이 `--model`, `ANTHROPIC_MODEL` 또는 `model` 설정으로 설정된 경우 Claude Code는 허용된 모델을 대체하고 계속합니다. 제한된 모델에 대해 `/model <name>`을 입력하면 `Run /model to choose a different model.`로 거부되고 세션은 현재 모델을 유지합니다.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Claude Code는 모델 패밀리 별칭(예: `opus`, `sonnet`, `haiku` 또는 `fable` 중 하나)을 최신 버전에 대한 요청이 아닌 해당 패밀리에 대한 요청으로 취급합니다. Anthropic API 및 [Claude Platform on AWS](/docs/ko/claude-platform-on-aws)에서 제한된 패밀리 별칭은 조직 및 `availableModels` 허용 목록이 허용하는 패밀리의 최신 버전으로 확인되며, 대체 알림은 해당 버전의 이름을 지정합니다. Claude Code는 패밀리의 모든 버전이 제한될 때만 `/model <alias>`를 거부합니다. v2.1.205 이전에는 패밀리 별칭이 같은 패밀리의 이전 버전이 허용되었을 때도 최신 버전만을 기반으로 대체되거나 거부되었습니다.

**수행할 작업:**

* `/model`을 실행하여 조직이 허용하는 모델 중에서 선택합니다. 제한된 모델은 선택기에서 숨겨집니다.
* 제한된 모델이 `--model`, `ANTHROPIC_MODEL` 또는 설정 파일의 `model` 필드에 설정된 경우 해당 값을 제거하거나 업데이트하여 각 시작 시 알림이 반복되지 않도록 합니다.
* 제한된 모델에 대한 액세스가 필요한 경우 조직 관리자에게 활성화를 요청합니다. [조직 모델 제한](/docs/ko/model-config#organization-model-restrictions)을 참조합니다.

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled는 이 모델에서 지원되지 않습니다
</h3>

Claude Code 버전이 Sonnet 5, Opus 4.8 또는 Opus 4.7의 최소값보다 오래되었습니다. CLI가 모델이 더 이상 허용하지 않는 사고 구성을 보냈습니다.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**수행할 작업:**

* `claude update`를 실행하고 Claude Code를 다시 시작합니다. Opus 4.7은 v2.1.111 이상이 필요합니다. Opus 4.8은 v2.1.154 이상이 필요합니다. Sonnet 5는 v2.1.197 이상이 필요합니다.
* 업그레이드할 수 없으면 `/model`을 실행하고 대신 Opus 4.6 또는 Sonnet 4.6을 선택합니다.
* [Agent SDK](/docs/ko/agent-sdk/overview)에서 이것을 맞으면 SDK 패키지를 대신 업그레이드합니다. Opus 4.8은 TypeScript SDK v0.3.154 이상 및 Python SDK v0.2.88 이상이 필요합니다. Sonnet 5는 TypeScript SDK v0.3.197 이상이 필요합니다.

<h3 id="thinking-budget-exceeds-output-limit">
  사고 예산이 출력 제한을 초과합니다
</h3>

구성된 확장 사고 예산이 최대 응답 길이를 초과하므로 실제 답변을 위한 공간이 남지 않습니다.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code는 Anthropic API에서 이러한 값을 자동으로 조정합니다. 일반적으로 [`MAX_THINKING_TOKENS`](/docs/ko/env-vars)가 공급자의 출력 제한보다 높게 설정되었거나 계획 모드가 사고 예산을 높일 때 Amazon Bedrock 또는 Google Cloud의 Agent Platform에서 이 오류가 표시됩니다.

**수행할 작업:**

* `MAX_THINKING_TOKENS`를 낮추거나 [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/ko/env-vars)를 사고 예산 위로 올립니다.
* [확장 사고](/docs/ko/model-config#extended-thinking)를 참조하여 예산이 출력 길이와 어떻게 상호 작용하는지 확인합니다.

<h3 id="tool-use-or-thinking-block-mismatch">
  도구 사용 또는 사고 블록 불일치
</h3>

대화 기록이 API에 일관성 없는 상태로 도달했습니다. 일반적으로 도구 호출이 중단되거나 턴이 스트림 중간에 편집된 후입니다.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

세 가지 변형 모두 동일한 의미입니다: 기록의 `tool_use`, `tool_result` 및 `thinking` 블록의 시퀀스가 더 이상 API가 예상하는 것과 일치하지 않습니다.

**수행할 작업:**

* Opus 4.7 또는 Opus 4.8을 사용하는 경우 먼저 `claude update`를 실행합니다. v2.1.156 이전 버전은 정상적인 도구 사용 중에 이 오류를 트리거할 수 있으며 `/rewind`는 이를 지우지 않습니다.
* `/rewind`를 실행하거나 Esc를 두 번 눌러 손상된 턴 전의 체크포인트로 뒤로 이동하고 거기서 계속합니다. [체크포인팅](/docs/ko/checkpointing)을 참조하여 체크포인트가 어떻게 생성되고 복원되는지 확인합니다.

<h3 id="usage-policy-refusal">
  사용 정책 거부
</h3>

API가 대화의 콘텐츠가 [사용 정책](https://www.anthropic.com/legal/aup) 확인을 트리거했기 때문에 응답을 거부했습니다. 메시지에는 거부가 잘못되었다고 생각하는 경우 지원팀에 인용할 수 있는 요청 ID가 포함됩니다.

```text theme={null}
API Error: Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.
```

확인은 최신 프롬프트뿐만 아니라 전체 대화를 평가하므로 동일한 세션에서 새 메시지를 보내면 일반적으로 동일한 거부를 다시 트리거합니다. `--continue` 또는 `--resume`으로 세션을 종료하고 다시 열 때도 마찬가지입니다. 디스크의 기록에 여전히 트리거 콘텐츠가 포함되어 있기 때문입니다. [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 및 [Microsoft Foundry](/docs/ko/microsoft-foundry)에서 이 메시지는 모델의 안전 조치가 사이버 보안 주제로 플래그한 요청도 포함합니다. [안전 조치가 사이버 보안 주제를 플래그했습니다](#safety-measures-flagged-a-cybersecurity-topic)를 참조합니다.

**수행할 작업:**

* Esc를 두 번 누르거나 `/rewind`를 실행하여 거부를 트리거한 턴 전의 체크포인트로 뒤로 이동한 다음 다시 표현하거나 다른 접근 방식을 취합니다. [체크포인팅](/docs/ko/checkpointing)을 참조합니다.
* 어느 턴이 원인인지 식별할 수 없으면 `/clear`를 실행하여 동일한 프로젝트에서 새 대화를 시작합니다. 이전 대화는 디스크에 보존되며 `/resume`에서 사용 가능합니다.
* [비대화형 모드](/docs/ko/headless)(`-p`)에서는 되감기를 사용할 수 없으므로 `--continue` 없이 새 세션에서 다시 표현된 프롬프트로 다시 시도합니다. 정책 확인은 모델에 따라 다르므로 `--model`로 다른 모델로 전환하면 일부 경우에 거부를 해결할 수도 있습니다.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  안전 조치가 사이버 보안 주제를 플래그했습니다
</h3>

모델의 안전 조치가 대화의 콘텐츠를 사이버 보안 주제로 플래그했습니다. 메시지는 요청을 플래그한 모델의 이름을 지정합니다:

```text theme={null}
API Error: Opus 4.8 has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude.

If you were not engaging in a cybersecurity topic, please send feedback via /feedback.
```

메시지는 정당한 사이버 보안 작업에 대한 액세스를 부여하는 [사이버 검증 프로그램](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)에 연결됩니다. 보안 조치 자체는 서버 측이며 v2.1.203보다 앞서 있습니다. 이 릴리스는 메시지의 표현과 연결되는 페이지만 변경했습니다.

표시되는 내용은 공급자 및 모드에 따라 다릅니다:

* [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 및 [Microsoft Foundry](/docs/ko/microsoft-foundry)에서 사이버 보안 플래그는 대신 [사용 정책 거부](#usage-policy-refusal) 메시지를 생성합니다.
* [비대화형 모드](/docs/ko/headless)는 `/feedback` 문장을 생략합니다.

v2.1.203 이전에는 메시지가 `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` 다음에 면제 양식 링크를 읽었습니다.

**수행할 작업:**

* 작업에 이 콘텐츠가 필요한 경우 [사이버 검증 프로그램](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)을 통해 액세스를 신청합니다.
* 요청이 사이버 보안 주제가 아닌 경우 `/feedback`을 실행하여 거짓 양성을 보고합니다.
* 동일한 세션에서 계속 작업하려면 Esc를 두 번 누르거나 `/rewind`를 실행하여 플래그를 트리거한 턴 전의 체크포인트로 뒤로 이동한 다음 다른 접근 방식을 취합니다. [체크포인팅](/docs/ko/checkpointing)을 참조합니다.

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

다른 치명적인 신호의 경우, 그리고 macOS의 종료 코드 137의 경우, 스크립트는 `Installation was killed before it could finish (exit code <N>)`을 출력하며 실제 종료 코드를 포함하고 메모리 부족 설명을 생략합니다. 메시지는 macOS 및 Linux가 사용하는 설치 스크립트에서 나오며, WSL 내부의 설치도 포함합니다. 네이티브 Windows 설치 스크립트는 절대 이를 출력하지 않습니다. v2.1.200 이전에는 스크립트가 셸의 단순한 `Killed` 줄로만 종료되었습니다.

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

연결은 유지되지만 10분 이내에 완료되지 않는 다운로드는 `Download timed out: exceeded the total deadline` 대신 실패합니다. Claude Code는 시간 초과된 다운로드를 재시도하지 않습니다. 왜냐하면 기한 내에 완료하기에 너무 느린 연결은 즉시 재시도에서도 완료되지 않기 때문입니다. 아래 단계는 두 메시지 모두에 적용됩니다. v2.1.205 이전에는 동일한 10분 기한이 HTTP 클라이언트의 일반적인 `timeout of 600000ms exceeded`로 보고되었습니다.

일반적인 원인은 전송이 완료되기 전에 긴 전송을 닫는 프록시 또는 게이트웨이입니다. Claude Code 바이너리는 큰 다운로드이므로, 일반 API 트래픽에는 영향을 주지 않는 프록시 연결 제한이 여전히 이를 중단할 수 있습니다.

**수행할 작업:**

* `claude update`를 다시 실행합니다. 그 외에 정상적인 네트워크에서는 다음 실행에서 다운로드가 일반적으로 성공합니다. 시간 초과 메시지의 경우 더 빠르거나 덜 제한된 네트워크에서 다시 실행합니다.
* 네트워크에 프록시가 필요한 경우 설치 프로그램 또는 `claude update`를 실행하기 전에 `HTTPS_PROXY`를 설정합니다. [네트워크 연결 확인](/docs/ko/troubleshoot-install#check-network-connectivity)을 참조하십시오.
* 회사 프록시가 계속 전송을 닫는 경우 네트워크 팀에 `downloads.claude.ai`에서 전체 다운로드를 허용하도록 요청합니다. [네트워크 액세스 요구 사항](/docs/ko/network-config#network-access-requirements)을 참조하십시오.
* 설치 진단을 위해 셸에서 `claude doctor`를 실행합니다

<h2 id="command-line-errors">
  명령줄 오류
</h2>

이러한 오류는 `claude` 명령줄 및 해당 하위 명령에서 발생합니다. Claude Code는 프롬프트를 실행하거나 API 요청을 보내기 전에 이를 출력합니다.

<h3 id="conflict-between-bg-and-print">
  \--bg와 --print 간의 충돌
</h3>

이 메시지는 Claude Code v2.1.198 이상이 필요합니다. 동일한 `claude` 호출에서 `--bg`를 `-p` 또는 `--print`와 결합했습니다. `--bg`는 나중에 `claude agents`로 연결할 수 있는 [백그라운드 세션](/docs/ko/agent-view#from-your-shell)을 시작하는 반면, `--print`는 [비대화형](/docs/ko/headless)으로 실행되며 `claude agents`가 연결하는 대화형 세션을 시작하지 않습니다. v2.1.198 이전에는 이 조합이 연결할 수 없는 백그라운드 작업을 자동으로 생성했습니다.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**수행할 작업:**

* `-p` 또는 `--print`를 제거합니다. `--bg`는 프롬프트를 위치 인수로 사용하므로 `claude --bg "<task>"`가 완전한 명령입니다. [셸에서 새 에이전트 디스패치](/docs/ko/agent-view#from-your-shell)를 참조하세요.
* 프롬프트를 비대화형으로 실행하고 백그라운드 세션을 생성하는 대신 결과를 출력하려면 `--bg`를 제거하고 `claude -p "<task>"`를 실행합니다.

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  \--json-schema 값이 유효한 JSON Schema가 아닙니다
</h3>

[`--json-schema`](/docs/ko/cli-reference#cli-flags)에 전달한 스키마가 [비대화형 모드](/docs/ko/headless#get-structured-output)에서 JSON Schema 컴파일에 실패했으므로 `claude`는 프롬프트를 실행하는 대신 종료 코드 1로 종료됩니다. v2.1.205 이전에는 유효하지 않은 스키마가 오류 없이 구조화되지 않은 출력을 생성했으며, `format` 키워드를 사용한 모든 스키마는 유효하지 않은 것으로 처리되었습니다.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

두 번째 콜론 뒤의 텍스트는 검증자의 진단이며 실패한 키워드 또는 위치를 나타냅니다. `"format": "email"`과 같은 `format` 키워드를 사용하는 스키마는 유효합니다. Claude Code는 `format`을 주석으로 허용하며 이를 적용하지 않습니다.

Claude Code는 스키마 컴파일 전에 두 가지 검사를 실행합니다. 구문 분석할 수 없는 JSON 값은 `Error: --json-schema is not valid JSON`으로 거부하고, 객체가 아닌 유효한 JSON은 `Error: --json-schema must be a JSON object`로 거부합니다.

**수행할 작업:**

* 진단이 나타내는 스키마 부분을 수정한 후 명령을 다시 실행합니다.
* 진단이 `schema too large`인 경우 스키마의 중첩 및 `$ref` 재사용을 줄입니다.
* [구조화된 출력 가져오기](/docs/ko/headless#get-structured-output)에서 작동하는 스키마 및 명령을 참조하세요.

<h3 id="could-not-import-a-server-from-claude-desktop">
  Claude Desktop에서 서버를 가져올 수 없습니다
</h3>

Claude Code는 `claude mcp add-from-claude-desktop`에서 선택한 서버 중 하나를 추가할 수 없습니다. 명령은 여전히 다른 선택된 서버를 가져오고 추가할 수 없는 각 서버마다 한 줄을 출력합니다. v2.1.205 이전에는 실패한 첫 번째 서버가 가져오기를 중지했으며 선택된 서버가 추가되지 않았습니다.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

서버 이름 뒤의 텍스트가 이유입니다. 가장 일반적인 것은 이름 검사입니다. Claude Desktop은 서버 이름에 공백 및 마침표와 같은 문자를 허용하지만 `claude mcp`는 문자, 숫자, 하이픈 및 밑줄로만 제한합니다. 다른 이유로는 검증에 실패한 서버 구성과 조직의 [MCP 정책](/docs/ko/managed-mcp)에 의해 차단된 서버가 있습니다.

**수행할 작업:**

* `claude_desktop_config.json`에서 서버 이름을 문자, 숫자, 하이픈 및 밑줄만 사용하도록 변경한 후 `claude mcp add-from-claude-desktop`을 다시 실행합니다.
* 유효한 이름으로 `claude mcp add` 또는 `claude mcp add-json`을 사용하여 해당 서버를 직접 추가합니다. [Claude Desktop에서 MCP 서버 가져오기](/docs/ko/mcp#import-mcp-servers-from-claude-desktop)를 참조하세요.

<h3 id="mcp-permission-prompt-tool-not-found">
  MCP 권한 프롬프트 도구를 찾을 수 없습니다
</h3>

[`--permission-prompt-tool`](/docs/ko/cli-reference#cli-flags)에 전달한 도구는 실행이 처음으로 권한 결정이 필요할 때 연결된 MCP 도구 중에 없었습니다. 이는 서버가 연결되지 않았거나 연결된 서버가 해당 이름의 도구를 노출하지 않기 때문입니다. Claude Code는 여전히 프롬프트를 보냅니다. [비대화형](/docs/ko/headless) 실행은 승인이 필요한 첫 번째 도구 호출에서 이 오류로 종료되고 종료 코드 1로 종료되므로 요청이 이루어졌음에도 불구하고 답변을 생성하지 않습니다. 첫 번째 프롬프트 전에 Claude Code는 [`MCP_TIMEOUT`](/docs/ko/env-vars)으로 설정된 서버당 연결 타임아웃 30초까지 해당 서버가 연결될 때까지 기다립니다. v2.1.206 이전에는 시작 시 서버가 연결을 완료할 때까지 기다리지 않았으므로 느리게 시작되지만 정상인 서버도 이 오류를 생성했습니다.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

`Available MCP tools:` 뒤의 목록은 대기가 끝났을 때 연결된 MCP 도구의 이름을 나타냅니다.

**수행할 작업:**

* 서버가 시작되고 연결 상태를 유지하는지 확인합니다. 동일한 디렉터리에서 `claude mcp list`를 실행하고 서버가 연결됨으로 나열되어 있는지 확인합니다.
* 도구 이름이 서버가 노출하는 `mcp__<server>__<tool>` 이름과 일치하는지 확인합니다.
* 서버를 시작하는 데 30초 이상이 필요한 경우 [`MCP_TIMEOUT`](/docs/ko/env-vars)을 높입니다.

<h2 id="plugin-errors">
  플러그인 오류
</h2>

이러한 오류는 [플러그인](/docs/ko/plugins) 및 [마켓플레이스](/docs/ko/plugin-marketplaces) 구성에서 발생합니다. 이 페이지의 메시지 중 하나를 생성하지 않는 플러그인 문제(예: 로드되지 않는 마켓플레이스 URL 또는 설치되지만 나타나지 않는 플러그인)의 경우 [플러그인 문제 해결](/docs/ko/discover-plugins#troubleshooting)을 참조하십시오.

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  마켓플레이스가 신뢰할 수 없는 소스에서 등록됨
</h3>

마켓플레이스가 [공식 Anthropic 마켓플레이스용으로 예약된](/docs/ko/plugin-marketplaces#marketplace-schema) 이름으로 등록되어 있지만, 등록된 소스가 `anthropics` GitHub 저장소가 아닙니다. Claude Code는 마켓플레이스를 로드하거나 새로 고칠 때마다 예약된 이름을 다시 확인하므로, 마켓플레이스와 여기서 설치된 플러그인이 로드되지 않습니다. v2.1.205 이전에는 마켓플레이스가 추가될 때만 이름이 확인되었으므로, 이름이 예약되기 전에 등록된 항목은 계속 로드되었습니다.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

**해야 할 일:**

* `claude plugin marketplace remove <name>`을 실행한 다음 공식 `github.com/anthropics` 저장소에서 마켓플레이스를 다시 추가합니다
* 이름이 예약되기 전에 해당 이름을 사용한 타사 마켓플레이스를 게시한 경우, 이름을 바꾸고 사용자에게 소스에서 다시 추가하도록 요청합니다
* [마켓플레이스 스키마](/docs/ko/plugin-marketplaces#marketplace-schema)에서 예약된 이름 목록을 참조하십시오

<h3 id="plugin-command-references-user-config">
  플러그인 명령이 셸 명령에서 user\_config를 참조함
</h3>

플러그인 훅, [모니터](/docs/ko/plugins-reference#monitors) 또는 MCP [`headersHelper`](/docs/ko/mcp#use-dynamic-headers-for-custom-authentication) 명령이 `${user_config.KEY}` [플러그인 옵션](/docs/ko/plugins-reference#user-configuration)을 참조하고, 대체된 문자열이 셸에 전달될 것입니다. `$(...)`, 백틱 또는 `;`을 포함하는 구성된 값은 여기서 코드로 실행될 수 있으므로, Claude Code는 값을 대체하는 대신 구성 요소 시작을 거부합니다. 확인은 명령 템플릿에서 실행되므로, 아직 값이 구성되지 않았을 때도 오류가 나타납니다. v2.1.207 이전에는 값이 셸 명령으로 대체되었습니다.

표현은 옵션을 참조한 표면에 따라 다릅니다. 셸 형식 훅은 다음을 보고합니다:

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

**해야 할 일:**

* 훅의 경우, `args` 배열을 추가하여 [exec 형식](/docs/ko/hooks#exec-form-and-shell-form)으로 실행되도록 합니다. 여기서 각 `${user_config.KEY}`는 그 사이에 셸이 없는 하나의 인수가 됩니다. 또는 참조를 제거하고 스크립트 내에서 `$CLAUDE_PLUGIN_OPTION_<KEY>` 환경 변수를 읽습니다
* 모니터의 경우, 참조를 제거하고 모니터 스크립트가 구성 파일에서 값을 읽도록 합니다
* `headersHelper`의 경우, `${user_config.KEY}`를 셸 구문 분석이 되지 않는 서버의 `headers` 필드로 이동하거나, 헬퍼 스크립트 내에서 값을 읽습니다

<h2 id="tool-errors">
  도구 오류
</h2>

이러한 오류는 Claude의 기본 제공 도구가 입력을 거부할 때 발생합니다. Claude는 대부분의 도구 오류를 자동으로 수정합니다. 아래의 두 가지 오류는 사용자가 제어하는 서브에이전트 정의 또는 권한 규칙에서 비롯되므로 사용자의 변경이 필요합니다.

<h3 id="agent-would-be-spawned-with-zero-tools">
  에이전트가 도구 없이 생성됨
</h3>

[서브에이전트의 `tools` 목록](/docs/ko/sub-agents#supported-frontmatter-fields)의 항목이 도구로 확인되지 않아 Claude Code가 작동할 수 없는 서브에이전트를 시작하는 대신 서브에이전트 시작을 거부합니다. 메시지는 항목을 확인되지 않은 이유별로 그룹화합니다: 인식되지 않은 도구, 서브에이전트에서 사용할 수 없는 도구, 또는 현재 세션의 도구와 일치하지 않는 인식된 도구입니다. `tools` 필드를 생략하면 이 거부가 트리거되지 않습니다. `mcp__github__*`와 같은 MCP 서버 패턴은 예외가 아닙니다: 해당 서버에서 연결된 도구가 없으면 패턴이 일치하지 않은 그룹에 있는 패턴으로 시작이 거부됩니다. v2.1.208 이전에는 서브에이전트가 도구 없이 시작되어 빈 결과 또는 혼란스러운 결과를 반환했습니다.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**수행할 작업:**

* 오류가 명시한 각 항목을 [서브에이전트에서 사용 가능한 도구](/docs/ko/sub-agents#available-tools)와 비교하여 수정합니다.
* 연결되지 않은 서버의 MCP 도구와 같이 세션에 없는 도구의 항목을 제거합니다.
* 서브에이전트에 부모가 가진 모든 도구를 제공하려면 도구를 나열하는 대신 `tools` 필드를 삭제합니다.

<h3 id="file-is-covered-by-a-read-deny-rule">
  파일이 Read 거부 규칙으로 보호됨
</h3>

Edit 도구가 [`Read` 거부 규칙](/docs/ko/permissions#read-and-edit)과 일치하는 경로에서 호출되었습니다. 여기에는 해당 경로에서 새 파일을 만드는 것도 포함됩니다. 편집은 Claude가 다시 읽을 수 있어야 하는 콘텐츠를 다시 작성하므로 파일 액세스 전에 호출이 거부됩니다. 규칙은 Edit 도구만 차단합니다: Write와 NotebookEdit은 `Read` 거부 규칙의 영향을 받지 않습니다. v2.1.208 이전에는 `Edit` 거부 규칙만 편집을 차단했으며 `Read` 거부 규칙 단독으로는 차단하지 않았습니다.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

**수행할 작업:**

* Claude가 파일을 편집할 수 있어야 하는 경우 `/permissions`의 `Read` 거부 규칙을 제거하거나 좁히거나 [설정](/docs/ko/settings#permission-settings)에서 제거합니다.
* 파일이 그대로 유지되어야 하는 경우 규칙을 유지하고 Write와 NotebookEdit 도구도 차단되도록 동일한 경로에 대한 `Edit` 거부 규칙을 추가합니다.

<h2 id="background-session-errors">
  백그라운드 세션 오류
</h2>

[백그라운드 세션](/docs/ko/agent-view)은 자체 대화형 터미널 없이 실행되므로 터미널이 필요한 명령은 다르게 작동합니다. 이러한 메시지는 백그라운드 세션의 기록, 에이전트 뷰 또는 연결 후에 나타납니다.

<h3 id="commands-refused-in-a-background-session">
  백그라운드 세션에서 거부된 명령
</h3>

대화형 대화 상자를 여는 명령은 백그라운드 세션에서 거부되며, 해당 위치에서 작동하는 양식의 이름을 지정하거나 일반 터미널에서 명령을 실행하도록 지시하는 메시지가 표시됩니다. `/install-github-app`, `/mcp` 설정 목록 및 MCP 서버 메뉴의 인증 작업은 모두 이러한 방식으로 거부됩니다. v2.1.208 이전에는 백그라운드 세션 내에서 대화 상자를 열었습니다.
v2.1.208에서만 `/model` 선택기도 백그라운드 세션에서 거부되었으며, `/upgrade`는 브라우저를 열지 않고 업그레이드 URL을 인쇄했습니다.

표현은 거부된 명령의 이름을 지정합니다. `/mcp` 설정 목록은 다음을 보고합니다:

```text theme={null}
Can't open MCP settings in a background session — use `/mcp enable|disable|reconnect <server>` to steer, or run /mcp from an interactive terminal to authenticate.
```

**수행할 작업:**

* `/mcp reconnect <server>`, `/mcp enable` 또는 `/mcp disable`과 같이 메시지에서 지정한 양식을 사용합니다
* 로그인 및 인증 흐름의 경우 터미널의 일반 `claude` 세션에서 명령을 실행합니다

<h3 id="claude_code_process_wrapper-launcher-errors">
  CLAUDE\_CODE\_PROCESS\_WRAPPER 런처 오류
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/ko/corporate-launcher)가 설정되어 있고 해당 값을 사용할 수 없으므로 Claude Code는 런처 없이 실행하지 않고 영향을 받는 프로세스를 시작하기를 거부합니다. 구성 문제는 변수 이름으로 시작하고 이유를 명시하는 메시지로 보고됩니다. 예를 들어:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

시작되었지만 Claude Code로 자신을 대체하지 않고 종료되는 런처는 시작하려던 세션을 실패하게 하며, 에이전트 뷰의 세션 행은 런처가 `must exec, not daemonize`를 수행해야 함을 보고하고 런처가 인쇄한 모든 항목을 따릅니다. 런처로 인해 시작할 수 없거나 백그라운드 서비스에 도달할 수 없는 세션은 런처 문제를 `Couldn't reach the background service (...)`의 이유로 보고합니다.

**수행할 작업:**

* 변수를 `exec "$@"`를 호출하여 끝나는 실행 파일의 절대 경로로 설정합니다. 전체 계약은 [런처 계약](/docs/ko/corporate-launcher#the-launcher-contract)을 참조하세요
* `/status`를 확인합니다. 이는 Self-exec 항목에서 해결된 시작 명령을 표시하고 실행 중인 백그라운드 서비스가 일치하지 않을 때 경고하거나, 셸에서 `claude daemon status`를 실행합니다
* [설정](/docs/ko/corporate-launcher#set-up-the-launcher)의 `env` 블록에서 값을 수정한 후 `claude daemon stop --any`로 백그라운드 서비스를 다시 시작하여 다음 디스패치가 래핑된 서비스를 시작하도록 합니다

<h2 id="configuration-warnings">
  구성 경고
</h2>

Claude Code는 대화에서 오류를 표시하는 대신 시작 시 이러한 메시지를 stderr에 작성합니다. 읽었지만 적용하지 않은 구성을 보고합니다.

<h3 id="workspace-has-not-been-trusted">
  작업 공간이 신뢰되지 않음
</h3>

Claude Code는 프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json`에서 `permissions.allow` 규칙 또는 `permissions.additionalDirectories` 항목을 찾았지만 [프로젝트 설정의 allow 규칙은 작업 공간 신뢰가 필요](/docs/ko/permissions#project-allow-rules-and-workspace-trust)하기 때문에 적용하지 않았습니다. 메시지의 개수, 설정 이름 및 파일 이름은 구성에 따라 다릅니다. `deny` 및 `ask` 규칙은 영향을 받지 않습니다.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**수행할 작업:**

* 디렉터리에서 `claude`를 실행하고 신뢰 대화를 수락합니다. 부모 디렉터리가 이미 신뢰된 경우에도 대화가 나타나며, 보류 중인 규칙을 나열하고 거절하고 규칙 없이 계속 작업할 수 있습니다. v2.1.200 이전에는 해당 상황에서 대화가 나타나지 않았으므로 이 단계를 완료할 수 없었습니다.
* `-p`를 사용한 [비대화형 모드](/docs/ko/headless)에서는 대화가 표시되지 않습니다. 메시지가 출력하는 정확한 `projects` 키를 사용하여 `~/.claude.json`에서 `hasTrustDialogAccepted` 항목을 설정합니다.
* 메시지가 `.claude/settings.local.json`을 지정하고 git 저장소 외부 또는 홈 디렉터리에서 Claude Code를 시작한 경우 v2.1.200 이상으로 업데이트합니다. 버전 2.1.196부터 2.1.199까지는 해당 작업 공간에서 자신의 `.claude/settings.local.json`을 저장소 제공으로 취급했습니다. v2.1.207 이상에서는 git 저장소 외부에서 폴더를 신뢰하지 않은 경우 업데이트만으로는 충분하지 않습니다. 폴더가 저장소 내부에 있지 않은지 확인하면 git이 실행되고, Claude Code는 신뢰 대화를 수락한 후에만 해당 확인을 실행하므로 첫 번째 단계를 사용합니다. 홈 디렉터리 및 기타 [구성 홈](/docs/ko/permissions#project-allow-rules-and-workspace-trust)은 제외되며 대화를 기다리지 않습니다. [프로젝트 allow 규칙 및 작업 공간 신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust)를 참조하세요.

<h2 id="responses-seem-lower-quality-than-usual">
  응답 품질이 평소보다 낮아 보입니다
</h2>

Claude의 답변이 예상보다 덜 능력 있어 보이지만 오류가 표시되지 않는 경우, 원인은 일반적으로 모델 자체가 아니라 대화 상태입니다. Claude Code는 모델 버전을 자동으로 변경하지 않습니다. 세 가지 특정 경우에만 폴백 모델로 전환할 수 있습니다.

* 구성된 [`--fallback-model`](/docs/ko/cli-reference#cli-flags)은 가용성 오류 후 해당 턴에만 인수를 받으며, 트랜스크립트에 공지가 표시됩니다.
* Amazon Bedrock 또는 Google Cloud의 Agent Platform 시작 확인에서 기본 모델을 사용할 수 없음을 발견합니다.
* [자동 모델 폴백](/docs/ko/model-config#automatic-model-fallback)은 Fable 5에서 세션을 기본 Opus 모델로 이동하고 트랜스크립트에 공지를 표시합니다.

아래의 모델 선택 확인은 두 번째 및 세 번째 경우를 포착합니다. 첫 번째는 `/model` 변경이 아니라 트랜스크립트 공지로 나타납니다. [모델 구성](/docs/ko/model-config)은 각 폴백이 적용되는 시기를 설명합니다.

먼저 다음을 확인하십시오.

* **모델 선택**: `/model`을 실행하여 예상하는 모델에 있는지 확인합니다. 이전 `/model` 선택 또는 `ANTHROPIC_MODEL` 환경 변수로 인해 의도한 것보다 작은 모델에 있을 수 있습니다.
* **노력 수준**: `/effort`를 실행하여 현재 추론 수준을 확인하고 어려운 디버깅 또는 설계 작업을 위해 높입니다. 기본값은 모델에 따라 다르므로 최대값 이하에 있다고 가정하기 전에 확인하십시오. 모델별 기본값 및 `ultrathink` 바로 가기는 [노력 수준 조정](/docs/ko/model-config#adjust-effort-level)을 참조하십시오.
* **컨텍스트 압력**: `/context`를 실행하여 윈도우가 얼마나 찼는지 확인합니다. 용량에 가까우면 자연스러운 지점에서 `/compact`를 실행하거나 `/clear`를 실행하여 새로 시작합니다. [컨텍스트 윈도우 탐색](/docs/ko/context-window)에서 자동 압축이 이전 턴에 어떻게 영향을 미치는지 확인하십시오.
* **오래된 지침**: 크거나 오래된 `CLAUDE.md` 파일 및 MCP 도구 정의는 컨텍스트를 소비하고 응답을 조종할 수 있습니다. `/doctor` 점검은 과도하게 큰 메모리 파일 및 사용하지 않는 확장을 표시하며, `/context`는 MCP 도구 토큰 사용을 표시합니다. v2.1.205 이전에는 `/doctor`가 과도하게 큰 메모리 파일 및 서브에이전트 정의를 표시하는 진단 화면을 열었습니다.

응답이 잘못되면 수정으로 회신하는 것보다 보통 되감기가 더 잘 작동합니다. Esc를 두 번 누르거나 `/rewind`를 실행하여 잘못된 턴 이전으로 돌아간 다음 더 구체적인 프롬프트로 다시 표현합니다. 스레드 내에서 수정하면 잘못된 시도가 컨텍스트에 남아 있어 나중의 답변을 고정할 수 있습니다. [체크포인팅](/docs/ko/checkpointing)을 참조하십시오.

위의 항목을 확인한 후에도 품질이 여전히 좋지 않으면 `/feedback`을 실행하고 예상한 것과 얻은 것을 설명합니다. 이 방식으로 제출된 피드백에는 대화 트랜스크립트가 포함되며, 이는 Anthropic이 실제 회귀를 진단하는 가장 빠른 방법입니다. 환경에서 `/feedback`을 사용할 수 없는 경우 [오류 보고](#report-an-error)를 참조하십시오.

Claude가 의심되는 프롬프트 주입에 대해 경고하거나 의심되는 주입으로 인해 요청을 거부하고, 경고가 명명하는 텍스트가 파일 또는 웹 콘텐츠가 아니라 Claude Code가 대화에 자동으로 추가하는 컨텍스트인 경우 `claude update`를 실행하고 다시 시도합니다. 업데이트 후 경고가 반복되면 플래그된 콘텐츠를 프롬프트에 다시 붙여넣는 대신 [보고](#report-an-error)하십시오. v2.1.201 이전에는 Sonnet 5가 같은 방식으로 일부 요청을 거부했습니다.

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
