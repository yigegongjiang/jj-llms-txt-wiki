> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Persistir sesiones en almacenamiento externo

> Refleja transcripciones de sesiones del SDK de Agent en su propio almacén de objetos, almacén de clave-valor o base de datos para que otros hosts puedan reanudar sus sesiones.

De forma predeterminada, el SDK escribe transcripciones de sesiones en archivos JSONL bajo `~/.claude/projects/` en el sistema de archivos local. Un adaptador `SessionStore` le permite reflejar esas transcripciones en su propio backend, como un almacén de objetos, un almacén de clave-valor o una base de datos, para que una sesión creada en un host pueda reanudarse en otro host que se ejecute desde un directorio de trabajo coincidente.

Razones comunes para usar un almacén de sesiones:

* **Implementaciones multi-host.** Las funciones sin servidor, los trabajadores con escalado automático y los ejecutores de CI no comparten un sistema de archivos. Un almacén compartido permite que las réplicas reanuden las sesiones de las demás.
* **Durabilidad.** Los contenedores locales son efímeros. Un almacén externo sobrevive a reinicios y redeploys.
* **Cumplimiento y auditoría.** Mantén transcripciones en almacenamiento que ya gobiernas, con tus propias reglas de retención, cifrado y controles de acceso.

<h2 id="the-sessionstore-interface">
  La interfaz `SessionStore`
</h2>

Un `SessionStore` es un objeto con dos métodos requeridos, `append` y `load`, y cuatro métodos opcionales. El SDK llama a `append` para escribir entradas de transcripción durante una consulta y a `load` para leerlas de nuevo para reanudar.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

  type SessionKey = {
    projectKey: string;
    sessionId: string;
    subpath?: string;
  };

  type SessionStore = {
    // Required
    append(key: SessionKey, entries: SessionStoreEntry[]): Promise<void>;
    load(key: SessionKey): Promise<SessionStoreEntry[] | null>;

    // Optional
    listSessions?(
      projectKey: string,
    ): Promise<Array<{ sessionId: string; mtime: number }>>;
    listSessionSummaries?(projectKey: string): Promise<SessionSummaryEntry[]>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };

  type SessionSummaryEntry = {
    sessionId: string;
    mtime: number;
    data: Record<string, unknown>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

  class SessionKey(TypedDict):
      project_key: str
      session_id: str
      subpath: NotRequired[str]

  class SessionStore(Protocol):
      # Required
      async def append(
          self, key: SessionKey, entries: list[SessionStoreEntry]
      ) -> None: ...
      async def load(self, key: SessionKey) -> list[SessionStoreEntry] | None: ...

      # Optional — omit or raise NotImplementedError
      async def list_sessions(
          self, project_key: str
      ) -> list[SessionStoreListEntry]: ...
      async def list_session_summaries(
          self, project_key: str
      ) -> list[SessionSummaryEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...

  class SessionSummaryEntry(TypedDict):
      session_id: str
      mtime: int
      data: dict[str, Any]
  ```
</CodeGroup>

`SessionKey` direcciona una transcripción. `projectKey` es una codificación estable y segura para el sistema de archivos del directorio de trabajo, `sessionId` es el UUID de la sesión, y `subpath` se establece cuando la entrada pertenece a una transcripción de subagente o archivo sidecar en lugar de la conversación principal.

Debido a que `projectKey` codifica el directorio de trabajo, reanude o continúe desde el almacén desde un directorio de trabajo que coincida con la ejecución original. En TypeScript, si establece [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/es/sessions#name-the-project-directory-yourself) junto a `CLAUDE_CONFIG_DIR` en la opción [`env`](/docs/es/agent-sdk/typescript#options) de una consulta, el SDK codifica las entradas de esa consulta, y sus búsquedas de `resume` y `continue`, por ese nombre en su lugar. Debido a que los ayudantes independientes como `listSessions` y `deleteSession` no toman `env` y leen el entorno del proceso, establezca `CLAUDE_CONFIG_DIR` y el mismo nombre en el entorno del proceso del host también. Requiere Agent SDK v0.3.234 o posterior.

Trate `subpath` como una clave de sufijo opaca; sigue el diseño en disco, por ejemplo `subagents/agent-<id>`. Cuando `subpath` no está definido, la clave se refiere a la transcripción principal.

| Método                 | Requerido | Se llama cuando                                                                                                                                                                                                                                                                                                                                                      |
| :--------------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`               | Sí        | Después de que cada lote de entradas de transcripción se escriba localmente. Las entradas son objetos seguros para JSON, uno por línea en el JSONL local.                                                                                                                                                                                                            |
| `load`                 | Sí        | Antes de que se genere el subproceso cuando `resume` está establecido o `continue: true` resuelve la sesión de almacén más reciente, y una vez por sesión cuando la enumeración se retrae de `listSessionSummaries`. Devuelve `null` si la sesión es desconocida.                                                                                                    |
| `listSessions`         | No        | Por `listSessions({ sessionStore })` y por `query()`/`startup()` con `continue: true`. Si no está definido, `continue: true` lanza una excepción, y `listSessions({ sessionStore })` lanza una excepción a menos que `listSessionSummaries` esté implementado.                                                                                                       |
| `listSessionSummaries` | No        | Por `listSessions({ sessionStore })` para leer metadatos de todas las sesiones en una llamada. Mantenga los resúmenes dentro de `append`. Si no está definido, la enumeración se retrae a `listSessions` más una `load` por sesión.                                                                                                                                  |
| `delete`               | No        | Por `deleteSession({ sessionStore })`. Eliminar la clave principal (sin `subpath`) debe cascada a todas las subclaves para esa sesión y también eliminar la entrada de resumen de la sesión, por lo que una sesión eliminada deja de aparecer en `listSessionSummaries`. Si no está definido, la eliminación es una no-op, que se adapta a backends de solo anexión. |
| `listSubkeys`          | No        | Durante la reanudación, para descubrir transcripciones de subagenteor. Si no está definido, solo se restaura la transcripción principal.                                                                                                                                                                                                                             |

En una `SessionSummaryEntry`, `mtime` es el tiempo de escritura del almacenamiento del sidecar y debe compartir una fuente de reloj con los valores `mtime` que `listSessions` devuelve. `data` es un estado propiedad del SDK opaco; persístalo textualmente sin interpretarlo.

Construya las entradas llamando al ayudante exportado `foldSessionSummary`, `fold_session_summary` en Python, en cada lote dentro de `append`. Omita lotes cuya clave tenga un `subpath`; las transcripciones de subagente no deben contribuir al resumen de la sesión principal. El fold nunca establece `mtime`: establézcalo en el momento de la persistencia, a través del argumento `options.mtime` en TypeScript o sobrescribiendo el campo en la entrada devuelta en Python. Las llamadas concurrentes a `append` para la misma sesión pueden competir en el sidecar, así que serialice la lectura-fold-escritura con una transacción, una comparación e intercambio, o un bloqueo por sesión; el fold en sí es puro.

Para lo que el SDK hace con la transcripción que `load` devuelve, consulte [Reanudar desde el almacén](#resume-from-the-store).

<h2 id="quick-start">
  Inicio rápido
</h2>

El SDK incluye un `InMemorySessionStore` para desarrollo y pruebas. El ejemplo a continuación ejecuta una consulta con el almacén adjunto, captura el ID de sesión del mensaje de resultado, luego reanuda desde el almacén en una segunda llamada `query()`. La segunda llamada pasa la misma instancia de almacén más `resume`, por lo que el SDK carga la transcripción del almacén en lugar del sistema de archivos local:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  try {
    for await (const message of query({
      prompt: "List the TypeScript files under src/",
      options: { sessionStore: store },
    })) {
      if (message.type === "result") {
        sessionId = message.session_id;
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, sessionId was already captured by the loop
    // above; connection or process failures yield no result message.
    console.error(`Session ended with an error: ${error}`);
  }

  // Resume from the store. The agent has full context from the first call.
  for await (const message of query({
    prompt: "Summarize what those files do",
    options: { sessionStore: store, resume: sessionId },
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      ClaudeAgentOptions,
      InMemorySessionStore,
      ResultMessage,
      query,
  )

  store = InMemorySessionStore()


  async def main():
      session_id = None
      try:
          async for message in query(
              prompt="List the Python files under src/",
              options=ClaudeAgentOptions(session_store=store),
          ):
              if isinstance(message, ResultMessage):
                  session_id = message.session_id
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, session_id was already captured by the
          # loop above; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")

      # Resume from the store. The agent has full context from the first call.
      async for message in query(
          prompt="Summarize what those files do",
          options=ClaudeAgentOptions(session_store=store, resume=session_id),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

La segunda consulta imprime un resumen de los archivos de la primera consulta, lo que muestra que el agente reanudó con contexto completo desde el almacén.

<h2 id="write-your-own-adapter">
  Escribe tu propio adaptador
</h2>

Implementa `append` y `load` contra tu backend. Añade `listSessions`, `listSessionSummaries`, `delete` y `listSubkeys` si deseas que `listSessions()`, lecturas de metadatos de una sola llamada, `deleteSession()` y la reanudación de subagentes funcionen contra el almacén.

Las entradas pasadas a `append` se escriben como `SessionStoreEntry` (un objeto `{ type: string; ... }`). Trátalas como valores opacos seguros para JSON: persístalas en orden y devuélvelas desde `load` en el mismo orden. `load` debe devolver entradas que sean profundamente iguales a lo que se anexó; la serialización byte-igual no es requerida, por lo que backends que reordenan claves de objeto, como un tipo de columna JSON binaria, están bien.

<h2 id="reference-implementations">
  Implementaciones de referencia
</h2>

Ambos repositorios del SDK incluyen adaptadores de referencia ejecutables bajo [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) en TypeScript y [`examples/session_stores/`](https://github.com/anthropics/claude-agent-sdk-python/tree/main/examples/session_stores) en Python. Hay un adaptador por tipo de almacenamiento, y cada uno muestra cómo `append` y `load` se asignan a ese tipo de backend. No se publican como paquetes; copia el adaptador del tipo más cercano a tu backend en tu proyecto, instala el cliente de tu backend y adáptalo.

| Tipo de almacenamiento                           | Modelo de almacenamiento                                                                                        | Adaptador de ejemplo                                                                                                                                                                                                                                       |
| :----------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Almacén de objetos                               | Un archivo de parte por `append()`; `load()` lista las partes, las ordena y las concatena.                      | S3 ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/s3_session_store.py))                   |
| Almacén de clave-valor                           | Una lista por transcripción que `append()` inserta y `load()` lee en rango, más un índice ordenado de sesiones. | Redis ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/redis_session_store.py))          |
| Base de datos relacional o almacén de documentos | Una fila o documento por entrada, almacenado como JSON y ordenado por una clave asignada en la inserción.       | Postgres ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/postgres_session_store.py)) |

Cada adaptador toma una instancia de cliente preconfigurada, por lo que controlas credenciales, TLS, región y agrupación. El siguiente ejemplo conecta el adaptador de almacén de objetos en `query()` y luego se reanuda desde él en otro host:

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import { S3Client } from "@aws-sdk/client-s3";
import { S3SessionStore } from "./S3SessionStore"; // copied from examples/session-stores/s3

const store = new S3SessionStore({
  bucket: "my-claude-sessions",
  prefix: "transcripts",
  client: new S3Client({ region: "us-east-1" }),
});

for await (const message of query({
  prompt: "Hello!",
  options: { sessionStore: store },
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Later, possibly on a different host:
for await (const message of query({
  prompt: "Continue where we left off",
  options: { sessionStore: store, resume: "previous-session-id" },
})) {
  // ...
}
```

<h3 id="validate-your-adapter">
  Valida tu adaptador
</h3>

Ambos SDKs incluyen un conjunto de conformidad que afirma el contrato de comportamiento que `append`, `load` y los métodos opcionales deben satisfacer. Las pruebas para métodos opcionales se omiten automáticamente cuando esos métodos no se implementan.

En TypeScript, copia [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) del directorio de ejemplos en tu suite de pruebas. En Python, el conjunto se incluye en el paquete. Para ejecutarlo con pytest, que no es una dependencia del SDK, instala pytest primero:

```bash theme={null}
pip install pytest
```

Luego pasa tu adaptador al conjunto en un archivo de prueba como una fábrica sin argumentos, que `run_session_store_conformance` llama una vez por contrato para construir un almacén nuevo:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.anyio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

Pasar la clase `MyRedisStore` en sí, como hace este ejemplo, funciona cuando el constructor no toma argumentos. Para un adaptador que toma un cliente preconfigurado, pasa una lambda que construya el almacén en su lugar. Debido a que los contratos reutilizan las mismas claves de sesión, cada almacén que devuelve la fábrica debe comenzar con almacenamiento vacío, así que haz que la lambda aprovisione almacenamiento de respaldo aislado por llamada, como un falso en memoria nuevo, un prefijo de clave único, o una base de datos de prueba nueva.

<h2 id="behavior-notes">
  Notas de comportamiento
</h2>

<h3 id="dual-write-architecture">
  Arquitectura de escritura dual
</h3>

El subproceso de Claude Code siempre escribe cada lote de entradas de transcripción en el disco local primero, y luego el SDK reenvía el mismo lote a `append()` de su almacén, por lo que el almacén es un espejo del transcripción local en lugar de un reemplazo. Cuál copia sobrevive a la ejecución depende de cómo se inició la ejecución:

* **Sesión nueva, o una reanudación cuando el almacén no tiene nada para la sesión**: la transcripción local bajo su directorio de configuración sobrevive a la ejecución, y el almacén recibe una copia.
* **Ejecución [reanudada desde el almacén](#resume-from-the-store)**: la copia local se elimina al final de la ejecución, por lo que el almacén contiene la única copia duradera.

Si no desea que una sesión nueva deje una transcripción en el disco local, establezca `CLAUDE_CONFIG_DIR` en un directorio temporal en `options.env`. Una ejecución reanudada desde el almacén ya elimina su copia local, por lo que no necesita tal configuración. En TypeScript, también propague `process.env` en `env`, ya que la [opción `env`](/docs/es/agent-sdk/typescript#options) reemplaza el entorno del subproceso.

Si su aplicación inicia sesión a través de archivos en el directorio de configuración, como credenciales de OAuth o un `apiKeyHelper` en su `settings.json` de usuario, copie primero esos archivos en el directorio temporal, o establezca `ANTHROPIC_API_KEY` en `env` en su lugar. De lo contrario, la ejecución falla con `Not logged in`.

Dos opciones entran en conflicto con el espejo, y el SDK lanza una excepción al inicio si combina cualquiera de ellas con un almacén:

* **`persistSession: false`** en TypeScript: desactiva las escrituras locales en las que se basa el espejo. El SDK de Python no tiene una opción equivalente.
* **Checkpointing de archivos**, `enableFileCheckpointing` en TypeScript o `enable_file_checkpointing` en Python: escribe sus copias de seguridad de archivos directamente en el disco local, y el SDK no las refleja en el almacén.

<h3 id="resume-from-the-store">
  Reanudación desde el almacén
</h3>

Cuando pasa `resume`, o `continue: true` en TypeScript o `continue_conversation=True` en Python, junto con un almacén, el SDK solicita una transcripción al almacén antes de generar el subproceso:

* **`resume`**: el SDK solicita la sesión cuyo ID pasó.
* **`continue: true`** o **`continue_conversation=True`**: el SDK solicita la sesión más nueva del almacén.

Cuando el almacén devuelve la transcripción, el SDK la escribe en un directorio de configuración temporal, ejecuta el subproceso con `CLAUDE_CONFIG_DIR` apuntando allí, y elimina el directorio cuando finaliza la ejecución. La transcripción local que esa ejecución escribe se elimina con él, por lo que el almacén contiene la única copia duradera en esta ruta.

El SDK también siembra el directorio temporal con archivos de su directorio de configuración real. Lo que copia difiere según el idioma:

* **TypeScript**: credenciales, `.claude.json`, y su `settings.json` de usuario. De `settings.json` elimina las claves que se comportan mal bajo un directorio de configuración temporal: `enabledPlugins`, `extraKnownMarketplaces`, su alias [`additionalMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces), y cualquier `CLAUDE_CONFIG_DIR` en el bloque `env` del archivo. Antes de Agent SDK v0.3.232, el SDK no eliminaba el alias. La autenticación configurada en configuración, como [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper), funciona cuando reanuda desde el almacén. Antes de Agent SDK v0.3.222, el SDK de TypeScript copiaba solo credenciales y `.claude.json`.
* **Python**: solo credenciales y `.claude.json`, por lo que una aplicación que se autentica a través de `apiKeyHelper` en su `settings.json` de usuario falla con `Not logged in` cuando reanuda desde un almacén. Un `apiKeyHelper` en configuración administrada o de proyecto aún funciona, porque Claude Code lee esos archivos desde ubicaciones que `CLAUDE_CONFIG_DIR` no afecta.

Cuando el almacén no tiene nada para la sesión, el SDK se ejecuta bajo su directorio de configuración real en su lugar, y el resultado depende de qué opción pasó:

* **`resume`**: ambos SDKs pasan el ID al subproceso, que reanuda la transcripción local exactamente como lo hace `resume` sin un almacén.
* **`continue: true`** en TypeScript: el SDK inicia una sesión nueva.
* **`continue_conversation=True`** en Python: el SDK continúa desde la sesión local más nueva.

<h3 id="mirror-writes-are-best-effort">
  Las escrituras de espejo son de mejor esfuerzo
</h3>

Si `append()` rechaza, el SDK reintenta el lote hasta dos veces más con un retroceso corto, para un máximo de tres intentos en total. Una llamada que agota el tiempo de espera no se reintenta, ya que la llamada original aún puede llegar. Si el lote aún falla, el SDK registra el error, emite un mensaje `{ type: "system", subtype: "mirror_error" }` en el iterador, descarta el lote y continúa la consulta. Debido a que un lote reintentado puede volver a entregar entradas que ya llegaron, deduplique por `entry.uuid` en su implementación de `append()`.

Una interrupción del almacén no interrumpe el agente, ya que el subproceso escribe localmente primero. Monitoree `mirror_error` si necesita detectar pérdida de datos del almacén. En una ejecución [reanudada desde el almacén](#resume-from-the-store), un lote descartado no tiene copia sobreviviente una vez que finaliza la ejecución.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` devuelve la cadena posterior a la compactación
</h3>

`getSessionMessages({ sessionStore })` devuelve la cadena de mensajes vinculada que el agente vería al reanudar. Después de la compactación automática, los turnos anteriores se reemplazan por un resumen, por lo que una sesión cuyo almacén contiene 503 entradas sin procesar puede devolver 18 mensajes de `getSessionMessages`. Para el historial sin procesar completo, incluidos los turnos anteriores a la compactación y las entradas de metadatos, llame a `store.load(key)` directamente.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` no es una copia byte a byte
</h3>

`forkSession({ sessionStore })` lee las entradas de origen, reescribe cada campo `sessionId` y remapea los UUID de mensajes, luego añade las entradas transformadas bajo una clave nueva. Una copia a nivel de adaptador o un atajo `CopyObject` produciría una transcripción que aún hace referencia al ID de sesión anterior, por lo que el SDK no usa uno.

<h3 id="subagent-transcripts">
  Transcripciones de subagentes
</h3>

Las transcripciones de subagentes se reflejan bajo `subpath: "subagents/agent-<id>"`. `listSubagents({ sessionStore })` requiere que el adaptador implemente `listSubkeys`; `getSubagentMessages({ sessionStore })` lo usa cuando está disponible pero recurre a la subruta directa cuando no está definido. La reanudación también llama a `listSubkeys` para restaurar archivos de subagentes; sin él, solo se materializa la transcripción principal.

<h3 id="retention">
  Retención
</h3>

El SDK nunca elimina de su almacén por su cuenta. La retención es responsabilidad del adaptador: use el mecanismo de caducidad o ciclo de vida de su backend, o ejecute una limpieza programada, de acuerdo con sus requisitos de cumplimiento.

Las transcripciones locales bajo `CLAUDE_CONFIG_DIR` se barren independientemente por la configuración `cleanupPeriodDays`, siguiendo las [reglas de barrido de retención](/docs/es/claude-directory#cleaned-up-automatically). Una ejecución [reanudada desde el almacén](#resume-from-the-store) no deja transcripción local, por lo que para esas ejecuciones la retención de su almacén es la única retención que existe.

<h2 id="supported-on">
  Compatible con
</h2>

Las siguientes funciones del SDK de TypeScript aceptan una opción `sessionStore` y operan contra el almacén en lugar del sistema de archivos local cuando se proporciona:

* [`query()`](/docs/es/agent-sdk/typescript#query)
* [`startup()`](/docs/es/agent-sdk/typescript#startup)
* [`listSessions()`](/docs/es/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](/docs/es/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](/docs/es/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](/docs/es/agent-sdk/typescript#renamesession)
* [`tagSession()`](/docs/es/agent-sdk/typescript#tagsession)
* [`deleteSession()`](/docs/es/agent-sdk/typescript)
* [`forkSession()`](/docs/es/agent-sdk/typescript)
* [`listSubagents()`](/docs/es/agent-sdk/typescript)
* [`getSubagentMessages()`](/docs/es/agent-sdk/typescript)

En el SDK de Python, establezca `session_store` en [`ClaudeAgentOptions`](/docs/es/agent-sdk/python#claudeagentoptions) para ejecutar `query()` contra un almacén. Las operaciones restantes tienen cada una una función respaldada por almacén de Python que toma el almacén como argumento: `list_sessions_from_store()`, `get_session_info_from_store()`, `get_session_messages_from_store()`, `list_subagents_from_store()`, `get_subagent_messages_from_store()`, `rename_session_via_store()`, `tag_session_via_store()`, `delete_session_via_store()` y `fork_session_via_store()`. `startup()` no tiene equivalente en Python. Las funciones independientes documentadas en la [referencia del SDK de Python](/docs/es/agent-sdk/python#functions), como `list_sessions()`, leen archivos de sesión locales.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Trabajar con sesiones](/docs/es/agent-sdk/sessions): Continúa, reanuda y bifurca sin un almacén personalizado
* [Alojar el SDK](/docs/es/agent-sdk/hosting): Patrones de implementación para entornos multi-host
* [TypeScript `Options`](/docs/es/agent-sdk/typescript#options): Referencia de opción completa
* [Implementaciones de referencia](#reference-implementations): Adaptadores de ejemplo ejecutables para un almacén de objetos, un almacén de clave-valor y una base de datos, en ambos repositorios del SDK
