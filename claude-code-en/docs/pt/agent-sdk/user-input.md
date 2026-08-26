> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lidar com aprovações e entrada do usuário

> Apresente as solicitações de aprovação e perguntas de esclarecimento do Claude aos usuários e retorne suas decisões ao SDK.

Ao trabalhar em uma tarefa, Claude às vezes precisa verificar com os usuários. Pode precisar de permissão antes de excluir arquivos ou precisar perguntar qual banco de dados usar para um novo projeto. Seu aplicativo precisa apresentar essas solicitações aos usuários para que Claude possa continuar com sua entrada.

Claude solicita entrada do usuário em duas situações: quando precisa de **permissão para usar uma ferramenta** (como excluir arquivos ou executar comandos) e quando tem **perguntas de esclarecimento** (por meio da ferramenta `AskUserQuestion`). Ambas acionam seu callback `canUseTool`, que pausa a execução até que você retorne uma resposta. Isso é diferente dos turnos de conversa normais, onde Claude termina e aguarda sua próxima mensagem.

Para perguntas de esclarecimento, Claude gera as perguntas e opções. Seu papel é apresentá-las aos usuários e retornar suas seleções. Você não pode adicionar suas próprias perguntas a este fluxo; se precisar perguntar algo aos usuários, faça isso separadamente na lógica do seu aplicativo.

O callback pode permanecer pendente indefinidamente. A execução permanece pausada até que seu callback retorne, e o SDK apenas cancela a espera quando a própria consulta é cancelada. Se um usuário puder levar mais tempo para responder do que seu processo pode razoavelmente permanecer em execução, retorne a decisão do [hook `defer`](/docs/pt/hooks#defer-a-tool-call-for-later), que permite que o processo saia e retome mais tarde a partir da sessão persistida.

Este guia mostra como detectar cada tipo de solicitação e responder apropriadamente.

<h2 id="detect-when-claude-needs-input">
  Detectar quando Claude precisa de entrada
</h2>

Passe um callback `canUseTool` nas opções de sua consulta. O callback é acionado sempre que Claude precisa de entrada do usuário, recebendo o nome da ferramenta e a entrada como argumentos:

<CodeGroup>
  ```python Python theme={null}
  async def handle_tool_request(tool_name, input_data, context):
      # Solicite ao usuário e retorne permitir ou negar
      ...


  options = ClaudeAgentOptions(can_use_tool=handle_tool_request)
  ```

  ```typescript TypeScript theme={null}
  async function handleToolRequest(toolName, input, options) {
    // options inclui { signal: AbortSignal, suggestions?: PermissionUpdate[] }
    // Solicite ao usuário e retorne permitir ou negar
  }

  const options = { canUseTool: handleToolRequest };
  ```
</CodeGroup>

O callback é acionado em dois casos:

1. **Ferramenta precisa de aprovação**: Claude quer usar uma ferramenta que não é aprovada automaticamente por uma [regra de permissão](/docs/pt/agent-sdk/permissions) ou modo de permissão. Verifique `tool_name` para a ferramenta (por exemplo, `"Bash"`, `"Write"`).
2. **Claude faz uma pergunta**: Claude chama a ferramenta `AskUserQuestion`. Verifique se `tool_name == "AskUserQuestion"` para tratá-la diferentemente. Se você especificar um array `tools`, inclua `AskUserQuestion` para que isso funcione. Veja [Lidar com perguntas de esclarecimento](#handle-clarifying-questions) para detalhes.

<Warning>
  **O callback nunca é acionado para ferramentas aprovadas automaticamente.** Qualquer aprovação anterior no [fluxo de avaliação de permissões](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated), uma regra de permissão ou um modo como `acceptEdits` ou `bypassPermissions`, resolve a chamada antes que `canUseTool` seja consultado. Se você listar uma ferramenta diretamente em `allowed_tools`, uma verificação `canUseTool` para essa ferramenta nunca é executada a menos que uma regra de pergunta ou modo `plan` redirecione a chamada de volta para um prompt. Para lógica que deve se aplicar a cada chamada de ferramenta, use um [hook `PreToolUse`](/docs/pt/agent-sdk/hooks), que é executado antes do resto do fluxo e pode permitir, negar ou modificar solicitações.

  `AskUserQuestion`, ferramentas MCP marcadas como [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool), e ferramentas de conector [que sua organização configurou como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) chegam ao callback mesmo quando uma regra de permissão corresponde. No modo `dontAsk` essas chamadas são negadas em vez disso, sem invocar o callback.
</Warning>

Você também pode usar o [hook `PermissionRequest`](/docs/pt/agent-sdk/hooks#available-hooks) para enviar notificações externas (Slack, email, push) quando Claude está aguardando aprovação.

<h2 id="handle-tool-approval-requests">
  Lidar com solicitações de aprovação de ferramentas
</h2>

Depois de passar um callback `canUseTool` nas opções de sua consulta, ele é acionado quando Claude quer usar uma ferramenta que nada anterior no fluxo de permissão aprovou. Seu callback recebe três argumentos:

| Argumento                           | Descrição                                                                                                                                                                                                                                                                                                                                       |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `toolName`                          | O nome da ferramenta que Claude quer usar (por exemplo, `"Bash"`, `"Write"`, `"Edit"`)                                                                                                                                                                                                                                                          |
| `input`                             | Os parâmetros que Claude está passando para a ferramenta. O conteúdo varia por ferramenta.                                                                                                                                                                                                                                                      |
| `options` (TS) / `context` (Python) | Contexto adicional incluindo `suggestions` opcional (entradas `PermissionUpdate` propostas para evitar re-solicitação) e um sinal de cancelamento. Em TypeScript, `signal` é um `AbortSignal`; em Python, o campo de sinal é reservado para uso futuro. Veja [`ToolPermissionContext`](/docs/pt/agent-sdk/python#toolpermissioncontext) para Python. |

O objeto `input` contém parâmetros específicos da ferramenta. Exemplos comuns:

| Ferramenta | Campos de entrada                       |
| ---------- | --------------------------------------- |
| `Bash`     | `command`, `description`, `timeout`     |
| `Write`    | `file_path`, `content`                  |
| `Edit`     | `file_path`, `old_string`, `new_string` |
| `Read`     | `file_path`, `offset`, `limit`          |

Veja a referência do SDK para esquemas de entrada completos: [Python](/docs/pt/agent-sdk/python#tool-input%2Foutput-types) | [TypeScript](/docs/pt/agent-sdk/typescript#tool-input-types).

Você pode exibir essas informações ao usuário para que ele possa decidir se permite ou rejeita a ação, e então retornar a resposta apropriada.

O exemplo a seguir pede ao Claude para criar e excluir um arquivo de teste. Quando Claude tenta cada operação, o callback imprime a solicitação de ferramenta no terminal e solicita aprovação s/n.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import (
      HookMatcher,
      PermissionResultAllow,
      PermissionResultDeny,
      ToolPermissionContext,
  )


  async def can_use_tool(
      tool_name: str, input_data: dict, context: ToolPermissionContext
  ) -> PermissionResultAllow | PermissionResultDeny:
      # Exiba a solicitação de ferramenta
      print(f"\nTool: {tool_name}")
      if tool_name == "Bash":
          print(f"Command: {input_data.get('command')}")
          if input_data.get("description"):
              print(f"Description: {input_data.get('description')}")
      else:
          print(f"Input: {input_data}")

      # Obtenha aprovação do usuário
      response = input("Allow this action? (y/n): ")

      # Retorne permitir ou negar com base na resposta do usuário
      if response.lower() == "y":
          # Permitir: ferramenta executa com a entrada original (ou modificada)
          return PermissionResultAllow(updated_input=input_data)
      else:
          # Negar: ferramenta não executa, Claude vê a mensagem
          return PermissionResultDeny(message="User denied this action")


  # Solução alternativa necessária: hook fictício mantém o fluxo aberto para can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Create a test file in /tmp and then delete it",
          },
      }


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline";

  // Auxiliar para solicitar entrada do usuário no terminal
  function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
    return new Promise((resolve) =>
      rl.question(question, (answer) => {
        rl.close();
        resolve(answer);
      })
    );
  }

  for await (const message of query({
    prompt: "Create a test file in /tmp and then delete it",
    options: {
      canUseTool: async (toolName, input) => {
        // Exiba a solicitação de ferramenta
        console.log(`\nTool: ${toolName}`);
        if (toolName === "Bash") {
          console.log(`Command: ${input.command}`);
          if (input.description) console.log(`Description: ${input.description}`);
        } else {
          console.log(`Input: ${JSON.stringify(input, null, 2)}`);
        }

        // Obtenha aprovação do usuário
        const response = await prompt("Allow this action? (y/n): ");

        // Retorne permitir ou negar com base na resposta do usuário
        if (response.toLowerCase() === "y") {
          // Permitir: ferramenta executa com a entrada original (ou modificada)
          return { behavior: "allow", updatedInput: input };
        } else {
          // Negar: ferramenta não executa, Claude vê a mensagem
          return { behavior: "deny", message: "User denied this action" };
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<Note>
  Em Python, `can_use_tool` requer [modo de streaming](/docs/pt/agent-sdk/streaming-vs-single-mode). Quando você passa um fluxo de mensagens finito através de `query(prompt=generator)` ou `ClaudeSDKClient.connect(prompt=async_iterable)`, o SDK fecha o fluxo de entrada após a última mensagem, antes que o callback de permissão possa ser invocado, a menos que um hook registrado ou servidor MCP em processo o mantenha aberto. O exemplo acima o mantém aberto com um hook `PreToolUse` que retorna `{"continue_": True}`. Conectar sem prompt e enviar mensagens através de `ClaudeSDKClient.query()` mantém o fluxo aberto por si só e não precisa de hook.
</Note>

Este exemplo usa um fluxo s/n onde qualquer entrada diferente de `s` é tratada como uma negação. Na prática, você pode construir uma interface de usuário mais rica que permite aos usuários modificar a solicitação, fornecer feedback ou redirecionar Claude completamente. Veja [Responder a solicitações de ferramentas](#respond-to-tool-requests) para todas as maneiras que você pode responder.

<h3 id="respond-to-tool-requests">
  Responder a solicitações de ferramentas
</h3>

Seu callback retorna um de dois tipos de resposta:

| Resposta     | Python                                     | TypeScript                            |
| ------------ | ------------------------------------------ | ------------------------------------- |
| **Permitir** | `PermissionResultAllow(updated_input=...)` | `{ behavior: "allow", updatedInput }` |
| **Negar**    | `PermissionResultDeny(message=...)`        | `{ behavior: "deny", message }`       |

Ao permitir, a ferramenta executa com a entrada que Claude solicitou, a menos que você retorne uma entrada modificada, `updatedInput` em TypeScript ou `updated_input` em Python. Antes da v2.1.207, Claude Code rejeitava um resultado de permissão que omitia `updatedInput` e negava a chamada de ferramenta com um erro de validação.

Ao negar, forneça uma mensagem explicando por quê. Claude vê esta mensagem e pode ajustar sua abordagem.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk.types import PermissionResultAllow, PermissionResultDeny

  # Permita que a ferramenta execute
  return PermissionResultAllow(updated_input=input_data)

  # Bloqueie a ferramenta
  return PermissionResultDeny(message="User rejected this action")
  ```

  ```typescript TypeScript theme={null}
  // Permita que a ferramenta execute
  return { behavior: "allow", updatedInput: input };

  // Bloqueie a ferramenta
  return { behavior: "deny", message: "User rejected this action" };
  ```
</CodeGroup>

Além de permitir ou negar, você pode modificar a entrada da ferramenta ou fornecer contexto que ajude Claude a ajustar sua abordagem:

* **Aprovar**: deixe a ferramenta executar conforme Claude solicitou
* **Aprovar com alterações**: modifique a entrada antes da execução (por exemplo, sanitize caminhos, adicione restrições)
* **Aprovar e lembrar**: repita uma regra de permissão sugerida para que chamadas correspondentes ignorem o prompt na próxima vez
* **Rejeitar**: bloqueie a ferramenta e diga ao Claude por quê
* **Sugerir alternativa**: bloqueie mas guie Claude para o que o usuário quer em vez disso
* **Redirecionar completamente**: use [entrada de streaming](/docs/pt/agent-sdk/streaming-vs-single-mode) para enviar ao Claude uma instrução completamente nova

<Tabs>
  <Tab title="Aprovar">
    O usuário aprova a ação como está. Passe a `input` do seu callback inalterada e a ferramenta executa exatamente como Claude solicitou.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          print(f"Claude wants to use {tool_name}")
          approved = await ask_user("Allow this action?")

          if approved:
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        console.log(`Claude wants to use ${toolName}`);
        const approved = await askUser("Allow this action?");

        if (approved) {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Aprovar com alterações">
    O usuário aprova mas quer modificar a solicitação primeiro. Você pode alterar a entrada antes da ferramenta executar. Claude vê o resultado mas não é informado de que você alterou nada. Útil para sanitizar parâmetros, adicionar restrições ou escopar acesso.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash":
              # Usuário aprovou, mas escope todos os comandos para sandbox
              sandboxed_input = {**input_data}
              sandboxed_input["command"] = input_data["command"].replace(
                  "/tmp", "/tmp/sandbox"
              )
              return PermissionResultAllow(updated_input=sandboxed_input)
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash") {
          // Usuário aprovou, mas escope todos os comandos para sandbox
          const sandboxedInput = {
            ...input,
            command: input.command.replace("/tmp", "/tmp/sandbox")
          };
          return { behavior: "allow", updatedInput: sandboxedInput };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Aprovar e lembrar">
    O usuário aprova e não quer ser perguntado novamente para este tipo de chamada. O terceiro argumento de callback carrega `suggestions`, uma matriz de entradas [`PermissionUpdate`](/docs/pt/agent-sdk/typescript#permissionupdate) prontas. Repita uma de volta em `updatedPermissions` para aplicá-la. Uma sugestão com o destino `localSettings` escreve a regra em `.claude/settings.local.json` para que futuras sessões ignorem o prompt para chamadas correspondentes.

    O exemplo Python requer `claude-agent-sdk` 0.1.80 ou posterior.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          choice = await ask_user(f"Allow {tool_name}?", ["once", "always", "no"])

          if choice == "always":
              persist = [
                  s for s in context.suggestions if s.destination == "localSettings"
              ]
              return PermissionResultAllow(
                  updated_input=input_data, updated_permissions=persist
              )
          if choice == "once":
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input, { suggestions = [] }) => {
        const choice = await askUser(`Allow ${toolName}?`, ["once", "always", "no"]);

        if (choice === "always") {
          const persist = suggestions.filter(
            (s) => s.destination === "localSettings"
          );
          return {
            behavior: "allow",
            updatedInput: input,
            updatedPermissions: persist
          };
        }
        if (choice === "once") {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Rejeitar">
    O usuário não quer que esta ação aconteça. Bloqueie a ferramenta e forneça uma mensagem explicando por quê. Claude vê esta mensagem e pode tentar uma abordagem diferente.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          approved = await ask_user(f"Allow {tool_name}?")

          if not approved:
              return PermissionResultDeny(message="User rejected this action")
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        const approved = await askUser(`Allow ${toolName}?`);

        if (!approved) {
          return {
            behavior: "deny",
            message: "User rejected this action"
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Sugerir alternativa">
    O usuário não quer esta ação específica, mas tem uma ideia diferente. Bloqueie a ferramenta e inclua orientação em sua mensagem. Claude lerá isso e decidirá como proceder com base em seu feedback.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash" and "rm" in input_data.get("command", ""):
              # Usuário não quer deletar, sugira arquivar em vez disso
              return PermissionResultDeny(
                  message="User doesn't want to delete files. They asked if you could compress them into an archive instead."
              )
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash" && input.command.includes("rm")) {
          // Usuário não quer deletar, sugira arquivar em vez disso
          return {
            behavior: "deny",
            message:
              "User doesn't want to delete files. They asked if you could compress them into an archive instead."
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Redirecionar completamente">
    Para uma mudança completa de direção (não apenas um empurrão), use [entrada de streaming](/docs/pt/agent-sdk/streaming-vs-single-mode) para enviar ao Claude uma nova instrução diretamente. Isso ignora a solicitação de ferramenta atual e dá ao Claude instruções completamente novas para seguir.
  </Tab>
</Tabs>

<h2 id="handle-clarifying-questions">
  Lidar com perguntas de esclarecimento
</h2>

Quando Claude precisa de mais direção em uma tarefa com múltiplas abordagens válidas, ele chama a ferramenta `AskUserQuestion`. Isso aciona seu callback `canUseTool` com `toolName` definido como `AskUserQuestion`. A entrada contém as perguntas do Claude como opções de múltipla escolha, que você exibe ao usuário e retorna suas seleções.

<Tip>
  Perguntas de esclarecimento são especialmente comuns no [modo `plan`](/docs/pt/agent-sdk/permissions#plan-mode-plan), onde Claude explora a base de código e faz perguntas antes de propor um plano. Isso torna o modo plan ideal para fluxos de trabalho interativos onde você quer que Claude reúna requisitos antes de fazer alterações.
</Tip>

Os passos a seguir mostram como lidar com perguntas de esclarecimento:

<Steps>
  <Step title="Passe um callback canUseTool">
    Passe um callback `canUseTool` nas opções de sua consulta. Por padrão, `AskUserQuestion` está disponível. Se você especificar um array `tools` para restringir as capacidades do Claude (por exemplo, um agente somente leitura com apenas `Read`, `Glob` e `Grep`), inclua `AskUserQuestion` nesse array. Caso contrário, Claude não será capaz de fazer perguntas de esclarecimento:

    <CodeGroup>
      ```python Python theme={null}
      async for message in query(
          prompt="Analyze this codebase",
          options=ClaudeAgentOptions(
              # Inclua AskUserQuestion em sua lista de ferramentas
              tools=["Read", "Glob", "Grep", "AskUserQuestion"],
              can_use_tool=can_use_tool,
          ),
      ):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      for await (const message of query({
        prompt: "Analyze this codebase",
        options: {
          // Inclua AskUserQuestion em sua lista de ferramentas
          tools: ["Read", "Glob", "Grep", "AskUserQuestion"],
          canUseTool: async (toolName, input) => {
            // Lidar com perguntas de esclarecimento aqui
          }
        }
      })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Detecte AskUserQuestion">
    Em seu callback, verifique se `toolName` é igual a `AskUserQuestion` para tratá-lo diferentemente de outras ferramentas:

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name: str, input_data: dict, context):
          if tool_name == "AskUserQuestion":
              # Sua implementação para coletar respostas do usuário
              return await handle_clarifying_questions(input_data)
          # Lidar com outras ferramentas normalmente
          return await prompt_for_approval(tool_name, input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "AskUserQuestion") {
          // Sua implementação para coletar respostas do usuário
          return handleClarifyingQuestions(input);
        }
        // Lidar com outras ferramentas normalmente
        return promptForApproval(toolName, input);
      };
      ```
    </CodeGroup>
  </Step>

  <Step title="Analise a entrada da pergunta">
    A entrada contém as perguntas do Claude em um array `questions`. Cada pergunta tem uma `question` (o texto a exibir), `options` (as escolhas) e `multiSelect` (se múltiplas seleções são permitidas):

    ```json theme={null}
    {
      "questions": [
        {
          "question": "How should I format the output?",
          "header": "Format",
          "options": [
            { "label": "Summary", "description": "Brief overview" },
            { "label": "Detailed", "description": "Full explanation" }
          ],
          "multiSelect": false
        },
        {
          "question": "Which sections should I include?",
          "header": "Sections",
          "options": [
            { "label": "Introduction", "description": "Opening context" },
            { "label": "Conclusion", "description": "Final summary" }
          ],
          "multiSelect": true
        }
      ]
    }
    ```

    Veja [Formato de pergunta](#question-format) para descrições completas de campos.
  </Step>

  <Step title="Colete respostas do usuário">
    Apresente as perguntas ao usuário e colete suas seleções. Como você faz isso depende de seu aplicativo: um prompt de terminal, um formulário web, um diálogo móvel, etc.
  </Step>

  <Step title="Retorne respostas ao Claude">
    Construa o objeto `answers` como um registro onde cada chave é o texto `question` e cada valor é o `label` da opção selecionada:

    | Do objeto de pergunta                                               | Use como |
    | ------------------------------------------------------------------- | -------- |
    | Campo `question` (por exemplo, `"How should I format the output?"`) | Chave    |
    | Campo `label` da opção selecionada (por exemplo, `"Summary"`)       | Valor    |

    Para perguntas de seleção múltipla, passe um array de labels ou junte-os com `", "`. Se você [suportar entrada de texto livre](#support-free-text-input), use o texto personalizado do usuário como o valor.

    <CodeGroup>
      ```python Python theme={null}
      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": {
                  "How should I format the output?": "Summary",
                  "Which sections should I include?": ["Introduction", "Conclusion"],
              },
          }
      )
      ```

      ```typescript TypeScript theme={null}
      return {
        behavior: "allow",
        updatedInput: {
          questions: input.questions,
          answers: {
            "How should I format the output?": "Summary",
            "Which sections should I include?": "Introduction, Conclusion"
          }
        }
      };
      ```
    </CodeGroup>
  </Step>
</Steps>

<h3 id="question-format">
  Formato de pergunta
</h3>

A entrada contém as perguntas geradas pelo Claude em um array `questions`. Cada pergunta tem estes campos:

| Campo         | Descrição                                                                                                                                     |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `question`    | O texto completo da pergunta a exibir                                                                                                         |
| `header`      | Rótulo curto para a pergunta (máximo 12 caracteres)                                                                                           |
| `options`     | Array de 2-4 escolhas, cada uma com `label` e `description`. TypeScript: opcionalmente `preview` (veja [abaixo](#option-previews-typescript)) |
| `multiSelect` | Se `true`, os usuários podem selecionar múltiplas opções                                                                                      |

A estrutura que seu callback recebe:

```json theme={null}
{
  "questions": [
    {
      "question": "How should I format the output?",
      "header": "Format",
      "options": [
        { "label": "Summary", "description": "Brief overview of key points" },
        { "label": "Detailed", "description": "Full explanation with examples" }
      ],
      "multiSelect": false
    }
  ]
}
```

<h4 id="option-previews-typescript">
  Visualizações de opção (TypeScript)
</h4>

`toolConfig.askUserQuestion.previewFormat` adiciona um campo `preview` a cada opção para que seu aplicativo possa mostrar uma simulação visual ao lado do rótulo. Sem esta configuração, Claude não gera visualizações e o campo está ausente.

| `previewFormat`       | `preview` contém                                                                                                           |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| não definido (padrão) | Campo está ausente. Claude não gera visualizações.                                                                         |
| `"markdown"`          | Arte ASCII e blocos de código cercados                                                                                     |
| `"html"`              | Um fragmento `<div>` estilizado (o SDK rejeita `<script>`, `<style>` e `<!DOCTYPE>` antes que seu callback seja executado) |

O formato se aplica a todas as perguntas na sessão. Claude inclui `preview` em opções onde uma comparação visual ajuda (escolhas de layout, esquemas de cores) e a omite onde não ajudaria (confirmações sim/não, escolhas apenas de texto). Verifique se há `undefined` antes de renderizar.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me choose a card layout",
  options: {
    toolConfig: {
      askUserQuestion: { previewFormat: "html" }
    },
    canUseTool: async (toolName, input) => {
      // input.questions[].options[].preview é uma string HTML ou undefined
      return { behavior: "allow", updatedInput: input };
    }
  }
})) {
  // ...
}
```

Uma opção com uma visualização HTML:

```json theme={null}
{
  "label": "Compact",
  "description": "Title and metric value only",
  "preview": "<div style=\"padding:12px;border:1px solid #ddd;border-radius:8px\"><div style=\"font-size:12px;color:#666\">Active users</div><div style=\"font-size:28px;font-weight:600\">1,284</div></div>"
}
```

<h3 id="response-format">
  Formato de resposta
</h3>

Retorne um objeto `answers` mapeando cada campo `question` da pergunta para o `label` da opção selecionada:

| Campo       | Descrição                                                                                      |
| ----------- | ---------------------------------------------------------------------------------------------- |
| `questions` | Passe o array de perguntas original (obrigatório para processamento de ferramentas)            |
| `answers`   | Objeto onde as chaves são texto de pergunta e os valores são labels selecionados               |
| `response`  | Resposta freeform opcional que o usuário digitou em vez de responder às perguntas estruturadas |

Para perguntas de seleção múltipla, passe um array de labels ou junte-os com `", "`. Para entrada de texto livre por pergunta, como uma opção "Outro", coloque o texto do usuário em `answers[question]` conforme mostrado em [Suporte para entrada de texto livre](#support-free-text-input). Defina `response` apenas quando sua interface do usuário permitir que o usuário descarte o cartão de pergunta e digite uma resposta geral que não seja uma resposta a nenhuma pergunta específica. Quando `response` é definido, Claude recebe "O usuário respondeu: …" em vez da lista de resposta por pergunta.

```json theme={null}
{
  "questions": [
    // ...
  ],
  "answers": {
    "How should I format the output?": "Summary",
    "Which sections should I include?": ["Introduction", "Conclusion"]
  }
}
```

<h4 id="support-free-text-input">
  Suporte para entrada de texto livre
</h4>

As opções predefinidas do Claude nem sempre cobrirão o que os usuários querem. Para permitir que os usuários digitem sua própria resposta:

* Exiba uma escolha "Outro" adicional após as opções do Claude que aceita entrada de texto
* Use o texto personalizado do usuário como o valor da resposta (não a palavra "Outro")

Veja o [exemplo completo](#complete-example) abaixo para uma implementação completa.

<h3 id="complete-example">
  Exemplo completo
</h3>

Claude faz perguntas de esclarecimento quando precisa de entrada do usuário para prosseguir. Por exemplo, quando solicitado a ajudar a decidir sobre uma pilha de tecnologia para um aplicativo móvel, Claude pode perguntar sobre cross-platform vs nativo, preferências de backend ou plataformas alvo. Essas perguntas ajudam Claude a tomar decisões que correspondem às preferências do usuário em vez de adivinhar.

Este exemplo lida com essas perguntas em um aplicativo de terminal. Aqui está o que acontece em cada etapa:

1. **Rotear a solicitação**: O callback `canUseTool` verifica se o nome da ferramenta é `"AskUserQuestion"` e roteia para um manipulador dedicado
2. **Exibir perguntas**: O manipulador percorre o array `questions` e imprime cada pergunta com opções numeradas
3. **Coletar entrada**: O usuário pode inserir um número para selecionar uma opção ou digitar texto livre diretamente (por exemplo, "jquery", "não sei")
4. **Mapear respostas**: O código verifica se a entrada é numérica (usa o label da opção) ou texto livre (usa o texto diretamente)
5. **Retornar ao Claude**: A resposta inclui tanto o array `questions` original quanto o mapeamento `answers`

Salve a versão TypeScript como `ask.ts` e execute-a com `npx tsx ask.ts`, ou salve a versão Python como `ask.py` e execute-a com `python ask.py`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import HookMatcher, PermissionResultAllow


  def parse_response(response: str, options: list) -> str:
      """Analise a entrada do usuário como número(s) de opção ou texto livre."""
      try:
          indices = [int(s.strip()) - 1 for s in response.split(",")]
          labels = [options[i]["label"] for i in indices if 0 <= i < len(options)]
          return ", ".join(labels) if labels else response
      except ValueError:
          return response


  async def handle_ask_user_question(input_data: dict) -> PermissionResultAllow:
      """Exiba as perguntas do Claude e colete respostas do usuário."""
      answers = {}

      for q in input_data.get("questions", []):
          print(f"\n{q['header']}: {q['question']}")

          options = q["options"]
          for i, opt in enumerate(options):
              print(f"  {i + 1}. {opt['label']} - {opt['description']}")
          if q.get("multiSelect"):
              print("  (Enter numbers separated by commas, or type your own answer)")
          else:
              print("  (Enter a number, or type your own answer)")

          response = input("Your choice: ").strip()
          answers[q["question"]] = parse_response(response, options)

      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": answers,
          }
      )


  async def can_use_tool(
      tool_name: str, input_data: dict, context
  ) -> PermissionResultAllow:
      # Rotear AskUserQuestion para nosso manipulador de perguntas
      if tool_name == "AskUserQuestion":
          return await handle_ask_user_question(input_data)
      # Auto-aprovar outras ferramentas para este exemplo
      return PermissionResultAllow(updated_input=input_data)


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Help me decide on the tech stack for a new mobile app",
          },
      }


  # Solução alternativa necessária: hook fictício mantém o fluxo aberto para can_use_tool
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline/promises";

  // Auxiliar para solicitar entrada do usuário no terminal
  async function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
    const answer = await rl.question(question);
    rl.close();
    return answer;
  }

  // Analise a entrada do usuário como número(s) de opção ou texto livre
  function parseResponse(response: string, options: any[]): string {
    const indices = response.split(",").map((s) => parseInt(s.trim()) - 1);
    const labels = indices
      .filter((i) => !isNaN(i) && i >= 0 && i < options.length)
      .map((i) => options[i].label);
    return labels.length > 0 ? labels.join(", ") : response;
  }

  // Exiba as perguntas do Claude e colete respostas do usuário
  async function handleAskUserQuestion(input: any) {
    const answers: Record<string, string> = {};

    for (const q of input.questions) {
      console.log(`\n${q.header}: ${q.question}`);

      const options = q.options;
      options.forEach((opt: any, i: number) => {
        console.log(`  ${i + 1}. ${opt.label} - ${opt.description}`);
      });
      if (q.multiSelect) {
        console.log("  (Enter numbers separated by commas, or type your own answer)");
      } else {
        console.log("  (Enter a number, or type your own answer)");
      }

      const response = (await prompt("Your choice: ")).trim();
      answers[q.question] = parseResponse(response, options);
    }

    // Retorne as respostas ao Claude (deve incluir perguntas originais)
    return {
      behavior: "allow",
      updatedInput: { questions: input.questions, answers }
    };
  }

  async function main() {
    for await (const message of query({
      prompt: "Help me decide on the tech stack for a new mobile app",
      options: {
        canUseTool: async (toolName, input) => {
          // Rotear AskUserQuestion para nosso manipulador de perguntas
          if (toolName === "AskUserQuestion") {
            return handleAskUserQuestion(input);
          }
          // Auto-aprovar outras ferramentas para este exemplo
          return { behavior: "allow", updatedInput: input };
        }
      }
    })) {
      if ("result" in message) console.log(message.result);
    }
  }

  main();
  ```
</CodeGroup>

<h2 id="limitations">
  Limitações
</h2>

* **Subagentes**: `AskUserQuestion` não está disponível em subagentes gerados por meio da ferramenta Agent
* **Limites de perguntas**: cada chamada `AskUserQuestion` suporta 1-4 perguntas com 2-4 opções cada

<h2 id="other-ways-to-get-user-input">
  Outras maneiras de obter entrada do usuário
</h2>

O callback `canUseTool` e a ferramenta `AskUserQuestion` cobrem a maioria dos cenários de aprovação e esclarecimento, mas o SDK oferece outras maneiras de obter entrada dos usuários:

<h3 id="streaming-input">
  Entrada de streaming
</h3>

Use [entrada de streaming](/docs/pt/agent-sdk/streaming-vs-single-mode) quando você precisar:

* **Interromper o agente no meio da tarefa**: enviar um sinal de cancelamento ou mudar de direção enquanto Claude está trabalhando
* **Fornecer contexto adicional**: adicionar informações que Claude precisa sem esperar que ele pergunte
* **Construir interfaces de chat**: permitir que os usuários enviem mensagens de acompanhamento durante operações de longa duração

A entrada de streaming é ideal para interfaces conversacionais onde os usuários interagem com o agente durante toda a execução, não apenas em pontos de aprovação.

<h3 id="custom-tools">
  Ferramentas personalizadas
</h3>

Use [ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools) quando você precisar:

* **Coletar entrada estruturada**: construir formulários, assistentes ou fluxos de trabalho de várias etapas que vão além do formato de múltipla escolha do `AskUserQuestion`
* **Integrar sistemas de aprovação externos**: conectar a plataformas de ticketing, fluxo de trabalho ou aprovação existentes
* **Implementar interações específicas do domínio**: criar ferramentas adaptadas às necessidades do seu aplicativo, como interfaces de revisão de código ou listas de verificação de implantação

As ferramentas personalizadas lhe dão controle total sobre a interação, mas requerem mais trabalho de implementação do que usar o callback `canUseTool` integrado.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Configurar permissões](/docs/pt/agent-sdk/permissions): configurar modos e regras de permissão
* [Controlar execução com hooks](/docs/pt/agent-sdk/hooks): executar código personalizado em pontos-chave do ciclo de vida do agente
* [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript#canusetool): documentação completa da API canUseTool
