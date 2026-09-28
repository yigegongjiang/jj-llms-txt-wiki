> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Справочник Agent SDK - Python

> Полный справочник API для Python Agent SDK, включая все функции, типы и классы.

<h2 id="installation">
  Установка
</h2>

Установите пакет в виртуальное окружение. На недавних установках Debian, Ubuntu и Homebrew Python запуск `pip install` для системного Python завершается ошибкой `error: externally-managed-environment`.

```bash theme={null}
python3 -m venv .venv
source .venv/bin/activate
pip install claude-agent-sdk
```

Для uv, Windows PowerShell и настройки API ключа см. [Начало работы в быстром старте Agent SDK](/docs/ru/agent-sdk/quickstart#setup).

<h2 id="choosing-between-query-and-claudesdkclient">
  Выбор между `query()` и `ClaudeSDKClient`
</h2>

Python SDK предоставляет два способа взаимодействия с Claude Code:

| Функция                          | `query()`                                         | `ClaudeSDKClient`                       |
| :------------------------------- | :------------------------------------------------ | :-------------------------------------- |
| **Сеанс**                        | Создает новый сеанс по умолчанию                  | Повторно использует один и тот же сеанс |
| **Разговор**                     | Один обмен                                        | Несколько обменов в одном контексте     |
| **Соединение**                   | Управляется автоматически                         | Ручное управление                       |
| **Потоковый ввод**               | ✅ Поддерживается                                  | ✅ Поддерживается                        |
| **Прерывания**                   | ❌ Не поддерживается                               | ✅ Поддерживается                        |
| **Hooks**                        | ✅ Поддерживается                                  | ✅ Поддерживается                        |
| **Пользовательские инструменты** | ✅ Поддерживается                                  | ✅ Поддерживается                        |
| **Продолжить чат**               | Ручное через `continue_conversation` или `resume` | ✅ Автоматическое                        |
| **Вариант использования**        | Одноразовые задачи                                | Непрерывные разговоры                   |

Используйте `ClaudeSDKClient` для интерактивных приложений, таких как интерфейсы чата, или когда следующее действие зависит от ответа Claude.

<h2 id="functions">
  Функции
</h2>

<Note>Блоки сигнатур и голые фрагменты `async for` / `async with` на этой странице являются иллюстративными. Чтобы их запустить, оберните тело в `async def main(): ...` и вызовите `asyncio.run(main())`.</Note>

<h3 id="query">
  `query()`
</h3>

Создает новый сеанс для каждого взаимодействия с Claude Code по умолчанию. Возвращает асинхронный итератор, который выдает сообщения по мере их поступления. Каждый вызов `query()` начинается с нуля без памяти о предыдущих взаимодействиях, если вы не передадите `continue_conversation=True` или `resume` в [`ClaudeAgentOptions`](#claudeagentoptions). См. [Sessions](/docs/ru/agent-sdk/sessions).

```python theme={null}
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None,
    transport: Transport | None = None
) -> AsyncIterator[Message]
```

<h4 id="parameters">
  Параметры
</h4>

| Параметр    | Тип                          | Описание                                                                                 |
| :---------- | :--------------------------- | :--------------------------------------------------------------------------------------- |
| `prompt`    | `str \| AsyncIterable[dict]` | Входная подсказка в виде строки или асинхронного итератора для режима потоковой передачи |
| `options`   | `ClaudeAgentOptions \| None` | Объект дополнительной конфигурации (по умолчанию `ClaudeAgentOptions()`, если None)      |
| `transport` | `Transport \| None`          | Дополнительный пользовательский транспорт для связи с процессом CLI                      |

<h4 id="returns">
  Возвращаемое значение
</h4>

Возвращает `AsyncIterator[Message]`, который выдает сообщения из разговора.

<h4 id="example-with-options">
  Пример - С параметрами
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    options = ClaudeAgentOptions(
        system_prompt="You are an expert Python developer",
        permission_mode="acceptEdits",
    )

    async for message in query(prompt="Create a Python web server", options=options):
        print(message)


asyncio.run(main())
```

<h3 id="tool">
  `tool()`
</h3>

Декоратор для определения MCP tools с проверкой типов.

```python theme={null}
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any],
    annotations: ToolAnnotations | None = None
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

<h4 id="parameters-2">
  Параметры
</h4>

| Параметр       | Тип                                             | Описание                                                                                             |
| :------------- | :---------------------------------------------- | :--------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Уникальный идентификатор инструмента                                                                 |
| `description`  | `str`                                           | Понятное описание того, что делает инструмент                                                        |
| `input_schema` | `type \| dict[str, Any]`                        | Схема, определяющая входные параметры инструмента. См. [Input schema options](#input-schema-options) |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Дополнительные аннотации MCP tool, предоставляющие подсказки поведения клиентам                      |

<h4 id="input-schema-options">
  Варианты схемы ввода
</h4>

1. **Простое сопоставление типов** (рекомендуется):

   ```python theme={null}
   {"text": str, "count": int, "enabled": bool}
   ```

2. **Формат JSON Schema** (для сложной валидации):
   ```python theme={null}
   {
       "type": "object",
       "properties": {
           "text": {"type": "string"},
           "count": {"type": "integer", "minimum": 0},
       },
       "required": ["text"],
   }
   ```

<h4 id="returns-2">
  Возвращаемое значение
</h4>

Функция-декоратор, которая оборачивает реализацию инструмента и возвращает экземпляр `SdkMcpTool`.

<h4 id="example">
  Пример
</h4>

```python theme={null}
from claude_agent_sdk import tool
from typing import Any


@tool("greet", "Greet a user", {"name": str})
async def greet(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Hello, {args['name']}!"}]}
```

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Подсказки поведения для инструмента, передаваемые как аргумент `annotations` функции [`tool()`](#tool). `ToolAnnotations` расширяет `mcp.types.ToolAnnotations` SDK MCP с полем `maxResultSizeChars`, и вы можете писать каждую подсказку в camelCase или snake\_case: `ToolAnnotations(readOnlyHint=True)` и `ToolAnnotations(read_only_hint=True)` эквивалентны. Вы также можете передать простой `mcp.types.ToolAnnotations` везде, где SDK принимает аннотации.

Имена snake\_case и типизированное поле `maxResultSizeChars` требуют Python Agent SDK 0.2.140 или позже. Версии 0.1.31 по 0.2.139 переэкспортируют `mcp.types.ToolAnnotations` без изменений. В версиях 0.1.55 по 0.2.139 вы все еще можете передать `maxResultSizeChars` как аргумент ключевого слова: класс MCP принимает дополнительные поля, и SDK пересылает значение в Claude Code.

Все поля являются дополнительными. Клиенты не должны полагаться на подсказки для решений безопасности.

| Поле                 | Тип            | По умолчанию | Описание                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------------------- | :------------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`              | `str \| None`  | `None`       | Понятное название инструмента                                                                                                                                                                                                                                                                                                                                                                                      |
| `readOnlyHint`       | `bool \| None` | `False`      | Если `True`, инструмент не изменяет свою среду                                                                                                                                                                                                                                                                                                                                                                     |
| `destructiveHint`    | `bool \| None` | `True`       | Если `True`, инструмент может выполнять деструктивные обновления (имеет смысл только когда `readOnlyHint` равен `False`)                                                                                                                                                                                                                                                                                           |
| `idempotentHint`     | `bool \| None` | `False`      | Если `True`, повторные вызовы с одинаковыми аргументами не имеют дополнительного эффекта (имеет смысл только когда `readOnlyHint` равен `False`)                                                                                                                                                                                                                                                                   |
| `openWorldHint`      | `bool \| None` | `True`       | Если `True`, инструмент взаимодействует с внешними сущностями (например, веб-поиск). Если `False`, область инструмента закрыта (например, инструмент памяти)                                                                                                                                                                                                                                                       |
| `maxResultSizeChars` | `int \| None`  | `None`       | Количество символов, до которого Claude Code сохраняет результат этого инструмента встроенным в разговор вместо сохранения в файл, до 500 000. Результаты, содержащие изображения, не затрагиваются. Параметр Claude Code, а не подсказка MCP: SDK отправляет его в `_meta` инструмента как `anthropic/maxResultSizeChars`. См. [Raise the limit for a specific tool](/docs/ru/mcp#raise-the-limit-for-a-specific-tool) |

```python theme={null}
from claude_agent_sdk import tool, ToolAnnotations
from typing import Any


@tool(
    "search",
    "Search the web",
    {"query": str},
    annotations=ToolAnnotations(readOnlyHint=True, openWorldHint=True),
)
async def search(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Results for: {args['query']}"}]}
```

<h3 id="create_sdk_mcp_server">
  `create_sdk_mcp_server()`
</h3>

Создайте встроенный MCP server, который работает в вашем приложении Python.

```python theme={null}
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

<h4 id="parameters-3">
  Параметры
</h4>

| Параметр  | Тип                             | По умолчанию | Описание                                                            |
| :-------- | :------------------------------ | :----------- | :------------------------------------------------------------------ |
| `name`    | `str`                           | -            | Уникальный идентификатор сервера                                    |
| `version` | `str`                           | `"1.0.0"`    | Строка версии сервера                                               |
| `tools`   | `list[SdkMcpTool[Any]] \| None` | `None`       | Список функций инструментов, созданных с помощью декоратора `@tool` |

<h4 id="returns-3">
  Возвращаемое значение
</h4>

Возвращает объект `McpSdkServerConfig`, который можно передать в `ClaudeAgentOptions.mcp_servers`.

<h4 id="example-2">
  Пример
</h4>

```python theme={null}
from claude_agent_sdk import tool, create_sdk_mcp_server, ClaudeAgentOptions


@tool("add", "Add two numbers", {"a": float, "b": float})
async def add(args):
    return {"content": [{"type": "text", "text": f"Sum: {args['a'] + args['b']}"}]}


@tool("multiply", "Multiply two numbers", {"a": float, "b": float})
async def multiply(args):
    return {"content": [{"type": "text", "text": f"Product: {args['a'] * args['b']}"}]}


calculator = create_sdk_mcp_server(
    name="calculator",
    version="2.0.0",
    tools=[add, multiply],  # Pass decorated functions
)

# Use with Claude
options = ClaudeAgentOptions(
    mcp_servers={"calc": calculator},
    allowed_tools=["mcp__calc__add", "mcp__calc__multiply"],
)
```

<h3 id="list_sessions">
  `list_sessions()`
</h3>

Выводит список прошлых сеансов с метаданными. Фильтруйте по каталогу проекта или выводите сеансы во всех проектах. Синхронно; возвращается немедленно.

```python theme={null}
def list_sessions(
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0,
    include_worktrees: bool = True
) -> list[SDKSessionInfo]
```

<h4 id="parameters-4">
  Параметры
</h4>

| Параметр            | Тип           | По умолчанию | Описание                                                                                                              |
| :------------------ | :------------ | :----------- | :-------------------------------------------------------------------------------------------------------------------- |
| `directory`         | `str \| None` | `None`       | Каталог для вывода сеансов. Если опущено, возвращает сеансы во всех проектах                                          |
| `limit`             | `int \| None` | `None`       | Максимальное количество возвращаемых сеансов                                                                          |
| `offset`            | `int`         | `0`          | Количество сеансов для пропуска с начала отсортированных результатов. Используйте с `limit` для разбиения на страницы |
| `include_worktrees` | `bool`        | `True`       | Когда `directory` находится внутри репозитория git, включайте сеансы из всех путей worktrees                          |

<h4 id="return-type-sdksessioninfo">
  Тип возвращаемого значения: `SDKSessionInfo`
</h4>

| Свойство        | Тип           | Описание                                                                                              |
| :-------------- | :------------ | :---------------------------------------------------------------------------------------------------- |
| `session_id`    | `str`         | Уникальный идентификатор сеанса                                                                       |
| `summary`       | `str`         | Отображаемое название: пользовательское название, автоматически созданное резюме или первая подсказка |
| `last_modified` | `int`         | Время последнего изменения в миллисекундах с начала эпохи                                             |
| `file_size`     | `int \| None` | Размер файла сеанса в байтах (`None` для удаленных хранилищ)                                          |
| `custom_title`  | `str \| None` | Название сеанса, установленное пользователем                                                          |
| `first_prompt`  | `str \| None` | Первая значимая подсказка пользователя в сеансе                                                       |
| `git_branch`    | `str \| None` | Ветка Git в конце сеанса                                                                              |
| `cwd`           | `str \| None` | Рабочий каталог для сеанса                                                                            |
| `tag`           | `str \| None` | Тег сеанса, установленный пользователем (см. [`tag_session()`](#tag_session))                         |
| `created_at`    | `int \| None` | Время создания сеанса в миллисекундах с начала эпохи                                                  |

<h4 id="example-3">
  Пример
</h4>

Выведите 10 самых последних сеансов для проекта. Результаты отсортированы по `last_modified` в убывающем порядке, поэтому первый элемент - самый новый. Опустите `directory`, чтобы искать во всех проектах.

```python theme={null}
from claude_agent_sdk import list_sessions

for session in list_sessions(directory="/path/to/project", limit=10):
    print(f"{session.summary} ({session.session_id})")
```

<h3 id="get_session_messages">
  `get_session_messages()`
</h3>

Извлекает сообщения из прошлого сеанса. Синхронно; возвращается немедленно.

```python theme={null}
def get_session_messages(
    session_id: str,
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0
) -> list[SessionMessage]
```

<h4 id="parameters-5">
  Параметры
</h4>

| Параметр     | Тип           | По умолчанию | Описание                                                        |
| :----------- | :------------ | :----------- | :-------------------------------------------------------------- |
| `session_id` | `str`         | обязательно  | ID сеанса для извлечения сообщений                              |
| `directory`  | `str \| None` | `None`       | Каталог проекта для поиска. Если опущено, ищет во всех проектах |
| `limit`      | `int \| None` | `None`       | Максимальное количество возвращаемых сообщений                  |
| `offset`     | `int`         | `0`          | Количество сообщений для пропуска с начала                      |

<h4 id="return-type-sessionmessage">
  Тип возвращаемого значения: `SessionMessage`
</h4>

| Свойство             | Тип                            | Описание                                                                                                                                                                                                                                                                          |
| :------------------- | :----------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `Literal["user", "assistant"]` | Роль сообщения                                                                                                                                                                                                                                                                    |
| `uuid`               | `str`                          | Уникальный идентификатор сообщения                                                                                                                                                                                                                                                |
| `session_id`         | `str`                          | Идентификатор сеанса                                                                                                                                                                                                                                                              |
| `message`            | `Any`                          | Необработанное содержимое сообщения                                                                                                                                                                                                                                               |
| `parent_tool_use_id` | `str \| None`                  | Для сообщений подагента, ID блока tool-use `Agent`, который его создал. `None` для сообщений основного сеанса и более старых сеансов                                                                                                                                              |
| `parent_agent_id`    | `str \| None`                  | Для сообщений от [вложенного подагента](/docs/ru/sub-agents#let-subagents-spawn-their-own-subagents), ID агента родительского подагента. `None` для сообщений основного сеанса, сообщений подагента верхнего уровня и более старых сеансов. Требует Python Agent SDK 0.2.140 или позже |

<h4 id="example-4">
  Пример
</h4>

```python theme={null}
from claude_agent_sdk import list_sessions, get_session_messages

sessions = list_sessions(limit=1)
if sessions:
    messages = get_session_messages(sessions[0].session_id)
    for msg in messages:
        print(f"[{msg.type}] {msg.uuid}")
```

<h3 id="get_session_info">
  `get_session_info()`
</h3>

Читает метаданные для одного сеанса по ID без сканирования полного каталога проекта. Синхронно; возвращается немедленно.

```python theme={null}
def get_session_info(
    session_id: str,
    directory: str | None = None,
) -> SDKSessionInfo | None
```

<h4 id="parameters-6">
  Параметры
</h4>

| Параметр     | Тип           | По умолчанию | Описание                                                             |
| :----------- | :------------ | :----------- | :------------------------------------------------------------------- |
| `session_id` | `str`         | обязательно  | UUID сеанса для поиска                                               |
| `directory`  | `str \| None` | `None`       | Путь каталога проекта. Если опущено, ищет во всех каталогах проектов |

Возвращает [`SDKSessionInfo`](#return-type-sdksessioninfo) или `None`, если сеанс не найден.

<h4 id="example-5">
  Пример
</h4>

Найдите метаданные одного сеанса без сканирования каталога проекта. Полезно, когда у вас уже есть ID сеанса из предыдущего запуска.

```python theme={null}
from claude_agent_sdk import get_session_info

info = get_session_info("550e8400-e29b-41d4-a716-446655440000")
if info:
    print(f"{info.summary} (branch: {info.git_branch}, tag: {info.tag})")
```

<h3 id="rename_session">
  `rename_session()`
</h3>

Переименовывает сеанс, добавляя запись с пользовательским названием. Повторные вызовы безопасны; побеждает самое последнее название. Синхронно.

```python theme={null}
def rename_session(
    session_id: str,
    title: str,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-7">
  Параметры
</h4>

| Параметр     | Тип           | По умолчанию | Описание                                                             |
| :----------- | :------------ | :----------- | :------------------------------------------------------------------- |
| `session_id` | `str`         | обязательно  | UUID сеанса для переименования                                       |
| `title`      | `str`         | обязательно  | Новое название. Должно быть непустым после удаления пробелов         |
| `directory`  | `str \| None` | `None`       | Путь каталога проекта. Если опущено, ищет во всех каталогах проектов |

Вызывает `ValueError`, если `session_id` не является допустимым UUID или `title` пуст; `FileNotFoundError`, если сеанс не найден.

<h4 id="example-6">
  Пример
</h4>

Переименуйте самый последний сеанс, чтобы его было легче найти позже. Новое название появляется в [`SDKSessionInfo.custom_title`](#return-type-sdksessioninfo) при последующих чтениях.

```python theme={null}
from claude_agent_sdk import list_sessions, rename_session

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    rename_session(sessions[0].session_id, "Refactor auth module")
```

<h3 id="tag_session">
  `tag_session()`
</h3>

Помечает сеанс. Передайте `None` для очистки тега. Повторные вызовы безопасны; побеждает самый последний тег. Синхронно.

```python theme={null}
def tag_session(
    session_id: str,
    tag: str | None,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-8">
  Параметры
</h4>

| Параметр     | Тип           | По умолчанию | Описание                                                                   |
| :----------- | :------------ | :----------- | :------------------------------------------------------------------------- |
| `session_id` | `str`         | обязательно  | UUID сеанса для пометки                                                    |
| `tag`        | `str \| None` | обязательно  | Строка тега или `None` для очистки. Очищается от Unicode перед сохранением |
| `directory`  | `str \| None` | `None`       | Путь каталога проекта. Если опущено, ищет во всех каталогах проектов       |

Вызывает `ValueError`, если `session_id` не является допустимым UUID или `tag` пуст после очистки; `FileNotFoundError`, если сеанс не найден.

<h4 id="example-7">
  Пример
</h4>

Пометьте сеанс, затем отфильтруйте по этому тегу при последующем чтении. Передайте `None` для очистки существующего тега.

```python theme={null}
from claude_agent_sdk import list_sessions, tag_session

# Tag the most recent session
sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    tag_session(sessions[0].session_id, "needs-review")

# Later: find all sessions with that tag
for session in list_sessions(directory="/path/to/project"):
    if session.tag == "needs-review":
        print(session.summary)
```

<h2 id="classes">
  Классы
</h2>

<h3 id="claudesdkclient">
  `ClaudeSDKClient`
</h3>

**Поддерживает сеанс разговора через несколько обменов.** Это эквивалент Python того, как функция `query()` TypeScript SDK работает внутри - она создает объект клиента, который может продолжать разговоры. См. [сравнение с `query()`](#choosing-between-query-and-claudesdkclient).

```python theme={null}
class ClaudeSDKClient:
    def __init__(self, options: ClaudeAgentOptions | None = None, transport: Transport | None = None)
    async def connect(self, prompt: str | AsyncIterable[dict] | None = None) -> None
    async def query(self, prompt: str | AsyncIterable[dict], session_id: str = "default") -> None
    async def receive_messages(self) -> AsyncIterator[Message]
    async def receive_response(self) -> AsyncIterator[Message]
    async def interrupt(self) -> None
    async def set_permission_mode(self, mode: PermissionMode) -> None
    async def set_model(self, model: str | None = None) -> None
    async def rewind_files(self, user_message_id: str) -> None
    async def get_mcp_status(self) -> McpStatusResponse
    async def reconnect_mcp_server(self, server_name: str) -> None
    async def toggle_mcp_server(self, server_name: str, enabled: bool) -> None
    async def stop_task(self, task_id: str) -> None
    async def get_server_info(self) -> dict[str, Any] | None
    async def disconnect(self) -> None
```

<h4 id="methods">
  Методы
</h4>

| Метод                                     | Описание                                                                                                                                                                       |
| :---------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `__init__(options)`                       | Инициализируйте клиент с дополнительной конфигурацией                                                                                                                          |
| `connect(prompt)`                         | Подключитесь к Claude с дополнительной начальной подсказкой или потоком сообщений                                                                                              |
| `query(prompt, session_id)`               | Отправьте новый запрос в режиме потоковой передачи                                                                                                                             |
| `receive_messages()`                      | Получайте все сообщения от Claude как асинхронный итератор                                                                                                                     |
| `receive_response()`                      | Получайте сообщения до и включая ResultMessage                                                                                                                                 |
| `interrupt()`                             | Отправьте сигнал прерывания (работает только в режиме потоковой передачи)                                                                                                      |
| `set_permission_mode(mode)`               | Измените режим разрешений для текущего сеанса                                                                                                                                  |
| `set_model(model)`                        | Измените модель для текущего сеанса. Передайте `None` для сброса на [модель по умолчанию Claude Code](/docs/ru/model-config)                                                        |
| `rewind_files(user_message_id)`           | Восстановите файлы в их состояние в указанном пользовательском сообщении. Требует `enable_file_checkpointing=True`. См. [File checkpointing](/docs/ru/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Получите статус всех настроенных MCP servers. Возвращает [`McpStatusResponse`](#mcpstatusresponse)                                                                             |
| `reconnect_mcp_server(server_name)`       | Повторите попытку подключения к MCP server, который не удался или был отключен                                                                                                 |
| `toggle_mcp_server(server_name, enabled)` | Включите или отключите MCP server в середине сеанса. Отключение удаляет его инструменты                                                                                        |
| `stop_task(task_id)`                      | Остановите выполняющуюся фоновую задачу. [`TaskNotificationMessage`](#tasknotificationmessage) со статусом `"stopped"` следует в потоке сообщений                              |
| `get_server_info()`                       | Получите информацию об инициализации сервера, включая доступные команды и стили вывода                                                                                         |
| `disconnect()`                            | Отключитесь от Claude                                                                                                                                                          |

<h4 id="context-manager-support">
  Поддержка менеджера контекста
</h4>

Клиент можно использовать как асинхронный менеджер контекста для автоматического управления соединением:

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient


async def main():
    async with ClaudeSDKClient() as client:
        await client.query("Hello Claude")
        async for message in client.receive_response():
            print(message)


asyncio.run(main())
```

> **Важно:** При итерации по сообщениям избегайте использования `break` для раннего выхода, так как это может вызвать проблемы с очисткой asyncio. Вместо этого позвольте итерации завершиться естественным образом или используйте флаги для отслеживания, когда вы нашли то, что вам нужно.

<h4 id="example-continuing-a-conversation">
  Пример - Продолжение разговора
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, AssistantMessage, TextBlock, ResultMessage


async def main():
    async with ClaudeSDKClient() as client:
        # First question
        await client.query("What's the capital of France?")

        # Process response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Follow-up question - the session retains the previous context
        await client.query("What's the population of that city?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Another follow-up - still in the same conversation
        await client.query("What are some famous landmarks there?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")


asyncio.run(main())
```

<h4 id="example-streaming-input-with-claudesdkclient">
  Пример - Потоковый ввод с ClaudeSDKClient
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient


async def message_stream():
    """Generate messages dynamically."""
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Analyze the following data:"},
    }
    await asyncio.sleep(0.5)
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Temperature: 25°C, Humidity: 60%"},
    }
    await asyncio.sleep(0.5)
    yield {
        "type": "user",
        "message": {"role": "user", "content": "What patterns do you see?"},
    }


async def main():
    async with ClaudeSDKClient() as client:
        # Stream input to Claude
        await client.query(message_stream())

        # Process response
        async for message in client.receive_response():
            print(message)

        # Follow-up in same session
        await client.query("Should we be concerned about these readings?")

        async for message in client.receive_response():
            print(message)


asyncio.run(main())
```

<h4 id="example-using-interrupts">
  Пример - Использование прерываний
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, ResultMessage


async def interruptible_task():
    options = ClaudeAgentOptions(allowed_tools=["Bash"], permission_mode="acceptEdits")

    async with ClaudeSDKClient(options=options) as client:
        # Start a long-running task
        await client.query("Count from 1 to 100 slowly, using the bash sleep command")

        # Let it run for a bit
        await asyncio.sleep(2)

        # Interrupt the task
        await client.interrupt()
        print("Task interrupted!")

        # Drain the interrupted task's messages (including its ResultMessage)
        async for message in client.receive_response():
            if isinstance(message, ResultMessage):
                print(f"Interrupted task: terminal_reason={message.terminal_reason!r}")
                # terminal_reason is "aborted_streaming" or "aborted_tools"
                # for interrupted turns

        # Send a new command
        await client.query("Just say hello instead")

        # Now receive the new response
        async for message in client.receive_response():
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(f"New result: {message.result}")


asyncio.run(interruptible_task())
```

<Note>
  **Поведение буфера после прерывания:** `interrupt()` отправляет сигнал остановки, но не очищает буфер сообщений. Сообщения, уже созданные прерванной задачей, включая ее `ResultMessage`, остаются в потоке. Вы должны слить их с помощью `receive_response()` перед чтением ответа на новый запрос. Если вы отправите новый запрос сразу после `interrupt()` и вызовете `receive_response()` только один раз, вы получите сообщения прерванной задачи, а не ответ на новый запрос.
</Note>

<h4 id="example-advanced-permission-control">
  Пример - Расширенное управление разрешениями
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions
from claude_agent_sdk.types import (
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def custom_permission_handler(
    tool_name: str, input_data: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    """Custom logic for tool permissions."""

    # Block writes to system directories
    if tool_name == "Write" and input_data.get("file_path", "").startswith("/system/"):
        return PermissionResultDeny(
            message="System directory write not allowed", interrupt=True
        )

    # Redirect sensitive file operations
    if tool_name in ["Write", "Edit"] and "config" in input_data.get("file_path", ""):
        safe_path = f"./sandbox/{input_data['file_path']}"
        return PermissionResultAllow(
            updated_input={**input_data, "file_path": safe_path}
        )

    # Allow everything else
    return PermissionResultAllow(updated_input=input_data)


async def main():
    # Don't also list the gated tools in allowed_tools: allow rules approve calls before can_use_tool runs
    options = ClaudeAgentOptions(can_use_tool=custom_permission_handler)

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Update the system config file")

        async for message in client.receive_response():
            # Will use sandbox path instead
            print(message)


asyncio.run(main())
```

<h2 id="types">
  Типы
</h2>

<Note>
  **`@dataclass` vs `TypedDict`:** Этот SDK использует два вида типов. Классы, украшенные `@dataclass` (такие как `ResultMessage`, `AgentDefinition`, `TextBlock`), являются экземплярами объектов во время выполнения и поддерживают доступ к атрибутам: `msg.result`. Классы, определенные с помощью `TypedDict` (такие как `ThinkingConfigEnabled`, `McpStdioServerConfig`, `SyncHookJSONOutput`), являются **простыми словарями во время выполнения** и требуют доступа к ключам: `config["budget_tokens"]`, а не `config.budget_tokens`. Синтаксис вызова `ClassName(field=value)` работает для обоих, но только dataclasses создают объекты с атрибутами.
</Note>

<h3 id="sdkmcptool">
  `SdkMcpTool`
</h3>

Определение для SDK MCP tool, созданного с помощью декоратора `@tool`.

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| Свойство       | Тип                                             | Описание                                                                                                                 |
| :------------- | :---------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Уникальный идентификатор инструмента                                                                                     |
| `description`  | `str`                                           | Понятное описание                                                                                                        |
| `input_schema` | `type[T] \| dict[str, Any]`                     | Схема для валидации ввода                                                                                                |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]`      | Асинхронная функция, которая обрабатывает выполнение инструмента                                                         |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Дополнительные аннотации инструмента (например `readOnlyHint`, `destructiveHint`, `openWorldHint`, `maxResultSizeChars`) |

<h3 id="transport">
  `Transport`
</h3>

Абстрактный базовый класс для пользовательских реализаций транспорта. Используйте это для связи с процессом Claude через пользовательский канал (например, удаленное соединение вместо локального подпроцесса).

<Warning>
  Это низкоуровневый внутренний API. Интерфейс может измениться в будущих выпусках. Пользовательские реализации должны быть обновлены в соответствии с любыми изменениями интерфейса.
</Warning>

```python theme={null}
from abc import ABC, abstractmethod
from collections.abc import AsyncIterator
from typing import Any


class Transport(ABC):
    @abstractmethod
    async def connect(self) -> None: ...

    @abstractmethod
    async def write(self, data: str) -> None: ...

    @abstractmethod
    def read_messages(self) -> AsyncIterator[dict[str, Any]]: ...

    @abstractmethod
    async def close(self) -> None: ...

    @abstractmethod
    def is_ready(self) -> bool: ...

    @abstractmethod
    async def end_input(self) -> None: ...
```

| Метод             | Описание                                                                      |
| :---------------- | :---------------------------------------------------------------------------- |
| `connect()`       | Подключите транспорт и подготовьте к связи                                    |
| `write(data)`     | Напишите необработанные данные (JSON + новая строка) в транспорт              |
| `read_messages()` | Асинхронный итератор, который выдает разобранные JSON сообщения               |
| `close()`         | Закройте соединение и очистите ресурсы                                        |
| `is_ready()`      | Возвращает `True`, если транспорт может отправлять и получать                 |
| `end_input()`     | Закройте входной поток (например, закройте stdin для транспортов подпроцесса) |

Импорт: `from claude_agent_sdk import Transport`

<h3 id="claudeagentoptions">
  `ClaudeAgentOptions`
</h3>

Dataclass конфигурации для запросов Claude Code.

```python theme={null}
@dataclass
class ClaudeAgentOptions:
    tools: list[str] | ToolsPreset | None = None
    allowed_tools: list[str] = field(default_factory=list)
    system_prompt: str | SystemPromptPreset | SystemPromptCustom | SystemPromptFile | None = None
    mcp_servers: dict[str, McpServerConfig] | str | Path = field(default_factory=dict)
    strict_mcp_config: bool = False
    permission_mode: PermissionMode | None = None
    continue_conversation: bool = False
    resume: str | None = None
    session_id: str | None = None
    max_turns: int | None = None
    max_budget_usd: float | None = None
    disallowed_tools: list[str] = field(default_factory=list)
    model: str | None = None
    fallback_model: str | None = None
    betas: list[SdkBeta] = field(default_factory=list)
    output_format: dict[str, Any] | None = None
    permission_prompt_tool_name: str | None = None
    cwd: str | Path | None = None
    cli_path: str | Path | None = None
    settings: str | None = None
    add_dirs: list[str | Path] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    extra_args: dict[str, str | None] = field(default_factory=dict)
    max_buffer_size: int | None = None
    debug_stderr: Any = sys.stderr  # Deprecated
    stderr: Callable[[str], None] | None = None
    can_use_tool: CanUseTool | None = None
    hooks: dict[HookEvent, list[HookMatcher]] | None = None
    user: str | None = None
    include_partial_messages: bool = False
    include_hook_events: bool = False
    forward_subagent_text: bool = False
    fork_session: bool = False
    resume_session_at: str | None = None
    resume_drops_turn: str | None = None
    agents: dict[str, AgentDefinition] | None = None
    setting_sources: list[SettingSource] | None = None
    skills: list[str] | Literal["all"] | None = None
    sandbox: SandboxSettings | None = None
    plugins: list[SdkPluginConfig] = field(default_factory=list)
    max_thinking_tokens: int | None = None  # Deprecated: use thinking instead
    thinking: ThinkingConfig | None = None
    effort: EffortLevel | None = None
    enable_file_checkpointing: bool = False
    session_store: SessionStore | None = None
    session_store_flush: SessionStoreFlushMode = "batched"
    load_timeout_ms: int = 60_000
    task_budget: TaskBudget | None = None
```

| Свойство                      | Тип                                                                                   | По умолчанию                       | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :---------------------------- | :------------------------------------------------------------------------------------ | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                             | Конфигурация инструментов. Используйте `{"type": "preset", "preset": "claude_code"}` для инструментов Claude Code по умолчанию                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                               | Инструменты для автоматического одобрения без запроса. Это не ограничивает Claude только этими инструментами. Если вы назовете один из [инструментов отслеживания задач](/docs/ru/agent-sdk/todo-tracking#model-availability) здесь, Claude Code также включает сеанс. Другие неуказанные инструменты переходят к `permission_mode` и `can_use_tool`. Используйте `disallowed_tools` для блокировки инструментов. См. [Permissions](/docs/ru/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                     |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptCustom \| SystemPromptFile \| None`         | `None`                             | Конфигурация системной подсказки. Передайте строку для пользовательской подсказки, `{"type": "preset", "preset": "claude_code"}` для системной подсказки Claude Code с дополнительным `"append"`, `{"type": "custom", "prompt": "..."}` для пользовательской подсказки, которая также может установить `"snapshot"`, или `{"type": "file", "path": "..."}` для загрузки большой подсказки с диска. См. [`SystemPromptPreset`](#systempromptpreset), [`SystemPromptCustom`](#systempromptcustom) и [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                          |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                               | Конфигурации MCP server или путь к файлу конфигурации                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `strict_mcp_config`           | `bool`                                                                                | `False`                            | Когда `True`, используйте только servers, переданные в `mcp_servers`, и игнорируйте проект `.mcp.json`, параметры пользователя, MCP servers, предоставленные plugins, и [claude.ai connectors](/docs/ru/mcp#use-mcp-servers-from-claude-ai). Соответствует флагу CLI `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                             | Режим разрешений для использования инструментов                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `continue_conversation`       | `bool`                                                                                | `False`                            | Продолжить самый последний разговор                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `resume`                      | `str \| None`                                                                         | `None`                             | ID сеанса для возобновления                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `session_id`                  | `str \| None`                                                                         | `None`                             | Используйте определенный ID сеанса вместо автоматически сгенерированного. Должен быть действительным UUID. Не может быть объединен с `continue_conversation` или `resume`, если `fork_session` также не установлен                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `max_turns`                   | `int \| None`                                                                         | `None`                             | Максимальное количество агентских ходов (раунды использования инструментов)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                             | Остановите запрос, когда оценка стоимости на стороне клиента достигнет этого значения USD. Сравнивается с той же оценкой, что и `total_cost_usd`. Для предостережений точности и поведения сброса см. [Track cost and usage](/docs/ru/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                               | Инструменты для отклонения. Простое имя, такое как `"Bash"`, удаляет инструмент из контекста Claude. Правило с областью действия, такое как `"Bash(rm *)"`, оставляет инструмент доступным и отклоняет совпадающие вызовы в каждом режиме разрешений, включая `bypassPermissions`, для команды [как написано](/docs/ru/permissions#bash-rule-limits). См. [Permissions](/docs/ru/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                 |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                            | Включить отслеживание изменений файлов для перемотки. См. [File checkpointing](/docs/ru/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `model`                       | `str \| None`                                                                         | `None`                             | Псевдоним модели Claude или полное имя модели. См. [accepted values and provider-specific IDs](/docs/ru/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `fallback_model`              | `str \| None`                                                                         | `None`                             | Резервная модель для использования, если основная модель не работает. Принимает список, разделенный запятыми. Для руководства см. [Choose a model](/docs/ru/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                               | Функции бета-версии для включения. См. [`SdkBeta`](#sdkbeta) для доступных опций                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                             | Формат вывода для структурированных ответов (например, `{"type": "json_schema", "schema": {...}}`). См. [Structured outputs](/docs/ru/agent-sdk/structured-outputs) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                             | Имя MCP tool для подсказок разрешений                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                             | Текущий рабочий каталог                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                             | Пользовательский путь к исполняемому файлу Claude Code CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `settings`                    | `str \| None`                                                                         | `None`                             | Путь к файлу параметров или встроенная строка JSON                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                               | Дополнительные каталоги Claude может получить доступ. SDK передает каждую запись в Claude Code как `--add-dir`, поэтому с параметром `project` setting source Claude Code также [загружает skills, commands и subagents каталога](/docs/ru/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `env`                         | `dict[str, str]`                                                                      | `{}`                               | Переменные окружения, объединенные поверх унаследованного окружения процесса. См. [Environment variables](/docs/ru/env-vars) для переменных, которые читает базовый CLI, и [Handle slow or stalled API responses](#handle-slow-or-stalled-api-responses) для переменных, связанных с тайм-аутом. Установите `CLAUDE_AGENT_SDK_CLIENT_APP` для идентификации вашего приложения в заголовке User-Agent                                                                                                                                                                                                                                                                                                                                                                    |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                               | Дополнительные аргументы CLI для прямой передачи в CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                             | Максимальные байты при буферизации stdout CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                       | *Устарело* - SDK игнорирует это значение. Используйте обратный вызов `stderr` для вывода stderr CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                             | Функция обратного вызова для вывода stderr из CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                             | Функция обратного вызова разрешения инструмента, вызываемая только когда [permission flow](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) переходит к подсказке. Не вызывается для вызовов, автоматически одобренных `allowed_tools`, правилами разрешения или `permission_mode`. Правило разрешения не предварительно одобряет [действия, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves). См. [`CanUseTool`](#canusetool) для деталей                                                                                                                                                                                                                                                             |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                             | Конфигурации hooks для перехвата событий                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `user`                        | `str \| None`                                                                         | `None`                             | На платформах POSIX учетная запись пользователя ОС, под которой работает подпроцесс Claude Code. Claude Code сохраняет окружение родительского процесса, включая `HOME`, и работает в `cwd`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `include_partial_messages`    | `bool`                                                                                | `False`                            | Включить события потоковой передачи частичных сообщений. Когда включено, выдаются сообщения [`StreamEvent`](#streamevent)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `include_hook_events`         | `bool`                                                                                | `False`                            | Включить события жизненного цикла hooks в поток сообщений как объекты `HookEventMessage`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `forward_subagent_text`       | `bool`                                                                                | `False`                            | Переадресовать текст subagent и блоки мышления в поток сообщений. Без этой опции Claude Code выдает subagent `tool_use` и `tool_result` блоки, но не текст или мышление. Требует Python Agent SDK 0.2.140 или позже                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `fork_session`                | `bool`                                                                                | `False`                            | При возобновлении с `resume` разветвитесь на новый ID сеанса вместо продолжения исходного сеанса                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `resume_session_at`           | `str \| None`                                                                         | `None`                             | При возобновлении загрузите разговор только до и включая сообщение с этим UUID. Используйте с `resume`, и обычно `fork_session`, для ветвления с более ранней точки. Требует Python Agent SDK 0.2.137 или позже                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `resume_drops_turn`           | `str \| None`                                                                         | `None`                             | UUID пользовательского запроса, чей ход усечение `resume_session_at` отбрасывает. Когда установлено, CLI отказывает в возобновлении, если отброшенный диапазон содержит записи, не относящиеся к этому ходу. Требует Python Agent SDK 0.2.137 или позже и Claude Code v2.1.223 или позже; CLI, поставляемый с этими версиями SDK, удовлетворяет требованию Claude Code                                                                                                                                                                                                                                                                                                                                                                                             |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                             | Программно определенные subagents                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                               | Загрузите пользовательские plugins из локальных путей. См. [Plugins](/docs/ru/agent-sdk/plugins) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                             | Программно настройте поведение sandbox. См. [Sandbox settings](#sandboxsettings) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (CLI defaults: all sources) | Контролируйте, какие параметры файловой системы загружать. Передайте `[]` для отключения пользовательских, проектных и локальных параметров. С `skills` установленным и этим полем не установленным, загружаются только пользовательские и проектные источники. Установите `setting_sources` явно, чтобы сохранить локальные параметры. Управляемая политика конечной точки загружается независимо; параметры, управляемые сервером, загружаются, когда сеанс аутентифицируется с учетными данными организации на [подходящей конфигурации](/docs/ru/server-managed-settings#platform-availability). Для входов, читаемых независимо от этой опции, см. [What settingSources does not control](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control) |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                             | Skills, доступные сеансу. Передайте `"all"` для включения каждого обнаруженного skill, или список имен skills. Передавайте только точные имена. SDK отклоняет неправильно сформированные и имена в форме подстановочных знаков с `ValueError` перед запуском процесса Claude Code; эта проверка требует Python Agent SDK 0.2.129 или позже. Когда установлено, SDK автоматически добавляет инструмент Skill в `allowed_tools`. Если вы также передаете `tools`, включите `"Skill"` в этот список. См. [Skills](/docs/ru/agent-sdk/skills)                                                                                                                                                                                                                               |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                             | *Устарело* - Максимальные токены для блоков мышления. Вместо этого используйте `thinking`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                             | Управляет поведением расширенного мышления. Имеет приоритет над `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                             | Уровень усилий для глубины мышления. См. [adjust the effort level](/docs/ru/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `session_store`               | [`SessionStore`](/docs/ru/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                             | Зеркалируйте стенограммы сеансов во внешний бэкэнд, чтобы любой хост мог их возобновить. См. [Persist sessions to external storage](/docs/ru/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                        | Когда сбрасывать записи зеркальной стенограммы в `session_store`. `"batched"` сбрасывает один раз за ход или когда буфер заполняется; `"eager"` запускает фоновый сброс после каждого кадра. Игнорируется, когда `session_store` равен `None`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `load_timeout_ms`             | `int`                                                                                 | `60000`                            | Тайм-аут для каждого вызова `session_store.load()` и `list_subkeys()` во время материализации возобновления, в миллисекундах                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `task_budget`                 | `TaskBudget \| None`                                                                  | `None`                             | Бюджет токенов на стороне API. Отправляется как `output_config.task_budget` с заголовком бета-версии `task-budgets-2026-03-13`. Передайте `{"total": <int>}`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

<h4 id="handle-slow-or-stalled-api-responses">
  Обработка медленных или зависших ответов API
</h4>

Подпроцесс CLI читает несколько переменных окружения, которые управляют тайм-аутами API и обнаружением зависания. Передайте их через `ClaudeAgentOptions.env`:

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions

options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS`: тайм-аут для каждого запроса на клиенте Anthropic в миллисекундах. По умолчанию `600000`. Применяется к основному циклу и всем subagents.
* `CLAUDE_CODE_MAX_RETRIES`: максимальное количество повторных попыток API. По умолчанию `10`, ограничено `15`. Каждая повторная попытка получает свое собственное окно `API_TIMEOUT_MS`, поэтому наихудшее время стены примерно `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` плюс отступ. Для автоматических запусков, которым нужно ждать через более длительные сбои, установите [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/ru/errors#tune-retry-behavior): он повторяет ошибки емкости бесконечно и, на Claude Code v2.1.199 или позже, повышает значение по умолчанию для других переходных ошибок до `300` и удаляет ограничение на эту переменную.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: сторожевой таймер зависания для subagents. Пока сторожевой таймер потока включен, значение по умолчанию составляет `CLAUDE_STREAM_IDLE_TIMEOUT_MS` плюс 5 минут, что составляет `600000`, если вы не повысите эту переменную. Со сторожевым таймером потока выключенным, значение по умолчанию составляет `600000`. До v2.1.257 значение по умолчанию всегда было `600000`.

  Таймер сбрасывается при каждом событии потока. При зависании Claude Code прерывает subagent и сообщает о зависании родителю. Для фонового subagent он также отмечает задачу как неудачную и прикрепляет любой частичный результат.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` с `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: сторожевой таймер потока, который прерывает запрос, когда заголовки прибыли, но тело ответа перестает потоковать. Сторожевой таймер включен по умолчанию для всех поставщиков; установите `CLAUDE_ENABLE_STREAM_WATCHDOG=0` для отключения. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` по умолчанию `300000` и зажимается до этого минимума. После прерывания, [Automatic retries](/docs/ru/errors#automatic-retries) охватывает то, что Claude Code делает, на основе того, как далеко прошел ответ.

  Пока сторожевой таймер ждет ответа, который шлюз позади `ANTHROPIC_BASE_URL` держит открытым с помощью ping-пингов keep-alive, хост, который устанавливает `include_partial_messages`, продолжает получать `ping` [`StreamEvent`](#streamevent) сообщения. Читайте эти кадры как живость, а не тайм-аут сеанса на молчание. До v2.1.257 кадры останавливались через 5 минут после последнего реального события потока.

<h3 id="outputformat">
  `OutputFormat`
</h3>

Конфигурация для валидации структурированного вывода. Передайте это как `dict` в поле `output_format` на `ClaudeAgentOptions`:

```python theme={null}
# Expected dict shape for output_format
{
    "type": "json_schema",
    "schema": {...},  # Your JSON Schema definition
}
```

| Поле     | Обязательно | Описание                                              |
| :------- | :---------- | :---------------------------------------------------- |
| `type`   | Да          | Должно быть `"json_schema"` для валидации JSON Schema |
| `schema` | Да          | Определение JSON Schema для валидации вывода          |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

Конфигурация для использования предустановленной системной подсказки Claude Code с дополнительными добавлениями.

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
    snapshot: NotRequired[bool]
```

| Поле                       | Обязательно | Описание                                                                                                                                                                                                                                                                                                                                                           |
| :------------------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | Да          | Должно быть `"preset"` для использования предустановленной системной подсказки                                                                                                                                                                                                                                                                                     |
| `preset`                   | Да          | Должно быть `"claude_code"` для использования системной подсказки Claude Code                                                                                                                                                                                                                                                                                      |
| `append`                   | Нет         | Дополнительные инструкции для добавления к предустановленной системной подсказке                                                                                                                                                                                                                                                                                   |
| `exclude_dynamic_sections` | Нет         | Переместите контекст для каждого сеанса, такой как рабочий каталог, флаг git-repo и пути памяти, из системной подсказки в первое пользовательское сообщение. Улучшает повторное использование кэша подсказок между пользователями и машинами. См. [Modify system prompts](/docs/ru/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |
| `snapshot`                 | Нет         | Установите значение `False` для перестройки системной подсказки при каждом запросе вместо [повторного использования подсказки, которую сеанс записал при первом запросе](/docs/ru/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Требует `claude-agent-sdk` v0.2.153 или позже                                                           |

<h3 id="systempromptcustom">
  `SystemPromptCustom`
</h3>

Пользовательская системная подсказка в форме объекта, эквивалентная передаче строки как `system_prompt`, которая также может установить `snapshot`. Требует `claude-agent-sdk` v0.2.153 или позже.

```python theme={null}
class SystemPromptCustom(TypedDict):
    type: Literal["custom"]
    prompt: str
    snapshot: NotRequired[bool]
```

| Поле       | Обязательно | Описание                                                                                                                                               |
| :--------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`     | Да          | Должно быть `"custom"`                                                                                                                                 |
| `prompt`   | Да          | Текст системной подсказки. Передается в CLI как аргумент командной строки, поэтому [ограничения длины командной строки](#systempromptfile) применяются |
| `snapshot` | Нет         | То же, что [`SystemPromptPreset.snapshot`](#systempromptpreset), применяется к `prompt`                                                                |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

Конфигурация для загрузки пользовательской системной подсказки из файла вместо передачи ее в виде строки. SDK сопоставляет это с флагом CLI [`--system-prompt-file`](/docs/ru/cli-reference#system-prompt-flags). Используйте форму файла, когда подсказка большая: SDK передает строку `system_prompt` в argv подпроцесса CLI, что подвергается ограничениям длины командной строки ОС перед отправкой SDK любого запроса API. На Linux один аргумент длиннее примерно 128 КБ не работает при порождении процесса с `Argument list too long`. На Windows вся командная строка ограничена примерно 32 КБ, поэтому форма строки не работает при более низком пороге.

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| Поле   | Обязательно | Описание                                            |
| :----- | :---------- | :-------------------------------------------------- |
| `type` | Да          | Должно быть `"file"` для загрузки подсказки с диска |
| `path` | Да          | Путь к файлу, содержащему системную подсказку       |

<h3 id="settingsource">
  `SettingSource`
</h3>

Управляет тем, какие источники конфигурации на основе файловой системы загружает SDK.

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| Значение    | Описание                                                                            | Местоположение                |
| :---------- | :---------------------------------------------------------------------------------- | :---------------------------- |
| `"user"`    | Глобальные параметры пользователя                                                   | `~/.claude/settings.json`     |
| `"project"` | Общие параметры проекта (контролируемые версией)                                    | `.claude/settings.json`       |
| `"local"`   | Локальные параметры проекта, gitignored когда Claude Code сохраняет параметр в него | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Поведение по умолчанию
</h4>

Когда `setting_sources` опущено или `None` и `skills` не установлен, `query()` загружает те же параметры файловой системы, что и Claude Code CLI: пользовательские, проектные и локальные. С `skills` установленным, строка [`setting_sources`](#claudeagentoptions) описывает текущее значение по умолчанию. Управляемые параметры политики загружаются во всех случаях; параметры, управляемые сервером, загружаются, когда сеанс аутентифицируется с учетными данными организации на [подходящей конфигурации](/docs/ru/server-managed-settings#platform-availability). Для получения дополнительной информации см. [What settingSources does not control](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control).

<h4 id="why-use-setting_sources">
  Почему использовать setting\_sources
</h4>

**Отключить параметры файловой системы:**

```python theme={null}
# Do not load user, project, or local settings from disk
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    async for message in query(
        prompt="Analyze this code",
        options=ClaudeAgentOptions(
            setting_sources=[]
        ),
    ):
        print(message)


asyncio.run(main())
```

<Note>
  В Python SDK 0.1.59 и более ранних версиях пустой список рассматривался так же, как опущение опции, поэтому `setting_sources=[]` не отключал параметры файловой системы. Обновитесь до более новой версии, если вам нужно, чтобы пустой список вступил в силу. TypeScript SDK не затронут.
</Note>

**Загрузить только определенные источники параметров:**

```python theme={null}
# Load only project settings, ignore user and local
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    async for message in query(
        prompt="Run CI checks",
        options=ClaudeAgentOptions(
            setting_sources=["project"]  # Only .claude/settings.json
        ),
    ):
        print(message)


asyncio.run(main())
```

**Приложения только SDK:**

```python theme={null}
# Define everything programmatically.
# Pass [] to opt out of filesystem setting sources.
import asyncio
from claude_agent_sdk import AgentDefinition, ClaudeAgentOptions, query


async def main():
    async for message in query(
        prompt="Review this PR",
        options=ClaudeAgentOptions(
            setting_sources=[],
            agents={
                "code-reviewer": AgentDefinition(
                    description="Reviews code changes",
                    prompt="You are a code reviewer. Report issues in the diff.",
                ),
            },
            allowed_tools=["Read", "Grep", "Glob"],
        ),
    ):
        print(message)


asyncio.run(main())
```

Для загрузки инструкций проекта CLAUDE.md включите `"project"` в `setting_sources`. См. [Modify system prompts](/docs/ru/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions) для того, как загрузка CLAUDE.md взаимодействует с опциями системной подсказки.

<h4 id="settings-precedence">
  Приоритет параметров
</h4>

Когда загружаются несколько источников, параметры объединяются с этим приоритетом (от наивысшего к наименьшему):

1. Локальные параметры (`.claude/settings.local.json`)
2. Параметры проекта (`.claude/settings.json`)
3. Параметры пользователя (`~/.claude/settings.json`)

Программные опции, такие как `agents`, `allowed_tools` и `settings`, переопределяют параметры пользователя, проекта и локальной файловой системы. Управляемые параметры политики имеют приоритет над программными опциями.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Конфигурация для subagent, определенного программно.

```python theme={null}
@dataclass
class AgentDefinition:
    description: str
    prompt: str
    tools: list[str] | None = None
    disallowedTools: list[str] | None = None
    model: str | None = None
    skills: list[str] | None = None
    memory: Literal["user", "project", "local"] | None = None
    mcpServers: list[str | dict[str, Any]] | None = None
    initialPrompt: str | None = None
    maxTurns: int | None = None
    background: bool | None = None
    effort: EffortLevel | int | None = None
    permissionMode: PermissionMode | None = None
```

| Поле              | Обязательно | Описание                                                                                                                                                                                                                                                      |
| :---------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`     | Да          | Описание на естественном языке, когда использовать этого агента                                                                                                                                                                                               |
| `prompt`          | Да          | Системная подсказка агента                                                                                                                                                                                                                                    |
| `tools`           | Нет         | Массив разрешенных имен инструментов. Если опущено, наследует каждый [инструмент, доступный для subagents](/docs/ru/sub-agents#available-tools)                                                                                                                    |
| `disallowedTools` | Нет         | Массив имен инструментов для удаления из набора инструментов агента. Также принимаются шаблоны уровня MCP server: `mcp__server` или `mcp__server__*` удаляет каждый инструмент с этого сервера, и `mcp__*` удаляет каждый MCP tool с любого сервера           |
| `model`           | Нет         | Переопределение модели для этого агента. Принимает псевдоним, такой как `"sonnet"`, `"opus"`, `"haiku"` или `"inherit"`, или полный ID модели. Когда вы опускаете его, Claude Code выбирает модель в [порядке модели subagent](/docs/ru/sub-agents#choose-a-model) |
| `skills`          | Нет         | Список имен skills для предварительной загрузки в контекст агента при запуске. Неуказанные skills остаются вызываемыми через инструмент Skill                                                                                                                 |
| `memory`          | Нет         | Источник памяти для этого агента: `"user"`, `"project"` или `"local"`                                                                                                                                                                                         |
| `mcpServers`      | Нет         | MCP servers, доступные этому агенту. Каждая запись - это имя сервера или встроенный словарь `{name: config}`                                                                                                                                                  |
| `initialPrompt`   | Нет         | Автоматически отправляется как первый ход пользователя, когда этот агент работает как основной агент потока                                                                                                                                                   |
| `maxTurns`        | Нет         | Максимальное количество агентских ходов перед остановкой агента                                                                                                                                                                                               |
| `background`      | Нет         | Запустите этого агента как неблокирующую фоновую задачу при вызове                                                                                                                                                                                            |
| `effort`          | Нет         | Уровень усилий рассуждения для этого агента. Принимает именованный уровень или целое число. См. [`EffortLevel`](#effortlevel)                                                                                                                                 |
| `permissionMode`  | Нет         | Режим разрешений для выполнения инструментов в этом агенте. [Правила наследования subagent](/docs/ru/agent-sdk/permissions#available-modes) решают, когда он применяется. См. [`PermissionMode`](#permissionmode)                                                  |

<Note>
  Имена полей `AgentDefinition` используют camelCase, такие как `disallowedTools`, `permissionMode` и `maxTurns`. Эти имена напрямую соответствуют формату проводки, общему с TypeScript SDK. Это отличается от `ClaudeAgentOptions`, который использует Python snake\_case для эквивалентных полей верхнего уровня, таких как `disallowed_tools` и `permission_mode`. Поскольку `AgentDefinition` является dataclass, передача ключевого слова snake\_case вызывает `TypeError` во время конструирования.
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

Режимы разрешений для управления выполнением инструментов.

```python theme={null}
PermissionMode = Literal[
    "default",  # Standard permission behavior
    "acceptEdits",  # Auto-accept file edits
    "plan",  # Planning mode - explore without editing
    "dontAsk",  # Deny anything not pre-approved instead of prompting
    "bypassPermissions",  # Bypass permission checks; explicit ask rules still prompt (use with caution)
    "auto",  # Model classifier approves or denies permission prompts
]
```

<h3 id="effortlevel">
  `EffortLevel`
</h3>

Уровни усилий для руководства глубиной мышления.

```python theme={null}
EffortLevel = Literal[
    "low",  # Minimal thinking, fastest responses
    "medium",  # Moderate thinking
    "high",  # Deep reasoning
    "xhigh",  # Extended reasoning; falls back to "high" on models that don't support it
    "max",  # Maximum effort
]
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Псевдоним типа для функций обратного вызова разрешения инструмента.

```python theme={null}
CanUseTool = Callable[
    [str, dict[str, Any], ToolPermissionContext], Awaitable[PermissionResult]
]
```

Обратный вызов получает:

* `tool_name`: Имя вызываемого инструмента
* `input_data`: Входные параметры инструмента
* `context`: `ToolPermissionContext` с дополнительной информацией

Возвращает `PermissionResult` (либо `PermissionResultAllow`, либо `PermissionResultDeny`).

Обратный вызов - это замена SDK для интерактивной подсказки разрешения: он вызывается только когда [permission evaluation flow](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) разрешается в подсказку. Вызовы инструментов, уже одобренные записью `allowed_tools`, правилом параметров разрешения или режимом разрешения, таким как `acceptEdits` или `bypassPermissions`, никогда его не вызывают. Правило разрешения не предварительно одобряет [действия, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves); см. [How permissions are evaluated](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) для того, какие из них достигают обратного вызова и что происходит в режиме `dontAsk` и `auto`. Чтобы контролировать каждый вызов инструмента, используйте вместо этого [hook `PreToolUse`](/docs/ru/agent-sdk/hooks).

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Информация контекста, передаваемая в обратные вызовы разрешения инструмента.

```python theme={null}
@dataclass
class ToolPermissionContext:
    signal: Any | None = None  # Future: abort signal support
    suggestions: list[PermissionUpdate] = field(default_factory=list)
    tool_use_id: str | None = None
    agent_id: str | None = None
    blocked_path: str | None = None
    decision_reason: str | None = None
    title: str | None = None
    display_name: str | None = None
    description: str | None = None
```

| Поле              | Тип                      | Описание                                                                                                                                                                                                                                  |
| :---------------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`          | `Any \| None`            | Зарезервировано для будущей поддержки сигнала прерывания                                                                                                                                                                                  |
| `suggestions`     | `list[PermissionUpdate]` | Предложения обновления разрешений из CLI. Подсказки Bash включают предложение с назначением `localSettings`, поэтому возврат его в `updated_permissions` записывает правило в `.claude/settings.local.json` и сохраняется между сеансами. |
| `tool_use_id`     | `str \| None`            | Идентификатор конкретного вызова инструмента, для которого эта подсказка предназначена. Всегда заполняется при доставке в `can_use_tool`                                                                                                  |
| `agent_id`        | `str \| None`            | ID sub-agent, когда вызов происходит из subagent; `None` для основного агента                                                                                                                                                             |
| `blocked_path`    | `str \| None`            | Путь к файлу, который вызвал запрос разрешения, если применимо. Например, когда команда Bash пытается получить доступ к пути вне разрешенных каталогов                                                                                    |
| `decision_reason` | `str \| None`            | Причина, по которой был вызван этот запрос разрешения. Переадресовано из `permissionDecisionReason` hook PreToolUse, когда hook вернул `"ask"`                                                                                            |
| `title`           | `str \| None`            | Полное предложение запроса разрешения, такое как `Claude wants to read foo.txt`. Используйте как основной текст подсказки, если присутствует                                                                                              |
| `display_name`    | `str \| None`            | Короткая именная фраза для действия инструмента, такая как `Read file`, подходящая для меток кнопок                                                                                                                                       |
| `description`     | `str \| None`            | Понятный подзаголовок для пользовательского интерфейса разрешений                                                                                                                                                                         |

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Тип объединения для результатов обратного вызова разрешения.

```python theme={null}
PermissionResult = PermissionResultAllow | PermissionResultDeny
```

<h3 id="permissionresultallow">
  `PermissionResultAllow`
</h3>

Результат, указывающий, что вызов инструмента должен быть разрешен.

```python theme={null}
@dataclass
class PermissionResultAllow:
    behavior: Literal["allow"] = "allow"
    updated_input: dict[str, Any] | None = None
    updated_permissions: list[PermissionUpdate] | None = None
```

| Поле                  | Тип                              | По умолчанию | Описание                                           |
| :-------------------- | :------------------------------- | :----------- | :------------------------------------------------- |
| `behavior`            | `Literal["allow"]`               | `"allow"`    | Должно быть "allow"                                |
| `updated_input`       | `dict[str, Any] \| None`         | `None`       | Измененный ввод для использования вместо оригинала |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None`       | Обновления разрешений для применения               |

<h3 id="permissionresultdeny">
  `PermissionResultDeny`
</h3>

Результат, указывающий, что вызов инструмента должен быть отклонен.

```python theme={null}
@dataclass
class PermissionResultDeny:
    behavior: Literal["deny"] = "deny"
    message: str = ""
    interrupt: bool = False
```

| Поле        | Тип               | По умолчанию | Описание                                               |
| :---------- | :---------------- | :----------- | :----------------------------------------------------- |
| `behavior`  | `Literal["deny"]` | `"deny"`     | Должно быть "deny"                                     |
| `message`   | `str`             | `""`         | Сообщение, объясняющее, почему инструмент был отклонен |
| `interrupt` | `bool`            | `False`      | Следует ли прерывать текущее выполнение                |

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Конфигурация для программного обновления разрешений.

```python theme={null}
@dataclass
class PermissionUpdate:
    type: Literal[
        "addRules",
        "replaceRules",
        "removeRules",
        "setMode",
        "addDirectories",
        "removeDirectories",
    ]
    rules: list[PermissionRuleValue] | None = None
    behavior: Literal["allow", "deny", "ask"] | None = None
    mode: PermissionMode | None = None
    directories: list[str] | None = None
    destination: (
        Literal["userSettings", "projectSettings", "localSettings", "session"] | None
    ) = None
```

| Поле          | Тип                                       | Описание                                           |
| :------------ | :---------------------------------------- | :------------------------------------------------- |
| `type`        | `Literal[...]`                            | Тип операции обновления разрешений                 |
| `rules`       | `list[PermissionRuleValue] \| None`       | Правила для операций добавления/замены/удаления    |
| `behavior`    | `Literal["allow", "deny", "ask"] \| None` | Поведение для операций на основе правил            |
| `mode`        | `PermissionMode \| None`                  | Режим для операции setMode                         |
| `directories` | `list[str] \| None`                       | Каталоги для операций добавления/удаления каталога |
| `destination` | `Literal[...] \| None`                    | Где применить обновление разрешений                |

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

Правило для добавления, замены или удаления в обновлении разрешений.

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

<h3 id="toolspreset">
  `ToolsPreset`
</h3>

Конфигурация предустановленных инструментов для использования набора инструментов Claude Code по умолчанию.

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Управляет поведением расширенного мышления. Объединение трех конфигураций:

```python theme={null}
ThinkingDisplay = Literal["summarized", "omitted"]


class ThinkingConfigAdaptive(TypedDict):
    type: Literal["adaptive"]
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigEnabled(TypedDict):
    type: Literal["enabled"]
    budget_tokens: int
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigDisabled(TypedDict):
    type: Literal["disabled"]


ThinkingConfig = ThinkingConfigAdaptive | ThinkingConfigEnabled | ThinkingConfigDisabled
```

| Вариант    | Поля                               | Описание                                          |
| :--------- | :--------------------------------- | :------------------------------------------------ |
| `adaptive` | `type`, `display`                  | Claude адаптивно решает, когда думать             |
| `enabled`  | `type`, `budget_tokens`, `display` | Включить мышление с определенным бюджетом токенов |
| `disabled` | `type`                             | Отключить мышление                                |

Дополнительное поле `display` управляет тем, возвращается ли текст мышления `"summarized"` или `"omitted"`. На Claude Opus 4.7 и более поздних версиях API по умолчанию используется `"omitted"`, поэтому установите `"summarized"` для получения содержимого мышления в выводах [`ThinkingBlock`](#thinkingblock). Claude Code не отправляет `display` на Amazon Bedrock или Google Cloud's Agent Platform, поэтому на этих поставщиках Opus 4.7 и позже возвращают пустые выводы `ThinkingBlock` даже когда вы устанавливаете `display` на `"summarized"`.

Поскольку это классы `TypedDict`, они являются простыми словарями во время выполнения. Либо создавайте их как литералы словарей, либо вызывайте класс как конструктор; оба создают `dict`. Получайте доступ к полям с помощью `config["budget_tokens"]`, а не `config.budget_tokens`:

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions, ThinkingConfigEnabled

# Option 1: dict literal (recommended, no import needed)
options = ClaudeAgentOptions(thinking={"type": "enabled", "budget_tokens": 20000})

# Option 2: constructor-style (returns a plain dict)
config = ThinkingConfigEnabled(type="enabled", budget_tokens=20000)
print(config["budget_tokens"])  # 20000
# config.budget_tokens would raise AttributeError
```

<h3 id="taskbudget">
  `TaskBudget`
</h3>

Бюджет задач на стороне API в токенах, используется с полем `task_budget` в `ClaudeAgentOptions`.

```python theme={null}
class TaskBudget(TypedDict):
    total: int
```

| Поле    | Тип   | Описание                        |
| :------ | :---- | :------------------------------ |
| `total` | `int` | Общий бюджет токенов для задачи |

Поскольку это `TypedDict`, передайте его как простой dict, такой как `ClaudeAgentOptions(task_budget={"total": 50000})`.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Тип Literal для функций бета-версии SDK.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Используйте с полем `betas` в `ClaudeAgentOptions` для включения функций бета-версии.

<Warning>
  Бета-версия `context-1m-2025-08-07` снята с производства по состоянию на 30 апреля 2026 года. Передача этого заголовка с Claude Sonnet 4.5 или Sonnet 4 не имеет эффекта, и запросы, превышающие стандартное окно контекста 200k-токенов, возвращают ошибку. Чтобы использовать окно контекста 1M-токенов, перейдите на [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 или Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), которые включают контекст 1M по стандартной цене без требования заголовка бета-версии.
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

Конфигурация для SDK MCP servers, созданных с помощью `create_sdk_mcp_server()`.

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Тип объединения для конфигураций MCP server.

```python theme={null}
McpServerConfig = (
    McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig
)
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```python theme={null}
class McpStdioServerConfig(TypedDict):
    type: NotRequired[Literal["stdio"]]  # Optional for backwards compatibility
    command: str
    args: NotRequired[list[str]]
    env: NotRequired[dict[str, str]]
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```python theme={null}
class McpSSEServerConfig(TypedDict):
    type: Literal["sse"]
    url: str
    headers: NotRequired[dict[str, str]]
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```python theme={null}
class McpHttpServerConfig(TypedDict):
    type: Literal["http"]
    url: str
    headers: NotRequired[dict[str, str]]
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

Конфигурация MCP server, как сообщается [`get_mcp_status()`](#methods). Это объединение всех вариантов транспорта [`McpServerConfig`](#mcpserverconfig) плюс вариант `claudeai-proxy` только для вывода для servers, проксированных через claude.ai.

```python theme={null}
McpServerStatusConfig = (
    McpStdioServerConfig
    | McpSSEServerConfig
    | McpHttpServerConfig
    | McpSdkServerConfigStatus
    | McpClaudeAIProxyServerConfig
)
```

`McpSdkServerConfigStatus` - это сериализуемая форма [`McpSdkServerConfig`](#mcpsdkserverconfig) с только полями `type` (`"sdk"`) и `name` (`str`); встроенный `instance` опущен. `McpClaudeAIProxyServerConfig` имеет поля `type` (`"claudeai-proxy"`), `url` (`str`) и `id` (`str`).

<h3 id="mcpstatusresponse">
  `McpStatusResponse`
</h3>

Ответ от [`ClaudeSDKClient.get_mcp_status()`](#methods). Оборачивает список статусов сервера под ключом `mcpServers`.

```python theme={null}
class McpStatusResponse(TypedDict):
    mcpServers: list[McpServerStatus]
```

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Статус подключенного MCP server, содержащийся в [`McpStatusResponse`](#mcpstatusresponse).

```python theme={null}
class McpServerStatus(TypedDict):
    name: str
    status: McpServerConnectionStatus  # "connected" | "failed" | "needs-auth" | "pending" | "disabled"
    serverInfo: NotRequired[McpServerInfo]
    error: NotRequired[str]
    config: NotRequired[McpServerStatusConfig]
    scope: NotRequired[str]
    tools: NotRequired[list[McpToolInfo]]
```

| Поле         | Тип                                                             | Описание                                                                                                                                                                           |
| :----------- | :-------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`       | `str`                                                           | Имя сервера                                                                                                                                                                        |
| `status`     | `str`                                                           | Один из `"connected"`, `"failed"`, `"needs-auth"`, `"pending"` или `"disabled"`                                                                                                    |
| `serverInfo` | `dict` (опционально)                                            | Имя и версия сервера (`{"name": str, "version": str}`)                                                                                                                             |
| `error`      | `str` (опционально)                                             | Сообщение об ошибке, если серверу не удалось подключиться                                                                                                                          |
| `config`     | [`McpServerStatusConfig`](#mcpserverstatusconfig) (опционально) | Конфигурация сервера. Та же форма, что и [`McpServerConfig`](#mcpserverconfig) (stdio, SSE, HTTP или SDK), плюс вариант `claudeai-proxy` для servers, подключенных через claude.ai |
| `scope`      | `str` (опционально)                                             | Область конфигурации                                                                                                                                                               |
| `tools`      | `list` (опционально)                                            | Инструменты, предоставляемые этим сервером, каждый с полями `name`, `description` и `annotations`                                                                                  |

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Конфигурация для загрузки plugins в SDK.

```python theme={null}
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| Поле   | Тип                | Описание                                                                          |
| :----- | :----------------- | :-------------------------------------------------------------------------------- |
| `type` | `Literal["local"]` | Должно быть `"local"` (в настоящее время поддерживаются только локальные plugins) |
| `path` | `str`              | Абсолютный или относительный путь к каталогу plugin                               |

**Пример:**

```python theme={null}
plugins = [
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"},
]
```

Для полной информации о создании и использовании plugins см. [Plugins](/docs/ru/agent-sdk/plugins).

<h2 id="message-types">
  Типы сообщений
</h2>

<h3 id="message">
  `Message`
</h3>

Тип объединения всех возможных сообщений.

```python theme={null}
Message = (
    UserMessage
    | AssistantMessage
    | SystemMessage
    | ResultMessage
    | StreamEvent
    | RateLimitEvent
    | ConversationResetMessage
)
```

<h3 id="usermessage">
  `UserMessage`
</h3>

Сообщение пользовательского ввода.

```python theme={null}
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
    uuid: str | None = None
    parent_tool_use_id: str | None = None
    tool_use_result: dict[str, Any] | None = None
    origin: MessageOrigin | None = None
```

| Поле                 | Тип                         | Описание                                                                                                                                                                                                     |
| :------------------- | :-------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content`            | `str \| list[ContentBlock]` | Содержимое сообщения как текст или блоки содержимого                                                                                                                                                         |
| `uuid`               | `str \| None`               | Уникальный идентификатор сообщения                                                                                                                                                                           |
| `parent_tool_use_id` | `str \| None`               | ID использования инструмента, если это сообщение является ответом результата инструмента                                                                                                                     |
| `tool_use_result`    | `dict[str, Any] \| None`    | Данные результата инструмента, если применимо                                                                                                                                                                |
| `origin`             | `MessageOrigin \| None`     | Происхождение этого сообщения, заполняется при внедренных ходах, таких как уведомления о задачах и сообщения от коллег. `None`, когда CLI не атрибутировал его. Требуется Python Agent SDK 0.2.137 или позже |

SDK передает `tool_use_result` из CLI без изменений. Для инструмента на внешнем MCP сервере, результат которого содержит блоки `resource_link`, словарь имеет ключ `resourceLinks`, содержащий список словарей с ключами типа TypeScript [`SDKMcpResourceLink`](/docs/ru/agent-sdk/typescript#sdkmcpresourcelink). Claude получает каждую ссылку как строку текста в результате инструмента. Для отображения файлов, возвращенных сервером, читайте `resourceLinks` вместо анализа этого текста. Ключ `resourceLinks` требует Python Agent SDK 0.2.150 или позже и Claude Code v2.1.257 или позже; CLI, поставляемый с этой версией SDK, удовлетворяет требованию Claude Code.

CLI опускает ключ, когда результат не содержит ссылок и при результатах от подагентов. CLI сохраняет максимум 50 ссылок на результат и прекращает добавление ссылок, когда список достигает 64 КиБ сериализованного JSON. Инструмент, который вы определяете внутри процесса с помощью [`tool()`](#tool), никогда не создает ключ, потому что SDK преобразует его блоки `resource_link` в текст перед тем, как CLI увидит результат.

<h3 id="assistantmessage">
  `AssistantMessage`
</h3>

Сообщение ответа помощника с блоками содержимого.

```python theme={null}
@dataclass
class AssistantMessage:
    content: list[ContentBlock]
    model: str
    parent_tool_use_id: str | None = None
    error: AssistantMessageError | None = None
    usage: dict[str, Any] | None = None
    message_id: str | None = None
    stop_reason: str | None = None
    session_id: str | None = None
    uuid: str | None = None
```

| Поле                 | Тип                                                          | Описание                                                                                                 |
| :------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| `content`            | `list[ContentBlock]`                                         | Список блоков содержимого в ответе                                                                       |
| `model`              | `str`                                                        | Модель, которая создала ответ                                                                            |
| `parent_tool_use_id` | `str \| None`                                                | ID использования инструмента, если это вложенный ответ                                                   |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Тип ошибки, если ответ столкнулся с ошибкой                                                              |
| `usage`              | `dict[str, Any] \| None`                                     | Использование токенов для каждого сообщения (те же ключи, что и [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | ID сообщения API. Несколько сообщений из одного хода имеют одинаковый ID                                 |
| `stop_reason`        | `str \| None`                                                | Причина остановки из API (например `end_turn`, `tool_use`)                                               |
| `session_id`         | `str \| None`                                                | ID сеанса, к которому принадлежит это сообщение                                                          |
| `uuid`               | `str \| None`                                                | Уникальный идентификатор сообщения в пределах транскрипта сеанса                                         |

<h3 id="assistantmessageerror">
  `AssistantMessageError`
</h3>

Возможные типы ошибок для сообщений помощника.

```python theme={null}
AssistantMessageError = Literal[
    "authentication_failed",
    "billing_error",
    "rate_limit",
    "invalid_request",
    "server_error",
    "unknown",
]
```

Базовый процесс CLI может выдавать типы ошибок, которые этот Literal не перечисляет, такие как `max_output_tokens`. SDK передает значение без изменений, поэтому обрабатывайте строки вне этого списка так же, как вы обрабатываете `unknown`. Тип TypeScript [`SDKAssistantMessageError`](/docs/ru/agent-sdk/typescript#sdkassistantmessage) перечисляет полный набор значений, которые может выдать CLI.

<h3 id="systemmessage">
  `SystemMessage`
</h3>

Системное сообщение с метаданными.

```python theme={null}
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

<h3 id="resultmessage">
  `ResultMessage`
</h3>

Финальное сообщение результата с информацией о стоимости и использовании.

```python theme={null}
@dataclass
class ResultMessage:
    subtype: str
    duration_ms: int
    duration_api_ms: int
    is_error: bool
    num_turns: int
    session_id: str
    stop_reason: str | None = None
    total_cost_usd: float | None = None
    usage: dict[str, Any] | None = None
    result: str | None = None
    structured_output: Any = None
    model_usage: dict[str, ModelUsage] | None = None
    permission_denials: list[Any] | None = None
    deferred_tool_use: DeferredToolUse | None = None
    errors: list[str] | None = None
    api_error_status: int | None = None
    uuid: str | None = None
    terminal_reason: str | None = None
    origin: MessageOrigin | None = None
```

Поле `subtype` определяет, какие другие поля заполнены. Это одно из `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"` или `"error_max_structured_output_retries"`. Dataclass Python объединяет все варианты в одну форму, поэтому поля, которые не применяются к возвращаемому подтипу, имеют значение `None`.

Несколько полей содержат диагностические детали о том, как разговор закончился:

* `is_error`: `True`, когда разговор закончился в состоянии ошибки. Всегда `True` на подтипах `error_*`. На `subtype="success"` это `True`, когда финальный запрос модели не удался, что означает, что цикл агента завершился, но последний вызов API вернул ошибку.
* `api_error_status`: код состояния HTTP завершающей ошибки API. `None`, когда ход закончился без ошибки. Заполняется только на `subtype="success"`.
* `result`: текст финального сообщения помощника на `subtype="success"` или `None` на подтипах `error_*`. Когда `subtype="success"` и `is_error=True`, это содержит строку ошибки API, если она доступна, но может быть пустой, поэтому проверьте `api_error_status` и предыдущее содержимое `AssistantMessage` для деталей.
* `errors`: строки ошибок уровня цикла, такие как сообщение о максимальных ходах. Заполняется только на подтипах `error_*`.
* `terminal_reason`: почему цикл запроса закончился, например `"completed"`, `"max_turns"`, `"api_error"`, `"aborted_streaming"` или `"aborted_tools"`. Значение `"aborted_streaming"` или `"aborted_tools"` означает, что ход был прерван до завершения. Частые причины - [`interrupt()`](#claudesdkclient) и callback разрешения, возвращающий [`PermissionResultDeny`](#permissionresultdeny) с `interrupt=True`. `None` на версиях CLI, которые предшествуют полю, на результатах локальных команд, таких как `/voice` или `/usage`, которые обходят цикл запроса, или на синтезированных результатах ошибок, выданных при критическом отказе сеанса. Зеркалирует [`SDKResultMessage.terminal_reason`](/docs/ru/agent-sdk/typescript#sdkresultmessage) TypeScript SDK, который перечисляет полный набор значений.
* `origin`: происхождение пользовательского сообщения, которое запустило этот ход. В [режиме потоковой передачи входных данных](/docs/ru/agent-sdk/streaming-vs-single-mode) проверьте это, чтобы отличить результат вашего собственного запроса, где `origin` равен `None` или `{"kind": "human"}`, от результата внедренного хода, такого как уведомление фоновой задачи. Требуется Python Agent SDK 0.2.137 или позже.

Словарь `usage` охватывает только основной цикл агента и исключает подагентов и другие вложенные или вспомогательные вызовы модели. В [режиме потоковой передачи входных данных](/docs/ru/agent-sdk/streaming-vs-single-mode) значения указаны за ход. Предпочитайте `model_usage` для учета токенов и стоимости. Словарь `usage` содержит следующие ключи, если они присутствуют:

| Ключ                          | Тип   | Описание                                                                                                                                                                                                 |
| ----------------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | Входные токены, потребленные циклом агента верхнего уровня. [Токены подагента не включены](/docs/ru/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); используйте `model_usage` для учета всего дерева. |
| `output_tokens`               | `int` | Выходные токены, созданные циклом агента верхнего уровня. Токены подагента не включены.                                                                                                                  |
| `cache_creation_input_tokens` | `int` | Токены, используемые для создания новых записей кэша.                                                                                                                                                    |
| `cache_read_input_tokens`     | `int` | Токены, прочитанные из существующих записей кэша.                                                                                                                                                        |

Словарь `model_usage` отображает имена моделей на использование для каждой модели. Он охватывает каждый вызов модели, сделанный через конвейер запроса: основной цикл, подагентов и внутренние вызовы, такие как компактирование и агентов Workflow. Вспомогательные вызовы вне этого конвейера, такие как классификатор разрешений и запросы подсчета токенов, исключены из `model_usage`. Рассматривайте `model_usage` как оценку, а не как выписку по счету.

В [режиме потоковой передачи входных данных](/docs/ru/agent-sdk/streaming-vs-single-mode) `model_usage` и `total_cost_usd` являются кумулятивными по ходам, поэтому читайте последний результат вместо суммирования по результатам. Вызов, который возобновляет сеанс, также учитывает [итоги, восстановленные из более ранних вызовов сеанса](/docs/ru/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). См. [Track costs in streaming input mode](/docs/ru/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) для сбросов и [Recover totals after a session crash](/docs/ru/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) для обнуленных результатов.

Каждое значение в `model_usage` - это TypedDict `ModelUsage`, импортируемый через `from claude_agent_sdk.types import ModelUsage`. Его ключи используют camelCase, потому что SDK передает значение без изменений из базового процесса CLI, соответствуя типу TypeScript [`ModelUsage`](/docs/ru/agent-sdk/typescript#modelusage):

| Ключ                       | Тип     | Описание                                                                                                                                                                                                                                                                                                                 |
| -------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `inputTokens`              | `int`   | Входные токены для этой модели.                                                                                                                                                                                                                                                                                          |
| `outputTokens`             | `int`   | Выходные токены для этой модели.                                                                                                                                                                                                                                                                                         |
| `cacheReadInputTokens`     | `int`   | Токены чтения кэша для этой модели.                                                                                                                                                                                                                                                                                      |
| `cacheCreationInputTokens` | `int`   | Токены создания кэша для этой модели.                                                                                                                                                                                                                                                                                    |
| `webSearchRequests`        | `int`   | Запросы веб-поиска, сделанные этой моделью.                                                                                                                                                                                                                                                                              |
| `thinkingTokens`           | `int`   | Токены мышления, созданные этой моделью, уже подсчитанные в `outputTokens`. Отсутствуют до тех пор, пока ход не запустится на версии Claude Code, которая его записывает, и не объявлены в TypedDict, поэтому читайте его с `.get()`. Требуется Python Agent SDK 0.2.150 или позже, чей поставляемый CLI его записывает. |
| `costUSD`                  | `float` | Предполагаемая стоимость в USD для этой модели, вычисленная на стороне клиента. См. [Track cost and usage](/docs/ru/agent-sdk/cost-tracking) для предостережений выставления счетов.                                                                                                                                          |
| `contextWindow`            | `int`   | Размер окна контекста для этой модели.                                                                                                                                                                                                                                                                                   |
| `maxOutputTokens`          | `int`   | Максимальный лимит выходных токенов для этой модели.                                                                                                                                                                                                                                                                     |
| `canonicalModel`           | `str`   | Канонический ID модели, используемый для поиска цены. Может отличаться от необработанной строки модели, по которой индексируется запись, такой как ID или псевдоним, специфичный для поставщика. Не всегда присутствует.                                                                                                 |
| `provider`                 | `str`   | Поставщик API, который обслуживал эту модель, такой как `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle` или `gateway`. Не всегда присутствует.                                                                                                                                                    |

<h3 id="streamevent">
  `StreamEvent`
</h3>

Событие потока для частичных обновлений сообщений во время потоковой передачи. Получается только когда `include_partial_messages=True` в `ClaudeAgentOptions`. Импортируйте через `from claude_agent_sdk.types import StreamEvent`.

```python theme={null}
@dataclass
class StreamEvent:
    uuid: str
    session_id: str
    event: dict[str, Any]  # The raw Claude API stream event
    parent_tool_use_id: str | None = None
```

| Поле                 | Тип              | Описание                                                                                                                                                                    |
| :------------------- | :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `uuid`               | `str`            | Уникальный идентификатор для этого события                                                                                                                                  |
| `session_id`         | `str`            | Идентификатор сеанса                                                                                                                                                        |
| `event`              | `dict[str, Any]` | Необработанные данные события потока Claude API                                                                                                                             |
| `parent_tool_use_id` | `str \| None`    | Всегда `None`. События потока выдаются только для основного сеанса. Для атрибуции подагента используйте полные сообщения, такие как [`AssistantMessage`](#assistantmessage) |

<h3 id="ratelimitevent">
  `RateLimitEvent`
</h3>

Выдается при изменении статуса ограничения скорости (например, с `"allowed"` на `"allowed_warning"`). Используйте это для предупреждения пользователей перед достижением жесткого лимита или для отката, когда статус `"rejected"`.

```python theme={null}
@dataclass
class RateLimitEvent:
    rate_limit_info: RateLimitInfo
    uuid: str
    session_id: str
```

| Поле              | Тип                               | Описание                               |
| :---------------- | :-------------------------------- | :------------------------------------- |
| `rate_limit_info` | [`RateLimitInfo`](#ratelimitinfo) | Текущее состояние ограничения скорости |
| `uuid`            | `str`                             | Уникальный идентификатор события       |
| `session_id`      | `str`                             | Идентификатор сеанса                   |

<h3 id="ratelimitinfo">
  `RateLimitInfo`
</h3>

Состояние ограничения скорости, переносимое [`RateLimitEvent`](#ratelimitevent).

```python theme={null}
RateLimitStatus = Literal["allowed", "allowed_warning", "rejected"]
RateLimitType = Literal[
    "five_hour", "seven_day", "seven_day_opus", "seven_day_sonnet", "overage"
]


@dataclass
class RateLimitInfo:
    status: RateLimitStatus
    resets_at: int | None = None
    rate_limit_type: RateLimitType | None = None
    utilization: float | None = None
    overage_status: RateLimitStatus | None = None
    overage_resets_at: int | None = None
    overage_disabled_reason: str | None = None
    raw: dict[str, Any] = field(default_factory=dict)
```

| Поле                      | Тип                       | Описание                                                                                                                                                                     |
| :------------------------ | :------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`                  | `RateLimitStatus`         | Текущий статус, один из `"allowed"`, `"allowed_warning"` или `"rejected"`. `"allowed_warning"` означает приближение к лимиту; `"rejected"` означает, что лимит был достигнут |
| `resets_at`               | `int \| None`             | Временная метка Unix, когда окно ограничения скорости сбрасывается                                                                                                           |
| `rate_limit_type`         | `RateLimitType \| None`   | Какое окно ограничения скорости применяется                                                                                                                                  |
| `utilization`             | `float \| None`           | Доля потребленного ограничения скорости (0,0 до 1,0)                                                                                                                         |
| `overage_status`          | `RateLimitStatus \| None` | Статус использования переплаты по мере использования, если применимо                                                                                                         |
| `overage_resets_at`       | `int \| None`             | Временная метка Unix, когда окно переплаты сбрасывается                                                                                                                      |
| `overage_disabled_reason` | `str \| None`             | Почему переплата недоступна, если статус `"rejected"`                                                                                                                        |
| `raw`                     | `dict[str, Any]`          | Полный необработанный словарь из CLI, включая поля, не смоделированные выше                                                                                                  |

<h3 id="conversationresetmessage">
  `ConversationResetMessage`
</h3>

Выдается при замене разговора без завершения соединения, например после `/clear`. См. [Track costs in streaming input mode](/docs/ru/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) для того, как сброс влияет на текущие итоги на последующих объектах `ResultMessage`. Требуется Python Agent SDK 0.2.137 или позже.

```python theme={null}
@dataclass
class ConversationResetMessage:
    new_conversation_id: str
    uuid: str
    session_id: str
```

| Поле                  | Тип   | Описание                                                                                                                     |
| :-------------------- | :---- | :--------------------------------------------------------------------------------------------------------------------------- |
| `new_conversation_id` | `str` | Непрозрачный идентификатор для свежего разговора. Не `session_id` последующих сообщений; читайте это из следующего сообщения |
| `uuid`                | `str` | Уникальный идентификатор сообщения                                                                                           |
| `session_id`          | `str` | ID сеанса, который был сброшен. Сообщения после сброса содержат новый `session_id`                                           |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

Выдается при запуске фоновой задачи. Фоновая задача - это все, что отслеживается вне основного хода: фоновая команда Bash, наблюдение [Monitor](#monitor), подагент, порожденный через инструмент Agent, или удаленный агент. Поле `task_type` говорит вам, какой. Это именование не связано с переименованием инструмента `Task` на `Agent`.

```python theme={null}
@dataclass
class TaskStartedMessage(SystemMessage):
    task_id: str
    description: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    task_type: str | None = None
```

| Поле          | Тип           | Описание                                                                                                              |
| :------------ | :------------ | :-------------------------------------------------------------------------------------------------------------------- |
| `task_id`     | `str`         | Уникальный идентификатор для задачи                                                                                   |
| `description` | `str`         | Описание задачи                                                                                                       |
| `uuid`        | `str`         | Уникальный идентификатор сообщения                                                                                    |
| `session_id`  | `str`         | Идентификатор сеанса                                                                                                  |
| `tool_use_id` | `str \| None` | Связанный ID использования инструмента                                                                                |
| `task_type`   | `str \| None` | Какой вид фоновой задачи: `"local_bash"` для фонового Bash и наблюдений Monitor, `"local_agent"` или `"remote_agent"` |

<h3 id="taskusage">
  `TaskUsage`
</h3>

Данные токенов и времени для фоновой задачи.

```python theme={null}
class TaskUsage(TypedDict):
    total_tokens: int
    tool_uses: int
    duration_ms: int
```

<h3 id="taskprogressmessage">
  `TaskProgressMessage`
</h3>

Выдается периодически с обновлениями прогресса для выполняющейся фоновой задачи.

```python theme={null}
@dataclass
class TaskProgressMessage(SystemMessage):
    task_id: str
    description: str
    usage: TaskUsage
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    last_tool_name: str | None = None
```

| Поле             | Тип           | Описание                                                |
| :--------------- | :------------ | :------------------------------------------------------ |
| `task_id`        | `str`         | Уникальный идентификатор для задачи                     |
| `description`    | `str`         | Описание текущего статуса                               |
| `usage`          | `TaskUsage`   | Использование токенов для этой задачи до сих пор        |
| `uuid`           | `str`         | Уникальный идентификатор сообщения                      |
| `session_id`     | `str`         | Идентификатор сеанса                                    |
| `tool_use_id`    | `str \| None` | Связанный ID использования инструмента                  |
| `last_tool_name` | `str \| None` | Имя последнего инструмента, который использовала задача |

<h3 id="tasknotificationmessage">
  `TaskNotificationMessage`
</h3>

Выдается при завершении, сбое или остановке фоновой задачи. Фоновые задачи включают команды Bash `run_in_background`, наблюдения Monitor и фоновые подагенты.

```python theme={null}
@dataclass
class TaskNotificationMessage(SystemMessage):
    task_id: str
    status: TaskNotificationStatus  # "completed" | "failed" | "stopped"
    output_file: str
    summary: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    usage: TaskUsage | None = None
```

| Поле          | Тип                      | Описание                                          |
| :------------ | :----------------------- | :------------------------------------------------ |
| `task_id`     | `str`                    | Уникальный идентификатор для задачи               |
| `status`      | `TaskNotificationStatus` | Один из `"completed"`, `"failed"` или `"stopped"` |
| `output_file` | `str`                    | Путь к файлу вывода задачи                        |
| `summary`     | `str`                    | Резюме результата задачи                          |
| `uuid`        | `str`                    | Уникальный идентификатор сообщения                |
| `session_id`  | `str`                    | Идентификатор сеанса                              |
| `tool_use_id` | `str \| None`            | Связанный ID использования инструмента            |
| `usage`       | `TaskUsage \| None`      | Финальное использование токенов для задачи        |

Когда CLI [перемещает длительный вызов инструмента MCP в фон](/docs/ru/mcp#automatic-backgrounding-of-long-tool-calls), результат инструмента для этого вызова содержит только заполнитель, и реальный результат вызова поступает в это сообщение. При уведомлении `"completed"` для такого вызова CLI добавляет ключ `resource_links`, перечисляющий файлы, возвращенные инструментом по ссылке, с теми же записями и ограничениями, что и ключ `resourceLinks` на [`UserMessage.tool_use_result`](#usermessage). Ключ `resource_links` требует Python Agent SDK 0.2.150 или позже и Claude Code v2.1.257 или позже; CLI, поставляемый с этой версией SDK, удовлетворяет требованию Claude Code.

Dataclass не имеет поля для `resource_links`. Читайте его из словаря `data`, который сообщение наследует от [`SystemMessage`](#systemmessage): `message.data.get("resource_links")`. Сопоставьте уведомление с вызовом, используя `tool_use_id`. CLI опускает ключ, когда результат не содержал ссылок и при уведомлениях для задач, которые не являются вызовами инструментов MCP.

<h2 id="content-block-types">
  Типы блоков содержимого
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Тип объединения всех блоков содержимого.

```python theme={null}
ContentBlock = (
    TextBlock
    | ThinkingBlock
    | ToolUseBlock
    | ToolResultBlock
    | ServerToolUseBlock
    | ServerToolResultBlock
)
```

<h3 id="textblock">
  `TextBlock`
</h3>

Блок содержимого текста.

```python theme={null}
@dataclass
class TextBlock:
    text: str
```

<h3 id="thinkingblock">
  `ThinkingBlock`
</h3>

Блок содержимого мышления (для моделей с возможностью мышления).

```python theme={null}
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

<h3 id="tooluseblock">
  `ToolUseBlock`
</h3>

Блок запроса использования инструмента.

```python theme={null}
@dataclass
class ToolUseBlock:
    id: str
    name: str
    input: dict[str, Any]
```

<h3 id="toolresultblock">
  `ToolResultBlock`
</h3>

Блок результата выполнения инструмента.

```python theme={null}
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

<h2 id="error-types">
  Типы ошибок
</h2>

Типы ниже определяют, что ваш код перехватывает. Для записей, привязанных к сообщениям об ошибках, которые эти типы вызывают, с причиной и исправлением для каждого, см. [Troubleshooting](/docs/ru/agent-sdk/troubleshooting).

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Базовый класс исключения для всех ошибок SDK.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

Когда одноразовый `query()` заканчивается результатом ошибки, например ошибкой превышения лимита ходов, SDK вызывает [`ResultError`](#resulterror) после выдачи финального сообщения результата. Версии Python Agent SDK до 0.2.140 вызывали простое `Exception`, которое не было подклассом `ClaudeSDKError`.

<h3 id="clinotfounderror">
  `CLINotFoundError`
</h3>

Вызывается, когда Claude Code CLI не установлен или не найден.

```python theme={null}
class CLINotFoundError(CLIConnectionError):
    def __init__(
        self, message: str = "Claude Code not found", cli_path: str | None = None
    ):
        """
        Args:
            message: Error message (default: "Claude Code not found")
            cli_path: Optional path to the CLI that was not found
        """
```

<h3 id="cliconnectionerror">
  `CLIConnectionError`
</h3>

Вызывается, когда соединение с Claude Code не удается.

```python theme={null}
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

<h3 id="processerror">
  `ProcessError`
</h3>

Вызывается, когда процесс Claude Code не работает.

```python theme={null}
class ProcessError(ClaudeSDKError):
    def __init__(
        self, message: str, exit_code: int | None = None, stderr: str | None = None
    ):
        self.exit_code = exit_code
        self.stderr = stderr
```

<h3 id="resulterror">
  `ResultError`
</h3>

Вызывается после финального [`ResultMessage`](#resultmessage), когда процесс Claude Code завершается, потому что запуск закончился результатом ошибки, таким как ошибка превышения лимита ходов или ошибка API. `ResultError` является подклассом `ProcessError`, поэтому существующий обработчик `except ProcessError` также его перехватывает. Его атрибуты содержат поля этого сообщения результата, поэтому вы можете разветвляться в зависимости от причины сбоя запуска без анализа текста сообщения. Требуется Python Agent SDK версии 0.2.140 или позже.

```python theme={null}
class ResultError(ProcessError):
    subtype: str | None  # "error_max_turns", "error_during_execution", ...; "success" when the run ended on a failed request
    errors: list[str]  # an empty list when the result message reported none
    result: str | None
    api_error_status: int | None
    terminal_reason: str | None  # "max_turns", "api_error", ...; check this before subtype
    session_id: str | None
    data: dict[str, Any]  # the raw result message payload
```

Чтобы различить сбои, проверьте `terminal_reason` перед `subtype`. Когда финальный запрос не удается, например при ошибке API, Claude Code сообщает `subtype` `"success"` с причиной в `terminal_reason`, например `"api_error"`; когда лимит, который вы установили, завершает запуск, такой как `max_turns` или `max_budget_usd`, он сообщает подтип `error_*`.

<h3 id="clijsondecodeerror">
  `CLIJSONDecodeError`
</h3>

Вызывается, когда разбор JSON не удается.

```python theme={null}
class CLIJSONDecodeError(ClaudeSDKError):
    def __init__(self, line: str, original_error: Exception):
        """
        Args:
            line: The line that failed to parse
            original_error: The original JSON decode exception
        """
        self.line = line
        self.original_error = original_error
```

<h2 id="hook-types">
  Типы hooks
</h2>

Для полного руководства по использованию hooks с примерами и общими шаблонами см. [Hooks guide](/docs/ru/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Поддерживаемые типы событий hooks.

```python theme={null}
HookEvent = Literal[
    "PreToolUse",  # Called before tool execution
    "PostToolUse",  # Called after tool execution
    "PostToolUseFailure",  # Called when a tool execution fails
    "UserPromptSubmit",  # Called when user submits a prompt
    "Stop",  # Called when stopping execution
    "SubagentStop",  # Called when a subagent stops
    "PreCompact",  # Called before message compaction
    "Notification",  # Called for notification events
    "SubagentStart",  # Called when a subagent starts
    "PermissionRequest",  # Called when a permission decision is needed
]
```

<Note>
  TypeScript SDK поддерживает дополнительные события hooks, которые еще недоступны в Python. См. [таблицу доступности hooks](/docs/ru/agent-sdk/hooks#available-hooks) для поддержки по SDK.
</Note>

<h3 id="hookcallback">
  `HookCallback`
</h3>

Определение типа для функций обратного вызова hooks.

```python theme={null}
HookCallback = Callable[[HookInput, str | None, HookContext], Awaitable[HookJSONOutput]]
```

Параметры:

* `input`: Строго типизированный ввод hooks с дискриминированными объединениями на основе `hook_event_name` (см. [`HookInput`](#hookinput))
* `tool_use_id`: Дополнительный идентификатор использования инструмента (для hooks, связанных с инструментами)
* `context`: Контекст hooks с дополнительной информацией

Возвращает [`HookJSONOutput`](#hookjsonoutput).

<h3 id="hookcontext">
  `HookContext`
</h3>

Информация контекста, передаваемая в обратные вызовы hooks.

```python theme={null}
class HookContext(TypedDict):
    signal: Any | None  # Future: abort signal support
```

<h3 id="hookmatcher">
  `HookMatcher`
</h3>

Конфигурация для сопоставления hooks с определенными событиями или инструментами.

```python theme={null}
@dataclass
class HookMatcher:
    matcher: str | None = (
        None  # Tool name or pattern to match (e.g., "Bash", "Write|Edit")
    )
    hooks: list[HookCallback] = field(
        default_factory=list
    )  # List of callbacks to execute
    timeout: float | None = (
        None  # Timeout in seconds. When omitted, the per-event default applies:
        # 600 for most events, 30 for UserPromptSubmit
    )
```

<h3 id="hookinput">
  `HookInput`
</h3>

Тип объединения всех типов ввода hooks. Фактический тип зависит от поля `hook_event_name`.

```python theme={null}
HookInput = (
    PreToolUseHookInput
    | PostToolUseHookInput
    | PostToolUseFailureHookInput
    | UserPromptSubmitHookInput
    | StopHookInput
    | SubagentStopHookInput
    | PreCompactHookInput
    | NotificationHookInput
    | SubagentStartHookInput
    | PermissionRequestHookInput
)
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

Базовые поля, присутствующие во всех типах ввода hooks.

```python theme={null}
class BaseHookInput(TypedDict):
    session_id: str
    transcript_path: str
    cwd: str
    permission_mode: NotRequired[str]
```

| Поле              | Тип                 | Описание                        |
| :---------------- | :------------------ | :------------------------------ |
| `session_id`      | `str`               | Текущий идентификатор сеанса    |
| `transcript_path` | `str`               | Путь к файлу стенограммы сеанса |
| `cwd`             | `str`               | Текущий рабочий каталог         |
| `permission_mode` | `str` (опционально) | Текущий режим разрешений        |

<h3 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h3>

Входные данные для событий hooks `PreToolUse`.

```python theme={null}
class PreToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PreToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Поле              | Тип                     | Описание                                                                        |
| :---------------- | :---------------------- | :------------------------------------------------------------------------------ |
| `hook_event_name` | `Literal["PreToolUse"]` | Всегда "PreToolUse"                                                             |
| `tool_name`       | `str`                   | Имя инструмента, который вот-вот будет выполнен                                 |
| `tool_input`      | `dict[str, Any]`        | Входные параметры для инструмента                                               |
| `tool_use_id`     | `str`                   | Уникальный идентификатор для этого использования инструмента                    |
| `agent_id`        | `str` (опционально)     | Идентификатор подагента, присутствует, когда hooks срабатывает внутри подагента |
| `agent_type`      | `str` (опционально)     | Тип подагента, присутствует, когда hooks срабатывает внутри подагента           |

<h3 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h3>

Входные данные для событий hooks `PostToolUse`.

```python theme={null}
class PostToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_response: Any
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Поле              | Тип                      | Описание                                                                        |
| :---------------- | :----------------------- | :------------------------------------------------------------------------------ |
| `hook_event_name` | `Literal["PostToolUse"]` | Всегда "PostToolUse"                                                            |
| `tool_name`       | `str`                    | Имя выполненного инструмента                                                    |
| `tool_input`      | `dict[str, Any]`         | Использованные входные параметры                                                |
| `tool_response`   | `Any`                    | Ответ от выполнения инструмента                                                 |
| `tool_use_id`     | `str`                    | Уникальный идентификатор для этого использования инструмента                    |
| `agent_id`        | `str` (опционально)      | Идентификатор подагента, присутствует, когда hooks срабатывает внутри подагента |
| `agent_type`      | `str` (опционально)      | Тип подагента, присутствует, когда hooks срабатывает внутри подагента           |

<h3 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h3>

Входные данные для событий hooks `PostToolUseFailure`. Вызывается, когда выполнение инструмента не удается.

```python theme={null}
class PostToolUseFailureHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUseFailure"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    error: str
    is_interrupt: NotRequired[bool]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Поле              | Тип                             | Описание                                                                                                                                                                                                                                                     |
| :---------------- | :------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Всегда "PostToolUseFailure"                                                                                                                                                                                                                                  |
| `tool_name`       | `str`                           | Имя инструмента, который не удался                                                                                                                                                                                                                           |
| `tool_input`      | `dict[str, Any]`                | Использованные входные параметры                                                                                                                                                                                                                             |
| `tool_use_id`     | `str`                           | Уникальный идентификатор для этого использования инструмента                                                                                                                                                                                                 |
| `error`           | `str`                           | Сообщение об ошибке из неудачного выполнения                                                                                                                                                                                                                 |
| `is_interrupt`    | `bool` (опционально)            | True, когда ошибка достигла Claude Code как прерывание, а не как ошибка, которую сообщил инструмент. Отмена выполняющегося инструмента с помощью `interrupt()` не срабатывает этот hooks; результат инструмента содержит сообщение о прерывании вместо этого |
| `agent_id`        | `str` (опционально)             | Идентификатор подагента, присутствует, когда hooks срабатывает внутри подагента                                                                                                                                                                              |
| `agent_type`      | `str` (опционально)             | Тип подагента, присутствует, когда hooks срабатывает внутри подагента                                                                                                                                                                                        |

<h3 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h3>

Входные данные для событий hooks `UserPromptSubmit`.

```python theme={null}
class UserPromptSubmitHookInput(BaseHookInput):
    hook_event_name: Literal["UserPromptSubmit"]
    prompt: str
```

| Поле              | Тип                           | Описание                             |
| :---------------- | :---------------------------- | :----------------------------------- |
| `hook_event_name` | `Literal["UserPromptSubmit"]` | Всегда "UserPromptSubmit"            |
| `prompt`          | `str`                         | Отправленная пользователем подсказка |

<h3 id="stophookinput">
  `StopHookInput`
</h3>

Входные данные для событий hooks `Stop`.

```python theme={null}
class StopHookInput(BaseHookInput):
    hook_event_name: Literal["Stop"]
    stop_hook_active: bool
```

| Поле               | Тип               | Описание                   |
| :----------------- | :---------------- | :------------------------- |
| `hook_event_name`  | `Literal["Stop"]` | Всегда "Stop"              |
| `stop_hook_active` | `bool`            | Активен ли hooks остановки |

<h3 id="subagentstophookinput">
  `SubagentStopHookInput`
</h3>

Входные данные для событий hooks `SubagentStop`.

```python theme={null}
class SubagentStopHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStop"]
    stop_hook_active: bool
    agent_id: str
    agent_transcript_path: str
    agent_type: str
```

| Поле                    | Тип                       | Описание                           |
| :---------------------- | :------------------------ | :--------------------------------- |
| `hook_event_name`       | `Literal["SubagentStop"]` | Всегда "SubagentStop"              |
| `stop_hook_active`      | `bool`                    | Активен ли hooks остановки         |
| `agent_id`              | `str`                     | Уникальный идентификатор подагента |
| `agent_transcript_path` | `str`                     | Путь к файлу стенограммы подагента |
| `agent_type`            | `str`                     | Тип подагента                      |

<h3 id="precompacthookinput">
  `PreCompactHookInput`
</h3>

Входные данные для событий hooks `PreCompact`.

```python theme={null}
class PreCompactHookInput(BaseHookInput):
    hook_event_name: Literal["PreCompact"]
    trigger: Literal["manual", "auto"]
    custom_instructions: str | None
```

| Поле                  | Тип                         | Описание                                   |
| :-------------------- | :-------------------------- | :----------------------------------------- |
| `hook_event_name`     | `Literal["PreCompact"]`     | Всегда "PreCompact"                        |
| `trigger`             | `Literal["manual", "auto"]` | Что вызвало уплотнение                     |
| `custom_instructions` | `str \| None`               | Пользовательские инструкции для уплотнения |

<h3 id="notificationhookinput">
  `NotificationHookInput`
</h3>

Входные данные для событий hooks `Notification`.

```python theme={null}
class NotificationHookInput(BaseHookInput):
    hook_event_name: Literal["Notification"]
    message: str
    title: NotRequired[str]
    notification_type: str
```

| Поле                | Тип                       | Описание                         |
| :------------------ | :------------------------ | :------------------------------- |
| `hook_event_name`   | `Literal["Notification"]` | Всегда "Notification"            |
| `message`           | `str`                     | Содержимое сообщения уведомления |
| `title`             | `str` (опционально)       | Название уведомления             |
| `notification_type` | `str`                     | Тип уведомления                  |

<h3 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h3>

Входные данные для событий hooks `SubagentStart`.

```python theme={null}
class SubagentStartHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStart"]
    agent_id: str
    agent_type: str
```

| Поле              | Тип                        | Описание                           |
| :---------------- | :------------------------- | :--------------------------------- |
| `hook_event_name` | `Literal["SubagentStart"]` | Всегда "SubagentStart"             |
| `agent_id`        | `str`                      | Уникальный идентификатор подагента |
| `agent_type`      | `str`                      | Тип подагента                      |

<h3 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h3>

Входные данные для событий hooks `PermissionRequest`. Позволяет hooks программно обрабатывать решения разрешений.

```python theme={null}
class PermissionRequestHookInput(BaseHookInput):
    hook_event_name: Literal["PermissionRequest"]
    tool_name: str
    tool_input: dict[str, Any]
    permission_suggestions: NotRequired[list[Any]]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Поле                     | Тип                            | Описание                                                                        |
| :----------------------- | :----------------------------- | :------------------------------------------------------------------------------ |
| `hook_event_name`        | `Literal["PermissionRequest"]` | Всегда "PermissionRequest"                                                      |
| `tool_name`              | `str`                          | Имя инструмента, запрашивающего разрешение                                      |
| `tool_input`             | `dict[str, Any]`               | Входные параметры для инструмента                                               |
| `permission_suggestions` | `list[Any]` (опционально)      | Предложенные обновления разрешений из CLI                                       |
| `agent_id`               | `str` (опционально)            | Идентификатор подагента, присутствует, когда hooks срабатывает внутри подагента |
| `agent_type`             | `str` (опционально)            | Тип подагента, присутствует, когда hooks срабатывает внутри подагента           |

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Тип объединения для возвращаемых значений обратного вызова hooks.

```python theme={null}
HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

Синхронный вывод hooks с полями управления и решения.

```python theme={null}
class SyncHookJSONOutput(TypedDict):
    # Control fields
    continue_: NotRequired[bool]  # Whether to proceed (default: True)
    suppressOutput: NotRequired[bool]  # Hide stdout from transcript
    stopReason: NotRequired[str]  # Message when continue is False

    # Decision fields
    decision: NotRequired[Literal["block"]]
    systemMessage: NotRequired[str]  # Warning message for user
    reason: NotRequired[str]  # Feedback for Claude

    # Hook-specific output
    hookSpecificOutput: NotRequired[HookSpecificOutput]
```

<Note>
  Используйте `continue_` (с подчеркиванием) в коде Python. Он автоматически преобразуется в `continue` при отправке в CLI.
</Note>

<h4 id="hookspecificoutput">
  `HookSpecificOutput`
</h4>

Дискриминированное объединение типов вывода, специфичных для события. Поле `hookEventName` определяет, какие поля действительны. Для полных деталей доступных полей для каждого события hooks см. [Control execution with hooks](/docs/ru/agent-sdk/hooks#outputs).

```python theme={null}
class PreToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PreToolUse"]
    permissionDecision: NotRequired[Literal["allow", "deny", "ask", "defer"]]
    permissionDecisionReason: NotRequired[str]
    updatedInput: NotRequired[dict[str, Any]]
    additionalContext: NotRequired[str]


class PostToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUse"]
    additionalContext: NotRequired[str]
    updatedToolOutput: NotRequired[Any]
    updatedMCPToolOutput: NotRequired[Any]  # Deprecated: use updatedToolOutput, which works for all tools


class PostToolUseFailureHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUseFailure"]
    additionalContext: NotRequired[str]


class UserPromptSubmitHookSpecificOutput(TypedDict):
    hookEventName: Literal["UserPromptSubmit"]
    additionalContext: NotRequired[str]


class NotificationHookSpecificOutput(TypedDict):
    hookEventName: Literal["Notification"]
    additionalContext: NotRequired[str]


class SubagentStartHookSpecificOutput(TypedDict):
    hookEventName: Literal["SubagentStart"]
    additionalContext: NotRequired[str]


class PermissionRequestHookSpecificOutput(TypedDict):
    hookEventName: Literal["PermissionRequest"]
    decision: dict[str, Any]


HookSpecificOutput = (
    PreToolUseHookSpecificOutput
    | PostToolUseHookSpecificOutput
    | PostToolUseFailureHookSpecificOutput
    | UserPromptSubmitHookSpecificOutput
    | NotificationHookSpecificOutput
    | SubagentStartHookSpecificOutput
    | PermissionRequestHookSpecificOutput
)
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

Асинхронный вывод hooks, который откладывает выполнение hooks.

```python theme={null}
class AsyncHookJSONOutput(TypedDict):
    async_: Literal[True]  # Set to True to defer execution
    asyncTimeout: NotRequired[int]  # Timeout in milliseconds
```

<Note>
  Используйте `async_` (с подчеркиванием) в коде Python. Он автоматически преобразуется в `async` при отправке в CLI.
</Note>

<h3 id="hook-usage-example">
  Пример использования hooks
</h3>

Этот пример регистрирует два hooks: один, который блокирует опасные команды bash, такие как `rm -rf /`, и другой, который регистрирует все использование инструментов для аудита. Hooks безопасности работает только на командах Bash (через `matcher`), в то время как hooks логирования работает на всех инструментах.

```python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, HookContext
from typing import Any


async def validate_bash_command(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Validate and potentially block dangerous bash commands."""
    if input_data["tool_name"] == "Bash":
        command = input_data["tool_input"].get("command", "")
        if "rm -rf /" in command:
            return {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": "Dangerous command blocked",
                }
            }
    return {}


async def log_tool_use(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage for auditing."""
    print(f"Tool used: {input_data.get('tool_name')}")
    return {}


options = ClaudeAgentOptions(
    hooks={
        "PreToolUse": [
            HookMatcher(
                matcher="Bash", hooks=[validate_bash_command], timeout=120
            ),  # 2 min for validation
            HookMatcher(
                hooks=[log_tool_use]
            ),  # Applies to all tools (per-event default timeout)
        ],
        "PostToolUse": [HookMatcher(hooks=[log_tool_use])],
    }
)

async def main():
    async for message in query(prompt="Analyze this codebase", options=options):
        print(message)


asyncio.run(main())
```

<h2 id="tool-input/output-types">
  Типы ввода/вывода инструментов
</h2>

Документация схем ввода/вывода для всех встроенных инструментов Claude Code. Хотя Python SDK не экспортирует их как типы, они представляют структуру входов и выходов инструментов в сообщениях.

<h3 id="agent">
  Agent
</h3>

**Имя инструмента:** `Agent`. Предыдущее имя `Task` по-прежнему принимается как псевдоним, и список `tools` в инициализации [`SystemMessage`](#systemmessage) сообщает об этом инструменте как `Task` для обратной совместимости.

**Ввод:**

```python theme={null}
{
    "description": str,  # Краткое описание задачи (3-5 слов)
    "prompt": str,  # Задача для выполнения агентом
    "subagent_type": str | None,  # Тип специализированного агента для использования
    "model": "sonnet" | "opus" | "haiku" | "fable" | None,  # Переопределение модели для этого агента
    "run_in_background": bool | None,  # Агенты запускаются в фоне по умолчанию; установите значение False для синхронного выполнения
    "name": str | None,  # Имя для порожденного агента
    "team_name": str | None,  # Устарело; игнорируется
    "mode": "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan" | None,  # Устарело; игнорируется. Правила наследования подагента определяют режим разрешений подагента
    "isolation": "worktree" | "remote" | None,  # Режим изоляции для изменений агента
}
```

Запускает нового агента для автономной обработки сложных многошаговых задач.

**Вывод (статус: `"completed"`):**

```python theme={null}
{
    "status": "completed",
    "agentId": str,  # ID агента, который был запущен
    "agentType": str | None,  # Тип подагента, который обработал задачу
    "content": [  # Блоки содержимого результата
        {
            "type": "text",
            "text": str,
            "citations": list | None,
        }
    ],
    "resolvedModel": str | None,  # Модель, на которой запустился подагент
    "modelsUsed": list[str] | None,  # Модели, использованные по порядку, с коллапсированными последовательными повторениями
    "totalToolUseCount": int,  # Количество вызовов инструментов, которые сделал агент
    "totalDurationMs": int,  # Длительность выполнения в миллисекундах
    "totalTokens": int,  # Количество токенов из финального запроса API, не из всего запуска
    "usage": {  # Статистика использования токенов
        "input_tokens": int,
        "output_tokens": int,
        "cache_creation_input_tokens": int | None,
        "cache_read_input_tokens": int | None,
        "server_tool_use": {"web_search_requests": int, "web_fetch_requests": int} | None,
        "service_tier": str | None,
        "cache_creation": {"ephemeral_1h_input_tokens": int, "ephemeral_5m_input_tokens": int} | None,
        "inference_geo": str | None,
        "speed": str | None,
        "iterations": Any | None,
        "output_tokens_details": {"thinking_tokens": int | None} | None,
    },
    "toolStats": {  # Совокупная активность инструментов для запуска
        "readCount": int,
        "searchCount": int,
        "bashCount": int,
        "editFileCount": int,
        "linesAdded": int,
        "linesRemoved": int,
        "otherToolCount": int,
        "frameCount": int | None,
    } | None,
    "prompt": str,  # Подсказка, которую запустил агент
    "worktreePath": str | None,  # Присутствует, когда Claude Code сохранил worktree подагента
    "worktreeBranch": str | None,  # Присутствует, когда Claude Code создал этот worktree с git
}
```

**Вывод (статус: `"async_launched"`):**

```python theme={null}
{
    "status": "async_launched",
    "isAsync": bool | None,  # True при фоновых запусках
    "agentId": str,  # ID запущенного агента
    "description": str,  # Описание задачи
    "resolvedModel": str | None,  # Модель, используемая при переходе в фоновый режим
    "modelsUsed": list[str] | None,  # Модели, использованные перед фоновым режимом, по порядку, с коллапсированными последовательными повторениями
    "prompt": str,  # Подсказка, которую запускает агент
    "outputFile": str,  # Путь файла, где записывается вывод агента
    "canReadOutputFile": bool | None,  # Может ли выходной файл быть прочитан напрямую
}
```

**Вывод (статус: `"remote_launched"`):**

```python theme={null}
{
    "status": "remote_launched",
    "taskId": str,  # ID удаленной задачи
    "sessionUrl": str,  # Ссылка на удаленный облачный сеанс
    "description": str,  # Описание задачи
    "prompt": str,  # Подсказка, которую запускает агент
    "outputFile": str,  # Путь файла, где записывается вывод агента
}
```

Возвращает результат от подагента. Вывод различается по полю `status`: `"completed"` для завершенных задач, `"async_launched"` для фоновых задач и `"remote_launched"` для задач, которые Claude Code отправил в удаленный облачный сеанс, где `sessionUrl` ссылается на этот сеанс и `taskId` его идентифицирует. Если Claude Code [сохранил изолированный worktree подагента](/docs/ru/worktrees#isolate-subagents-with-worktrees), `worktreePath` в варианте `completed` — это место, где его найти, и `worktreeBranch` — это его ветка, когда Claude Code создал worktree с git.

В варианте `completed`, `resolvedModel` называет модель, на которой запустился подагент, которая может отличаться от запрошенного входа `model`, когда применяется [`availableModels`](/docs/ru/model-config#restrict-model-selection) или другое переопределение. Это поле требует Claude Code v2.1.174 или позже. В варианте `async_launched`, `resolvedModel` называет модель, используемую, когда агент перешел в фоновый режим, поэтому обмен, произошедший перед фоновым режимом, отражается там. Поле `modelsUsed` в обоих вариантах перечисляет использованные модели по порядку, с коллапсированными последовательными повторениями; оно устанавливается только при обмене модели во время выполнения. `modelsUsed` и поведение `resolvedModel` во время фонового режима требуют Claude Code v2.1.212 или позже.

Claude Code заполняет `usage` и `totalTokens` из финального запроса API подагента, не из всего запуска. Когда присутствует, `thinking_tokens` под `output_tokens_details` в `usage` — это количество выходных токенов этого запроса, которые были токенами мышления. Ключ `output_tokens_details` требует Python SDK v0.2.136 или позже, который поставляется с Claude Code v2.1.228.

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Имя инструмента:** `AskUserQuestion`

Задает пользователю уточняющие вопросы во время выполнения. См. [Обработка одобрений и ввода пользователя](/docs/ru/agent-sdk/user-input#handle-clarifying-questions) для деталей использования.

**Ввод:**

```python theme={null}
{
    "questions": [  # Вопросы для пользователя (1-4 вопроса)
        {
            "question": str,  # Полный вопрос для пользователя
            "header": str,  # Очень короткая метка, отображаемая как чип/тег (макс 12 символов)
            "options": [  # Доступные варианты (2-4 варианта)
                {
                    "label": str,  # Текст отображения для этого варианта (1-5 слов)
                    "description": str,  # Объяснение того, что означает этот вариант
                    "preview": str | None,  # Содержимое предпросмотра, отображаемое при фокусировке на варианте
                }
            ],
            "multiSelect": bool,  # Установите значение true, чтобы разрешить множественный выбор
        }
    ],
    "answers": dict[str, str] | None,
    # Ответы пользователя, заполненные системой разрешений. Ответы с множественным выбором
    # — это строка, объединенная запятыми из выбранных меток; список меток принимается
    # на входе и приводится к этой форме
    "annotations": dict[str, dict] | None,
    # Аннотации для каждого вопроса от пользователя, ключ — текст вопроса.
    # Каждое значение может содержать "preview" (содержимое предпросмотра выбранного варианта)
    # и "notes" (свободные заметки по выбору)
    "metadata": dict | None,  # Метаданные аналитики, такие как {"source": "remember"}; не отображается пользователю
}
```

**Вывод:**

```python theme={null}
{
    "questions": [  # Вопросы, которые были заданы
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str, "preview": str | None}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Сопоставляет текст вопроса со строкой ответа
    # Ответы с множественным выбором разделены запятыми
    "response": str | None,
    # Свободный ответ, введенный вместо ответа на вопросы; когда установлено,
    # Claude получает "The user responded: ..." вместо списка ответов
    "annotations": dict[str, dict] | None,  # Аннотации "preview" и "notes" для каждого вопроса из выборов пользователя
    "afkTimeoutMs": int | None,  # Установлено, когда диалог автоматически разрешился после этого количества миллисекунд неактивности пользователя; отсутствует, когда пользователь ответил
}
```

<h3 id="bash">
  Bash
</h3>

**Имя инструмента:** `Bash`

**Ввод:**

```python theme={null}
{
    "command": str,  # Команда для выполнения
    "timeout": int | None,  # Необязательный тайм-аут в миллисекундах (макс 600000; более высокие значения зажимаются до максимума)
    "description": str | None,  # Четкое, краткое описание (5-10 слов)
    "run_in_background": bool | None,  # Установите значение true для выполнения в фоне
}
```

**Вывод:**

```python theme={null}
{
    "stdout": str,  # Вывод команды; stdout и stderr поступают объединенными в этот один чередующийся поток
    "stderr": str,  # Уведомления, которые добавляет сам инструмент, не stderr команды
    "interrupted": bool,  # Была ли команда прервана
    "isImage": bool | None,  # Содержит ли stdout данные изображения
    "backgroundTaskId": str | None,  # ID фоновой задачи, если команда выполняется в фоне
}
```

<h3 id="monitor">
  Monitor
</h3>

**Имя инструмента:** `Monitor`

Запускает фоновый источник и доставляет каждое событие в Claude, чтобы он мог реагировать без опроса: `command` запускает скрипт и выдает одно событие на строку stdout, а `ws` открывает WebSocket и выдает одно событие на текстовый фрейм. Укажите ровно один из `command` или `ws`.

Когда Monitor запускает команду, он следует тем же правилам разрешений, что и Bash; наблюдение за WebSocket запрашивает одобрение отдельно. Источник `ws` требует Claude Code v2.1.195 или позже. См. [Справочник инструмента Monitor](/docs/ru/tools-reference#monitor-tool) для поведения и доступности поставщика.

**Ввод:**

```python theme={null}
{
    "command": str | None,  # Скрипт оболочки; каждая строка stdout является событием, выход завершает наблюдение
    "ws": dict | None,  # Источник WebSocket: {"url": str, "protocols": list[str] | None}; каждый текстовый фрейм является событием
    "description": str,  # Краткое описание, показываемое в уведомлениях
    "timeout_ms": int | None,  # Завершить после этого срока (по умолчанию 300000, макс 3600000; эффективный срок составляет максимум 1800000)
}
```

**Вывод:**

```python theme={null}
{
    "taskId": str,  # ID фоновой задачи мониторинга
    "timeoutMs": int,  # Эффективный срок тайм-аута в миллисекундах
    "persistent": bool | None,  # False: каждое наблюдение имеет срок
}
```

<h3 id="edit">
  Edit
</h3>

**Имя инструмента:** `Edit`

**Ввод:**

```python theme={null}
{
    "file_path": str,  # Абсолютный путь к файлу для изменения
    "old_string": str,  # Текст для замены
    "new_string": str,  # Текст для замены на
    "replace_all": bool | None,  # Заменить все вхождения (по умолчанию False)
}
```

**Вывод:**

```python theme={null}
{
    "message": str,  # Сообщение подтверждения
    "replacements": int,  # Количество выполненных замен
    "file_path": str,  # Путь файла, который был отредактирован
}
```

<h3 id="read">
  Read
</h3>

**Имя инструмента:** `Read`

**Ввод:**

```python theme={null}
{
    "file_path": str,  # Абсолютный путь к файлу для чтения
    "offset": int | None,  # Номер строки для начала чтения
    "limit": int | None,  # Количество строк для чтения
}
```

**Вывод (текстовые файлы):**

```python theme={null}
{
    "content": str,  # Содержимое файла с номерами строк
    "total_lines": int,  # Общее количество строк в файле
    "lines_returned": int,  # Фактически возвращенные строки
}
```

**Вывод (изображения):**

```python theme={null}
{
    "image": str,  # Данные изображения в кодировке Base64
    "mime_type": str,  # MIME-тип изображения
    "file_size": int,  # Размер файла в байтах
}
```

<h3 id="write">
  Write
</h3>

**Имя инструмента:** `Write`

**Ввод:**

```python theme={null}
{
    "file_path": str,  # Абсолютный путь к файлу для записи
    "content": str,  # Содержимое для записи в файл
}
```

**Вывод:**

```python theme={null}
{
    "message": str,  # Сообщение об успехе
    "bytes_written": int,  # Количество записанных байтов
    "file_path": str,  # Путь файла, который был записан
}
```

<h3 id="glob">
  Glob
</h3>

**Имя инструмента:** `Glob`

**Ввод:**

```python theme={null}
{
    "pattern": str,  # Шаблон glob для сопоставления файлов
    "path": str | None,  # Каталог для поиска (по умолчанию текущий каталог)
}
```

**Вывод:**

```python theme={null}
{
    "matches": list[str],  # Массив совпадающих путей файлов
    "count": int,  # Количество найденных совпадений
    "search_path": str,  # Использованный каталог поиска
}
```

<h3 id="grep">
  Grep
</h3>

**Имя инструмента:** `Grep`

**Ввод:**

```python theme={null}
{
    "pattern": str,  # Шаблон регулярного выражения
    "path": str | None,  # Файл или каталог для поиска
    "glob": str | None,  # Шаблон glob для фильтрации файлов
    "type": str | None,  # Тип файла для поиска
    "output_mode": str | None,  # "content", "files_with_matches" или "count"
    "-i": bool | None,  # Поиск без учета регистра
    "-n": bool | None,  # Показать номера строк
    "-B": int | None,  # Строки для отображения перед каждым совпадением
    "-A": int | None,  # Строки для отображения после каждого совпадения
    "-C": int | None,  # Строки для отображения до и после
    "head_limit": int | None,  # Ограничить вывод первыми N строками/записями
    "multiline": bool | None,  # Включить многострочный режим
}
```

**Вывод (режим content):**

```python theme={null}
{
    "matches": [
        {
            "file": str,
            "line_number": int | None,
            "line": str,
            "before_context": list[str] | None,
            "after_context": list[str] | None,
        }
    ],
    "total_matches": int,
}
```

**Вывод (режим files\_with\_matches):**

```python theme={null}
{
    "files": list[str],  # Файлы, содержащие совпадения
    "count": int,  # Количество файлов с совпадениями
}
```

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Имя инструмента:** `NotebookEdit`

**Ввод:**

```python theme={null}
{
    "notebook_path": str,  # Абсолютный путь к записной книжке Jupyter
    "cell_id": str | None,  # ID ячейки для редактирования
    "new_source": str,  # Новый исходный код для ячейки
    "cell_type": "code" | "markdown" | None,  # Тип ячейки
    "edit_mode": "replace" | "insert" | "delete" | None,  # Тип операции редактирования
}
```

**Вывод:**

```python theme={null}
{
    "message": str,  # Сообщение об успехе
    "edit_type": "replaced" | "inserted" | "deleted",  # Тип выполненного редактирования
    "cell_id": str | None,  # ID ячейки, которая была затронута
    "total_cells": int,  # Общее количество ячеек в записной книжке после редактирования
}
```

<h3 id="webfetch">
  WebFetch
</h3>

**Имя инструмента:** `WebFetch`

**Ввод:**

```python theme={null}
{
    "url": str,  # URL для получения содержимого
    "prompt": str,  # Подсказка для запуска на полученном содержимом
}
```

**Вывод:**

```python theme={null}
{
    "bytes": int,  # Размер полученного содержимого в байтах
    "code": int,  # Код ответа HTTP
    "codeText": str,  # Текст кода ответа HTTP
    "result": str,  # Обработанный результат от применения подсказки к содержимому
    "durationMs": int,  # Время получения и обработки содержимого в миллисекундах
    "url": str,  # URL, который был получен
}
```

<h3 id="websearch">
  WebSearch
</h3>

**Имя инструмента:** `WebSearch`

**Ввод:**

```python theme={null}
{
    "query": str,  # Поисковый запрос для использования
    "allowed_domains": list[str] | None,  # Включать результаты только с этих доменов
    "blocked_domains": list[str] | None,  # Никогда не включать результаты с этих доменов
}
```

**Вывод:**

```python theme={null}
{
    "query": str,  # Поисковый запрос
    "results": list[str | {"tool_use_id": str, "content": list[{"title": str, "url": str}]}],
    "durationSeconds": float,  # Длительность поиска в секундах
}
```

<h3 id="todowrite">
  TodoWrite
</h3>

**Имя инструмента:** `TodoWrite`

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

**Ввод:**

```python theme={null}
{
    "todos": [
        {
            "content": str,  # Описание задачи
            "status": "pending" | "in_progress" | "completed",  # Статус задачи
            "activeForm": str,  # Активная форма описания
        }
    ]
}
```

**Вывод:**

```python theme={null}
{
    "message": str,  # Сообщение об успехе
    "stats": {"total": int, "pending": int, "in_progress": int, "completed": int},
}
```

<h3 id="taskcreate">
  TaskCreate
</h3>

**Имя инструмента:** `TaskCreate`

**Ввод:**

```python theme={null}
{
    "subject": str,  # Краткое название задачи
    "description": str,  # Подробное описание задачи
    "activeForm": str | None,  # Метка в настоящем времени, показываемая во время выполнения
    "metadata": dict | None,  # Произвольные метаданные вызывающей стороны
}
```

**Вывод:**

```python theme={null}
{
    "task": {"id": str, "subject": str},  # Созданная задача с назначенным ID
}
```

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Имя инструмента:** `TaskUpdate`

**Ввод:**

```python theme={null}
{
    "taskId": str,  # ID задачи для обновления
    "status": Literal["pending", "in_progress", "completed", "deleted"] | None,
    "subject": str | None,
    "description": str | None,
    "activeForm": str | None,
    "addBlocks": list[str] | None,  # ID задач, которые эта задача теперь блокирует
    "addBlockedBy": list[str] | None,  # ID задач, которые теперь блокируют эту задачу
    "owner": str | None,
    "metadata": dict | None,
}
```

**Вывод:**

```python theme={null}
{
    "success": bool,
    "taskId": str,
    "updatedFields": list[str],  # Названия полей, которые изменились
    "error": str | None,
    "statusChange": {"from": str, "to": str} | None,
}
```

<h3 id="taskget">
  TaskGet
</h3>

**Имя инструмента:** `TaskGet`

**Ввод:**

```python theme={null}
{
    "taskId": str,  # ID задачи для чтения
}
```

**Вывод:**

```python theme={null}
{
    "task": {
        "id": str,
        "subject": str,
        "description": str,
        "status": Literal["pending", "in_progress", "completed"],
        "blocks": list[str],
        "blockedBy": list[str],
    } | None,  # None, когда ID не найден
}
```

<h3 id="tasklist">
  TaskList
</h3>

**Имя инструмента:** `TaskList`

**Ввод:**

```python theme={null}
{}
```

**Вывод:**

```python theme={null}
{
    "tasks": [
        {
            "id": str,
            "subject": str,
            "status": Literal["pending", "in_progress", "completed"],
            "owner": str | None,
            "blockedBy": list[str],
        }
    ],
}
```

<h3 id="taskoutput">
  TaskOutput
</h3>

Удалено в Claude Code v2.1.277. Ранее получало вывод из запущенной или завершенной фоновой задачи, с `BashOutput`, принимаемым как псевдоним; Claude читает выходной файл фоновой задачи с помощью `Read` вместо этого.

Запись `disallowed_tools` или правило отказа, которое по-прежнему называет любое имя, игнорируется без предупреждения.

<h3 id="taskstop">
  TaskStop
</h3>

**Имя инструмента:** `TaskStop`. Предыдущие имена `KillShell` и `KillBash` по-прежнему принимаются как псевдонимы.

**Ввод:**

```python theme={null}
{
    "task_id": str | None,  # ID фоновой задачи для остановки
    "shell_id": str | None,  # Устарело: используйте task_id вместо этого
}
```

**Вывод:**

```python theme={null}
{
    "message": str,  # Сообщение о статусе операции
    "task_id": str,  # ID остановленной задачи
    "task_type": str,  # Тип остановленной задачи
    "command": str | None,  # Команда или описание остановленной задачи
}
```

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Имя инструмента:** `ExitPlanMode`

**Ввод:**

```python theme={null}
{
    "plan": str  # План для запуска пользователем на одобрение
}
```

**Вывод:**

```python theme={null}
{
    "message": str,  # Сообщение подтверждения
    "approved": bool | None,  # Одобрил ли пользователь план
}
```

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Имя инструмента:** `ListMcpResourcesTool`

**Ввод:**

```python theme={null}
{
    "server": str | None  # Необязательное имя сервера для фильтрации ресурсов
}
```

**Вывод:**

```python theme={null}
{
    "resources": [
        {
            "uri": str,
            "name": str,
            "description": str | None,
            "mimeType": str | None,
            "server": str,
        }
    ],
    "total": int,
}
```

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Имя инструмента:** `ReadMcpResourceTool`

**Ввод:**

```python theme={null}
{
    "server": str,  # Имя сервера MCP
    "uri": str,  # URI ресурса для чтения
}
```

**Вывод:**

```python theme={null}
{
    "contents": [
        {"uri": str, "mimeType": str | None, "text": str | None, "blob": str | None}
    ],
    "server": str,
}
```

<h2 id="build-a-continuous-conversation-interface">
  Построение интерфейса непрерывного разговора
</h2>

Следующий пример сохраняет один `ClaudeSDKClient` подключённым на протяжении нескольких ходов, поэтому Claude помнит предыдущие сообщения. Введите `new` для отключения и повторного подключения для новой сессии или `exit` для завершения разговора.

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    TextBlock,
)
import asyncio


class ConversationSession:
    """Maintains a single conversation session with Claude."""

    def __init__(self, options: ClaudeAgentOptions | None = None):
        self.client = ClaudeSDKClient(options)
        self.turn_count = 0

    async def start(self):
        await self.client.connect()
        print("Starting conversation session. Claude will remember context.")
        print(
            "Commands: 'exit' to quit, 'interrupt' to stop current task, 'new' for new session"
        )

        while True:
            user_input = input(f"\n[Turn {self.turn_count + 1}] You: ")

            if user_input.lower() == "exit":
                break
            elif user_input.lower() == "interrupt":
                await self.client.interrupt()
                print("Task interrupted!")
                continue
            elif user_input.lower() == "new":
                # Disconnect and reconnect for a fresh session
                await self.client.disconnect()
                await self.client.connect()
                self.turn_count = 0
                print("Started new conversation session (previous context cleared)")
                continue

            # Send message - the session retains all previous messages
            await self.client.query(user_input)
            self.turn_count += 1

            # Process response
            print(f"[Turn {self.turn_count}] Claude: ", end="")
            async for message in self.client.receive_response():
                if isinstance(message, AssistantMessage):
                    for block in message.content:
                        if isinstance(block, TextBlock):
                            print(block.text, end="")
            print()  # New line after response

        await self.client.disconnect()
        print(f"Conversation ended after {self.turn_count} turns.")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"], permission_mode="acceptEdits"
    )
    session = ConversationSession(options)
    await session.start()


# Example conversation:
# Turn 1 - You: "Create a file called hello.py"
# Turn 1 - Claude: "I'll create a hello.py file for you..."
# Turn 2 - You: "What's in that file?"
# Turn 2 - Claude: "The hello.py file I just created contains..." (remembers!)
# Turn 3 - You: "Add a main function to it"
# Turn 3 - Claude: "I'll add a main function to hello.py..." (knows which file!)

asyncio.run(main())
```

<h2 id="error-handling">
  Обработка ошибок
</h2>

Следующий пример оборачивает вызов `query()` в обработчики для четырех из [типов ошибок](#error-types), которые вызывает SDK.

Этот пример перехватывает [`ResultError`](#resulterror), что требует Python Agent SDK версии 0.2.140 или позже.

```python theme={null}
import asyncio

from claude_agent_sdk import (
    query,
    CLINotFoundError,
    ProcessError,
    ResultError,
    CLIJSONDecodeError,
)


async def main():
    try:
        async for message in query(prompt="Hello"):
            print(message)
    except CLINotFoundError:
        print(
            "Claude Code CLI not found. Try reinstalling: pip install --force-reinstall claude-agent-sdk"
        )
    # Catch ResultError before ProcessError, which it subclasses. Its message
    # carries the error text. A failed final request, such as an API error,
    # arrives with subtype "success", so branch on terminal_reason first.
    except ResultError as e:
        if e.terminal_reason == "api_error":
            print(f"API request failed: {e}")
        else:
            print(f"Query ended with an error result ({e.terminal_reason or e.subtype}): {e}")
    except ProcessError as e:
        print(f"Process failed with exit code: {e.exit_code}")
    except CLIJSONDecodeError as e:
        print(f"Failed to parse response: {e}")


asyncio.run(main())
```

<h2 id="sandbox-configuration">
  Конфигурация Sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Конфигурация для поведения sandbox. Используйте это для включения sandboxing команд и программной настройки ограничений сети.

```python theme={null}
class SandboxSettings(TypedDict, total=False):
    enabled: bool
    autoAllowBashIfSandboxed: bool
    excludedCommands: list[str]
    allowUnsandboxedCommands: bool
    network: SandboxNetworkConfig
    ignoreViolations: SandboxIgnoreViolations
    enableWeakerNestedSandbox: bool
```

| Property                    | Type                                                  | Default | Description                                                                                                                                                                                                                                             |
| :-------------------------- | :---------------------------------------------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `enabled`                   | `bool`                                                | `False` | Включить режим sandbox для выполнения команд                                                                                                                                                                                                            |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`  | Автоматически одобрять bash команды, когда sandbox включен                                                                                                                                                                                              |
| `excludedCommands`          | `list[str]`                                           | `[]`    | Команды, которые обходят ограничения sandbox, такие как `["docker *"]`. Они выполняются без sandbox автоматически без участия модели; [`sandbox.excludedCommands`](/docs/ru/settings-reference#sandbox-excludedcommands) описывает, когда применяется запись |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`  | Разрешить модели запрашивать выполнение команд вне sandbox. Когда `True`, модель может установить `dangerouslyDisableSandbox` в входных данных инструмента, что переходит к [системе разрешений](#permissions-fallback-for-unsandboxed-commands)        |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`  | Конфигурация sandbox для сети                                                                                                                                                                                                                           |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`  | Настройка того, какие нарушения sandbox игнорировать                                                                                                                                                                                                    |
| `enableWeakerNestedSandbox` | `bool`                                                | `False` | Включить более слабый вложенный sandbox для совместимости                                                                                                                                                                                               |

<Note>
  Sandbox зависит от поддержки платформы и, на Linux, инструментов, таких как `bubblewrap` и `socat`. По умолчанию, когда `enabled` имеет значение `True`, но sandbox не может запуститься, команды выполняются без sandbox с предупреждением на stderr. Это поведение отличается от TypeScript SDK, где `failIfUnavailable` по умолчанию имеет значение `true`.

  Установите `"failIfUnavailable": True` в параметрах sandbox, чтобы остановиться вместо этого. Ключ еще не объявлен на `SandboxSettings`, но SDK передает его в Claude Code, который его соблюдает. `query()` затем сообщает `ResultMessage` с `subtype="error_during_execution"` и причину в `errors`. Поскольку это одноразовый вызов `query()`, SDK вызывает исключение после выдачи этого результата ошибки, поэтому оберните цикл в блок try, чтобы продолжить после него. См. [Handle the result](/docs/ru/agent-sdk/agent-loop#handle-the-result) для контракта ошибки.
</Note>

<h4 id="example-usage">
  Пример использования
</h4>

```python theme={null}
import asyncio

from claude_agent_sdk import query, ClaudeAgentOptions

sandbox_settings = {
    "enabled": True,
    "autoAllowBashIfSandboxed": True,
    "failIfUnavailable": True,
    "network": {"allowLocalBinding": True},
}


async def main():
    try:
        async for message in query(
            prompt="Build and test my project",
            options=ClaudeAgentOptions(sandbox=sandbox_settings),
        ):
            print(message)
    except Exception as error:
        # A single-shot query() raises after yielding an error result,
        # such as when failIfUnavailable is set and the sandbox can't start.
        print(f"Session ended with an error: {error}")


asyncio.run(main())
```

<Warning>
  **Безопасность Unix socket**: Опция `allowUnixSockets` может предоставить доступ к системным сервисам, которые выходят за пределы sandbox. Например, разрешение `/var/run/docker.sock` фактически предоставляет полный доступ к хост-системе через Docker API, обходя изоляцию sandbox. Разрешайте только Unix sockets, которые строго необходимы, и поймите последствия безопасности каждого.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Конфигурация для сети в режиме sandbox. Эти параметры применяются к sandboxed Bash командам, когда `enabled` имеет значение `True` в родительском [`SandboxSettings`](#sandboxsettings). Они не ограничивают инструмент WebFetch, который использует [правила разрешений](/docs/ru/permissions#webfetch) вместо этого.

```python theme={null}
class SandboxNetworkConfig(TypedDict, total=False):
    allowedDomains: list[str]
    deniedDomains: list[str]
    allowManagedDomainsOnly: bool
    allowUnixSockets: list[str]
    allowAllUnixSockets: bool
    allowLocalBinding: bool
    allowMachLookup: list[str]
    httpProxyPort: int
    socksProxyPort: int
```

| Property                  | Type        | Default | Description                                                                                                                                                                                                                                    |
| :------------------------ | :---------- | :------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `list[str]` | `[]`    | Имена доменов, к которым могут получить доступ процессы в sandbox                                                                                                                                                                              |
| `deniedDomains`           | `list[str]` | `[]`    | Имена доменов, к которым процессы в sandbox не могут получить доступ. Имеет приоритет над `allowedDomains`                                                                                                                                     |
| `allowManagedDomainsOnly` | `bool`      | `False` | Только управляемые параметры: когда установлено в управляемых параметрах, игнорировать `allowedDomains` и `WebFetch(domain:...)` правила разрешений из источников неуправляемых параметров. Не имеет эффекта при установке через параметры SDK |
| `allowUnixSockets`        | `list[str]` | `[]`    | Пути Unix socket, к которым могут получить доступ процессы (например, Docker socket)                                                                                                                                                           |
| `allowAllUnixSockets`     | `bool`      | `False` | Разрешить доступ ко всем Unix sockets                                                                                                                                                                                                          |
| `allowLocalBinding`       | `bool`      | `False` | Разрешить процессам привязываться к локальным портам (например, для dev серверов)                                                                                                                                                              |
| `allowMachLookup`         | `list[str]` | `[]`    | Только macOS: имена сервисов XPC/Mach для разрешения. Поддерживает завершающий подстановочный знак                                                                                                                                             |
| `httpProxyPort`           | `int`       | `None`  | Порт HTTP прокси для сетевых запросов                                                                                                                                                                                                          |
| `socksProxyPort`          | `int`       | `None`  | Порт SOCKS прокси для сетевых запросов                                                                                                                                                                                                         |

<Note>
  Встроенный прокси sandbox применяет список разрешений сети на основе запрашиваемого имени хоста и не завершает и не проверяет трафик TLS, поэтому методы, такие как [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting), потенциально могут его обойти. См. [Sandboxing security limitations](/docs/ru/sandboxing#security-limitations) для деталей и [Secure deployment](/docs/ru/agent-sdk/secure-deployment#traffic-forwarding) для настройки прокси, завершающего TLS.
</Note>

<h3 id="sandboxignoreviolations">
  `SandboxIgnoreViolations`
</h3>

Конфигурация для игнорирования определенных нарушений sandbox.

```python theme={null}
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| Property  | Type        | Default | Description                                      |
| :-------- | :---------- | :------ | :----------------------------------------------- |
| `file`    | `list[str]` | `[]`    | Шаблоны путей файлов для игнорирования нарушений |
| `network` | `list[str]` | `[]`    | Шаблоны сети для игнорирования нарушений         |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback системы разрешений для команд без Sandbox
</h3>

Когда `allowUnsandboxedCommands` включен, модель может запросить выполнение команд вне sandbox, установив `dangerouslyDisableSandbox: True` во входных данных инструмента. Эти запросы переходят к существующей системе разрешений, что означает, что ваш обработчик `can_use_tool` будет вызван, позволяя вам реализовать пользовательскую логику авторизации.

Ваши записи `excludedCommands` вместо этого выводят команду из sandbox без участия модели; [`sandbox.excludedCommands`](/docs/ru/settings-reference#sandbox-excludedcommands) описывает, когда применяется запись.

Следующий пример регистрирует каждый запрос без sandbox и отклоняет его, если только ваша собственная логика авторизации не разрешит это:

```python theme={null}
import asyncio
from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    HookMatcher,
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


def is_command_authorized(command: str | None) -> bool:
    # Replace with your own authorization logic
    return False



async def can_use_tool(
    tool: str, input: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    # Check if the model is requesting to bypass the sandbox
    if tool == "Bash" and input.get("dangerouslyDisableSandbox"):
        # The model is requesting to run this command outside the sandbox
        print(f"Unsandboxed command requested: {input.get('command')}")

        if is_command_authorized(input.get("command")):
            return PermissionResultAllow()
        return PermissionResultDeny(
            message="Command not authorized for unsandboxed execution"
        )
    return PermissionResultAllow()


# Required: dummy hook keeps the stream open for can_use_tool
async def dummy_hook(input_data, tool_use_id, context):
    return {"continue_": True}


async def prompt_stream():
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Deploy my application"},
    }


async def main():
    async for message in query(
        prompt=prompt_stream(),
        options=ClaudeAgentOptions(
            sandbox={
                "enabled": True,
                "allowUnsandboxedCommands": True,  # Model can request unsandboxed execution
            },
            permission_mode="default",
            can_use_tool=can_use_tool,
            hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
        ),
    ):
        print(message)


asyncio.run(main())
```

<Warning>
  Команды, выполняемые с `dangerouslyDisableSandbox: True`, имеют полный доступ к системе. Убедитесь, что ваш обработчик `can_use_tool` тщательно проверяет эти запросы.

  Если `permission_mode` установлен на `bypassPermissions` и `allow_unsandboxed_commands` включен, модель может автономно выполнять команды вне sandbox без запросов одобрения, кроме [действий, которые ни один режим не одобряет автоматически](/docs/ru/permission-modes#actions-no-mode-auto-approves). Эта комбинация фактически позволяет модели молча выходить из изоляции sandbox.
</Warning>

<h2 id="see-also">
  См. также
</h2>

* [SDK overview](/docs/ru/agent-sdk/overview) - Общие концепции SDK
* [TypeScript SDK reference](/docs/ru/agent-sdk/typescript) - Документация TypeScript SDK
* [Custom tools](/docs/ru/agent-sdk/custom-tools) - Определение встроенных инструментов MCP для вызова Claude
* [CLI reference](/docs/ru/cli-reference) - Интерфейс командной строки
* [Common workflows](/docs/ru/common-workflows) - Пошаговые руководства
