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

Este guia cobre como hooks funcionam e como configurá-los, com exemplos para padrões comuns como bloquear ferramentas, modificar entradas e encaminhar notificações.

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
          await client.query("Update the database configuration")
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
    prompt: "Update the database configuration",
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

<h2 id="available-hooks">
  Hooks disponíveis
</h2>

O SDK fornece hooks para diferentes estágios de execução do agente. Alguns hooks estão disponíveis em ambos os SDKs, enquanto outros são apenas para TypeScript.

| Evento de Hook                                         | SDK Python | SDK TypeScript | O que o dispara                                                                                             | Caso de uso de exemplo                                                                    |
| ------------------------------------------------------ | ---------- | -------------- | ----------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `PreToolUse`                                           | Sim        | Sim            | Solicitação de chamada de ferramenta (pode bloquear ou modificar)                                           | Bloquear comandos shell perigosos                                                         |
| `PostToolUse`                                          | Sim        | Sim            | Resultado de execução de ferramenta                                                                         | Registrar todas as alterações de arquivo na trilha de auditoria                           |
| `PostToolUseFailure`                                   | Sim        | Sim            | Falha na execução de ferramenta                                                                             | Lidar ou registrar erros de ferramenta                                                    |
| `PostToolBatch`                                        | Não        | Sim            | Um lote completo de chamadas de ferramenta é resolvido, uma vez por lote antes da próxima chamada de modelo | Injetar convenções uma vez para todo o lote                                               |
| `UserPromptSubmit`                                     | Sim        | Sim            | Envio de prompt do usuário                                                                                  | Injetar contexto adicional em prompts                                                     |
| [`UserPromptExpansion`](/docs/pt/hooks#userpromptexpansion) | Não        | Sim            | Um comando digitado pelo usuário se expande em um prompt antes de chegar ao Claude                          | Bloquear um comando de invocação direta ou adicionar contexto quando uma skill é digitada |
| `MessageDisplay`                                       | Não        | Sim            | Uma mensagem do assistente com texto é concluída, uma vez por mensagem com o texto completo da mensagem     | Redigir ou reformatar o texto exibido sem alterar a transcrição                           |
| `Stop`                                                 | Sim        | Sim            | Parada de execução do agente                                                                                | Salvar estado da sessão antes de sair                                                     |
| `SubagentStart`                                        | Sim        | Sim            | Inicialização de subagente                                                                                  | Rastrear geração de tarefas paralelas                                                     |
| `SubagentStop`                                         | Sim        | Sim            | Conclusão de subagente                                                                                      | Agregar resultados de tarefas paralelas                                                   |
| `PreCompact`                                           | Sim        | Sim            | Solicitação de compactação de conversa                                                                      | Arquivar transcrição completa antes de resumir                                            |
| `PermissionRequest`                                    | Sim        | Sim            | Diálogo de permissão seria exibido                                                                          | Manipulação de permissão personalizada                                                    |
| `SessionStart`                                         | Não        | Sim            | Inicialização de sessão                                                                                     | Inicializar registro e telemetria                                                         |
| `SessionEnd`                                           | Não        | Sim            | Encerramento de sessão                                                                                      | Limpar recursos temporários                                                               |
| `Notification`                                         | Sim        | Sim            | Mensagens de status do agente                                                                               | Enviar atualizações de status do agente para Slack ou PagerDuty                           |
| `Setup`                                                | Não        | Sim            | Configuração/manutenção de sessão                                                                           | Executar tarefas de inicialização                                                         |
| `TeammateIdle`                                         | Não        | Sim            | Colega fica ocioso                                                                                          | Reatribuir trabalho ou notificar                                                          |
| `TaskCompleted`                                        | Não        | Sim            | Tarefa em segundo plano é concluída                                                                         | Agregar resultados de tarefas paralelas                                                   |
| `ConfigChange`                                         | Não        | Sim            | Arquivo de configuração muda                                                                                | Recarregar configurações dinamicamente                                                    |
| `WorktreeCreate`                                       | Não        | Sim            | Git worktree criado                                                                                         | Rastrear espaços de trabalho isolados                                                     |
| `WorktreeRemove`                                       | Não        | Sim            | Git worktree removido                                                                                       | Limpar recursos de espaço de trabalho                                                     |

<h2 id="configure-hooks">
  Configurar hooks
</h2>

Para configurar um hook, passe-o no campo `hooks` de suas opções de agente (`ClaudeAgentOptions` em Python, o objeto `options` em TypeScript):

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

Use matchers para filtrar quando seus callbacks são acionados. O campo `matcher` corresponde a um valor diferente dependendo do tipo de evento de hook. Por exemplo, hooks baseados em ferramentas correspondem ao nome da ferramenta, enquanto hooks `Notification` correspondem ao tipo de notificação. Veja a [referência de hooks do Claude Code](/docs/pt/hooks#matcher-patterns) para a lista completa de valores de matcher para cada tipo de evento.

Os matchers do SDK seguem as mesmas regras que [matchers em arquivos de configuração](/docs/pt/hooks#matcher-patterns). Um matcher contendo apenas letras, dígitos, `_`, `-`, espaços, `,` e `|` é comparado como uma string exata, com alternativas separadas por `|` ou `,` e espaço em branco opcional ao redor, então `Write|Edit` e `Write, Edit` correspondem exatamente a essas duas ferramentas e `code-reviewer` corresponde apenas a esse tipo de agente. Um matcher de `*`, uma string vazia, ou omitir o matcher inteiramente corresponde a cada ocorrência do evento.

Um matcher contendo qualquer outro caractere é avaliado como uma expressão regular sem âncora, então `^mcp__` corresponde a cada ferramenta MCP e `Edit.*` corresponde tanto a `Edit` quanto a `NotebookEdit`. Envolva uma expressão regular em `^` e `$` quando você precisar de uma correspondência de string inteira.

Um matcher como `mcp__memory` ou `mcp__brave-search` contém apenas caracteres de correspondência exata, então é comparado como uma string exata e não corresponde a nenhuma ferramenta; use `mcp__memory__.*` para corresponder a cada ferramenta desse servidor.

Hífens no conjunto de correspondência exata requerem um runtime do Claude Code v2.1.195 ou posterior. Em versões anteriores, um nome com hífen como `code-reviewer` é avaliado como uma expressão regular sem âncora e deve ser ancorado como `^code-reviewer$` para corresponder exatamente.

| Opção     | Tipo             | Padrão      | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| --------- | ---------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `matcher` | `string`         | `undefined` | Padrão correspondido contra o campo de filtro do evento, seguindo as regras de comparação acima. Para hooks de ferramenta, este é o nome da ferramenta. As ferramentas integradas incluem `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Agent` e outras (veja [Tipos de Entrada de Ferramenta](/docs/pt/agent-sdk/typescript#tool-input-types) para a lista completa). Ferramentas MCP usam o padrão `mcp__<server>__<action>`. |
| `hooks`   | `HookCallback[]` | -           | Obrigatório. Array de funções de callback a executar quando o padrão corresponde                                                                                                                                                                                                                                                                                                                                                           |
| `timeout` | `number`         | `60`        | Timeout em segundos                                                                                                                                                                                                                                                                                                                                                                                                                        |

Use o padrão `matcher` para direcionar ferramentas específicas sempre que possível. Um matcher com `'Bash'` é executado apenas para comandos Bash, enquanto omitir o padrão executa seus callbacks para cada ocorrência do evento.

Para hooks baseados em ferramentas, matchers filtram apenas pelo nome da ferramenta, não por caminhos de arquivo ou outros argumentos. Para filtrar por caminho de arquivo, verifique `tool_input.file_path` dentro de seu callback.

<Tip>
  **Descobrindo nomes de ferramentas:** Veja [Tipos de Entrada de Ferramenta](/docs/pt/agent-sdk/typescript#tool-input-types) para a lista completa de nomes de ferramentas integradas, ou adicione um hook sem um matcher para registrar todas as chamadas de ferramenta que sua sessão faz.

  **Nomenclatura de ferramentas MCP:** Ferramentas MCP sempre começam com `mcp__` seguido pelo nome do servidor e ação: `mcp__<server>__<action>`. Por exemplo, se você configurar um servidor chamado `playwright`, suas ferramentas serão nomeadas `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click` e assim por diante. O nome do servidor vem da chave que você usa na configuração `mcpServers`.
</Tip>

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

* **Campos de nível superior** funcionam da mesma forma em cada evento: `systemMessage` mostra uma mensagem ao usuário, e `continue` (`continue_` em Python) determina se o agente continua executando após este hook.
* **`hookSpecificOutput`** controla a operação atual. Os campos dentro dependem do tipo de evento de hook. Para hooks `PreToolUse`, é aqui que você define `permissionDecision` (`"allow"`, `"deny"`, `"ask"` ou `"defer"`), `permissionDecisionReason` e `updatedInput`. Retornar `"defer"` encerra a consulta para que você possa [retomá-la depois](/docs/pt/hooks#defer-a-tool-call-for-later). Para hooks `PostToolUse`, você pode definir `additionalContext` para anexar informações ao resultado da ferramenta. Para substituir a saída da ferramenta antes de Claude vê-la, defina `updatedToolOutput`, que funciona para qualquer ferramenta em ambos os SDKs. O campo mais antigo `updatedMCPToolOutput` substitui apenas a saída de ferramentas MCP e está descontinuado.

Retorne `{}` para permitir a operação sem alterações. Hooks de callback do SDK usam o mesmo formato de saída JSON que [hooks de comando shell do Claude Code](/docs/pt/hooks#json-output), que documenta cada campo e opção específica do evento. Para as definições de tipo do SDK, veja as referências do SDK [TypeScript](/docs/pt/agent-sdk/typescript#synchookjsonoutput) e [Python](/docs/pt/agent-sdk/python#synchookjsonoutput).

<Note>
  Quando múltiplos hooks ou regras de permissão se aplicam, `deny` tem prioridade sobre `defer`, que tem prioridade sobre `ask`, que tem prioridade sobre `allow`. Se qualquer hook retornar `deny`, a operação é bloqueada independentemente de outros hooks.
</Note>

<h4 id="asynchronous-output">
  Saída assíncrona
</h4>

Por padrão, o agente aguarda seu hook retornar antes de prosseguir. Se seu hook realiza um efeito colateral, como registro ou envio de webhook, e não precisa influenciar o comportamento do agente, você pode retornar uma saída assíncrona. Isso diz ao agente para continuar imediatamente sem aguardar o hook terminar:

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
  Ao usar `updatedInput`, você também deve incluir `permissionDecision: 'allow'` para aprovar automaticamente a entrada modificada ou `permissionDecision: 'ask'` para mostrá-la ao usuário. Com `'defer'`, `updatedInput` é ignorado. Sempre retorne um novo objeto em vez de mutar o `tool_input` original.
</Note>

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

* Uma lista exata separada por pipe (`Write|Edit|Delete`) dispara `file_security_hook` apenas para ferramentas de modificação de arquivo.
* Uma regex (`^mcp__`) dispara `mcp_audit_hook` para qualquer ferramenta MCP cujo nome começa com `mcp__`.
* Um matcher omitido dispara `global_logger` para cada chamada de ferramenta independentemente do nome.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              # Match file modification tools
              HookMatcher(matcher="Write|Edit|Delete", hooks=[file_security_hook]),
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
        { matcher: "Write|Edit|Delete", hooks: [fileSecurityHook] },

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

Hooks podem realizar operações assíncronas como requisições HTTP. Capture erros dentro de seu hook em vez de deixá-los se propagar, pois uma exceção não tratada pode interromper o agente.

Este exemplo envia um webhook após cada ferramenta ser concluída, registrando qual ferramenta foi executada e quando. O hook captura erros para que um webhook falhado não interrompa o agente:

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
          # Log the error but don't raise. A failed webhook shouldn't stop the agent
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
      // Don't re-throw. A failed webhook shouldn't stop the agent
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

<h3 id="forward-notifications-to-slack">
  Encaminhar notificações para Slack
</h3>

Use hooks `Notification` para receber notificações do sistema do agente e encaminhá-las para serviços externos. Notificações são disparadas para tipos de evento como:

* `permission_prompt` quando Claude precisa de permissão
* `idle_prompt` quando Claude está aguardando entrada
* `auth_success` quando a autenticação é concluída
* `elicitation_dialog`, `elicitation_complete` e `elicitation_response` para fluxos de elicitação de entrada do usuário

Cada notificação inclui um campo `message` com uma descrição legível por humanos e opcionalmente um `title`.

Este exemplo encaminha cada notificação para um canal Slack. Requer uma [URL de webhook de entrada do Slack](https://api.slack.com/messaging/webhooks), que você cria adicionando um app ao seu espaço de trabalho Slack e habilitando webhooks de entrada:

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

* Aumente o valor `timeout` na configuração `HookMatcher`
* Use o `AbortSignal` do terceiro argumento de callback para lidar com cancelamento graciosamente em TypeScript

Um callback `UserPromptSubmit` ou [`UserPromptExpansion`](/docs/pt/hooks#userpromptexpansion) que excede seu timeout bloqueia esse prompt com uma mensagem de timeout e a sessão continua. Interromper a consulta enquanto um callback está pendente cancela a chamada de ferramenta pendente. Antes da v2.1.208, um timeout de callback nesses eventos terminava a consulta com `error_during_execution`, e uma interrupção durante um callback `PreToolUse` pendente poderia deixar a chamada de ferramenta prosseguir.

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

* Retorne `permissionDecision: 'allow'` para aprovar automaticamente a entrada modificada, ou `'ask'` para mostrá-la ao usuário para aprovação

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

Ao gerar múltiplos subagentes, cada um pode solicitar permissões separadamente. Subagentes não herdam automaticamente permissões do agente pai. Para evitar prompts repetidos, use hooks `PreToolUse` para aprovar automaticamente ferramentas específicas ou configure regras de permissão que se aplicam a sessões de subagente.

<h3 id="recursive-hook-loops-with-subagents">
  Loops recursivos de hook com subagentes
</h3>

Um hook `UserPromptSubmit` que gera subagentes pode criar loops infinitos se esses subagentes acionarem o mesmo hook. Para evitar isso:

* Verifique um indicador de subagente na entrada do hook antes de gerar
* Use uma variável compartilhada ou estado de sessão para rastrear se você já está dentro de um subagente
* Escopo hooks para executar apenas para a sessão de agente de nível superior

<h3 id="systemmessage-not-appearing-in-output">
  systemMessage não aparecendo na saída
</h3>

O campo `systemMessage` mostra uma mensagem ao usuário, não ao modelo. Por padrão, o SDK expõe a saída de hook no fluxo de mensagens apenas para hooks `SessionStart` e `Setup`, portanto uma mensagem de qualquer outro evento de hook não aparece a menos que você defina `includeHookEvents` (`include_hook_events` em Python). Para passar contexto ao modelo, retorne [`additionalContext`](/docs/pt/hooks#add-context-for-claude).

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
