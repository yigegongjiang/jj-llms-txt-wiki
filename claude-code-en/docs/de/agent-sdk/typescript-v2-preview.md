> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# TypeScript SDK V2 Sitzungs-API (entfernt)

> Referenz für die entfernte V2 TypeScript Agent SDK Sitzungs-API mit sitzungsbasiertem Send/Stream-Muster für mehrteilige Gespräche.

<Warning>
  Die V2 Sitzungs-API wird nicht mehr unterstützt. TypeScript Agent SDK 0.3.142 entfernt `unstable_v2_createSession`, `unstable_v2_resumeSession`, `unstable_v2_prompt` und die Typen `SDKSession` und `SDKSessionOptions`.

  Verwenden Sie zur Migration die [`query()` API](/docs/de/agent-sdk/typescript) und die [Sitzungsoptionen](/docs/de/agent-sdk/sessions), die sie akzeptiert. Übergeben Sie ein `AsyncIterable<SDKUserMessage>` für mehrteilige Gespräche oder `options.resume`, um eine gespeicherte Sitzung fortzusetzen. Diese Seite wird als Referenz beibehalten, wenn Sie Code auf Agent SDK 0.2.x oder früher verwalten.
</Warning>

V2 war eine experimentelle Sitzungs-API, die die Notwendigkeit für asynchrone Generatoren und Yield-Koordination entfernte. Anstatt den Generator-Status über Turns hinweg zu verwalten, war jeder Turn ein separater `send()`/`stream()`-Zyklus. Die API-Oberfläche reduzierte sich auf drei Konzepte:

* `createSession()` / `resumeSession()`: Ein Gespräch starten oder fortsetzen
* `session.send()`: Eine Nachricht senden
* `session.stream()`: Die Antwort abrufen

<h2 id="installation">
  Installation
</h2>

Agent SDK 0.2.x ist die letzte Version, die die V2-Schnittstelle enthält. Die Paketversion sprang von 0.2.x direkt zu 0.3.142, sodass die oben genannte Entfernungsversion und die Installationsfestlegung unten dieselbe Grenze beschreiben. Um die letzte V2-kompatible Version zu installieren, legen Sie die Hauptversion und Nebenversion fest:

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk@0.2
```

<Note>
  Das SDK bündelt eine native Claude Code-Binärdatei für Ihre Plattform als optionale Abhängigkeit, sodass Sie Claude Code nicht separat installieren müssen.
</Note>

<h2 id="quick-start">
  Schnellstart
</h2>

<h3 id="one-shot-prompt">
  Einmalige Anfrage
</h3>

Für einfache Einzelturn-Abfragen, bei denen Sie keine Sitzung beibehalten müssen, verwenden Sie `unstable_v2_prompt()`. Dieses Beispiel sendet eine Mathefrage und protokolliert die Antwort:

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
  <summary>Siehe denselben Vorgang in V1</summary>

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
  Grundlegende Sitzung
</h3>

Für Interaktionen über eine einzelne Anfrage hinaus erstellen Sie eine Sitzung. V2 trennt das Senden und Streamen in unterschiedliche Schritte:

* `send()` sendet Ihre Nachricht
* `stream()` streamt die Antwort zurück

Diese explizite Trennung macht es einfacher, Logik zwischen Turns hinzuzufügen (wie das Verarbeiten von Antworten vor dem Senden von Folgefragen).

Das folgende Beispiel erstellt eine Sitzung, sendet „Hello!" an Claude und gibt die Textantwort aus. Es verwendet [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management) (TypeScript 5.2+), um die Sitzung automatisch zu schließen, wenn der Block beendet wird. Sie können auch `session.close()` manuell aufrufen.

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
  <summary>Siehe denselben Vorgang in V1</summary>

  In V1 fließen sowohl Ein- als auch Ausgabe durch einen einzelnen asynchronen Generator. Für eine grundlegende Anfrage sieht dies ähnlich aus, aber das Hinzufügen von mehrteiliger Logik erfordert eine Umstrukturierung zur Verwendung eines Eingabegenerators.

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
  Mehrteiliges Gespräch
</h3>

Sitzungen behalten den Kontext über mehrere Austausche hinweg bei. Um ein Gespräch fortzusetzen, rufen Sie `send()` erneut in derselben Sitzung auf. Claude merkt sich die vorherigen Turns.

Dieses Beispiel stellt eine Mathefrage und stellt dann eine Folgefrage, die sich auf die vorherige Antwort bezieht:

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
  <summary>Siehe denselben Vorgang in V1</summary>

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
  Sitzung fortsetzen
</h3>

Wenn Sie eine Sitzungs-ID aus einer vorherigen Interaktion haben, können Sie diese später fortsetzen. Dies ist nützlich für langfristige Workflows oder wenn Sie Gespräche über Anwendungsneustarts hinweg beibehalten müssen.

Dieses Beispiel erstellt eine Sitzung, speichert ihre ID, schließt sie und setzt das Gespräch dann fort:

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
  <summary>Siehe denselben Vorgang in V1</summary>

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
  Bereinigung
</h3>

Sitzungen können manuell oder automatisch mit [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management) geschlossen werden, einer TypeScript 5.2+-Funktion für automatische Ressourcenbereinigung. Wenn Sie eine ältere TypeScript-Version verwenden oder auf Kompatibilitätsprobleme stoßen, verwenden Sie stattdessen manuelle Bereinigung.

**Automatische Bereinigung (TypeScript 5.2+):**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// Session closes automatically when the block exits
```

**Manuelle Bereinigung:**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// ... use the session ...
session.close();
```

<h2 id="api-reference">
  API-Referenz
</h2>

<h3 id="unstable_v2_createsession">
  `unstable_v2_createSession()`
</h3>

Erstellt eine neue Sitzung für mehrteilige Gespräche.

```typescript theme={null}
function unstable_v2_createSession(options: {
  model: string;
  // Additional options supported
}): SDKSession;
```

<h3 id="unstable_v2_resumesession">
  `unstable_v2_resumeSession()`
</h3>

Setzt eine vorhandene Sitzung nach ID fort.

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

Einmalige Komfortfunktion für Einzelturn-Abfragen.

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
  SDKSession-Schnittstelle
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
  Funktionsverfügbarkeit
</h2>

Die V2 Sitzungs-API unterstützt nicht alle V1-Funktionen. Die folgenden erfordern das [V1 SDK](/docs/de/agent-sdk/typescript):

* Sitzungs-Forking (`forkSession`-Option)
* Einige erweiterte Streaming-Eingabemuster

<h2 id="see-also">
  Siehe auch
</h2>

* [TypeScript SDK-Referenz (V1)](/docs/de/agent-sdk/typescript) - Vollständige V1 SDK-Dokumentation
* [SDK-Übersicht](/docs/de/agent-sdk/overview) - Allgemeine SDK-Konzepte
* [V2-Beispiele auf GitHub](https://github.com/anthropics/claude-agent-sdk-demos/tree/main/hello-world-v2) - Funktionierende Code-Beispiele
