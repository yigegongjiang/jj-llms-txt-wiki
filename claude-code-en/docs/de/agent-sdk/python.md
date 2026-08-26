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

Für uv, Windows PowerShell und API-Schlüssel-Setup siehe [Erste Schritte in der Agent SDK-Übersicht](/docs/de/agent-sdk/overview#get-started).

<h2 id="choosing-between-query-and-claudesdkclient">
  Wahl zwischen `query()` und `ClaudeSDKClient`
</h2>

Das Python SDK bietet zwei Möglichkeiten, um mit Claude Code zu interagieren:

<h3 id="quick-comparison">
  Schnellvergleich
</h3>

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

<h3 id="when-to-use-query-one-off-tasks">
  Wann `query()` verwendet werden sollte (einmalige Aufgaben)
</h3>

**Am besten für:**

* Einmalige Fragen, bei denen Sie keinen Konversationsverlauf benötigen
* Unabhängige Aufgaben, die keinen Kontext aus vorherigen Austauschen erfordern
* Einfache Automatisierungsskripte
* Wenn Sie jedes Mal einen neuen Anfang möchten

<h3 id="when-to-use-claudesdkclient-continuous-conversation">
  Wann `ClaudeSDKClient` verwendet werden sollte (kontinuierliche Konversation)
</h3>

**Am besten für:**

* **Konversationen fortsetzen** - Wenn Claude den Kontext merken muss
* **Nachfolgefragen** - Aufbauend auf vorherigen Antworten
* **Interaktive Anwendungen** - Chat-Schnittstellen, REPLs
* **Antwortgesteuerte Logik** - Wenn die nächste Aktion von Claudes Antwort abhängt
* **Sitzungskontrolle** - Explizite Verwaltung des Konversationslebenszyklus

<h2 id="functions">
  Funktionen
</h2>

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
        cwd="/home/user/project",
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

| Parameter      | Typ                                             | Beschreibung                                                                     |
| :------------- | :---------------------------------------------- | :------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Eindeutige Kennung für das Tool                                                  |
| `description`  | `str`                                           | Lesbare Beschreibung, was das Tool tut                                           |
| `input_schema` | `type \| dict[str, Any]`                        | Schema, das die Eingabeparameter des Tools definiert (siehe unten)               |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Optionale MCP-Tool-Anmerkungen, die Verhaltenshinweise für Clients bereitstellen |

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

Erneut exportiert aus `mcp.types` (auch verfügbar als `from claude_agent_sdk import ToolAnnotations`). Alle Felder sind optionale Hinweise; Clients sollten sich nicht auf sie für Sicherheitsentscheidungen verlassen.

| Feld              | Typ            | Standard | Beschreibung                                                                                                                                          |
| :---------------- | :------------- | :------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `str \| None`  | `None`   | Lesbare Bezeichnung für das Tool                                                                                                                      |
| `readOnlyHint`    | `bool \| None` | `False`  | Wenn `True`, ändert das Tool seine Umgebung nicht                                                                                                     |
| `destructiveHint` | `bool \| None` | `True`   | Wenn `True`, kann das Tool destruktive Aktualisierungen durchführen (nur sinnvoll, wenn `readOnlyHint` `False` ist)                                   |
| `idempotentHint`  | `bool \| None` | `False`  | Wenn `True`, haben wiederholte Aufrufe mit denselben Argumenten keine zusätzliche Auswirkung (nur sinnvoll, wenn `readOnlyHint` `False` ist)          |
| `openWorldHint`   | `bool \| None` | `True`   | Wenn `True`, interagiert das Tool mit externen Entitäten (z. B. Websuche). Wenn `False`, ist die Domäne des Tools geschlossen (z. B. ein Memory-Tool) |

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

| Eigenschaft          | Typ                            | Beschreibung                         |
| :------------------- | :----------------------------- | :----------------------------------- |
| `type`               | `Literal["user", "assistant"]` | Nachrichtenrolle                     |
| `uuid`               | `str`                          | Eindeutige Nachrichtenkennung        |
| `session_id`         | `str`                          | Sitzungskennung                      |
| `message`            | `Any`                          | Roher Nachrichteninhalt              |
| `parent_tool_use_id` | `None`                         | Reserviert für zukünftige Verwendung |

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

# Tag a session
tag_session("550e8400-e29b-41d4-a716-446655440000", "needs-review")

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

**Behält eine Konversationssitzung über mehrere Austausche hinweg bei.** Dies ist das Python-Äquivalent dazu, wie die `query()`-Funktion des TypeScript SDK intern funktioniert - sie erstellt ein Client-Objekt, das Konversationen fortsetzen kann.

<h4 id="key-features">
  Wichtige Funktionen
</h4>

* **Sitzungskontinuität**: Behält Konversationskontext über mehrere `query()`-Aufrufe hinweg bei
* **Gleiche Konversation**: Die Sitzung behält vorherige Nachrichten bei
* **Unterbrechungsunterstützung**: Kann die Ausführung mitten in einer Aufgabe stoppen
* **Expliziter Lebenszyklus**: Sie kontrollieren, wann die Sitzung beginnt und endet
* **Antwortgesteuerte Abläufe**: Kann auf Antworten reagieren und Nachfolgefragen senden
* **Benutzerdefinierte Tools und Hooks**: Unterstützt benutzerdefinierte Tools (erstellt mit dem `@tool`-Dekorator) und Hooks

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
| `set_model(model)`                        | Ändern Sie das Modell für die aktuelle Sitzung. Übergeben Sie `None`, um auf Standard zurückzusetzen                                                                                             |
| `rewind_files(user_message_id)`           | Stellen Sie Dateien in ihren Zustand bei der angegebenen Benutzernachricht wieder her. Erfordert `enable_file_checkpointing=True`. Siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Rufen Sie den Status aller konfigurierten MCP-Server ab. Gibt [`McpStatusResponse`](#mcpstatusresponse) zurück                                                                                   |
| `reconnect_mcp_server(server_name)`       | Versuchen Sie, eine Verbindung zu einem MCP-Server herzustellen, der fehlgeschlagen ist oder getrennt wurde                                                                                      |
| `toggle_mcp_server(server_name, enabled)` | Aktivieren oder deaktivieren Sie einen MCP-Server während der Sitzung. Das Deaktivieren entfernt seine Tools                                                                                     |
| `stop_task(task_id)`                      | Stoppen Sie eine laufende Hintergrundaufgabe. Eine [`TaskNotificationMessage`](#tasknotificationmessage) mit Status `"stopped"` folgt im Nachrichtenstrom                                        |
| `get_server_info()`                       | Rufen Sie Serverinformationen einschließlich Sitzungs-ID und Funktionen ab                                                                                                                       |
| `disconnect()`                            | Trennen Sie die Verbindung zu Claude                                                                                                                                                             |

<h4 id="context-manager-support">
  Context Manager-Unterstützung
</h4>

Der Client kann als asynchroner Context Manager für automatische Verbindungsverwaltung verwendet werden:

```python theme={null}
async with ClaudeSDKClient() as client:
    await client.query("Hello Claude")
    async for message in client.receive_response():
        print(message)
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
  **Pufferverhalten nach Unterbrechung:** `interrupt()` sendet ein Stopsignal, löscht aber nicht den Nachrichtenpuffer. Nachrichten, die bereits von der unterbrochenen Aufgabe produziert wurden, einschließlich ihrer `ResultMessage` (mit `subtype="error_during_execution"`), bleiben im Stream. Sie müssen sie mit `receive_response()` entleeren, bevor Sie die Antwort auf eine neue Abfrage lesen. Wenn Sie unmittelbar nach `interrupt()` eine neue Abfrage senden und `receive_response()` nur einmal aufrufen, erhalten Sie die Nachrichten der unterbrochenen Aufgabe, nicht die Antwort der neuen Abfrage.
</Note>

<h4 id="example-advanced-permission-control">
  Beispiel - Erweiterte Berechtigungskontrolle
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

| Eigenschaft    | Typ                                        | Beschreibung                                                                                               |
| :------------- | :----------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                      | Eindeutige Kennung für das Tool                                                                            |
| `description`  | `str`                                      | Lesbare Beschreibung                                                                                       |
| `input_schema` | `type[T] \| dict[str, Any]`                | Schema für Eingabevalidierung                                                                              |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]` | Asynchrone Funktion, die die Tool-Ausführung handhabt                                                      |
| `annotations`  | `ToolAnnotations \| None`                  | Optionale MCP-Tool-Anmerkungen (z. B. `readOnlyHint`, `destructiveHint`, `openWorldHint`). Aus `mcp.types` |

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

| Eigenschaft                   | Typ                                                                                   | Standard                            | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------------------- | :------------------------------------------------------------------------------------ | :---------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                              | Tools-Konfiguration. Verwenden Sie `{"type": "preset", "preset": "claude_code"}` für die Standard-Tools von Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                                | Tools, die automatisch genehmigt werden, ohne zu fragen. Dies beschränkt Claude nicht nur auf diese Tools; nicht aufgelistete Tools fallen durch `permission_mode` und `can_use_tool`. Verwenden Sie `disallowed_tools`, um Tools zu blockieren. Siehe [Berechtigungen](/docs/de/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                           |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptFile \| None`                               | `None`                              | System-Prompt-Konfiguration. Übergeben Sie eine Zeichenkette für einen benutzerdefinierten Prompt, `{"type": "preset", "preset": "claude_code"}` für den System-Prompt von Claude Code mit optionalem `"append"`, oder `{"type": "file", "path": "..."}` zum Laden eines großen Prompts von der Festplatte. Siehe [`SystemPromptPreset`](#systempromptpreset) und [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                                                                                                         |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                                | MCP-Server-Konfigurationen oder Pfad zur Konfigurationsdatei                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `strict_mcp_config`           | `bool`                                                                                | `False`                             | Wenn `True`, verwenden Sie nur die Server, die in `mcp_servers` übergeben werden, und ignorieren Sie das Projekt `.mcp.json`, Benutzereinstellungen, von Plugins bereitgestellte MCP-Server und [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai). Entspricht dem CLI-Flag `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                                                    |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                              | Berechtigungsmodus für die Tool-Nutzung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `continue_conversation`       | `bool`                                                                                | `False`                             | Setzen Sie die neueste Konversation fort                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `resume`                      | `str \| None`                                                                         | `None`                              | Sitzungs-ID zum Fortsetzen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `max_turns`                   | `int \| None`                                                                         | `None`                              | Maximale agentengesteuerte Umdrehungen (Tool-Use-Rundgänge)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                              | Stoppen Sie die Abfrage, wenn die clientseitige Kostenschätzung diesen USD-Wert erreicht. Verglichen mit der gleichen Schätzung wie `total_cost_usd`; siehe [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking) für Genauigkeitsvorbehalt                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                                | Tools, die verweigert werden. Ein einfacher Name wie `"Bash"` entfernt das Tool aus Claudes Kontext. Eine scoped-Regel wie `"Bash(rm *)"` lässt das Tool verfügbar und verweigert übereinstimmende Aufrufe in jedem Berechtigungsmodus, einschließlich `bypassPermissions`. Siehe [Berechtigungen](/docs/de/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                             | Aktivieren Sie die Dateienänderungsverfolgung zum Zurückspulen. Siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `model`                       | `str \| None`                                                                         | `None`                              | Claude-Modell-Alias oder vollständiger Modellname. Siehe [akzeptierte Werte und anbieter-spezifische IDs](/docs/de/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `fallback_model`              | `str \| None`                                                                         | `None`                              | Fallback-Modell, das verwendet wird, wenn das primäre Modell fehlschlägt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                                | Beta-Funktionen zum Aktivieren. Siehe [`SdkBeta`](#sdkbeta) für verfügbare Optionen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                              | Ausgabeformat für strukturierte Antworten (z. B. `{"type": "json_schema", "schema": {...}}`). Siehe [Strukturierte Ausgaben](/docs/de/agent-sdk/structured-outputs) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                              | MCP-Tool-Name für Berechtigungsaufforderungen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                              | Aktuelles Arbeitsverzeichnis                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                              | Benutzerdefinierter Pfad zur Claude Code CLI-Ausführungsdatei                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `settings`                    | `str \| None`                                                                         | `None`                              | Pfad zur Einstellungsdatei                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                                | Zusätzliche Verzeichnisse, auf die Claude zugreifen kann                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `env`                         | `dict[str, str]`                                                                      | `{}`                                | Umgebungsvariablen, die auf der geerbten Prozessumgebung zusammengeführt werden. Siehe [Umgebungsvariablen](/docs/de/env-vars) für Variablen, die die zugrunde liegende CLI liest, und [Langsame oder steckengebliebene API-Antworten handhaben](#handle-slow-or-stalled-api-responses) für Timeout-bezogene Variablen                                                                                                                                                                                                                                                                                                                                                                                                 |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                                | Zusätzliche CLI-Argumente, die direkt an die CLI übergeben werden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                              | Maximale Bytes beim Puffern der CLI-Stdout                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                        | *Veraltet* - Dateiähnliches Objekt für Debug-Ausgabe. Verwenden Sie stattdessen den `stderr`-Callback                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                              | Callback-Funktion für stderr-Ausgabe von CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                              | Tool-Berechtigungs-Callback, der nur aufgerufen wird, wenn der [Berechtigungsfluss](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) zu einer Aufforderung führt. Nicht aufgerufen für Aufrufe, die automatisch von `allowed_tools`, Allow-Regeln oder `permission_mode` genehmigt werden. `AskUserQuestion`, Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, erreichen ihn auch, wenn Sie sie erlaubt haben; im `dontAsk`-Modus werden diese stattdessen verweigert. Siehe [`CanUseTool`](#canusetool) für Details |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                              | Hook-Konfigurationen zum Abfangen von Ereignissen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `user`                        | `str \| None`                                                                         | `None`                              | Benutzerkennung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `include_partial_messages`    | `bool`                                                                                | `False`                             | Schließen Sie partielle Nachrichtenstreaming-Ereignisse ein. Wenn aktiviert, werden [`StreamEvent`](#streamevent)-Nachrichten geliefert                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `include_hook_events`         | `bool`                                                                                | `False`                             | Schließen Sie Hook-Lebenszyklusereignisse im Nachrichtenstrom als `HookEventMessage`-Objekte ein                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `fork_session`                | `bool`                                                                                | `False`                             | Wenn Sie mit `resume` fortsetzen, verzweigen Sie sich zu einer neuen Sitzungs-ID, anstatt die ursprüngliche Sitzung fortzusetzen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                              | Programmgesteuert definierte Subagenten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                                | Laden Sie benutzerdefinierte Plugins aus lokalen Pfaden. Siehe [Plugins](/docs/de/agent-sdk/plugins) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                              | Konfigurieren Sie das Sandbox-Verhalten programmgesteuert. Siehe [Sandbox-Einstellungen](#sandboxsettings) für Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (CLI-Standard: alle Quellen) | Kontrollieren Sie, welche Dateisystem-Einstellungen geladen werden. Übergeben Sie `[]`, um Benutzer-, Projekt- und lokale Einstellungen zu deaktivieren. Verwaltete Richtlinieneinstellungen werden unabhängig davon geladen; Server-verwaltete Einstellungen werden abgerufen, wenn sich die Sitzung mit einer Organisationsanmeldedaten auf einer [berechtigten Konfiguration](/docs/de/server-managed-settings#platform-availability) authentifiziert. Siehe [Claude Code-Funktionen verwenden](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                            |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                              | Skills, die der Sitzung zur Verfügung stehen. Übergeben Sie `"all"`, um jeden erkannten Skill zu aktivieren, oder eine Liste von Skill-Namen. Wenn gesetzt, aktiviert das SDK das Skill-Tool automatisch in `allowed_tools`. Wenn Sie auch `tools` übergeben, schließen Sie `"Skill"` in diese Liste ein. Siehe [Skills](/docs/de/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                    |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                              | *Veraltet* - Maximale Token für Thinking-Blöcke. Verwenden Sie stattdessen `thinking`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                              | Steuert das Verhalten des erweiterten Denkens. Hat Vorrang vor `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                              | Anstrengungsstufe für die Denktiefe. Siehe [Anstrengungsstufe anpassen](/docs/de/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `session_store`               | [`SessionStore`](/docs/de/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                              | Spiegeln Sie Sitzungstranskripte zu einem externen Backend, damit jeder Host sie fortsetzen kann. Siehe [Sitzungen im externen Speicher beibehalten](/docs/de/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                         | Wann sollen gespiegelte Transkripteinträge zu `session_store` geleert werden. `"batched"` leert einmal pro Umdrehung oder wenn der Puffer voll wird; `"eager"` löst nach jedem Frame einen Hintergrund-Flush aus. Wird ignoriert, wenn `session_store` `None` ist                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

<h4 id="handle-slow-or-stalled-api-responses">
  Langsame oder steckengebliebene API-Antworten handhaben
</h4>

Die CLI-Subprozess liest mehrere Umgebungsvariablen, die API-Timeouts und Stall-Erkennung steuern. Übergeben Sie sie durch `ClaudeAgentOptions.env`:

```python theme={null}
options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS`: Pro-Request-Timeout auf dem Anthropic-Client in Millisekunden. Standard `600000`. Gilt für die Hauptschleife und alle Subagenten.
* `CLAUDE_CODE_MAX_RETRIES`: Maximale API-Wiederholungen. Standard `10`, begrenzt auf `15`. Jede Wiederholung erhält sein eigenes `API_TIMEOUT_MS`-Fenster, daher ist die schlimmste Wandzeit ungefähr `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` plus Backoff. Für unbeaufsichtigte Läufe, die längere Ausfallzeiten abwarten müssen, setzen Sie `CLAUDE_CODE_RETRY_WATCHDOG=1`: Es wiederholt Kapazitätsfehler unbegrenzt, und ab Claude Code v2.1.199 erhöht sich der Standard für andere vorübergehende Fehler auf `300` und entfernt die Obergrenze für diese Variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: Stall-Watchdog für Subagenten, die mit `run_in_background` gestartet werden. Standard `600000`. Setzt sich bei jedem Stream-Ereignis zurück; bei Stall bricht es den Subagenten ab, markiert die Aufgabe als fehlgeschlagen und zeigt den Fehler dem übergeordneten Element mit jedem Teilergebnis. Gilt nicht für synchrone Subagenten.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` mit `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: Bricht die Anfrage ab, wenn Header angekommen sind, aber der Antwortkörper nicht mehr streamt. Der Watchdog ist standardmäßig für alle Anbieter aktiviert; setzen Sie `CLAUDE_ENABLE_STREAM_WATCHDOG=0`, um ihn zu deaktivieren. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` hat einen Standard von `300000` und ist auf dieses Minimum begrenzt. Die abgebrochene Anfrage durchläuft den normalen Wiederholungspfad.

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
```

| Feld                       | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | Ja           | Muss `"preset"` sein, um einen Preset-System-Prompt zu verwenden                                                                                                                                                                                                                                                                                               |
| `preset`                   | Ja           | Muss `"claude_code"` sein, um den System-Prompt von Claude Code zu verwenden                                                                                                                                                                                                                                                                                   |
| `append`                   | Nein         | Zusätzliche Anweisungen, die an den Preset-System-Prompt angehängt werden                                                                                                                                                                                                                                                                                      |
| `exclude_dynamic_sections` | Nein         | Verschieben Sie sitzungsspezifischen Kontext wie Arbeitsverzeichnis, Git-Status und Memory-Pfade aus dem System-Prompt in die erste Benutzernachricht. Verbessert die Prompt-Cache-Wiederverwendung über Benutzer und Maschinen hinweg. Siehe [System-Prompts ändern](/docs/de/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |

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

| Wert        | Beschreibung                                             | Ort                           |
| :---------- | :------------------------------------------------------- | :---------------------------- |
| `"user"`    | Globale Benutzereinstellungen                            | `~/.claude/settings.json`     |
| `"project"` | Gemeinsame Projekteinstellungen (versionskontrolliert)   | `.claude/settings.json`       |
| `"local"`   | Lokale Projekteinstellungen (nicht versionskontrolliert) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Standardverhalten
</h4>

Wenn `setting_sources` weggelassen oder `None` ist, lädt `query()` die gleichen Dateisystem-Einstellungen wie die Claude Code CLI: Benutzer, Projekt und lokal. Verwaltete Richtlinieneinstellungen werden in allen Fällen geladen; Server-verwaltete Einstellungen werden abgerufen, wenn sich die Sitzung mit einer Organisationsanmeldedaten auf einer [berechtigten Konfiguration](/docs/de/server-managed-settings#platform-availability) authentifiziert. Siehe [Was settingSources nicht kontrolliert](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control) für Eingaben, die unabhängig von dieser Option gelesen werden, und wie man sie deaktiviert.

<h4 id="why-use-setting_sources">
  Warum setting\_sources verwenden
</h4>

**Dateisystem-Einstellungen deaktivieren:**

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
  Im Python SDK 0.1.59 und früher wurde eine leere Liste gleich behandelt wie das Weglassen der Option, daher hatte `setting_sources=[]` keine Auswirkung auf die Deaktivierung von Dateisystem-Einstellungen. Aktualisieren Sie auf eine neuere Version, wenn Sie benötigen, dass eine leere Liste wirksam wird. Das TypeScript SDK ist nicht betroffen.
</Note>

**Alle Dateisystem-Einstellungen explizit laden:**

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

**Nur bestimmte Einstellungsquellen laden:**

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

**Test- und CI-Umgebungen:**

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

**SDK-only-Anwendungen:**

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

**Laden von CLAUDE.md-Projektanweisungen:**

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
  Einstellungspriorität
</h4>

Wenn mehrere Quellen geladen werden, werden Einstellungen mit dieser Priorität zusammengeführt (höchste zu niedrigste):

1. Lokale Einstellungen (`.claude/settings.local.json`)
2. Projekteinstellungen (`.claude/settings.json`)
3. Benutzereinstellungen (`~/.claude/settings.json`)

Programmgesteuerte Optionen wie `agents` und `allowed_tools` überschreiben Benutzer-, Projekt- und lokale Dateisystem-Einstellungen. Verwaltete Richtlinieneinstellungen haben Vorrang vor programmgesteuerten Optionen.

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

| Feld              | Erforderlich | Beschreibung                                                                                                                                                                                                                                             |
| :---------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | Ja           | Natürlichsprachige Beschreibung, wann dieser Agent verwendet werden sollte                                                                                                                                                                               |
| `prompt`          | Ja           | Der System-Prompt des Agenten                                                                                                                                                                                                                            |
| `tools`           | Nein         | Array von zulässigen Tool-Namen. Wenn weggelassen, erbt alle Tools                                                                                                                                                                                       |
| `disallowedTools` | Nein         | Array von Tool-Namen, die aus dem Tool-Set des Agenten entfernt werden. MCP-Server-Level-Muster werden auch akzeptiert: `mcp__server` oder `mcp__server__*` entfernt jedes Tool von diesem Server, und `mcp__*` entfernt jedes MCP-Tool von jedem Server |
| `model`           | Nein         | Modell-Override für diesen Agenten. Akzeptiert einen Alias wie `"sonnet"`, `"opus"`, `"haiku"` oder `"inherit"`, oder eine vollständige Modell-ID. Wenn weggelassen, verwendet das Hauptmodell                                                           |
| `skills`          | Nein         | Liste von Skill-Namen, die diesem Agenten zur Verfügung stehen. Nicht aufgelistete Skills bleiben über das Skill-Tool aufrufbar                                                                                                                          |
| `memory`          | Nein         | Memory-Quelle für diesen Agenten: `"user"`, `"project"` oder `"local"`                                                                                                                                                                                   |
| `mcpServers`      | Nein         | MCP-Server, die diesem Agenten zur Verfügung stehen. Jeder Eintrag ist ein Servername oder ein Inline-`{name: config}`-Dict                                                                                                                              |
| `initialPrompt`   | Nein         | Wird automatisch als erste Benutzerdrehung eingereicht, wenn dieser Agent als Haupt-Thread-Agent läuft                                                                                                                                                   |
| `maxTurns`        | Nein         | Maximale Anzahl von Agenten-Umdrehungen, bevor der Agent stoppt                                                                                                                                                                                          |
| `background`      | Nein         | Führen Sie diesen Agenten als nicht-blockierende Hintergrundaufgabe aus, wenn aufgerufen                                                                                                                                                                 |
| `effort`          | Nein         | Reasoning-Anstrengungsstufe für diesen Agenten. Akzeptiert eine benannte Stufe oder eine Ganzzahl. Siehe [`EffortLevel`](#effortlevel)                                                                                                                   |
| `permissionMode`  | Nein         | Berechtigungsmodus für die Tool-Ausführung innerhalb dieses Agenten. Siehe [`PermissionMode`](#permissionmode)                                                                                                                                           |

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
    "auto",  # A model classifier approves or denies each tool call
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

Der Callback ist der SDK-Ersatz für die interaktive Berechtigungsaufforderung: Er wird nur aufgerufen, wenn der [Berechtigungsbewertungsfluss](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) zu einer Aufforderung führt. Tool-Aufrufe, die bereits von einem `allowed_tools`-Eintrag, einer Settings-Allow-Regel oder dem Berechtigungsmodus wie `acceptEdits` oder `bypassPermissions` genehmigt wurden, rufen ihn nie auf. Um jeden Tool-Aufruf zu kontrollieren, verwenden Sie stattdessen einen [`PreToolUse`-Hook](/docs/de/agent-sdk/hooks). `AskUserQuestion`, MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, und Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) erreichen den Callback auch, wenn eine Allow-Regel übereinstimmt. Im `dontAsk`-Modus werden diese Aufrufe stattdessen verweigert, ohne den Callback aufzurufen.

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Kontextinformationen, die an Tool-Berechtigungs-Callbacks übergeben werden.

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

| Feld              | Typ                      | Beschreibung                                                                                                                                                                                                                                                                  |
| :---------------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`          | `Any \| None`            | Reserviert für zukünftige Abort-Signal-Unterstützung                                                                                                                                                                                                                          |
| `suggestions`     | `list[PermissionUpdate]` | Berechtigungsaktualisierungsvorschläge von der CLI. Bash-Aufforderungen enthalten einen Vorschlag mit dem `localSettings`-Ziel, daher gibt das Zurückgeben in `updated_permissions` die Regel in `.claude/settings.local.json` aus und bleibt über Sitzungen hinweg bestehen. |
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

Das optionale Feld `display` steuert, ob Thinking-Text `"summarized"` oder `"omitted"` zurückgegeben wird. Bei Claude Opus 4.7 und später ist der API-Standard `"omitted"`, daher setzen Sie `"summarized"`, um Thinking-Inhalte in [`ThinkingBlock`](#thinkingblock)-Ausgaben zu erhalten.

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

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Literal-Typ für SDK-Beta-Funktionen.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Verwenden Sie mit dem Feld `betas` in `ClaudeAgentOptions`, um Beta-Funktionen zu aktivieren.

<Warning>
  Die `context-1m-2025-08-07`-Beta ist seit dem 30. April 2026 veraltet. Das Übergeben dieses Headers mit Claude Sonnet 4.5 oder Sonnet 4 hat keine Auswirkung, und Anfragen, die das Standard-200k-Token-Kontextfenster überschreiten, geben einen Fehler zurück. Um ein 1M-Token-Kontextfenster zu verwenden, migrieren Sie zu [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 oder Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), die 1M-Kontext zu Standardpreisen ohne Beta-Header enthalten.
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
```

| Feld                 | Typ                         | Beschreibung                                                     |
| :------------------- | :-------------------------- | :--------------------------------------------------------------- |
| `content`            | `str \| list[ContentBlock]` | Nachrichteninhalt als Text oder Inhaltsblöcke                    |
| `uuid`               | `str \| None`               | Eindeutige Nachrichtenkennung                                    |
| `parent_tool_use_id` | `str \| None`               | Tool-Use-ID, wenn diese Nachricht eine Tool-Ergebnis-Antwort ist |
| `tool_use_result`    | `dict[str, Any] \| None`    | Tool-Ergebnisdaten, falls zutreffend                             |

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
```

| Feld                 | Typ                                                          | Beschreibung                                                                                |
| :------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------------ |
| `content`            | `list[ContentBlock]`                                         | Liste von Inhaltsblöcken in der Antwort                                                     |
| `model`              | `str`                                                        | Modell, das die Antwort generiert hat                                                       |
| `parent_tool_use_id` | `str \| None`                                                | Tool-Use-ID, wenn dies eine verschachtelte Antwort ist                                      |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Fehlertyp, wenn die Antwort auf einen Fehler stieß                                          |
| `usage`              | `dict[str, Any] \| None`                                     | Token-Nutzung pro Nachricht (gleiche Schlüssel wie [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | API-Nachrichtenkennung. Mehrere Nachrichten aus einer Umdrehung teilen die gleiche ID       |

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
    "max_output_tokens",
    "unknown",
]
```

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
    model_usage: dict[str, Any] | None = None
    permission_denials: list[Any] | None = None
    deferred_tool_use: DeferredToolUse | None = None
    errors: list[str] | None = None
    api_error_status: int | None = None
    uuid: str | None = None
```

Das Feld `subtype` bestimmt, welche anderen Felder gefüllt werden. Es ist eines von `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"` oder `"error_max_structured_output_retries"`. Die Python-Dataclass vereinfacht alle Varianten in eine Form, daher sind Felder, die nicht auf den zurückgegebenen Subtyp zutreffen, `None`.

Mehrere Felder enthalten diagnostische Details, wenn das Gespräch mit einem Fehler endet:

* `is_error`: `True`, wenn das Gespräch in einem Fehlerzustand endete. Immer `True` bei den `error_*`-Subtypen. Bei `subtype="success"` ist es `True`, wenn die letzte Modellanfrage fehlgeschlagen ist, was bedeutet, dass die Agent-Schleife abgeschlossen wurde, aber der letzte API-Aufruf einen Fehler zurückgab.
* `api_error_status`: Der HTTP-Statuscode des beendenden API-Fehlers. `None`, wenn die Umdrehung ohne einen endete. Wird nur bei `subtype="success"` gefüllt.
* `result`: Text der endgültigen Assistent-Nachricht bei `subtype="success"` oder `None` bei den `error_*`-Subtypen. Wenn `subtype="success"` und `is_error=True`, enthält dies die API-Fehlerzeichenfolge, falls verfügbar, kann aber leer sein. Überprüfen Sie daher `api_error_status` und den vorherigen `AssistantMessage`-Inhalt für Details.
* `errors`: Fehlerzeichenfolgen auf Schleifenebene, wie die Max-Turns-Nachricht. Wird nur bei den `error_*`-Subtypen gefüllt.

Das `usage`-Dict enthält die folgenden Schlüssel, wenn vorhanden:

| Schlüssel                     | Typ   | Beschreibung                                                                                                                                                                                                                                 |
| ----------------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | Eingabe-Token, die von der Agent-Schleife auf oberster Ebene verbraucht werden. [Subagent-Token sind nicht enthalten](/docs/de/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); verwenden Sie `model_usage` für die Gesamtbaum-Abrechnung. |
| `output_tokens`               | `int` | Ausgabe-Token, die von der Agent-Schleife auf oberster Ebene generiert werden. Subagent-Token sind nicht enthalten.                                                                                                                          |
| `cache_creation_input_tokens` | `int` | Token, die zum Erstellen neuer Cache-Einträge verwendet wurden.                                                                                                                                                                              |
| `cache_read_input_tokens`     | `int` | Token, die aus vorhandenen Cache-Einträgen gelesen wurden.                                                                                                                                                                                   |

Das `model_usage`-Dict ordnet Modellnamen der Nutzung pro Modell zu. Die inneren Dict-Schlüssel verwenden camelCase, da der Wert unverändert vom zugrunde liegenden CLI-Prozess übergeben wird und dem TypeScript [`ModelUsage`](/docs/de/agent-sdk/typescript#modelusage)-Typ entspricht:

| Schlüssel                  | Typ     | Beschreibung                                                                                                                                                    |
| -------------------------- | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | Eingabe-Token für dieses Modell.                                                                                                                                |
| `outputTokens`             | `int`   | Ausgabe-Token für dieses Modell.                                                                                                                                |
| `cacheReadInputTokens`     | `int`   | Cache-Lese-Token für dieses Modell.                                                                                                                             |
| `cacheCreationInputTokens` | `int`   | Cache-Erstellungs-Token für dieses Modell.                                                                                                                      |
| `webSearchRequests`        | `int`   | Websuch-Anfragen, die von diesem Modell gestellt wurden.                                                                                                        |
| `costUSD`                  | `float` | Geschätzte Kosten in USD für dieses Modell, clientseitig berechnet. Siehe [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking) für Abrechnungsvorbehalt. |
| `contextWindow`            | `int`   | Kontextfenstergröße für dieses Modell.                                                                                                                          |
| `maxOutputTokens`          | `int`   | Maximale Ausgabe-Token-Grenze für dieses Modell.                                                                                                                |

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

<h2 id="content-block-types">
  Inhaltsblock-Typen
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Union-Typ aller Inhaltsblöcke.

```python theme={null}
ContentBlock = TextBlock | ThinkingBlock | ToolUseBlock | ToolResultBlock
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

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Basis-Ausnahmeklasse für alle SDK-Fehler.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

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
  Das TypeScript SDK unterstützt zusätzliche Hook-Ereignisse, die in Python noch nicht verfügbar sind: `SessionStart`, `SessionEnd`, `Setup`, `TeammateIdle`, `TaskCompleted`, `ConfigChange`, `WorktreeCreate`, `WorktreeRemove`, `PostToolBatch` und `MessageDisplay`.
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

Gibt ein [`HookJSONOutput`](#hookjsonoutput) zurück, das enthalten kann:

* `decision`: `"block"`, um die Aktion zu blockieren
* `systemMessage`: Warnmeldung, die dem Benutzer angezeigt wird
* `hookSpecificOutput`: Hook-spezifische Ausgabedaten

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
        None  # Timeout in seconds for all hooks in this matcher (default: 60)
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

| Feld              | Typ                             | Beschreibung                                                                           |
| :---------------- | :------------------------------ | :------------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Immer "PostToolUseFailure"                                                             |
| `tool_name`       | `str`                           | Name des Tools, das fehlgeschlagen ist                                                 |
| `tool_input`      | `dict[str, Any]`                | Eingabeparameter, die verwendet wurden                                                 |
| `tool_use_id`     | `str`                           | Eindeutige Kennung für diese Tool-Nutzung                                              |
| `error`           | `str`                           | Fehlermeldung aus der fehlgeschlagenen Ausführung                                      |
| `is_interrupt`    | `bool` (optional)               | Ob der Fehler durch eine Unterbrechung verursacht wurde                                |
| `agent_id`        | `str` (optional)                | Subagenten-Kennung, vorhanden, wenn der Hook innerhalb eines Subagenten ausgelöst wird |
| `agent_type`      | `str` (optional)                | Subagenten-Typ, vorhanden, wenn der Hook innerhalb eines Subagenten ausgelöst wird     |

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

Ein `TypedDict`, das den Hook-Ereignisnamen und ereignisspezifische Felder enthält. Die Form hängt vom `hookEventName`-Wert ab. Vollständige Details zu verfügbaren Feldern pro Hook-Ereignis finden Sie unter [Ausführung mit Hooks kontrollieren](/docs/de/agent-sdk/hooks#outputs).

Eine diskriminierte Union von ereignisspezifischen Ausgabetypen. Das Feld `hookEventName` bestimmt, welche Felder gültig sind.

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
  Tool-Eingabe-/Ausgabetypen
</h2>

Dokumentation von Eingabe-/Ausgabeschemas für alle integrierten Claude Code-Tools. Während das Python SDK diese nicht als Typen exportiert, stellen sie die Struktur von Tool-Eingaben und -Ausgaben in Nachrichten dar.

<h3 id="agent">
  Agent
</h3>

**Tool-Name:** `Agent` (früher `Task`, das immer noch als Alias akzeptiert wird)

**Eingabe:**

```python theme={null}
{
    "description": str,  # Eine kurze (3-5 Wörter) Beschreibung der Aufgabe
    "prompt": str,  # Die Aufgabe, die der Agent ausführen soll
    "subagent_type": str,  # Der Typ des spezialisierten Agenten, der verwendet werden soll
}
```

**Ausgabe:**

```python theme={null}
{
    "result": str,  # Endergebnis vom Subagenten
    "usage": dict | None,  # Token-Nutzungsstatistiken
    "total_cost_usd": float | None,  # Geschätzte Gesamtkosten in USD
    "duration_ms": int | None,  # Ausführungsdauer in Millisekunden
}
```

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
                }
            ],
            "multiSelect": bool,  # Auf true setzen, um mehrere Auswahlen zu ermöglichen
        }
    ],
    "answers": dict[str, str | list[str]] | None,
    # Benutzerantworten, die vom Berechtigungssystem ausgefüllt werden. Multi-Select-
    # Antworten können eine Liste von Labels oder eine kommagetrennte Zeichenkette sein
}
```

**Ausgabe:**

```python theme={null}
{
    "questions": [  # Die Fragen, die gestellt wurden
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Ordnet Fragetext der Antwortzeichenkette zu
    # Multi-Select-Antworten sind kommagetrennt
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
    "output": str,  # Kombinierte stdout- und stderr-Ausgabe
    "exitCode": int,  # Exit-Code des Befehls
    "killed": bool | None,  # Ob der Befehl aufgrund eines Timeouts beendet wurde
    "shellId": str | None,  # Shell-ID für Hintergrundprozesse
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
    "timeout_ms": int | None,  # Nach dieser Frist beenden (Standard 300000, max. 3600000)
    "persistent": bool | None,  # Für die Lebensdauer der Sitzung ausführen; mit TaskStop stoppen
}
```

**Ausgabe:**

```python theme={null}
{
    "taskId": str,  # ID der Background-Monitor-Aufgabe
    "timeoutMs": int,  # Timeout-Frist in Millisekunden (0 wenn persistent)
    "persistent": bool | None,  # True, wenn bis TaskStop oder Sitzungsende ausgeführt wird
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
  Ab Claude Code v2.1.142 ist `TodoWrite` standardmäßig deaktiviert. Verwenden Sie stattdessen `TaskCreate`, `TaskGet`, `TaskUpdate` und `TaskList`. Siehe [Zu Task-Tools migrieren](/docs/de/agent-sdk/todo-tracking#migrate-to-task-tools), um Ihren Überwachungscode zu aktualisieren, oder setzen Sie `CLAUDE_CODE_ENABLE_TASKS=0`, um zu `TodoWrite` zurückzukehren.
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

<h3 id="bashoutput">
  BashOutput
</h3>

**Tool-Name:** `BashOutput`

**Eingabe:**

```python theme={null}
{
    "bash_id": str,  # Die ID der Background-Shell
    "filter": str | None,  # Optionaler Regex zum Filtern von Ausgabezeilen
}
```

**Ausgabe:**

```python theme={null}
{
    "output": str,  # Neue Ausgabe seit der letzten Überprüfung
    "status": "running" | "completed" | "failed",  # Aktueller Shell-Status
    "exitCode": int | None,  # Exit-Code bei Abschluss
}
```

<h3 id="killbash">
  KillBash
</h3>

**Tool-Name:** `KillBash`

**Eingabe:**

```python theme={null}
{
    "shell_id": str  # Die ID der zu beendenden Background-Shell
}
```

**Ausgabe:**

```python theme={null}
{
    "message": str,  # Erfolgsmeldung
    "shell_id": str,  # ID der beendeten Shell
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

<h2 id="advanced-features-with-claudesdkclient">
  Erweiterte Funktionen mit ClaudeSDKClient
</h2>

<h3 id="building-a-continuous-conversation-interface">
  Erstellen einer kontinuierlichen Konversationsschnittstelle
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
  Verwendung von Hooks zur Verhaltensänderung
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
  Echtzeit-Fortschrittsüberwachung
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
  Beispielverwendung
</h2>

<h3 id="basic-file-operations-using-query">
  Grundlegende Dateivorgänge (mit query)
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
  Fehlerbehandlung
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
  Streaming-Modus mit Client
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
  Verwendung benutzerdefinierter Tools mit ClaudeSDKClient
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
| `excludedCommands`          | `list[str]`                                           | `[]`     | Befehle, die immer Sandbox-Beschränkungen umgehen (z. B. `["docker"]`). Diese werden automatisch ohne Modellbeteiligung unsandboxed ausgeführt                                                                                                                             |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`   | Erlauben Sie dem Modell, die Ausführung von Befehlen außerhalb der Sandbox anzufordern. Wenn `True`, kann das Modell `dangerouslyDisableSandbox` in der Tool-Eingabe setzen, was auf das [Berechtigungssystem](#permissions-fallback-for-unsandboxed-commands) zurückfällt |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`   | Netzwerkspezifische Sandbox-Konfiguration                                                                                                                                                                                                                                  |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`   | Konfigurieren Sie, welche Sandbox-Verstöße ignoriert werden sollen                                                                                                                                                                                                         |
| `enableWeakerNestedSandbox` | `bool`                                                | `False`  | Aktivieren Sie eine schwächere verschachtelte Sandbox für Kompatibilität                                                                                                                                                                                                   |

<Note>
  Die Sandbox hängt von der Plattformunterstützung ab und benötigt unter Linux Tools wie `bubblewrap` und `socat`. Standardmäßig werden Befehle, wenn `enabled` auf `True` gesetzt ist, aber die Sandbox nicht gestartet werden kann, unsandboxed mit einer Warnung auf stderr ausgeführt. Dieses Standardverhalten unterscheidet sich vom TypeScript SDK, wo `failIfUnavailable` standardmäßig auf `true` gesetzt ist.

  Setzen Sie `"failIfUnavailable": True` in Ihren Sandbox-Einstellungen, um stattdessen zu stoppen. Der Schlüssel ist noch nicht auf `SandboxSettings` deklariert, aber das SDK leitet ihn an Claude Code weiter, das ihn berücksichtigt. `query()` meldet dann eine `ResultMessage` mit `subtype="error_during_execution"` und den Grund in `errors`. Achten Sie auf diesen Subtyp, anstatt zu erwarten, dass `query()` vor dem Yielding von Nachrichten eine Ausnahme auslöst.
</Note>

<h4 id="example-usage-2">
  Beispielverwendung
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
  **Unix-Socket-Sicherheit**: Die Option `allowUnixSockets` kann Zugriff auf leistungsstarke Systemdienste gewähren. Zum Beispiel ermöglicht das Zulassen von `/var/run/docker.sock` effektiv vollständigen Host-Systemzugriff über die Docker-API und umgeht die Sandbox-Isolation. Erlauben Sie nur Unix-Sockets, die absolut notwendig sind, und verstehen Sie die Sicherheitsauswirkungen jedes einzelnen.
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

| Eigenschaft               | Typ         | Standard | Beschreibung                                                                                                                                                                                             |
| :------------------------ | :---------- | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `list[str]` | `[]`     | Domänennamen, auf die Sandbox-Prozesse zugreifen können                                                                                                                                                  |
| `deniedDomains`           | `list[str]` | `[]`     | Domänennamen, auf die Sandbox-Prozesse nicht zugreifen können. Hat Vorrang vor `allowedDomains`                                                                                                          |
| `allowManagedDomainsOnly` | `bool`      | `False`  | Nur verwaltete Einstellungen: Wenn in verwalteten Einstellungen gesetzt, ignorieren Sie `allowedDomains` aus nicht verwalteten Einstellungsquellen. Hat keine Auswirkung, wenn über SDK-Optionen gesetzt |
| `allowUnixSockets`        | `list[str]` | `[]`     | Unix-Socket-Pfade, auf die Prozesse zugreifen können (z. B. Docker-Socket)                                                                                                                               |
| `allowAllUnixSockets`     | `bool`      | `False`  | Erlauben Sie Zugriff auf alle Unix-Sockets                                                                                                                                                               |
| `allowLocalBinding`       | `bool`      | `False`  | Erlauben Sie Prozessen, sich an lokale Ports zu binden (z. B. für Dev-Server)                                                                                                                            |
| `allowMachLookup`         | `list[str]` | `[]`     | Nur macOS: XPC/Mach-Servicenamen zum Zulassen. Unterstützt ein nachfolgendes Platzhalterzeichen                                                                                                          |
| `httpProxyPort`           | `int`       | `None`   | HTTP-Proxy-Port für Netzwerkanfragen                                                                                                                                                                     |
| `socksProxyPort`          | `int`       | `None`   | SOCKS-Proxy-Port für Netzwerkanfragen                                                                                                                                                                    |

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

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Eine statische Liste von Befehlen, die immer automatisch die Sandbox umgehen (z. B. `["docker"]`). Das Modell hat keine Kontrolle darüber.
  * `allowUnsandboxedCommands`: Ermöglicht dem Modell, zur Laufzeit zu entscheiden, ob es die Ausführung außerhalb der Sandbox anfordert, indem es `dangerouslyDisableSandbox: True` in der Tool-Eingabe setzt.
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

Dieses Muster ermöglicht es Ihnen:

* **Modell-Anfragen prüfen**: Protokollieren Sie, wenn das Modell die Ausführung außerhalb der Sandbox anfordert
* **Allowlists implementieren**: Erlauben Sie nur bestimmten Befehlen, außerhalb der Sandbox ausgeführt zu werden
* **Genehmigungsworkflows hinzufügen**: Erfordern Sie explizite Autorisierung für privilegierte Operationen

<Warning>
  Befehle, die mit `dangerouslyDisableSandbox: True` ausgeführt werden, haben vollständigen Systemzugriff. Stellen Sie sicher, dass Ihr `can_use_tool`-Handler diese Anfragen sorgfältig validiert.

  Wenn `permission_mode` auf `bypassPermissions` gesetzt ist und `allow_unsandboxed_commands` aktiviert ist, kann das Modell autonom Befehle außerhalb der Sandbox ausführen, ohne dass Genehmigungsaufforderungen angezeigt werden. Diese Kombination ermöglicht dem Modell effektiv, die Sandbox-Isolation stillschweigend zu verlassen.
</Warning>

<h2 id="see-also">
  Siehe auch
</h2>

* [SDK-Übersicht](/docs/de/agent-sdk/overview) - Allgemeine SDK-Konzepte
* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript) - TypeScript SDK-Dokumentation
* [CLI-Referenz](/docs/de/cli-reference) - Befehlszeilenschnittstelle
* [Häufige Workflows](/docs/de/common-workflows) - Schritt-für-Schritt-Anleitungen
