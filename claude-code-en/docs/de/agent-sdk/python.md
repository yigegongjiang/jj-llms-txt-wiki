> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK Referenz - Python

> Vollständige API-Referenz für das Python Agent SDK, einschließlich aller Funktionen, Typen und Klassen.

<h2 id="installation">
  Installation
</h2>

Installieren Sie das Paket in einer virtuellen Umgebung. Bei aktuellen Debian-, Ubuntu- und Homebrew-Python-Installationen schlägt die Ausführung von `pip install` gegen System-Python mit `error: externally-managed-environment` fehl.

```bash theme={null}
python3 -m venv .venv
source .venv/bin/activate
pip install claude-agent-sdk
```

Für uv, Windows PowerShell und API-Schlüssel-Setup siehe [Setup in der Agent SDK-Schnellstartanleitung](/docs/de/agent-sdk/quickstart#setup).

<h2 id="choosing-between-query-and-claudesdkclient">
  Wahl zwischen `query()` und `ClaudeSDKClient`
</h2>

Das Python SDK bietet zwei Möglichkeiten, um mit Claude Code zu interagieren:

| Funktion                     | `query()`                                          | `ClaudeSDKClient`                      |
| :--------------------------- | :------------------------------------------------- | :------------------------------------- |
| **Sitzung**                  | Erstellt standardmäßig eine neue Sitzung           | Verwendet dieselbe Sitzung erneut      |
| **Konversation**             | Einzelner Austausch                                | Mehrere Austausche im gleichen Kontext |
| **Verbindung**               | Automatisch verwaltet                              | Manuelle Kontrolle                     |
| **Streaming-Eingabe**        | ✅ Unterstützt                                      | ✅ Unterstützt                          |
| **Unterbrechungen**          | ❌ Nicht unterstützt                                | ✅ Unterstützt                          |
| **Hooks**                    | ✅ Unterstützt                                      | ✅ Unterstützt                          |
| **Benutzerdefinierte Tools** | ✅ Unterstützt                                      | ✅ Unterstützt                          |
| **Konversation fortsetzen**  | Manuell über `continue_conversation` oder `resume` | ✅ Automatisch                          |
| **Anwendungsfall**           | Einmalige Aufgaben                                 | Kontinuierliche Konversationen         |

Verwenden Sie `ClaudeSDKClient` für interaktive Anwendungen wie Chat-Schnittstellen oder wenn die nächste Aktion von Claudes Antwort abhängt.

<h2 id="functions">
  Funktionen
</h2>

<Note>Signaturblöcke und bloße `async for` / `async with` Fragmente auf dieser Seite sind illustrativ. Um sie auszuführen, wickeln Sie den Text in `async def main(): ...` ein und rufen Sie `asyncio.run(main())` auf.</Note>

<h3 id="query">
  `query()`
</h3>

Erstellt für jede Interaktion mit Claude Code standardmäßig eine neue Sitzung. Gibt einen asynchronen Iterator zurück, der Nachrichten bei ihrer Ankunft liefert. Jeder Aufruf von `query()` beginnt neu ohne Erinnerung an vorherige Interaktionen, es sei denn, Sie übergeben `continue_conversation=True` oder `resume` in [`ClaudeAgentOptions`](#claudeagentoptions). Siehe [Sitzungen](/docs/de/agent-sdk/sessions).

```python theme={null}
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None,
    transport: Transport | None = None
) -> AsyncIterator[Message]
```

<h4 id="parameters">
  Parameter
</h4>

| Parameter   | Typ                          | Beschreibung                                                                               |
| :---------- | :--------------------------- | :----------------------------------------------------------------------------------------- |
| `prompt`    | `str \| AsyncIterable[dict]` | Die Eingabeaufforderung als Zeichenkette oder asynchroner Iterator für den Streaming-Modus |
| `options`   | `ClaudeAgentOptions \| None` | Optionales Konfigurationsobjekt (standardmäßig `ClaudeAgentOptions()`, wenn None)          |
| `transport` | `Transport \| None`          | Optionaler benutzerdefinierter Transport für die Kommunikation mit dem CLI-Prozess         |

<h4 id="returns">
  Rückgabewert
</h4>

Gibt einen `AsyncIterator[Message]` zurück, der Nachrichten aus der Konversation liefert.

<h4 id="example-with-options">
  Beispiel - Mit Optionen
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

Dekorator zum Definieren von MCP-Tools mit Typsicherheit.

```python theme={null}
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any],
    annotations: ToolAnnotations | None = None
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

<h4 id="parameters-2">
  Parameter
</h4>

| Parameter      | Typ                                             | Beschreibung                                                                                                |
| :------------- | :---------------------------------------------- | :---------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Eindeutige Kennung für das Tool                                                                             |
| `description`  | `str`                                           | Lesbare Beschreibung, was das Tool tut                                                                      |
| `input_schema` | `type \| dict[str, Any]`                        | Schema, das die Eingabeparameter des Tools definiert. Siehe [Eingabeschema-Optionen](#input-schema-options) |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Optionale MCP-Tool-Anmerkungen, die Verhaltenshinweise für Clients bereitstellen                            |

<h4 id="input-schema-options">
  Eingabeschema-Optionen
</h4>

1. **Einfache Typ-Zuordnung** (empfohlen):

   ```python theme={null}
   {"text": str, "count": int, "enabled": bool}
   ```

2. **JSON-Schema-Format** (für komplexe Validierung):
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
  Rückgabewert
</h4>

Eine Dekoratorfunktion, die die Tool-Implementierung umhüllt und eine `SdkMcpTool`-Instanz zurückgibt.

<h4 id="example">
  Beispiel
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

Verhaltenshinweise für ein Tool, die als `annotations`-Argument von [`tool()`](#tool) übergeben werden. `ToolAnnotations` erweitert die `mcp.types.ToolAnnotations` des MCP SDK um ein `maxResultSizeChars`-Feld, und Sie können jeden Hinweis in camelCase oder snake\_case schreiben: `ToolAnnotations(readOnlyHint=True)` und `ToolAnnotations(read_only_hint=True)` sind gleichwertig. Sie können auch überall dort, wo das SDK Anmerkungen akzeptiert, ein einfaches `mcp.types.ToolAnnotations` übergeben.

Die snake\_case-Namen und das typisierte `maxResultSizeChars`-Feld erfordern Python Agent SDK 0.2.140 oder später. Versionen 0.1.31 bis 0.2.139 exportieren `mcp.types.ToolAnnotations` unverändert erneut. In Versionen 0.1.55 bis 0.2.139 können Sie `maxResultSizeChars` immer noch als Schlüsselwortargument übergeben: Die MCP-Klasse akzeptiert zusätzliche Felder, und das SDK leitet den Wert an Claude Code weiter.

Alle Felder sind optional. Clients sollten sich nicht auf die Hinweise für Sicherheitsentscheidungen verlassen.

| Feld                 | Typ            | Standard | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :------------------- | :------------- | :------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`              | `str \| None`  | `None`   | Lesbare Bezeichnung für das Tool                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `readOnlyHint`       | `bool \| None` | `False`  | Wenn `True`, ändert das Tool seine Umgebung nicht                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `destructiveHint`    | `bool \| None` | `True`   | Wenn `True`, kann das Tool destruktive Aktualisierungen durchführen (nur sinnvoll, wenn `readOnlyHint` `False` ist)                                                                                                                                                                                                                                                                                                                                                     |
| `idempotentHint`     | `bool \| None` | `False`  | Wenn `True`, haben wiederholte Aufrufe mit denselben Argumenten keine zusätzliche Auswirkung (nur sinnvoll, wenn `readOnlyHint` `False` ist)                                                                                                                                                                                                                                                                                                                            |
| `openWorldHint`      | `bool \| None` | `True`   | Wenn `True`, interagiert das Tool mit externen Entitäten (z. B. Websuche). Wenn `False`, ist die Domäne des Tools geschlossen (z. B. ein Memory-Tool)                                                                                                                                                                                                                                                                                                                   |
| `maxResultSizeChars` | `int \| None`  | `None`   | Anzahl der Zeichen, bis zu denen Claude Code das Textergebnis dieses Tools inline in der Konversation behält, anstatt es in einer Datei zu speichern, bis zu 500.000. Ergebnisse, die Bilder enthalten, sind nicht betroffen. Eine Claude Code-Einstellung statt eines MCP-Hinweises: Das SDK sendet es in der `_meta` des Tools als `anthropic/maxResultSizeChars`. Siehe [Erhöhen Sie das Limit für ein bestimmtes Tool](/docs/de/mcp#raise-the-limit-for-a-specific-tool) |

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

Erstellt einen In-Process-MCP-Server, der in Ihrer Python-Anwendung ausgeführt wird.

```python theme={null}
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

<h4 id="parameters-3">
  Parameter
</h4>

| Parameter | Typ                             | Standard  | Beschreibung                                                             |
| :-------- | :------------------------------ | :-------- | :----------------------------------------------------------------------- |
| `name`    | `str`                           | -         | Eindeutige Kennung für den Server                                        |
| `version` | `str`                           | `"1.0.0"` | Versionsnummer des Servers                                               |
| `tools`   | `list[SdkMcpTool[Any]] \| None` | `None`    | Liste von Tool-Funktionen, die mit dem `@tool`-Dekorator erstellt wurden |

<h4 id="returns-3">
  Rückgabewert
</h4>

Gibt ein `McpSdkServerConfig`-Objekt zurück, das an `ClaudeAgentOptions.mcp_servers` übergeben werden kann.

<h4 id="example-2">
  Beispiel
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

Listet vergangene Sitzungen mit Metadaten auf. Filtern Sie nach Projektverzeichnis oder listen Sie Sitzungen über alle Projekte auf. Synchron; gibt sofort zurück.

```python theme={null}
def list_sessions(
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0,
    include_worktrees: bool = True
) -> list[SDKSessionInfo]
```

<h4 id="parameters-4">
  Parameter
</h4>

| Parameter           | Typ           | Standard | Beschreibung                                                                                                                        |
| :------------------ | :------------ | :------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| `directory`         | `str \| None` | `None`   | Verzeichnis, für das Sitzungen aufgelistet werden sollen. Wenn weggelassen, werden Sitzungen über alle Projekte zurückgegeben       |
| `limit`             | `int \| None` | `None`   | Maximale Anzahl der zurückzugebenden Sitzungen                                                                                      |
| `offset`            | `int`         | `0`      | Anzahl der Sitzungen, die vom Anfang der sortierten Ergebnisse übersprungen werden sollen. Verwenden Sie mit `limit` für Pagination |
| `include_worktrees` | `bool`        | `True`   | Wenn `directory` sich in einem Git-Repository befindet, Sitzungen aus allen worktrees einbeziehen                                   |

<h4 id="return-type-sdksessioninfo">
  Rückgabetyp: `SDKSessionInfo`
</h4>

| Eigenschaft     | Typ           | Beschreibung                                                                                            |
| :-------------- | :------------ | :------------------------------------------------------------------------------------------------------ |
| `session_id`    | `str`         | Eindeutige Sitzungskennung                                                                              |
| `summary`       | `str`         | Anzeigetitel: benutzerdefinierter Titel, automatisch generierte Zusammenfassung oder erste Aufforderung |
| `last_modified` | `int`         | Letzte Änderungszeit in Millisekunden seit Epoche                                                       |
| `file_size`     | `int \| None` | Sitzungsdateigröße in Bytes (`None` für Remote-Speicher-Backends)                                       |
| `custom_title`  | `str \| None` | Vom Benutzer festgelegter Sitzungstitel                                                                 |
| `first_prompt`  | `str \| None` | Erste aussagekräftige Benutzeraufforderung in der Sitzung                                               |
| `git_branch`    | `str \| None` | Git-Branch am Ende der Sitzung                                                                          |
| `cwd`           | `str \| None` | Arbeitsverzeichnis für die Sitzung                                                                      |
| `tag`           | `str \| None` | Vom Benutzer festgelegtes Sitzungs-Tag (siehe [`tag_session()`](#tag_session))                          |
| `created_at`    | `int \| None` | Sitzungserstellungszeit in Millisekunden seit Epoche                                                    |

<h4 id="example-3">
  Beispiel
</h4>

Geben Sie die 10 neuesten Sitzungen für ein Projekt aus. Die Ergebnisse werden nach `last_modified` absteigend sortiert, daher ist das erste Element das neueste. Lassen Sie `directory` weg, um über alle Projekte zu suchen.

```python theme={null}
from claude_agent_sdk import list_sessions

for session in list_sessions(directory="/path/to/project", limit=10):
    print(f"{session.summary} ({session.session_id})")
```

<h3 id="get_session_messages">
  `get_session_messages()`
</h3>

Ruft Nachrichten aus einer vergangenen Sitzung ab. Synchron; gibt sofort zurück.

```python theme={null}
def get_session_messages(
    session_id: str,
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0
) -> list[SessionMessage]
```

<h4 id="parameters-5">
  Parameter
</h4>

| Parameter    | Typ           | Standard     | Beschreibung                                                                     |
| :----------- | :------------ | :----------- | :------------------------------------------------------------------------------- |
| `session_id` | `str`         | erforderlich | Die Sitzungs-ID, für die Nachrichten abgerufen werden sollen                     |
| `directory`  | `str \| None` | `None`       | Projektverzeichnis zum Suchen. Wenn weggelassen, werden alle Projekte durchsucht |
| `limit`      | `int \| None` | `None`       | Maximale Anzahl der zurückzugebenden Nachrichten                                 |
| `offset`     | `int`         | `0`          | Anzahl der Nachrichten, die vom Anfang übersprungen werden sollen                |

<h4 id="return-type-sessionmessage">
  Rückgabetyp: `SessionMessage`
</h4>

| Eigenschaft          | Typ                            | Beschreibung                                                                                                                                                                                                                                                                                      |
| :------------------- | :----------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`               | `Literal["user", "assistant"]` | Nachrichtenrolle                                                                                                                                                                                                                                                                                  |
| `uuid`               | `str`                          | Eindeutige Nachrichtenkennung                                                                                                                                                                                                                                                                     |
| `session_id`         | `str`                          | Sitzungskennung                                                                                                                                                                                                                                                                                   |
| `message`            | `Any`                          | Roher Nachrichteninhalt                                                                                                                                                                                                                                                                           |
| `parent_tool_use_id` | `str \| None`                  | Für Subagent-Nachrichten die ID des erzeugenden `Agent`-Tool-Use-Blocks. `None` für Hauptsitzungs-Nachrichten und ältere Sitzungen                                                                                                                                                                |
| `parent_agent_id`    | `str \| None`                  | Für Nachrichten von einem [verschachtelten Subagent](/docs/de/sub-agents#let-subagents-spawn-their-own-subagents), die Agent-ID des übergeordneten Subagent. `None` für Hauptsitzungs-Nachrichten, Top-Level-Subagent-Nachrichten und ältere Sitzungen. Erfordert Python Agent SDK 0.2.140 oder später |

<h4 id="example-4">
  Beispiel
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

Liest Metadaten für eine einzelne Sitzung nach ID, ohne das vollständige Projektverzeichnis zu durchsuchen. Synchron; gibt sofort zurück.

```python theme={null}
def get_session_info(
    session_id: str,
    directory: str | None = None,
) -> SDKSessionInfo | None
```

<h4 id="parameters-6">
  Parameter
</h4>

| Parameter    | Typ           | Standard     | Beschreibung                                                                          |
| :----------- | :------------ | :----------- | :------------------------------------------------------------------------------------ |
| `session_id` | `str`         | erforderlich | UUID der zu suchenden Sitzung                                                         |
| `directory`  | `str \| None` | `None`       | Projektverzeichnispath. Wenn weggelassen, werden alle Projektverzeichnisse durchsucht |

Gibt [`SDKSessionInfo`](#return-type-sdksessioninfo) zurück, oder `None`, wenn die Sitzung nicht gefunden wird.

<h4 id="example-5">
  Beispiel
</h4>

Suchen Sie die Metadaten einer einzelnen Sitzung, ohne das Projektverzeichnis zu durchsuchen. Nützlich, wenn Sie bereits eine Sitzungs-ID aus einem vorherigen Durchlauf haben.

```python theme={null}
from claude_agent_sdk import get_session_info

info = get_session_info("550e8400-e29b-41d4-a716-446655440000")
if info:
    print(f"{info.summary} (branch: {info.git_branch}, tag: {info.tag})")
```

<h3 id="rename_session">
  `rename_session()`
</h3>

Benennt eine Sitzung um, indem ein benutzerdefinierter Titeleintrag angehängt wird. Wiederholte Aufrufe sind sicher; der neueste Titel gewinnt. Synchron.

```python theme={null}
def rename_session(
    session_id: str,
    title: str,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-7">
  Parameter
</h4>

| Parameter    | Typ           | Standard     | Beschreibung                                                                          |
| :----------- | :------------ | :----------- | :------------------------------------------------------------------------------------ |
| `session_id` | `str`         | erforderlich | UUID der umzubenennenden Sitzung                                                      |
| `title`      | `str`         | erforderlich | Neuer Titel. Muss nach dem Entfernen von Leerzeichen nicht leer sein                  |
| `directory`  | `str \| None` | `None`       | Projektverzeichnispath. Wenn weggelassen, werden alle Projektverzeichnisse durchsucht |

Wirft `ValueError`, wenn `session_id` keine gültige UUID ist oder `title` leer ist; `FileNotFoundError`, wenn die Sitzung nicht gefunden werden kann.

<h4 id="example-6">
  Beispiel
</h4>

Benennen Sie die neueste Sitzung um, damit sie später leichter zu finden ist. Der neue Titel wird in [`SDKSessionInfo.custom_title`](#return-type-sdksessioninfo) bei nachfolgenden Lesevorgängen angezeigt.

```python theme={null}
from claude_agent_sdk import list_sessions, rename_session

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    rename_session(sessions[0].session_id, "Refactor auth module")
```

<h3 id="tag_session">
  `tag_session()`
</h3>

Markiert eine Sitzung mit einem Tag. Übergeben Sie `None`, um das Tag zu löschen. Wiederholte Aufrufe sind sicher; das neueste Tag gewinnt. Synchron.

```python theme={null}
def tag_session(
    session_id: str,
    tag: str | None,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-8">
  Parameter
</h4>

| Parameter    | Typ           | Standard     | Beschreibung                                                                          |
| :----------- | :------------ | :----------- | :------------------------------------------------------------------------------------ |
| `session_id` | `str`         | erforderlich | UUID der zu markierenden Sitzung                                                      |
| `tag`        | `str \| None` | erforderlich | Tag-Zeichenkette oder `None` zum Löschen. Unicode-bereinigt vor dem Speichern         |
| `directory`  | `str \| None` | `None`       | Projektverzeichnispath. Wenn weggelassen, werden alle Projektverzeichnisse durchsucht |

Wirft `ValueError`, wenn `session_id` keine gültige UUID ist oder `tag` nach der Bereinigung leer ist; `FileNotFoundError`, wenn die Sitzung nicht gefunden werden kann.

<h4 id="example-7">
  Beispiel
</h4>

Markieren Sie eine Sitzung mit einem Tag, und filtern Sie später nach diesem Tag. Übergeben Sie `None`, um ein vorhandenes Tag zu löschen.

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
  Klassen
</h2>

<h3 id="claudesdkclient">
  `ClaudeSDKClient`
</h3>

**Behält eine Konversationssitzung über mehrere Austausche hinweg bei.** Dies ist das Python-Äquivalent dazu, wie die `query()`-Funktion des TypeScript SDK intern funktioniert - sie erstellt ein Client-Objekt, das Konversationen fortsetzen kann. Siehe den [Vergleich mit `query()`](#choosing-between-query-and-claudesdkclient).

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
  Methoden
</h4>

| Methode                                   | Beschreibung                                                                                                                                                                                     |
| :---------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `__init__(options)`                       | Initialisieren Sie den Client mit optionaler Konfiguration                                                                                                                                       |
| `connect(prompt)`                         | Verbinden Sie sich mit Claude mit einer optionalen anfänglichen Aufforderung oder einem Nachrichtenstrom                                                                                         |
| `query(prompt, session_id)`               | Senden Sie eine neue Anfrage im Streaming-Modus                                                                                                                                                  |
| `receive_messages()`                      | Empfangen Sie alle Nachrichten von Claude als asynchronen Iterator                                                                                                                               |
| `receive_response()`                      | Empfangen Sie Nachrichten bis einschließlich einer ResultMessage                                                                                                                                 |
| `interrupt()`                             | Senden Sie ein Unterbrechungssignal (funktioniert nur im Streaming-Modus)                                                                                                                        |
| `set_permission_mode(mode)`               | Ändern Sie den Berechtigungsmodus für die aktuelle Sitzung                                                                                                                                       |
| `set_model(model)`                        | Ändern Sie das Modell für die aktuelle Sitzung. Übergeben Sie `None`, um auf [Claude Code's Standardmodell](/docs/de/model-config) zurückzusetzen                                                     |
| `rewind_files(user_message_id)`           | Stellen Sie Dateien in ihren Zustand bei der angegebenen Benutzernachricht wieder her. Erfordert `enable_file_checkpointing=True`. Siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Rufen Sie den Status aller konfigurierten MCP-Server ab. Gibt [`McpStatusResponse`](#mcpstatusresponse) zurück                                                                                   |
| `reconnect_mcp_server(server_name)`       | Versuchen Sie, eine Verbindung zu einem MCP-Server herzustellen, der fehlgeschlagen ist oder getrennt wurde                                                                                      |
| `toggle_mcp_server(server_name, enabled)` | Aktivieren oder deaktivieren Sie einen MCP-Server während der Sitzung. Das Deaktivieren entfernt seine Tools                                                                                     |
| `stop_task(task_id)`                      | Stoppen Sie eine laufende Hintergrundaufgabe. Eine [`TaskNotificationMessage`](#tasknotificationmessage) mit Status `"stopped"` folgt im Nachrichtenstrom                                        |
| `get_server_info()`                       | Rufen Sie die Initialisierungsinformationen des Servers ab, einschließlich verfügbarer Befehle und Ausgabestile                                                                                  |
| `disconnect()`                            | Trennen Sie die Verbindung zu Claude                                                                                                                                                             |

<h4 id="context-manager-support">
  Context Manager-Unterstützung
</h4>

Der Client kann als asynchroner Context Manager für automatische Verbindungsverwaltung verwendet werden:

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

> **Wichtig:** Vermeiden Sie bei der Iteration über Nachrichten die Verwendung von `break`, um vorzeitig zu beenden, da dies zu asyncio-Bereinigungsproblemen führen kann. Lassen Sie die Iteration stattdessen natürlich abschließen oder verwenden Sie Flags, um zu verfolgen, wann Sie gefunden haben, was Sie brauchen.

<h4 id="example-continuing-a-conversation">
  Beispiel - Konversation fortsetzen
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
  Beispiel - Streaming-Eingabe mit ClaudeSDKClient
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
  Beispiel - Unterbrechungen verwenden
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
  **Pufferverhalten nach Unterbrechung:** `interrupt()` sendet ein Stopsignal, löscht aber nicht den Nachrichtenpuffer. Nachrichten, die bereits von der unterbrochenen Aufgabe produziert wurden, einschließlich ihrer `ResultMessage`, bleiben im Stream. Sie müssen sie mit `receive_response()` entleeren, bevor Sie die Antwort auf eine neue Abfrage lesen. Wenn Sie unmittelbar nach `interrupt()` eine neue Abfrage senden und `receive_response()` nur einmal aufrufen, erhalten Sie die Nachrichten der unterbrochenen Aufgabe, nicht die Antwort der neuen Abfrage.
</Note>

<h4 id="example-advanced-permission-control">
  Beispiel - Erweiterte Berechtigungskontrolle
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
  Typen
</h2>

<Note>
  **`@dataclass` vs `TypedDict`:** Dieses SDK verwendet zwei Arten von Typen. Klassen, die mit `@dataclass` dekoriert sind (wie `ResultMessage`, `AgentDefinition`, `TextBlock`), sind zur Laufzeit Objektinstanzen und unterstützen Attributzugriff: `msg.result`. Klassen, die mit `TypedDict` definiert sind (wie `ThinkingConfigEnabled`, `McpStdioServerConfig`, `SyncHookJSONOutput`), sind **zur Laufzeit einfache Dicts** und erfordern Schlüsselzugriff: `config["budget_tokens"]`, nicht `config.budget_tokens`. Die `ClassName(field=value)`-Aufrufsyntax funktioniert für beide, aber nur Dataclasses erzeugen Objekte mit Attributen.
</Note>

<h3 id="sdkmcptool">
  `SdkMcpTool`
</h3>

Definition für ein SDK MCP-Tool, das mit dem `@tool`-Dekorator erstellt wurde.

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| Eigenschaft    | Typ                                             | Beschreibung                                                                                                |
| :------------- | :---------------------------------------------- | :---------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Eindeutige Kennung für das Tool                                                                             |
| `description`  | `str`                                           | Lesbare Beschreibung                                                                                        |
| `input_schema` | `type[T] \| dict[str, Any]`                     | Schema für Eingabevalidierung                                                                               |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]`      | Asynchrone Funktion, die die Tool-Ausführung handhabt                                                       |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Optionale Tool-Anmerkungen (z. B. `readOnlyHint`, `destructiveHint`, `openWorldHint`, `maxResultSizeChars`) |

<h3 id="transport">
  `Transport`
</h3>

Abstrakte Basisklasse für benutzerdefinierte Transport-Implementierungen. Verwenden Sie dies, um mit dem Claude-Prozess über einen benutzerdefinierten Kanal zu kommunizieren (z. B. eine Remote-Verbindung statt eines lokalen Subprozesses).

<Warning>
  Dies ist eine Low-Level-interne API. Die Schnittstelle kann sich in zukünftigen Versionen ändern. Benutzerdefinierte Implementierungen müssen aktualisiert werden, um Schnittstellenänderungen zu entsprechen.
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

| Methode           | Beschreibung                                                               |
| :---------------- | :------------------------------------------------------------------------- |
| `connect()`       | Verbinden Sie den Transport und bereiten Sie ihn für die Kommunikation vor |
| `write(data)`     | Schreiben Sie Rohdaten (JSON + Zeilenumbruch) in den Transport             |
| `read_messages()` | Asynchroner Iterator, der geparste JSON-Nachrichten liefert                |
| `close()`         | Schließen Sie die Verbindung und bereinigen Sie Ressourcen                 |
| `is_ready()`      | Gibt `True` zurück, wenn der Transport senden und empfangen kann           |
| `end_input()`     | Schließen Sie den Eingabestrom (z. B. stdin für Subprozess-Transporte)     |

Import: `from claude_agent_sdk import Transport`

<h3 id="claudeagentoptions">
  `ClaudeAgentOptions`
</h3>

Konfigurationsdatenklasse für Claude Code-Abfragen.

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

| Eigenschaft                   | Typ                                                                                   | Standard                            | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :---------------------------- | :------------------------------------------------------------------------------------ | :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                              | Tools-Konfiguration. Verwenden Sie `{"type": "preset", "preset": "claude_code"}` für die Standard-Tools von Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                                | Tools, die automatisch genehmigt werden, ohne zu fragen. Dies beschränkt Claude nicht nur auf diese Tools. Wenn Sie einen der [Task-Tracking-Tools](/docs/de/agent-sdk/todo-tracking#model-availability) hier nennen, aktiviert Claude Code die Sitzung auch. Andere nicht aufgelistete Tools fallen durch `permission_mode` und `can_use_tool`. Verwenden Sie `disallowed_tools`, um Tools zu blockieren. Siehe [Berechtigungen](/docs/de/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                         |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptCustom \| SystemPromptFile \| None`         | `None`                              | System-Prompt-Konfiguration. Übergeben Sie eine Zeichenkette für einen benutzerdefinierten Prompt, `{"type": "preset", "preset": "claude_code"}` für den System-Prompt von Claude Code mit optionalem `"append"`, `{"type": "custom", "prompt": "..."}` für einen benutzerdefinierten Prompt, der auch `"snapshot"` setzen kann, oder `{"type": "file", "path": "..."}` zum Laden eines großen Prompts von der Festplatte. Siehe [`SystemPromptPreset`](#systempromptpreset), [`SystemPromptCustom`](#systempromptcustom) und [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                                                |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                                | MCP-Server-Konfigurationen oder Pfad zur Konfigurationsdatei                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `strict_mcp_config`           | `bool`                                                                                | `False`                             | Wenn `True`, verwenden Sie nur die Server, die in `mcp_servers` übergeben werden, und ignorieren Sie das Projekt `.mcp.json`, Benutzereinstellungen, von Plugins bereitgestellte MCP-Server und [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai). Entspricht dem CLI-Flag `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                              | Berechtigungsmodus für die Tool-Nutzung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `continue_conversation`       | `bool`                                                                                | `False`                             | Setzen Sie die neueste Konversation fort                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `resume`                      | `str \| None`                                                                         | `None`                              | Sitzungs-ID zum Fortsetzen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `session_id`                  | `str \| None`                                                                         | `None`                              | Verwenden Sie eine bestimmte Sitzungs-ID statt einer automatisch generierten. Muss eine gültige UUID sein. Kann nicht mit `continue_conversation` oder `resume` kombiniert werden, es sei denn, `fork_session` ist auch gesetzt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `max_turns`                   | `int \| None`                                                                         | `None`                              | Maximale agentengesteuerte Umdrehungen (Tool-Use-Rundgänge)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                              | Stoppen Sie die Abfrage, wenn die clientseitige Kostenschätzung diesen USD-Wert erreicht. Zählt nur die Ausgaben des Aufrufs selbst; Gesamtwerte aus einer fortgesetzten Sitzung zählen nicht. Für Genauigkeitsvorbehalt und Zurücksetzen-Verhalten siehe [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                                | Tools, die verweigert werden. Ein einfacher Name wie `"Bash"` entfernt das Tool aus Claudes Kontext. Eine scoped-Regel wie `"Bash(rm *)"` lässt das Tool verfügbar und verweigert übereinstimmende Aufrufe in jedem Berechtigungsmodus, einschließlich `bypassPermissions`, für den Befehl [wie geschrieben](/docs/de/permissions#bash-rule-limits). Siehe [Berechtigungen](/docs/de/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                               |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                             | Aktivieren Sie die Dateienänderungsverfolgung zum Zurückspulen. Siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `model`                       | `str \| None`                                                                         | `None`                              | Claude-Modell-Alias oder vollständiger Modellname. Siehe [akzeptierte Werte und anbieter-spezifische IDs](/docs/de/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `fallback_model`              | `str \| None`                                                                         | `None`                              | Fallback-Modell, das verwendet wird, wenn das primäre Modell fehlschlägt. Akzeptiert eine kommagetrennte Liste. Anleitungen finden Sie unter [Modell auswählen](/docs/de/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                                | Beta-Funktionen zum Aktivieren. Siehe [`SdkBeta`](#sdkbeta) für verfügbare Optionen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                              | Ausgabeformat für strukturierte Antworten (z. B. `{"type": "json_schema", "schema": {...}}`). Siehe [Strukturierte Ausgaben](/docs/de/agent-sdk/structured-outputs) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                              | MCP-Tool-Name für Berechtigungsaufforderungen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                              | Aktuelles Arbeitsverzeichnis                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                              | Benutzerdefinierter Pfad zur Claude Code CLI-Ausführungsdatei                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `settings`                    | `str \| None`                                                                         | `None`                              | Pfad zu einer Einstellungsdatei oder einer Inline-JSON-Zeichenkette                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                                | Zusätzliche Verzeichnisse, auf die Claude zugreifen kann. Das SDK übergibt jeden Eintrag an Claude Code als `--add-dir`, daher lädt Claude Code mit der `project`-Einstellungsquelle auch [die Skills, Befehle und Subagenten des Verzeichnisses](/docs/de/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `env`                         | `dict[str, str]`                                                                      | `{}`                                | Umgebungsvariablen, die auf der geerbten Prozessumgebung zusammengeführt werden. Siehe [Umgebungsvariablen](/docs/de/env-vars) für Variablen, die die zugrunde liegende CLI liest, und [Langsame oder steckengebliebene API-Antworten handhaben](#handle-slow-or-stalled-api-responses) für Timeout-bezogene Variablen. Setzen Sie `CLAUDE_AGENT_SDK_CLIENT_APP`, um Ihre App im User-Agent-Header zu identifizieren                                                                                                                                                                                                                                                                                                                                                                                                      |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                                | Zusätzliche CLI-Argumente, die direkt an die CLI übergeben werden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                              | Maximale Bytes beim Puffern der CLI-Stdout                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                        | *Veraltet* - Das SDK ignoriert diesen Wert. Verwenden Sie den `stderr`-Callback für CLI-stderr-Ausgabe                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                              | Callback-Funktion für stderr-Ausgabe von CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                              | Tool-Berechtigungs-Callback, der nur aufgerufen wird, wenn der [Berechtigungsfluss](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) zu einer Aufforderung führt. Nicht aufgerufen für Aufrufe, die automatisch von `allowed_tools`, Allow-Regeln oder `permission_mode` genehmigt werden. Eine Allow-Regel genehmigt nicht vorab die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves). Siehe [`CanUseTool`](#canusetool) für Details                                                                                                                                                                                                                                                                                                                    |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                              | Hook-Konfigurationen zum Abfangen von Ereignissen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `user`                        | `str \| None`                                                                         | `None`                              | Auf POSIX-Plattformen das OS-Benutzerkonto, unter dem der Claude Code-Subprozess läuft. Claude Code behält die Umgebung des übergeordneten Prozesses bei, einschließlich `HOME`, und läuft in `cwd`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `include_partial_messages`    | `bool`                                                                                | `False`                             | Schließen Sie partielle Nachrichtenstreaming-Ereignisse ein. Wenn aktiviert, werden [`StreamEvent`](#streamevent)-Nachrichten geliefert                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `include_hook_events`         | `bool`                                                                                | `False`                             | Schließen Sie Hook-Lebenszyklusereignisse im Nachrichtenstrom als `HookEventMessage`-Objekte ein                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `forward_subagent_text`       | `bool`                                                                                | `False`                             | Leiten Sie Subagenten-Text und Thinking-Blöcke im Nachrichtenstrom weiter. Ohne diese Option gibt Claude Code nur Subagenten-`tool_use`- und `tool_result`-Blöcke aus, aber keinen Text oder Thinking. Erfordert Python Agent SDK 0.2.140 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `fork_session`                | `bool`                                                                                | `False`                             | Wenn Sie mit `resume` fortsetzen, verzweigen Sie sich zu einer neuen Sitzungs-ID, anstatt die ursprüngliche Sitzung fortzusetzen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `resume_session_at`           | `str \| None`                                                                         | `None`                              | Beim Fortsetzen laden Sie die Konversation nur bis zu und einschließlich der Nachricht mit dieser UUID. Verwenden Sie mit `resume`, und normalerweise `fork_session`, um von einem früheren Punkt zu verzweigen. Erfordert Python Agent SDK 0.2.137 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `resume_drops_turn`           | `str \| None`                                                                         | `None`                              | UUID der Benutzereingabe, deren Umdrehung eine `resume_session_at`-Kürzung verwirft. Wenn gesetzt, weigert sich die CLI, die Wiederaufnahme durchzuführen, wenn der verworfene Bereich Einträge enthält, die nicht dieser Umdrehung zugeordnet werden können. Erfordert Python Agent SDK 0.2.137 oder später und Claude Code v2.1.223 oder später; die mit diesen SDK-Versionen gebündelte CLI erfüllt die Claude Code-Anforderung                                                                                                                                                                                                                                                                                                                                                                                   |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                              | Programmgesteuert definierte Subagenten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                                | Laden Sie benutzerdefinierte Plugins aus lokalen Pfaden. Siehe [Plugins](/docs/de/agent-sdk/plugins) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                              | Konfigurieren Sie das Sandbox-Verhalten programmgesteuert. Siehe [Sandbox-Einstellungen](#sandboxsettings) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (CLI-Standard: alle Quellen) | Kontrollieren Sie, welche Dateisystem-Einstellungen geladen werden. Übergeben Sie `[]`, um Benutzer-, Projekt- und lokale Einstellungen zu deaktivieren. Mit `skills` gesetzt und dieses Feld nicht gesetzt, werden nur Benutzer- und Projektquellen geladen. Setzen Sie `setting_sources` explizit, um lokale Einstellungen zu behalten. Endpoint-verwaltete Richtlinie wird unabhängig davon geladen; Server-verwaltete Einstellungen werden abgerufen, wenn sich die Sitzung mit einer Organisationsanmeldedaten auf einer [berechtigten Konfiguration](/docs/de/server-managed-settings#platform-availability) authentifiziert. Für Eingaben, die unabhängig von dieser Option gelesen werden, siehe [Was settingSources nicht kontrolliert](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control) |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                              | Skills, die der Sitzung zur Verfügung stehen. Übergeben Sie `"all"`, um jeden erkannten Skill zu aktivieren, oder eine Liste von Skill-Namen. Übergeben Sie nur exakte Namen. Das SDK lehnt fehlerhafte und Wildcard-Form-Namen mit einem `ValueError` ab, bevor der Claude Code-Prozess gestartet wird; diese Überprüfung erfordert Python Agent SDK 0.2.129 oder später. Wenn gesetzt, aktiviert das SDK das Skill-Tool automatisch in `allowed_tools`. Wenn Sie auch `tools` übergeben, schließen Sie `"Skill"` in diese Liste ein. Siehe [Skills](/docs/de/agent-sdk/skills)                                                                                                                                                                                                                                          |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                              | *Veraltet* - Maximale Token für Thinking-Blöcke. Verwenden Sie stattdessen `thinking`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                              | Steuert das Verhalten des erweiterten Denkens. Hat Vorrang vor `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                              | Anstrengungsstufe für die Denktiefe. Siehe [Anstrengungsstufe anpassen](/docs/de/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `session_store`               | [`SessionStore`](/docs/de/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                              | Spiegeln Sie Sitzungstranskripte zu einem externen Backend, damit jeder Host sie fortsetzen kann. Siehe [Sitzungen im externen Speicher beibehalten](/docs/de/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                         | Wann sollen gespiegelte Transkripteinträge zu `session_store` geleert werden. `"batched"` leert einmal pro Umdrehung oder wenn der Puffer voll wird; `"eager"` löst nach jedem Frame einen Hintergrund-Flush aus. Wird ignoriert, wenn `session_store` `None` ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `load_timeout_ms`             | `int`                                                                                 | `60000`                             | Pro-Aufruf-Timeout für `session_store.load()` und `list_subkeys()` während der Wiederaufnahme-Materialisierung in Millisekunden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `task_budget`                 | `TaskBudget \| None`                                                                  | `None`                              | API-seitiges Token-Budget. Wird als `output_config.task_budget` mit dem `task-budgets-2026-03-13`-Beta-Header gesendet. Übergeben Sie `{"total": <int>}`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

<h4 id="handle-slow-or-stalled-api-responses">
  Langsame oder steckengebliebene API-Antworten handhaben
</h4>

Die CLI-Subprozess liest mehrere Umgebungsvariablen, die API-Timeouts und Stall-Erkennung steuern. Übergeben Sie sie durch `ClaudeAgentOptions.env`:

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

* `API_TIMEOUT_MS`: Pro-Request-Timeout auf dem Anthropic-Client in Millisekunden. Standard `600000`. Gilt für die Hauptschleife und alle Subagenten.
* `CLAUDE_CODE_MAX_RETRIES`: Maximale API-Wiederholungen. Standard `10`, begrenzt auf `15`. Jede Wiederholung erhält sein eigenes `API_TIMEOUT_MS`-Fenster, daher ist die schlimmste Wandzeit ungefähr `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` plus Backoff. Für unbeaufsichtigte Läufe, die längere Ausfallzeiten abwarten müssen, setzen Sie [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/de/errors#tune-retry-behavior): Es wiederholt Kapazitätsfehler unbegrenzt und ab Claude Code v2.1.199 erhöht sich der Standard für andere vorübergehende Fehler auf `300` und entfernt die Obergrenze für diese Variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: Stall-Watchdog für Subagenten. Während der Stream-Watchdog an ist, ist der Standard `CLAUDE_STREAM_IDLE_TIMEOUT_MS` plus 5 Minuten, was `600000` ergibt, es sei denn, Sie erhöhen diese Variable. Mit dem Stream-Watchdog aus ist der Standard `600000`. Vor v2.1.257 war der Standard immer `600000`.

  Der Timer setzt sich bei jedem Stream-Ereignis zurück. Bei Stall bricht Claude Code den Subagenten ab und meldet den Stall dem übergeordneten Element. Für einen Hintergrund-Subagenten markiert es auch die Aufgabe als fehlgeschlagen und hängt jedes Teilergebnis an.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` mit `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: Stream-Watchdog, der die Anfrage abbricht, wenn Header angekommen sind, aber der Antwortkörper nicht mehr streamt. Der Watchdog ist standardmäßig für alle Anbieter aktiviert; setzen Sie `CLAUDE_ENABLE_STREAM_WATCHDOG=0`, um ihn zu deaktivieren. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` hat einen Standard von `300000` und ist auf dieses Minimum begrenzt. Nach dem Abbruch behandelt [Automatische Wiederholungen](/docs/de/errors#automatic-retries) das, was Claude Code tut, basierend darauf, wie weit die Antwort fortgeschritten war.

  Während der Watchdog auf eine Antwort wartet, die ein Gateway hinter `ANTHROPIC_BASE_URL` mit Keep-Alive-Pings offen hält, empfängt ein Host, der `include_partial_messages` setzt, weiterhin `ping`-[`StreamEvent`](#streamevent)-Nachrichten. Lesen Sie diese Frames als Lebenszeichen, anstatt die Sitzung bei Stille zu unterbrechen. Vor v2.1.257 stoppten die Frames 5 Minuten nach dem letzten echten Stream-Ereignis.

<h3 id="outputformat">
  `OutputFormat`
</h3>

Konfiguration für die Validierung strukturierter Ausgaben. Übergeben Sie dies als `dict` an das Feld `output_format` auf `ClaudeAgentOptions`:

```python theme={null}
# Expected dict shape for output_format
{
    "type": "json_schema",
    "schema": {...},  # Your JSON Schema definition
}
```

| Feld     | Erforderlich | Beschreibung                                          |
| :------- | :----------- | :---------------------------------------------------- |
| `type`   | Ja           | Muss `"json_schema"` für JSON-Schema-Validierung sein |
| `schema` | Ja           | JSON-Schema-Definition für Ausgabevalidierung         |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

Konfiguration für die Verwendung des Preset-System-Prompts von Claude Code mit optionalen Ergänzungen.

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
    snapshot: NotRequired[bool]
```

| Feld                       | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                           |
| :------------------------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | Ja           | Muss `"preset"` sein, um einen Preset-System-Prompt zu verwenden                                                                                                                                                                                                                                                                                                       |
| `preset`                   | Ja           | Muss `"claude_code"` sein, um den System-Prompt von Claude Code zu verwenden                                                                                                                                                                                                                                                                                           |
| `append`                   | Nein         | Zusätzliche Anweisungen, die an den Preset-System-Prompt angehängt werden                                                                                                                                                                                                                                                                                              |
| `exclude_dynamic_sections` | Nein         | Verschieben Sie sitzungsspezifischen Kontext wie Arbeitsverzeichnis, Git-Repo-Flag und Auto-Memory-Pfade aus dem System-Prompt in die erste Benutzernachricht. Verbessert die Prompt-Cache-Wiederverwendung über Benutzer und Maschinen hinweg. Siehe [System-Prompts ändern](/docs/de/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |
| `snapshot`                 | Nein         | Setzen Sie auf `False`, um den System-Prompt bei jeder Anfrage neu zu erstellen, anstatt [den Prompt wiederzuverwenden, den die Sitzung bei ihrer ersten Anfrage aufgezeichnet hat](/docs/de/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Erfordert `claude-agent-sdk` v0.2.153 oder später                                                |

<h3 id="systempromptcustom">
  `SystemPromptCustom`
</h3>

Ein benutzerdefinierter System-Prompt in Objektform, äquivalent zum Übergeben einer Zeichenkette als `system_prompt`, der auch `snapshot` setzen kann. Erfordert `claude-agent-sdk` v0.2.153 oder später.

```python theme={null}
class SystemPromptCustom(TypedDict):
    type: Literal["custom"]
    prompt: str
    snapshot: NotRequired[bool]
```

| Feld       | Erforderlich | Beschreibung                                                                                                                                         |
| :--------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`     | Ja           | Muss `"custom"` sein                                                                                                                                 |
| `prompt`   | Ja           | Der System-Prompt-Text. Wird an die CLI als Befehlszeilenargument übergeben, daher gelten die [Befehlszeilenlängenbeschränkungen](#systempromptfile) |
| `snapshot` | Nein         | Gleich wie [`SystemPromptPreset.snapshot`](#systempromptpreset), angewendet auf `prompt`                                                             |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

Konfiguration zum Laden eines benutzerdefinierten System-Prompts aus einer Datei, anstatt ihn als Zeichenkette zu übergeben. Das SDK ordnet dies dem CLI-Flag [`--system-prompt-file`](/docs/de/cli-reference#system-prompt-flags) zu. Verwenden Sie die Dateiform, wenn der Prompt groß ist: Das SDK übergibt einen Zeichenketten-`system_prompt` auf der CLI-Subprozess-argv, die OS-Befehlszeilenlängenbeschränkungen unterliegt, bevor das SDK eine API-Anfrage sendet. Auf Linux schlägt ein einzelnes Argument, das länger als ungefähr 128 KB ist, beim Prozessstart mit `Argument list too long` fehl. Unter Windows ist die gesamte Befehlszeile auf ungefähr 32 KB begrenzt, daher schlägt die Zeichenkettenform bei einem niedrigeren Schwellenwert fehl.

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| Feld   | Erforderlich | Beschreibung                                                  |
| :----- | :----------- | :------------------------------------------------------------ |
| `type` | Ja           | Muss `"file"` sein, um den Prompt von der Festplatte zu laden |
| `path` | Ja           | Pfad zu einer Datei, die den System-Prompt enthält            |

<h3 id="settingsource">
  `SettingSource`
</h3>

Steuert, welche dateisystembasierte Konfigurationsquellen das SDK Einstellungen aus lädt.

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| Wert        | Beschreibung                                                                               | Ort                           |
| :---------- | :----------------------------------------------------------------------------------------- | :---------------------------- |
| `"user"`    | Globale Benutzereinstellungen                                                              | `~/.claude/settings.json`     |
| `"project"` | Gemeinsame Projekteinstellungen (versionskontrolliert)                                     | `.claude/settings.json`       |
| `"local"`   | Lokale Projekteinstellungen, gitignored, wenn Claude Code eine Einstellung darin speichert | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Standardverhalten
</h4>

Wenn `setting_sources` weggelassen oder `None` ist und `skills` nicht gesetzt ist, lädt `query()` die gleichen Dateisystem-Einstellungen wie die Claude Code CLI: Benutzer, Projekt und lokal. Mit `skills` gesetzt, beschreibt die [`setting_sources`](#claudeagentoptions)-Zeile den aktuellen Standard. Endpoint-verwaltete Richtlinie wird in allen Fällen geladen; Server-verwaltete Einstellungen werden abgerufen, wenn sich die Sitzung mit einer Organisationsanmeldedaten auf einer [berechtigten Konfiguration](/docs/de/server-managed-settings#platform-availability) authentifiziert. Weitere Informationen finden Sie unter [Was settingSources nicht kontrolliert](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control).

<h4 id="why-use-setting_sources">
  Warum setting\_sources verwenden
</h4>

**Dateisystem-Einstellungen deaktivieren:**

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
  Im Python SDK 0.1.59 und früher wurde eine leere Liste gleich behandelt wie das Weglassen der Option, daher hatte `setting_sources=[]` keine Auswirkung auf die Deaktivierung von Dateisystem-Einstellungen. Aktualisieren Sie auf eine neuere Version, wenn Sie benötigen, dass eine leere Liste wirksam wird. Das TypeScript SDK ist nicht betroffen.
</Note>

**Nur bestimmte Einstellungsquellen laden:**

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

**SDK-only-Anwendungen:**

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

Um CLAUDE.md-Projektanweisungen zu laden, schließen Sie `"project"` in `setting_sources` ein. Siehe [System-Prompts ändern](/docs/de/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions) für die Interaktion des CLAUDE.md-Ladens mit den System-Prompt-Optionen.

<h4 id="settings-precedence">
  Einstellungspriorität
</h4>

Wenn mehrere Quellen geladen werden, werden Einstellungen mit dieser Priorität zusammengeführt (höchste zu niedrigste):

1. Lokale Einstellungen (`.claude/settings.local.json`)
2. Projekteinstellungen (`.claude/settings.json`)
3. Benutzereinstellungen (`~/.claude/settings.json`)

Programmgesteuerte Optionen wie `agents`, `allowed_tools` und `settings` überschreiben Benutzer-, Projekt- und lokale Dateisystem-Einstellungen. Verwaltete Richtlinieneinstellungen haben Vorrang vor programmgesteuerten Optionen.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Konfiguration für einen programmgesteuert definierten Subagenten.

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

| Feld              | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                |
| :---------------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | Ja           | Natürlichsprachige Beschreibung, wann dieser Agent verwendet werden sollte                                                                                                                                                                                                  |
| `prompt`          | Ja           | Der System-Prompt des Agenten                                                                                                                                                                                                                                               |
| `tools`           | Nein         | Array von zulässigen Tool-Namen. Wenn weggelassen, erbt jeden [Tool, der Subagenten zur Verfügung steht](/docs/de/sub-agents#available-tools)                                                                                                                                    |
| `disallowedTools` | Nein         | Array von Tool-Namen, die aus dem Tool-Set des Agenten entfernt werden. MCP-Server-Level-Muster werden auch akzeptiert: `mcp__server` oder `mcp__server__*` entfernt jedes Tool von diesem Server, und `mcp__*` entfernt jedes MCP-Tool von jedem Server                    |
| `model`           | Nein         | Modell-Override für diesen Agenten. Akzeptiert einen Alias wie `"sonnet"`, `"opus"`, `"haiku"` oder `"inherit"`, oder eine vollständige Modell-ID. Wenn Sie es weglassen, wählt Claude Code das Modell in der [Subagenten-Modellreihenfolge](/docs/de/sub-agents#choose-a-model) |
| `skills`          | Nein         | Liste von Skill-Namen, die beim Start in den Kontext des Agenten vorgeladen werden. Nicht aufgelistete Skills bleiben über das Skill-Tool aufrufbar                                                                                                                         |
| `memory`          | Nein         | Memory-Quelle für diesen Agenten: `"user"`, `"project"` oder `"local"`                                                                                                                                                                                                      |
| `mcpServers`      | Nein         | MCP-Server, die diesem Agenten zur Verfügung stehen. Jeder Eintrag ist ein Servername oder ein Inline-`{name: config}`-Dict                                                                                                                                                 |
| `initialPrompt`   | Nein         | Wird automatisch als erste Benutzerdrehung eingereicht, wenn dieser Agent als Haupt-Thread-Agent läuft                                                                                                                                                                      |
| `maxTurns`        | Nein         | Maximale Anzahl von Agenten-Umdrehungen, bevor der Agent stoppt                                                                                                                                                                                                             |
| `background`      | Nein         | Führen Sie diesen Agenten als nicht-blockierende Hintergrundaufgabe aus, wenn aufgerufen                                                                                                                                                                                    |
| `effort`          | Nein         | Reasoning-Anstrengungsstufe für diesen Agenten. Akzeptiert eine benannte Stufe oder eine Ganzzahl. Siehe [`EffortLevel`](#effortlevel)                                                                                                                                      |
| `permissionMode`  | Nein         | Berechtigungsmodus für die Tool-Ausführung innerhalb dieses Agenten. Die [Subagenten-Vererbungsregeln](/docs/de/agent-sdk/permissions#available-modes) entscheiden, wann er angewendet wird. Siehe [`PermissionMode`](#permissionmode)                                           |

<Note>
  `AgentDefinition`-Feldnamen verwenden camelCase, wie `disallowedTools`, `permissionMode` und `maxTurns`. Diese Namen werden direkt dem Drahtformat zugeordnet, das mit dem TypeScript SDK geteilt wird. Dies unterscheidet sich von `ClaudeAgentOptions`, das Python snake\_case für die entsprechenden Top-Level-Felder wie `disallowed_tools` und `permission_mode` verwendet. Da `AgentDefinition` eine Dataclass ist, wirft das Übergeben eines snake\_case-Schlüsselworts einen `TypeError` zur Konstruktionszeit auf.
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

Berechtigungsmodi zur Kontrolle der Tool-Ausführung.

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

Anstrengungsstufen zur Steuerung der Denktiefe.

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

Typ-Alias für Tool-Berechtigungs-Callback-Funktionen.

```python theme={null}
CanUseTool = Callable[
    [str, dict[str, Any], ToolPermissionContext], Awaitable[PermissionResult]
]
```

Der Callback empfängt:

* `tool_name`: Name des aufgerufenen Tools
* `input_data`: Die Eingabeparameter des Tools
* `context`: Ein `ToolPermissionContext` mit zusätzlichen Informationen

Gibt ein `PermissionResult` zurück (entweder `PermissionResultAllow` oder `PermissionResultDeny`).

Der Callback ist der SDK-Ersatz für die interaktive Berechtigungsaufforderung: Er wird nur aufgerufen, wenn der [Berechtigungsbewertungsfluss](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) zu einer Aufforderung führt. Tool-Aufrufe, die bereits von einem `allowed_tools`-Eintrag, einer Settings-Allow-Regel oder dem Berechtigungsmodus wie `acceptEdits` oder `bypassPermissions` genehmigt wurden, rufen ihn nie auf. Um jeden Tool-Aufruf zu kontrollieren, verwenden Sie stattdessen einen [`PreToolUse`-Hook](/docs/de/agent-sdk/hooks). Eine Allow-Regel genehmigt nicht vorab die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves); siehe [Wie Berechtigungen bewertet werden](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) für welche von ihnen den Callback erreichen und was in `dontAsk`- und `auto`-Modus passiert.

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Kontextinformationen, die an Tool-Berechtigungs-Callbacks übergeben werden.

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

| Feld              | Typ                      | Beschreibung                                                                                                                                                                                                                                                                  |
| :---------------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`          | `Any \| None`            | Reserviert für zukünftige Abort-Signal-Unterstützung                                                                                                                                                                                                                          |
| `suggestions`     | `list[PermissionUpdate]` | Berechtigungsaktualisierungsvorschläge von der CLI. Bash-Aufforderungen enthalten einen Vorschlag mit dem `localSettings`-Ziel, daher gibt das Zurückgeben in `updated_permissions` die Regel in `.claude/settings.local.json` aus und bleibt über Sitzungen hinweg bestehen. |
| `tool_use_id`     | `str \| None`            | Kennung des spezifischen Tool-Aufrufs, für den diese Aufforderung gilt. Wird immer gefüllt, wenn an `can_use_tool` geliefert                                                                                                                                                  |
| `agent_id`        | `str \| None`            | Sub-Agent-ID, wenn der Aufruf von einem Subagenten stammt; `None` für den Haupt-Agent                                                                                                                                                                                         |
| `blocked_path`    | `str \| None`            | Dateipfad, der die Berechtigungsanfrage ausgelöst hat, falls zutreffend. Zum Beispiel, wenn ein Bash-Befehl versucht, auf einen Pfad außerhalb zulässiger Verzeichnisse zuzugreifen                                                                                           |
| `decision_reason` | `str \| None`            | Grund, warum diese Berechtigungsanfrage ausgelöst wurde. Weitergeleitet von einem PreToolUse-Hook's `permissionDecisionReason`, wenn der Hook `"ask"` zurückgegeben hat                                                                                                       |
| `title`           | `str \| None`            | Vollständiger Berechtigungsaufforderungssatz, wie `Claude wants to read foo.txt`. Verwenden Sie als primären Aufforderungstext, wenn vorhanden                                                                                                                                |
| `display_name`    | `str \| None`            | Kurze Nominalphrase für die Tool-Aktion, wie `Read file`, geeignet für Schaltflächenbeschriftungen                                                                                                                                                                            |
| `description`     | `str \| None`            | Lesbare Untertitel für die Berechtigungs-UI                                                                                                                                                                                                                                   |

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Union-Typ für Berechtigungs-Callback-Ergebnisse.

```python theme={null}
PermissionResult = PermissionResultAllow | PermissionResultDeny
```

<h3 id="permissionresultallow">
  `PermissionResultAllow`
</h3>

Ergebnis, das angibt, dass der Tool-Aufruf zulässig sein sollte.

```python theme={null}
@dataclass
class PermissionResultAllow:
    behavior: Literal["allow"] = "allow"
    updated_input: dict[str, Any] | None = None
    updated_permissions: list[PermissionUpdate] | None = None
```

| Feld                  | Typ                              | Standard  | Beschreibung                                             |
| :-------------------- | :------------------------------- | :-------- | :------------------------------------------------------- |
| `behavior`            | `Literal["allow"]`               | `"allow"` | Muss "allow" sein                                        |
| `updated_input`       | `dict[str, Any] \| None`         | `None`    | Geänderte Eingabe, die stattdessen verwendet werden soll |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None`    | Berechtigungsaktualisierungen zum Anwenden               |

<h3 id="permissionresultdeny">
  `PermissionResultDeny`
</h3>

Ergebnis, das angibt, dass der Tool-Aufruf verweigert werden sollte.

```python theme={null}
@dataclass
class PermissionResultDeny:
    behavior: Literal["deny"] = "deny"
    message: str = ""
    interrupt: bool = False
```

| Feld        | Typ               | Standard | Beschreibung                                            |
| :---------- | :---------------- | :------- | :------------------------------------------------------ |
| `behavior`  | `Literal["deny"]` | `"deny"` | Muss "deny" sein                                        |
| `message`   | `str`             | `""`     | Nachricht, die erklärt, warum das Tool verweigert wurde |
| `interrupt` | `bool`            | `False`  | Ob die aktuelle Ausführung unterbrochen werden soll     |

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Konfiguration zum programmgesteuerten Aktualisieren von Berechtigungen.

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

| Feld          | Typ                                       | Beschreibung                                              |
| :------------ | :---------------------------------------- | :-------------------------------------------------------- |
| `type`        | `Literal[...]`                            | Der Typ der Berechtigungsaktualisierungsoperation         |
| `rules`       | `list[PermissionRuleValue] \| None`       | Regeln für Add/Replace/Remove-Operationen                 |
| `behavior`    | `Literal["allow", "deny", "ask"] \| None` | Verhalten für regelbasierte Operationen                   |
| `mode`        | `PermissionMode \| None`                  | Modus für setMode-Operation                               |
| `directories` | `list[str] \| None`                       | Verzeichnisse für Add/Remove-Verzeichnis-Operationen      |
| `destination` | `Literal[...] \| None`                    | Wo die Berechtigungsaktualisierung angewendet werden soll |

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

Eine Regel, die in einer Berechtigungsaktualisierung hinzugefügt, ersetzt oder entfernt werden soll.

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

<h3 id="toolspreset">
  `ToolsPreset`
</h3>

Preset-Tools-Konfiguration für die Verwendung des Standard-Tool-Sets von Claude Code.

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Steuert das Verhalten des erweiterten Denkens. Eine Union von drei Konfigurationen:

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

| Variante   | Felder                             | Beschreibung                                                |
| :--------- | :--------------------------------- | :---------------------------------------------------------- |
| `adaptive` | `type`, `display`                  | Claude entscheidet adaptiv, wann gedacht werden soll        |
| `enabled`  | `type`, `budget_tokens`, `display` | Aktivieren Sie das Denken mit einem bestimmten Token-Budget |
| `disabled` | `type`                             | Deaktivieren Sie das Denken                                 |

Das optionale Feld `display` steuert, ob Thinking-Text `"summarized"` oder `"omitted"` zurückgegeben wird. Bei Claude Opus 4.7 und später ist der API-Standard `"omitted"`, daher setzen Sie `"summarized"`, um Thinking-Inhalte in [`ThinkingBlock`](#thinkingblock)-Ausgaben zu erhalten. Claude Code sendet `display` nicht an Amazon Bedrock oder Google Cloud's Agent Platform, daher geben Opus 4.7 und später auf diesen Anbietern leere `ThinkingBlock`-Ausgaben zurück, auch wenn Sie `display` auf `"summarized"` setzen.

Da dies `TypedDict`-Klassen sind, sind sie zur Laufzeit einfache Dicts. Konstruieren Sie sie entweder als Dict-Literale oder rufen Sie die Klasse wie einen Konstruktor auf; beide erzeugen ein `dict`. Greifen Sie auf Felder mit `config["budget_tokens"]` zu, nicht mit `config.budget_tokens`:

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

API-seitiges Task-Budget in Token, verwendet mit dem Feld `task_budget` in `ClaudeAgentOptions`.

```python theme={null}
class TaskBudget(TypedDict):
    total: int
```

| Feld    | Typ   | Beschreibung                          |
| :------ | :---- | :------------------------------------ |
| `total` | `int` | Gesamtes Token-Budget für die Aufgabe |

Da dies eine `TypedDict` ist, übergeben Sie sie als einfaches Dict, wie `ClaudeAgentOptions(task_budget={"total": 50000})`.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Literal-Typ für SDK-Beta-Funktionen.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Verwenden Sie mit dem Feld `betas` in `ClaudeAgentOptions`, um Beta-Funktionen zu aktivieren.

<Warning>
  Die `context-1m-2025-08-07`-Beta ist seit dem 30. April 2026 veraltet. Das Übergeben dieses Headers mit Claude Sonnet 4.5 oder Sonnet 4 hat keine Auswirkung, und Anfragen, die das Standard-200k-Token-Kontextfenster überschreiten, geben einen Fehler zurück. Um ein 1M-Token-Kontextfenster zu verwenden, migrieren Sie zu [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 oder Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), die 1M-Kontext zu Standardpreisen ohne Beta-Header enthalten.
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

Konfiguration für SDK MCP-Server, die mit `create_sdk_mcp_server()` erstellt wurden.

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Union-Typ für MCP-Server-Konfigurationen.

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

Die Konfiguration eines MCP-Servers, wie von [`get_mcp_status()`](#methods) gemeldet. Dies ist die Union aller [`McpServerConfig`](#mcpserverconfig)-Transport-Varianten plus eine nur-Ausgabe-`claudeai-proxy`-Variante für Server, die durch claude.ai proxiert werden.

```python theme={null}
McpServerStatusConfig = (
    McpStdioServerConfig
    | McpSSEServerConfig
    | McpHttpServerConfig
    | McpSdkServerConfigStatus
    | McpClaudeAIProxyServerConfig
)
```

`McpSdkServerConfigStatus` ist die serialisierbare Form von [`McpSdkServerConfig`](#mcpsdkserverconfig) mit nur `type` (`"sdk"`) und `name` (`str`)-Feldern; die In-Process-`instance` wird weggelassen. `McpClaudeAIProxyServerConfig` hat `type` (`"claudeai-proxy"`), `url` (`str`) und `id` (`str`)-Felder.

<h3 id="mcpstatusresponse">
  `McpStatusResponse`
</h3>

Antwort von [`ClaudeSDKClient.get_mcp_status()`](#methods). Umhüllt die Liste der Server-Status unter dem `mcpServers`-Schlüssel.

```python theme={null}
class McpStatusResponse(TypedDict):
    mcpServers: list[McpServerStatus]
```

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Status eines verbundenen MCP-Servers, enthalten in [`McpStatusResponse`](#mcpstatusresponse).

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

| Feld         | Typ                                                          | Beschreibung                                                                                                                                                                                |
| :----------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`       | `str`                                                        | Servername                                                                                                                                                                                  |
| `status`     | `str`                                                        | Einer von `"connected"`, `"failed"`, `"needs-auth"`, `"pending"` oder `"disabled"`                                                                                                          |
| `serverInfo` | `dict` (optional)                                            | Servername und Version (`{"name": str, "version": str}`)                                                                                                                                    |
| `error`      | `str` (optional)                                             | Fehlermeldung, wenn der Server keine Verbindung herstellen konnte                                                                                                                           |
| `config`     | [`McpServerStatusConfig`](#mcpserverstatusconfig) (optional) | Server-Konfiguration. Gleiche Form wie [`McpServerConfig`](#mcpserverconfig) (stdio, SSE, HTTP oder SDK), plus eine `claudeai-proxy`-Variante für Server, die über claude.ai verbunden sind |
| `scope`      | `str` (optional)                                             | Konfigurationsbereich                                                                                                                                                                       |
| `tools`      | `list` (optional)                                            | Tools, die von diesem Server bereitgestellt werden, jeweils mit `name`, `description` und `annotations`-Feldern                                                                             |

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Konfiguration zum Laden von Plugins im SDK.

```python theme={null}
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| Feld   | Typ                | Beschreibung                                                        |
| :----- | :----------------- | :------------------------------------------------------------------ |
| `type` | `Literal["local"]` | Muss `"local"` sein (derzeit werden nur lokale Plugins unterstützt) |
| `path` | `str`              | Absoluter oder relativer Pfad zum Plugin-Verzeichnis                |

**Beispiel:**

```python theme={null}
plugins = [
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"},
]
```

Vollständige Informationen zum Erstellen und Verwenden von Plugins finden Sie unter [Plugins](/docs/de/agent-sdk/plugins).

<h2 id="message-types">
  Nachrichtentypen
</h2>

<h3 id="message">
  `Message`
</h3>

Union-Typ aller möglichen Nachrichten.

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

Benutzereingabe-Nachricht.

```python theme={null}
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
    uuid: str | None = None
    parent_tool_use_id: str | None = None
    tool_use_result: dict[str, Any] | None = None
    origin: MessageOrigin | None = None
```

| Feld                 | Typ                         | Beschreibung                                                                                                                                                                                                   |
| :------------------- | :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content`            | `str \| list[ContentBlock]` | Nachrichteninhalt als Text oder Inhaltsblöcke                                                                                                                                                                  |
| `uuid`               | `str \| None`               | Eindeutige Nachrichtenkennung                                                                                                                                                                                  |
| `parent_tool_use_id` | `str \| None`               | Tool-Use-ID, wenn diese Nachricht eine Tool-Ergebnis-Antwort ist                                                                                                                                               |
| `tool_use_result`    | `dict[str, Any] \| None`    | Tool-Ergebnisdaten, falls zutreffend                                                                                                                                                                           |
| `origin`             | `MessageOrigin \| None`     | Herkunft dieser Nachricht, gefüllt bei eingefügten Umdrehungen wie Task-Benachrichtigungen und Peer-Nachrichten. `None`, wenn die CLI sie nicht zugeordnet hat. Erfordert Python Agent SDK 0.2.137 oder später |

Das SDK übergibt `tool_use_result` unverändert von der CLI durch. Für ein Tool auf einem externen MCP-Server, dessen Ergebnis `resource_link`-Blöcke enthält, hat das Dict einen `resourceLinks`-Schlüssel, der eine Liste von Dicts mit den Schlüsseln des TypeScript-Typs [`SDKMcpResourceLink`](/docs/de/agent-sdk/typescript#sdkmcpresourcelink) enthält. Claude empfängt jeden Link als eine Textzeile im Tool-Ergebnis. Um die vom Server zurückgegebenen Dateien zu rendern, lesen Sie `resourceLinks` statt diesen Text zu analysieren. Der `resourceLinks`-Schlüssel erfordert Python Agent SDK 0.2.150 oder später und Claude Code v2.1.257 oder später; die mit dieser SDK-Version gebündelte CLI erfüllt die Claude-Code-Anforderung.

Die CLI lässt den Schlüssel weg, wenn das Ergebnis keine Links hat und bei Ergebnissen von Subagenten. Die CLI behält höchstens 50 Links pro Ergebnis und stoppt das Hinzufügen von Links, sobald die Liste 64 KiB serialisiertes JSON erreicht. Ein Tool, das Sie in-process mit [`tool()`](#tool) definieren, erzeugt niemals den Schlüssel, da das SDK seine `resource_link`-Blöcke zu Text vereinfacht, bevor die CLI das Ergebnis sieht.

<h3 id="assistantmessage">
  `AssistantMessage`
</h3>

Assistent-Antwortnachricht mit Inhaltsblöcken.

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

| Feld                 | Typ                                                          | Beschreibung                                                                                |
| :------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------------ |
| `content`            | `list[ContentBlock]`                                         | Liste von Inhaltsblöcken in der Antwort                                                     |
| `model`              | `str`                                                        | Modell, das die Antwort generiert hat                                                       |
| `parent_tool_use_id` | `str \| None`                                                | Tool-Use-ID, wenn dies eine verschachtelte Antwort ist                                      |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Fehlertyp, wenn die Antwort auf einen Fehler stieß                                          |
| `usage`              | `dict[str, Any] \| None`                                     | Token-Nutzung pro Nachricht (gleiche Schlüssel wie [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | API-Nachrichtenkennung. Mehrere Nachrichten aus einer Umdrehung teilen die gleiche ID       |
| `stop_reason`        | `str \| None`                                                | Stop-Grund von der API (z. B. `end_turn`, `tool_use`)                                       |
| `session_id`         | `str \| None`                                                | ID der Sitzung, zu der diese Nachricht gehört                                               |
| `uuid`               | `str \| None`                                                | Eindeutige Nachrichtenkennung innerhalb des Sitzungstranskripts                             |

<h3 id="assistantmessageerror">
  `AssistantMessageError`
</h3>

Mögliche Fehlertypen für Assistent-Nachrichten.

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

Der zugrunde liegende CLI-Prozess kann Fehlertypen ausgeben, die dieses Literal nicht auflistet, wie z. B. `max_output_tokens`. Das SDK übergibt den Wert unverändert durch, daher behandeln Sie Strings außerhalb dieser Liste wie `unknown`. Der TypeScript-Typ [`SDKAssistantMessageError`](/docs/de/agent-sdk/typescript#sdkassistantmessage) listet den vollständigen Satz von Werten auf, die die CLI ausgeben kann.

<h3 id="systemmessage">
  `SystemMessage`
</h3>

System-Nachricht mit Metadaten.

```python theme={null}
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

<h3 id="resultmessage">
  `ResultMessage`
</h3>

Endgültige Ergebnis-Nachricht mit Kosten- und Nutzungsinformationen.

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

Das Feld `subtype` bestimmt, welche anderen Felder gefüllt werden. Es ist eines von `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"` oder `"error_max_structured_output_retries"`. Die Python-Dataclass vereinfacht alle Varianten in eine Form, daher sind Felder, die nicht auf den zurückgegebenen Subtyp zutreffen, `None`.

Mehrere Felder enthalten diagnostische Details, wie das Gespräch endete:

* `is_error`: `True`, wenn das Gespräch in einem Fehlerzustand endete. Immer `True` bei den `error_*`-Subtypen. Bei `subtype="success"` ist es `True`, wenn die letzte Modellanfrage fehlgeschlagen ist, was bedeutet, dass die Agent-Schleife abgeschlossen wurde, aber der letzte API-Aufruf einen Fehler zurückgab.
* `api_error_status`: Der HTTP-Statuscode des beendenden API-Fehlers. `None`, wenn die Umdrehung ohne einen endete. Wird nur bei `subtype="success"` gefüllt.
* `result`: Text der endgültigen Assistent-Nachricht bei `subtype="success"` oder `None` bei den `error_*`-Subtypen. Wenn `subtype="success"` und `is_error=True`, enthält dies die API-Fehlerzeichenfolge, falls verfügbar, kann aber leer sein. Überprüfen Sie daher `api_error_status` und den vorherigen `AssistantMessage`-Inhalt für Details.
* `errors`: Fehlerzeichenfolgen auf Schleifenebene, wie die Max-Turns-Nachricht. Wird nur bei den `error_*`-Subtypen gefüllt.
* `terminal_reason`: Warum die Abfrage-Schleife endete, z. B. `"completed"`, `"max_turns"`, `"api_error"`, `"aborted_streaming"` oder `"aborted_tools"`. Ein Wert von `"aborted_streaming"` oder `"aborted_tools"` bedeutet, dass die Umdrehung vor Abschluss abgebrochen wurde. Häufige Ursachen sind [`interrupt()`](#claudesdkclient) und ein Berechtigungsrückruf, der [`PermissionResultDeny`](#permissionresultdeny) mit `interrupt=True` zurückgibt. `None` bei CLI-Versionen, die dem Feld vorausgehen, bei Ergebnissen von lokalen Befehlen wie `/voice` oder `/usage`, die die Abfrage-Schleife umgehen, oder bei synthetisierten Fehlerergebnissen, die ausgegeben werden, wenn die Sitzung fatal fehlschlägt. Spiegelt den TypeScript SDK-Typ [`SDKResultMessage.terminal_reason`](/docs/de/agent-sdk/typescript#sdkresultmessage) wider, der den vollständigen Satz von Werten auflistet.
* `origin`: Herkunft der Benutzernachricht, die diese Umdrehung ausgelöst hat. Im [Streaming-Eingabemodus](/docs/de/agent-sdk/streaming-vs-single-mode) überprüfen Sie dies, um das Ergebnis Ihrer eigenen Eingabeaufforderung zu unterscheiden, wobei `origin` `None` oder `{"kind": "human"}` ist, vom Ergebnis einer eingefügten Umdrehung wie einer Hintergrund-Task-Benachrichtigung. Erfordert Python Agent SDK 0.2.137 oder später.

Das `usage`-Dict deckt nur die Haupt-Agent-Schleife ab und schließt Subagenten und andere verschachtelte oder Hilfs-Modellaufrufe aus. Im [Streaming-Eingabemodus](/docs/de/agent-sdk/streaming-vs-single-mode) sind die Werte pro Umdrehung. Bevorzugen Sie `model_usage` für Token- und Kostenabrechnung. Das `usage`-Dict enthält die folgenden Schlüssel, wenn vorhanden:

| Schlüssel                     | Typ   | Beschreibung                                                                                                                                                                                                                                 |
| ----------------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | Eingabe-Token, die von der Agent-Schleife auf oberster Ebene verbraucht werden. [Subagent-Token sind nicht enthalten](/docs/de/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); verwenden Sie `model_usage` für die Gesamtbaum-Abrechnung. |
| `output_tokens`               | `int` | Ausgabe-Token, die von der Agent-Schleife auf oberster Ebene generiert werden. Subagent-Token sind nicht enthalten.                                                                                                                          |
| `cache_creation_input_tokens` | `int` | Token, die zum Erstellen neuer Cache-Einträge verwendet wurden.                                                                                                                                                                              |
| `cache_read_input_tokens`     | `int` | Token, die aus vorhandenen Cache-Einträgen gelesen wurden.                                                                                                                                                                                   |

Das `model_usage`-Dict ordnet Modellnamen der Nutzung pro Modell zu. Es deckt jeden Modellaufruf ab, der durch die Abfrage-Pipeline gemacht wird: die Hauptschleife, Subagenten und interne Aufrufe wie Komprimierung und Workflow-Agenten. Hilfsaufrufe außerhalb dieser Pipeline, wie der Berechtigungsklassifizierer und Token-Zählungsanfragen, sind von `model_usage` ausgeschlossen. Behandeln Sie `model_usage` als eine Schätzung, nicht als Abrechnungsauszug.

Im [Streaming-Eingabemodus](/docs/de/agent-sdk/streaming-vs-single-mode) sind `model_usage` und `total_cost_usd` kumulativ über Umdrehungen hinweg, daher lesen Sie das neueste Ergebnis statt über Ergebnisse zu summieren. Eine Anfrage, die eine Sitzung fortsetzt, zählt auch die [Gesamtwerte, die aus den früheren Aufrufen der Sitzung wiederhergestellt wurden](/docs/de/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Siehe [Kosten im Streaming-Eingabemodus verfolgen](/docs/de/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) für Zurückstellungen und [Gesamtwerte nach einem Sitzungsabsturz wiederherstellen](/docs/de/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) für auf Null gesetzte Ergebnisse.

Jeder Wert in `model_usage` ist ein `ModelUsage` TypedDict, importiert über `from claude_agent_sdk.types import ModelUsage`. Seine Schlüssel verwenden camelCase, da das SDK den Wert unverändert vom zugrunde liegenden CLI-Prozess übergibt und dem TypeScript-Typ [`ModelUsage`](/docs/de/agent-sdk/typescript#modelusage) entspricht:

| Schlüssel                  | Typ     | Beschreibung                                                                                                                                                                                                                                                                                                                                              |
| -------------------------- | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | Eingabe-Token für dieses Modell.                                                                                                                                                                                                                                                                                                                          |
| `outputTokens`             | `int`   | Ausgabe-Token für dieses Modell.                                                                                                                                                                                                                                                                                                                          |
| `cacheReadInputTokens`     | `int`   | Cache-Lese-Token für dieses Modell.                                                                                                                                                                                                                                                                                                                       |
| `cacheCreationInputTokens` | `int`   | Cache-Erstellungs-Token für dieses Modell.                                                                                                                                                                                                                                                                                                                |
| `webSearchRequests`        | `int`   | Websuch-Anfragen, die von diesem Modell gestellt wurden.                                                                                                                                                                                                                                                                                                  |
| `thinkingTokens`           | `int`   | Thinking-Token, die von diesem Modell generiert wurden, bereits in `outputTokens` gezählt. Nicht vorhanden, bis eine Umdrehung auf einer Claude-Code-Version läuft, die sie aufzeichnet, und nicht auf dem TypedDict deklariert, daher lesen Sie sie mit `.get()`. Erfordert Python Agent SDK 0.2.150 oder später, dessen gebündelte CLI sie aufzeichnet. |
| `costUSD`                  | `float` | Geschätzte Kosten in USD für dieses Modell, clientseitig berechnet. Siehe [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking) für Abrechnungsvorbehalt.                                                                                                                                                                                           |
| `contextWindow`            | `int`   | Kontextfenstergröße für dieses Modell.                                                                                                                                                                                                                                                                                                                    |
| `maxOutputTokens`          | `int`   | Maximale Ausgabe-Token-Grenze für dieses Modell.                                                                                                                                                                                                                                                                                                          |
| `canonicalModel`           | `str`   | Kanonische Modell-ID, die für die Preissuche verwendet wird. Kann sich vom Raw-Modell-String unterscheiden, nach dem der Eintrag verschlüsselt ist, wie eine anbieter-spezifische ID oder ein Alias. Nicht immer vorhanden.                                                                                                                               |
| `provider`                 | `str`   | API-Anbieter, der dieses Modell bereitgestellt hat, wie `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle` oder `gateway`. Nicht immer vorhanden.                                                                                                                                                                                     |

<h3 id="streamevent">
  `StreamEvent`
</h3>

Stream-Ereignis für partielle Nachrichtenaktualisierungen während des Streamings. Wird nur empfangen, wenn `include_partial_messages=True` in `ClaudeAgentOptions`. Import über `from claude_agent_sdk.types import StreamEvent`.

```python theme={null}
@dataclass
class StreamEvent:
    uuid: str
    session_id: str
    event: dict[str, Any]  # The raw Claude API stream event
    parent_tool_use_id: str | None = None
```

| Feld                 | Typ              | Beschreibung                                                                                                                                                                                    |
| :------------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `uuid`               | `str`            | Eindeutige Kennung für dieses Ereignis                                                                                                                                                          |
| `session_id`         | `str`            | Sitzungskennung                                                                                                                                                                                 |
| `event`              | `dict[str, Any]` | Die rohen Claude API-Stream-Ereignisdaten                                                                                                                                                       |
| `parent_tool_use_id` | `str \| None`    | Immer `None`. Stream-Ereignisse werden nur für die Hauptsitzung ausgegeben. Für die Zuordnung von Subagenten verwenden Sie vollständige Nachrichten wie [`AssistantMessage`](#assistantmessage) |

<h3 id="ratelimitevent">
  `RateLimitEvent`
</h3>

Wird ausgegeben, wenn sich der Rate-Limit-Status ändert (z. B. von `"allowed"` zu `"allowed_warning"`). Verwenden Sie dies, um Benutzer zu warnen, bevor sie eine harte Grenze erreichen, oder um zu backoff, wenn der Status `"rejected"` ist.

```python theme={null}
@dataclass
class RateLimitEvent:
    rate_limit_info: RateLimitInfo
    uuid: str
    session_id: str
```

| Feld              | Typ                               | Beschreibung                |
| :---------------- | :-------------------------------- | :-------------------------- |
| `rate_limit_info` | [`RateLimitInfo`](#ratelimitinfo) | Aktueller Rate-Limit-Status |
| `uuid`            | `str`                             | Eindeutige Ereigniskennung  |
| `session_id`      | `str`                             | Sitzungskennung             |

<h3 id="ratelimitinfo">
  `RateLimitInfo`
</h3>

Rate-Limit-Status, den [`RateLimitEvent`](#ratelimitevent) trägt.

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

| Feld                      | Typ                       | Beschreibung                                                                                                                       |
| :------------------------ | :------------------------ | :--------------------------------------------------------------------------------------------------------------------------------- |
| `status`                  | `RateLimitStatus`         | Aktueller Status. `"allowed_warning"` bedeutet, dass die Grenze näher rückt; `"rejected"` bedeutet, dass die Grenze erreicht wurde |
| `resets_at`               | `int \| None`             | Unix-Zeitstempel, wenn das Rate-Limit-Fenster zurückgesetzt wird                                                                   |
| `rate_limit_type`         | `RateLimitType \| None`   | Welches Rate-Limit-Fenster gilt                                                                                                    |
| `utilization`             | `float \| None`           | Anteil des Rate-Limits, das verbraucht wurde (0,0 bis 1,0)                                                                         |
| `overage_status`          | `RateLimitStatus \| None` | Status der Pay-as-you-go-Übernutzung, falls zutreffend                                                                             |
| `overage_resets_at`       | `int \| None`             | Unix-Zeitstempel, wenn das Übernutzungs-Fenster zurückgesetzt wird                                                                 |
| `overage_disabled_reason` | `str \| None`             | Warum Übernutzung nicht verfügbar ist, wenn Status `"rejected"` ist                                                                |
| `raw`                     | `dict[str, Any]`          | Vollständiges Rohdictionary von der CLI, einschließlich Felder, die oben nicht modelliert sind                                     |

<h3 id="conversationresetmessage">
  `ConversationResetMessage`
</h3>

Wird ausgegeben, wenn das Gespräch ersetzt wird, ohne die Verbindung zu beenden, z. B. nach `/clear`. Siehe [Kosten im Streaming-Eingabemodus verfolgen](/docs/de/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) für die Auswirkung eines Zurückstellens auf die laufenden Gesamtwerte bei späteren `ResultMessage`-Objekten. Erfordert Python Agent SDK 0.2.137 oder später.

```python theme={null}
@dataclass
class ConversationResetMessage:
    new_conversation_id: str
    uuid: str
    session_id: str
```

| Feld                  | Typ   | Beschreibung                                                                                                                                    |
| :-------------------- | :---- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `new_conversation_id` | `str` | Undurchsichtige Kennung für das neue Gespräch. Nicht die `session_id` der nachfolgenden Nachrichten; lesen Sie diese aus der nächsten Nachricht |
| `uuid`                | `str` | Eindeutige Nachrichtenkennung                                                                                                                   |
| `session_id`          | `str` | ID der Sitzung, die zurückgesetzt wurde. Nachrichten nach dem Zurücksetzen tragen eine neue `session_id`                                        |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

Wird ausgegeben, wenn eine Hintergrundaufgabe startet. Eine Hintergrundaufgabe ist alles, was außerhalb der Hauptumdrehung verfolgt wird: ein backgroundierter Bash-Befehl, eine [Monitor](#monitor)-Überwachung, ein Subagent, der über das Agent-Tool erzeugt wird, oder ein Remote-Agent. Das Feld `task_type` sagt Ihnen, welches. Diese Benennung ist nicht verwandt mit der `Task`-zu-`Agent`-Tool-Umbenennung.

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

| Feld          | Typ           | Beschreibung                                                                                                                           |
| :------------ | :------------ | :------------------------------------------------------------------------------------------------------------------------------------- |
| `task_id`     | `str`         | Eindeutige Kennung für die Aufgabe                                                                                                     |
| `description` | `str`         | Beschreibung der Aufgabe                                                                                                               |
| `uuid`        | `str`         | Eindeutige Nachrichtenkennung                                                                                                          |
| `session_id`  | `str`         | Sitzungskennung                                                                                                                        |
| `tool_use_id` | `str \| None` | Zugeordnete Tool-Use-ID                                                                                                                |
| `task_type`   | `str \| None` | Welche Art von Hintergrundaufgabe: `"local_bash"` für Background Bash und Monitor-Überwachungen, `"local_agent"` oder `"remote_agent"` |

<h3 id="taskusage">
  `TaskUsage`
</h3>

Token- und Timing-Daten für eine Hintergrundaufgabe.

```python theme={null}
class TaskUsage(TypedDict):
    total_tokens: int
    tool_uses: int
    duration_ms: int
```

<h3 id="taskprogressmessage">
  `TaskProgressMessage`
</h3>

Wird regelmäßig mit Fortschrittsaktualisierungen für eine laufende Hintergrundaufgabe ausgegeben.

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

| Feld             | Typ           | Beschreibung                                          |
| :--------------- | :------------ | :---------------------------------------------------- |
| `task_id`        | `str`         | Eindeutige Kennung für die Aufgabe                    |
| `description`    | `str`         | Aktuelle Statusbeschreibung                           |
| `usage`          | `TaskUsage`   | Token-Nutzung für diese Aufgabe bisher                |
| `uuid`           | `str`         | Eindeutige Nachrichtenkennung                         |
| `session_id`     | `str`         | Sitzungskennung                                       |
| `tool_use_id`    | `str \| None` | Zugeordnete Tool-Use-ID                               |
| `last_tool_name` | `str \| None` | Name des letzten Tools, das die Aufgabe verwendet hat |

<h3 id="tasknotificationmessage">
  `TaskNotificationMessage`
</h3>

Wird ausgegeben, wenn eine Hintergrundaufgabe abgeschlossen, fehlgeschlagen oder gestoppt wird. Hintergrundaufgaben umfassen `run_in_background`-Bash-Befehle, Monitor-Überwachungen und Background-Subagenten.

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

| Feld          | Typ                      | Beschreibung                                         |
| :------------ | :----------------------- | :--------------------------------------------------- |
| `task_id`     | `str`                    | Eindeutige Kennung für die Aufgabe                   |
| `status`      | `TaskNotificationStatus` | Einer von `"completed"`, `"failed"` oder `"stopped"` |
| `output_file` | `str`                    | Pfad zur Aufgabenausgabedatei                        |
| `summary`     | `str`                    | Zusammenfassung des Aufgabenergebnisses              |
| `uuid`        | `str`                    | Eindeutige Nachrichtenkennung                        |
| `session_id`  | `str`                    | Sitzungskennung                                      |
| `tool_use_id` | `str \| None`            | Zugeordnete Tool-Use-ID                              |
| `usage`       | `TaskUsage \| None`      | Endgültige Token-Nutzung für die Aufgabe             |

Wenn die CLI [einen langen MCP-Tool-Aufruf in den Hintergrund verschiebt](/docs/de/mcp#automatic-backgrounding-of-long-tool-calls), enthält das Tool-Ergebnis für diesen Aufruf nur einen Platzhalter und das echte Ergebnis des Aufrufs kommt in dieser Nachricht an. Bei einer `"completed"`-Benachrichtigung für einen solchen Aufruf fügt die CLI einen `resource_links`-Schlüssel hinzu, der die Dateien auflistet, die das Tool durch Referenz zurückgegeben hat, mit den gleichen Einträgen und Grenzen wie der `resourceLinks`-Schlüssel auf [`UserMessage.tool_use_result`](#usermessage). Der `resource_links`-Schlüssel erfordert Python Agent SDK 0.2.150 oder später und Claude Code v2.1.257 oder später; die mit dieser SDK-Version gebündelte CLI erfüllt die Claude-Code-Anforderung.

Die Dataclass hat kein Feld für `resource_links`. Lesen Sie es aus dem `data`-Dict, das die Nachricht von [`SystemMessage`](#systemmessage) erbt: `message.data.get("resource_links")`. Ordnen Sie die Benachrichtigung dem Aufruf mit `tool_use_id` zu. Die CLI lässt den Schlüssel weg, wenn das Ergebnis keine Links hatte und bei Benachrichtigungen für Aufgaben, die keine MCP-Tool-Aufrufe sind.

<h2 id="content-block-types">
  Inhaltsblock-Typen
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Union-Typ aller Inhaltsblöcke.

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

Text-Inhaltsblock.

```python theme={null}
@dataclass
class TextBlock:
    text: str
```

<h3 id="thinkingblock">
  `ThinkingBlock`
</h3>

Thinking-Inhaltsblock (für Modelle mit Thinking-Fähigkeit).

```python theme={null}
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

<h3 id="tooluseblock">
  `ToolUseBlock`
</h3>

Tool-Use-Anfrage-Block.

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

Tool-Ausführungs-Ergebnis-Block.

```python theme={null}
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

<h2 id="error-types">
  Fehlertypen
</h2>

Die folgenden Typen definieren, was Ihr Code abfängt. Für Einträge, die den Fehlermeldungen zugeordnet sind, die diese Typen auslösen, mit der Ursache und Behebung für jeden, siehe [Troubleshooting](/docs/de/agent-sdk/troubleshooting).

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Basis-Ausnahmeklasse für alle SDK-Fehler.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

Wenn eine einmalige `query()` mit einem Fehlerergebnis endet, beispielsweise ein Turn-Limit-Fehler, löst das SDK eine [`ResultError`](#resulterror) aus, nachdem die endgültige Ergebnismeldung ausgegeben wurde. Python Agent SDK-Versionen vor 0.2.140 lösten eine einfache `Exception` aus, die keine `ClaudeSDKError`-Unterklasse war.

<h3 id="clinotfounderror">
  `CLINotFoundError`
</h3>

Wird ausgelöst, wenn Claude Code CLI nicht installiert oder nicht gefunden ist.

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

Wird ausgelöst, wenn die Verbindung zu Claude Code fehlschlägt.

```python theme={null}
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

<h3 id="processerror">
  `ProcessError`
</h3>

Wird ausgelöst, wenn der Claude Code-Prozess fehlschlägt.

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

Wird ausgelöst, nachdem die endgültige [`ResultMessage`](#resultmessage) ausgegeben wurde, wenn der Claude Code-Prozess beendet wird, weil die Ausführung mit einem Fehlerergebnis endete, z. B. ein Turn-Limit-Fehler oder ein API-Fehler. `ResultError` ist eine Unterklasse von `ProcessError`, daher fängt ein vorhandener `except ProcessError`-Handler auch diesen ab. Seine Attribute enthalten die Felder dieser Ergebnismeldung, sodass Sie verzweigen können, warum die Ausführung fehlgeschlagen ist, ohne den Meldungstext zu analysieren. Erfordert Python Agent SDK 0.2.140 oder später.

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

Um Fehler zu unterscheiden, überprüfen Sie `terminal_reason` vor `subtype`. Wenn die endgültige Anfrage fehlschlägt, z. B. bei einem API-Fehler, meldet Claude Code `subtype` `"success"` mit der Ursache in `terminal_reason`, z. B. `"api_error"`; wenn ein von Ihnen festgelegtes Limit die Ausführung beendet, z. B. `max_turns` oder `max_budget_usd`, meldet es einen `error_*`-Subtyp.

<h3 id="clijsondecodeerror">
  `CLIJSONDecodeError`
</h3>

Wird ausgelöst, wenn JSON-Parsing fehlschlägt.

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
  Hook-Typen
</h2>

Einen umfassenden Leitfaden zur Verwendung von Hooks mit Beispielen und häufigen Mustern finden Sie im [Hooks-Leitfaden](/docs/de/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Unterstützte Hook-Ereignistypen.

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
  Das TypeScript SDK unterstützt zusätzliche Hook-Ereignisse, die in Python noch nicht verfügbar sind. Siehe die [Hook-Verfügbarkeitstabelle](/docs/de/agent-sdk/hooks#available-hooks) für SDK-spezifische Unterstützung.
</Note>

<h3 id="hookcallback">
  `HookCallback`
</h3>

Typ-Definition für Hook-Callback-Funktionen.

```python theme={null}
HookCallback = Callable[[HookInput, str | None, HookContext], Awaitable[HookJSONOutput]]
```

Parameter:

* `input`: Stark typisierte Hook-Eingabe mit diskriminierten Unions basierend auf `hook_event_name` (siehe [`HookInput`](#hookinput))
* `tool_use_id`: Optionale Tool-Use-Kennung (für Tool-bezogene Hooks)
* `context`: Hook-Kontext mit zusätzlichen Informationen

Gibt ein [`HookJSONOutput`](#hookjsonoutput) zurück.

<h3 id="hookcontext">
  `HookContext`
</h3>

Kontextinformationen, die an Hook-Callbacks übergeben werden.

```python theme={null}
class HookContext(TypedDict):
    signal: Any | None  # Future: abort signal support
```

<h3 id="hookmatcher">
  `HookMatcher`
</h3>

Konfiguration zum Abgleichen von Hooks mit bestimmten Ereignissen oder Tools.

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

Union-Typ aller Hook-Eingabetypen. Der tatsächliche Typ hängt vom Feld `hook_event_name` ab.

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

Basis-Felder, die in allen Hook-Eingabetypen vorhanden sind.

```python theme={null}
class BaseHookInput(TypedDict):
    session_id: str
    transcript_path: str
    cwd: str
    permission_mode: NotRequired[str]
```

| Feld              | Typ              | Beschreibung                      |
| :---------------- | :--------------- | :-------------------------------- |
| `session_id`      | `str`            | Aktuelle Sitzungskennung          |
| `transcript_path` | `str`            | Pfad zur Sitzungstranskript-Datei |
| `cwd`             | `str`            | Aktuelles Arbeitsverzeichnis      |
| `permission_mode` | `str` (optional) | Aktueller Berechtigungsmodus      |

<h3 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h3>

Eingabedaten für `PreToolUse`-Hook-Ereignisse.

```python theme={null}
class PreToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PreToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Feld              | Typ                     | Beschreibung                                                                           |
| :---------------- | :---------------------- | :------------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PreToolUse"]` | Immer "PreToolUse"                                                                     |
| `tool_name`       | `str`                   | Name des Tools, das ausgeführt werden soll                                             |
| `tool_input`      | `dict[str, Any]`        | Eingabeparameter für das Tool                                                          |
| `tool_use_id`     | `str`                   | Eindeutige Kennung für diese Tool-Nutzung                                              |
| `agent_id`        | `str` (optional)        | Subagenten-Kennung, vorhanden, wenn der Hook innerhalb eines Subagenten ausgelöst wird |
| `agent_type`      | `str` (optional)        | Subagenten-Typ, vorhanden, wenn der Hook innerhalb eines Subagenten ausgelöst wird     |

<h3 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h3>

Eingabedaten für `PostToolUse`-Hook-Ereignisse.

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

| Feld              | Typ                      | Beschreibung                                                                           |
| :---------------- | :----------------------- | :------------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUse"]` | Immer "PostToolUse"                                                                    |
| `tool_name`       | `str`                    | Name des Tools, das ausgeführt wurde                                                   |
| `tool_input`      | `dict[str, Any]`         | Eingabeparameter, die verwendet wurden                                                 |
| `tool_response`   | `Any`                    | Antwort aus der Tool-Ausführung                                                        |
| `tool_use_id`     | `str`                    | Eindeutige Kennung für diese Tool-Nutzung                                              |
| `agent_id`        | `str` (optional)         | Subagenten-Kennung, vorhanden, wenn der Hook innerhalb eines Subagenten ausgelöst wird |
| `agent_type`      | `str` (optional)         | Subagenten-Typ, vorhanden, wenn der Hook innerhalb eines Subagenten ausgelöst wird     |

<h3 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h3>

Eingabedaten für `PostToolUseFailure`-Hook-Ereignisse. Wird aufgerufen, wenn eine Tool-Ausführung fehlschlägt.

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

| Feld              | Typ                             | Beschreibung                                                                                                                                                                                                                                       |
| :---------------- | :------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Immer "PostToolUseFailure"                                                                                                                                                                                                                         |
| `tool_name`       | `str`                           | Name des Tools, das fehlgeschlagen ist                                                                                                                                                                                                             |
| `tool_input`      | `dict[str, Any]`                | Eingabeparameter, die verwendet wurden                                                                                                                                                                                                             |
| `tool_use_id`     | `str`                           | Eindeutige Kennung für diese Tool-Nutzung                                                                                                                                                                                                          |
| `error`           | `str`                           | Fehlermeldung aus der fehlgeschlagenen Ausführung                                                                                                                                                                                                  |
| `is_interrupt`    | `bool` (optional)               | True, wenn der Fehler Claude Code als Abbruch erreichte, anstatt als Fehler, den das Tool gemeldet hat. Das Abbrechen eines laufenden Tools mit `interrupt()` löst diesen Hook nicht aus; das Tool-Ergebnis enthält stattdessen die Abbruchmeldung |
| `agent_id`        | `str` (optional)                | Subagenten-Kennung, vorhanden, wenn der Hook innerhalb eines Subagenten ausgelöst wird                                                                                                                                                             |
| `agent_type`      | `str` (optional)                | Subagenten-Typ, vorhanden, wenn der Hook innerhalb eines Subagenten ausgelöst wird                                                                                                                                                                 |

<h3 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h3>

Eingabedaten für `UserPromptSubmit`-Hook-Ereignisse.

```python theme={null}
class UserPromptSubmitHookInput(BaseHookInput):
    hook_event_name: Literal["UserPromptSubmit"]
    prompt: str
```

| Feld              | Typ                           | Beschreibung                               |
| :---------------- | :---------------------------- | :----------------------------------------- |
| `hook_event_name` | `Literal["UserPromptSubmit"]` | Immer "UserPromptSubmit"                   |
| `prompt`          | `str`                         | Die vom Benutzer eingereichte Aufforderung |

<h3 id="stophookinput">
  `StopHookInput`
</h3>

Eingabedaten für `Stop`-Hook-Ereignisse.

```python theme={null}
class StopHookInput(BaseHookInput):
    hook_event_name: Literal["Stop"]
    stop_hook_active: bool
```

| Feld               | Typ               | Beschreibung               |
| :----------------- | :---------------- | :------------------------- |
| `hook_event_name`  | `Literal["Stop"]` | Immer "Stop"               |
| `stop_hook_active` | `bool`            | Ob der Stop-Hook aktiv ist |

<h3 id="subagentstophookinput">
  `SubagentStopHookInput`
</h3>

Eingabedaten für `SubagentStop`-Hook-Ereignisse.

```python theme={null}
class SubagentStopHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStop"]
    stop_hook_active: bool
    agent_id: str
    agent_transcript_path: str
    agent_type: str
```

| Feld                    | Typ                       | Beschreibung                             |
| :---------------------- | :------------------------ | :--------------------------------------- |
| `hook_event_name`       | `Literal["SubagentStop"]` | Immer "SubagentStop"                     |
| `stop_hook_active`      | `bool`                    | Ob der Stop-Hook aktiv ist               |
| `agent_id`              | `str`                     | Eindeutige Kennung für den Subagenten    |
| `agent_transcript_path` | `str`                     | Pfad zur Transkript-Datei des Subagenten |
| `agent_type`            | `str`                     | Typ des Subagenten                       |

<h3 id="precompacthookinput">
  `PreCompactHookInput`
</h3>

Eingabedaten für `PreCompact`-Hook-Ereignisse.

```python theme={null}
class PreCompactHookInput(BaseHookInput):
    hook_event_name: Literal["PreCompact"]
    trigger: Literal["manual", "auto"]
    custom_instructions: str | None
```

| Feld                  | Typ                         | Beschreibung                                         |
| :-------------------- | :-------------------------- | :--------------------------------------------------- |
| `hook_event_name`     | `Literal["PreCompact"]`     | Immer "PreCompact"                                   |
| `trigger`             | `Literal["manual", "auto"]` | Was die Komprimierung ausgelöst hat                  |
| `custom_instructions` | `str \| None`               | Benutzerdefinierte Anweisungen für die Komprimierung |

<h3 id="notificationhookinput">
  `NotificationHookInput`
</h3>

Eingabedaten für `Notification`-Hook-Ereignisse.

```python theme={null}
class NotificationHookInput(BaseHookInput):
    hook_event_name: Literal["Notification"]
    message: str
    title: NotRequired[str]
    notification_type: str
```

| Feld                | Typ                       | Beschreibung                       |
| :------------------ | :------------------------ | :--------------------------------- |
| `hook_event_name`   | `Literal["Notification"]` | Immer "Notification"               |
| `message`           | `str`                     | Benachrichtigungsnachrichteninhalt |
| `title`             | `str` (optional)          | Benachrichtigungstitel             |
| `notification_type` | `str`                     | Benachrichtigungstyp               |

<h3 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h3>

Eingabedaten für `SubagentStart`-Hook-Ereignisse.

```python theme={null}
class SubagentStartHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStart"]
    agent_id: str
    agent_type: str
```

| Feld              | Typ                        | Beschreibung                          |
| :---------------- | :------------------------- | :------------------------------------ |
| `hook_event_name` | `Literal["SubagentStart"]` | Immer "SubagentStart"                 |
| `agent_id`        | `str`                      | Eindeutige Kennung für den Subagenten |
| `agent_type`      | `str`                      | Typ des Subagenten                    |

<h3 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h3>

Eingabedaten für `PermissionRequest`-Hook-Ereignisse. Ermöglicht Hooks, Berechtigungsentscheidungen programmgesteuert zu handhaben.

```python theme={null}
class PermissionRequestHookInput(BaseHookInput):
    hook_event_name: Literal["PermissionRequest"]
    tool_name: str
    tool_input: dict[str, Any]
    permission_suggestions: NotRequired[list[Any]]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Feld                     | Typ                            | Beschreibung                                                                           |
| :----------------------- | :----------------------------- | :------------------------------------------------------------------------------------- |
| `hook_event_name`        | `Literal["PermissionRequest"]` | Immer "PermissionRequest"                                                              |
| `tool_name`              | `str`                          | Name des Tools, das Berechtigung anfordert                                             |
| `tool_input`             | `dict[str, Any]`               | Eingabeparameter für das Tool                                                          |
| `permission_suggestions` | `list[Any]` (optional)         | Vorgeschlagene Berechtigungsaktualisierungen von der CLI                               |
| `agent_id`               | `str` (optional)               | Subagenten-Kennung, vorhanden, wenn der Hook innerhalb eines Subagenten ausgelöst wird |
| `agent_type`             | `str` (optional)               | Subagenten-Typ, vorhanden, wenn der Hook innerhalb eines Subagenten ausgelöst wird     |

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Union-Typ für Hook-Callback-Rückgabewerte.

```python theme={null}
HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

Synchrone Hook-Ausgabe mit Kontroll- und Entscheidungsfeldern.

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
  Verwenden Sie `continue_` (mit Unterstrich) im Python-Code. Es wird automatisch in `continue` konvertiert, wenn es an die CLI gesendet wird.
</Note>

<h4 id="hookspecificoutput">
  `HookSpecificOutput`
</h4>

Eine diskriminierte Union von ereignisspezifischen `TypedDict`-Ausgabetypen. Das Feld `hookEventName` bestimmt, welche Felder gültig sind. Vollständige Details zu verfügbaren Feldern pro Hook-Ereignis finden Sie unter [Ausführung mit Hooks kontrollieren](/docs/de/agent-sdk/hooks#outputs).

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

Asynchrone Hook-Ausgabe, die Hook-Ausführung aufschiebt.

```python theme={null}
class AsyncHookJSONOutput(TypedDict):
    async_: Literal[True]  # Set to True to defer execution
    asyncTimeout: NotRequired[int]  # Timeout in milliseconds
```

<Note>
  Verwenden Sie `async_` (mit Unterstrich) im Python-Code. Es wird automatisch in `async` konvertiert, wenn es an die CLI gesendet wird.
</Note>

<h3 id="hook-usage-example">
  Hook-Verwendungsbeispiel
</h3>

Dieses Beispiel registriert zwei Hooks: einen, der gefährliche Bash-Befehle wie `rm -rf /` blockiert, und einen anderen, der alle Tool-Nutzung für Auditing protokolliert. Der Sicherheits-Hook wird nur auf Bash-Befehle ausgeführt (über den `matcher`), während der Logging-Hook auf alle Tools angewendet wird.

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
  Tool-Eingabe-/Ausgabetypen
</h2>

Dokumentation von Eingabe-/Ausgabeschemas für alle integrierten Claude Code-Tools. Während das Python SDK diese nicht als Typen exportiert, stellen sie die Struktur von Tool-Eingaben und -Ausgaben in Nachrichten dar.

<h3 id="agent">
  Agent
</h3>

**Tool-Name:** `Agent`. Der frühere Name `Task` wird immer noch als Alias akzeptiert, und die `tools`-Liste in der Init-[`SystemMessage`](#systemmessage) meldet dieses Tool als `Task` für Rückwärtskompatibilität.

**Eingabe:**

```python theme={null}
{
    "description": str,  # Eine kurze (3-5 Wörter) Beschreibung der Aufgabe
    "prompt": str,  # Die Aufgabe, die der Agent ausführen soll
    "subagent_type": str | None,  # Der Typ des spezialisierten Agenten, der verwendet werden soll
    "model": "sonnet" | "opus" | "haiku" | "fable" | None,  # Modellüberschreibung für diesen Agent
    "run_in_background": bool | None,  # Agenten laufen standardmäßig im Hintergrund; auf False setzen, um synchron auszuführen
    "name": str | None,  # Name für den erzeugten Agent
    "team_name": str | None,  # Veraltet; wird ignoriert
    "mode": "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan" | None,  # Veraltet; wird ignoriert. Die Vererbungsregeln für Subagenten bestimmen den Berechtigungsmodus eines Subagenten
    "isolation": "worktree" | "remote" | None,  # Isolationsmodus für die Änderungen des Agenten
}
```

Startet einen neuen Agent, um komplexe, mehrstufige Aufgaben autonom zu bewältigen.

**Ausgabe (Status: `"completed"`):**

```python theme={null}
{
    "status": "completed",
    "agentId": str,  # ID des Agenten, der ausgeführt wurde
    "agentType": str | None,  # Der Subagenten-Typ, der die Aufgabe bearbeitet hat
    "content": [  # Ergebnis-Inhaltsblöcke
        {
            "type": "text",
            "text": str,
            "citations": list | None,
        }
    ],
    "resolvedModel": str | None,  # Modell, auf dem der Subagent gestartet wurde
    "modelsUsed": list[str] | None,  # Verwendete Modelle in Reihenfolge, mit zusammengefassten aufeinanderfolgenden Wiederholungen
    "totalToolUseCount": int,  # Anzahl der Tool-Aufrufe, die der Agent durchgeführt hat
    "totalDurationMs": int,  # Ausführungsdauer in Millisekunden
    "totalTokens": int,  # Token-Anzahl aus der letzten API-Anfrage, nicht aus dem gesamten Durchlauf
    "usage": {  # Token-Nutzungsstatistiken
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
    "toolStats": {  # Aggregierte Tool-Aktivität für den Durchlauf
        "readCount": int,
        "searchCount": int,
        "bashCount": int,
        "editFileCount": int,
        "linesAdded": int,
        "linesRemoved": int,
        "otherToolCount": int,
        "frameCount": int | None,
    } | None,
    "prompt": str,  # Der Prompt, den der Agent ausgeführt hat
    "worktreePath": str | None,  # Vorhanden, wenn Claude Code den Worktree des Subagenten behalten hat
    "worktreeBranch": str | None,  # Vorhanden, wenn Claude Code diesen Worktree mit Git erstellt hat
}
```

**Ausgabe (Status: `"async_launched"`):**

```python theme={null}
{
    "status": "async_launched",
    "isAsync": bool | None,  # True bei Hintergrundstarts
    "agentId": str,  # ID des gestarteten Agenten
    "description": str,  # Die Aufgabenbeschreibung
    "resolvedModel": str | None,  # Modell in Verwendung beim Hintergrund-Übergangspunkt
    "modelsUsed": list[str] | None,  # Vor dem Hintergrund verwendete Modelle in Reihenfolge, mit zusammengefassten aufeinanderfolgenden Wiederholungen
    "prompt": str,  # Der Prompt, den der Agent ausführt
    "outputFile": str,  # Dateipfad, in den die Ausgabe des Agenten geschrieben wird
    "canReadOutputFile": bool | None,  # Ob die Ausgabedatei direkt gelesen werden kann
}
```

**Ausgabe (Status: `"remote_launched"`):**

```python theme={null}
{
    "status": "remote_launched",
    "taskId": str,  # ID der versendeten Aufgabe
    "sessionUrl": str,  # Link zur Cloud-Sitzung
    "description": str,  # Die Aufgabenbeschreibung
    "prompt": str,  # Der Prompt, den der Agent ausführt
    "outputFile": str,  # Dateipfad, in den die Ausgabe des Agenten geschrieben wird
}
```

Gibt das Ergebnis vom Subagenten zurück. Die Ausgabe wird nach dem `status`-Feld diskriminiert: `"completed"` für abgeschlossene Aufgaben, `"async_launched"` für Hintergrundaufgaben und `"remote_launched"` für Aufgaben, die Claude Code an eine Cloud-Sitzung versendet hat, wobei `sessionUrl` auf diese Sitzung verweist und `taskId` sie identifiziert. Wenn Claude Code [den isolierten Worktree des Subagenten behalten hat](/docs/de/worktrees#isolate-subagents-with-worktrees), ist `worktreePath` in der `completed`-Variante der Ort, wo man ihn findet, und `worktreeBranch` ist sein Branch, wenn Claude Code den Worktree mit Git erstellt hat.

In der `completed`-Variante benennt `resolvedModel` das Modell, auf dem der Subagent gestartet wurde, das sich vom angeforderten `model`-Input unterscheiden kann, wenn [`availableModels`](/docs/de/model-config#restrict-model-selection) oder eine andere Überschreibung gilt. Dieses Feld erfordert Claude Code v2.1.174 oder später. In der `async_launched`-Variante benennt `resolvedModel` das Modell in Verwendung, wenn der Agent in den Hintergrund wechselte, sodass ein Wechsel, der vor dem Hintergrund stattfand, dort widergespiegelt wird. Das `modelsUsed`-Feld in beiden Varianten listet die verwendeten Modelle in Reihenfolge auf, mit zusammengefassten aufeinanderfolgenden Wiederholungen; es wird nur gesetzt, wenn das Modell während des Durchlaufs gewechselt wurde. `modelsUsed` und das Hintergrund-Zeit-`resolvedModel`-Verhalten erfordern Claude Code v2.1.212 oder später.

Claude Code füllt `usage` und `totalTokens` aus der letzten API-Anfrage des Subagenten, nicht aus dem gesamten Durchlauf. Wenn vorhanden, ist `thinking_tokens` unter `output_tokens_details` in `usage` die Anzahl der Ausgabe-Token dieser Anfrage, die Denk-Token waren. Der `output_tokens_details`-Schlüssel erfordert Python SDK v0.2.136 oder später, das Claude Code v2.1.228 bündelt.

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Tool-Name:** `AskUserQuestion`

Stellt dem Benutzer während der Ausführung Klärungsfragen. Siehe [Genehmigungen und Benutzereingaben handhaben](/docs/de/agent-sdk/user-input#handle-clarifying-questions) für Verwendungsdetails.

**Eingabe:**

```python theme={null}
{
    "questions": [  # Fragen, die dem Benutzer gestellt werden (1-4 Fragen)
        {
            "question": str,  # Die vollständige Frage, die dem Benutzer gestellt werden soll
            "header": str,  # Sehr kurzes Label, das als Chip/Tag angezeigt wird (max. 12 Zeichen)
            "options": [  # Die verfügbaren Auswahlmöglichkeiten (2-4 Optionen)
                {
                    "label": str,  # Anzeigetext für diese Option (1-5 Wörter)
                    "description": str,  # Erklärung, was diese Option bedeutet
                    "preview": str | None,  # Vorschauinhalt, der angezeigt wird, wenn die Option fokussiert ist
                }
            ],
            "multiSelect": bool,  # Auf true setzen, um mehrere Auswahlen zu ermöglichen
        }
    ],
    "answers": dict[str, str] | None,
    # Benutzerantworten, die vom Berechtigungssystem ausgefüllt werden. Multi-Select-
    # Antworten sind eine kommagetrennte Zeichenkette von ausgewählten Labels; eine
    # Liste von Labels wird bei der Eingabe akzeptiert und in diese Form umgewandelt
    "annotations": dict[str, dict] | None,
    # Pro-Frage-Anmerkungen vom Benutzer, nach Fragetext indiziert.
    # Jeder Wert kann "preview" (der Vorschauinhalt der ausgewählten Option)
    # und "notes" (freie Notizen zur Auswahl) enthalten
    "metadata": dict | None,  # Analyse-Metadaten, wie {"source": "remember"}; wird dem Benutzer nicht angezeigt
}
```

**Ausgabe:**

```python theme={null}
{
    "questions": [  # Die Fragen, die gestellt wurden
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str, "preview": str | None}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Ordnet Fragetext der Antwortzeichenkette zu
    # Multi-Select-Antworten sind kommagetrennt
    "response": str | None,
    # Freie Antwort, die statt der Beantwortung der Fragen eingegeben wurde; wenn gesetzt,
    # erhält Claude "Der Benutzer hat geantwortet: ..." anstelle der Antworteliste
    "annotations": dict[str, dict] | None,  # Pro-Frage "preview" und "notes" aus den Auswahlen des Benutzers
    "afkTimeoutMs": int | None,  # Wird gesetzt, wenn der Dialog nach dieser vielen Millisekunden Benutzer-Inaktivität automatisch aufgelöst wurde; fehlt, wenn der Benutzer geantwortet hat
}
```

<h3 id="bash">
  Bash
</h3>

**Tool-Name:** `Bash`

**Eingabe:**

```python theme={null}
{
    "command": str,  # Der auszuführende Befehl
    "timeout": int | None,  # Optionales Timeout in Millisekunden (max. 600000; höhere Werte werden auf das Maximum begrenzt)
    "description": str | None,  # Klare, prägnante Beschreibung (5-10 Wörter)
    "run_in_background": bool | None,  # Auf true setzen, um im Hintergrund auszuführen
}
```

**Ausgabe:**

```python theme={null}
{
    "stdout": str,  # Die Ausgabe des Befehls; stdout und stderr kommen zusammengefasst in diesem einen verschachtelten Stream an
    "stderr": str,  # Hinweise, die das Tool selbst hinzufügt, nicht der stderr des Befehls
    "interrupted": bool,  # Ob der Befehl unterbrochen wurde
    "isImage": bool | None,  # Ob stdout Bilddaten enthält
    "backgroundTaskId": str | None,  # ID der Hintergrundaufgabe, wenn der Befehl im Hintergrund ausgeführt wird
}
```

<h3 id="monitor">
  Monitor
</h3>

**Tool-Name:** `Monitor`

Führt eine Background-Quelle aus und liefert jedes Ereignis an Claude, damit es reagieren kann, ohne zu pollen: `command` führt ein Skript aus und gibt ein Ereignis pro stdout-Zeile aus, und `ws` öffnet einen WebSocket und gibt ein Ereignis pro Textframe aus. Geben Sie genau eines von `command` oder `ws` an.

Wenn Monitor einen Befehl ausführt, folgt es den gleichen Berechtigungsregeln wie Bash; eine WebSocket-Überwachung fordert separat zur Genehmigung auf. Die `ws`-Quelle erfordert Claude Code v2.1.195 oder später. Siehe die [Monitor-Tool-Referenz](/docs/de/tools-reference#monitor-tool) für Verhalten und Provider-Verfügbarkeit.

**Eingabe:**

```python theme={null}
{
    "command": str | None,  # Shell-Skript; jede stdout-Zeile ist ein Ereignis, exit beendet die Überwachung
    "ws": dict | None,  # WebSocket-Quelle: {"url": str, "protocols": list[str] | None}; jeder Textframe ist ein Ereignis
    "description": str,  # Kurze Beschreibung, die in Benachrichtigungen angezeigt wird
    "timeout_ms": int | None,  # Frist in Millisekunden (Standard 300000, max. 3600000; die effektive Frist beträgt höchstens 1800000)
}
```

**Ausgabe:**

```python theme={null}
{
    "taskId": str,  # ID der Background-Monitor-Aufgabe
    "timeoutMs": int,  # Die effektive Frist der Überwachung in Millisekunden
    "persistent": bool | None,  # False: jede Überwachung hat eine Frist
}
```

<h3 id="edit">
  Edit
</h3>

**Tool-Name:** `Edit`

**Eingabe:**

```python theme={null}
{
    "file_path": str,  # Der absolute Pfad zur zu ändernden Datei
    "old_string": str,  # Der zu ersetzende Text
    "new_string": str,  # Der Text, durch den er ersetzt werden soll
    "replace_all": bool | None,  # Alle Vorkommen ersetzen (Standard False)
}
```

**Ausgabe:**

```python theme={null}
{
    "message": str,  # Bestätigungsmeldung
    "replacements": int,  # Anzahl der durchgeführten Ersetzungen
    "file_path": str,  # Dateipfad, der bearbeitet wurde
}
```

<h3 id="read">
  Read
</h3>

**Tool-Name:** `Read`

**Eingabe:**

```python theme={null}
{
    "file_path": str,  # Der absolute Pfad zur zu lesenden Datei
    "offset": int | None,  # Die Zeilennummer, ab der gelesen werden soll
    "limit": int | None,  # Die Anzahl der zu lesenden Zeilen
}
```

**Ausgabe (Textdateien):**

```python theme={null}
{
    "content": str,  # Dateiinhalt mit Zeilennummern
    "total_lines": int,  # Gesamtzahl der Zeilen in der Datei
    "lines_returned": int,  # Tatsächlich zurückgegebene Zeilen
}
```

**Ausgabe (Bilder):**

```python theme={null}
{
    "image": str,  # Base64-codierte Bilddaten
    "mime_type": str,  # MIME-Typ des Bildes
    "file_size": int,  # Dateigröße in Bytes
}
```

<h3 id="write">
  Write
</h3>

**Tool-Name:** `Write`

**Eingabe:**

```python theme={null}
{
    "file_path": str,  # Der absolute Pfad zur zu schreibenden Datei
    "content": str,  # Der in die Datei zu schreibende Inhalt
}
```

**Ausgabe:**

```python theme={null}
{
    "message": str,  # Erfolgsmeldung
    "bytes_written": int,  # Anzahl der geschriebenen Bytes
    "file_path": str,  # Dateipfad, der geschrieben wurde
}
```

<h3 id="glob">
  Glob
</h3>

**Tool-Name:** `Glob`

**Eingabe:**

```python theme={null}
{
    "pattern": str,  # Das Glob-Muster zum Abgleich von Dateien
    "path": str | None,  # Das zu durchsuchende Verzeichnis (Standard: cwd)
}
```

**Ausgabe:**

```python theme={null}
{
    "matches": list[str],  # Array von übereinstimmenden Dateipfaden
    "count": int,  # Anzahl der gefundenen Übereinstimmungen
    "search_path": str,  # Verwendetes Suchverzeichnis
}
```

<h3 id="grep">
  Grep
</h3>

**Tool-Name:** `Grep`

**Eingabe:**

```python theme={null}
{
    "pattern": str,  # Das reguläre Ausdrucksmuster
    "path": str | None,  # Datei oder Verzeichnis zum Durchsuchen
    "glob": str | None,  # Glob-Muster zum Filtern von Dateien
    "type": str | None,  # Dateityp zum Durchsuchen
    "output_mode": str | None,  # "content", "files_with_matches" oder "count"
    "-i": bool | None,  # Suche ohne Berücksichtigung der Groß-/Kleinschreibung
    "-n": bool | None,  # Zeilennummern anzeigen
    "-B": int | None,  # Zeilen vor jeder Übereinstimmung anzeigen
    "-A": int | None,  # Zeilen nach jeder Übereinstimmung anzeigen
    "-C": int | None,  # Zeilen vor und nach anzeigen
    "head_limit": int | None,  # Ausgabe auf erste N Zeilen/Einträge begrenzen
    "multiline": bool | None,  # Mehrzeilenmodus aktivieren
}
```

**Ausgabe (content-Modus):**

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

**Ausgabe (files\_with\_matches-Modus):**

```python theme={null}
{
    "files": list[str],  # Dateien mit Übereinstimmungen
    "count": int,  # Anzahl der Dateien mit Übereinstimmungen
}
```

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Tool-Name:** `NotebookEdit`

**Eingabe:**

```python theme={null}
{
    "notebook_path": str,  # Absoluter Pfad zum Jupyter-Notebook
    "cell_id": str | None,  # Die ID der zu bearbeitenden Zelle
    "new_source": str,  # Die neue Quelle für die Zelle
    "cell_type": "code" | "markdown" | None,  # Der Typ der Zelle
    "edit_mode": "replace" | "insert" | "delete" | None,  # Bearbeitungsvorgangstyp
}
```

**Ausgabe:**

```python theme={null}
{
    "message": str,  # Erfolgsmeldung
    "edit_type": "replaced" | "inserted" | "deleted",  # Typ der durchgeführten Bearbeitung
    "cell_id": str | None,  # Zellen-ID, die betroffen war
    "total_cells": int,  # Gesamtzellen im Notebook nach Bearbeitung
}
```

<h3 id="webfetch">
  WebFetch
</h3>

**Tool-Name:** `WebFetch`

**Eingabe:**

```python theme={null}
{
    "url": str,  # Die URL, von der Inhalte abgerufen werden sollen
    "prompt": str,  # Der Prompt, der auf den abgerufenen Inhalt angewendet werden soll
}
```

**Ausgabe:**

```python theme={null}
{
    "bytes": int,  # Größe des abgerufenen Inhalts in Bytes
    "code": int,  # HTTP-Antwortcode
    "codeText": str,  # HTTP-Antwortcodetext
    "result": str,  # Verarbeitetes Ergebnis aus der Anwendung des Prompts auf den Inhalt
    "durationMs": int,  # Zeit zum Abrufen und Verarbeiten des Inhalts in Millisekunden
    "url": str,  # URL, die abgerufen wurde
}
```

<h3 id="websearch">
  WebSearch
</h3>

**Tool-Name:** `WebSearch`

**Eingabe:**

```python theme={null}
{
    "query": str,  # Die zu verwendende Suchanfrage
    "allowed_domains": list[str] | None,  # Nur Ergebnisse von diesen Domains einbeziehen
    "blocked_domains": list[str] | None,  # Niemals Ergebnisse von diesen Domains einbeziehen
}
```

**Ausgabe:**

```python theme={null}
{
    "query": str,  # Die Suchanfrage
    "results": list[str | {"tool_use_id": str, "content": list[{"title": str, "url": str}]}],
    "durationSeconds": float,  # Suchdauer in Sekunden
}
```

<h3 id="todowrite">
  TodoWrite
</h3>

**Tool-Name:** `TodoWrite`

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Siehe [Modellverfügbarkeit](/docs/de/agent-sdk/todo-tracking#model-availability), um sich anzumelden.
</Note>

**Eingabe:**

```python theme={null}
{
    "todos": [
        {
            "content": str,  # Die Aufgabenbeschreibung
            "status": "pending" | "in_progress" | "completed",  # Aufgabenstatus
            "activeForm": str,  # Aktive Form der Beschreibung
        }
    ]
}
```

**Ausgabe:**

```python theme={null}
{
    "message": str,  # Erfolgsmeldung
    "stats": {"total": int, "pending": int, "in_progress": int, "completed": int},
}
```

<h3 id="taskcreate">
  TaskCreate
</h3>

**Tool-Name:** `TaskCreate`

**Eingabe:**

```python theme={null}
{
    "subject": str,  # Kurzer Aufgabentitel
    "description": str,  # Detaillierter Aufgabentext
    "activeForm": str | None,  # Präsens-Label, das während der Ausführung angezeigt wird
    "metadata": dict | None,  # Beliebige Aufrufer-Metadaten
}
```

**Ausgabe:**

```python theme={null}
{
    "task": {"id": str, "subject": str},  # Erstellte Aufgabe mit zugewiesener ID
}
```

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Tool-Name:** `TaskUpdate`

**Eingabe:**

```python theme={null}
{
    "taskId": str,  # ID der zu patchenden Aufgabe
    "status": Literal["pending", "in_progress", "completed", "deleted"] | None,
    "subject": str | None,
    "description": str | None,
    "activeForm": str | None,
    "addBlocks": list[str] | None,  # Aufgaben-IDs, die diese Aufgabe jetzt blockiert
    "addBlockedBy": list[str] | None,  # Aufgaben-IDs, die diese Aufgabe jetzt blockieren
    "owner": str | None,
    "metadata": dict | None,
}
```

**Ausgabe:**

```python theme={null}
{
    "success": bool,
    "taskId": str,
    "updatedFields": list[str],  # Namen der Felder, die sich geändert haben
    "error": str | None,
    "statusChange": {"from": str, "to": str} | None,
}
```

<h3 id="taskget">
  TaskGet
</h3>

**Tool-Name:** `TaskGet`

**Eingabe:**

```python theme={null}
{
    "taskId": str,  # ID der zu lesenden Aufgabe
}
```

**Ausgabe:**

```python theme={null}
{
    "task": {
        "id": str,
        "subject": str,
        "description": str,
        "status": Literal["pending", "in_progress", "completed"],
        "blocks": list[str],
        "blockedBy": list[str],
    } | None,  # None, wenn die ID nicht gefunden wird
}
```

<h3 id="tasklist">
  TaskList
</h3>

**Tool-Name:** `TaskList`

**Eingabe:**

```python theme={null}
{}
```

**Ausgabe:**

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

Entfernt in Claude Code v2.1.277. Zuvor wurde die Ausgabe einer laufenden oder abgeschlossenen Background-Aufgabe abgerufen, wobei `BashOutput` als Alias akzeptiert wurde; Claude liest die Ausgabedatei einer Background-Aufgabe stattdessen mit `Read`.

Ein `disallowed_tools`-Eintrag oder eine Ablehnungsregel, die immer noch einen der beiden Namen nennt, wird ohne Warnung ignoriert.

<h3 id="taskstop">
  TaskStop
</h3>

**Tool-Name:** `TaskStop`. Die früheren Namen `KillShell` und `KillBash` werden immer noch als Aliase akzeptiert.

**Eingabe:**

```python theme={null}
{
    "task_id": str | None,  # Die ID der zu stoppenden Background-Aufgabe
    "shell_id": str | None,  # Veraltet: verwenden Sie stattdessen task_id
}
```

**Ausgabe:**

```python theme={null}
{
    "message": str,  # Statusmeldung über den Vorgang
    "task_id": str,  # Die ID der gestoppten Aufgabe
    "task_type": str,  # Der Typ der gestoppten Aufgabe
    "command": str | None,  # Der Befehl oder die Beschreibung der gestoppten Aufgabe
}
```

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Tool-Name:** `ExitPlanMode`

**Eingabe:**

```python theme={null}
{
    "plan": str  # Der Plan, der vom Benutzer zur Genehmigung ausgeführt werden soll
}
```

**Ausgabe:**

```python theme={null}
{
    "message": str,  # Bestätigungsmeldung
    "approved": bool | None,  # Ob der Benutzer den Plan genehmigt hat
}
```

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Tool-Name:** `ListMcpResourcesTool`

**Eingabe:**

```python theme={null}
{
    "server": str | None  # Optionaler Servername zum Filtern von Ressourcen
}
```

**Ausgabe:**

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

**Tool-Name:** `ReadMcpResourceTool`

**Eingabe:**

```python theme={null}
{
    "server": str,  # Der MCP-Servername
    "uri": str,  # Die zu lesende Ressourcen-URI
}
```

**Ausgabe:**

```python theme={null}
{
    "contents": [
        {"uri": str, "mimeType": str | None, "text": str | None, "blob": str | None}
    ],
    "server": str,
}
```

<h2 id="build-a-continuous-conversation-interface">
  Erstellen einer kontinuierlichen Konversationsschnittstelle
</h2>

Das folgende Beispiel hält einen `ClaudeSDKClient` über mehrere Durchläufe hinweg verbunden, sodass Claude sich an frühere Nachrichten erinnert. Geben Sie `new` ein, um die Verbindung zu trennen und neu zu verbinden, um eine neue Sitzung zu starten, oder `exit`, um das Gespräch zu beenden.

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
  Fehlerbehandlung
</h2>

Das folgende Beispiel umhüllt einen `query()`-Aufruf mit Handlern für vier der [Fehlertypen](#error-types), die das SDK auslöst.

Dieses Beispiel fängt [`ResultError`](#resulterror) ab, was Python Agent SDK 0.2.140 oder später erfordert.

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
  Sandbox-Konfiguration
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Konfiguration für das Sandbox-Verhalten. Verwenden Sie dies, um Command-Sandboxing zu aktivieren und Netzwerkbeschränkungen programmgesteuert zu konfigurieren.

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

| Eigenschaft                 | Typ                                                   | Standard | Beschreibung                                                                                                                                                                                                                                                               |
| :-------------------------- | :---------------------------------------------------- | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `bool`                                                | `False`  | Aktivieren Sie den Sandbox-Modus für die Befehlsausführung                                                                                                                                                                                                                 |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`   | Genehmigen Sie Bash-Befehle automatisch, wenn die Sandbox aktiviert ist                                                                                                                                                                                                    |
| `excludedCommands`          | `list[str]`                                           | `[]`     | Befehle, die Sandbox-Beschränkungen umgehen, z. B. `["docker *"]`. Diese werden automatisch ohne Modellbeteiligung unsandboxed ausgeführt; [`sandbox.excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) behandelt, wann ein Eintrag gilt                  |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`   | Erlauben Sie dem Modell, die Ausführung von Befehlen außerhalb der Sandbox anzufordern. Wenn `True`, kann das Modell `dangerouslyDisableSandbox` in der Tool-Eingabe setzen, was auf das [Berechtigungssystem](#permissions-fallback-for-unsandboxed-commands) zurückfällt |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`   | Netzwerkspezifische Sandbox-Konfiguration                                                                                                                                                                                                                                  |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`   | Konfigurieren Sie, welche Sandbox-Verstöße ignoriert werden sollen                                                                                                                                                                                                         |
| `enableWeakerNestedSandbox` | `bool`                                                | `False`  | Aktivieren Sie eine schwächere verschachtelte Sandbox für Kompatibilität                                                                                                                                                                                                   |

<Note>
  Die Sandbox hängt von der Plattformunterstützung ab und benötigt unter Linux Tools wie `bubblewrap` und `socat`. Standardmäßig werden Befehle, wenn `enabled` auf `True` gesetzt ist, aber die Sandbox nicht gestartet werden kann, unsandboxed mit einer Warnung auf stderr ausgeführt. Dieses Standardverhalten unterscheidet sich vom TypeScript SDK, wo `failIfUnavailable` standardmäßig auf `true` gesetzt ist.

  Setzen Sie `"failIfUnavailable": True` in Ihren Sandbox-Einstellungen, um stattdessen zu stoppen. Der Schlüssel ist noch nicht auf `SandboxSettings` deklariert, aber das SDK leitet ihn an Claude Code weiter, das ihn berücksichtigt. `query()` meldet dann eine `ResultMessage` mit `subtype="error_during_execution"` und den Grund in `errors`. Da dies ein einzelner `query()`-Aufruf ist, löst das SDK nach dem Yielding dieses Fehler-Ergebnisses aus, daher wickeln Sie die Schleife in einen try-Block ein, um über ihn hinwegzugehen. Siehe [Ergebnis verarbeiten](/docs/de/agent-sdk/agent-loop#handle-the-result) für den Fehlervertrag.
</Note>

<h4 id="example-usage">
  Beispielverwendung
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
  **Unix-Socket-Sicherheit**: Die Option `allowUnixSockets` kann Zugriff auf Systemdienste gewähren, die außerhalb der Sandbox reichen. Zum Beispiel ermöglicht das Zulassen von `/var/run/docker.sock` effektiv vollständigen Host-Systemzugriff über die Docker-API und umgeht die Sandbox-Isolation. Erlauben Sie nur Unix-Sockets, die absolut notwendig sind, und verstehen Sie die Sicherheitsauswirkungen jedes einzelnen.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Netzwerkspezifische Konfiguration für den Sandbox-Modus. Diese Einstellungen gelten für Sandbox-Bash-Befehle, wenn `enabled` in den übergeordneten [`SandboxSettings`](#sandboxsettings) auf `True` gesetzt ist. Sie beschränken das WebFetch-Tool nicht, das stattdessen [Berechtigungsregeln](/docs/de/permissions#webfetch) verwendet.

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

| Eigenschaft               | Typ         | Standard | Beschreibung                                                                                                                                                                                                                                         |
| :------------------------ | :---------- | :------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `list[str]` | `[]`     | Domänennamen, auf die Sandbox-Prozesse zugreifen können                                                                                                                                                                                              |
| `deniedDomains`           | `list[str]` | `[]`     | Domänennamen, auf die Sandbox-Prozesse nicht zugreifen können. Hat Vorrang vor `allowedDomains`                                                                                                                                                      |
| `allowManagedDomainsOnly` | `bool`      | `False`  | Nur verwaltete Einstellungen: Wenn in verwalteten Einstellungen gesetzt, ignorieren Sie `allowedDomains` und `WebFetch(domain:...)`-Zulassungsregeln aus nicht verwalteten Einstellungsquellen. Hat keine Auswirkung, wenn über SDK-Optionen gesetzt |
| `allowUnixSockets`        | `list[str]` | `[]`     | Nur macOS: Unix-Socket-Pfade, auf die Prozesse zugreifen können, z. B. der Docker-Socket. Wird unter Linux ignoriert                                                                                                                                 |
| `allowAllUnixSockets`     | `bool`      | `False`  | Erlauben Sie Zugriff auf alle Unix-Sockets                                                                                                                                                                                                           |
| `allowLocalBinding`       | `bool`      | `False`  | Erlauben Sie Prozessen, sich an lokale Ports zu binden (z. B. für Dev-Server)                                                                                                                                                                        |
| `allowMachLookup`         | `list[str]` | `[]`     | Nur macOS: XPC/Mach-Servicenamen zum Zulassen. Unterstützt ein nachfolgendes Platzhalterzeichen                                                                                                                                                      |
| `httpProxyPort`           | `int`       | `None`   | HTTP-Proxy-Port für Netzwerkanfragen                                                                                                                                                                                                                 |
| `socksProxyPort`          | `int`       | `None`   | SOCKS-Proxy-Port für Netzwerkanfragen                                                                                                                                                                                                                |

<Note>
  Der integrierte Sandbox-Proxy erzwingt die Netzwerk-Allowlist basierend auf dem angeforderten Hostnamen und beendet oder inspiziert keinen TLS-Verkehr, daher können Techniken wie [Domain Fronting](https://en.wikipedia.org/wiki/Domain_fronting) ihn möglicherweise umgehen. Siehe [Sandboxing-Sicherheitsbeschränkungen](/docs/de/sandboxing#security-limitations) für Details und [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment#traffic-forwarding) für die Konfiguration eines TLS-terminierenden Proxys.
</Note>

<h3 id="sandboxignoreviolations">
  `SandboxIgnoreViolations`
</h3>

Konfiguration zum Ignorieren bestimmter Sandbox-Verstöße.

```python theme={null}
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| Eigenschaft | Typ         | Standard | Beschreibung                                               |
| :---------- | :---------- | :------- | :--------------------------------------------------------- |
| `file`      | `list[str]` | `[]`     | Dateipfad-Muster, für die Verstöße ignoriert werden sollen |
| `network`   | `list[str]` | `[]`     | Netzwerkmuster, für die Verstöße ignoriert werden sollen   |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Berechtigungen-Fallback für Unsandboxed-Befehle
</h3>

Wenn `allowUnsandboxedCommands` aktiviert ist, kann das Modell anfordern, Befehle außerhalb der Sandbox auszuführen, indem es `dangerouslyDisableSandbox: True` in der Tool-Eingabe setzt. Diese Anfragen fallen auf das bestehende Berechtigungssystem zurück, was bedeutet, dass Ihr `can_use_tool`-Handler aufgerufen wird, sodass Sie benutzerdefinierte Autorisierungslogik implementieren können.

Ihre `excludedCommands`-Einträge nehmen stattdessen einen Aufruf aus der Sandbox mit keiner Modellbeteiligung; [`sandbox.excludedCommands`](/docs/de/settings-reference#sandbox-excludedcommands) behandelt, wann ein Eintrag gilt.

Das folgende Beispiel protokolliert jede Unsandboxed-Anfrage und lehnt sie ab, es sei denn, Ihre eigene Autorisierungslogik erlaubt es:

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
  Befehle, die mit `dangerouslyDisableSandbox: True` ausgeführt werden, haben vollständigen Systemzugriff. Stellen Sie sicher, dass Ihr `can_use_tool`-Handler diese Anfragen sorgfältig validiert.

  Wenn `permission_mode` auf `bypassPermissions` gesetzt ist und `allow_unsandboxed_commands` aktiviert ist, kann das Modell autonom Befehle außerhalb der Sandbox ausführen, ohne Genehmigungsaufforderungen, abgesehen von den [Aktionen, die der No-Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves). Diese Kombination ermöglicht dem Modell effektiv, die Sandbox-Isolation stillschweigend zu verlassen.
</Warning>

<h2 id="see-also">
  Siehe auch
</h2>

* [SDK-Übersicht](/docs/de/agent-sdk/overview) - Allgemeine SDK-Konzepte
* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript) - TypeScript SDK-Dokumentation
* [Benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools) - Definieren Sie In-Process-MCP-Tools, die Claude aufrufen kann
* [CLI-Referenz](/docs/de/cli-reference) - Befehlszeilenschnittstelle
* [Häufige Workflows](/docs/de/common-workflows) - Schritt-für-Schritt-Anleitungen
