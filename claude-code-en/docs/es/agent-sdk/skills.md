> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Extienda agentes con skills

> Controle qué skills puede invocar Claude en sesiones del Claude Agent SDK, despache comandos por nombre y cree skills que sus sesiones descubran

Agent Skills extienden Claude con capacidades especializadas que Claude invoca cuando es relevante. Las Skills se empaquetan como archivos `SKILL.md` que contienen instrucciones, descripciones y recursos de apoyo opcionales. Esta página también cubre [comandos en sesiones del Agent SDK](#commands-in-agent-sdk-sessions).

Para obtener información completa sobre skills, incluidos beneficios, arquitectura y directrices de autoría, consulte la [descripción general de Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-agent-sdk">
  Cómo funcionan las skills con el Agent SDK
</h2>

Cuando se utiliza el Claude Agent SDK, las skills son:

* **Definidas como artefactos del sistema de archivos**: crea cada skill como un archivo `SKILL.md` en su propio directorio, como `.claude/skills/<name>/SKILL.md`
* **Cargadas desde el sistema de archivos**: el SDK carga skills desde ubicaciones del sistema de archivos gobernadas por `settingSources` (TypeScript) o `setting_sources` (Python)
* **Descubiertas automáticamente**: una vez que se cargan las configuraciones del sistema de archivos, el SDK descubre metadatos de skills al inicio desde directorios de usuario y proyecto, y carga el contenido completo cuando Claude invoca la skill
* **Invocadas por el modelo**: Claude elige autónomamente cuándo usarlas según el contexto
* **Invocadas por el usuario**: despache una skill directamente enviando `/<name>` en un prompt. Consulte [Comandos en sesiones del Agent SDK](#commands-in-agent-sdk-sessions)
* **Limitadas a través de la opción `skills`**: las skills descubiertas están habilitadas de forma predeterminada. Pase una lista de nombres de skills, `"all"`, o `[]` para controlar qué skills puede invocar Claude

A diferencia de los subagentes, que puede definir en la [opción `agents`](/docs/es/agent-sdk/subagents#programmatic-definition-recommended), crea skills como archivos en disco. El SDK no proporciona una API programática para registrarlas.

<Note>
  Las skills se descubren a través de las fuentes de configuración del sistema de archivos. Con opciones predeterminadas de `query()`, el SDK carga fuentes de usuario y proyecto, por lo que las skills en `~/.claude/skills/`, `<cwd>/.claude/skills/`, y `.claude/skills/` en cualquier directorio padre de `<cwd>` hasta la raíz del repositorio están disponibles. La fuente del proyecto también cubre `<dir>/.claude/skills/` en cada directorio que pase a través de `additionalDirectories` (TypeScript) o `add_dirs` (Python), porque el SDK pasa esos directorios a Claude Code como [`--add-dir`](/docs/es/skills#skills-from-additional-directories). Si establece `settingSources` explícitamente, incluya `'project'` para mantener las skills de proyecto y directorio agregado e `'user'` para mantener sus skills personales, o use la [opción `plugins`](/docs/es/agent-sdk/plugins) para cargar skills desde una ruta específica.
</Note>

<h2 id="use-skills-with-the-agent-sdk">
  Use skills con el Agent SDK
</h2>

Establezca la opción `skills` en `query()` para controlar qué skills puede invocar Claude en la sesión. Cuando se omite, las skills descubiertas están habilitadas y la herramienta Skill está disponible, coincidiendo con el comportamiento de CLI. Pase `"all"` para permitir que Claude invoque cada skill descubierta, una lista de nombres de skills para permitir solo esos, o `[]` para que Claude no invoque ninguno.

Por ejemplo, para permitir que Claude invoque solo dos skills nombradas:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

<h3 id="set-up-skills-in-a-session">
  Configure skills en una sesión
</h3>

Cuando establece `skills`, el SDK añade la herramienta Skill a `allowedTools` automáticamente. Si también pasa una lista explícita de `tools`, incluya `"Skill"` en esa lista para que Claude pueda invocar skills.

Una vez configurado, Claude descubre automáticamente skills desde el sistema de archivos e invoca las que son relevantes para la solicitud del usuario.

El siguiente ejemplo habilita cada skill descubierta en una sesión y preaprueba las herramientas que las skills comúnmente necesitan. El ejemplo establece `cwd` al directorio de trabajo actual del proceso, así que ejecútelo desde dentro de un proyecto que tenga un directorio `.claude/skills/` en el directorio actual o cualquier padre hasta la raíz del repositorio:

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
  Confirme que las skills se cargaron
</h3>

Cerca del inicio del stream, el SDK produce un mensaje del sistema con subtipo `init`. Verifique su matriz `skills` para confirmar que sus skills se cargaron antes de que Claude comience a trabajar. La matriz incluye las skills invocables por el usuario que ha definido con un campo frontmatter `description` o `when_to_use`, junto con [skills incluidas en Claude Code](/docs/es/skills#bundled-skills).

La matriz lista solo skills invocables por el usuario. Una skill con [`user-invocable: false`](/docs/es/skills#control-who-invokes-a-skill) en su frontmatter se carga y permanece disponible para Claude, pero no aparece en la matriz. La matriz lista las mismas skills independientemente de si están en su lista `skills`.

<h3 id="allow-only-specific-skills">
  Permita solo skills específicas
</h3>

Para permitir que Claude invoque solo skills específicas, pase sus nombres en la lista `skills`. Los nombres coinciden con el campo `name` en `SKILL.md` o el nombre del directorio de la skill. Use `plugin:skill` para skills proporcionadas por plugins.

La lista toma solo nombres exactos de skills. Si una entrada no puede funcionar como un nombre exacto, `query()` rechaza la lista antes de que comience la sesión. Consulte [Error de nombre de skill inválido](#invalid-skill-name-error) para las reglas de nombre y el error que cada SDK genera.

El modelo no ve skills no listadas y la herramienta Skill las rechaza, mientras que sus archivos permanecen en disco y permanecen accesibles a través de Read y Bash. Restringir la lista no restringe [despacho por nombre](#dispatch-commands-by-name).

Para permitir que Claude invoque cada skill descubierta, pase `skills: "all"` en lugar de un comodín.

<h2 id="commands-in-agent-sdk-sessions">
  Comandos en sesiones del Agent SDK
</h2>

Esta sección es la documentación de comandos del SDK. Un comando es cualquier cosa que ejecute enviando `/<name>` en un prompt. Las entradas en la superficie del comando difieren en lo que las respalda:

* **Comandos integrados**: ejecutan lógica codificada en el proceso de Claude Code que ejecuta el SDK, por ejemplo `/compact`
* **Skills incluidas**: artefactos de prompt incluidos con Claude Code, por ejemplo `/code-review`
* **Sus skills**: artefactos de prompt que crea, cada uno un directorio que contiene un archivo `SKILL.md`. El nombre de una skill invocable por el usuario se une a la superficie automáticamente, así que despachar su propio `/security-check` y ejecutar uno integrado funcionan de la misma manera
* **Archivos de comando personalizados**: una forma de artefacto más antigua con el mismo comportamiento, archivos Markdown planos en `.claude/commands/` cuyos nombres de archivo se convierten en nombres de comando. Las skills son su sucesor recomendado

De forma predeterminada, tanto usted como Claude pueden invocar cualquier skill. Puede restringir cualquiera de las dos rutas a través del [frontmatter](/docs/es/skills#control-who-invokes-a-skill) de la skill. Para definiciones de comando y skill, consulte las entradas [Comando](/docs/es/glossary#command) y [Skill](/docs/es/glossary#skill) del glosario. Consulte [Comandos en Claude Code](/docs/es/commands) para cada uno integrado y [Extienda Claude con skills](/docs/es/skills) para la guía completa de ambas formas de artefacto.

<h3 id="discover-available-commands">
  Descubra comandos disponibles
</h3>

Puede despachar comandos que funcionan sin una terminal interactiva a través del SDK. El mensaje `system/init` lista los disponibles en su sesión en su campo `slash_commands`. Los comandos que necesitan una terminal interactiva, como `/theme` y `/terminal-setup`, no aparecen en la lista. Acceda al campo cuando su sesión comience:

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

La lista impresa mezcla comandos integrados, skills incluidas, sus skills invocables por el usuario y archivos `.claude/commands/`:

```text theme={null}
Available commands: ["clear", "compact", "context", "usage", "code-review", "verify", "security-check", ...]
```

Una skill con [`user-invocable: false`](/docs/es/skills#control-who-invokes-a-skill) en su frontmatter no aparece en esta lista ni en la matriz `skills` de [Confirme que las skills se cargaron](#confirm-skills-loaded). Las sesiones que configuran [servidores MCP](/docs/es/agent-sdk/mcp) también pueden exponer [prompts MCP como comandos](/docs/es/mcp#use-mcp-prompts-as-commands).

<h3 id="dispatch-commands-by-name">
  Despache comandos por nombre
</h3>

Envíe un comando incluyéndolo en su cadena de prompt, de la misma manera que envía texto regular. El despacho no depende de la opción `skills`. Enviar `/<name>` ejecuta una skill invocable por el usuario incluso cuando su lista `skills` la omite. Los comandos que actúan en el historial de conversación, como `/compact`, necesitan mensajes previos para trabajar.

Un `/<name>` que no coincida ni con un comando en la sesión ni con un comando integrado de Claude Code no falla la consulta. Claude Code envía el prompt a Claude como un mensaje ordinario, con una nota de que el comando no se ejecutó, así que la consulta gasta un turno de modelo y devuelve la respuesta de Claude. Antes de v2.1.274, un `/<name>` que no coincidía con nada devolvía `Unknown command: /<name>` como resultado sin un turno de modelo.

Un `/<name>` que coincida con un comando integrado de Claude Code que no está disponible en la sesión, como `/theme`, devuelve `/theme isn't available in this environment.` como resultado sin un turno de modelo.

<Note>
  Un comando puede alcanzar el límite `maxTurns` / `max_turns` como cualquier otro prompt, terminando la consulta con un resultado de error en lugar de `success`. Para el contrato de resultado de error, consulte [Maneje el resultado](/docs/es/agent-sdk/agent-loop#handle-the-result). Si su comando podría alcanzar el límite, envuelva el bucle en un `try`/`catch` en TypeScript o `try`/`except` en Python, como se muestra en [Entrada de mensaje único](/docs/es/agent-sdk/streaming-vs-single-mode#single-message-input), o establezca `maxTurns` lo suficientemente alto para que el trabajo se complete.
</Note>

<h3 id="compact-history-with-/compact">
  Compacte el historial con `/compact`
</h3>

El comando `/compact` reduce el tamaño de su historial de conversación resumiendo mensajes más antiguos mientras preserva contexto importante. La compactación necesita una conversación existente con suficientes mensajes previos para resumir. Este ejemplo tiene una conversación primero, luego la compacta y lee el mensaje del sistema `compact_boundary` que reporta el resultado:

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
  Un mensaje `compact_boundary` solo llega cuando se ejecutó la compactación. Sin nada que resumir, `/compact` reporta la razón en lugar de generar un error. La ejecución aún termina con un resultado `success` y sin mensaje `compact_boundary`, y el texto del resultado lleva la razón, por ejemplo `Not enough messages to compact.` después de un intercambio corto único. Una llamada `query()` nueva de un solo disparo comienza con contexto vacío, así que use este patrón en una sesión con turnos previos, por ejemplo en [modo de entrada de streaming](/docs/es/agent-sdk/streaming-vs-single-mode) o cuando reanuda una sesión.
</Note>

<h3 id="reset-context-with-/clear">
  Reinicie el contexto con `/clear`
</h3>

El comando `/clear` reinicia la conversación a un contexto vacío, así que los prompts posteriores comienzan sin historial de conversación previo. La conversación anterior permanece en disco. Puede volver a esa conversación pasando su ID de sesión a la [opción `resume`](/docs/es/agent-sdk/sessions#resume-by-id).

`/clear` es útil en [modo de entrada de streaming](/docs/es/agent-sdk/streaming-vs-single-mode), donde envía múltiples prompts sobre una sola conexión. Para llamadas `query()` de un solo disparo, cada llamada ya comienza con contexto vacío, así que enviar `/clear` no tiene efecto práctico. Comience una nueva `query()` en su lugar.

<h2 id="create-skills">
  Cree skills
</h2>

Cree cada skill como un directorio que contiene un archivo `SKILL.md` con frontmatter YAML y contenido Markdown. El campo `description` determina cuándo Claude invoca su skill.

**Estructura de directorio de ejemplo**:

```text theme={null}
.claude/skills/security-check/
└── SKILL.md
```

<h3 id="choose-a-discovery-level">
  Elija un nivel de descubrimiento
</h3>

Guarde skills en uno de los dos [niveles de descubrimiento](/docs/es/skills#where-skills-live) más comunes:

* **Skills de proyecto**: `.claude/skills/`, disponibles solo en el proyecto actual
* **Skills personales**: `~/.claude/skills/`, disponibles en todos sus proyectos

Si tiene archivos de comando personalizados existentes en `.claude/commands/`, siguen funcionando. Un archivo de comando en `.claude/commands/deploy.md` crea `/deploy` y funciona de la misma manera que una skill en `.claude/skills/deploy/SKILL.md` lo haría. Si un archivo de comando y una skill comparten un nombre, consulte [Resuelva skills que comparten un nombre](/docs/es/skills#resolve-skills-that-share-a-name) para saber cuál se ejecuta. El SDK carga archivos `.claude/commands/` y `~/.claude/commands/` desde los mismos dos ámbitos que las skills. Consulte [Extienda Claude con skills](/docs/es/skills) para la guía completa de ambas formas de artefacto.

<h3 id="create-and-dispatch-your-first-skill">
  Cree y despache su primera skill
</h3>

Para ver el flujo completo, cree `.claude/skills/security-check/SKILL.md`:

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

Una vez que el archivo existe, la skill está disponible a través del SDK. Claude la invoca cuando una solicitud coincide con su descripción, y puede despatcharla directamente:

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

Una ejecución exitosa termina con un resultado `success` cuyo texto lleva los hallazgos del escaneo. Contra una pequeña aplicación Express con problemas sembrados, el texto del resultado comienza:

```text theme={null}
**Security scan of `app.js` — 4 findings (most severe first):**

1. **SQL Injection** (line 8) — `req.query.name` is concatenated directly into the SQL string. Trivially exploitable (`' OR '1'='1`, `'; DROP TABLE users;--`). **Fix:** use parameterized queries, e.g. `db.query("SELECT * FROM users WHERE name = ?", [req.query.name], cb)`.
...
```

El nombre de la skill también aparece en la matriz `slash_commands` del mensaje init.

<Note>
  Claude Code incluye skills incluidas `code-review` y `verify`. Si nombra un archivo `.claude/commands/` después de uno de ellos, por ejemplo `.claude/commands/code-review.md`, el archivo del comando sombrea la skill incluida y `slash_commands` lista el nombre una vez.
</Note>

<h2 id="pre-approve-tools-for-skills">
  Preapruebe herramientas para skills
</h2>

<Note>
  Para skills de proyecto y personales, Claude Code aplica el campo frontmatter [`allowed-tools`](/docs/es/skills#pre-approve-tools-for-a-skill) en sesiones del SDK. También puede preautorizar herramientas para estas skills a través de la opción `allowedTools` (`allowed_tools` en Python) en su configuración de consulta. Las skills [sincronizadas desde claude.ai](/docs/es/skills#how-claude-code-handles-the-frontmatter-of-a-synced-skill) siguen sus propias reglas de frontmatter.
</Note>

Las skills se ejecutan con las herramientas de la sesión. El ejemplo a continuación preaprueba `Read`, `Grep` y `Glob` con `allowedTools` (`allowed_tools` en Python), así que Claude puede inspeccionar archivos mientras ejecuta la [skill security-check](#create-and-dispatch-your-first-skill) sin detenerse para aprobación:

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

En el stream, la invocación de la skill aparece como un uso de herramienta Skill, seguido de llamadas Read en los archivos del proyecto. La ejecución termina con un resultado `success` cuyo texto lleva los hallazgos.

La lista preaprueba las herramientas nombradas en lugar de restringir las otras. Para el flujo de permisos completo, incluidos modos de permiso y la devolución de llamada `canUseTool`, consulte [Permisos](/docs/es/agent-sdk/permissions).

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="skills-not-found">
  Skills no encontradas
</h3>

**Verifique la configuración de settingSources**: el SDK descubre skills a través de las fuentes de configuración `user` y `project`. Si establece `settingSources`/`setting_sources` explícitamente y omite esas fuentes, el SDK no carga skills:

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

Para saber qué directorios de skills carga cada fuente, consulte la [tabla de fuentes del sistema de archivos](/docs/es/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources). Para más detalles sobre `settingSources`/`setting_sources`, consulte la [referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript#settingsource) o la [referencia del SDK de Python](/docs/es/agent-sdk/python#settingsource).

**Verifique el directorio de trabajo**: el SDK carga skills desde `.claude/skills/` en la opción `cwd` y en cada directorio padre hasta la raíz del repositorio. Asegúrese de que `cwd` apunte a o esté por debajo del directorio que contiene `.claude/skills/`, dentro del mismo repositorio:

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

Consulte [Use skills con el Agent SDK](#use-skills-with-the-agent-sdk) para el patrón completo.

**Verifique la ubicación del sistema de archivos**:

```bash theme={null}
# Check project skills
ls .claude/skills/*/SKILL.md

# Check personal skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill no se está utilizando
</h3>

**Verifique la opción `skills`**: si pasó una lista de `skills`, confirme que el nombre de la skill está incluido. Cuando Claude intenta invocar una skill no listada, la herramienta Skill devuelve `Skill <name> is not in this session's skills allowlist`. Añada el nombre a su lista, o despache la skill directamente enviando `/<name>` en un prompt, que funciona sin listar.

**Verifique la descripción**: asegúrese de que sea específica e incluya palabras clave relevantes. Consulte [Agent Skills best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) para obtener orientación sobre cómo escribir descripciones efectivas.

<h3 id="invalid-skill-name-error">
  Error de nombre de skill inválido
</h3>

Cuando un nombre en su lista `skills` no puede funcionar como un nombre exacto de skill, `query()` rechaza la lista antes de iniciar el proceso de Claude Code. Los nombres que desencadenan el rechazo incluyen:

* Un nombre vacío
* Un nombre que contiene paréntesis, comas o caracteres de control
* Un nombre relleno con espacios en blanco
* Una forma de comodín como un `*` desnudo o un sufijo `:*`

Cada SDK presenta el rechazo de manera diferente:

<Tabs>
  <Tab title="TypeScript">
    El SDK de TypeScript lanza un `Error` indicando la regla que la entrada rompió. Por ejemplo, `skills: ["docs:*"]` lanza:

    ```text theme={null}
    Invalid skill name "docs:*": wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Un nombre vacío reporta `Skill names must be non-empty strings.`

    Antes de TypeScript Agent SDK 0.3.221, el SDK no ejecutaba esta verificación.
  </Tab>

  <Tab title="Python">
    El SDK de Python genera `ValueError` indicando la regla que la entrada rompió. Por ejemplo, `skills=["docs:*"]` genera:

    ```text theme={null}
    ValueError: Invalid skill name 'docs:*': wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Un nombre vacío reporta `Skill names must be non-empty strings`.

    Antes de Python Agent SDK 0.2.129, el SDK no ejecutaba esta verificación.
  </Tab>
</Tabs>

<h3 id="additional-troubleshooting">
  Solución de problemas adicional
</h3>

Para la solución de problemas general de skills, como errores de sintaxis YAML y depuración, consulte la [sección de solución de problemas de skills de Claude Code](/docs/es/skills#troubleshooting).

<h2 id="next-steps">
  Próximos pasos
</h2>

La [guía de skills de Claude Code](/docs/es/skills) cubre la autoría en profundidad. Su orientación se aplica a sesiones del SDK. Comience con estas secciones:

* [Referencia de frontmatter](/docs/es/skills#frontmatter-reference): cada campo soportado
* [Pase argumentos a skills](/docs/es/skills#pass-arguments-to-skills): `$ARGUMENTS`, `$0`, `$1` y apilamiento de skills. La [tabla de sustitución completa](/docs/es/skills#available-string-substitutions) añade argumentos nombrados y las variables `${CLAUDE_*}`
* [Inyecte contexto dinámico](/docs/es/skills#inject-dynamic-context): líneas `` !`command` `` que se ejecutan antes de que Claude vea el contenido de la skill
* [Elija dónde cargan las skills](/docs/es/skills#where-skills-live): cada ubicación de skill, espacios de nombres de plugins y qué skill se ejecuta cuando dos comparten un nombre

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Comandos en Claude Code](/docs/es/commands): la superficie de comando completa, incluido cada uno integrado
* [Descripción general de Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview): descripción general conceptual, beneficios y arquitectura
* [Agent Skills best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): directrices de autoría para skills efectivas
* [Agent Skills cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): skills de ejemplo y plantillas
* [Subagentes en el SDK](/docs/es/agent-sdk/subagents): agentes similares basados en el sistema de archivos con opciones programáticas
* [Descripción general del SDK](/docs/es/agent-sdk/overview): conceptos generales del SDK
* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript): documentación completa de la API
* [Referencia del SDK de Python](/docs/es/agent-sdk/python): documentación completa de la API
