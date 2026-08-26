> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 권한 모드 선택

> Claude가 파일을 편집하거나 명령을 실행하기 전에 승인을 요청하는지 여부를 제어합니다. CLI에서 Shift+Tab으로 모드를 순환하거나 VS Code, Desktop 및 claude.ai의 모드 선택기를 사용합니다.

Claude가 파일을 편집하거나 셸 명령을 실행하거나 네트워크 요청을 수행하려고 할 때, 작업을 승인하도록 일시 중지하고 요청합니다. 권한 모드는 해당 일시 중지가 얼마나 자주 발생하는지를 제어합니다. 선택한 모드는 세션의 흐름을 형성합니다. 수동 모드에서는 각 작업이 나타날 때마다 검토하게 되며, 더 느슨한 모드에서는 Claude가 더 긴 중단 없는 작업을 수행하고 완료되면 보고할 수 있습니다. 민감한 작업의 경우 더 많은 감시를 선택하거나, 방향을 신뢰할 때 중단을 줄입니다.

<h2 id="available-modes">
  사용 가능한 모드
</h2>

각 모드는 편의성과 감시 사이에서 서로 다른 트레이드오프를 제공합니다. 아래 표는 각 모드에서 Claude가 권한 프롬프트 없이 수행할 수 있는 작업을 보여줍니다.

| 모드                                                                  | 요청 없이 실행되는 작업                                              | 최적 사용 사례           |
| :------------------------------------------------------------------ | :--------------------------------------------------------- | :----------------- |
| `default`                                                           | 읽기만 가능                                                     | 시작하기, 민감한 작업       |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | 읽기, 파일 편집, 일반적인 파일시스템 명령어 (`mkdir`, `touch`, `mv`, `cp` 등) | 검토 중인 코드 반복 작업     |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | 읽기만 가능                                                     | 변경 전 코드베이스 탐색      |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | 백그라운드 안전 검사를 포함한 모든 작업                                     | 장시간 작업, 프롬프트 피로 감소 |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | 사전 승인된 도구만                                                 | 제한된 CI 및 스크립트      |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | 모든 작업                                                      | 격리된 컨테이너 및 VM만     |

모든 작업을 검토하는 모드는 CLI에서 **Manual**이라고 명명되며, `claude --help`에서, VS Code 및 JetBrains 확장 프로그램에서, 그리고 데스크톱 앱에서도 **Manual**이라고 불립니다. 해당 설정 값은 `default`이며, 이는 hooks 및 SDK 통합에서 사용하는 값입니다. CLI는 값을 입력하는 모든 곳에서 `manual`을 별칭으로 허용합니다. 예를 들어 `claude --permission-mode manual` 또는 `"defaultMode": "manual"`입니다. Manual 레이블과 `manual` 별칭은 Claude Code v2.1.200 이상이 필요합니다. 데스크톱 앱의 레이블은 CLI 버전에 따라 달라지지 않습니다.

`bypassPermissions`를 제외한 모든 모드에서 [보호된 경로](#protected-paths)에 대한 쓰기는 자동으로 승인되지 않으며, 이는 저장소 상태와 Claude의 자체 설정을 실수로 인한 손상으로부터 보호합니다.

모드는 기본 설정을 정합니다. 특정 도구를 사전 승인하거나 차단하기 위해 [권한 규칙](/docs/ko/permissions#manage-permissions)을 위에 계층화합니다. 거부 규칙, 명시적 요청 규칙, [커넥터 도구에 대한 조직 `ask` 설정](/docs/ko/mcp#organization-controls-on-connector-tools), 그리고 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool) 마커는 `bypassPermissions`를 포함한 모든 모드에서 적용됩니다. 허용 규칙은 다른 모든 것이 이미 승인되었기 때문에 해당 모드에서는 효과가 없습니다.

<h2 id="switch-permission-modes">
  권한 모드 전환
</h2>

세션 중간, 시작 시 또는 지속적인 기본값으로 모드를 전환할 수 있습니다. 모드는 채팅에서 Claude에게 요청하는 것이 아니라 이러한 컨트롤을 통해 설정됩니다. 아래에서 인터페이스를 선택하여 모드를 변경하는 방법을 확인하십시오.

<Tabs>
  <Tab title="CLI">
    **세션 중**: `Shift+Tab`을 눌러 `default` → `acceptEdits` → `plan`을 순환합니다. 현재 모드는 상태 표시줄에 나타납니다. 수동 모드인 `default`는 회색 `⏸ manual mode on` 배지를 표시합니다. v2.1.203 이전에는 상태 표시줄이 수동 모드에서 배지를 표시하지 않았습니다.

    모든 모드가 기본 순환에 포함되는 것은 아닙니다:

    * `auto`: 계정이 [자동 모드 요구사항](#eliminate-prompts-with-auto-mode)을 충족할 때 나타나며, 이 모드로 순환하면 확인 프롬프트 없이 모드가 전환됩니다
    * `bypassPermissions`: `--permission-mode bypassPermissions`, `--dangerously-skip-permissions` 또는 `--allow-dangerously-skip-permissions`로 시작한 후 나타나며, `--allow-` 변형은 활성화하지 않고 순환에 모드를 추가합니다
    * `dontAsk`: 순환에 나타나지 않으며, `--permission-mode dontAsk`로 설정합니다

    활성화된 선택적 모드는 `plan` 다음에 슬롯되며, `bypassPermissions`가 먼저이고 `auto`가 마지막입니다. 둘 다 활성화된 경우 `auto`로 가는 길에 `bypassPermissions`를 순환합니다.

    **시작 시**: 플래그로 모드를 전달합니다.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **기본값으로**: [설정](/docs/ko/settings#settings-files)에서 `defaultMode`를 설정합니다.

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "acceptEdits"
      }
    }
    ```

    동일한 `--permission-mode` 플래그는 [비대화형 실행](/docs/ko/headless)을 위해 `-p`와 함께 작동합니다.
  </Tab>

  <Tab title="VS Code">
    **세션 중**: 프롬프트 상자 하단의 모드 표시기를 클릭합니다.

    **기본값으로**: VS Code 설정에서 `claudeCode.initialPermissionMode`를 설정하거나 Claude Code 확장 설정 패널을 사용합니다.

    모드 표시기는 다음 레이블을 표시하며, 각각이 적용되는 모드에 매핑됩니다:

    | UI 레이블             | 모드                  |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    v2.1.205 이전에는 확장이 `plan`을 Plan mode로, `auto`를 Auto mode로 레이블했습니다.

    자동 모드는 계정이 [자동 모드 섹션](#eliminate-prompts-with-auto-mode)에 나열된 모든 요구사항을 충족할 때 모드 표시기에 나타납니다. `claudeCode.initialPermissionMode` 설정은 `auto`를 허용하지 않습니다. 기본적으로 자동 모드로 시작하려면 [사용자 설정](/docs/ko/settings#settings-files)에서 `defaultMode`를 설정하십시오. Claude Code는 프로젝트 및 로컬 설정에서 `defaultMode: "auto"`를 무시합니다.

    권한 무시는 모드 표시기에 나타나기 전에 확장 설정에서 **Allow dangerously skip permissions** 토글이 필요합니다.

    확장 관련 세부사항은 [VS Code 가이드](/docs/ko/vs-code)를 참조하십시오.
  </Tab>

  <Tab title="JetBrains">
    JetBrains 플러그인은 IDE 터미널에서 Claude Code를 실행하므로 모드 전환은 CLI와 동일하게 작동합니다: `Shift+Tab`을 눌러 순환하거나 시작할 때 `--permission-mode`를 전달합니다.
  </Tab>

  <Tab title="Desktop">
    **세션 중**: 전송 버튼 옆의 모드 선택기를 사용합니다. 모든 모드가 선택기에 나타나는 것은 아닙니다:

    * **Auto**: 계정이 [자동 모드 요구사항](#eliminate-prompts-with-auto-mode)을 충족할 때 나타납니다
    * **Bypass permissions**: Pro 및 Max 플랜에서 Desktop 설정의 **Allow bypass permissions mode** 토글이 필요하며, Team 및 Enterprise 플랜에서는 조직 정책이 대신 제어합니다

    데스크톱 관련 세부사항은 Desktop 가이드의 [권한 모드 선택](/docs/ko/desktop#choose-a-permission-mode)을 참조하십시오.

    **기본값으로**: [설정](/docs/ko/settings#settings-files)에서 `defaultMode`를 설정합니다. 데스크톱 앱은 CLI와 동일한 설정 파일을 읽고 새 로컬 세션에 모드를 적용합니다.

    모드 선택기에서 선택한 모드는 폴더별로 기억되며 해당 폴더에 대해 `defaultMode`보다 우선합니다. Plan은 예외입니다: 선택하면 현재 세션에만 적용됩니다.

    이 예제는 새 로컬 세션의 기본값으로 Plan 모드를 설정합니다:

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "plan"
      }
    }
    ```
  </Tab>

  <Tab title="Web and mobile">
    [claude.ai/code](https://claude.ai/code)의 모드 드롭다운 또는 모바일 앱의 프롬프트 상자 옆을 사용합니다. 권한 프롬프트는 승인을 위해 claude.ai에 나타납니다. 나타나는 모드는 세션이 실행되는 위치에 따라 달라집니다:

    * **Cloud sessions** on [Claude Code on the web](/docs/ko/claude-code-on-the-web): Accept edits, Plan, and Auto. Accept edits는 `default` 모드에 해당합니다: 클라우드 환경은 모드에 관계없이 파일 편집을 사전 승인하므로 드롭다운은 수동 대신 Accept edits를 표시합니다. 클라우드 세션은 여전히 설정의 `defaultMode: "acceptEdits"`를 준수합니다. 자동 모드는 조직이 허용하고 선택한 모델이 지원할 때만 나타납니다. 권한 무시는 사용할 수 없습니다.
    * **[Remote Control](/docs/ko/remote-control) sessions** on your local machine: Manual, Accept edits, and Plan. 앱에서 Auto 또는 Bypass permissions를 선택할 수 없습니다. 드롭다운은 터미널에서 설정된 모드를 포함하여 로컬 세션이 있는 모드를 표시하며, 앱 또는 터미널에서 모드가 변경될 때 업데이트됩니다. 한 가지 예외는 권한 무시입니다: 세션은 해당 모드를 claude.ai에 보고하지 않으므로 터미널에서 전환해도 드롭다운이 표시하는 내용이 변경되지 않습니다. v2.1.202 이전에는 `/remote-control` 또는 `claude --remote-control`로 연결된 세션이 모드를 전혀 보고하지 않았으므로 claude.ai 및 모바일 앱이 세션이 실제로 있지 않은 모드를 표시할 수 있었습니다. 불일치는 레이블에만 영향을 미쳤습니다: Claude Code는 세션의 실제 모드에서 권한 프롬프트를 생성했으며, 여전히 승인을 위해 앱에 나타났습니다.

    Remote Control의 경우 호스트를 시작할 때 시작 모드를 설정할 수도 있습니다:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  acceptEdits 모드로 파일 편집 자동 승인
</h2>

`acceptEdits` 모드를 사용하면 Claude가 프롬프트 없이 작업 디렉토리에서 파일을 생성하고 편집할 수 있습니다. 이 모드가 활성화되어 있는 동안 상태 표시줄에 `⏵⏵ accept edits on`이 표시됩니다.

파일 편집 외에도 `acceptEdits` 모드는 일반적인 파일시스템 Bash 명령어를 자동으로 승인합니다: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`. 이러한 명령어는 `LANG=C` 또는 `NO_COLOR=1`과 같은 안전한 환경 변수가 접두사로 붙거나 `timeout`, `nice`, `nohup`과 같은 프로세스 래퍼가 붙을 때도 자동으로 승인됩니다. 파일 편집과 마찬가지로 자동 승인은 작업 디렉토리 또는 `additionalDirectories` 내의 경로에만 적용됩니다. 해당 범위 외의 경로, [보호된 경로](#protected-paths)에 대한 쓰기, 그리고 [읽기 전용 명령어 집합](/docs/ko/permissions#read-only-commands)을 제외한 다른 모든 Bash 명령어는 여전히 프롬프트를 표시합니다.

[PowerShell 도구](/docs/ko/tools-reference#powershell-tool)가 활성화되어 있으면 `acceptEdits` 모드는 범위 내 경로에서 `Set-Content`, `Add-Content`, `Clear-Content`, `Remove-Item`과 이들의 일반적인 별칭도 자동으로 승인합니다. 동일한 범위 및 보호된 경로 규칙이 적용됩니다.

편집을 인라인으로 승인하는 대신 편집기에서 또는 `git diff`를 통해 변경 사항을 검토하려는 경우 `acceptEdits`를 사용하세요.

Manual 모드에서 `Shift+Tab`을 한 번 누르면 이 모드로 진입하거나 직접 시작할 수 있습니다:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  편집하기 전에 계획 모드로 분석하기
</h2>

계획 모드는 Claude가 변경 사항을 연구하고 제안하되 적용하지 않도록 지시합니다. Claude는 파일을 읽고 셸 명령을 실행하여 탐색한 후 계획을 작성하지만 소스를 편집하지 않습니다. 권한 프롬프트는 수동 모드와 동일하게 적용됩니다. [자동 모드](/docs/ko/auto-mode-config)를 사용할 수 있고 `useAutoModeDuringPlan`이 켜져 있는 경우(기본값)는 예외입니다. 자동 모드가 활성화되면 분류기는 검색 및 파일 읽기와 같은 읽기 전용 명령을 프롬프트 없이 승인합니다. 어느 쪽이든 편집은 계획을 승인할 때까지 차단된 상태로 유지됩니다.

`Shift+Tab`을 누르거나 단일 프롬프트 앞에 `/plan`을 붙여서 계획 모드에 진입합니다. CLI에서 계획 모드로 시작할 수도 있습니다.

```bash theme={null}
claude --permission-mode plan
```

`Shift+Tab`을 다시 눌러 계획을 승인하지 않고 계획 모드를 종료합니다.

<h3 id="review-and-approve-a-plan">
  계획 검토 및 승인
</h3>

계획이 준비되면 Claude가 이를 제시하고 진행 방법을 묻습니다. 해당 프롬프트에서 다음을 수행할 수 있습니다.

* 자동 모드로 승인 및 시작
* 편집 승인 및 수락
* 각 편집을 수동으로 검토하며 승인
* 피드백으로 계획 계속 진행
* [Ultraplan](/docs/ko/ultraplan)으로 브라우저 기반 검토를 위해 개선

계획을 승인하면 계획 모드가 종료되고 세션이 각 승인 옵션이 설명하는 권한 모드로 전환되므로 Claude가 편집을 시작합니다. 다시 계획하려면 `Shift+Tab`으로 계획 모드로 돌아가거나 다음 프롬프트 앞에 `/plan`을 붙입니다.

`Ctrl+G`를 눌러 제안된 계획을 기본 텍스트 편집기에서 열고 Claude가 진행하기 전에 직접 편집합니다. [`showClearContextOnPlanAccept`](/docs/ko/settings#available-settings)가 활성화되면 각 승인 옵션도 먼저 계획 컨텍스트를 지울 수 있는 옵션을 제공합니다.

계획을 수락하면 계획 콘텐츠에서 세션 이름을 자동으로 지정합니다. 단, `--name` 또는 `/rename`으로 이미 이름을 설정한 경우는 제외됩니다.

<h3 id="set-plan-mode-as-the-default">
  계획 모드를 기본값으로 설정
</h3>

프로젝트에 대해 계획 모드를 기본값으로 설정하려면 `.claude/settings.json`에서 `defaultMode`를 설정합니다.

```json theme={null}
{
  "permissions": {
    "defaultMode": "plan"
  }
}
```

<h2 id="eliminate-prompts-with-auto-mode">
  자동 모드로 권한 프롬프트 제거
</h2>

자동 모드를 사용하면 Claude가 일상적인 권한 프롬프트 없이 실행됩니다. 별도의 분류기 모델이 실행 전에 작업을 검토하여 요청을 초과하는 모든 항목, 인식되지 않은 인프라를 대상으로 하는 항목, 또는 Claude가 읽은 악의적인 콘텐츠로 인해 발생한 것으로 보이는 항목을 차단합니다. 명시적 [요청 규칙](/docs/ko/permissions#manage-permissions)은 여전히 프롬프트를 강제합니다.

파일 시스템 루트 또는 홈 디렉토리를 대상으로 하는 제거(예: `rm -rf /` 및 `rm -rf ~`)는 분류기로 이동하는 대신 승인을 위해 프롬프트합니다. 이 프롬프트는 또한 명령에 `$(...)` 또는 백틱을 사용한 명령 치환이나 `<(...)`를 사용한 프로세스 치환이 포함될 때 발생하며, 제거가 `echo "$(rm -rf ~)"`처럼 치환 내부에 있거나 같은 명령의 다른 곳에 있는지 여부입니다. v2.1.208 이전에는 이러한 형식을 포함하는 명령이 프롬프트하는 대신 분류기로 이동했습니다.

자동 모드는 또한 Claude가 명확한 질문을 위해 멈추지 않고 계속 작업하도록 유도하지만, Claude는 프롬프트나 스킬이 명시적으로 이를 필요로 할 때 여전히 질문합니다. 권한 프롬프트를 유지하면서 더 강력한 자율 동작을 원하면 [사전 예방적 출력 스타일](/docs/ko/output-styles)을 대신 설정하십시오.

<Warning>
  자동 모드는 권한 프롬프트를 줄이지만 안전을 보장하지 않습니다. 일반적인 방향을 신뢰하는 작업에 사용하고, 민감한 작업에 대한 검토를 대체하는 것으로 사용하지 마십시오.
</Warning>

자동 모드는 계정이 다음 모든 요구 사항을 충족할 때만 사용 가능합니다:

* **플랜**: 모든 플랜.
* **소유자**: Team 및 Enterprise에서 소유자는 사용자가 켤 수 있기 전에 [Claude Code 관리자 설정](https://claude.ai/admin-settings/claude-code)에서 이를 활성화해야 합니다. 관리자는 [관리 설정](/docs/ko/permissions#managed-settings)에서 `permissions.disableAutoMode`를 `"disable"`로 설정하여 자동 모드를 끌 수도 있습니다. 데스크톱 앱의 Code 탭의 경우 `disableAutoMode`는 조직 수준 제어이며 관리자 설정 토글은 적용되지 않습니다.
* **모델**: Anthropic API에서 Claude Opus 4.6 이상 또는 Sonnet 4.6 이상. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 및 로그인한 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션에서는 Claude Sonnet 5, Opus 4.7 및 Opus 4.8만 지원됩니다. Sonnet 4.5, Opus 4.5, Haiku 및 claude-3 모델을 포함한 이전 모델은 어떤 제공자에서도 지원되지 않습니다.
* **제공자**: Anthropic API, Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 및 로그인한 Claude 앱 게이트웨이 세션에서 기본적으로 사용 가능합니다. v2.1.158부터 v2.1.206까지 자동 모드는 `CLAUDE_CODE_ENABLE_AUTO_MODE=1`을 설정할 때까지 Anthropic API를 제외한 모든 제공자에서 꺼져 있었습니다. v2.1.207은 이 요구 사항을 제거했습니다.

Claude Code가 자동 모드를 사용할 수 없다고 보고하면 이러한 요구 사항 중 하나가 충족되지 않은 것입니다. 이는 일시적인 중단이 아닙니다. 모델의 이름을 지정하고 자동 모드가 작업의 안전성을 "결정할 수 없다"고 말하는 별도의 메시지는 일시적인 분류기 중단입니다. [오류 참조](/docs/ko/errors#auto-mode-cannot-determine-the-safety-of-an-action)를 참조하십시오.

[설정](/docs/ko/settings#available-settings)에서 `defaultMode: "auto"`를 설정했고 세션이 오류 없이 `default` 모드로 시작하면 설정이 `.claude/settings.json` 또는 `.claude/settings.local.json`에 있을 가능성이 높습니다. Claude Code v2.1.142 이상은 이러한 파일의 `auto`를 무시하므로 저장소가 자신에게 자동 모드를 부여할 수 없습니다. `~/.claude/settings.json`으로 이동하십시오.

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Bedrock, Agent Platform 또는 Foundry에서 자동 모드
</h3>

[Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai), [Microsoft Foundry](/docs/ko/microsoft-foundry) 및 로그인한 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션에서 자동 모드는 기본적으로 `Shift+Tab` 사이클에 나타납니다. 사이클에 나타나는 것은 세션이 시작되는 모드를 변경하지 않습니다. 세션은 여전히 [`defaultMode`](/docs/ko/settings#available-settings)에서 시작하며, 이는 변경하지 않으면 Manual입니다. 이러한 제공자에서는 Claude Sonnet 5, Opus 4.7 및 Opus 4.8만 지원됩니다.

자동 모드를 기본 시작 모드로 만들려면 사용자 또는 관리 설정에서 `"permissions": {"defaultMode": "auto"}`를 설정하십시오.

개발자가 자동 모드를 사용하지 못하도록 하려면 [관리 설정](/docs/ko/permissions#managed-settings)에서 `disableAutoMode`를 `"disable"`로 설정하십시오. 이렇게 하면 `Shift+Tab` 사이클에서 `auto`가 제거되고 시작 시 `--permission-mode auto`가 거부됩니다.

v2.1.158부터 v2.1.206까지 자동 모드는 `CLAUDE_CODE_ENABLE_AUTO_MODE=1`을 설정할 때까지 이러한 제공자에서 꺼져 있었으며, Claude Code는 변수도 설정되지 않으면 이러한 제공자에서 `defaultMode: "auto"`를 무시했습니다. 변수는 호환성을 위해 여전히 허용되며 v2.1.207 이상에서는 효과가 없습니다.

<h3 id="what-the-classifier-blocks-by-default">
  분류기가 기본적으로 차단하는 항목
</h3>

분류기는 작업 디렉토리와 세션이 시작될 때 구성된 원격을 신뢰합니다. 세션 중에 `git remote add` 또는 `git remote set-url`로 추가되거나 다시 지정된 원격은 신뢰되지 않으며, [신뢰할 수 있는 인프라를 구성](/docs/ko/auto-mode-config)할 때까지 다른 모든 것은 외부로 취급됩니다. v2.1.200 이전에는 세션 중에 추가된 원격도 신뢰되었습니다.

**기본적으로 차단됨**:

* `curl | bash`와 같은 코드 다운로드 및 실행
* 외부 엔드포인트로 민감한 데이터 전송
* 프로덕션 배포 및 마이그레이션
* 클라우드 스토리지의 대량 삭제
* IAM 또는 저장소 권한 부여
* 공유 인프라 수정
* 세션 전에 존재했던 파일을 되돌릴 수 없게 파괴
* Force push
* 비밀 또는 개인 또는 위탁받은 데이터와 같은 민감한 콘텐츠를 전달하는 저장소의 기본 분기로의 푸시, 요청한 것과 관련하여 숨겨지거나 잘못 설명된 변경 사항을 전달하는 푸시, 저장소 외부에서 포팅되거나 처음 읽은 콘텐츠를 전달하는 푸시, 또는 요청한 풀 요청, 검토 또는 확인을 우회하는 푸시. 기본 분기로의 일반 푸시는 자체적으로 차단되지 않으며, 플래그된 푸시를 지우려면 플래그된 콘텐츠 또는 우회된 검토의 이름을 지정해야 하며, 푸시만 지정하는 것이 아닙니다. 분류기는 한 계층입니다. [`permissions.deny` 규칙](/docs/ko/permissions#manage-permissions)은 모든 모드에서 적용되며 기본 분기로의 푸시를 완전히 차단할 수 있으며, 원격의 자체 분기 보호는 여전히 적용됩니다. v2.1.203 이전에는 기본 분기로의 직접 푸시가 차단되었습니다.
* 분류기가 커밋되지 않은 변경 사항을 버릴 것으로 가정하는 `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop` 또는 `git stash clear`
* 이 세션에서 생성되지 않은 HEAD의 커밋에 대한 `git commit --amend`
* v2.1.198부터 HEAD의 커밋이 이미 푸시된 경우 `git commit --amend`. 메시지 전용 단어 변경은 차단되지 않습니다. 이 세션 중에 Claude가 생성한 커밋에서 새로 스테이징된 것이 없는 `--amend -m`
* `terraform destroy`, `pulumi destroy`, `cdk destroy` 또는 `terragrunt destroy`, 그리고 리소스를 파괴하는 계획 적용

Claude Code v2.1.195 이상은 기본적으로 더 많은 범주를 차단합니다. 여러 개는 민감한 원격 대상 및 보호된 IaC 범위와 같은 [환경](/docs/ko/auto-mode-config#define-trusted-infrastructure) 항목에 따라 달라지며, 이를 구체적인 이름으로 좁힐 수 있습니다.

* 비밀 관리자에 쓰기, 또는 DNS 레코드 또는 TLS 인증서 변경
* 인간이 승인하지 않은 풀 요청 병합, Claude의 자체 풀 요청 승인, 또는 CI 확인 비활성화
* `atlantis apply` 또는 봇의 `/deploy` 또는 `/merge`와 같은 자동화에 대한 명령 자체인 댓글 게시
* 프로덕션 기능 플래그 토글, 램핑 또는 삭제
* 보호된 IaC 범위에 인프라 변경 사항 적용, 또는 클러스터 노드 드레이닝 및 제거
* 다른 사용자의 작업까지 포착하는 레이블 선택기 또는 `--all`과 같이, 직접 지정한 리소스 범위를 넘어서는 공유 컴퓨팅 클러스터에 쓰기
* 모든 노드에서 실행되거나 클러스터 트래픽을 가로채는 DaemonSets 및 admission webhooks와 같은 Kubernetes 리소스 생성
* 민감한 원격 대상으로의 대화형 셸 또는 포트 포워드
* 로컬 서비스를 공개 인터넷에서 도달 가능하게 하는 터널 또는 역셸 열기
* 라이브 자격 증명 또는 토큰을 기록 또는 파일로 인쇄
* [환경](/docs/ko/auto-mode-config#define-trusted-infrastructure)에서 민감한 데이터 위치로 나열된 위치에 액세스하거나 데이터를 복사합니다. v2.1.198부터 이는 또한 항목이 제외하는 대상으로 한 위치에서 데이터를 전송하는 것을 차단합니다.
* 내부 패키지 레지스트리를 공개 레지스트리로 우회하는 패키지 설치. v2.1.198부터 이는 환경에 나열된 경우뿐만 아니라 대화에서 Claude에게 내부 레지스트리 또는 미러가 존재한다고 말한 경우에도 적용됩니다.
* `--insecure`와 같은 안전 가드를 해제하는 플래그로 명령 실행
* `--dangerously-skip-permissions` 또는 `--no-sandbox`로 시작된 것과 같이 인간 승인 또는 샌드박스 없이 실행되는 자율 에이전트 루프 시작. v2.1.198부터 이는 또한 `--yes-always`로 시작된 러너와 같이 격리 및 작업별 승인이 비활성화된 제3자 에이전트 또는 평가 하네스를 실행하는 것을 포함합니다.
* 페이지 콘텐츠, 쿠키 또는 자격 증명을 원본 외부로 보낼 수 있는 [Chrome의 Claude](/docs/ko/chrome) 브라우저 작업

Claude Code v2.1.198 이상도 기본적으로 다음을 차단합니다:

* 특정 명명된 경로가 아닌 와일드카드, glob 또는 나이 필터로 `/tmp`, `$TMPDIR` 또는 다른 공유 스크래치 또는 캐시 디렉토리의 파일 삭제
* 자신의 메시지가 해당 수신자에게 이러한 세부 정보를 승인하지 않은 경우 다른 사람 또는 공유 시스템으로 전송, 업로드, 게시 또는 작성된 콘텐츠에 민감한 세부 정보 포함. PR 및 이슈 본문, 커밋 메시지 및 댓글은 저장소가 신뢰 경계 외부이거나 공개인 경우 이러한 종류의 아웃바운드 콘텐츠로 계산되며, 조직의 자체 공개 저장소 포함; 내부 파일 경로, 코드명, 이메일 또는 계정 식별자와 같은 라이브 API 응답 데이터 및 인프라 식별자는 민감한 세부 정보로 계산됩니다. PR, 이슈 및 커밋 메시지 범위 지정은 Claude Code v2.1.200 이상이 필요합니다. PR 또는 이슈 본문의 API 응답의 라이브 개인 데이터(예: 이메일 주소, 계정 또는 조직 식별자 또는 사용 메트릭)는 저장소의 가시성 또는 신뢰 경계에 관계없이 이러한 세부 정보와 수신자의 이름을 지정해야 합니다. 이 확인은 Claude Code v2.1.203 이상이 필요합니다.
* Claude Code의 자체 tmux 창으로 키스트로크를 전송하여 자체 인터페이스를 구동합니다. 분류기는 이를 Claude가 자체 권한 또는 감시를 변경하는 것으로 취급합니다.

Claude Code v2.1.200 이상도 기본적으로 다음을 차단합니다:

* 인증, 액세스 제어, 입력 검증 또는 샌드박싱과 같은 보안 동작을 보호하는 테스트 또는 어설션 주석 처리, 삭제 또는 강제 통과
* Claude가 세션에서 생성하지 않은 상태 저장 리소스 삭제 또는 해제, 더 구체적인 삭제 규칙이 적용되지 않고 해당 리소스의 이름을 지정하지 않은 경우
* API 기본 URL, 프록시 엔드포인트, 웹훅 수신자 또는 레지스트리 미러를 작업에 맞지 않는 제3자 호스트로 다시 지정(`.env.example`과 같은 예제 파일 포함)
* `git remote set-url` 또는 `git remote add`로 푸시가 가는 위치 변경, 새 원격의 이름을 지정하지 않은 경우
* 공개로 알려진 저장소로 비밀 또는 개인 또는 위탁받은 데이터 푸시, 또는 해당 저장소의 자체 작업의 일부가 아닌 기밀 자료 푸시. dotfiles 저장소의 자체 주제는 개인 또는 위탁받은 데이터의 유일한 예외이며, 개인 저장소에서 공개 표면에 도달하는 콘텐츠는 동일한 방식으로 차단됩니다. 두 개선 사항 모두 Claude Code v2.1.203 이상이 필요합니다. v2.1.203 이전에는 개인 데이터가 기밀 자료와 함께 그룹화되었으며 해당 저장소의 자체 작업의 일부가 아닌 경우에만 차단되었습니다. 저장소의 가시성이 설정되지 않은 경우 분류기는 단독으로 차단하지 않습니다. 대신 다른 규칙에 대해 콘텐츠를 판단합니다.
* 다른 저장소 또는 조직에 대한 풀 요청 열기, `gh repo fork`로 포킹, 또는 제3자 저장소로 푸시, 해당 외부 대상의 이름을 지정하지 않은 경우

Claude Code v2.1.203 이상도 기본적으로 다음을 차단합니다:

* 민감한 로컬 저장소의 콘텐츠, 또는 이름, 경로 또는 유형이 민감한 것으로 표시된 파일의 콘텐츠가 커밋, 푸시, PR 또는 이슈 텍스트, gist 또는 붙여넣기 또는 패키지 게시에 들어가는 경우, 소스와 대상 모두의 이름을 지정하지 않은 경우. 세션 기록 및 대화 로그, SSH 키, 클라우드 자격 증명, 브라우저 프로필 및 셸 기록과 같은 자격 증명 및 구성 점 폴더, 그리고 사용자 데이터 내보내기는 모두 계산되며, 저장소가 비공개라는 것이 이를 지우지 않습니다.

Claude Code v2.1.205 이상도 기본적으로 다음을 차단합니다:

* Claude Code 세션 기록, `~/.claude/projects/` 또는 구성된 구성 디렉토리 아래의 `.jsonl` 기록 파일에 직접 또는 셸 명령을 통해 쓰기. 규칙은 또한 Claude Code가 자체 확인을 위해 각 기록 항목에 추가하는 메타데이터 줄을 포함합니다. 기록은 Claude Code가 작성하는 세션 상태이며 작업 파일이 아니며, 변조된 항목은 세션을 재개하면 모든 나중 확인에 도달하므로 자동 모드는 심층 방어로 이러한 쓰기를 차단합니다. 기록 읽기는 차단되지 않습니다.
* `rm -rf "$VAR"` 또는 `Remove-Item -Recurse -Force $dir`과 같은 재귀적 강제 삭제, 대상이 분류기가 보는 대화의 어디에도 할당되지 않은 셸 변수이거나 하나에 루트된 glob인 경우. 값은 분류기가 절대 받지 않는 이전 명령 출력에서만 나왔으므로 분류기는 삭제를 다른 삭제 규칙에 대해 확인할 수 없습니다. 분류기는 설계상 명령 출력이 아닌 대화를 읽으므로, 대상을 추측하는 대신 호출을 차단합니다. 삭제되는 정확한 경로를 직접 이름으로 지정하거나 Claude가 확인된 리터럴 경로를 명령에 작성하여 삭제를 다시 실행하면 블록이 지워집니다. 분류기가 대상을 해결할 수 있는 삭제는 영향을 받지 않습니다.

**기본적으로 허용됨**:

* 작업 디렉토리의 로컬 파일 작업
* 잠금 파일 또는 매니페스트에 선언된 종속성 설치
* `.env` 읽기 및 자격 증명을 일치하는 API로 전송
* 읽기 전용 HTTP 요청
* 시작한 분기 또는 Claude가 생성한 분기로 푸시
* 저장소 기본 분기로의 일상적인 푸시. v2.1.203 이전에는 기본 분기로의 직접 푸시가 차단되었습니다.

Claude Code v2.1.195 이상도 기본적으로 다음을 허용합니다:

* 같은 세션에서 Claude가 이전에 생성한 정확한 작업 삭제
* 작업의 일부로 보안 관련 코드, 구성 및 위협 모델 읽기, 검토 또는 작성
* 같은 다중 에이전트 세션에서 함께 작업하는 에이전트 간의 메시지
* [`environment`](/docs/ko/auto-mode-config#define-trusted-infrastructure)에 나열한 신뢰할 수 있는 도메인, 버킷 및 서비스로 데이터 전송. 이는 동일한 인프라에 대한 파괴적 또는 자격 증명 작업이 아닌 데이터 흐름만 포함합니다.
* [Chrome의 Claude](/docs/ko/chrome) 신뢰할 수 있는 내부 도메인, localhost 또는 명명한 URL로 탐색

샌드박스 네트워크 액세스 요청은 기본적으로 허용되는 대신 분류기를 통해 라우팅됩니다. v2.1.198부터 분류기는 모든 연결에서 다시 실행하는 대신 네트워크 호스트 및 포트에 대한 판정을 재사용합니다:

* 허용은 새 콘텐츠가 대화에 들어올 때까지 재사용되며, 이 시점에서 해당 호스트가 다시 확인됩니다.
* 대화형 CLI에서 거부는 턴이 끝날 때 삭제됩니다.
* [비대화형 모드](/docs/ko/headless) 및 Agent SDK 세션에는 턴 경계가 없으므로 거부는 실행의 나머지 부분에 대해 재사용됩니다.
* 권한 모드 또는 규칙을 변경하면 캐시된 모든 판정이 삭제됩니다.

`claude auto-mode defaults`를 실행하여 전체 규칙 목록을 확인하십시오. 일상적인 작업이 차단되면 관리자는 `autoMode.environment` 설정을 통해 신뢰할 수 있는 저장소, 버킷 및 서비스를 추가할 수 있습니다. [자동 모드 구성](/docs/ko/auto-mode-config)을 참조하십시오.

작업 분기로 푸시하기, 저장소 기본 분기로의 일상적인 푸시 만들기, 요청과 일치하는 풀 요청 생성은 모두 프롬프트 없이 실행됩니다. 분류기는 force push 또는 설정한 검토를 우회하는 콘텐츠와 같은 위험을 전달하는 푸시만 차단합니다. 자동 모드에 머물면서 이러한 작업 전에 인간 체크포인트를 요구하려면 `permissions.ask` 규칙을 추가하십시오. [일반적인 경계](/docs/ko/auto-mode-config#common-boundaries)를 참조하십시오.

<h3 id="boundaries-you-state-in-conversation">
  대화에서 명시한 경계
</h3>

분류기는 대화에서 명시한 경계를 차단 신호로 취급합니다. Claude에게 "푸시하지 마" 또는 "배포하기 전에 검토할 때까지 기다려"라고 말하면 분류기는 기본 규칙이 허용하더라도 일치하는 작업을 차단합니다. 경계는 나중 메시지에서 해제할 때까지 유효합니다. Claude의 조건이 충족되었다는 자체 판단은 이를 해제하지 않습니다.

경계는 규칙으로 저장되지 않습니다. 분류기는 각 확인에서 기록을 다시 읽으므로 [컨텍스트 압축](/docs/ko/costs#reduce-token-usage)이 경계를 명시한 메시지를 제거하면 경계가 손실될 수 있습니다. 하드 보장을 위해 [거부 규칙](/docs/ko/permissions#permission-rule-syntax)을 대신 추가하십시오.

<h3 id="when-auto-mode-falls-back">
  자동 모드가 폴백할 때
</h3>

거부된 각 작업은 알림을 표시하고 `/permissions`의 최근 거부 탭에 나타나며, 여기서 `r`을 눌러 수동 승인으로 다시 시도할 수 있습니다.

분류기가 작업을 연속으로 3번 또는 총 20번 차단하면 자동 모드가 일시 중지되고 Claude Code가 프롬프트를 다시 시작합니다. 프롬프트된 작업을 승인하면 자동 모드가 재개됩니다. 이러한 임계값은 구성할 수 없습니다. 허용된 모든 작업은 연속 카운터를 재설정하는 반면 총 카운터는 세션에 대해 유지되고 자체 제한이 폴백을 트리거할 때만 재설정됩니다.

[비대화형 모드](/docs/ko/headless)에서 `-p` 플래그를 사용하면 프롬프트할 사용자가 없으므로 반복된 블록이 세션을 중단합니다.

반복된 블록은 일반적으로 분류기가 인프라에 대한 컨텍스트를 놓치고 있음을 의미합니다. `/feedback`을 사용하여 거짓 양성을 보고하거나 관리자가 [신뢰할 수 있는 인프라를 구성](/docs/ko/auto-mode-config)하도록 하십시오.

<AccordionGroup>
  <Accordion title="분류기가 작업을 평가하는 방법">
    각 작업은 고정된 결정 순서를 거칩니다. 첫 번째 일치하는 단계가 승리합니다:

    1. [허용, 요청 또는 거부 규칙](/docs/ko/permissions#manage-permissions)과 일치하는 작업은 [보호된 경로](#protected-paths)에 대한 쓰기를 제외하고 즉시 해결되며, 이는 허용 규칙이 일치하더라도 분류기로 라우팅됩니다. [조직이 `ask`로 설정한](/docs/ko/mcp#organization-controls-on-connector-tools) 커넥터 도구 및 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구는 허용 규칙이 일치하더라도 직접 프롬프트합니다. 콘텐츠 범위 요청 규칙은 권한 프롬프트로 폴백합니다.
    2. 읽기 전용 작업 및 작업 디렉토리의 파일 편집은 [보호된 경로](#protected-paths)에 대한 쓰기를 제외하고 자동 승인됩니다.
    3. 다른 모든 것은 분류기로 이동합니다. [조직이 `ask`로 설정한](/docs/ko/mcp#organization-controls-on-connector-tools) 커넥터 도구는 분류기를 건너뛰고 직접 프롬프트하므로 조직 필수 승인은 자동 승인되지 않습니다. v2.1.199부터 [`_meta["anthropic/requiresUserInteraction"]`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구도 분류기를 건너뛰고 직접 프롬프트하므로 동의 단계는 도구 작성자를 대신하여 자동 승인되지 않습니다.
    4. 분류기가 차단하면 Claude는 이유를 받고 대안을 시도합니다.

    자동 모드에 들어가면 임의의 코드 실행을 부여하는 광범위한 허용 규칙이 삭제됩니다:

    * 무조건 `Bash(*)` 또는 `PowerShell(*)`
    * `Bash(python*)`과 같은 와일드카드 인터프리터
    * 패키지 관리자 실행 명령
    * `Agent` 허용 규칙

    `Bash(npm test)`와 같은 좁은 규칙은 유지됩니다. 삭제된 규칙은 자동 모드를 떠날 때 복원됩니다.

    분류기는 사용자 메시지, 도구 호출 및 CLAUDE.md 콘텐츠를 봅니다. 도구 결과는 제거되므로 파일 또는 웹 페이지의 악의적인 콘텐츠는 직접 조작할 수 없습니다. 별도의 서버 측 프로브가 들어오는 도구 결과를 스캔하고 Claude가 읽기 전에 의심스러운 콘텐츠에 플래그를 지정합니다. 이러한 계층이 함께 작동하는 방식에 대한 자세한 내용은 [자동 모드 발표](https://claude.com/blog/auto-mode) 및 [엔지니어링 심층 분석](https://www.anthropic.com/engineering/claude-code-auto-mode)을 참조하십시오.
  </Accordion>

  <Accordion title="자동 모드가 하위 에이전트를 처리하는 방법">
    분류기는 [하위 에이전트](/docs/ko/sub-agents) 작업을 세 지점에서 확인합니다:

    1. 하위 에이전트가 시작되기 전에 위임된 작업 설명이 평가되므로 위험해 보이는 작업은 생성 시 차단됩니다.
    2. 하위 에이전트가 실행되는 동안 각 작업은 부모 세션과 동일한 규칙으로 분류기를 통과하며, 하위 에이전트의 frontmatter의 모든 `permissionMode`는 무시됩니다.
    3. 하위 에이전트가 완료되면 분류기는 전체 작업 기록을 검토합니다. 해당 반환 확인이 우려 사항에 플래그를 지정하면 보안 경고가 하위 에이전트의 결과에 앞에 붙습니다.

    1단계는 Claude Code v2.1.178 이상이 필요합니다. 이전 버전은 2단계와 3단계에서 분류기를 적용했지만 하위 에이전트가 시작되기 전에 작업 설명을 평가하지 않았습니다.
  </Accordion>

  <Accordion title="비용 및 지연 시간">
    분류기는 `/model` 선택과 독립적인 서버 구성 모델에서 실행되므로 모델을 전환해도 분류기 가용성이 변경되지 않습니다. 분류기 호출은 토큰 사용량에 포함됩니다. 각 확인은 기록의 일부와 보류 중인 작업을 전송하여 실행 전에 왕복을 추가합니다. 보호된 경로 외부의 읽기 및 작업 디렉토리 편집은 분류기를 건너뛰므로 오버헤드는 주로 셸 명령 및 네트워크 작업에서 발생합니다. {{/* min-version: 2.1.198 */}}v2.1.198부터 샌드박스 네트워크 판정은 모든 연결에서 다시 분류되는 대신 호스트 및 포트에 대해 재사용되므로 동일한 호스트로의 반복된 연결은 각각 확인을 추가하지 않습니다. [분류기가 기본적으로 차단하는 항목](#what-the-classifier-blocks-by-default)은 허용 및 거부가 지속되는 기간을 설명합니다.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  dontAsk 모드로 사전 승인된 도구만 허용
</h2>

`dontAsk` 모드를 설정하면 Claude Code는 그 외에 프롬프트를 표시할 모든 도구 호출을 자동으로 거부합니다. Claude는 `permissions.allow` 규칙, [읽기 전용 Bash 명령어](/docs/ko/permissions#read-only-commands), 그리고 [PreToolUse 훅](/docs/ko/permissions#extend-permissions-with-hooks)으로 승인된 호출과 일치하는 작업만 실행합니다. CI 파이프라인이나 Claude가 정확히 수행할 수 있는 작업을 사전에 정의하는 제한된 환경에서 이 모드를 사용하세요. 세션은 입력을 기다리지 않습니다. 이 모드가 활성화되어 있는 동안 상태 표시줄에 `⏵⏵ don't ask on`이 표시됩니다.

Claude Code는 명시적인 [`ask` 규칙](/docs/ko/permissions#manage-permissions)과 일치하는 호출을 프롬프트를 표시하지 않고 거부합니다. 또한 내장 `AskUserQuestion` 도구와 [조직에서 `ask`로 설정한](/docs/ko/mcp#organization-controls-on-connector-tools) 커넥터 도구도 거부합니다. allow 규칙이 일치하더라도 마찬가지입니다. [`_meta["anthropic/requiresUserInteraction"]`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구도 동일한 방식으로 거부됩니다. 왜냐하면 승인 카드가 이 모드에서 수집하지 않는 답변이 필요하기 때문입니다. 이는 Claude Code v2.1.199 이상이 필요합니다.

[Claude Code on the web](/docs/ko/claude-code-on-the-web)의 클라우드 세션은 `defaultMode: "dontAsk"`를 무시합니다. 자세한 내용은 [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode)를 참조하세요.

시작 시 플래그로 설정합니다:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  bypassPermissions 모드로 모든 확인 건너뛰기
</h2>

`bypassPermissions` 모드는 권한 프롬프트와 안전 확인을 비활성화하여 도구 호출이 즉시 실행되도록 합니다. v2.1.126 이전에는 [보호된 경로](#protected-paths)에 대한 쓰기가 여전히 이 모드에서 프롬프트를 표시했습니다.

명시적 [ask 규칙](/docs/ko/permissions#manage-permissions)과 커넥터 도구 [조직에서 `ask`로 설정한](/docs/ko/mcp#organization-controls-on-connector-tools) 도구는 여전히 이 모드에서 프롬프트를 강제합니다. [`_meta["anthropic/requiresUserInteraction"]`](/docs/ko/mcp#require-approval-for-a-specific-tool)로 표시된 MCP 도구도 여전히 프롬프트를 표시합니다. 이는 Claude Code v2.1.199 이상이 필요합니다.

파일 시스템 루트 또는 홈 디렉터리를 대상으로 하는 제거(예: `rm -rf /` 및 `rm -rf ~`)는 모델 오류에 대한 차단기로서 여전히 프롬프트를 표시합니다. 차단기는 또한 명령에 `$(...)` 또는 백틱을 사용한 명령 치환이나 `<(...)`를 사용한 프로세스 치환이 포함될 때 작동합니다. 제거가 `echo "$(rm -rf ~)"`처럼 치환 내부에 있든 같은 명령의 다른 곳에 있든 상관없습니다. 일반 형식은 자체 명령으로 입력되었을 때 차단기가 도입된 이후 이 모드에서 프롬프트를 표시했습니다. v2.1.208 이전에는 이러한 형식을 포함하는 명령이 프롬프트를 표시하지 않았습니다.

<Warning>
  이 모드는 Claude Code가 호스트 시스템에 손상을 줄 수 없는 인터넷 접근이 없는 컨테이너, VM 또는 dev 컨테이너와 같은 격리된 환경에서만 사용하십시오.
</Warning>

활성화 플래그 중 하나 없이 시작된 세션에서 `bypassPermissions`에 진입할 수 없습니다. 활성화하려면 다음 중 하나로 다시 시작하십시오:

```bash theme={null}
claude --permission-mode bypassPermissions
```

`--dangerously-skip-permissions` 플래그는 동등합니다.

Linux 및 macOS에서 Claude Code는 root로 실행되거나 `sudo` 아래에서 실행될 때 이 모드에서 시작하기를 거부합니다:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

확인은 인식된 샌드박스 내에서 자동으로 건너뜁니다. 컨테이너에서 자율적으로 실행하려면 [dev 컨테이너](/docs/ko/devcontainer) 구성을 사용하십시오. 이는 Claude Code를 비루트 사용자로 실행합니다.

[웹의 Claude Code](/docs/ko/claude-code-on-the-web)는 설정 파일의 `defaultMode: "bypassPermissions"` 또는 `"dontAsk"`를 준수하지 않으므로 저장소의 체크인된 설정은 클라우드 세션을 bypass-permissions 모드에서 시작할 수 없습니다. 설정은 자동으로 무시되고 세션은 모드 드롭다운에 표시된 모드에서 시작됩니다. [권한 모드 전환](#switch-permission-modes)을 참조하여 클라우드 세션이 제공하는 모드를 확인하십시오.

<Warning>
  `bypassPermissions`는 프롬프트 주입 또는 의도하지 않은 작업에 대한 보호를 제공하지 않습니다. 훨씬 적은 권한 프롬프트로 백그라운드 안전 확인을 수행하려면 [자동 모드](#eliminate-prompts-with-auto-mode)를 대신 사용하십시오. 관리자는 [관리 설정](/docs/ko/permissions#managed-settings)에서 `permissions.disableBypassPermissionsMode`를 `"disable"`로 설정하여 이 모드를 차단할 수 있습니다.
</Warning>

<h2 id="protected-paths">
  보호된 경로
</h2>

`bypassPermissions`를 제외한 모든 모드에서 특정 경로에 대한 쓰기는 자동으로 승인되지 않습니다. 이는 저장소 상태와 Claude의 자체 구성이 실수로 손상되는 것을 방지합니다.

| 모드                               | 보호된 경로 쓰기 |
| :------------------------------- | :-------- |
| `default`, `acceptEdits`, `plan` | 프롬프트됨     |
| `auto`                           | 분류기로 라우팅됨 |
| `dontAsk`                        | 거부됨       |
| `bypassPermissions`              | 허용됨       |

설정 파일의 [`permissions.allow`](/docs/ko/permissions#manage-permissions) 규칙은 보호된 경로 쓰기를 사전에 승인하지 않습니다. 안전 검사는 Claude Code가 설정에서 allow 규칙을 평가하기 전에 실행되므로, `~/.claude/settings.json` 또는 `.claude/settings.json`의 `Edit(.claude/**)` 같은 항목은 위 표의 모드별 결과를 변경하지 않습니다. 프롬프트를 표시하는 모드에서는 `.claude/` 쓰기에 대한 프롬프트가 **예, Claude가 이 세션 동안 자신의 설정을 편집하도록 허용**을 제공하며, 이는 해당 세션에서 나중의 `.claude/` 쓰기를 다시 프롬프트하지 않고 승인합니다.

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

<h2 id="see-also">
  참고 항목
</h2>

* [권한](/docs/ko/permissions): allow, ask, deny 규칙; 관리형 정책
* [자동 모드 구성](/docs/ko/auto-mode-config): 조직이 신뢰하는 인프라를 분류기에 알립니다
* [Hooks](/docs/ko/hooks): `PreToolUse` 및 `PermissionRequest` 훅을 통한 사용자 정의 권한 로직
* [Ultraplan](/docs/ko/ultraplan): 브라우저 기반 검토를 통해 Claude Code 웹 세션에서 계획 모드 실행
* [보안](/docs/ko/security): 보안 조치 및 모범 사례
* [샌드박싱](/docs/ko/sandboxing): Bash 명령어에 대한 파일 시스템 및 네트워크 격리
* [비대화형 모드](/docs/ko/headless): `-p` 플래그를 사용하여 Claude Code 실행
