> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrar para Claude Agent SDK

> Guia para migrar os SDKs TypeScript e Python do Claude Code para o Claude Agent SDK

<h2 id="overview">
  Visão Geral
</h2>

O Claude Code SDK foi renomeado para o **Claude Agent SDK** e sua documentação foi reorganizada. Esta mudança reflete as capacidades mais amplas do SDK para construir agentes de IA além de apenas tarefas de codificação.

<h2 id="what’s-changed">
  O Que Mudou
</h2>

| Aspecto                    | Antigo                      | Novo                             |
| :------------------------- | :-------------------------- | :------------------------------- |
| **Nome do Pacote (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk` |
| **Pacote Python**          | `claude-code-sdk`           | `claude-agent-sdk`               |
| **Local da Documentação**  | Documentação do Claude Code | API Guide → Seção Agent SDK      |

<Note>
  **Mudanças na Documentação:** A documentação do Agent SDK foi movida da documentação do Claude Code para o API Guide em uma seção dedicada [Agent SDK](/docs/pt/agent-sdk/overview). A documentação do Claude Code agora se concentra na ferramenta CLI e recursos de automação.
</Note>

<h2 id="migration-steps">
  Etapas de Migração
</h2>

<h3 id="for-typescript/javascript-projects">
  Para Projetos TypeScript/JavaScript
</h3>

**1. Desinstale o pacote antigo:**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. Instale o novo pacote:**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. Atualize suas importações:**

Altere todas as importações de `@anthropic-ai/claude-code` para `@anthropic-ai/claude-agent-sdk`:

```typescript theme={null}
// Antes
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// Depois
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. Atualize as dependências do package.json:**

Se você tiver o pacote listado em seu `package.json`, atualize-o:

Antes:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-code": "^0.0.42"
  }
}
```

Depois:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-agent-sdk": "^0.2.0"
  }
}
```

**5. Revise [mudanças significativas](#breaking-changes)**

Faça as alterações de código necessárias para concluir a migração.

<h3 id="for-python-projects">
  Para Projetos Python
</h3>

**1. Desinstale o pacote antigo:**

```bash theme={null}
pip uninstall claude-code-sdk
```

**2. Instale o novo pacote:**

```bash theme={null}
pip install claude-agent-sdk
```

**3. Atualize suas importações:**

Altere todas as importações de `claude_code_sdk` para `claude_agent_sdk`:

```python theme={null}
# Antes
from claude_code_sdk import query, ClaudeCodeOptions

# Depois
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Atualize os nomes dos tipos:**

Altere `ClaudeCodeOptions` para `ClaudeAgentOptions`:

```python theme={null}
# Antes
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7")

# Depois
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7")
```

**5. Revise [mudanças significativas](#breaking-changes)**

Faça as alterações de código necessárias para concluir a migração.

<h2 id="breaking-changes">
  Mudanças significativas
</h2>

<Warning>
  Para melhorar o isolamento e a configuração explícita, o Claude Agent SDK v0.1.0 introduz mudanças significativas para usuários que migram do Claude Code SDK. Revise esta seção cuidadosamente antes de migrar.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions renomeado para ClaudeAgentOptions
</h3>

**O que mudou:** O tipo `ClaudeCodeOptions` do SDK Python foi renomeado para `ClaudeAgentOptions`.

**Migração:**

```python theme={null}
# ANTES (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# DEPOIS (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

**Por que isso mudou:** O nome do tipo agora corresponde à marca "Claude Agent SDK" e fornece consistência nas convenções de nomenclatura do SDK.

<h3 id="system-prompt-no-longer-default">
  Prompt do sistema não é mais padrão
</h3>

**O que mudou:** O SDK não usa mais o prompt do sistema do Claude Code por padrão.

**Migração:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // ANTES (v0.0.x) - Usava o prompt do sistema do Claude Code por padrão
  const before = query({ prompt: "Hello" });

  // DEPOIS (v0.1.0) - Usa prompt do sistema mínimo por padrão
  // Para obter o comportamento antigo, solicite explicitamente a predefinição do Claude Code:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Ou use um prompt do sistema personalizado:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  # ANTES (v0.0.x) - Usava o prompt do sistema do Claude Code por padrão
  async for message in query(prompt="Hello"):
      print(message)

  # DEPOIS (v0.1.0) - Usa prompt do sistema mínimo por padrão
  # Para obter o comportamento antigo, solicite explicitamente a predefinição do Claude Code:
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          system_prompt={"type": "preset", "preset": "claude_code"}  # Use a predefinição
      ),
  ):
      print(message)

  # Ou use um prompt do sistema personalizado:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
  ):
      print(message)
  ```
</CodeGroup>

**Por que isso mudou:** Fornece melhor controle e isolamento para aplicações SDK. Você agora pode construir agentes com comportamento personalizado sem herdar as instruções focadas em CLI do Claude Code.

<h3 id="settings-sources-default">
  Padrão de fontes de configurações
</h3>

Este padrão foi brevemente alterado em v0.1.0 e depois revertido, portanto nenhuma ação de migração é necessária.

**Comportamento atual:** Omitir `settingSources` em `query()` carrega configurações de usuário, projeto e sistema de arquivos local, correspondendo ao CLI. Isso inclui `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, arquivos CLAUDE.md e comandos personalizados.

Para executar isolado das configurações do sistema de arquivos, passe uma matriz vazia:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const isolatedResult = query({
    prompt: "Hello",
    options: {
      settingSources: [] // Nenhuma configuração do sistema de arquivos carregada
    }
  });

  // Ou carregue apenas fontes específicas:
  const projectOnlyResult = query({
    prompt: "Hello",
    options: {
      settingSources: ["project"] // Apenas configurações do projeto
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(setting_sources=[]),  # Nenhuma configuração do sistema de arquivos carregada
  ):
      print(message)

  # Ou carregue apenas fontes específicas:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          setting_sources=["project"]  # Apenas configurações do projeto
      ),
  ):
      print(message)
  ```
</CodeGroup>

O isolamento é especialmente importante para pipelines CI/CD, aplicações implantadas, ambientes de teste e sistemas multi-tenant onde personalizações locais não devem vazar.

<Note>
  O SDK v0.1.0 brevemente padronizou para nenhuma configuração carregada; isso foi revertido em versões subsequentes. Python SDK 0.1.59 e anteriores tratavam uma lista vazia da mesma forma que omitir a opção, portanto atualize antes de confiar em `setting_sources=[]`. Veja [O que settingSources não controla](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control) para entradas que são lidas mesmo quando `settingSources` é `[]`.
</Note>

<h2 id="why-the-rename">
  Por Que a Renomeação?
</h2>

O Claude Code SDK foi originalmente projetado para tarefas de codificação, mas evoluiu para um framework poderoso para construir todos os tipos de agentes de IA. O novo nome "Claude Agent SDK" reflete melhor suas capacidades:

* Construir agentes de negócios (assistentes jurídicos, consultores financeiros, suporte ao cliente)
* Criar agentes de codificação especializados (bots SRE, revisores de segurança, agentes de revisão de código)
* Desenvolver agentes personalizados para qualquer domínio com uso de ferramentas, integração MCP e muito mais

<h2 id="getting-help">
  Obtendo Ajuda
</h2>

Se você encontrar algum problema durante a migração:

**Para TypeScript/JavaScript:**

1. Verifique se todas as importações foram atualizadas para usar `@anthropic-ai/claude-agent-sdk`
2. Verifique se seu package.json tem o novo nome do pacote
3. Execute `npm install` para garantir que as dependências sejam atualizadas

**Para Python:**

1. Verifique se todas as importações foram atualizadas para usar `claude_agent_sdk`
2. Verifique se seu requirements.txt ou pyproject.toml tem o novo nome do pacote
3. Execute `pip install claude-agent-sdk` para garantir que o pacote seja instalado

<h2 id="next-steps">
  Próximas Etapas
</h2>

* Explore a [Visão Geral do Agent SDK](/docs/pt/agent-sdk/overview) para aprender sobre os recursos disponíveis
* Confira a [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript) para documentação detalhada da API
* Revise a [Referência do SDK Python](/docs/pt/agent-sdk/python) para documentação específica do Python
* Aprenda sobre [Ferramentas Personalizadas](/docs/pt/agent-sdk/custom-tools) e [Integração MCP](/docs/pt/agent-sdk/mcp)
