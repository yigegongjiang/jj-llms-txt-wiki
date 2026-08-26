> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent Skills en el SDK

> Extienda Claude con capacidades especializadas utilizando Agent Skills en el Claude Agent SDK

<h2 id="overview">
  Descripción general
</h2>

Agent Skills extienden Claude con capacidades especializadas que Claude invoca autónomamente cuando es relevante. Las Skills se empaquetan como archivos `SKILL.md` que contienen instrucciones, descripciones y recursos de apoyo opcionales.

Para obtener información completa sobre Skills, incluidos beneficios, arquitectura y directrices de autoría, consulte la [descripción general de Agent Skills](https://platform.claude.com/docs/es/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-sdk">
  Cómo funcionan las Skills con el SDK
</h2>

Cuando se utiliza el Claude Agent SDK, las Skills son:

1. **Definidas como artefactos del sistema de archivos**: Creadas como archivos `SKILL.md` en directorios específicos (`.claude/skills/`)
2. **Cargadas desde el sistema de archivos**: Las Skills se cargan desde ubicaciones del sistema de archivos gobernadas por `settingSources` (TypeScript) o `setting_sources` (Python)
3. **Descubiertas automáticamente**: Una vez que se cargan las configuraciones del sistema de archivos, los metadatos de Skill se descubren al inicio desde directorios de usuario y proyecto; el contenido completo se carga cuando se activa
4. **Invocadas por el modelo**: Claude elige autónomamente cuándo usarlas según el contexto
5. **Filtradas a través de la opción `skills`**: Las Skills descubiertas están habilitadas de forma predeterminada. Pase una lista de nombres de Skills, `"all"`, o `[]` para controlar cuáles están disponibles en la sesión

A diferencia de los subagentes (que se pueden definir mediante programación), las Skills deben crearse como artefactos del sistema de archivos. El SDK no proporciona una API programática para registrar Skills.

<Note>
  Las Skills se descubren a través de las fuentes de configuración del sistema de archivos. Con las opciones predeterminadas de `query()`, el SDK carga fuentes de usuario y proyecto, por lo que las Skills en `~/.claude/skills/`, `<cwd>/.claude/skills/`, y `.claude/skills/` en cualquier directorio padre de `<cwd>` hasta la raíz del repositorio están disponibles. Si establece `settingSources` explícitamente, incluya `'user'` o `'project'` para mantener el descubrimiento de Skills, o use la [opción `plugins`](/docs/es/agent-sdk/plugins) para cargar Skills desde una ruta específica.
</Note>

<h2 id="using-skills-with-the-sdk">
  Uso de Skills con el SDK
</h2>

Establezca la opción `skills` en `query()` para controlar qué Skills están disponibles para la sesión. Cuando se omite, las Skills descubiertas están habilitadas y la herramienta Skill está disponible, coincidiendo con el comportamiento de CLI. Pase `"all"` para habilitar cada Skill descubierta, una lista de nombres de Skill para habilitar solo esos, o `[]` para deshabilitar todos. Cuando establece `skills`, el SDK añade la herramienta Skill a `allowedTools` automáticamente. Si también pasa una lista explícita de `tools`, incluya `"Skill"` en esa lista para que Claude pueda invocar skills.

Una vez configurado, Claude descubre automáticamente Skills desde el sistema de archivos e invoca los cuando es relevante para la solicitud del usuario.

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

Para habilitar solo Skills específicas, pase sus nombres. Los nombres coinciden con el campo `name` en `SKILL.md` o el nombre del directorio de la Skill. Use `plugin:skill` para Skills proporcionadas por plugins.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

La opción `skills` es un filtro de contexto, no un sandbox. Las Skills no listadas se ocultan del modelo y se rechazan por la herramienta Skill, pero sus archivos permanecen en el disco y son accesibles a través de Read y Bash.

<h2 id="skill-locations">
  Ubicaciones de Skills
</h2>

Las Skills se cargan desde directorios del sistema de archivos según su configuración `settingSources`/`setting_sources`:

* **Project Skills** (`.claude/skills/`): Compartidas con su equipo a través de git - cargadas cuando `setting_sources` incluye `"project"`
* **User Skills** (`~/.claude/skills/`): Skills personales en todos los proyectos - cargadas cuando `setting_sources` incluye `"user"`
* **Plugin Skills**: Incluidas con plugins de Claude Code instalados

<h2 id="creating-skills">
  Creación de Skills
</h2>

Las Skills se definen como directorios que contienen un archivo `SKILL.md` con frontmatter YAML y contenido Markdown. El campo `description` determina cuándo Claude invoca su Skill.

**Estructura de directorio de ejemplo**:

```bash theme={null}
.claude/skills/processing-pdfs/
└── SKILL.md
```

Para obtener orientación completa sobre la creación de Skills, incluida la estructura de SKILL.md, Skills de múltiples archivos y ejemplos, consulte:

* [Agent Skills en Claude Code](/docs/es/skills): Guía completa con ejemplos
* [Agent Skills Best Practices](https://platform.claude.com/docs/es/agents-and-tools/agent-skills/best-practices): Directrices de autoría y convenciones de nomenclatura

<h2 id="tool-restrictions">
  Restricciones de herramientas
</h2>

<Note>
  El campo frontmatter `allowed-tools` en SKILL.md solo se admite cuando se utiliza directamente la CLI de Claude Code. **No se aplica cuando se utilizan Skills a través del SDK**.

  Cuando se utiliza el SDK, controle el acceso a herramientas a través de la opción principal `allowedTools` en su configuración de consulta.
</Note>

Para controlar el acceso a herramientas para Skills en aplicaciones SDK, use `allowedTools` para preautorizar herramientas específicas. Sin una devolución de llamada `canUseTool`, se deniega cualquier cosa que no esté en la lista:

<Note>
  Se asume que las declaraciones de importación del primer ejemplo están en los siguientes fragmentos de código.
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
  Descubrimiento de Skills disponibles
</h2>

Para ver qué Skills están disponibles en su aplicación SDK, simplemente pregunte a Claude:

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

Claude listará las Skills disponibles según su directorio de trabajo actual y plugins instalados.

<h2 id="testing-skills">
  Prueba de Skills
</h2>

Pruebe Skills haciendo preguntas que coincidan con sus descripciones:

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

Claude invoca automáticamente la Skill relevante si la descripción coincide con su solicitud.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="skills-not-found">
  Skills no encontradas
</h3>

**Verifique la configuración de settingSources**: Las Skills se descubren a través de las fuentes de configuración `user` y `project`. Si establece `settingSources`/`setting_sources` explícitamente y omite esas fuentes, las Skills no se cargan:

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

Para más detalles sobre `settingSources`/`setting_sources`, consulte la [referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript#settingsource) o la [referencia del SDK de Python](/docs/es/agent-sdk/python#settingsource).

**Verifique el directorio de trabajo**: El SDK carga Skills desde `.claude/skills/` en la opción `cwd` y en cada directorio padre hasta la raíz del repositorio. Asegúrese de que `cwd` apunte a o esté por debajo del directorio que contiene `.claude/skills/`, dentro del mismo repositorio:

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

Consulte la sección "Uso de Skills con el SDK" anterior para el patrón completo.

**Verifique la ubicación del sistema de archivos**:

```bash theme={null}
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill no se está utilizando
</h3>

**Verifique la opción `skills`**: Si pasó una lista de `skills`, confirme que el nombre de la Skill está incluido. Pasar `[]` deshabilita todas las Skills.

**Verifique la descripción**: Asegúrese de que sea específica e incluya palabras clave relevantes. Consulte [Agent Skills Best Practices](https://platform.claude.com/docs/es/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) para obtener orientación sobre cómo escribir descripciones efectivas.

<h3 id="additional-troubleshooting">
  Solución de problemas adicional
</h3>

Para la solución de problemas general de Skills (sintaxis YAML, depuración, etc.), consulte la [sección de solución de problemas de Skills de Claude Code](/docs/es/skills#troubleshooting).

<h2 id="related-documentation">
  Documentación relacionada
</h2>

<h3 id="skills-guides">
  Guías de Skills
</h3>

* [Agent Skills en Claude Code](/docs/es/skills): Guía completa de Skills con creación, ejemplos y solución de problemas
* [Agent Skills Overview](https://platform.claude.com/docs/es/agents-and-tools/agent-skills/overview): Descripción general conceptual, beneficios y arquitectura
* [Agent Skills Best Practices](https://platform.claude.com/docs/es/agents-and-tools/agent-skills/best-practices): Directrices de autoría para Skills efectivas
* [Agent Skills Cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): Skills de ejemplo y plantillas

<h3 id="sdk-resources">
  Recursos del SDK
</h3>

* [Subagents en el SDK](/docs/es/agent-sdk/subagents): Agentes similares basados en el sistema de archivos con opciones programáticas
* [Slash Commands en el SDK](/docs/es/agent-sdk/slash-commands): Comandos invocados por el usuario
* [Descripción general del SDK](/docs/es/agent-sdk/overview): Conceptos generales del SDK
* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript): Documentación completa de la API
* [Referencia del SDK de Python](/docs/es/agent-sdk/python): Documentación completa de la API
