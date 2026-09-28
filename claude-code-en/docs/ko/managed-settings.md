> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 관리형 설정 배포

> 모든 개발자의 머신에 관리형 설정을 배포합니다: OS별 전달 메커니즘, Claude Code가 관리형 소스를 결합하는 방식, 그리고 적용 여부를 확인하는 방법입니다.

관리형 설정은 조직이 모든 개발자의 머신에 배포하는 설정입니다. Claude Code는 이를 다른 모든 수준 위에 적용하므로, 사용자, 프로젝트, 로컬 또는 `--settings` 값이 이를 재정의할 수 없습니다. 단, 낮은 수준의 더 엄격한 값이 여전히 적용되는 몇 가지 [보안에 민감한 예외](/docs/ko/settings#exceptions-to-managed-settings-precedence)는 제외됩니다.

이 페이지는 관리형 설정을 배포하거나 설정이 적용되지 않는 이유를 디버깅하는 관리자를 위한 것입니다. 어떤 것을 적용할지 결정하려면 [적용할 항목 결정](/docs/ko/admin-setup#decide-what-to-enforce) 표부터 시작하세요. claude.ai 콘솔 경로는 [서버 관리형 설정](/docs/ko/server-managed-settings)을 참조하세요. 개발자의 자체 값이 들어가는 파일은 [설정](/docs/ko/settings)을 참조하세요.

<h2 id="deploy-a-managed-settings-file">
  관리형 설정 파일 배포
</h2>

이것이 각 머신에 정책을 적용하는 가장 빠른 방법입니다: `managed-settings.json` 파일입니다. 아직 관리형 설정을 어떻게 전달할지 결정하지 않았거나 기기가 MDM 관리 중이거나 개발자가 클라우드 세션을 실행 중이라면, 먼저 [전달 메커니즘 선택](#choose-a-delivery-mechanism)을 읽으세요.

<Steps>
  <Step title="managed-settings.json 작성">
    적용하기로 결정한 키를 포함하는 `managed-settings.json`을 작성합니다. 이는 `settings.json`과 동일한 JSON 형태입니다. [적용할 항목 결정](/docs/ko/admin-setup#decide-what-to-enforce) 표에는 각 제어 뒤의 키가 나열되어 있으며, [설정 참조](/docs/ko/settings-reference)의 각 항목은 관리형 소스가 이를 설정할 수 있는지 여부를 나타냅니다. 이 파일은 두 개의 파일 읽기를 차단하고, 바이패스 모드를 끄고, Claude Code가 사용자, 프로젝트, 로컬 파일 및 `--allowedTools`의 권한 규칙을 무시하도록 합니다:

    ```json managed-settings.json theme={null}
    {
      "permissions": {
        "deny": [
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    로그인 방법, 모델, MCP 서버, 마켓플레이스를 포함한 더 많은 관리형 키의 형태를 보여주는 더 완전한 예제는 [조직의 관리형 설정](/docs/ko/settings-example#an-organizations-managed-settings)을 참조하세요.
  </Step>

  <Step title="각 머신에 파일 배치">
    파일을 `managed-settings.json`으로 저장합니다. 운영 체제의 시스템 디렉토리에 저장하고, 이미 플릿에 파일을 배치하는 도구를 사용합니다:

    * **macOS**: `/Library/Application Support/ClaudeCode/managed-settings.json`
    * **Linux 및 WSL**: `/etc/claude-code/managed-settings.json`
    * **Windows**: `C:\Program Files\ClaudeCode\managed-settings.json`
  </Step>

  <Step title="정책이 적용되었는지 확인">
    한 머신에서 Claude Code 내에서 `/status`를 실행합니다. `Setting sources` 줄에 `Enterprise managed settings (file)`이 표시됩니다. 그 후 나머지 플릿에 배포합니다. [정책이 적용 중인지 확인](#check-that-a-policy-is-in-force)에서 줄이 누락된 경우 확인할 사항을 다룹니다.
  </Step>
</Steps>

<span id="managed-settings-delivery" />

<span id="delivery-mechanisms" />

<h2 id="choose-a-delivery-mechanism">
  전달 메커니즘 선택
</h2>

위의 단계에서 작성한 파일은 관리되는 설정을 머신에 적용하는 네 가지 방법 중 하나입니다. 모든 메커니즘은 `settings.json` 파일과 동일한 정책 키를 포함하므로 [설정 참조](/docs/ko/settings-reference)가 모든 메커니즘에 적용됩니다. 일부 키는 특정 소스와 연결되어 있으며, 각 항목의 Scope 줄에 어느 것인지 표시됩니다:

* **전달 제어**: [`policyHelper`](/docs/ko/settings-reference#policyhelper), [`wslInheritsWindowsSettings`](/docs/ko/settings-reference#wslinheritswindowssettings), 및 [`managedSourcesBehavior`](/docs/ko/settings-reference#managedsourcesbehavior)
* **게이트웨이 로그인 키**: [`forceLoginGatewayUrl`](/docs/ko/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/ko/settings-reference#gatewayinternalnetworks), 및 [`forceLoginMethod`](/docs/ko/settings-reference#forceloginmethod)의 `"gateway"` 값

관리되는 설정 파일, MDM 프로필 또는 claude.ai 콘솔은 도달하는 모든 사용자에게 하나의 정책을 적용합니다. 개발자 그룹에 다른 정책을 제공하려면 해당 그룹에 다른 파일 또는 프로필을 배포하면 됩니다. claude.ai 콘솔은 [아직 그룹을 대상으로 할 수 없지만](/docs/ko/server-managed-settings#current-limitations), 자체 호스팅 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)는 IdP 그룹별로 관리되는 설정을 전달합니다.

둘 이상의 메커니즘이 동일한 머신에 정책을 전달할 때, Claude Code는 기본적으로 하나를 사용하고 나머지는 무시합니다. [Claude Code가 관리되는 소스를 결합하는 방법](#how-claude-code-combines-managed-sources)에서 순서와 모든 소스에 적용되는 옵트인을 설명합니다.

MDM 및 파일 행은 함께 엔드포인트 관리 설정이라고 불리는데, 이는 정책이 개발자의 디바이스에 저장되기 때문입니다. 반면 서버 관리 행에서는 Claude Code가 정책을 가져옵니다.

아래 표를 사용하여 이미 디바이스를 관리하는 방식에 따라 메커니즘을 선택하세요.

| 메커니즘                                    | 전달 방식                                                                                                                                                    | Claude Code가 읽는 시점                                                                                                                      | 사용 시기                                              |
| :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------- |
| [서버 관리 설정](/docs/ko/server-managed-settings) | claude.ai 관리 콘솔 또는 자체 호스팅 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)에서                                                                                    | 시작 시 가져오고 매시간 폴링됨. [정책이 적용되는 위치 및 시기](#where-and-when-a-policy-applies) 참조                                                              | 각 머신을 건드리지 않고 claude.ai 조직의 정책을 변경할 수 있는 한 곳을 원할 때 |
| MDM 또는 OS 수준 정책                         | Jamf, Intune, Group Policy 또는 유사한 도구를 통해 macOS 구성 프로필 또는 Windows `HKLM` 레지스트리 값으로 제공됨. [각 메커니즘이 정책을 저장하는 위치](#where-each-mechanism-stores-the-policy) 참조 | 시작 시 읽고 30분마다 변경 사항 확인                                                                                                                  | 이미 MDM 또는 Group Policy로 디바이스를 관리할 때                |
| 파일 기반                                   | 각 머신의 시스템 디렉토리에 `managed-settings.json`으로 제공됨. [각 메커니즘이 정책을 저장하는 위치](#where-each-mechanism-stores-the-policy) 참조                                         | 시작 시 읽고 파일이 변경될 때 다시 로드됨                                                                                                                | MDM이 없는 머신, Linux 호스트 또는 직접 빌드한 이미지                |
| HKCU 레지스트리, Windows 및 WSL               | Windows `HKCU` 레지스트리 값으로 제공됨. [각 메커니즘이 정책을 저장하는 위치](#where-each-mechanism-stores-the-policy) 참조                                                          | 시작 시 읽고 30분마다 변경 사항 확인. Claude Code는 다른 관리되는 소스가 정책 키를 전달하지 않고 [호스트 제공 부모 설정](#let-an-embedding-host-add-policy)이 제한적인 키를 제공하지 않을 때만 사용 | 머신 수준 `HKLM` 키를 쓸 수 없을 때                           |

Jamf, Iru, Intune 및 Group Policy용 시작 템플릿은 [MDM 예제 저장소](https://github.com/anthropics/claude-code/tree/main/examples/mdm)에 있습니다.

`managed-mcp.json`을 통해 이들 중 어느 것과 함께 배포하거나 [`managedMcpServers`](/docs/ko/settings-reference#managedmcpservers) 키를 통해 제공하는 관리되는 MCP 서버의 경우 [관리되는 MCP 구성](/docs/ko/managed-mcp)을 참조하세요.

<h3 id="where-and-when-a-policy-applies">
  정책이 적용되는 위치 및 시기
</h3>

배포된 정책은 다음과 같이 개발자의 세션에 도달합니다:

* **표면**: 개발자의 머신, 터미널, VS Code 및 JetBrains 확장, 데스크톱 앱의 Code 탭 및 [Agent SDK](/docs/ko/agent-sdk/typescript) 세션은 이 모든 소스를 읽습니다. Agent SDK 세션은 `settingSources`가 사용자, 프로젝트 및 로컬 파일을 제외할 때도 관리되는 설정을 로드합니다.
* **클라우드 세션**: Anthropic 호스팅 환경의 세션은 디바이스의 MDM 프로필 또는 파일을 읽지 않으므로 정책은 서버 관리 설정에서 가져와야 합니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments)의 세션도 기본적으로 서버 관리 설정이 정책 키를 전달하지 않을 때만 러너 이미지의 관리되는 설정 파일을 읽습니다. [모든 관리자 소스에서 Claude Code가 읽는 키](#keys-read-from-every-admin-source)는 제외됩니다. [Claude Code가 관리되는 소스를 결합하는 방법](#how-claude-code-combines-managed-sources)에서 두 가지 모두에 적용되는 옵트인을 다룹니다.
* **Cowork 세션**: Claude Desktop 앱의 [Cowork](https://claude.com/docs/cowork/overview)는 Claude Code에서 세션을 실행합니다. Cowork 세션에서 Claude Code는 사용자가 Team 또는 Enterprise 계정으로 로그인할 때도 claude.ai 관리 콘솔에서 서버 관리 설정을 가져오지 않으므로 적용되는 정책은 세션이 실행되는 위치에 따라 달라집니다:

  * **사용자의 머신에서**: 기본적으로 Cowork 세션의 Claude Code는 해당 디바이스의 MDM 또는 OS 수준 정책과 관리되는 설정 파일을 읽으므로 정책을 거기에 배포하세요.
  * **전체 VM 샌드박스에서**: Claude Desktop 관리 구성이 [`requireCoworkFullVmSandbox`](https://claude.com/docs/third-party/claude-desktop/configuration#requirecoworkfullvmsandbox)를 설정할 때, Claude Code는 디바이스의 MDM 정책과 관리되는 설정 파일이 없는 가상 머신 내에서 실행됩니다.
  * **원격 Cowork 세션**: 이들은 Anthropic 관리 VM에서 실행되며, Claude Code는 읽을 디바이스 정책이 없습니다.

  세션이 실행되는 위치와 관계없이, claude.ai는 누군가 claude.ai에서 git 저장소의 마켓플레이스를 추가하거나 Cowork 탭의 **사용자 정의**에서 추가할 때 관리 콘솔의 [`strictKnownMarketplaces`](/docs/ko/settings-reference#strictknownmarketplaces) 및 [`blockedMarketplaces`](/docs/ko/settings-reference#blockedmarketplaces) 목록을 자체적으로 적용합니다. [제한이 작동하는 방식](/docs/ko/plugins/org#restrict-what-users-can-install)에서 해당 확인을 설명합니다. [표면 범위](/docs/ko/model-config#surface-coverage) 표는 Cowork를 다른 표면과 비교합니다.
* **실행 중인 세션**: 대부분의 변경 사항은 재시작 없이 [전달 메커니즘 표](#choose-a-delivery-mechanism)의 일정에 따라 실행 중인 세션에 도달합니다.
  * [`forceRemoteSettingsRefresh`](/docs/ko/settings-reference#forceremotesettingsrefresh), [`requiredMinimumVersion`](/docs/ko/settings-reference#requiredminimumversion) 및 [일부 사용자 편집 가능 키](/docs/ko/settings#when-edits-take-effect)에 대한 변경 사항은 다음 세션 시작 시 적용됩니다.
  * 새로운 또는 변경된 [`policyHelper`](/docs/ko/settings-reference#policyhelper) 항목은 다음 시작 시 적용됩니다. 서버 관리 설정이 해당 시작 시 도우미를 숨기면, 도우미는 가져오기가 해당 설정이 제거되었음을 보고하는 즉시 실행됩니다.
* **승인이 필요한 변경 사항**: [다음 시작을 기다리는 업데이트](/docs/ko/server-managed-settings#fetch-and-caching-behavior)를 제외하고, 후크 또는 `env` 변수와 같이 [승인이 필요한](/docs/ko/server-managed-settings#security-approval-dialogs) 설정에 대한 서버 관리 변경 사항은 개발자가 대화형 세션에서 대화 상자를 수락할 때까지 기다리며, IDE 확장 또는 Agent SDK가 호스팅하는 세션의 현재 실행에 적용됩니다. 다른 서버 관리 변경 사항은 다음 폴링에 적용됩니다.
* **장기 실행 세션**: 몇 주 동안 열려 있는 세션도 여전히 롤아웃에 뒤질 수 있습니다. [`requiredMinimumVersion`](/docs/ko/settings-reference#requiredminimumversion)은 오래된 바이너리가 시작되는 것을 차단하며 이미 실행 중인 세션을 종료하지 않습니다.

<span id="format-the-policy-for-each-platform" />

<h3 id="where-each-mechanism-stores-the-policy">
  각 메커니즘이 정책을 저장하는 위치
</h3>

키는 모든 곳에서 동일하지만 각 메커니즘은 다른 위치와 형식으로 저장합니다:

* **서버 관리**: Anthropic의 서버 또는 게이트웨이가 정책을 보유합니다. Claude Code는 시작 시 적용하고 [각 성공적인 가져오기 시 교체](/docs/ko/server-managed-settings#security-considerations)하는 로컬 캐시를 유지합니다.
* **macOS 구성 프로필**: `com.anthropic.claudecode` 관리 기본 설정 도메인. `managed-settings.json`과 동일한 최상위 키를 사용하며, 중첩된 설정은 딕셔너리이고 목록은 plist 배열입니다.
* **Windows HKLM 레지스트리**: `HKLM\SOFTWARE\Policies\ClaudeCode` 아래 `Settings`라는 이름의 `REG_SZ` 또는 `REG_EXPAND_SZ` 값으로 JSON입니다.
* **파일 기반**: `managed-settings.json`, 선택적 `managed-settings.d/` 디렉토리 및 `managed-mcp.json`은 시스템 디렉토리에 있습니다: macOS의 `/Library/Application Support/ClaudeCode/`, Linux 및 WSL의 `/etc/claude-code/`, Windows의 `C:\Program Files\ClaudeCode\`. Claude Code는 레거시 Windows 경로 `C:\ProgramData\ClaudeCode\managed-settings.json`을 읽지 않습니다.
* **Windows HKCU 레지스트리**: `HKCU\SOFTWARE\Policies\ClaudeCode` 아래 동일한 `Settings` 값입니다.

<h3 id="split-a-file-based-policy-across-teams">
  파일 기반 정책을 팀 간에 분할
</h3>

여러 팀이 하나의 정책의 일부를 소유하는 경우, 하나의 공유 파일을 편집하는 대신 각 부분을 `managed-settings.json`과 동일한 시스템 디렉토리의 `managed-settings.d/` 내 자체 파일에 넣으세요.

Claude Code는 `managed-settings.json`을 먼저 병합한 다음 디렉토리의 모든 `*.json` 파일을 알파벳 순서로 병합합니다. `10-telemetry.json` 및 `20-security.json`과 같은 숫자 접두사로 파일 이름을 지정하여 순서를 제어하세요. Claude Code는 숨겨진 파일과 `.json`으로 끝나지 않는 파일을 무시합니다.

두 파일이 동일한 키를 설정할 때, Claude Code는 다음 규칙에 따라 결합합니다:

* **단일 값**, 예: `"model": "opus"` 또는 `"cleanupPeriodDays": 7`: 나중 파일의 값이 이전 값을 대체합니다
* **목록**, 예: `permissions.deny` 또는 `sandbox.network.allowedDomains`: 두 목록이 결합되며 중복이 제거됩니다
* **중첩된 블록**, 예: `env` 또는 `sandbox`: 두 블록이 키별로 병합되며 내부의 각 키는 이 동일한 규칙을 따릅니다
* **`fallbackModel`**: 나중 체인이 이전 체인을 전체적으로 대체합니다
* **[`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces) 및 [`managedMcpServers`](/docs/ko/settings-reference#managedmcpservers)**: 동일한 이름의 나중 항목이 이전 항목을 전체적으로 대체합니다
* **[`modelPicker`](/docs/ko/settings-reference#modelpicker)**: 나중 라인업이 이전 라인업을 전체적으로 대체합니다

<span id="precedence-within-the-managed-tier" />

<span id="which-managed-source-claude-code-uses" />

<h2 id="how-claude-code-combines-managed-sources">
  Claude Code가 관리되는 소스를 결합하는 방식
</h2>

조직에서 동일한 머신에 관리되는 소스를 두 개 이상 제공할 때, [`managedSourcesBehavior`](/docs/ko/settings-reference#managedsourcesbehavior) 키가 Claude Code가 다른 소스로 수행할 작업을 결정합니다:

* **`"first-wins"`, 기본값**: Claude Code는 최소 하나의 정책 키를 제공하는 가장 높은 순위의 소스를 사용하고 [모든 관리 소스에서 읽은 키](#keys-read-from-every-admin-source)의 키를 제외하고 나머지는 무시합니다. Claude Code는 건너뛴 소스에 대해 경고를 표시하지 않습니다. `/status`는 [사용한 소스와 건너뛴 소스의 이름을 지정합니다](#read-the-source-in-/status).
* **`"merge"`**: Claude Code는 정책 키를 제공하는 모든 관리 소스를 적용하고 키의 종류별로 결합합니다: 대부분의 키에서는 더 높은 순위의 소스의 값이 적용되고, 목록은 합집합이며, 잠금은 가장 엄격한 값을 취합니다. [모든 관리되는 소스 구성](#compose-every-managed-source)에서 키를 설정할 위치와 각 종류의 키가 결합되는 방식을 설명합니다. Claude Code v2.1.242 이상이 필요합니다.

두 설정 모두 동일한 방식으로 소스의 순위를 지정합니다. 이 섹션에서 반복되는 용어는 다음과 같습니다:

* **정책 키**: 두 개의 제어 키인 [`wslInheritsWindowsSettings`](/docs/ko/settings-reference#wslinheritswindowssettings)와 [`managedSourcesBehavior`](/docs/ko/settings-reference#managedsourcesbehavior)를 제외한 모든 설정 키입니다. 이 두 키만 포함하는 관리되는 설정 파일이나 MDM 정책은 계산되지 않으며, Claude Code는 다음 소스로 이동합니다.
* **관리 소스**: 아래의 처음 세 소스 중 하나입니다. HKCU 레지스트리는 사용자가 쓸 수 있으므로 관리 소스가 아닙니다.

Claude Code는 다음 순서대로 소스를 확인합니다(우선순위가 높은 순서):

1. 원격 설정, claude.ai에서 [서버 관리 설정](/docs/ko/server-managed-settings)으로 또는 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)로 제공됩니다. Claude Code는 세션이 [적격 로그인 또는 키](/docs/ko/server-managed-settings#platform-availability)를 사용하여 Anthropic의 API에 직접 인증하거나 `/login`으로 게이트웨이에 로그인할 때만 이 소스를 가져옵니다. 다른 공급자에서 또는 `ANTHROPIC_BASE_URL`이 Anthropic의 API 이외의 다른 곳을 가리킬 때는 다음 소스에서 시작합니다.
2. MDM 또는 OS 수준 정책: macOS plist 또는 HKLM 레지스트리 키
3. 관리되는 설정 파일, `managed-settings.d/*.json` 및 `managed-settings.json`이 함께 병합됨
4. Windows의 HKCU 레지스트리, 그리고 HKLM 레지스트리 또는 Windows 관리 설정 파일이 [`wslInheritsWindowsSettings`](/docs/ko/settings-reference#wslinheritswindowssettings)를 켜고 HKCU 값도 이를 설정할 때 WSL의 HKCU 레지스트리입니다. Claude Code는 위의 소스가 정책 키를 제공하지 않고 [호스트 제공 부모 설정](#let-an-embedding-host-add-policy)이 제한적인 키를 제공하지 않을 때만 읽습니다.

이 다이어그램은 순위를 보여주며, Claude Code가 두 설정 중 하나에서 처음 세 소스에서 읽는 교차 소스 키의 예를 포함합니다:

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=53f6be49f06eff48e01422c8ae1bc2e6" className="dark:hidden" alt="원격 설정에서 상단부터 MDM, 관리되는 설정 파일, 하단의 HKCU 레지스트리까지 순위가 지정된 네 개의 관리되는 설정 소스를 보여주는 다이어그램입니다. 기본적으로 정책 키를 가진 첫 번째 소스가 정책을 제공하고 나머지는 건너뜁니다. managedSourcesBehavior가 merge로 설정되면 정책 키를 가진 모든 관리 소스가 기여하며, 키의 종류별로 결합되고, HKCU 레지스트리는 제외됩니다. 측면 패널은 샌드박스 잠금, forceRemoteSettingsRefresh, 변수별 env 병합과 같은 교차 소스 키가 HKCU 레지스트리를 제외하는 모든 관리 소스에서 읽혀짐을 보여줍니다." width="680" height="330" data-path="images/managed-source-precedence.svg" />

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence-dark.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=ae407a9a08a3d680e80cf1a2af845d71" className="hidden dark:block" alt="원격 설정에서 상단부터 MDM, 관리되는 설정 파일, 하단의 HKCU 레지스트리까지 순위가 지정된 네 개의 관리되는 설정 소스를 보여주는 다이어그램입니다. 기본적으로 정책 키를 가진 첫 번째 소스가 정책을 제공하고 나머지는 건너뜁니다. managedSourcesBehavior가 merge로 설정되면 정책 키를 가진 모든 관리 소스가 기여하며, 키의 종류별로 결합되고, HKCU 레지스트리는 제외됩니다. 측면 패널은 샌드박스 잠금, forceRemoteSettingsRefresh, 변수별 env 병합과 같은 교차 소스 키가 HKCU 레지스트리를 제외하는 모든 관리 소스에서 읽혀짐을 보여줍니다." width="680" height="330" data-path="images/managed-source-precedence-dark.svg" />

<h3 id="keys-read-from-every-admin-source">
  모든 관리 소스에서 읽은 키
</h3>

기본 `"first-wins"` 설정에서 Claude Code는 대부분의 키를 [선택한 소스](#how-claude-code-combines-managed-sources)에서만 읽고, 선택한 소스가 해당 키를 설정하지 않은 경우에도 더 낮은 순위의 소스의 값을 무시합니다.

몇 가지 키는 다르게 작동합니다. Claude Code는 모든 관리 소스에서 이들을 읽으므로, 선택한 소스가 설정하지 않을 때 더 낮은 순위의 MDM 정책이나 관리되는 설정 파일이 여전히 이들을 설정할 수 있습니다. Claude Code는 사용자가 쓸 수 있는 HKCU 레지스트리를 해당 스캔에서 제외합니다. HKCU가 유일한 소스이고 호스트가 부모 설정을 제공하지 않을 때, HKCU는 선택된 소스처럼 적용됩니다.

교차 소스 키는 다음을 포함합니다:

* `sandbox.network.allowManagedDomainsOnly` 및 `sandbox.filesystem.allowManagedReadPathsOnly`: 모든 관리 소스의 `true`가 잠금을 켭니다. 잠금이 켜져 있는 동안 Claude Code는 모든 관리 소스에서 잠금하는 허용 목록인 `sandbox.network.allowedDomains`를 `WebFetch(domain:...)` 허용 규칙 또는 `sandbox.filesystem.allowRead`와 함께 합집합합니다. 잠금이 없으면 Claude Code는 허용 목록을 다른 키처럼 취급하므로, `"first-wins"` 아래에서 선택되지 않은 관리 소스의 허용 목록은 무시됩니다.
* `allowAllClaudeAiMcps`
* `allowManagedMcpServersOnly`: 모든 관리 소스의 `true`가 MCP 허용 목록 잠금을 켭니다. 잠금이 켜져 있는 동안 관리되는 `allowedMcpServers` 목록은 하나를 설정하는 가장 높은 순위의 관리 소스에서 옵니다. 서버 관리 목록은 더 낮은 소스의 목록과 결합하지 않고 대체합니다.

  관리 소스가 목록을 설정하지 않으면, [부모 설정](#let-an-embedding-host-add-policy)이 목록을 제공하지 않는 한 거부 목록을 통과하는 모든 서버가 로드됩니다.

  잠금이 없으면 Claude Code는 적용하는 관리되는 소스에서 `allowedMcpServers`를 읽으므로, `"first-wins"` 아래에서 선택되지 않은 관리 소스의 목록은 무시됩니다. Claude Code v2.1.273 이상이 필요합니다.
* `deniedMcpServers` 및 [`disableClaudeAiConnectors`](/docs/ko/settings-reference#disableclaudeaiconnectors): 모든 관리 소스의 항목이나 `true`가 적용됩니다. Claude Code v2.1.273 이상이 필요합니다.
* 샌드박스 바이너리 경로 `sandbox.bwrapPath` 및 `sandbox.socatPath`
* 샌드박스 `ripgrep` 바이너리, [`sandbox.ripgrep`](/docs/ko/settings-reference#sandbox-ripgrep)
* `sandbox.filesystem.disabled` 및 `sandbox.network.strictAllowlist`
* [`useAutoModeDuringPlan`](/docs/ko/settings-reference#useautomodeduringplan), [`syncClaudeAiSkills`](/docs/ko/settings-reference#syncclaudeaiskills), 및 [`syncClaudeAiPlugins`](/docs/ko/settings-reference#syncclaudeaiplugins), 여기서 모든 관리 소스의 `false`가 동작을 끕니다. 개발자의 사용자 또는 로컬 설정의 `false`도 이를 끕니다. 각 키는 거부만 할 수 있습니다.
* [`enableArtifact`](/docs/ko/settings-reference#enableartifact), 여기서 모든 관리 소스의 `false`가 [Artifact 도구](/docs/ko/artifacts)를 끕니다. 개발자의 사용자, 프로젝트 또는 로컬 설정의 `false`도 이를 끕니다. 어떤 소스도 이를 다시 켤 수 없습니다. [어떤 하위 수준 값이 여전히 계산되는지](/docs/ko/settings#exceptions-to-managed-settings-precedence) 참조하세요. Claude Code v2.1.242 이상이 필요합니다.
* [`maxEffortLevel`](/docs/ko/settings-reference#maxeffortlevel), 여기서 모든 관리 소스의 가장 낮은 상한이 적용됩니다. 개발자가 자신의 설정이나 `--settings`에서 더 낮은 상한을 설정하면 Claude Code가 그것을 적용합니다. 어떤 소스도 상한을 올릴 수 없습니다. Claude Code v2.1.267 이상이 필요합니다.
* 모든 계층의 `attribution`에서 또는 더 이상 사용되지 않는 `includeCoAuthoredBy`에서 커밋 트레일러 옵트아웃
* [`forceRemoteSettingsRefresh`](/docs/ko/server-managed-settings)
* `env`, 관리 소스 전체에서 변수별로 병합됨: 각 변수는 이를 정의하는 가장 높은 우선순위 소스에서 오므로, 더 낮은 소스는 더 높은 소스가 설정하지 않은 변수를 채웁니다. 몇 가지 변수는 자신의 규칙을 따릅니다. [관리되는 소스 전체의 키별 예외](/docs/ko/server-managed-settings#per-key-exceptions-across-managed-sources)는 각각을 이름 지정합니다. Claude Code v2.1.223 이상이 필요합니다. v2.1.223 이전에는 Claude Code가 선택된 소스의 전체 `env` 블록만 적용했습니다.

[게이트웨이 로그인 키](#choose-a-delivery-mechanism)는 별도의 규칙을 따릅니다. Claude Code는 서버 관리 설정에서 이들을 읽지 않으므로, 서버 관리 설정이 선택된 소스인 동안 정책 키를 전달하는 머신의 가장 높은 순위의 관리 소스가 여전히 이들을 제공합니다. 그 아래에 순위가 지정된 관리 소스의 값이나 HKCU 레지스트리의 값은 무시됩니다.

관리 소스가 `allowManagedMcpServersOnly`를 설정하거나 `allowedMcpServers` 목록을 설정하고 그 값이 적용 중인 값이 아닐 때, `/status` 및 `claude doctor`는 해당 소스와 키의 이름을 지정합니다.

<h3 id="compose-every-managed-source">
  모든 관리되는 소스 구성
</h3>

Claude Code가 조직에서 제공하는 모든 관리 소스를 적용하도록 하려면, 배포하는 가장 높은 순위의 소스에서 [`managedSourcesBehavior`](/docs/ko/settings-reference#managedsourcesbehavior)를 `"merge"`로 설정합니다. Claude Code는 키 또는 정책 키를 전달하는 가장 높은 순위의 소스에서만 키를 읽으므로, 더 낮은 소스가 위의 소스와 병합하도록 자신을 선택할 수 없으며, 서버 관리 설정을 받지 않는 머신은 MDM 프로필에도 키가 필요합니다. 사용자가 쓸 수 있는 HKCU 레지스트리는 다른 소스와 병합되지 않습니다. Claude Code v2.1.242 이상이 필요합니다.

`"merge"` 아래에서 Claude Code는 더 낮은 소스의 목록 항목(예: `permissions.allow` 규칙 및 hooks)을 정책에 추가하므로, 가장 높은 소스 아래에 순위가 지정된 모든 소스가 관리자의 제어 아래에 있을 때만 켭니다.

이 표는 `"merge"` 아래에서 Claude Code가 각 종류의 키를 결합하는 방식을 보여줍니다. [`managedSourcesBehavior` 항목](/docs/ko/settings-reference#managedsourcesbehavior)은 세 행의 모든 키를 이름 지정합니다: 제한 허용 목록, 전체로 취한 값, 그리고 가장 높은 순위의 소스에서만 읽은 키입니다.

| 키의 종류                | Claude Code가 결합하는 방식                                                                           | 예                                                                                                                |
| :------------------- | :--------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| 목록                   | 모든 소스의 항목을 결합합니다                                                                               | `permissions.allow`, `hooks`, `sandbox.network.allowedDomains`, `deniedMcpServers`                               |
| 잠금                   | 모든 소스가 설정하는 가장 엄격한 값을 적용합니다. 더 느슨한 값은 가장 높은 순위의 소스에서만 적용됩니다                                    | `allowManagedHooksOnly`, `permissions.disableBypassPermissionsMode`, `crossSessionInbound`                       |
| 제한 허용 목록             | 더 낮은 소스의 항목을 추가하지 않고 이를 설정하는 가장 높은 순위의 소스에서 전체 목록을 취합니다                                        | `availableModels`, `allowedMcpServers`, `strictKnownMarketplaces`, `allowedChannelPlugins`, 및 `fallbackModel` 체인 |
| 전체로 취한 값             | 더 낮은 소스의 항목이나 필드를 결합하지 않고 이를 설정하는 가장 높은 순위의 소스에서 전체 값을 취합니다                                    | `sandbox.credentials.awsPairs`, `sandbox.ripgrep`                                                                |
| 제공된 MCP 서버           | 모든 소스의 서버 이름을 결합합니다. 두 소스가 동일한 이름을 설정할 때 더 높은 순위의 소스의 전체 항목을 적용합니다                             | `managedMcpServers`                                                                                              |
| 가장 높은 순위의 소스에서만 읽은 키 | 가장 높은 순위의 소스가 이를 설정하지 않은 경우에도 모든 더 낮은 소스의 키를 무시합니다                                             | `apiKeyHelper`와 같은 자격 증명 도우미, `forceLoginOrgUUID`와 같은 로그인 핀, `modelPicker`, `permissions.defaultMode`            |
| `env`                | 두 설정 모두에서 관리 소스 전체에서 변수별로 병합합니다. [모든 관리 소스에서 읽은 키](#keys-read-from-every-admin-source)에서 설명합니다 |                                                                                                                  |
| 다른 모든 키              | 이를 설정하는 가장 높은 순위의 소스에서 값을 취합니다                                                                 | `model`, `cleanupPeriodDays`                                                                                     |

머신에서 결합된 소스를 확인하려면 `/status`의 `Setting sources` 줄을 [읽으세요](#read-the-source-in-/status). 해당 섹션은 각 레이블의 의미를 설명합니다.

<h3 id="compute-the-policy-with-a-helper-program">
  도우미 프로그램으로 정책 계산
</h3>

[`policyHelper`](/docs/ko/settings-reference#policyhelper)는 MDM 정책이나 관리되는 설정 파일이 이름을 지정하는 실행 파일이며, Claude Code는 이를 실행하여 시작 시 관리되는 설정을 계산합니다. 선택된 소스가 하나를 구성하고 도우미가 `managedSettings` 객체를 내보낼 때, 해당 출력은 Claude Code가 읽는 것을 변경합니다:

* **내보낸 `managedSettings` 객체는 세션의 유일한 관리되는 설정입니다**, [그렇지 않으면 모든 관리 소스에서 읽은 키](#keys-read-from-every-admin-source)를 포함하여, [`forceRemoteSettingsRefresh` 제외, 자신의 시작 규칙이 있음](/docs/ko/settings-reference#forceremotesettingsrefresh)

도우미 실행이 실패하는 경우와 실패할 때 Claude Code가 수행하는 작업은 [도우미 실패](/docs/ko/settings-reference#helper-failures)를 참조하세요.

<span id="parent-settings-from-embedding-hosts" />

<span id="control-policy-from-an-embedding-host" />

<span id="merge-policy-from-an-embedding-host" />

<h3 id="let-an-embedding-host-add-policy">
  임베딩 호스트가 정책을 추가하도록 허용
</h3>

다른 애플리케이션이 Claude Code를 시작할 때(예: Claude Desktop, IDE 확장 또는 Agent SDK 앱), 해당 호스트는 SDK `managedSettings` 옵션을 통해 자신의 관리되는 설정을 전달할 수 있습니다. Claude Code는 이를 부모 설정이라고 부릅니다.

기본적으로 Claude Code는 관리 소스가 있을 때마다 부모 설정을 무시합니다: 서버 관리 설정, MDM 또는 OS 수준 정책, 또는 관리되는 설정 파일입니다.

부모 설정을 관리 소스와 함께 병합하도록 하려면, 가장 높은 우선순위의 관리 소스에서 [`parentSettingsBehavior`](/docs/ko/settings-reference#parentsettingsbehavior)를 `"merge"`로 설정합니다. Claude Code는 해당 소스에서만 키를 읽습니다.

Claude Code는 그 후 Claude가 수행할 수 있는 것을 제한하는 호스트의 값만 유지합니다. 알아야 할 한 가지 간격이 있습니다: `allowManaged*Only` 잠금도 설정하지 않으면 호스트의 권한 허용 규칙과 샌드박스 허용 목록이 여전히 적용됩니다. [부모 설정 제한](/docs/ko/claude-apps-gateway#restrict-parent-settings)에서 잠금을 참조하세요.

[`policyHelper`](/docs/ko/settings-reference#policyhelper)는 이 키와 관계없이 부모 병합을 끌 수 있습니다. 해당 항목은 언제인지 설명합니다.

Claude Code는 또한 부모 제공 값에 이러한 검사를 자체적으로 적용합니다:

* 모든 관리 소스가 `allowManagedPermissionRulesOnly`를 설정할 때, Claude Code는 더 높은 우선순위의 소스가 키를 설정하지 않은 경우에도 읽을 때 [부모 제공](/docs/ko/claude-apps-gateway#restrict-parent-settings) 권한 허용 규칙과 `additionalDirectories`를 삭제합니다. 키의 Claude Code가 적용하는 관리되는 설정에 대한 영향이나 병합하도록 선택한 부모 설정에서 옵니다.
* Claude Code는 적용하는 관리되는 설정에서 `forceLoginOrgUUID` 또는 `allowedMcpServers` 값을 적용하고 부모 제공 값을 차단합니다. MCP 허용 목록 잠금 외부에서 Claude Code가 적용하지 않는 더 낮은 관리 소스의 값은 부모의 값을 적용하거나 차단하지 않습니다.

  Claude Code v2.1.273 이상에서 `allowManagedMcpServersOnly`가 켜져 있는 동안, 하나를 설정하는 가장 높은 순위의 관리 소스의 `allowedMcpServers` 목록이 적용되고 부모의 값을 차단합니다([교차 소스 키](#keys-read-from-every-admin-source)로). 부모의 목록은 관리 소스가 하나를 설정하지 않을 때만 적용됩니다. [`managedSourcesBehavior`](/docs/ko/settings-reference#managedsourcesbehavior) 항목은 `"merge"` 아래에서 각 키를 제공하는 소스를 설명합니다. v2.1.223 이전에는 모든 관리 소스의 값이 부모의 값을 차단했습니다.
* `availableModels`의 경우 Claude Code는 적용하는 관리되는 설정의 값을 적용하고 부모 제공 목록을 차단합니다.
* `strictKnownMarketplaces`의 경우 Claude Code는 마찬가지로 적용하는 관리되는 설정의 목록을 적용하고 부모 제공 목록을 차단합니다. 부모의 목록은 적용된 관리 소스가 하나를 설정하지 않을 때만 적용됩니다. Claude Code v2.1.282 이상이 필요합니다.
* 부모 제공 `blockedMarketplaces`는 관리 소스가 설정하는 모든 거부 목록에 추가로 적용됩니다. Claude Code v2.1.282 이상이 필요합니다.

<h4 id="keep-cowork-folder-access-when-only-managed-rules-apply">
  관리되는 규칙만 적용될 때 Cowork 폴더 액세스 유지
</h4>

Claude Desktop 앱의 [Cowork](https://claude.com/docs/cowork/overview)는 Claude Code에서 세션을 실행하고 각 세션에 사용자가 연결하는 폴더와 같은 작업 폴더에 대한 액세스를 부여합니다. 세션을 시작할 때 제공하는 허용 규칙을 통해 액세스합니다. 관리되는 정책이 [`allowManagedPermissionRulesOnly`](/docs/ko/settings-reference#allowmanagedpermissionrulesonly)를 설정할 때, Claude Code는 관리되는 정책의 허용 규칙만 유지합니다: 호스트가 부모 설정으로, `--allowedTools`로, 또는 설정 파일에서 제공하는 허용 규칙을 삭제하므로, 해당 폴더에 대한 쓰기는 사전 승인을 잃습니다. 편집 전에 묻는 Cowork 세션에서 Cowork는 프롬프트를 표시할 수 없으며, Claude는 경로가 보호된 위치 또는 연결된 폴더 외부의 경로로 확인되므로 각 쓰기를 차단된 것으로 보고합니다.

쓰기를 복원하려면, Claude Code가 해당 머신에서 [선택](#precedence-within-the-managed-tier)하는 관리되는 소스에 해당 폴더에 대한 허용 규칙을 추가합니다: MDM 관리 플릿에서 이는 별도의 관리되는 설정 파일이 아닌 MDM 정책입니다. 이 예는 파일 형식을 사용하며, MDM 정책은 동일한 키를 취합니다. `allowManagedPermissionRulesOnly`를 설정한 상태로 유지하고 각 사용자의 홈 디렉토리에서 `CoworkProjects` 폴더 아래의 편집을 허용합니다. 경로를 사용자가 연결하는 폴더로 바꾸세요:

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true,
  "permissions": {
    "allow": [
      "Edit(~/CoworkProjects/**)"
    ]
  }
}
```

정책을 배포한 후 Claude는 새 Cowork 세션에서 해당 폴더 아래에 파일을 저장할 수 있습니다. [읽기 및 편집 규칙](/docs/ko/permissions#read-and-edit)은 경로 구문(절대 경로의 `//` 형식 포함)을 다룹니다.

<h3 id="what-a-developer-can-change">
  개발자가 변경할 수 있는 것
</h3>

개발자의 자신의 설정 파일, `--settings` 값, 및 프로젝트 파일은 관리되는 값을 재정의하지 않습니다. [예외](/docs/ko/settings#exceptions-to-managed-settings-precedence)는 더 엄격한 하위 수준 값만 계산하도록 합니다. 이 경우는 해당 규칙 외부에 있습니다:

* **세션의 모델**: 관리되는 `model`은 잠금이 아닌 기본값입니다. `--model` 및 `ANTHROPIC_MODEL`은 여전히 해당 세션의 모델을 선택하므로, [`availableModels`](/docs/ko/settings-reference#availablemodels)를 배포하여 선택을 제한합니다.
* **로컬 관리자 권한**: 머신의 관리자인 개발자는 관리되는 소스 자체를 편집할 수 있으므로, MDM 도구는 프로필이나 파일을 일정에 따라 다시 배포할 수 있으며, HKLM 레지스트리와 macOS 관리되는 기본 설정 도메인이 존재합니다.
* **서버 관리 캐시**: 서버 관리 설정은 Anthropic의 서버에서 오며, 로컬 캐시에 대한 편집은 [다음 성공적인 가져오기까지만 지속됩니다](/docs/ko/server-managed-settings#security-considerations).
* **다른 도구**: 관리되는 설정은 Claude Code만 바인딩합니다. 다른 도구에서 API를 호출하는 개발자는 이들 아래에 있지 않습니다.

<span id="verify-enforcement" />

<span id="verify-that-a-policy-is-in-force" />

<h2 id="check-that-a-policy-is-in-force">
  정책이 적용 중인지 확인
</h2>

개발자가 정책이 적용되지 않는다고 보고하거나, 롤아웃이 완료되었는지 확인한 후 전체 시스템에 배포하려는 경우가 있습니다. 해당 머신의 두 가지 명령어로 이를 확인할 수 있습니다. `/status`는 Claude Code가 선택한 관리형 소스를 표시하고, `claude doctor`는 삭제된 항목을 나열합니다.

<h3 id="read-the-source-in-/status">
  /status에서 소스 읽기
</h3>

개발자의 머신에서 Claude Code 내에서 `/status`를 실행하고 `Setting sources` 줄을 읽습니다. 관리형 소스가 적용 중일 때, 이 줄은 `Enterprise managed settings`를 나열하며 Claude Code가 선택한 소스를 괄호 안에 표시합니다:

* `(remote)`: claude.ai 또는 게이트웨이의 서버 관리 설정
* `(plist)` 또는 `(HKLM)`: MDM 또는 OS 정책
* `(file)`, `(drop-ins)`, 또는 `(file + drop-ins)`: `managed-settings.json`, 드롭인 디렉토리, 또는 둘 다
* `(remote + file, merged)` 또는 `, merged`로 끝나는 다른 목록: 조직이 [모든 관리형 소스를 구성](#compose-every-managed-source)하고 있으며, Claude Code가 나열된 소스를 정책으로 병합했습니다. 낮은 우선순위의 소스는 목록에 나타나지 않아도 여전히 `env` 변수를 제공할 수 있습니다. Claude Code v2.1.242 이상 필요
* `(HKCU)`: 사용자 쓰기 가능 레지스트리 폴백
* `(parent process)`: [임베딩 호스트](#let-an-embedding-host-add-policy)가 제공한 제한적 설정
* `(helper)`: 선택된 MDM 또는 파일 소스로 구성된 [`policyHelper`](/docs/ko/settings-reference#policyhelper)

Claude Code가 머신에서 관리형 소스를 찾았지만 선택하지 않은 경우, `Skipped sources`라는 두 번째 줄이 각 소스의 이름을 나열합니다. 이를 읽어 정책이 머신에 도달하지 않은 경우와 도달했지만 더 높은 우선순위의 소스가 재정의한 경우를 구분합니다. Claude Code v2.1.242 이상 필요합니다.

정책이 적용되지 않을 때, `Setting sources` 줄은 다음 두 가지 문제 중 어느 것인지 알려줍니다:

* **줄이 없음**: Claude Code가 정책 키를 전달하는 관리형 소스를 찾지 못했습니다.

  관리형 설정 파일을 배포한 경우, 파일이 OS의 경로에 있고 제어 키만이 아닌 [정책 키](#how-claude-code-combines-managed-sources)를 포함하는지 확인합니다. 유효하지 않은 JSON 파일은 이 상태를 생성하지 않습니다. Claude Code는 [시작을 거부](#find-entries-claude-code-dropped)합니다.

  대신 서버 관리 설정을 통해 배포한 경우, `claude doctor`를 실행하면 [가져오기 결과](/docs/ko/server-managed-settings#verify-settings-delivery)를 보고합니다.
* **줄이 배포한 소스가 아닌 다른 소스의 이름을 지정**: 더 높은 우선순위의 소스가 있고 Claude Code가 사용자의 소스를 무시했으며, `Skipped sources`가 이를 나열합니다. [Claude Code가 관리형 소스를 결합하는 방법](#how-claude-code-combines-managed-sources)에서 순서를 확인합니다.

<span id="invalid-entries-in-managed-settings" />

<h3 id="find-entries-claude-code-dropped">
  Claude Code가 삭제한 항목 찾기
</h3>

관리형 설정 파일, MDM 프로필, 레지스트리 값 또는 서버 관리 페이로드가 스키마 검증에 실패하면, Claude Code는 먼저 수정할 수 있는 개별 항목(예: 하나의 잘못된 권한 규칙)을 건너뛰고 각각에 대해 경고를 표시한 다음, 여전히 실패하는 최상위 키를 삭제하고 남은 모든 유효한 키를 계속 적용합니다.

Claude Code는 [`policyHelper`](/docs/ko/settings-reference#policyhelper)가 내보내는 `managedSettings`에 더 엄격합니다. 동일한 항목 수정을 수행하지만, 생존하는 스키마 위반은 전체 헬퍼 실행을 실패하게 하고, 시작 시 Claude Code는 시작을 거부하며, 이는 0이 아닌 값으로 종료되는 헬퍼와 동일합니다.

관리형 설정 파일, 드롭인 파일, MDM plist 또는 HKLM 레지스트리 값이 있지만 JSON 객체로 구문 분석할 수 없으면, Claude Code는 시작을 거부하고 [소스의 이름을 지정하는 오류](/docs/ko/errors#managed-settings-document-could-not-be-parsed)를 인쇄합니다. 다른 관리자 소스가 유효한 정책을 전달하는 경우에도 마찬가지입니다. 각 소스는 다음의 경우에 이런 방식으로 실패합니다:

* **관리형 설정 파일 또는 드롭인 파일**: 파일이 유효한 JSON이 아니거나 최상위 수준이 객체가 아님
* **MDM plist**: macOS의 `plutil`이 plist가 손상되었다고 보고하거나, 변환된 콘텐츠가 JSON 객체가 아님
* **HKLM 레지스트리 값**: `Settings` 값이 문자열이 아니거나, 비어 있거나, JSON 객체를 포함하지 않음

세 가지 소스 상태는 이 거부를 유발하지 않습니다:

* 없는 파일, 프로필 또는 레지스트리 값은 실패가 아닙니다. Claude Code는 해당 소스 없이 실행됩니다.
* 빈 관리형 설정 파일은 `{}`로 계산됩니다.
* 사용자 쓰기 가능 HKCU 레지스트리 키의 손상된 값은 시작을 차단하지 않습니다. Claude Code는 이를 `/status` 및 `claude doctor`의 알림으로 보고합니다.

관리형 설정 파일, 드롭인 파일 또는 `managed-settings.d/` 디렉토리를 읽을 수 없고 관리자 소스가 정책을 제공하지 않으면, claude.ai 또는 Claude Console 자격 증명으로 로그인한 세션은 관리자에게 문의하라는 메시지와 함께 시작 시 종료됩니다.

삭제된 항목을 찾으려면 다음 세 위치 중 하나를 확인합니다:

* 대화형 세션은 시작 시 잘못된 항목을 나열하는 대화 상자를 표시합니다.
* `-p`를 사용한 비대화형 실행은 stderr에 요약을 인쇄합니다.
* [`claude doctor`](/docs/ko/debug-your-config)는 각 잘못된 항목을 소스 및 필드와 함께 나열합니다.

<h4 id="keys-that-fail-closed">
  폐쇄 상태로 실패하는 키
</h4>

일부 적용 키는 유효하지 않을 때 삭제되지 않습니다. Claude Code는 값이 수정될 때까지 더 엄격한 폴백을 적용합니다. 표는 각 키에 대해 적용되는 내용을 보여줍니다:

| 필드                            | 존재하지만 유효하지 않을 때의 동작                                                                                                                                                                                                                                                                            |
| :---------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers`           | 값이 수정될 때까지 빈 허용 목록으로 적용되므로, 사용자가 추가하는 MCP 서버는 허용되지 않습니다. 조직이 [`managedMcpServers`](/docs/ko/settings-reference#managedmcpservers)를 통해 전달하는 서버는 여전히 로드되고, `managed-mcp.json` 서버는 [서버 평가 방법](/docs/ko/managed-mcp#how-a-server-is-evaluated)에 따라 로드됩니다. 개별 유효하지 않은 항목은 제거되고 유효한 부분 집합이 적용됩니다.              |
| `allowedHttpHookUrls`         | Claude Code는 값을 수정할 때까지 빈 관리형 [허용 목록](/docs/ko/settings-reference#allowedhttphookurls)을 적용하므로, HTTP 훅은 다른 설정 파일이 해당 URL을 나열하는 경우에만 실행됩니다. 개별 항목만 유효하지 않으면, Claude Code는 해당 항목을 제거하고 나머지를 적용합니다.                                                                                                     |
| `httpHookAllowedEnvVars`      | Claude Code는 값을 수정할 때까지 빈 관리형 [허용 목록](/docs/ko/settings-reference#httphookallowedenvvars)을 적용하므로, 헤더 변수는 다른 설정 파일이 이름을 지정하는 경우에만 보간됩니다. 개별 항목만 유효하지 않으면, Claude Code는 해당 항목을 제거하고 나머지를 적용합니다.                                                                                                       |
| `allowedChannelPlugins`       | 값을 수정할 때까지 빈 허용 목록을 적용하므로, `--channels`에 전달된 채널 플러그인은 허용되지 않습니다. 개별 항목만 유효하지 않으면, 해당 항목을 제거하고 나머지를 적용합니다.                                                                                                                                                                                      |
| `strictKnownMarketplaces`     | 값이 수정될 때까지 빈 허용 목록으로 적용되므로, [마켓플레이스 소스](/docs/ko/plugins/org#restrict-what-users-can-install)는 허용되지 않습니다. 유효하지 않거나 적용할 수 없는 개별 항목(예: 컴파일되지 않는 `hostPattern` 정규식)은 제거되고 유효한 부분 집합이 적용됩니다.                                                                                                            |
| `allowManagedHooksOnly`       | 수정될 때까지 `true`로 처리됩니다: [훅 제한](/docs/ko/settings-reference#allowmanagedhooksonly)이 적용되고, `disableCommandPluginSources`가 명시적으로 `false`가 아닌 한, 명령 소스 플러그인은 비활성화됩니다.                                                                                                                                    |
| `allowManagedMcpServersOnly`  | `true`로 처리됩니다.                                                                                                                                                                                                                                                                                 |
| `disableCommandPluginSources` | `true`로 처리되므로, 명령 소스 플러그인은 값이 수정될 때까지 비활성화된 상태로 유지됩니다.                                                                                                                                                                                                                                         |
| `disableSideloadFlags`        | 값이 수정될 때까지 `true`로 처리되며, [`disableSideloadFlags`](/docs/ko/settings-reference#disablesideloadflags)에 나열된 효과가 있습니다.                                                                                                                                                                                  |
| `availableModels`             | 수정될 때까지 빈 허용 목록으로 적용되므로, 기본 모델만 사용 가능합니다. 문자열이 아닌 항목은 제거되고 유효한 부분 집합이 적용됩니다.                                                                                                                                                                                                                   |
| `enforceAvailableModels`      | `true`로 처리됩니다.                                                                                                                                                                                                                                                                                 |
| `syncClaudeAiPlugins`         | `false`로 처리되므로, [claude.ai 플러그인](/docs/ko/settings-reference#syncclaudeaiplugins)의 동기화는 값이 수정될 때까지 꺼져 있습니다.                                                                                                                                                                                         |
| `forceLoginOrgUUID`           | 값이 수정될 때까지 조직이 로그인하도록 허용되지 않습니다.                                                                                                                                                                                                                                                               |
| `gatewayInternalNetworks`     | 유효하지 않은 값이 머신의 최상위 관리형 소스에서 오는 경우, `/login`은 값이 수정될 때까지 해당 머신의 모든 새로운 [클라우드 게이트웨이](/docs/ko/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) 로그인을 거부합니다.                                                                                                                        |
| `crossSessionInbound`         | 가장 제한적인 값인 `refuse`로 처리되므로, 인바운드 [크로스 세션 메시지](/docs/ko/cross-session-messaging#control-inbound-messages)는 값이 수정될 때까지 거부됩니다. 개발자는 [경고](/docs/ko/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse)를 봅니다.                                                                                       |
| `deniedMcpServers`            | 개별 유효하지 않은 항목은 제거되고 유효한 부분 집합이 적용됩니다. 완전히 유효하지 않은 값은 경고와 함께 삭제됩니다. 모든 서버를 거부하면 정책이 이름을 지정하지 않은 서버를 차단하기 때문입니다.                                                                                                                                                                                 |
| `blockedMarketplaces`         | 개별 유효하지 않은 항목은 제거되고 유효한 부분 집합이 적용됩니다. 구문 분석되지만 절대 일치할 수 없는 항목(예: 컴파일되지 않는 `hostPattern` 정규식)은 경고와 함께 유지됩니다. 값이 수정될 때까지 아무것도 차단하지 않지만, [마켓플레이스 제한](/docs/ko/plugins/org#restrict-what-users-can-install)은 활성 상태로 유지됩니다. 완전히 유효하지 않은 값은 경고와 함께 삭제됩니다. 모든 마켓플레이스를 차단하면 정책이 이름을 지정하지 않은 소스를 차단하기 때문입니다. |
| `sandbox.credentials`         | 복구 가능한 유효하지 않은 항목은 경고와 함께 `mode: "deny"`로 저하되고, 복구 불가능한 항목은 제거되며, 유효한 항목은 계속 적용됩니다. [관리형 설정의 유효하지 않은 자격 증명 항목](/docs/ko/settings-reference#invalid-credential-entries-in-managed-settings) 참조                                                                                                       |

`allowedHttpHookUrls` 및 `httpHookAllowedEnvVars`는 설정 파일 전체에서 병합되므로, 사용자, 프로젝트 또는 로컬 설정의 항목은 관리형 목록이 비어 있는 동안에도 계속 적용됩니다.

이 두 키와 `allowedChannelPlugins`에 대한 폴백은 Claude Code v2.1.267 이상이 필요합니다. 이전 버전은 값이나 항목이 유효하지 않으면 전체 키를 삭제합니다. `strictKnownMarketplaces`, `blockedMarketplaces` 및 `disableSideloadFlags` 폴백은 Claude Code v2.1.277 이상이 필요합니다. 이전 버전은 값이나 항목이 유효하지 않으면 전체 키를 삭제합니다.

`requiredMinimumVersion` 및 `requiredMaximumVersion`은 설계상 개방적으로 실패합니다. 유효하지 않은 값은 적용되지 않고 삭제됩니다.

이 허용은 관리형 설정에만 적용됩니다. 사용자, 프로젝트 및 로컬 설정 파일은 엄격합니다. JSON이나 최상위 수준 형태가 검증에 실패하는 파일은 전체적으로 거부되고 보고되며, 손상된 권한 규칙과 같이 실패하는 개별 항목은 경고와 함께 건너뛰어지고 파일의 나머지는 적용됩니다.

<span id="managed-only-settings" />

<h2 id="keys-only-a-managed-source-can-set">
  관리형 소스만 설정할 수 있는 키
</h2>

Claude Code는 다음 키들을 관리형 소스에서만 읽습니다. 사용자 또는 프로젝트 설정 파일에 이들을 배치하면 효과가 없습니다.

대부분은 잠금입니다. 잠금이 관리하는 값(예: 권한 규칙 또는 `sandbox.network.allowedDomains`)은 모든 수준에서 설정할 수 있는 일반 키이며, 잠금은 Claude Code에 관리형 값만 준수하도록 지시합니다.

이 표는 권한, 플러그인 및 전달 제어를 다룹니다. 여기에 나열되지 않은 키의 경우, [설정 참조](/docs/ko/settings-reference#all-settings) 인덱스의 범위 열에 관리형 전용 여부가 표시됩니다. 그곳의 나머지 관리형 전용 키에는 게이트웨이 로그인 URL, 버전, 브라우저, 모바일 시뮬레이터, SSH 호스트, Desktop 로컬 세션, 샌드박스 바이너리 경로, 모델 가격 책정 및 CLAUDE.md 제어가 포함됩니다.

| 설정                                                                                                                    | 설명                                                                                                                                                                                                                                                                                                                                                                                |
| :-------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`allowAllClaudeAiMcps`](/docs/ko/settings-reference#allowallclaudeaimcps)                                                 | Claude Code가 자체적으로 가져오는 claude.ai 커넥터를 배포된 `managed-mcp.json`과 함께 로드합니다. 이는 커넥터를 억제하는 대신 추가로 로드합니다                                                                                                                                                                                                                                                                                |
| [`allowedChannelPlugins`](/docs/ko/settings-reference#allowedchannelplugins)                                               | 메시지를 푸시할 수 있는 채널 플러그인의 허용 목록입니다. 설정되면 기본 Anthropic 허용 목록을 대체합니다. `channelsEnabled: true`가 필요합니다. [실행할 수 있는 채널 플러그인 제한](/docs/ko/channels#restrict-which-channel-plugins-can-run)을 참조하세요                                                                                                                                                                                                |
| [`allowManagedHooksOnly`](/docs/ko/settings-reference#allowmanagedhooksonly)                                               | `true`일 때 실행되는 훅을 제한합니다. [allowManagedHooksOnly에서 실행되는 항목](/docs/ko/settings-reference#what-runs-under-allowmanagedhooksonly)에서 전체 효과 목록을 참조하세요                                                                                                                                                                                                                                        |
| [`allowManagedMcpServersOnly`](/docs/ko/settings-reference#allowmanagedmcpserversonly)                                     | `true`일 때 관리형 설정의 `allowedMcpServers`만 적용됩니다. `deniedMcpServers`는 여전히 모든 소스에서 병합됩니다. 이를 설정할 수 있는 관리형 소스는 [모든 관리자 소스에서 읽는 키](#keys-read-from-every-admin-source)를 참조하고, [관리형 MCP 구성](/docs/ko/managed-mcp)을 참조하세요                                                                                                                                                                       |
| [`allowManagedPermissionRulesOnly`](/docs/ko/settings-reference#allowmanagedpermissionrulesonly)                           | 관리형 설정을 권한 규칙의 유일한 설정 소스로 만듭니다. 항목은 무시하는 모든 소스를 나열합니다                                                                                                                                                                                                                                                                                                                             |
| [`blockedMarketplaces`](/docs/ko/settings-reference#blockedmarketplaces)                                                   | 마켓플레이스 소스의 차단 목록입니다. 차단된 소스는 다운로드 전에 확인되므로 파일 시스템에 절대 닿지 않습니다. [관리형 마켓플레이스 제한](/docs/ko/plugins/org#restrict-what-users-can-install)을 참조하세요                                                                                                                                                                                                                                            |
| [`channelsEnabled`](/docs/ko/settings-reference#channelsenabled)                                                           | 조직에 대해 [채널](/docs/ko/channels)을 허용합니다. 각 플랜의 기본값은 [엔터프라이즈 제어](/docs/ko/channels#enterprise-controls)를 참조하세요                                                                                                                                                                                                                                                                                 |
| [`disableCommandPluginSources`](/docs/ko/settings-reference#disablecommandpluginsources)                                   | `true`일 때 [`command` 플러그인 소스](/docs/ko/plugins/marketplace-reference#command-plugin-source)를 완전히 차단하므로 마켓플레이스에서 선언한 명령이 실행되지 않습니다. 또한 마켓플레이스 [`headersHelper` 명령](/docs/ko/plugins/host-marketplace#authenticate-archive-downloads)을 차단합니다. 단, 관리형 설정 자체에서 선언한 마켓플레이스는 제외됩니다. 설정되지 않으면 `allowManagedHooksOnly`를 따릅니다. Claude Code v2.1.229 이상이 필요하며, `headersHelper` 차단은 v2.1.238 이상이 필요합니다 |
| [`disableSideloadFlags`](/docs/ko/settings-reference#disablesideloadflags)                                                 | 시작 시 `--plugin-dir`, `--plugin-url`, `--agents` 및 `--mcp-config` 플래그를 거부합니다. 클라우드 세션에서 Claude Code는 `--mcp-config`를 통해 서버가 전달한 MCP 서버를 삭제합니다. 단, 인프로세스 `type: "sdk"` 항목은 제외하고 세션을 시작합니다. Claude Code v2.1.193 이상이 필요합니다                                                                                                                                                           |
| [`forceRemoteSettingsRefresh`](/docs/ko/settings-reference#forceremotesettingsrefresh)                                     | `true`일 때 원격 관리형 설정이 새로 가져올 때까지 CLI 시작을 차단하고 가져오기가 실패하면 종료합니다. [실패 폐쇄 적용](/docs/ko/server-managed-settings#enforce-fail-closed-startup)을 참조하세요                                                                                                                                                                                                                                         |
| [`managedMcpServers`](/docs/ko/settings-reference#managedmcpservers)                                                       | 모든 사용자에게 자신의 서버와 함께 제공되는 원격 MCP 서버입니다. 이는 서버를 제공하며 아무것도 잠금하지 않습니다. [관리형 설정을 통해 서버 제공](/docs/ko/managed-mcp#provide-servers-through-managed-settings)을 참조하세요. Claude Code v2.1.259 이상이 필요합니다                                                                                                                                                                                            |
| [`managedSourcesBehavior`](/docs/ko/settings-reference#managedsourcesbehavior)                                             | Claude Code가 최우선 관리형 소스만 적용하는지 또는 [모든 관리형 소스를 구성](#compose-every-managed-source)하는지 여부입니다                                                                                                                                                                                                                                                                                         |
| [`parentSettingsBehavior`](/docs/ko/settings-reference#parentsettingsbehavior)                                             | 호스트에서 제공한 부모 설정이 관리형 정책 아래에 병합되는지 여부입니다                                                                                                                                                                                                                                                                                                                                           |
| [`pluginSuggestionMarketplaces`](/docs/ko/settings-reference#pluginsuggestionmarketplaces)                                 | Claude Code가 사용자에게 제안할 수 있는 플러그인의 마켓플레이스입니다                                                                                                                                                                                                                                                                                                                                       |
| [`pluginTrustMessage`](/docs/ko/settings-reference#plugintrustmessage)                                                     | 설치 전에 표시되는 플러그인 신뢰 경고에 추가되는 사용자 정의 메시지입니다                                                                                                                                                                                                                                                                                                                                         |
| [`policyHelper`](/docs/ko/settings-reference#policyhelper)                                                                 | 시작 시 관리형 설정을 계산하는 실행 파일입니다. [정책 도우미로 관리형 설정 계산](/docs/ko/settings-reference#policyhelper)을 참조하세요                                                                                                                                                                                                                                                                                       |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](/docs/ko/settings-reference#sandbox-filesystem-allowmanagedreadpathsonly) | `true`일 때 관리형 설정의 `filesystem.allowRead` 경로만 적용됩니다. `denyRead`는 여전히 모든 소스에서 병합됩니다                                                                                                                                                                                                                                                                                                 |
| [`sandbox.network.allowManagedDomainsOnly`](/docs/ko/settings-reference#sandbox-network-allowmanageddomainsonly)           | 관리형 `allowedDomains` 및 `WebFetch(domain:...)` 허용 규칙만 준수합니다. 다른 도메인을 프롬프트 없이 차단합니다                                                                                                                                                                                                                                                                                                 |
| [`strictKnownMarketplaces`](/docs/ko/settings-reference#strictknownmarketplaces)                                           | 사용자가 추가하고 플러그인을 설치할 수 있는 플러그인 마켓플레이스 소스를 제어합니다. [관리형 마켓플레이스 제한](/docs/ko/plugins/org#restrict-what-users-can-install)을 참조하세요                                                                                                                                                                                                                                                           |
| [`strictPluginOnlyCustomization`](/docs/ko/settings-reference#strictpluginonlycustomization)                               | 사용자 및 프로젝트 소스의 스킬, 에이전트, 훅 및 MCP 서버를 차단합니다. `true`는 네 가지 모두를 잠금하고, 배열은 어느 것을 잠금할지 지정합니다                                                                                                                                                                                                                                                                                           |
| [`wslInheritsWindowsSettings`](/docs/ko/settings-reference#wslinheritswindowssettings)                                     | HKLM 레지스트리 또는 `C:\Program Files\ClaudeCode` 아래의 파일에 설정되면 WSL이 Windows 정책 체인을 읽고, 해당 디렉터리 아래의 관리형 설정 파일 또는 드롭인이 [정책 키](#how-claude-code-combines-managed-sources)를 전달하지 않을 때만 `/etc/claude-code`를 읽습니다. 항목은 순서를 제공합니다                                                                                                                                                              |

<Note>
  Team 및 Enterprise 플랜에서 소유자는 [Claude Code 관리자 설정](https://claude.ai/admin-settings/claude-code)에서 조직 전체에 대해 [원격 제어](/docs/ko/remote-control) 및 [클라우드 세션](/docs/ko/claude-code-on-the-web)을 활성화하거나 비활성화합니다. 원격 제어는 [`disableRemoteControl`](/docs/ko/settings-reference#disableremotecontrol) 설정으로 장치별로 추가로 비활성화할 수 있습니다. 클라우드 세션에는 장치별 관리형 설정 키가 없습니다.

  이러한 조직 설정이 특정 머신에 도달했는지 확인하려면 해당 머신에서 `claude doctor`를 실행하고 `Organization policy` 줄을 읽으세요. 이 줄은 Claude Code가 정책을 로드한 위치 또는 로드하지 못한 이유를 표시합니다. Claude Code v2.1.261 이상이 필요합니다. 실행 중인 세션에서 정책이 로드되지 않으면 `/status`가 동일한 줄을 표시합니다.
</Note>

<h2 id="turn-telemetry-off-for-your-organization">
  조직의 원격 분석 끄기
</h2>

Claude Code는 기본적으로 Anthropic API를 사용하는 세션에서 Anthropic 운영 [원격 분석](/docs/ko/data-usage#telemetry-services)을 보냅니다. 직접, LLM 게이트웨이를 통해, 또는 사용자 정의 `ANTHROPIC_BASE_URL`을 통해. [API 제공자별 기본 동작](/docs/ko/data-usage#default-behaviors-by-api-provider)에서 어떤 제공자가 보내는지 설명합니다. 각 사람의 셸에 의존하지 않고 모든 개발자에 대해 끄려면, 관리형 설정의 `env` 블록을 통해 `DISABLE_TELEMETRY`를 전달합니다. 이 예제는 정책이 도달하는 모든 사람에 대해 `DISABLE_TELEMETRY`를 설정합니다:

```json theme={null}
{
  "env": {
    "DISABLE_TELEMETRY": "1"
  }
}
```

Claude Code는 `1` 값을 [승인 대화 상자](/docs/ko/server-managed-settings#environment-variables-and-the-approval-dialog)를 표시하지 않고 적용합니다.

원격 분석을 끄면, Claude Code는 정책이 도달하는 개발자에 대해 조직의 [분석 대시보드](/docs/ko/analytics)를 공급하는 사용 데이터 전송을 중단합니다. 변수는 또한 기능 플래그 가져오기를 끕니다. 이는 원격 제어, 기본 자동 모드, 및 [기능 플래그 가져오기가 필요한 다른 기능](/docs/ko/env-vars#features-that-need-feature-flag-fetching)을 해당 개발자에게 사용할 수 없게 만듭니다.

[정책이 적용되는 위치 및 시기](#where-and-when-a-policy-applies)에서 각 전달 메커니즘이 어떤 표면에 도달하는지 설명하고, [플랫폼 가용성](/docs/ko/server-managed-settings#platform-availability)에서 어떤 세션이 서버 관리형 설정 가져오기를 건너뛰는지 설명합니다.

조직이 고객 관리 암호화 키를 사용하고 Claude Code를 게이트웨이를 통해 라우팅하는 경우, [프록시 및 게이트웨이 구성](/docs/ko/third-party-integrations#configure-proxies-and-gateways)에서 해당 세션이 이 변수를 필요로 하는 이유를 설명합니다.

<h2 id="see-also">
  참고 항목
</h2>

* [조직을 위해 Claude Code 설정](/docs/ko/admin-setup): 적용할 항목 결정 및 방법
* [서버 관리형 설정](/docs/ko/server-managed-settings): claude.ai 콘솔 또는 게이트웨이에서 정책 전달
* [관리형 MCP 구성](/docs/ko/managed-mcp): 개발자가 사용할 수 있는 MCP 서버 제어
* [모든 설정](/docs/ko/settings-reference): 모든 키, 관리형 소스가 설정할 수 있는지 여부 포함
* [설정 파일 예제](/docs/ko/settings-example#an-organizations-managed-settings): 관리형 키의 형태를 보여주는 완전한 `managed-settings.json`
