> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia del SDK de Agent - Python

> Referencia completa de la API del SDK de Agent de Python, incluyendo todas las funciones, tipos y clases.

<h2 id="installation">
  Instalación
</h2>

Instale el paquete en un entorno virtual. En instalaciones recientes de Debian, Ubuntu y Homebrew Python, ejecutar `pip install` contra Python del sistema falla con `error: externally-managed-environment`.

```bash theme={null}
python3 -m venv .venv
source .venv/bin/activate
pip install claude-agent-sdk
```

Para uv, Windows PowerShell y configuración de claves API, consulte [Comenzar en la descripción general del Agent SDK](/docs/es/agent-sdk/overview#get-started).

<h2 id="choosing-between-query-and-claudesdkclient">
  Elegir entre `query()` y `ClaudeSDKClient`
</h2>

El SDK de Python proporciona dos formas de interactuar con Claude Code:

<h3 id="quick-comparison">
  Comparación rápida
</h3>

| Característica                  | `query()`                                          | `ClaudeSDKClient`                           |
| :------------------------------ | :------------------------------------------------- | :------------------------------------------ |
| **Sesión**                      | Crea una nueva sesión de forma predeterminada      | Reutiliza la misma sesión                   |
| **Conversación**                | Intercambio único                                  | Múltiples intercambios en el mismo contexto |
| **Conexión**                    | Se gestiona automáticamente                        | Control manual                              |
| **Entrada de streaming**        | ✅ Compatible                                       | ✅ Compatible                                |
| **Interrupciones**              | ❌ No compatible                                    | ✅ Compatible                                |
| **Hooks**                       | ✅ Compatible                                       | ✅ Compatible                                |
| **Herramientas personalizadas** | ✅ Compatible                                       | ✅ Compatible                                |
| **Continuar chat**              | Manual mediante `continue_conversation` o `resume` | ✅ Automático                                |
| **Caso de uso**                 | Tareas puntuales                                   | Conversaciones continuas                    |

<h3 id="when-to-use-query-one-off-tasks">
  Cuándo usar `query()` (tareas puntuales)
</h3>

**Mejor para:**

* Preguntas puntuales donde no necesita historial de conversación
* Tareas independientes que no requieren contexto de intercambios anteriores
* Scripts de automatización simple
* Cuando desea un comienzo nuevo cada vez

<h3 id="when-to-use-claudesdkclient-continuous-conversation">
  Cuándo usar `ClaudeSDKClient` (conversación continua)
</h3>

**Mejor para:**

* **Continuar conversaciones** - Cuando necesita que Claude recuerde el contexto
* **Preguntas de seguimiento** - Construir sobre respuestas anteriores
* **Aplicaciones interactivas** - Interfaces de chat, REPLs
* **Lógica impulsada por respuestas** - Cuando la siguiente acción depende de la respuesta de Claude
* **Control de sesión** - Gestionar explícitamente el ciclo de vida de la conversación

<h2 id="functions">
  Funciones
</h2>

<h3 id="query">
  `query()`
</h3>

Crea una nueva sesión para cada interacción con Claude Code de forma predeterminada. Devuelve un iterador asincrónico que produce mensajes a medida que llegan. Cada llamada a `query()` comienza de nuevo sin memoria de interacciones anteriores a menos que pase `continue_conversation=True` o `resume` en [`ClaudeAgentOptions`](#claudeagentoptions). Consulte [Sessions](/docs/es/agent-sdk/sessions).

```python theme={null}
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None,
    transport: Transport | None = None
) -> AsyncIterator[Message]
```

<h4 id="parameters">
  Parámetros
</h4>

| Parámetro   | Tipo                         | Descripción                                                                        |
| :---------- | :--------------------------- | :--------------------------------------------------------------------------------- |
| `prompt`    | `str \| AsyncIterable[dict]` | El prompt de entrada como una cadena o iterable asincrónico para modo de streaming |
| `options`   | `ClaudeAgentOptions \| None` | Objeto de configuración opcional (por defecto `ClaudeAgentOptions()` si es None)   |
| `transport` | `Transport \| None`          | Transporte personalizado opcional para comunicarse con el proceso CLI              |

<h4 id="returns">
  Devuelve
</h4>

Devuelve un `AsyncIterator[Message]` que produce mensajes de la conversación.

<h4 id="example-with-options">
  Ejemplo - Con opciones
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

Decorador para definir herramientas MCP con seguridad de tipos.

```python theme={null}
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any],
    annotations: ToolAnnotations | None = None
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

<h4 id="parameters-2">
  Parámetros
</h4>

| Parámetro      | Tipo                                            | Descripción                                                                                             |
| :------------- | :---------------------------------------------- | :------------------------------------------------------------------------------------------------------ |
| `name`         | `str`                                           | Identificador único para la herramienta                                                                 |
| `description`  | `str`                                           | Descripción legible de lo que hace la herramienta                                                       |
| `input_schema` | `type \| dict[str, Any]`                        | Esquema que define los parámetros de entrada de la herramienta (ver abajo)                              |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Anotaciones opcionales de herramienta MCP que proporcionan sugerencias de comportamiento a los clientes |

<h4 id="input-schema-options">
  Opciones de esquema de entrada
</h4>

1. **Mapeo de tipo simple** (recomendado):

   ```python theme={null}
   {"text": str, "count": int, "enabled": bool}
   ```

2. **Formato JSON Schema** (para validación compleja):
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
  Devuelve
</h4>

Una función decoradora que envuelve la implementación de la herramienta y devuelve una instancia de `SdkMcpTool`.

<h4 id="example">
  Ejemplo
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

Re-exportado desde `mcp.types` (también disponible como `from claude_agent_sdk import ToolAnnotations`). Todos los campos son sugerencias opcionales; los clientes no deben depender de ellos para decisiones de seguridad.

| Campo             | Tipo           | Predeterminado | Descripción                                                                                                                                                                                  |
| :---------------- | :------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `str \| None`  | `None`         | Título legible para la herramienta                                                                                                                                                           |
| `readOnlyHint`    | `bool \| None` | `False`        | Si es `True`, la herramienta no modifica su entorno                                                                                                                                          |
| `destructiveHint` | `bool \| None` | `True`         | Si es `True`, la herramienta puede realizar actualizaciones destructivas (solo significativo cuando `readOnlyHint` es `False`)                                                               |
| `idempotentHint`  | `bool \| None` | `False`        | Si es `True`, las llamadas repetidas con los mismos argumentos no tienen efecto adicional (solo significativo cuando `readOnlyHint` es `False`)                                              |
| `openWorldHint`   | `bool \| None` | `True`         | Si es `True`, la herramienta interactúa con entidades externas (por ejemplo, búsqueda web). Si es `False`, el dominio de la herramienta es cerrado (por ejemplo, una herramienta de memoria) |

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

Crea un servidor MCP en proceso que se ejecuta dentro de su aplicación Python.

```python theme={null}
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

<h4 id="parameters-3">
  Parámetros
</h4>

| Parámetro | Tipo                            | Predeterminado | Descripción                                                        |
| :-------- | :------------------------------ | :------------- | :----------------------------------------------------------------- |
| `name`    | `str`                           | -              | Identificador único para el servidor                               |
| `version` | `str`                           | `"1.0.0"`      | Cadena de versión del servidor                                     |
| `tools`   | `list[SdkMcpTool[Any]] \| None` | `None`         | Lista de funciones de herramienta creadas con el decorador `@tool` |

<h4 id="returns-3">
  Devuelve
</h4>

Devuelve un objeto `McpSdkServerConfig` que se puede pasar a `ClaudeAgentOptions.mcp_servers`.

<h4 id="example-2">
  Ejemplo
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

Lista sesiones pasadas con metadatos. Filtre por directorio de proyecto o liste sesiones en todos los proyectos. Sincrónico; devuelve inmediatamente.

```python theme={null}
def list_sessions(
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0,
    include_worktrees: bool = True
) -> list[SDKSessionInfo]
```

<h4 id="parameters-4">
  Parámetros
</h4>

| Parámetro           | Tipo          | Predeterminado | Descripción                                                                                                |
| :------------------ | :------------ | :------------- | :--------------------------------------------------------------------------------------------------------- |
| `directory`         | `str \| None` | `None`         | Directorio para listar sesiones. Cuando se omite, devuelve sesiones en todos los proyectos                 |
| `limit`             | `int \| None` | `None`         | Número máximo de sesiones a devolver                                                                       |
| `offset`            | `int`         | `0`            | Número de sesiones a omitir desde el inicio de los resultados ordenados. Úselo con `limit` para paginación |
| `include_worktrees` | `bool`        | `True`         | Cuando `directory` está dentro de un repositorio git, incluya sesiones de todas las rutas de worktree      |

<h4 id="return-type-sdksessioninfo">
  Tipo de retorno: `SDKSessionInfo`
</h4>

| Propiedad       | Tipo          | Descripción                                                                                     |
| :-------------- | :------------ | :---------------------------------------------------------------------------------------------- |
| `session_id`    | `str`         | Identificador único de sesión                                                                   |
| `summary`       | `str`         | Título de visualización: título personalizado, resumen generado automáticamente o primer prompt |
| `last_modified` | `int`         | Última hora de modificación en milisegundos desde la época                                      |
| `file_size`     | `int \| None` | Tamaño del archivo de sesión en bytes (`None` para backends de almacenamiento remoto)           |
| `custom_title`  | `str \| None` | Título de sesión establecido por el usuario                                                     |
| `first_prompt`  | `str \| None` | Primer prompt de usuario significativo en la sesión                                             |
| `git_branch`    | `str \| None` | Rama de Git al final de la sesión                                                               |
| `cwd`           | `str \| None` | Directorio de trabajo para la sesión                                                            |
| `tag`           | `str \| None` | Etiqueta de sesión establecida por el usuario (ver [`tag_session()`](#tag_session))             |
| `created_at`    | `int \| None` | Hora de creación de sesión en milisegundos desde la época                                       |

<h4 id="example-3">
  Ejemplo
</h4>

Imprima las 10 sesiones más recientes para un proyecto. Los resultados se ordenan por `last_modified` descendente, por lo que el primer elemento es el más nuevo. Omita `directory` para buscar en todos los proyectos.

```python theme={null}
from claude_agent_sdk import list_sessions

for session in list_sessions(directory="/path/to/project", limit=10):
    print(f"{session.summary} ({session.session_id})")
```

<h3 id="get_session_messages">
  `get_session_messages()`
</h3>

Recupera mensajes de una sesión pasada. Sincrónico; devuelve inmediatamente.

```python theme={null}
def get_session_messages(
    session_id: str,
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0
) -> list[SessionMessage]
```

<h4 id="parameters-5">
  Parámetros
</h4>

| Parámetro    | Tipo          | Predeterminado | Descripción                                                                       |
| :----------- | :------------ | :------------- | :-------------------------------------------------------------------------------- |
| `session_id` | `str`         | requerido      | El ID de sesión para recuperar mensajes                                           |
| `directory`  | `str \| None` | `None`         | Directorio de proyecto para buscar. Cuando se omite, busca en todos los proyectos |
| `limit`      | `int \| None` | `None`         | Número máximo de mensajes a devolver                                              |
| `offset`     | `int`         | `0`            | Número de mensajes a omitir desde el inicio                                       |

<h4 id="return-type-sessionmessage">
  Tipo de retorno: `SessionMessage`
</h4>

| Propiedad            | Tipo                           | Descripción                        |
| :------------------- | :----------------------------- | :--------------------------------- |
| `type`               | `Literal["user", "assistant"]` | Rol del mensaje                    |
| `uuid`               | `str`                          | Identificador único del mensaje    |
| `session_id`         | `str`                          | Identificador de sesión            |
| `message`            | `Any`                          | Contenido del mensaje sin procesar |
| `parent_tool_use_id` | `None`                         | Reservado para uso futuro          |

<h4 id="example-4">
  Ejemplo
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

Lee metadatos para una única sesión por ID sin escanear el directorio del proyecto completo. Sincrónico; devuelve inmediatamente.

```python theme={null}
def get_session_info(
    session_id: str,
    directory: str | None = None,
) -> SDKSessionInfo | None
```

<h4 id="parameters-6">
  Parámetros
</h4>

| Parámetro    | Tipo          | Predeterminado | Descripción                                                                                    |
| :----------- | :------------ | :------------- | :--------------------------------------------------------------------------------------------- |
| `session_id` | `str`         | requerido      | UUID de la sesión a buscar                                                                     |
| `directory`  | `str \| None` | `None`         | Ruta del directorio del proyecto. Cuando se omite, busca en todos los directorios del proyecto |

Devuelve [`SDKSessionInfo`](#return-type-sdksessioninfo), o `None` si la sesión no se encuentra.

<h4 id="example-5">
  Ejemplo
</h4>

Busque los metadatos de una única sesión sin escanear el directorio del proyecto. Útil cuando ya tiene un ID de sesión de una ejecución anterior.

```python theme={null}
from claude_agent_sdk import get_session_info

info = get_session_info("550e8400-e29b-41d4-a716-446655440000")
if info:
    print(f"{info.summary} (branch: {info.git_branch}, tag: {info.tag})")
```

<h3 id="rename_session">
  `rename_session()`
</h3>

Renombra una sesión agregando una entrada de título personalizado. Las llamadas repetidas son seguras; el título más reciente gana. Sincrónico.

```python theme={null}
def rename_session(
    session_id: str,
    title: str,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-7">
  Parámetros
</h4>

| Parámetro    | Tipo          | Predeterminado | Descripción                                                                                    |
| :----------- | :------------ | :------------- | :--------------------------------------------------------------------------------------------- |
| `session_id` | `str`         | requerido      | UUID de la sesión a renombrar                                                                  |
| `title`      | `str`         | requerido      | Nuevo título. Debe ser no vacío después de eliminar espacios en blanco                         |
| `directory`  | `str \| None` | `None`         | Ruta del directorio del proyecto. Cuando se omite, busca en todos los directorios del proyecto |

Genera `ValueError` si `session_id` no es un UUID válido o `title` está vacío; `FileNotFoundError` si la sesión no se puede encontrar.

<h4 id="example-6">
  Ejemplo
</h4>

Renombre la sesión más reciente para que sea más fácil de encontrar más tarde. El nuevo título aparece en [`SDKSessionInfo.custom_title`](#return-type-sdksessioninfo) en lecturas posteriores.

```python theme={null}
from claude_agent_sdk import list_sessions, rename_session

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    rename_session(sessions[0].session_id, "Refactor auth module")
```

<h3 id="tag_session">
  `tag_session()`
</h3>

Etiqueta una sesión. Pase `None` para borrar la etiqueta. Las llamadas repetidas son seguras; la etiqueta más reciente gana. Sincrónico.

```python theme={null}
def tag_session(
    session_id: str,
    tag: str | None,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-8">
  Parámetros
</h4>

| Parámetro    | Tipo          | Predeterminado | Descripción                                                                                    |
| :----------- | :------------ | :------------- | :--------------------------------------------------------------------------------------------- |
| `session_id` | `str`         | requerido      | UUID de la sesión a etiquetar                                                                  |
| `tag`        | `str \| None` | requerido      | Cadena de etiqueta, o `None` para borrar. Sanitizada de Unicode antes de almacenar             |
| `directory`  | `str \| None` | `None`         | Ruta del directorio del proyecto. Cuando se omite, busca en todos los directorios del proyecto |

Genera `ValueError` si `session_id` no es un UUID válido o `tag` está vacío después de la sanitización; `FileNotFoundError` si la sesión no se puede encontrar.

<h4 id="example-7">
  Ejemplo
</h4>

Etiquete una sesión, luego filtre por esa etiqueta en una lectura posterior. Pase `None` para borrar una etiqueta existente.

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
  Clases
</h2>

<h3 id="claudesdkclient">
  `ClaudeSDKClient`
</h3>

**Mantiene una sesión de conversación en múltiples intercambios.** Este es el equivalente de Python de cómo funciona internamente la función `query()` del SDK de TypeScript - crea un objeto cliente que puede continuar conversaciones.

<h4 id="key-features">
  Características clave
</h4>

* **Continuidad de sesión**: Mantiene el contexto de conversación en múltiples llamadas a `query()`
* **Misma conversación**: La sesión retiene mensajes anteriores
* **Soporte de interrupciones**: Puede detener la ejecución a mitad de tarea
* **Ciclo de vida explícito**: Usted controla cuándo comienza y termina la sesión
* **Flujo impulsado por respuestas**: Puede reaccionar a respuestas y enviar seguimientos
* **Herramientas personalizadas y hooks**: Admite herramientas personalizadas (creadas con el decorador `@tool`) y hooks

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
  Métodos
</h4>

| Método                                    | Descripción                                                                                                                                                                 |
| :---------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `__init__(options)`                       | Inicializa el cliente con configuración opcional                                                                                                                            |
| `connect(prompt)`                         | Conectar a Claude con un prompt inicial opcional o flujo de mensajes                                                                                                        |
| `query(prompt, session_id)`               | Enviar una nueva solicitud en modo de streaming                                                                                                                             |
| `receive_messages()`                      | Recibir todos los mensajes de Claude como un iterador asincrónico                                                                                                           |
| `receive_response()`                      | Recibir mensajes hasta e incluyendo un ResultMessage                                                                                                                        |
| `interrupt()`                             | Enviar señal de interrupción (solo funciona en modo de streaming)                                                                                                           |
| `set_permission_mode(mode)`               | Cambiar el modo de permiso para la sesión actual                                                                                                                            |
| `set_model(model)`                        | Cambiar el modelo para la sesión actual. Pase `None` para restablecer al predeterminado                                                                                     |
| `rewind_files(user_message_id)`           | Restaurar archivos a su estado en el mensaje de usuario especificado. Requiere `enable_file_checkpointing=True`. Ver [File checkpointing](/docs/es/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Obtener el estado de todos los servidores MCP configurados. Devuelve [`McpStatusResponse`](#mcpstatusresponse)                                                              |
| `reconnect_mcp_server(server_name)`       | Reintentar conectar a un servidor MCP que falló o fue desconectado                                                                                                          |
| `toggle_mcp_server(server_name, enabled)` | Habilitar o deshabilitar un servidor MCP a mitad de sesión. Deshabilitar elimina sus herramientas                                                                           |
| `stop_task(task_id)`                      | Detener una tarea de fondo en ejecución. Un [`TaskNotificationMessage`](#tasknotificationmessage) con estado `"stopped"` sigue en el flujo de mensajes                      |
| `get_server_info()`                       | Obtener información del servidor incluyendo ID de sesión y capacidades                                                                                                      |
| `disconnect()`                            | Desconectar de Claude                                                                                                                                                       |

<h4 id="context-manager-support">
  Soporte de gestor de contexto
</h4>

El cliente se puede usar como un gestor de contexto asincrónico para la gestión automática de conexiones:

```python theme={null}
async with ClaudeSDKClient() as client:
    await client.query("Hello Claude")
    async for message in client.receive_response():
        print(message)
```

> **Importante:** Al iterar sobre mensajes, evite usar `break` para salir temprano ya que esto puede causar problemas de limpieza de asyncio. En su lugar, deje que la iteración se complete naturalmente o use banderas para rastrear cuándo ha encontrado lo que necesita.

<h4 id="example-continuing-a-conversation">
  Ejemplo - Continuar una conversación
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
  Ejemplo - Entrada de streaming con ClaudeSDKClient
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
  Ejemplo - Usar interrupciones
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
  **Comportamiento del búfer después de la interrupción:** `interrupt()` envía una señal de parada pero no borra el búfer de mensajes. Los mensajes ya producidos por la tarea interrumpida, incluyendo su `ResultMessage` (con `subtype="error_during_execution"`), permanecen en el flujo. Debe drenarlos con `receive_response()` antes de leer la respuesta a una nueva consulta. Si envía una nueva consulta inmediatamente después de `interrupt()` y llama a `receive_response()` solo una vez, recibirá los mensajes de la tarea interrumpida, no la respuesta de la nueva consulta.
</Note>

<h4 id="example-advanced-permission-control">
  Ejemplo - Control de permisos avanzado
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
  Tipos
</h2>

<Note>
  **`@dataclass` vs `TypedDict`:** Este SDK utiliza dos tipos de tipos. Las clases decoradas con `@dataclass` (como `ResultMessage`, `AgentDefinition`, `TextBlock`) son instancias de objeto en tiempo de ejecución y admiten acceso de atributo: `msg.result`. Las clases definidas con `TypedDict` (como `ThinkingConfigEnabled`, `McpStdioServerConfig`, `SyncHookJSONOutput`) son **dicts simples en tiempo de ejecución** y requieren acceso de clave: `config["budget_tokens"]`, no `config.budget_tokens`. La sintaxis de llamada `ClassName(field=value)` funciona para ambos, pero solo las dataclasses producen objetos con atributos.
</Note>

<h3 id="sdkmcptool">
  `SdkMcpTool`
</h3>

Definición para una herramienta MCP del SDK creada con el decorador `@tool`.

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| Propiedad      | Tipo                                       | Descripción                                                                                                                 |
| :------------- | :----------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                      | Identificador único para la herramienta                                                                                     |
| `description`  | `str`                                      | Descripción legible                                                                                                         |
| `input_schema` | `type[T] \| dict[str, Any]`                | Esquema para validación de entrada                                                                                          |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]` | Función asincrónica que maneja la ejecución de la herramienta                                                               |
| `annotations`  | `ToolAnnotations \| None`                  | Anotaciones opcionales de herramienta MCP (por ejemplo, `readOnlyHint`, `destructiveHint`, `openWorldHint`). De `mcp.types` |

<h3 id="transport">
  `Transport`
</h3>

Clase base abstracta para implementaciones de transporte personalizado. Úsela para comunicarse con el proceso Claude a través de un canal personalizado (por ejemplo, una conexión remota en lugar de un subproceso local).

<Warning>
  Esta es una API interna de bajo nivel. La interfaz puede cambiar en versiones futuras. Las implementaciones personalizadas deben actualizarse para coincidir con cualquier cambio de interfaz.
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

| Método            | Descripción                                                                           |
| :---------------- | :------------------------------------------------------------------------------------ |
| `connect()`       | Conectar el transporte y prepararse para la comunicación                              |
| `write(data)`     | Escribir datos sin procesar (JSON + nueva línea) en el transporte                     |
| `read_messages()` | Iterador asincrónico que produce mensajes JSON analizados                             |
| `close()`         | Cerrar la conexión y limpiar recursos                                                 |
| `is_ready()`      | Devuelve `True` si el transporte puede enviar y recibir                               |
| `end_input()`     | Cerrar el flujo de entrada (por ejemplo, cerrar stdin para transportes de subproceso) |

Importar: `from claude_agent_sdk import Transport`

<h3 id="claudeagentoptions">
  `ClaudeAgentOptions`
</h3>

Dataclass de configuración para consultas de Claude Code.

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

| Propiedad                     | Tipo                                                                                  | Predeterminado                     | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :---------------------------- | :------------------------------------------------------------------------------------ | :--------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                             | Configuración de herramientas. Use `{"type": "preset", "preset": "claude_code"}` para las herramientas predeterminadas de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                               | Herramientas para aprobar automáticamente sin solicitar. Esto no restringe Claude solo a estas herramientas; las herramientas no listadas caen a través de `permission_mode` y `can_use_tool`. Use `disallowed_tools` para bloquear herramientas. Ver [Permisos](/docs/es/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                              |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptFile \| None`                               | `None`                             | Configuración de prompt del sistema. Pase una cadena para un prompt personalizado, `{"type": "preset", "preset": "claude_code"}` para el prompt del sistema de Claude Code con `"append"` opcional, o `{"type": "file", "path": "..."}` para cargar un prompt grande desde disco. Ver [`SystemPromptPreset`](#systempromptpreset) y [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                                                                                                                   |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                               | Configuraciones de servidor MCP o ruta al archivo de configuración                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `strict_mcp_config`           | `bool`                                                                                | `False`                            | Cuando es `True`, use solo los servidores pasados en `mcp_servers` e ignore el proyecto `.mcp.json`, la configuración del usuario, los servidores MCP proporcionados por plugins, y [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai). Se asigna a la bandera CLI `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                                       |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                             | Modo de permiso para el uso de herramientas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `continue_conversation`       | `bool`                                                                                | `False`                            | Continuar la conversación más reciente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `resume`                      | `str \| None`                                                                         | `None`                             | ID de sesión a reanudar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `max_turns`                   | `int \| None`                                                                         | `None`                             | Número máximo de turnos agentes (viajes de ronda de uso de herramientas)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                             | Detener la consulta cuando la estimación de costo del lado del cliente alcance este valor en USD. Comparado con la misma estimación que `total_cost_usd`; ver [Rastrear costo y uso](/docs/es/agent-sdk/cost-tracking) para advertencias de precisión                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                               | Herramientas para denegar. Un nombre simple como `"Bash"` elimina la herramienta del contexto de Claude. Una regla con alcance como `"Bash(rm *)"` deja la herramienta disponible y deniega llamadas coincidentes en cada modo de permiso, incluyendo `bypassPermissions`. Ver [Permisos](/docs/es/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                     |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                            | Habilitar el seguimiento de cambios de archivo para rebobinar. Ver [File checkpointing](/docs/es/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `model`                       | `str \| None`                                                                         | `None`                             | Alias de modelo Claude o nombre de modelo completo. Ver [valores aceptados e IDs específicos del proveedor](/docs/es/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `fallback_model`              | `str \| None`                                                                         | `None`                             | Modelo de respaldo a usar si el modelo principal falla                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                               | Características beta a habilitar. Ver [`SdkBeta`](#sdkbeta) para opciones disponibles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                             | Formato de salida para respuestas estructuradas (por ejemplo, `{"type": "json_schema", "schema": {...}}`). Ver [Salidas estructuradas](/docs/es/agent-sdk/structured-outputs) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                             | Nombre de herramienta MCP para solicitudes de permiso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                             | Directorio de trabajo actual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                             | Ruta personalizada al ejecutable CLI de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `settings`                    | `str \| None`                                                                         | `None`                             | Ruta al archivo de configuración                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                               | Directorios adicionales a los que Claude puede acceder                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `env`                         | `dict[str, str]`                                                                      | `{}`                               | Variables de entorno fusionadas en la parte superior del entorno del proceso heredado. Ver [Variables de entorno](/docs/es/env-vars) para variables que el CLI subyacente lee, y [Manejar respuestas de API lentas o estancadas](#handle-slow-or-stalled-api-responses) para variables relacionadas con tiempos de espera                                                                                                                                                                                                                                                                                                                                                                          |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                               | Argumentos CLI adicionales para pasar directamente al CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                             | Bytes máximos al almacenar en búfer la salida estándar del CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                       | *Deprecated* - Objeto similar a un archivo para salida de depuración. Use la devolución de llamada `stderr` en su lugar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                             | Función de devolución de llamada para salida stderr del CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                             | Función de devolución de llamada de permiso de herramienta, invocada solo cuando el [flujo de permiso](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) cae a través de un prompt. No se invoca para llamadas aprobadas automáticamente por `allowed_tools`, reglas de permiso, o `permission_mode`. `AskUserQuestion`, herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) la alcanzan incluso si las ha permitido; en modo `dontAsk` se deniegan en su lugar. Ver [`CanUseTool`](#canusetool) para detalles |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                             | Configuraciones de hook para interceptar eventos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `user`                        | `str \| None`                                                                         | `None`                             | Identificador de usuario                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `include_partial_messages`    | `bool`                                                                                | `False`                            | Incluir eventos de streaming de mensaje parcial. Cuando está habilitado, se producen mensajes [`StreamEvent`](#streamevent)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `include_hook_events`         | `bool`                                                                                | `False`                            | Incluir eventos de ciclo de vida de hook en el flujo de mensajes como objetos `HookEventMessage`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `fork_session`                | `bool`                                                                                | `False`                            | Cuando se reanuda con `resume`, bifurcar a un nuevo ID de sesión en lugar de continuar la sesión original                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                             | Subagentes definidos programáticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                               | Cargar plugins personalizados desde rutas locales. Ver [Plugins](/docs/es/agent-sdk/plugins) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                             | Configurar el comportamiento de sandbox programáticamente. Ver [Configuración de sandbox](#sandboxsettings) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (CLI defaults: all sources) | Controlar qué configuración del sistema de archivos cargar. Pase `[]` para deshabilitar la configuración de usuario, proyecto y local. La configuración de política administrada se carga independientemente; la configuración administrada por servidor se obtiene cuando la sesión se autentica con una credencial de organización en una [configuración elegible](/docs/es/server-managed-settings#platform-availability). Ver [Usar características de Claude Code](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                   |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                             | Skills disponibles para la sesión. Pase `"all"` para habilitar cada skill descubierto, o una lista de nombres de skills. Cuando se establece, el SDK agrega la herramienta Skill a `allowed_tools` automáticamente. Si también pasa `tools`, incluya `"Skill"` en esa lista. Ver [Skills](/docs/es/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                               |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                             | *Deprecated* - Tokens máximos para bloques de pensamiento. Use `thinking` en su lugar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                             | Controla el comportamiento de pensamiento extendido. Tiene precedencia sobre `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                             | Nivel de esfuerzo para la profundidad del pensamiento. Ver [ajustar el nivel de esfuerzo](/docs/es/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `session_store`               | [`SessionStore`](/docs/es/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                             | Reflejar transcripciones de sesión a un backend externo para que cualquier host pueda reanudarlas. Ver [Persistir sesiones en almacenamiento externo](/docs/es/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                        | Cuándo vaciar entradas de transcripción reflejadas a `session_store`. `"batched"` vacía una vez por turno o cuando el búfer se llena; `"eager"` activa un vaciado de fondo después de cada fotograma. Se ignora cuando `session_store` es `None`                                                                                                                                                                                                                                                                                                                                                                                                                                              |

<h4 id="handle-slow-or-stalled-api-responses">
  Manejar respuestas de API lentas o estancadas
</h4>

El subproceso CLI lee varias variables de entorno que controlan los tiempos de espera de API y la detección de estancamiento. Páselas a través de `ClaudeAgentOptions.env`:

```python theme={null}
options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS`: tiempo de espera por solicitud en el cliente de Anthropic, en milisegundos. Predeterminado `600000`. Se aplica al bucle principal y a todos los subagentes.
* `CLAUDE_CODE_MAX_RETRIES`: máximo de reintentos de API. Predeterminado `10`, limitado a `15`. Cada reintento obtiene su propia ventana `API_TIMEOUT_MS`, por lo que el tiempo de pared en el peor caso es aproximadamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` más backoff. Para ejecuciones desatendidas que necesitan esperar a través de interrupciones más largas, establezca `CLAUDE_CODE_RETRY_WATCHDOG=1`: reintentos de errores de capacidad indefinidamente, y a partir de Claude Code v2.1.199 eleva el valor predeterminado para otros errores transitorios a `300` y elimina el límite en esta variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: perro guardián de estancamiento para subagentes lanzados con `run_in_background`. Predeterminado `600000`. Se reinicia en cada evento de flujo; en caso de estancamiento, aborta el subagente, marca la tarea como fallida y expone el error al padre con cualquier resultado parcial. No se aplica a subagentes síncronos.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` con `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: aborta la solicitud cuando los encabezados han llegado pero el cuerpo de la respuesta deja de transmitir. El perro guardián está activado de forma predeterminada para todos los proveedores; establezca `CLAUDE_ENABLE_STREAM_WATCHDOG=0` para desactivarlo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` tiene un valor predeterminado de `300000` y se fija a ese mínimo. La solicitud abortada pasa por la ruta de reintento normal.

<h3 id="outputformat">
  `OutputFormat`
</h3>

Configuración para validación de salida estructurada. Pase esto como un `dict` al campo `output_format` en `ClaudeAgentOptions`:

```python theme={null}
# Expected dict shape for output_format
{
    "type": "json_schema",
    "schema": {...},  # Your JSON Schema definition
}
```

| Campo    | Requerido | Descripción                                             |
| :------- | :-------- | :------------------------------------------------------ |
| `type`   | Sí        | Debe ser `"json_schema"` para validación de JSON Schema |
| `schema` | Sí        | Definición de JSON Schema para validación de salida     |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

Configuración para usar el prompt del sistema preset de Claude Code con adiciones opcionales.

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
```

| Campo                      | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                  |
| :------------------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | Sí        | Debe ser `"preset"` para usar un prompt del sistema preset                                                                                                                                                                                                                                                                                   |
| `preset`                   | Sí        | Debe ser `"claude_code"` para usar el prompt del sistema de Claude Code                                                                                                                                                                                                                                                                      |
| `append`                   | No        | Instrucciones adicionales para agregar al prompt del sistema preset                                                                                                                                                                                                                                                                          |
| `exclude_dynamic_sections` | No        | Mover contexto por sesión como directorio de trabajo, estado de git y rutas de memoria del prompt del sistema al primer mensaje del usuario. Mejora la reutilización de caché de prompt en usuarios y máquinas. Ver [Modificar prompts del sistema](/docs/es/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

Configuración para cargar un prompt del sistema personalizado desde un archivo en lugar de pasarlo como una cadena. El SDK asigna esto a la bandera CLI [`--system-prompt-file`](/docs/es/cli-reference#system-prompt-flags). Use la forma de archivo cuando el prompt es grande: el SDK pasa un `system_prompt` de cadena en el argv del subproceso CLI, que está sujeto a límites de longitud de línea de comandos del SO antes de que el SDK envíe cualquier solicitud de API. En Linux, un único argumento más largo que aproximadamente 128 KB falla al generar el proceso con `Argument list too long`. En Windows, toda la línea de comandos está limitada a aproximadamente 32 KB, por lo que la forma de cadena falla en un umbral más bajo.

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| Campo  | Requerido | Descripción                                          |
| :----- | :-------- | :--------------------------------------------------- |
| `type` | Sí        | Debe ser `"file"` para cargar el prompt desde disco  |
| `path` | Sí        | Ruta a un archivo que contiene el prompt del sistema |

<h3 id="settingsource">
  `SettingSource`
</h3>

Controla qué fuentes de configuración basadas en el sistema de archivos carga el SDK.

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| Valor       | Descripción                                                    | Ubicación                     |
| :---------- | :------------------------------------------------------------- | :---------------------------- |
| `"user"`    | Configuración global del usuario                               | `~/.claude/settings.json`     |
| `"project"` | Configuración del proyecto compartido (controlada por versión) | `.claude/settings.json`       |
| `"local"`   | Configuración del proyecto local (no controlada por versión)   | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamiento predeterminado
</h4>

Cuando `setting_sources` se omite o es `None`, `query()` carga la misma configuración del sistema de archivos que el CLI de Claude Code: usuario, proyecto y local. La configuración de política administrada se carga en todos los casos; la configuración administrada por servidor se obtiene cuando la sesión se autentica con una credencial de organización en una [configuración elegible](/docs/es/server-managed-settings#platform-availability). Ver [Qué settingSources no controla](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control) para entradas que se leen independientemente de esta opción, y cómo deshabilitarlas.

<h4 id="why-use-setting_sources">
  Por qué usar setting\_sources
</h4>

**Deshabilitar configuración del sistema de archivos:**

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
  En Python SDK 0.1.59 y anteriores, una lista vacía se trataba igual que omitir la opción, por lo que `setting_sources=[]` no deshabilitaba la configuración del sistema de archivos. Actualice a una versión más nueva si necesita que una lista vacía tenga efecto. El SDK de TypeScript no se ve afectado.
</Note>

**Cargar toda la configuración del sistema de archivos explícitamente:**

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

**Cargar solo fuentes de configuración específicas:**

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

**Entornos de prueba e IC:**

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

**Aplicaciones solo SDK:**

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

**Cargando instrucciones del proyecto CLAUDE.md:**

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
  Precedencia de configuración
</h4>

Cuando se cargan múltiples fuentes, la configuración se fusiona con esta precedencia (mayor a menor):

1. Configuración local (`.claude/settings.local.json`)
2. Configuración del proyecto (`.claude/settings.json`)
3. Configuración del usuario (`~/.claude/settings.json`)

Las opciones programáticas como `agents` y `allowed_tools` anulan la configuración del sistema de archivos de usuario, proyecto y local. La configuración de política administrada tiene precedencia sobre las opciones programáticas.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Configuración para un subagente definido programáticamente.

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

| Campo             | Requerido | Descripción                                                                                                                                                                                                                                                                         |
| :---------------- | :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | Sí        | Descripción en lenguaje natural de cuándo usar este agente                                                                                                                                                                                                                          |
| `prompt`          | Sí        | El prompt del sistema del agente                                                                                                                                                                                                                                                    |
| `tools`           | No        | Matriz de nombres de herramientas permitidas. Si se omite, hereda todas las herramientas                                                                                                                                                                                            |
| `disallowedTools` | No        | Matriz de nombres de herramientas a eliminar del conjunto de herramientas del agente. También se aceptan patrones a nivel de servidor MCP: `mcp__server` o `mcp__server__*` elimina cada herramienta de ese servidor, y `mcp__*` elimina cada herramienta MCP de cualquier servidor |
| `model`           | No        | Anulación de modelo para este agente. Acepta un alias como `"sonnet"`, `"opus"`, `"haiku"`, o `"inherit"`, o un ID de modelo completo. Si se omite, usa el modelo principal                                                                                                         |
| `skills`          | No        | Lista de nombres de skills para precargar en el contexto del agente al inicio. Los skills no listados siguen siendo invocables a través de la herramienta Skill                                                                                                                     |
| `memory`          | No        | Fuente de memoria para este agente: `"user"`, `"project"`, o `"local"`                                                                                                                                                                                                              |
| `mcpServers`      | No        | Servidores MCP disponibles para este agente. Cada entrada es un nombre de servidor o un dict `{name: config}` en línea                                                                                                                                                              |
| `initialPrompt`   | No        | Auto-enviado como el primer turno de usuario cuando este agente se ejecuta como el agente del hilo principal                                                                                                                                                                        |
| `maxTurns`        | No        | Número máximo de turnos agentes antes de que el agente se detenga                                                                                                                                                                                                                   |
| `background`      | No        | Ejecutar este agente como una tarea de fondo no bloqueante cuando se invoca                                                                                                                                                                                                         |
| `effort`          | No        | Nivel de esfuerzo de razonamiento para este agente. Acepta un nivel nombrado o un entero. Ver [`EffortLevel`](#effortlevel)                                                                                                                                                         |
| `permissionMode`  | No        | Modo de permiso para la ejecución de herramientas dentro de este agente. Ver [`PermissionMode`](#permissionmode)                                                                                                                                                                    |

<Note>
  Los nombres de campo de `AgentDefinition` usan camelCase, como `disallowedTools`, `permissionMode` y `maxTurns`. Estos nombres se asignan directamente al formato de cable compartido con el SDK de TypeScript. Esto difiere de `ClaudeAgentOptions`, que usa snake\_case de Python para campos de nivel superior equivalentes como `disallowed_tools` y `permission_mode`. Porque `AgentDefinition` es una dataclass, pasar una palabra clave snake\_case genera un `TypeError` en el tiempo de construcción.
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

Modos de permiso para controlar la ejecución de herramientas.

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

Niveles de esfuerzo para guiar la profundidad del pensamiento.

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

Alias de tipo para funciones de devolución de llamada de permiso de herramienta.

```python theme={null}
CanUseTool = Callable[
    [str, dict[str, Any], ToolPermissionContext], Awaitable[PermissionResult]
]
```

La devolución de llamada recibe:

* `tool_name`: Nombre de la herramienta que se está llamando
* `input_data`: Los parámetros de entrada de la herramienta
* `context`: Un `ToolPermissionContext` con información adicional

Devuelve un `PermissionResult` (ya sea `PermissionResultAllow` o `PermissionResultDeny`).

La devolución de llamada es el reemplazo del SDK para el prompt de permiso interactivo: se invoca solo cuando el [flujo de evaluación de permiso](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) se resuelve en un prompt. Las llamadas de herramienta ya aprobadas por una entrada `allowed_tools`, una regla de permiso de configuración, o el modo de permiso, como `acceptEdits` o `bypassPermissions`, nunca la invocan. Para controlar cada llamada de herramienta, use un [hook `PreToolUse`](/docs/es/agent-sdk/hooks) en su lugar.

`AskUserQuestion`, herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool), y herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) la alcanzan incluso cuando una regla de permiso coincide. En modo `dontAsk` estas llamadas se deniegan en su lugar, sin invocar la devolución de llamada.

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Información de contexto pasada a devoluciones de llamada de permiso de herramienta.

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

| Campo             | Tipo                     | Descripción                                                                                                                                                                                                                                           |
| :---------------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`          | `Any \| None`            | Reservado para soporte de señal de aborto futuro                                                                                                                                                                                                      |
| `suggestions`     | `list[PermissionUpdate]` | Sugerencias de actualización de permiso del CLI. Los prompts de Bash incluyen una sugerencia con el destino `localSettings`, por lo que devolverla en `updated_permissions` escribe la regla en `.claude/settings.local.json` y persiste en sesiones. |
| `blocked_path`    | `str \| None`            | Ruta de archivo que activó la solicitud de permiso, cuando sea aplicable. Por ejemplo, cuando un comando Bash intenta acceder a una ruta fuera de directorios permitidos                                                                              |
| `decision_reason` | `str \| None`            | Razón por la que se activó esta solicitud de permiso. Reenviada desde el `permissionDecisionReason` de un hook PreToolUse cuando el hook devolvió `"ask"`                                                                                             |
| `title`           | `str \| None`            | Oración completa de solicitud de permiso, como `Claude wants to read foo.txt`. Use como texto de solicitud principal cuando esté presente                                                                                                             |
| `display_name`    | `str \| None`            | Frase de sustantivo corta para la acción de herramienta, como `Read file`, adecuada para etiquetas de botón                                                                                                                                           |
| `description`     | `str \| None`            | Subtítulo legible para la interfaz de usuario de permiso                                                                                                                                                                                              |

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Tipo de unión para resultados de devolución de llamada de permiso.

```python theme={null}
PermissionResult = PermissionResultAllow | PermissionResultDeny
```

<h3 id="permissionresultallow">
  `PermissionResultAllow`
</h3>

Resultado indicando que la llamada de herramienta debe permitirse.

```python theme={null}
@dataclass
class PermissionResultAllow:
    behavior: Literal["allow"] = "allow"
    updated_input: dict[str, Any] | None = None
    updated_permissions: list[PermissionUpdate] | None = None
```

| Campo                 | Tipo                             | Predeterminado | Descripción                                       |
| :-------------------- | :------------------------------- | :------------- | :------------------------------------------------ |
| `behavior`            | `Literal["allow"]`               | `"allow"`      | Debe ser "allow"                                  |
| `updated_input`       | `dict[str, Any] \| None`         | `None`         | Entrada modificada a usar en lugar de la original |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None`         | Actualizaciones de permiso a aplicar              |

<h3 id="permissionresultdeny">
  `PermissionResultDeny`
</h3>

Resultado indicando que la llamada de herramienta debe denegarse.

```python theme={null}
@dataclass
class PermissionResultDeny:
    behavior: Literal["deny"] = "deny"
    message: str = ""
    interrupt: bool = False
```

| Campo       | Tipo              | Predeterminado | Descripción                                         |
| :---------- | :---------------- | :------------- | :-------------------------------------------------- |
| `behavior`  | `Literal["deny"]` | `"deny"`       | Debe ser "deny"                                     |
| `message`   | `str`             | `""`           | Mensaje explicando por qué se denegó la herramienta |
| `interrupt` | `bool`            | `False`        | Si se debe interrumpir la ejecución actual          |

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Configuración para actualizar permisos programáticamente.

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

| Campo         | Tipo                                      | Descripción                                                 |
| :------------ | :---------------------------------------- | :---------------------------------------------------------- |
| `type`        | `Literal[...]`                            | El tipo de operación de actualización de permiso            |
| `rules`       | `list[PermissionRuleValue] \| None`       | Reglas para operaciones de agregar/reemplazar/eliminar      |
| `behavior`    | `Literal["allow", "deny", "ask"] \| None` | Comportamiento para operaciones basadas en reglas           |
| `mode`        | `PermissionMode \| None`                  | Modo para operación setMode                                 |
| `directories` | `list[str] \| None`                       | Directorios para operaciones de agregar/eliminar directorio |
| `destination` | `Literal[...] \| None`                    | Dónde aplicar la actualización de permiso                   |

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

Una regla a agregar, reemplazar o eliminar en una actualización de permiso.

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

<h3 id="toolspreset">
  `ToolsPreset`
</h3>

Configuración de herramientas preset para usar el conjunto de herramientas predeterminado de Claude Code.

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Controla el comportamiento de pensamiento extendido. Una unión de tres configuraciones:

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

| Variante   | Campos                             | Descripción                                                  |
| :--------- | :--------------------------------- | :----------------------------------------------------------- |
| `adaptive` | `type`, `display`                  | Claude decide adaptativamente cuándo pensar                  |
| `enabled`  | `type`, `budget_tokens`, `display` | Habilitar pensamiento con un presupuesto de token específico |
| `disabled` | `type`                             | Deshabilitar pensamiento                                     |

El campo opcional `display` controla si el texto de pensamiento se devuelve `"summarized"` u `"omitted"`. En Claude Opus 4.7 y posteriores, el valor predeterminado de la API es `"omitted"`, por lo que establezca `"summarized"` para recibir contenido de pensamiento en salidas [`ThinkingBlock`](#thinkingblock).

Porque estas son clases `TypedDict`, son dicts simples en tiempo de ejecución. Construya cualquiera como literales de dict o llame a la clase como un constructor; ambos producen un `dict`. Acceda a campos con `config["budget_tokens"]`, no `config.budget_tokens`:

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

Tipo literal para características beta del SDK.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Use con el campo `betas` en `ClaudeAgentOptions` para habilitar características beta.

<Warning>
  La beta `context-1m-2025-08-07` se retiró a partir del 30 de abril de 2026. Pasar este encabezado con Claude Sonnet 4.5 o Sonnet 4 no tiene efecto, y las solicitudes que exceden la ventana de contexto estándar de 200k tokens devuelven un error. Para usar una ventana de contexto de 1M tokens, migre a [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, o Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), que incluyen contexto de 1M a precios estándar sin encabezado beta requerido.
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

Configuración para servidores MCP del SDK creados con `create_sdk_mcp_server()`.

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Tipo de unión para configuraciones de servidor MCP.

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

La configuración de un servidor MCP como se reporta por [`get_mcp_status()`](#methods). Esta es la unión de todas las variantes de transporte [`McpServerConfig`](#mcpserverconfig) más una variante de salida única `claudeai-proxy` para servidores proxied a través de claude.ai.

```python theme={null}
McpServerStatusConfig = (
    McpStdioServerConfig
    | McpSSEServerConfig
    | McpHttpServerConfig
    | McpSdkServerConfigStatus
    | McpClaudeAIProxyServerConfig
)
```

`McpSdkServerConfigStatus` es la forma serializable de [`McpSdkServerConfig`](#mcpsdkserverconfig) con solo campos `type` (`"sdk"`) y `name` (`str`); la `instance` en proceso se omite. `McpClaudeAIProxyServerConfig` tiene campos `type` (`"claudeai-proxy"`), `url` (`str`), e `id` (`str`).

<h3 id="mcpstatusresponse">
  `McpStatusResponse`
</h3>

Respuesta de [`ClaudeSDKClient.get_mcp_status()`](#methods). Envuelve la lista de estados del servidor bajo la clave `mcpServers`.

```python theme={null}
class McpStatusResponse(TypedDict):
    mcpServers: list[McpServerStatus]
```

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Estado de un servidor MCP conectado, contenido en [`McpStatusResponse`](#mcpstatusresponse).

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

| Campo        | Tipo                                                         | Descripción                                                                                                                                                                                     |
| :----------- | :----------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`       | `str`                                                        | Nombre del servidor                                                                                                                                                                             |
| `status`     | `str`                                                        | Uno de `"connected"`, `"failed"`, `"needs-auth"`, `"pending"`, o `"disabled"`                                                                                                                   |
| `serverInfo` | `dict` (opcional)                                            | Nombre y versión del servidor (`{"name": str, "version": str}`)                                                                                                                                 |
| `error`      | `str` (opcional)                                             | Mensaje de error si el servidor no se conectó                                                                                                                                                   |
| `config`     | [`McpServerStatusConfig`](#mcpserverstatusconfig) (opcional) | Configuración del servidor. Misma forma que [`McpServerConfig`](#mcpserverconfig) (stdio, SSE, HTTP, o SDK), más una variante `claudeai-proxy` para servidores conectados a través de claude.ai |
| `scope`      | `str` (opcional)                                             | Alcance de configuración                                                                                                                                                                        |
| `tools`      | `list` (opcional)                                            | Herramientas proporcionadas por este servidor, cada una con campos `name`, `description`, y `annotations`                                                                                       |

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Configuración para cargar plugins en el SDK.

```python theme={null}
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| Campo  | Tipo               | Descripción                                                      |
| :----- | :----------------- | :--------------------------------------------------------------- |
| `type` | `Literal["local"]` | Debe ser `"local"` (actualmente solo se admiten plugins locales) |
| `path` | `str`              | Ruta absoluta o relativa al directorio del plugin                |

**Ejemplo:**

```python theme={null}
plugins = [
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"},
]
```

Para información completa sobre la creación y uso de plugins, ver [Plugins](/docs/es/agent-sdk/plugins).

<h2 id="message-types">
  Tipos de mensaje
</h2>

<h3 id="message">
  `Message`
</h3>

Tipo de unión de todos los mensajes posibles.

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

Mensaje de entrada del usuario.

```python theme={null}
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
    uuid: str | None = None
    parent_tool_use_id: str | None = None
    tool_use_result: dict[str, Any] | None = None
```

| Campo                | Tipo                        | Descripción                                                                           |
| :------------------- | :-------------------------- | :------------------------------------------------------------------------------------ |
| `content`            | `str \| list[ContentBlock]` | Contenido del mensaje como texto o bloques de contenido                               |
| `uuid`               | `str \| None`               | Identificador único del mensaje                                                       |
| `parent_tool_use_id` | `str \| None`               | ID de uso de herramienta si este mensaje es una respuesta de resultado de herramienta |
| `tool_use_result`    | `dict[str, Any] \| None`    | Datos de resultado de herramienta si es aplicable                                     |

<h3 id="assistantmessage">
  `AssistantMessage`
</h3>

Mensaje de respuesta del asistente con bloques de contenido.

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

| Campo                | Tipo                                                         | Descripción                                                                          |
| :------------------- | :----------------------------------------------------------- | :----------------------------------------------------------------------------------- |
| `content`            | `list[ContentBlock]`                                         | Lista de bloques de contenido en la respuesta                                        |
| `model`              | `str`                                                        | Modelo que generó la respuesta                                                       |
| `parent_tool_use_id` | `str \| None`                                                | ID de uso de herramienta si esta es una respuesta anidada                            |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Tipo de error si la respuesta encontró un error                                      |
| `usage`              | `dict[str, Any] \| None`                                     | Uso de token por mensaje (mismas claves que [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | ID de mensaje de API. Múltiples mensajes de un turno comparten el mismo ID           |

<h3 id="assistantmessageerror">
  `AssistantMessageError`
</h3>

Posibles tipos de error para mensajes del asistente.

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

Mensaje del sistema con metadatos.

```python theme={null}
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

<h3 id="resultmessage">
  `ResultMessage`
</h3>

Mensaje de resultado final con información de costo y uso.

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

El campo `subtype` determina cuáles otros campos se rellenan. Es uno de `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"`, o `"error_max_structured_output_retries"`. La clase de datos de Python aplana todas las variantes en una forma, por lo que los campos que no se aplican al subtipo devuelto son `None`.

Varios campos llevan detalle de diagnóstico cuando la conversación termina en un error:

* `is_error`: `True` cuando la conversación terminó en un estado de error. Siempre `True` en los subtipos `error_*`. En `subtype="success"` es `True` cuando la solicitud del modelo final falló, lo que significa que el bucle del agente se completó pero la última llamada a la API devolvió un error.
* `api_error_status`: el código de estado HTTP del error de API de terminación. `None` cuando el turno terminó sin uno. Se rellena solo en `subtype="success"`.
* `result`: texto del mensaje del asistente final en `subtype="success"`, o `None` en los subtipos `error_*`. Cuando `subtype="success"` e `is_error=True`, esto contiene la cadena de error de API si una está disponible pero puede estar vacía, así que verifique `api_error_status` y el contenido anterior de `AssistantMessage` para obtener detalles.
* `errors`: cadenas de error a nivel de bucle como el mensaje de máx-turnos. Se rellena solo en los subtipos `error_*`.

El dict `usage` contiene las siguientes claves cuando está presente:

| Clave                         | Tipo  | Descripción                                                                                                                                                                                                                          |
| ----------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `input_tokens`                | `int` | Tokens de entrada consumidos por el bucle del agente de nivel superior. [Los tokens de subagentes no se incluyen](/docs/es/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); use `model_usage` para contabilidad de árbol completo. |
| `output_tokens`               | `int` | Tokens de salida generados por el bucle del agente de nivel superior. Los tokens de subagentes no se incluyen.                                                                                                                       |
| `cache_creation_input_tokens` | `int` | Tokens usados para crear nuevas entradas de caché.                                                                                                                                                                                   |
| `cache_read_input_tokens`     | `int` | Tokens leídos de entradas de caché existentes.                                                                                                                                                                                       |

El dict `model_usage` asigna nombres de modelo a uso por modelo. Las claves del dict interno usan camelCase porque el valor se pasa sin modificar desde el proceso CLI subyacente, coincidiendo con el tipo [`ModelUsage`](/docs/es/agent-sdk/typescript#modelusage) de TypeScript:

| Clave                      | Tipo    | Descripción                                                                                                                                                       |
| -------------------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | Tokens de entrada para este modelo.                                                                                                                               |
| `outputTokens`             | `int`   | Tokens de salida para este modelo.                                                                                                                                |
| `cacheReadInputTokens`     | `int`   | Tokens de lectura de caché para este modelo.                                                                                                                      |
| `cacheCreationInputTokens` | `int`   | Tokens de creación de caché para este modelo.                                                                                                                     |
| `webSearchRequests`        | `int`   | Solicitudes de búsqueda web realizadas por este modelo.                                                                                                           |
| `costUSD`                  | `float` | Costo estimado en USD para este modelo, calculado del lado del cliente. Ver [Rastrear costo y uso](/docs/es/agent-sdk/cost-tracking) para advertencias de facturación. |
| `contextWindow`            | `int`   | Tamaño de ventana de contexto para este modelo.                                                                                                                   |
| `maxOutputTokens`          | `int`   | Límite de token de salida máximo para este modelo.                                                                                                                |

<h3 id="streamevent">
  `StreamEvent`
</h3>

Evento de flujo para actualizaciones de mensaje parcial durante el streaming. Solo se recibe cuando `include_partial_messages=True` en `ClaudeAgentOptions`. Importar vía `from claude_agent_sdk.types import StreamEvent`.

```python theme={null}
@dataclass
class StreamEvent:
    uuid: str
    session_id: str
    event: dict[str, Any]  # The raw Claude API stream event
    parent_tool_use_id: str | None = None
```

| Campo                | Tipo             | Descripción                                                                                                                                                                         |
| :------------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `uuid`               | `str`            | Identificador único para este evento                                                                                                                                                |
| `session_id`         | `str`            | Identificador de sesión                                                                                                                                                             |
| `event`              | `dict[str, Any]` | Los datos del evento de flujo de API de Claude sin procesar                                                                                                                         |
| `parent_tool_use_id` | `str \| None`    | Siempre `None`. Los eventos de flujo se emiten solo para la sesión principal. Para la atribución de subagentes, use mensajes completos como [`AssistantMessage`](#assistantmessage) |

<h3 id="ratelimitevent">
  `RateLimitEvent`
</h3>

Emitido cuando el estado del límite de velocidad cambia (por ejemplo, de `"allowed"` a `"allowed_warning"`). Use esto para advertir a los usuarios antes de que alcancen un límite duro, o para retroceder cuando el estado es `"rejected"`.

```python theme={null}
@dataclass
class RateLimitEvent:
    rate_limit_info: RateLimitInfo
    uuid: str
    session_id: str
```

| Campo             | Tipo                              | Descripción                           |
| :---------------- | :-------------------------------- | :------------------------------------ |
| `rate_limit_info` | [`RateLimitInfo`](#ratelimitinfo) | Estado actual del límite de velocidad |
| `uuid`            | `str`                             | Identificador único del evento        |
| `session_id`      | `str`                             | Identificador de sesión               |

<h3 id="ratelimitinfo">
  `RateLimitInfo`
</h3>

Estado del límite de velocidad llevado por [`RateLimitEvent`](#ratelimitevent).

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

| Campo                     | Tipo                      | Descripción                                                                                                       |
| :------------------------ | :------------------------ | :---------------------------------------------------------------------------------------------------------------- |
| `status`                  | `RateLimitStatus`         | Estado actual. `"allowed_warning"` significa acercarse al límite; `"rejected"` significa que se alcanzó el límite |
| `resets_at`               | `int \| None`             | Marca de tiempo Unix cuando se reinicia la ventana del límite de velocidad                                        |
| `rate_limit_type`         | `RateLimitType \| None`   | Qué ventana de límite de velocidad se aplica                                                                      |
| `utilization`             | `float \| None`           | Fracción del límite de velocidad consumido (0.0 a 1.0)                                                            |
| `overage_status`          | `RateLimitStatus \| None` | Estado del uso de exceso de pago por uso, si es aplicable                                                         |
| `overage_resets_at`       | `int \| None`             | Marca de tiempo Unix cuando se reinicia la ventana de exceso                                                      |
| `overage_disabled_reason` | `str \| None`             | Por qué el exceso no está disponible, si el estado es `"rejected"`                                                |
| `raw`                     | `dict[str, Any]`          | Dict sin procesar completo del CLI, incluyendo campos no modelados arriba                                         |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

Emitido cuando comienza una tarea de fondo. Una tarea de fondo es cualquier cosa rastreada fuera del turno principal: un comando Bash en segundo plano, un reloj de Monitor, un subagente generado a través de la herramienta Agent, o un agente remoto. El campo `task_type` le dice cuál. Este nombre no está relacionado con el cambio de nombre de herramienta `Task`-a-`Agent`.

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

| Campo         | Tipo          | Descripción                                                                                                             |
| :------------ | :------------ | :---------------------------------------------------------------------------------------------------------------------- |
| `task_id`     | `str`         | Identificador único para la tarea                                                                                       |
| `description` | `str`         | Descripción de la tarea                                                                                                 |
| `uuid`        | `str`         | Identificador único del mensaje                                                                                         |
| `session_id`  | `str`         | Identificador de sesión                                                                                                 |
| `tool_use_id` | `str \| None` | ID de uso de herramienta asociado                                                                                       |
| `task_type`   | `str \| None` | Qué tipo de tarea de fondo: `"local_bash"` para Bash de fondo y relojes de Monitor, `"local_agent"`, o `"remote_agent"` |

<h3 id="taskusage">
  `TaskUsage`
</h3>

Datos de token y tiempo para una tarea de fondo.

```python theme={null}
class TaskUsage(TypedDict):
    total_tokens: int
    tool_uses: int
    duration_ms: int
```

<h3 id="taskprogressmessage">
  `TaskProgressMessage`
</h3>

Emitido periódicamente con actualizaciones de progreso para una tarea de fondo en ejecución.

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

| Campo            | Tipo          | Descripción                                      |
| :--------------- | :------------ | :----------------------------------------------- |
| `task_id`        | `str`         | Identificador único para la tarea                |
| `description`    | `str`         | Descripción del estado actual                    |
| `usage`          | `TaskUsage`   | Uso de token para esta tarea hasta ahora         |
| `uuid`           | `str`         | Identificador único del mensaje                  |
| `session_id`     | `str`         | Identificador de sesión                          |
| `tool_use_id`    | `str \| None` | ID de uso de herramienta asociado                |
| `last_tool_name` | `str \| None` | Nombre de la última herramienta que usó la tarea |

<h3 id="tasknotificationmessage">
  `TaskNotificationMessage`
</h3>

Emitido cuando una tarea de fondo se completa, falla o se detiene. Las tareas de fondo incluyen comandos Bash `run_in_background`, relojes de Monitor y subagentes de fondo.

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

| Campo         | Tipo                     | Descripción                                     |
| :------------ | :----------------------- | :---------------------------------------------- |
| `task_id`     | `str`                    | Identificador único para la tarea               |
| `status`      | `TaskNotificationStatus` | Uno de `"completed"`, `"failed"`, o `"stopped"` |
| `output_file` | `str`                    | Ruta al archivo de salida de la tarea           |
| `summary`     | `str`                    | Resumen del resultado de la tarea               |
| `uuid`        | `str`                    | Identificador único del mensaje                 |
| `session_id`  | `str`                    | Identificador de sesión                         |
| `tool_use_id` | `str \| None`            | ID de uso de herramienta asociado               |
| `usage`       | `TaskUsage \| None`      | Uso de token final para la tarea                |

<h2 id="content-block-types">
  Tipos de bloque de contenido
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Tipo de unión de todos los bloques de contenido.

```python theme={null}
ContentBlock = TextBlock | ThinkingBlock | ToolUseBlock | ToolResultBlock
```

<h3 id="textblock">
  `TextBlock`
</h3>

Bloque de contenido de texto.

```python theme={null}
@dataclass
class TextBlock:
    text: str
```

<h3 id="thinkingblock">
  `ThinkingBlock`
</h3>

Bloque de contenido de pensamiento (para modelos con capacidad de pensamiento).

```python theme={null}
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

<h3 id="tooluseblock">
  `ToolUseBlock`
</h3>

Bloque de solicitud de uso de herramienta.

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

Bloque de resultado de ejecución de herramienta.

```python theme={null}
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

<h2 id="error-types">
  Tipos de error
</h2>

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Clase de excepción base para todos los errores del SDK.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

<h3 id="clinotfounderror">
  `CLINotFoundError`
</h3>

Se genera cuando Claude Code CLI no está instalado o no se encuentra.

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

Se genera cuando la conexión a Claude Code falla.

```python theme={null}
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

<h3 id="processerror">
  `ProcessError`
</h3>

Se genera cuando el proceso de Claude Code falla.

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

Se genera cuando el análisis JSON falla.

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
  Tipos de hook
</h2>

Para una guía completa sobre el uso de hooks con ejemplos y patrones comunes, ver la [Guía de Hooks](/docs/es/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Tipos de evento de hook soportados.

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
  El SDK de TypeScript admite eventos de hook adicionales no disponibles aún en Python: `SessionStart`, `SessionEnd`, `Setup`, `TeammateIdle`, `TaskCompleted`, `ConfigChange`, `WorktreeCreate`, `WorktreeRemove`, `PostToolBatch` y `MessageDisplay`.
</Note>

<h3 id="hookcallback">
  `HookCallback`
</h3>

Definición de tipo para funciones de devolución de llamada de hook.

```python theme={null}
HookCallback = Callable[[HookInput, str | None, HookContext], Awaitable[HookJSONOutput]]
```

Parámetros:

* `input`: Entrada de hook fuertemente tipada con uniones discriminadas basadas en `hook_event_name` (ver [`HookInput`](#hookinput))
* `tool_use_id`: Identificador de uso de herramienta opcional (para hooks relacionados con herramientas)
* `context`: Contexto de hook con información adicional

Devuelve un [`HookJSONOutput`](#hookjsonoutput) que puede contener:

* `decision`: `"block"` para bloquear la acción
* `systemMessage`: Mensaje de advertencia mostrado al usuario
* `hookSpecificOutput`: Datos de salida específicos del hook

<h3 id="hookcontext">
  `HookContext`
</h3>

Información de contexto pasada a devoluciones de llamada de hook.

```python theme={null}
class HookContext(TypedDict):
    signal: Any | None  # Future: abort signal support
```

<h3 id="hookmatcher">
  `HookMatcher`
</h3>

Configuración para hacer coincidir hooks con eventos o herramientas específicas.

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

Tipo de unión de todos los tipos de entrada de hook. El tipo real depende del campo `hook_event_name`.

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

Campos base presentes en todos los tipos de entrada de hook.

```python theme={null}
class BaseHookInput(TypedDict):
    session_id: str
    transcript_path: str
    cwd: str
    permission_mode: NotRequired[str]
```

| Campo             | Tipo             | Descripción                                |
| :---------------- | :--------------- | :----------------------------------------- |
| `session_id`      | `str`            | Identificador de sesión actual             |
| `transcript_path` | `str`            | Ruta al archivo de transcripción de sesión |
| `cwd`             | `str`            | Directorio de trabajo actual               |
| `permission_mode` | `str` (opcional) | Modo de permiso actual                     |

<h3 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h3>

Datos de entrada para eventos de hook `PreToolUse`.

```python theme={null}
class PreToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PreToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Campo             | Tipo                    | Descripción                                                                           |
| :---------------- | :---------------------- | :------------------------------------------------------------------------------------ |
| `hook_event_name` | `Literal["PreToolUse"]` | Siempre "PreToolUse"                                                                  |
| `tool_name`       | `str`                   | Nombre de la herramienta a punto de ejecutarse                                        |
| `tool_input`      | `dict[str, Any]`        | Parámetros de entrada para la herramienta                                             |
| `tool_use_id`     | `str`                   | Identificador único para este uso de herramienta                                      |
| `agent_id`        | `str` (opcional)        | Identificador de subagente, presente cuando el hook se dispara dentro de un subagente |
| `agent_type`      | `str` (opcional)        | Tipo de subagente, presente cuando el hook se dispara dentro de un subagente          |

<h3 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h3>

Datos de entrada para eventos de hook `PostToolUse`.

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

| Campo             | Tipo                     | Descripción                                                                           |
| :---------------- | :----------------------- | :------------------------------------------------------------------------------------ |
| `hook_event_name` | `Literal["PostToolUse"]` | Siempre "PostToolUse"                                                                 |
| `tool_name`       | `str`                    | Nombre de la herramienta que se ejecutó                                               |
| `tool_input`      | `dict[str, Any]`         | Parámetros de entrada que se utilizaron                                               |
| `tool_response`   | `Any`                    | Respuesta de la ejecución de la herramienta                                           |
| `tool_use_id`     | `str`                    | Identificador único para este uso de herramienta                                      |
| `agent_id`        | `str` (opcional)         | Identificador de subagente, presente cuando el hook se dispara dentro de un subagente |
| `agent_type`      | `str` (opcional)         | Tipo de subagente, presente cuando el hook se dispara dentro de un subagente          |

<h3 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h3>

Datos de entrada para eventos de hook `PostToolUseFailure`. Se llama cuando la ejecución de una herramienta falla.

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

| Campo             | Tipo                            | Descripción                                                                           |
| :---------------- | :------------------------------ | :------------------------------------------------------------------------------------ |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Siempre "PostToolUseFailure"                                                          |
| `tool_name`       | `str`                           | Nombre de la herramienta que falló                                                    |
| `tool_input`      | `dict[str, Any]`                | Parámetros de entrada que se utilizaron                                               |
| `tool_use_id`     | `str`                           | Identificador único para este uso de herramienta                                      |
| `error`           | `str`                           | Mensaje de error de la ejecución fallida                                              |
| `is_interrupt`    | `bool` (opcional)               | Si el fallo fue causado por una interrupción                                          |
| `agent_id`        | `str` (opcional)                | Identificador de subagente, presente cuando el hook se dispara dentro de un subagente |
| `agent_type`      | `str` (opcional)                | Tipo de subagente, presente cuando el hook se dispara dentro de un subagente          |

<h3 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h3>

Datos de entrada para eventos de hook `UserPromptSubmit`.

```python theme={null}
class UserPromptSubmitHookInput(BaseHookInput):
    hook_event_name: Literal["UserPromptSubmit"]
    prompt: str
```

| Campo             | Tipo                          | Descripción                      |
| :---------------- | :---------------------------- | :------------------------------- |
| `hook_event_name` | `Literal["UserPromptSubmit"]` | Siempre "UserPromptSubmit"       |
| `prompt`          | `str`                         | El prompt enviado por el usuario |

<h3 id="stophookinput">
  `StopHookInput`
</h3>

Datos de entrada para eventos de hook `Stop`.

```python theme={null}
class StopHookInput(BaseHookInput):
    hook_event_name: Literal["Stop"]
    stop_hook_active: bool
```

| Campo              | Tipo              | Descripción                      |
| :----------------- | :---------------- | :------------------------------- |
| `hook_event_name`  | `Literal["Stop"]` | Siempre "Stop"                   |
| `stop_hook_active` | `bool`            | Si el hook de parada está activo |

<h3 id="subagentstophookinput">
  `SubagentStopHookInput`
</h3>

Datos de entrada para eventos de hook `SubagentStop`.

```python theme={null}
class SubagentStopHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStop"]
    stop_hook_active: bool
    agent_id: str
    agent_transcript_path: str
    agent_type: str
```

| Campo                   | Tipo                      | Descripción                                    |
| :---------------------- | :------------------------ | :--------------------------------------------- |
| `hook_event_name`       | `Literal["SubagentStop"]` | Siempre "SubagentStop"                         |
| `stop_hook_active`      | `bool`                    | Si el hook de parada está activo               |
| `agent_id`              | `str`                     | Identificador único para el subagente          |
| `agent_transcript_path` | `str`                     | Ruta al archivo de transcripción del subagente |
| `agent_type`            | `str`                     | Tipo del subagente                             |

<h3 id="precompacthookinput">
  `PreCompactHookInput`
</h3>

Datos de entrada para eventos de hook `PreCompact`.

```python theme={null}
class PreCompactHookInput(BaseHookInput):
    hook_event_name: Literal["PreCompact"]
    trigger: Literal["manual", "auto"]
    custom_instructions: str | None
```

| Campo                 | Tipo                        | Descripción                                    |
| :-------------------- | :-------------------------- | :--------------------------------------------- |
| `hook_event_name`     | `Literal["PreCompact"]`     | Siempre "PreCompact"                           |
| `trigger`             | `Literal["manual", "auto"]` | Qué desencadenó la compactación                |
| `custom_instructions` | `str \| None`               | Instrucciones personalizadas para compactación |

<h3 id="notificationhookinput">
  `NotificationHookInput`
</h3>

Datos de entrada para eventos de hook `Notification`.

```python theme={null}
class NotificationHookInput(BaseHookInput):
    hook_event_name: Literal["Notification"]
    message: str
    title: NotRequired[str]
    notification_type: str
```

| Campo               | Tipo                      | Descripción                           |
| :------------------ | :------------------------ | :------------------------------------ |
| `hook_event_name`   | `Literal["Notification"]` | Siempre "Notification"                |
| `message`           | `str`                     | Contenido del mensaje de notificación |
| `title`             | `str` (opcional)          | Título de la notificación             |
| `notification_type` | `str`                     | Tipo de notificación                  |

<h3 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h3>

Datos de entrada para eventos de hook `SubagentStart`.

```python theme={null}
class SubagentStartHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStart"]
    agent_id: str
    agent_type: str
```

| Campo             | Tipo                       | Descripción                           |
| :---------------- | :------------------------- | :------------------------------------ |
| `hook_event_name` | `Literal["SubagentStart"]` | Siempre "SubagentStart"               |
| `agent_id`        | `str`                      | Identificador único para el subagente |
| `agent_type`      | `str`                      | Tipo del subagente                    |

<h3 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h3>

Datos de entrada para eventos de hook `PermissionRequest`. Permite que los hooks manejen decisiones de permiso programáticamente.

```python theme={null}
class PermissionRequestHookInput(BaseHookInput):
    hook_event_name: Literal["PermissionRequest"]
    tool_name: str
    tool_input: dict[str, Any]
    permission_suggestions: NotRequired[list[Any]]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Campo                    | Tipo                           | Descripción                                                                           |
| :----------------------- | :----------------------------- | :------------------------------------------------------------------------------------ |
| `hook_event_name`        | `Literal["PermissionRequest"]` | Siempre "PermissionRequest"                                                           |
| `tool_name`              | `str`                          | Nombre de la herramienta solicitando permiso                                          |
| `tool_input`             | `dict[str, Any]`               | Parámetros de entrada para la herramienta                                             |
| `permission_suggestions` | `list[Any]` (opcional)         | Actualizaciones de permiso sugeridas del CLI                                          |
| `agent_id`               | `str` (opcional)               | Identificador de subagente, presente cuando el hook se dispara dentro de un subagente |
| `agent_type`             | `str` (opcional)               | Tipo de subagente, presente cuando el hook se dispara dentro de un subagente          |

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Tipo de unión para valores de retorno de devolución de llamada de hook.

```python theme={null}
HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

Salida de hook sincrónico con campos de control y decisión.

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
  Use `continue_` (con guion bajo) en código Python. Se convierte automáticamente a `continue` cuando se envía al CLI.
</Note>

<h4 id="hookspecificoutput">
  `HookSpecificOutput`
</h4>

Un `TypedDict` que contiene el nombre del evento de hook y campos específicos del evento. La forma depende del valor `hookEventName`. Para detalles completos sobre campos disponibles por evento de hook, ver [Control execution with hooks](/docs/es/agent-sdk/hooks#outputs).

Una unión discriminada de tipos de salida específicos del evento. El campo `hookEventName` determina qué campos son válidos.

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

Salida de hook asincrónico que difiere la ejecución del hook.

```python theme={null}
class AsyncHookJSONOutput(TypedDict):
    async_: Literal[True]  # Set to True to defer execution
    asyncTimeout: NotRequired[int]  # Timeout in milliseconds
```

<Note>
  Use `async_` (con guion bajo) en código Python. Se convierte automáticamente a `async` cuando se envía al CLI.
</Note>

<h3 id="hook-usage-example">
  Ejemplo de uso de hook
</h3>

Este ejemplo registra dos hooks: uno que bloquea comandos bash peligrosos como `rm -rf /`, y otro que registra todo el uso de herramientas para auditoría. El hook de seguridad solo se ejecuta en comandos Bash (a través del `matcher`), mientras que el hook de registro se ejecuta en todas las herramientas.

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
  Tipos de entrada/salida de herramienta
</h2>

Documentación de esquemas de entrada/salida para todas las herramientas integradas de Claude Code. Aunque el SDK de Python no exporta estos como tipos, representan la estructura de entradas y salidas de herramientas en mensajes.

<h3 id="agent">
  Agent
</h3>

**Nombre de herramienta:** `Agent` (anteriormente `Task`, que aún se acepta como alias)

**Entrada:**

```python theme={null}
{
    "description": str,  # A short (3-5 word) description of the task
    "prompt": str,  # The task for the agent to perform
    "subagent_type": str,  # The type of specialized agent to use
}
```

**Salida:**

```python theme={null}
{
    "result": str,  # Final result from the subagent
    "usage": dict | None,  # Token usage statistics
    "total_cost_usd": float | None,  # Estimated total cost in USD
    "duration_ms": int | None,  # Execution duration in milliseconds
}
```

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Nombre de herramienta:** `AskUserQuestion`

Hace preguntas aclaratorias al usuario durante la ejecución. Ver [Manejar aprobaciones e entrada del usuario](/docs/es/agent-sdk/user-input#handle-clarifying-questions) para detalles de uso.

**Entrada:**

```python theme={null}
{
    "questions": [  # Questions to ask the user (1-4 questions)
        {
            "question": str,  # The complete question to ask the user
            "header": str,  # Very short label displayed as a chip/tag (max 12 chars)
            "options": [  # The available choices (2-4 options)
                {
                    "label": str,  # Display text for this option (1-5 words)
                    "description": str,  # Explanation of what this option means
                }
            ],
            "multiSelect": bool,  # Set to true to allow multiple selections
        }
    ],
    "answers": dict[str, str | list[str]] | None,
    # User answers populated by the permission system. Multi-select
    # answers may be a list of labels or a comma-joined string
}
```

**Salida:**

```python theme={null}
{
    "questions": [  # The questions that were asked
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Maps question text to answer string
    # Multi-select answers are comma-separated
}
```

<h3 id="bash">
  Bash
</h3>

**Nombre de herramienta:** `Bash`

**Entrada:**

```python theme={null}
{
    "command": str,  # The command to execute
    "timeout": int | None,  # Optional timeout in milliseconds (max 600000; higher values are clamped to the max)
    "description": str | None,  # Clear, concise description (5-10 words)
    "run_in_background": bool | None,  # Set to true to run in background
}
```

**Salida:**

```python theme={null}
{
    "output": str,  # Combined stdout and stderr output
    "exitCode": int,  # Exit code of the command
    "killed": bool | None,  # Whether command was killed due to timeout
    "shellId": str | None,  # Shell ID for background processes
}
```

<h3 id="monitor">
  Monitor
</h3>

**Nombre de herramienta:** `Monitor`

Ejecuta una fuente de fondo y entrega cada evento a Claude para que pueda reaccionar sin sondeo: `command` ejecuta un script y emite un evento por línea stdout, y `ws` abre un WebSocket y emite un evento por marco de texto. Proporcione exactamente uno de `command` o `ws`.

Cuando Monitor ejecuta un comando, sigue las mismas reglas de permiso que Bash; una vigilancia de WebSocket solicita aprobación por separado. La fuente `ws` requiere Claude Code v2.1.195 o posterior. Ver la [referencia de herramienta Monitor](/docs/es/tools-reference#monitor-tool) para comportamiento y disponibilidad de proveedor.

**Entrada:**

```python theme={null}
{
    "command": str | None,  # Shell script; each stdout line is an event, exit ends the watch
    "ws": dict | None,  # WebSocket source: {"url": str, "protocols": list[str] | None}; each text frame is an event
    "description": str,  # Short description shown in notifications
    "timeout_ms": int | None,  # Kill after this deadline (default 300000, max 3600000)
    "persistent": bool | None,  # Run for the lifetime of the session; stop with TaskStop
}
```

**Salida:**

```python theme={null}
{
    "taskId": str,  # ID of the background monitor task
    "timeoutMs": int,  # Timeout deadline in milliseconds (0 when persistent)
    "persistent": bool | None,  # True when running until TaskStop or session end
}
```

<h3 id="edit">
  Edit
</h3>

**Nombre de herramienta:** `Edit`

**Entrada:**

```python theme={null}
{
    "file_path": str,  # The absolute path to the file to modify
    "old_string": str,  # The text to replace
    "new_string": str,  # The text to replace it with
    "replace_all": bool | None,  # Replace all occurrences (default False)
}
```

**Salida:**

```python theme={null}
{
    "message": str,  # Confirmation message
    "replacements": int,  # Number of replacements made
    "file_path": str,  # File path that was edited
}
```

<h3 id="read">
  Read
</h3>

**Nombre de herramienta:** `Read`

**Entrada:**

```python theme={null}
{
    "file_path": str,  # The absolute path to the file to read
    "offset": int | None,  # The line number to start reading from
    "limit": int | None,  # The number of lines to read
}
```

**Salida (archivos de texto):**

```python theme={null}
{
    "content": str,  # File contents with line numbers
    "total_lines": int,  # Total number of lines in file
    "lines_returned": int,  # Lines actually returned
}
```

**Salida (imágenes):**

```python theme={null}
{
    "image": str,  # Base64 encoded image data
    "mime_type": str,  # Image MIME type
    "file_size": int,  # File size in bytes
}
```

<h3 id="write">
  Write
</h3>

**Nombre de herramienta:** `Write`

**Entrada:**

```python theme={null}
{
    "file_path": str,  # The absolute path to the file to write
    "content": str,  # The content to write to the file
}
```

**Salida:**

```python theme={null}
{
    "message": str,  # Success message
    "bytes_written": int,  # Number of bytes written
    "file_path": str,  # File path that was written
}
```

<h3 id="glob">
  Glob
</h3>

**Nombre de herramienta:** `Glob`

**Entrada:**

```python theme={null}
{
    "pattern": str,  # The glob pattern to match files against
    "path": str | None,  # The directory to search in (defaults to cwd)
}
```

**Salida:**

```python theme={null}
{
    "matches": list[str],  # Array of matching file paths
    "count": int,  # Number of matches found
    "search_path": str,  # Search directory used
}
```

<h3 id="grep">
  Grep
</h3>

**Nombre de herramienta:** `Grep`

**Entrada:**

```python theme={null}
{
    "pattern": str,  # The regular expression pattern
    "path": str | None,  # File or directory to search in
    "glob": str | None,  # Glob pattern to filter files
    "type": str | None,  # File type to search
    "output_mode": str | None,  # "content", "files_with_matches", or "count"
    "-i": bool | None,  # Case insensitive search
    "-n": bool | None,  # Show line numbers
    "-B": int | None,  # Lines to show before each match
    "-A": int | None,  # Lines to show after each match
    "-C": int | None,  # Lines to show before and after
    "head_limit": int | None,  # Limit output to first N lines/entries
    "multiline": bool | None,  # Enable multiline mode
}
```

**Salida (modo content):**

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

**Salida (modo files\_with\_matches):**

```python theme={null}
{
    "files": list[str],  # Files containing matches
    "count": int,  # Number of files with matches
}
```

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Nombre de herramienta:** `NotebookEdit`

**Entrada:**

```python theme={null}
{
    "notebook_path": str,  # Absolute path to the Jupyter notebook
    "cell_id": str | None,  # The ID of the cell to edit
    "new_source": str,  # The new source for the cell
    "cell_type": "code" | "markdown" | None,  # The type of the cell
    "edit_mode": "replace" | "insert" | "delete" | None,  # Edit operation type
}
```

**Salida:**

```python theme={null}
{
    "message": str,  # Success message
    "edit_type": "replaced" | "inserted" | "deleted",  # Type of edit performed
    "cell_id": str | None,  # Cell ID that was affected
    "total_cells": int,  # Total cells in notebook after edit
}
```

<h3 id="webfetch">
  WebFetch
</h3>

**Nombre de herramienta:** `WebFetch`

**Entrada:**

```python theme={null}
{
    "url": str,  # The URL to fetch content from
    "prompt": str,  # The prompt to run on the fetched content
}
```

**Salida:**

```python theme={null}
{
    "bytes": int,  # Size of the fetched content in bytes
    "code": int,  # HTTP response code
    "codeText": str,  # HTTP response code text
    "result": str,  # Processed result from applying the prompt to the content
    "durationMs": int,  # Time to fetch and process the content, in milliseconds
    "url": str,  # URL that was fetched
}
```

<h3 id="websearch">
  WebSearch
</h3>

**Nombre de herramienta:** `WebSearch`

**Entrada:**

```python theme={null}
{
    "query": str,  # The search query to use
    "allowed_domains": list[str] | None,  # Only include results from these domains
    "blocked_domains": list[str] | None,  # Never include results from these domains
}
```

**Salida:**

```python theme={null}
{
    "query": str,  # The search query
    "results": list[str | {"tool_use_id": str, "content": list[{"title": str, "url": str}]}],
    "durationSeconds": float,  # Search duration in seconds
}
```

<h3 id="todowrite">
  TodoWrite
</h3>

**Nombre de herramienta:** `TodoWrite`

<Note>
  A partir de Claude Code v2.1.142, `TodoWrite` está deshabilitado de forma predeterminada. Use `TaskCreate`, `TaskGet`, `TaskUpdate`, y `TaskList` en su lugar. Ver [Migrar a herramientas Task](/docs/es/agent-sdk/todo-tracking#migrate-to-task-tools) para actualizar su código de monitoreo, o establezca `CLAUDE_CODE_ENABLE_TASKS=0` para revertir a `TodoWrite`.
</Note>

**Entrada:**

```python theme={null}
{
    "todos": [
        {
            "content": str,  # The task description
            "status": "pending" | "in_progress" | "completed",  # Task status
            "activeForm": str,  # Active form of the description
        }
    ]
}
```

**Salida:**

```python theme={null}
{
    "message": str,  # Success message
    "stats": {"total": int, "pending": int, "in_progress": int, "completed": int},
}
```

<h3 id="taskcreate">
  TaskCreate
</h3>

**Nombre de herramienta:** `TaskCreate`

**Entrada:**

```python theme={null}
{
    "subject": str,  # Short task title
    "description": str,  # Detailed task body
    "activeForm": str | None,  # Present-tense label shown while in progress
    "metadata": dict | None,  # Arbitrary caller metadata
}
```

**Salida:**

```python theme={null}
{
    "task": {"id": str, "subject": str},  # Created task with assigned ID
}
```

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Nombre de herramienta:** `TaskUpdate`

**Entrada:**

```python theme={null}
{
    "taskId": str,  # ID of the task to patch
    "status": Literal["pending", "in_progress", "completed", "deleted"] | None,
    "subject": str | None,
    "description": str | None,
    "activeForm": str | None,
    "addBlocks": list[str] | None,  # Task IDs this task now blocks
    "addBlockedBy": list[str] | None,  # Task IDs that now block this task
    "owner": str | None,
    "metadata": dict | None,
}
```

**Salida:**

```python theme={null}
{
    "success": bool,
    "taskId": str,
    "updatedFields": list[str],  # Names of fields that changed
    "error": str | None,
    "statusChange": {"from": str, "to": str} | None,
}
```

<h3 id="taskget">
  TaskGet
</h3>

**Nombre de herramienta:** `TaskGet`

**Entrada:**

```python theme={null}
{
    "taskId": str,  # ID of the task to read
}
```

**Salida:**

```python theme={null}
{
    "task": {
        "id": str,
        "subject": str,
        "description": str,
        "status": Literal["pending", "in_progress", "completed"],
        "blocks": list[str],
        "blockedBy": list[str],
    } | None,  # None when the ID is not found
}
```

<h3 id="tasklist">
  TaskList
</h3>

**Nombre de herramienta:** `TaskList`

**Entrada:**

```python theme={null}
{}
```

**Salida:**

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

**Nombre de herramienta:** `BashOutput`

**Entrada:**

```python theme={null}
{
    "bash_id": str,  # The ID of the background shell
    "filter": str | None,  # Optional regex to filter output lines
}
```

**Salida:**

```python theme={null}
{
    "output": str,  # New output since last check
    "status": "running" | "completed" | "failed",  # Current shell status
    "exitCode": int | None,  # Exit code when completed
}
```

<h3 id="killbash">
  KillBash
</h3>

**Nombre de herramienta:** `KillBash`

**Entrada:**

```python theme={null}
{
    "shell_id": str  # The ID of the background shell to kill
}
```

**Salida:**

```python theme={null}
{
    "message": str,  # Success message
    "shell_id": str,  # ID of the killed shell
}
```

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nombre de herramienta:** `ExitPlanMode`

**Entrada:**

```python theme={null}
{
    "plan": str  # The plan to run by the user for approval
}
```

**Salida:**

```python theme={null}
{
    "message": str,  # Confirmation message
    "approved": bool | None,  # Whether user approved the plan
}
```

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nombre de herramienta:** `ListMcpResourcesTool`

**Entrada:**

```python theme={null}
{
    "server": str | None  # Optional server name to filter resources by
}
```

**Salida:**

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

**Nombre de herramienta:** `ReadMcpResourceTool`

**Entrada:**

```python theme={null}
{
    "server": str,  # The MCP server name
    "uri": str,  # The resource URI to read
}
```

**Salida:**

```python theme={null}
{
    "contents": [
        {"uri": str, "mimeType": str | None, "text": str | None, "blob": str | None}
    ],
    "server": str,
}
```

<h2 id="advanced-features-with-claudesdkclient">
  Características avanzadas con ClaudeSDKClient
</h2>

<h3 id="building-a-continuous-conversation-interface">
  Construir una interfaz de conversación continua
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
  Usar hooks para modificación de comportamiento
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
  Monitoreo de progreso en tiempo real
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
  Uso de ejemplo
</h2>

<h3 id="basic-file-operations-using-query">
  Operaciones básicas de archivo (usando query)
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
  Manejo de errores
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
  Modo de streaming con cliente
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
  Usar herramientas personalizadas con ClaudeSDKClient
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
  Configuración de sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Configuración para el comportamiento de sandbox. Use esto para habilitar el sandboxing de comandos y configurar restricciones de red programáticamente.

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

| Propiedad                   | Tipo                                                  | Predeterminado | Descripción                                                                                                                                                                                                                                                  |
| :-------------------------- | :---------------------------------------------------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `bool`                                                | `False`        | Habilitar modo sandbox para ejecución de comandos                                                                                                                                                                                                            |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`         | Aprobar automáticamente comandos bash cuando sandbox está habilitado                                                                                                                                                                                         |
| `excludedCommands`          | `list[str]`                                           | `[]`           | Comandos que siempre evitan restricciones de sandbox (por ejemplo, `["docker"]`). Estos se ejecutan sin sandbox automáticamente sin participación del modelo                                                                                                 |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`         | Permitir que el modelo solicite ejecutar comandos fuera del sandbox. Cuando es `True`, el modelo puede establecer `dangerouslyDisableSandbox` en entrada de herramienta, que vuelve al [sistema de permisos](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`         | Configuración de sandbox específica de red                                                                                                                                                                                                                   |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`         | Configurar qué violaciones de sandbox ignorar                                                                                                                                                                                                                |
| `enableWeakerNestedSandbox` | `bool`                                                | `False`        | Habilitar un sandbox anidado más débil para compatibilidad                                                                                                                                                                                                   |

<Note>
  El sandbox depende de la compatibilidad de la plataforma y, en Linux, de herramientas como `bubblewrap` y `socat`. De forma predeterminada, cuando `enabled` es `True` pero el sandbox no puede iniciarse, los comandos se ejecutan sin sandbox con una advertencia en stderr. Este comportamiento predeterminado difiere del SDK de TypeScript, donde `failIfUnavailable` tiene un valor predeterminado de `true`.

  Establezca `"failIfUnavailable": True` en su configuración de sandbox para detener en su lugar. La clave aún no está declarada en `SandboxSettings`, pero el SDK la reenvía a Claude Code, que la respeta. `query()` luego reporta un `ResultMessage` con `subtype="error_during_execution"` y la razón en `errors`. Observe ese subtipo en lugar de esperar que `query()` lance una excepción antes de ceder mensajes.
</Note>

<h4 id="example-usage-2">
  Ejemplo de uso
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
  **Seguridad de socket Unix**: La opción `allowUnixSockets` puede otorgar acceso a servicios del sistema poderosos. Por ejemplo, permitir `/var/run/docker.sock` efectivamente otorga acceso completo al sistema host a través de la API de Docker, evitando el aislamiento de sandbox. Solo permita sockets Unix que sean estrictamente necesarios y comprenda las implicaciones de seguridad de cada uno.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configuración específica de red para modo sandbox. Estas configuraciones se aplican a comandos Bash en sandbox cuando `enabled` es `True` en la [`SandboxSettings`](#sandboxsettings) principal. No restringen la herramienta WebFetch, que utiliza [reglas de permisos](/docs/es/permissions#webfetch) en su lugar.

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

| Propiedad                 | Tipo        | Predeterminado | Descripción                                                                                                                                                                                                                |
| :------------------------ | :---------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `list[str]` | `[]`           | Nombres de dominio que los procesos en sandbox pueden acceder                                                                                                                                                              |
| `deniedDomains`           | `list[str]` | `[]`           | Nombres de dominio que los procesos en sandbox no pueden acceder. Tiene precedencia sobre `allowedDomains`                                                                                                                 |
| `allowManagedDomainsOnly` | `bool`      | `False`        | Solo configuración administrada: cuando se establece en configuración administrada, ignorar `allowedDomains` de fuentes de configuración no administradas. No tiene efecto cuando se establece a través de opciones de SDK |
| `allowUnixSockets`        | `list[str]` | `[]`           | Rutas de socket Unix que los procesos pueden acceder (por ejemplo, socket de Docker)                                                                                                                                       |
| `allowAllUnixSockets`     | `bool`      | `False`        | Permitir acceso a todos los sockets Unix                                                                                                                                                                                   |
| `allowLocalBinding`       | `bool`      | `False`        | Permitir que los procesos se vinculen a puertos locales (por ejemplo, para servidores de desarrollo)                                                                                                                       |
| `allowMachLookup`         | `list[str]` | `[]`           | Solo macOS: nombres de servicios XPC/Mach para permitir. Admite un comodín al final                                                                                                                                        |
| `httpProxyPort`           | `int`       | `None`         | Puerto proxy HTTP para solicitudes de red                                                                                                                                                                                  |
| `socksProxyPort`          | `int`       | `None`         | Puerto proxy SOCKS para solicitudes de red                                                                                                                                                                                 |

<Note>
  El proxy de sandbox integrado aplica la lista de permitidos de red basada en el nombre de host solicitado y no termina ni inspecciona el tráfico TLS, por lo que técnicas como [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) potencialmente pueden evitarlo. Consulte [Limitaciones de seguridad de sandboxing](/docs/es/sandboxing#security-limitations) para obtener detalles y [Implementación segura](/docs/es/agent-sdk/secure-deployment#traffic-forwarding) para configurar un proxy que termine TLS.
</Note>

<h3 id="sandboxignoreviolations">
  `SandboxIgnoreViolations`
</h3>

Configuración para ignorar violaciones de sandbox específicas.

```python theme={null}
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| Propiedad | Tipo        | Predeterminado | Descripción                                          |
| :-------- | :---------- | :------------- | :--------------------------------------------------- |
| `file`    | `list[str]` | `[]`           | Patrones de ruta de archivo para ignorar violaciones |
| `network` | `list[str]` | `[]`           | Patrones de red para ignorar violaciones             |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Respaldo de permisos para comandos sin sandbox
</h3>

Cuando `allowUnsandboxedCommands` está habilitado, el modelo puede solicitar ejecutar comandos fuera del sandbox estableciendo `dangerouslyDisableSandbox: True` en la entrada de la herramienta. Estas solicitudes vuelven al sistema de permisos existente, lo que significa que se invocará su controlador `can_use_tool`, permitiéndole implementar lógica de autorización personalizada.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Una lista estática de comandos que siempre evitan el sandbox automáticamente (por ejemplo, `["docker"]`). El modelo no tiene control sobre esto.
  * `allowUnsandboxedCommands`: Permite que el modelo decida en tiempo de ejecución si solicitar ejecución sin sandbox estableciendo `dangerouslyDisableSandbox: True` en la entrada de la herramienta.
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

Este patrón le permite:

* **Auditar solicitudes del modelo**: Registrar cuándo el modelo solicita ejecución sin sandbox
* **Implementar listas de permitidos**: Solo permitir comandos específicos para ejecutarse sin sandbox
* **Agregar flujos de trabajo de aprobación**: Requerir autorización explícita para operaciones privilegiadas

<Warning>
  Los comandos que se ejecutan con `dangerouslyDisableSandbox: True` tienen acceso completo al sistema. Asegúrese de que su controlador `can_use_tool` valide estas solicitudes cuidadosamente.

  Si `permission_mode` se establece en `bypassPermissions` y `allow_unsandboxed_commands` está habilitado, el modelo puede ejecutar autónomamente comandos fuera del sandbox sin solicitudes de aprobación. Esta combinación efectivamente permite que el modelo escape del aislamiento de sandbox silenciosamente.
</Warning>

<h2 id="see-also">
  Ver también
</h2>

* [SDK overview](/docs/es/agent-sdk/overview) - Conceptos generales del SDK
* [TypeScript SDK reference](/docs/es/agent-sdk/typescript) - Documentación del SDK de TypeScript
* [CLI reference](/docs/es/cli-reference) - Interfaz de línea de comandos
* [Common workflows](/docs/es/common-workflows) - Guías paso a paso
