> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrar a Claude Agent SDK

> Guía para migrar los SDK de TypeScript y Python de Claude Code al Claude Agent SDK

<h2 id="overview">
  Descripción general
</h2>

El SDK de Claude Code ha sido renombrado a **Claude Agent SDK** y su documentación ha sido reorganizada. Este cambio refleja las capacidades más amplias del SDK para construir agentes de IA más allá de solo tareas de codificación.

<h2 id="what’s-changed">
  Qué ha cambiado
</h2>

| Aspecto                           | Anterior                     | Nuevo                            |
| :-------------------------------- | :--------------------------- | :------------------------------- |
| **Nombre del paquete (TS/JS)**    | `@anthropic-ai/claude-code`  | `@anthropic-ai/claude-agent-sdk` |
| **Paquete de Python**             | `claude-code-sdk`            | `claude-agent-sdk`               |
| **Ubicación de la documentación** | Documentación de Claude Code | API Guide → Sección Agent SDK    |

<Note>
  **Cambios en la documentación:** La documentación de Agent SDK se ha trasladado de la documentación de Claude Code a la Guía de API bajo una sección dedicada [Agent SDK](/docs/es/agent-sdk/overview). La documentación de Claude Code ahora se enfoca en la herramienta CLI y características de automatización.
</Note>

<h2 id="migration-steps">
  Pasos de migración
</h2>

<h3 id="for-typescript/javascript-projects">
  Para proyectos de TypeScript/JavaScript
</h3>

**1. Desinstale el paquete anterior:**

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

**4. Actualice las dependencias de package.json:**

Si tiene el paquete listado en su `package.json`, actualícelo:

Antes:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-code": "^0.0.42"
  }
}
```

Después:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-agent-sdk": "^0.2.0"
  }
}
```

**5. Revise [cambios importantes](#breaking-changes)**

Realice los cambios de código necesarios para completar la migración.

<h3 id="for-python-projects">
  Para proyectos de Python
</h3>

**1. Desinstale el paquete anterior:**

```bash theme={null}
pip uninstall claude-code-sdk
```

**2. Instale el nuevo paquete:**

```bash theme={null}
pip install claude-agent-sdk
```

**3. Actualice sus importaciones:**

Cambie todas las importaciones de `claude_code_sdk` a `claude_agent_sdk`:

```python theme={null}
# Antes
from claude_code_sdk import query, ClaudeCodeOptions

# Después
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Actualice los nombres de tipos:**

Cambie `ClaudeCodeOptions` a `ClaudeAgentOptions`:

```python theme={null}
# Antes
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7")

# Después
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7")
```

**5. Revise [cambios importantes](#breaking-changes)**

Realice los cambios de código necesarios para completar la migración.

<h2 id="breaking-changes">
  Cambios importantes
</h2>

<Warning>
  Para mejorar el aislamiento y la configuración explícita, Claude Agent SDK v0.1.0 introduce cambios importantes para los usuarios que migran desde Claude Code SDK. Revise esta sección cuidadosamente antes de migrar.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions renombrado a ClaudeAgentOptions
</h3>

**Qué cambió:** El tipo de SDK de Python `ClaudeCodeOptions` ha sido renombrado a `ClaudeAgentOptions`.

**Migración:**

```python theme={null}
# ANTES (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# DESPUÉS (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

**Por qué cambió:** El nombre del tipo ahora coincide con la marca "Claude Agent SDK" y proporciona consistencia en las convenciones de nomenclatura del SDK.

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

  // O use un prompt del sistema personalizado:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  # ANTES (v0.0.x) - Utilizaba el prompt del sistema de Claude Code de forma predeterminada
  async for message in query(prompt="Hello"):
      print(message)

  # DESPUÉS (v0.1.0) - Utiliza un prompt del sistema mínimo de forma predeterminada
  # Para obtener el comportamiento anterior, solicite explícitamente el preset de Claude Code:
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          system_prompt={"type": "preset", "preset": "claude_code"}  # Utilice el preset
      ),
  ):
      print(message)

  # O use un prompt del sistema personalizado:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
  ):
      print(message)
  ```
</CodeGroup>

**Por qué cambió:** Proporciona mejor control y aislamiento para aplicaciones SDK. Ahora puede construir agentes con comportamiento personalizado sin heredar las instrucciones enfocadas en CLI de Claude Code.

<h3 id="settings-sources-default">
  Predeterminado de fuentes de configuración
</h3>

Este predeterminado fue brevemente cambiado en v0.1.0 y luego revertido, por lo que no se requiere acción de migración.

**Comportamiento actual:** Omitir `settingSources` en `query()` carga la configuración del usuario, proyecto y sistema de archivos local, coincidiendo con la CLI. Esto incluye `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, archivos CLAUDE.md y comandos personalizados.

Para ejecutar aislado de la configuración del sistema de archivos, pase una matriz vacía:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const isolatedResult = query({
    prompt: "Hello",
    options: {
      settingSources: [] // No se carga configuración del sistema de archivos
    }
  });

  // O cargue solo fuentes específicas:
  const projectOnlyResult = query({
    prompt: "Hello",
    options: {
      settingSources: ["project"] // Solo configuración del proyecto
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(setting_sources=[]),  # No se carga configuración del sistema de archivos
  ):
      print(message)

  # O cargue solo fuentes específicas:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          setting_sources=["project"]  # Solo configuración del proyecto
      ),
  ):
      print(message)
  ```
</CodeGroup>

El aislamiento es especialmente importante para canalizaciones CI/CD, aplicaciones implementadas, entornos de prueba y sistemas multiinquilino donde las personalizaciones locales no deben filtrarse.

<Note>
  SDK v0.1.0 brevemente predeterminó a ninguna configuración cargada; esto fue revertido en versiones posteriores. Python SDK 0.1.59 y anteriores trataban una lista vacía igual que omitir la opción, así que actualice antes de confiar en `setting_sources=[]`. Vea [Lo que settingSources no controla](/docs/es/agent-sdk/claude-code-features#what-settingsources-does-not-control) para entradas que se leen incluso cuando `settingSources` es `[]`.
</Note>

<h2 id="why-the-rename">
  ¿Por qué el cambio de nombre?
</h2>

El SDK de Claude Code fue diseñado originalmente para tareas de codificación, pero ha evolucionado hacia un marco poderoso para construir todo tipo de agentes de IA. El nuevo nombre "Claude Agent SDK" refleja mejor sus capacidades:

* Construir agentes empresariales (asistentes legales, asesores financieros, soporte al cliente)
* Crear agentes de codificación especializados (bots SRE, revisores de seguridad, agentes de revisión de código)
* Desarrollar agentes personalizados para cualquier dominio con uso de herramientas, integración MCP y más

<h2 id="getting-help">
  Obtener ayuda
</h2>

Si encuentra algún problema durante la migración:

**Para TypeScript/JavaScript:**

1. Verifique que todas las importaciones se actualicen para usar `@anthropic-ai/claude-agent-sdk`
2. Verifique que su package.json tenga el nuevo nombre de paquete
3. Ejecute `npm install` para asegurar que las dependencias se actualicen

**Para Python:**

1. Verifique que todas las importaciones se actualicen para usar `claude_agent_sdk`
2. Verifique que su requirements.txt o pyproject.toml tenga el nuevo nombre de paquete
3. Ejecute `pip install claude-agent-sdk` para asegurar que el paquete esté instalado

<h2 id="next-steps">
  Próximos pasos
</h2>

* Explore la [Descripción general de Agent SDK](/docs/es/agent-sdk/overview) para aprender sobre las características disponibles
* Consulte la [Referencia de SDK de TypeScript](/docs/es/agent-sdk/typescript) para documentación detallada de la API
* Revise la [Referencia de SDK de Python](/docs/es/agent-sdk/python) para documentación específica de Python
* Aprenda sobre [Herramientas personalizadas](/docs/es/agent-sdk/custom-tools) e [Integración MCP](/docs/es/agent-sdk/mcp)
