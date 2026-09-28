> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 서버 관리 설정 구성

> 기기 관리 인프라 없이 서버 전달 설정을 통해 조직을 위해 Claude Code를 중앙에서 구성합니다.

서버 관리 설정을 통해 조직 소유자는 claude.ai 콘솔의 [**관리자 설정 > Claude Code > 관리 설정**](https://claude.ai/admin-settings/claude-code)에서 Claude Code를 중앙에서 구성할 수 있습니다. Claude Code 클라이언트는 사용자가 적격 자격증명으로 인증하고 서버 관리 전달이 지원되는 플랫폼에서 이러한 설정을 자동으로 가져옵니다. 적격 자격증명 및 플랫폼에 대해서는 [플랫폼 가용성](#platform-availability)을 참조하십시오.

<Note>
  서버 관리 설정은 [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_teams#team-&-enterprise) 및 [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_enterprise) 고객에게 제공됩니다.
</Note>

<h2 id="requirements">
  요구사항
</h2>

서버 관리 설정을 사용하려면 다음이 필요합니다.

* Claude for Teams 또는 Claude for Enterprise 플랜
* Claude 조직에서 구성을 보고 편집할 수 있는 Owner 또는 Primary Owner 역할
* `api.anthropic.com`에 대한 네트워크 액세스

<h2 id="choose-between-server-managed-and-endpoint-managed-settings">
  서버 관리 설정과 엔드포인트 관리 설정 중 선택
</h2>

Claude Code는 중앙 집중식 구성을 위한 두 가지 방식을 지원합니다. 서버 관리 설정은 Anthropic의 서버에서 구성을 전달합니다. [엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms)은 기본 OS 정책(macOS 관리 기본 설정, Windows 레지스트리) 또는 관리 설정 파일을 통해 기기에 직접 배포됩니다.

| 방식                                                          | 최적 대상                         | 보안 모델                                                     |
| :---------------------------------------------------------- | :---------------------------- | :-------------------------------------------------------- |
| **서버 관리 설정**                                                | MDM이 없는 조직 또는 관리되지 않는 기기의 사용자 | Claude Code가 시작 시 Anthropic의 서버에서 가져오고 세션 중 매시간 새로 고치는 설정 |
| **[엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms)** | MDM 또는 엔드포인트 관리가 있는 조직        | MDM 구성 프로필, 레지스트리 정책 또는 관리 설정 파일을 통해 기기에 배포되는 설정          |

기기가 MDM 또는 엔드포인트 관리 솔루션에 등록된 경우, 엔드포인트 관리 설정은 설정 파일을 OS 수준에서 사용자 수정으로부터 보호할 수 있으므로 더 강력한 보안 보장을 제공합니다. 엔드포인트 관리 설정은 Anthropic 호스팅 환경의 [클라우드 세션](/docs/ko/model-config#surface-coverage)에 도달하지 않으므로, 개발자가 클라우드 세션을 실행하는 조직은 서버 관리 설정도 함께 구성해야 합니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments)의 세션도 러너 이미지의 관리 설정 파일을 읽습니다. 아래의 [설정 우선순위](#settings-precedence)는 해당 파일이 적용되는 시기를 나타냅니다.

<h2 id="configure-server-managed-settings">
  서버 관리 설정 구성
</h2>

<Steps>
  <Step title="관리 콘솔 열기">
    claude.ai 콘솔에서 [**관리 설정 > Claude Code > 관리 설정**](https://claude.ai/admin-settings/claude-code)으로 이동합니다.

    링크가 Claude Code 페이지 대신 다른 관리 설정 페이지로 리디렉션되면 계정에 필요한 역할이 없습니다. 관리자 및 기타 비 소유자 역할은 관리 설정을 보거나 편집할 수 없으므로 조직의 소유자 또는 주 소유자에게 변경을 요청하십시오. [액세스 제어](#access-control)를 참조하십시오.
  </Step>

  <Step title="설정 정의">
    구성을 JSON으로 추가합니다. [`settings.json`에서 사용 가능한 모든 설정](/docs/ko/settings-reference#all-settings)이 지원되며, OS 수준 정책 전달로 제한된 설정을 제외합니다. [현재 제한사항](#current-limitations)에서 해당 짧은 목록을 참조하십시오. 여기에는 [hooks](/docs/ko/hooks), [환경 변수](/docs/ko/env-vars), 및 `allowManagedPermissionRulesOnly`와 같은 [관리 전용 설정](/docs/ko/managed-settings#managed-only-settings)이 포함됩니다.

    이 예제는 권한 거부 목록을 적용하고, 사용자가 권한을 우회하는 것을 방지하며, 권한 규칙을 관리 설정에 정의된 규칙으로만 제한합니다. `Bash(curl *)` 규칙은 `/usr/bin/curl` 또는 `sh -c 'curl …'`이 아닌 [Claude가 작성하는 방식](/docs/ko/permissions#bash-rule-limits)으로 `curl`과 일치합니다. 명령 텍스트에 의존하지 않는 네트워크 적용의 경우 [`sandbox` 블록에 `allowManagedDomainsOnly`](/docs/ko/sandboxing#configure-the-sandbox-for-your-organization)를 추가합니다.

    ```json theme={null}
    {
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Hook은 `settings.json`과 동일한 형식을 사용합니다.

    이 예제는 조직 전체에서 모든 파일 편집 후 감사 스크립트를 실행합니다:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              { "type": "command", "command": "/usr/local/bin/audit-edit.sh" }
            ]
          }
        ]
      }
    }
    ```

    Hook은 셸 명령을 실행하므로 대화형 세션의 사용자는 Claude Code가 이를 적용하기 전에 [보안 승인 대화](#security-approval-dialogs)를 봅니다.

    [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) 분류기를 구성하여 조직이 신뢰하는 저장소, 버킷 및 도메인을 알도록 하려면 `autoMode` 블록을 동일한 방식으로 전달하십시오. `autoMode` 항목이 분류기가 차단하는 것에 어떻게 영향을 미치는지, 그리고 `environment`, `allow`, `soft_deny`, 및 `hard_deny` 필드에 대한 중요한 경고는 [자동 모드 구성](/docs/ko/auto-mode-config)을 참조하십시오.
  </Step>

  <Step title="저장 및 배포">
    변경 사항을 저장합니다. Claude Code 클라이언트는 다음 시작 또는 시간별 폴링 주기에 업데이트된 설정을 수신합니다.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  설정 전달 확인
</h3>

설정이 적용되고 있는지 확인하려면 사용자에게 Claude Code를 다시 시작하도록 요청합니다. 구성에 [보안 승인 대화](#security-approval-dialogs)를 트리거하는 설정이 포함된 경우, 사용자는 Claude Code가 다음 시작 또는 실행 중인 대화형 세션에서 1시간 이내에 설정을 가져올 때 관리 설정을 설명하는 프롬프트를 봅니다. 사용자가 `/permissions`를 실행하여 유효한 권한 규칙을 확인하도록 하여 관리 권한 규칙이 활성화되어 있는지 확인할 수도 있습니다.

특정 머신에서 가져오기 결과를 확인하려면 사용자가 `claude doctor`를 실행하고 `Managed settings (remote)` 줄을 읽도록 하십시오. Claude Code v2.1.248 이상이 필요합니다. 이 줄은 다음 네 가지 결과 중 하나를 보고합니다:

* 전달된 설정이 로드됨
* 조직에 서버 관리 설정이 구성되지 않음
* 가져오기 실패, 원인 및 캐시된 정책이 여전히 적용되는지 여부
* Claude Code가 가져오기를 건너뜀, 이유 포함. [플랫폼 가용성](#platform-availability)에서 건너뛰는 공급자 및 구성을 참조하십시오

가져오기가 진행 중인 동안 줄은 대신 그것을 보고합니다.

실행 중인 세션에서 `/status`는 가져오기 실패 후 동일한 줄을 표시하며, 타사 공급자 변수 또는 사용자의 셸에서 내보낸 사용자 정의 `ANTHROPIC_BASE_URL`과 같은 일부 건너뛴 가져오기 원인의 경우입니다.

<h3 id="access-control">
  액세스 제어
</h3>

다음 역할이 서버 관리 설정을 관리할 수 있습니다:

* **주 소유자**
* **소유자**

설정 변경이 조직의 모든 사용자에게 적용되므로 신뢰할 수 있는 담당자에게만 액세스를 제한합니다.

<h3 id="managed-only-settings">
  관리 전용 설정
</h3>

대부분의 [설정 키](/docs/ko/settings-reference#all-settings)는 모든 범위에서 작동합니다. 소수의 키는 관리 설정에서만 읽혀지며 사용자 또는 프로젝트 설정 파일에 배치될 때 효과가 없습니다. 권한 및 플러그인 제어에 대해 [관리 전용 설정](/docs/ko/managed-settings#managed-only-settings)을 참조하거나, 전체 집합에 대해 [모든 설정](/docs/ko/settings-reference#all-settings) 인덱스의 범위 열을 읽으십시오.

<h3 id="current-limitations">
  현재 제한사항
</h3>

서버 관리 설정은 다음과 같은 제한사항이 있습니다:

* 설정은 조직의 모든 사용자에게 균일하게 적용됩니다. 그룹별 구성은 아직 지원되지 않습니다.
* [`managed-mcp.json`](/docs/ko/managed-mcp) 파일은 서버 관리 설정을 통해 배포할 수 없습니다. 대신 `allowedMcpServers` 및 `deniedMcpServers` 정책 키를 배포하십시오. Claude Code v2.1.259 이상에서는 [`managedMcpServers`](/docs/ko/managed-mcp#provide-servers-through-managed-settings)를 통해 원격 서버를 제공할 수도 있으며, 이는 `http` 및 `sse` 서버만 허용하고 파일이 하는 방식으로 독점적 제어를 하지 않습니다.

  Claude Code는 [시스템 경로](/docs/ko/managed-mcp#exclusive-control-with-managed-mcp-json)에 배포된 `managed-mcp.json`을 관리 설정 계층과 별도로 읽으므로, 서버 관리 설정이 적용 중일 때도 파일이 여전히 적용됩니다.
* OS 수준 정책 소스로 제한된 설정(예: `policyHelper` 및 `wslInheritsWindowsSettings`)은 적용되지 않습니다. 대신 MDM 또는 시스템 `managed-settings.json` 파일을 통해 배포하십시오. 그렇게 배포된 `policyHelper`는 [관리 계층 내 우선순위](/docs/ko/managed-settings#precedence-within-the-managed-tier)에서 선택된 소스가 해당 소스일 때만 실행됩니다.

<h2 id="settings-delivery">
  설정 전달
</h2>

<h3 id="settings-precedence">
  설정 우선순위
</h3>

서버 관리 설정과 [엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms)은 모두 Claude Code [설정 계층](/docs/ko/settings#settings-precedence)의 최상위 계층을 차지합니다. [관리 설정 우선순위의 예외](/docs/ko/settings#exceptions-to-managed-settings-precedence)를 제외하고는 명령줄 인수를 포함한 다른 설정 수준이 이들을 재정의할 수 없습니다.

관리 계층 내에서 Claude Code는 기본적으로 최소 하나의 정책 키를 전달하는 첫 번째 소스를 사용하며, 서버 관리 설정을 먼저 확인한 후 엔드포인트 관리 설정을 확인합니다. 단, [다음에서 다루는 예외 키](#per-key-exceptions-across-managed-sources)는 제외됩니다. [Claude Code가 관리 소스를 결합하는 방식](/docs/ko/managed-settings#precedence-within-the-managed-tier)에는 전체 순위, 제어 키에 대한 예외, 모든 소스에 적용되는 옵트인이 있습니다.

선택된 소스가 [`policyHelper`](/docs/ko/settings-reference#policyhelper)가 관리 설정을 제공하는 MDM 정책 또는 관리 설정 파일인 경우, 헬퍼의 출력이 해당 소스를 대체하여 실행을 위한 유일한 관리 구성이 됩니다. Claude Code는 서버 관리 설정이 정책 키를 전달하는 동안 MDM 또는 파일 기반 설정에 구성된 `policyHelper`를 참조하지 않습니다.

나중에 수행한 가져오기에서 서버 관리 설정이 제거된 것을 발견하면, Claude Code는 다음 시작 시가 아니라 즉시 해당 헬퍼를 실행합니다. [`policyHelper`](/docs/ko/settings-reference#policyhelper) 항목은 해당 실행이 실패할 때 발생하는 상황을 다룹니다.

관리 콘솔에서 서버 관리 구성을 지우고 엔드포인트 관리 plist 또는 레지스트리 정책으로 폴백할 의도가 있다면, [캐시된 설정](#fetch-and-caching-behavior)이 다음 성공적인 가져오기까지 클라이언트 머신에 유지되며, `model`과 같이 [다음 시작 시에만 적용되는](#fetch-and-caching-behavior) 키는 각 클라이언트가 다시 시작될 때까지 유효합니다. `/status`를 실행하여 어느 관리 소스가 활성 상태인지 확인하세요.

<h3 id="per-key-exceptions-across-managed-sources">
  관리 소스 전체의 키별 예외
</h3>

세 가지 종류의 키가 병합 금지 규칙의 예외입니다:

* **교차 소스 잠금 키**: 샌드박스 허용 목록 잠금과 같은 작은 키 집합으로, [관리 설정 페이지에 나열되어 있습니다](/docs/ko/managed-settings#precedence-within-the-managed-tier). Claude Code는 관리자 제어 관리 소스가 이들을 설정할 때 이들을 준수합니다. 사용자 쓰기 가능 HKCU 레지스트리 계층은 제외됩니다.

  [`policyHelper`](/docs/ko/settings-reference#policyhelper)가 관리 설정을 제공할 때, 그 출력은 이러한 확인을 읽는 유일한 소스입니다. 단, [`forceRemoteSettingsRefresh`](/docs/ko/settings-reference#forceremotesettingsrefresh)는 Claude Code가 시작 시 관리 소스에서 직접 읽습니다.
* **`env` 블록**: 자격증명 키와 쌍을 이루는 원격 분석 단위 및 라우팅 변수를 제외하고(아래에서 다룸), 관리자 제어 소스 전체에서 키별로 병합됩니다. 각 환경 변수에 대해 이를 정의하는 가장 높은 우선순위 소스가 우승하며, 낮은 관리 소스는 높은 소스가 설정하지 않은 변수를 채웁니다. 따라서 엔드포인트 관리 `env` 항목은 서버 관리 구성이 해당 변수를 설정하지 않을 때마다 적용되거나, 캐시된 서버 값이 [서버 확인 대기 중](#fetch-and-caching-behavior)일 때 적용됩니다. Claude Code v2.1.223 이상이 필요합니다. v2.1.223 이전에는 Claude Code가 선택된 소스의 전체 `env` 블록만 적용합니다.
  * **원격 분석 단위**: `OTEL_EXPORTER_OTLP_*` 내보내기 키, `OTEL_LOG_*` 콘텐츠 캡처 토글, `OTEL_LOGS_EXPORTER`, 그리고 베타 추적 변수 `ENABLE_BETA_TRACING_DETAILED` 및 `BETA_TRACING_ENDPOINT`는 이들 중 하나를 설정하는 가장 높은 소스를 단위로 따릅니다. `otelHeadersHelper` 자격증명 키를 전달하는 소스는 단위를 주장하지만, 선택된 소스일 때만 이러한 변수를 제공합니다. 선택되지 않은 소스가 키를 전달하면 이들 중 어느 것도 제공하지 않으며 여전히 낮은 소스가 이들을 채우는 것을 차단합니다. 어느 쪽이든, 한 소스의 내보내기 엔드포인트는 다른 소스의 자격증명과 쌍을 이룰 수 없습니다.
  * **자격증명 쌍 라우팅**: `apiKeyHelper` 또는 `otelHeadersHelper`와 같은 선택된 소스 전용 자격증명 키와 라우팅 변수를 쌍으로 하는 소스는 해당 슬롯을 획득할 때만 이러한 라우팅 변수를 제공합니다.
* **게이트웨이 로그인 키**: Claude Code는 서버 관리 설정에서 [`forceLoginGatewayUrl`](/docs/ko/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/ko/settings-reference#gatewayinternalnetworks), 또는 [`forceLoginMethod`](/docs/ko/settings-reference#forceloginmethod)의 `"gateway"` 값을 읽지 않으므로, 거기의 값은 MDM 정책 또는 관리 설정 파일에 설정된 값을 적용하지도 숨기지도 않습니다. [`managedSourcesBehavior` 항목](/docs/ko/settings-reference#managedsourcesbehavior)은 머신의 어느 관리 소스가 이들을 제공하는지 나타냅니다.

<h3 id="fetch-and-caching-behavior">
  가져오기 및 캐싱 동작
</h3>

Claude Code는 시작 시 Anthropic의 서버에서 설정을 가져오고 활성 세션 중에 매시간 업데이트를 폴링합니다.

[Claude 앱 게이트웨이](#platform-availability)를 통해 로그인한 클라이언트는 게이트웨이에서 설정을 가져오고 세션이 시작되기 전에 해당 가져오기를 기다리므로, 아래 목록의 가져오기는 이에 적용되지 않습니다. [실패 폐쇄 시작 적용](#enforce-fail-closed-startup)은 해당 가져오기가 실패할 때 발생하는 상황을 다룹니다.

**캐시된 설정 없이 첫 시작:**

* 개발자가 시작 시(예: 첫 실행 또는 `/logout` 후)에 로그인할 때, Claude Code는 세션을 열기 전에 가져오기를 최대 5초 동안 기다립니다. 정책이 시간 내에 도착하면, Claude Code는 첫 화면부터 이를 적용하고 [`companyAnnouncements`](/docs/ko/settings-reference#companyannouncements)를 표시합니다. 페이로드가 [보안 승인](#security-approval-dialogs)이 필요한 경우, Claude Code는 대기를 종료하고 개발자가 승인한 후 페이로드를 적용합니다.
* 다른 모든 시작에서, 그리고 5초 대기가 끝나면, Claude Code는 가져오기가 계속되는 동안 세션을 열므로, 설정이 로드되고 제한이 적용되기 전에 짧은 시간이 경과합니다.
* 가져오기가 실패하면, Claude Code는 서버 관리 설정 없이 계속 진행하고 대화형 세션에서 원격 정책이 적용되지 않음을 경고합니다. 엔드포인트 관리 설정은 여전히 적용됩니다. 관리 소스가 [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup)를 설정하면, Claude Code는 대신 종료됩니다.

**캐시된 설정으로 후속 시작:**

* 캐시된 설정은 캐시된 `modelPricing` 및 `managedMcpServers` 값과 Claude Code가 서버가 페이로드를 확인할 때까지 보류하는 환경 변수를 제외하고 시작 시 즉시 적용됩니다.
* 캐시된 [`modelPricing`](/docs/ko/settings-reference#modelpricing)은 세션의 가져오기가 페이로드를 확인할 때까지 적용되지 않습니다. 그때까지 개발자가 `/usage`에서 보는 비용 수치와 상태 줄은 정가입니다.
* 캐시된 [`managedMcpServers`](/docs/ko/settings-reference#managedmcpservers) 블록은 세션의 가져오기가 페이로드를 확인할 때까지 적용되지 않습니다. Claude Code는 MCP 서버를 연결하기 전에 해당 가져오기를 최대 30초 동안 기다립니다. 가져오기가 실패하거나 시간 초과되면, 세션은 조직의 서버 없이 시작되고, `/status`는 그렇게 표시하며, 나중에 가져오기가 이들을 확인하면 연결됩니다. 첫 시작을 포함한 전체 동작은 [제공된 서버가 연결될 때](/docs/ko/managed-mcp#when-provided-servers-connect)를 참조하세요. Claude Code v2.1.259 이상이 필요합니다.
* Claude Code는 백그라운드에서 새로운 설정을 가져옵니다.
* 캐시된 설정은 네트워크 장애를 통해 유지됩니다. 시작 가져오기가 실패하면, Claude Code는 대화형 세션에서 캐시된 정책이 적용 중임을 경고합니다.
* 가져오기가 성공할 때까지, 시작 시 보류된 값은 보류된 상태로 유지됩니다.

Claude Code는 서버가 세션의 페이로드를 확인할 때까지 캐시된 `env` 블록의 여러 범주의 변수를 보류합니다. 이는 캐시된 프록시, 인증서 기관, 엔드포인트, 또는 자격증명 값이 페이로드를 확인하는 설정 가져오기를 리디렉션, 가로채기, 또는 재인증하는 것을 방지합니다. 강화는 서버에서 가져온 설정 캐시에만 적용됩니다. MDM 또는 `managed-settings.json`을 통해 배포된 [엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms)은 영향을 받지 않습니다. 보류에는 Claude Code v2.1.198 이상이 필요합니다. v2.1.198 이전에는 전체 캐시된 `env` 블록이 시작 시 적용됩니다. 보류된 범주는 다음을 포함합니다:

* `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS`, mTLS 클라이언트 인증서 변수 `CLAUDE_CODE_CLIENT_CERT` 및 `CLAUDE_CODE_CLIENT_KEY`와 같은 프록시 및 TLS 구성
* `ANTHROPIC_BASE_URL`, `CLAUDE_CODE_USE_BEDROCK` 및 `CLAUDE_CODE_USE_VERTEX`와 같은 공급자 선택 변수, `ANTHROPIC_BEDROCK_BASE_URL`과 같은 공급자 엔드포인트 URL을 포함한 API 라우팅 및 공급자 선택
* `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `CLAUDE_CODE_OAUTH_TOKEN`과 같은 인증 자격증명
* 구성 디렉토리 선택기 `CLAUDE_CONFIG_DIR`
* Claude Code v2.1.223 이상에서 자격증명 소스 및 구성 디렉토리 선택기: `ANTHROPIC_FEDERATION_RULE_ID` 및 `ANTHROPIC_IDENTITY_TOKEN`과 같은 Workload Identity Federation 변수, 프로필 및 구성 디렉토리 선택기 `ANTHROPIC_PROFILE` 및 `ANTHROPIC_CONFIG_DIR`, 운영 체제 디렉토리 변수 `HOME`, `XDG_CONFIG_HOME`, `APPDATA`, `USERPROFILE`

Claude Code는 Workload Identity Federation 변수와 `ANTHROPIC_PROFILE` 및 `ANTHROPIC_CONFIG_DIR` 선택기를 시작 시에만 읽으므로, 서버에서 전달된 값은 가져오기가 성공한 후에도 세션의 자격증명 소스를 전환하지 않습니다. Claude Code v2.1.223 이상에서 이러한 선택기를 전달하려면 MDM 또는 `managed-settings.json`과 같은 [엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms)을 사용하세요. `CLAUDE_CONFIG_DIR` 및 운영 체제 디렉토리 변수의 경우, 보류 자체가 보호입니다. 캐시된 값은 서버가 페이로드를 확인할 때까지 환경 밖에 있습니다.

캐시된 `env` 블록의 다른 모든 키는 시작 시 적용됩니다. 서버가 페이로드를 확인한 후, 그리고 [보안 승인](#security-approval-dialogs)이 필요한 경우 승인하면, 보류된 변수는 세션의 나머지 동안 적용됩니다.

조직이 `api.anthropic.com`에 도달하기 위해 프록시가 필요한 경우, 보류는 서버에서 전달된 `env` 블록 자체에만 영향을 미칩니다. MDM 또는 `managed-settings.json`을 통해 [엔드포인트 관리](/docs/ko/managed-settings#delivery-mechanisms) `env` 블록에 설정된 프록시, 셸 환경, 또는 [사용자 설정](/docs/ko/settings#where-settings-live)에 설정된 프록시는 설정 가져오기에 도달합니다. 엔드포인트 관리 소스에는 Claude Code v2.1.223 이상이 필요합니다. 캐시된 서버 관리 프록시 값은 가져오기가 이를 확인할 때까지 보류되므로, 엔드포인트 관리 값은 키별로 채우고 가져오기 자체에 도달합니다. v2.1.223 이전에는 셸 환경 또는 사용자 설정을 사용하여 프록시가 캐시된 서버 페이로드와 함께 적용되도록 하세요. 첫 시작에는 캐시가 없으므로 초기 가져오기를 위해 엔드포인트 관리 소스, 셸 환경, 또는 사용자 설정이 여전히 필요합니다.

Claude Code는 대부분의 설정 업데이트를 재시작 없이 실행 중인 세션에 적용합니다. 일부 업데이트는 다음 시작 시에만 적용되며, OpenTelemetry 내보내기 구성, `model` 키, `env` 블록에서 변수 제거를 포함합니다.

<h3 id="invalid-entries-in-delivered-settings">
  전달된 설정의 잘못된 항목
</h3>

페이로드의 일부가 스키마 검증에 실패하면, Claude Code는 검증 오류를 표시하고 모든 나머지 유효한 설정을 적용합니다. [관리 설정의 잘못된 항목](/docs/ko/managed-settings#invalid-entries-in-managed-settings)은 이것이 무엇을 삭제하고 어느 키가 더 엄격한 값으로 폴백하는지 나타냅니다. Claude Code v2.1.169 이상이 필요합니다.

서버 관리 전달은 이러한 동작을 추가합니다:

* `~/.claude/remote-settings.json`의 캐시는 잘못된 항목이 제거된 구제된 페이로드를 저장합니다. 단, 잘못된 `cleanupPeriodDays` 및 `desktopSessionCleanupPeriodDays` 값은 캐시된 복사본에 남아 있으며 절대 적용되지 않습니다.
* 페이로드의 어떤 필드도 구제될 수 없고 페이로드가 이러한 보존 키만 아닌 경우, Claude Code는 페이로드를 거부하고, 마지막으로 수락된 캐시된 설정을 유지하며, 디버그 로그에 `Remote settings: Settings validation failed - no fields could be salvaged`를 씁니다. `forceRemoteSettingsRefresh`가 설정되면, CLI는 대신 종료됩니다.
* [보안 승인 대화](#security-approval-dialogs)는 구제된 페이로드를 평가하므로, 제거된 잘못된 항목은 승인을 위해 제시되지 않으며 절대 실행되지 않습니다.

전달 문제를 디버그하려면 `claude --debug-file <path>`를 실행하고 로그에서 `Remote settings`를 검색하세요. 조직에 배포하기 전에 테스트 머신에서 `claude doctor`로 페이로드 변경을 검증하세요.

<h3 id="enforce-fail-closed-startup">
  실패 폐쇄 시작 적용
</h3>

기본적으로 시작 시 원격 설정 가져오기가 실패하면, CLI는 마지막 성공적인 가져오기에서 캐시된 설정으로 계속 진행합니다. 단, [가져오기가 성공할 때까지 Claude Code가 보류하는 값](#fetch-and-caching-behavior)은 제외됩니다. 이전에 가져온 적이 없는 머신에서는 CLI가 서버 관리 설정 없이 계속 진행하며 여전히 디바이스의 모든 [엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms)을 적용합니다.

클라이언트가 캐시되거나 부재한 서버 관리 설정에서 시작하는 것을 중지하려면, 관리 설정에서 `forceRemoteSettingsRefresh: true`를 설정하세요.

[Claude 앱 게이트웨이](#platform-availability)를 통해 로그인한 클라이언트는 이 설정을 설정했는지 여부와 관계없이 시작 가져오기를 기다리며, 실패한 가져오기를 다음과 같이 처리합니다:

* 게이트웨이가 참석한 대화형 시작에 `401`로 응답하고 이 설정이 꺼져 있으면, 게이트웨이는 해당 로그인을 종료했습니다. Claude Code는 [`Cloud gateway session expired — run /login to reconnect.`](/docs/ko/errors#cloud-gateway-session-expired)를 인쇄하고 사용자가 `/login`을 실행할 때까지 게이트웨이에서 로그아웃된 상태로 세션을 엽니다.
* 가져오기가 다른 방식으로 실패하거나, `claude auth` 부명령을 제외한 다른 모든 종류의 시작에서 실패하면, 클라이언트는 오류와 함께 종료됩니다.

이 설정이 서버 관리 설정을 가져오는 세션에서 활성 상태일 때, CLI는 원격 설정이 새로 가져올 때까지 시작 시 차단됩니다. 가져오기가 실패하면, CLI는 정책 없이 진행하지 않고 종료됩니다. 이 설정은 자체 영속화됩니다. 서버에서 전달되면, 첫 번째 성공적인 새 세션 가져오기 전에도 후속 시작이 동일한 동작을 적용하도록 로컬로 캐시됩니다. [서버 관리 설정을 가져오지 않는](#platform-availability) 세션은 대기 없이 시작됩니다.

이를 활성화하려면 관리 설정 구성에 키를 추가하세요:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

[엔드포인트 관리](/docs/ko/managed-settings#delivery-mechanisms) MDM 프로필 또는 시스템 `managed-settings.json` 파일에서 이 키를 설정하여 첫 시작 시 실패 폐쇄 동작을 적용할 수도 있습니다. 이 플래그는 위의 [우선순위 규칙](#settings-precedence)의 예외입니다. Claude Code는 관리자 제어 관리 소스가 이를 설정할 때 이를 준수합니다. 캐시된 서버 관리 페이로드도 있으면 MDM 전달 값을 무시하지 않습니다.

[`policyHelper`](/docs/ko/settings-reference#policyhelper)가 관리 설정을 제공할 때, 그 출력은 Claude Code가 시작 후 읽는 키에 대해 다른 모든 관리 소스를 대체합니다. Claude Code가 이 키를 읽는 소스에 대해서는 [그 설정 항목](/docs/ko/settings-reference#forceremotesettingsrefresh)을 참조하세요. `policyHelper` 항목은 Claude Code가 헬퍼를 읽는 소스와 실행 시기를 나타냅니다.

설정 가져오기는 또한 중간 HTTP 프록시가 오래된 응답을 제공하지 않도록 `Cache-Control: no-cache` 헤더를 보냅니다.

이 설정을 활성화하기 전에, 네트워크 정책이 `api.anthropic.com`에 대한 연결을 허용하는지 확인하세요. 해당 엔드포인트에 도달할 수 없으면, CLI는 시작 시 종료되고 사용자는 Claude Code를 시작할 수 없습니다.

`claude auth login`과 같은 `claude auth` 부명령은 이 확인과 게이트웨이 시작 종료에서 제외되므로, 사용자는 만료된 자격증명이 설정 가져오기 실패의 원인일 때 재인증할 수 있습니다.

<h3 id="security-approval-dialogs">
  보안 승인 대화
</h3>

특정 설정은 보안 위험을 초래할 수 있으므로 Claude Code가 대화형 세션에서 이를 적용하기 전에 명시적인 사용자 승인이 필요합니다:

* **셸 명령 설정**: `apiKeyHelper`, `statusLine`, `otelHeadersHelper`와 같이 셸 명령을 실행하는 설정
* **샌드박스 바이너리 설정**: `sandbox.bwrapPath`, `sandbox.socatPath`, `sandbox.ripgrep`. 이러한 각 설정은 실행 파일을 가리키며, Claude Code는 해당 실행 파일을 실행합니다.
* **샌드박스 네트워크 및 격리 설정**: [샌드박스](/docs/ko/sandboxing) 설정으로 샌드박스 프록시가 트래픽을 읽고, 재라우팅하거나, 인증하거나, 샌드박스의 격리를 약화시킬 수 있습니다: `sandbox.network.tlsTerminate`, `sandbox.network.httpProxyPort`, `sandbox.network.socksProxyPort`, `sandbox.credentials`, `sandbox.allowAppleEvents`, `sandbox.enableWeakerNestedSandbox`, `sandbox.enableWeakerNetworkIsolation`, `sandbox.filesystem.disabled`, `sandbox.network.allowAllUnixSockets`, `sandbox.network.allowUnixSockets`, `sandbox.network.allowMachLookup`. `deny` 규칙만 포함하는 `sandbox.credentials` 블록은 프록시에 자격증명을 제공하지 않으므로 승인이 필요하지 않습니다. v2.1.251 이전에는 Claude Code가 이러한 설정을 승인 없이 적용했습니다.
* **사용자 정의 환경 변수**: 프록시 및 기본 URL 변수와 같이 사용자의 승인이 필요한 전달된 `env` 변수. [환경 변수 및 승인 대화](#environment-variables-and-the-approval-dialog)를 참조하세요.
* **훅 구성**: 모든 훅 정의

이러한 설정이 있으면, 사용자는 구성 중인 내용을 설명하는 보안 대화를 봅니다. 사용자는 진행하려면 승인해야 합니다. 사용자가 설정을 거부하면, Claude Code는 종료됩니다.

[`claudeMd`](/docs/ko/settings-reference#claudemd) 키를 통해 전달된 관리 CLAUDE.md는 Claude가 실행하는 명령이 아니라 Claude에 대한 지시 텍스트이므로 승인이 필요하지 않습니다. Claude Code는 여전히 이러한 지시를 따르는 동안 Claude가 사용하는 도구에 대한 [권한](/docs/ko/permissions)을 확인합니다. v2.1.260 이전에는 `claudeMd` 값이 승인을 요구했습니다.

<h4 id="approval-memory">
  승인 메모리
</h4>

Claude Code는 구성 디렉토리 `~/.claude`(또는 [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars)을 설정한 경우)에 승인을 기록합니다. 기록하는 내용은 설정 가져오기가 사용하는 자격증명에 따라 다릅니다:

* **`/login` 또는 `claude auth login`으로 저장된 claude.ai 로그인, 또는 [API 키 없는 콘솔 로그인](/docs/ko/authentication#sign-in-without-an-api-key)**: 조직당 하나의 승인으로, 가장 최근에 승인한 계정이 보유합니다.
* **[Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 로그인**: 게이트웨이당 하나의 승인입니다.

  같은 게이트웨이에서 로그아웃했다가 다시 로그인하면, 승인이 필요한 설정이 변경되지 않는 한 Claude Code는 대화를 다시 표시하지 않습니다. Claude Code는 이러한 설정이 변경될 때, 다른 게이트웨이에 로그인할 때, 같은 게이트웨이에 대해 새 인증서를 수락할 때 다시 표시합니다.

  Claude Code는 일반 HTTP를 통해 도달한 루프백 개발 게이트웨이에 대해 승인을 저장하지 않으므로, 각 로그인 후 대화가 다시 나타납니다.
* **API 키 또는 `CLAUDE_CODE_OAUTH_TOKEN`과 같은 다른 자격증명**: 전달된 설정에 대한 하나의 승인으로, 해당 구성 디렉토리의 설정 캐시된 복사본과 함께 유지됩니다. Claude Code는 승인이 필요한 설정이 변경될 때, 그리고 `/logout` 또는 `claude auth logout`을 실행한 후(둘 다 캐시된 복사본을 삭제함) 대화를 다시 표시합니다.

`sandbox.credentials` 또는 `sandbox.network.tlsTerminate`에 대한 승인은 동일한 전달된 설정의 [`sandbox.network.allowedDomains`](/docs/ko/settings-reference#sandbox-network-alloweddomains) 항목도 다룹니다. 두 설정 모두 해당 허용 목록에 작용하기 때문입니다. 대화는 관리자가 이러한 항목 중 하나를 추가하거나 제거할 때 다시 나타나며, `sandbox.network.allowedDomains`는 자체적으로 승인이 필요하지 않습니다.

저장된 claude.ai 로그인을 사용하면:

* 로그아웃했다가 다시 로그인하거나, 다른 조직으로 전환했다가 나중에 돌아오면, 이러한 설정이 변경되지 않는 한 Claude Code는 대화를 다시 표시하지 않습니다. 단, 그 사이에 다른 계정이 같은 구성 디렉토리의 해당 조직에 대해 이들을 승인한 경우는 제외됩니다.
* 다른 계정으로 같은 조직에 로그인하면, 설정이 변경되지 않았더라도 Claude Code는 대화를 다시 표시합니다. 해당 계정의 승인이 이전 승인을 대체하므로, 다시 전환하면 Claude Code는 한 번 더 대화를 표시합니다.

Claude Code는 항상 대화를 표시할 수 없습니다. 아래의 각 경우는 어느 설정이 적용되는지와 다음에 대화를 볼 때를 나타냅니다:

* **대화를 표시할 수 없는 대화형 세션**: Claude Code는 전달된 설정을 적용하지 않고 마지막으로 승인된 설정을 유지합니다. 대화는 대화를 표시할 수 있는 다음 세션에 나타납니다. Claude Code v2.1.211 이상이 필요합니다.
* **`claude install` 또는 `claude update`**: Claude Code는 두 명령 중 어느 것 중에도 대화를 표시하지 않습니다. 명령은 마지막으로 승인된 설정으로 실행되고, 대화는 다음 대화형 세션에 나타납니다. Claude Code가 [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup) 설정 또는 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 배포와 같이 시작 시 설정 가져오기를 기다리면, 대신 명령 중에 대화를 표시하며, 파이프에서의 설치 실행이 실패합니다. [설치 중 `Raw mode is not supported`](/docs/ko/troubleshoot-install#raw-mode-is-not-supported-during-install)를 참조하세요. v2.1.246 이전에는 Claude Code가 이러한 명령 중에도 대화를 표시하려고 했습니다.
* **오류가 응답 전에 대화를 닫음**: Claude Code는 전달된 설정을 적용하지 않고 마지막으로 승인된 설정을 유지합니다. 대화를 표시할 수 있는 다음 세션에 다시 표시합니다.
* **`claude -p` 또는 Agent SDK 세션과 같은 비대화형 실행**: Claude Code는 대화를 표시할 수 없으므로, 전달된 설정이 승인을 요구할 때, 해당 실행에만 이를 적용합니다. 이를 승인된 것으로 기록하거나 [로컬 캐시](#fetch-and-caching-behavior)에 쓰지 않으며, 다음 대화형 세션은 대화를 표시합니다. 사용자가 대화형 세션에서 승인할 때까지, 각 비대화형 실행은 시작 시 설정을 다시 가져옵니다. v2.1.207 이전에는 비대화형 실행이 설정을 승인된 것으로 저장했으므로, 나중의 대화형 세션은 이들에 대해 대화를 표시하지 않았습니다.

<h4 id="environment-variables-and-the-approval-dialog">
  환경 변수 및 승인 대화
</h4>

Claude Code는 다음을 포함하여 사용자 승인 대화를 표시하지 않고 일부 전달된 `env` 변수를 적용합니다:

* 기능 및 명령 토글
* `ANTHROPIC_MODEL`, `DISABLE_PROMPT_CACHING`, `CLAUDE_CODE_EFFORT_LEVEL`과 같은 모델 선택 및 동작 설정
* `DISABLE_AUTO_COMPACT`와 같은 컨텍스트 윈도우 및 압축 설정
* 터미널 UI 및 접근성 옵션
* 숫자 제한, 예산, 시간 초과

다른 전달된 변수는 적용되기 전에 사용자의 승인을 요구할 수 있습니다. 비어 있지 않은 프록시, 기본 URL, 또는 `OTEL_EXPORTER_OTLP_ENDPOINT` 값은 항상 그렇습니다. 전달된 변수가 승인을 필요로 할 때, 대화는 이를 이름으로 지정하므로, 사용자는 정책이 설정하도록 요청하는 것을 정확히 봅니다. v2.1.218 이전에는 Claude Code가 더 적은 변수를 사용자에게 묻지 않고 적용했으므로, `DISABLE_AUTO_COMPACT`와 같은 설정이 비어 있지 않은 값에서 대화를 트리거했습니다.

Claude Code는 변수 이름이 아니라 전달된 값에 따라 네 가지 개인정보 보호 토글이 승인을 필요로 하는지 결정합니다: `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, `DISABLE_ERROR_REPORTING`, `DISABLE_TELEMETRY`, `DO_NOT_TRACK`. `1` 또는 `true`와 같은 참 값은 추적, 보고 또는 기타 비필수 트래픽만 끄므로, Claude Code는 사용자에게 묻지 않고 이를 적용합니다. 다른 비어 있지 않은 값의 경우, Claude Code는 대화를 표시합니다. v2.1.218 이전에는 `DO_NOT_TRACK`을 제외한 모든 것이 비어 있지 않은 값에서 승인 없이 적용되었으며, `DO_NOT_TRACK`은 비어 있지 않은 값에서 대화를 트리거했습니다.

Claude Code는 또한 전달된 값에 따라 [`API_FORCE_IDLE_TIMEOUT`](/docs/ko/env-vars)이 승인을 필요로 하는지 결정합니다. 참 값은 [본문 유휴 시간 초과](/docs/ko/network-config#streaming-idle-watchdogs)만 켜므로, Claude Code는 사용자에게 묻지 않고 이를 적용합니다. 다른 비어 있지 않은 값의 경우, Claude Code는 대화를 표시합니다. v2.1.248 이전에는 비어 있지 않은 값이 대화를 트리거했습니다.

[`ANTHROPIC_CUSTOM_HEADERS`](/docs/ko/env-vars#variables)가 승인을 필요로 하는지 여부도 전달된 값에 따라 다릅니다. `Accept-Language`와 같이 요청에만 태그를 지정하는 헤더는 대화 없이 적용됩니다. 자격증명, 조직 또는 테넌트 선택기, 라우팅 또는 호스트 재정의, `Authorization`, `X-Api-Key`, `Host`, `anthropic-beta`, `X-Amzn-Bedrock-*` 헤더와 같은 API 동작 헤더의 이름을 지정하는 라인은 승인을 요구합니다. 라인의 이름이 유효한 HTTP 헤더 토큰이 아니거나 그 값이 HTTP 헤더가 전달할 수 없는 문자를 포함할 때도 승인이 필요합니다. 확인은 헤더 이름 내의 단어와 일치하므로, `client`와 `version`을 포함하는 `X-Client-Version`도 승인을 요구합니다. v2.1.251 이전에는 모든 `ANTHROPIC_CUSTOM_HEADERS` 값이 승인 없이 적용되었습니다.

[`ENABLE_BETA_TRACING_DETAILED`](/docs/ko/env-vars#variables) 또는 [`OTEL_LOG_RAW_API_BODIES`](/docs/ko/env-vars#variables)에 대한 `0` 또는 `false`와 같은 거짓 값은 상세 추적 또는 원본 API 본문 캡처만 끄므로 대화 없이 적용됩니다. 두 변수 모두에 대한 다른 비어 있지 않은 값은 승인을 요구합니다.

<h2 id="platform-availability">
  플랫폼 가용성
</h2>

서버 관리 설정은 `api.anthropic.com`에 대한 직접 연결이 필요합니다. 전달을 위해서는 세션이 다음 자격 증명 중 하나로 인증되어야 합니다:

* Team 또는 Enterprise OAuth 로그인
* `CLAUDE_CODE_OAUTH_TOKEN`을 통해 제공되는 OAuth 토큰
* 직접 구성된 API 키
* Anthropic 프로필의 `user_oauth` [Anthropic 프로필](/docs/ko/authentication#anthropic-profiles-and-federation-credentials), 프로필이 Anthropic API 이외의 `base_url`을 설정하지 않는 한. Claude Code v2.1.257 이상이 필요합니다.

[`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 스크립트에서 반환된 키도 [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) 자격 증명도 설정 가져오기를 트리거하지 않습니다.

Claude Desktop 앱의 [Cowork](https://claude.com/docs/cowork/overview) 세션에서, Claude Code는 사용자가 Team 또는 Enterprise 계정으로 로그인할 때에도 claude.ai 관리 콘솔에서 서버 관리 설정을 가져오지 않습니다. [정책이 적용되는 위치와 시기](/docs/ko/managed-settings#where-and-when-a-policy-applies)는 어떤 정책이 사용자 머신의 Cowork 세션과 원격 Cowork 세션에 도달하는지 다룹니다. claude.ai는 Cowork 사용자가 claude.ai의 git 저장소에서 또는 Cowork 탭의 **사용자 정의**에서 마켓플레이스를 추가할 때 여전히 [`strictKnownMarketplaces`](/docs/ko/settings-reference#strictknownmarketplaces) 및 [`blockedMarketplaces`](/docs/ko/settings-reference#blockedmarketplaces) 목록을 적용합니다. [제한 사항이 작동하는 방식](/docs/ko/plugins/org#restrict-what-users-can-install)은 해당 확인을 설명합니다.

셸에서 `CLAUDE_CODE_USE_*` 공급자 변수 또는 기본값이 아닌 `ANTHROPIC_BASE_URL`을 내보내면, Claude Code는 세션에 대한 설정 가져오기를 건너뜁니다. [`claude doctor` 및 `/status`는 건너뛴 가져오기와 그 원인을 보고합니다](#verify-settings-delivery).

서버 관리 `env` 블록으로 내보내기를 지울 수 없습니다. 왜냐하면 블록은 내보내기가 방지하는 가져오기를 통해 도착하기 때문입니다. [엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms) `env` 블록도 가져오기를 복원하지 않습니다. Claude Code는 관리 `env` 블록을 적용하기 전에 적격성을 확인하므로, 엔드포인트 관리 값은 세션의 공급자 선택을 변경하지만 가져오기는 건너뛴 상태로 유지됩니다.

서버 관리 전달을 복원하려면, 셸에서 내보내기를 제거하거나, 사용자 설정 `env` 블록에서 변수를 `""`로 설정합니다. 이는 적격성 확인 전에 적용됩니다. 사용자가 셸을 변경하도록 의존하지 않고 정책을 적용하려면, 대신 엔드포인트 관리 채널을 통해 설정을 전달합니다.

Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 및 [Claude Platform on AWS](/docs/ko/claude-platform-on-aws) 배포의 경우, 자체 호스팅 [Claude apps gateway](/docs/ko/claude-apps-gateway)는 동등한 원격 관리 설정 전달을 제공합니다. 게이트웨이에 로그인한 클라이언트는 `api.anthropic.com` 대신 게이트웨이에서 관리 설정을 가져옵니다. 시작 시 실패 의미론이 다릅니다. 게이트웨이에 도달할 수 없는 게이트웨이 클라이언트는 캐시된 설정으로 폴백하는 대신 오류로 종료되지만, 시간별 백그라운드 새로 고침은 두 채널 모두에서 실패 개방입니다.

<h2 id="audit-logging">
  감사 로깅
</h2>

설정 변경에 대한 감사 로그 이벤트는 규정 준수 API 또는 감사 로그 내보내기를 통해 사용할 수 있습니다. 액세스를 위해 Anthropic 계정 팀에 문의합니다.

감사 이벤트는 수행된 작업의 유형, 작업을 수행한 계정 및 기기, 이전 값과 새 값에 대한 참조를 포함합니다.

<h2 id="security-considerations">
  보안 고려사항
</h2>

서버 관리 설정은 중앙 집중식 정책 적용을 제공하지만 클라이언트 측 제어로 작동하며 보안 경계가 아닙니다. 관리되지 않는 기기에서 사용자는 이를 우회하기 위해 관리자 또는 sudo 액세스 권한이 필요하지 않습니다.

| 시나리오                                          | 동작                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :-------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 사용자가 캐시된 설정 파일을 편집함                           | 변조된 파일이 시작 시 적용되지만, [Claude Code가 보류하는 값](#fetch-and-caching-behavior)은 서버가 페이로드를 확인할 때까지 제외됩니다. 다음 서버 가져오기에서 올바른 설정이 복원되지만, [다음 시작 시에만 적용되는 키](#fetch-and-caching-behavior)(예: `model` 또는 `env` 블록에 추가된 변수)는 재시작할 때까지 적용 상태로 유지됩니다.                                                                                                                                                                                                                          |
| 사용자가 캐시된 설정 파일을 삭제함                           | [첫 시작 동작](#fetch-and-caching-behavior)이 발생합니다.                                                                                                                                                                                                                                                                                                                                                                                                                |
| 사용자가 수정된 Claude Code 바이너리를 실행함                | 수정된 클라이언트를 실행할 수 있는 사용자는 모든 클라이언트 측 제어를 우회할 수 있습니다.                                                                                                                                                                                                                                                                                                                                                                                                           |
| 사용자가 이전 Claude Code 버전을 실행함                   | 서버 관리 설정 이전의 버전은 이를 가져오거나 적용하지 않습니다.                                                                                                                                                                                                                                                                                                                                                                                                                          |
| API를 사용할 수 없음                                 | 캐시된 설정이 있으면 적용되지만, [가져오기가 성공할 때까지 Claude Code가 보류하는 값](#fetch-and-caching-behavior)은 제외됩니다. 캐시가 없으면 Claude Code는 다음 성공적인 가져오기까지 서버 관리 설정을 적용하지 않으며 기기의 [엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms)은 여전히 적용합니다. `forceRemoteSettingsRefresh: true`를 사용하면 CLI는 계속하는 대신 종료되지만, [`claude auth` 부분 명령](#enforce-fail-closed-startup)은 제외됩니다. [Claude 앱 게이트웨이](#platform-availability)를 통해 로그인한 클라이언트는 해당 설정 없이 시작 시 종료되며, 동일한 `claude auth` 예외가 적용됩니다. |
| 사용자가 다른 조직으로 인증함                              | 관리 조직 외부의 계정에 대해 설정이 전달되지 않습니다.                                                                                                                                                                                                                                                                                                                                                                                                                               |
| 사용자가 [타사 모델 공급자](#platform-availability)를 구성함 | 서버 관리 설정이 우회됩니다. 여기에는 `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS` 설정 또는 기본이 아닌 `ANTHROPIC_BASE_URL` 설정이 포함됩니다.                                                                                                                                                                                                                                                  |
| 네트워크 트래픽이 가로채지거나 리디렉션됨                        | 비활성화된 TLS 검증 또는 가로챈 트래픽은 클라이언트가 수신하는 설정을 변경할 수 있습니다.                                                                                                                                                                                                                                                                                                                                                                                                          |

로컬 설정 파일(예: `managed-settings.json`)에 대한 편집을 기록하려면 [`ConfigChange` hooks](/docs/ko/hooks#configchange)를 사용합니다. Claude Code는 서버 관리 설정이 도착하거나 새로 고쳐질 때, 또는 MDM 프로필이나 레지스트리 정책이 변경될 때 이를 실행하지 않으며, 훅은 `policy_settings` 변경을 차단할 수 없습니다.

클라이언트가 제공하는 자격 증명으로 사용자가 액세스할 수 있는 조직을 제한하려면 Claude 도움말 센터의 [테넌트 제한으로 네트워크 수준 액세스 제어 적용](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions)을 참조하십시오. 더 강력한 적용 보장을 위해 MDM 솔루션에 등록된 기기에서 [엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms)을 사용합니다.

<h2 id="see-also">
  참고 항목
</h2>

Claude Code 구성 관리를 위한 관련 페이지:

* [모든 설정](/docs/ko/settings-reference): 모든 설정 키
* [엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms): IT에서 기기에 배포하는 관리 설정
* [인증](/docs/ko/authentication): Claude Code에 대한 사용자 액세스 설정
* [보안](/docs/ko/security): 보안 보호 및 모범 사례
