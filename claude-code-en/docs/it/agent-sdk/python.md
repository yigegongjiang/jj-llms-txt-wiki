> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento SDK Agent - Python

> Riferimento API completo per Python Agent SDK, incluse tutte le funzioni, i tipi e le classi.

<h2 id="installation">
  Installazione
</h2>

Installare il pacchetto in un ambiente virtuale. Su recenti installazioni di Debian, Ubuntu e Homebrew Python, l'esecuzione di `pip install` su Python di sistema fallisce con `error: externally-managed-environment`.

```bash theme={null}
python3 -m venv .venv
source .venv/bin/activate
pip install claude-agent-sdk
```

Per uv, Windows PowerShell e configurazione della chiave API, vedere [Iniziare nella panoramica di Agent SDK](/docs/it/agent-sdk/overview#get-started).

<h2 id="choosing-between-query-and-claudesdkclient">
  Scelta tra `query()` e `ClaudeSDKClient`
</h2>

Python SDK fornisce due modi per interagire con Claude Code:

<h3 id="quick-comparison">
  Confronto rapido
</h3>

| Funzionalità                 | `query()`                                            | `ClaudeSDKClient`                |
| :--------------------------- | :--------------------------------------------------- | :------------------------------- |
| **Sessione**                 | Crea una nuova sessione per impostazione predefinita | Riutilizza la stessa sessione    |
| **Conversazione**            | Singolo scambio                                      | Più scambi nello stesso contesto |
| **Connessione**              | Gestita automaticamente                              | Controllo manuale                |
| **Input Streaming**          | ✅ Supportato                                         | ✅ Supportato                     |
| **Interruzioni**             | ❌ Non supportato                                     | ✅ Supportato                     |
| **Hooks**                    | ✅ Supportato                                         | ✅ Supportato                     |
| **Strumenti personalizzati** | ✅ Supportato                                         | ✅ Supportato                     |
| **Continua chat**            | Manuale tramite `continue_conversation` o `resume`   | ✅ Automatico                     |
| **Caso d'uso**               | Attività una tantum                                  | Conversazioni continue           |

<h3 id="when-to-use-query-one-off-tasks">
  Quando usare `query()` (attività una tantum)
</h3>

**Migliore per:**

* Domande una tantum dove non hai bisogno della cronologia della conversazione
* Attività indipendenti che non richiedono contesto da scambi precedenti
* Script di automazione semplici
* Quando vuoi un nuovo inizio ogni volta

<h3 id="when-to-use-claudesdkclient-continuous-conversation">
  Quando usare `ClaudeSDKClient` (conversazione continua)
</h3>

**Migliore per:**

* **Continuare conversazioni** - Quando hai bisogno che Claude ricordi il contesto
* **Domande di follow-up** - Costruire su risposte precedenti
* **Applicazioni interattive** - Interfacce chat, REPL
* **Logica guidata dalla risposta** - Quando l'azione successiva dipende dalla risposta di Claude
* **Controllo della sessione** - Gestire il ciclo di vita della conversazione in modo esplicito

<h2 id="functions">
  Funzioni
</h2>

<h3 id="query">
  `query()`
</h3>

Crea una nuova sessione per ogni interazione con Claude Code per impostazione predefinita. Restituisce un iteratore asincrono che produce messaggi man mano che arrivano. Ogni chiamata a `query()` inizia da zero senza memoria di interazioni precedenti a meno che non passiate `continue_conversation=True` o `resume` in [`ClaudeAgentOptions`](#claudeagentoptions). Vedi [Sessions](/docs/it/agent-sdk/sessions).

```python theme={null}
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None,
    transport: Transport | None = None
) -> AsyncIterator[Message]
```

<h4 id="parameters">
  Parametri
</h4>

| Parametro   | Tipo                         | Descrizione                                                                                       |
| :---------- | :--------------------------- | :------------------------------------------------------------------------------------------------ |
| `prompt`    | `str \| AsyncIterable[dict]` | Il prompt di input come stringa o iterabile asincrono per la modalità streaming                   |
| `options`   | `ClaudeAgentOptions \| None` | Oggetto di configurazione opzionale (per impostazione predefinita `ClaudeAgentOptions()` se None) |
| `transport` | `Transport \| None`          | Trasporto personalizzato opzionale per comunicare con il processo CLI                             |

<h4 id="returns">
  Restituisce
</h4>

Restituisce un `AsyncIterator[Message]` che produce messaggi dalla conversazione.

<h4 id="example-with-options">
  Esempio - Con opzioni
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

Decoratore per definire strumenti MCP con sicurezza dei tipi.

```python theme={null}
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any],
    annotations: ToolAnnotations | None = None
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

<h4 id="parameters-2">
  Parametri
</h4>

| Parametro      | Tipo                                            | Descrizione                                                                                     |
| :------------- | :---------------------------------------------- | :---------------------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Identificatore univoco per lo strumento                                                         |
| `description`  | `str`                                           | Descrizione leggibile di cosa fa lo strumento                                                   |
| `input_schema` | `type \| dict[str, Any]`                        | Schema che definisce i parametri di input dello strumento (vedi sotto)                          |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Annotazioni MCP dello strumento opzionali che forniscono suggerimenti comportamentali ai client |

<h4 id="input-schema-options">
  Opzioni dello schema di input
</h4>

1. **Mappatura di tipo semplice** (consigliato):

   ```python theme={null}
   {"text": str, "count": int, "enabled": bool}
   ```

2. **Formato JSON Schema** (per validazione complessa):
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
  Restituisce
</h4>

Una funzione decoratore che avvolge l'implementazione dello strumento e restituisce un'istanza `SdkMcpTool`.

<h4 id="example">
  Esempio
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

Riesportato da `mcp.types` (disponibile anche come `from claude_agent_sdk import ToolAnnotations`). Tutti i campi sono suggerimenti opzionali; i client non dovrebbero fare affidamento su di essi per decisioni di sicurezza.

| Campo             | Tipo           | Predefinito | Descrizione                                                                                                                                                              |
| :---------------- | :------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `str \| None`  | `None`      | Titolo leggibile per lo strumento                                                                                                                                        |
| `readOnlyHint`    | `bool \| None` | `False`     | Se `True`, lo strumento non modifica il suo ambiente                                                                                                                     |
| `destructiveHint` | `bool \| None` | `True`      | Se `True`, lo strumento può eseguire aggiornamenti distruttivi (significativo solo quando `readOnlyHint` è `False`)                                                      |
| `idempotentHint`  | `bool \| None` | `False`     | Se `True`, le chiamate ripetute con gli stessi argomenti non hanno effetto aggiuntivo (significativo solo quando `readOnlyHint` è `False`)                               |
| `openWorldHint`   | `bool \| None` | `True`      | Se `True`, lo strumento interagisce con entità esterne (ad esempio, ricerca web). Se `False`, il dominio dello strumento è chiuso (ad esempio, uno strumento di memoria) |

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

Crea un server MCP in-process che viene eseguito all'interno della tua applicazione Python.

```python theme={null}
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

<h4 id="parameters-3">
  Parametri
</h4>

| Parametro | Tipo                            | Predefinito | Descrizione                                                      |
| :-------- | :------------------------------ | :---------- | :--------------------------------------------------------------- |
| `name`    | `str`                           | -           | Identificatore univoco per il server                             |
| `version` | `str`                           | `"1.0.0"`   | Stringa della versione del server                                |
| `tools`   | `list[SdkMcpTool[Any]] \| None` | `None`      | Elenco di funzioni di strumento create con il decoratore `@tool` |

<h4 id="returns-3">
  Restituisce
</h4>

Restituisce un oggetto `McpSdkServerConfig` che può essere passato a `ClaudeAgentOptions.mcp_servers`.

<h4 id="example-2">
  Esempio
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

Elenca le sessioni passate con metadati. Filtra per directory di progetto o elenca le sessioni in tutti i progetti. Sincrono; restituisce immediatamente.

```python theme={null}
def list_sessions(
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0,
    include_worktrees: bool = True
) -> list[SDKSessionInfo]
```

<h4 id="parameters-4">
  Parametri
</h4>

| Parametro           | Tipo          | Predefinito | Descrizione                                                                                                    |
| :------------------ | :------------ | :---------- | :------------------------------------------------------------------------------------------------------------- |
| `directory`         | `str \| None` | `None`      | Directory per cui elencare le sessioni. Se omesso, restituisce le sessioni in tutti i progetti                 |
| `limit`             | `int \| None` | `None`      | Numero massimo di sessioni da restituire                                                                       |
| `offset`            | `int`         | `0`         | Numero di sessioni da saltare dall'inizio dei risultati ordinati. Usare con `limit` per la paginazione         |
| `include_worktrees` | `bool`        | `True`      | Quando `directory` si trova all'interno di un repository git, includi le sessioni da tutti i percorsi worktree |

<h4 id="return-type-sdksessioninfo">
  Tipo di ritorno: `SDKSessionInfo`
</h4>

| Proprietà       | Tipo          | Descrizione                                                                                         |
| :-------------- | :------------ | :-------------------------------------------------------------------------------------------------- |
| `session_id`    | `str`         | Identificatore di sessione univoco                                                                  |
| `summary`       | `str`         | Titolo di visualizzazione: titolo personalizzato, riepilogo generato automaticamente o primo prompt |
| `last_modified` | `int`         | Ora dell'ultima modifica in millisecondi dall'epoca                                                 |
| `file_size`     | `int \| None` | Dimensione del file di sessione in byte (`None` per backend di archiviazione remota)                |
| `custom_title`  | `str \| None` | Titolo della sessione impostato dall'utente                                                         |
| `first_prompt`  | `str \| None` | Primo prompt utente significativo nella sessione                                                    |
| `git_branch`    | `str \| None` | Ramo Git alla fine della sessione                                                                   |
| `cwd`           | `str \| None` | Directory di lavoro per la sessione                                                                 |
| `tag`           | `str \| None` | Tag della sessione impostato dall'utente (vedi [`tag_session()`](#tag_session))                     |
| `created_at`    | `int \| None` | Ora di creazione della sessione in millisecondi dall'epoca                                          |

<h4 id="example-3">
  Esempio
</h4>

Stampa le 10 sessioni più recenti per un progetto. I risultati sono ordinati per `last_modified` decrescente, quindi il primo elemento è il più recente. Ometti `directory` per cercare in tutti i progetti.

```python theme={null}
from claude_agent_sdk import list_sessions

for session in list_sessions(directory="/path/to/project", limit=10):
    print(f"{session.summary} ({session.session_id})")
```

<h3 id="get_session_messages">
  `get_session_messages()`
</h3>

Recupera i messaggi da una sessione passata. Sincrono; restituisce immediatamente.

```python theme={null}
def get_session_messages(
    session_id: str,
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0
) -> list[SessionMessage]
```

<h4 id="parameters-5">
  Parametri
</h4>

| Parametro    | Tipo          | Predefinito  | Descrizione                                                                 |
| :----------- | :------------ | :----------- | :-------------------------------------------------------------------------- |
| `session_id` | `str`         | obbligatorio | L'ID della sessione per cui recuperare i messaggi                           |
| `directory`  | `str \| None` | `None`       | Directory del progetto in cui cercare. Se omesso, cerca in tutti i progetti |
| `limit`      | `int \| None` | `None`       | Numero massimo di messaggi da restituire                                    |
| `offset`     | `int`         | `0`          | Numero di messaggi da saltare dall'inizio                                   |

<h4 id="return-type-sessionmessage">
  Tipo di ritorno: `SessionMessage`
</h4>

| Proprietà            | Tipo                           | Descrizione                         |
| :------------------- | :----------------------------- | :---------------------------------- |
| `type`               | `Literal["user", "assistant"]` | Ruolo del messaggio                 |
| `uuid`               | `str`                          | Identificatore di messaggio univoco |
| `session_id`         | `str`                          | Identificatore di sessione          |
| `message`            | `Any`                          | Contenuto del messaggio grezzo      |
| `parent_tool_use_id` | `None`                         | Riservato per uso futuro            |

<h4 id="example-4">
  Esempio
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

Legge i metadati per una singola sessione per ID senza scansionare la directory del progetto completo. Sincrono; restituisce immediatamente.

```python theme={null}
def get_session_info(
    session_id: str,
    directory: str | None = None,
) -> SDKSessionInfo | None
```

<h4 id="parameters-6">
  Parametri
</h4>

| Parametro    | Tipo          | Predefinito  | Descrizione                                                                                |
| :----------- | :------------ | :----------- | :----------------------------------------------------------------------------------------- |
| `session_id` | `str`         | obbligatorio | UUID della sessione da cercare                                                             |
| `directory`  | `str \| None` | `None`       | Percorso della directory del progetto. Se omesso, cerca in tutte le directory del progetto |

Restituisce [`SDKSessionInfo`](#return-type-sdksessioninfo), o `None` se la sessione non viene trovata.

<h4 id="example-5">
  Esempio
</h4>

Cerca i metadati di una singola sessione senza scansionare la directory del progetto. Utile quando hai già un ID di sessione da un'esecuzione precedente.

```python theme={null}
from claude_agent_sdk import get_session_info

info = get_session_info("550e8400-e29b-41d4-a716-446655440000")
if info:
    print(f"{info.summary} (branch: {info.git_branch}, tag: {info.tag})")
```

<h3 id="rename_session">
  `rename_session()`
</h3>

Rinomina una sessione aggiungendo una voce di titolo personalizzato. Le chiamate ripetute sono sicure; il titolo più recente vince. Sincrono.

```python theme={null}
def rename_session(
    session_id: str,
    title: str,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-7">
  Parametri
</h4>

| Parametro    | Tipo          | Predefinito  | Descrizione                                                                                |
| :----------- | :------------ | :----------- | :----------------------------------------------------------------------------------------- |
| `session_id` | `str`         | obbligatorio | UUID della sessione da rinominare                                                          |
| `title`      | `str`         | obbligatorio | Nuovo titolo. Deve essere non vuoto dopo la rimozione degli spazi bianchi                  |
| `directory`  | `str \| None` | `None`       | Percorso della directory del progetto. Se omesso, cerca in tutte le directory del progetto |

Genera `ValueError` se `session_id` non è un UUID valido o `title` è vuoto; `FileNotFoundError` se la sessione non può essere trovata.

<h4 id="example-6">
  Esempio
</h4>

Rinomina la sessione più recente in modo che sia più facile da trovare in seguito. Il nuovo titolo appare in [`SDKSessionInfo.custom_title`](#return-type-sdksessioninfo) nelle letture successive.

```python theme={null}
from claude_agent_sdk import list_sessions, rename_session

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    rename_session(sessions[0].session_id, "Refactor auth module")
```

<h3 id="tag_session">
  `tag_session()`
</h3>

Etichetta una sessione. Passa `None` per cancellare l'etichetta. Le chiamate ripetute sono sicure; l'etichetta più recente vince. Sincrono.

```python theme={null}
def tag_session(
    session_id: str,
    tag: str | None,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-8">
  Parametri
</h4>

| Parametro    | Tipo          | Predefinito  | Descrizione                                                                                 |
| :----------- | :------------ | :----------- | :------------------------------------------------------------------------------------------ |
| `session_id` | `str`         | obbligatorio | UUID della sessione da etichettare                                                          |
| `tag`        | `str \| None` | obbligatorio | Stringa di etichetta, o `None` per cancellare. Sanitizzato Unicode prima dell'archiviazione |
| `directory`  | `str \| None` | `None`       | Percorso della directory del progetto. Se omesso, cerca in tutte le directory del progetto  |

Genera `ValueError` se `session_id` non è un UUID valido o `tag` è vuoto dopo la sanitizzazione; `FileNotFoundError` se la sessione non può essere trovata.

<h4 id="example-7">
  Esempio
</h4>

Etichetta una sessione, quindi filtra per quell'etichetta in una lettura successiva. Passa `None` per cancellare un'etichetta esistente.

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
  Classi
</h2>

<h3 id="claudesdkclient">
  `ClaudeSDKClient`
</h3>

**Mantiene una sessione di conversazione in più scambi.** Questo è l'equivalente Python di come la funzione `query()` di TypeScript SDK funziona internamente - crea un oggetto client che può continuare le conversazioni.

<h4 id="key-features">
  Caratteristiche principali
</h4>

* **Continuità della sessione**: Mantiene il contesto della conversazione in più chiamate `query()`
* **Stessa conversazione**: La sessione conserva i messaggi precedenti
* **Supporto per interruzioni**: Può interrompere l'esecuzione a metà attività
* **Ciclo di vita esplicito**: Controlli quando la sessione inizia e termina
* **Flusso guidato dalla risposta**: Può reagire alle risposte e inviare follow-up
* **Strumenti personalizzati e hooks**: Supporta strumenti personalizzati (creati con il decoratore `@tool`) e hooks

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
  Metodi
</h4>

| Metodo                                    | Descrizione                                                                                                                                                             |
| :---------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `__init__(options)`                       | Inizializza il client con configurazione opzionale                                                                                                                      |
| `connect(prompt)`                         | Connettiti a Claude con un prompt iniziale opzionale o flusso di messaggi                                                                                               |
| `query(prompt, session_id)`               | Invia una nuova richiesta in modalità streaming                                                                                                                         |
| `receive_messages()`                      | Ricevi tutti i messaggi da Claude come iteratore asincrono                                                                                                              |
| `receive_response()`                      | Ricevi messaggi fino a e incluso un ResultMessage                                                                                                                       |
| `interrupt()`                             | Invia segnale di interruzione (funziona solo in modalità streaming)                                                                                                     |
| `set_permission_mode(mode)`               | Cambia la modalità di autorizzazione per la sessione corrente                                                                                                           |
| `set_model(model)`                        | Cambia il modello per la sessione corrente. Passa `None` per ripristinare il valore predefinito                                                                         |
| `rewind_files(user_message_id)`           | Ripristina i file al loro stato al messaggio utente specificato. Richiede `enable_file_checkpointing=True`. Vedi [File checkpointing](/docs/it/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Ottieni lo stato di tutti i server MCP configurati. Restituisce [`McpStatusResponse`](#mcpstatusresponse)                                                               |
| `reconnect_mcp_server(server_name)`       | Riprova a connettersi a un server MCP che ha fallito o è stato disconnesso                                                                                              |
| `toggle_mcp_server(server_name, enabled)` | Abilita o disabilita un server MCP a metà sessione. La disabilitazione rimuove i suoi strumenti                                                                         |
| `stop_task(task_id)`                      | Interrompi un'attività in background in esecuzione. Un [`TaskNotificationMessage`](#tasknotificationmessage) con stato `"stopped"` segue nel flusso di messaggi         |
| `get_server_info()`                       | Ottieni informazioni sul server incluso l'ID della sessione e le capacità                                                                                               |
| `disconnect()`                            | Disconnettiti da Claude                                                                                                                                                 |

<h4 id="context-manager-support">
  Supporto Context Manager
</h4>

Il client può essere utilizzato come context manager asincrono per la gestione automatica della connessione:

```python theme={null}
async with ClaudeSDKClient() as client:
    await client.query("Hello Claude")
    async for message in client.receive_response():
        print(message)
```

> **Importante:** Quando iteri sui messaggi, evita di usare `break` per uscire anticipatamente poiché questo può causare problemi di pulizia asyncio. Invece, lascia che l'iterazione si completi naturalmente o usa flag per tracciare quando hai trovato quello che cerchi.

<h4 id="example-continuing-a-conversation">
  Esempio - Continuare una conversazione
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
  Esempio - Input streaming con ClaudeSDKClient
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
  Esempio - Utilizzo di interruzioni
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
  **Comportamento del buffer dopo l'interruzione:** `interrupt()` invia un segnale di arresto ma non cancella il buffer dei messaggi. I messaggi già prodotti dall'attività interrotta, incluso il suo `ResultMessage` (con `subtype="error_during_execution"`), rimangono nel flusso. Devi drenare con `receive_response()` prima di leggere la risposta a una nuova query. Se invii una nuova query immediatamente dopo `interrupt()` e chiami `receive_response()` una sola volta, riceverai i messaggi dell'attività interrotta, non la risposta della nuova query.
</Note>

<h4 id="example-advanced-permission-control">
  Esempio - Controllo avanzato delle autorizzazioni
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
    # Non elencare anche gli strumenti gated in allowed_tools: le regole di autorizzazione approvano le chiamate prima che can_use_tool venga eseguito
    options = ClaudeAgentOptions(can_use_tool=custom_permission_handler)

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Update the system config file")

        async for message in client.receive_response():
            # Will use sandbox path instead
            print(message)


asyncio.run(main())
```

<h2 id="types">
  Tipi
</h2>

<Note>
  **`@dataclass` vs `TypedDict`:** Questo SDK utilizza due tipi di tipi. Le classi decorate con `@dataclass` (come `ResultMessage`, `AgentDefinition`, `TextBlock`) sono istanze di oggetti in fase di esecuzione e supportano l'accesso agli attributi: `msg.result`. Le classi definite con `TypedDict` (come `ThinkingConfigEnabled`, `McpStdioServerConfig`, `SyncHookJSONOutput`) sono **dicts semplici in fase di esecuzione** e richiedono l'accesso alle chiavi: `config["budget_tokens"]`, non `config.budget_tokens`. La sintassi di chiamata `ClassName(field=value)` funziona per entrambi, ma solo le dataclass producono oggetti con attributi.
</Note>

<h3 id="sdkmcptool">
  `SdkMcpTool`
</h3>

Definizione per uno strumento SDK MCP creato con il decoratore `@tool`.

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| Proprietà      | Tipo                                       | Descrizione                                                                                                           |
| :------------- | :----------------------------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                      | Identificatore univoco per lo strumento                                                                               |
| `description`  | `str`                                      | Descrizione leggibile                                                                                                 |
| `input_schema` | `type[T] \| dict[str, Any]`                | Schema per la validazione dell'input                                                                                  |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]` | Funzione asincrona che gestisce l'esecuzione dello strumento                                                          |
| `annotations`  | `ToolAnnotations \| None`                  | Annotazioni MCP dello strumento opzionali (ad es. `readOnlyHint`, `destructiveHint`, `openWorldHint`). Da `mcp.types` |

<h3 id="transport">
  `Transport`
</h3>

Classe base astratta per implementazioni di trasporto personalizzate. Usala per comunicare con il processo Claude su un canale personalizzato (ad esempio, una connessione remota invece di un subprocess locale).

<Warning>
  Questa è un'API interna di basso livello. L'interfaccia potrebbe cambiare nelle versioni future. Le implementazioni personalizzate devono essere aggiornate per corrispondere a eventuali modifiche dell'interfaccia.
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

| Metodo            | Descrizione                                                                   |
| :---------------- | :---------------------------------------------------------------------------- |
| `connect()`       | Connetti il trasporto e preparati per la comunicazione                        |
| `write(data)`     | Scrivi dati grezzi (JSON + newline) nel trasporto                             |
| `read_messages()` | Iteratore asincrono che produce messaggi JSON analizzati                      |
| `close()`         | Chiudi la connessione e pulisci le risorse                                    |
| `is_ready()`      | Restituisce `True` se il trasporto può inviare e ricevere                     |
| `end_input()`     | Chiudi il flusso di input (ad esempio, chiudi stdin per trasporti subprocess) |

Importazione: `from claude_agent_sdk import Transport`

<h3 id="claudeagentoptions">
  `ClaudeAgentOptions`
</h3>

Dataclass di configurazione per le query Claude Code.

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

| Proprietà                     | Tipo                                                                                  | Predefinito                                           | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :---------------------------- | :------------------------------------------------------------------------------------ | :---------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                                                | Configurazione degli strumenti. Usa `{"type": "preset", "preset": "claude_code"}` per gli strumenti predefiniti di Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                                                  | Strumenti da approvare automaticamente senza chiedere. Questo non limita Claude a solo questi strumenti; gli strumenti non elencati ricadono in `permission_mode` e `can_use_tool`. Usa `disallowed_tools` per bloccare gli strumenti. Vedi [Autorizzazioni](/docs/it/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                    |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptFile \| None`                               | `None`                                                | Configurazione del prompt di sistema. Passa una stringa per un prompt personalizzato, `{"type": "preset", "preset": "claude_code"}` per il prompt di sistema di Claude Code con `"append"` opzionale, o `{"type": "file", "path": "..."}` per caricare un prompt grande da disco. Vedi [`SystemPromptPreset`](#systempromptpreset) e [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                                                                                                                                    |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                                                  | Configurazioni del server MCP o percorso al file di configurazione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `strict_mcp_config`           | `bool`                                                                                | `False`                                               | Quando `True`, usa solo i server passati in `mcp_servers` e ignora il progetto `.mcp.json`, le impostazioni utente, i server MCP forniti dai plugin e i [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai). Mappa al flag CLI `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                                                | Modalità di autorizzazione per l'utilizzo dello strumento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `continue_conversation`       | `bool`                                                                                | `False`                                               | Continua la conversazione più recente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `resume`                      | `str \| None`                                                                         | `None`                                                | ID della sessione da riprendere                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `max_turns`                   | `int \| None`                                                                         | `None`                                                | Numero massimo di turni agentici (round trip di utilizzo dello strumento)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                                                | Interrompi la query quando la stima del costo lato client raggiunge questo valore in USD. Confrontato con la stessa stima di `total_cost_usd`; vedi [Traccia costo e utilizzo](/docs/it/agent-sdk/cost-tracking) per avvertenze di accuratezza                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                                                  | Strumenti da negare. Un nome semplice come `"Bash"` rimuove lo strumento dal contesto di Claude. Una regola con ambito come `"Bash(rm *)"` lascia lo strumento disponibile e nega le chiamate corrispondenti in ogni modalità di autorizzazione, incluso `bypassPermissions`. Vedi [Autorizzazioni](/docs/it/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                             |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                                               | Abilita il tracciamento dei cambiamenti dei file per il rewind. Vedi [File checkpointing](/docs/it/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `model`                       | `str \| None`                                                                         | `None`                                                | Alias del modello Claude o nome completo del modello. Vedi [valori accettati e ID specifici del provider](/docs/it/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `fallback_model`              | `str \| None`                                                                         | `None`                                                | Modello di fallback da utilizzare se il modello primario fallisce                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                                                  | Funzionalità beta da abilitare. Vedi [`SdkBeta`](#sdkbeta) per le opzioni disponibili                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                                                | Formato di output per risposte strutturate (ad es. `{"type": "json_schema", "schema": {...}}`). Vedi [Output strutturati](/docs/it/agent-sdk/structured-outputs) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                                                | Nome dello strumento MCP per i prompt di autorizzazione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                                                | Directory di lavoro corrente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                                                | Percorso personalizzato all'eseguibile CLI di Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `settings`                    | `str \| None`                                                                         | `None`                                                | Percorso al file di impostazioni                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                                                  | Directory aggiuntive a cui Claude può accedere                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `env`                         | `dict[str, str]`                                                                      | `{}`                                                  | Variabili di ambiente unite in cima all'ambiente del processo ereditato. Vedi [Variabili di ambiente](/docs/it/env-vars) per le variabili che la CLI sottostante legge, e [Gestisci risposte API lente o bloccate](#handle-slow-or-stalled-api-responses) per le variabili relative ai timeout                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                                                  | Argomenti CLI aggiuntivi da passare direttamente alla CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                                                | Byte massimi durante il buffering dell'stdout della CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                                          | *Deprecato* - Oggetto simile a un file per l'output di debug. Usa il callback `stderr` invece                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                                                | Funzione di callback per l'output stderr dalla CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                                                | Funzione di callback per l'autorizzazione dello strumento, invocata solo quando il [flusso di autorizzazione](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) ricade in un prompt. Non invocata per le chiamate auto-approvate da `allowed_tools`, regole di autorizzazione, o `permission_mode`. `AskUserQuestion`, strumenti connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) la raggiungono anche se li hai consentiti; in modalità `dontAsk` questi vengono negati invece. Vedi [`CanUseTool`](#canusetool) per i dettagli |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                                                | Configurazioni hook per intercettare gli eventi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `user`                        | `str \| None`                                                                         | `None`                                                | Identificatore utente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `include_partial_messages`    | `bool`                                                                                | `False`                                               | Includi eventi di streaming di messaggi parziali. Se abilitato, i messaggi [`StreamEvent`](#streamevent) vengono prodotti                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `include_hook_events`         | `bool`                                                                                | `False`                                               | Includi eventi del ciclo di vita dei hook nel flusso di messaggi come oggetti `HookEventMessage`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `fork_session`                | `bool`                                                                                | `False`                                               | Quando si riprende con `resume`, esegui il fork a un nuovo ID di sessione invece di continuare la sessione originale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                                                | Subagenti definiti programmaticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                                                  | Carica plugin personalizzati da percorsi locali. Vedi [Plugin](/docs/it/agent-sdk/plugins) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                                                | Configura il comportamento della sandbox a livello di programmazione. Vedi [Impostazioni sandbox](#sandboxsettings) per i dettagli                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (Impostazioni predefinite CLI: tutte le fonti) | Controlla quali impostazioni del filesystem caricare. Passa `[]` per disabilitare le impostazioni utente, progetto e locali. Le impostazioni della politica gestita vengono caricate indipendentemente; le impostazioni gestite dal server vengono recuperate quando la sessione si autentica con una credenziale organizzativa su una [configurazione idonea](/docs/it/server-managed-settings#platform-availability). Vedi [Usa le funzionalità di Claude Code](/docs/it/agent-sdk/claude-code-features#what-settingsources-does-not-control) per gli input che vengono letti indipendentemente da questa opzione, e come disabilitarli                                                                                 |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                                                | Skills disponibili per la sessione. Passa `"all"` per abilitare ogni skill scoperta, o un elenco di nomi di skill. Quando impostato, l'SDK aggiunge lo strumento Skill a `allowed_tools` automaticamente. Se passi anche `tools`, includi `"Skill"` in quell'elenco. Vedi [Skills](/docs/it/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                                                        |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                                                | *Deprecato* - Token massimi per i blocchi di pensiero. Usa `thinking` invece                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                                                | Controlla il comportamento del pensiero esteso. Ha la precedenza su `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                                                | Livello di sforzo per la profondità del pensiero. Vedi [regola il livello di sforzo](/docs/it/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `session_store`               | [`SessionStore`](/docs/it/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                                                | Specchia i trascritti di sessione in un backend esterno in modo che qualsiasi host possa riprenderli. Vedi [Persisti le sessioni nell'archiviazione esterna](/docs/it/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                                           | Quando eseguire il flush delle voci di trascritto mirrorato a `session_store`. `"batched"` esegue il flush una volta per turno o quando il buffer si riempie; `"eager"` attiva un flush in background dopo ogni frame. Ignorato quando `session_store` è `None`                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

<h4 id="handle-slow-or-stalled-api-responses">
  Gestisci risposte API lente o bloccate
</h4>

Il subprocess CLI legge diverse variabili di ambiente che controllano i timeout dell'API e il rilevamento dei blocchi. Passale attraverso `ClaudeAgentOptions.env`:

```python theme={null}
options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS`: timeout per richiesta sul client Anthropic, in millisecondi. Predefinito `600000`. Si applica al ciclo principale e a tutti i subagenti.
* `CLAUDE_CODE_MAX_RETRIES`: numero massimo di tentativi API. Predefinito `10`, limitato a `15`. Ogni tentativo ottiene la propria finestra `API_TIMEOUT_MS`, quindi il tempo di parete nel caso peggiore è approssimativamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` più backoff. Per esecuzioni incustodite che devono attendere interruzioni più lunghe, imposta `CLAUDE_CODE_RETRY_WATCHDOG=1`: ritenta gli errori di capacità indefinitamente, e a partire da Claude Code v2.1.199 aumenta il valore predefinito per altri errori transitori a `300` e rimuove il limite su questa variabile.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: watchdog di blocco per i subagenti lanciati con `run_in_background`. Predefinito `600000`. Si ripristina su ogni evento di flusso; in caso di blocco interrompe il subagente, contrassegna l'attività come non riuscita e presenta l'errore al genitore con qualsiasi risultato parziale. Non si applica ai subagenti sincroni.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` con `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: interrompe la richiesta quando le intestazioni sono arrivate ma il corpo della risposta smette di trasmettere. Il watchdog è attivo per impostazione predefinita per tutti i provider; imposta `CLAUDE_ENABLE_STREAM_WATCHDOG=0` per disabilitarlo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` predefinito a `300000` e viene bloccato a quel minimo. La richiesta interrotta passa attraverso il percorso di ripetizione normale.

<h3 id="outputformat">
  `OutputFormat`
</h3>

Configurazione per la validazione dell'output strutturato. Passa questo come `dict` al campo `output_format` su `ClaudeAgentOptions`:

```python theme={null}
# Expected dict shape for output_format
{
    "type": "json_schema",
    "schema": {...},  # Your JSON Schema definition
}
```

| Campo    | Obbligatorio | Descrizione                                                |
| :------- | :----------- | :--------------------------------------------------------- |
| `type`   | Sì           | Deve essere `"json_schema"` per la validazione JSON Schema |
| `schema` | Sì           | Definizione JSON Schema per la validazione dell'output     |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

Configurazione per l'utilizzo del prompt di sistema preset di Claude Code con aggiunte opzionali.

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
```

| Campo                      | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                       |
| :------------------------- | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | Sì           | Deve essere `"preset"` per utilizzare un prompt di sistema preset                                                                                                                                                                                                                                                                                                 |
| `preset`                   | Sì           | Deve essere `"claude_code"` per utilizzare il prompt di sistema di Claude Code                                                                                                                                                                                                                                                                                    |
| `append`                   | No           | Istruzioni aggiuntive da aggiungere al prompt di sistema preset                                                                                                                                                                                                                                                                                                   |
| `exclude_dynamic_sections` | No           | Sposta il contesto per sessione come directory di lavoro, il flag git-repo e i percorsi di memoria automatica dal prompt di sistema nel primo messaggio utente. Migliora il riutilizzo della cache dei prompt tra utenti e macchine. Vedi [Modifica i prompt di sistema](/docs/it/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

Configurazione per il caricamento di un prompt di sistema personalizzato da un file invece di passarlo come stringa. L'SDK mappa questo al flag CLI [`--system-prompt-file`](/docs/it/cli-reference#system-prompt-flags). Usa il modulo file quando il prompt è grande: l'SDK passa una stringa `system_prompt` sull'argv del subprocess CLI, che è soggetto ai limiti di lunghezza della riga di comando del sistema operativo prima che l'SDK invii qualsiasi richiesta API. Su Linux un singolo argomento più lungo di circa 128 KB fallisce al spawn del processo con `Argument list too long`. Su Windows l'intera riga di comando è limitata a circa 32 KB, quindi il modulo stringa fallisce a una soglia inferiore.

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| Campo  | Obbligatorio | Descrizione                                           |
| :----- | :----------- | :---------------------------------------------------- |
| `type` | Sì           | Deve essere `"file"` per caricare il prompt dal disco |
| `path` | Sì           | Percorso a un file contenente il prompt di sistema    |

<h3 id="settingsource">
  `SettingSource`
</h3>

Controlla quali fonti di configurazione basate su filesystem l'SDK carica le impostazioni da.

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| Valore      | Descrizione                                                      | Posizione                     |
| :---------- | :--------------------------------------------------------------- | :---------------------------- |
| `"user"`    | Impostazioni utente globali                                      | `~/.claude/settings.json`     |
| `"project"` | Impostazioni di progetto condivise (controllate dalla versione)  | `.claude/settings.json`       |
| `"local"`   | Impostazioni di progetto locali (non controllate dalla versione) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamento predefinito
</h4>

Quando `setting_sources` è omesso o `None`, `query()` carica le stesse impostazioni del filesystem della CLI di Claude Code: utente, progetto e locale. Le impostazioni della politica gestita vengono caricate in tutti i casi; le impostazioni gestite dal server vengono recuperate quando la sessione si autentica con una credenziale organizzativa su una [configurazione idonea](/docs/it/server-managed-settings#platform-availability). Vedi [Cosa settingSources non controlla](/docs/it/agent-sdk/claude-code-features#what-settingsources-does-not-control) per gli input che vengono letti indipendentemente da questa opzione, e come disabilitarli.

<h4 id="why-use-setting_sources">
  Perché usare setting\_sources
</h4>

**Disabilita le impostazioni del filesystem:**

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
  In Python SDK 0.1.59 e versioni precedenti, un elenco vuoto era trattato come l'omissione dell'opzione, quindi `setting_sources=[]` non disabilitava le impostazioni del filesystem. Aggiorna a una versione più recente se hai bisogno che un elenco vuoto abbia effetto. TypeScript SDK non è interessato.
</Note>

**Carica tutte le impostazioni del filesystem in modo esplicito:**

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

**Carica solo fonti di impostazioni specifiche:**

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

**Ambienti di test e CI:**

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

**Applicazioni solo SDK:**

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

**Caricamento delle istruzioni del progetto CLAUDE.md:**

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
  Precedenza delle impostazioni
</h4>

Quando più fonti vengono caricate, le impostazioni vengono unite con questa precedenza (da più alta a più bassa):

1. Impostazioni locali (`.claude/settings.local.json`)
2. Impostazioni di progetto (`.claude/settings.json`)
3. Impostazioni utente (`~/.claude/settings.json`)

Le opzioni programmatiche come `agents` e `allowed_tools` sovrascrivono le impostazioni del filesystem utente, progetto e locale. Le impostazioni della politica gestita hanno la precedenza sulle opzioni programmatiche.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Configurazione per un subagente definito programmaticamente.

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

| Campo             | Obbligatorio | Descrizione                                                                                                                                                                                                                                                         |
| :---------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`     | Sì           | Descrizione in linguaggio naturale di quando utilizzare questo agente                                                                                                                                                                                               |
| `prompt`          | Sì           | Il prompt di sistema dell'agente                                                                                                                                                                                                                                    |
| `tools`           | No           | Array di nomi di strumenti consentiti. Se omesso, eredita tutti gli strumenti                                                                                                                                                                                       |
| `disallowedTools` | No           | Array di nomi di strumenti da rimuovere dal set di strumenti dell'agente. Sono accettati anche i pattern a livello di server MCP: `mcp__server` o `mcp__server__*` rimuove ogni strumento da quel server, e `mcp__*` rimuove ogni strumento MCP da qualsiasi server |
| `model`           | No           | Override del modello per questo agente. Accetta un alias come `"sonnet"`, `"opus"`, `"haiku"`, o `"inherit"`, o un ID modello completo. Se omesso, utilizza il modello principale                                                                                   |
| `skills`          | No           | Elenco dei nomi di skills da precaricare nel contesto dell'agente all'avvio. Le skills non elencate rimangono invocabili attraverso lo strumento Skill                                                                                                              |
| `memory`          | No           | Fonte di memoria per questo agente: `"user"`, `"project"`, o `"local"`                                                                                                                                                                                              |
| `mcpServers`      | No           | Server MCP disponibili per questo agente. Ogni voce è un nome di server o un dict `{name: config}` inline                                                                                                                                                           |
| `initialPrompt`   | No           | Auto-inviato come il primo turno utente quando questo agente viene eseguito come agente del thread principale                                                                                                                                                       |
| `maxTurns`        | No           | Numero massimo di turni agentici prima che l'agente si fermi                                                                                                                                                                                                        |
| `background`      | No           | Esegui questo agente come attività in background non bloccante quando invocato                                                                                                                                                                                      |
| `effort`          | No           | Livello di sforzo di ragionamento per questo agente. Accetta un livello denominato o un numero intero. Vedi [`EffortLevel`](#effortlevel)                                                                                                                           |
| `permissionMode`  | No           | Modalità di autorizzazione per l'esecuzione dello strumento all'interno di questo agente. Vedi [`PermissionMode`](#permissionmode)                                                                                                                                  |

<Note>
  I nomi dei campi `AgentDefinition` usano camelCase, come `disallowedTools`, `permissionMode` e `maxTurns`. Questi nomi si mappano direttamente al formato wire condiviso con TypeScript SDK. Questo differisce da `ClaudeAgentOptions`, che usa Python snake\_case per i campi di livello superiore equivalenti come `disallowed_tools` e `permission_mode`. Poiché `AgentDefinition` è una dataclass, passare una parola chiave snake\_case genera un `TypeError` al momento della costruzione.
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

Modalità di autorizzazione per controllare l'esecuzione dello strumento.

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

Livelli di sforzo per guidare la profondità del pensiero.

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

Alias di tipo per le funzioni di callback di autorizzazione dello strumento.

```python theme={null}
CanUseTool = Callable[
    [str, dict[str, Any], ToolPermissionContext], Awaitable[PermissionResult]
]
```

Il callback riceve:

* `tool_name`: Nome dello strumento che viene chiamato
* `input_data`: I parametri di input dello strumento
* `context`: Un `ToolPermissionContext` con informazioni aggiuntive

Restituisce un `PermissionResult` (sia `PermissionResultAllow` che `PermissionResultDeny`).

Il callback è il sostituto SDK per il prompt di autorizzazione interattivo: viene invocato solo quando il [flusso di valutazione delle autorizzazioni](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) si risolve in un prompt. Le chiamate dello strumento già approvate da una voce `allowed_tools`, una regola di autorizzazione nelle impostazioni, o la modalità di autorizzazione, come `acceptEdits` o `bypassPermissions`, non lo invocano mai. Per controllare ogni chiamata dello strumento, usa un [hook `PreToolUse`](/docs/it/agent-sdk/hooks) invece.

`AskUserQuestion`, strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool), e strumenti connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) lo raggiungono anche quando una regola di autorizzazione corrisponde. In modalità `dontAsk` queste chiamate vengono negate invece, senza invocare il callback.

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Informazioni di contesto passate ai callback di autorizzazione dello strumento.

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

| Campo             | Tipo                     | Descrizione                                                                                                                                                                                                                                                          |
| :---------------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`          | `Any \| None`            | Riservato per il supporto futuro del segnale di interruzione                                                                                                                                                                                                         |
| `suggestions`     | `list[PermissionUpdate]` | Suggerimenti di aggiornamento delle autorizzazioni dalla CLI. I prompt Bash includono un suggerimento con la destinazione `localSettings`, quindi restituirlo in `updated_permissions` scrive la regola in `.claude/settings.local.json` e persiste tra le sessioni. |
| `blocked_path`    | `str \| None`            | Percorso del file che ha attivato la richiesta di autorizzazione, se applicabile. Ad esempio, quando un comando Bash tenta di accedere a un percorso al di fuori delle directory consentite                                                                          |
| `decision_reason` | `str \| None`            | Motivo per cui questa richiesta di autorizzazione è stata attivata. Inoltrato dal `permissionDecisionReason` di un hook PreToolUse quando l'hook ha restituito `"ask"`                                                                                               |
| `title`           | `str \| None`            | Frase completa del prompt di autorizzazione, come `Claude wants to read foo.txt`. Usa come testo del prompt principale quando presente                                                                                                                               |
| `display_name`    | `str \| None`            | Breve frase nominale per l'azione dello strumento, come `Read file`, adatta per etichette di pulsanti                                                                                                                                                                |
| `description`     | `str \| None`            | Sottotitolo leggibile per l'interfaccia utente di autorizzazione                                                                                                                                                                                                     |

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Tipo di unione per i risultati del callback di autorizzazione.

```python theme={null}
PermissionResult = PermissionResultAllow | PermissionResultDeny
```

<h3 id="permissionresultallow">
  `PermissionResultAllow`
</h3>

Risultato che indica che la chiamata dello strumento deve essere consentita.

```python theme={null}
@dataclass
class PermissionResultAllow:
    behavior: Literal["allow"] = "allow"
    updated_input: dict[str, Any] | None = None
    updated_permissions: list[PermissionUpdate] | None = None
```

| Campo                 | Tipo                             | Predefinito | Descrizione                                            |
| :-------------------- | :------------------------------- | :---------- | :----------------------------------------------------- |
| `behavior`            | `Literal["allow"]`               | `"allow"`   | Deve essere "allow"                                    |
| `updated_input`       | `dict[str, Any] \| None`         | `None`      | Input modificato da utilizzare al posto dell'originale |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None`      | Aggiornamenti delle autorizzazioni da applicare        |

<h3 id="permissionresultdeny">
  `PermissionResultDeny`
</h3>

Risultato che indica che la chiamata dello strumento deve essere negata.

```python theme={null}
@dataclass
class PermissionResultDeny:
    behavior: Literal["deny"] = "deny"
    message: str = ""
    interrupt: bool = False
```

| Campo       | Tipo              | Predefinito | Descrizione                                             |
| :---------- | :---------------- | :---------- | :------------------------------------------------------ |
| `behavior`  | `Literal["deny"]` | `"deny"`    | Deve essere "deny"                                      |
| `message`   | `str`             | `""`        | Messaggio che spiega perché lo strumento è stato negato |
| `interrupt` | `bool`            | `False`     | Se interrompere l'esecuzione corrente                   |

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Configurazione per l'aggiornamento delle autorizzazioni a livello di programmazione.

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

| Campo         | Tipo                                      | Descrizione                                                 |
| :------------ | :---------------------------------------- | :---------------------------------------------------------- |
| `type`        | `Literal[...]`                            | Il tipo di operazione di aggiornamento delle autorizzazioni |
| `rules`       | `list[PermissionRuleValue] \| None`       | Regole per le operazioni add/replace/remove                 |
| `behavior`    | `Literal["allow", "deny", "ask"] \| None` | Comportamento per le operazioni basate su regole            |
| `mode`        | `PermissionMode \| None`                  | Modalità per l'operazione setMode                           |
| `directories` | `list[str] \| None`                       | Directory per le operazioni add/remove directory            |
| `destination` | `Literal[...] \| None`                    | Dove applicare l'aggiornamento delle autorizzazioni         |

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

Una regola da aggiungere, sostituire o rimuovere in un aggiornamento delle autorizzazioni.

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

<h3 id="toolspreset">
  `ToolsPreset`
</h3>

Configurazione degli strumenti preset per l'utilizzo del set di strumenti predefinito di Claude Code.

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Controlla il comportamento del pensiero esteso. Un'unione di tre configurazioni:

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

| Variante   | Campi                              | Descrizione                                          |
| :--------- | :--------------------------------- | :--------------------------------------------------- |
| `adaptive` | `type`, `display`                  | Claude decide adattivamente quando pensare           |
| `enabled`  | `type`, `budget_tokens`, `display` | Abilita il pensiero con un budget di token specifico |
| `disabled` | `type`                             | Disabilita il pensiero                               |

Il campo opzionale `display` controlla se il testo di pensiero viene restituito `"summarized"` o `"omitted"`. Su Claude Opus 4.7 e versioni successive, l'impostazione predefinita dell'API è `"omitted"`, quindi imposta `"summarized"` per ricevere il contenuto di pensiero negli output [`ThinkingBlock`](#thinkingblock).

Poiché queste sono classi `TypedDict`, sono dicts semplici in fase di esecuzione. Costruiscile come letterali dict o chiama la classe come costruttore; entrambi producono un `dict`. Accedi ai campi con `config["budget_tokens"]`, non `config.budget_tokens`:

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

Tipo letterale per le funzionalità beta dell'SDK.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Usa con il campo `betas` in `ClaudeAgentOptions` per abilitare le funzionalità beta.

<Warning>
  La beta `context-1m-2025-08-07` è ritirata a partire dal 30 aprile 2026. Passare questo header con Claude Sonnet 4.5 o Sonnet 4 non ha effetto, e le richieste che superano la finestra di contesto standard di 200k token restituiscono un errore. Per utilizzare una finestra di contesto di 1M token, esegui la migrazione a [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, o Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), che includono 1M di contesto a prezzi standard senza header beta richiesto.
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

Configurazione per i server MCP dell'SDK creati con `create_sdk_mcp_server()`.

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Tipo di unione per le configurazioni del server MCP.

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

La configurazione di un server MCP come riportato da [`get_mcp_status()`](#methods). Questa è l'unione di tutte le varianti di trasporto [`McpServerConfig`](#mcpserverconfig) più una variante di output-only `claudeai-proxy` per i server proxy attraverso claude.ai.

```python theme={null}
McpServerStatusConfig = (
    McpStdioServerConfig
    | McpSSEServerConfig
    | McpHttpServerConfig
    | McpSdkServerConfigStatus
    | McpClaudeAIProxyServerConfig
)
```

`McpSdkServerConfigStatus` è la forma serializzabile di [`McpSdkServerConfig`](#mcpsdkserverconfig) con solo i campi `type` (`"sdk"`) e `name` (`str`); l'`instance` in-process viene omesso. `McpClaudeAIProxyServerConfig` ha i campi `type` (`"claudeai-proxy"`), `url` (`str`), e `id` (`str`).

<h3 id="mcpstatusresponse">
  `McpStatusResponse`
</h3>

Risposta da [`ClaudeSDKClient.get_mcp_status()`](#methods). Avvolge l'elenco degli stati del server sotto la chiave `mcpServers`.

```python theme={null}
class McpStatusResponse(TypedDict):
    mcpServers: list[McpServerStatus]
```

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Stato di un server MCP connesso, contenuto in [`McpStatusResponse`](#mcpstatusresponse).

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

| Campo        | Tipo                                                          | Descrizione                                                                                                                                                                           |
| :----------- | :------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`       | `str`                                                         | Nome del server                                                                                                                                                                       |
| `status`     | `str`                                                         | Uno di `"connected"`, `"failed"`, `"needs-auth"`, `"pending"`, o `"disabled"`                                                                                                         |
| `serverInfo` | `dict` (opzionale)                                            | Nome e versione del server (`{"name": str, "version": str}`)                                                                                                                          |
| `error`      | `str` (opzionale)                                             | Messaggio di errore se il server non si è connesso                                                                                                                                    |
| `config`     | [`McpServerStatusConfig`](#mcpserverstatusconfig) (opzionale) | Configurazione del server. Stessa forma di [`McpServerConfig`](#mcpserverconfig) (stdio, SSE, HTTP, o SDK), più una variante `claudeai-proxy` per i server connessi tramite claude.ai |
| `scope`      | `str` (opzionale)                                             | Ambito di configurazione                                                                                                                                                              |
| `tools`      | `list` (opzionale)                                            | Strumenti forniti da questo server, ognuno con i campi `name`, `description`, e `annotations`                                                                                         |

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Configurazione per il caricamento dei plugin nell'SDK.

```python theme={null}
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| Campo  | Tipo               | Descrizione                                                            |
| :----- | :----------------- | :--------------------------------------------------------------------- |
| `type` | `Literal["local"]` | Deve essere `"local"` (attualmente sono supportati solo plugin locali) |
| `path` | `str`              | Percorso assoluto o relativo alla directory del plugin                 |

**Esempio:**

```python theme={null}
plugins = [
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"},
]
```

Per informazioni complete sulla creazione e l'utilizzo dei plugin, vedi [Plugin](/docs/it/agent-sdk/plugins).

<h2 id="message-types">
  Tipi di messaggio
</h2>

<h3 id="message">
  `Message`
</h3>

Tipo di unione di tutti i possibili messaggi.

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

Messaggio di input dell'utente.

```python theme={null}
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
    uuid: str | None = None
    parent_tool_use_id: str | None = None
    tool_use_result: dict[str, Any] | None = None
```

| Campo                | Tipo                        | Descrizione                                                                                    |
| :------------------- | :-------------------------- | :--------------------------------------------------------------------------------------------- |
| `content`            | `str \| list[ContentBlock]` | Contenuto del messaggio come testo o blocchi di contenuto                                      |
| `uuid`               | `str \| None`               | Identificatore di messaggio univoco                                                            |
| `parent_tool_use_id` | `str \| None`               | ID di utilizzo dello strumento se questo messaggio è una risposta al risultato dello strumento |
| `tool_use_result`    | `dict[str, Any] \| None`    | Dati del risultato dello strumento se applicabile                                              |

<h3 id="assistantmessage">
  `AssistantMessage`
</h3>

Messaggio di risposta dell'assistente con blocchi di contenuto.

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

| Campo                | Tipo                                                         | Descrizione                                                                                 |
| :------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------------ |
| `content`            | `list[ContentBlock]`                                         | Elenco di blocchi di contenuto nella risposta                                               |
| `model`              | `str`                                                        | Modello che ha generato la risposta                                                         |
| `parent_tool_use_id` | `str \| None`                                                | ID di utilizzo dello strumento se questa è una risposta nidificata                          |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Tipo di errore se la risposta ha riscontrato un errore                                      |
| `usage`              | `dict[str, Any] \| None`                                     | Utilizzo dei token per messaggio (stesse chiavi di [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | ID del messaggio API. Più messaggi da un turno condividono lo stesso ID                     |

<h3 id="assistantmessageerror">
  `AssistantMessageError`
</h3>

Possibili tipi di errore per i messaggi dell'assistente.

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

Messaggio di sistema con metadati.

```python theme={null}
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

<h3 id="resultmessage">
  `ResultMessage`
</h3>

Messaggio di risultato finale con informazioni su costo e utilizzo.

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

Il campo `subtype` determina quali altri campi sono popolati. È uno di `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"`, o `"error_max_structured_output_retries"`. La dataclass Python appiattisce tutte le varianti in una forma, quindi i campi che non si applicano al subtype restituito sono `None`.

Diversi campi portano dettagli diagnostici quando la conversazione termina con un errore:

* `is_error`: `True` quando la conversazione è terminata in uno stato di errore. Sempre `True` sui subtype `error_*`. Su `subtype="success"` è `True` quando la richiesta del modello finale ha fallito, il che significa che il ciclo dell'agente è stato completato ma l'ultima chiamata API ha restituito un errore.
* `api_error_status`: il codice di stato HTTP dell'errore API terminale. `None` quando il turno è terminato senza uno. Popolato solo su `subtype="success"`.
* `result`: testo del messaggio dell'assistente finale su `subtype="success"`, o `None` sui subtype `error_*`. Quando `subtype="success"` e `is_error=True`, questo contiene la stringa di errore API se disponibile ma può essere vuoto, quindi controlla `api_error_status` e il contenuto di `AssistantMessage` precedente per i dettagli.
* `errors`: stringhe di errore a livello di ciclo come il messaggio max-turns. Popolato solo sui subtype `error_*`.

Il dict `usage` contiene le seguenti chiavi quando presenti:

| Chiave                        | Tipo  | Descrizione                                                                                                                                                                                                                        |
| ----------------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | Token di input consumati dal ciclo dell'agente di livello superiore. [I token dei subagenti non sono inclusi](/docs/it/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); usa `model_usage` per la contabilità dell'intero albero. |
| `output_tokens`               | `int` | Token di output generati dal ciclo dell'agente di livello superiore. I token dei subagenti non sono inclusi.                                                                                                                       |
| `cache_creation_input_tokens` | `int` | Token utilizzati per creare nuove voci di cache.                                                                                                                                                                                   |
| `cache_read_input_tokens`     | `int` | Token letti dalle voci di cache esistenti.                                                                                                                                                                                         |

Il dict `model_usage` mappa i nomi dei modelli all'utilizzo per modello. Le chiavi del dict interno usano camelCase perché il valore viene passato senza modifiche dal processo CLI sottostante, corrispondendo al tipo TypeScript [`ModelUsage`](/docs/it/agent-sdk/typescript#modelusage):

| Chiave                     | Tipo    | Descrizione                                                                                                                                                  |
| -------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `inputTokens`              | `int`   | Token di input per questo modello.                                                                                                                           |
| `outputTokens`             | `int`   | Token di output per questo modello.                                                                                                                          |
| `cacheReadInputTokens`     | `int`   | Token di lettura della cache per questo modello.                                                                                                             |
| `cacheCreationInputTokens` | `int`   | Token di creazione della cache per questo modello.                                                                                                           |
| `webSearchRequests`        | `int`   | Richieste di ricerca web effettuate da questo modello.                                                                                                       |
| `costUSD`                  | `float` | Costo stimato in USD per questo modello, calcolato lato client. Vedi [Traccia costo e utilizzo](/docs/it/agent-sdk/cost-tracking) per avvertenze di fatturazione. |
| `contextWindow`            | `int`   | Dimensione della finestra di contesto per questo modello.                                                                                                    |
| `maxOutputTokens`          | `int`   | Limite massimo di token di output per questo modello.                                                                                                        |

<h3 id="streamevent">
  `StreamEvent`
</h3>

Evento di flusso per aggiornamenti di messaggi parziali durante lo streaming. Ricevuto solo quando `include_partial_messages=True` in `ClaudeAgentOptions`. Importa tramite `from claude_agent_sdk.types import StreamEvent`.

```python theme={null}
@dataclass
class StreamEvent:
    uuid: str
    session_id: str
    event: dict[str, Any]  # The raw Claude API stream event
    parent_tool_use_id: str | None = None
```

| Campo                | Tipo             | Descrizione                                                                                                                                                                                   |
| :------------------- | :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `uuid`               | `str`            | Identificatore univoco per questo evento                                                                                                                                                      |
| `session_id`         | `str`            | Identificatore di sessione                                                                                                                                                                    |
| `event`              | `dict[str, Any]` | I dati dell'evento di flusso dell'API Claude grezzo                                                                                                                                           |
| `parent_tool_use_id` | `str \| None`    | Sempre `None`. Gli eventi di flusso vengono emessi solo per la sessione principale. Per l'attribuzione dei subagenti, utilizza messaggi completi come [`AssistantMessage`](#assistantmessage) |

<h3 id="ratelimitevent">
  `RateLimitEvent`
</h3>

Emesso quando lo stato del limite di velocità cambia (ad esempio, da `"allowed"` a `"allowed_warning"`). Usalo per avvertire gli utenti prima che raggiungano un limite rigido, o per fare backoff quando lo stato è `"rejected"`.

```python theme={null}
@dataclass
class RateLimitEvent:
    rate_limit_info: RateLimitInfo
    uuid: str
    session_id: str
```

| Campo             | Tipo                              | Descrizione                           |
| :---------------- | :-------------------------------- | :------------------------------------ |
| `rate_limit_info` | [`RateLimitInfo`](#ratelimitinfo) | Stato del limite di velocità corrente |
| `uuid`            | `str`                             | Identificatore di evento univoco      |
| `session_id`      | `str`                             | Identificatore di sessione            |

<h3 id="ratelimitinfo">
  `RateLimitInfo`
</h3>

Stato del limite di velocità trasportato da [`RateLimitEvent`](#ratelimitevent).

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

| Campo                     | Tipo                      | Descrizione                                                                                                                 |
| :------------------------ | :------------------------ | :-------------------------------------------------------------------------------------------------------------------------- |
| `status`                  | `RateLimitStatus`         | Stato corrente. `"allowed_warning"` significa avvicinarsi al limite; `"rejected"` significa che il limite è stato raggiunto |
| `resets_at`               | `int \| None`             | Timestamp Unix quando la finestra del limite di velocità si ripristina                                                      |
| `rate_limit_type`         | `RateLimitType \| None`   | Quale finestra del limite di velocità si applica                                                                            |
| `utilization`             | `float \| None`           | Frazione del limite di velocità consumato (0.0 a 1.0)                                                                       |
| `overage_status`          | `RateLimitStatus \| None` | Stato dell'utilizzo di overage pay-as-you-go, se applicabile                                                                |
| `overage_resets_at`       | `int \| None`             | Timestamp Unix quando la finestra di overage si ripristina                                                                  |
| `overage_disabled_reason` | `str \| None`             | Perché l'overage non è disponibile, se lo stato è `"rejected"`                                                              |
| `raw`                     | `dict[str, Any]`          | Dict grezzo completo dalla CLI, inclusi i campi non modellati sopra                                                         |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

Emesso quando un'attività in background inizia. Un'attività in background è qualsiasi cosa tracciata al di fuori del turno principale: un comando Bash in background, un watch Monitor, un subagente generato tramite lo strumento Agent, o un agente remoto. Il campo `task_type` ti dice quale. Questo nome non è correlato al rinomina dello strumento `Task`-to-`Agent`.

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

| Campo         | Tipo          | Descrizione                                                                                                                      |
| :------------ | :------------ | :------------------------------------------------------------------------------------------------------------------------------- |
| `task_id`     | `str`         | Identificatore univoco per l'attività                                                                                            |
| `description` | `str`         | Descrizione dell'attività                                                                                                        |
| `uuid`        | `str`         | Identificatore di messaggio univoco                                                                                              |
| `session_id`  | `str`         | Identificatore di sessione                                                                                                       |
| `tool_use_id` | `str \| None` | ID di utilizzo dello strumento associato                                                                                         |
| `task_type`   | `str \| None` | Quale tipo di attività in background: `"local_bash"` per Bash in background e watch Monitor, `"local_agent"`, o `"remote_agent"` |

<h3 id="taskusage">
  `TaskUsage`
</h3>

Dati di token e timing per un'attività in background.

```python theme={null}
class TaskUsage(TypedDict):
    total_tokens: int
    tool_uses: int
    duration_ms: int
```

<h3 id="taskprogressmessage">
  `TaskProgressMessage`
</h3>

Emesso periodicamente con aggiornamenti di progresso per un'attività in background in esecuzione.

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

| Campo            | Tipo          | Descrizione                                         |
| :--------------- | :------------ | :-------------------------------------------------- |
| `task_id`        | `str`         | Identificatore univoco per l'attività               |
| `description`    | `str`         | Descrizione dello stato corrente                    |
| `usage`          | `TaskUsage`   | Utilizzo dei token per questa attività finora       |
| `uuid`           | `str`         | Identificatore di messaggio univoco                 |
| `session_id`     | `str`         | Identificatore di sessione                          |
| `tool_use_id`    | `str \| None` | ID di utilizzo dello strumento associato            |
| `last_tool_name` | `str \| None` | Nome dell'ultimo strumento utilizzato dall'attività |

<h3 id="tasknotificationmessage">
  `TaskNotificationMessage`
</h3>

Emesso quando un'attività in background si completa, fallisce o viene interrotta. Le attività in background includono comandi Bash `run_in_background`, watch Monitor e subagenti in background.

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

| Campo         | Tipo                     | Descrizione                                     |
| :------------ | :----------------------- | :---------------------------------------------- |
| `task_id`     | `str`                    | Identificatore univoco per l'attività           |
| `status`      | `TaskNotificationStatus` | Uno di `"completed"`, `"failed"`, o `"stopped"` |
| `output_file` | `str`                    | Percorso al file di output dell'attività        |
| `summary`     | `str`                    | Riepilogo del risultato dell'attività           |
| `uuid`        | `str`                    | Identificatore di messaggio univoco             |
| `session_id`  | `str`                    | Identificatore di sessione                      |
| `tool_use_id` | `str \| None`            | ID di utilizzo dello strumento associato        |
| `usage`       | `TaskUsage \| None`      | Utilizzo dei token finale per l'attività        |

<h2 id="content-block-types">
  Tipi di blocco di contenuto
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Tipo di unione di tutti i blocchi di contenuto.

```python theme={null}
ContentBlock = TextBlock | ThinkingBlock | ToolUseBlock | ToolResultBlock
```

<h3 id="textblock">
  `TextBlock`
</h3>

Blocco di contenuto di testo.

```python theme={null}
@dataclass
class TextBlock:
    text: str
```

<h3 id="thinkingblock">
  `ThinkingBlock`
</h3>

Blocco di contenuto di pensiero (per modelli con capacità di pensiero).

```python theme={null}
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

<h3 id="tooluseblock">
  `ToolUseBlock`
</h3>

Blocco di richiesta di utilizzo dello strumento.

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

Blocco di risultato dell'esecuzione dello strumento.

```python theme={null}
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

<h2 id="error-types">
  Tipi di errore
</h2>

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Classe di eccezione base per tutti gli errori dell'SDK.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

<h3 id="clinotfounderror">
  `CLINotFoundError`
</h3>

Generato quando Claude Code CLI non è installato o non viene trovato.

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

Generato quando la connessione a Claude Code fallisce.

```python theme={null}
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

<h3 id="processerror">
  `ProcessError`
</h3>

Generato quando il processo Claude Code fallisce.

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

Generato quando l'analisi JSON fallisce.

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
  Tipi di Hook
</h2>

Per una guida completa sull'utilizzo degli hooks con esempi e modelli comuni, vedi la [guida Hooks](/docs/it/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Tipi di evento hook supportati.

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
  TypeScript SDK supporta eventi hook aggiuntivi non ancora disponibili in Python: `SessionStart`, `SessionEnd`, `Setup`, `TeammateIdle`, `TaskCompleted`, `ConfigChange`, `WorktreeCreate`, `WorktreeRemove`, `PostToolBatch` e `MessageDisplay`.
</Note>

<h3 id="hookcallback">
  `HookCallback`
</h3>

Definizione di tipo per le funzioni di callback hook.

```python theme={null}
HookCallback = Callable[[HookInput, str | None, HookContext], Awaitable[HookJSONOutput]]
```

Parametri:

* `input`: Input hook fortemente tipizzato con unioni discriminate basate su `hook_event_name` (vedi [`HookInput`](#hookinput))
* `tool_use_id`: Identificatore di utilizzo dello strumento opzionale (per hook correlati allo strumento)
* `context`: Contesto hook con informazioni aggiuntive

Restituisce un [`HookJSONOutput`](#hookjsonoutput) che può contenere:

* `decision`: `"block"` per bloccare l'azione
* `systemMessage`: Messaggio di avviso mostrato all'utente
* `hookSpecificOutput`: Dati di output specifici dell'hook

<h3 id="hookcontext">
  `HookContext`
</h3>

Informazioni di contesto passate ai callback hook.

```python theme={null}
class HookContext(TypedDict):
    signal: Any | None  # Future: abort signal support
```

<h3 id="hookmatcher">
  `HookMatcher`
</h3>

Configurazione per l'abbinamento degli hook a eventi o strumenti specifici.

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

Tipo di unione di tutti i tipi di input hook. Il tipo effettivo dipende dal campo `hook_event_name`.

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

Campi di base presenti in tutti i tipi di input hook.

```python theme={null}
class BaseHookInput(TypedDict):
    session_id: str
    transcript_path: str
    cwd: str
    permission_mode: NotRequired[str]
```

| Campo             | Tipo              | Descrizione                                   |
| :---------------- | :---------------- | :-------------------------------------------- |
| `session_id`      | `str`             | Identificatore di sessione corrente           |
| `transcript_path` | `str`             | Percorso al file di trascritto della sessione |
| `cwd`             | `str`             | Directory di lavoro corrente                  |
| `permission_mode` | `str` (opzionale) | Modalità di autorizzazione corrente           |

<h3 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h3>

Dati di input per gli eventi hook `PreToolUse`.

```python theme={null}
class PreToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PreToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Campo             | Tipo                    | Descrizione                                                                                |
| :---------------- | :---------------------- | :----------------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PreToolUse"]` | Sempre "PreToolUse"                                                                        |
| `tool_name`       | `str`                   | Nome dello strumento che sta per essere eseguito                                           |
| `tool_input`      | `dict[str, Any]`        | Parametri di input per lo strumento                                                        |
| `tool_use_id`     | `str`                   | Identificatore univoco per questo utilizzo dello strumento                                 |
| `agent_id`        | `str` (opzionale)       | Identificatore del subagente, presente quando l'hook si attiva all'interno di un subagente |
| `agent_type`      | `str` (opzionale)       | Tipo di subagente, presente quando l'hook si attiva all'interno di un subagente            |

<h3 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h3>

Dati di input per gli eventi hook `PostToolUse`.

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

| Campo             | Tipo                     | Descrizione                                                                                |
| :---------------- | :----------------------- | :----------------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUse"]` | Sempre "PostToolUse"                                                                       |
| `tool_name`       | `str`                    | Nome dello strumento che è stato eseguito                                                  |
| `tool_input`      | `dict[str, Any]`         | Parametri di input che sono stati utilizzati                                               |
| `tool_response`   | `Any`                    | Risposta dall'esecuzione dello strumento                                                   |
| `tool_use_id`     | `str`                    | Identificatore univoco per questo utilizzo dello strumento                                 |
| `agent_id`        | `str` (opzionale)        | Identificatore del subagente, presente quando l'hook si attiva all'interno di un subagente |
| `agent_type`      | `str` (opzionale)        | Tipo di subagente, presente quando l'hook si attiva all'interno di un subagente            |

<h3 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h3>

Dati di input per gli eventi hook `PostToolUseFailure`. Chiamato quando l'esecuzione di uno strumento fallisce.

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

| Campo             | Tipo                            | Descrizione                                                                                |
| :---------------- | :------------------------------ | :----------------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Sempre "PostToolUseFailure"                                                                |
| `tool_name`       | `str`                           | Nome dello strumento che ha fallito                                                        |
| `tool_input`      | `dict[str, Any]`                | Parametri di input che sono stati utilizzati                                               |
| `tool_use_id`     | `str`                           | Identificatore univoco per questo utilizzo dello strumento                                 |
| `error`           | `str`                           | Messaggio di errore dall'esecuzione fallita                                                |
| `is_interrupt`    | `bool` (opzionale)              | Se il fallimento è stato causato da un'interruzione                                        |
| `agent_id`        | `str` (opzionale)               | Identificatore del subagente, presente quando l'hook si attiva all'interno di un subagente |
| `agent_type`      | `str` (opzionale)               | Tipo di subagente, presente quando l'hook si attiva all'interno di un subagente            |

<h3 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h3>

Dati di input per gli eventi hook `UserPromptSubmit`.

```python theme={null}
class UserPromptSubmitHookInput(BaseHookInput):
    hook_event_name: Literal["UserPromptSubmit"]
    prompt: str
```

| Campo             | Tipo                          | Descrizione                   |
| :---------------- | :---------------------------- | :---------------------------- |
| `hook_event_name` | `Literal["UserPromptSubmit"]` | Sempre "UserPromptSubmit"     |
| `prompt`          | `str`                         | Il prompt inviato dall'utente |

<h3 id="stophookinput">
  `StopHookInput`
</h3>

Dati di input per gli eventi hook `Stop`.

```python theme={null}
class StopHookInput(BaseHookInput):
    hook_event_name: Literal["Stop"]
    stop_hook_active: bool
```

| Campo              | Tipo              | Descrizione                   |
| :----------------- | :---------------- | :---------------------------- |
| `hook_event_name`  | `Literal["Stop"]` | Sempre "Stop"                 |
| `stop_hook_active` | `bool`            | Se l'hook di arresto è attivo |

<h3 id="subagentstophookinput">
  `SubagentStopHookInput`
</h3>

Dati di input per gli eventi hook `SubagentStop`.

```python theme={null}
class SubagentStopHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStop"]
    stop_hook_active: bool
    agent_id: str
    agent_transcript_path: str
    agent_type: str
```

| Campo                   | Tipo                      | Descrizione                                  |
| :---------------------- | :------------------------ | :------------------------------------------- |
| `hook_event_name`       | `Literal["SubagentStop"]` | Sempre "SubagentStop"                        |
| `stop_hook_active`      | `bool`                    | Se l'hook di arresto è attivo                |
| `agent_id`              | `str`                     | Identificatore univoco per il subagente      |
| `agent_transcript_path` | `str`                     | Percorso al file di trascritto del subagente |
| `agent_type`            | `str`                     | Tipo del subagente                           |

<h3 id="precompacthookinput">
  `PreCompactHookInput`
</h3>

Dati di input per gli eventi hook `PreCompact`.

```python theme={null}
class PreCompactHookInput(BaseHookInput):
    hook_event_name: Literal["PreCompact"]
    trigger: Literal["manual", "auto"]
    custom_instructions: str | None
```

| Campo                 | Tipo                        | Descrizione                                    |
| :-------------------- | :-------------------------- | :--------------------------------------------- |
| `hook_event_name`     | `Literal["PreCompact"]`     | Sempre "PreCompact"                            |
| `trigger`             | `Literal["manual", "auto"]` | Cosa ha attivato la compattazione              |
| `custom_instructions` | `str \| None`               | Istruzioni personalizzate per la compattazione |

<h3 id="notificationhookinput">
  `NotificationHookInput`
</h3>

Dati di input per gli eventi hook `Notification`.

```python theme={null}
class NotificationHookInput(BaseHookInput):
    hook_event_name: Literal["Notification"]
    message: str
    title: NotRequired[str]
    notification_type: str
```

| Campo               | Tipo                      | Descrizione                         |
| :------------------ | :------------------------ | :---------------------------------- |
| `hook_event_name`   | `Literal["Notification"]` | Sempre "Notification"               |
| `message`           | `str`                     | Contenuto del messaggio di notifica |
| `title`             | `str` (opzionale)         | Titolo della notifica               |
| `notification_type` | `str`                     | Tipo di notifica                    |

<h3 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h3>

Dati di input per gli eventi hook `SubagentStart`.

```python theme={null}
class SubagentStartHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStart"]
    agent_id: str
    agent_type: str
```

| Campo             | Tipo                       | Descrizione                             |
| :---------------- | :------------------------- | :-------------------------------------- |
| `hook_event_name` | `Literal["SubagentStart"]` | Sempre "SubagentStart"                  |
| `agent_id`        | `str`                      | Identificatore univoco per il subagente |
| `agent_type`      | `str`                      | Tipo del subagente                      |

<h3 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h3>

Dati di input per gli eventi hook `PermissionRequest`. Consente agli hook di gestire le decisioni di autorizzazione a livello di programmazione.

```python theme={null}
class PermissionRequestHookInput(BaseHookInput):
    hook_event_name: Literal["PermissionRequest"]
    tool_name: str
    tool_input: dict[str, Any]
    permission_suggestions: NotRequired[list[Any]]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Campo                    | Tipo                           | Descrizione                                                                                |
| :----------------------- | :----------------------------- | :----------------------------------------------------------------------------------------- |
| `hook_event_name`        | `Literal["PermissionRequest"]` | Sempre "PermissionRequest"                                                                 |
| `tool_name`              | `str`                          | Nome dello strumento che richiede l'autorizzazione                                         |
| `tool_input`             | `dict[str, Any]`               | Parametri di input per lo strumento                                                        |
| `permission_suggestions` | `list[Any]` (opzionale)        | Aggiornamenti di autorizzazione suggeriti dalla CLI                                        |
| `agent_id`               | `str` (opzionale)              | Identificatore del subagente, presente quando l'hook si attiva all'interno di un subagente |
| `agent_type`             | `str` (opzionale)              | Tipo di subagente, presente quando l'hook si attiva all'interno di un subagente            |

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Tipo di unione per i valori di ritorno del callback hook.

```python theme={null}
HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

Output hook sincrono con campi di controllo e decisione.

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
  Usa `continue_` (con underscore) nel codice Python. Viene automaticamente convertito a `continue` quando inviato alla CLI.
</Note>

<h4 id="hookspecificoutput">
  `HookSpecificOutput`
</h4>

Un `TypedDict` contenente il nome dell'evento hook e i campi specifici dell'evento. La forma dipende dal valore `hookEventName`. Per i dettagli completi sui campi disponibili per evento hook, vedi [Controlla l'esecuzione con gli hooks](/docs/it/agent-sdk/hooks#outputs).

Un'unione discriminata di tipi di output specifici dell'evento. Il campo `hookEventName` determina quali campi sono validi.

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

Output hook asincrono che rinvia l'esecuzione dell'hook.

```python theme={null}
class AsyncHookJSONOutput(TypedDict):
    async_: Literal[True]  # Set to True to defer execution
    asyncTimeout: NotRequired[int]  # Timeout in milliseconds
```

<Note>
  Usa `async_` (con underscore) nel codice Python. Viene automaticamente convertito a `async` quando inviato alla CLI.
</Note>

<h3 id="hook-usage-example">
  Esempio di utilizzo di Hook
</h3>

Questo esempio registra due hook: uno che blocca i comandi bash pericolosi come `rm -rf /`, e un altro che registra tutto l'utilizzo dello strumento per il controllo. L'hook di sicurezza viene eseguito solo sui comandi Bash (tramite il `matcher`), mentre l'hook di registrazione viene eseguito su tutti gli strumenti.

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
  Tipi di input/output dello strumento
</h2>

Documentazione degli schemi di input/output per tutti gli strumenti Claude Code integrati. Mentre Python SDK non esporta questi come tipi, rappresentano la struttura degli input e output dello strumento nei messaggi.

<h3 id="agent">
  Agent
</h3>

**Nome dello strumento:** `Agent` (precedentemente `Task`, che è ancora accettato come alias)

**Input:**

```python theme={null}
{
    "description": str,  # Una breve descrizione del compito (3-5 parole)
    "prompt": str,  # Il compito che l'agente deve eseguire
    "subagent_type": str,  # Il tipo di agente specializzato da utilizzare
}
```

**Output:**

```python theme={null}
{
    "result": str,  # Risultato finale dal subagente
    "usage": dict | None,  # Statistiche di utilizzo dei token
    "total_cost_usd": float | None,  # Costo totale stimato in USD
    "duration_ms": int | None,  # Durata dell'esecuzione in millisecondi
}
```

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Nome dello strumento:** `AskUserQuestion`

Chiede all'utente domande di chiarimento durante l'esecuzione. Vedi [Gestisci approvazioni e input dell'utente](/docs/it/agent-sdk/user-input#handle-clarifying-questions) per i dettagli di utilizzo.

**Input:**

```python theme={null}
{
    "questions": [  # Domande da porre all'utente (1-4 domande)
        {
            "question": str,  # La domanda completa da porre all'utente
            "header": str,  # Etichetta molto breve visualizzata come chip/tag (max 12 caratteri)
            "options": [  # Le scelte disponibili (2-4 opzioni)
                {
                    "label": str,  # Testo visualizzato per questa opzione (1-5 parole)
                    "description": str,  # Spiegazione di cosa significa questa opzione
                }
            ],
            "multiSelect": bool,  # Impostare su true per consentire selezioni multiple
        }
    ],
    "answers": dict[str, str | list[str]] | None,
    # Risposte dell'utente popolate dal sistema di autorizzazione. Le risposte
    # multi-select possono essere un elenco di etichette o una stringa unita da virgole
}
```

**Output:**

```python theme={null}
{
    "questions": [  # Le domande che sono state poste
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Mappa il testo della domanda alla stringa di risposta
    # Le risposte multi-select sono separate da virgole
}
```

<h3 id="bash">
  Bash
</h3>

**Nome dello strumento:** `Bash`

**Input:**

```python theme={null}
{
    "command": str,  # Il comando da eseguire
    "timeout": int | None,  # Timeout opzionale in millisecondi (max 600000; i valori più alti vengono limitati al massimo)
    "description": str | None,  # Descrizione chiara e concisa (5-10 parole)
    "run_in_background": bool | None,  # Impostare su true per eseguire in background
}
```

**Output:**

```python theme={null}
{
    "output": str,  # Output combinato di stdout e stderr
    "exitCode": int,  # Codice di uscita del comando
    "killed": bool | None,  # Se il comando è stato terminato a causa del timeout
    "shellId": str | None,  # ID della shell per i processi in background
}
```

<h3 id="monitor">
  Monitor
</h3>

**Nome dello strumento:** `Monitor`

Esegue uno script in background e fornisce ogni evento a Claude in modo che possa reagire senza polling: `command` esegue uno script e emette un evento per riga stdout, e `ws` apre un WebSocket ed emette un evento per frame di testo. Fornire esattamente uno tra `command` o `ws`.

Quando Monitor esegue un comando, segue le stesse regole di autorizzazione di Bash; un monitoraggio WebSocket richiede l'approvazione separatamente. L'origine `ws` richiede Claude Code v2.1.195 o successivo. Vedi il [riferimento dello strumento Monitor](/docs/it/tools-reference#monitor-tool) per il comportamento e la disponibilità del provider.

**Input:**

```python theme={null}
{
    "command": str | None,  # Script shell; ogni riga stdout è un evento, exit termina il monitoraggio
    "ws": dict | None,  # Origine WebSocket: {"url": str, "protocols": list[str] | None}; ogni frame di testo è un evento
    "description": str,  # Breve descrizione mostrata nelle notifiche
    "timeout_ms": int | None,  # Termina dopo questa scadenza (default 300000, max 3600000)
    "persistent": bool | None,  # Esegui per la durata della sessione; ferma con TaskStop
}
```

**Output:**

```python theme={null}
{
    "taskId": str,  # ID dell'attività di monitoraggio in background
    "timeoutMs": int,  # Scadenza del timeout in millisecondi (0 quando persistente)
    "persistent": bool | None,  # True quando in esecuzione fino a TaskStop o fine sessione
}
```

<h3 id="edit">
  Edit
</h3>

**Nome dello strumento:** `Edit`

**Input:**

```python theme={null}
{
    "file_path": str,  # Il percorso assoluto del file da modificare
    "old_string": str,  # Il testo da sostituire
    "new_string": str,  # Il testo con cui sostituirlo
    "replace_all": bool | None,  # Sostituisci tutte le occorrenze (default False)
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Messaggio di conferma
    "replacements": int,  # Numero di sostituzioni effettuate
    "file_path": str,  # Percorso del file che è stato modificato
}
```

<h3 id="read">
  Read
</h3>

**Nome dello strumento:** `Read`

**Input:**

```python theme={null}
{
    "file_path": str,  # Il percorso assoluto del file da leggere
    "offset": int | None,  # Il numero di riga da cui iniziare la lettura
    "limit": int | None,  # Il numero di righe da leggere
}
```

**Output (File di testo):**

```python theme={null}
{
    "content": str,  # Contenuto del file con numeri di riga
    "total_lines": int,  # Numero totale di righe nel file
    "lines_returned": int,  # Righe effettivamente restituite
}
```

**Output (Immagini):**

```python theme={null}
{
    "image": str,  # Dati dell'immagine codificati in Base64
    "mime_type": str,  # Tipo MIME dell'immagine
    "file_size": int,  # Dimensione del file in byte
}
```

<h3 id="write">
  Write
</h3>

**Nome dello strumento:** `Write`

**Input:**

```python theme={null}
{
    "file_path": str,  # Il percorso assoluto del file da scrivere
    "content": str,  # Il contenuto da scrivere nel file
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Messaggio di successo
    "bytes_written": int,  # Numero di byte scritti
    "file_path": str,  # Percorso del file che è stato scritto
}
```

<h3 id="glob">
  Glob
</h3>

**Nome dello strumento:** `Glob`

**Input:**

```python theme={null}
{
    "pattern": str,  # Il pattern glob per abbinare i file
    "path": str | None,  # La directory da cercare (default cwd)
}
```

**Output:**

```python theme={null}
{
    "matches": list[str],  # Array dei percorsi dei file corrispondenti
    "count": int,  # Numero di corrispondenze trovate
    "search_path": str,  # Directory di ricerca utilizzata
}
```

<h3 id="grep">
  Grep
</h3>

**Nome dello strumento:** `Grep`

**Input:**

```python theme={null}
{
    "pattern": str,  # Il pattern di espressione regolare
    "path": str | None,  # File o directory da cercare
    "glob": str | None,  # Pattern glob per filtrare i file
    "type": str | None,  # Tipo di file da cercare
    "output_mode": str | None,  # "content", "files_with_matches", o "count"
    "-i": bool | None,  # Ricerca senza distinzione maiuscole/minuscole
    "-n": bool | None,  # Mostra i numeri di riga
    "-B": int | None,  # Righe da mostrare prima di ogni corrispondenza
    "-A": int | None,  # Righe da mostrare dopo ogni corrispondenza
    "-C": int | None,  # Righe da mostrare prima e dopo
    "head_limit": int | None,  # Limita l'output alle prime N righe/voci
    "multiline": bool | None,  # Abilita la modalità multilinea
}
```

**Output (modalità content):**

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

**Output (modalità files\_with\_matches):**

```python theme={null}
{
    "files": list[str],  # File contenenti corrispondenze
    "count": int,  # Numero di file con corrispondenze
}
```

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Nome dello strumento:** `NotebookEdit`

**Input:**

```python theme={null}
{
    "notebook_path": str,  # Percorso assoluto del notebook Jupyter
    "cell_id": str | None,  # L'ID della cella da modificare
    "new_source": str,  # La nuova sorgente per la cella
    "cell_type": "code" | "markdown" | None,  # Il tipo della cella
    "edit_mode": "replace" | "insert" | "delete" | None,  # Tipo di operazione di modifica
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Messaggio di successo
    "edit_type": "replaced" | "inserted" | "deleted",  # Tipo di modifica eseguita
    "cell_id": str | None,  # ID della cella interessata
    "total_cells": int,  # Numero totale di celle nel notebook dopo la modifica
}
```

<h3 id="webfetch">
  WebFetch
</h3>

**Nome dello strumento:** `WebFetch`

**Input:**

```python theme={null}
{
    "url": str,  # L'URL da cui recuperare il contenuto
    "prompt": str,  # Il prompt da eseguire sul contenuto recuperato
}
```

**Output:**

```python theme={null}
{
    "bytes": int,  # Dimensione del contenuto recuperato in byte
    "code": int,  # Codice di risposta HTTP
    "codeText": str,  # Testo del codice di risposta HTTP
    "result": str,  # Risultato elaborato dall'applicazione del prompt al contenuto
    "durationMs": int,  # Tempo per recuperare ed elaborare il contenuto, in millisecondi
    "url": str,  # URL che è stato recuperato
}
```

<h3 id="websearch">
  WebSearch
</h3>

**Nome dello strumento:** `WebSearch`

**Input:**

```python theme={null}
{
    "query": str,  # La query di ricerca da utilizzare
    "allowed_domains": list[str] | None,  # Includi solo risultati da questi domini
    "blocked_domains": list[str] | None,  # Non includere mai risultati da questi domini
}
```

**Output:**

```python theme={null}
{
    "query": str,  # La query di ricerca
    "results": list[str | {"tool_use_id": str, "content": list[{"title": str, "url": str}]}],
    "durationSeconds": float,  # Durata della ricerca in secondi
}
```

<h3 id="todowrite">
  TodoWrite
</h3>

**Nome dello strumento:** `TodoWrite`

<Note>
  A partire da Claude Code v2.1.142, `TodoWrite` è disabilitato per impostazione predefinita. Utilizza invece `TaskCreate`, `TaskGet`, `TaskUpdate` e `TaskList`. Vedi [Migra agli strumenti Task](/docs/it/agent-sdk/todo-tracking#migrate-to-task-tools) per aggiornare il tuo codice di monitoraggio, oppure imposta `CLAUDE_CODE_ENABLE_TASKS=0` per ripristinare `TodoWrite`.
</Note>

**Input:**

```python theme={null}
{
    "todos": [
        {
            "content": str,  # La descrizione del compito
            "status": "pending" | "in_progress" | "completed",  # Stato del compito
            "activeForm": str,  # Forma attiva della descrizione
        }
    ]
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Messaggio di successo
    "stats": {"total": int, "pending": int, "in_progress": int, "completed": int},
}
```

<h3 id="taskcreate">
  TaskCreate
</h3>

**Nome dello strumento:** `TaskCreate`

**Input:**

```python theme={null}
{
    "subject": str,  # Titolo breve del compito
    "description": str,  # Corpo dettagliato del compito
    "activeForm": str | None,  # Etichetta al tempo presente mostrata mentre in corso
    "metadata": dict | None,  # Metadati arbitrari del chiamante
}
```

**Output:**

```python theme={null}
{
    "task": {"id": str, "subject": str},  # Compito creato con ID assegnato
}
```

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Nome dello strumento:** `TaskUpdate`

**Input:**

```python theme={null}
{
    "taskId": str,  # ID del compito da modificare
    "status": Literal["pending", "in_progress", "completed", "deleted"] | None,
    "subject": str | None,
    "description": str | None,
    "activeForm": str | None,
    "addBlocks": list[str] | None,  # ID dei compiti che questo compito ora blocca
    "addBlockedBy": list[str] | None,  # ID dei compiti che ora bloccano questo compito
    "owner": str | None,
    "metadata": dict | None,
}
```

**Output:**

```python theme={null}
{
    "success": bool,
    "taskId": str,
    "updatedFields": list[str],  # Nomi dei campi che sono cambiati
    "error": str | None,
    "statusChange": {"from": str, "to": str} | None,
}
```

<h3 id="taskget">
  TaskGet
</h3>

**Nome dello strumento:** `TaskGet`

**Input:**

```python theme={null}
{
    "taskId": str,  # ID del compito da leggere
}
```

**Output:**

```python theme={null}
{
    "task": {
        "id": str,
        "subject": str,
        "description": str,
        "status": Literal["pending", "in_progress", "completed"],
        "blocks": list[str],
        "blockedBy": list[str],
    } | None,  # None quando l'ID non viene trovato
}
```

<h3 id="tasklist">
  TaskList
</h3>

**Nome dello strumento:** `TaskList`

**Input:**

```python theme={null}
{}
```

**Output:**

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

**Nome dello strumento:** `BashOutput`

**Input:**

```python theme={null}
{
    "bash_id": str,  # L'ID della shell in background
    "filter": str | None,  # Regex opzionale per filtrare le righe di output
}
```

**Output:**

```python theme={null}
{
    "output": str,  # Nuovo output dall'ultimo controllo
    "status": "running" | "completed" | "failed",  # Stato attuale della shell
    "exitCode": int | None,  # Codice di uscita al completamento
}
```

<h3 id="killbash">
  KillBash
</h3>

**Nome dello strumento:** `KillBash`

**Input:**

```python theme={null}
{
    "shell_id": str  # L'ID della shell in background da terminare
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Messaggio di successo
    "shell_id": str,  # ID della shell terminata
}
```

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nome dello strumento:** `ExitPlanMode`

**Input:**

```python theme={null}
{
    "plan": str  # Il piano da eseguire dall'utente per l'approvazione
}
```

**Output:**

```python theme={null}
{
    "message": str,  # Messaggio di conferma
    "approved": bool | None,  # Se l'utente ha approvato il piano
}
```

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nome dello strumento:** `ListMcpResourcesTool`

**Input:**

```python theme={null}
{
    "server": str | None  # Nome del server opzionale per filtrare le risorse
}
```

**Output:**

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

**Nome dello strumento:** `ReadMcpResourceTool`

**Input:**

```python theme={null}
{
    "server": str,  # Il nome del server MCP
    "uri": str,  # L'URI della risorsa da leggere
}
```

**Output:**

```python theme={null}
{
    "contents": [
        {"uri": str, "mimeType": str | None, "text": str | None, "blob": str | None}
    ],
    "server": str,
}
```

<h2 id="advanced-features-with-claudesdkclient">
  Funzionalità avanzate con ClaudeSDKClient
</h2>

<h3 id="building-a-continuous-conversation-interface">
  Costruire un'interfaccia di conversazione continua
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
  Utilizzo di hooks per la modifica del comportamento
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
  Monitoraggio del progresso in tempo reale
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
  Utilizzo di esempio
</h2>

<h3 id="basic-file-operations-using-query">
  Operazioni di file di base (usando query)
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
  Gestione degli errori
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
  Modalità streaming con client
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
  Utilizzo di strumenti personalizzati con ClaudeSDKClient
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
  Configurazione della Sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Configurazione per il comportamento della sandbox. Usala per abilitare il sandboxing dei comandi e configurare le restrizioni di rete a livello di programmazione.

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

| Proprietà                   | Tipo                                                  | Predefinito | Descrizione                                                                                                                                                                                                                                                                     |
| :-------------------------- | :---------------------------------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `enabled`                   | `bool`                                                | `False`     | Abilita la modalità sandbox per l'esecuzione dei comandi                                                                                                                                                                                                                        |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`      | Approva automaticamente i comandi bash quando la sandbox è abilitata                                                                                                                                                                                                            |
| `excludedCommands`          | `list[str]`                                           | `[]`        | Comandi che sempre bypassano le restrizioni della sandbox (ad es. `["docker"]`). Questi vengono eseguiti senza sandbox automaticamente senza coinvolgimento del modello                                                                                                         |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`      | Consenti al modello di richiedere l'esecuzione di comandi al di fuori della sandbox. Quando `True`, il modello può impostare `dangerouslyDisableSandbox` nell'input dello strumento, che ricade nel [sistema di autorizzazioni](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`      | Configurazione della sandbox specifica della rete                                                                                                                                                                                                                               |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`      | Configura quali violazioni della sandbox ignorare                                                                                                                                                                                                                               |
| `enableWeakerNestedSandbox` | `bool`                                                | `False`     | Abilita una sandbox nidificata più debole per la compatibilità                                                                                                                                                                                                                  |

<Note>
  La sandbox dipende dal supporto della piattaforma e, su Linux, da strumenti come `bubblewrap` e `socat`. Per impostazione predefinita, quando `enabled` è `True` ma la sandbox non può avviarsi, i comandi vengono eseguiti senza sandbox con un avviso su stderr. Questo comportamento predefinito differisce dall'SDK TypeScript, dove `failIfUnavailable` è predefinito su `true`.

  Imposta `"failIfUnavailable": True` nelle impostazioni della sandbox per interrompere invece. La chiave non è ancora dichiarata su `SandboxSettings`, ma l'SDK la inoltra a Claude Code, che la rispetta. `query()` quindi segnala un `ResultMessage` con `subtype="error_during_execution"` e il motivo in `errors`. Guarda quel sottotipo piuttosto che aspettarti che `query()` generi un'eccezione prima di cedere i messaggi.
</Note>

<h4 id="example-usage-2">
  Utilizzo di esempio
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
  **Sicurezza del socket Unix**: L'opzione `allowUnixSockets` può concedere l'accesso a potenti servizi di sistema. Ad esempio, consentire `/var/run/docker.sock` concede effettivamente l'accesso completo al sistema host tramite l'API Docker, bypassando l'isolamento della sandbox. Consenti solo i socket Unix strettamente necessari e comprendi le implicazioni di sicurezza di ognuno.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configurazione specifica della rete per la modalità sandbox. Queste impostazioni si applicano ai comandi Bash in sandbox quando `enabled` è `True` nella [`SandboxSettings`](#sandboxsettings) padre. Non limitano lo strumento WebFetch, che utilizza invece [regole di autorizzazione](/docs/it/permissions#webfetch).

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

| Proprietà                 | Tipo        | Predefinito | Descrizione                                                                                                                                                                               |
| :------------------------ | :---------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `list[str]` | `[]`        | Nomi di dominio a cui i processi in sandbox possono accedere                                                                                                                              |
| `deniedDomains`           | `list[str]` | `[]`        | Nomi di dominio a cui i processi in sandbox non possono accedere. Ha la precedenza su `allowedDomains`                                                                                    |
| `allowManagedDomainsOnly` | `bool`      | `False`     | Solo impostazioni gestite: quando impostato nelle impostazioni gestite, ignora `allowedDomains` da fonti di impostazioni non gestite. Non ha effetto quando impostato tramite opzioni SDK |
| `allowUnixSockets`        | `list[str]` | `[]`        | Percorsi dei socket Unix a cui i processi possono accedere (ad es. socket Docker)                                                                                                         |
| `allowAllUnixSockets`     | `bool`      | `False`     | Consenti l'accesso a tutti i socket Unix                                                                                                                                                  |
| `allowLocalBinding`       | `bool`      | `False`     | Consenti ai processi di associarsi alle porte locali (ad es. per server di sviluppo)                                                                                                      |
| `allowMachLookup`         | `list[str]` | `[]`        | Solo macOS: nomi dei servizi XPC/Mach da consentire. Supporta un carattere jolly finale                                                                                                   |
| `httpProxyPort`           | `int`       | `None`      | Porta proxy HTTP per le richieste di rete                                                                                                                                                 |
| `socksProxyPort`          | `int`       | `None`      | Porta proxy SOCKS per le richieste di rete                                                                                                                                                |

<Note>
  Il proxy sandbox integrato applica l'allowlist di rete in base al nome host richiesto e non termina o ispeziona il traffico TLS, quindi tecniche come il [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) possono potenzialmente bypassarlo. Vedi [Limitazioni di sicurezza del sandboxing](/docs/it/sandboxing#security-limitations) per i dettagli e [Distribuzione sicura](/docs/it/agent-sdk/secure-deployment#traffic-forwarding) per configurare un proxy che termina TLS.
</Note>

<h3 id="sandboxignoreviolations">
  `SandboxIgnoreViolations`
</h3>

Configurazione per ignorare violazioni specifiche della sandbox.

```python theme={null}
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| Proprietà | Tipo        | Predefinito | Descrizione                                             |
| :-------- | :---------- | :---------- | :------------------------------------------------------ |
| `file`    | `list[str]` | `[]`        | Modelli di percorso file per cui ignorare le violazioni |
| `network` | `list[str]` | `[]`        | Modelli di rete per cui ignorare le violazioni          |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback delle autorizzazioni per i comandi senza sandbox
</h3>

Quando `allowUnsandboxedCommands` è abilitato, il modello può richiedere di eseguire comandi al di fuori della sandbox impostando `dangerouslyDisableSandbox: True` nell'input dello strumento. Queste richieste ricadono nel sistema di autorizzazioni esistente, il che significa che il tuo handler `can_use_tool` verrà invocato, permettendoti di implementare una logica di autorizzazione personalizzata.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Un elenco statico di comandi che sempre bypassano la sandbox automaticamente (ad es. `["docker"]`). Il modello non ha controllo su questo.
  * `allowUnsandboxedCommands`: Consenti al modello di decidere in fase di esecuzione se richiedere l'esecuzione senza sandbox impostando `dangerouslyDisableSandbox: True` nell'input dello strumento.
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

Questo modello ti consente di:

* **Controllare le richieste del modello**: Registra quando il modello richiede l'esecuzione senza sandbox
* **Implementare allowlist**: Consenti solo comandi specifici di essere eseguiti senza sandbox
* **Aggiungere flussi di lavoro di approvazione**: Richiedi un'autorizzazione esplicita per operazioni privilegiate

<Warning>
  I comandi in esecuzione con `dangerouslyDisableSandbox: True` hanno accesso completo al sistema. Assicurati che il tuo handler `can_use_tool` validi queste richieste attentamente.

  Se `permission_mode` è impostato su `bypassPermissions` e `allow_unsandboxed_commands` è abilitato, il modello può autonomamente eseguire comandi al di fuori della sandbox senza alcun prompt di approvazione. Questa combinazione consente effettivamente al modello di sfuggire all'isolamento della sandbox silenziosamente.
</Warning>

<h2 id="see-also">
  Vedi anche
</h2>

* [Panoramica dell'SDK](/docs/it/agent-sdk/overview) - Concetti generali dell'SDK
* [Riferimento TypeScript SDK](/docs/it/agent-sdk/typescript) - Documentazione TypeScript SDK
* [Riferimento CLI](/docs/it/cli-reference) - Interfaccia della riga di comando
* [Flussi di lavoro comuni](/docs/it/common-workflows) - Guide passo dopo passo
