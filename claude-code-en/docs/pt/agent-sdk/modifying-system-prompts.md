> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modificando prompts do sistema

> Escolha entre a predefinição `claude_code` e um prompt do sistema personalizado, e personalize o comportamento com CLAUDE.md, estilos de saída, append ou um prompt totalmente personalizado.

Os prompts do sistema definem o comportamento, as capacidades e o estilo de resposta do Claude. Comece pela predefinição `claude_code` para ferramentas de codificação tipo CLI ou IDE onde um humano observa e orienta o trabalho. Escreva seu próprio prompt para agentes com uma superfície, identidade ou modelo de permissão diferentes.

<h2 id="how-system-prompts-work">
  Como os prompts do sistema funcionam
</h2>

Um prompt do sistema é o conjunto inicial de instruções que molda como o Claude se comporta ao longo de uma conversa. O Agent SDK tem três pontos de partida para isso:

* **Padrão mínimo**: quando você não define `systemPrompt` em TypeScript ou `system_prompt` em Python, o SDK usa um prompt mínimo que cobre chamadas de ferramentas, mas omite o resto do conteúdo do preset `claude_code`, incluindo suas instruções de segurança e proteção e seu contexto sobre o diretório de trabalho e ambiente. Isso difere de `claude -p`, que usa o prompt do sistema Claude Code por padrão. Se você está migrando da CLI e quer um comportamento correspondente, defina o preset `claude_code`.
* **Preset `claude_code`**: o prompt do sistema que a CLI do Claude Code usa, com instruções de uso de ferramentas, instruções de segurança e proteção, e contexto sobre o diretório de trabalho e ambiente. Defina `systemPrompt: { type: "preset", preset: "claude_code" }` em TypeScript ou `system_prompt={"type": "preset", "preset": "claude_code"}` em Python, opcionalmente com `append` para adicionar suas próprias instruções no final.
* **String personalizada**: um prompt que você escreve por conta própria. O SDK envia apenas o que você fornece.

<h3 id="decide-on-a-starting-point">
  Decida sobre um ponto de partida
</h3>

O fator decisivo é o quão próximo seu agente se assemelha ao Claude Code: um agente de codificação operando em um repositório, com um humano observando a saída em streaming e direcionando o trabalho. Quanto mais seu produto se afastar disso, mais você vai querer escrever seu próprio prompt.

| Você está construindo                                                                                                               | Use                               | O que você obtém                                                                                                                                           |
| :---------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Uma ferramenta de codificação tipo CLI ou IDE onde um humano observa e direciona, e os padrões do Claude Code são o que você quer   | Preset `claude_code`              | O prompt do Claude Code, incluindo orientação de ferramentas, regras de segurança e contexto de ambiente                                                   |
| O mesmo tipo de ferramenta, mais regras específicas do produto como padrões de codificação, formato de saída ou contexto de domínio | Preset `claude_code` com `append` | Tudo acima, com suas instruções adicionadas após o preset. Nada é removido, então essa é a customização de menor risco                                     |
| Um agente com uma superfície, identidade ou modelo de permissão diferente, ou um agente não-codificação                             | String de prompt personalizado    | Apenas o que você escreve. Você assume a responsabilidade de substituir a orientação de ferramentas e instruções de segurança que seu agente ainda precisa |
| Um loop de chamada de ferramentas fino sem persona de agente, onde você fornece todo o comportamento no prompt do usuário           | Nenhuma opção `systemPrompt`      | O padrão mínimo: suporte a chamadas de ferramentas e nada mais                                                                                             |

"Diferente do Claude Code" geralmente significa um dos seguintes:

* **Superfície diferente**: a saída não é lida em um terminal pela pessoa que a acionou. UIs de chat, consumidores de saída estruturada e automação não-codificação cada uma precisa de um prompt que corresponda a como sua saída é renderizada e revisada. Automação de codificação desatendida, como um job de CI que corrige erros de lint ou revisa diffs, ainda se encaixa no preset porque o trabalho em si é o que o preset foi escrito para.
* **Identidade diferente**: o agente não deve se apresentar como Claude Code. Um bot de suporte, um assistente de análise de dados ou qualquer agente específico de domínio precisa de seu próprio nome, escopo e persona.
* **Modelo de permissão diferente**: o agente é executado autonomamente sem um humano aprovando cada etapa, ou opera em um conjunto restrito de recursos. O prompt do Claude Code assume que um humano está no loop com acesso a um conjunto completo de ferramentas.
* **Tarefas não-codificação**: a maior parte do prompt do Claude Code é orientação de codificação. Para agentes de pesquisa, conteúdo ou operações, essa orientação compete com as instruções que você realmente precisa.

A [tabela de comparação](#compare-the-four-approaches) mostra o que cada método de customização preserva.

<h2 id="customize-agent-behavior">
  Personalizar o comportamento do agente
</h2>

`append` e uma string de prompt personalizada cada um alteram o prompt do sistema diretamente, e um estilo de saída altera as instruções que Claude Code fornece ao Claude para cada resposta. CLAUDE.md segue um caminho diferente: o SDK o lê e injeta seu conteúdo na conversa como contexto do projeto, então ele molda o comportamento junto com qualquer prompt do sistema que você escolher. [Skills](/docs/pt/agent-sdk/skills), [hooks](/docs/pt/agent-sdk/hooks), e [permissions](/docs/pt/agent-sdk/permissions) também moldam o comportamento fora do prompt do sistema e são cobertos em suas próprias páginas.

<h3 id="claude-md-files-for-project-level-instructions">
  Arquivos CLAUDE.md para instruções em nível de projeto
</h3>

Os arquivos CLAUDE.md fornecem ao Claude contexto e instruções persistentes do projeto. O SDK injeta seu conteúdo na conversa e deixa o prompt do sistema intocado, então funcionam com qualquer configuração de prompt do sistema. Para saber o que colocar em CLAUDE.md, onde colocá-lo e como escrever instruções eficazes, veja [When to add to CLAUDE.md](/docs/pt/memory#when-to-add-to-claude-md) e o resto de [How Claude remembers your project](/docs/pt/memory). Esta seção cobre o que é específico do SDK: como CLAUDE.md é carregado.

O SDK lê CLAUDE.md quando a fonte de configuração correspondente está habilitada: `'project'` carrega `CLAUDE.md` ou `.claude/CLAUDE.md` do diretório de trabalho, e `'user'` carrega `~/.claude/CLAUDE.md`. As opções padrão de `query()` habilitam ambas as fontes, então CLAUDE.md é carregado automaticamente. Se você definir `settingSources` em TypeScript ou `setting_sources` em Python explicitamente, inclua as fontes que você precisa. O carregamento de CLAUDE.md é controlado por fontes de configuração, não pela predefinição `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Carregar CLAUDE.md com o SDK
</h4>

Para carregar CLAUDE.md, defina `settingSources` para incluir o nível onde seu CLAUDE.md reside. O exemplo abaixo carrega um CLAUDE.md em nível de projeto junto com a predefinição `claude_code`, então Claude tem tanto o prompt do agente de codificação quanto as convenções do seu projeto:

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []


  async def main():
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


  asyncio.run(main())

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

Quando você executa qualquer um dos exemplos, o SDK transmite mensagens conforme Claude trabalha: uma mensagem de inicialização do sistema, mensagens do assistente, mensagens do usuário carregando resultados de ferramentas, e uma mensagem de resultado final com o resultado da sessão.

CLAUDE.md é persistente em todas as sessões em um projeto, compartilhado com sua equipe através do git, e descoberto automaticamente sem alterações de código. Não é carregado se você passar um array `settingSources` vazio.

<h3 id="output-styles-for-persistent-configurations">
  Estilos de saída para configurações persistentes
</h3>

Estilos de saída são configurações salvas de instruções que alteram o papel, tom e formato de saída do Claude. Eles são armazenados como arquivos markdown e podem ser reutilizados em sessões e projetos.

<h4 id="create-an-output-style">
  Criar um estilo de saída
</h4>

Um estilo de saída é um arquivo markdown com [frontmatter](/docs/pt/output-styles#frontmatter) para metadados, seguido pelo conteúdo do prompt. Salve-o em `~/.claude/output-styles/` para um estilo em nível de usuário disponível em cada projeto, ou `.claude/output-styles/` em seu repositório para um estilo em nível de projeto que você pode fazer commit e compartilhar com sua equipe.

Um estilo de saída personalizado deixa as instruções de engenharia de software da predefinição `claude_code` de fora e usa as suas próprias. Para mantê-las e colocar suas instruções em camadas no topo, defina `keep-coding-instructions: true` no frontmatter. Essas instruções estão apenas no prompt do sistema completo do Claude Code, então a configuração não tem efeito em uma sessão no prompt do sistema mais curto, que você ativa ou desativa com [`CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT`](/docs/pt/env-vars#variables). Mantenha-as quando seu agente ainda estiver fazendo trabalho de engenharia de software. Deixe-as de fora quando você estiver substituindo o papel completamente.

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

* **CLI**: execute `/output-style <style>`, por exemplo `/output-style concise`, ou execute `/config` e selecione um. O comando `/output-style` requer Claude Code v2.1.269 ou posterior.
* **Configurações**: defina `outputStyle` em `.claude/settings.local.json`
* **TypeScript SDK**: defina `outputStyle` dentro do objeto `settings` inline passado para `query()`, ou aponte `settings` para um arquivo de configurações que o defina. `outputStyle` não é um campo `Options` de nível superior:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

No SDK Python, defina `outputStyle` através da opção `settings`, que aceita uma string JSON como `'{"outputStyle": "Explanatory"}'` ou um caminho para um arquivo de configurações que o defina.

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []


  async def main():
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


  asyncio.run(main())
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Melhorar o cache de prompt entre usuários e máquinas
</h4>

Por padrão, duas sessões que usam a mesma predefinição `claude_code` e texto `append` ainda não podem compartilhar uma entrada de cache de prompt se forem executadas de diretórios de trabalho diferentes. Isso ocorre porque a predefinição incorpora contexto por sessão no prompt do sistema antes do seu texto `append`: o diretório de trabalho, se é um repositório git, a plataforma, o shell ativo, a versão do SO, e caminhos de auto-memória. Qualquer diferença nesse contexto produz um prompt do sistema diferente e uma falha de cache. O conteúdo de CLAUDE.md não afeta o cache do prompt do sistema porque o SDK o injeta na conversa, não no prompt do sistema.

Para tornar o prompt do sistema idêntico em sessões, defina `excludeDynamicSections: true` em TypeScript ou `"exclude_dynamic_sections": True` em Python. O contexto por sessão se move para a primeira mensagem do usuário, deixando apenas a predefinição estática e seu texto `append` no prompt do sistema para que configurações idênticas compartilhem uma entrada de cache em usuários e máquinas.

<Note>
  `excludeDynamicSections` requer `@anthropic-ai/claude-agent-sdk` v0.2.98 ou posterior, ou `claude-agent-sdk` v0.1.58 ou posterior para Python. Defina-o apenas no formulário de objeto predefinido. O SDK o ignora quando você passa um prompt personalizado em vez da predefinição; para manter as instruções de um prompt personalizado em cache no SDK TypeScript, veja [Cache the static part of a custom prompt](#cache-the-static-part-of-a-custom-prompt).
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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
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


  asyncio.run(main())
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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []


  async def main():
      async for message in query(
          prompt="Create a data processing pipeline",
          options=ClaudeAgentOptions(system_prompt=custom_prompt),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

Em Python, carregue um prompt personalizado grande de um arquivo com `system_prompt={"type": "file", "path": "..."}` em vez de passá-lo como uma string. O SDK Python passa um prompt de string como um argumento de linha de comando para o subprocesso CLI, então um prompt que excede o limite de comprimento de argumento do SO falha na geração de processo antes de qualquer solicitação de API ser enviada. No Linux o erro é `Argument list too long`. Veja [`SystemPromptFile`](/docs/pt/agent-sdk/python#systempromptfile) para os limites da plataforma e o comportamento do Windows.

<h4 id="cache-the-static-part-of-a-custom-prompt">
  Cache da parte estática de um prompt personalizado
</h4>

No SDK TypeScript, você pode passar um prompt personalizado como um array de strings em vez de uma string, com o marcador `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` entre a parte estática e o resto. Use isso quando seu prompt combina instruções que são as mesmas em cada solicitação com contexto que muda por solicitação, como o cliente ou ticket que o agente está tratando. Quando você passa ambas as partes como uma string, uma mudança na parte por solicitação muda todo o prompt do sistema, então as instruções estáticas perdem o cache também. Este formulário não está disponível no SDK Python; [`ClaudeAgentOptions`](/docs/pt/agent-sdk/python#claudeagentoptions) lista os formulários que `system_prompt` aceita.

<Note>
  O SDK divide o prompt apenas quando chama a API Claude diretamente ou executa em [Claude Platform on AWS](/docs/pt/claude-platform-on-aws). Em todas as outras configurações, como Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou um [LLM gateway](/docs/pt/llm-gateway-connect), ele envia todo o prompt como um bloco, o mesmo que passar uma string. O mesmo acontece sempre que você define [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/pt/llm-gateway-protocol#disable-pre-release-capabilities).
</Note>

Para dividir o prompt, importe `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` de `@anthropic-ai/claude-agent-sdk` e passe-o como seu próprio elemento de array entre as duas partes. O SDK envia as strings antes do marcador como um bloco de texto e as strings após ele como um segundo bloco, cada um com seu próprio ponto de quebra de cache. No exemplo abaixo, um agente de suporte carrega suas instruções de triagem de um arquivo e recebe detalhes sobre um ticket em cada solicitação, então as instruções permanecem em cache enquanto os detalhes do ticket mudam:

```typescript TypeScript theme={null}
import { readFile } from "node:fs/promises";
import { query, SYSTEM_PROMPT_DYNAMIC_BOUNDARY } from "@anthropic-ai/claude-agent-sdk";

// Identical on every request
const instructions = await readFile("triage-instructions.md", "utf8");
// Different on every request
const ticketContext = "Customer plan: Enterprise. Other open tickets from this customer: 3.";

for await (const message of query({
  prompt: "Triage ticket 4821",
  options: {
    systemPrompt: [instructions, SYSTEM_PROMPT_DYNAMIC_BOUNDARY, ticketContext]
  }
})) {
  // ...
}
```

[Track cache tokens](/docs/pt/agent-sdk/cost-tracking#track-cache-tokens) descreve os campos `cache_creation_input_tokens` e `cache_read_input_tokens` em cada mensagem de resultado.

O SDK monta os blocos do array da seguinte forma:

* O SDK une as strings em cada lado do marcador com uma linha em branco entre elas e remove o marcador em si, então o texto do marcador não chega ao Claude.
* Se você incluir o marcador mais de uma vez, o primeiro é a divisão e o SDK remove os outros.
* Se você deixar o marcador de fora, o SDK une todas as strings em um bloco, o mesmo que passar uma string.

Com os flags [`--system-prompt` ou `--system-prompt-file`](/docs/pt/cli-reference#system-prompt-flags) da CLI, o prompt é uma string, então não há array para carregar o marcador. Inclua uma linha contendo apenas `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` entre as partes estática e por solicitação em vez disso. Claude Code divide o prompt na primeira linha assim em dois blocos e remove essa linha. Requer Claude Code v2.1.275 ou posterior.

No SDK, prefira o formulário de array, que carrega o limite sem uma linha de marcador.

<h3 id="change-the-prompt-of-an-existing-session">
  Alterar o prompt de uma sessão existente
</h3>

Por padrão, se você passar um `append` ou prompt personalizado diferente quando retornar a uma sessão com `resume` ou `continue`, Claude não o vê na próxima volta. Claude Code registra o prompt do sistema na primeira solicitação de uma sessão e reutiliza esse registro até que a sessão seja compactada. O novo texto entra em vigor após essa compactação, ou em uma nova sessão.

<h4 id="update-claude’s-instructions-mid-session">
  Atualizar as instruções do Claude no meio da sessão
</h4>

Se as instruções que você coloca no prompt do sistema precisarem mudar enquanto uma sessão está em execução, por exemplo porque seu usuário mudou o agente para um modo somente leitura ou editou sua configuração em seu aplicativo, envie as novas instruções na conversa em vez de alterar `systemPrompt`:

* **Na sua próxima mensagem**: inclua as novas instruções na próxima mensagem do usuário que você enviar.
* **De um hook**: retorne [`additionalContext`](/docs/pt/hooks#add-context-for-claude) de um callback de hook `UserPromptSubmit` ou `PostToolUse` [hook callback](/docs/pt/agent-sdk/hooks#outputs), escrito como uma declaração factual como "The workspace is now read-only". O SDK insere o texto na conversa no ponto onde o hook foi acionado, então o prompt registrado permanece inalterado.

<h4 id="turn-recording-off-while-you-iterate-on-wording">
  Desativar o registro enquanto você itera na redação
</h4>

Enquanto você itera na redação do prompt e quer que cada edição chegue a uma sessão que você retoma, defina `snapshot` como false no formulário de objeto do prompt do sistema. Claude Code então reconstrói o prompt em cada solicitação. O campo está disponível no formulário de predefinição e personalizado de [`systemPrompt`](/docs/pt/agent-sdk/typescript#options) em TypeScript e de [`system_prompt`](/docs/pt/agent-sdk/python#systempromptpreset) em Python, e requer `@anthropic-ai/claude-agent-sdk` v0.3.257 ou posterior, ou `claude-agent-sdk` v0.2.153 ou posterior.

Mantenha o registro ativado em produção. Com o registro desativado, um `append` ou prompt personalizado diferente em uma sessão retomada chega ao Claude na próxima volta, e essa solicitação não pode reutilizar o [prompt cache](/docs/pt/prompt-caching#how-the-cache-is-organized) da sessão. Onde a API impõe [preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking), Claude também perde seu pensamento das voltas anteriores.

Fora de [cloud sessions](/docs/pt/cloud-environments), se você iniciar Claude Code em [bare mode](/docs/pt/headless#start-faster-with-bare-mode) passando `--bare` através de `extraArgs` ou definindo `CLAUDE_CODE_SIMPLE=1`, o registro fica desativado a menos que você defina `snapshot: true`.

Registrar um `append` ou prompt personalizado por padrão requer Claude Code v2.1.265 ou posterior, que o TypeScript Agent SDK agrupa a partir de v0.3.265 e o Python Agent SDK a partir de v0.2.153. Antes de Claude Code v2.1.268, sessões que não [buscam feature flags](/docs/pt/env-vars#features-that-need-feature-flag-fetching), incluindo sessões em Amazon Bedrock, Google Cloud's Agent Platform, e Microsoft Foundry, reconstruíram o prompt em cada solicitação e `snapshot` não tinha efeito.

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

<h2 id="combine-approaches">
  Combinar abordagens
</h2>

As abordagens se compõem. Um estilo de saída persistente ou CLAUDE.md define o comportamento de longa duração, e `append` adiciona instruções específicas da sessão no topo sem tocar na configuração salva.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Combinar um estilo de saída com adições específicas da sessão
</h3>

O exemplo abaixo assume que um estilo de saída Code Reviewer já está ativo. O bloco `append` adiciona áreas de foco específicas da sessão no topo da persona, para que uma única sessão de revisão possa priorizar OAuth e armazenamento de tokens sem alterar o estilo de saída salvo:

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []


  async def main():
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


  asyncio.run(main())
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
