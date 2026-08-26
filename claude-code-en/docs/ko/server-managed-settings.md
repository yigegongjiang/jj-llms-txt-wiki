> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 서버 관리 설정 구성

> 기기 관리 인프라 없이 Claude.ai의 웹 기반 인터페이스를 통해 조직을 위해 Claude Code를 중앙에서 구성합니다.

서버 관리 설정을 통해 조직 소유자는 claude.ai 콘솔의 [**관리자 설정 > Claude Code > 관리 설정**](https://claude.ai/admin-settings/claude-code)에서 Claude Code를 중앙에서 구성할 수 있습니다. Claude Code 클라이언트는 사용자가 조직 OAuth 로그인 또는 직접 구성된 API 키로 인증할 때 이러한 설정을 자동으로 가져오며, 서버 관리 전달이 지원되는 플랫폼에서 이를 받습니다. [플랫폼 가용성](#platform-availability)을 참조하십시오.

이 방식은 기기 관리 인프라가 없거나 관리되지 않는 기기의 사용자를 위해 설정을 관리해야 하는 조직을 위해 설계되었습니다.

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

Claude Code는 중앙 집중식 구성을 위한 두 가지 방식을 지원합니다. 서버 관리 설정은 Anthropic의 서버에서 구성을 전달합니다. [엔드포인트 관리 설정](/docs/ko/settings#settings-files)은 기본 OS 정책(macOS 관리 기본 설정, Windows 레지스트리) 또는 관리 설정 파일을 통해 기기에 직접 배포됩니다.

| 방식                                             | 최적 대상                         | 보안 모델                                            |
| :--------------------------------------------- | :---------------------------- | :----------------------------------------------- |
| **서버 관리 설정**                                   | MDM이 없는 조직 또는 관리되지 않는 기기의 사용자 | 인증 시 Anthropic의 서버에서 전달되는 설정                     |
| **[엔드포인트 관리 설정](/docs/ko/settings#settings-files)** | MDM 또는 엔드포인트 관리가 있는 조직        | MDM 구성 프로필, 레지스트리 정책 또는 관리 설정 파일을 통해 기기에 배포되는 설정 |

기기가 MDM 또는 엔드포인트 관리 솔루션에 등록된 경우, 엔드포인트 관리 설정은 설정 파일을 OS 수준에서 사용자 수정으로부터 보호할 수 있으므로 더 강력한 보안 보장을 제공합니다. 엔드포인트 관리 설정은 [클라우드 세션](/docs/ko/model-config#surface-coverage)에 도달하지 않으므로, 웹에서 Claude Code를 사용하는 조직은 서버 관리 설정도 함께 구성해야 합니다.

<h2 id="configure-server-managed-settings">
  서버 관리 설정 구성
</h2>

<Steps>
  <Step title="관리 콘솔 열기">
    claude.ai 콘솔에서 [**관리 설정 > Claude Code > 관리 설정**](https://claude.ai/admin-settings/claude-code)으로 이동합니다.

    링크가 Claude Code 페이지 대신 다른 관리 설정 페이지로 리디렉션되면 계정에 필요한 역할이 없습니다. 관리자 및 기타 비 소유자 역할은 관리 설정을 보거나 편집할 수 없으므로 조직의 소유자 또는 주 소유자에게 변경을 요청하십시오. [액세스 제어](#access-control)를 참조하십시오.
  </Step>

  <Step title="설정 정의">
    구성을 JSON으로 추가합니다. [`settings.json`에서 사용 가능한 모든 설정](/docs/ko/settings#available-settings)이 지원되며, OS 수준 정책 전달로 제한된 설정을 제외하고는 모두 지원됩니다. [현재 제한사항](#current-limitations)에서 해당 짧은 목록을 참조하십시오. 여기에는 [hooks](/docs/ko/hooks), [환경 변수](/docs/ko/env-vars), 및 `allowManagedPermissionRulesOnly`와 같은 [관리 전용 설정](/docs/ko/permissions#managed-only-settings)이 포함됩니다.

    이 예제는 권한 거부 목록을 적용하고, 사용자가 권한을 우회하는 것을 방지하며, 권한 규칙을 관리 설정에 정의된 규칙으로만 제한합니다.

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

    이 예제는 조직 전체에서 모든 파일 편집 후 감사 스크립트를 실행합니다.

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

    [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) 분류기를 구성하여 조직이 신뢰하는 저장소, 버킷 및 도메인을 알도록 하려면:

    ```json theme={null}
    {
      "autoMode": {
        "environment": [
          "Source control: github.example.com/acme-corp and all repos under it",
          "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
          "Trusted internal domains: *.corp.example.com"
        ]
      }
    }
    ```

    Hook은 셸 명령을 실행하므로 사용자는 적용되기 전에 [보안 승인 대화](#security-approval-dialogs)를 봅니다. `autoMode` 항목이 분류기가 차단하는 것에 어떻게 영향을 미치는지, 그리고 `environment`, `allow`, `soft_deny`, 및 `hard_deny` 필드에 대한 중요한 경고는 [자동 모드 구성](/docs/ko/auto-mode-config)을 참조하십시오.
  </Step>

  <Step title="저장 및 배포">
    변경 사항을 저장합니다. Claude Code 클라이언트는 다음 시작 또는 시간별 폴링 주기에 업데이트된 설정을 수신합니다.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  설정 전달 확인
</h3>

설정이 적용되고 있는지 확인하려면 사용자에게 Claude Code를 다시 시작하도록 요청합니다. 구성에 [보안 승인 대화](#security-approval-dialogs)를 트리거하는 설정이 포함된 경우, 사용자는 시작 시 관리 설정을 설명하는 프롬프트를 봅니다. 사용자가 `/permissions`를 실행하여 유효한 권한 규칙을 확인하도록 하여 관리 권한 규칙이 활성화되어 있는지 확인할 수도 있습니다.

<h3 id="access-control">
  액세스 제어
</h3>

다음 역할이 서버 관리 설정을 관리할 수 있습니다.

* **주 소유자**
* **소유자**

설정 변경이 조직의 모든 사용자에게 적용되므로 신뢰할 수 있는 담당자에게만 액세스를 제한합니다.

<h3 id="managed-only-settings">
  관리 전용 설정
</h3>

대부분의 [설정 키](/docs/ko/settings#available-settings)는 모든 범위에서 작동합니다. 소수의 키는 관리 설정에서만 읽혀지며 사용자 또는 프로젝트 설정 파일에 배치될 때 효과가 없습니다. 전체 목록은 [관리 전용 설정](/docs/ko/permissions#managed-only-settings)을 참조하십시오. 해당 목록에 없는 모든 설정은 여전히 관리 설정에 배치될 수 있으며 최고 우선순위를 갖습니다.

<h3 id="current-limitations">
  현재 제한사항
</h3>

서버 관리 설정은 다음과 같은 제한사항이 있습니다.

* 설정은 조직의 모든 사용자에게 균일하게 적용됩니다. 그룹별 구성은 아직 지원되지 않습니다.
* [`managed-mcp.json`](/docs/ko/managed-mcp) 파일은 서버 관리 설정을 통해 배포할 수 없습니다. 대신 `allowedMcpServers` 및 `deniedMcpServers` 정책 키를 배포하십시오.
* OS 수준 정책 소스로 제한된 설정(예: `policyHelper` 및 `wslInheritsWindowsSettings`)은 적용되지 않습니다. 대신 MDM 또는 시스템 `managed-settings.json` 파일을 통해 배포하십시오.

<h2 id="settings-delivery">
  설정 전달
</h2>

<h3 id="settings-precedence">
  설정 우선순위
</h3>

서버 관리 설정과 [엔드포인트 관리 설정](/docs/ko/settings#settings-files)은 모두 Claude Code [설정 계층](/docs/ko/settings#settings-precedence)의 최상위 계층을 차지합니다. 명령줄 인수를 포함한 다른 설정 수준은 이를 재정의할 수 없습니다.

관리 계층 내에서 구성된 [`policyHelper`](/docs/ko/settings#compute-managed-settings-with-a-policy-helper)는 서버 관리 설정을 포함한 다른 모든 관리 소스보다 우선합니다. 이 도구의 출력은 실행을 위한 유일한 관리 구성이 됩니다. 그렇지 않으면 비어있지 않은 구성을 전달하는 첫 번째 소스가 우선합니다. 서버 관리 설정이 먼저 확인되고, 그 다음 엔드포인트 관리 설정이 확인됩니다. 소스는 병합되지 않습니다. 서버 관리 설정이 어떤 키든 전달하면 다른 엔드포인트 관리 설정은 완전히 무시됩니다. 한 가지 예외가 적용됩니다. 샌드박스 허용 목록 잠금과 같은 [교차 소스 잠금 키](/docs/ko/settings#settings-precedence)의 작은 집합은 모든 관리자 제어 관리 소스가 이를 설정할 때 준수됩니다. 사용자 쓰기 가능 HKCU 레지스트리 계층은 제외됩니다. 서버 관리 설정이 아무것도 전달하지 않으면 엔드포인트 관리 설정이 적용됩니다.

엔드포인트 관리 plist 또는 레지스트리 정책으로 돌아가려는 의도로 관리 콘솔에서 서버 관리 구성을 지우는 경우, [캐시된 설정](#fetch-and-caching-behavior)이 다음 성공적인 가져오기까지 클라이언트 머신에 유지된다는 점을 주의하십시오. `/status`를 실행하여 어느 관리 소스가 활성화되어 있는지 확인합니다.

<h3 id="fetch-and-caching-behavior">
  가져오기 및 캐싱 동작
</h3>

Claude Code는 시작 시 Anthropic의 서버에서 설정을 가져오고 활성 세션 중에 시간별로 업데이트를 폴링합니다.

**캐시된 설정 없이 처음 시작:**

* Claude Code는 비동기적으로 설정을 가져옵니다.
* 가져오기가 실패하면 Claude Code는 관리 설정 없이 계속됩니다.
* 설정이 로드되기 전에 제한이 아직 적용되지 않는 짧은 시간이 있습니다.

**캐시된 설정으로 이후 시작:**

* 캐시된 설정은 시작 시 즉시 적용됩니다. 아래에 설명된 전송, 라우팅 및 인증 환경 변수는 제외됩니다.
* Claude Code는 백그라운드에서 새로운 설정을 가져옵니다.
* 캐시된 설정은 네트워크 장애를 통해 유지됩니다. 보류된 환경 변수는 가져오기가 성공할 때까지 보류된 상태로 유지됩니다.

v2.1.198 이상에서 Claude Code는 서버가 세션의 페이로드를 확인할 때까지 캐시된 `env` 블록의 세 가지 환경 변수 범주를 보류합니다. 이는 캐시된 프록시, 인증서 기관, 엔드포인트 또는 자격 증명 값이 설정 가져오기를 리디렉션, 가로채기 또는 다시 인증하는 것을 방지합니다. 강화는 서버에서 가져온 설정 캐시에만 적용됩니다. [엔드포인트 관리 설정](/docs/ko/settings#settings-files)은 MDM 또는 `managed-settings.json`을 통해 배포되며 영향을 받지 않습니다. 보류된 범주는 다음과 같습니다.

* `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS` 및 mTLS 클라이언트 인증서 변수 `CLAUDE_CODE_CLIENT_CERT`와 `CLAUDE_CODE_CLIENT_KEY`와 같은 프록시 및 TLS 구성
* `ANTHROPIC_BASE_URL`, `CLAUDE_CODE_USE_BEDROCK` 및 `CLAUDE_CODE_USE_VERTEX`와 같은 공급자 선택 변수, 그리고 `ANTHROPIC_BEDROCK_BASE_URL`과 같은 공급자 엔드포인트 URL을 포함한 API 라우팅 및 공급자 선택
* `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` 및 `CLAUDE_CODE_OAUTH_TOKEN`과 같은 인증 자격 증명

캐시된 `env` 블록의 다른 모든 키(예: 원격 분석 및 OpenTelemetry 구성)는 이전과 같이 시작 시 적용됩니다. 가져오기가 성공하면 보류된 변수는 세션의 나머지 기간 동안 적용됩니다.

조직에서 `api.anthropic.com`에 도달하기 위해 프록시가 필요한 경우, 관리 `env` 블록에만 설정하지 말고 셸 환경 또는 [사용자 설정](/docs/ko/settings#settings-files)에서 설정합니다. 첫 번째 시작에는 캐시가 없으므로 이러한 소스는 이미 초기 가져오기에 필요했습니다.

Claude Code는 OpenTelemetry 구성과 같은 고급 설정을 제외하고 재시작 없이 설정 업데이트를 자동으로 적용하며, 이는 적용되려면 전체 재시작이 필요합니다.

<h3 id="invalid-entries-in-delivered-settings">
  전달된 설정의 잘못된 항목
</h3>

전달된 페이로드는 다른 관리 소스와 동일한 규칙으로 관대하게 구문 분석됩니다. 페이로드에 스키마 검증에 실패하는 항목이 포함되어 있으면 Claude Code는 해당 항목을 제거하고 검증 오류를 표시하며 남은 모든 유효한 설정을 적용합니다. 보안 적용 필드 처리 방법을 포함한 필드 수준 동작은 [관리 설정의 잘못된 항목](/docs/ko/settings#invalid-entries-in-managed-settings)을 참조하십시오. Claude Code v2.1.169 이상이 필요합니다.

서버 관리 전달은 다음 동작을 추가합니다.

* `~/.claude/remote-settings.json`의 캐시는 잘못된 항목이 제거된 구제된 페이로드를 저장합니다. 원본 잘못된 페이로드는 절대 유지되지 않습니다.
* 페이로드의 어떤 필드도 구제할 수 없으면 Claude Code는 마지막으로 수락된 캐시된 설정을 유지하고 치명적 오류를 기록합니다.
* [보안 승인 대화](#security-approval-dialogs)는 구제된 페이로드를 평가하므로 제거된 잘못된 항목은 승인을 위해 표시되지 않으며 실행되지 않습니다.

전달 문제를 디버깅하려면 `claude --debug-file <path>`를 실행하고 로그에서 `Remote settings`를 검색합니다. 조직에 배포하기 전에 테스트 머신에서 `claude doctor`로 페이로드 변경을 검증합니다.

<h3 id="enforce-fail-closed-startup">
  강제 실패 폐쇄 시작
</h3>

기본적으로 시작 시 원격 설정 가져오기가 실패하면 CLI는 관리 설정 없이 계속됩니다. 이 짧은 적용되지 않은 시간이 허용되지 않는 환경의 경우, 관리 설정에서 `forceRemoteSettingsRefresh: true`를 설정합니다.

이 설정이 활성화되면 CLI는 시작 시 원격 설정이 새로 가져올 때까지 차단됩니다. 가져오기가 실패하면 정책 없이 진행하는 대신 CLI가 종료됩니다. 이 설정은 자체 영속성을 가집니다. 서버에서 전달되면 로컬로도 캐시되므로 새 세션의 첫 번째 성공적인 가져오기 전에도 이후 시작이 동일한 동작을 적용합니다.

이를 활성화하려면 관리 설정 구성에 키를 추가합니다.

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

[엔드포인트 관리](/docs/ko/settings#settings-files) MDM 프로필 또는 시스템 `managed-settings.json` 파일에서 이 키를 설정하여 첫 번째 시작 시 강제 실패 폐쇄 동작을 적용할 수도 있습니다. 서버 페이로드가 전달되기 전입니다. v2.1.191 이상에서는 이 플래그가 위의 [우선순위 규칙](#settings-precedence)의 예외입니다. 캐시된 서버 관리 페이로드도 있는 경우에도 모든 관리 소스에서 설정된 경우 이를 준수하므로 서버 관리 설정이 있을 때 MDM 전달 값이 무시되지 않습니다. 설정 가져오기는 또한 `Cache-Control: no-cache` 헤더를 전송하므로 중간 HTTP 프록시가 오래된 응답을 제공하지 않습니다.

이 설정을 활성화하기 전에 네트워크 정책이 `api.anthropic.com`에 대한 연결을 허용하는지 확인합니다. 해당 엔드포인트에 도달할 수 없으면 CLI는 시작 시 종료되고 사용자는 Claude Code를 시작할 수 없습니다.

v2.1.139 이상에서는 `claude auth` 하위 명령(예: `claude auth login`)이 이 확인에서 제외되므로 만료된 자격 증명이 설정 가져오기 실패의 원인인 경우 사용자가 다시 인증할 수 있습니다.

<h3 id="security-approval-dialogs">
  보안 승인 대화
</h3>

보안 위험을 초래할 수 있는 특정 설정은 Claude Code가 이를 적용하기 전에 명시적인 사용자 승인이 필요합니다.

* **셸 명령 설정**: 셸 명령을 실행하는 설정
* **사용자 정의 환경 변수**: 알려진 안전 허용 목록에 없는 변수
* **Hook 구성**: 모든 hook 정의
* **관리 CLAUDE.md 콘텐츠**: 관리 설정을 통해 전달된 `claudeMd` 값

이러한 설정이 있을 때 사용자는 구성되는 내용을 설명하는 보안 대화를 봅니다. 사용자는 진행하려면 승인해야 합니다. 사용자가 설정을 거부하면 Claude Code가 종료됩니다.

<Note>
  `claude -p` 또는 Agent SDK 세션과 같은 비대화형 실행은 대화를 표시할 수 없습니다. 전달된 설정이 승인을 요구할 때 Claude Code는 해당 실행에만 이를 적용합니다. [로컬 캐시](#fetch-and-caching-behavior)에 승인된 것으로 기록하거나 쓰지 않으며, 다음 대화형 세션은 대화를 표시합니다. 사용자가 대화형 세션에서 승인할 때까지 각 비대화형 실행은 시작 시 설정을 다시 가져옵니다. v2.1.207 이전에는 비대화형 실행이 설정을 승인된 것으로 저장했으므로 나중에 대화형 세션은 이에 대한 대화를 절대 표시하지 않았습니다.
</Note>

<h2 id="platform-availability">
  플랫폼 가용성
</h2>

서버 관리 설정은 `api.anthropic.com`에 대한 직접 연결이 필요하며, 전달을 위해서는 세션이 조직 OAuth 로그인 또는 직접 구성된 API 키로 인증되어야 합니다. [`apiKeyHelper`](/docs/ko/settings#available-settings) 스크립트에서 반환된 키는 설정 가져오기를 트리거하지 않습니다.

서버 관리 설정은 타사 모델 공급자를 사용할 때는 사용할 수 없습니다:

* Amazon Bedrock
* Google Cloud의 Agent Platform
* Microsoft Foundry
* [Claude Platform on AWS](/docs/ko/claude-platform-on-aws)
* `ANTHROPIC_BASE_URL` 또는 타사 [LLM gateways](/docs/ko/llm-gateway)를 통한 사용자 정의 API 엔드포인트

셸에서 `CLAUDE_CODE_USE_*` 공급자 변수 또는 기본값이 아닌 `ANTHROPIC_BASE_URL`을 내보내면, Claude Code는 세션에 대한 설정 가져오기를 건너뜁니다. 내보내기를 서버 관리 `env` 블록으로 지울 수 없습니다. 왜냐하면 블록은 내보내기가 방지하는 가져오기를 통해 도착하기 때문입니다. [엔드포인트 관리 설정](/docs/ko/settings#settings-files) `env` 블록도 가져오기를 복원하지 않습니다. Claude Code는 관리 `env` 블록을 적용하기 전에 적격성을 확인하므로, 재정의는 세션의 공급자 선택을 변경하지만 가져오기는 건너뛴 상태로 유지됩니다.

서버 관리 전달을 복원하려면, 셸에서 내보내기를 제거하거나, 사용자 설정 `env` 블록에서 변수를 `""`로 설정합니다. 이는 적격성 확인 전에 적용됩니다. 사용자가 셸을 변경하도록 의존하지 않고 정책을 적용하려면, 대신 엔드포인트 관리 채널을 통해 설정을 전달합니다.

Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry 배포의 경우, 자체 호스팅 [Claude apps gateway](/docs/ko/claude-apps-gateway)는 동등한 원격 관리 설정 전달을 제공합니다. 게이트웨이에 로그인한 클라이언트는 `api.anthropic.com` 대신 게이트웨이에서 관리 설정을 가져옵니다. 시작 시 실패 의미론이 다릅니다. 게이트웨이에 도달할 수 없는 게이트웨이 클라이언트는 캐시된 설정으로 폴백하는 대신 오류로 종료되지만, 시간별 백그라운드 새로 고침은 두 채널 모두에서 실패 개방입니다.

<h2 id="audit-logging">
  감사 로깅
</h2>

설정 변경에 대한 감사 로그 이벤트는 규정 준수 API 또는 감사 로그 내보내기를 통해 사용할 수 있습니다. 액세스를 위해 Anthropic 계정 팀에 문의합니다.

감사 이벤트는 수행된 작업의 유형, 작업을 수행한 계정 및 기기, 이전 값과 새 값에 대한 참조를 포함합니다.

<h2 id="security-considerations">
  보안 고려사항
</h2>

서버 관리 설정은 중앙 집중식 정책 적용을 제공하지만 클라이언트 측 제어로 작동하며 보안 경계가 아닙니다. 관리되지 않는 기기에서 사용자는 이를 우회하기 위해 관리자 또는 sudo 액세스 권한이 필요하지 않습니다.

| 시나리오                                          | 동작                                                                                                                                                                                                                                                                                                     |
| :-------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 사용자가 캐시된 설정 파일을 편집함                           | 변조된 파일이 시작 시 적용되지만 다음 서버 가져오기에서 올바른 설정이 복원됩니다. v2.1.198부터 `env` 블록의 전송, API 라우팅 및 인증 환경 변수는 [서버가 페이로드를 확인할 때까지 보류됩니다](#fetch-and-caching-behavior)                                                                                                                                                     |
| 사용자가 캐시된 설정 파일을 삭제함                           | 첫 시작 동작이 발생합니다. 설정이 비동기적으로 가져오지며 짧은 적용되지 않은 시간이 있습니다.                                                                                                                                                                                                                                                  |
| 사용자가 수정된 Claude Code 바이너리를 실행함                | 수정된 클라이언트를 실행할 수 있는 사용자는 모든 클라이언트 측 제어를 우회할 수 있습니다.                                                                                                                                                                                                                                                    |
| 사용자가 이전 Claude Code 버전을 실행함                   | 서버 관리 설정 이전의 버전은 이를 가져오거나 적용하지 않습니다.                                                                                                                                                                                                                                                                   |
| API를 사용할 수 없음                                 | 캐시된 설정이 있으면 적용되고, 그렇지 않으면 다음 성공적인 가져오기까지 관리 설정이 적용되지 않습니다. v2.1.198부터 캐시된 `env` 블록의 전송, API 라우팅 및 인증 환경 변수는 [가져오기 실패 시 보류됩니다](#fetch-and-caching-behavior). 캐시의 나머지 부분은 여전히 적용됩니다. `forceRemoteSettingsRefresh: true`를 사용하면 CLI는 계속하는 대신 종료됩니다. [`claude auth` 부분 명령](#enforce-fail-closed-startup) 제외 |
| 사용자가 다른 조직으로 인증함                              | 관리 조직 외부의 계정에 대해 설정이 전달되지 않습니다.                                                                                                                                                                                                                                                                        |
| 사용자가 [타사 모델 공급자](#platform-availability)를 구성함 | 서버 관리 설정이 우회됩니다. 여기에는 `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS` 설정 또는 기본이 아닌 `ANTHROPIC_BASE_URL` 설정이 포함됩니다.                                                                                           |
| 네트워크 트래픽이 가로채지거나 리디렉션됨                        | 비활성화된 TLS 검증 또는 가로챈 트래픽은 클라이언트가 수신하는 설정을 변경할 수 있습니다.                                                                                                                                                                                                                                                   |

런타임 구성 변경을 감지하려면 [`ConfigChange` hooks](/docs/ko/hooks#configchange)를 사용하여 수정 사항을 기록하거나 적용되기 전에 무단 변경을 차단합니다.

클라이언트가 제공하는 자격 증명으로 사용자가 액세스할 수 있는 조직을 제한하려면 Claude 도움말 센터의 [테넌트 제한으로 네트워크 수준 액세스 제어 적용](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions)을 참조하십시오. 더 강력한 적용 보장을 위해 MDM 솔루션에 등록된 기기에서 [엔드포인트 관리 설정](/docs/ko/settings#settings-files)을 사용합니다.

<h2 id="see-also">
  참고 항목
</h2>

Claude Code 구성 관리를 위한 관련 페이지:

* [설정](/docs/ko/settings): 사용 가능한 모든 설정을 포함한 완전한 구성 참조
* [엔드포인트 관리 설정](/docs/ko/settings#settings-files): IT에서 기기에 배포하는 관리 설정
* [인증](/docs/ko/authentication): Claude Code에 대한 사용자 액세스 설정
* [보안](/docs/ko/security): 보안 보호 및 모범 사례
