> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hooks 참조

> Claude Code hook 이벤트, 구성 스키마, JSON 입출력 형식, 종료 코드, 비동기 hook, HTTP hook, 프롬프트 hook, MCP 도구 hook에 대한 참조입니다.

<Tip>
  예제가 포함된 빠른 시작 가이드는 [hook으로 워크플로우 자동화](/docs/ko/hooks-guide)를 참조하세요.
</Tip>

Hook은 Claude Code의 수명 주기에서 특정 지점에 자동으로 실행되는 사용자 정의 셸 명령, HTTP 엔드포인트 또는 LLM 프롬프트입니다. 이 참조를 사용하여 이벤트 스키마, 구성 옵션, JSON 입출력 형식, 비동기 hook, HTTP hook, MCP 도구 hook과 같은 고급 기능을 조회할 수 있습니다. 처음으로 hook을 설정하는 경우 대신 [가이드](/docs/ko/hooks-guide)부터 시작하세요.

<h2 id="hook-lifecycle">
  Hook 수명 주기
</h2>

Hook은 Claude Code 세션 중 특정 지점에서 실행됩니다. 이벤트가 발생하고 matcher가 일치하면 Claude Code는 이벤트에 대한 JSON 컨텍스트를 hook 핸들러에 전달합니다. 명령 hook의 경우 입력은 stdin에 도착합니다. HTTP hook의 경우 POST 요청 본문으로 도착합니다. 그러면 핸들러는 입력을 검사하고 조치를 취한 후 선택적으로 결정을 반환할 수 있습니다.

이벤트는 세 가지 주기로 발생합니다:

* 세션당 한 번: `SessionStart` 및 `SessionEnd`
* 턴당 한 번: `UserPromptSubmit`, `Stop` 및 `StopFailure`
* 에이전트 루프 내의 모든 도구 호출에서: `PreToolUse` 및 `PostToolUse`

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=81b9256c1bbe8832553485f5d9e9c746" alt="선택적 Setup에서 SessionStart로 시작하여 턴당 루프(UserPromptSubmit, 슬래시 명령에 대한 UserPromptExpansion, 중첩된 에이전트 루프(PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), Stop 또는 StopFailure), TeammateIdle, PreCompact, PostCompact, SessionEnd를 거쳐 진행되는 hook 수명 주기 다이어그램. Elicitation 및 ElicitationResult는 MCP 도구 실행 내에 중첩되고, PermissionDenied는 PermissionRequest의 부분 분기(자동 모드 거부용), WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged는 독립적인 비동기 이벤트이며, MessageDisplay는 어시스턴트 메시지 텍스트가 스트리밍되는 동안 실행되는 표시 전용 이벤트입니다" width="520" height="1336" data-path="images/hooks-lifecycle.svg" />
  </Frame>
</div>

아래 표는 각 이벤트가 언제 발생하는지 요약합니다. [Hook 이벤트](#hook-events) 섹션에서는 각 이벤트의 전체 입력 스키마와 결정 제어 옵션을 문서화합니다.

| Event                 | When it fires                                                                                                                                                                                                                                         |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | When a session begins or resumes                                                                                                                                                                                                                      |
| `Setup`               | When you start Claude Code with `--init-only`, or with `--init` or `--maintenance` in `-p` mode. For one-time preparation in CI or scripts                                                                                                            |
| `UserPromptSubmit`    | When you submit a prompt, before Claude processes it                                                                                                                                                                                                  |
| `UserPromptExpansion` | When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion                                                                                                                                                    |
| `PreToolUse`          | Before a tool call executes. Can block it                                                                                                                                                                                                             |
| `PermissionRequest`   | When a tool call needs a permission decision                                                                                                                                                                                                          |
| `PermissionDenied`    | When auto mode denies a tool call, including denials without a classifier verdict. Use JSON `hookSpecificOutput.retry: true` to tell the model it may retry the denied tool call. Claude Code ignores `retry` when the classifier produced no verdict |
| `PostToolUse`         | After a tool call succeeds                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | After a tool call fails                                                                                                                                                                                                                               |
| `PostToolBatch`       | After a full batch of parallel tool calls resolves, before the next model call                                                                                                                                                                        |
| `Notification`        | When Claude Code sends a notification                                                                                                                                                                                                                 |
| `MessageDisplay`      | While assistant message text is displayed                                                                                                                                                                                                             |
| `SubagentStart`       | When a subagent is spawned                                                                                                                                                                                                                            |
| `SubagentStop`        | When a subagent finishes                                                                                                                                                                                                                              |
| `TaskCreated`         | When a task is being created via `TaskCreate`                                                                                                                                                                                                         |
| `TaskCompleted`       | When a task is being marked as completed                                                                                                                                                                                                              |
| `Stop`                | When Claude finishes responding                                                                                                                                                                                                                       |
| `StopFailure`         | When the turn ends due to an API error                                                                                                                                                                                                                |
| `TeammateIdle`        | When an [agent team](/docs/en/agent-teams) teammate is about to go idle                                                                                                                                                                                    |
| `InstructionsLoaded`  | When a CLAUDE.md or `.claude/rules/*.md` file is loaded into context. Fires at session start and when files are lazily loaded during a session                                                                                                        |
| `ConfigChange`        | When a configuration file changes during a session                                                                                                                                                                                                    |
| `CwdChanged`          | When the working directory changes, for example when Claude executes a `cd` command. Useful for reactive environment management with tools like direnv                                                                                                |
| `DirectoryAdded`      | When a working directory is added mid-session via `/add-dir` or the SDK `register_repo_root` control request                                                                                                                                          |
| `FileChanged`         | When a watched file changes on disk. The `matcher` field specifies which filenames to watch                                                                                                                                                           |
| `WorktreeCreate`      | When a worktree is being created via `--worktree`, `isolation: "worktree"`, or for a background session. Replaces default git behavior                                                                                                                |
| `WorktreeRemove`      | When a worktree is being removed at session exit, when a subagent finishes, or when you delete a background session                                                                                                                                   |
| `PreCompact`          | Before context compaction                                                                                                                                                                                                                             |
| `PostCompact`         | After context compaction completes                                                                                                                                                                                                                    |
| `PreModelSwitch`      | Before Claude Code applies a model switch that you or a client requested. Can block the switch                                                                                                                                                        |
| `PostModelSwitch`     | After the session's model changes, including changes Claude Code makes on its own, such as restoring the model when you resume a session                                                                                                              |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

<h3 id="how-a-hook-resolves">
  Hook이 어떻게 해결되는지
</h3>

이러한 부분들이 어떻게 함께 작동하는지 보려면 파괴적인 셸 명령을 차단하는 이 `PreToolUse` hook을 고려하세요. `matcher`는 Bash 도구 호출로 좁혀지고 `if` 조건은 `rm *`과 일치하는 Bash 부명령으로 더 좁혀지므로 `block-rm.sh`는 두 필터가 모두 일치할 때만 생성됩니다:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(rm *)",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

스크립트는 stdin에서 JSON 입력을 읽고 명령을 추출한 후 `rm -rf`를 포함하면 `permissionDecision`을 `"deny"`로 반환합니다:

```bash theme={null}
#!/bin/bash
# .claude/hooks/block-rm.sh
COMMAND=$(jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q 'rm -rf'; then
  jq -n '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: "Destructive command blocked by hook"
    }
  }'
else
  exit 0  # no decision; normal permission flow applies
fi
```

이제 Claude Code가 `Bash "rm -rf /tmp/build"`를 실행하기로 결정했다고 가정합니다. 다음은 발생하는 일입니다:

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" alt="Hook 해결 흐름: PreToolUse 이벤트 발생, matcher가 Bash 일치 확인, if 조건이 Bash(rm *) 일치 확인, hook 핸들러 실행, 결과가 Claude Code로 반환" width="930" height="270" data-path="images/hook-resolution.svg" />
</Frame>

<Steps>
  <Step title="이벤트 발생">
    `PreToolUse` 이벤트가 발생합니다. Claude Code는 도구 입력을 stdin의 hook에 JSON으로 전송합니다:

    ```json theme={null}
    { "tool_name": "Bash", "tool_input": { "command": "rm -rf /tmp/build" }, ... }
    ```
  </Step>

  <Step title="Matcher 확인">
    matcher `"Bash"`가 도구 이름과 일치하므로 이 hook 그룹이 활성화됩니다. matcher를 생략하거나 `"*"`를 사용하면 이벤트의 모든 발생에서 그룹이 활성화됩니다.
  </Step>

  <Step title="If 조건 확인">
    `if` 조건 `"Bash(rm *)"`은 `rm -rf /tmp/build`가 `rm *`과 일치하는 부명령이므로 일치하여 이 핸들러가 생성됩니다. 명령이 `npm test`였다면 `if` 검사가 실패하고 `block-rm.sh`는 절대 실행되지 않아 프로세스 생성 오버헤드를 피합니다. `if` 필드는 선택 사항입니다. 없으면 일치한 그룹의 모든 핸들러가 실행됩니다.
  </Step>

  <Step title="Hook 핸들러 실행">
    스크립트는 전체 명령을 검사하고 `rm -rf`를 찾으므로 stdout에 결정을 인쇄합니다:

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Destructive command blocked by hook"
      }
    }
    ```

    명령이 `rm file.txt`와 같은 더 안전한 `rm` 변형이었다면 스크립트는 대신 `exit 0`을 실행합니다. 출력이 없는 종료 코드 0은 hook이 보고할 결정이 없다는 의미이므로 도구 호출은 일반적인 [권한 흐름](/docs/ko/permissions)을 통해 계속됩니다. hook은 호출을 거부할 수 있지만 침묵을 유지하는 것은 이를 승인하지 않습니다.
  </Step>

  <Step title="Claude Code가 결과에 따라 행동">
    Claude Code는 JSON 결정을 읽고 도구 호출을 차단하며 Claude에 이유를 표시합니다.
  </Step>
</Steps>

아래 [구성](#configuration) 섹션에서는 전체 스키마를 문서화하고, 각 [hook 이벤트](#hook-events) 섹션에서는 명령이 받는 입력과 반환할 수 있는 출력을 문서화합니다.

<h2 id="configuration">
  구성
</h2>

Hook은 JSON 설정 파일에서 정의됩니다. 구성에는 세 가지 중첩 수준이 있습니다:

1. 응답할 [hook 이벤트](#hook-events)를 선택합니다 (예: `PreToolUse` 또는 `Stop`)
2. 발생 시기를 필터링할 [matcher 그룹](#matcher-patterns)을 추가합니다 (예: "Bash 도구에만")
3. 일치할 때 실행할 하나 이상의 [hook 핸들러](#hook-handler-fields)를 정의합니다

주석이 달린 예제를 포함한 완전한 설명은 위의 [Hook이 어떻게 해결되는지](#how-a-hook-resolves)를 참조하세요.

<Note>
  이 페이지는 각 수준에 대해 특정 용어를 사용합니다: 수명 주기 지점에 대해 **hook 이벤트**, 필터에 대해 **matcher 그룹**, 실행되는 셸 명령, HTTP 엔드포인트, MCP 도구, 프롬프트 또는 에이전트에 대해 **hook 핸들러**. "Hook"은 일반 기능을 나타냅니다.
</Note>

<h3 id="hook-locations">
  Hook 위치
</h3>

hook을 정의하는 위치는 그 범위를 결정합니다:

| 위치                                                         | 범위                | 공유 가능             |
| :--------------------------------------------------------- | :---------------- | :---------------- |
| `~/.claude/settings.json`                                  | 모든 프로젝트           | 아니오, 머신에 로컬       |
| `.claude/settings.json`                                    | 단일 프로젝트           | 예, 리포지토리에 커밋 가능   |
| `.claude/settings.local.json`                              | 단일 프로젝트           | 아니오, gitignored   |
| 관리형 정책 설정                                                  | 조직 전체             | 예, 관리자 제어         |
| [Plugin](/docs/ko/plugins) `hooks/hooks.json`                   | plugin이 활성화되었을 때  | 예, plugin과 함께 번들됨 |
| [Skill](/docs/ko/skills) 또는 [agent](/docs/ko/sub-agents) frontmatter | 컴포넌트가 활성화되어 있는 동안 | 예, 컴포넌트 파일에서 정의됨  |

설정 파일 해결에 대한 자세한 내용은 [설정](/docs/ko/settings)을 참조하세요.

엔터프라이즈 관리자는 `allowManagedHooksOnly`를 사용하여 사용자, 프로젝트 및 plugin hook을 차단할 수 있습니다. 관리형 설정 `enabledPlugins`에서 강제 활성화된 plugin의 hook은 면제되므로 관리자는 조직 마켓플레이스를 통해 검증된 hook을 배포할 수 있습니다. [Hook 구성](/docs/ko/settings#hook-configuration)을 참조하세요.

<h3 id="matcher-patterns">
  Matcher 패턴
</h3>

`matcher` 필드는 hook이 발생할 때를 필터링합니다. matcher가 평가되는 방식은 포함된 문자에 따라 다릅니다:

| Matcher 값                           | 평가 대상                                                 | 예제                                                                                                        |
| :---------------------------------- | :---------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- |
| `"*"`, `""` 또는 생략됨                  | 모두 일치                                                 | 이벤트의 모든 발생에서 발생                                                                                           |
| 문자, 숫자, `_`, `-`, 공백, `,`, `\|`만 포함 | 정확한 문자열 또는 `\|` 또는 `,`로 구분된 정확한 문자열 목록 (선택적 주변 공백 포함) | `Bash`는 Bash 도구만 일치; `Edit\|Write` 및 `Edit, Write`는 각각 두 도구 중 하나와 정확히 일치; `code-reviewer`는 해당 에이전트 유형만 일치 |
| 다른 문자 포함                            | JavaScript 정규식, 앵커 없음                                 | `^Notebook`은 Notebook으로 시작하는 모든 도구와 일치; `mcp__memory__.*`는 `memory` 서버의 모든 도구와 일치                         |

JavaScript의 `RegExp.prototype.test`로 테스트되는 정규식 경로의 matcher는 값의 어디든지 일치하면 성공합니다. `Edit.*`는 `Edit`과 `NotebookEdit` 모두와 일치합니다. 전체 문자열 일치가 필요할 때는 `^Edit$`처럼 패턴을 `^` 및 `$`로 감싸세요.

쉼표 구분자와 주변 공백 허용은 Claude Code v2.1.191 이상이 필요합니다.

정확한 일치 집합의 하이픈은 Claude Code v2.1.195 이상이 필요합니다. 이전 버전에서는 `code-reviewer`와 같은 하이픈이 있는 이름이 앵커 없는 정규식으로 평가되므로 `senior-code-reviewer`에도 발생합니다. 해당 버전에서 해당 이름만 일치하도록 `^code-reviewer$`로 앵커하세요.

`FileChanged` 및 `StopFailure`는 문자, 숫자, `_`, `|`만 포함하는 더 좁은 정확한 일치 집합을 사용합니다. matcher에 하이픈, 공백 또는 쉼표가 있으면 이 두 이벤트에 대해 정규식 경로에 유지되고 `|`만 대안을 구분합니다. 다음 표에서 matcher 지원이 있는 다른 모든 이벤트는 `|` 또는 `,`를 허용합니다.

`FileChanged` 이벤트는 감시 목록을 구축할 때 이러한 규칙을 따르지 않습니다. [FileChanged](#filechanged)를 참조하세요.

각 이벤트 유형은 다른 필드에서 일치합니다:

| 이벤트                                                                                                                                               | Matcher가 필터링하는 것                             | 예제 matcher 값                                                                                                                                                                        |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | 도구 이름                                        | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                    | 세션이 시작된 방식                                   | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                           | 설정을 트리거한 CLI 플래그                             | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                      | 세션이 종료된 이유                                   | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                    | 알림 유형                                        | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                   | 에이전트 유형                                      | `general-purpose`, `Explore`, `Plan`, 사용자 정의 에이전트 이름 또는 `^my-plugin:reviewer$`와 같은 plugin 범위 이름                                                                                     |
| `PreCompact`, `PostCompact`                                                                                                                       | 압축을 트리거한 것                                   | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                    | 에이전트 유형                                      | `SubagentStart`와 동일한 값                                                                                                                                                              |
| `ConfigChange`                                                                                                                                    | 구성 소스                                        | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `CwdChanged`                                                                                                                                      | matcher 지원 없음                                | 모든 디렉토리 변경에서 항상 발생                                                                                                                                                                  |
| `FileChanged`                                                                                                                                     | 감시할 리터럴 파일명 ([FileChanged](#filechanged) 참조) | `.envrc\|.env`                                                                                                                                                                      |
| `StopFailure`                                                                                                                                     | 오류 유형                                        | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                              | 로드 이유                                        | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `UserPromptExpansion`                                                                                                                             | 명령 이름                                        | skill 또는 명령 이름                                                                                                                                                                      |
| `Elicitation`                                                                                                                                     | MCP 서버 이름                                    | 구성된 MCP 서버 이름                                                                                                                                                                       |
| `ElicitationResult`                                                                                                                               | MCP 서버 이름                                    | `Elicitation`과 동일한 값                                                                                                                                                                |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | matcher 지원 없음                                | 모든 발생에서 항상 발생                                                                                                                                                                       |

matcher는 Claude Code가 stdin의 hook에 전송하는 [JSON 입력](#hook-input-and-output)의 필드에 대해 실행됩니다. 도구 이벤트의 경우 해당 필드는 `tool_name`입니다. 각 [hook 이벤트](#hook-events) 섹션에서는 해당 이벤트의 전체 matcher 값 집합과 입력 스키마를 나열합니다.

이 예제는 Claude가 파일을 쓰거나 편집할 때만 linting 스크립트를 실행합니다:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/lint-check.sh"
          }
        ]
      }
    ]
  }
}
```

`UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay`, `CwdChanged`는 matcher를 지원하지 않으며 모든 발생에서 항상 발생합니다. 이러한 이벤트에 `matcher` 필드를 추가하면 자동으로 무시됩니다.

도구 이벤트의 경우 개별 hook 핸들러에서 [`if` 필드](#common-fields)를 설정하여 더 좁게 필터링할 수 있습니다. `if`는 [권한 규칙 구문](/docs/ko/permissions)을 사용하여 도구 이름과 인수를 함께 일치시키므로 `"Bash(git *)"` 는 `git *` 패턴과 일치하는 모든 하위 명령에서 실행되고 `"Edit(*.ts)"`는 TypeScript 파일에만 실행됩니다.

<h4 id="match-mcp-tools">
  MCP 도구 일치
</h4>

[MCP](/docs/ko/mcp) 서버 도구는 도구 이벤트 (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`)에서 일반 도구로 나타나므로 다른 도구 이름과 동일한 방식으로 일치시킬 수 있습니다.

MCP 도구는 `mcp__<server>__<tool>` 명명 패턴을 따릅니다. 예를 들어:

* `mcp__memory__create_entities`: Memory 서버의 create entities 도구
* `mcp__filesystem__read_file`: Filesystem 서버의 read file 도구
* `mcp__github__search_repositories`: GitHub 서버의 search 도구

서버의 모든 도구와 일치하려면 서버 접두사에 `.*`를 추가합니다. `.*`는 필수입니다: `mcp__memory`와 같은 matcher는 문자와 밑줄만 포함하므로 정확한 문자열로 비교되고 도구와 일치하지 않습니다.

* `mcp__memory__.*`는 `memory` 서버의 모든 도구와 일치합니다
* `mcp__brave-search__.*`는 이름에 하이픈이 있는 서버의 모든 도구와 일치합니다
* `mcp__.*__write.*`는 모든 서버의 이름이 `write`로 시작하는 모든 도구와 일치합니다

정확한 일치 집합의 하이픈은 Claude Code v2.1.195 이상이 필요합니다. 이전 버전에서는 `mcp__brave-search`와 같은 bare 하이픈이 있는 접두사가 앵커 없는 정규식으로 평가되고 해당 서버의 모든 도구와 일치합니다. `mcp__brave-search__.*` 형식은 모든 버전에서 작동합니다.

[plugin 번들 MCP 서버](/docs/ko/mcp#plugin-provided-mcp-servers)의 도구는 plugin 이름을 포함하는 범위가 지정된 서버 세그먼트를 사용합니다: `mcp__plugin_<plugin-name>_<server-name>__<tool>`. bare 서버 키에 대해 작성된 matcher는 이러한 도구에 대해 발생하지 않습니다. `my-plugin`이라는 plugin이 `db` 키 아래에 서버를 번들하는 경우 `query` 도구는 `mcp__plugin_my-plugin_db__query`로 나타나므로 해당 서버의 모든 도구에 대한 matcher는 `mcp__plugin_my-plugin_db__.*`입니다. 핸들러의 [`if` 필드](#common-fields)에서 동일한 범위가 지정된 도구 이름을 사용합니다. 범위가 지정된 이름이 구축되는 방식에 대해서는 [Plugin 제공 MCP 서버](/docs/ko/mcp#plugin-provided-mcp-servers)를 참조하세요.

이 예제는 모든 memory 서버 작업을 기록하고 모든 MCP 서버의 쓰기 작업을 검증합니다:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "mcp__memory__.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Memory operation initiated' >> ~/mcp-operations.log"
          }
        ]
      },
      {
        "matcher": "mcp__.*__write.*",
        "hooks": [
          {
            "type": "command",
            "command": "/home/user/scripts/validate-mcp-write.py"
          }
        ]
      }
    ]
  }
}
```

<h3 id="hook-handler-fields">
  Hook 핸들러 필드
</h3>

내부 `hooks` 배열의 각 객체는 hook 핸들러입니다: matcher가 일치할 때 실행되는 셸 명령, HTTP 엔드포인트, MCP 도구, LLM 프롬프트 또는 에이전트입니다. 다섯 가지 유형이 있습니다:

* **[명령 hook](#command-hook-fields)** (`type: "command"`): 셸 명령을 실행합니다. 스크립트는 이벤트의 [JSON 입력](#hook-input-and-output)을 stdin에서 받고 종료 코드와 stdout을 통해 결과를 다시 전달합니다.
* **[HTTP hook](#http-hook-fields)** (`type: "http"`): 이벤트의 JSON 입력을 HTTP POST 요청으로 URL에 전송합니다. 엔드포인트는 명령 hook과 동일한 [JSON 출력 형식](#json-output)을 사용하여 응답 본문을 통해 결과를 다시 전달합니다.
* **[MCP 도구 hook](#mcp-tool-hook-fields)** (`type: "mcp_tool"`): 이미 연결된 [MCP 서버](/docs/ko/mcp)의 도구를 호출합니다. 도구의 텍스트 출력은 명령 hook stdout처럼 처리됩니다.
* **[프롬프트 hook](#prompt-and-agent-hook-fields)** (`type: "prompt"`): Claude 모델에 단일 턴 평가를 위한 프롬프트를 전송합니다. 모델은 yes/no 결정을 JSON으로 반환합니다. [프롬프트 기반 hook](#prompt-based-hooks)을 참조하세요.
* **[에이전트 hook](#prompt-and-agent-hook-fields)** (`type: "agent"`): Read, Grep, Glob과 같은 도구를 사용하여 결정을 반환하기 전에 조건을 확인할 수 있는 subagent를 생성합니다. 에이전트 hook은 실험적이며 변경될 수 있습니다. [에이전트 기반 hook](#agent-based-hooks)을 참조하세요.

일치하는 모든 hook은 병렬로 실행되며 동일한 핸들러는 자동으로 중복 제거됩니다. 명령 hook은 명령 문자열과 `args`로 중복 제거되고 HTTP hook은 URL로 중복 제거됩니다.

핸들러는 현재 디렉토리에서 Claude Code의 환경으로 실행됩니다. `$CLAUDE_CODE_REMOTE` 환경 변수는 원격 웹 환경에서 `"true"`로 설정되고 로컬 CLI에서는 설정되지 않습니다. v2.1.199부터 [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/ko/env-vars)는 로컬 세션이 활성 Remote Control 연결을 가지고 있는 동안 [Remote Control](/docs/ko/remote-control) 세션 ID로 설정됩니다.

<h4 id="common-fields">
  공통 필드
</h4>

이러한 필드는 모든 hook 유형에 적용됩니다:

| 필드              | 필수  | 설명                                                                                                                                                                                                                                                                                                                                                                                              |
| :-------------- | :-- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | 예   | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"` 또는 `"agent"`                                                                                                                                                                                                                                                                                                                                    |
| `if`            | 아니오 | `"Bash(git *)"` 또는 `"Edit(*.ts)"`와 같은 권한 규칙 구문을 사용하여 이 hook이 실행될 때를 필터링합니다. hook 명령은 도구 호출이 패턴과 일치할 때만 실행됩니다. [Bash 일치 테이블](#bash-if-matching) 아래에서 Bash 패턴이 하위 명령, `$()`, 백틱에 대해 어떻게 평가되는지 확인하세요. 도구 이벤트에서만 평가됩니다: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`. 다른 이벤트에서는 `if`가 설정된 hook이 절대 실행되지 않습니다. [권한 규칙](/docs/ko/permissions)과 동일한 구문을 사용합니다 |
| `timeout`       | 아니오 | 취소하기 전 초 단위. 기본값: `command`, `http`, `mcp_tool`의 경우 600; `prompt`의 경우 30; `agent`의 경우 60. [`UserPromptSubmit`](#userpromptsubmit)은 `command`, `http`, `mcp_tool`의 기본값을 30으로 낮추고 [`MessageDisplay`](#messagedisplay)는 10으로 낮춥니다                                                                                                                                                                  |
| `statusMessage` | 아니오 | hook이 실행되는 동안 표시되는 사용자 정의 스피너 메시지                                                                                                                                                                                                                                                                                                                                                               |
| `once`          | 아니오 | `true`인 경우 세션당 한 번만 실행된 후 제거됩니다. [Skill 및 에이전트의 Hook](#hooks-in-skills-and-agents)에서 선언된 hook에만 적용됨; 설정 파일 및 에이전트 frontmatter에서는 무시됨                                                                                                                                                                                                                                                            |

`if` 필드는 정확히 하나의 권한 규칙을 보유합니다. 규칙을 결합하기 위한 `&&`, `||` 또는 목록 구문이 없습니다. 여러 조건을 적용하려면 각각에 대해 별도의 hook 핸들러를 정의합니다.

<span id="bash-if-matching" />Bash 패턴의 경우 hook 명령이 실행되는지 여부는 패턴의 형태와 Claude가 호출하는 Bash 명령에 따라 다릅니다. 선행 `VAR=value` 할당은 일치하기 전에 제거됩니다.

| `if` 패턴            | Bash 명령                | Hook 실행? | 이유                                                       |
| :----------------- | :--------------------- | :------- | :------------------------------------------------------- |
| `Bash(git *)`      | `FOO=bar git push`     | 예        | 선행 할당이 제거됨; `git push`가 일치                               |
| `Bash(git *)`      | `npm test && git push` | 예        | 각 하위 명령이 확인됨; `git push`가 일치                             |
| `Bash(rm *)`       | `echo $(rm -rf /)`     | 예        | `$()` 및 백틱 내의 명령이 확인됨; `rm -rf /`가 일치                    |
| `Bash(rm *)`       | `echo $(date)`         | 아니오      | 어떤 하위 명령도 `rm *`과 일치하지 않음                                |
| `Bash(git push *)` | `echo $(date)`         | 예        | 명령 이름보다 더 많이 지정하는 패턴은 `$()`, 백틱 또는 `$VAR`에서 어쨌든 hook을 실행 |

필터는 또한 Bash 명령을 구문 분석할 수 없을 때 열려 있으므로 패턴과 관계없이 hook을 실행합니다. `if` 필터는 최선의 노력이므로 hard allow 또는 deny를 적용하려면 hook 대신 [권한 시스템](/docs/ko/permissions)을 사용하세요.

<h4 id="command-hook-fields">
  명령 hook 필드
</h4>

[공통 필드](#common-fields) 외에도 명령 hook은 이러한 필드를 허용합니다:

| 필드            | 필수  | 설명                                                                                                                                                                                                                                                                              |
| :------------ | :-- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `command`     | 예   | 실행할 셸 명령. `args`와 함께 직접 생성할 실행 파일입니다. [Exec 형식 및 셸 형식](#exec-form-and-shell-form) 참조                                                                                                                                                                                            |
| `args`        | 아니오 | 인수 목록. 존재할 때 `command`는 실행 파일로 해결되고 `args`를 인수 벡터로 하여 직접 생성되며 셸이 관여하지 않습니다. [Exec 형식 및 셸 형식](#exec-form-and-shell-form) 참조                                                                                                                                                      |
| `async`       | 아니오 | `true`인 경우 차단하지 않고 백그라운드에서 실행됩니다. [백그라운드에서 hook 실행](#run-hooks-in-the-background) 참조                                                                                                                                                                                            |
| `asyncRewake` | 아니오 | `true`인 경우 백그라운드에서 실행되고 종료 코드 2에서 Claude를 깨웁니다. `async`를 의미합니다. hook의 stderr 또는 stderr이 비어 있으면 stdout이 Claude에 시스템 알림으로 표시되므로 장기 실행 백그라운드 실패에 반응할 수 있습니다                                                                                                                        |
| `shell`       | 아니오 | 이 hook에 사용할 셸. `"bash"` 또는 `"powershell"`을 허용합니다. 기본값은 `"bash"` 또는 Git Bash가 설치되지 않았을 때 Windows에서 `"powershell"`입니다. `"powershell"`을 설정하면 Windows에서 PowerShell을 통해 명령을 실행합니다. `CLAUDE_CODE_USE_POWERSHELL_TOOL`이 필요하지 않습니다. hook이 PowerShell을 직접 생성하기 때문입니다. `args`가 설정되면 무시됩니다 |

<a id="exec-form-and-shell-form" />

<h5 id="exec-form-and-shell-form">
  Exec 형식 및 셸 형식
</h5>

명령 hook은 `args`가 설정되면 exec 형식으로 실행되고 `args`가 생략되면 셸 형식으로 실행됩니다. hook이 [경로 자리 표시자](#reference-scripts-by-path)를 참조할 때마다 `args`를 설정하세요. 각 요소는 따옴표 없이 하나의 인수로 전달됩니다. 파이프 또는 `&&`와 같은 셸 기능이 필요하거나 두 가지 우려 사항이 모두 적용되지 않을 때 `args`를 생략합니다.

**Exec 형식**은 `args`가 있을 때 실행됩니다. Claude Code는 `command`를 `PATH`의 실행 파일로 해결하고 `args`를 인수 벡터로 하여 직접 생성합니다. 셸이 없으므로 각 `args` 요소는 작성된 그대로 정확히 하나의 인수이며 `${CLAUDE_PLUGIN_ROOT}`와 같은 경로 자리 표시자는 `command` 및 각 `args` 요소로 일반 문자열로 대체됩니다. 아포스트로피, `$`, 백틱과 같은 특수 문자는 해석할 셸이 없으므로 그대로 전달됩니다. 어떤 플랫폼에서도 셸 토큰화가 발생하지 않습니다.

**셸 형식**은 `args`가 없을 때 실행됩니다. `command` 문자열은 셸로 전달됩니다: macOS 및 Linux에서는 `sh -c`, Windows에서는 Git Bash, Git Bash가 설치되지 않았을 때는 PowerShell입니다. `shell` 필드를 설정하여 명시적으로 선택합니다. 셸은 문자열을 토큰화하고 변수를 확장하며 파이프, `&&`, 리다이렉트, 글로브를 해석합니다.

<Note>
  Windows에서 exec 형식은 `.exe`와 같은 실제 실행 파일로 해결되는 `command`를 필요로 합니다. npm, npx, eslint 및 기타 도구가 `node_modules/.bin`에 설치하는 `.cmd` 및 `.bat` shim은 실행 파일이 아니며 셸 없이 생성될 수 없습니다. exec 형식으로 실행하려면 기본 스크립트를 `node`로 직접 호출합니다. 예를 들어 `"command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/node_modules/eslint/bin/eslint.js"]`. `node` 더하기 스크립트 경로 패턴은 `node.exe`가 실제 바이너리이므로 모든 플랫폼에서 작동합니다. `.cmd` 또는 `.bat` shim을 이름으로 실행하려면 셸 형식을 사용합니다.
</Note>

이 예제는 plugin과 함께 번들된 Node 스크립트를 실행합니다. Exec 형식은 해결된 스크립트 경로를 따옴표 없이 하나의 인수로 전달합니다:

```json theme={null}
{
  "type": "command",
  "command": "node",
  "args": ["${CLAUDE_PLUGIN_ROOT}/scripts/format.js", "--fix"]
}
```

동등한 셸 형식은 공백이나 특수 문자가 있는 경로를 처리하기 위해 따옴표가 필요합니다:

```json theme={null}
{
  "type": "command",
  "command": "node \"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.js --fix"
}
```

두 형식 모두 동일한 [경로 자리 표시자](#reference-scripts-by-path)를 지원하며 생성된 프로세스에서 환경 변수 `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`로 내보내므로 스크립트는 시작 방식과 관계없이 `process.env.CLAUDE_PLUGIN_ROOT`를 읽을 수 있습니다. Plugin hook은 추가로 [`${user_config.*}`](/docs/ko/plugins-reference#user-configuration) 값을 대체합니다. exec 형식에서만: 값은 `command` 및 각 `args` 요소로 일반 문자열로 대체되므로 셸이 다시 구문 분석하지 않습니다.

`${user_config.*}`를 참조하는 셸 형식 plugin hook 명령은 대신 [오류](/docs/ko/errors#plugin-command-references-user-config)로 실패합니다. 셸 형식 hook에서 옵션 값을 사용하려면 `$CLAUDE_PLUGIN_OPTION_<KEY>` 환경 변수 (예: `webhook_url` 옵션의 경우 `$CLAUDE_PLUGIN_OPTION_WEBHOOK_URL`)를 읽거나 `args`를 설정하여 hook을 exec 형식으로 전환합니다. v2.1.207 이전에는 셸 형식 plugin hook 명령도 `${user_config.*}`를 대체했습니다.

<Note>
  Exec 형식에서 `command`는 실행 파일 이름 또는 경로만입니다. `command`가 경로 구분자가 없는 bare 이름이고 `args`와 함께 공백을 포함하면 Claude Code는 경고를 기록합니다. 생성이 실패하기 때문입니다: `node script.js`라는 이름의 실행 파일이 없습니다. 추가 토큰을 `args`로 이동합니다. `C:\Program Files\nodejs\node.exe`와 같은 공백이 있는 절대 경로는 단일 유효한 실행 파일이며 경고를 트리거하지 않습니다.
</Note>

<h4 id="http-hook-fields">
  HTTP hook 필드
</h4>

[공통 필드](#common-fields) 외에도 HTTP hook은 이러한 필드를 허용합니다:

| 필드               | 필수  | 설명                                                                                                             |
| :--------------- | :-- | :------------------------------------------------------------------------------------------------------------- |
| `url`            | 예   | POST 요청을 전송할 URL                                                                                               |
| `headers`        | 아니오 | 키-값 쌍으로 된 추가 HTTP 헤더. 값은 `$VAR_NAME` 또는 `${VAR_NAME}` 구문을 사용한 환경 변수 보간을 지원합니다. `allowedEnvVars`에 나열된 변수만 해결됩니다 |
| `allowedEnvVars` | 아니오 | 헤더 값으로 보간될 수 있는 환경 변수 이름 목록. 나열되지 않은 변수에 대한 참조는 빈 문자열로 바뀝니다. 환경 변수 보간이 작동하려면 필수입니다                             |

Claude Code는 hook의 [JSON 입력](#hook-input-and-output)을 `Content-Type: application/json`과 함께 POST 요청 본문으로 전송합니다. 응답 본문은 명령 hook과 동일한 [JSON 출력 형식](#json-output)을 사용합니다.

오류 처리는 명령 hook과 다릅니다: 2xx가 아닌 응답, 연결 실패, 시간 초과는 모두 실행을 계속하도록 허용하는 차단하지 않는 오류를 생성합니다. 도구 호출을 차단하거나 권한을 거부하려면 `decision: "block"` 또는 `hookSpecificOutput`이 `permissionDecision: "deny"`를 포함하는 JSON 본문이 있는 2xx 응답을 반환합니다.

이 예제는 `PreToolUse` 이벤트를 로컬 검증 서비스로 전송하고 `MY_TOKEN` 환경 변수의 토큰으로 인증합니다:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/pre-tool-use",
            "timeout": 30,
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

<h4 id="mcp-tool-hook-fields">
  MCP 도구 hook 필드
</h4>

[공통 필드](#common-fields) 외에도 MCP 도구 hook은 이러한 필드를 허용합니다:

| 필드       | 필수  | 설명                                                                                                                                                                                                                          |
| :------- | :-- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server` | 예   | 구성된 MCP 서버의 이름. [plugin 번들 서버](/docs/ko/mcp#plugin-provided-mcp-servers)의 경우 범위가 지정된 이름 `plugin:<plugin-name>:<server-name>` (예: `plugin:my-plugin:db`)이며 bare 서버 키가 아닙니다. 서버는 이미 연결되어 있어야 합니다. hook은 OAuth 또는 연결 흐름을 트리거하지 않습니다 |
| `tool`   | 예   | 해당 서버에서 호출할 도구의 이름                                                                                                                                                                                                          |
| `input`  | 아니오 | 도구에 전달되는 인수. 문자열 값은 hook의 [JSON 입력](#hook-input-and-output)에서 `${path}` 치환을 지원합니다 (예: `"${tool_input.file_path}"`)                                                                                                          |

도구의 텍스트 콘텐츠는 명령 hook stdout처럼 처리됩니다: 유효한 [JSON 출력](#json-output)으로 구문 분석되면 결정으로 처리되고, 그렇지 않으면 일반 텍스트로 표시됩니다. 명명된 서버가 연결되지 않았거나 도구가 `isError: true`를 반환하면 hook은 차단하지 않는 오류를 생성하고 실행이 계속됩니다.

MCP 도구 hook은 Claude Code가 MCP 서버에 연결한 후 모든 hook 이벤트에서 사용 가능합니다. `SessionStart` 및 `Setup`은 일반적으로 서버가 연결을 완료하기 전에 발생하므로 이러한 이벤트의 hook은 첫 실행 시 "not connected" 오류를 예상해야 합니다.

이 예제는 각 `Write` 또는 `Edit` 후에 `my_server` MCP 서버의 `security_scan` 도구를 호출하고 편집된 파일의 경로를 전달합니다:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "security_scan",
            "input": { "file_path": "${tool_input.file_path}" }
          }
        ]
      }
    ]
  }
}
```

<h4 id="prompt-and-agent-hook-fields">
  프롬프트 및 에이전트 hook 필드
</h4>

[공통 필드](#common-fields) 외에도 프롬프트 및 에이전트 hook은 이러한 필드를 허용합니다:

| 필드       | 필수  | 설명                                                                                                               |
| :------- | :-- | :--------------------------------------------------------------------------------------------------------------- |
| `prompt` | 예   | 모델에 전송할 프롬프트 텍스트. hook 입력 JSON에 대한 자리 표시자로 `$ARGUMENTS` 사용. 리터럴 텍스트를 포함하려면 백슬래시로 이스케이프: `\$1.00`은 `$1.00`으로 렌더링됨 |
| `model`  | 아니오 | 평가에 사용할 모델. 기본값은 빠른 모델                                                                                           |

<h3 id="reference-scripts-by-path">
  경로별로 스크립트 참조
</h3>

프로젝트 또는 plugin 루트를 기준으로 hook 스크립트를 참조하려면 이러한 자리 표시자를 사용합니다. hook이 실행될 때의 작업 디렉토리와 관계없이:

* `${CLAUDE_PROJECT_DIR}`: 프로젝트 루트. Claude Code는 또한 이 변수를 [stdio MCP 서버](/docs/ko/mcp#option-3-add-a-local-stdio-server)와 plugin LSP 서버의 환경에서 설정합니다.
* `${CLAUDE_PLUGIN_ROOT}`: plugin의 설치 디렉토리, [plugin](/docs/ko/plugins)과 함께 번들된 스크립트의 경우. plugin 업데이트 시마다 변경됩니다.
* `${CLAUDE_PLUGIN_DATA}`: plugin의 [지속적 데이터 디렉토리](/docs/ko/plugins-reference#persistent-data-directory), plugin 업데이트를 거쳐 유지되어야 하는 종속성 및 상태의 경우.

경로 자리 표시자를 참조하는 모든 hook에 대해 [exec 형식](#exec-form-and-shell-form)을 선호합니다. Exec 형식은 각 `args` 요소를 셸 토큰화 없이 하나의 인수로 전달하므로 공백이나 특수 문자가 있는 경로는 따옴표가 필요하지 않습니다. 셸 형식에서는 각 자리 표시자를 큰따옴표로 감싸세요.

<Tabs>
  <Tab title="프로젝트 스크립트">
    이 예제는 `${CLAUDE_PROJECT_DIR}`을 사용하여 `Write` 또는 `Edit` 도구 호출 후 프로젝트의 `.claude/hooks/` 디렉토리에서 스타일 검사기를 실행합니다:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Plugin 스크립트">
    `hooks/hooks.json`에서 plugin hook을 정의하고 선택적 최상위 `description` 필드를 포함합니다. plugin이 활성화되면 해당 hook이 사용자 및 프로젝트 hook과 병합됩니다.

    이 예제는 plugin과 함께 번들된 형식 지정 스크립트를 실행합니다:

    ```json theme={null}
    {
      "description": "Automatic code formatting",
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PLUGIN_ROOT}/scripts/format.sh",
                "args": [],
                "timeout": 30
              }
            ]
          }
        ]
      }
    }
    ```

    plugin hook 생성에 대한 자세한 내용은 [plugin 컴포넌트 참조](/docs/ko/plugins-reference#hooks)를 참조하세요.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Skill 및 에이전트의 Hook
</h3>

설정 파일 및 plugin 외에도 hook은 frontmatter를 사용하여 [skill](/docs/ko/skills) 및 [subagent](/docs/ko/sub-agents)에서 직접 정의할 수 있습니다. 이러한 hook은 컴포넌트의 수명 주기로 범위가 지정되며 해당 컴포넌트가 활성화되어 있을 때만 실행됩니다.

모든 hook 이벤트가 지원됩니다. subagent의 경우 `Stop` hook은 subagent가 완료될 때 발생하는 이벤트이므로 자동으로 `SubagentStop`으로 변환됩니다.

Hook은 설정 기반 hook과 동일한 구성 형식을 사용하지만 컴포넌트의 수명으로 범위가 지정되고 완료될 때 정리됩니다.

이 skill은 각 `Bash` 명령 전에 보안 검증 스크립트를 실행하는 `PreToolUse` hook을 정의합니다:

```yaml theme={null}
---
name: secure-operations
description: Perform operations with security checks
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/security-check.sh"
---
```

에이전트는 YAML frontmatter에서 동일한 형식을 사용합니다.

<h3 id="the-/hooks-menu">
  `/hooks` 메뉴
</h3>

Claude Code에서 `/hooks`를 입력하여 구성된 hook의 읽기 전용 브라우저를 엽니다. 메뉴는 구성된 hook 수가 있는 모든 hook 이벤트를 표시하고, matcher로 드릴다운할 수 있으며, 각 hook 핸들러의 전체 세부 정보를 표시합니다. 구성을 확인하거나, hook이 어느 설정 파일에서 왔는지 확인하거나, hook의 명령, 프롬프트 또는 URL을 검사하는 데 사용합니다.

메뉴는 다섯 가지 hook 유형을 표시합니다: `command`, `prompt`, `agent`, `http`, `mcp_tool`. 각 hook은 소스를 나타내는 `[type]` 접두사와 레이블이 지정됩니다:

* `User`: `~/.claude/settings.json`에서
* `Project`: `.claude/settings.json`에서
* `Local`: `.claude/settings.local.json`에서
* `Plugin`: plugin의 `hooks/hooks.json`에서
* `Session`: 현재 세션을 위해 메모리에 등록됨
* `Built-in`: Claude Code에 의해 내부적으로 등록됨

hook을 선택하면 이벤트, matcher, 유형, 소스 파일, 전체 명령, 프롬프트 또는 URL을 표시하는 세부 정보 보기가 열립니다. 메뉴는 읽기 전용입니다: hook을 추가, 수정 또는 제거하려면 설정 JSON을 직접 편집하거나 Claude에 변경을 요청하세요.

<h3 id="disable-or-remove-hooks">
  Hook 비활성화 또는 제거
</h3>

hook을 제거하려면 설정 JSON 파일에서 해당 항목을 삭제합니다.

모든 hook을 제거하지 않고 임시로 비활성화하려면 설정 파일에서 `"disableAllHooks": true`를 설정합니다. 구성에 유지하면서 개별 hook을 비활성화할 수 있는 방법은 없습니다.

`disableAllHooks` 설정은 관리형 설정 계층을 준수합니다. 관리자가 관리형 정책 설정을 통해 hook을 구성한 경우 사용자, 프로젝트 또는 로컬 설정에서 설정된 `disableAllHooks`는 해당 관리형 hook을 비활성화할 수 없습니다. 관리형 설정 수준에서 설정된 `disableAllHooks`만 관리형 hook을 비활성화할 수 있습니다.

설정 파일의 hook에 대한 직접 편집은 일반적으로 파일 감시자에 의해 자동으로 선택됩니다.

<h2 id="hook-input-and-output">
  Hook 입출력
</h2>

명령 hook은 stdin을 통해 JSON 데이터를 받고 종료 코드, stdout, stderr를 통해 결과를 전달합니다. HTTP hook은 POST 요청 본문으로 동일한 JSON을 받고 HTTP 응답 본문을 통해 결과를 전달합니다. 이 섹션에서는 모든 이벤트에 공통적인 필드와 동작을 다룹니다. [Hook 이벤트](#hook-events) 아래의 각 이벤트 섹션에는 특정 입력 스키마와 결정 제어 옵션이 포함됩니다.

macOS 및 Linux에서 명령 hook은 v2.1.139부터 제어 터미널 없이 자신의 세션에서 실행됩니다. hook 프로세스 및 모든 자식 프로세스는 `/dev/tty`를 열거나 Claude Code 인터페이스에 직접 이스케이프 시퀀스를 보낼 수 없습니다. Windows에는 `/dev/tty`가 없습니다. 모든 플랫폼에서 사용자에게 메시지를 표시하려면 JSON 출력에서 [`systemMessage`](#json-output)를 반환합니다. 데스크톱 알림을 트리거하거나 창 제목을 설정하거나 벨을 울리려면 대신 [`terminalSequence`](#emit-terminal-notifications)를 반환합니다.

<h3 id="common-input-fields">
  공통 입력 필드
</h3>

Hook 이벤트는 각 [hook 이벤트](#hook-events) 섹션에서 문서화된 이벤트 특정 필드 외에 이러한 필드를 JSON으로 받습니다. 명령 hook의 경우 이 JSON은 stdin을 통해 도착합니다. HTTP hook의 경우 POST 요청 본문으로 도착합니다.

| 필드                | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `session_id`      | 현재 세션 식별자                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `prompt_id`       | 현재 처리 중인 사용자 프롬프트를 식별하는 UUID입니다. [OpenTelemetry 이벤트의 `prompt.id` 속성](/docs/ko/monitoring-usage#event-correlation-attributes)과 일치하므로 hook 출력을 단일 프롬프트의 원격 분석과 연관시킬 수 있습니다. 첫 번째 사용자 입력까지 없습니다. Claude Code v2.1.196 이상 필요                                                                                                                                                                                                                                                            |
| `transcript_path` | 대화 JSON 경로입니다. 트랜스크립트 파일은 비동기적으로 기록되며 메모리 내 대화보다 뒤떨어질 수 있으므로 hook이 발생할 때 현재 턴의 가장 최근 메시지를 아직 포함하지 않을 수 있습니다. 현재 턴의 최종 어시스턴트 텍스트가 필요한 hook은 트랜스크립트를 읽는 대신 [Stop](#stop) 및 [SubagentStop](#subagentstop)에서 `last_assistant_message`를 사용해야 합니다                                                                                                                                                                                                                                    |
| `cwd`             | hook이 호출될 때의 현재 작업 디렉토리                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `permission_mode` | 현재 [권한 모드](/docs/ko/permissions#permission-modes): `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` 또는 `"bypassPermissions"`. **수동**으로 표시된 모드는 `"default"`로 도착하며 `"manual"`로 도착하지 않으므로 `"default"`와 일치하는 스크립트는 계속 작동합니다. 모든 이벤트가 이 필드를 받는 것은 아닙니다. 각 [hook 이벤트](#hook-events) 섹션의 JSON 예제를 확인하세요                                                                                                                                                                       |
| `effort`          | 활성 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 보유하는 `level` 필드가 있는 객체: `"low"`, `"medium"`, `"high"`, `"xhigh"` 또는 `"max"`. 요청된 모델 노력이 현재 모델이 지원하는 것을 초과하면 이는 모델이 실제로 사용한 다운그레이드된 수준입니다. Ultracode는 별개의 수준이 아니며 `"xhigh"`로 보고됩니다. 객체는 [상태 줄](/docs/ko/statusline#available-data) `effort` 필드와 일치합니다. `PreToolUse`, `PostToolUse`, `Stop`, `SubagentStop`과 같은 도구 사용 컨텍스트 내에서 발생하는 이벤트에 대해 현재 모델이 노력 매개변수를 지원할 때 존재합니다. 수준은 `$CLAUDE_EFFORT` 환경 변수로 hook 명령 및 Bash 도구에서도 사용 가능합니다. |
| `hook_event_name` | 발생한 이벤트의 이름                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

`--agent`로 실행하거나 subagent 내부에서 실행할 때 두 개의 추가 필드가 포함됩니다:

| 필드           | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | subagent의 고유 식별자. hook이 subagent 호출 내부에서 발생할 때만 존재합니다. 이를 사용하여 subagent hook 호출을 메인 스레드 호출과 구별합니다.                                                                                                                                                                                                                                                                                                                                     |
| `agent_type` | 에이전트 이름 (예: `"Explore"` 또는 `"security-reviewer"`). 세션이 `--agent`를 사용하거나 hook이 subagent 내부에서 발생할 때 존재합니다. subagent의 경우 subagent의 유형이 세션의 `--agent` 값보다 우선합니다. [사용자 정의 subagent](/docs/ko/sub-agents)의 경우 이는 에이전트의 frontmatter에서 `name` 필드이며 파일명이 아닙니다. [플러그인](/docs/ko/plugins)에서 제공하는 subagent의 경우 이는 `my-plugin:reviewer`와 같은 플러그인 범위 식별자이며 bare frontmatter 이름이 아닙니다. 플러그인 범위 이름에 대해 matcher를 작성하는 방법은 [SubagentStart](#subagentstart)를 참조하세요. |

`SessionStart` hook만 `model` 필드를 받을 수 있으며 존재가 보장되지 않습니다. `$CLAUDE_MODEL` 환경 변수는 없습니다. hook 프로세스는 부모 환경을 상속하므로 셸에서 설정한 경우 `$ANTHROPIC_MODEL`을 읽을 수 있지만 세션 중에 `/model`로 모델을 전환할 때 해당 값은 변경되지 않습니다. 상속되지 않는 변수 집합이 하나 있습니다: Claude Code는 [hook을 포함한 생성하는 모든 서브프로세스에서 `OTEL_*` 내보내기 변수를 제거합니다](/docs/ko/monitoring-usage#administrator-configuration).

예를 들어 Bash 명령에 대한 `PreToolUse` hook은 stdin에서 다음을 받습니다:

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

`tool_name` 및 `tool_input` 필드는 이벤트 특정입니다. 각 [hook 이벤트](#hook-events) 섹션에서는 해당 이벤트의 추가 필드를 문서화합니다.

<h3 id="exit-code-output">
  종료 코드 출력
</h3>

hook 명령의 종료 코드는 Claude Code에 작업을 진행할지, 차단할지 또는 무시할지를 알려줍니다.

**종료 0**은 성공을 의미합니다. Claude Code는 [JSON 출력 필드](#json-output)에 대해 stdout을 구문 분석합니다. JSON 출력은 종료 0에서만 처리됩니다. 대부분의 이벤트에서 stdout은 디버그 로그에 기록되지만 트랜스크립트에는 표시되지 않습니다. 예외는 `UserPromptSubmit`, `UserPromptExpansion`, 및 `SessionStart`이며, 여기서 stdout은 Claude가 보고 작용할 수 있는 컨텍스트로 추가됩니다.

**종료 2**는 차단 오류를 의미합니다. Claude Code는 stdout과 그 안의 JSON을 무시합니다. 대신 stderr 텍스트가 Claude에 오류 메시지로 피드백됩니다. 효과는 이벤트에 따라 다릅니다: `PreToolUse`는 도구 호출을 차단하고 `UserPromptSubmit`은 프롬프트를 거부합니다. 전체 목록은 [이벤트별 종료 코드 2 동작](#exit-code-2-behavior-per-event)을 참조하세요.

**다른 종료 코드**는 대부분의 hook 이벤트에 대한 차단하지 않는 오류입니다. 트랜스크립트는 `<hook name> hook error` 알림을 표시하고 stderr의 첫 번째 줄을 표시하므로 `--debug` 없이도 원인을 식별할 수 있습니다. 실행이 계속되고 전체 stderr은 디버그 로그에 기록됩니다.

예를 들어 위험한 Bash 명령을 차단하는 hook 명령 스크립트:

```bash theme={null}
#!/bin/bash
# stdin에서 JSON 입력을 읽고 명령을 확인합니다
command=$(jq -r '.tool_input.command' < /dev/stdin)

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # 차단 오류: 도구 호출이 방지됨
fi

exit 0  # 성공: 도구 호출이 진행됨
```

<Warning>
  대부분의 hook 이벤트에서 종료 코드 2만 작업을 차단합니다. Claude Code는 종료 코드 1을 차단하지 않는 오류로 취급하고 작업을 진행합니다. 1이 기존 Unix 실패 코드이지만 말입니다. hook이 정책을 적용하려면 `exit 2`를 사용합니다. 예외는 `WorktreeCreate`이며, 0이 아닌 종료 코드는 worktree 생성을 중단합니다.
</Warning>

<h4 id="exit-code-2-behavior-per-event">
  이벤트별 종료 코드 2 동작
</h4>

종료 코드 2는 hook이 "멈춰, 이것을 하지 마"라고 신호하는 방식입니다. 효과는 이벤트에 따라 다릅니다. 일부 이벤트는 차단할 수 있는 작업을 나타내고 (아직 발생하지 않은 도구 호출처럼) 다른 이벤트는 이미 발생했거나 방지할 수 없는 것을 나타내기 때문입니다.

| Hook 이벤트              | 차단 가능? | 종료 코드 2에서 발생하는 것                                                                                   |
| :-------------------- | :----- | :------------------------------------------------------------------------------------------------- |
| `PreToolUse`          | 예      | 도구 호출을 차단합니다                                                                                       |
| `PermissionRequest`   | 예      | 권한을 거부합니다                                                                                          |
| `UserPromptSubmit`    | 예      | 프롬프트 처리를 차단하고 프롬프트를 지웁니다                                                                           |
| `UserPromptExpansion` | 예      | 확장을 차단합니다                                                                                          |
| `Stop`                | 예      | Claude가 중지되는 것을 방지하고 대화를 계속합니다                                                                     |
| `SubagentStop`        | 예      | subagent가 중지되는 것을 방지합니다                                                                            |
| `TeammateIdle`        | 예      | 팀원이 유휴 상태가 되는 것을 방지합니다 (팀원이 계속 작업함)                                                                |
| `TaskCreated`         | 예      | 작업 생성을 롤백합니다                                                                                       |
| `TaskCompleted`       | 예      | 작업이 완료로 표시되는 것을 방지합니다                                                                              |
| `ConfigChange`        | 예      | 구성 변경이 적용되는 것을 차단합니다 (`policy_settings` 제외)                                                        |
| `StopFailure`         | 아니오    | 출력과 종료 코드는 무시됩니다                                                                                   |
| `PostToolUse`         | 아니오    | Claude에 stderr을 표시합니다 (도구가 이미 실행됨)                                                                 |
| `PostToolUseFailure`  | 아니오    | Claude에 stderr을 표시합니다 (도구가 이미 실패함)                                                                 |
| `PostToolBatch`       | 예      | 다음 모델 호출 전에 에이전트 루프를 중지합니다                                                                         |
| `PermissionDenied`    | 아니오    | 종료 코드와 stderr은 무시됩니다 (거부가 이미 발생함). JSON `hookSpecificOutput.retry: true`를 사용하여 모델이 재시도할 수 있음을 알립니다 |
| `Notification`        | 아니오    | 사용자에게만 stderr을 표시합니다                                                                               |
| `SubagentStart`       | 아니오    | 사용자에게만 stderr을 표시합니다                                                                               |
| `SessionStart`        | 아니오    | 사용자에게만 stderr을 표시합니다                                                                               |
| `Setup`               | 아니오    | 사용자에게만 stderr을 표시합니다                                                                               |
| `SessionEnd`          | 아니오    | 사용자에게만 stderr을 표시합니다                                                                               |
| `CwdChanged`          | 아니오    | 사용자에게만 stderr을 표시합니다                                                                               |
| `FileChanged`         | 아니오    | 사용자에게만 stderr을 표시합니다                                                                               |
| `PreCompact`          | 예      | 압축을 차단합니다                                                                                          |
| `PostCompact`         | 아니오    | 사용자에게만 stderr을 표시합니다                                                                               |
| `Elicitation`         | 예      | elicitation을 거부합니다                                                                                 |
| `ElicitationResult`   | 예      | 응답을 차단합니다 (작업이 거부됨)                                                                                |
| `WorktreeCreate`      | 예      | 0이 아닌 종료 코드로 인해 worktree 생성이 실패합니다                                                                 |
| `WorktreeRemove`      | 아니오    | 실패는 디버그 모드에서만 기록됩니다                                                                                |
| `InstructionsLoaded`  | 아니오    | 종료 코드는 무시됩니다                                                                                       |
| `MessageDisplay`      | 아니오    | 원본 텍스트가 표시됩니다                                                                                      |

`SessionStart`, `Setup`, 및 `SubagentStart`의 경우 종료 코드 2 stderr은 트랜스크립트에 `<hook name> hook error` 알림으로 렌더링되며, [차단하지 않는 오류](#exit-code-output)와 동일한 방식입니다. Claude는 이를 보지 못하며 세션 또는 subagent는 진행됩니다. `SubagentStart`의 경우 알림은 부모 대화가 아닌 subagent의 자신의 트랜스크립트에 나타납니다.

Claude Code v2.1.199부터 `SessionStart`, `Setup`, 및 `SubagentStart`는 트랜스크립트에 종료 코드 2 stderr을 표시합니다. 이전 버전은 디버그 로그에만 기록했습니다.

<h3 id="http-response-handling">
  HTTP 응답 처리
</h3>

HTTP hook은 종료 코드와 stdout 대신 HTTP 상태 코드와 응답 본문을 사용합니다:

* **2xx 빈 본문**: 성공, 종료 코드 0과 출력 없음과 동등
* **2xx 일반 텍스트 본문**: 성공, 텍스트가 컨텍스트로 추가됨
* **2xx JSON 본문**: 성공, 명령 hook과 동일한 [JSON 출력](#json-output) 스키마를 사용하여 구문 분석됨
* **2xx가 아닌 상태**: 차단하지 않는 오류, 실행이 계속됨
* **연결 실패 또는 시간 초과**: 차단하지 않는 오류, 실행이 계속됨

명령 hook과 달리 HTTP hook은 상태 코드만으로 차단 오류를 신호할 수 없습니다. 도구 호출을 차단하거나 권한을 거부하려면 적절한 결정 필드를 포함하는 JSON 본문이 있는 2xx 응답을 반환합니다.

<h3 id="json-output">
  JSON 출력
</h3>

종료 코드를 사용하면 허용 또는 차단할 수 있지만 JSON 출력은 더 세밀한 제어를 제공합니다. 종료 코드 2로 차단하는 대신 종료 0으로 JSON 객체를 stdout에 인쇄합니다. Claude Code는 해당 JSON에서 특정 필드를 읽어 차단, 허용 또는 사용자에게 에스컬레이션을 포함한 동작을 제어합니다.

<Note>
  hook당 하나의 접근 방식을 선택해야 합니다. 둘 다 선택하지 마세요: 종료 코드만 사용하여 신호하거나 종료 0으로 JSON을 인쇄하여 구조화된 제어를 합니다. Claude Code는 종료 0에서만 JSON을 처리합니다. 종료 2로 나가면 JSON은 무시됩니다.
</Note>

hook의 stdout은 JSON 객체만 포함해야 합니다. 셸 프로필이 시작 시 텍스트를 인쇄하면 JSON 구문 분석을 방해할 수 있습니다. 문제 해결 가이드의 [JSON 검증 실패](/docs/ko/hooks-guide#json-validation-failed)를 참조하세요.

hook 출력 문자열 (`additionalContext`, `systemMessage`, 및 일반 stdout)은 10,000자로 제한됩니다. 이 제한을 초과하는 출력은 파일에 저장되고 미리보기 및 파일 경로로 바뀌며, 큰 도구 결과가 처리되는 방식과 동일합니다.

JSON 객체는 세 가지 종류의 필드를 지원합니다:

* **`continue`와 같은 범용 필드**는 모든 이벤트에서 작동합니다. 이들은 아래 표에 나열되어 있습니다.
* \*\*최상위 `decision` 및 `reason`\*\*은 일부 이벤트에서 차단하거나 피드백을 제공하는 데 사용됩니다.
* \*\*`hookSpecificOutput`\*\*은 더 풍부한 제어가 필요한 이벤트를 위한 중첩 객체입니다. 이벤트 이름으로 설정된 `hookEventName` 필드가 필요합니다.

| 필드                 | 기본값     | 설명                                                                                                                                                                                   |
| :----------------- | :------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `continue`         | `true`  | `false`인 경우 hook이 실행된 후 Claude가 완전히 중지됩니다. 모든 이벤트 특정 결정 필드보다 우선합니다                                                                                                                   |
| `stopReason`       | 없음      | `continue`가 `false`일 때 사용자에게 표시되는 메시지. Claude에는 표시되지 않음                                                                                                                              |
| `suppressOutput`   | `false` | `true`인 경우 디버그 로그에서 stdout을 숨깁니다                                                                                                                                                     |
| `systemMessage`    | 없음      | 사용자에게 표시되는 경고 메시지                                                                                                                                                                    |
| `terminalSequence` | 없음      | Claude Code가 사용자를 대신하여 내보낼 터미널 이스케이프 시퀀스 (예: 데스크톱 알림, 창 제목 또는 벨). OSC `0`/`1`/`2`/`9`/`99`/`777` 및 BEL로 제한됩니다. 값에 허용 목록 외의 항목이 포함되면 필드는 무시됩니다. `/dev/tty`를 사용할 수 없는 hook 대신 이를 사용합니다 |

Claude를 이벤트 유형과 관계없이 완전히 중지하려면:

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

<h4 id="emit-terminal-notifications">
  터미널 알림 내보내기
</h4>

`terminalSequence` 필드는 Claude Code v2.1.141 이상이 필요합니다.

Hook은 제어 터미널 없이 실행되므로 이스케이프 시퀀스를 `/dev/tty`에 직접 쓰는 것이 실패합니다. 대신 `terminalSequence` 필드에 이스케이프 시퀀스를 반환하면 Claude Code가 자신의 터미널 쓰기 경로를 통해 이를 내보냅니다. 이는 race-free이고 tmux 및 GNU screen 내에서 작동하며 `/dev/tty`가 없는 Windows에서도 작동합니다.

필드는 하나 이상의 허용 목록에 있는 이스케이프 시퀀스 문자열을 허용합니다:

* OSC `0`, `1`, `2`: 창 및 아이콘 제목
* OSC `9`: iTerm2, ConEmu, Windows Terminal, 및 WezTerm 알림 (`9;4` 작업 표시줄 진행률 포함)
* OSC `99`: Kitty 알림
* OSC `777`: urxvt, Ghostty, 및 Warp 알림
* 맨 BEL

시퀀스는 BEL 또는 ST로 종료될 수 있습니다. 허용 목록 외의 항목 (CSI 커서 및 색상 시퀀스, OSC 팔레트 시퀀스, OSC 8 하이퍼링크, OSC 52 클립보드 쓰기, 및 OSC 1337 포함)은 거부되고 필드는 무시됩니다.

아래 예제는 `Notification` hook에서 데스크톱 알림을 발생시킵니다. 이스케이프 시퀀스는 `printf` 8진수 이스케이프로 빌드되므로 제어 바이트가 셸 명령줄에 나타나지 않으며, `jq -n --arg`는 JSON 출력을 빌드하므로 알림 메시지의 따옴표, 백슬래시, 및 줄바꿈이 올바르게 이스케이프됩니다:

```bash theme={null}
#!/bin/bash
# Notification hook: Claude Code가 주의가 필요할 때 데스크톱을 ping합니다.
input=$(cat)
title="Claude Code'
body=$(jq -r '.message // 'Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

`{ "terminalSequence": "..." }` 형태는 모든 셸 또는 언어에서 동일합니다. Windows에서는 PowerShell 또는 스크립트에서 이스케이프 문자열을 빌드하고 동일한 JSON 객체를 내보냅니다.

<Note>
  `terminalSequence`는 이전에 `/dev/tty`에 직접 이스케이프 시퀀스를 작성한 hook의 지원되는 대체입니다. 허용 목록은 커서를 이동하거나 색상을 변경할 수 없는 시퀀스로 제한되므로 hook은 화면상의 프롬프트를 손상시킬 수 없습니다.
</Note>

<h4 id="add-context-for-claude">
  Claude를 위한 컨텍스트 추가
</h4>

`additionalContext` 필드는 hook에서 Claude의 컨텍스트 윈도우로 문자열을 전달합니다. Claude Code는 문자열을 시스템 미리 알림으로 래핑하고 hook이 발생한 지점에서 대화에 삽입합니다. Claude는 다음 모델 요청에서 미리 알림을 읽지만 인터페이스에 채팅 메시지로 나타나지 않습니다.

이벤트 이름과 함께 `hookSpecificOutput` 내에 `additionalContext`를 반환합니다:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "This file is generated. Edit src/schema.ts and run `bun generate` instead."
  }
}
```

미리 알림이 나타나는 위치는 이벤트에 따라 다릅니다:

* [SessionStart](#sessionstart), [Setup](#setup), 및 [SubagentStart](#subagentstart): 대화 시작, 첫 번째 프롬프트 전
* [UserPromptSubmit](#userpromptsubmit) 및 [UserPromptExpansion](#userpromptexpansion): 제출된 프롬프트 옆
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure), 및 [PostToolBatch](#posttoolbatch): 도구 결과 옆
* [Stop](#stop) 및 [SubagentStop](#subagentstop): 턴의 끝. 대화가 계속되므로 Claude가 피드백에 작용할 수 있습니다. [Stop 결정 제어](#stop-decision-control) 참조

여러 hook이 동일한 이벤트에 대해 `additionalContext`를 반환하면 Claude는 모든 값을 받습니다. 값이 10,000자를 초과하면 Claude Code는 전체 텍스트를 세션 디렉토리의 파일에 쓰고 짧은 미리보기와 함께 파일 경로를 Claude에 전달합니다.

Claude가 현재 환경 상태 또는 방금 실행된 작업에 대해 알아야 할 정보에 `additionalContext`를 사용합니다:

* **환경 상태**: 현재 분기, 배포 대상 또는 활성 기능 플래그
* **조건부 프로젝트 규칙**: 방금 편집한 파일에 적용되는 테스트 명령, 이 worktree에서 읽기 전용인 디렉토리
* **외부 데이터**: 사용자에게 할당된 열린 문제, 최근 CI 결과, 내부 서비스에서 가져온 콘텐츠

변경되지 않는 지침의 경우 [CLAUDE.md](/docs/ko/memory)를 선호합니다. 스크립트를 실행하지 않고 로드되며 정적 프로젝트 규칙의 표준 위치입니다.

명령형 시스템 지침이 아닌 사실 진술로 텍스트를 작성합니다. "배포 대상은 프로덕션입니다" 또는 "이 리포지토리는 `bun test`를 사용합니다"와 같은 표현은 프로젝트 정보로 읽힙니다. 대역 외 시스템 명령으로 표현된 텍스트는 Claude의 프롬프트 주입 방어를 트리거할 수 있으며, 이로 인해 Claude가 텍스트를 컨텍스트로 취급하는 대신 사용자에게 표시합니다.

주입되면 텍스트는 세션 트랜스크립트에 저장됩니다. `PostToolUse` 또는 `UserPromptSubmit`과 같은 중간 세션 이벤트의 경우 `--continue` 또는 `--resume`으로 재개하면 과거 턴에 대해 hook을 다시 실행하는 대신 저장된 텍스트를 재생하므로 타임스탬프 또는 커밋 SHA와 같은 값이 재개 시 오래됩니다. `SessionStart` hook은 `source`가 `"resume"`으로 설정된 재개 시 다시 실행되므로 컨텍스트를 새로 고칠 수 있습니다.

<h4 id="decision-control">
  결정 제어
</h4>

모든 이벤트가 JSON을 통해 동작을 차단하거나 제어하는 것을 지원하는 것은 아닙니다. 그렇게 하는 이벤트는 각각 다른 필드 집합을 사용하여 해당 결정을 표현합니다. hook을 작성하기 전에 이 표를 빠른 참조로 사용하세요:

| 이벤트                                                                                                                                 | 결정 패턴                      | 주요 필드                                                                                                                                                                                                          |
| :---------------------------------------------------------------------------------------------------------------------------------- | :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | 최상위 `decision`             | `decision: "block"`, `reason`. Stop 및 SubagentStop은 또한 [오류가 아닌 피드백을 위해 대화를 계속하는](#stop-decision-control) `hookSpecificOutput.additionalContext`를 허용합니다                                                         |
| TeammateIdle, TaskCreated, TaskCompleted                                                                                            | 종료 코드 또는 `continue: false` | 종료 코드 2는 stderr 피드백으로 작업을 차단합니다. JSON `{"continue": false, "stopReason": "..."}` 또한 팀원을 완전히 중지하여 `Stop` hook 동작과 일치합니다                                                                                         |
| PreToolUse                                                                                                                          | `hookSpecificOutput`       | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                        |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`       | `decision.behavior` (allow/deny)                                                                                                                                                                               |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`       | `retry: true`는 모델이 거부된 도구 호출을 재시도할 수 있음을 알립니다                                                                                                                                                                  |
| WorktreeCreate                                                                                                                      | 경로 반환                      | 명령 hook은 stdout에 경로를 인쇄합니다; HTTP hook은 `hookSpecificOutput.worktreePath`를 반환합니다. hook 실패 또는 누락된 경로는 생성을 실패합니다                                                                                                  |
| Elicitation                                                                                                                         | `hookSpecificOutput`       | `action` (accept/decline/cancel), `content` (form field values for accept)                                                                                                                                     |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`       | `action` (accept/decline/cancel), `content` (form field values override)                                                                                                                                       |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`       | `displayContent`는 화면에 표시된 텍스트를 바꿉니다. 표시 전용: 트랜스크립트 및 Claude가 보는 것은 원본을 유지합니다                                                                                                                                   |
| SessionStart, Setup, SubagentStart                                                                                                  | 컨텍스트만                      | `hookSpecificOutput.additionalContext`는 Claude를 위한 컨텍스트를 추가합니다. SessionStart는 또한 [`initialUserMessage`, `watchPaths`, `sessionTitle`, 및 `reloadSkills`](#sessionstart-decision-control)를 허용합니다. 차단 또는 결정 제어 없음 |
| WorktreeRemove, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, FileChanged                     | 없음                         | 결정 제어 없음. 로깅 또는 정리와 같은 부작용에 사용됨                                                                                                                                                                                |

일부 이벤트는 또한 허용 또는 차단하는 것이 아니라 콘텐츠를 다시 작성할 수 있습니다:

* `PreToolUse`: `hookSpecificOutput` 바로 아래의 `updatedInput`은 실행 전에 도구의 인수를 바꿉니다. [PreToolUse 결정 제어](#pretooluse-decision-control) 참조
* `PermissionRequest`: `decision` 객체 내의 `updatedInput`. [PermissionRequest 결정 제어](#permissionrequest-decision-control) 참조
* `PostToolUse`: `updatedToolOutput`은 도구의 결과를 바꿉니다. [PostToolUse 결정 제어](#posttooluse-decision-control) 참조
* `UserPromptSubmit`: 프롬프트를 바꿀 수 없습니다; `additionalContext`를 옆에만 주입합니다

편집 또는 변환 사용 사례의 경우 아웃바운드 도구 입력에 대해 `PreToolUse`에서 가로채고 인바운드 도구 결과에 대해 `PostToolUse`에서 가로채세요.

다음은 각 패턴의 실제 예입니다:

<Tabs>
  <Tab title="최상위 결정">
    `UserPromptSubmit`, `UserPromptExpansion`, `PostToolUse`, `PostToolUseFailure`, `PostToolBatch`, `Stop`, `SubagentStop`, `ConfigChange`, `PreCompact`에서 사용됩니다. 유일한 값은 `"block"`입니다. 작업을 진행하도록 허용하려면 JSON에서 `decision`을 생략하거나 JSON 없이 종료 0으로 나갑니다:

    ```json theme={null}
    {
      "decision": "block",
      "reason": "Test suite must pass before proceeding"
    }
    ```
  </Tab>

  <Tab title="PreToolUse">
    더 풍부한 제어를 위해 `hookSpecificOutput`을 사용합니다: 허용, 거부, 요청 또는 연기. 실행 전에 도구 입력을 수정하거나 Claude를 위한 추가 컨텍스트를 주입할 수도 있습니다. 전체 옵션 집합은 [PreToolUse 결정 제어](#pretooluse-decision-control)를 참조하세요.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Database writes are not allowed"
      }
    }
    ```
  </Tab>

  <Tab title="PermissionRequest">
    `hookSpecificOutput`을 사용하여 사용자를 대신하여 권한 요청을 허용하거나 거부합니다. 허용할 때 도구의 입력을 수정하거나 권한 규칙을 적용하여 사용자가 다시 프롬프트되지 않도록 할 수 있습니다. 전체 옵션 집합은 [PermissionRequest 결정 제어](#permissionrequest-decision-control)를 참조하세요.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PermissionRequest",
        "decision": {
          "behavior": "allow",
          "updatedInput": {
            "command": "npm run lint"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Bash 명령 검증, 프롬프트 필터링, 자동 승인 스크립트를 포함한 확장 예제는 가이드의 [자동화할 수 있는 것](/docs/ko/hooks-guide#what-you-can-automate)과 [Bash 명령 검증기 참조 구현](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py)을 참조하세요.

<h2 id="hook-events">
  Hook 이벤트
</h2>

각 이벤트는 hook이 실행될 수 있는 Claude Code의 수명 주기의 지점에 해당합니다. 아래 섹션은 수명 주기와 일치하도록 정렬됩니다: 세션 설정에서 에이전트 루프를 거쳐 세션 종료까지. 각 섹션에서는 이벤트가 언제 발생하는지, 지원하는 matcher, 받는 JSON 입력, 출력을 통해 동작을 제어하는 방법을 설명합니다.

<h3 id="sessionstart">
  SessionStart
</h3>

Claude Code가 새 세션을 시작하거나 기존 세션을 재개할 때 실행됩니다. 기존 문제나 코드베이스의 최근 변경 사항과 같은 개발 컨텍스트를 로드하거나 환경 변수를 설정하는 데 유용합니다. 스크립트가 필요하지 않은 정적 컨텍스트의 경우 [CLAUDE.md](/docs/ko/memory)를 사용하세요.

SessionStart는 모든 세션에서 실행되므로 이러한 hook을 빠르게 유지하세요. `type: "command"` 및 `type: "mcp_tool"` hook만 지원됩니다.

matcher 값은 세션이 시작된 방식에 해당합니다:

| Matcher   | 언제 발생하는지                              |
| :-------- | :------------------------------------ |
| `startup` | 새 세션                                  |
| `resume`  | `--resume`, `--continue` 또는 `/resume` |
| `clear`   | `/clear`                              |
| `compact` | 자동 또는 수동 압축                           |

<h4 id="sessionstart-input">
  SessionStart 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 SessionStart hook은 `source` 및 선택적으로 `model`, `agent_type`, `session_title`을 받습니다:

| 필드              | 설명                                                                                                                                       |
| :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `source`        | 세션이 시작된 방식: 새 세션의 경우 `"startup"`, 재개된 세션의 경우 `"resume"`, `/clear` 후 `"clear"`, 압축 후 `"compact"`                                          |
| `model`         | 활성 모델 식별자. 예를 들어 `/clear` 후 또는 대화 복구를 통해 세션이 복원될 때 생략될 수 있으므로 필드를 읽기 전에 확인하세요                                                            |
| `agent_type`    | `claude --agent <name>`으로 Claude Code를 시작할 때 존재하는 에이전트 이름                                                                                |
| `session_title` | 이미 설정된 경우 현재 세션 제목 (예: `--name` 또는 `/rename`을 통해). `sessionTitle`을 내보내는 hook은 사용자가 명시적으로 설정한 제목을 덮어쓰지 않도록 먼저 `session_title`을 확인할 수 있습니다 |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "startup",
  "model": "claude-sonnet-5"
}
```

<h4 id="sessionstart-decision-control">
  SessionStart 결정 제어
</h4>

hook 스크립트가 stdout에 인쇄하는 모든 텍스트는 Claude의 컨텍스트로 추가됩니다. 모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 이러한 이벤트 특정 필드를 반환할 수 있습니다:

| 필드                   | 설명                                                                                                                                                                           |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | Claude의 컨텍스트 시작 부분에 추가되는 문자열. 첫 번째 프롬프트 전에 추가됩니다. [Claude를 위한 컨텍스트 추가](#add-context-for-claude)를 참조하여 텍스트가 전달되는 방식과 포함할 내용을 확인하세요                                            |
| `initialUserMessage` | 세션의 첫 번째 사용자 메시지로 사용되는 문자열. [비대화형 모드](/docs/ko/headless)에서 `-p` 플래그와 함께 적용되며, 프롬프트가 제공되지 않으면 첫 번째 턴이 됩니다. 프롬프트가 제공되면 다음 턴으로 따릅니다. `additionalContext`와 달리 기존 턴에 첨부되는 이것은 턴을 생성합니다 |
| `sessionTitle`       | 세션 제목을 설정합니다. `/rename`과 동일한 효과입니다. 시작 폴더, git 분기 또는 worktree 이름에서 세션을 자동으로 이름 지정하는 데 사용합니다. `source`가 `"startup"` 또는 `"resume"`일 때만 적용됩니다; `"clear"` 및 `"compact"`에서는 무시됩니다 |
| `watchPaths`         | 이 세션 중에 [FileChanged](#filechanged) 이벤트를 감시할 절대 경로의 배열                                                                                                                       |
| `reloadSkills`       | 부울. `true`일 때 Claude Code는 SessionStart hook이 완료된 후 [skill](/docs/ko/skills) 및 명령 디렉토리를 다시 스캔하므로 hook이 설치한 skill은 첫 번째 프롬프트부터 같은 세션에서 사용 가능합니다                                    |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

이 이벤트에 대해 일반 stdout이 이미 Claude에 도달하므로 컨텍스트만 로드하는 hook은 JSON을 구축하지 않고 stdout에 직접 인쇄할 수 있습니다. `suppressOutput` 또는 `sessionTitle`과 같은 다른 필드와 컨텍스트를 결합해야 할 때 JSON 형식을 사용합니다.

SessionStart hook이 skill을 설치하거나 업데이트할 때 `reloadSkills`를 사용합니다. Skill 발견은 일반적으로 SessionStart hook이 완료되기 전에 실행되므로 hook이 `~/.claude/skills/` 또는 `.claude/skills/`에 작성하는 파일은 그렇지 않으면 다음 세션에만 나타납니다. 이 예제는 공유 skill 리포지토리를 동기화하고 다시 스캔을 요청합니다:

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

<h4 id="persist-environment-variables">
  환경 변수 유지
</h4>

SessionStart hook은 `CLAUDE_ENV_FILE` 환경 변수에 액세스할 수 있으며, 이는 후속 Bash 명령에 대한 환경 변수를 유지할 수 있는 파일 경로를 제공합니다.

개별 환경 변수를 설정하려면 `CLAUDE_ENV_FILE`에 `export` 문을 작성합니다. 다른 hook에서 설정한 변수를 유지하려면 추가 (`>>`)를 사용합니다:

```bash theme={null}
#!/bin/bash

if [ -n "$CLAUDE_ENV_FILE" ]; then
  echo 'export NODE_ENV=production' >> "$CLAUDE_ENV_FILE"
  echo 'export DEBUG_LOG=true' >> "$CLAUDE_ENV_FILE"
  echo 'export PATH="$PATH:./node_modules/.bin"' >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

설정 명령의 환경 변경을 모두 캡처하려면 내보낸 변수를 이전과 이후에 비교합니다:

```bash theme={null}
#!/bin/bash

ENV_BEFORE=$(export -p | sort)

# 환경을 수정하는 설정 명령을 실행합니다
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

이 파일에 작성된 모든 변수는 세션 중에 Claude Code가 실행하는 모든 후속 Bash 명령에서 사용 가능합니다.

<Note>
  `CLAUDE_ENV_FILE`은 SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged), [FileChanged](#filechanged) hook에 사용 가능합니다. 다른 hook 유형은 이 변수에 액세스할 수 없습니다.
</Note>

<h3 id="setup">
  Setup
</h3>

`--init-only`로 Claude Code를 시작하거나 [비대화형 모드](/docs/ko/headless)에서 `-p` 플래그와 함께 `--init` 또는 `--maintenance`로 시작할 때만 발생합니다. 일반 시작 시에는 발생하지 않습니다. 일회성 종속성 설치 또는 CI 또는 스크립트에서 명시적으로 트리거하는 예약된 정리에 사용합니다. 일반 세션 시작과 별도입니다. 세션별 초기화의 경우 [SessionStart](#sessionstart)를 대신 사용합니다.

matcher 값은 hook을 트리거한 CLI 플래그에 해당합니다:

| Matcher       | 언제 발생하는지                                   |
| :------------ | :----------------------------------------- |
| `init`        | `claude --init-only` 또는 `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                  |

`--init-only`는 Setup hook과 `startup` matcher가 있는 SessionStart hook을 실행한 다음 대화를 시작하지 않고 종료합니다. `--init` 및 `--maintenance`는 `-p`와 결합할 때만 Setup hook을 발생시킵니다; 대화형 세션에서 이 두 플래그는 현재 Setup hook을 발생시키지 않습니다.

Setup은 모든 시작 시 발생하지 않으므로 종속성이 설치된 plugin은 Setup만으로는 의존할 수 없습니다. 실제 패턴은 첫 사용 시 종속성을 확인하고 누락되면 설치하는 것입니다. 예를 들어 `${CLAUDE_PLUGIN_DATA}/node_modules`를 테스트하고 없으면 `npm install`을 실행하는 hook 또는 skill입니다. 설치된 종속성을 저장할 위치는 [지속적 데이터 디렉토리](/docs/ko/plugins-reference#persistent-data-directory)를 참조하세요.

<h4 id="setup-input">
  Setup 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 Setup hook은 `trigger` 필드를 받으며, 이는 `"init"` 또는 `"maintenance"`로 설정됩니다:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Setup",
  "trigger": "init"
}
```

<h4 id="setup-decision-control">
  Setup 결정 제어
</h4>

Setup hook은 차단할 수 없습니다. 0이 아닌 종료 코드 (2 포함)는 stderr을 사용자에게 `<hook name> hook error` 알림으로 표시하고 실행이 계속됩니다. [비대화형 모드](/docs/ko/headless)에서 hook 출력은 `--verbose`로 시작할 때만 나타납니다.

Claude의 컨텍스트에 정보를 전달하려면 JSON 출력에서 `additionalContext`를 반환합니다; 일반 stdout은 디버그 로그에만 작성됩니다. 모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 이러한 이벤트 특정 필드를 반환할 수 있습니다:

| 필드                  | 설명                                      |
| :------------------ | :-------------------------------------- |
| `additionalContext` | Claude의 컨텍스트에 추가되는 문자열. 여러 hook의 값이 연결됨 |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Setup",
    "additionalContext": "Dependencies installed: node_modules, .venv"
  }
}
```

Setup hook은 `CLAUDE_ENV_FILE`에 액세스할 수 있습니다. 해당 파일에 작성된 변수는 [SessionStart hook](#persist-environment-variables)과 마찬가지로 세션의 후속 Bash 명령에 유지됩니다. `type: "command"` 및 `type: "mcp_tool"` hook만 지원됩니다.

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

`CLAUDE.md` 또는 `.claude/rules/*.md` 파일이 컨텍스트에 로드될 때 발생합니다. 이 이벤트는 세션 시작 시 즉시 로드된 파일에 대해 발생하고 나중에 파일이 지연 로드될 때 다시 발생합니다. 예를 들어 Claude가 중첩된 `CLAUDE.md`를 포함하는 하위 디렉토리에 액세스할 때 또는 `paths:` frontmatter가 있는 조건부 규칙이 일치할 때입니다. hook은 차단 또는 결정 제어를 지원하지 않습니다. 관찰성 목적으로 비동기적으로 실행됩니다.

matcher는 `load_reason`에 대해 실행됩니다. 예를 들어 `"matcher": "session_start"`를 사용하여 세션 시작 시에만 로드된 파일에 대해 발생하거나 `"matcher": "path_glob_match|nested_traversal"`을 사용하여 지연 로드에만 발생합니다.

<h4 id="instructionsloaded-input">
  InstructionsLoaded 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 InstructionsLoaded hook은 이러한 필드를 받습니다:

| 필드                  | 설명                                                                                                                                                  |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | 로드된 명령 파일의 절대 경로                                                                                                                                    |
| `memory_type`       | 파일의 범위: `"User"`, `"Project"`, `"Local"` 또는 `"Managed"`                                                                                             |
| `load_reason`       | 파일이 로드된 이유: `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"` 또는 `"compact"`. `"compact"` 값은 압축 이벤트 후 명령 파일이 다시 로드될 때 발생합니다 |
| `globs`             | 파일의 `paths:` frontmatter의 경로 glob 패턴 (있는 경우). `path_glob_match` 로드에만 존재                                                                             |
| `trigger_file_path` | 지연 로드를 트리거한 파일의 경로                                                                                                                                  |
| `parent_file_path`  | 이 파일을 포함한 부모 명령 파일의 경로, `include` 로드의 경우                                                                                                            |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "InstructionsLoaded",
  "file_path": "/Users/my-project/CLAUDE.md",
  "memory_type": "Project",
  "load_reason": "session_start"
}
```

<h4 id="instructionsloaded-decision-control">
  InstructionsLoaded 결정 제어
</h4>

InstructionsLoaded hook은 결정 제어가 없습니다. 명령 로드를 차단하거나 수정할 수 없습니다. 감사 로깅, 규정 준수 추적 또는 관찰성을 위해 이 이벤트를 사용합니다.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

사용자가 프롬프트를 제출할 때, Claude가 처리하기 전에 실행됩니다. 이를 통해 프롬프트/대화를 기반으로 추가 컨텍스트를 추가하거나, 프롬프트를 검증하거나, 특정 유형의 프롬프트를 차단할 수 있습니다.

`UserPromptSubmit` hook은 `command`, `http`, `mcp_tool` 유형에 대해 기본 30초 시간 초과를 가지며, 이는 다른 이벤트에서 이러한 유형의 기본 600초보다 짧습니다. 이 hook은 모든 프롬프트 전에 실행되고 모델 처리가 완료될 때까지 차단하므로 stuck hook은 세션을 정지시킵니다. hook에 더 많은 시간이 필요하면 hook 항목에서 `timeout` 필드를 설정합니다.

시간 초과에 도달한 `UserPromptSubmit` hook은 취소되고 `additionalContext`를 포함한 출력이 삭제됩니다. 프롬프트는 여전히 해당 컨텍스트 없이 Claude에 도달합니다. v2.1.196부터 트랜스크립트는 hook의 이름, 발생한 시간 초과, 출력이 삭제되었음을 나타내는 알림을 표시합니다. 이전 버전은 알림 없이 hook을 취소합니다.

[Agent SDK callback hook](/docs/ko/agent-sdk/hooks)이 `UserPromptSubmit`에서 시간 초과에 도달하면 hook의 이름과 시간 초과를 나타내는 메시지로 프롬프트를 차단합니다. 왜냐하면 callback은 실패하지 않아야 하는 정책 게이트로 작동할 수 있기 때문입니다. 세션이 계속됩니다. v2.1.208 이전에는 callback 시간 초과가 실행 오류로 턴을 종료했습니다.

<h4 id="userpromptsubmit-input">
  UserPromptSubmit 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 UserPromptSubmit hook은 사용자가 제출한 텍스트를 포함하는 `prompt` 필드를 받습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptSubmit",
  "prompt": "Write a function to calculate the factorial of a number"
}
```

<h4 id="userpromptsubmit-decision-control">
  UserPromptSubmit 결정 제어
</h4>

`UserPromptSubmit` hook은 사용자 프롬프트 처리 여부를 제어하고 컨텍스트를 추가할 수 있습니다. 모든 [JSON 출력 필드](#json-output)를 사용할 수 있습니다.

종료 코드 0에서 대화에 컨텍스트를 추가하는 두 가지 방법이 있습니다:

* **일반 텍스트 stdout**: stdout에 작성된 JSON이 아닌 텍스트는 컨텍스트로 추가됩니다
* **`additionalContext`가 있는 JSON**: 더 많은 제어를 위해 아래 JSON 형식을 사용합니다. `additionalContext` 필드는 Claude가 읽는 시스템 알림으로 컨텍스트에 주입됩니다

일반 stdout은 트랜스크립트에 hook 출력으로 표시됩니다. `additionalContext` 값은 시스템 알림으로 주입되어 Claude가 표시되는 트랜스크립트 항목 없이 읽습니다.

프롬프트를 차단하려면 `decision`을 `"block"`으로 설정한 JSON 객체를 반환합니다:

| 필드                       | 설명                                                                           |
| :----------------------- | :--------------------------------------------------------------------------- |
| `decision`               | `"block"`은 프롬프트가 처리되는 것을 방지하고 컨텍스트에서 지웁니다. 생략하여 프롬프트를 진행하도록 허용               |
| `reason`                 | `decision`이 `"block"`일 때 사용자에게 표시됩니다. 컨텍스트에 추가되지 않음                          |
| `additionalContext`      | Claude의 컨텍스트에 추가되는 문자열. [Claude를 위한 컨텍스트 추가](#add-context-for-claude)를 참조하세요 |
| `sessionTitle`           | 세션 제목을 설정합니다. 프롬프트 내용을 기반으로 세션을 자동으로 이름 지정하는 데 사용합니다                         |
| `suppressOriginalPrompt` | `decision`이 `"block"`일 때 `true`인 경우 사용자에게 표시되는 차단 메시지에서 원본 프롬프트 텍스트를 생략합니다   |

```json theme={null}
{
  "decision": "block",
  "reason": "Explanation for decision",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "My additional context here",
    "sessionTitle": "My session title"
  }
}
```

<h3 id="userpromptexpansion">
  UserPromptExpansion
</h3>

사용자가 입력한 slash 명령이 Claude에 도달하기 전에 프롬프트로 확장될 때 실행됩니다. 이를 사용하여 특정 명령이 직접 호출되는 것을 차단하거나, 특정 skill에 대한 컨텍스트를 주입하거나, 사용자가 호출하는 명령을 기록합니다. 예를 들어 `deploy`와 일치하는 hook은 승인 파일이 없으면 `/deploy`를 차단할 수 있고, review skill과 일치하는 hook은 팀의 review 체크리스트를 `additionalContext`로 추가할 수 있습니다.

이 이벤트는 `PreToolUse`가 다루지 않는 경로를 다룹니다: `PreToolUse` hook이 `Skill` 도구와 일치하면 Claude가 도구를 호출할 때만 발생하지만, `/skillname`을 직접 입력하면 `PreToolUse`를 우회합니다. `UserPromptExpansion`은 그 직접 경로에서 발생합니다.

`command_name`에서 일치합니다. matcher를 비워두어 모든 prompt 유형 slash 명령에서 발생하도록 합니다.

<h4 id="userpromptexpansion-input">
  UserPromptExpansion 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 UserPromptExpansion hook은 `expansion_type`, `command_name`, `command_args`, `command_source`, 원본 `prompt` 문자열을 받습니다. `expansion_type` 필드는 skill 및 사용자 정의 명령의 경우 `slash_command`이거나 MCP 서버 프롬프트의 경우 `mcp_prompt`입니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../00893aaf.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptExpansion",
  "expansion_type": "slash_command",
  "command_name": "example-skill",
  "command_args": "arg1 arg2",
  "command_source": "plugin",
  "prompt": "/example-skill arg1 arg2"
}
```

<h4 id="userpromptexpansion-decision-control">
  UserPromptExpansion 결정 제어
</h4>

`UserPromptExpansion` hook은 확장을 차단하거나 컨텍스트를 추가할 수 있습니다. 모든 [JSON 출력 필드](#json-output)를 사용할 수 있습니다.

| 필드                  | 설명                                                                                        |
| :------------------ | :---------------------------------------------------------------------------------------- |
| `decision`          | `"block"`은 slash 명령이 확장되는 것을 방지합니다. 생략하여 진행하도록 허용                                         |
| `reason`            | `decision`이 `"block"`일 때 사용자에게 표시됩니다                                                      |
| `additionalContext` | 확장된 프롬프트와 함께 Claude의 컨텍스트에 추가되는 문자열. [Claude를 위한 컨텍스트 추가](#add-context-for-claude)를 참조하세요 |

```json theme={null}
{
  "decision": "block",
  "reason": "This slash command is not available",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptExpansion",
    "additionalContext": "Additional context for this expansion"
  }
}
```

<h3 id="messagedisplay">
  MessageDisplay
</h3>

어시스턴트 메시지가 화면으로 스트리밍되는 동안 실행됩니다. Claude Code는 메시지를 증분으로 표시합니다: 새로 완료된 줄의 배치가 렌더링될 준비가 될 때마다 hook이 한 번 실행되고 Claude Code는 hook의 대체 텍스트를 그 자리에 렌더링합니다. 긴 메시지는 여러 호출을 생성합니다; 짧은 메시지는 하나만 생성할 수 있습니다.

MessageDisplay를 사용하여:

* markdown을 제거하여 최소한의 표시
* 에이전트 SDK 애플리케이션이 사용자에게 표시하는 텍스트 변환
* Claude의 응답에서 API 키 또는 내부 호스트명 제거

Claude Code는 각 배치를 hook이 반환할 때까지 보유하므로 hook을 빠르게 유지하세요. hook이 실패하거나 시간 초과되면 Claude Code는 원본 텍스트를 표시합니다. 이 이벤트의 기본 시간 초과는 10초입니다; hook에 더 많은 시간이 필요하면 hook 항목에서 `timeout` 필드를 설정합니다.

MessageDisplay는 표시 전용입니다: 대체 텍스트는 화면에 렌더링되는 것만 변경합니다. 트랜스크립트와 Claude가 보는 것은 원본 텍스트를 유지하므로 Claude는 대체를 보지 못하고 verbose 모드는 원본을 표시합니다. hook은 어시스턴트 메시지 텍스트만 받으므로 도구 결과와 입력한 텍스트는 변경되지 않은 상태로 렌더링됩니다.

MessageDisplay는 matcher를 지원하지 않으며 텍스트를 스트리밍하는 모든 어시스턴트 메시지에 대해 발생합니다; 도구 호출 전용 응답과 같이 텍스트가 없는 메시지는 이를 트리거하지 않습니다.

비대화형 실행 (Agent SDK 쿼리 및 `claude -p` 포함)에서 MessageDisplay는 줄의 배치당 한 번이 아닌 어시스턴트 메시지당 한 번 실행됩니다. 단일 호출은 메시지가 완료된 후 도착하고 전체 메시지 텍스트를 전달합니다: `index`는 `0`, `final`은 `true`, `delta`는 전체 메시지를 보유합니다. 각 메시지에 대해 `delta` 텍스트를 수집하는 hook은 두 모드 모두에서 동일한 총 텍스트를 받습니다.

<h4 id="messagedisplay-input">
  MessageDisplay 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 MessageDisplay hook은 턴과 메시지의 식별자, 이 호출이 메시지 내에서의 위치, `delta`의 새 텍스트를 받습니다. 배치 경계는 텍스트가 스트리밍되는 방식에 따라 다르므로 줄이 특정 방식으로 그룹화될 것으로 예상하기보다는 `index` 및 `final`을 사용하여 메시지를 통한 진행 상황을 추적합니다.

| 필드           | 설명                                                                                                                                                                                                             |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `turn_id`    | 현재 턴의 UUID                                                                                                                                                                                                     |
| `message_id` | 표시되는 어시스턴트 메시지의 UUID. 메시지의 모든 배치에서 안정적입니다. 이는 API `msg_…` id가 아니므로 트랜스크립트 메시지 id와 상관관계를 지을 수 없습니다                                                                                                              |
| `index`      | 메시지 내 이 배치의 0 기반 인덱스                                                                                                                                                                                           |
| `final`      | 메시지의 마지막 배치에서 `true`. 각 메시지는 정확히 하나의 최종 배치를 가집니다                                                                                                                                                               |
| `delta`      | 이전 배치 이후의 새로 완료된 줄, 종료 줄바꿈 포함. 항상 전체 줄이며, 최종 배치는 줄 중간에 끝날 수 있습니다. 대화형 실행에서 메시지가 줄바꿈으로 끝나면 최종 배치의 delta는 비어 있으므로 비어 있지 않은 delta가 아닌 `final`을 메시지 끝 신호로 취급합니다. Agent SDK 및 `claude -p` 실행에서 단일 호출은 전체 메시지를 전달합니다 |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "MessageDisplay",
  "turn_id": "0c9e6a2f-7d41-4f4e-9a15-3f4f7c2b8d10",
  "message_id": "5b2a9c8e-1f63-4d8a-b7c4-9e0d2a6f1c3b",
  "index": 0,
  "final": false,
  "delta": "Here is the plan:\n"
}
```

<h4 id="messagedisplay-output">
  MessageDisplay 출력
</h4>

모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 MessageDisplay hook은 `displayContent`를 반환하여 화면의 delta를 바꿀 수 있습니다:

| 필드               | 설명                             |
| :--------------- | :----------------------------- |
| `displayContent` | delta 대신 표시되는 텍스트. 생략하여 원본을 표시 |

MessageDisplay hook은 결정 제어가 없습니다. 메시지를 차단하거나 트랜스크립트에 저장되거나 Claude에 전송되는 것을 변경할 수 없습니다.

이 예제는 Claude의 응답에서 markdown 형식을 제거하여 일반 텍스트 표시를 합니다. 스크립트는 stdin에서 각 배치를 읽고 `delta`에서 굵은 마커와 인라인 코드 백틱을 제거하고 결과를 `displayContent`로 반환합니다.

<Tabs>
  <Tab title="macOS/Linux">
    설정 파일에서 이벤트에 대한 명령 hook을 등록합니다:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    이 스크립트를 프로젝트의 `.claude/hooks/plain-display.sh`에 저장하고 `chmod +x`로 실행 가능하게 만듭니다:

    ```bash theme={null}
    #!/bin/bash
    jq '{hookSpecificOutput: {hookEventName: "MessageDisplay", displayContent: (.delta | gsub("\\*\\*"; "") | gsub("`"; ""))}}'
    ```

    스크립트는 `PATH`에 `jq`가 필요합니다.
  </Tab>

  <Tab title="Windows (PowerShell)">
    PowerShell을 통해 스크립트를 실행하는 명령 hook을 등록합니다:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    `-NoProfile` 플래그는 PowerShell 프로필 로드를 건너뛰어 hook이 빠르게 시작되도록 하고, `-ExecutionPolicy Bypass`는 PowerShell이 로컬 스크립트 파일을 실행하도록 합니다.

    이 스크립트를 프로젝트의 `.claude/hooks/plain-display.ps1`에 저장합니다:

    ```powershell theme={null}
    $batch = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $text = $batch.delta -replace '\*\*', '' -replace '`', ''
    @{
      hookSpecificOutput = @{
        hookEventName = "MessageDisplay"
        displayContent = $text
      }
    } | ConvertTo-Json
    ```
  </Tab>
</Tabs>

markdown이 없는 배치는 변경되지 않은 상태로 통과합니다. 스크립트가 실패하면 (예: `jq`가 누락된 경우) Claude Code는 원본 텍스트를 표시하고 [디버그 출력](#debug-hooks)에서만 실패를 기록하며 세션에서는 기록하지 않습니다.

<h3 id="pretooluse">
  PreToolUse
</h3>

Claude가 도구 매개변수를 생성한 후 도구 호출을 처리하기 전에 실행됩니다. 도구 이름에서 일치합니다: `Bash`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `WebFetch`, `WebSearch`, `AskUserQuestion`, `ExitPlanMode`, 모든 [MCP 도구 이름](#match-mcp-tools).

<Warning>
  PreToolUse는 Claude가 도구를 호출할 때만 실행됩니다. [프롬프트에서 `@`로 참조하는](/docs/ko/common-workflows#reference-files-and-directories) 파일은 도구 호출 없이 추가됩니다: Claude Code는 프롬프트를 구축하는 동안 해당 내용을 삽입하므로 `Read`와 일치하는 hook을 포함하여 PreToolUse hook이 발생하지 않습니다. 특정 경로를 `@` 참조에서 차단하려면 [`Read` 거부 규칙](/docs/ko/permissions#read-and-edit)을 대신 사용하세요.
</Warning>

[PreToolUse 결정 제어](#pretooluse-decision-control)를 사용하여 도구 사용을 허용, 거부, 요청 또는 연기합니다.

<h4 id="pretooluse-input">
  PreToolUse 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 PreToolUse hook은 `tool_name`, `tool_input`, `tool_use_id`를 받습니다. `tool_input` 필드는 도구에 따라 다릅니다:

<h5 id="bash">
  Bash
</h5>

셸 명령을 실행합니다.

| 필드                  | 유형  | 예제                 | 설명                                                                                       |
| :------------------ | :-- | :----------------- | :--------------------------------------------------------------------------------------- |
| `command`           | 문자열 | `"npm test"`       | 실행할 셸 명령                                                                                 |
| `description`       | 문자열 | `"Run test suite"` | 명령이 수행하는 작업의 선택적 설명                                                                      |
| `timeout`           | 숫자  | `120000`           | 선택적 시간 초과 (밀리초). [최대](/docs/ko/tools-reference#bash-tool-behavior) 이상의 값은 거부되지 않고 최대값으로 감소됩니다 |
| `run_in_background` | 부울  | `false`            | 명령을 백그라운드에서 실행할지 여부                                                                      |

<h5 id="write">
  Write
</h5>

파일을 생성하거나 덮어씁니다.

| 필드          | 유형  | 예제                    | 설명          |
| :---------- | :-- | :-------------------- | :---------- |
| `file_path` | 문자열 | `"/path/to/file.txt"` | 쓸 파일의 절대 경로 |
| `content`   | 문자열 | `"file content"`      | 파일에 쓸 내용    |

<h5 id="edit">
  Edit
</h5>

기존 파일의 문자열을 바꿉니다.

| 필드            | 유형  | 예제                    | 설명            |
| :------------ | :-- | :-------------------- | :------------ |
| `file_path`   | 문자열 | `"/path/to/file.txt"` | 편집할 파일의 절대 경로 |
| `old_string`  | 문자열 | `"original text"`     | 찾아 바꿀 텍스트     |
| `new_string`  | 문자열 | `"replacement text"`  | 대체 텍스트        |
| `replace_all` | 부울  | `false`               | 모든 발생을 바꿀지 여부 |

<h5 id="read">
  Read
</h5>

파일 내용을 읽습니다.

| 필드          | 유형  | 예제                    | 설명               |
| :---------- | :-- | :-------------------- | :--------------- |
| `file_path` | 문자열 | `"/path/to/file.txt"` | 읽을 파일의 절대 경로     |
| `offset`    | 숫자  | `10`                  | 읽기를 시작할 선택적 줄 번호 |
| `limit`     | 숫자  | `50`                  | 읽을 선택적 줄 수       |

<h5 id="glob">
  Glob
</h5>

glob 패턴과 일치하는 파일을 찾습니다.

| 필드        | 유형  | 예제               | 설명                            |
| :-------- | :-- | :--------------- | :---------------------------- |
| `pattern` | 문자열 | `"**/*.ts"`      | 파일과 일치시킬 glob 패턴              |
| `path`    | 문자열 | `"/path/to/dir"` | 검색할 선택적 디렉토리. 기본값은 현재 작업 디렉토리 |

<h5 id="grep">
  Grep
</h5>

정규식으로 파일 내용을 검색합니다.

| 필드            | 유형  | 예제               | 설명                                                                            |
| :------------ | :-- | :--------------- | :---------------------------------------------------------------------------- |
| `pattern`     | 문자열 | `"TODO.*fix"`    | 검색할 정규식 패턴                                                                    |
| `path`        | 문자열 | `"/path/to/dir"` | 검색할 선택적 파일 또는 디렉토리                                                            |
| `glob`        | 문자열 | `"*.ts"`         | 파일을 필터링할 선택적 glob 패턴                                                          |
| `output_mode` | 문자열 | `"content"`      | `"content"`, `"files_with_matches"` 또는 `"count"`. 기본값은 `"files_with_matches"` |
| `-i`          | 부울  | `true`           | 대소문자를 구분하지 않는 검색                                                              |
| `multiline`   | 부울  | `false`          | 다중 줄 일치 활성화                                                                   |

<h5 id="webfetch">
  WebFetch
</h5>

웹 콘텐츠를 가져오고 처리합니다.

| 필드       | 유형  | 예제                            | 설명                 |
| :------- | :-- | :---------------------------- | :----------------- |
| `url`    | 문자열 | `"https://example.com/api"`   | 콘텐츠를 가져올 URL       |
| `prompt` | 문자열 | `"Extract the API endpoints"` | 가져온 콘텐츠에서 실행할 프롬프트 |

<h5 id="websearch">
  WebSearch
</h5>

웹을 검색합니다.

| 필드                | 유형  | 예제                             | 설명                   |
| :---------------- | :-- | :----------------------------- | :------------------- |
| `query`           | 문자열 | `"react hooks best practices"` | 검색 쿼리                |
| `allowed_domains` | 배열  | `["docs.example.com"]`         | 선택적: 이러한 도메인의 결과만 포함 |
| `blocked_domains` | 배열  | `["spam.example.com"]`         | 선택적: 이러한 도메인의 결과 제외  |

<h5 id="agent">
  Agent
</h5>

[subagent](/docs/ko/sub-agents)를 생성합니다.

| 필드              | 유형  | 예제                         | 설명                  |
| :-------------- | :-- | :------------------------- | :------------------ |
| `prompt`        | 문자열 | `"Find all API endpoints"` | 에이전트가 수행할 작업        |
| `description`   | 문자열 | `"Find API endpoints"`     | 작업의 짧은 설명           |
| `subagent_type` | 문자열 | `"Explore"`                | 사용할 특화된 에이전트의 유형    |
| `model`         | 문자열 | `"sonnet"`                 | 기본값을 재정의할 선택적 모델 별칭 |

`PostToolUse`에서 완료된 Agent 호출의 `tool_response`는 subagent의 최종 텍스트와 사용 원격 측정을 전달합니다. hook에서 subagent별 비용을 기록하려면 이러한 필드를 읽으세요:

| 필드                  | 유형  | 예제                                                    | 설명                                                                                                                                                              |
| :------------------ | :-- | :---------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | 문자열 | `"completed"`                                         | `"completed"` (동기 호출의 경우), `"async_launched"` (백그라운드 subagent의 경우). v2.1.198부터 subagent는 기본적으로 백그라운드에서 실행되므로 생략된 `run_in_background`도 `"async_launched"`를 생성합니다 |
| `agentId`           | 문자열 | `"a4d2c8f1e0b3a297"`                                  | subagent 실행의 식별자                                                                                                                                                |
| `content`           | 배열  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | subagent의 최종 텍스트 블록                                                                                                                                             |
| `resolvedModel`     | 문자열 | `"claude-sonnet-4-5"`                                 | subagent가 실행된 모델. 요청된 모델과 다를 수 있습니다. Claude Code v2.1.174 이상 필요                                                                                                 |
| `totalTokens`       | 숫자  | `12450`                                               | subagent의 턴 전체에서 청구된 총 토큰                                                                                                                                       |
| `totalDurationMs`   | 숫자  | `48211`                                               | subagent 실행의 벽시계 기간                                                                                                                                             |
| `totalToolUseCount` | 숫자  | `7`                                                   | subagent가 수행한 도구 호출 수                                                                                                                                           |
| `usage`             | 객체  | `{"input_tokens": 8320, ...}`                         | 유형별 토큰 분석: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                            |

백그라운드 subagent의 경우 도구는 subagent를 시작한 후 즉시 반환되므로 `tool_response`는 사용 필드를 전달하지 않습니다. `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile`, `resolvedModel`이 있습니다.

`resolvedModel` 필드는 subagent가 실제로 실행되는 모델의 이름을 지정하며, 이는 `tool_input`의 `model` 값과 다를 수 있습니다. Claude Code v2.1.174 이상이 필요합니다.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

사용자에게 1\~4개의 객관식 질문을 합니다.

| 필드          | 유형 | 예제                                                                                                                 | 설명                                                                                                                         |
| :---------- | :- | :----------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| `questions` | 배열 | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | 제시할 질문, 각각 `question` 문자열, 짧은 `header`, `options` 배열, 선택적 `multiSelect` 플래그                                                |
| `answers`   | 객체 | `{"Which framework?": "React"}`                                                                                    | 선택적. 질문 텍스트를 선택한 옵션 레이블로 매핑합니다. 다중 선택 답변은 쉼표로 레이블을 결합합니다. Claude는 이 필드를 설정하지 않습니다. `updatedInput`을 통해 프로그래밍 방식으로 답변을 제공하세요 |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Claude가 [plan 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)를 떠나기 전에 계획을 제시하고 사용자에게 승인을 요청합니다. Claude는 도구를 호출하기 전에 계획을 파일에 디스크에 작성하므로 모델의 리터럴 `tool_input`은 일반적으로 비어 있습니다. Claude Code는 hook에 전달하기 전에 계획 내용과 파일 경로를 주입합니다.

| 필드               | 유형  | 예제                                          | 설명                                                                                                      |
| :--------------- | :-- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------ |
| `plan`           | 문자열 | `"## Refactor auth\n1. Extract..."`         | Markdown의 계획 내용. 디스크의 계획 파일에서 주입됨                                                                       |
| `planFilePath`   | 문자열 | `"/Users/.../plans/refactor-auth.md"`       | 계획 파일의 경로. 주입됨                                                                                          |
| `allowedPrompts` | 배열  | `[{"tool": "Bash", "prompt": "run tests"}]` | 더 이상 사용되지 않음. Claude Code는 필드를 수락하지만 무시합니다. v2.1.205 이전에는 Claude가 계획을 구현하기 위해 요청하는 prompt 기반 권한을 전달했습니다 |

`PostToolUse`에서 `tool_response`는 승인된 계획을 보유하는 `plan` 및 `filePath` 필드가 있는 객체이며, 내부 상태 플래그도 있습니다. 디스크에서 파일을 다시 읽는 대신 `tool_response.plan`에서 계획 내용을 읽으세요.

<h4 id="pretooluse-decision-control">
  PreToolUse 결정 제어
</h4>

`PreToolUse` hook은 도구 호출 진행 여부를 제어할 수 있습니다. 최상위 `decision` 필드를 사용하는 다른 hook과 달리 PreToolUse는 `hookSpecificOutput` 객체 내에 결정을 반환합니다. 이는 더 풍부한 제어를 제공합니다: 네 가지 결과 (허용, 거부, 요청 또는 연기) 및 실행 전에 도구 입력을 수정하는 기능.

| 필드                         | 설명                                                                                                                                                                                        |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"`는 권한 시스템을 우회합니다. `"deny"`는 도구 호출을 방지합니다. `"ask"`는 사용자에게 확인을 요청합니다. `"defer"`는 나중에 재개하도록 연기합니다. [권한 거부 및 요청 규칙](/docs/ko/permissions#manage-permissions)은 hook이 반환하는 것과 관계없이 여전히 평가됩니다 |
| `permissionDecisionReason` | `"allow"` 및 `"ask"`의 경우 사용자에게 표시되지만 Claude에는 표시되지 않습니다. `"deny"`의 경우 Claude에 표시됩니다. `"defer"`의 경우 무시됩니다                                                                                   |
| `updatedInput`             | 실행 전에 도구의 입력 매개변수를 수정합니다. 전체 입력 객체를 바꾸므로 변경되지 않은 필드를 수정된 필드와 함께 포함합니다. `"allow"`와 결합하여 자동 승인하거나 `"ask"`와 결합하여 수정된 입력을 사용자에게 표시합니다. `"defer"`의 경우 무시됩니다                                    |
| `additionalContext`        | 도구 결과와 함께 Claude의 컨텍스트에 추가되는 문자열. `"defer"`의 경우 무시됩니다. [Claude를 위한 컨텍스트 추가](#add-context-for-claude)를 참조하세요                                                                               |

여러 PreToolUse hook이 다른 결정을 반환할 때 우선순위는 `deny` > `defer` > `ask` > `allow`입니다.

hook이 `"ask"`를 반환하면 사용자에게 표시되는 권한 프롬프트에는 hook이 어디에서 왔는지를 나타내는 레이블이 포함됩니다: 예를 들어 `[User]`, `[Project]`, `[Plugin]` 또는 `[Local]`. 이는 사용자가 어느 구성 소스가 확인을 요청하는지 이해하는 데 도움이 됩니다.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "My reason here",
    "updatedInput": {
      "field_to_modify": "new value"
    },
    "additionalContext": "Current environment: production. Proceed with caution."
  }
}
```

`AskUserQuestion` 및 `ExitPlanMode`는 사용자 상호 작용이 필요하며 일반적으로 [비대화형 모드](/docs/ko/headless)에서 `-p` 플래그로 차단합니다. `permissionDecision: "allow"`를 `updatedInput`과 함께 반환하면 해당 요구 사항을 충족합니다: hook은 stdin에서 도구의 입력을 읽고 자신의 UI를 통해 답변을 수집하고 `updatedInput`에서 반환하여 도구가 프롬프트 없이 실행되도록 합니다. `"allow"`만 반환하는 것은 이러한 도구에 충분하지 않습니다. `AskUserQuestion`의 경우 원본 `questions` 배열을 에코백하고 각 질문의 텍스트를 선택한 답변으로 매핑하는 [`answers`](#askuserquestion) 객체를 추가합니다.

Connector 도구 ([조직이 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools))는 hook이 `"allow"`를 반환하더라도 프롬프트합니다.

v2.1.199부터 [`_meta["anthropic/requiresUserInteraction"]`](/docs/ko/mcp#require-approval-for-a-specific-tool)로 표시된 MCP 도구는 더 엄격합니다: hook은 `updatedInput`이 있거나 없이 `"allow"`로 승인 프롬프트를 건너뛸 수 없습니다. Claude Code는 hook이 도구가 필요한 상호 작용을 수집했는지 확인할 수 없기 때문입니다.

<Note>
  PreToolUse는 이전에 최상위 `decision` 및 `reason` 필드를 사용했지만 이 이벤트에는 더 이상 사용되지 않습니다. 대신 `hookSpecificOutput.permissionDecision` 및 `hookSpecificOutput.permissionDecisionReason`을 사용합니다. 더 이상 사용되지 않는 값 `"approve"` 및 `"block"`은 각각 `"allow"` 및 `"deny"`로 매핑됩니다. PostToolUse 및 Stop과 같은 다른 이벤트는 계속 최상위 `decision` 및 `reason`을 현재 형식으로 사용합니다.
</Note>

<h4 id="defer-a-tool-call-for-later">
  도구 호출을 나중에 재개하도록 연기
</h4>

`"defer"`는 Claude Code를 subprocess로 실행하고 JSON 출력을 읽는 Agent SDK 앱 또는 Claude Code 위에 구축된 사용자 정의 UI와 같은 통합을 위한 것입니다. 이를 통해 호출 프로세스가 Claude를 도구 호출에서 일시 중지하고 자신의 인터페이스를 통해 입력을 수집하고 중단된 위치에서 재개할 수 있습니다. Claude Code는 [비대화형 모드](/docs/ko/headless)에서 `-p` 플래그를 사용할 때만 이 값을 준수합니다. 대화형 세션에서는 경고를 기록하고 hook 결과를 무시합니다.

일반적인 경우는 `AskUserQuestion` 도구입니다: Claude가 사용자에게 뭔가를 묻고 싶지만 답변할 터미널이 없습니다. 왕복은 다음과 같이 작동합니다:

1. Claude가 `AskUserQuestion`을 호출합니다. `PreToolUse` hook이 발생합니다.
2. hook이 `permissionDecision: "defer"`를 반환합니다. 도구가 실행되지 않습니다. 프로세스는 `stop_reason: "tool_deferred"`로 종료되고 보류 중인 도구 호출이 트랜스크립트에 유지됩니다.
3. 호출 프로세스는 SDK 결과에서 `deferred_tool_use`를 읽고 자신의 UI에서 질문을 표시하고 답변을 기다립니다.
4. 호출 프로세스는 `claude -p --resume <session-id>`를 실행합니다. 동일한 도구 호출이 `PreToolUse`를 다시 발생시킵니다.
5. hook이 `permissionDecision: "allow"`를 `updatedInput`의 답변과 함께 반환합니다. 도구가 실행되고 Claude가 계속됩니다.

`deferred_tool_use` 필드는 도구의 `id`, `name`, `input`을 전달합니다. `input`은 실행 전에 캡처된 도구 호출을 위해 Claude가 생성한 매개변수입니다:

```json theme={null}
{
  "type": "result",
  "subtype": "success",
  "stop_reason": "tool_deferred",
  "session_id": "abc123",
  "deferred_tool_use": {
    "id": "toolu_01abc",
    "name": "AskUserQuestion",
    "input": { "questions": [{ "question": "Which framework?", "header": "Framework", "options": [{"label": "React"}, {"label": "Vue"}], "multiSelect": false }] }
  }
}
```

시간 초과 또는 재시도 제한이 없습니다. 세션은 재개할 때까지 디스크에 유지됩니다. 재개할 때 답변이 준비되지 않으면 hook이 `"defer"`를 다시 반환할 수 있고 프로세스는 동일한 방식으로 종료됩니다. 호출 프로세스는 결국 `"allow"` 또는 `"deny"`를 반환하여 루프를 끝낼 시기를 제어합니다.

`"defer"`는 Claude가 한 번에 단일 도구 호출을 만들 때만 작동합니다. Claude가 여러 도구 호출을 한 번에 만들면 `"defer"`는 경고와 함께 무시되고 도구는 일반 권한 흐름을 통해 진행됩니다. 제약이 존재하는 이유는 재개가 하나의 도구만 다시 실행할 수 있기 때문입니다: 다른 도구를 미해결 상태로 두지 않고 배치에서 하나의 호출을 연기할 방법이 없습니다.

연기된 도구가 재개할 때 더 이상 사용 가능하지 않으면 프로세스는 `stop_reason: "tool_deferred_unavailable"`과 `is_error: true`로 종료되고 hook이 발생하기 전에 종료됩니다. 이는 도구를 제공한 MCP 서버가 재개된 세션에 연결되지 않을 때 발생합니다. `deferred_tool_use` 페이로드는 여전히 포함되므로 어느 도구가 누락되었는지 식별할 수 있습니다.

<Note>
  `--resume`은 도구가 연기되었을 때 활성화된 권한 모드를 복원하므로 재개할 때 `--permission-mode`를 다시 전달할 필요가 없습니다. 예외는 `plan` 및 `bypassPermissions`이며, 이들은 절대 이월되지 않습니다. 재개할 때 `--permission-mode`를 명시적으로 전달하면 복원된 값을 재정의합니다.
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

사용자에게 권한 대화 상자가 표시될 때 실행됩니다.
[PermissionRequest 결정 제어](#permissionrequest-decision-control)를 사용하여 사용자를 대신하여 허용하거나 거부합니다.

도구 이름에서 일치합니다. PreToolUse와 동일한 값입니다.

<h4 id="permissionrequest-input">
  PermissionRequest 입력
</h4>

PermissionRequest hook은 PreToolUse hook과 같은 `tool_name` 및 `tool_input` 필드를 받지만 `tool_use_id`는 없습니다. 선택적 `permission_suggestions` 배열에는 사용자가 권한 대화 상자에서 일반적으로 볼 수 있는 "항상 허용" 옵션이 포함됩니다. 차이점은 hook이 발생할 때입니다: PermissionRequest hook은 권한 대화 상자가 사용자에게 표시되려고 할 때 실행되고, PreToolUse hook은 권한 상태와 관계없이 도구 실행 전에 실행됩니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PermissionRequest",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf node_modules",
    "description": "Remove node_modules directory"
  },
  "permission_suggestions": [
    {
      "type": "addRules",
      "rules": [{ "toolName": "Bash", "ruleContent": "rm -rf node_modules" }],
      "behavior": "allow",
      "destination": "localSettings"
    }
  ]
}
```

<h4 id="permissionrequest-decision-control">
  PermissionRequest 결정 제어
</h4>

`PermissionRequest` hook은 권한 요청을 허용하거나 거부할 수 있습니다. 모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 hook 스크립트는 이러한 이벤트 특정 필드가 있는 `decision` 객체를 반환할 수 있습니다:

| 필드                   | 설명                                                                                                                                                |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `behavior`           | `"allow"`는 권한을 부여하고, `"deny"`는 거부합니다. [권한 거부 및 요청 규칙](/docs/ko/permissions#manage-permissions)은 여전히 평가되므로 hook이 `"allow"`를 반환해도 일치하는 거부 규칙을 재정의하지 않습니다 |
| `updatedInput`       | `"allow"`만 해당: 실행 전에 도구의 입력 매개변수를 수정합니다. 전체 입력 객체를 바꾸므로 변경되지 않은 필드를 수정된 필드와 함께 포함합니다. 수정된 입력은 거부 및 요청 규칙에 대해 다시 평가됩니다                             |
| `updatedPermissions` | `"allow"`만 해당: 적용할 [권한 업데이트 항목](#permission-update-entries) 배열, 예를 들어 허용 규칙 추가 또는 세션 권한 모드 변경                                                     |
| `message`            | `"deny"`만 해당: Claude에 권한이 거부된 이유를 알립니다                                                                                                            |
| `interrupt`          | `"deny"`만 해당: `true`인 경우 Claude를 중지합니다                                                                                                            |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedInput": {
        "command": "npm run lint"
      }
    }
  }
}
```

<h4 id="permission-update-entries">
  권한 업데이트 항목
</h4>

`updatedPermissions` 출력 필드와 [`permission_suggestions` 입력 필드](#permissionrequest-input) 모두 동일한 항목 객체 배열을 사용합니다. 각 항목에는 다른 필드를 결정하는 `type`과 변경이 작성되는 위치를 제어하는 `destination`이 있습니다.

| `type`              | 필드                                 | 효과                                                                                                                                                                 |
| :------------------ | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `addRules`          | `rules`, `behavior`, `destination` | 권한 규칙을 추가합니다. `rules`는 `{toolName, ruleContent?}` 객체의 배열입니다. 전체 도구와 일치하려면 `ruleContent`를 생략합니다. `behavior`는 `"allow"`, `"deny"` 또는 `"ask"`입니다                      |
| `replaceRules`      | `rules`, `behavior`, `destination` | 주어진 `behavior`의 모든 규칙을 `destination`에서 제공된 `rules`로 바꿉니다                                                                                                           |
| `removeRules`       | `rules`, `behavior`, `destination` | 주어진 `behavior`의 일치하는 규칙을 제거합니다                                                                                                                                     |
| `setMode`           | `mode`, `destination`              | 권한 모드를 변경합니다. 유효한 모드는 `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan`, `manual` (기본값의 별칭)입니다. `manual` 별칭은 Claude Code v2.1.200 이상이 필요합니다 |
| `addDirectories`    | `directories`, `destination`       | 작업 디렉토리를 추가합니다. `directories`는 경로 문자열의 배열입니다                                                                                                                       |
| `removeDirectories` | `directories`, `destination`       | 작업 디렉토리를 제거합니다                                                                                                                                                     |

<Note>
  `setMode`와 `bypassPermissions`는 세션이 이미 bypass 모드를 사용 가능하게 시작된 경우에만 적용됩니다: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, 또는 설정의 `permissions.defaultMode: "bypassPermissions"`, 그리고 모드가 [`permissions.disableBypassPermissionsMode`](/docs/ko/permissions#managed-settings)에 의해 비활성화되지 않은 경우입니다. 그렇지 않으면 업데이트는 작동하지 않습니다. `bypassPermissions`는 `destination`과 관계없이 `defaultMode`로 절대 유지되지 않습니다.
</Note>

모든 항목의 `destination` 필드는 변경이 메모리에만 유지되는지 또는 설정 파일에 유지되는지를 결정합니다.

| `destination`     | 쓰기 대상                         |
| :---------------- | :---------------------------- |
| `session`         | 메모리 전용, 세션이 끝나면 삭제됨           |
| `localSettings`   | `.claude/settings.local.json` |
| `projectSettings` | `.claude/settings.json`       |
| `userSettings`    | `~/.claude/settings.json`     |

hook은 받은 `permission_suggestions` 중 하나를 자신의 `updatedPermissions` 출력으로 에코할 수 있으며, 이는 사용자가 대화 상자에서 해당 "항상 허용" 옵션을 선택하는 것과 동등합니다.

<h3 id="posttooluse">
  PostToolUse
</h3>

도구가 성공적으로 완료된 직후 실행됩니다.

도구 이름에서 일치합니다. PreToolUse와 동일한 값입니다.

<h4 id="posttooluse-input">
  PostToolUse 입력
</h4>

`PostToolUse` hook은 도구가 이미 성공적으로 실행된 후에 발생합니다. 입력에는 도구에 전송된 인수인 `tool_input`과 반환한 결과인 `tool_response`가 모두 포함됩니다. 둘 다의 정확한 스키마는 도구에 따라 다릅니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/file.txt",
    "content": "file content"
  },
  "tool_response": {
    "filePath": "/path/to/file.txt",
    "success": true
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| 필드            | 설명                                                        |
| :------------ | :-------------------------------------------------------- |
| `duration_ms` | 선택적. 도구 실행 시간 (밀리초). 권한 프롬프트 및 PreToolUse hook에 소요된 시간 제외 |

<h4 id="posttooluse-decision-control">
  PostToolUse 결정 제어
</h4>

`PostToolUse` hook은 도구 실행 후 Claude에 피드백을 제공할 수 있습니다. 모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 hook 스크립트는 이러한 이벤트 특정 필드를 반환할 수 있습니다:

| 필드                     | 설명                                                                                     |
| :--------------------- | :------------------------------------------------------------------------------------- |
| `decision`             | `"block"`은 Claude에 `reason`을 표시합니다. 생략하여 작업을 진행하도록 허용                                  |
| `reason`               | `decision`이 `"block"`일 때 Claude에 표시되는 설명                                               |
| `additionalContext`    | Claude의 컨텍스트에 도구 결과와 함께 추가되는 문자열. [Claude를 위한 컨텍스트 추가](#add-context-for-claude)를 참조하세요 |
| `updatedToolOutput`    | 도구의 출력을 제공된 값으로 바꿉니다. 값은 도구의 출력 형태와 일치해야 합니다                                           |
| `updatedMCPToolOutput` | [MCP 도구](#match-mcp-tools)만 해당: 도구의 출력을 제공된 값으로 바꿉니다                                   |

아래 예제는 `Bash` 호출의 출력을 바꿉니다. 대체 값은 `Bash` 도구의 출력 형태와 일치합니다:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Additional information for Claude",
    "updatedToolOutput": {
      "stdout": "[redacted]",
      "stderr": "",
      "interrupted": false,
      "isImage": false
    }
  }
}
```

<Warning>
  `updatedToolOutput`은 Claude가 보는 것만 변경합니다. 도구는 hook이 발생할 때까지 이미 실행되었으므로 작성된 파일, 실행된 명령 또는 전송된 네트워크 요청은 이미 적용되었습니다. OpenTelemetry 도구 span 및 분석 이벤트와 같은 원격 측정도 hook이 실행되기 전에 원본 출력을 캡처합니다. 도구 호출을 실행 전에 방지하거나 수정하려면 [PreToolUse](#pretooluse) hook을 대신 사용합니다.

  대체 값은 도구의 출력 형태와 일치해야 합니다. 기본 제공 도구는 일반 문자열이 아닌 구조화된 객체를 반환합니다. 예를 들어 `Bash`는 `stdout`, `stderr`, `interrupted`, `isImage` 필드가 있는 객체를 반환합니다. 기본 제공 도구의 경우 도구의 출력 스키마와 일치하지 않는 값은 무시되고 원본 출력이 사용됩니다. MCP 도구 출력은 스키마 검증 없이 통과됩니다. Claude가 필요한 오류 세부 정보를 제거하면 잘못된 가정으로 진행할 수 있습니다.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

도구 실행이 실패할 때 실행됩니다. 이 이벤트는 오류를 throw하거나 실패 결과를 반환하는 도구 호출에 대해 발생합니다. 이를 사용하여 실패를 기록하고, 경고를 보내거나, Claude에 수정 피드백을 제공합니다.

도구 이름에서 일치합니다. PreToolUse와 동일한 값입니다.

<Note>
  이 이벤트는 실행 전에 거부된 도구 호출에 대해 발생하지 않습니다: 알 수 없는 도구 이름, 스키마 또는 도구 특정 검증에 실패한 입력, 또는 권한 거부. 검증 거부는 `tool_use_error` 결과로 반환되고 hook이 실행되기 전에 발생하므로 `PreToolUse` 또는 이 이벤트를 발생시키지 않습니다. 권한 거부는 `PreToolUse`를 발생시키지만 이 이벤트는 발생시키지 않습니다; [PermissionDenied](#permissiondenied)를 참조하세요.
</Note>

<h4 id="posttoolusefailure-input">
  PostToolUseFailure 입력
</h4>

PostToolUseFailure hook은 PostToolUse와 동일한 `tool_name` 및 `tool_input` 필드를 받으며, 오류 정보는 최상위 필드로 받습니다:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUseFailure",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite"
  },
  "tool_use_id": "toolu_01ABC123...",
  "error": "Command exited with non-zero status code 1",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| 필드             | 설명                                                        |
| :------------- | :-------------------------------------------------------- |
| `error`        | 무엇이 잘못되었는지 설명하는 문자열                                       |
| `is_interrupt` | 선택적 부울로 실패가 사용자 중단으로 인한 것인지 여부를 나타냅니다                     |
| `duration_ms`  | 선택적. 도구 실행 시간 (밀리초). 권한 프롬프트 및 PreToolUse hook에 소요된 시간 제외 |

<h4 id="posttoolusefailure-decision-control">
  PostToolUseFailure 결정 제어
</h4>

`PostToolUseFailure` hook은 도구 실패 후 Claude에 컨텍스트를 제공할 수 있습니다. 모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 hook 스크립트는 이러한 이벤트 특정 필드를 반환할 수 있습니다:

| 필드                  | 설명                                                                                  |
| :------------------ | :---------------------------------------------------------------------------------- |
| `additionalContext` | Claude의 컨텍스트에 오류와 함께 추가되는 문자열. [Claude를 위한 컨텍스트 추가](#add-context-for-claude)를 참조하세요 |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUseFailure",
    "additionalContext": "Additional information about the failure for Claude"
  }
}
```

<h3 id="posttoolbatch">
  PostToolBatch
</h3>

배치의 모든 도구 호출이 해결된 후, Claude Code가 모델에 다음 요청을 보내기 전에 한 번 실행됩니다. `PostToolUse`는 도구당 한 번 발생하므로 Claude가 병렬 도구 호출을 만들 때 동시에 발생합니다. `PostToolBatch`는 전체 배치와 함께 정확히 한 번 발생하므로 단일 도구가 아닌 실행된 도구 집합에 따라 달라지는 컨텍스트를 주입하기에 적합한 위치입니다. 이 이벤트에는 matcher가 없습니다.

<h4 id="posttoolbatch-input">
  PostToolBatch 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 PostToolBatch hook은 배치의 모든 도구 호출을 설명하는 배열인 `tool_calls`를 받습니다:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolBatch",
  "tool_calls": [
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/accounts.py"},
      "tool_use_id": "toolu_01...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    },
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/transactions.py"},
      "tool_use_id": "toolu_02...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    }
  ]
}
```

`tool_response`는 모델이 해당 `tool_result` 블록에서 받는 것과 동일한 내용을 포함합니다. 값은 도구가 내보낸 것과 정확히 같은 직렬화된 문자열 또는 콘텐츠 블록 배열입니다. `Read`의 경우 원본 파일 내용이 아닌 줄 번호가 접두사로 붙은 텍스트를 의미합니다. 응답이 클 수 있으므로 필요한 필드만 구문 분석합니다.

<Note>
  `tool_response` 형태는 `PostToolUse`와 다릅니다. `PostToolUse`는 도구의 구조화된 `Output` 객체를 전달합니다 (예: `Write`의 경우 `{filePath: "...", success: true}`). `PostToolBatch`는 모델이 보는 직렬화된 `tool_result` 콘텐츠를 전달합니다.
</Note>

<h4 id="posttoolbatch-decision-control">
  PostToolBatch 결정 제어
</h4>

`PostToolBatch` hook은 Claude에 대한 컨텍스트를 주입할 수 있습니다. 모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 hook 스크립트는 이러한 이벤트 특정 필드를 반환할 수 있습니다:

| 필드                  | 설명                                                                                                                                |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | 다음 모델 호출 전에 한 번 주입되는 컨텍스트 문자열. [Claude를 위한 컨텍스트 추가](#add-context-for-claude)를 참조하여 전달 세부 정보, 포함할 내용, 재개된 세션이 과거 값을 처리하는 방식을 확인하세요 |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

`decision: "block"` 또는 `continue: false`를 반환하면 다음 모델 호출 전에 에이전트 루프가 중지됩니다.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

[자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) 분류기가 도구 호출을 거부할 때 실행됩니다. 이 hook은 자동 모드에서만 발생합니다: 권한 대화 상자를 수동으로 거부할 때, `PreToolUse` hook이 호출을 차단할 때, 또는 `deny` 규칙이 일치할 때 실행되지 않습니다. 이를 사용하여 분류기 거부를 기록하고, 구성을 조정하거나, 모델이 도구 호출을 재시도할 수 있음을 알립니다.

도구 이름에서 일치합니다. PreToolUse와 동일한 값입니다.

<h4 id="permissiondenied-input">
  PermissionDenied 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 PermissionDenied hook은 `tool_name`, `tool_input`, `tool_use_id`, `reason`을 받습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "auto",
  "hook_event_name": "PermissionDenied",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf /tmp/build",
    "description": "Clean build directory"
  },
  "tool_use_id": "toolu_01ABC123...",
  "reason": "Auto mode denied: command targets a path outside the project"
}
```

| 필드       | 설명                        |
| :------- | :------------------------ |
| `reason` | 분류기가 도구 호출을 거부한 이유에 대한 설명 |

<h4 id="permissiondenied-decision-control">
  PermissionDenied 결정 제어
</h4>

PermissionDenied hook은 모델이 거부된 도구 호출을 재시도할 수 있음을 알릴 수 있습니다. `hookSpecificOutput.retry`를 `true`로 설정한 JSON 객체를 반환합니다:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

`retry`가 `true`일 때 Claude Code는 모델이 도구 호출을 재시도할 수 있음을 알리는 메시지를 대화에 추가합니다. 거부 자체는 역전되지 않습니다. hook이 JSON을 반환하지 않거나 `retry: false`를 반환하면 거부가 유지되고 모델은 원래 거부 메시지를 받습니다.

<h3 id="notification">
  Notification
</h3>

Claude Code가 알림을 보낼 때 실행됩니다. 알림 유형에서 일치합니다. 생략하여 모든 알림 유형에 대해 hook을 실행합니다.

| Matcher                | 언제 발생하는지                                                                |
| :--------------------- | :---------------------------------------------------------------------- |
| `permission_prompt`    | Claude가 도구 사용 승인이 필요함                                                   |
| `idle_prompt`          | Claude가 완료되고 다음 프롬프트를 기다림                                               |
| `auth_success`         | 인증 완료                                                                   |
| `elicitation_dialog`   | MCP 서버가 elicitation 양식을 열음                                              |
| `elicitation_complete` | MCP elicitation 양식이 제출되거나 닫힘                                            |
| `elicitation_response` | MCP elicitation 응답이 서버로 다시 전송됨                                          |
| `agent_needs_input`    | 백그라운드 세션이 입력을 기다리기 시작함. [agent view](/docs/ko/agent-view)가 터미널에서 열려 있을 때만 발생 |
| `agent_completed`      | 백그라운드 세션이 완료되거나 실패함. [agent view](/docs/ko/agent-view)가 터미널에서 열려 있을 때만 발생    |

`agent_needs_input` 및 `agent_completed` 유형은 Claude Code v2.1.198 이상이 필요합니다.

별도의 matcher를 사용하여 알림 유형에 따라 다른 핸들러를 실행합니다. 이 구성은 Claude가 권한 승인이 필요할 때 권한 특정 경고 스크립트를 트리거하고 Claude가 유휴 상태일 때 다른 알림을 트리거합니다:

```json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/permission-alert.sh"
          }
        ]
      },
      {
        "matcher": "idle_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/idle-notification.sh"
          }
        ]
      }
    ]
  }
}
```

<h4 id="notification-input">
  Notification 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 Notification hook은 알림 텍스트가 있는 `message`, 선택적 `title`, 발생한 유형을 나타내는 `notification_type`을 받습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Notification",
  "message": "Claude needs your permission",
  "title": "Permission needed",
  "notification_type": "permission_prompt"
}
```

Notification hook은 알림을 차단하거나 수정할 수 없습니다. 이들은 외부 서비스로 알림을 전달하는 것과 같은 부작용을 위한 것입니다. 모든 hook에 사용 가능한 [JSON 출력 필드](#json-output)가 적용됩니다.

<h3 id="subagentstart">
  SubagentStart
</h3>

Agent 도구를 통해 Claude Code subagent가 생성될 때 실행됩니다. 에이전트 유형 이름으로 필터링할 matcher를 지원합니다. 기본 제공 에이전트의 경우 이는 `general-purpose`, `Explore`, `Plan`과 같은 에이전트 이름입니다. [사용자 정의 subagent](/docs/ko/sub-agents)의 경우 이는 파일명이 아닌 에이전트의 frontmatter의 `name` 필드입니다.

[plugin](/docs/ko/plugins)에서 제공하는 subagent의 경우 에이전트 유형은 `my-plugin:reviewer`와 같은 plugin 범위 식별자이며, 파일명이 아닙니다. 콜론은 plugin 범위 이름을 정규식 경로에 배치하므로 정확한 일치를 위해 matcher를 `^` 및 `$`로 고정합니다: `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  SubagentStart 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 SubagentStart hook은 subagent의 고유 식별자가 있는 `agent_id`와 에이전트 이름이 있는 `agent_type`을 받습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SubagentStart",
  "agent_id": "agent-abc123",
  "agent_type": "Explore"
}
```

SubagentStart hook은 subagent 생성을 차단할 수 없지만 subagent에 컨텍스트를 주입할 수 있습니다. 모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 다음을 반환할 수 있습니다:

| 필드                  | 설명                                                                                                     |
| :------------------ | :----------------------------------------------------------------------------------------------------- |
| `additionalContext` | subagent의 대화 시작 부분에 추가되는 문자열. 첫 번째 프롬프트 전에 추가됩니다. [Claude를 위한 컨텍스트 추가](#add-context-for-claude)를 참조하세요 |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

<h3 id="subagentstop">
  SubagentStop
</h3>

Claude Code subagent가 응답을 마쳤을 때 실행됩니다. 에이전트 유형에서 일치합니다. SubagentStart와 동일한 값입니다.

<h4 id="subagentstop-input">
  SubagentStop 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 SubagentStop hook은 `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path`, `last_assistant_message`를 받습니다. `agent_type` 필드는 matcher 필터링에 사용되는 값입니다. `transcript_path`는 메인 세션의 트랜스크립트이고 `agent_transcript_path`는 중첩된 `subagents/` 폴더에 저장된 subagent의 자체 트랜스크립트입니다. `last_assistant_message` 필드는 subagent의 최종 응답의 텍스트 내용을 포함하므로 hook은 트랜스크립트 파일을 구문 분석하지 않고도 액세스할 수 있습니다.

SubagentStop hook은 또한 [Stop 입력](#stop-input)에서 설명한 `background_tasks` 및 `session_crons` 배열을 받으며, Claude Code v2.1.145 이상에서 사용 가능합니다. 두 배열 모두 subagent가 아닌 부모 세션으로 범위가 지정됩니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../abc123.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "SubagentStop",
  "stop_hook_active": false,
  "agent_id": "def456",
  "agent_type": "Explore",
  "agent_transcript_path": "~/.claude/projects/.../abc123/subagents/agent-def456.jsonl",
  "last_assistant_message": "Analysis complete. Found 3 potential issues...",
  "background_tasks": [],
  "session_crons": []
}
```

SubagentStop hook은 [Stop hook](#stop-decision-control)과 동일한 결정 제어 형식을 사용합니다. 이들은 `hookSpecificOutput.additionalContext`를 지원하며 `hookEventName`을 `"SubagentStop"`으로 설정하여 subagent를 계속 실행하는 비오류 피드백을 제공합니다. `decision: "block"`을 `reason`과 함께 반환하면 subagent가 계속 실행되고 `reason`이 subagent의 다음 명령으로 전달됩니다. subagent가 반환한 후 부모 세션에 컨텍스트를 주입하려면 `Agent` 도구에서 [`PostToolUse`](#posttooluse) hook을 대신 사용합니다.

<h3 id="taskcreated">
  TaskCreated
</h3>

작업이 `TaskCreate` 도구를 통해 생성될 때 실행됩니다. 이를 사용하여 명명 규칙을 적용하거나, 작업 설명을 요구하거나, 특정 작업이 생성되는 것을 방지합니다.

`TaskCreated` hook이 코드 2로 종료되면 작업이 생성되지 않고 stderr 메시지가 모델에 피드백으로 피드백됩니다. 팀원을 다시 실행하는 대신 완전히 중지하려면 `{"continue": false, "stopReason": "..."}`이 있는 JSON을 반환합니다. TaskCreated hook은 matcher를 지원하지 않으며 모든 발생에서 발생합니다.

<h4 id="taskcreated-input">
  TaskCreated 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 TaskCreated hook은 `task_id`, `task_subject`, 선택적으로 `task_description`, `teammate_name`, `team_name`을 받습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| 필드                 | 설명                       |
| :----------------- | :----------------------- |
| `task_id`          | 생성되는 작업의 식별자             |
| `task_subject`     | 작업의 제목                   |
| `task_description` | 작업의 자세한 설명. 없을 수 있음      |
| `teammate_name`    | 작업을 생성하는 팀원의 이름. 없을 수 있음 |
| `team_name`        | 팀의 이름. 없을 수 있음           |

<h4 id="taskcreated-decision-control">
  TaskCreated 결정 제어
</h4>

TaskCreated hook은 작업 생성을 제어하는 두 가지 방법을 지원합니다:

* **종료 코드 2**: 작업이 생성되지 않고 stderr 메시지가 모델에 피드백으로 피드백됩니다.
* **JSON `{"continue": false, "stopReason": "..."}`**: 팀원을 완전히 중지하여 `Stop` hook 동작과 일치합니다. `stopReason`은 사용자에게 표시됩니다.

이 예제는 제목이 필수 형식을 따르지 않는 작업을 차단합니다:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

if [[ ! "$TASK_SUBJECT" =~ ^\[TICKET-[0-9]+\] ]]; then
  echo "Task subject must start with a ticket number, e.g. '[TICKET-123] Add feature'" >&2
  exit 2
fi

exit 0
```

<h3 id="taskcompleted">
  TaskCompleted
</h3>

작업이 완료로 표시될 때 실행됩니다. 이는 두 가지 상황에서 발생합니다: 모든 에이전트가 TaskUpdate 도구를 통해 명시적으로 작업을 완료로 표시할 때 또는 [에이전트 팀](/docs/ko/agent-teams) 팀원이 진행 중인 작업으로 자신의 턴을 마칠 때입니다. 이를 사용하여 테스트 통과 또는 lint 검사와 같은 완료 기준을 적용하기 전에 작업을 닫을 수 있습니다.

`TaskCompleted` hook이 코드 2로 종료되면 작업이 완료로 표시되지 않고 stderr 메시지가 모델에 피드백으로 피드백됩니다. 팀원을 다시 실행하는 대신 완전히 중지하려면 `{"continue": false, "stopReason": "..."}`이 있는 JSON을 반환합니다. TaskCompleted hook은 matcher를 지원하지 않으며 모든 발생에서 발생합니다.

<h4 id="taskcompleted-input">
  TaskCompleted 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 TaskCompleted hook은 `task_id`, `task_subject`, 선택적으로 `task_description`, `teammate_name`, `team_name`을 받습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCompleted",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| 필드                 | 설명                       |
| :----------------- | :----------------------- |
| `task_id`          | 완료되는 작업의 식별자             |
| `task_subject`     | 작업의 제목                   |
| `task_description` | 작업의 자세한 설명. 없을 수 있음      |
| `teammate_name`    | 작업을 완료하는 팀원의 이름. 없을 수 있음 |
| `team_name`        | 팀의 이름. 없을 수 있음           |

<h4 id="taskcompleted-decision-control">
  TaskCompleted 결정 제어
</h4>

TaskCompleted hook은 작업 완료를 제어하는 두 가지 방법을 지원합니다:

* **종료 코드 2**: 작업이 완료로 표시되지 않고 stderr 메시지가 모델에 피드백으로 피드백됩니다.
* **JSON `{"continue": false, "stopReason": "..."}`**: 팀원을 완전히 중지하여 `Stop` hook 동작과 일치합니다. `stopReason`은 사용자에게 표시됩니다.

이 예제는 테스트를 실행하고 실패하면 작업 완료를 차단합니다:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# 테스트 스위트를 실행합니다
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

메인 Claude Code 에이전트가 응답을 마쳤을 때 실행됩니다. 중지가 사용자 중단으로 인해 발생한 경우 실행되지 않습니다. API 오류는 [StopFailure](#stopfailure) 대신 발생합니다.

<Tip>
  [`/goal`](/docs/ko/goal) 명령은 세션 범위 prompt 기반 Stop hook의 기본 제공 바로 가기입니다. 조건이 유지될 때까지 Claude가 계속 작동하도록 하되 hook 구성을 작성하지 않으려는 경우 사용합니다.
</Tip>

<h4 id="stop-input">
  Stop 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 Stop hook은 `stop_hook_active`, `last_assistant_message`, `background_tasks`, `session_crons`를 받습니다. `stop_hook_active` 필드는 Claude Code가 이미 stop hook의 결과로 계속되고 있을 때 `true`입니다. 이 값을 확인하거나 트랜스크립트를 처리하여 Claude Code가 무한정 실행되는 것을 방지합니다. Claude Code는 8번 연속 차단 후 hook을 재정의하고 턴을 종료합니다.

`last_assistant_message` 필드는 Claude의 최종 응답의 텍스트 내용을 포함하므로 hook은 트랜스크립트 파일을 구문 분석하지 않고도 액세스할 수 있습니다. 방금 완료된 턴에 대해 작동하는 hook (예: 읽기 전용 또는 알림 hook)의 경우 트랜스크립트 파일에서 읽는 대신 이 필드를 사용하세요: 트랜스크립트 파일은 모든 버전에서 Stop 시간에 최종 메시지를 포함하도록 보장되지 않습니다.

Claude Code v2.1.145 이상에서 사용 가능한 `background_tasks` 및 `session_crons` 배열을 통해 hook은 "세션이 완료됨"과 "세션이 백그라운드 작업이 깨어날 때까지 일시 중지됨"을 구분할 수 있습니다. 작업 레지스트리에 도달할 수 있을 때 두 배열이 모두 존재하며, 진행 중이거나 예약된 것이 없을 때 비어 있습니다.

`background_tasks`의 각 항목은 하나의 진행 중인 작업을 설명하며 이러한 필드를 사용합니다:

| 필드            | 설명                                                                                                                                                                            |
| :------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | 작업 식별자                                                                                                                                                                        |
| `type`        | 친화적 작업 유형 레이블 (예: `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session`, `MCP task`). 각 레이블은 어느 Claude Code 기능이 작업을 생성했는지 식별합니다. 인식되지 않는 유형의 경우 원본 판별식으로 폴백 |
| `status`      | 현재 작업 상태                                                                                                                                                                      |
| `description` | 자유 텍스트 설명, 1000자로 제한되며 잘린 경우 문자열 내 `… [+N chars]` 마커 포함                                                                                                                       |
| `command`     | 셸 명령줄, 1000자로 제한. `shell` 작업에만 존재                                                                                                                                             |
| `agent_type`  | Subagent 유형 이름. `subagent` 작업에만 존재                                                                                                                                            |
| `server`      | MCP 서버 이름. `monitor` 및 `MCP task` 작업에만 존재                                                                                                                                     |
| `tool`        | MCP 도구 이름. `monitor` 및 `MCP task` 작업에만 존재                                                                                                                                     |
| `name`        | 워크플로우 이름. `workflow` 작업에만 존재                                                                                                                                                  |

`session_crons`의 각 항목은 `CronCreate`, `ScheduleWakeup`, `/loop`에서 소싱된 하나의 세션 범위 예약된 깨어남을 설명합니다:

| 필드          | 설명                                                                     |
| :---------- | :--------------------------------------------------------------------- |
| `id`        | Cron 작업 식별자                                                            |
| `schedule`  | Cron 표현식 (예: `0 9 * * 1-5`)                                            |
| `recurring` | 일회성 깨어남의 경우 `false` (일정이 단일 발생 시간을 인코딩), 모든 일치에서 다시 발생하는 작업의 경우 `true` |
| `prompt`    | cron이 발생할 때 제출되는 프롬프트, 1000자로 제한되며 동일한 `… [+N chars]` 마커 포함            |

이 예제는 하나의 진행 중인 셸 작업과 하나의 반복 cron이 있는 Stop 입력을 보여줍니다:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Stop",
  "stop_hook_active": true,
  "last_assistant_message": "I've completed the refactoring. Here's a summary...",
  "background_tasks": [
    {
      "id": "task-001",
      "type": "shell",
      "status": "running",
      "description": "tail logs",
      "command": "tail -f /var/log/syslog"
    }
  ],
  "session_crons": [
    {
      "id": "cron-001",
      "schedule": "0 9 * * 1-5",
      "recurring": true,
      "prompt": "check the build"
    }
  ]
}
```

<h4 id="stop-decision-control">
  Stop 결정 제어
</h4>

`Stop` 및 `SubagentStop` hook은 Claude가 계속할지 여부를 제어할 수 있습니다. 모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 hook 스크립트는 이러한 이벤트 특정 필드를 반환할 수 있습니다:

| 필드                                     | 설명                                                                                                             |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------- |
| `decision`                             | `"block"`은 Claude가 중지되는 것을 방지합니다. 생략하여 Claude가 중지하도록 허용                                                        |
| `reason`                               | Claude가 중지되는 것이 차단될 때 필수입니다. Claude에 계속해야 하는 이유를 알립니다                                                          |
| `hookSpecificOutput.additionalContext` | 비오류 피드백 Claude. 대화가 계속되므로 Claude가 이에 따라 행동할 수 있지만 `decision: "block"`과 달리 트랜스크립트에 hook 오류가 아닌 hook 피드백으로 표시됩니다 |

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

`additionalContext`를 사용하면 hook이 설계대로 작동하고 Claude에 지침을 제공할 때 (예: "완료하기 전에 테스트 스위트를 실행하세요"). 대화를 `decision: "block"`과 동일한 루프 보호를 통해 계속하지만 트랜스크립트는 이를 `Stop hook feedback`으로 표시하고 hook 오류 알림이 표시되지 않습니다:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Stop",
    "additionalContext": "Please run the test suite before finishing"
  }
}
```

<h3 id="stopfailure">
  StopFailure
</h3>

[Stop](#stop) 대신 턴이 API 오류로 인해 종료될 때 실행됩니다. 출력과 종료 코드는 무시됩니다. 이를 사용하여 실패를 기록하고, 경고를 보내거나, Claude가 API 오류로 인해 응답을 완료할 수 없을 때 복구 조치를 취합니다.

<h4 id="stopfailure-input">
  StopFailure 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 StopFailure hook은 `error`, 선택적 `error_details`, 선택적 `last_assistant_message`를 받습니다. `error` 필드는 오류 유형을 식별하며 matcher 필터링에 사용됩니다.

| 필드                       | 설명                                                                                                                                                                                           |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`                  | 오류 유형: `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens` 또는 `unknown` |
| `error_details`          | 사용 가능한 경우 오류에 대한 추가 세부 정보                                                                                                                                                                    |
| `last_assistant_message` | 대화에 표시되는 렌더링된 오류 텍스트. `Stop` 및 `SubagentStop`과 달리 이 필드는 Claude의 대화형 출력을 보유하고 `StopFailure`의 경우 `"API Error: Rate limit reached"`와 같은 API 오류 문자열 자체를 포함합니다                                    |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "StopFailure",
  "error": "rate_limit",
  "error_details": "429 Too Many Requests",
  "last_assistant_message": "API Error: Rate limit reached"
}
```

StopFailure hook은 결정 제어가 없습니다. 이들은 알림 및 로깅 목적으로만 실행됩니다.

<h3 id="teammateidle">
  TeammateIdle
</h3>

[에이전트 팀](/docs/ko/agent-teams) 팀원이 자신의 턴을 마친 후 유휴 상태가 되려고 할 때 실행됩니다. 이를 사용하여 lint 검사 통과 또는 출력 파일 존재 확인과 같은 팀원이 작업을 중지하기 전에 품질 게이트를 적용합니다.

`TeammateIdle` hook이 코드 2로 종료되면 팀원은 stderr 메시지를 피드백으로 받고 유휴 상태가 되는 대신 계속 작업합니다. 팀원을 다시 실행하는 대신 완전히 중지하려면 `{"continue": false, "stopReason": "..."}`이 있는 JSON을 반환합니다. TeammateIdle hook은 matcher를 지원하지 않으며 모든 발생에서 발생합니다.

<h4 id="teammateidle-input">
  TeammateIdle 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 TeammateIdle hook은 `teammate_name` 및 `team_name`을 받습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TeammateIdle",
  "teammate_name": "researcher",
  "team_name": "session-a1b2c3d4"
}
```

| 필드              | 설명                   |
| :-------------- | :------------------- |
| `teammate_name` | 유휴 상태가 되려고 하는 팀원의 이름 |
| `team_name`     | 팀의 이름                |

<h4 id="teammateidle-decision-control">
  TeammateIdle 결정 제어
</h4>

TeammateIdle hook은 팀원 동작을 제어하는 두 가지 방법을 지원합니다:

* **종료 코드 2**: 팀원은 stderr 메시지를 피드백으로 받고 유휴 상태가 되는 대신 계속 작업합니다.
* **JSON `{"continue": false, "stopReason": "..."}`**: 팀원을 완전히 중지하여 `Stop` hook 동작과 일치합니다. `stopReason`은 사용자에게 표시됩니다.

이 예제는 팀원이 유휴 상태가 되도록 허용하기 전에 빌드 아티팩트가 존재하는지 확인합니다:

```bash theme={null}
#!/bin/bash

if [ ! -f "./dist/output.js" ]; then
  echo "Build artifact missing. Run the build before stopping." >&2
  exit 2
fi

exit 0
```

<h3 id="configchange">
  ConfigChange
</h3>

세션 중에 구성 파일이 변경될 때 실행됩니다. 이를 사용하여 설정 변경을 감사하고, 보안 정책을 적용하거나, 구성 파일에 대한 무단 수정을 차단합니다.

ConfigChange hook은 설정 파일, 관리형 정책 설정, skill 파일의 변경에 대해 발생합니다. 입력의 `source` 필드는 어떤 유형의 구성이 변경되었는지 알려주고, 선택적 `file_path` 필드는 변경된 파일의 경로를 제공합니다.

matcher는 구성 소스에서 필터링합니다:

| Matcher            | 언제 발생하는지                         |
| :----------------- | :------------------------------- |
| `user_settings`    | `~/.claude/settings.json` 변경     |
| `project_settings` | `.claude/settings.json` 변경       |
| `local_settings`   | `.claude/settings.local.json` 변경 |
| `policy_settings`  | 관리형 정책 설정 변경                     |
| `skills`           | `.claude/skills/`의 skill 파일 변경   |

이 예제는 보안 감사를 위해 모든 구성 변경을 기록합니다:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/audit-config-change.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

<h4 id="configchange-input">
  ConfigChange 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 ConfigChange hook은 `source` 및 선택적으로 `file_path`를 받습니다. `source` 필드는 어떤 구성 유형이 변경되었는지 나타내고 `file_path`는 수정된 특정 파일의 경로를 제공합니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ConfigChange",
  "source": "project_settings",
  "file_path": "/Users/.../my-project/.claude/settings.json"
}
```

<h4 id="configchange-decision-control">
  ConfigChange 결정 제어
</h4>

ConfigChange hook은 구성 변경이 적용되는 것을 차단할 수 있습니다. 종료 코드 2 또는 JSON `decision`을 사용하여 변경을 방지합니다. 차단되면 새 설정이 실행 중인 세션에 적용되지 않습니다.

| 필드         | 설명                                           |
| :--------- | :------------------------------------------- |
| `decision` | `"block"`은 구성 변경이 적용되는 것을 방지합니다. 생략하여 변경을 허용 |
| `reason`   | `decision`이 `"block"`일 때 사용자에게 표시되는 설명       |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

`policy_settings` 변경은 차단할 수 없습니다. Hook은 여전히 `policy_settings` 소스에 대해 발생하므로 감사 로깅에 사용할 수 있지만 모든 차단 결정은 무시됩니다. 이는 엔터프라이즈 관리 설정이 항상 적용되도록 보장합니다.

<h3 id="cwdchanged">
  CwdChanged
</h3>

세션 중에 작업 디렉토리가 변경될 때 실행됩니다. 예를 들어 Claude가 `cd` 명령을 실행할 때입니다. 이를 사용하여 디렉토리 변경에 반응합니다: 환경 변수를 다시 로드하고, 프로젝트 특정 도구 체인을 활성화하거나, 설정 스크립트를 자동으로 실행합니다. [FileChanged](#filechanged)와 쌍을 이루어 [direnv](https://direnv.net/)와 같은 디렉토리별 환경을 관리하는 도구를 사용합니다.

CwdChanged hook은 `CLAUDE_ENV_FILE`에 액세스할 수 있습니다. 해당 파일에 작성된 변수는 [SessionStart hook](#persist-environment-variables)과 마찬가지로 세션의 후속 Bash 명령에 유지됩니다.

CwdChanged는 matcher를 지원하지 않으며 모든 디렉토리 변경에서 발생합니다.

<h4 id="cwdchanged-input">
  CwdChanged 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 CwdChanged hook은 `old_cwd` 및 `new_cwd`를 받습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project/src",
  "hook_event_name": "CwdChanged",
  "old_cwd": "/Users/my-project",
  "new_cwd": "/Users/my-project/src"
}
```

<h4 id="cwdchanged-output">
  CwdChanged 출력
</h4>

모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 CwdChanged hook은 `watchPaths`를 반환하여 [FileChanged](#filechanged)가 감시하는 파일 경로를 동적으로 설정할 수 있습니다:

| 필드           | 설명                                                                                        |
| :----------- | :---------------------------------------------------------------------------------------- |
| `watchPaths` | 절대 경로의 배열. 현재 동적 감시 목록을 바꿉니다 (matcher 구성의 경로는 항상 감시됨). 새 디렉토리에 들어갈 때 빈 배열을 반환하는 것이 일반적입니다 |

CwdChanged hook은 결정 제어가 없습니다. 디렉토리 변경을 차단할 수 없습니다.

<h3 id="filechanged">
  FileChanged
</h3>

감시된 파일이 디스크에서 변경될 때 실행됩니다. 프로젝트 구성 파일이 수정될 때 환경 변수를 다시 로드하는 데 유용합니다.

`matcher`는 이 이벤트에 대해 두 가지 역할을 합니다:

* **감시 목록 구축**: 값은 `|`로 분할되고 각 세그먼트는 작업 디렉토리의 리터럴 파일명으로 등록되므로 `".envrc|.env"`는 정확히 이 두 파일을 감시합니다. 정규식 패턴은 여기서 유용하지 않습니다: `^\.env`와 같은 값은 `^\.env`라는 리터럴 이름의 파일을 감시합니다.
* **hook 실행 필터링**: 감시된 파일이 변경되면 동일한 값이 표준 [matcher 규칙](#matcher-patterns)을 사용하여 변경된 파일의 basename에 대해 실행할 hook 그룹을 필터링합니다.

FileChanged hook은 `CLAUDE_ENV_FILE`에 액세스할 수 있습니다. 해당 파일에 작성된 변수는 [SessionStart hook](#persist-environment-variables)과 마찬가지로 세션의 후속 Bash 명령에 유지됩니다.

<h4 id="filechanged-input">
  FileChanged 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 FileChanged hook은 `file_path` 및 `event`를 받습니다.

| 필드          | 설명                                                               |
| :---------- | :--------------------------------------------------------------- |
| `file_path` | 변경된 파일의 절대 경로                                                    |
| `event`     | 발생한 일: `"change"` (파일 수정), `"add"` (파일 생성) 또는 `"unlink"` (파일 삭제) |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "FileChanged",
  "file_path": "/Users/my-project/.envrc",
  "event": "change"
}
```

<h4 id="filechanged-output">
  FileChanged 출력
</h4>

모든 hook에 사용 가능한 [JSON 출력 필드](#json-output) 외에도 FileChanged hook은 `watchPaths`를 반환하여 감시되는 파일 경로를 동적으로 업데이트할 수 있습니다:

| 필드           | 설명                                                                                                    |
| :----------- | :---------------------------------------------------------------------------------------------------- |
| `watchPaths` | 절대 경로의 배열. 현재 동적 감시 목록을 바꿉니다 (matcher 구성의 경로는 항상 감시됨). hook 스크립트가 변경된 파일을 기반으로 감시할 추가 파일을 발견할 때 사용합니다 |

FileChanged hook은 결정 제어가 없습니다. 파일 변경을 차단할 수 없습니다.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

`claude --worktree`를 실행하거나 [subagent가 `isolation: "worktree"`를 사용](/docs/ko/sub-agents#choose-the-subagent-scope)할 때 Claude Code는 `git worktree`를 사용하여 격리된 작업 복사본을 생성합니다. WorktreeCreate hook을 구성하면 기본 git 동작을 대체하여 SVN, Perforce 또는 Mercurial과 같은 다른 버전 제어 시스템을 사용할 수 있습니다.

hook은 생성된 worktree 디렉토리의 절대 경로를 반환해야 합니다. Claude Code는 이 경로를 격리된 세션의 작업 디렉토리로 사용합니다. 명령 hook은 stdout에 경로를 인쇄합니다; HTTP hook은 `hookSpecificOutput.worktreePath`를 반환합니다.

이 예제는 SVN 작업 복사본을 생성하고 Claude Code가 사용할 경로를 인쇄합니다. 리포지토리 URL을 자신의 것으로 바꾸세요:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

hook은 stdin의 JSON 입력에서 worktree `name`을 읽고, 새 디렉토리로 신선한 복사본을 체크아웃하고, 디렉토리 경로를 인쇄합니다. 마지막 줄의 `echo`는 Claude Code가 worktree 경로로 읽는 것입니다. 다른 모든 출력을 stderr로 리디렉션하여 경로를 방해하지 않도록 합니다.

<h4 id="worktreecreate-input">
  WorktreeCreate 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 WorktreeCreate hook은 `name` 필드를 받습니다. 이는 새 worktree의 slug 식별자이며, 사용자가 지정하거나 자동 생성됩니다 (예: `bold-oak-a3f2`).

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeCreate",
  "name": "feature-auth"
}
```

<h4 id="worktreecreate-output">
  WorktreeCreate 출력
</h4>

WorktreeCreate hook은 표준 허용/차단 결정 모델을 사용하지 않습니다. 대신 hook의 성공 또는 실패가 결과를 결정합니다. hook은 생성된 worktree 디렉토리의 절대 경로를 반환해야 합니다:

* **명령 hook** (`type: "command"`): stdout의 마지막 비어 있지 않은 줄로 경로를 인쇄합니다. Claude Code는 경로를 읽기 전에 ANSI 이스케이프 코드를 제거하므로 셸 시작 배너가 `echo` 전에 인쇄되면 무시됩니다. 다른 모든 hook 출력을 stderr로 리디렉션합니다.
* **HTTP hook** (`type: "http"`): 응답 본문에서 `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }`를 반환합니다.

hook이 실패하거나 경로를 생성하지 않으면 worktree 생성이 오류로 실패합니다.

Claude Code는 hook이 실행된 디렉토리에 대해 상대 경로를 해결합니다. 결과 경로가 Claude Code가 들어갈 수 있는 디렉토리가 아니면 세션은 경로를 이름으로 지정하는 오류를 인쇄하고 코드 1로 종료됩니다. v2.1.205 이전에는 상대 경로 또는 디스크에 존재하지 않는 경로가 세션 시작 시 충돌했고, `-p`를 사용하면 약 30초 동안 정지한 후 코드 0으로 종료되었습니다.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

[WorktreeCreate](#worktreecreate)의 정리 대응. 이 hook은 worktree가 제거될 때 발생합니다. `--worktree` 세션을 종료하고 제거하도록 선택하거나 `isolation: "worktree"`를 가진 subagent가 완료될 때입니다. git 기반 worktree의 경우 Claude는 `git worktree remove`로 정리를 자동으로 처리합니다. git이 아닌 버전 제어 시스템에 대해 WorktreeCreate hook을 구성한 경우 정리를 처리하려면 WorktreeRemove hook과 쌍을 이루세요. 없으면 worktree 디렉토리가 디스크에 남아 있습니다.

Claude Code는 WorktreeCreate가 stdout에 인쇄한 경로를 hook 입력의 `worktree_path`로 전달합니다. 이 예제는 해당 경로를 읽고 디렉토리를 제거합니다:

```json theme={null}
{
  "hooks": {
    "WorktreeRemove": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'jq -r .worktree_path | xargs rm -rf'"
          }
        ]
      }
    ]
  }
}
```

<h4 id="worktreeremove-input">
  WorktreeRemove 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 WorktreeRemove hook은 제거되는 worktree의 절대 경로인 `worktree_path` 필드를 받습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeRemove",
  "worktree_path": "/Users/.../my-project/.claude/worktrees/feature-auth"
}
```

WorktreeRemove hook은 결정 제어가 없습니다. worktree 제거를 차단할 수 없지만 버전 제어 상태 제거 또는 변경 아카이빙과 같은 정리 작업을 수행할 수 있습니다. hook 실패는 디버그 모드에서만 기록됩니다.

<h3 id="precompact">
  PreCompact
</h3>

Claude Code가 압축 작업을 실행하려고 하기 전에 실행됩니다.

matcher 값은 압축이 수동으로 또는 자동으로 트리거되었는지 나타냅니다:

| Matcher  | 언제 발생하는지                |
| :------- | :---------------------- |
| `manual` | `/compact`              |
| `auto`   | 컨텍스트 윈도우가 가득 찼을 때 자동 압축 |

종료 코드 2로 압축을 차단합니다. 수동 `/compact`의 경우 stderr 메시지가 사용자에게 표시됩니다. JSON `"decision": "block"`을 사용하여 차단할 수도 있습니다.

자동 압축 차단은 발생 시기에 따라 다른 효과를 가집니다. 컨텍스트 제한 전에 압축이 사전에 트리거된 경우 Claude Code는 이를 건너뛰고 대화가 압축되지 않은 상태로 계속됩니다. 컨텍스트 제한 오류를 복구하기 위해 압축이 트리거된 경우 기본 오류가 표시되고 현재 요청이 실패합니다.

<h4 id="precompact-input">
  PreCompact 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 PreCompact hook은 `trigger` 및 `custom_instructions`를 받습니다. `manual`의 경우 `custom_instructions`는 사용자가 `/compact`에 전달하는 것을 포함합니다. `auto`의 경우 `custom_instructions`는 비어 있습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": ""
}
```

<h3 id="postcompact">
  PostCompact
</h3>

Claude Code가 압축 작업을 완료한 후 실행됩니다. 이 이벤트를 사용하여 새로운 압축된 상태에 반응합니다. 예를 들어 생성된 요약을 기록하거나 외부 상태를 업데이트합니다.

`PreCompact`와 동일한 matcher 값이 적용됩니다:

| Matcher  | 언제 발생하는지                  |
| :------- | :------------------------ |
| `manual` | `/compact` 후              |
| `auto`   | 컨텍스트 윈도우가 가득 찼을 때 자동 압축 후 |

<h4 id="postcompact-input">
  PostCompact 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 PostCompact hook은 `trigger` 및 `compact_summary`를 받습니다. `compact_summary` 필드는 압축 작업에서 생성된 대화 요약을 포함합니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PostCompact",
  "trigger": "manual",
  "compact_summary": "Summary of the compacted conversation..."
}
```

PostCompact hook은 결정 제어가 없습니다. 압축 결과에 영향을 미칠 수 없지만 후속 작업을 수행할 수 있습니다.

<h3 id="sessionend">
  SessionEnd
</h3>

Claude Code 세션이 종료될 때 실행됩니다. 정리 작업, 세션 통계 로깅 또는 세션 상태 저장에 유용합니다. 종료 이유별로 필터링할 matcher를 지원합니다.

hook 입력의 `reason` 필드는 세션이 종료된 이유를 나타냅니다:

| 이유                            | 설명                       |
| :---------------------------- | :----------------------- |
| `clear`                       | `/clear` 명령으로 세션 지워짐     |
| `resume`                      | 대화형 `/resume`을 통해 세션 전환됨 |
| `logout`                      | 사용자 로그아웃                 |
| `prompt_input_exit`           | 프롬프트 입력이 표시되는 동안 사용자 종료  |
| `bypass_permissions_disabled` | 권한 우회 모드 비활성화됨           |
| `other`                       | 기타 종료 이유                 |

<h4 id="sessionend-input">
  SessionEnd 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 SessionEnd hook은 세션이 종료된 이유를 나타내는 `reason` 필드를 받습니다. 모든 값은 위의 [이유 표](#sessionend)를 참조하세요.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

SessionEnd hook은 결정 제어가 없습니다. 세션 종료를 차단할 수 없지만 정리 작업을 수행할 수 있습니다.

SessionEnd hook의 기본 시간 초과는 1.5초입니다. 이는 세션 종료, `/clear`, 대화형 `/resume`을 통한 세션 전환 모두에 적용됩니다. hook에 더 많은 시간이 필요하면 hook 구성에서 `timeout`을 설정합니다. 전체 예산은 설정 파일의 가장 높은 hook별 `timeout`으로 자동으로 올라가며, 최대 60초입니다. plugin 제공 hook에 설정된 시간 초과는 예산을 올리지 않습니다. 예산을 명시적으로 재정의하려면 `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` 환경 변수를 밀리초 단위로 설정합니다.

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

<h3 id="elicitation">
  Elicitation
</h3>

MCP 서버가 작업 중 사용자 입력을 요청할 때 실행됩니다. 기본적으로 Claude Code는 사용자가 응답할 수 있는 대화형 대화 상자를 표시합니다. Hook은 이 요청을 가로채고 프로그래밍 방식으로 응답하여 대화 상자를 완전히 건너뛸 수 있습니다.

matcher 필드는 MCP 서버 이름과 일치합니다.

<h4 id="elicitation-input">
  Elicitation 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 Elicitation hook은 `mcp_server_name`, `message`, 선택적으로 `mode`, `url`, `elicitation_id`, `requested_schema` 필드를 받습니다.

form 모드 elicitation (가장 일반적인 경우):

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please provide your credentials",
  "mode": "form",
  "requested_schema": {
    "type": "object",
    "properties": {
      "username": { "type": "string", "title": "Username" }
    }
  }
}
```

URL 모드 elicitation (브라우저 기반 인증):

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please authenticate",
  "mode": "url",
  "url": "https://auth.example.com/login"
}
```

<h4 id="elicitation-output">
  Elicitation 출력
</h4>

대화 상자를 표시하지 않고 프로그래밍 방식으로 응답하려면 `hookSpecificOutput`이 있는 JSON 객체를 반환합니다:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Elicitation",
    "action": "accept",
    "content": {
      "username": "alice"
    }
  }
}
```

| 필드        | 값                             | 설명                                        |
| :-------- | :---------------------------- | :---------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | 요청을 수락, 거부 또는 취소할지 여부                     |
| `content` | 객체                            | 제출할 form 필드 값. `action`이 `accept`일 때만 사용됨 |

종료 코드 2는 elicitation을 거부하고 stderr을 사용자에게 표시합니다.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

사용자가 MCP elicitation에 응답한 후 실행됩니다. Hook은 응답을 관찰하고, 수정하거나, MCP 서버로 다시 전송되기 전에 차단할 수 있습니다.

matcher 필드는 MCP 서버 이름과 일치합니다.

<h4 id="elicitationresult-input">
  ElicitationResult 입력
</h4>

[공통 입력 필드](#common-input-fields) 외에도 ElicitationResult hook은 `mcp_server_name`, `action`, 선택적으로 `mode`, `elicitation_id`, `content` 필드를 받습니다.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "ElicitationResult",
  "mcp_server_name": "my-mcp-server",
  "action": "accept",
  "content": { "username": "alice" },
  "mode": "form",
  "elicitation_id": "elicit-123"
}
```

<h4 id="elicitationresult-output">
  ElicitationResult 출력
</h4>

사용자의 응답을 재정의하려면 `hookSpecificOutput`이 있는 JSON 객체를 반환합니다:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "ElicitationResult",
    "action": "decline",
    "content": {}
  }
}
```

| 필드        | 값                             | 설명                                              |
| :-------- | :---------------------------- | :---------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | 사용자의 작업을 재정의합니다                                 |
| `content` | 객체                            | form 필드 값을 재정의합니다. `action`이 `accept`일 때만 의미 있음 |

종료 코드 2는 응답을 차단하여 효과적인 작업을 `decline`으로 변경합니다.

<h2 id="prompt-based-hooks">
  프롬프트 기반 hook
</h2>

명령, HTTP 및 MCP tool hook 외에도 Claude Code는 LLM을 사용하여 작업을 허용할지 차단할지 평가하는 프롬프트 기반 hook (`type: "prompt"`)과 도구 액세스가 있는 에이전트 검증자를 생성하는 에이전트 hook (`type: "agent"`)을 지원합니다. 모든 이벤트가 모든 hook 유형을 지원하는 것은 아닙니다.

다섯 가지 hook 유형 모두 (`command`, `http`, `mcp_tool`, `prompt`, `agent`)를 지원하는 이벤트:

* `PermissionDenied`
* `PermissionRequest`
* `PostToolBatch`
* `PostToolUse`
* `PostToolUseFailure`
* `PreToolUse`
* `Stop`
* `SubagentStop`
* `TaskCompleted`
* `TaskCreated`
* `TeammateIdle`
* `UserPromptExpansion`
* `UserPromptSubmit`

`command`, `http` 및 `mcp_tool` hook을 지원하지만 `prompt` 또는 `agent`는 지원하지 않는 이벤트:

* `ConfigChange`
* `CwdChanged`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `Notification`
* `PostCompact`
* `PreCompact`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` 및 `Setup`은 `command` 및 `mcp_tool` hook을 지원합니다. `http`, `prompt` 또는 `agent` hook은 지원하지 않습니다.

<h3 id="how-prompt-based-hooks-work">
  프롬프트 기반 hook이 어떻게 작동하는지
</h3>

프롬프트 기반 hook은 Bash 명령을 실행하는 대신:

1. hook 입력과 프롬프트를 Claude 모델 (기본값 Haiku)로 전송합니다
2. LLM은 결정을 포함하는 구조화된 JSON으로 응답합니다
3. Claude Code는 결정을 자동으로 처리합니다

<h3 id="prompt-hook-configuration">
  프롬프트 hook 구성
</h3>

`type`을 `"prompt"`로 설정하고 `command` 대신 `prompt` 문자열을 제공합니다. `$ARGUMENTS` 자리 표시자를 사용하여 hook의 JSON 입력 데이터를 프롬프트 텍스트에 주입합니다. Claude Code는 결합된 프롬프트와 입력을 빠른 Claude 모델로 전송하며, 이는 JSON 결정을 반환합니다.

이 `Stop` hook은 Claude가 완료되기 전에 모든 작업이 완료되었는지 평가하도록 LLM에 요청합니다:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Evaluate if Claude should stop: $ARGUMENTS. Check if all tasks are complete."
          }
        ]
      }
    ]
  }
}
```

| 필드                | 필수  | 설명                                                                                                                                                                         |
| :---------------- | :-- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | 예   | `"prompt"`여야 합니다                                                                                                                                                           |
| `prompt`          | 예   | LLM으로 전송할 프롬프트 텍스트. hook 입력 JSON에 대한 자리 표시자로 `$ARGUMENTS` 사용. `$ARGUMENTS`가 없으면 입력 JSON이 프롬프트에 추가됩니다                                                                       |
| `model`           | 아니오 | 평가에 사용할 모델. 기본값은 빠른 모델                                                                                                                                                     |
| `timeout`         | 아니오 | 초 단위 시간 초과. 기본값: 30                                                                                                                                                        |
| `continueOnBlock` | 아니오 | 프롬프트가 `ok: false`를 반환할 때 이유를 Claude에 다시 피드백하고 중지하는 대신 턴을 계속합니다. 기본값: `false`. 결과 `decision: "block"`에서 `continue: true`로 구현됩니다. 이벤트별 동작은 [응답 스키마](#response-schema)를 참조하세요 |

<h3 id="response-schema">
  응답 스키마
</h3>

LLM은 다음을 포함하는 JSON으로 응답해야 합니다:

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision"
}
```

| 필드       | 설명                                                                       |
| :------- | :----------------------------------------------------------------------- |
| `ok`     | `true`는 작업을 허용하고 `false`는 `decision: "block"`을 생성합니다. 아래의 이벤트별 동작을 참조하세요 |
| `reason` | `ok`가 `false`일 때 필수입니다. 차단 이유로 사용됩니다                                     |

`ok: false`에서 발생하는 상황은 이벤트에 따라 다릅니다:

* `Stop` 및 `SubagentStop`: 이유는 Claude의 다음 명령으로 피드백되며 턴이 계속됩니다
* `PreToolUse`: tool 호출이 거부되고 이유는 Claude에 tool 오류로 반환되며, 이는 명령 hook의 `permissionDecision: "deny"`와 동일합니다
* `PostToolUse`: 기본적으로 턴이 끝나고 이유는 채팅에 경고 줄로 나타납니다. `continueOnBlock: true`를 설정하여 이유를 Claude에 다시 피드백하고 턴을 계속하는 대신 사용합니다
* `PostToolBatch`, `UserPromptSubmit` 및 `UserPromptExpansion`: 턴이 끝나고 이유는 경고 줄로 나타납니다. 이러한 이벤트는 `continue`에 관계없이 `decision: "block"`에서 턴을 종료합니다
* `PostToolUseFailure`, `TaskCreated` 및 `TaskCompleted`: 이유는 Claude에 tool 오류로 반환되며, `PreToolUse`와 유사합니다
* `TeammateIdle`: 기본적으로 팀원이 중지되고 이유는 경고 줄로 나타납니다. `continueOnBlock: true`를 설정하여 이유를 팀원에게 다시 피드백하고 계속 작업하도록 유지합니다
* `PermissionRequest`: `ok: false`는 효과가 없습니다. hook에서 승인을 거부하려면 `hookSpecificOutput.decision.behavior: "deny"`를 반환하는 [명령 hook](#command-hook-fields)을 사용합니다
* `PermissionDenied`: `ok: false`는 거부가 이미 발생했기 때문에 효과가 없습니다. 이 이벤트가 읽는 유일한 출력은 `hookSpecificOutput.retry`이며, 프롬프트 및 에이전트 hook은 이를 설정할 수 없습니다. 이들은 이 이벤트에서 실행되지만 출력은 버려집니다. `retry`를 반환하려면 [명령 hook](#command-hook-fields)을 사용합니다

이벤트에 대해 더 세밀한 제어가 필요한 경우 [결정 제어](#decision-control)에 설명된 이벤트별 필드가 있는 [명령 hook](#command-hook-fields)을 사용합니다.

<h3 id="check-multiple-conditions-before-stopping">
  중지하기 전에 여러 조건 확인
</h3>

이 `Stop` hook은 Claude가 중지하기 전에 세 가지 조건을 확인하는 자세한 프롬프트를 사용합니다. `SubagentStop` hook은 [subagent](/docs/ko/sub-agents)가 중지해야 하는지 평가하는 동일한 형식을 사용합니다. `"ok"`가 `false`이면 Claude는 제공된 이유를 다음 명령으로 받으며 계속 작업합니다:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "You are evaluating whether Claude should stop working. Context: $ARGUMENTS\n\nAnalyze the conversation and determine if:\n1. All user-requested tasks are complete\n2. Any errors need to be addressed\n3. Follow-up work is needed\n\nRespond with JSON: {\"ok\": true} to allow stopping, or {\"ok\": false, \"reason\": \"your explanation\"} to continue working.",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

<h2 id="agent-based-hooks">
  에이전트 기반 hook
</h2>

<Warning>
  에이전트 hook은 실험적입니다. 동작 및 구성은 향후 릴리스에서 변경될 수 있습니다. 프로덕션 워크플로우의 경우 [명령 hook](#command-hook-fields)을 선호합니다.
</Warning>

에이전트 기반 hook (`type: "agent"`)은 프롬프트 기반 hook과 유사하지만 다중 턴 도구 액세스가 있습니다. 단일 LLM 호출 대신 에이전트 hook은 파일을 읽고, 코드를 검색하고, 코드베이스를 검사하여 조건을 확인할 수 있는 subagent를 생성합니다. 에이전트 hook은 프롬프트 기반 hook과 동일한 이벤트를 지원합니다.

<h3 id="how-agent-hooks-work">
  에이전트 hook이 어떻게 작동하는지
</h3>

에이전트 hook이 발생할 때:

1. Claude Code는 프롬프트와 hook의 JSON 입력을 가진 subagent를 생성합니다
2. subagent는 Read, Grep, Glob과 같은 도구를 사용하여 조사할 수 있습니다
3. 최대 50턴 후 subagent는 구조화된 `{ "ok": true/false }` 결정을 반환합니다
4. Claude Code는 프롬프트 hook과 동일한 방식으로 결정을 처리합니다

에이전트 hook은 검증이 hook 입력 데이터만으로 평가하는 것이 아니라 실제 파일이나 테스트 출력을 검사해야 할 때 유용합니다.

<h3 id="agent-hook-configuration">
  에이전트 hook 구성
</h3>

`type`을 `"agent"`로 설정하고 `prompt` 문자열을 제공합니다. 구성 필드는 [프롬프트 hook](#prompt-hook-configuration)과 동일하지만 더 긴 기본 시간 초과가 있습니다:

| 필드        | 필수  | 설명                                                          |
| :-------- | :-- | :---------------------------------------------------------- |
| `type`    | 예   | `"agent"`여야 합니다                                             |
| `prompt`  | 예   | 확인할 내용을 설명하는 프롬프트. hook 입력 JSON에 대한 자리 표시자로 `$ARGUMENTS` 사용 |
| `model`   | 아니오 | 사용할 모델. 기본값은 빠른 모델                                          |
| `timeout` | 아니오 | 초 단위 시간 초과. 기본값: 60                                         |

응답 스키마는 프롬프트 hook과 동일합니다: 허용하려면 `{ "ok": true }` 또는 차단하려면 `{ "ok": false, "reason": "..." }`.

이 `Stop` hook은 Claude가 완료되기 전에 모든 단위 테스트가 통과하는지 확인합니다:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

<h2 id="run-hooks-in-the-background">
  백그라운드에서 hook 실행
</h2>

기본적으로 hook은 완료될 때까지 Claude의 실행을 차단합니다. 배포, 테스트 스위트 또는 외부 API 호출과 같은 장기 실행 작업의 경우 `"async": true`를 설정하여 Claude가 계속 작업하는 동안 백그라운드에서 hook을 실행합니다. 비동기 hook은 차단하거나 Claude의 동작을 제어할 수 없습니다: `decision`, `permissionDecision`, `continue`와 같은 응답 필드는 효과가 없습니다. 제어했을 작업이 이미 완료되었기 때문입니다.

<h3 id="configure-an-async-hook">
  비동기 hook 구성
</h3>

hook 구성에 `"async": true`를 추가하여 Claude를 차단하지 않고 백그라운드에서 실행합니다. 이 필드는 `type: "command"` hook에서만 사용 가능합니다.

이 hook은 모든 `Write` 도구 호출 후 테스트 스크립트를 실행합니다. Claude는 `run-tests.sh`가 최대 120초 동안 실행되는 동안 즉시 계속 작업합니다. 스크립트가 완료되면 출력이 다음 대화 턴에 전달됩니다:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/run-tests.sh",
            "async": true,
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

`timeout` 필드는 백그라운드 프로세스의 최대 시간을 초 단위로 설정합니다. 지정하지 않으면 비동기 hook은 동기 hook과 동일한 10분 기본값을 사용합니다.

<h3 id="how-async-hooks-execute">
  비동기 hook이 어떻게 실행되는지
</h3>

비동기 hook이 발생하면 Claude Code는 hook 프로세스를 시작하고 완료를 기다리지 않고 즉시 계속합니다. hook은 동기 hook과 동일한 JSON 입력을 stdin을 통해 받습니다.

백그라운드 프로세스가 종료된 후 hook이 `additionalContext` 필드가 있는 JSON 응답을 생성한 경우 해당 콘텐츠는 다음 대화 턴에서 Claude에 컨텍스트로 전달됩니다. `systemMessage` 필드는 Claude가 아닌 사용자에게 표시됩니다.

Claude Code는 JSON 응답을 동기 hook과 동일한 [출력 스키마](#json-output)에 대해 검증하고, `systemMessage`가 문자열이 아닌 경우와 같이 값의 유형이 잘못된 필드를 전달하지 않고 삭제합니다. `--debug`로 실행하여 삭제된 각 필드의 이름을 지정하는 경고를 확인합니다. v2.1.202 이전에는 비동기 hook의 잘못된 형식의 JSON 출력이 세션을 충돌시킬 수 있었고, 세션이 재개될 때마다 충돌이 반복되었습니다.

비동기 hook 완료 알림은 기본적으로 억제됩니다. 보려면 `Ctrl+O`로 자세한 모드를 활성화하거나 `--verbose`로 Claude Code를 시작합니다.

<h3 id="run-tests-after-file-changes">
  파일 변경 후 테스트 실행
</h3>

이 hook은 Claude가 파일을 쓸 때마다 백그라운드에서 테스트 스위트를 시작한 후 테스트가 완료되면 결과를 Claude에 보고합니다. 이 스크립트를 프로젝트의 `.claude/hooks/run-tests-async.sh`에 저장하고 `chmod +x`로 실행 가능하게 만듭니다:

```bash theme={null}
#!/bin/bash
# run-tests-async.sh

# stdin에서 hook 입력을 읽습니다
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# 소스 파일에 대해서만 테스트를 실행합니다
if [[ "$FILE_PATH" != *.ts && "$FILE_PATH" != *.js ]]; then
  exit 0
fi

# 테스트를 실행하고 additionalContext를 통해 결과를 Claude에 보고합니다
RESULT=$(npm test 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  MSG="Tests passed after editing $FILE_PATH"
else
  MSG="Tests failed after editing $FILE_PATH: $RESULT"
fi
jq -nc --arg msg "$MSG" '{hookSpecificOutput: {hookEventName: "PostToolUse", additionalContext: $msg}}'
```

그런 다음 프로젝트 루트의 `.claude/settings.json`에 이 구성을 추가합니다. `async: true` 플래그를 사용하면 Claude가 테스트 실행 중에 계속 작업할 수 있습니다:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/run-tests-async.sh",
            "args": [],
            "async": true,
            "timeout": 300
          }
        ]
      }
    ]
  }
}
```

<h3 id="limitations">
  제한 사항
</h3>

비동기 hook은 동기 hook과 비교하여 여러 제약이 있습니다:

* `type: "command"` hook만 `async`를 지원합니다. 프롬프트 기반 hook은 비동기적으로 실행될 수 없습니다.
* 비동기 hook은 도구 호출을 차단하거나 결정을 반환할 수 없습니다. hook이 완료될 때까지 트리거 작업이 이미 진행되었습니다.
* Hook 출력은 다음 대화 턴에 전달됩니다. 세션이 유휴 상태이면 응답은 다음 사용자 상호 작용까지 기다립니다. 예외: `asyncRewake` hook이 종료 코드 2로 종료되면 세션이 유휴 상태일 때도 Claude를 즉시 깨웁니다.
* 각 실행은 별도의 백그라운드 프로세스를 생성합니다. 동일한 비동기 hook의 여러 발생에 걸쳐 중복 제거가 없습니다.

<h2 id="security-considerations">
  보안 고려 사항
</h2>

<h3 id="disclaimer">
  면책 조항
</h3>

명령 hook은 시스템 사용자의 전체 권한으로 실행됩니다.

<Warning>
  명령 hook은 전체 사용자 권한으로 셸 명령을 실행합니다. 사용자 계정이 액세스할 수 있는 모든 파일을 수정, 삭제 또는 액세스할 수 있습니다. 구성에 추가하기 전에 모든 hook 명령을 검토하고 테스트하세요.
</Warning>

<h3 id="security-best-practices">
  보안 모범 사례
</h3>

hook을 작성할 때 이러한 사례를 염두에 두세요:

* **입력 검증 및 살균**: 입력 데이터를 맹목적으로 신뢰하지 마세요
* **항상 셸 변수를 따옴표로 감싸세요**: `$VAR` 대신 `"$VAR"` 사용
* **경로 순회 차단**: 파일 경로에서 `..` 확인
* **절대 경로 사용**: 스크립트의 전체 경로를 지정하세요. exec 형식에서는 `${CLAUDE_PROJECT_DIR}`을 사용하고 경로는 따옴표가 필요하지 않습니다. shell 형식에서는 큰따옴표로 감싸세요
* **민감한 파일 건너뛰기**: `.env`, `.git/`, 키 등을 피하세요

<h2 id="windows-powershell-tool">
  Windows PowerShell 도구
</h2>

Windows에서 명령 hook에 `"shell": "powershell"`을 설정하여 PowerShell에서 개별 hook을 실행할 수 있습니다. Hook은 PowerShell을 직접 생성하므로 `CLAUDE_CODE_USE_POWERSHELL_TOOL`이 설정되어 있는지 여부와 관계없이 작동합니다. Claude Code는 PowerShell 7 이상의 실행 파일인 `pwsh.exe`를 자동 감지하고 Windows PowerShell 5.1의 `powershell.exe`로 폴백합니다.

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "shell": "powershell",
            "command": "Write-Host 'File written'"
          }
        ]
      }
    ]
  }
}
```

PowerShell 셸 형식 명령에서 프로젝트 루트를 참조하려면 `${CLAUDE_PROJECT_DIR}` 또는 `$env:CLAUDE_PROJECT_DIR`을 작성합니다. v2.1.198부터 Claude Code는 hook이 `settings.json`, 플러그인 또는 스킬에 정의되어 있는지 여부와 관계없이 PowerShell 셸 형식 명령에서 `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_PLUGIN_ROOT}` 및 `${CLAUDE_PLUGIN_DATA}` 자리 표시자를 PowerShell의 `${env:NAME}` 형식으로 다시 작성합니다. PowerShell은 구문 분석 후 내보낸 환경에서 값을 확인하므로 자리 표시자는 큰따옴표로 묶인 문자열 내에서는 작동하지만 PowerShell이 변수를 확장하지 않는 작은따옴표로 묶인 문자열 내에서는 작동하지 않습니다.

v2.1.198 이전에는 이 다시 쓰기가 플러그인 hook에만 적용되었습니다. 이전 버전에서는 `settings.json` hook이 `$env:` 형식이나 [exec 형식](#exec-form-and-shell-form)이 필요하며, 여기서 `${CLAUDE_PROJECT_DIR}`은 hook이 정의된 위치와 관계없이 각 `args` 요소에서 대체됩니다.

PowerShell hook에서 `$CLAUDE_PROJECT_DIR`의 단순한 형식을 작성하지 마십시오. PowerShell은 이를 정의되지 않은 로컬 변수로 구문 분석하고 `$null`로 확인하므로 스크립트 경로가 프로젝트 루트 접두사 없이 남습니다. Claude Code는 해당 형식을 다시 작성하지 않으며 대신 [디버그 로그](#debug-hooks)에 경고를 기록합니다.

아래 예제는 모든 버전에서 작동하는 `$env:` 형식으로 프로젝트 스크립트를 실행하는 `settings.json` hook을 보여줍니다:

```json theme={null}
{
  "type": "command",
  "shell": "powershell",
  "command": "& \"$env:CLAUDE_PROJECT_DIR\\.claude\\hooks\\check.ps1\""
}
```

<h2 id="debug-hooks">
  Hook 디버그
</h2>

Hook 실행 세부 정보, 일치한 hook, 종료 코드, 전체 stdout 및 stderr은 디버그 로그 파일에 기록됩니다. `claude --debug-file <path>`로 Claude Code를 시작하여 로그를 알려진 위치에 작성하거나 `claude --debug`를 실행하고 `~/.claude/debug/<session-id>.txt`에서 로그를 읽습니다. `--debug` 플래그는 터미널에 인쇄하지 않습니다.

```text theme={null}
[DEBUG] Executing hooks for PostToolUse:Write
[DEBUG] Found 1 hook commands to execute
[DEBUG] Executing hook command: <Your command> with timeout 600000ms
[DEBUG] Hook command completed with status 0: <Your stdout>
```

더 세밀한 hook 일치 세부 정보를 보려면 `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose`를 설정하여 hook matcher 수 및 쿼리 일치와 같은 추가 로그 줄을 확인합니다.

hook이 발생하지 않음, 무한 Stop hook 루프 또는 구성 오류와 같은 일반적인 문제 해결은 가이드의 [제한 사항 및 문제 해결](/docs/ko/hooks-guide#limitations-and-troubleshooting)을 참조하세요. 더 광범위한 진단 안내는 `/context`, `/doctor` 및 설정 우선순위를 다루는 [구성 디버그](/docs/ko/debug-your-config)를 참조하세요.
