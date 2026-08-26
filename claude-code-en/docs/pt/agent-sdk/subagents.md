> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagentes no SDK

> Defina e invoque subagentes para isolar contexto, executar tarefas em paralelo e aplicar instruções especializadas em suas aplicações Claude Agent SDK.

Subagentes são instâncias de agente separadas que seu agente principal pode gerar para lidar com subtarefas focadas.
Use subagentes para isolar contexto, executar múltiplas análises em paralelo e aplicar instruções especializadas sem adicionar ao prompt do agente principal.

Este guia explica como definir e usar subagentes no SDK usando o parâmetro `agents`.

<h2 id="overview">
  Visão geral
</h2>

Você pode criar subagentes de três maneiras:

* **Programaticamente**: use o parâmetro `agents` em suas opções `query()`. Veja as referências [TypeScript](/docs/pt/agent-sdk/typescript#agentdefinition) e [Python](/docs/pt/agent-sdk/python#agentdefinition)
* **Baseado em sistema de arquivos**: defina agentes como arquivos markdown em diretórios `.claude/agents/`. Veja [definindo subagentes como arquivos](/docs/pt/sub-agents)
* **Propósito geral integrado**: Claude pode invocar o subagente integrado `general-purpose` a qualquer momento via a ferramenta Agent sem você definir nada

Este guia se concentra na abordagem programática, que é recomendada para aplicações SDK.

Quando você define subagentes, Claude determina se deve invocá-los com base no campo `description` de cada subagente. Escreva descrições claras que expliquem quando usar o subagente, e Claude delegará automaticamente tarefas apropriadas. Você também pode solicitar explicitamente um subagente pelo nome em seu prompt, por exemplo "Use o agente code-reviewer para...".

<h2 id="benefits-of-using-subagents">
  Benefícios de usar subagentes
</h2>

<h3 id="context-isolation">
  Isolamento de contexto
</h3>

Cada subagente é executado em sua própria conversa nova. Chamadas de ferramentas intermediárias e resultados permanecem dentro do subagente; apenas sua mensagem final retorna ao pai. Veja [O que subagentes herdam](#what-subagents-inherit) para saber exatamente o que está no contexto do subagente.

**Exemplo:** um subagente `research-assistant` pode explorar dezenas de arquivos sem que nenhum desse conteúdo se acumule na conversa principal. O pai recebe um resumo conciso, não cada arquivo que o subagente leu.

<h3 id="parallelization">
  Paralelização
</h3>

Múltiplos subagentes podem ser executados simultaneamente, portanto subtarefas independentes terminam no tempo do mais lento em vez da soma de todos eles.

**Exemplo:** durante uma revisão de código, você pode executar os subagentes `style-checker`, `security-scanner` e `test-coverage` simultaneamente em vez de sequencialmente.

<h3 id="specialized-instructions-and-knowledge">
  Instruções e conhecimento especializados
</h3>

Cada subagente pode ter prompts de sistema personalizados com expertise específica, melhores práticas e restrições.

**Exemplo:** um subagente `database-migration` pode ter conhecimento detalhado sobre melhores práticas SQL, estratégias de reversão e verificações de integridade de dados que seriam ruído desnecessário nas instruções do agente principal.

<h3 id="tool-restrictions">
  Restrições de ferramentas
</h3>

Subagentes podem ser limitados a ferramentas específicas, reduzindo o risco de ações não intencionais.

**Exemplo:** um subagente `doc-reviewer` pode ter acesso apenas às ferramentas Read e Grep, garantindo que possa analisar mas nunca modifique acidentalmente seus arquivos de documentação.

<h2 id="create-subagents">
  Criar subagentes
</h2>

<h3 id="programmatic-definition-recommended">
  Definição programática (recomendada)
</h3>

Defina subagentes diretamente em seu código usando o parâmetro `agents`. Claude invoca subagentes através da ferramenta `Agent`, portanto inclua `Agent` em `allowedTools` para aprovar automaticamente invocações de subagentes sem um prompt de permissão.

A maioria dos exemplos nesta página imprime apenas o resultado final. Para confirmar que Claude delegou a um subagente em vez de responder diretamente, veja [Detectar invocação de subagente](#detect-subagent-invocation).

Este exemplo cria dois subagentes: um revisor de código com acesso somente leitura e um executor de testes que pode executar comandos.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools, including Agent for subagent invocation
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
      // Auto-approve these tools, including Agent for subagent invocation
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

| Campo             | Tipo                                                        | Obrigatório | Descrição                                                                                                                                                                                                                                                                        |
| :---------------- | :---------------------------------------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Sim         | Descrição em linguagem natural de quando usar este agente                                                                                                                                                                                                                        |
| `prompt`          | `string`                                                    | Sim         | O prompt do sistema do agente definindo seu papel e comportamento                                                                                                                                                                                                                |
| `tools`           | `string[]`                                                  | Não         | Array de nomes de ferramentas permitidas. Se omitido, herda todas as ferramentas                                                                                                                                                                                                 |
| `disallowedTools` | `string[]`                                                  | Não         | Array de nomes de ferramentas a remover do conjunto de ferramentas do agente. Padrões de nível de servidor MCP também são aceitos: `mcp__server` ou `mcp__server__*` remove todas as ferramentas desse servidor, e `mcp__*` remove todas as ferramentas MCP de qualquer servidor |
| `model`           | `string`                                                    | Não         | Substituição de modelo para este agente. Aceita um alias como `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, ou um ID de modelo completo. Padrão é o modelo principal se omitido                                                                                       |
| `skills`          | `string[]`                                                  | Não         | Lista de nomes de skills para pré-carregar no contexto do agente na inicialização. Skills não listadas permanecem invocáveis através da ferramenta Skill                                                                                                                         |
| `memory`          | `'user' \| 'project' \| 'local'`                            | Não         | Fonte de memória para este agente                                                                                                                                                                                                                                                |
| `mcpServers`      | `(string \| object)[]`                                      | Não         | Servidores MCP disponíveis para este agente, por nome ou configuração inline                                                                                                                                                                                                     |
| `initialPrompt`   | `string`                                                    | Não         | Auto-enviado como o primeiro turno do usuário quando este agente é executado como o agente da thread principal. Ignorado quando o agente é invocado como um subagente                                                                                                            |
| `maxTurns`        | `number`                                                    | Não         | Número máximo de turnos agentic antes do agente parar                                                                                                                                                                                                                            |
| `background`      | `boolean`                                                   | Não         | Executar este agente como uma tarefa de fundo não-bloqueante quando invocado                                                                                                                                                                                                     |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | Não         | Nível de esforço de raciocínio para este agente                                                                                                                                                                                                                                  |
| `permissionMode`  | `PermissionMode`                                            | Não         | Modo de permissão para execução de ferramentas dentro deste agente                                                                                                                                                                                                               |

No SDK Python, nomes de campo com múltiplas palavras como `disallowedTools` e `mcpServers` mantêm sua ortografia camelCase para corresponder ao formato de transmissão em vez de seguir a convenção snake\_case do Python. Veja a referência [`AgentDefinition`](/docs/pt/agent-sdk/python#agentdefinition) para detalhes.

Dois comportamentos de subagente mudaram no Claude Code v2.1.198:

* Subagentes são executados em fundo por padrão. Uma chamada de ferramenta Agent que omite a entrada [`run_in_background`](/docs/pt/agent-sdk/typescript) inicia um subagente em fundo, e Claude define `run_in_background: false` quando precisa do resultado antes de continuar. Antes da v2.1.198, omitir `run_in_background` executava o subagente sincronamente. Defina o campo `background` como `true` para forçar execução em fundo para um agente específico independentemente do que Claude solicita.
* Um subagente herda a configuração de pensamento estendido da sessão principal. Em versões anteriores, o pensamento estendido é desabilitado dentro de subagentes independentemente da configuração da sessão principal.

<Note>
  A partir do Claude Code v2.1.172, subagentes podem gerar seus próprios subagentes. Um subagente cinco níveis abaixo do agente principal não pode gerar mais subagentes, independentemente de ser executado em primeiro plano ou em fundo. Para evitar que um subagente gere outros, omita `Agent` de seu array `tools` ou adicione-o a `disallowedTools`. Veja [subagentes aninhados](/docs/pt/sub-agents#spawn-nested-subagents) para as regras de profundidade completas.
</Note>

<h3 id="filesystem-based-definition-alternative">
  Definição baseada em sistema de arquivos (alternativa)
</h3>

Você também pode definir subagentes como arquivos markdown em diretórios `.claude/agents/`. Veja a [documentação de subagentes Claude Code](/docs/pt/sub-agents) para detalhes sobre essa abordagem. Agentes definidos programaticamente têm precedência sobre agentes baseados em sistema de arquivos com o mesmo nome.

<Note>
  Mesmo sem definir subagentes personalizados, Claude pode gerar o subagente integrado `general-purpose`. Isso é útil para delegar tarefas de pesquisa ou exploração sem criar agentes especializados. Inclua `Agent` em `allowedTools` para que essas invocações sejam aprovadas automaticamente sem um prompt de permissão.
</Note>

<h2 id="what-subagents-inherit">
  O que subagentes herdam
</h2>

A janela de contexto de um subagente começa nova, sem conversa pai, mas não está vazia. O único conteúdo que você passa do pai para o subagente é a string de prompt da ferramenta Agent, então inclua quaisquer caminhos de arquivo, mensagens de erro ou decisões que o subagente precise diretamente nesse prompt.

Um subagente que possui a ferramenta [`SendMessage`](/docs/pt/tools-reference) começa com uma lista dos outros agentes nomeados em execução na sessão, para que saiba quais nomes pode enviar mensagens. Claude Code adiciona a lista ao primeiro turno do subagente automaticamente. Um [fork](/docs/pt/sub-agents#fork-the-current-conversation) não recebe a lista porque herda a conversa do pai. A lista requer Claude Code v2.1.206 ou posterior.

| O subagente recebe                                                                                                                          | O subagente não recebe                                                           |
| :------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------- |
| Seu próprio prompt do sistema (`AgentDefinition.prompt`) e o prompt da ferramenta Agent                                                     | O histórico de conversa do pai ou resultados de ferramentas                      |
| CLAUDE.md do projeto (carregado via [`settingSources`](/docs/pt/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)) | Conteúdo de skill pré-carregado, a menos que listado em `AgentDefinition.skills` |
| Definições de ferramentas (herdadas do pai, ou o subconjunto em `tools`)                                                                    | O prompt do sistema do pai                                                       |

<Note>
  O pai recebe a mensagem final do subagente verbatim como o resultado da ferramenta Agent, mas pode resumi-la em sua própria resposta. Para preservar a saída do subagente verbatim na resposta voltada para o usuário, inclua uma instrução para fazer isso no prompt ou opção `systemPrompt` que você passa para a chamada principal `query()`.
</Note>

Um erro de API que encerra o subagente antecipadamente, como um limite de taxa, nunca é entregue como seu resultado. Se um limite de taxa, sobrecarga ou erro de servidor cortar um subagente em primeiro plano que já produziu saída de texto, a ferramenta Agent retorna essa saída parcial com uma nota de que o subagente não terminou. Um subagente que não produziu nada, ou cuja única saída foram chamadas de ferramentas sem texto, falha com uma mensagem de erro, `Agent terminated early due to an API error`, seguida pelo detalhe do erro. Veja [API errors in subagents](/docs/pt/sub-agents#api-errors-in-subagents) para o comportamento em primeiro plano e em segundo plano.

Este tratamento de saída parcial requer Claude Code v2.1.199 ou posterior. Na v2.1.199, um limite de taxa, sobrecarga ou erro de servidor deixou a forma apenas de chamadas de ferramentas com um resultado parcial vazio contendo apenas a nota de corte.

<h2 id="invoke-subagents">
  Invocando subagentes
</h2>

<h3 id="automatic-invocation">
  Invocação automática
</h3>

Claude decide automaticamente quando invocar subagentes com base na tarefa e na `description` de cada subagente. Por exemplo, se você definir um subagente `performance-optimizer` com a descrição "Performance optimization specialist for query tuning", Claude o invocará quando seu prompt mencionar otimizar consultas.

Escreva descrições claras e específicas para que Claude possa corresponder tarefas ao subagente certo.

<h3 id="explicit-invocation">
  Invocação explícita
</h3>

Para garantir que Claude use um subagente específico, mencione-o pelo nome em seu prompt:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Isso ignora a correspondência automática e invoca diretamente o subagente nomeado.

<h3 id="dynamic-agent-configuration">
  Configuração dinâmica de agente
</h3>

Você pode criar definições de agente dinamicamente com base em condições de tempo de execução. Este exemplo cria um revisor de segurança com diferentes níveis de rigor, usando um modelo mais poderoso para revisões rigorosas.

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

Claude invoca subagentes através da ferramenta Agent. Para detectar quando um subagente é invocado, verifique blocos `tool_use` onde `name` é `"Agent"`. Mensagens de dentro do contexto de um subagente incluem um campo `parent_tool_use_id`.

<Note>
  O nome da ferramenta foi renomeado de `"Task"` para `"Agent"` no Claude Code v2.1.63. Lançamentos atuais do SDK emitem `"Agent"` em blocos `tool_use` mas ainda usam `"Task"` na lista de ferramentas `system:init` e em `result.permission_denials[].tool_name`. Verificar ambos os valores em `block.name` garante compatibilidade entre versões do SDK.
</Note>

A estrutura de mensagem difere entre SDKs. Em Python, blocos de conteúdo são acessados diretamente via `message.content`. Em TypeScript, `SDKAssistantMessage` envolve a mensagem da API Claude, então o conteúdo é acessado via `message.message.content`.

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
  Retomando subagentes
</h2>

Você pode retomar um subagente para continuar de onde parou em vez de começar do zero. Um subagente retomado retém seu histórico de conversa completo, incluindo todas as chamadas de ferramentas anteriores, resultados e raciocínio.

Quando um subagente é concluído, o resultado da ferramenta Agent inclui um bloco de texto contendo `agentId: <id>`. Os agentes integrados [`Explore` e `Plan`](/docs/pt/sub-agents#built-in-subagents) são de uma única execução e não retornam um `agentId`, então use um agente personalizado ou `general-purpose` quando você precisar retomar. Para retomar um subagente programaticamente:

1. **Capture o ID da sessão**: extraia `session_id` de mensagens durante a primeira query
2. **Extraia o ID do agente**: analise `agentId` do texto do resultado da ferramenta Agent
3. **Retome a sessão**: passe `resume: sessionId` nas opções da segunda query e inclua o ID do agente em seu prompt

<Note>
  Você deve retomar a mesma sessão para acessar a transcrição do subagente. Cada chamada `query()` inicia uma nova sessão por padrão, então passe `resume: sessionId` para continuar na mesma sessão.

  Ao usar um agente personalizado, passe a mesma definição de agente no parâmetro `agents` para ambas as queries.
</Note>

O exemplo abaixo define um agente personalizado `endpoint-finder`. A primeira query o executa e captura o ID da sessão e ID do agente do resultado da ferramenta Agent, então a segunda query retoma a sessão para fazer uma pergunta de acompanhamento que requer contexto da primeira análise.

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

Transcrições de subagentes persistem independentemente da conversa principal:

* **Compactação de conversa principal**: quando a conversa principal se compacta, transcrições de subagentes não são afetadas. Elas são armazenadas em arquivos separados.
* **Persistência de sessão**: transcrições de subagentes persistem dentro de sua sessão. Você pode retomar um subagente após reiniciar Claude Code retomando a mesma sessão.
* **Limpeza automática**: transcrições são limpas com base na configuração `cleanupPeriodDays`, que tem como padrão 30 dias.

<h2 id="tool-restrictions-2">
  Restrições de ferramentas
</h2>

Subagentes podem ter acesso restrito a ferramentas via o campo `tools`:

* **Omita o campo**: agente herda todas as ferramentas disponíveis (padrão)
* **Especifique ferramentas**: agente pode usar apenas ferramentas listadas

Este exemplo cria um agente de análise somente leitura que pode examinar código mas não pode modificar arquivos ou executar comandos.

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

| Caso de uso             | Ferramentas                             | Descrição                                                   |
| :---------------------- | :-------------------------------------- | :---------------------------------------------------------- |
| Análise somente leitura | `Read`, `Grep`, `Glob`                  | Pode examinar código mas não modificar ou executar          |
| Execução de testes      | `Bash`, `Read`, `Grep`                  | Pode executar comandos e analisar saída                     |
| Modificação de código   | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Acesso completo de leitura/escrita sem execução de comandos |
| Acesso completo         | Todas as ferramentas                    | Herda todas as ferramentas do pai (omita o campo `tools`)   |

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

* **Verifique se as invocações de Agent são aprovadas**: inclua `Agent` em `allowedTools` para aprovar automaticamente chamadas de subagentes. Sem isso, as invocações de Agent caem no seu callback `canUseTool` ou, no modo `dontAsk`, são negadas
* **Use prompting explícito**: mencione o subagente pelo nome em seu prompt, por exemplo "Use o agente code-reviewer para..."
* **Escreva uma descrição clara**: explique exatamente quando usar o subagente para que Claude possa corresponder tarefas apropriadamente

<h3 id="filesystem-based-agents-not-loading">
  Agentes baseados em sistema de arquivos não carregando
</h3>

Claude Code monitora `~/.claude/agents/` e `.claude/agents/` e detecta um arquivo de agente novo ou editado em alguns segundos, sem necessidade de reinicialização. Se uma definição nunca aparecer, trabalhe através dessas causas:

* **Novo diretório `agents`**: o monitor cobre apenas diretórios que existiam quando a sessão começou, então o primeiro arquivo em um novo diretório precisa de uma reinicialização de sessão. Esta é a causa mais comum.
* **Frontmatter inválido ou um `name` duplicado**: verifique o YAML do arquivo e se um agente existente já usa o `name`.
* **`--disable-slash-commands`**: sessões iniciadas com essa flag não monitoram esses diretórios e sempre precisam de uma reinicialização para carregar novos arquivos.
* **Um agente programático com o mesmo nome**: `agents` passados para `query()` substituem um agente do sistema de arquivos com o mesmo nome.

Para o formato do arquivo, veja [como escrever arquivos de subagente](/docs/pt/sub-agents#write-subagent-files).

<h3 id="long-prompt-failures-on-windows">
  Falhas de prompt longo no Windows
</h3>

No Windows, subagentes com prompts muito longos podem falhar devido ao limite de comprimento de linha de comando de 8191 caracteres. Mantenha prompts concisos ou use agentes baseados em sistema de arquivos para instruções complexas.

<h2 id="related-documentation">
  Documentação relacionada
</h2>

* [Subagentes Claude Code](/docs/pt/sub-agents): documentação abrangente de subagentes incluindo definições baseadas em sistema de arquivos
* [Fluxos de trabalho dinâmicos](/docs/pt/workflows): orquestre muitos subagentes a partir de um script para trabalhos muito grandes para uma conversa
* [Visão geral do SDK](/docs/pt/agent-sdk/overview): começando com o Claude Agent SDK
