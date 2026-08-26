> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Trabalhar com sessões

> Como as sessões persistem o histórico de conversas do agente e quando usar continue, resume e fork para retornar a uma execução anterior.

Uma sessão é o histórico de conversas que o SDK acumula enquanto seu agente trabalha. Ela contém seu prompt, cada chamada de ferramenta que o agente fez, cada resultado de ferramenta e cada resposta. O SDK a escreve em disco automaticamente para que você possa retornar a ela mais tarde.

Retornar a uma sessão significa que o agente tem contexto completo de antes: arquivos que já leu, análises que já realizou, decisões que já tomou. Você pode fazer uma pergunta de acompanhamento, se recuperar de uma interrupção ou ramificar para tentar uma abordagem diferente.

<Note>
  As sessões persistem a **conversa**, não o sistema de arquivos. Para capturar e reverter alterações de arquivo que o agente fez, use [file checkpointing](/docs/pt/agent-sdk/file-checkpointing).
</Note>

Este guia cobre como escolher a abordagem certa para seu aplicativo, as interfaces do SDK que rastreiam sessões automaticamente, como capturar IDs de sessão e usar `resume` e `fork` manualmente, e o que você precisa saber sobre retomar sessões entre hosts.

<h2 id="choose-an-approach">
  Escolha uma abordagem
</h2>

Quanto gerenciamento de sessão você precisa depende da forma do seu aplicativo. O gerenciamento de sessão entra em jogo quando você envia múltiplos prompts que devem compartilhar contexto. Dentro de uma única chamada `query()`, o agente já faz quantas voltas precisar, e prompts de permissão e `AskUserQuestion` são [tratados em loop](/docs/pt/agent-sdk/user-input) (eles não encerram a chamada).

| O que você está construindo                                           | O que usar                                                                                                                                                   |
| :-------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tarefa única: prompt único, sem acompanhamento                        | Nada extra. Uma chamada `query()` resolve.                                                                                                                   |
| Chat multi-turno em um processo                                       | [`ClaudeSDKClient` (Python) ou `continue: true` (TypeScript)](#automatic-session-management). O SDK rastreia a sessão para você sem nenhum tratamento de ID. |
| Retomar de onde parou após reinicialização do processo                | `continue_conversation=True` (Python) / `continue: true` (TypeScript). Retoma a sessão mais recente no diretório, sem necessidade de ID.                     |
| Retomar uma sessão passada específica (não a mais recente)            | Capture o ID da sessão e passe para `resume`.                                                                                                                |
| Tentar uma abordagem alternativa sem perder a original                | Faça fork da sessão.                                                                                                                                         |
| Tarefa sem estado, não quer nada escrito em disco (apenas TypeScript) | Defina [`persistSession: false`](/docs/pt/agent-sdk/typescript#options). A sessão existe apenas na memória durante a chamada. Python sempre persiste em disco.    |

<h3 id="continue-resume-and-fork">
  Continue, resume e fork
</h3>

Continue, resume e fork são campos de opção que você define em `query()` ([`ClaudeAgentOptions`](/docs/pt/agent-sdk/python#claudeagentoptions) em Python, [`Options`](/docs/pt/agent-sdk/typescript#options) em TypeScript).

**Continue** e **resume** ambos retomam uma sessão existente e adicionam a ela. A diferença é como eles encontram essa sessão:

* **Continue** encontra a sessão mais recente no diretório atual. Você não rastreia nada. Funciona bem quando seu aplicativo executa uma conversa por vez.
* **Resume** recebe um ID de sessão específico. Você rastreia o ID. Necessário quando você tem múltiplas sessões (por exemplo, uma por usuário em um aplicativo multi-usuário) ou quer retornar a uma que não é a mais recente.

**Fork** é diferente: cria uma nova sessão que começa com uma cópia do histórico da original. A original permanece inalterada. Use fork para tentar uma direção diferente enquanto mantém a opção de voltar.

<h2 id="automatic-session-management">
  Gerenciamento automático de sessão
</h2>

Ambos os SDKs oferecem uma interface que rastreia o estado da sessão para você entre chamadas, para que você não passe IDs manualmente. Use-os para conversas multi-turno dentro de um único processo.

<h3 id="python-claudesdkclient">
  Python: `ClaudeSDKClient`
</h3>

[`ClaudeSDKClient`](/docs/pt/agent-sdk/python#claudesdkclient) trata IDs de sessão internamente. Cada chamada para `client.query()` continua automaticamente a mesma sessão. Chame [`client.receive_response()`](/docs/pt/agent-sdk/python#claudesdkclient) para iterar sobre as mensagens da consulta atual. Use o cliente como um gerenciador de contexto assíncrono para que a configuração e encerramento da conexão sejam tratados para você, ou chame `connect()` e `disconnect()` manualmente.

Este exemplo executa duas consultas contra o mesmo `client`. A primeira pede ao agente para analisar um módulo; a segunda pede para refatorar esse módulo. Como ambas as chamadas passam pela mesma instância do cliente, a segunda consulta tem contexto completo da primeira sem nenhum `resume` ou ID de sessão explícito:

```python Python theme={null}
import asyncio
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ResultMessage,
    TextBlock,
)


def print_response(message):
    """Print only the human-readable parts of a message."""
    if isinstance(message, AssistantMessage):
        for block in message.content:
            if isinstance(block, TextBlock):
                print(block.text)
    elif isinstance(message, ResultMessage):
        cost = (
            f"${message.total_cost_usd:.4f}"
            if message.total_cost_usd is not None
            else "N/A"
        )
        print(f"[done: {message.subtype}, cost: {cost}]")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Edit", "Glob", "Grep"],
    )

    async with ClaudeSDKClient(options=options) as client:
        # First query: client captures the session ID internally
        await client.query("Analyze the auth module")
        async for message in client.receive_response():
            print_response(message)

        # Second query: automatically continues the same session
        await client.query("Now refactor it to use JWT")
        async for message in client.receive_response():
            print_response(message)


asyncio.run(main())
```

Veja a [referência do SDK Python](/docs/pt/agent-sdk/python#choosing-between-query-and-claudesdkclient) para detalhes sobre quando usar `ClaudeSDKClient` versus a função `query()` independente.

<h3 id="typescript-continue-true">
  TypeScript: `continue: true`
</h3>

O SDK TypeScript não tem um objeto cliente que mantém sessão como o `ClaudeSDKClient` do Python. Em vez disso, passe `continue: true` em cada chamada `query()` subsequente e o SDK retoma a sessão mais recente no diretório atual. Nenhum rastreamento de ID necessário.

Este exemplo faz duas chamadas `query()` separadas. A primeira cria uma sessão nova; a segunda define `continue: true`, que diz ao SDK para encontrar e retomar a sessão mais recente em disco. O agente tem contexto completo da primeira chamada:

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// First query: creates a new session
for await (const message of query({
  prompt: "Analyze the auth module",
  options: { allowedTools: ["Read", "Glob", "Grep"] }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Second query: continue: true resumes the most recent session
for await (const message of query({
  prompt: "Now refactor it to use JWT",
  options: {
    continue: true,
    allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

<Note>
  A [API de sessão V2](/docs/pt/agent-sdk/typescript-v2-preview) experimental, que fornecia `createSession()` com um padrão `send` / `stream`, foi removida no TypeScript Agent SDK 0.3.142. Use a função `query()` e as opções de sessão descritas nesta página.
</Note>

<h2 id="use-session-options-with-query">
  Use opções de sessão com `query()`
</h2>

<h3 id="capture-the-session-id">
  Capture o ID da sessão
</h3>

Resume e fork requerem um ID de sessão. Leia-o do campo `session_id` na mensagem de resultado ([`ResultMessage`](/docs/pt/agent-sdk/python#resultmessage) em Python, [`SDKResultMessage`](/docs/pt/agent-sdk/typescript#sdkresultmessage) em TypeScript), que está presente em cada resultado independentemente de sucesso ou erro. Em TypeScript, o ID também está disponível mais cedo como um campo direto na `SystemMessage` de inicialização; em Python, está aninhado dentro de `SystemMessage.data`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      session_id = None

      async for message in query(
          prompt="Analyze the auth module and suggest improvements",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep"],
          ),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id
              if message.subtype == "success":
                  print(message.result)

      print(f"Session ID: {session_id}")
      return session_id


  session_id = asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  for await (const message of query({
    prompt: "Analyze the auth module and suggest improvements",
    options: { allowedTools: ["Read", "Glob", "Grep"] }
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
      if (message.subtype === "success") {
        console.log(message.result);
      }
    }
  }

  console.log(`Session ID: ${sessionId}`);
  ```
</CodeGroup>

<h3 id="resume-by-id">
  Retomar por ID
</h3>

Passe um ID de sessão para `resume` para retornar a essa sessão específica. O agente retoma com contexto completo de onde a sessão parou. Razões comuns para retomar:

* **Acompanhamento de uma tarefa concluída.** O agente já analisou algo; agora você quer que ele aja com base nessa análise sem reler arquivos.
* **Recuperação de um limite.** A primeira execução terminou com `error_max_turns` ou `error_max_budget_usd` (veja [Handle the result](/docs/pt/agent-sdk/agent-loop#handle-the-result)); retome com um limite mais alto.
* **Reiniciar seu processo.** Você capturou o ID antes do desligamento e quer restaurar a conversa.

Este exemplo retoma a sessão de [Capture o ID da sessão](#capture-the-session-id) com um prompt de acompanhamento. Como você está retomando, o agente já tem a análise anterior em contexto:

<CodeGroup>
  ```python Python theme={null}
  # Earlier session analyzed the code; now build on that analysis
  async for message in query(
      prompt="Now implement the refactoring you suggested",
      options=ClaudeAgentOptions(
          resume=session_id,
          allowed_tools=["Read", "Edit", "Write", "Glob", "Grep"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Earlier session analyzed the code; now build on that analysis
  for await (const message of query({
    prompt: "Now implement the refactoring you suggested",
    options: {
      resume: sessionId,
      allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Você deve ver uma resposta que se baseia na análise anterior em vez de começar do zero. Isso confirma que o agente retomou a sessão com seu contexto anterior intacto.

<Tip>
  Se uma chamada `resume` retornar uma sessão nova em vez do histórico esperado, a causa mais comum é um `cwd` incompatível. As sessões são armazenadas em `~/.claude/projects/<encoded-cwd>/*.jsonl`, ou em `$CLAUDE_CONFIG_DIR/projects/<encoded-cwd>/*.jsonl` se você definir a variável de ambiente `CLAUDE_CONFIG_DIR`, onde `<encoded-cwd>` é o diretório de trabalho absoluto com cada caractere não alfanumérico substituído por `-` (então `/Users/me/proj` se torna `-Users-me-proj`). Se sua chamada resume for executada de um diretório diferente, o SDK procura no lugar errado. O arquivo de sessão também precisa existir na máquina atual.
</Tip>

Para retomar sessões entre máquinas ou em ambientes sem servidor, espelhe transcrições para armazenamento compartilhado com um adaptador [`SessionStore`](/docs/pt/agent-sdk/session-storage).

<h3 id="fork-to-explore-alternatives">
  Fork para explorar alternativas
</h3>

Forking cria uma nova sessão que começa com uma cópia do histórico da original, mas diverge a partir desse ponto. O fork recebe seu próprio ID de sessão; o ID e histórico da original permanecem inalterados. Você acaba com duas sessões independentes que pode retomar separadamente.

<Note>
  Forking ramifica o histórico de conversas, não o sistema de arquivos. Se um agente com fork editar arquivos, essas alterações são reais e visíveis para qualquer sessão trabalhando no mesmo diretório. Para ramificar e reverter alterações de arquivo, use [file checkpointing](/docs/pt/agent-sdk/file-checkpointing).
</Note>

Este exemplo se baseia em [Capture o ID da sessão](#capture-the-session-id): você já analisou um módulo de autenticação em `session_id` e quer explorar OAuth2 sem perder o thread focado em JWT. O primeiro bloco faz fork da sessão e captura o ID do fork (`forked_id`); o segundo bloco retoma o `session_id` original para continuar pelo caminho JWT. Você agora tem dois IDs de sessão apontando para dois históricos separados:

<CodeGroup>
  ```python Python theme={null}
  # Fork: branch from session_id into a new session
  forked_id = None
  async for message in query(
      prompt="Instead of JWT, outline how OAuth2 would work for the auth module",
      options=ClaudeAgentOptions(
          resume=session_id,
          fork_session=True,
          max_turns=5,
      ),
  ):
      if isinstance(message, ResultMessage):
          forked_id = message.session_id  # The fork's ID, distinct from session_id
          if message.subtype == "success":
              print(message.result)

  print(f"Forked session: {forked_id}")

  # Original session is untouched; resuming it continues the JWT thread
  async for message in query(
      prompt="Continue with the JWT approach",
      options=ClaudeAgentOptions(resume=session_id),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Fork: branch from sessionId into a new session
  let forkedId: string | undefined;

  for await (const message of query({
    prompt: "Instead of JWT, outline how OAuth2 would work for the auth module",
    options: {
      resume: sessionId,
      forkSession: true,
      maxTurns: 5
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      forkedId = message.session_id; // The fork's ID, distinct from sessionId
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }

  console.log(`Forked session: ${forkedId}`);

  // Original session is untouched; resuming it continues the JWT thread
  for await (const message of query({
    prompt: "Continue with the JWT approach",
    options: { resume: sessionId }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Você deve ver que `forkedId` difere do ID de sessão original. Retomar a sessão original ainda continua o thread JWT, o que confirma que o fork não modificou o histórico original.

<h2 id="resume-across-hosts">
  Retomar entre hosts
</h2>

Os arquivos de sessão são locais para a máquina que os criou. Para retomar uma sessão em um host diferente (workers de CI, contêineres efêmeros, sem servidor), você tem duas opções:

* **Mover o arquivo de sessão.** Persista `~/.claude/projects/<encoded-cwd>/<session-id>.jsonl` da primeira execução e restaure-o para o mesmo caminho no novo host antes de chamar `resume`. O `cwd` deve corresponder.
* **Não confie em retomada de sessão.** Capture os resultados que você precisa (saída de análise, decisões, diffs de arquivo) como estado do aplicativo e passe-os para o prompt de uma sessão nova. Isso geralmente é mais robusto do que enviar arquivos de transcrição.

Ambos os SDKs expõem funções para enumerar sessões em disco e ler suas mensagens: [`listSessions()`](/docs/pt/agent-sdk/typescript#listsessions) e [`getSessionMessages()`](/docs/pt/agent-sdk/typescript#getsessionmessages) em TypeScript, [`list_sessions()`](/docs/pt/agent-sdk/python#list_sessions) e [`get_session_messages()`](/docs/pt/agent-sdk/python#get_session_messages) em Python. Use-os para construir seletores de sessão personalizados, lógica de limpeza ou visualizadores de transcrição.

Ambos os SDKs também expõem funções para procurar e mutar sessões individuais: [`get_session_info()`](/docs/pt/agent-sdk/python#get_session_info), [`rename_session()`](/docs/pt/agent-sdk/python#rename_session) e [`tag_session()`](/docs/pt/agent-sdk/python#tag_session) em Python, e [`getSessionInfo()`](/docs/pt/agent-sdk/typescript#getsessioninfo), [`renameSession()`](/docs/pt/agent-sdk/typescript#renamesession) e [`tagSession()`](/docs/pt/agent-sdk/typescript#tagsession) em TypeScript. Use-os para organizar sessões por tag ou dar-lhes títulos legíveis por humanos.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [How the agent loop works](/docs/pt/agent-sdk/agent-loop): Entenda voltas, mensagens e acumulação de contexto dentro de uma sessão
* [File checkpointing](/docs/pt/agent-sdk/file-checkpointing): Faça snapshot e reverta alterações de arquivo que o agente fez dentro de uma sessão
* [Python `ClaudeAgentOptions`](/docs/pt/agent-sdk/python#claudeagentoptions): Referência completa de opções de sessão para Python
* [TypeScript `Options`](/docs/pt/agent-sdk/typescript#options): Referência completa de opções de sessão para TypeScript
