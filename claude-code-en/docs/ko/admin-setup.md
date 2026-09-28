> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 조직을 위한 Claude Code 설정

> Claude Code를 배포하는 관리자를 위한 의사결정 맵으로, API 제공자, 관리 설정, 정책 시행, 사용량 모니터링 및 데이터 처리를 다룹니다.

Claude Code는 로컬 개발자 구성보다 우선하는 관리 설정을 통해 조직 정책을 시행합니다. 이러한 설정은 Claude 관리자 콘솔, 모바일 기기 관리(MDM) 시스템 또는 디스크의 파일에서 전달합니다. 설정은 Claude가 도달할 수 있는 도구, 명령, 서버 및 네트워크 대상을 제어합니다.

이 페이지는 배포 결정을 순서대로 안내합니다. 각 행은 아래 섹션 및 해당 영역의 참조 페이지로 연결됩니다.

<Note>
  SSO, SCIM 프로비저닝 및 시트 할당은 Claude 계정 수준에서 구성됩니다. 해당 단계는 [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) 및 [시트 할당](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan)을 참조하세요.
</Note>

| 결정                                                       | 선택 항목                        | 참조                                                                                                                                                                            |
| :------------------------------------------------------- | :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [API 제공자 선택](#choose-your-api-provider)                  | Claude Code가 인증하는 위치 및 청구 방식 | [Authentication](/docs/ko/authentication), [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ko/google-vertex-ai), [Microsoft Foundry](/docs/ko/microsoft-foundry) |
| [설정이 기기에 도달하는 방식 결정](#decide-how-settings-reach-devices) | 관리 정책이 개발자 머신에 도달하는 방식       | [Server-managed settings](/docs/ko/server-managed-settings), [Delivery mechanisms](/docs/ko/managed-settings#delivery-mechanisms)                                                       |
| [시행할 항목 결정](#decide-what-to-enforce)                     | 허용되는 도구, 명령 및 통합             | [Permissions](/docs/ko/permissions), [Sandboxing](/docs/ko/sandboxing)                                                                                                                  |
| [사용량 가시성 설정](#set-up-usage-visibility)                   | 지출 및 채택을 추적하는 방식             | [Analytics](/docs/ko/analytics), [Monitoring](/docs/ko/monitoring-usage), [Costs](/docs/ko/costs)                                                                                            |
| [데이터 처리 검토](#review-data-handling)                       | 데이터 보존 및 규정 준수 태세            | [Data usage](/docs/ko/data-usage), [Security](/docs/ko/security)                                                                                                                        |

<h2 id="choose-your-api-provider">
  API 제공자 선택
</h2>

Claude Code는 여러 API 제공자 중 하나를 통해 Claude에 연결됩니다. 선택에 따라 청구, 인증, 상속하는 규정 준수 태세 및 개발자가 사용할 수 있는 Claude Code 기능이 결정됩니다.

| 제공자                           | 다음의 경우 선택                                                                 |
| :---------------------------- | :------------------------------------------------------------------------ |
| Claude for Teams / Enterprise | Claude Code와 claude.ai를 실행할 인프라 없이 사용자당 구독 하나로 원하는 경우입니다. 이것이 기본 권장사항입니다. |
| Claude Console                | API 우선이거나 종량제 청구를 원하는 경우                                                  |
| Amazon Bedrock                | 기존 AWS 규정 준수 제어 및 청구를 상속하려는 경우                                            |
| Google Cloud's Agent Platform | 기존 GCP 규정 준수 제어 및 청구를 상속하려는 경우                                            |
| Microsoft Foundry             | 기존 Azure 규정 준수 제어 및 청구를 상속하려는 경우                                          |

일부 Claude Code 기능에는 claude.ai 계정이 필요합니다. [Cloud sessions](/docs/ko/claude-code-on-the-web), [Routines](/docs/ko/routines), [Code Review](/docs/ko/code-review), [Remote Control](/docs/ko/remote-control) 및 [Chrome extension](/docs/ko/chrome)은 Console API 키 또는 클라우드 제공자 자격증명만으로는 사용할 수 없습니다. Amazon Bedrock, Google Cloud's Agent Platform 또는 Microsoft Foundry를 통해 배포하는 경우 개발자가 Claude for Teams 또는 Enterprise 시트도 필요한지 계획하세요. 각 기능 페이지에는 해당 플랜 요구사항이 나열되어 있습니다.

인증, 지역 및 기능 패리티를 다루는 전체 제공자 비교는 [enterprise deployment overview](/docs/ko/third-party-integrations)를 참조하세요. 각 제공자의 인증 설정은 [Authentication](/docs/ko/authentication)에 있습니다.

[Network configuration](/docs/ko/network-config)의 프록시 및 방화벽 요구사항은 제공자와 관계없이 적용됩니다. 여러 제공자 앞에 단일 엔드포인트를 원하거나 중앙 집중식 요청 로깅을 원하는 경우 [LLM gateway](/docs/ko/llm-gateway)를 참조하세요.

<h2 id="decide-how-settings-reach-devices">
  설정이 기기에 도달하는 방식 결정
</h2>

관리 설정은 조직 정책을 정의합니다. Claude Code는 아래 표의 네 가지 소스를 우선순위 순서대로 확인합니다. [Claude Code가 관리 소스를 결합하는 방식](/docs/ko/managed-settings#precedence-within-the-managed-tier)에서는 어떤 소스가 적용되는지, 정책 도우미가 무엇을 변경하는지, 모든 소스를 구성하는 방법을 설명합니다. 표는 의사결정 맵입니다.

| 메커니즘                    | 전달                                                                                                                                                                                                    | 우선순위 | 플랫폼            |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--- | :------------- |
| Server-managed          | claude.ai 관리자 콘솔 또는 게이트웨이 로그인을 위한 자체 호스팅 [Claude apps gateway](/docs/ko/claude-apps-gateway)                                                                                                               | 최고   | 모두             |
| plist / registry policy | macOS: `com.anthropic.claudecode` plist<br />Windows: `HKLM\SOFTWARE\Policies\ClaudeCode`                                                                                                             | 높음   | macOS, Windows |
| File-based managed      | macOS: `/Library/Application Support/ClaudeCode/managed-settings.json`<br />Linux and WSL: `/etc/claude-code/managed-settings.json`<br />Windows: `C:\Program Files\ClaudeCode\managed-settings.json` | 중간   | 모두             |
| Windows user registry   | `HKCU\SOFTWARE\Policies\ClaudeCode`                                                                                                                                                                   | 최저   | Windows만       |

Claude Code는 시작 시 server-managed 설정을 가져오고 세션 중에 매시간 새로 고치며, 배포할 엔드포인트 인프라가 없습니다. claude.ai 관리자 콘솔을 통한 전달에는 Claude for Teams 또는 Enterprise 플랜이 필요합니다. Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry에 배포된 경우 [Claude apps gateway](/docs/ko/claude-apps-gateway)를 실행하여 동일한 원격 전달을 받을 수 있거나, 대신 파일 기반 또는 OS 수준 메커니즘 중 하나를 사용할 수 있습니다.

조직이 제공자를 혼합하는 경우 claude.ai 사용자를 위해 [server-managed settings](/docs/ko/server-managed-settings)를 구성하고 다른 사용자도 관리 정책을 받을 수 있도록 [file-based or plist/registry fallback](/docs/ko/managed-settings#delivery-mechanisms)을 구성하세요.

plist 및 HKLM 레지스트리 위치는 모든 제공자와 함께 작동하며 관리자 권한이 필요하므로 변조에 저항합니다. HKCU의 Windows 사용자 레지스트리는 상승 권한 없이 쓸 수 있으므로 시행 채널이 아닌 편의 기본값으로 취급하세요.

기본적으로 WSL은 `/etc/claude-code`의 Linux 파일 경로만 읽습니다. Windows 레지스트리 및 `C:\Program Files\ClaudeCode` 정책을 같은 머신의 WSL로 확장하려면 관리자 전용 Windows 소스 중 하나에서 [`wslInheritsWindowsSettings: true`](/docs/ko/settings-reference#wslinheritswindowssettings)를 설정하세요.

선택한 메커니즘이 무엇이든 관리 값은 사용자 및 프로젝트 설정보다 우선합니다. 단, 몇 가지 보안에 민감한 [예외](/docs/ko/settings#exceptions-to-managed-settings-precedence)가 있습니다. `permissions.allow` 및 `permissions.deny`와 같은 배열 설정은 모든 소스의 항목을 병합하므로 개발자는 관리 목록을 확장할 수 있지만 제거할 수는 없습니다. `fallbackModel`, `availableModels` 및 [`modelPicker`](/docs/ko/settings-reference#modelpicker)의 경우 관리 값은 하위 계층을 병합하지 않고 대체합니다.

<h3 id="wsl-sessions-in-claude-code-desktop">
  Claude Code Desktop의 WSL 세션
</h3>

Windows에서 [Claude Code Desktop은 WSL 2 배포판 내부에서 Code 세션을 실행할 수 있습니다](/docs/ko/desktop-wsl). 세션의 Claude Code 프로세스는 배포판 내부에서 실행되므로 위의 WSL 검색 경로를 통해 관리 설정을 확인합니다. Windows 전용 소스는 `wslInheritsWindowsSettings: true`가 배포되지 않으면 이에 도달하지 않습니다.

Claude Desktop은 조직에서 관리하는 것으로 감지된 기기(예: `C:\Program Files\ClaudeCode\managed-settings.json`이 존재하는 경우)에서 기본적으로 WSL 세션을 끕니다. 이를 켜려면 Windows 레지스트리 정책을 배포하세요. 이는 Claude Desktop v1.19367.0 이상이 필요합니다:

* `HKLM\SOFTWARE\Policies\Claude` 아래에 `disableWslSessions`라는 값을 만들고 `REG_SZ` 문자열 `false` 또는 `REG_DWORD` `0`으로 설정합니다. 이 값은 관리 설정을 전달하는 `ClaudeCode` 키와 별도로 Claude Desktop 정책 키 아래에 있습니다. HKLM 아래에 값을 배포하세요. 이는 관리자 권한이 필요합니다. HKCU 아래의 값은 WSL 세션을 활성화하지 않습니다.
* `C:\Program Files\ClaudeCode\managed-settings.json`을 배포하는 경우 제자리에 두세요. `disableWslSessions`이 HKLM 아래에서 `false`가 되면 Desktop은 해당 파일이 있어도 WSL 세션을 허용합니다.

Desktop은 WSL 세션이 시작될 때마다 정책을 읽으므로 배포 후 앱을 다시 시작할 필요가 없습니다.

기기가 여전히 WSL 세션을 거부하는 경우 해당 기기의 Claude Desktop에서 **Help > Troubleshooting > Show Logs in Explorer**를 열어 로그 폴더의 복사본을 다운로드에 저장합니다. 해당 복사본의 `main.log`에서 `[wslPolicyGate] denying WSL session`을 검색합니다. 거부 이유는 괄호 안에 따라옵니다(예: `(cli-file-present)`). Claude Desktop이 `.exe` 설치 프로그램으로 설치된 경우 `%APPDATA%\Claude\logs\main.log`에서 라이브 파일을 읽을 수도 있습니다.

WSL 세션이 활성화된 후 관리 설정을 확장합니다:

* HKLM 레지스트리 또는 `C:\Program Files\ClaudeCode` 파일을 통해 `wslInheritsWindowsSettings: true`를 배포하여 WSL 세션이 호스트 세션과 동일한 정책을 상속하도록 합니다.
* WSL 세션 내에서 `/status`를 실행하여 확인하고 `Setting sources` 줄을 읽습니다. 이를 해석하려면 [/status에서 소스 읽기](/docs/ko/managed-settings#read-the-source-in-/status)를 참조하세요.

WSL 2 유틸리티 VM 내부의 프로세스는 Windows 측 엔드포인트 감지 센서에 표시되지 않습니다. 배포판 내 프로세스 및 파일 활동을 관찰하려면 엔드포인트 감지 공급업체의 WSL 지침에서 배포판 내에서 실행할 수 있는 Linux 센서와 필요한 제외 사항을 확인하세요. Claude Code의 [OpenTelemetry 도구 실행 텔레메트리](/docs/ko/monitoring-usage)는 WSL 및 네이티브 세션에 대해 동일하게 내보내집니다.

<h2 id="decide-what-to-enforce">
  시행할 항목 결정
</h2>

관리 설정은 도구, 샌드박스 실행, MCP 서버 및 플러그인 소스 제한, 실행되는 hooks 제어를 잠글 수 있습니다. 각 행은 이를 구동하는 설정 키가 있는 제어 표면입니다.

| 제어                                                                              | 기능                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | 주요 설정                                                                                                                                   |
| :------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| [Permission rules](/docs/ko/permissions)                                             | 특정 도구 및 명령 허용, 요청 또는 거부                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `permissions.allow`, `permissions.deny`                                                                                                 |
| [Permission lockdown](/docs/ko/permissions#managed-only-settings)                    | 관리 설정을 [권한 규칙의 유일한 설정 소스](/docs/ko/settings-reference#allowmanagedpermissionrulesonly)로 만듭니다. `--dangerously-skip-permissions` 비활성화                                                                                                                                                                                                                                                                                                                                                                             | `allowManagedPermissionRulesOnly`, `permissions.disableBypassPermissionsMode`                                                           |
| [Starting permission mode](/docs/ko/permission-modes#which-mode-a-session-starts-in) | 기본 제공 시작 권한 모드 대신 개발자의 터미널 세션이 시작되는 권한 모드를 선택하거나 자동 모드를 제거합니다. VS Code 확장은 Pro, Max 및 Team 플랜에서만 설정한 `defaultMode`를 읽습니다. [Switch permission modes](/docs/ko/permission-modes#switch-permission-modes)는 확장이 읽는 내용을 나열합니다                                                                                                                                                                                                                                                                                        | `permissions.defaultMode`, `permissions.disableAutoMode`                                                                                |
| [Sandboxing](/docs/ko/sandboxing)                                                    | 도메인 허용 목록이 있는 OS 수준 파일 시스템 및 네트워크 격리                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `sandbox.enabled`, `sandbox.network.allowedDomains`                                                                                     |
| [Managed policy CLAUDE.md](/docs/ko/memory#deploy-organization-wide-claude-md)       | 모든 세션에서 로드되는 조직 전체 지침, 제외할 수 없음                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | 관리 정책 경로의 파일                                                                                                                            |
| [MCP server control](/docs/ko/managed-mcp)                                           | 사용자가 추가하거나 연결할 수 있는 MCP 서버 제한, 고정된 집합 배포, 또는 모든 사용자에게 자신의 서버와 함께 원격 서버 제공                                                                                                                                                                                                                                                                                                                                                                                                                                  | `allowedMcpServers`, `deniedMcpServers`, `allowManagedMcpServersOnly`, `managedMcpServers`, 또는 배포된 `managed-mcp.json` 파일                |
| [Plugin marketplace control](/docs/ko/plugins/org#restrict-what-users-can-install)   | 사용자가 추가하고 설치할 수 있는 마켓플레이스 소스 제한, 단일 실행을 위해 플러그인, 에이전트 및 MCP 서버를 사이드로드하는 CLI 플래그 거부, [`command` 플러그인 소스](/docs/ko/plugins/marketplace-reference#command-plugin-source) 차단, 마켓플레이스의 플러그인을 제안할 수 있는 항목 허용 목록                                                                                                                                                                                                                                                                                                       | `strictKnownMarketplaces`, `blockedMarketplaces`, `disableSideloadFlags`, `disableCommandPluginSources`, `pluginSuggestionMarketplaces` |
| [Customization lockdown](/docs/ko/settings-reference#strictpluginonlycustomization)  | skills, agents, hooks 및 MCP 서버를 사용자 및 프로젝트 소스에서 차단하여 플러그인 또는 관리 설정에서만 제공되도록 함. skills를 잠그면 [개발자가 claude.ai에서 활성화하는 skills](/docs/ko/skills#where-synced-skills-load)가 동기화되는 것도 중지됩니다                                                                                                                                                                                                                                                                                                                            | `strictPluginOnlyCustomization`                                                                                                         |
| [Disable claude.ai sync](/docs/ko/settings-reference#syncclaudeaiskills)             | Claude Code가 개발자가 claude.ai에서 활성화하는 [skills](/docs/ko/skills#how-synced-skills-behave) 및 [plugins](/docs/ko/plugins/loading#synced-plugins)를 로드하는 것을 중지합니다. 조직에서 claude.ai의 Skills를 비활성화하면 Claude Code는 둘 다 동기화를 중지하며, v2.1.273 이상에서는 이미 동기화된 항목도 제거합니다. Skills를 비활성화하지 않고 둘 중 하나를 중지하려면 관리 설정에서 해당 키를 `false`로 설정합니다                                                                                                                                                                                                | `syncClaudeAiSkills`, `syncClaudeAiPlugins`                                                                                             |
| [Hook restrictions](/docs/ko/settings-reference#allowmanagedhooksonly)               | 실행되는 hooks 제한 및 HTTP hook URL 제한; [allowManagedHooksOnly에서 실행되는 항목](/docs/ko/settings-reference#what-runs-under-allowmanagedhooksonly)에서 전체 효과 목록을 참조하세요                                                                                                                                                                                                                                                                                                                                                        | `allowManagedHooksOnly`, `allowedHttpHookUrls`                                                                                          |
| [Login enforcement](/docs/ko/settings-reference#forceloginmethod)                    | 특정 방법 또는 Anthropic 조직으로 로그인 제한합니다. 방법 제한은 VS Code 확장, Agent SDK, `claude setup-token` 및 `/install-github-app`에 적용되며, 터미널의 대화형 로그인 화면은 `/login` 또는 처음 실행 온보딩으로 도달하며 방법을 미리 선택하지만 시행하지는 않습니다. Claude Code는 터미널, VS Code 확장 및 Agent SDK의 claude.ai 계정 로그인에 대해 조직을 확인하며, Claude Console 로그인 또는 [gateway](/docs/ko/claude-apps-gateway) 로그인에 대해서는 확인하지 않습니다. v2.1.212 이전에는 터미널 로그인만 두 키를 모두 적용했습니다. 설정되면 `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` 또는 `apiKeyHelper`로 인증된 세션은 시작 시 차단됩니다. 클라우드 공급자 세션은 영향을 받지 않습니다 | `forceLoginMethod`, `forceLoginOrgUUID`                                                                                                 |
| [Disable agent view](/docs/ko/agent-view#how-background-sessions-are-hosted)         | `claude agents`, `--bg`, `/background` 및 온디맨드 감독자 비활성화                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `disableAgentView`                                                                                                                      |
| [Configure the corporate launcher](/docs/ko/corporate-launcher)                      | [background-agent supervisor](/docs/ko/agent-view#how-background-sessions-are-hosted), 해당 워커 및 [다른 적용 대상 백그라운드 프로세스](/docs/ko/corporate-launcher#what-the-launcher-covers)에 에이전트 뷰를 끄는 대신 필수 corporate launcher를 접두사로 붙입니다                                                                                                                                                                                                                                                                                           | `processWrapper`                                                                                                                        |
| [Model restrictions](/docs/ko/model-config#restrict-model-selection)                 | `availableModels`는 선택기에 나타나는 모델을 필터링합니다. `enforceAvailableModels`를 추가하면 자동 선택된 기본 모델도 제한합니다. 이 설정이 CLI, 웹 및 IDE에 어떻게 도달하는지는 [surface coverage](/docs/ko/model-config#surface-coverage)를 참조하세요                                                                                                                                                                                                                                                                                                                   | `availableModels`, `enforceAvailableModels`                                                                                             |
| [Effort cap](/docs/ko/settings-reference#maxeffortlevel)                             | 모든 모델 또는 모델별로 모든 공급자에서 [effort level](/docs/ko/model-config#adjust-effort-level)을 제한합니다                                                                                                                                                                                                                                                                                                                                                                                                                         | `maxEffortLevel`                                                                                                                        |
| [Version floor](/docs/ko/settings-reference#minimumversion)                          | 자동 업데이트가 조직 전체 최소값 아래로 설치되는 것을 방지                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `minimumVersion`                                                                                                                        |
| [Required version range](/docs/ko/settings-reference#requiredminimumversion)         | 실행 중인 버전이 조직 승인 범위를 벗어날 때 시작을 거부합니다. 다운그레이드만 차단하는 `minimumVersion`보다 더 강력합니다                                                                                                                                                                                                                                                                                                                                                                                                                               | `requiredMinimumVersion`, `requiredMaximumVersion`                                                                                      |
| [Telemetry opt-out](/docs/ko/data-usage#telemetry-services)                          | 모든 디바이스에서 Anthropic 바운드 사용 메트릭, 오류 보고서 및 설문조사 비활성화                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `env`에 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`을 `1`로 설정; 연결된 섹션에서 카테고리별 변수를 나열합니다                                                    |

claude.ai 또는 Anthropic API를 통해 인증하는 구성원이 있고 Claude Enterprise 플랜을 사용 중인 경우 설정을 배포하지 않고도 조직의 관리 설정에서 모델을 관리할 수 있습니다.

* [Organization model restrictions](/docs/ko/model-config#organization-model-restrictions): 개별 모델을 비활성화합니다. 서버 측에서 시행됩니다.
* [Organization default model](/docs/ko/model-config#organization-default-model): 새 세션이 시작되는 모델을 설정합니다. 조직이 기본값을 시행하지 않는 한 사용자가 변경할 수 있으며, 이는 제한된 조직 집합에서 사용 가능합니다. 가용성에 대해 Anthropic 계정 팀에 문의하세요.
* [Organization effort limits](/docs/ko/model-config#organization-effort-limits): 역할별 노력 수준을 제한합니다. 서버 측에서 시행됩니다.

이러한 제어 중 어느 것도 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 [Claude Platform on AWS](/docs/ko/claude-platform-on-aws)의 세션에 도달하지 않습니다. 이러한 공급자에서는 관리 설정을 대신 사용합니다. 제한을 위해 `availableModels`, 기본값을 위해 `model`, 그리고 노력 제한을 위해 [`maxEffortLevel`](/docs/ko/settings-reference#maxeffortlevel)을 사용합니다.

[Cloud sessions](/docs/ko/claude-code-on-the-web)은 자체 관리 표면을 가지고 있습니다. 관리 설정의 Cloud 환경 페이지에서 소유자는 구성원의 클라우드 세션에 대한 [network access level](/docs/ko/cloud-environments#network-access), 환경 변수 및 설정 스크립트를 설정하는 [organization-shared environments](/docs/ko/cloud-environments#organization-shared-environments)를 생성합니다. 소유자는 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)에서 조직의 기본 환경을 별도로 선택합니다.

권한 규칙 및 샌드박싱은 다양한 계층을 다룹니다. WebFetch를 거부하면 Claude의 fetch 도구가 차단되지만 Bash가 허용되면 `curl` 및 `wget`은 여전히 모든 URL에 도달할 수 있습니다. 샌드박싱은 OS 수준에서 시행되는 네트워크 도메인 허용 목록으로 그 격차를 닫습니다.

이러한 제어가 방어하는 위협 모델은 [Security](/docs/ko/security)를 참조하세요.

<h2 id="set-up-usage-visibility">
  사용량 가시성 설정
</h2>

보고해야 할 내용에 따라 모니터링을 선택하세요. 대시보드, API 및 지출 제어는 Claude for Teams 또는 Enterprise 플랜과 Claude Console 조직 간에 다르므로, 기능을 중심으로 보고를 계획하기 전에 가용성 열을 확인하세요.

| 기능                     | 제공 항목                                                                 | 가용성                                                                                                                                                                                                                          | 시작 위치                                                 |
| :--------------------- | :-------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- |
| Usage monitoring       | 세션, 도구 및 토큰의 OpenTelemetry 내보내기                                       | 모든 제공자                                                                                                                                                                                                                       | [Monitoring usage](/docs/ko/monitoring-usage)              |
| Analytics dashboard    | Teams / Enterprise의 채택 및 기여도 메트릭(리더보드 포함); Console의 사용자별 사용량 및 지출 메트릭 | Teams / Enterprise at [claude.ai/analytics](https://claude.ai/analytics/claude-code), Console at [platform.claude.com/claude-code](https://platform.claude.com/claude-code)                                                  | [Analytics](/docs/ko/analytics)                            |
| Programmatic reporting | API를 통한 사용자별 사용량 및 비용 데이터                                             | Enterprise의 경우 [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics), Console의 경우 [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) | [Costs](/docs/ko/costs#manage-costs-for-your-organization) |
| Spend controls         | 지출 제한 및 속도 제한                                                         | Teams / Enterprise의 관리자 설정, Console의 워크스페이스 제한; 타사 클라우드의 경우, 클라우드 예산 제어 또는 사용자별 [지출 제한](/docs/ko/claude-apps-gateway-spend-limits)이 있는 [Claude apps gateway](/docs/ko/claude-apps-gateway)                                             | [Costs](/docs/ko/costs#manage-costs-for-your-organization) |

Teams 및 Enterprise에서 사용자별 사용량 및 지출 수치는 분석 대시보드가 아닌 조직의 분석 설정에 있는 [지출 보고서](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)에서 제공됩니다. 클라우드 제공자는 AWS Cost Explorer, GCP Billing 또는 Azure Cost Management를 통해 지출을 노출합니다. Claude 채팅, Claude Code 및 Cowork 전반에 걸쳐 엔터프라이즈 예산을 계획하려면 [Claude Enterprise 소비 가이드](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide)를 참조하세요.

<h2 id="review-data-handling">
  데이터 처리 검토
</h2>

Team, Enterprise, Claude API 및 클라우드 제공자 플랜에서 Anthropic은 코드 또는 프롬프트에 대해 모델을 학습하지 않습니다. API 제공자가 보존 및 규정 준수 태세를 결정합니다.

| 주제                        | 알아야 할 사항                                       | 시작 위치                                          |
| :------------------------ | :--------------------------------------------- | :--------------------------------------------- |
| Data usage policy         | Anthropic이 수집하는 항목, 보존 기간, 학습에 사용되지 않는 항목      | [Data usage](/docs/ko/data-usage)                   |
| Zero Data Retention (ZDR) | 요청 완료 후 저장되지 않음. Claude for Enterprise에서 사용 가능 | [Zero data retention](/docs/ko/zero-data-retention) |
| Security architecture     | 네트워크 모델, 암호화, 인증, 감사 추적                        | [Security](/docs/ko/security)                       |

요청 수준 감사 로깅이 필요하거나 데이터 민감도별로 트래픽을 라우팅하려면 개발자와 제공자 사이에 게이트웨이를 배치하세요. 자체 호스팅된 [Claude apps gateway](/docs/ko/claude-apps-gateway)는 IdP 신원을 포함한 요청별 감사 로그를 기록하거나 다른 [LLM gateway](/docs/ko/llm-gateway)를 사용하세요. 규제 요구사항 및 인증은 [Legal and compliance](/docs/ko/legal-and-compliance)를 참조하세요.

<h2 id="verify-and-onboard">
  확인 및 온보딩
</h2>

관리 설정을 구성한 후 개발자가 Claude Code 내에서 `/status`를 실행하도록 하세요. **Status** 탭에서 `Setting sources` 줄은 `Enterprise managed settings` 다음에 괄호 안의 소스를 표시합니다. [활성 설정 확인](/docs/ko/managed-settings#verify-enforcement)에서 레이블을 확인하세요.

개발자가 시작하는 데 도움이 되도록 다음 리소스를 공유하세요:

* [빠른 시작](/docs/ko/quickstart): 설치부터 프로젝트 작업까지의 첫 세션 안내
* [일반적인 워크플로우](/docs/ko/common-workflows): 코드 검토, 리팩토링 및 디버깅과 같은 일상적인 작업의 패턴
* [Claude Code 101](https://academy.claude.com/courses/claude-code-101) 및 [Claude Code in Action](https://academy.claude.com/courses/claude-code-in-action): [Claude Academy](https://academy.claude.com/)의 무료 자기 주도식 과정

로그인 문제의 경우 개발자에게 [인증 문제 해결](/docs/ko/troubleshoot-install#login-and-authentication)을 참조하도록 안내하세요. 가장 일반적인 해결 방법은 다음과 같습니다:

* `/logout`을 실행한 후 `/login`을 실행하여 계정 전환
* 엔터프라이즈 인증 옵션이 누락된 경우 `claude update` 실행
* 업데이트 후 터미널 다시 시작

개발자가 "You haven't been added to your organization yet"을 보면 해당 시트에 Claude Code 액세스가 포함되지 않으며 관리자 콘솔에서 업데이트해야 합니다.

<h2 id="next-steps">
  다음 단계
</h2>

제공자 및 전달 메커니즘을 선택한 후 자세한 구성으로 이동하세요:

* [Server-managed settings](/docs/ko/server-managed-settings): Claude 관리자 콘솔에서 관리 정책 전달
* [All settings](/docs/ko/settings-reference): 모든 설정 키, 파일 위치 및 예제
* [Which value Claude Code uses](/docs/ko/settings#which-value-claude-code-uses): 관리, 프로젝트, 로컬 및 사용자 설정 전반의 우선순위 규칙
* [Monorepos and large repos](/docs/ko/large-codebases): 모노레포에 배포하는 조직을 위한 디렉터리별 구성 패턴
* [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud's Agent Platform](/docs/ko/google-vertex-ai), [Microsoft Foundry](/docs/ko/microsoft-foundry): 제공자별 배포
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, 시트 관리 및 롤아웃 플레이북
