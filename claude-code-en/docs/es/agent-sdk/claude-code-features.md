> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usar características de Claude Code en el SDK

> Cargue instrucciones de proyecto, skills, hooks y otras características de Claude Code en sus agentes SDK.

El Agent SDK se basa en la misma base que Claude Code, lo que significa que sus agentes SDK tienen acceso a las mismas características basadas en el sistema de archivos: instrucciones de proyecto (`CLAUDE.md` y reglas), skills, hooks y más.

Cuando omite `settingSources`, `query()` lee la misma configuración del sistema de archivos que la CLI de Claude Code: configuración de usuario, proyecto y local, archivos `CLAUDE.md` y skills, agentes y comandos en `.claude/`. Para ejecutar sin estos, pase `settingSources: []`, lo que limita el agente a lo que configure programáticamente. La configuración de políticas administradas y la configuración global `~/.claude.json` se leen independientemente de esta opción. Para obtener más información, consulte [Qué settingSources no controla](#what-settingsources-does-not-control).

<h2 id="control-filesystem-settings-with-settingsources">
  Controlar la configuración del sistema de archivos con settingSources
</h2>

La opción de fuentes de configuración ([`setting_sources`](/docs/es/agent-sdk/python#claudeagentoptions) en Python, [`settingSources`](/docs/es/agent-sdk/typescript#settingsource) en TypeScript) controla qué configuración basada en el sistema de archivos carga el SDK. Pase una lista explícita para optar por fuentes específicas, o pase una matriz vacía para deshabilitar la configuración de usuario, proyecto y local.

Este ejemplo carga tanto la configuración a nivel de usuario como a nivel de proyecto estableciendo `settingSources` en `["user", "project"]`:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage
  import asyncio


  async def main():
      async for message in query(
          prompt="Help me refactor the auth module",
          options=ClaudeAgentOptions(
              # "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
              # Together they give the agent access to CLAUDE.md, skills, hooks, and
              # permissions from both locations.
              setting_sources=["user", "project"],
              allowed_tools=["Read", "Edit", "Bash"],
          ),
      ):
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(f"\nResult: {message.result}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me refactor the auth module",
    options: {
      // "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
      // Together they give the agent access to CLAUDE.md, skills, hooks, and
      // permissions from both locations.
      settingSources: ["user", "project"],
      allowedTools: ["Read", "Edit", "Bash"]
    }
  })) {
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "text") console.log(block.text);
      }
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(`\nResult: ${message.result}`);
    }
  }
  ```
</CodeGroup>

Cuando se ejecuta, la respuesta del asistente se imprime en stdout, seguida de una línea de resultado final una vez que se completa la ejecución.

Cada fuente carga la configuración desde una ubicación específica, donde `<cwd>` es el directorio de trabajo que pasa a través de la opción `cwd`, o el directorio actual del proceso si no está establecido. Para la definición de tipo completa, consulte [`SettingSource`](/docs/es/agent-sdk/typescript#settingsource) (TypeScript) o [`SettingSource`](/docs/es/agent-sdk/python#settingsource) (Python).

| Fuente      | Qué carga                                                                                                                       | Ubicación                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :---------- | :------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"project"` | `settings.json` del proyecto y hooks; CLAUDE.md del proyecto y `.claude/rules/*.md`; skills, comandos y subagentes del proyecto | `<cwd>/.claude/` para `settings.json` y hooks; `<cwd>` y cada directorio padre para CLAUDE.md y rules; `<cwd>` y cada directorio padre hasta la raíz del repositorio para skills, comandos y subagentes, más las carpetas `.claude/skills/`, `.claude/commands/` y `.claude/agents/` de cada directorio que pase a través de la opción `additionalDirectories` o `add_dirs`, que el SDK pasa a Claude Code como [`--add-dir`](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) |
| `"user"`    | `settings.json` del usuario; CLAUDE.md del usuario y `~/.claude/rules/*.md`; skills, comandos y subagentes del usuario          | `~/.claude/` para `settings.json`, CLAUDE.md y rules; `~/.claude/skills/`, `~/.claude/commands/` y `~/.claude/agents/` para skills, comandos y subagentes                                                                                                                                                                                                                                                                                                                                                 |
| `"local"`   | CLAUDE.local.md, `.claude/settings.local.json`                                                                                  | `<cwd>/.claude/` para `settings.local.json`; `<cwd>` y cada directorio padre para CLAUDE.local.md                                                                                                                                                                                                                                                                                                                                                                                                         |

Omitir `settingSources` es equivalente a `["user", "project", "local"]`.

La opción `cwd` determina dónde busca el SDK las entradas a nivel de proyecto. `settings.json` del proyecto y hooks se cargan solo desde `<cwd>/.claude/` sin fallback de directorio padre.

<h3 id="what-settingsources-does-not-control">
  Qué settingSources no controla
</h3>

`settingSources` cubre la configuración de usuario, proyecto y local. Algunas entradas se leen independientemente de su valor:

| Entrada                                                                                                                                  | Comportamiento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Para deshabilitar                                                                                                                                                                                                                                                                                              |
| :--------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Configuración de políticas administradas                                                                                                 | Política administrada por el punto de conexión, ya sea plist MDM, política de registro o archivos de configuración administrada, se carga desde el host. [La configuración administrada por servidor](/docs/es/server-managed-settings) se obtiene en una [configuración elegible](/docs/es/server-managed-settings#platform-availability) cuando la sesión se autentica con un inicio de sesión OAuth de la organización, una clave API configurada directamente, o un [perfil de Anthropic](/docs/es/authentication#anthropic-profiles-and-federation-credentials) `user_oauth` | Política de punto de conexión: elimine el archivo de configuración administrada, plist o política de registro del host. Configuración administrada por servidor: un [Propietario](/docs/es/server-managed-settings#access-control) en su organización de Claude los controla; no se puede deshabilitar desde el SDK |
| Configuración global `~/.claude.json`                                                                                                    | Siempre se lee                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Reubique con `CLAUDE_CONFIG_DIR` en `env`                                                                                                                                                                                                                                                                      |
| Memoria automática en `~/.claude/projects/<project>/memory/`                                                                             | Se carga en el mensaje del sistema al inicio de la sesión. El agente escribe nuevos recuerdos allí con las herramientas estándar `Write` y `Edit` en lugar de una herramienta de memoria dedicada, por lo que esas herramientas deben estar habilitadas para que el agente guarde recuerdos                                                                                                                                                                                                                                                                        | Establezca `autoMemoryEnabled: false` en la configuración, o `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` en `env`                                                                                                                                                                                                      |
| [Conectores MCP de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai)                                                                    | Se cargan cuando la sesión se autentica con su inicio de sesión en claude.ai. No se cargan cuando `CLAUDE_CODE_OAUTH_TOKEN` contiene un token de [`claude setup-token`](/docs/es/authentication#generate-a-long-lived-token), que solo puede hacer solicitudes de modelo. Pasar `mcpServers: {}` no suprime los conectores                                                                                                                                                                                                                                              | Establezca `strictMcpConfig: true`, [`disableClaudeAiConnectors: true`](/docs/es/mcp#disable-claude-ai-connectors) en la configuración, o `ENABLE_CLAUDEAI_MCP_SERVERS=false` en `env`                                                                                                                              |
| [`sandbox.credentials`](/docs/es/sandboxing#protect-credentials) entradas `deny` y entradas de `mask` de archivo en `~/.claude/settings.json` | Cuando se ejecuta el [sandbox de comandos](/docs/es/sandboxing), Claude Code aplica las entradas `deny` y mantiene las entradas de `mask` de `credentials.files` como restricciones incluso cuando `settingSources` excluye la configuración del usuario. Claude Code usa estas entradas solo para reducir lo que los comandos en sandbox pueden acceder                                                                                                                                                                                                                | Elimine las entradas de `~/.claude/settings.json`                                                                                                                                                                                                                                                              |

<Warning>
  No confíe en las opciones predeterminadas de `query()` para el aislamiento multiinquilino. Debido a que las entradas anteriores se leen independientemente de `settingSources`, un proceso SDK puede recopilar configuración a nivel de host y memoria por directorio. Para implementaciones multiinquilino, ejecute cada inquilino en su propio sistema de archivos y establezca `settingSources: []` más `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` en `env`. [La configuración administrada por servidor](/docs/es/server-managed-settings) se obtiene cuando el proceso se autentica con una credencial de la organización; el aislamiento del sistema de archivos no las elimina. Consulte [Implementación segura](/docs/es/agent-sdk/secure-deployment).
</Warning>

<h2 id="project-instructions-claude-md-and-rules">
  Instrucciones de proyecto (CLAUDE.md y reglas)
</h2>

Los archivos `CLAUDE.md` y los archivos `.claude/rules/*.md` proporcionan a su agente contexto persistente sobre su proyecto: convenciones de codificación, comandos de compilación, decisiones de arquitectura e instrucciones. Cuando `settingSources` incluye `"project"`, como en el [ejemplo de `settingSources`](#control-filesystem-settings-with-settingsources), el SDK carga estos archivos en el contexto al inicio de la sesión. El agente luego sigue sus convenciones de proyecto sin que tenga que repetirlas en cada mensaje.

<h3 id="claude-md-load-locations">
  Ubicaciones de carga de CLAUDE.md
</h3>

| Nivel                        | Ubicación                                                                  | Cuándo se carga                                                                                             |
| :--------------------------- | :------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- |
| Proyecto (raíz)              | `<cwd>/CLAUDE.md` o `<cwd>/.claude/CLAUDE.md`                              | `settingSources` incluye `"project"`                                                                        |
| Reglas del proyecto          | `<cwd>/.claude/rules/*.md` y `.claude/rules/*.md` en cada directorio padre | `settingSources` incluye `"project"`                                                                        |
| Proyecto (directorios padre) | Archivos `CLAUDE.md` en directorios por encima de `cwd`                    | `settingSources` incluye `"project"`, se carga al inicio de la sesión                                       |
| Proyecto (directorios hijo)  | Archivos `CLAUDE.md` en subdirectorios de `cwd`                            | `settingSources` incluye `"project"`, se carga bajo demanda cuando el agente lee un archivo en ese subárbol |
| Local                        | `<cwd>/CLAUDE.local.md` y `CLAUDE.local.md` en cada directorio padre       | `settingSources` incluye `"local"`                                                                          |
| Usuario                      | `~/.claude/CLAUDE.md`                                                      | `settingSources` incluye `"user"`                                                                           |
| Reglas del usuario           | `~/.claude/rules/*.md`                                                     | `settingSources` incluye `"user"`                                                                           |

Todos los niveles son aditivos: si existen archivos `CLAUDE.md` tanto de proyecto como de usuario, el agente ve ambos. No hay una regla de precedencia dura entre niveles; si las instrucciones entran en conflicto, el resultado depende de cómo Claude las interprete. Escriba reglas que no entren en conflicto, o indique la precedencia explícitamente en el archivo más específico ("Estas instrucciones de proyecto anulan cualquier valor predeterminado conflictivo a nivel de usuario").

<Tip>
  También puede inyectar contexto directamente a través de `systemPrompt` sin usar archivos `CLAUDE.md`. Consulte [Modificar mensajes del sistema](/docs/es/agent-sdk/modifying-system-prompts). Use `CLAUDE.md` cuando desee que el mismo contexto se comparta entre sesiones interactivas de Claude Code y sus agentes SDK.
</Tip>

Para saber cómo estructurar y organizar el contenido de `CLAUDE.md`, consulte [Administrar la memoria de Claude](/docs/es/memory).

<h2 id="skills">
  Skills
</h2>

Los skills son archivos markdown que proporcionan a su agente conocimiento especializado y flujos de trabajo invocables. A diferencia de `CLAUDE.md` (que se carga cada sesión), los skills se cargan bajo demanda. El agente recibe descripciones de skills al inicio y carga el contenido completo cuando es relevante.

Los skills se descubren desde el sistema de archivos a través de `settingSources`. Cuando la opción `skills` en `query()` se omite, los skills de usuario y proyecto descubiertos se habilitan y la herramienta Skill está disponible, coincidiendo con el comportamiento de CLI. Para controlar qué skills están habilitados, pase `skills` como `"all"`, una lista de nombres de skills, o `[]` para deshabilitar todos. Cuando `skills` está configurado, el SDK agrega automáticamente la herramienta Skill a `allowedTools`. Si también pasa una lista explícita de `tools`, incluya `"Skill"` en esa lista para que Claude pueda invocar skills.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage
  import asyncio


  # Skills in .claude/skills/ are discovered automatically
  # when settingSources includes "project"
  async def main():
      async for message in query(
          prompt="Review this PR using our code review checklist",
          options=ClaudeAgentOptions(
              setting_sources=["user", "project"],
              skills="all",
              allowed_tools=["Read", "Grep", "Glob"],
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Skills in .claude/skills/ are discovered automatically
  // when settingSources includes "project"
  for await (const message of query({
    prompt: "Review this PR using our code review checklist",
    options: {
      settingSources: ["user", "project"],
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<Note>
  Los skills deben crearse como artefactos del sistema de archivos (`.claude/skills/<name>/SKILL.md`). El SDK no tiene una API programática para registrar skills. Consulte [Agent Skills en el SDK](/docs/es/agent-sdk/skills) para obtener detalles completos.
</Note>

<h2 id="hooks">
  Hooks
</h2>

El SDK admite dos formas de definir hooks, y se ejecutan lado a lado:

* **Hooks del sistema de archivos:** comandos de shell definidos en `settings.json`, cargados cuando `settingSources` incluye la fuente relevante. Estos son los mismos hooks que configuraría para [sesiones interactivas de Claude Code](/docs/es/hooks-guide).
* **Hooks programáticos:** funciones de devolución de llamada pasadas directamente a `query()`. Se ejecutan en el proceso de su aplicación y pueden devolver decisiones estructuradas. Consulte [Controlar la ejecución con hooks](/docs/es/agent-sdk/hooks).

Las devoluciones de llamada de hooks reciben la entrada de la herramienta y devuelven un diccionario de decisión. Devolver `{}` significa permitir que la herramienta continúe. Para bloquear la ejecución, devuelva un objeto `hookSpecificOutput` con `permissionDecision: "deny"` y un `permissionDecisionReason`. La razón se envía a Claude como el resultado de la herramienta. Consulte la [guía de hooks](/docs/es/agent-sdk/hooks) para la firma de devolución de llamada completa y los tipos de retorno.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, ResultMessage
  import asyncio


  # PreToolUse hook callback. Positional args:
  #   input_data: HookInput dict with tool_name, tool_input, hook_event_name
  #   tool_use_id: str | None, the ID of the tool call being intercepted
  #   context: HookContext, reserved for future abort-signal support
  async def audit_bash(input_data, tool_use_id, context):
      command = input_data.get("tool_input", {}).get("command", "")
      if "rm -rf" in command:
          return {
              "hookSpecificOutput": {
                  "hookEventName": "PreToolUse",
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Destructive command blocked",
              }
          }
      return {}  # Empty dict: allow the tool to proceed


  # Filesystem hooks from .claude/settings.json run automatically
  # when settingSources loads them. You can also add programmatic hooks:
  async def main():
      async for message in query(
          prompt="Refactor the auth module",
          options=ClaudeAgentOptions(
              setting_sources=["project"],  # Loads hooks from .claude/settings.json
              hooks={
                  "PreToolUse": [
                      HookMatcher(matcher="Bash", hooks=[audit_bash]),
                  ]
              },
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type HookInput, type HookJSONOutput } from "@anthropic-ai/claude-agent-sdk";

  // PreToolUse hook callback. HookInput is a discriminated union on
  // hook_event_name, so narrowing on it gives TypeScript the right
  // tool_input shape for this event.
  const auditBash = async (input: HookInput): Promise<HookJSONOutput> => {
    if (input.hook_event_name !== "PreToolUse") return {};
    const toolInput = input.tool_input as { command?: string };
    if (toolInput.command?.includes("rm -rf")) {
      return {
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: "Destructive command blocked",
        },
      };
    }
    return {}; // Empty object: allow the tool to proceed
  };

  // Filesystem hooks from .claude/settings.json run automatically
  // when settingSources loads them. You can also add programmatic hooks:
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      settingSources: ["project"], // Loads hooks from .claude/settings.json
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [auditBash] }]
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h3 id="when-to-use-which-hook-type">
  Cuándo usar qué tipo de hook
</h3>

| Tipo de hook                                            | Mejor para                                                                                                                                                                                                                                                                                                                                      |
| :------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Sistema de archivos** (`settings.json`)               | Compartir hooks entre sesiones CLI y SDK. Admite `"command"` (scripts de shell), `"http"` (POST a un punto final), `"mcp_tool"` (llamar a la herramienta de un servidor MCP conectado), `"prompt"` (LLM evalúa un mensaje), y `"agent"` (genera un agente verificador). Se ejecutan en el agente principal y en cualquier subagente que genere. |
| **Programático** (devoluciones de llamada en `query()`) | Lógica específica de la aplicación, decisiones estructuradas e integración en proceso. Estas también se ejecutan dentro de subagentes. La entrada del hook, el primer argumento de la devolución de llamada, lleva campos `agent_id` y `agent_type` que identifican qué agente activó el hook.                                                  |

<Note>
  El SDK de TypeScript admite eventos de hook adicionales más allá de Python, incluidos `SessionStart`, `SessionEnd`, `TeammateIdle` y `TaskCompleted`. Consulte la [guía de hooks](/docs/es/agent-sdk/hooks) para la tabla de compatibilidad de eventos completa.
</Note>

Para obtener detalles completos sobre hooks programáticos, consulte [Controlar la ejecución con hooks](/docs/es/agent-sdk/hooks). Para la sintaxis de hooks del sistema de archivos, consulte [Hooks](/docs/es/hooks).

<h2 id="choose-the-right-feature">
  Elige la función correcta
</h2>

El Agent SDK te proporciona acceso a varias formas de extender el comportamiento de tu agente. Si no estás seguro de cuál usar, esta tabla asigna objetivos comunes al enfoque correcto.

| Quieres...                                                                                                        | Usa                                           | Superficie del SDK                                                                                                                                                                                            |
| :---------------------------------------------------------------------------------------------------------------- | :-------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Establecer convenciones de proyecto que tu agente siempre sigue                                                   | [CLAUDE.md](/docs/es/memory)                       | `settingSources: ["project"]` lo carga automáticamente                                                                                                                                                        |
| Dar al agente material de referencia que carga cuando es relevante                                                | [Skills](/docs/es/agent-sdk/skills)                | opción `settingSources` + `skills`                                                                                                                                                                            |
| Ejecutar un flujo de trabajo reutilizable (desplegar, revisar, lanzar)                                            | [User-invocable skills](/docs/es/agent-sdk/skills) | opción `settingSources` + `skills`                                                                                                                                                                            |
| Delegar una subtarea aislada a un contexto nuevo (investigación, revisión)                                        | [Subagentes](/docs/es/agent-sdk/subagents)         | parámetro `agents` + `allowedTools: ["Agent"]`                                                                                                                                                                |
| Coordinar múltiples instancias de Claude Code con listas de tareas compartidas y mensajería directa entre agentes | [Agent teams](/docs/es/agent-teams)                | No se configura directamente a través de opciones del SDK. Los equipos de agentes son una función de CLI donde una sesión actúa como líder del equipo, coordinando el trabajo entre compañeros independientes |
| Ejecutar lógica determinista en llamadas de herramientas (auditoría, bloqueo, transformación)                     | [Hooks](/docs/es/agent-sdk/hooks)                  | parámetro `hooks` con callbacks, o scripts de shell cargados a través de `settingSources`                                                                                                                     |
| Dar a Claude acceso a herramientas estructuradas para un servicio externo                                         | [MCP](/docs/es/agent-sdk/mcp)                      | parámetro `mcpServers`                                                                                                                                                                                        |

Cada función que habilites se suma a la ventana de contexto de tu agente. Para costos por función y cómo estas funciones se superponen, consulta [Extend Claude Code](/docs/es/features-overview#understand-context-costs).

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Extender Claude Code](/docs/es/features-overview): Descripción conceptual de todas las características de extensión, con tablas de comparación y análisis de costos de contexto
* [Skills en el SDK](/docs/es/agent-sdk/skills): Guía completa para usar skills de forma programática
* [Subagentes](/docs/es/agent-sdk/subagents): Define e invoca subagentes para tareas aisladas
* [Hooks](/docs/es/agent-sdk/hooks): Intercepta y controla el comportamiento del agente en puntos clave de ejecución
* [Permisos](/docs/es/agent-sdk/permissions): Controla el acceso a herramientas con modos, reglas y callbacks
* [Prompts del sistema](/docs/es/agent-sdk/modifying-system-prompts): Inyecta contexto sin archivos CLAUDE.md
