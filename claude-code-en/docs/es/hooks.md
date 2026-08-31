> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia de hooks

> Referencia para eventos de hooks de Claude Code, esquema de configuración, formatos de entrada/salida JSON, códigos de salida, hooks asincronos, hooks HTTP, hooks de prompt y hooks de herramientas MCP.

<Tip>
  Para una guía de inicio rápido con ejemplos, consulte [Automatizar acciones con hooks](/docs/es/hooks-guide).
</Tip>

Los hooks son comandos de shell definidos por el usuario, puntos finales HTTP o prompts de LLM que se ejecutan automáticamente en puntos específicos del ciclo de vida de Claude Code. Utilice esta referencia para buscar esquemas de eventos, opciones de configuración, formatos de entrada/salida JSON y características avanzadas como hooks asincronos, hooks HTTP y hooks de herramientas MCP. Si está configurando hooks por primera vez, comience con la [guía](/docs/es/hooks-guide) en su lugar.

<h2 id="hook-lifecycle">
  Ciclo de vida de los hooks
</h2>

Los hooks se activan en puntos específicos durante una sesión de Claude Code. Cuando se activa un evento y un matcher coincide, Claude Code pasa contexto JSON sobre el evento a su controlador de hook. Para hooks de comando, la entrada llega en stdin. Para hooks HTTP, llega como el cuerpo de la solicitud POST. Su controlador puede entonces inspeccionar la entrada, tomar medidas y opcionalmente devolver una decisión.

Los eventos se dividen en tres cadencias:

* una vez por sesión: `SessionStart` y `SessionEnd`
* una vez por turno: `UserPromptSubmit`, `Stop` y `StopFailure`
* en cada llamada a herramienta dentro del bucle agentico: `PreToolUse` y `PostToolUse`

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=81b9256c1bbe8832553485f5d9e9c746" alt="Diagrama del ciclo de vida de hooks que muestra Setup opcional alimentando a SessionStart, luego un bucle por turno que contiene UserPromptSubmit, UserPromptExpansion para slash commands, el bucle agentico anidado (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), y Stop o StopFailure, seguido de TeammateIdle, PreCompact, PostCompact y SessionEnd, con Elicitation y ElicitationResult anidados dentro de la ejecución de herramientas MCP, PermissionDenied como una rama lateral de PermissionRequest para denegaciones en modo automático, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged y FileChanged como eventos asincronos independientes, y MessageDisplay como un evento de solo visualización que se ejecuta mientras el texto del mensaje del asistente se transmite" width="520" height="1336" data-path="images/hooks-lifecycle.svg" />
  </Frame>
</div>

La tabla a continuación resume cuándo se activa cada evento. La sección [Hook events](#hook-events) documenta el esquema de entrada completo y las opciones de control de decisión para cada uno.

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
| `PreModelSwitch`      | Before Claude Code applies a model switch that you or a client requested. Can block the switch                                                                                                                                                        |
| `PostModelSwitch`     | After the session's model changes, including changes Claude Code makes on its own, such as restoring the model when you resume a session                                                                                                              |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

<h3 id="how-a-hook-resolves">
  Cómo se resuelve un hook
</h3>

Para ver cómo encajan estas piezas, considere este hook `PreToolUse` que bloquea comandos de shell destructivos. El `matcher` se reduce a llamadas a herramientas Bash y la condición `if` se reduce aún más a subcomandos Bash que coinciden con `rm *`, por lo que `block-rm.sh` solo se genera cuando ambos filtros coinciden:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(rm *)",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

El script lee la entrada JSON desde stdin, extrae el comando y devuelve una `permissionDecision` de `"deny"` si contiene `rm -rf`:

```bash theme={null}
#!/bin/bash
# .claude/hooks/block-rm.sh
COMMAND=$(jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q 'rm -rf'; then
  jq -n '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: "Destructive command blocked by hook"
    }
  }'
else
  exit 0  # no decision; normal permission flow applies
fi
```

Ahora suponga que Claude Code decide ejecutar `Bash "rm -rf /tmp/build"`. Esto es lo que sucede:

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" alt="Diagrama de resolución de hooks: PreToolUse se activa, el matcher verifica la coincidencia de Bash, luego la condición if verifica la coincidencia de Bash(rm *). Si ambos coinciden, el comando del hook se ejecuta y devuelve permissionDecision deny, por lo que la llamada a la herramienta se bloquea y Claude Code continúa. Si alguna verificación no coincide, el hook se omite y la llamada a la herramienta se permite continuar." width="930" height="270" data-path="images/hook-resolution.svg" />
</Frame>

<Steps>
  <Step title="Se activa el evento">
    El evento `PreToolUse` se activa. Claude Code envía la entrada de la herramienta como JSON en stdin al hook:

    ```json theme={null}
    { "tool_name": "Bash", "tool_input": { "command": "rm -rf /tmp/build" }, ... }
    ```
  </Step>

  <Step title="El matcher verifica">
    El matcher `"Bash"` coincide con el nombre de la herramienta, por lo que se activa este grupo de hooks. Si omite el matcher o usa `"*"`, el grupo se activa en cada ocurrencia del evento.
  </Step>

  <Step title="La condición if verifica">
    La condición `if` `"Bash(rm *)"` coincide porque `rm -rf /tmp/build` es un subcomando que coincide con `rm *`, por lo que se genera este controlador. Si el comando hubiera sido `npm test`, la verificación `if` habría fallado y `block-rm.sh` nunca se habría ejecutado, evitando la sobrecarga de generación de procesos. El campo `if` es opcional; sin él, cada controlador en el grupo coincidente se ejecuta.
  </Step>

  <Step title="Se ejecuta el controlador de hooks">
    El script inspecciona el comando completo y encuentra `rm -rf`, por lo que imprime una decisión en stdout:

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Destructive command blocked by hook"
      }
    }
    ```

    Si el comando hubiera sido una variante más segura de `rm` como `rm file.txt`, el script habría alcanzado `exit 0` en su lugar. El código de salida 0 sin salida significa que el hook no tiene decisión que reportar, por lo que la llamada a la herramienta continúa a través del [flujo de permisos](/docs/es/permissions) normal. El hook puede denegar la llamada, pero permanecer en silencio no la aprueba.
  </Step>

  <Step title="Claude Code actúa sobre el resultado">
    Claude Code lee la decisión JSON, bloquea la llamada a la herramienta y muestra a Claude la razón.
  </Step>
</Steps>

La sección [Configuration](#configuration) a continuación documenta el esquema completo, y cada sección [hook event](#hook-events) documenta qué entrada recibe su comando y qué salida puede devolver.

<h2 id="configuration">
  Configuración
</h2>

Los hooks se definen en archivos de configuración JSON. La configuración tiene tres niveles de anidamiento:

1. Elija un [hook event](#hook-events) al que responder, como `PreToolUse` o `Stop`
2. Agregue un [matcher group](#matcher-patterns) para filtrar cuándo se activa, como "solo para la herramienta Bash"
3. Defina uno o más [hook handlers](#hook-handler-fields) para ejecutar cuando coincida

Consulte [Cómo se resuelve un hook](#how-a-hook-resolves) arriba para un recorrido completo con un ejemplo anotado.

<Note>
  Esta página utiliza términos específicos para cada nivel: **hook event** para el punto del ciclo de vida, **matcher group** para el filtro y **hook handler** para el comando de shell, punto final HTTP, herramienta MCP, prompt o agente que se ejecuta. "Hook" por sí solo se refiere a la característica general.
</Note>

<h3 id="hook-locations">
  Ubicaciones de hooks
</h3>

Dónde defina un hook determina su alcance:

| Ubicación                                                 | Alcance                            | Compartible                                     |
| :-------------------------------------------------------- | :--------------------------------- | :---------------------------------------------- |
| `~/.claude/settings.json`                                 | Todos sus proyectos                | No, local en su máquina                         |
| `.claude/settings.json`                                   | Proyecto único                     | Sí, puede ser confirmado en el repositorio      |
| `.claude/settings.local.json`                             | Proyecto único                     | No, ignorado por git cuando Claude Code lo crea |
| Configuración de política administrada                    | Toda la organización               | Sí, controlado por administrador                |
| [Plugin](/docs/es/plugins) `hooks/hooks.json`                  | Cuando el plugin está habilitado   | Sí, incluido con el plugin                      |
| [Skill](/docs/es/skills) o [agent](/docs/es/sub-agents) frontmatter | Mientras el componente está activo | Sí, definido en el archivo del componente       |

Para obtener detalles sobre la resolución de archivos de configuración, consulte [settings](/docs/es/settings).

Los administradores empresariales pueden usar `allowManagedHooksOnly` para bloquear hooks de usuario, proyecto y plugin. Los hooks de plugins habilitados forzosamente en la configuración administrada `enabledPlugins` están exentos, por lo que los administradores pueden distribuir hooks verificados a través de un mercado de la organización. Consulte [Hook configuration](/docs/es/settings#hook-configuration).

<h3 id="matcher-patterns">
  Patrones de matcher
</h3>

El campo `matcher` filtra cuándo se activan los hooks. Cómo se evalúa un matcher depende de los caracteres que contenga:

| Valor del matcher                                    | Evaluado como                                                                                                  | Ejemplo                                                                                                                                                                                            |
| :--------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"*"`, `""` u omitido                                | Coincidir todo                                                                                                 | se activa en cada ocurrencia del evento                                                                                                                                                            |
| Solo letras, dígitos, `_`, `-`, espacios, `,` y `\|` | Cadena exacta, o lista de cadenas exactas separadas por `\|` o `,` con espacios en blanco opcionales alrededor | `Bash` coincide solo con la herramienta Bash; `Edit\|Write` y `Edit, Write` cada uno coincide con cualquiera de las herramientas exactamente; `code-reviewer` coincide solo con ese tipo de agente |
| Contiene cualquier otro carácter                     | Expresión regular de JavaScript, sin anclar                                                                    | `^Notebook` coincide con cualquier herramienta cuyo nombre comience con Notebook; `mcp__memory__.*` coincide con cada herramienta del servidor `memory`                                            |

Un matcher en la ruta de expresión regular se prueba con `RegExp.prototype.test` de JavaScript, que tiene éxito en una coincidencia en cualquier lugar del valor. `Edit.*` coincide tanto con `Edit` como con `NotebookEdit`; envuelva el patrón en `^` y `$`, como en `^Edit$`, cuando necesite una coincidencia de cadena completa.

Los separadores de coma y la tolerancia de espacios en blanco circundantes requieren Claude Code v2.1.191 o posterior.

Los guiones en el conjunto de coincidencia exacta requieren Claude Code v2.1.195 o posterior. En versiones anteriores, un nombre con guiones como `code-reviewer` se evalúa como una expresión regular sin anclar, por lo que también se activa para `senior-code-reviewer`; anclelo como `^code-reviewer$` en esas versiones para coincidir solo con ese nombre.

`FileChanged` y `StopFailure` utilizan un conjunto de coincidencia exacta más estrecho de solo letras, dígitos, `_` y `|`. Un guión, espacio o coma en un matcher para esos dos eventos lo mantiene en la ruta de expresión regular, y solo `|` separa alternativas. Todos los demás eventos con soporte de matcher en la tabla que sigue aceptan `|` o `,`.

El evento `FileChanged` no sigue estas reglas al construir su lista de vigilancia. Consulte [FileChanged](#filechanged).

Cada tipo de evento coincide en un campo diferente:

| Evento                                                                                                                                            | En qué filtra el matcher                                                      | Valores de matcher de ejemplo                                                                                                                                                       |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | nombre de la herramienta                                                      | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                    | cómo comenzó la sesión                                                        | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                           | qué bandera CLI desencadenó la configuración                                  | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                      | por qué terminó la sesión                                                     | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                    | tipo de notificación                                                          | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                   | tipo de agente                                                                | `general-purpose`, `Explore`, `Plan`, nombres de agentes personalizados, o nombres con alcance de plugin como `^my-plugin:reviewer$`                                                |
| `PreCompact`, `PostCompact`                                                                                                                       | qué desencadenó la compactación                                               | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                    | tipo de agente                                                                | los mismos valores que `SubagentStart`                                                                                                                                              |
| `ConfigChange`                                                                                                                                    | fuente de configuración                                                       | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `CwdChanged`                                                                                                                                      | sin soporte de matcher                                                        | siempre se activa en cada cambio de directorio                                                                                                                                      |
| `FileChanged`                                                                                                                                     | nombres de archivo literales a vigilar (consulte [FileChanged](#filechanged)) | `.envrc\|.env`                                                                                                                                                                      |
| `StopFailure`                                                                                                                                     | tipo de error                                                                 | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                              | razón de carga                                                                | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `UserPromptExpansion`                                                                                                                             | nombre del comando                                                            | sus nombres de skill o comando                                                                                                                                                      |
| `Elicitation`                                                                                                                                     | nombre del servidor MCP                                                       | sus nombres de servidor MCP configurados                                                                                                                                            |
| `ElicitationResult`                                                                                                                               | nombre del servidor MCP                                                       | los mismos valores que `Elicitation`                                                                                                                                                |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | sin soporte de matcher                                                        | siempre se activa en cada ocurrencia                                                                                                                                                |

El matcher se ejecuta contra un campo de la [entrada JSON](#hook-input-and-output) que Claude Code envía a su hook en stdin. Para eventos de herramientas, ese campo es `tool_name`. Cada sección [hook event](#hook-events) enumera el conjunto completo de valores de matcher y el esquema de entrada para ese evento.

Este ejemplo ejecuta un script de linting solo cuando Claude escribe o edita un archivo:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/lint-check.sh"
          }
        ]
      }
    ]
  }
}
```

`UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` y `CwdChanged` no admiten matchers y siempre se activan en cada ocurrencia. Si agrega un campo `matcher` a estos eventos, se ignora silenciosamente.

Para eventos de herramientas, puede filtrar más estrechamente estableciendo el campo [`if`](#common-fields) en controladores de hooks individuales. `if` utiliza [sintaxis de regla de permiso](/docs/es/permissions) para coincidir con el nombre de la herramienta y los argumentos juntos, por lo que `"Bash(git *)"` se ejecuta cuando cualquier subcomando de la entrada de Bash coincide con `git *` y `"Edit(*.ts)"` se ejecuta solo para archivos TypeScript.

<h4 id="match-mcp-tools">
  Coincidir herramientas MCP
</h4>

Las herramientas del servidor [MCP](/docs/es/mcp) aparecen como herramientas normales en eventos de herramientas (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`), por lo que puede hacerlas coincidir de la misma manera que cualquier otro nombre de herramienta.

Las herramientas MCP siguen el patrón de nomenclatura `mcp__<server>__<tool>`, por ejemplo:

* `mcp__memory__create_entities`: herramienta crear entidades del servidor Memory
* `mcp__filesystem__read_file`: herramienta leer archivo del servidor Filesystem
* `mcp__github__search_repositories`: herramienta de búsqueda del servidor GitHub

Para coincidir con cada herramienta de un servidor, agregue `.*` al prefijo del servidor. El `.*` es requerido: un matcher como `mcp__memory` o `mcp__brave-search` contiene solo caracteres de coincidencia exacta, por lo que se compara como una cadena exacta y no coincide con ninguna herramienta.

* `mcp__memory__.*` coincide con todas las herramientas del servidor `memory`
* `mcp__brave-search__.*` coincide con todas las herramientas de un servidor cuyo nombre contiene un guión
* `mcp__.*__write.*` coincide con cualquier herramienta cuyo nombre comience con `write` de cualquier servidor

Los guiones en el conjunto de coincidencia exacta requieren Claude Code v2.1.195 o posterior. En versiones anteriores, un prefijo con guiones simple como `mcp__brave-search` se evalúa como una expresión regular sin anclar y coincide con cada herramienta de ese servidor. La forma `mcp__brave-search__.*` funciona en cada versión.

Las herramientas de un [servidor MCP incluido en plugin](/docs/es/mcp#plugin-provided-mcp-servers) utilizan un segmento de servidor con alcance que incluye el nombre del plugin: `mcp__plugin_<plugin-name>_<server-name>__<tool>`. Un matcher escrito contra la clave del servidor simple nunca se activa para estas herramientas. Para un plugin llamado `my-plugin` que incluye un servidor bajo la clave `db`, una herramienta `query` aparece como `mcp__plugin_my-plugin_db__query`, por lo que el matcher para cada herramienta de ese servidor es `mcp__plugin_my-plugin_db__.*`. Use el mismo nombre de herramienta con alcance en el campo [`if`](#common-fields) de un controlador. Consulte [Plugin-provided MCP servers](/docs/es/mcp#plugin-provided-mcp-servers) para ver cómo se construye el nombre con alcance.

Este ejemplo registra todas las operaciones del servidor de memoria y valida operaciones de escritura de cualquier servidor MCP:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "mcp__memory__.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Memory operation initiated' >> ~/mcp-operations.log"
          }
        ]
      },
      {
        "matcher": "mcp__.*__write.*",
        "hooks": [
          {
            "type": "command",
            "command": "/home/user/scripts/validate-mcp-write.py"
          }
        ]
      }
    ]
  }
}
```

<h3 id="hook-handler-fields">
  Campos del controlador de hooks
</h3>

Cada objeto en el array `hooks` interno es un controlador de hook: el comando de shell, punto final HTTP, herramienta MCP, prompt de LLM o agente que se ejecuta cuando el matcher coincide. Hay cinco tipos:

* **[Command hooks](#command-hook-fields)** (`type: "command"`): ejecutan un comando de shell. Su script recibe la [entrada JSON](#hook-input-and-output) del evento en stdin y comunica resultados a través de códigos de salida y stdout.
* **[HTTP hooks](#http-hook-fields)** (`type: "http"`): envían la entrada JSON del evento como una solicitud HTTP POST a una URL. El punto final comunica resultados a través del cuerpo de la respuesta usando el mismo [formato de salida JSON](#json-output) que los hooks de comando.
* **[MCP tool hooks](#mcp-tool-hook-fields)** (`type: "mcp_tool"`): llaman a una herramienta en un servidor [MCP](/docs/es/mcp) ya conectado. La salida de texto de la herramienta se trata como stdout de hook de comando.
* **[Prompt hooks](#prompt-and-agent-hook-fields)** (`type: "prompt"`): envían un prompt a un modelo Claude para evaluación de un solo turno. El modelo devuelve una decisión sí/no como JSON. Consulte [Prompt-based hooks](#prompt-based-hooks).
* **[Agent hooks](#prompt-and-agent-hook-fields)** (`type: "agent"`): generan un subagente que puede usar herramientas como Read, Grep y Glob para verificar condiciones antes de devolver una decisión. Los hooks de agente son experimentales y pueden cambiar. Consulte [Agent-based hooks](#agent-based-hooks).

Todos los hooks coincidentes se ejecutan en paralelo, y los controladores idénticos se deduplicarán automáticamente. Los hooks de comando se deduplicarán por cadena de comando y `args`, y los hooks HTTP se deduplicarán por URL.

Los controladores se ejecutan en el directorio actual con el entorno de Claude Code. La variable de entorno `$CLAUDE_CODE_REMOTE` se establece en `"true"` en entornos web remotos y no se establece en la CLI local. A partir de v2.1.199, [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/es/env-vars) se establece en el ID de sesión de [Remote Control](/docs/es/remote-control) mientras la sesión local tiene una conexión Remote Control activa.

<h4 id="common-fields">
  Campos comunes
</h4>

Estos campos se aplican a todos los tipos de hooks:

| Campo           | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :-------------- | :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | sí        | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"` o `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `if`            | no        | Sintaxis de regla de permiso para filtrar cuándo se ejecuta este hook, como `"Bash(git *)"` o `"Edit(*.ts)"`. El hook solo se genera si la llamada a herramienta coincide con el patrón. Consulte la tabla [Bash matching](#bash-if-matching) a continuación para ver cómo los patrones de Bash se evalúan contra subcomandos, `$()` y backticks. Solo se evalúa en eventos de herramientas: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` y `PermissionDenied`. En otros eventos, un hook con `if` establecido nunca se ejecuta. Utiliza la misma sintaxis que [reglas de permiso](/docs/es/permissions) |
| `timeout`       | no        | Segundos antes de cancelar. Valores predeterminados: 600 para `command`, `http` y `mcp_tool`; 30 para `prompt`; 60 para `agent`. [`UserPromptSubmit`](#userpromptsubmit) reduce el valor predeterminado de `command`, `http` y `mcp_tool` a 30, y [`MessageDisplay`](#messagedisplay) lo reduce a 10                                                                                                                                                                                                                                                                                                                          |
| `statusMessage` | no        | Mensaje de spinner personalizado mostrado mientras se ejecuta el hook                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `once`          | no        | Si es `true`, se ejecuta una vez por sesión y luego se elimina. Solo se honra para hooks declarados en [skill frontmatter](#hooks-in-skills-and-agents); se ignora en archivos de configuración y frontmatter de agente                                                                                                                                                                                                                                                                                                                                                                                                       |

El campo `if` contiene exactamente una regla de permiso. No hay sintaxis `&&`, `||` o de lista para combinar reglas; para aplicar múltiples condiciones, defina un controlador de hook separado para cada una.

<span id="bash-if-matching" />Para patrones de Bash, si su comando hook se ejecuta depende de la forma del patrón y del comando Bash que Claude está invocando. Las asignaciones `VAR=value` iniciales se eliminan antes de coincidir.

| patrón `if`        | Comando Bash           | ¿Se ejecuta el hook? | Por qué                                                                                                                  |
| :----------------- | :--------------------- | :------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `FOO=bar git push`     | sí                   | las asignaciones iniciales se eliminan; `git push` coincide                                                              |
| `Bash(git *)`      | `npm test && git push` | sí                   | cada subcomando se verifica; `git push` coincide                                                                         |
| `Bash(rm *)`       | `echo $(rm -rf /)`     | sí                   | los comandos dentro de `$()` y backticks se verifican; `rm -rf /` coincide                                               |
| `Bash(rm *)`       | `echo $(date)`         | no                   | ningún subcomando coincide con `rm *`                                                                                    |
| `Bash(git push *)` | `echo $(date)`         | sí                   | los patrones que especifican más que el nombre del comando ejecutan el hook de todas formas en `$()`, backticks o `$VAR` |

El filtro también falla abierto, ejecutando su hook independientemente del patrón, cuando el comando Bash no se puede analizar. Debido a que el filtro `if` es de mejor esfuerzo, use el [sistema de permisos](/docs/es/permissions) en lugar de un hook para aplicar un permiso o denegación duro.

<h4 id="command-hook-fields">
  Campos de comando hook
</h4>

Además de los [campos comunes](#common-fields), los hooks de comando aceptan estos campos:

| Campo         | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                     |
| :------------ | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `command`     | sí        | Comando de shell a ejecutar. Con `args`, el ejecutable a generar directamente. Consulte [Exec form and shell form](#exec-form-and-shell-form)                                                                                                                                                                                                                                   |
| `args`        | no        | Lista de argumentos. Cuando está presente, `command` se resuelve como un ejecutable y se genera directamente con `args` como el vector de argumentos, sin shell involucrado. Consulte [Exec form and shell form](#exec-form-and-shell-form)                                                                                                                                     |
| `async`       | no        | Si es `true`, se ejecuta en segundo plano sin bloquear. Consulte [Run hooks in the background](#run-hooks-in-the-background)                                                                                                                                                                                                                                                    |
| `asyncRewake` | no        | Si es `true`, se ejecuta en segundo plano y despierta a Claude en código de salida 2. Implica `async`. El stderr del hook, o stdout si stderr está vacío, se muestra a Claude como un recordatorio del sistema para que pueda reaccionar a un fallo de fondo de larga duración                                                                                                  |
| `shell`       | no        | Shell a usar para este hook. Acepta `"bash"` o `"powershell"`. Por defecto es `"bash"`, o `"powershell"` en Windows cuando Git Bash no está instalado. Establecer `"powershell"` ejecuta el comando a través de PowerShell en Windows. No requiere `CLAUDE_CODE_USE_POWERSHELL_TOOL` ya que los hooks generan PowerShell directamente. Se ignora cuando `args` está establecido |

<a id="exec-form-and-shell-form" />

<h5 id="exec-form-and-shell-form">
  Exec form y shell form
</h5>

Un hook de comando se ejecuta como exec form cuando `args` está establecido, y shell form cuando `args` se omite. Establezca `args` siempre que el hook haga referencia a un [marcador de posición de ruta](#reference-scripts-by-path), ya que cada elemento se pasa como un argumento sin comillas. Omita `args` cuando necesite características de shell como pipes o `&&`, o cuando ninguna preocupación se aplique.

**Exec form** se ejecuta cuando `args` está presente. Claude Code resuelve `command` como un ejecutable en `PATH` y lo genera directamente con `args` como el vector de argumentos. No hay shell, por lo que cada elemento `args` es exactamente un argumento tal como está escrito, y los marcadores de posición de ruta como `${CLAUDE_PLUGIN_ROOT}` se sustituyen en `command` y en cada elemento `args` como cadenas simples. Los caracteres especiales como apóstrofes, `$` y backticks pasan literalmente porque no hay shell para interpretarlos. No ocurre tokenización de shell en ninguna plataforma.

**Shell form** se ejecuta cuando `args` se omite. La cadena `command` se pasa a un shell: `sh -c` en macOS y Linux, Git Bash en Windows, o PowerShell cuando Git Bash no está instalado. Establezca el campo `shell` para elegir explícitamente. El shell tokeniza la cadena, expande variables e interpreta pipes, `&&`, redirecciones y globs.

<Note>
  En Windows, exec form requiere que `command` se resuelva a un ejecutable real como `.exe`. Los shims `.cmd` y `.bat` que npm, npx, eslint y otras herramientas instalan en `node_modules/.bin` no son ejecutables y no pueden generarse sin un shell. Para ejecutarlos en exec form, invoque el script subyacente con `node` directamente, por ejemplo `"command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/node_modules/eslint/bin/eslint.js"]`. El patrón `node` más ruta de script funciona en cada plataforma porque `node.exe` es un binario real. Para ejecutar un shim `.cmd` o `.bat` por nombre, use shell form.
</Note>

Este ejemplo ejecuta un script de Node incluido con un plugin. Exec form pasa la ruta de script resuelta como un argumento sin comillas:

```json theme={null}
{
  "type": "command",
  "command": "node",
  "args": ["${CLAUDE_PLUGIN_ROOT}/scripts/format.js", "--fix"]
}
```

La forma de shell equivalente necesita comillas para manejar rutas con espacios o caracteres especiales:

```json theme={null}
{
  "type": "command",
  "command": "node \"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.js --fix"
}
```

Ambas formas admiten los mismos [marcadores de posición de ruta](#reference-scripts-by-path), y ambas los exportan como las variables de entorno `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT` y `CLAUDE_PLUGIN_DATA` en el proceso generado, por lo que un script puede leer `process.env.CLAUDE_PLUGIN_ROOT` independientemente de cómo se haya lanzado.

Los hooks de plugin además sustituyen valores [`${user_config.*}`](/docs/es/plugins-reference#user-configuration), solo en exec form: el valor se sustituye en `command` y en cada elemento `args` como una cadena simple, por lo que ningún shell lo vuelve a analizar.

Un hook de plugin en shell form cuyo `command` hace referencia a `${user_config.*}` falla con un [error](/docs/es/errors#plugin-command-references-user-config) en lugar de ejecutarse. Para usar un valor de opción de un hook en shell form, lea la variable de entorno `$CLAUDE_PLUGIN_OPTION_<KEY>`, como `$CLAUDE_PLUGIN_OPTION_WEBHOOK_URL` para una opción `webhook_url`, o establezca `args` para cambiar el hook a exec form. Antes de v2.1.207, los comandos de hook de plugin en shell form también sustituían `${user_config.*}`.

<Note>
  En exec form, `command` es solo el nombre o ruta del ejecutable. Si `command` es un nombre simple sin separador de ruta y contiene espacios junto con `args`, Claude Code registra una advertencia porque la generación fallará: no hay ejecutable llamado `node script.js`. Mueva los tokens adicionales a `args`. Las rutas absolutas con espacios, como `C:\Program Files\nodejs\node.exe`, son un ejecutable válido único y no desencadenan la advertencia.
</Note>

<h4 id="http-hook-fields">
  Campos de hook HTTP
</h4>

Además de los [campos comunes](#common-fields), los hooks HTTP aceptan estos campos:

| Campo            | Requerido | Descripción                                                                                                                                                                                                                                       |
| :--------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `url`            | sí        | URL a la que enviar la solicitud POST                                                                                                                                                                                                             |
| `headers`        | no        | Encabezados HTTP adicionales como pares clave-valor. Los valores admiten interpolación de variables de entorno usando la sintaxis `$VAR_NAME` o `${VAR_NAME}`. Solo se resuelven las variables enumeradas en `allowedEnvVars`                     |
| `allowedEnvVars` | no        | Lista de nombres de variables de entorno que pueden interpolarse en valores de encabezado. Las referencias a variables no enumeradas se reemplazan con cadenas vacías. Requerido para que funcione cualquier interpolación de variable de entorno |

Claude Code envía la [entrada JSON](#hook-input-and-output) del hook como el cuerpo de la solicitud POST con `Content-Type: application/json`. El cuerpo de la respuesta usa el mismo [formato de salida JSON](#json-output) que los hooks de comando.

El manejo de errores difiere de los hooks de comando: las respuestas que no son 2xx, los fallos de conexión y los tiempos de espera agotados producen errores sin bloqueo que permiten que la ejecución continúe. Para bloquear una llamada a herramienta o denegar un permiso, devuelva una respuesta 2xx con un cuerpo JSON que contenga `decision: "block"` o un `hookSpecificOutput` con `permissionDecision: "deny"`.

Este ejemplo envía eventos `PreToolUse` a un servicio de validación local, autenticándose con un token de la variable de entorno `MY_TOKEN`:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/pre-tool-use",
            "timeout": 30,
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

<h4 id="mcp-tool-hook-fields">
  Campos de hook de herramienta MCP
</h4>

Además de los [campos comunes](#common-fields), los hooks de herramienta MCP aceptan estos campos:

| Campo    | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                     |
| :------- | :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server` | sí        | Nombre de un servidor MCP configurado. Para un [servidor incluido en plugin](/docs/es/mcp#plugin-provided-mcp-servers), este es el nombre con alcance `plugin:<plugin-name>:<server-name>`, como `plugin:my-plugin:db`, no la clave del servidor simple. El servidor ya debe estar conectado; el hook nunca desencadena un flujo OAuth o de conexión |
| `tool`   | sí        | Nombre de la herramienta a llamar en ese servidor                                                                                                                                                                                                                                                                                               |
| `input`  | no        | Argumentos pasados a la herramienta. Los valores de cadena admiten sustitución `${path}` de la [entrada JSON](#hook-input-and-output) del hook, como `"${tool_input.file_path}"`                                                                                                                                                                |

La salida de texto de la herramienta se trata como stdout de hook de comando: si se analiza como [salida JSON](#json-output) válida, se procesa como una decisión; de lo contrario, se muestra como texto sin formato. Si el servidor nombrado no está conectado, o la herramienta devuelve `isError: true`, el hook produce un error sin bloqueo y la ejecución continúa.

Los hooks de herramienta MCP están disponibles en cada evento de hook una vez que Claude Code se ha conectado a sus servidores MCP. `SessionStart` y `Setup` típicamente se activan antes de que los servidores terminen de conectarse, por lo que los hooks en esos eventos deben esperar el error "no conectado" en la primera ejecución.

Este ejemplo llama a la herramienta `security_scan` en el servidor MCP `my_server` después de cada `Write` o `Edit`, pasando la ruta del archivo editado:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "security_scan",
            "input": { "file_path": "${tool_input.file_path}" }
          }
        ]
      }
    ]
  }
}
```

<h4 id="prompt-and-agent-hook-fields">
  Campos de hook de prompt y agente
</h4>

Además de los [campos comunes](#common-fields), los hooks de prompt y agente aceptan estos campos:

| Campo    | Requerido | Descripción                                                                                                                                                                                                   |
| :------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `prompt` | sí        | Texto del prompt a enviar al modelo. Use `$ARGUMENTS` como marcador de posición para la entrada JSON del hook. Escape con una barra invertida para incluir texto literal: `\$1.00` se representa como `$1.00` |
| `model`  | no        | Modelo a usar para evaluación. Por defecto es un modelo rápido                                                                                                                                                |

<h3 id="reference-scripts-by-path">
  Referenciar scripts por ruta
</h3>

Use estos marcadores de posición para referenciar scripts de hooks relativos a la raíz del proyecto o plugin, independientemente del directorio de trabajo cuando se ejecuta el hook:

* `${CLAUDE_PROJECT_DIR}`: la raíz del proyecto. Claude Code también establece esta variable en el entorno de [servidores MCP stdio](/docs/es/mcp#option-3-add-a-local-stdio-server) y servidores LSP de plugin.
* `${CLAUDE_PLUGIN_ROOT}`: el directorio de instalación del plugin, para scripts incluidos con un [plugin](/docs/es/plugins). Cambia en cada actualización de plugin.
* `${CLAUDE_PLUGIN_DATA}`: el [directorio de datos persistentes](/docs/es/plugins-reference#persistent-data-directory) del plugin, para dependencias y estado que deben sobrevivir a las actualizaciones de plugin.

Prefiera [exec form](#exec-form-and-shell-form) para cualquier hook que haga referencia a un marcador de posición de ruta. Exec form pasa cada elemento `args` como un argumento sin tokenización de shell, por lo que las rutas con espacios o caracteres especiales no necesitan comillas. En shell form, envuelva cada marcador de posición entre comillas dobles.

<Tabs>
  <Tab title="Scripts de proyecto">
    Este ejemplo usa `${CLAUDE_PROJECT_DIR}` para ejecutar un verificador de estilo desde el directorio `.claude/hooks/` del proyecto después de cualquier llamada a herramienta `Write` o `Edit`:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Scripts de plugin">
    Defina hooks de plugin en `hooks/hooks.json` con un campo `description` opcional de nivel superior. Cuando se habilita un plugin, sus hooks se fusionan con sus hooks de usuario y proyecto.

    Este ejemplo ejecuta un script de formato incluido con el plugin:

    ```json theme={null}
    {
      "description": "Automatic code formatting",
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PLUGIN_ROOT}/scripts/format.sh",
                "args": [],
                "timeout": 30
              }
            ]
          }
        ]
      }
    }
    ```

    Consulte la [referencia de componentes de plugin](/docs/es/plugins-reference#hooks) para obtener detalles sobre cómo crear hooks de plugin.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hooks en skills y agentes
</h3>

Además de archivos de configuración y plugins, los hooks pueden definirse directamente en [skills](/docs/es/skills) y [subagentes](/docs/es/sub-agents) usando frontmatter. Estos hooks se limitan al ciclo de vida del componente y solo se ejecutan cuando ese componente está activo.

Se admiten todos los eventos de hook. Para subagentes, los hooks `Stop` se convierten automáticamente a `SubagentStop` ya que ese es el evento que se activa cuando un subagente se completa.

Los hooks usan el mismo formato de configuración que los hooks basados en configuración pero se limitan a la vida útil del componente y se limpian cuando finaliza.

Esta skill define un hook `PreToolUse` que ejecuta un script de validación de seguridad antes de cada comando `Bash`:

```yaml theme={null}
---
name: secure-operations
description: Perform operations with security checks
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/security-check.sh"
---
```

Los agentes usan el mismo formato en su frontmatter YAML.

<h3 id="the-/hooks-menu">
  El menú `/hooks`
</h3>

Escriba `/hooks` en Claude Code para abrir un navegador de solo lectura para sus hooks configurados. El menú muestra cada evento de hook con un recuento de hooks configurados, le permite profundizar en matchers y muestra los detalles completos de cada controlador de hook. Úselo para verificar la configuración, verificar desde qué archivo de configuración proviene un hook o inspeccionar el comando, prompt o URL de un hook.

El menú muestra los cinco tipos de hooks: `command`, `prompt`, `agent`, `http` y `mcp_tool`. Cada hook está etiquetado con un prefijo `[type]` y una fuente que indica dónde se definió:

* `User`: de `~/.claude/settings.json`
* `Project`: de `.claude/settings.json`
* `Local`: de `.claude/settings.local.json`
* `Plugin`: de `hooks/hooks.json` de un plugin
* `Session`: registrado en memoria para la sesión actual
* `Built-in`: registrado internamente por Claude Code

Seleccionar un hook abre una vista de detalle que muestra su evento, matcher, tipo, archivo de origen y el comando, prompt o URL completo. El menú es de solo lectura: para agregar, modificar o eliminar hooks, edite el JSON de configuración directamente o pida a Claude que haga el cambio.

<h3 id="disable-or-remove-hooks">
  Deshabilitar o eliminar hooks
</h3>

Para eliminar un hook, elimine su entrada del archivo de configuración JSON.

Para deshabilitar temporalmente todos los hooks sin eliminarlos, establezca `"disableAllHooks": true` en su archivo de configuración. No hay forma de deshabilitar un hook individual mientras se mantiene en la configuración.

La configuración `disableAllHooks` respeta la jerarquía de configuración administrada. Si un administrador ha configurado hooks a través de configuración de política administrada, `disableAllHooks` establecido en configuración de usuario, proyecto o local no puede deshabilitar esos hooks administrados. Solo `disableAllHooks` establecido en el nivel de configuración administrada puede deshabilitar hooks administrados.

Las ediciones directas de hooks en archivos de configuración normalmente se capturan automáticamente por el observador de archivos.

<h2 id="hook-input-and-output">
  Entrada y salida de hooks
</h2>

Los hooks de comando reciben datos JSON a través de stdin y comunican resultados a través de códigos de salida, stdout y stderr. Los hooks HTTP reciben el mismo JSON que el cuerpo de la solicitud POST y comunican resultados a través del cuerpo de la respuesta HTTP. Esta sección cubre campos y comportamiento comunes a todos los eventos. Cada sección de evento bajo [Hook events](#hook-events) incluye su esquema de entrada específico y opciones de control de decisión.

En macOS y Linux, los hooks de comando se ejecutan en su propia sesión sin una terminal de control a partir de v2.1.139. El proceso de hook y cualquier proceso secundario no pueden abrir `/dev/tty` o enviar secuencias de escape directamente a la interfaz de Claude Code. Windows no tiene `/dev/tty`. Para mostrar un mensaje al usuario en cualquier plataforma, devuelva [`systemMessage`](#json-output) en la salida JSON. Para activar una notificación de escritorio, establecer un título de ventana o sonar la campana, devuelva [`terminalSequence`](#emit-terminal-notifications) en su lugar.

<h3 id="common-input-fields">
  Campos de entrada comunes
</h3>

Los eventos de hook reciben estos campos como JSON, además de campos específicos del evento documentados en cada sección [hook event](#hook-events). Para hooks de comando, este JSON llega a través de stdin. Para hooks HTTP, llega como el cuerpo de la solicitud POST.

| Campo             | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `session_id`      | Identificador de sesión actual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `prompt_id`       | UUID que identifica el prompt del usuario que se está procesando actualmente. Coincide con el [atributo `prompt.id` en eventos de OpenTelemetry](/docs/es/monitoring-usage#event-correlation-attributes), para que pueda correlacionar la salida del hook con la telemetría de un único prompt. Ausente hasta la primera entrada del usuario. Requiere Claude Code v2.1.196 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `transcript_path` | Ruta al JSON de conversación. El archivo de transcripción se escribe de forma asincrónica y puede rezagarse con respecto a la conversación en memoria, por lo que es posible que aún no incluya los mensajes más recientes del turno actual cuando se activa un hook. Los hooks que necesitan el texto del asistente final del turno actual deben usar `last_assistant_message` en [Stop](#stop) y [SubagentStop](#subagentstop) en lugar de leer la transcripción                                                                                                                                                                                                                                                                                                                                                       |
| `cwd`             | Directorio de trabajo actual cuando se invoca el hook                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `permission_mode` | [Modo de permiso](/docs/es/permissions#permission-modes) actual: `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` o `"bypassPermissions"`. El modo etiquetado como **Manual** llega como `"default"`, nunca como `"manual"`, por lo que los scripts que coinciden con `"default"` siguen funcionando. No todos los eventos reciben este campo. Consulte el ejemplo JSON en cada sección [hook event](#hook-events)                                                                                                                                                                                                                                                                                                                                                                                               |
| `effort`          | Objeto con un campo `level` que contiene el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) activo para el turno: `"low"`, `"medium"`, `"high"`, `"xhigh"` o `"max"`. Si el esfuerzo solicitado del modelo excede lo que el modelo actual admite, este es el nivel degradado que el modelo realmente utilizó. Ultracode no es un nivel distinto y se reporta como `"xhigh"`. El objeto coincide con el campo `effort` de la [línea de estado](/docs/es/statusline#available-data). Presente para eventos que se activan dentro de un contexto de uso de herramienta, como `PreToolUse`, `PostToolUse`, `Stop` y `SubagentStop`, cuando el modelo actual admite el parámetro de esfuerzo. El nivel también está disponible para comandos de hook y la herramienta Bash como la variable de entorno `$CLAUDE_EFFORT`. |
| `hook_event_name` | Nombre del evento que se activó                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |

Cuando se ejecuta con `--agent` o dentro de un subagente, se incluyen dos campos adicionales:

| Campo        | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | Identificador único para el subagente. Presente solo cuando el hook se activa dentro de una llamada de subagente. Use esto para distinguir llamadas de hook de subagente de llamadas de hilo principal.                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `agent_type` | Nombre del agente (por ejemplo, `"Explore"` o `"security-reviewer"`). Presente cuando la sesión usa `--agent` o el hook se activa dentro de un subagente. Para subagentes, el tipo del subagente tiene precedencia sobre el valor `--agent` de la sesión. Para [subagentes personalizados](/docs/es/sub-agents), este es el campo `name` del frontmatter del agente, no el nombre del archivo. Para subagentes enviados por un [plugin](/docs/es/plugins), este es el identificador con alcance de plugin como `my-plugin:reviewer`, no el nombre de frontmatter desnudo. Consulte [SubagentStart](#subagentstart) para saber cómo escribir un matcher contra un nombre con alcance de plugin. |

Solo los hooks [`SessionStart`](#sessionstart) pueden recibir un campo `model`, y no se garantiza que esté presente. No hay variable de entorno `$CLAUDE_MODEL`. Un proceso de hook hereda el entorno principal, por lo que puede leer `$ANTHROPIC_MODEL` si lo establece en su shell, pero ese valor no cambia cuando cambia de modelos con `/model` durante una sesión. Un conjunto de variables no se hereda: Claude Code [elimina variables exportadoras `OTEL_*` de cada subproceso que genera](/docs/es/monitoring-usage#administrator-configuration), incluyendo hooks.

Por ejemplo, un hook `PreToolUse` para un comando Bash recibe esto en stdin:

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

Los campos `tool_name` y `tool_input` son específicos del evento. Cada sección [hook event](#hook-events) documenta los campos adicionales para ese evento.

<h3 id="exit-code-output">
  Salida de código de salida
</h3>

El código de salida de su comando de hook le dice a Claude Code si la acción debe proceder, ser bloqueada o ser ignorada.

**Exit 0** significa éxito. Claude Code analiza stdout para [campos de salida JSON](#json-output). La salida JSON solo se procesa en exit 0. Para la mayoría de eventos, stdout se escribe en el registro de depuración pero no se muestra en la transcripción. Las excepciones son `UserPromptSubmit`, `UserPromptExpansion` y `SessionStart`, donde stdout se agrega como contexto que Claude puede ver y actuar.

**Exit 2** significa un error de bloqueo. Claude Code ignora stdout y cualquier JSON en él. En su lugar, el texto de stderr se devuelve a Claude como un mensaje de error. El efecto depende del evento: `PreToolUse` bloquea la llamada a herramienta, `UserPromptSubmit` rechaza el prompt, y así sucesivamente. Consulte [exit code 2 behavior](#exit-code-2-behavior-per-event) para la lista completa.

**Cualquier otro código de salida** es un error sin bloqueo para la mayoría de eventos de hook. La transcripción muestra un aviso `<hook name> hook error` seguido de la primera línea de stderr, para que pueda identificar la causa sin `--debug`. La ejecución continúa y el stderr completo se escribe en el registro de depuración.

Por ejemplo, un script de comando de hook que bloquea comandos Bash peligrosos:

```bash theme={null}
#!/bin/bash
# Lee entrada JSON desde stdin, verifica el comando
command=$(jq -r '.tool_input.command' < /dev/stdin)

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Blocking error: tool call is prevented
fi

exit 0  # No decision: the normal permission flow applies
```

<Warning>
  Para la mayoría de eventos de hook, solo el código de salida 2 bloquea la acción. Claude Code trata el código de salida 1 como un error sin bloqueo y procede con la acción, aunque 1 es el código de fallo convencional de Unix. Si su hook está destinado a aplicar una política, use `exit 2`. La excepción es `WorktreeCreate`, donde cualquier código de salida distinto de cero aborta la creación de worktree.
</Warning>

<h4 id="exit-code-2-behavior-per-event">
  Comportamiento del código de salida 2 por evento
</h4>

El código de salida 2 es la forma en que un hook señala "detente, no hagas esto". El efecto depende del evento, porque algunos eventos representan acciones que pueden bloquearse (como una llamada a herramienta que aún no ha sucedido) y otros representan cosas que ya sucedieron o no pueden prevenirse.

| Evento de hook        | ¿Puede bloquear? | Qué sucede en exit 2                                                                                                                                           |
| :-------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`          | Sí               | Bloquea la llamada a herramienta                                                                                                                               |
| `PermissionRequest`   | Sí               | Deniega el permiso                                                                                                                                             |
| `UserPromptSubmit`    | Sí               | Bloquea el procesamiento del prompt y borra el prompt                                                                                                          |
| `UserPromptExpansion` | Sí               | Bloquea la expansión                                                                                                                                           |
| `Stop`                | Sí               | Evita que Claude se detenga, continúa la conversación                                                                                                          |
| `SubagentStop`        | Sí               | Evita que el subagente se detenga                                                                                                                              |
| `TeammateIdle`        | Sí               | Evita que el compañero se quede inactivo, por lo que continúa trabajando                                                                                       |
| `TaskCreated`         | Sí               | Revierte la creación de la tarea                                                                                                                               |
| `TaskCompleted`       | Sí               | Evita que la tarea se marque como completada                                                                                                                   |
| `ConfigChange`        | Sí               | Bloquea que el cambio de configuración tenga efecto (excepto `policy_settings`)                                                                                |
| `StopFailure`         | No               | La salida y el código de salida se ignoran                                                                                                                     |
| `PostToolUse`         | No               | Muestra stderr a Claude; la herramienta ya se ejecutó                                                                                                          |
| `PostToolUseFailure`  | No               | Muestra stderr a Claude; la herramienta ya falló                                                                                                               |
| `PostToolBatch`       | Sí               | Detiene el bucle agentico antes de la siguiente llamada al modelo                                                                                              |
| `PermissionDenied`    | No               | El código de salida y stderr se ignoran porque la denegación ya ocurrió. Use JSON `hookSpecificOutput.retry: true` para decirle al modelo que puede reintentar |
| `Notification`        | No               | Muestra stderr solo al usuario                                                                                                                                 |
| `SubagentStart`       | No               | Muestra stderr solo al usuario                                                                                                                                 |
| `SessionStart`        | No               | Muestra stderr solo al usuario                                                                                                                                 |
| `Setup`               | No               | Muestra stderr solo al usuario                                                                                                                                 |
| `SessionEnd`          | No               | Muestra stderr solo al usuario                                                                                                                                 |
| `CwdChanged`          | No               | Muestra stderr solo al usuario                                                                                                                                 |
| `FileChanged`         | No               | Muestra stderr solo al usuario                                                                                                                                 |
| `PreCompact`          | Sí               | Bloquea la compactación                                                                                                                                        |
| `PostCompact`         | No               | Muestra stderr solo al usuario                                                                                                                                 |
| `Elicitation`         | Sí               | Deniega la elicitación                                                                                                                                         |
| `ElicitationResult`   | Sí               | Bloquea la respuesta (la acción se convierte en decline)                                                                                                       |
| `WorktreeCreate`      | Sí               | Cualquier código de salida distinto de cero causa que la creación de worktree falle                                                                            |
| `WorktreeRemove`      | No               | Los fallos se registran solo en modo de depuración                                                                                                             |
| `InstructionsLoaded`  | No               | El código de salida se ignora                                                                                                                                  |
| `MessageDisplay`      | No               | Se muestra el texto original                                                                                                                                   |

Para `SessionStart`, `Setup` y `SubagentStart`, el stderr del código de salida 2 se renderiza en la transcripción como un aviso `<hook name> hook error`, de la misma manera que un [error sin bloqueo](#exit-code-output). Claude no lo ve, y la sesión o subagente procede. Para `SubagentStart`, el aviso aparece en la propia transcripción del subagente, no en la conversación principal.

A partir de Claude Code v2.1.199, `SessionStart`, `Setup` y `SubagentStart` muestran stderr del código de salida 2 en la transcripción. Las versiones anteriores lo escribían solo en el registro de depuración.

<h3 id="http-response-handling">
  Manejo de respuesta HTTP
</h3>

Los hooks HTTP usan códigos de estado HTTP y cuerpos de respuesta en lugar de códigos de salida y stdout:

* **2xx con un cuerpo vacío**: éxito, equivalente a código de salida 0 sin salida
* **2xx con un cuerpo de texto plano**: éxito, el texto se agrega como contexto
* **2xx con un cuerpo JSON**: éxito, analizado usando el mismo esquema [JSON output](#json-output) que los hooks de comando
* **Estado que no es 2xx**: error sin bloqueo, la ejecución continúa
* **Fallo de conexión o tiempo de espera agotado**: error sin bloqueo, la ejecución continúa

A diferencia de los hooks de comando, los hooks HTTP no pueden señalar un error de bloqueo solo a través de códigos de estado. Para bloquear una llamada a herramienta o denegar un permiso, devuelva una respuesta 2xx con un cuerpo JSON que contenga los campos de decisión apropiados.

<h3 id="json-output">
  Salida JSON
</h3>

Los códigos de salida le permiten bloquear o permanecer en silencio, pero la salida JSON le da un control más granular. En lugar de salir con código 2 para bloquear, salga 0 e imprima un objeto JSON en stdout. Claude Code lee campos específicos de ese JSON para controlar el comportamiento, incluyendo [decision control](#decision-control) para bloquear, permitir o escalar al usuario.

<Note>
  Debe elegir un enfoque por hook, no ambos: use códigos de salida solos para señalizar, o salga 0 e imprima JSON para control estructurado. Claude Code solo procesa JSON en exit 0. Si sale 2, cualquier JSON se ignora.
</Note>

El stdout de su hook debe contener solo el objeto JSON. Si su perfil de shell imprime texto al inicio, puede interferir con el análisis JSON. Consulte [JSON validation failed](/docs/es/hooks-guide#json-validation-failed) en la guía de solución de problemas.

Las cadenas de salida de hook, incluyendo `additionalContext`, `systemMessage` y stdout plano, están limitadas a 10.000 caracteres. La salida que excede este límite se guarda en un archivo y se reemplaza con una vista previa y ruta de archivo, de la misma manera que se manejan los resultados de herramientas grandes.

El objeto JSON admite tres tipos de campos:

* **Campos universales** como `continue` funcionan en todos los eventos. Estos se enumeran en la tabla a continuación.
* **`decision` y `reason` de nivel superior** son utilizados por algunos eventos para bloquear o proporcionar retroalimentación.
* **`hookSpecificOutput`** es un objeto anidado para eventos que necesitan control más rico. Requiere un campo `hookEventName` establecido en el nombre del evento.

| Campo              | Predeterminado | Descripción                                                                                                                                                                                                                                                                                                                                                      |
| :----------------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `continue`         | `true`         | Si es `false`, Claude detiene el procesamiento completamente después de que se ejecuta el hook. Tiene precedencia sobre cualquier campo de decisión específico del evento                                                                                                                                                                                        |
| `stopReason`       | ninguno        | Mensaje mostrado al usuario cuando `continue` es `false`. No se muestra a Claude                                                                                                                                                                                                                                                                                 |
| `suppressOutput`   | `false`        | Si es `true`, oculta stdout del hook de la transcripción. Stdout aún aparece en el registro de depuración                                                                                                                                                                                                                                                        |
| `systemMessage`    | ninguno        | Mensaje de advertencia mostrado al usuario                                                                                                                                                                                                                                                                                                                       |
| `terminalSequence` | ninguno        | Una secuencia de escape de terminal para que Claude Code emita en su nombre, como una notificación de escritorio, título de ventana o campana. Restringido a OSC `0`/`1`/`2`/`9`/`99`/`777` y BEL. Si el valor contiene algo fuera de la lista de permitidos, el campo se ignora. Use esto en lugar de escribir en `/dev/tty`, que no está disponible para hooks |

Para detener Claude completamente independientemente del tipo de evento:

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

<h4 id="emit-terminal-notifications">
  Emitir notificaciones de terminal
</h4>

El campo `terminalSequence` requiere Claude Code v2.1.141 o posterior.

Los hooks se ejecutan sin una terminal de control, por lo que escribir secuencias de escape directamente en `/dev/tty` falla. En su lugar, devuelva la secuencia de escape en el campo `terminalSequence` y Claude Code la emite por usted a través de su propia ruta de escritura de terminal. Esto es libre de carreras, funciona dentro de tmux y GNU screen, y funciona en Windows donde no hay `/dev/tty`.

El campo acepta una cadena de una o más secuencias de escape permitidas:

* OSC `0`, `1`, `2`: títulos de ventana e icono
* OSC `9`: notificaciones de iTerm2, ConEmu, Windows Terminal y WezTerm, incluyendo progreso de barra de tareas `9;4`
* OSC `99`: notificaciones de Kitty
* OSC `777`: notificaciones de urxvt, Ghostty y Warp
* BEL desnudo

Las secuencias pueden terminarse con BEL o con ST. Cualquier cosa fuera de la lista de permitidos, incluyendo secuencias de cursor y color CSI, secuencias de paleta OSC, hipervínculos OSC 8, escrituras de portapapeles OSC 52 y OSC 1337, se rechaza y el campo se ignora.

El ejemplo a continuación dispara una notificación de escritorio desde un hook `Notification`. La secuencia de escape se construye con escapes octales `printf` para que los bytes de control nunca aparezcan en la línea de comandos del shell, y `jq -n --arg` construye la salida JSON para que las comillas, barras invertidas y saltos de línea en el mensaje de notificación se escapen correctamente:

```bash theme={null}
#!/bin/bash
# Hook de notificación: ping al escritorio cuando Claude Code necesita atención.
input=$(cat)
title="Claude Code'
body=$(jq -r '.message // 'Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

La forma `{ "terminalSequence": "..." }` es la misma desde cualquier shell o lenguaje. En Windows, construya la cadena de escape en PowerShell o un script y emita el mismo objeto JSON.

<Note>
  `terminalSequence` es el reemplazo compatible para hooks que anteriormente escribían secuencias de escape directamente en `/dev/tty`. La lista de permitidos está restringida a secuencias que no pueden mover el cursor o alterar colores, por lo que un hook nunca puede corromper un prompt en pantalla.
</Note>

<h4 id="add-context-for-claude">
  Agregar contexto para Claude
</h4>

El campo `additionalContext` pasa una cadena de su hook a la ventana de contexto de Claude. Claude Code envuelve la cadena en un recordatorio del sistema e la inserta en la conversación en el punto donde se activó el hook. Claude lee el recordatorio en la siguiente solicitud del modelo, pero no aparece como un mensaje de chat en la interfaz.

Devuelva `additionalContext` dentro de `hookSpecificOutput` junto al nombre del evento:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "This file is generated. Edit src/schema.ts and run `bun generate` instead."
  }
}
```

Dónde aparece el recordatorio depende del evento:

* [SessionStart](#sessionstart), [Setup](#setup) y [SubagentStart](#subagentstart): al inicio de la conversación, antes del primer prompt
* [UserPromptSubmit](#userpromptsubmit) y [UserPromptExpansion](#userpromptexpansion): junto al prompt enviado
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure) y [PostToolBatch](#posttoolbatch): junto al resultado de la herramienta
* [Stop](#stop) y [SubagentStop](#subagentstop): al final del turno. La conversación continúa para que Claude pueda actuar sobre la retroalimentación. Consulte [Stop decision control](#stop-decision-control)

Cuando varios hooks devuelven `additionalContext` para el mismo evento, Claude recibe todos los valores. Si un valor excede 10.000 caracteres, Claude Code escribe el texto completo en un archivo en el directorio de sesión y pasa a Claude la ruta del archivo con una vista previa corta en su lugar.

Use `additionalContext` para información que Claude debe conocer sobre el estado actual de su entorno o la operación que acaba de ejecutarse:

* **Estado del entorno**: la rama actual, destino de implementación o banderas de características activas
* **Reglas de proyecto condicionales**: qué comando de prueba se aplica al archivo que acaba de editar, qué directorios son de solo lectura en este worktree
* **Datos externos**: problemas abiertos asignados a usted, resultados recientes de CI, contenido obtenido de un servicio interno

Para instrucciones que nunca cambian, prefiera [CLAUDE.md](/docs/es/memory). Se carga sin ejecutar un script y es el lugar estándar para convenciones de proyecto estáticas.

Escriba el texto como declaraciones factuales en lugar de instrucciones de sistema imperativas. Frases como "El destino de implementación es producción" o "Este repositorio usa `bun test`" se leen como información del proyecto. El texto enmarcado como comandos de sistema fuera de banda puede activar las defensas de inyección de prompts de Claude, lo que hace que Claude le muestre el texto en lugar de tratarlo como contexto.

Una vez inyectado, el texto se guarda en la transcripción de sesión. Para eventos a mitad de sesión como `PostToolUse` o `UserPromptSubmit`, reanudar con `--continue` o `--resume` reproduce el texto guardado en lugar de volver a ejecutar el hook para turnos anteriores, por lo que valores como marcas de tiempo o SHAs de commit se vuelven obsoletos al reanudar. Los hooks `SessionStart` se ejecutan nuevamente al reanudar con `source` establecido en `"resume"`, por lo que pueden actualizar su contexto.

<h4 id="decision-control">
  Control de decisión
</h4>

No todos los eventos admiten bloqueo o control de comportamiento a través de JSON. Los eventos que lo hacen cada uno usan un conjunto diferente de campos para expresar esa decisión. Use esta tabla como referencia rápida antes de escribir un hook:

| Eventos                                                                                                                             | Patrón de decisión                   | Campos clave                                                                                                                                                                                                                             |
| :---------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | `decision` de nivel superior         | `decision: "block"`, `reason`. Stop y SubagentStop también aceptan `hookSpecificOutput.additionalContext` para [retroalimentación sin error que continúa la conversación](#stop-decision-control)                                        |
| TeammateIdle, TaskCreated, TaskCompleted                                                                                            | Código de salida o `continue: false` | El código de salida 2 bloquea la acción con retroalimentación de stderr. JSON `{"continue": false, "stopReason": "..."}` también detiene al compañero completamente, coincidiendo con el comportamiento del hook `Stop`                  |
| PreToolUse                                                                                                                          | `hookSpecificOutput`                 | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                  |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`                 | `decision.behavior` (allow/deny)                                                                                                                                                                                                         |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`                 | `retry: true` le dice al modelo que puede reintentar la llamada a herramienta denegada                                                                                                                                                   |
| WorktreeCreate                                                                                                                      | ruta return                          | El hook de comando imprime la ruta en stdout; el hook HTTP devuelve `hookSpecificOutput.worktreePath`. El fallo del hook o la ruta faltante falla la creación                                                                            |
| Elicitation                                                                                                                         | `hookSpecificOutput`                 | `action` (accept/decline/cancel), `content` (valores de campo de formulario para accept)                                                                                                                                                 |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`                 | `action` (accept/decline/cancel), `content` (valores de campo de formulario override)                                                                                                                                                    |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`                 | `displayContent` reemplaza el texto mostrado en pantalla. Solo visualización: la transcripción y lo que Claude ve mantienen el original                                                                                                  |
| SessionStart, Setup, SubagentStart                                                                                                  | Solo contexto                        | `hookSpecificOutput.additionalContext` agrega contexto para Claude. SessionStart también acepta [`initialUserMessage`, `watchPaths`, `sessionTitle` y `reloadSkills`](#sessionstart-decision-control). Sin bloqueo o control de decisión |
| WorktreeRemove, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, FileChanged                     | Ninguno                              | Sin control de decisión. Se usa para efectos secundarios como registro o limpieza                                                                                                                                                        |

Algunos eventos también pueden reescribir contenido en lugar de solo permitir o bloquearlo:

* `PreToolUse`: `updatedInput` directamente bajo `hookSpecificOutput` reemplaza los argumentos de una herramienta antes de que se ejecute. Consulte [PreToolUse decision control](#pretooluse-decision-control)
* `PermissionRequest`: `updatedInput` dentro del objeto `decision`. Consulte [PermissionRequest decision control](#permissionrequest-decision-control)
* `PostToolUse`: `updatedToolOutput` reemplaza el resultado de la herramienta. Consulte [PostToolUse decision control](#posttooluse-decision-control)
* `UserPromptSubmit`: no puede reemplazar el prompt; solo inyecta `additionalContext` junto a él

Para casos de uso de redacción o transformación, intercepte en `PreToolUse` para entradas de herramientas salientes y `PostToolUse` para resultados de herramientas entrantes.

Aquí hay ejemplos de cada patrón en acción:

<Tabs>
  <Tab title="Decisión de nivel superior">
    Utilizado por `UserPromptSubmit`, `UserPromptExpansion`, `PostToolUse`, `PostToolUseFailure`, `PostToolBatch`, `Stop`, `SubagentStop`, `ConfigChange` y `PreCompact`. El único valor es `"block"`. Para permitir que la acción continúe, omita `decision` de su JSON, o salga 0 sin ningún JSON en absoluto:

    ```json theme={null}
    {
      "decision": "block",
      "reason": "Test suite must pass before proceeding"
    }
    ```
  </Tab>

  <Tab title="PreToolUse">
    Usa `hookSpecificOutput` para control más rico: permitir, denegar, o escalar al usuario. También puede modificar la entrada de la herramienta antes de que se ejecute o inyectar contexto adicional para Claude. Consulte [PreToolUse decision control](#pretooluse-decision-control) para el conjunto completo de opciones.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Database writes are not allowed"
      }
    }
    ```
  </Tab>

  <Tab title="PermissionRequest">
    Usa `hookSpecificOutput` para permitir o denegar una solicitud de permiso en nombre del usuario. Al permitir, también puede modificar la entrada de la herramienta o aplicar reglas de permiso para que el usuario no sea solicitado nuevamente. Consulte [PermissionRequest decision control](#permissionrequest-decision-control) para el conjunto completo de opciones.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PermissionRequest",
        "decision": {
          "behavior": "allow",
          "updatedInput": {
            "command": "npm run lint"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Para ejemplos extendidos incluyendo validación de comandos Bash, filtrado de prompts y scripts de aprobación automática, consulte [What you can automate](/docs/es/hooks-guide#what-you-can-automate) en la guía y la [implementación de referencia del validador de comandos Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py).

<h2 id="hook-events">
  Eventos de hook
</h2>

Cada evento corresponde a un punto en el ciclo de vida de Claude Code donde los hooks pueden ejecutarse. Las secciones a continuación se ordenan para coincidir con el ciclo de vida: desde la configuración de sesión a través del bucle agentico hasta el final de la sesión. Cada sección describe cuándo se activa el evento, qué matchers admite, la entrada JSON que recibe y cómo controlar el comportamiento a través de la salida.

<h3 id="sessionstart">
  SessionStart
</h3>

Se ejecuta cuando Claude Code inicia una nueva sesión o reanuda una sesión existente. Útil para cargar contexto de desarrollo como problemas existentes o cambios recientes en su base de código, o configurar variables de entorno. Para contexto estático que no requiere un script, use [CLAUDE.md](/docs/es/memory) en su lugar.

SessionStart se ejecuta en cada sesión, así que mantenga estos hooks rápidos. Solo se admiten hooks `type: "command"` y `type: "mcp_tool"`.

El valor del matcher corresponde a cómo se inició la sesión:

| Matcher   | Cuándo se activa                     |
| :-------- | :----------------------------------- |
| `startup` | Nueva sesión                         |
| `resume`  | `--resume`, `--continue` o `/resume` |
| `clear`   | `/clear`                             |
| `compact` | Compactación automática o manual     |

<h4 id="sessionstart-input">
  Entrada de SessionStart
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks SessionStart reciben `source` y opcionalmente `model`, `agent_type` y `session_title`:

| Campo           | Descripción                                                                                                                                                                                                                                           |
| :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `source`        | Cómo comenzó la sesión: `"startup"` para nuevas sesiones, `"resume"` para sesiones reanudadas, `"clear"` después de `/clear` o `"compact"` después de compactación                                                                                    |
| `model`         | El identificador del modelo activo. Puede omitirse, por ejemplo después de `/clear` o cuando se restaura una sesión a través de recuperación de conversación, así que verifique el campo antes de leerlo                                              |
| `agent_type`    | El nombre del agente, presente cuando inicia Claude Code con `claude --agent <name>`                                                                                                                                                                  |
| `session_title` | El título de sesión actual si ya está establecido, por ejemplo a través de `--name` o `/rename`. Un hook que emite `sessionTitle` puede verificar `session_title` primero para evitar sobrescribir un título que el usuario estableció explícitamente |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "startup",
  "model": "claude-sonnet-5"
}
```

<h4 id="sessionstart-decision-control">
  Control de decisión de SessionStart
</h4>

Cualquier texto que su script de hook imprima en stdout se agrega como contexto para Claude. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, puede devolver estos campos específicos del evento:

| Campo                | Descripción                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | Cadena agregada al contexto de Claude al inicio de la conversación, antes del primer prompt. Consulte [Agregar contexto para Claude](#add-context-for-claude) para saber cómo se entrega el texto y qué poner en él                                                                                                                                                               |
| `initialUserMessage` | Cadena utilizada como el primer mensaje de usuario de la sesión. Se aplica en [modo no interactivo](/docs/es/headless) con la bandera `-p`, donde se convierte en el primer turno incluso si no se proporciona ningún prompt. Si se proporciona un prompt, sigue como el siguiente turno. A diferencia de `additionalContext`, que se adjunta a un turno existente, esto crea el turno |
| `sessionTitle`       | Establece el título de la sesión, con el mismo efecto que `/rename`. Use para nombrar sesiones automáticamente desde la carpeta de lanzamiento, rama de git o nombre de worktree. Se aplica solo cuando `source` es `"startup"` o `"resume"`; se ignora en `"clear"` y `"compact"`                                                                                                |
| `watchPaths`         | Array de rutas absolutas para monitorear eventos [FileChanged](#filechanged) durante esta sesión                                                                                                                                                                                                                                                                                  |
| `reloadSkills`       | Booleano. Cuando es `true`, Claude Code vuelve a escanear los directorios [skill](/docs/es/skills) y de comandos después de que se completen los hooks SessionStart, por lo que las skills que instaló el hook están disponibles en la misma sesión, comenzando con el primer prompt                                                                                                   |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Dado que el stdout plano ya llega a Claude para este evento, un hook que solo carga contexto puede imprimir en stdout directamente sin construir JSON. Use el formulario JSON cuando necesite combinar contexto con otros campos como `suppressOutput` o `sessionTitle`.

Use `reloadSkills` cuando un hook SessionStart instala o actualiza skills. El descubrimiento de skills normalmente se ejecuta antes de que se completen los hooks SessionStart, por lo que los archivos que el hook escribe en `~/.claude/skills/` o `.claude/skills/` de otro modo solo aparecerían en la siguiente sesión. Este ejemplo sincroniza un repositorio de skills compartido y solicita el re-escaneo:

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

<h4 id="persist-environment-variables">
  Persistir variables de entorno
</h4>

Los hooks SessionStart tienen acceso a la variable de entorno `CLAUDE_ENV_FILE`, que proporciona una ruta de archivo donde puede persistir variables de entorno para comandos Bash posteriores.

Para establecer variables de entorno individuales, escriba declaraciones `export` en `CLAUDE_ENV_FILE`. Use append (`>>`) para preservar variables establecidas por otros hooks:

```bash theme={null}
#!/bin/bash

if [ -n "$CLAUDE_ENV_FILE" ]; then
  echo 'export NODE_ENV=production' >> "$CLAUDE_ENV_FILE"
  echo 'export DEBUG_LOG=true' >> "$CLAUDE_ENV_FILE"
  echo 'export PATH="$PATH:./node_modules/.bin"' >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Para capturar todos los cambios de entorno de comandos de configuración, compare las variables exportadas antes y después:

```bash theme={null}
#!/bin/bash

ENV_BEFORE=$(export -p | sort)

# Ejecute sus comandos de configuración que modifican el entorno
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Cualquier variable escrita en este archivo estará disponible en todos los comandos Bash posteriores que Claude Code ejecute durante la sesión.

<Note>
  `CLAUDE_ENV_FILE` está disponible para hooks SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged) y [FileChanged](#filechanged). Otros tipos de hooks no tienen acceso a esta variable.
</Note>

<h3 id="setup">
  Setup
</h3>

Se activa solo cuando inicia Claude Code con `--init-only`, o con `--init` o `--maintenance` en [modo no interactivo](/docs/es/headless) con la bandera `-p`. No se activa en el inicio normal. Úselo para instalación de dependencias única o limpieza programada que desencadena explícitamente desde CI o scripts, separado del inicio de sesión normal. Para inicialización por sesión, use [SessionStart](#sessionstart) en su lugar.

El valor del matcher corresponde a la bandera CLI que desencadenó el hook:

| Matcher       | Cuándo se activa                          |
| :------------ | :---------------------------------------- |
| `init`        | `claude --init-only` o `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                 |

`--init-only` ejecuta hooks Setup y hooks SessionStart con el matcher `startup`, luego sale sin iniciar una conversación. `--init` y `--maintenance` activan hooks Setup solo cuando se combinan con `-p`; en una sesión interactiva esas dos banderas actualmente no activan hooks Setup.

Debido a que Setup no se activa en cada lanzamiento, un plugin que necesita una dependencia instalada no puede confiar solo en Setup. El patrón práctico es verificar la dependencia en el primer uso e instalar si falta, por ejemplo un hook o skill que pruebe `${CLAUDE_PLUGIN_DATA}/node_modules` y ejecute `npm install` si está ausente. Consulte el [directorio de datos persistentes](/docs/es/plugins-reference#persistent-data-directory) para saber dónde almacenar dependencias instaladas.

<h4 id="setup-input">
  Entrada de Setup
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks Setup reciben un campo `trigger` establecido en `"init"` o `"maintenance"`:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Setup",
  "trigger": "init"
}
```

<h4 id="setup-decision-control">
  Control de decisión de Setup
</h4>

Los hooks Setup no pueden bloquear. Cualquier código de salida distinto de cero, incluyendo 2, muestra stderr al usuario como un aviso de `<hook name> hook error`, y la ejecución continúa. En [modo no interactivo](/docs/es/headless), la salida del hook aparece solo cuando inicia con `--verbose`.

Para pasar información al contexto de Claude, devuelva `additionalContext` en salida JSON; el stdout plano se escribe solo en el registro de depuración. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, puede devolver estos campos específicos del evento:

| Campo               | Descripción                                                                         |
| :------------------ | :---------------------------------------------------------------------------------- |
| `additionalContext` | Cadena agregada al contexto de Claude. Los valores de múltiples hooks se concatenan |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Setup",
    "additionalContext": "Dependencies installed: node_modules, .venv"
  }
}
```

Los hooks Setup tienen acceso a `CLAUDE_ENV_FILE`. Las variables escritas en ese archivo persisten en comandos Bash posteriores para la sesión, al igual que en los [hooks SessionStart](#persist-environment-variables). Solo se admiten hooks `type: "command"` y `type: "mcp_tool"`.

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

Se activa cuando se carga un archivo `CLAUDE.md` o `.claude/rules/*.md` en contexto. Este evento se activa al inicio de la sesión para archivos cargados con entusiasmo y nuevamente más tarde cuando se cargan archivos de forma perezosa, por ejemplo cuando Claude accede a un subdirectorio que contiene un `CLAUDE.md` anidado o cuando reglas condicionales con frontmatter `paths:` coinciden. El hook no admite bloqueo o control de decisión. Se ejecuta de forma asincrónica con fines de observabilidad.

El matcher se ejecuta contra `load_reason`. Por ejemplo, use `"matcher": "session_start"` para activarse solo para archivos cargados al inicio de la sesión, o `"matcher": "path_glob_match|nested_traversal"` para activarse solo para cargas perezosas.

<h4 id="instructionsloaded-input">
  Entrada de InstructionsLoaded
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks InstructionsLoaded reciben estos campos:

| Campo               | Descripción                                                                                                                                                                                                                                |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Ruta absoluta al archivo de instrucciones que se cargó                                                                                                                                                                                     |
| `memory_type`       | Alcance del archivo: `"User"`, `"Project"`, `"Local"` o `"Managed"`                                                                                                                                                                        |
| `load_reason`       | Por qué se cargó el archivo: `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"` o `"compact"`. El valor `"compact"` se activa cuando los archivos de instrucciones se recargan después de un evento de compactación |
| `globs`             | Patrones de glob de ruta del frontmatter `paths:` del archivo, si los hay. Presente solo para cargas `path_glob_match`                                                                                                                     |
| `trigger_file_path` | Ruta al archivo cuyo acceso desencadenó esta carga, para cargas perezosas                                                                                                                                                                  |
| `parent_file_path`  | Ruta al archivo de instrucciones padre que incluyó este, para cargas `include`                                                                                                                                                             |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "InstructionsLoaded",
  "file_path": "/Users/my-project/CLAUDE.md",
  "memory_type": "Project",
  "load_reason": "session_start"
}
```

<h4 id="instructionsloaded-decision-control">
  Control de decisión de InstructionsLoaded
</h4>

Los hooks InstructionsLoaded no tienen control de decisión. No pueden bloquear o modificar la carga de instrucciones. Use este evento para registro de auditoría, seguimiento de cumplimiento u observabilidad.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

Se ejecuta cuando el usuario envía un prompt, antes de que Claude lo procese. Esto le permite agregar contexto adicional basado en el prompt/conversación, validar prompts o bloquear ciertos tipos de prompts.

Los hooks `UserPromptSubmit` tienen un tiempo de espera predeterminado de 30 segundos para tipos `command`, `http` y `mcp_tool`, más corto que el predeterminado de 600 segundos para esos tipos en otros eventos. Debido a que este hook se ejecuta antes de cada prompt y bloquea el procesamiento del modelo hasta que se completa, un hook atascado detiene la sesión. Si su hook necesita más tiempo, establezca el campo `timeout` en la entrada del hook.

Un hook `UserPromptSubmit` de tipo `command`, `http` o `mcp_tool` que alcanza su tiempo de espera se cancela y su salida, incluyendo cualquier `additionalContext`, se descarta. El prompt aún llega a Claude sin ese contexto. A partir de v2.1.196, la transcripción muestra un aviso que nombra el hook, el tiempo de espera que se activó y que la salida fue descartada. Las versiones anteriores cancelan el hook sin aviso.

Un hook de callback [Agent SDK](/docs/es/agent-sdk/hooks) en `UserPromptSubmit` que alcanza su tiempo de espera bloquea el prompt con un mensaje que nombra el hook y el tiempo de espera, porque un callback allí puede estar actuando como una puerta de política que no debe fallar abierta. La sesión continúa. Antes de v2.1.208, un tiempo de espera de callback en ese evento terminaba el turno con un error de ejecución.

<h4 id="userpromptsubmit-input">
  Entrada de UserPromptSubmit
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks UserPromptSubmit reciben el campo `prompt` que contiene el texto que el usuario envió.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptSubmit",
  "prompt": "Write a function to calculate the factorial of a number"
}
```

<h4 id="userpromptsubmit-decision-control">
  Control de decisión de UserPromptSubmit
</h4>

Los hooks `UserPromptSubmit` pueden controlar si se procesa un prompt de usuario y agregar contexto. Todos los [campos de salida JSON](#json-output) están disponibles.

Hay dos formas de agregar contexto a la conversación en código de salida 0:

* **Stdout de texto plano**: cualquier texto que no sea JSON escrito en stdout se agrega como contexto
* **JSON con `additionalContext`**: use el formato JSON a continuación para más control. El campo `additionalContext` se agrega como contexto

El stdout plano se muestra como salida de hook en la transcripción. El valor de `additionalContext` se inyecta como un recordatorio del sistema que Claude lee sin una entrada de transcripción visible.

Para bloquear un prompt, devuelva un objeto JSON con `decision` establecido en `"block"`:

| Campo                    | Descripción                                                                                                                         |
| :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| `decision`               | `"block"` evita que el prompt se procese y lo borra del contexto. Omita para permitir que el prompt continúe                        |
| `reason`                 | Se muestra al usuario cuando `decision` es `"block"`. No se agrega al contexto                                                      |
| `additionalContext`      | Cadena agregada al contexto de Claude junto con el prompt enviado. Consulte [Agregar contexto para Claude](#add-context-for-claude) |
| `sessionTitle`           | Establece el título de la sesión. Use para nombrar sesiones automáticamente basándose en el contenido del prompt                    |
| `suppressOriginalPrompt` | Si es `true` cuando `decision` es `"block"`, omite el texto del prompt original del mensaje de bloqueo mostrado al usuario          |

```json theme={null}
{
  "decision": "block",
  "reason": "Explanation for decision",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "My additional context here",
    "sessionTitle": "My session title"
  }
}
```

<h3 id="userpromptexpansion">
  UserPromptExpansion
</h3>

Se ejecuta cuando un comando de barra diagonal escrito por el usuario se expande en un prompt antes de llegar a Claude. Use esto para bloquear comandos específicos de invocación directa, inyectar contexto para una skill particular o registrar qué comandos invocan los usuarios. Por ejemplo, un hook que coincida con `deploy` puede bloquear `/deploy` a menos que esté presente un archivo de aprobación, o un hook que coincida con una skill de revisión puede agregar la lista de verificación de revisión del equipo como `additionalContext`.

Este evento cubre la ruta que `PreToolUse` no cubre: un hook `PreToolUse` que coincida con la herramienta `Skill` se activa solo cuando Claude llama a la herramienta, pero escribir `/skillname` directamente omite `PreToolUse`. `UserPromptExpansion` se activa en esa ruta directa.

Coincide en `command_name`. Deje el matcher vacío para activarse en cada comando de barra diagonal de tipo prompt.

<h4 id="userpromptexpansion-input">
  Entrada de UserPromptExpansion
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks UserPromptExpansion reciben `expansion_type`, `command_name`, `command_args`, `command_source` y la cadena `prompt` original. El campo `expansion_type` es `slash_command` para skills y comandos personalizados, o `mcp_prompt` para prompts de servidor MCP.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../00893aaf.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptExpansion",
  "expansion_type": "slash_command",
  "command_name": "example-skill",
  "command_args": "arg1 arg2",
  "command_source": "plugin",
  "prompt": "/example-skill arg1 arg2"
}
```

<h4 id="userpromptexpansion-decision-control">
  Control de decisión de UserPromptExpansion
</h4>

Los hooks `UserPromptExpansion` pueden bloquear la expansión o agregar contexto. Todos los [campos de salida JSON](#json-output) están disponibles.

| Campo               | Descripción                                                                                                                           |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------ |
| `decision`          | `"block"` evita que el comando de barra diagonal se expanda. Omita para permitir que continúe                                         |
| `reason`            | Se muestra al usuario cuando `decision` es `"block"`                                                                                  |
| `additionalContext` | Cadena agregada al contexto de Claude junto con el prompt expandido. Consulte [Agregar contexto para Claude](#add-context-for-claude) |

```json theme={null}
{
  "decision": "block",
  "reason": "This slash command is not available",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptExpansion",
    "additionalContext": "Additional context for this expansion"
  }
}
```

<h3 id="messagedisplay">
  MessageDisplay
</h3>

Se ejecuta mientras un mensaje del asistente se transmite a la pantalla. Claude Code muestra el mensaje en incrementos: cada vez que un lote de líneas recién completadas está listo para renderizar, el hook se ejecuta una vez con esas líneas y Claude Code renderiza el texto de reemplazo del hook en su lugar. Un mensaje largo produce varias llamadas; un mensaje corto puede producir solo una.

Use MessageDisplay para:

* eliminar markdown para una visualización mínima
* transformar el texto que una aplicación del Agent SDK muestra a sus usuarios
* redactar claves API o nombres de host internos de las respuestas de Claude

Claude Code retiene cada lote hasta que su hook regresa, así que mantenga el hook rápido. Si el hook falla o agota el tiempo de espera, Claude Code muestra el texto original. El tiempo de espera predeterminado para este evento es 10 segundos; si su hook necesita más tiempo, establezca el campo `timeout` en la entrada del hook.

MessageDisplay es solo para visualización: el texto de reemplazo cambia solo lo que se renderiza en pantalla. La transcripción y lo que Claude ve mantienen el texto original, por lo que Claude nunca ve el reemplazo, y el modo detallado muestra el original. El hook recibe solo texto de mensaje del asistente, por lo que los resultados de herramientas y el texto que escribe se renderizarán sin cambios.

MessageDisplay no admite matchers y se activa para cada mensaje del asistente que transmite texto; los mensajes sin texto, como respuestas de solo llamada de herramienta, no lo activan.

En ejecuciones no interactivas, incluidas consultas del Agent SDK y `claude -p`, MessageDisplay se ejecuta una vez por mensaje del asistente en lugar de una vez por lote de líneas. La llamada única llega después de que se completa el mensaje y lleva el texto del mensaje completo: `index` es `0`, `final` es `true` y `delta` contiene el mensaje completo. Un hook que recopila el texto `delta` para cada mensaje recibe el mismo texto total en ambos modos.

<h4 id="messagedisplay-input">
  Entrada de MessageDisplay
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks MessageDisplay reciben identificadores para el turno y el mensaje, la posición de esta llamada dentro del mensaje y el nuevo texto en `delta`. Los límites de lote dependen de cómo se transmite el texto, así que use `index` y `final` para rastrear el progreso a través de un mensaje en lugar de esperar que las líneas se agrupen de una manera particular.

| Campo        | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `turn_id`    | UUID del turno actual                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `message_id` | UUID del mensaje del asistente que se está mostrando. Estable en cada lote del mismo mensaje. Este no es el `msg_…` id de API, por lo que no se puede correlacionar con ids de mensaje de transcripción                                                                                                                                                                                                                                                             |
| `index`      | Índice basado en cero de este lote dentro del mensaje                                                                                                                                                                                                                                                                                                                                                                                                               |
| `final`      | `true` en el último lote del mensaje. Cada mensaje tiene exactamente un lote final                                                                                                                                                                                                                                                                                                                                                                                  |
| `delta`      | Las líneas recién completadas desde el lote anterior, incluyendo saltos de línea finales. Siempre líneas completas, excepto el lote final que puede terminar a mitad de línea. En ejecuciones interactivas, el delta del lote final está vacío cuando el mensaje termina en un salto de línea, así que trate `final`, no un delta no vacío, como la señal de fin de mensaje. En ejecuciones del Agent SDK y `claude -p`, la llamada única lleva el mensaje completo |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "MessageDisplay",
  "turn_id": "0c9e6a2f-7d41-4f4e-9a15-3f4f7c2b8d10",
  "message_id": "5b2a9c8e-1f63-4d8a-b7c4-9e0d2a6f1c3b",
  "index": 0,
  "final": false,
  "delta": "Here is the plan:\n"
}
```

<h4 id="messagedisplay-output">
  Salida de MessageDisplay
</h4>

Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, los hooks MessageDisplay pueden devolver `displayContent` para reemplazar el delta en pantalla:

| Campo            | Descripción                                                         |
| :--------------- | :------------------------------------------------------------------ |
| `displayContent` | Texto mostrado en lugar del delta. Omítalo para mostrar el original |

Los hooks MessageDisplay no tienen control de decisión. No pueden bloquear el mensaje o cambiar lo que se almacena en la transcripción o se envía a Claude.

Este ejemplo elimina el formato markdown de las respuestas de Claude para una visualización de texto plano. El script lee cada lote desde stdin, elimina marcadores de negrita y backticks de código en línea de `delta` y devuelve el resultado como `displayContent`.

<Tabs>
  <Tab title="macOS/Linux">
    Registre un hook de comando para el evento en su archivo de configuración:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    Guarde este script en `.claude/hooks/plain-display.sh` en su proyecto y hágalo ejecutable con `chmod +x`:

    ```bash theme={null}
    #!/bin/bash
    jq '{hookSpecificOutput: {hookEventName: "MessageDisplay", displayContent: (.delta | gsub("\\*\\*"; "") | gsub("`"; ""))}}'
    ```

    El script necesita `jq` en su `PATH`.
  </Tab>

  <Tab title="Windows (PowerShell)">
    Registre un hook de comando que ejecute el script a través de PowerShell:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    La bandera `-NoProfile` omite cargar su perfil de PowerShell para que el hook se inicie rápido, y `-ExecutionPolicy Bypass` permite que PowerShell ejecute el archivo de script local.

    Guarde este script en `.claude/hooks/plain-display.ps1` en su proyecto:

    ```powershell theme={null}
    $batch = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $text = $batch.delta -replace '\*\*', '' -replace '`', ''
    @{
      hookSpecificOutput = @{
        hookEventName = "MessageDisplay"
        displayContent = $text
      }
    } | ConvertTo-Json
    ```
  </Tab>
</Tabs>

Los lotes sin markdown pasan sin cambios. Si el script falla, por ejemplo porque `jq` falta, Claude Code muestra el texto original y nota el fallo solo en [salida de depuración](#debug-hooks), no en la sesión.

<h3 id="pretooluse">
  PreToolUse
</h3>

Se ejecuta después de que Claude crea parámetros de herramienta y antes de procesar la llamada a herramienta. Coincide en el nombre de la herramienta: `Bash`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `WebFetch`, `WebSearch`, `AskUserQuestion`, `ExitPlanMode` y cualquier [nombre de herramienta MCP](#match-mcp-tools).

<Warning>
  PreToolUse se ejecuta solo cuando Claude llama a una herramienta. Los archivos que [referencia con `@` en su prompt](/docs/es/common-workflows#reference-files-and-directories) se agregan sin ninguna llamada a herramienta: Claude Code inserta sus contenidos mientras construye el prompt, por lo que no se activa ningún hook PreToolUse para ellos, incluyendo hooks que coincidan con `Read`. Para bloquear rutas específicas de referencias `@`, use una [regla de denegación `Read`](/docs/es/permissions#read-and-edit) en su lugar.
</Warning>

Use [PreToolUse decision control](#pretooluse-decision-control) para permitir, denegar, preguntar o diferir la llamada a herramienta.

<h4 id="pretooluse-input">
  Entrada de PreToolUse
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks PreToolUse reciben `tool_name`, `tool_input` y `tool_use_id`. Los campos `tool_input` dependen de la herramienta:

<h5 id="bash">
  Bash
</h5>

Ejecuta comandos de shell.

| Campo               | Tipo    | Ejemplo            | Descripción                                                                                                                                                            |
| :------------------ | :------ | :----------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | string  | `"npm test"`       | El comando de shell a ejecutar                                                                                                                                         |
| `description`       | string  | `"Run test suite"` | Descripción opcional de lo que hace el comando                                                                                                                         |
| `timeout`           | number  | `120000`           | Tiempo de espera opcional en milisegundos. Los valores por encima del [máximo](/docs/es/tools-reference#bash-tool-behavior) se reducen al máximo en lugar de ser rechazados |
| `run_in_background` | boolean | `false`            | Si se ejecuta el comando en segundo plano                                                                                                                              |

<h5 id="write">
  Write
</h5>

Crea o sobrescribe un archivo.

| Campo       | Tipo   | Ejemplo               | Descripción                         |
| :---------- | :----- | :-------------------- | :---------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Ruta absoluta al archivo a escribir |
| `content`   | string | `"file content"`      | Contenido a escribir en el archivo  |

<h5 id="edit">
  Edit
</h5>

Reemplaza una cadena en un archivo existente.

| Campo         | Tipo    | Ejemplo               | Descripción                            |
| :------------ | :------ | :-------------------- | :------------------------------------- |
| `file_path`   | string  | `"/path/to/file.txt"` | Ruta absoluta al archivo a editar      |
| `old_string`  | string  | `"original text"`     | Texto a encontrar y reemplazar         |
| `new_string`  | string  | `"replacement text"`  | Texto de reemplazo                     |
| `replace_all` | boolean | `false`               | Si se reemplazan todas las ocurrencias |

<h5 id="read">
  Read
</h5>

Lee contenidos de archivo.

| Campo       | Tipo   | Ejemplo               | Descripción                                         |
| :---------- | :----- | :-------------------- | :-------------------------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Ruta absoluta al archivo a leer                     |
| `offset`    | number | `10`                  | Número de línea opcional para comenzar a leer desde |
| `limit`     | number | `50`                  | Número opcional de líneas a leer                    |

<h5 id="glob">
  Glob
</h5>

Encuentra archivos que coincidan con un patrón glob.

| Campo     | Tipo   | Ejemplo          | Descripción                                                                     |
| :-------- | :----- | :--------------- | :------------------------------------------------------------------------------ |
| `pattern` | string | `"**/*.ts"`      | Patrón glob para coincidir archivos contra                                      |
| `path`    | string | `"/path/to/dir"` | Directorio opcional para buscar. Por defecto es el directorio de trabajo actual |

<h5 id="grep">
  Grep
</h5>

Busca contenidos de archivo con expresiones regulares.

| Campo         | Tipo    | Ejemplo          | Descripción                                                                            |
| :------------ | :------ | :--------------- | :------------------------------------------------------------------------------------- |
| `pattern`     | string  | `"TODO.*fix"`    | Patrón de expresión regular a buscar                                                   |
| `path`        | string  | `"/path/to/dir"` | Archivo o directorio opcional para buscar                                              |
| `glob`        | string  | `"*.ts"`         | Patrón glob opcional para filtrar archivos                                             |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"` o `"count"`. Por defecto es `"files_with_matches"` |
| `-i`          | boolean | `true`           | Búsqueda insensible a mayúsculas y minúsculas                                          |
| `multiline`   | boolean | `false`          | Habilitar coincidencia multilínea                                                      |

<h5 id="webfetch">
  WebFetch
</h5>

Obtiene y procesa contenido web.

| Campo    | Tipo   | Ejemplo                       | Descripción                                |
| :------- | :----- | :---------------------------- | :----------------------------------------- |
| `url`    | string | `"https://example.com/api"`   | URL para obtener contenido de              |
| `prompt` | string | `"Extract the API endpoints"` | Prompt a ejecutar en el contenido obtenido |

<h5 id="websearch">
  WebSearch
</h5>

Busca en la web.

| Campo             | Tipo   | Ejemplo                        | Descripción                                         |
| :---------------- | :----- | :----------------------------- | :-------------------------------------------------- |
| `query`           | string | `"react hooks best practices"` | Consulta de búsqueda                                |
| `allowed_domains` | array  | `["docs.example.com"]`         | Opcional: incluir solo resultados de estos dominios |
| `blocked_domains` | array  | `["spam.example.com"]`         | Opcional: excluir resultados de estos dominios      |

<h5 id="agent">
  Agent
</h5>

Genera un [subagente](/docs/es/sub-agents).

| Campo           | Tipo   | Ejemplo                    | Descripción                                            |
| :-------------- | :----- | :------------------------- | :----------------------------------------------------- |
| `prompt`        | string | `"Find all API endpoints"` | La tarea para que el agente realice                    |
| `description`   | string | `"Find API endpoints"`     | Descripción breve de la tarea                          |
| `subagent_type` | string | `"Explore"`                | Tipo de agente especializado a usar                    |
| `model`         | string | `"sonnet"`                 | Alias de modelo opcional para anular el predeterminado |

En `PostToolUse`, `tool_response` para una llamada Agent completada lleva el texto final del subagente junto con telemetría de uso. Lea estos campos para registrar el costo por subagente desde un hook:

| Campo               | Tipo   | Ejemplo                                               | Descripción                                                                                                                                                                                                                                                     |
| :------------------ | :----- | :---------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` para subagentes en primer plano, `"async_launched"` para subagentes en segundo plano. A partir de v2.1.198, los subagentes se ejecutan en segundo plano por defecto, por lo que un `run_in_background` omitido también produce `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Identificador para la ejecución del subagente                                                                                                                                                                                                                   |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | Los bloques de texto finales del subagente                                                                                                                                                                                                                      |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Modelo en el que se ejecutó el subagente, que puede diferir del modelo solicitado. Requiere Claude Code v2.1.174 o posterior                                                                                                                                    |
| `totalTokens`       | number | `12450`                                               | Tokens totales facturados en los turnos del subagente                                                                                                                                                                                                           |
| `totalDurationMs`   | number | `48211`                                               | Duración de reloj de pared de la ejecución del subagente                                                                                                                                                                                                        |
| `totalToolUseCount` | number | `7`                                                   | Recuento de llamadas a herramientas que hizo el subagente                                                                                                                                                                                                       |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Desglose de tokens por tipo: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                                                                          |

Para subagentes en segundo plano, la herramienta regresa inmediatamente después de lanzar, por lo que `tool_response` no lleva campos de uso. Tiene `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile` y `resolvedModel`.

El campo `resolvedModel` nombra el modelo en el que se ejecuta realmente el subagente, que puede diferir del valor `model` en `tool_input`, como cuando `availableModels` u otra anulación se aplica. Requiere Claude Code v2.1.174 o posterior.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Hace al usuario una a cuatro preguntas de opción múltiple.

| Campo       | Tipo   | Ejemplo                                                                                                            | Descripción                                                                                                                                                                                                                                       |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Preguntas a presentar, cada una con una cadena `question`, `header` corto, array `options` y bandera `multiSelect` opcional                                                                                                                       |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Opcional. Asigna texto de pregunta a la etiqueta de opción seleccionada. Las respuestas de selección múltiple unen etiquetas con comas. Claude no establece este campo; suministrarlo a través de `updatedInput` para responder programáticamente |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Presenta un plan y pide al usuario que lo apruebe antes de que Claude salga del [modo plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode). Claude escribe el plan en un archivo en el disco antes de llamar a la herramienta, por lo que el `tool_input` literal del modelo es típicamente vacío. Claude Code inyecta el contenido del plan y la ruta del archivo antes de pasar la entrada a los hooks.

| Campo            | Tipo   | Ejemplo                                     | Descripción                                                                                                                                                          |
| :--------------- | :----- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Contenido del plan en Markdown. Inyectado desde el archivo del plan en el disco                                                                                      |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Ruta al archivo del plan. Inyectado                                                                                                                                  |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Deprecado. Claude Code acepta el campo pero lo ignora. Antes de v2.1.205, llevaba permisos basados en prompts que Claude estaba solicitando para implementar el plan |

En `PostToolUse`, `tool_response` es un objeto con campos `plan` y `filePath` que contienen el plan aprobado, más banderas de estado internas. Lea `tool_response.plan` para el contenido del plan en lugar de releer el archivo desde el disco.

<h4 id="pretooluse-decision-control">
  Control de decisión de PreToolUse
</h4>

Los hooks `PreToolUse` pueden controlar si procede una llamada a herramienta. A diferencia de otros hooks que usan un campo `decision` de nivel superior, PreToolUse devuelve su decisión dentro de un objeto `hookSpecificOutput`. Esto le da control más rico: cuatro resultados (permitir, denegar, preguntar o diferir) más la capacidad de modificar la entrada de la herramienta antes de la ejecución.

| Campo                      | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` omite el sistema de permisos, excepto para [herramientas que requieren interacción del usuario](#pretooluse-decision-control) y herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools). `"deny"` evita la llamada a herramienta. `"ask"` solicita al usuario que confirme. `"defer"` sale correctamente para que la herramienta pueda reanudarse más tarde. Las reglas [Deny and ask](/docs/es/permissions#manage-permissions) aún se evalúan independientemente de lo que devuelva el hook |
| `permissionDecisionReason` | Para `"allow"` y `"ask"`, se muestra al usuario pero no a Claude. Para `"deny"`, se muestra a Claude. Para `"defer"`, se ignora                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `updatedInput`             | Modifica los parámetros de entrada de la herramienta antes de la ejecución. Reemplaza el objeto de entrada completo, así que incluya campos sin cambios junto con los modificados. Combinar con `"allow"` para aprobación automática, o `"ask"` para mostrar la entrada modificada al usuario. Para `"defer"`, se ignora                                                                                                                                                                                                                                       |
| `additionalContext`        | Cadena agregada al contexto de Claude junto con el resultado de la herramienta. Ignorado cuando `permissionDecision` es `"defer"`. Consulte [Agregar contexto para Claude](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                                                            |

Cuando múltiples hooks PreToolUse devuelven diferentes decisiones, la precedencia es `deny` > `defer` > `ask` > `allow`.

Cuando un hook devuelve `"ask"`, el diálogo de permiso mostrado al usuario incluye una etiqueta que identifica de dónde proviene el hook: por ejemplo, `[User]`, `[Project]`, `[Plugin]` o `[Local]`. Esto ayuda a los usuarios a entender qué fuente de configuración está solicitando confirmación.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "My reason here",
    "updatedInput": {
      "field_to_modify": "new value"
    },
    "additionalContext": "Current environment: production. Proceed with caution."
  }
}
```

`AskUserQuestion` y `ExitPlanMode` requieren interacción del usuario y normalmente se bloquean en [modo no interactivo](/docs/es/headless) con la bandera `-p`. Devolver `permissionDecision: "allow"` junto con `updatedInput` satisface ese requisito: el hook lee la entrada de la herramienta desde stdin, recopila la respuesta a través de su propia interfaz de usuario y la devuelve en `updatedInput` para que la herramienta se ejecute sin solicitar. Devolver `"allow"` solo no es suficiente para estas herramientas. Para `AskUserQuestion`, repita el array `questions` original y agregue un objeto [`answers`](#askuserquestion) que asigne el texto de cada pregunta a la respuesta elegida.

Herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) solicitan incluso cuando un hook devuelve `"allow"`.

A partir de v2.1.199, una herramienta MCP cuyo servidor la marca con [`_meta["anthropic/requiresUserInteraction"]`](/docs/es/mcp#require-approval-for-a-specific-tool) es más estricta: un hook no puede omitir su solicitud de aprobación con `"allow"`, con o sin `updatedInput`, porque Claude Code no puede confirmar que el hook recopiló la interacción que la herramienta necesita.

<Note>
  PreToolUse anteriormente usaba campos `decision` y `reason` de nivel superior, pero estos están deprecados para este evento. Use `hookSpecificOutput.permissionDecision` y `hookSpecificOutput.permissionDecisionReason` en su lugar. Los valores deprecados `"approve"` y `"block"` se asignan a `"allow"` y `"deny"` respectivamente. Otros eventos como PostToolUse y Stop continúan usando `decision` y `reason` de nivel superior como su formato actual.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Diferir una llamada a herramienta para más tarde
</h4>

`"defer"` es para integraciones que ejecutan `claude -p` como un subproceso y leen su salida JSON, como una aplicación del Agent SDK o una interfaz de usuario personalizada construida sobre Claude Code. Permite que ese proceso de llamada pause Claude en una llamada a herramienta, recopile entrada a través de su propia interfaz y reanude donde se quedó. Claude Code honra este valor solo en [modo no interactivo](/docs/es/headless) con la bandera `-p`. En sesiones interactivas registra una advertencia e ignora el resultado del hook.

La herramienta `AskUserQuestion` es el caso típico: Claude quiere hacer algo al usuario, pero no hay terminal para responder. El viaje de ida y vuelta funciona así:

1. Claude llama a `AskUserQuestion`. Se activa el hook `PreToolUse`.
2. El hook devuelve `permissionDecision: "defer"`. La herramienta no se ejecuta. El proceso sale con `stop_reason: "tool_deferred"` y la llamada a herramienta pendiente preservada en la transcripción.
3. El proceso de llamada lee `deferred_tool_use` del resultado del SDK, muestra la pregunta en su propia interfaz de usuario y espera una respuesta.
4. El proceso de llamada ejecuta `claude -p --resume <session-id>`. La misma llamada a herramienta activa `PreToolUse` nuevamente.
5. El hook devuelve `permissionDecision: "allow"` con la respuesta en `updatedInput`. La herramienta se ejecuta y Claude continúa.

El campo `deferred_tool_use` lleva el `id`, `name` e `input` de la herramienta. El `input` son los parámetros que Claude generó para la llamada a herramienta, capturados antes de la ejecución:

```json theme={null}
{
  "type": "result",
  "subtype": "success",
  "stop_reason": "tool_deferred",
  "session_id": "abc123",
  "deferred_tool_use": {
    "id": "toolu_01abc",
    "name": "AskUserQuestion",
    "input": { "questions": [{ "question": "Which framework?", "header": "Framework", "options": [{"label": "React"}, {"label": "Vue"}], "multiSelect": false }] }
  }
}
```

No hay límite de tiempo de espera o reintento. La sesión permanece en el disco hasta que la reanude, sujeta al barrido de retención [`cleanupPeriodDays`](/docs/es/settings#available-settings) que elimina archivos de sesión después de 30 días por defecto. Si la respuesta no está lista cuando reanuda, el hook puede devolver `"defer"` nuevamente y el proceso sale de la misma manera. El proceso de llamada controla cuándo romper el bucle devolviendo finalmente `"allow"` o `"deny"` del hook.

`"defer"` solo funciona cuando Claude hace una única llamada a herramienta en el turno. Si Claude hace varias llamadas a herramientas a la vez, `"defer"` se ignora con una advertencia y la herramienta procede a través del flujo de permiso normal. La restricción existe porque reanudar solo puede re-ejecutar una herramienta: no hay forma de diferir una llamada de un lote sin dejar las otras sin resolver.

Si la herramienta diferida ya no está disponible cuando reanuda, el proceso sale con `stop_reason: "tool_deferred_unavailable"` e `is_error: true` antes de que se active el hook. Esto sucede cuando un servidor MCP que proporcionó la herramienta no está conectado para la sesión reanudada. El payload `deferred_tool_use` aún se incluye para que pueda identificar qué herramienta desapareció.

<Note>
  `--resume` restaura el modo de permiso que estaba activo cuando se diferió la herramienta, por lo que no necesita pasar `--permission-mode` nuevamente. Las excepciones son `plan` y `bypassPermissions`, que nunca se llevan. Pasar `--permission-mode` explícitamente en reanudar anula el valor restaurado.
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

Se ejecuta cuando se muestra un diálogo de permiso al usuario.
Use [PermissionRequest decision control](#permissionrequest-decision-control) para permitir o denegar en nombre del usuario.

Coincide en el nombre de la herramienta, los mismos valores que PreToolUse.

<h4 id="permissionrequest-input">
  Entrada de PermissionRequest
</h4>

Los hooks PermissionRequest reciben campos `tool_name` y `tool_input` como los hooks PreToolUse, pero sin `tool_use_id`. Un array `permission_suggestions` opcional contiene las opciones "siempre permitir" que el usuario normalmente vería en el diálogo de permiso. La diferencia es cuándo se activa el hook: los hooks PermissionRequest se ejecutan cuando un diálogo de permiso está a punto de mostrarse al usuario, mientras que los hooks PreToolUse se ejecutan antes de la ejecución de la herramienta independientemente del estado de permiso.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PermissionRequest",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf node_modules",
    "description": "Remove node_modules directory"
  },
  "permission_suggestions": [
    {
      "type": "addRules",
      "rules": [{ "toolName": "Bash", "ruleContent": "rm -rf node_modules" }],
      "behavior": "allow",
      "destination": "localSettings"
    }
  ]
}
```

<h4 id="permissionrequest-decision-control">
  Control de decisión de PermissionRequest
</h4>

Los hooks `PermissionRequest` pueden permitir o denegar solicitudes de permiso. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, su script de hook puede devolver un objeto `decision` con estos campos específicos del evento:

| Campo                | Descripción                                                                                                                                                                                                                                                                       |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `behavior`           | `"allow"` otorga el permiso, `"deny"` lo deniega. Las reglas [Deny and ask](/docs/es/permissions#manage-permissions) aún se evalúan, por lo que un hook que devuelve `"allow"` no anula una regla de denegación coincidente                                                            |
| `updatedInput`       | Solo para `"allow"`: modifica los parámetros de entrada de la herramienta antes de la ejecución. Reemplaza el objeto de entrada completo, así que incluya campos sin cambios junto con los modificados. La entrada modificada se re-evalúa contra reglas de denegación y pregunta |
| `updatedPermissions` | Solo para `"allow"`: array de [entradas de actualización de permiso](#permission-update-entries) a aplicar, como agregar una regla de permiso o cambiar el modo de permiso de sesión                                                                                              |
| `message`            | Solo para `"deny"`: le dice a Claude por qué se denegó el permiso                                                                                                                                                                                                                 |
| `interrupt`          | Solo para `"deny"`: si es `true`, detiene a Claude                                                                                                                                                                                                                                |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedInput": {
        "command": "npm run lint"
      }
    }
  }
}
```

<h4 id="permission-update-entries">
  Entradas de actualización de permiso
</h4>

El campo de salida `updatedPermissions` y el campo de entrada [`permission_suggestions`](#permissionrequest-input) ambos usan el mismo array de objetos de entrada. Cada entrada tiene un `type` que determina sus otros campos, y un `destination` que controla dónde se escribe el cambio.

| `type`              | Campos                             | Efecto                                                                                                                                                                                                                      |
| :------------------ | :--------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `addRules`          | `rules`, `behavior`, `destination` | Agrega reglas de permiso. `rules` es un array de objetos `{toolName, ruleContent?}`. Omita `ruleContent` para coincidir con toda la herramienta. `behavior` es `"allow"`, `"deny"` o `"ask"`                                |
| `replaceRules`      | `rules`, `behavior`, `destination` | Reemplaza todas las reglas del `behavior` dado en el `destination` con las `rules` proporcionadas                                                                                                                           |
| `removeRules`       | `rules`, `behavior`, `destination` | Elimina reglas coincidentes del `behavior` dado                                                                                                                                                                             |
| `setMode`           | `mode`, `destination`              | Cambia el modo de permiso. Los modos válidos son `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan` y `manual` como alias para `default`. El alias `manual` requiere Claude Code v2.1.200 o posterior |
| `addDirectories`    | `directories`, `destination`       | Agrega directorios de trabajo. `directories` es un array de cadenas de ruta                                                                                                                                                 |
| `removeDirectories` | `directories`, `destination`       | Elimina directorios de trabajo                                                                                                                                                                                              |

<Note>
  `setMode` con `bypassPermissions` solo tiene efecto si la sesión se lanzó con modo de omisión ya disponible: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` o `permissions.defaultMode: "bypassPermissions"` en configuración, y el modo no está deshabilitado por [`permissions.disableBypassPermissionsMode`](/docs/es/permissions#managed-settings). De lo contrario, la actualización es una no-op. `bypassPermissions` nunca se persiste como `defaultMode` independientemente de `destination`.
</Note>

El campo `destination` en cada entrada determina si el cambio permanece en memoria o persiste en un archivo de configuración.

| `destination`     | Escribe en                                           |
| :---------------- | :--------------------------------------------------- |
| `session`         | solo en memoria, descartado cuando termina la sesión |
| `localSettings`   | `.claude/settings.local.json`                        |
| `projectSettings` | `.claude/settings.json`                              |
| `userSettings`    | `~/.claude/settings.json`                            |

Un hook puede ecoar una de las `permission_suggestions` que recibió como su propia salida `updatedPermissions`, que es equivalente a que el usuario seleccione esa opción "siempre permitir" en el diálogo.

<h3 id="posttooluse">
  PostToolUse
</h3>

Se ejecuta inmediatamente después de que una herramienta se completa exitosamente.

Coincide en el nombre de la herramienta, los mismos valores que PreToolUse.

<h4 id="posttooluse-input">
  Entrada de PostToolUse
</h4>

Los hooks `PostToolUse` se activan después de que una herramienta ya se ha ejecutado exitosamente. La entrada incluye tanto `tool_input`, los argumentos enviados a la herramienta, como `tool_response`, el resultado que devolvió. El esquema exacto para ambos depende de la herramienta.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/file.txt",
    "content": "file content"
  },
  "tool_response": {
    "filePath": "/path/to/file.txt",
    "success": true
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Campo         | Descripción                                                                                                                             |
| :------------ | :-------------------------------------------------------------------------------------------------------------------------------------- |
| `duration_ms` | Opcional. Tiempo de ejecución de la herramienta en milisegundos. Excluye el tiempo dedicado a solicitudes de permiso y hooks PreToolUse |

<h4 id="posttooluse-decision-control">
  Control de decisión de PostToolUse
</h4>

Los hooks `PostToolUse` pueden proporcionar retroalimentación a Claude después de la ejecución de la herramienta. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, su script de hook puede devolver estos campos específicos del evento:

| Campo                  | Descripción                                                                                                                                                       |
| :--------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`             | `"block"` agrega la `reason` junto al resultado de la herramienta. Claude aún ve la salida original; para reemplazarla, use `updatedToolOutput`                   |
| `reason`               | Explicación mostrada a Claude cuando `decision` es `"block"`                                                                                                      |
| `additionalContext`    | Cadena agregada al contexto de Claude junto con el resultado de la herramienta. Consulte [Agregar contexto para Claude](#add-context-for-claude)                  |
| `updatedToolOutput`    | Reemplaza la salida de la herramienta con el valor proporcionado antes de que se envíe a Claude. El valor debe coincidir con la forma de salida de la herramienta |
| `updatedMCPToolOutput` | Reemplaza la salida para [herramientas MCP](#match-mcp-tools) solo. Prefiera `updatedToolOutput`, que funciona para todas las herramientas                        |

El ejemplo a continuación reemplaza la salida de una llamada `Bash`. El valor de reemplazo coincide con la forma de salida de la herramienta `Bash`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Additional information for Claude",
    "updatedToolOutput": {
      "stdout": "[redacted]",
      "stderr": "",
      "interrupted": false,
      "isImage": false
    }
  }
}
```

<Warning>
  `updatedToolOutput` solo cambia lo que Claude ve. La herramienta ya se ha ejecutado en el momento en que se activa el hook, por lo que cualquier archivo escrito, comando ejecutado o solicitud de red enviada ya ha tenido efecto. La telemetría como spans de herramientas OpenTelemetry y eventos de análisis también capturan la salida original antes de que se ejecute el hook. Para evitar o modificar una llamada a herramienta antes de que se ejecute, use un hook [PreToolUse](#pretooluse) en su lugar.

  El valor de reemplazo debe coincidir con la forma de salida de la herramienta. Las herramientas integradas devuelven objetos estructurados en lugar de cadenas simples. Por ejemplo, `Bash` devuelve un objeto con campos `stdout`, `stderr`, `interrupted` e `isImage`. Para herramientas integradas, un valor que no coincida con el esquema de salida de la herramienta se ignora y se usa la salida original. La salida de herramientas MCP se pasa sin validación de esquema. Eliminar detalles de error que Claude necesita puede hacer que continúe con una suposición falsa.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

Se ejecuta cuando falla una herramienta que comenzó a ejecutarse: la herramienta lanzó un error, o una herramienta MCP devolvió un resultado de error. Use esto para registrar fallos, enviar alertas o proporcionar retroalimentación correctiva a Claude.

Coincide en el nombre de la herramienta, los mismos valores que PreToolUse.

<Note>
  Este evento no se activa para llamadas a herramientas rechazadas antes de la ejecución: un nombre de herramienta desconocido, entrada que falla en validación de esquema o específica de herramienta, o una denegación de permiso. Los rechazos de validación se devuelven como resultados `tool_use_error` y ocurren antes de que se ejecuten los hooks, por lo que no activan ni `PreToolUse` ni este evento. Las denegaciones de permiso activan `PreToolUse` pero no este evento; consulte [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  Entrada de PostToolUseFailure
</h4>

Los hooks PostToolUseFailure reciben los mismos campos `tool_name` y `tool_input` que PostToolUse, junto con información de error como campos de nivel superior:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUseFailure",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite"
  },
  "tool_use_id": "toolu_01ABC123...",
  "error": "Command exited with non-zero status code 1",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Campo          | Descripción                                                                                                                             |
| :------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| `error`        | Cadena que describe qué salió mal                                                                                                       |
| `is_interrupt` | Booleano opcional que indica si el fallo fue causado por interrupción del usuario                                                       |
| `duration_ms`  | Opcional. Tiempo de ejecución de la herramienta en milisegundos. Excluye el tiempo dedicado a solicitudes de permiso y hooks PreToolUse |

<h4 id="posttoolusefailure-decision-control">
  Control de decisión de PostToolUseFailure
</h4>

Los hooks `PostToolUseFailure` pueden proporcionar contexto a Claude después de un fallo de herramienta. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, su script de hook puede devolver estos campos específicos del evento:

| Campo               | Descripción                                                                                                                |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Cadena agregada al contexto de Claude junto con el error. Consulte [Agregar contexto para Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUseFailure",
    "additionalContext": "Additional information about the failure for Claude"
  }
}
```

<h3 id="posttoolbatch">
  PostToolBatch
</h3>

Se ejecuta una vez después de que cada llamada a herramienta en un lote se haya resuelto, antes de que Claude Code envíe la siguiente solicitud al modelo. `PostToolUse` se activa una vez por herramienta, lo que significa que se activa simultáneamente cuando Claude hace llamadas a herramientas paralelas. `PostToolBatch` se activa exactamente una vez con el lote completo, por lo que es el lugar correcto para inyectar contexto que dependa del conjunto de herramientas que se ejecutaron en lugar de cualquier herramienta individual. No hay matcher para este evento.

<h4 id="posttoolbatch-input">
  Entrada de PostToolBatch
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks PostToolBatch reciben `tool_calls`, un array que describe cada llamada a herramienta en el lote:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolBatch",
  "tool_calls": [
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/accounts.py"},
      "tool_use_id": "toolu_01...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    },
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/transactions.py"},
      "tool_use_id": "toolu_02...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    }
  ]
}
```

`tool_response` contiene el mismo contenido que el modelo recibe en el bloque `tool_result` correspondiente. El valor es una cadena serializada o un array de bloque de contenido, exactamente como lo emitió la herramienta. Para `Read`, eso significa texto con prefijo de número de línea en lugar de contenidos de archivo sin procesar. Las respuestas pueden ser grandes, así que analice solo los campos que necesita.

<Note>
  La forma de `tool_response` difiere de la de `PostToolUse`. `PostToolUse` pasa el objeto `Output` estructurado de la herramienta, como `{filePath: "...", success: true}` para `Write`; `PostToolBatch` pasa el contenido `tool_result` serializado que el modelo ve.
</Note>

<h4 id="posttoolbatch-decision-control">
  Control de decisión de PostToolBatch
</h4>

Los hooks `PostToolBatch` pueden inyectar contexto para Claude. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, su script de hook puede devolver estos campos específicos del evento:

| Campo               | Descripción                                                                                                                                                                                                                                     |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Cadena de contexto inyectada una vez antes de la siguiente llamada al modelo. Consulte [Agregar contexto para Claude](#add-context-for-claude) para detalles de entrega, qué poner en él y cómo las sesiones reanudadas manejan valores pasados |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Devolver `decision: "block"` o `continue: false` detiene el bucle agentico antes de la siguiente llamada al modelo.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

Se ejecuta cuando el clasificador de [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) deniega una llamada a herramienta. Este hook solo se activa en modo automático: no se ejecuta cuando deniega manualmente un diálogo de permiso, cuando un hook `PreToolUse` bloquea una llamada o cuando una regla `deny` coincide. Use esto para registrar denegaciones del clasificador, ajustar configuración o decirle al modelo que puede reintentar la llamada a herramienta.

Coincide en el nombre de la herramienta, los mismos valores que PreToolUse.

<h4 id="permissiondenied-input">
  Entrada de PermissionDenied
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks PermissionDenied reciben `tool_name`, `tool_input`, `tool_use_id` y `reason`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "auto",
  "hook_event_name": "PermissionDenied",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf /tmp/build",
    "description": "Clean build directory"
  },
  "tool_use_id": "toolu_01ABC123...",
  "reason": "Auto mode denied: command targets a path outside the project"
}
```

| Campo    | Descripción                                                                     |
| :------- | :------------------------------------------------------------------------------ |
| `reason` | La explicación del clasificador para por qué se denegó la llamada a herramienta |

<h4 id="permissiondenied-decision-control">
  Control de decisión de PermissionDenied
</h4>

Los hooks PermissionDenied pueden decirle al modelo que puede reintentar la llamada a herramienta denegada. Devuelva un objeto JSON con `hookSpecificOutput.retry` establecido en `true`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Cuando `retry` es `true`, Claude Code agrega un mensaje a la conversación diciéndole al modelo que puede reintentar la llamada a herramienta. La denegación en sí no se revierte. Si su hook no devuelve JSON, o devuelve `retry: false`, la denegación se mantiene y el modelo recibe el mensaje de rechazo original.

<h3 id="notification">
  Notification
</h3>

Se ejecuta cuando Claude Code envía notificaciones. Coincide en el tipo de notificación. Omita el matcher para ejecutar hooks para todos los tipos de notificación.

| Matcher                | Cuándo se activa                                                                                                                             |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`    | Claude necesita que apruebe un uso de herramienta                                                                                            |
| `idle_prompt`          | Claude está hecho y esperando su siguiente prompt                                                                                            |
| `auth_success`         | La autenticación se completa                                                                                                                 |
| `elicitation_dialog`   | Un servidor MCP abre un formulario de elicitación                                                                                            |
| `elicitation_complete` | Un formulario de elicitación MCP se envía o se descarta                                                                                      |
| `elicitation_response` | Una respuesta de elicitación MCP se envía de vuelta al servidor                                                                              |
| `agent_needs_input`    | Una sesión en segundo plano comienza a esperar su entrada. Se activa solo mientras [agent view](/docs/es/agent-view) está abierto en una terminal |
| `agent_completed`      | Una sesión en segundo plano termina o falla. Se activa solo mientras [agent view](/docs/es/agent-view) está abierto en una terminal               |

Los tipos `agent_needs_input` y `agent_completed` requieren Claude Code v2.1.198 o posterior.

Use matchers separados para ejecutar diferentes controladores dependiendo del tipo de notificación. Esta configuración desencadena un script de alerta específico de permiso cuando Claude necesita aprobación de permiso y una notificación diferente cuando Claude ha estado inactivo:

```json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/permission-alert.sh"
          }
        ]
      },
      {
        "matcher": "idle_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/idle-notification.sh"
          }
        ]
      }
    ]
  }
}
```

<h4 id="notification-input">
  Entrada de Notification
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks Notification reciben `message` con el texto de notificación, un `title` opcional y `notification_type` que indica qué tipo se activó.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Notification",
  "message": "Claude needs your permission",
  "title": "Permission needed",
  "notification_type": "permission_prompt"
}
```

Los hooks Notification no pueden bloquear o modificar notificaciones. Están destinados a efectos secundarios como reenviar la notificación a un servicio externo. Los [campos de salida JSON](#json-output) comunes como `systemMessage` se aplican.

<h3 id="subagentstart">
  SubagentStart
</h3>

Se ejecuta cuando se genera un subagente de Claude Code a través de la herramienta Agent. Admite matchers para filtrar por nombre de tipo de agente. Para agentes integrados, este es el nombre del agente como `general-purpose`, `Explore` o `Plan`. Para [subagentes personalizados](/docs/es/sub-agents), este es el campo `name` del frontmatter del agente, no el nombre del archivo.

Para subagentes enviados por un [plugin](/docs/es/plugins), el tipo de agente es el identificador con alcance de plugin como `my-plugin:reviewer`, no el nombre del frontmatter desnudo. El colon coloca un nombre con alcance de plugin en la ruta de expresión regular, así que ancle el matcher con `^` y `$` para una coincidencia exacta: `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  Entrada de SubagentStart
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks SubagentStart reciben `agent_id` con el identificador único para el subagente y `agent_type` con el nombre del agente que el matcher filtra.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SubagentStart",
  "agent_id": "agent-abc123",
  "agent_type": "Explore"
}
```

Los hooks SubagentStart no pueden bloquear la creación de subagentes, pero pueden inyectar contexto en el subagente. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, puede devolver:

| Campo               | Descripción                                                                                                                                                         |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `additionalContext` | Cadena agregada al contexto del subagente al inicio de su conversación, antes de su primer prompt. Consulte [Agregar contexto para Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

<h3 id="subagentstop">
  SubagentStop
</h3>

Se ejecuta cuando un subagente de Claude Code ha terminado de responder. Coincide en el tipo de agente, los mismos valores que SubagentStart.

<h4 id="subagentstop-input">
  Entrada de SubagentStop
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks SubagentStop reciben `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path` y `last_assistant_message`. El campo `agent_type` es el valor usado para filtrado de matcher. El `transcript_path` es la transcripción de la sesión principal, mientras que `agent_transcript_path` es la propia transcripción del subagente almacenada en una carpeta `subagents/` anidada. El campo `last_assistant_message` contiene el contenido de texto de la respuesta final del subagente, por lo que los hooks pueden acceder a él sin analizar el archivo de transcripción.

Los hooks SubagentStop también reciben los arrays `background_tasks` y `session_crons` descritos bajo [Stop input](#stop-input), disponibles en Claude Code v2.1.145 o posterior. Ambos arrays están limitados a la sesión padre, no al subagente.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../abc123.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "SubagentStop",
  "stop_hook_active": false,
  "agent_id": "def456",
  "agent_type": "Explore",
  "agent_transcript_path": "~/.claude/projects/.../abc123/subagents/agent-def456.jsonl",
  "last_assistant_message": "Analysis complete. Found 3 potential issues...",
  "background_tasks": [],
  "session_crons": []
}
```

Los hooks SubagentStop usan el mismo formato de control de decisión que los [hooks Stop](#stop-decision-control), incluyendo `hookSpecificOutput.additionalContext` con `hookEventName` establecido en `"SubagentStop"`, para retroalimentación sin error que mantiene el subagente en ejecución. Devolver `decision: "block"` con una `reason` mantiene el subagente en ejecución y entrega `reason` al subagente como su siguiente instrucción. Para inyectar contexto en la sesión padre después de que un subagente regresa, use un hook [`PostToolUse`](#posttooluse) en la herramienta `Agent` en su lugar.

<h3 id="taskcreated">
  TaskCreated
</h3>

Se ejecuta cuando se está creando una tarea a través de la herramienta `TaskCreate`. Use esto para aplicar convenciones de nomenclatura, requerir descripciones de tareas o evitar que se creen ciertas tareas.

Cuando un hook `TaskCreated` sale con código 2, la tarea no se crea y el mensaje de stderr se devuelve al modelo como retroalimentación. Para detener al compañero completamente en lugar de re-ejecutarlo, devuelva JSON con `{"continue": false, "stopReason": "..."}`. Los hooks TaskCreated no admiten matchers y se activan en cada ocurrencia.

<h4 id="taskcreated-input">
  Entrada de TaskCreated
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks TaskCreated reciben `task_id`, `task_subject` y opcionalmente `task_description`, `teammate_name` y `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Campo              | Descripción                                                                 |
| :----------------- | :-------------------------------------------------------------------------- |
| `task_id`          | Identificador de la tarea que se está creando                               |
| `task_subject`     | Título de la tarea                                                          |
| `task_description` | Descripción detallada de la tarea. Puede estar ausente                      |
| `teammate_name`    | Nombre del compañero que crea la tarea. Puede estar ausente                 |
| `team_name`        | Nombre del equipo derivado de la sesión; se eliminará en una versión futura |

<h4 id="taskcreated-decision-control">
  Control de decisión de TaskCreated
</h4>

Los hooks TaskCreated admiten dos formas de controlar la creación de tareas:

* **Código de salida 2**: la tarea no se crea y el mensaje de stderr se devuelve al modelo como retroalimentación.
* **JSON `{"continue": false, "stopReason": "..."}`**: detiene al compañero completamente, coincidiendo con el comportamiento del hook `Stop`. El `stopReason` se muestra al usuario.

Este ejemplo bloquea tareas cujos asuntos no siguen el formato requerido:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

if [[ ! "$TASK_SUBJECT" =~ ^\[TICKET-[0-9]+\] ]]; then
  echo "Task subject must start with a ticket number, e.g. '[TICKET-123] Add feature'" >&2
  exit 2
fi

exit 0
```

<h3 id="taskcompleted">
  TaskCompleted
</h3>

Se ejecuta cuando una tarea está siendo marcada como completada. Esto se activa en dos situaciones: cuando cualquier agente marca explícitamente una tarea como completada a través de la herramienta TaskUpdate, o cuando un compañero de [equipo de agentes](/docs/es/agent-teams) termina su turno con tareas en progreso. Use esto para aplicar criterios de finalización como pasar pruebas o verificaciones de lint antes de que una tarea pueda cerrarse.

Cuando un hook `TaskCompleted` sale con código 2, la tarea no se marca como completada y el mensaje de stderr se devuelve al modelo como retroalimentación. Para detener al compañero completamente en lugar de re-ejecutarlo, devuelva JSON con `{"continue": false, "stopReason": "..."}`. Los hooks TaskCompleted no admiten matchers y se activan en cada ocurrencia.

<h4 id="taskcompleted-input">
  Entrada de TaskCompleted
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks TaskCompleted reciben `task_id`, `task_subject` y opcionalmente `task_description`, `teammate_name` y `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCompleted",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Campo              | Descripción                                                                 |
| :----------------- | :-------------------------------------------------------------------------- |
| `task_id`          | Identificador de la tarea que se está completando                           |
| `task_subject`     | Título de la tarea                                                          |
| `task_description` | Descripción detallada de la tarea. Puede estar ausente                      |
| `teammate_name`    | Nombre del compañero que completa la tarea. Puede estar ausente             |
| `team_name`        | Nombre del equipo derivado de la sesión; se eliminará en una versión futura |

<h4 id="taskcompleted-decision-control">
  Control de decisión de TaskCompleted
</h4>

Los hooks TaskCompleted admiten dos formas de controlar la finalización de tareas:

* **Código de salida 2**: la tarea no se marca como completada y el mensaje de stderr se devuelve al modelo como retroalimentación.
* **JSON `{"continue": false, "stopReason": "..."}`**: detiene al compañero completamente, coincidiendo con el comportamiento del hook `Stop`. El `stopReason` se muestra al usuario.

Este ejemplo ejecuta pruebas y bloquea la finalización de tareas si fallan:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# Ejecute el conjunto de pruebas
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

Se ejecuta cuando el agente principal de Claude Code ha terminado de responder. No se ejecuta si la detención ocurrió debido a una interrupción del usuario. Los errores de API activan [StopFailure](#stopfailure) en su lugar.

<Tip>
  El comando [`/goal`](/docs/es/goal) es un atajo integrado para un hook Stop basado en prompt con alcance de sesión. Úselo cuando desee que Claude siga trabajando hasta que se cumpla una condición sin escribir configuración de hook.
</Tip>

<h4 id="stop-input">
  Entrada de Stop
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks Stop reciben `stop_hook_active`, `last_assistant_message`, `background_tasks` y `session_crons`. El campo `stop_hook_active` es `true` cuando Claude Code ya está continuando como resultado de un hook de parada. Verifique este valor o procese la transcripción para evitar bloquear en una condición que nunca se resolverá. Claude Code anula el hook y termina el turno después de 8 bloqueos consecutivos.

El campo `last_assistant_message` contiene el contenido de texto de la respuesta final de Claude, por lo que los hooks pueden acceder a él sin analizar el archivo de transcripción.

Los arrays `background_tasks` y `session_crons`, disponibles en Claude Code v2.1.145 o posterior, permiten que los hooks distingan "la sesión está hecha" de "la sesión está en pausa esperando que el trabajo en segundo plano la despierte". Ambos arrays están presentes cuando el registro de tareas es accesible y están vacíos cuando nada está en vuelo o programado.

Cada entrada en `background_tasks` describe una tarea en vuelo y usa estos campos:

| Campo         | Descripción                                                                                                                                                                                                                                                            |
| :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | Identificador de tarea                                                                                                                                                                                                                                                 |
| `type`        | Etiqueta de tipo de tarea amigable como `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session` o `MCP task`. Cada etiqueta identifica qué característica de Claude Code creó la tarea. Vuelve al discriminante sin procesar para tipos no reconocidos |
| `status`      | Estado actual de la tarea                                                                                                                                                                                                                                              |
| `description` | Descripción de texto libre, limitada a 1000 caracteres con un marcador `… [+N chars]` en cadena cuando se recorta                                                                                                                                                      |
| `command`     | Línea de comando de shell, limitada a 1000 caracteres. Presente solo para tareas `shell`                                                                                                                                                                               |
| `agent_type`  | Nombre de tipo de subagente. Presente solo para tareas `subagent`                                                                                                                                                                                                      |
| `server`      | Nombre del servidor MCP. Presente solo para tareas `monitor` y `MCP task`                                                                                                                                                                                              |
| `tool`        | Nombre de herramienta MCP. Presente solo para tareas `monitor` y `MCP task`                                                                                                                                                                                            |
| `name`        | Nombre del flujo de trabajo. Presente solo para tareas `workflow`                                                                                                                                                                                                      |

Cada entrada en `session_crons` describe un despertar programado con alcance de sesión, originado en `CronCreate`, `ScheduleWakeup` y `/loop`:

| Campo       | Descripción                                                                                                                                               |
| :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`        | Identificador de tarea cron                                                                                                                               |
| `schedule`  | Expresión cron, por ejemplo `0 9 * * 1-5`                                                                                                                 |
| `recurring` | `false` para despertares únicos cuya programación codifica un tiempo de disparo único, `true` para tareas que se disparan nuevamente en cada coincidencia |
| `prompt`    | Prompt enviado cuando se dispara el cron, limitado a 1000 caracteres con el mismo marcador `… [+N chars]`                                                 |

Este ejemplo muestra una entrada de Stop con una tarea de shell en vuelo y un cron recurrente:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Stop",
  "stop_hook_active": true,
  "last_assistant_message": "I've completed the refactoring. Here's a summary...",
  "background_tasks": [
    {
      "id": "task-001",
      "type": "shell",
      "status": "running",
      "description": "tail logs",
      "command": "tail -f /var/log/syslog"
    }
  ],
  "session_crons": [
    {
      "id": "cron-001",
      "schedule": "0 9 * * 1-5",
      "recurring": true,
      "prompt": "check the build"
    }
  ]
}
```

<h4 id="stop-decision-control">
  Control de decisión de Stop
</h4>

Los hooks `Stop` y `SubagentStop` pueden controlar si Claude continúa. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, su script de hook puede devolver estos campos específicos del evento:

| Campo                                  | Descripción                                                                                                                                                                                                                                    |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`                             | `"block"` evita que Claude se detenga. Omita para permitir que Claude se detenga                                                                                                                                                               |
| `reason`                               | Requerido cuando `decision` es `"block"`. Le dice a Claude por qué debe continuar                                                                                                                                                              |
| `hookSpecificOutput.additionalContext` | Retroalimentación sin error para Claude. La conversación continúa para que Claude pueda actuar sobre ella, pero a diferencia de `decision: "block"` se muestra en la transcripción como retroalimentación de hook en lugar de un error de hook |

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Use `additionalContext` cuando el hook está funcionando como se diseñó y dando orientación a Claude, como "ejecute el conjunto de pruebas antes de terminar". Mantiene la conversación a través de las mismas protecciones de bucle que `decision: "block"`, es decir, la entrada `stop_hook_active` y el límite de 8 continuaciones consecutivas, pero la transcripción la etiqueta como `Stop hook feedback` y no se muestra ninguna notificación de error de hook:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Stop",
    "additionalContext": "Please run the test suite before finishing"
  }
}
```

<h3 id="stopfailure">
  StopFailure
</h3>

Se ejecuta en lugar de [Stop](#stop) cuando el turno termina debido a un error de API. La salida y el código de salida se ignoran. Use esto para registrar fallos, enviar alertas o tomar acciones de recuperación cuando Claude no puede completar una respuesta debido a límites de velocidad, problemas de autenticación u otros errores de API.

<h4 id="stopfailure-input">
  Entrada de StopFailure
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks StopFailure reciben `error`, `error_details` opcional y `last_assistant_message` opcional. El campo `error` identifica el tipo de error y se usa para filtrado de matcher.

| Campo                    | Descripción                                                                                                                                                                                                                                                           |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`                  | Tipo de error: `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens` u `unknown`                                                                   |
| `error_details`          | Detalles adicionales sobre el error, cuando estén disponibles                                                                                                                                                                                                         |
| `last_assistant_message` | El texto de error renderizado mostrado en la conversación. A diferencia de `Stop` y `SubagentStop`, donde este campo contiene la salida conversacional de Claude, para `StopFailure` contiene la cadena de error de API en sí, como `"API Error: Rate limit reached"` |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "StopFailure",
  "error": "rate_limit",
  "error_details": "429 Too Many Requests",
  "last_assistant_message": "API Error: Rate limit reached"
}
```

Los hooks StopFailure no tienen control de decisión. Se ejecutan solo con fines de notificación y registro.

<h3 id="teammateidle">
  TeammateIdle
</h3>

Se ejecuta cuando un compañero de [equipo de agentes](/docs/es/agent-teams) está a punto de quedarse inactivo después de terminar su turno. Use esto para aplicar puertas de calidad antes de que un compañero deje de trabajar, como requerir que pasen verificaciones de lint o verificar que existan archivos de salida.

Cuando un hook `TeammateIdle` sale con código 2, el compañero recibe el mensaje de stderr como retroalimentación y continúa trabajando en lugar de quedarse inactivo. Para detener al compañero completamente en lugar de re-ejecutarlo, devuelva JSON con `{"continue": false, "stopReason": "..."}`. Los hooks TeammateIdle no admiten matchers y se activan en cada ocurrencia.

<h4 id="teammateidle-input">
  Entrada de TeammateIdle
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks TeammateIdle reciben `teammate_name` y `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TeammateIdle",
  "teammate_name": "researcher",
  "team_name": "session-a1b2c3d4"
}
```

| Campo           | Descripción                                                                 |
| :-------------- | :-------------------------------------------------------------------------- |
| `teammate_name` | Nombre del compañero que está a punto de quedarse inactivo                  |
| `team_name`     | Nombre del equipo derivado de la sesión; se eliminará en una versión futura |

<h4 id="teammateidle-decision-control">
  Control de decisión de TeammateIdle
</h4>

Los hooks TeammateIdle admiten dos formas de controlar el comportamiento del compañero:

* **Código de salida 2**: el compañero recibe el mensaje de stderr como retroalimentación y continúa trabajando en lugar de quedarse inactivo.
* **JSON `{"continue": false, "stopReason": "..."}`**: detiene al compañero completamente, coincidiendo con el comportamiento del hook `Stop`. El `stopReason` se muestra al usuario.

Este ejemplo verifica que exista un artefacto de compilación antes de permitir que un compañero se quede inactivo:

```bash theme={null}
#!/bin/bash

if [ ! -f "./dist/output.js" ]; then
  echo "Build artifact missing. Run the build before stopping." >&2
  exit 2
fi

exit 0
```

<h3 id="configchange">
  ConfigChange
</h3>

Se ejecuta cuando un archivo de configuración cambia durante una sesión. Use esto para auditar cambios de configuración, aplicar políticas de seguridad o bloquear modificaciones no autorizadas a archivos de configuración.

Los hooks ConfigChange se activan para cambios en archivos de configuración, configuración de política administrada y archivos de skill. El campo `source` en la entrada le dice qué tipo de configuración cambió, y el campo `file_path` opcional proporciona la ruta al archivo cambiado.

El matcher filtra en la fuente de configuración:

| Matcher            | Cuándo se activa                                  |
| :----------------- | :------------------------------------------------ |
| `user_settings`    | `~/.claude/settings.json` cambia                  |
| `project_settings` | `.claude/settings.json` cambia                    |
| `local_settings`   | `.claude/settings.local.json` cambia              |
| `policy_settings`  | Cambios de configuración de política administrada |
| `skills`           | Un archivo de skill en `.claude/skills/` cambia   |

Este ejemplo registra todos los cambios de configuración para auditoría de seguridad:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/audit-config-change.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

<h4 id="configchange-input">
  Entrada de ConfigChange
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks ConfigChange reciben `source` y opcionalmente `file_path`. El campo `source` indica qué tipo de configuración cambió, y `file_path` proporciona la ruta al archivo específico que se modificó.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ConfigChange",
  "source": "project_settings",
  "file_path": "/Users/.../my-project/.claude/settings.json"
}
```

<h4 id="configchange-decision-control">
  Control de decisión de ConfigChange
</h4>

Los hooks ConfigChange pueden bloquear cambios de configuración para que no tengan efecto. Use código de salida 2 o un JSON `decision` para evitar el cambio. Cuando se bloquea, la nueva configuración no se aplica a la sesión en ejecución.

| Campo      | Descripción                                                                              |
| :--------- | :--------------------------------------------------------------------------------------- |
| `decision` | `"block"` evita que el cambio de configuración se aplique. Omita para permitir el cambio |
| `reason`   | Explicación mostrada al usuario cuando `decision` es `"block"`                           |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

Los cambios de `policy_settings` no pueden bloquearse. Los hooks aún se activan para fuentes de `policy_settings`, por lo que puede usarlos para registro de auditoría, pero cualquier decisión de bloqueo se ignora. Esto asegura que la configuración administrada por empresa siempre tenga efecto.

<h3 id="cwdchanged">
  CwdChanged
</h3>

Se ejecuta cuando el directorio de trabajo cambia durante una sesión, por ejemplo cuando Claude ejecuta un comando `cd`. Use esto para reaccionar a cambios de directorio: recargar variables de entorno, activar cadenas de herramientas específicas del proyecto o ejecutar scripts de configuración automáticamente. Se empareja con [FileChanged](#filechanged) para herramientas como [direnv](https://direnv.net/) que administran el entorno por directorio.

Los hooks CwdChanged tienen acceso a `CLAUDE_ENV_FILE`. Las variables escritas en ese archivo persisten en comandos Bash posteriores para la sesión, al igual que en los [hooks SessionStart](#persist-environment-variables).

CwdChanged no admite matchers y se activa en cada cambio de directorio.

<h4 id="cwdchanged-input">
  Entrada de CwdChanged
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks CwdChanged reciben `old_cwd` y `new_cwd`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project/src",
  "hook_event_name": "CwdChanged",
  "old_cwd": "/Users/my-project",
  "new_cwd": "/Users/my-project/src"
}
```

<h4 id="cwdchanged-output">
  Salida de CwdChanged
</h4>

Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, los hooks CwdChanged pueden devolver `watchPaths` para establecer dinámicamente qué rutas de archivo [FileChanged](#filechanged) monitorea:

| Campo        | Descripción                                                                                                                                                                                                                                  |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array de rutas absolutas. Reemplaza la lista de monitoreo dinámica actual. Las rutas de su configuración de `matcher` siempre se monitorean. Devolver un array vacío borra la lista dinámica, que es típico al entrar en un nuevo directorio |

Los hooks CwdChanged no tienen control de decisión. No pueden bloquear el cambio de directorio.

<h3 id="filechanged">
  FileChanged
</h3>

Se ejecuta cuando un archivo monitoreado cambia en el disco. Útil para recargar variables de entorno cuando se modifican archivos de configuración del proyecto.

El `matcher` para este evento sirve dos propósitos:

* **Construir la lista de vigilancia**: el valor se divide en `|` y cada segmento se registra como un nombre de archivo literal en el directorio de trabajo, por lo que `".envrc|.env"` vigila exactamente esos dos archivos. Los patrones regex no son útiles aquí: un valor como `^\.env` vigilaría un archivo literalmente nombrado `^\.env`.
* **Filtrar qué hooks se ejecutan**: cuando cambia un archivo vigilado, el mismo valor filtra qué grupos de hooks se ejecutan usando las [reglas de matcher](#matcher-patterns) estándar contra el basename del archivo cambiado.

Los hooks FileChanged tienen acceso a `CLAUDE_ENV_FILE`. Las variables escritas en ese archivo persisten en comandos Bash posteriores para la sesión, al igual que en los [hooks SessionStart](#persist-environment-variables).

<h4 id="filechanged-input">
  Entrada de FileChanged
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks FileChanged reciben `file_path` y `event`.

| Campo       | Descripción                                                                                                               |
| :---------- | :------------------------------------------------------------------------------------------------------------------------ |
| `file_path` | Ruta absoluta al archivo que cambió                                                                                       |
| `event`     | Qué sucedió: `"change"` para un archivo modificado, `"add"` para un archivo creado o `"unlink"` para un archivo eliminado |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "FileChanged",
  "file_path": "/Users/my-project/.envrc",
  "event": "change"
}
```

<h4 id="filechanged-output">
  Salida de FileChanged
</h4>

Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, los hooks FileChanged pueden devolver `watchPaths` para actualizar dinámicamente qué rutas de archivo se monitorean:

| Campo        | Descripción                                                                                                                                                                                                                                                 |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array de rutas absolutas. Reemplaza la lista de monitoreo dinámica actual. Las rutas de su configuración de `matcher` siempre se monitorean. Use esto cuando su script de hook descubra archivos adicionales para monitorear basados en el archivo cambiado |

Los hooks FileChanged no tienen control de decisión. No pueden bloquear el cambio de archivo.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

Se ejecuta cuando se está creando un worktree, ya sea desde `claude --worktree` o desde un [subagente usando `isolation: "worktree"`](/docs/es/sub-agents#choose-the-subagent-scope). Por defecto Claude Code crea la copia de trabajo aislada con `git worktree`. Configurar un hook WorktreeCreate reemplaza ese comportamiento predeterminado de git, permitiéndole usar un sistema de control de versiones diferente como SVN, Perforce o Mercurial.

Debido a que el hook reemplaza el comportamiento predeterminado completamente, [`.worktreeinclude`](/docs/es/worktrees#copy-gitignored-files-into-worktrees) no se procesa. Si necesita copiar archivos de configuración local como `.env` en el nuevo worktree, hágalo dentro de su script de hook.

El hook debe devolver la ruta al directorio de worktree creado. Claude Code usa esta ruta como el directorio de trabajo para la sesión aislada. Consulte [WorktreeCreate output](#worktreecreate-output) para saber cómo cada tipo de hook devuelve la ruta.

Este ejemplo crea una copia de trabajo SVN e imprime la ruta para que Claude Code la use. Reemplace la URL del repositorio con la suya:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

El hook lee el `name` del worktree de la entrada JSON en stdin, verifica una copia fresca en un nuevo directorio e imprime la ruta del directorio. El `echo` en la última línea es lo que Claude Code lee como la ruta del worktree. Redirija cualquier otra salida a stderr para que no interfiera con la ruta.

<h4 id="worktreecreate-input">
  Entrada de WorktreeCreate
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks WorktreeCreate reciben el campo `name`. Este es un identificador slug para el nuevo worktree, especificado por el usuario o generado automáticamente, por ejemplo `bold-oak-a3f2`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeCreate",
  "name": "feature-auth"
}
```

<h4 id="worktreecreate-output">
  Salida de WorktreeCreate
</h4>

Los hooks WorktreeCreate no usan el modelo de decisión de permitir/bloquear estándar. En su lugar, el éxito o fallo del hook determina el resultado. El hook debe devolver la ruta al directorio de worktree creado:

* **Hooks de comando** (`type: "command"`): imprimen la ruta como la última línea no vacía de stdout. Claude Code elimina códigos de escape ANSI antes de leer esa línea, por lo que los banners de inicio de shell impresos antes de su `echo` se ignoran. Redirija cualquier otra salida de hook a stderr.
* **Hooks HTTP** (`type: "http"`): devuelven `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` en el cuerpo de la respuesta.

Si el hook falla o no produce ruta, la creación de worktree falla con un error.

Claude Code resuelve una ruta relativa contra el directorio en el que se ejecutó el hook. Si la ruta resultante no es un directorio al que Claude Code pueda entrar, la sesión imprime un error nombrando la ruta y sale con código 1. Antes de v2.1.205, una ruta relativa o una ruta que no existía en el disco causaba un bloqueo de la sesión al inicio, y con `-p` se detenía durante aproximadamente 30 segundos antes de salir con código 0.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

Se ejecuta cuando se está eliminando un worktree, ya sea cuando sale de una sesión `--worktree` y elige eliminarlo, o cuando un subagente con `isolation: "worktree"` finaliza. Esta es la contraparte de limpieza de [WorktreeCreate](#worktreecreate).

Para worktrees basados en git, Claude Code maneja la limpieza automáticamente con `git worktree remove`. Si configuró un hook WorktreeCreate para un sistema de control de versiones que no es git, emparéjelo con un hook WorktreeRemove para manejar la limpieza. Sin uno, el directorio de worktree se deja en el disco.

Claude Code pasa la ruta devuelta por WorktreeCreate como `worktree_path` en la entrada del hook. Este ejemplo lee esa ruta y elimina el directorio:

```json theme={null}
{
  "hooks": {
    "WorktreeRemove": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'jq -r .worktree_path | xargs rm -rf'"
          }
        ]
      }
    ]
  }
}
```

<h4 id="worktreeremove-input">
  Entrada de WorktreeRemove
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks WorktreeRemove reciben el campo `worktree_path`, que es la ruta absoluta al worktree que se está eliminando.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeRemove",
  "worktree_path": "/Users/.../my-project/.claude/worktrees/feature-auth"
}
```

Los hooks WorktreeRemove no tienen control de decisión. No pueden bloquear la eliminación de worktree pero pueden realizar tareas de limpieza como eliminar estado de control de versiones o archivar cambios. Los fallos de hook se registran solo en modo de depuración.

<h3 id="precompact">
  PreCompact
</h3>

Se ejecuta antes de que Claude Code esté a punto de ejecutar una operación de compactación.

El valor del matcher indica si la compactación fue desencadenada manualmente o automáticamente:

| Matcher  | Cuándo se activa                                                 |
| :------- | :--------------------------------------------------------------- |
| `manual` | `/compact`                                                       |
| `auto`   | Compactación automática cuando la ventana de contexto está llena |

Salga con código 2 para bloquear la compactación. Para un `/compact` manual, el mensaje de stderr se muestra al usuario. También puede bloquear devolviendo JSON con `"decision": "block"`.

Bloquear la compactación automática tiene diferentes efectos dependiendo de cuándo se active. Si la compactación fue desencadenada de forma proactiva antes del límite de contexto, Claude Code la omite y la conversación continúa sin compactar. Si la compactación fue desencadenada para recuperarse de un error de límite de contexto ya devuelto por la API, el error subyacente aparece y la solicitud actual falla.

<h4 id="precompact-input">
  Entrada de PreCompact
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks PreCompact reciben `trigger` e `custom_instructions`. Para `manual`, `custom_instructions` contiene lo que el usuario pasa a `/compact`. Para `auto`, `custom_instructions` está vacío.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": ""
}
```

<h3 id="postcompact">
  PostCompact
</h3>

Se ejecuta después de que Claude Code completa una operación de compactación. Use este evento para reaccionar al nuevo estado compactado, por ejemplo para registrar el resumen generado o actualizar el estado externo.

Los mismos valores de matcher se aplican que para `PreCompact`:

| Matcher  | Cuándo se activa                                                            |
| :------- | :-------------------------------------------------------------------------- |
| `manual` | Después de `/compact`                                                       |
| `auto`   | Después de compactación automática cuando la ventana de contexto está llena |

<h4 id="postcompact-input">
  Entrada de PostCompact
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks PostCompact reciben `trigger` y `compact_summary`. El campo `compact_summary` contiene el resumen de conversación generado por la operación de compactación.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PostCompact",
  "trigger": "manual",
  "compact_summary": "Summary of the compacted conversation..."
}
```

Los hooks PostCompact no tienen control de decisión. No pueden afectar el resultado de compactación pero pueden realizar tareas de seguimiento.

<h3 id="sessionend">
  SessionEnd
</h3>

Se ejecuta cuando termina una sesión de Claude Code. Útil para tareas de limpieza, registro de estadísticas de sesión o guardado del estado de sesión. Admite matchers para filtrar por razón de salida.

El campo `reason` en la entrada del hook indica por qué terminó la sesión:

| Razón                         | Descripción                                              |
| :---------------------------- | :------------------------------------------------------- |
| `clear`                       | Sesión borrada con comando `/clear`                      |
| `resume`                      | Sesión cambiada a través de `/resume` interactivo        |
| `logout`                      | Usuario cerró sesión                                     |
| `prompt_input_exit`           | Usuario salió mientras la entrada del prompt era visible |
| `bypass_permissions_disabled` | El modo de permisos de omisión fue deshabilitado         |
| `other`                       | Otras razones de salida                                  |

<h4 id="sessionend-input">
  Entrada de SessionEnd
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks SessionEnd reciben un campo `reason` que indica por qué terminó la sesión. Consulte la tabla de razones anterior para todos los valores.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

Los hooks SessionEnd no tienen control de decisión. No pueden bloquear la terminación de sesión pero pueden realizar tareas de limpieza.

Los hooks SessionEnd tienen un tiempo de espera predeterminado de 1,5 segundos. Esto se aplica tanto a la salida de sesión como a `/clear` y al cambio de sesiones a través de `/resume` interactivo. Si un hook necesita más tiempo, establezca un `timeout` por hook en la configuración del hook. El presupuesto general se aumenta automáticamente al tiempo de espera por hook más alto configurado en archivos de configuración, hasta 60 segundos. Los tiempos de espera establecidos en hooks proporcionados por plugins no aumentan el presupuesto. Para anular el presupuesto explícitamente, establezca la variable de entorno `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` en milisegundos.

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

<h3 id="elicitation">
  Elicitation
</h3>

Se ejecuta cuando un servidor MCP solicita entrada del usuario a mitad de la tarea. Por defecto, Claude Code muestra un diálogo interactivo para que el usuario responda. Los hooks pueden interceptar esta solicitud y responder programáticamente, omitiendo el diálogo completamente.

El campo matcher coincide con el nombre del servidor MCP.

<h4 id="elicitation-input">
  Entrada de Elicitation
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks Elicitation reciben `mcp_server_name`, `message` y campos opcionales `mode`, `url`, `elicitation_id` y `requested_schema`.

Para elicitación en modo formulario (el caso más común):

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please provide your credentials",
  "mode": "form",
  "requested_schema": {
    "type": "object",
    "properties": {
      "username": { "type": "string", "title": "Username" }
    }
  }
}
```

Para elicitación en modo URL (autenticación basada en navegador):

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please authenticate",
  "mode": "url",
  "url": "https://auth.example.com/login"
}
```

<h4 id="elicitation-output">
  Salida de Elicitation
</h4>

Para responder programáticamente sin mostrar el diálogo, devuelva un objeto JSON con `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Elicitation",
    "action": "accept",
    "content": {
      "username": "alice"
    }
  }
}
```

| Campo     | Valores                       | Descripción                                                                      |
| :-------- | :---------------------------- | :------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Si aceptar, rechazar o cancelar la solicitud                                     |
| `content` | object                        | Valores de campo de formulario a enviar. Solo se usa cuando `action` es `accept` |

El código de salida 2 deniega la elicitación y muestra stderr al usuario.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

Se ejecuta después de que un usuario responde a una elicitación MCP. Los hooks pueden observar, modificar o bloquear la respuesta antes de que se envíe de vuelta al servidor MCP.

El campo matcher coincide con el nombre del servidor MCP.

<h4 id="elicitationresult-input">
  Entrada de ElicitationResult
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks ElicitationResult reciben `mcp_server_name`, `action` y campos opcionales `mode`, `elicitation_id` y `content`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "ElicitationResult",
  "mcp_server_name": "my-mcp-server",
  "action": "accept",
  "content": { "username": "alice" },
  "mode": "form",
  "elicitation_id": "elicit-123"
}
```

<h4 id="elicitationresult-output">
  Salida de ElicitationResult
</h4>

Para anular la respuesta del usuario, devuelva un objeto JSON con `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "ElicitationResult",
    "action": "decline",
    "content": {}
  }
}
```

| Campo     | Valores                       | Descripción                                                                          |
| :-------- | :---------------------------- | :----------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Anula la acción del usuario                                                          |
| `content` | object                        | Anula valores de campo de formulario. Solo significativo cuando `action` es `accept` |

El código de salida 2 bloquea la respuesta, cambiando la acción efectiva a `decline`.

<h2 id="prompt-based-hooks">
  Hooks basados en prompts
</h2>

Además de hooks de comando, HTTP y herramientas MCP, Claude Code admite hooks basados en prompts (`type: "prompt"`) que usan un LLM para evaluar si permitir o bloquear una acción, y hooks de agente (`type: "agent"`) que generan un verificador agentico con acceso a herramientas. No todos los eventos admiten todos los tipos de hooks.

Eventos que admiten los cinco tipos de hooks (`command`, `http`, `mcp_tool`, `prompt` y `agent`):

* `PermissionDenied`
* `PermissionRequest`
* `PostToolBatch`
* `PostToolUse`
* `PostToolUseFailure`
* `PreToolUse`
* `Stop`
* `SubagentStop`
* `TaskCompleted`
* `TaskCreated`
* `TeammateIdle`
* `UserPromptExpansion`
* `UserPromptSubmit`

Eventos que admiten hooks `command`, `http` y `mcp_tool` pero no `prompt` o `agent`:

* `ConfigChange`
* `CwdChanged`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `Notification`
* `PostCompact`
* `PreCompact`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` y `Setup` admiten hooks `command` y `mcp_tool`. No admiten hooks `http`, `prompt` o `agent`.

<h3 id="how-prompt-based-hooks-work">
  Cómo funcionan los hooks basados en prompts
</h3>

En lugar de ejecutar un comando Bash, los hooks basados en prompts:

1. Envían la entrada del hook y su prompt a un modelo Claude, Haiku por defecto
2. El LLM responde con JSON estructurado que contiene una decisión
3. Claude Code procesa la decisión automáticamente

<h3 id="prompt-hook-configuration">
  Configuración de hook de prompt
</h3>

Establezca `type` en `"prompt"` y proporcione una cadena `prompt` en lugar de un `command`. Use el marcador de posición `$ARGUMENTS` para inyectar datos de entrada JSON del hook en su texto de prompt. Claude Code envía el prompt combinado e entrada a un modelo Claude rápido, que devuelve una decisión JSON.

Este hook `Stop` le pide al LLM que evalúe si todas las tareas están completas antes de permitir que Claude finalice:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Evaluate if Claude should stop: $ARGUMENTS. Check if all tasks are complete."
          }
        ]
      }
    ]
  }
}
```

| Campo             | Requerido | Descripción                                                                                                                                                                                                                                                                                           |
| :---------------- | :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | sí        | Debe ser `"prompt"`                                                                                                                                                                                                                                                                                   |
| `prompt`          | sí        | El texto del prompt a enviar al LLM. Use `$ARGUMENTS` como marcador de posición para la entrada JSON del hook. Si `$ARGUMENTS` no está presente, la entrada JSON se agrega al prompt                                                                                                                  |
| `model`           | no        | Modelo a usar para evaluación. Por defecto es un modelo rápido                                                                                                                                                                                                                                        |
| `timeout`         | no        | Tiempo de espera en segundos. Predeterminado: 30                                                                                                                                                                                                                                                      |
| `continueOnBlock` | no        | Cuando el prompt devuelve `ok: false`, retroalimente la razón a Claude y continúe el turno en lugar de detener. Predeterminado: `false`. Implementado como `continue: true` en la `decision: "block"` resultante. Consulte [Esquema de respuesta](#response-schema) para el comportamiento por evento |

<h3 id="response-schema">
  Esquema de respuesta
</h3>

El LLM debe responder con JSON que contenga:

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision"
}
```

| Campo    | Descripción                                                                                                         |
| :------- | :------------------------------------------------------------------------------------------------------------------ |
| `ok`     | `true` para permitir. `false` produce una `decision: "block"`. Consulte el comportamiento por evento a continuación |
| `reason` | Requerido cuando `ok` es `false`. Se usa como la razón del bloqueo                                                  |

Lo que sucede en `ok: false` depende del evento:

* `Stop` y `SubagentStop`: la razón se retroalimenta a Claude como su siguiente instrucción y el turno continúa
* `PreToolUse`: la llamada de herramienta se deniega y la razón se devuelve a Claude como el error de la herramienta, equivalente a un hook de comando con `permissionDecision: "deny"`
* `PostToolUse`: por defecto el turno termina y la razón aparece en el chat como una línea de advertencia. Establezca `continueOnBlock: true` para retroalimentar la razón a Claude y continuar el turno en lugar de detener
* `PostToolBatch`, `UserPromptSubmit` y `UserPromptExpansion`: el turno termina y la razón aparece como una línea de advertencia. Estos eventos terminan el turno en `decision: "block"` independientemente de `continue`
* `PostToolUseFailure`, `TaskCreated` y `TaskCompleted`: la razón se devuelve a Claude como un error de herramienta, similar a `PreToolUse`
* `TeammateIdle`: por defecto el compañero se detiene y la razón aparece como una línea de advertencia. Establezca `continueOnBlock: true` para retroalimentar la razón al compañero y mantenerlo trabajando en su lugar
* `PermissionRequest`: `ok: false` no tiene efecto. Para denegar una aprobación desde un hook, use un [hook de comando](#command-hook-fields) que devuelva `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied`: `ok: false` no tiene efecto porque la denegación ya sucedió. La única salida que este evento lee es `hookSpecificOutput.retry`, que los hooks de prompt y agente no pueden establecer. Se ejecutan en este evento, pero su salida se descarta. Use un [hook de comando](#command-hook-fields) para devolver `retry`

Si necesita un control más fino en cualquier evento, use un [hook de comando](#command-hook-fields) con los campos por evento descritos en [Control de decisión](#decision-control).

<h3 id="check-multiple-conditions-before-stopping">
  Verificar múltiples condiciones antes de detener
</h3>

Este hook `Stop` usa un prompt detallado para verificar tres condiciones antes de permitir que Claude se detenga. Los hooks `SubagentStop` usan el mismo formato para evaluar si un [subagente](/docs/es/sub-agents) debe detenerse. Si `"ok"` es `false`, Claude continúa trabajando con la razón proporcionada como su siguiente instrucción:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "You are evaluating whether Claude should stop working. Context: $ARGUMENTS\n\nAnalyze the conversation and determine if:\n1. All user-requested tasks are complete\n2. Any errors need to be addressed\n3. Follow-up work is needed\n\nRespond with JSON: {\"ok\": true} to allow stopping, or {\"ok\": false, \"reason\": \"your explanation\"} to continue working.",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

<h2 id="agent-based-hooks">
  Hooks basados en agentes
</h2>

<Warning>
  Los hooks de agente son experimentales. El comportamiento y la configuración pueden cambiar en futuras versiones. Para flujos de trabajo de producción, prefiera [command hooks](#command-hook-fields).
</Warning>

Los hooks basados en agentes (`type: "agent"`) son como hooks basados en prompts pero con acceso a herramientas de múltiples turnos. En lugar de una única llamada LLM, un hook de agente genera un subagente que puede leer archivos, buscar código e inspeccionar la base de código para verificar condiciones. Los hooks de agente admiten los mismos eventos que los hooks basados en prompts.

<h3 id="how-agent-hooks-work">
  Cómo funcionan los hooks de agente
</h3>

Cuando se activa un hook de agente:

1. Claude Code genera un subagente con su prompt y la entrada JSON del hook
2. El subagente puede usar herramientas como Read, Grep y Glob para investigar
3. Después de hasta 50 turnos, el subagente devuelve una decisión estructurada `{ "ok": true/false }`
4. Claude Code procesa la decisión de la misma manera que un hook de prompt

Los hooks de agente son útiles cuando la verificación requiere inspeccionar archivos reales o salida de prueba, no solo evaluar los datos de entrada del hook solos.

<h3 id="agent-hook-configuration">
  Configuración de hook de agente
</h3>

Establezca `type` en `"agent"` y proporcione una cadena `prompt`. Los campos de configuración son los mismos que los [hooks de prompt](#prompt-hook-configuration), con un tiempo de espera predeterminado más largo:

| Campo     | Requerido | Descripción                                                                                                 |
| :-------- | :-------- | :---------------------------------------------------------------------------------------------------------- |
| `type`    | sí        | Debe ser `"agent"`                                                                                          |
| `prompt`  | sí        | Prompt que describe qué verificar. Use `$ARGUMENTS` como marcador de posición para la entrada JSON del hook |
| `model`   | no        | Modelo a usar. Por defecto es un modelo rápido                                                              |
| `timeout` | no        | Tiempo de espera en segundos. Predeterminado: 60                                                            |

El esquema de respuesta es el mismo que los hooks de prompt: `{ "ok": true }` para permitir o `{ "ok": false, "reason": "..." }` para bloquear.

Este hook `Stop` verifica que todas las pruebas unitarias pasen antes de permitir que Claude finalice:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

<h2 id="run-hooks-in-the-background">
  Ejecutar hooks en segundo plano
</h2>

Por defecto, los hooks bloquean la ejecución de Claude hasta que se completen. Para tareas de larga duración como implementaciones, conjuntos de pruebas o llamadas a API externas, establezca `"async": true` para ejecutar el hook en segundo plano mientras Claude continúa trabajando. Los hooks asincronos no pueden bloquear o controlar el comportamiento de Claude: campos de respuesta como `decision`, `permissionDecision` y `continue` no tienen efecto, porque la acción que habrían controlado ya se ha completado.

<h3 id="configure-an-async-hook">
  Configurar un hook asincrónico
</h3>

Agregue `"async": true` a la configuración de un hook de comando para ejecutarlo en segundo plano sin bloquear a Claude. Este campo solo está disponible en hooks `type: "command"`.

Este hook ejecuta un script de prueba después de cada llamada a herramienta `Write`. Claude continúa trabajando inmediatamente mientras `run-tests.sh` se ejecuta durante hasta 120 segundos. Cuando el script finaliza, su salida se entrega en el siguiente turno de conversación:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/run-tests.sh",
            "async": true,
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

El campo `timeout` establece el tiempo máximo en segundos para el proceso de fondo. Si no se especifica, los hooks asincronos usan el mismo predeterminado de 10 minutos que los hooks sincronos.

<h3 id="how-async-hooks-execute">
  Cómo se ejecutan los hooks asincronos
</h3>

Cuando se activa un hook asincrónico, Claude Code inicia el proceso del hook e inmediatamente continúa sin esperar a que finalice. El hook recibe la misma entrada JSON a través de stdin que un hook sincrónico.

Después de que el proceso de fondo sale, si el hook produjo una respuesta JSON con un campo `additionalContext`, ese contenido se entrega a Claude como contexto en el siguiente turno de conversación. Un campo `systemMessage` se muestra a usted, no a Claude.

Claude Code valida esa respuesta JSON contra el mismo [esquema de salida](#json-output) que los hooks sincronos, y descarta cualquier campo cuyo valor tenga el tipo incorrecto, como un `systemMessage` que no sea una cadena, en lugar de entregarlo. Ejecute con `--debug` para ver una advertencia que nombre cada campo descartado. Antes de v2.1.202, la salida JSON malformada de un hook asincrónico podría bloquear la sesión, y el bloqueo se repetía cada vez que se reanudaba la sesión.

Las notificaciones de finalización de hooks asincronos se suprimen por defecto. Para verlas, habilite el modo detallado con `Ctrl+O` o inicie Claude Code con `--verbose`.

<h3 id="run-tests-after-file-changes">
  Ejecutar pruebas después de cambios de archivo
</h3>

Este hook inicia un conjunto de pruebas en segundo plano cada vez que Claude escribe un archivo, luego reporta los resultados a Claude cuando las pruebas finalizan. Guarde este script en `.claude/hooks/run-tests-async.sh` en su proyecto y hágalo ejecutable con `chmod +x`:

```bash theme={null}
#!/bin/bash
# run-tests-async.sh

# Lee entrada de hook desde stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Solo ejecute pruebas para archivos de origen
if [[ "$FILE_PATH" != *.ts && "$FILE_PATH" != *.js ]]; then
  exit 0
fi

# Ejecute pruebas e informe resultados a Claude a través de additionalContext
RESULT=$(npm test 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  MSG="Tests passed after editing $FILE_PATH"
else
  MSG="Tests failed after editing $FILE_PATH: $RESULT"
fi
jq -nc --arg msg "$MSG" '{hookSpecificOutput: {hookEventName: "PostToolUse", additionalContext: $msg}}'
```

Luego agregue esta configuración a `.claude/settings.json` en la raíz de su proyecto. La bandera `async: true` permite que Claude continúe trabajando mientras se ejecutan las pruebas:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/run-tests-async.sh",
            "args": [],
            "async": true,
            "timeout": 300
          }
        ]
      }
    ]
  }
}
```

<h3 id="limitations">
  Limitaciones
</h3>

Los hooks asincronos tienen varias restricciones en comparación con los hooks sincronos:

* Solo los hooks `type: "command"` admiten `async`. Los hooks basados en prompts no pueden ejecutarse de forma asincrónica.
* Los hooks asincronos no pueden bloquear llamadas a herramientas o devolver decisiones. En el momento en que se completa el hook, la acción desencadenante ya ha procedido.
* La salida del hook se entrega en el siguiente turno de conversación. Si la sesión está inactiva, la respuesta espera hasta la siguiente interacción del usuario. Excepción: un hook `asyncRewake` que sale con código 2 despierta a Claude inmediatamente incluso cuando la sesión está inactiva.
* Cada ejecución crea un proceso de fondo separado. No hay deduplicación en múltiples activaciones del mismo hook asincrónico.

<h2 id="security-considerations">
  Consideraciones de seguridad
</h2>

<h3 id="disclaimer">
  Descargo de responsabilidad
</h3>

Los hooks de comando se ejecutan con los permisos completos del usuario del sistema.

<Warning>
  Los hooks de comando ejecutan comandos de shell con sus permisos de usuario completos. Pueden modificar, eliminar o acceder a cualquier archivo al que su cuenta de usuario pueda acceder. Revise y pruebe todos los comandos de hook antes de agregarlos a su configuración.
</Warning>

<h3 id="security-best-practices">
  Mejores prácticas de seguridad
</h3>

Tenga en cuenta estas prácticas al escribir hooks:

* **Validar y sanitizar entradas**: nunca confíe en datos de entrada ciegamente
* **Siempre entrecomillar variables de shell**: use `"$VAR"` no `$VAR`
* **Bloquear traversal de ruta**: verifique `..` en rutas de archivo
* **Usar rutas absolutas**: especifique rutas completas para scripts. En forma exec, use `${CLAUDE_PROJECT_DIR}` y la ruta no necesita entrecomillarse. En forma shell, envuélvala en comillas dobles
* **Omitir archivos sensibles**: evite `.env`, `.git/`, claves, etc.

<h2 id="windows-powershell-tool">
  Herramienta PowerShell en Windows
</h2>

En Windows, puede ejecutar hooks individuales en PowerShell estableciendo `"shell": "powershell"` en un hook de comando. Los hooks generan PowerShell directamente, por lo que esto funciona independientemente de si `CLAUDE_CODE_USE_POWERSHELL_TOOL` está establecido. Claude Code detecta automáticamente `pwsh.exe`, el ejecutable de PowerShell 7 y posterior, y recurre a `powershell.exe` para Windows PowerShell 5.1.

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "shell": "powershell",
            "command": "Write-Host 'File written'"
          }
        ]
      }
    ]
  }
}
```

Para hacer referencia al directorio raíz del proyecto desde un comando de forma shell de PowerShell, escriba `${CLAUDE_PROJECT_DIR}` o `$env:CLAUDE_PROJECT_DIR`. A partir de v2.1.198, Claude Code reescribe los marcadores de posición `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_PLUGIN_ROOT}` y `${CLAUDE_PLUGIN_DATA}` en un comando de forma shell de PowerShell a la forma `${env:NAME}` de PowerShell, ya sea que el hook esté definido en `settings.json`, un plugin o una skill. PowerShell luego resuelve el valor del entorno exportado después del análisis, por lo que el marcador de posición funciona dentro de cadenas entre comillas dobles pero no dentro de cadenas entre comillas simples, donde PowerShell nunca expande variables.

Antes de v2.1.198, esta reescritura se aplicaba solo a hooks de plugins. En versiones anteriores, un hook de `settings.json` necesita la forma `$env:` o [forma exec](#exec-form-and-shell-form), donde `${CLAUDE_PROJECT_DIR}` se sustituye en cada elemento `args` independientemente de dónde se defina el hook.

No escriba la ortografía desnuda `$CLAUDE_PROJECT_DIR` en un hook de PowerShell. PowerShell la analiza como una variable local indefinida y la resuelve a `$null`, lo que deja la ruta del script sin su prefijo de directorio raíz del proyecto. Claude Code no reescribe esa forma; en su lugar, registra una advertencia en el [registro de depuración](#debug-hooks).

El ejemplo a continuación muestra un hook de `settings.json` que ejecuta un script del proyecto con la forma `$env:`, que funciona en todas las versiones:

```json theme={null}
{
  "type": "command",
  "shell": "powershell",
  "command": "& \"$env:CLAUDE_PROJECT_DIR\\.claude\\hooks\\check.ps1\""
}
```

<h2 id="debug-hooks">
  Depurar hooks
</h2>

Los detalles de ejecución de hooks, incluyendo qué hooks coincidieron, sus códigos de salida y stdout y stderr completos, se escriben en el archivo de registro de depuración. Inicie Claude Code con `claude --debug-file <path>` para escribir el registro en una ubicación conocida, o ejecute `claude --debug` y lea el registro en `~/.claude/debug/<session-id>.txt`. La bandera `--debug` no imprime en la terminal.

```text theme={null}
[DEBUG] Executing hooks for PostToolUse:Write
[DEBUG] Found 1 hook commands to execute
[DEBUG] Executing hook command: <Your command> with timeout 600000ms
[DEBUG] Hook command completed with status 0: <Your stdout>
```

Para detalles de coincidencia de hooks más granulares, establezca `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose` para ver líneas de registro adicionales como recuentos de matchers de hooks y coincidencia de consultas.

Para solucionar problemas comunes como hooks que no se activan, bucles infinitos de hooks Stop o errores de configuración, consulte [Limitaciones y solución de problemas](/docs/es/hooks-guide#limitations-and-troubleshooting) en la guía. Para un recorrido de diagnóstico más amplio que cubra `/context`, `/doctor` y precedencia de configuración, consulte [Depure su configuración](/docs/es/debug-your-config).
