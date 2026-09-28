> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Interceptar e controlar o comportamento do agente com hooks

> Interceptar e personalizar o comportamento do agente em pontos-chave de execução com hooks

Hooks são funções de callback que executam seu código em resposta a eventos do agente, como uma ferramenta sendo chamada, uma sessão iniciando ou a execução parando. Com hooks, você pode:

* **Bloquear operações perigosas** antes de serem executadas, como comandos shell destrutivos ou acesso a arquivos não autorizado
* **Registrar e auditar** cada chamada de ferramenta para conformidade, depuração ou análise
* **Transformar entradas e saídas** para sanitizar dados, injetar credenciais ou redirecionar caminhos de arquivo
* **Exigir aprovação humana** para ações sensíveis como gravações em banco de dados ou chamadas de API
* **Rastrear ciclo de vida da sessão** para gerenciar estado, limpar recursos ou enviar notificações

<h2 id="how-hooks-work">
  Como hooks funcionam
</h2>

<Steps>
  <Step title="Um evento é disparado">
    Algo acontece durante a execução do agente e o SDK dispara um evento: uma ferramenta está prestes a ser chamada (`PreToolUse`), uma ferramenta retornou um resultado (`PostToolUse`), um subagente iniciou ou parou, o agente está ocioso ou a execução terminou. Veja a [lista completa de eventos](#available-hooks).
  </Step>

  <Step title="O SDK coleta hooks registrados">
    O SDK verifica se há hooks registrados para esse tipo de evento. Isso inclui hooks de callback que você passa em `options.hooks` e hooks de comando shell de arquivos de configuração quando a entrada [`settingSources`](/docs/pt/agent-sdk/typescript#settingsource) ou [`setting_sources`](/docs/pt/agent-sdk/python#settingsource) correspondente está habilitada, o que é o padrão para opções `query()`.
  </Step>

  <Step title="Matchers filtram quais hooks são executados">
    Se um hook tem um padrão [`matcher`](#matchers) (como `"Write|Edit"`), o SDK o testa contra o alvo do evento (por exemplo, o nome da ferramenta). Hooks sem um matcher são executados para cada evento desse tipo.
  </Step>

  <Step title="Funções de callback são executadas">
    Cada [função de callback](#callback-functions) do hook correspondente recebe informações sobre o que está acontecendo: o nome da ferramenta, seus argumentos, o ID da sessão e outros detalhes específicos do evento.
  </Step>

  <Step title="Seu callback retorna uma decisão">
    Após realizar qualquer operação (registro, chamadas de API, validação), seu callback retorna um [objeto de saída](#outputs) que diz ao agente o que fazer: permitir a operação, bloqueá-la, modificar a entrada ou injetar contexto na conversa.
  </Step>
</Steps>

O exemplo a seguir reúne essas etapas. Ele registra um hook `PreToolUse` (etapa 1) com um matcher `"Write|Edit"` (etapa 3) para que o callback seja acionado apenas para ferramentas de escrita de arquivo. Quando acionado, o callback recebe a entrada da ferramenta (etapa 4), verifica se o caminho do arquivo tem como alvo um arquivo `.env` e retorna `permissionDecision: "deny"` para bloquear a operação (etapa 5):

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeSDKClient,
      ClaudeAgentOptions,
      HookMatcher,
      ResultMessage,
  )


  # Define a hook callback that receives tool call details
  async def protect_env_files(input_data, tool_use_id, context):
      # Extract the file path from the tool's input arguments
      file_path = input_data["tool_input"].get("file_path", "")
      file_name = file_path.split("/")[-1]

      # Block the operation if targeting a .env file
      if file_name == ".env":
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Cannot modify .env files",
              }
          }

      # Return empty object to allow the operation
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for PreToolUse events
              # The matcher filters to only Write and Edit tool calls
              "PreToolUse": [HookMatcher(matcher="Write|Edit", hooks=[protect_env_files])]
          }
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Create a .env file with the standard local development database configuration")
          async for message in client.receive_response():
              # Filter for assistant and result messages
              if isinstance(message, (AssistantMessage, ResultMessage)):
                  print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback with the HookCallback type
  const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast input to the specific hook type for type safety
    const preInput = input as PreToolUseHookInput;

    // Cast tool_input to access its properties (typed as unknown in the SDK)
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;
    const fileName = filePath?.split("/").pop();

    // Block the operation if targeting a .env file
    if (fileName === ".env") {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Cannot modify .env files"
        }
      };
    }

    // Return empty object to allow the operation
    return {};
  };

  for await (const message of query({
    prompt: "Create a .env file with the standard local development database configuration",
    options: {
      hooks: {
        // Register the hook for PreToolUse events
        // The matcher filters to only Write and Edit tool calls
        PreToolUse: [{ matcher: "Write|Edit", hooks: [protectEnvFiles] }]
      }
    }
  })) {
    // Filter for assistant and result messages
    if (message.type === "assistant" || message.type === "result") {
      console.log(message);
    }
  }
  ```
</CodeGroup>

Quando você executa qualquer um dos scripts, Claude tenta criar o arquivo `.env`, o hook nega a chamada da ferramenta e a resposta final do Claude explica que ele não pode criar arquivos `.env`.

<h2 id="available-hooks">
  Hooks disponíveis
</h2>

O SDK fornece hooks para diferentes estágios de execução do agente. Alguns hooks estão disponíveis em ambos os SDKs, enquanto outros são apenas para TypeScript.

| Evento de Hook                                         | SDK Python | SDK TypeScript | O que o dispara                                                                                                                                                | Caso de uso de exemplo                                                                                                                                                                     |
| ------------------------------------------------------ | ---------- | -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `PreToolUse`                                           | Sim        | Sim            | Solicitação de chamada de ferramenta (pode bloquear ou modificar)                                                                                              | Bloquear comandos shell perigosos                                                                                                                                                          |
| `PostToolUse`                                          | Sim        | Sim            | Resultado de execução de ferramenta                                                                                                                            | Registrar todas as alterações de arquivo na trilha de auditoria                                                                                                                            |
| `PostToolUseFailure`                                   | Sim        | Sim            | Falha na execução de ferramenta                                                                                                                                | Lidar ou registrar erros de ferramenta                                                                                                                                                     |
| `PostToolBatch`                                        | Não        | Sim            | Um lote completo de chamadas de ferramenta é resolvido, uma vez por lote antes da próxima chamada de modelo                                                    | Injetar convenções uma vez para todo o lote                                                                                                                                                |
| `UserPromptSubmit`                                     | Sim        | Sim            | Envio de prompt do usuário                                                                                                                                     | Injetar contexto adicional em prompts                                                                                                                                                      |
| [`UserPromptExpansion`](/docs/pt/hooks#userpromptexpansion) | Não        | Sim            | Um comando digitado pelo usuário, ou um prompt MCP, se expande em um prompt antes de chegar ao Claude. Não dispara quando Claude invoca uma skill por si mesmo | Bloquear um comando de invocação direta ou adicionar contexto quando uma skill é digitada                                                                                                  |
| `MessageDisplay`                                       | Não        | Sim            | Uma mensagem do assistente com texto é concluída, uma vez por mensagem com o texto completo da mensagem                                                        | Redigir ou reformatar o texto exibido sem alterar a transcrição                                                                                                                            |
| `Stop`                                                 | Sim        | Sim            | Parada de execução do agente                                                                                                                                   | Salvar estado da sessão antes de sair                                                                                                                                                      |
| `StopFailure`                                          | Não        | Sim            | O turno termina com um erro de API em vez de uma parada normal                                                                                                 | Registrar falhas ou enviar alertas                                                                                                                                                         |
| `SubagentStart`                                        | Sim        | Sim            | Inicialização de subagente                                                                                                                                     | Rastrear geração de tarefas paralelas                                                                                                                                                      |
| `SubagentStop`                                         | Sim        | Sim            | Conclusão de subagente                                                                                                                                         | Agregar resultados de tarefas paralelas                                                                                                                                                    |
| `PreCompact`                                           | Sim        | Sim            | Solicitação de compactação de conversa                                                                                                                         | Arquivar transcrição completa antes de resumir                                                                                                                                             |
| `PostCompact`                                          | Não        | Sim            | Compactação de conversa é concluída                                                                                                                            | Registrar o resumo gerado                                                                                                                                                                  |
| [`PreModelSwitch`](/docs/pt/hooks#premodelswitch)           | Não        | Sim            | Uma mudança de modelo solicitada, antes de acontecer (pode bloquear)                                                                                           | Bloquear a mudança para um modelo específico                                                                                                                                               |
| [`PostModelSwitch`](/docs/pt/hooks#postmodelswitch)         | Não        | Sim            | O modelo da sessão muda, incluindo um fallback automático                                                                                                      | Dar ao Claude orientação específica do modelo para o novo modelo                                                                                                                           |
| `PermissionRequest`                                    | Sim        | Sim            | Uma chamada de ferramenta precisa de uma decisão de permissão                                                                                                  | Manipulação de permissão personalizada                                                                                                                                                     |
| `PermissionDenied`                                     | Não        | Sim            | O modo automático nega uma chamada de ferramenta, incluindo negações sem um veredicto do classificador                                                         | Registrar negações, ou informar ao modelo que ele pode tentar novamente; Claude Code ignora `retry: true` para negações sem veredicto. Veja [PermissionDenied](/docs/pt/hooks#permissiondenied) |
| `SessionStart`                                         | Não        | Sim            | Inicialização de sessão                                                                                                                                        | Inicializar registro e telemetria                                                                                                                                                          |
| `SessionEnd`                                           | Não        | Sim            | Encerramento de sessão                                                                                                                                         | Limpar recursos temporários                                                                                                                                                                |
| `Notification`                                         | Sim        | Sim            | Mensagens de status do agente                                                                                                                                  | Enviar atualizações de status do agente para Slack ou PagerDuty                                                                                                                            |
| `Setup`                                                | Não        | Sim            | Configuração/manutenção de sessão                                                                                                                              | Executar tarefas de inicialização                                                                                                                                                          |
| `TeammateIdle`                                         | Não        | Sim            | Colega fica ocioso                                                                                                                                             | Reatribuir trabalho ou notificar                                                                                                                                                           |
| `TaskCreated`                                          | Não        | Sim            | Uma tarefa é criada via a ferramenta `TaskCreate`                                                                                                              | Aplicar convenções de nomenclatura de tarefas                                                                                                                                              |
| [`TaskCompleted`](/docs/pt/hooks#taskcompleted)             | Não        | Sim            | Uma tarefa é marcada como concluída                                                                                                                            | Exigir testes aprovados antes de uma tarefa ser fechada                                                                                                                                    |
| `Elicitation`                                          | Não        | Sim            | Um servidor MCP solicita entrada do usuário no meio da tarefa                                                                                                  | Responder a solicitações de entrada MCP programaticamente                                                                                                                                  |
| `ElicitationResult`                                    | Não        | Sim            | Um usuário responde a uma elicitação MCP                                                                                                                       | Modificar ou bloquear a resposta antes de ela retornar ao servidor                                                                                                                         |
| `ConfigChange`                                         | Não        | Sim            | Arquivo de configuração muda                                                                                                                                   | Recarregar configurações dinamicamente                                                                                                                                                     |
| `InstructionsLoaded`                                   | Não        | Sim            | Um arquivo `CLAUDE.md` ou arquivo de regras é carregado no contexto                                                                                            | Auditar quais arquivos de instrução são carregados                                                                                                                                         |
| `WorktreeCreate`                                       | Não        | Sim            | Git worktree criado                                                                                                                                            | Rastrear espaços de trabalho isolados                                                                                                                                                      |
| `WorktreeRemove`                                       | Não        | Sim            | Git worktree removido                                                                                                                                          | Limpar recursos de espaço de trabalho                                                                                                                                                      |
| `CwdChanged`                                           | Não        | Sim            | O diretório de trabalho muda durante uma sessão                                                                                                                | Recarregar variáveis de ambiente por diretório                                                                                                                                             |
| `FileChanged`                                          | Não        | Sim            | Um arquivo monitorado é modificado, criado ou deletado                                                                                                         | Recarregar configuração quando arquivos do projeto mudam                                                                                                                                   |
| `DirectoryAdded`                                       | Não        | Sim            | Um diretório de trabalho é adicionado durante uma sessão                                                                                                       | Instalar dependências para um repositório adicionado no meio da sessão                                                                                                                     |

<h2 id="configure-hooks">
  Configurar hooks
</h2>

Para configurar um hook, passe-o no campo `hooks` de suas opções de agente (`ClaudeAgentOptions` em Python, o objeto `options` em TypeScript). Este trecho assume que você já definiu um callback de hook, como `protect_env_files` em Python ou `protectEnvFiles` em TypeScript do exemplo acima:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={"PreToolUse": [HookMatcher(matcher="Bash", hooks=[my_callback])]}
  )

  async with ClaudeSDKClient(options=options) as client:
      await client.query("Your prompt")
      async for message in client.receive_response():
          print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Your prompt",
    options: {
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [myCallback] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

A opção `hooks` é um dicionário em Python ou um objeto em TypeScript, onde:

* **Chaves**: [nomes de eventos de hook](#available-hooks) como `'PreToolUse'`, `'PostToolUse'` e `'Stop'`
* **Valores**: arrays de [matchers](#matchers), cada um contendo um padrão de filtro opcional e suas [funções de callback](#callback-functions)

<h3 id="matchers">
  Matchers
</h3>

Use matchers para filtrar quando seus callbacks são acionados. O campo `matcher` corresponde a um valor diferente dependendo do tipo de evento de hook. Por exemplo, hooks baseados em ferramentas correspondem ao nome da ferramenta, enquanto hooks `Notification` correspondem ao tipo de notificação.

Os matchers do SDK seguem as mesmas regras que [matchers em arquivos de configuração](/docs/pt/hooks#matcher-patterns). Essa seção documenta os caminhos de avaliação de string exata e expressão regular, seus requisitos de versão e os valores de matcher para cada tipo de evento.

| Opção     | Tipo             | Padrão      | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| --------- | ---------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `matcher` | `string`         | `undefined` | Padrão correspondido contra o campo de filtro do evento, seguindo as [regras para matchers em arquivos de configuração](/docs/pt/hooks#matcher-patterns). Para hooks de ferramenta, este é o nome da ferramenta. As ferramentas integradas incluem `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Agent` e outras (veja [Tipos de Entrada de Ferramenta](/docs/pt/agent-sdk/typescript#tool-input-types) para a lista completa). Ferramentas MCP usam o padrão `mcp__<server>__<action>`, onde `<server>` é a chave que você usa na configuração `mcpServers`. |
| `hooks`   | `HookCallback[]` | -           | Obrigatório. Array de funções de callback a executar quando o padrão corresponde                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `timeout` | `number`         | `undefined` | Timeout em segundos. Quando omitido, Claude Code aplica o [timeout padrão do evento](#hook-timeout). Seus callbacks do SDK seguem os padrões de hook `command`                                                                                                                                                                                                                                                                                                                                                                                                      |

Use o padrão `matcher` para direcionar ferramentas específicas sempre que possível. Um matcher com `'Bash'` é executado apenas para comandos Bash, enquanto omitir o padrão executa seus callbacks para cada ocorrência do evento. Omita-o propositalmente para registrar cada chamada de ferramenta que sua sessão faz.

<h3 id="callback-functions">
  Funções de callback
</h3>

<h4 id="inputs">
  Entradas
</h4>

Cada callback de hook recebe três argumentos:

* **Dados de entrada:** um objeto tipado contendo detalhes do evento. Cada tipo de hook tem sua própria forma de entrada. Por exemplo, `PreToolUseHookInput` inclui `tool_name` e `tool_input`, enquanto `NotificationHookInput` inclui `message`. Veja as definições de tipo completas nas referências do SDK [TypeScript](/docs/pt/agent-sdk/typescript#hookinput) e [Python](/docs/pt/agent-sdk/python#hookinput).
  * Todas as entradas de hook compartilham `session_id`, `cwd` e `hook_event_name`.
  * `agent_id` e `agent_type` são preenchidos quando o hook é acionado dentro de um subagente. Em TypeScript, estes estão na entrada de hook base e disponíveis para todos os tipos de hook. Em Python, eles são campos opcionais em `PreToolUse`, `PostToolUse`, `PostToolUseFailure` e `PermissionRequest`, e campos obrigatórios em `SubagentStart` e `SubagentStop`.
* **ID de uso de ferramenta** (`str | None` / `string | undefined`): correlaciona eventos `PreToolUse` e `PostToolUse` para a mesma chamada de ferramenta.
* **Contexto:** em TypeScript, contém uma propriedade `signal` (`AbortSignal`) para cancelamento. Em Python, este argumento é reservado para uso futuro.

<h4 id="outputs">
  Saídas
</h4>

Seu callback retorna um objeto com duas categorias de campos:

* **Campos de nível superior** são aceitos em cada evento: `systemMessage` mostra uma mensagem ao usuário, e `continue` (`continue_` em Python) determina se o agente continua executando após este hook. Alguns eventos descartam-nos ou os entregam em outro lugar. A seção de cada [evento](/docs/pt/hooks#hook-events) na página de hooks diz onde eles chegam.
* **`hookSpecificOutput`** controla a operação atual. Os campos que você define dentro dependem do tipo de evento de hook:
  * Para hooks `PreToolUse`, é aqui que você define `permissionDecision` (`"allow"`, `"deny"`, `"ask"` ou `"defer"`), `permissionDecisionReason` e `updatedInput`. Se você retornar `"defer"`, a consulta termina para que você possa [retomá-la depois](/docs/pt/hooks#defer-a-tool-call-for-later).
  * Para hooks `PostToolUse`, você pode definir `additionalContext` para anexar informações ao resultado da ferramenta. Para substituir a saída da ferramenta antes de Claude vê-la, defina `updatedToolOutput`, que funciona para qualquer ferramenta em ambos os SDKs. O campo mais antigo `updatedMCPToolOutput` substitui apenas a saída de ferramentas MCP e está descontinuado.
  * No SDK TypeScript, um callback `PostToolUse` também pode retornar `classifierContext`, uma nota breve sobre o resultado da chamada de ferramenta para o classificador de permissão [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode). Como seu callback é executado no próprio processo de sua aplicação, o classificador pode pesar uma declaração de usuário que você transmite na nota como intenção do usuário. O campo requer Agent SDK TypeScript v0.3.236 ou posterior. [Anotar um resultado para o classificador de modo automático](/docs/pt/hooks#annotate-a-result-for-the-auto-mode-classifier) cobre o limite de comprimento, a regra somente síncrona e o que não colocar na nota.

Retorne `{}` para permitir a operação sem alterações. Hooks de callback do SDK usam o mesmo formato de saída JSON que [hooks de comando shell do Claude Code](/docs/pt/hooks#json-output), que documenta cada campo e opção específica do evento. Para as definições de tipo do SDK, veja as referências do SDK [TypeScript](/docs/pt/agent-sdk/typescript#synchookjsonoutput) e [Python](/docs/pt/agent-sdk/python#synchookjsonoutput).

<Note>
  Quando múltiplos hooks ou regras de permissão se aplicam, `deny` tem prioridade sobre `defer`, que tem prioridade sobre `ask`, que tem prioridade sobre `allow`. Se qualquer hook retornar `deny`, a operação é bloqueada independentemente de outros hooks.
</Note>

<h4 id="asynchronous-output">
  Saída assíncrona
</h4>

Por padrão, o agente aguarda seu hook retornar antes de prosseguir. Se seu hook realiza um efeito colateral, como registro ou envio de webhook, e não precisa influenciar o comportamento do agente, você pode retornar uma saída assíncrona. Isso diz ao agente para continuar imediatamente sem aguardar o hook terminar. Neste trecho, `send_to_logging_service` em Python e `sendToLoggingService` em TypeScript representam qualquer função de registro que você defina:

<CodeGroup>
  ```python Python theme={null}
  async def async_hook(input_data, tool_use_id, context):
      # Start a background task, then return immediately
      asyncio.create_task(send_to_logging_service(input_data))
      return {"async_": True, "asyncTimeout": 30000}
  ```

  ```typescript TypeScript theme={null}
  const asyncHook: HookCallback = async (input, toolUseID, { signal }) => {
    // Start a background task, then return immediately
    sendToLoggingService(input).catch(console.error);
    return { async: true, asyncTimeout: 30000 };
  };
  ```
</CodeGroup>

| Campo          | Tipo     | Descrição                                                                                                                 |
| -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------- |
| `async`        | `true`   | Sinaliza modo assíncrono. O agente prossegue sem aguardar. Em Python, use `async_` para evitar a palavra-chave reservada. |
| `asyncTimeout` | `number` | Timeout opcional em milissegundos para a operação em segundo plano                                                        |

<Note>
  Saídas assíncronas não podem bloquear, modificar ou injetar contexto na operação, pois o agente já avançou. Use-as apenas para efeitos colaterais como registro, métricas ou notificações.
</Note>

<h2 id="examples">
  Exemplos
</h2>

Vários exemplos nesta seção mostram apenas a função de callback. Para executar um, registre o callback sob o evento correspondente no campo `hooks` de suas opções, conforme mostrado em [Configurar hooks](#configure-hooks).

<h3 id="modify-tool-input">
  Modificar entrada de ferramenta
</h3>

Este exemplo intercepta chamadas de ferramenta Write e reescreve o argumento `file_path` para prepender `/sandbox`, redirecionando todas as gravações de arquivo para um diretório em sandbox. O callback retorna `updatedInput` com o caminho modificado e `permissionDecision: 'allow'` para aprovar automaticamente a operação reescrita:

<CodeGroup>
  ```python Python theme={null}
  async def redirect_to_sandbox(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      if input_data["tool_name"] == "Write":
          original_path = input_data["tool_input"].get("file_path", "")
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "updatedInput": {
                      **input_data["tool_input"],
                      "file_path": f"/sandbox{original_path}",
                  },
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    if (preInput.tool_name === "Write") {
      const originalPath = toolInput.file_path as string;
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          updatedInput: {
            ...toolInput,
            file_path: `/sandbox${originalPath}`
          }
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<Note>
  Combine `updatedInput` com `permissionDecision: 'allow'` para aprovar automaticamente a entrada modificada, ou `permissionDecision: 'ask'` para mostrá-la ao usuário. Se você omitir `permissionDecision`, a entrada modificada ainda se aplica e flui pela avaliação de permissão normal. Com `'defer'`, `updatedInput` é ignorado. Sempre retorne um novo objeto em vez de mutar o `tool_input` original.
</Note>

Para confirmar o redirecionamento, defina o prefixo para um caminho em que você possa escrever, como `./sandbox` ou `/tmp/sandbox` (macOS não permite criar um diretório `/sandbox` no nível raiz), depois peça ao agente para escrever um arquivo: o resultado da ferramenta Write no fluxo de mensagens nomeia o caminho com seu prefixo de sandbox em vez do que Claude solicitou.

<h3 id="add-context-and-block-a-tool">
  Adicionar contexto e bloquear uma ferramenta
</h3>

Este exemplo bloqueia gravações no diretório `/etc` e explica o motivo tanto para o modelo quanto para o usuário:

* `permissionDecision: 'deny'` interrompe a chamada de ferramenta.
* `permissionDecisionReason` informa ao modelo por que, para que ele evite tentar novamente.
* `systemMessage` mostra ao usuário o que aconteceu.

<CodeGroup>
  ```python Python theme={null}
  async def block_etc_writes(input_data, tool_use_id, context):
      file_path = input_data["tool_input"].get("file_path", "")

      if file_path.startswith("/etc"):
          return {
              # Top-level field: message shown to the user
              "systemMessage": "Remember: system directories like /etc are protected.",
              # hookSpecificOutput: block the operation
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Writing to /etc is not allowed",
              },
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;

    if (filePath?.startsWith("/etc")) {
      return {
        // Top-level field: message shown to the user
        systemMessage: "Remember: system directories like /etc are protected.",
        // hookSpecificOutput: block the operation
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Writing to /etc is not allowed"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="auto-approve-specific-tools">
  Aprovar automaticamente ferramentas específicas
</h3>

Por padrão, o agente pode solicitar permissão antes de usar certas ferramentas. Este exemplo aprova automaticamente ferramentas de sistema de arquivos somente leitura (Read, Glob, Grep) retornando `permissionDecision: 'allow'`, permitindo que sejam executadas sem confirmação do usuário enquanto deixa todas as outras ferramentas sujeitas a verificações de permissão normais:

<CodeGroup>
  ```python Python theme={null}
  async def auto_approve_read_only(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      read_only_tools = ["Read", "Glob", "Grep"]
      if input_data["tool_name"] in read_only_tools:
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "permissionDecisionReason": "Read-only tool auto-approved",
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const readOnlyTools = ["Read", "Glob", "Grep"];
    if (readOnlyTools.includes(preInput.tool_name)) {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          permissionDecisionReason: "Read-only tool auto-approved"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="register-multiple-hooks">
  Registrar múltiplos hooks
</h3>

Quando um evento é disparado, todos os hooks correspondentes são executados em paralelo. Para decisões de permissão, o resultado mais restritivo vence: um único `deny` bloqueia a chamada de ferramenta independentemente do que os outros hooks retornam. Como a ordem de conclusão é não-determinística, escreva cada hook para agir independentemente em vez de depender de outro hook ter sido executado primeiro.

O exemplo abaixo registra três verificações independentes para cada chamada de ferramenta:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              HookMatcher(hooks=[authorization_check]),
              HookMatcher(hooks=[input_validator]),
              HookMatcher(hooks=[audit_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        { hooks: [authorizationCheck] },
        { hooks: [inputValidator] },
        { hooks: [auditLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="filter-with-multi-tool-matchers">
  Filtrar com matchers de múltiplas ferramentas
</h3>

Use matchers de múltiplas ferramentas para compartilhar um callback entre ferramentas relacionadas. Este exemplo registra três matchers com escopos diferentes:

* Uma lista exata separada por pipe (`Write|Edit|NotebookEdit`) dispara `file_security_hook` apenas para ferramentas de modificação de arquivo.
* Uma regex (`^mcp__`) dispara `mcp_audit_hook` para qualquer ferramenta MCP cujo nome começa com `mcp__`.
* Um matcher omitido dispara `global_logger` para cada chamada de ferramenta independentemente do nome.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              # Match file modification tools
              HookMatcher(matcher="Write|Edit|NotebookEdit", hooks=[file_security_hook]),
              # Match all MCP tools
              HookMatcher(matcher="^mcp__", hooks=[mcp_audit_hook]),
              # Match everything (no matcher)
              HookMatcher(hooks=[global_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        // Match file modification tools
        { matcher: "Write|Edit|NotebookEdit", hooks: [fileSecurityHook] },

        // Match all MCP tools
        { matcher: "^mcp__", hooks: [mcpAuditHook] },

        // Match everything (no matcher)
        { hooks: [globalLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="track-subagent-activity">
  Rastrear atividade de subagente
</h3>

Use hooks `SubagentStop` para monitorar quando subagentes terminam seu trabalho. Veja o tipo de entrada completo nas referências do SDK [TypeScript](/docs/pt/agent-sdk/typescript#hookinput) e [Python](/docs/pt/agent-sdk/python#hookinput). Este exemplo registra um resumo cada vez que um subagente é concluído:

<CodeGroup>
  ```python Python theme={null}
  async def subagent_tracker(input_data, tool_use_id, context):
      # Log subagent details when it finishes
      print(f"[SUBAGENT] Completed: {input_data['agent_id']}")
      print(f"  Transcript: {input_data['agent_transcript_path']}")
      print(f"  Tool use ID: {tool_use_id}")
      print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
      return {}


  options = ClaudeAgentOptions(
      hooks={"SubagentStop": [HookMatcher(hooks=[subagent_tracker])]}
  )
  ```

  ```typescript TypeScript theme={null}
  import { HookCallback, SubagentStopHookInput } from "@anthropic-ai/claude-agent-sdk";

  const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to SubagentStopHookInput to access subagent-specific fields
    const subInput = input as SubagentStopHookInput;

    // Log subagent details when it finishes
    console.log(`[SUBAGENT] Completed: ${subInput.agent_id}`);
    console.log(`  Transcript: ${subInput.agent_transcript_path}`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${subInput.stop_hook_active}`);
    return {};
  };

  const options = {
    hooks: {
      SubagentStop: [{ hooks: [subagentTracker] }]
    }
  };
  ```
</CodeGroup>

<h3 id="make-http-requests-from-hooks">
  Fazer requisições HTTP a partir de hooks
</h3>

Hooks podem realizar operações assíncronas como requisições HTTP. Capture erros dentro de seu hook em vez de deixá-los se propagar.

Este exemplo envia um webhook após cada ferramenta ser concluída, registrando qual ferramenta foi executada e quando. O hook captura erros de um webhook falhado:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request
  from datetime import datetime


  def _send_webhook(tool_name):
      """Synchronous helper that POSTs tool usage data to an external webhook."""
      data = json.dumps(
          {
              "tool": tool_name,
              "timestamp": datetime.now().isoformat(),
          }
      ).encode()
      req = urllib.request.Request(
          "https://api.example.com/webhook",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def webhook_notifier(input_data, tool_use_id, context):
      # Only fire after a tool completes (PostToolUse), not before
      if input_data["hook_event_name"] != "PostToolUse":
          return {}

      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_webhook, input_data["tool_name"])
      except Exception as e:
          # Log the error but don't raise
          print(f"Webhook request failed: {e}")

      return {}
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PostToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
    // Only fire after a tool completes (PostToolUse), not before
    if (input.hook_event_name !== "PostToolUse") return {};

    try {
      await fetch("https://api.example.com/webhook", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tool: (input as PostToolUseHookInput).tool_name,
          timestamp: new Date().toISOString()
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      // Handle cancellation separately from other errors
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Webhook request cancelled");
      }
      // Don't re-throw
    }

    return {};
  };

  // Register as a PostToolUse hook
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      hooks: {
        PostToolUse: [{ hooks: [webhookNotifier] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Para confirmar que o hook é disparado, aponte a URL do webhook para um endpoint que você possa monitorar e envie um prompt que use uma ferramenta: o hook envia um POST com o nome da ferramenta e o timestamp após cada ferramenta ser concluída.

<h3 id="forward-notifications-to-slack">
  Encaminhar notificações para Slack
</h3>

Use hooks `Notification` para receber notificações do sistema do agente e encaminhá-las para serviços externos. Em sessões do SDK, Claude Code executa este hook para os seguintes tipos de notificação:

* [`permission_prompt`](/docs/pt/hooks#notification) uma vez que uma solicitação de permissão aguardou cerca de seis segundos em seu callback [`canUseTool`](/docs/pt/agent-sdk/user-input). Requer Agent SDK TypeScript v0.3.233 ou posterior, ou Agent SDK Python v0.2.139 ou posterior
* `elicitation_complete` e `elicitation_response` para fluxos de elicitação de entrada do usuário

Claude Code emite os outros tipos, como `idle_prompt`, `auth_success` e `elicitation_dialog`, a partir da interface interativa que sessões do SDK não executam.

Cada notificação inclui um campo `message` com uma descrição legível por humanos e opcionalmente um `title`.

Este exemplo encaminha cada notificação para um canal Slack. Requer uma [URL de webhook de entrada do Slack](https://docs.slack.dev/messaging/sending-messages-using-incoming-webhooks/), que você cria adicionando um app ao seu espaço de trabalho Slack e habilitando webhooks de entrada:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request

  from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, HookMatcher


  def _send_slack_notification(message):
      """Synchronous helper that sends a message to Slack via incoming webhook."""
      data = json.dumps({"text": f"Agent status: {message}"}).encode()
      req = urllib.request.Request(
          "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def notification_handler(input_data, tool_use_id, context):
      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_slack_notification, input_data.get("message", ""))
      except Exception as e:
          print(f"Failed to send notification: {e}")

      # Return empty object. Notification hooks don't modify agent behavior
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for Notification events (no matcher needed)
              "Notification": [HookMatcher(hooks=[notification_handler])],
          },
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Analyze this codebase")
          async for message in client.receive_response():
              print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback that sends notifications to Slack
  const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to NotificationHookInput to access the message field
    const notification = input as NotificationHookInput;

    try {
      // POST the notification message to a Slack incoming webhook
      await fetch("https://hooks.slack.com/services/YOUR/WEBHOOK/URL", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          text: `Agent status: ${notification.message}`
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Notification cancelled");
      } else {
        console.error("Failed to send notification:", error);
      }
    }

    // Return empty object. Notification hooks don't modify agent behavior
    return {};
  };

  // Register the hook for Notification events (no matcher needed)
  for await (const message of query({
    prompt: "Analyze this codebase",
    options: {
      hooks: {
        Notification: [{ hooks: [notificationHandler] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Quando um evento `Notification` é disparado, o hook publica a `message` da notificação, prefixada com `Agent status:`, para o canal que seu webhook aponta.

<h2 id="fix-common-issues">
  Corrigir problemas comuns
</h2>

<h3 id="hook-not-firing">
  Hook não é disparado
</h3>

* Verifique se o nome do evento de hook está correto e sensível a maiúsculas/minúsculas (`PreToolUse`, não `preToolUse`)
* Verifique se seu padrão de matcher corresponde exatamente ao nome da ferramenta
* Certifique-se de que o hook está sob o tipo de evento correto em `options.hooks`
* Para hooks não baseados em ferramentas que suportam matchers, como `Notification` e `SubagentStop`, matchers correspondem a campos diferentes, e `Stop` ignora matchers completamente (veja [padrões de matcher](/docs/pt/hooks#matcher-patterns))
* Hooks podem não ser disparados quando o agente atinge o limite [`max_turns`](/docs/pt/agent-sdk/python#claudeagentoptions) porque a sessão termina antes que hooks possam ser executados

<h3 id="matcher-not-filtering-as-expected">
  Matcher não filtra como esperado
</h3>

Matchers apenas correspondem a nomes de ferramentas, não a caminhos de arquivo ou outros argumentos. Para filtrar por caminho de arquivo, verifique `tool_input.file_path` dentro de seu hook:

```typescript theme={null}
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;
  if (!filePath?.endsWith(".md")) return {}; // Skip non-markdown files
  // Process markdown files...
  return {};
};
```

<h3 id="hook-timeout">
  Timeout de hook
</h3>

Claude Code executa cada callback com um timeout, que você define em segundos com o campo `timeout` em seu `HookMatcher`. Quando você não define um, Claude Code usa o padrão do evento: 600 segundos para a maioria dos eventos, 30 segundos para `UserPromptSubmit`, `PreModelSwitch` e `PostModelSwitch`, e 10 segundos para `MessageDisplay`. Claude Code executa callbacks `SessionEnd` durante o desligamento sob o orçamento de timeout mais curto de [SessionEnd](/docs/pt/hooks#sessionend-input), 1,5 segundos por padrão.

Quando um callback excede seu timeout, Claude Code o cancela e descarta sua saída, e a sessão continua em vez de travar. O que acontece a seguir depende do evento:

* `PreToolUse`: Claude Code não executa a chamada de ferramenta, Claude recebe um resultado de ferramenta informando que o hook não respondeu antes de seu timeout, e a volta continua. Se outro hook `PreToolUse` retornou uma negação explícita, Claude recebe essa negação em vez do erro de timeout. Antes da v2.1.210, Claude Code relatava o timeout a Claude como uma rejeição do usuário, o que fazia sessões autônomas pararem e aguardarem entrada.
* `PostToolUse` e `PostToolUseFailure`: Claude Code mantém o resultado da ferramenta e a volta continua.
* `UserPromptSubmit` e [`UserPromptExpansion`](/docs/pt/hooks#userpromptexpansion): Claude Code bloqueia o prompt com uma mensagem nomeando o hook e o timeout, e a sessão continua. Como um callback nesses eventos pode atuar como uma porta de política, Claude Code nunca deixa um prompt com timeout passar sem ser verificado. Antes da v2.1.208, Claude Code terminava a consulta com `error_during_execution` quando um callback nesses eventos expirava.
* `Stop` e `SubagentStop`: o callback com timeout conta como retornando nenhuma decisão. O agente ou subagente para como se esse callback o tivesse permitido, e uma decisão de seus outros hooks no evento ainda se aplica. Antes do Claude Code v2.1.273, um callback `Stop` ou `SubagentStop` com timeout contava como uma execução de hook falhada, e Claude Code descartava as decisões de seus outros hooks no evento.
* `SessionStart`: o callback com timeout conta como retornando nenhuma saída, e a sessão continua com a saída de seus outros hooks `SessionStart`.
* `PreModelSwitch`: Claude Code bloqueia a mudança de modelo. Um hook que não responde não aprovou a mudança.
* Outros eventos, como `Notification`, `PreCompact` e `PostModelSwitch`: Claude Code registra a falha e continua.

A primeira vez que um callback `Stop` ou `SessionStart` expira na sessão principal, Claude Code também adiciona uma [`SDKInformationalMessage`](/docs/pt/agent-sdk/typescript#sdkinformationalmessage) ao fluxo de mensagens dizendo que o aplicativo que dirige a sessão não respondeu. Timeouts posteriores não repetem essa mensagem enquanto seu aplicativo permanecer sem resposta.

Se você interromper a consulta enquanto um callback está pendente, Claude Code cancela a chamada de ferramenta pendente. Antes da v2.1.208, a chamada de ferramenta ainda poderia prosseguir se você interrompesse durante um callback `PreToolUse` pendente.

Se seu callback precisar de mais tempo, defina um `timeout` mais alto em seu `HookMatcher`. Em TypeScript, use o `AbortSignal` do terceiro argumento de callback para lidar com cancelamento graciosamente quando o timeout dispara.

<h3 id="tool-blocked-unexpectedly">
  Ferramenta bloqueada inesperadamente
</h3>

* Verifique todos os hooks `PreToolUse` para retornos `permissionDecision: 'deny'`
* Adicione registro aos seus hooks para ver qual `permissionDecisionReason` eles estão retornando
* Verifique se padrões de matcher não são muito amplos: um matcher vazio corresponde a todas as ferramentas

<h3 id="modified-input-not-applied">
  Entrada modificada não aplicada
</h3>

* Certifique-se de que `updatedInput` está dentro de `hookSpecificOutput`, não no nível superior:

  ```typescript theme={null}
  return {
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "allow",
      updatedInput: { command: "new command" }
    }
  };
  ```

* Não emparelhe `updatedInput` com `permissionDecision: 'defer'`, que descarta a entrada modificada. Omitir `permissionDecision` é aceitável: a entrada modificada ainda se aplica através da avaliação de permissão normal. Você também pode retornar `'allow'` para aprovar automaticamente a entrada modificada ou `'ask'` para mostrá-la ao usuário para aprovação

* Inclua `hookEventName` em `hookSpecificOutput` para identificar qual tipo de hook a saída é

<h3 id="session-hooks-not-available-in-python">
  Hooks de sessão não disponíveis em Python
</h3>

`SessionStart` e `SessionEnd` podem ser registrados como hooks de callback do SDK em TypeScript, mas não estão disponíveis no SDK Python porque seu tipo `HookEvent` os omite. Em Python, eles estão disponíveis apenas como [hooks de comando shell](/docs/pt/hooks#hook-events) definidos em arquivos de configuração como `.claude/settings.json`. Para carregar hooks de comando shell de sua aplicação SDK, inclua a fonte de configuração apropriada com [`setting_sources`](/docs/pt/agent-sdk/python#settingsource) ou [`settingSources`](/docs/pt/agent-sdk/typescript#settingsource):

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["project"],  # Loads .claude/settings.json including hooks
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    settingSources: ["project"] // Loads .claude/settings.json including hooks
  };
  ```
</CodeGroup>

Para executar lógica de inicialização como um callback do SDK Python, use a primeira mensagem de `client.receive_response()` como seu gatilho.

<h3 id="subagent-permission-prompts-multiplying">
  Prompts de permissão de subagente se multiplicando
</h3>

Ao gerar múltiplos subagentes, cada um pode solicitar permissões separadamente para suas próprias chamadas de ferramenta. Para evitar prompts repetidos, use hooks `PreToolUse` para aprovar automaticamente ferramentas específicas, ou configure regras de permissão, que subagentes [herdam da conversa pai](/docs/pt/sub-agents#permission-modes).

<h3 id="recursive-hook-loops-with-subagents">
  Loops recursivos de hook com subagentes
</h3>

Um hook `UserPromptSubmit` que gera subagentes pode criar loops infinitos se esses subagentes acionarem o mesmo hook. Para evitar isso:

* Use uma variável compartilhada ou estado de sessão para rastrear se você já está dentro de um subagente
* Escopo hooks para executar apenas para a sessão de agente de nível superior

<h3 id="systemmessage-not-appearing-in-output">
  systemMessage não aparecendo na saída
</h3>

O campo `systemMessage` mostra uma mensagem ao usuário, não ao modelo. No Claude Code v2.1.227 ou posterior, o `systemMessage` de um hook pode aparecer no fluxo de mensagens como uma [`SDKInformationalMessage`](/docs/pt/agent-sdk/typescript#sdkinformationalmessage). Se aparece ou não depende do evento. Cada [seção de evento](/docs/pt/hooks#hook-events) na página de hooks diz como a saída aparece. Para passar contexto ao modelo, retorne [`additionalContext`](/docs/pt/hooks#add-context-for-claude).

Antes da v2.1.227, o SDK expunha a saída de hook no fluxo de mensagens apenas para hooks `SessionStart` e `Setup`. Para qualquer outro evento, a saída aparecia apenas nos eventos de ciclo de vida que [`includeHookEvents`](/docs/pt/agent-sdk/typescript#options) (`include_hook_events` em Python) adiciona. A entrada dessa opção cobre quais eventos de ciclo de vida cada evento de hook produz.

Se você precisar expor decisões de hook para sua aplicação de forma confiável, registre-as separadamente ou use um canal de saída dedicado.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Referência de hooks do Claude Code](/docs/pt/hooks): esquemas JSON de entrada/saída completos, documentação de eventos e padrões de matcher
* [Guia de hooks do Claude Code](/docs/pt/hooks-guide): exemplos de hooks de comando shell e passo a passo
* [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript): tipos de hook, definições de entrada/saída e opções de configuração
* [Referência do SDK Python](/docs/pt/agent-sdk/python): tipos de hook, definições de entrada/saída e opções de configuração
* [Permissões](/docs/pt/agent-sdk/permissions): controlar o que seu agente pode fazer
* [Ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools): construir ferramentas para estender capacidades do agente
