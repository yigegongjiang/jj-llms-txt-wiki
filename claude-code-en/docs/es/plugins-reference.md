> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia de plugins

> Referencia técnica completa para el sistema de plugins de Claude Code, incluyendo esquemas, comandos CLI y especificaciones de componentes.

<Tip>
  ¿Buscas instalar plugins? Consulta [Descubrir e instalar plugins](/docs/es/discover-plugins). Para crear plugins, consulta [Plugins](/docs/es/plugins). Para distribuir plugins, consulta [Marketplaces de plugins](/docs/es/plugin-marketplaces).
</Tip>

Esta referencia proporciona especificaciones técnicas completas para el sistema de plugins de Claude Code, incluyendo esquemas de componentes, comandos CLI y herramientas de desarrollo.

Un **plugin** es un directorio independiente de componentes que extiende Claude Code con funcionalidad personalizada. Los componentes del plugin incluyen skills, agents, hooks, servidores MCP, servidores LSP y monitores.

<h2 id="plugin-components-reference">
  Referencia de componentes de plugins
</h2>

<h3 id="skills">
  Skills
</h3>

Los plugins añaden skills a Claude Code, creando atajos `/name` que usted o Claude pueden invocar.

**Ubicación**: Directorio `skills/` o `commands/` en la raíz del plugin, o un único archivo `SKILL.md` en la raíz del plugin

**Formato de archivo**: Los skills son directorios con `SKILL.md`; los comandos son archivos markdown simples

**Estructura de skill**:

```text theme={null}
skills/
├── pdf-processor/
│   ├── SKILL.md
│   ├── reference.md (opcional)
│   └── scripts/ (opcional)
└── code-reviewer/
    └── SKILL.md
```

**Comportamiento de integración**:

* Los skills y comandos se descubren automáticamente cuando se instala el plugin
* Claude puede invocarlos automáticamente según el contexto de la tarea
* Los skills pueden incluir archivos de apoyo junto a SKILL.md

Si un plugin no tiene un directorio `skills/` y ningún campo de manifiesto `skills`, un `SKILL.md` en la raíz del plugin se carga como una única skill. Establezca el campo frontmatter `name` para controlar el nombre de invocación de la skill. Sin él, Claude Code recurre al nombre del directorio de instalación, que para plugins instalados desde el marketplace es una cadena de versión que cambia en cada actualización. Para plugins que distribuyen más de una skill, use el diseño de directorio `skills/` mostrado arriba.

Para obtener detalles completos, consulte [Skills](/docs/es/skills).

<h3 id="agents">
  Agents
</h3>

Los plugins pueden proporcionar subagents especializados para tareas específicas que Claude puede invocar automáticamente cuando sea apropiado.

**Ubicación**: Directorio `agents/` en la raíz del plugin

**Formato de archivo**: Archivos markdown que describen las capacidades del agent

**Estructura del agent**:

```markdown theme={null}
---
name: agent-name
description: En qué se especializa este agent y cuándo Claude debe invocarlo
model: sonnet
effort: medium
maxTurns: 20
disallowedTools: Write, Edit
---

Prompt del sistema detallado para el agent describiendo su rol, experiencia y comportamiento.
```

Los agents del plugin soportan campos frontmatter `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background` e `isolation`. El único valor válido de `isolation` es `"worktree"`. Por razones de seguridad, `hooks`, `mcpServers` y `permissionMode` no se soportan para agents distribuidos con plugins.

**Puntos de integración**:

* Los agents aparecen en la interfaz [@-mention typeahead](/docs/es/sub-agents#invoke-subagents-explicitly) bajo su nombre con alcance, como `my-plugin:code-reviewer`, una vez que el plugin está habilitado
* Claude puede invocar agents automáticamente según el contexto de la tarea
* Los agents pueden ser invocados manualmente por los usuarios
* Los agents del plugin funcionan junto con los agents integrados de Claude

Para obtener detalles completos, consulte [Subagents](/docs/es/sub-agents).

<h3 id="hooks">
  Hooks
</h3>

Los plugins pueden proporcionar manejadores de eventos que responden automáticamente a eventos de Claude Code.

**Ubicación**: `hooks/hooks.json` en la raíz del plugin, o en línea en plugin.json

**Formato**: Configuración JSON con coincidencias de eventos y acciones

**Configuración de hook**:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format-code.sh"
          }
        ]
      }
    ]
  }
}
```

Los hooks del plugin responden a los mismos eventos del ciclo de vida que los [hooks definidos por el usuario](/docs/es/hooks):

| Event                 | When it fires                                                                                                                                                                                                                                         |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | When a session begins or resumes                                                                                                                                                                                                                      |
| `Setup`               | When you start Claude Code with `--init-only`, or with `--init` or `--maintenance` in `-p` mode. For one-time preparation in CI or scripts                                                                                                            |
| `UserPromptSubmit`    | When you submit a prompt, before Claude processes it                                                                                                                                                                                                  |
| `UserPromptExpansion` | When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion                                                                                                                                                    |
| `PreToolUse`          | Before a tool call executes. Can block it                                                                                                                                                                                                             |
| `PermissionRequest`   | When a tool call needs a permission decision                                                                                                                                                                                                          |
| `PermissionDenied`    | When auto mode denies a tool call, including denials without a classifier verdict. Use JSON `hookSpecificOutput.retry: true` to tell the model it may retry the denied tool call. Claude Code ignores `retry` when the classifier produced no verdict |
| `PostToolUse`         | After a tool call succeeds                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | After a tool call fails                                                                                                                                                                                                                               |
| `PostToolBatch`       | After a full batch of parallel tool calls resolves, before the next model call                                                                                                                                                                        |
| `Notification`        | When Claude Code sends a notification                                                                                                                                                                                                                 |
| `MessageDisplay`      | While assistant message text is displayed                                                                                                                                                                                                             |
| `SubagentStart`       | When a subagent is spawned                                                                                                                                                                                                                            |
| `SubagentStop`        | When a subagent finishes                                                                                                                                                                                                                              |
| `TaskCreated`         | When a task is being created via `TaskCreate`                                                                                                                                                                                                         |
| `TaskCompleted`       | When a task is being marked as completed                                                                                                                                                                                                              |
| `Stop`                | When Claude finishes responding                                                                                                                                                                                                                       |
| `StopFailure`         | When the turn ends due to an API error                                                                                                                                                                                                                |
| `TeammateIdle`        | When an [agent team](/docs/en/agent-teams) teammate is about to go idle                                                                                                                                                                                    |
| `InstructionsLoaded`  | When a CLAUDE.md or `.claude/rules/*.md` file is loaded into context. Fires at session start and when files are lazily loaded during a session                                                                                                        |
| `ConfigChange`        | When a configuration file changes during a session                                                                                                                                                                                                    |
| `CwdChanged`          | When the working directory changes, for example when Claude executes a `cd` command. Useful for reactive environment management with tools like direnv                                                                                                |
| `DirectoryAdded`      | When a working directory is added mid-session via `/add-dir` or the SDK `register_repo_root` control request                                                                                                                                          |
| `FileChanged`         | When a watched file changes on disk. The `matcher` field specifies which filenames to watch                                                                                                                                                           |
| `WorktreeCreate`      | When a worktree is being created via `--worktree`, `isolation: "worktree"`, or for a background session. Replaces default git behavior                                                                                                                |
| `WorktreeRemove`      | When a worktree is being removed at session exit, when a subagent finishes, or when you delete a background session                                                                                                                                   |
| `PreCompact`          | Before context compaction                                                                                                                                                                                                                             |
| `PostCompact`         | After context compaction completes                                                                                                                                                                                                                    |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

**Tipos de hook**:

* `command`: ejecutar comandos de shell o scripts
* `http`: enviar el JSON del evento como una solicitud POST a una URL
* `mcp_tool`: llamar a una herramienta en un [servidor MCP](/docs/es/mcp) configurado
* `prompt`: evaluar un prompt con un LLM (usa el marcador de posición `$ARGUMENTS` para el contexto)
* `agent`: ejecutar un verificador agentic con herramientas para tareas de verificación complejas

Los hooks que apuntan al [servidor MCP agrupado](#mcp-servers) propio del plugin deben usar sus nombres con alcance. Los coincidentes de herramientas y los campos `if` toman el nombre de herramienta con alcance `mcp__plugin_<plugin-name>_<server-name>__<tool>`, y el campo `server` de un hook `mcp_tool` toma `plugin:<plugin-name>:<server-name>`. Un coincidente escrito contra la clave del servidor desnuda nunca se dispara. Consulte [Coincidir herramientas MCP](/docs/es/hooks#match-mcp-tools) y [Servidores MCP proporcionados por plugins](/docs/es/mcp#plugin-provided-mcp-servers).

<h3 id="mcp-servers">
  MCP servers
</h3>

Los plugins pueden agrupar servidores Model Context Protocol (MCP) para conectar Claude Code con herramientas y servicios externos.

**Ubicación**: `.mcp.json` en la raíz del plugin, o en línea en plugin.json

**Formato**: Configuración estándar del servidor MCP

**Configuración del servidor MCP**:

```json theme={null}
{
  "mcpServers": {
    "plugin-database": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_PATH": "${CLAUDE_PLUGIN_ROOT}/data"
      }
    },
    "plugin-api-client": {
      "command": "npx",
      "args": ["@company/mcp-server", "--plugin-mode"]
    }
  }
}
```

**Comportamiento de integración**:

* Los servidores MCP del plugin se inician automáticamente cuando se habilita el plugin
* Los servidores aparecen como herramientas MCP estándar en el kit de herramientas de Claude
* Las capacidades del servidor se integran sin problemas con las herramientas existentes de Claude
* Los servidores del plugin se pueden configurar independientemente de los servidores MCP del usuario

<h3 id="lsp-servers">
  LSP servers
</h3>

<Tip>
  ¿Buscas usar plugins LSP? Instálalos desde el marketplace oficial: busca "lsp" en la pestaña Discover de `/plugin`. Esta sección documenta cómo crear plugins LSP para lenguajes no cubiertos por el marketplace oficial.
</Tip>

Los plugins pueden proporcionar servidores [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) para dar a Claude inteligencia de código en tiempo real mientras trabaja en su base de código.

La integración de LSP proporciona:

* **Diagnósticos instantáneos**: Claude ve errores y advertencias inmediatamente después de cada edición
* **Navegación de código**: ir a definición, encontrar referencias e información al pasar el ratón
* **Conciencia del lenguaje**: información de tipo y documentación para símbolos de código

**Ubicación**: `.lsp.json` en la raíz del plugin, o en línea en `plugin.json`

**Formato**: Configuración JSON que asigna nombres de servidores de lenguaje a sus configuraciones

**Formato del archivo `.lsp.json`**:

```json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

**En línea en `plugin.json`**:

```json theme={null}
{
  "name": "my-plugin",
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": {
        ".go": "go"
      }
    }
  }
}
```

**Campos requeridos:**

| Campo                 | Descripción                                                 |
| :-------------------- | :---------------------------------------------------------- |
| `command`             | El binario LSP a ejecutar (debe estar en PATH)              |
| `extensionToLanguage` | Asigna extensiones de archivo a identificadores de lenguaje |

**Campos opcionales:**

| Campo                   | Descripción                                                                                                                                                                                                                 |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `args`                  | Argumentos de línea de comandos para el servidor LSP                                                                                                                                                                        |
| `transport`             | Transporte de comunicación: `stdio` (predeterminado) o `socket`                                                                                                                                                             |
| `env`                   | Variables de entorno a establecer al iniciar el servidor                                                                                                                                                                    |
| `initializationOptions` | Opciones pasadas al servidor durante la inicialización                                                                                                                                                                      |
| `settings`              | Configuración pasada a través de `workspace/didChangeConfiguration`                                                                                                                                                         |
| `workspaceFolder`       | Ruta de carpeta de espacio de trabajo para el servidor                                                                                                                                                                      |
| `startupTimeout`        | Tiempo máximo para esperar el inicio del servidor (milisegundos)                                                                                                                                                            |
| `shutdownTimeout`       | Tiempo máximo para esperar el apagado elegante (milisegundos). Cuando se agota el tiempo de espera, Claude Code termina el proceso del servidor. Cuando no se establece, no se aplica ningún tiempo de espera               |
| `restartOnCrash`        | Si se debe reiniciar el servidor después de que se bloquee. El valor predeterminado es `true`. Establezca en `false` para dejar un servidor bloqueado detenido en lugar de reiniciarlo                                      |
| `maxRestarts`           | Número máximo de intentos de reinicio antes de rendirse                                                                                                                                                                     |
| `diagnostics`           | Si se deben insertar diagnósticos en el contexto de Claude después de ediciones (predeterminado `true`). Establezca en `false` para mantener la navegación de código pero suprimir la inyección automática de diagnósticos. |

`restartOnCrash` y `shutdownTimeout` requieren Claude Code v2.1.205 o posterior. Antes de v2.1.205, el esquema de configuración aceptaba ambas opciones pero establecer cualquiera de ellas hacía que Claude Code omitiera ese servidor LSP completamente al inicio, con la razón visible solo en la salida de `claude --debug`.

**Múltiples servidores para la misma extensión**: cuando más de un servidor LSP habilitado declara la misma extensión de archivo en `extensionToLanguage`, ya sea que los servidores provengan de un plugin o de diferentes plugins, el primer servidor registrado maneja archivos con esa extensión y los otros nunca se inician. La interfaz `/plugin` muestra una advertencia nombrando el plugin cuyo servidor está activo.

**Servidores que fallan al inicializarse**: Claude Code omite un servidor cuya configuración es inválida, por ejemplo uno que falta `command` o `extensionToLanguage`, y los otros servidores configurados aún se inician. Ejecute `claude --debug` para ver por qué se omitió un servidor.

Un servidor omitido no reclama sus extensiones de archivo, por lo que otro servidor válido que declare la misma extensión, del mismo plugin o de un plugin diferente, aún maneja esos archivos. Antes de v2.1.205, un servidor que falló al inicializarse aún reclamaba sus extensiones y bloqueaba otro servidor válido para la misma extensión.

<Warning>
  **Debe instalar el binario del servidor de lenguaje por separado.** Los plugins LSP configuran cómo Claude Code se conecta a un servidor de lenguaje, pero no incluyen el servidor en sí. Si ve `Executable not found in $PATH` en la pestaña Errors de `/plugin`, instale el binario requerido para su lenguaje.
</Warning>

**Plugins LSP disponibles:**

| Plugin              | Servidor de lenguaje       | Comando de instalación                                                                       |
| :------------------ | :------------------------- | :------------------------------------------------------------------------------------------- |
| `pyright-lsp`       | Pyright (Python)           | `pip install pyright` o `npm install -g pyright`                                             |
| `typescript-lsp`    | TypeScript Language Server | `npm install -g typescript-language-server typescript`                                       |
| `rust-analyzer-lsp` | rust-analyzer              | [Ver instalación de rust-analyzer](https://rust-analyzer.github.io/manual.html#installation) |

Instale el servidor de lenguaje primero, luego instale el plugin desde el marketplace.

<h3 id="monitors">
  Monitors
</h3>

Los plugins pueden declarar monitores de fondo que Claude Code inicia automáticamente cuando el plugin está activo. Cada monitor ejecuta un comando de shell durante la vida útil de la sesión y entrega cada línea de stdout a Claude como una notificación, para que Claude pueda reaccionar a entradas de registro, cambios de estado o eventos sondeados sin que se le pida que inicie la vigilancia por sí mismo.

Los monitores del plugin utilizan el mismo mecanismo que la [herramienta Monitor](/docs/es/tools-reference#monitor-tool) y comparten sus restricciones de disponibilidad. Se ejecutan solo en sesiones CLI interactivas, se ejecutan sin sandbox al mismo nivel de confianza que los [hooks](#hooks), y se omiten en hosts donde la herramienta Monitor no está disponible.

**Ubicación**: `monitors/monitors.json` en la raíz del plugin, o en línea en `plugin.json`

**Formato**: Array JSON de entradas de monitor

El siguiente `monitors/monitors.json` vigila un endpoint de estado de implementación y un registro de errores local:

```json theme={null}
[
  {
    "name": "deploy-status",
    "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
    "description": "Cambios de estado de implementación"
  },
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Registro de errores de la aplicación",
    "when": "on-skill-invoke:debug"
  }
]
```

Para declarar monitores en línea, establezca `experimental.monitors` en `plugin.json` en el mismo array. Para cargar desde una ruta no predeterminada, establezca `experimental.monitors` en una cadena de ruta relativa como `"./config/monitors.json"`. Los monitores son un [componente experimental](#experimental-components).

**Campos requeridos:**

| Campo         | Descripción                                                                                                                      |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Identificador único dentro del plugin. Previene procesos duplicados cuando el plugin se recarga o se invoca una skill nuevamente |
| `command`     | Comando de shell ejecutado como un proceso de fondo persistente en el directorio de trabajo de la sesión                         |
| `description` | Resumen breve de lo que se está vigilando. Se muestra en el panel de tareas y en resúmenes de notificaciones                     |

**Campos opcionales:**

| Campo  | Descripción                                                                                                                                                                                                                                        |
| :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `when` | Controla cuándo se inicia el monitor. `"always"` lo inicia al inicio de la sesión y en la recarga del plugin, y es el predeterminado. `"on-skill-invoke:<skill-name>"` lo inicia la primera vez que se distribuye la skill nombrada en este plugin |

El valor `command` soporta las [sustituciones de ruta](#environment-variables) `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PLUGIN_DATA}` y `${CLAUDE_PROJECT_DIR}`, más cualquier `${ENV_VAR}` del entorno. Prefije el comando con `cd "${CLAUDE_PLUGIN_ROOT}" && ` si el script necesita ejecutarse desde el directorio del plugin.

Un comando `command` de monitor no puede hacer referencia a valores [`${user_config.*}`](#user-configuration). El comando se ejecuta a través de un shell, por lo que Claude Code rechaza el monitor con un [error](/docs/es/errors#plugin-command-references-user-config) en lugar de sustituir el valor. Los procesos de monitor no reciben variables de entorno `CLAUDE_PLUGIN_OPTION_<KEY>`, por lo que el script de monitor debe leer el valor de un archivo de configuración que posee. Antes de v2.1.207, los comandos de monitor sustituían valores `${user_config.*}`.

Deshabilitar un plugin a mitad de sesión no detiene los monitores que ya se están ejecutando. Se detienen cuando termina la sesión.

<h3 id="themes">
  Themes
</h3>

Los plugins pueden distribuir temas de color que aparecen en `/theme` junto con los presets integrados y los temas locales del usuario. Un tema es un archivo JSON en `themes/` con un preset `base` y un mapa disperso `overrides` de tokens de color. Los temas son un [componente experimental](#experimental-components).

```json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Seleccionar un tema de plugin persiste `custom:<plugin-name>:<slug>` en la configuración del usuario. Los temas de plugin son de solo lectura; presionar `Ctrl+E` en uno en `/theme` lo copia en `~/.claude/themes/` para que el usuario pueda editar la copia.

***

<h2 id="plugin-installation-scopes">
  Alcances de instalación de plugins
</h2>

Cuando instala un plugin, elige un **alcance** que determina dónde está disponible el plugin y quién más puede usarlo:

| Alcance   | Archivo de configuración                                  | Caso de uso                                                            |
| :-------- | :-------------------------------------------------------- | :--------------------------------------------------------------------- |
| `user`    | `~/.claude/settings.json`                                 | Plugins personales disponibles en todos los proyectos (predeterminado) |
| `project` | `.claude/settings.json`                                   | Plugins de equipo compartidos a través del control de versiones        |
| `local`   | `.claude/settings.local.json`                             | Plugins específicos del proyecto, ignorados por git                    |
| `managed` | [Configuración administrada](/docs/es/settings#settings-files) | Plugins administrados (solo lectura, solo actualizar)                  |

Los plugins utilizan el mismo sistema de alcance que otras configuraciones de Claude Code. Para instrucciones de instalación y banderas de alcance, consulte [Instalar plugins](/docs/es/discover-plugins#install-plugins). Para una explicación completa de los alcances, consulte [Alcances de configuración](/docs/es/settings#configuration-scopes).

***

<h2 id="skills-directory-plugins">
  Plugins de directorio de skills
</h2>

Cualquier carpeta bajo un directorio de skills que contenga un manifiesto `.claude-plugin/plugin.json` se carga como un plugin llamado `<name>@skills-dir` en la siguiente sesión, sin marketplace y sin paso de instalación. Cree uno con [`plugin init`](#plugin-init). A diferencia de una instalación del marketplace, el plugin se descubre en su lugar en lugar de copiarse en la caché del plugin.

Un árbol de directorio de skills soporta tres cosas distintas:

| Lo que tiene                                  | Qué es                                                                                |
| :-------------------------------------------- | :------------------------------------------------------------------------------------ |
| `<skills-dir>/foo/SKILL.md` sin manifiesto    | Una [skill](/docs/es/skills) simple llamada `foo`                                          |
| `<skills-dir>/foo/.claude-plugin/plugin.json` | Un plugin `foo@skills-dir`, que puede agrupar sus propias skills, agents, hooks y más |
| `<plugin>/skills/bar/SKILL.md`                | Una skill `bar` empaquetada dentro de un plugin                                       |

<h3 id="choose-where-the-plugin-loads-from">
  Elija dónde se carga el plugin
</h3>

| Directorio de skills    | Alcance  | Se carga                                                                                                   |
| :---------------------- | :------- | :--------------------------------------------------------------------------------------------------------- |
| `~/.claude/skills/`     | personal | En cada proyecto, ya que la ubicación es solo suya                                                         |
| `<cwd>/.claude/skills/` | proyecto | Solo después de que acepte el diálogo de [confianza](/docs/es/settings) del espacio de trabajo para esa carpeta |

Un plugin de alcance de proyecto se verifica en el repositorio y llega a cada colaborador que lo clona. Porque ese contenido proviene del repositorio en lugar de usted, se carga solo después de la misma puerta de confianza que rige `.claude/settings.json`, y los componentes que ejecutan código están restringidos aún más:

* Los servidores MCP que declara pasan por la [misma aprobación por servidor](/docs/es/mcp) que un `.mcp.json` del proyecto
* Los servidores LSP se inician solo después de que confíe en el espacio de trabajo
* Los [monitores de fondo](#monitors) no se cargan

Los plugins de alcance personal no tienen ninguna de estas restricciones.

<Warning>
  Los plugins `@skills-dir` de alcance de proyecto se cargan solo desde `.claude/skills/` del directorio donde inicia Claude Code. No [caminan hasta la raíz del repositorio](/docs/es/skills#automatic-discovery-from-parent-and-nested-directories) de la forma en que lo hacen las skills y comandos simples, por lo que lanzar desde un subdirectorio pierde un plugin que vive en la raíz del repositorio. Inicie desde la raíz del repositorio, o ejecute `/reload-plugins` después de cambiar de directorio.
</Warning>

<h3 id="edit-reload-and-disable-a-skills-directory-plugin">
  Editar, recargar y deshabilitar un plugin de directorio de skills
</h3>

Los cambios que realiza en el `SKILL.md` de una skill tienen efecto inmediatamente en la sesión actual. Los cambios en otros componentes del plugin, como `hooks/`, `.mcp.json`, `agents/` y `output-styles/`, no lo hacen. Ejecute `/reload-plugins` o reinicie Claude Code para recogerlos. Consulte [Detección de cambios en vivo](/docs/es/skills#live-change-detection).

Para dejar de cargar un plugin de directorio de skills, elimine su carpeta o deshabilítelo por nombre. No hay paso de `uninstall` porque nada se instaló desde un marketplace.

```bash theme={null}
claude plugin disable my-tool@skills-dir
```

***

<h2 id="plugin-manifest-schema">
  Esquema del manifiesto del plugin
</h2>

El archivo `.claude-plugin/plugin.json` define los metadatos y la configuración de su plugin. Esta sección documenta todos los campos y opciones soportados.

El manifiesto es opcional. Si se omite, Claude Code descubre automáticamente componentes en [ubicaciones predeterminadas](#file-locations-reference) y deriva el nombre del plugin del nombre del directorio. Use un manifiesto cuando necesite proporcionar metadatos o rutas de componentes personalizadas.

<h3 id="complete-schema">
  Esquema completo
</h3>

```json theme={null}
{
  "name": "plugin-name",
  "displayName": "Plugin Name",
  "version": "1.2.0",
  "description": "Brief plugin description",
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://github.com/author"
  },
  "homepage": "https://docs.example.com/plugin",
  "repository": "https://github.com/author/plugin",
  "license": "MIT",
  "keywords": ["keyword1", "keyword2"],
  "skills": "./custom/skills/",
  "commands": ["./custom/commands/special.md"],
  "agents": ["./custom/agents/reviewer.md"],
  "hooks": "./config/hooks.json",
  "mcpServers": "./mcp-config.json",
  "outputStyles": "./styles/",
  "lspServers": "./.lsp.json",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./monitors.json"
  },
  "dependencies": [
    "helper-lib",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

<h3 id="required-fields">
  Campos requeridos
</h3>

Si incluye un manifiesto, `name` es el único campo requerido.

| Campo  | Tipo   | Descripción                                                                                                                                                                                                                                                           | Ejemplo              |
| :----- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------- |
| `name` | string | Identificador único (kebab-case, sin espacios). Cuando una [entrada del marketplace](/docs/es/plugin-marketplaces#plugin-entries) lista el plugin bajo un nombre diferente, el nombre de la entrada del marketplace es lo que usan las claves `enabledPlugins` y `/plugin` | `"deployment-tools"` |

Este nombre se utiliza para espacios de nombres de componentes. Por ejemplo, en la interfaz de usuario, el agent `agent-creator` para el plugin con nombre `plugin-dev` aparecerá como `plugin-dev:agent-creator`.

<h3 id="unrecognized-fields">
  Campos no reconocidos
</h3>

Claude Code ignora los campos de nivel superior que no reconoce. Puede mantener metadatos de otro ecosistema en `plugin.json` y el plugin aún se carga. Esto hace que sea práctico mantener un manifiesto que funcione como un manifiesto de extensión de VS Code o Cursor, un `package.json` de npm, o un manifiesto de paquete MCPB/DXT.

`claude plugin validate` reporta campos no reconocidos como advertencias, no como errores. Si un campo está a uno o dos caracteres de uno reconocido, la advertencia sugiere el nombre probable previsto. Un plugin con solo advertencias de campos no reconocidos aún pasa la validación y se carga en tiempo de ejecución.

Los campos con el tipo incorrecto aún fallan. Por ejemplo, un valor `keywords` que es una cadena en lugar de un array es un error de carga, y `claude plugin validate` lo reporta como tal.

Pase `--strict` para tratar las advertencias como errores. Úselo en CI para detectar un nombre de campo mal escrito o un campo dejado de otra herramienta de manifiesto antes de publicar, aunque el plugin se cargue en tiempo de ejecución.

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h3 id="metadata-fields">
  Campos de metadatos
</h3>

| Campo            | Tipo    | Descripción                                                                                                                                                                                                                                                                                                                                                                                                         | Ejemplo                                                           |
| :--------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------- |
| `$schema`        | string  | URL del esquema JSON para autocompletado y validación del editor. Claude Code ignora este campo en el momento de la carga.                                                                                                                                                                                                                                                                                          | `"https://json.schemastore.org/claude-code-plugin-manifest.json"` |
| `displayName`    | string  | Nombre legible por humanos mostrado en el selector `/plugin` y otras superficies de interfaz de usuario. Recurre a `name` cuando se omite. A diferencia de `name`, puede contener espacios y cualquier capitalización. No se utiliza para espacios de nombres o búsqueda. Requiere Claude Code v2.1.143 o posterior.                                                                                                | `"Deployment Tools"`                                              |
| `version`        | string  | Opcional. Versión semántica. Establecer esto fija el plugin a esa cadena de versión, por lo que los usuarios solo reciben actualizaciones cuando la incrementa. Si se omite, Claude Code recurre al SHA del commit de git, por lo que cada commit se trata como una nueva versión. Si también se establece en la entrada del marketplace, `plugin.json` gana. Consulte [Gestión de versiones](#version-management). | `"2.1.0"`                                                         |
| `description`    | string  | Explicación breve del propósito del plugin                                                                                                                                                                                                                                                                                                                                                                          | `"Deployment automation tools"`                                   |
| `author`         | object  | Información del autor                                                                                                                                                                                                                                                                                                                                                                                               | `{"name": "Dev Team", "email": "dev@company.com"}`                |
| `homepage`       | string  | URL de documentación                                                                                                                                                                                                                                                                                                                                                                                                | `"https://docs.example.com"`                                      |
| `repository`     | string  | URL del código fuente                                                                                                                                                                                                                                                                                                                                                                                               | `"https://github.com/user/plugin"`                                |
| `license`        | string  | Identificador de licencia                                                                                                                                                                                                                                                                                                                                                                                           | `"MIT"`, `"Apache-2.0"`                                           |
| `keywords`       | array   | Etiquetas de descubrimiento                                                                                                                                                                                                                                                                                                                                                                                         | `["deployment", "ci-cd"]`                                         |
| `defaultEnabled` | boolean | Si el plugin se inicia en un estado habilitado cuando el usuario no ha establecido uno. Predeterminado a `true`. Consulte [Habilitación predeterminada](#default-enablement). Requiere Claude Code v2.1.154 o posterior.                                                                                                                                                                                            | `false`                                                           |

<h3 id="default-enablement">
  Habilitación predeterminada
</h3>

Establezca `defaultEnabled: false` en `plugin.json` para enviar un plugin que se instale deshabilitado. El usuario lo activa con `claude plugin enable <plugin>` o la interfaz `/plugin`. Úselo para plugins que añaden costo o alcance que un usuario debe optar por usar, como uno que se conecta a un servicio externo. Esto requiere Claude Code v2.1.154 o posterior. Las versiones anteriores ignoran el campo y habilitan el plugin en la instalación.

`defaultEnabled` es la alternativa cuando nada más ha decidido el estado del plugin. Dos cosas tienen prioridad sobre él:

* **La configuración del usuario**: una entrada para el plugin en `enabledPlugins` en cualquier alcance de configuración. Una vez escrita, persiste en actualizaciones y reinstalaciones del plugin, por lo que cambiar `defaultEnabled` en una versión posterior no cambia un usuario existente.
* **Un requisito de dependencia**: cuando un plugin es requerido por otro que está activo, Claude Code escribe `true` para él en el tiempo de instalación o habilitación. Eso le da una configuración explícita, por lo que su propio predeterminado ya no se aplica. Consulte [Habilitar o deshabilitar un plugin con dependencias](/docs/es/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies).

El mismo campo puede aparecer en la entrada del marketplace de un plugin, donde tiene prioridad sobre el valor en `plugin.json`. Consulte [Campos de plugin opcionales](/docs/es/plugin-marketplaces#optional-plugin-fields).

<h3 id="component-path-fields">
  Campos de ruta de componentes
</h3>

| Campo                   | Tipo                  | Descripción                                                                                                                                                                                                              | Ejemplo                                              |
| :---------------------- | :-------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `skills`                | string\|array         | Directorios de skills personalizados que contienen `<name>/SKILL.md`. Se añade al predeterminado `skills/`. Consulte [Reglas de comportamiento de rutas](#path-behavior-rules) para la excepción de raíz del marketplace | `"./custom/skills/"`                                 |
| `commands`              | string\|array         | Archivos de skill planos `.md` o directorios personalizados (reemplaza el predeterminado `commands/`)                                                                                                                    | `"./custom/cmd.md"` o `["./cmd1.md"]`                |
| `agents`                | string\|array         | Archivos de agent personalizados (reemplaza el predeterminado `agents/`)                                                                                                                                                 | `"./custom/agents/reviewer.md"`                      |
| `hooks`                 | string\|array\|object | Rutas de configuración de hooks o configuración en línea                                                                                                                                                                 | `"./my-extra-hooks.json"`                            |
| `mcpServers`            | string\|array\|object | Rutas de configuración de MCP o configuración en línea                                                                                                                                                                   | `"./my-extra-mcp-config.json"`                       |
| `outputStyles`          | string\|array         | Archivos/directorios de estilos de salida personalizados (reemplaza el predeterminado `output-styles/`)                                                                                                                  | `"./styles/"`                                        |
| `lspServers`            | string\|array\|object | Configuraciones de [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) para inteligencia de código (ir a definición, encontrar referencias, etc.)                                          | `"./.lsp.json"`                                      |
| `experimental.themes`   | string\|array         | Archivos/directorios de temas de color (reemplaza el predeterminado `themes/`). Consulte [Themes](#themes)                                                                                                               | `"./themes/"`                                        |
| `experimental.monitors` | string\|array         | Configuraciones de [Monitor](/docs/es/tools-reference#monitor-tool) de fondo que se inician automáticamente cuando el plugin está activo. Consulte [Monitors](#monitors)                                                      | `"./monitors.json"`                                  |
| `userConfig`            | object                | Valores configurables por el usuario solicitados al habilitar. Consulte [Configuración del usuario](#user-configuration)                                                                                                 | Ver abajo                                            |
| `channels`              | array                 | Declaraciones de canales para inyección de mensajes (estilo Telegram, Slack, Discord). Consulte [Channels](#channels)                                                                                                    | Ver abajo                                            |
| `dependencies`          | array                 | Otros plugins que requiere este plugin, opcionalmente con restricciones de versión semántica. Consulte [Restringir versiones de dependencias de plugins](/docs/es/plugin-dependencies)                                        | `[{ "name": "secrets-vault", "version": "~2.1.0" }]` |

<h3 id="experimental-components">
  Componentes experimentales
</h3>

Los componentes bajo la clave `experimental`, `themes` y `monitors`, tienen un esquema de manifiesto que puede cambiar entre versiones mientras se estabilizan. Dónde los declare es una migración separada: el nivel superior aún funciona, `claude plugin validate` advierte, y una versión futura requerirá `experimental.*`.

<h3 id="user-configuration">
  Configuración del usuario
</h3>

El campo `userConfig` declara valores que Claude Code solicita al usuario cuando se habilita el plugin. Use esto en lugar de requerir que los usuarios editen manualmente `settings.json`.

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Su endpoint de API del equipo"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token de autenticación de API",
      "sensitive": true
    }
  }
}
```

Las claves deben ser identificadores válidos. Cada opción soporta estos campos:

| Campo         | Requerido | Descripción                                                                                                 |
| :------------ | :-------- | :---------------------------------------------------------------------------------------------------------- |
| `type`        | Sí        | Uno de `string`, `number`, `boolean`, `directory`, o `file`                                                 |
| `title`       | Sí        | Etiqueta mostrada en el diálogo de configuración                                                            |
| `description` | Sí        | Texto de ayuda mostrado debajo del campo                                                                    |
| `sensitive`   | No        | Si es `true`, enmascara la entrada y almacena el valor en almacenamiento seguro en lugar de `settings.json` |
| `required`    | No        | Si es `true`, la validación falla cuando el campo está vacío                                                |
| `default`     | No        | Valor utilizado cuando el usuario no proporciona nada                                                       |
| `multiple`    | No        | Para tipo `string`, permite un array de cadenas                                                             |
| `min` / `max` | No        | Límites para tipo `number`                                                                                  |

Cada valor está disponible para sustitución como `${user_config.KEY}` en configuraciones de servidores MCP y LSP, comandos de hooks y comandos de monitores. Los valores no sensibles también pueden sustituirse en contenido de skills y agents. Todos los valores se exportan a procesos de hooks y subprocesos de servidores MCP y LSP como variables de entorno `CLAUDE_PLUGIN_OPTION_<KEY>`, donde `<KEY>` es la clave de opción en mayúsculas.

Los campos que se ejecutan en un shell rechazan `${user_config.*}`: sustituir un valor configurado en un comando shell permitiría que el shell ejecute lo que contenga ese valor, por lo que el componente falla con un [error](/docs/es/errors#plugin-command-references-user-config) en su lugar. Cada campo rechazado tiene una forma alternativa de pasar el valor:

| Campo rechazado                                                              | Cómo pasar el valor                                                                                                      |
| :--------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| Comandos de hooks de forma shell                                             | Use [forma exec](/docs/es/hooks#exec-form-and-shell-form) con `args`, o lea `CLAUDE_PLUGIN_OPTION_<KEY>` del entorno del hook |
| Comandos de [Monitor](#monitors)                                             | Lea el valor de un archivo de configuración en el script                                                                 |
| MCP [`headersHelper`](/docs/es/mcp#use-dynamic-headers-for-custom-authentication) | Lea el valor de un archivo de configuración en el script                                                                 |

Antes de v2.1.207, estos campos sustituían valores `${user_config.KEY}`; actualice los plugins que dependían de esto.

Los valores no sensibles se almacenan bajo la clave [`pluginConfigs`](/docs/es/settings#pluginconfigs) en `settings.json` como `pluginConfigs[<plugin-id>].options`. Claude Code escribe la clave en la configuración del usuario y la lee desde la configuración del usuario, la bandera `--settings` y la configuración administrada solo; las entradas en `.claude/settings.json` o `.claude/settings.local.json` de un proyecto se ignoran. Antes de v2.1.207, Claude Code también leía la configuración del proyecto y local.

Los valores sensibles van al Keychain de macOS, o a `~/.claude/.credentials.json` en plataformas donde no hay un keychain soportado disponible. El almacenamiento en keychain se comparte con tokens OAuth y tiene un límite total aproximado de 2 KB, así que mantenga los valores sensibles pequeños.

<h3 id="channels">
  Canales
</h3>

El campo `channels` permite que un plugin declare uno o más canales de mensajes que inyecten contenido en la conversación. Cada canal se vincula a un servidor MCP que proporciona el plugin.

```json theme={null}
{
  "channels": [
    {
      "server": "telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Token del bot de Telegram",
          "sensitive": true
        },
        "owner_id": {
          "type": "string",
          "title": "Owner ID",
          "description": "Su ID de usuario de Telegram"
        }
      }
    }
  ]
}
```

El campo `server` es requerido y debe coincidir con una clave en los `mcpServers` del plugin. El `userConfig` opcional por canal usa el mismo esquema que el campo de nivel superior, permitiendo que el plugin solicite tokens de bot o IDs de propietario cuando se habilita el plugin.

<h3 id="path-behavior-rules">
  Reglas de comportamiento de rutas
</h3>

Si una ruta personalizada reemplaza o extiende el directorio predeterminado del plugin depende del campo:

* **Reemplaza el predeterminado**: `commands`, `agents`, `outputStyles`, `experimental.themes`, `experimental.monitors`. Por ejemplo, cuando el manifiesto especifica `commands`, el directorio predeterminado `commands/` no se escanea. Para mantener el predeterminado y añadir más, enumérelo explícitamente: `"commands": ["./commands/", "./extras/"]`
* **Se añade al predeterminado**: `skills`. El directorio predeterminado `skills/` siempre se escanea, y los directorios enumerados en `skills` se cargan junto a él. Excepción: para una [entrada del marketplace cuya `source` se resuelve a la raíz del marketplace](/docs/es/plugin-marketplaces#advanced-plugin-entries), declarar subdirectorios específicos reemplaza el escaneo predeterminado de `skills/`
* **Reglas de fusión propias**: [hooks](#hooks), [MCP servers](#mcp-servers) y [LSP servers](#lsp-servers). Consulte cada sección para ver cómo se combinan múltiples fuentes

Cuando un plugin tiene tanto una carpeta predeterminada como la clave de manifiesto coincidente, Claude Code v2.1.140 y posterior marca la carpeta ignorada en `claude plugin list` y la vista de detalles `/plugin`. El plugin aún se carga usando las rutas del manifiesto. No se muestra advertencia cuando la clave del manifiesto apunta a la carpeta predeterminada, por ejemplo `"commands": ["./commands/deploy.md"]`, porque la carpeta se aborda explícitamente en ese caso.

Para todos los campos de ruta:

* Todas las rutas deben ser relativas a la raíz del plugin y comenzar con `./`
* Los componentes de rutas personalizadas utilizan las mismas reglas de nomenclatura y espacios de nombres
* Se pueden especificar múltiples rutas como arrays
* Cuando una ruta de skill apunta a un directorio que contiene un `SKILL.md` directamente, por ejemplo `"skills": ["./"]` apuntando a la raíz del plugin, el campo frontmatter `name` en `SKILL.md` determina el nombre de invocación de la skill. Esto proporciona un nombre estable independientemente del directorio de instalación. Si `name` no se establece en el frontmatter, el nombre base del directorio se usa como alternativa.

Un plugin que tiene un `SKILL.md` en su raíz, sin subdirectorio `skills/`, y sin campo de manifiesto `skills` se carga automáticamente como un plugin de una sola skill en Claude Code v2.1.142 y posterior. No necesita establecer `"skills": ["./"]` en `plugin.json` para este diseño. El nombre de invocación de la skill sigue la misma regla que arriba: el campo frontmatter `name`, o el nombre base del directorio como alternativa.

**Ejemplos de rutas**:

```json theme={null}
{
  "commands": [
    "./specialized/deploy.md",
    "./utilities/batch-process.md"
  ],
  "agents": [
    "./custom-agents/reviewer.md",
    "./custom-agents/tester.md"
  ]
}
```

<h3 id="environment-variables">
  Variables de entorno
</h3>

Claude Code proporciona tres variables para hacer referencia a rutas:

| Variable                | Se resuelve a                                                                                                                        | Úsela para                                                                                           |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | Ruta absoluta al directorio de instalación del plugin                                                                                | Scripts, binarios y archivos de configuración incluidos con el plugin                                |
| `${CLAUDE_PLUGIN_DATA}` | [Directorio persistente](#persistent-data-directory) que sobrevive a las actualizaciones del plugin, creado en la primera referencia | Dependencias instaladas como `node_modules` o entornos virtuales de Python, código generado y cachés |
| `${CLAUDE_PROJECT_DIR}` | La raíz del proyecto                                                                                                                 | Scripts y archivos de configuración locales del proyecto                                             |

Los tres se exportan como variables de entorno a procesos de hooks y a subprocesos de servidores MCP y LSP. Qué campos sustituyen en línea depende del componente del plugin:

| Componente del plugin              | Campos donde se resuelven los placeholders       |
| :--------------------------------- | :----------------------------------------------- |
| Contenido de skill y agent         | En cualquier lugar donde aparezca el placeholder |
| Comandos de hook y monitor         | En cualquier lugar donde aparezca el placeholder |
| Servidores MCP `stdio`             | `command`, `args`, `env`                         |
| Servidores MCP `http`, `sse`, `ws` | `url`, `headers`, `headersHelper`                |
| Servidores LSP                     | `command`, `args`, `env`, `workspaceFolder`      |

En comandos de hooks, use [forma exec](/docs/es/hooks#exec-form-and-shell-form) con `args` para que cada ruta se pase como un argumento sin comillas. En hooks de forma shell y comandos de monitores, envuélvalo en comillas dobles, como en `"${CLAUDE_PROJECT_DIR}/scripts/server.sh"`. Este hook de forma shell ejecuta un script incluido con un plugin:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

`${CLAUDE_PLUGIN_ROOT}` cambia cuando se actualiza el plugin. El directorio de la versión anterior permanece en el disco durante aproximadamente siete días después de una actualización antes de la limpieza, pero trátelo como efímero y no escriba estado aquí.

Cuando un plugin se actualiza a mitad de sesión, los comandos de hooks, monitores, servidores MCP y servidores LSP siguen usando la ruta de la versión anterior. Ejecute `/reload-plugins` para cambiar hooks, servidores MCP y servidores LSP a la nueva ruta; los monitores requieren un reinicio de sesión.

Los servidores MCP también pueden llamar a la solicitud `roots/list` para leer los directorios de trabajo de la sesión en tiempo de ejecución. Consulte [qué devuelve `roots/list` y cuándo Claude Code notifica al servidor de cambios](/docs/es/mcp#option-3-add-a-local-stdio-server).

<h4 id="persistent-data-directory">
  Directorio de datos persistente
</h4>

El directorio `${CLAUDE_PLUGIN_DATA}` se resuelve a `~/.claude/plugins/data/{id}/`, donde `{id}` es el identificador del plugin con caracteres fuera de `a-z`, `A-Z`, `0-9`, `_` y `-` reemplazados por `-`. Para un plugin instalado como `formatter@my-marketplace`, el directorio es `~/.claude/plugins/data/formatter-my-marketplace/`.

Un uso común es instalar dependencias de lenguaje una vez y reutilizarlas en sesiones y actualizaciones de plugins. Porque el directorio de datos sobrevive a cualquier versión única del plugin, una verificación de existencia de directorio solo no puede detectar cuándo una actualización cambia el manifiesto de dependencias del plugin. El patrón recomendado compara el manifiesto incluido contra una copia en el directorio de datos y reinstala cuando difieren.

Este hook `SessionStart` instala `node_modules` en la primera ejecución y nuevamente siempre que una actualización del plugin incluya un `package.json` cambiado:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "diff -q \"${CLAUDE_PLUGIN_ROOT}/package.json\" \"${CLAUDE_PLUGIN_DATA}/package.json\" >/dev/null 2>&1 || (cd \"${CLAUDE_PLUGIN_DATA}\" && cp \"${CLAUDE_PLUGIN_ROOT}/package.json\" . && npm install) || rm -f \"${CLAUDE_PLUGIN_DATA}/package.json\""
          }
        ]
      }
    ]
  }
}
```

El `diff` sale con código distinto de cero cuando la copia almacenada falta o difiere de la incluida, cubriendo tanto la primera ejecución como las actualizaciones que cambian dependencias. Si `npm install` falla, el `rm` final elimina el manifiesto copiado para que la siguiente sesión reintente.

Los scripts incluidos en `${CLAUDE_PLUGIN_ROOT}` pueden ejecutarse contra los `node_modules` persistidos:

```json theme={null}
{
  "mcpServers": {
    "routines": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": {
        "NODE_PATH": "${CLAUDE_PLUGIN_DATA}/node_modules"
      }
    }
  }
}
```

El directorio de datos se elimina automáticamente cuando desinstala el plugin del último alcance donde está instalado. La interfaz `/plugin` muestra el tamaño del directorio y solicita confirmación antes de eliminar. La CLI elimina por defecto; pase [`--keep-data`](#plugin-uninstall) para preservarlo.

***

<h2 id="plugin-caching-and-file-resolution">
  Almacenamiento en caché de plugins y resolución de archivos
</h2>

Los plugins se especifican de una de dos formas:

* A través de `claude --plugin-dir` o `claude --plugin-url`, durante la duración de una sesión.
* A través de un marketplace, instalado para sesiones futuras.

Por razones de seguridad y verificación, Claude Code copia plugins del *marketplace* a la **caché de plugins** local del usuario (`~/.claude/plugins/cache`) en lugar de usarlos en su lugar. Entender este comportamiento es importante al desarrollar plugins que hacen referencia a archivos externos.

Cada versión instalada es un directorio separado en la caché. Cuando actualiza o desinstala un plugin, el directorio de versión anterior se marca como huérfano y se elimina automáticamente 7 días después. El período de gracia permite que las sesiones de Claude Code concurrentes que ya cargaron la versión anterior sigan ejecutándose sin errores.

Las herramientas Glob y Grep de Claude omiten directorios de versión huérfanos durante búsquedas, por lo que los resultados de archivos no incluyen código de plugin obsoleto.

<h3 id="path-traversal-limitations">
  Limitaciones de traversal de rutas
</h3>

Los plugins instalados no pueden hacer referencia a archivos fuera de su directorio. Las rutas que traversan fuera de la raíz del plugin (como `../shared-utils`) no funcionarán después de la instalación porque esos archivos externos no se copian a la caché.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Compartir archivos dentro de un marketplace con enlaces simbólicos
</h3>

Si su plugin necesita compartir archivos con otras partes del mismo marketplace, puede crear enlaces simbólicos dentro de su directorio de plugin. La forma en que se maneja un enlace simbólico cuando el plugin se copia en la caché depende de dónde se resuelva su destino:

* **Dentro del directorio propio del plugin:** el enlace simbólico se preserva como un enlace simbólico relativo en la caché, por lo que sigue resolviendo al destino copiado en tiempo de ejecución.
* **En otro lugar dentro del mismo marketplace:** el enlace simbólico se desreferencia. El contenido del destino se copia en la caché en su lugar. Esto permite que el directorio `skills/` de un meta-plugin se vincule a skills definidas por otros plugins en el marketplace.
* **Fuera del marketplace:** el enlace simbólico se omite por seguridad. Esto evita que los plugins extraigan archivos arbitrarios del host, como rutas del sistema, en la caché.

Para plugins instalados con `--plugin-dir` o desde una ruta local, solo se preservan los enlaces simbólicos que se resuelven dentro del directorio propio del plugin. Todos los demás se omiten.

El siguiente comando crea un enlace desde dentro de un plugin del marketplace a una skill compartida definida por un plugin hermano. En Windows, use `mklink /D` desde un símbolo del sistema elevado o habilite el Modo de desarrollador:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

Esto proporciona flexibilidad mientras se mantienen los beneficios de seguridad del sistema de almacenamiento en caché.

***

<h2 id="plugin-directory-structure">
  Estructura del directorio del plugin
</h2>

<h3 id="standard-plugin-layout">
  Diseño estándar del plugin
</h3>

Un plugin completo sigue esta estructura:

```text theme={null}
enterprise-plugin/
├── .claude-plugin/           # Directorio de metadatos (opcional)
│   └── plugin.json             # manifiesto del plugin
├── skills/                   # Skills
│   ├── code-reviewer/
│   │   └── SKILL.md
│   └── pdf-processor/
│       ├── SKILL.md
│       └── scripts/
├── commands/                 # Skills como archivos .md planos
│   ├── status.md
│   └── logs.md
├── agents/                   # Definiciones de subagent
│   ├── security-reviewer.md
│   ├── performance-tester.md
│   └── compliance-checker.md
├── output-styles/            # Definiciones de estilo de salida
│   └── terse.md
├── themes/                   # Definiciones de tema de color
│   └── dracula.json
├── monitors/                 # Configuraciones de monitor de fondo
│   └── monitors.json
├── hooks/                    # Configuraciones de hooks
│   ├── hooks.json           # Configuración principal de hooks
│   └── security-hooks.json  # Hooks adicionales
├── bin/                      # Ejecutables del plugin añadidos a PATH
│   └── my-tool               # Invocable como comando desnudo en herramienta Bash
├── settings.json            # Configuración predeterminada para el plugin
├── .mcp.json                # Definiciones del servidor MCP
├── .lsp.json                # Configuraciones del servidor LSP
├── scripts/                 # Scripts de hooks y utilidades
│   ├── security-scan.sh
│   ├── format-code.py
│   └── deploy.js
├── LICENSE                  # Archivo de licencia
└── CHANGELOG.md             # Historial de versiones
```

<Warning>
  El directorio `.claude-plugin/` contiene el archivo `plugin.json`. Todos los otros directorios (commands/, agents/, skills/, output-styles/, themes/, monitors/, hooks/) deben estar en la raíz del plugin, no dentro de `.claude-plugin/`.
</Warning>

Un archivo `CLAUDE.md` en la raíz del plugin no se carga como contexto del proyecto. Los plugins contribuyen contexto a través de skills, agents y hooks en lugar de CLAUDE.md. Para enviar instrucciones que se carguen en el contexto de Claude, colóquelas en un [skill](#skills).

<h3 id="file-locations-reference">
  Referencia de ubicaciones de archivos
</h3>

| Componente            | Ubicación predeterminada     | Propósito                                                                                                                                                                                            |
| :-------------------- | :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Manifiesto**        | `.claude-plugin/plugin.json` | Metadatos y configuración del plugin (opcional)                                                                                                                                                      |
| **Skills**            | `skills/`                    | Skills con estructura `<name>/SKILL.md`                                                                                                                                                              |
| **Comandos**          | `commands/`                  | Skills como archivos Markdown planos. Use `skills/` para nuevos plugins                                                                                                                              |
| **Agents**            | `agents/`                    | Archivos Markdown de Subagent                                                                                                                                                                        |
| **Estilos de salida** | `output-styles/`             | Definiciones de estilo de salida                                                                                                                                                                     |
| **Temas**             | `themes/`                    | Definiciones de tema de color                                                                                                                                                                        |
| **Hooks**             | `hooks/hooks.json`           | Configuración de hooks                                                                                                                                                                               |
| **Servidores MCP**    | `.mcp.json`                  | Definiciones del servidor MCP                                                                                                                                                                        |
| **Servidores LSP**    | `.lsp.json`                  | Configuraciones del servidor de lenguaje                                                                                                                                                             |
| **Monitores**         | `monitors/monitors.json`     | Configuraciones de monitor de fondo                                                                                                                                                                  |
| **Ejecutables**       | `bin/`                       | Ejecutables añadidos al `PATH` de la herramienta Bash. Los archivos aquí son invocables como comandos desnudos en cualquier llamada de herramienta Bash mientras el plugin está habilitado           |
| **Configuración**     | `settings.json`              | Configuración predeterminada aplicada cuando se habilita el plugin. Actualmente solo se soportan las claves [`agent`](/docs/es/sub-agents) y [`subagentStatusLine`](/docs/es/statusline#subagent-status-lines) |

***

<h2 id="cli-commands-reference">
  Referencia de comandos CLI
</h2>

Claude Code proporciona comandos CLI para la gestión de plugins no interactiva, útil para scripting y automatización.

<h3 id="plugin-init">
  plugin init
</h3>

Cree un nuevo plugin en `~/.claude/skills/<name>/`. En la siguiente sesión de Claude Code se carga automáticamente como `<name>@skills-dir` y aparece en `/plugin` y `claude plugin list` sin paso de instalación.

Consulte [Plugins de directorio de skills](#skills-directory-plugins) para requisitos de alcance y confianza.

```bash theme={null}
claude plugin init <name> [options]
```

**Argumentos:**

* `<name>`: Nombre del plugin. Se convierte en el espacio de nombres de la skill y el nombre del directorio bajo `~/.claude/skills/`, por lo que no puede contener espacios ni separadores de ruta.

**Opciones:**

| Opción                   | Descripción                                                                                                                  | Predeterminado          |
| :----------------------- | :--------------------------------------------------------------------------------------------------------------------------- | :---------------------- |
| `--description <text>`   | Descripción del manifiesto                                                                                                   |                         |
| `--author <name>`        | Nombre del autor                                                                                                             | `git config user.name`  |
| `--author-email <email>` | Email del autor                                                                                                              | `git config user.email` |
| `--with <components...>` | También crear carpetas de componentes. Valores válidos: `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style`, `channel` |                         |
| `-f, --force`            | Sobrescribir un `.claude-plugin/` existente en el destino                                                                    |                         |
| `-h, --help`             | Mostrar ayuda para el comando                                                                                                |                         |

**Alias:** `new`

Cada valor `--with` añade un archivo de inicio para ese componente, listo para editar:

| Componente     | Lo que crea                                                                                                 |
| :------------- | :---------------------------------------------------------------------------------------------------------- |
| `skills`       | Una skill `<name>:example` adicional con espacio de nombres junto a la predeterminada                       |
| `agents`       | Una definición de subagent en `agents/`                                                                     |
| `hooks`        | Un `hooks/hooks.json` con un manejador de eventos de ejemplo                                                |
| `mcp`          | Un `.mcp.json` con ejemplos de servidor HTTP y stdio                                                        |
| `lsp`          | Un ejemplo de `.lsp.json` de servidor de lenguaje                                                           |
| `output-style` | Un `output-styles/<name>.md` que se aplica automáticamente mientras el plugin está habilitado               |
| `channel`      | Un [canal](/docs/es/channels) basado en MCP: un servidor stdio (`server.ts`), su `.mcp.json` y un `package.json` |

El plugin creado usa la fuente `@skills-dir` en lugar de un marketplace. Los administradores pueden bloquear esta fuente con `strictKnownMarketplaces` o añadiendo `{"source": "skills-dir"}` a `blockedMarketplaces` en [configuración administrada](/docs/es/plugin-marketplaces#managed-marketplace-restrictions). Cuando se bloquea, `plugin init` falla antes de escribir.

**Ejemplos:**

```bash theme={null}
# Crear un plugin mínimo
claude plugin init my-helper

# Crear con carpetas de skill y hook
claude plugin init my-helper --with skills hooks

# Sobrescribir un scaffold existente
claude plugin init my-helper --force
```

<h3 id="plugin-install">
  plugin install
</h3>

Instala un plugin desde los marketplaces disponibles.

```bash theme={null}
claude plugin install <plugin> [options]
```

**Argumentos:**

* `<plugin>`: Nombre del plugin o `plugin-name@marketplace-name` para un marketplace específico

**Opciones:**

| Opción                | Descripción                                          | Predeterminado |
| :-------------------- | :--------------------------------------------------- | :------------- |
| `-s, --scope <scope>` | Alcance de instalación: `user`, `project`, o `local` | `user`         |
| `-h, --help`          | Mostrar ayuda para el comando                        |                |

El alcance determina qué archivo de configuración se añade el plugin instalado. Por ejemplo, `--scope project` escribe en `enabledPlugins` en .claude/settings.json, haciendo que el plugin esté disponible para todos los que clonan el repositorio del proyecto.

**Ejemplos:**

```bash theme={null}
# Instalar en alcance de usuario (predeterminado)
claude plugin install formatter@my-marketplace

# Instalar en alcance de proyecto (compartido con el equipo)
claude plugin install formatter@my-marketplace --scope project

# Instalar en alcance local (ignorado por git)
claude plugin install formatter@my-marketplace --scope local
```

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

Elimina un plugin instalado.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

**Argumentos:**

* `<plugin>`: Nombre del plugin o `plugin-name@marketplace-name`

**Opciones:**

| Opción                | Descripción                                                                                                                          | Predeterminado |
| :-------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------- |
| `-s, --scope <scope>` | Desinstalar del alcance: `user`, `project`, o `local`                                                                                | `user`         |
| `--keep-data`         | Preservar el directorio de datos persistente del plugin                                                                              |                |
| `--prune`             | También eliminar las dependencias instaladas automáticamente que ningún otro plugin requiere. Consulte [plugin prune](#plugin-prune) |                |
| `-y, --yes`           | Omitir la solicitud de confirmación de `--prune`. Requerido cuando stdin no es un TTY                                                |                |
| `-h, --help`          | Mostrar ayuda para el comando                                                                                                        |                |

**Alias:** `remove`, `rm`

Por defecto, desinstalar del último alcance restante también elimina el directorio `${CLAUDE_PLUGIN_DATA}` del plugin. Use `--keep-data` para preservarlo, por ejemplo cuando reinstale después de probar una nueva versión.

<h3 id="plugin-prune">
  plugin prune
</h3>

Elimina las dependencias de plugins instaladas automáticamente que ya no son requeridas por ningún plugin instalado. Las dependencias que Claude Code incluyó para satisfacer el campo [`dependencies`](/docs/es/plugin-dependencies) de otro plugin se eliminan; los plugins que instaló directamente nunca se tocan.

```bash theme={null}
claude plugin prune [options]
```

**Opciones:**

| Opción                | Descripción                                                              | Predeterminado |
| :-------------------- | :----------------------------------------------------------------------- | :------------- |
| `-s, --scope <scope>` | Limpiar en alcance: `user`, `project`, o `local`                         | `user`         |
| `--dry-run`           | Listar lo que se eliminaría sin eliminar nada                            |                |
| `-y, --yes`           | Omitir la solicitud de confirmación. Requerido cuando stdin no es un TTY |                |
| `-h, --help`          | Mostrar ayuda para el comando                                            |                |

**Alias:** `autoremove`

El comando lista las dependencias huérfanas y solicita confirmación antes de eliminarlas. Para eliminar un plugin y limpiar sus dependencias en un paso, ejecute `claude plugin uninstall <plugin> --prune`.

<Note>
  `claude plugin prune` requiere Claude Code v2.1.121 o posterior.
</Note>

<h3 id="plugin-enable">
  plugin enable
</h3>

Habilita un plugin deshabilitado. Si el plugin declara [dependencias](/docs/es/plugin-dependencies), Claude Code las habilita transitivamente en el mismo alcance, y el comando falla cuando una dependencia no está instalada.

```bash theme={null}
claude plugin enable <plugin> [options]
```

**Argumentos:**

* `<plugin>`: Nombre del plugin o `plugin-name@marketplace-name`

**Opciones:**

| Opción                | Descripción                                       | Predeterminado |
| :-------------------- | :------------------------------------------------ | :------------- |
| `-s, --scope <scope>` | Alcance a habilitar: `user`, `project`, o `local` | `user`         |
| `-h, --help`          | Mostrar ayuda para el comando                     |                |

<h3 id="plugin-disable">
  plugin disable
</h3>

Deshabilita un plugin sin desinstalarlo. Falla cuando otro plugin habilitado [depende de](/docs/es/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies) el objetivo. El mensaje de error incluye un comando encadenado que deshabilita primero cada dependiente.

```bash theme={null}
claude plugin disable <plugin> [options]
```

**Argumentos:**

* `<plugin>`: Nombre del plugin o `plugin-name@marketplace-name`

**Opciones:**

| Opción                | Descripción                                          | Predeterminado |
| :-------------------- | :--------------------------------------------------- | :------------- |
| `-s, --scope <scope>` | Alcance a deshabilitar: `user`, `project`, o `local` | `user`         |
| `-h, --help`          | Mostrar ayuda para el comando                        |                |

<h3 id="plugin-update">
  plugin update
</h3>

Actualiza un plugin a la versión más reciente.

```bash theme={null}
claude plugin update <plugin> [options]
```

**Argumentos:**

* `<plugin>`: Nombre del plugin o `plugin-name@marketplace-name`

**Opciones:**

| Opción                | Descripción                                                   | Predeterminado |
| :-------------------- | :------------------------------------------------------------ | :------------- |
| `-s, --scope <scope>` | Alcance a actualizar: `user`, `project`, `local`, o `managed` | `user`         |
| `-h, --help`          | Mostrar ayuda para el comando                                 |                |

***

<h3 id="plugin-list">
  plugin list
</h3>

Lista los plugins instalados con su versión, marketplace de origen y estado de habilitación.

```bash theme={null}
claude plugin list [options]
```

**Opciones:**

| Opción        | Descripción                                                       | Predeterminado |
| :------------ | :---------------------------------------------------------------- | :------------- |
| `--json`      | Salida como JSON                                                  |                |
| `--available` | Incluir plugins disponibles desde marketplaces. Requiere `--json` |                |
| `-h, --help`  | Mostrar ayuda para el comando                                     |                |

Dentro de una sesión interactiva, `/plugin list` imprime el mismo listado en línea. La forma interactiva acepta `--enabled` o `--disabled` para mostrar solo plugins en ese estado, y `ls` como abreviatura de `list`.

<h3 id="plugin-details">
  plugin details
</h3>

Muestra el inventario de componentes de un plugin y el costo de tokens proyectado. La salida lista todos los componentes que el plugin contribuye, agrupados como Skills, Agents, Hooks, servidores MCP y servidores LSP, junto con una estimación de cuántos tokens añade a cada sesión. El grupo Skills incluye tanto entradas de `skills/` como de `commands/`.

```bash theme={null}
claude plugin details <name>
```

**Argumentos:**

* `<name>`: Nombre del plugin o `plugin-name@marketplace-name`

**Opciones:**

| Opción       | Descripción                   | Predeterminado |
| :----------- | :---------------------------- | :------------- |
| `-h, --help` | Mostrar ayuda para el comando |                |

La salida muestra dos cifras de costo para cada componente:

* **Always-on:** tokens añadidos a cada sesión por el texto de listado del plugin, como descripciones de skills, descripciones de agents, y nombres de comandos, independientemente de si algún componente se activa.
* **On-invoke:** tokens que cuesta un componente cuando se activa. Se muestra por componente, no como total del plugin, porque una sesión típica invoca solo un subconjunto de componentes.

Este ejemplo muestra cómo se ve la salida para un plugin con dos skills:

```
dependency-guard 1.2.0
  Dependency analysis for Claude Code sessions
  Source: dependency-guard@example-marketplace

Component inventory
  Skills (2)  scan-dependencies, review-changes
  Agents (0)
  Hooks (1)  (harness-only — no model context cost)
  MCP servers (0)
  LSP servers (0)

Projected token cost
  Always-on:   ~180 tok   added to every session

Per-component (rounded)
  component            always-on  on-invoke
  scan-dependencies        ~100      ~2400
  review-changes            ~80      ~1800

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

El total always-on se calcula a través de la API `count_tokens` para su modelo activo. Los números por componente se escalan proporcionalmente desde ese total. Si la API es inaccesible, el comando recurre a una estimación basada en caracteres.

<h3 id="plugin-tag">
  plugin tag
</h3>

Crea una etiqueta de lanzamiento de git para el plugin en el directorio actual. Ejecute desde dentro de la carpeta del plugin. Consulte [Etiquetar lanzamientos de plugins](/docs/es/plugin-dependencies#tag-plugin-releases-for-version-resolution).

```bash theme={null}
claude plugin tag [options]
```

**Opciones:**

| Opción        | Descripción                                                                         | Predeterminado |
| :------------ | :---------------------------------------------------------------------------------- | :------------- |
| `--push`      | Enviar la etiqueta al repositorio remoto después de crearla                         |                |
| `--dry-run`   | Imprimir lo que se etiquetaría sin crear la etiqueta                                |                |
| `-f, --force` | Crear la etiqueta incluso si el árbol de trabajo está sucio o la etiqueta ya existe |                |
| `-h, --help`  | Mostrar ayuda para el comando                                                       |                |

***

<h2 id="debugging-and-development-tools">
  Herramientas de depuración y desarrollo
</h2>

<h3 id="debugging-commands">
  Comandos de depuración
</h3>

Use `claude --debug` para ver detalles de carga de plugins:

Esto muestra:

* Qué plugins se están cargando
* Cualquier error en los manifiestos del plugin
* Registro de skills, agents y hooks
* Inicialización del servidor MCP

<h3 id="common-issues">
  Problemas comunes
</h3>

| Problema                            | Causa                               | Solución                                                                                                                                                                       |
| :---------------------------------- | :---------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plugin no se carga                  | `plugin.json` inválido              | Ejecute `claude plugin validate` o `/plugin validate` para verificar `plugin.json`, frontmatter de skill/agent/comando y `hooks/hooks.json` para errores de sintaxis y esquema |
| Skills no aparecen                  | Estructura de directorio incorrecta | Asegúrese de que `skills/` o `commands/` esté en la raíz del plugin, no dentro de `.claude-plugin/`                                                                            |
| Hooks no se disparan                | Script no ejecutable                | Ejecute `chmod +x script.sh`                                                                                                                                                   |
| MCP server falla                    | Falta `${CLAUDE_PLUGIN_ROOT}`       | Use la variable para todas las rutas del plugin                                                                                                                                |
| Errores de ruta                     | Se utilizan rutas absolutas         | Todas las rutas deben ser relativas y comenzar con `./`                                                                                                                        |
| LSP `Executable not found in $PATH` | Servidor de lenguaje no instalado   | Instale el binario (p. ej., `npm install -g typescript-language-server typescript`)                                                                                            |

<h3 id="example-error-messages">
  Mensajes de error de ejemplo
</h3>

**Errores de validación de manifiesto**:

* `Invalid JSON syntax: Unexpected token } in JSON at position 142`: busque comas faltantes, comas extra o cadenas sin comillas
* `Plugin has an invalid manifest file at .claude-plugin/plugin.json. Validation errors: name: Required`: falta un campo requerido
* `Plugin has a corrupt manifest file at .claude-plugin/plugin.json. JSON parse error: ...`: error de sintaxis JSON

**Errores de carga de plugin**:

* `Warning: No commands found in plugin my-plugin custom directory: ./cmds. Expected .md files or SKILL.md in subdirectories.`: la ruta del comando existe pero no contiene archivos de comando válidos
* `Plugin directory not found at path: ./plugins/my-plugin. Check that the marketplace entry has the correct path.`: la ruta `source` en marketplace.json apunta a un directorio inexistente
* `Plugin my-plugin has conflicting manifests: both plugin.json and marketplace entry specify components.`: elimine definiciones de componentes duplicadas o elimine `strict: false` en la entrada del marketplace

<h3 id="hook-troubleshooting">
  Solución de problemas de hooks
</h3>

**El script del hook no se ejecuta**:

1. Verifique que el script sea ejecutable: `chmod +x ./scripts/your-script.sh`
2. Verifique la línea shebang: La primera línea debe ser `#!/bin/bash` o `#!/usr/bin/env bash`
3. Verifique que la ruta use `${CLAUDE_PLUGIN_ROOT}`: `"command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/your-script.sh"`
4. Pruebe el script manualmente: `./scripts/your-script.sh`

**El hook no se dispara en los eventos esperados**:

1. Verifique que el nombre del evento sea correcto (sensible a mayúsculas): `PostToolUse`, no `postToolUse`
2. Verifique que el patrón del matcher coincida con sus herramientas: `"matcher": "Write|Edit"` para operaciones de archivo
3. Confirme que el tipo de hook sea válido: `command`, `http`, `mcp_tool`, `prompt`, o `agent`

<h3 id="mcp-server-troubleshooting">
  Solución de problemas del servidor MCP
</h3>

**El servidor no se inicia**:

1. Verifique que el comando exista y sea ejecutable
2. Verifique que todas las rutas usen la variable `${CLAUDE_PLUGIN_ROOT}`
3. Verifique los registros del servidor MCP: `claude --debug` muestra errores de inicialización
4. Pruebe el servidor manualmente fuera de Claude Code

**Las herramientas del servidor no aparecen**:

1. Asegúrese de que el servidor esté correctamente configurado en `.mcp.json` o `plugin.json`
2. Verifique que el servidor implemente correctamente el protocolo MCP
3. Busque tiempos de espera de conexión en la salida de depuración

<h3 id="directory-structure-mistakes">
  Errores de estructura de directorio
</h3>

**Síntomas**: El plugin se carga pero faltan componentes (skills, agents, hooks).

**Estructura correcta**: Los componentes deben estar en la raíz del plugin, no dentro de `.claude-plugin/`. Solo `plugin.json` pertenece a `.claude-plugin/`.

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json      ← Solo el manifiesto aquí
├── commands/            ← A nivel de raíz
├── agents/              ← A nivel de raíz
└── hooks/               ← A nivel de raíz
```

Si sus componentes están dentro de `.claude-plugin/`, muévalos a la raíz del plugin.

**Lista de verificación de depuración**:

1. Ejecute `claude --debug` y busque mensajes "loading plugin"
2. Verifique que cada directorio de componentes esté listado en la salida de depuración
3. Verifique que los permisos de archivo permitan leer los archivos del plugin

***

<h2 id="distribution-and-versioning-reference">
  Referencia de distribución y versionado
</h2>

<h3 id="version-management">
  Gestión de versiones
</h3>

Claude Code utiliza la versión del plugin como clave de caché que determina si hay una actualización disponible. Cuando ejecuta `/plugin update` o se activa la actualización automática, Claude Code calcula la versión actual y omite la actualización si coincide con la que ya está instalada.

La versión se resuelve a partir de la primera de estas que esté configurada:

1. El campo `version` en el `plugin.json` del plugin
2. El campo `version` en la entrada del marketplace del plugin en `marketplace.json`
3. El SHA del commit de git del origen del plugin, para fuentes `github`, `url`, `git-subdir` y relative-path en un marketplace alojado en git
4. `unknown`, para fuentes `npm` o directorios locales que no estén dentro de un repositorio de git

Esto le proporciona dos formas de versionar un plugin:

| Enfoque                   | Cómo                                                                      | Comportamiento de actualización                                                                                                                                                        | Mejor para                                            |
| :------------------------ | :------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- |
| **Versión explícita**     | Establezca `"version": "2.1.0"` en `plugin.json`                          | Los usuarios reciben actualizaciones solo cuando aumenta este campo. Enviar nuevos commits sin aumentarlo no tiene efecto, y `/plugin update` informa "already at the latest version". | Plugins publicados con ciclos de lanzamiento estables |
| **Versión de commit-SHA** | Omita `version` tanto de `plugin.json` como de la entrada del marketplace | Los usuarios reciben actualizaciones en cada nuevo commit a la fuente de git del plugin                                                                                                | Plugins internos o de equipo en desarrollo activo     |

<Warning>
  Si establece `version` en `plugin.json`, debe aumentarlo cada vez que desee que los usuarios reciban cambios. Enviar nuevos commits por sí solo no es suficiente, porque Claude Code ve la misma cadena de versión y mantiene la copia en caché. Si está iterando rápidamente, deje `version` sin establecer para que se use el SHA del commit de git en su lugar.
</Warning>

Si utiliza versiones explícitas, siga el [versionado semántico](https://semver.org) (`MAJOR.MINOR.PATCH`): aumente MAJOR para cambios de ruptura, MINOR para nuevas características, PATCH para correcciones de errores. Documente los cambios en un `CHANGELOG.md`.

***

<h2 id="see-also">
  Ver también
</h2>

* [Plugins](/docs/es/plugins) - Tutoriales y uso práctico
* [Marketplaces de plugins](/docs/es/plugin-marketplaces) - Crear y gestionar marketplaces
* [Skills](/docs/es/skills) - Detalles de desarrollo de skills
* [Subagents](/docs/es/sub-agents) - Configuración y capacidades del agent
* [Hooks](/docs/es/hooks) - Manejo de eventos y automatización
* [MCP](/docs/es/mcp) - Integración de herramientas externas
* [Configuración](/docs/es/settings) - Opciones de configuración para plugins
