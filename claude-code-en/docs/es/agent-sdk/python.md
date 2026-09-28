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

Para uv, Windows PowerShell y configuración de claves API, consulte [Configuración en la guía de inicio rápido del Agent SDK](/docs/es/agent-sdk/quickstart#setup).

<h2 id="choosing-between-query-and-claudesdkclient">
  Elegir entre `query()` y `ClaudeSDKClient`
</h2>

El SDK de Python proporciona dos formas de interactuar con Claude Code:

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

Utilice `ClaudeSDKClient` para aplicaciones interactivas como interfaces de chat, o cuando la siguiente acción dependa de la respuesta de Claude.

<h2 id="functions">
  Funciones
</h2>

<Note>Los bloques de firma y fragmentos desnudos de `async for` / `async with` en esta página son ilustrativos. Para ejecutarlos, envuelva el cuerpo en `async def main(): ...` y llame a `asyncio.run(main())`.</Note>

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

| Parámetro      | Tipo                                            | Descripción                                                                                                                      |
| :------------- | :---------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Identificador único para la herramienta                                                                                          |
| `description`  | `str`                                           | Descripción legible de lo que hace la herramienta                                                                                |
| `input_schema` | `type \| dict[str, Any]`                        | Esquema que define los parámetros de entrada de la herramienta. Consulte [Opciones de esquema de entrada](#input-schema-options) |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Anotaciones opcionales de herramienta MCP que proporcionan sugerencias de comportamiento a los clientes                          |

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

Sugerencias de comportamiento para una herramienta, pasadas como el argumento `annotations` de [`tool()`](#tool). `ToolAnnotations` extiende `mcp.types.ToolAnnotations` del SDK de MCP con un campo `maxResultSizeChars`, y puede escribir cada sugerencia en camelCase o snake\_case: `ToolAnnotations(readOnlyHint=True)` y `ToolAnnotations(read_only_hint=True)` son equivalentes. También puede pasar un `mcp.types.ToolAnnotations` simple dondequiera que el SDK acepte anotaciones.

Los nombres snake\_case y el campo `maxResultSizeChars` tipado requieren Python Agent SDK 0.2.140 o posterior. Las versiones 0.1.31 a 0.2.139 re-exportan `mcp.types.ToolAnnotations` sin cambios. En las versiones 0.1.55 a 0.2.139 aún puede pasar `maxResultSizeChars` como argumento de palabra clave: la clase MCP acepta campos adicionales, y el SDK reenvía el valor a Claude Code.

Todos los campos son opcionales. Los clientes no deben depender de las sugerencias para decisiones de seguridad.

| Campo                | Tipo           | Predeterminado | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------- | :------------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`              | `str \| None`  | `None`         | Título legible para la herramienta                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `readOnlyHint`       | `bool \| None` | `False`        | Si es `True`, la herramienta no modifica su entorno                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `destructiveHint`    | `bool \| None` | `True`         | Si es `True`, la herramienta puede realizar actualizaciones destructivas (solo significativo cuando `readOnlyHint` es `False`)                                                                                                                                                                                                                                                                                                                                                                 |
| `idempotentHint`     | `bool \| None` | `False`        | Si es `True`, las llamadas repetidas con los mismos argumentos no tienen efecto adicional (solo significativo cuando `readOnlyHint` es `False`)                                                                                                                                                                                                                                                                                                                                                |
| `openWorldHint`      | `bool \| None` | `True`         | Si es `True`, la herramienta interactúa con entidades externas (por ejemplo, búsqueda web). Si es `False`, el dominio de la herramienta es cerrado (por ejemplo, una herramienta de memoria)                                                                                                                                                                                                                                                                                                   |
| `maxResultSizeChars` | `int \| None`  | `None`         | Número de caracteres hasta los cuales Claude Code mantiene el resultado de texto de esta herramienta en línea en la conversación en lugar de guardarlo en un archivo, hasta 500.000. Los resultados que contienen imágenes no se ven afectados. Una configuración de Claude Code en lugar de una sugerencia MCP: el SDK la envía en `_meta` de la herramienta como `anthropic/maxResultSizeChars`. Consulte [Raise the limit for a specific tool](/docs/es/mcp#raise-the-limit-for-a-specific-tool) |

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

| Propiedad            | Tipo                           | Descripción                                                                                                                                                                                                                                                                                     |
| :------------------- | :----------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `Literal["user", "assistant"]` | Rol del mensaje                                                                                                                                                                                                                                                                                 |
| `uuid`               | `str`                          | Identificador único del mensaje                                                                                                                                                                                                                                                                 |
| `session_id`         | `str`                          | Identificador de sesión                                                                                                                                                                                                                                                                         |
| `message`            | `Any`                          | Contenido del mensaje sin procesar                                                                                                                                                                                                                                                              |
| `parent_tool_use_id` | `str \| None`                  | Para mensajes de subagente, el id del bloque de uso de herramienta `Agent` que lo generó. `None` para mensajes de sesión principal y sesiones más antiguas                                                                                                                                      |
| `parent_agent_id`    | `str \| None`                  | Para mensajes de un [subagente anidado](/docs/es/sub-agents#let-subagents-spawn-their-own-subagents), el id del agente del subagente padre. `None` para mensajes de sesión principal, mensajes de subagente de nivel superior y sesiones más antiguas. Requiere Python Agent SDK 0.2.140 o posterior |

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
  Clases
</h2>

<h3 id="claudesdkclient">
  `ClaudeSDKClient`
</h3>

**Mantiene una sesión de conversación en múltiples intercambios.** Este es el equivalente de Python de cómo funciona internamente la función `query()` del SDK de TypeScript - crea un objeto cliente que puede continuar conversaciones. Consulte la [comparación con `query()`](#choosing-between-query-and-claudesdkclient).

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
| `set_model(model)`                        | Cambiar el modelo para la sesión actual. Pase `None` para restablecer al [modelo predeterminado de Claude Code](/docs/es/model-config)                                           |
| `rewind_files(user_message_id)`           | Restaurar archivos a su estado en el mensaje de usuario especificado. Requiere `enable_file_checkpointing=True`. Ver [File checkpointing](/docs/es/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Obtener el estado de todos los servidores MCP configurados. Devuelve [`McpStatusResponse`](#mcpstatusresponse)                                                              |
| `reconnect_mcp_server(server_name)`       | Reintentar conectar a un servidor MCP que falló o fue desconectado                                                                                                          |
| `toggle_mcp_server(server_name, enabled)` | Habilitar o deshabilitar un servidor MCP a mitad de sesión. Deshabilitar elimina sus herramientas                                                                           |
| `stop_task(task_id)`                      | Detener una tarea de fondo en ejecución. Un [`TaskNotificationMessage`](#tasknotificationmessage) con estado `"stopped"` sigue en el flujo de mensajes                      |
| `get_server_info()`                       | Obtener la información de inicialización del servidor, incluyendo comandos disponibles y estilos de salida                                                                  |
| `disconnect()`                            | Desconectar de Claude                                                                                                                                                       |

<h4 id="context-manager-support">
  Soporte de gestor de contexto
</h4>

El cliente se puede usar como un gestor de contexto asincrónico para la gestión automática de conexiones:

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
  **Comportamiento del búfer después de la interrupción:** `interrupt()` envía una señal de parada pero no borra el búfer de mensajes. Los mensajes ya producidos por la tarea interrumpida, incluyendo su `ResultMessage`, permanecen en el flujo. Debe drenarlos con `receive_response()` antes de leer la respuesta a una nueva consulta. Si envía una nueva consulta inmediatamente después de `interrupt()` y llama a `receive_response()` solo una vez, recibirá los mensajes de la tarea interrumpida, no la respuesta de la nueva consulta.
</Note>

<h4 id="example-advanced-permission-control">
  Ejemplo - Control de permisos avanzado
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
  Tipos
</h2>

<Note>
  **`@dataclass` vs `TypedDict`:** Este SDK utiliza dos tipos de clases. Las clases decoradas con `@dataclass` (como `ResultMessage`, `AgentDefinition`, `TextBlock`) son instancias de objetos en tiempo de ejecución y admiten acceso por atributo: `msg.result`. Las clases definidas con `TypedDict` (como `ThinkingConfigEnabled`, `McpStdioServerConfig`, `SyncHookJSONOutput`) son **dicts simples en tiempo de ejecución** y requieren acceso por clave: `config["budget_tokens"]`, no `config.budget_tokens`. La sintaxis de llamada `ClassName(field=value)` funciona para ambas, pero solo las dataclasses producen objetos con atributos.
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

| Propiedad      | Tipo                                            | Descripción                                                                                                                  |
| :------------- | :---------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Identificador único para la herramienta                                                                                      |
| `description`  | `str`                                           | Descripción legible por humanos                                                                                              |
| `input_schema` | `type[T] \| dict[str, Any]`                     | Esquema para validación de entrada                                                                                           |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]`      | Función asincrónica que maneja la ejecución de la herramienta                                                                |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Anotaciones de herramienta opcionales (por ejemplo `readOnlyHint`, `destructiveHint`, `openWorldHint`, `maxResultSizeChars`) |

<h3 id="transport">
  `Transport`
</h3>

Clase base abstracta para implementaciones de transporte personalizadas. Utilícela para comunicarse con el proceso Claude a través de un canal personalizado (por ejemplo, una conexión remota en lugar de un subproceso local).

<Warning>
  Esta es una API interna de bajo nivel. La interfaz puede cambiar en futuras versiones. Las implementaciones personalizadas deben actualizarse para coincidir con cualquier cambio de interfaz.
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

Importación: `from claude_agent_sdk import Transport`

<h3 id="claudeagentoptions">
  `ClaudeAgentOptions`
</h3>

Dataclass de configuración para consultas de Claude Code.

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

| Propiedad                     | Tipo                                                                                  | Predeterminado                                             | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :---------------------------- | :------------------------------------------------------------------------------------ | :--------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                                                     | Configuración de herramientas. Utilice `{"type": "preset", "preset": "claude_code"}` para las herramientas predeterminadas de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                                                       | Herramientas para aprobar automáticamente sin solicitar. Esto no restringe Claude solo a estas herramientas. Si nombra una de las [herramientas de seguimiento de tareas](/docs/es/agent-sdk/todo-tracking#model-availability) aquí, Claude Code también opta por la sesión. Otras herramientas no listadas se transfieren a `permission_mode` y `can_use_tool`. Utilice `disallowed_tools` para bloquear herramientas. Consulte [Permisos](/docs/es/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                          |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptCustom \| SystemPromptFile \| None`         | `None`                                                     | Configuración de indicación del sistema. Pase una cadena para un indicador personalizado, `{"type": "preset", "preset": "claude_code"}` para el indicador del sistema de Claude Code con `"append"` opcional, `{"type": "custom", "prompt": "..."}` para un indicador personalizado que también puede establecer `"snapshot"`, o `{"type": "file", "path": "..."}` para cargar un indicador grande desde el disco. Consulte [`SystemPromptPreset`](#systempromptpreset), [`SystemPromptCustom`](#systempromptcustom), y [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                                 |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                                                       | Configuraciones de servidor MCP o ruta al archivo de configuración                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `strict_mcp_config`           | `bool`                                                                                | `False`                                                    | Cuando es `True`, utilice solo los servidores pasados en `mcp_servers` e ignore el proyecto `.mcp.json`, la configuración del usuario, los servidores MCP proporcionados por plugins, y los [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai). Se asigna a la bandera CLI `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                                                     | Modo de permiso para el uso de herramientas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `continue_conversation`       | `bool`                                                                                | `False`                                                    | Continuar la conversación más reciente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `resume`                      | `str \| None`                                                                         | `None`                                                     | ID de sesión para reanudar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `session_id`                  | `str \| None`                                                                         | `None`                                                     | Utilice un ID de sesión específico en lugar de uno generado automáticamente. Debe ser un UUID válido. No se puede combinar con `continue_conversation` o `resume` a menos que `fork_session` también esté configurado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `max_turns`                   | `int \| None`                                                                         | `None`                                                     | Máximo de turnos agentes (viajes de ronda de uso de herramientas)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                                                     | Detener la consulta cuando la estimación de costo del lado del cliente alcance este valor en USD. Se compara con la misma estimación que `total_cost_usd`. Para advertencias de precisión y comportamiento de reinicio, consulte [Rastrear costo y uso](/docs/es/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                                                       | Herramientas a denegar. Un nombre simple como `"Bash"` elimina la herramienta del contexto de Claude. Una regla con alcance como `"Bash(rm *)"` deja la herramienta disponible y deniega llamadas coincidentes en cada modo de permiso, incluido `bypassPermissions`, para el comando [tal como está escrito](/docs/es/permissions#bash-rule-limits). Consulte [Permisos](/docs/es/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                            |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                                                    | Habilitar el seguimiento de cambios de archivo para rebobinar. Consulte [Punto de control de archivo](/docs/es/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `model`                       | `str \| None`                                                                         | `None`                                                     | Alias de modelo Claude o nombre de modelo completo. Consulte [valores aceptados e IDs específicos del proveedor](/docs/es/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `fallback_model`              | `str \| None`                                                                         | `None`                                                     | Modelo de respaldo a utilizar si el modelo principal falla. Acepta una lista separada por comas. Para orientación, consulte [Elegir un modelo](/docs/es/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                                                       | Características beta para habilitar. Consulte [`SdkBeta`](#sdkbeta) para opciones disponibles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                                                     | Formato de salida para respuestas estructuradas (por ejemplo, `{"type": "json_schema", "schema": {...}}`). Consulte [Salidas estructuradas](/docs/es/agent-sdk/structured-outputs) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                                                     | Nombre de herramienta MCP para indicadores de permiso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                                                     | Directorio de trabajo actual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                                                     | Ruta personalizada al ejecutable CLI de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `settings`                    | `str \| None`                                                                         | `None`                                                     | Ruta a un archivo de configuración o una cadena JSON en línea                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                                                       | Directorios adicionales a los que Claude puede acceder. El SDK pasa cada entrada a Claude Code como `--add-dir`, por lo que con la fuente de configuración `project` Claude Code también [carga las skills, comandos y subagentes del directorio](/docs/es/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `env`                         | `dict[str, str]`                                                                      | `{}`                                                       | Variables de entorno fusionadas en la parte superior del entorno de proceso heredado. Consulte [Variables de entorno](/docs/es/env-vars) para variables que lee la CLI subyacente, y [Manejar respuestas API lentas o estancadas](#handle-slow-or-stalled-api-responses) para variables relacionadas con tiempos de espera. Establezca `CLAUDE_AGENT_SDK_CLIENT_APP` para identificar su aplicación en el encabezado User-Agent                                                                                                                                                                                                                                                                                                                                                                      |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                                                       | Argumentos CLI adicionales para pasar directamente a la CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                                                     | Máximo de bytes al almacenar en búfer la salida estándar de CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                                               | *Obsoleto* - El SDK ignora este valor. Utilice la devolución de llamada `stderr` para salida de stderr de CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                                                     | Función de devolución de llamada para salida de stderr desde CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                                                     | Devolución de llamada de permiso de herramienta, invocada solo cuando el [flujo de permiso](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) se transfiere a un indicador. No se invoca para llamadas aprobadas automáticamente por `allowed_tools`, reglas de permiso, o `permission_mode`. Una regla de permiso no aprueba previamente las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves). Consulte [`CanUseTool`](#canusetool) para detalles                                                                                                                                                                                                                                                                                 |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                                                     | Configuraciones de hooks para interceptar eventos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `user`                        | `str \| None`                                                                         | `None`                                                     | En plataformas POSIX, la cuenta de usuario del SO bajo la cual se ejecuta el subproceso de Claude Code. Claude Code mantiene el entorno del proceso padre, incluido `HOME`, y se ejecuta en `cwd`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `include_partial_messages`    | `bool`                                                                                | `False`                                                    | Incluir eventos de transmisión de mensajes parciales. Cuando está habilitado, se producen mensajes [`StreamEvent`](#streamevent)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `include_hook_events`         | `bool`                                                                                | `False`                                                    | Incluir eventos del ciclo de vida de hooks en el flujo de mensajes como objetos `HookEventMessage`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `forward_subagent_text`       | `bool`                                                                                | `False`                                                    | Reenviar bloques de texto y pensamiento de subagentes en el flujo de mensajes. Sin esta opción, Claude Code emite bloques `tool_use` y `tool_result` de subagentes pero no texto ni pensamiento. Requiere Python Agent SDK 0.2.140 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `fork_session`                | `bool`                                                                                | `False`                                                    | Al reanudar con `resume`, bifurcar a un nuevo ID de sesión en lugar de continuar la sesión original                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `resume_session_at`           | `str \| None`                                                                         | `None`                                                     | Al reanudar, cargar la conversación solo hasta e incluyendo el mensaje con este UUID. Utilice con `resume`, y generalmente `fork_session`, para ramificar desde un punto anterior. Requiere Python Agent SDK 0.2.137 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `resume_drops_turn`           | `str \| None`                                                                         | `None`                                                     | UUID del indicador del usuario cuyo turno descarta un truncamiento `resume_session_at`. Cuando se establece, la CLI rechaza la reanudación si el rango descartado contiene entradas no atribuibles a ese turno. Requiere Python Agent SDK 0.2.137 o posterior y Claude Code v2.1.223 o posterior; la CLI incluida en esas versiones de SDK satisface el requisito de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                                                     | Subagentes definidos programáticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                                                       | Cargar plugins personalizados desde rutas locales. Consulte [Plugins](/docs/es/agent-sdk/plugins) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                                                     | Configurar el comportamiento del sandbox programáticamente. Consulte [Configuración de sandbox](#sandboxsettings) para detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (Valores predeterminados de CLI: todas las fuentes) | Controlar qué configuración del sistema de archivos cargar. Pase `[]` para deshabilitar la configuración de usuario, proyecto y local. Con `skills` establecido y este campo sin establecer, solo se cargan las fuentes de usuario y proyecto. Establezca `setting_sources` explícitamente para mantener la configuración local. La política administrada por punto final se carga independientemente; la configuración administrada por servidor se obtiene cuando la sesión se autentica con una credencial de organización en una [configuración elegible](/docs/es/server-managed-settings#platform-availability). Para entradas leídas independientemente de esta opción, consulte [Lo que settingSources no controla](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control) |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                                                     | Skills disponibles para la sesión. Pase `"all"` para habilitar cada skill descubierto, o una lista de nombres de skills. Pase solo nombres exactos. El SDK rechaza nombres mal formados y en forma de comodín con un `ValueError` antes de iniciar el proceso de Claude Code; esta verificación requiere Python Agent SDK 0.2.129 o posterior. Cuando se establece, el SDK agrega automáticamente la herramienta Skill a `allowed_tools`. Si también pasa `tools`, incluya `"Skill"` en esa lista. Consulte [Skills](/docs/es/agent-sdk/skills)                                                                                                                                                                                                                                                      |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                                                     | *Obsoleto* - Máximo de tokens para bloques de pensamiento. Utilice `thinking` en su lugar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                                                     | Controla el comportamiento del pensamiento extendido. Tiene precedencia sobre `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                                                     | Nivel de esfuerzo para la profundidad del pensamiento. Consulte [ajustar el nivel de esfuerzo](/docs/es/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `session_store`               | [`SessionStore`](/docs/es/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                                                     | Reflejar transcripciones de sesión en un backend externo para que otro host pueda reanudarlas. Consulte [Persistir sesiones en almacenamiento externo](/docs/es/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                                                | Cuándo vaciar entradas de transcripción reflejadas a `session_store`. `"batched"` vacía una vez por turno o cuando el búfer se llena; `"eager"` activa un vaciado en segundo plano después de cada fotograma. Se ignora cuando `session_store` es `None`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `load_timeout_ms`             | `int`                                                                                 | `60000`                                                    | Tiempo de espera por llamada para `session_store.load()` y `list_subkeys()` durante la materialización de reanudación, en milisegundos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `task_budget`                 | `TaskBudget \| None`                                                                  | `None`                                                     | Presupuesto de tokens del lado de la API. Se envía como `output_config.task_budget` con el encabezado beta `task-budgets-2026-03-13`. Pase `{"total": <int>}`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h4 id="handle-slow-or-stalled-api-responses">
  Manejar respuestas API lentas o estancadas
</h4>

El subproceso CLI lee varias variables de entorno que controlan los tiempos de espera de API y la detección de estancamiento. Páselas a través de `ClaudeAgentOptions.env`:

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

* `API_TIMEOUT_MS`: tiempo de espera por solicitud en el cliente de Anthropic, en milisegundos. Predeterminado `600000`. Se aplica al bucle principal y a todos los subagentes.
* `CLAUDE_CODE_MAX_RETRIES`: máximo de reintentos de API. Predeterminado `10`, limitado a `15`. Cada reintento obtiene su propia ventana `API_TIMEOUT_MS`, por lo que el tiempo de pared en el peor caso es aproximadamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` más backoff. Para ejecuciones desatendidas que necesitan esperar a través de interrupciones más largas, establezca [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/es/errors#tune-retry-behavior): reintenta errores de capacidad transitoria indefinidamente y, en Claude Code v2.1.199 o posterior, eleva el predeterminado para otros errores transitorios a `300` y elimina el límite en esta variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: perro guardián de estancamiento para subagentes. Mientras el perro guardián de flujo está activado, el predeterminado es `CLAUDE_STREAM_IDLE_TIMEOUT_MS` más 5 minutos, lo que suma `600000` a menos que eleve esa variable. Con el perro guardián de flujo desactivado, el predeterminado es `600000`. Antes de v2.1.257, el predeterminado era siempre `600000`.

  El temporizador se reinicia en cada evento de flujo. En un estancamiento, Claude Code aborta el subagente e informa el estancamiento al padre. Para un subagente en segundo plano, también marca la tarea como fallida y adjunta cualquier resultado parcial.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` con `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: perro guardián de flujo que aborta la solicitud cuando los encabezados han llegado pero el cuerpo de respuesta deja de transmitirse. El perro guardián está activado de forma predeterminada para todos los proveedores; establezca `CLAUDE_ENABLE_STREAM_WATCHDOG=0` para desactivarlo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` tiene un predeterminado de `300000` y se fija a ese mínimo. Después del aborto, [Reintentos automáticos](/docs/es/errors#automatic-retries) cubre lo que Claude Code hace, según qué tan lejos haya progresado la respuesta.

  Mientras el perro guardián espera una respuesta que una puerta de enlace detrás de `ANTHROPIC_BASE_URL` mantiene abierta con pings de keep-alive, un host que establece `include_partial_messages` sigue recibiendo mensajes [`StreamEvent`](#streamevent) de `ping`. Lea esos fotogramas como vivacidad en lugar de agotar el tiempo de espera de la sesión en silencio. Antes de v2.1.257, los fotogramas se detenían 5 minutos después del último evento de flujo real.

<h3 id="outputformat">
  `OutputFormat`
</h3>

Configuración para validación de salida estructurada. Pase esto como un `dict` al campo `output_format` en `ClaudeAgentOptions`:

```python theme={null}
# Forma de dict esperada para output_format
{
    "type": "json_schema",
    "schema": {...},  # Su definición de JSON Schema
}
```

| Campo    | Requerido | Descripción                                             |
| :------- | :-------- | :------------------------------------------------------ |
| `type`   | Sí        | Debe ser `"json_schema"` para validación de JSON Schema |
| `schema` | Sí        | Definición de JSON Schema para validación de salida     |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

Configuración para usar el indicador del sistema preestablecido de Claude Code con adiciones opcionales.

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
    snapshot: NotRequired[bool]
```

| Campo                      | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | Sí        | Debe ser `"preset"` para usar un indicador del sistema preestablecido                                                                                                                                                                                                                                                                                                                        |
| `preset`                   | Sí        | Debe ser `"claude_code"` para usar el indicador del sistema de Claude Code                                                                                                                                                                                                                                                                                                                   |
| `append`                   | No        | Instrucciones adicionales para agregar al indicador del sistema preestablecido                                                                                                                                                                                                                                                                                                               |
| `exclude_dynamic_sections` | No        | Mover contexto por sesión como directorio de trabajo, la bandera de repositorio git, y rutas de memoria automática del indicador del sistema al primer mensaje del usuario. Mejora la reutilización de caché de indicadores entre usuarios y máquinas. Consulte [Modificar indicadores del sistema](/docs/es/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |
| `snapshot`                 | No        | Establezca en `False` para reconstruir el indicador del sistema en cada solicitud en lugar de [reutilizar el indicador que la sesión registró en su primera solicitud](/docs/es/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Requiere `claude-agent-sdk` v0.2.153 o posterior                                                                                    |

<h3 id="systempromptcustom">
  `SystemPromptCustom`
</h3>

Un indicador del sistema personalizado en forma de objeto, equivalente a pasar una cadena como `system_prompt`, que también puede establecer `snapshot`. Requiere `claude-agent-sdk` v0.2.153 o posterior.

```python theme={null}
class SystemPromptCustom(TypedDict):
    type: Literal["custom"]
    prompt: str
    snapshot: NotRequired[bool]
```

| Campo      | Requerido | Descripción                                                                                                                                                                          |
| :--------- | :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`     | Sí        | Debe ser `"custom"`                                                                                                                                                                  |
| `prompt`   | Sí        | El texto del indicador del sistema. Se pasa a la CLI como un argumento de línea de comandos, por lo que se aplican los [límites de longitud de línea de comandos](#systempromptfile) |
| `snapshot` | No        | Igual que [`SystemPromptPreset.snapshot`](#systempromptpreset), aplicado a `prompt`                                                                                                  |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

Configuración para cargar un indicador del sistema personalizado desde un archivo en lugar de pasarlo como una cadena. El SDK asigna esto a la bandera CLI [`--system-prompt-file`](/docs/es/cli-reference#system-prompt-flags). Utilice la forma de archivo cuando el indicador es grande: el SDK pasa un `system_prompt` de cadena en el argv del subproceso CLI, que está sujeto a límites de longitud de línea de comandos del SO antes de que el SDK envíe cualquier solicitud de API. En Linux, un único argumento más largo que aproximadamente 128 KB falla al generar el proceso con `Argument list too long`. En Windows, toda la línea de comandos está limitada a aproximadamente 32 KB, por lo que la forma de cadena falla en un umbral más bajo.

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| Campo  | Requerido | Descripción                                               |
| :----- | :-------- | :-------------------------------------------------------- |
| `type` | Sí        | Debe ser `"file"` para cargar el indicador desde el disco |
| `path` | Sí        | Ruta a un archivo que contiene el indicador del sistema   |

<h3 id="settingsource">
  `SettingSource`
</h3>

Controla qué fuentes de configuración basadas en el sistema de archivos carga el SDK.

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| Valor       | Descripción                                                                                           | Ubicación                     |
| :---------- | :---------------------------------------------------------------------------------------------------- | :---------------------------- |
| `"user"`    | Configuración global del usuario                                                                      | `~/.claude/settings.json`     |
| `"project"` | Configuración del proyecto compartido (controlada por versión)                                        | `.claude/settings.json`       |
| `"local"`   | Configuración del proyecto local, ignorada en git cuando Claude Code guarda una configuración en ella | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamiento predeterminado
</h4>

Cuando `setting_sources` se omite o es `None` y `skills` no está establecido, `query()` carga la misma configuración del sistema de archivos que la CLI de Claude Code: usuario, proyecto y local. Con `skills` establecido, la fila [`setting_sources`](#claudeagentoptions) describe el predeterminado actual. La política administrada por punto final se carga en todos los casos; la configuración administrada por servidor se obtiene cuando la sesión se autentica con una credencial de organización en una [configuración elegible](/docs/es/server-managed-settings#platform-availability). Para más información, consulte [Lo que settingSources no controla](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control).

<h4 id="why-use-setting_sources">
  Por qué usar setting\_sources
</h4>

**Deshabilitar configuración del sistema de archivos:**

```python theme={null}
# No cargar configuración de usuario, proyecto o local desde el disco
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
  En Python SDK 0.1.59 y anterior, una lista vacía se trataba igual que omitir la opción, por lo que `setting_sources=[]` no deshabilitaba la configuración del sistema de archivos. Actualice a una versión más reciente si necesita que una lista vacía tenga efecto. El SDK de TypeScript no se ve afectado.
</Note>

**Cargar solo fuentes de configuración específicas:**

```python theme={null}
# Cargar solo configuración del proyecto, ignorar usuario y local
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    async for message in query(
        prompt="Run CI checks",
        options=ClaudeAgentOptions(
            setting_sources=["project"]  # Solo .claude/settings.json
        ),
    ):
        print(message)


asyncio.run(main())
```

**Aplicaciones solo SDK:**

```python theme={null}
# Definir todo programáticamente.
# Pase [] para optar por no usar fuentes de configuración del sistema de archivos.
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

Para cargar instrucciones del proyecto CLAUDE.md, incluya `"project"` en `setting_sources`. Consulte [Modificar indicadores del sistema](/docs/es/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions) para ver cómo la carga de CLAUDE.md interactúa con las opciones de indicador del sistema.

<h4 id="settings-precedence">
  Precedencia de configuración
</h4>

Cuando se cargan múltiples fuentes, la configuración se fusiona con esta precedencia (mayor a menor):

1. Configuración local (`.claude/settings.local.json`)
2. Configuración del proyecto (`.claude/settings.json`)
3. Configuración del usuario (`~/.claude/settings.json`)

Las opciones programáticas como `agents`, `allowed_tools` y `settings` anulan la configuración del sistema de archivos de usuario, proyecto y local. La configuración de política administrada tiene precedencia sobre las opciones programáticas.

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
| `prompt`          | Sí        | El indicador del sistema del agente                                                                                                                                                                                                                                                 |
| `tools`           | No        | Matriz de nombres de herramientas permitidas. Si se omite, hereda cada [herramienta disponible para subagentes](/docs/es/sub-agents#available-tools)                                                                                                                                     |
| `disallowedTools` | No        | Matriz de nombres de herramientas a eliminar del conjunto de herramientas del agente. También se aceptan patrones a nivel de servidor MCP: `mcp__server` o `mcp__server__*` elimina cada herramienta de ese servidor, y `mcp__*` elimina cada herramienta MCP de cualquier servidor |
| `model`           | No        | Anulación de modelo para este agente. Acepta un alias como `"sonnet"`, `"opus"`, `"haiku"`, o `"inherit"`, o un ID de modelo completo. Cuando lo omite, Claude Code elige el modelo en el [orden de modelo de subagente](/docs/es/sub-agents#choose-a-model)                             |
| `skills`          | No        | Lista de nombres de skills para precargar en el contexto del agente al inicio. Las skills no listadas siguen siendo invocables a través de la herramienta Skill                                                                                                                     |
| `memory`          | No        | Fuente de memoria para este agente: `"user"`, `"project"`, o `"local"`                                                                                                                                                                                                              |
| `mcpServers`      | No        | Servidores MCP disponibles para este agente. Cada entrada es un nombre de servidor o un dict `{name: config}` en línea                                                                                                                                                              |
| `initialPrompt`   | No        | Se envía automáticamente como el primer turno del usuario cuando este agente se ejecuta como el agente del hilo principal                                                                                                                                                           |
| `maxTurns`        | No        | Número máximo de turnos agentes antes de que el agente se detenga                                                                                                                                                                                                                   |
| `background`      | No        | Ejecutar este agente como una tarea de fondo no bloqueante cuando se invoca                                                                                                                                                                                                         |
| `effort`          | No        | Nivel de esfuerzo de razonamiento para este agente. Acepta un nivel nombrado o un entero. Consulte [`EffortLevel`](#effortlevel)                                                                                                                                                    |
| `permissionMode`  | No        | Modo de permiso para la ejecución de herramientas dentro de este agente. Las [reglas de herencia de subagentes](/docs/es/agent-sdk/permissions#available-modes) deciden cuándo se aplica. Consulte [`PermissionMode`](#permissionmode)                                                   |

<Note>
  Los nombres de campo `AgentDefinition` utilizan camelCase, como `disallowedTools`, `permissionMode` y `maxTurns`. Estos nombres se asignan directamente al formato de cable compartido con el SDK de TypeScript. Esto difiere de `ClaudeAgentOptions`, que utiliza snake\_case de Python para los campos de nivel superior equivalentes como `disallowed_tools` y `permission_mode`. Debido a que `AgentDefinition` es una dataclass, pasar una palabra clave snake\_case genera un `TypeError` en el tiempo de construcción.
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

Modos de permiso para controlar la ejecución de herramientas.

```python theme={null}
PermissionMode = Literal[
    "default",  # Comportamiento de permiso estándar
    "acceptEdits",  # Aceptar automáticamente ediciones de archivo
    "plan",  # Modo de planificación - explorar sin editar
    "dontAsk",  # Denegar cualquier cosa no preaprobada en lugar de solicitar
    "bypassPermissions",  # Omitir verificaciones de permiso; las reglas de solicitud explícita aún solicitan (usar con cuidado)
    "auto",  # El clasificador del modelo aprueba o deniega indicadores de permiso
]
```

<h3 id="effortlevel">
  `EffortLevel`
</h3>

Niveles de esfuerzo para guiar la profundidad del pensamiento.

```python theme={null}
EffortLevel = Literal[
    "low",  # Pensamiento mínimo, respuestas más rápidas
    "medium",  # Pensamiento moderado
    "high",  # Razonamiento profundo
    "xhigh",  # Razonamiento extendido; vuelve a "high" en modelos que no lo admiten
    "max",  # Esfuerzo máximo
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

La devolución de llamada es el reemplazo del SDK para el indicador de permiso interactivo: se invoca solo cuando el [flujo de evaluación de permiso](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) se resuelve en un indicador. Las llamadas de herramienta ya aprobadas por una entrada `allowed_tools`, una regla de permiso de configuración, o el modo de permiso, como `acceptEdits` o `bypassPermissions`, nunca la invocan. Para controlar cada llamada de herramienta, utilice un [hook `PreToolUse`](/docs/es/agent-sdk/hooks) en su lugar.

Una regla de permiso no aprueba previamente las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves); consulte [Cómo se evalúan los permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) para ver cuál de ellas llega a la devolución de llamada y qué sucede en modo `dontAsk` y `auto`.

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Información de contexto pasada a devoluciones de llamada de permiso de herramienta.

```python theme={null}
@dataclass
class ToolPermissionContext:
    signal: Any | None = None  # Futuro: soporte de señal de aborto
    suggestions: list[PermissionUpdate] = field(default_factory=list)
    tool_use_id: str | None = None
    agent_id: str | None = None
    blocked_path: str | None = None
    decision_reason: str | None = None
    title: str | None = None
    display_name: str | None = None
    description: str | None = None
```

| Campo             | Tipo                     | Descripción                                                                                                                                                                                                                                                    |
| :---------------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`          | `Any \| None`            | Reservado para soporte futuro de señal de aborto                                                                                                                                                                                                               |
| `suggestions`     | `list[PermissionUpdate]` | Sugerencias de actualización de permiso de la CLI. Los indicadores de Bash incluyen una sugerencia con el destino `localSettings`, por lo que devolverla en `updated_permissions` escribe la regla en `.claude/settings.local.json` y persiste entre sesiones. |
| `tool_use_id`     | `str \| None`            | Identificador de la llamada de herramienta específica para la que es este indicador. Siempre se completa cuando se entrega a `can_use_tool`                                                                                                                    |
| `agent_id`        | `str \| None`            | ID del subagente cuando la llamada se origina desde un subagente; `None` para el agente principal                                                                                                                                                              |
| `blocked_path`    | `str \| None`            | Ruta de archivo que activó la solicitud de permiso, cuando sea aplicable. Por ejemplo, cuando un comando Bash intenta acceder a una ruta fuera de directorios permitidos                                                                                       |
| `decision_reason` | `str \| None`            | Razón por la que se activó esta solicitud de permiso. Reenviado desde el `permissionDecisionReason` de un hook PreToolUse cuando el hook devolvió `"ask"`                                                                                                      |
| `title`           | `str \| None`            | Oración de indicador de permiso completo, como `Claude wants to read foo.txt`. Utilice como texto de indicador principal cuando esté presente                                                                                                                  |
| `display_name`    | `str \| None`            | Frase de sustantivo corto para la acción de herramienta, como `Read file`, adecuada para etiquetas de botón                                                                                                                                                    |
| `description`     | `str \| None`            | Subtítulo legible por humanos para la interfaz de usuario de permiso                                                                                                                                                                                           |

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

Configuración de herramientas preestablecidas para usar el conjunto de herramientas predeterminado de Claude Code.

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Controla el comportamiento del pensamiento extendido. Una unión de tres configuraciones:

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

El campo `display` opcional controla si el texto de pensamiento se devuelve `"summarized"` u `"omitted"`. En Claude Opus 4.7 y posterior, el predeterminado de API es `"omitted"`, por lo que establezca `"summarized"` para recibir contenido de pensamiento en salidas [`ThinkingBlock`](#thinkingblock). Claude Code no envía `display` a Amazon Bedrock ni a la Plataforma de Agentes de Google Cloud, por lo que en esos proveedores Opus 4.7 y posterior devuelven salidas `ThinkingBlock` vacías incluso cuando establece `display` en `"summarized"`.

Debido a que estas son clases `TypedDict`, son dicts simples en tiempo de ejecución. Construyalas como literales de dict o llame a la clase como un constructor; ambos producen un `dict`. Acceda a los campos con `config["budget_tokens"]`, no `config.budget_tokens`:

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions, ThinkingConfigEnabled

# Opción 1: literal de dict (recomendado, no se necesita importación)
options = ClaudeAgentOptions(thinking={"type": "enabled", "budget_tokens": 20000})

# Opción 2: estilo constructor (devuelve un dict simple)
config = ThinkingConfigEnabled(type="enabled", budget_tokens=20000)
print(config["budget_tokens"])  # 20000
# config.budget_tokens generaría AttributeError
```

<h3 id="taskbudget">
  `TaskBudget`
</h3>

Presupuesto de tarea del lado de la API en tokens, utilizado con el campo `task_budget` en `ClaudeAgentOptions`.

```python theme={null}
class TaskBudget(TypedDict):
    total: int
```

| Campo   | Tipo  | Descripción                              |
| :------ | :---- | :--------------------------------------- |
| `total` | `int` | Presupuesto de token total para la tarea |

Debido a que esto es un `TypedDict`, páselo como un dict simple, como `ClaudeAgentOptions(task_budget={"total": 50000})`.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Tipo literal para características beta del SDK.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Utilice con el campo `betas` en `ClaudeAgentOptions` para habilitar características beta.

<Warning>
  La beta `context-1m-2025-08-07` se retiró a partir del 30 de abril de 2026. Pasar este encabezado con Claude Sonnet 4.5 o Sonnet 4 no tiene efecto, y las solicitudes que exceden la ventana de contexto estándar de 200k tokens devuelven un error. Para usar una ventana de contexto de 1M tokens, migre a [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, o Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), que incluyen contexto de 1M a precios estándar sin encabezado beta requerido.
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

Configuración para servidores MCP del SDK creados con `create_sdk_mcp_server()`.

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # Instancia del servidor MCP
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
    type: NotRequired[Literal["stdio"]]  # Opcional para compatibilidad hacia atrás
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

La configuración de un servidor MCP tal como se informa mediante [`get_mcp_status()`](#methods). Esta es la unión de todas las variantes de transporte [`McpServerConfig`](#mcpserverconfig) más una variante de solo salida `claudeai-proxy` para servidores proxificados a través de claude.ai.

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

Para información completa sobre cómo crear y usar plugins, consulte [Plugins](/docs/es/agent-sdk/plugins).

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
    | ConversationResetMessage
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
    origin: MessageOrigin | None = None
```

| Campo                | Tipo                        | Descripción                                                                                                                                                                                       |
| :------------------- | :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `content`            | `str \| list[ContentBlock]` | Contenido del mensaje como texto o bloques de contenido                                                                                                                                           |
| `uuid`               | `str \| None`               | Identificador único del mensaje                                                                                                                                                                   |
| `parent_tool_use_id` | `str \| None`               | ID de uso de herramienta si este mensaje es una respuesta de resultado de herramienta                                                                                                             |
| `tool_use_result`    | `dict[str, Any] \| None`    | Datos de resultado de herramienta si es aplicable                                                                                                                                                 |
| `origin`             | `MessageOrigin \| None`     | Procedencia de este mensaje, rellenado en turnos inyectados como notificaciones de tareas y mensajes de pares. `None` cuando la CLI no lo atribuyó. Requiere Python Agent SDK 0.2.137 o posterior |

El SDK pasa `tool_use_result` a través de la CLI sin modificar. Para una herramienta en un servidor MCP externo cuyo resultado contiene bloques `resource_link`, el dict tiene una clave `resourceLinks` que contiene una lista de dicts con las claves del tipo TypeScript [`SDKMcpResourceLink`](/docs/es/agent-sdk/typescript#sdkmcpresourcelink). Claude recibe cada enlace como una línea de texto en el resultado de la herramienta. Para renderizar los archivos que devolvió el servidor, lea `resourceLinks` en lugar de analizar ese texto. La clave `resourceLinks` requiere Python Agent SDK 0.2.150 o posterior y Claude Code v2.1.257 o posterior; la CLI incluida con esa versión del SDK satisface el requisito de Claude Code.

La CLI omite la clave cuando el resultado no tiene enlaces y en resultados de subagentes. La CLI mantiene como máximo 50 enlaces por resultado y deja de agregar enlaces una vez que la lista alcanza 64 KiB de JSON serializado. Una herramienta que define en proceso con [`tool()`](#tool) nunca produce la clave, porque el SDK aplana sus bloques `resource_link` a texto antes de que la CLI vea el resultado.

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
    stop_reason: str | None = None
    session_id: str | None = None
    uuid: str | None = None
```

| Campo                | Tipo                                                         | Descripción                                                                          |
| :------------------- | :----------------------------------------------------------- | :----------------------------------------------------------------------------------- |
| `content`            | `list[ContentBlock]`                                         | Lista de bloques de contenido en la respuesta                                        |
| `model`              | `str`                                                        | Modelo que generó la respuesta                                                       |
| `parent_tool_use_id` | `str \| None`                                                | ID de uso de herramienta si esta es una respuesta anidada                            |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Tipo de error si la respuesta encontró un error                                      |
| `usage`              | `dict[str, Any] \| None`                                     | Uso de token por mensaje (mismas claves que [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | ID de mensaje de API. Múltiples mensajes de un turno comparten el mismo ID           |
| `stop_reason`        | `str \| None`                                                | Razón de parada de la API (por ejemplo, `end_turn`, `tool_use`)                      |
| `session_id`         | `str \| None`                                                | ID de la sesión a la que pertenece este mensaje                                      |
| `uuid`               | `str \| None`                                                | Identificador único del mensaje dentro de la transcripción de sesión                 |

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
    "unknown",
]
```

El proceso CLI subyacente puede emitir tipos de error que este Literal no enumera, como `max_output_tokens`. El SDK pasa el valor sin modificar, así que trate las cadenas fuera de esta lista de la manera que trata `unknown`. El tipo TypeScript [`SDKAssistantMessageError`](/docs/es/agent-sdk/typescript#sdkassistantmessage) enumera el conjunto completo de valores que la CLI puede emitir.

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
    model_usage: dict[str, ModelUsage] | None = None
    permission_denials: list[Any] | None = None
    deferred_tool_use: DeferredToolUse | None = None
    errors: list[str] | None = None
    api_error_status: int | None = None
    uuid: str | None = None
    terminal_reason: str | None = None
    origin: MessageOrigin | None = None
```

El campo `subtype` determina cuáles otros campos se rellenan. Es uno de `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"`, o `"error_max_structured_output_retries"`. La clase de datos de Python aplana todas las variantes en una forma, por lo que los campos que no se aplican al subtipo devuelto son `None`.

Varios campos llevan detalle de diagnóstico sobre cómo terminó la conversación:

* `is_error`: `True` cuando la conversación terminó en un estado de error. Siempre `True` en los subtipos `error_*`. En `subtype="success"` es `True` cuando la solicitud del modelo final falló, lo que significa que el bucle del agente se completó pero la última llamada a la API devolvió un error.
* `api_error_status`: el código de estado HTTP del error de API de terminación. `None` cuando el turno terminó sin uno. Se rellena solo en `subtype="success"`.
* `result`: texto del mensaje del asistente final en `subtype="success"`, o `None` en los subtipos `error_*`. Cuando `subtype="success"` e `is_error=True`, esto contiene la cadena de error de API si una está disponible pero puede estar vacía, así que verifique `api_error_status` y el contenido anterior de `AssistantMessage` para obtener detalles.
* `errors`: cadenas de error a nivel de bucle como el mensaje de máx-turnos. Se rellena solo en los subtipos `error_*`.
* `terminal_reason`: por qué terminó el bucle de consulta, como `"completed"`, `"max_turns"`, `"api_error"`, `"aborted_streaming"`, o `"aborted_tools"`. Un valor de `"aborted_streaming"` o `"aborted_tools"` significa que el turno fue abortado antes de completarse. Las causas comunes son [`interrupt()`](#claudesdkclient) y una devolución de llamada de permiso que devuelve [`PermissionResultDeny`](#permissionresultdeny) con `interrupt=True`. `None` en versiones de CLI que preceden al campo, en resultados de comandos locales como `/voice` o `/usage`, que omiten el bucle de consulta, o en resultados de error sintetizados emitidos cuando la sesión falla fatalmente. Refleja el [`SDKResultMessage.terminal_reason`](/docs/es/agent-sdk/typescript#sdkresultmessage) del SDK de TypeScript, que enumera el conjunto completo de valores.
* `origin`: origen del mensaje del usuario que activó este turno. En [modo de entrada de streaming](/docs/es/agent-sdk/streaming-vs-single-mode), verifique esto para distinguir el resultado de su propio prompt, donde `origin` es `None` o `{"kind": "human"}`, del resultado de un turno inyectado como una notificación de tarea de fondo. Requiere Python Agent SDK 0.2.137 o posterior.

El dict `usage` cubre solo el bucle del agente principal y excluye subagentes y otras llamadas de modelo anidadas o auxiliares. En [modo de entrada de streaming](/docs/es/agent-sdk/streaming-vs-single-mode), los valores son por turno. Prefiera `model_usage` para contabilidad de token y costo. El dict `usage` contiene las siguientes claves cuando está presente:

| Clave                         | Tipo  | Descripción                                                                                                                                                                                                                          |
| ----------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `input_tokens`                | `int` | Tokens de entrada consumidos por el bucle del agente de nivel superior. [Los tokens de subagentes no se incluyen](/docs/es/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); use `model_usage` para contabilidad de árbol completo. |
| `output_tokens`               | `int` | Tokens de salida generados por el bucle del agente de nivel superior. Los tokens de subagentes no se incluyen.                                                                                                                       |
| `cache_creation_input_tokens` | `int` | Tokens usados para crear nuevas entradas de caché.                                                                                                                                                                                   |
| `cache_read_input_tokens`     | `int` | Tokens leídos de entradas de caché existentes.                                                                                                                                                                                       |

El dict `model_usage` asigna nombres de modelo a uso por modelo. Cubre cada llamada de modelo realizada a través de la canalización de consulta: el bucle principal, subagentes y llamadas internas como compactación y agentes de Workflow. Las llamadas auxiliares fuera de esa canalización, como el clasificador de permisos y solicitudes de conteo de tokens, se excluyen de `model_usage`. Trate `model_usage` como una estimación, no como un estado de facturación.

En [modo de entrada de streaming](/docs/es/agent-sdk/streaming-vs-single-mode), `model_usage` y `total_cost_usd` son acumulativos entre turnos, así que lea el resultado más reciente en lugar de sumar entre resultados. Una llamada que reanuda una sesión también cuenta los [totales restaurados de las llamadas anteriores de la sesión](/docs/es/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Vea [Rastrear costos en modo de entrada de streaming](/docs/es/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) para reiniciaciones y [Recuperar totales después de un bloqueo de sesión](/docs/es/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) para resultados puestos a cero.

Cada valor en `model_usage` es un TypedDict `ModelUsage`, importado vía `from claude_agent_sdk.types import ModelUsage`. Sus claves usan camelCase porque el SDK pasa el valor sin modificar desde el proceso CLI subyacente, coincidiendo con el tipo TypeScript [`ModelUsage`](/docs/es/agent-sdk/typescript#modelusage):

| Clave                      | Tipo    | Descripción                                                                                                                                                                                                                                                                                                    |
| -------------------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | Tokens de entrada para este modelo.                                                                                                                                                                                                                                                                            |
| `outputTokens`             | `int`   | Tokens de salida para este modelo.                                                                                                                                                                                                                                                                             |
| `cacheReadInputTokens`     | `int`   | Tokens de lectura de caché para este modelo.                                                                                                                                                                                                                                                                   |
| `cacheCreationInputTokens` | `int`   | Tokens de creación de caché para este modelo.                                                                                                                                                                                                                                                                  |
| `webSearchRequests`        | `int`   | Solicitudes de búsqueda web realizadas por este modelo.                                                                                                                                                                                                                                                        |
| `thinkingTokens`           | `int`   | Tokens de pensamiento generados por este modelo, ya contados en `outputTokens`. Ausente hasta que un turno se ejecute en una versión de Claude Code que lo registre, y no declarado en el TypedDict, así que léalo con `.get()`. Requiere Python Agent SDK 0.2.150 o posterior, cuya CLI incluida lo registra. |
| `costUSD`                  | `float` | Costo estimado en USD para este modelo, calculado del lado del cliente. Vea [Rastrear costo y uso](/docs/es/agent-sdk/cost-tracking) para advertencias de facturación.                                                                                                                                              |
| `contextWindow`            | `int`   | Tamaño de ventana de contexto para este modelo.                                                                                                                                                                                                                                                                |
| `maxOutputTokens`          | `int`   | Límite de token de salida máximo para este modelo.                                                                                                                                                                                                                                                             |
| `canonicalModel`           | `str`   | ID de modelo canónico utilizado para la búsqueda de precios. Puede diferir de la cadena de modelo sin procesar por la que se indexa la entrada, como un ID específico del proveedor o un alias. No siempre presente.                                                                                           |
| `provider`                 | `str`   | Proveedor de API que sirvió este modelo, como `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle`, o `gateway`. No siempre presente.                                                                                                                                                        |

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

<h3 id="conversationresetmessage">
  `ConversationResetMessage`
</h3>

Emitido cuando la conversación se reemplaza sin terminar la conexión, como después de `/clear`. Vea [Rastrear costos en modo de entrada de streaming](/docs/es/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) para cómo un reinicio afecta los totales en ejecución en objetos `ResultMessage` posteriores. Requiere Python Agent SDK 0.2.137 o posterior.

```python theme={null}
@dataclass
class ConversationResetMessage:
    new_conversation_id: str
    uuid: str
    session_id: str
```

| Campo                 | Tipo  | Descripción                                                                                                                  |
| :-------------------- | :---- | :--------------------------------------------------------------------------------------------------------------------------- |
| `new_conversation_id` | `str` | Identificador opaco para la conversación nueva. No es el `session_id` de mensajes posteriores; lea eso del siguiente mensaje |
| `uuid`                | `str` | Identificador único del mensaje                                                                                              |
| `session_id`          | `str` | ID de la sesión que fue reiniciada. Los mensajes después del reinicio llevan un nuevo `session_id`                           |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

Emitido cuando comienza una tarea de fondo. Una tarea de fondo es cualquier cosa rastreada fuera del turno principal: un comando Bash en segundo plano, un reloj de [Monitor](#monitor), un subagente generado a través de la herramienta Agent, o un agente remoto. El campo `task_type` le dice cuál. Este nombre no está relacionado con el cambio de nombre de herramienta `Task`-a-`Agent`.

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

Cuando la CLI [mueve una llamada de herramienta MCP larga al fondo](/docs/es/mcp#automatic-backgrounding-of-long-tool-calls), el resultado de la herramienta para esa llamada contiene solo un marcador de posición y el resultado real de la llamada llega en este mensaje. En una notificación `"completed"` para tal llamada, la CLI agrega una clave `resource_links` que enumera los archivos que devolvió la herramienta por referencia, con las mismas entradas y límites que la clave `resourceLinks` en [`UserMessage.tool_use_result`](#usermessage). La clave `resource_links` requiere Python Agent SDK 0.2.150 o posterior y Claude Code v2.1.257 o posterior; la CLI incluida con esa versión del SDK satisface el requisito de Claude Code.

La clase de datos no tiene campo para `resource_links`. Léalo del dict `data` que el mensaje hereda de [`SystemMessage`](#systemmessage): `message.data.get("resource_links")`. Haga coincidir la notificación con la llamada usando `tool_use_id`. La CLI omite la clave cuando el resultado no tenía enlaces y en notificaciones para tareas que no son llamadas de herramienta MCP.

<h2 id="content-block-types">
  Tipos de bloque de contenido
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Tipo de unión de todos los bloques de contenido.

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

Los tipos a continuación definen lo que su código detecta. Para entradas vinculadas a los mensajes de error que estos tipos generan, con la causa y la solución para cada uno, consulte [Solución de problemas](/docs/es/agent-sdk/troubleshooting).

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Clase de excepción base para todos los errores del SDK.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

Cuando una consulta `query()` de un solo turno termina con un resultado de error, por ejemplo un error de límite de turnos, el SDK genera una [`ResultError`](#resulterror) después de ceder el mensaje de resultado final. Las versiones del SDK del Agente Python anteriores a 0.2.140 generaban una `Exception` simple que no era una subclase de `ClaudeSDKError`.

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

<h3 id="resulterror">
  `ResultError`
</h3>

Se genera después del [`ResultMessage`](#resultmessage) final cuando el proceso de Claude Code se cierra porque la ejecución terminó con un resultado de error, como un error de límite de turnos o un error de API. `ResultError` es una subclase de `ProcessError`, por lo que un controlador `except ProcessError` existente también lo detecta. Sus atributos llevan los campos de ese mensaje de resultado, por lo que puede ramificarse según por qué falló la ejecución sin analizar el texto del mensaje. Requiere Python Agent SDK 0.2.140 o posterior.

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

Para distinguir los fallos, compruebe `terminal_reason` antes de `subtype`. Cuando la solicitud final falla, como en un error de API, Claude Code informa `subtype` `"success"` con la causa en `terminal_reason`, por ejemplo `"api_error"`; cuando un límite que establece termina la ejecución, como `max_turns` o `max_budget_usd`, informa un subtipo `error_*`.

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
  El SDK de TypeScript admite eventos de hook adicionales no disponibles aún en Python. Ver la [tabla de disponibilidad de hooks](/docs/es/agent-sdk/hooks#available-hooks) para el soporte por SDK.
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

Devuelve un [`HookJSONOutput`](#hookjsonoutput).

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
        None  # Timeout in seconds. When omitted, the per-event default applies:
        # 600 for most events, 30 for UserPromptSubmit
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

| Campo             | Tipo                            | Descripción                                                                                                                                                                                                                                                                         |
| :---------------- | :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Siempre "PostToolUseFailure"                                                                                                                                                                                                                                                        |
| `tool_name`       | `str`                           | Nombre de la herramienta que falló                                                                                                                                                                                                                                                  |
| `tool_input`      | `dict[str, Any]`                | Parámetros de entrada que se utilizaron                                                                                                                                                                                                                                             |
| `tool_use_id`     | `str`                           | Identificador único para este uso de herramienta                                                                                                                                                                                                                                    |
| `error`           | `str`                           | Mensaje de error de la ejecución fallida                                                                                                                                                                                                                                            |
| `is_interrupt`    | `bool` (opcional)               | Verdadero cuando el fallo llegó a Claude Code como una interrupción en lugar de como un error que la herramienta reportó. Cancelar una herramienta en ejecución con `interrupt()` no dispara este hook; el resultado de la herramienta lleva el mensaje de interrupción en su lugar |
| `agent_id`        | `str` (opcional)                | Identificador de subagente, presente cuando el hook se dispara dentro de un subagente                                                                                                                                                                                               |
| `agent_type`      | `str` (opcional)                | Tipo de subagente, presente cuando el hook se dispara dentro de un subagente                                                                                                                                                                                                        |

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

Una unión discriminada de tipos de salida específicos del evento `TypedDict`. El campo `hookEventName` determina qué campos son válidos. Para detalles completos sobre campos disponibles por evento de hook, ver [Control execution with hooks](/docs/es/agent-sdk/hooks#outputs).

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
  Tipos de entrada/salida de herramienta
</h2>

Documentación de esquemas de entrada/salida para todas las herramientas integradas de Claude Code. Aunque el SDK de Python no exporta estos como tipos, representan la estructura de entradas y salidas de herramientas en mensajes.

<h3 id="agent">
  Agent
</h3>

**Nombre de herramienta:** `Agent`. El nombre anterior `Task` aún se acepta como alias, y la lista `tools` en el [`SystemMessage`](#systemmessage) de inicialización reporta esta herramienta como `Task` para compatibilidad hacia atrás.

**Entrada:**

```python theme={null}
{
    "description": str,  # A short (3-5 word) description of the task
    "prompt": str,  # The task for the agent to perform
    "subagent_type": str | None,  # The type of specialized agent to use
    "model": "sonnet" | "opus" | "haiku" | "fable" | None,  # Model override for this agent
    "run_in_background": bool | None,  # Agents run in the background by default; set to False to run synchronously
    "name": str | None,  # Name for the spawned agent
    "team_name": str | None,  # Deprecated; ignored
    "mode": "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan" | None,  # Deprecated; ignored. The subagent inheritance rules decide a subagent's permission mode
    "isolation": "worktree" | "remote" | None,  # Isolation mode for the agent's changes
}
```

Lanza un nuevo agente para manejar tareas complejas y multietapa de forma autónoma.

**Salida (estado: `"completed"`):**

```python theme={null}
{
    "status": "completed",
    "agentId": str,  # ID of the agent that ran
    "agentType": str | None,  # The subagent type that handled the task
    "content": [  # Result content blocks
        {
            "type": "text",
            "text": str,
            "citations": list | None,
        }
    ],
    "resolvedModel": str | None,  # Model the subagent started on
    "modelsUsed": list[str] | None,  # Models used in order, with consecutive repeats collapsed
    "totalToolUseCount": int,  # Number of tool calls the agent made
    "totalDurationMs": int,  # Execution duration in milliseconds
    "totalTokens": int,  # Token count from the final API request, not the whole run
    "usage": {  # Token usage statistics
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
    "toolStats": {  # Aggregate tool activity for the run
        "readCount": int,
        "searchCount": int,
        "bashCount": int,
        "editFileCount": int,
        "linesAdded": int,
        "linesRemoved": int,
        "otherToolCount": int,
        "frameCount": int | None,
    } | None,
    "prompt": str,  # The prompt the agent ran
    "worktreePath": str | None,  # Present when Claude Code kept the subagent's worktree
    "worktreeBranch": str | None,  # Present when Claude Code created that worktree with git
}
```

**Salida (estado: `"async_launched"`):**

```python theme={null}
{
    "status": "async_launched",
    "isAsync": bool | None,  # True on background launches
    "agentId": str,  # ID of the launched agent
    "description": str,  # The task description
    "resolvedModel": str | None,  # Model in use at the backgrounding transition
    "modelsUsed": list[str] | None,  # Models used before backgrounding, in order, with consecutive repeats collapsed
    "prompt": str,  # The prompt the agent runs
    "outputFile": str,  # File path where the agent's output is written
    "canReadOutputFile": bool | None,  # Whether the output file can be read directly
}
```

**Salida (estado: `"remote_launched"`):**

```python theme={null}
{
    "status": "remote_launched",
    "taskId": str,  # ID of the dispatched task
    "sessionUrl": str,  # Link to the cloud session
    "description": str,  # The task description
    "prompt": str,  # The prompt the agent runs
    "outputFile": str,  # File path where the agent's output is written
}
```

Devuelve el resultado del subagente. La salida se discrimina en el campo `status`: `"completed"` para tareas terminadas, `"async_launched"` para tareas en segundo plano, y `"remote_launched"` para tareas que Claude Code envió a una sesión en la nube, donde `sessionUrl` enlaza a esa sesión y `taskId` la identifica. Si Claude Code [mantuvo el worktree aislado del subagente](/docs/es/worktrees#isolate-subagents-with-worktrees), `worktreePath` en la variante `completed` es donde encontrarlo, y `worktreeBranch` es su rama cuando Claude Code creó el worktree con git.

En la variante `completed`, `resolvedModel` nombra el modelo en el que comenzó el subagente, que puede diferir del `model` de entrada solicitado cuando [`availableModels`](/docs/es/model-config#restrict-model-selection) u otra anulación se aplica. Este campo requiere Claude Code v2.1.174 o posterior. En la variante `async_launched`, `resolvedModel` nombra el modelo en uso cuando el agente se movió al segundo plano, por lo que un cambio que ocurrió antes del envío a segundo plano se refleja allí. El campo `modelsUsed` en ambas variantes enumera los modelos utilizados en orden, con repeticiones consecutivas colapsadas; se establece solo cuando el modelo se cambió durante la ejecución. `modelsUsed` y el comportamiento de `resolvedModel` en el momento del envío a segundo plano requieren Claude Code v2.1.212 o posterior.

Claude Code completa `usage` y `totalTokens` desde la solicitud final de API del subagente, no desde toda la ejecución. Cuando está presente, `thinking_tokens` bajo `output_tokens_details` en `usage` es el número de tokens de salida de esa solicitud que fueron tokens de pensamiento. La clave `output_tokens_details` requiere Python SDK v0.2.136 o posterior, que incluye Claude Code v2.1.228.

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
                    "preview": str | None,  # Preview content rendered when the option is focused
                }
            ],
            "multiSelect": bool,  # Set to true to allow multiple selections
        }
    ],
    "answers": dict[str, str] | None,
    # User answers populated by the permission system. Multi-select
    # answers are a comma-joined string of the selected labels; a
    # list of labels is accepted on input and coerced to that form
    "annotations": dict[str, dict] | None,
    # Per-question annotations from the user, keyed by question text.
    # Each value can carry "preview" (the selected option's preview
    # content) and "notes" (free-text notes on the selection)
    "metadata": dict | None,  # Analytics metadata, such as {"source": "remember"}; not displayed to the user
}
```

**Salida:**

```python theme={null}
{
    "questions": [  # The questions that were asked
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str, "preview": str | None}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Maps question text to answer string
    # Multi-select answers are comma-separated
    "response": str | None,
    # Freeform reply typed instead of answering the questions; when set,
    # Claude receives "The user responded: ..." in place of the answer list
    "annotations": dict[str, dict] | None,  # Per-question "preview" and "notes" from the user's selections
    "afkTimeoutMs": int | None,  # Set when the dialog auto-resolved after this many milliseconds of user inactivity; absent when the user answered
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
    "stdout": str,  # The command's output; stdout and stderr arrive merged into this one interleaved stream
    "stderr": str,  # Notices the tool itself adds, not the command's stderr
    "interrupted": bool,  # Whether the command was interrupted
    "isImage": bool | None,  # Whether stdout contains image data
    "backgroundTaskId": str | None,  # ID of the background task if command is running in background
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
    "timeout_ms": int | None,  # Deadline in milliseconds (default 300000, max 3600000; the effective deadline is at most 1800000)
}
```

**Salida:**

```python theme={null}
{
    "taskId": str,  # ID of the background monitor task
    "timeoutMs": int,  # The watch's effective deadline in milliseconds
    "persistent": bool | None,  # False: every watch has a deadline
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
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Ver [Disponibilidad de modelos](/docs/es/agent-sdk/todo-tracking#model-availability) para optar por participar.
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

<h3 id="taskoutput">
  TaskOutput
</h3>

Eliminado en Claude Code v2.1.277. Anteriormente recuperaba salida de una tarea en ejecución o completada, con `BashOutput` aceptado como alias; Claude lee el archivo de salida de una tarea en segundo plano con `Read` en su lugar.

Una entrada `disallowed_tools` o una regla de denegación que aún nombre cualquiera de estos nombres se ignora sin una advertencia.

<h3 id="taskstop">
  TaskStop
</h3>

**Nombre de herramienta:** `TaskStop`. Los nombres anteriores `KillShell` y `KillBash` aún se aceptan como alias.

**Entrada:**

```python theme={null}
{
    "task_id": str | None,  # The ID of the background task to stop
    "shell_id": str | None,  # Deprecated: use task_id instead
}
```

**Salida:**

```python theme={null}
{
    "message": str,  # Status message about the operation
    "task_id": str,  # The ID of the task that was stopped
    "task_type": str,  # The type of the task that was stopped
    "command": str | None,  # The command or description of the stopped task
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

<h2 id="build-a-continuous-conversation-interface">
  Construir una interfaz de conversación continua
</h2>

El siguiente ejemplo mantiene un `ClaudeSDKClient` conectado a través de turnos, para que Claude recuerde los mensajes anteriores. Escriba `new` para desconectar y reconectar para una sesión nueva, o `exit` para terminar la conversación.

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
  Manejo de errores
</h2>

El siguiente ejemplo envuelve una llamada a `query()` en manejadores para cuatro de los [tipos de error](#error-types) que el SDK genera.

Este ejemplo captura [`ResultError`](#resulterror), que requiere Python Agent SDK 0.2.140 o posterior.

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

| Propiedad                   | Tipo                                                  | Predeterminado | Descripción                                                                                                                                                                                                                                                     |
| :-------------------------- | :---------------------------------------------------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `bool`                                                | `False`        | Habilitar modo sandbox para ejecución de comandos                                                                                                                                                                                                               |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`         | Aprobar automáticamente comandos bash cuando sandbox está habilitado                                                                                                                                                                                            |
| `excludedCommands`          | `list[str]`                                           | `[]`           | Comandos que evitan restricciones de sandbox, como `["docker *"]`. Estos se ejecutan sin sandbox automáticamente sin participación del modelo; [`sandbox.excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands) cubre cuándo se aplica una entrada |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`         | Permitir que el modelo solicite ejecutar comandos fuera del sandbox. Cuando es `True`, el modelo puede establecer `dangerouslyDisableSandbox` en entrada de herramienta, que vuelve al [sistema de permisos](#permissions-fallback-for-unsandboxed-commands)    |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`         | Configuración de sandbox específica de red                                                                                                                                                                                                                      |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`         | Configurar qué violaciones de sandbox ignorar                                                                                                                                                                                                                   |
| `enableWeakerNestedSandbox` | `bool`                                                | `False`        | Habilitar un sandbox anidado más débil para compatibilidad                                                                                                                                                                                                      |

<Note>
  El sandbox depende de la compatibilidad de la plataforma y, en Linux, de herramientas como `bubblewrap` y `socat`. De forma predeterminada, cuando `enabled` es `True` pero el sandbox no puede iniciarse, los comandos se ejecutan sin sandbox con una advertencia en stderr. Este comportamiento predeterminado difiere del SDK de TypeScript, donde `failIfUnavailable` tiene un valor predeterminado de `true`.

  Establezca `"failIfUnavailable": True` en su configuración de sandbox para detener en su lugar. La clave aún no está declarada en `SandboxSettings`, pero el SDK la reenvía a Claude Code, que la respeta. `query()` luego reporta un `ResultMessage` con `subtype="error_during_execution"` y la razón en `errors`. Debido a que se trata de una llamada única a `query()`, el SDK lanza después de ceder ese resultado de error, así que envuelva el bucle en un bloque try para continuar pasado él. Consulte [Manejar el resultado](/docs/es/agent-sdk/agent-loop#handle-the-result) para el contrato de error.
</Note>

<h4 id="example-usage">
  Ejemplo de uso
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
  **Seguridad de socket Unix**: La opción `allowUnixSockets` puede otorgar acceso a servicios del sistema que alcanzan fuera del sandbox. Por ejemplo, permitir `/var/run/docker.sock` efectivamente otorga acceso completo al sistema host a través de la API de Docker, evitando el aislamiento de sandbox. Solo permita sockets Unix que sean estrictamente necesarios y comprenda las implicaciones de seguridad de cada uno.
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

| Propiedad                 | Tipo        | Predeterminado | Descripción                                                                                                                                                                                                                                                                 |
| :------------------------ | :---------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `list[str]` | `[]`           | Nombres de dominio que los procesos en sandbox pueden acceder                                                                                                                                                                                                               |
| `deniedDomains`           | `list[str]` | `[]`           | Nombres de dominio que los procesos en sandbox no pueden acceder. Tiene precedencia sobre `allowedDomains`                                                                                                                                                                  |
| `allowManagedDomainsOnly` | `bool`      | `False`        | Solo configuración administrada: cuando se establece en configuración administrada, ignorar `allowedDomains` y reglas de permitidos de `WebFetch(domain:...)` de fuentes de configuración no administradas. No tiene efecto cuando se establece a través de opciones de SDK |
| `allowUnixSockets`        | `list[str]` | `[]`           | Solo macOS: rutas de socket Unix que los procesos pueden acceder, como el socket de Docker. Se ignora en Linux                                                                                                                                                              |
| `allowAllUnixSockets`     | `bool`      | `False`        | Permitir acceso a todos los sockets Unix                                                                                                                                                                                                                                    |
| `allowLocalBinding`       | `bool`      | `False`        | Permitir que los procesos se vinculen a puertos locales (por ejemplo, para servidores de desarrollo)                                                                                                                                                                        |
| `allowMachLookup`         | `list[str]` | `[]`           | Solo macOS: nombres de servicios XPC/Mach para permitir. Admite un comodín al final                                                                                                                                                                                         |
| `httpProxyPort`           | `int`       | `None`         | Puerto proxy HTTP para solicitudes de red                                                                                                                                                                                                                                   |
| `socksProxyPort`          | `int`       | `None`         | Puerto proxy SOCKS para solicitudes de red                                                                                                                                                                                                                                  |

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

Sus entradas de `excludedCommands` en su lugar evitan el sandbox sin participación del modelo; [`sandbox.excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands) cubre cuándo se aplica una entrada.

El siguiente ejemplo registra cada solicitud sin sandbox y la deniega a menos que su propia lógica de autorización la permita:

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
  Los comandos que se ejecutan con `dangerouslyDisableSandbox: True` tienen acceso completo al sistema. Asegúrese de que su controlador `can_use_tool` valide estas solicitudes cuidadosamente.

  Si `permission_mode` se establece en `bypassPermissions` y `allow_unsandboxed_commands` está habilitado, el modelo puede ejecutar autónomamente comandos fuera del sandbox sin solicitudes de aprobación, aparte de las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves). Esta combinación efectivamente permite que el modelo escape del aislamiento de sandbox silenciosamente.
</Warning>

<h2 id="see-also">
  Ver también
</h2>

* [SDK overview](/docs/es/agent-sdk/overview) - Conceptos generales del SDK
* [TypeScript SDK reference](/docs/es/agent-sdk/typescript) - Documentación del SDK de TypeScript
* [Custom tools](/docs/es/agent-sdk/custom-tools) - Define herramientas MCP en proceso para que Claude llame
* [CLI reference](/docs/es/cli-reference) - Interfaz de línea de comandos
* [Common workflows](/docs/es/common-workflows) - Guías paso a paso
