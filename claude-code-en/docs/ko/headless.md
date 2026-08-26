> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code를 프로그래밍 방식으로 실행하기

> Agent SDK를 사용하여 CLI, Python 또는 TypeScript에서 Claude Code를 프로그래밍 방식으로 실행합니다.

[Agent SDK](/docs/ko/agent-sdk/overview)는 Claude Code를 구동하는 동일한 도구, 에이전트 루프 및 컨텍스트 관리를 제공합니다. 스크립트 및 CI/CD용 CLI로 사용하거나 완전한 프로그래밍 방식 제어를 위한 [Python](/docs/ko/agent-sdk/python) 및 [TypeScript](/docs/ko/agent-sdk/typescript) 패키지로 사용할 수 있습니다.

Claude Code를 비대화형 모드에서 실행하려면 프롬프트와 함께 `-p`를 전달하고 [CLI 옵션](/docs/ko/cli-reference)을 사용합니다:

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

이 페이지는 CLI(`claude -p`)를 통한 Agent SDK 사용을 다룹니다. 구조화된 출력, 도구 승인 콜백 및 기본 메시지 객체가 있는 Python 및 TypeScript SDK 패키지의 경우 [전체 Agent SDK 문서](/docs/ko/agent-sdk/overview)를 참조하십시오.

<h2 id="basic-usage">
  기본 사용법
</h2>

`-p`(또는 `--print`) 플래그를 모든 `claude` 명령에 추가하여 비대화형으로 실행합니다. 모든 [CLI 옵션](/docs/ko/cli-reference)은 `-p`와 함께 작동합니다:

* `--continue`는 [대화 계속하기](#continue-conversations)용
* `--allowedTools`는 [도구 자동 승인](#auto-approve-tools)용
* `--output-format`은 [구조화된 출력](#get-structured-output)용

이 예제는 코드베이스에 대해 Claude에 질문하고 응답을 출력합니다:

```bash theme={null}
claude -p "What does the auth module do?"
```

<h3 id="start-faster-with-bare-mode">
  베어 모드로 더 빠르게 시작하기
</h3>

`--bare`를 추가하여 hooks, skills, plugins, MCP 서버, 자동 메모리 및 CLAUDE.md의 자동 검색을 건너뛰어 시작 시간을 단축합니다. 이를 사용하지 않으면 `claude -p`는 대화형 세션과 동일한 [컨텍스트](/docs/ko/how-claude-code-works#the-context-window)를 로드하며, 작업 디렉토리 또는 `~/.claude`에 구성된 모든 항목을 포함합니다.

베어 모드는 모든 머신에서 동일한 결과가 필요한 CI 및 스크립트에 유용합니다. 팀원의 `~/.claude`에 있는 hook이나 프로젝트의 `.mcp.json`에 있는 MCP 서버는 베어 모드가 이들을 읽지 않기 때문에 실행되지 않습니다. 명시적으로 전달하는 플래그만 적용됩니다.

이 예제는 베어 모드에서 일회성 요약 작업을 실행하고 Read 도구를 사전 승인하여 권한 프롬프트 없이 호출이 완료되도록 합니다:

```bash theme={null}
claude --bare -p "Summarize this file" --allowedTools "Read"
```

베어 모드에서 Claude는 Bash, 파일 읽기 및 파일 편집 도구에 액세스할 수 있습니다. 플래그를 사용하여 필요한 컨텍스트를 전달합니다:

| 로드할 항목      | 사용                                                      |
| ----------- | ------------------------------------------------------- |
| 시스템 프롬프트 추가 | `--append-system-prompt`, `--append-system-prompt-file` |
| 설정          | `--settings <file-or-json>`                             |
| MCP 서버      | `--mcp-config <file-or-json>`                           |
| 사용자 정의 에이전트 | `--agents <json>`                                       |
| 플러그인        | `--plugin-dir <path>`, `--plugin-url <url>`             |

베어 모드는 OAuth 및 키체인 읽기를 건너뜁니다. Anthropic 인증은 `ANTHROPIC_API_KEY` 또는 `--settings`에 전달된 JSON의 `apiKeyHelper`에서 가져와야 합니다. Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry는 일반적인 공급자 자격 증명을 사용합니다.

<Note>
  `--bare`는 스크립트 및 SDK 호출에 권장되는 모드이며 향후 릴리스에서 `-p`의 기본값이 될 것입니다.
</Note>

<h3 id="background-tasks-at-exit">
  종료 시 백그라운드 작업
</h3>

Claude가 `claude -p` 실행 중에 [백그라운드 Bash 작업](/docs/ko/tools-reference#bash-tool-behavior)을 시작하는 경우(예: 개발 서버 또는 감시 빌드), 해당 셸은 Claude가 최종 결과를 반환하고 stdin이 닫힌 후 약 5초 후에 종료됩니다. 유예 기간을 통해 결과 직후에 완료되는 작업이 여전히 출력을 전달할 수 있습니다. v2.1.163 이전에는 종료되지 않는 백그라운드 프로세스가 `claude -p` 호출을 무한정 열어 두었습니다.

백그라운드 [서브에이전트](/docs/ko/sub-agents) 및 워크플로우는 5초 유예 기간에서 제외됩니다. 이들의 결과가 최종 출력의 일부이기 때문에 `claude -p`는 이들이 완료될 때까지 기다립니다. v2.1.182부터 해당 대기는 기본적으로 10분으로 제한되므로 중단된 백그라운드 에이전트가 프로세스를 무한정 열어 두지 않습니다. [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/ko/env-vars)로 상한을 조정하거나 제한 없이 대기하도록 `0`으로 설정합니다.

<h2 id="examples">
  예제
</h2>

이 예제들은 일반적인 CLI 패턴을 강조합니다. CI 및 기타 스크립트 호출의 경우 로컬에 구성된 항목을 선택하지 않도록 [`--bare`](#start-faster-with-bare-mode)를 추가합니다.

<h3 id="pipe-data-through-claude">
  Claude를 통해 데이터 파이프하기
</h3>

비대화형 모드는 stdin을 읽으므로 다른 명령줄 도구처럼 데이터를 파이프하고 응답을 리디렉션할 수 있습니다.

이 예제는 빌드 로그를 Claude에 파이프하고 설명을 파일에 씁니다:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

`--output-format json`을 사용하면 응답 페이로드에 `total_cost_usd`와 모델별 비용 분석이 포함되므로 스크립트 호출자는 [사용 대시보드](/docs/ko/costs)를 참조하지 않고도 호출당 지출을 추적할 수 있습니다.

<Note>
  Claude Code v2.1.128부터 파이프된 stdin은 10MB로 제한됩니다. 제한을 초과하면 Claude Code는 명확한 오류 메시지와 함께 0이 아닌 상태로 종료됩니다. 더 큰 입력으로 작업하려면 콘텐츠를 파일에 작성하고 파이프하는 대신 프롬프트에서 파일 경로를 참조합니다.
</Note>

<h3 id="add-claude-to-a-build-script">
  빌드 스크립트에 Claude 추가
</h3>

비대화형 호출을 스크립트로 래핑하여 Claude를 프로젝트별 린터 또는 검토자로 사용할 수 있습니다.

이 `package.json` 스크립트는 `main`에 대한 diff를 Claude에 파이프하고 오타를 보고하도록 요청합니다. diff를 파이프하면 Claude가 이를 읽기 위해 Bash 권한이 필요하지 않으며, 이스케이프된 큰따옴표는 스크립트를 Windows에 이식 가능하게 유지합니다:

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

<h3 id="get-structured-output">
  구조화된 출력 가져오기
</h3>

`--output-format`을 사용하여 응답이 반환되는 방식을 제어합니다:

* `text`(기본값): 일반 텍스트 출력
* `json`: 결과, 세션 ID 및 메타데이터가 포함된 구조화된 JSON
* `stream-json`: 실시간 스트리밍을 위한 줄 구분 JSON

이 예제는 세션 메타데이터와 함께 프로젝트 요약을 JSON으로 반환하며, 텍스트 결과는 `result` 필드에 있습니다:

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

특정 스키마를 준수하는 출력을 얻으려면 `--output-format json`을 `--json-schema` 및 [JSON Schema](https://json-schema.org/) 정의와 함께 사용합니다. 응답에는 요청에 대한 메타데이터(세션 ID, 사용량 등)가 포함되며 구조화된 출력은 `structured_output` 필드에 있습니다.

이 예제는 함수 이름을 추출하고 문자열 배열로 반환합니다:

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

JSON Schema가 유효하지 않으면 `claude`는 `Error: --json-schema is not a valid JSON Schema`와 함께 종료되고 검증자의 진단이 뒤따릅니다. Claude Code는 `format` 키워드를 사용하는 스키마(예: `"format": "email"`)를 허용하지만 `format`을 주석으로 취급하고 이를 적용하지 않습니다. v2.1.205 이전에는 Claude Code가 유효하지 않은 스키마를 자동으로 무시하고 구조화되지 않은 텍스트를 반환했으며, `format`을 포함하는 모든 스키마를 유효하지 않은 것으로 취급했습니다.

<Tip>
  [jq](https://jqlang.github.io/jq/)와 같은 도구를 사용하여 응답을 구문 분석하고 특정 필드를 추출합니다:

  ```bash theme={null}
  # 텍스트 결과 추출
  claude -p "Summarize this project" --output-format json | jq -r '.result'

  # 구조화된 출력 추출
  claude -p "Extract function names from auth.py" \
    --output-format json \
    --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
    | jq '.structured_output'
  ```
</Tip>

<h3 id="stream-responses">
  응답 스트리밍
</h3>

`--output-format stream-json`을 `--verbose` 및 `--include-partial-messages`와 함께 사용하여 생성되는 토큰을 수신합니다. 각 줄은 이벤트를 나타내는 JSON 객체입니다:

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

스트림의 마지막 줄은 최종 응답 텍스트, 비용 및 세션 메타데이터가 포함된 `result` 메시지입니다. v2.1.208 이전에는 큰 응답을 파이프하면 최종 줄이 잘리고 `result` 메시지가 생략될 수 있었습니다.

다음 예제는 [jq](https://jqlang.github.io/jq/)를 사용하여 텍스트 델타를 필터링하고 스트리밍 텍스트만 표시합니다. `-r` 플래그는 원본 문자열(따옴표 없음)을 출력하고 `-j`는 줄 바꿈 없이 조인하므로 토큰이 계속 스트리밍됩니다:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

API 요청이 재시도 가능한 오류로 실패하면 Claude Code는 재시도하기 전에 `system/api_retry` 이벤트를 내보냅니다. 이를 사용하여 재시도 진행 상황을 표시하거나 사용자 정의 백오프 로직을 구현할 수 있습니다.

| 필드               | 유형            | 설명                                                                                                                                                                                           |
| ---------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`    | 메시지 유형                                                                                                                                                                                       |
| `subtype`        | `"api_retry"` | 이를 재시도 이벤트로 식별                                                                                                                                                                               |
| `attempt`        | 정수            | 현재 시도 번호, 1부터 시작                                                                                                                                                                             |
| `max_retries`    | 정수            | 허용된 총 재시도 횟수                                                                                                                                                                                 |
| `retry_delay_ms` | 정수            | 다음 시도까지의 밀리초                                                                                                                                                                                 |
| `error_status`   | 정수 또는 null    | HTTP 상태 코드, 또는 HTTP 응답이 없는 연결 오류의 경우 `null`                                                                                                                                                  |
| `error`          | 문자열           | 오류 범주: `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens` 또는 `unknown` |
| `uuid`           | 문자열           | 고유 이벤트 식별자                                                                                                                                                                                   |
| `session_id`     | 문자열           | 이벤트가 속한 세션                                                                                                                                                                                   |

`system/init` 이벤트는 모델, 도구, MCP 서버 및 로드된 플러그인을 포함한 세션 메타데이터를 보고합니다. 이는 시작 이벤트가 앞에 오지 않는 한 스트림의 첫 번째 이벤트입니다:

* [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ko/env-vars)이 설정되었을 때 `plugin_install` 이벤트입니다.
* 구성된 [`SessionStart`](/docs/ko/hooks#sessionstart) 또는 [`Setup`](/docs/ko/hooks#setup) 훅이 실행되는 동안 [`hook_started`, `hook_progress` 및 `hook_response` 이벤트](/docs/ko/agent-sdk/typescript#sdkhookstartedmessage)입니다. 이들은 훅이 생성할 때 스트리밍됩니다. Claude Code v2.1.169부터 v2.1.203까지는 훅이 완료된 후 한 배치로 전달했으며, 여전히 `system/init` 앞에 있었습니다. v2.1.204는 라이브 전달을 복원했습니다.

이 이벤트는 또한 이 Claude Code 버전이 구현하는 프로토콜 동작의 이름을 지정하는 선택적 `capabilities` 문자열 배열을 전달합니다(예: `interrupt_receipt_v1`). 버전 문자열을 비교하는 대신 기능을 감지하는 데 사용하고 인식하지 못하는 값은 무시합니다. 이 필드는 Claude Code v2.1.205 이상이 필요하며 이전 버전에는 없습니다. 기능 목록은 [`SDKSystemMessage`](/docs/ko/agent-sdk/typescript#sdksystemmessage)를 참조하십시오.

플러그인 필드를 사용하여 플러그인이 로드되지 않았을 때 CI를 실패하게 합니다:

| 필드              | 유형 | 설명                                                                                                                                                                             |
| --------------- | -- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `plugins`       | 배열 | 성공적으로 로드된 플러그인, 각각 `name` 및 `path` 포함                                                                                                                                          |
| `plugin_errors` | 배열 | 만족하지 않은 종속성 버전 및 `--plugin-dir` 로드 실패(예: 누락된 경로 또는 유효하지 않은 아카이브)와 같은 플러그인 로드 시간 오류, 각각 `plugin`, `type` 및 `message` 포함. 영향을 받는 플러그인은 강등되고 `plugins`에서 제외됩니다. 오류가 없을 때 키는 생략됩니다 |

[`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ko/env-vars)이 설정되면 Claude Code는 첫 번째 턴 전에 마켓플레이스 플러그인이 설치되는 동안 `system/plugin_install` 이벤트를 내보냅니다. 이를 사용하여 자신의 UI에서 설치 진행 상황을 표시합니다.

| 필드           | 유형                                                      | 설명                                                                            |
| ------------ | ------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `type`       | `"system"`                                              | 메시지 유형                                                                        |
| `subtype`    | `"plugin_install"`                                      | 이를 플러그인 설치 이벤트로 식별                                                            |
| `status`     | `"started"`, `"installed"`, `"failed"` 또는 `"completed"` | `started` 및 `completed`는 전체 설치를 괄호로 묶음; `installed` 및 `failed`는 개별 마켓플레이스를 보고 |
| `name`       | 문자열, 선택 사항                                              | 마켓플레이스 이름, `installed` 및 `failed`에 표시                                         |
| `error`      | 문자열, 선택 사항                                              | 실패 메시지, `failed`에 표시                                                          |
| `uuid`       | 문자열                                                     | 고유 이벤트 식별자                                                                    |
| `session_id` | 문자열                                                     | 이벤트가 속한 세션                                                                    |

콜백 및 메시지 객체를 사용한 프로그래밍 방식 스트리밍의 경우 Agent SDK 문서의 [실시간 응답 스트리밍](/docs/ko/agent-sdk/streaming-output)을 참조하십시오.

<h3 id="auto-approve-tools">
  도구 자동 승인
</h3>

`--allowedTools`를 사용하여 Claude가 프롬프트 없이 특정 도구를 사용하도록 합니다. 이 예제는 테스트 스위트를 실행하고 실패를 수정하며, Claude가 권한을 요청하지 않고 Bash 명령을 실행하고 파일을 읽고 편집할 수 있도록 합니다:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

전체 세션에 대한 기준선을 설정하려면 개별 도구를 나열하는 대신 [권한 모드](/docs/ko/permission-modes)를 전달합니다. `dontAsk`는 `permissions.allow` 규칙이나 [읽기 전용 명령 집합](/docs/ko/permissions#read-only-commands)에 없는 모든 항목을 거부하며, 이는 잠긴 CI 실행에 유용합니다. `AskUserQuestion`, 조직이 [`ask`로 설정한](/docs/ko/mcp#organization-controls-on-connector-tools) 커넥터 도구, 그리고 [`requiresUserInteraction`으로 표시된](/docs/ko/mcp#require-approval-for-a-specific-tool) MCP 도구는 허용 규칙이 일치하더라도 거부됩니다.

`acceptEdits`는 Claude가 프롬프트 없이 파일을 쓸 수 있도록 하고 `mkdir`, `touch`, `mv` 및 `cp`와 같은 일반적인 파일 시스템 명령을 자동 승인합니다. 다른 셸 명령 및 네트워크 요청은 여전히 `--allowedTools` 항목이나 `permissions.allow` 규칙이 필요하며, 그렇지 않으면 시도될 때 실행이 중단됩니다:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="create-a-commit">
  커밋 생성
</h3>

이 예제는 스테이징된 변경 사항을 검토하고 적절한 메시지로 커밋을 생성합니다:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

`--allowedTools` 플래그는 [권한 규칙 구문](/docs/ko/settings#permission-rule-syntax)을 사용합니다. 뒤의 ` *`는 접두사 일치를 활성화하므로 `Bash(git diff *)`는 `git diff`로 시작하는 모든 명령을 허용합니다. 공백이 중요합니다: 없으면 `Bash(git diff*)`도 `git diff-index`와 일치합니다.

<Note>
  사용자가 호출한 [skills](/docs/ko/skills) 및 사용자 정의 명령은 `-p` 모드에서 작동합니다: 프롬프트 문자열에 `/skill-name`을 포함하면 Claude Code가 실행하기 전에 이를 확장합니다. `/login`과 같은 대화형 대화를 열어주는 기본 제공 명령은 `-p` 모드에서 사용할 수 없습니다. `/model`, `/effort`, `/fast`, `/color` 및 `/rename`은 값을 인수로 받습니다(예: `/model sonnet`). `/mcp`는 인수 없이 서버 상태의 텍스트 요약을 출력합니다. 이러한 형식은 Claude Code v2.1.205 이상이 필요하며 각 명령의 [가용성 참고 사항](/docs/ko/commands#all-commands)을 따릅니다. `-p` 호출에서 설정을 변경하려면 `/config`에 `key=value`를 전달합니다(예: `/config thinking=false`).
</Note>

<h3 id="customize-the-system-prompt">
  시스템 프롬프트 사용자 정의
</h3>

`--append-system-prompt`를 사용하여 Claude Code의 기본 동작을 유지하면서 지침을 추가합니다. 이 예제는 PR diff를 Claude에 파이프하고 보안 취약점을 검토하도록 지시합니다:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

기본 프롬프트를 완전히 바꾸는 `--system-prompt`를 포함한 더 많은 옵션은 [시스템 프롬프트 플래그](/docs/ko/cli-reference#system-prompt-flags)를 참조하십시오.

<h3 id="continue-conversations">
  대화 계속하기
</h3>

`--continue`를 사용하여 가장 최근 대화를 계속하거나 `--resume`을 세션 ID와 함께 사용하여 특정 대화를 계속합니다. 이 예제는 검토를 실행한 다음 후속 프롬프트를 보냅니다:

```bash theme={null}
# 첫 번째 요청
claude -p "Review this codebase for performance issues"

# 가장 최근 대화 계속
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

여러 대화를 실행 중인 경우 세션 ID를 캡처하여 특정 대화를 재개합니다:

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

동일한 디렉터리에서 두 명령을 실행합니다: 세션 ID 조회는 현재 프로젝트 디렉터리 및 해당 git worktrees로 범위가 지정됩니다. 전체 범위 규칙은 [세션 재개](/docs/ko/sessions#resume-a-session)를 참조하십시오.

<h2 id="next-steps">
  다음 단계
</h2>

* [Agent SDK 빠른 시작](/docs/ko/agent-sdk/quickstart): Python 또는 TypeScript로 첫 번째 에이전트 구축
* [CLI 참조](/docs/ko/cli-reference): 모든 CLI 플래그 및 옵션
* [GitHub Actions](/docs/ko/github-actions): GitHub 워크플로우에서 Agent SDK 사용
* [GitLab CI/CD](/docs/ko/gitlab-ci-cd): GitLab 파이프라인에서 Agent SDK 사용
