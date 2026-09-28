> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Миграция на Claude Agent SDK

> Руководство по миграции Claude Code TypeScript и Python SDK на Claude Agent SDK

<h2 id="overview">
  Обзор
</h2>

Claude Code SDK был переименован в **Claude Agent SDK**, и его документация была переорганизована. Это изменение отражает более широкие возможности SDK для создания AI-агентов, выходящих за рамки только задач кодирования.

Переходите с OpenAI Agents SDK? [Рецепт миграции OpenAI Agents SDK](https://platform.claude.com/cookbook/claude-agent-sdk-04-migrating-from-openai-agents-sdk) отображает каждый примитив на Claude Agent SDK через один проработанный пример.

<h2 id="what’s-changed">
  Что изменилось
</h2>

| Аспект                          | Старое                      | Новое                                                                    |
| :------------------------------ | :-------------------------- | :----------------------------------------------------------------------- |
| **Имя пакета (TS/JS)**          | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk`                                         |
| **Python пакет**                | `claude-code-sdk`           | `claude-agent-sdk`                                                       |
| **Местоположение документации** | Claude Code docs            | Claude Code docs → выделенный раздел [Agent SDK](/docs/ru/agent-sdk/overview) |

<h2 id="migration-steps">
  Этапы миграции
</h2>

<h3 id="for-typescript/javascript-projects">
  Для проектов TypeScript/JavaScript
</h3>

**1. Удалите старый пакет:**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. Установите новый пакет:**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. Обновите ваши импорты:**

Измените все импорты с `@anthropic-ai/claude-code` на `@anthropic-ai/claude-agent-sdk`:

```typescript theme={null}
// До
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// После
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. Обновите package.json:**

Если `@anthropic-ai/claude-code` всё ещё указан в вашем `package.json`, замените его на `@anthropic-ai/claude-agent-sdk` и также обновите диапазон версий, например с `"^0.0.42"` на `"^0.3.0"`.

**5. Ознакомьтесь с [критическими изменениями](#breaking-changes)**

Внесите необходимые изменения в код для завершения миграции.

<h3 id="for-python-projects">
  Для проектов Python
</h3>

**1. Удалите старый пакет:**

```bash theme={null}
pip uninstall -y claude-code-sdk
```

Если старый пакет не установлен, pip выведет `WARNING: Skipping claude-code-sdk as it is not installed.` Это нормально, и вы можете перейти к следующему этапу.

**2. Установите новый пакет:**

```bash theme={null}
pip install claude-agent-sdk
```

Если `claude-code-sdk` указан в вашем `requirements.txt` или `pyproject.toml`, замените его на `claude-agent-sdk`.

**3. Обновите ваши импорты:**

Измените все импорты с `claude_code_sdk` на `claude_agent_sdk`:

```python theme={null}
# До
from claude_code_sdk import query, ClaudeCodeOptions

# После
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Ознакомьтесь с [критическими изменениями](#breaking-changes)**

Внесите необходимые изменения в код для завершения миграции.

<h2 id="breaking-changes">
  Критические изменения
</h2>

<Warning>
  Для улучшения изоляции и явной конфигурации Claude Agent SDK v0.1.0 вводит критические изменения для пользователей, переходящих с Claude Code SDK.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions переименован в ClaudeAgentOptions
</h3>

**Что изменилось:** Тип Python SDK `ClaudeCodeOptions` был переименован в `ClaudeAgentOptions`.

**Миграция:**

```python theme={null}
# BEFORE (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# AFTER (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

<h3 id="system-prompt-no-longer-default">
  Системный промпт больше не используется по умолчанию
</h3>

**Что изменилось:** SDK больше не использует системный промпт Claude Code по умолчанию.

**Миграция:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // BEFORE (v0.0.x) - Used Claude Code's system prompt by default
  const before = query({ prompt: "Hello" });

  // AFTER (v0.1.0) - Uses minimal system prompt by default
  // To get the old behavior, explicitly request Claude Code's preset:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Or use a custom system prompt:
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
      # BEFORE (v0.0.x) - Used Claude Code's system prompt by default
      async for message in query(prompt="Hello"):
          print(message)

      # AFTER (v0.1.0) - Uses minimal system prompt by default
      # To get the old behavior, explicitly request Claude Code's preset:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              system_prompt={"type": "preset", "preset": "claude_code"}  # Use the preset
          ),
      ):
          print(message)

      # Or use a custom system prompt:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="settings-sources-default">
  Источники параметров по умолчанию
</h3>

Это значение по умолчанию было кратко изменено в v0.1.0 для загрузки без параметров файловой системы, а затем восстановлено, поэтому никаких действий по миграции не требуется.

**Текущее поведение:** Пропуск `settingSources` в `query()` загружает параметры пользователя, проекта и локальной файловой системы, соответствуя CLI. Это включает `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, файлы CLAUDE.md и пользовательские команды.

Для работы в изоляции от параметров файловой системы передайте `settingSources: []` или `setting_sources=[]` в Python. См. [Control filesystem settings with settingSources](/docs/ru/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) для информации о том, что загружает каждый источник.

Изоляция особенно важна для конвейеров CI/CD, развёрнутых приложений, тестовых сред и многопользовательских систем, где локальные настройки не должны утекать.

<Note>
  Python SDK 0.1.59 и более ранние версии обрабатывали пустой список так же, как пропуск опции, поэтому обновитесь перед использованием `setting_sources=[]`. См. [What settingSources does not control](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control) для входных данных, которые читаются даже когда `settingSources` равен `[]`.
</Note>

<h2 id="next-steps">
  Следующие шаги
</h2>

* Изучите [Обзор Agent SDK](/docs/ru/agent-sdk/overview), чтобы узнать о доступных функциях
* Ознакомьтесь со [Справочником TypeScript SDK](/docs/ru/agent-sdk/typescript) для подробной документации API
* Просмотрите [Справочник Python SDK](/docs/ru/agent-sdk/python) для документации, специфичной для Python
* Узнайте о [Пользовательских инструментах](/docs/ru/agent-sdk/custom-tools) и [Интеграции MCP](/docs/ru/agent-sdk/mcp)
