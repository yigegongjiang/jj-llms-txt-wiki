> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Skills로 에이전트 확장하기

> Claude Agent SDK 세션에서 Claude가 호출할 수 있는 Skills를 제어하고, 이름으로 명령을 전달하며, 세션이 발견하는 Skills를 작성합니다

Agent Skills는 Claude가 관련성이 있을 때 호출하는 전문화된 기능으로 Claude를 확장합니다. Skills는 지침, 설명 및 선택적 지원 리소스를 포함하는 `SKILL.md` 파일로 패키징됩니다. 이 페이지는 또한 [Agent SDK 세션의 명령](#commands-in-agent-sdk-sessions)을 다룹니다.

Skills에 대한 이점, 아키텍처 및 작성 지침을 포함한 포괄적인 정보는 [Agent Skills 개요](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview)를 참조하십시오.

<h2 id="how-skills-work-with-the-agent-sdk">
  Agent SDK와 Skills의 작동 방식
</h2>

Claude Agent SDK를 사용할 때 Skills는 다음과 같이 작동합니다:

* **파일 시스템 아티팩트로 정의됨**: 각 Skill을 `.claude/skills/<name>/SKILL.md`와 같은 자신의 디렉토리에 `SKILL.md` 파일로 생성합니다
* **파일 시스템에서 로드됨**: SDK는 `settingSources`(TypeScript) 또는 `setting_sources`(Python)에 의해 관리되는 파일 시스템 위치에서 Skills를 로드합니다
* **자동으로 발견됨**: 파일 시스템 설정이 로드되면 SDK는 시작 시 사용자 및 프로젝트 디렉토리에서 Skill 메타데이터를 발견하고, Claude가 Skill을 호출할 때 전체 콘텐츠를 로드합니다
* **모델에 의해 호출됨**: Claude는 컨텍스트를 기반으로 자율적으로 사용할 시기를 선택합니다
* **사용자에 의해 호출됨**: 프롬프트에서 `/<name>`을 전송하여 Skill을 직접 전달합니다. [Agent SDK 세션의 명령](#commands-in-agent-sdk-sessions)을 참조하십시오
* **`skills` 옵션을 통해 범위 지정됨**: 발견된 Skills는 기본적으로 활성화됩니다. Skill 이름 목록, `"all"` 또는 `[]`를 전달하여 Claude가 호출할 수 있는 Skills를 제어합니다

Subagents와 달리 [`agents` 옵션](/docs/ko/agent-sdk/subagents#programmatic-definition-recommended)에서 정의할 수 있으며, Skills는 디스크에 파일로 생성합니다. SDK는 Skills를 등록하기 위한 프로그래밍 API를 제공하지 않습니다.

<Note>
  Skills는 파일 시스템 설정 소스를 통해 발견됩니다. 기본 `query()` 옵션을 사용하면 SDK는 사용자 및 프로젝트 소스를 로드하므로 `~/.claude/skills/`, `<cwd>/.claude/skills/` 및 `<cwd>`의 상위 디렉토리부터 저장소 루트까지의 `.claude/skills/`에 있는 Skills를 사용할 수 있습니다. 프로젝트 소스는 또한 `additionalDirectories`(TypeScript) 또는 `add_dirs`(Python)를 통해 전달하는 각 디렉토리의 `<dir>/.claude/skills/`를 포함합니다. SDK가 해당 디렉토리를 Claude Code에 [`--add-dir`](/docs/ko/skills#skills-from-additional-directories)로 전달하기 때문입니다. `settingSources`를 명시적으로 설정하는 경우 프로젝트 및 추가 디렉토리 Skills를 유지하려면 `'project'`를 포함하고 개인 Skills를 유지하려면 `'user'`를 포함하거나, [`plugins` 옵션](/docs/ko/agent-sdk/plugins)을 사용하여 특정 경로에서 Skills를 로드하십시오.
</Note>

<h2 id="use-skills-with-the-agent-sdk">
  Agent SDK와 함께 Skills 사용하기
</h2>

`query()`의 `skills` 옵션을 설정하여 세션에서 Claude가 호출할 수 있는 Skills를 제어합니다. 생략하면 발견된 Skills가 활성화되고 Skill 도구를 사용할 수 있으며, 이는 CLI 동작과 일치합니다. `"all"`을 전달하여 Claude가 모든 발견된 Skill을 호출하도록 하거나, Skill 이름 목록을 전달하여 해당 Skill만 허용하거나, `[]`를 전달하여 Claude가 어떤 Skill도 호출하지 않도록 합니다.

예를 들어 Claude가 두 개의 명명된 Skill만 호출하도록 하려면:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

<h3 id="set-up-skills-in-a-session">
  세션에서 Skills 설정하기
</h3>

`skills`를 설정하면 SDK가 Skill 도구를 `allowedTools`에 자동으로 추가합니다. 명시적 `tools` 목록도 전달하는 경우 Claude가 Skills를 호출할 수 있도록 해당 목록에 `"Skill"`을 포함하십시오.

구성되면 Claude는 파일 시스템에서 Skills를 자동으로 발견하고 사용자의 요청과 관련이 있을 때 호출합니다.

다음 예제는 모든 발견된 Skill을 활성화하고 Skills가 일반적으로 필요로 하는 도구를 사전 승인합니다. 예제는 `cwd`를 프로세스의 현재 작업 디렉토리로 설정하므로 현재 디렉토리 또는 저장소 루트까지의 상위 디렉토리에 `.claude/skills/` 디렉토리가 있는 프로젝트 내에서 실행하십시오:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd=os.getcwd(),  # .claude/skills/ here or in a parent directory
          setting_sources=["user", "project"],  # Load skills from filesystem
          skills="all",  # Let Claude invoke every discovered skill
          allowed_tools=["Read", "Write", "Bash"],
      )

      async for message in query(
          prompt="Help me process this PDF document", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me process this PDF document",
    options: {
      cwd: process.cwd(), // .claude/skills/ here or in a parent directory
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all", // Let Claude invoke every discovered skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="confirm-skills-loaded">
  Skills 로드 확인하기
</h3>

스트림의 시작 부분 근처에서 SDK는 서브타입 `init`이 있는 시스템 메시지를 생성합니다. 해당 `skills` 배열을 확인하여 Claude가 작업을 시작하기 전에 Skills가 로드되었는지 확인하십시오. 배열에는 정의한 사용자 호출 가능 Skills와 [Claude Code에 포함된 번들 Skills](/docs/ko/skills#bundled-skills)가 포함됩니다.

배열은 사용자 호출 가능 Skills만 나열합니다. frontmatter에 [`user-invocable: false`](/docs/ko/skills#control-who-invokes-a-skill)가 있는 Skill은 로드되고 Claude에서 사용 가능하지만 배열에 나타나지 않습니다. 배열은 세션이 발견한 것을 반영하고 `skills` 목록에 있는지 여부와 관계없이 동일한 Skills를 나열합니다.

<h3 id="allow-only-specific-skills">
  특정 Skills만 허용하기
</h3>

Claude가 특정 Skills만 호출하도록 하려면 해당 이름을 `skills` 목록에 전달합니다. 이름은 `SKILL.md`의 `name` 필드 또는 Skill의 디렉토리 이름과 일치합니다. 플러그인에서 제공하는 Skills의 경우 `plugin:skill`을 사용합니다.

목록은 정확한 Skill 이름만 사용합니다. 항목이 정확한 이름으로 작동할 수 없으면 `query()`는 세션이 시작되기 전에 목록을 거부합니다. [Invalid skill name error](#invalid-skill-name-error)에서 이름 규칙과 각 SDK가 발생시키는 오류를 참조하십시오.

모델은 나열되지 않은 Skills를 보지 못하고 Skill 도구가 이를 거부하지만, 해당 파일은 디스크에 남아 있으며 Read 및 Bash를 통해 접근할 수 있습니다. 목록을 제한해도 [이름으로 전달](#dispatch-commands-by-name)을 제한하지 않습니다.

모든 발견된 Skill을 호출하도록 하려면 와일드카드 대신 `skills: "all"`을 전달하십시오.

<h2 id="commands-in-agent-sdk-sessions">
  Agent SDK 세션의 명령
</h2>

이 섹션은 SDK의 명령 문서입니다. 명령은 프롬프트에서 `/<name>`을 전송하여 실행하는 모든 것입니다. 명령 표면의 항목은 이를 지원하는 것이 다릅니다:

* **기본 제공 명령**: SDK가 실행하는 Claude Code 프로세스에 코딩된 로직을 실행합니다. 예를 들어 `/compact`
* **번들 Skills**: Claude Code에 포함된 프롬프트 아티팩트입니다. 예를 들어 `/code-review`
* **사용자 Skills**: 사용자가 작성하는 프롬프트 아티팩트이며, 각각 `SKILL.md` 파일을 보유하는 디렉토리입니다. 사용자 호출 가능 Skill의 이름이 표면에 자동으로 조인되므로 자신의 `/security-check`를 전달하고 기본 제공을 실행하는 것이 동일한 방식으로 작동합니다
* **사용자 정의 명령 파일**: 동일한 동작을 하는 이전 아티팩트 형식이며, `.claude/commands/`의 평면 Markdown 파일이며 파일 이름이 명령 이름이 됩니다. Skills는 권장되는 후속입니다

기본적으로 사용자와 Claude 모두 모든 Skill을 호출할 수 있습니다. Skill의 [frontmatter](/docs/ko/skills#control-who-invokes-a-skill)를 통해 각 경로를 제한할 수 있습니다. 두 용어의 정의는 용어집의 [Command](/docs/ko/glossary#command) 및 [Skill](/docs/ko/glossary#skill) 항목을 참조하십시오. [Claude Code의 명령](/docs/ko/commands)에서 모든 기본 제공 명령을 참조하고 [Skills로 Claude 확장하기](/docs/ko/skills)에서 두 아티팩트 형식의 완전한 가이드를 참조하십시오.

<h3 id="discover-available-commands">
  사용 가능한 명령 발견하기
</h3>

SDK를 통해 대화형 터미널 없이 작동하는 명령을 전달할 수 있습니다. `system/init` 메시지는 `slash_commands` 필드의 세션에서 사용 가능한 명령을 나열합니다. `/theme` 및 `/terminal-setup`과 같이 대화형 터미널이 필요한 명령은 목록에 나타나지 않습니다. 세션이 시작될 때 필드에 액세스합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello Claude",
    options: { maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available commands:", message.slash_commands);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(prompt="Hello Claude", options=ClaudeAgentOptions(max_turns=1)):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available commands:", message.data["slash_commands"])


  asyncio.run(main())
  ```
</CodeGroup>

인쇄된 목록은 기본 제공 명령, 번들 Skills, 사용자 호출 가능 Skills 및 `.claude/commands/` 파일을 혼합합니다:

```text theme={null}
Available commands: ["clear", "compact", "context", "usage", "code-review", "verify", "security-check", ...]
```

frontmatter에 [`user-invocable: false`](/docs/ko/skills#control-who-invokes-a-skill)가 있는 Skill은 이 목록이나 [Skills 로드 확인하기](#confirm-skills-loaded)의 `skills` 배열에 나타나지 않습니다. [MCP 서버](/docs/ko/agent-sdk/mcp)를 구성하는 세션은 또한 [MCP 프롬프트를 명령으로 노출](/docs/ko/mcp#use-mcp-prompts-as-commands)할 수 있습니다.

<h3 id="dispatch-commands-by-name">
  이름으로 명령 전달하기
</h3>

프롬프트 문자열에 명령을 포함하여 일반 텍스트를 전송하는 것과 동일한 방식으로 명령을 전송합니다. 전달은 `skills` 옵션에 따라 달라지지 않습니다. `/<name>`을 전송하면 `skills` 목록이 이를 생략할 때도 사용자 호출 가능 Skill을 실행합니다. `/compact`와 같이 대화 기록에 작용하는 명령은 작업할 이전 메시지가 필요합니다.

`/<name>`이 세션의 명령이나 기본 제공 Claude Code 명령과 일치하지 않으면 쿼리가 실패하지 않습니다. Claude Code는 프롬프트를 Claude에 일반 메시지로 전송하며, 명령이 실행되지 않았다는 메모를 포함하므로 쿼리는 모델 턴을 소비하고 Claude의 응답을 반환합니다. v2.1.274 이전에는 아무것도 일치하지 않는 `/<name>`이 모델 턴 없이 `Unknown command: /<name>`을 결과로 반환했습니다.

세션에서 사용할 수 없는 `/theme`과 같은 기본 제공 Claude Code 명령과 일치하는 `/<name>`은 모델 턴 없이 `/theme isn't available in this environment.`를 결과로 반환합니다.

<Note>
  명령은 다른 프롬프트처럼 `maxTurns` / `max_turns` 제한에 도달할 수 있으며, `success` 대신 오류 결과로 쿼리를 종료합니다. 오류 결과 계약은 [결과 처리](/docs/ko/agent-sdk/agent-loop#handle-the-result)를 참조하십시오. 명령이 제한에 도달할 수 있으면 TypeScript에서 `try`/`catch`로 루프를 래핑하거나 Python에서 `try`/`except`로 래핑하십시오. [단일 메시지 입력](/docs/ko/agent-sdk/streaming-vs-single-mode#single-message-input)에 표시된 대로 또는 작업이 완료될 수 있도록 `maxTurns`를 충분히 높게 설정하십시오.
</Note>

<h3 id="compact-history-with-/compact">
  `/compact`로 기록 압축하기
</h3>

`/compact` 명령은 이전 메시지를 요약하면서 중요한 컨텍스트를 보존하여 대화 기록의 크기를 줄입니다. 압축은 요약할 충분한 이전 메시지가 있는 기존 대화가 필요합니다. 이 예제는 먼저 대화를 한 다음 압축하고 결과를 보고하는 `compact_boundary` 시스템 메시지를 읽습니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Compaction needs existing history, so have a conversation first
  try {
    for await (const message of query({
      prompt: "Explain what this project does",
      options: { maxTurns: 2 }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the follow-up query below still runs.
    console.error(`Session ended with an error: ${error}`);
  }

  // Compact the same conversation
  for await (const message of query({
    prompt: "/compact",
    options: { continue: true, maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "compact_boundary") {
      console.log("Compaction completed");
      console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
      console.log("Trigger:", message.compact_metadata.trigger);
      // Example output:
      // Compaction completed
      // Pre-compaction tokens: 1842
      // Trigger: manual
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage, SystemMessage


  async def main():
      # Compaction needs existing history, so have a conversation first
      try:
          async for message in query(
              prompt="Explain what this project does",
              options=ClaudeAgentOptions(max_turns=2),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the follow-up query below still runs.
          print(f"Session ended with an error: {error}")

      # Compact the same conversation
      async for message in query(
          prompt="/compact",
          options=ClaudeAgentOptions(continue_conversation=True, max_turns=1),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "compact_boundary":
              print("Compaction completed")
              print("Pre-compaction tokens:", message.data["compact_metadata"]["pre_tokens"])
              print("Trigger:", message.data["compact_metadata"]["trigger"])
              # Example output:
              # Compaction completed
              # Pre-compaction tokens: 1842
              # Trigger: manual


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  `compact_boundary` 메시지는 압축이 실행되었을 때만 도착합니다. 요약할 것이 없으면 `/compact`는 대신 이유를 보고합니다. 실행은 여전히 `success` 결과로 끝나고 `compact_boundary` 메시지가 없으며, 결과 텍스트는 이유를 전달합니다. 예를 들어 짧은 교환 후 `Not enough messages to compact.`입니다. 새로운 원샷 `query()` 호출은 빈 컨텍스트로 시작하므로 이 패턴을 이전 턴이 있는 세션에서 사용하십시오. 예를 들어 [스트리밍 입력 모드](/docs/ko/agent-sdk/streaming-vs-single-mode)에서 또는 세션을 재개할 때입니다.
</Note>

<h3 id="reset-context-with-/clear">
  `/clear`로 컨텍스트 재설정하기
</h3>

`/clear` 명령은 대화를 빈 컨텍스트로 재설정하므로 후속 프롬프트는 이전 대화 기록 없이 시작합니다. 이전 대화는 디스크에 남아 있습니다. [`resume` 옵션](/docs/ko/agent-sdk/sessions#resume-by-id)에 세션 ID를 전달하여 해당 대화로 돌아갈 수 있습니다.

`/clear`는 여러 프롬프트를 단일 연결을 통해 전송하는 [스트리밍 입력 모드](/docs/ko/agent-sdk/streaming-vs-single-mode)에서 유용합니다. 원샷 `query()` 호출의 경우 각 호출은 이미 빈 컨텍스트로 시작하므로 `/clear`를 전송하는 것은 실질적인 효과가 없습니다. 대신 새로운 `query()`를 시작하십시오.

<h2 id="create-skills">
  Skills 생성하기
</h2>

각 Skill을 YAML frontmatter 및 Markdown 콘텐츠가 포함된 `SKILL.md` 파일을 포함하는 디렉토리로 생성합니다. `description` 필드는 Claude가 Skill을 호출하는 시기를 결정합니다.

**예제 디렉토리 구조**:

```text theme={null}
.claude/skills/security-check/
└── SKILL.md
```

<h3 id="choose-a-discovery-level">
  발견 수준 선택하기
</h3>

두 가지 가장 일반적인 [발견 수준](/docs/ko/skills#where-skills-live)에서 Skills를 저장합니다:

* **프로젝트 Skills**: `.claude/skills/`, 현재 프로젝트에서만 사용 가능
* **개인 Skills**: `~/.claude/skills/`, 모든 프로젝트에서 사용 가능

`.claude/commands/`에 기존 사용자 정의 명령 파일이 있으면 계속 작동합니다. `.claude/commands/deploy.md`의 명령 파일은 `/deploy`를 생성하고 `.claude/skills/deploy/SKILL.md`의 Skill과 동일한 방식으로 작동합니다. 명령 파일과 Skill이 이름을 공유하면 [이름을 공유하는 Skills 해결하기](/docs/ko/skills#resolve-skills-that-share-a-name)를 참조하여 어느 것이 실행되는지 확인하십시오. SDK는 Skills와 동일한 두 범위에서 `.claude/commands/` 및 `~/.claude/commands/` 파일을 로드합니다. 두 아티팩트 형식의 완전한 가이드는 [Skills로 Claude 확장하기](/docs/ko/skills)를 참조하십시오.

<h3 id="create-and-dispatch-your-first-skill">
  첫 번째 Skill 생성 및 전달하기
</h3>

전체 흐름을 보려면 `.claude/skills/security-check/SKILL.md`를 생성합니다:

```markdown theme={null}
---
name: security-check
description: Run a security vulnerability scan
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

파일이 존재하면 Skill은 SDK를 통해 사용 가능합니다. Claude는 요청이 설명과 일치할 때 호출하고, 프롬프트에서 `/<name>`을 전송하여 직접 전달할 수 있습니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "/security-check",
    options: { maxTurns: 10 }
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
      async for message in query(
          prompt="/security-check", options=ClaudeAgentOptions(max_turns=10)
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

성공적인 실행은 텍스트가 스캔 결과를 전달하는 `success` 결과로 끝납니다. 시드된 문제가 있는 작은 Express 앱에 대해 결과 텍스트는 다음과 같이 시작합니다:

```text theme={null}
**Security scan of `app.js` — 4 findings (most severe first):**

1. **SQL Injection** (line 8) — `req.query.name` is concatenated directly into the SQL string. Trivially exploitable (`' OR '1'='1`, `'; DROP TABLE users;--`). **Fix:** use parameterized queries, e.g. `db.query("SELECT * FROM users WHERE name = ?", [req.query.name], cb)`.
...
```

Skill의 이름은 또한 init 메시지의 `slash_commands` 배열에 나타납니다.

<Note>
  Claude Code는 번들 `code-review` 및 `verify` Skills를 포함합니다. `.claude/commands/` 파일의 이름을 그 중 하나로 지정하면, 예를 들어 `.claude/commands/code-review.md`, 파일의 명령이 번들 Skill을 섀도우하고 `slash_commands`는 이름을 한 번 나열합니다.
</Note>

<h2 id="pre-approve-tools-for-skills">
  Skills를 위한 도구 사전 승인하기
</h2>

<Note>
  프로젝트 및 개인 Skills의 경우 Claude Code는 SDK 세션에서 [`allowed-tools`](/docs/ko/skills#pre-approve-tools-for-a-skill) frontmatter 필드를 적용합니다. 쿼리 구성의 `allowedTools` 옵션(`allowed_tools` in Python)을 통해 이러한 Skills를 위한 도구를 사전 승인할 수도 있습니다. [claude.ai에서 동기화된](/docs/ko/skills#how-claude-code-handles-the-frontmatter-of-a-synced-skill) Skills는 자신의 frontmatter 규칙을 따릅니다.
</Note>

Skills는 세션의 도구로 실행됩니다. 아래 예제는 `allowedTools`(`allowed_tools` in Python)를 사용하여 `Read`, `Grep` 및 `Glob`을 사전 승인하므로 Claude는 [security-check Skill](#create-and-dispatch-your-first-skill)을 실행하는 동안 승인을 기다리지 않고 파일을 검사할 수 있습니다:

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )


  async def main():
      async for message in query(prompt="Check this project for security issues", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Check this project for security issues",
    options: {
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

스트림에서 Skill 호출은 Skill 도구 사용으로 나타나고, 그 뒤에 프로젝트 파일에 대한 Read 호출이 나타납니다. 실행은 텍스트가 결과를 전달하는 `success` 결과로 끝납니다.

목록은 다른 것을 제한하기보다는 명명된 도구를 사전 승인합니다. 권한 모드 및 `canUseTool` 콜백을 포함한 전체 권한 흐름은 [권한](/docs/ko/agent-sdk/permissions)을 참조하십시오.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="skills-not-found">
  Skills를 찾을 수 없음
</h3>

**settingSources 구성 확인**: SDK는 `user` 및 `project` 설정 소스를 통해 Skills를 발견합니다. `settingSources`/`setting_sources`를 명시적으로 설정하고 해당 소스를 생략하면 SDK는 Skills를 로드하지 않습니다:

<CodeGroup>
  ```python Python theme={null}
  # Skills not loaded: setting_sources excludes user and project
  options = ClaudeAgentOptions(setting_sources=[], skills="all")

  # Skills loaded: user and project sources included
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Skills not loaded: settingSources excludes user and project
  const optionsWithoutSkills = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const optionsWithSkills = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

각 소스가 로드하는 Skill 디렉토리는 [파일 시스템 소스 테이블](/docs/ko/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)을 참조하십시오. `settingSources`/`setting_sources`에 대한 자세한 내용은 [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript#settingsource) 또는 [Python SDK 참조](/docs/ko/agent-sdk/python#settingsource)를 참조하십시오.

**작업 디렉토리 확인**: SDK는 `cwd` 옵션의 `.claude/skills/` 및 저장소 루트까지의 모든 상위 디렉토리에서 Skills를 로드합니다. `cwd`가 `.claude/skills/`를 포함하는 디렉토리를 가리키거나 그 아래에 있으며, 동일한 저장소 내에 있는지 확인하십시오:

<CodeGroup>
  ```python Python theme={null}
  # Ensure your cwd points to the directory containing .claude/skills/
  options = ClaudeAgentOptions(
      cwd="/path/to/project",  # .claude/skills/ here or in a parent directory
      setting_sources=["user", "project"],  # Loads skills from these sources
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Ensure your cwd points to the directory containing .claude/skills/
  const options = {
    cwd: "/path/to/project", // .claude/skills/ here or in a parent directory
    settingSources: ["user", "project"], // Loads skills from these sources
    skills: "all"
  };
  ```
</CodeGroup>

[Agent SDK와 함께 Skills 사용하기](#use-skills-with-the-agent-sdk)를 참조하여 완전한 패턴을 확인하십시오.

**파일 시스템 위치 확인**:

```bash theme={null}
# Check project skills
ls .claude/skills/*/SKILL.md

# Check personal skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill이 사용되지 않음
</h3>

**`skills` 옵션 확인**: `skills` 목록을 전달한 경우 Skill의 이름이 포함되어 있는지 확인하십시오. Claude가 나열되지 않은 Skill을 호출하려고 하면 Skill 도구는 `Skill <name> is not in this session's skills allowlist`를 반환합니다. 목록에 이름을 추가하거나 프롬프트에서 `/<name>`을 전송하여 Skill을 직접 전달하십시오. 이는 나열 없이 작동합니다.

**설명 확인**: 구체적이고 관련 키워드를 포함하는지 확인하십시오. 효과적인 설명 작성에 대한 지침은 [Agent Skills 모범 사례](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions)를 참조하십시오.

<h3 id="invalid-skill-name-error">
  Invalid skill name error
</h3>

`skills` 목록의 이름이 정확한 Skill 이름으로 작동할 수 없으면 `query()`는 Claude Code 프로세스를 시작하기 전에 목록을 거부합니다. 거부를 트리거하는 이름은 다음을 포함합니다:

* 빈 이름
* 괄호, 쉼표 또는 제어 문자를 포함하는 이름
* 공백으로 채워진 이름
* 맨 `*` 또는 `:*` 접미사와 같은 와일드카드 형식

각 SDK는 거부를 다르게 표시합니다:

<Tabs>
  <Tab title="TypeScript">
    TypeScript SDK는 항목이 위반한 규칙을 명시하는 `Error`를 발생시킵니다. 예를 들어 `skills: ["docs:*"]`는 다음을 발생시킵니다:

    ```text theme={null}
    Invalid skill name "docs:*": wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    빈 이름은 `Skill names must be non-empty strings.`를 보고합니다.

    TypeScript Agent SDK 0.3.221 이전에는 SDK가 이 검사를 실행하지 않았습니다.
  </Tab>

  <Tab title="Python">
    Python SDK는 항목이 위반한 규칙을 명시하는 `ValueError`를 발생시킵니다. 예를 들어 `skills=["docs:*"]`는 다음을 발생시킵니다:

    ```text theme={null}
    ValueError: Invalid skill name 'docs:*': wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    빈 이름은 `Skill names must be non-empty strings`를 보고합니다.

    Python Agent SDK 0.2.129 이전에는 SDK가 이 검사를 실행하지 않았습니다.
  </Tab>
</Tabs>

<h3 id="additional-troubleshooting">
  추가 문제 해결
</h3>

YAML 구문 오류 및 디버깅과 같은 일반적인 Skills 문제 해결은 [Claude Code Skills 문제 해결 섹션](/docs/ko/skills#troubleshooting)을 참조하십시오.

<h2 id="next-steps">
  다음 단계
</h2>

[Claude Code Skills 가이드](/docs/ko/skills)는 심층적인 작성을 다룹니다. 해당 지침은 SDK 세션에 적용됩니다. 다음 섹션부터 시작하십시오:

* [Frontmatter 참조](/docs/ko/skills#frontmatter-reference): 지원되는 모든 필드
* [Skills에 인수 전달하기](/docs/ko/skills#pass-arguments-to-skills): `$ARGUMENTS`, `$0`, `$1` 및 Skill 스택. [전체 대체 테이블](/docs/ko/skills#available-string-substitutions)은 명명된 인수 및 `${CLAUDE_*}` 변수를 추가합니다
* [동적 컨텍스트 주입하기](/docs/ko/skills#inject-dynamic-context): Claude가 Skill 콘텐츠를 보기 전에 실행되는 `` !`command` `` 라인
* [Skills가 로드되는 위치 선택하기](/docs/ko/skills#where-skills-live): 모든 Skill 위치, 플러그인 네임스페이싱 및 두 개가 이름을 공유할 때 어느 것이 실행되는지

<h2 id="related-resources">
  관련 리소스
</h2>

* [Claude Code의 명령](/docs/ko/commands): 모든 기본 제공을 포함한 전체 명령 표면
* [Agent Skills 개요](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview): 개념적 개요, 이점 및 아키텍처
* [Agent Skills 모범 사례](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): 효과적인 Skills를 위한 작성 지침
* [Agent Skills 쿡북](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): 예제 Skills 및 템플릿
* [SDK의 Subagents](/docs/ko/agent-sdk/subagents): 프로그래밍 옵션이 있는 유사한 파일 시스템 기반 에이전트
* [SDK 개요](/docs/ko/agent-sdk/overview): 일반 SDK 개념
* [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript): 완전한 API 문서
* [Python SDK 참조](/docs/ko/agent-sdk/python): 완전한 API 문서
