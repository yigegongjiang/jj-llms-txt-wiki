> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# SDK의 플러그인

> Agent SDK를 통해 스킬, 에이전트, 훅 및 MCP 서버를 추가하여 Claude Code를 확장하는 사용자 정의 플러그인 로드

플러그인을 사용하면 프로젝트 전체에서 공유할 수 있는 사용자 정의 기능으로 Claude Code를 확장할 수 있습니다. Agent SDK를 통해 로컬 디렉터리에서 플러그인을 프로그래밍 방식으로 로드하여 에이전트 세션에 기능을 추가할 수 있습니다. 플러그인은 다음을 포함할 수 있습니다:

* **Skills**: Claude가 관련성이 있을 때 자율적으로 호출하는 기능입니다. `/plugin-name:skill-name`으로 플러그인 스킬을 직접 호출할 수도 있습니다.
* **Agents**: 특정 작업을 위한 전문화된 서브에이전트
* **Hooks**: 도구 사용 및 기타 이벤트에 응답하는 이벤트 핸들러
* **MCP servers**: Model Context Protocol을 통한 외부 도구 통합

플러그인 구조 및 플러그인 생성 방법에 대한 완전한 정보는 [플러그인](/docs/ko/plugins/overview)을 참조하십시오.

<h2 id="loading-plugins">
  플러그인 로드
</h2>

옵션 구성에서 로컬 파일 시스템 경로를 제공하여 플러그인을 로드합니다. `type` 필드는 `"local"`이어야 하며, 이는 SDK가 허용하는 유일한 값입니다. SDK는 다양한 위치에서 여러 플러그인을 로드하는 것을 지원합니다.

[마켓플레이스](/docs/ko/plugins/overview)를 통해 배포되거나 원격 저장소에서 플러그인을 사용하려면 먼저 다운로드한 후 로컬 디렉터리 경로를 제공합니다. 플러그인이 필요한 디렉터리 레이아웃은 아래의 [플러그인 구조 참조](#plugin-structure-reference)를 참조하십시오.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello",
    options: {
      plugins: [
        { type: "local", path: "./my-plugin" },
        { type: "local", path: "/absolute/path/to/another-plugin" }
      ]
    }
  })) {
    // Plugin commands, agents, and other features are now available
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              plugins=[
                  {"type": "local", "path": "./my-plugin"},
                  {"type": "local", "path": "/absolute/path/to/another-plugin"},
              ]
          ),
      ):
          # Plugin commands, agents, and other features are now available
          pass


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="path-specifications">
  경로 사양
</h3>

플러그인 경로는 다음과 같을 수 있습니다:

* **상대 경로**: `cwd` 옵션을 기준으로 확인됨(예: `"./plugins/my-plugin"`)
* **절대 경로**: 전체 파일 시스템 경로(예: `"/home/user/plugins/my-plugin"`)

<Note>
  경로는 플러그인의 루트 디렉터리를 가리켜야 합니다: `skills/`, `agents/`, `hooks/`, `commands/` 또는 `.claude-plugin/`의 상위 디렉터리입니다.
</Note>

<h2 id="verifying-plugin-installation">
  플러그인 설치 확인
</h2>

플러그인이 성공적으로 로드되면 시스템 초기화 메시지에 나타납니다. 플러그인을 사용할 수 있는지 확인할 수 있습니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello",
    options: {
      plugins: [{ type: "local", path: "./my-plugin" }]
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      // 로드된 플러그인 확인
      console.log("Plugins:", message.plugins);
      // 예: [{ name: "my-plugin", path: "/absolute/path/to/my-plugin" }]

      // 플러그인 스킬은 플러그인 이름을 접두사로 하여 나타납니다
      console.log("Skills:", message.skills);
      // 예: ["my-plugin:greet"]

      // 플러그인 명령어는 동일한 접두사를 사용하며, 스킬도 여기에 나타납니다
      console.log("Commands:", message.slash_commands);
      // 예: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              plugins=[{"type": "local", "path": "./my-plugin"}]
          ),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              # 로드된 플러그인 확인
              print("Plugins:", message.data.get("plugins"))
              # 예: [{"name": "my-plugin", "path": "/absolute/path/to/my-plugin"}]

              # 플러그인 스킬은 플러그인 이름을 접두사로 하여 나타납니다
              print("Skills:", message.data.get("skills"))
              # 예: ["my-plugin:greet"]

              # 플러그인 명령어는 동일한 접두사를 사용하며, 스킬도 여기에 나타납니다
              print("Commands:", message.data.get("slash_commands"))
              # 예: ["compact", "context", "my-plugin:custom-command", "my-plugin:greet"]


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="use-plugin-skills">
  플러그인 스킬 사용
</h2>

플러그인의 스킬은 충돌을 피하기 위해 플러그인 이름으로 자동 네임스페이스됩니다. 직접 호출하려면 프롬프트로 `/plugin-name:skill-name`을 전송하십시오.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Load a plugin with a custom /greet skill
  for await (const message of query({
    prompt: "/my-plugin:greet", // Use plugin skill with namespace
    options: {
      plugins: [{ type: "local", path: "./my-plugin" }]
    }
  })) {
    // Claude executes the custom greeting skill from the plugin
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, TextBlock


  async def main():
      # Load a plugin with a custom /greet skill
      async for message in query(
          prompt="/my-plugin:greet",  # Use plugin skill with namespace
          options=ClaudeAgentOptions(
              plugins=[{"type": "local", "path": "./my-plugin"}]
          ),
      ):
          # Claude executes the custom greeting skill from the plugin
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if isinstance(block, TextBlock):
                      print(f"Claude: {block.text}")


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  CLI를 통해 플러그인을 설치한 경우(예: `/plugin install my-plugin@marketplace`), SDK에서 설치 경로를 제공하여 사용할 수 있습니다. CLI로 설치된 플러그인은 `~/.claude/plugins/`에서 확인하십시오.
</Note>

<h2 id="complete-example">
  완전한 예제
</h2>

플러그인 로드 및 사용을 보여주는 전체 예제입니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import { fileURLToPath } from "node:url";

  async function runWithPlugin() {
    const pluginPath = fileURLToPath(new URL("./plugins/my-plugin", import.meta.url));

    console.log("Loading plugin from:", pluginPath);

    for await (const message of query({
      prompt: "What custom commands do you have available?",
      options: {
        plugins: [{ type: "local", path: pluginPath }],
        maxTurns: 3
      }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        console.log("Loaded plugins:", message.plugins);
        console.log("Available skills:", message.skills);
        console.log("Available commands:", message.slash_commands);
      }

      if (message.type === "assistant") {
        console.log("Assistant:", message.message.content);
      }
    }
  }

  runWithPlugin().catch(console.error);
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  """Example demonstrating how to use plugins with the Agent SDK."""

  import asyncio
  from pathlib import Path

  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeAgentOptions,
      SystemMessage,
      TextBlock,
      query,
  )


  async def run_with_plugin():
      """Example using a custom plugin."""
      plugin_path = Path(__file__).parent / "plugins" / "my-plugin"

      print(f"Loading plugin from: {plugin_path}")

      options = ClaudeAgentOptions(
          plugins=[{"type": "local", "path": str(plugin_path)}],
          max_turns=3,
      )

      async for message in query(
          prompt="What custom commands do you have available?", options=options
      ):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print(f"Loaded plugins: {message.data.get('plugins')}")
              print(f"Available skills: {message.data.get('skills')}")
              print(f"Available commands: {message.data.get('slash_commands')}")

          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if isinstance(block, TextBlock):
                      print(f"Assistant: {block.text}")


  if __name__ == "__main__":
      asyncio.run(run_with_plugin())
  ```
</CodeGroup>

<h2 id="plugin-structure-reference">
  플러그인 구조 참조
</h2>

플러그인 디렉터리는 일반적으로 `.claude-plugin/plugin.json` 매니페스트 파일을 포함합니다. 매니페스트는 선택사항입니다. 생략하면 Claude Code가 디렉터리 레이아웃에서 구성 요소를 자동으로 검색합니다. 디렉터리는 다음을 포함할 수 있습니다:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # 플러그인 매니페스트 (선택사항, 없어도 구성 요소 자동 검색됨)
├── skills/                   # Agent Skills (자율적으로 호출되거나 /plugin-name:skill-name을 통해 호출됨)
│   └── my-skill/
│       └── SKILL.md
├── commands/                 # 스킬을 평면 .md 파일로
│   └── custom-cmd.md
├── agents/                   # 사용자 정의 에이전트
│   └── specialist.md
├── hooks/                    # 이벤트 핸들러
│   └── hooks.json
└── .mcp.json                # MCP 서버 정의
```

<Note>
  `commands/` 디렉터리는 스킬을 평면 Markdown 파일로 보유합니다. 새로운 플러그인의 경우 `skills/`를 사용하십시오. Claude Code는 두 위치를 모두 지원합니다.
</Note>

<h2 id="multiple-plugin-sources">
  여러 플러그인 소스
</h2>

다양한 위치의 플러그인을 결합합니다:

```typescript theme={null}
import * as os from "node:os";
import * as path from "node:path";

plugins: [
  { type: "local", path: "./local-plugin" },
  {
    type: "local",
    path: path.join(os.homedir(), ".claude", "custom-plugins", "shared-plugin")
  }
];
```

<Note>
  SDK는 `~/plugins`와 같은 틸드 경로를 확장하지 않습니다. 플러그인 경로가 존재하지 않으면 SDK는 해당 플러그인을 건너뛰고 세션이 계속되므로, 초기화 메시지의 `plugins` 목록을 확인하여 각 플러그인이 로드되었는지 확인하십시오.
</Note>

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="plugin-not-loading">
  플러그인이 로드되지 않음
</h3>

플러그인이 초기화 메시지에 나타나지 않으면:

1. **경로 확인**: 경로가 플러그인 루트 디렉터리를 가리키는지 확인합니다. 이는 `skills/`, `agents/`, `hooks/`, `commands/` 또는 `.claude-plugin/`의 상위 디렉터리입니다
2. **plugin.json 검증**: 플러그인에 매니페스트가 포함되어 있으면 유효한 JSON 구문을 가지고 있는지 확인합니다
3. **파일 권한 확인**: 플러그인 디렉터리를 읽을 수 있는지 확인합니다
4. **디렉터리 존재 확인**: SDK는 존재하지 않는 경로를 건너뛰며, 플러그인이 초기화 메시지의 `plugins` 목록에 나타나지 않습니다

<h3 id="skills-not-appearing">
  스킬이 나타나지 않음
</h3>

플러그인 스킬이 작동하지 않으면:

1. **네임스페이스 사용**: `/plugin-name:skill-name`으로 플러그인 스킬을 호출합니다
2. **초기화 메시지 확인**: 스킬이 올바른 네임스페이스와 함께 `skills` 목록에 나타나는지 확인합니다
3. **스킬 파일 검증**: 각 스킬이 `skills/` 아래의 자체 하위 디렉터리에 `SKILL.md` 파일을 가지고 있는지 확인합니다. 예를 들어 `skills/my-skill/SKILL.md`입니다

<h2 id="see-also">
  참고 항목
</h2>

* [플러그인](/docs/ko/plugins/overview) - 완전한 플러그인 개발 가이드
* [플러그인 참조](/docs/ko/plugins/manifest-reference) - 기술 사양
* [명령어](/docs/ko/agent-sdk/skills#dispatch-commands-by-name) - SDK에서 명령어 디스패치
* [서브에이전트](/docs/ko/agent-sdk/subagents) - 전문화된 에이전트 작업
* [스킬](/docs/ko/agent-sdk/skills) - Agent Skills 사용
