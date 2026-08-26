> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Use Claude Code features in the SDK

> Load project instructions, skills, hooks, and other Claude Code features into your SDK agents.

O Agent SDK é construído na mesma base que Claude Code, o que significa que seus agentes SDK têm acesso aos mesmos recursos baseados em sistema de arquivos: instruções de projeto (`CLAUDE.md` e regras), skills, hooks e muito mais.

Quando você omite `settingSources`, `query()` lê as mesmas configurações do sistema de arquivos que a CLI Claude Code: configurações de usuário, projeto e local, arquivos `CLAUDE.md` e skills, agentes e comandos em `.claude/`. Para executar sem estes, passe `settingSources: []`, o que limita o agente ao que você configura programaticamente. As configurações de política gerenciada e a configuração global `~/.claude.json` são lidas independentemente desta opção. Veja [O que settingSources não controla](#what-settingsources-does-not-control).

Para uma visão geral conceitual do que cada recurso faz e quando usá-lo, veja [Extend Claude Code](/docs/pt/features-overview).

<h2 id="control-filesystem-settings-with-settingsources">
  Controlar configurações do sistema de arquivos com settingSources
</h2>

A opção de fontes de configuração ([`setting_sources`](/docs/pt/agent-sdk/python#claudeagentoptions) em Python, [`settingSources`](/docs/pt/agent-sdk/typescript#settingsource) em TypeScript) controla quais configurações baseadas em sistema de arquivos o SDK carrega. Passe uma lista explícita para optar por fontes específicas, ou passe um array vazio para desabilitar configurações de usuário, projeto e local.

Este exemplo carrega configurações de nível de usuário e nível de projeto definindo `settingSources` para `["user", "project"]`:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage

  async for message in query(
      prompt="Help me refactor the auth module",
      options=ClaudeAgentOptions(
          # "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
          # Together they give the agent access to CLAUDE.md, skills, hooks, and
          # permissions from both locations.
          setting_sources=["user", "project"],
          allowed_tools=["Read", "Edit", "Bash"],
      ),
  ):
      if isinstance(message, AssistantMessage):
          for block in message.content:
              if hasattr(block, "text"):
                  print(block.text)
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(f"\nResult: {message.result}")
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me refactor the auth module",
    options: {
      // "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
      // Together they give the agent access to CLAUDE.md, skills, hooks, and
      // permissions from both locations.
      settingSources: ["user", "project"],
      allowedTools: ["Read", "Edit", "Bash"]
    }
  })) {
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "text") console.log(block.text);
      }
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(`\nResult: ${message.result}`);
    }
  }
  ```
</CodeGroup>

Cada fonte carrega configurações de um local específico, onde `<cwd>` é o diretório de trabalho que você passa via opção `cwd`, ou o diretório atual do processo se não definido. Para a definição de tipo completa, veja [`SettingSource`](/docs/pt/agent-sdk/typescript#settingsource) (TypeScript) ou [`SettingSource`](/docs/pt/agent-sdk/python#settingsource) (Python).

| Fonte       | O que carrega                                                                                               | Local                                                                                                                                                                  |
| :---------- | :---------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"project"` | CLAUDE.md do projeto, `.claude/rules/*.md`, skills do projeto, hooks do projeto, `settings.json` do projeto | `<cwd>/.claude/` para `settings.json` e hooks; `<cwd>` e cada diretório pai para CLAUDE.md e rules; `<cwd>` e cada diretório pai até a raiz do repositório para skills |
| `"user"`    | CLAUDE.md do usuário, `~/.claude/rules/*.md`, skills do usuário, configurações do usuário                   | `~/.claude/`                                                                                                                                                           |
| `"local"`   | CLAUDE.local.md, `.claude/settings.local.json`                                                              | `<cwd>/.claude/` para `settings.local.json`; `<cwd>` e cada diretório pai para CLAUDE.local.md                                                                         |

Omitir `settingSources` é equivalente a `["user", "project", "local"]`.

A opção `cwd` determina onde o SDK procura por entradas de nível de projeto. CLAUDE.md e rules carregam de `<cwd>` e de cada diretório pai. Skills carregam de `<cwd>` e de cada diretório pai até a raiz do repositório. `settings.json` do projeto e hooks carregam apenas de `<cwd>/.claude/` sem fallback de diretório pai.

<h3 id="what-settingsources-does-not-control">
  O que settingSources não controla
</h3>

`settingSources` cobre configurações de usuário, projeto e local. Algumas entradas são lidas independentemente de seu valor:

| Entrada                                                               | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                       | Para desabilitar                                                                                                                                                                                                                           |
| :-------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Configurações de política gerenciada                                  | Política gerenciada pelo endpoint, seja plist MDM, política de registro ou arquivos de configurações gerenciadas, carrega do host; [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) são buscadas em uma [configuração elegível](/docs/pt/server-managed-settings#platform-availability) quando a sessão se autentica com um login OAuth de organização ou uma chave de API configurada diretamente | Política de endpoint: remova o arquivo de configurações gerenciadas, plist ou política de registro do host. Configurações gerenciadas pelo servidor: controladas pelo administrador da sua organização; não podem ser desabilitadas do SDK |
| Configuração global `~/.claude.json`                                  | Sempre lida                                                                                                                                                                                                                                                                                                                                                                                                         | Relocalize com `CLAUDE_CONFIG_DIR` em `env`                                                                                                                                                                                                |
| Memória automática em `~/.claude/projects/<project>/memory/`          | Carregada no prompt do sistema no início da sessão. O agente escreve novas memórias lá com as ferramentas padrão `Write` e `Edit` em vez de uma ferramenta de memória dedicada, portanto essas ferramentas devem estar habilitadas para o agente salvar memórias                                                                                                                                                    | Defina `autoMemoryEnabled: false` nas configurações, ou `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` em `env`                                                                                                                                       |
| [Conectores MCP do claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) | Carregados quando o método de autenticação ativo é uma assinatura do claude.ai. Passar `mcpServers: {}` não os suprime                                                                                                                                                                                                                                                                                              | Defina `strictMcpConfig: true`, [`disableClaudeAiConnectors: true`](/docs/pt/mcp#disable-claude-ai-connectors) nas configurações, ou `ENABLE_CLAUDEAI_MCP_SERVERS=false` em `env`                                                               |

<Warning>
  Não confie nas opções padrão de `query()` para isolamento multi-tenant. Porque as entradas acima são lidas independentemente de `settingSources`, um processo SDK pode pegar configuração de nível de host e memória por diretório. Para implantações multi-tenant, execute cada tenant em seu próprio sistema de arquivos e defina `settingSources: []` mais `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` em `env`. [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) são buscadas quando o processo se autentica com uma credencial de organização; isolamento do sistema de arquivos não as remove. Veja [Implantação segura](/docs/pt/agent-sdk/secure-deployment).
</Warning>

<h2 id="project-instructions-claude-md-and-rules">
  Instruções do projeto (CLAUDE.md e regras)
</h2>

Arquivos `CLAUDE.md` e arquivos `.claude/rules/*.md` dão ao seu agente contexto persistente sobre seu projeto: convenções de codificação, comandos de compilação, decisões de arquitetura e instruções. Quando `settingSources` inclui `"project"` (como no exemplo acima), o SDK carrega esses arquivos em contexto no início da sessão. O agente então segue suas convenções de projeto sem você repeti-las em cada prompt.

<h3 id="claude-md-load-locations">
  CLAUDE.md load locations
</h3>

| Nível                      | Local                                                                   | Quando carregado                                                                                         |
| :------------------------- | :---------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| Projeto (raiz)             | `<cwd>/CLAUDE.md` ou `<cwd>/.claude/CLAUDE.md`                          | `settingSources` inclui `"project"`                                                                      |
| Regras do projeto          | `<cwd>/.claude/rules/*.md` e `.claude/rules/*.md` em cada diretório pai | `settingSources` inclui `"project"`                                                                      |
| Projeto (diretórios pai)   | Arquivos `CLAUDE.md` em diretórios acima de `cwd`                       | `settingSources` inclui `"project"`, carregado no início da sessão                                       |
| Projeto (diretórios filho) | Arquivos `CLAUDE.md` em subdiretórios de `cwd`                          | `settingSources` inclui `"project"`, carregado sob demanda quando o agente lê um arquivo nessa subárvore |
| Local                      | `<cwd>/CLAUDE.local.md` e `CLAUDE.local.md` em cada diretório pai       | `settingSources` inclui `"local"`                                                                        |
| Usuário                    | `~/.claude/CLAUDE.md`                                                   | `settingSources` inclui `"user"`                                                                         |
| Regras do usuário          | `~/.claude/rules/*.md`                                                  | `settingSources` inclui `"user"`                                                                         |

Todos os níveis são aditivos: se existem arquivos `CLAUDE.md` de projeto e usuário, o agente vê ambos. Não há regra de precedência rígida entre níveis; se as instruções conflitarem, o resultado depende de como Claude as interpreta. Escreva regras não conflitantes, ou declare precedência explicitamente no arquivo mais específico ("Estas instruções de projeto substituem quaisquer padrões conflitantes de nível de usuário").

<Tip>
  Você também pode injetar contexto diretamente via `systemPrompt` sem usar arquivos `CLAUDE.md`. Veja [Modify system prompts](/docs/pt/agent-sdk/modifying-system-prompts). Use `CLAUDE.md` quando você quer que o mesmo contexto seja compartilhado entre sessões interativas de Claude Code e seus agentes SDK.
</Tip>

Para como estruturar e organizar conteúdo `CLAUDE.md`, veja [Manage Claude's memory](/docs/pt/memory).

<h2 id="skills">
  Skills
</h2>

Skills são arquivos markdown que dão ao seu agente conhecimento especializado e fluxos de trabalho invocáveis. Diferentemente de `CLAUDE.md` (que carrega a cada sessão), skills carregam sob demanda. O agente recebe descrições de skills na inicialização e carrega o conteúdo completo quando relevante.

Skills são descobertos do sistema de arquivos através de `settingSources`. Quando a opção `skills` em `query()` é omitida, skills de usuário e projeto descobertos são habilitados e a ferramenta Skill fica disponível, correspondendo ao comportamento da CLI. Para controlar quais skills são habilitados, passe `skills` como `"all"`, uma lista de nomes de skills, ou `[]` para desabilitar todos. Quando `skills` é definido, o SDK adiciona a ferramenta Skill a `allowedTools` automaticamente. Se você também passar uma lista explícita de `tools`, inclua `"Skill"` nessa lista para que Claude possa invocar skills.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Skills in .claude/skills/ are discovered automatically
  # when settingSources includes "project"
  async for message in query(
      prompt="Review this PR using our code review checklist",
      options=ClaudeAgentOptions(
          setting_sources=["user", "project"],
          skills="all",
          allowed_tools=["Read", "Grep", "Glob"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Skills in .claude/skills/ are discovered automatically
  // when settingSources includes "project"
  for await (const message of query({
    prompt: "Review this PR using our code review checklist",
    options: {
      settingSources: ["user", "project"],
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<Note>
  Skills devem ser criados como artefatos do sistema de arquivos (`.claude/skills/<name>/SKILL.md`). O SDK não tem uma API programática para registrar skills. Veja [Agent Skills in the SDK](/docs/pt/agent-sdk/skills) para detalhes completos.
</Note>

Para mais sobre criar e usar skills, veja [Agent Skills in the SDK](/docs/pt/agent-sdk/skills).

<h2 id="hooks">
  Hooks
</h2>

O SDK suporta duas maneiras de definir hooks, e eles executam lado a lado:

* **Filesystem hooks:** comandos shell definidos em `settings.json`, carregados quando `settingSources` inclui a fonte relevante. Estes são os mesmos hooks que você configuraria para [sessões interativas de Claude Code](/docs/pt/hooks-guide).
* **Programmatic hooks:** funções de callback passadas diretamente para `query()`. Estes executam em seu processo de aplicação e podem retornar decisões estruturadas. Veja [Control execution with hooks](/docs/pt/agent-sdk/hooks).

Ambos os tipos executam durante o mesmo ciclo de vida de hook. Se você já tem hooks no `settings.json` do seu projeto e você define `settingSources: ["project"]`, esses hooks executam automaticamente no SDK sem configuração extra.

Callbacks de hook recebem a entrada da ferramenta e retornam um dict de decisão. Retornar `{}` significa permitir que a ferramenta prossiga. Para bloquear a execução, retorne um objeto `hookSpecificOutput` com `permissionDecision: "deny"` e um `permissionDecisionReason`. A razão é enviada para Claude como o resultado da ferramenta. Os campos `decision` e `reason` de nível superior estão descontinuados para `PreToolUse`. Veja o [hooks guide](/docs/pt/agent-sdk/hooks) para a assinatura de callback completa e tipos de retorno.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, ResultMessage


  # PreToolUse hook callback. Positional args:
  #   input_data: HookInput dict with tool_name, tool_input, hook_event_name
  #   tool_use_id: str | None, the ID of the tool call being intercepted
  #   context: HookContext, carries session metadata
  async def audit_bash(input_data, tool_use_id, context):
      command = input_data.get("tool_input", {}).get("command", "")
      if "rm -rf" in command:
          return {
              "hookSpecificOutput": {
                  "hookEventName": "PreToolUse",
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Destructive command blocked",
              }
          }
      return {}  # Empty dict: allow the tool to proceed


  # Filesystem hooks from .claude/settings.json run automatically
  # when settingSources loads them. You can also add programmatic hooks:
  async for message in query(
      prompt="Refactor the auth module",
      options=ClaudeAgentOptions(
          setting_sources=["project"],  # Loads hooks from .claude/settings.json
          hooks={
              "PreToolUse": [
                  HookMatcher(matcher="Bash", hooks=[audit_bash]),
              ]
          },
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query, type HookInput, type HookJSONOutput } from "@anthropic-ai/claude-agent-sdk";

  // PreToolUse hook callback. HookInput is a discriminated union on
  // hook_event_name, so narrowing on it gives TypeScript the right
  // tool_input shape for this event.
  const auditBash = async (input: HookInput): Promise<HookJSONOutput> => {
    if (input.hook_event_name !== "PreToolUse") return {};
    const toolInput = input.tool_input as { command?: string };
    if (toolInput.command?.includes("rm -rf")) {
      return {
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: "Destructive command blocked",
        },
      };
    }
    return {}; // Empty object: allow the tool to proceed
  };

  // Filesystem hooks from .claude/settings.json run automatically
  // when settingSources loads them. You can also add programmatic hooks:
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      settingSources: ["project"], // Loads hooks from .claude/settings.json
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [auditBash] }]
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h3 id="when-to-use-which-hook-type">
  Quando usar qual tipo de hook
</h3>

| Tipo de hook                              | Melhor para                                                                                                                                                                                                                                                                                                                                   |
| :---------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Filesystem** (`settings.json`)          | Compartilhar hooks entre sessões CLI e SDK. Suporta `"command"` (scripts shell), `"http"` (POST para um endpoint), `"mcp_tool"` (chamar a ferramenta de um servidor MCP conectado), `"prompt"` (LLM avalia um prompt), e `"agent"` (spawns um agente verificador). Estes disparam no agente principal e em qualquer subagente que ele spawna. |
| **Programmatic** (callbacks em `query()`) | Lógica específica da aplicação, decisões estruturadas e integração em processo. Estes também disparam dentro de subagentes. O callback recebe `agent_id` e `agent_type` para distinguir.                                                                                                                                                      |

<Note>
  O SDK TypeScript suporta eventos de hook adicionais além de Python, incluindo `SessionStart`, `SessionEnd`, `TeammateIdle`, e `TaskCompleted`. Veja o [hooks guide](/docs/pt/agent-sdk/hooks) para a tabela de compatibilidade de eventos completa.
</Note>

Para detalhes completos sobre hooks programáticos, veja [Control execution with hooks](/docs/pt/agent-sdk/hooks). Para sintaxe de hook do sistema de arquivos, veja [Hooks](/docs/pt/hooks).

<h2 id="choose-the-right-feature">
  Escolha o recurso certo
</h2>

O Agent SDK oferece acesso a várias maneiras de estender o comportamento do seu agente. Se você não tem certeza qual usar, esta tabela mapeia objetivos comuns para a abordagem correta.

| Você quer...                                                                                                         | Use                                           | Superfície SDK                                                                                                                                                           |
| :------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Definir convenções de projeto que seu agente sempre segue                                                            | [CLAUDE.md](/docs/pt/memory)                       | `settingSources: ["project"]` carrega automaticamente                                                                                                                    |
| Dar ao agente material de referência que ele carrega quando relevante                                                | [Skills](/docs/pt/agent-sdk/skills)                | opção `settingSources` + `skills`                                                                                                                                        |
| Executar um fluxo de trabalho reutilizável (deploy, review, release)                                                 | [User-invocable skills](/docs/pt/agent-sdk/skills) | opção `settingSources` + `skills`                                                                                                                                        |
| Delegar uma subtarefa isolada para um contexto fresco (research, review)                                             | [Subagents](/docs/pt/agent-sdk/subagents)          | parâmetro `agents` + `allowedTools: ["Agent"]`                                                                                                                           |
| Coordenar múltiplas instâncias de Claude Code com listas de tarefas compartilhadas e mensagens diretas entre agentes | [Agent teams](/docs/pt/agent-teams)                | Não configurado diretamente via opções SDK. Agent teams são um recurso CLI onde uma sessão atua como o líder da equipe, coordenando trabalho entre colegas independentes |
| Executar lógica determinística em chamadas de ferramenta (audit, block, transform)                                   | [Hooks](/docs/pt/agent-sdk/hooks)                  | parâmetro `hooks` com callbacks, ou scripts shell carregados via `settingSources`                                                                                        |
| Dar a Claude acesso estruturado a ferramenta para um serviço externo                                                 | [MCP](/docs/pt/agent-sdk/mcp)                      | parâmetro `mcpServers`                                                                                                                                                   |

<Tip>
  **Subagents versus agent teams:** Subagents são efêmeros e isolados: conversa fresca, uma tarefa, resumo retornado ao pai. Agent teams coordenam múltiplas instâncias independentes de Claude Code que compartilham uma lista de tarefas e se mensageiam diretamente. Agent teams são um recurso CLI. Veja [What subagents inherit](/docs/pt/agent-sdk/subagents#what-subagents-inherit) e a [agent teams comparison](/docs/pt/agent-teams#compare-with-subagents) para detalhes.
</Tip>

Cada recurso que você habilita adiciona à janela de contexto do seu agente. Para custos por recurso e como esses recursos se sobrepõem, veja [Extend Claude Code](/docs/pt/features-overview#understand-context-costs).

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Extend Claude Code](/docs/pt/features-overview): Visão geral conceitual de todos os recursos de extensão, com tabelas de comparação e análise de custo de contexto
* [Skills in the SDK](/docs/pt/agent-sdk/skills): Guia completo para usar skills programaticamente
* [Subagents](/docs/pt/agent-sdk/subagents): Defina e invoque subagents para subtarefas isoladas
* [Hooks](/docs/pt/agent-sdk/hooks): Intercepte e controle comportamento do agente em pontos-chave de execução
* [Permissions](/docs/pt/agent-sdk/permissions): Controle acesso a ferramentas com modos, regras e callbacks
* [System prompts](/docs/pt/agent-sdk/modifying-system-prompts): Injete contexto sem arquivos CLAUDE.md
