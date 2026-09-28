> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Сохранение сеансов во внешнее хранилище

> Зеркалируйте стенограммы сеансов Agent SDK в собственное хранилище объектов, хранилище ключ-значение или базу данных, чтобы другие хосты могли возобновить ваши сеансы.

По умолчанию SDK записывает стенограммы сеансов в файлы JSONL в папке `~/.claude/projects/` на локальной файловой системе. Адаптер `SessionStore` позволяет зеркалировать эти стенограммы в собственный бэкенд, такой как хранилище объектов, хранилище ключ-значение или база данных, чтобы сеанс, созданный на одном хосте, можно было возобновить на другом хосте, работающем из одного и того же рабочего каталога.

Основные причины использования хранилища сеансов:

* **Развертывания на нескольких хостах.** Бессерверные функции, автомасштабируемые рабочие процессы и CI-раннеры не используют общую файловую систему. Общее хранилище позволяет репликам возобновлять сеансы друг друга.
* **Надежность.** Локальные контейнеры являются временными. Внешнее хранилище сохраняется при перезагрузках и переразвертываниях.
* **Соответствие и аудит.** Сохраняйте стенограммы в хранилище, которым вы уже управляете, с собственными правилами хранения, шифрованием и контролем доступа.

<h2 id="the-sessionstore-interface">
  Интерфейс `SessionStore`
</h2>

`SessionStore` — это объект с двумя обязательными методами, `append` и `load`, и четырьмя необязательными методами. SDK вызывает `append` для записи записей стенограммы во время запроса и `load` для их чтения при возобновлении.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

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
    listSessionSummaries?(projectKey: string): Promise<SessionSummaryEntry[]>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };

  type SessionSummaryEntry = {
    sessionId: string;
    mtime: number;
    data: Record<string, unknown>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

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
      async def list_session_summaries(
          self, project_key: str
      ) -> list[SessionSummaryEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...

  class SessionSummaryEntry(TypedDict):
      session_id: str
      mtime: int
      data: dict[str, Any]
  ```
</CodeGroup>

`SessionKey` адресует одну стенограмму. `projectKey` — это стабильное, безопасное для файловой системы кодирование рабочей директории, `sessionId` — это UUID сеанса, а `subpath` устанавливается, когда запись принадлежит стенограмме подагента или файлу сайдкара, а не основному разговору.

Поскольку `projectKey` кодирует рабочую директорию, возобновляйте или продолжайте из хранилища из рабочей директории, соответствующей исходному запуску. В TypeScript, если вы установите [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/ru/sessions#name-the-project-directory-yourself) рядом с `CLAUDE_CONFIG_DIR` в опции [`env`](/docs/ru/agent-sdk/typescript#options) запроса, SDK будет ключировать записи этого запроса и его поиски `resume` и `continue` по этому имени вместо этого. Поскольку автономные вспомогательные функции, такие как `listSessions` и `deleteSession`, не принимают `env` и читают переменные окружения процесса, установите `CLAUDE_CONFIG_DIR` и то же имя в переменных окружения хост-процесса. Требуется Agent SDK v0.3.234 или позже.

Рассматривайте `subpath` как непрозрачный суффикс ключа; он следует макету на диске, например `subagents/agent-<id>`. Когда `subpath` не определен, ключ ссылается на основную стенограмму.

| Метод                  | Обязательный | Вызывается когда                                                                                                                                                                                                                                                                                                                              |
| :--------------------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`               | Да           | После записи каждого пакета записей стенограммы локально. Записи — это объекты, безопасные для JSON, по одному на строку в локальном JSONL.                                                                                                                                                                                                   |
| `load`                 | Да           | Перед порождением подпроцесса, когда установлен `resume`, или `continue: true` разрешает самый новый сеанс хранилища, и один раз за сеанс при перечислении, если происходит откат от `listSessionSummaries`. Возвращайте `null`, если сеанс неизвестен.                                                                                       |
| `listSessions`         | Нет          | По `listSessions({ sessionStore })` и по `query()`/`startup()` с `continue: true`. Если не определено, `continue: true` выбрасывает исключение, и `listSessions({ sessionStore })` выбрасывает исключение, если не реализован `listSessionSummaries`.                                                                                         |
| `listSessionSummaries` | Нет          | По `listSessions({ sessionStore })` для чтения метаданных всех сеансов в одном вызове. Поддерживайте сводки внутри `append`. Если не определено, перечисление откатывается к `listSessions` плюс `load` для каждого сеанса.                                                                                                                   |
| `delete`               | Нет          | По `deleteSession({ sessionStore })`. Удаление основного ключа (без `subpath`) должно каскадировать на все подключи для этого сеанса и также удалить запись сводки сеанса, чтобы удаленный сеанс перестал появляться в `listSessionSummaries`. Если не определено, удаление — это холостой ход, что подходит для добавляемых только бэкендов. |
| `listSubkeys`          | Нет          | Во время возобновления для обнаружения стенограмм подагентов. Если не определено, восстанавливается только основная стенограмма.                                                                                                                                                                                                              |

В `SessionSummaryEntry` `mtime` — это время записи хранилища сайдкара и должно использовать один источник часов со значениями `mtime`, которые возвращает `listSessions`. `data` — это непрозрачное состояние, принадлежащее SDK; сохраняйте его дословно без интерпретации.

Создавайте записи, вызывая экспортированный вспомогательный метод `foldSessionSummary`, `fold_session_summary` в Python, для каждого пакета внутри `append`. Пропускайте пакеты, чей ключ имеет `subpath`; стенограммы подагентов не должны вносить вклад в сводку основного сеанса. Fold никогда не устанавливает `mtime`: отметьте его во время сохранения через аргумент `options.mtime` в TypeScript или перезаписав поле на возвращаемой записи в Python. Одновременные вызовы `append` для одного сеанса могут конкурировать на сайдкаре, поэтому сериализуйте чтение-fold-запись с помощью транзакции, compare-and-swap или блокировки для каждого сеанса; сам fold является чистым.

Для информации о том, что SDK делает со стенограммой, которую возвращает `load`, см. [Возобновление из хранилища](#resume-from-the-store).

<h2 id="quick-start">
  Быстрый старт
</h2>

SDK поставляется с `InMemorySessionStore` для разработки и тестирования. Пример ниже запускает запрос с подключенным хранилищем, захватывает ID сеанса из результирующего сообщения, а затем возобновляет из хранилища во втором вызове `query()`. Второй вызов передает тот же экземпляр хранилища плюс `resume`, поэтому SDK загружает стенограмму из хранилища вместо локальной файловой системы:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  try {
    for await (const message of query({
      prompt: "List the TypeScript files under src/",
      options: { sessionStore: store },
    })) {
      if (message.type === "result") {
        sessionId = message.session_id;
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, sessionId was already captured by the loop
    // above; connection or process failures yield no result message.
    console.error(`Session ended with an error: ${error}`);
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
      try:
          async for message in query(
              prompt="List the Python files under src/",
              options=ClaudeAgentOptions(session_store=store),
          ):
              if isinstance(message, ResultMessage):
                  session_id = message.session_id
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, session_id was already captured by the
          # loop above; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")

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

Реализуйте `append` и `load` для вашего бэкенда. Добавьте `listSessions`, `listSessionSummaries`, `delete` и `listSubkeys`, если вы хотите, чтобы `listSessions()`, одноразовое чтение метаданных, `deleteSession()` и возобновление подагента работали с хранилищем.

Записи, переданные в `append`, типизированы как `SessionStoreEntry` (объект `{ type: string; ... }`). Рассматривайте их как непрозрачные значения, безопасные для JSON: сохраняйте их по порядку и возвращайте из `load` в том же порядке. `load` должен возвращать записи, которые глубоко равны тому, что было добавлено; сериализация, равная по байтам, не требуется, поэтому бэкенды, которые переупорядочивают ключи объектов, такие как столбец двоичного JSON, подходят.

<h2 id="reference-implementations">
  Эталонные реализации
</h2>

Оба репозитория SDK включают запускаемые эталонные адаптеры в [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) на TypeScript и [`examples/session_stores/`](https://github.com/anthropics/claude-agent-sdk-python/tree/main/examples/session_stores) на Python. Существует один адаптер для каждого типа хранилища, и каждый показывает, как `append` и `load` отображаются на этот вид бэкенда. Они не опубликованы как пакеты; скопируйте адаптер для типа, наиболее близкого к вашему бэкенду, в ваш проект, установите клиент вашего бэкенда и адаптируйте его.

| Тип хранилища                         | Модель хранилища                                                                                                                           | Пример адаптера                                                                                                                                                                                                                                            |
| :------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Object store                          | Один файл части на `append()`; `load()` перечисляет части, сортирует их и объединяет.                                                      | S3 ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/s3_session_store.py))                   |
| Key-value store                       | Один список на стенограмму, в который `append()` добавляет и из которого `load()` читает в диапазоне, плюс отсортированный индекс сеансов. | Redis ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/redis_session_store.py))          |
| Relational database or document store | Одна строка или документ на запись, сохраненные как JSON и упорядоченные по ключу, назначенному при вставке.                               | Postgres ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/postgres_session_store.py)) |

Каждый адаптер принимает предварительно настроенный экземпляр клиента, поэтому вы контролируете учетные данные, TLS, регион и пулинг. Следующий пример подключает адаптер object-store к `query()` и затем возобновляет работу с ним на другом хосте:

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

В TypeScript скопируйте [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) из директории примеров в ваш набор тестов. В Python набор поставляется в пакете. Чтобы запустить его с pytest, который не является зависимостью SDK, сначала установите pytest:

```bash theme={null}
pip install pytest
```

Затем передайте ваш адаптер в набор в файле теста как фабрику без аргументов, которую `run_session_store_conformance` вызывает один раз для каждого контракта, чтобы построить свежее хранилище:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.anyio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

Передача самого класса `MyRedisStore`, как в этом примере, работает, когда конструктор не принимает аргументы. Для адаптера, который принимает предварительно настроенный клиент, передайте вместо этого лямбду, которая конструирует хранилище. Поскольку контракты повторно используют одни и те же ключи сеансов, каждое хранилище, которое возвращает фабрика, должно начинаться с пустого хранилища, поэтому пусть лямбда предоставляет изолированное резервное хранилище для каждого вызова, такое как свежий поддельный объект в памяти, уникальный префикс ключа или новую тестовую базу данных.

<h2 id="behavior-notes">
  Примечания о поведении
</h2>

<h3 id="dual-write-architecture">
  Архитектура двойной записи
</h3>

Подпроцесс Claude Code всегда сначала записывает каждый пакет записей стенограммы на локальный диск, а затем SDK пересылает тот же пакет в `append()` вашего хранилища, поэтому хранилище является зеркалом локальной стенограммы, а не её заменой. Какая копия пережит выполнение, зависит от того, как было запущено выполнение:

* **Новый сеанс или возобновление, когда хранилище не содержит ничего для сеанса**: локальная стенограмма в вашей директории конфигурации пережит выполнение, и хранилище получит копию.
* **Выполнение [возобновлено из хранилища](#resume-from-the-store)**: локальная копия удаляется в конце выполнения, поэтому хранилище содержит единственную надёжную копию.

Если вы не хотите, чтобы новый сеанс оставлял стенограмму на локальном диске, установите `CLAUDE_CONFIG_DIR` на временную директорию в `options.env`. Выполнение, возобновленное из хранилища, уже удаляет свою локальную копию, поэтому ему не требуется такая настройка. В TypeScript распространите `process.env` в `env` также, поскольку [опция `env`](/docs/ru/agent-sdk/typescript#options) заменяет окружение подпроцесса.

Если ваше приложение входит через файлы в директории конфигурации, такие как учётные данные OAuth или `apiKeyHelper` в вашем пользовательском `settings.json`, сначала скопируйте эти файлы во временную директорию, или установите `ANTHROPIC_API_KEY` в `env` вместо этого. В противном случае выполнение завершится с ошибкой `Not logged in`.

Две опции конфликтуют с зеркалом, и SDK выбрасывает исключение при запуске, если вы объедините любую из них с хранилищем:

* **`persistSession: false`** в TypeScript: отключает локальные записи, на которых построено зеркало. Python SDK не имеет эквивалентной опции.
* **Контрольные точки файлов**, `enableFileCheckpointing` в TypeScript или `enable_file_checkpointing` в Python: записывает резервные копии файлов прямо на локальный диск, и SDK не зеркалирует их в хранилище.

<h3 id="resume-from-the-store">
  Возобновление из хранилища
</h3>

Когда вы передаёте `resume`, или `continue: true` в TypeScript или `continue_conversation=True` в Python, вместе с хранилищем, SDK запрашивает у хранилища стенограмму перед тем, как порождает подпроцесс:

* **`resume`**: SDK запрашивает сеанс, чей ID вы передали.
* **`continue: true`** или **`continue_conversation=True`**: SDK запрашивает самый новый сеанс хранилища.

Когда хранилище возвращает стенограмму, SDK записывает её во временную директорию конфигурации, запускает подпроцесс с `CLAUDE_CONFIG_DIR`, указывающим туда, и удаляет директорию по окончании выполнения. Локальная стенограмма, которую записывает это выполнение, удаляется вместе с ней, поэтому хранилище содержит единственную надёжную копию на этом пути.

SDK также заполняет временную директорию файлами из вашей реальной директории конфигурации. Что копируется, отличается по языкам:

* **TypeScript**: учётные данные, `.claude.json` и ваш пользовательский `settings.json`. Из `settings.json` он удаляет ключи, которые ведут себя неправильно во временной директории конфигурации: `enabledPlugins`, `extraKnownMarketplaces`, его [alias `additionalMarketplaces`](/docs/ru/settings-reference#extraknownmarketplaces) и любой `CLAUDE_CONFIG_DIR` в блоке `env` файла. До Agent SDK v0.3.232 SDK не удалял alias. Аутентификация, настроенная в settings, такая как [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper), работает, когда вы возобновляете из хранилища. До Agent SDK v0.3.222 TypeScript SDK копировал только учётные данные и `.claude.json`.
* **Python**: только учётные данные и `.claude.json`, поэтому приложение, которое аутентифицируется через `apiKeyHelper` в вашем пользовательском `settings.json`, завершается с ошибкой `Not logged in` при возобновлении из хранилища. `apiKeyHelper` в управляемых или проектных settings всё ещё работает, потому что Claude Code читает эти файлы из местоположений, на которые не влияет `CLAUDE_CONFIG_DIR`.

Когда хранилище не содержит ничего для сеанса, SDK запускается в вашей реальной директории конфигурации вместо этого, и результат зависит от того, какую опцию вы передали:

* **`resume`**: оба SDK передают ID через подпроцесс, который возобновляет локальную стенограмму точно так же, как `resume` без хранилища.
* **`continue: true`** в TypeScript: SDK запускает новый сеанс.
* **`continue_conversation=True`** в Python: SDK продолжает с самого нового локального сеанса.

<h3 id="mirror-writes-are-best-effort">
  Зеркальные записи — это лучшие усилия
</h3>

Если `append()` отклоняет, SDK повторяет попытку пакета ещё два раза с коротким отступом, всего максимум три попытки. Вызов, который истекает по времени, не повторяется, поскольку исходный вызов может всё ещё приземлиться. Если пакет всё ещё не удаётся, SDK регистрирует ошибку, выдаёт сообщение `{ type: "system", subtype: "mirror_error" }` в итератор, отбрасывает пакет и продолжает запрос. Поскольку повторный пакет может повторно доставить записи, которые уже приземлились, дедублируйте по `entry.uuid` в вашей реализации `append()`.

Сбой хранилища не прерывает агента, поскольку подпроцесс записывает локально в первую очередь. Отслеживайте `mirror_error`, если вам нужно обнаружить потерю данных хранилища. При выполнении [возобновленном из хранилища](#resume-from-the-store), отброшенный пакет не имеет выжившей копии по окончании выполнения.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` возвращает цепь после компактирования
</h3>

`getSessionMessages({ sessionStore })` возвращает связанную цепь сообщений, которую агент видел бы при возобновлении. После автоматического компактирования более ранние ходы заменяются резюме, поэтому сеанс, чье хранилище содержит 503 необработанные записи, может возвращать 18 сообщений из `getSessionMessages`. Для полной необработанной истории, включая ходы до компактирования и записи метаданных, вызовите `store.load(key)` напрямую.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` — это не побайтовая копия
</h3>

`forkSession({ sessionStore })` читает исходные записи, переписывает каждое поле `sessionId` и переназначает UUID сообщений, затем добавляет преобразованные записи под новым ключом. Копия на уровне адаптера или ярлык `CopyObject` создали бы стенограмму, которая всё ещё ссылается на старый ID сеанса, поэтому SDK не использует один.

<h3 id="subagent-transcripts">
  Стенограммы подагентов
</h3>

Стенограммы подагентов зеркалируются под `subpath: "subagents/agent-<id>"`. `listSubagents({ sessionStore })` требует, чтобы адаптер реализовал `listSubkeys`; `getSubagentMessages({ sessionStore })` использует его, когда доступно, но возвращается к прямому подпути, когда он не определен. Возобновление также вызывает `listSubkeys` для восстановления файлов подагентов; без него материализуется только основная стенограмма.

<h3 id="retention">
  Хранение
</h3>

SDK никогда не удаляет из вашего хранилища самостоятельно. Хранение — это ответственность адаптера: используйте механизм истечения срока действия или жизненного цикла вашего бэкенда, или запустите запланированную очистку, в соответствии с вашими требованиями соответствия. Локальные стенограммы в `CLAUDE_CONFIG_DIR` очищаются независимо параметром `cleanupPeriodDays`, следуя [правилам очистки при сохранении](/docs/ru/claude-directory#cleaned-up-automatically). Выполнение [возобновленное из хранилища](#resume-from-the-store) не оставляет локальную стенограмму, поэтому для этих выполнений хранение вашего хранилища — это единственное хранение, которое существует.

<h2 id="supported-on">
  Поддерживается на
</h2>

Следующие функции TypeScript SDK принимают опцию `sessionStore` и работают с хранилищем вместо локальной файловой системы, когда она предоставляется:

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

В Python SDK установите `session_store` в [`ClaudeAgentOptions`](/docs/ru/agent-sdk/python#claudeagentoptions) для запуска `query()` против хранилища. Остальные операции имеют каждая функцию с поддержкой хранилища на Python, которая принимает хранилище в качестве аргумента: `list_sessions_from_store()`, `get_session_info_from_store()`, `get_session_messages_from_store()`, `list_subagents_from_store()`, `get_subagent_messages_from_store()`, `rename_session_via_store()`, `tag_session_via_store()`, `delete_session_via_store()` и `fork_session_via_store()`. `startup()` не имеет эквивалента на Python. Автономные функции, задокументированные в [справочнике Python SDK](/docs/ru/agent-sdk/python#functions), такие как `list_sessions()`, читают локальные файлы сеансов.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

* [Работа с сеансами](/docs/ru/agent-sdk/sessions): Продолжение, возобновление и разветвление без пользовательского хранилища
* [Размещение SDK](/docs/ru/agent-sdk/hosting): Шаблоны развертывания для сред с несколькими хостами
* [TypeScript `Options`](/docs/ru/agent-sdk/typescript#options): Полная справка по опциям
* [Эталонные реализации](#reference-implementations): Запускаемые примеры адаптеров для хранилища объектов, хранилища ключ-значение и базы данных в обоих репозиториях SDK
