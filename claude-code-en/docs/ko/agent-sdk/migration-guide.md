> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Agent SDK로 마이그레이션

> Claude Code TypeScript 및 Python SDK를 Claude Agent SDK로 마이그레이션하기 위한 가이드

<h2 id="overview">
  개요
</h2>

Claude Code SDK는 **Claude Agent SDK**로 이름이 변경되었으며 설명서가 재구성되었습니다. 이 변경은 코딩 작업을 넘어 AI 에이전트를 구축하기 위한 SDK의 더 광범위한 기능을 반영합니다.

OpenAI Agents SDK에서 마이그레이션하고 있습니까? [OpenAI Agents SDK 마이그레이션 레시피](https://platform.claude.com/cookbook/claude-agent-sdk-04-migrating-from-openai-agents-sdk)는 단일 작업 예제를 통해 각 기본 요소를 Claude Agent SDK에 매핑합니다.

<h2 id="what’s-changed">
  변경 사항
</h2>

| 항목                 | 이전                          | 신규                                                         |
| :----------------- | :-------------------------- | :--------------------------------------------------------- |
| **패키지 이름 (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk`                           |
| **Python 패키지**     | `claude-code-sdk`           | `claude-agent-sdk`                                         |
| **문서 위치**          | Claude Code 문서              | Claude Code 문서 → 전용 [Agent SDK](/docs/ko/agent-sdk/overview) 섹션 |

<h2 id="migration-steps">
  마이그레이션 단계
</h2>

<h3 id="for-typescript/javascript-projects">
  TypeScript/JavaScript 프로젝트의 경우
</h3>

**1. 기존 패키지 제거:**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. 새 패키지 설치:**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. 임포트 업데이트:**

`@anthropic-ai/claude-code`에서 `@anthropic-ai/claude-agent-sdk`로 모든 임포트를 변경합니다:

```typescript theme={null}
// Before
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// After
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. package.json 업데이트:**

`package.json`에 `@anthropic-ai/claude-code`가 여전히 나열되어 있으면 `@anthropic-ai/claude-agent-sdk`로 바꾸고 버전 범위도 업데이트합니다. 예를 들어 `"^0.0.42"`에서 `"^0.3.0"`으로 변경합니다.

**5. [주요 변경 사항](#breaking-changes) 검토**

마이그레이션을 완료하기 위해 필요한 코드 변경을 수행합니다.

<h3 id="for-python-projects">
  Python 프로젝트의 경우
</h3>

**1. 기존 패키지 제거:**

```bash theme={null}
pip uninstall -y claude-code-sdk
```

기존 패키지가 설치되어 있지 않으면 pip에서 `WARNING: Skipping claude-code-sdk as it is not installed.`를 출력합니다. 이는 정상이며 다음 단계로 진행할 수 있습니다.

**2. 새 패키지 설치:**

```bash theme={null}
pip install claude-agent-sdk
```

`requirements.txt` 또는 `pyproject.toml`에 `claude-code-sdk`가 나열되어 있으면 `claude-agent-sdk`로 바꿉니다.

**3. 임포트 업데이트:**

`claude_code_sdk`에서 `claude_agent_sdk`로 모든 임포트를 변경합니다:

```python theme={null}
# Before
from claude_code_sdk import query, ClaudeCodeOptions

# After
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. [주요 변경 사항](#breaking-changes) 검토**

마이그레이션을 완료하기 위해 필요한 코드 변경을 수행합니다.

<h2 id="breaking-changes">
  주요 변경 사항
</h2>

<Warning>
  격리 및 명시적 구성을 개선하기 위해 Claude Agent SDK v0.1.0은 Claude Code SDK에서 마이그레이션하는 사용자를 위한 주요 변경 사항을 도입합니다.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions가 ClaudeAgentOptions로 이름 변경됨
</h3>

**변경 사항:** Python SDK 타입 `ClaudeCodeOptions`가 `ClaudeAgentOptions`로 이름이 변경되었습니다.

**마이그레이션:**

```python theme={null}
# BEFORE (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# AFTER (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

<h3 id="system-prompt-no-longer-default">
  시스템 프롬프트가 더 이상 기본값이 아님
</h3>

**변경 사항:** SDK는 더 이상 기본적으로 Claude Code의 시스템 프롬프트를 사용하지 않습니다.

**마이그레이션:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // BEFORE (v0.0.x) - 기본적으로 Claude Code의 시스템 프롬프트를 사용했습니다
  const before = query({ prompt: "Hello" });

  // AFTER (v0.1.0) - 기본적으로 최소 시스템 프롬프트를 사용합니다
  // 이전 동작을 얻으려면 Claude Code의 프리셋을 명시적으로 요청하세요:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // 또는 사용자 정의 시스템 프롬프트를 사용하세요:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio


  async def main():
      # BEFORE (v0.0.x) - 기본적으로 Claude Code의 시스템 프롬프트를 사용했습니다
      async for message in query(prompt="Hello"):
          print(message)

      # AFTER (v0.1.0) - 기본적으로 최소 시스템 프롬프트를 사용합니다
      # 이전 동작을 얻으려면 Claude Code의 프리셋을 명시적으로 요청하세요:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              system_prompt={"type": "preset", "preset": "claude_code"}  # 프리셋 사용
          ),
      ):
          print(message)

      # 또는 사용자 정의 시스템 프롬프트를 사용하세요:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="settings-sources-default">
  설정 소스 기본값
</h3>

이 기본값은 v0.1.0에서 파일 시스템 설정을 로드하지 않도록 잠시 변경되었다가 되돌려졌으므로 마이그레이션 조치가 필요하지 않습니다.

**현재 동작:** `query()`에서 `settingSources`를 생략하면 사용자, 프로젝트 및 로컬 파일 시스템 설정을 로드하며, 이는 CLI와 일치합니다. 여기에는 `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, CLAUDE.md 파일 및 사용자 정의 명령이 포함됩니다.

파일 시스템 설정에서 격리된 상태로 실행하려면 `settingSources: []` 또는 Python에서 `setting_sources=[]`를 전달하세요. 각 소스가 로드하는 항목에 대해서는 [settingSources로 파일 시스템 설정 제어](/docs/ko/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)를 참조하세요.

격리는 특히 CI/CD 파이프라인, 배포된 애플리케이션, 테스트 환경 및 로컬 사용자 정의가 유출되지 않아야 하는 다중 테넌트 시스템에서 중요합니다.

<Note>
  Python SDK 0.1.59 이하는 빈 목록을 옵션 생략과 동일하게 처리했으므로 `setting_sources=[]`에 의존하기 전에 업그레이드하세요. `settingSources`가 `[]`일 때도 읽혀지는 입력에 대해서는 [settingSources가 제어하지 않는 항목](/docs/ko/agent-sdk/claude-code-features#what-settingsources-does-not-control)을 참조하세요.
</Note>

<h2 id="next-steps">
  다음 단계
</h2>

* [Agent SDK 개요](/docs/ko/agent-sdk/overview)를 탐색하여 사용 가능한 기능에 대해 알아봅니다
* [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript)를 확인하여 자세한 API 설명서를 봅니다
* [Python SDK 참조](/docs/ko/agent-sdk/python)를 검토하여 Python 관련 설명서를 봅니다
* [사용자 정의 도구](/docs/ko/agent-sdk/custom-tools) 및 [MCP 통합](/docs/ko/agent-sdk/mcp)에 대해 알아봅니다
