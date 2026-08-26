> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 참조

> Claude Code 플러그인 시스템의 완전한 기술 참조, 스키마, CLI 명령어 및 컴포넌트 사양 포함.

<Tip>
  플러그인을 설치하려고 하시나요? [플러그인 발견 및 설치](/docs/ko/discover-plugins)를 참조하세요. 플러그인 생성에 대해서는 [플러그인](/docs/ko/plugins)을 참조하세요. 플러그인 배포에 대해서는 [플러그인 마켓플레이스](/docs/ko/plugin-marketplaces)를 참조하세요.
</Tip>

이 참조는 Claude Code 플러그인 시스템의 완전한 기술 사양을 제공하며, 컴포넌트 스키마, CLI 명령어 및 개발 도구를 포함합니다.

**플러그인**은 Claude Code를 사용자 정의 기능으로 확장하는 자체 포함된 컴포넌트 디렉토리입니다. 플러그인 컴포넌트에는 skills, agents, hooks, MCP servers, LSP servers 및 monitors가 포함됩니다.

<h2 id="plugin-components-reference">
  플러그인 컴포넌트 참조
</h2>

<h3 id="skills">
  Skills
</h3>

플러그인은 Claude Code에 skills를 추가하여 사용자나 Claude가 호출할 수 있는 `/name` 바로가기를 생성합니다.

**위치**: 플러그인 루트의 `skills/` 또는 `commands/` 디렉토리, 또는 플러그인 루트의 단일 `SKILL.md` 파일

**파일 형식**: Skills는 `SKILL.md`가 있는 디렉토리이고, commands는 간단한 마크다운 파일입니다.

**Skill 구조**:

```text theme={null}
skills/
├── pdf-processor/
│   ├── SKILL.md
│   ├── reference.md (선택사항)
│   └── scripts/ (선택사항)
└── code-reviewer/
    └── SKILL.md
```

**통합 동작**:

* Skills와 commands는 플러그인이 설치될 때 자동으로 발견됩니다.
* Claude는 작업 컨텍스트에 따라 자동으로 이들을 호출할 수 있습니다.
* Skills는 SKILL.md와 함께 지원 파일을 포함할 수 있습니다.

플러그인에 `skills/` 디렉토리가 없고 `skills` manifest 필드가 없으면, 플러그인 루트의 `SKILL.md`가 단일 skill로 로드됩니다. frontmatter `name` 필드를 설정하여 skill의 호출 이름을 제어하세요. 이 필드가 없으면 Claude Code는 설치 디렉토리 이름으로 폴백되며, 마켓플레이스에서 설치된 플러그인의 경우 매 업데이트마다 변경되는 버전 문자열입니다. 둘 이상의 skill을 제공하는 플러그인의 경우 위에 표시된 `skills/` 디렉토리 레이아웃을 사용하세요.

완전한 세부 정보는 [Skills](/docs/ko/skills)를 참조하세요.

<h3 id="agents">
  Agents
</h3>

플러그인은 Claude가 적절할 때 자동으로 호출할 수 있는 특정 작업을 위한 특화된 subagents를 제공할 수 있습니다.

**위치**: 플러그인 루트의 `agents/` 디렉토리

**파일 형식**: 에이전트 기능을 설명하는 마크다운 파일

**Agent 구조**:

```markdown theme={null}
---
name: agent-name
description: 이 에이전트가 전문으로 하는 분야와 Claude가 이를 호출해야 할 때
model: sonnet
effort: medium
maxTurns: 20
disallowedTools: Write, Edit
---

에이전트의 역할, 전문성 및 동작을 설명하는 상세한 시스템 프롬프트입니다.
```

플러그인 agents는 `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background` 및 `isolation` frontmatter 필드를 지원합니다. 유일한 유효한 `isolation` 값은 `"worktree"`입니다. 보안상의 이유로 `hooks`, `mcpServers` 및 `permissionMode`는 플러그인 제공 agents에서 지원되지 않습니다.

**통합 지점**:

* Agents는 [@-mention 타입어헤드](/docs/ko/sub-agents#invoke-subagents-explicitly)에 `my-plugin:code-reviewer`와 같은 범위가 지정된 이름으로 나타나며, 플러그인이 활성화되면 표시됩니다.
* Claude는 작업 컨텍스트에 따라 agents를 자동으로 호출할 수 있습니다.
* Agents는 사용자가 수동으로 호출할 수 있습니다.
* 플러그인 agents는 기본 제공 Claude agents와 함께 작동합니다.

완전한 세부 정보는 [Subagents](/docs/ko/sub-agents)를 참조하세요.

<h3 id="hooks">
  Hooks
</h3>

플러그인은 Claude Code 이벤트에 자동으로 응답하는 이벤트 핸들러를 제공할 수 있습니다.

**위치**: 플러그인 루트의 `hooks/hooks.json` 또는 plugin.json에 인라인

**형식**: 이벤트 매처 및 작업이 있는 JSON 구성

**Hook 구성**:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format-code.sh"
          }
        ]
      }
    ]
  }
}
```

플러그인 hooks는 [사용자 정의 hooks](/docs/ko/hooks)와 동일한 라이프사이클 이벤트에 응답합니다:

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
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

**Hook 유형**:

* `command`: 셸 명령어 또는 스크립트 실행
* `http`: 이벤트 JSON을 URL로 POST 요청으로 전송
* `mcp_tool`: 구성된 [MCP server](/docs/ko/mcp)에서 도구 호출
* `prompt`: LLM으로 프롬프트 평가 (컨텍스트에 대해 `$ARGUMENTS` 플레이스홀더 사용)
* `agent`: 복잡한 검증 작업을 위해 도구가 있는 에이전트 검증자 실행

플러그인의 자체 [번들 MCP server](#mcp-servers)를 대상으로 하는 Hooks는 범위가 지정된 이름을 사용해야 합니다. 도구 매처 및 `if` 필드는 범위가 지정된 도구 이름 `mcp__plugin_<plugin-name>_<server-name>__<tool>`을 사용하고, `mcp_tool` hook의 `server` 필드는 `plugin:<plugin-name>:<server-name>`을 사용합니다. 베어 서버 키에 대해 작성된 매처는 절대 실행되지 않습니다. [MCP 도구 매칭](/docs/ko/hooks#match-mcp-tools) 및 [플러그인 제공 MCP servers](/docs/ko/mcp#plugin-provided-mcp-servers)를 참조하세요.

<h3 id="mcp-servers">
  MCP servers
</h3>

플러그인은 Claude Code를 외부 도구 및 서비스와 연결하기 위해 Model Context Protocol (MCP) servers를 번들로 제공할 수 있습니다.

**위치**: 플러그인 루트의 `.mcp.json` 또는 plugin.json에 인라인

**형식**: 표준 MCP 서버 구성

**MCP 서버 구성**:

```json theme={null}
{
  "mcpServers": {
    "plugin-database": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_PATH": "${CLAUDE_PLUGIN_ROOT}/data"
      }
    },
    "plugin-api-client": {
      "command": "npx",
      "args": ["@company/mcp-server", "--plugin-mode"]
    }
  }
}
```

**통합 동작**:

* 플러그인 MCP servers는 플러그인이 활성화될 때 자동으로 시작됩니다.
* Servers는 Claude의 도구 키트에서 표준 MCP 도구로 나타납니다.
* 서버 기능은 Claude의 기존 도구와 원활하게 통합됩니다.
* 플러그인 servers는 사용자 MCP servers와 독립적으로 구성할 수 있습니다.

<h3 id="lsp-servers">
  LSP servers
</h3>

<Tip>
  LSP 플러그인을 사용하려고 하시나요? 공식 마켓플레이스에서 설치하세요: `/plugin` Discover 탭에서 "lsp"를 검색하세요. 이 섹션은 공식 마켓플레이스에서 다루지 않는 언어에 대해 LSP 플러그인을 만드는 방법을 문서화합니다.
</Tip>

플러그인은 [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) servers를 제공하여 코드베이스에서 작업할 때 Claude에게 실시간 코드 인텔리전스를 제공할 수 있습니다.

LSP 통합은 다음을 제공합니다:

* **즉시 진단**: Claude는 각 편집 후 즉시 오류 및 경고를 봅니다.
* **코드 네비게이션**: 정의로 이동, 참조 찾기 및 호버 정보
* **언어 인식**: 코드 기호에 대한 타입 정보 및 문서

**위치**: 플러그인 루트의 `.lsp.json` 또는 `plugin.json`에 인라인

**형식**: 언어 서버 이름을 해당 구성에 매핑하는 JSON 구성

**`.lsp.json` 파일 형식**:

```json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

**`plugin.json`에 인라인**:

```json theme={null}
{
  "name": "my-plugin",
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": {
        ".go": "go"
      }
    }
  }
}
```

**필수 필드:**

| 필드                    | 설명                         |
| :-------------------- | :------------------------- |
| `command`             | 실행할 LSP 바이너리 (PATH에 있어야 함) |
| `extensionToLanguage` | 파일 확장자를 언어 식별자에 매핑         |

**선택사항 필드:**

| 필드                      | 설명                                                                                          |
| :---------------------- | :------------------------------------------------------------------------------------------ |
| `args`                  | LSP 서버의 명령줄 인수                                                                              |
| `transport`             | 통신 전송: `stdio` (기본값) 또는 `socket`                                                            |
| `env`                   | 서버 시작 시 설정할 환경 변수                                                                           |
| `initializationOptions` | 초기화 중에 서버에 전달되는 옵션                                                                          |
| `settings`              | `workspace/didChangeConfiguration`을 통해 전달되는 설정                                              |
| `workspaceFolder`       | 서버의 작업 공간 폴더 경로                                                                             |
| `startupTimeout`        | 서버 시작을 기다릴 최대 시간 (밀리초)                                                                      |
| `shutdownTimeout`       | 정상 종료를 기다릴 최대 시간 (밀리초). 시간 초과가 경과하면 Claude Code가 서버 프로세스를 종료합니다. 설정하지 않으면 시간 초과가 적용되지 않습니다. |
| `restartOnCrash`        | 서버가 충돌한 후 다시 시작할지 여부. 기본값은 `true`입니다. 충돌한 서버를 다시 시작하지 않고 중지된 상태로 두려면 `false`로 설정하세요.        |
| `maxRestarts`           | 포기하기 전 최대 재시작 시도 횟수                                                                         |
| `diagnostics`           | Claude의 컨텍스트에 진단을 푸시할지 여부 (기본값 `true`). 코드 네비게이션은 유지하되 자동 진단 주입을 억제하려면 `false`로 설정하세요.      |

`restartOnCrash` 및 `shutdownTimeout`은 Claude Code v2.1.205 이상이 필요합니다. v2.1.205 이전에는 구성 스키마가 두 옵션을 모두 허용했지만 둘 중 하나를 설정하면 Claude Code가 시작 시 해당 LSP 서버를 완전히 건너뛰었으며, 그 이유는 `claude --debug` 출력에서만 볼 수 있었습니다.

**동일한 확장자에 대한 여러 서버**: 하나 이상의 활성화된 LSP 서버가 `extensionToLanguage`에서 동일한 파일 확장자를 선언할 때, 서버가 하나의 플러그인에서 오든 다른 플러그인에서 오든, 첫 번째로 등록된 서버가 해당 확장자의 파일을 처리하고 다른 서버는 절대 시작되지 않습니다. `/plugin` 인터페이스는 활성 서버인 플러그인의 이름을 지정하는 경고를 표시합니다.

**초기화에 실패한 서버**: Claude Code는 구성이 유효하지 않은 서버 (예: `command` 또는 `extensionToLanguage`가 누락된 서버)를 건너뛰고, 다른 구성된 서버는 여전히 시작됩니다. `claude --debug`를 실행하여 서버가 건너뛴 이유를 확인하세요.

건너뛴 서버는 파일 확장자를 요청하지 않으므로, 동일한 확장자를 선언하는 다른 유효한 서버 (동일한 플러그인 또는 다른 플러그인에서)가 여전히 해당 파일을 처리합니다. v2.1.205 이전에는 초기화에 실패한 서버가 여전히 확장자를 요청했고 동일한 확장자에 대한 다른 유효한 서버를 차단했습니다.

<Warning>
  **언어 서버 바이너리를 별도로 설치해야 합니다.** LSP 플러그인은 Claude Code가 언어 서버에 연결하는 방법을 구성하지만, 서버 자체는 포함하지 않습니다. `/plugin` Errors 탭에서 `Executable not found in $PATH`를 보면 언어에 필요한 바이너리를 설치하세요.
</Warning>

**사용 가능한 LSP 플러그인:**

| 플러그인                | 언어 서버                      | 설치 명령어                                                                          |
| :------------------ | :------------------------- | :------------------------------------------------------------------------------ |
| `pyright-lsp`       | Pyright (Python)           | `pip install pyright` 또는 `npm install -g pyright`                               |
| `typescript-lsp`    | TypeScript Language Server | `npm install -g typescript-language-server typescript`                          |
| `rust-analyzer-lsp` | rust-analyzer              | [rust-analyzer 설치 참조](https://rust-analyzer.github.io/manual.html#installation) |

먼저 언어 서버를 설치한 다음 마켓플레이스에서 플러그인을 설치하세요.

<h3 id="monitors">
  Monitors
</h3>

플러그인은 플러그인이 활성화될 때 Claude Code가 자동으로 시작하는 백그라운드 monitors를 선언할 수 있습니다. 각 monitor는 세션 동안 셸 명령어를 실행하고 모든 stdout 라인을 Claude에게 알림으로 전달하므로 Claude는 로그 항목, 상태 변경 또는 폴링된 이벤트에 반응할 수 있으며 자신이 watch를 시작하도록 요청받을 필요가 없습니다.

플러그인 monitors는 [Monitor tool](/docs/ko/tools-reference#monitor-tool)과 동일한 메커니즘을 사용하며 해당 가용성 제약을 공유합니다. 이들은 대화형 CLI 세션에서만 실행되고, [hooks](#hooks)와 동일한 신뢰 수준에서 샌드박스 없이 실행되며, Monitor tool을 사용할 수 없는 호스트에서는 건너뜁니다.

**위치**: 플러그인 루트의 `monitors/monitors.json` 또는 plugin.json에 인라인

**형식**: monitor 항목의 JSON 배열

다음 `monitors/monitors.json`은 배포 상태 엔드포인트와 로컬 오류 로그를 감시합니다:

```json theme={null}
[
  {
    "name": "deploy-status",
    "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
    "description": "배포 상태 변경"
  },
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "애플리케이션 오류 로그",
    "when": "on-skill-invoke:debug"
  }
]
```

monitors를 인라인으로 선언하려면 `plugin.json`의 `experimental.monitors`를 동일한 배열로 설정하세요. 기본이 아닌 경로에서 로드하려면 `experimental.monitors`를 `"./config/monitors.json"`과 같은 상대 경로 문자열로 설정하세요. Monitors는 [실험적 컴포넌트](#experimental-components)입니다.

**필수 필드:**

| 필드            | 설명                                                               |
| :------------ | :--------------------------------------------------------------- |
| `name`        | 플러그인 내에서 고유한 식별자. 플러그인이 다시 로드되거나 skill이 다시 호출될 때 중복 프로세스를 방지합니다. |
| `command`     | 세션 작업 디렉토리에서 영구 백그라운드 프로세스로 실행되는 셸 명령어                           |
| `description` | 감시 중인 항목에 대한 간단한 요약. 작업 패널 및 알림 요약에 표시됩니다.                       |

**선택사항 필드:**

| 필드     | 설명                                                                                                                                          |
| :----- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `when` | monitor가 시작되는 시기를 제어합니다. `"always"`는 세션 시작 및 플러그인 다시 로드 시 시작하며 기본값입니다. `"on-skill-invoke:<skill-name>"`은 이 플러그인의 명명된 skill이 처음 발송될 때 시작합니다. |

`command` 값은 [경로 대체](#environment-variables) `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PLUGIN_DATA}`, `${CLAUDE_PROJECT_DIR}` 및 환경의 모든 `${ENV_VAR}`을 지원합니다. 스크립트가 플러그인 자체 디렉토리에서 실행되어야 하는 경우 명령어 앞에 `cd "${CLAUDE_PLUGIN_ROOT}" && `를 붙이세요.

monitor `command`는 [`${user_config.*}`](#user-configuration) 값을 참조할 수 없습니다. 명령어는 셸을 통해 실행되므로 Claude Code는 값을 대체하는 대신 [오류](/docs/ko/errors#plugin-command-references-user-config)로 monitor를 거부합니다. Monitor 프로세스는 `CLAUDE_PLUGIN_OPTION_<KEY>` 환경 변수를 받지 않으므로 monitor 스크립트가 자신이 소유한 구성 파일에서 값을 읽도록 하세요. v2.1.207 이전에는 monitor 명령어가 `${user_config.*}` 값을 대체했습니다.

세션 중간에 플러그인을 비활성화해도 이미 실행 중인 monitors는 중지되지 않습니다. 세션이 끝날 때 중지됩니다.

<h3 id="themes">
  Themes
</h3>

플러그인은 `/theme`에 기본 제공 프리셋 및 사용자의 로컬 테마와 함께 나타나는 색상 테마를 제공할 수 있습니다. 테마는 `themes/` 디렉토리의 JSON 파일로, `base` 프리셋과 색상 토큰의 sparse `overrides` 맵을 포함합니다. Themes는 [실험적 컴포넌트](#experimental-components)입니다.

```json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

플러그인 테마를 선택하면 사용자의 구성에 `custom:<plugin-name>:<slug>`이 유지됩니다. 플러그인 테마는 읽기 전용입니다. `/theme`에서 하나에 `Ctrl+E`를 누르면 `~/.claude/themes/`로 복사되어 사용자가 복사본을 편집할 수 있습니다.

***

<h2 id="plugin-installation-scopes">
  플러그인 설치 범위
</h2>

플러그인을 설치할 때 플러그인이 사용 가능한 위치와 다른 사람이 사용할 수 있는지를 결정하는 **범위**를 선택합니다:

| 범위        | 설정 파일                                  | 사용 사례                          |
| :-------- | :------------------------------------- | :----------------------------- |
| `user`    | `~/.claude/settings.json`              | 모든 프로젝트에서 사용 가능한 개인 플러그인 (기본값) |
| `project` | `.claude/settings.json`                | 버전 제어를 통해 공유되는 팀 플러그인          |
| `local`   | `.claude/settings.local.json`          | 프로젝트별 플러그인, gitignored         |
| `managed` | [관리되는 설정](/docs/ko/settings#settings-files) | 관리되는 플러그인 (읽기 전용, 업데이트만 가능)    |

플러그인은 다른 Claude Code 구성과 동일한 범위 시스템을 사용합니다. 설치 지침 및 범위 플래그는 [플러그인 설치](/docs/ko/discover-plugins#install-plugins)를 참조하세요. 범위에 대한 완전한 설명은 [구성 범위](/docs/ko/settings#configuration-scopes)를 참조하세요.

***

<h2 id="skills-directory-plugins">
  Skills-directory 플러그인
</h2>

skills 디렉토리 아래의 모든 폴더가 `.claude-plugin/plugin.json` 매니페스트를 포함하면 다음 세션에서 `<name>@skills-dir`이라는 플러그인으로 로드되며, 마켓플레이스나 설치 단계가 없습니다. [`plugin init`](#plugin-init)으로 스캐폴드하세요. 마켓플레이스 설치와 달리 플러그인은 플러그인 캐시에 복사되지 않고 제자리에서 발견됩니다.

skills 디렉토리 트리는 세 가지 서로 다른 것을 지원합니다:

| 무엇을 가지고 있는지                                   | 무엇인지                                                                 |
| :-------------------------------------------- | :------------------------------------------------------------------- |
| 매니페스트가 없는 `<skills-dir>/foo/SKILL.md`         | `foo`라는 일반 [skill](/docs/ko/skills)                                       |
| `<skills-dir>/foo/.claude-plugin/plugin.json` | `foo@skills-dir` 플러그인으로, 자체 skills, agents, hooks 등을 번들로 제공할 수 있습니다. |
| `<plugin>/skills/bar/SKILL.md`                | 플러그인 내에 패키지된 `bar` skill                                             |

<h3 id="choose-where-the-plugin-loads-from">
  플러그인이 로드되는 위치 선택
</h3>

| Skills 디렉토리             | 범위       | 로드                                             |
| :---------------------- | :------- | :--------------------------------------------- |
| `~/.claude/skills/`     | personal | 위치가 당신의 것이므로 모든 프로젝트에서                         |
| `<cwd>/.claude/skills/` | project  | 해당 폴더에 대한 작업 공간 [신뢰 대화](/docs/ko/settings)를 수락한 후에만 |

프로젝트 범위 플러그인은 저장소에 체크인되고 복제하는 모든 협력자에게 도달합니다. 해당 콘텐츠는 저장소에서 오므로 `.claude/settings.json`을 관리하는 것과 동일한 신뢰 게이트 후에만 로드되며, 코드를 실행하는 컴포넌트는 추가로 제한됩니다:

* 선언하는 MCP servers는 프로젝트 `.mcp.json`과 동일한 [서버별 승인](/docs/ko/mcp)을 거칩니다.
* LSP servers는 작업 공간을 신뢰한 후에만 시작됩니다.
* [백그라운드 monitors](#monitors)는 로드되지 않습니다.

개인 범위 플러그인에는 이러한 제한이 없습니다.

<Warning>
  프로젝트 범위 `@skills-dir` 플러그인은 Claude Code를 시작하는 디렉토리의 `.claude/skills/`에서만 로드됩니다. 일반 skills 및 commands가 하는 것처럼 [저장소 루트로 이동](/docs/ko/skills#automatic-discovery-from-parent-and-nested-directories)하지 않으므로 서브디렉토리에서 시작하면 저장소 루트에 있는 플러그인을 놓칩니다. 저장소 루트에서 시작하거나 디렉토리를 변경한 후 `/reload-plugins`를 실행하세요.
</Warning>

<h3 id="edit-reload-and-disable-a-skills-directory-plugin">
  Skills-directory 플러그인 편집, 다시 로드 및 비활성화
</h3>

skill의 `SKILL.md`에 대한 변경 사항은 현재 세션에서 즉시 적용됩니다. `hooks/`, `.mcp.json`, `agents/` 및 `output-styles/`와 같은 플러그인의 다른 컴포넌트에 대한 변경 사항은 그렇지 않습니다. `/reload-plugins`를 실행하거나 Claude Code를 다시 시작하여 이들을 선택하세요. [라이브 변경 감지](/docs/ko/skills#live-change-detection)를 참조하세요.

skills-directory 플러그인 로드를 중지하려면 해당 폴더를 삭제하거나 이름으로 비활성화하세요. 마켓플레이스에서 아무것도 설치되지 않았으므로 `uninstall` 단계가 없습니다.

```bash theme={null}
claude plugin disable my-tool@skills-dir
```

***

<h2 id="plugin-manifest-schema">
  플러그인 매니페스트 스키마
</h2>

`.claude-plugin/plugin.json` 파일은 플러그인의 메타데이터 및 구성을 정의합니다. 이 섹션은 지원되는 모든 필드 및 옵션을 문서화합니다.

매니페스트는 선택사항입니다. 생략하면 Claude Code는 [기본 위치](#file-locations-reference)에서 컴포넌트를 자동으로 발견하고 디렉토리 이름에서 플러그인 이름을 파생합니다. 메타데이터를 제공하거나 사용자 정의 컴포넌트 경로가 필요할 때 매니페스트를 사용하세요.

<h3 id="complete-schema">
  완전한 스키마
</h3>

```json theme={null}
{
  "name": "plugin-name",
  "displayName": "Plugin Name",
  "version": "1.2.0",
  "description": "Brief plugin description",
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://github.com/author"
  },
  "homepage": "https://docs.example.com/plugin",
  "repository": "https://github.com/author/plugin",
  "license": "MIT",
  "keywords": ["keyword1", "keyword2"],
  "skills": "./custom/skills/",
  "commands": ["./custom/commands/special.md"],
  "agents": ["./custom/agents/reviewer.md"],
  "hooks": "./config/hooks.json",
  "mcpServers": "./mcp-config.json",
  "outputStyles": "./styles/",
  "lspServers": "./.lsp.json",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./monitors.json"
  },
  "dependencies": [
    "helper-lib",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

<h3 id="required-fields">
  필수 필드
</h3>

매니페스트를 포함하는 경우 `name`이 유일한 필수 필드입니다.

| 필드     | 타입     | 설명                                                                                                                                                            | 예시                   |
| :----- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------- |
| `name` | string | 고유 식별자 (kebab-case, 공백 없음). [마켓플레이스 항목](/docs/ko/plugin-marketplaces#plugin-entries)이 플러그인을 다른 이름으로 나열할 때 마켓플레이스 항목 이름이 `enabledPlugins` 키 및 `/plugin`이 사용하는 것입니다. | `"deployment-tools"` |

이 이름은 컴포넌트 네임스페이싱에 사용됩니다. 예를 들어 UI에서 이름이 `plugin-dev`인 플러그인의 agent `agent-creator`는 `plugin-dev:agent-creator`로 나타납니다.

<h3 id="unrecognized-fields">
  인식되지 않은 필드
</h3>

Claude Code는 인식하지 못하는 최상위 필드를 무시합니다. 다른 생태계의 메타데이터를 `plugin.json`에 유지할 수 있으며 플러그인은 여전히 로드됩니다. 이를 통해 VS Code 또는 Cursor 확장 매니페스트, npm `package.json` 또는 MCPB/DXT 번들 매니페스트로도 작동하는 하나의 매니페스트를 유지하는 것이 실용적입니다.

`claude plugin validate`는 인식되지 않은 필드를 오류가 아닌 경고로 보고합니다. 필드가 인식된 필드와 한두 글자 차이나면 경고는 의도된 이름을 제안합니다. 인식되지 않은 필드 경고만 있는 플러그인은 여전히 검증을 통과하고 런타임에 로드됩니다.

잘못된 타입의 필드는 여전히 실패합니다. 예를 들어 `keywords` 값이 배열 대신 문자열인 경우 로드 오류이며 `claude plugin validate`는 이를 오류로 보고합니다.

`--strict`를 전달하여 경고를 오류로 취급합니다. CI에서 이를 사용하여 플러그인이 런타임에 로드되더라도 게시하기 전에 오타가 난 필드 이름이나 다른 도구의 매니페스트에서 남겨진 필드를 포착합니다.

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h3 id="metadata-fields">
  메타데이터 필드
</h3>

| 필드               | 타입      | 설명                                                                                                                                                                                                                  | 예시                                                                |
| :--------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------- |
| `$schema`        | string  | 편집기 자동 완성 및 검증을 위한 JSON Schema URL. Claude Code는 로드 시 이 필드를 무시합니다.                                                                                                                                                  | `"https://json.schemastore.org/claude-code-plugin-manifest.json"` |
| `displayName`    | string  | `/plugin` 선택기 및 기타 UI 표면에 표시되는 사람이 읽을 수 있는 이름입니다. 생략하면 `name`으로 폴백됩니다. `name`과 달리 공백과 모든 대소문자를 포함할 수 있습니다. 네임스페이싱 또는 조회에 사용되지 않습니다. Claude Code v2.1.143 이상이 필요합니다.                                                 | `"Deployment Tools"`                                              |
| `version`        | string  | 선택사항. 의미 있는 버전입니다. 이를 설정하면 플러그인이 해당 버전 문자열로 고정되므로 사용자는 버전을 올릴 때만 업데이트를 받습니다. 생략하면 Claude Code는 git 커밋 SHA로 폴백되므로 모든 커밋이 새 버전으로 취급됩니다. 마켓플레이스 항목에도 설정된 경우 `plugin.json`이 우선합니다. [버전 관리](#version-management)를 참조하세요. | `"2.1.0"`                                                         |
| `description`    | string  | 플러그인 목적에 대한 간단한 설명                                                                                                                                                                                                  | `"배포 자동화 도구"`                                                     |
| `author`         | object  | 작성자 정보                                                                                                                                                                                                              | `{"name": "Dev Team", "email": "dev@company.com"}`                |
| `homepage`       | string  | 문서 URL                                                                                                                                                                                                              | `"https://docs.example.com"`                                      |
| `repository`     | string  | 소스 코드 URL                                                                                                                                                                                                           | `"https://github.com/user/plugin"`                                |
| `license`        | string  | 라이선스 식별자                                                                                                                                                                                                            | `"MIT"`, `"Apache-2.0"`                                           |
| `keywords`       | array   | 발견 태그                                                                                                                                                                                                               | `["deployment", "ci-cd"]`                                         |
| `defaultEnabled` | boolean | 사용자가 설정하지 않았을 때 플러그인이 활성화된 상태로 시작할지 여부입니다. 기본값은 `true`입니다. [기본 활성화](#default-enablement)를 참조하세요. Claude Code v2.1.154 이상이 필요합니다.                                                                                    | `false`                                                           |

<h3 id="default-enablement">
  기본 활성화
</h3>

`plugin.json`에서 `defaultEnabled: false`를 설정하여 비활성화된 상태로 설치되는 플러그인을 제공하세요. 사용자는 `claude plugin enable <plugin>` 또는 `/plugin` 인터페이스로 켭니다. 외부 서비스에 연결하는 플러그인과 같이 사용자가 옵트인해야 하는 비용이나 범위를 추가하는 플러그인에 사용하세요. 이는 Claude Code v2.1.154 이상이 필요합니다. 이전 버전은 필드를 무시하고 설치 시 플러그인을 활성화합니다.

`defaultEnabled`는 다른 것이 플러그인의 상태를 결정하지 않았을 때의 폴백입니다. 두 가지가 이를 우선합니다:

* **사용자의 설정**: 모든 설정 범위에서 플러그인에 대한 `enabledPlugins`의 항목입니다. 작성되면 플러그인 업데이트 및 재설치 전체에서 유지되므로 나중 릴리스에서 `defaultEnabled`를 변경해도 기존 사용자를 뒤집지 않습니다.
* **종속성 요구 사항**: 플러그인이 활성화된 다른 플러그인에 의해 필요할 때 Claude Code는 설치 또는 활성화 시 `true`를 작성합니다. 이는 명시적 설정을 제공하므로 자체 기본값이 더 이상 적용되지 않습니다. [종속성이 있는 플러그인 활성화 또는 비활성화](/docs/ko/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies)를 참조하세요.

동일한 필드가 플러그인의 마켓플레이스 항목에 나타날 수 있으며, 여기서 `plugin.json`의 값보다 우선합니다. [선택사항 플러그인 필드](/docs/ko/plugin-marketplaces#optional-plugin-fields)를 참조하세요.

<h3 id="component-path-fields">
  컴포넌트 경로 필드
</h3>

| 필드                      | 타입                    | 설명                                                                                                                      | 예시                                                   |
| :---------------------- | :-------------------- | :---------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `skills`                | string\|array         | `<name>/SKILL.md`를 포함하는 사용자 정의 skill 디렉토리 (기본 `skills/` 외에 추가). [경로 동작 규칙](#path-behavior-rules)에서 마켓플레이스 루트 예외를 참조하세요. | `"./custom/skills/"`                                 |
| `commands`              | string\|array         | 사용자 정의 평면 `.md` skill 파일 또는 디렉토리 (기본 `commands/` 대체)                                                                    | `"./custom/cmd.md"` 또는 `["./cmd1.md"]`               |
| `agents`                | string\|array         | 사용자 정의 agent 파일 (기본 `agents/` 대체)                                                                                       | `"./custom/agents/reviewer.md"`                      |
| `hooks`                 | string\|array\|object | Hook 구성 경로 또는 인라인 구성                                                                                                    | `"./my-extra-hooks.json"`                            |
| `mcpServers`            | string\|array\|object | MCP 구성 경로 또는 인라인 구성                                                                                                     | `"./my-extra-mcp-config.json"`                       |
| `outputStyles`          | string\|array         | 사용자 정의 출력 스타일 파일/디렉토리 (기본 `output-styles/` 대체)                                                                          | `"./styles/"`                                        |
| `lspServers`            | string\|array\|object | [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) 코드 인텔리전스 구성 (정의로 이동, 참조 찾기 등)         | `"./.lsp.json"`                                      |
| `experimental.themes`   | string\|array         | 색상 테마 파일/디렉토리 (기본 `themes/` 대체). [테마](#themes) 참조                                                                       | `"./themes/"`                                        |
| `experimental.monitors` | string\|array         | 플러그인이 활성화될 때 자동으로 시작되는 백그라운드 [Monitor](/docs/ko/tools-reference#monitor-tool) 구성. [Monitors](#monitors) 참조                   | `"./monitors.json"`                                  |
| `userConfig`            | object                | 플러그인이 활성화될 때 사용자에게 프롬프트하는 사용자 구성 가능 값. [사용자 구성](#user-configuration) 참조                                                 | 아래 참조                                                |
| `channels`              | array                 | 메시지 주입을 위한 채널 선언 (Telegram, Slack, Discord 스타일). [채널](#channels) 참조                                                     | 아래 참조                                                |
| `dependencies`          | array                 | 이 플러그인이 필요로 하는 다른 플러그인, 선택적으로 semver 버전 제약 포함. [플러그인 종속성 버전 제약](/docs/ko/plugin-dependencies) 참조                             | `[{ "name": "secrets-vault", "version": "~2.1.0" }]` |

<h3 id="experimental-components">
  실험적 컴포넌트
</h3>

`experimental` 키 아래의 컴포넌트인 `themes` 및 `monitors`는 안정화되는 동안 릴리스 간에 변경될 수 있는 매니페스트 스키마를 가집니다. 이들을 선언하는 위치는 별도의 마이그레이션입니다. 최상위 수준은 여전히 작동하고, `claude plugin validate`는 경고하며, 향후 릴리스에서는 `experimental.*`이 필요합니다.

<h3 id="user-configuration">
  사용자 구성
</h3>

`userConfig` 필드는 플러그인이 활성화될 때 Claude Code가 사용자에게 프롬프트하는 값을 선언합니다. 사용자가 `settings.json`을 수동으로 편집하도록 요구하는 대신 이를 사용하세요.

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API 엔드포인트",
      "description": "팀의 API 엔드포인트"
    },
    "api_token": {
      "type": "string",
      "title": "API 토큰",
      "description": "API 인증 토큰",
      "sensitive": true
    }
  }
}
```

키는 유효한 식별자여야 합니다. 각 옵션은 다음 필드를 지원합니다:

| 필드            | 필수  | 설명                                                        |
| :------------ | :-- | :-------------------------------------------------------- |
| `type`        | 예   | `string`, `number`, `boolean`, `directory` 또는 `file` 중 하나 |
| `title`       | 예   | 구성 대화에 표시되는 레이블                                           |
| `description` | 예   | 필드 아래에 표시되는 도움말 텍스트                                       |
| `sensitive`   | 아니오 | `true`인 경우 입력을 마스킹하고 값을 `settings.json` 대신 보안 저장소에 저장합니다. |
| `required`    | 아니오 | `true`인 경우 필드가 비어 있으면 검증이 실패합니다.                          |
| `default`     | 아니오 | 사용자가 아무것도 제공하지 않을 때 사용되는 값                                |
| `multiple`    | 아니오 | `string` 타입의 경우 문자열 배열 허용                                 |
| `min` / `max` | 아니오 | `number` 타입의 범위                                           |

각 값은 MCP 및 LSP 서버 구성과 hook 명령어에서 `${user_config.KEY}`로 대체할 수 있습니다. 민감하지 않은 값은 skill 및 agent 콘텐츠에서도 대체할 수 있습니다. 모든 값은 hook 프로세스에 `CLAUDE_PLUGIN_OPTION_<KEY>` 환경 변수로 내보내집니다. 여기서 `<KEY>`는 옵션 키를 대문자로 표기한 것입니다.

셸에서 실행되는 필드는 `${user_config.*}`를 거부합니다. 구성된 값을 셸 명령어에 대체하면 셸이 해당 값이 포함하는 모든 것을 실행할 수 있으므로 컴포넌트는 [오류](/docs/ko/errors#plugin-command-references-user-config)로 실패합니다. 거부된 각 필드에는 값을 전달하는 대체 방법이 있습니다:

| 거부된 필드                                                                       | 값을 전달하는 방법                                                                                                       |
| :--------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| Shell-form hook 명령어                                                          | [exec form](/docs/ko/hooks#exec-form-and-shell-form)을 `args`와 함께 사용하거나 hook의 환경에서 `CLAUDE_PLUGIN_OPTION_<KEY>`를 읽습니다. |
| [Monitor](#monitors) 명령어                                                     | 스크립트의 구성 파일에서 값을 읽습니다.                                                                                           |
| MCP [`headersHelper`](/docs/ko/mcp#use-dynamic-headers-for-custom-authentication) | 스크립트의 구성 파일에서 값을 읽습니다.                                                                                           |

v2.1.207 이전에는 이러한 필드가 `${user_config.KEY}` 값을 대체했습니다. 이에 의존하는 플러그인을 업데이트하세요.

민감하지 않은 값은 `settings.json`의 `pluginConfigs[<plugin-id>].options` 아래에 저장됩니다. Claude Code는 키를 사용자 설정에 작성하고 사용자 설정, `--settings` 플래그 및 관리되는 설정에서만 읽습니다. 프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json`의 항목은 무시됩니다. v2.1.207 이전에는 Claude Code가 프로젝트 및 로컬 설정도 읽었습니다.

민감한 값은 macOS Keychain으로 이동하거나, 지원되는 키체인을 사용할 수 없는 플랫폼에서는 `~/.claude/.credentials.json`으로 이동합니다. 키체인 저장소는 OAuth 토큰과 공유되며 약 2 KB의 총 제한이 있으므로 민감한 값을 작게 유지하세요.

<h3 id="channels">
  채널
</h3>

`channels` 필드를 사용하면 플러그인이 하나 이상의 메시지 채널을 선언하여 대화에 콘텐츠를 주입할 수 있습니다. 각 채널은 플러그인이 제공하는 MCP 서버에 바인딩됩니다.

```json theme={null}
{
  "channels": [
    {
      "server": "telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "봇 토큰",
          "description": "Telegram 봇 토큰",
          "sensitive": true
        },
        "owner_id": {
          "type": "string",
          "title": "소유자 ID",
          "description": "Telegram 사용자 ID"
        }
      }
    }
  ]
}
```

`server` 필드는 필수이며 플러그인의 `mcpServers`의 키와 일치해야 합니다. 선택사항인 채널별 `userConfig`는 최상위 필드와 동일한 스키마를 사용하여 플러그인이 플러그인이 활성화될 때 봇 토큰 또는 소유자 ID를 프롬프트할 수 있습니다.

<h3 id="path-behavior-rules">
  경로 동작 규칙
</h3>

사용자 정의 경로가 플러그인의 기본 디렉토리를 대체하는지 확장하는지는 필드에 따라 다릅니다:

* **기본값 대체**: `commands`, `agents`, `outputStyles`, `experimental.themes`, `experimental.monitors`. 예를 들어 매니페스트가 `commands`를 지정하면 기본 `commands/` 디렉토리는 스캔되지 않습니다. 기본값을 유지하고 더 많은 것을 추가하려면 명시적으로 나열하세요: `"commands": ["./commands/", "./extras/"]`
* **기본값에 추가**: `skills`. 기본 `skills/` 디렉토리는 항상 스캔되며, `skills`에 나열된 디렉토리는 함께 로드됩니다. 예외: [소스가 마켓플레이스 루트로 확인되는 마켓플레이스 항목](/docs/ko/plugin-marketplaces#advanced-plugin-entries)의 경우 특정 서브디렉토리를 선언하면 기본 `skills/` 스캔을 대체합니다.
* **자체 병합 규칙**: [hooks](#hooks), [MCP servers](#mcp-servers) 및 [LSP servers](#lsp-servers). 각 섹션에서 여러 소스가 어떻게 결합되는지 참조하세요.

플러그인에 기본 폴더와 일치하는 매니페스트 키가 모두 있으면 Claude Code v2.1.140 이상은 `claude plugin list` 및 `/plugin` 상세 보기에서 무시된 폴더에 플래그를 지정합니다. 플러그인은 여전히 매니페스트 경로를 사용하여 로드됩니다. 매니페스트 키가 기본 폴더를 가리킬 때는 경고가 표시되지 않습니다 (예: `"commands": ["./commands/deploy.md"]`). 이 경우 폴더가 명시적으로 처리되기 때문입니다.

모든 경로 필드의 경우:

* 모든 경로는 플러그인 루트에 상대적이어야 하며 `./`로 시작해야 합니다.
* 사용자 정의 경로의 컴포넌트는 동일한 명명 및 네임스페이싱 규칙을 사용합니다.
* 여러 경로를 배열로 지정할 수 있습니다.
* skill 경로가 `SKILL.md`를 직접 포함하는 디렉토리를 가리킬 때 (예: 플러그인 루트를 가리키는 `"skills": ["./"]`), frontmatter의 `name` 필드가 skill의 호출 이름을 결정합니다. 이는 설치 디렉토리와 관계없이 안정적인 이름을 제공합니다. `name`이 frontmatter에 설정되지 않으면 디렉토리 basename이 폴백으로 사용됩니다.

플러그인이 루트에 `SKILL.md`를 가지고 있고, `skills/` 서브디렉토리가 없으며, `skills` 매니페스트 필드가 없으면 Claude Code v2.1.142 이상에서 자동으로 단일 skill 플러그인으로 로드됩니다. 이 레이아웃에 대해 `plugin.json`에서 `"skills": ["./"]`를 설정할 필요가 없습니다. skill의 호출 이름은 위와 동일한 규칙을 따릅니다: frontmatter `name` 필드 또는 디렉토리 basename을 폴백으로 사용합니다.

**경로 예시**:

```json theme={null}
{
  "commands": [
    "./specialized/deploy.md",
    "./utilities/batch-process.md"
  ],
  "agents": [
    "./custom-agents/reviewer.md",
    "./custom-agents/tester.md"
  ]
}
```

<h3 id="environment-variables">
  환경 변수
</h3>

Claude Code는 플러그인 경로를 참조하기 위한 세 가지 변수를 제공합니다:

| 변수                      | 확인 대상                                                                     | 사용 목적                                                   |
| :---------------------- | :------------------------------------------------------------------------ | :------------------------------------------------------ |
| `${CLAUDE_PLUGIN_ROOT}` | 플러그인 설치 디렉토리의 절대 경로                                                       | 플러그인과 함께 번들로 제공되는 스크립트, 바이너리 및 구성 파일                    |
| `${CLAUDE_PLUGIN_DATA}` | [영구 데이터 디렉토리](#persistent-data-directory) (첫 참조 시 생성되며 플러그인 업데이트 후에도 유지됨) | `node_modules` 또는 Python 가상 환경과 같은 설치된 종속성, 생성된 코드 및 캐시 |
| `${CLAUDE_PROJECT_DIR}` | 프로젝트 루트                                                                   | 프로젝트 로컬 스크립트 및 구성 파일                                    |

세 변수 모두 hook 프로세스 및 MCP 및 LSP 서버 서브프로세스에 환경 변수로 내보내집니다. 어느 필드가 인라인으로 플레이스홀더를 대체하는지는 플러그인 컴포넌트에 따라 다릅니다:

| 플러그인 컴포넌트                  | 플레이스홀더가 확인되는 필드                             |
| :------------------------- | :------------------------------------------ |
| Skill 및 agent 콘텐츠          | 플레이스홀더가 나타나는 모든 곳                           |
| Hook 및 monitor 명령어         | 플레이스홀더가 나타나는 모든 곳                           |
| MCP `stdio` 서버             | `command`, `args`, `env`                    |
| MCP `http`, `sse`, `ws` 서버 | `url`, `headers`, `headersHelper`           |
| LSP 서버                     | `command`, `args`, `env`, `workspaceFolder` |

hook 명령어에서 [exec form](/docs/ko/hooks#exec-form-and-shell-form)을 `args`와 함께 사용하여 각 경로가 따옴표 없이 하나의 인수로 전달되도록 하세요. shell-form hook 및 monitor 명령어에서 `"${CLAUDE_PROJECT_DIR}/scripts/server.sh"`와 같이 큰따옴표로 변수를 감싸세요. 이 shell-form hook은 플러그인과 함께 번들된 스크립트를 실행합니다:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

`${CLAUDE_PLUGIN_ROOT}`는 플러그인이 업데이트될 때 변경됩니다. 이전 버전의 디렉토리는 업데이트 후 약 7일 동안 디스크에 남아 있지만 이를 임시로 취급하고 여기에 상태를 작성하지 마세요.

플러그인이 세션 중에 업데이트될 때 hook 명령어, monitors, MCP 서버 및 LSP 서버는 이전 버전의 경로를 계속 사용합니다. `/reload-plugins`를 실행하여 hook, MCP 서버 및 LSP 서버를 새 경로로 전환하세요. monitors는 세션 재시작이 필요합니다.

MCP 서버는 또한 `roots/list` 요청을 호출하여 런타임에 세션의 작업 디렉토리를 읽을 수 있습니다. [`roots/list`가 반환하는 것과 Claude Code가 서버에 변경을 알리는 시기](/docs/ko/mcp#option-3-add-a-local-stdio-server)를 참조하세요.

<h4 id="persistent-data-directory">
  영구 데이터 디렉토리
</h4>

`${CLAUDE_PLUGIN_DATA}` 디렉토리는 `~/.claude/plugins/data/{id}/`로 확인되며, 여기서 `{id}`는 `a-z`, `A-Z`, `0-9`, `_` 및 `-` 외부의 문자가 `-`로 대체된 플러그인 식별자입니다. `formatter@my-marketplace`로 설치된 플러그인의 경우 디렉토리는 `~/.claude/plugins/data/formatter-my-marketplace/`입니다.

일반적인 사용은 언어 종속성을 한 번 설치하고 세션 및 플러그인 업데이트 전체에서 재사용하는 것입니다. 데이터 디렉토리가 단일 플러그인 버전보다 오래 지속되므로 디렉토리 존재 여부만 확인하면 업데이트가 플러그인의 종속성 매니페스트를 변경할 때를 감지할 수 없습니다. 권장 패턴은 번들된 매니페스트를 데이터 디렉토리의 복사본과 비교하고 다를 때 다시 설치합니다.

이 `SessionStart` hook은 첫 실행 시 `node_modules`를 설치하고 플러그인 업데이트가 변경된 `package.json`을 포함할 때마다 다시 설치합니다:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "diff -q \"${CLAUDE_PLUGIN_ROOT}/package.json\" \"${CLAUDE_PLUGIN_DATA}/package.json\" >/dev/null 2>&1 || (cd \"${CLAUDE_PLUGIN_DATA}\" && cp \"${CLAUDE_PLUGIN_ROOT}/package.json\" . && npm install) || rm -f \"${CLAUDE_PLUGIN_DATA}/package.json\""
          }
        ]
      }
    ]
  }
}
```

`diff`는 저장된 복사본이 누락되거나 번들된 복사본과 다를 때 0이 아닌 값으로 종료되어 첫 실행과 종속성 변경 업데이트를 모두 다룹니다. `npm install`이 실패하면 후행 `rm`은 복사된 매니페스트를 제거하므로 다음 세션이 다시 시도합니다.

`${CLAUDE_PLUGIN_ROOT}`에 번들된 스크립트는 지속된 `node_modules`에 대해 실행할 수 있습니다:

```json theme={null}
{
  "mcpServers": {
    "routines": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": {
        "NODE_PATH": "${CLAUDE_PLUGIN_DATA}/node_modules"
      }
    }
  }
}
```

데이터 디렉토리는 플러그인을 설치한 마지막 범위에서 제거할 때 자동으로 삭제됩니다. `/plugin` 인터페이스는 디렉토리 크기를 표시하고 삭제 전에 프롬프트합니다. CLI는 기본적으로 삭제합니다. [`--keep-data`](#plugin-uninstall)를 전달하여 유지하세요.

***

<h2 id="plugin-caching-and-file-resolution">
  플러그인 캐싱 및 파일 해석
</h2>

플러그인은 두 가지 방법 중 하나로 지정됩니다:

* `claude --plugin-dir` 또는 `claude --plugin-url`을 통해, 세션 기간 동안.
* 마켓플레이스를 통해, 향후 세션을 위해 설치됨.

보안 및 검증 목적으로 Claude Code는 *마켓플레이스* 플러그인을 제자리에서 사용하는 대신 사용자의 로컬 **플러그인 캐시** (`~/.claude/plugins/cache`)에 복사합니다. 외부 파일을 참조하는 플러그인을 개발할 때 이 동작을 이해하는 것이 중요합니다.

각 설치된 버전은 캐시의 별도 디렉토리입니다. 플러그인을 업데이트하거나 제거하면 이전 버전 디렉토리는 고아로 표시되고 7일 후 자동으로 제거됩니다. 유예 기간을 통해 이미 이전 버전을 로드한 동시 Claude Code 세션이 오류 없이 계속 실행될 수 있습니다.

Claude의 Glob 및 Grep 도구는 검색 중에 고아 버전 디렉토리를 건너뛰므로 파일 결과에는 오래된 플러그인 코드가 포함되지 않습니다.

<h3 id="path-traversal-limitations">
  경로 순회 제한
</h3>

설치된 플러그인은 해당 디렉토리 외부의 파일을 참조할 수 없습니다. 플러그인 루트 외부를 순회하는 경로(예: `../shared-utils`)는 설치 후 작동하지 않습니다. 왜냐하면 이러한 외부 파일이 캐시에 복사되지 않기 때문입니다.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  마켓플레이스 내에서 심볼릭 링크를 사용하여 파일 공유
</h3>

플러그인이 동일한 마켓플레이스의 다른 부분과 파일을 공유해야 하는 경우 플러그인 디렉토리 내에 심볼릭 링크를 만들 수 있습니다. 플러그인이 캐시에 복사될 때 심볼릭 링크가 처리되는 방식은 해당 대상이 어디로 해석되는지에 따라 달라집니다:

* **플러그인 자체 디렉토리 내:** 심볼릭 링크는 캐시에 상대 심볼릭 링크로 보존되므로 런타임에 복사된 대상으로 계속 해석됩니다.
* **동일한 마켓플레이스 내의 다른 곳:** 심볼릭 링크는 역참조됩니다. 대상의 콘텐츠가 캐시에 복사됩니다. 이를 통해 메타 플러그인의 `skills/` 디렉토리가 마켓플레이스의 다른 플러그인으로 정의된 skills에 링크할 수 있습니다.
* **마켓플레이스 외부:** 심볼릭 링크는 보안상의 이유로 건너뜁니다. 이는 플러그인이 시스템 경로와 같은 임의의 호스트 파일을 캐시로 가져오는 것을 방지합니다.

`--plugin-dir`으로 설치되거나 로컬 경로에서 설치된 플러그인의 경우 플러그인 자체 디렉토리 내에서 해석되는 심볼릭 링크만 보존됩니다. 다른 모든 것은 건너뜁니다.

다음 명령어는 마켓플레이스 플러그인 내부에서 형제 플러그인으로 정의된 공유 skill로의 링크를 만듭니다. Windows에서는 관리자 권한 명령 프롬프트에서 `mklink /D`를 사용하거나 개발자 모드를 활성화하세요:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

이는 캐싱 시스템의 보안 이점을 유지하면서 유연성을 제공합니다.

***

<h2 id="plugin-directory-structure">
  플러그인 디렉토리 구조
</h2>

<h3 id="standard-plugin-layout">
  표준 플러그인 레이아웃
</h3>

완전한 플러그인은 다음 구조를 따릅니다:

```text theme={null}
enterprise-plugin/
├── .claude-plugin/           # 메타데이터 디렉토리 (선택사항)
│   └── plugin.json             # 플러그인 매니페스트
├── skills/                   # Skills
│   ├── code-reviewer/
│   │   └── SKILL.md
│   └── pdf-processor/
│       ├── SKILL.md
│       └── scripts/
├── commands/                 # 평면 .md 파일로서의 Skills
│   ├── status.md
│   └── logs.md
├── agents/                   # Subagent 정의
│   ├── security-reviewer.md
│   ├── performance-tester.md
│   └── compliance-checker.md
├── output-styles/            # 출력 스타일 정의
│   └── terse.md
├── themes/                   # 색상 테마 정의
│   └── dracula.json
├── monitors/                 # 백그라운드 모니터 구성
│   └── monitors.json
├── hooks/                    # Hook 구성
│   ├── hooks.json           # 주 hook 구성
│   └── security-hooks.json  # 추가 hooks
├── bin/                      # PATH에 추가된 플러그인 실행 파일
│   └── my-tool               # Bash tool에서 bare 명령어로 호출 가능
├── settings.json            # 플러그인의 기본 설정
├── .mcp.json                # MCP 서버 정의
├── .lsp.json                # LSP 서버 구성
├── scripts/                 # Hook 및 유틸리티 스크립트
│   ├── security-scan.sh
│   ├── format-code.py
│   └── deploy.js
├── LICENSE                  # 라이선스 파일
└── CHANGELOG.md             # 버전 기록
```

<Warning>
  `.claude-plugin/` 디렉토리는 `plugin.json` 파일을 포함합니다. 다른 모든 디렉토리 (commands/, agents/, skills/, output-styles/, themes/, monitors/, hooks/)는 `.claude-plugin/` 내부가 아닌 플러그인 루트에 있어야 합니다.
</Warning>

`CLAUDE.md` 파일이 플러그인 루트에 있어도 프로젝트 컨텍스트로 로드되지 않습니다. 플러그인은 `CLAUDE.md`가 아닌 skills, agents, hooks를 통해 컨텍스트를 제공합니다. Claude의 컨텍스트에 로드되는 지침을 제공하려면 [skill](#skills)에 배치하십시오.

<h3 id="file-locations-reference">
  파일 위치 참조
</h3>

| 컴포넌트              | 기본 위치                        | 목적                                                                                                                            |
| :---------------- | :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------- |
| **매니페스트**         | `.claude-plugin/plugin.json` | 플러그인 메타데이터 및 구성 (선택사항)                                                                                                        |
| **Skills**        | `skills/`                    | `<name>/SKILL.md` 구조의 Skills                                                                                                  |
| **Commands**      | `commands/`                  | 평면 마크다운 파일로서의 Skills. 새 플러그인에는 `skills/` 사용                                                                                   |
| **Agents**        | `agents/`                    | Subagent 마크다운 파일                                                                                                              |
| **Output styles** | `output-styles/`             | 출력 스타일 정의                                                                                                                     |
| **Themes**        | `themes/`                    | 색상 테마 정의                                                                                                                      |
| **Hooks**         | `hooks/hooks.json`           | Hook 구성                                                                                                                       |
| **MCP servers**   | `.mcp.json`                  | MCP 서버 정의                                                                                                                     |
| **LSP servers**   | `.lsp.json`                  | 언어 서버 구성                                                                                                                      |
| **Monitors**      | `monitors/monitors.json`     | 백그라운드 모니터 구성                                                                                                                  |
| **Executables**   | `bin/`                       | Bash tool의 `PATH`에 추가된 실행 파일. 여기의 파일은 플러그인이 활성화된 동안 모든 Bash tool 호출에서 bare 명령어로 호출 가능                                         |
| **Settings**      | `settings.json`              | 플러그인이 활성화될 때 적용되는 기본 구성. 현재 [`agent`](/docs/ko/sub-agents) 및 [`subagentStatusLine`](/docs/ko/statusline#subagent-status-lines) 키만 지원됩니다 |

***

<h2 id="cli-commands-reference">
  CLI 명령어 참조
</h2>

Claude Code는 스크립팅 및 자동화에 유용한 비대화형 플러그인 관리를 위한 CLI 명령어를 제공합니다.

<h3 id="plugin-init">
  plugin init
</h3>

`~/.claude/skills/<name>/`에서 새 플러그인을 스캐폴드합니다. 다음 Claude Code 세션에서 `<name>@skills-dir`으로 자동으로 로드되고 설치 단계 없이 `/plugin` 및 `claude plugin list`에 나타납니다.

[Skills-directory 플러그인](#skills-directory-plugins)에서 범위 및 신뢰 요구 사항을 참조하세요.

```bash theme={null}
claude plugin init <name> [options]
```

**인수:**

* `<name>`: 플러그인 이름. skill 네임스페이스 및 `~/.claude/skills/` 아래의 디렉토리 이름이 되므로 공백이나 경로 구분자를 포함할 수 없습니다.

**옵션:**

| 옵션                       | 설명                                                                                            | 기본값                     |
| :----------------------- | :-------------------------------------------------------------------------------------------- | :---------------------- |
| `--description <text>`   | 매니페스트 설명                                                                                      |                         |
| `--author <name>`        | 작성자 이름                                                                                        | `git config user.name`  |
| `--author-email <email>` | 작성자 이메일                                                                                       | `git config user.email` |
| `--with <components...>` | 컴포넌트 폴더도 스캐폴드합니다. 유효한 값: `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style`, `channel` |                         |
| `-f, --force`            | 대상의 기존 `.claude-plugin/` 덮어쓰기                                                                 |                         |
| `-h, --help`             | 명령어 도움말 표시                                                                                    |                         |

**별칭:** `new`

각 `--with` 값은 해당 컴포넌트에 대한 스타터 파일을 추가하여 편집할 준비가 됩니다:

| 컴포넌트           | 스캐폴드하는 것                                                                                |
| :------------- | :-------------------------------------------------------------------------------------- |
| `skills`       | 기본 skill과 함께 추가 네임스페이스 `<name>:example` skill                                           |
| `agents`       | `agents/` subagent 정의                                                                   |
| `hooks`        | 샘플 이벤트 핸들러가 있는 `hooks/hooks.json`                                                       |
| `mcp`          | HTTP 및 stdio 서버 예시가 있는 `.mcp.json`                                                      |
| `lsp`          | 언어 서버 예시가 있는 `.lsp.json`                                                                |
| `output-style` | 플러그인이 활성화된 동안 자동으로 적용되는 `output-styles/<name>.md`                                       |
| `channel`      | MCP 기반 [channel](/docs/ko/channels): stdio 서버 (`server.ts`), 해당 `.mcp.json` 및 `package.json` |

스캐폴드된 플러그인은 마켓플레이스가 아닌 `@skills-dir` 소스를 사용합니다. 관리자는 [관리되는 설정](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions)에서 `strictKnownMarketplaces`로 이 소스를 차단하거나 `blockedMarketplaces`에 `{"source": "skills-dir"}`을 추가할 수 있습니다. 차단되면 `plugin init`은 작성하기 전에 실패합니다.

**예시:**

```bash theme={null}
# 최소 플러그인 스캐폴드
claude plugin init my-helper

# skill 및 hook 폴더로 스캐폴드
claude plugin init my-helper --with skills hooks

# 기존 스캐폴드 덮어쓰기
claude plugin init my-helper --force
```

<h3 id="plugin-install">
  plugin install
</h3>

사용 가능한 마켓플레이스에서 플러그인을 설치합니다.

```bash theme={null}
claude plugin install <plugin> [options]
```

**인수:**

* `<plugin>`: 플러그인 이름 또는 특정 마켓플레이스의 경우 `plugin-name@marketplace-name`

**옵션:**

| 옵션                    | 설명                                  | 기본값    |
| :-------------------- | :---------------------------------- | :----- |
| `-s, --scope <scope>` | 설치 범위: `user`, `project` 또는 `local` | `user` |
| `-h, --help`          | 명령어 도움말 표시                          |        |

범위는 설치된 플러그인이 추가되는 설정 파일을 결정합니다. 예를 들어 `--scope project`는 `.claude/settings.json`의 `enabledPlugins`에 쓰므로 프로젝트 저장소를 복제하는 모든 사람이 플러그인을 사용할 수 있습니다.

**예시:**

```bash theme={null}
# 사용자 범위에 설치 (기본값)
claude plugin install formatter@my-marketplace

# 프로젝트 범위에 설치 (팀과 공유)
claude plugin install formatter@my-marketplace --scope project

# 로컬 범위에 설치 (gitignored)
claude plugin install formatter@my-marketplace --scope local
```

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

설치된 플러그인을 제거합니다.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

**인수:**

* `<plugin>`: 플러그인 이름 또는 `plugin-name@marketplace-name`

**옵션:**

| 옵션                    | 설명                                                                     | 기본값    |
| :-------------------- | :--------------------------------------------------------------------- | :----- |
| `-s, --scope <scope>` | 범위에서 제거: `user`, `project` 또는 `local`                                  | `user` |
| `--keep-data`         | 플러그인의 [영구 데이터 디렉토리](#persistent-data-directory) 유지                     |        |
| `--prune`             | 다른 플러그인이 필요로 하지 않는 자동 설치된 종속성도 제거합니다. [plugin prune](#plugin-prune) 참조 |        |
| `-y, --yes`           | `--prune` 확인 프롬프트 건너뛰기. stdin이 TTY가 아닐 때 필수                            |        |
| `-h, --help`          | 명령어 도움말 표시                                                             |        |

**별칭:** `remove`, `rm`

기본적으로 마지막 남은 범위에서 제거하면 플러그인의 `${CLAUDE_PLUGIN_DATA}` 디렉토리도 삭제됩니다. 새 버전 테스트 후 재설치할 때와 같이 유지하려면 `--keep-data`를 사용하세요.

<h3 id="plugin-prune">
  plugin prune
</h3>

더 이상 설치된 플러그인에서 필요로 하지 않는 자동 설치된 플러그인 종속성을 제거합니다. Claude Code가 다른 플러그인의 [`dependencies`](/docs/ko/plugin-dependencies) 필드를 만족하기 위해 가져온 종속성은 제거되며, 직접 설치한 플러그인은 절대 건드리지 않습니다.

```bash theme={null}
claude plugin prune [options]
```

**옵션:**

| 옵션                    | 설명                                    | 기본값    |
| :-------------------- | :------------------------------------ | :----- |
| `-s, --scope <scope>` | 범위에서 정리: `user`, `project` 또는 `local` | `user` |
| `--dry-run`           | 제거될 항목을 나열하되 실제로 제거하지 않음              |        |
| `-y, --yes`           | 확인 프롬프트 건너뛰기. stdin이 TTY가 아닐 때 필수     |        |
| `-h, --help`          | 명령어 도움말 표시                            |        |

**별칭:** `autoremove`

명령어는 고아 종속성을 나열하고 제거하기 전에 확인을 요청합니다. 플러그인을 제거하고 한 단계에서 종속성을 정리하려면 `claude plugin uninstall <plugin> --prune`을 실행하세요.

<Note>
  `claude plugin prune`은 Claude Code v2.1.121 이상이 필요합니다.
</Note>

<h3 id="plugin-enable">
  plugin enable
</h3>

비활성화된 플러그인을 활성화합니다. 플러그인이 [종속성](/docs/ko/plugin-dependencies)을 선언하면 Claude Code는 동일한 범위에서 이들을 전이적으로 활성화하며, 종속성이 설치되지 않으면 명령어가 실패합니다.

```bash theme={null}
claude plugin enable <plugin> [options]
```

**인수:**

* `<plugin>`: 플러그인 이름 또는 `plugin-name@marketplace-name`

**옵션:**

| 옵션                    | 설명                                    | 기본값    |
| :-------------------- | :------------------------------------ | :----- |
| `-s, --scope <scope>` | 활성화할 범위: `user`, `project` 또는 `local` | `user` |
| `-h, --help`          | 명령어 도움말 표시                            |        |

<h3 id="plugin-disable">
  plugin disable
</h3>

플러그인을 제거하지 않고 비활성화합니다. 다른 활성화된 플러그인이 대상에 [종속되어](/docs/ko/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies) 있으면 실패합니다. 오류 메시지에는 먼저 모든 종속 플러그인을 비활성화하는 연쇄 명령어가 포함됩니다.

```bash theme={null}
claude plugin disable <plugin> [options]
```

**인수:**

* `<plugin>`: 플러그인 이름 또는 `plugin-name@marketplace-name`

**옵션:**

| 옵션                    | 설명                                     | 기본값    |
| :-------------------- | :------------------------------------- | :----- |
| `-s, --scope <scope>` | 비활성화할 범위: `user`, `project` 또는 `local` | `user` |
| `-h, --help`          | 명령어 도움말 표시                             |        |

<h3 id="plugin-update">
  plugin update
</h3>

플러그인을 최신 버전으로 업데이트합니다.

```bash theme={null}
claude plugin update <plugin> [options]
```

**인수:**

* `<plugin>`: 플러그인 이름 또는 `plugin-name@marketplace-name`

**옵션:**

| 옵션                    | 설명                                                | 기본값    |
| :-------------------- | :------------------------------------------------ | :----- |
| `-s, --scope <scope>` | 업데이트할 범위: `user`, `project`, `local` 또는 `managed` | `user` |
| `-h, --help`          | 명령어 도움말 표시                                        |        |

***

<h3 id="plugin-list">
  plugin list
</h3>

설치된 플러그인을 버전, 소스 마켓플레이스 및 활성화 상태와 함께 나열합니다.

```bash theme={null}
claude plugin list [options]
```

**옵션:**

| 옵션            | 설명                                   | 기본값 |
| :------------ | :----------------------------------- | :-- |
| `--json`      | JSON으로 출력                            |     |
| `--available` | 마켓플레이스에서 사용 가능한 플러그인 포함. `--json` 필요 |     |
| `-h, --help`  | 명령어 도움말 표시                           |     |

대화형 세션 내에서 `/plugin list`는 동일한 목록을 인라인으로 출력합니다. 대화형 형식은 `--enabled` 또는 `--disabled`를 허용하여 해당 상태의 플러그인만 표시하며, `ls`를 `list`의 약자로 사용합니다.

<h3 id="plugin-details">
  plugin details
</h3>

플러그인의 컴포넌트 인벤토리 및 예상 토큰 비용을 표시합니다. 출력은 플러그인이 기여하는 모든 컴포넌트를 Skills, Agents, Hooks, MCP 서버 및 LSP 서버로 그룹화하여 나열하며, 각 세션에 추가되는 토큰 수의 추정치를 함께 표시합니다. Skills 그룹에는 `skills/` 및 `commands/` 항목이 모두 포함됩니다.

```bash theme={null}
claude plugin details <name>
```

**인수:**

* `<name>`: 플러그인 이름 또는 `plugin-name@marketplace-name`

**옵션:**

| 옵션           | 설명         | 기본값 |
| :----------- | :--------- | :-- |
| `-h, --help` | 명령어 도움말 표시 |     |

출력은 각 컴포넌트에 대해 두 가지 비용 수치를 표시합니다:

* **Always-on:** 컴포넌트가 실행되는지 여부와 관계없이 플러그인의 목록 텍스트(예: 스킬 설명, 에이전트 설명, 명령어 이름)에 의해 모든 세션에 추가되는 토큰입니다.
* **On-invoke:** 컴포넌트가 실행될 때 비용이 드는 토큰입니다. 일반적인 세션에서는 컴포넌트의 일부만 호출되므로 플러그인 전체가 아닌 컴포넌트별로 표시됩니다.

다음 예시는 두 개의 스킬이 있는 플러그인의 출력 모습을 보여줍니다:

```
dependency-guard 1.2.0
  Dependency analysis for Claude Code sessions
  Source: dependency-guard@example-marketplace

Component inventory
  Skills (2)  scan-dependencies, review-changes
  Agents (0)
  Hooks (1)  (harness-only — no model context cost)
  MCP servers (0)
  LSP servers (0)

Projected token cost
  Always-on:   ~180 tok   added to every session

Per-component (rounded)
  component            always-on  on-invoke
  scan-dependencies        ~100      ~2400
  review-changes            ~80      ~1800

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Always-on 합계는 활성 모델에 대한 `count_tokens` API를 통해 계산됩니다. 컴포넌트별 수치는 해당 합계에서 비례적으로 조정됩니다. API에 연결할 수 없으면 명령어는 문자 기반 추정으로 폴백됩니다.

<h3 id="plugin-tag">
  plugin tag
</h3>

현재 디렉토리의 플러그인에 대한 릴리스 git 태그를 생성합니다. 플러그인의 폴더 내에서 실행하세요. [플러그인 릴리스 태그 지정](/docs/ko/plugin-dependencies#tag-plugin-releases-for-version-resolution)을 참조하세요.

```bash theme={null}
claude plugin tag [options]
```

**옵션:**

| 옵션            | 설명                             | 기본값 |
| :------------ | :----------------------------- | :-- |
| `--push`      | 태그를 생성한 후 원격으로 푸시              |     |
| `--dry-run`   | 태그를 생성하지 않고 태그 지정될 내용 출력       |     |
| `-f, --force` | 작업 트리가 더티하거나 태그가 이미 존재해도 태그 생성 |     |
| `-h, --help`  | 명령어 도움말 표시                     |     |

***

<h2 id="debugging-and-development-tools">
  디버깅 및 개발 도구
</h2>

<h3 id="debugging-commands">
  디버깅 명령어
</h3>

`claude --debug`를 사용하여 플러그인 로딩 세부 정보를 확인하세요:

이는 다음을 표시합니다:

* 로드되는 플러그인
* 플러그인 매니페스트의 오류
* Skill, agent 및 hook 등록
* MCP 서버 초기화

<h3 id="common-issues">
  일반적인 문제
</h3>

| 문제                                  | 원인                         | 해결책                                                                                                                                      |
| :---------------------------------- | :------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| 플러그인이 로드되지 않음                       | 잘못된 `plugin.json`          | `claude plugin validate` 또는 `/plugin validate`를 실행하여 `plugin.json`, skill/agent/command frontmatter 및 `hooks/hooks.json`의 구문 및 스키마 오류 확인 |
| Skills가 나타나지 않음                     | 잘못된 디렉토리 구조                | `skills/` 또는 `commands/`가 플러그인 루트에 있는지 확인, `.claude-plugin/` 내부가 아님                                                                      |
| Hooks가 실행되지 않음                      | 스크립트가 실행 가능하지 않음           | `chmod +x script.sh` 실행                                                                                                                  |
| MCP 서버 실패                           | `${CLAUDE_PLUGIN_ROOT}` 누락 | 모든 플러그인 경로에 변수 사용                                                                                                                        |
| 경로 오류                               | 절대 경로 사용됨                  | 모든 경로는 상대적이어야 하며 `./`로 시작해야 함                                                                                                            |
| LSP `Executable not found in $PATH` | 언어 서버가 설치되지 않음             | 바이너리 설치 (예: `npm install -g typescript-language-server typescript`)                                                                      |

<h3 id="example-error-messages">
  예시 오류 메시지
</h3>

**매니페스트 검증 오류**:

* `Invalid JSON syntax: Unexpected token } in JSON at position 142`: 누락된 쉼표, 추가 쉼표 또는 따옴표 없는 문자열 확인
* `Plugin has an invalid manifest file at .claude-plugin/plugin.json. Validation errors: name: Required`: 필수 필드가 누락됨
* `Plugin has a corrupt manifest file at .claude-plugin/plugin.json. JSON parse error: ...`: JSON 구문 오류

**플러그인 로딩 오류**:

* `Warning: No commands found in plugin my-plugin custom directory: ./cmds. Expected .md files or SKILL.md in subdirectories.`: 명령어 경로가 존재하지만 유효한 명령어 파일이 없음
* `Plugin directory not found at path: ./plugins/my-plugin. Check that the marketplace entry has the correct path.`: marketplace.json의 `source` 경로가 존재하지 않는 디렉토리를 가리킴
* `Plugin my-plugin has conflicting manifests: both plugin.json and marketplace entry specify components.`: 중복 컴포넌트 정의 제거 또는 marketplace 항목에서 `strict: false` 제거

<h3 id="hook-troubleshooting">
  Hook 문제 해결
</h3>

**Hook 스크립트가 실행되지 않음**:

1. 스크립트가 실행 가능한지 확인: `chmod +x ./scripts/your-script.sh`
2. shebang 라인 확인: 첫 번째 줄은 `#!/bin/bash` 또는 `#!/usr/bin/env bash`여야 함
3. 경로가 `${CLAUDE_PLUGIN_ROOT}` 사용하는지 확인: `"command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/your-script.sh"`
4. 스크립트를 수동으로 테스트: `./scripts/your-script.sh`

**Hook이 예상 이벤트에서 트리거되지 않음**:

1. 이벤트 이름이 올바른지 확인 (대소문자 구분): `PostToolUse`, `postToolUse` 아님
2. 매처 패턴이 도구와 일치하는지 확인: 파일 작업의 경우 `"matcher": "Write|Edit"`
3. Hook 유형이 유효한지 확인: `command`, `http`, `mcp_tool`, `prompt` 또는 `agent`

<h3 id="mcp-server-troubleshooting">
  MCP 서버 문제 해결
</h3>

**서버가 시작되지 않음**:

1. 명령어가 존재하고 실행 가능한지 확인
2. 모든 경로가 `${CLAUDE_PLUGIN_ROOT}` 변수를 사용하는지 확인
3. MCP 서버 로그 확인: `claude --debug`는 초기화 오류를 표시합니다
4. Claude Code 외부에서 서버를 수동으로 테스트

**서버 도구가 나타나지 않음**:

1. 서버가 `.mcp.json` 또는 `plugin.json`에 올바르게 구성되었는지 확인
2. 서버가 MCP 프로토콜을 올바르게 구현하는지 확인
3. 디버그 출력에서 연결 시간 초과 확인

<h3 id="directory-structure-mistakes">
  디렉토리 구조 실수
</h3>

**증상**: 플러그인이 로드되지만 컴포넌트 (skills, agents, hooks)가 누락됨.

**올바른 구조**: 컴포넌트는 플러그인 루트에 있어야 하며 `.claude-plugin/` 내부가 아닙니다. `plugin.json`만 `.claude-plugin/`에 속합니다.

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json      ← 매니페스트만 여기
├── commands/            ← 루트 수준
├── agents/              ← 루트 수준
└── hooks/               ← 루트 수준
```

컴포넌트가 `.claude-plugin/` 내부에 있으면 플러그인 루트로 이동하세요.

**디버그 체크리스트**:

1. `claude --debug`를 실행하고 "loading plugin" 메시지를 찾으세요
2. 각 컴포넌트 디렉토리가 디버그 출력에 나열되는지 확인
3. 파일 권한이 플러그인 파일 읽기를 허용하는지 확인

***

<h2 id="distribution-and-versioning-reference">
  배포 및 버전 관리 참조
</h2>

<h3 id="version-management">
  버전 관리
</h3>

Claude Code는 플러그인의 버전을 캐시 키로 사용하여 업데이트를 사용할 수 있는지 여부를 결정합니다. `/plugin update`를 실행하거나 자동 업데이트가 실행되면 Claude Code는 현재 버전을 계산하고 이미 설치된 버전과 일치하면 업데이트를 건너뜁니다.

버전은 다음 중 설정된 첫 번째 항목에서 확인됩니다:

1. 플러그인의 `plugin.json`에 있는 `version` 필드
2. `marketplace.json`의 플러그인 마켓플레이스 항목에 있는 `version` 필드
3. git 호스팅 마켓플레이스의 `github`, `url`, `git-subdir` 및 상대 경로 소스에 대한 플러그인 소스의 git 커밋 SHA
4. git 저장소 내에 있지 않은 `npm` 소스 또는 로컬 디렉토리의 경우 `unknown`

이는 플러그인을 버전 관리하는 두 가지 방법을 제공합니다:

| 접근 방식         | 방법                                          | 업데이트 동작                                                                                             | 최적 사용                    |
| :------------ | :------------------------------------------ | :-------------------------------------------------------------------------------------------------- | :----------------------- |
| **명시적 버전**    | `plugin.json`에서 `"version": "2.1.0"`으로 설정   | 사용자는 이 필드를 범프할 때만 업데이트를 받습니다. 이를 범프하지 않고 새 커밋을 푸시하면 효과가 없으며 `/plugin update`는 "이미 최신 버전입니다"를 보고합니다. | 안정적인 릴리스 주기가 있는 게시된 플러그인 |
| **커밋-SHA 버전** | `plugin.json` 및 마켓플레이스 항목 모두에서 `version` 생략 | 사용자는 플러그인의 git 소스에 대한 모든 새 커밋에서 업데이트를 받습니다                                                          | 활발히 개발 중인 내부 또는 팀 플러그인   |

<Warning>
  `plugin.json`에서 `version`을 설정하면 사용자가 변경 사항을 받기를 원할 때마다 이를 범프해야 합니다. 새 커밋을 푸시하는 것만으로는 충분하지 않습니다. Claude Code가 동일한 버전 문자열을 보고 캐시된 사본을 유지하기 때문입니다. 빠르게 반복하는 경우 `version`을 설정하지 않은 상태로 두어 대신 git 커밋 SHA가 사용되도록 하세요.
</Warning>

명시적 버전을 사용하는 경우 [의미 있는 버전 관리](https://semver.org)(`MAJOR.MINOR.PATCH`)를 따르세요: 주요 변경 사항의 경우 MAJOR를 범프하고, 새로운 기능의 경우 MINOR를 범프하고, 버그 수정의 경우 PATCH를 범프하세요. `CHANGELOG.md`에서 변경 사항을 문서화하세요.

***

<h2 id="see-also">
  참고 항목
</h2>

* [플러그인](/docs/ko/plugins) - 튜토리얼 및 실제 사용
* [플러그인 마켓플레이스](/docs/ko/plugin-marketplaces) - 마켓플레이스 생성 및 관리
* [Skills](/docs/ko/skills) - Skill 개발 세부 정보
* [Subagents](/docs/ko/sub-agents) - Agent 구성 및 기능
* [Hooks](/docs/ko/hooks) - 이벤트 처리 및 자동화
* [MCP](/docs/ko/mcp) - 외부 도구 통합
* [설정](/docs/ko/settings) - 플러그인의 구성 옵션
