> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Миграция на Claude Agent SDK

> Руководство по миграции Claude Code TypeScript и Python SDK на Claude Agent SDK

<h2 id="overview">
  Обзор
</h2>

Claude Code SDK был переименован в **Claude Agent SDK**, и его документация была переорганизована. Это изменение отражает более широкие возможности SDK для создания AI-агентов, выходящих за рамки только задач кодирования.

<h2 id="what’s-changed">
  Что изменилось
</h2>

| Аспект                        | Старое                      | Новое                            |
| :---------------------------- | :-------------------------- | :------------------------------- |
| **Имя пакета (TS/JS)**        | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk` |
| **Python пакет**              | `claude-code-sdk`           | `claude-agent-sdk`               |
| **Расположение документации** | Claude Code документация    | API Guide → Agent SDK раздел     |

<Note>
  **Изменения в документации:** Документация Agent SDK переместилась из Claude Code документации в API Guide в отдельный раздел [Agent SDK](/docs/ru/agent-sdk/overview). Документация Claude Code теперь сосредоточена на инструменте CLI и функциях автоматизации.
</Note>

<h2 id="migration-steps">
  Шаги миграции
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

**4. Обновите зависимости в package.json:**

Если у вас есть пакет в вашем `package.json`, обновите его:

До:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-code": "^0.0.42"
  }
}
```

После:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-agent-sdk": "^0.2.0"
  }
}
```

**5. Просмотрите [критические изменения](#breaking-changes)**

Внесите необходимые изменения в код для завершения миграции.

<h3 id="for-python-projects">
  Для Python проектов
</h3>

**1. Удалите старый пакет:**

```bash theme={null}
pip uninstall claude-code-sdk
```

**2. Установите новый пакет:**

```bash theme={null}
pip install claude-agent-sdk
```

**3. Обновите ваши импорты:**

Измените все импорты с `claude_code_sdk` на `claude_agent_sdk`:

```python theme={null}
# До
from claude_code_sdk import query, ClaudeCodeOptions

# После
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Обновите имена типов:**

Измените `ClaudeCodeOptions` на `ClaudeAgentOptions`:

```python theme={null}
# До
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7")

# После
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7")
```

**5. Просмотрите [критические изменения](#breaking-changes)**

Внесите необходимые изменения в код для завершения миграции.

<h2 id="breaking-changes">
  Критические изменения
</h2>

<Warning>
  Для улучшения изоляции и явной конфигурации Claude Agent SDK v0.1.0 вводит критические изменения для пользователей, переходящих с Claude Code SDK. Внимательно просмотрите этот раздел перед миграцией.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions переименован в ClaudeAgentOptions
</h3>

**Что изменилось:** Тип Python SDK `ClaudeCodeOptions` был переименован в `ClaudeAgentOptions`.

**Миграция:**

```python theme={null}
# ДО (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# ПОСЛЕ (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

**Почему это изменилось:** Имя типа теперь соответствует брендингу "Claude Agent SDK" и обеспечивает согласованность в соглашениях об именовании SDK.

<h3 id="system-prompt-no-longer-default">
  Системный промпт больше не используется по умолчанию
</h3>

**Что изменилось:** SDK больше не использует системный промпт Claude Code по умолчанию.

**Миграция:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // ДО (v0.0.x) - Использовал системный промпт Claude Code по умолчанию
  const before = query({ prompt: "Hello" });

  // ПОСЛЕ (v0.1.0) - Использует минимальный системный промпт по умолчанию
  // Чтобы получить старое поведение, явно запросите предустановку Claude Code:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Или используйте пользовательский системный промпт:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  # ДО (v0.0.x) - Использовал системный промпт Claude Code по умолчанию
  async for message in query(prompt="Hello"):
      print(message)

  # ПОСЛЕ (v0.1.0) - Использует минимальный системный промпт по умолчанию
  # Чтобы получить старое поведение, явно запросите предустановку Claude Code:
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          system_prompt={"type": "preset", "preset": "claude_code"}  # Используйте предустановку
      ),
  ):
      print(message)

  # Или используйте пользовательский системный промпт:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
  ):
      print(message)
  ```
</CodeGroup>

**Почему это изменилось:** Обеспечивает лучший контроль и изоляцию для приложений SDK. Теперь вы можете создавать агентов с пользовательским поведением без наследования инструкций, ориентированных на CLI Claude Code.

<h3 id="settings-sources-default">
  Значения по умолчанию для источников настроек
</h3>

Это значение по умолчанию было кратко изменено в v0.1.0, а затем восстановлено, поэтому никаких действий по миграции не требуется.

**Текущее поведение:** Пропуск `settingSources` в `query()` загружает пользовательские, проектные и локальные настройки файловой системы, соответствуя CLI. Это включает `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, файлы CLAUDE.md и пользовательские команды.

Для запуска в изоляции от настроек файловой системы передайте пустой массив:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const isolatedResult = query({
    prompt: "Hello",
    options: {
      settingSources: [] // Настройки файловой системы не загружаются
    }
  });

  // Или загрузите только определённые источники:
  const projectOnlyResult = query({
    prompt: "Hello",
    options: {
      settingSources: ["project"] // Только настройки проекта
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(setting_sources=[]),  # Настройки файловой системы не загружаются
  ):
      print(message)

  # Или загрузите только определённые источники:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          setting_sources=["project"]  # Только настройки проекта
      ),
  ):
      print(message)
  ```
</CodeGroup>

Изоляция особенно важна для конвейеров CI/CD, развёрнутых приложений, тестовых сред и многопользовательских систем, где локальные настройки не должны просачиваться.

<Note>
  SDK v0.1.0 кратко использовал значение по умолчанию без загруженных настроек; это было восстановлено в последующих выпусках. Python SDK 0.1.59 и более ранние версии обрабатывали пустой список так же, как пропуск опции, поэтому обновитесь перед использованием `setting_sources=[]`. Смотрите [Что settingSources не контролирует](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control) для входных данных, которые читаются даже когда `settingSources` равен `[]`.
</Note>

<h2 id="why-the-rename">
  Почему переименование?
</h2>

Claude Code SDK был первоначально разработан для задач кодирования, но он превратился в мощную платформу для создания всех типов AI-агентов. Новое имя "Claude Agent SDK" лучше отражает его возможности:

* Создание бизнес-агентов (помощники по правовым вопросам, финансовые консультанты, поддержка клиентов)
* Создание специализированных агентов кодирования (боты SRE, рецензенты безопасности, агенты проверки кода)
* Разработка пользовательских агентов для любой области с использованием инструментов, интеграции MCP и многого другого

<h2 id="getting-help">
  Получение помощи
</h2>

Если вы столкнулись с какими-либо проблемами во время миграции:

**Для TypeScript/JavaScript:**

1. Проверьте, что все импорты обновлены для использования `@anthropic-ai/claude-agent-sdk`
2. Убедитесь, что ваш package.json содержит новое имя пакета
3. Запустите `npm install`, чтобы убедиться, что зависимости обновлены

**Для Python:**

1. Проверьте, что все импорты обновлены для использования `claude_agent_sdk`
2. Убедитесь, что ваш requirements.txt или pyproject.toml содержит новое имя пакета
3. Запустите `pip install claude-agent-sdk`, чтобы убедиться, что пакет установлен

<h2 id="next-steps">
  Следующие шаги
</h2>

* Изучите [Обзор Agent SDK](/docs/ru/agent-sdk/overview), чтобы узнать о доступных функциях
* Ознакомьтесь со [Справочником TypeScript SDK](/docs/ru/agent-sdk/typescript) для подробной документации API
* Просмотрите [Справочник Python SDK](/docs/ru/agent-sdk/python) для документации, специфичной для Python
* Узнайте о [Пользовательских инструментах](/docs/ru/agent-sdk/custom-tools) и [Интеграции MCP](/docs/ru/agent-sdk/mcp)
