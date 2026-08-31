> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# hooks를 사용하여 작업 자동화

> Claude Code가 파일을 편집하거나 작업을 완료하거나 입력이 필요할 때 자동으로 셸 명령을 실행합니다. 코드 형식 지정, 알림 전송, 명령 검증 및 프로젝트 규칙 적용합니다.

Hooks는 Claude Code의 라이프사이클의 특정 지점에서 실행되는 사용자 정의 셸 명령입니다. 이들은 Claude Code의 동작에 대한 결정론적 제어를 제공하여 LLM이 실행하도록 선택하는 것에 의존하기보다는 특정 작업이 항상 발생하도록 보장합니다. Hooks를 사용하여 프로젝트 규칙을 적용하고, 반복적인 작업을 자동화하며, Claude Code를 기존 도구와 통합합니다.

판단이 필요한 결정의 경우 결정론적 규칙이 아닌 경우, [프롬프트 기반 hooks](#prompt-based-hooks) 또는 [에이전트 기반 hooks](#agent-based-hooks)를 사용할 수도 있습니다. 이들은 Claude 모델을 사용하여 조건을 평가합니다.

Claude Code를 확장하는 다른 방법은 [skills](/docs/ko/skills)를 참조하여 Claude에 추가 지침과 실행 가능한 명령을 제공하고, [subagents](/docs/ko/sub-agents)를 사용하여 격리된 컨텍스트에서 작업을 실행하며, [plugins](/docs/ko/plugins)를 사용하여 프로젝트 전체에서 공유할 확장을 패키징합니다.

<Tip>
  이 가이드는 일반적인 사용 사례와 시작 방법을 다룹니다. 전체 이벤트 스키마, JSON 입출력 형식 및 비동기 hooks 및 MCP 도구 hooks와 같은 고급 기능은 [Hooks 참조](/docs/ko/hooks)를 참조하세요.
</Tip>

<h2 id="set-up-your-first-hook">
  첫 번째 hook 설정
</h2>

Hook을 만들려면 [설정 파일](#configure-hook-location)에 `hooks` 블록을 추가합니다. 이 연습은 데스크톱 알림 hook을 만들므로 Claude가 터미널을 보는 대신 입력을 기다릴 때마다 알림을 받습니다.

<Steps>
  <Step title="설정에 hook 추가">
    `~/.claude/settings.json`을 열고 `Notification` hook을 추가합니다. 아래 예제는 macOS용 `osascript`를 사용합니다. Linux 및 Windows 명령은 [Claude가 입력이 필요할 때 알림 받기](#get-notified-when-claude-needs-input)를 참조하세요.

    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    설정 파일에 이미 `hooks` 키가 있으면 전체 객체를 바꾸는 대신 `Notification`을 기존 이벤트 키의 형제로 추가합니다. 각 이벤트 이름은 단일 `hooks` 객체 내의 키입니다:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write" }]
          }
        ],
        "Notification": [
          {
            "matcher": "",
            "hooks": [{ "type": "command", "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'" }]
          }
        ]
      }
    }
    ```

    CLI에서 원하는 것을 설명하여 Claude에게 hook을 작성하도록 요청할 수도 있습니다.
  </Step>

  <Step title="구성 확인">
    `/hooks`를 입력하여 hooks 브라우저를 엽니다. 구성된 hooks가 있는 각 이벤트 옆에 개수가 있는 사용 가능한 모든 hook 이벤트 목록이 표시됩니다. `Notification`을 선택하여 새 hook이 목록에 나타나는지 확인합니다. Hook을 선택하면 세부 정보가 표시됩니다: 이벤트, matcher, 유형, 소스 파일 및 명령.
  </Step>

  <Step title="hook 테스트">
    `Esc`를 눌러 CLI로 돌아갑니다. Claude에게 권한이 필요한 작업을 수행하도록 요청한 다음 터미널에서 전환합니다. 데스크톱 알림을 받아야 합니다.
  </Step>
</Steps>

<Tip>
  `/hooks` 메뉴는 읽기 전용입니다. Hooks를 추가, 수정 또는 제거하려면 설정 JSON을 직접 편집하거나 Claude에게 변경을 요청합니다.
</Tip>

<h2 id="what-you-can-automate">
  자동화할 수 있는 것
</h2>

Hooks를 사용하면 Claude Code의 라이프사이클의 주요 지점에서 코드를 실행할 수 있습니다: 편집 후 파일 형식 지정, 실행 전 명령 차단, Claude가 입력이 필요할 때 알림 전송, 세션 시작 시 컨텍스트 주입 등. 전체 hook 이벤트 목록은 [Hooks 참조](/docs/ko/hooks#hook-lifecycle)를 참조하세요.

각 예제에는 [설정 파일](#configure-hook-location)에 추가하는 즉시 사용 가능한 구성 블록이 포함되어 있습니다.

별도의 모델 검토를 실행하고 결과를 세션에 다시 피드백하는 hooks의 프로덕션 예제는 [`security-guidance` 플러그인이 Claude Code와 통합되는 방식](/docs/ko/security-guidance#how-the-plugin-integrates-with-claude-code)을 참조하세요.

<h3 id="get-notified-when-claude-needs-input">
  Claude가 입력이 필요할 때 알림 받기
</h3>

Claude가 작업을 완료하고 입력이 필요할 때마다 데스크톱 알림을 받으므로 터미널을 확인하지 않고 다른 작업으로 전환할 수 있습니다.

이 hook은 Claude가 입력 또는 권한을 기다릴 때 발생하는 `Notification` 이벤트를 사용합니다. 각 탭은 플랫폼의 기본 알림 명령을 사용합니다. `~/.claude/settings.json`에 추가합니다:

<Tabs>
  <Tab title="macOS">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="알림이 나타나지 않으면">
      `osascript`는 기본 제공 Script Editor 앱을 통해 알림을 라우팅합니다. Script Editor에 알림 권한이 없으면 명령이 자동으로 실패하고 macOS는 권한을 부여하도록 프롬프트하지 않습니다. Terminal에서 이를 한 번 실행하여 Script Editor가 알림 설정에 나타나도록 합니다:

      ```bash theme={null}
      osascript -e 'display notification "test"'
      ```

      아직 아무것도 나타나지 않습니다. **시스템 설정 > 알림**을 열고 목록에서 **Script Editor**를 찾은 다음 **알림 허용**을 켭니다. 명령을 다시 실행하여 테스트 알림이 나타나는지 확인합니다.
    </Accordion>
  </Tab>

  <Tab title="Linux">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "notify-send 'Claude Code' 'Claude Code needs your attention'"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe -Command \"[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); [System.Windows.Forms.MessageBox]::Show('Claude Code needs your attention', 'Claude Code')\""
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

빈 `matcher`는 모든 알림 유형에서 발생합니다. 특정 이벤트에서만 발생하도록 하려면 다음 값 중 하나로 설정합니다:

| Matcher                | 발생 시점                                                                  |
| :--------------------- | :--------------------------------------------------------------------- |
| `permission_prompt`    | Claude가 도구 사용을 승인하도록 요청할 때                                             |
| `idle_prompt`          | Claude가 완료되고 다음 프롬프트를 기다릴 때                                            |
| `auth_success`         | 인증이 완료될 때                                                              |
| `elicitation_dialog`   | MCP 서버가 유도 양식을 열 때                                                     |
| `elicitation_complete` | MCP 유도 양식이 제출되거나 닫힐 때                                                  |
| `elicitation_response` | MCP 유도 응답이 서버로 다시 전송될 때                                                |
| `agent_needs_input`    | 백그라운드 세션이 입력을 기다리기 시작합니다. [agent view](/docs/ko/agent-view)가 열려 있을 때만 발생합니다 |
| `agent_completed`      | 백그라운드 세션이 완료되거나 실패합니다. [agent view](/docs/ko/agent-view)가 열려 있을 때만 발생합니다    |

`agent_needs_input` 및 `agent_completed` matcher는 Claude Code v2.1.198 이상이 필요합니다.

`/hooks`를 입력하고 `Notification`을 선택하여 hook이 등록되었는지 확인합니다. 전체 이벤트 스키마는 [Notification 참조](/docs/ko/hooks#notification)를 참조하세요.

<h3 id="auto-format-code-after-edits">
  편집 후 코드 자동 형식 지정
</h3>

Claude가 편집하는 모든 파일에서 [Prettier](https://prettier.io/)를 자동으로 실행하여 수동 개입 없이 형식이 일관되게 유지되도록 합니다.

이 hook은 `PostToolUse` 이벤트를 `Edit|Write` matcher와 함께 사용하므로 파일 편집 도구 후에만 실행됩니다. 명령은 [`jq`](https://jqlang.github.io/jq/)를 사용하여 편집된 파일 경로를 추출하고 Prettier에 전달합니다. 프로젝트 루트의 `.claude/settings.json`에 추가합니다:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

Claude Code v2.1.191 이상에서는 matcher를 `Edit,Write`로도 작성할 수 있습니다. 이러한 버전에서는 `|`와 `,`이 도구 이름 matcher의 상호 교환 가능한 목록 구분자이기 때문입니다.

<Note>
  이 페이지의 Bash 예제는 JSON 구문 분석을 위해 `jq`를 사용합니다. macOS에서 `brew install jq`로, Debian 및 Ubuntu에서 `apt-get install jq`로 설치하거나 [`jq` 다운로드](https://jqlang.github.io/jq/download/)를 참조하세요.
</Note>

<h3 id="block-edits-to-protected-files">
  보호된 파일에 대한 편집 차단
</h3>

Claude가 `.env`, `package-lock.json` 또는 `.git/`의 모든 항목과 같은 민감한 파일을 수정하지 못하도록 방지합니다. Claude는 편집이 차단된 이유를 설명하는 피드백을 받으므로 접근 방식을 조정할 수 있습니다.

이 예제는 hook이 호출하는 별도의 스크립트 파일을 사용합니다. 스크립트는 대상 파일 경로를 보호된 패턴 목록과 비교하고 종료 코드 2로 종료하여 편집을 차단합니다.

<Steps>
  <Step title="hook 스크립트 만들기">
    이를 `.claude/hooks/protect-files.sh`에 저장합니다:

    ```bash theme={null}
    #!/bin/bash
    # protect-files.sh

    INPUT=$(cat)
    FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

    PROTECTED_PATTERNS=(".env" "package-lock.json" ".git/")

    for pattern in "${PROTECTED_PATTERNS[@]}"; do
      if [[ "$FILE_PATH" == *"$pattern"* ]]; then
        echo "Blocked: $FILE_PATH matches protected pattern '$pattern'" >&2
        exit 2
      fi
    done

    exit 0
    ```
  </Step>

  <Step title="macOS 및 Linux에서 스크립트를 실행 가능하게 만들기">
    Hook 스크립트는 Claude Code가 실행하려면 실행 가능해야 합니다:

    ```bash theme={null}
    chmod +x .claude/hooks/protect-files.sh
    ```
  </Step>

  <Step title="hook 등록">
    모든 `Edit` 또는 `Write` 도구 호출 전에 스크립트를 실행하는 `PreToolUse` hook을 `.claude/settings.json`에 추가합니다:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              {
                "type": "command",
                "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  압축 후 컨텍스트 다시 주입
</h3>

Claude의 컨텍스트 윈도우가 가득 차면 압축은 대화를 요약하여 공간을 확보합니다. 이는 중요한 세부 정보를 잃을 수 있습니다. `compact` matcher와 함께 `SessionStart` hook을 사용하여 모든 압축 후 중요한 컨텍스트를 다시 주입합니다.

명령이 stdout에 쓰는 모든 텍스트는 Claude의 컨텍스트에 추가됩니다. 이 예제는 Claude에게 프로젝트 규칙과 최근 작업을 상기시킵니다. 프로젝트 루트의 `.claude/settings.json`에 추가합니다:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: use Bun, not npm. Run bun test before committing. Current sprint: auth refactor.'"
          }
        ]
      }
    ]
  }
}
```

`echo`를 `git log --oneline -5`와 같이 동적 출력을 생성하는 모든 명령으로 바꿀 수 있습니다. 모든 세션 시작 시 컨텍스트를 주입하려면 [CLAUDE.md](/docs/ko/memory) 사용을 고려하세요. 환경 변수는 [`CLAUDE_ENV_FILE`](/docs/ko/hooks#persist-environment-variables)을 참조하세요.

<h3 id="audit-configuration-changes">
  구성 변경 감사
</h3>

세션 중에 설정 또는 skills 파일이 변경될 때를 추적합니다. `ConfigChange` 이벤트는 외부 프로세스 또는 편집기가 구성 파일을 수정할 때 발생하므로 규정 준수를 위해 변경 사항을 기록하거나 무단 수정을 차단할 수 있습니다.

이 예제는 각 변경을 감사 로그에 추가합니다. `~/.claude/settings.json`에 추가합니다:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "jq -c '{timestamp: now | todate, source: .source, file: .file_path}' >> ~/claude-config-audit.log"
          }
        ]
      }
    ]
  }
}
```

Matcher는 구성 유형으로 필터링합니다: `user_settings`, `project_settings`, `local_settings`, `policy_settings` 또는 `skills`. 변경이 적용되지 않도록 차단하려면 종료 코드 2로 종료하거나 `{"decision": "block"}`을 반환합니다. 전체 입력 스키마는 [ConfigChange 참조](/docs/ko/hooks#configchange)를 참조하세요.

<h3 id="reload-environment-when-directory-or-files-change">
  디렉토리 또는 파일이 변경될 때 환경 다시 로드
</h3>

일부 프로젝트는 어느 디렉토리에 있는지에 따라 다른 환경 변수를 설정합니다. [direnv](https://direnv.net/)와 같은 도구는 셸에서 자동으로 이를 수행하지만 Claude의 Bash 도구는 자체적으로 해당 변경 사항을 선택하지 않습니다.

`SessionStart` hook을 `CwdChanged` hook과 쌍으로 사용하면 이를 해결합니다. `SessionStart`는 시작한 디렉토리에 대한 변수를 로드하고 `CwdChanged`는 Claude가 디렉토리를 변경할 때마다 이를 다시 로드합니다. 둘 다 `CLAUDE_ENV_FILE`에 쓰며, Claude Code는 각 Bash 명령 전에 이를 스크립트 프리앰블로 실행합니다. `~/.claude/settings.json`에 추가합니다:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ],
    "CwdChanged": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

`direnv allow`를 `.envrc`가 있는 각 디렉토리에서 한 번 실행하여 direnv가 이를 로드할 수 있도록 허용합니다. direnv 대신 devbox 또는 nix를 사용하는 경우 `direnv export bash` 대신 `devbox shellenv` 또는 `devbox global shellenv`를 사용하면 동일한 패턴이 작동합니다.

모든 디렉토리 변경이 아닌 특정 파일에 반응하려면 `FileChanged`를 `matcher`와 함께 사용하여 감시할 파일 이름을 나열합니다 (파이프로 구분). 감시 목록을 구성하기 위해 이 값은 정규식으로 평가되지 않고 리터럴 파일 이름으로 분할됩니다. [FileChanged](/docs/ko/hooks#filechanged)를 참조하여 파일이 변경될 때 어떤 hook 그룹이 실행되는지 필터링하는 방법도 확인하세요. 이 예제는 작업 디렉토리에서 `.envrc` 및 `.env`를 감시합니다:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": ".envrc|.env",
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

입력 스키마, `watchPaths` 출력 및 `CLAUDE_ENV_FILE` 세부 정보는 [CwdChanged](/docs/ko/hooks#cwdchanged) 및 [FileChanged](/docs/ko/hooks#filechanged) 참조 항목을 참조하세요.

<h3 id="auto-approve-specific-permission-prompts">
  특정 권한 프롬프트 자동 승인
</h3>

항상 허용하는 도구 호출에 대한 승인 대화를 건너뜁니다. 이 예제는 `ExitPlanMode`를 자동 승인합니다. 이는 Claude가 계획 제시를 완료하고 진행을 요청할 때 호출하는 도구이므로 계획이 준비될 때마다 프롬프트가 표시되지 않습니다.

위의 종료 코드 예제와 달리 자동 승인을 위해서는 hook이 JSON 결정을 stdout에 작성해야 합니다. `PermissionRequest` hook은 Claude Code가 권한 대화를 표시하려고 할 때 발생하며 `"behavior": "allow"`를 반환하면 사용자 대신 답변합니다.

Matcher는 hook을 `ExitPlanMode`로만 범위를 지정하므로 다른 프롬프트는 영향을 받지 않습니다. `~/.claude/settings.json`에 추가합니다:

```json theme={null}
{
  "hooks": {
    "PermissionRequest": [
      {
        "matcher": "ExitPlanMode",
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\": {\"hookEventName\": \"PermissionRequest\", \"decision\": {\"behavior\": \"allow\"}}}'"
          }
        ]
      }
    ]
  }
}
```

Hook이 승인하면 Claude Code는 계획 모드를 종료하고 계획 모드에 들어가기 전에 활성화되었던 권한 모드를 복원합니다. 트랜스크립트는 대화가 나타났을 위치에 "Allowed by PermissionRequest hook"을 표시합니다. Hook 경로는 항상 현재 대화를 유지합니다: 대화가 할 수 있는 방식으로 컨텍스트를 지우고 새로운 구현 세션을 시작할 수 없습니다.

대신 특정 권한 모드를 설정하려면 hook의 출력에 `setMode` 항목이 있는 `updatedPermissions` 배열이 포함될 수 있습니다. `mode` 값은 `default`, `acceptEdits` 또는 `bypassPermissions`과 같은 모든 권한 모드이며 `destination: "session"`은 현재 세션에만 적용합니다.

<Note>
  `bypassPermissions`은 세션이 이미 bypass 모드로 시작된 경우에만 적용됩니다: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` 또는 설정의 `permissions.defaultMode: "bypassPermissions"`이며 [`permissions.disableBypassPermissionsMode`](/docs/ko/permissions#managed-settings)로 비활성화되지 않습니다. 이는 `defaultMode`로 절대 지속되지 않습니다.
</Note>

세션을 `acceptEdits`로 전환하려면 hook이 이 JSON을 stdout에 작성합니다:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedPermissions": [
        { "type": "setMode", "mode": "acceptEdits", "destination": "session" }
      ]
    }
  }
}
```

Matcher를 가능한 한 좁게 유지합니다. `.*`와 일치하거나 matcher를 비워두면 파일 쓰기 및 셸 명령을 포함한 모든 권한 프롬프트를 자동 승인합니다. 전체 결정 필드 집합은 [PermissionRequest 참조](/docs/ko/hooks#permissionrequest-decision-control)를 참조하세요.

<h2 id="how-hooks-work">
  Hooks 작동 방식
</h2>

Hook 이벤트는 Claude Code의 라이프사이클의 특정 지점에서 발생합니다. 이벤트가 발생하면 일치하는 모든 hooks가 병렬로 실행되고 동일한 hook 명령은 자동으로 중복 제거됩니다. 아래 표는 각 이벤트와 발생 시기를 보여줍니다:

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

각 hook에는 실행 방식을 결정하는 `type`이 있습니다. 대부분의 hooks는 `"type": "command"`를 사용하여 셸 명령을 실행합니다. 네 가지 다른 유형을 사용할 수 있습니다:

* `"type": "http"`: 이벤트 데이터를 URL에 POST합니다. [HTTP hooks](#http-hooks)를 참조하세요.
* `"type": "mcp_tool"`: 이미 연결된 MCP 서버에서 도구를 호출합니다. [MCP tool hooks](/docs/ko/hooks#mcp-tool-hook-fields)를 참조하세요.
* `"type": "prompt"`: 단일 턴 LLM 평가입니다. [프롬프트 기반 hooks](#prompt-based-hooks)를 참조하세요.
* `"type": "agent"`: 도구 액세스를 통한 다중 턴 검증입니다. 에이전트 hooks는 실험적이며 변경될 수 있습니다. [에이전트 기반 hooks](#agent-based-hooks)를 참조하세요.

<h3 id="combine-results-from-multiple-hooks">
  여러 hooks의 결과 결합
</h3>

여러 hooks가 동일한 이벤트와 일치하면 모든 hook의 명령이 완료될 때까지 실행된 후 Claude Code가 결과를 병합합니다. 하나의 hook이 `deny`를 반환하는 것이 형제 hooks의 실행을 중지하지 않습니다. 한 hook의 `deny`가 다른 hook의 부작용을 억제하는 것에 의존하지 마세요.

모든 일치하는 hooks가 완료된 후 Claude Code는 출력을 결합합니다. `PreToolUse` 권한 결정의 경우 가장 제한적인 답변이 우선합니다: `deny`, `defer`, `ask`, `allow` 순서입니다. `additionalContext`의 텍스트는 모든 hook에서 유지되고 Claude와 함께 전달됩니다.

아래 예제는 `Bash`에 두 개의 `PreToolUse` hooks를 등록합니다. 첫 번째는 모든 명령을 로그 파일에 추가하고 0으로 종료합니다. 두 번째는 명령에 `rm -rf`가 포함되어 있을 때 거부하기 위해 2로 종료하는 스크립트를 실행합니다:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r .tool_input.command >> ~/.claude/bash.log"
          },
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-rm-rf.sh"
          }
        ]
      }
    ]
  }
}
```

Claude가 `rm -rf /tmp/build`를 실행하려고 할 때 두 hooks 모두 병렬로 실행됩니다. 로깅 hook은 명령을 `~/.claude/bash.log`에 쓰고 0으로 종료하여 결정이 없음을 보고합니다. 보안 규칙 hook은 2로 종료하여 도구 호출을 거부합니다. 거부가 우선하므로 Claude Code는 명령을 차단하고 Claude에게 보안 규칙의 stderr를 표시합니다. 로깅 hook이 이미 실행되었기 때문에 로그 항목은 여전히 기록됩니다.

<h3 id="read-input-and-return-output">
  입력 읽기 및 출력 반환
</h3>

Hooks는 stdin, stdout, stderr 및 종료 코드를 통해 Claude Code와 통신합니다. 이벤트가 발생하면 Claude Code는 이벤트별 데이터를 JSON으로 스크립트의 stdin에 전달합니다. 스크립트는 해당 데이터를 읽고 작업을 수행한 다음 종료 코드를 통해 Claude Code에 다음 작업을 알립니다.

<h4 id="hook-input">
  Hook 입력
</h4>

모든 이벤트에는 `session_id` 및 `cwd`와 같은 공통 필드가 포함되지만 각 이벤트 유형은 다른 데이터를 추가합니다. 예를 들어 Claude가 Bash 명령을 실행할 때 `PreToolUse` hook은 stdin에서 다음과 같은 것을 받습니다:

```json theme={null}
{
  "session_id": "abc123",          // 이 세션의 고유 ID
  "cwd": "/Users/sarah/myproject", // 이벤트가 발생했을 때의 작업 디렉토리
  "hook_event_name": "PreToolUse", // 이 hook을 트리거한 이벤트
  "tool_name": "Bash",             // Claude가 사용하려는 도구
  "tool_input": {                  // Claude가 도구에 전달한 인수
    "command": "npm test"          // Bash의 경우 이것은 셸 명령입니다
  }
}
```

스크립트는 해당 JSON을 구문 분석하고 해당 필드에 대해 작동할 수 있습니다. `UserPromptSubmit` hooks는 `prompt` 텍스트를 대신 받고, `SessionStart` hooks는 `source` (startup, resume, clear, compact)를 받으며, 등등입니다. 공유 필드는 참조의 [공통 입력 필드](/docs/ko/hooks#common-input-fields)를 참조하고 각 이벤트별 섹션에서 이벤트별 스키마를 참조하세요.

<h4 id="hook-output">
  Hook 출력
</h4>

스크립트는 stdout 또는 stderr에 쓰고 특정 코드로 종료하여 Claude Code에 다음 작업을 알립니다. 예를 들어 명령을 차단하려는 `PreToolUse` hook:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  # stderr는 Claude의 피드백이 됩니다
  exit 2 # exit 2 = 작업 차단
fi

exit 0  # exit 0 = 진행 허용
```

종료 코드는 다음에 일어날 일을 결정합니다:

* **Exit 0**: hook은 이의를 제기하지 않으며 작업이 정상적으로 진행됩니다. `PreToolUse` hook의 경우 이것은 도구 호출을 승인하지 않습니다: 정상적인 [권한 흐름](/docs/ko/permissions)이 여전히 적용됩니다. `UserPromptSubmit`, `UserPromptExpansion` 및 `SessionStart` hooks의 경우 stdout에 쓰는 모든 것이 Claude의 컨텍스트에 추가됩니다.
* **Exit 2**: 작업이 차단됩니다. stderr에 이유를 쓰면 Claude가 피드백으로 받아 조정할 수 있습니다. 일부 이벤트는 차단될 수 없습니다: `SessionStart`, `Setup`, `Notification` 및 기타의 경우 exit 2는 stderr를 사용자에게 표시하고 실행이 계속됩니다. 이벤트별 전체 목록은 [이벤트별 exit 코드 2 동작](/docs/ko/hooks#exit-code-2-behavior-per-event)을 참조하세요.
* **다른 종료 코드**: 작업이 진행됩니다. 트랜스크립트는 `<hook name> hook error` 공지를 표시한 후 stderr의 첫 번째 줄을 표시합니다. 전체 stderr는 [디버그 로그](/docs/ko/hooks#debug-hooks)로 이동합니다.

<h4 id="structured-json-output">
  구조화된 JSON 출력
</h4>

종료 코드는 차단하거나 침묵하는 것만 허용합니다. 더 많은 제어를 위해 exit 0을 하고 stdout에 JSON 객체를 인쇄합니다.

<Note>
  Exit 2를 사용하여 stderr 메시지로 차단하거나 exit 0을 사용하여 구조화된 제어를 위해 JSON을 사용합니다. 혼합하지 마세요: Claude Code는 exit 2일 때 JSON을 무시합니다.
</Note>

예를 들어 `PreToolUse` hook은 도구 호출을 거부하고 이유를 알리거나 사용자 승인을 위해 에스컬레이션할 수 있습니다:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Use rg instead of grep for better performance"
  }
}
```

`"deny"`를 사용하면 Claude Code는 도구 호출을 취소하고 `permissionDecisionReason`을 Claude에게 피드백으로 전달합니다. 이 `permissionDecision` 값은 `PreToolUse`에만 해당합니다:

* `"allow"`: 대화형 권한 프롬프트를 건너뜁니다. 거부 및 요청 규칙을 포함한 엔터프라이즈 관리형 거부 목록은 여전히 적용됩니다. 조직이 `ask`로 설정한 [커넥터 도구](/docs/ko/mcp#organization-controls-on-connector-tools) 및 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구에 대한 프롬프트도 마찬가지입니다.
* `"deny"`: 도구 호출을 취소하고 이유를 Claude에 전송합니다
* `"ask"`: 일반적으로 사용자에게 권한 프롬프트를 표시합니다

네 번째 값인 `"defer"`는 `-p` 플래그가 있는 [비대화형 모드](/docs/ko/headless)에서 사용 가능합니다. 도구 호출을 보존하여 프로세스를 종료하므로 Agent SDK 래퍼가 입력을 수집하고 재개할 수 있습니다. 참조의 [나중에 도구 호출 연기](/docs/ko/hooks#defer-a-tool-call-for-later)를 참조하세요.

`"allow"`를 반환하면 대화형 프롬프트를 건너뜁니다. 하지만 [권한 규칙](/docs/ko/permissions#manage-permissions)을 재정의하지 않습니다. 거부 규칙이 도구 호출과 일치하면 hook이 `"allow"`를 반환하더라도 호출이 차단됩니다. 요청 규칙이 일치하면 사용자가 여전히 프롬프트됩니다. 조직이 `ask`로 설정한 [커넥터 도구](/docs/ko/mcp#organization-controls-on-connector-tools) 및 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구도 마찬가지입니다. 이는 [관리형 설정](/docs/ko/settings#settings-files)을 포함한 모든 설정 범위의 거부 규칙이 hook 승인보다 항상 우선한다는 의미입니다.

다른 이벤트는 다른 결정 패턴을 사용합니다. 예를 들어 `PostToolUse` 및 `Stop` hooks는 최상위 `decision: "block"` 필드를 사용하고 `PermissionRequest`는 `hookSpecificOutput.decision.behavior`를 사용합니다. 이벤트별 전체 분석은 참조의 [요약 표](/docs/ko/hooks#decision-control)를 참조하세요.

`UserPromptSubmit` hooks의 경우 `hookSpecificOutput.additionalContext`를 대신 사용하여 Claude의 컨텍스트에 텍스트를 주입합니다. `additionalContext`를 `hookSpecificOutput` 내에 중첩하세요. JSON의 최상위 수준에 배치하면 Claude Code는 이를 자동으로 무시합니다. 예를 들어 이 출력은 모든 프롬프트에 현재 브랜치 상태를 추가합니다:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "Current branch: release-42. Deploy freeze until Friday."
  }
}
```

프롬프트 차단 및 세션 제목 설정을 포함한 전체 출력 형태는 [UserPromptSubmit 결정 제어](/docs/ko/hooks#userpromptsubmit-decision-control)를 참조하세요.

`type: "prompt"`를 사용하는 Hooks는 출력을 다르게 처리합니다: [프롬프트 기반 hooks](#prompt-based-hooks)를 참조하세요.

<h3 id="filter-hooks-with-matchers">
  Matchers로 hooks 필터링
</h3>

Matcher가 없으면 hook은 이벤트의 모든 발생에서 발생합니다. Matchers를 사용하면 범위를 좁힐 수 있습니다. 예를 들어 모든 도구 호출 후가 아닌 파일 편집 후에만 포매터를 실행하려면 `PostToolUse` hook에 matcher를 추가합니다:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "prettier --write ..." }
        ]
      }
    ]
  }
}
```

`"Edit|Write"` matcher는 Claude가 `Edit` 또는 `Write` 도구를 사용할 때만 발생하고 `Bash`, `Read` 또는 다른 도구를 사용할 때는 발생하지 않습니다. Claude Code v2.1.191 이상에서는 쉼표도 같은 방식으로 대안을 구분하므로 `"Edit, Write"`는 동등합니다. [Matcher 패턴](/docs/ko/hooks#matcher-patterns)을 참조하여 일반 이름과 정규식이 평가되는 방식을 확인하세요.

<Note>
  Claude는 또한 `Bash` 도구를 통해 셸 명령을 실행하여 파일을 생성하거나 수정할 수 있습니다. Hook이 규정 준수 스캔 또는 감사 로깅과 같이 모든 파일 변경을 확인해야 하는 경우 턴당 한 번 작업 트리를 스캔하는 [`Stop`](/docs/ko/hooks#stop) hook을 추가합니다. 호출당 범위를 대신 원하면 `Bash`도 일치시키고 스크립트가 `git status --porcelain`으로 수정되고 추적되지 않은 파일을 나열하도록 합니다.
</Note>

각 이벤트 유형은 특정 필드에서 일치합니다:

| 이벤트                                                                                                                                                             | Matcher가 필터링하는 것                                        | 예제 matcher 값                                                                                                                                                                        |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                                      | 도구 이름                                                   | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                                  | 세션이 시작된 방식                                              | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                                         | 어떤 CLI 플래그가 설정을 트리거했는지                                  | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                                    | 세션이 종료된 이유                                              | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                                  | 알림 유형                                                   | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                                 | 에이전트 유형                                                 | `general-purpose`, `Explore`, `Plan` 또는 사용자 정의 에이전트 이름                                                                                                                              |
| `PreCompact`, `PostCompact`                                                                                                                                     | 압축을 트리거한 것                                              | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                                  | 에이전트 유형                                                 | `SubagentStart`와 동일한 값                                                                                                                                                              |
| `ConfigChange`                                                                                                                                                  | 구성 소스                                                   | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `StopFailure`                                                                                                                                                   | 오류 유형                                                   | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                                            | 로드 이유                                                   | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `Elicitation`                                                                                                                                                   | MCP 서버 이름                                               | 구성된 MCP 서버 이름                                                                                                                                                                       |
| `ElicitationResult`                                                                                                                                             | MCP 서버 이름                                               | `Elicitation`과 동일한 값                                                                                                                                                                |
| `FileChanged`                                                                                                                                                   | 감시할 리터럴 파일 이름 ([FileChanged](/docs/ko/hooks#filechanged) 참조) | `.envrc\|.env`                                                                                                                                                                      |
| `UserPromptExpansion`                                                                                                                                           | 명령 이름                                                   | skill 또는 명령 이름                                                                                                                                                                      |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `MessageDisplay` | matcher 지원 없음                                           | 모든 발생에서 항상 발생                                                                                                                                                                       |

다양한 이벤트 유형에서 matchers를 보여주는 몇 가지 추가 예제:

<Tabs>
  <Tab title="모든 Bash 명령 기록">
    `Bash` 도구 호출만 일치시키고 각 명령을 파일에 기록합니다. `PostToolUse` 이벤트는 명령이 완료된 후 발생하므로 `tool_input.command`는 실행된 내용을 포함합니다. Hook은 stdin에서 이벤트 데이터를 JSON으로 받고 `jq -r '.tool_input.command'`는 명령 문자열만 추출하며 `>>`는 로그 파일에 추가합니다:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "jq -r '.tool_input.command' >> ~/.claude/command-log.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="MCP 도구 일치">
    MCP 도구는 기본 제공 도구와 다른 명명 규칙을 사용합니다: `mcp__<server>__<tool>`. 여기서 `<server>`는 MCP 서버 이름이고 `<tool>`은 제공하는 도구입니다. 예를 들어 `mcp__github__search_repositories` 또는 `mcp__filesystem__read_file`. [플러그인 번들 서버](/docs/ko/mcp#plugin-provided-mcp-servers)의 도구는 대신 `mcp__plugin_my-plugin_db__query`와 같은 범위가 지정된 서버 세그먼트를 사용합니다. 정규식 matcher를 사용하여 특정 서버의 모든 도구를 대상으로 하거나 `mcp__.*__write.*`와 같은 패턴으로 서버 전체에서 일치합니다. 참조의 [MCP 도구 일치](/docs/ko/hooks#match-mcp-tools)를 참조하여 전체 예제 목록을 확인하세요.

    아래 명령은 hook의 JSON 입력에서 `jq`를 사용하여 도구 이름을 추출하고 stderr에 씁니다. stderr는 stdout을 깨끗하게 유지하고 메시지를 [디버그 로그](/docs/ko/hooks#debug-hooks)로 보냅니다:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "mcp__github__.*",
            "hooks": [
              {
                "type": "command",
                "command": "echo \"GitHub tool called: $(jq -r '.tool_name')\" >&2"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="세션 종료 시 정리">
    `SessionEnd` 이벤트는 세션이 종료된 이유에 대한 matchers를 지원합니다. 이 hook은 일반 종료가 아닌 `/clear`를 실행할 때만 발생합니다:

    ```json theme={null}
    {
      "hooks": {
        "SessionEnd": [
          {
            "matcher": "clear",
            "hooks": [
              {
                "type": "command",
                "command": "rm -f /tmp/claude-scratch-*.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

전체 matcher 구문은 [Hooks 참조](/docs/ko/hooks#configuration)를 참조하세요.

<h4 id="filter-by-tool-name-and-arguments-with-the-if-field">
  도구 이름 및 인수로 `if` 필드를 사용하여 필터링
</h4>

`if` 필드는 [권한 규칙 구문](/docs/ko/permissions)을 사용하여 도구 이름과 인수를 함께 사용하여 hooks를 필터링하므로 hook 프로세스는 도구 호출이 일치할 때만 생성됩니다. 이는 도구 이름만으로 그룹 수준에서 필터링하는 `matcher`를 초과합니다.

예를 들어 모든 Bash 명령이 아닌 `git` 명령을 사용할 때만 hook을 실행하려면:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git *)",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-git-policy.sh"
          }
        ]
      }
    ]
  }
}
```

Hook 명령이 실행되는지 여부는 `if` 패턴의 형태와 Claude가 호출하는 Bash 명령에 따라 달라집니다:

| `if` 패턴            | Bash 명령                | Hook이 실행되나요? | 이유                                                          |
| :----------------- | :--------------------- | :----------- | :---------------------------------------------------------- |
| `Bash(git *)`      | `git push`             | 예            | 명령 이름이 일치합니다                                                |
| `Bash(git *)`      | `npm test && git push` | 예            | 각 서브명령이 확인됩니다; `git push`가 일치합니다                            |
| `Bash(git *)`      | `echo $(git log)`      | 예            | `$()` 및 백틱 내의 명령이 확인됩니다; `git log`가 일치합니다                   |
| `Bash(git *)`      | `echo $(date)`         | 아니오          | 서브명령이 `git *`과 일치하지 않습니다                                    |
| `Bash(git push *)` | `echo $(date)`         | 예            | 명령 이름보다 더 많이 지정하는 패턴은 `$()`, 백틱 또는 `$VAR`에서 어쨌든 hook을 실행합니다 |

필터는 또한 실패 시 열려 있으므로 Bash 명령을 구문 분석할 수 없을 때 패턴에 관계없이 hook을 실행합니다. 필터는 최선의 노력이므로 하드 허용 또는 거부를 적용하려면 hook 대신 [권한 시스템](/docs/ko/permissions)을 사용합니다.

`if` 필드는 권한 규칙과 동일한 패턴을 허용합니다: `"Bash(git *)"`, `"Edit(*.ts)"` 등. 여러 도구 이름을 일치시키려면 각각 자신의 `if` 값을 가진 별도의 핸들러를 사용하거나 파이프 교대가 지원되는 `matcher` 수준에서 일치합니다.

`if`는 도구 이벤트에서만 작동합니다: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` 및 `PermissionDenied`. 다른 이벤트에 추가하면 hook이 실행되지 않습니다.

<h3 id="configure-hook-location">
  Hook 위치 구성
</h3>

Hook을 추가하는 위치는 범위를 결정합니다:

| 위치                                                         | 범위                         | 공유 가능           |
| :--------------------------------------------------------- | :------------------------- | :-------------- |
| `~/.claude/settings.json`                                  | 모든 프로젝트                    | 아니오, 컴퓨터에 로컬    |
| `.claude/settings.json`                                    | 단일 프로젝트                    | 예, 리포지토리에 커밋 가능 |
| `.claude/settings.local.json`                              | 단일 프로젝트                    | 아니오, gitignored |
| 관리형 정책 설정                                                  | 조직 전체                      | 예, 관리자 제어       |
| [Plugin](/docs/ko/plugins) `hooks/hooks.json`                   | 플러그인이 활성화되었을 때             | 예, 플러그인과 함께 번들됨 |
| [Skill](/docs/ko/skills) 또는 [agent](/docs/ko/sub-agents) frontmatter | Skill 또는 에이전트가 활성화되어 있는 동안 | 예, 컴포넌트 파일에 정의됨 |

Claude Code에서 [`/hooks`](/docs/ko/hooks#the-%2Fhooks-menu)를 실행하여 이벤트별로 그룹화된 모든 구성된 hooks를 찾아봅니다.

모든 hooks를 비활성화하려면 설정 파일에서 `"disableAllHooks": true`를 설정합니다. 관리형 설정에서 구성된 Hooks는 `disableAllHooks`도 설정되지 않는 한 실행됩니다.

Claude Code가 실행 중인 동안 설정 파일을 직접 편집하면 파일 감시자가 일반적으로 hook 변경을 자동으로 선택합니다.

<h2 id="prompt-based-hooks">
  프롬프트 기반 hooks
</h2>

결정론적 규칙이 아닌 판단이 필요한 결정의 경우 `type: "prompt"` hooks를 사용합니다. 셸 명령을 실행하는 대신 Claude Code는 프롬프트와 hook의 입력 데이터를 Claude 모델(기본적으로 Haiku)에 전송하여 결정을 내립니다. 더 많은 기능이 필요한 경우 `model` 필드로 다른 모델을 지정할 수 있습니다.

모델의 유일한 작업은 yes/no 결정을 JSON으로 반환하는 것입니다:

* `"ok": true`: 작업이 진행됩니다
* `"ok": false`: 작업이 차단됩니다. 이벤트에 따라 다음과 같이 작동합니다:
  * `Stop` 및 `SubagentStop`: `reason`이 Claude에게 피드백으로 전달되어 계속 작업합니다
  * `PreToolUse`: 도구 호출이 거부되고 `reason`이 도구 오류로 Claude에게 반환되어 조정하고 계속할 수 있습니다
  * `PostToolUse`, `PostToolBatch`, `UserPromptSubmit`, 및 `UserPromptExpansion`: 턴이 종료되고 `reason`이 경고 줄로 채팅에 나타납니다

이 예제는 `Stop` hook을 사용하여 모든 요청된 작업이 완료되었는지 모델에 묻습니다. 모델이 `"ok": false`를 반환하면 Claude는 계속 작업하고 `reason`을 다음 지침으로 사용합니다:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Check if all tasks are complete. If not, respond with {\"ok\": false, \"reason\": \"what remains to be done\"}."
          }
        ]
      }
    ]
  }
}
```

전체 구성 옵션은 참조의 [프롬프트 기반 hooks](/docs/ko/hooks#prompt-based-hooks)를 참조하세요.

<h2 id="agent-based-hooks">
  에이전트 기반 hooks
</h2>

<Warning>
  에이전트 hooks는 실험적입니다. 동작 및 구성은 향후 릴리스에서 변경될 수 있습니다. 프로덕션 워크플로우의 경우 [명령 hooks](/docs/ko/hooks#command-hook-fields)를 선호합니다.
</Warning>

검증에 파일 검사 또는 명령 실행이 필요한 경우 `type: "agent"` hooks를 사용합니다. 단일 LLM 호출을 수행하는 프롬프트 hooks와 달리 에이전트 hooks는 파일을 읽고 코드를 검색하며 결정을 반환하기 전에 다른 도구를 사용할 수 있는 subagent를 생성합니다.

에이전트 hooks는 프롬프트 hooks와 동일한 `"ok"` / `"reason"` 응답 형식을 사용하지만 기본 타임아웃이 60초이고 최대 50개의 도구 사용 턴입니다.

이 예제는 Claude가 중지되기 전에 테스트가 통과하는지 확인합니다:

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

Hook 입력 데이터만으로 결정을 내릴 수 있을 때 프롬프트 hooks를 사용합니다. 코드베이스의 실제 상태에 대해 무언가를 확인해야 할 때 에이전트 hooks를 사용합니다.

전체 구성 옵션은 참조의 [에이전트 기반 hooks](/docs/ko/hooks#agent-based-hooks)를 참조하세요.

<h2 id="http-hooks">
  HTTP hooks
</h2>

`type: "http"` hooks를 사용하여 셸 명령을 실행하는 대신 이벤트 데이터를 HTTP 엔드포인트에 POST합니다. 엔드포인트는 명령 hook이 stdin에서 받을 것과 동일한 JSON을 받고 동일한 JSON 형식을 사용하여 HTTP 응답 본문을 통해 결과를 반환합니다.

HTTP hooks는 웹 서버, 클라우드 함수 또는 외부 서비스가 hook 로직을 처리하기를 원할 때 유용합니다: 예를 들어 팀 전체에서 도구 사용 이벤트를 기록하는 공유 감사 서비스입니다.

이 예제는 모든 도구 사용을 로컬 로깅 서비스에 게시합니다:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/tool-use",
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

엔드포인트는 명령 hooks와 동일한 [출력 형식](/docs/ko/hooks#json-output)을 사용하여 JSON 응답 본문을 반환해야 합니다. 도구 호출을 차단하려면 적절한 `hookSpecificOutput` 필드와 함께 2xx 응답을 반환합니다. HTTP 상태 코드만으로는 작업을 차단할 수 없습니다.

헤더 값은 `$VAR_NAME` 또는 `${VAR_NAME}` 구문을 사용한 환경 변수 보간을 지원합니다. `allowedEnvVars` 배열에 나열된 변수만 해결됩니다. 다른 모든 `$VAR` 참조는 비어 있습니다.

전체 구성 옵션 및 응답 처리는 참조의 [HTTP hooks](/docs/ko/hooks#http-hook-fields)를 참조하세요.

<h2 id="limitations-and-troubleshooting">
  제한 사항 및 문제 해결
</h2>

<h3 id="limitations">
  제한 사항
</h3>

Hooks를 설계할 때 다음 제약 사항을 염두에 두십시오:

* 명령 hooks는 stdout, stderr 및 종료 코드를 통해서만 통신합니다. `/` 명령이나 도구 호출을 직접 트리거할 수 없습니다. `additionalContext`를 통해 반환된 텍스트는 Claude가 일반 텍스트로 읽는 시스템 알림으로 주입됩니다. HTTP hooks는 응답 본문을 통해 통신합니다.
* Hook 타임아웃은 유형에 따라 다릅니다. `timeout` 필드(초 단위)로 hook당 재정의할 수 있습니다.
  * `command`, `http`, `mcp_tool`: 10분. `UserPromptSubmit`은 이를 30초로 낮추고, `MessageDisplay`는 이를 10초로 낮춥니다.
  * `prompt`: 30초.
  * `agent`: 60초.
* `PostToolUse` hooks는 도구가 이미 실행되었으므로 작업을 취소할 수 없습니다.
* `PermissionRequest` hooks는 [비대화형 모드](/docs/ko/headless)에서 `-p` 플래그와 함께 발생하지 않습니다. 자동화된 권한 결정을 위해 `PreToolUse` hooks를 사용합니다.
* `Stop` hooks는 작업 완료 시에만이 아니라 Claude가 응답을 완료할 때마다 발생합니다. 사용자 중단 시에는 발생하지 않습니다. API 오류는 대신 [StopFailure](/docs/ko/hooks#stopfailure)를 발생시킵니다.
* 여러 `PreToolUse` hooks가 [`updatedInput`](/docs/ko/hooks#pretooluse)을 반환하여 도구의 인수를 다시 쓸 때 마지막으로 완료된 것이 우선합니다. Hooks는 병렬로 실행되므로 순서는 비결정적입니다. 동일한 도구의 입력을 수정하는 hook이 두 개 이상 있는 것을 피합니다.

<h3 id="hooks-and-permission-modes">
  Hooks 및 권한 모드
</h3>

`PreToolUse` hooks는 모든 권한 모드 확인 전에 발생합니다. `permissionDecision: "deny"`를 반환하는 hook은 `bypassPermissions` 모드 또는 `--dangerously-skip-permissions`에서도 도구를 차단합니다. 이를 통해 사용자가 권한 모드를 변경하여 우회할 수 없는 정책을 적용할 수 있습니다.

반대는 사실이 아닙니다: `"allow"`를 반환하는 hook은 설정의 거부 규칙을 우회하지 않으며, 조직이 `ask`로 설정한 [커넥터 도구](/docs/ko/mcp#organization-controls-on-connector-tools) 또는 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구의 프롬프트를 억제할 수 없습니다. Hooks는 제한을 강화할 수 있지만 권한 규칙이 허용하는 것을 초과하여 완화할 수 없습니다.

<h3 id="hook-not-firing">
  Hook이 발생하지 않음
</h3>

Hook이 구성되었지만 실행되지 않습니다.

* `/hooks`를 실행하고 hook이 올바른 이벤트 아래에 나타나는지 확인합니다
* Matcher 패턴이 도구 이름과 정확히 일치하는지 확인합니다. Matchers는 대소문자 구분입니다
* 올바른 이벤트 유형을 트리거하는지 확인합니다: `PreToolUse`는 도구 실행 전에 발생하고 `PostToolUse`는 후에 발생합니다
* 비대화형 모드에서 `-p` 플래그와 함께 `PermissionRequest` hooks를 사용하는 경우 대신 `PreToolUse`로 전환합니다

<h3 id="hook-error-in-output">
  출력에 Hook 오류
</h3>

트랜스크립트에 "PreToolUse hook error: ..." 같은 메시지가 표시됩니다.

* 스크립트가 예기치 않게 0이 아닌 코드로 종료되었습니다. 샘플 JSON을 파이프하여 수동으로 테스트합니다:
  ```bash theme={null}
  echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
  echo $?  # 종료 코드 확인
  ```
* "command not found"가 표시되면 절대 경로를 사용하거나 `${CLAUDE_PROJECT_DIR}`을 사용하여 스크립트를 참조합니다. 셸 인용을 완전히 피하려면 `"args": []`를 추가하여 [exec form](/docs/ko/hooks#exec-form-and-shell-form)으로 전환하면 셸 없이 스크립트를 직접 생성합니다
* "jq: command not found"가 표시되면 `jq`를 설치하거나 JSON 구문 분석을 위해 Python/Node.js를 사용합니다
* 스크립트가 실행되지 않으면 실행 가능하게 만듭니다: `chmod +x ./my-hook.sh`

<h3 id="/hooks-shows-no-hooks-configured">
  `/hooks`에 구성된 hooks가 없음
</h3>

설정 파일을 편집했지만 hooks가 메뉴에 나타나지 않습니다.

* 파일 편집은 일반적으로 자동으로 선택됩니다. 몇 초 후에 나타나지 않으면 파일 감시자가 변경을 놓쳤을 수 있습니다: 세션을 다시 시작하여 강제로 다시 로드합니다.
* JSON이 유효한지 확인합니다: 후행 쉼표 및 주석은 허용되지 않습니다
* 설정 파일이 올바른 위치에 있는지 확인합니다: 프로젝트 hooks의 경우 `.claude/settings.json`, 전역 hooks의 경우 `~/.claude/settings.json`

<h3 id="stop-hook-hits-the-block-cap">
  Stop hook이 블록 상한에 도달함
</h3>

Claude가 계속 작업하는 대신 중지하고 Stop hook이 너무 많은 횟수를 연속으로 차단했다는 경고로 턴을 종료합니다.

Claude Code는 Stop hook이 진행 없이 8번 연속으로 차단한 후 재정의합니다. Hook 스크립트는 이미 트리거되었는지 확인해야 합니다. JSON 입력에서 `stop_hook_active` 필드를 구문 분석하고 `true`인 경우 조기에 종료합니다:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Claude가 중지되도록 허용
fi
# ... hook 로직의 나머지
```

Hook이 수렴하기 위해 8번 이상의 반복이 필요한 경우 [`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](/docs/ko/env-vars)으로 상한을 올립니다.

<h3 id="json-validation-failed">
  JSON 검증 실패
</h3>

Hook 스크립트가 유효한 JSON을 출력하더라도 Claude Code에 JSON 구문 분석 오류가 표시됩니다.

Claude Code가 셸 형식 명령 hook(`args` 없는 hook)을 실행할 때 macOS 및 Linux에서는 기본적으로 `sh -c`를 생성하거나 Windows에서는 Git Bash를 생성합니다. 이 셸은 비대화형이지만 Git Bash 및 일부 구성(예: `BASH_ENV`가 `~/.bashrc`를 가리킴)은 여전히 프로필을 소싱합니다. 해당 프로필에 무조건적인 `echo` 문이 포함되어 있으면 출력이 hook의 JSON에 앞에 붙습니다:

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

Claude Code는 이를 JSON으로 구문 분석하려고 하고 실패합니다. 이를 수정하려면 셸 프로필의 echo 문을 래핑하여 대화형 셸에서만 실행되도록 합니다:

```bash theme={null}
# ~/.zshrc 또는 ~/.bashrc에서
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

`$-` 변수는 셸 플래그를 포함하고 `i`는 대화형을 의미합니다. Hooks는 비대화형 셸에서 실행되므로 echo는 건너뜁니다.

<h3 id="debug-techniques">
  디버그 기법
</h3>

트랜스크립트 보기는 `Ctrl+O`로 전환되며 발생한 각 hook에 대해 한 줄 요약을 표시합니다: 성공은 자동으로 표시되고, 차단 오류는 stderr를 표시하며, 차단하지 않는 오류는 `<hook name> hook error` 공지를 표시한 후 stderr의 첫 번째 줄을 표시합니다.

전체 실행 세부 정보(일치한 hooks, 종료 코드, stdout 및 stderr 포함)는 디버그 로그를 읽습니다. `claude --debug-file /tmp/claude.log`로 Claude Code를 시작하여 알려진 경로에 쓰거나 다른 터미널에서 `tail -f /tmp/claude.log`를 실행합니다. 해당 플래그 없이 시작한 경우 세션 중에 `/debug`를 실행하여 로깅을 활성화하고 로그 경로를 찾습니다.

<h2 id="learn-more">
  자세히 알아보기
</h2>

* [Hooks 참조](/docs/ko/hooks): 전체 이벤트 스키마, JSON 출력 형식, 비동기 hooks 및 MCP 도구 hooks
* [보안 고려 사항](/docs/ko/hooks#security-considerations): 공유 또는 프로덕션 환경에서 hooks를 배포하기 전에 검토합니다
* [Bash 명령 검증기 예제](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py): 완전한 참조 구현
