> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 조직의 MCP 서버 액세스 제어

> 관리형 구성 파일, 허용 목록 및 거부 목록을 사용하여 사용자가 추가하거나 연결할 수 있는 MCP 서버를 제한합니다.

기본적으로 Claude Code를 실행하는 모든 사용자는 선택한 모든 [MCP 서버](/docs/ko/mcp)에 연결할 수 있습니다. Anthropic은 [Anthropic Directory](https://claude.ai/directory)에 추가하기 전에 커넥터를 [나열 기준](https://claude.com/docs/connectors/building/review-criteria)에 따라 검토하지만, MCP 서버에 대한 보안 감사나 관리를 수행하지 않습니다. 관리자는 조직에서 실행되는 서버를 제한할 수 있으며, 승인된 고정 집합을 배포하는 것부터 MCP를 완전히 비활성화하는 것까지 가능합니다.

이 페이지에서는 다음을 다룹니다:

* [필요한 제어 수준에 맞는 패턴 선택](#choose-a-pattern)
* [`managed-mcp.json`으로 고정 서버 집합 배포](#exclusive-control-with-managed-mcp-json), [MCP 완전히 비활성화](#disable-mcp-entirely) 방법 포함
* [허용 목록 및 거부 목록으로 서버 제어](#policy-based-control-with-allowlists-and-denylists)
* [제한이 서버를 차단할 때 사용자에게 표시되는 내용 알리기](#how-restrictions-appear-to-users)
* [조직이 실제로 사용하는 서버 모니터링](#monitor-mcp-usage)

<Note>
  [보안](/docs/ko/security) 페이지에서는 MCP 위협 모델과 서버를 승인하기 전에 평가하는 방법을 다룹니다. [적용할 항목 결정](/docs/ko/admin-setup#decide-what-to-enforce)에서는 MCP 제한을 다른 관리 제어와 함께 다룹니다.
</Note>

<h2 id="choose-a-pattern">
  패턴 선택
</h2>

Claude Code는 다양한 제한 수준을 지원합니다. 각 패턴은 아래에서 다루는 메커니즘 중 하나 또는 둘 다를 사용합니다: 고정 집합을 배포하기 위한 `managed-mcp.json`과 사용자가 구성하는 항목을 필터링하기 위한 `allowedMcpServers`/`deniedMcpServers`.

| 패턴            | 수행 작업                                             | 구성                                                                                       |
| :------------ | :------------------------------------------------ | :--------------------------------------------------------------------------------------- |
| **MCP 비활성화**  | 어디서도 서버가 로드되지 않음                                  | 빈 서버 맵이 있는 `managed-mcp.json`                                                            |
| **고정 배포**     | 모든 사용자가 동일한 서버를 받으며 다른 서버를 추가할 수 없음               | 원하는 서버가 있는 `managed-mcp.json`                                                            |
| **승인된 카탈로그**  | 승인된 서버 목록을 게시하고, 사용자가 원하는 서버를 추가하며, 다른 모든 항목은 차단됨 | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                 |
| **플러그인 서버만**  | 서버는 플러그인에서만 가져올 수 있으며, 사용자는 자신의 서버를 추가할 수 없음      | [`strictPluginOnlyCustomization`](/docs/ko/settings#strictpluginonlycustomization)과 목록의 `mcp` |
| **소프트 허용 목록** | 사용자가 자신의 설정에서 확대할 수 있는 허용 목록 적용                   | `allowManagedMcpServersOnly` 없는 `allowedMcpServers`                                      |
| **거부 목록만**    | 알려진 나쁜 서버를 차단하고 다른 모든 항목은 허용                      | `deniedMcpServers`                                                                       |
| **제한 없음**     | 사용자가 모든 항목을 추가                                    | 관리형 MCP 구성을 배포하지 않음                                                                      |

<Note>
  Claude Code에는 사용자가 검색하고 설치할 수 있는 기본 제공 MCP 서버 레지스트리가 없습니다. 승인된 카탈로그 패턴의 경우, 승인된 목록과 해당 `claude mcp add` 명령을 사용자가 찾을 수 있는 위치(예: 내부 wiki)에서 공유하거나, [관리형 플러그인 마켓플레이스](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions)를 통해 플러그인으로 서버를 배포하여 사용자가 `/plugin`에서 검색하고 설치할 수 있도록 합니다.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  managed-mcp.json으로 독점 제어
</h2>

`managed-mcp.json` 파일을 배포하면 Claude Code는 해당 파일이 정의하는 서버만 로드합니다. 사용자는 플러그인 제공 서버를 포함한 다른 MCP 서버를 추가, 수정 또는 사용할 수 없습니다. 이 파일은 [관리형 집합과 함께 허용](#allow-claude-ai-connectors-alongside-the-managed-set)하지 않는 한 claude.ai 커넥터도 억제합니다.

두 가지 다른 설정이 관리형 집합을 추가로 필터링할 수 있습니다:

* `allowedMcpServers` 및 `deniedMcpServers`는 관리형 서버에도 적용되므로, 이를 통과하지 못하는 관리형 서버는 로드되지 않습니다.
* 사용자의 자신의 `deniedMcpServers`는 설정에서 병합되므로, 사용자는 자신을 위해 관리형 서버를 차단할 수 있습니다.

전체 확인 순서는 [서버 평가 방법](#how-a-server-is-evaluated)을 참조하십시오.

`managed-mcp.json`은 독립 실행형 파일이므로 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 전달될 수 없습니다. 관리자 권한으로 시스템 경로에 쓸 수 있는 모든 프로세스가 배포할 수 있습니다. 규모가 큰 경우, 일반적으로 Jamf 또는 macOS의 구성 프로필, Windows의 그룹 정책 또는 Intune, Linux의 플릿 관리 등의 장치 관리 도구를 통합니다. Claude Code는 다음 경로 중 하나에서 파일을 찾습니다:

| 플랫폼         | 경로                                                         |
| :---------- | :--------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux 및 WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows     | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

파일은 프로젝트 [`.mcp.json`](/docs/ko/mcp#project-scope) 파일과 동일한 형식을 사용합니다:

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

<h3 id="authenticate-with-per-user-credentials">
  사용자별 자격증명으로 인증
</h3>

머신의 모든 사용자가 이 파일을 읽을 수 있으므로, `env` 블록에 API 키나 다른 자격증명을 저장하지 마십시오. 대신 다음 중 하나를 사용하여 사용자별 자격증명을 전달합니다:

* [`${VAR}` 확장](/docs/ko/mcp#environment-variable-expansion-in-mcp-json)으로 각 사용자의 환경에서 비밀을 읽습니다.
* [OAuth 또는 사용자별 헤더](/docs/ko/mcp#authenticate-with-remote-mcp-servers)로 각 사용자가 자신으로 인증합니다.
* [`headersHelper`](/docs/ko/mcp#use-dynamic-headers-for-custom-authentication)로 연결 시간에 자격증명을 생성합니다.

<h3 id="validate-the-configuration">
  구성 검증
</h3>

파일이 적용되는지 확인하려면 관리형 머신에서 두 가지 확인을 실행합니다:

1. `claude mcp list`는 `managed-mcp.json`의 서버만 표시합니다. 사용자의 자신의 서버가 여전히 나타나면, 파일이 읽혀지지 않습니다. 경로와 권한을 확인합니다.
2. `claude mcp add --transport http test https://example.com/mcp`는 `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`로 실패합니다. URL이 실제 서버일 필요는 없습니다. 정책 확인이 무엇이든 연결되기 전에 명령을 거부하기 때문입니다.

<h3 id="disable-mcp-entirely">
  MCP 완전히 비활성화
</h3>

빈 서버 맵을 포함하는 `managed-mcp.json`을 배포하여 모든 MCP 서버를 차단합니다:

```json theme={null}
{
  "mcpServers": {}
}
```

사용자는 `/mcp`에서 MCP 서버를 보지 못하며, `claude mcp add`는 위의 엔터프라이즈 정책 오류로 실패합니다. 사용자가 이전에 구성한 서버는 다음 번에 세션을 시작할 때 로드를 중지하며, 정책이 이유인지에 대한 경고가 없습니다.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  관리형 집합과 함께 claude.ai 커넥터 허용
</h3>

`managed-mcp.json`을 배포하면 기본적으로 조직의 claude.ai 관리 콘솔에서 관리자가 구성한 커넥터를 포함하여 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 억제합니다. 이러한 커넥터를 `managed-mcp.json`의 서버와 함께 로드하려면, [관리형 설정 소스](/docs/ko/admin-setup#decide-how-settings-reach-devices)에서 `"allowAllClaudeAiMcps": true`를 설정합니다. Claude Code v2.1.149 이상이 필요합니다.

설정이 활성화되면, Claude Code는 `managed-mcp.json`이 배포되지 않은 경우 로드할 것과 동일한 claude.ai 커넥터를 로드합니다. [허용 목록 및 거부 목록](#policy-based-control-with-allowlists-and-denylists)은 여전히 이러한 커넥터에 적용되므로, `deniedMcpServers`로 특정 커넥터를 차단할 수 있습니다. 이 설정은 claude.ai 커넥터에만 영향을 미치며, 플러그인 제공 서버는 억제된 상태로 유지됩니다.

Claude Code는 이 설정을 관리자 제어 정책 계층에서만 읽습니다: 서버 관리 설정, MDM 배포 plist 또는 HKLM 레지스트리 키, 또는 시스템 `managed-settings.json` 파일입니다. 사용자 또는 프로젝트 설정에 배치하면 효과가 없으므로, 사용자는 독점 제어가 억제한 커넥터를 다시 활성화할 수 없습니다.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  허용 목록 및 거부 목록으로 정책 기반 제어
</h2>

허용 목록 및 거부 목록은 구성된 서버 중 로드할 수 있는 서버를 필터링합니다. 이는 레지스트리가 아닙니다: 허용 목록 또는 거부 목록이 적용되기 전에 사용자, 플러그인 또는 `managed-mcp.json`에 의해 서버를 추가해야 합니다. 사용자에게 서버를 배포하려면 [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json)을 사용합니다. 두 목록은 또한 [`--mcp-config` CLI 플래그](/docs/ko/cli-reference#cli-flags)로 전달된 서버를 필터링합니다. `--strict-mcp-config`는 로드되는 구성 파일을 제한하며 두 목록을 우회하지 않습니다.

허용 목록을 권위 있게 만들려면, [관리형 설정 소스](/docs/ko/admin-setup#decide-how-settings-reach-devices)(예: 서버 관리 설정 또는 배포된 `managed-settings.json` 파일)에서 `allowedMcpServers` 및 `allowManagedMcpServersOnly: true`를 함께 설정합니다. [허용 목록을 관리형 설정만으로 제한](#restrict-the-allowlist-to-managed-settings-only)은 구성을 보여줍니다. `allowManagedMcpServersOnly` 없이, 모든 설정 소스의 허용 목록이 병합되며, 사용자의 자신의 `~/.claude/settings.json`을 포함하므로, 사용자는 허용 목록이 허용하는 항목을 확대할 수 있습니다. 거부 목록은 소스에 관계없이 병합됩니다.

<Note>
  `allowManagedMcpServersOnly`는 `allowManagedPermissionRulesOnly`와 별개이며, 이는 [권한 규칙](/docs/ko/permissions#managed-settings)만 잠급니다. 해당 플래그를 설정해도 MCP 허용 목록을 적용하지 않습니다.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  URL, 명령 또는 이름으로 서버 일치
</h3>

`allowedMcpServers` 및 `deniedMcpServers`는 항목 목록입니다. 각 항목은 URL, 명령 또는 이름으로 서버를 식별하는 단일 키가 있는 객체입니다:

| 키               | 일치                                  | 사용 대상             |
| :-------------- | :---------------------------------- | :---------------- |
| `serverUrl`     | 원격 서버 URL, 정확하거나 `*` 와일드카드 포함       | HTTP 및 SSE 서버     |
| `serverCommand` | stdio 서버를 시작하는 정확한 명령 및 인수          | Stdio 서버          |
| `serverName`    | 사용자 할당 레이블. 정확한 일치만, 와일드카드는 확장되지 않음 | 둘 다, 하지만 아래 경고 참조 |

`allowedMcpServers`를 설정하지 않는 것은 빈 배열로 설정하는 것과 다릅니다:

| 설정                  | 설정하지 않음 (기본값) | 빈 배열 `[]` | 채워짐         |
| :------------------ | :------------ | :-------- | :---------- |
| `allowedMcpServers` | 모든 서버 허용      | 서버 없음 허용  | 일치하는 서버만 허용 |
| `deniedMcpServers`  | 서버 없음 차단      | 서버 없음 차단  | 일치하는 서버 차단  |

[관리형 설정의 잘못된 항목](/docs/ko/settings#invalid-entries-in-managed-settings)을 참조하여 항목이 스키마 검증에 실패할 때 발생하는 상황을 확인합니다.

<Warning>
  `serverName` 항목은 두 목록 중 하나에서 보안 제어가 아닙니다. 이름은 `claude mcp add`를 실행하거나 구성 파일을 편집할 때 사용자가 할당하는 레이블이며, 기본 서버가 아니므로, 사용자는 모든 서버를 `github`라고 부를 수 있습니다. claude.ai 커넥터의 경우 이름은 claude.ai에서 반환하는 표시 이름이며, 이는 변경될 수 있습니다. 실제로 실행되는 서버를 적용하려면, `serverCommand` 또는 `serverUrl` 항목을 추가합니다.
</Warning>

`serverName` 검증은 두 목록 간에 다릅니다:

* `deniedMcpServers`에서, `serverName`은 모든 비어있지 않은 문자열을 허용하므로, 표시 이름으로 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 차단할 수 있습니다. 예를 들어, `{ "serverName": "claude.ai Slack" }`은 Slack 커넥터를 차단합니다. 거부가 이름 변경에 강력해야 할 때 또는 커넥터 이름이 충돌하고 ` (N)` 접미사를 얻을 때 `serverUrl` 항목을 선호합니다.
* `allowedMcpServers`에서, `serverName`은 문자, 숫자, 하이픈 및 밑줄로 제한됩니다. claude.ai 커넥터를 허용 목록에 추가하려면 `serverUrl`을 사용합니다.

모든 claude.ai 커넥터를 끄려면, [`disableClaudeAiConnectors`](/docs/ko/mcp#disable-claude-ai-connectors)를 참조합니다.

<h3 id="how-a-server-is-evaluated">
  서버 평가 방법
</h3>

`managed-mcp.json`의 서버를 포함한 서버를 로드하기 전에, Claude Code는 순서대로 세 가지 확인을 실행합니다:

1. **목록 병합.** 모든 설정 소스의 허용 목록 및 거부 목록 항목이 하나의 허용 목록 및 하나의 거부 목록으로 결합됩니다. `allowManagedMcpServersOnly`가 `true`일 때, 관리형 허용 목록만 유지됩니다. 거부 목록은 항상 모든 소스에서 병합됩니다.
2. **거부 목록 확인.** URL, 명령 또는 이름으로 거부 목록 항목과 일치하는 서버는 차단됩니다. 거부 목록 일치를 재정의하는 것은 없습니다.
3. **허용 목록 확인.** `allowedMcpServers`가 어디서도 설정되지 않으면, 거부 목록을 통과한 모든 서버가 로드됩니다. 설정되면, 서버가 일치해야 하는 항목은 아래 표에 표시된 유형에 따라 다릅니다.

| 서버 유형            | 일치할 때 허용됨                                                                 |
| :--------------- | :------------------------------------------------------------------------ |
| 원격 (HTTP 또는 SSE) | `serverUrl` 항목. `serverName` 일치는 허용 목록에 `serverUrl` 항목이 없을 때만 계산됨         |
| Stdio            | `serverCommand` 항목. `serverName` 일치는 허용 목록에 `serverCommand` 항목이 없을 때만 계산됨 |

이러한 확인 내에서 세 가지 일치 규칙이 적용됩니다:

* **명령은 정확하게 일치합니다.** 모든 인수, 순서대로. `["npx", "-y", "server"]`는 `["npx", "server"]` 또는 `["npx", "-y", "server", "--flag"]`와 일치하지 않습니다.
* **`serverCommand` 및 `serverUrl` 값은 일치하기 전에 확장됩니다.** 정책 항목과 서버의 구성된 값 모두 `.mcp.json`과 동일한 [`${VAR}` 및 `${VAR:-default}` 확장](/docs/ko/mcp#environment-variable-expansion-in-mcp-json)을 거치므로, `["${HOME}/bin/server"]`로 작성된 항목은 동일한 참조 또는 확장된 경로를 사용하는 서버 구성과 일치합니다. Windows에서는 `${HOME}` 대신 `${USERPROFILE}`과 같이 설정된 환경 변수를 참조합니다. `serverName` 값은 문자 그대로 일치하며 절대 확장되지 않습니다.
* **URL은 `*` 와일드카드를 지원합니다** 패턴의 어디서나, 스키마 포함. 호스트명 일치는 대소문자를 구분하지 않으며 후행 FQDN 점을 무시하므로, `https://Mcp.Example.com/*`는 `https://mcp.example.com/api`와 일치합니다. 경로는 대소문자를 구분합니다.

| 패턴                          | 허용                                     |
| :-------------------------- | :------------------------------------- |
| `https://mcp.example.com/*` | 특정 도메인의 모든 경로                          |
| `https://mcp.example.com`   | 또한 해당 도메인의 모든 경로. 경로가 없는 패턴은 모든 경로와 일치 |
| `https://*.example.com/*`   | `example.com`의 모든 하위 도메인               |
| `http://localhost:*/*`      | localhost의 모든 포트                       |
| `*://mcp.example.com/*`     | 특정 도메인으로의 모든 스키마                       |

`${VAR}` 확장은 Claude Code의 자체 프로세스 환경을 읽으므로, 변수를 참조하는 `serverCommand` 또는 `serverUrl` 정책 항목은 사용자가 설정한 모든 값으로 확장됩니다. 적용을 위해 의존하는 항목에는 리터럴 URL 및 명령을 사용합니다.

<h3 id="example-configuration">
  예제 구성
</h3>

아래 구성은 거부 목록이 있는 하드 허용 목록을 설정합니다. 강조된 줄은 나머지 목록이 평가되는 방식을 변경하며, 블록 후의 설명은 각각을 설명합니다:

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **3번 줄**: 첫 번째 `serverUrl` 항목. 하나가 존재하면, 모든 원격 서버는 URL 패턴과 일치해야 하므로, 사용자는 허용된 이름을 제공하여 나열되지 않은 원격 서버를 얻을 수 없습니다.
* **5번 줄**: 첫 번째 `serverCommand` 항목. stdio 서버에 대해 동일한 효과이므로, 모든 로컬 서버는 나열된 명령과 정확하게 일치해야 합니다.
* **11번 줄**: 거부 목록의 `serverName` 항목. 거부 목록 항목은 항상 적용되므로, `dangerous-server`라는 모든 서버는 URL 또는 명령에 관계없이 차단됩니다.

이 허용 목록의 `serverName` 항목은 두 전송 유형이 이미 더 엄격한 항목을 가지고 있으므로 아무것도 일치하지 않습니다.

아래 아코디언은 다른 허용 목록 및 거부 목록 조합에 대해 서버가 평가되는 방식을 안내합니다.

<Accordion title="URL만 허용 목록">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://mcp.example.com/*" },
      { "serverUrl": "https://*.internal.example.com/*" }
    ]
  }
  ```

  | 서버                                              | 결과                      |
  | :---------------------------------------------- | :---------------------- |
  | `https://mcp.example.com/api`의 HTTP 서버          | 허용됨: URL 패턴과 일치         |
  | `https://api.internal.example.com/mcp`의 HTTP 서버 | 허용됨: 와일드카드 하위 도메인과 일치   |
  | `https://external.example.com/mcp`의 HTTP 서버     | 차단됨: URL 패턴과 일치하지 않음    |
  | 모든 명령이 있는 Stdio 서버                              | 차단됨: 일치할 이름 또는 명령 항목 없음 |
</Accordion>

<Accordion title="명령만 허용 목록">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | 서버                                               | 결과                |
  | :----------------------------------------------- | :---------------- |
  | `["npx", "-y", "approved-package"]`가 있는 Stdio 서버 | 허용됨: 명령과 일치       |
  | `["node", "server.js"]`가 있는 Stdio 서버             | 차단됨: 명령과 일치하지 않음  |
  | `my-api`라는 HTTP 서버                               | 차단됨: 일치할 이름 항목 없음 |
</Accordion>

<Accordion title="혼합 이름 및 명령 허용 목록">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | 서버                                                               | 결과                                     |
  | :--------------------------------------------------------------- | :------------------------------------- |
  | `["npx", "-y", "approved-package"]`가 있는 `local-tool`이라는 Stdio 서버 | 허용됨: 명령과 일치                            |
  | `["node", "server.js"]`가 있는 `local-tool`이라는 Stdio 서버             | 차단됨: 명령 항목이 존재하지만 일치하지 않음              |
  | `["node", "server.js"]`가 있는 `github`라는 Stdio 서버                  | 차단됨: stdio 서버는 명령 항목이 존재할 때 명령과 일치해야 함 |
  | `github`라는 HTTP 서버                                               | 허용됨: 이름과 일치                            |
  | `other-api`라는 HTTP 서버                                            | 차단됨: 이름과 일치하지 않음                       |
</Accordion>

<Accordion title="이름만 허용 목록">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverName": "internal-tool" }
    ]
  }
  ```

  | 서버                                    | 결과               |
  | :------------------------------------ | :--------------- |
  | 모든 명령이 있는 `github`라는 Stdio 서버         | 허용됨: 명령 제한 없음    |
  | 모든 명령이 있는 `internal-tool`이라는 Stdio 서버 | 허용됨: 명령 제한 없음    |
  | `github`라는 HTTP 서버                    | 허용됨: 이름과 일치      |
  | `other`라는 모든 서버                       | 차단됨: 이름과 일치하지 않음 |
</Accordion>

<Accordion title="거부 목록 재정의가 있는 허용 목록">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://*.example.com/*" }
    ],
    "deniedMcpServers": [
      { "serverUrl": "https://staging.example.com/*" }
    ]
  }
  ```

  | 서버                                         | 결과                                 |
  | :----------------------------------------- | :--------------------------------- |
  | `https://mcp.example.com/api`의 HTTP 서버     | 허용됨: 허용 목록 URL 패턴과 일치, 거부 목록 일치 없음 |
  | `https://staging.example.com/api`의 HTTP 서버 | 차단됨: 둘 다 일치하지만 거부 목록이 우선           |
  | `https://other.com/mcp`의 HTTP 서버           | 차단됨: 허용 목록과 일치하지 않음                |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  허용 목록을 관리형 설정만으로 제한
</h3>

관리형 허용 목록이 유일하게 적용되도록 하려면, 관리형 설정 파일에서 `allowManagedMcpServersOnly`를 설정합니다:

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

`allowManagedMcpServersOnly`가 `true`일 때, 사용자, 프로젝트 및 로컬 설정의 허용 목록은 무시됩니다. 거부 목록은 여전히 모든 소스에서 병합되므로, 사용자는 항상 자신을 위해 서버를 차단할 수 있습니다.

<h2 id="how-restrictions-appear-to-users">
  제한이 사용자에게 표시되는 방식
</h2>

제한이 서버를 차단할 때, 사용자는 `claude mcp add`에서 오류를 보거나 서버가 조용히 로드를 중지합니다. 이 표를 사용하여 이러한 보고서를 인식하고 변경을 롤아웃하기 전에 사용자에게 예상되는 사항을 알립니다:

| 제한                                               | 사용자가 보는 것                                                                                                  |
| :----------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`이 있고 사용자가 `claude mcp add`를 실행 | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` |
| 서버가 거부 목록에 있고 사용자가 `claude mcp add`를 실행          | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                        |
| 서버가 허용 목록에 없고 사용자가 `claude mcp add`를 실행          | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                         |
| 이전에 구성된 서버가 이제 정책에 의해 차단됨                        | 서버가 `/mcp` 및 `claude mcp list`에서 조용히 사라지며 경고 없음                                                            |

마지막 경우, 사용자는 정책이 서버가 사라진 이유인지에 대한 신호를 받지 못하므로, 새 제한을 롤아웃할 때 영향을 받는 사용자에게 어떤 서버가 차단되는지 알립니다.

<h2 id="monitor-mcp-usage">
  MCP 사용 모니터링
</h2>

[OpenTelemetry 내보내기](/docs/ko/monitoring-usage)가 구성되면, Claude Code는 사용자가 호출하는 MCP 서버 및 도구를 기록할 수 있습니다. `OTEL_LOG_TOOL_DETAILS=1`을 설정하여 도구 이벤트에 MCP 서버 및 도구 이름을 포함한 다음, 수집기에서 집계하여 사용자가 실제로 연결하는 서버를 확인합니다. 내보내기를 설정하고 전체 이벤트 스키마는 [모니터링](/docs/ko/monitoring-usage)을 참조하십시오.

<h2 id="configuration-summary">
  구성 요약
</h2>

이 페이지에서 다루는 모든 파일 및 설정, 제어 항목 및 전달 방법:

| 표면                           | 제어 항목                                              | 위치                                                                                                         | 전달 방법                                                                                                                        |
| :--------------------------- | :------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | 고정 서버 집합, 독점 제어                                    | 시스템 경로: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, 또는 `C:\Program Files\ClaudeCode\` | MDM, GPO, 플릿 관리, 또는 관리자 권한이 있는 모든 프로세스. 서버 관리 설정을 통해 설정할 수 없음                                                                |
| `allowedMcpServers`          | 허용된 서버의 허용 목록                                      | 모든 [설정 파일](/docs/ko/settings#settings-files); 모든 소스의 항목이 `allowManagedMcpServersOnly`가 설정되지 않으면 병합              | 적용을 위해, [관리형 설정 소스](/docs/ko/admin-setup#decide-how-settings-reach-devices): 서버 관리 설정, `managed-settings.json`, MDM 프로필, 또는 레지스트리 |
| `deniedMcpServers`           | 차단된 서버의 거부 목록                                      | 모든 설정 파일; 모든 소스의 항목이 병합                                                                                    | `allowedMcpServers`와 동일                                                                                                      |
| `allowManagedMcpServersOnly` | 허용 목록을 관리형 소스만으로 잠금                                | 관리형 설정 소스만; 설정은 다른 곳에서 효과 없음                                                                               | `allowedMcpServers`와 동일                                                                                                      |
| `allowAllClaudeAiMcps`       | `managed-mcp.json`과 함께 claude.ai 커넥터를 로드하고 억제하지 않음 | 관리형 설정 소스만; 설정은 다른 곳에서 효과 없음                                                                               | `allowedMcpServers`와 동일                                                                                                      |

<h2 id="related-resources">
  관련 리소스
</h2>

* [적용할 항목 결정](/docs/ko/admin-setup#decide-what-to-enforce): 권한 규칙, 샌드박싱 및 다른 관리 제어와 함께 MCP 제한
* [MCP를 통해 Claude Code를 도구에 연결](/docs/ko/mcp): 전송, 범위 및 인증을 포함한 전체 MCP 참조
* [설정](/docs/ko/settings): 설정 계층 구조 및 관리형 설정이 우선하는 방식
* [서버 관리 설정](/docs/ko/server-managed-settings): Claude.ai 관리 콘솔에서 `allowedMcpServers` 및 `deniedMcpServers` 전달
* [보안](/docs/ko/security): 이러한 제어가 방어하는 위협 모델
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, 시트 관리 및 롤아웃 플레이북
