> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 설정

> 전역 및 프로젝트 수준 설정과 환경 변수로 Claude Code를 구성합니다.

Claude Code는 사용자의 필요에 맞게 동작을 구성할 수 있는 다양한 설정을 제공합니다. `/config` 명령을 실행하여 Claude Code를 구성할 수 있으며, 이는 상태 정보를 보고 구성 옵션을 수정할 수 있는 탭 형식의 설정 인터페이스를 엽니다. v2.1.181부터는 인터페이스를 열지 않고 `/config`에 `key=value`를 전달하여 단일 옵션을 변경할 수 있습니다. 예를 들어 `/config verbose=true`입니다.

<h2 id="configuration-scopes">
  구성 범위
</h2>

Claude Code는 범위 시스템을 사용하여 구성이 어디에 적용되고 누가 공유하는지 결정합니다. 범위를 이해하면 개인 사용, 팀 협업 또는 엔터프라이즈 배포를 위해 Claude Code를 구성하는 방법을 결정하는 데 도움이 됩니다.

<h3 id="available-scopes">
  사용 가능한 범위
</h3>

| 범위          | 위치                                                        | 영향을 받는 대상                                                                                  | 팀과 공유?           |
| :---------- | :-------------------------------------------------------- | :----------------------------------------------------------------------------------------- | :--------------- |
| **Managed** | 서버 관리 설정, plist / 레지스트리 또는 시스템 수준 `managed-settings.json` | 서버 관리 전달을 위한 모든 조직 구성원; plist, HKLM 레지스트리 및 파일 전달을 위한 머신의 모든 사용자; HKCU 레지스트리 전달을 위한 현재 사용자 | 예 (IT에서 배포)      |
| **User**    | `~/.claude/` 디렉토리                                         | 모든 프로젝트에서 사용자                                                                              | 아니오              |
| **Project** | 저장소의 `.claude/`                                           | 이 저장소의 모든 협업자                                                                              | 예 (git에 커밋됨)     |
| **Local**   | `.claude/settings.local.json`                             | 이 저장소에서만 사용자                                                                               | 아니오 (gitignored) |

<h3 id="when-to-use-each-scope">
  각 범위를 사용할 때
</h3>

**Managed 범위**는 다음을 위한 것입니다:

* 조직 전체에서 적용해야 하는 보안 정책
* 재정의할 수 없는 규정 준수 요구 사항
* IT/DevOps에서 배포한 표준화된 구성

**User 범위**는 다음에 가장 적합합니다:

* 모든 곳에서 원하는 개인 설정 (테마, 편집기 설정)
* 모든 프로젝트에서 사용하는 도구 및 플러그인
* API 키 및 인증 (안전하게 저장됨)

**Project 범위**는 다음에 가장 적합합니다:

* 팀 공유 설정 (권한, hooks, MCP servers)
* 전체 팀이 가져야 할 플러그인
* 협업자 간 도구 표준화

**Local 범위**는 다음에 가장 적합합니다:

* 특정 프로젝트에 대한 개인 재정의
* 팀과 공유하기 전에 구성 테스트
* 다른 사용자에게는 작동하지 않을 머신 특정 설정

<h3 id="how-scopes-interact">
  범위가 상호 작용하는 방식
</h3>

동일한 설정이 여러 범위에서 구성되면 Claude Code는 우선순위 순서대로 적용합니다:

1. **Managed** (최고): 아무것도 재정의할 수 없음
2. **명령줄 인수**: 임시 세션 재정의
3. **Local**: 프로젝트 및 사용자 설정 재정의
4. **Project**: 사용자 설정 재정의
5. **User** (최저): 다른 것이 설정을 지정하지 않을 때 적용

예를 들어, 사용자 설정에서 `spinnerTipsEnabled`를 `true`로 설정하고 프로젝트 설정에서 `false`로 설정하면 프로젝트 값이 적용됩니다. 권한 규칙은 재정의하지 않고 범위 전체에서 병합되기 때문에 다르게 작동합니다. [설정 우선순위](#settings-precedence)를 참조하십시오.

<h3 id="what-uses-scopes">
  범위를 사용하는 것
</h3>

범위는 많은 Claude Code 기능에 적용됩니다:

| 기능              | 사용자 위치                    | 프로젝트 위치                            | Local 위치                      |
| :-------------- | :------------------------ | :--------------------------------- | :---------------------------- |
| **Settings**    | `~/.claude/settings.json` | `.claude/settings.json`            | `.claude/settings.local.json` |
| **Subagents**   | `~/.claude/agents/`       | `.claude/agents/`                  | 없음                            |
| **MCP servers** | `~/.claude.json`          | `.mcp.json`                        | `~/.claude.json` (프로젝트별)      |
| **Plugins**     | `~/.claude/settings.json` | `.claude/settings.json`            | `.claude/settings.local.json` |
| **CLAUDE.md**   | `~/.claude/CLAUDE.md`     | `CLAUDE.md` 또는 `.claude/CLAUDE.md` | `CLAUDE.local.md`             |

Windows에서 `~/.claude`로 표시된 경로는 `%USERPROFILE%\.claude`로 확인됩니다.

***

<h2 id="settings-files">
  설정 파일
</h2>

`settings.json` 파일은 계층적 설정을 통해 Claude Code를 구성하기 위한 공식 메커니즘입니다:

* **사용자 설정**은 `~/.claude/settings.json`에 정의되며 모든 프로젝트에 적용됩니다.
* **프로젝트 설정**은 프로젝트 디렉토리에 저장됩니다:
  * 소스 제어에 체크인되고 팀과 공유되는 설정을 위한 `.claude/settings.json`
  * 체크인되지 않은 설정을 위한 `.claude/settings.local.json`으로, 개인 설정 및 실험에 유용합니다. Claude Code는 `.claude/settings.local.json`이 생성될 때 git을 구성하여 이를 무시하도록 합니다. 파일을 직접 생성하는 경우 gitignore에 수동으로 추가합니다.

    이 파일은 저장소가 아닌 사용자의 파일이므로 `allow` 권한 규칙이 `.claude/settings.json` 허용 규칙이 요구하는 [작업 공간 신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust) 단계 없이 적용됩니다. 저장소가 파일을 제공하는 경우 (예: 커밋하여) 작업 공간 신뢰가 여전히 적용됩니다.
* **Managed 설정**: 중앙 집중식 제어가 필요한 조직의 경우 Claude Code는 managed 설정을 위한 여러 전달 메커니즘을 지원합니다. 모두 동일한 JSON 형식을 사용하며 사용자 또는 프로젝트 설정으로 재정의할 수 없습니다:

  * **서버 관리 설정**: Anthropic의 서버에서 claude.ai 관리 콘솔을 통해 또는 자체 호스팅 [Claude apps gateway](/docs/ko/claude-apps-gateway)에서 원격으로 전달됩니다. [서버 관리 설정](/docs/ko/server-managed-settings)을 참조하세요.
  * **MDM/OS 수준 정책**: macOS 및 Windows의 기본 장치 관리를 통해 전달됩니다:
    * macOS: `com.anthropic.claudecode` managed preferences domain. plist의 최상위 키는 `managed-settings.json`을 반영하며, 중첩된 설정은 딕셔너리이고 배열은 plist 배열입니다. Jamf, Iru (Kandji) 또는 유사한 MDM 도구의 구성 프로필을 통해 배포합니다.
    * Windows: `HKLM\SOFTWARE\Policies\ClaudeCode` 레지스트리 키와 JSON을 포함하는 `Settings` 값 (REG\_SZ 또는 REG\_EXPAND\_SZ) (그룹 정책 또는 Intune을 통해 배포)
    * Windows (사용자 수준): `HKCU\SOFTWARE\Policies\ClaudeCode` (최저 정책 우선순위, 관리자 수준 소스가 없을 때만 사용)
  * **파일 기반**: 시스템 디렉토리에 배포된 `managed-settings.json` 및 `managed-mcp.json`:

    * macOS: `/Library/Application Support/ClaudeCode/`
    * Linux 및 WSL: `/etc/claude-code/`
    * Windows: `C:\Program Files\ClaudeCode\`

    <Warning>
      레거시 Windows 경로 `C:\ProgramData\ClaudeCode\managed-settings.json`은 v2.1.75부터 더 이상 지원되지 않습니다. 해당 위치에 설정을 배포한 관리자는 파일을 `C:\Program Files\ClaudeCode\managed-settings.json`으로 마이그레이션해야 합니다.
    </Warning>

    파일 기반 managed 설정은 `managed-settings.json`과 동일한 시스템 디렉토리에 `managed-settings.d/` 드롭인 디렉토리도 지원합니다. 이를 통해 별도의 팀이 단일 파일 편집을 조정하지 않고 독립적인 정책 조각을 배포할 수 있습니다.

    systemd 규칙을 따르면 `managed-settings.json`이 먼저 기본으로 병합되고, 드롭인 디렉토리의 모든 `*.json` 파일이 알파벳순으로 정렬되어 위에 병합됩니다. 스칼라 값의 경우 나중 파일이 이전 파일을 재정의합니다. 배열은 연결되고 중복 제거됩니다. 객체는 깊게 병합됩니다. `.`로 시작하는 숨겨진 파일은 무시됩니다.

    병합 순서를 제어하려면 숫자 접두사를 사용합니다 (예: `10-telemetry.json` 및 `20-security.json`).

  [managed 설정](/docs/ko/permissions#managed-only-settings) 및 [Managed MCP 구성](/docs/ko/managed-mcp)을 참조하세요.

  이 [저장소](https://github.com/anthropics/claude-code/tree/main/examples/mdm)에는 Jamf, Iru (Kandji), Intune 및 그룹 정책에 대한 시작 배포 템플릿이 포함되어 있습니다. 이를 시작점으로 사용하고 필요에 맞게 조정합니다.

  <Note>
    Managed 배포는 `strictKnownMarketplaces`를 사용하여 **플러그인 마켓플레이스 추가**를 제한할 수도 있습니다. 자세한 내용은 [Managed 마켓플레이스 제한](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions)을 참조하세요.
  </Note>
* **기타 구성**은 `~/.claude.json`에 저장됩니다. 이 파일에는 OAuth 세션, [MCP server](/docs/ko/mcp) 구성 (사용자 및 local 범위), 프로젝트별 상태 (허용된 도구, 신뢰 설정) 및 다양한 캐시가 포함됩니다. 프로젝트 범위 MCP 서버는 `.mcp.json`에 별도로 저장됩니다.

<Note>
  Claude Code는 자동으로 구성 파일의 타임스탐프가 지정된 백업을 생성하고 데이터 손실을 방지하기 위해 가장 최근의 5개 백업을 유지합니다.
</Note>

```JSON 예제 settings.json theme={null}
{
  "$schema": "https://json.schemastore.org/claude-code-settings.json",
  "permissions": {
    "allow": [
      "Bash(npm run lint)",
      "Bash(npm run test *)",
      "Read(~/.zshrc)"
    ],
    "deny": [
      "Bash(curl *)",
      "Read(./.env)",
      "Read(./.env.*)",
      "Read(./secrets/**)"
    ]
  },
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp"
  },
  "companyAnnouncements": [
    "Welcome to Acme Corp! Review our code guidelines at docs.acme.com",
    "Reminder: Code reviews required for all PRs",
    "New security policy in effect"
  ]
}
```

위의 예제에서 `$schema` 줄은 Claude Code 설정에 대한 [공식 JSON 스키마](https://json.schemastore.org/claude-code-settings.json)를 가리킵니다. 이를 `settings.json`에 추가하면 VS Code, Cursor 및 JSON 스키마 검증을 지원하는 다른 편집기에서 자동 완성 및 인라인 검증이 활성화됩니다.

게시된 스키마는 주기적으로 업데이트되며 가장 최근 CLI 릴리스에서 추가된 설정을 포함하지 않을 수 있으므로, 최근에 문서화된 필드에 대한 검증 경고가 반드시 구성이 유효하지 않음을 의미하지는 않습니다.

<h3 id="when-edits-take-effect">
  편집이 적용되는 시기
</h3>

Claude Code는 설정 파일을 감시하고 변경될 때 다시 로드하므로 대부분의 키에 대한 편집은 재시작 없이 실행 중인 세션에 적용됩니다. 여기에는 `permissions`, `hooks` 및 `apiKeyHelper`와 같은 자격 증명 도우미가 포함됩니다. 다시 로드는 사용자, 프로젝트, local 및 managed 설정을 포함하며, 감지된 각 변경에 대해 [`ConfigChange` hook](/docs/ko/hooks#configchange)이 실행됩니다.

몇 가지 키는 세션 시작 시 한 번 읽혀지고 대신 다음 재시작에 적용됩니다:

* `model`: 세션 중에 전환하려면 [`/model`](/docs/ko/model-config#setting-your-model)을 사용합니다
* [`outputStyle`](/docs/ko/output-styles): 시스템 프롬프트의 일부로, `/clear` 또는 재시작 시 다시 빌드됩니다

<h3 id="invalid-entries-in-managed-settings">
  Managed 설정의 유효하지 않은 항목
</h3>

Managed 설정은 관대하게 파싱됩니다. Managed 구성에 스키마 검증에 실패하는 항목이 포함되어 있으면 Claude Code는 해당 항목을 제거하고 경고를 기록하며 남은 모든 유효한 정책을 적용합니다. 단일 오타가 조직의 나머지 정책을 비활성화할 수 없습니다. [`/doctor`](/docs/ko/debug-your-config#check-resolved-settings)를 실행하여 제거된 항목을 소스 파일 및 필드와 함께 나열합니다.

이 동작은 세 가지 전달 메커니즘 모두에서 일관됩니다: [서버 관리 설정](/docs/ko/server-managed-settings), MDM을 통해 배포된 plist 및 레지스트리 정책, 그리고 `managed-settings.json` 파일. Claude Code v2.1.169 이상이 필요합니다.

보안 적용 필드는 전체적으로 제거되는 대신 필드별로 처리됩니다:

| 필드                           | 존재하지만 유효하지 않을 때의 동작                                                                                                                                          |
| :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers`          | 값이 수정될 때까지 MCP 서버가 허용되지 않도록 빈 허용 목록으로 적용됩니다. 개별 유효하지 않은 항목은 제거되고 유효한 부분 집합이 적용됩니다.                                                                           |
| `allowManagedMcpServersOnly` | `true`로 처리됩니다.                                                                                                                                               |
| `availableModels`            | 값이 수정될 때까지 기본 모델만 사용 가능하도록 빈 허용 목록으로 적용됩니다. 개별 비문자열 항목은 제거되고 유효한 부분 집합이 적용됩니다. v2.1.175 이상에 적용됩니다.                                                           |
| `enforceAvailableModels`     | }`true`로 처리됩니다. v2.1.175 이상에 적용됩니다.                                                                                                                          |
| `forceLoginOrgUUID`          | 값이 수정될 때까지 조직이 로그인할 수 없습니다.                                                                                                                                  |
| `deniedMcpServers`           | 개별 유효하지 않은 항목은 제거되고 유효한 부분 집합이 적용됩니다. 완전히 유효하지 않은 값은 경고와 함께 삭제됩니다. 모든 서버를 거부하면 정책이 명명하지 않은 서버를 차단하기 때문입니다.                                                   |
| `sandbox.credentials`        | }`files` 또는 `envVars`의 개별 유효하지 않은 항목은 경고와 함께 제거되고 유효한 부분 집합이 적용됩니다. 완전히 유효하지 않은 `credentials` 값은 경고와 함께 삭제되지만 `sandbox`의 나머지는 여전히 적용됩니다. v2.1.191 이상에 적용됩니다. |

`requiredMinimumVersion` 및 `requiredMaximumVersion`은 설계상 실패하도록 열려 있습니다: 유효하지 않은 값은 적용되지 않고 제거되므로 잘못된 정책 푸시가 Claude Code 시작을 방지할 수 없습니다.

검증 오류는 세 곳에 표시됩니다:

* 대화형 세션은 시작 시 유효하지 않은 항목을 나열하는 대화를 표시합니다.
* `-p`를 사용한 헤드리스 실행은 stderr에 요약을 인쇄합니다.
* [`claude doctor`](/docs/ko/debug-your-config)는 각 유효하지 않은 항목을 소스 및 필드와 함께 나열합니다.

정책 변경을 검증하려면 전사 배포 전에 테스트 머신에서 `claude doctor`를 실행합니다.

이 관대함은 managed 설정에만 적용됩니다. 사용자, 프로젝트 및 local 설정 파일은 엄격합니다: 검증에 실패하는 파일은 전체적으로 거부되고 보고됩니다.

<h3 id="available-settings">
  사용 가능한 설정
</h3>

`settings.json`은 여러 옵션을 지원합니다:

| 키                                  | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | 예제                                                                                                                              |
| :--------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `advisorModel`                     | 서버 측 [advisor 도구](/docs/ko/advisor)를 위한 모델입니다. `"opus"`, `"sonnet"` 또는 `"fable"` (v2.1.170+)과 같은 모델 별칭 또는 전체 모델 ID를 허용합니다. `/advisor`를 실행할 때 자동으로 작성됩니다. Advisor를 비활성화하려면 설정 해제합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `"opus"`                                                                                                                        |
| `agent`                            | 메인 스레드를 명명된 subagent로 실행하고 `claude agents`에서 디스패치된 세션의 기본 에이전트를 설정합니다. 해당 subagent의 시스템 프롬프트, 도구 제한 및 모델을 적용합니다. [subagents 명시적으로 호출](/docs/ko/sub-agents#invoke-subagents-explicitly)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `"code-reviewer"`                                                                                                               |
| `agentPushNotifEnabled`            | **기본값**: `false`. [Remote Control](/docs/ko/remote-control)이 연결되어 있을 때 Claude가 장시간 작업이 완료될 때와 같이 휴대폰에 사전 예방적 푸시 알림을 보낼 수 있도록 허용합니다. `/config`에 **Claude가 결정할 때 푸시**로 표시됩니다. [모바일 푸시 알림](/docs/ko/remote-control#mobile-push-notifications)을 참조하세요. Claude Code v2.1.119 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `true`                                                                                                                          |
| `allowAllClaudeAiMcps`             | (Managed 설정만) 배포된 `managed-mcp.json`과 함께 claude.ai 커넥터를 로드합니다. 그렇지 않으면 독점적 제어를 취하고 이를 억제합니다. [Managed MCP 구성](/docs/ko/managed-mcp)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `true`                                                                                                                          |
| `allowedChannelPlugins`            | (Managed 설정만) 메시지를 푸시할 수 있는 채널 플러그인의 허용 목록입니다. 설정되면 기본 Anthropic 허용 목록을 대체합니다. 정의되지 않음 = 기본값으로 폴백, 빈 배열 = 모든 채널 플러그인 차단. `channelsEnabled: true`가 필요합니다. [채널 플러그인 실행 제한](/docs/ko/channels#restrict-which-channel-plugins-can-run)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `[{ "marketplace": "claude-plugins-official", "plugin": "telegram" }]`                                                          |
| `allowedHttpHookUrls`              | HTTP hooks가 대상으로 할 수 있는 URL 패턴의 허용 목록입니다. `*`를 와일드카드로 지원합니다. 설정되면 일치하지 않는 URL을 가진 hooks는 차단됩니다. 정의되지 않음 = 제한 없음, 빈 배열 = 모든 HTTP hooks 차단. 배열은 설정 소스 전체에서 병합됩니다. [Hook 구성](#hook-configuration)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `["https://hooks.example.com/*"]`                                                                                               |
| `allowedMcpServers`                | Managed 설정에서 설정되면 사용자가 구성할 수 있는 MCP 서버의 허용 목록입니다. 정의되지 않음 = 제한 없음, 빈 배열 = 잠금. 모든 범위에 적용됩니다. 거부 목록이 우선합니다. [Managed MCP 구성](/docs/ko/managed-mcp)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `[{ "serverName": "github" }]`                                                                                                  |
| `allowManagedHooksOnly`            | (Managed 설정만) Managed hooks, SDK hooks 및 managed 설정 `enabledPlugins`에서 강제 활성화된 플러그인의 hooks만 로드됩니다. 사용자, 프로젝트 및 다른 모든 플러그인 hooks는 차단됩니다. [Hook 구성](#hook-configuration)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `true`                                                                                                                          |
| `allowManagedMcpServersOnly`       | (Managed 설정만) Managed 설정의 `allowedMcpServers`만 존중됩니다. `deniedMcpServers`는 여전히 모든 소스에서 병합됩니다. 사용자는 여전히 MCP 서버를 추가할 수 있지만 관리자 정의 허용 목록만 적용됩니다. [Managed MCP 구성](/docs/ko/managed-mcp)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `true`                                                                                                                          |
| `allowManagedPermissionRulesOnly`  | (Managed 설정만) 사용자 및 프로젝트 설정이 `allow`, `ask` 또는 `deny` 권한 규칙을 정의하는 것을 방지합니다. Managed 설정의 규칙만 적용됩니다. [Managed 전용 설정](/docs/ko/permissions#managed-only-settings)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `true`                                                                                                                          |
| `alwaysThinkingEnabled`            | 모든 세션에 대해 기본적으로 [확장 사고](/docs/ko/model-config#extended-thinking)를 활성화합니다. 일반적으로 직접 편집하기보다는 `/config` 명령을 통해 구성됩니다. 사고를 강제로 끄려면 `env`에서 [`MAX_THINKING_TOKENS=0`](/docs/ko/env-vars)을 설정합니다. 이는 Anthropic API에서 사고를 비활성화합니다. Fable 5는 제외되며, 이는 사고를 끌 수 없습니다. [제3자 공급자](/docs/ko/third-party-integrations)에서 이는 `thinking` 매개변수를 생략하며, 적응형 추론 모델은 여전히 사고할 수 있습니다                                                                                                                                                                                                                                                                                                                                                                                                           | `true`                                                                                                                          |
| `apiKeyHelper`                     | 시스템 셸 (`/bin/sh` on macOS and Linux, `cmd` on Windows)을 통해 실행될 사용자 정의 명령으로 인증 값을 생성합니다. 이 값은 모델 요청에 대해 `X-Api-Key` 및 `Authorization: Bearer` 헤더로 전송됩니다. [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](/docs/ko/env-vars)로 새로고침 간격을 설정합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `/bin/generate_temp_api_key.sh`                                                                                                 |
| `askUserQuestionTimeout`           | **기본값**: `"never"`. 답변되지 않은 [`AskUserQuestion`](/docs/ko/tools-reference) 대화가 이미 선택한 옵션으로 자동 계속되기 전의 유휴 시간입니다. `"60s"`, `"5m"`, `"10m"` 또는 `"never"`를 허용합니다. 기본값을 사용하면 질문은 답변할 때까지 기다립니다. `/config`에 **질문 자동 계속 시간 초과**로 표시됩니다. 프로젝트 또는 local 설정에서는 읽지 않습니다. Claude Code v2.1.200 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `"5m"`                                                                                                                          |
| `attribution`                      | git 커밋 및 pull request에 대한 attribution을 사용자 정의합니다. [Attribution 설정](#attribution-settings)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `{"commit": "🤖 Generated with Claude Code", "pr": ""}`                                                                         |
| `autoCompactEnabled`               | **기본값**: `true`. 컨텍스트가 한계에 접근할 때 자동으로 대화를 압축합니다. `/config`에 **자동 압축**으로 표시됩니다. 환경 변수로 비활성화하려면 `env`에서 [`DISABLE_AUTO_COMPACT`](/docs/ko/env-vars)를 설정합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `false`                                                                                                                         |
| `autoMemoryDirectory`              | [자동 메모리](/docs/ko/memory#storage-location) 저장소를 위한 사용자 정의 디렉토리입니다. 절대 경로 또는 `~/` 접두사 경로를 허용합니다. 프로젝트 또는 local 설정에서 이는 작업 공간 신뢰 대화를 수락한 후에만 적용됩니다. 복제된 저장소가 이 파일을 제공할 수 있기 때문입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `"~/my-memory-dir"`                                                                                                             |
| `autoMemoryEnabled`                | **기본값**: `true`. [자동 메모리](/docs/ko/memory#enable-or-disable-auto-memory)를 활성화합니다. `false`일 때 Claude는 자동 메모리 디렉토리에서 읽거나 쓰지 않습니다. 세션 중에 `/memory`로도 전환할 수 있습니다. 환경 변수로 비활성화하려면 `env`에서 [`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](/docs/ko/env-vars)를 설정합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `false`                                                                                                                         |
| `autoMode`                         | [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) 분류기가 차단하고 허용하는 것을 사용자 정의합니다. `environment`, `allow`, `soft_deny` 및 `hard_deny` 배열의 산문 규칙을 포함합니다. 배열에 리터럴 문자열 `"$defaults"`를 포함하여 해당 위치에서 기본 제공 규칙을 상속합니다. [자동 모드 구성](/docs/ko/auto-mode-config)을 참조하세요. 사용자 설정, `--settings` 플래그 및 managed 설정에서만 읽습니다. 프로젝트 `.claude/settings.json` 및 local `.claude/settings.local.json`에서는 무시됩니다. v2.1.207 이전에는 `.claude/settings.local.json`도 읽혔습니다                                                                                                                                                                                                                                                                                                          | `{"soft_deny": ["$defaults", "Never run terraform apply"]}`                                                                     |
| `autoMode.classifyAllShell`        | **기본값**: `false`. `true`일 때 자동 모드가 활성화되어 있는 동안 모든 Bash 및 PowerShell 허용 규칙을 일시 중단하므로 모든 셸 명령이 임의 코드 실행 패턴과 일치하는 규칙뿐만 아니라 분류기를 통해 라우팅됩니다. [분류기를 통해 모든 셸 명령 라우팅](/docs/ko/auto-mode-config#route-all-shell-commands-through-the-classifier)을 참조하세요. Claude Code v2.1.193 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `true`                                                                                                                          |
| `autoScrollEnabled`                | **기본값**: `true`. [fullscreen 렌더링](/docs/ko/fullscreen)에서 새 출력을 대화의 맨 아래로 따릅니다. `/config`에 **자동 스크롤**로 표시됩니다. 이것이 꺼져 있을 때도 권한 프롬프트는 여전히 보기로 스크롤됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `false`                                                                                                                         |
| `autoUpdatesChannel`               | **기본값**: `"latest"`. 업데이트를 따를 릴리스 채널입니다. 일반적으로 약 1주일 된 버전이고 주요 회귀가 있는 버전을 건너뛰는 `"stable"`을 사용하거나 가장 최근 릴리스인 `"latest"`를 사용합니다. 자동 업데이트를 완전히 비활성화하려면 `env`에서 [`DISABLE_AUTOUPDATER`](/docs/ko/setup#disable-auto-updates)를 설정합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `"stable"`                                                                                                                      |
| `availableModels`                  | 사용자가 메인 세션, [subagents](/docs/ko/sub-agents), [skills](/docs/ko/skills) 및 [advisor](/docs/ko/advisor)를 위해 선택할 수 있는 모델을 제한합니다. `enforceAvailableModels`도 설정되지 않으면 기본 옵션에는 영향을 주지 않습니다. [모델 선택 제한](/docs/ko/model-config#restrict-model-selection)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `["sonnet", "haiku"]`                                                                                                           |
| `awaySummaryEnabled`               | 몇 분 동안 터미널에서 떨어져 있다가 돌아올 때 한 줄 세션 요약을 표시합니다. 비활성화하려면 `false`로 설정하거나 `/config`에서 세션 요약을 끕니다. [`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](/docs/ko/env-vars)와 동일합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `true`                                                                                                                          |
| `awsAuthRefresh`                   | `.aws` 디렉토리를 수정하는 사용자 정의 스크립트 ([고급 자격 증명 구성](/docs/ko/amazon-bedrock#advanced-credential-configuration) 참조)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `aws sso login --profile myprofile`                                                                                             |
| `awsCredentialExport`              | AWS 자격 증명이 포함된 JSON을 출력하는 사용자 정의 스크립트 ([고급 자격 증명 구성](/docs/ko/amazon-bedrock#advanced-credential-configuration) 참조)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `/bin/generate_aws_grant.sh`                                                                                                    |
| `axScreenReader`                   | 화면 판독기 친화적 출력을 렌더링합니다: 장식적 테두리나 애니메이션 없는 평면 텍스트. 화면 판독기 모드는 항상 클래식 렌더러를 사용하므로 활성화되어 있는 동안 `tui` 설정은 영향을 주지 않습니다. 연결된 [배경 세션](/docs/ko/agent-view)은 여전히 fullscreen을 렌더링합니다. [`CLAUDE_AX_SCREEN_READER`](/docs/ko/env-vars) 환경 변수 및 [`--ax-screen-reader`](/docs/ko/cli-reference#cli-flags) 플래그가 우선합니다. Claude Code v2.1.181 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                    | `true`                                                                                                                          |
| `blockedMarketplaces`              | (Managed 설정만) 마켓플레이스 소스의 차단 목록입니다. 마켓플레이스 추가 및 플러그인 설치, 업데이트, 새로고침 및 자동 업데이트에 적용되므로 정책이 설정되기 전에 추가된 마켓플레이스는 플러그인을 가져오는 데 사용할 수 없습니다. 차단된 소스는 다운로드 전에 확인되므로 파일 시스템에 닿지 않습니다. [Managed 마켓플레이스 제한](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `[{ "source": "github", "repo": "untrusted/plugins" }]`                                                                         |
| `browserExternalPageTools`         | (Managed 설정만) 데스크톱 앱의 [Browser 창](/docs/ko/desktop#browse-external-sites)에서 외부 페이지를 읽거나 작용하기 위해 Claude가 도구를 사용하는 것을 방지하려면 `"disabled"`로 설정합니다. 사용자는 여전히 외부 사이트로 직접 이동할 수 있으며 local dev 서버 미리보기는 영향을 받지 않습니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `"disabled"`                                                                                                                    |
| `channelsEnabled`                  | (Managed 설정만) 조직을 위해 [channels](/docs/ko/channels)를 허용합니다. Claude.ai Team 및 Enterprise 플랜에서 설정되지 않거나 `false`이면 채널이 차단됩니다. [Anthropic Console](/docs/ko/authentication#claude-console-authentication) 계정이 API 키 인증을 사용하는 경우 조직이 managed 설정을 배포하지 않으면 기본적으로 채널이 허용되며, 이 경우 이 키를 `true`로 설정해야 합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `true`                                                                                                                          |
| `claudeMd`                         | (Managed 설정만) 조직 관리 메모리로 주입된 CLAUDE.md 스타일 지침입니다. Managed 또는 정책 설정에서 설정된 경우에만 적용되며 사용자, 프로젝트 및 local 설정에서는 무시됩니다. [조직 전체 CLAUDE.md](/docs/ko/memory#deploy-organization-wide-claude-md)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `"Always run make lint before committing."`                                                                                     |
| `claudeMdExcludes`                 | [메모리](/docs/ko/memory)를 로드할 때 건너뛸 `CLAUDE.md` 파일의 Glob 패턴 또는 절대 경로입니다. 패턴은 절대 파일 경로와 일치합니다. 사용자, 프로젝트 및 local 메모리에만 적용됩니다. managed 정책 파일은 제외할 수 없습니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `["**/vendor/**/CLAUDE.md"]`                                                                                                    |
| `cleanupPeriodDays`                | **기본값**: `30`일, 최소 `1`. Claude Code는 이 기간보다 오래된 [세션 파일 및 기타 애플리케이션 데이터](/docs/ko/claude-directory#cleaned-up-automatically)를 시작 시 삭제합니다. `0`으로 설정하면 검증 오류가 발생합니다. 또한 시작 시 [고아 worktrees](/docs/ko/worktrees#clean-up-worktrees)의 자동 제거에 대한 나이 기준을 제어합니다. Claude Code가 설정 파일을 읽거나 파싱할 수 없으면 보존 정리 스윕을 일시 중단하고 파일을 수정할 때까지 `/status`에 경고를 표시합니다. 단, [managed 설정](/docs/ko/server-managed-settings)이 `cleanupPeriodDays`를 제공하는 경우 스윕은 managed 값에서 실행됩니다. v2.1.203 이전에는 정리가 해당 상태에서 30일 기본값으로 실행되었고 더 긴 `cleanupPeriodDays`가 유지하려던 트랜스크립트를 삭제할 수 있었습니다. 30일보다 최신인 파일은 절대 제거되지 않았습니다. 트랜스크립트 쓰기를 완전히 비활성화하려면 [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/ko/env-vars) 환경 변수를 설정합니다. 비대화형 모드에서 `-p`와 함께 `--no-session-persistence`를 전달하거나 Agent SDK에서 `persistSession: false`를 설정합니다. | `20`                                                                                                                            |
| `companyAnnouncements`             | 시작 시 사용자에게 표시할 공지사항입니다. 여러 공지사항이 제공되면 무작위로 순환됩니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `["Welcome to Acme Corp! Review our code guidelines at docs.acme.com"]`                                                         |
| `defaultShell`                     | **기본값**: `"bash"`, 또는 Bash를 사용할 수 없을 때 Windows에서 `"powershell"`. 입력 상자 `!` 명령의 기본 셸입니다. `"bash"` 또는 `"powershell"`을 허용합니다. `"powershell"`을 설정하면 Windows에서 대화형 `!` 명령을 PowerShell을 통해 라우팅합니다. `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`이 필요합니다. [PowerShell 도구](/docs/ko/tools-reference#powershell-tool)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `"powershell"`                                                                                                                  |
| `deniedMcpServers`                 | Managed 설정에서 설정되면 명시적으로 차단된 MCP 서버의 거부 목록입니다. Managed 서버를 포함한 모든 범위에 적용됩니다. 거부 목록이 허용 목록보다 우선합니다. [Managed MCP 구성](/docs/ko/managed-mcp)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `[{ "serverName": "filesystem" }]`                                                                                              |
| `disableAgentView`                 | [배경 에이전트 및 에이전트 보기](/docs/ko/agent-view)를 끄려면 `true`로 설정합니다: `claude agents`, `--bg`, `/background` 및 온디맨드 감독자. 일반적으로 [managed 설정](/docs/ko/permissions#managed-settings)에서 설정됩니다. `CLAUDE_CODE_DISABLE_AGENT_VIEW`를 `1`로 설정하는 것과 동일합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `true`                                                                                                                          |
| `disableAllHooks`                  | 모든 [hooks](/docs/ko/hooks) 및 사용자 정의 [상태 줄](/docs/ko/statusline) 비활성화                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `true`                                                                                                                          |
| `disableArtifact`                  | [Artifact](/docs/ko/artifacts) 도구를 비활성화하려면 `true`로 설정합니다. 이는 세션 출력을 claude.ai의 비공개 웹 페이지로 게시합니다. `CLAUDE_CODE_DISABLE_ARTIFACT`를 `1`로 설정하는 것과 동일합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `true`                                                                                                                          |
| `disableAutoMode`                  | [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)가 활성화되는 것을 방지하려면 `"disable"`로 설정합니다. `Shift+Tab` 순환에서 `auto`를 제거하고 시작 시 `--permission-mode auto`를 거부합니다. [managed 설정](/docs/ko/permissions#managed-settings)에서 사용자가 재정의할 수 없을 때 가장 유용합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `"disable"`                                                                                                                     |
| `disableBrowserExternalNavigation` | (Managed 설정만) 데스크톱 앱의 [Browser 창](/docs/ko/desktop#browse-external-sites)에서 외부 탐색을 끄려면 `true`로 설정합니다. 사용자와 Claude 모두 외부 사이트로 이동할 수 없으며 localhost dev 서버 미리보기는 영향을 받지 않습니다. 값은 JSON 부울 `true`여야 합니다. 문자열 `"true"`는 무시됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `true`                                                                                                                          |
| `disableBundledSkills`             | [skills](/docs/ko/skills) 및 Claude Code와 함께 제공되는 워크플로우를 비활성화하려면 `true`로 설정합니다: 번들 skills 및 워크플로우는 완전히 제거되고, `/init`과 같은 기본 제공 슬래시 명령은 입력 가능하지만 모델에서 숨겨집니다. `/doctor`는 기본 제공 명령처럼 입력 가능하지만 [`DISABLE_DOCTOR_COMMAND`](/docs/ko/env-vars)로 숨깁니다. 플러그인의 skills, `.claude/skills/` 및 `.claude/commands/`는 영향을 받지 않습니다. `CLAUDE_CODE_DISABLE_BUNDLED_SKILLS`를 `1`로 설정하는 것과 동일합니다                                                                                                                                                                                                                                                                                                                                                                                         | `true`                                                                                                                          |
| `disableClaudeAiConnectors`        | }[claude.ai MCP 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 비활성화하여 자동 가져오기 또는 연결되지 않도록 합니다. 모든 설정 범위에서 설정합니다. 모든 소스에서 `true`가 우선하므로 체크인된 프로젝트 `.claude/settings.json`이 저장소를 클라우드 커넥터에서 제외할 수 있지만 프로젝트 수준 `false`는 사용자 또는 정책 수준 `true`를 재정의할 수 없습니다. `--mcp-config`를 통해 명시적으로 전달된 서버는 영향을 받지 않습니다. 모든 커넥터를 거부하는 대신 개별 커넥터를 거부하려면 [`deniedMcpServers`](/docs/ko/managed-mcp)를 사용합니다. Claude Code v2.1.182 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                            | `true`                                                                                                                          |
| `disableDeepLinkRegistration`      | Claude Code가 시작 시 운영 체제에 `claude-cli://` 프로토콜 핸들러를 등록하는 것을 방지하려면 `"disable"`로 설정합니다. [Deep links](/docs/ko/deep-links)를 사용하면 외부 도구가 사전 채워진 프롬프트로 Claude Code 세션을 열 수 있습니다. 프로토콜 핸들러 등록이 제한되거나 별도로 관리되는 환경에서 유용합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `"disable"`                                                                                                                     |
| `disabledMcpjsonServers`           | `.mcp.json` 파일에서 거부할 특정 MCP 서버 목록                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `["filesystem"]`                                                                                                                |
| `disableRemoteControl`             | }[Remote Control](/docs/ko/remote-control) 비활성화: `claude remote-control`, `--remote-control` 플래그, 자동 시작 및 세션 내 전환을 차단합니다. 일반적으로 장치별 MDM 적용을 위해 [managed 설정](/docs/ko/permissions#managed-settings)에 배치되지만 모든 범위에서 작동합니다. Claude Code v2.1.128 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `true`                                                                                                                          |
| `disableSideloadFlags`             | }(Managed 설정만) 시작 시 `--plugin-dir`, `--plugin-url`, `--agents` 및 `--mcp-config` CLI 플래그를 거부합니다. 사용자는 단일 실행을 위해 [`strictKnownMarketplaces`](#strictknownmarketplaces)를 우회하기 위해 이를 전달할 수 있습니다. 또한 현재 [Cowork](/docs/ko/desktop) local 세션인 CLI를 내부적으로 생성하는 모든 표면에서 이러한 플래그를 거부합니다. 모든 서버가 in-process `type: "sdk"` 항목인 `--mcp-config`는 여전히 허용되므로 Agent SDK 및 VS Code 확장이 계속 작동합니다. `claude mcp add`, `.mcp.json` 또는 SDK `setMcpServers()`를 차단하지 않습니다. 서버별 MCP 제어를 위해 [`allowedMcpServers`](/docs/ko/managed-mcp)와 쌍을 이룹니다. Claude Code v2.1.193 이상이 필요합니다                                                                                                                                                                                                              | `true`                                                                                                                          |
| `disableSkillShellExecution`       | [skills](/docs/ko/skills) 및 사용자, 프로젝트, 플러그인 또는 추가 디렉토리 소스의 사용자 정의 명령에서 `` !`...` `` 및 ` ```! ` 블록에 대한 인라인 셸 실행을 비활성화합니다. 명령은 실행되는 대신 `[shell command execution disabled by policy]`로 대체됩니다. 번들 및 managed skills는 영향을 받지 않습니다. [managed 설정](/ko/permissions#managed-settings)에서 사용자가 재정의할 수 없을 때 가장 유용합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `true`                                                                                                                          |
| `disableWorkflows`                 | **기본값**: `false`. [동적 워크플로우](/docs/ko/workflows#turn-workflows-off) 및 번들 워크플로우 명령을 비활성화합니다. `CLAUDE_CODE_DISABLE_WORKFLOWS`를 `1`로 설정하는 것과 동일합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `true`                                                                                                                          |
| `editorMode`                       | **기본값**: `"normal"`. 입력 프롬프트의 키 바인딩 모드: `"normal"` 또는 `"vim"`. `/config`에 **편집기 모드**로 표시됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `"vim"`                                                                                                                         |
| `effortLevel`                      | [노력 수준](/docs/ko/model-config#adjust-effort-level)을 세션 간에 유지합니다. `"low"`, `"medium"`, `"high"` 또는 `"xhigh"`를 허용합니다. `/effort`를 이러한 값 중 하나로 실행할 때 자동으로 작성됩니다. `--effort` 및 [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/ko/env-vars)은 한 세션에 대해 이를 재정의합니다. [노력 수준 조정](/docs/ko/model-config#adjust-effort-level)에서 지원되는 모델을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `"xhigh"`                                                                                                                       |
| `enableAllProjectMcpServers`       | 프로젝트 `.mcp.json` 파일에 정의된 모든 MCP 서버를 자동으로 승인합니다. }v2.1.196부터 `claude mcp list` 및 `claude mcp get`은 [저장소에 체크인되지 않은 설정 파일](/docs/ko/mcp#managing-your-servers)에서만 신뢰할 수 없는 폴더에서 이 키를 존중합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `true`                                                                                                                          |
| `enableArtifact`                   | }이 사용자에 대해 [Artifact](/docs/ko/artifacts) 도구를 활성화하거나 비활성화합니다. 설정되지 않으면 기본값은 계정의 기능 [가용성](/docs/ko/artifacts#availability)을 따릅니다. `/config`의 **Artifacts** 행이 이 키를 작성합니다. Managed `disableArtifact` 및 조직의 [관리자 설정](/docs/ko/artifacts#manage-artifacts-for-your-organization)이 우선하며, 키는 프로젝트 및 local 설정 (`.claude/settings.json`, `.claude/settings.local.json`)에서 무시됩니다. 저장소가 이를 커밋할 수 있기 때문입니다. Claude Code v2.1.196 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                          | `true`                                                                                                                          |
| `enabledMcpjsonServers`            | `.mcp.json` 파일에서 승인할 특정 MCP 서버 목록입니다. }v2.1.196부터 `claude mcp list` 및 `claude mcp get`은 [저장소에 체크인되지 않은 설정 파일](/docs/ko/mcp#managing-your-servers)에서만 신뢰할 수 없는 폴더에서 이 키를 존중합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `["memory", "github"]`                                                                                                          |
| `enforceAvailableModels`           | }}}Extend the `availableModels` allowlist to the Default model. When `true` in managed settings and `availableModels` is a non-empty array, the Default option falls back to the first allowlisted entry that is available, but only when the model Default would resolve to (the [organization default](/docs/ko/model-config#organization-default-model) when one applies, otherwise the account-type default) is not in the allowlist; an allowlisted default is kept as-is. Has no effect when `availableModels` is unset or empty. See [Enforce the allowlist for the Default model](/docs/ko/model-config#enforce-the-allowlist-for-the-default-model). Requires Claude Code v2.1.175 or later                                                                | `true`                                                                                                                          |
| `env`                              | 모든 세션에 적용될 환경 변수 및 Claude Code가 생성하는 하위 프로세스에 적용됩니다. 변수를 `""`로 설정하여 셸 내보내기를 빈 문자열로 재정의합니다. Claude Code는 이를 설정 해제로 처리합니다. 하위 프로세스는 여전히 빈 값을 상속합니다. `NO_COLOR` 및 `FORCE_COLOR`는 여기에 설정되면 하위 프로세스에만 도달합니다. Claude Code의 자체 인터페이스 색상을 변경하려면 Claude를 시작하기 전에 셸에서 이를 설정합니다. }v2.1.195부터 Claude Code의 호스팅 환경이 설정하는 `CLAUDE_CODE_REMOTE` 및 `CLAUDE_CODE_ACCOUNT_UUID`와 같은 ID 변수는 여기에 설정되면 무시됩니다                                                                                                                                                                                                                                                                                                                                                                   | `{"FOO": "bar"}`                                                                                                                |
| `fallbackModel`                    | 기본 모델이 과부하이거나 사용할 수 없을 때 순서대로 시도할 폴백 모델입니다. Claude Code는 턴의 나머지 부분에 대해 체인의 다음 사용 가능한 모델로 전환하고 알림을 표시합니다. `"default"`는 기본 모델로 확장됩니다. 체인은 3개 모델로 제한되며 추가 항목은 무시됩니다. 대부분의 배열 설정과 달리 이 키는 설정 파일 전체에서 병합되지 않습니다: 이를 정의하는 최고 우선순위 파일이 전체 체인을 제공합니다. [`--fallback-model`](/docs/ko/cli-reference#cli-flags) 플래그는 한 세션에 대해 이를 재정의합니다. [폴백 모델 체인](/docs/ko/model-config#fallback-model-chains)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                        | `["claude-sonnet-5", "claude-haiku-4-5"]`                                                                                       |
| `fastMode`                         | 사용 가능한 세션에 대해 [fast mode](/docs/ko/fast-mode)를 켭니다. `/fast`로 전환하면 사용자 설정에서 `true`를 작성하고 fast mode를 끌 때 키를 제거합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `true`                                                                                                                          |
| `fastModePerSessionOptIn`          | `true`일 때 fast mode는 세션 간에 지속되지 않습니다. 각 세션은 fast mode가 꺼진 상태로 시작되며 사용자가 `/fast`로 활성화해야 합니다. 사용자의 fast mode 설정은 여전히 저장됩니다. [세션별 옵트인 필요](/docs/ko/fast-mode#require-per-session-opt-in)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `true`                                                                                                                          |
| `feedbackSurveyRate`               | [세션 품질 설문조사](/docs/ko/data-usage#session-quality-surveys)가 적격일 때 나타날 확률 (0–1). 완전히 억제하려면 `0`으로 설정하거나 `env`에서 [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](/docs/ko/env-vars)를 설정합니다. Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry를 사용할 때 유용하며 기본 샘플 레이트가 적용되지 않습니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `0.05`                                                                                                                          |
| `fileCheckpointingEnabled`         | }**기본값**: `true`. 각 편집 전에 파일을 스냅샷하여 [`/rewind`](/docs/ko/checkpointing)가 이를 복원할 수 있도록 합니다. `/config`에 \*\*코드 되감기 (체크포인트)\*\*로 표시됩니다. 환경 변수로 비활성화하려면 `env`에서 [`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](/docs/ko/env-vars)를 설정합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `false`                                                                                                                         |
| `fileSuggestion`                   | `@` 파일 자동 완성을 위한 사용자 정의 스크립트를 구성합니다. [파일 제안 설정](#file-suggestion-settings)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `{"type": "command", "command": "~/.claude/file-suggestion.sh"}`                                                                |
| `footerLinksRegexes`               | }정규식이 턴 출력과 일치할 때 바닥글에 클릭 가능한 배지를 렌더링합니다. 각 항목에는 `pattern`, 명명된 캡처 그룹에서 채워진 `{name}` 자리 표시자가 있는 `url` 템플릿 및 선택적 `label`이 있습니다. 사용자, `--settings` 플래그 및 managed 설정에서만 읽습니다. [바닥글 링크 배지](#footer-link-badges)에서 URL 제약, 스키마 허용 목록 및 제한을 참조하세요. Claude Code v2.1.176 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `[{"type": "regex", "pattern": "\\b(?<key>PROJ-\\d+)\\b", "url": "https://issues.example.com/browse/{key}", "label": "{key}"}]` |
| `forceLoginMethod`                 | `claudeai`를 사용하여 Claude.ai 계정으로만 로그인을 제한하거나, `console`을 사용하여 Claude Console 계정으로만 제한하거나, `gateway`를 사용하여 클라우드 게이트웨이로만 제한합니다. [Claude apps gateway](/docs/ko/claude-apps-gateway)를 참조하세요. Managed 설정에서 설정되면 `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` 또는 `apiKeyHelper`로 인증된 세션은 시작 시 차단됩니다. 어느 값도 먼저 자사 OAuth 없이 만족할 수 없기 때문입니다. Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry와 같은 제3자 공급자 세션은 차단되지 않습니다: 이들은 Anthropic이 아닌 클라우드 공급자에 대해 인증합니다                                                                                                                                                                                                                                                                                            | `claudeai`                                                                                                                      |
| `forceLoginGatewayUrl`             | `/login` 클라우드 게이트웨이 화면에서 게이트웨이 URL을 사전 채우고 잠급니다. 이 키 또는 `forceLoginMethod: "gateway"` 중 하나가 해당 화면을 표시합니다. URL이 채워지도록 둘 다 설정합니다. Managed 정책 계층에서만 적용됩니다. 사용자 및 프로젝트 설정에서는 무시됩니다. [Claude apps gateway](/docs/ko/claude-apps-gateway#set-the-gateway-url)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `"https://claude-gateway.example.com"`                                                                                          |
| `forceLoginOrgUUID`                | 로그인이 특정 Anthropic 조직에 속하도록 요구합니다. 단일 UUID 문자열을 허용하며, 이는 로그인 중에 해당 조직을 사전 선택하거나, 나열된 조직이 사전 선택 없이 허용되는 UUID 배열을 허용합니다. Managed 설정에서 설정되면 인증된 계정이 나열된 조직에 속하지 않으면 로그인이 실패합니다. `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` 또는 `apiKeyHelper`로 인증된 세션은 시작 시 차단됩니다. 조직 멤버십을 확인할 수 없기 때문입니다. Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry와 같은 제3자 공급자 세션은 차단되지 않습니다: 클라우드 IAM을 사용하여 사용할 수 있는 클라우드 계정을 제한합니다. 빈 배열은 실패하고 잘못된 구성 메시지로 로그인을 차단합니다                                                                                                                                                                                                                                                                                              | `"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"` 또는 `["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]`  |
| `forceRemoteSettingsRefresh`       | (Managed 설정만) 원격 managed 설정이 서버에서 새로 가져올 때까지 CLI 시작을 차단합니다. 가져오기가 실패하면 캐시된 또는 설정 없이 계속하는 대신 CLI가 종료됩니다. 설정되지 않으면 시작이 원격 설정을 기다리지 않고 계속됩니다. [실패 폐쇄 적용](/docs/ko/server-managed-settings#enforce-fail-closed-startup)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `true`                                                                                                                          |
| `gcpAuthRefresh`                   | GCP Application Default Credentials가 만료되거나 로드할 수 없을 때 새로고침하는 사용자 정의 스크립트입니다. [고급 자격 증명 구성](/docs/ko/google-vertex-ai#advanced-credential-configuration)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `gcloud auth application-default login`                                                                                         |
| `hooks`                            | 라이프사이클 이벤트에서 실행할 사용자 정의 명령을 구성합니다. 형식은 [hooks 문서](/docs/ko/hooks)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | [hooks](/docs/ko/hooks) 참조                                                                                                           |
| `httpHookAllowedEnvVars`           | HTTP hooks가 헤더에 보간할 수 있는 환경 변수 이름의 허용 목록입니다. 설정되면 각 hook의 유효한 `allowedEnvVars`는 이 설정과의 교집합입니다. 정의되지 않음 = 제한 없음. 배열은 설정 소스 전체에서 병합됩니다. [Hook 구성](#hook-configuration)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `["MY_TOKEN", "HOOK_SECRET"]`                                                                                                   |
| `includeGitInstructions`           | **기본값**: `true`. Claude의 시스템 프롬프트에 기본 제공 커밋 및 PR 워크플로우 지침 및 git 상태 스냅샷을 포함합니다. 예를 들어 자신의 git 워크플로우 skills을 사용할 때 이를 `false`로 설정하여 둘 다 제거합니다. `CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS` 환경 변수가 설정되면 이 설정보다 우선합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `false`                                                                                                                         |
| `inputNeededNotifEnabled`          | }**기본값**: `false`. [Remote Control](/docs/ko/remote-control)이 연결되어 있을 때 권한 프롬프트 또는 질문이 입력을 기다리고 있을 때 휴대폰에 푸시 알림을 보냅니다. `/config`에 **작업이 필요할 때 푸시**로 표시됩니다. [모바일 푸시 알림](/docs/ko/remote-control#mobile-push-notifications)을 참조하세요. Claude Code v2.1.119 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `true`                                                                                                                          |
| `language`                         | Claude의 선호 응답 언어를 구성합니다 (예: `"japanese"`, `"spanish"`, `"french"`). Claude는 기본적으로 이 언어로 응답합니다. 또한 [음성 받아쓰기](/docs/ko/voice-dictation#change-the-dictation-language) 언어를 설정합니다. }v2.1.176부터 설정되지 않으면 세션 제목이 대화의 언어와 일치합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `"japanese"`                                                                                                                    |
| `minimumVersion`                   | 배경 자동 업데이트 및 `claude update`가 이 버전 아래로 설치되는 것을 방지하는 하한입니다. `"latest"` 채널에서 `"stable"`로 전환할 때 `/config`를 통해 현재 버전에 머물기 또는 다운그레이드를 허용하라는 메시지가 표시됩니다. 머물기를 선택하면 이 값이 설정됩니다. 또한 [managed 설정](/docs/ko/permissions#managed-settings)에서 조직 전체 최소값을 고정하는 데 유용합니다. 시작을 완전히 차단하는 하드 플로어는 `requiredMinimumVersion`을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                | `"2.1.100"`                                                                                                                     |
| `model`                            | Claude Code에 사용할 기본 모델을 재정의합니다. `--model` 및 [`ANTHROPIC_MODEL`](/docs/ko/model-config#environment-variables)은 한 세션에 대해 이를 재정의합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `"claude-sonnet-5"`                                                                                                             |
| `modelOverrides`                   | Anthropic 모델 ID를 Amazon Bedrock 추론 프로필 ARN과 같은 공급자 특정 모델 ID로 매핑합니다. 각 모델 선택기 항목은 공급자 API를 호출할 때 매핑된 값을 사용합니다. [버전별 모델 ID 재정의](/docs/ko/model-config#override-model-ids-per-version)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `{"claude-opus-4-6": "arn:aws:bedrock:..."}`                                                                                    |
| `otelHeadersHelper`                | 동적 OpenTelemetry 헤더를 생성하는 스크립트입니다. 시작 시 및 주기적으로 실행됩니다. [`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](/docs/ko/env-vars)로 새로고침 간격을 설정합니다. [동적 헤더](/docs/ko/monitoring-usage#dynamic-headers)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `/bin/generate_otel_headers.sh`                                                                                                 |
| `outputStyle`                      | 시스템 프롬프트를 조정하기 위한 출력 스타일을 구성합니다. [출력 스타일 문서](/docs/ko/output-styles)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `"Explanatory"`                                                                                                                 |
| `parentSettingsBehavior`           | }(Managed 설정만) **기본값**: `"first-wins"`. Agent SDK 또는 IDE 확장과 같은 embedding host 프로세스에 의해 프로그래밍 방식으로 제공되는 managed 설정이 관리자 배포 managed 계층도 있을 때 적용되는지 여부를 제어합니다. `"first-wins"`: 부모 제공 설정이 삭제되고 관리자 계층만 적용됩니다. `"merge"`: 부모 제공 설정이 관리자 계층 아래에 적용되며, 정책을 강화할 수 있지만 완화할 수 없도록 필터링됩니다. 관리자 계층이 배포되지 않으면 영향을 주지 않습니다. Claude Code v2.1.133 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                             | `"merge"`                                                                                                                       |
| `permissions`                      | 권한의 구조는 아래 표를 참조하세요.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |                                                                                                                                 |
| `plansDirectory`                   | **기본값**: `~/.claude/plans`. 계획 파일이 저장되는 위치를 사용자 정의합니다. 경로는 프로젝트 루트에 상대적입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `"./plans"`                                                                                                                     |
| `pluginSuggestionMarketplaces`     | (Managed 설정만) 플러그인이 상황별 설치 제안으로 나타날 수 있는 마켓플레이스 이름입니다. 제안은 각 플러그인의 마켓플레이스 항목의 `relevance` 선언에서 나옵니다. 이름은 마켓플레이스가 머신에 등록되고 등록된 소스가 managed 설정에서도 선언될 때만 적용됩니다. 해당 이름에 대한 `extraKnownMarketplaces` 항목으로 또는 `strictKnownMarketplaces`의 항목으로 선언됩니다. 허용 목록 이름 아래에 다른 소스에서 등록된 마켓플레이스는 무시됩니다. 공식 마켓플레이스는 소스 요구 사항에서 제외됩니다: 이름만 허용 목록에 있으면 충분합니다. 이름은 공식 Anthropic 소스에서만 등록될 수 있기 때문입니다.                                                                                                                                                                                                                                                                                                                                                                         | `["acme-corp-plugins"]`                                                                                                         |
| `pluginTrustMessage`               | (Managed 설정만) 설치 전에 표시되는 플러그인 신뢰 경고에 추가될 사용자 정의 메시지입니다. 이를 사용하여 조직 특정 컨텍스트를 추가합니다. 예를 들어 내부 마켓플레이스의 플러그인이 검증되었음을 확인합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `"All plugins from our marketplace are approved by IT"`                                                                         |
| `policyHelper`                     | }관리자 배포 실행 파일로 시작 시 managed 설정을 동적으로 계산합니다. MDM 또는 시스템 `managed-settings.json` 파일에서만 적용됩니다. [정책 도우미로 managed 설정 계산](#compute-managed-settings-with-a-policy-helper)을 참조하세요. Claude Code v2.1.136 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `{"path": "/usr/local/bin/claude-policy"}`                                                                                      |
| `preferredNotifChannel`            | **기본값**: `"auto"`. 작업 완료 및 권한 프롬프트 알림 방법입니다. `"auto"`, `"terminal_bell"`, `"iterm2"`, `"iterm2_with_bell"`, `"kitty"`, `"ghostty"` 또는 `"notifications_disabled"`를 허용합니다. `"auto"`는 iTerm2, Ghostty 및 Kitty에서 데스크톱 알림을 보내고 다른 터미널에서는 아무것도 하지 않습니다. 모든 터미널에서 벨 문자를 울리려면 `"terminal_bell"`을 설정합니다. `/config`에 **알림**으로 표시됩니다. [터미널 벨 또는 알림 받기](/docs/ko/terminal-config#get-a-terminal-bell-or-notification)를 참조하세요                                                                                                                                                                                                                                                                                                                                               | `"terminal_bell"`                                                                                                               |
| `prefersReducedMotion`             | 접근성을 위해 UI 애니메이션 (스피너, shimmer, flash 효과) 감소 또는 비활성화                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `true`                                                                                                                          |
| `prUrlTemplate`                    | PR 배지에 대한 URL 템플릿으로 바닥글 및 도구 결과 요약에 표시됩니다. `gh`에서 보고한 PR URL에서 `{host}`, `{owner}`, `{repo}`, `{number}` 및 `{url}`을 대체합니다. `github.com` 대신 내부 코드 검토 도구를 가리키도록 사용합니다. Claude의 산문에서 `#123` 자동 링크에는 영향을 주지 않습니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `"https://reviews.example.com/{owner}/{repo}/pull/{number}"`                                                                    |
| `remoteControlAtStartup`           | }각 대화형 세션이 시작될 때 [Remote Control](/docs/ko/remote-control)을 자동으로 연결합니다. `/remote-control`을 기다리는 대신 `true`로 설정하여 항상 자동 연결하거나, `false`로 설정하여 절대 자동 연결하지 않거나, 조직의 기본값을 따르려면 설정 해제합니다. `/config`에 **모든 세션에 대해 Remote Control 활성화**로 표시됩니다. [모든 세션에 대해 Remote Control 활성화](/docs/ko/remote-control#enable-remote-control-for-all-sessions)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                        | `false`                                                                                                                         |
| `requiredMaximumVersion`           | Managed 설정만. 시작할 수 있는 최대 Claude Code 버전입니다. 실행 중인 버전이 더 최신이면 Claude Code는 시작 시 종료되고 사용자에게 조직의 승인된 방법을 통해 승인된 버전을 설치하도록 지시합니다. `claude install <version>`도 작동할 수 있습니다. 배경 자동 업데이트 및 `claude update`는 천장 위의 버전을 건너뜁니다. 범위 내 설치는 범위 내로 유지됩니다. `claude update`, `claude install` 및 `claude doctor`는 사용자가 복구할 수 있도록 천장 위에서 계속 작동합니다. 이 설정보다 먼저 나온 버전은 무시합니다                                                                                                                                                                                                                                                                                                                                                                                                    | `"2.1.150"`                                                                                                                     |
| `requiredMinimumVersion`           | Managed 설정만. 시작하는 데 필요한 최소 Claude Code 버전입니다. 실행 중인 버전이 더 오래되면 Claude Code는 시작 시 종료되고 사용자에게 조직의 승인된 방법을 통해 업데이트하도록 지시합니다. `claude update`, `claude install` 및 `claude doctor`는 사용자가 복구할 수 있도록 바닥 아래에서 계속 작동합니다. 시작을 차단하지 않지만 다운그레이드를 방지하는 `minimumVersion`과 다릅니다. 이 설정보다 먼저 나온 버전은 무시합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `"2.1.150"`                                                                                                                     |
| `respectGitignore`                 | **기본값**: `true`. `@` 파일 선택기가 `.gitignore` 패턴을 존중할지 여부를 제어합니다. `true`일 때 `.gitignore` 패턴과 일치하는 파일은 제안에서 제외됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `false`                                                                                                                         |
| `respondToBashCommands`            | }**기본값**: `true`. 입력 상자 `!` 셸 명령이 실행된 후 Claude가 응답할지 여부입니다. 응답 없이 명령 출력을 컨텍스트에 추가하려면 `false`로 설정합니다. [접두사 `!`를 사용한 셸 모드](/docs/ko/interactive-mode#shell-mode-with-prefix)를 참조하세요. Claude Code v2.1.186 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `false`                                                                                                                         |
| `showClearContextOnPlanAccept`     | **기본값**: `false`. 계획 수락 화면에서 "컨텍스트 지우기" 옵션을 표시합니다. 옵션을 복원하려면 `true`로 설정합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `true`                                                                                                                          |
| `showThinkingSummaries`            | **기본값**: `false`. 대화형 세션에서 [확장 사고](/docs/ko/model-config#extended-thinking) 요약을 표시합니다. 설정되지 않거나 `false`일 때 사고 블록은 API에 의해 편집되고 축소된 스텁으로 표시됩니다. 편집은 표시되는 내용만 변경하고 모델이 생성하는 내용은 변경하지 않습니다. 사고 지출을 줄이려면 [예산을 낮추거나 사고를 비활성화](/docs/ko/model-config#extended-thinking)하세요. 이 설정은 비대화형 모드 (`-p`), Agent SDK 또는 VS Code와 같은 IDE 확장에서는 영향을 주지 않습니다                                                                                                                                                                                                                                                                                                                                                                                                                          | `true`                                                                                                                          |
| `showTurnDuration`                 | **기본값**: `true`. 응답 후 턴 지속 시간 메시지를 표시합니다 (예: "Cooked for 1m 6s"). `/config`에 **턴 지속 시간 표시**로 표시됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `false`                                                                                                                         |
| `skillListingBudgetFraction`       | **기본값**: `0.01`. Claude가 각 턴에 보는 [skill 목록](/docs/ko/skills#skill-descriptions-are-cut-short)을 위해 예약된 모델의 컨텍스트 윈도우의 분수입니다. 목록이 예산을 초과하면 가장 적게 사용되는 skills의 설명이 베어 이름으로 축소되어 Claude가 여전히 호출할 수 있지만 이유를 보지 못합니다. 더 많은 설명을 보이려면 높이고 턴당 더 많은 컨텍스트를 사용합니다. 더 많은 skills을 [`skillListingMaxDescChars`](#available-settings) 아래에 맞추려면 낮춥니다                                                                                                                                                                                                                                                                                                                                                                                                                             | `0.02`                                                                                                                          |
| `skillListingMaxDescChars`         | **기본값**: `1536`. Claude가 각 턴에 보는 [skill 목록](/docs/ko/skills#skill-descriptions-are-cut-short)의 결합된 `description` 및 `when_to_use` 텍스트에 대한 skill별 문자 제한입니다. 이 길이보다 긴 텍스트는 잘립니다. 더 긴 설명을 유지하려면 높이고 턴당 더 많은 컨텍스트를 사용합니다. 더 많은 skills을 [`skillListingBudgetFraction`](#available-settings)에 맞추려면 낮춥니다                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `2048`                                                                                                                          |
| `skillOverrides`                   | }skill 이름으로 키가 지정된 skill별 가시성 재정의입니다. 값은 `"on"`, `"name-only"`, `"user-invocable-only"` 또는 `"off"`입니다. SKILL.md를 편집하지 않고 skill을 숨기거나 축소할 수 있습니다. 플러그인 skills에는 적용되지 않으며, 이는 `/plugin`을 통해 관리됩니다. `/skills` 메뉴는 이를 `.claude/settings.local.json`에 작성합니다. [설정에서 skill 가시성 재정의](/docs/ko/skills#override-skill-visibility-from-settings)를 참조하세요. Claude Code v2.1.129 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                   | `{"legacy-context": "name-only", "deploy": "off"}`                                                                              |
| `skipWebFetchPreflight`            | [WebFetch 도메인 안전 검사](/docs/ko/data-usage#webfetch-domain-safety-check)를 건너뜁니다. 이 검사는 각 요청된 호스트명을 가져오기 전에 `api.anthropic.com`으로 전송합니다. Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry 배포와 같이 Anthropic으로의 트래픽을 차단하는 환경에서 `true`로 설정합니다. 건너뛰면 WebFetch는 차단 목록을 참조하지 않고 모든 URL을 시도합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `true`                                                                                                                          |
| `spinnerTipsEnabled`               | **기본값**: `true`. Claude가 작업 중일 때 스피너에 팁을 표시합니다. 팁을 비활성화하려면 `false`로 설정합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `false`                                                                                                                         |
| `spinnerTipsOverride`              | 사용자 정의 문자열로 스피너 팁을 재정의합니다. `tips`: 팁 문자열 배열. `excludeDefault`: `true`이면 사용자 정의 팁만 표시하고, `false`이거나 없으면 사용자 정의 팁이 기본 제공 팁과 병합됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `{ "excludeDefault": true, "tips": ["Use our internal tool X"] }`                                                               |
| `spinnerVerbs`                     | 스피너에 표시되는 작업 동사를 사용자 정의합니다. `mode`를 `"replace"`로 설정하여 동사만 사용하거나 `"append"`로 설정하여 기본값에 추가합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `{"mode": "append", "verbs": ["Pondering", "Crafting"]}`                                                                        |
| `sshConfigs`                       | [Desktop](/docs/ko/desktop#pre-configure-ssh-connections-for-your-team) 환경 드롭다운에 표시할 SSH 연결입니다. 각 항목에는 `id`, `name` 및 `sshHost`가 필요하며, `sshPort`, `sshIdentityFile` 및 `startDirectory`는 선택 사항입니다. Managed 설정에서 설정되면 연결은 사용자에게 읽기 전용입니다. Managed 및 사용자 설정에서만 읽음                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `[{"id": "dev-vm", "name": "Dev VM", "sshHost": "user@dev.example.com"}]`                                                       |
| `statusLine`                       | 컨텍스트를 표시하기 위한 사용자 정의 상태 줄을 구성합니다. 객체의 선택적 `padding`, `refreshInterval` 및 `hideVimModeIndicator` 필드는 간격, 주기적 재실행 및 프롬프트 아래의 기본 제공 vim 모드 표시기 숨김 여부를 제어합니다. [`statusLine` 문서](/docs/ko/statusline#manually-configure-a-status-line)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `{"type": "command", "command": "~/.claude/statusline.sh"}`                                                                     |
| `strictKnownMarketplaces`          | (Managed 설정만) 플러그인 마켓플레이스 소스의 허용 목록입니다. 정의되지 않음 = 제한 없음, 빈 배열 = 잠금. 마켓플레이스 추가 및 플러그인 설치, 업데이트, 새로고침 및 자동 업데이트에 적용되므로 정책이 설정되기 전에 추가된 마켓플레이스는 플러그인을 가져오는 데 사용할 수 없습니다. [Managed 마켓플레이스 제한](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `[{ "source": "github", "repo": "acme-corp/plugins" }]`                                                                         |
| `strictPluginOnlyCustomization`    | (Managed 설정만) 플러그인 또는 managed 설정에서만 올 수 있도록 사용자 및 프로젝트 소스에서 skills, agents, hooks 및 MCP 서버를 차단합니다. `true`는 네 가지 모두를 잠그고, 배열은 명명된 것만 잠급니다. [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `["skills", "hooks"]`                                                                                                           |
| `syntaxHighlightingDisabled`       | diffs, 코드 블록 및 파일 미리보기에서 구문 강조 비활성화                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `true`                                                                                                                          |
| `teammateMode`                     | **기본값**: `in-process`. [에이전트 팀](/docs/ko/agent-teams) 팀원이 표시되는 방식입니다. `in-process`, `auto` (tmux 또는 iTerm2에서 분할 창 선택, 그 외에는 in-process), `tmux` (tmux 또는 iTerm2를 사용하여 분할 창 선택, 터미널에서 감지됨) 또는 }`iterm2` (iTerm2 기본 분할 창 via `it2` CLI, v2.1.186에서 추가됨)를 허용합니다. 기본값은 v2.1.179에서 `auto`에서 변경되었습니다. `--teammate-mode`은 한 세션에 대해 이를 재정의합니다. [디스플레이 모드 선택](/docs/ko/agent-teams#choose-a-display-mode)을 참조하세요                                                                                                                                                                                                                                                                                                                                                             | `"auto"`                                                                                                                        |
| `terminalProgressBarEnabled`       | **기본값**: `true`. 지원되는 터미널에서 터미널 진행률 표시줄을 표시합니다: ConEmu, Ghostty 1.2.0+ 및 iTerm2 3.6.6+. `/config`에 **터미널 진행률 표시줄**로 표시됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `false`                                                                                                                         |
| `theme`                            | }**기본값**: `"dark"`. 인터페이스의 색상 테마입니다. `"auto"`, `"dark"`, `"light"`, `"dark-daltonized"`, `"light-daltonized"`, `"dark-ansi"`, `"light-ansi"` 또는 `"custom:<slug>"` 또는 `"custom:<plugin-name>:<slug>"`과 같은 사용자 정의 테마 참조를 허용합니다. [사용자 정의 테마 만들기](/docs/ko/terminal-config#create-a-custom-theme)를 참조하세요. `/config`에 **테마**로 표시됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                 | `"dark"`                                                                                                                        |
| `tui`                              | 터미널 UI 렌더러입니다. 깜박임 없는 [alt-screen 렌더러](/docs/ko/fullscreen)가 있는 가상화된 스크롤백을 위해 `"fullscreen"`을 사용합니다. 클래식 메인 화면 렌더러를 위해 `"default"`를 사용합니다. `/tui`를 통해 설정합니다. [`CLAUDE_CODE_NO_FLICKER`](/docs/ko/env-vars) 환경 변수도 설정할 수 있습니다. [에이전트 보기](/docs/ko/agent-view)에서 열린 배경 세션은 이 설정과 관계없이 항상 fullscreen 렌더러를 사용합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `"fullscreen"`                                                                                                                  |
| `ultracode`                        | 세션에 대해 [ultracode](/docs/ko/workflows#let-claude-decide-with-ultracode)를 켭니다. 세션 전용이며 `settings.json`에서 읽지 않습니다. `/effort ultracode`, `--settings` 또는 Agent SDK 제어 요청을 통해 설정합니다. }ultracode를 이미 켜진 상태로 세션을 시작하려면 `claude --effort ultracode`로 시작합니다. Claude Code v2.1.203 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `true`                                                                                                                          |
| `useAutoModeDuringPlan`            | **기본값**: `true`. 자동 모드를 사용할 수 있을 때 계획 모드가 자동 모드 의미론을 사용할지 여부입니다. 공유 프로젝트 설정에서는 읽지 않음. `/config`에 "계획 중 자동 모드 사용"으로 표시됨                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `false`                                                                                                                         |
| `verbose`                          | }**기본값**: `false`. 잘린 요약 대신 전체 도구 출력을 표시합니다. `/config`에 **상세 출력**으로 표시됩니다. `--verbose` 플래그는 한 세션에 대해 이를 재정의합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `true`                                                                                                                          |
| `viewMode`                         | 시작 시 기본 트랜스크립트 보기 모드입니다. `"default"`, `"verbose"` 또는 `"focus"`를 허용합니다. 설정되면 sticky `/focus` 선택을 재정의합니다. `--verbose` 플래그는 한 세션에 대해 이를 재정의합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `"verbose"`                                                                                                                     |
| `vimInsertModeRemaps`              | }[vim 편집기 모드](/docs/ko/interactive-mode#vim-editor-mode)에서 Escape로 두 키 INSERT 모드 시퀀스를 매핑합니다. 각 키는 정확히 두 개의 인쇄 가능한 문자이며 순서대로 입력되고 `"<Esc>"`는 유일하게 지원되는 대상입니다. 다른 항목은 무시됩니다. 사용자, `--settings` 플래그 및 managed 설정에서만 읽습니다. 저장소의 체크인된 설정이 키 입력을 다시 매핑할 수 없습니다. `editorMode`가 `"vim"`이 아니면 영향을 주지 않습니다. [INSERT 모드 키 시퀀스 다시 매핑](/docs/ko/interactive-mode#remap-insert-mode-key-sequences)을 참조하세요. Claude Code v2.1.208 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                         | `{"jj": "<Esc>"}`                                                                                                               |
| `voice`                            | [음성 받아쓰기](/docs/ko/voice-dictation) 설정입니다. `enabled`는 받아쓰기를 켜고, `mode`는 `"hold"` 또는 `"tap"`을 선택하고, `autoSubmit`은 hold 모드에서 키 릴리스 시 프롬프트를 전송합니다. `/voice`를 실행할 때 자동으로 작성됩니다. Claude.ai 계정이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `{ "enabled": true, "mode": "tap" }`                                                                                            |
| `voiceEnabled`                     | `voice.enabled`에 대한 레거시 별칭입니다. `voice` 객체를 선호합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `true`                                                                                                                          |
| `wheelScrollAccelerationEnabled`   | }**기본값**: `true`. [fullscreen 렌더링](/docs/ko/fullscreen#mouse-wheel-scrolling)에서 빠른 스크롤 중에 마우스 휠 스크롤 속도를 가속화합니다. 휠 노치당 일정한 스크롤 속도를 원하면 `false`로 설정합니다. Claude Code v2.1.174 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `false`                                                                                                                         |
| `workflowKeywordTriggerEnabled`    | }**기본값**: `true`. 프롬프트의 단어 `ultracode`이 [동적 워크플로우](/docs/ko/workflows#ask-for-a-workflow-in-your-prompt)를 트리거할지 여부입니다. 하나를 트리거하지 않고 단어를 입력하려면 `false`로 설정합니다. `ultracode` 노력 설정, `/workflows` 및 저장된 워크플로우 명령은 영향을 받지 않습니다. `/config`에 **Ultracode 키워드 트리거**로 표시됩니다. v2.1.157에서 추가됨; v2.1.160 이전에 트리거 키워드는 `workflow`였습니다                                                                                                                                                                                                                                                                                                                                                                                                                                       | `false`                                                                                                                         |
| `wslInheritsWindowsSettings`       | (Windows managed 설정만) `true`일 때 WSL의 Claude Code는 `/etc/claude-code`에 추가하여 Windows 정책 체인에서 managed 설정을 읽으며 Windows 소스가 우선합니다. Windows 관리자가 작성해야 하는 HKLM 레지스트리 키 또는 `C:\Program Files\ClaudeCode\managed-settings.json`에서 설정된 경우에만 적용됩니다. HKCU 정책도 WSL에 적용되려면 플래그를 HKCU 자체에도 설정해야 합니다. 기본 Windows에는 영향을 주지 않습니다                                                                                                                                                                                                                                                                                                                                                                                                                                            | `true`                                                                                                                          |

<h3 id="global-config-settings">
  전역 구성 설정
</h3>

이러한 설정은 `settings.json`이 아닌 `~/.claude.json`에 저장됩니다. 이들을 `settings.json`에 추가하면 스키마 검증 오류가 발생합니다.

<Note>
  v2.1.119 이전 버전은 `theme`, `verbose`, `editorMode`, `autoCompactEnabled` 및 `preferredNotifChannel`을 포함한 여러 `/config` 설정 키를 `settings.json` 대신 여기에 저장합니다.
</Note>

| 키                            | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                            | 예제         |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------- |
| `autoConnectIde`             | **기본값**: `false`. Claude Code가 외부 터미널에서 시작될 때 실행 중인 IDE에 자동으로 연결합니다. VS Code 또는 JetBrains 터미널 외부에서 실행할 때 `/config`에 \*\*IDE에 자동 연결 (외부 터미널)\*\*로 표시됩니다. [`CLAUDE_CODE_AUTO_CONNECT_IDE`](/docs/ko/env-vars) 환경 변수가 설정되면 이를 재정의합니다                                                                                                                                                                                                                                  | `true`     |
| `autoInstallIdeExtension`    | **기본값**: `true`. VS Code 터미널에서 실행할 때 Claude Code IDE 확장을 자동으로 설치합니다. VS Code 또는 JetBrains 터미널 내에서 실행할 때 `/config`에 **IDE 확장 자동 설치**로 표시됩니다. [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/ko/env-vars) 환경 변수도 설정할 수 있습니다                                                                                                                                                                                                                                             | `false`    |
| `externalEditorContext`      | **기본값**: `false`. `Ctrl+G`로 외부 편집기를 열 때 Claude의 이전 응답을 `#` 주석 처리된 컨텍스트로 앞에 붙입니다. `/config`에 **외부 편집기에 마지막 응답 표시**로 표시됩니다                                                                                                                                                                                                                                                                                                                                      | `true`     |
| `permissionExplainerEnabled` | **기본값**: `true`. Bash 또는 PowerShell 권한 프롬프트에서 `Ctrl+E`를 누를 때 모델 생성 [명령 설명](/docs/ko/permissions#permission-system)을 표시합니다. 바로 가기를 끄려면 `false`로 설정합니다                                                                                                                                                                                                                                                                                                               | `false`    |
| `teammateDefaultModel`       | [에이전트 팀](/docs/ko/agent-teams) 팀원을 위한 기본 모델로 spawn 프롬프트가 하나를 지정하지 않을 때 사용됩니다. `"sonnet"`과 같은 모델 별칭으로 설정하거나 lead의 현재 `/model` 선택을 상속하려면 `null`로 설정합니다. `/config`에 **기본 팀원 모델**로 표시됩니다                                                                                                                                                                                                                                                                               | `"sonnet"` |
| `workflowSizeGuideline`      | }**기본값**: `unrestricted`, 이는 지침을 보내지 않습니다. 동적 워크플로우가 작성하는 [에이전트 수를 목표로 설정](/docs/ko/workflows#set-a-size-guideline)합니다. Claude Code는 값을 Claude에 조언으로 보내며 적용된 상한이 아닙니다. `unrestricted`, `small`, `medium` 또는 `large`를 허용합니다. `/config`에 **동적 워크플로우 크기**로 표시됩니다. `/config workflowSizeGuideline=small`으로 직접 설정할 수도 있습니다. Claude Code v2.1.202 이상이 필요합니다. }지침의 에이전트 수는 [`Large workflow` 경고](/docs/ko/workflows#cost)의 기본 임계값도 대체합니다. 이 동작은 Claude Code v2.1.203 이상이 필요합니다 | `"small"`  |

<h3 id="worktree-settings">
  Worktree 설정
</h3>

`--worktree`가 git worktrees를 생성하고 관리하는 방식을 구성합니다.

| 키                             | 설명                                                                                                                                                                                                                                                                                                                                                                                  | 예제                                    |
| :---------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------ |
| `worktree.baseRef`            | 새 worktrees가 분기하는 ref입니다. `"fresh"` (기본값)은 깨끗한 트리와 일치하는 원격에 대해 `origin/<default-branch>`에서 분기합니다. `"head"`는 현재 local `HEAD`에서 분기하므로 푸시되지 않은 커밋 및 feature-branch 상태가 worktree에 있습니다. 연결된 worktree 내에서 `"head"`는 해당 worktree의 `HEAD`로 해석되며 메인 체크아웃의 것이 아닙니다. `--worktree`, `EnterWorktree` 도구 및 subagent 격리에 적용됩니다                                                                    | `"head"`                              |
| `worktree.symlinkDirectories` | 각 worktree에서 중복을 피하기 위해 메인 저장소에서 symlink할 디렉토리입니다. 기본적으로 디렉토리는 symlink되지 않습니다                                                                                                                                                                                                                                                                                                       | `["node_modules", ".cache"]`          |
| `worktree.sparsePaths`        | git sparse-checkout을 통해 각 worktree에서 체크아웃할 디렉토리입니다. 나열된 경로만 디스크에 작성되므로 대규모 monorepos에서 더 빠릅니다. sparse worktree가 존재하는 동안 git은 저장소의 공유 `.git/config`에서 `extensions.worktreeConfig`를 활성화합니다. [필요한 디렉토리만 체크아웃](/docs/ko/large-codebases#check-out-only-the-directories-you-need)을 참조하세요                                                                                                      | `["packages/my-app", "shared/utils"]` |
| `worktree.bgIsolation`        | }[배경 세션](/docs/ko/agent-view#how-file-edits-are-isolated)의 격리 모드입니다. `"worktree"` (기본값)은 `EnterWorktree`가 호출될 때까지 메인 체크아웃에서 `Edit`/`Write`를 차단합니다. }git 저장소 외부에서 실패하는 [`WorktreeCreate` hook](/docs/ko/worktrees#non-git-version-control)이 블록을 해제하여 세션이 작업 디렉토리를 제자리에서 편집할 수 있도록 합니다. Claude Code v2.1.203 이상이 필요합니다. `"none"`은 배경 작업이 작업 복사본을 직접 편집하도록 허용합니다. Claude Code v2.1.143 이상이 필요합니다 | `"none"`                              |

worktrees에 `.env`와 같은 gitignored 파일을 복사하려면 설정 대신 프로젝트 루트의 [`.worktreeinclude` 파일](/docs/ko/worktrees#copy-gitignored-files-into-worktrees)을 사용합니다.

<h3 id="permission-settings">
  권한 설정
</h3>

| 키                                   | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | 예제                                                                     |
| :---------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `allow`                             | 도구 사용을 허용하는 권한 규칙 배열입니다. 도구 이름 globs는 `mcp__<server>__get_*`과 같이 리터럴 `mcp__<server>__` 접두사 뒤의 도구 위치에서만 지원됩니다. 서버 세그먼트는 glob 없어야 합니다. 패턴 매칭 세부 사항은 아래 [권한 규칙 구문](#permission-rule-syntax)을 참조하세요                                                                                                                                                                                                                                                                                          | `[ "Bash(git diff *)" ]`                                               |
| `ask`                               | 도구 사용 시 확인을 요청하는 권한 규칙 배열입니다. 패턴 매칭 세부 사항은 아래 [권한 규칙 구문](#permission-rule-syntax)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                 | `[ "Bash(git push *)" ]`                                               |
| `deny`                              | 도구 사용을 거부하는 권한 규칙 배열입니다. 이를 사용하여 Claude Code 액세스에서 민감한 파일을 제외합니다. 도구 이름은 glob 패턴을 허용합니다: `"*"`는 모든 도구를 거부하고 `"mcp__*"`는 모든 MCP 도구를 거부합니다. [권한 규칙 구문](#permission-rule-syntax) 및 [Bash 권한 제한](/docs/ko/permissions#tool-specific-permission-rules)을 참조하세요                                                                                                                                                                                                                                      | `[ "WebFetch", "Bash(curl *)", "Read(./.env)", "Read(./secrets/**)" ]` |
| `additionalDirectories`             | Claude가 액세스할 수 있는 추가 [작업 디렉토리](/docs/ko/permissions#working-directories)입니다. 대부분의 `.claude/` 구성은 이러한 디렉토리에서 [발견되지 않습니다](/docs/ko/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                           | `[ "../docs/" ]`                                                       |
| `defaultMode`                       | Claude Code를 열 때 기본 [권한 모드](/docs/ko/permission-modes)입니다. 유효한 값: `default`, `acceptEdits`, `plan`, `auto`, `dontAsk`, `bypassPermissions`, 그리고 }`manual` (CLI 및 VS Code와 JetBrains 확장에서 Manual로 표시되는 모드의 별칭). `manual` 별칭은 Claude Code v2.1.200 이상이 필요합니다. }v2.1.142부터 프로젝트 또는 local 설정 (`.claude/settings.json`, `.claude/settings.local.json`)에서 설정되면 `auto`가 무시되므로 저장소가 자신에게 자동 모드를 부여할 수 없습니다. 대신 `~/.claude/settings.json`에서 설정합니다. `--permission-mode` CLI 플래그는 단일 세션에 대해 이 설정을 재정의합니다 | `"acceptEdits"`                                                        |
| `disableBypassPermissionsMode`      | `bypassPermissions` 모드가 활성화되는 것을 방지하려면 `"disable"`로 설정합니다. 이는 `--dangerously-skip-permissions` 명령줄 플래그를 비활성화합니다. 일반적으로 [managed 설정](/docs/ko/permissions#managed-settings)에 배치되어 조직 정책을 적용하지만 모든 범위에서 작동합니다                                                                                                                                                                                                                                                                                   | `"disable"`                                                            |
| `skipDangerousModePermissionPrompt` | `--dangerously-skip-permissions` 또는 `defaultMode: "bypassPermissions"`를 통해 bypass permissions 모드에 들어가기 전에 표시되는 확인 프롬프트를 건너뜁니다. 신뢰할 수 없는 저장소가 프롬프트를 자동으로 우회하는 것을 방지하기 위해 프로젝트 설정 (`.claude/settings.json`)에서 설정되면 무시됩니다                                                                                                                                                                                                                                                                   | `true`                                                                 |

<h3 id="permission-rule-syntax">
  권한 규칙 구문
</h3>

권한 규칙은 `Tool` 또는 `Tool(specifier)` 형식을 따릅니다. 규칙은 순서대로 평가됩니다: 먼저 거부 규칙, 그 다음 요청, 그 다음 허용. 첫 번째 일치 규칙이 우승합니다. 규칙 특이성과 관계없이 결과를 결정합니다. [권한 규칙 평가 순서](/docs/ko/permissions#manage-permissions)를 참조하세요.

빠른 예제:

| 규칙                             | 효과                           |
| :----------------------------- | :--------------------------- |
| `Bash`                         | 모든 Bash 명령과 일치               |
| `Bash(npm run *)`              | `npm run`으로 시작하는 명령과 일치      |
| `Read(./.env)`                 | `.env` 파일 읽기와 일치             |
| `WebFetch(domain:example.com)` | example.com에 대한 fetch 요청과 일치 |

Read, Edit, WebFetch, MCP 및 Agent 규칙에 대한 와일드카드 동작, 도구 특정 패턴 및 Bash 패턴의 보안 제한을 포함한 완전한 규칙 구문 참조는 [권한 규칙 구문](/docs/ko/permissions#permission-rule-syntax)을 참조하세요.

<h3 id="sandbox-settings">
  Sandbox 설정
</h3>

고급 샌드박싱 동작을 구성합니다. 샌드박싱은 bash 명령을 파일 시스템 및 네트워크에서 격리합니다. 자세한 내용은 [Sandboxing](/docs/ko/sandboxing)을 참조하세요.

| 키                                      | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | 예제                                                   |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `enabled`                              | bash 샌드박싱 활성화 (macOS, Linux 및 WSL2). 기본값: false                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `true`                                               |
| `failIfUnavailable`                    | `sandbox.enabled`가 true이지만 샌드박스를 시작할 수 없는 경우 (종속성 누락, 지원되지 않는 플랫폼) 시작 시 오류로 종료합니다. false (기본값)일 때 경고가 표시되고 명령이 샌드박싱되지 않은 상태로 실행됩니다. Managed 설정 배포에서 샌드박싱을 하드 게이트로 요구하는 경우를 위한 것입니다                                                                                                                                                                                                                                                                                                                                                                                | `true`                                               |
| `autoAllowBashIfSandboxed`             | 샌드박싱되면 bash 명령 자동 승인. 기본값: true                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `true`                                               |
| `excludedCommands`                     | 샌드박스 외부에서 실행해야 하는 명령                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `["docker *"]`                                       |
| `allowUnsandboxedCommands`             | `dangerouslyDisableSandbox` 매개변수를 통해 샌드박스 외부에서 명령을 실행하도록 허용합니다. `false`로 설정되면 `dangerouslyDisableSandbox` 이스케이프 해치가 완전히 비활성화되고 모든 명령은 샌드박싱되거나 `excludedCommands`에 있어야 합니다. 엄격한 샌드박싱이 필요한 엔터프라이즈 정책에 유용합니다. 기본값: true                                                                                                                                                                                                                                                                                                                                              | `false`                                              |
| `filesystem.allowWrite`                | 샌드박싱된 명령이 쓸 수 있는 추가 경로입니다. 배열은 모든 설정 범위에서 병합됩니다: 사용자, 프로젝트 및 managed 경로가 결합되고 대체되지 않습니다. `Edit(...)` 허용 권한 규칙의 경로와도 병합됩니다. [경로 접두사](#sandbox-path-prefixes)를 참조하세요.                                                                                                                                                                                                                                                                                                                                                                                               | `["/tmp/build", "~/.kube"]`                          |
| `filesystem.denyWrite`                 | 샌드박싱된 명령이 쓸 수 없는 경로입니다. 배열은 모든 설정 범위에서 병합됩니다. `Edit(...)` 거부 권한 규칙의 경로와도 병합됩니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `["/etc", "/usr/local/bin"]`                         |
| `filesystem.denyRead`                  | 샌드박싱된 명령이 읽을 수 없는 경로입니다. 배열은 모든 설정 범위에서 병합됩니다. `Read(...)` 거부 권한 규칙의 경로와도 병합됩니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `["~/.aws/credentials"]`                             |
| `filesystem.allowRead`                 | `denyRead` 영역 내에서 읽기를 다시 허용할 경로입니다. `denyRead`보다 우선합니다. 배열은 모든 설정 범위에서 병합됩니다. 이를 사용하여 작업 공간 전용 읽기 액세스 패턴을 만듭니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `["."]`                                              |
| `filesystem.allowManagedReadPathsOnly` | (Managed 설정만) Managed 설정의 `filesystem.allowRead` 경로만 존중됩니다. `denyRead`는 여전히 모든 소스에서 병합됩니다. 기본값: false                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `true`                                               |
| `credentials.files`                    | }샌드박싱된 명령이 읽을 수 없는 자격 증명 파일 또는 디렉토리입니다. `filesystem.denyRead`와 동일한 읽기 블록을 적용합니다. 별도의 키는 자격 증명 경로를 `credentials.envVars`와 함께 그룹화하고 일반 파일 시스템 규칙과 분리합니다. 각 항목은 `{ "path": "...", "mode": "deny" }`입니다. 경로는 `filesystem.*` 설정과 동일한 [접두사](#sandbox-path-prefixes)를 사용합니다. 배열은 모든 설정 범위에서 병합됩니다. `deny`만 지원됩니다. Claude Code v2.1.187 이상이 필요합니다.                                                                                                                                                                                                                        | `[{ "path": "~/.aws/credentials", "mode": "deny" }]` |
| `credentials.envVars`                  | }[샌드박싱된 명령에서 보호](/docs/ko/sandboxing#protect-credentials)할 환경 변수입니다. 각 항목에는 `name` 및 `mode`가 있습니다. 이름은 문자 또는 밑줄로 시작하고 문자, 숫자 및 밑줄만 포함해야 합니다. `deny`는 샌드박싱된 명령의 환경에서 변수를 제거합니다. Claude Code v2.1.187 이상이 필요합니다. }`mask`는 샌드박스 내에서 변수를 세션별 sentinel 값으로 바꾸고 샌드박스 프록시는 해당 항목의 `injectHosts`에 대한 아웃바운드 요청에서 실제 값을 대체합니다. `network.tlsTerminate`가 필요하고 Claude Code v2.1.199 이상이 필요합니다. `mask` 항목은 사용자, managed 또는 CLI `--settings` 설정에서만 적용되며 `.claude/settings.json` 또는 `.claude/settings.local.json`에서는 적용되지 않습니다. 배열은 모든 설정 범위에서 병합되고 동일한 변수가 두 모드로 나타날 때 `deny`가 우선합니다. | `[{ "name": "GITHUB_TOKEN", "mode": "deny" }]`       |
| `credentials.envVars[].injectHosts`    | 샌드박스 프록시가 `mask` 항목의 실제 값을 대체할 호스트입니다. 각 호스트는 `network.allowedDomains`에서도 정확히 또는 와일드카드로 포함되어야 합니다. 설정되지 않으면 프록시가 `network.allowedDomains`의 모든 호스트에 대해 값을 대체합니다. `mode`가 `deny`일 때 허용되지만 무시됩니다. Claude Code v2.1.199 이상이 필요합니다. }                                                                                                                                                                                                                                                                                                                                  | `["api.github.com"]`                                 |
| `credentials.allowPlaintextInject`     | 일반 HTTP 요청뿐만 아니라 TLS 종료 HTTPS에서 `mask` 대체를 허용합니다. 일반 HTTP에서 업스트림 ID는 확인되지 않고 자격 증명은 평문으로 이동하므로 신뢰할 수 있는 테스트 네트워크 외부에서는 이를 끕니다. 사용자, managed 또는 CLI `--settings` 설정에서만 적용되며 `.claude/settings.json` 또는 `.claude/settings.local.json`에서는 적용되지 않습니다. 기본값: false. Claude Code v2.1.199 이상이 필요합니다. }                                                                                                                                                                                                                                                                   | `true`                                               |
| `network.allowUnixSockets`             | (macOS만) Unix 소켓 경로 샌드박스에서 액세스 가능. Linux 및 WSL2에서는 무시되며, seccomp 필터가 소켓 경로를 검사할 수 없습니다. 대신 `allowAllUnixSockets`를 사용합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                          | `["~/.ssh/agent-socket"]`                            |
| `network.allowAllUnixSockets`          | 샌드박스에서 모든 Unix 소켓 연결을 허용합니다. Linux 및 WSL2에서 이는 `socket(AF_UNIX, ...)` 호출을 차단하는 seccomp 필터를 건너뛰므로 Unix 소켓을 허용하는 유일한 방법입니다. 기본값: false                                                                                                                                                                                                                                                                                                                                                                                                                              | `true`                                               |
| `network.allowLocalBinding`            | localhost 포트에 바인딩 허용 (macOS만). 기본값: false                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `true`                                               |
| `network.allowMachLookup`              | 샌드박스가 조회할 수 있는 추가 XPC/Mach 서비스 이름 (macOS만). 접두사 일치를 위해 단일 후행 `*`를 지원합니다. iOS Simulator 또는 Playwright와 같이 XPC를 통해 통신하는 도구에 필요합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                  | `["com.apple.coresimulator.*"]`                      |
| `network.allowedDomains`               | 아웃바운드 네트워크 트래픽을 허용할 도메인 배열입니다. 와일드카드를 지원합니다 (예: `*.example.com`).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `["github.com", "*.npmjs.org"]`                      |
| `network.deniedDomains`                | 아웃바운드 네트워크 트래픽을 차단할 도메인 배열입니다. `allowedDomains`와 동일한 와일드카드 구문을 지원합니다. 둘 다 일치할 때 `allowedDomains`보다 우선합니다. 모든 설정 소스와 관계없이 병합됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                    | `["sensitive.cloud.example.com"]`                    |
| `network.allowManagedDomainsOnly`      | (Managed 설정만) Managed 설정의 `allowedDomains` 및 `WebFetch(domain:...)` 허용 규칙만 존중됩니다. 사용자, 프로젝트 및 local 설정의 도메인은 무시됩니다. 허용되지 않은 도메인은 사용자에게 메시지를 표시하지 않고 자동으로 차단됩니다. 거부된 도메인은 여전히 모든 소스에서 존중됩니다. 기본값: false                                                                                                                                                                                                                                                                                                                                                            | `true`                                               |
| `network.httpProxyPort`                | 자신의 프록시를 가져오려는 경우 사용되는 HTTP 프록시 포트입니다. 지정되지 않으면 Claude가 자신의 프록시를 실행합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `8080`                                               |
| `network.socksProxyPort`               | 자신의 프록시를 가져오려는 경우 사용되는 SOCKS5 프록시 포트입니다. 지정되지 않으면 Claude가 자신의 프록시를 실행합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `8081`                                               |
| `network.tlsTerminate`                 | }실험적입니다. 샌드박스 프록시 내에서 TLS를 종료하여 HTTPS 요청의 내용을 읽을 수 있습니다. `mask` [자격 증명 대체](/docs/ko/sandboxing#protect-credentials)에 필요합니다. 세션에 대한 임시 인증 기관을 생성하려면 `{}`로 설정하거나 자신의 것을 사용하려면 `caCertPath` 및 `caKeyPath`로 설정합니다. 사용자, managed 또는 CLI `--settings` 설정에서만 적용되며 `.claude/settings.json` 또는 `.claude/settings.local.json`에서는 적용되지 않습니다. Claude Code v2.1.199 이상이 필요합니다. }                                                                                                                                                                                                    | `{}`                                                 |
| `enableWeakerNestedSandbox`            | 권한이 없는 Docker 환경에서 더 약한 샌드박스를 활성화합니다 (Linux 및 WSL2만). **보안을 감소시킵니다.** 기본값: false                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `true`                                               |
| `enableWeakerNetworkIsolation`         | (macOS만) 샌드박스에서 시스템 TLS 신뢰 서비스 (`com.apple.trustd.agent`)에 대한 액세스를 허용합니다. MITM 프록시 및 사용자 정의 CA를 사용하는 `httpProxyPort`를 사용할 때 `gh`, `gcloud` 및 `terraform`과 같은 Go 기반 도구가 TLS 인증서를 확인하는 데 필요합니다. **보안을 감소시킵니다** 잠재적 데이터 유출 경로를 열어서. 기본값: false                                                                                                                                                                                                                                                                                                                       | `true`                                               |
| `allowAppleEvents`                     | (macOS만) 샌드박싱된 명령이 Apple Events를 보낼 수 있도록 허용합니다. `open`, `osascript` 및 URL을 브라우저에서 열 수 있는 도구에 필요하며, 그렇지 않으면 오류 `-600`으로 실패합니다. **코드 실행 격리를 제거합니다.** 샌드박싱된 명령은 사용자 프롬프트 없이 다른 애플리케이션을 샌드박싱되지 않은 상태로 시작할 수 있습니다. 또한 Terminal과 같은 실행 중인 애플리케이션에 AppleScript 명령을 보낼 수 있으며, 이는 앱별 macOS 자동화 동의 프롬프트 (TCC)의 대상입니다. 사용자, managed 또는 CLI 설정에서만 적용되며 프로젝트 설정에서는 적용되지 않습니다. 기본값: false                                                                                                                                                                                      | `true`                                               |
| `bwrapPath`                            | (Managed 설정만, Linux/WSL2) bubblewrap (`bwrap`) 바이너리의 절대 경로입니다. `PATH`를 통한 자동 감지를 재정의합니다. [managed 설정](/docs/ko/settings#settings-files)에서만 적용되며 사용자 또는 프로젝트 설정에서는 적용되지 않습니다. `bwrap`이 managed 환경에서 비표준 위치에 설치된 경우 유용합니다.                                                                                                                                                                                                                                                                                                                                               | `/opt/admin/bwrap`                                   |
| `socatPath`                            | (Managed 설정만, Linux/WSL2) 샌드박스 네트워크 프록시에 사용되는 `socat` 바이너리의 절대 경로입니다. `PATH`를 통한 자동 감지를 재정의합니다. Managed 설정에서만 적용됩니다.                                                                                                                                                                                                                                                                                                                                                                                                                                              | `/opt/admin/socat`                                   |

<h4 id="sandbox-path-prefixes">
  Sandbox 경로 접두사
</h4>

`filesystem.allowWrite`, `filesystem.denyWrite`, `filesystem.denyRead`, `filesystem.allowRead` 및 `credentials.files`의 경로는 다음 접두사를 지원합니다:

| 접두사            | 의미                                                      | 예제                                                                  |
| :------------- | :------------------------------------------------------ | :------------------------------------------------------------------ |
| `/`            | 파일 시스템 루트의 절대 경로                                        | `/tmp/build`는 `/tmp/build`로 유지됨                                     |
| `~/`           | 홈 디렉토리에 상대적                                             | `~/.kube`는 `$HOME/.kube`가 됨                                         |
| `./` 또는 접두사 없음 | 프로젝트 설정의 경우 프로젝트 루트에 상대적이거나 사용자 설정의 경우 `~/.claude`에 상대적 | `./output`은 `.claude/settings.json`에서 `<project-root>/output`으로 해결됨 |

이전 `//path` 접두사는 절대 경로에 대해 여전히 작동합니다. 이전에 프로젝트 상대 해결을 기대하면서 단일 슬래시 `/path`를 사용한 경우 `./path`로 전환합니다. 이 구문은 `/path`를 프로젝트 상대로 사용하는 [Read 및 Edit 권한 규칙](/docs/ko/permissions#read-and-edit)과 다릅니다. Sandbox 파일 시스템 경로는 표준 규칙을 사용합니다: `/tmp/build`는 절대 경로입니다.

**구성 예제:**

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": true,
    "excludedCommands": ["docker *"],
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"],
      "denyRead": ["~/.aws/credentials"]
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "registry.yarnpkg.com"],
      "deniedDomains": ["uploads.github.com"],
      "allowUnixSockets": [
        "/var/run/docker.sock"
      ],
      "allowLocalBinding": true
    }
  }
}
```

**파일 시스템 및 네트워크 제한**은 함께 병합되는 두 가지 방식으로 구성할 수 있습니다:

* **`sandbox.filesystem` 설정** (위에 표시됨): OS 수준 샌드박스 경계에서 경로를 제어합니다. 이러한 제한은 Claude의 파일 도구뿐만 아니라 모든 하위 프로세스 명령 (예: `kubectl`, `terraform`, `npm`)에 적용됩니다.
* **권한 규칙**: `Edit` 허용/거부 규칙을 사용하여 Claude의 파일 도구 액세스를 제어하고, `Read` 거부 규칙을 사용하여 읽기를 차단하고, `WebFetch` 허용/거부 규칙을 사용하여 네트워크 도메인을 제어합니다. 이러한 규칙의 경로도 샌드박스 구성에 병합됩니다.

<h3 id="attribution-settings">
  Attribution 설정
</h3>

Claude Code는 git 커밋 및 pull request에 attribution을 추가합니다. 이들은 별도로 구성됩니다:

* 커밋은 기본적으로 [git trailers](https://git-scm.com/docs/git-interpret-trailers) (예: `Co-Authored-By`)를 사용하며 사용자 정의하거나 비활성화할 수 있습니다
* Pull request 설명은 일반 텍스트입니다

| 키            | 설명                                                                                                                                                 |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| `commit`     | git 커밋에 대한 attribution으로 모든 trailers를 포함합니다. 빈 문자열은 커밋 attribution을 숨깁니다                                                                           |
| `pr`         | Pull request 설명에 대한 attribution입니다. 빈 문자열은 pull request attribution을 숨깁니다                                                                          |
| `sessionUrl` | 웹 또는 Remote Control 세션에서 실행할 때 커밋에 `Claude-Session` trailer로 claude.ai 세션 링크를 추가할지 여부 및 pull request 설명의 링크. 기본값: `true`. 링크를 생략하려면 `false`로 설정합니다 |

**기본 커밋 attribution:**

```text theme={null}
Co-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>
```

세션의 활성 모델을 반영하는 trailer의 모델 이름입니다.

**기본 pull request attribution:**

```text theme={null}
🤖 Generated with [Claude Code](https://claude.com/claude-code)
```

**예제:**

```json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": ""
  }
}
```

<Note>
  `attribution` 설정은 더 이상 사용되지 않는 `includeCoAuthoredBy` 설정보다 우선합니다. 모든 attribution을 숨기려면 `commit` 및 `pr`을 빈 문자열로 설정하고 `sessionUrl`을 `false`로 설정합니다.
</Note>

<h3 id="file-suggestion-settings">
  파일 제안 설정
</h3>

`@` 파일 경로 자동 완성을 위한 사용자 정의 명령을 구성합니다. 기본 제공 파일 제안은 빠른 파일 시스템 순회를 사용하지만 대규모 monorepos는 사전 구축된 파일 인덱스 또는 사용자 정의 도구와 같은 프로젝트 특정 인덱싱의 이점을 얻을 수 있습니다.

```json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

명령은 `CLAUDE_PROJECT_DIR`을 포함한 [hooks](/docs/ko/hooks)와 동일한 환경 변수로 실행됩니다. stdin을 통해 `query` 필드가 있는 JSON을 받습니다:

```json theme={null}
{"query": "src/comp"}
```

stdout에 줄 바꿈으로 구분된 파일 경로를 출력합니다 (현재 15개로 제한됨):

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

**예제:**

```bash theme={null}
#!/bin/bash
query=$(cat | jq -r '.query')
# your-repo-file-index를 자신의 파일 검색 명령으로 바꿉니다
your-repo-file-index --query "$query" | head -20
```

<h3 id="footer-link-badges">
  바닥글 링크 배지
</h3>

`footerLinksRegexes` 설정은 입력 상자 아래 바닥글에 클릭 가능한 배지를 렌더링합니다. 이를 사용하여 검토 도구 및 이슈 추적기와 같은 프로젝트 CLI에서 인쇄한 ID를 세션 링크로 변환합니다.

각 항목의 `pattern` 정규식은 턴 출력과 일치합니다: 도구 결과 (파일 내용 및 가져온 페이지 포함) 및 Claude의 자체 응답. `url` 및 `label`의 `{name}` 자리 표시자는 패턴의 명명된 캡처 그룹에서 채워집니다.

다음 예제는 `PROJ-1234`와 같은 이슈 키가 턴 출력에 나타날 때마다 배지를 렌더링합니다. `(?<key>...)` 명명된 그룹이 키를 캡처하고 `{key}`가 URL 및 레이블로 대체됩니다:

```json ~/.claude/settings.json theme={null}
{
  "footerLinksRegexes": [
    {
      "type": "regex",
      "pattern": "\\b(?<key>PROJ-\\d+)\\b",
      "url": "https://issues.example.com/browse/{key}",
      "label": "{key}"
    }
  ]
}
```

이렇게 구성하면 `PROJ-1234`가 도구 결과 또는 Claude의 응답에 나타날 때 `PROJ-1234` 칩이 바닥글에 나타나 `https://issues.example.com/browse/PROJ-1234`로 연결됩니다.

다음 제약이 각 항목에 적용됩니다:

| 제약         | 동작                                                                                                                                                                           |
| :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| URL origin | 캡처된 값은 URL 인코딩되고 구성된 URL은 템플릿의 리터럴 origin을 공유해야 합니다. 캡처는 경로 세그먼트 또는 쿼리 값을 채울 수 있지만 링크가 가리키는 위치를 변경할 수 없습니다                                                                   |
| URL 길이     | 2048자보다 긴 구성된 URL은 삭제됩니다                                                                                                                                                     |
| URL 스키마    | `https`, `http` 또는 인식된 편집기 또는 작업 공간 deep-link 스키마여야 합니다: `vscode`, `vscode-insiders`, `cursor`, `windsurf`, `zed`, `jetbrains`, `idea`, `slack`, `linear`, `notion`, `figma` |
| 레이블        | 기본값은 일치한 텍스트이며 28개 표시 열로 잘립니다                                                                                                                                                |
| 배지 수       | 최대 5개 배지가 렌더링됩니다. 가장 오래된 것이 새로운 일치로 대체되고 `/clear`가 이를 제거합니다                                                                                                                  |
| 설정 범위      | 사용자 설정, `--settings` 플래그 및 managed 설정에서만 읽습니다. 프로젝트 `.claude/settings.json` 및 local `.claude/settings.local.json`에서는 무시됩니다                                                   |

턴이 완료되면 Claude Code는 메인 스레드에서 각 항목의 `pattern` 정규식을 턴 출력과 일치시키므로 느린 정규식은 완료될 때까지 UI를 차단합니다. `(a+)+$`와 같은 중첩된 수량자는 특정 입력에 대해 지수적으로 오래 걸릴 수 있고 세션을 고정시킬 수 있으므로 각 `pattern`을 선형으로 유지하고 `+` 또는 `*` 중첩을 피합니다.

바닥글 배지는 구성된 [사용자 정의 상태 줄](/docs/ko/statusline)과 함께 렌더링됩니다. 어느 것도 다른 것을 대체하지 않습니다. 세션 데이터에서 자신의 콘텐츠를 계산하는 스크립트 기반 행에 상태 줄을 사용하고 스크립트 없이 대화에서 ID를 링크로 변환하려면 바닥글 배지를 사용합니다.

<h3 id="hook-configuration">
  Hook 구성
</h3>

이러한 설정은 어떤 hooks가 실행될 수 있는지와 HTTP hooks가 액세스할 수 있는 것을 제어합니다. `allowManagedHooksOnly` 설정은 [managed 설정](#settings-files)에서만 구성할 수 있습니다. URL 및 env var 허용 목록은 모든 설정 수준에서 설정할 수 있으며 소스 전체에서 병합됩니다.

**`allowManagedHooksOnly`가 `true`일 때의 동작:**

* Managed hooks 및 SDK hooks가 로드됨
* Managed 설정 `enabledPlugins`에서 강제 활성화된 플러그인의 hooks가 로드됩니다. 이를 통해 관리자는 조직 마켓플레이스를 통해 검증된 hooks를 배포하면서 다른 모든 것을 차단할 수 있습니다. 신뢰는 전체 `plugin@marketplace` ID로 부여되므로 다른 마켓플레이스의 동일한 이름의 플러그인은 차단된 상태로 유지됩니다
* 사용자 hooks, 프로젝트 hooks 및 다른 모든 플러그인 hooks는 차단됩니다

**HTTP hook URL 제한:**

HTTP hooks가 대상으로 할 수 있는 URL을 제한합니다. 일치를 위해 `*`를 와일드카드로 지원합니다. 배열이 정의되면 일치하지 않는 URL을 대상으로 하는 HTTP hooks는 자동으로 차단됩니다. 호스트명 일치는 대소문자를 구분하지 않으며 후행 FQDN 점을 무시하여 DNS 의미론과 일치합니다.

```json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

**HTTP hook 환경 변수 제한:**

HTTP hooks가 헤더 값에 보간할 수 있는 환경 변수 이름을 제한합니다. 각 hook의 유효한 `allowedEnvVars`는 이 설정과의 교집합입니다.

```json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

<h3 id="compute-managed-settings-with-a-policy-helper">
  정책 도우미로 managed 설정 계산
</h3>

`policyHelper` 설정은 시작 시 managed 설정을 동적으로 계산하는 실행 파일을 가리키므로 관리자는 장치 상태, ID 또는 원격 서비스에서 정책을 파생시킬 수 있습니다. MDM 또는 시스템 `managed-settings.json` 파일에서 구성합니다. Claude Code는 사용자 설정, 프로젝트 설정, HKCU 레지스트리 하이브 및 [서버 관리 설정](/docs/ko/server-managed-settings)을 포함한 다른 범위에 나타나는 `policyHelper`를 무시합니다.

설정은 다음 키를 허용합니다:

| 키                   | 유형     | 설명                                                                   |
| ------------------- | ------ | -------------------------------------------------------------------- |
| `path`              | string | 도우미 실행 파일의 절대 경로                                                     |
| `timeoutMs`         | number | 도우미가 실패한 것으로 처리하기 전에 대기할 시간                                          |
| `refreshIntervalMs` | number | 백그라운드에서 도우미를 다시 실행할 빈도. 새로고침을 비활성화하려면 `0`으로 설정하거나 최소 `60000`으로 설정합니다 |

도우미는 stdout에 JSON 봉투를 작성합니다. 설정을 최상위 수준이 아닌 `managedSettings` 키 아래에 배치합니다. 왜냐하면 베어 설정 객체는 `managedSettings` undefined로 파싱되고 아무것도 적용하지 않기 때문입니다:

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  },
  "claudeMd": "# Organization context\n...",
  "appendSystemPrompt": "Always cite the internal style guide."
}
```

도우미가 `managedSettings`를 내보낼 때 해당 객체는 실행을 위해 파일 기반 managed 설정을 대체합니다. 도우미가 시작 시 0이 아닌 값으로 종료되면 Claude Code는 오류를 인쇄하고 시작을 거부하므로 중단 복원력이 필요한 도우미는 자신의 캐시에서 제공하고 `0`으로 종료해야 합니다.

<h3 id="settings-precedence">
  설정 우선순위
</h3>

설정은 우선순위 순서대로 적용됩니다. 가장 높음에서 가장 낮음:

1. **Managed 설정** ([서버 관리](/docs/ko/server-managed-settings), [MDM/OS 수준 정책](#configuration-scopes) 또는 [managed 설정](#settings-files))
   * IT에서 서버 전달, MDM 구성 프로필, 레지스트리 정책 또는 managed 설정 파일을 통해 배포한 정책
   * 명령줄 인수를 포함한 다른 수준으로 재정의할 수 없음
   * Managed 계층 내에서 우선순위는: [`policyHelper`](#compute-managed-settings-with-a-policy-helper) 출력 (구성된 경우 유일한 managed 소스 사용) > 원격 (claude.ai [서버 관리](/docs/ko/server-managed-settings) 또는 [Claude apps gateway](/docs/ko/claude-apps-gateway) 전달) > MDM/OS 수준 정책 > 파일 기반 (`managed-settings.d/*.json` + `managed-settings.json`) > HKCU 레지스트리 (Windows만). 하나의 managed 소스만 사용되며 소스는 병합되지 않습니다. 파일 기반 계층 내에서 드롭인 파일과 기본 파일이 함께 병합됩니다.
   * Agent SDK 또는 IDE 확장과 같은 embedding host는 SDK `managedSettings` 옵션을 통해 정책을 제공할 수 있습니다. 기본적으로 이는 관리자 배포 managed 계층이 있을 때 무시됩니다: 서버 관리 설정, MDM 또는 OS 수준 정책, 또는 managed 설정 파일. 사용자 쓰기 가능 HKCU 레지스트리 폴백은 관리자 배포 소스로 계산되지 않습니다. 관리자는 [`parentSettingsBehavior`](#available-settings)를 `"merge"`로 설정하여 옵트인할 수 있습니다. embedder의 값은 필터링되므로 managed 정책을 강화할 수 있지만 완화할 수 없습니다.

2. **명령줄 인수**
   * 특정 세션에 대한 임시 재정의. `--settings <file-or-json>`을 통해 전달된 JSON은 파일 기반 설정과 동일한 규칙을 사용하여 병합됩니다: 여기에 설정된 키는 local, project 또는 user 설정의 동일한 키를 재정의하고, 키를 생략하면 낮은 계층 값이 유지됩니다

3. **Local 프로젝트 설정** (`.claude/settings.local.json`)
   * 개인 프로젝트 특정 설정

4. **공유 프로젝트 설정** (`.claude/settings.json`)
   * 소스 제어의 팀 공유 프로젝트 설정

5. **사용자 설정** (`~/.claude/settings.json`)
   * 개인 전역 설정

이 계층 구조는 조직 정책이 항상 적용되면서도 팀과 개인이 자신의 경험을 사용자 정의할 수 있도록 보장합니다. CLI, [VS Code 확장](/docs/ko/vs-code) 또는 [JetBrains IDE](/docs/ko/jetbrains)에서 Claude Code를 실행하든 동일한 우선순위가 적용됩니다.

예를 들어 사용자 설정이 `permissions.defaultMode`를 `acceptEdits`로 설정하지만 프로젝트의 공유 설정이 이를 `default`로 설정하면 프로젝트 값이 적용됩니다. 아래 예제는 배열 값 설정 (예: 권한 규칙)이 대신 어떻게 결합되는지를 다룹니다.

<Note>
  **배열 설정은 범위 전체에서 병합됩니다.** 동일한 배열 값 설정 (예: `sandbox.filesystem.allowWrite` 또는 `permissions.allow`)이 여러 범위에 나타나면 배열은 **연결되고 중복 제거되며** 대체되지 않습니다. 이는 낮은 우선순위 범위가 높은 우선순위 범위에서 설정한 항목을 재정의하지 않고 항목을 추가할 수 있음을 의미하며 그 반대도 마찬가지입니다. 예를 들어 managed 설정이 `allowWrite`를 `["/opt/company-tools"]`로 설정하고 사용자가 `["~/.kube"]`를 추가하면 두 경로 모두 최종 구성에 포함됩니다. 두 가지 예외가 있습니다: [`fallbackModel`](#available-settings)은 위치가 의미를 가지는 순서가 지정된 체인입니다: 이를 정의하는 최고 우선순위 파일이 전체 값을 제공합니다. [`availableModels`](#available-settings): }[최고 우선순위 managed 소스](/docs/ko/server-managed-settings#settings-precedence)가 이를 정의할 때 해당 목록이 그대로 적용되고 사용자, 프로젝트 및 local 항목은 이를 확장할 수 없습니다. 비 managed 범위 전체에서 배열은 평소대로 병합됩니다. [병합 동작](/docs/ko/model-config#merge-behavior)을 참조하세요.
</Note>

<h3 id="verify-active-settings">
  활성 설정 확인
</h3>

Claude Code 내에서 `/status`를 실행하여 활성 설정 소스를 확인합니다. 메뉴 내에서 **Status** 탭에는 이 세션에 대해 Claude Code가 로드한 각 계층을 나열하는 `Setting sources` 줄이 포함됩니다 (예: `User settings` 또는 `Project local settings`). [managed 설정](/docs/ko/admin-setup#decide-how-settings-reach-devices)이 적용되면 항목은 전달 채널을 괄호로 표시합니다 (예: `Enterprise managed settings (remote)`, `(plist)`, `(HKLM)`, `(HKCU)` 또는 `(file)`). 계층은 해당 소스가 최소 하나의 키로 로드될 때만 목록에 나타나므로 빈 목록은 설정 소스를 찾을 수 없음을 의미합니다.

`Setting sources` 줄은 어떤 소스가 읽혀지는지 확인합니다. 각 개별 키를 제공한 계층을 표시하지는 않습니다. **Config** 탭은 동일한 대화에서 테마 및 verbose 출력과 같은 고정된 토글 집합의 편집기입니다. `settings.json` 내용의 보기가 아닙니다.

설정 파일에 유효하지 않은 JSON 또는 검증에 실패한 값이 포함되어 있으면 Claude Code는 시작 시 설정 문제 알림을 표시하고 `/status`는 영향을 받는 파일을 나열합니다. 각 오류의 세부 사항을 보려면 `/doctor`를 실행합니다.

<h3 id="key-points-about-the-configuration-system">
  구성 시스템의 핵심 포인트
</h3>

* **메모리 파일 (`CLAUDE.md`)**: Claude가 시작 시 로드하는 지침 및 컨텍스트를 포함합니다
* **설정 파일 (JSON)**: 권한, 환경 변수 및 도구 동작을 구성합니다
* **Skills**: `/skill-name`으로 호출하거나 Claude가 자동으로 로드할 수 있는 사용자 정의 프롬프트
* **MCP servers**: 추가 도구 및 통합으로 Claude Code를 확장합니다
* **우선순위**: 높은 수준 구성 (Managed)이 낮은 수준 (User/Project)을 재정의합니다
* **상속**: 설정은 범위 전체에서 병합됩니다. 스칼라 값은 높은 우선순위 범위에서 재정의되고, 배열은 연결됩니다 (예외: `fallbackModel`은 최고 우선순위 범위가 전체 체인을 제공합니다. v2.1.175부터 `availableModels`도 managed 또는 정책 값이 낮은 우선순위 항목을 완전히 대체합니다)

<h3 id="system-prompt">
  시스템 프롬프트
</h3>

Claude Code의 내부 시스템 프롬프트는 게시되지 않습니다. 사용자 정의 지침을 추가하려면 `CLAUDE.md` 파일 또는 `--append-system-prompt` 플래그를 사용합니다.

<h3 id="exclude-sensitive-files">
  민감한 파일 제외
</h3>

API 키, 비밀 및 환경 파일과 같은 민감한 정보가 포함된 파일에서 Claude Code가 액세스하는 것을 방지하려면 `.claude/settings.json` 파일에서 `permissions.deny` 설정을 사용합니다:

```json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)",
      "Read(./secrets/**)",
      "Read(./config/credentials.json)",
      "Read(./build)"
    ]
  }
}
```

이는 더 이상 사용되지 않는 `ignorePatterns` 구성을 대체합니다. 이러한 패턴과 일치하는 파일은 파일 검색 및 검색 결과에서 제외되며 이러한 파일에 대한 읽기 작업이 거부됩니다.

<h2 id="subagent-configuration">
  Subagent 구성
</h2>

Claude Code는 사용자 및 프로젝트 수준 모두에서 구성할 수 있는 사용자 정의 AI subagents를 지원합니다. 이러한 subagents는 YAML frontmatter가 있는 Markdown 파일로 저장됩니다:

* **사용자 subagents**: `~/.claude/agents/` - 모든 프로젝트에서 사용 가능
* **프로젝트 subagents**: `.claude/agents/` - 프로젝트에 특정이며 팀과 공유할 수 있음

Subagent 파일은 사용자 정의 프롬프트 및 도구 권한이 있는 특화된 AI 어시스턴트를 정의합니다. [subagents 문서](/docs/ko/sub-agents)에서 subagents 생성 및 사용에 대해 자세히 알아보세요.

<h2 id="plugin-configuration">
  플러그인 구성
</h2>

Claude Code는 skills, agents, hooks 및 MCP servers로 기능을 확장할 수 있는 플러그인 시스템을 지원합니다. 플러그인은 마켓플레이스를 통해 배포되며 사용자 및 저장소 수준 모두에서 구성할 수 있습니다.

<h3 id="plugin-settings">
  플러그인 설정
</h3>

`settings.json`의 플러그인 관련 설정:

```json theme={null}
{
  "enabledPlugins": {
    "formatter@acme-tools": true,
    "deployer@acme-tools": true,
    "analyzer@security-plugins": false
  },
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    }
  }
}
```

<h4 id="enabledplugins">
  `enabledPlugins`
</h4>

어떤 플러그인이 활성화되는지 제어합니다. 형식: `"plugin-name@marketplace-name": true/false`. 어떤 범위에서도 항목이 없는 플러그인은 해당 [`defaultEnabled`](/docs/ko/plugins-reference#default-enablement) 값으로 폴백됩니다.

**범위**:

* **사용자 설정** (`~/.claude/settings.json`): 개인 플러그인 설정
* **프로젝트 설정** (`.claude/settings.json`): 팀과 공유되는 프로젝트 특정 플러그인
* **Local 설정** (`.claude/settings.local.json`): 머신별 재정의, Claude Code가 생성할 때 gitignored됨
* **Managed 설정** (`managed-settings.json`): 모든 범위에서 설치를 차단하고 마켓플레이스에서 플러그인을 숨기는 조직 전체 정책 재정의

<Note>
  프로젝트 설정은 사용자 설정보다 우선순위가 높으므로 `~/.claude/settings.json`에서 플러그인을 `false`로 설정해도 프로젝트의 `.claude/settings.json`이 활성화하는 플러그인은 비활성화되지 않습니다. 머신에서 프로젝트 활성화 플러그인을 거부하려면 대신 `.claude/settings.local.json`에서 `false`로 설정합니다.

  Managed 설정으로 강제 활성화된 플러그인은 managed 설정이 local 설정을 재정의하므로 이 방식으로 비활성화할 수 없습니다.

  Claude Code v2.1.195부터 GitHub 저장소 또는 npm 패키지와 같은 외부 소스의 플러그인을 프로젝트의 `.claude/settings.json`에서 활성화해도 다른 사람을 위해 설치되지 않습니다. 플러그인을 로드하는 모든 경로는 각 사용자에게 실행되기 전에 [플러그인을 설치하고 신뢰](/docs/ko/discover-plugins#configure-team-marketplaces)하도록 요청합니다.
</Note>

**예제**:

```json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

<h4 id="pluginconfigs">
  `pluginConfigs`
</h4>

플러그인의 [`userConfig`](/docs/ko/plugins-reference#user-configuration) 프롬프트가 수집하는 민감하지 않은 옵션 값을 저장하며, 플러그인 ID로 키가 지정됩니다. Claude Code는 플러그인의 구성 대화 상자를 작성할 때 이 키를 사용자 설정에 기록하므로 수동으로 편집할 필요가 없습니다. 민감한 옵션은 macOS Keychain에 저장되거나 지원되는 keychain이 없는 플랫폼에서는 `~/.claude/.credentials.json`에 저장됩니다.

이 예제는 `acme-tools` 마켓플레이스에서 설치된 플러그인의 한 가지 옵션을 저장합니다:

```json theme={null}
{
  "pluginConfigs": {
    "deployer@acme-tools": {
      "options": {
        "api_endpoint": "https://api.example.com"
      }
    }
  }
}
```

`pluginConfigs`는 사용자 설정, `--settings` 플래그 및 managed 설정에서만 읽습니다. 프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json`의 항목은 무시됩니다. 이러한 값이 플러그인 hook, MCP 및 LSP 구성에 대체되기 때문이며, 복제된 저장소는 이들을 제공할 수 없어야 합니다. v2.1.207 이전에는 프로젝트 및 local 설정도 읽혔습니다.

<h4 id="extraknownmarketplaces">
  `extraKnownMarketplaces`
</h4>

저장소에서 사용 가능하게 해야 할 추가 마켓플레이스를 정의합니다. 일반적으로 팀 멤버가 필요한 플러그인 소스에 액세스할 수 있도록 저장소 수준 설정에서 사용됩니다.

**저장소에 `extraKnownMarketplaces`가 포함되면**:

1. 팀 멤버는 폴더를 신뢰할 때 마켓플레이스를 설치하라는 메시지를 받습니다
2. 그 다음 팀 멤버는 해당 마켓플레이스에서 플러그인을 설치하라는 메시지를 받습니다
3. 사용자는 원하지 않는 마켓플레이스 또는 플러그인을 건너뛸 수 있습니다 (사용자 설정에 저장됨)
4. 설치는 신뢰 경계를 존중하고 명시적 동의가 필요합니다

**예제**:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    },
    "security-plugins": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/security/plugins.git"
      }
    }
  }
}
```

**마켓플레이스 소스 유형**:

* `github`: GitHub 저장소 (`repo` 사용)
* `git`: 모든 git URL (`url` 사용)
* `directory`: 로컬 파일 시스템 경로 (`path` 사용, 개발 전용)
* `hostPattern`: 마켓플레이스 호스트와 일치하는 정규식 패턴 (`hostPattern` 사용)
* `settings`: 별도의 호스팅 저장소 없이 settings.json에 직접 선언된 인라인 마켓플레이스 (`name` 및 `plugins` 사용)

`git` 소스 유형은 자체 호스팅 GitLab 및 Bitbucket을 포함한 모든 git 호스팅 서비스에서 작동합니다. Claude Code는 해당 머신에서 `git clone`이 사용할 것과 동일한 인증으로 저장소를 복제합니다: 구성된 credential helpers 또는 SSH 키. `GITHUB_TOKEN`과 같은 공급자 토큰은 이를 읽는 credential helper를 통해서만 적용됩니다. 설정 세부 정보는 [Private repositories](/docs/ko/plugin-marketplaces#private-repositories)를 참조하세요.

`github` 및 `git` 소스의 경우 `source` 객체 내부 (`repo` 또는 `url`과 함께)에 `"skipLfs": true`를 설정하여 Claude Code가 마켓플레이스 저장소를 복제하거나 업데이트할 때 Git LFS 다운로드를 건너뜁니다. LFS 포인터 파일은 해당 콘텐츠를 다운로드하는 대신 포인터로 유지됩니다. 저장소에 플러그인 콘텐츠와 무관한 대용량 LFS 객체가 포함되어 있을 때 이를 사용합니다. Claude Code v2.1.153 이상이 필요합니다.

각 마켓플레이스 항목은 선택적 `autoUpdate` Boolean도 허용합니다. `source`와 함께 `"autoUpdate": true`를 설정하여 Claude Code가 해당 마켓플레이스를 새로고침하고 시작 시 설치된 플러그인을 업데이트하도록 합니다. 생략하면 공식 Anthropic 마켓플레이스는 기본값이 `true`이고 다른 모든 마켓플레이스는 기본값이 `false`입니다. [자동 업데이트 구성](/docs/ko/discover-plugins#configure-auto-updates)을 참조하세요.

`source: 'settings'`를 사용하여 호스팅된 마켓플레이스 저장소를 설정하지 않고 작은 플러그인 세트를 인라인으로 선언합니다. 여기에 나열된 플러그인은 GitHub 또는 npm과 같은 외부 소스를 참조해야 합니다. 여전히 `enabledPlugins`에서 각 플러그인을 별도로 활성화해야 합니다.

```json theme={null}
{
  "extraKnownMarketplaces": {
    "team-tools": {
      "source": {
        "source": "settings",
        "name": "team-tools",
        "plugins": [
          {
            "name": "code-formatter",
            "source": {
              "source": "github",
              "repo": "acme-corp/code-formatter"
            }
          }
        ]
      }
    }
  }
}
```

<h4 id="strictknownmarketplaces">
  `strictKnownMarketplaces`
</h4>

**Managed 설정만**: 사용자가 추가할 수 있는 플러그인 마켓플레이스를 제어합니다. 이 설정은 [managed 설정](/docs/ko/settings#settings-files)에서만 구성할 수 있으며 관리자에게 마켓플레이스 소스에 대한 엄격한 제어를 제공합니다.

**Managed 설정 파일 위치**:

* **macOS**: `/Library/Application Support/ClaudeCode/managed-settings.json`
* **Linux 및 WSL**: `/etc/claude-code/managed-settings.json`
* **Windows**: `C:\Program Files\ClaudeCode\managed-settings.json`

**주요 특성**:

* Managed 설정 (`managed-settings.json`)에서만 사용 가능
* 사용자 또는 프로젝트 설정으로 재정의할 수 없음 (최고 우선순위)
* 네트워크/파일 시스템 작업 전에 적용됨 (차단된 소스는 실행되지 않음)
* `hostPattern` 및 `pathPattern`을 제외한 소스 사양에 대해 정확한 일치를 사용합니다. `hostPattern` 및 `pathPattern`은 정규식 일치를 사용합니다

**허용 목록 동작**:

* `undefined` (기본값): 제한 없음 - 사용자는 모든 마켓플레이스를 추가할 수 있습니다
* 빈 배열 `[]`: 완전 잠금 - 사용자는 새 마켓플레이스를 추가할 수 없습니다
* 소스 목록: 사용자는 정확히 일치하는 마켓플레이스만 추가할 수 있습니다

**지원되는 모든 소스 유형**:

허용 목록은 여러 마켓플레이스 소스 유형을 지원합니다. 대부분의 소스는 정확한 일치를 사용하는 반면 `hostPattern` 및 `pathPattern`은 마켓플레이스 호스트 및 파일 시스템 경로에 대한 정규식 일치를 각각 사용합니다.

1. **GitHub 저장소**:

```json theme={null}
{ "source": "github", "repo": "acme-corp/approved-plugins" }
{ "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" }
{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }
```

필드: `repo` (필수), `ref` (선택: 분기 또는 태그), `path` (선택: 하위 디렉토리)

2. **Git 저장소**:

```json theme={null}
{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }
{ "source": "git", "url": "https://bitbucket.org/acme-corp/plugins.git", "ref": "production" }
{ "source": "git", "url": "ssh://git@git.example.com/plugins.git", "ref": "v3.1", "path": "approved" }
```

필드: `url` (필수), `ref` (선택: 분기 또는 태그), `path` (선택: 하위 디렉토리)

3. **URL 기반 마켓플레이스**:

```json theme={null}
{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }
{ "source": "url", "url": "https://cdn.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }
```

필드: `url` (필수), `headers` (선택: 인증된 액세스를 위한 HTTP 헤더)

<Note>
  URL 기반 마켓플레이스는 `marketplace.json` 파일만 다운로드합니다. 서버에서 플러그인 파일을 다운로드하지 않습니다. URL 기반 마켓플레이스의 플러그인은 상대 경로가 아닌 외부 소스 (GitHub, npm 또는 git URL)를 사용해야 합니다. 상대 경로가 있는 플러그인의 경우 대신 Git 기반 마켓플레이스를 사용합니다. [문제 해결](/docs/ko/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces)을 참조하세요.
</Note>

4. **NPM 패키지**:

```json theme={null}
{ "source": "npm", "package": "@acme-corp/claude-plugins" }
{ "source": "npm", "package": "@acme-corp/approved-marketplace" }
```

필드: `package` (필수, 범위가 지정된 패키지 지원)

5. **파일 경로**:

```json theme={null}
{ "source": "file", "path": "/usr/local/share/claude/acme-marketplace.json" }
{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }
```

필드: `path` (필수: marketplace.json 파일의 절대 경로)

6. **디렉토리 경로**:

```json theme={null}
{ "source": "directory", "path": "/usr/local/share/claude/acme-plugins" }
{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }
```

필드: `path` (필수: `.claude-plugin/marketplace.json`을 포함하는 디렉토리의 절대 경로)

7. **호스트 패턴 일치**:

```json theme={null}
{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }
{ "source": "hostPattern", "hostPattern": "^gitlab\\.internal\\.example\\.com$" }
```

필드: `hostPattern` (필수: 마켓플레이스 호스트와 일치하는 정규식 패턴)

각 저장소를 열거하지 않고 특정 호스트의 모든 마켓플레이스를 허용하려면 호스트 패턴 일치를 사용합니다. 이는 개발자가 자신의 마켓플레이스를 만드는 내부 GitHub Enterprise 또는 GitLab 서버가 있는 조직에 유용합니다.

소스 유형별 호스트 추출:

* `github`: 항상 `github.com`에 대해 일치
* `git`: URL에서 호스트 이름 추출 (HTTPS 및 SSH 형식 지원)
* `url`: URL에서 호스트 이름 추출
* `npm`, `file`, `directory`: 호스트 패턴 일치에 지원되지 않음

8. **경로 패턴 일치**:

```json theme={null}
{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }
{ "source": "pathPattern", "pathPattern": ".*" }
```

필드: `pathPattern` (필수: `file` 및 `directory` 소스의 `path` 필드와 일치하는 정규식 패턴)

네트워크 소스에 대한 `hostPattern` 제한과 함께 파일 시스템 기반 마켓플레이스를 허용하려면 경로 패턴 일치를 사용합니다. 모든 로컬 경로를 허용하려면 `".*"`를 설정하거나 특정 디렉토리로 제한하려면 더 좁은 패턴을 설정합니다.

**구성 예제**:

예제: 특정 마켓플레이스만 허용:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "github",
      "repo": "acme-corp/approved-plugins"
    },
    {
      "source": "github",
      "repo": "acme-corp/security-tools",
      "ref": "v2.0"
    },
    {
      "source": "url",
      "url": "https://plugins.example.com/marketplace.json"
    },
    {
      "source": "npm",
      "package": "@acme-corp/compliance-plugins"
    }
  ]
}
```

예제: 모든 마켓플레이스 추가 비활성화:

```json theme={null}
{
  "strictKnownMarketplaces": []
}
```

예제: 내부 git 서버의 모든 마켓플레이스 허용:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

**정확한 일치 요구 사항**:

마켓플레이스 소스는 사용자의 추가가 허용되려면 정확히 일치해야 합니다. git 기반 소스 (`github` 및 `git`)의 경우 이는 모든 선택적 필드를 포함합니다:

* `repo` 또는 `url`이 정확히 일치해야 합니다
* `ref` 필드가 정확히 일치해야 합니다 (또는 둘 다 정의되지 않음)
* `path` 필드가 정확히 일치해야 합니다 (또는 둘 다 정의되지 않음)

일치하지 않는 소스의 예:

```json theme={null}
// 이들은 다른 소스입니다:
{ "source": "github", "repo": "acme-corp/plugins" }
{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }

// 이것도 다릅니다:
{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }
{ "source": "github", "repo": "acme-corp/plugins" }
```

**`extraKnownMarketplaces`와의 비교**:

| 측면         | `strictKnownMarketplaces` | `extraKnownMarketplaces` |
| ---------- | ------------------------- | ------------------------ |
| **목적**     | 조직 정책 적용                  | 팀 편의                     |
| **설정 파일**  | `managed-settings.json`만  | 모든 설정 파일                 |
| **동작**     | 허용 목록에 없는 추가 차단           | 누락된 마켓플레이스 자동 설치         |
| **적용 시기**  | 네트워크/파일 시스템 작업 전          | 사용자 신뢰 프롬프트 후            |
| **재정의 가능** | 아니오 (최고 우선순위)             | 예 (높은 우선순위 설정으로)         |
| **소스 형식**  | 직접 소스 객체                  | 중첩된 소스가 있는 명명된 마켓플레이스    |
| **사용 사례**  | 규정 준수, 보안 제한              | 온보딩, 표준화                 |

**형식 차이**:

`strictKnownMarketplaces`는 직접 소스 객체를 사용합니다:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/plugins" }
  ]
}
```

`extraKnownMarketplaces`는 명명된 마켓플레이스가 필요합니다:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "github", "repo": "acme-corp/plugins" }
    }
  }
}
```

**함께 사용**:

`strictKnownMarketplaces`는 정책 게이트입니다: 사용자가 추가할 수 있는 것을 제어하지만 마켓플레이스를 등록하지 않습니다. 모든 사용자를 위해 마켓플레이스를 제한하고 사전 등록하려면 `managed-settings.json`에서 둘 다 설정합니다:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/plugins" }
  ],
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "github", "repo": "acme-corp/plugins" }
    }
  }
}
```

`strictKnownMarketplaces`만 설정되면 사용자는 여전히 `/plugin marketplace add`를 통해 허용된 마켓플레이스를 수동으로 추가할 수 있지만 자동으로 사용 가능하지 않습니다.

**중요 참고 사항**:

* 제한은 네트워크 요청 또는 파일 시스템 작업 전에 확인됩니다
* 차단되면 사용자는 소스가 managed 정책으로 차단되었음을 나타내는 명확한 오류 메시지를 봅니다
* 제한은 마켓플레이스 추가 및 플러그인 설치, 업데이트, 새로고침 및 자동 업데이트에 적용됩니다. 정책이 설정되기 전에 추가된 마켓플레이스는 해당 소스가 더 이상 허용 목록과 일치하지 않으면 플러그인을 설치하거나 업데이트하는 데 사용할 수 없습니다
* Managed 설정은 최고 우선순위를 가지며 재정의할 수 없습니다

사용자 대면 문서는 [Managed 마켓플레이스 제한](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions)을 참조하세요.

<h4 id="strictpluginonlycustomization">
  `strictPluginOnlyCustomization`
</h4>

**Managed 설정만**: skills, agents, hooks 및 MCP servers가 사용자 및 프로젝트 소스에서 로드되는 것을 차단하므로 플러그인 또는 managed 설정에서만 가져올 수 있습니다. `strictKnownMarketplaces`와 결합하여 전체 사용자 정의 공급 체인을 제어합니다: 마켓플레이스 허용 목록은 사용자가 설치할 수 있는 플러그인을 제어하고 이 설정은 플러그인 또는 managed 설정에서 오지 않는 모든 것을 차단합니다.

값은 모든 4개 표면을 잠그려면 `true`이거나 잠글 표면을 명명하는 배열입니다:

```json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

각 잠긴 표면에 대해 Claude Code는 사용자 수준 및 프로젝트 수준 소스를 건너뛰고 플러그인 제공 및 managed 소스만 로드합니다:

| 표면       | 잠금 시 차단됨                                 | 여전히 로드됨                                                         |
| :------- | :--------------------------------------- | :-------------------------------------------------------------- |
| `skills` | `~/.claude/skills/`, `.claude/skills/`   | 플러그인 skills, 번들된 skills, managed 정책 디렉토리의 skills                |
| `agents` | `~/.claude/agents/`, `.claude/agents/`   | 플러그인 agents, 기본 제공 agents, managed 정책 디렉토리의 agents              |
| `hooks`  | 사용자, 프로젝트 및 local `settings.json`의 hooks | 플러그인 hooks, managed 설정의 hooks                                   |
| `mcp`    | `~/.claude.json` 및 `.mcp.json`의 서버       | 플러그인 MCP servers, [`managed-mcp.json`](/docs/ko/managed-mcp) servers |

Claude Code 버전이 인식하지 못하는 표면 이름은 설정 파일을 실패시키지 않고 무시되므로 모든 클라이언트가 업데이트되기 전에 새 표면 이름을 추가할 수 있습니다.

<h3 id="manage-plugins">
  플러그인 관리
</h3>

`/plugin` 명령을 사용하여 플러그인을 대화형으로 관리합니다:

* 마켓플레이스에서 사용 가능한 플러그인 찾아보기
* 플러그인 설치/제거
* 플러그인 활성화/비활성화
* 플러그인 세부 정보 보기 (제공되는 skills, agents, hooks)
* 마켓플레이스 추가/제거

[플러그인 문서](/docs/ko/plugins)에서 플러그인 시스템에 대해 자세히 알아보세요.

<h2 id="environment-variables">
  환경 변수
</h2>

환경 변수를 사용하면 설정 파일을 편집하지 않고 Claude Code 동작을 제어할 수 있습니다. 모든 변수는 [`settings.json`](#available-settings)의 `env` 키 아래에서 구성하여 모든 세션에 적용하거나 팀에 배포할 수 있습니다.

전체 목록은 [환경 변수 참조](/docs/ko/env-vars)를 참조하세요.

<h2 id="tools-available-to-claude">
  Claude가 사용할 수 있는 도구
</h2>

Claude Code는 파일 읽기, 편집, 검색, 명령 실행 및 subagents 조율을 위한 도구 세트에 액세스할 수 있습니다. 도구 이름은 권한 규칙 및 hook 매처에서 사용하는 정확한 문자열입니다.

전체 목록 및 Bash 도구 동작 세부 사항은 [도구 참조](/docs/ko/tools-reference)를 참조하세요.

<h2 id="see-also">
  참고 항목
</h2>

* [권한](/docs/ko/permissions): 권한 시스템, 규칙 구문, 도구 특정 패턴 및 관리형 정책
* [인증](/docs/ko/authentication): Claude Code에 대한 사용자 액세스 설정
* [구성 디버깅](/docs/ko/debug-your-config): 설정, 훅 또는 MCP 서버가 적용되지 않는 이유를 진단합니다
* [설치 및 로그인 문제 해결](/docs/ko/troubleshoot-install): 설치, 인증 및 플랫폼 문제
