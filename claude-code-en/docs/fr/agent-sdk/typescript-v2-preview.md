> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# API de session TypeScript SDK V2 (supprimée)

> Référence pour l'API de session supprimée V2 du SDK Agent TypeScript, avec des modèles send/stream basés sur les sessions pour les conversations multi-tours.

<Warning>
  L'API de session V2 n'est plus supportée. TypeScript Agent SDK 0.3.142 supprime `unstable_v2_createSession`, `unstable_v2_resumeSession`, `unstable_v2_prompt`, et les types `SDKSession` et `SDKSessionOptions`.

  Pour migrer, utilisez l'[API `query()`](/docs/fr/agent-sdk/typescript) et les [options de session](/docs/fr/agent-sdk/sessions) qu'elle accepte. Passez un `AsyncIterable<SDKUserMessage>` pour les conversations multi-tours, ou `options.resume` pour continuer une session sauvegardée. Cette page est conservée à titre de référence si vous maintenez du code sur Agent SDK 0.2.x ou antérieur.
</Warning>

V2 était une API de session expérimentale qui supprimait le besoin de générateurs asynchrones et de coordination de rendement. Au lieu de gérer l'état du générateur entre les tours, chaque tour était un cycle `send()`/`stream()` séparé. La surface de l'API se réduisait à trois concepts :

* `createSession()` / `resumeSession()` : Démarrer ou continuer une conversation
* `session.send()` : Envoyer un message
* `session.stream()` : Obtenir la réponse

<h2 id="installation">
  Installation
</h2>

Agent SDK 0.2.x est la dernière version qui inclut l'interface V2. La version du package a sauté de 0.2.x directement à 0.3.142, donc la version de suppression ci-dessus et l'épingle d'installation ci-dessous décrivent la même limite. Pour installer la dernière version compatible avec V2, épinglez la version majeure et mineure :

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk@0.2
```

<Note>
  Le SDK regroupe un binaire Claude Code natif pour votre plateforme en tant que dépendance optionnelle, vous n'avez donc pas besoin d'installer Claude Code séparément.
</Note>

<h2 id="quick-start">
  Démarrage rapide
</h2>

<h3 id="one-shot-prompt">
  Invite unique
</h3>

Pour les requêtes simples à un seul tour où vous n'avez pas besoin de maintenir une session, utilisez `unstable_v2_prompt()`. Cet exemple envoie une question mathématique et enregistre la réponse :

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
  <summary>Voir la même opération en V1</summary>

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
  Session de base
</h3>

Pour les interactions au-delà d'une seule invite, créez une session. V2 sépare l'envoi et la diffusion en étapes distinctes :

* `send()` envoie votre message
* `stream()` diffuse la réponse

Cette séparation explicite facilite l'ajout de logique entre les tours (comme le traitement des réponses avant d'envoyer des suites).

L'exemple ci-dessous crée une session, envoie « Hello ! » à Claude et imprime la réponse textuelle. Il utilise [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management) (TypeScript 5.2+) pour fermer automatiquement la session lorsque le bloc se termine. Vous pouvez également appeler `session.close()` manuellement.

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
  <summary>Voir la même opération en V1</summary>

  En V1, l'entrée et la sortie circulent toutes les deux via un seul générateur asynchrone. Pour une invite de base, cela ressemble à quelque chose de similaire, mais l'ajout de logique multi-tours nécessite une restructuration pour utiliser un générateur d'entrée.

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
  Conversation multi-tours
</h3>

Les sessions persistent le contexte à travers plusieurs échanges. Pour continuer une conversation, appelez `send()` à nouveau sur la même session. Claude se souvient des tours précédents.

Cet exemple pose une question mathématique, puis pose une suite qui fait référence à la réponse précédente :

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
  <summary>Voir la même opération en V1</summary>

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
  Reprise de session
</h3>

Si vous avez un ID de session d'une interaction précédente, vous pouvez le reprendre plus tard. Ceci est utile pour les flux de travail de longue durée ou lorsque vous devez persister les conversations entre les redémarrages d'application.

Cet exemple crée une session, stocke son ID, la ferme, puis reprend la conversation :

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
  <summary>Voir la même opération en V1</summary>

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
  Nettoyage
</h3>

Les sessions peuvent être fermées manuellement ou automatiquement en utilisant [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management), une fonctionnalité TypeScript 5.2+ pour le nettoyage automatique des ressources. Si vous utilisez une version TypeScript plus ancienne ou rencontrez des problèmes de compatibilité, utilisez plutôt le nettoyage manuel.

**Nettoyage automatique (TypeScript 5.2+) :**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// Session closes automatically when the block exits
```

**Nettoyage manuel :**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// ... use the session ...
session.close();
```

<h2 id="api-reference">
  Référence API
</h2>

<h3 id="unstable_v2_createsession">
  `unstable_v2_createSession()`
</h3>

Crée une nouvelle session pour les conversations multi-tours.

```typescript theme={null}
function unstable_v2_createSession(options: {
  model: string;
  // Additional options supported
}): SDKSession;
```

<h3 id="unstable_v2_resumesession">
  `unstable_v2_resumeSession()`
</h3>

Reprend une session existante par ID.

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

Fonction de commodité unique pour les requêtes à un seul tour.

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
  Interface SDKSession
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
  Disponibilité des fonctionnalités
</h2>

L'API de session V2 ne supporte pas toutes les fonctionnalités V1. Les éléments suivants nécessitent l'utilisation du [SDK V1](/docs/fr/agent-sdk/typescript) :

* Forking de session (option `forkSession`)
* Certains modèles de flux d'entrée avancés

<h2 id="see-also">
  Voir aussi
</h2>

* [Référence SDK TypeScript (V1)](/docs/fr/agent-sdk/typescript) - Documentation complète du SDK V1
* [Aperçu SDK](/docs/fr/agent-sdk/overview) - Concepts généraux du SDK
* [Exemples V2 sur GitHub](https://github.com/anthropics/claude-agent-sdk-demos/tree/main/hello-world-v2) - Exemples de code fonctionnels
