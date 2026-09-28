> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagentes no SDK

> Defina e invoque subagentes para isolar contexto, executar tarefas em paralelo e aplicar instruções especializadas em suas aplicações Claude Agent SDK.

Subagentes são instâncias de agente separadas que seu agente principal pode gerar para lidar com subtarefas focadas.
Use-os para isolar contexto, executar múltiplas análises em paralelo e aplicar instruções especializadas sem adicionar ao prompt do agente principal.

<h2 id="overview">
  Visão Geral
</h2>

Você pode criar subagentes de três maneiras:

* **Programaticamente**: use o parâmetro `agents` nas suas opções `query()`. Consulte as referências [TypeScript](/docs/pt/agent-sdk/typescript#agentdefinition) e [Python](/docs/pt/agent-sdk/python#agentdefinition)
* **Baseado em sistema de arquivos**: defina agentes como arquivos markdown em diretórios `.claude/agents/`. Consulte [definindo subagentes como arquivos](/docs/pt/sub-agents)
* **Propósito geral integrado**: Claude pode invocar o subagente `general-purpose` integrado a qualquer momento através da ferramenta Agent sem que você defina nada

Este guia se concentra na abordagem programática, que é recomendada para aplicações SDK.

<h2 id="benefits-of-using-subagents">
  Benefícios do uso de subagentes
</h2>

Como os subagentes são instâncias de agente separadas, delegar trabalho para eles oferece quatro benefícios:

* **Isolamento de contexto**: cada subagente é executado em sua própria conversa, que começa do zero, a menos que o subagente seja um [fork](/docs/pt/sub-agents#fork-the-current-conversation). De qualquer forma, chamadas de ferramentas intermediárias e resultados permanecem dentro do subagente; apenas sua mensagem final retorna ao agente pai. Um subagente `research-assistant` pode explorar dezenas de arquivos sem que nenhum desse conteúdo se acumule na conversa principal. O agente pai recebe um resumo conciso, não cada arquivo que o subagente leu. Consulte [O que os subagentes herdam](#what-subagents-inherit) para saber exatamente o que está no contexto do subagente.
* **Paralelização**: múltiplos subagentes podem ser executados simultaneamente, portanto subtarefas independentes são concluídas no tempo do mais lento, em vez da soma de todos eles. Durante uma revisão de código, você pode executar os subagentes `style-checker`, `security-scanner` e `test-coverage` simultaneamente em vez de sequencialmente.
* **Instruções e conhecimento especializados**: cada subagente pode ter um prompt de sistema personalizado com expertise específica, melhores práticas e restrições. Um subagente `database-migration` pode ter conhecimento detalhado sobre melhores práticas de SQL, estratégias de reversão e verificações de integridade de dados que seriam ruído desnecessário nas instruções do agente principal.
* **Restrições de ferramentas**: subagentes podem ser limitados a ferramentas específicas, reduzindo o risco de ações não intencionais. Um subagente `doc-reviewer` pode ter acesso apenas às ferramentas Read e Grep, garantindo que possa analisar, mas nunca modifique acidentalmente seus arquivos de documentação.

<h2 id="create-subagents">
  Criar subagentes
</h2>

<h3 id="programmatic-definition-recommended">
  Definição programática (recomendado)
</h3>

Defina subagentes diretamente no seu código usando o parâmetro `agents`. Claude invoca subagentes através da ferramenta `Agent`.

A maioria dos exemplos nesta página imprime apenas o resultado final. Para confirmar que Claude delegou a um subagente em vez de responder diretamente, consulte [Detectar invocação de subagente](#detect-subagent-invocation).

Este exemplo cria dois subagentes: um revisor de código com acesso somente leitura e um executor de testes que pode executar comandos.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      # description tells Claude when to use this subagent
                      description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                      # prompt defines the subagent's behavior and expertise
                      prompt="""You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.""",
                      # tools restricts what the subagent can do (read-only here)
                      tools=["Read", "Grep", "Glob"],
                      # model overrides the default model for this subagent
                      model="sonnet",
                  ),
                  "test-runner": AgentDefinition(
                      description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                      prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures""",
                      # Bash access lets this subagent run test commands
                      tools=["Bash", "Read", "Grep"],
                  ),
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Review the authentication module for security issues",
    options: {
      // Auto-approve these tools
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-reviewer": {
          // description tells Claude when to use this subagent
          description:
            "Expert code review specialist. Use for quality, security, and maintainability reviews.",
          // prompt defines the subagent's behavior and expertise
          prompt: `You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.`,
          // tools restricts what the subagent can do (read-only here)
          tools: ["Read", "Grep", "Glob"],
          // model overrides the default model for this subagent
          model: "sonnet"
        },
        "test-runner": {
          description:
            "Runs and analyzes test suites. Use for test execution and coverage analysis.",
          prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures`,
          // Bash access lets this subagent run test commands
          tools: ["Bash", "Read", "Grep"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="agentdefinition-configuration">
  Configuração de AgentDefinition
</h3>

| Campo             | Tipo                                                        | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                                       |
| :---------------- | :---------------------------------------------------------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Sim         | Descrição em linguagem natural de quando usar este agente                                                                                                                                                                                                                                                                                                                                       |
| `prompt`          | `string`                                                    | Sim         | O prompt do sistema do agente definindo seu papel e comportamento                                                                                                                                                                                                                                                                                                                               |
| `tools`           | `string[]`                                                  | Não         | Array de nomes de ferramentas permitidas. Se omitido, herda todas as [ferramentas disponíveis para subagentes](/docs/pt/sub-agents#available-tools)                                                                                                                                                                                                                                                  |
| `disallowedTools` | `string[]`                                                  | Não         | Array de nomes de ferramentas a remover do conjunto de ferramentas do agente. Padrões de nível de servidor MCP também são aceitos: `mcp__server` ou `mcp__server__*` remove todas as ferramentas desse servidor, e `mcp__*` remove todas as ferramentas MCP de qualquer servidor                                                                                                                |
| `model`           | `string`                                                    | Não         | Substituição de modelo para este agente. Aceita um alias como `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, ou um ID de modelo completo. `'inherit'` usa o modelo principal. Quando você o omite, Claude Code escolhe o modelo na [ordem de modelo de subagente](/docs/pt/sub-agents#choose-a-model)                                                                                      |
| `skills`          | `string[]`                                                  | Não         | Lista de nomes de skills a pré-carregar no contexto do agente na inicialização. Skills não listadas permanecem invocáveis através da ferramenta Skill                                                                                                                                                                                                                                           |
| `memory`          | `'user' \| 'project' \| 'local'`                            | Não         | Fonte de memória para este agente                                                                                                                                                                                                                                                                                                                                                               |
| `mcpServers`      | `(string \| object)[]`                                      | Não         | Servidores MCP disponíveis para este agente, por nome ou configuração inline                                                                                                                                                                                                                                                                                                                    |
| `initialPrompt`   | `string`                                                    | Não         | Auto-enviado como o primeiro turno do usuário quando este agente é executado como o agente de thread principal. Ignorado quando o agente é invocado como um subagente                                                                                                                                                                                                                           |
| `maxTurns`        | `number`                                                    | Não         | Número máximo de turnos agentic antes do agente parar. Quando o agente atinge o limite, Claude Code retorna sua saída marcada como parcial, e você pode [retomar o agente](#resume-subagents) para continuar. A marcação parcial requer Claude Code v2.1.246 ou posterior                                                                                                                       |
| `background`      | `boolean`                                                   | Não         | Executar este agente como uma tarefa de background não-bloqueante quando invocado                                                                                                                                                                                                                                                                                                               |
| `omitClaudeMd`    | `boolean`                                                   | Não         | Executar este agente sem os arquivos CLAUDE.md do usuário, projeto e local quando é executado como um subagente; arquivos de política gerenciados ainda são carregados. Ignorado quando o agente é executado como o agente de thread principal. Requer TypeScript Agent SDK v0.3.271 ou posterior. O SDK Python [`AgentDefinition`](/docs/pt/agent-sdk/python#agentdefinition) não possui este campo |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | Não         | Nível de esforço de raciocínio para este agente                                                                                                                                                                                                                                                                                                                                                 |
| `permissionMode`  | `PermissionMode`                                            | Não         | Modo de permissão para execução de ferramentas dentro deste agente. As [regras de herança de subagente](/docs/pt/agent-sdk/permissions#available-modes) decidem quando se aplica                                                                                                                                                                                                                     |

No SDK Python, nomes de campos com múltiplas palavras como `disallowedTools` e `mcpServers` mantêm sua ortografia camelCase para corresponder ao formato de transmissão em vez de seguir a convenção snake\_case do Python. Consulte a referência [`AgentDefinition`](/docs/pt/agent-sdk/python#agentdefinition) para detalhes.

Subagentes são executados em background por padrão. Uma chamada da ferramenta Agent que omite a entrada [`run_in_background`](/docs/pt/sub-agents#run-subagents-in-foreground-or-background) inicia um subagente em background, e Claude define `run_in_background: false` quando precisa do resultado antes de continuar. Defina o campo `background` como `true` para forçar execução em background para um agente específico independentemente do que Claude solicita. Antes do Claude Code v2.1.198, o padrão de background estava sendo implementado gradualmente, e uma chamada da ferramenta Agent que omitia `run_in_background` poderia executar o subagente sincronamente.

Subagentes também podem gerar subagentes próprios. Para limitar a profundidade dessa aninhação, quantos subagentes são executados de uma vez e quanto uma consulta gasta, consulte [Limitar profundidade, concorrência e gasto de subagentes](#cap-subagent-depth-concurrency-and-spend).

<h3 id="filesystem-based-definition-alternative">
  Definição baseada em sistema de arquivos (alternativa)
</h3>

Você também pode definir subagentes como arquivos markdown em diretórios `.claude/agents/`. Consulte a [documentação de subagentes do Claude Code](/docs/pt/sub-agents) para detalhes sobre essa abordagem. Agentes definidos programaticamente têm precedência sobre agentes baseados em sistema de arquivos com o mesmo nome.

<Note>
  Quando Claude chama a ferramenta Agent sem um `subagent_type`, ele obtém o subagente `general-purpose` integrado, que Claude pode gerar mesmo quando você não define agentes próprios. Definir [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/pt/env-vars) remove esse padrão, e tal chamada falha com [`subagent_type is required`](/docs/pt/errors#subagent-type-is-required).
</Note>

<h2 id="what-subagents-inherit">
  O que os subagentes herdam
</h2>

A menos que o subagente seja um [fork](/docs/pt/sub-agents#fork-the-current-conversation), sua janela de contexto começa do zero, sem a conversa pai, mas não está vazia. O único conteúdo que você passa do pai para o subagente é a string de prompt da ferramenta Agent, então inclua quaisquer caminhos de arquivo, mensagens de erro ou decisões que o subagente precise diretamente nesse prompt.

Um subagente que possui a ferramenta [`SendMessage`](/docs/pt/tools-reference) começa com uma lista dos outros agentes nomeados em execução na sessão, para que saiba quais nomes pode usar para enviar mensagens. Claude Code adiciona a lista ao primeiro turno do subagente automaticamente. Um [fork](/docs/pt/sub-agents#fork-the-current-conversation) não recebe a lista porque herda a conversa pai em vez disso.

Um subagente também herda a configuração de pensamento estendido da sessão principal.

A tabela abaixo lista o que o contexto de um subagente não-fork contém e o que deixa de fora.

| O subagente recebe                                                                                                                                                                                                     | O subagente não recebe                                                           |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------- |
| Seu próprio prompt de sistema (`AgentDefinition.prompt`) e o prompt da ferramenta Agent                                                                                                                                | O histórico de conversa ou resultados de ferramentas do pai                      |
| Project CLAUDE.md (carregado via [`settingSources`](/docs/pt/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)), a menos que o agente defina [`omitClaudeMd`](#agentdefinition-configuration) | Conteúdo de skill pré-carregado, a menos que listado em `AgentDefinition.skills` |
| Definições de ferramentas (herdadas do pai ou o subconjunto em `tools`, [filtrado para execuções em background](/docs/pt/sub-agents#available-tools))                                                                       | O prompt de sistema do pai                                                       |

<Note>
  O pai recebe a mensagem final do subagente como resultado da ferramenta Agent, mas pode resumi-la em sua própria resposta. Para preservar a saída do subagente verbatim na resposta voltada para o usuário, inclua uma instrução para fazer isso no prompt ou na opção `systemPrompt` que você passa para a chamada principal `query()`.

  Na v2.1.210 e posterior, Claude Code [verifica a mensagem final para padrões em forma de instrução](/docs/pt/sub-agents#subagent-output-scanning) antes do pai lê-la. A verificação trata três tipos de padrão de forma diferente:

  * **Imitação de tag de controle**: Claude Code neutraliza uma tag que apenas o harness emite, como um bloco `<system-reminder>`, no local. Ele insere uma barra invertida após o colchete angular de abertura e não deleta nada.
  * **Menções de configuração de permissão**: Claude Code mantém referências à configuração de permissão, como `.claude/settings.json`, `bypassPermissions`, ou `--dangerously-skip-permissions`, conforme escrito.
  * **Marcadores de turno**: uma linha que começa com `Human:` ou `Assistant:` recebe uma barra invertida antes dos dois-pontos, para que a mensagem não possa imitar um limite de turno de conversa.

  Para uma correspondência de tag de controle ou configuração de permissão, Claude Code prepara uma linha de marcador `[harness: ...]` nomeando os padrões correspondidos; uma correspondência de marcador de turno não adiciona a linha de marcador. Essas são as únicas modificações que a verificação faz: ela nunca remove ou reformula o texto do subagente.
</Note>

Um erro de API que encerra o subagente mais cedo, como um limite de taxa, nunca é entregue como seu resultado. Veja [Erros de API em subagentes](/docs/pt/sub-agents#api-errors-in-subagents) para o comportamento em primeiro plano e em background.

<h2 id="invoke-subagents">
  Invocar suagentes
</h2>

<h3 id="automatic-invocation">
  Invocação automática
</h3>

Claude decide automaticamente quando invocar suagentes com base na tarefa e na `description` de cada suagente. Por exemplo, se você definir um suagente `performance-optimizer` com a descrição "Performance optimization specialist for query tuning", Claude o invocará quando seu prompt mencionar otimização de consultas.

Escreva descrições claras e específicas para que Claude possa corresponder tarefas ao suagente correto.

<h3 id="explicit-invocation">
  Invocação explícita
</h3>

Para garantir que Claude use um suagente específico, mencione-o pelo nome em seu prompt:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Isso ignora a correspondência automática e invoca diretamente o suagente nomeado.

<h3 id="dynamic-agent-configuration">
  Configuração dinâmica de agentes
</h3>

Você pode criar definições de agentes dinamicamente com base em condições de tempo de execução. Este exemplo cria um revisor de segurança com diferentes níveis de rigor, usando um modelo mais capaz para revisões rigorosas.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  # Factory function that returns an AgentDefinition
  # This pattern lets you customize agents based on runtime conditions
  def create_security_agent(security_level: str) -> AgentDefinition:
      is_strict = security_level == "strict"
      return AgentDefinition(
          description="Security code reviewer",
          # Customize the prompt based on strictness level
          prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
          tools=["Read", "Grep", "Glob"],
          # Key insight: use a more capable model for high-stakes reviews
          model="opus" if is_strict else "sonnet",
      )


  async def main():
      # The agent is created at query time, so each request can use different settings
      async for message in query(
          prompt="Review this PR for security issues",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  # Call the factory with your desired configuration
                  "security-reviewer": create_security_agent("strict")
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

  // Factory function that returns an AgentDefinition
  // This pattern lets you customize agents based on runtime conditions
  function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
    const isStrict = securityLevel === "strict";
    return {
      description: "Security code reviewer",
      // Customize the prompt based on strictness level
      prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
      tools: ["Read", "Grep", "Glob"],
      // Key insight: use a more capable model for high-stakes reviews
      model: isStrict ? "opus" : "sonnet"
    };
  }

  // The agent is created at query time, so each request can use different settings
  for await (const message of query({
    prompt: "Review this PR for security issues",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        // Call the factory with your desired configuration
        "security-reviewer": createSecurityAgent("strict")
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h2 id="detect-subagent-invocation">
  Detectar invocação de subagente
</h2>

Claude invoca subagentes através da ferramenta Agent. Para detectar quando um subagente é invocado, procure por blocos `tool_use` onde `name` é `"Agent"`. Mensagens de dentro do contexto de um subagente incluem um campo `parent_tool_use_id`.

<Note>
  A ferramenta aparece como `"Agent"` em blocos `tool_use`, mas como `"Task"` na lista de ferramentas `system:init`. Antes do Claude Code v2.1.63, blocos `tool_use` também a nomeavam como `"Task"`. Para manter a detecção funcionando em diferentes versões do SDK, corresponda ambos os valores em `block.name`.
</Note>

A estrutura da mensagem difere entre SDKs. Em Python, você acessa blocos de conteúdo diretamente via `message.content`. Em TypeScript, `SDKAssistantMessage` envolve a mensagem da API Claude, então você acessa o conteúdo via `message.message.content`.

Este exemplo itera através de mensagens transmitidas, registrando quando um subagente é invocado e quando mensagens subsequentes originam-se de dentro do contexto de execução desse subagente.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


  async def main():
      async for message in query(
          prompt="Use the code-reviewer agent to review this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      description="Expert code reviewer.",
                      prompt="Analyze code quality and suggest improvements.",
                      tools=["Read", "Glob", "Grep"],
                  )
              },
          ),
      ):
          # Check for subagent invocation. Match both names: older SDK
          # versions emitted "Task", current versions emit "Agent".
          if hasattr(message, "content") and message.content:
              for block in message.content:
                  if isinstance(block, ToolUseBlock) and block.name in (
                      "Task",
                      "Agent",
                  ):
                      print(f"Subagent invoked: {block.input.get('subagent_type')}")

          # Check if this message is from within a subagent's context
          if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
              print("  (running inside subagent)")

          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the code-reviewer agent to review this codebase",
    options: {
      allowedTools: ["Read", "Glob", "Grep", "Agent"],
      agents: {
        "code-reviewer": {
          description: "Expert code reviewer.",
          prompt: "Analyze code quality and suggest improvements.",
          tools: ["Read", "Glob", "Grep"]
        }
      }
    }
  })) {
    const msg = message as any;

    // Check for subagent invocation. Match both names: older SDK versions
    // emitted "Task", current versions emit "Agent".
    for (const block of msg.message?.content ?? []) {
      if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
        console.log(`Subagent invoked: ${block.input.subagent_type}`);
      }
    }

    // Check if this message is from within a subagent's context
    if (msg.parent_tool_use_id) {
      console.log("  (running inside subagent)");
    }

    if ("result" in message) {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h2 id="resume-subagents">
  Retomar subagentes
</h2>

Você pode retomar um subagente para continuar de onde parou em vez de começar do zero. Um subagente retomado mantém seu histórico de conversa completo, incluindo todas as chamadas de ferramentas anteriores, resultados e raciocínio.

Quando um subagente para no limite de [`maxTurns`](#agentdefinition-configuration), Claude Code marca a saída no resultado da ferramenta Agent como parcial, para que Claude saiba que a execução está inacabada.

Quando um subagente é concluído, o resultado da ferramenta Agent inclui um bloco de texto contendo `agentId: <id>`. Os agentes [`Explore` e `Plan`](/docs/pt/sub-agents#built-in-subagents) integrados são de uma única tentativa e não retornam um `agentId`, portanto use um agente personalizado ou `general-purpose` quando precisar retomar. Para retomar um subagente programaticamente:

1. **Capture o ID da sessão**: extraia `session_id` das mensagens durante a primeira consulta
2. **Extraia o ID do agente**: analise `agentId` do texto do resultado da ferramenta Agent
3. **Retome a sessão**: passe `resume: sessionId` nas opções da segunda consulta e inclua o ID do agente no seu prompt. Cada chamada `query()` inicia uma nova sessão por padrão, e você deve retomar a mesma sessão para acessar a transcrição do subagente.

<Note>
  Ao usar um agente personalizado, passe a mesma definição de agente no parâmetro `agents` para ambas as consultas.
</Note>

O exemplo abaixo define um agente `endpoint-finder` personalizado. A primeira consulta o executa e captura o ID da sessão e o ID do agente do resultado da ferramenta Agent, então a segunda consulta retoma a sessão para fazer uma pergunta de acompanhamento que requer contexto da primeira análise.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import re
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

  AGENTS = {
      "endpoint-finder": AgentDefinition(
          description="Locates and catalogs API endpoints in a codebase.",
          prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
          tools=["Read", "Grep", "Glob"],
      )
  }


  def extract_agent_id(block: ToolResultBlock) -> str | None:
      """Extract agentId from an Agent tool result's text content."""
      parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
      for part in parts:
          if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
              return match.group(1)
      return None


  async def main():
      agent_id = None
      session_id = None

      # First invocation - run the endpoint-finder subagent
      try:
          async for message in query(
              prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
          ):
              # Capture session_id from ResultMessage (needed to resume this session)
              if hasattr(message, "session_id"):
                  session_id = message.session_id
              # Search tool results for the agentId trailer
              for block in getattr(message, "content", None) or []:
                  if isinstance(block, ToolResultBlock):
                      agent_id = extract_agent_id(block) or agent_id
              # Print the final result
              if hasattr(message, "result"):
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so session_id and agent_id have already been captured by the loop above.
          print(f"Session ended with an error: {error}")

      # Second invocation - resume and ask follow-up
      if agent_id and session_id:
          async for message in query(
              prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)
      else:
          print("No agentId found in the first query, so there is no subagent to resume.")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

  const agents = {
    "endpoint-finder": {
      description: "Locates and catalogs API endpoints in a codebase.",
      prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
      tools: ["Read", "Grep", "Glob"]
    }
  };

  // Stringify content to search for agentId without traversing nested block types
  function extractAgentId(message: SDKMessage): string | undefined {
    if (message.type !== "assistant" && message.type !== "user") return undefined;
    const content = JSON.stringify(message.message.content);
    const match = content.match(/agentId:\s*([\w-]+)/);
    return match?.[1];
  }

  let agentId: string | undefined;
  let sessionId: string | undefined;

  // First invocation - run the endpoint-finder subagent
  try {
    for await (const message of query({
      prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
    })) {
      // Capture session_id from ResultMessage (needed to resume this session)
      if ("session_id" in message) sessionId = message.session_id;
      // Search message content for the agentId (appears in Agent tool results)
      const extractedId = extractAgentId(message);
      if (extractedId) agentId = extractedId;
      // Print the final result
      if ("result" in message) console.log(message.result);
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so sessionId and agentId have already been captured by the loop above.
    console.error(`Session ended with an error: ${error}`);
  }

  // Second invocation - resume and ask follow-up
  if (agentId && sessionId) {
    for await (const message of query({
      prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
    })) {
      if ("result" in message) console.log(message.result);
    }
  } else {
    console.log("No agentId found in the first query, so there is no subagent to resume.");
  }
  ```
</CodeGroup>

As transcrições de subagentes são armazenadas em arquivos separados e persistem independentemente da conversa principal. Consulte [retomar subagentes em Claude Code](/docs/pt/sub-agents#resume-subagents) para o comportamento de compactação e o período de limpeza `cleanupPeriodDays`.

<h2 id="tool-restrictions">
  Restrições de ferramentas
</h2>

Use o campo `tools` para limitar o que um suagente pode fazer:

* **Omitir `tools`**: o suagente obtém todas as [ferramentas disponíveis para suagentes](/docs/pt/sub-agents#available-tools)
* **Listar ferramentas**: o suagente obtém apenas aquelas. Um revisor de código que nunca deve editar arquivos, por exemplo, obtém `["Read", "Grep", "Glob"]`

Uma ferramenta que você deixa de fora não está na sessão do suagente: Claude funciona sem ela, sem prompt de permissão ou erro.

Este exemplo cria um agente de análise somente leitura que pode examinar código, mas não pode modificar arquivos ou executar comandos.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Analyze the architecture of this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-analyzer": AgentDefinition(
                      description="Static code analysis and architecture review",
                      prompt="""You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.""",
                      # Read-only tools: no Edit, Write, or Bash access
                      tools=["Read", "Grep", "Glob"],
                  )
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Analyze the architecture of this codebase",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-analyzer": {
          description: "Static code analysis and architecture review",
          prompt: `You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.`,
          // Read-only tools: no Edit, Write, or Bash access
          tools: ["Read", "Grep", "Glob"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="common-tool-combinations">
  Combinações comuns de ferramentas
</h3>

| Caso de uso             | Ferramentas                             | Descrição                                                               |
| :---------------------- | :-------------------------------------- | :---------------------------------------------------------------------- |
| Análise somente leitura | `Read`, `Grep`, `Glob`                  | Pode examinar código, mas não modificar ou executar                     |
| Execução de testes      | `Bash`, `Read`, `Grep`                  | Pode executar comandos e analisar saída                                 |
| Modificação de código   | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Acesso completo de leitura/escrita sem execução de comandos             |
| Acesso completo         | Todas as ferramentas                    | Herda as ferramentas disponíveis para suagentes (omita o campo `tools`) |

<h2 id="cap-subagent-depth-concurrency-and-spend">
  Limitar profundidade, concorrência e gastos de subagentes
</h2>

<Note>
  Esta seção descreve TypeScript SDK v0.3.219 e Python SDK v0.2.127 e posteriores, as versões que incluem Claude Code v2.1.219 ou posteriores. Em versões anteriores, alguns desses limites estão ausentes ou têm padrões diferentes, portanto atualize antes de confiar neles para limitar uma execução. A [referência de variáveis de ambiente](/docs/pt/env-vars) e [turnos e orçamento](/docs/pt/agent-sdk/agent-loop#turns-and-budget) registram a versão do Claude Code que adicionou cada variável e a aplicação do limite de gastos do subagente.
</Note>

Claude decide por conta própria quando gerar um subagente e quantos gerar. Cada subagente faz suas próprias solicitações de API, que contam para o `total_cost_usd` da consulta, e um subagente pode gerar subagentes próprios, portanto um prompt pode crescer em uma árvore de agentes.

Você pode limitar esse crescimento de três maneiras: quão profundamente os subagentes se aninham, quantos são executados simultaneamente e quanto a consulta inteira gasta. Defina os limites de profundidade e concorrência como variáveis de ambiente através da opção [`env`](/docs/pt/agent-sdk/typescript#options), e o limite de gastos como uma opção de consulta:

| Limite       | Defina com                                               | Padrão                                                                                                                       | O que Claude Code faz no limite                                                                                                                                                                                                                                                                                                                                     |
| :----------- | :------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Profundidade | [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/pt/env-vars)   | `3` camadas de subagentes abaixo do seu agente principal. `1` impede que seus subagentes gerem qualquer um dos seus próprios | Deixa um subagente na camada inferior incapaz de gerar, portanto ele faz seu trabalho delegado por conta própria. Veja [subagentes aninhados](/docs/pt/sub-agents#let-subagents-spawn-their-own-subagents)                                                                                                                                                               |
| Concorrência | [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/pt/env-vars)   | `20` subagentes em execução simultaneamente, contando cada subagente que Claude gera com a ferramenta Agent                  | Recusa gerar outro subagente, retornando `Concurrent subagent limit reached`, até que a contagem em execução caia abaixo do limite. Sessões com [ultracode](/docs/pt/model-config#adjust-effort-level) ativo nunca são recusadas. Veja o [limite de subagente concorrente](/docs/pt/sub-agents#concurrent-subagent-limit)                                                     |
| Gastos       | `maxBudgetUsd` em TypeScript, `max_budget_usd` em Python | Sem limite. Conta o gasto da própria chamada, incluindo solicitações de subagentes                                           | Aplica o limite de três maneiras: recusa gerar mais subagentes, retornando `Budget limit reached`, interrompe subagentes em segundo plano que ainda estão em execução e encerra a consulta com o subtipo de resultado `error_max_budget_usd`. Para como os limites se comportam em uma sessão, veja [turnos e orçamento](/docs/pt/agent-sdk/agent-loop#turns-and-budget) |

Os dois SDKs tratam a opção `env` de forma diferente: o SDK TypeScript substitui o ambiente do subprocesso por ela, portanto espalhe `process.env` nela para manter variáveis como `PATH`, enquanto o SDK Python a mescla no ambiente herdado. Este exemplo desativa o aninhamento, permite no máximo cinco subagentes por vez e interrompe a consulta uma vez que o gasto estimado atinja \$5:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      try:
          async for message in query(
              prompt="Audit every service in this repo for unhandled promise rejections",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"],
                  # env is merged on top of the inherited environment
                  env={
                      "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "1",
                      "CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS": "5",
                  },
                  max_budget_usd=5.0,
              ),
          ):
              if isinstance(message, ResultMessage):
                  print(f"{message.subtype}: ${message.total_cost_usd}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the budget-capped result has already been printed above.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Audit every service in this repo for unhandled promise rejections",
      options: {
        allowedTools: ["Read", "Grep", "Glob", "Agent"],
        // env replaces the subprocess environment, so spread process.env to keep PATH
        env: {
          ...process.env,
          CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH: "1",
          CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS: "5",
        },
        maxBudgetUsd: 5,
      },
    })) {
      if (message.type === "result") {
        console.log(`${message.subtype}: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the budget-capped result has already been logged above.
    console.error(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

O que você vê depende de qual limite, se houver, a consulta atinge:

* **Abaixo do limite de gastos**: você vê `success` e o custo estimado.
* **No limite de gastos**: você vê `error_max_budget_usd` com um custo em ou acima de `5`, e então seu manipulador de erro é executado.
* **No limite de concorrência**: você vê um bloco `tool_result` no fluxo de mensagens carregando `Concurrent subagent limit reached`. Claude recebe o mesmo bloco como resultado da ferramenta Agent.

<h3 id="run-opus-5-with-subagents">
  Executar Opus 5 com subagentes
</h3>

Claude Opus 5 delega para subagentes mais prontamente do que modelos anteriores, portanto os [limites de profundidade, concorrência e gastos](#cap-subagent-depth-concurrency-and-spend) importam mais em consultas que executam Opus 5. O [guia de prompting do Opus 5](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5#controlling-subagent-spawning) tem uma instrução de delegação que você pode adicionar a qualquer prompt. Se Claude Code adiciona uma instrução própria depende de qual [prompt do sistema](/docs/pt/agent-sdk/modifying-system-prompts#how-system-prompts-work) você usa:

* **Predefinição `claude_code`**: quando o modelo é Opus 5, Claude Code adiciona uma linha ao seu prompt do sistema dizendo ao Claude para não chamar a ferramenta Agent a menos que seja solicitado. A ferramenta Agent permanece disponível.
* **Um prompt personalizado, ou nenhum `systemPrompt`**: Claude Code não constrói seu prompt do sistema, portanto essa linha está ausente. Adicione a instrução de delegação do guia de prompting ao seu próprio prompt.

Qualquer instrução apenas orienta Claude, portanto defina os limites também. Claude Code os aplica porém Claude decide delegar.

<h2 id="scale-up-with-dynamic-workflows">
  Escalar com fluxos de trabalho dinâmicos
</h2>

Subagentes funcionam bem para algumas tarefas delegadas por turno. Para execuções que coordenam dezenas a centenas de agentes, use a ferramenta `Workflow`, que move a orquestração para um script que o runtime executa fora do contexto da conversa. Veja [fluxos de trabalho dinâmicos](/docs/pt/workflows) para como fluxos de trabalho diferem da delegação de subagentes turno a turno.

A ferramenta `Workflow` está disponível no TypeScript Agent SDK v0.3.149 e posterior. Inclua `Workflow` em `allowedTools` para aprovar automaticamente execuções de fluxo de trabalho. Os esquemas de entrada e saída da ferramenta estão listados na [referência TypeScript](/docs/pt/agent-sdk/typescript#workflow).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-delegating-to-subagents">
  Claude não delegando para subagentes
</h3>

Se Claude completa tarefas diretamente em vez de delegar para seu subagente:

* **Use prompting explícito**: mencione o subagente pelo nome em seu prompt, por exemplo "Use o agente code-reviewer para verificar o módulo de autenticação"
* **Escreva uma descrição clara**: explique exatamente quando usar o subagente para que Claude possa corresponder tarefas apropriadamente

<h3 id="filesystem-based-agents-not-loading">
  Agentes baseados em sistema de arquivos não carregando
</h3>

Claude Code monitora `~/.claude/agents/` e `.claude/agents/` e detecta um arquivo de agente novo ou editado em alguns segundos, sem necessidade de reinicialização. Se uma definição nunca aparecer, trabalhe através dessas causas:

* **Novo diretório `agents`**: o monitor cobre apenas diretórios que existiam quando a sessão começou, então o primeiro arquivo em um novo diretório precisa de uma reinicialização de sessão. Esta é a causa mais comum.
* **Frontmatter inválido ou um `name` duplicado**: verifique o YAML do arquivo e se um agente existente já usa o `name`.
* **`--disable-slash-commands`**: sessões iniciadas com essa flag não monitoram esses diretórios e sempre precisam de uma reinicialização para carregar novos arquivos.
* **Um arquivo sob um diretório adicionado**: Claude Code carrega `.claude/agents/` de diretórios adicionados com a opção `add_dirs` (Python) ou `additionalDirectories` (TypeScript), ou a CLI `--add-dir` ou `/add-dir`, mas não os monitora, então um arquivo novo ou editado lá precisa de uma reinicialização de sessão.
* **Um agente programático com o mesmo nome**: `agents` passados para `query()` substituem um agente do sistema de arquivos com o mesmo nome.

Para o formato do arquivo, veja [como escrever arquivos de subagente](/docs/pt/sub-agents#write-subagent-files).

<h2 id="related-documentation">
  Documentação relacionada
</h2>

* [Subagentes Claude Code](/docs/pt/sub-agents): documentação abrangente de subagentes incluindo definições baseadas em sistema de arquivos
* [Fluxos de trabalho dinâmicos](/docs/pt/workflows): orquestre muitos subagentes a partir de um script para trabalhos muito grandes para uma conversa
* [Visão geral do SDK](/docs/pt/agent-sdk/overview): começando com o Claude Agent SDK
