> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rastrear tarefas

> Rastreie tarefas em sessões do Agent SDK e renderize o progresso do Claude em sua aplicação a partir de chamadas de ferramentas estruturadas

Claude Code fornece as [ferramentas de rastreamento de tarefas](/docs/pt/tools-reference#task-tool-availability) por padrão apenas nos modelos listados em [Disponibilidade de modelos](#model-availability). Modelos mais novos rastreiam trabalho com múltiplas etapas sem uma lista de tarefas escrita, portanto nesses você não precisa de nada nesta página para Claude trabalhar através de tarefas com múltiplas etapas.

Em uma sessão que possui as ferramentas de rastreamento de tarefas, Claude mantém uma lista de tarefas escrita, atualizando o status de cada item conforme trabalha. Você vê cada mudança no fluxo de mensagens como uma chamada de ferramenta estruturada. Opte por uma sessão apenas quando sua aplicação lê essas chamadas de ferramentas, seja para registrar atividade de tarefas ou para renderizar sua própria exibição de progresso.

<h2 id="model-availability">
  Disponibilidade de modelos
</h2>

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.
</Note>

Em um modelo que não possui as ferramentas por padrão, a menos que você opte por uma sessão, você não vê blocos `tool_use` para elas no fluxo de mensagens. O Agent SDK aplica esses padrões através do binário Claude Code que ele agrupa. Se você apontar `pathToClaudeCodeExecutable` (TypeScript) ou `cli_path` (Python) para sua própria instalação do Claude Code, você obtém quaisquer ferramentas que essa instalação fornece, sob seus próprios padrões. Para ver o conjunto exato em uma sessão em execução, [verifique quais ferramentas estão disponíveis](/docs/pt/tools-reference#check-which-tools-are-available). Para optar por uma sessão, faça um dos seguintes:

* Nomeie uma das ferramentas na opção [`allowedTools`](/docs/pt/agent-sdk/permissions#allow-and-deny-rules) (TypeScript) ou `allowed_tools` (Python)
* Liste as ferramentas na opção `tools`, que restringe as ferramentas integradas da sessão àquelas que ela nomeia. Inclua as ferramentas que você deseja junto com as outras ferramentas integradas que você usa
* Defina `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` na opção `env`, como os exemplos nesta página fazem. No TypeScript, `env` substitui o ambiente do subprocesso, então espalhe `...process.env` para manter variáveis herdadas. No Python, `env` é mesclado no topo do ambiente herdado

<h2 id="todo-lifecycle">
  Ciclo de vida das tarefas
</h2>

Claude move cada tarefa através de um ciclo de vida previsível:

1. **Criada**: Claude adiciona a tarefa como `pending` quando identifica uma tarefa
2. **Ativada**: Claude define a tarefa como `in_progress` quando inicia o trabalho
3. **Concluída**: Claude marca como concluída quando a tarefa termina com sucesso
4. **Removida**: Claude deleta uma tarefa que não precisa mais definindo `status: "deleted"` em uma chamada `TaskUpdate`

<h2 id="when-claude-creates-todos">
  Quando Claude cria tarefas
</h2>

Em uma [sessão que possui as ferramentas de rastreamento de tarefas](#model-availability), Claude cria tarefas para a maioria dos trabalhos com múltiplas etapas, como:

* **Tarefas complexas com múltiplas etapas** que exigem três ou mais ações distintas
* **Listas de tarefas fornecidas pelo usuário** quando vários itens são mencionados
* **Operações mais longas** que se beneficiam do rastreamento de progresso
* **Solicitações explícitas** quando os usuários pedem organização de tarefas

Claude pode pular tarefas para solicitações muito curtas ou de uma única etapa.

<h2 id="examples">
  Exemplos
</h2>

Antes de executar estes exemplos, instale o Claude Agent SDK seguindo o [guia de início rápido](/docs/pt/agent-sdk/quickstart). Cada exemplo nesta página compartilha a mesma configuração de permissões e comportamento de saída:

* **Modo de permissão**: os exemplos de prompt pedem ao Claude para fazer trabalho real em um projeto, então cada exemplo define `permissionMode: "acceptEdits"` (TypeScript) ou `permission_mode="acceptEdits"` (Python) para aprovar automaticamente as edições de arquivo que o trabalho produz. Veja [Modos de permissão](/docs/pt/agent-sdk/permissions#permission-modes) para as alternativas.
* **Limite de turnos**: cada exemplo é executado até que o agente termine e produza sua mensagem de resultado final. Se uma sessão atingir seu limite de turnos primeiro, essa mensagem de resultado terá o subtipo `error_max_turns`. Verifique `subtype` para detectar esse encerramento.
* **Tratamento de erros**: estes exemplos usam chamadas `query()` de um único disparo. Após produzir um resultado `error_max_turns`, `query()` lança um erro que inclui `Reached maximum number of turns`. Cada exemplo envolve seu loop em um bloco try para sair corretamente quando isso acontece. Veja [Lidar com o resultado](/docs/pt/agent-sdk/agent-loop#handle-the-result) para os subtipos de resultado.

<Note>
  As mensagens do sistema de tarefas, [`SDKTaskNotificationMessage`](/docs/pt/agent-sdk/typescript#sdktasknotificationmessage) (TypeScript) ou [`TaskNotificationMessage`](/docs/pt/agent-sdk/python#tasknotificationmessage) (Python) entre elas, relatam tarefas em segundo plano, como comandos em segundo plano e suagentes. No fluxo de mensagens, você vê atividade de tarefas como blocos `tool_use` nas mensagens do assistente.
</Note>

<h3 id="monitor-todo-changes">
  Monitorar mudanças de tarefas
</h3>

O exemplo a seguir observa o fluxo do assistente para blocos `tool_use` `TaskCreate` e `TaskUpdate` e imprime uma linha `+` com o assunto de cada nova tarefa e uma linha de atualização com o ID da tarefa e o novo status de cada mudança de status. Use esta forma quando você deseja um registro de atividade de tarefas em vez de uma exibição renderizada. As linhas `+` não incluem os IDs atribuídos, então este registro não pode corresponder atualizações de volta às suas criações. Para manter essa correlação, capture os IDs como [Exibir progresso em tempo real](#display-progress-in-real-time) faz.

A entrada `tool_use` transmitida é a forma bruta que o modelo emitiu. Claude Code repara alguns nomes de chave próximos mas incorretos antes da execução, mapeando `id` ou `task_id` para `taskId` e `active_form` para `activeForm`, mas esse reparo não é refletido no fluxo. Leia os campos de entrada de `TaskUpdate` defensivamente, como ambos os exemplos nesta página fazem, em vez de assumir que o nome canônico está sempre presente.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
      // Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
      options: { maxTurns: 15, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
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
              prompt="Create a static website with a home page, an about page, and a shared stylesheet, and track progress with todos",
              # Keeps the Task tools on models where Claude Code otherwise doesn't provide them.
              options=ClaudeAgentOptions(max_turns=15, permission_mode="acceptEdits", env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"}),
          ):
              if not isinstance(message, AssistantMessage):
                  continue
              for block in message.content:
                  if not isinstance(block, ToolUseBlock):
                      continue
                  if block.name == "TaskCreate":
                      print(f"+ {block.input.get('subject', '')}")
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

<h3 id="display-progress-in-real-time">
  Exibir progresso em tempo real
</h3>

O exemplo a seguir observa o fluxo do assistente para blocos `tool_use` `TaskCreate` e `TaskUpdate` e mantém um mapa de tarefas codificadas por ID de tarefa em uma classe `TaskTracker`, renderizando novamente um resumo de progresso a cada mudança. O resumo conta tarefas concluídas e em progresso e mostra o rótulo `activeForm` de cada item ativo no lugar de seu `subject`. Use esta forma quando sua aplicação mantém uma exibição de progresso em vez de registrar cada evento.

O ID de tarefa atribuído não está na entrada de `TaskCreate`. Claude Code entrega a saída estruturada de cada ferramenta na mensagem do usuário que carrega seu bloco `tool_result`, no campo `tool_use_result`. Para `TaskCreate`, esse objeto é documentado para TypeScript como `TaskCreateOutput` em [Tipos de Saída de Ferramenta](/docs/pt/agent-sdk/typescript#tool-output-types), e em Python o campo é um dict simples da mesma forma. O rastreador emparelha cada bloco `tool_result` com sua chamada `tool_use` por `tool_use_id` e lê `task.id` da mensagem emparelhada `tool_use_result`. Claude pode ler a lista de volta com `TaskList` e os detalhes completos de uma tarefa com `TaskGet`.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  type Task = { subject: string; activeForm?: string; status: string };

  class TaskTracker {
    private tasks = new Map<string, Task>();
    private pendingCreates = new Map<string, { subject: string; activeForm?: string }>();

    displayProgress() {
      if (this.tasks.size === 0) {
        console.log("\nProgress: no open tasks\n");
        return;
      }

      const items = [...this.tasks.values()];
      const completed = items.filter((t) => t.status === "completed").length;
      const inProgress = items.filter((t) => t.status === "in_progress").length;

      console.log(`\nProgress: ${completed}/${this.tasks.size} completed`);
      console.log(`Currently working on: ${inProgress} task(s)\n`);

      for (const [id, task] of this.tasks) {
        const icon =
          task.status === "completed" ? "✅" : task.status === "in_progress" ? "🔧" : "❌";
        const text = task.status === "in_progress" && task.activeForm ? task.activeForm : task.subject;
        console.log(`${id}. ${icon} ${text}`);
      }
    }

    handleToolUse(block: { id: string; name: string; input: unknown }) {
      if (block.name === "TaskCreate") {
        const input = block.input as { subject: string; activeForm?: string; active_form?: string };
        this.pendingCreates.set(block.id, {
          subject: input.subject,
          activeForm: input.activeForm ?? input.active_form,
        });
      } else if (block.name === "TaskUpdate") {
        const input = block.input as {
          taskId?: string;
          id?: string;
          task_id?: string;
          status?: string;
          activeForm?: string;
          active_form?: string;
        };
        const taskId = input.taskId ?? input.id ?? input.task_id;
        if (!taskId) return;
        if (input.status === "deleted") {
          this.tasks.delete(taskId);
          this.displayProgress();
          return;
        }
        const task = this.tasks.get(taskId);
        if (!task) return;
        if (input.status) task.status = input.status;
        const active = input.activeForm ?? input.active_form;
        if (active) task.activeForm = active;
        this.displayProgress();
      }
    }

    handleToolResult(block: { tool_use_id: string; is_error?: boolean }, result: unknown) {
      const create = this.pendingCreates.get(block.tool_use_id);
      if (!create) return;
      this.pendingCreates.delete(block.tool_use_id);
      if (block.is_error) return;
      // The result's user message carries the tool's structured output as
      // tool_use_result; for TaskCreate that's TaskCreateOutput,
      // { task: { id, subject } }.
      const out = result as { task?: { id: string } };
      if (!out?.task?.id) return;
      this.tasks.set(out.task.id, { ...create, status: "pending" });
      this.displayProgress();
    }

    async trackQuery(prompt: string) {
      try {
        for await (const message of query({
          prompt,
          options: { maxTurns: 20, permissionMode: "acceptEdits", env: { ...process.env, CLAUDE_CODE_ENABLE_TODO_TOOLS: "1" } },
        })) {
          if (message.type === "assistant") {
            for (const block of message.message.content) {
              if (block.type === "tool_use") this.handleToolUse(block);
            }
          }
          if (message.type === "user" && Array.isArray(message.message.content)) {
            for (const block of message.message.content) {
              if (block.type === "tool_result") this.handleToolResult(block, message.tool_use_result);
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
  const tracker = new TaskTracker();
  await tracker.trackQuery("Build a complete authentication system with todos");
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      AssistantMessage,
      UserMessage,
      ToolUseBlock,
      ToolResultBlock,
  )


  class TaskTracker:
      def __init__(self):
          self.tasks: dict[str, dict] = {}
          self.pending_creates: dict[str, dict] = {}

      def display_progress(self):
          if not self.tasks:
              print("\nProgress: no open tasks\n")
              return

          completed = len([t for t in self.tasks.values() if t["status"] == "completed"])
          in_progress = len([t for t in self.tasks.values() if t["status"] == "in_progress"])

          print(f"\nProgress: {completed}/{len(self.tasks)} completed")
          print(f"Currently working on: {in_progress} task(s)\n")

          for task_id, task in self.tasks.items():
              icon = (
                  "✅"
                  if task["status"] == "completed"
                  else "🔧"
                  if task["status"] == "in_progress"
                  else "❌"
              )
              text = (
                  task["activeForm"]
                  if task["status"] == "in_progress" and task.get("activeForm")
                  else task["subject"]
              )
              print(f"{task_id}. {icon} {text}")

      def handle_tool_use(self, block: ToolUseBlock):
          if block.name == "TaskCreate":
              self.pending_creates[block.id] = {
                  "subject": block.input.get("subject", ""),
                  "activeForm": block.input.get("activeForm") or block.input.get("active_form"),
              }
          elif block.name == "TaskUpdate":
              task_id = (
                  block.input.get("taskId")
                  or block.input.get("id")
                  or block.input.get("task_id")
              )
              if not task_id:
                  return
              if block.input.get("status") == "deleted":
                  self.tasks.pop(task_id, None)
                  self.display_progress()
                  return
              task = self.tasks.get(task_id)
              if not task:
                  return
              if block.input.get("status"):
                  task["status"] = block.input["status"]
              active = block.input.get("activeForm") or block.input.get("active_form")
              if active:
                  task["activeForm"] = active
              self.display_progress()

      def handle_tool_result(self, block: ToolResultBlock, tool_use_result):
          create = self.pending_creates.pop(block.tool_use_id, None)
          if create is None or block.is_error:
              return
          # The result's user message carries the tool's structured output as
          # tool_use_result; for TaskCreate that's {"task": {"id": ..., "subject": ...}}.
          task = (tool_use_result or {}).get("task") or {}
          if not task.get("id"):
              return
          self.tasks[task["id"]] = {**create, "status": "pending"}
          self.display_progress()

      async def track_query(self, prompt: str):
          try:
              async for message in query(
                  prompt=prompt,
                  options=ClaudeAgentOptions(
                      max_turns=20,
                      permission_mode="acceptEdits",
                      env={"CLAUDE_CODE_ENABLE_TODO_TOOLS": "1"},
                  ),
              ):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              self.handle_tool_use(block)
                  if isinstance(message, UserMessage) and isinstance(message.content, list):
                      for block in message.content:
                          if isinstance(block, ToolResultBlock):
                              self.handle_tool_result(block, message.tool_use_result)
          except Exception as error:
              # A single-shot query() raises after yielding an error result,
              # such as when the max_turns limit is hit.
              print(f"Session ended with an error: {error}")


  # Usage
  async def main():
      tracker = TaskTracker()
      await tracker.track_query("Build a complete authentication system with todos")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="related-documentation">
  Documentação relacionada
</h2>

* [Referência do Agent SDK - TypeScript](/docs/pt/agent-sdk/typescript): as opções, tipos e esquemas de ferramentas para o SDK TypeScript, incluindo os tipos de entrada e saída da ferramenta Task
* [Referência do Agent SDK - Python](/docs/pt/agent-sdk/python): as opções, tipos e documentação de ferramentas para o SDK Python
* [Entrada de Streaming](/docs/pt/agent-sdk/streaming-vs-single-mode): os dois modos de entrada e quando usar entrada de streaming em vez das chamadas de um único disparo que estes exemplos usam
* [Dê ao Claude ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools): defina suas próprias ferramentas com o servidor MCP em processo do SDK
