> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrar para Claude Agent SDK

> Guia para migrar os SDKs TypeScript e Python do Claude Code para o Claude Agent SDK

<h2 id="overview">
  Visão Geral
</h2>

O Claude Code SDK foi renomeado para o **Claude Agent SDK** e sua documentação foi reorganizada. Esta mudança reflete as capacidades mais amplas do SDK para construir agentes de IA além de apenas tarefas de codificação.

Migrando do OpenAI Agents SDK? A [receita de migração do OpenAI Agents SDK](https://platform.claude.com/cookbook/claude-agent-sdk-04-migrating-from-openai-agents-sdk) mapeia cada primitivo para o Claude Agent SDK através de um único exemplo prático.

<h2 id="what’s-changed">
  O Que Mudou
</h2>

| Aspecto                    | Antigo                      | Novo                                                                  |
| :------------------------- | :-------------------------- | :-------------------------------------------------------------------- |
| **Nome do Pacote (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk`                                      |
| **Pacote Python**          | `claude-code-sdk`           | `claude-agent-sdk`                                                    |
| **Local da Documentação**  | Claude Code docs            | Claude Code docs → seção dedicada [Agent SDK](/docs/pt/agent-sdk/overview) |

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

**4. Atualize package.json:**

Se `@anthropic-ai/claude-code` ainda estiver listado em seu `package.json`, substitua-o por `@anthropic-ai/claude-agent-sdk` e atualize também o intervalo de versão, por exemplo de `"^0.0.42"` para `"^0.3.0"`.

**5. Revise [mudanças significativas](#breaking-changes)**

Faça as alterações de código necessárias para concluir a migração.

<h3 id="for-python-projects">
  Para Projetos Python
</h3>

**1. Desinstale o pacote antigo:**

```bash theme={null}
pip uninstall -y claude-code-sdk
```

Se o pacote antigo não estiver instalado, pip imprime `WARNING: Skipping claude-code-sdk as it is not installed.` Isso é esperado e você pode continuar para a próxima etapa.

**2. Instale o novo pacote:**

```bash theme={null}
pip install claude-agent-sdk
```

Se `claude-code-sdk` estiver listado em seu `requirements.txt` ou `pyproject.toml`, substitua-o por `claude-agent-sdk`.

**3. Atualize suas importações:**

Altere todas as importações de `claude_code_sdk` para `claude_agent_sdk`:

```python theme={null}
# Antes
from claude_code_sdk import query, ClaudeCodeOptions

# Depois
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Revise [mudanças significativas](#breaking-changes)**

Faça as alterações de código necessárias para concluir a migração.

<h2 id="breaking-changes">
  Mudanças significativas
</h2>

<Warning>
  Para melhorar o isolamento e a configuração explícita, Claude Agent SDK v0.1.0 introduz mudanças significativas para usuários que migram do Claude Code SDK.
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

<h3 id="system-prompt-no-longer-default">
  System prompt não é mais padrão
</h3>

**O que mudou:** O SDK não usa mais o system prompt do Claude Code por padrão.

**Migração:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // ANTES (v0.0.x) - Usava o system prompt do Claude Code por padrão
  const before = query({ prompt: "Hello" });

  // DEPOIS (v0.1.0) - Usa um system prompt mínimo por padrão
  // Para obter o comportamento anterior, solicite explicitamente a predefinição do Claude Code:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Ou use um system prompt personalizado:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio


  async def main():
      # ANTES (v0.0.x) - Usava o system prompt do Claude Code por padrão
      async for message in query(prompt="Hello"):
          print(message)

      # DEPOIS (v0.1.0) - Usa um system prompt mínimo por padrão
      # Para obter o comportamento anterior, solicite explicitamente a predefinição do Claude Code:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              system_prompt={"type": "preset", "preset": "claude_code"}  # Use a predefinição
          ),
      ):
          print(message)

      # Ou use um system prompt personalizado:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="settings-sources-default">
  Padrão de fontes de configurações
</h3>

Este padrão foi brevemente alterado em v0.1.0 para não carregar configurações do sistema de arquivos e depois foi revertido, portanto nenhuma ação de migração é necessária.

**Comportamento atual:** Omitir `settingSources` em `query()` carrega as configurações do usuário, projeto e sistema de arquivos local, correspondendo à CLI. Isso inclui `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, arquivos CLAUDE.md e comandos personalizados.

Para executar isolado das configurações do sistema de arquivos, passe `settingSources: []`, ou `setting_sources=[]` em Python. Consulte [Control filesystem settings with settingSources](/docs/pt/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) para saber o que cada fonte carrega.

O isolamento é especialmente importante para pipelines de CI/CD, aplicações implantadas, ambientes de teste e sistemas multi-tenant, onde as personalizações locais não devem vazar.

<Note>
  Python SDK 0.1.59 e anteriores tratavam uma lista vazia da mesma forma que omitir a opção, portanto atualize antes de confiar em `setting_sources=[]`. Consulte [What settingSources does not control](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control) para entradas que são lidas mesmo quando `settingSources` é `[]`.
</Note>

<h2 id="next-steps">
  Próximas Etapas
</h2>

* Explore a [Visão Geral do Agent SDK](/docs/pt/agent-sdk/overview) para aprender sobre os recursos disponíveis
* Confira a [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript) para documentação detalhada da API
* Revise a [Referência do SDK Python](/docs/pt/agent-sdk/python) para documentação específica do Python
* Aprenda sobre [Ferramentas Personalizadas](/docs/pt/agent-sdk/custom-tools) e [Integração MCP](/docs/pt/agent-sdk/mcp)
