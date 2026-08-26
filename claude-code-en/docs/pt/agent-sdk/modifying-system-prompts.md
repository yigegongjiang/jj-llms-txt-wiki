> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modificando prompts do sistema

> Escolha entre a predefinição `claude_code` e um prompt do sistema personalizado, e personalize o comportamento com CLAUDE.md, estilos de saída, append ou um prompt totalmente personalizado.

Os prompts do sistema definem o comportamento, as capacidades e o estilo de resposta do Claude. Comece pela predefinição `claude_code` para ferramentas de codificação tipo CLI ou IDE onde um humano observa e orienta o trabalho. Escreva seu próprio prompt para agentes com uma superfície, identidade ou modelo de permissão diferentes.

Esta página cobre:

* [Como os prompts do sistema funcionam](#how-system-prompts-work), com uma tabela de decisão para escolher entre a predefinição, a predefinição com `append` e um prompt personalizado
* [Personalize o comportamento do agente](#customize-agent-behavior) com arquivos CLAUDE.md, estilos de saída, `append` ou uma string personalizada
* [Compare as quatro abordagens](#compare-the-four-approaches) por persistência, escopo e o que elas preservam
* [Combine abordagens](#combine-approaches) para sobrepor métodos de personalização

<h2 id="how-system-prompts-work">
  Como os prompts do sistema funcionam
</h2>

Um prompt do sistema é o conjunto inicial de instruções que molda como o Claude se comporta ao longo de uma conversa. O Agent SDK tem três pontos de partida para isso:

* **Padrão mínimo**: quando você não define `systemPrompt` em TypeScript ou `system_prompt` em Python, o SDK usa um prompt mínimo que cobre chamadas de ferramentas, mas omite as diretrizes de codificação do Claude Code, estilo de resposta e contexto do projeto. Isso difere de `claude -p`, que usa o prompt completo do Claude Code por padrão. Se você está migrando da CLI e quer um comportamento correspondente, defina o preset `claude_code`.
* **Preset `claude_code`**: o prompt do sistema completo que a CLI do Claude Code usa, com instruções de uso de ferramentas, diretrizes de estilo e formatação de código, regras de tom de resposta e verbosidade, instruções de segurança e proteção, e contexto sobre o diretório de trabalho e ambiente. Defina `systemPrompt: { type: "preset", preset: "claude_code" }` em TypeScript ou `system_prompt={"type": "preset", "preset": "claude_code"}` em Python, opcionalmente com `append` para adicionar suas próprias instruções no final.
* **String personalizada**: um prompt que você escreve por conta própria. O SDK envia apenas o que você fornece.

<h3 id="decide-on-a-starting-point">
  Decida sobre um ponto de partida
</h3>

O fator decisivo é o quão próximo seu agente se assemelha ao Claude Code: um agente de codificação operando em um repositório, com um humano observando a saída em streaming e direcionando o trabalho. Quanto mais seu produto se afastar disso, mais você vai querer escrever seu próprio prompt.

| Você está construindo                                                                                                               | Use                               | O que você obtém                                                                                                                                            |
| :---------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Uma ferramenta de codificação tipo CLI ou IDE onde um humano observa e direciona, e os padrões do Claude Code são o que você quer   | Preset `claude_code`              | O prompt completo do Claude Code: orientação de ferramentas, regras de segurança, respostas amigáveis ao terminal, consciência de convenções de repositório |
| O mesmo tipo de ferramenta, mais regras específicas do produto como padrões de codificação, formato de saída ou contexto de domínio | Preset `claude_code` com `append` | Tudo acima, com suas instruções adicionadas após o preset. Nada é removido, então essa é a customização de menor risco                                      |
| Um agente com uma superfície, identidade ou modelo de permissão diferente, ou um agente não-codificação                             | String de prompt personalizado    | Apenas o que você escreve. Você assume a responsabilidade de substituir a orientação de ferramentas e instruções de segurança que seu agente ainda precisa  |
| Um loop de chamada de ferramentas fino sem persona de agente, onde você fornece todo o comportamento no prompt do usuário           | Nenhuma opção `systemPrompt`      | O padrão mínimo: suporte a chamadas de ferramentas e nada mais                                                                                              |

"Diferente do Claude Code" geralmente significa um dos seguintes:

* **Superfície diferente**: a saída não é lida em um terminal pela pessoa que a acionou. UIs de chat, consumidores de saída estruturada e automação não-codificação cada uma precisa de um prompt que corresponda a como sua saída é renderizada e revisada. Automação de codificação desatendida, como um job de CI que corrige erros de lint ou revisa diffs, ainda se encaixa no preset porque o trabalho em si é o que o preset foi escrito para.
* **Identidade diferente**: o agente não deve se apresentar como Claude Code. Um bot de suporte, um assistente de análise de dados ou qualquer agente específico de domínio precisa de seu próprio nome, escopo e persona.
* **Modelo de permissão diferente**: o agente é executado autonomamente sem um humano aprovando cada etapa, ou opera em um conjunto restrito de recursos. O prompt do Claude Code assume que um humano está no loop com acesso a um conjunto completo de ferramentas.
* **Tarefas não-codificação**: a maior parte do prompt do Claude Code é orientação de codificação. Para agentes de pesquisa, conteúdo ou operações, essa orientação compete com as instruções que você realmente precisa.

A [tabela de comparação](#compare-the-four-approaches) mostra o que cada método de customização preserva.

<h2 id="customize-agent-behavior">
  Personalizar o comportamento do agente
</h2>

Estilos de saída, `append`, e uma string de prompt personalizada cada um alteram o prompt do sistema diretamente. CLAUDE.md segue um caminho diferente: o SDK o lê e injeta seu conteúdo na conversa como contexto do projeto, não no prompt do sistema, então ele molda o comportamento junto com qualquer prompt do sistema que você escolher. [Skills](/docs/pt/agent-sdk/skills), [hooks](/docs/pt/agent-sdk/hooks), e [permissions](/docs/pt/agent-sdk/permissions) também moldam o comportamento fora do prompt do sistema e são cobertos em suas próprias páginas.

<h3 id="claude-md-files-for-project-level-instructions">
  Arquivos CLAUDE.md para instruções em nível de projeto
</h3>

Os arquivos CLAUDE.md fornecem ao Claude contexto e instruções persistentes do projeto. O SDK injeta seu conteúdo na conversa, não no prompt do sistema, então funcionam com qualquer configuração de prompt do sistema. Para saber o que colocar em CLAUDE.md, onde colocá-lo e como escrever instruções eficazes, veja [How Claude remembers your project](/docs/pt/memory). Esta seção cobre o que é específico do SDK: como CLAUDE.md é carregado.

O SDK lê CLAUDE.md quando a fonte de configuração correspondente está habilitada: `'project'` carrega `CLAUDE.md` ou `.claude/CLAUDE.md` do diretório de trabalho, e `'user'` carrega `~/.claude/CLAUDE.md`. As opções padrão de `query()` habilitam ambas as fontes, então CLAUDE.md é carregado automaticamente. Se você definir `settingSources` em TypeScript ou `setting_sources` em Python explicitamente, inclua as fontes que você precisa. O carregamento de CLAUDE.md é controlado por fontes de configuração, não pela predefinição `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Carregar CLAUDE.md com o SDK
</h4>

Para carregar CLAUDE.md, defina `settingSources` para incluir o nível onde seu CLAUDE.md reside. O exemplo abaixo carrega um CLAUDE.md em nível de projeto junto com a predefinição `claude_code`, então Claude tem tanto o prompt completo do agente de codificação quanto as convenções do seu projeto:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Add a new React component for user profiles",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code" // Use Claude Code's system prompt
      },
      settingSources: ["project"] // Loads CLAUDE.md from project
    }
  })) {
    messages.push(message);
  }

  // Now Claude has access to your project guidelines from CLAUDE.md
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []

  async for message in query(
      prompt="Add a new React component for user profiles",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",  # Use Claude Code's system prompt
          },
          setting_sources=["project"],  # Loads CLAUDE.md from project
      ),
  ):
      messages.append(message)

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

CLAUDE.md é persistente em todas as sessões em um projeto, compartilhado com sua equipe através do git, e descoberto automaticamente sem alterações de código. Não é carregado se você passar um array `settingSources` vazio.

<h3 id="output-styles-for-persistent-configurations">
  Estilos de saída para configurações persistentes
</h3>

Estilos de saída são configurações salvas que modificam o prompt do sistema do Claude. Eles são armazenados como arquivos markdown e podem ser reutilizados em sessões e projetos.

<h4 id="create-an-output-style">
  Criar um estilo de saída
</h4>

Um estilo de saída é um arquivo markdown com [frontmatter](/docs/pt/output-styles#frontmatter) para metadados, seguido pelo conteúdo do prompt. Salve-o em `~/.claude/output-styles/` para um estilo em nível de usuário disponível em cada projeto, ou `.claude/output-styles/` em seu repositório para um estilo em nível de projeto que você pode fazer commit e compartilhar com sua equipe.

Por padrão, um estilo de saída personalizado substitui as instruções de engenharia de software da predefinição `claude_code` pelas suas próprias. Para mantê-las e colocar suas instruções em camadas no topo, defina `keep-coding-instructions: true` no frontmatter. Mantenha-as quando seu agente ainda estiver fazendo trabalho de engenharia de software. Deixe-as de fora quando você estiver substituindo o papel completamente.

O exemplo abaixo define uma persona de revisão de código que mantém as instruções de codificação, já que revisar código ainda se beneficia da orientação de segurança e qualidade de código do Claude Code. Salve-o como `~/.claude/output-styles/code-reviewer.md` para torná-lo disponível em todos os projetos:

```markdown ~/.claude/output-styles/code-reviewer.md theme={null}
---
name: Code Reviewer
description: Thorough code review assistant
keep-coding-instructions: true
---

You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)
```

<h4 id="activate-an-output-style">
  Ativar um estilo de saída
</h4>

Uma vez criado, ative estilos de saída via:

* **CLI**: execute `/config` e selecione um estilo de saída
* **Configurações**: defina `outputStyle` em `.claude/settings.local.json`
* **TypeScript SDK**: defina `outputStyle` dentro do objeto `settings` inline passado para `query()`, ou aponte `settings` para um arquivo de configurações que o defina. `outputStyle` não é um campo `Options` de nível superior:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

O SDK Python não tem uma opção para selecionar um estilo de saída programaticamente. Para implantações apenas de código onde você não pode escrever em `.claude/settings.local.json`, use `append` ou uma string de prompt personalizada.

**Nota para usuários do SDK:** Estilos de saída são carregados quando você inclui `settingSources: ['user']` ou `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` ou `setting_sources=["project"]` (Python) em suas opções.

<h3 id="append-to-the-claude_code-preset">
  Adicionar à predefinição `claude_code`
</h3>

Você pode usar a predefinição Claude Code com uma propriedade `append` para adicionar suas instruções personalizadas enquanto preserva toda a funcionalidade integrada.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Help me write a Python function to calculate fibonacci numbers",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "Always include detailed docstrings and type hints in Python code."
      }
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []

  async for message in query(
      prompt="Help me write a Python function to calculate fibonacci numbers",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "Always include detailed docstrings and type hints in Python code.",
          }
      ),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Melhorar o cache de prompt entre usuários e máquinas
</h4>

Por padrão, duas sessões que usam a mesma predefinição `claude_code` e texto `append` ainda não podem compartilhar uma entrada de cache de prompt se forem executadas de diretórios de trabalho diferentes. Isso ocorre porque a predefinição incorpora contexto por sessão no prompt do sistema antes do seu texto `append`: o diretório de trabalho, se é um repositório git, a plataforma, o shell ativo, a versão do SO, e caminhos de auto-memória. Qualquer diferença nesse contexto produz um prompt do sistema diferente e uma falha de cache. O conteúdo de CLAUDE.md não afeta o cache do prompt do sistema porque o SDK o injeta na conversa, não no prompt do sistema.

Para tornar o prompt do sistema idêntico em sessões, defina `excludeDynamicSections: true` em TypeScript ou `"exclude_dynamic_sections": True` em Python. O contexto por sessão se move para a primeira mensagem do usuário, deixando apenas a predefinição estática e seu texto `append` no prompt do sistema para que configurações idênticas compartilhem uma entrada de cache em usuários e máquinas.

<Note>
  `excludeDynamicSections` requer `@anthropic-ai/claude-agent-sdk` v0.2.98 ou posterior, ou `claude-agent-sdk` v0.1.58 ou posterior para Python. Aplica-se apenas ao formulário de objeto predefinido e não tem efeito quando `systemPrompt` é uma string.
</Note>

O exemplo a seguir emparelha um bloco `append` compartilhado com `excludeDynamicSections` para que uma frota de agentes executados de diretórios diferentes possa reutilizar o mesmo prompt do sistema em cache:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Triage the open issues in this repo",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "You operate Acme's internal triage workflow. Label issues by component and severity.",
        excludeDynamicSections: true
      }
    }
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Triage the open issues in this repo",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "You operate Acme's internal triage workflow. Label issues by component and severity.",
              "exclude_dynamic_sections": True,
          },
      ),
  ):
      ...
  ```
</CodeGroup>

**Tradeoffs:** o diretório de trabalho, a flag de repositório git, a plataforma, o shell ativo, a versão do SO, e caminhos de auto-memória ainda chegam ao Claude, mas como parte da primeira mensagem do usuário em vez do prompt do sistema. Instruções na mensagem do usuário têm peso marginalmente menor do que o mesmo texto no prompt do sistema, então Claude pode depender delas menos fortemente ao raciocinar sobre o diretório atual ou caminhos de auto-memória. Ative esta opção quando a reutilização de cache entre sessões for mais importante do que contexto de ambiente maximamente autoritário.

Para a flag equivalente no modo CLI não interativo, veja [`--exclude-dynamic-system-prompt-sections`](/docs/pt/cli-reference).

<h3 id="custom-system-prompts">
  Prompts do sistema personalizados
</h3>

Você pode fornecer uma string personalizada como `systemPrompt` para substituir completamente o padrão pelas suas próprias instruções.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const customPrompt = `You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices`;

  const messages = [];

  for await (const message of query({
    prompt: "Create a data processing pipeline",
    options: {
      systemPrompt: customPrompt
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []

  async for message in query(
      prompt="Create a data processing pipeline",
      options=ClaudeAgentOptions(system_prompt=custom_prompt),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h2 id="compare-the-four-approaches">
  Comparação das quatro abordagens
</h2>

Os quatro métodos de personalização diferem em onde vivem, como são compartilhados e o que preservam da predefinição `claude_code`.

| Recurso                     | CLAUDE.md              | Estilos de Saída              | `systemPrompt` com append | `systemPrompt` personalizado     |
| --------------------------- | ---------------------- | ----------------------------- | ------------------------- | -------------------------------- |
| **Persistência**            | Arquivo por projeto    | Salvo como arquivos           | Apenas sessão             | Apenas sessão                    |
| **Reutilização**            | Por projeto            | Entre projetos                | Duplicação de código      | Duplicação de código             |
| **Gerenciamento**           | No sistema de arquivos | CLI + arquivos                | No código                 | No código                        |
| **Ferramentas padrão**      | Preservadas            | Preservadas                   | Preservadas               | Perdidas (a menos que incluídas) |
| **Segurança integrada**     | Mantida                | Mantida                       | Mantida                   | Deve ser adicionada              |
| **Contexto de ambiente**    | Automático             | Automático                    | Automático                | Deve ser fornecido               |
| **Nível de personalização** | Apenas adições         | Substituir ou estender padrão | Apenas adições            | Controle completo                |
| **Controle de versão**      | Com projeto            | Sim                           | Com código                | Com código                       |
| **Escopo**                  | Específico do projeto  | Usuário ou projeto            | Sessão de código          | Sessão de código                 |

"Com append" significa usar `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` em TypeScript ou `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` em Python. CLAUDE.md não altera o prompt do sistema em si: o SDK injeta seu conteúdo na conversa como contexto do projeto.

<h2 id="use-cases-and-best-practices">
  Casos de uso e melhores práticas
</h2>

<h3 id="when-to-use-claude-md">
  Quando usar CLAUDE.md
</h3>

Use CLAUDE.md para instruções que devem se aplicar a cada sessão em um projeto, independentemente de qual prompt do sistema a sessão usa: padrões de codificação, comandos comuns, contexto de arquitetura e convenções de equipe. CLAUDE.md é confirmado em seu repositório, portanto permanece sincronizado com o código que descreve. Consulte [Quando adicionar a CLAUDE.md](/docs/pt/memory#when-to-add-to-claude-md) para orientação completa.

Os arquivos CLAUDE.md são carregados quando a fonte de configuração `project` está habilitada, o que é o padrão para as opções padrão de `query()`. Se você definir `settingSources` em TypeScript ou `setting_sources` em Python explicitamente, inclua `'project'` para continuar carregando CLAUDE.md em nível de projeto.

<h3 id="when-to-use-output-styles">
  Quando usar estilos de saída
</h3>

Os estilos de saída são para personas que você deseja reutilizar em toda a CLI e SDK sem alterar o código da aplicação. Como vivem como arquivos em `.claude/output-styles`, a mesma persona está disponível em `/config` na CLI e em qualquer sessão SDK que carregue a fonte de configuração correspondente.

**Melhor para:**

* Mudanças de comportamento persistentes em sessões
* Configurações compartilhadas em equipe
* Assistentes especializados como revisor de código, cientista de dados ou assistente DevOps
* Modificações de prompt complexas que precisam de versionamento

**Exemplos:**

* Criar um assistente dedicado de otimização SQL
* Construir um revisor de código focado em segurança
* Desenvolver um assistente de ensino com pedagogia específica

<h3 id="when-to-use-systemprompt-with-append">
  Quando usar `systemPrompt` com append
</h3>

Use `append` quando a predefinição `claude_code` já se adequa ao seu produto e você só precisa adicionar instruções extras. Você mantém a orientação de ferramentas, regras de segurança e convenções de codificação da predefinição sem reimplementá-las.

**Melhor para:**

* Adicionar padrões de codificação ou preferências específicas
* Personalizar formatação de saída
* Adicionar conhecimento específico do domínio
* Modificar verbosidade de resposta
* Aprimorar o comportamento padrão do Claude Code sem perder instruções de ferramentas

<h3 id="when-to-use-custom-systemprompt">
  Quando usar `systemPrompt` personalizado
</h3>

Use um prompt personalizado quando a superfície, identidade ou modelo de permissão do seu agente diferir do Claude Code, conforme descrito em [Decidir sobre um ponto de partida](#decide-on-a-starting-point). Você define o conjunto completo de instruções, incluindo qualquer orientação de ferramentas e regras de segurança que seu agente precisa.

**Melhor para:**

* Controle completo sobre o comportamento do Claude
* Tarefas especializadas de sessão única
* Testar novas estratégias de prompt
* Situações onde ferramentas padrão não são necessárias
* Construir agentes especializados com comportamento único

<h2 id="combine-approaches">
  Combinando abordagens
</h2>

Esses métodos se compõem. Um estilo de saída persistente ou CLAUDE.md define o comportamento de longa duração, e `append` adiciona instruções específicas da sessão sem tocar na configuração salva.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Combinar um estilo de saída com adições específicas da sessão
</h3>

O exemplo abaixo assume que um estilo de saída Code Reviewer já está ativo. O bloco `append` adiciona áreas de foco específicas da sessão sobre a persona, para que uma única sessão de revisão possa priorizar OAuth e armazenamento de tokens sem alterar o estilo de saída salvo:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Assuming "Code Reviewer" output style is active (via /config or settings)
  // Add session-specific focus areas
  const messages = [];

  for await (const message of query({
    prompt: "Review this authentication module",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: `
          For this review, prioritize:
          - OAuth 2.0 compliance
          - Token storage security
          - Session management
        `
      }
    }
  })) {
    messages.push(message);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []

  async for message in query(
      prompt="Review this authentication module",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": """
              For this review, prioritize:
              - OAuth 2.0 compliance
              - Token storage security
              - Session management
              """,
          }
      ),
  ):
      messages.append(message)
  ```
</CodeGroup>

<h2 id="see-also">
  Veja também
</h2>

* [Estilos de saída](/docs/pt/output-styles): criar, gerenciar e compartilhar estilos de saída para a CLI, incluindo o formato de arquivo e locais de armazenamento
* [Como Claude se lembra do seu projeto](/docs/pt/memory): o que colocar em CLAUDE.md, onde colocá-lo e como escrever instruções de projeto eficazes
* [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript): o tipo `Options` completo, incluindo `systemPrompt`, `settingSources` e `settings`
* [Referência do SDK Python](/docs/pt/agent-sdk/python): o tipo `ClaudeAgentOptions` completo, incluindo `system_prompt` e `setting_sources`
* [Configurações](/docs/pt/settings): a referência `settings.json`, incluindo onde estilos de saída e outras configurações são armazenados
