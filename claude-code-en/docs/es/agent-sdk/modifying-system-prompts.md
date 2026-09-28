> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modificación de indicaciones del sistema

> Elija entre el preset `claude_code` y una indicación del sistema personalizada, y personalice el comportamiento con CLAUDE.md, estilos de salida, append, o una indicación completamente personalizada.

Las indicaciones del sistema definen el comportamiento, las capacidades y el estilo de respuesta de Claude. Comience con el preset `claude_code` para herramientas de codificación tipo CLI o IDE donde un humano observa y dirige el trabajo. Escriba su propia indicación para agentes con una superficie, identidad o modelo de permisos diferente.

<h2 id="how-system-prompts-work">
  Cómo funcionan las indicaciones del sistema
</h2>

Una indicación del sistema es el conjunto inicial de instrucciones que forma cómo se comporta Claude durante toda una conversación. El SDK del Agente tiene tres puntos de partida para ella:

* **Predeterminado mínimo**: cuando no establece `systemPrompt` en TypeScript o `system_prompt` en Python, el SDK utiliza una indicación mínima que cubre la invocación de herramientas pero omite el resto del contenido del preset `claude_code`, incluidas sus instrucciones de seguridad y protección y su contexto sobre el directorio de trabajo y el entorno. Esto difiere de `claude -p`, que utiliza la indicación del sistema de Claude Code de forma predeterminada. Si está migrando desde la CLI y desea un comportamiento coincidente, establezca el preset `claude_code`.
* **Preset `claude_code`**: la indicación del sistema que utiliza la CLI de Claude Code, con instrucciones de uso de herramientas, instrucciones de seguridad y protección, y contexto sobre el directorio de trabajo y el entorno. Establezca `systemPrompt: { type: "preset", preset: "claude_code" }` en TypeScript o `system_prompt={"type": "preset", "preset": "claude_code"}` en Python, opcionalmente con `append` para agregar sus propias instrucciones al final.
* **Cadena personalizada**: una indicación que usted escribe. El SDK envía solo lo que proporciona.

<h3 id="decide-on-a-starting-point">
  Decida sobre un punto de partida
</h3>

El factor decisivo es cuán estrechamente su agente se asemeja a Claude Code: un agente de codificación que opera en un repositorio, con un humano observando la salida en streaming y dirigiendo el trabajo. Cuanto más se aleje su producto de eso, más querrá escribir su propia indicación.

| Está construyendo                                                                                                                              | Utilice                            | Lo que obtiene                                                                                                                                           |
| :--------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Una herramienta de codificación tipo CLI o IDE donde un humano observa y dirige, y los valores predeterminados de Claude Code son lo que desea | Preset `claude_code`               | La indicación de Claude Code, incluida la orientación de herramientas, reglas de seguridad y contexto del entorno                                        |
| El mismo tipo de herramienta, más reglas específicas del producto como estándares de codificación, formato de salida o contexto de dominio     | Preset `claude_code` con `append`  | Todo lo anterior, con sus instrucciones agregadas después del preset. Nada se elimina, por lo que esta es la personalización de menor riesgo             |
| Un agente con una superficie diferente, identidad o modelo de permisos, o un agente no codificador                                             | Cadena de indicación personalizada | Solo lo que escribe. Usted asume la responsabilidad de reemplazar la orientación de herramientas e instrucciones de seguridad que su agente aún necesita |
| Un bucle de invocación de herramientas delgado sin persona de agente, donde proporciona todo el comportamiento en la indicación del usuario    | Sin opción `systemPrompt`          | El predeterminado mínimo: soporte de invocación de herramientas y nada más                                                                               |

"Diferente de Claude Code" generalmente significa uno de los siguientes:

* **Superficie diferente**: la salida no se lee en una terminal por la persona que la activó. Las interfaces de chat, los consumidores de salida estructurada y la automatización no codificadora cada una necesita una indicación que coincida con cómo se representa y revisa su salida. La automatización de codificación desatendida, como un trabajo de CI que corrige errores de lint o revisa diffs, aún se ajusta al preset porque el trabajo en sí es para lo que se escribió el preset.
* **Identidad diferente**: el agente no debe presentarse a sí mismo como Claude Code. Un bot de soporte, un asistente de análisis de datos, o cualquier agente específico del dominio necesita su propio nombre, alcance y persona.
* **Modelo de permisos diferente**: el agente se ejecuta de forma autónoma sin que un humano apruebe cada paso, u opera en un conjunto estrecho de recursos. La indicación de Claude Code asume que un humano está en el bucle con acceso a un conjunto completo de herramientas.
* **Tareas no codificadoras**: la mayoría de la indicación de Claude Code es orientación de codificación. Para agentes de investigación, contenido u operaciones, esa orientación compite con las instrucciones que realmente necesita.

La [tabla de comparación](#compare-the-four-approaches) muestra qué preserva cada método de personalización.

<h2 id="customize-agent-behavior">
  Personalizar el comportamiento del agente
</h2>

`append` y una cadena de prompt personalizada cada uno cambian el prompt del sistema directamente, y un estilo de salida cambia las instrucciones que Claude Code le da a Claude para cada respuesta. CLAUDE.md toma un camino diferente: el SDK lo lee e inyecta su contenido en la conversación como contexto del proyecto, por lo que forma el comportamiento junto con cualquier prompt del sistema que elija. [Skills](/docs/es/agent-sdk/skills), [hooks](/docs/es/agent-sdk/hooks) y [permissions](/docs/es/agent-sdk/permissions) también forman el comportamiento fuera del prompt del sistema y se tratan en sus propias páginas.

<h3 id="claude-md-files-for-project-level-instructions">
  Archivos CLAUDE.md para instrucciones a nivel de proyecto
</h3>

Los archivos CLAUDE.md le dan a Claude contexto e instrucciones persistentes del proyecto. El SDK inyecta su contenido en la conversación y deja el prompt del sistema sin tocar, por lo que funcionan con cualquier configuración de prompt del sistema. Para saber qué poner en CLAUDE.md, dónde colocarlo y cómo escribir instrucciones efectivas, consulte [When to add to CLAUDE.md](/docs/es/memory#when-to-add-to-claude-md) y el resto de [How Claude remembers your project](/docs/es/memory). Esta sección cubre lo específico del SDK: cómo se carga CLAUDE.md.

El SDK lee CLAUDE.md cuando la fuente de configuración coincidente está habilitada: `'project'` carga `CLAUDE.md` o `.claude/CLAUDE.md` desde el directorio de trabajo, y `'user'` carga `~/.claude/CLAUDE.md`. Las opciones predeterminadas de `query()` habilitan ambas fuentes, por lo que CLAUDE.md se carga automáticamente. Si establece `settingSources` en TypeScript o `setting_sources` en Python explícitamente, incluya las fuentes que necesita. La carga de CLAUDE.md se controla mediante fuentes de configuración, no por el preset `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Cargar CLAUDE.md con el SDK
</h4>

Para cargar CLAUDE.md, establezca `settingSources` para incluir el nivel donde mantiene su CLAUDE.md. El ejemplo a continuación carga un CLAUDE.md a nivel de proyecto junto con el preset `claude_code`, por lo que Claude tiene tanto el prompt del agente de codificación como las convenciones de su proyecto:

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

Cuando ejecuta cualquiera de los ejemplos, el SDK transmite mensajes mientras Claude trabaja: un mensaje de inicialización del sistema, mensajes del asistente, mensajes del usuario que llevan resultados de herramientas y un mensaje de resultado final con el resultado de la sesión.

CLAUDE.md es persistente en todas las sesiones de un proyecto, se comparte con su equipo a través de git y se descubre automáticamente sin cambios de código. No se carga si pasa una matriz `settingSources` vacía.

<h3 id="output-styles-for-persistent-configurations">
  Estilos de salida para configuraciones persistentes
</h3>

Los estilos de salida son conjuntos guardados de instrucciones que cambian el rol, el tono y el formato de salida de Claude. Se almacenan como archivos markdown y se pueden reutilizar en sesiones y proyectos.

<h4 id="create-an-output-style">
  Crear un estilo de salida
</h4>

Un estilo de salida es un archivo markdown con [frontmatter](/docs/es/output-styles#frontmatter) para metadatos, seguido del contenido del prompt. Guárdelo en `~/.claude/output-styles/` para un estilo a nivel de usuario disponible en cada proyecto, o `.claude/output-styles/` en su repositorio para un estilo a nivel de proyecto que pueda confirmar y compartir con su equipo.

Un estilo de salida personalizado deja fuera las instrucciones de ingeniería de software del preset `claude_code` y usa las suyas propias. Para mantenerlas y superponer sus instrucciones encima, establezca `keep-coding-instructions: true` en el frontmatter. Esas instrucciones solo están en el prompt del sistema completo de Claude Code, por lo que la configuración no tiene efecto en una sesión en el prompt del sistema más corto, que activa o desactiva con [`CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT`](/docs/es/env-vars#variables). Manténgalas cuando su agente aún esté realizando trabajo de ingeniería de software. Déjelas fuera cuando esté reemplazando el rol completamente.

El ejemplo a continuación define una persona de revisión de código que mantiene las instrucciones de codificación, ya que revisar código aún se beneficia de la orientación de seguridad y calidad de código de Claude Code. Guárdelo como `~/.claude/output-styles/code-reviewer.md` para que esté disponible en todos los proyectos:

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

* **CLI**: ejecute `/output-style <style>`, por ejemplo `/output-style concise`, o ejecute `/config` y seleccione uno. El comando `/output-style` requiere Claude Code v2.1.269 o posterior.
* **Configuración**: establezca `outputStyle` en `.claude/settings.local.json`
* **TypeScript SDK**: establezca `outputStyle` dentro del objeto `settings` en línea pasado a `query()`, o apunte `settings` a un archivo de configuración que lo establezca. `outputStyle` no es un campo `Options` de nivel superior:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

En el SDK de Python, establezca `outputStyle` a través de la opción `settings`, que toma una cadena JSON como `'{"outputStyle": "Explanatory"}'` o una ruta a un archivo de configuración que lo establezca.

**Nota para usuarios del SDK:** Los estilos de salida se cargan cuando incluye `settingSources: ['user']` o `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` o `setting_sources=["project"]` (Python) en sus opciones.

<h3 id="append-to-the-claude_code-preset">
  Agregar al preset `claude_code`
</h3>

Puede usar el preset Claude Code con una propiedad `append` para agregar sus instrucciones personalizadas mientras preserva toda la funcionalidad integrada.

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
  Mejorar el almacenamiento en caché de prompts entre usuarios y máquinas
</h4>

De forma predeterminada, dos sesiones que usan el mismo preset `claude_code` y texto `append` aún no pueden compartir una entrada de caché de prompt si se ejecutan desde directorios de trabajo diferentes. Esto se debe a que el preset incrusta contexto por sesión en el prompt del sistema antes de su texto `append`: el directorio de trabajo, si es un repositorio de git, la plataforma, el shell activo, la versión del SO y las rutas de memoria automática. Cualquier diferencia en ese contexto produce un prompt del sistema diferente y una falta de caché. El contenido de CLAUDE.md no afecta el caché del prompt del sistema porque el SDK lo inyecta en la conversación, no en el prompt del sistema.

Para hacer que el prompt del sistema sea idéntico en todas las sesiones, establezca `excludeDynamicSections: true` en TypeScript o `"exclude_dynamic_sections": True` en Python. El contexto por sesión se mueve al primer mensaje del usuario, dejando solo el preset estático y su texto `append` en el prompt del sistema para que las configuraciones idénticas compartan una entrada de caché en usuarios y máquinas.

<Note>
  `excludeDynamicSections` requiere `@anthropic-ai/claude-agent-sdk` v0.2.98 o posterior, o `claude-agent-sdk` v0.1.58 o posterior para Python. Establézcalo solo en la forma de objeto preset. El SDK lo ignora cuando pasa un prompt personalizado en lugar del preset; para mantener las instrucciones de un prompt personalizado almacenadas en caché en el SDK de TypeScript, consulte [Cache the static part of a custom prompt](#cache-the-static-part-of-a-custom-prompt).
</Note>

El siguiente ejemplo empareja un bloque `append` compartido con `excludeDynamicSections` para que una flota de agentes que se ejecutan desde directorios diferentes puedan reutilizar el mismo prompt del sistema almacenado en caché:

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

**Compensaciones:** el directorio de trabajo, la bandera de repositorio de git, la plataforma, el shell activo, la versión del SO y las rutas de memoria automática aún llegan a Claude, pero como parte del primer mensaje del usuario en lugar del prompt del sistema. Las instrucciones en el mensaje del usuario tienen un peso marginalmente menor que el mismo texto en el prompt del sistema, por lo que Claude puede depender menos de ellas al razonar sobre el directorio actual o las rutas de memoria automática. Habilite esta opción cuando la reutilización de caché entre sesiones sea más importante que el contexto de entorno máximamente autorizado.

Para la bandera equivalente en modo CLI no interactivo, consulte [`--exclude-dynamic-system-prompt-sections`](/docs/es/cli-reference).

<h3 id="custom-system-prompts">
  Prompts del sistema personalizados
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

En Python, cargue un prompt personalizado grande desde un archivo con `system_prompt={"type": "file", "path": "..."}` en lugar de pasarlo como una cadena. El SDK de Python pasa un prompt de cadena como un argumento de línea de comandos al subproceso CLI, por lo que un prompt que excede el límite de longitud de argumento del SO falla en el spawn del proceso antes de que se envíe cualquier solicitud de API. En Linux el error es `Argument list too long`. Consulte [`SystemPromptFile`](/docs/es/agent-sdk/python#systempromptfile) para los umbrales de plataforma y el comportamiento de Windows.

<h4 id="cache-the-static-part-of-a-custom-prompt">
  Almacenar en caché la parte estática de un prompt personalizado
</h4>

En el SDK de TypeScript, puede pasar un prompt personalizado como una matriz de cadenas en lugar de una cadena, con el marcador `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` entre la parte estática y el resto. Úselo cuando su prompt combine instrucciones que son iguales en cada solicitud con contexto que cambia por solicitud, como el cliente o el ticket que maneja el agente. Cuando pasa ambas partes como una cadena, un cambio en la parte por solicitud cambia todo el prompt del sistema, por lo que las instrucciones estáticas también pierden el caché. La forma de matriz no está disponible en el SDK de Python; [`ClaudeAgentOptions`](/docs/es/agent-sdk/python#claudeagentoptions) enumera las formas que `system_prompt` acepta.

<Note>
  Claude Code divide el prompt solo cuando llama a la API de Claude directamente o se ejecuta en [Claude Platform on AWS](/docs/es/claude-platform-on-aws). En todas las demás configuraciones, como Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o una [LLM gateway](/docs/es/llm-gateway-connect), envía todo el prompt como un bloque, igual que pasar una cadena. Lo mismo sucede siempre que establezca [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/es/llm-gateway-protocol#disable-pre-release-capabilities).
</Note>

Para dividir el prompt, importe `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` de `@anthropic-ai/claude-agent-sdk` y páselo como su propio elemento de matriz entre las dos partes. El SDK envía las cadenas antes del marcador como un bloque de texto y las cadenas después como un segundo bloque, cada uno con su propio punto de ruptura de caché. En el ejemplo a continuación, un agente de soporte carga sus instrucciones de clasificación desde un archivo y recibe detalles sobre un ticket en cada solicitud, por lo que las instrucciones permanecen almacenadas en caché mientras los detalles del ticket cambian:

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

[Track cache tokens](/docs/es/agent-sdk/cost-tracking#track-cache-tokens) describe los campos `cache_creation_input_tokens` y `cache_read_input_tokens` en cada mensaje de resultado.

El SDK ensambla los bloques de la matriz de la siguiente manera:

* El SDK une las cadenas en cada lado del marcador con una línea en blanco entre ellas y elimina el marcador en sí, por lo que el texto del marcador no llega a Claude.
* Si incluye el marcador más de una vez, el primero es la división y el SDK elimina los demás.
* Si deja el marcador fuera, el SDK une todas las cadenas en un bloque, igual que pasar una cadena.

Con las banderas [`--system-prompt` o `--system-prompt-file`](/docs/es/cli-reference#system-prompt-flags) de la CLI, el prompt es una cadena, por lo que no hay matriz para llevar el marcador. Incluya una línea que contenga solo `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` entre las partes estáticas y por solicitud en su lugar. Claude Code divide el prompt en la primera línea de este tipo en los mismos dos bloques y elimina esa línea. Requiere Claude Code v2.1.275 o posterior.

En el SDK, prefiera la forma de matriz, que lleva el límite sin una línea de marcador.

<h3 id="change-the-prompt-of-an-existing-session">
  Cambiar el prompt de una sesión existente
</h3>

De forma predeterminada, si pasa un `append` o prompt personalizado diferente cuando regresa a una sesión con `resume` o `continue`, Claude no lo ve en el siguiente turno. Claude Code registra el prompt del sistema en la primera solicitud de una sesión y reutiliza ese registro hasta que la sesión se compacta. El nuevo texto entra en vigor después de esa compactación, o en una nueva sesión.

<h4 id="update-claude’s-instructions-mid-session">
  Actualizar las instrucciones de Claude a mitad de sesión
</h4>

Si las instrucciones que pone en el prompt del sistema necesitan cambiar mientras se ejecuta una sesión, por ejemplo porque su usuario cambió el agente a un modo de solo lectura o editó su configuración en su aplicación, envíe las nuevas instrucciones en la conversación en lugar de cambiar `systemPrompt`:

* **En su siguiente mensaje**: incluya las nuevas instrucciones en el siguiente mensaje del usuario que envíe.
* **Desde un hook**: devuelva [`additionalContext`](/docs/es/hooks#add-context-for-claude) de un callback de hook `UserPromptSubmit` o `PostToolUse` [hook callback](/docs/es/agent-sdk/hooks#outputs), escrito como una declaración fáctica como "The workspace is now read-only". El SDK inserta el texto en la conversación en el punto donde se activó el hook, por lo que el prompt registrado permanece sin cambios.

<h4 id="turn-recording-off-while-you-iterate-on-wording">
  Desactivar el registro mientras itera en la redacción
</h4>

Mientras itera en la redacción del prompt y desea que cada edición llegue a una sesión que reanuda, establezca `snapshot` en false en la forma de objeto del prompt del sistema. Claude Code luego reconstruye el prompt en cada solicitud. El campo está disponible en las formas preset y personalizada de [`systemPrompt`](/docs/es/agent-sdk/typescript#options) en TypeScript y de [`system_prompt`](/docs/es/agent-sdk/python#systempromptpreset) en Python, y requiere `@anthropic-ai/claude-agent-sdk` v0.3.257 o posterior, o `claude-agent-sdk` v0.2.153 o posterior.

Mantenga el registro activado en producción. Con el registro desactivado, un `append` o prompt personalizado diferente en una sesión reanudada llega a Claude en el siguiente turno, y esa solicitud no puede reutilizar el [prompt cache](/docs/es/prompt-caching#how-the-cache-is-organized) de la sesión. Donde la API impone [preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking), Claude también pierde su pensamiento de turnos anteriores.

Fuera de [cloud sessions](/docs/es/cloud-environments), si inicia Claude Code en [bare mode](/docs/es/headless#start-faster-with-bare-mode) pasando `--bare` a través de `extraArgs` o estableciendo `CLAUDE_CODE_SIMPLE=1`, el registro permanece desactivado a menos que establezca `snapshot: true`.

El registro de un `append` o prompt personalizado de forma predeterminada requiere Claude Code v2.1.265 o posterior, que el TypeScript Agent SDK agrupa desde v0.3.265 y el Python Agent SDK desde v0.2.153. Antes de Claude Code v2.1.268, las sesiones que no [fetch feature flags](/docs/es/env-vars#features-that-need-feature-flag-fetching), incluidas las sesiones en Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry, reconstruían el prompt en cada solicitud y `snapshot` no tenía efecto.

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

<h2 id="combine-approaches">
  Combinar enfoques
</h2>

Los enfoques se componen. Un estilo de salida persistente o CLAUDE.md establece el comportamiento de larga duración, y `append` superpone instrucciones específicas de la sesión sin tocar la configuración guardada.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Combinar un estilo de salida con adiciones específicas de la sesión
</h3>

El ejemplo a continuación asume que un estilo de salida Code Reviewer ya está activo. El bloque `append` superpone áreas de enfoque específicas de la sesión sobre la persona, de modo que una única sesión de revisión puede priorizar OAuth y almacenamiento de tokens sin cambiar el estilo de salida guardado:

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
  Ver también
</h2>

* [Estilos de salida](/docs/es/output-styles): crear, gestionar y compartir estilos de salida para la CLI, incluido el formato de archivo y las ubicaciones de almacenamiento
* [Cómo Claude recuerda su proyecto](/docs/es/memory): qué poner en CLAUDE.md, dónde colocarlo y cómo escribir instrucciones de proyecto efectivas
* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript): el tipo `Options` completo, incluidos `systemPrompt`, `settingSources` y `settings`
* [Referencia del SDK de Python](/docs/es/agent-sdk/python): el tipo `ClaudeAgentOptions` completo, incluidos `system_prompt` y `setting_sources`
* [Configuración](/docs/es/settings): la referencia de `settings.json`, incluido dónde se almacenan los estilos de salida y otras configuraciones
