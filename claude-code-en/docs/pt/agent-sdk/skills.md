> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent Skills no SDK

> Estenda Claude com capacidades especializadas usando Agent Skills no Claude Agent SDK

<h2 id="overview">
  Visão Geral
</h2>

Agent Skills estendem Claude com capacidades especializadas que Claude invoca autonomamente quando relevante. Skills são empacotadas como arquivos `SKILL.md` contendo instruções, descrições e recursos de suporte opcionais.

Para informações abrangentes sobre Skills, incluindo benefícios, arquitetura e diretrizes de autoria, consulte a [visão geral de Agent Skills](https://platform.claude.com/docs/pt/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-sdk">
  Como Skills Funcionam com o SDK
</h2>

Ao usar o Claude Agent SDK, Skills são:

1. **Definidas como artefatos do sistema de arquivos**: Criadas como arquivos `SKILL.md` em diretórios específicos (`.claude/skills/`)
2. **Carregadas do sistema de arquivos**: Skills são carregadas de locais do sistema de arquivos governados por `settingSources` (TypeScript) ou `setting_sources` (Python)
3. **Descobertas automaticamente**: Uma vez que as configurações do sistema de arquivos são carregadas, os metadados de Skill são descobertos na inicialização a partir de diretórios de usuário e projeto; conteúdo completo carregado quando acionado
4. **Invocadas pelo modelo**: Claude escolhe autonomamente quando usá-las com base no contexto
5. **Filtradas via opção `skills`**: Skills descobertas são habilitadas por padrão. Passe uma lista de nomes de skills, `"all"`, ou `[]` para controlar quais estão disponíveis na sessão

Diferentemente de subagentes (que podem ser definidos programaticamente), Skills devem ser criadas como artefatos do sistema de arquivos. O SDK não fornece uma API programática para registrar Skills.

<Note>
  Skills são descobertas através das fontes de configuração do sistema de arquivos. Com opções padrão de `query()`, o SDK carrega fontes de usuário e projeto, portanto skills em `~/.claude/skills/`, `<cwd>/.claude/skills/` e `.claude/skills/` em qualquer diretório pai de `<cwd>` até a raiz do repositório estão disponíveis. Se você definir `settingSources` explicitamente, inclua `'user'` ou `'project'` para manter a descoberta de skills, ou use a [opção `plugins`](/docs/pt/agent-sdk/plugins) para carregar skills de um caminho específico.
</Note>

<h2 id="using-skills-with-the-sdk">
  Usando Skills com o SDK
</h2>

Defina a opção `skills` em `query()` para controlar quais Skills estão disponíveis para a sessão. Quando omitida, Skills descobertas são habilitadas e a ferramenta Skill está disponível, correspondendo ao comportamento da CLI. Passe `"all"` para habilitar cada Skill descoberta, uma lista de nomes de Skill para habilitar apenas aquelas, ou `[]` para desabilitar todas. Quando você define `skills`, o SDK adiciona a ferramenta Skill a `allowedTools` automaticamente. Se você também passar uma lista explícita de `tools`, inclua `"Skill"` nessa lista para que Claude possa invocar skills.

Uma vez configurado, Claude descobre automaticamente Skills do sistema de arquivos e as invoca quando relevante para a solicitação do usuário.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd="/path/to/project",  # Project with .claude/skills/
          setting_sources=["user", "project"],  # Load Skills from filesystem
          skills="all",  # Enable every discovered Skill
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
      cwd: "/path/to/project", // Project with .claude/skills/
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all", // Enable every discovered Skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Para habilitar apenas Skills específicas, passe seus nomes. Os nomes correspondem ao campo `name` em `SKILL.md` ou ao nome do diretório da Skill. Use `plugin:skill` para Skills fornecidas por plugin.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

A opção `skills` é um filtro de contexto, não uma sandbox. Skills não listadas são ocultadas do modelo e rejeitadas pela ferramenta Skill, mas seus arquivos permanecem no disco e são acessíveis através de Read e Bash.

<h2 id="skill-locations">
  Locais de Skill
</h2>

Skills são carregadas de diretórios do sistema de arquivos com base na sua configuração `settingSources`/`setting_sources`:

* **Project Skills** (`.claude/skills/`): Compartilhadas com sua equipe via git - carregadas quando `setting_sources` inclui `"project"`
* **User Skills** (`~/.claude/skills/`): Skills pessoais em todos os projetos - carregadas quando `setting_sources` inclui `"user"`
* **Plugin Skills**: Agrupadas com plugins Claude Code instalados

<h2 id="creating-skills">
  Criando Skills
</h2>

Skills são definidas como diretórios contendo um arquivo `SKILL.md` com frontmatter YAML e conteúdo Markdown. O campo `description` determina quando Claude invoca sua Skill.

**Exemplo de estrutura de diretório**:

```bash theme={null}
.claude/skills/processing-pdfs/
└── SKILL.md
```

Para orientação completa sobre criação de Skills, incluindo estrutura SKILL.md, Skills multi-arquivo e exemplos, consulte:

* [Agent Skills no Claude Code](/docs/pt/skills): Guia completo com exemplos
* [Agent Skills Best Practices](https://platform.claude.com/docs/pt/agents-and-tools/agent-skills/best-practices): Diretrizes de autoria e convenções de nomenclatura

<h2 id="tool-restrictions">
  Restrições de Ferramenta
</h2>

<Note>
  O campo frontmatter `allowed-tools` em SKILL.md é suportado apenas ao usar Claude Code CLI diretamente. **Ele não se aplica ao usar Skills através do SDK**.

  Ao usar o SDK, controle o acesso à ferramenta através da opção principal `allowedTools` na sua configuração de query.
</Note>

Para controlar o acesso à ferramenta para Skills em aplicações SDK, use `allowedTools` para pré-aprovar ferramentas específicas. Sem um callback `canUseTool`, qualquer coisa não na lista é negada:

<Note>
  As instruções de importação do primeiro exemplo são assumidas nos seguintes trechos de código.
</Note>

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )

  async for message in query(prompt="Analyze the codebase structure", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Analyze the codebase structure",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"],
      permissionMode: "dontAsk" // Deny anything not in allowedTools
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h2 id="discovering-available-skills">
  Descobrindo Skills Disponíveis
</h2>

Para ver quais Skills estão disponíveis em sua aplicação SDK, simplesmente pergunte a Claude:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
  )

  async for message in query(prompt="What Skills are available?", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "What Skills are available?",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all"
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude listará as Skills disponíveis com base no seu diretório de trabalho atual e plugins instalados.

<h2 id="testing-skills">
  Testando Skills
</h2>

Teste Skills fazendo perguntas que correspondam às suas descrições:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      cwd="/path/to/project",
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Bash"],
  )

  async for message in query(prompt="Extract text from invoice.pdf", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Extract text from invoice.pdf",
    options: {
      cwd: "/path/to/project",
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude invoca automaticamente a Skill relevante se a descrição corresponder à sua solicitação.

<h2 id="troubleshooting">
  Solução de Problemas
</h2>

<h3 id="skills-not-found">
  Skills Não Encontradas
</h3>

**Verifique a configuração settingSources**: Skills são descobertas através das fontes de configuração `user` e `project`. Se você definir `settingSources`/`setting_sources` explicitamente e omitir essas fontes, skills não são carregadas:

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
  const options = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const options = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Para mais detalhes sobre `settingSources`/`setting_sources`, consulte a [referência TypeScript SDK](/docs/pt/agent-sdk/typescript#settingsource) ou [referência Python SDK](/docs/pt/agent-sdk/python#settingsource).

**Verifique o diretório de trabalho**: O SDK carrega Skills de `.claude/skills/` na opção `cwd` e em todos os diretórios pai até a raiz do repositório. Certifique-se de que `cwd` aponta para ou abaixo do diretório contendo `.claude/skills/`, dentro do mesmo repositório:

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

Consulte a seção "Usando Skills com o SDK" acima para o padrão completo.

**Verifique o local do sistema de arquivos**:

```bash theme={null}
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill Não Sendo Usada
</h3>

**Verifique a opção `skills`**: Se você passou uma lista `skills`, confirme que o nome da skill está incluído. Passar `[]` desabilita todas as skills.

**Verifique a descrição**: Certifique-se de que é específica e inclui palavras-chave relevantes. Consulte [Agent Skills Best Practices](https://platform.claude.com/docs/pt/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) para orientação sobre como escrever descrições eficazes.

<h3 id="additional-troubleshooting">
  Solução de Problemas Adicional
</h3>

Para solução de problemas geral de Skills (sintaxe YAML, depuração, etc.), consulte a [seção de solução de problemas de Skills do Claude Code](/docs/pt/skills#troubleshooting).

<h2 id="related-documentation">
  Documentação Relacionada
</h2>

<h3 id="skills-guides">
  Guias de Skills
</h3>

* [Agent Skills no Claude Code](/docs/pt/skills): Guia completo de Skills com criação, exemplos e solução de problemas
* [Agent Skills Overview](https://platform.claude.com/docs/pt/agents-and-tools/agent-skills/overview): Visão geral conceitual, benefícios e arquitetura
* [Agent Skills Best Practices](https://platform.claude.com/docs/pt/agents-and-tools/agent-skills/best-practices): Diretrizes de autoria para Skills eficazes
* [Agent Skills Cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): Skills de exemplo e templates

<h3 id="sdk-resources">
  Recursos SDK
</h3>

* [Subagents no SDK](/docs/pt/agent-sdk/subagents): Agentes similares baseados em sistema de arquivos com opções programáticas
* [Slash Commands no SDK](/docs/pt/agent-sdk/slash-commands): Comandos invocados pelo usuário
* [Visão Geral do SDK](/docs/pt/agent-sdk/overview): Conceitos gerais do SDK
* [Referência TypeScript SDK](/docs/pt/agent-sdk/typescript): Documentação completa da API
* [Referência Python SDK](/docs/pt/agent-sdk/python): Documentação completa da API
