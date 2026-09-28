> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Persistir sessões em armazenamento externo

> Espelhe transcrições de sessão do Agent SDK para seu próprio armazenamento de objetos, armazenamento de chave-valor ou banco de dados para que outros hosts possam retomar suas sessões.

Por padrão, o SDK escreve transcrições de sessão em arquivos JSONL em `~/.claude/projects/` no sistema de arquivos local. Um adaptador `SessionStore` permite que você espelhe essas transcrições para seu próprio backend, como um armazenamento de objetos, um armazenamento de chave-valor ou um banco de dados, para que uma sessão criada em um host possa ser retomada em outro host executando a partir de um diretório de trabalho correspondente.

Razões comuns para usar um session store:

* **Implantações multi-host.** Funções serverless, workers com autoscaling e runners de CI não compartilham um sistema de arquivos. Um store compartilhado permite que réplicas retomem as sessões umas das outras.
* **Durabilidade.** Contêineres locais são efêmeros. Um store externo sobrevive a reinicializações e redeploys.
* **Conformidade e auditoria.** Mantenha transcrições em armazenamento que você já governa, com suas próprias regras de retenção, criptografia e controles de acesso.

<h2 id="the-sessionstore-interface">
  A interface `SessionStore`
</h2>

Um `SessionStore` é um objeto com dois métodos obrigatórios, `append` e `load`, e quatro métodos opcionais. O SDK chama `append` para escrever entradas de transcrição durante uma consulta e `load` para lê-las novamente para retomada.

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

`SessionKey` endereça uma transcrição. `projectKey` é uma codificação estável e segura para sistema de arquivos do diretório de trabalho, `sessionId` é o UUID da sessão, e `subpath` é definido quando a entrada pertence a uma transcrição de subagente ou arquivo sidecar em vez da conversa principal.

Como `projectKey` codifica o diretório de trabalho, retome ou continue a partir do armazenamento de um diretório de trabalho que corresponda ao da execução original. No TypeScript, se você definir [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/pt/sessions#name-the-project-directory-yourself) ao lado de `CLAUDE_CONFIG_DIR` na opção [`env`](/docs/pt/agent-sdk/typescript#options) de uma consulta, o SDK codifica as entradas dessa consulta, e suas buscas de `resume` e `continue`, por esse nome. Como auxiliares independentes como `listSessions` e `deleteSession` não recebem `env` e leem o ambiente do processo, defina `CLAUDE_CONFIG_DIR` e o mesmo nome no ambiente do processo host também. Requer Agent SDK v0.3.234 ou posterior.

Trate `subpath` como um sufixo de chave opaco; ele segue o layout em disco, por exemplo `subagents/agent-<id>`. Quando `subpath` é indefinido, a chave se refere à transcrição principal.

| Método                 | Obrigatório | Chamado quando                                                                                                                                                                                                                                                                                                                                   |
| :--------------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`               | Sim         | Após cada lote de entradas de transcrição ser escrito localmente. As entradas são objetos seguros para JSON, um por linha no JSONL local.                                                                                                                                                                                                        |
| `load`                 | Sim         | Antes do subprocess ser gerado quando `resume` é definido ou `continue: true` resolve a sessão de armazenamento mais recente, e uma vez por sessão ao listar retorna de `listSessionSummaries`. Retorne `null` se a sessão for desconhecida.                                                                                                     |
| `listSessions`         | Não         | Por `listSessions({ sessionStore })` e por `query()`/`startup()` com `continue: true`. Se indefinido, `continue: true` lança, e `listSessions({ sessionStore })` lança a menos que `listSessionSummaries` seja implementado.                                                                                                                     |
| `listSessionSummaries` | Não         | Por `listSessions({ sessionStore })` para ler metadados de todas as sessões em uma única chamada. Mantenha os resumos dentro de `append`. Se indefinido, a listagem retorna para `listSessions` mais um `load` por sessão.                                                                                                                       |
| `delete`               | Não         | Por `deleteSession({ sessionStore })`. Deletar a chave principal (sem `subpath`) deve cascatear para todas as subchaves dessa sessão e também remover a entrada de resumo da sessão, para que uma sessão deletada pare de aparecer em `listSessionSummaries`. Se indefinido, a exclusão é uma no-op, o que é adequado para backends append-only. |
| `listSubkeys`          | Não         | Durante retomada, para descobrir transcrições de subagentes. Se indefinido, apenas a transcrição principal é restaurada.                                                                                                                                                                                                                         |

Em um `SessionSummaryEntry`, `mtime` é o tempo de escrita do armazenamento do sidecar e deve compartilhar uma fonte de relógio com os valores `mtime` que `listSessions` retorna. `data` é um estado opaco de propriedade do SDK; persista-o verbatim sem interpretá-lo.

Construa as entradas chamando o auxiliar exportado `foldSessionSummary`, `fold_session_summary` em Python, em cada lote dentro de `append`. Pule lotes cuja chave tem um `subpath`; transcrições de subagentes não devem contribuir para o resumo da sessão principal. O fold nunca define `mtime`: carimbe-o no tempo de persistência, através do argumento `options.mtime` no TypeScript ou sobrescrevendo o campo na entrada retornada em Python. Chamadas `append` concorrentes para a mesma sessão podem correr no sidecar, então serialize a leitura-fold-escrita com uma transação, um compare-and-swap, ou um lock por sessão; o fold em si é puro.

Para o que o SDK faz com a transcrição que `load` retorna, veja [Retomar do armazenamento](#resume-from-the-store).

<h2 id="quick-start">
  Início rápido
</h2>

O SDK fornece um `InMemorySessionStore` para desenvolvimento e testes. O exemplo abaixo executa uma consulta com o store anexado, captura o ID da sessão da mensagem de resultado e depois retoma do store em uma segunda chamada `query()`. A segunda chamada passa a mesma instância de store mais `resume`, para que o SDK carregue a transcrição do store em vez do sistema de arquivos local:

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

A segunda consulta imprime um resumo dos arquivos da primeira consulta, o que mostra que o agente retomou com contexto completo do store.

<h2 id="write-your-own-adapter">
  Escreva seu próprio adaptador
</h2>

Implemente `append` e `load` contra seu backend. Adicione `listSessions`, `listSessionSummaries`, `delete` e `listSubkeys` se você quiser que `listSessions()`, leituras de metadados em uma única chamada, `deleteSession()` e retomada de subagentes funcionem contra o store.

As entradas passadas para `append` são digitadas como `SessionStoreEntry` (um objeto `{ type: string; ... }`). Trate-as como valores JSON-safe opacos: persista-as em ordem e retorne-as de `load` na mesma ordem. `load` deve retornar entradas que sejam deep-equal ao que foi anexado; serialização byte-equal não é necessária, então um backend que reordena chaves de objeto, como um tipo de coluna JSON binária, é adequado.

<h2 id="reference-implementations">
  Implementações de referência
</h2>

Ambos os repositórios do SDK incluem adaptadores de referência executáveis em [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) em TypeScript e [`examples/session_stores/`](https://github.com/anthropics/claude-agent-sdk-python/tree/main/examples/session_stores) em Python. Há um adaptador por tipo de armazenamento, e cada um mostra como `append` e `load` mapeiam para esse tipo de backend. Eles não são publicados como pacotes; copie o adaptador para o tipo mais próximo do seu backend para seu projeto, instale o cliente do seu backend e adapte-o.

| Tipo de armazenamento                                    | Modelo de armazenamento                                                                                        | Adaptador de exemplo                                                                                                                                                                                                                                       |
| :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Armazenamento de objetos                                 | Um arquivo de parte por `append()`; `load()` lista as partes, as ordena e as concatena.                        | S3 ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/s3_session_store.py))                   |
| Armazenamento de chave-valor                             | Uma lista por transcrição que `append()` envia e `load()` lê em intervalo, mais um índice ordenado de sessões. | Redis ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/redis_session_store.py))          |
| Banco de dados relacional ou armazenamento de documentos | Uma linha ou documento por entrada, armazenado como JSON e ordenado por uma chave atribuída na inserção.       | Postgres ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/postgres_session_store.py)) |

Cada adaptador recebe uma instância de cliente pré-configurada, para que você controle credenciais, TLS, região e pooling. O exemplo a seguir conecta o adaptador de armazenamento de objetos em `query()` e depois retoma a partir dele em outro host:

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

Em TypeScript, copie [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) do diretório de exemplos para seu conjunto de testes. Em Python, o conjunto é fornecido no pacote. Para executá-lo com pytest, que não é uma dependência do SDK, instale o pytest primeiro:

```bash theme={null}
pip install pytest
```

Em seguida, passe seu adaptador para o conjunto em um arquivo de teste como uma fábrica sem argumentos, que `run_session_store_conformance` chama uma vez por contrato para construir um armazenamento novo:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.anyio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

Passar a classe `MyRedisStore` em si, como este exemplo faz, funciona quando o construtor não recebe argumentos. Para um adaptador que recebe um cliente pré-configurado, passe um lambda que constrói o armazenamento em vez disso. Como os contratos reutilizam as mesmas chaves de sessão, cada armazenamento que a fábrica retorna deve começar com armazenamento vazio, então faça o lambda provisionar armazenamento de suporte isolado por chamada, como um novo falso na memória, um prefixo de chave único ou um novo banco de dados de teste.

<h2 id="behavior-notes">
  Notas de comportamento
</h2>

<h3 id="dual-write-architecture">
  Arquitetura de escrita dupla
</h3>

O subprocess Claude Code sempre escreve cada lote de entradas de transcrição no disco local primeiro, e o SDK então encaminha o mesmo lote para `append()` do seu store, então o store é um espelho da transcrição local em vez de uma substituição para ela. Qual cópia sobrevive à execução depende de como a execução foi iniciada:

* **Sessão nova, ou uma retomada quando o store não tem nada para a sessão**: a transcrição local sob seu diretório de configuração sobrevive à execução, e o store recebe uma cópia.
* **Execução [retomada do store](#resume-from-the-store)**: a cópia local é deletada ao final da execução, então o store mantém a única cópia durável.

Se você não quer que uma sessão nova deixe uma transcrição no disco local, defina `CLAUDE_CONFIG_DIR` para um diretório temporário em `options.env`. Uma execução retomada do store já deleta sua cópia local, então não precisa de tal configuração. Em TypeScript, espalhe `process.env` em `env` também, já que a [opção `env`](/docs/pt/agent-sdk/typescript#options) substitui o ambiente do subprocess.

Se seu aplicativo faz login através de arquivos no diretório de configuração, como credenciais OAuth ou um `apiKeyHelper` em seu `settings.json` de usuário, copie esses arquivos para o diretório temporário primeiro, ou defina `ANTHROPIC_API_KEY` em `env` em vez disso. Caso contrário, a execução falha com `Not logged in`.

Duas opções entram em conflito com o espelho, e o SDK lança na inicialização se você combinar qualquer uma com um store:

* **`persistSession: false`** em TypeScript: desativa as escritas locais nas quais o espelho é construído. O SDK Python não tem opção equivalente.
* **Checkpointing de arquivo**, `enableFileCheckpointing` em TypeScript ou `enable_file_checkpointing` em Python: escreve seus backups de arquivo diretamente no disco local, e o SDK não os espelha para o store.

<h3 id="resume-from-the-store">
  Retomar do store
</h3>

Quando você passa `resume`, ou `continue: true` em TypeScript ou `continue_conversation=True` em Python, junto com um store, o SDK pede ao store uma transcrição antes de spawnar o subprocess:

* **`resume`**: o SDK pede a sessão cujo ID você passou.
* **`continue: true`** ou **`continue_conversation=True`**: o SDK pede a sessão mais nova do store.

Quando o store retorna a transcrição, o SDK a escreve em um diretório de configuração temporário, executa o subprocess com `CLAUDE_CONFIG_DIR` apontando para lá, e deleta o diretório quando a execução termina. A transcrição local que essa execução escreve é deletada com ele, que é por que o store mantém a única cópia durável neste caminho.

O SDK também semeia o diretório temporário com arquivos do seu diretório de configuração real. O que ele copia difere por linguagem:

* **TypeScript**: credenciais, `.claude.json`, e seu `settings.json` de usuário. De `settings.json` ele remove as chaves que se comportam mal em um diretório de configuração temporário: `enabledPlugins`, `extraKnownMarketplaces`, seu alias [`additionalMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces), e qualquer `CLAUDE_CONFIG_DIR` no bloco `env` do arquivo. Antes do Agent SDK v0.3.232, o SDK não removia o alias. Autenticação configurada em configurações, como [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper), funciona quando você retoma do store. Antes do Agent SDK v0.3.222, o SDK TypeScript copiava apenas credenciais e `.claude.json`.
* **Python**: apenas credenciais e `.claude.json`, então um aplicativo que se autentica através de `apiKeyHelper` em seu `settings.json` de usuário falha com `Not logged in` ao retomar de um store. Um `apiKeyHelper` em configurações gerenciadas ou de projeto ainda funciona, porque Claude Code lê esses arquivos de locais que `CLAUDE_CONFIG_DIR` não afeta.

Quando o store não tem nada para a sessão, o SDK executa sob seu diretório de configuração real em vez disso, e o resultado depende de qual opção você passou:

* **`resume`**: ambos os SDKs passam o ID através para o subprocess, que retoma a transcrição local exatamente como `resume` faz sem um store.
* **`continue: true`** em TypeScript: o SDK inicia uma sessão nova.
* **`continue_conversation=True`** em Python: o SDK continua da sessão local mais nova.

<h3 id="mirror-writes-are-best-effort">
  Escritas de espelho são best-effort
</h3>

Se `append()` rejeita, o SDK tenta novamente o lote até mais duas vezes com um backoff curto, para no máximo três tentativas no total. Uma chamada que expira não é retentada, já que a chamada original ainda pode chegar. Se o lote ainda falhar, o SDK registra o erro, emite uma mensagem `{ type: "system", subtype: "mirror_error" }` para o iterador, descarta o lote, e continua a consulta. Como um lote retentado pode reentrega entradas que já chegaram, deduplicar por `entry.uuid` em sua implementação de `append()`.

Uma interrupção de store não interrompe o agente, já que o subprocess escreve localmente primeiro. Monitore `mirror_error` se você precisar detectar perda de dados de store. Em uma execução [retomada do store](#resume-from-the-store), um lote descartado não tem cópia sobrevivente uma vez que a execução termina.

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

O SDK nunca deleta de seu store por conta própria. Retenção é responsabilidade do adaptador: use o mecanismo de expiração ou ciclo de vida do seu backend, ou execute limpeza agendada, de acordo com seus requisitos de conformidade.

Transcrições locais em `CLAUDE_CONFIG_DIR` são limpas independentemente pela configuração `cleanupPeriodDays`, seguindo as [regras de limpeza de retenção](/docs/pt/claude-directory#cleaned-up-automatically). Uma execução [retomada do store](#resume-from-the-store) não deixa transcrição local, então para essas execuções a retenção do seu store é a única retenção que existe.

<h2 id="supported-on">
  Suportado em
</h2>

As seguintes funções do SDK TypeScript aceitam uma opção `sessionStore` e operam contra o store em vez do sistema de arquivos local quando fornecido:

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

No SDK Python, defina `session_store` em [`ClaudeAgentOptions`](/docs/pt/agent-sdk/python#claudeagentoptions) para executar `query()` contra um store. As operações restantes cada uma têm uma função Python com suporte a store que recebe o store como argumento: `list_sessions_from_store()`, `get_session_info_from_store()`, `get_session_messages_from_store()`, `list_subagents_from_store()`, `get_subagent_messages_from_store()`, `rename_session_via_store()`, `tag_session_via_store()`, `delete_session_via_store()` e `fork_session_via_store()`. `startup()` não tem equivalente em Python. As funções autônomas documentadas na [referência do SDK Python](/docs/pt/agent-sdk/python#functions), como `list_sessions()`, leem arquivos de sessão locais.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Trabalhar com sessões](/docs/pt/agent-sdk/sessions): Continuar, retomar e fazer fork sem um store personalizado
* [Hospedar o SDK](/docs/pt/agent-sdk/hosting): Padrões de implantação para ambientes multi-host
* [TypeScript `Options`](/docs/pt/agent-sdk/typescript#options): Referência completa de opções
* [Implementações de referência](#reference-implementations): Adaptadores de exemplo executáveis para um object store, um key-value store e um banco de dados, em ambos os repositórios do SDK
