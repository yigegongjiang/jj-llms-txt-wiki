> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Persistir sesiones en almacenamiento externo

> Refleja transcripciones de sesiones en S3, Redis o tu propio backend para que cualquier host pueda reanudarlas.

De forma predeterminada, el SDK escribe transcripciones de sesiones en archivos JSONL bajo `~/.claude/projects/` en el sistema de archivos local. Un adaptador `SessionStore` te permite reflejar esas transcripciones en tu propio backend, como S3, Redis o una base de datos, para que una sesión creada en un host pueda reanudarse en otro.

Razones comunes para usar un almacén de sesiones:

* **Implementaciones multi-host.** Las funciones sin servidor, los trabajadores con escalado automático y los ejecutores de CI no comparten un sistema de archivos. Un almacén compartido permite que cualquier réplica reanude cualquier sesión.
* **Durabilidad.** Los contenedores locales son efímeros. Un almacén respaldado por S3 o una base de datos sobrevive a reinicios y redeploys.
* **Cumplimiento y auditoría.** Mantén transcripciones en almacenamiento que ya gobiernas, con tus propias reglas de retención, cifrado y controles de acceso.

<h2 id="the-sessionstore-interface">
  La interfaz `SessionStore`
</h2>

Un `SessionStore` es un objeto con dos métodos requeridos, `append` y `load`, y tres métodos opcionales. El SDK llama a `append` para escribir entradas de transcripción durante una consulta y a `load` para leerlas de nuevo para reanudar.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry.

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
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  // SessionStore, SessionKey, SessionStoreEntry.

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
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...
  ```
</CodeGroup>

`SessionKey` direcciona una transcripción. `projectKey` es una codificación estable y segura para el sistema de archivos del directorio de trabajo, `sessionId` es el UUID de la sesión, y `subpath` se establece cuando la entrada pertenece a una transcripción de subagenteor archivo sidecar en lugar de la conversación principal. Trata `subpath` como una clave de sufijo opaca; sigue el diseño en disco, por ejemplo `subagents/agent-<id>`. Cuando `subpath` no está definido, la clave se refiere a la transcripción principal.

| Método         | Requerido | Se llama cuando                                                                                                                                                                                                                    |
| :------------- | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`       | Sí        | Después de que cada lote de entradas de transcripción se escriba localmente. Las entradas son objetos seguros para JSON, uno por línea en el JSONL local.                                                                          |
| `load`         | Sí        | Una vez antes de que se genere el subproceso, cuando `resume` está establecido. Devuelve `null` si la sesión es desconocida.                                                                                                       |
| `listSessions` | No        | Por `listSessions({ sessionStore })` y por `query()`/`startup()` con `continue: true`. Si no está definido, esas llamadas lanzan una excepción.                                                                                    |
| `delete`       | No        | Por `deleteSession({ sessionStore })`. Eliminar la clave principal (sin `subpath`) debe cascada a todas las subclaves para esa sesión. Si no está definido, la eliminación es una no-op, que se adapta a backends de solo anexión. |
| `listSubkeys`  | No        | Durante la reanudación, para descubrir transcripciones de subagenteor. Si no está definido, solo se restaura la transcripción principal.                                                                                           |

<h2 id="quick-start">
  Inicio rápido
</h2>

El SDK incluye un `InMemorySessionStore` para desarrollo y pruebas. El ejemplo a continuación ejecuta una consulta con el almacén adjunto, captura el ID de sesión del mensaje de resultado, luego reanuda desde el almacén en una segunda llamada `query()`. La segunda llamada pasa la misma instancia de almacén más `resume`, por lo que el SDK carga la transcripción del almacén en lugar del sistema de archivos local:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  for await (const message of query({
    prompt: "List the TypeScript files under src/",
    options: { sessionStore: store },
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
    }
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
      async for message in query(
          prompt="List the Python files under src/",
          options=ClaudeAgentOptions(session_store=store),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id

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

Implementa `append` y `load` contra tu backend. Añade `listSessions`, `delete` y `listSubkeys` si deseas que `listSessions()`, `deleteSession()` y la reanudación de subagenteor funcionen contra el almacén.

Las entradas pasadas a `append` se escriben como `SessionStoreEntry` (un objeto `{ type: string; ... }`). Trátalas como valores opacos seguros para JSON: persístalas en orden y devuélvelas desde `load` en el mismo orden. `load` debe devolver entradas que sean profundamente iguales a lo que se anexó; la serialización byte-igual no es requerida, por lo que backends como Postgres `jsonb` que reordenan claves de objeto están bien.

<h2 id="reference-implementations">
  Implementaciones de referencia
</h2>

El repositorio del SDK de TypeScript incluye adaptadores de referencia ejecutables para S3, Redis y Postgres bajo [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores). No se publican en npm; copia el archivo `src/` que necesites en tu proyecto e instala el cliente backend correspondiente.

| Adaptador                                                                                                                      | Cliente backend      | Modelo de almacenamiento                                                                |
| :----------------------------------------------------------------------------------------------------------------------------- | :------------------- | :-------------------------------------------------------------------------------------- |
| [`S3SessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3)             | `@aws-sdk/client-s3` | Un archivo de parte JSONL por `append()`; `load()` lista, ordena y concatena.           |
| [`RedisSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis)       | `ioredis`            | Lista `RPUSH`/`LRANGE` por transcripción, más un índice de conjunto ordenado de sesión. |
| [`PostgresSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres) | `pg`                 | Una fila por entrada en una tabla `jsonb`, ordenada por `BIGSERIAL`.                    |

Cada adaptador toma una instancia de cliente preconfigurada, por lo que controlas credenciales, TLS, región y agrupación. Por ejemplo, con S3:

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

En TypeScript, copia [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) del directorio de ejemplos en tu suite de pruebas. En Python, el conjunto se incluye en el paquete:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.asyncio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

<h2 id="behavior-notes">
  Notas de comportamiento
</h2>

<h3 id="dual-write-architecture">
  Arquitectura de escritura dual
</h3>

El almacén es un espejo, no un reemplazo. El subproceso de Claude Code siempre escribe primero en disco local; el SDK luego reenvía cada lote a `append()`. Si deseas que la copia local sea efímera, apunta `CLAUDE_CONFIG_DIR` a un directorio temporal en `options.env`. Debido a que el espejo depende de escrituras locales, `sessionStore` no se puede combinar con `persistSession: false`; el SDK lanza si estableces ambos. También lanza si se combina con `enableFileCheckpointing`, ya que los blobs de copia de seguridad del historial de archivos se escriben directamente en disco local y no se reflejan en el almacén.

<h3 id="mirror-writes-are-best-effort">
  Las escrituras de espejo son de mejor esfuerzo
</h3>

Si `append()` rechaza, el SDK reintenta el lote hasta dos veces más con un retroceso corto, para un máximo de tres intentos en total. Una llamada que agota el tiempo de espera no se reintenta, ya que la llamada original puede seguir llegando. Si el lote sigue fallando, el error se registra, se emite un mensaje `{ type: "system", subtype: "mirror_error" }` en el iterador, el lote se descarta y la consulta continúa. La transcripción local ya es duradera en disco, por lo que una interrupción del almacén no interrumpe al agente ni pierde datos localmente. Monitorea `mirror_error` si necesitas detectar pérdida de datos del almacén. Debido a que un lote reintentado puede reentrega entradas que ya llegaron, deduplica por `entry.uuid` en tu implementación de `append()`.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` devuelve la cadena posterior a la compactación
</h3>

`getSessionMessages({ sessionStore })` devuelve la cadena de mensajes vinculada que el agente vería al reanudar. Después de la compactación automática, los turnos anteriores se reemplazan por un resumen, por lo que una sesión cuyo almacén contiene 503 entradas sin procesar puede devolver 18 mensajes desde `getSessionMessages`. Para el historial sin procesar completo, incluidos los turnos previos a la compactación y las entradas de metadatos, llama a `store.load(key)` directamente.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` no es una copia byte
</h3>

`forkSession({ sessionStore })` lee las entradas de origen, reescribe cada campo `sessionId` y remapea UUIDs de mensajes, luego anexa las entradas transformadas bajo una clave nueva. Una copia a nivel de adaptador o un atajo `CopyObject` produciría una transcripción que aún hace referencia al ID de sesión anterior, por lo que el SDK no usa uno.

<h3 id="subagent-transcripts">
  Transcripciones de subagente
</h3>

Las transcripciones de subagente se reflejan bajo `subpath: "subagents/agent-<id>"`. `listSubagents({ sessionStore })` requiere que el adaptador implemente `listSubkeys`; `getSubagentMessages({ sessionStore })` lo usa cuando está disponible pero vuelve al subpath directo cuando no está definido. La reanudación también llama a `listSubkeys` para restaurar archivos de subagente; sin él, solo se materializa la transcripción principal.

<h3 id="retention">
  Retención
</h3>

El SDK nunca elimina de tu almacén por su cuenta. La retención es responsabilidad del adaptador: implementa TTLs, políticas de ciclo de vida de S3 o limpieza programada según tus requisitos de cumplimiento. Las transcripciones locales bajo `CLAUDE_CONFIG_DIR` se barren independientemente por la configuración `cleanupPeriodDays`.

<h2 id="supported-on">
  Compatible con
</h2>

Las siguientes funciones del SDK aceptan una opción `sessionStore` y operan contra el almacén en lugar del sistema de archivos local cuando se proporciona:

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

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Trabajar con sesiones](/docs/es/agent-sdk/sessions): Continúa, reanuda y bifurca sin un almacén personalizado
* [Alojar el SDK](/docs/es/agent-sdk/hosting): Patrones de implementación para entornos multi-host
* [TypeScript `Options`](/docs/es/agent-sdk/typescript#options): Referencia de opción completa
* [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores): Adaptadores de referencia ejecutables de S3, Redis y Postgres
