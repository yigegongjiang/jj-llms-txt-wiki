> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 조직의 MCP 서버 접근 제어

> 관리형 구성 파일, 관리형 설정, 허용 목록 및 거부 목록을 사용하여 사용자가 추가하거나 연결할 수 있는 MCP 서버를 제한하거나 모든 사용자에게 서버를 제공합니다.

기본적으로 Claude Code를 실행하는 모든 사용자는 선택한 [MCP 서버](/docs/ko/mcp)를 연결할 수 있습니다. Anthropic은 [Anthropic Directory](https://claude.ai/directory)에 추가하기 전에 커넥터를 [나열 기준](https://claude.com/docs/connectors/building/review-criteria)에 따라 검토하지만, MCP 서버에 대한 보안 감사나 관리를 수행하지 않습니다. 관리자는 조직에서 실행되는 서버를 제한할 수 있으며, 승인된 고정 서버 집합을 배포하는 것부터 MCP를 완전히 비활성화하는 것까지 가능하며, 모든 사용자에게 서버를 제공할 수도 있습니다.

이러한 제한 사항은 Claude Code가 자체적으로 로드하는 서버(claude.ai에서 가져오는 커넥터 포함)를 다룹니다. 데스크톱 앱이 로컬 및 SSH 세션에 제공하는 커넥터는 프로세스 내에서 도착하며 claude.ai 조직 설정에서 관리됩니다. [커넥터가 Claude Code에 도달하는 방식](/docs/ko/mcp#how-connectors-reach-claude-code)은 클라우드 세션을 포함한 각 종류의 세션에서 커넥터에 적용되는 제어를 보여줍니다.

이 페이지에서는 다음을 다룹니다:

* [필요한 제어 수준에 맞는 패턴 선택](#choose-a-pattern)
* [`managed-mcp.json`으로 고정 서버 집합 배포](#exclusive-control-with-managed-mcp-json) ([MCP를 완전히 비활성화하는 방법](#disable-mcp-entirely) 포함)
* [관리형 설정을 통해 서버 제공](#provide-servers-through-managed-settings) (사용자가 자신의 서버 유지)
* [허용 목록 및 거부 목록으로 서버 제어](#policy-based-control-with-allowlists-and-denylists)
* [제한이 서버를 차단할 때 사용자에게 표시되는 내용](#how-restrictions-appear-to-users)
* [조직이 실제로 사용하는 서버 모니터링](#monitor-mcp-usage)

<Note>
  [보안](/docs/ko/security) 페이지는 MCP 위협 모델과 서버를 승인하기 전에 평가하는 방법을 다룹니다. [적용할 항목 결정](/docs/ko/admin-setup#decide-what-to-enforce)은 다른 관리 제어와 함께 MCP 제한을 다룹니다.
</Note>

<h2 id="choose-a-pattern">
  패턴 선택
</h2>

Claude Code는 다양한 제한 수준을 지원합니다. 각 패턴은 다음 중 하나 이상의 메커니즘을 사용합니다: 고정 집합을 배포하기 위한 `managed-mcp.json`, 사용자가 추가한 서버와 함께 서버를 제공하기 위한 `managedMcpServers` 관리 설정, 그리고 사용자가 구성하는 항목을 필터링하기 위한 `allowedMcpServers`/`deniedMcpServers`.

| 패턴            | 기능                                                                                                                                                                 | 구성                                                                                                 |
| :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| **MCP 비활성화**  | [세션을 시작한 앱이 등록하는 인프로세스 서버](#exclusive-control-with-managed-mcp-json) 및 [managedMcpServers를 통해 제공하는](#provide-servers-through-managed-settings) 서버를 제외한 서버가 로드되지 않음 | 빈 서버 맵이 있는 `managed-mcp.json`                                                                      |
| **고정 배포**     | 모든 사용자가 동일한 서버를 받으며 다른 서버를 추가할 수 없음                                                                                                                                | 원하는 서버가 있는 `managed-mcp.json`                                                                      |
| **제공된 서버**    | 모든 사용자가 나열한 원격 서버를 받고 자신의 서버를 유지함                                                                                                                                  | 관리 설정의 `managedMcpServers`                                                                         |
| **승인된 카탈로그**  | 승인된 서버 목록을 게시하고, 사용자가 원하는 서버를 추가하며, 다른 모든 것은 차단됨                                                                                                                   | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                           |
| **플러그인 서버만**  | 사용자가 `~/.claude.json` 또는 `.mcp.json`을 통해 서버를 추가할 수 없으며, 플러그인 서버는 여전히 로드됨                                                                                           | [`strictPluginOnlyCustomization`](/docs/ko/settings-reference#strictpluginonlycustomization)과 목록의 `mcp` |
| **소프트 허용 목록** | 사용자가 자신의 설정에서 확대할 수 있는 허용 목록 적용                                                                                                                                    | `allowManagedMcpServersOnly` 없는 `allowedMcpServers`                                                |
| **거부 목록만**    | 알려진 나쁜 서버를 차단하고 다른 모든 것을 허용                                                                                                                                        | `deniedMcpServers`                                                                                 |
| **제한 없음**     | 사용자가 아무것이나 추가                                                                                                                                                      | 관리 MCP 구성을 배포하지 않음                                                                                 |

<Note>
  Claude Code에는 사용자가 검색하고 설치할 수 있는 기본 제공 MCP 서버 레지스트리가 없습니다. 승인된 카탈로그 패턴의 경우, 승인된 목록과 해당 `claude mcp add` 명령을 사용자가 찾을 수 있는 위치(예: 내부 위키)에서 공유하거나, [관리 플러그인 마켓플레이스](/docs/ko/plugins/org#restrict-what-users-can-install)를 통해 플러그인으로 서버를 배포하여 사용자가 `/plugin`에서 검색하고 설치할 수 있도록 합니다.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  managed-mcp.json으로 독점 제어
</h2>

`managed-mcp.json` 파일을 배포하면 Claude Code는 다음 서버만 로드합니다:

* 파일에서 정의한 서버
* [managedMcpServers를 통해 제공하는](#provide-servers-through-managed-settings) 서버
* 세션을 시작한 앱이 등록하는 인프로세스 서버(예: VS Code 확장 프로그램의 자체 서버 또는 [데스크톱 앱이 제공하는 커넥터](/docs/ko/mcp#how-connectors-reach-claude-code))

사용자는 플러그인 제공 서버 및 [`--mcp-config` CLI 플래그](/docs/ko/cli-reference#cli-flags)로 전달된 서버를 포함하여 다른 MCP 서버를 추가, 수정 또는 사용할 수 없습니다. 또한 이 파일은 [관리되는 집합과 함께 허용](#allow-claude-ai-connectors-alongside-the-managed-set)하지 않는 한 Claude Code가 자체적으로 가져오는 claude.ai 커넥터를 억제합니다.

<h3 id="deploy-managed-mcp-json">
  managed-mcp.json 배포
</h3>

`managed-mcp.json`은 독립 실행형 파일이므로 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 제공될 수 없습니다. 독점 제어 없이 관리되는 설정을 통해 서버를 제공하려면 [`managedMcpServers`](#provide-servers-through-managed-settings)를 사용하세요.

관리자 권한이 있는 시스템 경로에 쓸 수 있는 모든 프로세스가 파일을 배포할 수 있습니다. 전체 플릿에서는 일반적으로 Jamf 또는 macOS의 구성 프로필, Windows의 그룹 정책 또는 Intune, Linux의 선택한 플릿 관리 도구 등의 디바이스 관리 도구를 통해 배포됩니다. Claude Code는 다음 경로 중 하나에서 파일을 찾습니다:

| 플랫폼         | 경로                                                         |
| :---------- | :--------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux 및 WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows     | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

이 파일은 프로젝트 [`.mcp.json`](/docs/ko/mcp#project-scope) 파일과 동일한 형식을 사용합니다:

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

머신의 모든 사용자가 이 파일을 읽을 수 있으므로 `env` 블록에 API 키 또는 기타 자격증명을 저장하지 마세요. 대신 다음 중 하나를 사용하여 사용자별 자격증명을 전달하세요:

* [`${VAR}` 확장](/docs/ko/mcp#environment-variable-expansion-in-mcp-json)을 사용하여 각 사용자의 환경에서 비밀을 읽습니다.
* [OAuth 또는 사용자별 헤더](/docs/ko/mcp#authenticate-with-remote-mcp-servers)를 사용하여 각 사용자가 자신으로 인증합니다.
* [`headersHelper`](/docs/ko/mcp#use-dynamic-headers-for-custom-authentication)를 사용하여 연결 시간에 자격증명을 생성합니다.

<h3 id="servers-passed-with-mcp-config-or-strict-mcp-config">
  `--mcp-config` 또는 `--strict-mcp-config`로 전달된 서버
</h3>

세션이 `managed-mcp.json`이 배포된 상태에서 `--mcp-config`를 통해 서버를 수신하면, 사용자가 보는 내용은 워크스테이션과 클라우드 세션 간에 다릅니다:

* 워크스테이션에서 Claude Code는 `You cannot dynamically configure MCP servers when an enterprise MCP config is present`라는 메시지와 함께 시작 시 종료됩니다.
* [클라우드 세션](/docs/ko/claude-code-on-the-web)에서 파일이 배포된 호스트(예: [자체 호스팅 러너](/docs/ko/self-hosted-environments-configuration#mcp-servers))에서 Claude Code는 관리되는 서버만으로 시작하고 claude.ai 커넥터 및 클라우드 호스트가 `--mcp-config`를 통해 제공하는 다른 서버를 건너뜁니다. 세션의 어떤 것도 사용자에게 어떤 서버가 제외되었는지 알려주지 않습니다. Claude Code는 stderr의 경고에서 이름을 지정하며, 자체 호스팅 러너는 이를 `debug` 로그 수준에서 기록합니다.

`--strict-mcp-config` 플래그는 관리되는 집합을 교체하도록 요청합니다. 사용자가 이러한 파일이 배포된 상태에서 이를 전달하면 Claude Code는 워크스테이션과 클라우드 세션 모두에서 시작 시 종료됩니다.

<h3 id="how-allowlists-and-denylists-apply-to-the-managed-set">
  허용 목록 및 거부 목록이 관리되는 집합에 적용되는 방식
</h3>

거부 목록은 `managed-mcp.json`의 서버를 추가로 필터링할 수 있습니다:

* `deniedMcpServers`는 관리되는 서버에도 적용되므로 항목과 일치하는 관리되는 서버는 로드되지 않습니다.
* 사용자의 자체 `deniedMcpServers`는 설정에서 병합되므로 사용자는 자신을 위해 관리되는 서버를 차단할 수 있습니다.

`allowedMcpServers`는 `managed-mcp.json`의 서버에 적용되지 않습니다. 한 가지 예외가 있습니다: Claude Code는 정의가 [`${VAR}` 확장](/docs/ko/mcp#environment-variable-expansion-in-mcp-json)을 사용하는 서버를 허용 목록에 대해 확인합니다. 이는 해당 서버의 유효한 구성이 파일만이 아닌 각 사용자의 환경에서 나오기 때문입니다. v2.1.259 이전에는 허용 목록이 설정될 때마다 모든 관리되는 서버가 허용 목록을 통과해야 했습니다. 서버가 평가되는 방식에 대해서는 [서버가 평가되는 방식](#how-a-server-is-evaluated)을 참조하여 `${VAR}` 확인을 트리거하는 필드와 전체 확인 순서를 확인하세요.

`allowedMcpServers`를 사용하여 자신의 `managed-mcp.json` 서버 중 일부가 로드되지 않도록 유지한 경우, 해당 서버는 `${VAR}` 확장을 사용하지 않는 한 각 사용자의 v2.1.259 이상의 첫 번째 시작 시 로드되기 시작합니다. 프롬프트나 공지 없이: `deniedMcpServers`만 해당 서버에서 계속 빼집니다. 거부 목록 항목을 추가하거나 사용자가 업그레이드하기 전에 그룹별로 별도의 `managed-mcp.json`을 배포하세요.

<h3 id="validate-the-configuration">
  구성 검증
</h3>

파일이 적용되고 있는지 확인하려면 관리되는 머신에서 두 가지 확인을 실행하세요:

1. `claude mcp list`는 `managed-mcp.json`의 서버와 `managedMcpServers`를 통해 제공하는 서버만 표시합니다. 다음 두 가지 다른 결과는 문제가 있음을 의미합니다:
   * 사용자의 자체 서버가 여전히 나타나면 Claude Code가 파일을 읽지 않는 것입니다. 경로와 상위 디렉터리의 권한을 확인하세요.
   * 파일의 서버가 나타나지 않고 `MCP config diagnostics` 섹션이 엔터프라이즈 구성을 파싱 실패로 표시하면 Claude Code가 파일을 읽거나 파싱할 수 없습니다. 해당 섹션에서 명명한 오류를 수정한 다음 사용자가 Claude Code를 다시 시작하도록 하세요.
2. `claude mcp add --transport http test https://example.com/mcp`는 `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`로 실패합니다. URL이 실제 서버일 필요는 없습니다. 정책 확인이 무엇이든 연락하기 전에 명령을 거부하기 때문입니다.

<h3 id="disable-mcp-entirely">
  MCP 완전히 비활성화
</h3>

빈 서버 맵을 포함하는 `managed-mcp.json`을 배포하여 [세션을 시작한 앱이 등록하는 인프로세스 서버](#exclusive-control-with-managed-mcp-json)를 제외한 모든 MCP 서버를 차단하세요:

```json theme={null}
{
  "mcpServers": {}
}
```

`claude mcp add`는 위의 엔터프라이즈 정책 오류로 실패합니다. 사용자가 이전에 구성한 서버는 다음 번에 세션을 시작할 때 로드를 중지합니다. 정책이 이유라는 경고는 없습니다. `managedMcpServers`를 통해 제공하는 서버는 빈 맵 아래에서도 로드되므로 MCP를 완전히 비활성화하려면 해당 키도 설정하지 마세요.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  관리되는 집합과 함께 claude.ai 커넥터 허용
</h3>

기본적으로 `managed-mcp.json`을 배포하면 Claude Code가 자체적으로 가져오는 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)(조직을 위해 관리자가 claude.ai 관리 콘솔에서 구성한 커넥터 포함)가 억제됩니다. 이러한 커넥터를 `managed-mcp.json`의 서버와 함께 로드하려면 [관리되는 설정 소스](/docs/ko/admin-setup#decide-how-settings-reach-devices)에서 `"allowAllClaudeAiMcps": true`를 설정하세요.

설정이 활성화되면 Claude Code는 `managed-mcp.json`이 배포되지 않은 경우 로드할 것과 동일한 claude.ai 커넥터를 로드합니다. [허용 목록 및 거부 목록](#policy-based-control-with-allowlists-and-denylists)은 여전히 해당 커넥터에 적용되므로 `deniedMcpServers`로 특정 커넥터를 차단할 수 있습니다. 이 설정은 Claude Code가 자체적으로 가져오는 claude.ai 커넥터에만 영향을 미칩니다. 플러그인 제공 서버는 억제된 상태로 유지됩니다.

클라우드 세션과 데스크톱 앱의 로컬 및 SSH 세션은 [Claude Code에 커넥터가 도달하는 방식](/docs/ko/mcp#how-connectors-reach-claude-code)에서 설명하는 다른 방식으로 커넥터를 수신합니다. 클라우드 세션을 실행하는 호스트(예: [자체 호스팅 러너 호스트](/docs/ko/self-hosted-environments-configuration#mcp-servers))의 `managed-mcp.json`은 `allowAllClaudeAiMcps`를 설정했는지 여부와 관계없이 해당 세션의 커넥터를 억제합니다. `managed-mcp.json`은 데스크톱 앱이 로컬 및 SSH 세션에 제공하는 커넥터에 도달하지 않습니다.

Claude Code는 `allowAllClaudeAiMcps`를 관리자 제어 정책 계층에서만 읽습니다: 서버 관리 설정, MDM 배포 plist 또는 HKLM 레지스트리 키, 또는 시스템 `managed-settings.json` 파일. 사용자 또는 프로젝트 설정에 배치하면 효과가 없으므로 사용자는 독점 제어가 억제한 커넥터를 다시 활성화할 수 없습니다.

<h2 id="provide-servers-through-managed-settings">
  관리되는 설정을 통해 서버 제공
</h2>

모든 사용자에게 MCP를 독점적으로 제어하지 않으면서 원격 MCP 서버 세트를 제공하려면, [관리되는 설정 소스](/docs/ko/admin-setup#decide-how-settings-reach-devices)의 `managedMcpServers` 아래에 나열합니다: 서버 관리 설정, [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway-config#what-goes-in-cli) 정책, MDM 프로필 또는 레지스트리 정책, 또는 `managed-settings.json`. 사용자는 자신이 추가한 서버를 유지하고 귀사의 서버를 추가로 받습니다. Claude Code v2.1.259 이상이 필요합니다. 이전 클라이언트는 이 키를 무시합니다.

값은 서버 이름으로 키가 지정된 객체입니다. 각 항목은 프로젝트 [`.mcp.json`](/docs/ko/mcp#project-scope) 파일의 HTTP 또는 SSE 서버와 동일한 형태이며, [원격 MCP 서버로 인증](/docs/ko/mcp#authenticate-with-remote-mcp-servers)에 설명된 선택적 `headers` 및 `oauth` 멤버를 포함합니다. 이 예제는 각 사용자가 OAuth로 로그인하는 검색 서버와 조직에서 발급한 헤더를 전송하는 레코드 서버를 제공합니다:

```json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    },
    "records": {
      "type": "http",
      "url": "https://records.example.com/mcp",
      "headers": {
        "X-Records-Key": "key-issued-for-all-claude-code-users"
      }
    }
  }
}
```

사용자를 포함하여 머신의 관리되는 설정을 읽을 수 있는 모든 사람이 여기에 설정한 헤더 값을 읽을 수 있습니다. 해당 전체 대상자를 위해 발급된 자격 증명을 사용하거나, `headers`를 생략하고 각 사용자가 OAuth로 로그인하도록 합니다.

<h3 id="what-an-entry-can-contain">
  항목에 포함될 수 있는 것
</h3>

Claude Code는 아래의 모든 검사를 통과할 때만 항목을 로드합니다. 하나라도 실패하면 항목을 삭제하고, `/status`로 읽을 수 있는 알림을 기록하며, 여전히 다른 항목을 로드합니다:

* `type`은 `http` 또는 `sse`입니다. `.mcp.json`에서와 같이 `streamable-http`는 `http`의 별칭으로 허용됩니다.
* `url`은 `https://` URL입니다. Claude Code는 `localhost`를 가리키는 것을 포함하여 일반 `http://` URL을 거부합니다.
* 항목에 `command`, `args`, `env` 또는 `headersHelper` 멤버가 없으므로, 관리되는 설정 문서는 사용자의 머신에서 실행할 프로그램을 절대 지정하지 않습니다.
* 어떤 값도 `${VAR}` 참조를 포함하지 않습니다. Claude Code는 이러한 항목에서 환경 변수를 확장하지 않으므로 리터럴 값을 작성합니다.
* 서버 이름은 문자, 숫자, 하이픈 및 밑줄만 포함하고, 어떤 키나 값도 제어 또는 보이지 않는 서식 문자를 포함하지 않습니다.

Claude Desktop에는 동일한 이름의 관리되는 설정이 있으며, 그 값은 다른 항목 형태의 배열이므로, 하나를 다른 것으로 복사하지 마십시오. Claude Code는 배열 형태를 허용하지 않으며 로드하는 대신 알림을 기록합니다.

Claude 앱 게이트웨이는 부팅할 때 동일한 검사를 실행합니다. [정책의 MCP 서버](/docs/ko/claude-apps-gateway-config#mcp-servers-in-a-policy)를 참조하십시오.

<h3 id="how-provided-servers-load">
  제공된 서버가 로드되는 방식
</h3>

이러한 규칙은 제공된 서버가 다른 서버 정의 또는 이 페이지의 다른 설정과 겹칠 때 무엇이 로드되는지 결정합니다:

* 제공된 서버는 로컬, 프로젝트 또는 사용자 범위의 동일한 이름의 서버보다 우선하며, 동일한 URL을 가리키는 플러그인 서버 또는 claude.ai 커넥터보다 우선합니다.
* `managed-mcp.json`도 배포하면, Claude Code는 해당 파일의 서버와 제공된 서버를 함께 로드하며, 둘 다 이름을 정의할 때 파일의 항목이 우선합니다.
* 제공된 서버는 [`strictPluginOnlyCustomization`](/docs/ko/settings-reference#strictpluginonlycustomization)이 `mcp` 표면을 잠글 때 계속 로드됩니다.
* `deniedMcpServers`는 사용자 자신의 설정의 항목을 포함하여 제공된 서버에 적용되므로, 사용자는 자신을 위해 하나를 차단할 수 있습니다. 제공된 서버는 `allowedMcpServers` 항목이 필요하지 않습니다.

`managed-mcp.json`을 배포하지 않은 경우, 실행별 플래그는 의미를 유지합니다:

* 사용자가 동일한 이름으로 `--mcp-config`와 함께 전달하는 서버는 해당 실행에 대해 제공된 서버를 대체하며 `allowedMcpServers`에 대해 검사됩니다.
* `--strict-mcp-config`는 제공된 서버를 다른 모든 구성된 서버와 함께 제외합니다.

`managed-mcp.json`이 배포된 경우, 두 플래그 모두 [managed-mcp.json을 사용한 독점 제어](#exclusive-control-with-managed-mcp-json)에서 설명하는 대로 동작합니다.

<h3 id="what-users-can-see-and-change">
  사용자가 보고 변경할 수 있는 것
</h3>

사용자는 제공된 서버를 편집하거나 제거할 수 없습니다:

* `claude mcp remove`는 서버가 조직에서 제공된다고 보고합니다.
* `managed-mcp.json`을 배포하지 않은 경우, 사용자가 동일한 이름으로 추가하는 항목은 저장되지만 귀사의 항목이 있는 동안에는 사용되지 않습니다.
* 사용자는 여전히 [`/mcp`](/docs/ko/mcp#disable-a-server-without-removing-it)에서 제공된 서버를 자신을 위해 끌 수 있으며, 이는 **Managed MCPs** 아래에 제공된 서버를 나열합니다.

`claude mcp get` 및 `/mcp`는 제공된 서버의 URL을 호스트만으로 표시합니다(예: `https://mcp.example.com/…`). `claude mcp get`은 헤더 이름을 값 없이 표시합니다.

<h3 id="where-managedmcpservers-applies">
  `managedMcpServers`가 적용되는 위치
</h3>

Claude Code는 [Claude Code가 관리되는 소스를 결합하는 방식](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)에서 선택하는 관리되는 소스에서 `managedMcpServers`를 읽습니다. 해당 소스가 [`managedSourcesBehavior`](/docs/ko/settings-reference#managedsourcesbehavior)를 `"merge"`로 설정하면, Claude Code는 대신 모든 관리자 소스의 서버를 제공하며, 두 소스가 동일한 이름을 정의할 때 더 높은 순위의 소스의 항목이 전체적으로 적용됩니다. 사용자 쓰기 가능 HKCU 레지스트리, [임베딩 호스트가 제공하는 부모 설정](/docs/ko/managed-settings#parent-settings-from-embedding-hosts), 또는 사용자, 프로젝트 또는 로컬 설정 파일에서는 절대 키를 읽지 않으며, 경고와 함께 키를 삭제합니다.

Claude Code는 타사 배포의 Claude Desktop 앱의 Code 탭이나 앱의 Cowork 세션에서 키를 읽지 않습니다. Claude Desktop이 해당 세션의 MCP 서버를 자체적으로 제공하고 잠그기 때문입니다. `/status` 및 `claude doctor`는 관리되는 설정이 해당 위치에 키를 전달할 때 그렇게 말합니다.

<h3 id="when-provided-servers-connect">
  제공된 서버가 연결되는 시기
</h3>

`managedMcpServers`가 서버 관리 설정을 통해 도착할 때, 그 타이밍은 [가져오기 및 캐싱 동작](/docs/ko/server-managed-settings#fetch-and-caching-behavior)을 따릅니다:

* 캐시된 설정이 있는 머신에서, Claude Code는 서버가 세션에 대한 설정을 확인할 때까지 이 키의 캐시된 복사본을 보류하고, MCP 서버를 로드하기 전에 해당 확인을 기다립니다. 확인이 실패하면, 세션은 제공된 서버 없이 계속되며 `/status`는 이들이 보류되었다고 말합니다.
* 머신의 첫 번째 시작에서, 아직 캐시된 것이 없으면, 설정이 도착하기 전에 시작하는 대화형 세션은 제공된 서버를 도착하는 즉시 연결하고, 이미 시작한 `claude -p` 실행은 이들 없이 완료할 수 있습니다.

[게이트웨이 로그인](/docs/ko/claude-apps-gateway-config#precedence-with-other-managed-sources)을 사용하면, Claude Code는 세션이 시작되기 전에 정책을 로드하므로, 어느 경우든 제공된 서버를 지연하거나 건너뛰지 않습니다.

이미 실행 중인 대화형 세션은 키에 대한 편집을 적용합니다:

* **서버 추가**: Claude Code는 업데이트된 설정이 도착할 때 이를 연결하며, 재시작이 필요하지 않습니다.
* **서버의 항목 변경**: 해당 세션은 새 정의로 이를 다시 연결합니다.
* **서버 제거**: 실행 중인 대화형 세션은 변경된 설정을 읽으면 이를 연결 해제합니다. 비대화형(`-p`) 실행은 종료될 때까지 이를 유지합니다.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  허용 목록 및 거부 목록을 사용한 정책 기반 제어
</h2>

허용 목록 및 거부 목록은 구성된 서버 중 로드할 수 있는 서버를 필터링합니다. 이들은 레지스트리가 아닙니다. 서버는 사용자, 플러그인 또는 조직에 의해 추가되어야 두 목록 중 하나가 적용됩니다.

조직이 `managedMcpServers`를 통해 제공하는 서버는 허용 목록 항목 없이 로드되며, [서버 평가 방식](#how-a-server-is-evaluated)에서 `managed-mcp.json` 서버를 다룹니다. 거부 목록은 인프로세스 `type: "sdk"` 항목을 제외한 모든 서버에 적용됩니다.

서버를 사용자에게 배포하려면 [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json) 또는 [`managedMcpServers`](#provide-servers-through-managed-settings)를 사용합니다. 두 목록 모두 [`--mcp-config` CLI 플래그](/docs/ko/cli-reference#cli-flags)로 전달된 서버를 필터링하며, 인프로세스 `type: "sdk"` 항목을 제외합니다. `--strict-mcp-config`는 로드되는 구성 파일을 제한하며 두 목록을 우회하지 않습니다.

허용 목록을 권위 있게 만들려면 [관리 설정 소스](/docs/ko/admin-setup#decide-how-settings-reach-devices)(예: 서버 관리 설정 또는 배포된 `managed-settings.json` 파일)에서 `allowedMcpServers`와 `allowManagedMcpServersOnly: true`를 함께 설정합니다.

잠금은 모든 관리자 제어 관리 소스에서 적용되므로 배포된 파일의 잠금은 MCP를 언급하지 않는 서버 관리 설정도 사용 중일 때 적용됩니다. 잠금이 켜져 있는 동안 관리 허용 목록은 하나를 설정하는 가장 높은 순위의 관리 소스에서 옵니다. 소스 전체에서 잠금 및 허용 목록을 읽으려면 Claude Code v2.1.273 이상이 필요합니다.

[허용 목록을 관리 설정만으로 제한](#restrict-the-allowlist-to-managed-settings-only)에서 구성을 보여줍니다.

`allowManagedMcpServersOnly`가 없으면 사용자의 `~/.claude/settings.json`을 포함한 모든 설정 범위의 허용 목록이 병합되므로 사용자가 허용 목록이 허용하는 범위를 확대할 수 있습니다. 거부 목록은 범위에 관계없이 병합됩니다.

<Note>
  `allowManagedMcpServersOnly`는 `allowManagedPermissionRulesOnly`와 별개이며, 후자는 [권한 규칙](/docs/ko/permissions#managed-settings)만 잠급니다. 해당 플래그를 설정해도 MCP 허용 목록은 적용되지 않습니다.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  URL, 명령 또는 이름으로 서버 일치
</h3>

`allowedMcpServers` 및 `deniedMcpServers`는 항목 목록입니다. 각 항목은 URL, 명령 또는 이름으로 서버를 식별하는 단일 키를 가진 객체입니다.

| 키               | 일치 대상                                     | 사용 대상                 |
| :-------------- | :---------------------------------------- | :-------------------- |
| `serverUrl`     | 원격 서버 URL, 정확하거나 `*` 와일드카드 포함             | HTTP 및 SSE 서버         |
| `serverCommand` | stdio 서버를 시작하는 정확한 명령 및 인수                | Stdio 서버              |
| `serverName`    | 사용자가 할당한 레이블. 정확한 일치만 가능하며 와일드카드는 확장되지 않음 | 두 유형 모두, 하지만 아래 경고 참조 |

`allowedMcpServers`를 설정하지 않는 것은 빈 배열로 설정하는 것과 다릅니다.

| 설정                  | 설정하지 않음 (기본값) | 빈 배열 `[]`                                      | 채워짐                                                  |
| :------------------ | :------------ | :--------------------------------------------- | :--------------------------------------------------- |
| `allowedMcpServers` | 모든 서버 허용      | [조직 자체](#how-a-server-is-evaluated)를 제외한 서버 없음 | [조직 자체](#how-a-server-is-evaluated)를 제외한 일치하는 서버만 허용 |
| `deniedMcpServers`  | 차단된 서버 없음     | 차단된 서버 없음                                      | 일치하는 서버 차단                                           |

관리 설정의 잘못된 항목에 대해서는 [관리 설정의 잘못된 항목](/docs/ko/managed-settings#invalid-entries-in-managed-settings)을 참조합니다.

<Warning>
  두 목록 중 하나의 `serverName` 항목은 보안 제어가 아닙니다. 이름은 `claude mcp add`를 실행하거나 구성 파일을 편집할 때 사용자가 할당하는 레이블이지 기본 서버가 아니므로 사용자는 모든 서버를 `github`라고 부를 수 있습니다. claude.ai 커넥터의 경우 이름은 claude.ai에서 반환하는 표시 이름이며 변경될 수 있습니다. 실제로 실행되는 서버를 적용하려면 `serverCommand` 또는 `serverUrl` 항목을 추가합니다.
</Warning>

`serverName` 검증은 두 목록 간에 다릅니다.

* `deniedMcpServers`에서 `serverName`은 선행 또는 후행 공백이 없는 비어 있지 않은 모든 문자열을 허용하므로 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 표시 이름으로 차단할 수 있습니다. 예를 들어 `{ "serverName": "claude.ai Slack" }`은 Slack 커넥터를 차단합니다. 거부가 이름 변경에 강력해야 하거나 커넥터 이름이 충돌하여 ` (N)` 접미사를 얻을 때는 `serverUrl` 항목을 선호합니다.
* `allowedMcpServers`에서 `serverName`은 문자, 숫자, 하이픈 및 밑줄로 제한됩니다. Claude Code가 자체적으로 가져오는 claude.ai 커넥터를 허용 목록에 추가하려면 `serverUrl`을 사용합니다. 클라우드 호스트가 자체 호스팅 세션에 제공하는 커넥터의 경우 대신 [커넥터 트래픽이 네트워크를 떠남](/docs/ko/self-hosted-environments-deploy#connector-traffic-leaves-your-network)에 나열된 항목을 사용합니다.

Claude Code가 자체적으로 가져오는 모든 claude.ai 커넥터를 끄려면 [`disableClaudeAiConnectors`](/docs/ko/mcp#disable-claude-ai-connectors)를 참조합니다.

<h3 id="how-a-server-is-evaluated">
  서버 평가 방식
</h3>

서버를 로드하기 전에 `managed-mcp.json`의 서버를 포함하여 Claude Code는 아래의 세 가지 검사를 순서대로 실행합니다. 사용자가 서버를 다시 연결하거나 `/mcp`에서 비활성화된 서버를 다시 켤 때 다시 실행합니다. 인프로세스 `type: "sdk"` 서버는 [세션을 시작한 앱이 등록](/docs/ko/mcp#how-connectors-reach-claude-code)하며 세 가지 모두 건너뜁니다.

1. **목록 병합.** 모든 설정 범위의 허용 목록 및 거부 목록 항목이 하나의 허용 목록과 하나의 거부 목록으로 결합됩니다. `allowManagedMcpServersOnly`가 `true`일 때 관리 허용 목록만 유지됩니다. 거부 목록은 항상 모든 범위에서 병합됩니다. 둘 이상의 관리 소스가 있을 때 [모든 관리 소스에서 읽은 키](/docs/ko/managed-settings#keys-read-from-every-admin-source)에서 관리 범위의 목록을 제공하는 소스를 설명합니다.
2. **거부 목록 확인.** URL, 명령 또는 이름으로 거부 목록 항목과 일치하는 서버는 차단됩니다. 거부 목록 일치를 무시하는 것은 없습니다.
3. **허용 목록 확인.** `allowedMcpServers`가 어디에도 설정되지 않으면 거부 목록을 통과한 모든 서버가 로드됩니다. 설정되면 서버가 일치해야 하는 것은 아래 표에 표시된 유형에 따라 다릅니다.

   조직 자체의 서버는 이 검사를 건너뜁니다. 모든 `managedMcpServers` 항목과 값에 `${VAR}` 확장을 사용하지 않는 모든 `managed-mcp.json` 항목입니다. Chrome의 Claude, Claude Code가 실행 중인 VS Code 또는 JetBrains IDE에 연결하는 `ide` 서버, CLI 자체가 구성하는 서버와 같은 기본 제공 서버도 건너뜁니다.

   명령, 인수, `env`, URL 또는 헤더에서 `${VAR}` 확장을 사용하는 `managed-mcp.json` 서버는 여전히 확인되며, 사용자, 플러그인, `--mcp-config` 또는 claude.ai가 추가하는 모든 서버도 확인됩니다.

| 서버 유형            | 일치할 때 허용됨                                                                 |
| :--------------- | :------------------------------------------------------------------------ |
| 원격 (HTTP 또는 SSE) | `serverUrl` 항목. `serverName` 일치는 허용 목록에 `serverUrl` 항목이 없을 때만 계산됨         |
| Stdio            | `serverCommand` 항목. `serverName` 일치는 허용 목록에 `serverCommand` 항목이 없을 때만 계산됨 |

이러한 검사 내에서 세 가지 일치 규칙이 적용됩니다.

* **명령은 정확하게 일치합니다.** 모든 인수, 순서대로. `["npx", "-y", "server"]`는 `["npx", "server"]` 또는 `["npx", "-y", "server", "--flag"]`와 일치하지 않습니다.
* **`serverCommand` 및 `serverUrl` 값은 일치 전에 확장됩니다.** 정책 항목과 서버의 구성된 값 모두 [`${VAR}` 및 `${VAR:-default}` 확장](/docs/ko/mcp#environment-variable-expansion-in-mcp-json)을 거치므로 `["${HOME}/bin/server"]`로 작성된 항목은 동일한 참조 또는 확장된 경로를 사용하는 서버 구성과 일치합니다. Windows에서는 `${HOME}` 대신 `${USERPROFILE}`과 같이 설정된 환경 변수를 참조합니다. `serverName` 값은 문자 그대로 일치하며 절대 확장되지 않습니다. 두 쪽은 다른 환경을 읽습니다. [정책 항목 확장 방식](#how-policy-entries-expand)에서 어느 것이고 허용 목록 및 거부 목록 항목이 어떻게 다른지 다룹니다.
* **URL은 패턴의 어디든 `*` 와일드카드를 지원합니다.** 스키마 포함. 호스트명 일치는 대소문자를 구분하지 않으며 후행 FQDN 점을 무시하므로 `https://Mcp.Example.com/*`은 `https://mcp.example.com/api`와 일치합니다. 경로는 대소문자를 구분합니다.

| 패턴                          | 허용                                     |
| :-------------------------- | :------------------------------------- |
| `https://mcp.example.com/*` | 특정 도메인의 모든 경로                          |
| `https://mcp.example.com`   | 또한 해당 도메인의 모든 경로. 경로가 없는 패턴은 모든 경로와 일치 |
| `https://*.example.com/*`   | `example.com`의 모든 하위 도메인               |
| `http://localhost:*/*`      | localhost의 모든 포트                       |
| `*://mcp.example.com/*`     | 특정 도메인으로의 모든 스키마                       |

<h4 id="how-policy-entries-expand">
  정책 항목 확장 방식
</h4>

서버의 구성된 값은 `.mcp.json`의 나머지와 같이 라이브 프로세스 환경에서 확장됩니다. 정책 항목은 고정된 환경에서 확장되므로 프로젝트 또는 사용자 설정 파일에 의해 설정된 변수가 허용 목록 항목의 의미를 변경할 수 없습니다. 정책 항목은 여전히 참조하는 모든 변수에 대해 시작 셸의 값에 따라 달라지므로 적용에 의존하는 항목에는 리터럴 URL 및 명령을 사용합니다.

| 항목 목록               | 확장 대상                                                                                           | URL 항목의 스키마, 호스트 또는 경로 범위를 변경하는 확장 |
| ------------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------- |
| `allowedMcpServers` | Claude Code가 시작한 환경, 더하기 관리 설정의 `env` 값                                                         | Claude Code는 항목을 무시합니다             |
| `deniedMcpServers`  | 동일하며, 시작 값이 없고 `:-default`가 없는 변수는 저장소 외부의 설정 파일(예: 사용자 또는 관리 설정)에서 채워지며, 이는 항목이 일치하는 범위를 확대합니다 | 항목은 여전히 일치합니다                      |

Claude Code v2.1.219 이상이 필요합니다.

<h3 id="example-configuration">
  예제 구성
</h3>

아래 구성은 거부 목록이 있는 하드 허용 목록을 설정합니다. 강조된 줄은 나머지 목록의 평가 방식을 변경하며, 블록 후의 설명은 각각을 설명합니다.

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

* **3번 줄**: 첫 번째 `serverUrl` 항목. 하나가 존재하면 모든 원격 서버는 URL 패턴과 일치해야 하므로 사용자는 허용된 이름을 제공하여 나열되지 않은 원격 서버를 얻을 수 없습니다.
* **5번 줄**: 첫 번째 `serverCommand` 항목. stdio 서버에 대해 동일한 효과이므로 모든 로컬 서버는 나열된 명령과 정확하게 일치해야 합니다.
* **11번 줄**: 거부 목록의 `serverName` 항목. 거부 목록 항목은 항상 적용되므로 `dangerous-server`라는 모든 서버는 URL 또는 명령에 관계없이 차단됩니다.

이 허용 목록의 `serverName` 항목은 두 전송 유형 모두 이미 더 엄격한 항목이 있으므로 아무것도 일치하지 않습니다.

아래 아코디언은 다른 허용 목록 및 거부 목록 조합에 대해 서버가 평가되는 방식을 설명합니다.

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
  | 모든 명령의 Stdio 서버                                 | 차단됨: 일치할 이름 또는 명령 항목 없음 |
</Accordion>

<Accordion title="명령만 허용 목록">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | 서버                                            | 결과                |
  | :-------------------------------------------- | :---------------- |
  | `["npx", "-y", "approved-package"]`의 Stdio 서버 | 허용됨: 명령과 일치       |
  | `["node", "server.js"]`의 Stdio 서버             | 차단됨: 명령과 일치하지 않음  |
  | `my-api`라는 HTTP 서버                            | 차단됨: 일치할 이름 항목 없음 |
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

  | 서버                                                            | 결과                                     |
  | :------------------------------------------------------------ | :------------------------------------- |
  | `["npx", "-y", "approved-package"]`의 `local-tool`이라는 Stdio 서버 | 허용됨: 명령과 일치                            |
  | `["node", "server.js"]`의 `local-tool`이라는 Stdio 서버             | 차단됨: 명령 항목이 존재하지만 일치하지 않음              |
  | `["node", "server.js"]`의 `github`라는 Stdio 서버                  | 차단됨: stdio 서버는 명령 항목이 존재할 때 명령과 일치해야 함 |
  | `github`라는 HTTP 서버                                            | 허용됨: 이름과 일치                            |
  | `other-api`라는 HTTP 서버                                         | 차단됨: 이름이 일치하지 않음                       |
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

  | 서버                                 | 결과               |
  | :--------------------------------- | :--------------- |
  | 모든 명령의 `github`라는 Stdio 서버         | 허용됨: 명령 제한 없음    |
  | 모든 명령의 `internal-tool`이라는 Stdio 서버 | 허용됨: 명령 제한 없음    |
  | `github`라는 HTTP 서버                 | 허용됨: 이름과 일치      |
  | `other`라는 모든 서버                    | 차단됨: 이름이 일치하지 않음 |
</Accordion>

<Accordion title="거부 목록 무시가 있는 허용 목록">
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
  허용 목록을 관리 설정만으로 제한
</h3>

관리 허용 목록이 유일하게 적용되도록 하려면 관리 설정 파일에서 `allowManagedMcpServersOnly`를 설정합니다.

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

`allowManagedMcpServersOnly`가 `true`일 때 사용자, 프로젝트 및 로컬 설정의 허용 목록은 무시됩니다. 거부 목록은 여전히 모든 설정 범위에서 병합되므로 사용자는 항상 자신을 위해 서버를 차단할 수 있습니다.

<h2 id="how-restrictions-appear-to-users">
  제한 사항이 사용자에게 표시되는 방식
</h2>

`managed-mcp.json`이 배포되고 세션에 `--mcp-config` 서버도 있을 때 시작 시 사용자가 보는 내용은 [managed-mcp.json을 사용한 독점 제어](#exclusive-control-with-managed-mcp-json)를 참조하십시오. 이 표를 사용하여 다른 보고서를 인식하고 변경 사항을 배포하기 전에 사용자에게 예상되는 사항을 알려주십시오:

| 제한 사항                                                      | 사용자가 보는 내용                                                                                                                   |
| :--------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`이 있고 사용자가 `claude mcp add`를 실행함          | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`                   |
| 서버가 거부 목록에 있고 사용자가 `claude mcp add`를 실행함                   | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                                          |
| 서버가 허용 목록에 없고 사용자가 `claude mcp add`를 실행함                   | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                                           |
| 사용자가 `managedMcpServers`의 서버에서 `claude mcp remove`를 실행함    | `MCP server "<name>" is provided by your organization (managed settings) and cannot be removed locally.`                     |
| 이전에 구성된 서버가 이제 정책에 의해 차단됨                                  | 서버가 `/mcp` 및 `claude mcp list`에서 사라짐                                                                                         |
| 세션이 실행 중인 동안 서버가 차단되고 사용자가 **다시 연결**을 선택하거나 `/mcp`에서 다시 켜짐 | [`MCP server <name> is blocked by enterprise managed policy`](/docs/ko/errors#mcp-server-is-blocked-by-enterprise-managed-policy) |

서버가 조용히 사라질 때 사용자는 정책이 이유라는 신호를 받지 못하므로 새로운 제한 사항을 배포할 때 영향을 받는 사용자에게 어떤 서버가 차단되었는지 알려주십시오.

<h2 id="monitor-mcp-usage">
  MCP 사용 모니터링
</h2>

[OpenTelemetry 내보내기](/docs/ko/monitoring-usage)가 구성되면, Claude Code는 사용자가 호출하는 MCP 서버 및 도구를 기록할 수 있습니다. `OTEL_LOG_TOOL_DETAILS=1`을 설정하여 도구 이벤트에 MCP 서버 및 도구 이름을 포함한 다음, 수집기에서 집계하여 사용자가 실제로 연결하는 서버를 확인합니다. 내보내기를 설정하고 전체 이벤트 스키마는 [모니터링](/docs/ko/monitoring-usage)을 참조하십시오.

<h2 id="configuration-summary">
  구성 요약
</h2>

이 페이지에서 다루는 모든 파일 및 설정, 제어 항목 및 전달 방법:

| 표면                           | 제어 항목                                                                                                                                                                                | 위치                                                                                                                                   | 전달 방법                                                                                                                               |
| :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | 고정 서버 집합, 독점 제어                                                                                                                                                                      | 시스템 경로: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, 또는 `C:\Program Files\ClaudeCode\`                           | MDM, GPO, 플릿 관리, 또는 관리자 권한이 있는 모든 프로세스. 서버 관리 설정을 통해 설정할 수 없음                                                                       |
| `managedMcpServers`          | 모든 사용자에게 자신의 서버와 함께 제공되는 원격 서버                                                                                                                                                       | 관리형 설정 소스만; 설정은 다른 곳에서 효과 없음                                                                                                         | [관리형 설정 소스](/docs/ko/admin-setup#decide-how-settings-reach-devices): 서버 관리 설정, 게이트웨이 정책, `managed-settings.json`, MDM 프로필, 또는 HKLM 레지스트리 |
| `allowedMcpServers`          | 허용된 서버의 허용 목록                                                                                                                                                                        | 모든 [설정 범위](/docs/ko/settings#where-settings-live); [서버 평가 방식](#how-a-server-is-evaluated)에서 여러 범위 및 관리형 소스의 목록이 결합되는 방식을 설명               | 적용을 위해, [관리형 설정 소스](/docs/ko/admin-setup#decide-how-settings-reach-devices): 서버 관리 설정, `managed-settings.json`, MDM 프로필, 또는 레지스트리        |
| `deniedMcpServers`           | 차단된 서버의 거부 목록                                                                                                                                                                        | 모든 설정 범위; [서버 평가 방식](#how-a-server-is-evaluated)에서 여러 범위 및 관리형 소스의 목록이 결합되는 방식을 설명                                                   | `allowedMcpServers`와 동일                                                                                                             |
| `allowManagedMcpServersOnly` | 허용 목록을 관리형 소스만으로 잠금                                                                                                                                                                  | 관리형 설정 소스만; [모든 관리형 소스에서 읽은 키](/docs/ko/managed-settings#keys-read-from-every-admin-source)에서 어떤 관리형 소스가 이를 켤 수 있는지 설명. 설정은 다른 범위에서 효과 없음 | `allowedMcpServers`와 동일                                                                                                             |
| `allowAllClaudeAiMcps`       | Claude Code가 자체적으로 가져오는 claude.ai 커넥터를 `managed-mcp.json`과 함께 로드. [클라우드 세션을 실행하는 호스트의 `managed-mcp.json`은 여전히 해당 세션의 커넥터를 억제](#allow-claude-ai-connectors-alongside-the-managed-set) | 관리형 설정 소스만; 설정은 다른 곳에서 효과 없음                                                                                                         | `allowedMcpServers`와 동일                                                                                                             |

<h2 id="related-resources">
  관련 리소스
</h2>

* [적용할 항목 결정](/docs/ko/admin-setup#decide-what-to-enforce): 권한 규칙, 샌드박싱 및 다른 관리 제어와 함께 MCP 제한
* [MCP를 통해 Claude Code를 도구에 연결](/docs/ko/mcp): 전송, 범위 및 인증을 포함한 전체 MCP 참조
* [설정](/docs/ko/settings): 설정 계층 구조 및 관리형 설정이 우선하는 방식
* [서버 관리 설정](/docs/ko/server-managed-settings): Claude.ai 관리 콘솔에서 `allowedMcpServers` 및 `deniedMcpServers` 전달
* [보안](/docs/ko/security): 이러한 제어가 방어하는 위협 모델
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, 시트 관리 및 롤아웃 플레이북
