> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 참조 - Python

> Python Agent SDK의 완전한 API 참조로, 모든 함수, 타입 및 클래스를 포함합니다.

<h2 id="installation">
  설치
</h2>

최근 Debian, Ubuntu, 및 Homebrew Python 설치에서 가상 환경에 패키지를 설치합니다. 시스템 Python에 대해 `pip install`을 실행하면 `error: externally-managed-environment` 오류가 발생합니다.

```bash theme={null}
python3 -m venv .venv
source .venv/bin/activate
pip install claude-agent-sdk
```

uv, Windows PowerShell, 및 API 키 설정에 대해서는 [Agent SDK 개요에서 시작하기](/docs/ko/agent-sdk/overview#get-started)를 참조하십시오.

<h2 id="choosing-between-query-and-claudesdkclient">
  `query()`와 `ClaudeSDKClient` 중 선택하기
</h2>

Python SDK는 Claude Code와 상호작용하는 두 가지 방법을 제공합니다.

<h3 id="quick-comparison">
  빠른 비교
</h3>

| 기능            | `query()`                                  | `ClaudeSDKClient` |
| :------------ | :----------------------------------------- | :---------------- |
| **세션**        | 기본적으로 새 세션 생성                              | 동일한 세션 재사용        |
| **대화**        | 단일 교환                                      | 동일한 컨텍스트에서 여러 교환  |
| **연결**        | 자동으로 관리됨                                   | 수동 제어             |
| **스트리밍 입력**   | ✅ 지원됨                                      | ✅ 지원됨             |
| **중단**        | ❌ 지원 안 함                                   | ✅ 지원됨             |
| **Hooks**     | ✅ 지원됨                                      | ✅ 지원됨             |
| **사용자 정의 도구** | ✅ 지원됨                                      | ✅ 지원됨             |
| **대화 계속하기**   | `continue_conversation` 또는 `resume`을 통한 수동 | ✅ 자동              |
| **사용 사례**     | 일회성 작업                                     | 지속적인 대화           |

<h3 id="when-to-use-query-one-off-tasks">
  `query()` 사용 시기 (일회성 작업)
</h3>

**최적의 경우:**

* 대화 기록이 필요 없는 일회성 질문
* 이전 교환의 컨텍스트가 필요 없는 독립적인 작업
* 간단한 자동화 스크립트
* 매번 새로 시작하고 싶을 때

<h3 id="when-to-use-claudesdkclient-continuous-conversation">
  `ClaudeSDKClient` 사용 시기 (지속적인 대화)
</h3>

**최적의 경우:**

* **대화 계속하기** - Claude가 컨텍스트를 기억해야 할 때
* **후속 질문** - 이전 응답을 기반으로 구축
* **대화형 애플리케이션** - 채팅 인터페이스, REPL
* **응답 기반 로직** - 다음 작업이 Claude의 응답에 따라 달라질 때
* **세션 제어** - 대화 수명 주기를 명시적으로 관리

<h2 id="functions">
  함수
</h2>

<h3 id="query">
  `query()`
</h3>

Claude Code와의 각 상호작용을 위해 기본적으로 새 세션을 생성합니다. 메시지가 도착하면 생성하는 비동기 반복자를 반환합니다. `query()`에 대한 각 호출은 `continue_conversation=True` 또는 [`ClaudeAgentOptions`](#claudeagentoptions)에서 `resume`을 전달하지 않는 한 이전 상호작용의 메모리 없이 새로 시작합니다. [세션](/docs/ko/agent-sdk/sessions)을 참조하세요.

```python theme={null}
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None,
    transport: Transport | None = None
) -> AsyncIterator[Message]
```

<h4 id="parameters">
  매개변수
</h4>

| 매개변수        | 타입                           | 설명                                                  |
| :---------- | :--------------------------- | :-------------------------------------------------- |
| `prompt`    | `str \| AsyncIterable[dict]` | 문자열 또는 스트리밍 모드용 비동기 반복 가능 객체로서의 입력 프롬프트             |
| `options`   | `ClaudeAgentOptions \| None` | 선택적 구성 객체 (None인 경우 `ClaudeAgentOptions()`로 기본값 설정) |
| `transport` | `Transport \| None`          | CLI 프로세스와 통신하기 위한 선택적 사용자 정의 전송                     |

<h4 id="returns">
  반환값
</h4>

대화에서 메시지를 생성하는 `AsyncIterator[Message]`를 반환합니다.

<h4 id="example-with-options">
  예제 - 옵션 포함
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

타입 안전성을 갖춘 MCP 도구를 정의하기 위한 데코레이터입니다.

```python theme={null}
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any],
    annotations: ToolAnnotations | None = None
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

<h4 id="parameters-2">
  매개변수
</h4>

| 매개변수           | 타입                                              | 설명                               |
| :------------- | :---------------------------------------------- | :------------------------------- |
| `name`         | `str`                                           | 도구의 고유 식별자                       |
| `description`  | `str`                                           | 도구가 수행하는 작업에 대한 인간이 읽을 수 있는 설명   |
| `input_schema` | `type \| dict[str, Any]`                        | 도구의 입력 매개변수를 정의하는 스키마 (아래 참조)    |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | 클라이언트에 동작 힌트를 제공하는 선택적 MCP 도구 주석 |

<h4 id="input-schema-options">
  입력 스키마 옵션
</h4>

1. **간단한 타입 매핑** (권장):

   ```python theme={null}
   {"text": str, "count": int, "enabled": bool}
   ```

2. **JSON Schema 형식** (복잡한 검증용):
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
  반환값
</h4>

도구 구현을 래핑하고 `SdkMcpTool` 인스턴스를 반환하는 데코레이터 함수입니다.

<h4 id="example">
  예제
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

`mcp.types`에서 다시 내보낸 것입니다 (`from claude_agent_sdk import ToolAnnotations`로도 사용 가능). 모든 필드는 선택적 힌트이며, 클라이언트는 보안 결정을 위해 이에 의존해서는 안 됩니다.

| 필드                | 타입             | 기본값     | 설명                                                                                  |
| :---------------- | :------------- | :------ | :---------------------------------------------------------------------------------- |
| `title`           | `str \| None`  | `None`  | 도구의 인간이 읽을 수 있는 제목                                                                  |
| `readOnlyHint`    | `bool \| None` | `False` | `True`인 경우, 도구는 환경을 수정하지 않습니다                                                       |
| `destructiveHint` | `bool \| None` | `True`  | `True`인 경우, 도구는 파괴적인 업데이트를 수행할 수 있습니다 (`readOnlyHint`가 `False`일 때만 의미 있음)           |
| `idempotentHint`  | `bool \| None` | `False` | `True`인 경우, 동일한 인수로 반복 호출해도 추가 효과가 없습니다 (`readOnlyHint`가 `False`일 때만 의미 있음)         |
| `openWorldHint`   | `bool \| None` | `True`  | `True`인 경우, 도구는 외부 엔티티와 상호작용합니다 (예: 웹 검색). `False`인 경우, 도구의 도메인은 폐쇄적입니다 (예: 메모리 도구) |

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

Python 애플리케이션 내에서 실행되는 인프로세스 MCP 서버를 생성합니다.

```python theme={null}
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

<h4 id="parameters-3">
  매개변수
</h4>

| 매개변수      | 타입                              | 기본값       | 설명                          |
| :-------- | :------------------------------ | :-------- | :-------------------------- |
| `name`    | `str`                           | -         | 서버의 고유 식별자                  |
| `version` | `str`                           | `"1.0.0"` | 서버 버전 문자열                   |
| `tools`   | `list[SdkMcpTool[Any]] \| None` | `None`    | `@tool` 데코레이터로 생성된 도구 함수 목록 |

<h4 id="returns-3">
  반환값
</h4>

`ClaudeAgentOptions.mcp_servers`에 전달할 수 있는 `McpSdkServerConfig` 객체를 반환합니다.

<h4 id="example-2">
  예제
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

메타데이터가 포함된 과거 세션을 나열합니다. 프로젝트 디렉토리로 필터링하거나 모든 프로젝트의 세션을 나열합니다. 동기식이며 즉시 반환됩니다.

```python theme={null}
def list_sessions(
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0,
    include_worktrees: bool = True
) -> list[SDKSessionInfo]
```

<h4 id="parameters-4">
  매개변수
</h4>

| 매개변수                | 타입            | 기본값    | 설명                                                      |
| :------------------ | :------------ | :----- | :------------------------------------------------------ |
| `directory`         | `str \| None` | `None` | 세션을 나열할 디렉토리. 생략하면 모든 프로젝트의 세션을 반환합니다                   |
| `limit`             | `int \| None` | `None` | 반환할 최대 세션 수                                             |
| `offset`            | `int`         | `0`    | 정렬된 결과의 시작 부분에서 건너뛸 세션 수. `limit`과 함께 페이지 매김에 사용합니다     |
| `include_worktrees` | `bool`        | `True` | `directory`가 git 저장소 내에 있을 때, 모든 worktree 경로의 세션을 포함합니다 |

<h4 id="return-type-sdksessioninfo">
  반환 타입: `SDKSessionInfo`
</h4>

| 속성              | 타입            | 설명                                                  |
| :-------------- | :------------ | :-------------------------------------------------- |
| `session_id`    | `str`         | 고유 세션 식별자                                           |
| `summary`       | `str`         | 표시 제목: 사용자 정의 제목, 자동 생성된 요약 또는 첫 프롬프트               |
| `last_modified` | `int`         | 에포크 이후 마지막 수정 시간 (밀리초)                              |
| `file_size`     | `int \| None` | 세션 파일 크기 (바이트) (원격 저장소 백엔드의 경우 `None`)              |
| `custom_title`  | `str \| None` | 사용자가 설정한 세션 제목                                      |
| `first_prompt`  | `str \| None` | 세션의 첫 번째 의미 있는 사용자 프롬프트                             |
| `git_branch`    | `str \| None` | 세션 끝의 Git 브랜치                                       |
| `cwd`           | `str \| None` | 세션의 작업 디렉토리                                         |
| `tag`           | `str \| None` | 사용자가 설정한 세션 태그 ([`tag_session()`](#tag_session) 참조) |
| `created_at`    | `int \| None` | 에포크 이후 세션 생성 시간 (밀리초)                               |

<h4 id="example-3">
  예제
</h4>

프로젝트의 10개 최신 세션을 인쇄합니다. 결과는 `last_modified` 내림차순으로 정렬되므로 첫 번째 항목이 가장 최신입니다. `directory`를 생략하면 모든 프로젝트를 검색합니다.

```python theme={null}
from claude_agent_sdk import list_sessions

for session in list_sessions(directory="/path/to/project", limit=10):
    print(f"{session.summary} ({session.session_id})")
```

<h3 id="get_session_messages">
  `get_session_messages()`
</h3>

과거 세션에서 메시지를 검색합니다. 동기식이며 즉시 반환됩니다.

```python theme={null}
def get_session_messages(
    session_id: str,
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0
) -> list[SessionMessage]
```

<h4 id="parameters-5">
  매개변수
</h4>

| 매개변수         | 타입            | 기본값    | 설명                                 |
| :----------- | :------------ | :----- | :--------------------------------- |
| `session_id` | `str`         | 필수     | 메시지를 검색할 세션 ID                     |
| `directory`  | `str \| None` | `None` | 검색할 프로젝트 디렉토리. 생략하면 모든 프로젝트를 검색합니다 |
| `limit`      | `int \| None` | `None` | 반환할 최대 메시지 수                       |
| `offset`     | `int`         | `0`    | 시작 부분에서 건너뛸 메시지 수                  |

<h4 id="return-type-sessionmessage">
  반환 타입: `SessionMessage`
</h4>

| 속성                   | 타입                             | 설명            |
| :------------------- | :----------------------------- | :------------ |
| `type`               | `Literal["user", "assistant"]` | 메시지 역할        |
| `uuid`               | `str`                          | 고유 메시지 식별자    |
| `session_id`         | `str`                          | 세션 식별자        |
| `message`            | `Any`                          | 원본 메시지 콘텐츠    |
| `parent_tool_use_id` | `None`                         | 향후 사용을 위해 예약됨 |

<h4 id="example-4">
  예제
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

전체 프로젝트 디렉토리를 스캔하지 않고 ID로 단일 세션의 메타데이터를 읽습니다. 동기식이며 즉시 반환됩니다.

```python theme={null}
def get_session_info(
    session_id: str,
    directory: str | None = None,
) -> SDKSessionInfo | None
```

<h4 id="parameters-6">
  매개변수
</h4>

| 매개변수         | 타입            | 기본값    | 설명                                     |
| :----------- | :------------ | :----- | :------------------------------------- |
| `session_id` | `str`         | 필수     | 조회할 세션의 UUID                           |
| `directory`  | `str \| None` | `None` | 프로젝트 디렉토리 경로. 생략하면 모든 프로젝트 디렉토리를 검색합니다 |

[`SDKSessionInfo`](#return-type-sdksessioninfo)를 반환하거나, 세션을 찾을 수 없으면 `None`을 반환합니다.

<h4 id="example-5">
  예제
</h4>

프로젝트 디렉토리를 스캔하지 않고 단일 세션의 메타데이터를 조회합니다. 이전 실행에서 세션 ID를 이미 가지고 있을 때 유용합니다.

```python theme={null}
from claude_agent_sdk import get_session_info

info = get_session_info("550e8400-e29b-41d4-a716-446655440000")
if info:
    print(f"{info.summary} (branch: {info.git_branch}, tag: {info.tag})")
```

<h3 id="rename_session">
  `rename_session()`
</h3>

사용자 정의 제목 항목을 추가하여 세션의 이름을 바꿉니다. 반복 호출은 안전하며, 가장 최신 제목이 우선합니다. 동기식입니다.

```python theme={null}
def rename_session(
    session_id: str,
    title: str,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-7">
  매개변수
</h4>

| 매개변수         | 타입            | 기본값    | 설명                                     |
| :----------- | :------------ | :----- | :------------------------------------- |
| `session_id` | `str`         | 필수     | 이름을 바꿀 세션의 UUID                        |
| `title`      | `str`         | 필수     | 새 제목. 공백을 제거한 후 비어 있지 않아야 합니다          |
| `directory`  | `str \| None` | `None` | 프로젝트 디렉토리 경로. 생략하면 모든 프로젝트 디렉토리를 검색합니다 |

`session_id`가 유효한 UUID가 아니거나 `title`이 비어 있으면 `ValueError`를 발생시킵니다. 세션을 찾을 수 없으면 `FileNotFoundError`를 발생시킵니다.

<h4 id="example-6">
  예제
</h4>

가장 최신 세션의 이름을 바꿔서 나중에 찾기 쉽게 합니다. 새 제목은 이후 읽기에서 [`SDKSessionInfo.custom_title`](#return-type-sdksessioninfo)에 나타납니다.

```python theme={null}
from claude_agent_sdk import list_sessions, rename_session

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    rename_session(sessions[0].session_id, "Refactor auth module")
```

<h3 id="tag_session">
  `tag_session()`
</h3>

세션에 태그를 지정합니다. 태그를 지우려면 `None`을 전달합니다. 반복 호출은 안전하며, 가장 최신 태그가 우선합니다. 동기식입니다.

```python theme={null}
def tag_session(
    session_id: str,
    tag: str | None,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-8">
  매개변수
</h4>

| 매개변수         | 타입            | 기본값    | 설명                                       |
| :----------- | :------------ | :----- | :--------------------------------------- |
| `session_id` | `str`         | 필수     | 태그를 지정할 세션의 UUID                         |
| `tag`        | `str \| None` | 필수     | 태그 문자열 또는 지우려면 `None`. 저장하기 전에 유니코드 정규화됨 |
| `directory`  | `str \| None` | `None` | 프로젝트 디렉토리 경로. 생략하면 모든 프로젝트 디렉토리를 검색합니다   |

`session_id`가 유효한 UUID가 아니거나 `tag`가 정규화 후 비어 있으면 `ValueError`를 발생시킵니다. 세션을 찾을 수 없으면 `FileNotFoundError`를 발생시킵니다.

<h4 id="example-7">
  예제
</h4>

세션에 태그를 지정한 다음, 나중에 읽을 때 해당 태그로 필터링합니다. 기존 태그를 지우려면 `None`을 전달합니다.

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
  클래스
</h2>

<h3 id="claudesdkclient">
  `ClaudeSDKClient`
</h3>

**여러 교환에 걸쳐 대화 세션을 유지합니다.** 이것은 TypeScript SDK의 `query()` 함수가 내부적으로 작동하는 방식의 Python 동등물입니다 - 대화를 계속할 수 있는 클라이언트 객체를 생성합니다.

<h4 id="key-features">
  주요 기능
</h4>

* **세션 연속성**: 여러 `query()` 호출에 걸쳐 대화 컨텍스트 유지
* **동일한 대화**: 세션이 이전 메시지를 유지합니다
* **중단 지원**: 작업 중간에 실행을 중지할 수 있습니다
* **명시적 수명 주기**: 세션이 시작되고 끝나는 시점을 제어합니다
* **응답 기반 흐름**: 응답에 반응하고 후속 조치를 보낼 수 있습니다
* **사용자 정의 도구 및 hooks**: 사용자 정의 도구 (`@tool` 데코레이터로 생성) 및 hooks를 지원합니다

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
  메서드
</h4>

| 메서드                                       | 설명                                                                                                           |
| :---------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| `__init__(options)`                       | 선택적 구성으로 클라이언트 초기화                                                                                           |
| `connect(prompt)`                         | 선택적 초기 프롬프트 또는 메시지 스트림으로 Claude에 연결                                                                          |
| `query(prompt, session_id)`               | 스트리밍 모드에서 새 요청 전송                                                                                            |
| `receive_messages()`                      | Claude의 모든 메시지를 비동기 반복자로 수신                                                                                  |
| `receive_response()`                      | ResultMessage를 포함하여 메시지 수신                                                                                   |
| `interrupt()`                             | 중단 신호 전송 (스트리밍 모드에서만 작동)                                                                                     |
| `set_permission_mode(mode)`               | 현재 세션의 권한 모드 변경                                                                                              |
| `set_model(model)`                        | 현재 세션의 모델 변경. 기본값으로 재설정하려면 `None` 전달                                                                         |
| `rewind_files(user_message_id)`           | 지정된 사용자 메시지의 상태로 파일 복원. `enable_file_checkpointing=True` 필요. [파일 체크포인팅](/docs/ko/agent-sdk/file-checkpointing) 참조 |
| `get_mcp_status()`                        | 구성된 모든 MCP 서버의 상태 가져오기. [`McpStatusResponse`](#mcpstatusresponse) 반환                                         |
| `reconnect_mcp_server(server_name)`       | 실패했거나 연결이 끊긴 MCP 서버에 다시 연결 시도                                                                                |
| `toggle_mcp_server(server_name, enabled)` | 세션 중간에 MCP 서버 활성화 또는 비활성화. 비활성화하면 도구 제거                                                                      |
| `stop_task(task_id)`                      | 실행 중인 백그라운드 작업 중지. 상태 `"stopped"`인 [`TaskNotificationMessage`](#tasknotificationmessage)가 메시지 스트림에서 따릅니다     |
| `get_server_info()`                       | 세션 ID 및 기능을 포함한 서버 정보 가져오기                                                                                   |
| `disconnect()`                            | Claude에서 연결 해제                                                                                               |

<h4 id="context-manager-support">
  컨텍스트 관리자 지원
</h4>

클라이언트는 자동 연결 관리를 위한 비동기 컨텍스트 관리자로 사용할 수 있습니다.

```python theme={null}
async with ClaudeSDKClient() as client:
    await client.query("Hello Claude")
    async for message in client.receive_response():
        print(message)
```

> **중요:** 메시지를 반복할 때, asyncio 정리 문제를 일으킬 수 있으므로 `break`를 사용하여 조기에 종료하지 마십시오. 대신 반복이 자연스럽게 완료되도록 하거나 플래그를 사용하여 필요한 것을 찾았을 때를 추적하십시오.

<h4 id="example-continuing-a-conversation">
  예제 - 대화 계속하기
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
  예제 - ClaudeSDKClient를 사용한 스트리밍 입력
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
  예제 - 중단 사용
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
  **중단 후 버퍼 동작:** `interrupt()`는 중지 신호를 보내지만 메시지 버퍼를 지우지 않습니다. 중단된 작업에서 이미 생성된 메시지 (해당 `ResultMessage` 포함, `subtype="error_during_execution"`)는 스트림에 남아 있습니다. 새 쿼리의 응답을 읽기 전에 `receive_response()`로 이들을 드레인해야 합니다. `interrupt()` 직후에 새 쿼리를 보내고 `receive_response()`를 한 번만 호출하면, 새 쿼리의 응답이 아닌 중단된 작업의 메시지를 받게 됩니다.
</Note>

<h4 id="example-advanced-permission-control">
  예제 - 고급 권한 제어
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
    # 게이트된 도구를 allowed_tools에도 나열하지 마십시오: allow 규칙은 can_use_tool이 실행되기 전에 호출을 승인합니다
    options = ClaudeAgentOptions(can_use_tool=custom_permission_handler)

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Update the system config file")

        async for message in client.receive_response():
            # Will use sandbox path instead
            print(message)


asyncio.run(main())
```

<h2 id="types">
  타입
</h2>

<Note>
  **`@dataclass` vs `TypedDict`:** 이 SDK는 두 가지 종류의 타입을 사용합니다. `@dataclass`로 장식된 클래스 (`ResultMessage`, `AgentDefinition`, `TextBlock`)는 런타임에 객체 인스턴스이며 속성 접근을 지원합니다: `msg.result`. `TypedDict`로 정의된 클래스 (`ThinkingConfigEnabled`, `McpStdioServerConfig`, `SyncHookJSONOutput`)는 **런타임에 일반 딕셔너리**이며 키 접근이 필요합니다: `config["budget_tokens"]`, `config.budget_tokens`가 아닙니다. `ClassName(field=value)` 호출 구문은 둘 다에서 작동하지만, dataclass만 속성이 있는 객체를 생성합니다.
</Note>

<h3 id="sdkmcptool">
  `SdkMcpTool`
</h3>

`@tool` 데코레이터로 생성된 SDK MCP 도구의 정의입니다.

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| 속성             | 타입                                         | 설명                                                                                   |
| :------------- | :----------------------------------------- | :----------------------------------------------------------------------------------- |
| `name`         | `str`                                      | 도구의 고유 식별자                                                                           |
| `description`  | `str`                                      | 인간이 읽을 수 있는 설명                                                                       |
| `input_schema` | `type[T] \| dict[str, Any]`                | 입력 검증을 위한 스키마                                                                        |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]` | 도구 실행을 처리하는 비동기 함수                                                                   |
| `annotations`  | `ToolAnnotations \| None`                  | 선택적 MCP 도구 주석 (예: `readOnlyHint`, `destructiveHint`, `openWorldHint`). `mcp.types`에서 |

<h3 id="transport">
  `Transport`
</h3>

사용자 정의 전송 구현을 위한 추상 기본 클래스입니다. 이를 사용하여 사용자 정의 채널 (예: 로컬 서브프로세스 대신 원격 연결)을 통해 Claude 프로세스와 통신합니다.

<Warning>
  이것은 낮은 수준의 내부 API입니다. 인터페이스는 향후 릴리스에서 변경될 수 있습니다. 사용자 정의 구현은 인터페이스 변경에 맞게 업데이트되어야 합니다.
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

| 메서드               | 설명                                    |
| :---------------- | :------------------------------------ |
| `connect()`       | 전송을 연결하고 통신을 준비합니다                    |
| `write(data)`     | 원본 데이터 (JSON + 줄바꿈)를 전송에 씁니다          |
| `read_messages()` | 구문 분석된 JSON 메시지를 생성하는 비동기 반복자         |
| `close()`         | 연결을 닫고 리소스를 정리합니다                     |
| `is_ready()`      | 전송이 송수신할 수 있으면 `True`를 반환합니다          |
| `end_input()`     | 입력 스트림을 닫습니다 (예: 서브프로세스 전송의 stdin 닫기) |

가져오기: `from claude_agent_sdk import Transport`

<h3 id="claudeagentoptions">
  `ClaudeAgentOptions`
</h3>

Claude Code 쿼리를 위한 구성 dataclass입니다.

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

| 속성                            | 타입                                                                                    | 기본값                     | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :---------------------------- | :------------------------------------------------------------------------------------ | :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                  | 도구 구성. Claude Code의 기본 도구를 위해 `{"type": "preset", "preset": "claude_code"}` 사용                                                                                                                                                                                                                                                                                                                                                                              |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                    | 프롬프트 없이 자동 승인할 도구. 이것은 Claude를 이 도구로만 제한하지 않습니다. 나열되지 않은 도구는 `permission_mode` 및 `can_use_tool`로 넘어갑니다. `disallowed_tools`를 사용하여 도구를 차단합니다. [권한](/docs/ko/agent-sdk/permissions#allow-and-deny-rules) 참조                                                                                                                                                                                                                                                         |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptFile \| None`                               | `None`                  | 시스템 프롬프트 구성. 사용자 정의 프롬프트의 경우 문자열을 전달하거나, Claude Code의 시스템 프롬프트를 위해 `{"type": "preset", "preset": "claude_code"}`를 선택적 `"append"`와 함께 사용하거나, `{"type": "file", "path": "..."}` 형식으로 디스크에서 큰 프롬프트를 로드합니다. [`SystemPromptPreset`](#systempromptpreset) 및 [`SystemPromptFile`](#systempromptfile) 참조                                                                                                                                                            |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                    | MCP 서버 구성 또는 구성 파일 경로                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `strict_mcp_config`           | `bool`                                                                                | `False`                 | `True`일 때, `mcp_servers`에 전달된 서버만 사용하고 프로젝트 `.mcp.json`, 사용자 설정, 플러그인 제공 MCP 서버 및 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 무시합니다. CLI `--strict-mcp-config` 플래그에 매핑됩니다                                                                                                                                                                                                                                                                      |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                  | 도구 사용을 위한 권한 모드                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `continue_conversation`       | `bool`                                                                                | `False`                 | 가장 최신 대화 계속하기                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `resume`                      | `str \| None`                                                                         | `None`                  | 재개할 세션 ID                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `max_turns`                   | `int \| None`                                                                         | `None`                  | 최대 에이전트 턴 (도구 사용 왕복)                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                  | 클라이언트 측 비용 추정이 이 USD 값에 도달하면 쿼리 중지. `total_cost_usd`와 동일한 추정과 비교됨. 정확도 주의 사항은 [비용 및 사용량 추적](/docs/ko/agent-sdk/cost-tracking) 참조                                                                                                                                                                                                                                                                                                                                 |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                    | 거부할 도구. `"Bash"`와 같은 단순 이름은 Claude의 컨텍스트에서 도구를 제거합니다. `"Bash(rm *)"` 같은 범위 지정 규칙은 도구를 사용 가능하게 유지하고 `bypassPermissions`를 포함한 모든 권한 모드에서 일치하는 호출을 거부합니다. [권한](/docs/ko/agent-sdk/permissions#allow-and-deny-rules) 참조                                                                                                                                                                                                                                              |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                 | 되감기를 위한 파일 변경 추적 활성화. [파일 체크포인팅](/docs/ko/agent-sdk/file-checkpointing) 참조                                                                                                                                                                                                                                                                                                                                                                                       |
| `model`                       | `str \| None`                                                                         | `None`                  | Claude 모델 별칭 또는 전체 모델 이름. [허용되는 값 및 공급자별 ID](/docs/ko/model-config#available-models) 참조                                                                                                                                                                                                                                                                                                                                                                          |
| `fallback_model`              | `str \| None`                                                                         | `None`                  | 기본 모델이 실패할 경우 사용할 폴백 모델                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                    | 활성화할 베타 기능. 사용 가능한 옵션은 [`SdkBeta`](#sdkbeta) 참조                                                                                                                                                                                                                                                                                                                                                                                                             |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                  | 구조화된 응답을 위한 출력 형식 (예: `{"type": "json_schema", "schema": {...}}`). 자세한 내용은 [구조화된 출력](/docs/ko/agent-sdk/structured-outputs) 참조                                                                                                                                                                                                                                                                                                                                   |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                  | 권한 프롬프트를 위한 MCP 도구 이름                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                  | 현재 작업 디렉토리                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                  | Claude Code CLI 실행 파일의 사용자 정의 경로                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `settings`                    | `str \| None`                                                                         | `None`                  | 설정 파일 경로                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                    | Claude가 접근할 수 있는 추가 디렉토리                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `env`                         | `dict[str, str]`                                                                      | `{}`                    | 상속된 프로세스 환경 위에 병합된 환경 변수. 기본 CLI가 읽는 변수는 [환경 변수](/docs/ko/env-vars) 참조. 시간 초과 관련 변수는 [느리거나 정지된 API 응답 처리](#handle-slow-or-stalled-api-responses) 참조                                                                                                                                                                                                                                                                                                              |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                    | CLI에 직접 전달할 추가 CLI 인수                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                  | CLI stdout 버퍼링 시 최대 바이트                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`            | *Deprecated* - 디버그 출력을 위한 파일 유사 객체. 대신 `stderr` 콜백 사용                                                                                                                                                                                                                                                                                                                                                                                                       |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                  | CLI의 stderr 출력을 위한 콜백 함수                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                  | 도구 권한 콜백 함수. [권한 흐름](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)이 프롬프트로 넘어갈 때만 호출됩니다. `allowed_tools`, 허용 규칙 또는 `permission_mode`로 자동 승인된 호출에 대해서는 호출되지 않습니다. `AskUserQuestion`, 커넥터 도구 [조직이 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools), 및 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구는 허용 규칙이 일치하더라도 이에 도달합니다. `dontAsk` 모드에서는 대신 거부됩니다. [`CanUseTool`](#canusetool)에서 자세한 내용 참조 |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                  | 이벤트 가로채기를 위한 hook 구성                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `user`                        | `str \| None`                                                                         | `None`                  | 사용자 식별자                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `include_partial_messages`    | `bool`                                                                                | `False`                 | 부분 메시지 스트리밍 이벤트 포함. 활성화되면 [`StreamEvent`](#streamevent) 메시지가 생성됩니다                                                                                                                                                                                                                                                                                                                                                                                          |
| `include_hook_events`         | `bool`                                                                                | `False`                 | 메시지 스트림에 hook 라이프사이클 이벤트를 `HookEventMessage` 객체로 포함                                                                                                                                                                                                                                                                                                                                                                                                         |
| `fork_session`                | `bool`                                                                                | `False`                 | `resume`으로 재개할 때, 원본 세션을 계속하는 대신 새 세션 ID로 포크합니다                                                                                                                                                                                                                                                                                                                                                                                                             |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                  | 프로그래밍 방식으로 정의된 서브에이전트                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                    | 로컬 경로에서 사용자 정의 플러그인 로드. 자세한 내용은 [플러그인](/docs/ko/agent-sdk/plugins) 참조                                                                                                                                                                                                                                                                                                                                                                                            |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                  | 프로그래밍 방식으로 샌드박스 동작 구성. 자세한 내용은 [샌드박스 설정](#sandboxsettings) 참조                                                                                                                                                                                                                                                                                                                                                                                               |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (CLI 기본값: 모든 소스) | 로드할 파일 시스템 설정을 제어합니다. 사용자, 프로젝트 및 로컬 설정을 비활성화하려면 `[]`를 전달합니다. 관리형 정책 설정은 어쨌든 로드됩니다. 서버 관리 설정은 [적격 구성](/docs/ko/server-managed-settings#platform-availability)에서 조직 자격증명으로 세션이 인증될 때 가져옵니다. [Claude Code 기능 사용](/docs/ko/agent-sdk/claude-code-features#what-settingsources-does-not-control)에서 이것이 제어하지 않는 입력 및 비활성화 방법 참조                                                                                                                                            |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                  | 세션에서 사용 가능한 스킬. 모든 발견된 스킬을 활성화하려면 `"all"`을 전달하거나, 스킬 이름 목록을 전달합니다. 설정하면 SDK는 `allowed_tools`에 Skill 도구를 자동으로 추가합니다. `tools`도 전달하는 경우 해당 목록에 `"Skill"`을 포함합니다. [스킬](/docs/ko/agent-sdk/skills) 참조                                                                                                                                                                                                                                                                 |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                  | *Deprecated* - 생각 블록의 최대 토큰. 대신 `thinking` 사용                                                                                                                                                                                                                                                                                                                                                                                                               |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                  | 확장된 생각 동작을 제어합니다. `max_thinking_tokens`보다 우선합니다                                                                                                                                                                                                                                                                                                                                                                                                             |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                  | 생각 깊이를 위한 노력 수준. [노력 수준 조정](/docs/ko/model-config#adjust-effort-level) 참조                                                                                                                                                                                                                                                                                                                                                                                        |
| `session_store`               | [`SessionStore`](/docs/ko/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                  | 세션 기록을 외부 백엔드로 미러링하여 모든 호스트가 이를 재개할 수 있도록 합니다. [외부 저장소에 세션 유지](/docs/ko/agent-sdk/session-storage) 참조                                                                                                                                                                                                                                                                                                                                                            |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`             | `session_store`에 미러링된 기록 항목을 플러시할 시기. `"batched"`는 턴당 한 번 또는 버퍼가 가득 찰 때 플러시합니다. `"eager"`는 모든 프레임 후에 백그라운드 플러시를 트리거합니다. `session_store`가 `None`일 때 무시됩니다                                                                                                                                                                                                                                                                                                    |

<h4 id="handle-slow-or-stalled-api-responses">
  느리거나 정지된 API 응답 처리
</h4>

CLI 서브프로세스는 API 시간 초과 및 정지 감지를 제어하는 여러 환경 변수를 읽습니다. `ClaudeAgentOptions.env`를 통해 전달합니다:

```python theme={null}
options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS`: Anthropic 클라이언트의 요청당 시간 초과 (밀리초). 기본값 `600000`. 주 루프 및 모든 서브에이전트에 적용됩니다.
* `CLAUDE_CODE_MAX_RETRIES`: 최대 API 재시도. 기본값 `10`, 최대 `15`로 제한됨. 각 재시도는 자체 `API_TIMEOUT_MS` 윈도우를 가지므로, 최악의 경우 벽시간은 대략 `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` 더하기 백오프입니다. 더 긴 중단을 기다려야 하는 무인 실행의 경우, `CLAUDE_CODE_RETRY_WATCHDOG=1`을 설정하여 용량 오류를 무한정 재시도합니다. 그리고 Claude Code v2.1.199 기준으로 다른 일시적 오류의 기본값을 `300`으로 올리고 이 변수의 상한을 제거합니다.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: `run_in_background`으로 시작된 서브에이전트의 정지 감시견. 기본값 `600000`. 각 스트림 이벤트에서 재설정됩니다. 정지 시 서브에이전트를 중단하고, 작업을 실패로 표시하고, 부분 결과와 함께 오류를 부모에게 표시합니다. 동기 서브에이전트에는 적용되지 않습니다.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` with `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: 헤더가 도착했지만 응답 본문이 스트리밍을 중지할 때 요청을 중단합니다. 감시견은 모든 공급자에 대해 기본적으로 켜져 있습니다. `CLAUDE_ENABLE_STREAM_WATCHDOG=0`으로 설정하여 비활성화합니다. `CLAUDE_STREAM_IDLE_TIMEOUT_MS`는 기본값 `300000`이고 해당 최소값으로 제한됩니다. 중단된 요청은 정상 재시도 경로를 거칩니다.

<h3 id="outputformat">
  `OutputFormat`
</h3>

구조화된 출력 검증을 위한 구성입니다. 이를 `ClaudeAgentOptions`의 `output_format` 필드에 `dict`로 전달합니다:

```python theme={null}
# output_format에 대한 예상 dict 형태
{
    "type": "json_schema",
    "schema": {...},  # Your JSON Schema definition
}
```

| 필드       | 필수 | 설명                                        |
| :------- | :- | :---------------------------------------- |
| `type`   | 예  | JSON Schema 검증을 위해 `"json_schema"`이어야 합니다 |
| `schema` | 예  | 출력 검증을 위한 JSON Schema 정의                  |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

선택적 추가 사항과 함께 Claude Code의 프리셋 시스템 프롬프트를 사용하기 위한 구성입니다.

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
```

| 필드                         | 필수  | 설명                                                                                                                                                                                                           |
| :------------------------- | :-- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | 예   | 프리셋 시스템 프롬프트를 사용하려면 `"preset"`이어야 합니다                                                                                                                                                                        |
| `preset`                   | 예   | Claude Code의 시스템 프롬프트를 사용하려면 `"claude_code"`이어야 합니다                                                                                                                                                          |
| `append`                   | 아니오 | 프리셋 시스템 프롬프트에 추가할 추가 지침                                                                                                                                                                                      |
| `exclude_dynamic_sections` | 아니오 | 작업 디렉토리, git 상태 및 메모리 경로와 같은 세션별 컨텍스트를 시스템 프롬프트에서 첫 사용자 메시지로 이동합니다. 사용자 및 머신 간 프롬프트 캐시 재사용을 개선합니다. [시스템 프롬프트 수정](/docs/ko/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) 참조 |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

파일에서 사용자 정의 시스템 프롬프트를 로드하기 위한 구성입니다. 문자열로 전달하는 대신 파일 형식을 사용합니다. SDK는 이를 CLI [`--system-prompt-file`](/docs/ko/cli-reference#system-prompt-flags) 플래그에 매핑합니다. 프롬프트가 큰 경우 파일 형식을 사용합니다: SDK는 문자열 `system_prompt`를 CLI 서브프로세스 argv에 전달하며, 이는 SDK가 API 요청을 보내기 전에 OS 명령줄 길이 제한의 대상입니다. Linux에서 대략 128 KB보다 긴 단일 인수는 `Argument list too long` 오류로 프로세스 생성에 실패합니다. Windows에서는 전체 명령줄이 대략 32 KB로 제한되므로 문자열 형식은 더 낮은 임계값에서 실패합니다.

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| 필드     | 필수 | 설명                                |
| :----- | :- | :-------------------------------- |
| `type` | 예  | 디스크에서 프롬프트를 로드하려면 `"file"`이어야 합니다 |
| `path` | 예  | 시스템 프롬프트를 포함하는 파일의 경로             |

<h3 id="settingsource">
  `SettingSource`
</h3>

SDK가 설정을 로드하는 파일 시스템 기반 구성 소스를 제어합니다.

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| 값           | 설명                      | 위치                            |
| :---------- | :---------------------- | :---------------------------- |
| `"user"`    | 전역 사용자 설정               | `~/.claude/settings.json`     |
| `"project"` | 공유 프로젝트 설정 (버전 제어됨)     | `.claude/settings.json`       |
| `"local"`   | 로컬 프로젝트 설정 (버전 제어되지 않음) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  기본 동작
</h4>

`setting_sources`가 생략되거나 `None`일 때, `query()`는 Claude Code CLI와 동일한 파일 시스템 설정을 로드합니다: 사용자, 프로젝트 및 로컬. 관리형 정책 설정은 모든 경우에 로드됩니다. 서버 관리 설정은 [적격 구성](/docs/ko/server-managed-settings#platform-availability)에서 조직 자격증명으로 세션이 인증될 때 가져옵니다. [Claude Code 기능 사용](/docs/ko/agent-sdk/claude-code-features#what-settingsources-does-not-control)에서 이것이 제어하지 않는 입력 및 비활성화 방법을 참조하십시오.

<h4 id="why-use-setting_sources">
  setting\_sources를 사용하는 이유
</h4>

**파일 시스템 설정 비활성화:**

```python theme={null}
# 디스크에서 사용자, 프로젝트 또는 로컬 설정을 로드하지 않습니다
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
  Python SDK 0.1.59 이하에서는 빈 목록이 옵션을 생략하는 것과 동일하게 처리되었으므로 `setting_sources=[]`는 파일 시스템 설정을 비활성화하지 않았습니다. 빈 목록이 적용되어야 하는 경우 최신 릴리스로 업그레이드하십시오. TypeScript SDK는 영향을 받지 않습니다.
</Note>

**모든 파일 시스템 설정을 명시적으로 로드:**

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

**특정 설정 소스만 로드:**

```python theme={null}
# 프로젝트 설정만 로드, 사용자 및 로컬 무시
async for message in query(
    prompt="Run CI checks",
    options=ClaudeAgentOptions(
        setting_sources=["project"]  # Only .claude/settings.json
    ),
):
    print(message)
```

**테스트 및 CI 환경:**

```python theme={null}
# 로컬 설정을 제외하여 CI에서 일관된 동작 보장
async for message in query(
    prompt="Run tests",
    options=ClaudeAgentOptions(
        setting_sources=["project"],  # Only team-shared settings
        permission_mode="bypassPermissions",
    ),
):
    print(message)
```

**SDK 전용 애플리케이션:**

```python theme={null}
# 모든 것을 프로그래밍 방식으로 정의합니다.
# 파일 시스템 설정 소스를 거부하려면 []를 전달합니다.
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

**CLAUDE.md 프로젝트 지침 로드:**

```python theme={null}
# 프로젝트 설정을 로드하여 CLAUDE.md 파일 포함
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
  설정 우선순위
</h4>

여러 소스가 로드될 때, 설정은 이 우선순위로 병합됩니다 (높음에서 낮음):

1. 로컬 설정 (`.claude/settings.local.json`)
2. 프로젝트 설정 (`.claude/settings.json`)
3. 사용자 설정 (`~/.claude/settings.json`)

`agents` 및 `allowed_tools`와 같은 프로그래밍 방식의 옵션은 사용자, 프로젝트 및 로컬 파일 시스템 설정을 재정의합니다. 관리형 정책 설정은 프로그래밍 방식의 옵션보다 우선합니다.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

프로그래밍 방식으로 정의된 서브에이전트의 구성입니다.

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

| 필드                | 필수  | 설명                                                                                                                                         |
| :---------------- | :-- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | 예   | 이 에이전트를 사용할 시기에 대한 자연어 설명                                                                                                                  |
| `prompt`          | 예   | 에이전트의 시스템 프롬프트                                                                                                                             |
| `tools`           | 아니오 | 허용된 도구 이름의 배열. 생략하면 모든 도구를 상속합니다                                                                                                           |
| `disallowedTools` | 아니오 | 에이전트의 도구 세트에서 제거할 도구 이름의 배열. MCP 서버 수준 패턴도 허용됩니다: `mcp__server` 또는 `mcp__server__*`는 해당 서버의 모든 도구를 제거하고, `mcp__*`는 모든 서버의 모든 MCP 도구를 제거합니다 |
| `model`           | 아니오 | 이 에이전트의 모델 재정의. `"sonnet"`, `"opus"`, `"haiku"`, `"inherit"` 같은 별칭 또는 전체 모델 ID를 허용합니다. 생략하면 주 모델을 사용합니다                                    |
| `skills`          | 아니오 | 이 에이전트가 사용할 수 있는 스킬 이름 목록                                                                                                                  |
| `memory`          | 아니오 | 이 에이전트의 메모리 소스: `"user"`, `"project"`, 또는 `"local"`                                                                                        |
| `mcpServers`      | 아니오 | 이 에이전트가 사용할 수 있는 MCP 서버. 각 항목은 서버 이름 또는 인라인 `{name: config}` dict입니다                                                                       |
| `initialPrompt`   | 아니오 | 이 에이전트가 주 스레드 에이전트로 실행될 때 첫 사용자 턴으로 자동 제출됨                                                                                                 |
| `maxTurns`        | 아니오 | 에이전트가 중지되기 전의 최대 에이전트 턴 수                                                                                                                  |
| `background`      | 아니오 | 호출될 때 이 에이전트를 비차단 백그라운드 작업으로 실행합니다                                                                                                         |
| `effort`          | 아니오 | 이 에이전트의 추론 노력 수준. 명명된 수준 또는 정수를 허용합니다. [`EffortLevel`](#effortlevel) 참조                                                                    |
| `permissionMode`  | 아니오 | 이 에이전트 내의 도구 실행을 위한 권한 모드. [`PermissionMode`](#permissionmode) 참조                                                                          |

<Note>
  `AgentDefinition` 필드 이름은 `disallowedTools`, `permissionMode`, `maxTurns`와 같은 camelCase를 사용합니다. 이 이름은 TypeScript SDK와 공유되는 와이어 형식에 직접 매핑됩니다. 이는 `disallowed_tools` 및 `permission_mode`와 같은 동등한 최상위 필드에 Python snake\_case를 사용하는 `ClaudeAgentOptions`와 다릅니다. `AgentDefinition`은 dataclass이므로, snake\_case 키워드를 전달하면 구성 시 `TypeError`를 발생시킵니다.
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

도구 실행을 제어하기 위한 권한 모드입니다.

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

생각 깊이를 안내하기 위한 노력 수준입니다.

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

도구 권한 콜백 함수의 타입 별칭입니다.

```python theme={null}
CanUseTool = Callable[
    [str, dict[str, Any], ToolPermissionContext], Awaitable[PermissionResult]
]
```

콜백은 다음을 수신합니다:

* `tool_name`: 호출되는 도구의 이름
* `input_data`: 도구의 입력 매개변수
* `context`: 추가 정보가 있는 `ToolPermissionContext`

`PermissionResult` (`PermissionResultAllow` 또는 `PermissionResultDeny`)를 반환합니다.

콜백은 대화형 권한 프롬프트의 SDK 대체입니다: [권한 평가 흐름](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)이 프롬프트로 해결될 때만 호출됩니다. `allowed_tools` 항목, 설정 허용 규칙 또는 `acceptEdits` 또는 `bypassPermissions`와 같은 권한 모드로 이미 승인된 도구 호출은 이를 호출하지 않습니다. 모든 도구 호출을 제어하려면 [`PreToolUse` hook](/docs/ko/agent-sdk/hooks)을 대신 사용합니다.

`AskUserQuestion`, [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구, 및 [조직이 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)한 커넥터 도구는 허용 규칙이 일치하더라도 콜백에 도달합니다. `dontAsk` 모드에서는 대신 거부됩니다.

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

도구 권한 콜백에 전달되는 컨텍스트 정보입니다.

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

| 필드                | 타입                       | 설명                                                                                                                                            |
| :---------------- | :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`          | `Any \| None`            | 향후 중단 신호 지원을 위해 예약됨                                                                                                                           |
| `suggestions`     | `list[PermissionUpdate]` | CLI의 권한 업데이트 제안. Bash 프롬프트는 `localSettings` 대상이 있는 제안을 포함하므로, `updated_permissions`에서 반환하면 규칙을 `.claude/settings.local.json`에 쓰고 세션 간에 유지합니다. |
| `blocked_path`    | `str \| None`            | 권한 요청을 트리거한 파일 경로 (해당하는 경우). 예를 들어, Bash 명령이 허용된 디렉토리 외부의 경로에 접근하려고 할 때                                                                       |
| `decision_reason` | `str \| None`            | 이 권한 요청이 트리거된 이유. PreToolUse hook이 `"ask"`를 반환했을 때 hook의 `permissionDecisionReason`에서 전달됨                                                     |
| `title`           | `str \| None`            | 전체 권한 프롬프트 문장 (예: `Claude wants to read foo.txt`). 존재할 때 기본 프롬프트 텍스트로 사용                                                                      |
| `display_name`    | `str \| None`            | 도구 작업의 짧은 명사구 (예: `Read file`), 버튼 레이블에 적합                                                                                                    |
| `description`     | `str \| None`            | 권한 UI를 위한 인간이 읽을 수 있는 부제목                                                                                                                     |

<h3 id="permissionresult">
  `PermissionResult`
</h3>

권한 콜백 결과의 합집합 타입입니다.

```python theme={null}
PermissionResult = PermissionResultAllow | PermissionResultDeny
```

<h3 id="permissionresultallow">
  `PermissionResultAllow`
</h3>

도구 호출이 허용되어야 함을 나타내는 결과입니다.

```python theme={null}
@dataclass
class PermissionResultAllow:
    behavior: Literal["allow"] = "allow"
    updated_input: dict[str, Any] | None = None
    updated_permissions: list[PermissionUpdate] | None = None
```

| 필드                    | 타입                               | 기본값       | 설명               |
| :-------------------- | :------------------------------- | :-------- | :--------------- |
| `behavior`            | `Literal["allow"]`               | `"allow"` | "allow"이어야 합니다   |
| `updated_input`       | `dict[str, Any] \| None`         | `None`    | 원본 대신 사용할 수정된 입력 |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None`    | 적용할 권한 업데이트      |

<h3 id="permissionresultdeny">
  `PermissionResultDeny`
</h3>

도구 호출이 거부되어야 함을 나타내는 결과입니다.

```python theme={null}
@dataclass
class PermissionResultDeny:
    behavior: Literal["deny"] = "deny"
    message: str = ""
    interrupt: bool = False
```

| 필드          | 타입                | 기본값      | 설명                   |
| :---------- | :---------------- | :------- | :------------------- |
| `behavior`  | `Literal["deny"]` | `"deny"` | "deny"이어야 합니다        |
| `message`   | `str`             | `""`     | 도구가 거부된 이유를 설명하는 메시지 |
| `interrupt` | `bool`            | `False`  | 현재 실행을 중단할지 여부       |

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

프로그래밍 방식으로 권한을 업데이트하기 위한 구성입니다.

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

| 필드            | 타입                                        | 설명                     |
| :------------ | :---------------------------------------- | :--------------------- |
| `type`        | `Literal[...]`                            | 권한 업데이트 작업의 타입         |
| `rules`       | `list[PermissionRuleValue] \| None`       | 추가/교체/제거 작업을 위한 규칙     |
| `behavior`    | `Literal["allow", "deny", "ask"] \| None` | 규칙 기반 작업을 위한 동작        |
| `mode`        | `PermissionMode \| None`                  | setMode 작업을 위한 모드      |
| `directories` | `list[str] \| None`                       | 디렉토리 추가/제거 작업을 위한 디렉토리 |
| `destination` | `Literal[...] \| None`                    | 권한 업데이트를 적용할 위치        |

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

권한 업데이트에서 추가, 교체 또는 제거할 규칙입니다.

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

<h3 id="toolspreset">
  `ToolsPreset`
</h3>

Claude Code의 기본 도구 세트를 사용하기 위한 프리셋 도구 구성입니다.

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

확장된 생각 동작을 제어합니다. 세 가지 구성의 합집합입니다:

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

| 변형         | 필드                                 | 설명                          |
| :--------- | :--------------------------------- | :-------------------------- |
| `adaptive` | `type`, `display`                  | Claude가 생각할 시기를 적응적으로 결정합니다 |
| `enabled`  | `type`, `budget_tokens`, `display` | 특정 토큰 예산으로 생각 활성화           |
| `disabled` | `type`                             | 생각 비활성화                     |

선택적 `display` 필드는 생각 텍스트가 `"summarized"` 또는 `"omitted"`로 반환되는지 제어합니다. Claude Opus 4.7 이상에서 API 기본값은 `"omitted"`이므로, [`ThinkingBlock`](#thinkingblock) 출력에서 생각 콘텐츠를 받으려면 `"summarized"`를 설정합니다.

이들은 `TypedDict` 클래스이므로 런타임에 일반 dict입니다. dict 리터럴로 구성하거나 클래스를 생성자처럼 호출합니다. 둘 다 `dict`를 생성합니다. `config.budget_tokens`가 아닌 `config["budget_tokens"]`로 필드에 접근합니다:

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

SDK 베타 기능의 리터럴 타입입니다.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

`ClaudeAgentOptions`의 `betas` 필드와 함께 사용하여 베타 기능을 활성화합니다.

<Warning>
  `context-1m-2025-08-07` 베타는 2026년 4월 30일부터 폐기되었습니다. Claude Sonnet 4.5 또는 Sonnet 4와 함께 이 헤더를 전달하면 효과가 없으며, 표준 200k 토큰 컨텍스트 윈도우를 초과하는 요청은 오류를 반환합니다. 1M 토큰 컨텍스트 윈도우를 사용하려면 [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 또는 Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview)로 마이그레이션하십시오. 이들은 베타 헤더 없이 표준 가격으로 1M 컨텍스트를 포함합니다.
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

`create_sdk_mcp_server()`로 생성된 SDK MCP 서버의 구성입니다.

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

MCP 서버 구성의 합집합 타입입니다.

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

[`get_mcp_status()`](#methods)에서 보고한 MCP 서버의 구성입니다. 이것은 모든 [`McpServerConfig`](#mcpserverconfig) 전송 변형의 합집합에 claude.ai를 통해 프록시된 서버를 위한 출력 전용 `claudeai-proxy` 변형을 더한 것입니다.

```python theme={null}
McpServerStatusConfig = (
    McpStdioServerConfig
    | McpSSEServerConfig
    | McpHttpServerConfig
    | McpSdkServerConfigStatus
    | McpClaudeAIProxyServerConfig
)
```

`McpSdkServerConfigStatus`는 [`McpSdkServerConfig`](#mcpsdkserverconfig)의 직렬화 가능한 형식이며 `type` (`"sdk"`) 및 `name` (`str`) 필드만 있습니다. 인프로세스 `instance`는 생략됩니다. `McpClaudeAIProxyServerConfig`는 `type` (`"claudeai-proxy"`), `url` (`str`) 및 `id` (`str`) 필드를 가집니다.

<h3 id="mcpstatusresponse">
  `McpStatusResponse`
</h3>

[`ClaudeSDKClient.get_mcp_status()`](#methods)의 응답입니다. 서버 상태 목록을 `mcpServers` 키 아래에 래핑합니다.

```python theme={null}
class McpStatusResponse(TypedDict):
    mcpServers: list[McpServerStatus]
```

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

[`McpStatusResponse`](#mcpstatusresponse)에 포함된 연결된 MCP 서버의 상태입니다.

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

| 필드           | 타입                                                       | 설명                                                                                                                               |
| :----------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `name`       | `str`                                                    | 서버 이름                                                                                                                            |
| `status`     | `str`                                                    | `"connected"`, `"failed"`, `"needs-auth"`, `"pending"` 또는 `"disabled"` 중 하나                                                      |
| `serverInfo` | `dict` (선택사항)                                            | 서버 이름 및 버전 (`{"name": str, "version": str}`)                                                                                     |
| `error`      | `str` (선택사항)                                             | 서버가 연결에 실패한 경우 오류 메시지                                                                                                            |
| `config`     | [`McpServerStatusConfig`](#mcpserverstatusconfig) (선택사항) | 서버 구성. [`McpServerConfig`](#mcpserverconfig)와 동일한 형태 (stdio, SSE, HTTP 또는 SDK), 더하기 claude.ai를 통해 연결된 서버를 위한 `claudeai-proxy` 변형 |
| `scope`      | `str` (선택사항)                                             | 구성 범위                                                                                                                            |
| `tools`      | `list` (선택사항)                                            | 이 서버가 제공하는 도구, 각각 `name`, `description` 및 `annotations` 필드 포함                                                                    |

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

SDK에서 플러그인을 로드하기 위한 구성입니다.

```python theme={null}
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| 필드     | 타입                 | 설명                                 |
| :----- | :----------------- | :--------------------------------- |
| `type` | `Literal["local"]` | `"local"`이어야 합니다 (현재 로컬 플러그인만 지원됨) |
| `path` | `str`              | 플러그인 디렉토리의 절대 또는 상대 경로             |

**예제:**

```python theme={null}
plugins = [
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"},
]
```

플러그인 생성 및 사용에 대한 완전한 정보는 [플러그인](/docs/ko/agent-sdk/plugins)을 참조하십시오.

<h2 id="message-types">
  메시지 타입
</h2>

<h3 id="message">
  `Message`
</h3>

모든 가능한 메시지의 합집합 타입입니다.

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

사용자 입력 메시지입니다.

```python theme={null}
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
    uuid: str | None = None
    parent_tool_use_id: str | None = None
    tool_use_result: dict[str, Any] | None = None
```

| 필드                   | 타입                          | 설명                           |
| :------------------- | :-------------------------- | :--------------------------- |
| `content`            | `str \| list[ContentBlock]` | 텍스트 또는 콘텐츠 블록으로서의 메시지 콘텐츠    |
| `uuid`               | `str \| None`               | 고유 메시지 식별자                   |
| `parent_tool_use_id` | `str \| None`               | 이 메시지가 도구 결과 응답인 경우 도구 사용 ID |
| `tool_use_result`    | `dict[str, Any] \| None`    | 해당하는 경우 도구 결과 데이터            |

<h3 id="assistantmessage">
  `AssistantMessage`
</h3>

콘텐츠 블록이 있는 어시스턴트 응답 메시지입니다.

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

| 필드                   | 타입                                                           | 설명                                                           |
| :------------------- | :----------------------------------------------------------- | :----------------------------------------------------------- |
| `content`            | `list[ContentBlock]`                                         | 응답의 콘텐츠 블록 목록                                                |
| `model`              | `str`                                                        | 응답을 생성한 모델                                                   |
| `parent_tool_use_id` | `str \| None`                                                | 이것이 중첩된 응답인 경우 도구 사용 ID                                      |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | 응답이 오류를 만난 경우 오류 타입                                          |
| `usage`              | `dict[str, Any] \| None`                                     | 메시지별 토큰 사용량 ([`ResultMessage.usage`](#resultmessage)와 동일한 키) |
| `message_id`         | `str \| None`                                                | API 메시지 ID. 한 턴의 여러 메시지는 동일한 ID를 공유합니다                       |

<h3 id="assistantmessageerror">
  `AssistantMessageError`
</h3>

어시스턴트 메시지의 가능한 오류 타입입니다.

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

메타데이터가 있는 시스템 메시지입니다.

```python theme={null}
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

<h3 id="resultmessage">
  `ResultMessage`
</h3>

비용 및 사용량 정보가 있는 최종 결과 메시지입니다.

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

`subtype` 필드는 다른 필드가 채워지는지 결정합니다. 이는 `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"` 또는 `"error_max_structured_output_retries"` 중 하나입니다. Python 데이터클래스는 모든 변형을 하나의 형태로 평탄화하므로 반환된 서브타입에 적용되지 않는 필드는 `None`입니다.

대화가 오류로 끝날 때 여러 필드가 진단 세부 정보를 전달합니다:

* `is_error`: 대화가 오류 상태로 끝났을 때 `True`입니다. 항상 `error_*` 서브타입에서 `True`입니다. `subtype="success"`에서는 최종 모델 요청이 실패했을 때 `True`입니다. 즉, 에이전트 루프가 완료되었지만 마지막 API 호출이 오류를 반환했습니다.
* `api_error_status`: 종료 API 오류의 HTTP 상태 코드입니다. 턴이 오류 없이 끝났을 때 `None`입니다. `subtype="success"`에서만 채워집니다.
* `result`: `subtype="success"`에서 최종 어시스턴트 메시지의 텍스트이거나, `error_*` 서브타입에서 `None`입니다. `subtype="success"`이고 `is_error=True`일 때, 이는 사용 가능한 경우 API 오류 문자열을 보유하지만 비어 있을 수 있으므로 `api_error_status`와 이전 `AssistantMessage` 콘텐츠를 확인하십시오.
* `errors`: 최대 턴 메시지와 같은 루프 수준 오류 문자열입니다. `error_*` 서브타입에서만 채워집니다.

`usage` dict는 존재할 때 다음 키를 포함합니다:

| 키                             | 타입    | 설명                                                                                                                                                 |
| ----------------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | 최상위 에이전트 루프에서 소비된 입력 토큰입니다. [서브에이전트 토큰은 포함되지 않습니다](/docs/ko/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); 전체 트리 회계의 경우 `model_usage`를 사용하십시오. |
| `output_tokens`               | `int` | 최상위 에이전트 루프에서 생성된 출력 토큰입니다. 서브에이전트 토큰은 포함되지 않습니다.                                                                                                  |
| `cache_creation_input_tokens` | `int` | 새 캐시 항목을 생성하는 데 사용된 토큰입니다.                                                                                                                         |
| `cache_read_input_tokens`     | `int` | 기존 캐시 항목에서 읽은 토큰입니다.                                                                                                                               |

`model_usage` dict는 모델 이름을 모델별 사용량에 매핑합니다. 내부 dict 키는 camelCase를 사용합니다. 기본 CLI 프로세스에서 수정되지 않은 상태로 전달되므로 TypeScript [`ModelUsage`](/docs/ko/agent-sdk/typescript#modelusage) 타입과 일치합니다:

| 키                          | 타입      | 설명                                                                                             |
| -------------------------- | ------- | ---------------------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | 이 모델의 입력 토큰입니다.                                                                                |
| `outputTokens`             | `int`   | 이 모델의 출력 토큰입니다.                                                                                |
| `cacheReadInputTokens`     | `int`   | 이 모델의 캐시 읽기 토큰입니다.                                                                             |
| `cacheCreationInputTokens` | `int`   | 이 모델의 캐시 생성 토큰입니다.                                                                             |
| `webSearchRequests`        | `int`   | 이 모델이 수행한 웹 검색 요청입니다.                                                                          |
| `costUSD`                  | `float` | 이 모델의 추정 비용 (USD), 클라이언트 측 계산입니다. 청구 주의 사항은 [비용 및 사용량 추적](/docs/ko/agent-sdk/cost-tracking) 참조하십시오. |
| `contextWindow`            | `int`   | 이 모델의 컨텍스트 윈도우 크기입니다.                                                                          |
| `maxOutputTokens`          | `int`   | 이 모델의 최대 출력 토큰 제한입니다.                                                                          |

<h3 id="streamevent">
  `StreamEvent`
</h3>

스트리밍 중 부분 메시지 업데이트를 위한 스트림 이벤트입니다. `ClaudeAgentOptions`에서 `include_partial_messages=True`일 때만 수신됩니다. `from claude_agent_sdk.types import StreamEvent`를 통해 가져옵니다.

```python theme={null}
@dataclass
class StreamEvent:
    uuid: str
    session_id: str
    event: dict[str, Any]  # The raw Claude API stream event
    parent_tool_use_id: str | None = None
```

| 필드                   | 타입               | 설명                                                                                                              |
| :------------------- | :--------------- | :-------------------------------------------------------------------------------------------------------------- |
| `uuid`               | `str`            | 이 이벤트의 고유 식별자입니다                                                                                                |
| `session_id`         | `str`            | 세션 식별자입니다                                                                                                       |
| `event`              | `dict[str, Any]` | 원본 Claude API 스트림 이벤트 데이터입니다                                                                                    |
| `parent_tool_use_id` | `str \| None`    | 항상 `None`입니다. 스트림 이벤트는 주 세션에서만 발생합니다. 서브에이전트 속성의 경우 [`AssistantMessage`](#assistantmessage)와 같은 완전한 메시지를 사용하십시오 |

<h3 id="ratelimitevent">
  `RateLimitEvent`
</h3>

속도 제한 상태가 변경될 때 발생합니다 (예: `"allowed"`에서 `"allowed_warning"`으로). 이를 사용하여 사용자에게 하드 제한에 도달하기 전에 경고하거나, 상태가 `"rejected"`일 때 백오프합니다.

```python theme={null}
@dataclass
class RateLimitEvent:
    rate_limit_info: RateLimitInfo
    uuid: str
    session_id: str
```

| 필드                | 타입                                | 설명             |
| :---------------- | :-------------------------------- | :------------- |
| `rate_limit_info` | [`RateLimitInfo`](#ratelimitinfo) | 현재 속도 제한 상태입니다 |
| `uuid`            | `str`                             | 고유 이벤트 식별자입니다  |
| `session_id`      | `str`                             | 세션 식별자입니다      |

<h3 id="ratelimitinfo">
  `RateLimitInfo`
</h3>

[`RateLimitEvent`](#ratelimitevent)에 의해 전달되는 속도 제한 상태입니다.

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

| 필드                        | 타입                        | 설명                                                                            |
| :------------------------ | :------------------------ | :---------------------------------------------------------------------------- |
| `status`                  | `RateLimitStatus`         | 현재 상태입니다. `"allowed_warning"`은 제한에 접근 중을 의미합니다. `"rejected"`는 제한에 도달했음을 의미합니다 |
| `resets_at`               | `int \| None`             | 속도 제한 윈도우가 재설정되는 Unix 타임스탬프입니다                                                |
| `rate_limit_type`         | `RateLimitType \| None`   | 어느 속도 제한 윈도우가 적용되는지입니다                                                        |
| `utilization`             | `float \| None`           | 소비된 속도 제한의 분수입니다 (0.0 \~ 1.0)                                                 |
| `overage_status`          | `RateLimitStatus \| None` | 해당하는 경우 종량제 초과 사용 상태입니다                                                       |
| `overage_resets_at`       | `int \| None`             | 초과 사용 윈도우가 재설정되는 Unix 타임스탬프입니다                                                |
| `overage_disabled_reason` | `str \| None`             | 상태가 `"rejected"`인 경우 초과 사용을 사용할 수 없는 이유입니다                                    |
| `raw`                     | `dict[str, Any]`          | 위에서 모델링되지 않은 필드를 포함한 CLI의 전체 원본 dict입니다                                       |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

백그라운드 작업이 시작될 때 발생합니다. 백그라운드 작업은 주 턴 외부에서 추적되는 모든 것입니다: 백그라운드 Bash 명령, [Monitor](#monitor) 감시, Agent 도구를 통해 생성된 서브에이전트 또는 원격 에이전트입니다. `task_type` 필드가 어느 것인지 알려줍니다. 이 명명은 `Task`-to-`Agent` 도구 이름 변경과 무관합니다.

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

| 필드            | 타입            | 설명                                                                                               |
| :------------ | :------------ | :----------------------------------------------------------------------------------------------- |
| `task_id`     | `str`         | 작업의 고유 식별자입니다                                                                                    |
| `description` | `str`         | 작업의 설명입니다                                                                                        |
| `uuid`        | `str`         | 고유 메시지 식별자입니다                                                                                    |
| `session_id`  | `str`         | 세션 식별자입니다                                                                                        |
| `tool_use_id` | `str \| None` | 관련 도구 사용 ID입니다                                                                                   |
| `task_type`   | `str \| None` | 백그라운드 작업의 종류입니다: 백그라운드 Bash 및 Monitor 감시의 경우 `"local_bash"`, `"local_agent"` 또는 `"remote_agent"` |

<h3 id="taskusage">
  `TaskUsage`
</h3>

백그라운드 작업의 토큰 및 타이밍 데이터입니다.

```python theme={null}
class TaskUsage(TypedDict):
    total_tokens: int
    tool_uses: int
    duration_ms: int
```

<h3 id="taskprogressmessage">
  `TaskProgressMessage`
</h3>

실행 중인 백그라운드 작업에 대한 진행 상황 업데이트로 주기적으로 발생합니다.

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

| 필드               | 타입            | 설명                      |
| :--------------- | :------------ | :---------------------- |
| `task_id`        | `str`         | 작업의 고유 식별자입니다           |
| `description`    | `str`         | 현재 상태 설명입니다             |
| `usage`          | `TaskUsage`   | 지금까지 이 작업의 토큰 사용량입니다    |
| `uuid`           | `str`         | 고유 메시지 식별자입니다           |
| `session_id`     | `str`         | 세션 식별자입니다               |
| `tool_use_id`    | `str \| None` | 관련 도구 사용 ID입니다          |
| `last_tool_name` | `str \| None` | 작업이 마지막으로 사용한 도구의 이름입니다 |

<h3 id="tasknotificationmessage">
  `TaskNotificationMessage`
</h3>

백그라운드 작업이 완료, 실패 또는 중지될 때 발생합니다. 백그라운드 작업에는 `run_in_background` Bash 명령, Monitor 감시 및 백그라운드 서브에이전트가 포함됩니다.

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

| 필드            | 타입                       | 설명                                               |
| :------------ | :----------------------- | :----------------------------------------------- |
| `task_id`     | `str`                    | 작업의 고유 식별자입니다                                    |
| `status`      | `TaskNotificationStatus` | `"completed"`, `"failed"` 또는 `"stopped"` 중 하나입니다 |
| `output_file` | `str`                    | 작업 출력 파일의 경로입니다                                  |
| `summary`     | `str`                    | 작업 결과의 요약입니다                                     |
| `uuid`        | `str`                    | 고유 메시지 식별자입니다                                    |
| `session_id`  | `str`                    | 세션 식별자입니다                                        |
| `tool_use_id` | `str \| None`            | 관련 도구 사용 ID입니다                                   |
| `usage`       | `TaskUsage \| None`      | 작업의 최종 토큰 사용량입니다                                 |

<h2 id="content-block-types">
  콘텐츠 블록 타입
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

모든 콘텐츠 블록의 합집합 타입입니다.

```python theme={null}
ContentBlock = TextBlock | ThinkingBlock | ToolUseBlock | ToolResultBlock
```

<h3 id="textblock">
  `TextBlock`
</h3>

텍스트 콘텐츠 블록입니다.

```python theme={null}
@dataclass
class TextBlock:
    text: str
```

<h3 id="thinkingblock">
  `ThinkingBlock`
</h3>

생각 콘텐츠 블록입니다 (생각 기능이 있는 모델용).

```python theme={null}
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

<h3 id="tooluseblock">
  `ToolUseBlock`
</h3>

도구 사용 요청 블록입니다.

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

도구 실행 결과 블록입니다.

```python theme={null}
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

<h2 id="error-types">
  오류 타입
</h2>

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

모든 SDK 오류의 기본 예외 클래스입니다.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

<h3 id="clinotfounderror">
  `CLINotFoundError`
</h3>

Claude Code CLI가 설치되지 않았거나 찾을 수 없을 때 발생합니다.

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

Claude Code 연결이 실패할 때 발생합니다.

```python theme={null}
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

<h3 id="processerror">
  `ProcessError`
</h3>

Claude Code 프로세스가 실패할 때 발생합니다.

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

JSON 구문 분석이 실패할 때 발생합니다.

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
  Hook 타입
</h2>

hooks 사용에 대한 포괄적인 가이드, 예제 및 일반적인 패턴은 [Hooks 가이드](/docs/ko/agent-sdk/hooks)를 참조하십시오.

<h3 id="hookevent">
  `HookEvent`
</h3>

지원되는 hook 이벤트 타입입니다.

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
  TypeScript SDK는 Python에서 아직 사용할 수 없는 추가 hook 이벤트를 지원합니다: `SessionStart`, `SessionEnd`, `Setup`, `TeammateIdle`, `TaskCompleted`, `ConfigChange`, `WorktreeCreate`, `WorktreeRemove`, `PostToolBatch` 및 `MessageDisplay`.
</Note>

<h3 id="hookcallback">
  `HookCallback`
</h3>

hook 콜백 함수의 타입 정의입니다.

```python theme={null}
HookCallback = Callable[[HookInput, str | None, HookContext], Awaitable[HookJSONOutput]]
```

매개변수:

* `input`: `hook_event_name`을 기반으로 한 판별된 합집합이 있는 강타입 hook 입력 ([`HookInput`](#hookinput) 참조)
* `tool_use_id`: 선택적 도구 사용 식별자 (도구 관련 hooks의 경우)
* `context`: 추가 정보가 있는 hook 컨텍스트

다음을 포함할 수 있는 [`HookJSONOutput`](#hookjsonoutput)을 반환합니다.

* `decision`: 작업을 차단하려면 `"block"`
* `systemMessage`: 사용자에게 표시되는 경고 메시지
* `hookSpecificOutput`: hook 특정 출력 데이터

<h3 id="hookcontext">
  `HookContext`
</h3>

hook 콜백에 전달되는 컨텍스트 정보입니다.

```python theme={null}
class HookContext(TypedDict):
    signal: Any | None  # Future: abort signal support
```

<h3 id="hookmatcher">
  `HookMatcher`
</h3>

특정 이벤트 또는 도구에 hooks를 일치시키기 위한 구성입니다.

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

모든 hook 입력 타입의 합집합 타입입니다. 실제 타입은 `hook_event_name` 필드에 따라 달라집니다.

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

모든 hook 입력 타입에 존재하는 기본 필드입니다.

```python theme={null}
class BaseHookInput(TypedDict):
    session_id: str
    transcript_path: str
    cwd: str
    permission_mode: NotRequired[str]
```

| 필드                | 타입           | 설명           |
| :---------------- | :----------- | :----------- |
| `session_id`      | `str`        | 현재 세션 식별자    |
| `transcript_path` | `str`        | 세션 기록 파일의 경로 |
| `cwd`             | `str`        | 현재 작업 디렉토리   |
| `permission_mode` | `str` (선택사항) | 현재 권한 모드     |

<h3 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h3>

`PreToolUse` hook 이벤트의 입력 데이터입니다.

```python theme={null}
class PreToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PreToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| 필드                | 타입                      | 설명                                    |
| :---------------- | :---------------------- | :------------------------------------ |
| `hook_event_name` | `Literal["PreToolUse"]` | 항상 "PreToolUse"                       |
| `tool_name`       | `str`                   | 실행될 도구의 이름                            |
| `tool_input`      | `dict[str, Any]`        | 도구의 입력 매개변수                           |
| `tool_use_id`     | `str`                   | 이 도구 사용의 고유 식별자                       |
| `agent_id`        | `str` (선택사항)            | 서브에이전트 식별자, hook이 서브에이전트 내에서 발생할 때 존재 |
| `agent_type`      | `str` (선택사항)            | 서브에이전트 타입, hook이 서브에이전트 내에서 발생할 때 존재  |

<h3 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h3>

`PostToolUse` hook 이벤트의 입력 데이터입니다.

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

| 필드                | 타입                       | 설명                                    |
| :---------------- | :----------------------- | :------------------------------------ |
| `hook_event_name` | `Literal["PostToolUse"]` | 항상 "PostToolUse"                      |
| `tool_name`       | `str`                    | 실행된 도구의 이름                            |
| `tool_input`      | `dict[str, Any]`         | 사용된 입력 매개변수                           |
| `tool_response`   | `Any`                    | 도구 실행의 응답                             |
| `tool_use_id`     | `str`                    | 이 도구 사용의 고유 식별자                       |
| `agent_id`        | `str` (선택사항)             | 서브에이전트 식별자, hook이 서브에이전트 내에서 발생할 때 존재 |
| `agent_type`      | `str` (선택사항)             | 서브에이전트 타입, hook이 서브에이전트 내에서 발생할 때 존재  |

<h3 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h3>

`PostToolUseFailure` hook 이벤트의 입력 데이터입니다. 도구 실행이 실패할 때 호출됩니다.

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

| 필드                | 타입                              | 설명                                    |
| :---------------- | :------------------------------ | :------------------------------------ |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | 항상 "PostToolUseFailure"               |
| `tool_name`       | `str`                           | 실패한 도구의 이름                            |
| `tool_input`      | `dict[str, Any]`                | 사용된 입력 매개변수                           |
| `tool_use_id`     | `str`                           | 이 도구 사용의 고유 식별자                       |
| `error`           | `str`                           | 실패한 실행의 오류 메시지                        |
| `is_interrupt`    | `bool` (선택사항)                   | 실패가 중단으로 인한 것인지 여부                    |
| `agent_id`        | `str` (선택사항)                    | 서브에이전트 식별자, hook이 서브에이전트 내에서 발생할 때 존재 |
| `agent_type`      | `str` (선택사항)                    | 서브에이전트 타입, hook이 서브에이전트 내에서 발생할 때 존재  |

<h3 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h3>

`UserPromptSubmit` hook 이벤트의 입력 데이터입니다.

```python theme={null}
class UserPromptSubmitHookInput(BaseHookInput):
    hook_event_name: Literal["UserPromptSubmit"]
    prompt: str
```

| 필드                | 타입                            | 설명                    |
| :---------------- | :---------------------------- | :-------------------- |
| `hook_event_name` | `Literal["UserPromptSubmit"]` | 항상 "UserPromptSubmit" |
| `prompt`          | `str`                         | 사용자가 제출한 프롬프트         |

<h3 id="stophookinput">
  `StopHookInput`
</h3>

`Stop` hook 이벤트의 입력 데이터입니다.

```python theme={null}
class StopHookInput(BaseHookInput):
    hook_event_name: Literal["Stop"]
    stop_hook_active: bool
```

| 필드                 | 타입                | 설명                      |
| :----------------- | :---------------- | :---------------------- |
| `hook_event_name`  | `Literal["Stop"]` | 항상 "Stop"               |
| `stop_hook_active` | `bool`            | stop hook이 활성화되어 있는지 여부 |

<h3 id="subagentstophookinput">
  `SubagentStopHookInput`
</h3>

`SubagentStop` hook 이벤트의 입력 데이터입니다.

```python theme={null}
class SubagentStopHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStop"]
    stop_hook_active: bool
    agent_id: str
    agent_transcript_path: str
    agent_type: str
```

| 필드                      | 타입                        | 설명                      |
| :---------------------- | :------------------------ | :---------------------- |
| `hook_event_name`       | `Literal["SubagentStop"]` | 항상 "SubagentStop"       |
| `stop_hook_active`      | `bool`                    | stop hook이 활성화되어 있는지 여부 |
| `agent_id`              | `str`                     | 서브에이전트의 고유 식별자          |
| `agent_transcript_path` | `str`                     | 서브에이전트의 기록 파일 경로        |
| `agent_type`            | `str`                     | 서브에이전트의 타입              |

<h3 id="precompacthookinput">
  `PreCompactHookInput`
</h3>

`PreCompact` hook 이벤트의 입력 데이터입니다.

```python theme={null}
class PreCompactHookInput(BaseHookInput):
    hook_event_name: Literal["PreCompact"]
    trigger: Literal["manual", "auto"]
    custom_instructions: str | None
```

| 필드                    | 타입                          | 설명               |
| :-------------------- | :-------------------------- | :--------------- |
| `hook_event_name`     | `Literal["PreCompact"]`     | 항상 "PreCompact"  |
| `trigger`             | `Literal["manual", "auto"]` | 압축을 트리거한 것       |
| `custom_instructions` | `str \| None`               | 압축을 위한 사용자 정의 지침 |

<h3 id="notificationhookinput">
  `NotificationHookInput`
</h3>

`Notification` hook 이벤트의 입력 데이터입니다.

```python theme={null}
class NotificationHookInput(BaseHookInput):
    hook_event_name: Literal["Notification"]
    message: str
    title: NotRequired[str]
    notification_type: str
```

| 필드                  | 타입                        | 설명                |
| :------------------ | :------------------------ | :---------------- |
| `hook_event_name`   | `Literal["Notification"]` | 항상 "Notification" |
| `message`           | `str`                     | 알림 메시지 콘텐츠        |
| `title`             | `str` (선택사항)              | 알림 제목             |
| `notification_type` | `str`                     | 알림의 타입            |

<h3 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h3>

`SubagentStart` hook 이벤트의 입력 데이터입니다.

```python theme={null}
class SubagentStartHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStart"]
    agent_id: str
    agent_type: str
```

| 필드                | 타입                         | 설명                 |
| :---------------- | :------------------------- | :----------------- |
| `hook_event_name` | `Literal["SubagentStart"]` | 항상 "SubagentStart" |
| `agent_id`        | `str`                      | 서브에이전트의 고유 식별자     |
| `agent_type`      | `str`                      | 서브에이전트의 타입         |

<h3 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h3>

`PermissionRequest` hook 이벤트의 입력 데이터입니다. hooks가 프로그래밍 방식으로 권한 결정을 처리할 수 있습니다.

```python theme={null}
class PermissionRequestHookInput(BaseHookInput):
    hook_event_name: Literal["PermissionRequest"]
    tool_name: str
    tool_input: dict[str, Any]
    permission_suggestions: NotRequired[list[Any]]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| 필드                       | 타입                             | 설명                                    |
| :----------------------- | :----------------------------- | :------------------------------------ |
| `hook_event_name`        | `Literal["PermissionRequest"]` | 항상 "PermissionRequest"                |
| `tool_name`              | `str`                          | 권한을 요청하는 도구의 이름                       |
| `tool_input`             | `dict[str, Any]`               | 도구의 입력 매개변수                           |
| `permission_suggestions` | `list[Any]` (선택사항)             | CLI의 제안된 권한 업데이트                      |
| `agent_id`               | `str` (선택사항)                   | 서브에이전트 식별자, hook이 서브에이전트 내에서 발생할 때 존재 |
| `agent_type`             | `str` (선택사항)                   | 서브에이전트 타입, hook이 서브에이전트 내에서 발생할 때 존재  |

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

hook 콜백 반환 값의 합집합 타입입니다.

```python theme={null}
HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

제어 및 결정 필드가 있는 동기식 hook 출력입니다.

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
  Python 코드에서 `continue_` (언더스코어 포함)를 사용합니다. CLI로 전송할 때 자동으로 `continue`로 변환됩니다.
</Note>

<h4 id="hookspecificoutput">
  `HookSpecificOutput`
</h4>

hook 이벤트 이름과 이벤트 특정 필드를 포함하는 `TypedDict`입니다. 형태는 `hookEventName` 값에 따라 달라집니다. hook 이벤트별 사용 가능한 필드에 대한 전체 세부 정보는 [hooks로 실행 제어](/docs/ko/agent-sdk/hooks#outputs)를 참조하십시오.

이벤트 특정 출력 타입의 판별된 합집합입니다. `hookEventName` 필드가 어느 필드가 유효한지 결정합니다.

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

hook 실행을 연기하는 비동기 hook 출력입니다.

```python theme={null}
class AsyncHookJSONOutput(TypedDict):
    async_: Literal[True]  # Set to True to defer execution
    asyncTimeout: NotRequired[int]  # Timeout in milliseconds
```

<Note>
  Python 코드에서 `async_` (언더스코어 포함)를 사용합니다. CLI로 전송할 때 자동으로 `async`로 변환됩니다.
</Note>

<h3 id="hook-usage-example">
  Hook 사용 예제
</h3>

이 예제는 두 개의 hooks를 등록합니다: `rm -rf /`와 같은 위험한 bash 명령을 차단하는 하나, 감사를 위해 모든 도구 사용을 기록하는 다른 하나. 보안 hook은 `matcher`를 통해 Bash 명령에서만 실행되고, 로깅 hook은 모든 도구에서 실행됩니다.

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
  도구 입력/출력 타입
</h2>

모든 기본 Claude Code 도구의 입력/출력 스키마 문서입니다. Python SDK는 이들을 타입으로 내보내지 않지만, 메시지의 도구 입력 및 출력 구조를 나타냅니다.

<h3 id="agent">
  Agent
</h3>

**도구 이름:** `Agent` (이전 `Task`, 여전히 별칭으로 허용됨)

**입력:**

```python theme={null}
{
    "description": str,  # 작업의 짧은 설명 (3-5단어)
    "prompt": str,  # 에이전트가 수행할 작업
    "subagent_type": str,  # 사용할 특화된 에이전트의 유형
}
```

**출력:**

```python theme={null}
{
    "result": str,  # 서브에이전트의 최종 결과
    "usage": dict | None,  # 토큰 사용 통계
    "total_cost_usd": float | None,  # USD로 예상되는 총 비용
    "duration_ms": int | None,  # 실행 시간(밀리초)
}
```

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**도구 이름:** `AskUserQuestion`

실행 중에 사용자에게 명확히 하는 질문을 합니다. 사용 세부 정보는 [승인 및 사용자 입력 처리](/docs/ko/agent-sdk/user-input#handle-clarifying-questions)를 참조하십시오.

**입력:**

```python theme={null}
{
    "questions": [  # 사용자에게 물어볼 질문 (1-4개 질문)
        {
            "question": str,  # 사용자에게 물어볼 완전한 질문
            "header": str,  # 칩/태그로 표시되는 매우 짧은 레이블 (최대 12자)
            "options": [  # 사용 가능한 선택 (2-4개 옵션)
                {
                    "label": str,  # 이 옵션의 표시 텍스트 (1-5단어)
                    "description": str,  # 이 옵션이 의미하는 바에 대한 설명
                }
            ],
            "multiSelect": bool,  # 여러 선택을 허용하려면 true로 설정
        }
    ],
    "answers": dict[str, str | list[str]] | None,
    # 권한 시스템에 의해 채워진 사용자 답변. 다중 선택
    # 답변은 레이블 목록 또는 쉼표로 연결된 문자열일 수 있습니다
}
```

**출력:**

```python theme={null}
{
    "questions": [  # 물어본 질문
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # 질문 텍스트를 답변 문자열에 매핑
    # 다중 선택 답변은 쉼표로 구분됨
}
```

<h3 id="bash">
  Bash
</h3>

**도구 이름:** `Bash`

**입력:**

```python theme={null}
{
    "command": str,  # 실행할 명령
    "timeout": int | None,  # 선택적 시간 초과(밀리초 단위, 최대 600000; 더 높은 값은 최대값으로 고정됨)
    "description": str | None,  # 명확하고 간결한 설명 (5-10단어)
    "run_in_background": bool | None,  # 백그라운드에서 실행하려면 true로 설정
}
```

**출력:**

```python theme={null}
{
    "output": str,  # 결합된 stdout 및 stderr 출력
    "exitCode": int,  # 명령의 종료 코드
    "killed": bool | None,  # 시간 초과로 인해 명령이 종료되었는지 여부
    "shellId": str | None,  # 백그라운드 프로세스의 셸 ID
}
```

<h3 id="monitor">
  Monitor
</h3>

**도구 이름:** `Monitor`

백그라운드 소스를 실행하고 각 이벤트를 Claude에 전달하여 폴링 없이 반응할 수 있도록 합니다. `command`는 스크립트를 실행하고 stdout 줄당 하나의 이벤트를 내보내며, `ws`는 WebSocket을 열고 텍스트 프레임당 하나의 이벤트를 내보냅니다. `command` 또는 `ws` 중 정확히 하나를 제공하십시오.

Monitor가 명령을 실행할 때, Bash와 동일한 권한 규칙을 따릅니다. WebSocket 감시는 별도로 승인을 요청합니다. `ws` 소스는 Claude Code v2.1.195 이상이 필요합니다. 동작 및 제공자 가용성은 [Monitor 도구 참조](/docs/ko/tools-reference#monitor-tool)를 참조하십시오.

**입력:**

```python theme={null}
{
    "command": str | None,  # 셸 스크립트; 각 stdout 줄은 이벤트이고, 종료는 감시를 끝냅니다
    "ws": dict | None,  # WebSocket 소스: {"url": str, "protocols": list[str] | None}; 각 텍스트 프레임은 이벤트입니다
    "description": str,  # 알림에 표시되는 짧은 설명
    "timeout_ms": int | None,  # 이 기한 후 종료 (기본값 300000, 최대 3600000)
    "persistent": bool | None,  # 세션의 수명 동안 실행; TaskStop으로 중지
}
```

**출력:**

```python theme={null}
{
    "taskId": str,  # 백그라운드 모니터 작업의 ID
    "timeoutMs": int,  # 밀리초 단위의 시간 초과 기한 (지속적일 때 0)
    "persistent": bool | None,  # TaskStop 또는 세션 종료까지 실행 중일 때 True
}
```

<h3 id="edit">
  Edit
</h3>

**도구 이름:** `Edit`

**입력:**

```python theme={null}
{
    "file_path": str,  # 수정할 파일의 절대 경로
    "old_string": str,  # 바꿀 텍스트
    "new_string": str,  # 바꿀 텍스트
    "replace_all": bool | None,  # 모든 항목 바꾸기 (기본값 False)
}
```

**출력:**

```python theme={null}
{
    "message": str,  # 확인 메시지
    "replacements": int,  # 수행된 바꾸기 수
    "file_path": str,  # 편집된 파일 경로
}
```

<h3 id="read">
  Read
</h3>

**도구 이름:** `Read`

**입력:**

```python theme={null}
{
    "file_path": str,  # 읽을 파일의 절대 경로
    "offset": int | None,  # 읽기를 시작할 줄 번호
    "limit": int | None,  # 읽을 줄 수
}
```

**출력 (텍스트 파일):**

```python theme={null}
{
    "content": str,  # 줄 번호가 있는 파일 내용
    "total_lines": int,  # 파일의 총 줄 수
    "lines_returned": int,  # 실제로 반환된 줄
}
```

**출력 (이미지):**

```python theme={null}
{
    "image": str,  # Base64로 인코딩된 이미지 데이터
    "mime_type": str,  # 이미지 MIME 타입
    "file_size": int,  # 파일 크기(바이트)
}
```

<h3 id="write">
  Write
</h3>

**도구 이름:** `Write`

**입력:**

```python theme={null}
{
    "file_path": str,  # 쓸 파일의 절대 경로
    "content": str,  # 파일에 쓸 내용
}
```

**출력:**

```python theme={null}
{
    "message": str,  # 성공 메시지
    "bytes_written": int,  # 쓴 바이트 수
    "file_path": str,  # 쓴 파일 경로
}
```

<h3 id="glob">
  Glob
</h3>

**도구 이름:** `Glob`

**입력:**

```python theme={null}
{
    "pattern": str,  # 파일과 일치시킬 glob 패턴
    "path": str | None,  # 검색할 디렉토리 (기본값은 cwd)
}
```

**출력:**

```python theme={null}
{
    "matches": list[str],  # 일치하는 파일 경로 배열
    "count": int,  # 찾은 일치 수
    "search_path": str,  # 사용된 검색 디렉토리
}
```

<h3 id="grep">
  Grep
</h3>

**도구 이름:** `Grep`

**입력:**

```python theme={null}
{
    "pattern": str,  # 정규 표현식 패턴
    "path": str | None,  # 검색할 파일 또는 디렉토리
    "glob": str | None,  # 파일을 필터링할 glob 패턴
    "type": str | None,  # 검색할 파일 타입
    "output_mode": str | None,  # "content", "files_with_matches" 또는 "count"
    "-i": bool | None,  # 대소문자 구분 없는 검색
    "-n": bool | None,  # 줄 번호 표시
    "-B": int | None,  # 각 일치 전에 표시할 줄
    "-A": int | None,  # 각 일치 후에 표시할 줄
    "-C": int | None,  # 전후에 표시할 줄
    "head_limit": int | None,  # 출력을 처음 N개 줄/항목으로 제한
    "multiline": bool | None,  # 다중 줄 모드 활성화
}
```

**출력 (content 모드):**

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

**출력 (files\_with\_matches 모드):**

```python theme={null}
{
    "files": list[str],  # 일치를 포함하는 파일
    "count": int,  # 일치를 포함하는 파일 수
}
```

<h3 id="notebookedit">
  NotebookEdit
</h3>

**도구 이름:** `NotebookEdit`

**입력:**

```python theme={null}
{
    "notebook_path": str,  # Jupyter 노트북의 절대 경로
    "cell_id": str | None,  # 편집할 셀의 ID
    "new_source": str,  # 셀의 새로운 소스
    "cell_type": "code" | "markdown" | None,  # 셀의 타입
    "edit_mode": "replace" | "insert" | "delete" | None,  # 편집 작업 타입
}
```

**출력:**

```python theme={null}
{
    "message": str,  # 성공 메시지
    "edit_type": "replaced" | "inserted" | "deleted",  # 수행된 편집 타입
    "cell_id": str | None,  # 영향을 받은 셀 ID
    "total_cells": int,  # 편집 후 노트북의 총 셀 수
}
```

<h3 id="webfetch">
  WebFetch
</h3>

**도구 이름:** `WebFetch`

**입력:**

```python theme={null}
{
    "url": str,  # 콘텐츠를 가져올 URL
    "prompt": str,  # 가져온 콘텐츠에서 실행할 프롬프트
}
```

**출력:**

```python theme={null}
{
    "bytes": int,  # 가져온 콘텐츠의 크기(바이트)
    "code": int,  # HTTP 응답 코드
    "codeText": str,  # HTTP 응답 코드 텍스트
    "result": str,  # 콘텐츠에 프롬프트를 적용한 처리된 결과
    "durationMs": int,  # 콘텐츠를 가져오고 처리하는 데 걸린 시간(밀리초)
    "url": str,  # 가져온 URL
}
```

<h3 id="websearch">
  WebSearch
</h3>

**도구 이름:** `WebSearch`

**입력:**

```python theme={null}
{
    "query": str,  # 사용할 검색 쿼리
    "allowed_domains": list[str] | None,  # 이 도메인의 결과만 포함
    "blocked_domains": list[str] | None,  # 이 도메인의 결과는 절대 포함하지 않음
}
```

**출력:**

```python theme={null}
{
    "query": str,  # 검색 쿼리
    "results": list[str | {"tool_use_id": str, "content": list[{"title": str, "url": str}]}],
    "durationSeconds": float,  # 검색 시간(초)
}
```

<h3 id="todowrite">
  TodoWrite
</h3>

**도구 이름:** `TodoWrite`

<Note>
  Claude Code v2.1.142부터 `TodoWrite`는 기본적으로 비활성화되어 있습니다. 대신 `TaskCreate`, `TaskGet`, `TaskUpdate`, `TaskList`를 사용하십시오. 모니터링 코드를 업데이트하는 방법은 [작업 도구로 마이그레이션](/docs/ko/agent-sdk/todo-tracking#migrate-to-task-tools)을 참조하거나, `CLAUDE_CODE_ENABLE_TASKS=0`을 설정하여 `TodoWrite`로 되돌리십시오.
</Note>

**입력:**

```python theme={null}
{
    "todos": [
        {
            "content": str,  # 작업 설명
            "status": "pending" | "in_progress" | "completed",  # 작업 상태
            "activeForm": str,  # 설명의 활성 형식
        }
    ]
}
```

**출력:**

```python theme={null}
{
    "message": str,  # 성공 메시지
    "stats": {"total": int, "pending": int, "in_progress": int, "completed": int},
}
```

<h3 id="taskcreate">
  TaskCreate
</h3>

**도구 이름:** `TaskCreate`

**입력:**

```python theme={null}
{
    "subject": str,  # 짧은 작업 제목
    "description": str,  # 상세한 작업 본문
    "activeForm": str | None,  # 진행 중일 때 표시되는 현재형 레이블
    "metadata": dict | None,  # 임의의 호출자 메타데이터
}
```

**출력:**

```python theme={null}
{
    "task": {"id": str, "subject": str},  # 할당된 ID가 있는 생성된 작업
}
```

<h3 id="taskupdate">
  TaskUpdate
</h3>

**도구 이름:** `TaskUpdate`

**입력:**

```python theme={null}
{
    "taskId": str,  # 패치할 작업의 ID
    "status": Literal["pending", "in_progress", "completed", "deleted"] | None,
    "subject": str | None,
    "description": str | None,
    "activeForm": str | None,
    "addBlocks": list[str] | None,  # 이 작업이 이제 차단하는 작업 ID
    "addBlockedBy": list[str] | None,  # 이제 이 작업을 차단하는 작업 ID
    "owner": str | None,
    "metadata": dict | None,
}
```

**출력:**

```python theme={null}
{
    "success": bool,
    "taskId": str,
    "updatedFields": list[str],  # 변경된 필드의 이름
    "error": str | None,
    "statusChange": {"from": str, "to": str} | None,
}
```

<h3 id="taskget">
  TaskGet
</h3>

**도구 이름:** `TaskGet`

**입력:**

```python theme={null}
{
    "taskId": str,  # 읽을 작업의 ID
}
```

**출력:**

```python theme={null}
{
    "task": {
        "id": str,
        "subject": str,
        "description": str,
        "status": Literal["pending", "in_progress", "completed"],
        "blocks": list[str],
        "blockedBy": list[str],
    } | None,  # ID를 찾을 수 없을 때 None
}
```

<h3 id="tasklist">
  TaskList
</h3>

**도구 이름:** `TaskList`

**입력:**

```python theme={null}
{}
```

**출력:**

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

**도구 이름:** `BashOutput`

**입력:**

```python theme={null}
{
    "bash_id": str,  # 백그라운드 셸의 ID
    "filter": str | None,  # 출력 줄을 필터링할 선택적 정규식
}
```

**출력:**

```python theme={null}
{
    "output": str,  # 마지막 확인 이후의 새로운 출력
    "status": "running" | "completed" | "failed",  # 현재 셸 상태
    "exitCode": int | None,  # 완료 시 종료 코드
}
```

<h3 id="killbash">
  KillBash
</h3>

**도구 이름:** `KillBash`

**입력:**

```python theme={null}
{
    "shell_id": str  # 종료할 백그라운드 셸의 ID
}
```

**출력:**

```python theme={null}
{
    "message": str,  # 성공 메시지
    "shell_id": str,  # 종료된 셸의 ID
}
```

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**도구 이름:** `ExitPlanMode`

**입력:**

```python theme={null}
{
    "plan": str  # 사용자 승인을 위해 실행할 계획
}
```

**출력:**

```python theme={null}
{
    "message": str,  # 확인 메시지
    "approved": bool | None,  # 사용자가 계획을 승인했는지 여부
}
```

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**도구 이름:** `ListMcpResourcesTool`

**입력:**

```python theme={null}
{
    "server": str | None  # 리소스를 필터링할 선택적 서버 이름
}
```

**출력:**

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

**도구 이름:** `ReadMcpResourceTool`

**입력:**

```python theme={null}
{
    "server": str,  # MCP 서버 이름
    "uri": str,  # 읽을 리소스 URI
}
```

**출력:**

```python theme={null}
{
    "contents": [
        {"uri": str, "mimeType": str | None, "text": str | None, "blob": str | None}
    ],
    "server": str,
}
```

<h2 id="advanced-features-with-claudesdkclient">
  ClaudeSDKClient를 사용한 고급 기능
</h2>

<h3 id="building-a-continuous-conversation-interface">
  지속적인 대화 인터페이스 구축
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
  동작 수정을 위해 Hooks 사용
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
  실시간 진행 상황 모니터링
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
  예제 사용
</h2>

<h3 id="basic-file-operations-using-query">
  기본 파일 작업 (query 사용)
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
  오류 처리
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
  클라이언트를 사용한 스트리밍 모드
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
  ClaudeSDKClient를 사용한 사용자 정의 도구
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
  샌드박스 구성
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

샌드박스 동작을 위한 구성입니다. 이를 사용하여 명령 샌드박싱을 활성화하고 프로그래밍 방식으로 네트워크 제한을 구성합니다.

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

| 속성                          | 타입                                                    | 기본값     | 설명                                                                                                                                                                   |
| :-------------------------- | :---------------------------------------------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `bool`                                                | `False` | 명령 실행을 위한 샌드박스 모드 활성화                                                                                                                                                |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`  | 샌드박스가 활성화되면 bash 명령 자동 승인                                                                                                                                            |
| `excludedCommands`          | `list[str]`                                           | `[]`    | 항상 샌드박스 제한을 우회하는 명령 (예: `["docker"]`). 이들은 모델 개입 없이 자동으로 샌드박스되지 않은 상태로 실행됩니다                                                                                         |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`  | 모델이 샌드박스 외부에서 명령 실행을 요청하도록 허용합니다. `True`일 때, 모델은 도구 입력에서 `dangerouslyDisableSandbox`를 설정할 수 있으며, 이는 [권한 시스템](#permissions-fallback-for-unsandboxed-commands)으로 폴백됩니다 |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`  | 네트워크 특정 샌드박스 구성                                                                                                                                                      |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`  | 무시할 샌드박스 위반을 구성합니다                                                                                                                                                   |
| `enableWeakerNestedSandbox` | `bool`                                                | `False` | 호환성을 위해 더 약한 중첩 샌드박스 활성화                                                                                                                                             |

<Note>
  샌드박스는 플랫폼 지원에 따라 달라지며, Linux에서는 `bubblewrap` 및 `socat`과 같은 도구가 필요합니다. 기본적으로 `enabled`가 `True`이지만 샌드박스를 시작할 수 없을 때, 명령은 stderr에 경고와 함께 샌드박스되지 않은 상태로 실행됩니다. 이 기본값은 `failIfUnavailable`이 `true`로 기본 설정되는 TypeScript SDK와 다릅니다.

  대신 중지하려면 샌드박스 설정에서 `"failIfUnavailable": True`를 설정하십시오. 이 키는 아직 `SandboxSettings`에 선언되지 않았지만, SDK는 이를 Claude Code로 전달하며, Claude Code는 이를 준수합니다. 그러면 `query()`는 메시지를 생성하기 전에 예외를 발생시키지 않고 `subtype="error_during_execution"`과 `errors`의 이유를 포함한 `ResultMessage`를 보고합니다. `query()`가 메시지를 생성하기 전에 예외를 발생시킬 것으로 예상하지 말고 해당 서브타입을 감시하십시오.
</Note>

<h4 id="example-usage-2">
  사용 예제
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
  **Unix 소켓 보안**: `allowUnixSockets` 옵션은 강력한 시스템 서비스에 대한 접근을 부여할 수 있습니다. 예를 들어, `/var/run/docker.sock`을 허용하면 Docker API를 통해 샌드박스 격리를 우회하여 전체 호스트 시스템 접근을 효과적으로 부여합니다. 엄격히 필요한 Unix 소켓만 허용하고 각각의 보안 영향을 이해하십시오.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

샌드박스 모드를 위한 네트워크 특정 구성입니다. 이러한 설정은 부모 [`SandboxSettings`](#sandboxsettings)에서 `enabled`가 `True`일 때 샌드박스된 Bash 명령에 적용됩니다. 이들은 [권한 규칙](/docs/ko/permissions#webfetch)을 대신 사용하는 WebFetch 도구를 제한하지 않습니다.

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

| 속성                        | 타입          | 기본값     | 설명                                                                                           |
| :------------------------ | :---------- | :------ | :------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `list[str]` | `[]`    | 샌드박스된 프로세스가 접근할 수 있는 도메인 이름                                                                  |
| `deniedDomains`           | `list[str]` | `[]`    | 샌드박스된 프로세스가 접근할 수 없는 도메인 이름입니다. `allowedDomains`보다 우선합니다                                     |
| `allowManagedDomainsOnly` | `bool`      | `False` | 관리되는 설정만: 관리되는 설정에서 설정되면, 관리되지 않는 설정 소스의 `allowedDomains`를 무시합니다. SDK 옵션을 통해 설정할 때는 효과가 없습니다 |
| `allowUnixSockets`        | `list[str]` | `[]`    | 프로세스가 접근할 수 있는 Unix 소켓 경로 (예: Docker 소켓)                                                     |
| `allowAllUnixSockets`     | `bool`      | `False` | 모든 Unix 소켓에 대한 접근 허용                                                                         |
| `allowLocalBinding`       | `bool`      | `False` | 프로세스가 로컬 포트에 바인딩하도록 허용 (예: 개발 서버용)                                                           |
| `allowMachLookup`         | `list[str]` | `[]`    | macOS만 해당: 허용할 XPC/Mach 서비스 이름입니다. 후행 와일드카드를 지원합니다                                           |
| `httpProxyPort`           | `int`       | `None`  | 네트워크 요청을 위한 HTTP 프록시 포트                                                                      |
| `socksProxyPort`          | `int`       | `None`  | 네트워크 요청을 위한 SOCKS 프록시 포트                                                                     |

<Note>
  기본 제공 샌드박스 프록시는 요청된 호스트명을 기반으로 네트워크 허용 목록을 적용하며 TLS 트래픽을 종료하거나 검사하지 않으므로, [도메인 프론팅](https://en.wikipedia.org/wiki/Domain_fronting)과 같은 기술이 이를 우회할 수 있습니다. 자세한 내용은 [샌드박싱 보안 제한 사항](/docs/ko/sandboxing#security-limitations)을 참조하고, TLS 종료 프록시 구성은 [안전한 배포](/docs/ko/agent-sdk/secure-deployment#traffic-forwarding)를 참조하십시오.
</Note>

<h3 id="sandboxignoreviolations">
  `SandboxIgnoreViolations`
</h3>

특정 샌드박스 위반을 무시하기 위한 구성입니다.

```python theme={null}
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| 속성        | 타입          | 기본값  | 설명               |
| :-------- | :---------- | :--- | :--------------- |
| `file`    | `list[str]` | `[]` | 위반을 무시할 파일 경로 패턴 |
| `network` | `list[str]` | `[]` | 위반을 무시할 네트워크 패턴  |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  샌드박스되지 않은 명령을 위한 권한 폴백
</h3>

`allowUnsandboxedCommands`가 활성화되면, 모델은 도구 입력에서 `dangerouslyDisableSandbox: True`를 설정하여 샌드박스 외부에서 명령 실행을 요청할 수 있습니다. 이러한 요청은 기존 권한 시스템으로 폴백되므로, `can_use_tool` 핸들러가 호출되어 사용자 정의 인증 로직을 구현할 수 있습니다.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: 항상 자동으로 샌드박스를 우회하는 명령의 정적 목록 (예: `["docker"]`). 모델이 이를 제어할 수 없습니다.
  * `allowUnsandboxedCommands`: 모델이 도구 입력에서 `dangerouslyDisableSandbox: True`를 설정하여 런타임에 샌드박스되지 않은 실행을 요청하도록 합니다.
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

이 패턴을 사용하면 다음을 수행할 수 있습니다.

* **모델 요청 감사**: 모델이 샌드박스되지 않은 실행을 요청할 때 로깅
* **허용 목록 구현**: 특정 명령만 샌드박스되지 않은 상태로 실행하도록 허용
* **승인 워크플로우 추가**: 권한 있는 작업에 대한 명시적 인증 필요

<Warning>
  `dangerouslyDisableSandbox: True`로 실행되는 명령은 전체 시스템 접근 권한이 있습니다. `can_use_tool` 핸들러가 이러한 요청을 신중하게 검증하는지 확인하십시오.

  `permission_mode`가 `bypassPermissions`로 설정되고 `allow_unsandboxed_commands`가 활성화되면, 모델은 승인 프롬프트 없이 샌드박스 외부에서 명령을 자동으로 실행할 수 있습니다. 이 조합은 모델이 샌드박스 격리를 조용히 탈출할 수 있도록 효과적으로 허용합니다.
</Warning>

<h2 id="see-also">
  참고 항목
</h2>

* [SDK 개요](/docs/ko/agent-sdk/overview) - 일반 SDK 개념
* [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript) - TypeScript SDK 문서
* [CLI 참조](/docs/ko/cli-reference) - 명령줄 인터페이스
* [일반적인 워크플로우](/docs/ko/common-workflows) - 단계별 가이드
