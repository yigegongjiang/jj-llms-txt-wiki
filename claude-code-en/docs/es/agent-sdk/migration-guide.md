> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrar a Claude Agent SDK

> Guía para migrar los SDK de TypeScript y Python de Claude Code al Claude Agent SDK

<h2 id="overview">
  Descripción general
</h2>

El Claude Code SDK ha sido renombrado a **Claude Agent SDK** y su documentación ha sido reorganizada. Este cambio refleja las capacidades más amplias del SDK para construir agentes de IA más allá de solo tareas de codificación.

¿Está migrando desde el OpenAI Agents SDK? La [receta de migración de OpenAI Agents SDK](https://platform.claude.com/cookbook/claude-agent-sdk-04-migrating-from-openai-agents-sdk) asigna cada primitiva al Claude Agent SDK a través de un único ejemplo trabajado.

<h2 id="what’s-changed">
  Qué ha cambiado
</h2>

| Aspecto                           | Anterior                    | Nuevo                                                                   |
| :-------------------------------- | :-------------------------- | :---------------------------------------------------------------------- |
| **Nombre del paquete (TS/JS)**    | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk`                                        |
| **Paquete de Python**             | `claude-code-sdk`           | `claude-agent-sdk`                                                      |
| **Ubicación de la documentación** | Claude Code docs            | Claude Code docs → sección dedicada [Agent SDK](/docs/es/agent-sdk/overview) |

<h2 id="migration-steps">
  Pasos de migración
</h2>

<h3 id="for-typescript/javascript-projects">
  Para proyectos de TypeScript/JavaScript
</h3>

**1. Desinstale el paquete antiguo:**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. Instale el nuevo paquete:**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. Actualice sus importaciones:**

Cambie todas las importaciones de `@anthropic-ai/claude-code` a `@anthropic-ai/claude-agent-sdk`:

```typescript theme={null}
// Antes
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// Después
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. Actualice package.json:**

Si `@anthropic-ai/claude-code` aún aparece en su `package.json`, reemplácelo con `@anthropic-ai/claude-agent-sdk` y actualice también el rango de versión, por ejemplo de `"^0.0.42"` a `"^0.3.0"`.

**5. Revise [cambios importantes](#breaking-changes)**

Realice los cambios de código necesarios para completar la migración.

<h3 id="for-python-projects">
  Para proyectos de Python
</h3>

**1. Desinstale el paquete antiguo:**

```bash theme={null}
pip uninstall -y claude-code-sdk
```

Si el paquete antiguo no está instalado, pip imprime `WARNING: Skipping claude-code-sdk as it is not installed.` Esto es esperado y puede continuar al siguiente paso.

**2. Instale el nuevo paquete:**

```bash theme={null}
pip install claude-agent-sdk
```

Si `claude-code-sdk` aparece en su `requirements.txt` o `pyproject.toml`, reemplácelo con `claude-agent-sdk`.

**3. Actualice sus importaciones:**

Cambie todas las importaciones de `claude_code_sdk` a `claude_agent_sdk`:

```python theme={null}
# Antes
from claude_code_sdk import query, ClaudeCodeOptions

# Después
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Revise [cambios importantes](#breaking-changes)**

Realice los cambios de código necesarios para completar la migración.

<h2 id="breaking-changes">
  Cambios importantes
</h2>

<Warning>
  Para mejorar el aislamiento y la configuración explícita, Claude Agent SDK v0.1.0 introduce cambios importantes para los usuarios que migran desde Claude Code SDK.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions renombrado a ClaudeAgentOptions
</h3>

**Qué cambió:** El tipo `ClaudeCodeOptions` del SDK de Python ha sido renombrado a `ClaudeAgentOptions`.

**Migración:**

```python theme={null}
# ANTES (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# DESPUÉS (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

<h3 id="system-prompt-no-longer-default">
  El prompt del sistema ya no es predeterminado
</h3>

**Qué cambió:** El SDK ya no utiliza el prompt del sistema de Claude Code de forma predeterminada.

**Migración:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // ANTES (v0.0.x) - Utilizaba el prompt del sistema de Claude Code de forma predeterminada
  const before = query({ prompt: "Hello" });

  // DESPUÉS (v0.1.0) - Utiliza un prompt del sistema mínimo de forma predeterminada
  // Para obtener el comportamiento anterior, solicite explícitamente el preset de Claude Code:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // O utilice un prompt del sistema personalizado:
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
      # ANTES (v0.0.x) - Utilizaba el prompt del sistema de Claude Code de forma predeterminada
      async for message in query(prompt="Hello"):
          print(message)

      # DESPUÉS (v0.1.0) - Utiliza un prompt del sistema mínimo de forma predeterminada
      # Para obtener el comportamiento anterior, solicite explícitamente el preset de Claude Code:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              system_prompt={"type": "preset", "preset": "claude_code"}  # Use the preset
          ),
      ):
          print(message)

      # O utilice un prompt del sistema personalizado:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="settings-sources-default">
  Predeterminado de fuentes de configuración
</h3>

Este predeterminado fue brevemente cambiado en v0.1.0 para no cargar ninguna configuración del sistema de archivos y luego fue revertido, por lo que no se necesita ninguna acción de migración.

**Comportamiento actual:** Omitir `settingSources` en `query()` carga la configuración del usuario, proyecto y sistema de archivos local, coincidiendo con la CLI. Esto incluye `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, archivos CLAUDE.md y comandos personalizados.

Para ejecutarse aislado de la configuración del sistema de archivos, pase `settingSources: []`, o `setting_sources=[]` en Python. Consulte [Control filesystem settings with settingSources](/docs/es/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) para ver qué carga cada fuente.

El aislamiento es especialmente importante para canalizaciones de CI/CD, aplicaciones implementadas, entornos de prueba y sistemas multiinquilino donde las personalizaciones locales no deben filtrarse.

<Note>
  Python SDK 0.1.59 y anteriores trataban una lista vacía igual que omitir la opción, así que actualice antes de confiar en `setting_sources=[]`. Consulte [What settingSources does not control](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control) para ver las entradas que se leen incluso cuando `settingSources` es `[]`.
</Note>

<h2 id="next-steps">
  Próximos pasos
</h2>

* Explore la [Descripción general de Agent SDK](/docs/es/agent-sdk/overview) para aprender sobre las características disponibles
* Consulte la [Referencia de SDK de TypeScript](/docs/es/agent-sdk/typescript) para documentación detallada de la API
* Revise la [Referencia de SDK de Python](/docs/es/agent-sdk/python) para documentación específica de Python
* Aprenda sobre [Herramientas personalizadas](/docs/es/agent-sdk/custom-tools) e [Integración MCP](/docs/es/agent-sdk/mcp)
