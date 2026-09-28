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

`-p`(또는 `--print`) 플래그를 모든 `claude` 명령에 추가하여 비대화형으로 실행합니다. 모든 [CLI 옵션](/docs/ko/cli-reference)이 `-p`와 함께 작동하는 것은 아닙니다. Claude Code는 `--bg`를 거부하고, 작업 설명과 함께 `--cloud`를 거부하며, 충돌을 명시하는 오류가 발생합니다. 세션 ID와 함께 `--cloud`를 사용하고 `-p`를 사용하면 [해당 클라우드 세션에 메시지를 큐에 넣고](/docs/ko/claude-code-on-the-web#send-follow-ups-from-the-cli) 종료합니다. `-p`와 함께 사용할 옵션은 종종 다음을 포함합니다:

* `--continue`는 [대화 계속하기](#continue-conversations)용
* `--allowedTools`는 [도구 자동 승인](#auto-approve-tools)용
* `--output-format`은 [구조화된 출력](#get-structured-output)용

이 예제는 코드베이스에 대해 Claude에 질문하고 응답을 출력합니다:

```bash theme={null}
claude -p "What does the auth module do?"
```

Claude Code는 성공 시 코드 0으로 종료되고 실행이 실패하면 0이 아닌 코드로 종료되므로 스크립트는 종료 상태에 따라 분기할 수 있습니다. 잘못된 플래그를 전달하면 Claude Code는 실행이 시작되기 전에 오류를 stderr에 보고합니다. 실행 중에 인증 누락과 같은 오류가 발생하면 Claude Code는 오류를 stdout의 결과로 출력합니다.

<h3 id="start-faster-with-bare-mode">
  베어 모드로 더 빠르게 시작하기
</h3>

`--bare`를 추가하여 hooks, skills, 사용자 정의 명령, [서브에이전트](/docs/ko/sub-agents), 설치된 플러그인, MCP 서버, 자동 메모리 및 CLAUDE.md의 자동 검색을 건너뛰어 시작 시간을 단축합니다. 이를 사용하지 않으면 `claude -p`는 대화형 세션과 동일한 [컨텍스트](/docs/ko/how-claude-code-works#the-context-window)를 로드하며, 작업 디렉토리 또는 `~/.claude`에 구성된 모든 항목을 포함합니다.

베어 모드는 모든 머신에서 동일한 결과가 필요한 CI 및 스크립트에 유용합니다. 팀원의 `~/.claude`에 있는 hook이나 프로젝트의 `.mcp.json`에 있는 MCP 서버는 베어 모드가 이들을 읽지 않기 때문에 실행되지 않습니다. `--add-dir`로 지정한 디렉토리는 부분적인 예외입니다: 베어 모드는 해당 `.claude/skills/` 폴더에서 skills를 로드하지만 여전히 해당 `.claude/commands/` 및 `.claude/agents/` 폴더를 건너뜁니다. [추가 디렉토리의 Skills](/docs/ko/skills#skills-from-additional-directories)는 로드되는 항목과 로드되지 않는 항목을 다룹니다.

`--bare` 없이 `-p` 세션은 프로젝트의 `.claude/settings.json`에서 hooks를 실행하고 해당 `.mcp.json`의 서버를 연결합니다. 이는 신뢰한 적이 없는 폴더에서도 마찬가지입니다. `-p` 세션은 워크스페이스 신뢰 대화 상자나 서버별 승인 프롬프트를 표시하지 않습니다. [폴더를 신뢰하기 전에 실행되는 항목](/docs/ko/permissions#what-runs-before-you-trust-a-folder)은 `-p` 아래의 각 종류의 저장소 콘텐츠와 이를 제외하는 방법을 다룹니다.

이 예제는 베어 모드에서 일회성 요약 작업을 실행하고 Read 도구를 사전 승인하여 권한 프롬프트 없이 호출이 완료되도록 합니다. 베어 모드는 구독 로그인을 사용하지 않기 때문에 실행하기 전에 `ANTHROPIC_API_KEY`를 설정합니다:

```bash theme={null}
claude --bare -p "Summarize README.md" --allowedTools "Read"
```

베어 모드에서 Claude Code는 OAuth 자격 증명이나 시스템 키체인을 읽지 않습니다. Anthropic API의 경우 환경에서 `ANTHROPIC_API_KEY`를 설정하고, [Claude Console](https://platform.claude.com)에서 생성한 키를 사용하거나, `--settings` JSON에서 `apiKeyHelper`를 제공합니다. Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry는 일반적인 공급자 자격 증명을 계속 읽습니다.

베어 모드에서 Claude는 Bash, 파일 읽기 및 파일 편집 도구에 액세스할 수 있습니다. 플래그를 사용하여 필요한 컨텍스트를 전달합니다:

| 로드할 항목      | 사용                                                      |
| ----------- | ------------------------------------------------------- |
| 시스템 프롬프트 추가 | `--append-system-prompt`, `--append-system-prompt-file` |
| 설정          | `--settings <file-or-json>`                             |
| MCP 서버      | `--mcp-config <file-or-json>`                           |
| 사용자 정의 에이전트 | `--agents <json>`                                       |
| 플러그인        | `--plugin-dir <path>`, `--plugin-url <url>`             |

<Note>
  `--bare`는 스크립트 및 SDK 호출에 권장되는 모드이며 향후 릴리스에서 `-p`의 기본값이 될 것입니다.
</Note>

<h3 id="background-tasks-at-exit">
  종료 시 백그라운드 작업
</h3>

Claude가 `claude -p` 실행 중에 [백그라운드 Bash 작업](/docs/ko/tools-reference#bash-tool-behavior)을 시작하는 경우(예: 개발 서버 또는 감시 빌드), 해당 셸은 Claude가 최종 결과를 반환하고 stdin이 닫힌 후 약 5초 후에 종료됩니다. 유예 기간을 통해 결과 직후에 완료되는 작업이 여전히 출력을 전달할 수 있습니다.

Claude가 `claude -p` 실행 중에 백그라운드 [서브에이전트](/docs/ko/sub-agents) 또는 워크플로우를 시작하면 `claude -p`는 해당 작업이 완료될 때까지 열려 있습니다. 왜냐하면 해당 결과가 최종 출력의 일부이기 때문입니다.

기본적으로 대기는 연속 유휴 대기 10분 후에 종료되므로 중단된 서브에이전트 또는 워크플로우가 프로세스를 무한정 열어 두지 않습니다. 이 시점에서 Claude Code는 여전히 실행 중인 모든 항목을 중지하고 해당 부분 결과를 삭제합니다. 제한을 변경하려면 [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/ko/env-vars)를 설정하거나 제한 없이 대기하도록 `0`으로 설정합니다.

Claude가 `claude -p` 실행 중에 [Monitor](/docs/ko/tools-reference#monitor-tool) 감시를 시작하면 Claude Code는 감시가 시간 초과되거나 10분 상한이 대기를 종료할 때까지 감시를 기다립니다. 대기하는 동안 Claude는 감시가 보고하는 항목에 계속 응답합니다. 기본적으로 감시는 Claude가 시작한 후 5분 후에 시간 초과됩니다.

<h3 id="stop-a-run-with-sigterm">
  SIGTERM으로 실행 중지
</h3>

`claude -p` 실행을 SIGTERM으로 중지하면(예: `kill` 또는 프로세스 감독자에서), Claude Code는 코드 143으로 종료됩니다. Claude Code는 진행 중인 턴을 완료하지 않은 상태로 두고 해당 턴에 대한 결과를 기록하지 않습니다. 턴을 대신 종료하려면 SIGINT를 보내거나 Agent SDK의 `interrupt()`를 호출한 후 프로세스를 중지합니다.

SIGTERM에서 Claude Code는 여전히 실행 중인 모든 Bash 명령의 프로세스 트리를 종료합니다. Claude Code는 [`SessionEnd` hooks](/docs/ko/hooks#sessionend)를 실행하고 종료합니다. 종료하는 동안 Claude Code는 새로운 도구 호출을 시작하지 않고, 새로운 모델 요청을 보내지 않으며, `SessionEnd` 이외의 hook을 실행하지 않습니다. 신호가 도착했을 때 실행이 명령 중간에 있거나 권한 프롬프트에 대한 답변을 기다리고 있었다면 Claude Code는 다음과 같이 해당 단계를 처리합니다:

* **명령 실행 중**: Claude Code는 명령을 세션에서 종료된 것으로 기록합니다.
* **권한 프롬프트에 대한 답변 대기 중**: 프로세스에 SIGTERM을 보내면 Claude Code는 프롬프트를 답변하지 않은 상태로 둡니다. 프로그램이 Agent SDK를 통해 세션을 닫으면 SDK는 신호를 보내기 전에 Claude Code의 입력을 종료하고 Claude Code는 입력이 종료되는 즉시 프롬프트를 취소합니다.

[세션을 재개](#continue-conversations)할 때 Claude Code는 SIGTERM이 완료하지 않은 턴을 계속합니다.

<h2 id="examples">
  예제
</h2>

이 예제들은 일반적인 CLI 패턴을 강조합니다. `auth.py` 또는 `build-error.txt`와 같이 파일을 이름으로 지정하는 명령의 경우, 자신의 프로젝트에서 파일을 대체하십시오. CI 또는 기타 스크립트 환경에서는 [`--bare`](#start-faster-with-bare-mode)를 추가하여 Claude Code가 호스트의 hooks, 플러그인, 자동 메모리 또는 `CLAUDE.md`를 로드하지 않고 시작하도록 하십시오.

<h3 id="pipe-data-through-claude">
  Claude를 통해 데이터 파이프
</h3>

비대화형 모드는 stdin을 읽으므로 다른 명령줄 도구처럼 데이터를 파이프하고 응답을 리디렉션할 수 있습니다.

이 예제는 빌드 로그를 Claude로 파이프하고 설명을 파일에 씁니다:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

`--output-format json`을 사용하면 응답 페이로드에 `total_cost_usd`와 모델별 비용 분석이 포함되므로 스크립트 호출자는 [사용 대시보드](/docs/ko/costs)를 참조하지 않고도 지출을 추적할 수 있습니다. `--continue` 또는 `--resume`으로 이전 대화를 계속할 때 실행은 대화의 전체 합계를 보고하며, [이전 실행의 지출이 포함됩니다](/docs/ko/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). 두 수치 모두 [클라이언트 측 추정치](/docs/ko/agent-sdk/cost-tracking)이며 실제 청구서와 다를 수 있습니다.

<Note>
  파이프된 stdin은 10MB로 제한됩니다. 제한을 초과하면 Claude Code는 명확한 오류 메시지와 함께 0이 아닌 상태로 종료됩니다. 더 큰 입력으로 작업하려면 콘텐츠를 파일에 작성하고 파이프하는 대신 프롬프트에서 파일 경로를 참조하십시오.
</Note>

Claude Code가 stdin을 읽을 수 없는 경우(예: 이를 시작한 프로세스가 끝을 연결 해제한 경우) Claude Code는 stderr에 경고를 인쇄하고 명령줄의 프롬프트로 계속 진행합니다. v2.1.211 이전에는 Windows에서 읽을 수 없는 stdin이 세션을 충돌시키거나 출력 없이 자동으로 종료되었습니다.

<h3 id="add-claude-to-a-build-script">
  빌드 스크립트에 Claude 추가
</h3>

비대화형 호출을 스크립트로 래핑하여 Claude를 프로젝트별 린터 또는 검토자로 사용할 수 있습니다.

이 `package.json` 스크립트는 `main`에 대한 diff를 Claude로 파이프하고 오타를 보고하도록 요청합니다. diff를 파이프하면 Claude가 이를 읽기 위해 Bash 권한이 필요하지 않으며, 이스케이프된 큰따옴표는 스크립트를 Windows에 이식 가능하게 유지합니다:

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

`npm run lint:claude`로 실행하십시오.

<h3 id="get-structured-output">
  구조화된 출력 얻기
</h3>

`--output-format`을 사용하여 응답이 반환되는 방식을 제어합니다:

* `text` (기본값): 일반 텍스트 출력
* `json`: 결과, 세션 ID 및 메타데이터가 포함된 구조화된 JSON
* `stream-json`: 실시간 스트리밍을 위한 줄 구분 JSON

이 예제는 프로젝트 요약을 세션 메타데이터와 함께 JSON으로 반환하며, 텍스트 결과는 `result` 필드에 있습니다:

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

특정 스키마를 준수하는 출력을 얻으려면 `--output-format json`을 `--json-schema`와 [JSON Schema](https://json-schema.org/) 정의와 함께 사용합니다. 응답에는 요청에 대한 메타데이터(세션 ID, 사용량 등)가 포함되며 구조화된 출력은 `structured_output` 필드에 있습니다.

이 예제는 함수 이름을 추출하고 문자열 배열로 반환합니다:

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

값이 유효한 JSON Schema가 아니면 `claude`는 `Error: --json-schema is not a valid JSON Schema`로 종료되고 그 뒤에 검증자의 진단이 따릅니다. Claude Code는 `"format": "email"`과 같은 `format` 키워드를 사용하는 스키마를 허용하지만 `format`을 주석으로 취급하고 적용하지 않습니다. v2.1.205 이전에는 Claude Code가 유효하지 않은 스키마를 자동으로 무시하고 구조화되지 않은 텍스트를 반환했으며, `format`을 포함하는 모든 스키마를 유효하지 않은 것으로 취급했습니다.

<Tip>
  [jq](https://jqlang.org/)와 같은 도구를 사용하여 응답을 구문 분석하고 특정 필드를 추출합니다:

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

스트림의 마지막 줄은 최종 응답 텍스트, 비용 및 세션 메타데이터가 포함된 `result` 메시지입니다.

소비자가 스트림을 천천히 읽으면 Claude Code는 대기 중인 출력이 드레인될 때까지 기다리며, 대기 시간을 여전히 대기 중인 양에 따라 조정하고 최대 30초로 제한합니다. v2.1.214 이전에는 종료 대기가 약 2초로 제한되어 큰 응답의 끝이 잘릴 수 있었습니다.

다음 예제는 [jq](https://jqlang.org/)를 사용하여 텍스트 델타를 필터링하고 스트리밍 텍스트만 표시합니다. `-r` 플래그는 원시 문자열(따옴표 없음)을 출력하고 `-j`는 줄 바꿈 없이 조인하므로 토큰이 연속으로 스트리밍됩니다:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

콜백 및 메시지 객체를 사용한 프로그래밍 방식의 스트리밍은 Agent SDK 문서의 [실시간으로 응답 스트리밍](/docs/ko/agent-sdk/streaming-output)을 참조하십시오.

<h4 id="follow-subagent-messages">
  서브에이전트 메시지 따라가기
</h4>

[서브에이전트](/docs/ko/sub-agents)의 메시지는 스트림에 `assistant` 및 `user` 메시지로 나타나며, 이들의 `parent_tool_use_id` 필드는 서브에이전트를 생성한 도구 호출의 ID입니다. 주 대화의 메시지는 해당 필드에 `null`을 포함합니다.

[포그라운드](/docs/ko/sub-agents#run-subagents-in-foreground-or-background)에서 실행 중인 서브에이전트의 첫 번째 메시지는 이를 구동하는 프롬프트를 전달하는 `user` 메시지입니다. 첫 번째 메시지 이후 Claude Code는 다음을 내보냅니다:

* **기본값**: 서브에이전트의 `tool_use` 및 `tool_result` 블록.
* **[`--forward-subagent-text`](/docs/ko/cli-reference#cli-flags) 또는 [`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`](/docs/ko/env-vars) 사용**: 서브에이전트의 텍스트 및 사고 블록도 포함되므로 각 서브에이전트의 트랜스크립트를 재구성할 수 있습니다. 이는 Claude Code v2.1.211 이상이 필요합니다.

두 옵션 중 하나를 활성화하면 Claude Code는 [모든 중첩 깊이의 서브에이전트](/docs/ko/sub-agents#let-subagents-spawn-their-own-subagents)에서 메시지를 전달합니다. 각 서브에이전트가 Agent 도구 또는 [포크된 스킬](/docs/ko/skills#run-skills-in-a-subagent)로 시작되었는지 여부와 관계없이 메시지를 전달합니다. 포크된 스킬이 생성하는 서브에이전트의 메시지와 서브에이전트 또는 다른 포크된 스킬 내에서 시작된 포크된 스킬의 메시지는 Claude Code v2.1.275 이상이 필요합니다. `parent_tool_use_id`에서 중첩된 서브에이전트의 메시지는 이를 시작한 Agent 또는 Skill 도구 호출의 ID를 포함하므로 이러한 ID를 따라 전체 중첩 트리를 재구성할 수 있습니다. v2.1.219 이전에는 중첩된 서브에이전트의 메시지가 스트림에 나타나지 않았습니다.

[서브에이전트에서 실행되는](/docs/ko/skills#run-skills-in-a-subagent) 스킬은 스트림에 동일한 방식으로 나타납니다: 포크된 스킬의 첫 번째 메시지는 실행을 구동하는 스킬 콘텐츠를 전달하는 `user` 메시지입니다. 두 옵션 중 하나를 활성화하면 스트림은 포크된 스킬의 텍스트 및 사고 블록도 포함합니다. v2.1.265 이전에는 포크된 스킬의 `tool_use` 및 `tool_result` 블록만 스트림에 나타났습니다.

<h4 id="handle-api-retries">
  API 재시도 처리
</h4>

API 요청이 재시도 가능한 오류로 실패하면 Claude Code는 재시도하기 전에 `system/api_retry` 이벤트를 내보냅니다. v2.1.246 이상에서 `401` 또는 `403`이 [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 자격 증명을 거부하면 Claude Code는 처음 두 번의 재시도를 조용히 수행하고 이벤트 없이 진행한 다음 세 번째 연속 재시도부터 평소대로 이벤트를 내보냅니다. 조용한 재시도는 여전히 `attempt`에 포함됩니다. 이벤트를 사용하여 자신의 인터페이스에서 재시도 진행 상황을 표시할 수 있습니다.

| 필드               | 유형            | 설명                                                                                                                                                                                                                                           |
| ---------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`    | 메시지 유형                                                                                                                                                                                                                                       |
| `subtype`        | `"api_retry"` | 이를 재시도 이벤트로 식별                                                                                                                                                                                                                               |
| `attempt`        | 정수            | 현재 시도 번호, 1부터 시작                                                                                                                                                                                                                             |
| `max_retries`    | 정수            | 이 실패의 원인에 대해 허용된 총 재시도 횟수, 세션 전체 예산보다 적을 수 있음                                                                                                                                                                                                |
| `retry_delay_ms` | 정수            | 다음 시도까지의 밀리초                                                                                                                                                                                                                                 |
| `error_status`   | 정수 또는 null    | 실패한 시도의 HTTP 상태 코드, 또는 시도가 API에서 HTTP 응답을 받지 못한 경우 `null`                                                                                                                                                                                    |
| `no_response`    | 객체, 선택 사항     | 실패한 시도가 [시간 내에 응답 헤더를 받지 못한](/docs/ko/errors#no-response-from-api) 경우에만 존재합니다. `waited_ms`는 해당 시도가 대기한 시간이고 `retry_wait_ms`는 재시도가 대기할 시간입니다. 이러한 이벤트에서 `max_retries`는 이 원인이 일반적으로 받는 하나의 재시도를 반영하며 세션 전체 예산이 아닙니다. Claude Code v2.1.261 이상이 필요합니다 |
| `error`          | 문자열           | 오류 범주: `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, 또는 `unknown`   |
| `uuid`           | 문자열           | 고유 이벤트 식별자                                                                                                                                                                                                                                   |
| `session_id`     | 문자열           | 이벤트가 속한 세션                                                                                                                                                                                                                                   |

<h4 id="read-session-metadata">
  세션 메타데이터 읽기
</h4>

`system/init` 이벤트는 모델, 도구, MCP 서버 및 로드된 플러그인을 포함한 세션 메타데이터를 보고합니다. 시작 이벤트가 앞에 있지 않으면 스트림의 첫 번째 이벤트입니다:

* [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ko/env-vars)이 설정된 경우 `plugin_install` 이벤트.
* 구성된 [`SessionStart`](/docs/ko/hooks#sessionstart) 또는 [`Setup`](/docs/ko/hooks#setup) 훅이 실행되는 동안 [`hook_started`, `hook_progress` 및 `hook_response` 이벤트](/docs/ko/agent-sdk/typescript#sdkhookstartedmessage). 이들은 훅이 생성할 때 스트리밍됩니다. Claude Code v2.1.169부터 v2.1.203까지는 훅이 완료된 후 한 배치로 전달했으며 여전히 `system/init` 앞에 있었습니다; v2.1.204는 라이브 전달을 복원했습니다.

이벤트는 또한 이 Claude Code 버전이 구현하는 프로토콜 동작을 이름으로 지정하는 문자열의 선택적 `capabilities` 배열을 포함합니다(예: `interrupt_receipt_v1` 또는 `interrupt_cancel_queued_v1`). 버전 문자열을 비교하는 대신 기능 감지를 위해 이를 확인하고 인식하지 못하는 값은 무시합니다. 필드는 Claude Code v2.1.205 이상이 필요하며 이전 버전에서는 없습니다. 기능 목록은 [`SDKSystemMessage`](/docs/ko/agent-sdk/typescript#sdksystemmessage)를 참조하십시오.

<h4 id="fail-ci-when-a-plugin-or-mcp-server-doesn’t-load">
  플러그인 또는 MCP 서버가 로드되지 않으면 CI 실패
</h4>

`system/init` 이벤트의 플러그인 필드를 사용하여 로드되지 않은 플러그인을 포착합니다:

| 필드              | 유형 | 설명                                                                                                                                                                               |
| --------------- | -- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | 배열 | 성공적으로 로드된 플러그인, 각각 `name` 및 `path` 포함                                                                                                                                            |
| `plugin_errors` | 배열 | 플러그인 로드 시간 오류, 각각 `plugin`, `type` 및 `message` 포함. 만족하지 않은 종속성 버전 및 누락된 경로 또는 유효하지 않은 아카이브와 같은 `--plugin-dir` 로드 실패를 포함합니다. 영향을 받은 플러그인은 강등되고 `plugins`에서 없습니다. 오류가 없으면 키가 생략됩니다 |

MCP 서버 필드도 동일한 방식으로 사용합니다.&#x20;
`-p`와 함께 [`--mcp-config`](/docs/ko/cli-reference#cli-flags)를 전달하면 Claude Code는 첫 번째 턴을 실행하기 전에 여전히 대기 중인 서버를 기다리며, [`MCP_TIMEOUT`](/docs/ko/env-vars) 시작 시간 초과(기본값 30초)까지 기다립니다. [캐시된 도구 목록](/docs/ko/agent-sdk/mcp#connection-timing)이 있는 원격 서버는 대기를 건너뛰고 `system/init`에서 `pending`을 표시하며 첫 번째 도구 호출에서 연결합니다. 대기는 Claude Code v2.1.221 이상이 필요합니다.

Claude Code는 시작 시 각 `--mcp-config` 항목을 검증하고 검증에 실패한 항목을 건너뜁니다(예: `type`이 없는 `url` 항목). 실행이 계속되고 깔끔하게 종료되므로 이러한 필드를 확인하여 로드되지 않은 서버를 포착합니다:

| 필드                  | 유형 | 설명                                                                                                                                                                                                                                                                                                              |
| ------------------- | -- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_servers`       | 배열 | 세션의 MCP 서버, 각각 `name` 및 `status` 포함                                                                                                                                                                                                                                                                             |
| `mcp_server_errors` | 배열 | 구성 검증으로 건너뛴 `--mcp-config` 항목, 각각 `name`, `type` 및 `message` 포함. `type`은 `unknown_type`, `url_missing_type`, `invalid_config` 또는 `reserved_name`과 같은 건너뛰기 범주입니다; 인식하지 못하는 값은 일반 건너뛰기로 취급합니다. 영향을 받은 서버는 `mcp_servers`에서 없습니다. 오류가 없으면 키가 생략되므로 CI 게이트는 비어 있지 않은 배열에서 실패할 수 있습니다. Claude Code v2.1.219 이상이 필요합니다 |

명령을 터미널에서 직접 실행하면 Claude Code는 또한 `Warning: 1 MCP server skipped due to invalid config:`와 같은 시작 경고를 stderr에 인쇄하고 각 건너뛴 항목의 이유가 따릅니다. stderr를 리디렉션하거나 CI 러너 또는 SDK 호스트와 같은 프로그램이 이를 캡처하면 Claude Code는 경고를 인쇄하지 않고 건너뛴 항목만 `mcp_server_errors` 필드에 보고합니다. 경고는 Claude Code v2.1.219 이상이 필요합니다.

<h4 id="track-plugin-installs">
  플러그인 설치 추적
</h4>

[`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ko/env-vars)이 설정되면 Claude Code는 첫 번째 턴 전에 마켓플레이스 플러그인이 설치되는 동안 `system/plugin_install` 이벤트를 내보냅니다. 이를 사용하여 자신의 UI에서 설치 진행 상황을 표시합니다.

| 필드           | 유형                                                      | 설명                                                                                 |
| ------------ | ------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `type`       | `"system"`                                              | 메시지 유형                                                                             |
| `subtype`    | `"plugin_install"`                                      | 이를 플러그인 설치 이벤트로 식별                                                                 |
| `status`     | `"started"`, `"installed"`, `"failed"` 또는 `"completed"` | `started` 및 `completed`는 전체 설치를 괄호로 묶습니다; `installed` 및 `failed`는 개별 마켓플레이스를 보고합니다 |
| `name`       | 문자열, 선택 사항                                              | 마켓플레이스 이름, `installed` 및 `failed`에 존재                                              |
| `error`      | 문자열, 선택 사항                                              | 실패 메시지, `failed`에 존재                                                               |
| `uuid`       | 문자열                                                     | 고유 이벤트 식별자                                                                         |
| `session_id` | 문자열                                                     | 이벤트가 속한 세션                                                                         |

<h3 id="auto-approve-tools">
  도구 자동 승인
</h3>

`--allowedTools`를 사용하여 Claude가 특정 도구를 프롬프트 없이 사용하도록 허용합니다. 이 예제는 테스트 스위트를 실행하고 실패를 수정하며 Claude가 권한을 요청하지 않고 Bash 명령을 실행하고 파일을 읽고 편집하도록 허용합니다:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

전체 세션에 대한 기준선을 설정하려면 개별 도구를 나열하는 대신 [권한 모드](/docs/ko/permission-modes)를 전달합니다. `-p`의 경우 [기본 시작 권한 모드](/docs/ko/permission-modes#which-mode-a-session-starts-in)는 모든 플랜에서 Manual이므로 원하는 권한 모드를 전달합니다:

* **`auto`**: `--permission-mode auto`를 전달하여 분류기가 대부분의 작업을 검토하도록 합니다
* **`dontAsk`**: Claude Code는 그렇지 않으면 프롬프트할 모든 호출을 거부하며, 이는 잠긴 CI 실행에 유용합니다. Manual 모드에서 승인이 필요하지 않은 작업(예: 작업 디렉토리의 파일 읽기 및 [읽기 전용 명령 집합](/docs/ko/permissions#read-only-commands))과 `--allowedTools` 항목 또는 `permissions.allow` 규칙이 적용되는 작업은 여전히 실행됩니다. `AskUserQuestion`, 조직이 [`ask`](/docs/ko/mcp#organization-controls-on-connector-tools)로 설정한 커넥터 도구 및 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구는 허용 규칙이 일치해도 거부됩니다
* **`acceptEdits`**: Claude는 프롬프트 없이 파일을 작성하고 Claude Code는 `mkdir`, `touch`, `mv` 및 `cp`와 같은 일반적인 파일 시스템 명령을 자동 승인합니다. [모드가 자동 승인하지 않는 작업](/docs/ko/permission-modes#actions-no-mode-auto-approves)은 여전히 적용됩니다. 읽기 전용 명령 집합을 제외하고 다른 셸 명령 및 네트워크 요청은 여전히 `--allowedTools` 항목 또는 `permissions.allow` 규칙이 필요합니다. [`acceptEdits`가 자동 승인하는 것](/docs/ko/permission-modes#auto-approve-file-edits-with-acceptedits-mode)의 전체 목록을 참조하십시오

이 예제는 `acceptEdits`를 기준선으로 하여 린트 수정을 적용합니다:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="turn-off-permission-prompts-in-unattended-runs">
  무인 실행에서 권한 프롬프트 끄기
</h3>

권한 프롬프트에 응답할 수 있는 사람이 없을 때(예: 예약된 작업) `--permission-prompts none`을 전달합니다. 플래그는 실행에 권한 호스트가 있을 때 가장 중요합니다: [`canUseTool` 콜백](/docs/ko/agent-sdk/user-input)이 있는 Agent SDK 앱 또는 [`--permission-prompt-tool`](/docs/ko/cli-reference#cli-flags)로 전달하는 MCP 도구. 플래그 없이 실행은 해당 호스트가 각 권한 요청에 응답할 때까지 기다립니다.

플래그를 사용하면 실행은 호스트를 참조하거나 기다리지 않습니다. 프롬프트할 모든 것은 `PermissionRequest` 훅이 허용하지 않으면 거부되고, Claude는 아무도 요청을 승인할 수 없으며 재시도하지 않도록 지시받고, 실행이 계속됩니다. 호스트가 없는 `-p` 실행에서 이러한 요청은 어느 쪽이든 거부되며 플래그는 또한 Claude에게 재시도하지 않도록 지시합니다. 권한 규칙, [`PermissionRequest` 훅](/docs/ko/hooks#permissionrequest) 및 설정한 권한 모드는 여전히 모든 호출을 먼저 결정합니다; Claude Code는 다른 것이 해결하지 않는 요청만 거부합니다.

이 예제는 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서 무인 작업을 실행합니다. 분류기는 평소대로 각 작업을 검토하고 Claude Code는 프롬프트로 돌아갔을 모든 것을 거부합니다:

```bash theme={null}
claude -p "Update the dependency pins and run the tests" --permission-mode auto --permission-prompts none
```

`--permission-prompts none`을 사용하면 Claude Code는 사람의 답변이 필요한 도구(예: [`AskUserQuestion`](/docs/ko/tools-reference#askuserquestion-tool-behavior))를 제거하므로 Claude는 이를 호출할 수 없습니다. [`Elicitation` 훅](/docs/ko/hooks#elicitation)이 응답하지 않는 모든 [MCP 유도 요청](/docs/ko/mcp#respond-to-mcp-elicitation-requests)은 취소됩니다.

`--output-format stream-json`을 사용하면 거부는 `permission_denied` 시스템 메시지로 나타나고 최종 결과 메시지는 이들을 `permission_denials`에 나열합니다.

<Note>
  `--permission-prompts` 플래그는 Claude Code v2.1.259 이상이 필요합니다. 이전 버전은 알 수 없는 옵션 오류로 거부합니다.
</Note>

<h3 id="create-a-commit">
  커밋 생성
</h3>

이 예제는 스테이징된 변경 사항을 검토하고 적절한 메시지로 커밋을 생성합니다:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

`--allowedTools` 플래그는 [권한 규칙 구문](/docs/ko/settings-reference#permission-rule-syntax)을 사용합니다. 뒤의 ` *`는 접두사 일치를 활성화하므로 `Bash(git diff *)`는 `git diff`로 시작하는 모든 명령을 허용합니다. 공백은 중요합니다: 없으면 `Bash(git diff*)`는 `git diff-index`도 일치합니다.

<Note>
  명령 지원은 `-p` 모드에서 다릅니다:

  * 사용자 호출 [스킬](/docs/ko/skills) 및 사용자 정의 명령이 작동합니다. 프롬프트 문자열에 `/skill-name`을 포함하고 Claude Code는 실행하기 전에 이를 확장합니다.
  * 터미널 인터페이스에서만 실행되는 `/login`과 같은 기본 제공 명령은 사용할 수 없습니다.
  *

  `/model`, `/effort`, `/fast`, `/color` 및 `/rename`은 값을 인수로 허용합니다(예: `/model sonnet`). `/mcp`는 인수 없이 서버 상태의 텍스트 요약을 인쇄합니다. 이러한 형식은 Claude Code v2.1.205 이상이 필요하며 각 명령의 [가용성 참고 사항](/docs/ko/commands#all-commands)을 따릅니다.

  * 설정을 변경하려면 `/config`에 `key=value`를 전달합니다(예: `/config thinking=false`).
  *

  `/output-style <style>`은 [출력 스타일](/docs/ko/output-styles)을 전환하고 `/output-style`만으로 이들을 나열합니다. Claude Code v2.1.269 이상이 필요합니다.
</Note>

<h3 id="customize-the-system-prompt">
  시스템 프롬프트 사용자 정의
</h3>

`--append-system-prompt`를 사용하여 Claude Code의 기본 동작을 유지하면서 지침을 추가합니다. 이 예제는 PR diff를 Claude로 파이프하고 보안 취약점을 검토하도록 지시합니다. 셸 스크립트로 저장합니다(예: `review.sh`):

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

스크립트에서 `"$1"`은 명령줄에서 전달하는 첫 번째 인수를 나타냅니다. `bash review.sh 123`을 실행하면 셸은 `"$1"`을 `123`으로 바꾸므로 스크립트는 PR 123의 diff를 가져옵니다. Claude Code는 검토를 JSON으로 인쇄하며 텍스트는 `result` 필드에 있습니다.

더 많은 옵션(기본 프롬프트를 완전히 바꾸는 `--system-prompt` 포함)은 [시스템 프롬프트 플래그](/docs/ko/cli-reference#system-prompt-flags)를 참조하십시오.

<h3 id="continue-conversations">
  대화 계속
</h3>

`--continue`를 사용하여 가장 최근 대화를 계속하거나 `--resume`을 세션 ID와 함께 사용하여 특정 대화를 계속합니다. Claude Code v2.1.257 이상에서 `--continue`를 전달하면 Claude Code는 완료되었지만 여전히 실행 중이 아닌 [백그라운드 세션](/docs/ko/sessions#resume-a-session)을 엽니다. 이 예제는 검토를 실행한 다음 후속 프롬프트를 보냅니다:

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

두 명령을 다른 디렉토리에서 실행할 수 있습니다: Claude Code는 [세션 ID로 세션을 찾습니다](/docs/ko/sessions#resume-a-session) 이 머신의 모든 프로젝트에서. v2.1.223 이전에는 Claude Code가 현재 프로젝트 디렉토리 및 git worktrees에서만 ID를 찾았으므로 두 명령을 같은 디렉토리에서 실행해야 했습니다.

세션 ID 대신 `--resume`에 세션의 `.jsonl` [트랜스크립트 파일](/docs/ko/sessions#where-transcripts-are-stored)의 절대 경로를 전달할 수 있으며 Claude Code는 해당 파일에 저장된 대화를 계속합니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [Agent SDK 빠른 시작](/docs/ko/agent-sdk/quickstart): Python 또는 TypeScript로 첫 번째 에이전트 구축
* [CLI 참조](/docs/ko/cli-reference): 모든 CLI 플래그 및 옵션
* [GitHub Actions](/docs/ko/github-actions): GitHub 워크플로우에서 Agent SDK 사용
* [GitLab CI/CD](/docs/ko/gitlab-ci-cd): GitLab 파이프라인에서 Agent SDK 사용
