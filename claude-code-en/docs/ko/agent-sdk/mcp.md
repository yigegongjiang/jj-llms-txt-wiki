> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 외부 도구와 MCP로 연결하기

> MCP 서버를 구성하여 에이전트를 외부 도구로 확장합니다. 전송 유형, 대규모 도구 세트를 위한 도구 검색, 인증 및 오류 처리를 다룹니다.

[Model Context Protocol (MCP)](https://modelcontextprotocol.io/docs/getting-started/intro)는 AI 에이전트를 외부 도구 및 데이터 소스에 연결하기 위한 개방형 표준입니다. MCP를 사용하면 에이전트가 데이터베이스를 쿼리하고, Slack 및 GitHub와 같은 API와 통합하며, 사용자 정의 도구 구현을 작성하지 않고도 다른 서비스에 연결할 수 있습니다.

MCP 서버는 로컬 프로세스로 실행되거나, HTTP를 통해 연결되거나, SDK 애플리케이션 내에서 직접 실행될 수 있습니다.

<Note>
  이 페이지는 Agent SDK에 대한 MCP 구성을 다룹니다. Claude Code CLI에 MCP 서버를 추가하여 모든 프로젝트에서 로드되도록 하려면 [MCP 설치 범위](/docs/ko/mcp#mcp-installation-scopes)를 참조하세요.
</Note>

<h2 id="quickstart">
  빠른 시작
</h2>

이 예제는 [HTTP 전송](#http%2Fsse-servers)을 사용하여 [Claude Code 문서](https://code.claude.com/docs) MCP 서버에 연결하고 [`allowedTools`](#allow-mcp-tools)를 와일드카드와 함께 사용하여 서버의 모든 도구를 허용합니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the docs MCP server to explain what hooks are in Claude Code",
    options: {
      mcpServers: {
        "claude-code-docs": {
          type: "http",
          url: "https://code.claude.com/docs/mcp"
        }
      },
      allowedTools: ["mcp__claude-code-docs__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "claude-code-docs": {
                  "type": "http",
                  "url": "https://code.claude.com/docs/mcp",
              }
          },
          allowed_tools=["mcp__claude-code-docs__*"],
      )

      async for message in query(
          prompt="Use the docs MCP server to explain what hooks are in Claude Code",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

에이전트는 문서 서버에 연결하고, hooks에 대한 정보를 검색하며, 결과를 반환합니다.

<h2 id="add-an-mcp-server">
  MCP 서버 추가
</h2>

`query()` 호출 시 코드에서 MCP 서버를 구성하거나, [`settingSources`](#from-a-config-file)를 통해 로드되는 `.mcp.json` 파일에서 구성할 수 있습니다.

<h3 id="in-code">
  코드에서
</h3>

`mcpServers` 옵션에서 MCP 서버를 직접 전달합니다. 이 예제는 `/Users/me/projects`에 대한 로컬 파일시스템 MCP 서버를 시작합니다. 해당 경로를 머신의 디렉토리로 바꾸세요:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List files in my project",
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "filesystem": {
                  "command": "npx",
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-filesystem",
                      "/Users/me/projects",
                  ],
              }
          },
          allowed_tools=["mcp__filesystem__*"],
      )

      async for message in query(prompt="List files in my project", options=options):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="from-a-config-file">
  구성 파일에서
</h3>

프로젝트 루트에 `.mcp.json` 파일을 생성합니다. 이 파일은 `project` 설정 소스가 활성화되어 있을 때 선택되며, 기본 `query()` 옵션에서는 활성화되어 있습니다. `settingSources`를 명시적으로 설정하는 경우, 이 파일이 로드되도록 `"project"`를 포함하세요. `/Users/me/projects`를 머신의 디렉토리로 바꾸세요:

```json theme={null}
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
    }
  }
}
```

<h2 id="connection-timing">
  연결 타이밍
</h2>

Claude Code는 `options.mcpServers`에 전달한 서버를 시작 시 등록하고, 첫 번째 턴 대기(있는 경우)가 해결되면 [init 메시지](#error-handling)를 내보냅니다. 각 `options.mcpServers` 서버가 첫 번째 턴을 지연시키는지 여부와 연결 시기는 해당 서버의 유형에 따라 달라집니다:

| 서버 유형                                         | 첫 번째 턴을 지연시킵니까?                | 첫 번째 턴 대기 타임아웃                                           |
| :-------------------------------------------- | :----------------------------- | :------------------------------------------------------- |
| stdio 서버 또는 캐시된 도구 목록이 없는 HTTP/SSE 서버         | 예, 연결될 때까지                     | [`MCP_TIMEOUT`](/docs/ko/env-vars), 기본값 30초; 해당 기한에 연결이 실패합니다 |
| 캐시된 도구 목록이 있는 원격 서버, Claude Code가 이전 연결에서 저장함 | 아니요; 캐시된 도구는 첫 번째 턴부터 사용 가능합니다 | 없음; 첫 번째 도구 호출 시 연결되며, 해당 지연된 연결에는 자체 타임아웃이 있습니다         |
| 인프로세스 [SDK 서버](#sdk-mcp-servers)              | 예, 연결되고 도구를 나열할 때까지            | 없음; 연결 및 도구 나열 요청 각각에는 자체 타임아웃이 있습니다                     |

[설정 파일](#from-a-config-file)(예: `.mcp.json`)이나 플러그인에서 로드된 서버는 일반적으로 init 메시지에서 `pending` 상태를 표시합니다. `options.mcpServers`가 stdio, HTTP 또는 SSE 서버를 포함할 때, 첫 번째 턴은 이러한 보류 중인 서버도 기다리며, `MCP_TIMEOUT`까지 대기합니다. `options.mcpServers`가 비어 있거나 SDK 서버만 포함할 때, 첫 번째 턴은 대신 최대 2초까지 대기합니다:

* **[도구 검색](/docs/ko/agent-sdk/tool-search)(기본값)**: 대기는 [`alwaysLoad: true`](/docs/ko/mcp#exempt-a-server-from-deferral)로 구성된 여전히 보류 중인 서버를 포함하고 나머지는 포함하지 않습니다. 나머지는 백그라운드에서 계속 연결됩니다. [도구 가용성](/docs/ko/mcp#tool-availability)은 Claude가 연결된 후 해당 도구에 어떻게 도달하는지 설명합니다.
* **도구 검색 없음**: 대기는 모든 보류 중인 서버를 포함합니다. [도구 검색 구성](/docs/ko/agent-sdk/tool-search#configure-tool-search)은 도구 검색을 끄는 방법을 다룹니다. 예를 들어 `disallowedTools`를 통해 세션에서 `ToolSearch` 도구를 제외하면, 세션도 도구 검색 없이 실행됩니다.

`permissionPromptToolName`을 설정하면, 첫 번째 턴은 모든 경우에 해당 도구의 서버도 기다리며, `MCP_TIMEOUT`까지 대기합니다.

첫 번째 턴 대기를 직접 설정하려면, [`env` 옵션](/docs/ko/agent-sdk/configuration#set-environment-variables)에 `CLAUDE_CODE_MCP_STARTUP_WAIT_MS`를 추가합니다(예: `CLAUDE_CODE_MCP_STARTUP_WAIT_MS: "5000"`). 그러면 첫 번째 턴은 도구 검색 가용 여부와 관계없이 모든 보류 중인 서버를 기다리며 최대 그 많은 밀리초까지 대기합니다. 이 기한은 또한 `options.mcpServers`의 stdio, HTTP 및 SSE 서버에 대한 `MCP_TIMEOUT` 첫 번째 턴 대기를 대체합니다. `CLAUDE_CODE_MCP_STARTUP_WAIT_MS`는 Claude Code v2.1.274 이상이 필요합니다.

대기가 끝날 때 여전히 보류 중인 서버는 백그라운드에서 계속 연결됩니다. 변수를 `0`으로 설정하여 대기를 건너뜁니다. `permissionPromptToolName` 서버는 값에 관계없이 자체 `MCP_TIMEOUT` 대기를 유지합니다.

init 메시지가 전송되기 전에 첫 번째 턴 대기보다 별도의 이전 단계에서 시작 자체를 차단하려면:

* [`MCP_CONNECTION_NONBLOCKING`](/docs/ko/env-vars)을 `0`으로 설정하여 전체 연결 배치에서 차단합니다. Claude Code는 기본적으로 해당 대기를 5초로 제한합니다. [`MCP_CONNECT_TIMEOUT_MS`](/docs/ko/env-vars) 환경 변수를 밀리초 단위로 조정하여 제한을 조정합니다. 해당 기한에 보류 중인 서버는 백그라운드에서 계속 연결됩니다.
* 서버의 구성에서 `alwaysLoad: true`를 설정하여 해당 도구를 첫 번째 턴에서 전체 스키마로 사용 가능하게 하고, [도구 검색 지연에서 제외](/docs/ko/mcp#exempt-a-server-from-deferral)합니다. Claude Code는 시작 시 해당 서버의 도구를 기다리며, 같은 기한으로 제한되고, 다른 서버는 백그라운드에서 계속 연결됩니다. 캐시된 도구 목록이 있는 원격 서버는 위의 표에 따라 연결 없이 도구를 제공합니다.

`init` 서브타입이 있는 `system` 메시지는 내보낼 때 각 서버의 상태를 보고합니다. 해당 상태를 읽으려면 [오류 처리](#error-handling)를 참조하세요.

<h2 id="allow-mcp-tools">
  MCP 도구 허용
</h2>

MCP 도구는 Claude가 사용하기 전에 명시적 권한이 필요합니다. 권한이 없으면 Claude는 도구를 사용할 수 있음을 확인하지만 호출할 수 없습니다.

<h3 id="tool-naming-convention">
  도구 명명 규칙
</h3>

MCP 도구는 `mcp__<server-name>__<tool-name>` 패턴을 따릅니다. 예를 들어, `"github"`라는 이름의 GitHub 서버에 `list_issues` 도구가 있으면 `mcp__github__list_issues`가 됩니다.

<h3 id="auto-approve-with-allowedtools">
  allowedTools를 사용한 자동 승인
</h3>

`allowedTools`를 사용하여 특정 MCP 도구를 자동 승인하면 Claude가 권한 프롬프트 없이 도구를 사용할 수 있습니다:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        // your servers
      },
      allowedTools: [
        "mcp__github__*", // All tools from the github server
        "mcp__db__query", // Only the query tool from db server
        "mcp__slack__send_message" // Only send_message from slack server
      ]
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=[
          "mcp__github__*",  # All tools from the github server
          "mcp__db__query",  # Only the query tool from db server
          "mcp__slack__send_message",  # Only send_message from slack server
      ],
  )
  ```
</CodeGroup>

와일드카드(`*`)를 사용하면 각 도구를 개별적으로 나열하지 않고도 서버의 모든 도구를 허용할 수 있습니다.

<Note>
  **MCP 액세스를 위해 권한 모드보다 `allowedTools`를 선호합니다.** `permissionMode: "acceptEdits"`는 MCP 도구를 자동 승인하지 않습니다(파일 편집 및 파일 시스템 Bash 명령만 해당). `permissionMode: "bypassPermissions"`는 MCP 도구를 자동 승인하지만 대부분의 다른 안전 프롬프트도 비활성화하므로 필요한 것보다 범위가 넓습니다. 남아있는 프롬프트는 [권한이 평가되는 방식](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)을 참조하십시오. `allowedTools`의 와일드카드는 원하는 MCP 서버에만 정확히 액세스 권한을 부여하고 그 이상은 부여하지 않습니다. 전체 비교는 [권한 모드](/docs/ko/agent-sdk/permissions#permission-modes)를 참조하십시오.
</Note>

<h3 id="discover-available-tools">
  사용 가능한 도구 검색
</h3>

MCP 서버가 제공하는 도구를 확인하려면 서버의 설명서를 확인하거나 `system` init 메시지의 `tools` 배열을 검사합니다. MCP 도구 이름은 `mcp__`로 시작합니다.

Claude Code는 `options.mcpServers`에 전달된 서버에 대해 [첫 번째 턴 연결 대기](#connection-timing) 후 init 메시지를 내보내므로, `tools` 배열은 그때까지 연결된 각 서버의 `mcp__` 도구와 [캐시된 도구 목록](#connection-timing)이 있는 서버의 도구를 나열합니다. 이러한 서버는 첫 사용 시 연결됩니다. 아직 연결되지 않은 다른 서버의 도구는 없습니다. 각 서버의 상태를 읽으려면 [오류 처리](#error-handling)를 참조하십시오.

이 필터는 MCP 도구 이름을 출력합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    mcpServers: {
      // your servers
    },
  };

  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      const mcpTools = message.tools.filter((name) => name.startsWith("mcp__"));
      console.log("Available MCP tools:", mcpTools);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              # your servers
          },
      )
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              mcp_tools = [t for t in message.data.get("tools", []) if t.startswith("mcp__")]
              print("Available MCP tools:", mcp_tools)


  asyncio.run(main())
  ```
</CodeGroup>

Claude에게 서버에서 사용 가능한 도구를 나열하도록 요청할 수도 있습니다.

<h2 id="transport-types">
  전송 유형
</h2>

MCP 서버는 다양한 전송 프로토콜을 사용하여 에이전트와 통신합니다. 서버의 문서를 확인하여 지원하는 전송 방식을 확인하세요:

* 문서에 **실행할 명령어**가 제시된 경우(예: `npx @modelcontextprotocol/server-filesystem`), stdio를 사용하세요
* 문서에 **URL**이 제시된 경우, HTTP 또는 SSE를 사용하세요
* 코드에서 자체 도구를 구축하는 경우, SDK MCP 서버를 사용하세요

<h3 id="stdio-servers">
  stdio 서버
</h3>

stdin/stdout을 통해 통신하는 로컬 프로세스입니다. 동일한 머신에서 실행하는 MCP 서버에 이를 사용하세요. `.mcp.json` 형식의 경우, [설정 파일에서](#from-a-config-file) 표시된 동일한 필드를 사용하세요. 코드에서는 명령어와 해당 인수를 전달하세요. `/Users/me/projects`를 머신의 디렉터리로 바꾸세요:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__read_file", "mcp__filesystem__list_directory"]
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          "filesystem": {
              "command": "npx",
              "args": [
                  "-y",
                  "@modelcontextprotocol/server-filesystem",
                  "/Users/me/projects",
              ],
          }
      },
      allowed_tools=["mcp__filesystem__read_file", "mcp__filesystem__list_directory"],
  )
  ```
</CodeGroup>

<h3 id="http/sse-servers">
  HTTP/SSE 서버
</h3>

클라우드 호스팅 MCP 서버 및 원격 API에 HTTP 또는 SSE를 사용하세요. `.mcp.json` 형식의 경우, [HTTP 원격 서버 헤더](#http-headers-for-remote-servers)의 예시와 동일한 필드를 사용하고, SSE 서버의 경우 `"type": "sse"`를 사용하세요. 코드에서는 서버의 URL을 전달하세요:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        "remote-api": {
          type: "sse",
          url: "https://api.example.com/mcp/sse",
          headers: {
            Authorization: `Bearer ${process.env.API_TOKEN}`
          }
        }
      },
      allowedTools: ["mcp__remote-api__*"]
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          "remote-api": {
              "type": "sse",
              "url": "https://api.example.com/mcp/sse",
              "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
          }
      },
      allowed_tools=["mcp__remote-api__*"],
  )
  ```
</CodeGroup>

스트리밍 가능한 HTTP 전송의 경우, 대신 `"type": "http"`을 사용하세요. `.mcp.json` 및 기타 JSON 설정 파일에서는 `"streamable-http"`이 `"http"`의 별칭으로 허용됩니다. SDK의 `McpHttpServerConfig` 유형은 `"http"`만 선언하므로, 코드에서 전달하는 서버의 경우 `"http"`를 사용하세요.

<h3 id="sdk-mcp-servers">
  SDK MCP 서버
</h3>

별도의 서버 프로세스를 실행하는 대신 애플리케이션 코드에서 직접 사용자 정의 도구를 정의하세요. 구현 세부 사항은 [사용자 정의 도구 가이드](/docs/ko/agent-sdk/custom-tools)를 참조하세요.

[`initialize` 제어 요청](/docs/ko/agent-sdk/typescript#sdkcontrolinitializeresponse)으로 등록된 SDK MCP 서버는 Claude Code가 요청을 처리하는 즉시 연결을 시작합니다.

<h2 id="mcp-tool-search">
  MCP 도구 검색
</h2>

많은 MCP 도구가 구성되어 있을 때, 도구 정의가 컨텍스트 윈도우의 상당한 부분을 차지할 수 있습니다. 도구 검색은 컨텍스트에서 도구 정의를 보류하고 각 턴에서 Claude가 필요로 하는 도구만 로드하여 이 문제를 해결합니다.

도구 검색은 기본적으로 활성화되어 있습니다. 구성 옵션, 모범 사례 및 사용자 정의 SDK 도구와 함께 도구 검색을 사용하는 방법에 대해서는 [도구 검색](/docs/ko/agent-sdk/tool-search)을 참조하십시오.

<h2 id="authentication">
  인증
</h2>

대부분의 MCP 서버는 외부 서비스에 접근하기 위해 인증이 필요합니다. 서버 구성에서 환경 변수를 통해 자격 증명을 전달합니다.

<h3 id="pass-credentials-via-environment-variables">
  환경 변수를 통해 자격 증명 전달
</h3>

`env` 필드를 사용하여 API 키, 토큰 및 기타 자격 증명을 MCP 서버에 전달합니다:

<Tabs>
  <Tab title="코드에서">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "api-server": {
              command: "npx",
              args: ["-y", "@your-org/api-mcp-server"],
              env: {
                API_KEY: process.env.API_KEY
              }
            }
          },
          allowedTools: ["mcp__api-server__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "api-server": {
                  "command": "npx",
                  "args": ["-y", "@your-org/api-mcp-server"],
                  "env": {"API_KEY": os.environ["API_KEY"]},
              }
          },
          allowed_tools=["mcp__api-server__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "api-server": {
          "command": "npx",
          "args": ["-y", "@your-org/api-mcp-server"],
          "env": {
            "API_KEY": "${API_KEY}"
          }
        }
      }
    }
    ```

    `${API_KEY}` 구문은 런타임에 환경 변수를 확장합니다.
  </Tab>
</Tabs>

<h3 id="http-headers-for-remote-servers">
  원격 서버용 HTTP 헤더
</h3>

HTTP 및 SSE 서버의 경우 서버 구성에서 직접 인증 헤더를 전달합니다:

<Tabs>
  <Tab title="코드에서">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "secure-api": {
              type: "http",
              url: "https://api.example.com/mcp",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__secure-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "secure-api": {
                  "type": "http",
                  "url": "https://api.example.com/mcp",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__secure-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "secure-api": {
          "type": "http",
          "url": "https://api.example.com/mcp",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```

    `${API_TOKEN}` 구문은 런타임에 환경 변수를 확장합니다.
  </Tab>
</Tabs>

헤더로 인증된 원격 서버의 완전한 작동 예제는 [저장소에서 이슈 나열](#list-issues-from-a-repository)을 참조하십시오.

<h3 id="oauth2-authentication">
  OAuth2 인증
</h3>

[MCP 사양은 OAuth 2.1을 지원합니다](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization). SDK는 브라우저를 열거나 대화형 OAuth 흐름을 실행하지 않습니다. 구성된 서버가 인증 챌린지를 반환하고 저장된 토큰이 없을 때, 에이전트 실행은 해당 서버의 도구 없이 계속되며, 서버는 `needs-auth` 상태를 보고합니다. [시스템 초기화 메시지](/docs/ko/agent-sdk/typescript#sdksystemmessage)의 `mcp_servers` 배열은 내보낼 때 해당 서버에 대해 여전히 `pending`을 표시할 수 있습니다. 서버에 자격 증명이 필요한지 확인하려면 TypeScript SDK에서 `mcpServerStatus()`를 폴링하거나 Python에서 [`get_mcp_status()`](/docs/ko/agent-sdk/python#methods)를 폴링합니다.

자격 증명을 제공하려면 자신의 애플리케이션에서 OAuth 흐름을 완료하고 결과 액세스 토큰을 서버의 `headers`에 전달합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  // After completing OAuth flow in your app.
  // Implement getAccessTokenFromOAuthFlow for your OAuth provider.
  const accessToken = await getAccessTokenFromOAuthFlow();

  const options = {
    mcpServers: {
      "oauth-api": {
        type: "http",
        url: "https://api.example.com/mcp",
        headers: {
          Authorization: `Bearer ${accessToken}`
        }
      }
    },
    allowedTools: ["mcp__oauth-api__*"]
  };
  ```

  ```python Python theme={null}
  # After completing OAuth flow in your app.
  # Implement get_access_token_from_oauth_flow for your OAuth provider.
  access_token = await get_access_token_from_oauth_flow()

  options = ClaudeAgentOptions(
      mcp_servers={
          "oauth-api": {
              "type": "http",
              "url": "https://api.example.com/mcp",
              "headers": {"Authorization": f"Bearer {access_token}"},
          }
      },
      allowed_tools=["mcp__oauth-api__*"],
  )
  ```
</CodeGroup>

<h2 id="examples">
  예제
</h2>

<h3 id="list-issues-from-a-repository">
  저장소에서 이슈 나열
</h3>

이 예제는 원격 [GitHub MCP 서버](https://github.com/github/github-mcp-server)에 연결하여 최근 이슈를 나열합니다. 이 예제에는 MCP 연결 및 도구 호출을 확인하기 위한 디버그 로깅이 포함되어 있습니다.

실행하기 전에 쿼리하려는 저장소에 대한 읽기 액세스 권한이 있는 [GitHub 개인 액세스 토큰](https://github.com/settings/personal-access-tokens)을 생성하고 환경 변수로 설정합니다:

```bash theme={null}
export GITHUB_TOKEN=YOUR_GITHUB_PAT
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          type: "http",
          url: "https://api.githubcopilot.com/mcp/",
          headers: {
            Authorization: `Bearer ${process.env.GITHUB_TOKEN}`
          }
        }
      },
      allowedTools: ["mcp__github__list_issues"]
    }
  })) {
    // Verify MCP server connected successfully
    if (message.type === "system" && message.subtype === "init") {
      console.log("MCP servers:", message.mcp_servers);
    }

    // Log when Claude calls an MCP tool
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "tool_use" && block.name.startsWith("mcp__")) {
          console.log("MCP tool called:", block.name);
        }
      }
    }

    // Print the final result
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      SystemMessage,
      AssistantMessage,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "type": "http",
                  "url": "https://api.githubcopilot.com/mcp/",
                  "headers": {"Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )

      async for message in query(
          prompt="List the 3 most recent issues in anthropics/claude-code",
          options=options,
      ):
          # Verify MCP server connected successfully
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("MCP servers:", message.data.get("mcp_servers"))

          # Log when Claude calls an MCP tool
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "name") and block.name.startswith("mcp__"):
                      print("MCP tool called:", block.name)

          # Print the final result
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

`MCP servers:` 줄에서 `github`에 대한 `status`가 `connected`이면 토큰이 작동함을 확인합니다. Claude Code가 서버에 대한 [캐시된 도구 목록](#connection-timing)을 가지고 있으면 상태가 `pending`으로 표시될 수 있으며 서버는 첫 번째 도구 호출 시 연결됩니다. 상태가 `failed` 또는 `needs-auth`이면 결과를 신뢰하기 전에 [오류 처리](#error-handling)를 참조하십시오. Claude는 서버를 사용할 수 없을 때 기본 제공 도구로 폴백할 수 있기 때문입니다.

<h3 id="query-a-database">
  데이터베이스 쿼리
</h3>

이 예제는 [DBHub](https://github.com/bytebase/dbhub)를 사용하여 Postgres 데이터베이스를 쿼리합니다. 에이전트는 자동으로 데이터베이스 스키마를 검색하고 SQL 쿼리를 작성하며 결과를 반환합니다.

DBHub의 `execute_sql` 도구는 제한하지 않는 한 에이전트가 내보내는 모든 SQL(쓰기 포함)을 실행합니다. [DBHub 구성 파일](https://dbhub.ai/config/toml)에서 `readonly = true`를 설정하면 DBHub는 `INSERT`, `UPDATE`, `DELETE` 및 DDL 문을 거부하므로 에이전트가 쓰기를 내보내더라도 예제는 데이터를 수정할 수 없습니다. DBHub는 구성을 로드할 때 프로세스 환경에서 `${DATABASE_URL}`을 확인하므로 연결 문자열이 파일 외부에 유지됩니다. 스크립트 옆에 이 `dbhub.toml`을 생성합니다:

```toml dbhub.toml theme={null}
[[sources]]
id = "production"
dsn = "${DATABASE_URL}"

[[tools]]
name = "execute_sql"
source = "production"
readonly = true
```

그러면 스크립트는 연결 문자열을 직접 전달하는 대신 DBHub가 구성 파일을 가리키도록 합니다. 실행하기 전에 `DATABASE_URL` 환경 변수를 연결 문자열로 설정합니다. 자리 표시자 값을 자신의 데이터베이스 세부 정보로 바꿉니다:

```bash theme={null}
export DATABASE_URL=postgresql://user:password@localhost:5432/mydb
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // dbhub.toml sets readonly = true, so execute_sql rejects writes
          args: ["-y", "@bytebase/dbhub", "--config", "dbhub.toml"]
        }
      },
      allowedTools: ["mcp__postgres__execute_sql"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "postgres": {
                  "command": "npx",
                  # dbhub.toml sets readonly = true, so execute_sql rejects writes
                  "args": [
                      "-y",
                      "@bytebase/dbhub",
                      "--config",
                      "dbhub.toml",
                  ],
              }
          },
          allowed_tools=["mcp__postgres__execute_sql"],
      )

      # Natural language query - Claude writes the SQL
      async for message in query(
          prompt="How many users signed up last week? Break it down by day.",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  오류 처리
</h2>

MCP 서버는 여러 가지 이유로 연결에 실패할 수 있습니다. 서버 프로세스가 설치되지 않았거나, 자격 증명이 유효하지 않거나, 원격 서버에 도달할 수 없을 수 있습니다.

Claude Code는 각 쿼리의 시작 부분에서 서브타입이 `init`인 `system` 메시지를 내보냅니다. 이 메시지에는 각 MCP 서버의 연결 상태가 포함됩니다. `status` 필드는 `"pending"`, `"connected"`, `"failed"`, `"needs-auth"` 또는 `"disabled"`일 수 있습니다. Claude Code는 `options.mcpServers`에 전달된 서버에 대해 [첫 번째 턴 연결 대기](#connection-timing) 후에 init 메시지를 내보내므로, 대기 시간 내에 연결된 서버는 `"connected"`를 표시합니다.

init 메시지에서 `"pending"`을 그 자체로 실패로 취급하지 마십시오. 다음 중 하나를 의미할 수 있습니다.

* 서버가 아직 연결되지 않았습니다. [Claude Code가 첫 번째 턴 전에 얼마나 오래 기다리는지](#connection-timing) 참조하십시오.
* 서버의 도구 목록이 [캐시에서 제공](#connection-timing)되었으며, 첫 사용 시 연결이 이루어졌습니다.
* 연결 기한이 만료되었습니다. 이러한 서버는 타이밍에 따라 `"pending"` 또는 `"failed"`를 보고합니다.

사용할 수 없는 서버를 감지하려면 `"failed"` 또는 `"needs-auth"`를 확인하십시오.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Process data",
      options: {
        mcpServers: {
          // Replace dataServer with your server configuration
          "data-processor": dataServer
        }
      }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        const unavailableServers = message.mcp_servers.filter(
          (s) => s.status === "failed" || s.status === "needs-auth"
        );

        if (unavailableServers.length > 0) {
          console.warn("Unavailable MCP servers:", unavailableServers);
        }
      }

      if (message.type === "result" && message.subtype === "error_during_execution") {
        console.error("Execution failed");
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branch above has
    // already run; a failure to start or reach the Claude Code process
    // yields no result message. MCP servers that fail to connect don't
    // throw: use the status check above, and note that servers still
    // "pending" at init need a later status check.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      # Replace data_server with your server configuration
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      try:
          async for message in query(prompt="Process data", options=options):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  unavailable_servers = [
                      s
                      for s in message.data.get("mcp_servers", [])
                      if s.get("status") in ("failed", "needs-auth")
                  ]

                  if unavailable_servers:
                      print(f"Unavailable MCP servers: {unavailable_servers}")

              if (
                  isinstance(message, ResultMessage)
                  and message.subtype == "error_during_execution"
              ):
                  print("Execution failed")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branch above has
          # already run; a failure to start or reach the Claude Code process
          # yields no result message. MCP servers that fail to connect don't
          # raise: use the status check above, and note that servers still
          # "pending" at init need a later status check.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

원격 서버의 상태는 `"connected"`를 보고한 후에도 변경될 수 있습니다. 세션 중간에 연결이 끊어지면 Claude Code는 서버를 다시 `"pending"`으로 이동하면서 [재연결](/docs/ko/mcp#automatic-reconnection)합니다. 나중에 TypeScript에서 `mcpServerStatus()` 호출 또는 Python에서 [`ClaudeSDKClient.get_mcp_status()`](/docs/ko/agent-sdk/python#methods)를 호출하면 이전에 연결된 것으로 본 서버에 대해 `"pending"`을 보고할 수 있으며, 사용자 측에서 구성 변경이 없습니다.

5번의 재연결 시도가 실패한 후 서버는 `"failed"`를 보고하거나, 다시 인증이 필요할 때 `"needs-auth"`를 보고합니다. 수동으로 다시 시도하려면 TypeScript에서 [`reconnectMcpServer()`](/docs/ko/agent-sdk/typescript#methods)를 호출하거나 Python에서 [`ClaudeSDKClient.reconnect_mcp_server()`](/docs/ko/agent-sdk/python#methods)를 호출하십시오.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="server-shows-failed-status">
  서버가 "failed" 상태를 표시함
</h3>

`init` 메시지를 확인하여 어떤 서버가 연결에 실패했는지 확인하세요:

<CodeGroup>
  ```typescript TypeScript theme={null}
  if (message.type === "system" && message.subtype === "init") {
    for (const server of message.mcp_servers) {
      if (server.status === "failed") {
        console.error(`Server ${server.name} failed to connect`);
      }
    }
  }
  ```

  ```python Python theme={null}
  if isinstance(message, SystemMessage) and message.subtype == "init":
      for server in message.data.get("mcp_servers", []):
          if server.get("status") == "failed":
              print(f"Server {server['name']} failed to connect")
  ```
</CodeGroup>

`"pending"` 상태는 서버가 실패했다는 의미가 아닙니다. 초기화 시 적용되는 경우는 [오류 처리](#error-handling)를 참조하세요. 세션 후반부에 업데이트된 상태를 얻으려면 TypeScript SDK에서 쿼리의 `mcpServerStatus()` 메서드를 호출하거나, Python에서 [`ClaudeSDKClient.get_mcp_status()`](/docs/ko/agent-sdk/python#methods)를 호출하세요.

일반적인 원인:

* **누락된 환경 변수**: 필수 토큰과 자격 증명이 설정되어 있는지 확인하세요. stdio 서버의 경우 `env` 필드가 서버가 예상하는 것과 일치하는지 확인하세요.
* **서버가 설치되지 않음**: `npx` 명령의 경우 패키지가 존재하고 Node.js가 PATH에 있는지 확인하세요.
* **잘못된 연결 문자열**: 데이터베이스 서버의 경우 연결 문자열 형식을 확인하고 데이터베이스에 액세스할 수 있는지 확인하세요.
* **네트워크 문제**: 원격 HTTP/SSE 서버의 경우 URL에 도달할 수 있는지 확인하고 방화벽이 연결을 허용하는지 확인하세요.

<h3 id="tools-not-being-called">
  도구가 호출되지 않음
</h3>

Claude가 도구를 보지만 사용하지 않는 경우 `allowedTools`로 권한을 부여했는지 확인하세요:

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        // your servers
      },
      allowedTools: ["mcp__servername__*"] // Auto-approve calls from this server
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=["mcp__servername__*"],  # Auto-approve calls from this server
  )
  ```
</CodeGroup>

<h3 id="connection-timeouts">
  연결 시간 초과
</h3>

MCP 서버 연결은 기본적으로 30초 후 시간 초과됩니다. 실행 중인 도구 호출이 걸릴 수 있는 시간을 변경하려면 [`MCP_TOOL_TIMEOUT`](/docs/ko/env-vars)을 설정하세요. 서버가 시작하는 데 더 오래 걸리면 연결이 실패합니다. [`MCP_TIMEOUT`](/docs/ko/env-vars) 환경 변수(밀리초 단위)로 연결 제한을 높이세요. 더 많은 시작 시간이 필요한 서버의 경우 다음도 고려하세요:

* 사용 가능한 경우 더 가벼운 서버 사용
* 에이전트를 시작하기 전에 서버 사전 준비
* 느린 초기화 원인에 대한 서버 로그 확인

TypeScript에서는 [`timeout`을 `createSdkMcpServer()`에 전달](/docs/ko/agent-sdk/typescript#createsdkmcpserver)하여 단일 [SDK MCP 서버](#sdk-mcp-servers)에 대한 도구 호출 제한을 설정할 수 있습니다.

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  도구 출력이 최대 허용 토큰을 초과함
</h3>

SDK는 Claude Code와 동일한 MCP 출력 제한을 적용합니다. 이미지 콘텐츠가 없는 도구 결과가 25,000 토큰보다 크면 Claude Code는 출력을 파일에 저장하고 도구 결과를 파일 경로의 이름을 지정하는 오류 메시지로 바꾸므로 에이전트가 출력을 부분적으로 다시 읽을 수 있습니다.

[`MAX_MCP_OUTPUT_TOKENS`](/docs/ko/env-vars) 환경 변수로 제한을 높이세요. 서버가 `anthropic/maxResultSizeChars` 주석으로 더 높은 도구별 제한을 선언하는 방법을 포함한 전체 동작은 [MCP 출력 제한 및 경고](/docs/ko/mcp#mcp-output-limits-and-warnings)를 참조하세요.

<h2 id="related-resources">
  관련 리소스
</h2>

* **[사용자 정의 도구 가이드](/docs/ko/agent-sdk/custom-tools)**: SDK 애플리케이션과 함께 인프로세스로 실행되는 자신만의 MCP 서버를 구축합니다
* **[권한](/docs/ko/agent-sdk/permissions)**: `allowedTools` 및 `disallowedTools`를 사용하여 에이전트가 사용할 수 있는 MCP 도구를 제어합니다
* **[TypeScript SDK 참조](/docs/ko/agent-sdk/typescript)**: MCP 구성 옵션을 포함한 전체 API 참조
* **[Python SDK 참조](/docs/ko/agent-sdk/python)**: MCP 구성 옵션을 포함한 전체 API 참조
* **[MCP 서버 디렉토리](https://github.com/modelcontextprotocol/servers)**: 데이터베이스, API 등을 위한 사용 가능한 MCP 서버를 찾아봅니다
