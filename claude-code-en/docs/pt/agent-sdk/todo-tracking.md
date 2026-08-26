> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Listas de Tarefas

> Rastreie e exiba tarefas usando o Claude Agent SDK para gerenciamento organizado de tarefas

O rastreamento de tarefas fornece uma forma estruturada de gerenciar tarefas e exibir o progresso aos usuários. O Claude Agent SDK inclui funcionalidade integrada de tarefas que ajuda a organizar fluxos de trabalho complexos e manter os usuários informados sobre a progressão das tarefas.

<Note>
  A partir do TypeScript Agent SDK 0.3.142 e Claude Code v2.1.142, as sessões usam as ferramentas Task estruturadas `TaskCreate`, `TaskUpdate`, `TaskGet` e `TaskList` em vez de `TodoWrite`. O SDK Python obtém essa mudança da CLI Claude Code que ele inicia, não da versão do pacote Python: a mudança se aplica uma vez que essa CLI — a cópia incluída dentro do pacote pip, ou uma que você aponta com `cli_path` — é v2.1.142 ou posterior. Consulte [Migrar para ferramentas Task](#migrate-to-task-tools) para saber como o código de monitoramento muda. Os exemplos nesta página definem `CLAUDE_CODE_ENABLE_TASKS=0` para continuar mostrando `TodoWrite` para sessões que ainda não foram migradas.
</Note>

<h3 id="todo-lifecycle">
  Ciclo de Vida das Tarefas
</h3>

As tarefas seguem um ciclo de vida previsível:

1. **Criadas** como `pending` quando as tarefas são identificadas
2. **Ativadas** para `in_progress` quando o trabalho começa
3. **Concluídas** quando a tarefa termina com sucesso
4. **Removidas** quando todas as tarefas em um grupo são concluídas

<h3 id="when-todos-are-used">
  Quando as Tarefas São Usadas
</h3>

O SDK cria tarefas para a maioria dos trabalhos com múltiplas etapas, como:

* **Tarefas complexas com múltiplas etapas** que exigem 3 ou mais ações distintas
* **Listas de tarefas fornecidas pelo usuário** quando vários itens são mencionados
* **Operações não triviais** que se beneficiam do rastreamento de progresso
* **Solicitações explícitas** quando os usuários pedem organização de tarefas

Pode pular tarefas para solicitações muito curtas ou de uma única etapa.

<h2 id="examples">
  Exemplos
</h2>

Antes de executar estes exemplos, instale o Claude Agent SDK seguindo o [guia de início rápido](/docs/pt/agent-sdk/quickstart).

Cada exemplo é executado até que o agente termine e produza sua mensagem de resultado final. Se uma sessão atingir seu limite de turnos primeiro, essa mensagem de resultado terá o subtipo `error_max_turns`. Verifique `subtype` para detectar esse encerramento.

Estes exemplos usam chamadas `query()` de um único disparo. Após produzir um resultado `error_max_turns`, `query()` lança um erro que inclui `Reached maximum number of turns`. Cada exemplo envolve seu loop em um bloco try para sair corretamente quando isso acontece.

Consulte [Lidar com o resultado](/docs/pt/agent-sdk/agent-loop#handle-the-result) para os subtipos de resultado.

<h3 id="monitoring-todo-changes">
  Monitorando Mudanças de Tarefas
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Optimize my React app performance and track progress with todos",
      // Re-enable TodoWrite, which this example monitors. Without it, the SDK uses
      // Task tools instead and these tool_use blocks never appear.
      options: { maxTurns: 15, env: { ...process.env, CLAUDE_CODE_ENABLE_TASKS: "0" } }
    })) {
      // Todo updates are reflected in the message stream
      if (message.type === "assistant") {
        for (const block of message.message.content) {
          if (block.type === "tool_use" && block.name === "TodoWrite") {
            const todos = block.input.todos;

            console.log("Todo Status Update:");
            todos.forEach((todo, index) => {
              const status =
                todo.status === "completed" ? "✅" : todo.status === "in_progress" ? "🔧" : "❌";
              console.log(`${index + 1}. ${status} ${todo.content}`);
            });
          }
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // such as when the maxTurns limit is hit.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock


  async def main():
      try:
          async for message in query(
              prompt="Optimize my React app performance and track progress with todos",
              # Re-enable TodoWrite, which this example monitors. Without it, the SDK uses
              # Task tools instead and these tool_use blocks never appear.
              options=ClaudeAgentOptions(max_turns=15, env={"CLAUDE_CODE_ENABLE_TASKS": "0"}),
          ):
              # Todo updates are reflected in the message stream
              if isinstance(message, AssistantMessage):
                  for block in message.content:
                      if isinstance(block, ToolUseBlock) and block.name == "TodoWrite":
                          todos = block.input["todos"]

                          print("Todo Status Update:")
                          for i, todo in enumerate(todos):
                              status = (
   "✅"
   if todo["status"] == "completed"
   else "🔧"
   if todo["status"] == "in_progress"
   else "❌"
                              )
                              print(f"{i + 1}. {status} {todo['content']}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # such as when the max_turns limit is hit.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="real-time-progress-display">
  Exibição de Progresso em Tempo Real
</h3>

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  class TodoTracker {
    private todos: any[] = [];

    displayProgress() {
      if (this.todos.length === 0) return;

      const completed = this.todos.filter((t) => t.status === "completed").length;
      const inProgress = this.todos.filter((t) => t.status === "in_progress").length;
      const total = this.todos.length;

      console.log(`\nProgress: ${completed}/${total} completed`);
      console.log(`Currently working on: ${inProgress} task(s)\n`);

      this.todos.forEach((todo, index) => {
        const icon =
          todo.status === "completed" ? "✅" : todo.status === "in_progress" ? "🔧" : "❌";
        const text = todo.status === "in_progress" ? todo.activeForm : todo.content;
        console.log(`${index + 1}. ${icon} ${text}`);
      });
    }

    async trackQuery(prompt: string) {
      try {
        for await (const message of query({
          prompt,
          // Re-enable TodoWrite, which this tracker watches for.
          options: { maxTurns: 20, env: { ...process.env, CLAUDE_CODE_ENABLE_TASKS: "0" } }
        })) {
          if (message.type === "assistant") {
            for (const block of message.message.content) {
              if (block.type === "tool_use" && block.name === "TodoWrite") {
                this.todos = block.input.todos;
                this.displayProgress();
              }
            }
          }
        }
      } catch (error) {
        // A single-shot query() throws after yielding an error result,
        // such as when the maxTurns limit is hit.
        console.log(`Session ended with an error: ${error}`);
      }
    }
  }

  // Usage
  const tracker = new TodoTracker();
  await tracker.trackQuery("Build a complete authentication system with todos");
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
  from typing import List, Dict


  class TodoTracker:
      def __init__(self):
          self.todos: List[Dict] = []

      def display_progress(self):
          if not self.todos:
              return

          completed = len([t for t in self.todos if t["status"] == "completed"])
          in_progress = len([t for t in self.todos if t["status"] == "in_progress"])
          total = len(self.todos)

          print(f"\nProgress: {completed}/{total} completed")
          print(f"Currently working on: {in_progress} task(s)\n")

          for i, todo in enumerate(self.todos):
              icon = (
                  "✅"
                  if todo["status"] == "completed"
                  else "🔧"
                  if todo["status"] == "in_progress"
                  else "❌"
              )
              text = (
                  todo["activeForm"]
                  if todo["status"] == "in_progress"
                  else todo["content"]
              )
              print(f"{i + 1}. {icon} {text}")

      async def track_query(self, prompt: str):
          try:
              async for message in query(
                  prompt=prompt,
                  # Re-enable TodoWrite, which this tracker watches for.
                  options=ClaudeAgentOptions(max_turns=20, env={"CLAUDE_CODE_ENABLE_TASKS": "0"}),
              ):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock) and block.name == "TodoWrite":
                              self.todos = block.input["todos"]
                              self.display_progress()
          except Exception as error:
              # A single-shot query() raises after yielding an error result,
              # such as when the max_turns limit is hit.
              print(f"Session ended with an error: {error}")


  # Usage
  async def main():
      tracker = TodoTracker()
      await tracker.track_query("Build a complete authentication system with todos")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="migrate-to-task-tools">
  Migrar para ferramentas Task
</h2>

As ferramentas Task dividem a única chamada `TodoWrite` em `TaskCreate` para cada novo item e `TaskUpdate` para cada mudança de status, com `TaskList` e `TaskGet` disponíveis para o modelo ler de volta a lista atual. Seu código de monitoramento ainda inspeciona blocos `tool_use` no fluxo do assistente, mas mantém um mapa codificado por ID de tarefa em vez de substituir a lista inteira a cada chamada. As ferramentas Task são o padrão a partir do TypeScript Agent SDK 0.3.142 e Claude Code v2.1.142, portanto nenhuma mudança em `options.env` é necessária.

| Com `TodoWrite`                                              | Com ferramentas Task                                                                                                                                                                                                                                                                                              |
| ------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Uma chamada de ferramenta reescreve o array `todos` completo | `TaskCreate` adiciona um item, `TaskUpdate` corrige um item por `taskId`                                                                                                                                                                                                                                          |
| Corresponder `block.name === "TodoWrite"`                    | Corresponder `block.name === "TaskCreate"` ou `"TaskUpdate"`                                                                                                                                                                                                                                                      |
| Forma do item: `{ content, status, activeForm }`             | Entrada de `TaskCreate`: `{ subject, description, activeForm?, metadata? }`. Entrada de `TaskUpdate`: `{ taskId, status?, subject?, description?, activeForm?, addBlocks?, addBlockedBy?, owner?, metadata? }`. `status` é `"pending"`, `"in_progress"` ou `"completed"`; defina `status: "deleted"` para deletar |
| Renderizar `block.input.todos` diretamente                   | Acumular itens entre chamadas, ou ler um snapshot de um resultado de ferramenta `TaskList`                                                                                                                                                                                                                        |

O ID de tarefa atribuído não está na entrada de `TaskCreate`. Ele volta no `tool_result` correspondente como `{ task: { id, subject } }`, então capture-o do bloco de resultado para codificar seu mapa. O exemplo a seguir mostra a mudança mínima para o loop [Monitorando Mudanças de Tarefas](#monitoring-todo-changes). Ele lê apenas entradas de `tool_use` e ignora a captura de IDs de blocos `tool_result`. Para renderizar uma lista completa, observe um resultado de ferramenta `TaskList` no fluxo ou acumule resultados de `TaskCreate` e entradas de `TaskUpdate` em um mapa.

O input `tool_use` transmitido é a forma bruta que o modelo emitiu. Claude Code repara alguns nomes de chave próximos mas incorretos antes da execução, mapeando `id` ou `task_id` para `taskId` e `active_form` para `activeForm`, mas esse reparo não é refletido no fluxo. Leia os campos de entrada de `TaskUpdate` defensivamente, como os exemplos abaixo fazem, em vez de assumir que o nome canônico está sempre presente.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Optimize my React app performance and track progress with todos",
      options: { maxTurns: 15 },
    })) {
      if (message.type !== "assistant") continue;
      for (const block of message.message.content) {
        if (block.type !== "tool_use") continue;
        if (block.name === "TaskCreate") {
          const input = block.input as { subject: string };
          console.log(`+ ${input.subject}`);
        } else if (block.name === "TaskUpdate") {
          const input = block.input as {
            taskId?: string;
            id?: string;
            task_id?: string;
            status?: string;
          };
          const taskId = input.taskId ?? input.id ?? input.task_id;
          if (taskId && input.status) console.log(`  ${taskId} -> ${input.status}`);
        }
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock

  async def main():
      try:
          async for message in query(
              prompt="Optimize my React app performance and track progress with todos",
              options=ClaudeAgentOptions(max_turns=15),
          ):
              if not isinstance(message, AssistantMessage):
                  continue
              for block in message.content:
                  if not isinstance(block, ToolUseBlock):
                      continue
                  if block.name == "TaskCreate":
                      print(f"+ {block.input['subject']}")
                  elif block.name == "TaskUpdate" and block.input.get("status"):
                      task_id = (
                          block.input.get("taskId")
                          or block.input.get("id")
                          or block.input.get("task_id")
                      )
                      if task_id:
                          print(f"  {task_id} -> {block.input['status']}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="related-documentation">
  Documentação Relacionada
</h2>

* [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript)
* [Referência do SDK Python](/docs/pt/agent-sdk/python)
* [Streaming vs Modo Único](/docs/pt/agent-sdk/streaming-vs-single-mode)
* [Ferramentas Personalizadas](/docs/pt/agent-sdk/custom-tools)
