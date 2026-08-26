> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# MCP를 통해 Claude Code를 도구에 연결하기

> Model Context Protocol을 사용하여 Claude Code를 도구에 연결하는 방법을 알아봅니다.

Claude Code는 AI 도구 통합을 위한 오픈 소스 표준인 [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction)를 통해 수백 개의 외부 도구 및 데이터 소스에 연결할 수 있습니다. MCP 서버는 Claude Code에 도구, 데이터베이스 및 API에 대한 액세스를 제공합니다.

다른 도구(예: 이슈 추적기 또는 모니터링 대시보드)에서 채팅으로 데이터를 복사하는 자신을 발견할 때 서버를 연결하세요. 연결되면 Claude는 붙여넣은 내용에서 작업하는 대신 해당 시스템을 직접 읽고 작동할 수 있습니다.

첫 번째 서버를 연결하는 경우 단계별 안내를 위해 [MCP 빠른 시작](/docs/ko/mcp-quickstart)으로 시작하세요. 이 페이지는 전체 참고 자료입니다.

<h2 id="what-you-can-do-with-mcp">
  MCP로 할 수 있는 것
</h2>

MCP 서버가 연결되면 Claude Code에 다음을 요청할 수 있습니다:

* **이슈 추적기에서 기능 구현**: "JIRA 이슈 ENG-4521에 설명된 기능을 추가하고 GitHub에서 PR을 생성하세요."
* **모니터링 데이터 분석**: "Sentry와 Statsig을 확인하여 ENG-4521에 설명된 기능의 사용량을 확인하세요."
* **데이터베이스 쿼리**: "PostgreSQL 데이터베이스를 기반으로 기능 ENG-4521을 사용한 무작위 사용자 10명의 이메일을 찾으세요."
* **디자인 통합**: "Slack에 게시된 새로운 Figma 디자인을 기반으로 표준 이메일 템플릿을 업데이트하세요."
* **워크플로우 자동화**: "이 10명의 사용자를 새로운 기능에 대한 피드백 세션에 초대하는 Gmail 초안을 생성하세요."
* **외부 이벤트에 반응**: MCP 서버는 [채널](/docs/ko/channels)로도 작동할 수 있으며, 세션에 메시지를 푸시하므로 Claude는 자리를 비운 동안 Telegram 메시지, Discord 채팅 또는 webhook 이벤트에 반응할 수 있습니다.

<h2 id="find-and-build-mcp-servers">
  MCP 서버 찾기 및 구축
</h2>

[Anthropic Directory](https://claude.ai/directory)에서 검토된 커넥터를 찾아보세요. Directory 커넥터는 Claude Code와 동일한 MCP 인프라를 사용하므로 `claude mcp add`를 사용하여 여기에 나열된 모든 원격 서버를 추가할 수 있습니다.

<Warning>
  연결하기 전에 각 서버를 신뢰할 수 있는지 확인하세요. 외부 콘텐츠를 가져오는 서버는 [프롬프트 주입 위험](/docs/ko/security#protect-against-prompt-injection)에 노출될 수 있습니다.
</Warning>

자신만의 서버를 구축하려면 프로토콜 기본 사항에 대한 [MCP 서버 가이드](https://modelcontextprotocol.io/docs/develop/build-server)와 인증, 테스트 및 Directory 제출에 대한 [Claude 커넥터 구축 문서](https://claude.com/docs/connectors/building)를 참조하세요.

공식 [`mcp-server-dev` 플러그인](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/mcp-server-dev)을 사용하여 Claude가 서버를 스캐폴드하도록 할 수도 있습니다.

<Steps>
  <Step title="플러그인 설치">
    Claude Code 세션에서 다음을 실행하세요:

    ```
    /plugin install mcp-server-dev@claude-plugins-official
    ```

    Claude Code가 마켓플레이스를 찾을 수 없다고 보고하면 먼저 `/plugin marketplace add anthropics/claude-plugins-official`을 실행한 다음 설치를 다시 시도하세요. 설치가 완료되면 `/reload-plugins`를 실행하여 현재 세션에서 활성화하세요.
  </Step>

  <Step title="빌드 스킬 실행">
    ```
    /mcp-server-dev:build-mcp-server
    ```

    Claude가 사용 사례에 대해 묻고 원격 HTTP 또는 로컬 stdio 서버를 스캐폴드합니다.
  </Step>
</Steps>

<h2 id="installing-mcp-servers">
  MCP 서버 설치
</h2>

MCP 서버는 필요에 따라 여러 가지 방식으로 구성할 수 있습니다:

<h3 id="option-1-add-a-remote-http-server">
  옵션 1: 원격 HTTP 서버 추가
</h3>

HTTP 서버는 원격 MCP 서버에 연결하기 위한 권장 옵션입니다. 이는 클라우드 기반 서비스에 가장 널리 지원되는 전송 방식입니다.

```bash theme={null}
# 기본 구문
claude mcp add --transport http <name> <url>

# 실제 예: Notion에 연결
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Bearer 토큰을 사용한 예
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

`.mcp.json`, `~/.claude.json` 또는 `claude mcp add-json`을 통해 JSON으로 MCP 서버를 구성할 때, `type` 필드는 `http`의 별칭으로 `streamable-http`를 허용합니다. MCP 사양은 이 전송에 대해 `streamable-http`라는 이름을 사용하므로 서버 설명서에서 복사한 구성이 수정 없이 작동합니다.

`url`은 있지만 `type`이 없는 JSON 항목은 구성 오류입니다. Claude Code는 `type`이 없는 항목을 stdio 서버로 읽기 때문입니다. Claude Code는 해당 서버를 건너뛰고 `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`를 보고합니다. v2.1.202 이전에는 Claude Code가 이 잘못된 구성을 `command: expected string, received undefined`로 보고했습니다.

<h3 id="option-2-add-a-remote-sse-server">
  옵션 2: 원격 SSE 서버 추가
</h3>

<Warning>
  SSE (Server-Sent Events) 전송은 더 이상 사용되지 않습니다. 가능한 경우 HTTP 서버를 사용하세요.
</Warning>

```bash theme={null}
# 기본 구문
claude mcp add --transport sse <name> <url>

# 실제 예: Asana에 연결
claude mcp add --transport sse asana https://mcp.asana.com/sse

# 인증 헤더를 사용한 예
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  옵션 3: 로컬 stdio 서버 추가
</h3>

Stdio 서버는 컴퓨터에서 로컬 프로세스로 실행됩니다. 시스템에 직접 액세스하거나 사용자 정의 스크립트가 필요한 도구에 이상적입니다.

Claude Code는 생성된 서버의 환경에서 `CLAUDE_PROJECT_DIR`을 프로젝트 루트로 설정하므로 서버는 작업 디렉터리에 의존하지 않고 프로젝트 상대 경로를 확인할 수 있습니다. 이는 hooks가 `CLAUDE_PROJECT_DIR` 변수에서 받는 것과 동일한 디렉터리입니다. 서버 프로세스 내에서 읽으세요. 예를 들어 Node에서는 `process.env.CLAUDE_PROJECT_DIR` 또는 Python에서는 `os.environ["CLAUDE_PROJECT_DIR"]`입니다.

`CLAUDE_PROJECT_DIR`은 안정적인 프로젝트 루트이며 세션 중에 작업 디렉터리를 추가하거나 제거할 때 변경되지 않습니다. 자신의 파일 시스템 액세스를 허용된 디렉터리 집합으로 제한하는 서버는 대신 MCP `roots/list` 요청을 구현해야 합니다. Claude Code는 `roots/list`에 세션의 시작 디렉터리와 `--add-dir`, `/add-dir` 또는 `additionalDirectories` 설정으로 부여한 모든 [추가 작업 디렉터리](/docs/ko/permissions#working-directories)로 응답합니다. Claude Code는 해당 집합이 변경될 때 `notifications/roots/list_changed`를 보냅니다. v2.1.203 이전에는 `roots/list`가 시작 디렉터리만 반환했고 Claude Code는 `notifications/roots/list_changed`를 보내지 않았습니다.

이 변수는 Claude Code 자체의 환경이 아닌 서버의 환경에 설정되므로 프로젝트 또는 사용자 범위의 `.mcp.json` `command` 또는 `args`에서 `${VAR}` 확장을 통해 참조하려면 `${CLAUDE_PROJECT_DIR:-.}`와 같은 기본값이 필요합니다. 플러그인 제공 MCP 구성은 `${CLAUDE_PROJECT_DIR}`을 직접 대체하며 기본값이 필요하지 않습니다.

```bash theme={null}
# 기본 구문
claude mcp add [options] <name> -- <command> [args...]

# 실제 예: Airtable 서버 추가
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **중요: 서버 인수를 `--`로 구분**

  Stdio 서버의 경우, `--` (이중 대시)는 Claude의 자체 옵션(예: `--transport`, `--env`, `--scope`)과 서버를 실행하는 명령 및 인수를 구분합니다. `--` 이후의 모든 것은 서버에 그대로 전달됩니다.

  예를 들어:

  * `claude mcp add --transport stdio myserver -- npx server` → `npx server` 실행
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → 환경에서 `KEY=value`를 사용하여 `python server.py --port 8080` 실행

  `--`가 없으면 Claude Code는 위의 `--port`와 같은 서버의 플래그를 자신의 옵션으로 구문 분석하려고 시도합니다.

  `--env`는 여러 `KEY=value` 쌍을 허용합니다. 서버 이름이 `--env` 직후에 오면 CLI는 이름을 다른 쌍으로 읽고 거부하므로 위의 예와 같이 `--env`와 서버 이름 사이에 최소한 하나의 다른 옵션을 배치하세요.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  옵션 4: 원격 WebSocket 서버 추가
</h3>

WebSocket 서버는 지속적인 양방향 연결을 유지하므로 Claude에 예고 없이 이벤트를 푸시하는 원격 MCP 서버에 적합합니다. 서버가 요청에만 응답하는 경우 HTTP를 대신 사용하세요. HTTP는 OAuth 및 `claude mcp add --transport` 플래그를 지원하지만 WebSocket은 둘 다 지원하지 않습니다.

`.mcp.json` 또는 `claude mcp add-json`으로 WebSocket 서버를 구성하세요:

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

`type: "ws"` 항목은 `http`와 동일한 `url`, `headers`, `headersHelper`, `timeout` 및 `alwaysLoad` 필드를 허용합니다. 인증은 헤더 전용이므로 `headers`에 정적 토큰을 전달하거나 [`headersHelper`](#use-dynamic-headers-for-custom-authentication)를 사용하여 연결 시 토큰을 생성하세요. `claude mcp add --transport` 플래그는 `ws`를 허용하지 않습니다.

<h3 id="managing-your-servers">
  서버 관리
</h3>

구성한 후에는 다음 명령으로 MCP 서버를 관리할 수 있습니다:

```bash theme={null}
# 구성된 모든 서버 나열
claude mcp list

# 특정 서버의 세부 정보 가져오기
claude mcp get github

# 서버 제거
claude mcp remove github

# (Claude Code 내에서) 서버 상태 확인
/mcp
```

`.mcp.json`의 프로젝트 범위 서버 중 승인을 기다리는 서버는 `claude mcp list`에 `⏸ 승인 대기 중`으로 나타납니다. `claude`를 대화형으로 실행하여 검토하고 승인하세요. `claude mcp get <name>`은 보류 중인 서버를 `⏸ 승인 대기 중`으로 표시하고 거부된 서버를 `✗ 거부됨`으로 표시합니다.

v2.1.196부터 `claude mcp list` 및 `claude mcp get`은 `.mcp.json` 승인을 `claude`를 실행하고 작업 영역 신뢰 대화 상자를 수락하여 작업 영역을 신뢰할 때까지 저장소에 체크인되지 않은 설정 파일에서만 읽습니다. 복제된 저장소는 자신의 서버를 승인할 수 없습니다: 프로젝트의 `.claude/settings.json`에 커밋된 [`enableAllProjectMcpServers` 또는 `enabledMcpjsonServers`](/docs/ko/settings#available-settings)는 신뢰할 수 없는 폴더에서 무시되며, 서버는 연결되고 상태 확인되는 대신 `⏸ 승인 대기 중`으로 유지됩니다.

이러한 소스의 승인은 신뢰할 수 없는 폴더에서도 적용됩니다:

* 사용자 `~/.claude/settings.json`
* 관리되는 설정
* `--settings`로 전달된 설정

추적되지 않은 `.claude/settings.local.json`의 승인도 적용되지만, 해당 폴더 또는 상위 디렉터리에 대한 신뢰 대화 상자를 수락한 후에만 적용됩니다: Claude Code는 git을 실행하여 파일이 추적되는지 확인하며, 신뢰할 수 있는 폴더에서만 해당 확인을 실행합니다. 신뢰한 적이 없는 폴더에서는 파일의 승인이 신뢰 대화 상자를 기다립니다. 단, 폴더가 자신의 구성 홈인 경우는 제외됩니다: 홈 디렉터리 또는 `.claude`를 [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars)으로 설정한 디렉터리입니다. v2.1.207 이전에는 신뢰한 적이 없는 폴더에서 추적되지 않은 `.claude/settings.local.json`이 서버를 승인했습니다.

모든 설정 파일의 `disabledMcpjsonServers` 항목은 여전히 서버를 거부합니다.

`/mcp` 패널은 각 연결된 서버 옆에 도구 개수를 표시하고 도구 기능을 광고하지만 도구를 노출하지 않는 서버에 플래그를 지정합니다.

URL이 비어 있는 원격 서버의 구성은 `/mcp`, `claude mcp list` 및 [`/plugin`](/docs/ko/plugins) 관리자에서 `not configured`로 표시되며, Claude Code는 연결을 시도하지 않습니다. 플러그인은 나중에 구성할 커넥터에 대한 자리 표시자 항목을 포함할 수 있으므로 Claude Code가 오류 또는 설정 문제로 보고하지 않습니다. `/mcp`의 서버 세부 정보 보기에는 `No URL configured for this server`가 표시됩니다. 연결하려면 항목의 `url`을 설정하세요. v2.1.208 이전에는 Claude Code가 빈 `url`을 재연결 프롬프트와 함께 구성 문제로 보고했습니다.

요청이 백그라운드에서 아직 연결 중인 서버의 도구가 필요한 경우 Claude는 해당 서버가 연결될 때까지 기다립니다. [도구 검색](#scale-with-mcp-tool-search)이 활성화되어 있으면 (기본값), 대기는 `ToolSearch` 호출 내에서 발생합니다. Google Cloud의 Agent Platform, 사용자 정의 `ANTHROPIC_BASE_URL` 또는 `ENABLE_TOOL_SEARCH=false`와 같이 도구 검색이 없는 구성에서는 Claude가 대신 `WaitForMcpServers` 도구를 사용합니다.

일부 서버 이름은 Claude Code의 기본 제공 서버용으로 예약되어 있습니다: `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview`, 및 `Claude Browser`. 구성에서 예약된 이름의 서버를 정의하면 Claude Code는 로드 시 이를 건너뛰고 이름을 바꾸도록 요청하는 경고를 표시합니다. `claude mcp add`는 예약된 이름을 오류로 거부합니다.

`Claude Preview` 및 `Claude Browser`는 모두 [Claude Code 데스크톱 앱의 미리보기 창](/docs/ko/desktop#preview-your-app)이 사용하는 기본 제공 서버의 이름입니다. v2.1.205 이전에는 `Claude Browser`가 예약되지 않았으므로 사용자 구성 서버가 해당 이름으로 등록될 수 있었습니다.

<h3 id="dynamic-tool-updates">
  동적 도구 업데이트
</h3>

Claude Code는 MCP `list_changed` 알림을 지원하므로 MCP 서버가 연결을 끊었다가 다시 연결할 필요 없이 사용 가능한 도구, 프롬프트 및 리소스를 동적으로 업데이트할 수 있습니다. MCP 서버가 `list_changed` 알림을 보내면 Claude Code는 해당 서버에서 사용 가능한 기능을 자동으로 새로 고칩니다.

<h3 id="automatic-reconnection">
  자동 재연결
</h3>

HTTP 또는 SSE 서버가 세션 중에 연결이 끊어지면 Claude Code는 지수 백오프를 사용하여 자동으로 재연결합니다: 최대 5번의 시도, 1초 지연으로 시작하여 매번 두 배씩 증가합니다. 서버는 재연결이 진행 중인 동안 `/mcp`에서 보류 중으로 나타납니다. 5번의 실패 시도 후 서버는 실패로 표시되며 `/mcp`에서 수동으로 다시 시도할 수 있습니다. Stdio 서버는 로컬 프로세스이며 자동으로 재연결되지 않습니다.

HTTP 또는 SSE 서버가 시작 시 초기 연결에 실패할 때도 동일한 백오프가 적용됩니다. v2.1.121부터 Claude Code는 5xx 응답, 연결 거부 또는 시간 초과와 같은 일시적 오류에 대해 초기 연결을 최대 3번 재시도한 후, 여전히 연결할 수 없으면 서버를 실패로 표시합니다. 인증 및 찾을 수 없음 오류는 해결하기 위해 구성 변경이 필요하므로 재시도되지 않습니다.

구성된 서버가 연결에 실패하면 Claude Code는 Claude에 어느 서버가 실패했는지와 연결 오류를 알립니다. 이는 일치하는 도구를 찾지 못한 `ToolSearch` 결과를 포함합니다. 따라서 Claude는 응답에서 연결 실패를 보고합니다. [도구 검색](#scale-with-mcp-tool-search)이 필요합니다. 이는 기본적으로 활성화됩니다. 사용자 정의 `ANTHROPIC_BASE_URL`, `ENABLE_TOOL_SEARCH=false` 또는 도구 검색을 지원하지 않는 모델과 같이 도구 검색이 없는 구성에서, 그리고 Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry에서는 Claude Code가 실패한 서버 연결을 Claude에 보고하지 않습니다. v2.1.205 이전에는 Claude Code가 연결 오류를 Claude에 전달하지 않았으며, Claude는 실패한 서버의 도구가 구성되지 않은 것처럼 응답할 수 있었습니다.

v2.1.191부터 성공적인 연결 후 실행되는 기능 검색 요청(예: `tools/list`, `prompts/list`, `resources/list`)도 일시적 네트워크 및 서버 오류를 짧은 백오프로 최대 3번 재시도합니다. 인증 오류, 4xx 응답 및 요청 시간 초과는 재시도되지 않습니다.

<h3 id="push-messages-with-channels">
  채널을 사용한 메시지 푸시
</h3>

MCP 서버는 또한 메시지를 세션에 직접 푸시할 수 있으므로 Claude는 CI 결과, 모니터링 경고 또는 채팅 메시지와 같은 외부 이벤트에 반응할 수 있습니다. 이를 활성화하려면 서버가 `claude/channel` 기능을 선언하고 시작 시 `--channels` 플래그로 옵트인합니다. 공식적으로 지원되는 채널을 사용하려면 [채널](/docs/ko/channels)을 참조하거나, 자신만의 채널을 구축하려면 [채널 참조](/docs/ko/channels-reference)를 참조하세요.

<Tip>
  팁:

  * `-s` 또는 `--scope` 플래그를 사용하여 구성이 저장되는 위치를 지정하세요:
    * `local` (기본값): 현재 프로젝트에서만 사용자에게만 사용 가능. 이전 버전에서는 이 범위를 `project`라고 불렀습니다
    * `project`: `.mcp.json` 파일을 통해 프로젝트의 모든 사람과 공유
    * `user`: 모든 프로젝트에서 사용자에게 사용 가능. 이전 버전에서는 이 범위를 `global`이라고 불렀습니다
  * `-e` 또는 `--env` 플래그로 환경 변수를 설정하세요 (예: `-e KEY=value`)
  * `--transport` 및 `--header` 플래그는 `-t` 및 `-H` 단축형도 허용합니다
  * `MCP_TIMEOUT` 환경 변수를 사용하여 MCP 서버 시작 시간 초과를 구성하세요 (예: `MCP_TIMEOUT=10000 claude`는 10초 시간 초과를 설정)
  * 서버당 도구 실행 시간 초과를 설정하려면 해당 서버의 `.mcp.json` 항목에 밀리초 단위의 `timeout` 필드를 추가하세요. 예를 들어 10분의 경우 `"timeout": 600000`입니다. 이는 해당 서버에만 `MCP_TOOL_TIMEOUT` 환경 변수를 재정의합니다
  * Claude Code는 MCP 도구 출력이 10,000 토큰을 초과할 때 경고를 표시하고 기본적으로 출력을 25,000 토큰으로 제한합니다. 이 제한을 늘리려면 `MAX_MCP_OUTPUT_TOKENS` 환경 변수를 설정하세요 (예: `MAX_MCP_OUTPUT_TOKENS=50000`). 경고 임계값은 고정됩니다. [MCP 출력 제한 및 경고](#mcp-output-limits-and-warnings)를 참조하세요
  * OAuth 2.0 인증이 필요한 원격 서버로 인증하려면 `/mcp`를 사용하세요
</Tip>

서버당 `timeout`은 도구 호출당 하드 월클록 제한이며, 서버의 진행 알림은 이를 연장하지 않습니다. 1000 미만의 값은 무시되고 `MCP_TOOL_TIMEOUT`으로 넘어가거나, 해당 변수가 설정되지 않은 경우 약 28시간의 기본값으로 넘어갑니다. HTTP, SSE 또는 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai) 서버의 경우 서버의 첫 응답 바이트까지 각 요청을 포함하는 요청당 두 번째 타이머도 있습니다. 해당 타이머는 60초이며, 서버당 `timeout` 또는 `MCP_TOOL_TIMEOUT`을 설정하지 않으면 60초 이상으로 설정하면 요청당 타이머가 해당 값으로 올라가고, 더 낮은 값은 단축하지 않으며, 설정되지 않은 `MCP_TOOL_TIMEOUT`의 28시간 기본값은 절대 공급하지 않습니다. Stdio 및 WebSocket 서버에는 요청당 타이머가 없습니다. v2.1.162 이전에는 1000 미만의 값이 1초로 내림되었습니다.

서버당 최소 1000의 `timeout`은 또한 아래에 설명된 유휴 시간 초과의 하한으로 작동합니다: Claude Code는 서버당 `timeout`보다 더 빨리 유휴 상태로 인해 해당 서버의 도구 호출을 중단하지 않습니다. Claude Code v2.1.203 이상이 필요합니다.

MCP 서버에 대한 도구 호출이 유휴 윈도우 동안 응답 및 진행 알림을 보내지 않으면 월클록 제한을 기다리는 대신 오류로 중단됩니다. 유휴 시간 초과에는 Claude Code v2.1.187 이상이 필요합니다. IDE 서버 및 SDK 인프로세스 서버를 제외한 모든 서버 유형에 적용됩니다. 유휴 윈도우는 HTTP, SSE, WebSocket 및 [claude.ai 커넥터](#use-mcp-servers-from-claude-ai) 서버의 경우 기본값 5분, stdio 서버의 경우 30분입니다. v2.1.203 이전에는 stdio 서버가 유휴 시간 초과에서 제외되었습니다.

[`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/ko/env-vars) 환경 변수를 밀리초 단위로 설정하여 유휴 윈도우를 변경하거나, `0`으로 설정하여 확인을 비활성화하세요.

<h3 id="plugin-provided-mcp-servers">
  플러그인 제공 MCP 서버
</h3>

[플러그인](/docs/ko/plugins)은 MCP 서버를 번들로 제공할 수 있으며, 플러그인이 활성화되면 도구 및 통합을 자동으로 제공합니다. 플러그인 MCP 서버는 사용자 구성 서버와 동일하게 작동합니다.

**플러그인 MCP 서버의 작동 방식**:

* 플러그인은 플러그인 루트의 `.mcp.json` 또는 `plugin.json`에 인라인으로 MCP 서버를 정의합니다
* 플러그인이 활성화되면 MCP 서버가 자동으로 시작됩니다
* 플러그인 MCP 도구는 수동으로 구성된 MCP 도구와 함께 나타납니다
* 플러그인 서버는 플러그인 설치를 통해 관리됩니다 (`/mcp` 명령이 아님)

**플러그인 MCP 구성 예**:

플러그인 루트의 `.mcp.json`:

```json theme={null}
{
  "mcpServers": {
    "database-tools": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_URL": "${DB_URL}"
      }
    }
  }
}
```

또는 `plugin.json`에 인라인:

```json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**플러그인 MCP 기능**:

* **자동 라이프사이클**: 세션 시작 시 활성화된 플러그인의 서버가 자동으로 연결됩니다. 세션 중에 플러그인을 활성화하거나 비활성화하면 `/reload-plugins`를 실행하여 MCP 서버를 연결하거나 연결 해제합니다
* **경로 자리 표시자**: `${CLAUDE_PLUGIN_ROOT}`는 플러그인의 설치 디렉터리로 확인되고, `${CLAUDE_PLUGIN_DATA}`는 [지속적인 상태](/docs/ko/plugins-reference#persistent-data-directory) 디렉터리로 확인되며, `${CLAUDE_PROJECT_DIR}`은 안정적인 프로젝트 루트로 확인됩니다. 대체는 다음에 적용됩니다:
  * `stdio` 서버: `command`, `args`, `env`
  * `http`, `sse` 및 `ws` 서버: `url`, `headers` 및 `headersHelper`. v2.1.195 이전에는 `headersHelper`가 자리 표시자를 리터럴 문자열로 전달했습니다
* **사용자 환경 액세스**: 수동으로 구성된 서버와 동일한 환경 변수에 액세스
* **여러 전송 유형**: stdio, SSE, HTTP 및 WebSocket 전송 지원 (전송 지원은 서버에 따라 다를 수 있음)

**플러그인 MCP 서버 보기**:

```bash theme={null}
# Claude Code 내에서 플러그인 서버를 포함한 모든 MCP 서버 보기
/mcp
```

플러그인 서버는 플러그인에서 온 것을 나타내는 표시기와 함께 목록에 나타납니다.

**플러그인 MCP 도구 이름**:

플러그인 번들 MCP 서버의 도구는 호출 가능한 이름에 플러그인 이름과 서버 키를 모두 포함합니다. 전체 형식은 `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`이며, `A-Z`, `a-z`, `0-9`, `_`, `-` 외의 모든 문자는 `_`로 바뀝니다. `my-plugin`이라는 플러그인에 번들된 `database-tools` 서버의 경우, `query` 도구는 다음과 같이 호출할 수 있습니다:

```
mcp__plugin_my-plugin_database-tools__query
```

[권한 규칙](/docs/ko/permissions)에서 도구를 참조할 때, 스킬의 `allowed-tools` 목록에서, [서브에이전트의 `tools` 필드](/docs/ko/sub-agents#available-tools)에서, 또는 [hook matcher](/docs/ko/hooks#match-mcp-tools)에서 이 전체 이름을 사용하세요. `mcp__database-tools__.*`와 같은 베어 서버 키에 대해 작성된 hook matcher는 플러그인 번들 서버에 대해 절대 실행되지 않습니다.

서버 자체는 `plugin:<plugin-name>:<server-name>` (예: `plugin:my-plugin:database-tools`)과 같은 범위 지정 이름으로 등록됩니다. 구성된 서버 이름이 예상되는 위치(예: [`mcp_tool` hook의 `server` 필드](/docs/ko/hooks#mcp-tool-hook-fields))에서 해당 이름을 사용하세요.

**플러그인 MCP 서버의 이점**:

* **번들 배포**: 도구 및 서버가 함께 패키징됨
* **자동 설정**: 수동 MCP 구성이 필요 없음
* **팀 일관성**: 플러그인이 설치되면 모든 사람이 동일한 도구를 얻음

플러그인과 함께 MCP 서버를 번들로 제공하는 방법에 대한 자세한 내용은 [플러그인 구성 요소 참조](/docs/ko/plugins-reference#mcp-servers)를 참조하세요.

<h2 id="mcp-installation-scopes">
  MCP 설치 범위
</h2>

MCP 서버는 세 가지 범위에서 구성할 수 있습니다. 선택한 범위는 서버가 로드되는 프로젝트와 구성이 팀과 공유되는지 여부를 제어합니다. 관리자는 [관리형 구성](#managed-mcp-configuration)을 통해 엔터프라이즈 수준에서 서버를 배포할 수도 있습니다.

| 범위                     | 로드 위치    | 팀과 공유        | 저장 위치                |
| ---------------------- | -------- | ------------ | -------------------- |
| [로컬](#local-scope)     | 현재 프로젝트만 | 아니오          | `~/.claude.json`     |
| [프로젝트](#project-scope) | 현재 프로젝트만 | 예, 버전 제어를 통해 | 프로젝트 루트의 `.mcp.json` |
| [사용자](#user-scope)     | 모든 프로젝트  | 아니오          | `~/.claude.json`     |

<h3 id="local-scope">
  로컬 범위
</h3>

로컬 범위는 기본값입니다. 로컬 범위 서버는 추가한 프로젝트에서만 로드되며 사용자에게만 비공개입니다. Claude Code는 해당 프로젝트의 경로 아래 `~/.claude.json`에 저장하므로 다른 프로젝트에는 동일한 서버가 나타나지 않습니다. 개인 개발 서버, 실험적 구성 또는 버전 제어에 포함하고 싶지 않은 자격 증명이 있는 서버에 로컬 범위를 사용하세요.

<Note>
  MCP 서버의 "로컬 범위"라는 용어는 일반 로컬 설정과 다릅니다. MCP 로컬 범위 서버는 `~/.claude.json` (홈 디렉토리)에 저장되고, 일반 로컬 설정은 `.claude/settings.local.json` (프로젝트 디렉토리)을 사용합니다. 설정 파일 위치에 대한 자세한 내용은 [설정](/docs/ko/settings#settings-files)을 참조하세요.
</Note>

```bash theme={null}
# 로컬 범위 서버 추가 (기본값)
claude mcp add --transport http stripe https://mcp.stripe.com

# 명시적으로 로컬 범위 지정
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

`/path/to/your/project`에서 실행할 때 명령은 `~/.claude.json` 내의 현재 프로젝트 항목에 서버를 작성합니다. 아래 예는 결과를 보여줍니다:

```json theme={null}
{
  "projects": {
    "/path/to/your/project": {
      "mcpServers": {
        "stripe": {
          "type": "http",
          "url": "https://mcp.stripe.com"
        }
      }
    }
  }
}
```

<h3 id="project-scope">
  프로젝트 범위
</h3>

프로젝트 범위 서버는 프로젝트 루트 디렉토리의 `.mcp.json` 파일에 구성을 저장하여 팀 협업을 가능하게 합니다. 이 파일은 버전 제어에 체크인되도록 설계되어 모든 팀 멤버가 동일한 MCP 도구 및 서비스에 액세스할 수 있도록 합니다. 프로젝트 범위 서버를 추가하면 Claude Code는 자동으로 이 파일을 생성하거나 적절한 구성 구조로 업데이트합니다.

```bash theme={null}
# 프로젝트 범위 서버 추가
claude mcp add --transport http paypal --scope project https://mcp.paypal.com/mcp
```

결과 `.mcp.json` 파일은 표준화된 형식을 따릅니다:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "command": "/path/to/server",
      "args": [],
      "env": {}
    }
  }
}
```

보안상의 이유로 Claude Code는 `.mcp.json` 파일의 프로젝트 범위 서버를 사용하기 전에 승인을 요청합니다. 이러한 승인 선택을 재설정해야 하는 경우 `claude mcp reset-project-choices` 명령을 사용하세요.

<h3 id="user-scope">
  사용자 범위
</h3>

사용자 범위 서버는 `~/.claude.json`에 저장되며 교차 프로젝트 접근성을 제공하므로 컴퓨터의 모든 프로젝트에서 사용할 수 있으면서 사용자 계정에만 비공개입니다. 이 범위는 개인 유틸리티 서버, 개발 도구 또는 다양한 프로젝트에서 자주 사용하는 서비스에 적합합니다.

```bash theme={null}
# 사용자 서버 추가
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

<h3 id="scope-hierarchy-and-precedence">
  범위 계층 및 우선순위
</h3>

동일한 서버가 둘 이상의 위치에 정의되면 Claude Code는 가장 높은 우선순위 소스의 정의를 사용하여 한 번 연결합니다. 해당 소스의 전체 서버 항목이 사용되며, 필드는 범위 간에 병합되지 않습니다.

1. 로컬 범위
2. 프로젝트 범위
3. 사용자 범위
4. [플러그인 제공 서버](/docs/ko/plugins)
5. [Claude.ai 커넥터](#use-mcp-servers-from-claude-ai)

세 범위는 이름으로 중복을 일치시킵니다. 플러그인과 커넥터는 엔드포인트로 일치하므로 위의 서버와 동일한 URL 또는 명령을 가리키는 것은 중복으로 처리됩니다.

<h3 id="environment-variable-expansion-in-mcp-json">
  `.mcp.json`의 환경 변수 확장
</h3>

Claude Code는 `.mcp.json` 파일의 환경 변수 확장을 지원하므로 팀이 구성을 공유하면서 머신 특정 경로 및 API 키와 같은 민감한 값에 대한 유연성을 유지할 수 있습니다.

**지원되는 구문:**

* `${VAR}` - 환경 변수 `VAR`의 값으로 확장
* `${VAR:-default}` - `VAR`이 설정되면 확장, 그렇지 않으면 `default` 사용

**확장 위치:**
환경 변수는 다음에서 확장할 수 있습니다:

* `command` - 서버 실행 파일 경로
* `args` - 명령줄 인수
* `env` - 서버에 전달되는 환경 변수
* `url` - HTTP 서버 유형의 경우
* `headers` - HTTP 서버 인증의 경우

**변수 확장을 사용한 예**:

```json theme={null}
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

참조된 환경 변수가 설정되지 않았고 기본값이 없으면 Claude Code는 값에 리터럴 `${VAR}` 텍스트를 남겨두고 해당 서버에 대해 누락된 변수 경고를 보고합니다. 구성은 여전히 로드되므로 변수를 설정하거나 `:-default` 폴백을 추가하여 서버가 의도한 값으로 시작하도록 하세요.

<h2 id="practical-examples">
  실제 예
</h2>

<h3 id="example-monitor-errors-with-sentry">
  예: Sentry로 오류 모니터링
</h3>

```bash theme={null}
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
```

Sentry 계정으로 인증합니다:

```text theme={null}
/mcp
```

그런 다음 프로덕션 문제를 디버깅합니다:

```text theme={null}
지난 24시간 동안 가장 일반적인 오류는 무엇입니까?
```

```text theme={null}
오류 ID abc123의 스택 추적을 보여주세요
```

```text theme={null}
어떤 배포가 이러한 새로운 오류를 도입했습니까?
```

<h3 id="example-connect-to-github-for-code-reviews">
  예: 코드 검토를 위해 GitHub에 연결
</h3>

GitHub의 원격 MCP 서버는 헤더로 전달된 GitHub 개인 액세스 토큰으로 인증합니다. 하나를 얻으려면 [GitHub 토큰 설정](https://github.com/settings/personal-access-tokens)을 열고, Claude가 작업하려는 리포지토리에 액세스할 수 있는 새로운 세분화된 토큰을 생성한 다음 서버를 추가하세요:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

그런 다음 GitHub로 작업합니다:

```text theme={null}
PR #456을 검토하고 개선 사항을 제안하세요
```

```text theme={null}
방금 발견한 버그에 대한 새 이슈를 생성하세요
```

```text theme={null}
나에게 할당된 모든 열린 PR을 보여주세요
```

<h3 id="example-query-your-postgresql-database">
  예: PostgreSQL 데이터베이스 쿼리
</h3>

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

그런 다음 자연스럽게 데이터베이스를 쿼리합니다:

```text theme={null}
이번 달 총 수익은 얼마입니까?
```

```text theme={null}
주문 테이블의 스키마를 보여주세요
```

```text theme={null}
지난 90일 동안 구매하지 않은 고객을 찾으세요
```

<h2 id="authenticate-with-remote-mcp-servers">
  원격 MCP 서버로 인증
</h2>

많은 클라우드 기반 MCP 서버는 인증이 필요합니다. Claude Code는 보안 연결을 위해 OAuth 2.0을 지원합니다.

Claude Code는 서버가 `401 Unauthorized` 또는 `403 Forbidden`으로 응답할 때 원격 서버를 인증이 필요한 것으로 표시합니다. 로그인하지 않은 서버의 경우 두 상태 코드 모두 `/mcp`에서 서버를 플래그하여 OAuth 흐름을 완료할 수 있습니다.

이미 로그인한 OAuth 서버에 대한 요청이 `401 Unauthorized`를 반환하면 Claude Code는 저장된 토큰을 새로 고치고 재연결한 후 요청을 한 번 재시도합니다. 해당 재시도도 실패한 경우에만 `/mcp`에서 서버를 플래그합니다. v2.1.206 이전에는 네트워크 오류와 같은 일시적인 이유로 토큰 새로 고침이 실패하면 새로 고침 토큰이 여전히 유효했음에도 불구하고 OAuth 서버를 세션의 나머지 기간 동안 인증이 필요한 것으로 플래그했습니다.

v2.1.195부터 토큰 새로 고침이 서버가 저장된 새로 고침 토큰을 거부하기 때문에 실패하면 Claude Code는 즉시 `/mcp`를 가리키는 알림을 표시합니다. 연결된 서버의 메뉴에서 다시 인증하기를 제공하므로 다음 도구 호출이 실패하기 전에 다시 로그인할 수 있습니다.

인증 서버를 가리키는 `WWW-Authenticate` 헤더를 반환하는 사용자 정의 서버는 다른 원격 서버와 동일한 자동 검색을 받습니다.

v2.1.193부터 Claude Code는 하나 이상의 구성된 서버가 인증이 필요할 때 시작 알림을 표시하므로 어떤 서버가 로그인이 필요한지 알아내기 위해 `/mcp`를 열 필요가 없습니다.

비대화형 모드에는 `/mcp` 패널이 없으므로 Claude Code는 OAuth 흐름을 실행할 수 없습니다. v2.1.196부터 구성된 서버가 `claude -p` 또는 [도구 검색](#scale-with-mcp-tool-search)이 활성화된 Agent SDK 실행 중에 인증이 필요할 때 (기본값), Claude Code는 Claude에게 서버의 도구가 인증할 때까지 사용할 수 없음을 알립니다. Claude는 서버가 구성되지 않은 것처럼 응답하는 대신 로그인이 필요한 서버의 이름을 지정할 수 있습니다. `/mcp`를 사용하는 대화형 세션에서 또는 `claude mcp login <name>`으로 로그인을 완료합니다.

서버에 대해 `headers.Authorization`을 구성했는데 서버가 해당 헤더를 거부하면 Claude Code는 OAuth로 폴백하지 않고 연결이 실패한 것으로 보고합니다. MCP 엔드포인트에 대해 토큰이 유효한지 확인하거나 OAuth 흐름을 사용하려면 헤더를 제거합니다.

<Steps>
  <Step title="인증이 필요한 서버 추가">
    예를 들어:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Claude Code 내에서 /mcp 명령 사용">
    Claude Code에서 다음 명령을 사용합니다:

    ```text theme={null}
    /mcp
    ```

    그런 다음 브라우저에서 로그인 단계를 따릅니다.
  </Step>
</Steps>

<Tip>
  팁:

  * 인증 토큰은 안전하게 저장되고 자동으로 새로 고쳐집니다
  * `/mcp` 메뉴에서 "Clear authentication"을 사용하여 액세스를 취소합니다
  * 브라우저가 자동으로 열리지 않으면 제공된 URL을 복사하여 수동으로 엽니다
  * 인증 후 브라우저 리디렉션이 연결 오류로 실패하면 브라우저의 주소 표시줄에서 전체 콜백 URL을 복사하여 Claude Code에 나타나는 URL 프롬프트에 붙여넣습니다
  * OAuth 인증은 HTTP 서버에서 작동합니다
</Tip>

<h3 id="authenticate-from-the-command-line">
  명령줄에서 인증
</h3>

v2.1.186부터 `claude mcp login <name>`은 구성된 서버의 OAuth 흐름을 셸에서 직접 실행하므로 세션 내에서 `/mcp` 패널을 열 필요가 없습니다.

```bash theme={null}
claude mcp login sentry
```

나중에 저장된 자격 증명을 지우려면 `claude mcp logout <name>`을 실행합니다.

v2.1.191부터 명령은 SSH 세션 중이거나 디스플레이 서버가 없는 Linux와 같이 로컬 브라우저를 사용할 수 없는 경우를 감지하고 브라우저를 열려고 시도하는 대신 인증 URL을 출력합니다. 로컬 머신에서 URL을 열고 브라우저의 주소 표시줄에서 전체 리디렉션 URL을 프롬프트에 다시 붙여넣습니다. 명령은 붙여넣기 단계를 위해 대화형 터미널이 필요하므로 `ssh -t`로 연결합니다. 로컬 브라우저가 감지되었을 때도 URL 프롬프트를 강제하려면 `--no-browser`를 전달합니다.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  고정 OAuth 콜백 포트 사용
</h3>

일부 MCP 서버는 미리 등록된 특정 리디렉션 URI가 필요합니다. 기본적으로 Claude Code는 OAuth 콜백을 위해 무작위로 사용 가능한 포트를 선택합니다. `--callback-port`를 사용하여 포트를 고정하여 `http://localhost:PORT/callback` 형식의 사전 등록된 리디렉션 URI와 일치하도록 합니다.

`--callback-port`를 단독으로 사용할 수 있습니다 (동적 클라이언트 등록 포함) 또는 `--client-id`와 함께 사용할 수 있습니다 (사전 구성된 자격 증명 포함).

```bash theme={null}
# 동적 클라이언트 등록을 사용한 고정 콜백 포트
claude mcp add --transport http \
  --callback-port 8080 \
  my-server https://mcp.example.com/mcp
```

<h3 id="use-pre-configured-oauth-credentials">
  사전 구성된 OAuth 자격 증명 사용
</h3>

일부 MCP 서버는 동적 클라이언트 등록을 통한 자동 OAuth 설정을 지원하지 않습니다. "Incompatible auth server: does not support dynamic client registration"과 같은 오류가 표시되면 서버에 사전 구성된 자격 증명이 필요합니다. Claude Code는 또한 동적 클라이언트 등록 대신 클라이언트 ID 메타데이터 문서 (CIMD)를 사용하는 서버를 지원하며 자동으로 검색합니다. 자동 검색이 실패하면 먼저 서버의 개발자 포털을 통해 OAuth 앱을 등록한 다음 서버를 추가할 때 자격 증명을 제공합니다.

<Steps>
  <Step title="서버로 OAuth 앱 등록">
    서버의 개발자 포털을 통해 앱을 생성하고 클라이언트 ID와 클라이언트 시크릿을 기록합니다.

    많은 서버는 리디렉션 URI도 필요합니다. 그렇다면 포트를 선택하고 `http://localhost:PORT/callback` 형식으로 리디렉션 URI를 등록합니다. 다음 단계에서 `--callback-port`와 함께 동일한 포트를 사용합니다.
  </Step>

  <Step title="자격 증명으로 서버 추가">
    다음 방법 중 하나를 선택합니다. `--callback-port`에 사용되는 포트는 사용 가능한 모든 포트일 수 있습니다. 이전 단계에서 등록한 리디렉션 URI와 일치하기만 하면 됩니다.

    <Tabs>
      <Tab title="claude mcp add">
        `--client-id`를 사용하여 앱의 클라이언트 ID를 전달합니다. `--client-secret` 플래그는 마스킹된 입력으로 시크릿을 요청합니다:

        ```bash theme={null}
        claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>

      <Tab title="claude mcp add-json">
        JSON 구성에 `oauth` 객체를 포함하고 `--client-secret`을 별도의 플래그로 전달합니다:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' \
          --client-secret
        ```
      </Tab>

      <Tab title="claude mcp add-json (콜백 포트만)">
        동적 클라이언트 등록을 사용하면서 포트를 고정하려면 클라이언트 ID 없이 `--callback-port`를 사용합니다:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"callbackPort":8080}}'
        ```
      </Tab>

      <Tab title="CI / 환경 변수">
        환경 변수를 통해 시크릿을 설정하여 대화형 프롬프트를 건너뜁니다:

        ```bash theme={null}
        MCP_CLIENT_SECRET=your-secret claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="Claude Code에서 인증">
    Claude Code에서 `/mcp`를 실행하고 브라우저 로그인 흐름을 따릅니다.
  </Step>
</Steps>

<Tip>
  팁:

  * 클라이언트 시크릿은 구성에 저장되지 않고 시스템 키체인 (macOS) 또는 자격 증명 파일에 안전하게 저장됩니다
  * 서버가 시크릿이 없는 공개 OAuth 클라이언트를 사용하는 경우 `--client-secret` 없이 `--client-id`만 사용합니다
  * `--callback-port`는 `--client-id`와 함께 또는 없이 사용할 수 있습니다
  * 이러한 플래그는 HTTP 및 SSE 전송에만 적용됩니다. stdio 서버에는 영향을 주지 않습니다
  * `claude mcp get <name>`을 사용하여 OAuth 자격 증명이 서버에 대해 구성되었는지 확인합니다
</Tip>

<h3 id="override-oauth-metadata-discovery">
  OAuth 메타데이터 검색 재정의
</h3>

특정 OAuth 인증 서버 메타데이터 URL을 가리켜 기본 검색 체인을 우회하도록 Claude Code를 설정합니다. MCP 서버의 표준 엔드포인트가 오류를 반환하거나 내부 프록시를 통해 검색을 라우팅하려는 경우에 `authServerMetadataUrl`을 설정합니다. 기본적으로 Claude Code는 먼저 `/.well-known/oauth-protected-resource`에서 RFC 9728 보호된 리소스 메타데이터를 확인한 다음 `/.well-known/oauth-authorization-server`에서 RFC 8414 인증 서버 메타데이터로 돌아갑니다.

`.mcp.json`의 서버 구성의 `oauth` 객체에 `authServerMetadataUrl`을 설정합니다:

```json theme={null}
{
  "mcpServers": {
    "my-server": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "authServerMetadataUrl": "https://auth.example.com/.well-known/openid-configuration"
      }
    }
  }
}
```

URL은 `https://`를 사용해야 합니다. 메타데이터 URL의 `scopes_supported`는 업스트림 서버가 광고하는 범위를 재정의합니다.

<h3 id="restrict-oauth-scopes">
  OAuth 범위 제한
</h3>

`oauth.scopes`를 설정하여 인증 흐름 중에 Claude Code가 요청하는 범위를 고정합니다. 이는 업스트림 인증 서버가 광고하는 것보다 더 많은 범위를 부여하고 싶지 않을 때 MCP 서버를 보안 팀이 승인한 부분 집합으로 제한하는 지원되는 방법입니다. 값은 RFC 6749 §3.3의 `scope` 매개변수 형식과 일치하는 단일 공백으로 구분된 문자열입니다.

```json theme={null}
{
  "mcpServers": {
    "slack": {
      "type": "http",
      "url": "https://mcp.slack.com/mcp",
      "oauth": {
        "scopes": "channels:read chat:write search:read"
      }
    }
  }
}
```

`oauth.scopes`는 `authServerMetadataUrl`과 서버가 `/.well-known`에서 검색하는 범위 모두보다 우선합니다. 설정하지 않으면 MCP 서버가 요청된 범위 집합을 결정합니다.

v2.1.196부터 `oauth.scopes`가 설정되지 않으면 Claude Code는 서버의 `WWW-Authenticate` 헤더 또는 보호된 리소스 메타데이터에서 제공하는 범위를 요청하고 둘 다 제공하지 않을 때 `scope` 매개변수를 보내지 않습니다. 더 이상 자동으로 검색된 인증 서버 메타데이터에서 전체 `scopes_supported` 카탈로그를 요청하지 않습니다. 해당 카탈로그를 요청하면 관리자 전용 또는 템플릿 범위를 광고하는 ID 공급자가 `invalid_scope` 오류로 인증 요청을 거부하게 했습니다. 구성된 `authServerMetadataUrl`에서 가져온 메타데이터는 여전히 `scopes_supported`를 요청된 범위로 제공합니다.

인증 서버가 `scopes_supported`에서 `offline_access`를 광고하면 Claude Code는 액세스 토큰을 새로운 브라우저 로그인 없이 새로 고칠 수 있도록 고정된 범위에 추가합니다.

서버가 나중에 도구 호출에 대해 403 `insufficient_scope`을 반환하면 Claude Code는 동일한 고정된 범위로 다시 인증합니다. 필요한 도구가 고정된 범위 외의 범위를 요구할 때 `oauth.scopes`를 확대합니다.

<h3 id="use-dynamic-headers-for-custom-authentication">
  사용자 정의 인증을 위한 동적 헤더 사용
</h3>

MCP 서버가 OAuth (예: Kerberos, 단기 토큰 또는 내부 SSO)가 아닌 다른 인증 체계를 사용하는 경우 `headersHelper`를 사용하여 연결 시간에 요청 헤더를 생성합니다. Claude Code는 명령을 실행하고 출력을 연결 헤더에 병합합니다.

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "/opt/bin/get-mcp-auth-headers.sh"
    }
  }
}
```

명령은 인라인일 수도 있습니다:

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "echo '{\"Authorization\": \"Bearer '\"$(get-token)\"'\"}'"
    }
  }
}
```

**요구 사항:**

* 명령은 JSON 객체의 문자열 키-값 쌍을 stdout에 작성해야 합니다
* 명령은 10초 시간 초과를 사용하여 셸에서 실행되며, 세션의 현재 작업 디렉토리에서 실행됩니다. 스크립트에 절대 경로를 사용하거나 `PATH`의 명령을 사용합니다
* 동적 헤더는 동일한 이름의 정적 `headers`를 재정의합니다

헬퍼는 각 연결 (세션 시작 및 재연결 시)에서 새로 실행됩니다. 캐싱이 없으므로 스크립트는 토큰 재사용을 담당합니다.

v2.1.193부터 도구 호출이 `401 Unauthorized` 또는 `403 Forbidden`을 반환하면 Claude Code는 자동으로 헬퍼를 다시 실행하고 새로운 헤더로 재연결한 다음 호출을 한 번 재시도합니다. Claude Code는 해당 재시도도 실패한 경우에만 서버를 `/mcp`에서 인증이 필요한 것으로 표시합니다.

Claude Code는 헬퍼를 실행할 때 다음 환경 변수를 설정합니다:

| 변수                            | 값                                                                          |
| :---------------------------- | :------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | MCP 서버의 이름                                                                 |
| `CLAUDE_CODE_MCP_SERVER_URL`  | MCP 서버의 URL                                                                |
| `CLAUDE_PLUGIN_ROOT`          | 플러그인의 루트 디렉토리. [플러그인](/docs/ko/plugins-reference#mcp-servers)이 서버를 제공할 때만 설정됩니다 |

이를 사용하여 여러 MCP 서버를 제공하는 단일 헬퍼 스크립트를 작성합니다.

플러그인 제공 서버의 경우 헬퍼는 또한 작업 디렉토리가 플러그인 루트로 설정된 상태에서 실행되므로 상대 `headersHelper` 경로는 세션의 작업 디렉토리가 아닌 플러그인 디렉토리 내에서 확인됩니다. Claude Code v2.1.195 이상이 필요합니다.

플러그인 제공 `headersHelper`는 명령이 셸을 통해 실행되기 때문에 플러그인의 [`${user_config.*}`](/docs/ko/plugins-reference#user-configuration) 값을 참조할 수 없습니다. Claude Code는 서버를 [오류](/docs/ko/errors#plugin-command-references-user-config)와 함께 잘못 구성된 것으로 보고하고 값을 대체하지 않습니다. `${user_config.KEY}`를 셸 구문 분석되지 않는 서버의 `headers` 필드에 넣거나 헬퍼 스크립트가 자신의 환경 또는 구성 파일에서 값을 읽도록 합니다. v2.1.207 이전에는 `headersHelper`가 `${user_config.*}` 값을 대체했습니다.

<Note>
  `headersHelper`는 임의의 셸 명령을 실행합니다. 프로젝트 또는 로컬 범위에서 정의될 때 작업 공간 신뢰 대화 상자를 수락한 후에만 실행됩니다.
</Note>

<h2 id="add-mcp-servers-from-json-configuration">
  JSON 구성에서 MCP 서버 추가
</h2>

MCP 서버에 대한 JSON 구성이 있는 경우 직접 추가할 수 있습니다:

<Steps>
  <Step title="JSON에서 MCP 서버 추가">
    ```bash theme={null}
    # 기본 구문
    claude mcp add-json <name> '<json>'

    # 예: JSON 구성으로 HTTP 서버 추가
    claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp","headers":{"Authorization":"Bearer token"}}'

    # 예: JSON 구성으로 stdio 서버 추가
    claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"],"env":{"CACHE_DIR":"/tmp"}}'

    # 예: 사전 구성된 OAuth 자격 증명으로 HTTP 서버 추가
    claude mcp add-json my-server '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' --client-secret
    ```
  </Step>

  <Step title="서버가 추가되었는지 확인">
    ```bash theme={null}
    claude mcp get weather-api
    ```
  </Step>
</Steps>

<Tip>
  팁:

  * JSON이 셸에서 올바르게 이스케이프되었는지 확인합니다
  * JSON은 MCP 서버 구성 스키마를 준수해야 합니다
  * `--scope user`를 사용하여 프로젝트 특정 구성 대신 사용자 구성에 서버를 추가할 수 있습니다
</Tip>

<h2 id="import-mcp-servers-from-claude-desktop">
  Claude Desktop에서 MCP 서버 가져오기
</h2>

Claude Desktop에서 MCP 서버를 이미 구성한 경우 가져올 수 있습니다:

<Steps>
  <Step title="Claude Desktop에서 서버 가져오기">
    ```bash theme={null}
    # 기본 구문 
    claude mcp add-from-claude-desktop 
    ```
  </Step>

  <Step title="가져올 서버 선택">
    명령을 실행한 후 가져올 서버를 선택할 수 있는 대화형 대화 상자가 표시됩니다.
  </Step>

  <Step title="서버가 가져와졌는지 확인">
    ```bash theme={null}
    claude mcp list 
    ```
  </Step>
</Steps>

`claude mcp` 명령을 통해 추가된 서버 이름은 문자, 숫자, 하이픈 및 언더스코어만 포함할 수 있습니다. Claude Desktop은 해당 제한을 적용하지 않으므로 공백과 같은 다른 문자를 포함하는 이름의 Claude Desktop 서버는 가져올 수 없습니다. 가져오기는 거부된 각 이름을 보고하며 선택한 다른 서버는 계속 가져옵니다. v2.1.205 이전에는 첫 번째 잘못된 이름이 가져오기를 중지했으며 선택한 서버 중 어느 것도 추가되지 않았습니다.

<Tip>
  팁:

  * 이 기능은 macOS 및 Windows Subsystem for Linux (WSL)에서만 작동합니다
  * 이러한 플랫폼의 표준 위치에서 Claude Desktop 구성 파일을 읽습니다
  * `--scope user` 플래그를 사용하여 사용자 구성에 서버를 추가합니다
  * 가져온 서버는 이름에 문자, 숫자, 하이픈 및 언더스코어만 포함될 때 Claude Desktop과 동일한 이름을 유지합니다. Claude Code는 다른 문자를 포함하는 이름의 서버를 보고하고 건너뜁니다
  * 동일한 이름의 서버가 이미 존재하면 숫자 접미사가 붙습니다 (예: `server_1`)
</Tip>

<h2 id="use-mcp-servers-from-claude-ai">
  Claude.ai에서 MCP 서버 사용
</h2>

[Claude.ai](https://claude.ai) 계정으로 Claude Code에 로그인한 경우 Claude.ai에서 추가한 MCP 서버(즉, [커넥터](https://claude.com/docs/connectors))는 Claude Code에서 자동으로 사용 가능합니다:

<Steps>
  <Step title="Claude.ai에서 MCP 서버 구성">
    [claude.ai/customize/connectors](https://claude.ai/customize/connectors)에서 서버를 추가합니다. Team 및 Enterprise 플랜에서는 관리자만 서버를 추가할 수 있습니다.
  </Step>

  <Step title="MCP 서버 인증">
    Claude.ai에서 필요한 인증 단계를 완료합니다.
  </Step>

  <Step title="Claude Code에서 서버 보기 및 관리">
    Claude Code에서 다음 명령을 사용합니다:

    ```text theme={null}
    /mcp
    ```

    Claude.ai의 서버는 Claude.ai에서 온 것을 나타내는 표시기와 함께 목록에 나타납니다.
  </Step>
</Steps>

v2.1.161부터 이전에 로그인한 적이 없는 커넥터는 claude.ai 섹션 끝의 `Show unused connectors` 행 뒤에 축소되므로 조직에서 프로비저닝한 목록이 패널을 채우지 않습니다. 행을 선택하여 확장합니다. 이전에 로그인한 커넥터는 현재 재인증이 필요한 경우에도 계속 표시됩니다.

Claude.ai 커넥터는 활성 [인증 방법](/docs/ko/authentication#authentication-precedence)이 Claude.ai 구독인 경우에만 가져옵니다. `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `apiKeyHelper` 또는 Amazon Bedrock이나 Google Cloud의 Agent Platform과 같은 타사 공급자가 활성화되어 있으면 로드되지 않습니다. 이전에 `/login`을 실행했더라도 마찬가지입니다. `/mcp`에 추가한 커넥터가 나열되지 않으면 `/status`를 실행하여 활성화된 인증 방법을 확인하고, 해당 환경 변수를 설정 해제하거나 `apiKeyHelper` 설정을 제거한 후 `/login`을 실행하여 Claude.ai 계정을 선택합니다.

Claude Code에서 추가한 서버는 동일한 URL을 가리키는 claude.ai 커넥터보다 [우선순위](#scope-hierarchy-and-precedence)를 갖습니다. 이 경우 `/mcp`는 커넥터를 숨김으로 표시하고 커넥터를 사용하려는 경우 중복을 제거하는 방법을 표시합니다.

Microsoft 365, Gmail, Google Calendar와 같은 일부 Anthropic 호스팅 커넥터는 업스트림 ID 공급자가 claude.ai에서 등록한 리디렉션 URL만 허용하기 때문에 Claude Code에서 로컬 OAuth를 지원하지 않습니다. v2.1.162부터 이러한 호스트 중 하나를 `/mcp`에서 인증하면 대신 claude.ai의 설정 → 커넥터에서 연결하도록 지시하는 메시지가 표시됩니다. 거기에서 연결되면 커넥터가 Claude Code에 자동으로 나타납니다.

<h3 id="organization-controls-on-connector-tools">
  커넥터 도구에 대한 조직 제어
</h3>

조직은 [claude.ai 커넥터](https://claude.com/docs/connectors)의 도구별 제어를 설정할 수 있습니다. Claude Code는 시작 시 이러한 설정을 읽고 로컬에서 적용합니다. `/mcp`를 실행하여 커넥터의 각 도구에 적용되는 설정을 확인합니다.

* **도구가 `ask`로 설정됨**: Claude Code는 `Your organization requires approval for this tool` 이유로 모든 호출에서 프롬프트를 표시합니다. 프롬프트는 `acceptEdits`, `auto`, `bypassPermissions` [권한 모드](/docs/ko/permissions#permission-modes)에서도 나타나며 선택을 기억하는 옵션을 제공하지 않습니다. 도구와 일치하는 [허용 규칙](/docs/ko/permissions)도 프롬프트를 건너뛰지 않습니다. 프롬프트를 표시하지 않는 `dontAsk` 모드에서는 Claude Code가 호출을 거부합니다.
* **도구가 `blocked`로 설정됨**: Claude Code는 Claude가 보기 전에 도구를 필터링하므로 도구 목록에 나타나지 않습니다.

이러한 제어를 적용하려면 Claude Code v2.1.129 이상이 필요합니다. 이전 버전은 설정을 무시하고 표준 권한 흐름을 적용합니다.

<h3 id="disable-claude-ai-connectors">
  Claude.ai 커넥터 비활성화
</h3>

Claude Code에서 claude.ai MCP 서버를 비활성화하려면 모든 설정 범위에서 [`disableClaudeAiConnectors`](/docs/ko/settings#available-settings)를 `true`로 설정합니다:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

이 설정은 모든 소스 true 의미론을 사용합니다: 모든 설정 소스의 `true`가 우선순위를 갖습니다. 체크인된 프로젝트 `.claude/settings.json`은 클라우드 커넥터에서 저장소를 제외할 수 있지만, 프로젝트 수준의 `false`는 사용자 또는 정책 수준의 `true`가 비활성화한 커넥터를 다시 활성화할 수 없습니다. `--mcp-config`를 통해 명시적으로 전달된 서버는 영향을 받지 않습니다.

`ENABLE_CLAUDEAI_MCP_SERVERS` 환경 변수를 `false`로 설정할 수도 있으며, 이는 현재 셸 세션에 대해 동일한 효과를 갖습니다:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

모든 claude.ai 커넥터를 비활성화하는 대신 개별 claude.ai 커넥터를 차단하려면 이름 또는 URL 패턴으로 [`deniedMcpServers`](/docs/ko/managed-mcp)에 추가합니다. 예를 들어 `serverName` 항목 `"claude.ai Slack"`은 Slack 커넥터를 차단합니다. 현재 프로젝트에만 커넥터를 켜거나 끄려면 `/mcp` 패널을 사용합니다.

<Note>
  이러한 클라이언트 측 설정은 로컬 Claude Code 세션을 관리합니다. [Claude Code on the web](/docs/ko/claude-code-on-the-web) 세션에서는 claude.ai 커넥터가 원격 호스트에 의해 프로비저닝되고 명시적 `--mcp-config` 항목으로 도착하므로 `disableClaudeAiConnectors`는 적용되지 않습니다. 커넥터 URL은 세션 프록시를 통해 다시 작성되므로 공급업체 URL을 대상으로 하는 `deniedMcpServers` `serverUrl` 패턴은 일치하지 않습니다. 클라우드 세션이 사용할 수 있는 커넥터를 관리하려면 claude.ai 조직 설정에서 관리합니다.
</Note>

<h2 id="use-claude-code-as-an-mcp-server">
  Claude Code를 MCP 서버로 사용
</h2>

Claude Code 자체를 다른 애플리케이션이 연결할 수 있는 MCP 서버로 사용할 수 있습니다:

```bash theme={null}
# Claude를 stdio MCP 서버로 시작
claude mcp serve
```

claude\_desktop\_config.json에 이 구성을 추가하여 Claude Desktop에서 사용할 수 있습니다:

```json theme={null}
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

<Warning>
  **실행 파일 경로 구성**: `command` 필드는 Claude Code 실행 파일을 참조해야 합니다. `claude` 명령이 시스템의 PATH에 없으면 실행 파일의 전체 경로를 지정해야 합니다.

  전체 경로를 찾으려면:

  ```bash theme={null}
  which claude
  ```

  그런 다음 구성에서 전체 경로를 사용합니다:

  ```json theme={null}
  {
    "mcpServers": {
      "claude-code": {
        "type": "stdio",
        "command": "/full/path/to/claude",
        "args": ["mcp", "serve"],
        "env": {}
      }
    }
  }
  ```

  올바른 실행 파일 경로가 없으면 `spawn claude ENOENT`와 같은 오류가 발생합니다.
</Warning>

<Tip>
  팁:

  * 서버는 View, Edit, LS 등과 같은 Claude의 도구에 대한 액세스를 제공합니다.
  * Claude Desktop에서 Claude에게 디렉토리의 파일을 읽고, 편집하는 등을 요청해 보세요.
  * 이 MCP 서버는 Claude Code의 도구만 MCP 클라이언트에 노출하므로 클라이언트는 개별 도구 호출에 대한 사용자 확인을 구현할 책임이 있습니다.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  MCP 출력 제한 및 경고
</h2>

MCP 도구가 큰 출력을 생성할 때 Claude Code는 토큰 사용량을 관리하여 대화 컨텍스트가 압도되지 않도록 합니다:

* **출력 경고 임계값**: Claude Code는 MCP 도구 출력이 10,000 토큰을 초과할 때 경고를 표시합니다
* **구성 가능한 제한**: `MAX_MCP_OUTPUT_TOKENS` 환경 변수를 사용하여 최대 허용 MCP 출력 토큰을 조정할 수 있습니다
* **기본 제한**: 기본 최대값은 25,000 토큰입니다
* **범위**: 환경 변수는 자신의 제한을 선언하지 않는 도구에 적용됩니다. [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool)를 설정하는 도구는 `MAX_MCP_OUTPUT_TOKENS`이 설정된 것과 관계없이 텍스트 콘텐츠에 대해 해당 값을 사용합니다. 이미지 데이터를 반환하는 도구는 여전히 `MAX_MCP_OUTPUT_TOKENS`의 영향을 받습니다

큰 출력을 생성하는 도구의 제한을 늘리려면:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

이는 다음을 수행하는 MCP 서버로 작업할 때 특히 유용합니다:

* 대규모 데이터 세트 또는 데이터베이스 쿼리
* 상세한 보고서 또는 문서 생성
* 광범위한 로그 파일 또는 디버깅 정보 처리

<h3 id="raise-the-limit-for-a-specific-tool">
  특정 도구의 제한 늘리기
</h3>

MCP 서버를 구축하는 경우 도구의 `tools/list` 응답 항목에서 `_meta["anthropic/maxResultSizeChars"]`를 설정하여 개별 도구가 기본 디스크 유지 임계값보다 큰 결과를 반환할 수 있습니다. Claude Code는 해당 도구의 임계값을 주석 처리된 값으로 올립니다 (최대 500,000자의 하드 상한까지).

이는 데이터베이스 스키마 또는 전체 파일 트리와 같이 본질적으로 크지만 필요한 출력을 반환하는 도구에 유용합니다. 주석 처리 없이 기본 임계값을 초과하는 결과는 디스크에 유지되고 대화에서 파일 참조로 대체됩니다.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

주석 처리는 텍스트 콘텐츠에 대해 `MAX_MCP_OUTPUT_TOKENS`과 독립적으로 적용되므로 사용자는 도구가 선언하는 도구에 대해 환경 변수를 올릴 필요가 없습니다. 이미지 데이터를 반환하는 도구는 여전히 토큰 제한의 영향을 받습니다.

<Warning>
  특정 MCP 서버에서 자주 출력 경고가 발생하면 `MAX_MCP_OUTPUT_TOKENS` 제한을 늘리는 것을 고려하세요. 또한 서버 작성자에게 `anthropic/maxResultSizeChars` 주석을 추가하거나 응답을 페이지 매김하도록 요청할 수 있습니다. 주석은 이미지 콘텐츠를 반환하는 도구에는 영향을 주지 않습니다. 이러한 경우 `MAX_MCP_OUTPUT_TOKENS`을 올리는 것이 유일한 옵션입니다.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  루트 수준 결합자가 있는 도구 입력 스키마
</h2>

일부 MCP 서버는 도구의 입력 스키마를 JSON Schema 합집합으로 선언하며, `anyOf`, `oneOf` 또는 `allOf`가 스키마의 최상위 수준에 있습니다. Claude API는 스키마 루트에서 이러한 키워드를 허용하지 않습니다. 이는 `properties` 내에 중첩된 결합자를 허용하며, Claude Code는 변경 없이 전송합니다.

Claude Code v2.1.195부터 루트 수준 결합자가 있는 도구는 사용 가능한 상태로 유지됩니다. API에 도구를 보내기 전에 Claude Code는 스키마를 단일 객체로 평탄화하고 Claude에게 어떤 매개변수 그룹이 함께 속하는지 알려주는 문장을 도구의 설명 앞에 추가합니다:

* `allOf`: 모든 분기의 속성이 병합되고, 각 분기의 `required` 목록이 여전히 적용됩니다
* `anyOf` 및 `oneOf`: 모든 분기의 속성이 병합되고, 각 분기의 `required` 목록은 스키마에 의해 강제되지 않고 도구 설명에 설명됩니다

서버는 Claude가 선택한 인수를 수신하므로 서버 측에서 조합을 계속 검증하세요.

Claude Code가 API가 허용하는 스키마를 생성할 수 없거나 오프라인 머신과 같이 재작성을 활성화하는 원격 구성을 받지 않는 배포에서는 해당 도구 하나를 건너뛰고, 서버의 로그에 이유를 기록하고, 서버의 다른 도구는 사용 가능하게 유지합니다. v2.1.195보다 이전 버전은 입력 스키마에 루트 수준의 `anyOf`, `oneOf` 또는 `allOf`가 있는 모든 도구를 건너뜁니다.

<h2 id="require-approval-for-a-specific-tool">
  특정 도구에 대한 승인 필요
</h2>

MCP 서버를 구축하는 경우 도구의 `tools/list` 응답 항목에서 `_meta["anthropic/requiresUserInteraction"]`을 `true`로 설정하여 도구를 모든 호출에서 명시적 승인이 필요한 것으로 표시할 수 있습니다. 값은 JSON 부울 `true`여야 하며, 다른 값은 무시됩니다.

Claude Code는 `acceptEdits`, `auto`, `bypassPermissions` [권한 모드](/docs/ko/permissions#permission-modes)에서도 해당 도구의 권한 프롬프트를 모든 호출에서 표시하고 "다시 묻지 않기" 옵션을 제공하지 않습니다. 도구와 일치하는 [허용 규칙](/docs/ko/permissions#permission-rule-syntax)도 프롬프트를 건너뛰지 않습니다. `dontAsk` 모드에서는 프롬프트를 표시하지 않으므로 Claude Code는 호출을 거부합니다.

프롬프트는 사람에게 도달해야 합니다. [`--permission-prompt-tool`](/docs/ko/cli-reference#cli-flags)을 사용하는 비대화형 모드에서 플래그된 도구에 대한 프롬프트 도구의 `allow` 결과는 `MCP tool requires user interaction; not supported via --permission-prompt-tool` 메시지와 함께 거부로 변환됩니다. Agent SDK의 [`canUseTool` 콜백](/docs/ko/agent-sdk/permissions)은 이러한 호출을 수신하고 승인할 수 있습니다. SDK 호스트는 사용자에게 이를 표시할 것으로 예상되기 때문입니다.

이를 사용하여 권한 프롬프트 자체가 요점인 도구(예: 동의 또는 액세스 부여 단계)에 사용하세요. 자동 승인은 인간이 동의하지 않았다는 의미이기 때문입니다. 동일한 서버의 다른 도구는 정상적인 권한 동작을 유지합니다.

다음 `tools/list` 항목은 한 도구를 항상 승인이 필요한 것으로 표시합니다.

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

`anthropic/requiresUserInteraction` 주석은 Claude Code v2.1.199 이상이 필요합니다. 이전 버전은 이를 무시하고 표준 권한 흐름을 적용합니다.

세션이 [Remote Control](/docs/ko/remote-control)에 연결되거나 SDK 호스트에 연결되면 Claude Code는 권한 요청을 사용자 상호작용이 필요한 것으로 표시하므로 클라이언트는 한 번의 탭 승인 작업 대신 도구의 권한 프롬프트를 표시합니다.

<h2 id="respond-to-mcp-elicitation-requests">
  MCP elicitation 요청에 응답
</h2>

MCP 서버는 작업 중에 구조화된 입력을 요청할 수 있습니다(elicitation). 서버가 자체적으로 얻을 수 없는 정보가 필요할 때 Claude Code는 대화형 대화 상자를 표시하고 응답을 서버에 다시 전달합니다. 사용자 측에서 구성이 필요하지 않습니다: 서버가 요청할 때 elicitation 대화 상자가 자동으로 나타납니다.

서버는 두 가지 방식으로 입력을 요청할 수 있습니다:

* **양식 모드**: Claude Code는 서버에서 정의한 양식 필드가 있는 대화 상자를 표시합니다(예: 사용자 이름 및 암호 프롬프트). 필드를 입력하고 제출합니다.
* **URL 모드**: Claude Code는 인증 또는 승인을 위해 브라우저 URL을 엽니다. 브라우저에서 흐름을 완료한 다음 CLI에서 확인합니다.

elicitation 요청에 자동으로 응답하려면 대화 상자를 표시하지 않고 [`Elicitation` hook](/docs/ko/hooks#elicitation)을 사용하세요.

elicitation을 사용하는 MCP 서버를 구축하는 경우 [MCP elicitation 사양](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation)에서 프로토콜 세부 정보 및 스키마 예를 참조하세요.

<h2 id="use-mcp-resources">
  MCP 리소스 사용
</h2>

MCP 서버는 파일을 참조하는 방식과 유사하게 @ 멘션을 사용하여 참조할 수 있는 리소스를 노출할 수 있습니다.

<h3 id="reference-mcp-resources">
  MCP 리소스 참조
</h3>

<Steps>
  <Step title="사용 가능한 리소스 나열">
    프롬프트에 `@`를 입력하여 연결된 모든 MCP 서버의 사용 가능한 리소스를 확인합니다. 리소스는 자동 완성 메뉴의 파일과 함께 나타납니다.
  </Step>

  <Step title="특정 리소스 참조">
    `@server:protocol://resource/path` 형식을 사용하여 리소스를 참조합니다:

    ```text theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="여러 리소스 참조">
    단일 프롬프트에서 여러 리소스를 참조할 수 있습니다:

    ```text theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  팁:

  * 리소스는 참조될 때 자동으로 가져와지고 첨부 파일로 포함됩니다
  * 리소스 경로는 @ 멘션 자동 완성에서 퍼지 검색 가능합니다
  * Claude Code는 서버가 지원할 때 MCP 리소스를 나열하고 읽을 수 있는 도구를 자동으로 제공합니다
  * 리소스는 MCP 서버가 제공하는 모든 유형의 콘텐츠를 포함할 수 있습니다 (텍스트, JSON, 구조화된 데이터 등)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  MCP Tool Search로 확장
</h2>

Tool Search는 MCP 컨텍스트 사용량을 낮게 유지하여 도구 정의를 Claude가 필요할 때까지 연기합니다. 세션 시작 시 도구 이름과 서버 지침만 로드되므로 더 많은 MCP 서버를 추가해도 컨텍스트 윈도우에 미치는 영향이 최소화됩니다. Claude Code는 서버당 고정된 도구 상한을 부과하지 않습니다. 실질적인 한계는 컨텍스트 윈도우 예산입니다.

<h3 id="how-it-works">
  작동 방식
</h3>

Tool Search는 기본적으로 활성화됩니다. MCP 도구는 미리 로드되지 않고 연기되며, Claude는 검색 도구를 사용하여 작업에 필요할 때 관련 도구를 검색합니다. Claude가 실제로 사용하는 도구만 컨텍스트에 들어갑니다. 사용자 관점에서 MCP 도구는 이전과 정확히 동일하게 작동합니다.

임계값 기반 로딩을 선호하는 경우 `ENABLE_TOOL_SEARCH=auto`를 설정하여 컨텍스트 윈도우의 10% 이내에 맞을 때 스키마를 미리 로드하고 오버플로우만 연기합니다. 모든 옵션은 [Tool Search 구성](#configure-tool-search)을 참조하세요.

<h3 id="for-mcp-server-authors">
  MCP 서버 작성자용
</h3>

MCP 서버를 구축하는 경우 Tool Search가 활성화되면 서버 지침 필드가 더 유용해집니다. 서버 지침은 Claude가 [skills](/docs/ko/skills)의 작동 방식과 유사하게 도구를 검색할 시기를 이해하는 데 도움이 됩니다.

다음을 설명하는 명확하고 설명적인 서버 지침을 추가합니다:

* 도구가 처리하는 작업의 범주
* Claude가 도구를 검색해야 할 때
* 서버가 제공하는 주요 기능

Claude Code는 도구 설명 및 서버 지침을 각각 2KB에서 자릅니다. 자르기를 피하려면 간결하게 유지하고 중요한 세부 정보를 시작 부분에 배치합니다.

<h3 id="configure-tool-search">
  Tool Search 구성
</h3>

Tool Search는 기본적으로 활성화됩니다: MCP 도구는 연기되고 필요에 따라 검색됩니다. Claude Code는 Google Cloud의 Agent Platform에서 기본적으로 비활성화합니다. `ANTHROPIC_BASE_URL`이 비 자사 호스트를 가리킬 때도 비활성화됩니다(대부분의 프록시가 `tool_reference` 블록을 전달하지 않기 때문). 폴백을 재정의하려면 `ENABLE_TOOL_SEARCH`를 명시적으로 설정합니다.

[`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/ko/env-vars)를 설정하면 Tool Search가 꺼지고, `ENABLE_TOOL_SEARCH`는 이를 재정의할 수 없습니다. 이 변수는 `defer_loading` 도구 정의 및 `tool_reference` 콘텐츠 블록이 필요로 하는 베타 헤더를 제거합니다.

Tool Search는 `tool_reference` 블록을 지원하는 모델이 필요합니다: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 및 이후 모델. 현재 목록은 [API 문서의 모델 호환성](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility)을 참조하세요. Google Cloud의 Agent Platform에서는 Claude Sonnet 4.5 이상 및 Claude Opus 4.5 이상에서 Tool Search가 지원됩니다.

`ENABLE_TOOL_SEARCH` 환경 변수로 Tool Search 동작을 제어합니다:

| 값         | 동작                                                                                                                                                                                                  |
| :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (설정되지 않음) | 모든 MCP 도구 연기되고 필요에 따라 로드됨. Google Cloud의 Agent Platform 또는 `ANTHROPIC_BASE_URL`이 비 자사 호스트일 때 미리 로드로 돌아감                                                                                             |
| `true`    | 모든 MCP 도구 연기. Claude Code는 Google Cloud의 Agent Platform 및 프록시를 통해서도 베타 헤더를 전송합니다. Google Cloud의 Agent Platform의 Sonnet 4.5 또는 Opus 4.5보다 이전 모델에서 요청이 실패하거나 `tool_reference` 블록을 지원하지 않는 프록시에서 실패합니다 |
| `auto`    | 임계값 모드: 도구가 컨텍스트 윈도우의 10% 이내에 맞으면 미리 로드, 그렇지 않으면 연기                                                                                                                                                 |
| `auto:N`  | 사용자 정의 백분율을 사용한 임계값 모드, `N`은 0-100 (예: `auto:5`는 5%)                                                                                                                                                |
| `false`   | 모든 MCP 도구 미리 로드, 연기 없음                                                                                                                                                                              |

```bash theme={null}
# 사용자 정의 5% 임계값 사용
ENABLE_TOOL_SEARCH=auto:5 claude

# Tool Search 완전히 비활성화
ENABLE_TOOL_SEARCH=false claude
```

또는 [settings.json `env` 필드](/docs/ko/settings#available-settings)에서 값을 설정합니다.

`ToolSearch` 도구를 특별히 비활성화할 수도 있습니다:

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  서버를 연기에서 제외
</h3>

서버의 도구가 검색 단계 없이 항상 Claude에게 표시되어야 하는 경우 해당 서버의 구성에서 `alwaysLoad`를 `true`로 설정합니다. 그러면 `ENABLE_TOOL_SEARCH` 설정에 관계없이 해당 서버의 모든 도구가 세션 시작 시 컨텍스트에 로드됩니다. 매 턴마다 Claude가 필요로 하는 소수의 도구에 이를 사용합니다. 각 미리 로드된 도구는 대화에 사용할 수 있는 컨텍스트를 소비하기 때문입니다.

다음 `.mcp.json` 항목은 한 HTTP 서버를 제외하면서 다른 서버는 연기된 상태로 유지합니다:

```json theme={null}
{
  "mcpServers": {
    "core-tools": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "alwaysLoad": true
    }
  }
}
```

`alwaysLoad` 필드는 모든 서버 유형에서 사용 가능하며 Claude Code v2.1.121 이상이 필요합니다. MCP 서버는 도구의 `_meta` 객체에 `"anthropic/alwaysLoad": true`를 포함하여 개별 도구를 항상 로드되도록 표시할 수도 있으며, 이는 해당 도구에만 동일한 효과를 갖습니다.

`alwaysLoad: true`를 설정하면 서버가 연결될 때까지 시작이 차단되며, 표준 5초 연결 타임아웃으로 제한됩니다. 이는 MCP 시작이 기본적으로 [비차단](/docs/ko/env-vars)이더라도 적용됩니다. 첫 번째 프롬프트가 빌드될 때 도구가 있어야 하기 때문입니다. 다른 서버는 계속해서 백그라운드에서 연결됩니다.

<h2 id="use-mcp-prompts-as-commands">
  MCP 프롬프트를 명령으로 사용
</h2>

MCP 서버는 Claude Code에서 명령으로 사용 가능하게 되는 프롬프트를 노출할 수 있습니다.

<h3 id="execute-mcp-prompts">
  MCP 프롬프트 실행
</h3>

<Steps>
  <Step title="사용 가능한 프롬프트 검색">
    `/`를 입력하여 MCP 서버의 프롬프트를 포함한 모든 사용 가능한 명령을 확인합니다. MCP 프롬프트는 `/mcp__servername__promptname` 형식으로 나타납니다.
  </Step>

  <Step title="인수 없이 프롬프트 실행">
    ```text theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="인수를 사용하여 프롬프트 실행">
    많은 프롬프트는 인수를 허용합니다. 명령 뒤에 공백으로 구분하여 전달합니다:

    ```text theme={null}
    /mcp__github__pr_review 456
    ```

    ```text theme={null}
    /mcp__jira__create_issue "로그인 흐름의 버그" high
    ```
  </Step>
</Steps>

<Tip>
  팁:

  * MCP 프롬프트는 연결된 서버에서 동적으로 검색됩니다
  * 인수는 프롬프트의 정의된 매개변수를 기반으로 구문 분석됩니다
  * 프롬프트 결과는 대화에 직접 주입됩니다
  * 서버 및 프롬프트 이름은 정규화됩니다 (공백은 밑줄이 됨)
</Tip>

<h2 id="managed-mcp-configuration">
  관리되는 MCP 구성
</h2>

중앙 집중식 제어가 필요한 조직의 경우 MCP 서버에 사용자가 연결할 수 있는 서버를 제어하려면 [관리되는 MCP 구성](/docs/ko/managed-mcp)을 참조하십시오. 이는 `managed-mcp.json`을 사용하여 고정된 서버 세트 배포, `allowedMcpServers` 및 `deniedMcpServers`로 서버 제한, 서버가 차단될 때 사용자가 보는 내용을 다룹니다.
