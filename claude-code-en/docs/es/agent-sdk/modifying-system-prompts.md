> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modificación de indicaciones del sistema

> Elija entre el preset `claude_code` y una indicación del sistema personalizada, y personalice el comportamiento con CLAUDE.md, estilos de salida, append, o una indicación completamente personalizada.

Las indicaciones del sistema definen el comportamiento, las capacidades y el estilo de respuesta de Claude. Comience con el preset `claude_code` para herramientas de codificación tipo CLI o IDE donde un humano observa y dirige el trabajo. Escriba su propia indicación para agentes con una superficie, identidad o modelo de permisos diferente.

Esta página cubre:

* [Cómo funcionan las indicaciones del sistema](#how-system-prompts-work), con una tabla de decisión para elegir entre el preset, el preset con `append`, y una indicación personalizada
* [Personalizar el comportamiento del agente](#customize-agent-behavior) con archivos CLAUDE.md, estilos de salida, `append`, o una cadena personalizada
* [Comparar los cuatro enfoques](#compare-the-four-approaches) por persistencia, alcance, y qué preservan
* [Combinar enfoques](#combine-approaches) para superponer métodos de personalización juntos

<h2 id="how-system-prompts-work">
  Cómo funcionan las indicaciones del sistema
</h2>

Una indicación del sistema es el conjunto inicial de instrucciones que forma cómo se comporta Claude durante toda una conversación. El SDK del Agente tiene tres puntos de partida para ella:

* **Predeterminado mínimo**: cuando no establece `systemPrompt` en TypeScript o `system_prompt` en Python, el SDK utiliza una indicación mínima que cubre la invocación de herramientas pero omite las directrices de codificación de Claude Code, el estilo de respuesta y el contexto del proyecto. Esto difiere de `claude -p`, que utiliza la indicación completa de Claude Code de forma predeterminada. Si está migrando desde la CLI y desea un comportamiento coincidente, establezca el preset `claude_code`.
* **Preset `claude_code`**: la indicación del sistema completa que utiliza la CLI de Claude Code, con instrucciones de uso de herramientas, directrices de estilo y formato de código, reglas de tono de respuesta y verbosidad, instrucciones de seguridad y protección, y contexto sobre el directorio de trabajo y el entorno. Establezca `systemPrompt: { type: "preset", preset: "claude_code" }` en TypeScript o `system_prompt={"type": "preset", "preset": "claude_code"}` en Python, opcionalmente con `append` para agregar sus propias instrucciones al final.
* **Cadena personalizada**: una indicación que usted escribe. El SDK envía solo lo que proporciona.

<h3 id="decide-on-a-starting-point">
  Decida sobre un punto de partida
</h3>

El factor decisivo es cuán estrechamente su agente se asemeja a Claude Code: un agente de codificación que opera en un repositorio, con un humano observando la salida en streaming y dirigiendo el trabajo. Cuanto más se aleje su producto de eso, más querrá escribir su propia indicación.

| Está construyendo                                                                                                                              | Utilice                            | Lo que obtiene                                                                                                                                                           |
| :--------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Una herramienta de codificación tipo CLI o IDE donde un humano observa y dirige, y los valores predeterminados de Claude Code son lo que desea | Preset `claude_code`               | La indicación completa de Claude Code: orientación de herramientas, reglas de seguridad, respuestas amigables con la terminal, conciencia de convenciones de repositorio |
| El mismo tipo de herramienta, más reglas específicas del producto como estándares de codificación, formato de salida o contexto de dominio     | Preset `claude_code` con `append`  | Todo lo anterior, con sus instrucciones agregadas después del preset. Nada se elimina, por lo que esta es la personalización de menor riesgo                             |
| Un agente con una superficie diferente, identidad o modelo de permisos, o un agente no codificador                                             | Cadena de indicación personalizada | Solo lo que escribe. Usted asume la responsabilidad de reemplazar la orientación de herramientas e instrucciones de seguridad que su agente aún necesita                 |
| Un bucle de invocación de herramientas delgado sin persona de agente, donde proporciona todo el comportamiento en la indicación del usuario    | Sin opción `systemPrompt`          | El predeterminado mínimo: soporte de invocación de herramientas y nada más                                                                                               |

"Diferente de Claude Code" generalmente significa uno de los siguientes:

* **Superficie diferente**: la salida no se lee en una terminal por la persona que la activó. Las interfaces de chat, los consumidores de salida estructurada y la automatización no codificadora cada una necesita una indicación que coincida con cómo se representa y revisa su salida. La automatización de codificación desatendida, como un trabajo de CI que corrige errores de lint o revisa diffs, aún se ajusta al preset porque el trabajo en sí es para lo que se escribió el preset.
* **Identidad diferente**: el agente no debe presentarse a sí mismo como Claude Code. Un bot de soporte, un asistente de análisis de datos, o cualquier agente específico del dominio necesita su propio nombre, alcance y persona.
* **Modelo de permisos diferente**: el agente se ejecuta de forma autónoma sin que un humano apruebe cada paso, u opera en un conjunto estrecho de recursos. La indicación de Claude Code asume que un humano está en el bucle con acceso a un conjunto completo de herramientas.
* **Tareas no codificadoras**: la mayoría de la indicación de Claude Code es orientación de codificación. Para agentes de investigación, contenido u operaciones, esa orientación compite con las instrucciones que realmente necesita.

La [tabla de comparación](#compare-the-four-approaches) muestra qué preserva cada método de personalización.

<h2 id="customize-agent-behavior">
  Personalizar el comportamiento del agente
</h2>

Los estilos de salida, `append`, y una cadena de indicación personalizada cada uno cambian la indicación del sistema directamente. CLAUDE.md toma un camino diferente: el SDK lo lee e inyecta su contenido en la conversación como contexto del proyecto, no en la indicación del sistema, por lo que forma el comportamiento junto con cualquier indicación del sistema que elija. [Skills](/docs/es/agent-sdk/skills), [hooks](/docs/es/agent-sdk/hooks), y [permissions](/docs/es/agent-sdk/permissions) también forman el comportamiento fuera de la indicación del sistema y se cubren en sus propias páginas.

<h3 id="claude-md-files-for-project-level-instructions">
  Archivos CLAUDE.md para instrucciones a nivel de proyecto
</h3>

Los archivos CLAUDE.md proporcionan a Claude contexto e instrucciones persistentes del proyecto. El SDK inyecta su contenido en la conversación, no en la indicación del sistema, por lo que funcionan con cualquier configuración de indicación del sistema. Para saber qué poner en CLAUDE.md, dónde colocarlo y cómo escribir instrucciones efectivas, consulte [Cómo Claude recuerda su proyecto](/docs/es/memory). Esta sección cubre lo específico del SDK: cómo se carga CLAUDE.md.

El SDK lee CLAUDE.md cuando la fuente de configuración coincidente está habilitada: `'project'` carga `CLAUDE.md` o `.claude/CLAUDE.md` desde el directorio de trabajo, y `'user'` carga `~/.claude/CLAUDE.md`. Las opciones predeterminadas de `query()` habilitan ambas fuentes, por lo que CLAUDE.md se carga automáticamente. Si establece `settingSources` en TypeScript o `setting_sources` en Python explícitamente, incluya las fuentes que necesita. La carga de CLAUDE.md se controla mediante fuentes de configuración, no por el preset `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Cargar CLAUDE.md con el SDK
</h4>

Para cargar CLAUDE.md, establezca `settingSources` para incluir el nivel donde vive su CLAUDE.md. El ejemplo a continuación carga un CLAUDE.md a nivel de proyecto junto con el preset `claude_code`, por lo que Claude tiene tanto la indicación completa del agente de codificación como las convenciones de su proyecto:

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

CLAUDE.md es persistente en todas las sesiones de un proyecto, se comparte con su equipo a través de git, y se descubre automáticamente sin cambios de código. No se carga si pasa un array `settingSources` vacío.

<h3 id="output-styles-for-persistent-configurations">
  Estilos de salida para configuraciones persistentes
</h3>

Los estilos de salida son configuraciones guardadas que modifican la indicación del sistema de Claude. Se almacenan como archivos markdown y se pueden reutilizar en sesiones y proyectos.

<h4 id="create-an-output-style">
  Crear un estilo de salida
</h4>

Un estilo de salida es un archivo markdown con [frontmatter](/docs/es/output-styles#frontmatter) para metadatos, seguido del contenido de la indicación. Guárdelo en `~/.claude/output-styles/` para un estilo a nivel de usuario disponible en cada proyecto, o `.claude/output-styles/` en su repositorio para un estilo a nivel de proyecto que pueda confirmar y compartir con su equipo.

De forma predeterminada, un estilo de salida personalizado reemplaza las instrucciones de ingeniería de software del preset `claude_code` con las suyas propias. Para mantenerlas y superponer sus instrucciones encima, establezca `keep-coding-instructions: true` en el frontmatter. Manténgalas cuando su agente aún esté realizando trabajo de ingeniería de software. Déjelas fuera cuando esté reemplazando el rol completamente.

El ejemplo a continuación define una persona de revisión de código que mantiene las instrucciones de codificación, ya que revisar código aún se beneficia de la orientación de seguridad y calidad de código de Claude Code. Guárdelo como `~/.claude/output-styles/code-reviewer.md` para hacerlo disponible en todos los proyectos:

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
  Activar un estilo de salida
</h4>

Una vez creado, active los estilos de salida a través de:

* **CLI**: ejecute `/config` y seleccione un estilo de salida
* **Configuración**: establezca `outputStyle` en `.claude/settings.local.json`
* **TypeScript SDK**: establezca `outputStyle` dentro del objeto `settings` en línea pasado a `query()`, o apunte `settings` a un archivo de configuración que lo establezca. `outputStyle` no es un campo `Options` de nivel superior:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

El SDK de Python no tiene una opción para seleccionar un estilo de salida mediante programación. Para implementaciones solo de código donde no puede escribir en `.claude/settings.local.json`, use `append` o una cadena de indicación personalizada en su lugar.

**Nota para usuarios del SDK:** Los estilos de salida se cargan cuando incluye `settingSources: ['user']` o `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` o `setting_sources=["project"]` (Python) en sus opciones.

<h3 id="append-to-the-claude_code-preset">
  Agregar al preset `claude_code`
</h3>

Puede usar el preset de Claude Code con una propiedad `append` para agregar sus instrucciones personalizadas mientras preserva toda la funcionalidad integrada.

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
  Mejorar el almacenamiento en caché de indicaciones entre usuarios y máquinas
</h4>

De forma predeterminada, dos sesiones que usan el mismo preset `claude_code` y texto `append` aún no pueden compartir una entrada de caché de indicación si se ejecutan desde diferentes directorios de trabajo. Esto se debe a que el preset incrusta contexto por sesión en la indicación del sistema antes de su texto `append`: el directorio de trabajo, si es un repositorio de git, la plataforma, el shell activo, la versión del SO, y rutas de memoria automática. Cualquier diferencia en ese contexto produce una indicación del sistema diferente y un error de caché. El contenido de CLAUDE.md no afecta el caché de indicación del sistema porque el SDK lo inyecta en la conversación, no en la indicación del sistema.

Para hacer que la indicación del sistema sea idéntica en todas las sesiones, establezca `excludeDynamicSections: true` en TypeScript o `"exclude_dynamic_sections": True` en Python. El contexto por sesión se mueve al primer mensaje del usuario, dejando solo el preset estático y su texto `append` en la indicación del sistema para que las configuraciones idénticas compartan una entrada de caché en usuarios y máquinas.

<Note>
  `excludeDynamicSections` requiere `@anthropic-ai/claude-agent-sdk` v0.2.98 o posterior, o `claude-agent-sdk` v0.1.58 o posterior para Python. Se aplica solo a la forma de objeto preset y no tiene efecto cuando `systemPrompt` es una cadena.
</Note>

El siguiente ejemplo empareja un bloque `append` compartido con `excludeDynamicSections` para que una flota de agentes que se ejecutan desde diferentes directorios pueda reutilizar la misma indicación del sistema almacenada en caché:

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

**Compensaciones:** el directorio de trabajo, la bandera de repositorio de git, la plataforma, el shell activo, la versión del SO, y rutas de memoria automática aún llegan a Claude, pero como parte del primer mensaje del usuario en lugar de la indicación del sistema. Las instrucciones en el mensaje del usuario tienen un peso marginalmente menor que el mismo texto en la indicación del sistema, por lo que Claude puede depender menos de ellas al razonar sobre el directorio actual o rutas de memoria automática. Habilite esta opción cuando la reutilización de caché entre sesiones sea más importante que el contexto de entorno máximamente autorizado.

Para la bandera equivalente en modo CLI no interactivo, consulte [`--exclude-dynamic-system-prompt-sections`](/docs/es/cli-reference).

<h3 id="custom-system-prompts">
  Indicaciones del sistema personalizadas
</h3>

Puede proporcionar una cadena personalizada como `systemPrompt` para reemplazar completamente la predeterminada con sus propias instrucciones.

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
  Comparación de los cuatro enfoques
</h2>

Los cuatro métodos de personalización difieren en dónde residen, cómo se comparten y qué preservan del preset `claude_code`.

| Característica                   | CLAUDE.md                 | Estilos de salida                    | `systemPrompt` con append | `systemPrompt` personalizado       |
| -------------------------------- | ------------------------- | ------------------------------------ | ------------------------- | ---------------------------------- |
| **Persistencia**                 | Archivo por proyecto      | Guardado como archivos               | Solo sesión               | Solo sesión                        |
| **Reutilización**                | Por proyecto              | Entre proyectos                      | Duplicación de código     | Duplicación de código              |
| **Gestión**                      | En el sistema de archivos | CLI + archivos                       | En código                 | En código                          |
| **Herramientas predeterminadas** | Preservadas               | Preservadas                          | Preservadas               | Perdidas (a menos que se incluyan) |
| **Seguridad integrada**          | Mantenida                 | Mantenida                            | Mantenida                 | Debe agregarse                     |
| **Contexto del entorno**         | Automático                | Automático                           | Automático                | Debe proporcionarse                |
| **Nivel de personalización**     | Solo adiciones            | Reemplazar o extender predeterminado | Solo adiciones            | Control completo                   |
| **Control de versiones**         | Con proyecto              | Sí                                   | Con código                | Con código                         |
| **Alcance**                      | Específico del proyecto   | Usuario o proyecto                   | Sesión de código          | Sesión de código                   |

"Con append" significa usar `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` en TypeScript o `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` en Python. CLAUDE.md no cambia el prompt del sistema en sí: el SDK inyecta su contenido en la conversación como contexto del proyecto.

<h2 id="use-cases-and-best-practices">
  Casos de uso y mejores prácticas
</h2>

<h3 id="when-to-use-claude-md">
  Cuándo usar CLAUDE.md
</h3>

Use CLAUDE.md para instrucciones que deben aplicarse a cada sesión en un proyecto, independientemente de qué indicación del sistema use la sesión: estándares de codificación, comandos comunes, contexto de arquitectura y convenciones del equipo. CLAUDE.md se confirma en su repositorio, por lo que se mantiene sincronizado con el código que describe. Consulte [Cuándo agregar a CLAUDE.md](/docs/es/memory#when-to-add-to-claude-md) para obtener orientación completa.

Los archivos CLAUDE.md se cargan cuando la fuente de configuración `project` está habilitada, que lo está para las opciones predeterminadas de `query()`. Si establece `settingSources` en TypeScript o `setting_sources` en Python explícitamente, incluya `'project'` para mantener la carga de CLAUDE.md a nivel de proyecto.

<h3 id="when-to-use-output-styles">
  Cuándo usar estilos de salida
</h3>

Los estilos de salida son para personas que desea reutilizar en la CLI y SDK sin cambiar el código de la aplicación. Debido a que viven como archivos en `.claude/output-styles`, la misma persona está disponible desde `/config` en la CLI y desde cualquier sesión de SDK que cargue la fuente de configuración coincidente.

**Mejor para:**

* Cambios de comportamiento persistentes en sesiones
* Configuraciones compartidas por el equipo
* Asistentes especializados como revisor de código, científico de datos o asistente de DevOps
* Modificaciones de indicaciones complejas que necesitan control de versiones

**Ejemplos:**

* Crear un asistente dedicado de optimización SQL
* Construir un revisor de código enfocado en seguridad
* Desarrollar un asistente de enseñanza con pedagogía específica

<h3 id="when-to-use-systemprompt-with-append">
  Cuándo usar `systemPrompt` con append
</h3>

Use `append` cuando el preset `claude_code` ya se ajusta a su producto y solo necesita agregar instrucciones adicionales. Mantiene la orientación de herramientas del preset, las reglas de seguridad y las convenciones de codificación sin reimplementarlas.

**Mejor para:**

* Agregar estándares de codificación o preferencias específicas
* Personalizar el formato de salida
* Agregar conocimiento específico del dominio
* Modificar la verbosidad de la respuesta
* Mejorar el comportamiento predeterminado de Claude Code sin perder instrucciones de herramientas

<h3 id="when-to-use-custom-systemprompt">
  Cuándo usar `systemPrompt` personalizado
</h3>

Use un indicador personalizado cuando la superficie, identidad o modelo de permisos de su agente difiera del de Claude Code, como se describe en [Decidir sobre un punto de partida](#decide-on-a-starting-point). Usted define el conjunto completo de instrucciones, incluida cualquier orientación de herramientas y reglas de seguridad que su agente necesite.

**Mejor para:**

* Control completo sobre el comportamiento de Claude
* Tareas especializadas de una sola sesión
* Prueba de nuevas estrategias de indicaciones
* Situaciones donde las herramientas predeterminadas no son necesarias
* Construcción de agentes especializados con comportamiento único

<h2 id="combine-approaches">
  Combinación de enfoques
</h2>

Estos métodos se componen. Un estilo de salida persistente o CLAUDE.md establece el comportamiento de larga duración, y `append` superpone instrucciones específicas de sesión sin tocar la configuración guardada.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Combinación de un estilo de salida con adiciones específicas de sesión
</h3>

El ejemplo a continuación asume que un estilo de salida Code Reviewer ya está activo. El bloque `append` superpone áreas de enfoque específicas de sesión sobre la persona, de modo que una única sesión de revisión puede priorizar OAuth y almacenamiento de tokens sin cambiar el estilo de salida guardado:

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
  Ver también
</h2>

* [Estilos de salida](/docs/es/output-styles): crear, gestionar y compartir estilos de salida para la CLI, incluido el formato de archivo y las ubicaciones de almacenamiento
* [Cómo Claude recuerda su proyecto](/docs/es/memory): qué poner en CLAUDE.md, dónde colocarlo y cómo escribir instrucciones de proyecto efectivas
* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript): el tipo `Options` completo, incluidos `systemPrompt`, `settingSources` y `settings`
* [Referencia del SDK de Python](/docs/es/agent-sdk/python): el tipo `ClaudeAgentOptions` completo, incluidos `system_prompt` y `setting_sources`
* [Configuración](/docs/es/settings): la referencia de `settings.json`, incluido dónde se almacenan los estilos de salida y otras configuraciones
