> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Справочник Agent SDK - TypeScript

> Полный справочник API для TypeScript Agent SDK, включая все функции, типы и интерфейсы.

<script src="/docs/components/typescript-sdk-type-links.js" defer />

<h2 id="installation">
  Установка
</h2>

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

<Note>
  SDK поставляется с нативным бинарным файлом Claude Code для вашей платформы в качестве опциональной зависимости, такой как `@anthropic-ai/claude-agent-sdk-darwin-arm64`. Вам не нужно устанавливать Claude Code отдельно. Если ваш менеджер пакетов пропускает опциональные зависимости, SDK выбросит ошибку `Native CLI binary for <platform> not found`; установите [`pathToClaudeCodeExecutable`](#options) на отдельно установленный бинарный файл `claude` вместо этого.
</Note>

<h3 id="compile-to-a-single-executable">
  Компиляция в единый исполняемый файл
</h3>

Когда вы компилируете приложение в единый исполняемый файл с помощью `bun build --compile`, SDK не может разрешить упакованный бинарный файл CLI во время выполнения. `require.resolve` не работает внутри виртуальной файловой системы `$bunfs` скомпилированного исполняемого файла, поэтому SDK выбросит ошибку `Native CLI binary for <platform> not found`.

Чтобы обойти это, встройте бинарный файл платформы как файловый ресурс, извлеките его на реальный путь при запуске с помощью `extractFromBunfs()` и передайте этот путь в [`pathToClaudeCodeExecutable`](#options).

Вспомогательная функция `extractFromBunfs()` требует `@anthropic-ai/claude-agent-sdk` версии 0.3.144 или позже. Пример ниже выполняет сборку для macOS на Apple Silicon:

```typescript theme={null}
import binPath from "@anthropic-ai/claude-agent-sdk-darwin-arm64/claude" with { type: "file" };
import { extractFromBunfs } from "@anthropic-ai/claude-agent-sdk/extract";
import { query } from "@anthropic-ai/claude-agent-sdk";

const cliPath = extractFromBunfs(binPath);

for await (const message of query({
  prompt: "Hello",
  options: { pathToClaudeCodeExecutable: cliPath },
})) {
  console.log(message);
}
```

`extractFromBunfs()` копирует встроенный бинарный файл из виртуальной файловой системы скомпилированного исполняемого файла в каталог временных файлов для каждого пользователя и возвращает реальный путь. Вне скомпилированного исполняемого файла он возвращает входной путь без изменений, поэтому тот же код работает в разработке без модификации.

Каждый скомпилированный исполняемый файл содержит бинарный файл одной платформы. Совместите пакет платформы в импорте с вашим `--target`:

* Для кросс-компиляции установите пакет несовпадающей платформы, например `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`.
* На Windows подпуть бинарного файла — это `claude.exe`, например `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`.

<h2 id="functions">
  Функции
</h2>

<h3 id="query">
  `query()`
</h3>

Основная функция для взаимодействия с Claude Code. Создаёт асинхронный генератор, который потоком передаёт сообщения по мере их поступления.

```typescript theme={null}
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query;
```

<h4 id="parameters">
  Параметры
</h4>

| Параметр  | Тип                                                              | Описание                                                                             |
| :-------- | :--------------------------------------------------------------- | :----------------------------------------------------------------------------------- |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | Входной запрос в виде строки или асинхронного итерируемого объекта для режима потока |
| `options` | [`Options`](#options)                                            | Опциональный объект конфигурации (см. тип Options ниже)                              |

<h4 id="returns">
  Возвращаемое значение
</h4>

Возвращает объект [`Query`](#query-object), который расширяет `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` дополнительными методами.

<h3 id="startup">
  `startup()`
</h3>

Предварительно разогревает подпроцесс CLI, запуская его и завершая инициализационное рукопожатие до того, как запрос будет доступен. Возвращённый дескриптор [`WarmQuery`](#warmquery) принимает запрос позже и записывает его в уже готовый процесс, поэтому первый вызов `query()` разрешается без затрат на запуск подпроцесса и инициализацию в строке.

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  Параметры
</h4>

| Параметр              | Тип                   | Описание                                                                                                                                                                       |
| :-------------------- | :-------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`             | [`Options`](#options) | Опциональный объект конфигурации. Аналогичен параметру `options` функции `query()`                                                                                             |
| `initializeTimeoutMs` | `number`              | Максимальное время в миллисекундах для ожидания инициализации подпроцесса. По умолчанию `60000`. Если инициализация не завершится вовремя, промис отклонится с ошибкой timeout |

<h4 id="returns-2">
  Возвращаемое значение
</h4>

Возвращает `Promise<`[`WarmQuery`](#warmquery)`>`, который разрешается после того, как подпроцесс запущен и завершил инициализационное рукопожатие.

<h4 id="example">
  Пример
</h4>

Вызовите `startup()` рано, например при загрузке приложения, затем вызовите `.query()` на возвращённом дескрипторе, когда запрос будет готов. Это перемещает запуск подпроцесса и инициализацию из критического пути.

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// Оплатите стоимость запуска заранее
const warm = await startup({ options: { maxTurns: 3 } });

// Позже, когда запрос готов, это происходит мгновенно
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

Создаёт определение типобезопасного MCP tool для использования с SDK MCP серверами.

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Параметры
</h4>

| Параметр      | Тип                                                               | Описание                                                                          |
| :------------ | :---------------------------------------------------------------- | :-------------------------------------------------------------------------------- |
| `name`        | `string`                                                          | Имя tool                                                                          |
| `description` | `string`                                                          | Описание того, что делает tool                                                    |
| `inputSchema` | `Schema extends AnyZodRawShape`                                   | Zod схема, определяющая входные параметры tool (поддерживает Zod 3 и Zod 4)       |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | Асинхронная функция, которая выполняет логику tool                                |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)` }`       | Опциональные аннотации MCP tool, предоставляющие поведенческие подсказки клиентам |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Переэкспортировано из `@modelcontextprotocol/sdk/types.js`. Все поля являются опциональными подсказками; клиенты не должны полагаться на них для решений безопасности.

| Поле              | Тип       | По умолчанию | Описание                                                                                                                                         |
| :---------------- | :-------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `string`  | `undefined`  | Удобочитаемое название для tool                                                                                                                  |
| `readOnlyHint`    | `boolean` | `false`      | Если `true`, tool не изменяет свою среду                                                                                                         |
| `destructiveHint` | `boolean` | `true`       | Если `true`, tool может выполнять деструктивные обновления (имеет смысл только когда `readOnlyHint` равен `false`)                               |
| `idempotentHint`  | `boolean` | `false`      | Если `true`, повторные вызовы с одинаковыми аргументами не имеют дополнительного эффекта (имеет смысл только когда `readOnlyHint` равен `false`) |
| `openWorldHint`   | `boolean` | `true`       | Если `true`, tool взаимодействует с внешними сущностями (например, веб-поиск). Если `false`, область tool закрыта (например, tool памяти)        |

```typescript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const searchTool = tool(
  "search",
  "Search the web",
  { query: z.string() },
  async ({ query }) => {
    return { content: [{ type: "text", text: `Results for: ${query}` }] };
  },
  { annotations: { readOnlyHint: true, openWorldHint: true } }
);
```

<h3 id="createsdkmcpserver">
  `createSdkMcpServer()`
</h3>

Создаёт экземпляр MCP сервера, который работает в том же процессе, что и ваше приложение.

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Параметры
</h4>

| Параметр          | Тип                           | Описание                                                       |
| :---------------- | :---------------------------- | :------------------------------------------------------------- |
| `options.name`    | `string`                      | Имя MCP сервера                                                |
| `options.version` | `string`                      | Опциональная строка версии                                     |
| `options.tools`   | `Array<SdkMcpToolDefinition>` | Массив определений tool, созданных с помощью [`tool()`](#tool) |

<h3 id="listsessions">
  `listSessions()`
</h3>

Обнаруживает и перечисляет прошлые сессии с лёгкими метаданными. Фильтруйте по директории проекта или перечисляйте сессии во всех проектах.

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
  Параметры
</h4>

| Параметр                   | Тип       | По умолчанию | Описание                                                                              |
| :------------------------- | :-------- | :----------- | :------------------------------------------------------------------------------------ |
| `options.dir`              | `string`  | `undefined`  | Директория для перечисления сессий. Если опущено, возвращает сессии во всех проектах  |
| `options.limit`            | `number`  | `undefined`  | Максимальное количество сессий для возврата                                           |
| `options.includeWorktrees` | `boolean` | `true`       | Когда `dir` находится внутри git репозитория, включайте сессии из всех путей worktree |

<h4 id="return-type-sdksessioninfo">
  Тип возврата: `SDKSessionInfo`
</h4>

| Свойство       | Тип                   | Описание                                                                                                 |
| :------------- | :-------------------- | :------------------------------------------------------------------------------------------------------- |
| `sessionId`    | `string`              | Уникальный идентификатор сессии (UUID)                                                                   |
| `summary`      | `string`              | Отображаемое название: пользовательское название, автоматически сгенерированное резюме или первый запрос |
| `lastModified` | `number`              | Время последнего изменения в миллисекундах с эпохи                                                       |
| `fileSize`     | `number \| undefined` | Размер файла сессии в байтах. Заполняется только для локального хранилища JSONL                          |
| `customTitle`  | `string \| undefined` | Пользовательское название сессии (через `/rename`)                                                       |
| `firstPrompt`  | `string \| undefined` | Первый значимый пользовательский запрос в сессии                                                         |
| `gitBranch`    | `string \| undefined` | Git ветка в конце сессии                                                                                 |
| `cwd`          | `string \| undefined` | Рабочая директория для сессии                                                                            |
| `tag`          | `string \| undefined` | Пользовательский тег сессии (см. [`tagSession()`](#tagsession))                                          |
| `createdAt`    | `number \| undefined` | Время создания в миллисекундах с эпохи, из временной метки первой записи                                 |

<h4 id="example-2">
  Пример
</h4>

Выведите 10 самых последних сессий для проекта. Результаты отсортированы по `lastModified` в убывающем порядке, поэтому первый элемент является самым новым. Опустите `dir` для поиска во всех проектах.

```typescript theme={null}
import { listSessions } from "@anthropic-ai/claude-agent-sdk";

const sessions = await listSessions({ dir: "/path/to/project", limit: 10 });

for (const session of sessions) {
  console.log(`${session.summary} (${session.sessionId})`);
}
```

<h3 id="getsessionmessages">
  `getSessionMessages()`
</h3>

Читает сообщения пользователя и ассистента из прошлой сессии.

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  Параметры
</h4>

| Параметр         | Тип      | По умолчанию | Описание                                                                  |
| :--------------- | :------- | :----------- | :------------------------------------------------------------------------ |
| `sessionId`      | `string` | обязательно  | UUID сессии для чтения (см. `listSessions()`)                             |
| `options.dir`    | `string` | `undefined`  | Директория проекта для поиска сессии. Если опущено, ищет во всех проектах |
| `options.limit`  | `number` | `undefined`  | Максимальное количество сообщений для возврата                            |
| `options.offset` | `number` | `undefined`  | Количество сообщений для пропуска с начала                                |

<h4 id="return-type-sessionmessage">
  Тип возврата: `SessionMessage`
</h4>

| Свойство             | Тип                     | Описание                                                                                                                                                                                                                                                                 |
| :------------------- | :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Роль сообщения                                                                                                                                                                                                                                                           |
| `uuid`               | `string`                | Уникальный идентификатор сообщения                                                                                                                                                                                                                                       |
| `session_id`         | `string`                | Сессия, к которой принадлежит это сообщение                                                                                                                                                                                                                              |
| `message`            | `unknown`               | Необработанная полезная нагрузка сообщения из транскрипта                                                                                                                                                                                                                |
| `parent_tool_use_id` | `string \| null`        | Для сообщений подагента, `tool_use_id` вызова tool `Agent`, который его запустил. `null` для сообщений основной сессии и более старых сессий                                                                                                                             |
| `parent_agent_id`    | `string \| null`        | Для сообщений от [вложенного подагента](/docs/ru/sub-agents#spawn-nested-subagents), `agentId` подагента, который его запустил. `null` для сообщений основной сессии, сообщений от подагентов верхнего уровня и более старых сессий. Требуется Claude Code v2.1.202 или позже |

<h4 id="example-3">
  Пример
</h4>

```typescript theme={null}
import { listSessions, getSessionMessages } from "@anthropic-ai/claude-agent-sdk";

const [latest] = await listSessions({ dir: "/path/to/project", limit: 1 });

if (latest) {
  const messages = await getSessionMessages(latest.sessionId, {
    dir: "/path/to/project",
    limit: 20
  });

  for (const msg of messages) {
    console.log(`[${msg.type}] ${msg.uuid}`);
  }
}
```

<h3 id="getsessioninfo">
  `getSessionInfo()`
</h3>

Читает метаданные для одной сессии по ID без сканирования полной директории проекта.

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  Параметры
</h4>

| Параметр      | Тип      | По умолчанию | Описание                                                                 |
| :------------ | :------- | :----------- | :----------------------------------------------------------------------- |
| `sessionId`   | `string` | обязательно  | UUID сессии для поиска                                                   |
| `options.dir` | `string` | `undefined`  | Путь директории проекта. Если опущено, ищет во всех директориях проектов |

Возвращает [`SDKSessionInfo`](#return-type-sdksessioninfo) или `undefined`, если сессия не найдена.

<h3 id="renamesession">
  `renameSession()`
</h3>

Переименовывает сессию, добавляя запись пользовательского названия. Повторные вызовы безопасны; побеждает самое последнее название.

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-8">
  Параметры
</h4>

| Параметр      | Тип      | По умолчанию | Описание                                                                 |
| :------------ | :------- | :----------- | :----------------------------------------------------------------------- |
| `sessionId`   | `string` | обязательно  | UUID сессии для переименования                                           |
| `title`       | `string` | обязательно  | Новое название. Должно быть непустым после удаления пробелов             |
| `options.dir` | `string` | `undefined`  | Путь директории проекта. Если опущено, ищет во всех директориях проектов |

<h3 id="tagsession">
  `tagSession()`
</h3>

Помечает сессию. Передайте `null` для очистки тега. Повторные вызовы безопасны; побеждает самый последний тег.

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-9">
  Параметры
</h4>

| Параметр      | Тип              | По умолчанию | Описание                                                                 |
| :------------ | :--------------- | :----------- | :----------------------------------------------------------------------- |
| `sessionId`   | `string`         | обязательно  | UUID сессии для пометки                                                  |
| `tag`         | `string \| null` | обязательно  | Строка тега или `null` для очистки                                       |
| `options.dir` | `string`         | `undefined`  | Путь директории проекта. Если опущено, ищет во всех директориях проектов |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

Разрешает эффективные параметры Claude Code для заданной директории, используя тот же механизм слияния, что и CLI, без запуска Claude CLI. Используйте его для проверки того, какую конфигурацию увидит вызов `query()` перед его вызовом.

<Note>
  Эта функция находится в альфа-версии и её API может измениться перед стабилизацией. Она читает источники MDM, включая macOS plist и Windows HKLM/HKCU, для паритета с запуском CLI, но не выполняет настроенный администратором подпроцесс `policyHelper`. Поле `permissions.defaultMode` возвращается как есть из всех уровней, включая параметры проекта. Фильтр доверия, который CLI применяет перед соблюдением режимов повышенных разрешений, не применяется.
</Note>

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Параметры
</h4>

`resolveSettings()` принимает один объект параметров. Все поля опциональны.

| Параметр                        | Тип                                   | По умолчанию    | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------------ | :------------------------------------ | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options.cwd`                   | `string`                              | `process.cwd()` | Директория для разрешения параметров проекта и локальных параметров относительно                                                                                                                                                                                                                                                                                                                                                            |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Все источники   | Какие источники файловой системы загружать. Передайте `[]` для пропуска пользовательских, проектных и локальных параметров. Параметры управляемой политики загружаются во всех случаях. Параметры, управляемые сервером, берутся из `serverManagedSettings`, когда хост их передаёт, или читаются из кэша CLI на диске в противном случае; снимок не загружает их из сети                                                                   |
| `options.managedSettings`       | `Settings`                            | `undefined`     | Ограничивающие параметры уровня политики, предоставленные хостом встраивания. Отбрасываются по умолчанию, когда присутствует развёрнутый администратором управляемый уровень; объединяются под этим уровнем, когда [`parentSettingsBehavior`](/docs/ru/settings#available-settings) равен `"merge"`. Неограничивающие ключи, такие как `model`, молча отбрасываются, поэтому этот параметр может усилить управляемую политику, но не ослабить её |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | Полезная нагрузка параметров, управляемых сервером, из `/api/claude_code/settings`. Неограничивающие ключи проходят без фильтрации                                                                                                                                                                                                                                                                                                          |

<h4 id="return-type-resolvedsettings">
  Тип возврата: `ResolvedSettings`
</h4>

`resolveSettings()` возвращает объект, описывающий объединённые параметры и источник, который внёс каждый ключ.

| Свойство     | Тип                                                 | Описание                                                                                                     |
| :----------- | :-------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| `effective`  | `Settings`                                          | Объединённые параметры после применения всех включённых источников в порядке приоритета                      |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | Для каждого ключа верхнего уровня в `effective`, какой источник предоставил значение                         |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | Необработанные параметры для каждого источника, упорядоченные от самого низкого к самому высокому приоритету |

<h4 id="example-4">
  Пример
</h4>

Пример ниже разрешает параметры для директории проекта и выводит источник, который контролирует период очистки.

```typescript theme={null}
import { resolveSettings } from "@anthropic-ai/claude-agent-sdk";

const { effective, provenance } = await resolveSettings({
  cwd: "/path/to/project",
  settingSources: ["user", "project", "local"],
});

console.log(`Cleanup period: ${effective.cleanupPeriodDays} days`);
console.log(`Set by: ${provenance.cleanupPeriodDays?.source}`);
```

<h2 id="types">
  Типы
</h2>

<h3 id="options">
  `Options`
</h3>

Объект конфигурации для функции `query()`.

| Свойство                          | Тип                                                                                                      | По умолчанию                                             | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `abortController`                 | `AbortController`                                                                                        | `new AbortController()`                                  | Контроллер для отмены операций                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `additionalDirectories`           | `string[]`                                                                                               | `[]`                                                     | Дополнительные директории, к которым Claude может получить доступ                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `agent`                           | `string`                                                                                                 | `undefined`                                              | Имя агента для основного потока. Агент должен быть определён в опции `agents` или в настройках                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                  | `undefined`                                              | Программно определите подагентов                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `agentProgressSummaries`          | `boolean`                                                                                                | `false`                                                  | Когда `true`, генерируйте однострочные сводки прогресса для подагентов и пересылайте их на события [`task_progress`](#sdktaskprogressmessage) через поле `summary`. Применяется к подагентам переднего плана и фонового режима                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                | `false`                                                  | Включите обход разрешений. Требуется при использовании `permissionMode: 'bypassPermissions'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `allowedTools`                    | `string[]`                                                                                               | `[]`                                                     | Инструменты для автоматического одобрения без запроса. Это не ограничивает Claude только этими инструментами; неперечисленные инструменты переходят к `permissionMode` и `canUseTool`. Используйте `disallowedTools` для блокировки инструментов. См. [Разрешения](/docs/ru/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                  |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                | `[]`                                                     | Включите бета-функции                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                              | `undefined`                                              | Пользовательская функция разрешения, вызываемая только когда [поток разрешения](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) переходит к запросу. Не вызывается для вызовов, автоматически одобренных `allowedTools`, правилами разрешения или `permissionMode`. `AskUserQuestion`, инструменты соединителя [установленные вашей организацией на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) и инструменты MCP, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool), достигают её даже если вы их разрешили; в режиме `dontAsk` они вместо этого отклоняются. См. [`CanUseTool`](#canusetool) для деталей |
| `continue`                        | `boolean`                                                                                                | `false`                                                  | Продолжите самый последний диалог                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `cwd`                             | `string`                                                                                                 | `process.cwd()`                                          | Текущая рабочая директория                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `debug`                           | `boolean`                                                                                                | `false`                                                  | Включите режим отладки для процесса Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `debugFile`                       | `string`                                                                                                 | `undefined`                                              | Запишите журналы отладки в определённый путь файла. Неявно включает режим отладки                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `disallowedTools`                 | `string[]`                                                                                               | `[]`                                                     | Инструменты для отклонения. Простое имя, такое как `"Bash"`, удаляет инструмент из контекста Claude. Правило с областью видимости, такое как `"Bash(rm *)"`, оставляет инструмент доступным и отклоняет совпадающие вызовы в каждом режиме разрешения, включая `bypassPermissions`. См. [Разрешения](/docs/ru/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                        | По умолчанию модели                                      | Контролирует, сколько усилий Claude вкладывает в свой ответ. Работает с адаптивным мышлением для направления глубины мышления. См. [adjust the effort level](/docs/ru/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `enableFileCheckpointing`         | `boolean`                                                                                                | `false`                                                  | Включите отслеживание изменений файлов для перемотки. См. [File checkpointing](/docs/ru/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `env`                             | `Record<string, string \| undefined>`                                                                    | `process.env`                                            | Переменные окружения. Когда установлено, это заменяет окружение подпроцесса вместо объединения с `process.env`, поэтому передайте `{ ...process.env, YOUR_VAR: 'value' }` для сохранения унаследованных переменных, таких как `PATH`. См. [Handle slow or stalled API responses](#handle-slow-or-stalled-api-responses) для примера этого паттерна и [Environment variables](/docs/ru/env-vars) для переменных, которые читает базовый CLI. Установите `CLAUDE_AGENT_SDK_CLIENT_APP` для идентификации вашего приложения в заголовке User-Agent                                                                                                                          |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                              | Автоопределение                                          | Среда выполнения JavaScript для использования                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `executableArgs`                  | `string[]`                                                                                               | `[]`                                                     | Аргументы для передачи исполняемому файлу                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `extraArgs`                       | `Record<string, string \| null>`                                                                         | `{}`                                                     | Дополнительные аргументы                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `fallbackModel`                   | `string`                                                                                                 | `undefined`                                              | Модель для использования, если основная не работает                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `forkSession`                     | `boolean`                                                                                                | `false`                                                  | При возобновлении с `resume` разветвитесь на новый ID сессии вместо продолжения исходной сессии                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `forwardSubagentText`             | `boolean`                                                                                                | `false`                                                  | Пересылайте текст подагента и блоки мышления как сообщения ассистента и пользователя с установленным `parent_tool_use_id`, чтобы потребители могли отобразить вложенный транскрипт. По умолчанию только блоки `tool_use` и `tool_result` от подагентов выдаются                                                                                                                                                                                                                                                                                                                                                                                                     |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`        | `{}`                                                     | Обратные вызовы hooks для событий                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `includeHookEvents`               | `boolean`                                                                                                | `false`                                                  | Включите события жизненного цикла hooks для каждого события hooks в поток сообщений как [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage) и [`SDKHookResponseMessage`](#sdkhookresponsemessage). События жизненного цикла для hooks `SessionStart` и `Setup` всегда включены и не требуют этой опции                                                                                                                                                                                                                                                                                                           |
| `includePartialMessages`          | `boolean`                                                                                                | `false`                                                  | Включите события частичных сообщений                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `loadTimeoutMs`                   | `number`                                                                                                 | `60000`                                                  | *Alpha.* Timeout в миллисекундах для каждого вызова `sessionStore.load()` и `sessionStore.listSubkeys()` во время материализации возобновления. Если адаптер не завершится в этом окне, запрос не удаётся вместо зависания. Игнорируется, когда `sessionStore` не установлен                                                                                                                                                                                                                                                                                                                                                                                        |
| `managedSettings`                 | `Settings`                                                                                               | `undefined`                                              | Настройки уровня политики, предоставленные порождающим родительским процессом. Отбрасываются, когда уровень управляемых настроек, контролируемый IT, уже существует на машине, если только этот администратор не согласится с `parentSettingsBehavior: 'merge'`. Отфильтрованы только для ключей, ограничивающих доступ, независимо                                                                                                                                                                                                                                                                                                                                 |
| `maxBudgetUsd`                    | `number`                                                                                                 | `undefined`                                              | Остановите запрос, когда оценка стоимости на стороне клиента достигнет этого значения USD. Сравнивается с той же оценкой, что и `total_cost_usd`; см. [Track cost and usage](/docs/ru/agent-sdk/cost-tracking) для предостережений точности                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `maxThinkingTokens`               | `number`                                                                                                 | `undefined`                                              | *Устарело:* Используйте вместо этого `thinking`. Максимальные токены для процесса мышления                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `maxTurns`                        | `number`                                                                                                 | `undefined`                                              | Максимальное количество агентских ходов (раунды использования инструмента)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                  | `{}`                                                     | Конфигурации MCP серверов                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `model`                           | `string`                                                                                                 | По умолчанию из CLI                                      | Псевдоним модели Claude или полное имя модели. См. [accepted values and provider-specific IDs](/docs/ru/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`          | `undefined`                                              | Обратный вызов для обработки запросов MCP elicitation. Вызывается, когда MCP сервер запрашивает ввод пользователя и ни один hook не обрабатывает его первым. Если не предоставлено, необработанные запросы elicitation автоматически отклоняются                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                            | `undefined`                                              | Определите формат вывода для результатов агента. См. [Structured outputs](/docs/ru/agent-sdk/structured-outputs) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `outputStyle`                     | `string`                                                                                                 | `undefined`                                              | Не поле `Options`. Установите `outputStyle` во встроенном объекте [`settings`](/docs/ru/settings) или файле настроек вместо этого. См. [Activate an output style](/docs/ru/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                 | Автоопределение из встроенного нативного бинарного файла | Путь к исполняемому файлу Claude Code. Требуется только если опциональные зависимости были пропущены при установке или ваша платформа не в поддерживаемом наборе                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                      | `'default'`                                              | Режим разрешения для сессии                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `permissionPromptToolName`        | `string`                                                                                                 | `undefined`                                              | Имя MCP инструмента для запросов разрешения                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `persistSession`                  | `boolean`                                                                                                | `true`                                                   | Когда `false`, отключает сохранение сессии на диск. Сессии не могут быть возобновлены позже                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `planModeInstructions`            | `string`                                                                                                 | `undefined`                                              | Пользовательские инструкции рабочего процесса для Plan Mode. Когда `permissionMode` это `'plan'`, эта строка заменяет тело рабочего процесса режима плана по умолчанию. CLI по-прежнему оборачивает его с преамбулой принудительного соблюдения только для чтения и нижним колонтитулом протокола ExitPlanMode                                                                                                                                                                                                                                                                                                                                                      |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                | `[]`                                                     | Загружайте пользовательские plugins из локальных путей. См. [Plugins](/docs/ru/agent-sdk/plugins) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `promptSuggestions`               | `boolean`                                                                                                | `false`                                                  | Включите предложения запросов. Выдаёт сообщение `prompt_suggestion` после каждого хода с предсказанным следующим пользовательским запросом                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `resume`                          | `string`                                                                                                 | `undefined`                                              | ID сессии для возобновления                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `resumeSessionAt`                 | `string`                                                                                                 | `undefined`                                              | Возобновите сессию в определённом UUID сообщения                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                    | `undefined`                                              | Программно настройте поведение sandbox. См. [Sandbox settings](#sandboxsettings) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `sessionId`                       | `string`                                                                                                 | Автогенерируемый                                         | Используйте определённый UUID для сессии вместо автогенерирования                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `sessionStore`                    | [`SessionStore`](/docs/ru/agent-sdk/session-storage#the-sessionstore-interface)                               | `undefined`                                              | Зеркалируйте транскрипты сессий на внешний бэкенд, чтобы любой хост мог их возобновить. См. [Persist sessions to external storage](/docs/ru/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                   | `'batched'`                                              | *Alpha.* Режим flush для `sessionStore`. Игнорируется, когда `sessionStore` не установлен                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `settings`                        | `string \| Settings`                                                                                     | `undefined`                                              | Встроенный объект [settings](/docs/ru/settings) или путь к файлу настроек. Заполняет слой flag-settings в [порядке приоритета](/docs/ru/settings#settings-precedence). Измените во время выполнения с помощью [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                    | Значения по умолчанию CLI (все источники)                | Контролируйте, какие настройки файловой системы загружать. Передайте `[]` для отключения пользовательских, проектных и локальных настроек. Управляемые политикой настройки загружаются независимо; серверные управляемые настройки загружаются, когда сессия аутентифицируется с учётными данными организации на [подходящей конфигурации](/docs/ru/server-managed-settings#platform-availability). См. [Use Claude Code features](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                              |
| `skills`                          | `string[] \| 'all'`                                                                                      | `undefined`                                              | Skills доступные для сессии. Передайте `'all'` для включения каждого обнаруженного skill, или список имён skills. Когда установлено, SDK автоматически добавляет инструмент Skill в `allowedTools`. Если вы также передаёте `tools`, включите `'Skill'` в этот список. См. [Skills](/docs/ru/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                           |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                              | `undefined`                                              | Пользовательская функция для запуска процесса Claude Code. Используйте для запуска Claude Code на ВМ, контейнерах или удалённых окружениях                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `stderr`                          | `(data: string) => void`                                                                                 | `undefined`                                              | Обратный вызов для вывода stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `strictMcpConfig`                 | `boolean`                                                                                                | `false`                                                  | Используйте только серверы, переданные в `mcpServers`, и игнорируйте проект `.mcp.json`, пользовательские настройки, MCP серверы, предоставленные plugins, и [claude.ai connectors](/docs/ru/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `systemPrompt`                    | `string \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean }` | `undefined` (минимальный запрос)                         | Конфигурация системного запроса. Передайте строку для пользовательского запроса или `{ type: 'preset', preset: 'claude_code' }` для использования системного запроса Claude Code. При использовании формы объекта preset добавьте `append` для расширения его дополнительными инструкциями и установите `excludeDynamicSections: true` для перемещения контекста для каждой сессии в первое пользовательское сообщение для [лучшего переиспользования prompt caching на разных машинах](/docs/ru/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)                                                                                    |
| `taskBudget`                      | `{ total: number }`                                                                                      | `undefined`                                              | *Alpha.* Бюджет задачи на стороне API в токенах. Когда установлено, модели сообщается её оставшийся бюджет токенов, чтобы она могла регулировать использование инструмента и завершить работу до лимита                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                      | `{ type: 'adaptive' }` для поддерживаемых моделей        | Контролирует поведение мышления/рассуждения Claude. См. [`ThinkingConfig`](#thinkingconfig) для опций                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `title`                           | `string`                                                                                                 | `undefined`                                              | Отображаемое название для сессии. При возобновлении через `resume` или `continue`, сохранённое название возобновляемой сессии имеет приоритет; используйте [`renameSession()`](#renamesession) для переименования существующей сессии                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `toolAliases`                     | `Record<string, string>`                                                                                 | `undefined`                                              | Отображайте встроенные имена инструментов на имена MCP инструментов, чтобы Claude вызывал вашу реализацию MCP вместо встроенной. Например, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                              | `undefined`                                              | Конфигурация для встроенного поведения инструмента. См. [`ToolConfig`](#toolconfig) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                  | `undefined`                                              | Конфигурация инструмента. Передайте массив имён инструментов или используйте preset для получения встроенных инструментов Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

<h4 id="handle-slow-or-stalled-api-responses">
  Handle slow or stalled API responses
</h4>

Подпроцесс CLI читает несколько переменных окружения, которые контролируют timeout API и обнаружение зависания. Передайте их через опцию `env`:

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    env: {
      ...process.env,
      API_TIMEOUT_MS: "120000",
      CLAUDE_CODE_MAX_RETRIES: "2",
      CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS: "120000",
    },
  },
});
```

* `API_TIMEOUT_MS`: timeout для каждого запроса на клиенте Anthropic, в миллисекундах. По умолчанию `600000`. Применяется к основному циклу и всем подагентам.
* `CLAUDE_CODE_MAX_RETRIES`: максимальное количество повторных попыток API. По умолчанию `10`, ограничено `15`. Каждая повторная попытка получает своё собственное окно `API_TIMEOUT_MS`, поэтому наихудший случай wall time примерно `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` плюс backoff. Для автоматических запусков, которым нужно ждать через более длительные сбои, установите `CLAUDE_CODE_RETRY_WATCHDOG=1`: он повторяет ошибки ёмкости бесконечно, и начиная с Claude Code v2.1.199 повышает значение по умолчанию для других переходящих ошибок до `300` и удаляет ограничение на эту переменную.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: watchdog зависания для подагентов, запущенных с `run_in_background`. По умолчанию `600000`. Сбрасывается при каждом событии потока; при зависании прерывает подагента, отмечает задачу как неудачную и выводит ошибку родителю с любым частичным результатом. Не применяется к синхронным подагентам.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` с `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: прерывает запрос, когда заголовки получены, но тело ответа перестаёт потоком передаваться. Watchdog включен по умолчанию для всех поставщиков; установите `CLAUDE_ENABLE_STREAM_WATCHDOG=0` для отключения. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` по умолчанию `300000` и зажимается до этого минимума. Прерванный запрос проходит через обычный путь повторной попытки.

<h3 id="query-object">
  `Query` object
</h3>

Интерфейс, возвращаемый функцией `query()`.

```typescript theme={null}
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<SDKControlInterruptResponse | undefined>;
  rewindFiles(
    userMessageId: string,
    options?: { dryRun?: boolean }
  ): Promise<RewindFilesResult>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  applyFlagSettings(settings: { [K in keyof Settings]?: Settings[K] | null }): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Methods
</h4>

| Метод                                  | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Прерывает запрос. Доступно только в режиме потока входных данных. Когда CLI объявляет возможность `interrupt_receipt_v1` в [`SDKSystemMessage.capabilities`](#sdksystemmessage), разрешается с помощью [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse), в котором перечислены поставленные в очередь сообщения, которые пережили прерывание. Разрешается `undefined` на CLI до v2.1.205                                                                                                            |
| `rewindFiles(userMessageId, options?)` | Восстанавливает файлы в их состояние в указанном пользовательском сообщении. Передайте `{ dryRun: true }` для предпросмотра изменений. Требует `enableFileCheckpointing: true`. См. [File checkpointing](/docs/ru/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                       |
| `setPermissionMode()`                  | Изменяет режим разрешения (доступно только в режиме потока входных данных)                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `setModel()`                           | Изменяет модель (доступно только в режиме потока входных данных)                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `setMaxThinkingTokens()`               | *Устарело:* Используйте вместо этого опцию `thinking`. Изменяет максимальные токены мышления. Передача `null` сбрасывает мышление к значению по умолчанию сессии: переопределение в середине сессии очищается, и мышление остаётся отключённым для сессий, у которых оно отключено                                                                                                                                                                                                                               |
| `applyFlagSettings(settings)`          | Объединяет настройки в слой flag settings сессии во время выполнения (доступно только в режиме потока входных данных). См. [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                           |
| `initializationResult()`               | Возвращает полный результат инициализации, включая поддерживаемые команды, модели, информацию об учётной записи и конфигурацию стиля вывода                                                                                                                                                                                                                                                                                                                                                                      |
| `reinitialize()`                       | Повторно отправляет запрос управления `initialize` работающему CLI и возвращает свежий результат вместо кэшированного результата первого подключения. Используйте его после разрыва транспорта, такого как переподключение к сессии после отключения, чтобы ожидающие запросы разрешения снова достигли вашего обратного вызова `canUseTool`. Сделайте обратный вызов идемпотентным для каждого ID запроса, потому что запрос, чей ответ был потерян, отправляется снова. Требует Claude Code v2.1.195 или позже |
| `supportedCommands()`                  | Возвращает доступные slash commands                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `supportedModels()`                    | Возвращает доступные модели с информацией отображения                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `supportedAgents()`                    | Возвращает доступные подагентов как [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `mcpServerStatus()`                    | Возвращает статус подключённых MCP серверов                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `accountInfo()`                        | Возвращает информацию об учётной записи                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `reconnectMcpServer(serverName)`       | Переподключитесь к MCP серверу по имени                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `toggleMcpServer(serverName, enabled)` | Включите или отключите MCP сервер по имени                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `setMcpServers(servers)`               | Динамически замените набор MCP серверов для этой сессии. Возвращает информацию о том, какие серверы были добавлены, удалены и какие ошибки                                                                                                                                                                                                                                                                                                                                                                       |
| `streamInput(stream)`                  | Потоком передавайте входные сообщения к запросу для многооборотных диалогов                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `stopTask(taskId)`                     | Остановите выполняющуюся фоновую задачу по ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `close()`                              | Закройте запрос и завершите базовый процесс. Принудительно завершает запрос и очищает все ресурсы                                                                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Изменяет [настройки](/docs/ru/settings) на работающей сессии без перезагрузки запроса. Используйте её, когда настройка, у которой нет выделенного setter, должна измениться в середине сессии, например, ужесточение `permissions` после того, как агент прочитает ненадёжный ввод. `setModel()` и `setPermissionMode()` являются выделенными setters для этих двух ключей; `applyFlagSettings()` является общей формой, которая принимает любое подмножество ключей настроек, и передача `model` здесь ведёт себя так же, как `setModel()`.

Только некоторые ключи вступают в силу в середине сессии:

* **Применяется на следующем ходу**: `model`, `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Переключение `agent` также применяет переопределение модели этого агента, hooks и системный запрос на следующем ходу.
* **Нет эффекта в середине сессии**: опции системного запроса. Они разрешаются один раз при запуске, поэтому работающая сессия сохраняет исходное значение, даже если вызов успешен. Чтобы их изменить, запустите новую сессию.

`effortLevel` принимает имя [уровня усилий](/docs/ru/model-config#adjust-effort-level). Он также принимает `"ultracode"`, который запускает сессию на уровне усилий `xhigh` и включает [ultracode](/docs/ru/workflows#let-claude-decide-with-ultracode). Тип `Settings` объявляет `effortLevel` без этого значения, поэтому передайте эквивалент `{ ultracode: true }` в TypeScript. Значение `ultracode` требует Claude Code v2.1.203 или позже и принимается только `applyFlagSettings()`, а не ключом `effortLevel` в файле настроек.

Значения записываются в слой flag-settings, тот же слой, который встроенная опция `settings` функции `query()` заполняет при запуске. Flag settings находятся рядом с верхней частью [порядка приоритета настроек](/docs/ru/settings#settings-precedence): они переопределяют пользовательские, проектные и локальные настройки, и только управляемые политикой настройки могут их переопределить. Это тот же уровень, который [раздел приоритета на странице](#settings-precedence) называет программными опциями.

Последовательные вызовы выполняют shallow-merge ключей верхнего уровня. Второй вызов с `{ permissions: {...} }` заменяет весь объект `permissions` из предыдущего вызова, а не выполняет deep-merge в него. Чтобы очистить ключ из слоя flag и вернуться к источникам с более низким приоритетом, передайте `null` для этого ключа. Передача `undefined` не имеет эффекта, потому что сериализация JSON её отбрасывает.

Доступно только в режиме потока входных данных, то же ограничение, что и `setModel()` и `setPermissionMode()`.

Пример ниже переключает активную модель в середине сессии, а затем очищает переопределение, чтобы модель вернулась к тому, что указывают пользовательские или проектные настройки.

```typescript theme={null}
const q = query({ prompt: messageStream });

// Переопределите модель для остальной части сессии
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Позже: очистите переопределение и вернитесь к настройкам с более низким приоритетом
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` только для TypeScript. Python SDK не предоставляет эквивалентный метод.
</Note>

<h3 id="warmquery">
  `WarmQuery`
</h3>

Дескриптор, возвращаемый [`startup()`](#startup). Подпроцесс уже запущен и инициализирован, поэтому вызов `query()` на этом дескрипторе записывает запрос непосредственно в готовый процесс без задержки запуска.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Methods
</h4>

| Метод           | Описание                                                                                                                                       |
| :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| `query(prompt)` | Отправьте запрос к предварительно разогретому подпроцессу и верните [`Query`](#query-object). Может быть вызван только один раз на `WarmQuery` |
| `close()`       | Закройте подпроцесс без отправки запроса. Используйте это для отказа от тёплого запроса, который больше не нужен                               |

`WarmQuery` реализует `AsyncDisposable`, поэтому его можно использовать с `await using` для автоматической очистки.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

Тип возврата `initializationResult()`. Содержит данные инициализации сессии.

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
};
```

Когда клиент отправляет `initialize` сессии, которая уже работает, обёртка control-response также содержит опциональный массив `pending_permission_requests`. Поле находится на самой обёртке response, а не в полезной нагрузке `SDKControlInitializeResponse` выше. Каждая запись является полным сообщением `control_request` с той же формой `{ type: "control_request", request_id, request }`, которую сессия потоком передаёт для запросов разрешения во время работы.

Это запросы, которые были выданы до подключения клиента и всё ещё ожидают ответа. SDK читает массив для вас и отправляет каждую запись в ваш обратный вызов [`canUseTool`](#canusetool), то же переотправление, которое [`reinitialize()`](#query-object) запускает после разрыва транспорта. Обрабатывайте повторяющиеся ID запросов идемпотентно, потому что запись может повторить запрос, который обратный вызов уже получил до отключения соединения.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

Квитанция прерывания: значение, которое [`interrupt()`](#query-object) разрешается с помощью на CLI, который объявляет возможность `interrupt_receipt_v1` в [`SDKSystemMessage.capabilities`](#sdksystemmessage). Требует Claude Code v2.1.205 или позже. Более ранние CLI отвечают на прерывание с пустой полезной нагрузкой успеха, поэтому `interrupt()` разрешается `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
};
```

`still_queued` перечисляет UUID пользовательских сообщений, которые пережили прерывание: сообщения всё ещё в очереди, плюс любой пакет уже выведенный для следующего хода, но ещё не достижимый прерыванием. Каждое из них работает как свой собственный ход после прерывания, если вы его не отмените первым. Используйте квитанцию, чтобы решить, нужно ли что-то переотправлять; переотправка сообщения, которое уже указано, создаёт дублирующийся ход.

Интерпретируйте список с этими предостережениями:

* Только сообщения, которые были поставлены в очередь с UUID, появляются. Пустой массив не означает, что ничего больше не будет работать.
* Только сообщения основного потока указаны. Сообщения, адресованные подагенту, выходят за рамки.
* Список может включать UUID, которые ваш клиент никогда не отправлял, такие как триггеры [scheduled task](/docs/ru/scheduled-tasks). Игнорируйте UUID, которые вы не узнаёте, вместо того чтобы рассматривать их как ошибку.

Квитанция — это снимок, сделанный в момент обработки прерывания, и при чистом прерывании она прибывает до [`SDKResultMessage`](#sdkresultmessage) прерванного хода. Прочитайте квитанцию, а не проверяйте очередь после этого результата: цикл немедленно запускает следующий поставленный в очередь ход, поэтому очередь, которую вы проверяете после результата, уже изменилась.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Конфигурация для подагента, определённого программно.

```typescript theme={null}
type AgentDefinition = {
  description: string;
  tools?: string[];
  disallowedTools?: string[];
  prompt: string;
  model?: string;
  mcpServers?: AgentMcpServerSpec[];
  skills?: string[];
  initialPrompt?: string;
  maxTurns?: number;
  background?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Поле                                  | Обязательно | Описание                                                                                                                                                                                                                                                |
| :------------------------------------ | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`                         | Да          | Описание на естественном языке, когда использовать этого агента                                                                                                                                                                                         |
| `tools`                               | Нет         | Массив разрешённых имён инструментов. Если опущено, наследует все инструменты от родителя. Для предварительной загрузки Skills в контекст агента используйте поле `skills` вместо указания `'Skill'` здесь                                              |
| `disallowedTools`                     | Нет         | Массив имён инструментов для явного запрещения для этого агента. Также принимаются паттерны уровня MCP сервера: `mcp__server` или `mcp__server__*` удаляет каждый инструмент с этого сервера, и `mcp__*` удаляет каждый MCP инструмент с любого сервера |
| `prompt`                              | Да          | Системный запрос агента                                                                                                                                                                                                                                 |
| `model`                               | Нет         | Переопределение модели для этого агента. Принимает псевдоним, такой как `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, или полный ID модели. Если опущено или `'inherit'`, использует основную модель                                         |
| `mcpServers`                          | Нет         | Спецификации MCP серверов для этого агента                                                                                                                                                                                                              |
| `skills`                              | Нет         | Массив имён skills для предварительной загрузки в контекст агента                                                                                                                                                                                       |
| `initialPrompt`                       | Нет         | Автоматически отправляется как первый пользовательский ход, когда этот агент работает как агент основного потока                                                                                                                                        |
| `maxTurns`                            | Нет         | Максимальное количество агентских ходов (раунды API) перед остановкой                                                                                                                                                                                   |
| `background`                          | Нет         | Запустите этого агента как неблокирующую фоновую задачу при вызове                                                                                                                                                                                      |
| `memory`                              | Нет         | Источник памяти для этого агента: `'user'`, `'project'` или `'local'`                                                                                                                                                                                   |
| `effort`                              | Нет         | Уровень усилий рассуждения для этого агента. Принимает именованный уровень или целое число                                                                                                                                                              |
| `permissionMode`                      | Нет         | Режим разрешения для выполнения инструмента в этом агенте. См. [`PermissionMode`](#permissionmode)                                                                                                                                                      |
| `criticalSystemReminder_EXPERIMENTAL` | Нет         | Экспериментально: Критическое напоминание, добавленное в системный запрос                                                                                                                                                                               |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

Указывает MCP серверы, доступные подагенту. Может быть именем сервера (строка, ссылающаяся на сервер из конфигурации `mcpServers` родителя) или встроенной конфигурацией сервера, записью, отображающей имена серверов на конфигурации.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

Где `McpServerConfigForProcessTransport` это `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`.

<h3 id="settingsource">
  `SettingSource`
</h3>

Контролирует, какие источники конфигурации на основе файловой системы SDK загружает настройки из.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| Значение    | Описание                                                | Местоположение                |
| :---------- | :------------------------------------------------------ | :---------------------------- |
| `'user'`    | Глобальные пользовательские настройки                   | `~/.claude/settings.json`     |
| `'project'` | Общие настройки проекта (контролируемые версией)        | `.claude/settings.json`       |
| `'local'`   | Локальные настройки проекта (не контролируемые версией) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Default behavior
</h4>

Когда `settingSources` опущено или `undefined`, `query()` загружает те же настройки файловой системы, что и CLI Claude Code: пользовательские, проектные и локальные. Управляемые политикой настройки загружаются во всех случаях; серверные управляемые настройки загружаются, когда сессия аутентифицируется с учётными данными организации на [подходящей конфигурации](/docs/ru/server-managed-settings#platform-availability). См. [What settingSources does not control](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control) для входных данных, которые читаются независимо от этой опции, и как их отключить.

<h4 id="why-use-settingsources">
  Why use settingSources
</h4>

**Отключите настройки файловой системы:**

```typescript theme={null}
// Не загружайте пользовательские, проектные или локальные настройки с диска
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Загружайте все настройки файловой системы явно:**

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ["user", "project", "local"] // Загружайте все настройки
  }
});
```

**Загружайте только определённые источники настроек:**

```typescript theme={null}
// Загружайте только настройки проекта, игнорируйте пользовательские и локальные
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Только .claude/settings.json
  }
});
```

**Тестирование и окружения CI:**

```typescript theme={null}
// Обеспечьте согласованное поведение в CI, исключив локальные настройки
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ["project"], // Только общие командные настройки
    permissionMode: "bypassPermissions"
  }
});
```

**SDK-только приложения:**

```typescript theme={null}
// Определите всё программно.
// Передайте [] для отказа от источников настроек файловой системы.
const result = query({
  prompt: "Review this PR",
  options: {
    settingSources: [],
    agents: {
      /* ... */
    },
    mcpServers: {
      /* ... */
    },
    allowedTools: ["Read", "Grep", "Glob"]
  }
});
```

**Загрузка инструкций проекта CLAUDE.md:**

```typescript theme={null}
// Загружайте настройки проекта для включения файлов CLAUDE.md
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code" // Используйте системный запрос Claude Code
    },
    settingSources: ["project"], // Загружает CLAUDE.md из директории проекта
    allowedTools: ["Read", "Write", "Edit"]
  }
});
```

<h4 id="settings-precedence">
  Settings precedence
</h4>

Когда загружаются несколько источников, настройки объединяются с этим приоритетом (от высшего к низшему):

1. Локальные настройки (`.claude/settings.local.json`)
2. Настройки проекта (`.claude/settings.json`)
3. Пользовательские настройки (`~/.claude/settings.json`)

Программные опции, такие как `agents`, `allowedTools` и `settings`, переопределяют пользовательские, проектные и локальные настройки файловой системы. Управляемые политикой настройки имеют приоритет над программными опциями.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Стандартное поведение разрешения
  | "acceptEdits" // Автоматически принимайте редактирования файлов
  | "bypassPermissions" // Обойдите все проверки разрешения; явные правила запроса всё ещё запрашивают
  | "plan" // Plan Mode - исследуйте без редактирования
  | "dontAsk" // Не запрашивайте разрешения, отклоняйте, если не предварительно одобрено
  | "auto"; // Используйте классификатор модели для одобрения или отклонения каждого вызова инструмента
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Тип пользовательской функции разрешения для контроля использования инструмента.

Функция является заменой SDK для интерактивного запроса разрешения: она вызывается только когда [поток оценки разрешения](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) разрешается в запрос. Вызовы инструментов, уже одобренные записью `allowedTools`, правилом настроек разрешения или режимом разрешения, такие как `acceptEdits` или `bypassPermissions`, никогда её не вызывают. Чтобы контролировать каждый вызов инструмента, используйте вместо этого [hook `PreToolUse`](/docs/ru/agent-sdk/hooks).

`AskUserQuestion`, инструменты MCP, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool) и инструменты соединителя [установленные вашей организацией на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) достигают функции даже когда правило разрешения совпадает. В режиме `dontAsk` эти вызовы вместо этого отклоняются, без вызова её.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| Опция            | Тип                                         | Описание                                                                                                                                                                                                                                                                                                                                     |
| :--------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Сигнализируется, если операция должна быть отменена                                                                                                                                                                                                                                                                                          |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Предложенные обновления разрешения, чтобы пользователь не был запрошен снова для этого инструмента. Запросы Bash включают предложение с назначением `localSettings` [destination](#permissionupdatedestination), поэтому возврат его в `updatedPermissions` записывает правило в `.claude/settings.local.json` и сохраняется между сессиями. |
| `blockedPath`    | `string`                                    | Путь файла, который вызвал запрос разрешения, если применимо                                                                                                                                                                                                                                                                                 |
| `decisionReason` | `string`                                    | Объясняет, почему был вызван этот запрос разрешения                                                                                                                                                                                                                                                                                          |
| `toolUseID`      | `string`                                    | Уникальный идентификатор для этого конкретного вызова инструмента в сообщении ассистента                                                                                                                                                                                                                                                     |
| `agentID`        | `string`                                    | Если работает в подагенте, ID подагента                                                                                                                                                                                                                                                                                                      |
| `requestId`      | `string`                                    | `request_id` обёртки `control_request`. `control_response`, которую ваше приложение отправляет вне SDK, такую как подписанный HTTP POST, должна повторить это значение, чтобы процесс Claude Code мог сопоставить ответ с запросом                                                                                                           |

Обратный вызов обычно разрешает запрос, возвращая [`PermissionResult`](#permissionresult), который SDK записывает обратно через свой транспорт как `control_response`. Возвращайте `null` только когда ваше приложение уже отправило `control_response` для этого запроса через свой собственный канал, повторив `requestId`; SDK затем пропускает запись ответа в свой транспорт. Возврат `null` в любом другом случае оставляет вызов инструмента заблокированным бесконечно, потому что `control_response` никогда не отправляется и запросы разрешения не имеют timeout.

Опция `requestId` и возвращаемое значение `null` требуют Claude Code v2.1.199 или позже.

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Результат проверки разрешения.

```typescript theme={null}
type PermissionResult =
  | {
      behavior: "allow";
      updatedInput?: Record<string, unknown>;
      updatedPermissions?: PermissionUpdate[];
      toolUseID?: string;
    }
  | {
      behavior: "deny";
      message: string;
      interrupt?: boolean;
      toolUseID?: string;
    };
```

<h3 id="toolconfig">
  `ToolConfig`
</h3>

Конфигурация для встроенного поведения инструмента.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| Поле                            | Тип                    | Описание                                                                                                                                                                                    |
| :------------------------------ | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Выбирает поле `preview` на опциях [`AskUserQuestion`](/docs/ru/agent-sdk/user-input#question-format) и устанавливает его формат содержимого. Если не установлено, Claude не выдаёт предпросмотры |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Конфигурация для MCP серверов.

```typescript theme={null}
type McpServerConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```typescript theme={null}
type McpStdioServerConfig = {
  type?: "stdio";
  command: string;
  args?: string[];
  env?: Record<string, string>;
};
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```typescript theme={null}
type McpSSEServerConfig = {
  type: "sse";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```typescript theme={null}
type McpHttpServerConfig = {
  type: "http";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcpsdkserverconfigwithinstance">
  `McpSdkServerConfigWithInstance`
</h4>

```typescript theme={null}
type McpSdkServerConfigWithInstance = {
  type: "sdk";
  name: string;
  instance: McpServer;
};
```

<h4 id="mcpclaudeaiproxyserverconfig">
  `McpClaudeAIProxyServerConfig`
</h4>

```typescript theme={null}
type McpClaudeAIProxyServerConfig = {
  type: "claudeai-proxy";
  url: string;
  id: string;
};
```

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Конфигурация для загрузки plugins в SDK.

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| Поле               | Тип       | Описание                                                                                                                                                                                                      |
| :----------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`             | `'local'` | Должно быть `'local'` (в настоящее время поддерживаются только локальные plugins)                                                                                                                             |
| `path`             | `string`  | Абсолютный или относительный путь к директории plugin                                                                                                                                                         |
| `skipMcpDiscovery` | `boolean` | Когда `true`, SDK загружает skills, hooks, agents и commands из этого plugin, но не читает его `.mcp.json` или manifest `mcpServers`. Установите это, когда ваше приложение владеет подключениями MCP plugin. |

**Пример:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Для полной информации о создании и использовании plugins см. [Plugins](/docs/ru/agent-sdk/plugins).

<h2 id="message-types">
  Типы сообщений
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

Тип объединения всех возможных сообщений, возвращаемых запросом.

```typescript theme={null}
type SDKMessage =
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage
  | SDKStatusMessage
  | SDKLocalCommandOutputMessage
  | SDKHookStartedMessage
  | SDKHookProgressMessage
  | SDKHookResponseMessage
  | SDKPluginInstallMessage
  | SDKToolProgressMessage
  | SDKAuthStatusMessage
  | SDKTaskNotificationMessage
  | SDKTaskStartedMessage
  | SDKTaskProgressMessage
  | SDKTaskUpdatedMessage
  | SDKBackgroundTasksChangedMessage
  | SDKThinkingTokensMessage
  | SDKSessionStateChangedMessage
  | SDKWorkerShuttingDownMessage
  | SDKCommandsChangedMessage
  | SDKNotificationMessage
  | SDKFilesPersistedEvent
  | SDKToolUseSummaryMessage
  | SDKMemoryRecallMessage
  | SDKRateLimitEvent
  | SDKElicitationCompleteMessage
  | SDKPermissionDeniedMessage
  | SDKPromptSuggestionMessage
  | SDKAPIRetryMessage
  | SDKMirrorErrorMessage
  | SDKInformationalMessage
  | SDKConversationResetMessage;
```

<h3 id="sdkassistantmessage">
  `SDKAssistantMessage`
</h3>

Сообщение ответа ассистента.

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // Из Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
};
```

Поле `message` это [`BetaMessage`](https://platform.claude.com/docs/ru/api/messages/create) из Anthropic SDK. Оно включает поля, такие как `id`, `content`, `model`, `stop_reason` и `usage`.

`SDKAssistantMessageError` это один из: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'` или `'unknown'`. `'model_not_found'` означает, что выбранная модель не существует или недоступна для вашей учётной записи или развёртывания. `'overloaded'` означает, что API вернул 529, потому что сервер работает на полную мощность, в отличие от `'rate_limit'`, который является 429 в отношении вашей квоты.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

Сообщение пользовательского ввода.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // Из Anthropic SDK
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
};
```

Установите `shouldQuery` на `false` для добавления сообщения в транскрипт без запуска хода ассистента. Сообщение удерживается и объединяется в следующее пользовательское сообщение, которое запускает ход. Используйте это для внедрения контекста, такого как вывод команды, которую вы запустили вне полосы, без траты вызова модели на это.

На сообщении, которое содержит блок `tool_result`, `tool_use_result` это объект структурированного вывода инструмента, а не текст, отправленный модели. Его форма зависит от инструмента, названного соответствующим блоком `tool_use`, поэтому поле типизировано как `unknown`; встроенные формы перечислены в разделе [Типы вывода инструментов](#tool-output-types).

Для инструмента `Agent`, `tool_use_result` это [`AgentOutput`](#agent-2). На результате `completed`, `content` содержит отчёт подагента без ID агента и трейлера использования, которые Claude Code добавляет к тексту `tool_result`, поэтому отображайте из `tool_use_result` вместо анализа этого текста.

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

Повторно воспроизведённое пользовательское сообщение с требуемым UUID.

```typescript theme={null}
type SDKUserMessageReplay = {
  type: "user";
  uuid: UUID;
  session_id: string;
  message: MessageParam;
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  isReplay: true;
};
```

Пользовательский ход, внедрённый извне сеанса, один, чьё [`origin`](#sdkmessageorigin) имеет вид `peer` или `channel`, достигает потока как повтор, был ли он доставлен во время активного хода или запустил новый ход, пока сеанс был неактивен. До v2.1.207 внедрённый ход, доставленный, пока сеанс был неактивен, не производил никакого сообщения в потоке и появлялся только при повторном чтении транскрипта.

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

Финальное сообщение результата.

```typescript theme={null}
type SDKResultMessage =
  | {
      type: "result";
      subtype: "success";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      api_error_status?: number | null;
      num_turns: number;
      result: string;
      stop_reason: string | null;
      ttft_ms?: number;
      ttft_stream_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    }
  | {
      type: "result";
      subtype:
        | "error_max_turns"
        | "error_during_execution"
        | "error_max_budget_usd"
        | "error_max_structured_output_retries";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      stop_reason: string | null;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      errors: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    };
```

Несколько полей в результате содержат диагностические детали помимо `subtype`:

* `api_error_status`: HTTP код состояния ошибки API, которая завершила диалог. Отсутствует или имеет значение `null`, когда ход завершился без ошибки API.
* `ttft_ms`: время до первого токена в миллисекундах, измеренное при поступлении первого полного сообщения ассистента. Присутствует только на успешной ветви.
* `ttft_stream_ms`: время в миллисекундах до первого события потока `message_start`, когда открывается поток ответа. Ниже, чем `ttft_ms`; разница между ними — это время, потраченное на потоковую передачу первого сообщения. Присутствует только на успешной ветви.
* `terminal_reason`: почему цикл завершился. Один из `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"` или `"turn_setup_failed"`.
* `fast_mode_state`: один из `"on"`, `"off"` или `"cooldown"`.

Поле `origin` передаёт [`SDKMessageOrigin`](#sdkmessageorigin) пользовательского сообщения, которое запустило этот результат. Когда фоновая задача завершается и SDK внедряет синтетический ход продолжения, результирующее `SDKResultMessage` содержит `origin: { kind: "task-notification" }`. Проверьте это поле, чтобы различить результаты, которые отвечают на ваш запрос, от результатов, выданных для продолжений фоновых задач, чтобы вы могли маршрутизировать или подавлять последние. Поле отсутствует для результатов, выданных перед любым пользовательским ходом, таких как ошибки при запуске.

Когда hook `PreToolUse` возвращает `permissionDecision: "defer"`, результат имеет `stop_reason: "tool_deferred"` и `deferred_tool_use` содержит `id`, `name` и `input` ожидающего инструмента. Прочитайте это поле, чтобы отобразить запрос в вашем собственном пользовательском интерфейсе, затем возобновите с тем же `session_id` для продолжения. Смотрите [Отложить вызов инструмента на потом](/docs/ru/hooks#defer-a-tool-call-for-later) для полного цикла.

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

Сообщение инициализации системы.

```typescript theme={null}
type SDKSystemMessage = {
  type: "system";
  subtype: "init";
  uuid: UUID;
  session_id: string;
  agents?: string[];
  apiKeySource: ApiKeySource;
  betas?: string[];
  claude_code_version: string;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  capabilities?: string[];
};
```

Массив `capabilities` называет поведения протокола, которые реализует этот CLI, поэтому вы можете обнаруживать функции вместо сравнения строк `claude_code_version`. Это открытый набор: игнорируйте значения, которые вы не распознаёте, и проверяйте конкретную возможность, поведение которой вы используете. Поле требует Claude Code v2.1.205 или позже и отсутствует на более ранних CLI.

| Возможность            | Значение                                                                                                                                                                                       |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1` | [`interrupt()`](#query-object) разрешается с помощью [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse), квитанции, называющей сообщения в очереди, которые выживают при прерывании |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

Потоковое частичное сообщение (только когда `includePartialMessages` равен true). Поле `parent_tool_use_id` всегда имеет значение `null`: события потока выдаются только для основного сеанса. Для атрибуции подагента используйте полные сообщения, которые содержат `parent_tool_use_id`, или включите [`forwardSubagentText`](#options) для получения текста и размышлений подагента в виде полных сообщений.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // Из Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Время до первого токена в мс, присутствует только на событиях message_start
};
```

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

Сообщение, указывающее границу компактирования диалога.

```typescript theme={null}
type SDKCompactBoundaryMessage = {
  type: "system";
  subtype: "compact_boundary";
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: "manual" | "auto";
    pre_tokens: number;
  };
};
```

<h3 id="sdkinformationalmessage">
  `SDKInformationalMessage`
</h3>

Универсальный текстовый баннер, выданный циклом. Содержит строки статуса без ошибок, обратную связь hook, такую как причина блокировки hook `UserPromptSubmit`, и вывод команды. Отобразите `content` как простой текст на заданном `level`.

```typescript theme={null}
type SDKInformationalMessage = {
  type: "system";
  subtype: "informational";
  content: string;
  level: "info" | "notice" | "suggestion" | "warning";
  tool_use_id?: string;
  prevent_continuation?: boolean;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkworkershuttingdownmessage">
  `SDKWorkerShuttingDownMessage`
</h3>

Выданное при корректном завершении работника, чтобы удалённые клиенты могли показать, почему работник исчез, вместо ожидания истечения времени ожидания сердцебиения. `reason` это короткая строка в формате snake\_case, установленная хост-CLI, такая как `"host_exit"` или `"remote_control_disabled"`. Действуйте на основе этого только при потоковой передаче в реальном времени. Возобновленный сеанс воспроизводит прошлые экземпляры этого сообщения, поэтому игнорируйте их в этом случае.

```typescript theme={null}
type SDKWorkerShuttingDownMessage = {
  type: "system";
  subtype: "worker_shutting_down";
  reason: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkplugininstallmessage">
  `SDKPluginInstallMessage`
</h3>

Событие прогресса установки plugin. Выдаётся, когда установлена [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ru/env-vars), поэтому ваше приложение Agent SDK может отслеживать установку marketplace plugin перед первым ходом. Статусы `started` и `completed` заключают в скобки общую установку. Статусы `installed` и `failed` сообщают об отдельных marketplaces и включают `name`.

```typescript theme={null}
type SDKPluginInstallMessage = {
  type: "system";
  subtype: "plugin_install";
  status: "started" | "installed" | "failed" | "completed";
  name?: string;
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpermissiondeniedmessage">
  `SDKPermissionDeniedMessage`
</h3>

Событие потока, выданное, когда система разрешений автоматически отклоняет вызов инструмента без интерактивного запроса. Используйте его для отображения отклонения в вашем пользовательском интерфейсе по мере его возникновения, а не только наблюдая результат инструмента `is_error`, который следует за ним. Интерактивный путь запроса достигает вашего приложения отдельно через callback [`canUseTool`](#canusetool). Отклонения, выданные hook `PreToolUse`, не сообщаются через это событие.

Это событие требует Claude Code v2.1.136 или позже.

```typescript theme={null}
type SDKPermissionDeniedMessage = {
  type: "system";
  subtype: "permission_denied";
  tool_name: string;
  tool_use_id: string;
  agent_id?: string;
  decision_reason_type?: string;
  decision_reason?: string;
  message: string;
  uuid: UUID;
  session_id: string;
};
```

| Поле                   | Тип      | Описание                                                                                                                             |
| ---------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `tool_name`            | `string` | Имя инструмента, который был отклонён                                                                                                |
| `tool_use_id`          | `string` | ID блока `tool_use`, на который отвечает это отклонение                                                                              |
| `agent_id`             | `string` | ID подагента, когда отклонённый вызов возник внутри подагента. Зеркалирует поле на `can_use_tool` для маршрутизации на стороне хоста |
| `decision_reason_type` | `string` | Дискриминатор для компонента, который принял решение, такой как `"rule"`, `"mode"`, `"classifier"` или `"asyncAgent"`                |
| `decision_reason`      | `string` | Понятная человеку причина от компонента, принявшего решение, если доступна                                                           |
| `message`              | `string` | Сообщение об отказе, возвращённое модели в `tool_result`                                                                             |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

Информация об отклонённом использовании tool.

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Происхождение сообщения с ролью пользователя. Это появляется как `origin` на [`SDKUserMessage`](#sdkusermessage) и передаётся на соответствующее [`SDKResultMessage`](#sdkresultmessage), чтобы вы могли определить, что запустило данный ход.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      name?: string;
      senderTaskId?: string;
      body?: string;
    }
  | { kind: "task-notification" }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" };
```

| `kind`              | Значение                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Прямой ввод от конечного пользователя. На пользовательских сообщениях отсутствующий `origin` также означает ввод человека.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `channel`           | Сообщение, поступающее на [канал](/docs/ru/channels). `server` это имя исходного MCP сервера.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `peer`              | Сообщение от другого агента. Для внутрипроцессного [товарища по команде](/docs/ru/agent-teams), отправляющего на `main` через `SendMessage`, `from` это имя товарища по команде и `senderTaskId` это его ID задачи. Для кросс-сеансового пира, такого как другой локальный процесс Claude Code, `from` это адрес отправителя и `senderTaskId` отсутствует. }`name` и `body` требуют Claude Code v2.1.205 или позже. `name` это отображаемое имя отправителя, нормализованное Claude Code: оно удаляет управляющие символы Unicode, формат, суррогаты и разделители строк или абзацев, затем обрезает результат и ограничивает его 64 кодовыми точками с многоточием. `body` это декодированное тело сообщения с удалённой оболочкой пира, побайтово совпадающее с тем, что видит модель. Для сообщения товарища по команде `body` всегда присутствует; для кросс-сеансового пира оно присутствует только когда ход точно представляет собой одну оболочку пира, сформированную Claude Code. Отобразите `name` и `body` вместо повторного анализа текста сообщения. |
| `task-notification` | Синтетический ход, внедрённый после завершения фоновой задачи. Смотрите [`SDKTaskNotificationMessage`](#sdktasknotificationmessage).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `coordinator`       | Сообщение от координатора команды в [команде агентов](/docs/ru/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `auto-continuation` | Синтетический ход, внедрённый, когда сеанс продолжается без свежего пользовательского ввода, такой как результат команды, который запускает последующий запрос.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

<h2 id="hook-types">
  Типы hooks
</h2>

Для полного руководства по использованию hooks с примерами и общими паттернами см. [руководство Hooks](/docs/ru/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Доступные события hooks.

```typescript theme={null}
type HookEvent =
  | "PreToolUse"
  | "PostToolUse"
  | "PostToolUseFailure"
  | "PostToolBatch"
  | "Notification"
  | "UserPromptSubmit"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PermissionRequest"
  | "Setup"
  | "TeammateIdle"
  | "TaskCompleted"
  | "ConfigChange"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "MessageDisplay";
```

<h3 id="hookcallback">
  `HookCallback`
</h3>

Тип функции обратного вызова hook.

```typescript theme={null}
type HookCallback = (
  input: HookInput, // Объединение всех типов входных данных hook
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

<h3 id="hookcallbackmatcher">
  `HookCallbackMatcher`
</h3>

Конфигурация hook с опциональным matcher.

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // Timeout в секундах для всех hooks в этом matcher
}
```

<h3 id="hookinput">
  `HookInput`
</h3>

Тип объединения всех типов входных данных hook.

```typescript theme={null}
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | PostToolBatchHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCompletedHookInput
  | ConfigChangeHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | MessageDisplayHookInput;
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

Базовый интерфейс, который расширяют все типы входных данных hook.

```typescript theme={null}
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  prompt_id?: string;
  permission_mode?: string;
  effort?: { level: string };
  agent_id?: string;
  agent_type?: string;
};
```

Поле `prompt_id` — это UUID, идентифицирующий пользовательский запрос, который в настоящий момент обрабатывается. Он совпадает с [атрибутом `prompt.id` на событиях OpenTelemetry](/docs/ru/monitoring-usage#event-correlation-attributes) и отсутствует до первого ввода пользователя. Требуется Claude Code v2.1.196 или позже.

<h4 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h4>

```typescript theme={null}
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: "PreToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
};
```

<h4 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h4>

```typescript theme={null}
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: "PostToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
  tool_use_id: string;
  duration_ms?: number;
};
```

<h4 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h4>

```typescript theme={null}
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: "PostToolUseFailure";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  error: string;
  is_interrupt?: boolean;
  duration_ms?: number;
};
```

<h4 id="posttoolbatchhookinput">
  `PostToolBatchHookInput`
</h4>

Срабатывает один раз после того, как каждый вызов инструмента в пакете разрешится, перед следующим запросом модели. `tool_response` содержит сериализованное содержимое `tool_result`, которое видит модель; форма отличается от структурированного объекта `Output` в `PostToolUseHookInput`.

```typescript theme={null}
type PostToolBatchHookInput = BaseHookInput & {
  hook_event_name: "PostToolBatch";
  tool_calls: PostToolBatchToolCall[];
};

type PostToolBatchToolCall = {
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  tool_response?: unknown;
};
```

<h4 id="notificationhookinput">
  `NotificationHookInput`
</h4>

```typescript theme={null}
type NotificationHookInput = BaseHookInput & {
  hook_event_name: "Notification";
  message: string;
  title?: string;
  notification_type: string;
};
```

<h4 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h4>

```typescript theme={null}
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: "UserPromptSubmit";
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact";
  agent_type?: string;
  model?: string;
};
```

<h4 id="sessionendhookinput">
  `SessionEndHookInput`
</h4>

```typescript theme={null}
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: "SessionEnd";
  reason: ExitReason; // Строка из массива EXIT_REASONS
};
```

<h4 id="stophookinput">
  `StopHookInput`
</h4>

```typescript theme={null}
type StopHookInput = BaseHookInput & {
  hook_event_name: "Stop";
  stop_hook_active: boolean;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};
```

<h4 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h4>

```typescript theme={null}
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: "SubagentStart";
  agent_id: string;
  agent_type: string;
};
```

<h4 id="subagentstophookinput">
  `SubagentStopHookInput`
</h4>

```typescript theme={null}
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: "SubagentStop";
  stop_hook_active: boolean;
  agent_id: string;
  agent_transcript_path: string;
  agent_type: string;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};

type BackgroundTaskSummary = {
  id: string;
  type: string;
  status: string;
  description: string;
  command?: string;
  agent_type?: string;
  server?: string;
  tool?: string;
  name?: string;
};

type SessionCronSummary = {
  id: string;
  schedule: string;
  recurring: boolean;
  prompt: string;
};
```

<h4 id="precompacthookinput">
  `PreCompactHookInput`
</h4>

```typescript theme={null}
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: "PreCompact";
  trigger: "manual" | "auto";
  custom_instructions: string | null;
};
```

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
};
```

<h4 id="setuphookinput">
  `SetupHookInput`
</h4>

```typescript theme={null}
type SetupHookInput = BaseHookInput & {
  hook_event_name: "Setup";
  trigger: "init" | "maintenance";
};
```

<h4 id="teammateidlehookinput">
  `TeammateIdleHookInput`
</h4>

```typescript theme={null}
type TeammateIdleHookInput = BaseHookInput & {
  hook_event_name: "TeammateIdle";
  teammate_name: string;
  /** @deprecated с версии v2.1.178. Содержит имя команды, полученное из сессии; будет удалено. */
  team_name: string;
};
```

<h4 id="taskcompletedhookinput">
  `TaskCompletedHookInput`
</h4>

```typescript theme={null}
type TaskCompletedHookInput = BaseHookInput & {
  hook_event_name: "TaskCompleted";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated с версии v2.1.178. Содержит имя команды, полученное из сессии; будет удалено. */
  team_name?: string;
};
```

<h4 id="configchangehookinput">
  `ConfigChangeHookInput`
</h4>

```typescript theme={null}
type ConfigChangeHookInput = BaseHookInput & {
  hook_event_name: "ConfigChange";
  source:
    | "user_settings"
    | "project_settings"
    | "local_settings"
    | "policy_settings"
    | "skills";
  file_path?: string;
};
```

<h4 id="worktreecreatehookinput">
  `WorktreeCreateHookInput`
</h4>

```typescript theme={null}
type WorktreeCreateHookInput = BaseHookInput & {
  hook_event_name: "WorktreeCreate";
  name: string;
};
```

<h4 id="worktreeremovehookinput">
  `WorktreeRemoveHookInput`
</h4>

```typescript theme={null}
type WorktreeRemoveHookInput = BaseHookInput & {
  hook_event_name: "WorktreeRemove";
  worktree_path: string;
};
```

<h4 id="messagedisplayhookinput">
  `MessageDisplayHookInput`
</h4>

```typescript theme={null}
type MessageDisplayHookInput = BaseHookInput & {
  hook_event_name: "MessageDisplay";
  turn_id: string;
  message_id: string;
  index: number;
  final: boolean;
  delta: string;
};
```

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Возвращаемое значение hook.

```typescript theme={null}
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

```typescript theme={null}
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
};
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

```typescript theme={null}
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: "approve" | "block";
  systemMessage?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: "PreToolUse";
        permissionDecision?: "allow" | "deny" | "ask" | "defer";
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
        additionalContext?: string;
      }
    | {
        hookEventName: "UserPromptSubmit";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated Используйте `updatedToolOutput`, который работает для всех инструментов. */
        updatedMCPToolOutput?: unknown;
      }
    | {
        hookEventName: "PostToolUseFailure";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolBatch";
        additionalContext?: string;
      }
    | {
        hookEventName: "Notification";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionRequest";
        decision:
          | {
              behavior: "allow";
              updatedInput?: Record<string, unknown>;
              updatedPermissions?: PermissionUpdate[];
            }
          | {
              behavior: "deny";
              message?: string;
              interrupt?: boolean;
            };
      };
};
```

<h2 id="tool-input-types">
  Типы входных данных tool
</h2>

Документация схем входных данных для всех встроенных tools Claude Code. Эти типы экспортируются из `@anthropic-ai/claude-agent-sdk` и могут быть использованы для типобезопасного взаимодействия с tools.

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

Объединение всех типов входных данных tool, экспортируемое из `@anthropic-ai/claude-agent-sdk`.

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | AskUserQuestionInput
  | BashInput
  | TaskOutputInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ReadMcpResourceInput
  | SubscribeMcpResourceInput
  | SubscribePollingInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | UnsubscribeMcpResourceInput
  | UnsubscribePollingInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Имя tool:** `Agent` (ранее `Task`, который всё ещё принимается как псевдоним)

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan";
  isolation?: "worktree";
};
```

Запускает нового агента для автономной обработки сложных многошаговых задач.

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Имя tool:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionInput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
};
```

Задаёт пользователю уточняющие вопросы во время выполнения. См. [Обработка одобрений и пользовательского ввода](/docs/ru/agent-sdk/user-input#handle-clarifying-questions) для деталей использования.

<h3 id="bash">
  Bash
</h3>

**Имя tool:** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // milliseconds, max 600000; higher values are clamped to the max
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

Выполняет bash команды в постоянной сессии shell с опциональным timeout и фоновым выполнением.

<h3 id="monitor">
  Monitor
</h3>

**Имя tool:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
  description: string;
  timeout_ms?: number;
  persistent?: boolean;
};
```

Запускает фоновый источник и доставляет каждое событие к Claude, чтобы он мог реагировать без опроса: `command` запускает скрипт и выдаёт одно событие на строку stdout, а `ws` открывает WebSocket и выдаёт одно событие на текстовый фрейм. Укажите ровно один из `command` или `ws`. Источник `ws` требует Claude Code v2.1.195 или позже.

Установите `persistent: true` для наблюдений на уровне сессии, таких как хвосты логов. Когда Monitor запускает команду, он следует тем же правилам разрешения, что и Bash; наблюдение WebSocket запрашивает одобрение отдельно. См. [справочник tool Monitor](/docs/ru/tools-reference#monitor-tool) для поведения и доступности провайдера.

<h3 id="taskoutput">
  TaskOutput
</h3>

**Имя tool:** `TaskOutput`

```typescript theme={null}
type TaskOutputInput = {
  task_id: string;
  block: boolean;
  timeout: number;
};
```

Получает вывод из выполняющейся или завершённой фоновой задачи.

<h3 id="edit">
  Edit
</h3>

**Имя tool:** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

Выполняет точные замены строк в файлах.

<h3 id="read">
  Read
</h3>

**Имя tool:** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
};
```

Читает файлы из локальной файловой системы, включая текст, изображения, PDF и Jupyter notebooks. Используйте `pages` для диапазонов страниц PDF (например, `"1-5"`).

<h3 id="write">
  Write
</h3>

**Имя tool:** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

Записывает файл в локальную файловую систему, перезаписывая, если он существует.

<h3 id="glob">
  Glob
</h3>

**Имя tool:** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

Быстрое сопоставление паттернов файлов, которое работает с любым размером кодовой базы.

<h3 id="grep">
  Grep
</h3>

**Имя tool:** `Grep`

```typescript theme={null}
type GrepInput = {
  pattern: string;
  path?: string;
  glob?: string;
  type?: string;
  output_mode?: "content" | "files_with_matches" | "count";
  "-i"?: boolean;
  "-n"?: boolean;
  "-B"?: number;
  "-A"?: number;
  "-C"?: number;
  context?: number;
  head_limit?: number;
  offset?: number;
  multiline?: boolean;
};
```

Мощный tool поиска, построенный на ripgrep с поддержкой regex.

<h3 id="taskstop">
  TaskStop
</h3>

**Имя tool:** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // Устарело: используйте task_id
};
```

Останавливает выполняющуюся фоновую задачу или shell по ID. Начиная с v2.1.198, `task_id` также принимает товарища по команде agent-team или именованного фонового агента по ID агента или имени.

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Имя tool:** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

Редактирует ячейки в файлах Jupyter notebook.

<h3 id="webfetch">
  WebFetch
</h3>

**Имя tool:** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
};
```

Получает содержимое с URL и обрабатывает его с помощью модели AI.

<h3 id="websearch">
  WebSearch
</h3>

**Имя tool:** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

Ищет в веб и возвращает отформатированные результаты.

<h3 id="workflow">
  Workflow
</h3>

**Имя tool:** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown;
  resumeFromRunId?: string;
};
```

Запускает [динамический workflow](/docs/ru/workflows): скрипт, который организует множество подагентов в фоне и возвращает один консолидированный результат. Tool `Workflow` доступен в Agent SDK v0.3.149 и позже. Требуется хотя бы один из `script`, `name` или `scriptPath`.

| Поле              | Тип       | Описание                                                                                                                                                                                                                                                                                                               |
| ----------------- | --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Встроенный скрипт workflow. Должен начинаться с `export const meta = { name, description }` как литерал, за которым следует тело скрипта с использованием `agent()`, `parallel()`, `pipeline()` и `phase()`. Опциональный массив `phases` в `meta` группирует агентов под названными этапами в представлении прогресса |
| `name`            | `string`  | Имя встроенного workflow или сохранённого в `.claude/workflows/`. Разрешается в скрипт                                                                                                                                                                                                                                 |
| `scriptPath`      | `string`  | Путь к файлу скрипта workflow на диске. Имеет приоритет над `script` и `name`. Каждый вызов сохраняет свой скрипт и возвращает путь в результате, поэтому вы можете отредактировать этот файл и повторно вызвать с тем же `scriptPath` для итерации                                                                    |
| `args`            | `unknown` | Входное значение, доступное скрипту как глобальная переменная `args`, для параметризованных именованных workflows, таких как исследовательский вопрос или список путей файлов. Передавайте массивы и объекты как фактические значения JSON, а не как JSON-кодированную строку                                          |
| `resumeFromRunId` | `string`  | Run ID предыдущего вызова `Workflow` для возобновления. Завершённые вызовы `agent()` с неизменёнными входными данными возвращают кэшированные результаты; только изменённые или новые вызовы выполняются в реальном времени. Только в одной сессии                                                                     |

<h3 id="todowrite">
  TodoWrite
</h3>

**Имя tool:** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Создаёт и управляет структурированным списком задач для отслеживания прогресса.

<Note>
  Начиная с TypeScript Agent SDK 0.3.142, `TodoWrite` отключён по умолчанию. Используйте вместо этого `TaskCreate`, `TaskGet`, `TaskUpdate` и `TaskList`. См. [Миграция на Task tools](/docs/ru/agent-sdk/todo-tracking#migrate-to-task-tools) для обновления кода мониторинга, или установите `CLAUDE_CODE_ENABLE_TASKS=0` для возврата к `TodoWrite`.
</Note>

<h3 id="taskcreate">
  TaskCreate
</h3>

**Имя tool:** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

Создаёт одну задачу и возвращает её назначенный ID.

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Имя tool:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateInput = {
  taskId: string;
  status?: "pending" | "in_progress" | "completed" | "deleted";
  subject?: string;
  description?: string;
  activeForm?: string;
  addBlocks?: string[];
  addBlockedBy?: string[];
  owner?: string;
  metadata?: Record<string, unknown>;
};
```

Исправляет одну задачу по ID. Установите `status` на `"deleted"` для её удаления.

<h3 id="taskget">
  TaskGet
</h3>

**Имя tool:** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

Возвращает полные детали для одной задачи или `null`, когда ID не найден.

<h3 id="tasklist">
  TaskList
</h3>

**Имя tool:** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

Возвращает снимок всех задач в текущем списке.

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Имя tool:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** Устарело: больше не используется. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
};
```

Выходит из режима планирования. Поле `allowedPrompts` устарело и игнорируется; Claude Code всё ещё принимает его, чтобы существующие вызывающие стороны и транскрипты проходили валидацию. До v2.1.205 он запрашивал разрешения Bash на основе запроса для реализации плана.

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Имя tool:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

Перечисляет доступные MCP ресурсы из подключённых серверов.

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Имя tool:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

Читает определённый MCP ресурс с сервера.

<h3 id="enterworktree">
  EnterWorktree
</h3>

**Имя tool:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

Создаёт и входит во временный git worktree для изолированной работы. Передайте `path` для переключения в существующий worktree вместо создания нового. На первом входе целевой объект должен быть зарегистрированным worktree текущего репозитория или, в многорепозиторном рабочем пространстве, репозитория, вложенного внутри него; из сессии worktree он должен находиться под `.claude/worktrees/` репозитория сессии. `name` и `path` являются взаимоисключающими.

<h2 id="tool-output-types">
  Типы выходных данных Tool
</h2>

Документация схем выходных данных для всех встроенных tools Claude Code. Эти типы экспортируются из `@anthropic-ai/claude-agent-sdk` и представляют фактические данные ответа, возвращаемые каждым tool.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Объединение всех типов выходных данных tool.

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | AskUserQuestionOutput
  | BashOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | MonitorOutput
  | NotebookEditOutput
  | ReadMcpResourceOutput
  | TaskCreateOutput
  | TaskGetOutput
  | TaskListOutput
  | TaskStopOutput
  | TaskUpdateOutput
  | TodoWriteOutput
  | WebFetchOutput
  | WebSearchOutput
  | WorkflowOutput;
```

<h3 id="agent-2">
  Agent
</h3>

**Имя tool:** `Agent` (ранее `Task`, который всё ещё принимается как псевдоним)

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      totalToolUseCount: number;
      totalDurationMs: number;
      totalTokens: number;
      usage: {
        input_tokens: number;
        output_tokens: number;
        cache_creation_input_tokens: number | null;
        cache_read_input_tokens: number | null;
        server_tool_use: {
          web_search_requests: number;
          web_fetch_requests: number;
        } | null;
        service_tier: string | null;
        cache_creation: {
          ephemeral_1h_input_tokens: number;
          ephemeral_5m_input_tokens: number;
        } | null;
        inference_geo?: string | null;
        speed?: string | null;
        iterations?: unknown;
      };
      toolStats?: {
        readCount: number;
        searchCount: number;
        bashCount: number;
        editFileCount: number;
        linesAdded: number;
        linesRemoved: number;
        otherToolCount: number;
        frameCount?: number;
      };
      prompt: string;
      worktreePath?: string;
      worktreeBranch?: string;
    }
  | {
      status: "async_launched";
      isAsync?: true;
      agentId: string;
      description: string;
      resolvedModel?: string;
      prompt: string;
      outputFile: string;
      canReadOutputFile?: boolean;
    }
  | {
      status: "remote_launched";
      taskId: string;
      sessionUrl: string;
      description: string;
      prompt: string;
      outputFile: string;
    };
```

Возвращает результат от подагента. Дискриминирован по полю `status`: `"completed"` для завершённых задач, `"async_launched"` для фоновых задач и `"remote_launched"` для задач, которые Claude Code отправил в удалённый облачный сеанс, где `sessionUrl` ссылается на этот сеанс и `taskId` его идентифицирует.

Поле `resolvedModel` на вариантах `completed` и `async_launched` указывает модель, на которой фактически работал подагент, которая может отличаться от запрошенного входного параметра `model` когда применяется [`availableModels`](/docs/ru/model-config#restrict-model-selection) или другое переопределение. Это поле требует Claude Code v2.1.174 или позже.

На варианте `completed` `worktreePath` устанавливается, когда подагент работал в изолированном git worktree, и `worktreeBranch` называет ветку этого worktree, когда Claude Code её создал. `usage.service_tier` содержит строку уровня обслуживания, которую API сообщила для запросов подагента.

До v2.1.207 опубликованный тип был более узким. Он опускал `worktreePath`, `worktreeBranch`, `citations`, `toolStats.frameCount` и поля использования `inference_geo`, `speed` и `iterations`, и он типизировал `service_tier` как `"standard" | "priority" | "batch"`. Поля, которые тип отмечает как необязательные, могут отсутствовать в результатах, записанных более ранними версиями.

<h3 id="askuserquestion-2">
  AskUserQuestion
</h3>

**Имя tool:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionOutput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers: Record<string, string>;
  response?: string;
};
```

Возвращает заданные вопросы и ответы пользователя. `response` устанавливается, когда пользователь ввёл свободный ответ вместо ответа на структурированные вопросы; когда присутствует, Claude получает "Пользователь ответил: …" вместо списка ответов по вопросам.

<h3 id="bash-2">
  Bash
</h3>

**Имя tool:** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
};
```

Возвращает вывод команды с разделённым stdout/stderr. Фоновые команды включают `backgroundTaskId`.

<h3 id="monitor-2">
  Monitor
</h3>

**Имя tool:** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

Возвращает ID фоновой задачи для выполняющегося монитора. Используйте этот ID с `TaskStop` для раннего отмены наблюдения.

<h3 id="edit-2">
  Edit
</h3>

**Имя tool:** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  userModified: boolean;
  replaceAll: boolean;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
  };
};
```

Возвращает структурированный diff операции редактирования.

<h3 id="read-2">
  Read
</h3>

**Имя tool:** `Read`

```typescript theme={null}
type FileReadOutput =
  | {
      type: "text";
      file: {
        filePath: string;
        content: string;
        numLines: number;
        startLine: number;
        totalLines: number;
      };
    }
  | {
      type: "image";
      file: {
        base64: string;
        type: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        originalSize: number;
        dimensions?: {
          originalWidth?: number;
          originalHeight?: number;
          displayWidth?: number;
          displayHeight?: number;
        };
      };
    }
  | {
      type: "notebook";
      file: {
        filePath: string;
        cells: unknown[];
      };
    }
  | {
      type: "pdf";
      file: {
        filePath: string;
        base64: string;
        originalSize: number;
      };
    }
  | {
      type: "parts";
      file: {
        filePath: string;
        originalSize: number;
        count: number;
        outputDir: string;
      };
    };
```

Возвращает содержимое файла в формате, подходящем для типа файла. Дискриминирован по полю `type`.

<h3 id="write-2">
  Write
</h3>

**Имя tool:** `Write`

```typescript theme={null}
type FileWriteOutput = {
  type: "create" | "update";
  filePath: string;
  content: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  originalFile: string | null;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
  };
};
```

Возвращает результат записи с информацией структурированного diff.

<h3 id="glob-2">
  Glob
</h3>

**Имя tool:** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
};
```

Возвращает пути файлов, соответствующие паттерну glob, отсортированные по времени изменения.

<h3 id="grep-2">
  Grep
</h3>

**Имя tool:** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Возвращает результаты поиска. Форма варьируется по `mode`: список файлов, содержимое с совпадениями или количество совпадений.

<h3 id="taskstop-2">
  TaskStop
</h3>

**Имя tool:** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

Возвращает подтверждение после остановки фоновой задачи.

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**Имя tool:** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  cell_id?: string;
  cell_type: "code" | "markdown";
  language: string;
  edit_mode: string;
  error?: string;
  notebook_path: string;
  original_file: string;
  updated_file: string;
};
```

Возвращает результат редактирования notebook с исходным и обновлённым содержимым файла.

<h3 id="webfetch-2">
  WebFetch
</h3>

**Имя tool:** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
};
```

Возвращает полученное содержимое с HTTP статусом и метаданными.

<h3 id="websearch-2">
  WebSearch
</h3>

**Имя tool:** `WebSearch`

```typescript theme={null}
type WebSearchOutput = {
  query: string;
  results: Array<
    | {
        tool_use_id: string;
        content: Array<{ title: string; url: string }>;
      }
    | string
  >;
  durationSeconds: number;
};
```

Возвращает результаты поиска из веб.

<h3 id="workflow-2">
  Workflow
</h3>

**Имя tool:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched";
  taskId: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  error?: string;
};
```

Возвращает результат сразу после того, как tool принимает вызов. Окончательный результат поступает позже как завершение задачи. Проверьте `error` перед тем, как рассматривать запуск как начатый: скрипт, который не прошёл проверку синтаксиса, возвращает `status: "async_launched"` с установленным `error` и никогда не запускается.

| Поле            | Тип                | Описание                                                                                                                                                              |
| --------------- | ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched"` | Tool принял вызов. Это единственное значение, которое принимает поле                                                                                                  |
| `taskId`        | `string`           | Идентификатор фоновой задачи для запуска                                                                                                                              |
| `runId`         | `string`           | Идентификатор запуска workflow для передачи как `resumeFromRunId` при последующем вызове                                                                              |
| `summary`       | `string`           | Однострочное описание того, что делает workflow                                                                                                                       |
| `transcriptDir` | `string`           | Директория, где записываются транскрипты подагента во время выполнения                                                                                                |
| `scriptPath`    | `string`           | Путь к сохранённому скрипту workflow для этого запуска. Отредактируйте его и передайте обратно как `scriptPath` для повторного запуска без повторной отправки скрипта |
| `error`         | `string`           | Устанавливается, когда скрипт не прошёл проверку синтаксиса. Если присутствует, запуск не начался несмотря на статус `async_launched`                                 |

<h3 id="todowrite-2">
  TodoWrite
</h3>

**Имя tool:** `TodoWrite`

```typescript theme={null}
type TodoWriteOutput = {
  oldTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
  newTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Возвращает предыдущие и обновлённые списки задач.

<Note>
  Начиная с TypeScript Agent SDK 0.3.142, `TodoWrite` отключён по умолчанию. Используйте вместо этого `TaskCreate`, `TaskGet`, `TaskUpdate` и `TaskList`. Смотрите [Миграция на Task tools](/docs/ru/agent-sdk/todo-tracking#migrate-to-task-tools) для обновления кода мониторинга, или установите `CLAUDE_CODE_ENABLE_TASKS=0` для возврата к `TodoWrite`.
</Note>

<h3 id="taskcreate-2">
  TaskCreate
</h3>

**Имя tool:** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

Возвращает созданную задачу с назначенным ей ID.

<h3 id="taskupdate-2">
  TaskUpdate
</h3>

**Имя tool:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateOutput = {
  success: boolean;
  taskId: string;
  updatedFields: string[];
  error?: string;
  statusChange?: {
    from: string;
    to: string;
  };
};
```

Возвращает результат обновления, включая какие поля изменились.

<h3 id="taskget-2">
  TaskGet
</h3>

**Имя tool:** `TaskGet`

```typescript theme={null}
type TaskGetOutput = {
  task: {
    id: string;
    subject: string;
    description: string;
    status: "pending" | "in_progress" | "completed";
    blocks: string[];
    blockedBy: string[];
  } | null;
};
```

Возвращает полную запись задачи или `null` когда ID не найден.

<h3 id="tasklist-2">
  TaskList
</h3>

**Имя tool:** `TaskList`

```typescript theme={null}
type TaskListOutput = {
  tasks: Array<{
    id: string;
    subject: string;
    status: "pending" | "in_progress" | "completed";
    owner?: string;
    blockedBy: string[];
  }>;
};
```

Возвращает снимок всех задач в текущем списке.

<h3 id="exitplanmode-2">
  ExitPlanMode
</h3>

**Имя tool:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

Возвращает состояние плана после выхода из режима планирования.

<h3 id="listmcpresources-2">
  ListMcpResources
</h3>

**Имя tool:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

Возвращает массив доступных MCP ресурсов.

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**Имя tool:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
  }>;
};
```

Возвращает содержимое запрошенного MCP ресурса.

<h3 id="enterworktree-2">
  EnterWorktree
</h3>

**Имя tool:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

Возвращает информацию о git worktree.

<h2 id="permission-types">
  Типы разрешений
</h2>

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Операции для обновления разрешений.

```typescript theme={null}
type PermissionUpdate =
  | {
      type: "addRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "replaceRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "setMode";
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "addDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    };
```

<h3 id="permissionbehavior">
  `PermissionBehavior`
</h3>

```typescript theme={null}
type PermissionBehavior = "allow" | "deny" | "ask";
```

<h3 id="permissionupdatedestination">
  `PermissionUpdateDestination`
</h3>

```typescript theme={null}
type PermissionUpdateDestination =
  | "userSettings" // Глобальные пользовательские настройки
  | "projectSettings" // Настройки проекта для каждой директории
  | "localSettings" // Локальные настройки проекта
  | "session" // Только текущая сессия
  | "cliArg"; // Аргумент CLI
```

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

```typescript theme={null}
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
};
```

<h2 id="other-types">
  Другие типы
</h2>

<h3 id="apikeysource">
  `ApiKeySource`
</h3>

```typescript theme={null}
type ApiKeySource = "user" | "project" | "org" | "temporary" | "oauth";
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Доступные бета-функции, которые можно включить через опцию `betas`. См. [Заголовки Beta](https://platform.claude.com/docs/ru/api/beta-headers) для дополнительной информации.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  Бета `context-1m-2025-08-07` снята с производства по состоянию на 30 апреля 2026 года. Передача этого значения с Claude Sonnet 4.5 или Sonnet 4 не имеет эффекта, и запросы, превышающие стандартное окно контекста 200k-токенов, возвращают ошибку. Для использования окна контекста 1M-токенов перейдите на [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 или Claude Opus 4.8](https://platform.claude.com/docs/ru/about-claude/models/overview), которые включают контекст 1M по стандартной цене без требуемого заголовка beta.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Информация о доступной slash команде.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
};
```

<h3 id="modelinfo">
  `ModelInfo`
</h3>

Информация о доступной модели.

```typescript theme={null}
type ModelInfo = {
  value: string;
  resolvedModel?: string;
  displayName: string;
  description: string;
  supportsEffort?: boolean;
  supportedEffortLevels?: ("low" | "medium" | "high" | "xhigh" | "max")[];
  supportsAdaptiveThinking?: boolean;
  supportsFastMode?: boolean;
  supportsAutoMode?: boolean;
};
```

| Поле                       | Тип                                                                | Описание                                                                                                                                                                                                                                                                                                                                                            |
| :------------------------- | :----------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `value`                    | `string`                                                           | Идентификатор модели для передачи в вызовы API                                                                                                                                                                                                                                                                                                                      |
| `resolvedModel`            | `string \| undefined`                                              | Канонический идентификатор модели провода, на который разрешается `value` этой записи. Запись псевдонима, такая как `sonnet`, разрешается на явный идентификатор модели, такой как `claude-sonnet-5`, поэтому хост может сопоставить сохранённый явный идентификатор модели с записью псевдонима, которая его охватывает. Требуется Claude Code v2.1.197 или позже. |
| `displayName`              | `string`                                                           | Удобочитаемое отображаемое имя                                                                                                                                                                                                                                                                                                                                      |
| `description`              | `string`                                                           | Описание возможностей модели                                                                                                                                                                                                                                                                                                                                        |
| `supportsEffort`           | `boolean \| undefined`                                             | Поддерживает ли эта модель уровни усилий                                                                                                                                                                                                                                                                                                                            |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | Уровни усилий, которые принимает эта модель                                                                                                                                                                                                                                                                                                                         |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | Поддерживает ли эта модель адаптивное мышление, где Claude решает, когда и сколько думать                                                                                                                                                                                                                                                                           |
| `supportsFastMode`         | `boolean \| undefined`                                             | Поддерживает ли эта модель быстрый режим                                                                                                                                                                                                                                                                                                                            |
| `supportsAutoMode`         | `boolean \| undefined`                                             | Поддерживает ли эта модель автоматический режим                                                                                                                                                                                                                                                                                                                     |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

Информация о доступном подагенте, который может быть вызван через tool Agent.

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| Поле          | Тип                   | Описание                                                                                 |
| :------------ | :-------------------- | :--------------------------------------------------------------------------------------- |
| `name`        | `string`              | Идентификатор типа агента (например, `"Explore"`, `"general-purpose"`)                   |
| `description` | `string`              | Описание, когда использовать этого агента                                                |
| `model`       | `string \| undefined` | Псевдоним модели, который использует этот агент. Если опущено, наследует модель родителя |

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Статус подключённого MCP сервера.

```typescript theme={null}
type McpServerStatus = {
  name: string;
  status: "connected" | "failed" | "needs-auth" | "pending" | "disabled";
  serverInfo?: {
    name: string;
    version: string;
  };
  error?: string;
  config?: McpServerStatusConfig;
  scope?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
  }[];
};
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

Конфигурация MCP сервера, как сообщается `mcpServerStatus()`. Это объединение всех типов транспорта MCP сервера.

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

См. [`McpServerConfig`](#mcpserverconfig) для деталей по каждому типу транспорта.

<h3 id="accountinfo">
  `AccountInfo`
</h3>

Информация об учётной записи для аутентифицированного пользователя.

```typescript theme={null}
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
};
```

<h3 id="modelusage">
  `ModelUsage`
</h3>

Статистика использования для каждой модели, возвращаемая в сообщениях результата. Значение `costUSD` это оценка на стороне клиента. См. [Отслеживание стоимости и использования](/docs/ru/agent-sdk/cost-tracking) для предостережений выставления счётов.

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
};
```

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

Версия [`Usage`](#usage) со всеми nullable полями, сделанными non-nullable.

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

Статистика использования токенов. Это тип `BetaUsage` из `@anthropic-ai/sdk`.

```typescript theme={null}
type Usage = {
  input_tokens: number;
  output_tokens: number;
  cache_creation_input_tokens: number | null;
  cache_read_input_tokens: number | null;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  } | null;
  server_tool_use: BetaServerToolUsage | null;
  service_tier: "standard" | "priority" | "batch" | null;
  speed: "standard" | "fast" | null;
  inference_geo: string | null;
  iterations: BetaIterationsUsage | null;
};
```

`BetaServerToolUsage` и `BetaIterationsUsage` определены в `@anthropic-ai/sdk`.

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

Тип результата MCP tool (из `@modelcontextprotocol/sdk/types.js`). `structuredContent` это объект JSON, который может быть возвращён вместе с `content`, включая блоки изображений. См. [Возврат структурированных данных](/docs/ru/agent-sdk/custom-tools#return-structured-data).

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // Дополнительные поля варьируются по типу
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Контролирует поведение мышления/рассуждения Claude. Имеет приоритет над устаревшим `maxThinkingTokens`.

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // Модель определяет, когда и сколько рассуждать (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // Фиксированный бюджет токенов мышления
  | { type: "disabled" }; // Без расширенного мышления
```

Опциональное поле `display` контролирует, возвращается ли текст мышления `"summarized"` или `"omitted"`. На Claude Opus 4.7 и позже, значение по умолчанию API это `"omitted"`, поэтому установите `"summarized"` для получения содержимого мышления в блоках `thinking`.

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

Интерфейс для пользовательского запуска процесса (используется с опцией `spawnClaudeCodeProcess`). `ChildProcess` уже удовлетворяет этому интерфейсу.

```typescript theme={null}
interface SpawnedProcess {
  stdin: Writable;
  stdout: Readable;
  readonly killed: boolean;
  readonly exitCode: number | null;
  kill(signal: NodeJS.Signals): boolean;
  on(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  on(event: "error", listener: (error: Error) => void): void;
  once(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  once(event: "error", listener: (error: Error) => void): void;
  off(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  off(event: "error", listener: (error: Error) => void): void;
}
```

<h3 id="spawnoptions">
  `SpawnOptions`
</h3>

Опции, передаваемые пользовательской функции spawn.

```typescript theme={null}
interface SpawnOptions {
  command: string;
  args: string[];
  cwd?: string;
  env: Record<string, string | undefined>;
  signal: AbortSignal;
}
```

<Note>
  Поле `signal` сообщает вашей функции spawn, когда нужно разобрать процесс. Передайте его как опцию `signal` в `spawn()` Node, или передайте его обработчику разборки вашей VM или контейнера.

  Этот сигнал не срабатывает в момент отмены [`Options.abortController`](#options). SDK сначала закрывает stdin процесса и ждёт около двух секунд, чтобы CLI мог корректно завершить работу, затем отменяет этот сигнал. Чтобы реагировать в момент отмены вызывающей стороной, слушайте на вашем собственном `Options.abortController.signal`, на который может ссылаться ваша функция spawn из её охватывающей области.
</Note>

<h3 id="mcpsetserversresult">
  `McpSetServersResult`
</h3>

Результат операции `setMcpServers()`.

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

<h3 id="rewindfilesresult">
  `RewindFilesResult`
</h3>

Результат операции `rewindFiles()`.

```typescript theme={null}
type RewindFilesResult = {
  canRewind: boolean;
  error?: string;
  filesChanged?: string[];
  insertions?: number;
  deletions?: number;
};
```

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

Сообщение обновления статуса (например, компактирование).

```typescript theme={null}
type SDKStatusMessage = {
  type: "system";
  subtype: "status";
  status: "compacting" | null;
  permissionMode?: PermissionMode;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktasknotificationmessage">
  `SDKTaskNotificationMessage`
</h3>

Уведомление, когда фоновая задача завершается, не работает или остановлена. Фоновые задачи включают команды Bash `run_in_background`, наблюдения [Monitor](#monitor) и фоновые подагентов.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolusesummarymessage">
  `SDKToolUseSummaryMessage`
</h3>

Резюме использования tool в диалоге.

```typescript theme={null}
type SDKToolUseSummaryMessage = {
  type: "tool_use_summary";
  summary: string;
  preceding_tool_use_ids: string[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookstartedmessage">
  `SDKHookStartedMessage`
</h3>

Выдаётся, когда hook начинает выполняться.

Claude Code доставляет это сообщение, [`SDKHookProgressMessage`](#sdkhookprogressmessage) и [`SDKHookResponseMessage`](#sdkhookresponsemessage) в поток сообщений немедленно, включая во время выполнения hook `SessionStart` или `Setup` во время запуска сессии. Claude Code v2.1.169 через v2.1.203 доставляли эти сообщения в одном пакете после завершения hook `SessionStart` или `Setup`; v2.1.204 восстановил живую доставку.

```typescript theme={null}
type SDKHookStartedMessage = {
  type: "system";
  subtype: "hook_started";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookprogressmessage">
  `SDKHookProgressMessage`
</h3>

Выдаётся во время выполнения hook с выводом stdout/stderr.

```typescript theme={null}
type SDKHookProgressMessage = {
  type: "system";
  subtype: "hook_progress";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  stdout: string;
  stderr: string;
  output: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookresponsemessage">
  `SDKHookResponseMessage`
</h3>

Выдаётся, когда hook завершает выполнение.

```typescript theme={null}
type SDKHookResponseMessage = {
  type: "system";
  subtype: "hook_response";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  output: string;
  stdout: string;
  stderr: string;
  exit_code?: number;
  outcome: "success" | "error" | "cancelled";
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolprogressmessage">
  `SDKToolProgressMessage`
</h3>

Выдаётся периодически во время выполнения tool для указания прогресса.

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "tool_progress";
  tool_use_id: string;
  tool_name: string;
  parent_tool_use_id: string | null;
  elapsed_time_seconds: number;
  task_id?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

Выдаётся во время потоков аутентификации.

```typescript theme={null}
type SDKAuthStatusMessage = {
  type: "auth_status";
  isAuthenticating: boolean;
  output: string[];
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskstartedmessage">
  `SDKTaskStartedMessage`
</h3>

Выдаётся, когда фоновая задача начинается. Поле `task_type` это `"local_bash"` для фоновых команд Bash и наблюдений [Monitor](#monitor), `"local_agent"` для подагентов или `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

Выдаётся периодически во время выполнения подагента или фоновой задачи. Поле `summary` заполняется только когда включён [`agentProgressSummaries`](#options).

```typescript theme={null}
type SDKTaskProgressMessage = {
  type: "system";
  subtype: "task_progress";
  task_id: string;
  tool_use_id?: string;
  description: string;
  subagent_type?: string;
  usage: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  last_tool_name?: string;
  summary?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskupdatedmessage">
  `SDKTaskUpdatedMessage`
</h3>

Выдаётся, когда состояние фоновой задачи изменяется, например, когда она переходит из `running` в `completed`. Объедините `patch` в вашу локальную карту задач, индексированную по `task_id`. Поле `end_time` это временная метка Unix epoch в миллисекундах, сравнимая с `Date.now()`.

```typescript theme={null}
type SDKTaskUpdatedMessage = {
  type: "system";
  subtype: "task_updated";
  task_id: string;
  patch: {
    status?: "pending" | "running" | "completed" | "failed" | "killed";
    description?: string;
    end_time?: number;
    total_paused_ms?: number;
    error?: string;
    is_backgrounded?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkbackgroundtaskschangedmessage">
  `SDKBackgroundTasksChangedMessage`
</h3>

Выдаётся всякий раз, когда набор активных фоновых задач изменяется: задача начинается, завершается, убивается или переднеплановый агент переводится в фоновый режим. Массив `tasks` это полный активный набор. Замените любой кэшированный набор каждым payload вместо сопряжения событий `task_started` и `task_notification`, поэтому следующее изменение членства исправит любое событие, которое вы пропустили.

Порядок относительно этих событий для каждой задачи не определён, поэтому не коррелируйте два потока.

Ничего не выдаётся при запуске. Сбросьте на пустой набор всякий раз, когда процесс CLI сессии запускается или перезапускается, и позвольте следующему изменению членства переполнить его.

Требуется Claude Code v2.1.203 или позже.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Выдаётся во время создания Claude блока мышления, включая отредактированный, с текущей оценкой токенов мышления, созданных до сих пор. `estimated_tokens` это текущий итог для текущего блока мышления и `estimated_tokens_delta` это приращение, переносимое этим кадром. Используйте его для отображения прогресса. Окончательный подсчёт для цикла агента верхнего уровня это `usage.output_tokens` сообщения результата, который [не включает токены подагентов](/docs/ru/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); используйте [`modelUsage`](#modelusage) для учёта всего дерева.

Требуется Claude Code v2.1.153 или позже.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkfilespersistedevent">
  `SDKFilesPersistedEvent`
</h3>

Выдаётся, когда контрольные точки файлов сохраняются на диск.

```typescript theme={null}
type SDKFilesPersistedEvent = {
  type: "system";
  subtype: "files_persisted";
  files: { filename: string; file_id: string }[];
  failed: { filename: string; error: string }[];
  processed_at: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkratelimitevent">
  `SDKRateLimitEvent`
</h3>

Выдаётся, когда сессия встречает ограничение скорости.

```typescript theme={null}
type SDKRateLimitEvent = {
  type: "rate_limit_event";
  rate_limit_info: {
    status: "allowed" | "allowed_warning" | "rejected";
    resetsAt?: number;
    utilization?: number;
    errorCode?: "credits_required";
    canUserPurchaseCredits?: boolean;
    hasChargeableSavedPaymentMethod?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

Когда `errorCode` это `"credits_required"`, отклонение происходит от подписки claude.ai, чьё включённое использование исчерпано, и сессия не может продолжаться, пока пользователь не купит кредиты использования. `canUserPurchaseCredits` указывает, может ли аутентифицированный пользователь купить кредиты для учётной записи, и `hasChargeableSavedPaymentMethod` указывает, есть ли сохранённый способ оплаты в файле. Все три поля отсутствуют на событиях ограничения скорости, которые не являются отклонениями, требующими кредитов. Требуется Claude Code v2.1.181 или позже.

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Вывод из локальной slash команды (например, `/voice` или `/usage`). Отображается как текст в стиле ассистента в транскрипте.

```typescript theme={null}
type SDKLocalCommandOutputMessage = {
  type: "system";
  subtype: "local_command_output";
  content: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkcommandschangedmessage">
  `SDKCommandsChangedMessage`
</h3>

Выдаётся, когда набор доступных команд изменяется во время сессии, например, когда skills обнаруживаются при входе агента в подпапку. Массив `commands` это полный обновлённый список, поэтому замените любой кэшированный список команд этим payload. Повторный вызов `supportedCommands()` не эквивалентен: этот метод возвращает снимок, захваченный при инициализации, и не отражает изменения во время сессии.

```typescript theme={null}
type SDKCommandsChangedMessage = {
  type: "system";
  subtype: "commands_changed";
  commands: SlashCommand[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpromptsuggestionmessage">
  `SDKPromptSuggestionMessage`
</h3>

Выдаётся после каждого хода, когда `promptSuggestions` включён. Содержит предсказанный следующий пользовательский запрос.

```typescript theme={null}
type SDKPromptSuggestionMessage = {
  type: "prompt_suggestion";
  suggestion: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkconversationresetmessage">
  `SDKConversationResetMessage`
</h3>

Выдаётся, когда диалог сессии заменяется без завершения сессии, например, после `/clear`, при выходе из режима плана или когда начинается новый диалог. Смонтируйте пустой транскрипт под `new_conversation_id` и отбросьте любой кэшированный заголовок сессии.

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

Опубликованные типизации SDK объявляют `SDKConversationResetMessage` в Claude Code v2.1.203 и позже. До v2.1.203, `SDKMessage` ссылалась на тип без его объявления, поэтому сужение на `type === "conversation_reset"` не прошло проверку типов, когда `skipLibCheck` был отключён.

<h3 id="aborterror">
  `AbortError`
</h3>

Пользовательский класс ошибки для операций отмены.

```typescript theme={null}
class AbortError extends Error {}
```

<h2 id="sandbox-configuration">
  Конфигурация Sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Конфигурация для поведения sandbox. Используйте это для включения sandboxing команд и программной конфигурации ограничений сети.

```typescript theme={null}
type SandboxSettings = {
  enabled?: boolean;
  failIfUnavailable?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: SandboxNetworkConfig;
  filesystem?: SandboxFilesystemConfig;
  ignoreViolations?: Record<string, string[]>;
  enableWeakerNestedSandbox?: boolean;
  ripgrep?: { command: string; args?: string[] };
};
```

| Свойство                    | Тип                                                   | По умолчанию | Описание                                                                                                                                                                                                                                  |
| :-------------------------- | :---------------------------------------------------- | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`      | Включите режим sandbox для выполнения команд                                                                                                                                                                                              |
| `failIfUnavailable`         | `boolean`                                             | `true`       | Остановитесь при запуске, если `enabled` имеет значение `true`, но sandbox не может запуститься. Установите `false` для возврата к выполнению без sandbox с предупреждением на stderr                                                     |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`       | Автоматически одобряйте bash команды, когда sandbox включён                                                                                                                                                                               |
| `excludedCommands`          | `string[]`                                            | `[]`         | Команды, которые всегда обходят ограничения sandbox (например, `['docker']`). Они работают без sandbox автоматически без участия модели                                                                                                   |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`       | Разрешите модели запрашивать выполнение команд вне sandbox. Когда `true`, модель может установить `dangerouslyDisableSandbox` в входных данных tool, что переходит к [системе разрешений](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined`  | Конфигурация sandbox, специфичная для сети                                                                                                                                                                                                |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined`  | Конфигурация sandbox, специфичная для файловой системы, для ограничений чтения/записи                                                                                                                                                     |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined`  | Карта категорий нарушений на паттерны для игнорирования (например, `{ file: ['/tmp/*'], network: ['localhost'] }`)                                                                                                                        |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`      | Включите более слабый вложенный sandbox для совместимости                                                                                                                                                                                 |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined`  | Конфигурация пользовательского бинарного файла ripgrep для окружений sandbox                                                                                                                                                              |

<Note>
  Sandbox зависит от поддержки платформы и, на Linux, инструментов, таких как `bubblewrap` и `socat`. Когда `enabled` имеет значение `true` и sandbox не может запуститься, `query()` сообщает сообщение `result` с `subtype: "error_during_execution"` и причину в `errors`. Для одного вызова сообщения `query()` SDK выбрасывает после выдачи этого результата ошибки, поэтому оберните цикл в блок try для продолжения после него. Смотрите [Handle the result](/docs/ru/agent-sdk/agent-loop#handle-the-result) для контракта ошибки.

  Для выполнения без sandbox вместо этого установите `failIfUnavailable: false`.
</Note>

<h4 id="example-usage">
  Пример использования
</h4>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Build and test my project",
    options: {
      sandbox: {
        enabled: true,
        autoAllowBashIfSandboxed: true,
        network: {
          allowLocalBinding: true
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // such as when the sandbox can't start (failIfUnavailable defaults to true).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Безопасность Unix socket:** Опция `allowUnixSockets` может предоставить доступ к мощным системным сервисам. Например, разрешение `/var/run/docker.sock` фактически предоставляет полный доступ к хост-системе через Docker API, обходя изоляцию sandbox. Разрешайте только Unix sockets, которые строго необходимы, и поймите последствия безопасности каждого.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Конфигурация, специфичная для сети, для режима sandbox. Эти параметры применяются к sandboxed Bash командам, когда `enabled` имеет значение `true` в родительском [`SandboxSettings`](#sandboxsettings). Они не ограничивают инструмент WebFetch, который использует [правила разрешений](/docs/ru/permissions#webfetch) вместо этого.

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Свойство                  | Тип        | По умолчанию | Описание                                                                                                                                                                                                                                                                                                          |
| :------------------------ | :--------- | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`         | Имена доменов, к которым процессы в sandbox могут получить доступ                                                                                                                                                                                                                                                 |
| `deniedDomains`           | `string[]` | `[]`         | Имена доменов, к которым процессы в sandbox не могут получить доступ. Имеет приоритет над `allowedDomains`                                                                                                                                                                                                        |
| `allowManagedDomainsOnly` | `boolean`  | `false`      | Только управляемые параметры. Когда установлено в [управляемых параметрах](/docs/ru/permissions#managed-settings), только записи `allowedDomains` из управляемых параметров учитываются, а записи из пользовательских, проектных или локальных параметров игнорируются. Не имеет эффекта при установке через опции SDK |
| `allowLocalBinding`       | `boolean`  | `false`      | Разрешите процессам привязываться к локальным портам (например, для dev серверов)                                                                                                                                                                                                                                 |
| `allowUnixSockets`        | `string[]` | `[]`         | Пути Unix socket, к которым процессы могут получить доступ (например, Docker socket)                                                                                                                                                                                                                              |
| `allowAllUnixSockets`     | `boolean`  | `false`      | Разрешите доступ ко всем Unix sockets                                                                                                                                                                                                                                                                             |
| `httpProxyPort`           | `number`   | `undefined`  | Порт HTTP прокси для сетевых запросов                                                                                                                                                                                                                                                                             |
| `socksProxyPort`          | `number`   | `undefined`  | Порт SOCKS прокси для сетевых запросов                                                                                                                                                                                                                                                                            |

<Note>
  Встроенный прокси sandbox применяет `allowedDomains` на основе запрашиваемого имени хоста и не завершает и не проверяет трафик TLS, поэтому такие методы, как [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting), потенциально могут его обойти. Смотрите [Ограничения безопасности Sandboxing](/docs/ru/sandboxing#security-limitations) для деталей и [Безопасное развёртывание](/docs/ru/agent-sdk/secure-deployment#traffic-forwarding) для конфигурации прокси, завершающего TLS.
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

Конфигурация, специфичная для файловой системы, для режима sandbox.

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| Свойство     | Тип        | По умолчанию | Описание                                               |
| :----------- | :--------- | :----------- | :----------------------------------------------------- |
| `allowWrite` | `string[]` | `[]`         | Паттерны путей файлов для разрешения доступа на запись |
| `denyWrite`  | `string[]` | `[]`         | Паттерны путей файлов для запрещения доступа на запись |
| `denyRead`   | `string[]` | `[]`         | Паттерны путей файлов для запрещения доступа на чтение |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback разрешений для команд вне Sandbox
</h3>

Когда `allowUnsandboxedCommands` включён, модель может запросить выполнение команд вне sandbox, установив `dangerouslyDisableSandbox: true` во входных данных tool. Эти запросы переходят к существующей системе разрешений, что означает, что ваш обработчик `canUseTool` вызывается, позволяя вам реализовать пользовательскую логику авторизации. В примере ниже `isCommandAuthorized` служит заместителем для проверки авторизации, которую вы определяете.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Статический список команд, которые всегда автоматически обходят sandbox (например, `['docker']`). Модель не имеет контроля над этим.
  * `allowUnsandboxedCommands`: Позволяет модели решать во время выполнения, запрашивать ли выполнение вне sandbox, установив `dangerouslyDisableSandbox: true` во входных данных tool.
</Note>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // Модель может запросить выполнение вне sandbox
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Проверьте, запрашивает ли модель обход sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // Модель запрашивает выполнение этой команды вне sandbox
        console.log(`Unsandboxed command requested: ${input.command}`);

        if (isCommandAuthorized(input.command)) {
          return { behavior: "allow" as const, updatedInput: input };
        }
        return {
          behavior: "deny" as const,
          message: "Command not authorized for unsandboxed execution"
        };
      }
      return { behavior: "allow" as const, updatedInput: input };
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

Этот паттерн позволяет вам:

* **Аудит запросов модели:** Логируйте, когда модель запрашивает выполнение вне sandbox
* **Реализуйте allowlists:** Разрешайте только определённые команды работать вне sandbox
* **Добавьте рабочие процессы одобрения:** Требуйте явной авторизации для привилегированных операций

<Warning>
  Команды, работающие с `dangerouslyDisableSandbox: true`, имеют полный доступ к системе. Убедитесь, что ваш обработчик `canUseTool` тщательно проверяет эти запросы.

  Если `permissionMode` установлен на `bypassPermissions` и `allowUnsandboxedCommands` включён, модель может автономно выполнять команды вне sandbox без каких-либо запросов одобрения (явное [`ask` правило](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) всё ещё заставляет один). Эта комбинация фактически позволяет модели молча выходить из изоляции sandbox.
</Warning>

<h2 id="see-also">
  См. также
</h2>

* [Обзор SDK](/docs/ru/agent-sdk/overview) - Общие концепции SDK
* [Справочник Python SDK](/docs/ru/agent-sdk/python) - Документация Python SDK
* [Справочник CLI](/docs/ru/cli-reference) - Интерфейс командной строки
* [Общие рабочие процессы](/docs/ru/common-workflows) - Пошаговые руководства
