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

Для uv, Windows PowerShell и настройки API ключа см. [Начало работы в обзоре Agent SDK](/docs/ru/agent-sdk/overview#get-started).

<h2 id="choosing-between-query-and-claudesdkclient">
  Выбор между `query()` и `ClaudeSDKClient`
</h2>

Python SDK предоставляет два способа взаимодействия с Claude Code:

<h3 id="quick-comparison">
  Быстрое сравнение
</h3>

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

<h3 id="when-to-use-query-one-off-tasks">
  Когда использовать `query()` (одноразовые задачи)
</h3>

**Лучше всего для:**

* Одноразовых вопросов, когда вам не нужна история разговора
* Независимых задач, которые не требуют контекста из предыдущих обменов
* Простых скриптов автоматизации
* Когда вы хотите начать с чистого листа каждый раз

<h3 id="when-to-use-claudesdkclient-continuous-conversation">
  Когда использовать `ClaudeSDKClient` (непрерывный разговор)
</h3>

**Лучше всего для:**

* **Продолжения разговоров** - Когда вам нужно, чтобы Claude помнил контекст
* **Уточняющих вопросов** - Построение на основе предыдущих ответов
* **Интерактивных приложений** - Интерфейсы чата, REPL
* **Логики, управляемой ответом** - Когда следующее действие зависит от ответа Claude
* **Управления сеансом** - Явное управление жизненным циклом разговора

<h2 id="functions">
  Функции
</h2>

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
        cwd="/home/user/project",
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

| Параметр       | Тип                                             | Описание                                                                        |
| :------------- | :---------------------------------------------- | :------------------------------------------------------------------------------ |
| `name`         | `str`                                           | Уникальный идентификатор инструмента                                            |
| `description`  | `str`                                           | Понятное описание того, что делает инструмент                                   |
| `input_schema` | `type \| dict[str, Any]`                        | Схема, определяющая входные параметры инструмента (см. ниже)                    |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Дополнительные аннотации MCP tool, предоставляющие подсказки поведения клиентам |

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

Переэкспортировано из `mcp.types` (также доступно как `from claude_agent_sdk import ToolAnnotations`). Все поля являются дополнительными подсказками; клиенты не должны полагаться на них для решений безопасности.

| Поле              | Тип            | По умолчанию | Описание                                                                                                                                                     |
| :---------------- | :------------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `str \| None`  | `None`       | Понятное название инструмента                                                                                                                                |
| `readOnlyHint`    | `bool \| None` | `False`      | Если `True`, инструмент не изменяет свою среду                                                                                                               |
| `destructiveHint` | `bool \| None` | `True`       | Если `True`, инструмент может выполнять деструктивные обновления (имеет смысл только когда `readOnlyHint` равен `False`)                                     |
| `idempotentHint`  | `bool \| None` | `False`      | Если `True`, повторные вызовы с одинаковыми аргументами не имеют дополнительного эффекта (имеет смысл только когда `readOnlyHint` равен `False`)             |
| `openWorldHint`   | `bool \| None` | `True`       | Если `True`, инструмент взаимодействует с внешними сущностями (например, веб-поиск). Если `False`, область инструмента закрыта (например, инструмент памяти) |

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
from claude_agent_sdk import tool, create_sdk_mcp_server


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

| Свойство             | Тип                            | Описание                                   |
| :------------------- | :----------------------------- | :----------------------------------------- |
| `type`               | `Literal["user", "assistant"]` | Роль сообщения                             |
| `uuid`               | `str`                          | Уникальный идентификатор сообщения         |
| `session_id`         | `str`                          | Идентификатор сеанса                       |
| `message`            | `Any`                          | Необработанное содержимое сообщения        |
| `parent_tool_use_id` | `None`                         | Зарезервировано для будущего использования |

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

# Tag a session
tag_session("550e8400-e29b-41d4-a716-446655440000", "needs-review")

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

**Поддерживает сеанс разговора через несколько обменов.** Это эквивалент Python того, как функция `query()` TypeScript SDK работает внутри - она создает объект клиента, который может продолжать разговоры.

<h4 id="key-features">
  Ключевые особенности
</h4>

* **Непрерывность сеанса**: Поддерживает контекст разговора через несколько вызовов `query()`
* **Один разговор**: Сеанс сохраняет предыдущие сообщения
* **Поддержка прерываний**: Может остановить выполнение в середине задачи
* **Явный жизненный цикл**: Вы контролируете, когда сеанс начинается и заканчивается
* **Поток, управляемый ответом**: Может реагировать на ответы и отправлять дополнения
* **Пользовательские инструменты и hooks**: Поддерживает пользовательские инструменты (созданные с помощью декоратора `@tool`) и hooks

```python theme={null}
class ClaudeSDKClient:
    def __init__(self, options: ClaudeAgentOptions | None = None, transport: Transport | None = None)
    async def connect(self, prompt: str | AsyncIterable[dict] | None = None) -> None
    async def query(self, prompt: str | AsyncIterable[dict], session_id: str = "default") -> None
    async def receive_messages(self) -> AsyncIterator[Message]
    async def receive_response(self) -> AsyncIterator[Message]
    async def interrupt(self) -> None
    async def set_permission_mode(self, mode: str) -> None
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
| `set_model(model)`                        | Измените модель для текущего сеанса. Передайте `None` для сброса на значение по умолчанию                                                                                      |
| `rewind_files(user_message_id)`           | Восстановите файлы в их состояние в указанном пользовательском сообщении. Требует `enable_file_checkpointing=True`. См. [File checkpointing](/docs/ru/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Получите статус всех настроенных MCP servers. Возвращает [`McpStatusResponse`](#mcpstatusresponse)                                                                             |
| `reconnect_mcp_server(server_name)`       | Повторите попытку подключения к MCP server, который не удался или был отключен                                                                                                 |
| `toggle_mcp_server(server_name, enabled)` | Включите или отключите MCP server в середине сеанса. Отключение удаляет его инструменты                                                                                        |
| `stop_task(task_id)`                      | Остановите выполняющуюся фоновую задачу. [`TaskNotificationMessage`](#tasknotificationmessage) со статусом `"stopped"` следует в потоке сообщений                              |
| `get_server_info()`                       | Получите информацию о сервере, включая ID сеанса и возможности                                                                                                                 |
| `disconnect()`                            | Отключитесь от Claude                                                                                                                                                          |

<h4 id="context-manager-support">
  Поддержка менеджера контекста
</h4>

Клиент можно использовать как асинхронный менеджер контекста для автоматического управления соединением:

```python theme={null}
async with ClaudeSDKClient() as client:
    await client.query("Hello Claude")
    async for message in client.receive_response():
        print(message)
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
                print(f"Interrupted task finished with subtype={message.subtype!r}")
                # subtype is "error_during_execution" for interrupted tasks

        # Send a new command
        await client.query("Just say hello instead")

        # Now receive the new response
        async for message in client.receive_response():
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(f"New result: {message.result}")


asyncio.run(interruptible_task())
```

<Note>
  **Поведение буфера после прерывания:** `interrupt()` отправляет сигнал остановки, но не очищает буфер сообщений. Сообщения, уже созданные прерванной задачей, включая ее `ResultMessage` (с `subtype="error_during_execution"`), остаются в потоке. Вы должны слить их с помощью `receive_response()` перед чтением ответа на новый запрос. Если вы отправите новый запрос сразу после `interrupt()` и вызовете `receive_response()` только один раз, вы получите сообщения прерванной задачи, а не ответ на новый запрос.
</Note>

<h4 id="example-advanced-permission-control">
  Пример - Расширенное управление разрешениями
</h4>

```python theme={null}
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

| Свойство       | Тип                                        | Описание                                                                                                         |
| :------------- | :----------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                      | Уникальный идентификатор инструмента                                                                             |
| `description`  | `str`                                      | Понятное описание                                                                                                |
| `input_schema` | `type[T] \| dict[str, Any]`                | Схема для валидации ввода                                                                                        |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]` | Асинхронная функция, которая обрабатывает выполнение инструмента                                                 |
| `annotations`  | `ToolAnnotations \| None`                  | Дополнительные аннотации MCP tool (например, `readOnlyHint`, `destructiveHint`, `openWorldHint`). Из `mcp.types` |

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
    system_prompt: str | SystemPromptPreset | SystemPromptFile | None = None
    mcp_servers: dict[str, McpServerConfig] | str | Path = field(default_factory=dict)
    strict_mcp_config: bool = False
    permission_mode: PermissionMode | None = None
    continue_conversation: bool = False
    resume: str | None = None
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
    fork_session: bool = False
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
```

| Свойство                      | Тип                                                                                   | По умолчанию                       | Описание                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :---------------------------- | :------------------------------------------------------------------------------------ | :--------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                             | Конфигурация инструментов. Используйте `{"type": "preset", "preset": "claude_code"}` для инструментов Claude Code по умолчанию                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                               | Инструменты для автоматического одобрения без запроса. Это не ограничивает Claude только этими инструментами; неуказанные инструменты переходят к `permission_mode` и `can_use_tool`. Используйте `disallowed_tools` для блокировки инструментов. См. [Permissions](/docs/ru/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                   |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptFile \| None`                               | `None`                             | Конфигурация системной подсказки. Передайте строку для пользовательской подсказки, `{"type": "preset", "preset": "claude_code"}` для системной подсказки Claude Code с дополнительным `"append"`, или `{"type": "file", "path": "..."}` для загрузки большой подсказки с диска. См. [`SystemPromptPreset`](#systempromptpreset) и [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                                                                                             |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                               | Конфигурации MCP server или путь к файлу конфигурации                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `strict_mcp_config`           | `bool`                                                                                | `False`                            | Когда `True`, используйте только servers, переданные в `mcp_servers`, и игнорируйте проект `.mcp.json`, параметры пользователя, MCP servers, предоставленные plugins, и [claude.ai connectors](/docs/ru/mcp#use-mcp-servers-from-claude-ai). Соответствует флагу CLI `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                                 |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                             | Режим разрешений для использования инструментов                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `continue_conversation`       | `bool`                                                                                | `False`                            | Продолжить самый последний разговор                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `resume`                      | `str \| None`                                                                         | `None`                             | ID сеанса для возобновления                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `max_turns`                   | `int \| None`                                                                         | `None`                             | Максимальное количество агентских ходов (раунды использования инструментов)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                             | Остановите запрос, когда оценка стоимости на стороне клиента достигнет этого значения USD. Сравнивается с той же оценкой, что и `total_cost_usd`; см. [Track cost and usage](/docs/ru/agent-sdk/cost-tracking) для предостережений точности                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                               | Инструменты для отклонения. Простое имя, такое как `"Bash"`, удаляет инструмент из контекста Claude. Правило с областью действия, такое как `"Bash(rm *)"`, оставляет инструмент доступным и отклоняет совпадающие вызовы в каждом режиме разрешений, включая `bypassPermissions`. См. [Permissions](/docs/ru/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                  |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                            | Включить отслеживание изменений файлов для перемотки. См. [File checkpointing](/docs/ru/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `model`                       | `str \| None`                                                                         | `None`                             | Модель Claude для использования. См. [accepted values and provider-specific IDs](/docs/ru/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `fallback_model`              | `str \| None`                                                                         | `None`                             | Резервная модель для использования, если основная модель не работает                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                               | Функции бета-версии для включения. См. [`SdkBeta`](#sdkbeta) для доступных опций                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                             | Формат вывода для структурированных ответов (например, `{"type": "json_schema", "schema": {...}}`). См. [Structured outputs](/docs/ru/agent-sdk/structured-outputs) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                             | Имя MCP tool для подсказок разрешений                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                             | Текущий рабочий каталог                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                             | Пользовательский путь к исполняемому файлу Claude Code CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `settings`                    | `str \| None`                                                                         | `None`                             | Путь к файлу параметров                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                               | Дополнительные каталоги, к которым Claude может получить доступ                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `env`                         | `dict[str, str]`                                                                      | `{}`                               | Переменные окружения, объединенные поверх унаследованного окружения процесса. См. [Environment variables](/docs/ru/env-vars) для переменных, которые читает базовый CLI, и [Handle slow or stalled API responses](#handle-slow-or-stalled-api-responses) для переменных, связанных с тайм-аутом                                                                                                                                                                                                                                                                                                                                                                            |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                               | Дополнительные аргументы CLI для прямой передачи в CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                             | Максимальные байты при буферизации stdout CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                       | *Устарело* - Объект, подобный файлу, для вывода отладки. Вместо этого используйте обратный вызов `stderr`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                             | Функция обратного вызова для вывода stderr из CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                             | Функция обратного вызова разрешения инструмента, вызываемая только когда [permission flow](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) переходит к подсказке. Не вызывается для вызовов, автоматически одобренных `allowed_tools`, правилами разрешения или `permission_mode`. `AskUserQuestion`, connector tools [установленные вашей организацией на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) и MCP tools, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool), достигают его даже если вы их разрешили; в режиме `dontAsk` они вместо этого отклоняются. См. [`CanUseTool`](#canusetool) для деталей |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                             | Конфигурации hooks для перехвата событий                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `user`                        | `str \| None`                                                                         | `None`                             | Идентификатор пользователя                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `include_partial_messages`    | `bool`                                                                                | `False`                            | Включить события потоковой передачи частичных сообщений. Когда включено, выдаются сообщения [`StreamEvent`](#streamevent)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `include_hook_events`         | `bool`                                                                                | `False`                            | Включить события жизненного цикла hooks в поток сообщений как объекты `HookEventMessage`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `fork_session`                | `bool`                                                                                | `False`                            | При возобновлении с `resume` разветвитесь на новый ID сеанса вместо продолжения исходного сеанса                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                             | Программно определенные подагенты                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                               | Загрузите пользовательские plugins из локальных путей. См. [Plugins](/docs/ru/agent-sdk/plugins) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                             | Программно настройте поведение sandbox. См. [Sandbox settings](#sandboxsettings) для деталей                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (CLI defaults: all sources) | Контролируйте, какие параметры файловой системы загружать. Передайте `[]` для отключения пользовательских, проектных и локальных параметров. Управляемые параметры политики загружаются независимо; параметры, управляемые сервером, загружаются, когда сеанс аутентифицируется с учетными данными организации на [подходящей конфигурации](/docs/ru/server-managed-settings#platform-availability). См. [Use Claude Code features](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control) для входов, которые читаются независимо от этой опции, и как их отключить                                                                                     |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                             | Skills, доступные сеансу. Передайте `"all"` для включения каждого обнаруженного skill, или список имен skills. Когда установлено, SDK автоматически добавляет инструмент Skill в `allowed_tools`. Если вы также передаете `tools`, включите `"Skill"` в этот список. См. [Skills](/docs/ru/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                               |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                             | *Устарело* - Максимальные токены для блоков мышления. Вместо этого используйте `thinking`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                             | Управляет поведением расширенного мышления. Имеет приоритет над `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                             | Уровень усилий для глубины мышления. См. [adjust the effort level](/docs/ru/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `session_store`               | [`SessionStore`](/docs/ru/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                             | Зеркалируйте стенограммы сеансов во внешний бэкэнд, чтобы любой хост мог их возобновить. См. [Persist sessions to external storage](/docs/ru/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                        | Когда сбрасывать записи зеркальной стенограммы в `session_store`. `"batched"` сбрасывает один раз за ход или когда буфер заполняется; `"eager"` запускает фоновый сброс после каждого кадра. Игнорируется, когда `session_store` равен `None`                                                                                                                                                                                                                                                                                                                                                                                                                         |

<h4 id="handle-slow-or-stalled-api-responses">
  Обработка медленных или зависших ответов API
</h4>

Подпроцесс CLI читает несколько переменных окружения, которые управляют тайм-аутами API и обнаружением зависания. Передайте их через `ClaudeAgentOptions.env`:

```python theme={null}
options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS`: тайм-аут для каждого запроса на клиенте Anthropic в миллисекундах. По умолчанию `600000`. Применяется к основному циклу и всем подагентам.
* `CLAUDE_CODE_MAX_RETRIES`: максимальное количество повторных попыток API. По умолчанию `10`, ограничено `15`. Каждая повторная попытка получает свое собственное окно `API_TIMEOUT_MS`, поэтому наихудшее время стены примерно `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` плюс отступ. Для автоматических запусков, которым нужно ждать через более длительные сбои, установите `CLAUDE_CODE_RETRY_WATCHDOG=1`: он повторяет ошибки емкости бесконечно, и начиная с Claude Code v2.1.199 повышает значение по умолчанию для других переходных ошибок до `300` и удаляет ограничение на эту переменную.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: сторожевой таймер зависания для подагентов, запущенных с `run_in_background`. По умолчанию `600000`. Сбрасывается при каждом событии потока; при зависании он прерывает подагента, отмечает задачу как неудачную и выводит ошибку родителю с любым частичным результатом. Не применяется к синхронным подагентам.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` с `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: прерывает запрос, когда заголовки прибыли, но тело ответа перестает потоковать. Сторожевой таймер включен по умолчанию для всех поставщиков; установите `CLAUDE_ENABLE_STREAM_WATCHDOG=0` для отключения. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` по умолчанию `300000` и зажимается до этого минимума. Прерванный запрос проходит через обычный путь повторной попытки.

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
```

| Поле                       | Обязательно | Описание                                                                                                                                                                                                                                                                                                                                                        |
| :------------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | Да          | Должно быть `"preset"` для использования предустановленной системной подсказки                                                                                                                                                                                                                                                                                  |
| `preset`                   | Да          | Должно быть `"claude_code"` для использования системной подсказки Claude Code                                                                                                                                                                                                                                                                                   |
| `append`                   | Нет         | Дополнительные инструкции для добавления к предустановленной системной подсказке                                                                                                                                                                                                                                                                                |
| `exclude_dynamic_sections` | Нет         | Переместите контекст для каждого сеанса, такой как рабочий каталог, статус git и пути памяти, из системной подсказки в первое пользовательское сообщение. Улучшает повторное использование кэша подсказок между пользователями и машинами. См. [Modify system prompts](/docs/ru/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |

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

| Значение    | Описание                                                | Местоположение                |
| :---------- | :------------------------------------------------------ | :---------------------------- |
| `"user"`    | Глобальные параметры пользователя                       | `~/.claude/settings.json`     |
| `"project"` | Общие параметры проекта (контролируемые версией)        | `.claude/settings.json`       |
| `"local"`   | Локальные параметры проекта (не контролируемые версией) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Поведение по умолчанию
</h4>

Когда `setting_sources` опущено или `None`, `query()` загружает те же параметры файловой системы, что и Claude Code CLI: пользовательские, проектные и локальные. Управляемые параметры политики загружаются во всех случаях; параметры, управляемые сервером, загружаются, когда сеанс аутентифицируется с учетными данными организации на [подходящей конфигурации](/docs/ru/server-managed-settings#platform-availability). См. [What settingSources does not control](/docs/ru/agent-sdk/claude-code-features#what-settingsources-does-not-control) для входов, которые читаются независимо от этой опции, и как их отключить.

<h4 id="why-use-setting_sources">
  Почему использовать setting\_sources
</h4>

**Отключить параметры файловой системы:**

```python theme={null}
# Do not load user, project, or local settings from disk
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=[]
    ),
):
    print(message)
```

<Note>
  В Python SDK 0.1.59 и более ранних версиях пустой список рассматривался так же, как опущение опции, поэтому `setting_sources=[]` не отключал параметры файловой системы. Обновитесь до более новой версии, если вам нужно, чтобы пустой список вступил в силу. TypeScript SDK не затронут.
</Note>

**Загрузить все параметры файловой системы явно:**

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=["user", "project", "local"]
    ),
):
    print(message)
```

**Загрузить только определенные источники параметров:**

```python theme={null}
# Load only project settings, ignore user and local
async for message in query(
    prompt="Run CI checks",
    options=ClaudeAgentOptions(
        setting_sources=["project"]  # Only .claude/settings.json
    ),
):
    print(message)
```

**Тестирование и окружения CI:**

```python theme={null}
# Ensure consistent behavior in CI by excluding local settings
async for message in query(
    prompt="Run tests",
    options=ClaudeAgentOptions(
        setting_sources=["project"],  # Only team-shared settings
        permission_mode="bypassPermissions",
    ),
):
    print(message)
```

**Приложения только SDK:**

```python theme={null}
# Define everything programmatically.
# Pass [] to opt out of filesystem setting sources.
async for message in query(
    prompt="Review this PR",
    options=ClaudeAgentOptions(
        setting_sources=[],
        agents={...},
        mcp_servers={...},
        allowed_tools=["Read", "Grep", "Glob"],
    ),
):
    print(message)
```

**Загрузка инструкций проекта CLAUDE.md:**

```python theme={null}
# Load project settings to include CLAUDE.md files
async for message in query(
    prompt="Add a new feature following project conventions",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",  # Use Claude Code's system prompt
        },
        setting_sources=["project"],  # Loads CLAUDE.md from project
        allowed_tools=["Read", "Write", "Edit"],
    ),
):
    print(message)
```

<h4 id="settings-precedence">
  Приоритет параметров
</h4>

Когда загружаются несколько источников, параметры объединяются с этим приоритетом (от наивысшего к наименьшему):

1. Локальные параметры (`.claude/settings.local.json`)
2. Параметры проекта (`.claude/settings.json`)
3. Параметры пользователя (`~/.claude/settings.json`)

Программные опции, такие как `agents` и `allowed_tools`, переопределяют параметры пользователя, проекта и локальной файловой системы. Управляемые параметры политики имеют приоритет над программными опциями.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Конфигурация для подагента, определенного программно.

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

| Поле              | Обязательно | Описание                                                                                                                                                                                                                                            |
| :---------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | Да          | Описание на естественном языке, когда использовать этого агента                                                                                                                                                                                     |
| `prompt`          | Да          | Системная подсказка агента                                                                                                                                                                                                                          |
| `tools`           | Нет         | Массив разрешенных имен инструментов. Если опущено, наследует все инструменты                                                                                                                                                                       |
| `disallowedTools` | Нет         | Массив имен инструментов для удаления из набора инструментов агента. Также принимаются шаблоны уровня MCP server: `mcp__server` или `mcp__server__*` удаляет каждый инструмент с этого сервера, и `mcp__*` удаляет каждый MCP tool с любого сервера |
| `model`           | Нет         | Переопределение модели для этого агента. Принимает псевдоним, такой как `"sonnet"`, `"opus"`, `"haiku"` или `"inherit"`, или полный ID модели. Если опущено, использует основную модель                                                             |
| `skills`          | Нет         | Список имен skills для предварительной загрузки в контекст агента при запуске. Неуказанные skills остаются вызываемыми через инструмент Skill                                                                                                       |
| `memory`          | Нет         | Источник памяти для этого агента: `"user"`, `"project"` или `"local"`                                                                                                                                                                               |
| `mcpServers`      | Нет         | MCP servers, доступные этому агенту. Каждая запись - это имя сервера или встроенный словарь `{name: config}`                                                                                                                                        |
| `initialPrompt`   | Нет         | Автоматически отправляется как первый ход пользователя, когда этот агент работает как основной агент потока                                                                                                                                         |
| `maxTurns`        | Нет         | Максимальное количество агентских ходов перед остановкой агента                                                                                                                                                                                     |
| `background`      | Нет         | Запустите этого агента как неблокирующую фоновую задачу при вызове                                                                                                                                                                                  |
| `effort`          | Нет         | Уровень усилий рассуждения для этого агента. Принимает именованный уровень или целое число. См. [`EffortLevel`](#effortlevel)                                                                                                                       |
| `permissionMode`  | Нет         | Режим разрешений для выполнения инструментов в этом агенте. См. [`PermissionMode`](#permissionmode)                                                                                                                                                 |

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
    "auto",  # A model classifier approves or denies each tool call
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

Обратный вызов - это замена SDK для интерактивной подсказки разрешения: он вызывается только когда [permission evaluation flow](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) разрешается в подсказку. Вызовы инструментов, уже одобренные записью `allowed_tools`, правилом параметров разрешения или режимом разрешения, таким как `acceptEdits` или `bypassPermissions`, никогда его не вызывают. `AskUserQuestion`, MCP tools, отмеченные [`requiresUserInteraction`](/docs/ru/mcp#require-approval-for-a-specific-tool), и connector tools [установленные вашей организацией на `ask`](/docs/ru/mcp#organization-controls-on-connector-tools) достигают обратного вызова даже когда правило разрешения совпадает. В режиме `dontAsk` эти вызовы вместо этого отклоняются, без вызова обратного вызова. Чтобы контролировать каждый вызов инструмента, используйте вместо этого [hook `PreToolUse`](/docs/ru/agent-sdk/hooks).

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Информация контекста, передаваемая в обратные вызовы разрешения инструмента.

```python theme={null}
@dataclass
class ToolPermissionContext:
    signal: Any | None = None  # Future: abort signal support
    suggestions: list[PermissionUpdate] = field(default_factory=list)
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

Дополнительное поле `display` управляет тем, возвращается ли текст мышления `"summarized"` или `"omitted"`. На Claude Opus 4.7 и более поздних версиях API по умолчанию используется `"omitted"`, поэтому установите `"summarized"` для получения содержимого мышления в выводах [`ThinkingBlock`](#thinkingblock).

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

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Тип Literal для функций бета-версии SDK.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Используйте с полем `betas` в `ClaudeAgentOptions` для включения функций бета-версии.

<Warning>
  Бета-версия `context-1m-2025-08-07` снята с производства по состоянию на 30 апреля 2026 года. Передача этого заголовка с Claude Sonnet 4.5 или Sonnet 4 не имеет эффекта, и запросы, превышающие стандартное окно контекста 200k-токенов, возвращают ошибку. Чтобы использовать окно контекста 1M-токенов, перейдите на [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 или Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), которые включают контекст 1M по стандартной цене без требования заголовка бета-версии.
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
```

| Поле                 | Тип                         | Описание                                                                                 |
| :------------------- | :-------------------------- | :--------------------------------------------------------------------------------------- |
| `content`            | `str \| list[ContentBlock]` | Содержимое сообщения как текст или блоки содержимого                                     |
| `uuid`               | `str \| None`               | Уникальный идентификатор сообщения                                                       |
| `parent_tool_use_id` | `str \| None`               | ID использования инструмента, если это сообщение является ответом результата инструмента |
| `tool_use_result`    | `dict[str, Any] \| None`    | Данные результата инструмента, если применимо                                            |

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
```

| Поле                 | Тип                                                          | Описание                                                                                                 |
| :------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| `content`            | `list[ContentBlock]`                                         | Список блоков содержимого в ответе                                                                       |
| `model`              | `str`                                                        | Модель, которая создала ответ                                                                            |
| `parent_tool_use_id` | `str \| None`                                                | ID использования инструмента, если это вложенный ответ                                                   |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Тип ошибки, если ответ столкнулся с ошибкой                                                              |
| `usage`              | `dict[str, Any] \| None`                                     | Использование токенов для каждого сообщения (те же ключи, что и [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | ID сообщения API. Несколько сообщений из одного хода имеют одинаковый ID                                 |

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
    "max_output_tokens",
    "unknown",
]
```

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
    model_usage: dict[str, Any] | None = None
    permission_denials: list[Any] | None = None
    deferred_tool_use: DeferredToolUse | None = None
    errors: list[str] | None = None
    api_error_status: int | None = None
    uuid: str | None = None
```

Поле `subtype` определяет, какие другие поля заполнены. Это одно из `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"` или `"error_max_structured_output_retries"`. Dataclass Python объединяет все варианты в одну форму, поэтому поля, которые не применяются к возвращаемому подтипу, имеют значение `None`.

Несколько полей содержат диагностические детали, когда разговор заканчивается с ошибкой:

* `is_error`: `True`, когда разговор закончился в состоянии ошибки. Всегда `True` на подтипах `error_*`. На `subtype="success"` это `True`, когда финальный запрос модели не удался, что означает, что цикл агента завершился, но последний вызов API вернул ошибку.
* `api_error_status`: код состояния HTTP завершающей ошибки API. `None`, когда ход закончился без ошибки. Заполняется только на `subtype="success"`.
* `result`: текст финального сообщения помощника на `subtype="success"` или `None` на подтипах `error_*`. Когда `subtype="success"` и `is_error=True`, это содержит строку ошибки API, если она доступна, но может быть пустой, поэтому проверьте `api_error_status` и предыдущее содержимое `AssistantMessage` для деталей.
* `errors`: строки ошибок уровня цикла, такие как сообщение о максимальных ходах. Заполняется только на подтипах `error_*`.

Словарь `usage` содержит следующие ключи, если они присутствуют:

| Ключ                          | Тип   | Описание                                                                                                                                                                                                 |
| ----------------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | Входные токены, потребленные циклом агента верхнего уровня. [Токены подагента не включены](/docs/ru/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); используйте `model_usage` для учета всего дерева. |
| `output_tokens`               | `int` | Выходные токены, созданные циклом агента верхнего уровня. Токены подагента не включены.                                                                                                                  |
| `cache_creation_input_tokens` | `int` | Токены, используемые для создания новых записей кэша.                                                                                                                                                    |
| `cache_read_input_tokens`     | `int` | Токены, прочитанные из существующих записей кэша.                                                                                                                                                        |

Словарь `model_usage` отображает имена моделей на использование для каждой модели. Внутренние ключи словаря используют camelCase, потому что значение передается без изменений из базового процесса CLI, соответствуя типу TypeScript [`ModelUsage`](/docs/ru/agent-sdk/typescript#modelusage):

| Ключ                       | Тип     | Описание                                                                                                                                                                        |
| -------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | Входные токены для этой модели.                                                                                                                                                 |
| `outputTokens`             | `int`   | Выходные токены для этой модели.                                                                                                                                                |
| `cacheReadInputTokens`     | `int`   | Токены чтения кэша для этой модели.                                                                                                                                             |
| `cacheCreationInputTokens` | `int`   | Токены создания кэша для этой модели.                                                                                                                                           |
| `webSearchRequests`        | `int`   | Запросы веб-поиска, сделанные этой моделью.                                                                                                                                     |
| `costUSD`                  | `float` | Предполагаемая стоимость в USD для этой модели, вычисленная на стороне клиента. См. [Track cost and usage](/docs/ru/agent-sdk/cost-tracking) для предостережений выставления счетов. |
| `contextWindow`            | `int`   | Размер окна контекста для этой модели.                                                                                                                                          |
| `maxOutputTokens`          | `int`   | Максимальный лимит выходных токенов для этой модели.                                                                                                                            |

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

| Поле                      | Тип                       | Описание                                                                                                          |
| :------------------------ | :------------------------ | :---------------------------------------------------------------------------------------------------------------- |
| `status`                  | `RateLimitStatus`         | Текущий статус. `"allowed_warning"` означает приближение к лимиту; `"rejected"` означает, что лимит был достигнут |
| `resets_at`               | `int \| None`             | Временная метка Unix, когда окно ограничения скорости сбрасывается                                                |
| `rate_limit_type`         | `RateLimitType \| None`   | Какое окно ограничения скорости применяется                                                                       |
| `utilization`             | `float \| None`           | Доля потребленного ограничения скорости (0,0 до 1,0)                                                              |
| `overage_status`          | `RateLimitStatus \| None` | Статус использования переплаты по мере использования, если применимо                                              |
| `overage_resets_at`       | `int \| None`             | Временная метка Unix, когда окно переплаты сбрасывается                                                           |
| `overage_disabled_reason` | `str \| None`             | Почему переплата недоступна, если статус `"rejected"`                                                             |
| `raw`                     | `dict[str, Any]`          | Полный необработанный словарь из CLI, включая поля, не смоделированные выше                                       |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

Выдается при запуске фоновой задачи. Фоновая задача - это все, что отслеживается вне основного хода: фоновая команда Bash, наблюдение Monitor, подагент, порожденный через инструмент Agent, или удаленный агент. Поле `task_type` говорит вам, какой. Это именование не связано с переименованием инструмента `Task` на `Agent`.

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

<h2 id="content-block-types">
  Типы блоков содержимого
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Тип объединения всех блоков содержимого.

```python theme={null}
ContentBlock = TextBlock | ThinkingBlock | ToolUseBlock | ToolResultBlock
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

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Базовый класс исключения для всех ошибок SDK.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

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
  TypeScript SDK поддерживает дополнительные события hooks, которые еще недоступны в Python: `SessionStart`, `SessionEnd`, `Setup`, `TeammateIdle`, `TaskCompleted`, `ConfigChange`, `WorktreeCreate`, `WorktreeRemove`, `PostToolBatch` и `MessageDisplay`.
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

Возвращает [`HookJSONOutput`](#hookjsonoutput), который может содержать:

* `decision`: `"block"` для блокировки действия
* `systemMessage`: Предупреждающее сообщение, показываемое пользователю
* `hookSpecificOutput`: Вывод, специфичный для hooks

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
        None  # Timeout in seconds for all hooks in this matcher (default: 60)
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

| Поле              | Тип                             | Описание                                                                        |
| :---------------- | :------------------------------ | :------------------------------------------------------------------------------ |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Всегда "PostToolUseFailure"                                                     |
| `tool_name`       | `str`                           | Имя инструмента, который не удался                                              |
| `tool_input`      | `dict[str, Any]`                | Использованные входные параметры                                                |
| `tool_use_id`     | `str`                           | Уникальный идентификатор для этого использования инструмента                    |
| `error`           | `str`                           | Сообщение об ошибке из неудачного выполнения                                    |
| `is_interrupt`    | `bool` (опционально)            | Была ли ошибка вызвана прерыванием                                              |
| `agent_id`        | `str` (опционально)             | Идентификатор подагента, присутствует, когда hooks срабатывает внутри подагента |
| `agent_type`      | `str` (опционально)             | Тип подагента, присутствует, когда hooks срабатывает внутри подагента           |

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

`TypedDict`, содержащий имя события hooks и поля, специфичные для события. Форма зависит от значения `hookEventName`. Для полных деталей доступных полей для каждого события hooks см. [Control execution with hooks](/docs/ru/agent-sdk/hooks#outputs).

Дискриминированное объединение типов вывода, специфичных для события. Поле `hookEventName` определяет, какие поля действительны.

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
            ),  # Applies to all tools (default 60s timeout)
        ],
        "PostToolUse": [HookMatcher(hooks=[log_tool_use])],
    }
)

async for message in query(prompt="Analyze this codebase", options=options):
    print(message)
```

<h2 id="tool-input/output-types">
  Типы ввода/вывода инструментов
</h2>

Документация схем ввода/вывода для всех встроенных инструментов Claude Code. Хотя Python SDK не экспортирует их как типы, они представляют структуру входов и выходов инструментов в сообщениях.

<h3 id="agent">
  Agent
</h3>

**Имя инструмента:** `Agent` (ранее `Task`, который все еще принимается как псевдоним)

**Ввод:**

```python theme={null}
{
    "description": str,  # Краткое описание задачи (3-5 слов)
    "prompt": str,  # Задача для выполнения агентом
    "subagent_type": str,  # Тип специализированного агента для использования
}
```

**Вывод:**

```python theme={null}
{
    "result": str,  # Финальный результат от подагента
    "usage": dict | None,  # Статистика использования токенов
    "total_cost_usd": float | None,  # Предполагаемая общая стоимость в USD
    "duration_ms": int | None,  # Длительность выполнения в миллисекундах
}
```

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
                }
            ],
            "multiSelect": bool,  # Установите значение true, чтобы разрешить множественный выбор
        }
    ],
    "answers": dict[str, str | list[str]] | None,
    # Ответы пользователя, заполненные системой разрешений. Ответы с множественным выбором
    # могут быть списком меток или строкой, объединенной запятыми
}
```

**Вывод:**

```python theme={null}
{
    "questions": [  # Вопросы, которые были заданы
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Сопоставляет текст вопроса со строкой ответа
    # Ответы с множественным выбором разделены запятыми
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
    "output": str,  # Объединенный вывод stdout и stderr
    "exitCode": int,  # Код выхода команды
    "killed": bool | None,  # Была ли команда убита из-за тайм-аута
    "shellId": str | None,  # ID оболочки для фоновых процессов
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
    "timeout_ms": int | None,  # Завершить после этого срока (по умолчанию 300000, макс 3600000)
    "persistent": bool | None,  # Запускать в течение всего времени сеанса; остановить с помощью TaskStop
}
```

**Вывод:**

```python theme={null}
{
    "taskId": str,  # ID фоновой задачи мониторинга
    "timeoutMs": int,  # Срок тайм-аута в миллисекундах (0 при постоянном режиме)
    "persistent": bool | None,  # True при выполнении до TaskStop или конца сеанса
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
  Начиная с Claude Code v2.1.142, `TodoWrite` отключен по умолчанию. Используйте вместо этого `TaskCreate`, `TaskGet`, `TaskUpdate` и `TaskList`. См. [Миграция на инструменты Task](/docs/ru/agent-sdk/todo-tracking#migrate-to-task-tools) для обновления кода мониторинга, или установите `CLAUDE_CODE_ENABLE_TASKS=0` для возврата к `TodoWrite`.
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

<h3 id="bashoutput">
  BashOutput
</h3>

**Имя инструмента:** `BashOutput`

**Ввод:**

```python theme={null}
{
    "bash_id": str,  # ID фоновой оболочки
    "filter": str | None,  # Необязательное регулярное выражение для фильтрации строк вывода
}
```

**Вывод:**

```python theme={null}
{
    "output": str,  # Новый вывод с момента последней проверки
    "status": "running" | "completed" | "failed",  # Текущий статус оболочки
    "exitCode": int | None,  # Код выхода при завершении
}
```

<h3 id="killbash">
  KillBash
</h3>

**Имя инструмента:** `KillBash`

**Ввод:**

```python theme={null}
{
    "shell_id": str  # ID фоновой оболочки для завершения
}
```

**Вывод:**

```python theme={null}
{
    "message": str,  # Сообщение об успехе
    "shell_id": str,  # ID завершенной оболочки
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

<h2 id="advanced-features-with-claudesdkclient">
  Расширенные функции с ClaudeSDKClient
</h2>

<h3 id="building-a-continuous-conversation-interface">
  Построение интерфейса непрерывного разговора
</h3>

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

<h3 id="using-hooks-for-behavior-modification">
  Использование hooks для модификации поведения
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    HookMatcher,
    HookContext,
)
import asyncio
from typing import Any


async def pre_tool_logger(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage before execution."""
    tool_name = input_data.get("tool_name", "unknown")
    print(f"[PRE-TOOL] About to use: {tool_name}")

    # You can modify or block the tool execution here
    if tool_name == "Bash" and "rm -rf" in str(input_data.get("tool_input", {})):
        return {
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "deny",
                "permissionDecisionReason": "Dangerous command blocked",
            }
        }
    return {}


async def post_tool_logger(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log results after tool execution."""
    tool_name = input_data.get("tool_name", "unknown")
    print(f"[POST-TOOL] Completed: {tool_name}")
    return {}


async def user_prompt_modifier(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Add context to user prompts."""
    original_prompt = input_data.get("prompt", "")

    # Add a timestamp as additional context for Claude to see
    from datetime import datetime

    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    return {
        "hookSpecificOutput": {
            "hookEventName": "UserPromptSubmit",
            "additionalContext": f"[Submitted at {timestamp}] Original prompt: {original_prompt}",
        }
    }


async def main():
    options = ClaudeAgentOptions(
        hooks={
            "PreToolUse": [
                HookMatcher(hooks=[pre_tool_logger]),
                HookMatcher(matcher="Bash", hooks=[pre_tool_logger]),
            ],
            "PostToolUse": [HookMatcher(hooks=[post_tool_logger])],
            "UserPromptSubmit": [HookMatcher(hooks=[user_prompt_modifier])],
        },
        allowed_tools=["Read", "Write", "Bash"],
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("List files in current directory")

        async for message in client.receive_response():
            # Hooks will automatically log tool usage
            pass


asyncio.run(main())
```

<h3 id="real-time-progress-monitoring">
  Мониторинг прогресса в реальном времени
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ToolUseBlock,
    ToolResultBlock,
    TextBlock,
)
import asyncio


async def monitor_progress():
    options = ClaudeAgentOptions(
        allowed_tools=["Write", "Bash"], permission_mode="acceptEdits"
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Create 5 Python files with different sorting algorithms")

        # Monitor progress in real-time
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, ToolUseBlock):
                        if block.name == "Write":
                            file_path = block.input.get("file_path", "")
                            print(f"Creating: {file_path}")
                    elif isinstance(block, ToolResultBlock):
                        print("Completed tool execution")
                    elif isinstance(block, TextBlock):
                        print(f"Claude says: {block.text[:100]}...")

        print("Task completed!")


asyncio.run(monitor_progress())
```

<h2 id="example-usage">
  Пример использования
</h2>

<h3 id="basic-file-operations-using-query">
  Базовые операции с файлами (используя query)
</h3>

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
import asyncio


async def create_project():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"],
        permission_mode="acceptEdits",
        cwd="/home/user/project",
    )

    async for message in query(
        prompt="Create a Python project structure with setup.py", options=options
    ):
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, ToolUseBlock):
                    print(f"Using tool: {block.name}")


asyncio.run(create_project())
```

<h3 id="error-handling">
  Обработка ошибок
</h3>

```python theme={null}
from claude_agent_sdk import query, CLINotFoundError, ProcessError, CLIJSONDecodeError

try:
    async for message in query(prompt="Hello"):
        print(message)
except CLINotFoundError:
    print(
        "Claude Code CLI not found. Try reinstalling: pip install --force-reinstall claude-agent-sdk"
    )
except ProcessError as e:
    print(f"Process failed with exit code: {e.exit_code}")
except CLIJSONDecodeError as e:
    print(f"Failed to parse response: {e}")
```

<h3 id="streaming-mode-with-client">
  Режим потоковой передачи с клиентом
</h3>

```python theme={null}
from claude_agent_sdk import ClaudeSDKClient
import asyncio


async def interactive_session():
    async with ClaudeSDKClient() as client:
        # Send initial message
        await client.query("What's the weather like?")

        # Process responses
        async for msg in client.receive_response():
            print(msg)

        # Send follow-up
        await client.query("Tell me more about that")

        # Process follow-up response
        async for msg in client.receive_response():
            print(msg)


asyncio.run(interactive_session())
```

<h3 id="using-custom-tools-with-claudesdkclient">
  Использование пользовательских инструментов с ClaudeSDKClient
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    tool,
    create_sdk_mcp_server,
    AssistantMessage,
    TextBlock,
)
import asyncio
from typing import Any


# Define custom tools with @tool decorator
@tool("calculate", "Perform mathematical calculations", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        result = eval(args["expression"], {"__builtins__": {}})
        return {"content": [{"type": "text", "text": f"Result: {result}"}]}
    except Exception as e:
        return {
            "content": [{"type": "text", "text": f"Error: {str(e)}"}],
            "is_error": True,
        }


@tool("get_time", "Get current time", {})
async def get_time(args: dict[str, Any]) -> dict[str, Any]:
    from datetime import datetime

    current_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    return {"content": [{"type": "text", "text": f"Current time: {current_time}"}]}


async def main():
    # Create SDK MCP server with custom tools
    my_server = create_sdk_mcp_server(
        name="utilities", version="1.0.0", tools=[calculate, get_time]
    )

    # Configure options with the server
    options = ClaudeAgentOptions(
        mcp_servers={"utils": my_server},
        allowed_tools=["mcp__utils__calculate", "mcp__utils__get_time"],
    )

    # Use ClaudeSDKClient for interactive tool usage
    async with ClaudeSDKClient(options=options) as client:
        await client.query("What's 123 * 456?")

        # Process calculation response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Calculation: {block.text}")

        # Follow up with time query
        await client.query("What time is it now?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Time: {block.text}")


asyncio.run(main())
```

<h2 id="sandbox-configuration">
  Конфигурация sandbox
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

| Свойство                    | Тип                                                   | По умолчанию | Описание                                                                                                                                                                                                                                |
| :-------------------------- | :---------------------------------------------------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `bool`                                                | `False`      | Включить режим sandbox для выполнения команд                                                                                                                                                                                            |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`       | Автоматически одобрять команды bash, когда sandbox включен                                                                                                                                                                              |
| `excludedCommands`          | `list[str]`                                           | `[]`         | Команды, которые всегда обходят ограничения sandbox (например, `["docker"]`). Они работают без sandbox автоматически без участия модели                                                                                                 |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`       | Разрешить модели запрашивать выполнение команд вне sandbox. Когда `True`, модель может установить `dangerouslyDisableSandbox` в входе инструмента, что переходит к [системе разрешений](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`       | Конфигурация sandbox, специфичная для сети                                                                                                                                                                                              |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`       | Настройте, какие нарушения sandbox игнорировать                                                                                                                                                                                         |
| `enableWeakerNestedSandbox` | `bool`                                                | `False`      | Включить более слабый вложенный sandbox для совместимости                                                                                                                                                                               |

<Note>
  Sandbox зависит от поддержки платформы и, в Linux, от инструментов, таких как `bubblewrap` и `socat`. По умолчанию, когда `enabled` имеет значение `True`, но sandbox не может запуститься, команды выполняются без sandbox с предупреждением на stderr. Это поведение по умолчанию отличается от TypeScript SDK, где `failIfUnavailable` по умолчанию имеет значение `true`.

  Установите `"failIfUnavailable": True` в параметрах sandbox, чтобы остановиться вместо этого. Ключ еще не объявлен на `SandboxSettings`, но SDK передает его в Claude Code, который его соблюдает. `query()` затем сообщает `ResultMessage` с `subtype="error_during_execution"` и причину в `errors`. Следите за этим подтипом, а не ожидайте, что `query()` вызовет исключение перед выдачей сообщений.
</Note>

<h4 id="example-usage-2">
  Пример использования
</h4>

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, SandboxSettings

sandbox_settings: SandboxSettings = {
    "enabled": True,
    "autoAllowBashIfSandboxed": True,
    "network": {"allowLocalBinding": True},
}

async for message in query(
    prompt="Build and test my project",
    options=ClaudeAgentOptions(sandbox=sandbox_settings),
):
    print(message)
```

<Warning>
  **Безопасность Unix socket**: Опция `allowUnixSockets` может предоставить доступ к мощным системным сервисам. Например, разрешение `/var/run/docker.sock` фактически предоставляет полный доступ к хост-системе через Docker API, обходя изоляцию sandbox. Разрешайте только Unix sockets, которые строго необходимы, и поймите последствия безопасности каждого.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Конфигурация, специфичная для сети, для режима sandbox. Эти параметры применяются к sandboxed Bash командам, когда `enabled` имеет значение `True` в родительском [`SandboxSettings`](#sandboxsettings). Они не ограничивают инструмент WebFetch, который вместо этого использует [правила разрешений](/docs/ru/permissions#webfetch).

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

| Свойство                  | Тип         | По умолчанию | Описание                                                                                                                                                                                          |
| :------------------------ | :---------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowedDomains`          | `list[str]` | `[]`         | Имена доменов, к которым могут получить доступ процессы в sandbox                                                                                                                                 |
| `deniedDomains`           | `list[str]` | `[]`         | Имена доменов, к которым процессы в sandbox не могут получить доступ. Имеет приоритет над `allowedDomains`                                                                                        |
| `allowManagedDomainsOnly` | `bool`      | `False`      | Только управляемые параметры: когда установлено в управляемых параметрах, игнорируйте `allowedDomains` из источников неуправляемых параметров. Не имеет эффекта при установке через параметры SDK |
| `allowUnixSockets`        | `list[str]` | `[]`         | Пути Unix socket, к которым процессы могут получить доступ (например, Docker socket)                                                                                                              |
| `allowAllUnixSockets`     | `bool`      | `False`      | Разрешить доступ ко всем Unix sockets                                                                                                                                                             |
| `allowLocalBinding`       | `bool`      | `False`      | Разрешить процессам привязываться к локальным портам (например, для dev servers)                                                                                                                  |
| `allowMachLookup`         | `list[str]` | `[]`         | Только macOS: имена сервисов XPC/Mach для разрешения. Поддерживает завершающий подстановочный знак                                                                                                |
| `httpProxyPort`           | `int`       | `None`       | Порт HTTP proxy для сетевых запросов                                                                                                                                                              |
| `socksProxyPort`          | `int`       | `None`       | Порт SOCKS proxy для сетевых запросов                                                                                                                                                             |

<Note>
  Встроенный прокси sandbox обеспечивает соблюдение списка разрешенных сетей на основе запрашиваемого имени хоста и не завершает и не проверяет трафик TLS, поэтому такие методы, как [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting), потенциально могут его обойти. Подробнее см. в разделе [Ограничения безопасности Sandboxing](/docs/ru/sandboxing#security-limitations) и [Безопасное развертывание](/docs/ru/agent-sdk/secure-deployment#traffic-forwarding) для настройки прокси, завершающего TLS.
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

| Свойство  | Тип         | По умолчанию | Описание                                         |
| :-------- | :---------- | :----------- | :----------------------------------------------- |
| `file`    | `list[str]` | `[]`         | Шаблоны путей файлов для игнорирования нарушений |
| `network` | `list[str]` | `[]`         | Шаблоны сети для игнорирования нарушений         |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback разрешений для команд без sandbox
</h3>

Когда `allowUnsandboxedCommands` включен, модель может запросить выполнение команд вне sandbox, установив `dangerouslyDisableSandbox: True` во входе инструмента. Эти запросы переходят к существующей системе разрешений, что означает, что ваш обработчик `can_use_tool` будет вызван, позволяя вам реализовать пользовательскую логику авторизации.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Статический список команд, которые всегда обходят sandbox автоматически (например, `["docker"]`). Модель не имеет контроля над этим.
  * `allowUnsandboxedCommands`: Позволяет модели решать во время выполнения, запрашивать ли выполнение без sandbox, установив `dangerouslyDisableSandbox: True` во входе инструмента.
</Note>

```python theme={null}
from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    HookMatcher,
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


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
```

Этот шаблон позволяет вам:

* **Аудит запросов модели**: Логируйте, когда модель запрашивает выполнение без sandbox
* **Реализовать allowlists**: Разрешайте только определенные команды работать без sandbox
* **Добавить рабочие процессы одобрения**: Требуйте явной авторизации для привилегированных операций

<Warning>
  Команды, работающие с `dangerouslyDisableSandbox: True`, имеют полный доступ к системе. Убедитесь, что ваш обработчик `can_use_tool` тщательно проверяет эти запросы.

  Если `permission_mode` установлен на `bypassPermissions` и `allow_unsandboxed_commands` включен, модель может автономно выполнять команды вне sandbox без каких-либо подсказок одобрения (явное [`ask` правило](/docs/ru/agent-sdk/permissions#how-permissions-are-evaluated) все еще требует одного). Эта комбинация фактически позволяет модели молча выходить из изоляции sandbox.
</Warning>

<h2 id="see-also">
  См. также
</h2>

* [SDK overview](/docs/ru/agent-sdk/overview) - Общие концепции SDK
* [TypeScript SDK reference](/docs/ru/agent-sdk/typescript) - Документация TypeScript SDK
* [CLI reference](/docs/ru/cli-reference) - Интерфейс командной строки
* [Common workflows](/docs/ru/common-workflows) - Пошаговые руководства
