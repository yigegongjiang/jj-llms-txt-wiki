> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Persistir sessões em armazenamento externo

> Espelhe transcrições de sessão para S3, Redis ou seu próprio backend para que qualquer host possa retomá-las.

Por padrão, o SDK escreve transcrições de sessão em arquivos JSONL em `~/.claude/projects/` no sistema de arquivos local. Um adaptador `SessionStore` permite que você espelhe essas transcrições para seu próprio backend, como S3, Redis ou um banco de dados, para que uma sessão criada em um host possa ser retomada em outro.

Razões comuns para usar um session store:

* **Implantações multi-host.** Funções serverless, workers com autoscaling e runners de CI não compartilham um sistema de arquivos. Um store compartilhado permite que qualquer réplica retome qualquer sessão.
* **Durabilidade.** Contêineres locais são efêmeros. Um store apoiado por S3 ou um banco de dados sobrevive a reinicializações e redeploys.
* **Conformidade e auditoria.** Mantenha transcrições em armazenamento que você já governa, com suas próprias regras de retenção, criptografia e controles de acesso.

<h2 id="the-sessionstore-interface">
  A interface `SessionStore`
</h2>

Um `SessionStore` é um objeto com dois métodos obrigatórios, `append` e `load`, e três métodos opcionais. O SDK chama `append` para escrever entradas de transcrição durante uma consulta e `load` para lê-las novamente para retomada.

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
  # SessionStore, SessionKey, SessionStoreEntry.

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

`SessionKey` endereça uma transcrição. `projectKey` é uma codificação estável e segura para sistema de arquivos do diretório de trabalho, `sessionId` é o UUID da sessão, e `subpath` é definido quando a entrada pertence a uma transcrição de subagente ou arquivo sidecar em vez da conversa principal. Trate `subpath` como um sufixo de chave opaco; ele segue o layout em disco, por exemplo `subagents/agent-<id>`. Quando `subpath` é indefinido, a chave se refere à transcrição principal.

| Método         | Obrigatório | Chamado quando                                                                                                                                                                                                           |
| :------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`       | Sim         | Após cada lote de entradas de transcrição ser escrito localmente. As entradas são objetos seguros para JSON, um por linha no JSONL local.                                                                                |
| `load`         | Sim         | Uma vez antes do subprocess ser gerado, quando `resume` é definido. Retorne `null` se a sessão for desconhecida.                                                                                                         |
| `listSessions` | Não         | Por `listSessions({ sessionStore })` e por `query()`/`startup()` com `continue: true`. Se indefinido, essas chamadas lançam.                                                                                             |
| `delete`       | Não         | Por `deleteSession({ sessionStore })`. Deletar a chave principal (sem `subpath`) deve cascatear para todas as subchaves dessa sessão. Se indefinido, a exclusão é uma no-op, o que é adequado para backends append-only. |
| `listSubkeys`  | Não         | Durante retomada, para descobrir transcrições de subagentes. Se indefinido, apenas a transcrição principal é restaurada.                                                                                                 |

<h2 id="quick-start">
  Início rápido
</h2>

O SDK fornece um `InMemorySessionStore` para desenvolvimento e testes. O exemplo abaixo executa uma consulta com o store anexado, captura o ID da sessão da mensagem de resultado e depois retoma do store em uma segunda chamada `query()`. A segunda chamada passa a mesma instância de store mais `resume`, para que o SDK carregue a transcrição do store em vez do sistema de arquivos local:

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

A segunda consulta imprime um resumo dos arquivos da primeira consulta, o que mostra que o agente retomou com contexto completo do store.

<h2 id="write-your-own-adapter">
  Escreva seu próprio adaptador
</h2>

Implemente `append` e `load` contra seu backend. Adicione `listSessions`, `delete` e `listSubkeys` se você quiser que `listSessions()`, `deleteSession()` e retomada de subagentes funcionem contra o store.

As entradas passadas para `append` são digitadas como `SessionStoreEntry` (um objeto `{ type: string; ... }`). Trate-as como valores JSON-safe opacos: persista-as em ordem e retorne-as de `load` na mesma ordem. `load` deve retornar entradas que sejam deep-equal ao que foi anexado; serialização byte-equal não é necessária, então backends como Postgres `jsonb` que reordenam chaves de objeto são adequados.

<h2 id="reference-implementations">
  Implementações de referência
</h2>

O repositório do SDK TypeScript inclui adaptadores de referência executáveis para S3, Redis e Postgres em [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores). Eles não são publicados no npm; copie o arquivo `src/` que você precisa para seu projeto e instale o cliente backend correspondente.

| Adaptador                                                                                                                      | Cliente backend      | Modelo de armazenamento                                                                |
| :----------------------------------------------------------------------------------------------------------------------------- | :------------------- | :------------------------------------------------------------------------------------- |
| [`S3SessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3)             | `@aws-sdk/client-s3` | Um arquivo de parte JSONL por `append()`; `load()` lista, ordena e concatena.          |
| [`RedisSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis)       | `ioredis`            | Lista `RPUSH`/`LRANGE` por transcrição, mais um índice de conjunto ordenado de sessão. |
| [`PostgresSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres) | `pg`                 | Uma linha por entrada em uma tabela `jsonb`, ordenada por `BIGSERIAL`.                 |

Cada adaptador recebe uma instância de cliente pré-configurada, para que você controle credenciais, TLS, região e pooling. Por exemplo, com S3:

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
  Valide seu adaptador
</h3>

Ambos os SDKs fornecem um conjunto de conformidade que afirma o contrato comportamental que `append`, `load` e os métodos opcionais devem satisfazer. Testes para métodos opcionais pulam automaticamente quando esses métodos não são implementados.

Em TypeScript, copie [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) do diretório de exemplos para seu conjunto de testes. Em Python, o conjunto é fornecido no pacote:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.asyncio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

<h2 id="behavior-notes">
  Notas de comportamento
</h2>

<h3 id="dual-write-architecture">
  Arquitetura de escrita dupla
</h3>

O store é um espelho, não uma substituição. O subprocess Claude Code sempre escreve no disco local primeiro; o SDK então encaminha cada lote para `append()`. Se você quiser que a cópia local seja efêmera, aponte `CLAUDE_CONFIG_DIR` para um diretório temporário em `options.env`. Como o espelho depende de escritas locais, `sessionStore` não pode ser combinado com `persistSession: false`; o SDK lança se você definir ambos. Também lança se combinado com `enableFileCheckpointing`, já que blobs de backup de histórico de arquivo são escritos diretamente no disco local e não são espelhados para o store.

<h3 id="mirror-writes-are-best-effort">
  Escritas de espelho são best-effort
</h3>

Se `append()` rejeita, o SDK tenta novamente o lote até mais duas vezes com um backoff curto, para no máximo três tentativas no total. Uma chamada que expira não é retentada, já que a chamada original ainda pode chegar. Se o lote ainda falhar, o erro é registrado, uma mensagem `{ type: "system", subtype: "mirror_error" }` é emitida para o iterador, o lote é descartado, e a consulta continua. A transcrição local já é durável no disco, então uma interrupção de store não interrompe o agente ou perde dados localmente. Monitore `mirror_error` se você precisar detectar perda de dados de store. Como um lote retentado pode reentrega entradas que já chegaram, deduplicar por `entry.uuid` em sua implementação de `append()`.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` retorna a cadeia pós-compactação
</h3>

`getSessionMessages({ sessionStore })` retorna a cadeia de mensagens vinculada que o agente veria ao retomar. Após auto-compactação, turnos anteriores são substituídos por um resumo, então uma sessão cujo store contém 503 entradas brutas pode retornar 18 mensagens de `getSessionMessages`. Para o histórico bruto completo, incluindo turnos pré-compactação e entradas de metadados, chame `store.load(key)` diretamente.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` não é uma cópia byte
</h3>

`forkSession({ sessionStore })` lê as entradas de origem, reescreve cada campo `sessionId` e remapeia UUIDs de mensagem, depois anexa as entradas transformadas sob uma nova chave. Uma cópia em nível de adaptador ou atalho `CopyObject` produziria uma transcrição que ainda referencia o ID de sessão antigo, então o SDK não usa um.

<h3 id="subagent-transcripts">
  Transcrições de subagentes
</h3>

Transcrições de subagentes são espelhadas em `subpath: "subagents/agent-<id>"`. `listSubagents({ sessionStore })` requer que o adaptador implemente `listSubkeys`; `getSubagentMessages({ sessionStore })` o usa quando disponível, mas volta para o subpath direto quando é indefinido. Retomada também chama `listSubkeys` para restaurar arquivos de subagentes; sem ele, apenas a transcrição principal é materializada.

<h3 id="retention">
  Retenção
</h3>

O SDK nunca deleta de seu store por conta própria. Retenção é responsabilidade do adaptador: implemente TTLs, políticas de ciclo de vida S3 ou limpeza agendada de acordo com seus requisitos de conformidade. Transcrições locais em `CLAUDE_CONFIG_DIR` são limpas independentemente pela configuração `cleanupPeriodDays`.

<h2 id="supported-on">
  Suportado em
</h2>

As seguintes funções do SDK aceitam uma opção `sessionStore` e operam contra o store em vez do sistema de arquivos local quando fornecido:

* [`query()`](/docs/pt/agent-sdk/typescript#query)
* [`startup()`](/docs/pt/agent-sdk/typescript#startup)
* [`listSessions()`](/docs/pt/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](/docs/pt/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](/docs/pt/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](/docs/pt/agent-sdk/typescript#renamesession)
* [`tagSession()`](/docs/pt/agent-sdk/typescript#tagsession)
* [`deleteSession()`](/docs/pt/agent-sdk/typescript)
* [`forkSession()`](/docs/pt/agent-sdk/typescript)
* [`listSubagents()`](/docs/pt/agent-sdk/typescript)
* [`getSubagentMessages()`](/docs/pt/agent-sdk/typescript)

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Trabalhar com sessões](/docs/pt/agent-sdk/sessions): Continuar, retomar e fazer fork sem um store personalizado
* [Hospedar o SDK](/docs/pt/agent-sdk/hosting): Padrões de implantação para ambientes multi-host
* [TypeScript `Options`](/docs/pt/agent-sdk/typescript#options): Referência completa de opções
* [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores): Adaptadores de referência executáveis para S3, Redis e Postgres
