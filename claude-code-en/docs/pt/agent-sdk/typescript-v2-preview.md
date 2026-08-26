> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# API de sessão TypeScript SDK V2 (removida)

> Referência para a API de sessão removida V2 do SDK do Agent TypeScript, com padrões de envio/stream baseados em sessão para conversas multi-turno.

<Warning>
  A API de sessão V2 não é mais suportada. TypeScript Agent SDK 0.3.142 remove `unstable_v2_createSession`, `unstable_v2_resumeSession`, `unstable_v2_prompt` e os tipos `SDKSession` e `SDKSessionOptions`.

  Para migrar, use a [API `query()`](/docs/pt/agent-sdk/typescript) e as [opções de sessão](/docs/pt/agent-sdk/sessions) que ela aceita. Passe um `AsyncIterable<SDKUserMessage>` para conversas multi-turno, ou `options.resume` para continuar uma sessão salva. Esta página é mantida como referência se você mantém código no Agent SDK 0.2.x ou anterior.
</Warning>

V2 era uma API de sessão experimental que removeu a necessidade de geradores assíncronos e coordenação de yield. Em vez de gerenciar o estado do gerador entre turnos, cada turno era um ciclo `send()`/`stream()` separado. A superfície da API se reduzia a três conceitos:

* `createSession()` / `resumeSession()`: Iniciar ou continuar uma conversa
* `session.send()`: Enviar uma mensagem
* `session.stream()`: Obter a resposta

<h2 id="installation">
  Instalação
</h2>

Agent SDK 0.2.x é a última versão que inclui a interface V2. A versão do pacote saltou de 0.2.x diretamente para 0.3.142, portanto a versão de remoção acima e o pin de instalação abaixo descrevem o mesmo limite. Para instalar a última versão compatível com V2, fixe a versão principal e secundária:

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk@0.2
```

<Note>
  O SDK agrupa um binário nativo do Claude Code para sua plataforma como uma dependência opcional, portanto você não precisa instalar o Claude Code separadamente.
</Note>

<h2 id="quick-start">
  Início rápido
</h2>

<h3 id="one-shot-prompt">
  Prompt único
</h3>

Para consultas simples de turno único onde você não precisa manter uma sessão, use `unstable_v2_prompt()`. Este exemplo envia uma pergunta de matemática e registra a resposta:

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
  <summary>Veja a mesma operação em V1</summary>

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
  Sessão básica
</h3>

Para interações além de um único prompt, crie uma sessão. V2 separa envio e streaming em etapas distintas:

* `send()` envia sua mensagem
* `stream()` transmite a resposta

Esta separação explícita torna mais fácil adicionar lógica entre turnos (como processar respostas antes de enviar acompanhamentos).

O exemplo abaixo cria uma sessão, envia "Hello!" para Claude e imprime a resposta de texto. Ele usa [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management) (TypeScript 5.2+) para fechar automaticamente a sessão quando o bloco sai. Você também pode chamar `session.close()` manualmente.

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
  <summary>Veja a mesma operação em V1</summary>

  Em V1, tanto entrada quanto saída fluem através de um único gerador assíncrono. Para um prompt básico, isso parece semelhante, mas adicionar lógica multi-turno requer reestruturação para usar um gerador de entrada.

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
  Conversa multi-turno
</h3>

As sessões persistem contexto em múltiplas trocas. Para continuar uma conversa, chame `send()` novamente na mesma sessão. Claude se lembra dos turnos anteriores.

Este exemplo faz uma pergunta de matemática e depois faz um acompanhamento que referencia a resposta anterior:

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
  <summary>Veja a mesma operação em V1</summary>

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
  Retomada de sessão
</h3>

Se você tiver um ID de sessão de uma interação anterior, poderá retomá-lo mais tarde. Isso é útil para fluxos de trabalho de longa duração ou quando você precisa persistir conversas entre reinicializações de aplicativo.

Este exemplo cria uma sessão, armazena seu ID, a fecha e depois retoma a conversa:

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
  <summary>Veja a mesma operação em V1</summary>

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
  Limpeza
</h3>

As sessões podem ser fechadas manualmente ou automaticamente usando [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management), um recurso do TypeScript 5.2+ para limpeza automática de recursos. Se você estiver usando uma versão mais antiga do TypeScript ou encontrar problemas de compatibilidade, use limpeza manual em seu lugar.

**Limpeza automática (TypeScript 5.2+):**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// Session closes automatically when the block exits
```

**Limpeza manual:**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// ... use the session ...
session.close();
```

<h2 id="api-reference">
  Referência da API
</h2>

<h3 id="unstable_v2_createsession">
  `unstable_v2_createSession()`
</h3>

Cria uma nova sessão para conversas multi-turno.

```typescript theme={null}
function unstable_v2_createSession(options: {
  model: string;
  // Additional options supported
}): SDKSession;
```

<h3 id="unstable_v2_resumesession">
  `unstable_v2_resumeSession()`
</h3>

Retoma uma sessão existente por ID.

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

Função de conveniência única para consultas de turno único.

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
  Disponibilidade de recursos
</h2>

A API de sessão V2 não suporta todos os recursos V1. Os seguintes requerem o [SDK V1](/docs/pt/agent-sdk/typescript):

* Bifurcação de sessão (opção `forkSession`)
* Alguns padrões avançados de entrada de streaming

<h2 id="see-also">
  Veja também
</h2>

* [Referência do SDK TypeScript (V1)](/docs/pt/agent-sdk/typescript) - Documentação completa do SDK V1
* [Visão geral do SDK](/docs/pt/agent-sdk/overview) - Conceitos gerais do SDK
* [Exemplos V2 no GitHub](https://github.com/anthropics/claude-agent-sdk-demos/tree/main/hello-world-v2) - Exemplos de código funcionando
