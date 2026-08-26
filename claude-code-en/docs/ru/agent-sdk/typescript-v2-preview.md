> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# TypeScript SDK V2 session API (removed)

> Справочник по удалённому V2 TypeScript Agent SDK session API с паттернами отправки/потока на основе сессий для многооборотных разговоров.

<Warning>
  V2 session API больше не поддерживается. TypeScript Agent SDK 0.3.142 удаляет `unstable_v2_createSession`, `unstable_v2_resumeSession`, `unstable_v2_prompt` и типы `SDKSession` и `SDKSessionOptions`.

  Для миграции используйте [`query()` API](/docs/ru/agent-sdk/typescript) и [параметры сессии](/docs/ru/agent-sdk/sessions), которые он принимает. Передайте `AsyncIterable<SDKUserMessage>` для многооборотных разговоров или `options.resume` для продолжения сохранённой сессии. Эта страница сохранена для справки, если вы поддерживаете код на Agent SDK 0.2.x или более ранней версии.
</Warning>

V2 был экспериментальным session API, который устранил необходимость в асинхронных генераторах и координации yield. Вместо управления состоянием генератора между оборотами, каждый оборот представлял собой отдельный цикл `send()`/`stream()`. Поверхность API сводилась к трём концепциям:

* `createSession()` / `resumeSession()`: Начать или продолжить разговор
* `session.send()`: Отправить сообщение
* `session.stream()`: Получить ответ

<h2 id="installation">
  Установка
</h2>

Agent SDK версии 0.2.x — это последняя версия, которая включает интерфейс V2. Версия пакета перепрыгнула с 0.2.x прямо на 0.3.142, поэтому версия удаления выше и указание установки ниже описывают одну и ту же границу. Чтобы установить последний совместимый с V2 релиз, зафиксируйте основную и дополнительную версию:

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk@0.2
```

<Note>
  SDK поставляется с нативным бинарным файлом Claude Code для вашей платформы в качестве опциональной зависимости, поэтому вам не нужно устанавливать Claude Code отдельно.
</Note>

<h2 id="quick-start">
  Быстрый старт
</h2>

<h3 id="one-shot-prompt">
  Однократный запрос
</h3>

Для простых однооборотных запросов, когда вам не нужно поддерживать сессию, используйте `unstable_v2_prompt()`. Этот пример отправляет математический вопрос и логирует ответ:

```typescript theme={null}
import { unstable_v2_prompt } from "@anthropic-ai/claude-agent-sdk";

const result = await unstable_v2_prompt("What is 2 + 2?", {
  model: "claude-opus-4-7"
});
if (result.subtype === "success") {
  console.log(result.result);
}
```

<details>
  <summary>Посмотрите ту же операцию в V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "What is 2 + 2?",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "result" && msg.subtype === "success") {
      console.log(msg.result);
    }
  }
  ```
</details>

<h3 id="basic-session">
  Базовая сессия
</h3>

Для взаимодействий, выходящих за рамки одного запроса, создайте сессию. V2 разделяет отправку и потоковую передачу на отдельные шаги:

* `send()` отправляет ваше сообщение
* `stream()` передаёт ответ потоком

Это явное разделение облегчает добавление логики между оборотами (например, обработка ответов перед отправкой последующих сообщений).

Пример ниже создаёт сессию, отправляет "Hello!" в Claude и выводит текстовый ответ. Он использует [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management) (TypeScript 5.2+) для автоматического закрытия сессии при выходе из блока. Вы также можете вызвать `session.close()` вручную.

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Hello!");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>Посмотрите ту же операцию в V1</summary>

  В V1 входные и выходные данные проходят через один асинхронный генератор. Для базового запроса это выглядит похоже, но добавление многооборотной логики требует переструктурирования для использования входного генератора.

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "Hello!",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

<h3 id="multi-turn-conversation">
  Многооборотный разговор
</h3>

Сессии сохраняют контекст между несколькими обменами. Чтобы продолжить разговор, вызовите `send()` снова на той же сессии. Claude помнит предыдущие обороты.

Этот пример задаёт математический вопрос, а затем задаёт последующий вопрос, который ссылается на предыдущий ответ:

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

// Turn 1
await session.send("What is 5 + 3?");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}

// Turn 2
await session.send("Multiply that by 2");
for await (const msg of session.stream()) {
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>Посмотрите ту же операцию в V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Must create an async iterable to feed messages
  async function* createInputStream() {
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "What is 5 + 3?" }] },
      parent_tool_use_id: null
    };
    // Must coordinate when to yield next message
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "Multiply by 2" }] },
      parent_tool_use_id: null
    };
  }

  const q = query({
    prompt: createInputStream(),
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

<h3 id="session-resume">
  Возобновление сессии
</h3>

Если у вас есть ID сессии из предыдущего взаимодействия, вы можете возобновить её позже. Это полезно для долгоживущих рабочих процессов или когда вам нужно сохранить разговоры между перезагрузками приложения.

Этот пример создаёт сессию, сохраняет её ID, закрывает её, а затем возобновляет разговор:

```typescript theme={null}
import {
  unstable_v2_createSession,
  unstable_v2_resumeSession,
  type SDKMessage
} from "@anthropic-ai/claude-agent-sdk";

// Helper to extract text from assistant messages
function getAssistantText(msg: SDKMessage): string | null {
  if (msg.type !== "assistant") return null;
  return msg.message.content
    .filter((block) => block.type === "text")
    .map((block) => block.text)
    .join("");
}

// Create initial session and have a conversation
const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Remember this number: 42");

// Get the session ID from any received message
let sessionId: string | undefined;
for await (const msg of session.stream()) {
  sessionId = msg.session_id;
  const text = getAssistantText(msg);
  if (text) console.log("Initial response:", text);
}

console.log("Session ID:", sessionId);
session.close();

// Later: resume the session using the stored ID
await using resumedSession = unstable_v2_resumeSession(sessionId!, {
  model: "claude-opus-4-7"
});

await resumedSession.send("What number did I ask you to remember?");
for await (const msg of resumedSession.stream()) {
  const text = getAssistantText(msg);
  if (text) console.log("Resumed response:", text);
}
```

<details>
  <summary>Посмотрите ту же операцию в V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Create initial session
  const initialQuery = query({
    prompt: "Remember this number: 42",
    options: { model: "claude-opus-4-7" }
  });

  // Get session ID from any message
  let sessionId: string | undefined;
  for await (const msg of initialQuery) {
    sessionId = msg.session_id;
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Initial response:", text);
    }
  }

  console.log("Session ID:", sessionId);

  // Later: resume the session
  const resumedQuery = query({
    prompt: "What number did I ask you to remember?",
    options: {
      model: "claude-opus-4-7",
      resume: sessionId
    }
  });

  for await (const msg of resumedQuery) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Resumed response:", text);
    }
  }
  ```
</details>

<h3 id="cleanup">
  Очистка
</h3>

Сессии можно закрывать вручную или автоматически, используя [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management), функцию TypeScript 5.2+ для автоматической очистки ресурсов. Если вы используете более старую версию TypeScript или столкнулись с проблемами совместимости, используйте вместо этого ручную очистку.

**Автоматическая очистка (TypeScript 5.2+):**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// Session closes automatically when the block exits
```

**Ручная очистка:**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// ... use the session ...
session.close();
```

<h2 id="api-reference">
  Справочник API
</h2>

<h3 id="unstable_v2_createsession">
  `unstable_v2_createSession()`
</h3>

Создаёт новую сессию для многооборотных разговоров.

```typescript theme={null}
function unstable_v2_createSession(options: {
  model: string;
  // Additional options supported
}): SDKSession;
```

<h3 id="unstable_v2_resumesession">
  `unstable_v2_resumeSession()`
</h3>

Возобновляет существующую сессию по ID.

```typescript theme={null}
function unstable_v2_resumeSession(
  sessionId: string,
  options: {
    model: string;
    // Additional options supported
  }
): SDKSession;
```

<h3 id="unstable_v2_prompt">
  `unstable_v2_prompt()`
</h3>

Однократная удобная функция для однооборотных запросов.

```typescript theme={null}
function unstable_v2_prompt(
  prompt: string,
  options: {
    model: string;
    // Additional options supported
  }
): Promise<SDKResultMessage>;
```

<h3 id="sdksession-interface">
  Интерфейс SDKSession
</h3>

```typescript theme={null}
interface SDKSession {
  readonly sessionId: string;
  send(message: string | SDKUserMessage): Promise<void>;
  stream(): AsyncGenerator<SDKMessage, void>;
  close(): void;
}
```

<h2 id="feature-availability">
  Доступность функций
</h2>

V2 session API не поддерживает все функции V1. Следующие требуют использования [V1 SDK](/docs/ru/agent-sdk/typescript):

* Разветвление сессий (опция `forkSession`)
* Некоторые продвинутые паттерны потокового ввода

<h2 id="see-also">
  См. также
</h2>

* [Справочник TypeScript SDK (V1)](/docs/ru/agent-sdk/typescript) - Полная документация V1 SDK
* [Обзор SDK](/docs/ru/agent-sdk/overview) - Общие концепции SDK
* [Примеры V2 на GitHub](https://github.com/anthropics/claude-agent-sdk-demos/tree/main/hello-world-v2) - Рабочие примеры кода
