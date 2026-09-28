> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Estenda agentes com skills

> Controle quais skills Claude pode invocar em sessões do Claude Agent SDK, despache comandos por nome e crie skills que suas sessões descobrem

Agent Skills estendem Claude com capacidades especializadas que Claude invoca quando relevante. Skills são empacotadas como arquivos `SKILL.md` contendo instruções, descrições e recursos de suporte opcionais. Esta página também cobre [comandos em sessões do Agent SDK](#commands-in-agent-sdk-sessions).

Para informações abrangentes sobre skills, incluindo benefícios, arquitetura e diretrizes de autoria, consulte a [visão geral de Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-agent-sdk">
  Como skills funcionam com o Agent SDK
</h2>

Ao usar o Claude Agent SDK, skills são:

* **Definidas como artefatos do sistema de arquivos**: você cria cada skill como um arquivo `SKILL.md` em seu próprio diretório, como `.claude/skills/<name>/SKILL.md`
* **Carregadas do sistema de arquivos**: o SDK carrega skills dos locais do sistema de arquivos governados por `settingSources` (TypeScript) ou `setting_sources` (Python)
* **Descobertas automaticamente**: uma vez que as configurações do sistema de arquivos são carregadas, o SDK descobre metadados de skill na inicialização a partir de diretórios de usuário e projeto, e carrega o conteúdo completo quando Claude invoca a skill
* **Invocadas pelo modelo**: Claude escolhe autonomamente quando usá-las com base no contexto
* **Invocadas pelo usuário**: você despache uma skill diretamente enviando `/<name>` em um prompt. Consulte [Comandos em sessões do Agent SDK](#commands-in-agent-sdk-sessions)
* **Escopo via opção `skills`**: skills descobertas são habilitadas por padrão. Passe uma lista de nomes de skills, `"all"` ou `[]` para controlar quais skills Claude pode invocar

Diferentemente de subagentes, que você pode definir na [opção `agents`](/docs/pt/agent-sdk/subagents#programmatic-definition-recommended), você cria skills como arquivos em disco. O SDK não fornece uma API programática para registrá-las.

<Note>
  Skills são descobertas através das fontes de configuração do sistema de arquivos. Com opções padrão de `query()`, o SDK carrega fontes de usuário e projeto, portanto skills em `~/.claude/skills/`, `<cwd>/.claude/skills/` e `.claude/skills/` em qualquer diretório pai de `<cwd>` até a raiz do repositório estão disponíveis. A fonte de projeto também cobre `<dir>/.claude/skills/` em cada diretório que você passa através de `additionalDirectories` (TypeScript) ou `add_dirs` (Python), porque o SDK passa esses diretórios para Claude Code como [`--add-dir`](/docs/pt/skills#skills-from-additional-directories). Se você definir `settingSources` explicitamente, inclua `'project'` para manter skills de projeto e diretório adicionado e `'user'` para manter suas skills pessoais, ou use a [opção `plugins`](/docs/pt/agent-sdk/plugins) para carregar skills de um caminho específico.
</Note>

<h2 id="use-skills-with-the-agent-sdk">
  Use skills com o Agent SDK
</h2>

Defina a opção `skills` em `query()` para controlar quais skills Claude pode invocar na sessão. Quando omitida, skills descobertas são habilitadas e a ferramenta Skill está disponível, correspondendo ao comportamento da CLI. Passe `"all"` para deixar Claude invocar cada skill descoberta, uma lista de nomes de skills para permitir apenas aquelas, ou `[]` para deixar Claude invocar nenhuma.

Por exemplo, para deixar Claude invocar apenas duas skills nomeadas:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

<h3 id="set-up-skills-in-a-session">
  Configure skills em uma sessão
</h3>

Quando você define `skills`, o SDK adiciona a ferramenta Skill a `allowedTools` automaticamente. Se você também passar uma lista explícita de `tools`, inclua `"Skill"` nessa lista para que Claude possa invocar skills.

Uma vez configurado, Claude descobre automaticamente skills do sistema de arquivos e as invoca quando relevante para a solicitação do usuário.

O exemplo a seguir habilita cada skill descoberta em uma sessão e pré-aprova as ferramentas que skills comumente precisam. O exemplo define `cwd` para o diretório de trabalho atual do processo, portanto execute-o dentro de um projeto que tenha um diretório `.claude/skills/` no diretório atual ou em qualquer pai até a raiz do repositório:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd=os.getcwd(),  # .claude/skills/ here or in a parent directory
          setting_sources=["user", "project"],  # Load skills from filesystem
          skills="all",  # Let Claude invoke every discovered skill
          allowed_tools=["Read", "Write", "Bash"],
      )

      async for message in query(
          prompt="Help me process this PDF document", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me process this PDF document",
    options: {
      cwd: process.cwd(), // .claude/skills/ here or in a parent directory
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all", // Let Claude invoke every discovered skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="confirm-skills-loaded">
  Confirme skills carregadas
</h3>

Perto do início do stream, o SDK produz uma mensagem de sistema com subtipo `init`. Verifique seu array `skills` para confirmar que suas skills foram carregadas antes de Claude começar a trabalhar. O array inclui as skills invocáveis pelo usuário que você definiu com um campo frontmatter `description` ou `when_to_use`, junto com [skills agrupadas incluídas com Claude Code](/docs/pt/skills#bundled-skills).

O array lista apenas skills invocáveis pelo usuário. Uma skill com [`user-invocable: false`](/docs/pt/skills#control-who-invokes-a-skill) em seu frontmatter carrega e permanece disponível para Claude, mas não aparece no array. O array lista as mesmas skills independentemente de estarem ou não em sua lista `skills`.

<h3 id="allow-only-specific-skills">
  Permita apenas skills específicas
</h3>

Para deixar Claude invocar apenas skills específicas, passe seus nomes na lista `skills`. Os nomes correspondem ao campo `name` em `SKILL.md` ou ao nome do diretório da skill. Use `plugin:skill` para skills fornecidas por plugin.

A lista leva apenas nomes de skills exatos. Se uma entrada não puder funcionar como um nome exato, `query()` rejeita a lista antes da sessão começar. Consulte [Erro de nome de skill inválido](#invalid-skill-name-error) para as regras de nome e o erro que cada SDK levanta.

O modelo não vê skills não listadas e a ferramenta Skill as rejeita, enquanto seus arquivos permanecem em disco e permanecem acessíveis através de Read e Bash. Restringir a lista não restringe [despacho por nome](#dispatch-commands-by-name).

Para deixar Claude invocar cada skill descoberta, passe `skills: "all"` em vez de um curinga.

<h2 id="commands-in-agent-sdk-sessions">
  Comandos em sessões do Agent SDK
</h2>

Esta seção é a documentação de comando do SDK. Um comando é qualquer coisa que você executa enviando `/<name>` em um prompt. As entradas na superfície de comando diferem no que as respalda:

* **Comandos integrados**: executam lógica codificada no processo Claude Code que o SDK executa, por exemplo `/compact`
* **Skills agrupadas**: artefatos de prompt incluídos com Claude Code, por exemplo `/code-review`
* **Suas skills**: artefatos de prompt que você cria, cada um um diretório contendo um arquivo `SKILL.md`. O nome de uma skill invocável pelo usuário se une à superfície automaticamente, portanto despachar seu próprio `/security-check` e executar um integrado funcionam da mesma forma
* **Arquivos de comando personalizados**: uma forma de artefato mais antiga com o mesmo comportamento, arquivos Markdown simples em `.claude/commands/` cujos nomes de arquivo se tornam nomes de comando. Skills são seu sucessor recomendado

Por padrão, tanto você quanto Claude podem invocar qualquer skill. Você pode restringir qualquer caminho através do [frontmatter](/docs/pt/skills#control-who-invokes-a-skill) da skill. Para definições de comando e skill, consulte as entradas [Comando](/docs/pt/glossary#command) e [Skill](/docs/pt/glossary#skill) do glossário. Consulte [Comandos em Claude Code](/docs/pt/commands) para cada integrado e [Estenda Claude com skills](/docs/pt/skills) para o guia completo de ambas as formas de artefato.

<h3 id="discover-available-commands">
  Descubra comandos disponíveis
</h3>

Você pode despachar comandos que funcionam sem um terminal interativo através do SDK. A mensagem `system/init` lista os disponíveis em sua sessão em seu campo `slash_commands`. Comandos que precisam de um terminal interativo, como `/theme` e `/terminal-setup`, não aparecem na lista. Acesse o campo quando sua sessão começar:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello Claude",
    options: { maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available commands:", message.slash_commands);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(prompt="Hello Claude", options=ClaudeAgentOptions(max_turns=1)):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available commands:", message.data["slash_commands"])


  asyncio.run(main())
  ```
</CodeGroup>

A lista impressa mistura comandos integrados, skills agrupadas, suas skills invocáveis pelo usuário e arquivos `.claude/commands/`:

```text theme={null}
Available commands: ["clear", "compact", "context", "usage", "code-review", "verify", "security-check", ...]
```

Uma skill com [`user-invocable: false`](/docs/pt/skills#control-who-invokes-a-skill) em seu frontmatter não aparece nesta lista ou no array `skills` de [Confirme skills carregadas](#confirm-skills-loaded). Sessões que configuram [servidores MCP](/docs/pt/agent-sdk/mcp) também podem expor [prompts MCP como comandos](/docs/pt/mcp#use-mcp-prompts-as-commands).

<h3 id="dispatch-commands-by-name">
  Despache comandos por nome
</h3>

Envie um comando incluindo-o em sua string de prompt, da mesma forma que você envia texto regular. O despacho não depende da opção `skills`. Enviar `/<name>` executa uma skill invocável pelo usuário mesmo quando sua lista `skills` a omite. Comandos que atuam no histórico de conversa, como `/compact`, precisam de mensagens anteriores para trabalhar.

Um `/<name>` que não corresponde nem a um comando na sessão nem a um comando integrado do Claude Code não falha a query. Claude Code envia o prompt para Claude como uma mensagem ordinária, com uma nota de que o comando não foi executado, portanto a query gasta um turno de modelo e retorna a resposta de Claude. Antes da v2.1.274, um `/<name>` que não correspondia a nada retornava `Unknown command: /<name>` como o resultado sem um turno de modelo.

Um `/<name>` que corresponde a um comando integrado do Claude Code que não está disponível na sessão, como `/theme`, retorna `/theme isn't available in this environment.` como o resultado sem um turno de modelo.

<Note>
  Um comando pode atingir o limite `maxTurns` / `max_turns` como qualquer outro prompt, terminando a query com um resultado de erro em vez de `success`. Para o contrato de resultado de erro, consulte [Manipule o resultado](/docs/pt/agent-sdk/agent-loop#handle-the-result). Se seu comando pode atingir o limite, envolva o loop em um `try`/`catch` em TypeScript ou `try`/`except` em Python, como mostrado em [Entrada de Mensagem Única](/docs/pt/agent-sdk/streaming-vs-single-mode#single-message-input), ou defina `maxTurns` alto o suficiente para o trabalho ser concluído.
</Note>

<h3 id="compact-history-with-/compact">
  Compacte histórico com `/compact`
</h3>

O comando `/compact` reduz o tamanho do seu histórico de conversa resumindo mensagens mais antigas enquanto preserva contexto importante. A compactação precisa de uma conversa existente com mensagens anteriores suficientes para resumir. Este exemplo tem uma conversa primeiro, depois a compacta e lê a mensagem de sistema `compact_boundary` que relata o resultado:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Compaction needs existing history, so have a conversation first
  try {
    for await (const message of query({
      prompt: "Explain what this project does",
      options: { maxTurns: 2 }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the follow-up query below still runs.
    console.error(`Session ended with an error: ${error}`);
  }

  // Compact the same conversation
  for await (const message of query({
    prompt: "/compact",
    options: { continue: true, maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "compact_boundary") {
      console.log("Compaction completed");
      console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
      console.log("Trigger:", message.compact_metadata.trigger);
      // Example output:
      // Compaction completed
      // Pre-compaction tokens: 1842
      // Trigger: manual
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage, SystemMessage


  async def main():
      # Compaction needs existing history, so have a conversation first
      try:
          async for message in query(
              prompt="Explain what this project does",
              options=ClaudeAgentOptions(max_turns=2),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the follow-up query below still runs.
          print(f"Session ended with an error: {error}")

      # Compact the same conversation
      async for message in query(
          prompt="/compact",
          options=ClaudeAgentOptions(continue_conversation=True, max_turns=1),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "compact_boundary":
              print("Compaction completed")
              print("Pre-compaction tokens:", message.data["compact_metadata"]["pre_tokens"])
              print("Trigger:", message.data["compact_metadata"]["trigger"])
              # Example output:
              # Compaction completed
              # Pre-compaction tokens: 1842
              # Trigger: manual


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  Uma mensagem `compact_boundary` só chega quando a compactação foi executada. Sem nada para resumir, `/compact` relata o motivo em vez de levantar. A execução ainda termina com um resultado `success` e nenhuma mensagem `compact_boundary`, e o texto do resultado carrega o motivo, por exemplo `Not enough messages to compact.` após uma única troca curta. Uma chamada `query()` nova e única começa com contexto vazio, portanto use este padrão em uma sessão com turnos anteriores, por exemplo em [modo de entrada de streaming](/docs/pt/agent-sdk/streaming-vs-single-mode) ou ao retomar uma sessão.
</Note>

<h3 id="reset-context-with-/clear">
  Redefina contexto com `/clear`
</h3>

O comando `/clear` redefine a conversa para um contexto vazio, portanto prompts subsequentes começam sem histórico de conversa anterior. A conversa anterior permanece em disco. Você pode retornar a essa conversa passando seu ID de sessão para a [opção `resume`](/docs/pt/agent-sdk/sessions#resume-by-id).

`/clear` é útil em [modo de entrada de streaming](/docs/pt/agent-sdk/streaming-vs-single-mode), onde você envia múltiplos prompts sobre uma única conexão. Para chamadas `query()` únicas, cada chamada já começa com contexto vazio, portanto enviar `/clear` não tem efeito prático. Comece uma nova `query()` em vez disso.

<h2 id="create-skills">
  Crie skills
</h2>

Crie cada skill como um diretório contendo um arquivo `SKILL.md` com frontmatter YAML e conteúdo Markdown. O campo `description` determina quando Claude invoca sua skill.

**Exemplo de estrutura de diretório**:

```text theme={null}
.claude/skills/security-check/
└── SKILL.md
```

<h3 id="choose-a-discovery-level">
  Escolha um nível de descoberta
</h3>

Salve skills em um dos dois [níveis de descoberta](/docs/pt/skills#where-skills-live) mais comuns:

* **Skills de projeto**: `.claude/skills/`, disponíveis apenas no projeto atual
* **Skills pessoais**: `~/.claude/skills/`, disponíveis em todos os seus projetos

Se você tem arquivos de comando personalizados existentes em `.claude/commands/`, eles continuam funcionando. Um arquivo de comando em `.claude/commands/deploy.md` cria `/deploy` e funciona da mesma forma que uma skill em `.claude/skills/deploy/SKILL.md` faria. Se um arquivo de comando e uma skill compartilham um nome, consulte [Resolva skills que compartilham um nome](/docs/pt/skills#resolve-skills-that-share-a-name) para qual executa. O SDK carrega arquivos `.claude/commands/` e `~/.claude/commands/` dos mesmos dois escopos que skills. Consulte [Estenda Claude com skills](/docs/pt/skills) para o guia completo de ambas as formas de artefato.

<h3 id="create-and-dispatch-your-first-skill">
  Crie e despache sua primeira skill
</h3>

Para ver o fluxo completo, crie `.claude/skills/security-check/SKILL.md`:

```markdown theme={null}
---
name: security-check
description: Run a security vulnerability scan
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

Uma vez que o arquivo existe, a skill está disponível através do SDK. Claude a invoca quando uma solicitação corresponde à sua descrição, e você pode despachá-la diretamente:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "/security-check",
    options: { maxTurns: 10 }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      async for message in query(
          prompt="/security-check", options=ClaudeAgentOptions(max_turns=10)
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

Uma execução bem-sucedida termina com um resultado `success` cujo texto carrega as descobertas da varredura. Contra um pequeno aplicativo Express com problemas semeados, o texto do resultado começa:

```text theme={null}
**Security scan of `app.js` — 4 findings (most severe first):**

1. **SQL Injection** (line 8) — `req.query.name` is concatenated directly into the SQL string. Trivially exploitable (`' OR '1'='1`, `'; DROP TABLE users;--`). **Fix:** use parameterized queries, e.g. `db.query("SELECT * FROM users WHERE name = ?", [req.query.name], cb)`.
...
```

O nome da skill também aparece no array `slash_commands` da mensagem init.

<Note>
  Claude Code inclui skills agrupadas `code-review` e `verify`. Se você nomear um arquivo `.claude/commands/` após uma delas, por exemplo `.claude/commands/code-review.md`, o arquivo de comando sombreia a skill agrupada e `slash_commands` lista o nome uma vez.
</Note>

<h2 id="pre-approve-tools-for-skills">
  Pré-aprove ferramentas para skills
</h2>

<Note>
  Para skills de projeto e pessoais, Claude Code aplica o campo frontmatter [`allowed-tools`](/docs/pt/skills#pre-approve-tools-for-a-skill) em sessões do SDK. Você também pode pré-aprovar ferramentas para essas skills através da opção `allowedTools` (`allowed_tools` em Python) em sua configuração de query. Skills [sincronizadas de claude.ai](/docs/pt/skills#how-claude-code-handles-the-frontmatter-of-a-synced-skill) seguem suas próprias regras de frontmatter.
</Note>

Skills executam com as ferramentas da sessão. O exemplo abaixo pré-aprova `Read`, `Grep` e `Glob` com `allowedTools` (`allowed_tools` em Python), portanto Claude pode inspecionar arquivos enquanto executa a [skill security-check](#create-and-dispatch-your-first-skill) sem parar para aprovação:

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )


  async def main():
      async for message in query(prompt="Check this project for security issues", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Check this project for security issues",
    options: {
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

No stream, a invocação de skill aparece como um uso de ferramenta Skill, seguido por chamadas Read nos arquivos do projeto. A execução termina com um resultado `success` cujo texto carrega as descobertas.

A lista pré-aprova as ferramentas nomeadas em vez de restringir as outras. Para o fluxo de permissão completo, incluindo modos de permissão e o callback `canUseTool`, consulte [Permissões](/docs/pt/agent-sdk/permissions).

<h2 id="troubleshooting">
  Solução de problemas
</h2>

<h3 id="skills-not-found">
  Skills não encontradas
</h3>

**Verifique a configuração settingSources**: o SDK descobre skills através das fontes de configuração `user` e `project`. Se você definir `settingSources`/`setting_sources` explicitamente e omitir essas fontes, o SDK não carrega skills:

<CodeGroup>
  ```python Python theme={null}
  # Skills not loaded: setting_sources excludes user and project
  options = ClaudeAgentOptions(setting_sources=[], skills="all")

  # Skills loaded: user and project sources included
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Skills not loaded: settingSources excludes user and project
  const optionsWithoutSkills = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const optionsWithSkills = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Para qual diretório de skill cada fonte carrega, consulte a [tabela de fontes do sistema de arquivos](/docs/pt/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources). Para mais detalhes sobre `settingSources`/`setting_sources`, consulte a [referência TypeScript SDK](/docs/pt/agent-sdk/typescript#settingsource) ou [referência Python SDK](/docs/pt/agent-sdk/python#settingsource).

**Verifique o diretório de trabalho**: o SDK carrega skills de `.claude/skills/` na opção `cwd` e em cada diretório pai até a raiz do repositório. Certifique-se de que `cwd` aponta para ou abaixo do diretório contendo `.claude/skills/`, dentro do mesmo repositório:

<CodeGroup>
  ```python Python theme={null}
  # Ensure your cwd points to the directory containing .claude/skills/
  options = ClaudeAgentOptions(
      cwd="/path/to/project",  # .claude/skills/ here or in a parent directory
      setting_sources=["user", "project"],  # Loads skills from these sources
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Ensure your cwd points to the directory containing .claude/skills/
  const options = {
    cwd: "/path/to/project", // .claude/skills/ here or in a parent directory
    settingSources: ["user", "project"], // Loads skills from these sources
    skills: "all"
  };
  ```
</CodeGroup>

Consulte [Use skills com o Agent SDK](#use-skills-with-the-agent-sdk) para o padrão completo.

**Verifique o local do sistema de arquivos**:

```bash theme={null}
# Check project skills
ls .claude/skills/*/SKILL.md

# Check personal skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill não sendo usada
</h3>

**Verifique a opção `skills`**: se você passou uma lista `skills`, confirme que o nome da skill está incluído. Quando Claude tenta invocar uma skill não listada, a ferramenta Skill retorna `Skill <name> is not in this session's skills allowlist`. Adicione o nome à sua lista, ou despache a skill diretamente enviando `/<name>` em um prompt, que funciona sem listar.

**Verifique a descrição**: certifique-se de que é específica e inclui palavras-chave relevantes. Consulte [Práticas recomendadas de Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) para orientação sobre como escrever descrições eficazes.

<h3 id="invalid-skill-name-error">
  Erro de nome de skill inválido
</h3>

Quando um nome em sua lista `skills` não pode funcionar como um nome de skill exato, `query()` rejeita a lista antes de iniciar o processo Claude Code. Os nomes que acionam a rejeição incluem:

* Um nome vazio
* Um nome contendo parênteses, vírgulas ou caracteres de controle
* Um nome preenchido com espaço em branco
* Uma forma curinga como um `*` simples ou um sufixo `:*`

Cada SDK superficializa a rejeição de forma diferente:

<Tabs>
  <Tab title="TypeScript">
    O SDK TypeScript lança um `Error` declarando a regra que a entrada quebrou. Por exemplo, `skills: ["docs:*"]` lança:

    ```text theme={null}
    Invalid skill name "docs:*": wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Um nome vazio relata `Skill names must be non-empty strings.`

    Antes do TypeScript Agent SDK 0.3.221, o SDK não executava esta verificação.
  </Tab>

  <Tab title="Python">
    O SDK Python levanta `ValueError` declarando a regra que a entrada quebrou. Por exemplo, `skills=["docs:*"]` levanta:

    ```text theme={null}
    ValueError: Invalid skill name 'docs:*': wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Um nome vazio relata `Skill names must be non-empty strings`.

    Antes do Python Agent SDK 0.2.129, o SDK não executava esta verificação.
  </Tab>
</Tabs>

<h3 id="additional-troubleshooting">
  Solução de problemas adicional
</h3>

Para solução de problemas geral de skills, como erros de sintaxe YAML e depuração, consulte a [seção de solução de problemas de skills do Claude Code](/docs/pt/skills#troubleshooting).

<h2 id="next-steps">
  Próximos passos
</h2>

O [guia de skills do Claude Code](/docs/pt/skills) cobre autoria em profundidade. Sua orientação se aplica a sessões do SDK. Comece com estas seções:

* [Referência de frontmatter](/docs/pt/skills#frontmatter-reference): cada campo suportado
* [Passe argumentos para skills](/docs/pt/skills#pass-arguments-to-skills): `$ARGUMENTS`, `$0`, `$1` e empilhamento de skills. A [tabela de substituição completa](/docs/pt/skills#available-string-substitutions) adiciona argumentos nomeados e as variáveis `${CLAUDE_*}`
* [Injete contexto dinâmico](/docs/pt/skills#inject-dynamic-context): linhas `` !`command` `` que executam antes de Claude ver o conteúdo da skill
* [Escolha onde skills carregam](/docs/pt/skills#where-skills-live): cada local de skill, namespacing de plugin e qual skill executa quando dois compartilham um nome

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Comandos em Claude Code](/docs/pt/commands): a superfície de comando completa, incluindo cada integrado
* [Visão geral de Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview): visão geral conceitual, benefícios e arquitetura
* [Práticas recomendadas de Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): diretrizes de autoria para skills eficazes
* [Livro de receitas de Agent Skills](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): skills de exemplo e templates
* [Subagentes no SDK](/docs/pt/agent-sdk/subagents): agentes similares baseados em sistema de arquivos com opções programáticas
* [Visão geral do SDK](/docs/pt/agent-sdk/overview): conceitos gerais do SDK
* [Referência TypeScript SDK](/docs/pt/agent-sdk/typescript): documentação completa da API
* [Referência Python SDK](/docs/pt/agent-sdk/python): documentação completa da API
