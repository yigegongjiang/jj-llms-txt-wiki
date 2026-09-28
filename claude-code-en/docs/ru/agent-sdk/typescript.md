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
  SDK поставляется с нативным бинарным файлом Claude Code для вашей платформы в качестве опциональной зависимости, такой как `@anthropic-ai/claude-agent-sdk-darwin-arm64`. Большинство установок не требуют отдельной установки Claude Code. Версия SDK отслеживает версию упакованного Claude Code. SDK v0.3.191 поставляется с Claude Code v2.1.191, поэтому функция на этой странице, которая требует определённую версию Claude Code, нуждается в выпуске SDK с тем же номером патча или позже. Если ваш менеджер пакетов пропускает опциональные зависимости, SDK выбросит ошибку `Native CLI binary for <platform>-<arch> not found`; установите [`pathToClaudeCodeExecutable`](#options) на отдельно установленный бинарный файл `claude` вместо этого.

  Если ваш менеджер пакетов не применяет поле `libc` npm, как это делает Yarn 1.x, вы получите оба пакета платформы glibc и musl на Linux, примерно удвоив размер установки. На Agent SDK v0.2.141 или позже SDK всё ещё запускает правильный вариант. Чтобы освободить место в образе контейнера, удалите пакет платформы, который не соответствует libc, где работает ваше приложение; для среды выполнения glibc на x64 это `rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl`. На машине разработки удаление временное, так как Yarn переустанавливает пакет при следующем изменении зависимостей.
</Note>

<h3 id="compile-to-a-single-executable">
  Компиляция в единый исполняемый файл
</h3>

Когда вы компилируете приложение в единый исполняемый файл с помощью `bun build --compile`, SDK не может разрешить упакованный бинарный файл CLI во время выполнения. `require.resolve` не работает внутри виртуальной файловой системы `$bunfs` скомпилированного исполняемого файла, поэтому SDK выбросит ошибку `Native CLI binary for <platform>-<arch> not found`.

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
  extras?: { annotations?: ToolAnnotations; searchHint?: string; alwaysLoad?: boolean }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Параметры
</h4>

| Параметр      | Тип                                                                                                    | Описание                                                                                                                                                                                                                                                                                                                              |
| :------------ | :----------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | `string`                                                                                               | Имя tool                                                                                                                                                                                                                                                                                                                              |
| `description` | `string`                                                                                               | Описание того, что делает tool                                                                                                                                                                                                                                                                                                        |
| `inputSchema` | `Schema extends AnyZodRawShape`                                                                        | Zod схема, определяющая входные параметры tool (поддерживает Zod 3 и Zod 4)                                                                                                                                                                                                                                                           |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>`                                      | Асинхронная функция, которая выполняет логику tool                                                                                                                                                                                                                                                                                    |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | Опциональные аннотации. `annotations` предоставляет поведенческие подсказки MCP клиентам. `searchHint` — это однострочная фраза возможностей, показываемая в списке отложенных tool, когда активен [поиск tool](/docs/ru/agent-sdk/tool-search). `alwaysLoad: true` сохраняет полную схему этого tool в начальном запросе вместо отложения |

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
  instructions?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
  alwaysLoad?: boolean;
  timeout?: number;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Параметры
</h4>

| Параметр               | Тип                           | Описание                                                                                                                                                                                                                                                                    |
| :--------------------- | :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.name`         | `string`                      | Имя MCP сервера                                                                                                                                                                                                                                                             |
| `options.version`      | `string`                      | Опциональная строка версии                                                                                                                                                                                                                                                  |
| `options.instructions` | `string`                      | Опциональные инструкции сервера, возвращаемые из `initialize` и предоставляемые модели как блок инструкций MCP                                                                                                                                                              |
| `options.tools`        | `Array<SdkMcpToolDefinition>` | Массив определений tool, созданных с помощью [`tool()`](#tool)                                                                                                                                                                                                              |
| `options.alwaysLoad`   | `boolean`                     | Когда `true`, каждый tool с этого сервера остаётся в начальном запросе и никогда не откладывается за [поиском tool](/docs/ru/agent-sdk/tool-search). Объединяется с `alwaysLoad` для каждого tool в [`tool()`](#tool)                                                            |
| `options.timeout`      | `number`                      | Timeout в миллисекундах для вызовов tool этого сервера. Claude Code применяет его к этому серверу вместо [`MCP_TOOL_TIMEOUT`](/docs/ru/env-vars). Передайте целое число не менее 1000. Claude Code игнорирует другие значения. Требуется TypeScript Agent SDK v0.3.248 или позже |

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

| Свойство             | Тип                     | Описание                                                                                                                                                                                                                                                                                  |
| :------------------- | :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Роль сообщения                                                                                                                                                                                                                                                                            |
| `uuid`               | `string`                | Уникальный идентификатор сообщения                                                                                                                                                                                                                                                        |
| `session_id`         | `string`                | Сессия, к которой принадлежит это сообщение                                                                                                                                                                                                                                               |
| `message`            | `unknown`               | Необработанная полезная нагрузка сообщения из транскрипта                                                                                                                                                                                                                                 |
| `parent_tool_use_id` | `string \| null`        | Для сообщений подагента, `tool_use_id` вызова tool `Agent` или `Skill`, который его запустил. `null` для сообщений основной сессии и более старых сессий                                                                                                                                  |
| `parent_agent_id`    | `string \| null`        | Для сообщений от [вложенного подагента](/docs/ru/sub-agents#let-subagents-spawn-their-own-subagents), `agentId` подагента, который его запустил. `null` для сообщений основной сессии, сообщений от подагентов верхнего уровня и более старых сессий. Требуется Claude Code v2.1.202 или позже |

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
  Эта функция находится в альфа-версии и её API может измениться перед стабилизацией.
</Note>

Снимок отличается от того, что применяет живая сессия `query()`:

* **`policyHelper`**: `resolveSettings()` читает источники MDM, включая macOS plist и Windows HKLM/HKCU, но не выполняет настроенный администратором подпроцесс `policyHelper`.
* **Параметры, управляемые сервером**: `resolveSettings()` не загружает [параметры, управляемые сервером](/docs/ru/managed-settings#delivery-mechanisms). Передайте их как `options.serverManagedSettings` для включения.
* **`defaultMode`**: снимок возвращает `permissions.defaultMode` как есть из каждого уровня, поэтому он может включать значения `'auto'` и `'bypassPermissions'` из параметров проекта и локальных параметров, которые [живая сессия игнорирует](/docs/ru/permission-modes#which-mode-a-session-starts-in).

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Параметры
</h4>

`resolveSettings()` принимает один объект параметров. Все поля опциональны.

| Параметр                        | Тип                                   | По умолчанию    | Описание                                                                                                                                                                                                                                                                                                                                                            |
| :------------------------------ | :------------------------------------ | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options.cwd`                   | `string`                              | `process.cwd()` | Директория для разрешения параметров проекта и локальных параметров относительно                                                                                                                                                                                                                                                                                    |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Все источники   | Какие источники файловой системы загружать. Передайте `[]` для пропуска пользовательских, проектных и локальных параметров. [Политика, управляемая конечной точкой](/docs/ru/managed-settings#delivery-mechanisms), загружается во всех случаях. `resolveSettings()` включает параметры, управляемые сервером, только когда вы передаёте `options.serverManagedSettings` |
| `options.managedSettings`       | `Settings`                            | `undefined`     | Параметры уровня политики, предоставленные хостом встраивания. Следует тем же правилам, что и [`managedSettings` в `Options`](#options), за исключением того, что `resolveSettings()` не выполняет настроенный [`policyHelper`](/docs/ru/settings-reference#policyhelper), поэтому снимок может включать параметры, которые живая сессия отбрасывает                     |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | Полезная нагрузка параметров, управляемых сервером, из `/api/claude_code/settings`. Неограничивающие ключи проходят без фильтрации                                                                                                                                                                                                                                  |

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

Пример ниже разрешает параметры для директории проекта и выводит источник, который контролирует период очистки. На машине, где ни один файл параметров не устанавливает `cleanupPeriodDays`, обе выведенные строки показывают `undefined` для значения, что является ожидаемым результатом, а не ошибкой.

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

| Свойство                          | Тип                                                                                                                                                                                                            | По умолчанию                                                        | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                                                                                                                              | `new AbortController()`                                             | Контроллер для отмены операций                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `additionalDirectories`           | `string[]`                                                                                                                                                                                                     | `[]`                                                                | Дополнительные каталоги, к которым Claude может получить доступ. SDK передает каждую запись в Claude Code как `--add-dir`, поэтому с параметром `project` source Claude Code также [загружает навыки, команды и подагентов каталога](/docs/ru/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `agent`                           | `string`                                                                                                                                                                                                       | `undefined`                                                         | Имя агента для основного потока. Агент должен быть определен в параметре `agents` или в параметрах                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                                                                                                                        | `undefined`                                                         | Программно определите подагентов                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `agentProgressSummaries`          | `boolean`                                                                                                                                                                                                      | `false`                                                             | Когда `true`, генерирует однострочные сводки прогресса для подагентов и пересылает их на события [`task_progress`](#sdktaskprogressmessage) через поле `summary`. Применяется к подагентам переднего плана и фонового режима                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                                                                                                                      | `false`                                                             | Включить обход разрешений. Требуется при использовании `permissionMode: 'bypassPermissions'`, при запуске или позже через `setPermissionMode()`. См. [режим плана](/docs/ru/agent-sdk/permissions#plan-mode-plan) для взаимодействия с `permissionMode: 'plan'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `allowedTools`                    | `string[]`                                                                                                                                                                                                     | `[]`                                                                | Инструменты для автоматического одобрения без запроса. Это не ограничивает Claude только этими инструментами. Если вы назовете один из [инструментов отслеживания задач](/docs/ru/agent-sdk/todo-tracking#model-availability) здесь, Claude Code также включит сеанс. Другие неуказанные инструменты переходят к `permissionMode` и `canUseTool`. Используйте `disallowedTools` для блокировки инструментов. См. [Разрешения](/docs/ru/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                                                                                                                      | `[]`                                                                | Включить бета-функции                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                                                                                                                                    | `undefined`                                                         | Пользовательская функция разрешений, вызываемая только когда [поток разрешений](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) переходит к запросу. Не вызывается для вызовов, автоматически одобренных `allowedTools`, правилами разрешения или `permissionMode`. Правило разрешения не предварительно одобряет [действия, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves). См. [`CanUseTool`](#canusetool) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `continue`                        | `boolean`                                                                                                                                                                                                      | `false`                                                             | Продолжить самый последний разговор                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `cwd`                             | `string`                                                                                                                                                                                                       | `process.cwd()`                                                     | Текущий рабочий каталог                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `debug`                           | `boolean`                                                                                                                                                                                                      | `false`                                                             | Включить режим отладки для процесса Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `debugFile`                       | `string`                                                                                                                                                                                                       | `undefined`                                                         | Записать журналы отладки в определенный путь файла. Неявно включает режим отладки                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `disallowedTools`                 | `string[]`                                                                                                                                                                                                     | `[]`                                                                | Инструменты для отказа. Простое имя, такое как `"Bash"`, удаляет инструмент из контекста Claude. Правило с областью действия, такое как `"Bash(rm *)"`, оставляет инструмент доступным и отклоняет соответствующие вызовы в каждом режиме разрешений, включая `bypassPermissions`, для команды [как написано](/docs/ru/permissions#bash-rule-limits). См. [Разрешения](/docs/ru/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                                                                                                                              | `undefined`                                                         | Контролирует, сколько усилий Claude вкладывает в свой ответ. Работает с адаптивным мышлением для направления глубины мышления. См. [отрегулировать уровень усилий](/docs/ru/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `enableFileCheckpointing`         | `boolean`                                                                                                                                                                                                      | `false`                                                             | Включить отслеживание изменений файлов для перемотки. См. [File checkpointing](/docs/ru/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `env`                             | `Record<string, string \| undefined>`                                                                                                                                                                          | `process.env`                                                       | Переменные окружения. Когда установлено, это заменяет окружение подпроцесса вместо слияния с `process.env`, поэтому передайте `{ ...process.env, YOUR_VAR: 'value' }` для сохранения унаследованных переменных, таких как `PATH`. См. [Обработка медленных или зависших ответов API](#handle-slow-or-stalled-api-responses) для примера этого паттерна и [Переменные окружения](/docs/ru/env-vars) для переменных, которые читает базовый CLI. Установите `CLAUDE_AGENT_SDK_CLIENT_APP` для идентификации вашего приложения в заголовке User-Agent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                                                                                                                                    | Автоопределение                                                     | Используемая среда выполнения JavaScript                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `executableArgs`                  | `string[]`                                                                                                                                                                                                     | `[]`                                                                | Аргументы для передачи исполняемому файлу                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `extraArgs`                       | `Record<string, string \| null>`                                                                                                                                                                               | `{}`                                                                | Дополнительные аргументы                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `fallbackModel`                   | `string`                                                                                                                                                                                                       | `undefined`                                                         | Модель для использования, если основная модель не работает. Принимает список, разделенный запятыми. Для порядка и ограничения см. [Цепочки резервных моделей](/docs/ru/model-config#fallback-model-chains). Для рекомендаций см. [Выберите модель](/docs/ru/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `forkSession`                     | `boolean`                                                                                                                                                                                                      | `false`                                                             | При возобновлении с `resume` разветвить на новый ID сеанса вместо продолжения исходного сеанса                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `forwardSubagentText`             | `boolean`                                                                                                                                                                                                      | `false`                                                             | Пересылать текст подагента и блоки мышления как сообщения помощника и пользователя с установленным `parent_tool_use_id`, чтобы потребители могли отобразить вложенную стенограмму. Без этого параметра Claude Code выдает блоки `tool_use` и `tool_result` подагента, но не текст или мышление. Сообщения от подагентов на каждой глубине вложенности пересылаются на Claude Code v2.1.219 и позже; до v2.1.219 появлялись только сообщения от подагентов глубины 1. Сообщения подагентов, которые разветвленный навык порождает, и вложенные разветвленные навыки требуют v2.1.275 или позже                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`                                                                                                              | `{}`                                                                | Обратные вызовы hook для событий                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `includeHookEvents`               | `boolean`                                                                                                                                                                                                      | `false`                                                             | Включить события жизненного цикла hook в поток сообщений как [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage) и [`SDKHookResponseMessage`](#sdkhookresponsemessage). События жизненного цикла для hook `SessionStart` и `Setup` всегда включены и не требуют этого параметра. Некоторые события hook, такие как `Notification`, `SessionEnd`, `PreCompact` и `PostCompact`, никогда не создают `SDKHookStartedMessage`, даже с этим параметром. Для этих событий Claude Code все еще выдает `SDKHookProgressMessage`, пока hook команды работает более одной секунды, и выдает `SDKHookResponseMessage` только когда hook [работающий в фоновом режиме](/docs/ru/hooks#run-hooks-in-the-background) завершается                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `includePartialMessages`          | `boolean`                                                                                                                                                                                                      | `false`                                                             | Включить события частичных сообщений                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `loadTimeoutMs`                   | `number`                                                                                                                                                                                                       | `60000`                                                             | *Alpha.* Тайм-аут в миллисекундах для каждого вызова `sessionStore.load()` и `sessionStore.listSubkeys()` во время материализации возобновления. Если адаптер не разрешится в этом окне, запрос не удается вместо зависания. Игнорируется, когда `sessionStore` не установлен                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `managedSettings`                 | `Settings`                                                                                                                                                                                                     | `undefined`                                                         | Параметры уровня политики, которые ваш хост-процесс предоставляет порожденному сеансу. На машинах с развернутыми администратором управляемыми параметрами Claude Code игнорирует их, если только источник управляемых параметров администратора с наивысшим приоритетом не установит `parentSettingsBehavior: 'merge'`, и никогда не объединяет их, пока [`policyHelper`](/docs/ru/settings-reference#policyhelper) предоставляет управляемые параметры. Объединенные значения проходят через фильтр только для ограничений; [Ограничить параметры родителя](/docs/ru/claude-apps-gateway#restrict-parent-settings) охватывает то, что допускает фильтр и блокировки `allowManaged*Only`. Хост, который устанавливает [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ru/env-vars), имеет три ключа, прочитанные прямо из этого полезного груза: его [конфигурация модели](/docs/ru/model-config#restrict-model-selection) на Claude Code v2.1.222 или позже, [`modelPricing`](/docs/ru/settings-reference#modelpricing) когда ни один управляемый источник не устанавливает его на v2.1.246 или позже, и его запись `ENABLE_TOOL_SEARCH` env на v2.1.247 или позже                                                                                                                                                                                            |
| `maxBudgetUsd`                    | `number`                                                                                                                                                                                                       | `undefined`                                                         | Остановить запрос, когда оценка стоимости на стороне клиента достигает этого значения в USD. Сравнивается с той же оценкой, что и `total_cost_usd`. Для предостережений точности и поведения сброса см. [Отслеживание стоимости и использования](/docs/ru/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `maxThinkingTokens`               | `number`                                                                                                                                                                                                       | `undefined`                                                         | *Устарело:* Используйте `thinking` вместо этого. Максимальные токены для процесса мышления                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `maxTurns`                        | `number`                                                                                                                                                                                                       | `undefined`                                                         | Максимальное количество агентивных ходов (раунды использования инструментов)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                                                                                                                        | `{}`                                                                | Конфигурации MCP сервера                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `model`                           | `string`                                                                                                                                                                                                       | По умолчанию из CLI                                                 | Псевдоним модели Claude или полное имя модели. См. [принятые значения и ID, специфичные для поставщика](/docs/ru/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`                                                                                                                | `undefined`                                                         | Обратный вызов для обработки запросов MCP elicitation. Вызывается, когда MCP сервер запрашивает ввод пользователя и ни один hook не обрабатывает его первым. Если не предоставлено, необработанные запросы elicitation автоматически отклоняются                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                                                                                                                                  | `undefined`                                                         | Определите формат вывода для результатов агента. См. [Структурированные выходы](/docs/ru/agent-sdk/structured-outputs) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `outputStyle`                     | `string`                                                                                                                                                                                                       | `undefined`                                                         | Не поле `Options`. Установите `outputStyle` в встроенном объекте [`settings`](/docs/ru/settings) или файле параметров вместо этого. См. [Активировать стиль вывода](/docs/ru/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                                                                                                                       | Автоматически разрешено из встроенного собственного двоичного файла | Путь к исполняемому файлу Claude Code. Требуется только если дополнительные зависимости были пропущены во время установки или ваша платформа не входит в поддерживаемый набор                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                                                                                                                            | `'default'`                                                         | Режим разрешений для сеанса                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `permissionPromptToolName`        | `string`                                                                                                                                                                                                       | `undefined`                                                         | Имя инструмента MCP для запросов разрешений                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `permissionPrompts`               | `'host' \| 'none'`                                                                                                                                                                                             | `'host'`                                                            | Кто отвечает на запросы разрешений: `'host'` маршрутизирует их на ваш обратный вызов [`canUseTool`](#canusetool) или инструмент `permissionPromptToolName`, и `'none'` [отклоняет вызовы, которые иначе запросили бы](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated). Требует Claude Code v2.1.259 или позже                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `persistSession`                  | `boolean`                                                                                                                                                                                                      | `true`                                                              | Когда `false`, отключает сохранение сеанса на диск. Сеансы не могут быть возобновлены позже                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `planModeInstructions`            | `string`                                                                                                                                                                                                       | `undefined`                                                         | Пользовательские инструкции рабочего процесса для режима плана. Когда `permissionMode` имеет значение `'plan'`, эта строка заменяет основной текст рабочего процесса режима плана по умолчанию. CLI по-прежнему оборачивает его с преамбулой принудительного применения только для чтения и нижним колонтитулом протокола ExitPlanMode                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                                                                                                                      | `[]`                                                                | Загрузить пользовательские плагины из локальных путей. См. [Плагины](/docs/ru/agent-sdk/plugins) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `projectConfigRoot`               | `string`                                                                                                                                                                                                       | `undefined`                                                         | Абсолютный путь доверенного checkout, который `cwd` является worktree. Claude Code читает параметры проекта, `.mcp.json` и команды, агентов, навыки, рабочие процессы, процедуры и стили вывода проекта `.claude/` из этого каталога вместо `cwd`, и устанавливает `CLAUDE_PROJECT_DIR` на него. hooks, вспомогательные скрипты, такие как `apiKeyHelper`, и stdio MCP серверы начинают с этого каталога как их рабочего каталога. Файлы `CLAUDE.md` и `.claude/rules/` по-прежнему загружаются из `cwd`. Требует Claude Code v2.1.275 или позже                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `promptSuggestions`               | `boolean`                                                                                                                                                                                                      | `false`                                                             | Включить предложения подсказок. После хода Claude Code выдает сообщение `prompt_suggestion`, содержащее предсказанную следующую подсказку пользователя. Claude Code не генерирует предложение для некоторых ходов, например, когда ваша учетная запись близка к лимиту использования или находится на нем. См. [Когда Claude Code пропускает предложения](/docs/ru/interactive-mode#when-claude-code-skips-suggestions)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `resume`                          | `string`                                                                                                                                                                                                       | `undefined`                                                         | ID сеанса для возобновления                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `resumeDropsTurn`                 | `string`                                                                                                                                                                                                       | `undefined`                                                         | С `resumeSessionAt`: UUID подсказки хода, который усеченное возобновление намеревается отбросить. Claude Code отказывает в возобновлении, когда отброшенный диапазон содержит что-либо, не относящееся к этому ходу, такое как поглощенные сообщения в очереди или уведомления о задачах, и называет флаг `--resume-drops-turn` в сообщении об отказе. Только Agent SDK и возобновления в режиме печати читают пару. Требует Claude Code v2.1.223 или позже                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `resumeSessionAt`                 | `string`                                                                                                                                                                                                       | `undefined`                                                         | Возобновить сеанс в определенном UUID сообщения                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                                                                                                                          | `undefined`                                                         | Программно настройте поведение sandbox. См. [Параметры Sandbox](#sandboxsettings) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `sessionId`                       | `string`                                                                                                                                                                                                       | Автогенерируемый                                                    | Используйте определенный UUID для сеанса вместо автогенерации                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `sessionStore`                    | [`SessionStore`](/docs/ru/agent-sdk/session-storage#the-sessionstore-interface)                                                                                                                                     | `undefined`                                                         | Зеркалировать стенограммы сеанса во внешний бэкэнд, чтобы другой хост мог их возобновить. См. [Сохранить сеансы во внешнее хранилище](/docs/ru/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                                                                                                                         | `'batched'`                                                         | *Alpha.* Режим сброса для `sessionStore`. Игнорируется, когда `sessionStore` не установлен                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `settings`                        | `string \| Settings`                                                                                                                                                                                           | `undefined`                                                         | Встроенный объект [settings](/docs/ru/settings), путь файла параметров или встроенная строка JSON. Заполняет уровень параметров флага в [порядке приоритета](/docs/ru/settings#settings-precedence). Измените во время выполнения с помощью [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                                                                                                                          | Значения по умолчанию CLI (все источники)                           | Контролируйте, какие параметры файловой системы загружать. Передайте `[]` для отключения параметров пользователя, проекта и локальных параметров. [Управляемая политика конечной точки](/docs/ru/managed-settings#delivery-mechanisms) загружается независимо; параметры, управляемые сервером, извлекаются, когда сеанс аутентифицируется с учетными данными организации на [подходящей конфигурации](/docs/ru/server-managed-settings#platform-availability). См. [Использование функций Claude Code](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `skills`                          | `string[] \| 'all'`                                                                                                                                                                                            | `undefined`                                                         | Навыки, доступные для сеанса. Передайте `'all'` для включения каждого обнаруженного навыка или список имен навыков. Передавайте только точные имена. На Agent SDK v0.3.221 или позже SDK отклоняет неправильно сформированные и имена в форме подстановочных знаков с ошибкой перед запуском процесса Claude Code. Когда установлено, SDK автоматически добавляет инструмент Skill в `allowedTools`. Если вы также передаете `tools`, включите `'Skill'` в этот список. См. [Навыки](/docs/ru/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                                                                                                                                    | `undefined`                                                         | Пользовательская функция для порождения процесса Claude Code. Используйте для запуска Claude Code на виртуальных машинах, в контейнерах или удаленных окружениях                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `stderr`                          | `(data: string) => void`                                                                                                                                                                                       | `undefined`                                                         | Обратный вызов для вывода stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `strictMcpConfig`                 | `boolean`                                                                                                                                                                                                      | `false`                                                             | Используйте только серверы, переданные в `mcpServers`, и игнорируйте проект `.mcp.json`, параметры пользователя, MCP серверы, предоставленные плагинами, и [соединители claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `systemPrompt`                    | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined` (минимальная подсказка)                                 | Конфигурация системной подсказки. Передайте строку для пользовательской подсказки или `{ type: 'preset', preset: 'claude_code' }` для использования системной подсказки Claude Code. Передайте массив строк с экспортированной константой `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` между статической и частями для каждого запроса для [кэширования статической части пользовательской подсказки](/docs/ru/agent-sdk/modifying-system-prompts#cache-the-static-part-of-a-custom-prompt). При использовании формы объекта preset добавьте `append` для расширения его дополнительными инструкциями и установите `excludeDynamicSections: true` для перемещения контекста для каждого сеанса в первое сообщение пользователя для [лучшего повторного использования кэша подсказок на разных машинах](/docs/ru/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines). Установите `snapshot: false` для перестроения подсказки при каждом запросе вместо [повторного использования подсказки, которую сеанс записал при первом запросе](/docs/ru/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Для установки `snapshot` на пользовательской подсказке передайте форму `{ type: 'custom', prompt }`. Форма `{ type: 'custom' }` и поле `snapshot` требуют TypeScript Agent SDK v0.3.257 или позже |
| `taskBudget`                      | `{ total: number }`                                                                                                                                                                                            | `undefined`                                                         | *Alpha.* Бюджет задач на стороне API в токенах. Когда установлено, модели сообщается оставшийся бюджет токенов, чтобы она могла контролировать использование инструментов и завершить работу перед лимитом                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                                                                                                                            | `{ type: 'adaptive' }` для поддерживаемых моделей                   | Контролирует поведение мышления/рассуждения Claude. См. [`ThinkingConfig`](#thinkingconfig) для параметров                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `title`                           | `string`                                                                                                                                                                                                       | `undefined`                                                         | Отображаемое название для сеанса. При возобновлении через `resume` или `continue` сохраненное название возобновленного сеанса имеет приоритет; используйте [`renameSession()`](#renamesession) для переименования существующего сеанса                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `toolAliases`                     | `Record<string, string>`                                                                                                                                                                                       | `undefined`                                                         | Сопоставьте встроенные имена инструментов с именами инструментов MCP, чтобы Claude вызывал вашу реализацию MCP вместо встроенной. Например, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                                                                                                                                    | `undefined`                                                         | Конфигурация для поведения встроенного инструмента. См. [`ToolConfig`](#toolconfig) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                                                                                                                        | `undefined`                                                         | Конфигурация инструмента. Передайте массив имен инструментов или используйте preset для получения инструментов Claude Code по умолчанию                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

<h4 id="handle-slow-or-stalled-api-responses">
  Обработка медленных или зависших ответов API
</h4>

Подпроцесс CLI читает несколько переменных окружения, которые контролируют тайм-ауты API и обнаружение зависания. Передайте их через параметр `env`:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

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

* `API_TIMEOUT_MS`: тайм-аут для каждого запроса на клиенте Anthropic в миллисекундах. По умолчанию `600000`. Применяется к основному циклу и всем подагентам.
* `CLAUDE_CODE_MAX_RETRIES`: максимальное количество повторных попыток API. По умолчанию `10`, ограничено `15`. Каждая повторная попытка получает свое собственное окно `API_TIMEOUT_MS`, поэтому наихудшее время стены примерно `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` плюс backoff. Для автоматических запусков, которым нужно ждать более длительных сбоев, установите [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/ru/errors#tune-retry-behavior): он повторяет переходящие ошибки емкости бесконечно и, на Claude Code v2.1.199 или позже, повышает значение по умолчанию для других переходящих ошибок до `300` и удаляет ограничение на эту переменную.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: сторож зависания для подагентов. Пока сторож потока включен, значение по умолчанию — `CLAUDE_STREAM_IDLE_TIMEOUT_MS` плюс 5 минут, что составляет `600000`, если вы не повысите эту переменную. Со сторожем потока выключенным, значение по умолчанию — `600000`. До v2.1.257 значение по умолчанию всегда было `600000`.

  Таймер сбрасывается при каждом событии потока. При зависании Claude Code прерывает подагента и сообщает о зависании родителю. Для фонового подагента он также отмечает задачу как неудачную и прикрепляет любой частичный результат.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` с `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: сторож потока, который прерывает запрос, когда заголовки прибыли, но тело ответа перестает потоковать. Сторож включен по умолчанию для всех поставщиков; установите `CLAUDE_ENABLE_STREAM_WATCHDOG=0` для отключения. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` по умолчанию `300000` и зажимается на этот минимум. После прерывания [Автоматические повторные попытки](/docs/ru/errors#automatic-retries) охватывает то, что Claude Code делает, на основе того, как далеко продвинулся ответ.

  Пока сторож ждет ответа, который шлюз позади `ANTHROPIC_BASE_URL` держит открытым с ping-пингами keep-alive, хост, который устанавливает `includePartialMessages`, продолжает получать `ping` [события потока](#sdkpartialassistantmessage), поэтому читайте эти кадры как живость, а не тайм-аут сеанса на молчании. До v2.1.257 кадры останавливались через 5 минут после последнего реального события потока.

<h3 id="query-object">
  Объект `Query`
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
  applyFlagSettings(settings: {
    [K in keyof Settings]?: K extends 'effortLevel'
      ? 'low' | 'medium' | 'high' | 'xhigh' | 'max' | null
      : Settings[K] | null;
  }): Promise<void>;
  updateSettings(
    source: 'localSettings' | 'userSettings',
    settings: Record<string, unknown>,
  ): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  getContextUsage(opts?: {
    detail?: 'summary' | 'full';
  }): Promise<SDKControlGetContextUsageResponse>;
  readFile(
    path: string,
    options?: { maxBytes?: number; encoding?: 'utf-8' | 'base64' }
  ): Promise<SDKControlReadFileResponse | null>;
  reloadSkills(): Promise<SDKControlReloadSkillsResponse>;
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
  Методы
</h4>

| Метод                                  | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `interrupt()`                          | Прерывает запрос. Доступно только в режиме потокового ввода. Когда CLI объявляет возможность `interrupt_receipt_v1` в [`SDKSystemMessage.capabilities`](#sdksystemmessage), разрешается с помощью [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse), перечисляющего сообщения, которые были в ожидании при поступлении прерывания. Разрешается `undefined` на CLI до v2.1.205                                                                                                                                                                           |
| `rewindFiles(userMessageId, options?)` | Восстанавливает файлы в их состояние в указанном сообщении пользователя. Передайте `{ dryRun: true }` для предварительного просмотра изменений. Требует `enableFileCheckpointing: true`. См. [File checkpointing](/docs/ru/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                 |
| `setPermissionMode()`                  | Изменяет режим разрешений (доступно только в режиме потокового ввода)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `setModel()`                           | Изменяет модель (доступно только в режиме потокового ввода). Передача `undefined` или строки `"default"` сбрасывает на [модель Claude Code по умолчанию](/docs/ru/model-config)                                                                                                                                                                                                                                                                                                                                                                                          |
| `setMaxThinkingTokens()`               | *Устарело:* Используйте параметр `thinking` вместо этого. Изменяет максимальные токены мышления. Передача `null` сбрасывает мышление на значение по умолчанию сеанса: переопределение в середине сеанса очищается, и мышление остается отключенным для сеансов, у которых оно отключено                                                                                                                                                                                                                                                                             |
| `applyFlagSettings(settings)`          | Объединяет параметры в уровень параметров флага сеанса во время выполнения (доступно только в режиме потокового ввода). См. [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                             |
| `updateSettings(source, settings)`     | Записывает один разрешенный ключ в файл локальных параметров проекта или файл параметров пользователя, чтобы значение сохранялось для последующих сеансов. См. [`updateSettings()`](#updatesettings). Требует TypeScript SDK v0.3.257 или позже, который поставляется с Claude Code v2.1.257                                                                                                                                                                                                                                                                        |
| `initializationResult()`               | Возвращает полный результат инициализации, включая поддерживаемые команды, модели, информацию об учетной записи и конфигурацию стиля вывода                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `reinitialize()`                       | Повторно отправляет запрос управления `initialize` на работающий CLI и возвращает свежий результат вместо кэшированного результата первого подключения. Используйте его после разрыва транспорта, например, переподключение к сеансу после отключения, чтобы ожидающие запросы разрешений снова достигли вашего обратного вызова `canUseTool`. Сделайте обратный вызов идемпотентным для каждого ID запроса, потому что запрос, чей ответ был потерян, отправляется снова. Требует Claude Code v2.1.195 или позже                                                   |
| `supportedCommands()`                  | Возвращает доступные команды. Из Agent SDK v0.3.216 список отражает изменения команд в середине сеанса; см. [`SDKCommandsChangedMessage`](#sdkcommandschangedmessage)                                                                                                                                                                                                                                                                                                                                                                                               |
| `supportedModels()`                    | Возвращает доступные модели с информацией об отображении                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `supportedAgents()`                    | Возвращает доступные подагентов как [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `mcpServerStatus()`                    | Возвращает статус подключенных MCP серверов                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `getContextUsage(opts?)`               | Возвращает [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse), разбивая использование контекстного окна сеанса по категориям, навыкам и инструментам. С параметром по умолчанию `detail`, это те же данные, которые `/context` показывает в интерактивном сеансе. Параметр [`detail`](#sdkcontrolgetcontextusageresponse) требует Agent SDK v0.3.257 или позже                                                                                                                                                                               |
| `readFile(path, options?)`             | Читает файл из файловой системы сеанса. Claude Code разрешает путь против `cwd`; [Что `readFile()` может читать](#what-readfile-can-read) перечисляет файлы, которые он обслуживает. Передайте `{ maxBytes }` для изменения ограничения чтения (по умолчанию 1 МБ, потолок 10 МБ) и `{ encoding: 'base64' }` для двоичных файлов, таких как изображения. Разрешается с помощью [`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse) или `null` при отказе в разрешении, отсутствующем файле или ошибке транспорта. Требует TypeScript SDK v0.2.121 или позже |
| `reloadSkills()`                       | Перезагружает навыки с диска, поэтому навыки, которые вы добавляете или редактируете в середине сеанса, становятся доступны для работающего сеанса. Разрешается с помощью [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse), перечисляющего доступные навыки после перезагрузки. Требует Agent SDK v0.3.163 или позже                                                                                                                                                                                                                             |
| `accountInfo()`                        | Возвращает информацию об учетной записи                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `reconnectMcpServer(serverName)`       | Переподключить MCP сервер по имени. Если имя также совпадает с записью в файле параметров, таком как `.mcp.json` или `~/.claude.json`, Claude Code переподключает сервер, который вы настроили через [`mcpServers`](#options) или `setMcpServers()`, а не запись файла параметров. Этот порядок разрешения требует Claude Code v2.1.257 или позже                                                                                                                                                                                                                   |
| `toggleMcpServer(serverName, enabled)` | Включить или отключить MCP сервер по имени с тем же разрешением имени, что и `reconnectMcpServer()`. Отключение отключает сервер                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `setMcpServers(servers)`               | Динамически замените набор MCP серверов для этого сеанса. Разрешается с помощью [`McpSetServersResult`](#mcpsetserversresult), называющего, какие серверы были добавлены и удалены, и любые ошибки                                                                                                                                                                                                                                                                                                                                                                  |
| `streamInput(stream)`                  | Потоковые входные сообщения в запрос для многоходовых разговоров                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `stopTask(taskId)`                     | Остановить работающую фоновую задачу по ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `close()`                              | Закройте запрос и завершите базовый процесс. Принудительно завершает запрос и очищает все ресурсы                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Изменяет [параметры](/docs/ru/settings) на работающем сеансе без перезагрузки запроса. Используйте его, когда параметр, у которого нет выделенного сеттера, должен измениться в середине сеанса, например, ужесточение `permissions` после того, как агент прочитает ненадежный ввод. `setModel()` и `setPermissionMode()` — это выделенные сеттеры для этих двух ключей; `applyFlagSettings()` — это общая форма, которая принимает любое подмножество ключей параметров, и передача `model` здесь ведет себя так же, как `setModel()`.

Только некоторые ключи вступают в силу в середине сеанса:

* **Применяется при следующем ходе**: `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Переключение `agent` также применяет переопределение модели и hooks этого агента при следующем ходе. Его системная подсказка применяется при следующем ходе или, в сеансе, который [повторно использует записанную системную подсказку](/docs/ru/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session), после компактирования сеанса.
* **Применяется во время текущего хода**: `model`. Если вы переключаете `model` пока Claude работает над ходом, ответ, который Claude уже генерирует, завершается на старой модели, и остаток хода, начиная со следующего вызова Claude Code к модели, использует новую. Подагенты сохраняют свою собственную модель. До v2.1.212 переключение в середине хода ждало следующего хода.
* **Нет эффекта в середине сеанса**: параметры системной подсказки. Они разрешаются один раз при запуске, поэтому работающий сеанс сохраняет исходное значение, даже если вызов успешен. Чтобы их изменить, запустите новый сеанс.

`effortLevel` принимает имя [уровня усилий](/docs/ru/model-config#adjust-effort-level). Он также принимает `"ultracode"`, который запрашивает усилие `xhigh` с включенным [ultracode](/docs/ru/workflows#let-claude-decide-with-ultracode). `applyFlagSettings()` объявляет `effortLevel` без этого значения, поэтому передайте эквивалент `{ ultracode: true }` в TypeScript. Значение `ultracode` требует Claude Code v2.1.203 или позже и принимается только `applyFlagSettings()`, а не ключом `effortLevel` в файле параметров.

Значения записываются в уровень параметров флага, тот же уровень, который встроенный параметр `settings` функции `query()` заполняет при запуске. Это тот же уровень, который раздел [приоритета на странице](#settings-precedence) называет программными параметрами.

Последовательные вызовы выполняют поверхностное слияние ключей верхнего уровня. Второй вызов с `{ permissions: {...} }` заменяет весь объект `permissions` из предыдущего вызова, а не глубоко объединяется в него.

Чтобы очистить ключ, который вы установили с помощью `applyFlagSettings()`, передайте `null` для этого ключа. Большинство ключей затем возвращаются к значению, которое параметр `settings` функции `query()` установил при запуске, затем к источникам с более низким приоритетом. Очищенный `model` сбрасывается на [модель Claude Code по умолчанию](/docs/ru/model-config), даже когда файл параметров устанавливает `model`. Передача `undefined` не имеет эффекта, потому что сериализация JSON удаляет ее.

Три ключа помимо `model` сбрасывают состояние сеанса вместо возврата:

* `effortLevel: null` возвращает сеанс к уровню усилий модели по умолчанию, а не к параметру `effort` функции `query()` или `effortLevel` из файла параметров.
* `agent: null` запускает основной поток без агента, начиная со следующего хода, а не восстанавливает параметр `agent` функции `query()` или `agent` из файла параметров. Если очищенный агент применил свою собственную модель, сеанс возвращается к модели, которую он разрешил при запуске.
* `ultracode: null` отключает ultracode, как это делает `false`, а не восстанавливает значение `ultracode` из файла параметров. Сеанс сохраняет свой текущий уровень усилий, поэтому передайте `effortLevel` в том же вызове, чтобы его изменить.

Доступно только в режиме потокового ввода, то же ограничение, что и `setModel()` и `setPermissionMode()`.

Пример ниже переключает активную модель в середине сеанса, а затем очищает переопределение, чтобы модель сбросилась на [модель Claude Code по умолчанию](/docs/ru/model-config).

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// Override the model for the rest of the session
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Later: clear the override; the model resets to Claude Code's default
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` только для TypeScript. Python SDK не предоставляет эквивалентный метод.
</Note>

<h4 id="updatesettings">
  `updateSettings()`
</h4>

Записывает один разрешенный ключ в файл параметров на диск, чтобы значение сохранялось для последующих сеансов, которые загружают этот источник. Каждый источник принимает один ключ со строковым значением:

* **`"localSettings"`**: принимает `outputStyle` и объединяет его в файл локальных параметров проекта, `.claude/settings.local.json`. Новый стиль вступает в силу при следующем запросе сеанса.
* **`"userSettings"`**: принимает `effortLevel` и сохраняет его как [уровень усилий](/docs/ru/model-config#adjust-effort-level) по умолчанию для текущей модели сеанса, под [`modelSettings`](/docs/ru/settings-reference#modelsettings) в файле параметров пользователя. Передача `max` ничего не записывает, потому что `max` только для сеанса. Работающий сеанс сохраняет свой текущий уровень усилий в любом случае, поэтому вызовите [`applyFlagSettings()`](#applyflagsettings) когда вы также хотите это изменить. Этот источник требует TypeScript SDK v0.3.277 или позже, который поставляется с Claude Code v2.1.277.

Вызов отклоняется, когда запрос содержит любой другой ключ, когда сеанс работает через удаленный транспорт, и когда [`settingSources`](#options) сеанса исключают источник, который вы назвали. Удаление ключа не поддерживается.

<h3 id="warmquery">
  `WarmQuery`
</h3>

Дескриптор, возвращаемый [`startup()`](#startup). Подпроцесс уже порожден и инициализирован, поэтому вызов `query()` на этом дескрипторе записывает подсказку непосредственно в готовый процесс без задержки запуска.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Методы
</h4>

| Метод           | Описание                                                                                                                                                 |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `query(prompt)` | Отправьте подсказку на предварительно прогретый подпроцесс и верните [`Query`](#query-object). Может быть вызван только один раз для каждого `WarmQuery` |
| `close()`       | Закройте подпроцесс без отправки подсказки. Используйте это для отказа от теплого запроса, который больше не нужен                                       |

`WarmQuery` реализует `AsyncDisposable`, поэтому его можно использовать с `await using` для автоматической очистки.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

Тип возврата `initializationResult()`. Содержит данные инициализации сеанса.

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
  fast_mode_disabled_reason?: FastModeDisabledReason;
  hooks_applied?: boolean;
};
```

`hooks_applied` сообщает, зарегистрировал ли Claude Code `hooks`, которые несла запрос `initialize`. SDK отправляет этот запрос один раз при запуске сеанса и снова при каждом вызове [`reinitialize()`](#query-object). Поле требует Agent SDK v0.3.238 или позже.

Claude Code опускает поле, когда запрос не содержал hooks. Когда запрос содержал hooks, значение зависит от того, является ли запрос первой инициализацией сеанса и, для повторного, как он достиг сеанса:

* `true`: Claude Code зарегистрировал hooks. Первая инициализация сеанса возвращает это значение. Повторная инициализация, отправленная через stdin CLI, также возвращает `true`. В этом случае hooks в новом запросе заменяют hooks, зарегистрированные ранее.
* `false`: Claude Code игнорировал hooks. Повторная инициализация, отправленная на удаленный сеанс, возвращает это значение, поэтому второй клиент, присоединяющийся к сеансу, не может заменить hooks, которые зарегистрировал первый клиент.

До Agent SDK v0.3.238 ответ никогда не содержал поле, и Claude Code игнорировал `hooks` при каждой повторной инициализации.

Ответ всегда сообщает `fast_mode_state`, и когда что-то блокирует [быстрый режим](/docs/ru/fast-mode), `fast_mode_disabled_reason` несет код причины рядом с ним, поэтому вы можете объяснить заблокированное состояние вместо повторного вывода доступности. Оба поведения требуют Claude Code v2.1.219 или позже. До v2.1.219 ответ опускал `fast_mode_state`, когда быстрый режим был недоступен, и никогда не содержал причину. Для кодов причин и их значений см. [`fast_mode_disabled_reason`](#sdkresultmessage) в сообщении результата.

Оболочка управления-ответа для успешного `initialize` также содержит массив `pending_permission_requests`. Поле находится на самой оболочке ответа, а не в полезной нагрузке `SDKControlInitializeResponse` выше. Каждая запись — это полное сообщение `control_request` с той же формой `{ type: "control_request", request_id, request }`, которую сеанс потоком для запросов разрешений во время работы.

Массив перечисляет запросы разрешений, которые этот процесс Claude Code выдал и еще не разрешил. SDK читает массив для вас и отправляет каждую запись на ваш обратный вызов [`canUseTool`](#canusetool), то же переоформление, которое [`reinitialize()`](#query-object) запускает после разрыва транспорта. Обрабатывайте повторяющиеся ID запросов идемпотентно, потому что запись может повторить запрос, который обратный вызов уже получил перед отключением соединения.

Массив всегда присутствует в успешном ответе `initialize` и пуст, когда этот процесс не имеет неразрешенного запроса разрешения. Требует Claude Code v2.1.268 или позже. Более ранние версии могли опустить поле, поэтому если вы анализируете протокол проводов самостоятельно, рассматривайте отсутствующее поле как более старый CLI, а не как доказательство того, что ничего не ожидается.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

Квитанция прерывания: значение, которое [`interrupt()`](#query-object) разрешает на CLI, который объявляет возможность `interrupt_receipt_v1` в [`SDKSystemMessage.capabilities`](#sdksystemmessage). Требует Claude Code v2.1.205 или позже. Более ранние CLI отвечают на прерывание с пустой полезной нагрузкой успеха, поэтому `interrupt()` разрешается `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued` перечисляет UUID пользовательских сообщений, которые были в ожидании при поступлении прерывания: сообщения все еще в очереди, плюс любые сообщения, которые Claude Code уже вынул из очереди для следующего хода. После того как первый ход сеанса начался, Claude Code обрабатывает перечисленные сообщения после прерывания, если вы их не отмените первыми, и может объединить несколько в один ход. Если вы прерываете перед началом первого хода, Claude Code прерывает этот ход, как только он начинается, и перечисленные сообщения в этом ходе не получают ответ.

Используйте квитанцию, чтобы решить, нужно ли что-то переотправлять. Перечисленное сообщение, которое вы не отмените, входит в разговор независимо от того, получит ли оно ответ, поэтому переотправка его доставляет его Claude дважды.

Интерпретируйте список с этими предостережениями:

* Появляются только сообщения, которые были поставлены в очередь с UUID. Пустой массив не означает, что ничего больше не будет работать.
* Перечислены только сообщения основного потока. Сообщения, адресованные подагенту, выходят за рамки.
* Список может включать UUID, которые ваш клиент никогда не отправлял, такие как триггеры [запланированной задачи](/docs/ru/scheduled-tasks). Игнорируйте UUID, которые вы не узнаете, вместо того чтобы рассматривать их как ошибку.

Клиент, который управляет протоколом управления CLI напрямую, а не через `interrupt()`, может установить `cancel_queued: true` на запрос управления `interrupt`. Claude Code v2.1.219 и позже объявляет поддержку с возможностью `interrupt_cancel_queued_v1` в [`SDKSystemMessage.capabilities`](#sdksystemmessage); более старые CLI игнорируют поле и оставляют сообщения в очереди для обычного запуска. Такое прерывание также отменяет каждое сообщение, которое иначе было бы перечислено в `still_queued`: квитанция перечисляет их в `cancelled` вместо этого, `still_queued` пуст, и ни один из них не запускается.

Список `cancelled` содержит те же предостережения, что и `still_queued`. Метод `interrupt()` никогда не отправляет `cancel_queued`, поэтому квитанции, которые он разрешает, не содержат `cancelled`.

Квитанция — это снимок, сделанный в момент обработки прерывания, и при чистом прерывании она прибывает перед [`SDKResultMessage`](#sdkresultmessage) прерванного хода. Читайте квитанцию, а не проверяйте очередь после этого результата: цикл немедленно запускает следующий ход в очереди, поэтому очередь, которую вы проверяете после результата, уже изменилась.

<h3 id="sdkcontrolgetcontextusageresponse">
  `SDKControlGetContextUsageResponse`
</h3>

Тип возврата [`getContextUsage()`](#query-object). С параметром по умолчанию `detail`, это та же полезная нагрузка, которую Claude Code отображает для команды `/context` в интерактивном сеансе, поэтому наряду с подсчетом токенов она содержит поля отображения, такие как `color` и `gridRows`, которые Claude Code использует для рисования сетки использования `/context`.

Аргумент `detail` метода выбирает, как Claude Code подсчитывает каждую категорию. С параметром по умолчанию `'full'`, Claude Code подсчитывает каждую категорию с запросами API подсчета токенов. Передайте `{ detail: 'summary' }` для получения ответа из использования последнего ответа и локальных оценок вместо этого. Никакие запросы подсчета токенов не выходят, и числа для каждой категории приблизительны. Аргумент `detail` требует Agent SDK v0.3.257 или позже.

Когда вы отправляете `/context` как подсказку вместо вызова метода, Claude Code прикрепляет полезную нагрузку [`SDKContextUsage`](#sdkcontextusage) к полю `context_usage` сообщения помощника, которое доставляет результат. Это поле требует Agent SDK v0.3.232 или позже.

```typescript theme={null}
type SDKControlGetContextUsageResponse = {
  categories: {
    name: string;
    tokens: number;
    color: string;
    isDeferred?: boolean;
  }[];
  totalTokens: number;
  maxTokens: number;
  rawMaxTokens: number;
  percentage: number;
  gridRows: {
    color: string;
    isFilled: boolean;
    categoryName: string;
    tokens: number;
    percentage: number;
    squareFullness: number;
  }[][];
  model: string;
  memoryFiles: {
    path: string;
    type: string;
    tokens: number;
  }[];
  mcpTools: {
    name: string;
    serverName: string;
    tokens: number;
    isLoaded?: boolean;
  }[];
  deferredBuiltinTools?: {
    name: string;
    tokens: number;
    isLoaded: boolean;
  }[];
  systemTools?: {
    name: string;
    tokens: number;
  }[];
  systemPromptSections?: {
    name: string;
    tokens: number;
  }[];
  agents: {
    agentType: string;
    source: string;
    tokens: number;
  }[];
  slashCommands?: {
    totalCommands: number;
    includedCommands: number;
    tokens: number;
  };
  skills?: {
    totalSkills: number;
    includedSkills: number;
    tokens: number;
    skillFrontmatter: {
      name: string;
      source: string;
      tokens: number;
    }[];
  };
  autoCompactThreshold?: number;
  isAutoCompactEnabled: boolean;
  messageBreakdown?: {
    toolCallTokens: number;
    toolResultTokens: number;
    attachmentTokens: number;
    assistantMessageTokens: number;
    userMessageTokens: number;
    redirectedContextTokens: number;
    unattributedTokens: number;
    toolCallsByType: {
      name: string;
      callTokens: number;
      resultTokens: number;
    }[];
    attachmentsByType: {
      name: string;
      tokens: number;
    }[];
  };
  apiUsage: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens: number;
    cache_read_input_tokens: number;
  } | null;
};
```

Читайте атрибуцию токенов из полей коллекции:

* `categories` содержит итоги для каждой категории.
* `mcpTools` и `agents` атрибутируют токены отдельным инструментам MCP и подагентам.
* `memoryFiles` перечисляет каждый загруженный файл памяти с его стоимостью.
* `skills.skillFrontmatter` атрибутирует токены списка навыков каждому включенному навыку. Подсчеты для каждого навыка измеряют запись каждого навыка, как Claude Code фактически отправляет ее, что может быть короче полного frontmatter навыка. Сравните `skills.totalSkills` с `skills.includedSkills`, чтобы увидеть, попал ли каждый обнаруженный навык в список.

`totalTokens` — это текущее использование контекста сеанса, а `maxTokens` — это окно, против которого измеряется использование. Это окно — контекстное окно модели или более низкое окно автокомпактирования, когда оно применяется. `rawMaxTokens` содержит то же значение, что и `maxTokens`, и `percentage` — это `totalTokens` как округленный процент этого окна.

Claude Code оставляет дополнительные диагностики `deferredBuiltinTools`, `systemTools` и `systemPromptSections` неустановленными, поэтому ожидайте их отсутствия, даже если тип их объявляет.

<h3 id="sdkcontrolreadfileresponse">
  `SDKControlReadFileResponse`
</h3>

Тип возврата [`readFile()`](#query-object).

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents` содержит текст файла или данные base64, когда вы запросили `encoding: 'base64'`; поле `encoding` ответа установлено на `'base64'` в этом случае. `absPath` — это разрешенный абсолютный путь. `truncated` установлено, когда файл был длиннее ограничения `maxBytes` и содержимое было обрезано на этом пределе.

<h4 id="what-readfile-can-read">
  Что `readFile()` может читать
</h4>

`readFile()` обслуживает более узкий набор файлов, чем инструмент Read:

* Обычный файл внутри одного из рабочих каталогов сеанса, таких как `cwd` и `additionalDirectories`
* Несколько собственных файлов Claude Code для сеанса, таких как результаты инструментов

Правила отказа и запроса `Read` по-прежнему блокируют соответствующий путь, и широкое правило разрешения `Read` не открывает остальную часть файловой системы для `readFile()`. Для всего остального вызов разрешается с `null`.

<h3 id="sdkcontrolreloadskillsresponse">
  `SDKControlReloadSkillsResponse`
</h3>

Тип возврата [`reloadSkills()`](#query-object).

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills` перечисляет доступные навыки после перезагрузки в той же форме [`SlashCommand`](#slashcommand), которую возвращает `supportedCommands()`.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Конфигурация для подагента, определенного программно.

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
  omitClaudeMd?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Поле                                  | Требуется | Описание                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------------------ | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | Да        | Описание на естественном языке, когда использовать этого агента                                                                                                                                                                                                                                                                                                                          |
| `tools`                               | Нет       | Массив разрешенных имен инструментов. Если опущено, наследует каждый [инструмент, доступный подагентам](/docs/ru/sub-agents#available-tools). Чтобы предварительно загрузить Skills в контекст агента, используйте поле `skills` вместо перечисления `'Skill'` здесь                                                                                                                          |
| `disallowedTools`                     | Нет       | Массив имен инструментов для явного запрета для этого агента. Также принимаются шаблоны уровня MCP сервера: `mcp__server` или `mcp__server__*` удаляет каждый инструмент с этого сервера, и `mcp__*` удаляет каждый инструмент MCP с любого сервера                                                                                                                                      |
| `prompt`                              | Да        | Системная подсказка агента                                                                                                                                                                                                                                                                                                                                                               |
| `model`                               | Нет       | Переопределение модели для этого агента. Принимает псевдоним, такой как `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, или полный ID модели. `'inherit'` использует основную модель. Когда вы его опускаете, Claude Code выбирает модель в [порядке модели подагента](/docs/ru/sub-agents#choose-a-model)                                                                           |
| `mcpServers`                          | Нет       | Спецификации MCP сервера для этого агента                                                                                                                                                                                                                                                                                                                                                |
| `skills`                              | Нет       | Массив имен навыков для предварительной загрузки в контекст агента                                                                                                                                                                                                                                                                                                                       |
| `initialPrompt`                       | Нет       | Автоматически отправляется как первый ход пользователя, когда этот агент работает как агент основного потока                                                                                                                                                                                                                                                                             |
| `maxTurns`                            | Нет       | Максимальное количество агентивных ходов (раунды API) перед остановкой                                                                                                                                                                                                                                                                                                                   |
| `background`                          | Нет       | Запустить этого агента как неблокирующую фоновую задачу при вызове                                                                                                                                                                                                                                                                                                                       |
| `omitClaudeMd`                        | Нет       | Запустить этого агента без файлов CLAUDE.md пользователя, проекта и локальных файлов, когда он работает как подагент; управляемые файлы политики по-прежнему загружаются. Используйте его для агентов, которые берут все необходимое из подсказки инструмента Agent. Игнорируется, когда этот агент работает как агент основного потока. Требует TypeScript Agent SDK v0.3.271 или позже |
| `memory`                              | Нет       | Источник памяти для этого агента: `'user'`, `'project'` или `'local'`                                                                                                                                                                                                                                                                                                                    |
| `effort`                              | Нет       | Уровень усилий рассуждения для этого агента. Принимает названный уровень или целое число                                                                                                                                                                                                                                                                                                 |
| `permissionMode`                      | Нет       | Режим разрешений для выполнения инструментов в этом агенте. [Правила наследования подагента](/docs/ru/agent-sdk/permissions#available-modes) решают, когда он применяется. См. [`PermissionMode`](#permissionmode)                                                                                                                                                                            |
| `criticalSystemReminder_EXPERIMENTAL` | Нет       | Экспериментальный: Критическое напоминание, добавленное в системную подсказку                                                                                                                                                                                                                                                                                                            |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

Указывает MCP серверы, доступные подагенту. Может быть именем сервера (строка, ссылающаяся на сервер из конфигурации `mcpServers` родителя) или встроенной записью конфигурации сервера, сопоставляющей имена серверов с конфигурациями.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

Где `McpServerConfigForProcessTransport` — это `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`.

<h3 id="settingsource">
  `SettingSource`
</h3>

Контролирует, какие источники конфигурации на основе файловой системы SDK загружает параметры из.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| Значение    | Описание                                                                            | Местоположение                |
| :---------- | :---------------------------------------------------------------------------------- | :---------------------------- |
| `'user'`    | Глобальные параметры пользователя                                                   | `~/.claude/settings.json`     |
| `'project'` | Общие параметры проекта (контролируемые версией)                                    | `.claude/settings.json`       |
| `'local'`   | Локальные параметры проекта, gitignored когда Claude Code сохраняет параметр в него | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Поведение по умолчанию
</h4>

Когда `settingSources` опущен или `undefined`, `query()` загружает те же параметры файловой системы, что и CLI Claude Code: пользователя, проекта и локальные. См. [Что settingSources не контролирует](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control) для входов, которые читаются независимо от этого параметра, и как их отключить.

<h4 id="why-use-settingsources">
  Почему использовать settingSources
</h4>

**Отключить параметры файловой системы:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Do not load user, project, or local settings from disk
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Загрузить только определенные источники параметров:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Load only project settings, ignore user and local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Only .claude/settings.json
  }
});
```

Чтобы загрузить инструкции проекта CLAUDE.md, включите `"project"` в `settingSources`. См. [Изменить системные подсказки](/docs/ru/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions) для того, как загрузка CLAUDE.md взаимодействует с параметрами системной подсказки.

<h4 id="settings-precedence">
  Приоритет параметров
</h4>

Когда загружаются несколько источников, параметры объединяются с этим приоритетом (от наивысшего к наименьшему):

1. Локальные параметры (`.claude/settings.local.json`)
2. Параметры проекта (`.claude/settings.json`)
3. Параметры пользователя (`~/.claude/settings.json`)

Программные параметры, такие как `agents`, `allowedTools` и `settings`, переопределяют параметры файловой системы пользователя, проекта и локальные. Параметры управляемой политики имеют приоритет над программными параметрами.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Standard permission behavior
  | "acceptEdits" // Auto-accept file edits
  | "bypassPermissions" // Bypass permission checks; explicit ask rules still prompt
  | "plan" // Planning mode - explore without editing
  | "dontAsk" // Don't prompt for permissions, deny if not pre-approved
  | "auto"; // Model classifier approves or denies permission prompts
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Тип пользовательской функции разрешений для управления использованием инструментов.

Функция — это замена SDK для интерактивного запроса разрешений: она вызывается только когда [поток оценки разрешений](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) разрешается в запрос. Вызовы инструментов, уже одобренные записью `allowedTools`, правилом разрешения параметров или режимом разрешений, таким как `acceptEdits` или `bypassPermissions`, никогда не вызывают его. Чтобы контролировать каждый вызов инструмента, используйте hook [`PreToolUse`](/docs/ru/agent-sdk/hooks) вместо этого.

Правило разрешения не предварительно одобряет [действия, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves); см. [Как оцениваются разрешения](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) для того, какие из них достигают обратного вызова и что происходит в режиме `dontAsk` и `auto`.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    mcpServer?: { name: string; source: string };
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| Параметр         | Тип                                         | Описание                                                                                                                                                                                                                                                                                                                                       |
| :--------------- | :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Сигнализируется, если операция должна быть прервана                                                                                                                                                                                                                                                                                            |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Предложенные обновления разрешений, чтобы пользователь не был запрошен снова для этого инструмента. Подсказки Bash включают предложение с назначением `localSettings` [destination](#permissionupdatedestination), поэтому возврат его в `updatedPermissions` записывает правило в `.claude/settings.local.json` и сохраняется между сеансами. |
| `blockedPath`    | `string`                                    | Путь файла, который вызвал запрос разрешения, если применимо                                                                                                                                                                                                                                                                                   |
| `mcpServer`      | `{ name: string; source: string }`          | Для инструмента `mcp__*`, MCP сервер, который его обслуживает, и откуда определение этого сервера пришло, с полями [`McpServerProvenance`](#mcpserverprovenance). Отсутствует для других инструментов. Требует Agent SDK v0.3.274 или позже                                                                                                    |
| `decisionReason` | `string`                                    | Объясняет, почему был вызван этот запрос разрешения                                                                                                                                                                                                                                                                                            |
| `toolUseID`      | `string`                                    | Уникальный идентификатор для этого конкретного вызова инструмента в сообщении помощника                                                                                                                                                                                                                                                        |
| `agentID`        | `string`                                    | Если работает в подагенте, ID подагента                                                                                                                                                                                                                                                                                                        |
| `requestId`      | `string`                                    | `request_id` оболочки `control_request`. `control_response`, которую ваше приложение отправляет вне SDK, такую как подписанный HTTP POST, должна повторить это значение, чтобы процесс Claude Code мог сопоставить ответ с запросом                                                                                                            |

Обратный вызов обычно разрешает запрос, возвращая [`PermissionResult`](#permissionresult), который SDK записывает обратно через свой транспорт как `control_response`. Возвращайте `null` только когда ваше приложение уже отправило `control_response` для этого запроса через свой собственный канал, повторив `requestId`; SDK затем пропускает запись ответа на свой транспорт. Возврат `null` в любом другом случае оставляет вызов инструмента заблокированным неопределенно, потому что `control_response` никогда не отправляется и запросы разрешений не имеют тайм-аута.

Параметр `requestId` и возвращаемое значение `null` требуют Claude Code v2.1.199 или позже.

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

Конфигурация для поведения встроенного инструмента.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| Поле                            | Тип                    | Описание                                                                                                                                                                                         |
| :------------------------------ | :--------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Включает поле `preview` на параметрах [`AskUserQuestion`](/docs/ru/agent-sdk/user-input#question-format) и устанавливает его формат содержимого. Когда не установлено, Claude не выдает предпросмотры |

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
  timeout?: number;
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

Конфигурация для загрузки плагинов в SDK.

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| Поле               | Тип       | Описание                                                                                                                                                                                                        |
| :----------------- | :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | Должно быть `'local'` (в настоящее время поддерживаются только локальные плагины)                                                                                                                               |
| `path`             | `string`  | Абсолютный или относительный путь к каталогу плагина                                                                                                                                                            |
| `skipMcpDiscovery` | `boolean` | Когда `true`, SDK загружает навыки, hooks, агентов и команды из этого плагина, но не читает его `.mcp.json` или манифест `mcpServers`. Установите это, когда ваше приложение владеет подключениями MCP плагина. |

**Пример:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Для полной информации о создании и использовании плагинов см. [Плагины](/docs/ru/agent-sdk/plugins).

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
  message: BetaMessage; // From Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
  aborted?: true;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Поле `message` — это [`BetaMessage`](https://platform.claude.com/docs/en/api/messages/create) из Anthropic SDK. Оно включает поля, такие как `id`, `content`, `model`, `stop_reason` и `usage`.

`SDKAssistantMessageError` — это одно из следующих значений: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'account_on_hold'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, `'cloud_credential_error'` или `'unknown'`. Четыре из этих значений означают больше, чем говорят их названия:

* `'model_not_found'`: выбранная модель не существует или недоступна для вашей учётной записи или развёртывания
* `'overloaded'`: API вернул 529, потому что сервер работает на полную мощность, в отличие от `'rate_limit'`, который является 429 в отношении вашей квоты
* `'account_on_hold'`: [ваша учётная запись заморожена](/docs/ru/errors#your-account-is-on-hold)
* `'cloud_credential_error'`: Claude Code не смог получить пригодные учётные данные AWS или Google Cloud на машине, на которой он работает, поэтому запрос не достиг поставщика облачных услуг. Обычная причина — истёкший или никогда не завершённый вход в облако на этой машине, хотя временно недоступный сервис учётных данных сообщает то же значение. См. [Could not load AWS or Google Cloud credentials](/docs/ru/errors#could-not-load-aws-or-google-cloud-credentials). Требует TypeScript Agent SDK v0.3.267 или позже, который включает Claude Code v2.1.267

`aborted` имеет значение `true`, когда прерывание или отмена усекли сообщение ассистента до завершения потока: сообщение не имеет `stop_reason` и содержимое может заканчиваться посередине слова. Это поле отсутствует на нормально завершённых сообщениях. Требует Agent SDK v0.3.214 или позже.

Claude Code устанавливает `user_message_uuid` и `user_message_uuids` на первое сообщение ассистента в этом ходу при условиях, описанных в [`user_message_uuid`](#user_message_uuid).

`timestamp` — это время ISO 8601, когда содержимое сообщения закончило генерироваться на процессе, который его создал. Значение поступает с часов этой машины, поэтому используйте его только для отображения и не упорядочивайте сообщения по нему. Один ход API может создать несколько сообщений ассистента, которые имеют одинаковый `message.id`, каждое со своим `timestamp`. Когда это поле отсутствует, используйте время получения сообщения.

`context_usage` — это структурированная копия отчёта `/context`, типизированная как [`SDKContextUsage`](#sdkcontextusage), и требует Agent SDK v0.3.232 или позже. Когда вы отправляете `/context` как подсказку, Claude Code доставляет отчёт как сообщение ассистента, чьё `message.content` содержит таблицу markdown, и прикрепляет `context_usage` к этому же сообщению. Claude Code не устанавливает это поле ни на каком другом сообщении ассистента, и более ранние версии доставляют таблицу `/context` без него, поэтому читайте разбивку из поля, когда оно присутствует, и используйте текст markdown, когда его нет.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

Сообщение пользовательского ввода.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // From Anthropic SDK
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
};
```

Установите `pasted_content` для отправки содержимого, которое пользователь вставил в ваш пользовательский интерфейс подсказки, а не напечатал, одна запись на вставку, каждая — строка или массив блоков содержимого. Claude Code добавляет текст каждой записи после напечатанного текста по порядку и может обернуть каждую вставку в теги `<pasted_content>`. Блоки, отличные от текста, игнорируются, поэтому отправляйте изображения и документы в `message.content`. Требует Agent SDK v0.3.277 или позже.

Установите `shouldQuery` в `false`, чтобы добавить сообщение в стенограмму без запуска хода ассистента. Сообщение удерживается и объединяется со следующим пользовательским сообщением, которое запускает ход. Используйте это для внедрения контекста, такого как вывод команды, которую вы запустили вне полосы, без затрат вызова модели на это.

На сообщении, которое содержит блок `tool_result`, `tool_use_result` — это структурированный объект вывода инструмента, а не текст, отправленный модели. Его форма зависит от инструмента, названного соответствующим блоком `tool_use`, поэтому поле типизировано как `unknown`; встроенные формы перечислены в разделе [Tool Output Types](#tool-output-types).

Для инструмента `Agent` `tool_use_result` — это [`AgentOutput`](#agent-2). На результате `completed` `content` содержит отчёт подагента без ID агента и трейлера использования, который Claude Code добавляет к тексту `tool_result`, поэтому выполняйте рендеринг из `tool_use_result` вместо анализа этого текста.

Для инструмента MCP, чей результат содержит блоки `resource_link`, `tool_use_result` — это объект с массивом `resourceLinks` записей [`SDKMcpResourceLink`](#sdkmcpresourcelink). Claude получает каждую ссылку как строку текста в блоке `tool_result`, поэтому читайте `resourceLinks`, чтобы выполнить рендеринг файлов, которые вернул сервер, вместо анализа этого текста. Claude Code опускает `resourceLinks`, когда результат не содержит ссылок и на результатах от подагентов, сохраняет максимум 50 ссылок на результат и прекращает добавление ссылок, когда массив достигает 64 КиБ сериализованного JSON. `resourceLinks` требует Agent SDK v0.3.257 или позже.

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

Воспроизведённое пользовательское сообщение с обязательным UUID.

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

Пользовательский ход, внедрённый извне сеанса, один, чей [`origin`](#sdkmessageorigin) имеет вид `peer` или `channel`, поступает в поток как воспроизведение, был ли он доставлен во время активного хода или запустил новый ход, пока сеанс был неактивен. До v2.1.207 внедрённый ход, доставленный, пока сеанс был неактивен, не создавал сообщение в потоке и появлялся только при повторном чтении стенограммы.

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
      user_message_uuid?: string;
      user_message_uuids?: string[];
      request_sent_wall_ms?: number;
      first_content_frame_ms?: number;
      first_stream_post_ms?: number;
      first_stream_post_ack_ms?: number;
      first_stream_post_wall_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
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
      queued_turn_count?: number;
      errors: string[];
      startup_failure_reason?: SDKStartupFailureReason;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    };
```

Несколько полей в результате содержат диагностические детали, выходящие за рамки `subtype`:

* `api_error_status`: код состояния HTTP ошибки API, которая завершила разговор. Отсутствует или `null`, когда ход завершился без ошибки API.
* `ttft_ms`: время до первого токена в миллисекундах, измеренное при поступлении первого полного сообщения ассистента. Присутствует только на ветви успеха.
* `ttft_stream_ms`: время в миллисекундах до первого события потока `message_start`, когда открывается поток ответов. Ниже, чем `ttft_ms`; разница между ними — это время, потраченное на потоковую передачу первого сообщения. Присутствует только на ветви успеха.
* `user_message_uuid`: `uuid` сообщения, которое вы отправили и на которое этот ход ответил. См. [`user_message_uuid`](#user_message_uuid) для того, какие результаты его содержат.
* `user_message_uuids`: `uuid`s каждого сообщения, которое вы отправили и на которое Claude Code ответил в этом ходу. См. [`user_message_uuids`](#user_message_uuids).
* `request_sent_wall_ms`: миллисекунды эпохи, в которые Claude Code отправил запрос API, для объединения с серверными временными метками. Присутствует только вместе с [`user_message_uuid`](#user_message_uuid), на результате успеха с `is_error` false, чей ход отправил запрос API.
* `first_content_frame_ms`: время в миллисекундах до первого события потока `content_block_start` или `content_block_delta`, считая блоки мышления как содержимое. Присутствует только на ветви успеха, когда `is_error` имеет значение false. Требует Agent SDK v0.3.260 или позже.
* `first_stream_post_ms`, `first_stream_post_ack_ms`, `first_stream_post_wall_ms`: сроки загрузки первого события потока хода. Claude Code записывает их только в сеансах, которые он потоком передаёт на claude.ai, такие как [облачные сеансы](/docs/ru/claude-code-on-the-web), и результаты, которые выдаёт `query()`, их не содержат. Требует Agent SDK v0.3.260 или позже.
* `usage`: только основной цикл агента. Исключает вызовы подагента и вспомогательной модели и является за ход в сеансах с потоковым вводом. Предпочитайте `modelUsage` для учёта токенов/затрат.
* `modelUsage`: итоги по моделям для каждого вызова модели, сделанного через конвейер запросов во время этого вызова `query()`, включая основной цикл, подагентов и внутренние вызовы, такие как компактирование и агентов Workflow. Вспомогательные вызовы вне этого конвейера, такие как классификатор разрешений и запросы подсчёта токенов, исключены. Вызов, который возобновляет сеанс, также учитывает [итоги по моделям, восстановленные из более ранних вызовов сеанса](/docs/ru/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). В сеансах с потоковым вводом итоги накапливаются по ходам, поэтому читайте последний результат, а не суммируйте по результатам. См. [Track costs in streaming input mode](/docs/ru/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) для сбросов и [Recover totals after a session crash](/docs/ru/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) для обнулённых результатов.
* `total_cost_usd`: кумулятивная предполагаемая стоимость в USD, охватывающая те же вызовы, что и `modelUsage`, и сбрасываемая в тех же точках. Вызов, который возобновляет сеанс, также учитывает [итоги, восстановленные из более ранних вызовов сеанса](/docs/ru/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Это оценка, а не выписка по счёту. См. [Track cost and usage](/docs/ru/agent-sdk/cost-tracking) для предостережений по точности.
* `queued_turn_count`: количество сообщений, которые вы отправили с `origin: { kind: "human" }`, которые всё ещё ожидают, когда Claude Code создал результат. См. [`queued_turn_count`](#queued_turn_count) для того, что говорят вам `0` и отсутствующее поле.
* `startup_failure_reason`: почему Claude Code отказался запускаться, на результате `error_during_execution`, который он записывает перед выходом при известной ошибке запуска. См. [`startup_failure_reason`](#startup_failure_reason) для значений и того, какие ошибки его содержат. Требует Agent SDK v0.3.274 или позже.
* `terminal_reason`: почему цикл завершился. Одно из значений: `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"` или `"turn_setup_failed"`.
* `fast_mode_state`: одно из значений `"on"`, `"off"` или `"cooldown"`.
* `fast_mode_disabled_reason`: почему [fast mode](/docs/ru/fast-mode) недоступен прямо сейчас. Отсутствует, когда ничто не блокирует fast mode, хотя запрос может всё ещё работать на стандартной скорости. Во время охлаждения после ограничения скорости fast mode Claude Code сообщает `fast_mode_state: "cooldown"` без кода причины и повторно включает fast mode, когда охлаждение истекает. Требует Claude Code v2.1.219 или позже.

Используйте код причины, чтобы объяснить, почему fast mode отключён в вашем собственном пользовательском интерфейсе, вместо повторного вывода доступности. Каждый код называет проверку, которая заблокировала fast mode:

| Код причины            | Значение                                                                                                                                          |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `free`                 | Учётная запись не имеет платной подписки или кредитов использования, которые требует fast mode                                                    |
| `preference`           | Организация отключила fast mode                                                                                                                   |
| `extra_usage_disabled` | Кредиты использования отключены для учётной записи                                                                                                |
| `network_error`        | [Проверка доступности](/docs/ru/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) не смогла достичь `api.anthropic.com`                         |
| `unknown`              | Claude Code не смог определить доступность                                                                                                        |
| `not_first_party`      | Сеанс использует поставщика, отличного от Anthropic API                                                                                           |
| `disabled_by_env`      | [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/ru/env-vars) установлен                                                                                        |
| `model_not_allowed`    | Модель Opus fast mode отсутствует в списке разрешений [`availableModels`](/docs/ru/model-config#restrict-model-selection) организации                  |
| `sdk_opt_in_required`  | Сеанс не согласился на fast mode: передайте `fastMode: true` в опции [`settings`](#options) или через [`applyFlagSettings()`](#applyflagsettings) |
| `pending`              | Проверка доступности ещё не завершена                                                                                                             |

Одна и та же пара полей появляется на [`SDKSystemMessage`](#sdksystemmessage) и на [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse), поэтому вы можете прочитать состояние fast mode перед первым ходом.

Поле `origin` пересылает [`SDKMessageOrigin`](#sdkmessageorigin) пользовательского сообщения, которое запустило этот результат. Когда SDK внедряет синтетический ход продолжения, такой как для завершённой фоновой задачи, результирующее `SDKResultMessage` содержит `origin: { kind: "task-notification" }`. Подпрограммы, чей триггер сработал и проверенные сервером сообщения из ваших других сеансов поступают с этим видом, каждое с `subkind`, описанным в [Task-notification subkinds](#task-notification-subkinds). Проверьте `kind`, чтобы различить результаты, которые отвечают на вашу подсказку, от внедрённых продолжений перед маршрутизацией или подавлением их.

Когда несколько завершений фоновой задачи ставятся в очередь вместе, Claude Code может ответить на них в одном ходу, а не в одном ходу каждый. Каждое завершение всё ещё создаёт свой результат с этим происхождением. Все, кроме последнего из завершений, на которые Claude Code отвечает вместе, создают пустые результаты с `num_turns: 0` по порядку, и результат последнего содержит ход, который отвечает на них все.

Это поле отсутствует для результатов, выданных перед любым пользовательским ходом, такие как ошибки запуска.

Когда хук `PreToolUse` возвращает `permissionDecision: "defer"`, результат имеет `stop_reason: "tool_deferred"` и `deferred_tool_use` содержит `id`, `name` и `input` ожидающего инструмента. Читайте это поле, чтобы отобразить запрос в вашем собственном пользовательском интерфейсе, затем возобновите с тем же `session_id`, чтобы продолжить. См. [Defer a tool call for later](/docs/ru/hooks#defer-a-tool-call-for-later) для полного цикла.

<h4 id="user_message_uuid">
  `user_message_uuid`
</h4>

`uuid` [`SDKUserMessage`](#sdkusermessage), на который ход отвечает, повторённый, чтобы вы могли сопоставить ответ Claude Code с сообщением, которое вы отправили. Claude Code повторяет `uuid` только если вы установили его на сообщение. Это поле необязательно на `SDKUserMessage`, и строковая подсказка, переданная в `query()`, не содержит ни одного.

Какое из ваших сообщений отвечает ход, зависит от того, как ход начался:

* **Обычное сообщение, которое вы отправили**, то есть без `isSynthetic: true`: ход отвечает на это сообщение на протяжении всего его выполнения. Когда вы отправляете несколько сообщений близко друг к другу, Claude Code может объединить их в один ход, и поле затем содержит только `uuid` последнего сообщения. Чтобы сопоставить ответ с любым из объединённых сообщений, используйте [`user_message_uuids`](#user_message_uuids).
* **Сообщение, которое вы отправили с `isSynthetic: true`**: ход сначала отвечает на это сообщение. Если Claude Code подхватит обычное сообщение вашего между вызовами инструментов, ход будет отвечать на подхваченное сообщение с этого момента. Повторение `uuid` синтетического сообщения требует Agent SDK v0.3.265 или позже; более ранние версии ничего не повторяют на синтетических ходах.
* **Подсказка, которую Claude Code сгенерировал сам**, такая как ход, который продолжает прерванную работу после перезагрузки сеанса: ход сначала не отвечает ни на какое ваше сообщение и его кадры не содержат повторения. Если Claude Code подхватит обычное сообщение вашего между вызовами инструментов, ход будет отвечать на это сообщение с этого момента. Повторение подхвата требует Agent SDK v0.3.265 или позже; более ранние версии ничего не повторяют на этих ходах.

Claude Code повторяет `uuid` отвеченного сообщения на трёх видах кадра:

* **Результат**: каждый результат хода, который ответил на сообщение, которое вы отправили. Каждый такой результат содержит его на Agent SDK v0.3.265 или позже. До v0.3.265 результат успеха хода, который запустило обычное сообщение, не содержал его, когда ход не отправил запрос API или завершился отложенным вызовом инструмента. До v0.3.246 результаты ошибок также не содержали его, и до v0.3.216 каждый результат не содержал его.
* **Первый ответ хода**: первое [сообщение ассистента](#sdkassistantmessage) или с `includePartialMessages` первое [событие потока](#sdkpartialassistantmessage), чей `event.type` не является `ping`, чтобы вы могли привязать ответ перед поступлением результата. Когда ход ничего не потоком передаёт, Claude Code устанавливает его на первое сообщение ассистента вместо этого. Повторение первого ответа требует Agent SDK v0.3.246 или позже. Когда сообщение, на которое ход отвечает, изменяется в середине хода, первый ответ после изменения также содержит это поле на Agent SDK v0.3.265 или позже; более ранние версии устанавливают его на один кадр ответа за ход.
* **Каждый кадр [`thinking_tokens`](#sdkthinkingtokensmessage) хода**: чтобы вы могли отнести прогресс мышления к сообщению, которое вы отправили, без ожидания первого ответа хода. Требует Agent SDK v0.3.260 или позже.

Claude Code опускает это поле в этих случаях:

* Кадры ответов, отличные от этих первых ответов
* Кадры подагента
* Ходы, которые не отвечают ни на какое сообщение с `uuid`: ход ответил на сообщение, которое вы отправили без одного, или Claude Code запустил ход сам и не подхватил обычное сообщение, которое имеет один
* Результаты, которые не отвечают ни на какое ваше сообщение, такие как обнулённый результат после сбоя рабочего процесса

<h4 id="user_message_uuids">
  `user_message_uuids`
</h4>

`uuid`s каждого сообщения, которое вы отправили и на которое Claude Code ответил в этом ходу. Когда вы отправляете несколько сообщений близко друг к другу, Claude Code может объединить их в один ход, и `user_message_uuid` затем называет только последнее из них. Чтобы сопоставить ответ с любым из объединённых сообщений, ищите `uuid` этого сообщения в любом месте этого списка. Требует Agent SDK v0.3.259 или позже.

Claude Code устанавливает список вместе с `user_message_uuid` на каждом кадре ответа, который содержит это поле, и на результате. Для полного набора кадров, которые содержат `user_message_uuid`, и версии, которую требует каждый, см. [`user_message_uuid`](#user_message_uuid). Список всегда содержит `user_message_uuid` и содержит максимум 64 записи.

Когда Claude Code подхватит обычное сообщение, которое вы отправили, пока ход выполнялся, он добавляет `uuid` этого сообщения в список результата.

Когда первый ответ или результат содержит `user_message_uuid` без списка, он поступил из более ранней версии Claude Code, поэтому используйте одно поле.

<h4 id="queued_turn_count">
  `queued_turn_count`
</h4>

Количество сообщений, которые вы отправили с [`origin: { kind: "human" }`](#sdkmessageorigin), которые всё ещё ожидают в очереди команд, когда Claude Code создал результат. Требует Agent SDK v0.3.242 или позже.

Что говорят вам `0` и отсутствующее поле:

* **`0`**: Claude Code не считает сообщения, которые вы отправили без этого `origin`, и не считает уведомления задач, поэтому ход может всё ещё следовать.
* **Отсутствует**: финальный результат, который Claude Code выдаёт после сбоя или фатальной ошибки запуска, опускает это поле и [может содержать обнулённые итоги](/docs/ru/agent-sdk/cost-tracking#recover-totals-after-a-session-crash).

<h4 id="startup_failure_reason">
  `startup_failure_reason`
</h4>

Почему Claude Code отказался запускаться, чтобы ваше приложение могло предложить исправление вместо повтора. Claude Code устанавливает его на результате `error_during_execution`, который он записывает перед выходом при известной ошибке запуска. Этот результат содержит обнулённые итоги, и его массив `errors` содержит тот же текст, что и stderr. Это поле отсутствует на каждом другом результате. Требует Agent SDK v0.3.274 или позже.

Установите `CLAUDE_CODE_STARTUP_FAILURE_RESULTS` в `1` в [`env`](#options), чтобы получить этот результат для каждого значения `SDKStartupFailureReason`. Без этой переменной Claude Code записывает результат только для этих ошибок, а остальные заканчиваются выводом stderr, ненулевым выходом и без сообщения результата:

* Возобновление, которое Claude Code останавливает, потому что оно [не может вернуть сеанс в его worktree](/docs/ru/worktrees#the-session-resumes-outside-its-worktree), с `worktree_unverified` или `worktree_resume_refused`. Этот раздел говорит, какая ошибка содержит какое значение.
* Отказанное [`continue`](#options) разговора, который фоновый сеанс удерживает, с `session_held_by_background`. Для отказанного [`resume`](#options) такого разговора Claude Code записывает результат только когда переменная установлена.

```typescript theme={null}
type SDKStartupFailureReason =
  | "org_pin_api_key_conflict"
  | "org_verify_failed"
  | "org_pin_mismatch"
  | "managed_settings_invalid"
  | "remote_settings_required_unavailable"
  | "gateway_signin_required"
  | "gateway_access_denied"
  | "proxy_invalid"
  | "temp_dir_unusable"
  | "cwd_unavailable"
  | "shell_tool_missing"
  | "session_held_by_background"
  | "worktree_resume_refused"
  | "worktree_unverified"
  | "cli_version_too_old"
  | "bypass_root";
```

Каждое значение называет один отказ:

| Значение                               | Что остановило сеанс                                                                                                                                                                                                  |
| :------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `org_pin_api_key_conflict`             | Управляемые параметры [требуют вход в первую сторону или облачный шлюз](/docs/ru/authentication#restrict-login-to-your-organization), и вместо этого настроены ключ API Anthropic, токен аутентификации или `apiKeyHelper` |
| `org_verify_failed`                    | Организация входа не смогла быть проверена против булавки, например из-за сбоя сети или отозванного токена                                                                                                            |
| `org_pin_mismatch`                     | Вход принадлежит организации, которую булавка не разрешает                                                                                                                                                            |
| `managed_settings_invalid`             | Управляемые параметры политики не смогли быть прочитаны, или булавка не называет организацию                                                                                                                          |
| `remote_settings_required_unavailable` | Управляемые параметры, которые требует организация, не смогли быть загружены                                                                                                                                          |
| `gateway_signin_required`              | [Облачный шлюз](/docs/ru/claude-apps-gateway) завершил этот вход                                                                                                                                                           |
| `gateway_access_denied`                | Запрос управляемых параметров облачному шлюзу вернулся с 403, который [таблица устранения неполадок](/docs/ru/claude-apps-gateway-deploy#troubleshooting) шлюза охватывает                                                 |
| `proxy_invalid`                        | Параметр прокси не является полным URL                                                                                                                                                                                |
| `temp_dir_unusable`                    | Временный каталог для каждого пользователя небезопасен или не смог быть создан                                                                                                                                        |
| `cwd_unavailable`                      | Рабочий каталог был удалён, перемещён или не может быть прочитан                                                                                                                                                      |
| `shell_tool_missing`                   | На Windows нет доступного инструмента оболочки: Git Bash отсутствует и PowerShell отсутствует или отключён с `CLAUDE_CODE_USE_POWERSHELL_TOOL`                                                                        |
| `session_held_by_background`           | Разговор для возобновления или продолжения работает как [фоновый сеанс](/docs/ru/agent-view)                                                                                                                               |
| `worktree_resume_refused`              | Worktree сеанса не прошёл проверки безопасности, или возобновление было запущено изнутри него. `errors` говорит, продолжится ли запуск того же возобновления без worktree                                             |
| `worktree_unverified`                  | Worktree сеанса не смог быть проверен прямо сейчас, и повтор может быть успешным                                                                                                                                      |
| `cli_version_too_old`                  | Эта версия Claude Code ниже минимума, который требует Anthropic                                                                                                                                                       |
| `bypass_root`                          | Режим разрешений обхода был запрошен при работе от имени root                                                                                                                                                         |

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
    source?: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  terminal_slash_commands?: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  fast_mode_state?: FastModeState;
  fast_mode_disabled_reason?: FastModeDisabledReason;
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | null;
  capabilities?: string[];
};
```

`fast_mode_state` сообщает состояние [fast mode](/docs/ru/fast-mode) сеанса. Когда что-то блокирует fast mode, `fast_mode_disabled_reason` называет проверку, которая его заблокировала; это поле требует Claude Code v2.1.219 или позже. Для кодов причин и их значений см. [`fast_mode_disabled_reason`](#sdkresultmessage) на сообщении результата.

`terminal_slash_commands` называет записи в `slash_commands`, чей интерфейс привязан к локальному терминалу, такие как `exit`. Вы можете отправлять их как любую другую запись в `slash_commands`; это поле существует, чтобы удалённый или мобильный клиент мог скрыть их из своих меню команд. Это поле присутствует только когда оно не пусто, и требует Agent SDK v0.3.229 или позже.

*

`source` на каждой записи `mcp_servers`: откуда поступило определение сервера, с теми же значениями, что и `source` [`McpServerStatus`](#mcpserverstatus). Требует Agent SDK v0.3.274 или позже.

*

`effort`: [уровень усилий](/docs/ru/model-config#adjust-effort-level), который Claude Code отправляет на следующий запрос сеанса, или `null`, когда он не отправляет ни один. Claude Code устанавливает это поле только на сообщение инициализации, которое оно отправляет клиентам [Remote Control](/docs/ru/remote-control), и опускает его из сообщения инициализации, которое читает ваше приложение. Требует Agent SDK v0.3.234 или позже.

Массив `capabilities` называет поведения протокола, которые реализует этот CLI, чтобы вы могли обнаруживать функции вместо сравнения строк `claude_code_version`. Это открытый набор: игнорируйте значения, которые вы не узнаёте, и проверяйте конкретную возможность, поведение которой вы используете. Это поле требует Claude Code v2.1.205 или позже и отсутствует на более ранних CLI.

| Возможность                  | Значение                                                                                                                                                                                                                                                                                                      |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1`       | [`interrupt()`](#query-object) разрешается с помощью [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse), квитанции, в которой перечислены сообщения, которые были в ожидании при поступлении прерывания                                                                                            |
| `interrupt_cancel_queued_v1` | Запрос управления `interrupt` соблюдает `cancel_queued: true`, отменяя сообщения, которые квитанция в противном случае перечислила бы в `still_queued`, и перечисляя их в `cancelled` вместо этого. См. [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse). Требует Claude Code v2.1.219 или позже |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

Потоковое частичное сообщение (только когда `includePartialMessages` имеет значение true). Поле `parent_tool_use_id` всегда `null`: события потока выдаются только для основного сеанса. Для атрибуции подагента используйте полные сообщения, которые содержат `parent_tool_use_id`, или включите [`forwardSubagentText`](#options), чтобы получать текст и мышление подагента как полные сообщения.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // From Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Time to first token in ms, present only on message_start events
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Claude Code устанавливает `user_message_uuid` и `user_message_uuids` на первое событие потока без ping хода и снова, когда сообщение, на которое ход отвечает, изменяется, при условиях в [`user_message_uuid`](#user_message_uuid).

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

Сообщение, указывающее границу компактирования разговора.

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

Общий текстовый баннер, выданный циклом. Содержит строки статуса без ошибок, обратную связь хука, такую как причина блокировки хука `UserPromptSubmit`, и вывод команды. На Claude Code v2.1.227 или позже [`systemMessage`](/docs/ru/hooks#json-output) хука может поступить как это сообщение, с каждой строкой с префиксом имени хука, такой как `PostToolUse:Bash says:`. Поступает ли `systemMessage` хука как это сообщение, зависит от события. Каждый [раздел события](/docs/ru/hooks#hook-events) на странице hooks говорит, как выводится вывод. Выполняйте рендеринг `content` как простого текста на заданном `level`.

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

Выданное при корректном завершении рабочего процесса, чтобы удалённые клиенты могли показать, почему рабочий процесс вышел, вместо ожидания истечения времени ожидания сердцебиения. `reason` — это короткая строка snake\_case, установленная хостом CLI, такая как `"host_exit"` или `"remote_control_disabled"`. Действуйте на это только при потоковой передаче в реальном времени. Возобновленный сеанс воспроизводит прошлые экземпляры этого сообщения, поэтому игнорируйте их в этом случае.

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

Событие прогресса установки плагина. Выданное когда [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ru/env-vars) установлен, чтобы ваше приложение Agent SDK могло отслеживать установку плагина marketplace перед первым ходом. Статусы `started` и `completed` заключают общую установку. Статусы `installed` и `failed` сообщают об отдельных marketplaces и включают `name`.

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

Событие потока, выданное когда система разрешений отказывает в вызове инструмента без интерактивного приглашения. Используйте его для отображения отказа в вашем пользовательском интерфейсе по мере его возникновения, а не только наблюдая результат инструмента `is_error`, который следует. Какие отказы оно сообщает, зависит от того, как запуск обрабатывает приглашения разрешений:

* **С обратным вызовом [`canUseTool`](#canusetool) и значением по умолчанию [`permissionPrompts: 'host'`](#options)**: приглашения разрешений идут в ваш обратный вызов, и это событие сообщает об отказах, которые Claude Code решает самостоятельно без его вызова.
*

**Без ни одного**: голый запуск `-p` или `query()`, который не устанавливает ни `canUseTool`, ни `permissionPromptToolName`, отказывает любому вызову инструмента, который бы подсказал, и это событие сообщает об этих отказах, а также об отказах, которые Claude Code решает самостоятельно. До v2.1.223 Claude Code не выдавал это событие в запусках без обратного вызова.

* **С инструментом приглашения MCP**, установленным с `permissionPromptToolName` или флагом [`--permission-prompt-tool`](/docs/ru/cli-reference#cli-flags), и значением по умолчанию `permissionPrompts: 'host'`: Claude Code вообще не выдаёт это событие, даже для отказов правила, которые он решает самостоятельно.
*

**С [`permissionPrompts: 'none'`](#options)**: Claude Code отказывает вызовам, которые бы подсказали, даже когда также установлены `canUseTool` или инструмент приглашения MCP, и это событие сообщает об этих отказах, а также об отказах, которые Claude Code решает самостоятельно. Требует Claude Code v2.1.259 или позже.

В каждой конфигурации это событие пропускает любой отказ, решённый на пути хука `PreToolUse`, отказал ли сам хук вызову или правило отрицания переопределило решение хука разрешить или спросить. Это событие также является лучшим усилием: иногда Claude Code записывает отказ без выдачи этого события, поэтому `permission_denials` на [сообщении результата](#sdkresultmessage) является авторитетным записью.

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

| Поле                   | Тип      | Описание                                                                                                                            |
| ---------------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `tool_name`            | `string` | Имя инструмента, который был отказан                                                                                                |
| `tool_use_id`          | `string` | ID блока `tool_use`, на который этот отказ отвечает                                                                                 |
| `agent_id`             | `string` | ID подагента, когда отказанный вызов возник внутри подагента. Зеркалирует поле на `can_use_tool` для маршрутизации на стороне хоста |
| `decision_reason_type` | `string` | Дискриминатор для компонента, который решил, такой как `"rule"`, `"mode"`, `"classifier"` или `"asyncAgent"`                        |
| `decision_reason`      | `string` | Понятная причина от решающего компонента, когда доступна                                                                            |
| `message`              | `string` | Сообщение об отказе, возвращённое модели в `tool_result`                                                                            |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

Информация об отказанном использовании инструмента.

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkcontextusage">
  `SDKContextUsage`
</h3>

Структурированная форма отчёта `/context`, переносимая как `context_usage` на [`SDKAssistantMessage`](#sdkassistantmessage), который доставляет результат `/context`. Agent SDK v0.3.232 и позже экспортируют тип. В отличие от [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse), он содержит только данные, необходимые для отображения разбивки использования, без полей отображения, таких как `color` и `gridRows`.

```typescript theme={null}
type SDKContextUsage = {
  model: string;
  total_tokens: number;
  raw_max_tokens: number;
  percentage: number;
  over_limit?: {
    tokens_over: number;
    kind: "hard_limit" | "compaction_window";
  };
  categories: SDKContextUsageCategory[];
  mcp_tools: {
    name: string;
    server_name: string;
    tokens: number;
  }[];
  memory_files: {
    path: string;
    type: string;
    tokens: number;
  }[];
  agents: {
    agent_type: string;
    source: string;
    tokens: number;
  }[];
  skills?: {
    name: string;
    source: string;
    plugin_name?: string;
    tokens: number;
  }[];
};
```

Таблица перечисляет, что Claude Code помещает в каждое поле. Поля от `model` до `over_limit` описывают сеанс в целом, и поля коллекции приписывают токены отдельным элементам.

| Поле             | Тип                                                       | Описание                                                                                                                                                                                                                                                                                                      |
| ---------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model`          | `string`                                                  | Модель основного цикла, для которой Claude Code вычислил использование, а не подагента                                                                                                                                                                                                                        |
| `total_tokens`   | `number`                                                  | Оценка Claude Code токенов в использовании. Не зажата в окно, поэтому может превышать `raw_max_tokens`, когда сеанс превышает лимит                                                                                                                                                                           |
| `raw_max_tokens` | `number`                                                  | Контекстное окно модели или нижнее [окно auto-compact](/docs/ru/model-config#context-window-and-auto-compaction), когда оно применяется, такое как установленное вами или граница 200K, которую Claude Code применяет к некоторым моделям с окном 1M-токена. Claude Code измеряет `total_tokens` против этого окна |
| `percentage`     | `number`                                                  | `total_tokens` как округлённый процент `raw_max_tokens`, поэтому может превышать 100, когда сеанс превышает лимит                                                                                                                                                                                             |
| `over_limit`     | `object`                                                  | Присутствует только когда `total_tokens` превышает `raw_max_tokens`. `tokens_over` — это количество превышения, и `kind` говорит, как Claude Code разрешил окно                                                                                                                                               |
| `categories`     | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | Одна запись на строку разбивки использования по категориям                                                                                                                                                                                                                                                    |
| `mcp_tools`      | `object[]`                                                | Токены, приписанные каждому инструменту MCP, с его проводным именем, таким как `mcp__linear__create_issue`, и его `server_name`                                                                                                                                                                               |
| `memory_files`   | `object[]`                                                | Токены, приписанные каждому загруженному файлу памяти, с его `path` и меткой источника, такой как `Project` или `User` в `type`                                                                                                                                                                               |
| `agents`         | `object[]`                                                | Токены, приписанные каждому определению пользовательского подагента, с идентификатором источника, таким как `projectSettings`, `userSettings` или `plugin`. Встроенные подагенты не перечислены                                                                                                               |
| `skills`         | `object[]`                                                | Токены, приписанные каждому навыку в списке навыков, с идентификатором источника и, для навыков плагина, именем плагина в `plugin_name`. Отсутствует, когда никакие навыки не вносят токены                                                                                                                   |

`over_limit.kind` записывает, как Claude Code разрешил окно, а не принимает ли API следующий запрос:

* `hard_limit`: окно — это то, что Claude Code считает собственным лимитом модели, за пределами которого API отказывает запросы
* `compaction_window`: окно — это окно политики компактирования, которое может совпадать или не совпадать с лимитом модели

Claude Code развивает тип аддитивно, добавляя новые данные как необязательные поля, а не переформатируя существующие. Читайте поля, которые вы знаете, и игнорируйте те, которые вы не узнаёте.

<h3 id="sdkcontextusagecategory">
  `SDKContextUsageCategory`
</h3>

Одна строка разбивки использования `/context` по категориям.

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

Таблица перечисляет, что Claude Code помещает в каждое поле строки.

| Поле     | Тип      | Описание                                                                                                               |
| -------- | -------- | ---------------------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | Имя отображения строки, как печатает `/context`, такое как `Messages`. Классифицируйте строки по `kind`, а не по имени |
| `tokens` | `number` | Количество токенов строки. Строки могут содержать нулевые токены                                                       |
| `kind`   | `string` | Что представляет строка: `used`, `free`, `buffer` или `deferred`                                                       |

Каждое значение `kind` говорит, что представляют собой токены строки:

* `used`: содержимое, которое занимает контекстное окно
* `free`: оставшееся окно
* `buffer`: резерв компактирования
* `deferred`: схемы инструментов, которые Claude Code удерживает вне окна и исключает из расчёта использования, перечисленные для осведомления

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Происхождение сообщения с ролью пользователя. Это появляется как `origin` на [`SDKUserMessage`](#sdkusermessage) и пересылается на соответствующее [`SDKResultMessage`](#sdkresultmessage), чтобы вы могли сказать, что запустило данный ход.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      fromMode?: "bypass" | "prompting";
      name?: string;
      fromSession?: string;
      senderTaskId?: string;
      body?: string;
      verifiedPeerPid?: number;
    }
  | {
      kind: "task-notification";
      subkind?: "scheduled-trigger" | "peer-send-message";
    }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" }
  | { kind: "unclassified" };
```

| `kind`              | Значение                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `human`             | Прямой ввод от конечного пользователя. Если ваше приложение пересылает то, что пользователь напечатал, как пользовательское сообщение, установите его `origin` в `{ kind: "human" }` явно: Claude Code рассматривает пользовательское сообщение без `origin` как неатрибутированное и проверяет, что требуют подсказку, введённую человеком, такие как ключевое слово [`ultracode` workflow](/docs/ru/workflows#ask-for-a-workflow-in-your-prompt), не принимают её. До v2.1.210 Claude Code рассматривал отсутствующий `origin` на пользовательском сообщении как пользовательский ввод. |
| `channel`           | Сообщение, поступающее на [канал](/docs/ru/channels). `server` — это имя исходного сервера MCP.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `peer`              | Сообщение от другого агента: внутрипроцессный [товарищ по команде](/docs/ru/agent-teams) или [кросс-сеансный одноранговый](/docs/ru/cross-session-messaging), другой из ваших сеансов Claude Code. См. [Peer origin fields](#peer-origin-fields) для семантики каждого поля и модели доверия.                                                                                                                                                                                                                                                                                                  |
| `task-notification` | Синтетический ход, внедрённый для доставки, которая поступает без свежей подсказки пользователя, такой как завершённая фоновая задача; см. [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) для этого варианта. Необязательный `subkind` отмечает, что вызвало уведомление. См. [Task-notification subkinds](#task-notification-subkinds).                                                                                                                                                                                                                                |
| `coordinator`       | Сообщение от координатора команды в [команде агентов](/docs/ru/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `auto-continuation` | Синтетический ход, внедрённый когда сеанс продолжается без свежего пользовательского ввода, такой как результат команды, который запускает подсказку продолжения.                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `unclassified`      | Внедрённый ход, чьё происхождение не смогло быть определено. Требует Claude Code v2.1.223 или позже. Когда Claude Code получает [`SDKUserMessage`](#sdkusermessage) с `isSynthetic: true` и не может классифицировать его как любой другой `kind`, он устанавливает этот вид по мере поступления сообщения и кадрирует ход модели как источник, отличный от пользователя, а не рассматривает его как пользовательский ввод. Ваше приложение не должно устанавливать это значение.                                                                                                    |

<h3 id="task-notification-subkinds">
  Task-notification subkinds
</h3>

Когда Claude Code доставляет уведомление задачи в сеанс, он устанавливает `subkind` на `origin` уведомления только если серверы Anthropic проверили, откуда поступило это уведомление. `subkind` требует Claude Code v2.1.213 или позже и принимает одно из двух значений:

* `scheduled-trigger`: уведомление — это сохранённая подсказка [подпрограммы](/docs/ru/routines), доставленная, потому что один из триггеров подпрограммы сработал: её расписание, её [API триггер](/docs/ru/routines#add-an-api-trigger), её [GitHub триггер](/docs/ru/routines#add-a-github-trigger) или **Run now**. Claude Code кадрирует их модели как назначенную задачу сеанса, с другим уведомлением от [уведомления, которое несут другие уведомления задач](#sdktasknotificationmessage).
*

`peer-send-message`: уведомление — это сообщение, которое другой из ваших сеансов отправил с инструментом `send_message` на стороне сервера, который используют [облачные сеансы](/docs/ru/claude-code-on-the-web) для обмена сообщениями друг с другом, а не [кросс-сеансный инструмент `SendMessage`](/docs/ru/cross-session-messaging), и серверы Anthropic проверили, что оба сеанса принадлежат одной и той же приватной группе сеансов. Требует Claude Code v2.1.224 или позже. Доставка `send_message`, которую серверы не проверили таким образом, не получает `subkind`.

Каждое другое уведомление задачи не имеет `subkind`. Это включает [запланированные задачи](/docs/ru/scheduled-tasks), которые срабатывают на вашей собственной машине, [активность PR](/docs/ru/claude-code-on-the-web#how-claude-responds-to-pr-activity), доставленную в сеанс, и фоновые события, такие как завершённая задача. Сообщения от [кросс-сеансного инструмента `SendMessage`](/docs/ru/cross-session-messaging) вообще не являются уведомлениями задач: поступают ли они из сеанса на той же машине или через серверы Anthropic с другой машины, Claude Code даёт им `kind: "peer"` и [поля происхождения одноранговой сети](#peer-origin-fields).

<h3 id="peer-origin-fields">
  Peer origin fields
</h3>

Происхождение `peer` идентифицирует, какой агент отправил сообщение: внутрипроцессный [товарищ по команде](/docs/ru/agent-teams), отправляющий в `main` с `SendMessage`, или [кросс-сеансный одноранговый](/docs/ru/cross-session-messaging), другой из ваших сеансов Claude Code. Кросс-сеансные одноранговые требуют Claude Code v2.1.224 или позже на macOS и Linux; см. [доступность кросс-сеансного обмена сообщениями](/docs/ru/cross-session-messaging#availability) для требования собственного Windows. Кросс-сеансный одноранговый может работать на той же машине или на [другой из ваших машин](/docs/ru/cross-session-messaging#message-sessions-on-other-machines) или [в облаке](/docs/ru/claude-code-on-the-web), когда его сообщение поступает через Remote Control. Два вида отправителя заполняют поля по-разному:

* `from`: имя товарища по команде или адрес отправителя для кросс-сеансного одноранговой. Для [одностороннего кросс-машинного сообщения](/docs/ru/cross-session-messaging#message-sessions-on-other-machines) отправитель не имеет адреса ответа и `from` — это `"unknown"`. Значение создано отправителем; `verifiedPeerPid` — это проверенная личность.
*

`fromMode`: класс разрешений отправляющего сеанса, `bypass` или `prompting`, объявленный хостом, который передаёт одноранговое сообщение между вашими сеансами, такой как [настольное приложение](/docs/ru/desktop#work-across-sessions). Claude Code читает его в получающем сеансе, когда применяет [входящие элементы управления](/docs/ru/cross-session-messaging#control-inbound-messages). Требует Agent SDK v0.3.234 или позже.

* `senderTaskId`: ID задачи товарища по команде. Отсутствует для кросс-сеансного одноранговой.
*

`name`: отображаемое имя отправителя, нормализованное Claude Code: оно удаляет управление Unicode, формат, суррогат и разделители строк или абзацев, затем обрезает результат и ограничивает его 64 кодовыми точками с многоточием. Требует Claude Code v2.1.205 или позже.

*

`body`: декодированное тело сообщения с удалённой оболочкой одноранговой сети, байт-точное с тем, что видит модель. Всегда присутствует для сообщения товарища по команде; для кросс-сеансного одноранговой, присутствует только когда ход — это ровно одна оболочка одноранговой сети, сформированная Claude Code. Выполняйте рендеринг `name` и `body` вместо повторного анализа текста сообщения. Требует Claude Code v2.1.205 или позже.

*

`fromSession`: ID сеанса отправителя, открываемый хостом, установленный хостом отправителя, чтобы ваш пользовательский интерфейс мог ссылаться обратно на отправляющий сеанс. Как `from`, это утверждение отправителя: используйте его только как цель навигации и не рассматривайте его как доказательство личности отправителя. Требует Claude Code v2.1.216 или позже.

*

`verifiedPeerPid`: ID процесса процесса, который подключился к сокету кросс-сеансного обмена сообщениями этого сеанса, проверенный ядром и прочитанный из самого соединения, никогда из полезной нагрузки. Используйте его, а не `from`, чтобы идентифицировать отправителя: `from` может быть подделан любым процессом того же пользователя. Это поле отсутствует, когда Claude Code не может его проверить, такой как на Windows или неокончательный ввод, поэтому отсутствующее значение означает, что отправитель не проверен. Для передаваемого трафика он идентифицирует реле, а не автора сообщения, и ID процессов перерабатываются, поэтому рассматривайте его как происхождение, а не как токен аутентификации. Требует Claude Code v2.1.216 или позже.

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
  | "UserPromptExpansion"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "StopFailure"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PostCompact"
  | "PreModelSwitch"
  | "PostModelSwitch"
  | "PermissionRequest"
  | "PermissionDenied"
  | "Setup"
  | "TeammateIdle"
  | "TaskCreated"
  | "TaskCompleted"
  | "Elicitation"
  | "ElicitationResult"
  | "ConfigChange"
  | "DirectoryAdded"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "InstructionsLoaded"
  | "CwdChanged"
  | "FileChanged"
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
  | PermissionDeniedHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | UserPromptExpansionHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | StopFailureHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PostCompactHookInput
  | PreModelSwitchHookInput
  | PostModelSwitchHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCreatedHookInput
  | TaskCompletedHookInput
  | ElicitationHookInput
  | ElicitationResultHookInput
  | ConfigChangeHookInput
  | InstructionsLoadedHookInput
  | DirectoryAddedHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | CwdChangedHookInput
  | FileChangedHookInput
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
  mcp_server?: McpServerProvenance;
};
```

`mcp_server` присутствует, когда инструмент поступает с сервера MCP; см. [`McpServerProvenance`](#mcpserverprovenance). Входные данные `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` и `PermissionDenied` содержат то же поле. Это поле требует Agent SDK v0.3.274 или позже.

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
  mcp_server?: McpServerProvenance;
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
  mcp_server?: McpServerProvenance;
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

<h4 id="permissiondeniedhookinput">
  `PermissionDeniedHookInput`
</h4>

```typescript theme={null}
type PermissionDeniedHookInput = BaseHookInput & {
  hook_event_name: "PermissionDenied";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  reason: string;
  mcp_server?: McpServerProvenance;
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
  session_title?: string;
};
```

<h4 id="userpromptexpansionhookinput">
  `UserPromptExpansionHookInput`
</h4>

```typescript theme={null}
type UserPromptExpansionHookInput = BaseHookInput & {
  hook_event_name: "UserPromptExpansion";
  expansion_type: "slash_command" | "mcp_prompt";
  command_name: string;
  command_args: string;
  command_source?: string;
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact" | "fork";
  agent_type?: string;
  model?: string;
  session_title?: string;
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

<h4 id="stopfailurehookinput">
  `StopFailureHookInput`
</h4>

```typescript theme={null}
type StopFailureHookInput = BaseHookInput & {
  hook_event_name: "StopFailure";
  error: SDKAssistantMessageError;
  error_details?: string;
  last_assistant_message?: string;
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

<h4 id="postcompacthookinput">
  `PostCompactHookInput`
</h4>

```typescript theme={null}
type PostCompactHookInput = BaseHookInput & {
  hook_event_name: "PostCompact";
  trigger: "manual" | "auto";
  compact_summary: string;
};
```

<h4 id="premodelswitchhookinput">
  `PreModelSwitchHookInput`
</h4>

Срабатывает перед тем, как запрошенное переключение модели вступит в силу. `context_tokens` и поля после него оценивают, какие затраты на повторную отправку разговора новой модели. Для полного описания полей и семантики блокирования см. [PreModelSwitch](/docs/ru/hooks#premodelswitch).

```typescript theme={null}
type PreModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PreModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="postmodelswitchhookinput">
  `PostModelSwitchHookInput`
</h4>

Срабатывает после изменения модели сессии. Он содержит те же поля, что и `PreModelSwitchHookInput`, с двумя дополнительными значениями `source`. См. [PostModelSwitch](/docs/ru/hooks#postmodelswitch).

```typescript theme={null}
type PostModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PostModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk" | "auto" | "resume";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
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
  mcp_server?: McpServerProvenance;
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

<h4 id="taskcreatedhookinput">
  `TaskCreatedHookInput`
</h4>

```typescript theme={null}
type TaskCreatedHookInput = BaseHookInput & {
  hook_event_name: "TaskCreated";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated с версии v2.1.178. Содержит имя команды, полученное из сессии; будет удалено. */
  team_name?: string;
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

<h4 id="elicitationhookinput">
  `ElicitationHookInput`
</h4>

```typescript theme={null}
type ElicitationHookInput = BaseHookInput & {
  hook_event_name: "Elicitation";
  mcp_server_name: string;
  message: string;
  mode?: "form" | "url";
  url?: string;
  elicitation_id?: string;
  requested_schema?: Record<string, unknown>;
};
```

<h4 id="elicitationresulthookinput">
  `ElicitationResultHookInput`
</h4>

```typescript theme={null}
type ElicitationResultHookInput = BaseHookInput & {
  hook_event_name: "ElicitationResult";
  mcp_server_name: string;
  elicitation_id?: string;
  mode?: "form" | "url";
  action: "accept" | "decline" | "cancel";
  content?: Record<string, unknown>;
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

<h4 id="instructionsloadedhookinput">
  `InstructionsLoadedHookInput`
</h4>

```typescript theme={null}
type InstructionsLoadedHookInput = BaseHookInput & {
  hook_event_name: "InstructionsLoaded";
  file_path: string;
  memory_type: "User" | "Project" | "Local" | "Managed";
  load_reason:
    | "session_start"
    | "nested_traversal"
    | "path_glob_match"
    | "include"
    | "compact";
  globs?: string[];
  trigger_file_path?: string;
  parent_file_path?: string;
};
```

<h4 id="directoryaddedhookinput">
  `DirectoryAddedHookInput`
</h4>

```typescript theme={null}
type DirectoryAddedHookInput = BaseHookInput & {
  hook_event_name: "DirectoryAdded";
  directory: string;
  source: "slash_command" | "register_repo_root";
};
```

`directory` — это абсолютный путь к добавленной директории. `source` — это `"slash_command"`, когда `/add-dir` добавил её, и `"register_repo_root"`, когда это сделал запрос управления SDK.

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

<h4 id="cwdchangedhookinput">
  `CwdChangedHookInput`
</h4>

```typescript theme={null}
type CwdChangedHookInput = BaseHookInput & {
  hook_event_name: "CwdChanged";
  old_cwd: string;
  new_cwd: string;
};
```

<h4 id="filechangedhookinput">
  `FileChangedHookInput`
</h4>

```typescript theme={null}
type FileChangedHookInput = BaseHookInput & {
  hook_event_name: "FileChanged";
  file_path: string;
  event: "change" | "add" | "unlink";
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
  /**
   * Терминальная escape-последовательность (например OSC 9 / OSC 777 desktop-notification)
   * для Claude Code, которую нужно выдать от вашего имени. Разрешены только notification/title OSCs
   * (0, 1, 2, 9, 99, 777) и BEL; значение, содержащее что-либо ещё, игнорируется целиком. Только интерактивный CLI
   * выдаёт его; SDK игнорирует это поле.
   */
  terminalSequence?: string;
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
        sessionTitle?: string;
        /** Когда decision — "block", опустите исходный запрос из сообщения блокировки. */
        suppressOriginalPrompt?: boolean;
      }
    | {
        hookEventName: "UserPromptExpansion";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
        initialUserMessage?: string;
        sessionTitle?: string;
        watchPaths?: string[];
        /**
         * Повторно сканируйте директории skills и commands после завершения hooks SessionStart,
         * чтобы skills, установленные hook, были доступны в той же сессии.
         */
        reloadSkills?: boolean;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "PreModelSwitch";
        /**
         * Тот же контракт, что и PreToolUse: "allow" продолжает, "deny" отменяет
         * переключение, "ask" просит пользователя подтвердить. Только /model в
         * интерактивной сессии показывает этот запрос; каждая другая поверхность,
         * включая set_model запросы, рассматривает "ask" как отказ.
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** Достигает модели со следующим запросом, который обслуживает новая модель. */
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        /**
         * Краткая заметка о результате этого вызова инструмента для классификатора
         * разрешений в автоматическом режиме. Ограничена 2000 символами, общая для
         * всех hooks, которые отвечают на один и тот же вызов; учитывается только при
         * синхронных ответах hook. Не копируйте ненадёжный вывод инструмента в неё.
         */
        classifierContext?: string;
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
        hookEventName: "Stop";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStop";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionDenied";
        retry?: boolean;
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
      }
    | {
        hookEventName: "Elicitation";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "ElicitationResult";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "CwdChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "FileChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "WorktreeCreate";
        worktreePath: string;
      }
    | {
        hookEventName: "MessageDisplay";
        /** Текст, отображаемый вместо delta. Опустите (или верните delta без изменений), чтобы отобразить исходный. */
        displayContent?: string;
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

Объединение типов входных данных tool, экспортируемое из `@anthropic-ai/claude-agent-sdk`; члены включают:

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | ArtifactInput
  | AskUserQuestionInput
  | BashInput
  | CronCreateInput
  | CronDeleteInput
  | CronListInput
  | EnterPlanModeInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | ExitWorktreeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ProjectsInput
  | PushNotificationInput
  | ReadMcpResourceDirInput
  | ReadMcpResourceInput
  | RefreshMcpToolsInput
  | RemoteTriggerInput
  | ReportFindingsInput
  | ScheduleWakeupInput
  | ShowOnboardingRolePickerInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Имя tool:** `Agent`. Предыдущее имя `Task` всё ещё принимается как псевдоним, и массив `tools` в сообщении инициализации [`SDKSystemMessage`](#sdksystemmessage) в настоящее время перечисляет этот tool как `Task` для обратной совместимости.

<Note>
  Поле `mode` устарело и игнорируется в Claude Code v2.1.212 или позже. Подагент работает либо в режиме разрешений родительской сессии, либо в режиме его определения [`permissionMode`](#agentdefinition), и [правила наследования подагента](/docs/ru/agent-sdk/permissions#available-modes) решают, какой из них.
</Note>

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // Устарело; игнорируется
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // Устарело; игнорируется. Правила наследования подагента решают режим разрешений подагента
  isolation?: "worktree" | "remote";
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
  answers?: Record<string, string>;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  metadata?: { source?: string };
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

Выполняет команды Bash с опциональным timeout и фоновым выполнением. Рабочий каталог сохраняется между командами, включая команды, запущенные в более поздних ходах многоходовой сессии; состояние shell, такое как экспортированные переменные окружения, не сохраняется. Для ограничений на то, какие изменения каталога переносятся, см. [Что сохраняется между командами](/docs/ru/tools-reference#what-persists-between-commands).

<h3 id="monitor">
  Monitor
</h3>

**Имя tool:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  description: string;
  timeout_ms: number;
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
};
```

Запускает фоновый источник и доставляет каждое событие к Claude, чтобы он мог реагировать без опроса: `command` запускает скрипт и выдаёт одно событие на строку stdout, а `ws` открывает WebSocket и выдаёт одно событие на текстовый фрейм. Укажите ровно один из `command` или `ws`. Источник `ws` требует Claude Code v2.1.195 или позже.

`timeout_ms` — это крайний срок наблюдения в миллисекундах. По умолчанию он равен 300000 и принимает значения до 3600000. Эффективный крайний срок составляет максимум 1800000, что составляет 30 минут, поэтому большее принятое значение сокращается до этого. В крайний срок наблюдение заканчивается и Claude получает одно уведомление, чтобы он мог начать новое наблюдение, если оно ему всё ещё нужно.

Экспортированный тип отмечает `timeout_ms` как обязательный, потому что схема заполняет значение по умолчанию; вызов, который его опускает, проходит валидацию.

Когда Monitor запускает команду, он следует тем же правилам разрешения, что и Bash; наблюдение WebSocket запрашивает одобрение отдельно. См. [справочник tool Monitor](/docs/ru/tools-reference#monitor-tool) для поведения и доступности провайдера.

<h3 id="taskoutput">
  TaskOutput
</h3>

Удалено в Claude Code v2.1.277 вместе с его типом `TaskOutputInput`. Ранее получало вывод из выполняющейся или завершённой фоновой задачи; Claude читает выходной файл фоновой задачи с помощью `Read` вместо этого.

Запись `disallowedTools` или правило отказа, которое всё ещё называет `TaskOutput`, игнорируется без предупреждения.

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

Для PDF Claude получает содержимое файла внутри `tool_result` вызова Read. Чтение, которое возвращает выход `pdf` [output](#tool-output-types), содержит блок сводки `text`, за которым следует блок `document`. Чтение, которое возвращает выход `parts`, содержит блок сводки `text`, за которым следует один блок на извлечённую страницу: блок `image` или блок `text`, называющий страницу, когда Claude Code не смог отобразить её как изображение. До Agent SDK v0.3.242 Claude Code доставлял содержимое файла как отдельное сообщение `user` после результата tool.

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
  "-o"?: boolean; // print only the matched parts of each line; requires output_mode: "content"
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
  args?: unknown; // any JSON value; the published typings render this as an object map
  resumeFromRunId?: string;
  title?: string; // ignored; the script's meta block sets the title
  description?: string; // ignored; the script's meta block sets the description
};
```

Запускает [динамический workflow](/docs/ru/workflows): скрипт, который организует множество подагентов в фоне и возвращает один консолидированный результат. Tool `Workflow` доступен в Agent SDK v0.3.149 и позже. Требуется хотя бы один из `script`, `name` или `scriptPath`.

| Поле              | Тип       | Описание                                                                                                                                                                                                                                                                                                                                                    |
| ----------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Встроенный скрипт workflow. Должен начинаться с `export const meta = { name, description }` как литерал, за которым следует тело скрипта с использованием `agent()`, `parallel()`, `pipeline()` и `phase()`. Опциональный массив `phases` в `meta` группирует агентов под названными этапами в представлении прогресса                                      |
| `name`            | `string`  | Имя встроенного workflow или сохранённого в `.claude/workflows/`. Разрешается в скрипт                                                                                                                                                                                                                                                                      |
| `scriptPath`      | `string`  | Путь к файлу скрипта workflow на диске. Имеет приоритет над `script` и `name`. Claude Code сохраняет скрипт каждого вызова и возвращает путь в результате, поэтому вы можете отредактировать этот файл и повторно вызвать с тем же `scriptPath` для итерации                                                                                                |
| `args`            | `unknown` | Входное значение, доступное скрипту как глобальная переменная `args`, для параметризованных именованных workflows, таких как исследовательский вопрос или список путей файлов. Передавайте массивы и объекты как фактические значения JSON, а не как JSON-кодированную строку                                                                               |
| `resumeFromRunId` | `string`  | Run ID предыдущего вызова `Workflow` для возобновления. Завершённые вызовы `agent()` с неизменёнными входными данными обычно возвращают кэшированные результаты; остальные выполняются в реальном времени. [Возобновление после паузы](/docs/ru/workflows#resume-after-a-pause) охватывает, какие завершённые вызовы повторно выполняются. Только в одной сессии |
| `title`           | `string`  | Игнорируется; блок `meta` скрипта устанавливает заголовок                                                                                                                                                                                                                                                                                                   |
| `description`     | `string`  | Игнорируется; блок `meta` скрипта устанавливает описание                                                                                                                                                                                                                                                                                                    |

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
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  См. [Доступность модели](/docs/ru/agent-sdk/todo-tracking#model-availability) для подключения.
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
  [k: string]: unknown;
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

<h3 id="exitworktree">
  ExitWorktree
</h3>

**Имя tool:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

Выходит из текущего git worktree и возвращается в исходный рабочий каталог. Действие `keep` оставляет worktree и ветку на диске, а `remove` удаляет оба. `discard_changes` должно быть `true` при удалении worktree, который имеет незафиксированные файлы или неслитые коммиты.

<h3 id="enterplanmode">
  EnterPlanMode
</h3>

**Имя tool:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

Входит в режим планирования, где Claude исследует и представляет план перед внесением изменений.

<h3 id="croncreate">
  CronCreate
</h3>

**Имя tool:** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

Планирует запуск запроса по расписанию cron из 5 полей в локальном времени. Установите `recurring` на `false` для срабатывания один раз при следующем совпадении. Задачи по умолчанию ограничены сессией: запуск новой беседы очищает их, а возобновление с `--resume` или `--continue` восстанавливает задачи, которые не истекли. См. [Запланированные задачи](/docs/ru/scheduled-tasks).

Установка `durable` на `true` запрашивает сохранение в `.claude/scheduled_tasks.json`, чтобы задача пережила перезагрузки. Долговечное планирование доступно не в каждой сессии: когда оно недоступно, Claude Code принимает `durable: true`, но создаёт задачу только для сессии. Прочитайте поле `durable` в выводе, чтобы увидеть, сохранилась ли задача.

<h3 id="crondelete">
  CronDelete
</h3>

**Имя tool:** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

Удаляет запланированную задачу cron по ID, возвращённому из `CronCreate`.

<h3 id="cronlist">
  CronList
</h3>

**Имя tool:** `CronList`

```typescript theme={null}
type CronListInput = {};
```

Перечисляет запланированные задачи cron: долговечные задачи из `.claude/scheduled_tasks.json` и задачи только для сессии из текущей сессии.

<h3 id="schedulewakeup">
  ScheduleWakeup
</h3>

**Имя tool:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

Планирует одноразовое пробуждение, которое срабатывает заданный запрос после задержки. Этот tool поддерживает команду `/loop` с собственным темпом. Среда выполнения зажимает `delaySeconds` между 60 и 3600 секундами. Поля `delaySeconds`, `reason`, `prompt` и `noop` обязательны, если `stop` не true. `noop: true` сообщает о пробуждении, где ничего не изменилось. Установка `stop: true` отменяет ожидающее пробуждение и завершает самостоятельный `/loop`. Поле `stop` требует Claude Code v2.1.202 или позже. См. [строку ScheduleWakeup в справочнике tools](/docs/ru/tools-reference).

<h3 id="remotetrigger">
  RemoteTrigger
</h3>

**Имя tool:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerInput = {
  action:
    | "list"
    | "get"
    | "create"
    | "update"
    | "run"
    | "create_webhook_trigger"
    | "list_runs"
    | "get_run_log";
  trigger_id?: string;
  session_id?: string;
  cursor?: string;
  body?: {
    [k: string]: unknown;
  };
};
```

Управляет [Routines](/docs/ru/routines), запланированными и активируемыми запусками Claude Code, размещёнными в облаке. Этот tool поддерживает команду `/schedule`. `trigger_id` обязателен для действий `get`, `update`, `run` и `list_runs`. `body` обязателен для `create`, `update` и `create_webhook_trigger`, и опционален для `run`.

`create_webhook_trigger` присоединяет источник событий к существующей routine, такой как [событие GitHub](/docs/ru/routines#add-a-github-trigger), которое её срабатывает. `body` называет источник, события и routine для срабатывания. Требует Claude Code v2.1.225 или позже.

`list_runs` перечисляет недавние запуски routine, а `get_run_log` читает журнал одного запуска. `session_id` называет запуск для чтения из результата `list_runs`, а `cursor` разбивает результаты любого действия на страницы. Оба действия требуют Claude Code v2.1.227 или позже.

Этот tool доступен только когда сессия аутентифицирована с учётной записью claude.ai на плане с включённой функцией Routines, и отсутствует, когда политика вашей организации отключает [Claude Code в веб](/docs/ru/claude-code-on-the-web). В Claude Code v2.1.227 или позже tool также отсутствует, когда Owner [отключил routines для организации](/docs/ru/routines#routines-are-disabled-by-your-organizations-policy). До v2.1.227 сессия с отключённым только переключателем routines всё ещё показывала tool, и сервер отклонял его вызовы.

<h3 id="pushnotification">
  PushNotification
</h3>

**Имя tool:** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

Отправляет проактивное push-уведомление пользователю. Держите `message` под 200 символами, потому что мобильные операционные системы обрезают более длинный текст. См. [строку PushNotification в справочнике tools](/docs/ru/tools-reference) для доступности провайдера; доставка push проходит через инфраструктуру, размещённую Anthropic, которая недоступна из Amazon Bedrock, Claude Platform на AWS, Google Cloud's Agent Platform или Microsoft Foundry.

<h3 id="repl">
  REPL
</h3>

Удалено в v2.1.275. До v2.1.274 экспериментальный tool `REPL` можно было включить с помощью `CLAUDE_CODE_REPL=1` в опции [`env`](#options).

<h3 id="reportfindings">
  ReportFindings
</h3>

**Имя tool:** `ReportFindings`

```typescript theme={null}
type ReportFindingsInput = {
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Сообщает о результатах проверки кода как структурированный список, чтобы Claude Code мог их отобразить вместо вывода их как текст. `level` — это уровень усилий, на котором выполнялась проверка. Результаты упорядочены от наиболее серьёзных, максимум 32 на вызов, и массив пуст, когда ничего не выжило. Требует Claude Code v2.1.196 или позже.

Каждый результат содержит эти поля:

* `file`: путь, относительный к репозиторию, в котором находится результат. Опциональный `line` — это 1-индексированная строка, к которой он привязан.
* `summary`: однострочное утверждение дефекта. `failure_scenario` описывает конкретные входные данные и состояние, которые приводят к неправильному выводу или сбою.
* `short_summary`: опциональный сжатый ярлык максимум 60 символов для компактного отображения. Требует Claude Code v2.1.212 или позже.
* `category`: опциональный короткий kebab-case слаг типа результата, такой как `correctness` или `test-coverage`. Требует Claude Code v2.1.199 или позже.
* `verdict`: устанавливается, когда выполнялся проход проверки; отсутствует при встроенных проверках.
* `outcome`: устанавливается только при повторном сообщении после применения исправлений.

<h3 id="artifact">
  Artifact
</h3>

**Имя tool:** `Artifact`

```typescript theme={null}
type ArtifactInput = {
  action?: "publish" | "list";
  file_path?: string;
  favicon?: string;
  icon?: string;
  limit?: number;
  scope?: "mine" | "shared" | "all";
  title?: string;
  description?: string;
  label?: string;
  url?: string;
  force?: boolean;
  capabilities?: Record<string, unknown>;
  contract?: "latest" | string;
};
```

Публикует локальный файл `.html` или `.md` как размещённую страницу артефакта или перечисляет опубликованные артефакты пользователя. Опустите `action` или передайте `"publish"` для публикации `file_path`, который обязателен для действия публикации. Каждое поле ниже применяется к публикации:

* `icon`: одно короткое общее слово для значка артефакта на вкладке браузера, такое как `chart` или `map`. Claude включает его при первой публикации и опускает при обновлении, что сохраняет сохранённый значок артефакта.
* `favicon`: устарело, и Claude его опускает.
* `title`: называет опубликованную страницу на вкладке браузера и в галерее, когда HTML файл не имеет тега `<title>`.
* `url`: нацеливается на существующий артефакт для обновления на месте вместо создания нового.

`force` — это последняя мера перезаписи, которая отбрасывает более новую версию, опубликованную другой сессией. При конфликте неудачная публикация возвращает более новое содержимое; Claude объединяет его изменения с этим содержимым или повторно читает артефакт и публикует снова. Передавайте `force` только, когда пользователь явно просит отбросить эту версию.

Передайте `"list"` для перечисления опубликованных артефактов пользователя; только `limit` и `scope` могут его сопровождать. `scope` по умолчанию `"mine"`, который перечисляет артефакты, которыми владеет пользователь; `"shared"` перечисляет артефакты, которыми другие люди поделились с пользователем, и `"all"` перечисляет оба.

* `capabilities`: возможности среды выполнения, которые использует опубликованная страница, ключ по названию возможности, такой как [коннекторы, которые может вызывать страница](/docs/ru/artifacts#pull-live-data-with-mcp-connectors). Сервис артефактов проверяет объявление и отклоняет публикацию, которая называет возможность, которую учётная запись не может использовать, или даёт ей недействительную конфигурацию. Передайте `{}` для очистки сохранённого объявления и опустите поле при повторном развёртывании, чтобы сохранить его. Требует Agent SDK v0.3.235 или позже.
* `contract`: версия среды выполнения, на которой работает опубликованная страница. Опустите её, чтобы сохранить текущую версию артефакта, передайте `"latest"` для обновления или передайте конкретную версию для закрепления или отката. Требует Agent SDK v0.3.235 или позже.

Типы экспортируются, но tool отключён по умолчанию в сессиях Agent SDK. Публикация также требует каждого условия в [таблице доступности артефактов](/docs/ru/artifacts#availability), которые сессии, аутентифицированные с помощью ключа API, не удовлетворяют.

<h3 id="projects">
  Projects
</h3>

**Имя tool:** `Projects`

```typescript theme={null}
type ProjectsInput = {
  method:
    | "project_info"
    | "project_read"
    | "project_search"
    | "project_write"
    | "project_delete";
  path?: string;
  content?: string;
  local_path?: string;
  present_to_user?: boolean;
  query?: string;
  n?: number;
};
```

Читает и записывает Project claude.ai, присоединённый к сессии. Отправляет по `method`:

* `project_info`: возвращает метаданные проекта и список документов.
* `project_read`: читает один документ по `path`.
* `project_search`: запрашивает базу знаний проекта с помощью `query`. `n` ограничивает совпадения и по умолчанию равно 5.
* `project_write`: создаёт или заменяет документ по `path` из ровно одного из `content`, который содержит встроенный текст, или `local_path`, который называет файл внутри рабочего каталога. `present_to_user: true` отмечает написанный документ как доставляемый результат, который пользователю нужно увидеть.
* `project_delete`: удаляет документ по `path`.

<h3 id="readmcpresourcedir">
  ReadMcpResourceDir
</h3>

**Имя tool:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

Перечисляет прямых потомков ресурса каталога на сервере MCP. Используется только для сервера, который объявил поддержку перечисления каталогов; перечисление не является рекурсивным. Перечисление каталогов не включено в каждой сессии: когда оно отключено, вызов возвращает пустой список `resources` и поле `error` сообщает, что перечисление каталогов не включено.

<h3 id="refreshmcptools">
  RefreshMcpTools
</h3>

**Имя tool:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // refresh only this server; omit to refresh all connected servers
};
```

Повторно запрашивает список tools подключённых серверов MCP и применяет любые изменения. Типы экспортируются, но Claude Code регистрирует tool только, когда вы установите `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1` в опции [`env`](#options), и только в сессиях с хотя бы одним сервером MCP. Требует Claude Code v2.1.211 или позже.

<h3 id="showonboardingrolepicker">
  ShowOnboardingRolePicker
</h3>

**Имя tool:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

Отображает кликабельную строку выбора ролей во время адаптации Cowork, чтобы пользователь мог выбрать свою роль и получить установленный соответствующий плагин. Не принимает аргументы; список ролей определяется клиентом. Вызов блокируется до ответа пользователя.

<h3 id="mcpinput">
  McpInput
</h3>

**Имя tool:** динамические имена tools MCP формы `mcp__<server>__<tool>`

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

Аргументы tools MCP — это открытый объект: каждый сервер определяет свои собственные параметры, поэтому тип не накладывает ограничений на имена полей или значения. Обратитесь к собственной схеме tools сервера для полей, которые принимает конкретный tool.

<h2 id="tool-output-types">
  Типы выходных данных инструментов
</h2>

Документация схем выходных данных для всех встроенных инструментов Claude Code. Эти типы экспортируются из `@anthropic-ai/claude-agent-sdk` и представляют фактические данные ответа, возвращаемые каждым инструментом.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Объединение типов выходных данных инструментов, экспортируемых из `@anthropic-ai/claude-agent-sdk`; члены включают:

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | ArtifactOutput
  | AskUserQuestionOutput
  | BashOutput
  | CronCreateOutput
  | CronDeleteOutput
  | CronListOutput
  | EnterPlanModeOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | ExitWorktreeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | McpOutput
  | MonitorOutput
  | NotebookEditOutput
  | ProjectsOutput
  | PushNotificationOutput
  | ReadMcpResourceDirOutput
  | ReadMcpResourceOutput
  | RefreshMcpToolsOutput
  | RemoteTriggerOutput
  | ReportFindingsOutput
  | ScheduleWakeupOutput
  | ShowOnboardingRolePickerOutput
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

**Имя инструмента:** `Agent`. Предыдущее имя `Task` по-прежнему принимается как псевдоним, и массив `tools` в сообщении инициализации [`SDKSystemMessage`](#sdksystemmessage) в настоящее время перечисляет этот инструмент как `Task` для обратной совместимости.

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      modelsUsed?: string[];
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
        output_tokens_details?: {
          thinking_tokens?: number | null;
        } | null;
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
      modelsUsed?: string[];
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

Возвращает результат от подагента. Различается по полю `status`: `"completed"` для завершённых задач, `"async_launched"` для фоновых задач и `"remote_launched"` для задач, которые Claude Code отправил в облачный сеанс, где `sessionUrl` ссылается на этот сеанс и `taskId` его идентифицирует.

На варианте `completed` `resolvedModel` называет модель, на которой подагент начал работу, которая может отличаться от запрошенного входного параметра `model`, когда применяется [`availableModels`](/docs/ru/model-config#restrict-model-selection) или другое переопределение. Это поле требует Claude Code v2.1.174 или более поздней версии. На `async_launched` оно называет модель, используемую при переводе задачи в фоновый режим.

`modelsUsed` перечисляет модели, которые использовал подагент, по порядку. Поле присутствует только при смене модели во время выполнения, и модель появляется снова при возврате к ней. На `async_launched` список охватывает модели, используемые до перевода в фоновый режим. Как `modelsUsed`, так и поведение фонового режима `resolvedModel` требуют Claude Code v2.1.212 или более поздней версии.

Если Claude Code [сохранил изолированное рабочее дерево подагента](/docs/ru/worktrees#isolate-subagents-with-worktrees), `worktreePath` в результате `completed` указывает, где его найти. `worktreeBranch` — это его ветка, присутствующая, когда Claude Code создал рабочее дерево с git.

Claude Code заполняет `usage` и `totalTokens` из финального запроса API подагента, а не из всего выполнения, поэтому `usage.service_tier` — это строка уровня обслуживания, которую API сообщила в этом запросе. Если присутствует, `usage.output_tokens_details.thinking_tokens` — это количество токенов вывода этого запроса, которые были токенами мышления. Поле `output_tokens_details` требует TypeScript SDK v0.3.228 или более поздней версии, которая поставляется с Claude Code v2.1.228.

`usage.output_tokens_details` соответствует [`Usage.output_tokens_details`](#usage) по смыслу, ограниченному этим финальным запросом, но каждый уровень здесь является необязательным. Проверьте как объект, так и поле, например `usage.output_tokens_details?.thinking_tokens ?? 0`, вместо прямого чтения.

До v2.1.207 опубликованный тип был более узким. Он опускал `worktreePath`, `worktreeBranch`, `citations`, `toolStats.frameCount` и поля использования `inference_geo`, `speed` и `iterations`, и он типизировал `service_tier` как `"standard" | "priority" | "batch"`. Поля, которые тип отмечает как необязательные, могут отсутствовать в результатах, записанных более ранними версиями.

<h3 id="askuserquestion-2">
  AskUserQuestion
</h3>

**Имя инструмента:** `AskUserQuestion`

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
  annotations?: Record<string, { preview?: string; notes?: string }>;
  afkTimeoutMs?: number;
};
```

Возвращает заданные вопросы и ответы пользователя. `response` устанавливается, когда пользователь ввёл свободный ответ вместо ответа на структурированные вопросы; если присутствует, Claude получает «Пользователь ответил: …» вместо списка ответов по вопросам.

<h3 id="bash-2">
  Bash
</h3>

**Имя инструмента:** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  timedOutAfterMs?: number;
  backgroundCwdHint?: string;
  backgroundEndsWithFinalResponse?: true;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  noOutputExpected?: boolean;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
  staleReadFileStateHint?: string;
  ghRateLimitHint?: string;
  gitOperation?: {
    commit?: { sha: string; kind: "committed" | "amended" | "cherry-picked"; branch?: string };
    push?: { branch: string };
    branch?: { ref: string; action: "merged" | "rebased" };
    pr?: {
      number: number;
      url?: string;
      action: "created" | "edited" | "merged" | "commented" | "closed" | "reopened" | "ready" | "draft" | "auto-merge-enabled" | "auto-merge-disabled";
    };
  };
};
```

Поля `stdout`, `stderr` и `backgroundTaskId` содержат:

| Поле               | Что оно содержит                                                                                               |
| ------------------ | -------------------------------------------------------------------------------------------------------------- |
| `stdout`           | Stdout и stderr команды, объединённые в один чередующийся поток                                                |
| `stderr`           | Уведомления, которые добавляет сам инструмент, такие как сброс рабочего каталога оболочки, а не stderr команды |
| `backgroundTaskId` | Присутствует для фоновых команд                                                                                |

`timedOutAfterMs` — это тайм-аут в миллисекундах, установленный, когда команда достигла своего тайм-аута и перешла в фоновый режим, а не начала там явно. `backgroundCwdHint` устанавливается, когда фоновая команда содержала встроенную команду изменения каталога, такую как `cd`, `pushd`, `popd` или `chdir`, и отмечает, что рабочий каталог сеанса не изменился. Оба поля требуют Claude Code v2.1.210 или более поздней версии.

Когда подагент, работающий на переднем плане, владеет фоновой командой, Claude Code завершает команду, когда этот подагент даёт свой финальный ответ. Claude Code устанавливает `backgroundEndsWithFinalResponse` в `true` для таких команд и опускает поле, когда команда сохраняется после хода, как команды, запущенные основным разговором или фоновыми подагентами. Поле требует Claude Code v2.1.227 или более поздней версии.

Claude Code устанавливает `gitOperation.commit.branch` в ветку, названную в строке сводки коммита git, и опускает её для коммита, сделанного на отсоединённой HEAD. Поле требует Agent SDK v0.3.227 или более поздней версии. Claude Code сообщает команду `gh pr reopen` как действие PR `reopened`, что требует Agent SDK v0.3.234 или более поздней версии.

<h3 id="monitor-2">
  Monitor
</h3>

**Имя инструмента:** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

Возвращает ID фоновой задачи для работающего монитора. Используйте этот ID с `TaskStop` для отмены наблюдения раньше.

<h3 id="edit-2">
  Edit
</h3>

**Имя инструмента:** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string | null;
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
    repository?: string | null;
  };
};
```

Возвращает структурированный diff операции редактирования.

<h3 id="read-2">
  Read
</h3>

**Имя инструмента:** `Read`

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
        /** True when a whole-file read was auto-paginated because it exceeded the token cap (the content is a partial first page). */
        truncatedByTokenCap?: boolean;
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
      /** Document page number of the first extracted page; labels the page images in the tool_result content. */
      firstPage?: number;
      /** In-process only: the page-image bytes are delivered as image blocks in the tool_result content and aren't retained on the emitted tool_use_result, so this key is absent there. */
      pages?: {
        base64: string;
        mediaType: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        error?: string;
      }[];
    }
  | {
      type: "file_unchanged";
      file: {
        filePath: string;
      };
      /** Set when the dedup matched a startup-seeded entry (CLAUDE.md / nested memory) rather than a prior Read tool_result. */
      source?: "seeded";
    };
```

Возвращает содержимое файла в формате, подходящем для типа файла. Различается по полю `type`.

<h3 id="write-2">
  Write
</h3>

**Имя инструмента:** `Write`

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
    repository?: string | null;
  };
  userModified?: boolean;
};
```

Возвращает результат записи с информацией структурированного diff. То, что содержат `originalFile` и `structuredPatch`, зависит от записи:

* Для вновь созданного файла `originalFile` равен null и `structuredPatch` пуст
* При перезаписи `originalFile` содержит предыдущее содержимое, за исключением случаев, когда это содержимое больше примерно 10 МБ: Claude Code затем пропускает diff и возвращает `originalFile` null и `structuredPatch` пуст
* `structuredPatch` также пуст, когда запись ничего не изменила или diff истёк по времени

<h3 id="glob-2">
  Glob
</h3>

**Имя инструмента:** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
  totalMatches?: number;
  countIsComplete?: boolean;
};
```

Возвращает пути файлов, соответствующие шаблону glob, отсортированные по времени изменения.

`totalMatches` и `countIsComplete` требуют Claude Code v2.1.191 или более поздней версии. `totalMatches` сообщает количество совпадающих файлов до усечения. Когда `countIsComplete` равен false, `totalMatches` является нижней границей, потому что базовый поиск усёк свой собственный вывод.

<h3 id="grep-2">
  Grep
</h3>

**Имя инструмента:** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  totalFiles?: number;
  totalLines?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Возвращает результаты поиска. Форма варьируется в зависимости от `mode`: список файлов, содержимое с совпадениями или подсчёты совпадений. В режиме `count` `numFiles` и `numMatches` — это итоги по полному набору результатов, а не по разбитому на страницы срезу. До v2.1.208 `head_limit` или `offset`, который усекал перечисленные записи, также усекал эти итоги.

`totalFiles` требует Claude Code v2.1.208 или более поздней версии и сообщает общее количество результатов до разбиения на страницы `head_limit` и `offset` в режиме `files_with_matches`. `totalLines` требует Claude Code v2.1.210 или более поздней версии и сообщает общее количество строк до разбиения на страницы в режиме `content`.

<h3 id="taskstop-2">
  TaskStop
</h3>

**Имя инструмента:** `TaskStop`

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

**Имя инструмента:** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  old_source?: string;
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

Возвращает результат редактирования ноутбука с исходным и обновлённым содержимым файла.

<h3 id="webfetch-2">
  WebFetch
</h3>

**Имя инструмента:** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
  artifactRead?: {
    slug: string;
    ver?: string;
    seeded?: false;
  };
};
```

Возвращает полученное содержимое со статусом HTTP и метаданными.

`artifactRead` — это собственная запись Claude Code о чтении артефакта, присутствующая только когда Claude получил артефакт, который сеанс может опубликовать. Claude Code читает его обратно, когда сеанс возобновляется, чтобы более поздняя публикация строилась на правильной версии; ваш код не должен действовать на основе этого. `slug` называет артефакт, `ver` — это версия, которую чтение записало, и отсутствует, когда оно ничего не записало, и `seeded: false` отмечает чтение, полный источник которого не достиг Claude. Поле `seeded` требует Agent SDK v0.3.239 или более поздней версии.

<h3 id="websearch-2">
  WebSearch
</h3>

**Имя инструмента:** `WebSearch`

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
  searchCount?: number;
};
```

Возвращает результаты поиска из веб-сети.

<h3 id="workflow-2">
  Workflow
</h3>

**Имя инструмента:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched" | "remote_launched";
  taskId: string;
  taskType?: "local_workflow" | "remote_agent";
  workflowName?: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  sessionUrl?: string; // set when the workflow launched as a cloud session
  warning?: string;
  error?: string;
};
```

Возвращается сразу после того, как инструмент принимает вызов. Финальный результат приходит позже как завершение задачи. Проверьте `error` перед тем, как рассматривать выполнение как начатое: скрипт, который не проходит проверку синтаксиса, возвращает `status: "async_launched"` с установленным `error` и никогда не выполняется.

| Поле            | Тип                                     | Описание                                                                                                                                                                                                  |
| --------------- | --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched" \| "remote_launched"` | Инструмент принял вызов. `"async_launched"` для выполнений в процессе, `"remote_launched"` для выполнений, отправленных в облачный сеанс вместо выполнения в процессе                                     |
| `taskId`        | `string`                                | Идентификатор фоновой задачи для выполнения                                                                                                                                                               |
| `taskType`      | `"local_workflow" \| "remote_agent"`    | Тип задачи зарегистрированной фоновой задачи, соответствующий ветке `status`                                                                                                                              |
| `workflowName`  | `string`                                | `meta.name` из скрипта workflow                                                                                                                                                                           |
| `runId`         | `string`                                | Идентификатор выполнения workflow для передачи как `resumeFromRunId` при более позднем вызове. Отсутствует для выполнений `remote_launched`, где URL облачного сеанса является дескриптором возобновления |
| `summary`       | `string`                                | Однострочное описание того, что делает workflow                                                                                                                                                           |
| `transcriptDir` | `string`                                | Каталог, где записываются стенограммы подагентов во время выполнения                                                                                                                                      |
| `scriptPath`    | `string`                                | Путь к сохранённому скрипту workflow для этого выполнения. Отредактируйте его и передайте обратно как `scriptPath` для повторного выполнения без повторной отправки скрипта                               |
| `sessionUrl`    | `string`                                | URL облачного сеанса, установленный, когда `status` равен `"remote_launched"`                                                                                                                             |
| `warning`       | `string`                                | Неблокирующее предупреждение, такое как локальное состояние git, отличающееся от отправленной ветки, которую будет клонировать облачный сеанс                                                             |
| `error`         | `string`                                | Установлено, когда скрипт не проходит проверку синтаксиса. Если присутствует, выполнение не началось несмотря на статус запуска                                                                           |

<h3 id="todowrite-2">
  TodoWrite
</h3>

**Имя инструмента:** `TodoWrite`

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
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Смотрите [Доступность модели](/docs/ru/agent-sdk/todo-tracking#model-availability) для подключения.
</Note>

<h3 id="taskcreate-2">
  TaskCreate
</h3>

**Имя инструмента:** `TaskCreate`

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

**Имя инструмента:** `TaskUpdate`

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

**Имя инструмента:** `TaskGet`

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

Возвращает полную запись задачи или `null`, когда ID не найден.

<h3 id="tasklist-2">
  TaskList
</h3>

**Имя инструмента:** `TaskList`

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

**Имя инструмента:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  planWasEdited?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

Возвращает состояние плана после выхода из режима плана.

<h3 id="listmcpresources-2">
  ListMcpResources
</h3>

**Имя инструмента:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

Возвращает массив доступных ресурсов MCP.

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**Имя инструмента:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
    blobSavedTo?: string;
  }>;
  error?: string;
};
```

Возвращает содержимое запрошенного ресурса MCP.

<h3 id="enterworktree-2">
  EnterWorktree
</h3>

**Имя инструмента:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

Возвращает информацию о git рабочем дереве.

<h3 id="exitworktree-2">
  ExitWorktree
</h3>

**Имя инструмента:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeOutput = {
  action: "keep" | "remove";
  originalCwd: string;
  worktreePath: string;
  worktreeBranch?: string;
  tmuxSessionName?: string;
  discardedFiles?: number;
  discardedCommits?: number;
  message: string;
};
```

Возвращает выполненное действие и детали о рабочем дереве, из которого был выход.

<h3 id="enterplanmode-2">
  EnterPlanMode
</h3>

**Имя инструмента:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

Возвращает подтверждение того, что режим плана был активирован.

<h3 id="croncreate-2">
  CronCreate
</h3>

**Имя инструмента:** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // true when persisted to .claude/scheduled_tasks.json; false when session-only
};
```

Возвращает ID задачи и понятное для человека описание расписания.

<h3 id="crondelete-2">
  CronDelete
</h3>

**Имя инструмента:** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

Возвращает ID удалённой задачи.

<h3 id="cronlist-2">
  CronList
</h3>

**Имя инструмента:** `CronList`

```typescript theme={null}
type CronListOutput = {
  jobs: {
    id: string;
    cron: string;
    humanSchedule: string;
    prompt: string;
    recurring?: boolean;
    durable?: boolean;
  }[];
};
```

Возвращает запланированные cron задачи: долговечные задачи из `.claude/scheduled_tasks.json` и задачи только для сеанса из текущего сеанса. Задача только для сеанса содержит `durable: false`; задачи, прочитанные с диска, опускают это поле.

<h3 id="schedulewakeup-2">
  ScheduleWakeup
</h3>

**Имя инструмента:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

Возвращает, когда пробуждение сработает как временная метка эпохи в миллисекундах, фактически использованную задержку и была ли запрошенная задержка ограничена. Поле `stopped` равно `true`, когда вызов завершил цикл с `stop: true`. Это требует Claude Code v2.1.202 или более поздней версии. Поле `cancelledWakeups` подсчитывает, сколько ожидающих пробуждений отменил вызов `stop: true`. Значение 0 означает, что ничего не было в ожидании, и повторяющийся `/loop` cron не отменяется `stop: true`. Это требует Claude Code v2.1.206 или более поздней версии.

<h3 id="remotetrigger-2">
  RemoteTrigger
</h3>

**Имя инструмента:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

Возвращает статус ответа API и тело для операции триггера.

<h3 id="pushnotification-2">
  PushNotification
</h3>

**Имя инструмента:** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

Возвращает детали доставки, включая отправлено ли push или локальное уведомление и почему доставка была пропущена.

<h3 id="reportfindings-2">
  ReportFindings
</h3>

**Имя инструмента:** `ReportFindings`

```typescript theme={null}
type ReportFindingsOutput = {
  count: number;
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Возвращает количество сообщённых находок, уровень усилий, на котором выполнялся обзор, и находки, повторённые обратно для тела результата. Требует Claude Code v2.1.196 или более поздней версии. Повторённое поле `short_summary` требует Claude Code v2.1.212 или более поздней версии.

<h3 id="artifact-2">
  Artifact
</h3>

**Имя инструмента:** `Artifact`

```typescript theme={null}
type ArtifactOutput =
  | {
      url: string;
      path: string;
      title?: string;
      version?: string;
      capabilities?: unknown;
      stored?: {
        contract: string;
        capabilities?: Record<string, unknown>;
      };
      warnings?: string[];
      contract?: string;
      updated?: boolean;
      liveSubscription?: string;
    }
  | {
      artifacts: Array<{
        title: string;
        url: string;
        updatedAt?: string;
        rel?: "mine" | "shared";
      }>;
      truncated?: boolean;
      scope?: "shared" | "all";
    };
```

Возвращает `url` опубликованной страницы и локальный `path`, который был опубликован для действия публикации, с `updated`, установленным в true, когда публикация переразвернула существующий артефакт, и `warnings`, содержащие любые рекомендации времени публикации. Действие списка возвращает строки `artifacts` вместо этого, с `truncated`, установленным, когда существует больше артефактов, чем запрошенный лимит. На списках, область которых не `"mine"`, каждая строка содержит `rel`, отмечающий, владеет ли пользователь артефактом или он был с ними поделён, и `scope` вывода записывает, какая область, отличная от стандартной, произвела список; оба отсутствуют на стандартных списках.

<h3 id="projects-2">
  Projects
</h3>

**Имя инструмента:** `Projects`

```typescript theme={null}
type ProjectsOutput =
  | {
      method: "project_info";
      notice?: string;
      name: string;
      description: string;
      instructions: string;
      docs: Array<{ path: string; created_at: string | null }>;
      files?: Array<{
        path: string;
        file_kind: string;
        created_at: string | null;
      }>;
      sync_sources?: Array<{
        type: string | null;
        config: Record<string, unknown>;
      }>;
      knowledge: {
        knowledge_size: number;
        max_knowledge_size: number;
      };
    }
  | {
      method: "project_read";
      notice?: string;
      path: string;
      file_kind?: string;
      content?: string;
      local_file?: string;
      created_at: string | null;
    }
  | {
      method: "project_search";
      notice?: string;
      rag: boolean;
      hits?: Array<{ name?: string; doc_uuid?: string; text?: string }>;
      docs?: string[];
    }
  | {
      method: "project_write";
      notice?: string;
      path: string;
      doc_uuid: string;
      replaced: boolean;
      present_to_user?: boolean;
      local_path?: string;
    }
  | {
      method: "project_delete";
      notice?: string;
      path: string;
      deleted: boolean;
    };
```

Различается по полю `method`, отражая входные данные. `project_read` возвращает небольшие текстовые документы встроенными в `content` и записывает более крупные документы в путь `local_file` вместо этого; `project_search` возвращает RAG `hits` с `rag: true`, когда индекс проекта доступен, и откатывается на список пути `docs` в противном случае.

<h3 id="readmcpresourcedir-2">
  ReadMcpResourceDir
</h3>

**Имя инструмента:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirOutput = {
  resources: Array<{
    uri: string;
    name: string;
    mimeType?: string;
  }>;
  error?: string;
};
```

Возвращает прямых потомков ресурса каталога. Подкаталоги появляются с mimeType `"inode/directory"`; `error` содержит понятное для человека сообщение, когда сервер не смог перечислить каталог.

<h3 id="refreshmcptools-2">
  RefreshMcpTools
</h3>

**Имя инструмента:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // tools now available from this server
  added?: string[]; // tool names this refresh added
  removed?: string[]; // tool names this refresh removed
  error?: string; // why the refresh failed or the server was unavailable
}>;
```

Возвращает одну запись на сервер: `refreshed` означает, что переопрошенный список инструментов был применён, `error` означает, что переопрос не удался и предыдущий набор инструментов был сохранён, и `not_connected` означает, что сервер не имеет активного соединения для опроса.

<h3 id="showonboardingrolepicker-2">
  ShowOnboardingRolePicker
</h3>

**Имя инструмента:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

Возвращает выбор пользователя: `role`, когда они выбрали чип роли или ввели один, и `dismissed: true`, когда они закрыли средство выбора. Пустой объект означает, что пользователь одобрил вызов без выбора роли.

<h3 id="mcpoutput">
  McpOutput
</h3>

**Имя инструмента:** динамические имена инструментов MCP вида `mcp__<server>__<tool>`

```typescript theme={null}
type McpOutput =
  | string
  | {
      type: string;
      [k: string]: unknown;
    }[]
  | {
      [k: string]: unknown;
    };
```

Результаты инструментов MCP возвращаются как строка или массив блоков содержимого, в зависимости от сервера. Конечная ветка простого объекта в экспортируемом типе — это артефакт генерации схемы: SDK не возвращает голый объект, потому что структурированный вывод сервера сериализуется в строку JSON перед возвратом. Во время выполнения значение также может быть `undefined`, хотя экспортируемый тип это не моделирует.

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

Источник API ключа для запросов сессии, сообщаемый как `apiKeySource` в инициализирующем сообщении [`SDKSystemMessage`](#sdksystemmessage).

```typescript theme={null}
type ApiKeySource =
  | "ANTHROPIC_API_KEY"
  | "apiKeyHelper"
  | "/login managed key"
  | "none"
  | "user"
  | "project"
  | "org"
  | "temporary"
  | "oauth";
```

Claude Code сообщает одно из четырёх значений:

| Значение             | Используемый ключ                                                                                                                |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_API_KEY`  | Ключ в переменной окружения `ANTHROPIC_API_KEY`                                                                                  |
| `apiKeyHelper`       | Ключ, возвращённый вашей командой [`apiKeyHelper`](/docs/ru/settings-reference#apikeyhelper)                                          |
| `/login managed key` | Ключ, сохранённый Claude Code при входе с учётной записью [Claude Console](/docs/ru/authentication#claude-console-authentication)     |
| `none`               | Нет API ключа. Сессия аутентифицируется другим способом, например, через вход claude.ai, токен-носитель или облачного провайдера |

Agent SDK v0.3.234 и позже перечисляют эти четыре значения в типе. Тип также сохраняет `user`, `project`, `org`, `temporary` и `oauth`, чтобы старый код всё ещё компилировался, и Claude Code их не сообщает.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Доступные бета-функции, которые можно включить через опцию `betas`. См. [Beta headers](https://platform.claude.com/docs/en/api/beta-headers) для дополнительной информации.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  Бета `context-1m-2025-08-07` снята с производства по состоянию на 30 апреля 2026 года. Передача этого значения с Claude Sonnet 4.5 или Sonnet 4 не имеет эффекта, и запросы, превышающие стандартное окно контекста 200k-токенов, возвращают ошибку. Для использования окна контекста 1M-токенов перейдите на [Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 или Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), которые включают контекст 1M по стандартной цене без требуемого заголовка beta.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Информация о доступной команде.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

`builtin` это `true` в строке, когда команда является собственной Claude Code и ввод `/name` её запускает. Она отсутствует для команды, определённой пользователем, проектом, плагином или MCP сервером, и для встроенной команды, которую один из них [заменяет по имени](/docs/ru/skills#resolve-skills-that-share-a-name). Требует Agent SDK v0.3.277 или позже.

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

| Поле          | Тип                   | Описание                                                                                                                                                                                                                              |
| :------------ | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | `string`              | Идентификатор типа агента (например, `"Explore"`, `"general-purpose"`)                                                                                                                                                                |
| `description` | `string`              | Описание, когда использовать этого агента                                                                                                                                                                                             |
| `model`       | `string \| undefined` | Модель, которую использует этот агент: псевдоним или идентификатор модели, или `'inherit'` для модели родителя. Когда это `undefined`, Claude Code выбирает модель в [порядке выбора модели подагента](/docs/ru/sub-agents#choose-a-model) |

<h3 id="mcpserverprovenance">
  `McpServerProvenance`
</h3>

MCP сервер, который обслуживает tool `mcp__*`, и источник определения этого сервера. Входные данные hook [`PreToolUse`](#pretoolusehookinput), `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` и `PermissionDenied` несут его как `mcp_server`, а опции [`CanUseTool`](#canusetool) несут его как `mcpServer`. Оба опускают его для tools, которые не поступают от MCP сервера.

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| Поле     | Тип      | Описание                                                                                                                |
| :------- | :------- | :---------------------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | Имя, под которым зарегистрирован сервер, то же значение, которое [`mcpServerStatus()`](#query-object) сообщает для него |
| `source` | `string` | Источник определения сервера: `sdk`, `plugin` или область конфигурации                                                  |

`source` принимает одно из следующих значений. Набор открыт, поэтому рассматривайте значение, которое вы не узнаёте, как настроенный источник, никогда не как `sdk`:

* **`sdk`**: встроенный в процесс сервер, который зарегистрировало ваше приложение. Только приложение-хост SDK может зарегистрировать один, поэтому настроенный сервер никогда не сообщает `sdk`, независимо от его имени.
* **`plugin`**: сервер, который предоставляет [plugin](/docs/ru/agent-sdk/plugins). Его `name` это форма с областью видимости `plugin:<plugin-name>:<server-name>`, описанная в разделе [plugin-provided MCP servers](/docs/ru/mcp#plugin-provided-mcp-servers).
* **Область конфигурации**: `user`, `project`, `local`, `dynamic`, `managed`, `enterprise`, `claudeai` или `agent`. Сервер `.mcp.json` сообщает `project`, и [MCP installation scopes](/docs/ru/mcp#mcp-installation-scopes) определяет `local`, `project` и `user`. Серверы, которые ваше приложение передаёт в опции [`mcpServers`](#options), кроме встроенных в процесс SDK серверов, сообщают `dynamic`.

Основывайте решения о доверии на `source`, а не на `name` или префиксе имени tool `mcp__<server>__`. Для любого источника, кроме `sdk`, `name` это недоверенный текст: экранируйте его перед отображением.

`McpServerProvenance` и поля, которые его несут, требуют Agent SDK v0.3.274 или позже.

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
  source?: string;
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

`source` говорит, откуда поступило определение сервера, с теми же значениями и правилом доверия, что и `source` [`McpServerProvenance`](#mcpserverprovenance). Поле требует Agent SDK v0.3.274 или позже и отсутствует в более ранних версиях.

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
  thinkingTokens?: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
  canonicalModel?: string;
  provider?: string;
  costBasis?: 'list' | 'managed' | 'unknown';
};
```

`thinkingTokens` подсчитывает токены мышления, которые сгенерировала эта модель. `outputTokens` уже включает их, поэтому не складывайте эти два значения вместе. Поле отсутствует до тех пор, пока ход не запустится на версии Claude Code, которая его записывает, поэтому возобновлённая сессия, которая началась на более ранней версии, сообщает частичный подсчёт. `thinkingTokens` требует Agent SDK v0.3.257 или позже.

Поля `canonicalModel` и `provider` требуют Claude Code v2.1.218 или позже. `canonicalModel` это канонический идентификатор модели, который используется для поиска цены; он может отличаться от исходной строки модели, которая является ключом записи, например, когда эта строка это идентификатор провайдера или псевдоним.

`provider` называет API бэкенд, который обслуживал модель, такой как `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle` или `gateway`.

`costBasis` называет таблицу цен, которая установила цену на последний запрос модели: `list` для цены списка, `managed` для таблицы [`modelPricing`](/docs/ru/settings-reference#modelpricing) или `unknown`, когда ни одна не совпала с идентификатором модели. Поле требует Claude Code v2.1.246 или позже.

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
  output_tokens_details: BetaOutputTokensDetails | null;
};
```

`BetaServerToolUsage`, `BetaIterationsUsage` и `BetaOutputTokensDetails` определены в `@anthropic-ai/sdk`.

`output_tokens_details` разбивает выставленный счёт по категориям. В настоящее время он содержит одно поле, `thinking_tokens: number`, подсчитывающее выходные токены, которые модель сгенерировала как внутреннее рассуждение, включая разделители блока мышления. Поле `output_tokens_details` требует TypeScript SDK v0.3.228 или позже, который поставляется с Claude Code v2.1.228.

* **Выставление счётов**: читайте разбивку для наблюдаемости, а не для выставления счётов. `output_tokens` остаётся авторитетным итогом, и `output_tokens - thinking_tokens` приблизительно соответствует выходу без рассуждений.
* **Что подсчитывает**: исходное рассуждение, которое произвела модель, которое может быть длиннее текста мышления, возвращённого в теле ответа. API вычисляет его путём повторной токенизации этого исходного текста, поэтому он может отличаться от точного подсчёта генерации модели на несколько токенов.
* **Потоковая передача**: на потоковых сообщениях ассистента эта разбивка, как и `output_tokens`, это заполнитель `message_start` и не содержит реального подсчёта, поэтому читайте её из сообщения результата `usage` как [Читайте выходные токены из сообщения результата](/docs/ru/agent-sdk/cost-tracking#read-output-tokens-from-the-result-message) описывает. На сообщении результата `thinking_tokens` читает `0`, когда модель или провайдер не сообщает разбивку.
* **Случаи `null`**: `output_tokens_details` сам по себе это `null` на сообщениях ассистента, которые Claude Code синтезирует, такие как сообщения об ошибках API.

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

<h3 id="sdkmcpresourcelink">
  `SDKMcpResourceLink`
</h3>

Один файл, который MCP tool вернул по ссылке. Claude Code создаёт каждую запись из блока `resource_link` в результате tool и доставляет список как `resourceLinks` на [`SDKUserMessage.tool_use_result`](#sdkusermessage) или как `resource_links` на [`SDKTaskNotificationMessage`](#sdktasknotificationmessage), когда вызов завершился в фоне. Требует Agent SDK v0.3.257 или позже.

```typescript theme={null}
type SDKMcpResourceLink = {
  uri: string;
  name: string;
  title?: string;
  description?: string;
  mimeType?: string;
  size?: number;
  annotations?: Record<string, unknown>;
};
```

Claude Code отбрасывает блок, чей `uri` или `name` не является строкой, и опускает опциональное поле, чьё значение не соответствует указанному типу.

| Поле          | Тип                                    | Описание                                               |
| :------------ | :------------------------------------- | :----------------------------------------------------- |
| `uri`         | `string`                               | URI ресурса, как его вернул сервер                     |
| `name`        | `string`                               | Имя, которое сервер дал ресурсу                        |
| `title`       | `string \| undefined`                  | Отображаемое название, когда сервер его установил      |
| `description` | `string \| undefined`                  | Описание, когда сервер его установил                   |
| `mimeType`    | `string \| undefined`                  | MIME тип, когда сервер его установил                   |
| `size`        | `number \| undefined`                  | Размер в байтах, когда сервер его установил            |
| `annotations` | `Record<string, unknown> \| undefined` | Объект MCP аннотаций блока, когда сервер его установил |

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

Опциональное поле `display` контролирует, возвращается ли текст мышления `"summarized"` или `"omitted"`. На Claude Opus 4.7 и позже, значение по умолчанию API это `"omitted"`, поэтому установите `"summarized"` для получения содержимого мышления в блоках `thinking`. Claude Code не отправляет `display` на Amazon Bedrock или Google Cloud's Agent Platform, поэтому на этих провайдерах Opus 4.7 и позже возвращают пустые блоки `thinking` даже когда вы установите `display` на `"summarized"`.

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

Когда вы вызываете `setMcpServers()`, Claude Code применяет эти правила:

* **Серверы, которые вызов не называет**: Claude Code держит серверы, предоставленные плагинами, работающими. Требует Agent SDK v0.3.210 или позже.
* **Серверы, которые вызов называет**: за исключением встроенных серверов, которые CLI запустил при запуске, Claude Code заменяет работающий сервер только когда его конфигурация отличается от переданной вами.
* **Встроенные серверы, которые CLI запустил при запуске**: если вызов называет один, Claude Code отбрасывает эту запись и сообщает её в `errors`.

Обещание разрешается после того, как вновь добавленные stdio, HTTP и SSE серверы подключатся или не смогут подключиться, поэтому tools из серверов, которые подключились, доступны на следующем ходу.

`added` перечисляет серверы, которые Claude Code добавил или заменил, подключились они или нет. Сервер, который не смог подключиться, появляется как в `added`, так и в `errors`, с текстом ошибки под `errors` и строкой `failed` в [`mcpServerStatus()`](#methods). До Claude Code v2.1.257 сервер, попытка подключения которого выбросила исключение, сообщался только под `errors`.

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
  skippedLinks?: number;
};
```

`skippedLinks` подсчитывает отслеживаемые пути, которые перемотка отказалась восстанавливать или удалять для безопасности ссылок: символическая ссылка, жёсткая ссылка или другой не обычный файл по отслеживаемому пути, родительский каталог, который больше не разрешается туда, где он указывал при создании контрольной точки, или резервная копия, которая не могла быть прочитана безопасно. Поле требует Claude Code v2.1.216 или позже. Предварительный вызов с `rewindFiles(userMessageId, { dryRun: true })` никогда его не устанавливает.

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

Уведомление, когда фоновая задача завершается, не работает или остановлена. Фоновые задачи включают команды Bash `run_in_background`, наблюдения [Monitor](#monitor) и фоновые подагенты. Для поля `ambient` см. [`SDKTaskStartedMessage`](#sdktaskstartedmessage), которое его определяет и его требование версии.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  ambient?: boolean;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  resource_links?: SDKMcpResourceLink[];
  uuid: UUID;
  session_id: string;
};
```

Когда Claude Code [перемещает долгий вызов MCP tool в фоновый режим](/docs/ru/mcp#automatic-backgrounding-of-long-tool-calls), блок `tool_result` для этого вызова содержит только заполнитель и реальный результат вызова приходит в этом уведомлении. Сопоставьте уведомление с вызовом с помощью `tool_use_id`. На уведомлении `completed`, `resource_links` перечисляет файлы, которые tool вернул по ссылке как записи [`SDKMcpResourceLink`](#sdkmcpresourcelink), с теми же ограничениями 50-ссылок и 64 KiB, что и [`tool_use_result.resourceLinks`](#sdkusermessage). Claude Code опускает `resource_links`, когда результат не имел ссылок и на уведомлениях для задач, которые не являются вызовами MCP tool. `resource_links` требует Agent SDK v0.3.257 или позже.

Claude Code добавляет уведомление к каждому уведомлению задачи, которое он отправляет модели, за исключением доставок с меткой подвида [`scheduled-trigger`](#task-notification-subkinds), которые несут вместо этого фреймворк назначенной задачи. Уведомление указывает, что не произошло никакого взаимодействия с человеком, поэтому модель не рассматривает уведомление как инструкцию пользователя или одобрение.

Чтобы обнаружить ход уведомления задачи, проверьте `origin.kind === "task-notification"` на [`SDKUserMessage`](#sdkusermessage) или [`SDKResultMessage`](#sdkresultmessage) вместо сопоставления текста уведомления. Читайте `subkind` из того же поля, если вам нужно знать, что его вызвало. До v2.1.205 Claude Code оставлял уведомление на уведомлениях, которые приходили, пока сессия была неактивна.

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
  heartbeat?: boolean;
  subagent_type?: string;
  subagent_retry?: {
    agent_id: string;
    attempt: number;
    max_retries: number;
    retry_delay_ms: number;
    error_status: number | null;
    error_category: string;
  };
  uuid: UUID;
  session_id: string;
};
```

Пока вызов tool выполняется в основном диалоге, Claude Code выдаёт сообщение `tool_progress` каждые 30 секунд с `heartbeat: true`. Каждый heartbeat содержит имя tool и прошедшие секунды, поэтому вы можете отличить долгоживущий вызов от зависшей сессии. Claude Code не выдаёт heartbeats для вызовов tool внутри подагента. Поле `heartbeat` требует Agent SDK v0.3.214 или позже. До v2.1.257 Claude Code не выдавал heartbeats для вызова Agent tool на переднем плане либо.

На сообщениях `tool_progress` для tool Agent, кроме heartbeats, `subagent_type` называет работающий тип подагента, такой как `general-purpose`. `subagent_retry` присутствует, пока этот подагент ждёт отката ошибки API, такой как ограничение скорости или перегрузка, с одним сообщением на попытку повтора. Оба поля требуют Agent SDK v0.3.214 или позже.

Чтобы отобразить индикатор повтора из `subagent_retry`:

* Отслеживайте индикатор по `parent_tool_use_id`, который уникален для каждого подагента. `tool_use_id` совместно используется параллельными подагентами из одного хода ассистента, поэтому отслеживание по нему позволило бы обновлению одного подагента очистить индикатор другого.
* Очистите индикатор, когда позже приходит `tool_progress` для того же `parent_tool_use_id` без `subagent_retry` и без `heartbeat: true`, или когда приходит сообщение результата tool. Кадры с `heartbeat: true` сообщают только о живости, поэтому сохраняйте индикатор, когда один приходит. `attempt` может превышать `max_retries` при постоянном повторе, поэтому не выводите очистку из счётчиков.
* Рассматривайте `error_category` как токен для выбора вашего собственного текста сообщения, а не как текст отображения. Значения это `rate_limit`, `overloaded`, `authentication_failed`, `server_error`, `cloud_credential_error` и `unknown`. Обрабатывайте значение, которое вы не узнаёте, так же, как вы обрабатываете `unknown`, потому что более поздние выпуски могут добавлять значения.

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

Выдаётся, когда задача начинается. Поле `task_type` это `"local_bash"` для команд Bash и наблюдений [Monitor](#monitor), `"local_agent"` для подагентов или `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  is_backgrounded?: boolean;
  spawn_depth?: number;
  ambient?: boolean;
  uuid: UUID;
  session_id: string;
};
```

`ambient` это `true` для задач, которые не являются частью работы сессии, такие как задачи, которые Claude Code запускает для своей собственной работы. Наблюдатели живого обновления также являются ambient, включая наблюдателей, которых попросил пользователь. Исключите ambient задачи из индикаторов активности. Поле требует Agent SDK v0.3.247 или позже.

`ambient` также появляется на [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) и на записях [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage).

`is_backgrounded` и `spawn_depth` описывают, как Claude Code запустил задачу. Оба поля требуют Agent SDK v0.3.238 или позже.

* `is_backgrounded`: Claude Code устанавливает его на задачах `"local_agent"` и `"local_bash"`. `true` означает, что задача выполняется в фоне. `false` означает, что задача выполняется на переднем плане, и вызов tool, который её запустил, остаётся заблокированным до тех пор, пока задача не завершится или не переместится в фоновый режим.
* `spawn_depth`: Claude Code устанавливает его только на задачах `"local_agent"`. Подагент, который основной поток запустил, имеет глубину `1`. Подагент, который подагент глубины `1` запустил, имеет глубину `2`, и так далее.

[Возобновлённый подагент](/docs/ru/agent-sdk/subagents#resume-subagents) всегда сообщает `is_backgrounded: true`, потому что Claude Code запускает каждый возобновлённый подагент в фоне. Когда задача на переднем плане позже переместится в фоновый режим, Claude Code сообщает новое значение `is_backgrounded` в сообщении [`task_updated`](#sdktaskupdatedmessage) вместо отправки второго `task_started`.

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

Выдаётся всякий раз, когда набор активных фоновых задач изменяется: задача начинается, завершается, убивается, задача на переднем плане переводится в фоновый режим, или поле `description` или `ambient` задачи изменяется.

Массив `tasks` это полный активный набор. Замените любой кэшированный набор каждым payload вместо сопряжения событий `task_started` и `task_notification`, поэтому следующее изменение членства исправит любое событие, которое вы пропустили.

Порядок относительно этих событий для каждой задачи не определён, поэтому не коррелируйте два потока.

Ничего не выдаётся при запуске. Сбросьте на пустой набор всякий раз, когда процесс CLI сессии запускается или перезапускается, и позвольте следующему изменению членства переполнить его.

Когда вы отправляете повторный запрос управления `initialize` работающей сессии, такой как с [`reinitialize()`](#query-object) после разрыва транспорта, Claude Code следует ответу снимком текущего активного набора, даже когда он пуст. Переподключающийся хост поэтому узнаёт, что работает, без ожидания следующего изменения членства. До Agent SDK v0.3.239 Claude Code не отправлял снимок после повторного `initialize`.

Требуется Claude Code v2.1.203 или позже.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
    ambient?: boolean;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Выдаётся во время создания Claude блока мышления, включая отредактированный. `estimated_tokens` это текущая оценка токенов мышления, сгенерированных до сих пор в текущем блоке, и `estimated_tokens_delta` это приращение, переносимое этим кадром. Используйте эти оценки для отображения прогресса.

Когда модель или провайдер сообщает разбивку, окончательный подсчёт для цикла агента верхнего уровня это [`usage.output_tokens_details.thinking_tokens`](#usage) сообщения результата, который [не включает токены подагентов](/docs/ru/agent-sdk/cost-tracking#get-the-total-cost-of-a-query).

Требуется Claude Code v2.1.153 или позже.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  user_message_uuid?: string;
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

Claude Code не выдаёт этот тип сообщения. Когда вы отправляете команду, такую как `/context` или `/usage`, как запрос, её вывод приходит как [`SDKAssistantMessage`](#sdkassistantmessage).

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

Выдаётся, когда набор доступных команд изменяется во время сессии, например, когда Claude Code обнаруживает skills при входе агента в подпапку. Массив `commands` это полный обновлённый список, поэтому замените любой кэшированный список команд этим payload. Вызов [`supportedCommands()`](#query-object) после этого сообщения возвращает тот же обновлённый список, потому что метод отслеживает последний push; это требует Agent SDK v0.3.216 или позже. В более ранних версиях SDK `supportedCommands()` возвращает снимок, захваченный при инициализации, и никогда не отражает изменения во время сессии.

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

Выдаётся после хода, когда [`promptSuggestions`](#options) включён и Claude Code сгенерировал предложение для этого хода. Содержит предсказанный следующий пользовательский запрос. Для ходов, которые не получают никаких, см. [Когда Claude Code пропускает предложения](/docs/ru/interactive-mode#when-claude-code-skips-suggestions).

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

Выдаётся, когда диалог сессии заменяется без завершения сессии. В вызове `query()` только `/clear` и его псевдонимы производят это сообщение. Смонтируйте пустой транскрипт под `new_conversation_id` и отбросьте любой кэшированный заголовок сессии.

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

`AbortError` это единственный класс ошибки в типизированном API SDK. Другие сбои, такие как выход процесса Claude Code или неудача при запуске, отклоняют итерацию сообщения с ошибками, которые не несут класс SDK для сопоставления. [Troubleshooting](/docs/ru/agent-sdk/troubleshooting) ключает эти ошибки по сообщению, с причиной и исправлением для каждого.

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

| Свойство                    | Тип                                                   | По умолчанию | Описание                                                                                                                                                                                                                                             |
| :-------------------------- | :---------------------------------------------------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`      | Включите режим sandbox для выполнения команд                                                                                                                                                                                                         |
| `failIfUnavailable`         | `boolean`                                             | `true`       | Остановитесь при запуске, если `enabled` имеет значение `true`, но sandbox не может запуститься. Установите `false` для возврата к выполнению без sandbox с предупреждением на stderr                                                                |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`       | Автоматически одобряйте bash команды, когда sandbox включён                                                                                                                                                                                          |
| `excludedCommands`          | `string[]`                                            | `[]`         | Команды, которые обходят ограничения sandbox, такие как `['docker *']`. Они работают без sandbox автоматически без участия модели; [`sandbox.excludedCommands`](/docs/ru/settings-reference#sandbox-excludedcommands) описывает, когда применяется запись |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`       | Разрешите модели запрашивать выполнение команд вне sandbox. Когда `true`, модель может установить `dangerouslyDisableSandbox` в входных данных tool, что переходит к [системе разрешений](#permissions-fallback-for-unsandboxed-commands)            |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined`  | Конфигурация sandbox, специфичная для сети                                                                                                                                                                                                           |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined`  | Конфигурация sandbox, специфичная для файловой системы, для ограничений чтения/записи                                                                                                                                                                |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined`  | Карта подстрок команд или `*` для каждой команды к подстрокам текста нарушения для игнорирования, такие как `{ "*": ['/etc/hosts'] }`; см. [`sandbox.ignoreViolations`](/docs/ru/settings-reference#sandbox-ignoreviolations)                             |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`      | Включите более слабый вложенный sandbox для совместимости                                                                                                                                                                                            |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined`  | Конфигурация пользовательского бинарного файла ripgrep для окружений sandbox                                                                                                                                                                         |

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
  **Безопасность Unix socket:** Опция `allowUnixSockets` может предоставить доступ к системным сервисам, которые выходят за пределы sandbox. Например, разрешение `/var/run/docker.sock` фактически предоставляет полный доступ к хост-системе через Docker API, обходя изоляцию sandbox. Разрешайте только Unix sockets, которые строго необходимы, и поймите последствия безопасности каждого.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Конфигурация, специфичная для сети, для режима sandbox. Эти параметры применяются к sandboxed Bash командам, когда `enabled` имеет значение `true` в родительском [`SandboxSettings`](#sandboxsettings). Они не ограничивают инструмент WebFetch, который использует [правила разрешений](/docs/ru/permissions#webfetch) вместо этого.

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  strictAllowlist?: boolean;
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Свойство                  | Тип        | По умолчанию | Описание                                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------------ | :--------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`         | Имена доменов, к которым процессы в sandbox могут получить доступ                                                                                                                                                                                                                                                                                                                                |
| `deniedDomains`           | `string[]` | `[]`         | Имена доменов, к которым процессы в sandbox не могут получить доступ. Имеет приоритет над `allowedDomains`                                                                                                                                                                                                                                                                                       |
| `strictAllowlist`         | `boolean`  | `false`      | Запретите sandboxed командам доступ к хостам вне [сетевого allowlist](/docs/ru/sandboxing#network-isolation) вместо запроса. Применяется только для sandboxed команд; встроенные инструменты, такие как WebFetch, не контролируются этим. Учитывается только из пользовательских, управляемых или CLI `--settings` параметров; параметры проекта игнорируются. Требует Claude Code v2.1.219 или позже |
| `allowManagedDomainsOnly` | `boolean`  | `false`      | Только управляемые параметры. Когда установлено в [управляемых параметрах](/docs/ru/managed-settings), только записи `allowedDomains` и правила разрешения `WebFetch(domain:...)` из управляемых параметров учитываются, а записи разрешения из пользовательских, проектных или локальных параметров игнорируются. Не имеет эффекта при установке через опции SDK                                     |
| `allowLocalBinding`       | `boolean`  | `false`      | Разрешите процессам привязываться к локальным портам (например, для dev серверов)                                                                                                                                                                                                                                                                                                                |
| `allowUnixSockets`        | `string[]` | `[]`         | Пути Unix socket, к которым процессы могут получить доступ (например, Docker socket)                                                                                                                                                                                                                                                                                                             |
| `allowAllUnixSockets`     | `boolean`  | `false`      | Разрешите доступ ко всем Unix sockets                                                                                                                                                                                                                                                                                                                                                            |
| `httpProxyPort`           | `number`   | `undefined`  | Порт HTTP прокси для сетевых запросов                                                                                                                                                                                                                                                                                                                                                            |
| `socksProxyPort`          | `number`   | `undefined`  | Порт SOCKS прокси для сетевых запросов                                                                                                                                                                                                                                                                                                                                                           |

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

Когда `allowUnsandboxedCommands` включён, модель может запросить выполнение команд вне sandbox, установив `dangerouslyDisableSandbox: true` во входных данных tool. Эти запросы переходят к существующей системе разрешений, что означает, что ваш обработчик `canUseTool` вызывается, позволяя вам реализовать пользовательскую логику авторизации.

Ваши записи `excludedCommands` вместо этого автоматически обходят sandbox без участия модели; [`sandbox.excludedCommands`](/docs/ru/settings-reference#sandbox-excludedcommands) описывает, когда применяется запись.

В примере ниже `isCommandAuthorized` служит заместителем для проверки авторизации, которую вы определяете.

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

<Warning>
  Команды, работающие с `dangerouslyDisableSandbox: true`, имеют полный доступ к системе. Убедитесь, что ваш обработчик `canUseTool` тщательно проверяет эти запросы.

  Если `permissionMode` установлен на `bypassPermissions` и `allowUnsandboxedCommands` включён, модель может автономно выполнять команды вне sandbox без запросов одобрения, кроме [действий, которые режим no не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves). Эта комбинация фактически позволяет модели молча выходить из изоляции sandbox.
</Warning>

<h2 id="see-also">
  См. также
</h2>

* [Обзор SDK](/docs/ru/agent-sdk/overview) - Общие концепции SDK
* [Справочник Python SDK](/docs/ru/agent-sdk/python) - Документация Python SDK
* [Справочник CLI](/docs/ru/cli-reference) - Интерфейс командной строки
* [Общие рабочие процессы](/docs/ru/common-workflows) - Пошаговые руководства
