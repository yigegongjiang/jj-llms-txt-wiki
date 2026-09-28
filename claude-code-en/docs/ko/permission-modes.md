> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 권한 모드 선택

> Claude가 작업하기 전에 묻는지 여부를 제어합니다. CLI에서 Shift+Tab, VS Code의 모드 표시기 또는 Desktop의 모드 선택기로 권한 모드를 전환합니다.

권한 모드는 Claude가 먼저 묻지 않고 세션에서 수행할 수 있는 작업을 설정합니다. Manual 모드에서는 Claude Code가 파일을 편집하거나 셸 명령을 실행하거나 네트워크에 접근하는 대부분의 작업 전에 멈추고 사용자에게 묻습니다. [auto 모드](#eliminate-prompts-with-auto-mode)에서는 두 번째 모델인 분류기가 사용자 대신 작업을 검토합니다. [분류기가 작업을 평가하는 방식](#how-the-classifier-evaluates-actions)에서는 분류기가 검토하는 작업과 건너뛰는 작업을 나열합니다.

Pro, Max, Team 플랜에서는 기본 제공되는 시작 권한 모드가 auto 모드입니다. [세션이 시작되는 모드](#which-mode-a-session-starts-in)에서는 시작 권한 모드를 변경하는 표면과 설정을 다룹니다. 실행 중인 세션의 권한 모드는 언제든지 변경할 수 있습니다.

<h2 id="available-modes">
  사용 가능한 모드
</h2>

각 모드는 편의성과 감시 사이에서 서로 다른 트레이드오프를 만듭니다. 아래 표는 각 모드에서 Claude가 권한 프롬프트 없이 수행할 수 있는 작업을 보여줍니다. 수동 모드는 구성 값인 `default` 아래에 나타납니다.

| 모드                                                                  | 요청 없이 실행되는 작업                                                             | 최적 사용 사례             |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------ | :------------------- |
| `default`                                                           | 읽기만                                                                       | 모든 작업을 직접 검토, 민감한 작업 |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | 읽기, 파일 편집, 일반적인 파일시스템 명령어 (`mkdir`, `touch`, `mv`, `cp` 등)                | 검토 중인 코드 반복 작업       |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | 읽기, 그리고 [자동 모드](#eliminate-prompts-with-auto-mode)를 사용할 수 있을 때 분류기 승인 명령어 | 변경 전 코드베이스 탐색        |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | 백그라운드 안전 검사를 포함한 모든 작업                                                    | 장시간 작업, 프롬프트 피로 감소   |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | 읽기 및 사전 승인된 도구; 프롬프트를 표시할 모든 작업은 거부됨                                      | 잠금된 CI 및 스크립트        |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | 모든 작업                                                                     | 격리된 컨테이너 및 VM만 해당    |

모든 작업을 검토하는 모드는 CLI에서 **Manual**이라고 명명되며, `claude --help`에서, VS Code 및 JetBrains 확장 프로그램에서, 그리고 데스크톱 앱에서도 **Manual**이라고 명명됩니다. 구성 값은 `default`이며, 이는 hooks 및 SDK 통합이 사용하는 값입니다. CLI는 값을 입력하는 모든 곳에서 `manual`을 별칭으로 허용합니다. 예를 들어 `claude --permission-mode manual` 또는 `"defaultMode": "manual"`입니다. Manual 레이블과 `manual` 별칭은 Claude Code v2.1.200 이상이 필요합니다. 데스크톱 앱의 레이블은 CLI 버전에 따라 달라지지 않습니다.

[보호된 경로](#protected-paths)에 대한 쓰기는 `bypassPermissions` 모드에서와 bypass 권한을 사용할 수 있는 plan-mode 세션에서를 제외하고는 자동 승인되지 않습니다. 이는 [모드 사이클에 `bypassPermissions`를 배치하는 방식으로 시작된](#switch-permission-modes) 대화형 터미널 세션을 의미합니다.

모드는 기준선을 설정합니다. 특정 도구를 사전 승인하거나 차단하기 위해 [권한 규칙](/docs/ko/permissions#manage-permissions)을 위에 계층화합니다. 거부 규칙은 `bypassPermissions`를 포함한 모든 모드에서 차단합니다. 거부 및 요청 규칙은 Claude가 호출할 수 있는 다른 도구가 최소 하나 이상 있는 한 [`EndConversation`](/docs/ko/tools-reference#endconversation-tool-behavior)에 적용되지 않습니다. 허용 규칙은 `bypassPermissions`에서 효과가 없습니다.

<h3 id="actions-no-mode-auto-approves">
  어떤 모드도 자동 승인하지 않는 작업
</h3>

Claude Code는 `bypassPermissions`를 포함한 어떤 모드에서도 다음을 자동 승인하지 않습니다. 각 항목은 각 모드에서 대신 어떤 일이 발생하는지를 설명하는 섹션으로 연결됩니다:

* 명시적 [요청 규칙](/docs/ko/permissions#manage-permissions)과 일치하는 도구
* 조직이 [요청으로 설정한](/docs/ko/mcp#organization-controls-on-connector-tools) 커넥터 도구, 해당 설정이 Claude Code에 도달하는 세션에서
* 사용자 상호작용이 필요한 도구: 기본 제공 `AskUserQuestion` 도구 및 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구
* [중요 경로](#critical-paths)를 대상으로 하는 `rm` 및 `rmdir` 제거, 어떤 허용 규칙이나 `PreToolUse` hook `"allow"`도 승인하지 않음
* [세션 간 메시징 보안 조치](#skip-all-checks-with-bypasspermissions-mode)
* [`permissions.blockReadsOutsideWorkingDirectories`](/docs/ko/settings-reference#permissions-blockreadsoutsideworkingdirectories)가 켜져 있는 동안 작업 디렉토리 외부의 읽기: 인식된 파일 읽기 Bash 명령어는 자동 모드 및 `bypassPermissions` 모드에서도 프롬프트하며, 샌드박스 외부에서 실행하기 위해 승인이 필요한 모든 [샌드박스 해제 재시도](/docs/ko/sandboxing#the-unsandboxed-retry-escape-hatch)도 마찬가지입니다. Claude Code v2.1.257 이상이 필요합니다.

  셸 파서가 추적할 수 없는 명령어(예: 두 번 이상 디렉토리를 변경하거나 서브셸을 실행하는 명령어)는 외부 경로를 명명하지 않더라도 동일한 방식으로 프롬프트합니다. 이 프롬프트는 명령어가 [샌드박스](/docs/ko/sandboxing)에서 실행되고 샌드박스가 블록을 적용할 때는 적용되지 않습니다.

<h2 id="common-setups">
  일반적인 설정
</h2>

권한 모드는 Claude가 작업 전에 물어보는지 여부를 결정하고, [Bash sandbox](/docs/ko/sandboxing) 및 외부 [isolation boundaries](/docs/ko/sandbox-environments)는 작업이 실행되면 도달할 수 있는 것을 결정합니다. 아래의 각 행은 목표를 시작점으로 그곳에 도달하는 플래그 또는 설정 및 필요한 격리와 쌍을 이룹니다. [사용 가능한 모드](#available-modes)는 각 모드에서 프롬프트 없이 실행되는 것을 나열합니다.

| 원하는 것                     | 시작                                                                                                                                                | 필요한 격리                                                                                                                                                          | 참고                                                                                                                                                                          |
| :------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 모든 작업을 직접 검토              | Manual 모드: `claude --permission-mode default`                                                                                                     | 없음                                                                                                                                                              | 민감한 작업, 낯선 코드                                                                                                                                                               |
| 분류기 없이 더 적은 프롬프트로 로컬에서 반복 | Manual 모드 + [auto-allow mode](/docs/ko/sandboxing#sandbox-modes)의 Bash sandbox: `claude --permission-mode default`, 그 다음 `/sandbox` 실행 및 auto-allow 선택 | 기본 제공 Bash sandbox, macOS, Linux, WSL2                                                                                                                          | Deny 규칙은 여전히 적용되고, `Bash(git push *)`와 같이 명령을 지정하는 ask 규칙은 여전히 프롬프트합니다. 설정 파일에서 대신 sandbox를 켜려면 [`sandbox.enabled`](/docs/ko/settings-reference#sandbox-enabled)를 `true`로 설정합니다. |
| 변경 전에 탐색                  | `claude --permission-mode plan`                                                                                                                   | 없음                                                                                                                                                              | Claude Code는 [계획을 승인](#review-and-approve-a-plan)할 때까지 편집을 차단합니다.                                                                                                           |
| auto mode에서 자동 실행         | `claude --permission-mode auto`, Pro, Max, Team의 [기본 제공 시작 권한 모드](#which-mode-a-session-starts-in)                                                | 없음; sandbox 또는 컨테이너는 심층 방어를 추가합니다.                                                                                                                              | [지원되는 모델](#eliminate-prompts-with-auto-mode)이 필요하고, 조직이 [auto mode를 끌 수 있습니다](#eliminate-prompts-with-auto-mode).                                                           |
| 정확한 allowlist로 CI에서 실행    | `claude -p "run the test suite" --permission-mode dontAsk --allowedTools "Bash(npm test)" "Read"`                                                 | CI 러너가 제공하는 것 이상 없음                                                                                                                                             | [웹의 Claude Code](/docs/ko/claude-code-on-the-web)는 설정 파일의 `dontAsk`를 무시합니다.                                                                                                      |
| 컨테이너 내에서 완전히 자동 실행        | `claude -p "<prompt>" --dangerously-skip-permissions`                                                                                             | 필수: 컨테이너, VM, 또는 [sandbox runtime](/docs/ko/sandbox-environments#sandbox-runtime); Linux 및 macOS에서 [non-root user](#skip-all-checks-with-bypasspermissions-mode)로 실행 | 웹의 Claude Code는 설정 파일의 이 모드를 무시합니다. 이 `-p` 실행에서 [여전히 프롬프트할 몇 가지 호출](#skip-all-checks-with-bypasspermissions-mode)은 대신 거부됩니다.                                                |

Bash sandbox와 auto mode는 독립적으로 작동하고 결합합니다. [Sandbox modes](/docs/ko/sandboxing#sandbox-modes)에 나열된 예외가 있습니다. 전체 상호작용은 [How sandboxing relates to permissions and permission modes](/docs/ko/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes) 및 [How isolation relates to permission modes](/docs/ko/sandbox-environments#how-isolation-relates-to-permission-modes)를 참조하세요.

<h2 id="which-mode-a-session-starts-in">
  세션이 시작되는 모드
</h2>

터미널에서 새 세션을 시작할 때 Claude Code는 다음 중 첫 번째로 적용되는 것에서 권한 모드를 가져옵니다:

1. `--permission-mode` 플래그 또는 `--dangerously-skip-permissions`

2. [설정 파일](/docs/ko/settings#where-settings-live)의 `permissions.defaultMode`

   `.claude/settings.json` 또는 `.claude/settings.local.json`에서 `"auto"`를 설정하면 값이 적용되지 않고 Claude Code는 `~/.claude/settings.json`의 `defaultMode` 대신 기본 제공 기본값을 사용합니다. 이 두 파일에서 `"bypassPermissions"`를 설정하면 적용되지 않으며 세션은 Manual 모드에서 시작됩니다. 다른 값은 모든 설정 파일에서 적용됩니다.

3. 기본 제공 기본값

VS Code 확장이 시작하는 대화는 [권한 모드 전환](#switch-permission-modes)의 확장 자체 목록을 따릅니다. Claude Code가 재개된 세션을 시작하는 권한 모드는 [resume 시 권한 모드](/docs/ko/sessions#permission-mode-on-resume)를 참조하세요.

기본 제공 `auto` 기본값은 macOS, Linux, WSL에서 Claude Code v2.1.228 이상이 필요하고, 네이티브 Windows에서는 v2.1.233 이상이 필요합니다. 이전 버전에서는 기본 제공 기본값은 Manual입니다.

기본 제공 기본값은 Claude Code를 실행하는 방법, 플랜, Claude Code가 기능 플래그를 가져올 수 있는지 여부에 따라 달라집니다. 세션과 일치하는 첫 번째 행이 적용됩니다. 표는 터미널 또는 VS Code 확장을 통해 시작하는 세션을 다룹니다. 데스크톱 앱 및 claude.ai는 [권한 모드 전환](#switch-permission-modes)의 Desktop 및 Web 탭을 참조하세요.

| Claude Code를 실행하는 방법                                                                                                                                                             | 기본 제공 시작 권한 모드 |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------- |
| 모든 설정 파일이 `disableAutoMode`를 `"disable"`로 설정                                                                                                                                     | `default`      |
| [기능 플래그 가져오기](/docs/ko/env-vars#features-that-need-feature-flag-fetching)가 꺼짐                                                                                                         | `default`      |
| [이 기본값을 추가하는 버전으로 설치 또는 업그레이드 후 첫 번째 세션](/docs/ko/env-vars#first-session-after-an-install-or-upgrade), 신선한 설치 후 Claude Code가 시간 내에 플래그를 가져오지 않은 경우 제외                                 | `default`      |
| `claude -p` 또는 [Agent SDK](/docs/ko/agent-sdk/permissions)                                                                                                                            | `default`      |
| Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/ko/claude-platform-on-aws), 또는 로그인한 [Claude apps gateway](/docs/ko/claude-apps-gateway) 세션 | `default`      |
| 터미널 또는 [VS Code 확장](/docs/ko/vs-code)의 Pro, Max, Team 플랜                                                                                                                              | `auto`         |
| Enterprise 플랜 또는 Claude Console API 키                                                                                                                                            | `default`      |

기능 플래그 가져오기가 꺼져 있거나 플래그가 아직 도착하지 않은 [설치 또는 업그레이드 후 첫 번째 세션](/docs/ko/env-vars#first-session-after-an-install-or-upgrade)에서 VS Code 확장은 시작 권한 모드를 선택할 때 모든 설정 파일을 무시합니다.

플래그, 설정 파일, 또는 기본 제공 기본값이 `auto`를 선택하지만 auto mode를 세션에서 사용할 수 없으면 Claude Code는 대신 Manual에서 세션을 시작합니다. Auto mode는 세션이 [가용성 요구사항](#eliminate-prompts-with-auto-mode)을 충족하지 않을 때 사용할 수 없습니다. 예를 들어 설정 파일이 이를 끄거나 지원하지 않는 모델이거나, Anthropic이 서버 측에서 임시로 이를 끈 경우입니다.

기본 제공 기본값이 처음으로 세션 중 하나를 auto mode에서 시작할 때 Claude Code는 이 페이지로 연결되는 알림을 표시합니다:

* 터미널에서 세션 상단에 한 번
* VS Code 확장에서 새 대화 화면의 카드로 해제할 때까지 유지됨

Pro, Max, Team 플랜에서 `~/.claude/settings.json`이 `auto` 이외의 `defaultMode`를 설정하고 다른 설정 파일이 설정하지 않으면 세션은 해당 모드에서 계속 시작됩니다. Claude Code는 터미널 또는 VS Code 확장에서 한 번 설정을 auto mode로 변경할지 묻습니다. 거부하면 설정은 그대로 유지됩니다.

<h3 id="start-in-a-different-mode">
  다른 권한 모드에서 시작
</h3>

한 세션, 또는 머신, 프로젝트, 또는 조직의 모든 세션에 대한 기본값으로 시작 권한 모드를 설정할 수 있습니다. 둘 이상의 설정 파일이 `permissions.defaultMode`를 설정하면 [설정 우선순위](/docs/ko/settings#settings-precedence)가 결정하므로 프로젝트 또는 관리형 값이 `~/.claude/settings.json`을 능가합니다. 이미 실행 중인 세션의 권한 모드를 변경하려면 [권한 모드 전환](#switch-permission-modes)을 참조하세요.

| 시작 권한 모드를 설정하려면         | 이렇게 하세요                                                                                                                                                                                                                                                                     |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 시작하려는 한 세션              | 권한 모드를 플래그로 전달합니다. 예: `claude --permission-mode default`                                                                                                                                                                                                                    |
| 이 머신에서 시작하는 모든 터미널 세션   | `~/.claude/settings.json`에서 `permissions.defaultMode`를 설정합니다. VS Code 확장이 읽는 것은 [권한 모드 전환](#switch-permission-modes)을 참조하세요.                                                                                                                                                |
| 한 프로젝트에서 시작하는 모든 터미널 세션 | 프로젝트의 `.claude/settings.json`에서 `permissions.defaultMode`를 설정합니다. 터미널에서 시작하는 세션은 `auto` 및 `bypassPermissions`를 제외한 모든 값을 준수합니다. VS Code 확장이 시작하는 세션은 시작 권한 모드에 대해 프로젝트 설정을 읽지 않습니다.                                                                                         |
| 조직의 모든 터미널 세션           | [관리형 설정](/docs/ko/managed-settings)에서 `permissions.defaultMode`를 설정합니다. 터미널 세션은 해당 모드에서 시작하고 사람들은 여전히 auto mode로 전환할 수 있습니다. VS Code 확장이 읽는 것은 [권한 모드 전환](#switch-permission-modes)을 참조하세요. auto mode를 제거하여 아무도 선택할 수 없도록 하려면 `permissions.disableAutoMode`를 `"disable"`로 설정합니다. |

이 예제는 머신의 모든 터미널 세션을 Manual 모드(설정 값 `default`)에서 시작하도록 합니다. `~/.claude/settings.json`에 저장합니다:

```json theme={null}
{
  "permissions": {
    "defaultMode": "default"
  }
}
```

다음 세션은 상태 표시줄에 `⏸ manual mode on`을 표시합니다.

<h2 id="switch-permission-modes">
  권한 모드 전환
</h2>

각 인터페이스는 세션 중에 권한 모드를 전환하기 위한 자체 컨트롤과 새 세션이 시작되는 권한 모드를 선택하는 자체 방법을 가집니다. 인터페이스를 선택하여 컨트롤을 확인하세요.

<Tabs>
  <Tab title="CLI">
    **세션 중**: `Shift+Tab`을 눌러 권한 모드를 순환합니다. `auto`에서 첫 번째 누름은 `default`로 전환하고 순환은 `default` → `acceptEdits` → `plan` → `default`로 돌아갑니다. 아래에 설명된 선택적 모드는 `plan` 후에 슬롯됩니다. 상태 표시줄은 활성 모드를 `default`의 경우 회색 `⏸ manual mode on`으로, 또는 `⏵⏵ accept edits on`, `⏸ plan mode on`, `⏵⏵ auto mode on`, `⏵⏵ don't ask on`, 또는 `⏵⏵ bypass permissions on`으로 표시합니다.

    모든 모드가 기본 순환에 포함되는 것은 아닙니다:

    * `auto`: [auto mode를 사용할 수 있을 때](#eliminate-prompts-with-auto-mode) 나타나며, 이로 순환하면 확인 프롬프트 없이 권한 모드가 전환됩니다.
    * `bypassPermissions`: `--permission-mode bypassPermissions`, `--dangerously-skip-permissions`, `--allow-dangerously-skip-permissions`, 또는 [user, `--settings`, 또는 managed settings](/docs/ko/settings-reference#permissions-defaultmode)의 `permissions.defaultMode: "bypassPermissions"` 후에 나타납니다. `--allow-` 변형은 활성화하지 않고 순환에 권한 모드를 추가합니다.
    * `dontAsk`: 순환에 나타나지 않으며, `--permission-mode dontAsk`로 설정합니다.

    활성화된 선택적 모드는 `plan` 후에 슬롯되며, `bypassPermissions`가 먼저이고 `auto`가 마지막입니다. 둘 다 활성화된 경우 `auto`로 가는 길에 `bypassPermissions`를 순환합니다.

    **Bash 권한 프롬프트에서**: Manual 및 `acceptEdits` 권한 모드에서 [auto mode](#eliminate-prompts-with-auto-mode)를 사용할 수 있을 때 Claude Code는 Bash 명령의 권한 프롬프트에 **Yes, and switch to auto mode**를 추가합니다. 이를 선택하여 명령을 승인하고 세션을 auto mode로 전환합니다. [PowerShell tool](/docs/ko/tools-reference#powershell-tool) 프롬프트는 옵션을 제공하지 않습니다. Claude Code v2.1.247 이상이 필요합니다.

    Claude Code는 [`ask` 규칙](/docs/ko/permissions#manage-permissions) 중 하나 또는 [hook](/docs/ko/hooks#pretooluse-decision-control)에 의해 강제된 프롬프트에 옵션을 추가하지 않습니다. auto mode는 여전히 이러한 프롬프트를 표시하므로 전환해도 제거되지 않습니다.

    **시작 시**: 권한 모드를 플래그로 전달합니다.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **기본값으로**: [다른 권한 모드에서 시작](#start-in-a-different-mode)에 설명된 대로 원하는 범위에서 `permissions.defaultMode`를 설정합니다.

    동일한 `--permission-mode` 플래그는 [비대화형 실행](/docs/ko/headless)을 위해 `-p`와 함께 작동합니다.
  </Tab>

  <Tab title="VS Code">
    **세션 중**: 프롬프트 상자 하단의 모드 표시기를 클릭합니다. 이 페이지의 모드에 대해 다음 레이블을 사용합니다:

    | UI 레이블             | 모드                  |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    **기본값으로**: 대화가 시작되는 권한 모드를 고정하려면 VS Code 사용자 설정에서 `claudeCode.initialPermissionMode`를 `default`, `manual`, `acceptEdits`, `plan`, 또는 `bypassPermissions`로 설정합니다. 설정은 `auto`를 허용하지 않습니다. Auto에서 시작하려면 설정하지 않은 상태로 두고 아래 항목 2에 설명된 대로 모드 표시기에서 **Auto**를 한 번 선택합니다. 확장은 다음 중 첫 번째로 적용되는 것에서 각 새 대화를 시작합니다:

    1. `claudeCode.initialPermissionMode`
    2. 마지막으로 모드 표시기에서 선택한 모드(Manual, Edit automatically, 또는 Auto인 경우). Plan 또는 Bypass permissions를 선택하면 해당 대화에만 적용됩니다.
    3. [관리형 설정](/docs/ko/managed-settings) 또는 `~/.claude/settings.json`의 `permissions.defaultMode`, Pro, Max, Team 플랜에서 [기능 플래그 가져오기](#which-mode-a-session-starts-in)를 사용할 수 있는 경우
    4. 플랜, 제공자, 조직 설정에 대한 [기본 제공 기본값](#which-mode-a-session-starts-in)

    확장은 시작 권한 모드에 대해 프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json`을 읽지 않으며, 항목 3의 조건을 충족하지 않는 대화에서는 설정 파일을 전혀 읽지 않습니다. `claudeCode.claudeProcessWrapper`가 설정되면 항목 3과 4도 적용되지 않습니다: 항목 1 또는 항목 2가 권한 모드를 설정하지 않으면 이러한 대화는 Manual에서 시작됩니다.

    Auto는 [auto mode를 사용할 수 있을 때](#eliminate-prompts-with-auto-mode) 모드 표시기에 나타납니다.

    Bypass permissions는 확장 설정의 **Allow dangerously skip permissions** 토글이 필요합니다. 없으면 권한 모드가 표시기에 나타나지 않으며, 항목 1 또는 항목 3의 `bypassPermissions` 값은 대신 대화를 Manual에서 시작합니다. 항목의 Auto는 auto mode를 사용할 수 없을 때 마찬가지로 대화를 Manual에서 시작합니다.

    [VS Code 가이드](/docs/ko/vs-code)에서 확장 관련 세부사항을 참조하세요.
  </Tab>

  <Tab title="JetBrains">
    JetBrains 플러그인은 IDE 터미널에서 Claude Code를 실행하므로 권한 모드 전환은 CLI와 동일하게 작동합니다: `Shift+Tab`을 눌러 순환하거나 시작할 때 `--permission-mode`를 전달합니다.
  </Tab>

  <Tab title="Desktop">
    **세션 중**: Code 탭에서 전송 버튼 옆의 모드 선택기를 사용합니다. 모든 모드가 선택기에 나타나는 것은 아닙니다:

    * **Auto**: [auto mode를 사용할 수 있을 때](#eliminate-prompts-with-auto-mode) 나타납니다.
    * **Bypass permissions**: Pro 및 Max 플랜에서 Desktop 설정의 **Allow bypass permissions mode** 토글이 필요합니다. Team 및 Enterprise 플랜에서는 조직 정책이 대신 제어합니다.

    Cowork 탭은 이러한 모드를 사용하지 않습니다. Cowork는 자체 권한 모드를 가지며 별도로 활성화되고, Cowork 탭은 계정에 대해 기본값을 초과하는 모드가 활성화될 때까지 모드 선택기를 표시하지 않습니다. [Cowork 문서](https://claude.com/docs/cowork/overview)를 참조하세요.

    데스크톱 관련 세부사항은 Desktop 가이드의 [권한 모드 선택](/docs/ko/desktop#choose-a-permission-mode)을 참조하세요.

    **기본값으로**: [설정](/docs/ko/settings#where-settings-live)에서 `defaultMode`를 설정합니다. 데스크톱 앱은 CLI와 동일한 설정 파일을 읽고 새 로컬 세션에 권한 모드를 적용합니다.

    모드 선택기에서 선택한 모드는 폴더별로 기억되며 해당 폴더에 대해 `defaultMode`보다 우선합니다. Plan은 예외입니다: 선택하면 현재 세션에만 적용됩니다.

    [다른 권한 모드에서 시작](#start-in-a-different-mode) 아래의 예제에서 `defaultMode`가 설정 파일의 어디로 가는지 참조하세요.
  </Tab>

  <Tab title="Web and mobile">
    [claude.ai/code](https://claude.ai/code)의 모드 드롭다운 또는 모바일 앱의 프롬프트 상자 옆을 사용합니다. 권한 프롬프트는 승인을 위해 claude.ai에 나타납니다. 나타나는 모드는 세션이 실행되는 위치에 따라 달라집니다:

    * **[Claude Code on the web](/docs/ko/claude-code-on-the-web)의 클라우드 세션**: Accept edits, Plan, and Auto. Accept edits는 `default` 모드에 해당합니다: 클라우드 세션은 모드에 관계없이 파일 편집을 사전 승인하므로 드롭다운은 Manual 대신 Accept edits를 표시합니다. 클라우드 세션은 여전히 설정의 `defaultMode: "acceptEdits"`를 준수합니다. Auto mode는 조직이 허용하고 선택한 모델이 지원할 때만 나타납니다. Bypass permissions는 사용할 수 없습니다.
    * **로컬 머신의 [Remote Control](/docs/ko/remote-control) 세션**: Manual, Accept edits, and Plan. 앱에서 Auto 또는 Bypass permissions를 선택할 수 없습니다.
      * Bypass permissions 제외, 드롭다운은 터미널에서 설정된 모드를 포함하여 로컬 세션이 있는 권한 모드를 표시합니다. 앱 또는 터미널에서 권한 모드가 변경될 때 업데이트됩니다. 세션은 Bypass permissions를 claude.ai에 보고하지 않으므로 터미널에서 전환해도 드롭다운이 표시하는 내용이 변경되지 않습니다.
      * [desktop app](/docs/ko/desktop) 또는 [VS Code extension](/docs/ko/vs-code)이 호스팅하는 세션은 터미널에서 호스팅하는 세션과 동일하게 발생할 때 claude.ai에 권한 모드 변경을 보고합니다.
      * v2.1.202 이전에는 `/remote-control` 또는 `claude --remote-control`로 연결된 세션이 모드를 전혀 보고하지 않았으므로 claude.ai 및 모바일 앱이 세션이 실제로 있지 않은 권한 모드를 표시할 수 있었습니다. 불일치는 레이블에만 영향을 미쳤습니다. Claude Code는 세션의 실제 권한 모드에서 권한 프롬프트를 생성했으며, 여전히 승인을 위해 앱에 나타났습니다.

    Remote Control의 경우 로컬 머신이 claude.ai 계정으로 로그인되어야 합니다. API 키는 지원되지 않습니다. 해당 로컬 세션을 시작할 때 시작 권한 모드를 설정할 수도 있습니다:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  acceptEdits 모드로 파일 편집 자동 승인
</h2>

`acceptEdits` 모드를 사용하면 Claude가 프롬프트 없이 작업 디렉토리에서 파일을 생성하고 편집할 수 있습니다. 이 모드가 활성화되어 있는 동안 상태 표시줄에 `⏵⏵ accept edits on`이 표시됩니다.

파일 편집 외에도 `acceptEdits` 모드는 일반적인 파일시스템 Bash 명령어를 자동으로 승인합니다: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`. 이러한 명령어는 `LANG=C` 또는 `NO_COLOR=1`과 같은 안전한 환경 변수가 접두사로 붙거나 `timeout`, `nice`, `nohup`과 같은 프로세스 래퍼가 붙을 때도 자동으로 승인됩니다. 파일 편집과 마찬가지로 자동 승인은 작업 디렉토리 또는 `additionalDirectories` 내의 경로에만 적용됩니다. 해당 범위 외의 경로, [보호된 경로](#protected-paths)에 대한 쓰기, [critical path](#critical-paths)를 대상으로 하는 `rm` 및 `rmdir` 제거, 그리고 [읽기 전용 집합](/docs/ko/permissions#read-only-commands)을 제외한 다른 모든 Bash 명령어는 여전히 프롬프트를 표시합니다.

[PowerShell tool](/docs/ko/tools-reference#powershell-tool)이 활성화되어 있으면 `acceptEdits` 모드는 범위 내 경로에서 `Set-Content`, `Add-Content`, `Clear-Content`, `Remove-Item`과 이들의 일반적인 별칭도 자동으로 승인합니다. 동일한 범위 및 보호된 경로 규칙이 적용되고, `Remove-Item`은 [자체 확인](#remove-item-in-powershell)을 받습니다. `Set-Content .\notes.txt "It's done"`의 아포스트로피와 같이 따옴표 문자를 포함하는 위치 인수는 Claude Code가 따옴표 및 따옴표 없는 읽기가 다른 인수를 정적으로 검증할 수 없기 때문에 범위 내 경로에서도 여전히 프롬프트를 표시합니다. 프롬프트를 피하려면 `-Value`와 같은 명명된 매개변수를 통해 콘텐츠를 전달합니다.

편집을 인라인으로 승인하는 대신 편집기에서 또는 `git diff`를 통해 변경 사항을 검토하려는 경우 `acceptEdits`를 사용하세요.

Manual 모드에서 `Shift+Tab`을 한 번 누르면 이 모드로 진입하거나 직접 시작할 수 있습니다:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  편집하기 전에 계획 모드로 분석하기
</h2>

계획 모드는 Claude가 변경 사항을 연구하고 제안하도록 하되 실제로 적용하지 않습니다. Claude는 파일을 읽고 셸 명령을 실행하여 탐색한 후 계획을 작성하지만 소스를 편집하지 않습니다. [bypassPermissions 모드로 모든 검사 건너뛰기](#skip-all-checks-with-bypasspermissions-mode)가 가능한 대화형 터미널 세션을 제외하고, 편집은 계획을 승인할 때까지 차단됩니다.

[자동 모드](/docs/ko/auto-mode-config)를 사용할 수 있고 기본적으로 켜져 있는 `useAutoModeDuringPlan` 설정이 활성화되어 있으면, 분류기는 계획 중에 셸 명령을 검토하며 사용자에게 프롬프트를 표시하지 않습니다. 승인된 명령은 실행되고 거부된 명령은 차단됩니다. 그렇지 않으면 [기본 제공 읽기 전용 집합](/docs/ko/permissions#read-only-commands) 외의 명령은 승인을 요청하며, 샌드박스의 [자동 허용 모드](/docs/ko/sandboxing#sandbox-modes)가 활성화된 경우에도 마찬가지입니다. bypassPermissions를 사용할 수 있는 대화형 터미널 세션에서는 분류기나 프롬프트가 계획 명령에 적용되지 않습니다. [bypassPermissions 모드로 모든 검사 건너뛰기](#skip-all-checks-with-bypasspermissions-mode)는 여전히 프롬프트가 표시되는 몇 가지 항목을 다룹니다. v2.1.212부터 v2.1.217까지는 bypassPermissions가 없는 세션에서 자동 모드를 사용할 수 있는지 여부와 관계없이 읽기 전용 집합 외의 모든 명령에 대해 프롬프트를 표시했습니다.

계획 모드에 들어가려면 `Shift+Tab`을 누르거나 단일 프롬프트 앞에 `/plan`을 붙입니다. CLI에서 계획 모드로 시작할 수도 있습니다:

```bash theme={null}
claude --permission-mode plan
```

계획을 승인하지 않고 계획 모드를 종료하려면 `Shift+Tab`을 다시 누릅니다.

<h3 id="review-and-approve-a-plan">
  계획 검토 및 승인
</h3>

계획이 준비되면 Claude가 계획을 제시하고 어떻게 진행할지 묻습니다. 해당 프롬프트에서 다음을 선택할 수 있습니다:

* **예, 자동 모드 사용**: 승인하고 [자동 모드](#eliminate-prompts-with-auto-mode)로 시작합니다. 자동 모드를 조직에서 비활성화한 경우 등 [세션에서 사용할 수 없으면](#eliminate-prompts-with-auto-mode), 이 옵션은 **예, 편집 자동 수락**으로 표시됩니다. bypassPermissions가 활성화된 상태로 세션을 시작한 경우, 옵션은 **예, 이 세션에 대해 BYPASS PERMISSIONS(추가 프롬프트 없음)로 전환**으로 표시됩니다.
* **예, 편집 수동 승인**: 승인하고 각 편집을 개별적으로 검토합니다.
* **아니요, 계획 계속**: 계획 모드에 머물러 있고 Claude에게 변경할 사항을 알립니다.

계획을 승인하면 계획 모드가 종료되고 세션이 각 승인 옵션이 설명하는 권한 모드로 전환되므로 Claude가 편집을 시작합니다. 다시 계획하려면 `Shift+Tab`으로 계획 모드로 돌아가거나 다음 프롬프트 앞에 `/plan`을 붙입니다.

`Ctrl+G`를 눌러 제안된 계획을 기본 텍스트 편집기에서 열고 Claude가 진행하기 전에 직접 편집합니다. [`showClearContextOnPlanAccept`](/docs/ko/settings-reference#showclearcontextonplanaccept)가 활성화되면 목록에 계획을 승인하고 계획 컨텍스트를 지우는 첫 번째 옵션이 추가됩니다.

계획을 수락하면 세션에 계획을 기반으로 한 [생성된 제목](/docs/ko/sessions#name-your-sessions)이 지정되며, 이미 세션의 이름을 지정하지 않은 경우입니다.

<h3 id="set-plan-mode-as-the-default">
  계획 모드를 기본값으로 설정
</h3>

프로젝트의 터미널 세션에 대해 계획 모드를 기본값으로 설정하려면 `.claude/settings.json`에서 `defaultMode`를 `plan`으로 설정합니다. 이는 [다른 권한 모드에서 시작](#start-in-a-different-mode)의 예시에 표시된 대로 배치됩니다. [VS Code 확장](/docs/ko/vs-code)이 시작하는 대화는 시작 권한 모드에 대한 프로젝트 설정을 읽지 않습니다. 대신 VS Code 사용자 설정에서 `claudeCode.initialPermissionMode`를 `plan`으로 설정합니다.

<h2 id="eliminate-prompts-with-auto-mode">
  자동 모드로 권한 프롬프트 제거
</h2>

자동 모드를 사용하면 Claude가 일상적인 권한 프롬프트 없이 실행됩니다. 별도의 분류기 모델이 실행 전에 작업을 검토하여 요청을 초과하거나 인식되지 않은 인프라를 대상으로 하거나 Claude가 읽은 악의적인 콘텐츠로 인해 발생한 것으로 보이는 모든 것을 차단합니다. 명시적인 [요청 규칙](/docs/ko/permissions#manage-permissions)은 여전히 프롬프트를 강제합니다.

Pro, Max, Team 플랜에서 자동 모드는 [세션이 시작되는 기본 권한 모드](#which-mode-a-session-starts-in)입니다.

분류기는 또한 Claude가 [`SendMessage`](/docs/ko/tools-reference)를 사용하여 다른 에이전트에 보내는 각 메시지를 검토합니다. 일반 텍스트이든 구조화된 [에이전트 팀](/docs/ko/agent-teams) 메시지이든, Claude Code가 전달하기 전에 자동 모드와 [분류기가 명령을 검토하는 동안 계획 모드](#analyze-before-you-edit-with-plan-mode) 모두에서 검토합니다. 전송 검토에는 Claude Code v2.1.222 이상이 필요합니다.

분류기는 또한 `rm` 및 `rmdir` 제거를 검토하고 승인하거나 차단합니다. 이는 `rm -rf /` 및 `rm -rf ~`와 같은 [중요 경로](#critical-paths)를 대상으로 하며, 제거가 명령 또는 프로세스 치환 내부에 있을 때도 포함됩니다.

자동 모드는 또한 Claude가 명확한 질문을 위해 멈추지 않고 계속 작업하도록 권장하지만, Claude는 여전히 프롬프트나 스킬이 명시적으로 이를 요구할 때 질문합니다. 더 강력한 자율 동작을 원하면서도 여전히 프롬프트를 표시하는 모드를 원한다면 [사전 예방적 출력 스타일](/docs/ko/output-styles)을 설정하세요.

<Warning>
  자동 모드는 권한 프롬프트를 줄이지만 안전을 보장하지 않습니다. 일반적인 방향을 신뢰하는 작업에 사용하고, 민감한 작업에 대한 검토 대체물로 사용하지 마세요.
</Warning>

자동 모드는 계정이 다음 모든 요구 사항을 충족할 때만 사용 가능합니다:

* **플랜**: 모든 플랜.
* **조직**: Team 및 Enterprise에서 자동 모드는 기본적으로 사용 가능합니다. 관리자는 [관리 설정](/docs/ko/managed-settings)에서 `permissions.disableAutoMode`를 `"disable"`로 설정하여 조직에 대해 자동 모드를 끌 수 있습니다.
* **모델**: Anthropic API 및 [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws)에서 Claude Opus 4.6 이상, Sonnet 4.6 이상, 또는 [Fable 모델](/docs/ko/model-config#work-with-fable). Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry, 그리고 로그인한 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션에서는 Claude Sonnet 5, Opus 4.7 이상, 그리고 Fable 모델만 지원됩니다. Sonnet 4.5, Opus 4.5, Haiku, claude-3 모델을 포함한 이전 모델은 어떤 제공자에서도 지원되지 않습니다.
* **제공자**: Anthropic API, AWS의 Claude Platform, Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry, 그리고 로그인한 Claude 앱 게이트웨이 세션에서 기본적으로 사용 가능합니다.

Claude Code가 자동 모드를 사용할 수 없다고 보고하면 먼저 이러한 요구 사항을 확인하고 설정 파일이 [`disableAutoMode`](/docs/ko/settings-reference#disableautomode)를 설정하는지 확인하세요. Anthropic이 서버 측에서 자동 모드를 끄거나 서버가 계정에 대해 자동 모드를 거부했을 수도 있습니다. 두 답변 중 하나를 받은 세션은 세션이 끝날 때까지 자동 모드를 끈 상태로 유지하므로 나중에 새 세션을 시작하세요.

모델 이름을 지정하고 자동 모드가 작업의 안전성을 "결정할 수 없다"고 말하는 별도의 메시지는 분류기 요청이 실패했음을 의미합니다. 이 실패는 일반적으로 일시적이지만 Amazon Bedrock에서는 계정이 명명된 모델을 호출할 수 있을 때까지 반복될 수 있습니다. 원인과 해결 방법은 [오류 참조](/docs/ko/errors#auto-mode-cannot-determine-the-safety-of-an-action)를 참조하세요.

[설정](/docs/ko/settings-reference#all-settings)에서 `defaultMode: "auto"`를 설정했는데 터미널 세션이 오류 없이 수동 모드로 시작되면 설정이 `.claude/settings.json` 또는 `.claude/settings.local.json`에 있을 가능성이 높습니다. `auto`는 이러한 파일에서 적용되지 않습니다. `~/.claude/settings.json`으로 이동하세요. VS Code 확장이 시작한 대화의 경우 [권한 모드 전환](#switch-permission-modes) 대신 확장의 자체 목록을 확인하세요.

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Bedrock, Agent Platform 또는 Foundry의 자동 모드
</h3>

[Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai), [Microsoft Foundry](/docs/ko/microsoft-foundry), 그리고 로그인한 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션에서 자동 모드는 기본적으로 `Shift+Tab` 사이클에 나타납니다. 사이클에 나타나는 것이 세션이 시작되는 권한 모드를 변경하지는 않습니다. 이러한 제공자에서 터미널 세션은 [`defaultMode`](/docs/ko/settings-reference#permissions-defaultmode)로 시작하며, 이는 변경하지 않으면 수동이고, [VS Code 확장](/docs/ko/vs-code)의 대화는 `claudeCode.initialPermissionMode` 또는 확장에서 선택한 모드가 설정하지 않으면 수동으로 시작됩니다. 이러한 제공자에서는 Claude Sonnet 5, Opus 4.7 이상, 그리고 Fable 모델만 지원됩니다.

자동 모드를 기본 시작 권한 모드로 만들려면 사용자 또는 관리 설정에서 `"permissions": {"defaultMode": "auto"}`를 설정하세요. VS Code 확장이 시작한 세션에서는 모드 표시기에서 **자동**을 선택하세요. [권한 모드 전환](#switch-permission-modes)은 해당 선택을 능가하는 것을 다룹니다.

[`/doctor`](/docs/ko/commands#all-commands) 점검은 Anthropic API와 동일한 방식으로 이러한 제공자에서 사용자 설정 기본값을 제안합니다.

개발자가 자동 모드를 사용하지 못하도록 하려면 [관리 설정](/docs/ko/managed-settings)에서 `disableAutoMode`를 `"disable"`로 설정하세요. 이는 `Shift+Tab` 사이클에서 `auto`를 제거하고, `--permission-mode auto`로 시작한 세션은 수동으로 시작됩니다. 이미 자동 모드로 실행 중인 세션은 설정이 [관리자 배포 소스](/docs/ko/managed-settings#which-managed-source-claude-code-uses)에서 해당 세션에 도달할 때 자동 모드를 떠나고 `auto mode disabled by settings`를 표시합니다. v2.1.251 이전에는 실행 중인 세션이 끝날 때까지 자동 모드를 유지했습니다.

v2.1.158부터 v2.1.206까지 자동 모드는 `CLAUDE_CODE_ENABLE_AUTO_MODE=1`을 설정할 때까지 이러한 제공자에서 꺼져 있었고, Claude Code는 변수도 설정되지 않으면 이러한 제공자에서 `defaultMode: "auto"`를 무시했습니다. 변수는 호환성을 위해 여전히 허용되며 v2.1.207 이후로는 효과가 없습니다.

<h3 id="server-side-classifier-review">
  서버 측 분류기 검토
</h3>

자동 모드에서 Claude Code는 서버에 [결정 순서](#how-the-classifier-evaluates-actions)가 검토를 위해 보내는 작업을 확인하도록 요청할 수 있습니다. 이는 세션의 모델 요청의 일부로 자체 분류기 요청을 보내는 대신 수행됩니다. 이러한 세션은 다음을 요청합니다:

* **Anthropic API에 대한 직접 연결**: 대화형 터미널 세션에서 모든 claude.ai 플랜 및 Claude API를 사용하는 계정에서 Anthropic이 롤아웃할 때. Pro, Max, Team 플랜에서 Claude Code v2.1.271 이상이 필요하고, Enterprise 플랜 및 Claude API 계정에서 v2.1.278 이상이 필요합니다. v2.1.282부터 [기능 플래그를 가져오지 않는](/docs/ko/env-vars#features-that-need-feature-flag-fetching) 세션(예: 원격 분석을 끈 경우)은 모든 종류의 세션에서 기본적으로 서버에 요청합니다.
* **클라우드 제공자, LLM 게이트웨이 또는 프록시**: [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws), Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry에서, 그리고 `ANTHROPIC_BASE_URL`을 [LLM 게이트웨이 또는 프록시](/docs/ko/llm-gateway)로 가리킬 때마다, 플랜에 관계없이. 기본적으로 요청하려면 Claude Code v2.1.278 이상이 필요합니다.
* **로그인한 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션**: Claude Code v2.1.280 이상이 필요합니다

서버가 작업을 검토하는 경우 해당 판정이 결정합니다. 두 가지 다른 결과가 가능합니다:

* **서버가 세션을 검토하지 않음**: 응답이 검토 결과 없이 완료되거나 서버가 이 세션을 검토하지 않는다고 답합니다. 가장 일반적인 원인은 검토 요청이나 결과를 삭제하는 LLM 게이트웨이 또는 프록시이며, 아직 서버 측 검사가 없는 플랫폼, 지역 또는 자격 증명입니다. Claude Code는 자체 분류기 요청으로 폴백합니다. 이 폴백이 세션의 나머지 기간 동안 유지되면 [분류기 요청 요금에 대한 알림](/docs/ko/auto-mode-classifier-billing)을 표시합니다. 이러한 요청이 청구되는 계정에서.
* **서버가 작업에 대한 판정을 제공하지 않음**: Claude Code는 검토되지 않은 상태로 실행하는 대신 작업을 거부합니다. 모든 연결에서 이는 응답이 검토 결과가 도착하기 전에 끝나거나 결과가 Claude Code가 읽을 수 없는 형식으로 도착할 때 발생합니다. 응답을 단축하거나 결과를 다시 작성하는 LLM 게이트웨이 또는 프록시가 둘 다 발생할 수 있습니다. Anthropic API에 대한 직접 연결에서는 서버의 작업 검사가 실패할 때도 발생합니다(예: 시간 초과). [서버가 안전 판정을 반환하지 않음](/docs/ko/errors#the-server-returned-no-safety-verdict)은 거부 메시지, 거부가 반복될 때 발생하는 일, 그리고 해결 방법을 다룹니다.

서버에 요청하는 것을 건너뛰고 항상 Claude Code의 자체 분류기 요청을 사용하려면 [`CLAUDE_CODE_AUTO_MODE_SERVER=0`](/docs/ko/env-vars)을 설정하세요. Anthropic API에 대한 직접 연결에서 변수에는 Claude Code v2.1.281 이상이 필요합니다. 거기서 `1`로 설정하면 아직 서버 검토가 없는 세션(예: `-p` 또는 Agent SDK 세션)에서 서버 검토를 켭니다. `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`도 설정하지 않은 경우. `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`을 설정하고 `CLAUDE_CODE_AUTO_MODE_SERVER`를 설정하지 않으면 Claude Code도 서버에 요청하는 것을 중지합니다.

<h3 id="what-the-classifier-blocks-by-default">
  분류기가 기본적으로 차단하는 것
</h3>

분류기는 작업 디렉토리와 세션이 시작될 때 구성된 원격을 신뢰합니다. 세션 중에 `git remote add` 또는 `git remote set-url`로 추가되거나 다시 가리킨 원격은 신뢰되지 않으며, [신뢰할 수 있는 인프라를 구성](/docs/ko/auto-mode-config)할 때까지 다른 모든 것은 외부로 취급됩니다. v2.1.200 이전에는 세션 중에 추가된 원격도 신뢰되었습니다.

**기본적으로 차단됨**:

* `curl | bash`와 같은 코드 다운로드 및 실행
* 외부 엔드포인트로 민감한 데이터 전송
* 프로덕션 배포 및 마이그레이션
* 클라우드 스토리지의 대량 삭제
* IAM 또는 리포지토리 권한 부여
* 공유 인프라 수정
* 세션 전에 존재했던 파일을 돌이킬 수 없게 파괴
* 강제 푸시
* 실행될 때 비밀이나 민감한 데이터를 리포지토리 외부로 보내거나 배포가 노출하는 것을 확대할 변경 사항을 커밋하거나 푸시합니다. 이는 비밀을 아직 받지 않는 대상으로 전달하는 CI 워크플로우 또는 배포 구성, 비밀 저장소를 읽고 데이터를 보내는 스크립트 또는 설정 단계, 그리고 배포가 게시하는 것을 확대하는 구성 변경(예: 레지스트리, 가시성, 아티팩트 또는 소스맵 설정)을 포함합니다. 검사는 모든 분기에 적용되고, 리포지토리가 공개인 경우에도 적용되며, 커밋이나 푸시가 파이프라인을 트리거하는지 여부에 관계없이 커밋하거나 푸시할 때 발생합니다. 이를 해제하려면 커밋이나 푸시만이 아니라 실행 효과를 명명해야 합니다. v2.1.211 이전에는 이 검사가 기본 분기로 범위가 지정되었습니다. 거기로의 푸시는 민감한 콘텐츠, 요청한 것과 비교하여 숨겨지거나 잘못 설명된 변경 사항, 리포지토리 외부에서 이식된 콘텐츠, 또는 요청한 검토를 우회하는 콘텐츠를 전달할 때 차단되었습니다.
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop`, 또는 `git stash clear`. 분류기는 이것이 커밋되지 않은 변경 사항을 삭제할 것으로 가정합니다.
* 커밋이 이 세션에서 생성되지 않았을 때 `git commit --amend`
* v2.1.198부터 커밋이 이미 푸시되었을 때 `git commit --amend`. 메시지 전용 단어 변경은 차단되지 않습니다: 새로 스테이징된 것이 없는 `--amend -m`. Claude가 이 세션 중에 생성한 커밋에서
* `terraform destroy`, `pulumi destroy`, `cdk destroy`, 또는 `terragrunt destroy`, 그리고 리소스를 파괴하는 계획 적용

Claude Code v2.1.195 이상은 기본적으로 더 많은 범주를 차단합니다. 여러 개는 민감한 원격 대상 및 보호된 IaC 범위와 같은 [환경](/docs/ko/auto-mode-config#define-trusted-infrastructure) 항목에 따라 달라지며, 이를 구체적인 이름으로 좁힐 수 있습니다.

* 비밀 관리자에 쓰기, 또는 DNS 레코드 또는 TLS 인증서 변경
* 인간이 승인하지 않은 풀 요청 병합, Claude의 자체 풀 요청 승인, 또는 CI 검사 비활성화
* `atlantis apply` 또는 봇의 `/deploy` 또는 `/merge`와 같은 자동화에 대한 명령 자체인 댓글 게시
* 프로덕션 기능 플래그 토글, 램프 또는 삭제
* 보호된 IaC 범위에 인프라 변경 사항 적용, 또는 클러스터 노드 드레이닝 및 제거
* 레이블 선택기 또는 `--all`과 같이 다른 사용자의 작업을 포착하는 명명된 리소스를 초과하는 공유 컴퓨팅 클러스터에 대한 쓰기
* 모든 노드에서 실행되거나 클러스터 트래픽을 가로채는 Kubernetes 리소스 생성(예: DaemonSets 및 승인 웹훅)
* 민감한 원격 대상으로의 대화형 셸 또는 포트 포워드
* 로컬 서비스를 공개 인터넷에서 도달 가능하게 하는 터널 또는 역셸 열기
* 라이브 자격 증명 또는 토큰을 기록 또는 파일에 인쇄
* [환경](/docs/ko/auto-mode-config#define-trusted-infrastructure)에서 민감한 데이터 위치로 나열된 위치에 액세스하거나 해당 위치에서 데이터 복사. v2.1.198부터 이는 또한 한 위치에서 항목이 제외하는 대상으로 데이터를 보내는 것을 차단합니다.
* 내부 패키지 레지스트리를 공개 레지스트리로 우회하는 패키지 설치 라우팅. v2.1.198부터 이는 또한 환경에 나열되지 않은 경우에도 대화에서 Claude에게 내부 레지스트리 또는 미러가 존재한다고 말했을 때 적용됩니다.
* `--insecure`와 같은 안전 가드를 해제하는 플래그로 명령 실행
* `--dangerously-skip-permissions` 또는 `--no-sandbox`로 시작된 것과 같이 인간 승인이나 샌드박스 없이 실행되는 자율 에이전트 루프 시작. v2.1.198부터 이는 또한 `--yes-always`로 시작된 러너와 같이 격리 및 작업별 승인이 비활성화된 상태로 제3자 에이전트 또는 평가 하네스를 실행하는 것을 포함합니다.
* [Chrome의 Claude](/docs/ko/chrome) 브라우저 작업으로 페이지 콘텐츠, 쿠키 또는 자격 증명을 출처 외부로 보낼 수 있음

Claude Code v2.1.198 이상은 또한 기본적으로 다음을 차단합니다:

* 와일드카드, 글로브 또는 나이 필터가 아닌 특정 명명된 경로로 `/tmp`, `$TMPDIR` 또는 다른 공유 스크래치 또는 캐시 디렉토리의 파일 삭제
* 자신의 메시지가 해당 수신자에게 이러한 세부 정보를 승인하지 않았을 때 전송, 업로드, 게시 또는 다른 사람이나 공유 시스템에 작성된 콘텐츠에 민감한 세부 정보 포함. PR 및 이슈 본문, 커밋 메시지, 그리고 댓글은 리포지토리가 신뢰 경계 외부이거나 공개일 때 이러한 종류의 아웃바운드 콘텐츠로 계산됩니다. 조직의 자체 공개 리포지토리 포함; 내부 파일 경로, 코드명, 이메일 또는 계정 식별자와 같은 라이브 API 응답 데이터, 그리고 인프라 식별자는 민감한 세부 정보로 계산됩니다. PR, 이슈, 그리고 커밋 메시지 범위 지정에는 Claude Code v2.1.200 이상이 필요합니다. PR 또는 이슈 본문의 API 응답의 라이브 개인 데이터(예: 이메일 주소, 계정 또는 조직 식별자, 또는 사용 메트릭)에는 리포지토리의 가시성이나 신뢰 경계에 관계없이 해당 세부 정보와 수신자를 명명해야 합니다. 이 검사에는 Claude Code v2.1.203 이상이 필요합니다.
* Claude Code의 자체 tmux 창으로 키스트로크를 보내 자체 인터페이스를 구동합니다. 분류기는 이를 Claude가 자체 권한이나 감시를 변경하는 것으로 취급합니다.

Claude Code v2.1.200 이상은 또한 기본적으로 다음을 차단합니다:

* 인증, 액세스 제어, 입력 검증 또는 샌드박싱과 같은 보안 동작을 보호하는 테스트 또는 어설션을 주석 처리, 삭제 또는 강제 통과
* 세션에서 Claude가 생성하지 않은 상태 저장 리소스 삭제 또는 해제. 더 구체적인 삭제 규칙이 적용되지 않고 해당 리소스를 명명하지 않았을 때
* API 기본 URL, 프록시 엔드포인트, 웹훅 수신자 또는 레지스트리 미러를 작업에 맞지 않는 제3자 호스트로 다시 가리키기. `.env.example`과 같은 예제 파일 포함
* `git remote set-url` 또는 `git remote add`로 푸시가 가는 위치 변경. 새 원격을 명명하지 않은 경우
* 공개로 알려진 리포지토리로 비밀이나 개인 또는 신뢰할 수 있는 데이터 푸시, 또는 해당 리포지토리의 자체 작업의 일부가 아닌 기밀 자료를 거기로 푸시. 닷파일 리포지토리의 자체 주제는 개인 또는 신뢰할 수 있는 데이터의 유일한 예외이며, 개인 리포지토리에서 공개 표면에 도달하는 콘텐츠는 동일한 방식으로 차단됩니다. 두 개선 모두 Claude Code v2.1.203 이상이 필요합니다. v2.1.203 이전에는 개인 데이터가 기밀 자료와 함께 그룹화되었고 해당 리포지토리의 자체 작업의 일부가 아닐 때만 차단되었습니다. 리포지토리의 가시성이 확립되지 않으면 분류기는 그것만으로 차단하지 않습니다. 대신 다른 규칙에 대해 콘텐츠를 판단합니다.
* 다른 리포지토리 또는 조직에 대한 풀 요청 열기, `gh repo fork`로 포킹, 또는 제3자 리포지토리로 푸시. 해당 외부 대상을 명명하지 않은 경우

Claude Code v2.1.203 이상은 또한 기본적으로 다음을 차단합니다:

* 민감한 로컬 저장소의 콘텐츠, 또는 이름, 경로 또는 유형이 민감한 것으로 표시하는 파일의 콘텐츠가 커밋, 푸시, PR 또는 이슈 텍스트, gist 또는 붙여넣기, 또는 패키지 게시에 들어가기. 소스와 대상을 모두 명명하지 않은 경우. 세션 기록 및 대화 로그, SSH 키, 클라우드 자격 증명, 브라우저 프로필, 셸 기록과 같은 자격 증명 및 구성 점 폴더, 그리고 사용자 데이터 내보내기 모두 계산됩니다. 리포지토리가 개인이어도 이를 해제하지 않습니다.

Claude Code v2.1.205 이상은 또한 기본적으로 다음을 차단합니다:

* Claude Code 세션 기록, `~/.claude/projects/` 또는 구성된 구성 디렉토리 아래의 `.jsonl` 기록 파일에 쓰기. 셸 명령을 통해 직접 또는 간접적으로. 규칙은 또한 Claude Code가 자체 검사를 위해 각 기록 항목에 추가하는 메타데이터 줄을 포함합니다. 기록 읽기는 차단되지 않습니다.
* 대화에서 분류기가 보는 어디에도 할당되지 않은 셸 변수 또는 글로브가 루트인 재귀적 강제 삭제(예: `rm -rf "$VAR"` 또는 `Remove-Item -Recurse -Force $dir`). 값은 이전 명령 출력에서만 나왔으며, 분류기는 절대 받지 않으므로 분류기는 삭제 대상을 다른 삭제 규칙에 대해 확인할 수 없습니다. 삭제되는 정확한 경로를 명명하거나 Claude가 해결된 리터럴 경로가 명령에 작성된 상태로 삭제를 다시 실행할 때 블록이 해제됩니다. 분류기가 대상을 해결할 수 있는 삭제는 영향을 받지 않습니다. 베어 `*` 또는 `/*` 또는 `\*`로 끝나는 `Remove-Item` 대상은 분류기에 도달하지 않습니다: Claude Code는 [이를 직접 거부합니다](#remove-item-in-powershell).

Claude Code v2.1.257 이상은 또한 기본적으로 다음을 차단합니다:

* `169.254.169.254`와 같은 클라우드 인스턴스 메타데이터 엔드포인트에서 자격 증명 요청, 또는 머신의 자체 서비스 계정 또는 노드 ID로 클라우드, 클러스터 또는 레지스트리 호출을 명시적으로 인증
* 직접 요청이 아닌 다른 경로로 공개 호스트에 도달(예: 터널, 역셸, 또는 외부를 가리키도록 다시 작성된 리졸버 또는 프록시 구성)
* 호스트가 아닌 작업에 속하는 자격 증명 읽기(예: 노드 인증서 또는 노드의 컨테이너 레지스트리 인증)
* Claude가 시작하지 않은 형제 컨테이너, 포드 또는 VM에 연결 또는 스캔, 또는 아래의 노드

Claude Code가 이 중 하나를 허용하도록 의도된 곳에서 실행되면 `autoMode.environment`의 [호스트 포함 항목](/docs/ko/auto-mode-config#define-trusted-infrastructure)에서 해당 설정을 설명하세요.

Claude Code v2.1.261 이상은 또한 기본적으로 다음을 차단합니다:

* 메시지, PR 또는 이슈 텍스트, 문서 또는 링크가 열리거나 가져올 다른 곳에서 공개 붙여넣기, 다이어그램 또는 데이터 공유 서비스로의 링크 게시 또는 작성. URL 자체가 공유되는 콘텐츠를 전달할 때. 해당 서비스를 명명하지 않은 경우

**기본적으로 허용됨**:

* 작업 디렉토리의 로컬 파일 작업
* 잠금 파일 또는 매니페스트에 선언된 종속성 설치
* `.env` 읽기 및 자격 증명을 일치하는 API로 전송
* 읽기 전용 HTTP 요청
* 작업 중인 리포지토리의 모든 분기로 푸시. 기본 분기 포함. 이름이 배포 또는 게시 대상으로 표시하는 비기본 분기(예: `production` 또는 `gh-pages`)는 포함되지 않습니다: 분류기는 거기로의 푸시를 자체 조건에 따라 판단합니다. 푸시의 콘텐츠는 여전히 다른 규칙에 대해 확인되고, [`permissions.deny` 규칙](/docs/ko/permissions#manage-permissions)은 여전히 모든 모드에서 [작성된 대로](/docs/ko/permissions#bash-rule-limits) 푸시 명령을 차단할 수 있으며, 원격의 자체 분기 보호는 여전히 적용됩니다. v2.1.211 이전에는 시작한 분기, Claude가 생성한 분기, 그리고 기본 분기로의 일상적인 푸시만 기본적으로 허용되었으며, v2.1.203 이전에는 기본 분기로의 모든 직접 푸시가 차단되었습니다.

Claude Code v2.1.195 이상은 또한 기본적으로 다음을 허용합니다:

* 같은 세션에서 Claude가 이전에 생성한 정확한 작업 삭제
* 작업의 일부로 보안 관련 코드, 구성 및 위협 모델 읽기, 검토 또는 작성
* 같은 다중 에이전트 세션에서 함께 작업하는 에이전트 간의 메시지
* [`environment`](/docs/ko/auto-mode-config#define-trusted-infrastructure)에 나열한 신뢰할 수 있는 도메인, 버킷 및 서비스로 데이터 전송. 이는 동일한 인프라에 대한 파괴적이거나 자격 증명 작업이 아닌 데이터 흐름만 포함합니다.
* [Chrome의 Claude](/docs/ko/chrome) 신뢰할 수 있는 내부 도메인, localhost 또는 명명한 URL로의 탐색

샌드박스된 명령은 기본적으로 네트워크 액세스를 받지 않습니다. Claude는 명령이 필요한 호스트를 명령 자체에 명명하고, 분류기는 명령과 함께 이를 검토하며, 승인된 목록은 해당 명령만을 위해 이러한 호스트를 엽니다. [명령별 허용 도메인](/docs/ko/sandboxing#per-command-allowed-domains-in-auto-mode)은 목록이 열 수 있는 것과 열 수 없는 것, 그리고 명령이 나열되지 않은 호스트에 도달할 때 발생하는 일을 다룹니다.

`claude auto-mode defaults`를 실행하여 전체 규칙 목록을 JSON으로 인쇄하세요. 일상적인 작업이 차단되면 관리자는 `autoMode.environment` 설정을 통해 신뢰할 수 있는 리포지토리, 버킷 및 서비스를 추가할 수 있습니다: [자동 모드 구성](/docs/ko/auto-mode-config)을 참조하세요.

작업 중인 리포지토리의 모든 분기로 푸시하고 요청과 일치하는 풀 요청을 생성하는 것은 프롬프트 없이 실행됩니다. 푸시 또는 풀 요청이 [차단 목록](#what-the-classifier-blocks-by-default)에 해당하지 않는 한(예: 리포지토리를 떠나는 비밀이나 민감한 데이터, 또는 다른 리포지토리 또는 조직을 대상으로 하는 풀 요청). 자동 모드에 머물면서 이러한 명령 전에 인간 체크포인트를 요구하려면 `permissions.ask` 규칙을 추가하세요. 이는 명령 [작성된 대로](/docs/ko/permissions#bash-rule-limits)와 일치합니다: [일반적인 경계](/docs/ko/auto-mode-config#common-boundaries)를 참조하세요.

<h3 id="first-read-outside-the-working-directories">
  작업 디렉토리 외부의 첫 번째 읽기
</h3>

[`permissions.blockReadsOutsideWorkingDirectories`](/docs/ko/settings-reference#permissions-blockreadsoutsideworkingdirectories)가 꺼져 있는 동안 파일 읽기는 자동 모드에서 프롬프트 없이 실행됩니다. [작업 디렉토리](/docs/ko/permissions#working-directories) 외부의 경로에서도 포함. Claude가 처음으로 Read, Grep 또는 Glob 도구를 작업 디렉토리 외부의 경로에서 사용할 때 Claude Code는 이러한 읽기를 계속 허용할지 여부를 묻습니다.

프롬프트는 비대화형 `-p` 실행이나 백그라운드 세션에 나타나지 않습니다. 거기서의 읽기는 이전과 같이 실행됩니다.

답변에 관계없이 Claude는 계속 작업합니다:

* **계속 허용**: 읽기가 실행되고, 작업 디렉토리 외부의 이후 읽기는 이전과 같이 실행되며, Claude Code는 프롬프트가 다시 나타나지 않도록 답변을 기록합니다.
* **지금부터 차단**: 읽기가 거부되고, Claude Code는 [`permissions.blockReadsOutsideWorkingDirectories`](/docs/ko/settings-reference#permissions-blockreadsoutsideworkingdirectories)를 사용자 설정에서 `true`로 설정합니다. 이는 파일 도구가 모든 이후 세션 및 모든 권한 모드에서 이러한 읽기를 거부하게 합니다. 나중에 Claude가 이러한 경로를 읽도록 하려면 `/add-dir`로 디렉토리를 추가하거나 설정을 제거하세요.
* **다음에 다시 묻기**: 읽기가 거부되고, 작업 디렉토리 외부의 다음 읽기는 다시 프롬프트합니다.

<h3 id="boundaries-you-state-in-conversation">
  대화에서 명시한 경계
</h3>

분류기는 대화에서 명시한 경계를 차단 신호로 취급합니다. "푸시하지 마" 또는 "배포하기 전에 검토할 때까지 기다려"라고 Claude에게 말하면 분류기는 기본 규칙이 허용하더라도 일치하는 작업을 차단합니다. 경계는 이후 메시지에서 해제할 때까지 유효합니다. Claude의 자체 판단이 조건이 충족되었다는 것은 이를 해제하지 않습니다.

경계는 규칙으로 저장되지 않습니다. 분류기는 각 검사에서 기록을 다시 읽으므로 [컨텍스트 압축](/docs/ko/costs#reduce-token-usage)이 경계를 명시한 메시지를 제거하면 경계가 손실될 수 있습니다. 확실한 보장을 위해 [거부 규칙](/docs/ko/permissions#permission-rule-syntax)을 대신 추가하세요.

<h3 id="approvals-you-state-in-conversation">
  대화에서 명시한 승인
</h3>

Claude에게 차단된 작업이 허용된다고 말하면 분류기는 이를 승인으로 읽고 차단을 해제할 수 있습니다. 표현 방식이 작업 실행 여부와 승인이 도달하는 범위를 결정합니다:

* **작업과 세부 사항 명명**: 메시지는 작업과 위험하게 만드는 특정 사항(예: 강제 푸시의 분기)을 명명해야 합니다. 동사만 명명하는 것은 아무것도 해제하지 않으므로 "강제 푸시할 수 있습니다"는 차단을 제자리에 두고 있습니다.
* **한 작업을 포함하도록 예상**: 승인은 명명한 파괴적 작업을 포함하므로 이후 작업은 승인을 부여하지 않으면 다시 차단됩니다. 일상적인 패턴을 한 번에 하나씩 승인하는 것을 중지하려면 [`autoMode.allow`](/docs/ko/auto-mode-config#override-the-block-and-allow-rules)에 추가하세요.
* **일부 차단은 제자리에 유지됨**: [분류기의 우선 순위 순서](/docs/ko/auto-mode-config#override-the-block-and-allow-rules)는 승인이 도달할 수 있는 차단을 설정합니다. 이를 실행할 수 없는 단계를 실행하려면 [자동 모드를 떠나고](#switch-permission-modes) 권한 프롬프트에 답하세요.

<h3 id="when-auto-mode-falls-back">
  자동 모드가 폴백할 때
</h3>

자동 모드가 세션의 작업을 승인할 수 없을 때 발생하는 일은 경우에 따라 다릅니다:

* **차단된 작업**: Claude Code는 알림을 표시하고 `/permissions` 아래 **최근 거부됨** 탭에 작업을 나열합니다. 여기서 `r`을 눌러 수동 승인으로 다시 시도할 수 있습니다. 분류기가 [작업에 대한 판정을 생성하지 않을 때](/docs/ko/errors#auto-mode-cannot-determine-the-safety-of-an-action). 자동 모드와 별개인 안전 검사가 분류기의 자체 요청을 거부했거나 응답이 구문 분석되지 않았기 때문에 Claude Code는 알림이나 **최근 거부됨** 항목 없이 작업을 거부합니다.
* **반복된 차단**: 분류기가 작업을 연속으로 3번 또는 총 20번 차단하면 자동 모드가 일시 중지되고 Claude Code는 프롬프트를 다시 시작합니다. 프롬프트된 작업을 승인하면 자동 모드가 재개됩니다. 이러한 임계값은 구성할 수 없습니다. 허용된 작업은 연속 카운터를 재설정하는 반면 총 카운터는 세션에 대해 유지되고 자체 제한이 폴백을 트리거할 때만 재설정됩니다. Claude Code는 [자동 모드와 별개인 안전 검사가 분류기의 요청을 거부할 때](/docs/ko/errors#auto-mode-cannot-determine-the-safety-of-an-action) 거부를 어느 임계값에도 계산하지 않습니다. 연결된 항목은 Claude Code가 이러한 거부를 처리하는 방법을 다룹니다.
* **프롬프트할 수 없는 세션**: [`--permission-prompt-tool`](/docs/ko/cli-reference#cli-flags) 없는 [비대화형](/docs/ko/headless) `-p` 실행은 폴백할 프롬프트가 없습니다. 반복된 차단이 임계값에 도달하면 작업이 실행되지 않고 Claude는 계속 작업합니다. [자동 모드와 별개인 안전 검사가 분류기의 요청을 거부할 때](/docs/ko/errors#auto-mode-cannot-determine-the-safety-of-an-action)도 동일하게 적용됩니다. Claude Code는 어느 경우든 실행을 중지하지 않습니다.
* **서버에서 판정 없음**: [서버 측 분류기 검토](#server-side-classifier-review) 아래에서 Claude Code는 서버가 판정을 제공하지 않는 작업을 거부하고, 연속으로 판정이 없는 10개의 응답 후 턴을 중지합니다. [서버가 안전 판정을 반환하지 않음](/docs/ko/errors#the-server-returned-no-safety-verdict)을 참조하세요.
* **검사 중 모드 전환**: 분류기 검사가 보류 중일 때 권한 모드를 전환하면 Claude Code는 새 모드가 요청하지 않았을 판정을 버립니다. 대신 승인을 위해 프롬프트되거나 작업이 [`dontAsk` 모드](#allow-only-pre-approved-tools-with-dontask-mode)에서 자동 거부됩니다.

반복된 차단은 일반적으로 분류기가 인프라에 대한 컨텍스트를 놓치고 있음을 의미합니다. `/feedback`을 사용하여 거짓 양성을 보고하거나 관리자가 [신뢰할 수 있는 인프라를 구성](/docs/ko/auto-mode-config)하도록 하세요.

<span id="how-the-classifier-evaluates-actions" />

<AccordionGroup>
  <Accordion title="분류기가 작업을 평가하는 방법">
    각 작업은 고정된 결정 순서를 거칩니다. 첫 번째 일치 단계가 승리합니다:

    1. [허용, 요청 또는 거부 규칙](/docs/ko/permissions#manage-permissions)과 일치하는 작업은 다음 예외를 제외하고 즉시 해결됩니다:
       * [보호된 경로](#protected-paths)에 대한 쓰기는 허용 규칙이 일치할 때도 분류기로 라우팅됩니다. `rm` 및 `rmdir` 제거가 Claude Code v2.1.218 이상에서 [중요 경로](#critical-paths)를 대상으로 하는 경우도 마찬가지입니다.
       * [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구는 허용 규칙이 일치할 때도 직접 프롬프트합니다. 조직이 [요청으로 설정한](/docs/ko/mcp#organization-controls-on-connector-tools) 커넥터 도구도 해당 설정이 Claude Code에 도달하는 세션에서 마찬가지입니다.
       * [명령별 허용 도메인](/docs/ko/sandboxing#per-command-allowed-domains-in-auto-mode)을 전달하는 셸 명령도 허용 규칙이 일치할 때 분류기로 라우팅됩니다. 규칙이 명령을 승인하기 때문입니다. 호스트가 아닙니다.
       * `Bash(git push *)`와 같은 명령의 콘텐츠에 일치하는 규칙을 요청하면 권한 프롬프트로 폴백합니다.
    2. 읽기 전용 작업 및 작업 디렉토리의 파일 편집은 자동 승인됩니다. [보호된 경로](#protected-paths) 및 [작업 디렉토리 외부의 첫 번째 읽기](#first-read-outside-the-working-directories)에 대한 쓰기는 제외됩니다. 이는 프롬프트합니다.
       * [서버 측 분류기 검토](#server-side-classifier-review)가 있는 세션에서 읽기 전용 및 [샌드박스된](/docs/ko/sandboxing#sandbox-modes) 셸 명령은 해당 검토를 기다리고 플래그가 지정되면 차단됩니다.
    3. 다른 모든 것은 분류기로 이동합니다. 단계 1에서 직접 프롬프트하는 커넥터 도구 및 `requiresUserInteraction` MCP 도구는 분류기에 도달하지 않으므로 조직 필수 승인이나 동의 단계도 자동 승인되지 않습니다.
    4. 분류기가 차단하면 Claude는 이유를 받고 대안을 시도합니다. 대부분의 세션에서 이유는 분류기가 일치한 규칙(예: `[Data Exfiltration]`)을 명명하며, 서면 설명을 제공하지 않습니다. [거부 검토](/docs/ko/auto-mode-config#review-denials)를 참조하세요.

    자동 모드에 들어가면 임의의 코드 실행을 부여하는 광범위한 허용 규칙이 삭제됩니다:

    * 무조건 `Bash(*)` 또는 `PowerShell(*)`
    * `Bash(python*)`과 같은 와일드카드 인터프리터
    * 패키지 관리자 실행 명령
    * `Agent` 허용 규칙
    * [`Monitor`](/docs/ko/tools-reference#monitor-tool) 허용 규칙. Claude Code는 Monitor 명령을 셸을 통해 실행하기 때문입니다.

    `Bash(npm test)`와 같은 좁은 규칙은 유효합니다. Claude Code는 자동 모드를 떠날 때 삭제된 규칙을 복원합니다. v2.1.236 이전에는 Claude Code가 자동 모드에서 `Monitor` 허용 규칙을 유효하게 두었으므로 전체 도구와 일치하는 규칙이 분류기 검토 없이 Monitor 명령을 승인했습니다.

    Claude Code는 또한 `git reset --hard` 또는 `rm -rf`와 같이 커밋되지 않은 작업을 삭제할 명령 전에 `git status`를 자체적으로 실행하고 분류기에 스테이징된, 수정된 또는 추적되지 않은 작업이 있는지 표시합니다. Claude Code는 리포지토리의 git 구성이 `status.showUntrackedFiles=no`를 설정할 때도 해당 검사에서 추적되지 않은 파일을 보고합니다.

    Claude Code 자체가 보낸 분류기 요청에서 분류기는 사용자 메시지, 파일 읽기 및 검색과 같은 읽기 전용 조회 이외의 도구 호출, 그리고 CLAUDE.md 콘텐츠를 봅니다. 도구 결과는 이러한 요청에서 제거되므로 파일이나 웹 페이지의 악의적인 콘텐츠는 분류기를 직접 조작할 수 없습니다.

    [PostToolUse 훅의 `classifierContext` 필드](/docs/ko/hooks#annotate-a-result-for-the-auto-mode-classifier)로 호출의 결과에 주석을 달 수 있습니다. 분류기는 이를 애플리케이션 제공 컨텍스트로 읽습니다. 필드에는 Claude Code v2.1.236 이상이 필요합니다.

    별도의 서버 측 프로브는 들어오는 도구 결과를 스캔하고 Claude가 읽기 전에 의심스러운 콘텐츠에 플래그를 지정합니다. 이러한 계층이 함께 작동하는 방식에 대한 자세한 내용은 [자동 모드 발표](https://claude.com/blog/auto-mode) 및 [엔지니어링 심층 분석](https://www.anthropic.com/engineering/claude-code-auto-mode)을 참조하세요.
  </Accordion>

  <Accordion title="자동 모드가 서브에이전트를 처리하는 방법">
    분류기는 [서브에이전트](/docs/ko/sub-agents) 작업을 세 지점에서 확인합니다:

    1. 서브에이전트가 시작되기 전에 위임된 작업 설명이 평가되므로 위험해 보이는 작업은 생성 시간에 차단됩니다.
    2. 서브에이전트가 실행되는 동안 각 작업은 부모 세션과 동일한 규칙으로 분류기를 거치며, 서브에이전트의 frontmatter의 모든 `permissionMode`는 무시됩니다.
    3. 서브에이전트가 완료되면 분류기는 부모가 보고서를 읽기 전에 서브에이전트의 작업과 최종 보고서를 검토합니다. 분류기가 서브에이전트의 작업이나 보고서에 플래그를 지정하거나 별도의 API 안전 검사가 검토를 거부하면 보고서는 여전히 전달되며 보안 경고가 앞에 붙습니다. 분류기를 검토할 수 없으면 보고서는 서브에이전트의 작업을 확인한 후 작업하도록 주의하는 메모와 함께 도착합니다.
  </Accordion>

  <Accordion title="비용 및 지연">
    분류기는 기본적으로 `/model` 선택이 아닌 Claude Sonnet 5에서 실행됩니다. Anthropic이 서버 측에서 구성하는 분류기 모델이 해당 기본값보다 우선합니다. 세션의 모델이 Claude Sonnet 4.6이거나 [`availableModels`](/docs/ko/model-config#restrict-model-selection)이 Sonnet 5를 제외할 때 분류기는 세션의 모델 대신 실행되거나 세션이 [Fable 모델](/docs/ko/model-config#work-with-fable)에서 실행될 때 Opus 모델에서 실행됩니다. Anthropic API 이외의 제공자에서 해당 Opus 폴백은 제공자의 기본 Opus 모델입니다.

    Enterprise 플랜 및 Claude API를 사용하는 계정, [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws), Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry에서 분류기 호출은 토큰 사용량에 계산됩니다. 각 검사는 기록의 일부와 보류 중인 작업을 보내며 실행 전에 왕복을 추가합니다. 읽기 및 보호된 경로 외부의 작업 디렉토리 편집은 분류기를 건너뛰므로 오버헤드는 주로 셸 명령 및 네트워크 작업에서 나옵니다. 서버가 세션의 모델 요청의 일부로 작업을 검토하는 경우 계산할 별도의 분류기 호출이 없습니다. [서버 측 분류기 검토](#server-side-classifier-review)를 참조하세요.

    샌드박스된 네트워크 액세스는 명령별 분류기 요청을 추가하지 않습니다. 분류기는 [명령이 명명하는 호스트](/docs/ko/sandboxing#per-command-allowed-domains-in-auto-mode)를 명령과 함께 한 번의 검토로 판단하고, Claude Code는 분류기를 다시 호출하지 않고 승인된 목록에 대해 각 연결을 확인합니다.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  dontAsk 모드로 사전 승인된 도구만 허용
</h2>

`dontAsk` 모드를 설정하면 Claude Code는 그 외에 프롬프트를 표시할 모든 도구 호출을 자동으로 거부합니다. Claude는 여전히 Manual 모드에서 승인이 필요 없는 작업을 실행합니다. 예를 들어 작업 디렉터리 내의 파일 읽기 및 [읽기 전용 Bash 명령](/docs/ko/permissions#read-only-commands), `permissions.allow` 규칙과 일치하는 작업, 그리고 [PreToolUse 훅](/docs/ko/permissions#extend-permissions-with-hooks)으로 승인된 호출입니다. 이 모드는 CI 파이프라인이나 Claude가 수행할 수 있는 작업을 사전에 정의하는 제한된 환경에서 사용합니다. 세션은 입력을 기다리지 않습니다. 이 모드가 활성화되어 있는 동안 상태 표시줄에 `⏵⏵ don't ask on`이 표시됩니다.

Claude Code는 프롬프트를 표시하는 대신 명시적인 [`ask` 규칙](/docs/ko/permissions#manage-permissions)과 일치하는 호출을 거부합니다. 또한 `allow` 규칙이 일치하더라도 기본 제공 `AskUserQuestion` 도구를 거부하며, 해당 설정이 Claude Code에 도달하는 세션에서 조직이 [`ask`로 설정한](/docs/ko/mcp#organization-controls-on-connector-tools) 커넥터 도구도 동일하게 거부합니다. [`_meta["anthropic/requiresUserInteraction"]`](/docs/ko/mcp#require-approval-for-a-specific-tool)로 표시된 MCP 도구도 동일한 방식으로 거부합니다. 이는 승인 카드가 이 모드에서 수집하지 않는 답변이 필요하기 때문입니다. 이 기능은 Claude Code v2.1.199 이상이 필요합니다.

`rm` 및 `rmdir` 제거가 [중요 경로](#critical-paths)를 대상으로 하는 경우(예: `rm -rf /` 및 `rm -rf ~`), `allow` 규칙이 일치하거나 `PreToolUse` 훅이 허용하더라도 거부됩니다.

[클라우드 세션](/docs/ko/claude-code-on-the-web)은 `defaultMode: "dontAsk"`를 무시합니다. 자세한 내용은 [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode)를 참조하세요.

시작 시 플래그로 설정합니다:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  bypassPermissions 모드로 모든 확인 건너뛰기
</h2>

`bypassPermissions` 모드는 권한 프롬프트와 안전 확인을 비활성화하여 [보호된 경로](#protected-paths)에 대한 쓰기를 포함한 도구 호출이 즉시 실행되도록 합니다.

[작업 없음 모드가 자동 승인하는 작업](#actions-no-mode-auto-approves)은 이 모드에서도 계속 프롬프트를 표시합니다.

두 가지 [세션 간 메시징](/docs/ko/cross-session-messaging) 보안 조치는 이 모드에서, 그리고 권한 우회가 가능한 계획 모드 세션에서도 계속 적용됩니다:

* 이 머신을 넘어 다른 세션으로의 메시지에 대한 [`isolatePeerMachines`](/docs/ko/settings-reference#isolatepeermachines) 승인 프롬프트는 계속 나타납니다.
* [`crossSessionInbound`](/docs/ko/cross-session-messaging#control-inbound-messages) 값이 적용되지 않을 때, Claude Code는 다른 세션에서의 인바운드 메시지를 승인 대기 상태로 유지하며, 송신 세션이 권한 프롬프트도 우회 중임을 식별할 때만 묻지 않고 전달합니다. 메시지가 대기 중인 상태에서 권한 모드를 종료하면, Claude Code는 인바운드 규칙을 다시 적용하고 현재 수락하는 대기 중인 메시지를 전달합니다.

권한 우회가 가능한 대화형 터미널 세션에서, Claude Code는 [계획 모드의](#analyze-before-you-edit-with-plan-mode) 차단도 적용하지 않습니다. Claude는 여전히 편집 없이 계획하도록 지시받지만, 계획 중에 시도하는 파일 편집이나 셸 명령은 프롬프트 없이 실행됩니다. 명시적 [요청 규칙](/docs/ko/permissions#manage-permissions)과 [중요 경로](#critical-paths)를 대상으로 하는 `rm` 및 `rmdir` 제거는 계속 프롬프트를 표시합니다.

계획 모드는 Claude Code가 대화형 터미널 없이 실행되는 모든 곳에서 차단을 유지합니다. 여기에는 `-p`를 사용한 [비대화형 실행](/docs/ko/headless), [Agent SDK](/docs/ko/agent-sdk/permissions#plan-mode-plan) 세션, 그리고 [VS Code 확장](/docs/ko/vs-code)의 채팅 패널의 대화가 포함됩니다. 거기서 `--allow-dangerously-skip-permissions`는 나중에 `bypassPermissions`를 선택 가능하게 합니다.

<Warning>
  이 모드는 인터넷 접근이 없는 컨테이너, VM 또는 개발 컨테이너와 같은 격리된 환경에서만 사용하십시오. 여기서 Claude Code는 호스트 시스템에 손상을 줄 수 없습니다.
</Warning>

활성화되지 않은 상태에서 시작한 세션에서 `bypassPermissions`로 진입할 수 없습니다. [`permissions.defaultMode: "bypassPermissions"`](/docs/ko/settings-reference#permissions-defaultmode)로 시작할 때 활성화하거나 활성화 플래그를 사용하십시오:

```bash theme={null}
claude --permission-mode bypassPermissions
```

`--dangerously-skip-permissions` 플래그는 동등합니다.

Claude Code는 [`--restricted`](/docs/ko/cli-reference#cli-flags)로 시작한 세션에서 `bypassPermissions`를 거부합니다. `--restricted`는 Claude Code v2.1.248 이상이 필요합니다.

이 모드를 활성화하여 대화형 세션을 처음 시작할 때, Claude Code는 권한 확인 없이 수행된 작업에 대한 책임을 수락하도록 요청하는 경고 대화상자를 표시합니다. Claude Code는 사용자 설정에 승인을 저장하므로 대화상자는 한 번만 나타납니다. 거부하면 Claude Code가 종료됩니다. [비대화형 모드](/docs/ko/headless)에서는 대화상자가 표시되지 않으며, `--bg`로 시작한 [백그라운드 세션](/docs/ko/agent-view)은 대화형 세션에서 대화상자를 수락할 때까지 거부됩니다.

Linux 및 macOS에서, Claude Code는 root 또는 `sudo` 권한으로 실행할 때 이 모드에서 시작하기를 거부합니다:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

확인은 인식된 샌드박스 내에서 자동으로 건너뜁니다. 컨테이너에서 자율적으로 실행하려면, Claude Code를 비루트 사용자로 실행하는 [개발 컨테이너](/docs/ko/devcontainer) 구성을 사용하십시오.

[웹의 Claude Code](/docs/ko/claude-code-on-the-web)는 설정 파일의 `defaultMode: "bypassPermissions"` 또는 `"dontAsk"`를 적용하지 않으므로, 저장소의 체크인된 설정은 클라우드 세션을 권한 우회 모드로 시작할 수 없습니다. 설정은 자동으로 무시되고 세션은 모드 드롭다운에 표시된 권한 모드로 시작됩니다. [권한 모드 전환](#switch-permission-modes)에서 클라우드 세션이 제공하는 모드를 참조하십시오.

<Warning>
  `bypassPermissions`는 프롬프트 주입이나 의도하지 않은 작업에 대한 보호를 제공하지 않습니다. 훨씬 적은 권한 프롬프트로 백그라운드 안전 확인을 하려면 [자동 모드](#eliminate-prompts-with-auto-mode)를 대신 사용하십시오. 관리자는 [관리 설정](/docs/ko/managed-settings)에서 `permissions.disableBypassPermissionsMode`를 `"disable"`로 설정하여 이 모드를 차단할 수 있습니다.
</Warning>

<h2 id="protected-paths">
  보호된 경로
</h2>

`bypassPermissions` 모드와 [bypass permissions를 사용할 수 있는](#skip-all-checks-with-bypasspermissions-mode) plan 모드의 대화형 터미널 세션을 제외하고는 특정 경로에 대한 쓰기는 자동으로 승인되지 않습니다. 이는 저장소 상태와 Claude의 자체 설정이 실수로 손상되는 것을 방지합니다.

| 모드                       | 보호된 경로 쓰기                                                                                                                                                                                         |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`, `acceptEdits` | 프롬프트됨                                                                                                                                                                                             |
| `plan`                   | [bypass permissions를 사용할 수 있는](#skip-all-checks-with-bypasspermissions-mode) 대화형 터미널 세션에서 허용됨. 그렇지 않으면 계획 중에 [auto mode](#eliminate-prompts-with-auto-mode)를 사용할 수 있을 때 분류기로 라우팅되고, 그렇지 않으면 프롬프트됨 |
| `auto`                   | 분류기로 라우팅됨                                                                                                                                                                                         |
| `dontAsk`                | 거부됨                                                                                                                                                                                               |
| `bypassPermissions`      | 허용됨                                                                                                                                                                                               |

[`--restricted`](/docs/ko/cli-reference#cli-flags)로 시작된 세션에서(Claude Code v2.1.248 이상 필요) 분류기는 보호된 경로 쓰기를 승인할 수 없습니다.

설정 파일의 [`permissions.allow`](/docs/ko/permissions#manage-permissions) 규칙은 보호된 경로 쓰기를 사전에 승인하지 않습니다. 안전 확인은 Claude Code가 설정에서 allow 규칙을 평가하기 전에 실행되므로, `~/.claude/settings.json` 또는 `.claude/settings.json`의 `Edit(.claude/**)` 같은 항목은 위 표의 모드별 결과를 변경하지 않습니다. 프롬프트를 표시하는 모드에서는 `.claude/` 쓰기에 대한 프롬프트가 **Yes, and allow Claude to edit its own settings for this session**을 제공하며, 이는 해당 세션에서 나중의 `.claude/` 쓰기를 다시 프롬프트하지 않고 승인합니다.

보호된 디렉토리:

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, `.claude/worktrees` 제외 (Claude가 자신의 git worktrees를 저장하는 위치)

보호된 파일:

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="critical-paths">
  Critical paths
</h2>

Claude Code는 [`permissions.allow`](/docs/ko/permissions#manage-permissions) 규칙이나 `"allow"`를 반환하는 [`PreToolUse` hook](/docs/ko/permissions#extend-permissions-with-hooks)이 critical path를 대상으로 하는 `rm` 또는 `rmdir` 명령을 승인하도록 허용하지 않습니다. 다른 프롬프트를 건너뛰는 모드에서도 마찬가지입니다. 이 차단기는 모델 오류를 방지합니다. 일치하는 deny 규칙은 여전히 명령을 완전히 차단합니다.

대신 발생하는 일은 권한 모드에 따라 달라집니다:

| 모드                       | Claude Code가 critical-path 제거로 수행하는 작업                                                                                     |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | 승인을 요청합니다                                                                                                                  |
| `plan`                   | 승인을 요청합니다. [계획 중에 auto mode를 사용할 수 있고](#analyze-before-you-edit-with-plan-mode) bypass permissions를 사용할 수 없으면 대신 분류기로 보냅니다 |
| `auto`                   | [분류기](#eliminate-prompts-with-auto-mode)로 보냅니다                                                                             |
| `dontAsk`                | 거부합니다                                                                                                                      |
| `bypassPermissions`      | 승인을 요청합니다                                                                                                                  |

명시적 [ask 규칙](/docs/ko/permissions#manage-permissions)이 명령과 일치하면 Claude Code는 `auto` mode에서도 승인을 요청합니다. 승인을 요청하는 모드에서 [`PermissionRequest` hook](/docs/ko/hooks#permissionrequest)은 다른 프롬프트처럼 프롬프트에 답할 수 있습니다.

Claude Code는 `rm` 또는 `rmdir` 대상을 다음 중 하나일 때 critical path로 취급합니다:

* 파일시스템 루트
* 최상위 디렉토리, 즉 루트의 직접 자식(예: `/usr`, `/etc`, `/data`)
* 홈 디렉토리
* Windows 드라이브 루트 및 최상위 디렉토리(예: `C:\` 및 `C:\Windows`)
* 작업 디렉토리 및 부모
* 추가 작업 디렉토리 및 부모, 단 제거가 하나 아래의 glob일 때만(예: `rm -rf <dir>/*`). `rm -rf <dir>`은 디렉토리 자체에서 이 확인을 트리거하지 않습니다

Claude Code는 또한 `rm -rf "$DIR"/*`와 같이 셸 변수 직접 아래의 glob 또는 후행 슬래시를 critical-path 제거로 취급합니다. 변수가 비어 있으면 명령이 파일시스템 루트에서 제거가 되기 때문입니다.

이 변수 경우에 대한 프롬프트는 플래그된 `rm`의 이름을 지정하고 확인을 통과하도록 다시 작성하는 방법을 설명합니다:

* `$DIR`과 같은 변수의 경우 각 확장을 보호하여 변수가 설정되지 않았거나 비어 있을 때 셸이 오류로 중지되도록 합니다(예: `rm -rf "${DIR:?}"/*`). 또는 리터럴 경로를 사용합니다
* `$HOME`과 같이 일반적으로 설정되는 변수의 경우 리터럴 경로를 사용합니다

이러한 방식으로 모든 확장이 보호되는 제거는 critical-path 제거가 아니므로 `bypassPermissions` mode에서는 프롬프트 없이 실행됩니다.

`(...)` 내부의 서브셸, `{ ...; }` 내부의 brace group, `$(...)` 또는 백틱을 사용한 명령 치환, 또는 `<(...)` 내부의 프로세스 치환 내에 제거를 숨기는 것은 확인을 건너뛰지 않습니다. Claude Code는 `(rm -rf ~)` 또는 `echo "$(rm -rf ~)"`처럼 중첩된 형식 내부에 있든 같은 명령의 다른 곳에 있든 critical-path 제거를 찾습니다.

<h3 id="remove-item-in-powershell">
  PowerShell의 Remove-Item
</h3>

[PowerShell tool](/docs/ko/tools-reference#powershell-tool)을 활성화하면 Claude Code는 `Remove-Item`에 `rm` critical-path 목록과 별개의 자체 확인을 제공합니다. 결과는 대상에 따라 달라지며 첫 번째 일치하는 경우가 적용됩니다:

* **System paths**: 파일시스템 루트 및 최상위 디렉토리, 드라이브 루트 및 최상위 디렉토리, 홈 디렉토리. Claude Code는 모든 모드에서 묻지 않고 명령을 거부합니다.
* **Wildcards**: bare `*`, 또는 `/*` 또는 `\*`로 끝나는 모든 대상(예: `$dir/*`과 같은 셸 변수 아래의 glob). Claude Code는 [분류기](#eliminate-prompts-with-auto-mode)가 보기 전에 모든 모드에서 묻지 않고 명령을 거부합니다.
* **작업 디렉토리 또는 부모, `-Recurse` 포함**: Claude Code는 다른 승인이 필요한 명령처럼 명령을 취급하므로 승인을 요청하는 모드에서 묻고, `auto` mode에서 분류기로 보내고, `dontAsk` mode에서 거부합니다. `bypassPermissions` mode는 이 확인을 건너뜁니다.

<h2 id="see-also">
  참고 항목
</h2>

* [권한](/docs/ko/permissions): allow, ask, deny 규칙; 관리형 정책
* [auto mode 구성](/docs/ko/auto-mode-config): 분류기에 조직이 신뢰하는 인프라를 알립니다.
* [Hooks](/docs/ko/hooks): `PreToolUse` 및 `PermissionRequest` hooks를 통한 사용자 정의 권한 로직
* [보안](/docs/ko/security): safeguards 및 모범 사례
* [Sandboxing](/docs/ko/sandboxing): Bash 명령어에 대한 파일시스템 및 네트워크 격리
* [비대화형 모드](/docs/ko/headless): `-p` 플래그를 사용하여 Claude Code 실행
