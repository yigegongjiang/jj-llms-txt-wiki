> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# API de sesión de TypeScript SDK V2 (eliminada)

> Referencia para la API de sesión eliminada V2 del SDK del Agente TypeScript, con patrones de envío/transmisión basados en sesiones para conversaciones de múltiples turnos.

<Warning>
  La API de sesión V2 ya no es compatible. TypeScript Agent SDK 0.3.142 elimina `unstable_v2_createSession`, `unstable_v2_resumeSession`, `unstable_v2_prompt` y los tipos `SDKSession` y `SDKSessionOptions`.

  Para migrar, use la [API `query()`](/docs/es/agent-sdk/typescript) y las [opciones de sesión](/docs/es/agent-sdk/sessions) que acepta. Pase un `AsyncIterable<SDKUserMessage>` para conversaciones de múltiples turnos, u `options.resume` para continuar una sesión guardada. Esta página se mantiene como referencia si mantiene código en Agent SDK 0.2.x o anterior.
</Warning>

V2 fue una API de sesión experimental que eliminó la necesidad de generadores asincronos y coordinación de rendimiento. En lugar de gestionar el estado del generador entre turnos, cada turno era un ciclo `send()`/`stream()` separado. La superficie de la API se redujo a tres conceptos:

* `createSession()` / `resumeSession()`: Iniciar o continuar una conversación
* `session.send()`: Enviar un mensaje
* `session.stream()`: Obtener la respuesta

<h2 id="installation">
  Instalación
</h2>

Agent SDK 0.2.x es la última versión que incluye la interfaz V2. La versión del paquete saltó de 0.2.x directamente a 0.3.142, por lo que la versión de eliminación anterior y el pin de instalación a continuación describen el mismo límite. Para instalar la última versión compatible con V2, fije la versión principal y secundaria:

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk@0.2
```

<Note>
  El SDK incluye un binario nativo de Claude Code para su plataforma como una dependencia opcional, por lo que no necesita instalar Claude Code por separado.
</Note>

<h2 id="quick-start">
  Inicio rápido
</h2>

<h3 id="one-shot-prompt">
  Solicitud de un solo turno
</h3>

Para consultas simples de un solo turno donde no necesita mantener una sesión, use `unstable_v2_prompt()`. Este ejemplo envía una pregunta matemática y registra la respuesta:

```typescript theme={null}
import { unstable_v2_prompt } from "@anthropic-ai/claude-agent-sdk";

const result = await unstable_v2_prompt("What is 2 + 2?", {
  model: "claude-opus-4-7"
});
if (result.subtype === "success") {
  console.log(result.result);
}
```

<details>
  <summary>Vea la misma operación en V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "What is 2 + 2?",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "result" && msg.subtype === "success") {
      console.log(msg.result);
    }
  }
  ```
</details>

<h3 id="basic-session">
  Sesión básica
</h3>

Para interacciones más allá de una solicitud única, cree una sesión. V2 separa el envío y la transmisión en pasos distintos:

* `send()` envía su mensaje
* `stream()` transmite la respuesta

Esta separación explícita facilita agregar lógica entre turnos (como procesar respuestas antes de enviar seguimientos).

El ejemplo a continuación crea una sesión, envía "¡Hola!" a Claude e imprime la respuesta de texto. Utiliza [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management) (TypeScript 5.2+) para cerrar automáticamente la sesión cuando el bloque sale. También puede llamar a `session.close()` manualmente.

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Hello!");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>Vea la misma operación en V1</summary>

  En V1, tanto la entrada como la salida fluyen a través de un único generador asincrónico. Para una solicitud básica, esto se ve similar, pero agregar lógica de múltiples turnos requiere reestructuración para usar un generador de entrada.

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "Hello!",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

<h3 id="multi-turn-conversation">
  Conversación de múltiples turnos
</h3>

Las sesiones persisten el contexto en múltiples intercambios. Para continuar una conversación, llame a `send()` nuevamente en la misma sesión. Claude recuerda los turnos anteriores.

Este ejemplo hace una pregunta matemática y luego hace un seguimiento que hace referencia a la respuesta anterior:

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

// Turn 1
await session.send("What is 5 + 3?");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}

// Turn 2
await session.send("Multiply that by 2");
for await (const msg of session.stream()) {
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>Vea la misma operación en V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Must create an async iterable to feed messages
  async function* createInputStream() {
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "What is 5 + 3?" }] },
      parent_tool_use_id: null
    };
    // Must coordinate when to yield next message
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "Multiply by 2" }] },
      parent_tool_use_id: null
    };
  }

  const q = query({
    prompt: createInputStream(),
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

<h3 id="session-resume">
  Reanudación de sesión
</h3>

Si tiene un ID de sesión de una interacción anterior, puede reanudarlo más tarde. Esto es útil para flujos de trabajo de larga duración o cuando necesita persistir conversaciones entre reinicios de aplicaciones.

Este ejemplo crea una sesión, almacena su ID, la cierra y luego reanuda la conversación:

```typescript theme={null}
import {
  unstable_v2_createSession,
  unstable_v2_resumeSession,
  type SDKMessage
} from "@anthropic-ai/claude-agent-sdk";

// Helper to extract text from assistant messages
function getAssistantText(msg: SDKMessage): string | null {
  if (msg.type !== "assistant") return null;
  return msg.message.content
    .filter((block) => block.type === "text")
    .map((block) => block.text)
    .join("");
}

// Create initial session and have a conversation
const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Remember this number: 42");

// Get the session ID from any received message
let sessionId: string | undefined;
for await (const msg of session.stream()) {
  sessionId = msg.session_id;
  const text = getAssistantText(msg);
  if (text) console.log("Initial response:", text);
}

console.log("Session ID:", sessionId);
session.close();

// Later: resume the session using the stored ID
await using resumedSession = unstable_v2_resumeSession(sessionId!, {
  model: "claude-opus-4-7"
});

await resumedSession.send("What number did I ask you to remember?");
for await (const msg of resumedSession.stream()) {
  const text = getAssistantText(msg);
  if (text) console.log("Resumed response:", text);
}
```

<details>
  <summary>Vea la misma operación en V1</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Create initial session
  const initialQuery = query({
    prompt: "Remember this number: 42",
    options: { model: "claude-opus-4-7" }
  });

  // Get session ID from any message
  let sessionId: string | undefined;
  for await (const msg of initialQuery) {
    sessionId = msg.session_id;
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Initial response:", text);
    }
  }

  console.log("Session ID:", sessionId);

  // Later: resume the session
  const resumedQuery = query({
    prompt: "What number did I ask you to remember?",
    options: {
      model: "claude-opus-4-7",
      resume: sessionId
    }
  });

  for await (const msg of resumedQuery) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Resumed response:", text);
    }
  }
  ```
</details>

<h3 id="cleanup">
  Limpieza
</h3>

Las sesiones se pueden cerrar manualmente o automáticamente usando [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management), una característica de TypeScript 5.2+ para la limpieza automática de recursos. Si está utilizando una versión anterior de TypeScript o encuentra problemas de compatibilidad, use la limpieza manual en su lugar.

**Limpieza automática (TypeScript 5.2+):**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// Session closes automatically when the block exits
```

**Limpieza manual:**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// ... use the session ...
session.close();
```

<h2 id="api-reference">
  Referencia de API
</h2>

<h3 id="unstable_v2_createsession">
  `unstable_v2_createSession()`
</h3>

Crea una nueva sesión para conversaciones de múltiples turnos.

```typescript theme={null}
function unstable_v2_createSession(options: {
  model: string;
  // Additional options supported
}): SDKSession;
```

<h3 id="unstable_v2_resumesession">
  `unstable_v2_resumeSession()`
</h3>

Reanuda una sesión existente por ID.

```typescript theme={null}
function unstable_v2_resumeSession(
  sessionId: string,
  options: {
    model: string;
    // Additional options supported
  }
): SDKSession;
```

<h3 id="unstable_v2_prompt">
  `unstable_v2_prompt()`
</h3>

Función de conveniencia de un solo turno para consultas de un solo turno.

```typescript theme={null}
function unstable_v2_prompt(
  prompt: string,
  options: {
    model: string;
    // Additional options supported
  }
): Promise<SDKResultMessage>;
```

<h3 id="sdksession-interface">
  Interfaz SDKSession
</h3>

```typescript theme={null}
interface SDKSession {
  readonly sessionId: string;
  send(message: string | SDKUserMessage): Promise<void>;
  stream(): AsyncGenerator<SDKMessage, void>;
  close(): void;
}
```

<h2 id="feature-availability">
  Disponibilidad de características
</h2>

La API de sesión V2 no admite todas las características de V1. Lo siguiente requiere el [SDK V1](/docs/es/agent-sdk/typescript):

* Bifurcación de sesiones (opción `forkSession`)
* Algunos patrones avanzados de entrada de transmisión

<h2 id="see-also">
  Véase también
</h2>

* [Referencia del SDK TypeScript (V1)](/docs/es/agent-sdk/typescript) - Documentación completa del SDK V1
* [Descripción general del SDK](/docs/es/agent-sdk/overview) - Conceptos generales del SDK
* [Ejemplos de V2 en GitHub](https://github.com/anthropics/claude-agent-sdk-demos/tree/main/hello-world-v2) - Ejemplos de código funcionales
