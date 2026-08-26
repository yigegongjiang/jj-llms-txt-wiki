> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Visão geral do Agent SDK

> Construa agentes de IA em produção com Claude Code como uma biblioteca

Construa agentes de IA que leem arquivos autonomamente, executam comandos, pesquisam na web, editam código e muito mais. O Agent SDK oferece as mesmas ferramentas, loop de agente e gerenciamento de contexto que alimentam Claude Code, programável em Python e TypeScript. Para entender o raciocínio por trás do design do harness de agente, consulte [A harness for every task: dynamic workflows in Claude Code](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code) no blog.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Find and fix the bug in auth.py",
          options=ClaudeAgentOptions(allowed_tools=["Read", "Edit", "Bash"]),
      ):
          print(message)  # Claude reads the file, finds the bug, edits it


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Find and fix the bug in auth.ts",
    options: { allowedTools: ["Read", "Edit", "Bash"] }
  })) {
    console.log(message); // Claude reads the file, finds the bug, edits it
  }
  ```
</CodeGroup>

O Agent SDK inclui ferramentas integradas para ler arquivos, executar comandos e editar código, para que seu agente possa começar a trabalhar imediatamente sem você implementar a execução de ferramentas. Mergulhe no guia de início rápido ou explore agentes reais construídos com o SDK:

<CardGroup cols={2}>
  <Card title="Guia de Início Rápido" icon="play" href="/docs/pt/agent-sdk/quickstart">
    Construa um agente de correção de bugs em minutos
  </Card>

  <Card title="Agentes de exemplo" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Assistente de email, agente de pesquisa e muito mais
  </Card>
</CardGroup>

<h2 id="get-started">
  Comece agora
</h2>

<Steps>
  <Step title="Instale o SDK">
    <Tabs>
      <Tab title="TypeScript">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) é um gerenciador de pacotes Python rápido que lida com ambientes virtuais automaticamente:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Crie e ative um ambiente virtual, depois instale o pacote. Instalar em um ambiente virtual evita a falha `error: externally-managed-environment` que o Python do sistema em instalações recentes do Debian, Ubuntu e Homebrew retorna para `pip install` fora de um venv.

        No macOS ou Linux:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        No Windows:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Se o PowerShell bloquear `Activate.ps1` com um erro de política de execução, execute `Set-ExecutionPolicy -Scope Process RemoteSigned` primeiro.

        O pacote Python requer Python 3.10 ou posterior. Se o pip relatar `No matching distribution found for claude-agent-sdk`, seu interpretador é mais antigo que 3.10. Execute `python3 --version` no macOS ou Linux, ou `py --version` no Windows, para verificar.
      </Tab>
    </Tabs>

    <Note>
      O SDK TypeScript agrupa um binário nativo do Claude Code para sua plataforma como uma dependência opcional, portanto você não precisa instalar Claude Code separadamente.
    </Note>
  </Step>

  <Step title="Defina sua chave de API">
    Obtenha uma chave de API do [Console](https://platform.claude.com/), depois defina-a como uma variável de ambiente.

    No macOS ou Linux:

    ```bash theme={null}
    export ANTHROPIC_API_KEY=sk-ant-xxxxx
    ```

    No Windows PowerShell:

    ```powershell theme={null}
    $env:ANTHROPIC_API_KEY = "sk-ant-xxxxx"
    ```

    O SDK também suporta autenticação via provedores de API de terceiros:

    * **Amazon Bedrock**: defina a variável de ambiente `CLAUDE_CODE_USE_BEDROCK=1` e configure as credenciais da AWS
    * **Claude Platform on AWS**: defina `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` e `ANTHROPIC_AWS_WORKSPACE_ID`, depois configure as credenciais da AWS
    * **Google Cloud's Agent Platform**: defina a variável de ambiente `CLAUDE_CODE_USE_VERTEX=1` e configure as credenciais do Google Cloud
    * **Microsoft Azure**: defina a variável de ambiente `CLAUDE_CODE_USE_FOUNDRY=1` e configure as credenciais do Azure

    Consulte os guias de configuração para [Amazon Bedrock](/docs/pt/amazon-bedrock), [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) ou [Microsoft Foundry](/docs/pt/microsoft-foundry) para obter detalhes.

    <Note>
      A menos que previamente aprovado, a Anthropic não permite que desenvolvedores terceirizados ofereçam login claude.ai ou limites de taxa para seus produtos, incluindo agentes construídos no Claude Agent SDK. Use os métodos de autenticação de chave de API descritos neste documento.
    </Note>
  </Step>

  <Step title="Execute seu primeiro agente">
    Este exemplo cria um agente que lista arquivos em seu diretório atual usando ferramentas integradas.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="What files are in this directory?",
              options=ClaudeAgentOptions(allowed_tools=["Bash", "Glob"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "What files are in this directory?",
        options: { allowedTools: ["Bash", "Glob"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Step>
</Steps>

**Pronto para construir?** Siga o [Guia de Início Rápido](/docs/pt/agent-sdk/quickstart) para criar um agente que encontra e corrige bugs em minutos.

<h2 id="capabilities">
  Capacidades
</h2>

Tudo o que torna Claude Code poderoso está disponível no SDK:

<Tabs>
  <Tab title="Ferramentas integradas">
    Seu agente pode ler arquivos, executar comandos e pesquisar bases de código imediatamente. As ferramentas principais incluem:

    | Ferramenta                                                                  | O que faz                                                                         |
    | --------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
    | **Read**                                                                    | Ler qualquer arquivo no diretório de trabalho                                     |
    | **Write**                                                                   | Criar novos arquivos                                                              |
    | **Edit**                                                                    | Fazer edições precisas em arquivos existentes                                     |
    | **Bash**                                                                    | Executar comandos de terminal, scripts, operações git                             |
    | **Monitor**                                                                 | Observar um script em segundo plano e reagir a cada linha de saída como um evento |
    | **Glob**                                                                    | Encontrar arquivos por padrão (`**/*.ts`, `src/**/*.py`)                          |
    | **Grep**                                                                    | Pesquisar conteúdo de arquivos com regex                                          |
    | **WebSearch**                                                               | Pesquisar na web por informações atuais                                           |
    | **WebFetch**                                                                | Buscar e analisar conteúdo de páginas da web                                      |
    | **[AskUserQuestion](/docs/pt/agent-sdk/user-input#handle-clarifying-questions)** | Fazer perguntas de esclarecimento ao usuário com opções de múltipla escolha       |

    Este exemplo cria um agente que pesquisa sua base de código por comentários TODO:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Find all TODO comments and create a summary",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob", "Grep"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Find all TODO comments and create a summary",
        options: { allowedTools: ["Read", "Glob", "Grep"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="hooks">
    Execute código personalizado em pontos-chave do ciclo de vida do agente. Os hooks do SDK usam funções de retorno de chamada para validar, registrar, bloquear ou transformar o comportamento do agente.

    **Hooks disponíveis:** `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit` e muito mais.

    Este exemplo registra todas as alterações de arquivo em um arquivo de auditoria:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from datetime import datetime
      from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher


      async def log_file_change(input_data, tool_use_id, context):
          file_path = input_data.get("tool_input", {}).get("file_path", "unknown")
          with open("./audit.log", "a") as f:
              f.write(f"{datetime.now()}: modified {file_path}\n")
          return {}


      async def main():
          async for message in query(
              prompt="Refactor utils.py to improve readability",
              options=ClaudeAgentOptions(
                  permission_mode="acceptEdits",
                  hooks={
                      "PostToolUse": [
                          HookMatcher(matcher="Edit|Write", hooks=[log_file_change])
                      ]
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query, HookCallback } from "@anthropic-ai/claude-agent-sdk";
      import { appendFile } from "fs/promises";

      const logFileChange: HookCallback = async (input) => {
        const filePath = (input as any).tool_input?.file_path ?? "unknown";
        await appendFile("./audit.log", `${new Date().toISOString()}: modified ${filePath}\n`);
        return {};
      };

      for await (const message of query({
        prompt: "Refactor utils.py to improve readability",
        options: {
          permissionMode: "acceptEdits",
          hooks: {
            PostToolUse: [{ matcher: "Edit|Write", hooks: [logFileChange] }]
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Saiba mais sobre hooks →](/docs/pt/agent-sdk/hooks)
  </Tab>

  <Tab title="Subagentes">
    Crie agentes especializados para lidar com subtarefas focadas. Seu agente principal delega trabalho e os subagentes relatam resultados.

    Defina agentes personalizados com instruções especializadas. Os subagentes são invocados via a ferramenta Agent, então inclua `Agent` em `allowedTools` para aprovar automaticamente essas invocações:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


      async def main():
          async for message in query(
              prompt="Use the code-reviewer agent to review this codebase",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep", "Agent"],
                  agents={
                      "code-reviewer": AgentDefinition(
                          description="Expert code reviewer for quality and security reviews.",
                          prompt="Analyze code quality and suggest improvements.",
                          tools=["Read", "Glob", "Grep"],
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
        prompt: "Use the code-reviewer agent to review this codebase",
        options: {
          allowedTools: ["Read", "Glob", "Grep", "Agent"],
          agents: {
            "code-reviewer": {
              description: "Expert code reviewer for quality and security reviews.",
              prompt: "Analyze code quality and suggest improvements.",
              tools: ["Read", "Glob", "Grep"]
            }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    As mensagens dentro do contexto de um subagente incluem um campo `parent_tool_use_id`, permitindo que você rastreie quais mensagens pertencem a qual execução de subagente.

    [Saiba mais sobre subagentes →](/docs/pt/agent-sdk/subagents)
  </Tab>

  <Tab title="MCP">
    Conecte-se a sistemas externos via Model Context Protocol: bancos de dados, navegadores, APIs e [centenas mais](https://github.com/modelcontextprotocol/servers).

    Este exemplo conecta o [servidor Playwright MCP](https://github.com/microsoft/playwright-mcp) para dar ao seu agente capacidades de automação de navegador:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Open example.com and describe what you see",
              options=ClaudeAgentOptions(
                  mcp_servers={
                      "playwright": {"command": "npx", "args": ["@playwright/mcp@latest"]}
                  }
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Open example.com and describe what you see",
        options: {
          mcpServers: {
            playwright: { command: "npx", args: ["@playwright/mcp@latest"] }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Saiba mais sobre MCP →](/docs/pt/agent-sdk/mcp)
  </Tab>

  <Tab title="Permissões">
    Controle exatamente quais ferramentas seu agente pode usar. Permita operações seguras, bloqueie operações perigosas ou exija aprovação para ações sensíveis.

    <Note>
      Para prompts de aprovação interativa e a ferramenta `AskUserQuestion`, consulte [Lidar com aprovações e entrada do usuário](/docs/pt/agent-sdk/user-input).
    </Note>

    Este exemplo cria um agente somente leitura que pode analisar mas não modificar código. `allowed_tools` pré-aprova `Read`, `Glob` e `Grep`.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Review this code for best practices",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep"],
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Review this code for best practices",
        options: {
          allowedTools: ["Read", "Glob", "Grep"]
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Saiba mais sobre permissões →](/docs/pt/agent-sdk/permissions)
  </Tab>

  <Tab title="Sessões">
    Mantenha contexto em múltiplas trocas. Claude se lembra de arquivos lidos, análises feitas e histórico de conversa. Retome sessões depois ou divida-as para explorar diferentes abordagens.

    Este exemplo captura o ID da sessão da primeira consulta, depois retoma para continuar com contexto completo:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


      async def main():
          session_id = None

          # First query: capture the session ID
          async for message in query(
              prompt="Read the authentication module",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob"]),
          ):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  session_id = message.data["session_id"]

          # Resume with full context from the first query
          async for message in query(
              prompt="Now find all places that call it",  # "it" = auth module
              options=ClaudeAgentOptions(resume=session_id),
          ):
              if isinstance(message, ResultMessage):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      let sessionId: string | undefined;

      // First query: capture the session ID
      for await (const message of query({
        prompt: "Read the authentication module",
        options: { allowedTools: ["Read", "Glob"] }
      })) {
        if (message.type === "system" && message.subtype === "init") {
          sessionId = message.session_id;
        }
      }

      // Resume with full context from the first query
      for await (const message of query({
        prompt: "Now find all places that call it", // "it" = auth module
        options: { resume: sessionId }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Saiba mais sobre sessões →](/docs/pt/agent-sdk/sessions)
  </Tab>
</Tabs>

<h3 id="claude-code-features">
  Recursos do Claude Code
</h3>

O SDK também suporta a configuração baseada em sistema de arquivos do Claude Code. Com opções padrão, o SDK carrega estas do `.claude/` em seu diretório de trabalho e `~/.claude/`. Para restringir quais fontes carregam, defina `setting_sources` (Python) ou `settingSources` (TypeScript) em suas opções.

| Recurso                                          | Descrição                                                                                | Localização                        |
| ------------------------------------------------ | ---------------------------------------------------------------------------------------- | ---------------------------------- |
| [Skills](/docs/pt/agent-sdk/skills)                   | Capacidades especializadas que Claude usa automaticamente ou você invoca com `/name`     | `.claude/skills/*/SKILL.md`        |
| [Commands](/docs/pt/agent-sdk/slash-commands)         | Comandos personalizados no formato legado. Use skills para novos comandos personalizados | `.claude/commands/*.md`            |
| [Memory](/docs/pt/agent-sdk/modifying-system-prompts) | Contexto do projeto e instruções                                                         | `CLAUDE.md` ou `.claude/CLAUDE.md` |
| [Plugins](/docs/pt/agent-sdk/plugins)                 | Estenda com skills, agentes, hooks e servidores MCP                                      | Programático via opção `plugins`   |

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Compare o Agent SDK com outras ferramentas Claude
</h2>

A Plataforma Claude oferece múltiplas maneiras de construir com Claude. Aqui está como o Agent SDK se encaixa:

<Tabs>
  <Tab title="Agent SDK vs Client SDK">
    O [Anthropic Client SDK](https://platform.claude.com/docs/pt/api/client-sdks) oferece acesso direto à API: você envia prompts e implementa a execução de ferramentas você mesmo. O **Agent SDK** oferece Claude com execução de ferramentas integrada.

    Com o Client SDK, você implementa um loop de ferramentas. Com o Agent SDK, Claude o manipula:

    <CodeGroup>
      ```python Python theme={null}
      # Client SDK: You implement the tool loop
      response = client.messages.create(...)
      while response.stop_reason == "tool_use":
          result = your_tool_executor(response.tool_use)
          response = client.messages.create(tool_result=result, **params)

      # Agent SDK: Claude handles tools autonomously
      async for message in query(prompt="Fix the bug in auth.py"):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      // Client SDK: You implement the tool loop
      let response = await client.messages.create({ ...params });
      while (response.stop_reason === "tool_use") {
        const result = yourToolExecutor(response.tool_use);
        response = await client.messages.create({ tool_result: result, ...params });
      }

      // Agent SDK: Claude handles tools autonomously
      for await (const message of query({ prompt: "Fix the bug in auth.ts" })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Agent SDK vs Claude Code CLI">
    Mesmas capacidades, interface diferente:

    | Caso de uso                | Melhor escolha |
    | -------------------------- | -------------- |
    | Desenvolvimento interativo | CLI            |
    | Pipelines CI/CD            | SDK            |
    | Aplicações personalizadas  | SDK            |
    | Tarefas únicas             | CLI            |
    | Automação em produção      | SDK            |

    Muitas equipes usam ambas: CLI para desenvolvimento diário, SDK para produção. Os fluxos de trabalho se traduzem diretamente entre eles.
  </Tab>

  <Tab title="Agent SDK vs Managed Agents">
    [Managed Agents](https://platform.claude.com/docs/pt/managed-agents/overview) é uma API REST hospedada: a Anthropic executa o agente e a sandbox, e sua aplicação envia eventos e transmite resultados de volta. O **Agent SDK** é uma biblioteca que executa o loop do agente dentro de seu próprio processo.

    |                                | Agent SDK                                                                                   | Managed Agents                                                                                             |
    | ------------------------------ | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
    | **Executa em**                 | Seu processo, sua infraestrutura                                                            | Infraestrutura gerenciada pela Anthropic                                                                   |
    | **Interface**                  | Biblioteca Python ou TypeScript                                                             | API REST                                                                                                   |
    | **O agente trabalha em**       | Arquivos em sua infraestrutura                                                              | Uma sandbox gerenciada por sessão                                                                          |
    | **Estado da sessão**           | JSONL em seu sistema de arquivos                                                            | Log de eventos hospedado pela Anthropic                                                                    |
    | **Ferramentas personalizadas** | Funções Python ou TypeScript em processo                                                    | Claude dispara a ferramenta; você executa e retorna resultados                                             |
    | **Melhor para**                | Prototipagem local, agentes que trabalham diretamente em seu sistema de arquivos e serviços | Agentes de produção sem operar infraestrutura de sandbox ou sessão, sessões de longa duração e assíncronas |

    Um caminho comum é fazer prototipagem com o Agent SDK localmente e depois migrar para Managed Agents para produção.
  </Tab>
</Tabs>

<h2 id="changelog">
  Changelog
</h2>

Veja o changelog completo para atualizações do SDK, correções de bugs e novos recursos:

* **TypeScript SDK**: [ver CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [ver CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="reporting-bugs">
  Relatando bugs
</h2>

Se você encontrar bugs ou problemas com o Agent SDK:

* **TypeScript SDK**: [relatar problemas no GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [relatar problemas no GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Diretrizes de marca
</h2>

Para parceiros integrando o Claude Agent SDK, o uso de marca Claude é opcional. Ao fazer referência a Claude em seu produto:

**Permitido:**

* "Claude Agent" (preferido para menus suspensos)
* "Claude" (quando dentro de um menu já rotulado "Agents")
* "{YourAgentName} Powered by Claude" (se você tiver um nome de agente existente)

**Não permitido:**

* "Claude Code" ou "Claude Code Agent"
* Arte ASCII com marca Claude Code ou elementos visuais que imitam Claude Code

Seu produto deve manter sua própria marca e não parecer ser Claude Code ou qualquer produto Anthropic. Para perguntas sobre conformidade de marca, entre em contato com a [equipe de vendas](https://www.anthropic.com/contact-sales) da Anthropic.

<h2 id="license-and-terms">
  Licença e termos
</h2>

O uso do Claude Agent SDK é regido pelos [Termos de Serviço Comercial da Anthropic](https://www.anthropic.com/legal/commercial-terms), incluindo quando você o usa para alimentar produtos e serviços que você disponibiliza para seus próprios clientes e usuários finais, exceto na medida em que um componente específico ou dependência seja coberto por uma licença diferente conforme indicado no arquivo LICENSE desse componente.

<h2 id="next-steps">
  Próximos passos
</h2>

<CardGroup cols={2}>
  <Card title="Guia de Início Rápido" icon="play" href="/docs/pt/agent-sdk/quickstart">
    Construa um agente que encontra e corrige bugs em minutos
  </Card>

  <Card title="Agentes de exemplo" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Assistente de email, agente de pesquisa e muito mais
  </Card>

  <Card title="TypeScript SDK" icon="code" href="/docs/pt/agent-sdk/typescript">
    Referência completa da API TypeScript e exemplos
  </Card>

  <Card title="Python SDK" icon="code" href="/docs/pt/agent-sdk/python">
    Referência completa da API Python e exemplos
  </Card>
</CardGroup>
