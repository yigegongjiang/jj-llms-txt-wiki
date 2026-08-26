> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Сохранение сеансов во внешнее хранилище

> Зеркалируйте стенограммы сеансов в S3, Redis или собственный бэкенд, чтобы любой хост мог их возобновить.

По умолчанию SDK записывает стенограммы сеансов в файлы JSONL в папке `~/.claude/projects/` на локальной файловой системе. Адаптер `SessionStore` позволяет зеркалировать эти стенограммы в собственный бэкенд, такой как S3, Redis или база данных, чтобы сеанс, созданный на одном хосте, можно было возобновить на другом.

Основные причины использования хранилища сеансов:

* **Развертывания на нескольких хостах.** Бессерверные функции, автомасштабируемые рабочие процессы и CI-раннеры не используют общую файловую систему. Общее хранилище позволяет любой реплике возобновить любой сеанс.
* **Надежность.** Локальные контейнеры являются временными. Хранилище, поддерживаемое S3 или базой данных, сохраняется при перезагрузках и переразвертываниях.
* **Соответствие и аудит.** Сохраняйте стенограммы в хранилище, которым вы уже управляете, с собственными правилами хранения, шифрованием и контролем доступа.

<h2 id="the-sessionstore-interface">
  Интерфейс `SessionStore`
</h2>

`SessionStore` — это объект с двумя обязательными методами, `append` и `load`, и тремя необязательными методами. SDK вызывает `append` для записи записей стенограммы во время запроса и `load` для их чтения при возобновлении.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry.

  type SessionKey = {
    projectKey: string;
    sessionId: string;
    subpath?: string;
  };

  type SessionStore = {
    // Required
    append(key: SessionKey, entries: SessionStoreEntry[]): Promise<void>;
    load(key: SessionKey): Promise<SessionStoreEntry[] | null>;

    // Optional
    listSessions?(
      projectKey: string,
    ): Promise<Array<{ sessionId: string; mtime: number }>>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry.

  class SessionKey(TypedDict):
      project_key: str
      session_id: str
      subpath: NotRequired[str]

  class SessionStore(Protocol):
      # Required
      async def append(
          self, key: SessionKey, entries: list[SessionStoreEntry]
      ) -> None: ...
      async def load(self, key: SessionKey) -> list[SessionStoreEntry] | None: ...

      # Optional — omit or raise NotImplementedError
      async def list_sessions(
          self, project_key: str
      ) -> list[SessionStoreListEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...
  ```
</CodeGroup>

`SessionKey` адресует одну стенограмму. `projectKey` — это стабильное, безопасное для файловой системы кодирование рабочей директории, `sessionId` — это UUID сеанса, а `subpath` устанавливается, когда запись принадлежит стенограмме подагента или файлу сайдкара, а не основному разговору. Рассматривайте `subpath` как непрозрачный суффикс ключа; он следует макету на диске, например `subagents/agent-<id>`. Когда `subpath` не определен, ключ ссылается на основную стенограмму.

| Метод          | Обязательный | Вызывается когда                                                                                                                                                                                                                     |
| :------------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`       | Да           | После записи каждого пакета записей стенограммы локально. Записи — это объекты, безопасные для JSON, по одному на строку в локальном JSONL.                                                                                          |
| `load`         | Да           | Один раз перед порождением подпроцесса, когда установлен `resume`. Возвращайте `null`, если сеанс неизвестен.                                                                                                                        |
| `listSessions` | Нет          | По `listSessions({ sessionStore })` и по `query()`/`startup()` с `continue: true`. Если не определено, эти вызовы выбрасывают исключение.                                                                                            |
| `delete`       | Нет          | По `deleteSession({ sessionStore })`. Удаление основного ключа (без `subpath`) должно каскадировать на все подключи для этого сеанса. Если не определено, удаление — это холостой ход, что подходит для добавляемых только бэкендов. |
| `listSubkeys`  | Нет          | Во время возобновления для обнаружения стенограмм подагентов. Если не определено, восстанавливается только основная стенограмма.                                                                                                     |

<h2 id="quick-start">
  Быстрый старт
</h2>

SDK поставляется с `InMemorySessionStore` для разработки и тестирования. Пример ниже запускает запрос с подключенным хранилищем, захватывает ID сеанса из результирующего сообщения, а затем возобновляет из хранилища во втором вызове `query()`. Второй вызов передает тот же экземпляр хранилища плюс `resume`, поэтому SDK загружает стенограмму из хранилища вместо локальной файловой системы:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  for await (const message of query({
    prompt: "List the TypeScript files under src/",
    options: { sessionStore: store },
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
    }
  }

  // Resume from the store. The agent has full context from the first call.
  for await (const message of query({
    prompt: "Summarize what those files do",
    options: { sessionStore: store, resume: sessionId },
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      ClaudeAgentOptions,
      InMemorySessionStore,
      ResultMessage,
      query,
  )

  store = InMemorySessionStore()


  async def main():
      session_id = None
      async for message in query(
          prompt="List the Python files under src/",
          options=ClaudeAgentOptions(session_store=store),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id

      # Resume from the store. The agent has full context from the first call.
      async for message in query(
          prompt="Summarize what those files do",
          options=ClaudeAgentOptions(session_store=store, resume=session_id),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

Второй запрос выводит сводку файлов из первого запроса, что показывает, что агент возобновил работу с полным контекстом из хранилища.

<h2 id="write-your-own-adapter">
  Напишите собственный адаптер
</h2>

Реализуйте `append` и `load` для вашего бэкенда. Добавьте `listSessions`, `delete` и `listSubkeys`, если вы хотите, чтобы `listSessions()`, `deleteSession()` и возобновление подагента работали с хранилищем.

Записи, переданные в `append`, типизированы как `SessionStoreEntry` (объект `{ type: string; ... }`). Рассматривайте их как непрозрачные значения, безопасные для JSON: сохраняйте их по порядку и возвращайте из `load` в том же порядке. `load` должен возвращать записи, которые глубоко равны тому, что было добавлено; сериализация, равная по байтам, не требуется, поэтому бэкенды, такие как Postgres `jsonb`, которые переупорядочивают ключи объектов, подходят.

<h2 id="reference-implementations">
  Эталонные реализации
</h2>

Репозиторий TypeScript SDK включает запускаемые эталонные адаптеры для S3, Redis и Postgres в [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores). Они не опубликованы в npm; скопируйте нужный файл `src/` в ваш проект и установите соответствующий клиент бэкенда.

| Адаптер                                                                                                                        | Клиент бэкенда       | Модель хранения                                                                     |
| :----------------------------------------------------------------------------------------------------------------------------- | :------------------- | :---------------------------------------------------------------------------------- |
| [`S3SessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3)             | `@aws-sdk/client-s3` | Один файл части JSONL на `append()`; `load()` перечисляет, сортирует и объединяет.  |
| [`RedisSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis)       | `ioredis`            | Список `RPUSH`/`LRANGE` на стенограмму плюс индекс отсортированного набора сеансов. |
| [`PostgresSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres) | `pg`                 | Одна строка на запись в таблице `jsonb`, упорядоченная по `BIGSERIAL`.              |

Каждый адаптер принимает предварительно настроенный экземпляр клиента, поэтому вы контролируете учетные данные, TLS, регион и пулинг. Например, с S3:

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import { S3Client } from "@aws-sdk/client-s3";
import { S3SessionStore } from "./S3SessionStore"; // copied from examples/session-stores/s3

const store = new S3SessionStore({
  bucket: "my-claude-sessions",
  prefix: "transcripts",
  client: new S3Client({ region: "us-east-1" }),
});

for await (const message of query({
  prompt: "Hello!",
  options: { sessionStore: store },
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Later, possibly on a different host:
for await (const message of query({
  prompt: "Continue where we left off",
  options: { sessionStore: store, resume: "previous-session-id" },
})) {
  // ...
}
```

<h3 id="validate-your-adapter">
  Проверьте ваш адаптер
</h3>

Оба SDK поставляются с набором соответствия, который утверждает поведенческий контракт, который должны удовлетворять `append`, `load` и необязательные методы. Тесты для необязательных методов автоматически пропускаются, когда эти методы не реализованы.

В TypeScript скопируйте [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) из директории примеров в ваш набор тестов. В Python набор поставляется в пакете:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.asyncio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

<h2 id="behavior-notes">
  Примечания о поведении
</h2>

<h3 id="dual-write-architecture">
  Архитектура двойной записи
</h3>

Хранилище — это зеркало, а не замена. Подпроцесс Claude Code всегда сначала записывает на локальный диск; затем SDK пересылает каждый пакет в `append()`. Если вы хотите, чтобы локальная копия была временной, укажите `CLAUDE_CONFIG_DIR` на временную директорию в `options.env`. Поскольку зеркало зависит от локальных записей, `sessionStore` не может быть объединен с `persistSession: false`; SDK выбрасывает исключение, если вы установите оба. Он также выбрасывает исключение, если объединен с `enableFileCheckpointing`, поскольку резервные копии истории файлов записываются непосредственно на локальный диск и не зеркалируются в хранилище.

<h3 id="mirror-writes-are-best-effort">
  Зеркальные записи — это лучшие усилия
</h3>

Если `append()` отклоняет, SDK повторяет попытку пакета еще два раза с коротким отступом, всего максимум три попытки. Вызов, который истекает по времени, не повторяется, поскольку исходный вызов может все еще приземлиться. Если пакет все еще не удается, ошибка регистрируется, сообщение `{ type: "system", subtype: "mirror_error" }` выдается в итератор, пакет отбрасывается и запрос продолжается. Локальная стенограмма уже надежна на диске, поэтому сбой хранилища не прерывает агента и не теряет данные локально. Отслеживайте `mirror_error`, если вам нужно обнаружить потерю данных хранилища. Поскольку повторный пакет может повторно доставить записи, которые уже приземлились, дедублируйте по `entry.uuid` в вашей реализации `append()`.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` возвращает цепь после компактирования
</h3>

`getSessionMessages({ sessionStore })` возвращает связанную цепь сообщений, которую агент видел бы при возобновлении. После автоматического компактирования более ранние ходы заменяются резюме, поэтому сеанс, чье хранилище содержит 503 необработанные записи, может возвращать 18 сообщений из `getSessionMessages`. Для полной необработанной истории, включая ходы до компактирования и записи метаданных, вызовите `store.load(key)` напрямую.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` — это не побайтовая копия
</h3>

`forkSession({ sessionStore })` читает исходные записи, переписывает каждое поле `sessionId` и переназначает UUID сообщений, затем добавляет преобразованные записи под новым ключом. Копия на уровне адаптера или ярлык `CopyObject` создали бы стенограмму, которая все еще ссылается на старый ID сеанса, поэтому SDK не использует один.

<h3 id="subagent-transcripts">
  Стенограммы подагентов
</h3>

Стенограммы подагентов зеркалируются под `subpath: "subagents/agent-<id>"`. `listSubagents({ sessionStore })` требует, чтобы адаптер реализовал `listSubkeys`; `getSubagentMessages({ sessionStore })` использует его, когда доступно, но возвращается к прямому подпути, когда он не определен. Возобновление также вызывает `listSubkeys` для восстановления файлов подагентов; без него материализуется только основная стенограмма.

<h3 id="retention">
  Хранение
</h3>

SDK никогда не удаляет из вашего хранилища самостоятельно. Хранение — это ответственность адаптера: реализуйте TTL, политики жизненного цикла S3 или запланированную очистку в соответствии с вашими требованиями соответствия. Локальные стенограммы в `CLAUDE_CONFIG_DIR` очищаются независимо параметром `cleanupPeriodDays`.

<h2 id="supported-on">
  Поддерживается на
</h2>

Следующие функции SDK принимают опцию `sessionStore` и работают с хранилищем вместо локальной файловой системы, когда она предоставляется:

* [`query()`](/docs/ru/agent-sdk/typescript#query)
* [`startup()`](/docs/ru/agent-sdk/typescript#startup)
* [`listSessions()`](/docs/ru/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](/docs/ru/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](/docs/ru/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](/docs/ru/agent-sdk/typescript#renamesession)
* [`tagSession()`](/docs/ru/agent-sdk/typescript#tagsession)
* [`deleteSession()`](/docs/ru/agent-sdk/typescript)
* [`forkSession()`](/docs/ru/agent-sdk/typescript)
* [`listSubagents()`](/docs/ru/agent-sdk/typescript)
* [`getSubagentMessages()`](/docs/ru/agent-sdk/typescript)

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Работа с сеансами](/docs/ru/agent-sdk/sessions): Продолжение, возобновление и разветвление без пользовательского хранилища
* [Размещение SDK](/docs/ru/agent-sdk/hosting): Шаблоны развертывания для сред с несколькими хостами
* [TypeScript `Options`](/docs/ru/agent-sdk/typescript#options): Полная справка по опциям
* [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores): Запускаемые эталонные адаптеры S3, Redis и Postgres
